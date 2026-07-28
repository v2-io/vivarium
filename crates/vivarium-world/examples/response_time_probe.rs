//! The two numbers that govern whether an epoch count and an epoch size mean
//! anything, measured on live terrain: the per-epoch **Courant number** of the
//! erosional kinematic wave, and the **analytical response time in epochs**
//! (discrete `T_A`, Gasparini et al. 2024 / Whipple & Tucker 1999 at n=1).
//!
//! Why this probe exists ( `#obs-erosion-residual-is-driver-bound` ): erosion
//! has no runtime convergence gate — sustained uplift pins the residual at the
//! driver's rate — so the honest epoch count must come from *a-priori* physics,
//! and at n=1 that number is computable from the network before running. This
//! probe is the first measurement of it in this tree. It also measures the
//! Courant number, which bounds how much the *transient* (exactly what the
//! stage chain materializes) can be trusted at our epoch size (Braun & Willett
//! 2013 §7.2 exclude the transient from their accuracy claim).
//!
//! Same seeding as `erosion_settle_probe` (the builder's own kernel setup:
//! pulled initial topography, uplift, precip weights) so the numbers describe
//! the kernel actually run, not a bare stand-in. Store is a temp dir; the
//! probe writes nothing into any real world.
//!
//! Run: `cargo run --release --example response_time_probe`

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::planet::Planet;
use vivarium_world::query::World;
use vivarium_world::sample::cell_size_m;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;

const SEED: u64 = 0xF1D2_42B2_1D8D_89EA; // the settle probe's seed — comparable rows
const NX: usize = 64;
/// Channelization threshold for the response figures, in median cell areas —
/// an instrument choice (headwaters are creep-governed, not wave-governed).
const CHANNEL_MIN_CELLS: f32 = 10.0;

fn census_at(world: &World, face: Face, level: u8, oi: u32, oj: u32) {
    census_at_seed(world, SEED, face, level, oi, oj)
}

/// Census the INITIAL surface (builder's own seeding) at `seed` — computes
/// through `world`'s store, so pass a temp-store world.
fn census_at_seed(world: &World, seed: u64, face: Face, level: u8, oi: u32, oj: u32) {
    let (initial_topo, _) = world.initial_topography(face, level, oi, oj, NX);
    let (uplift, _) = world.uplift_tile(face, level, oi, oj, NX);
    let (precip, _) = world.climate_tile(face, level, oi, oj, NX);
    let mean = precip.iter().sum::<f32>() / precip.len().max(1) as f32;
    let pw: Vec<f32> =
        if mean > 0.0 { precip.iter().map(|p| p / mean).collect() } else { vec![1.0; precip.len()] };
    let surf = |cell: vivarium_world::sphere::CellId| -> f64 {
        let (cf, ci, cj, _) = cell.to_face_ij();
        if cf.index() == face.index() && ci >= oi && cj >= oj {
            let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
            if di < NX && dj < NX {
                return initial_topo[dj * NX + di] as f64;
            }
        }
        vivarium_world::gen::initial_topography_m(seed, cell, level)
    };
    let mut f = Fluvial::from_surface(seed, face, level, oi, oj, NX, surf);
    f.set_uplift_rate(uplift);
    f.set_precip_weight(pw);
    let p = FluvialParams::default();
    let c = f.response_census(&p, CHANNEL_MIN_CELLS);
    let cell_km = cell_size_m(level, Planet::EARTH.radius_m) / 1000.0;
    println!(
        "L{level:<2} f{} ({oi:>4},{oj:>4}) ~{cell_km:>6.2} km/cell  subaerial {:>4}/{}  channel {:>4}  maxcatch {:>7.0}  \
         Courant p50 {:>9.3e} max {:>9.3e}   response epochs p50 {:>8.1} p90 {:>8.1} max {:>8.1}",
        face.index(),
        c.subaerial,
        NX * NX,
        c.channel_cells,
        c.max_catchment_cells,
        c.courant_p50,
        c.courant_max,
        c.response_epochs_p50,
        c.response_epochs_p90,
        c.response_epochs_max,
    );
}

