//! **The pull** — the explorer's entire knowledge of the world flows through
//! this one thread, and it holds the only store handle in the process.
//!
//! That handle is read-only ( `Store::open_read_only` ), so the wall is not a
//! rule this module has to remember: the store refuses the write and counts the
//! refusal, and the HUD shows the count. The spike this replaces had exactly one
//! wall violation and it was here — a warmer thread calling
//! `World::epoch_reduction`, which computes *and puts* on a miss, so an explorer
//! left open on an unbuilt world quietly materialized the cooling ladder.
//!
//! What replaces it is not "render nothing unbuilt". It is the honest split the
//! store was already able to report and nothing displayed:
//!
//! - a stage the builder materialized is a **store citizen** — the world's own
//!   answer, arriving in microseconds;
//! - a stage it did not is **view-computed** — correct, off the frame path, and
//!   written nowhere, so it evaporates when the window closes.
//!
//! Both are drawable. Only the first is the world's. The timeline paints them
//! differently and the HUD names which one you are looking at, which makes
//! `vivarium build` *visible* rather than optional magic — the thing a strict
//! built-only view would achieve by showing a black screen.

use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use vivarium_world::lithosphere;
use vivarium_world::query::World;
use vivarium_world::sea_level;
use vivarium_world::sphere::{CellId, CubeCoord, Face};
use vivarium_world::store::Store;
use vivarium_world::watch::{self, BuildState, Coverage, TileFlags};

use crate::lens::{Chain, FrameFacts, Ladder, Lens, SeaProvenance};
use crate::mesh::{self, FaceInput, FaceMesh, SeamStats};
use crate::paint::Paint;
use crate::water::{WaterField, WET_M};

/// One build request. Equality is what suppresses redundant rebuilds, so every
/// field here is something a *different picture* depends on.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Request {
    pub level: u8,
    /// `None` = the whole globe, six faces. `Some` = **one window into one
    /// face**, which is how anything finer than a whole-face monolith gets
    /// drawn: L13 is 8192² cells per face, and six of those is not a mesh, it is
    /// a memory error with a nice comment.
    ///
    /// The window is not a different renderer. It is the same mesher with a
    /// non-zero origin ( `mesh::FaceInput` ), because a globe path and a region
    /// path that share no meshing code will disagree about the world, and the
    /// disagreement will look like terrain.
    pub patch: Option<Patch>,
    pub exag: f32,
    pub paint: Paint,
    pub lens: Lens,
    /// Which settle history is on the time axis (index into `Chain::all`).
    /// A world can hold more than one, and which you want is not inferable.
    pub cohort: usize,
    /// Full-scale for the change ramp (m) — a different scale is a different
    /// picture, so it belongs in the equality that suppresses rebuilds.
    pub change_scale_m: f32,
}

/// A window `nx × nx` into one face's cell grid at `(oi, oj)`, at the request's
/// level.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Patch {
    pub face: u8,
    pub oi: u32,
    pub oj: u32,
    pub nx: usize,
}

impl Patch {
    /// The window of width `nx` centred on face cell `(i, j)`, slid to stay
    /// inside the face. Sliding rather than clipping keeps the window a constant
    /// size, so the mesh cost and the cell scale do not change as you approach an
    /// edge — a view whose resolution silently drops near a face boundary would
    /// be an artefact of the chart, drawn as if it were the world.
    pub fn centred(face: u8, level: u8, i: u32, j: u32, nx: usize) -> Patch {
        let face_n = 1u32 << level;
        let nx = nx.min(face_n as usize);
        let half = nx as u32 / 2;
        let hi = face_n - nx as u32;
        Patch { face, oi: i.saturating_sub(half).min(hi), oj: j.saturating_sub(half).min(hi), nx }
    }
}

