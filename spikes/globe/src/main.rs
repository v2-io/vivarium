//! vivarium-globe — spin the planet, zoom in, see what the spine puts above water.
//!
//! The Phase-2 "smallest-first visible win" (`doc/plan/abyssal-parity-plan.md`):
//! the coarse fBm spine rendered as a whole globe — continents and oceans, pre-
//! erosion, pre-everything. Google-Earth verbs only: drag to spin, wheel to zoom.
//!
//! **The discipline this view exists to demonstrate** (the new frame, `store.rs` +
//! `query.rs`): a view *only queries; it owns no world state*. Every elevation on
//! screen arrived through [`query::spine_tile`] via a persistent [`Store`] — the
//! first pull computes and memoizes; every later launch (and every zoom back out)
//! is a store HIT, and the HUD counts both so you can watch the world being built
//! once and reused. The only state living here is camera state and the meshes the
//! pulled tiles were folded into.
//!
//! **Honesty line** (also on the HUD): this relief is the band-limited fBm surface
//! *prior* — a placeholder that conserves nothing and encodes no tectonics or
//! erosion. It decides land vs water and gives the eye something true-to-the-
//! current-rung to hold; it is not earned terrain. Relief is exaggerated ×20 by
//! default (standard cartographic practice — honest ×1 is a billiard ball, exactly
//! like the real Earth) and the HUD states the factor.
//!
//! Run: `cargo run --release -p vivarium-globe`
//! Controls: drag spin (inertia) · wheel / -/= zoom · arrows spin · [ ] level ·
//!           A auto-level · X relief factor · ,/. scrub solar hour · N/M scrub
//!           day-of-year · P play the diurnal cycle · Y headlight-vs-ephemeris ·
//!           R reset · Esc quit. The sun is the REAL Phase-1 ephemeris by
//!           default (terminator, seasons, polar day/night from planet.rs's
//!           identities); the ethereal viewer scrubs time freely — a pure view
//!           freedom, since the analytic regime makes any moment queryable.
//! Env: VIVARIUM_WORLD (world dir = manifest + store; default
//!      ~/.cache/vivarium/globe-world — a fresh seed is minted and *persisted*
//!      on first run, deterministic ever after; point at another world dir to
//!      view another world; the view never handles a bare seed) ·
//!      VIVARIUM_AUTOSHOT=1 + VIVARIUM_SHOT=path (screenshot then exit — the
//!      worldview verification idiom).

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use vivarium_world::gen::{self, SEA_LEVEL_M};
use vivarium_world::planet::Planet;
use vivarium_world::query::{Source, World};
use vivarium_world::spec::WorldSpec;
use vivarium_world::sphere::{CubeCoord, Face};
use vivarium_world::store::Store;

/// Render unit = 1 km (f32-safe at planetary scale: km magnitudes ≈ 6.4e3, where
/// f32 still resolves sub-metre; metres would put us at 6.4e6 with ~0.5 m ulps).
fn radius_km() -> f32 {
    (Planet::EARTH.radius_m / 1000.0) as f32
}

/// Sampling levels the whole-globe build supports. L9 = 512² cells/face ≈ 20 km
/// cells (~1.6 M vertices for the globe — comfortable). Finer than L9 wants the
/// per-region quadtree (the next rung), not a bigger monolith.
const LEVEL_MIN: u8 = 5;
const LEVEL_MAX: u8 = 9;

/// Relief exaggeration cycle for X. 1 = honest (billiard ball, truthfully).
const EXAG_STEPS: [f32; 4] = [1.0, 10.0, 20.0, 50.0];

const SPACE: Color = Color::srgb(0.012, 0.014, 0.022);

// --- The pull: worker thread that queries the world ---------------------------------
//
// The view's entire knowledge of the world flows through this one function. The
// worker owns the Store handle; the ECS side owns nothing but the result meshes.