fn main() {
    // Part 1 — the LIVE world's terrain, read-only. The subaerial network the
    // kernel actually carved is the one whose response time means anything:
    // the initial surface at these seeds is almost entirely submarine (the
    // settle probe's 3911-subaerial footprint is land that UPLIFT raised
    // during its 400-epoch run, not land that was there at epoch zero — a
    // fact this probe's first draft exposed by measuring 165).
    let world_dir = std::env::var("VIVARIUM_WORLD").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        dirs_fallback().join(".cache/vivarium/globe-world")
    });
    match Store::open_read_only(&world_dir) {
        Ok(store) => match vivarium_world::spec::WorldSpec::load(&world_dir) {
            Ok(Some(spec)) => {
                let world = World::new(&store, spec.seed);
                println!(
                    "== live world \"{}\" (seed {}) — kinematic-wave census of the eroded bed ==",
                    spec.name, spec.seed
                );
                println!(
                    "(k_dt {}, m {}, channel ≥ {CHANNEL_MIN_CELLS} median cell areas; read-only, nothing written)",
                    FluvialParams::default().k_dt,
                    FluvialParams::default().m
                );
                println!("Courant ≫ 1 ⇒ the transient outruns the scheme's accuracy claim (endpoint unaffected).");
                println!("response epochs = a-priori count for the wave to cross the channel network.\n");
                // COHORT-AWARE, never merged: a store can hold beds carved
                // under several source trees (the verify loop stales the store
                // on every src edit), and mixing them would census a terrain
                // nobody built (the same fault class the explorer's cohort
                // discipline forbids on the time axis). Each src cohort is
                // reported under its own label; the current binary's cohort is
                // marked. A stale cohort is honest data for network SHAPE.
                let roots = store.roots().unwrap_or_default();
                let mut srcs: Vec<String> = Vec::new();
                for r in &roots {
                    if r.key.starts_with("erosion-tile@")
                        && vivarium_world::watch::key_field(&r.key, "aspect").is_none()
                    {
                        if let Some(s) = vivarium_world::watch::key_field(&r.key, "src") {
                            if !srcs.iter().any(|x| x == s) {
                                srcs.push(s.to_string());
                            }
                        }
                    }
                }
                let cur = vivarium_world::nomotheke::SRC_HASH;
                srcs.sort_by_key(|s| s != cur); // current cohort first, if present
                if let Ok(pick) = std::env::var("COHORT") {
                    srcs.retain(|s| s.starts_with(&pick));
                }
                let sea = vivarium_world::sea_level::derived_sea_level_m(spec.seed) as f32;
                for src in srcs.iter().take(3) {
                    let tag = if src == cur { "CURRENT" } else { "stale-src" };
                    let mut regions = world
                        .load_eroded_regions_where(|k| vivarium_world::watch::key_field(k, "src") == Some(src));
                    // Landiest tiles first — the network census is about land.
                    regions.sort_by_key(|r| std::cmp::Reverse(r.h.iter().filter(|&&h| h > sea).count()));
                    println!("cohort src={src} ({tag}):");
                    for r in regions.iter().take(4) {
                        let mut f = Fluvial::from_region(r);
                        let c = f.response_census(&FluvialParams::default(), CHANNEL_MIN_CELLS);
                        let cell_km = cell_size_m(r.level, Planet::EARTH.radius_m) / 1000.0;
                        println!(
                            "  L{:<2} f{} ({:>4},{:>4}) ~{cell_km:>6.2} km/cell  subaerial {:>4}/{}  channel {:>4}  maxcatch {:>7.0}  \
                             Courant p50 {:>9.3e} max {:>9.3e}   response epochs p50 {:>8.1} p90 {:>8.1} max {:>8.1}",
                            r.level,
                            r.face.index(),
                            r.oi,
                            r.oj,
                            c.subaerial,
                            r.nx * r.nx,
                            c.channel_cells,
                            c.max_catchment_cells,
                            c.courant_p50,
                            c.courant_max,
                            c.response_epochs_p50,
                            c.response_epochs_p90,
                            c.response_epochs_max,
                        );
                    }
                }
            }
            _ => println!("(live world at {} has no manifest — skipping live census)", world_dir.display()),
        },
        Err(e) => println!("(no live world at {} — {e}; skipping live census)", world_dir.display()),
    }

    // Part 1b — candidate FINE patches at the live seed: the L13 image of the
    // landiest L9 tile spans 16×16 L13 tiles; census a spread of them on the
    // INITIAL surface (temp store — computing a fresh tile writes, and the live
    // handle rightly refuses that). This is the number that picks the fine
    // patch's epoch count before anything is built.
    if let Ok(Some(spec)) = Store::open_read_only(&world_dir)
        .and_then(|s| Ok(vivarium_world::spec::WorldSpec::load(&world_dir)?.map(|sp| (s, sp))))
    {
        let (store, spec) = spec;
        let world = World::new(&store, spec.seed);
        // Current cohort if present, else the first cohort found — never merged.
        let mut regions = world.load_current_eroded_regions();
        if regions.is_empty() {
            let roots = store.roots().unwrap_or_default();
            if let Some(src) = roots.iter().find_map(|r| {
                (r.key.starts_with("erosion-tile@"))
                    .then(|| vivarium_world::watch::key_field(&r.key, "src"))
                    .flatten()
            }) {
                regions = world.load_eroded_regions_cohort(src);
            }
        }
        let sea = vivarium_world::sea_level::derived_sea_level_m(spec.seed) as f32;
        regions.sort_by_key(|r| std::cmp::Reverse(r.h.iter().filter(|&&h| h > sea).count()));
        if let Some(r) = regions.first() {
            println!(
                "\n== candidate L13 patches (live seed, initial surface) inside landiest L9 tile f{} ({},{}) ==",
                r.face.index(),
                r.oi,
                r.oj
            );
            let tmp = std::env::temp_dir().join(format!("vivarium-l13-cand-{}", std::process::id()));
            let ts = Store::open(&tmp).unwrap();
            let tw = World::new(&ts, spec.seed);
            let (base_i, base_j) = (r.oi * 16, r.oj * 16); // L9 → L13 image
            for (di, dj) in [(256u32, 256u32), (256, 640), (640, 256), (640, 640), (448, 448)] {
                census_at_seed(&tw, spec.seed, r.face, 13, base_i + di, base_j + dj);
            }
            let _ = std::fs::remove_dir_all(&tmp);
        }
    }

    // Part 2 — the builder's own seeding (initial surface + uplift + precip),
    // reproducible from a bare seed in a temp store: the settle probe's f2
    // footprint at L9 and its L13 interior, for the fine-patch scale question.
    println!("\n== initial-surface census (settle-probe seeding, seed {SEED:#x}) — mostly-submarine start ==");
    let dir = std::env::temp_dir().join(format!("vivarium-response-probe-{}", std::process::id()));
    let store = Store::open(&dir).unwrap();
    let world = World::new(&store, SEED);
    census_at(&world, Face::from_index(2), 9, 64, 64);
    for (oi, oj) in [(1536u32, 1536u32), (1280, 1792)] {
        census_at(&world, Face::from_index(2), 13, oi, oj);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

fn dirs_fallback() -> std::path::PathBuf {
    std::env::var("HOME").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from("."))
}