/// A completed frame: six meshes plus everything the HUD needs to describe them.
pub struct Frame {
    pub req: Request,
    pub faces: Vec<FaceMesh>,
    /// The pulled tiles, kept so the cursor pick reports elevation from the same
    /// queried data the meshes were built from — never a second computation that
    /// could drift from what is on screen.
    pub tiles: Vec<Vec<f32>>,
    pub seam: SeamStats,
    pub facts: FrameFacts,
    /// Roots this frame's census was read from, for the honesty block.
    pub roots: Vec<vivarium_world::store::RootEntry>,
    /// Freshly-observed ladder residency (a builder in another terminal may have
    /// landed stages since the last frame).
    pub ladder_built: Vec<bool>,
    /// The settle history as the worker last censused it — the chain grows while
    /// a builder runs, so the scrub's own axis has to age with the store.
    pub chain: Chain,
}

/// News the worker sends without a full rebuild.
pub enum Msg {
    Frame(Box<Frame>),
    /// The store's root count changed — a builder is working.
    Landings(usize),
}

/// Deep-time surfaces already built, keyed by `(face, level, T_p bits)`.
///
/// Playback advances only when the next stage's surface lands, so this
/// evaluation *is* the frame rate — measured at 113 ms per whole-globe frame at
/// L8, 9.2 s for an 81-stage lap (`examples/epoch_surface_timing`). Playback
/// loops, so without a cache every lap re-pays the whole thing to redraw
/// surfaces it has already drawn. The ladder is a fixed, finite, fated set: the
/// same `(face, level, T_p)` is bit-identical every time it is asked for, which
/// is exactly the condition under which caching is free of consequence.
///
/// A **view-side working set, not a durability tier** — the same distinction
/// `#form-store-as-save` FE(6) draws for the reduction caches. It holds nothing
/// the store could not regenerate, and dropping it costs only recomputation.
#[derive(Default)]
struct StageSurfaceCache {
    tiles: Mutex<HashMap<(u8, u8, u64), Vec<f32>>>,
}

impl StageSurfaceCache {
    /// A lap at L8 is 6 faces × 65 536 cells × 4 B × 81 stages ≈ 127 MB — enough
    /// to matter on a laptop also running a renderer. Clearing wholesale rather
    /// than evicting one entry keeps a lap coherent: a half-evicted ladder
    /// stutters unpredictably mid-sweep, which reads as a worse bug than a
    /// uniformly slower one.
    const MAX_CELLS: usize = 48 * 1024 * 1024 / 4;

    fn get_or_build(&self, seed: u64, face: Face, level: u8, nx: usize, tp: f64) -> Vec<f32> {
        let key = (face.index(), level, tp.to_bits());
        if let Some(hit) = self.tiles.lock().unwrap().get(&key) {
            return hit.clone();
        }
        let tile = stage_surface_tile(seed, face, level, nx, tp);
        let mut map = self.tiles.lock().unwrap();
        if map.len() * nx * nx > Self::MAX_CELLS {
            map.clear();
        }
        map.insert(key, tile.clone());
        tile
    }
}

/// One whole-face height tile at a deep-time stage — the stage's tectonic
/// surface (bathymetry + the freeboard that mantle temperature earns) sampled
/// per cell. A pure read of the cooling-chain law: observe-only, never a store
/// write, never a fluvial run.
fn stage_surface_tile(seed: u64, face: Face, level: u8, nx: usize, tp: f64) -> Vec<f32> {
    let mut tile = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(face, i, j, level);
            tile.push(sea_level::tectonic_surface_at_tp(seed, cell, level, tp) as f32);
        }
    }
    tile
}

/// Whole-face uncarved prior at `level` — O(nx²) fBm, no region archive walk.
fn prior_face_tile(seed: u64, face: Face, level: u8, nx: usize) -> Vec<f32> {
    let mut tile = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(face, i, j, level);
            tile.push(vivarium_world::gen::initial_topography_m(seed, cell, level) as f32);
        }
    }
    tile
}