/// One face's mesh, built CPU-side by the worker.
struct FaceMesh {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

/// A completed whole-globe build.
struct GlobeMsg {
    level: u8,
    exag: f32,
    faces: Vec<FaceMesh>,
    /// The pulled tiles themselves (face-indexed), kept so the HUD's coordinate
    /// pick reports elevation from the same queried data the meshes were built
    /// from — never a separate computation that could drift.
    tiles: Vec<Vec<f32>>,
    computed: usize,
    hits: usize,
    pull_s: f32,
    /// Fraction of cells above [`SEA_LEVEL_M`] (≈ by-area: equiangular cells
    /// subtend near-equal solid angles, max/min ≈ 1.41 — so "≈", not "=").
    land_frac: f32,
    /// The SEAM INSTRUMENT (see [`seam_stats`]). The view's mesh bridges face
    /// edges C0 — which would quietly *hide* a world-side cross-face-prior
    /// discontinuity (Joseph: "don't accidentally hide our bug by making the
    /// globe look too nice") — so the disagreement is measured and displayed
    /// instead: the whole-globe twin of gen.rs's seam-continuity probe.
    /// Healthy = cross ≈ within on both mean and max.
    seam: SeamStats,
}

/// Corner-grid elevations for one face, with a one-corner **ghost ring**:
/// `(nx+3)²` heights (m above bedrock datum), indexed so grid `(i, j)` is corner
/// `(i−1, j−1)` — the ring at `−1` and `nx+1` lies *on the neighbouring faces*.
///
/// Cell-value fetch for one face's neighbourhood: in-face cells come from the
/// pulled tile (the source of truth); out-of-face cells are fetched by
/// evaluating the prior at the ghost cell's *centre* direction: `to_unit`'s tan
/// runs past the face edge and `from_unit` re-homes the direction onto the
/// neighbour face, landing (by the equiangular identity tan(π/4+x)·tan(π/4−x)
/// = 1) exactly half-a-cell *inside* it, where the face choice is unambiguous.
///
/// Never sample *on* the edge itself: a direction with |u| = 1 sits on
/// `from_unit`'s dominant-axis tie, and the two faces build that component
/// through different float paths (a literal 1.0 vs tan(π/4) = 0.99999…), so the
/// tie resolves differently per face and each samples a different edge cell —
/// whole-edge elevation cliffs, found live as a 60 km-deep skirt canyon.
///
/// (Leans on the spine being a pure function of (seed, position), which at this
/// rung it is — the spine IS the prior. When the spine matures past that,
/// out-of-face cells must come from neighbour-face tile pulls instead; noted so
/// the shortcut can't silently outlive its premise.)
fn cell_value(world: &World, face: Face, level: u8, tile: &[f32], ci: i64, cj: i64) -> f32 {
    let nx = 1usize << level;
    let n = nx as i64;
    if ci >= 0 && ci < n && cj >= 0 && cj < n {
        tile[cj as usize * nx + ci as usize]
    } else {
        let cu = ((ci as f64 + 0.5) / nx as f64) * 2.0 - 1.0;
        let cv = ((cj as f64 + 0.5) / nx as f64) * 2.0 - 1.0;
        let dir = CubeCoord { face, u: cu, v: cv }.to_unit();
        let cell = CubeCoord::from_unit(dir).cell(level);
        gen::surface_prior_m(world.seed(), cell, level) as f32
    }
}

/// Corner-grid elevations with a one-corner ghost ring: `(nx+3)²` heights,
/// grid `(gi, gj)` = corner `(gi−1, gj−1)`. Every corner — interior, boundary,
/// ghost — is the average of its 4 adjacent **cells** via [`cell_value`], so
/// both faces of a shared edge average the same four discrete cells and agree
/// to summation-order ulps: the mesh is C0 at face seams by construction.
///
/// NOTE this C0 bridge is a *mesh* property, deliberately NOT a claim about the
/// world: where the prior itself is discontinuous across an edge (the known
/// engine-side per-face-fBm deficiency, ~2–3 km cliffs), the bridge renders a
/// one-cell V-trench — and the seam *instrument* ([`seam_stats`], HUD + S
/// paint) keeps that disagreement loudly visible rather than smoothed away.
fn corner_heights(world: &World, face: Face, level: u8, tile: &[f32]) -> Vec<f32> {
    let nx = 1usize << level;
    let gn = nx + 3; // corners −1 ..= nx+1
    let mut h = vec![0.0f32; gn * gn];
    for gj in 0..gn {
        for gi in 0..gn {
            let (ki, kj) = (gi as i64 - 1, gj as i64 - 1); // corner coords
            let sum = cell_value(world, face, level, tile, ki - 1, kj - 1)
                + cell_value(world, face, level, tile, ki, kj - 1)
                + cell_value(world, face, level, tile, ki - 1, kj)
                + cell_value(world, face, level, tile, ki, kj);
            h[gj * gn + gi] = sum * 0.25;
        }
    }
    h
}

/// Per-face seam aggregates, in metres. `cross` = elevation step ACROSS the
/// edge (tile edge cell vs the neighbour face's abutting cell); `within` = the
/// matched-arc-length baseline (edge cell vs its inward neighbour) — the same
/// cross-vs-within method as gen.rs's seam-continuity probe, wall-to-wall.
#[derive(Clone, Copy, Default)]
struct SeamStats {
    cross_max: f32,
    within_max: f32,
    cross_sum: f64,
    within_sum: f64,
    n: usize,
}

/// The seam instrument. Returns the aggregates plus a per-corner **excess**
/// grid `(nx+1)²` for the S paint mode: excess = cross step beyond 3× the
/// *local* within step (plus a 100 m floor). Terrain that is merely steep is
/// steep on both measures and stays dark; a genuine discontinuity is loud on
/// cross alone and lights up — the instrument discriminates, it doesn't just
/// flag mountains that happen to touch an edge.
fn seam_stats(world: &World, face: Face, level: u8, tile: &[f32]) -> (SeamStats, Vec<f32>) {
    let nx = 1usize << level;
    let n1 = nx + 1;
    let n = nx as i64;
    let mut excess = vec![0.0f32; n1 * n1];
    let mut st = SeamStats::default();
    // (in-cell, ghost-cell, within-neighbour, the two bounding corners) per edge cell.
    let mut edge = |ic: (i64, i64), gc: (i64, i64), wc: (i64, i64), ca: (usize, usize), cb: (usize, usize)| {
        let h = cell_value(world, face, level, tile, ic.0, ic.1);
        let d_cross = (h - cell_value(world, face, level, tile, gc.0, gc.1)).abs();
        let d_within = (h - cell_value(world, face, level, tile, wc.0, wc.1)).abs();
        st.cross_max = st.cross_max.max(d_cross);
        st.within_max = st.within_max.max(d_within);
        st.cross_sum += d_cross as f64;
        st.within_sum += d_within as f64;
        let e = (d_cross - 3.0 * d_within - 100.0).max(0.0);
        for (i, j) in [ca, cb] {
            let k = j * n1 + i;
            excess[k] = excess[k].max(e);
        }
    };
    for k in 0..nx {
        let ki = k as i64;
        edge((ki, 0), (ki, -1), (ki, 1), (k, 0), (k + 1, 0)); // j = 0 edge
        edge((ki, n - 1), (ki, n), (ki, n - 2), (k, nx), (k + 1, nx)); // j = nx−1 edge
        edge((0, ki), (-1, ki), (1, ki), (0, k), (0, k + 1)); // i = 0 edge
        edge((n - 1, ki), (n, ki), (n - 2, ki), (nx, k), (nx, k + 1)); // i = nx−1 edge
    }
    st.n = 4 * nx;
    (st, excess)
}

/// Hypsometric colour for an elevation (m above bedrock datum). Water is a depth
/// ramp (shelf → abyss), land runs green → tan → brown → snow. The shoreline is
/// where the datum says it is — colour, not artistry, decides land vs water.
///
/// Returned in *linear* RGB: the mesh COLOR attribute is linear, and feeding it
/// sRGB washes everything toward white (the first-light screenshot's defect).
fn shade(h_m: f32) -> [f32; 4] {
    let lerp3 = |a: [f32; 3], b: [f32; 3], t: f32| {
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
    };
    let rel = h_m - SEA_LEVEL_M as f32;
    let c = if rel <= 0.0 {
        let t = (-rel / 3800.0).clamp(0.0, 1.0).powf(0.65);
        lerp3([0.25, 0.49, 0.62], [0.015, 0.07, 0.20], t)
    } else if rel < 350.0 {
        lerp3([0.30, 0.47, 0.24], [0.45, 0.52, 0.27], rel / 350.0)
    } else if rel < 1300.0 {
        lerp3([0.45, 0.52, 0.27], [0.61, 0.53, 0.36], (rel - 350.0) / 950.0)
    } else if rel < 2300.0 {
        lerp3([0.61, 0.53, 0.36], [0.47, 0.38, 0.31], (rel - 1300.0) / 1000.0)
    } else {
        lerp3([0.47, 0.38, 0.31], [0.93, 0.94, 0.96], ((rel - 2300.0) / 900.0).min(1.0))
    };
    // sRGB → linear (exact piecewise transfer function).
    let lin = |s: f32| if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) };
    [lin(c[0]), lin(c[1]), lin(c[2]), 1.0]
}

