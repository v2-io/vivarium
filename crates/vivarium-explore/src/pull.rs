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
use std::sync::Mutex;

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

        // Census state, refreshed when the root count moves (a builder running in
        // another terminal). Landings are needed only for replay, and reading
        // them is a directory walk, so they are read lazily and re-read when the
        // census changes.
        let mut roots = store.roots().unwrap_or_default();
        let mut cov = Coverage::parse(&roots);
        let mut water = WaterField::load(&world, &cov);
        let mut landings: Vec<watch::Landing> = Vec::new();
        let mut chain = Chain::read(&roots, 0);
        crate::lens::read_residuals(&store, &roots, &mut chain);

        while let Ok(req) = rx.recv() {
            let t0 = std::time::Instant::now();

            // Re-read the census each frame: cheap next to a mesh build, and it
            // is what makes live-watching work at all — an explorer open beside a
            // running builder should show tiles arriving.
            let now = store.roots().unwrap_or_default();
            if now.len() != roots.len() {
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
                _ => (roots.clone(), Coverage::parse(&roots)),
            };

            let regions = match req.lens {
                Lens::Present => world.observe().load_current_eroded_regions(),
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
                    }),
                    None => Vec::new(),
                },
                Lens::Replay(n) => {
                    if landings.is_empty() {
                        landings = watch::landings(&dir).unwrap_or_default();
                    }
                    let keys: BTreeSet<String> = crate::lens::replay_key_set(&landings, n);
                    world.observe().load_eroded_regions_where(|k| keys.contains(k))
                }
            };
            let census = world.observe().eroded_region_census();

            // The ghost ring lies on neighbouring faces, so it must come from the
            // same law the in-face tile came from, or the seam instrument would
            // measure the gap between two different surfaces rather than the
            // world's own discontinuity.
            //
            // **It reads the loaded regions.** For a whole-face unit the ghost
            // ring falls on other faces, where a carved tile usually is not — so
            // this was written as the bare prior and the shortcut cost nothing
            // visible. For a *window* into a face the ghost ring is ordinary
            // in-face terrain, carved like everything around it, and the prior
            // there would put a one-cell moat of uncarved ground around every
            // patch: a manufactured discontinuity, at exactly the scale a fine
            // view exists to inspect. Reading the regions is the same law the
            // in-face tile came from, which is what this comment always asked for.
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
                } else {
                    vivarium_world::erosion::surface_at(seed, cell, regions_ref) as f32
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

            let faces: Vec<FaceMesh> = std::thread::scope(|s| {
                let (world, regions, cov, water, cache, ghost, units) =
                    (&world, &regions, &frame_cov, &water, &stage_cache, &ghost, &units);
                let handles: Vec<_> = units
                    .iter()
                    .map(|&unit| {
                        s.spawn(move || {
                            let (f, oi, oj, nx) = (unit.face, unit.oi, unit.oj, unit.nx);
                            let face = Face::from_index(f);
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
                                _ => world.observe().assemble_surface_tile(face, level, oi, oj, nx, regions).0,
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
                                            vivarium_world::erosion::surface_at(seed, cid, &[]) as f32
                                        );
                                    }
                                }
                                b
                            } else {
                                Vec::new()
                            };
                            let (mut dep_cells, mut dep_capacity_m3, mut dep_deepest_m) =
                                (0usize, 0.0f64, 0.0f32);
                            // The depression channel: what the DRAWN surface
                            // could hold. Run through `Fluvial::drainage_surface`
                            // — the same reader `base_level_probe`,
                            // `discharge_probe` and the unit tests use — so the
                            // picture and the numbers cannot drift apart. The
                            // reader saves and restores the heights it reads
                            // (`drainage_surface_restores_the_world_it_read`), so
                            // it cannot advance anything, and it opens no store.
                            //
                            // The contract is set to NoFluxWall EXPLICITLY rather
                            // than inferred. A window short of a whole face infers
                            // `BaseLevelSink`, which makes the window's own rim an
                            // outlet and drains every basin that reaches it — the
                            // reader would then report ~0 capacity and the paint
                            // would be black for a reason that is about the reader
                            // rather than the world ( #form-declared-boundary-contract ;
                            // the same understatement measured at
                            // #obs-tile-outlets-grade-away-the-basins FE(5), where a
                            // sink-contract reader read 19.67% against a wall
                            // reader's 63.5%). The wall has its own bias — it hands
                            // the window no outlet at all when there is no coast in
                            // it — and the HUD says which one is up.
                            let depression: Vec<f32> = if req.paint.needs_depression() {
                                let region = vivarium_world::erosion::ErodedRegion {
                                    face,
                                    level,
                                    oi,
                                    oj,
                                    nx,
                                    h: tile.clone(),
                                    seed,
                                };
                                let mut f = vivarium_world::erosion::Fluvial::from_region(&region);
                                f.set_edge_contract(
                                    vivarium_world::erosion::EdgeContract::NoFluxWall,
                                );
                                let ds = f.drainage_surface();
                                dep_cells = ds.stats.depression_cells;
                                dep_capacity_m3 = ds.stats.depression_volume_m3;
                                dep_deepest_m = ds.stats.deepest_depression_m;
                                ds.fill_depth
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
                                        let (gi, gj) = g(i, j);
                                        let cid = CellId::from_face_ij(face, gi, gj, level);
                                        match vivarium_world::erosion::tier_at(cid, regions) {
                                            Some(t) => *tiers.entry(t).or_default() += 1,
                                            None => fb += 1,
                                        }
                                        let d = water_at(i, j);
                                        if d > WET_M {
                                            wc += 1;
                                            if h > sea_m {
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
                eroded_tiles: regions.len(),
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