/// Whether a store key's tile can contribute heights to a view patch — pure key
/// parse, no payload decode.
fn key_overlaps_patch(key: &str, p: Patch, view_level: u8) -> bool {
    let face = match watch::key_field(key, "face").and_then(|v| v.parse::<u8>().ok()) {
        Some(f) => f,
        None => return false,
    };
    if face != p.face {
        return false;
    }
    let level = match watch::key_field(key, "level").and_then(|v| v.parse::<u8>().ok()) {
        Some(l) => l,
        None => return false,
    };
    let oi = match watch::key_field(key, "oi").and_then(|v| v.parse::<u32>().ok()) {
        Some(v) => v,
        None => return false,
    };
    let oj = match watch::key_field(key, "oj").and_then(|v| v.parse::<u32>().ok()) {
        Some(v) => v,
        None => return false,
    };
    let nx = match watch::key_field(key, "nx").and_then(|v| v.parse::<u32>().ok()) {
        Some(v) => v,
        None => return false,
    };
    let (roi, roj, rw, rh) = bounds_at_level(level, oi, oj, nx, view_level);
    let (poi, poj, pw, ph) = (p.oi, p.oj, p.nx as u32, p.nx as u32);
    // Expand so halo/ghost edges still find a neighbour region.
    let expand = 2u32;
    let roi = roi.saturating_sub(expand);
    let roj = roj.saturating_sub(expand);
    let rw = rw + 2 * expand;
    let rh = rh + 2 * expand;
    roi < poi + pw && roi + rw > poi && roj < poj + ph && roj + rh > poj
}

/// Axis-aligned bounds of a square tile, re-expressed at `to_level`.
fn bounds_at_level(from_level: u8, oi: u32, oj: u32, nx: u32, to_level: u8) -> (u32, u32, u32, u32) {
    if to_level >= from_level {
        let s = 1u32 << (to_level - from_level);
        (oi * s, oj * s, nx * s, nx * s)
    } else {
        let s = 1u32 << (from_level - to_level);
        (oi / s, oj / s, (nx + s - 1) / s, (nx + s - 1) / s)
    }
}