/// Build one face's mesh from its pulled tile. Geometry: corners projected onto
/// the sphere at `R + max(0, h − sea) · exag` — the ocean renders as the smooth
/// sea-level sphere (bathymetry is colour-only), which is exactly the ask: the
/// landmasses *above water*.
fn build_face(
    world: &World,
    face: Face,
    level: u8,
    tile: &[f32],
    exag: f32,
    audit: bool,
) -> (FaceMesh, SeamStats) {
    let nx = 1usize << level;
    let n1 = nx + 1; // rendered corners per edge
    let gn = nx + 3; // + ghost ring (normals only)
    let r_km = radius_km();
    let h = corner_heights(world, face, level, tile);
    let (seam, excess) = seam_stats(world, face, level, tile);

    // Positions over the WHOLE ghost grid (normals need them); colors and the
    // final vertex buffer use only the interior n1 × n1 slice.
    let gidx = |i: usize, j: usize| j * gn + i;
    let mut gpos = vec![Vec3::ZERO; gn * gn];
    for gj in 0..gn {
        for gi in 0..gn {
            let u = ((gi as f64 - 1.0) / nx as f64) * 2.0 - 1.0;
            let v = ((gj as f64 - 1.0) / nx as f64) * 2.0 - 1.0;
            let d = CubeCoord { face, u, v }.to_unit();
            let hm = h[gidx(gi, gj)];
            let r = (r_km + ((hm - SEA_LEVEL_M as f32).max(0.0) / 1000.0) * exag) as f64;
            gpos[gidx(gi, gj)] = Vec3::new((d[0] * r) as f32, (d[1] * r) as f32, (d[2] * r) as f32);
        }
    }

    // Winding is face-chirality-dependent (each face's (u,v) → 3-space basis
    // differs in handedness), so probe one quad against the outward direction and
    // flip the whole face if needed — measured, not assumed.
    //
    // `flip = true` selects the NATURAL grid winding [a,b,c] below, so it must
    // hold exactly when the natural winding's geometric normal points OUTWARD
    // (n·a > 0 ⇒ CCW seen from outside ⇒ front-facing under Bevy's default
    // cull). The original probe had this comparison inverted (`< 0.0`) — every
    // face emitted inward-wound triangles, the near hemisphere was culled, and
    // the globe rendered as the far shell seen from inside (Joseph's live
    // sighting, 2026-07-10). A screenshot can't catch this class of bug without
    // a chirality reference: a mirrored coastline still reads as "a coastline."
    let flip = {
        let (a, b, c) = (gpos[gidx(1, 1)], gpos[gidx(2, 1)], gpos[gidx(1, 2)]);
        (b - a).cross(c - a).dot(a) > 0.0
    };

    // Smooth normals accumulated from triangle faces over the ghost grid — the
    // boundary vertices thus feel the neighbour face's slopes, and both sides of
    // an edge shade continuously. Relief shading is what makes the exaggerated
    // terrain read as form rather than noise.
    let mut gnorm = vec![Vec3::ZERO; gn * gn];
    for gj in 0..gn - 1 {
        for gi in 0..gn - 1 {
            let (a, b, c, d) = (gidx(gi, gj), gidx(gi + 1, gj), gidx(gi + 1, gj + 1), gidx(gi, gj + 1));
            let tris: [[usize; 3]; 2] = if flip { [[a, b, c], [a, c, d]] } else { [[a, c, b], [a, d, c]] };
            for t in tris {
                let n = (gpos[t[1]] - gpos[t[0]]).cross(gpos[t[2]] - gpos[t[0]]);
                for k in t {
                    gnorm[k] += n;
                }
            }
        }
    }

    // Extract the interior slice into the vertex buffers.
    let idx = |i: usize, j: usize| (j * n1 + i) as u32;
    let mut positions = vec![[0.0f32; 3]; n1 * n1];
    let mut normals = vec![[0.0f32; 3]; n1 * n1];
    let mut colors = vec![[0.0f32; 4]; n1 * n1];
    for j in 0..n1 {
        for i in 0..n1 {
            let g = gidx(i + 1, j + 1);
            positions[idx(i, j) as usize] = gpos[g].to_array();
            normals[idx(i, j) as usize] = gnorm[g].normalize_or_zero().to_array();
            let mut col = shade(h[g]);
            if audit {
                let t = (excess[j * n1 + i] / 1000.0).clamp(0.0, 1.0);
                if t > 0.0 {
                    // Blend toward magenta (linear space): disagreement made loud.
                    col = [col[0] + (1.0 - col[0]) * t, col[1] * (1.0 - t), col[2] + (1.0 - col[2]) * t, 1.0];
                }
            }
            colors[idx(i, j) as usize] = col;
        }
    }
    let mut indices = Vec::with_capacity(nx * nx * 6);
    for j in 0..nx {
        for i in 0..nx {
            let (a, b, c, d) = (idx(i, j), idx(i + 1, j), idx(i + 1, j + 1), idx(i, j + 1));
            if flip {
                indices.extend_from_slice(&[a, b, c, a, c, d]);
            } else {
                indices.extend_from_slice(&[a, c, b, a, d, c]);
            }
        }
    }

    // Perimeter SKIRTS (chunked-LOD standard practice): the two faces meeting at
    // a cube edge compute the shared corners through different trig/permutation
    // paths, so positions can disagree by a last ulp — a sub-pixel hairline that
    // shows as background-black speckle. A short inward ribbon behind each edge
    // makes any such crack show terrain colour instead. Both windings are
    // emitted (the ribbon is seen from face-dependent sides), and the parent
    // vertex's normal/colour carry over so lighting stays continuous.
    let drop_km = 60.0;
    let edges: [Vec<u32>; 4] = [
        (0..n1).map(|i| idx(i, 0)).collect(),
        (0..n1).map(|i| idx(i, n1 - 1)).collect(),
        (0..n1).map(|j| idx(0, j)).collect(),
        (0..n1).map(|j| idx(n1 - 1, j)).collect(),
    ];
    for edge in edges {
        let base = positions.len() as u32;
        for &v in &edge {
            let p = Vec3::from(positions[v as usize]);
            let sunk = p * (1.0 - drop_km / p.length().max(1.0));
            positions.push(sunk.to_array());
            normals.push(normals[v as usize]);
            colors.push(colors[v as usize]);
        }
        for k in 0..edge.len() - 1 {
            let (a, b) = (edge[k], edge[k + 1]);
            let (sa, sb) = (base + k as u32, base + k as u32 + 1);
            indices.extend_from_slice(&[a, b, sb, a, sb, sa, a, sb, b, a, sa, sb]);
        }
    }

    (FaceMesh { positions, normals, colors, indices }, seam)
}

