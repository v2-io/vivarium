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

use crate::lens::{FrameFacts, Ladder, Lens, SeaProvenance};
use crate::mesh::{self, FaceInput, FaceMesh, SeamStats};
use crate::paint::Paint;
use crate::water::{WaterField, WET_M};

/// One build request. Equality is what suppresses redundant rebuilds, so every
/// field here is something a *different picture* depends on.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Request {
    pub level: u8,
    pub exag: f32,
    pub paint: Paint,
    pub lens: Lens,
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
                let _ = tx.send(Msg::Landings(roots.len()));
            }

            let level = req.level;
            let nx = 1usize << level;

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
                Lens::Present => world.load_current_eroded_regions(),
                Lens::Stage(_) => Vec::new(),
                Lens::Replay(n) => {
                    if landings.is_empty() {
                        landings = watch::landings(&dir).unwrap_or_default();
                    }
                    let keys: BTreeSet<String> = crate::lens::replay_key_set(&landings, n);
                    world.load_eroded_regions_where(|k| keys.contains(k))
                }
            };
            let census = world.eroded_region_census();

            // The ghost ring lies on neighbouring faces, so it must come from the
            // same law the in-face tile came from, or the seam instrument would
            // measure the gap between two different surfaces rather than the
            // world's own discontinuity.
            let is_stage = matches!(req.lens, Lens::Stage(_));
            let ghost = move |face: Face, ci: i64, cj: i64, level: u8| -> f32 {
                let n = 1usize << level;
                let cu = ((ci as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cv = ((cj as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let dir = CubeCoord { face, u: cu, v: cv }.to_unit();
                let cell = CubeCoord::from_unit(dir).cell(level);
                if is_stage {
                    sea_level::tectonic_surface_at_tp(seed, cell, level, tp) as f32
                } else {
                    vivarium_world::erosion::surface_at(seed, cell, &[]) as f32
                }
            };

            // --- build the six faces in parallel ------------------------------
            let mut tiles: Vec<Vec<f32>> = Vec::with_capacity(6);
            let mut seam = SeamStats::default();
            let (mut land, mut total, mut prior_fallback) = (0usize, 0usize, 0usize);
            let (mut inland_water, mut water_cells) = (0usize, 0usize);

            let faces: Vec<FaceMesh> = std::thread::scope(|s| {
                let (world, regions, cov, water, cache, ghost) =
                    (&world, &regions, &frame_cov, &water, &stage_cache, &ghost);
                let handles: Vec<_> = (0u8..6)
                    .map(|f| {
                        s.spawn(move || {
                            let face = Face::from_index(f);
                            let tile = match req.lens {
                                Lens::Stage(_) => cache.get_or_build(seed, face, level, nx, tp),
                                _ => world.assemble_surface_tile(face, level, 0, 0, nx, regions).0,
                            };

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
                                let (bi, bj) = (to_build_level(ci), to_build_level(cj));
                                (cov.state_at_cell(f, bi, bj), cov.flags_at_cell(f, bi, bj))
                            };
                            let water_at = |ci: u32, cj: u32| -> f32 {
                                if matches!(req.lens, Lens::Stage(_)) {
                                    return 0.0; // water tiles exist only for the present
                                }
                                water.depth_at(f, ci, cj, level)
                            };

                            // Per-face tallies — the countable half of the frame.
                            let (mut l, mut fb, mut iw, mut wc) = (0usize, 0usize, 0usize, 0usize);
                            for j in 0..nx as u32 {
                                for i in 0..nx as u32 {
                                    let h = tile[j as usize * nx + i as usize];
                                    if h > sea_m {
                                        l += 1;
                                    }
                                    if !is_stage {
                                        let cid = CellId::from_face_ij(face, i, j, level);
                                        if vivarium_world::erosion::tier_at(cid, regions).is_none() {
                                            fb += 1;
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
                                tile: &tile,
                                exag: req.exag,
                                sea_m,
                                mode: req.paint,
                                ghost,
                                state: &state,
                                water: &water_at,
                                water_max_m: water.max_depth_m,
                            });
                            (fm, tile, fseam, l, fb, iw, wc)
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|h| {
                        let (fm, tile, fseam, l, fb, iw, wc) = h.join().expect("face build panicked");
                        land += l;
                        prior_fallback += fb;
                        inland_water += iw;
                        water_cells += wc;
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
            };

            let frame = Frame {
                req,
                faces,
                tiles,
                seam,
                facts,
                roots: frame_roots,
                ladder_built: ladder.built.clone(),
            };
            if tx.send(Msg::Frame(Box::new(frame))).is_err() {
                return; // window closed
            }
        }
    });
}