/// Spawn the worker. It owns the store handle for the process's lifetime; the
/// ECS side owns nothing but meshes and camera state ( #form-core-view-wall
/// FE(2): a view may hold camera state, meshes, and HUD state, and no
/// authoritative world state).
pub fn spawn(
    dir: PathBuf,
    seed: u64,
    demanded_frames: u32,
    view_frames: u32,
    rx: Receiver<Request>,
    tx: Sender<Msg>,
) {
    std::thread::spawn(move || {
        let store = match Store::open_read_only(&dir) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[explore] cannot open store at {}: {e}", dir.display());
                return;
            }
        };
        let world = World::new(&store, seed);
        let mut ladder = Ladder::read(&world, demanded_frames, view_frames);
        let stage_cache = StageSurfaceCache::default();

        // Census state, refreshed when the listing epoch moves (same-process put
        // generation, or entry-count fingerprint for a builder in another
        // terminal). `roots_shared` is hot: one body scan per epoch, not per
        // pull (#disc-explorer-instrument-parity P0). Landings are replay-only
        // and re-read when the census changes.
        let mut roots = store.roots_shared().unwrap_or_else(|_| Arc::new(Vec::new()));
        let mut cov = Coverage::parse(&roots);
        let mut water = WaterField::load(&world, &cov);
        let mut landings: Vec<watch::Landing> = Vec::new();
        let mut chain = Chain::read(&roots, 0);
        crate::lens::read_residuals(&store, &roots, &mut chain);
        // Full roots/ readdir is O(archive); only for live builder watch, not every pull.
        let mut last_external_check = Instant::now();
        const EXTERNAL_CENSUS_INTERVAL: Duration = Duration::from_secs(1);

        while let Ok(first) = rx.recv() {
            // **Latest request wins.** While a multi-second mesh builds, the UI
            // keeps sending the current zoom/level. Draining the backlog here
            // means we never pay for intermediate zooms the hand already left —
            // that was the "does the wrong thing very slowly" path.
            let mut req = first;
            while let Ok(newer) = rx.try_recv() {
                req = newer;
            }
            let t0 = Instant::now();

            if last_external_check.elapsed() >= EXTERNAL_CENSUS_INTERVAL {
                let _ = store.roots_invalidate_if_external();
                last_external_check = Instant::now();
            }
            // Generation-hot listing: Arc clone when warm; body scan only on epoch move.
            let now = store.roots_shared().unwrap_or_else(|_| Arc::new(Vec::new()));
            if !Arc::ptr_eq(&now, &roots) {
                roots = now;
                cov = Coverage::parse(&roots);
                water = WaterField::load(&world, &cov);
                landings.clear();
                ladder.refresh_residency(&world);
                chain = Chain::read(&roots, chain.sel);
                crate::lens::read_residuals(&store, &roots, &mut chain);
                let _ = tx.send(Msg::Landings(roots.len()));
            }
            if !chain.all.is_empty() && req.cohort % chain.all.len() != chain.sel {
                chain = Chain::read(&roots, req.cohort);
                crate::lens::read_residuals(&store, &roots, &mut chain);
            }

            let level = req.level;
            // The unit of work: either six whole faces, or one window into one.
            // Everything downstream is written against this list, so the two
            // modes are one code path with a different list.
            let units: Vec<Patch> = match req.patch {
                Some(p) => vec![p],
                None => {
                    let n = 1usize << level;
                    (0u8..6).map(|f| Patch { face: f, oi: 0, oj: 0, nx: n }).collect()
                }
            };
            let _nx = units[0].nx;

            // --- what surface, and on what datum -----------------------------
            let tp = crate::lens::lens_tp(req.lens, &ladder);
            let (sea_m, sea_provenance) = match world.epoch_reduction_hit(tp) {
                Some((r, _)) => (r.derived_sea_m as f32, SeaProvenance::StoreCitizen),
                None => {
                    // Not built. Compute it here — off the frame path, into no
                    // store — and say so. The alternative (refuse to draw) makes
                    // a cold world a black window; the alternative the spike took
                    // (compute and PUT) makes the view a builder.
                    (sea_level::derived_sea_level_at_tp(seed, tp) as f32, SeaProvenance::ViewComputed)
                }
            };

            // The census this frame is described by. In replay it is the prefix,
            // so the overlay ages with the picture instead of describing the
            // finished world over a half-built one.
            let (frame_roots, frame_cov): (Vec<_>, Coverage) = match req.lens {
                Lens::Replay(n) => {
                    if landings.is_empty() {
                        landings = watch::landings(&dir).unwrap_or_default();
                    }
                    let rs = crate::lens::replay_roots(&landings, n);
                    let c = Coverage::parse(&rs);
                    (rs, c)
                }
                _ => (roots.as_ref().clone(), Coverage::parse(&roots)),
            };

            // **Key-side reject before decode.** Two filters, both free string
            // parses on the root list (hundreds of keys is nothing; hundreds of
            // *decoded* 64² payloads is the multi-second path):
            //
            // 1. **Level:** a region at L answers only cells at level ≥ L
            //    (`ErodedRegion::grid_pos`). L9 tiles can never cover an L7 view
            //    cell — loading them for a far globe was pure waste (open-view
            //    6+ s while painting 100% prior).
            // 2. **Patch frustum:** window mode only needs overlapping tiles.
            //
            // 400 tiles is a tiny R. The wrong family was O(C·R) with R useless,
            // not "400 is large."
            let patch = req.patch;
            let view_level = req.level;
            let key_in_view = |k: &str| -> bool {
                let Some(rl) = watch::key_field(k, "level").and_then(|v| v.parse::<u8>().ok()) else {
                    return false;
                };
                // Region level must be ≤ view level or it cannot cover any cell.
                if rl > view_level {
                    return false;
                }
                match patch {
                    None => true,
                    Some(p) => key_overlaps_patch(k, p, view_level),
                }
            };
            let regions = match req.lens {
                Lens::Present => {
                    let cur = vivarium_world::nomotheke::SRC_HASH;
                    world.observe().load_eroded_regions_where(|k| {
                        watch::key_field(k, "src") == Some(cur) && key_in_view(k)
                    })
                }
                Lens::Stage(_) => Vec::new(),
                // One world-moment: this cohort's source tree, this exact epoch.
                // Both fields are required — the epoch alone would assemble
                // stages from two different kernels into one surface, and the
                // source alone is the whole settle history at once.
                Lens::Erosion(i) => match chain.stage_predicate(i) {
                    Some((src, lvl, epoch)) => world.observe().load_eroded_regions_where(|k| {
                        watch::key_field(k, "src") == Some(src.as_str())
                            && watch::key_field(k, "level").and_then(|v| v.parse::<u8>().ok())
                                == Some(lvl)
                            && watch::key_field(k, "epochs").and_then(|v| v.parse::<u32>().ok())
                                == Some(epoch)
                            && key_in_view(k)
                    }),
                    None => Vec::new(),
                },
                Lens::Replay(n) => {
                    if landings.is_empty() {
                        landings = watch::landings(&dir).unwrap_or_default();
                    }
                    let keys: BTreeSet<String> = crate::lens::replay_key_set(&landings, n);
                    world.observe().load_eroded_regions_where(|k| keys.contains(k) && key_in_view(k))
                }
            };
            let census = world.observe().eroded_region_census();
            let pure_prior = regions.is_empty() && !matches!(req.lens, Lens::Stage(_));

            // The ghost ring lies on neighbouring faces, so it must come from the
            // same law the in-face tile came from, or the seam instrument would
            // measure the gap between two different surfaces rather than the
            // world's own discontinuity.
            let is_stage = matches!(req.lens, Lens::Stage(_));
            let regions_ref = &regions;
            let ghost = move |face: Face, ci: i64, cj: i64, level: u8| -> f32 {
                let n = 1usize << level;
                let cu = ((ci as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cv = ((cj as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let dir = CubeCoord { face, u: cu, v: cv }.to_unit();
                let cell = CubeCoord::from_unit(dir).cell(level);
                if is_stage {
                    sea_level::tectonic_surface_at_tp(seed, cell, level, tp) as f32
                } else if pure_prior || regions_ref.is_empty() {
                    vivarium_world::gen::initial_topography_m(seed, cell, level) as f32
                } else {
                    vivarium_world::erosion::surface_at_carved(seed, cell, regions_ref) as f32
                }
            };

            // --- build the units in parallel ----------------------------------
            let mut tiles: Vec<Vec<f32>> = Vec::with_capacity(6);
            let mut seam = SeamStats::default();
            let (mut land, mut total, mut prior_fallback) = (0usize, 0usize, 0usize);
            let (mut inland_water, mut water_cells) = (0usize, 0usize);
            let (mut chg_sum, mut chg_min, mut chg_max) = (0.0f64, 0.0f32, 0.0f32);
            let (mut rising, mut falling) = (0usize, 0usize);
            let mut tier_cells: std::collections::BTreeMap<u8, usize> = Default::default();
            // Depression capacity summed over the drawn units. Summing is
            // correct only because each unit is read as its own walled domain:
            // these are per-window capacities, not one planetary basin census,
            // and the HUD says so rather than implying a globe-wide total.
            let (mut dep_cells_total, mut dep_capacity_total, mut dep_deepest_total) =
                (0usize, 0.0f64, 0.0f32);

            // **Standing water is computed at the level that RAN, once per region.**
            // Not per drawn unit: a reader over the drawn surface computes a
            // derived physical quantity on a surface no rung produced — bilinear
            // over the carve plus the prior's detail increment — and the answer is
            // then mostly undrained prior dimples rather than basins (measured 8.5×
            // on `examples/lake_surface_probe`; #form-fidelity-ladder FE(7)–(9)).
            // Sampling the region's own field down to the view is a *view of the
            // rendered physics*; recomputing on the drawn surface is a second
            // physics. The region is also the wider domain, so fewer basins are cut
            // by its rim than by a tile's ( #obs-tile-outlets-grade-away-the-basins ).
            // Depression lakes computed per face inside the unit loop (face-local
            // R only). Ocean is always classified on the *assembled tile* — that
            // is the surface being painted ( #form-ocean-is-connectivity-not-elevation ).

            let faces: Vec<FaceMesh> = std::thread::scope(|s| {
                let (world, regions, cov, water, cache, ghost, units) =
                    (&world, &regions, &frame_cov, &water, &stage_cache, &ghost, &units);
                let handles: Vec<_> = units
                    .iter()
                    .map(|&unit| {
                        s.spawn(move || {
                            let (f, oi, oj, nx) = (unit.face, unit.oi, unit.oj, unit.nx);
                            let face = Face::from_index(f);
                            // Column-block spine (explore hot path): only regions
                            // on this face participate in per-cell walks.
                            let face_regions: Vec<vivarium_world::erosion::ErodedRegion> =
                                regions.iter().filter(|r| r.face == face).cloned().collect();
                            let tile = match req.lens {
                                // The deep-time cache is keyed by whole face; a
                                // window is computed directly rather than cached,
                                // since a fine window is a different key space and
                                // caching it would evict the ladder mid-sweep.
                                Lens::Stage(_) if oi == 0 && oj == 0 && nx == 1usize << level =>
                                    cache.get_or_build(seed, face, level, nx, tp),
                                Lens::Stage(_) => {
                                    let mut t = Vec::with_capacity(nx * nx);
                                    for j in 0..nx as u32 {
                                        for i in 0..nx as u32 {
                                            let c = CellId::from_face_ij(face, oi + i, oj + j, level);
                                            t.push(
                                                sea_level::tectonic_surface_at_tp(seed, c, level, tp)
                                                    as f32,
                                            );
                                        }
                                    }
                                    t
                                }
                                _ if pure_prior && oi == 0 && oj == 0 && nx == 1usize << level => {
                                    prior_face_tile(seed, face, level, nx)
                                }
                                _ if pure_prior => {
                                    let mut t = Vec::with_capacity(nx * nx);
                                    for j in 0..nx as u32 {
                                        for i in 0..nx as u32 {
                                            let c = CellId::from_face_ij(face, oi + i, oj + j, level);
                                            t.push(
                                                vivarium_world::gen::initial_topography_m(seed, c, level)
                                                    as f32,
                                            );
                                        }
                                    }
                                    t
                                }
                                _ => world
                                    .observe()
                                    .assemble_surface_tile(face, level, oi, oj, nx, &face_regions)
                                    .0,
                            };

                            // Every per-cell query below is asked in FACE cells,
                            // because that is the frame the census and the water
                            // field are indexed in; only the height tile is
                            // patch-local. Mixing the two is the whole bug class
                            // a windowed view invites, so the translation happens
                            // once, here, and is named.
                            let g = |ci: u32, cj: u32| (oi + ci, oj + cj);

                            // Per-cell provenance, sampled at the CENSUS level so
                            // a coarse view still reports the true tile boundary.
                            let to_build_level = |c: u32| -> u32 {
                                if level <= cov.level { c << (cov.level - level) } else { c >> (level - cov.level) }
                            };
                            let state = |ci: u32, cj: u32| -> (BuildState, TileFlags) {
                                if matches!(req.lens, Lens::Stage(_)) {
                                    // No tile of any nomos exists off the present
                                    // epoch. Saying "unbuilt" here is the truth,
                                    // and it is why deep time paints uniformly
                                    // grey in provenance mode.
                                    return (BuildState::Unbuilt, TileFlags::default());
                                }
                                let (gi, gj) = g(ci, cj);
                                let (bi, bj) = (to_build_level(gi), to_build_level(gj));
                                (cov.state_at_cell(f, bi, bj), cov.flags_at_cell(f, bi, bj))
                            };
                            let water_at = |ci: u32, cj: u32| -> f32 {
                                if matches!(req.lens, Lens::Stage(_)) {
                                    return 0.0; // water tiles exist only for the present
                                }
                                let (gi, gj) = g(ci, cj);
                                water.depth_at(f, gi, gj, level)
                            };

                            // The change channel's baseline: the **uncarved
                            // initial topography**, which is a pure function of
                            // the seed ( `erosion::surface_at` with no regions is
                            // exactly `gen::initial_topography_m` ). Evaluating a
                            // law the view does not author, off the frame path,
                            // writing nowhere — the same standing as the deep-time
                            // stage surfaces above. Paid only when the change
                            // paint is up, because it is one evaluation per cell.
                            let baseline: Vec<f32> = if req.paint.needs_change() {
                                let mut b = Vec::with_capacity(nx * nx);
                                for j in 0..nx as u32 {
                                    for i in 0..nx as u32 {
                                        let (gi, gj) = g(i, j);
                                        let cid = CellId::from_face_ij(face, gi, gj, level);
                                        b.push(
                                            vivarium_world::erosion::prior_at_carve_level(
                                                seed,
                                                cid,
                                                &face_regions,
                                            ) as f32,
                                        );
                                    }
                                }
                                b
                            } else {
                                Vec::new()
                            };
                            let (mut dep_cells, mut dep_capacity_m3, mut dep_deepest_m) =
                                (0usize, 0.0f64, 0.0f32);
                            // Depression: wet-limit on covering carve regions only
                            // (face-local). Pure-prior views report zero — there is
                            // no process bed to fill; the prior's closed form is not
                            // a lake census.
                            let depression: Vec<f32> = if req.paint.needs_depression()
                                && !face_regions.is_empty()
                            {
                                let face_lakes: Vec<Vec<f32>> =
                                    face_regions.iter().map(|r| r.standing_water()).collect();
                                let radius = vivarium_world::planet::Planet::EARTH.radius_m;
                                let mut out = vec![0.0f32; nx * nx];
                                for j in 0..nx {
                                    for i in 0..nx {
                                        let (gi, gj) = g(i as u32, j as u32);
                                        let cell = CellId::from_face_ij(face, gi, gj, level);
                                        for (ri, r) in face_regions.iter().enumerate().rev() {
                                            let Some(k) = r.carved_index(cell) else { continue };
                                            let d = face_lakes[ri][k];
                                            out[j * nx + i] = d;
                                            if d > 1.0 {
                                                dep_cells += 1;
                                                dep_capacity_m3 += d as f64
                                                    * vivarium_world::measure::cell_area_m2(
                                                        face, gi as u64, gj as u64, level, radius,
                                                    );
                                                dep_deepest_m = dep_deepest_m.max(d);
                                            }
                                            break;
                                        }
                                    }
                                }
                                out
                            } else {
                                Vec::new()
                            };
                            let depression_max_m =
                                depression.iter().copied().fold(0.0f32, f32::max);
                            let depression_at = |ci: u32, cj: u32| -> f32 {
                                if depression.is_empty() {
                                    return 0.0;
                                }
                                depression[cj as usize * nx + ci as usize]
                            };
                            // Ocean on the painted surface — one O(nx²) mask.
                            let ocean_tile =
                                vivarium_world::sea_level::ocean_mask(&tile, nx, sea_m);
                            let ocean_at = |ci: u32, cj: u32| -> bool {
                                ocean_tile[cj as usize * nx + ci as usize]
                            };
                            let change_at = |ci: u32, cj: u32| -> f32 {
                                if baseline.is_empty() {
                                    return 0.0;
                                }
                                tile[cj as usize * nx + ci as usize]
                                    - baseline[cj as usize * nx + ci as usize]
                            };

                            // Per-face tallies — the countable half of the frame.
                            let (mut l, mut fb, mut iw, mut wc) = (0usize, 0usize, 0usize, 0usize);
                            let (mut cs, mut cmin, mut cmax) = (0.0f64, 0.0f32, 0.0f32);
                            let (mut ri, mut fa) = (0usize, 0usize);
                            // Which fidelity tier answered, per cell. On a fine
                            // view over a coarse build this is the whole story:
                            // #form-fidelity-ladder means a coarse region still
                            // ANSWERS a fine cell (bilinear carve plus the fine
                            // prior's detail re-added), so the picture is full of
                            // fine relief that no fluvial kernel ever computed.
                            // Nothing looks wrong. Counting the tiers is the only
                            // thing that says so.
                            let mut tiers: std::collections::BTreeMap<u8, usize> = Default::default();
                            for j in 0..nx as u32 {
                                for i in 0..nx as u32 {
                                    let h = tile[j as usize * nx + i as usize];
                                    if h > sea_m {
                                        l += 1;
                                    }
                                    if !baseline.is_empty() {
                                        let d = change_at(i, j);
                                        cs += d as f64;
                                        cmin = cmin.min(d);
                                        cmax = cmax.max(d);
                                        if d > 0.5 {
                                            ri += 1;
                                        } else if d < -0.5 {
                                            fa += 1;
                                        }
                                    }
                                    if !is_stage {
                                        if pure_prior || face_regions.is_empty() {
                                            fb += 1;
                                        } else {
                                            let (gi, gj) = g(i, j);
                                            let cid = CellId::from_face_ij(face, gi, gj, level);
                                            match vivarium_world::erosion::tier_at(cid, &face_regions)
                                            {
                                                Some(t) => *tiers.entry(t).or_default() += 1,
                                                None => fb += 1,
                                            }
                                        }
                                        let d = water_at(i, j);
                                        if d > WET_M {
                                            wc += 1;
                                            if !ocean_at(i, j) {
                                                iw += 1;
                                            }
                                        }
                                    }
                                }
                            }

                            let (fm, fseam) = mesh::build_face(&FaceInput {
                                face,
                                level,
                                oi,
                                oj,
                                nx,
                                tile: &tile,
                                exag: req.exag,
                                sea_m,
                                mode: req.paint,
                                ghost,
                                state: &state,
                                water: &water_at,
                                water_max_m: water.max_depth_m,
                                depression: &depression_at,
                                depression_max_m,
                                change: &change_at,
                                change_scale_m: req.change_scale_m,
                                is_ocean: &ocean_at,
                            });
                            (
                                fm,
                                tile,
                                fseam,
                                l,
                                fb,
                                iw,
                                wc,
                                cs,
                                cmin,
                                cmax,
                                ri,
                                fa,
                                tiers,
                                (dep_cells, dep_capacity_m3, dep_deepest_m),
                            )
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|h| {
                        let (fm, tile, fseam, l, fb, iw, wc, cs, cmin, cmax, ri, fa, tr, dep) =
                            h.join().expect("face build panicked");
                        dep_cells_total += dep.0;
                        dep_capacity_total += dep.1;
                        dep_deepest_total = dep_deepest_total.max(dep.2);
                        for (t, n) in tr {
                            *tier_cells.entry(t).or_default() += n;
                        }
                        land += l;
                        prior_fallback += fb;
                        inland_water += iw;
                        water_cells += wc;
                        chg_sum += cs;
                        chg_min = chg_min.min(cmin);
                        chg_max = chg_max.max(cmax);
                        rising += ri;
                        falling += fa;
                        total += tile.len();
                        tiles.push(tile);
                        seam.merge(&fseam);
                        fm
                    })
                    .collect()
            });

            let facts = FrameFacts {
                // When the level filter empties `regions`, still report store-fresh
                // carve count so the HUD can say "tiles exist but none apply."
                eroded_tiles: if regions.is_empty() {
                    census.fresh
                } else {
                    regions.len()
                },
                stale_tiles: census.stale,
                prior_fallback_frac: prior_fallback as f32 / total.max(1) as f32,
                land_frac: land as f32 / total.max(1) as f32,
                inland_water_cells: inland_water,
                water_cells,
                water_requested: water.requested,
                water_loaded: water.loaded,
                sea_m,
                sea_provenance,
                craton_growth: lithosphere::craton_growth(tp),
                craton_sites: lithosphere::craton_sites(seed).len(),
                tp_c: tp,
                refused_writes: store.refused_writes(),
                pull_s: t0.elapsed().as_secs_f32(),
                change_mean: (chg_sum / total.max(1) as f64) as f32,
                change_min: chg_min,
                change_max: chg_max,
                frac_rising: rising as f32 / total.max(1) as f32,
                frac_falling: falling as f32 / total.max(1) as f32,
                stage_epoch: match req.lens {
                    Lens::Erosion(i) => chain.epoch(i),
                    _ => None,
                },
                stage_tiles: regions.len(),
                tier_cells,
                cells: total,
                depression_cells: dep_cells_total,
                depression_capacity_m3: dep_capacity_total,
                depression_deepest_m: dep_deepest_total,
            };

            let frame = Frame {
                req,
                faces,
                tiles,
                seam,
                facts,
                roots: frame_roots,
                ladder_built: ladder.built.clone(),
                chain: chain.clone(),
            };
            if tx.send(Msg::Frame(Box::new(frame))).is_err() {
                return; // window closed
            }
        }
    });
}