/// The worker: owns the World (store + seed), serves (level, exag) build requests,
/// one at a time, latest result wins on the ECS side. Faces build in parallel
/// (they share only the Store, which is safe: worst case two threads compute the
/// same object and the put is idempotent).
fn spawn_worker(world_dir: PathBuf, seed: u64, rx: Receiver<(u8, f32, bool)>, tx: Sender<GlobeMsg>) {
    std::thread::spawn(move || {
        let store = match Store::open(&world_dir) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[globe] cannot open store at {}: {e}", world_dir.display());
                return;
            }
        };
        let world = World::new(&store, seed);
        while let Ok((level, exag, audit)) = rx.recv() {
            let t0 = std::time::Instant::now();
            let nx = 1usize << level;
            let mut computed = 0usize;
            let mut hits = 0usize;
            let mut land = 0usize;
            let mut total = 0usize;
            let mut tiles = Vec::with_capacity(6);
            let mut seam = SeamStats::default();
            let faces: Vec<FaceMesh> = std::thread::scope(|s| {
                let world = &world;
                let handles: Vec<_> = (0u8..6)
                    .map(|f| {
                        s.spawn(move || {
                            let face = Face::from_index(f);
                            let (tile, src) = world.spine_tile(face, level, 0, 0, nx);
                            let land = tile.iter().filter(|&&h| h as f64 > SEA_LEVEL_M).count();
                            let (mesh, fseam) = build_face(world, face, level, &tile, exag, audit);
                            (mesh, tile, src, land, fseam)
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|h| {
                        let (fm, tile, src, l, fseam) = h.join().expect("face build panicked");
                        match src {
                            Source::Computed => computed += 1,
                            Source::Hit => hits += 1,
                        }
                        land += l;
                        total += tile.len();
                        tiles.push(tile);
                        seam.cross_max = seam.cross_max.max(fseam.cross_max);
                        seam.within_max = seam.within_max.max(fseam.within_max);
                        seam.cross_sum += fseam.cross_sum;
                        seam.within_sum += fseam.within_sum;
                        seam.n += fseam.n;
                        fm
                    })
                    .collect()
            });
            let msg = GlobeMsg {
                level,
                exag,
                faces,
                tiles,
                computed,
                hits,
                pull_s: t0.elapsed().as_secs_f32(),
                land_frac: land as f32 / total.max(1) as f32,
                seam,
            };
            if tx.send(msg).is_err() {
                return; // view closed
            }
        }
    });
}

// --- ECS resources -------------------------------------------------------------------

/// Orbit-camera state — the *only* state the view owns besides pulled meshes.
#[derive(Resource)]
struct Orbit {
    yaw: f32,
    pitch: f32,
    /// Distance from planet centre, km.
    dist: f32,
    /// Drag inertia (rad/s) — the globe keeps spinning when you let go.
    vel_yaw: f32,
    vel_pitch: f32,
}

impl Default for Orbit {
    fn default() -> Self {
        // VIVARIUM_DIST (km from centre) frames scripted/verification shots;
        // interactive sessions just start at the whole-globe view.
        let dist = std::env::var("VIVARIUM_DIST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3.0 * radius_km());
        Orbit { yaw: -0.9, pitch: 0.42, dist, vel_yaw: 0.0, vel_pitch: 0.0 }
    }
}

/// Ethereal time-freedom over the Phase-1 sky (Joseph's ask, 2026-07-10). The
/// analytic regime means ANY moment is queryable — the ephemeris is a pure
/// function of (day, hour), so scrubbing time is a *view* freedom touching no
/// world state ("declare causally, materialize lazily" cashing out as UX). An
/// ethereal exo viewer sees the sun even though the in-world Phase-2 sky is
/// diffuse (the clouds part at Phase 4): noumenal access, honestly labeled.
#[derive(Resource)]
struct SunEphemeris {
    /// Day of year, 0..365.25 — 0 = northern vernal equinox (planet.rs convention).
    day: f32,
    /// Solar time at longitude 0, hours 0..24.
    hour: f32,
    /// Diurnal playback, solar hours per real second (0 = paused).
    play: f32,
    /// The old camera-following light (a stated view convenience, no day/night
    /// claim) — Y toggles back to it.
    headlight: bool,
}

impl Default for SunEphemeris {
    fn default() -> Self {
        // June-solstice-ish mid-morning: the obliquity is plainly visible in the
        // terminator's slant and the sunlit pole.
        SunEphemeris { day: 91.0, hour: 10.0, play: 0.0, headlight: false }
    }
}

/// World-frame unit vector toward the sun — the same declination/hour-angle
/// identities as `planet.rs::insolation`/`sun_direction_enu` (kept in lockstep;
/// the subsolar point is where the local hour angle is zero, i.e. lon =
/// 2π(0.5 − day_fraction)), on the +Y-pole frame `Geo` uses.
fn sun_world_dir(eph: &SunEphemeris) -> Vec3 {
    let tilt = Planet::EARTH.axial_tilt_rad;
    let yf = (eph.day as f64 / 365.25).rem_euclid(1.0);
    let decl = tilt * (std::f64::consts::TAU * yf).sin();
    let df = (eph.hour as f64 / 24.0).rem_euclid(1.0);
    let slon = std::f64::consts::TAU * (0.5 - df);
    Vec3::new((decl.cos() * slon.cos()) as f32, decl.sin() as f32, (decl.cos() * slon.sin()) as f32)
}

/// Stats of the last landed build, for the HUD.
struct BuiltStats {
    level: u8,
    exag: f32,
    computed: usize,
    hits: usize,
    pull_s: f32,
    land_frac: f32,
    seam: SeamStats,
}

/// What the globe should be showing vs what it is showing.
#[derive(Resource)]
struct GlobeState {
    auto_level: bool,
    level: u8,
    exag_i: usize,
    /// Seam-audit paint mode (S): cross-face disagreement in magenta.
    audit: bool,
    /// (level, exag, audit) of the last *requested* build; None until first request.
    requested: Option<(u8, f32, bool)>,
    inflight: bool,
    built: Option<BuiltStats>,
}

impl Default for GlobeState {
    fn default() -> Self {
        // VIVARIUM_AUDIT=1 starts in seam-audit paint mode (scripted verification).
        let audit = std::env::var("VIVARIUM_AUDIT").map(|v| v == "1").unwrap_or(false);
        GlobeState { auto_level: true, level: 8, exag_i: 2, audit, requested: None, inflight: false, built: None }
    }
}

#[derive(Resource)]
struct BuildTx(Sender<(u8, f32, bool)>);
#[derive(Resource)]
struct BuildRx(Mutex<Receiver<GlobeMsg>>);
/// The viewed world's identity, for the HUD (name + seed from its manifest).
#[derive(Resource)]
struct WorldIdent {
    name: String,
    seed: u64,
}

/// The last landed build's pulled tiles (level, face-indexed) — the coordinate
/// pick reads elevation out of these, i.e. out of the query results themselves.
#[derive(Resource, Default)]
struct TilesRes(Option<(u8, Vec<Vec<f32>>)>);

#[derive(Component)]
struct FaceEntity;
#[derive(Component)]
struct GlobeCam;
#[derive(Component)]
struct SunLight;
#[derive(Component)]
struct HudText;

fn world_dir() -> PathBuf {
    if let Ok(p) = std::env::var("VIVARIUM_WORLD") {
        return PathBuf::from(p);
    }
    let cache = std::env::var("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".cache")
    });
    cache.join("vivarium").join("globe-world")
}

fn main() {
    // Open (or found) the world: manifest first — the seed lives there and only
    // there (spec.rs: the one place a bare seed is handled), then hand it to the
    // worker inside a `World`.
    let dir = world_dir();
    let spec = WorldSpec::load_or_create(&dir, "first-light").unwrap_or_else(|e| {
        eprintln!("[globe] cannot open world at {}: {e}", dir.display());
        std::process::exit(1);
    });
    println!(
        "[globe] world \"{}\" (seed {:016x}) at {} — the store IS the save; copy the dir and the world moves.",
        spec.name,
        spec.seed,
        dir.display()
    );
    let (req_tx, req_rx) = std::sync::mpsc::channel::<(u8, f32, bool)>();
    let (res_tx, res_rx) = std::sync::mpsc::channel::<GlobeMsg>();
    spawn_worker(dir.clone(), spec.seed, req_rx, res_tx);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vivarium — globe (fBm spine, pre-erosion)".into(),
                resolution: bevy::window::WindowResolution::new(1280, 800),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(SPACE))
        .insert_resource(Orbit::default())
        .insert_resource(GlobeState::default())
        .insert_resource(BuildTx(req_tx))
        .insert_resource(BuildRx(Mutex::new(res_rx)))
        .insert_resource(WorldIdent { name: spec.name, seed: spec.seed })
        .insert_resource(TilesRes::default())
        .insert_resource(SunEphemeris::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (input_update, level_update, apply_builds, camera_update, hud_update, maybe_screenshot))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 45f32.to_radians(),
            near: 5.0,          // km — closest approach is ~120 km altitude
            far: 90_000.0,      // km — well past the farthest orbit
            ..default()
        }),
        Transform::default(),
        GlobeCam,
    ));
    // Sun + cool fill, steered to follow the camera (Google-Earth-style "the side
    // you look at is lit" — a view convenience, stated as such; no day/night claim).
    commands.spawn((
        DirectionalLight { color: Color::srgb(1.0, 0.97, 0.90), illuminance: 12_000.0, shadows_enabled: false, ..default() },
        Transform::default(),
        SunLight,
    ));
    commands.insert_resource(GlobalAmbientLight { color: Color::srgb(0.65, 0.72, 0.85), brightness: 240.0, affects_lightmapped_meshes: true });

    commands.spawn((
        Text::new("pulling the spine..."),
        TextFont { font_size: 14.0, ..default() },
        TextColor(Color::srgb(0.85, 0.87, 0.90)),
        Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(10.0), padding: UiRect::all(Val::Px(6.0)), ..default() },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)),
        HudText,
    ));

    println!("[globe] drag spin · wheel zoom · [ ] level · A auto-level · X relief · ,/. hour · N/M day · P play · Y headlight · R reset · Esc quit");
}

