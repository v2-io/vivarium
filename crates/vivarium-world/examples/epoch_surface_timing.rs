//! **What does one deep-time playback frame actually cost?**
//!
//! The globe's playback advances only when the next epoch's surface rebuild
//! lands, so the frame rate *is* this cost. Every stage is already materialized
//! in the store (`warmed 81/81`), which makes "still slow" a claim about the
//! per-frame surface evaluation rather than about generation — and that is worth
//! separating by measurement rather than assuming, because they have completely
//! different fixes.
//!
//! Measures, per playback level: the cost of one whole-globe epoch surface (6
//! faces of `nx²` cells through `sea_level::tectonic_surface_at_tp`), warm.
//!
//! Run: `cargo run --release -p vivarium-world --example epoch_surface_timing`

use vivarium_world::mantle_thermal::{cooling_stages_refined, potential_temp_c, refinements_for};
use vivarium_world::planet::Planet;
use vivarium_world::query::World;
use vivarium_world::sample::cell_size_m;
use vivarium_world::sea_level;
use vivarium_world::sphere::{CellId, Face};
use vivarium_world::spec::WorldSpec;
use vivarium_world::store::Store;

fn main() {
    let dir = std::env::var("VIVARIUM_WORLD").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        std::path::PathBuf::from(home).join(".cache/vivarium/globe-world")
    });
    let seed = WorldSpec::load(&dir).ok().flatten().map(|s| s.seed).unwrap_or(0);
    let store = Store::open(&dir).expect("store");
    let world = World::new(&store, seed);
    let frames = WorldSpec::load(&dir).ok().flatten().map(|s| s.demand.frames).unwrap_or(6);
    let stages = cooling_stages_refined(refinements_for(frames as usize));
    let tp = potential_temp_c(stages[stages.len() / 2]);

    // Warm the reduction so we time the SURFACE evaluation, not the pour — the
    // globe has already done this by the time playback starts (`warmed 81/81`).
    let t0 = std::time::Instant::now();
    let (_r, src) = world.epoch_reduction(tp);
    println!("epoch reduction for T_p {tp:.0} C: {src:?} in {:.1?}\n", t0.elapsed());

    println!("{:>6} {:>10} {:>12} {:>14} {:>16}", "level", "cells", "cell km", "one frame", "81-frame lap");
    for level in [6u8, 7, 8] {
        let nx = 1usize << level;
        let t0 = std::time::Instant::now();
        let mut sink = 0.0f64;
        for f in 0..6 {
            let face = Face::from_index(f);
            for j in 0..nx as u32 {
                for i in 0..nx as u32 {
                    let cell = CellId::from_face_ij(face, i, j, level);
                    sink += sea_level::tectonic_surface_at_tp(seed, cell, level, tp);
                }
            }
        }
        let el = t0.elapsed();
        let cells = 6 * nx * nx;
        println!(
            "{level:>6} {cells:>10} {:>12.0} {:>14.1?} {:>16.1?}",
            cell_size_m(level, Planet::EARTH.radius_m) / 1000.0,
            el,
            el * stages.len() as u32
        );
        std::hint::black_box(sink);
    }
    println!(
        "\nA lap is what playback costs EVERY time round, because nothing caches the built\n\
         surfaces — the globe loops, so the second lap pays the same as the first. Whether the\n\
         fix is a cache, a coarser playback level, or both is a judgement about what the frame\n\
         is for: at whole-planet zoom the finest level is far below what the screen resolves."
    );
}