/// Google-Earth verbs: drag spins (with inertia), wheel zooms toward the surface.
fn input_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut orbit: ResMut<Orbit>,
    mut state: ResMut<GlobeState>,
    mut eph: ResMut<SunEphemeris>,
    mut exit: MessageWriter<AppExit>,
) {
    let dt = time.delta_secs().max(1e-4);
    let r = radius_km();

    // Drag → spin. Sensitivity scales with altitude so it always feels like
    // grabbing the surface under the cursor.
    let mut d = Vec2::ZERO;
    for m in motion.read() {
        d += m.delta;
    }
    let grab = 0.0022 * ((orbit.dist - r) / r).clamp(0.03, 2.5);
    if buttons.pressed(MouseButton::Left) && d != Vec2::ZERO {
        // Grab-the-surface semantics, sign set EMPIRICALLY (Joseph, 2026-07-10:
        // dragging left must take the globe's front left, not the rear): the
        // front face follows the cursor. Resolution of the sign's history: the
        // original sign was tuned against the INSIDE-OUT globe (the winding
        // bug), whose mirror flips apparent chirality — two bugs masking each
        // other; fixing the winding made the drag wrong. If camera_update's
        // yaw→eye mapping ever changes, re-verify by dragging.
        orbit.yaw += d.x * grab;
        orbit.pitch += d.y * grab;
        orbit.vel_yaw = d.x * grab / dt;
        orbit.vel_pitch = d.y * grab / dt;
    } else {
        // Inertia: keep spinning, decay smoothly.
        orbit.yaw += orbit.vel_yaw * dt;
        orbit.pitch += orbit.vel_pitch * dt;
        let decay = (-3.0 * dt).exp();
        orbit.vel_yaw *= decay;
        orbit.vel_pitch *= decay;
    }
    // Arrow keys spin too (constant angular rate feels right for keys).
    let key_rate = 1.2 * dt * ((orbit.dist - r) / r).clamp(0.08, 1.0).max(0.15);
    // Arrows mirror the drag semantics (left arrow ≈ dragging left).
    if keys.pressed(KeyCode::ArrowLeft) {
        orbit.yaw -= key_rate;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        orbit.yaw += key_rate;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        orbit.pitch += key_rate;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        orbit.pitch -= key_rate;
    }
    orbit.pitch = orbit.pitch.clamp(-1.55, 1.55);

    // Ethereal time scrub (the Phase-1 sky): ,/. hour · N/M day · P play · Y headlight.
    if keys.pressed(KeyCode::Comma) {
        eph.hour -= 6.0 * dt;
    }
    if keys.pressed(KeyCode::Period) {
        eph.hour += 6.0 * dt;
    }
    if keys.pressed(KeyCode::KeyN) {
        eph.day -= 40.0 * dt;
    }
    if keys.pressed(KeyCode::KeyM) {
        eph.day += 40.0 * dt;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        eph.play = if eph.play == 0.0 { 2.0 } else { 0.0 };
    }
    if keys.just_pressed(KeyCode::KeyY) {
        eph.headlight = !eph.headlight;
    }
    eph.hour += eph.play * dt;
    while eph.hour >= 24.0 {
        eph.hour -= 24.0;
        eph.day += 1.0;
    }
    while eph.hour < 0.0 {
        eph.hour += 24.0;
        eph.day -= 1.0;
    }
    eph.day = eph.day.rem_euclid(365.25);

    // Zoom: exponential in altitude — each notch takes the same *fraction* of the
    // remaining distance to the surface, so the approach never overshoots.
    let mut scroll = 0.0f32;
    for w in wheel.read() {
        scroll += match w.unit {
            MouseScrollUnit::Line => w.y * 1.0,
            MouseScrollUnit::Pixel => w.y * 0.02,
        };
    }
    if keys.pressed(KeyCode::Equal) {
        scroll += 2.2 * dt;
    }
    if keys.pressed(KeyCode::Minus) {
        scroll -= 2.2 * dt;
    }
    if scroll != 0.0 {
        let alt = orbit.dist - r;
        orbit.dist = r + (alt * (1.12f32).powf(-scroll)).clamp(120.0, 7.0 * r);
    }

    if keys.just_pressed(KeyCode::KeyR) {
        *orbit = Orbit::default();
    }
    if keys.just_pressed(KeyCode::KeyX) {
        state.exag_i = (state.exag_i + 1) % EXAG_STEPS.len();
    }
    if keys.just_pressed(KeyCode::KeyS) {
        state.audit = !state.audit;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        state.auto_level = true;
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        state.auto_level = false;
        state.level = state.level.saturating_sub(1).max(LEVEL_MIN);
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        state.auto_level = false;
        state.level = (state.level + 1).min(LEVEL_MAX);
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

/// Resolution-on-zoom: pick the sampling level from altitude (finer spine levels
/// as you approach — each level's fBm runs more octaves, so detail is *pulled*,
/// not interpolated), and request a rebuild whenever (level, exag) drifts from
/// what is built. One build in flight; the desired-vs-built check re-fires until
/// they agree, so latest always wins.
fn level_update(orbit: Res<Orbit>, mut state: ResMut<GlobeState>, tx: Res<BuildTx>) {
    if state.auto_level {
        let r = radius_km();
        let alt = (orbit.dist - r).max(120.0);
        let quarter = r * std::f32::consts::FRAC_PI_2; // face edge arc, km
        let target_cell = alt * 0.005; // ~cells subtend ≲0.3° from the camera
        let l = (quarter / target_cell).log2().ceil() as i32;
        state.level = l.clamp(LEVEL_MIN as i32, LEVEL_MAX as i32) as u8;
    }
    let want = (state.level, EXAG_STEPS[state.exag_i], state.audit);
    if !state.inflight && state.requested != Some(want) {
        if tx.0.send(want).is_ok() {
            state.requested = Some(want);
            state.inflight = true;
        }
    }
}

/// Fold a completed build into the scene: swap all six face meshes at once (no
/// mixed-level flicker) and record the pull stats for the HUD.
fn apply_builds(
    mut commands: Commands,
    rx: Res<BuildRx>,
    mut state: ResMut<GlobeState>,
    mut tiles_res: ResMut<TilesRes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    old: Query<Entity, With<FaceEntity>>,
) {
    let Ok(msg) = rx.0.lock().unwrap().try_recv() else {
        return;
    };
    state.inflight = false;
    tiles_res.0 = Some((msg.level, msg.tiles));
    for e in &old {
        commands.entity(e).despawn();
    }
    let mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.92,
        reflectance: 0.08,
        ..default()
    });
    for fm in msg.faces {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, fm.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, fm.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, fm.colors);
        mesh.insert_indices(Indices::U32(fm.indices));
        commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(mat.clone()), Transform::default(), FaceEntity));
    }
    state.built = Some(BuiltStats {
        level: msg.level,
        exag: msg.exag,
        computed: msg.computed,
        hits: msg.hits,
        pull_s: msg.pull_s,
        land_frac: msg.land_frac,
        seam: msg.seam,
    });
}

fn camera_update(
    orbit: Res<Orbit>,
    eph: Res<SunEphemeris>,
    mut cam: Query<&mut Transform, (With<GlobeCam>, Without<SunLight>)>,
    mut sun: Query<&mut Transform, (With<SunLight>, Without<GlobeCam>)>,
) {
    let dir = Vec3::new(
        orbit.pitch.cos() * orbit.yaw.cos(),
        orbit.pitch.sin(),
        orbit.pitch.cos() * orbit.yaw.sin(),
    );
    let eye = dir * orbit.dist;
    let t = Transform::from_translation(eye).looking_at(Vec3::ZERO, Vec3::Y);
    if let Ok(mut c) = cam.single_mut() {
        *c = t;
    }
    if let Ok(mut s) = sun.single_mut() {
        if eph.headlight {
            // The old view convenience: light rides the camera, viewed side lit.
            s.rotation = t.rotation * Quat::from_rotation_y(0.55) * Quat::from_rotation_x(-0.45);
        } else {
            // The REAL sky: light travels from the subsolar direction toward the
            // planet, so the terminator, the seasons, and polar day/night are the
            // planet's actual Phase-1 rhythms, not stagecraft.
            *s = Transform::default().looking_to(-sun_world_dir(&eph), Vec3::Y);
        }
    }
}

/// Cursor ray → planet-sphere hit → surface direction. Pure: this is the same
/// math a "point at the globe to plant a beacon" flow will reuse (the pick is
/// against the sea-level sphere, so on exaggerated tall land the picked cell can
/// be a cell or two off at grazing angles — honest limit of picking a datum
/// sphere; pick against the relief mesh if that ever matters).
fn pick_direction(camera: &Camera, cam_gt: &GlobalTransform, cursor: Vec2, r: f32) -> Option<[f64; 3]> {
    let ray = camera.viewport_to_world(cam_gt, cursor).ok()?;
    let (o, d) = (ray.origin, *ray.direction);
    let b = o.dot(d);
    let disc = b * b - (o.dot(o) - r * r);
    if disc < 0.0 {
        return None; // cursor misses the planet
    }
    let t = -b - disc.sqrt();
    if t <= 0.0 {
        return None;
    }
    let p = (o + d * t).normalize();
    Some([p.x as f64, p.y as f64, p.z as f64])
}

fn hud_update(
    orbit: Res<Orbit>,
    eph: Res<SunEphemeris>,
    state: Res<GlobeState>,
    ident: Res<WorldIdent>,
    tiles: Res<TilesRes>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform), With<GlobeCam>>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    let Ok(mut text) = hud.single_mut() else {
        return;
    };
    let r = radius_km();
    let alt = orbit.dist - r;

    // The coordinate readout (Joseph's ask): both registers — the CellId-ish
    // (face, level, i, j) that beacons/foci and VIVARIUM_FOCUS_I/J speak, and
    // lat/lon + elevation for human intuition. Elevation comes from the pulled
    // tile itself (the query result), not a recomputation.
    let pick_line = (|| {
        let cursor = windows.iter().next()?.cursor_position()?;
        let (camera, gt) = cam.iter().next()?;
        let dir = pick_direction(camera, gt, cursor, r)?;
        let (lvl, tiles) = tiles.0.as_ref()?;
        let cc = CubeCoord::from_unit(dir);
        let (face, i, j, _) = cc.cell(*lvl).to_face_ij();
        let nx = 1usize << lvl;
        let h = tiles[face.index() as usize][j as usize * nx + i as usize];
        let rel = h - SEA_LEVEL_M as f32;
        let g = cc.to_geo();
        let (glat, glon) = (g.lat.to_degrees(), g.lon.to_degrees());
        Some(format!(
            "pick F{} L{} i={} j={} | {:.2}{} {:.2}{} | {:+.0} m {}",
            face.index(),
            lvl,
            i,
            j,
            glat.abs(),
            if glat >= 0.0 { "N" } else { "S" },
            glon.abs(),
            if glon >= 0.0 { "E" } else { "W" },
            rel,
            if rel > 0.0 { "(land)" } else { "(sea floor)" },
        ))
    })()
    .unwrap_or_else(|| "pick: point at the globe".to_string());
    // The surface point under screen centre, in human coordinates.
    let dir = Vec3::new(
        orbit.pitch.cos() * orbit.yaw.cos(),
        orbit.pitch.sin(),
        orbit.pitch.cos() * orbit.yaw.sin(),
    );
    let geo = CubeCoord::from_unit([dir.x as f64, dir.y as f64, dir.z as f64]).to_geo();
    let (lat, lon) = (geo.lat.to_degrees(), geo.lon.to_degrees());
    // ASCII only: Bevy's default font has no glyphs for middle-dot/degree/etc.
    let status = match (&state.built, state.inflight) {
        (None, _) => "pulling the spine...".to_string(),
        (Some(b), inflight) => {
            let cell_km = (r * std::f32::consts::FRAC_PI_2) / (1u32 << b.level) as f32;
            let s = &b.seam;
            let (c_mean, w_mean) = (
                (s.cross_sum / s.n.max(1) as f64) as f32,
                (s.within_sum / s.n.max(1) as f64) as f32,
            );
            format!(
                "world \"{}\" (seed {:016x}) | spine L{} | cell ~{cell_km:.0} km | pull {} computed / {} hit, {:.2} s | land ~{:.0}%{}\n\
                 alt {alt:.0} km | centre {:.1}{} {:.1}{} | relief x{:.0} (1 = honest) | level {}\n\
                 {pick_line}\n\
                 face-seam dh: cross {c_mean:.0} m mean, {:.0} m max | within-face {w_mean:.0} m mean, {:.0} m max{}\n\
                 fBm surface prior -- placeholder relief: conserves nothing, no tectonics/erosion yet\n\
                 drag spin | wheel zoom | [ ] level | A auto | X relief | S seam audit | R reset | Esc quit",
                ident.name,
                ident.seed,
                b.level,
                b.computed,
                b.hits,
                b.pull_s,
                b.land_frac * 100.0,
                if inflight { " | rebuilding..." } else { "" },
                lat.abs(),
                if lat >= 0.0 { "N" } else { "S" },
                lon.abs(),
                if lon >= 0.0 { "E" } else { "W" },
                b.exag,
                if state.auto_level { "auto".to_string() } else { format!("manual L{}", state.level) },
                s.cross_max,
                s.within_max,
                if state.audit { " | AUDIT PAINT ON" } else { "" },
            )
        }
    };
    let yf = (eph.day as f64 / 365.25).rem_euclid(1.0);
    let decl_deg = (Planet::EARTH.axial_tilt_rad * (std::f64::consts::TAU * yf).sin()).to_degrees();
    let sun_line = if eph.headlight {
        "sun: HEADLIGHT (view convenience, no day/night claim) | Y = real ephemeris".to_string()
    } else {
        format!(
            "sun: day {:.0}/365 {:04.1}h @lon0 | subsolar lat {:+.1} deg (axial tilt 23.44 deg) | {} | ,/. hour N/M day P play Y headlight",
            eph.day,
            eph.hour,
            decl_deg,
            if eph.play != 0.0 { "PLAYING" } else { "paused" },
        )
    };
    text.0 = format!("{status}\n{sun_line}");
}

/// The worldview verification idiom: VIVARIUM_AUTOSHOT=1 waits for the first
/// build to land (plus a settle), saves a screenshot (VIVARIUM_SHOT or /tmp),
/// and exits — so a session can *see* the globe it shipped.
fn maybe_screenshot(
    time: Res<Time>,
    state: Res<GlobeState>,
    mut commands: Commands,
    mut armed_at: Local<Option<f32>>,
    mut shot: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    if armed_at.is_none() && state.built.is_some() {
        *armed_at = Some(t);
    }
    let Some(t0) = *armed_at else {
        return;
    };
    if !*shot && t > t0 + 1.0 {
        let path = PathBuf::from(std::env::var("VIVARIUM_SHOT").unwrap_or_else(|_| "/tmp/vivarium_globe_shot.png".into()));
        eprintln!("[globe] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > t0 + 2.5 {
        exit.write(AppExit::Success);
    }
}
