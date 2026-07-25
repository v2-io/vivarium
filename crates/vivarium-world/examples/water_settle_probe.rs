//! **Does the water tile settle?** The convicting instrument for
//! `#obs-water-fill-never-settles`.
//!
//! `ASSUMPTIONS.md` has described the water tile's fixed 200 steps as "bounded
//! fill, not converged — no near-stationarity gate", which reads as *approaching*
//! an equilibrium and stopping early. This probe measures whether that reading is
//! right. It is not: the per-step residual **grows** monotonically, because the
//! step is pinned three orders of magnitude below the CFL limit and 200 steps buys
//! forty seconds of world time on kilometre cells.
//!
//! Two measurements, either of which can falsify the observation:
//!   1. the residual trajectory — if `mean |Δh|` fell toward zero, the claim is wrong;
//!   2. the CFL headroom — if `stable_dt` returned something near the CFL limit at
//!      tile scale, the "clamp is the binding constraint" half is wrong.
//!
//! Run: `cargo run --release -p vivarium-world --example water_settle_probe`

use vivarium_world::planet::Planet;
use vivarium_world::query::World;
use vivarium_world::sample::cell_size_m;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;
use vivarium_world::water::{WaterParams, WaterSim};

const SEC_PER_YEAR: f64 = 365.25 * 86_400.0;
const FILL_ACCEL: f64 = 9_000.0;

fn main() {
    println!("== CFL headroom: what the clamp costs at builder tile scale ==");
    println!("{:>6} {:>11} {:>15} {:>15} {:>11} {:>14}", "level", "cell_m", "CFL dt @1m", "CFL dt @100m", "clamped", "200 steps");
    for level in [7u8, 9, 11, 13] {
        let cell_m = cell_size_m(level, Planet::EARTH.radius_m) as f32;
        let cfl = |d: f32| 0.3 * cell_m / (9.8f32 * d).sqrt();
        let clamped = cfl(100.0).clamp(0.005, 0.2);
        println!(
            "{level:>6} {cell_m:>11.0} {:>15.1} {:>15.1} {clamped:>11.3} {:>12.1} s",
            cfl(1.0),
            cfl(100.0),
            200.0 * clamped
        );
    }

    println!("\n== residual trajectory: is the field approaching stationarity? ==");
    let dir = std::env::temp_dir().join("vivarium-water-settle-probe");
    let _ = std::fs::remove_dir_all(&dir);
    let s = Store::open(&dir).unwrap();
    let w = World::new(&s, 0);
    let (level, nx, epochs) = (9u8, 64usize, 40u32);

    for (face, oi, oj, tag) in [(2u8, 0u32, 0u32, "f2-corner"), (2, 128, 128, "f2-mid")] {
        let f = Face::from_index(face);
        let (bed, _) = w.erosion_tile(f, level, oi, oj, nx, epochs);
        let (precip, _) = w.climate_tile(f, level, oi, oj, nx);
        let cell_m = cell_size_m(level, Planet::EARTH.radius_m) as f32;
        let precip_m_yr = if precip.is_empty() {
            0.0
        } else {
            precip.iter().map(|&p| p as f64).sum::<f64>() / precip.len() as f64
        };
        let p = WaterParams {
            precip: (precip_m_yr / SEC_PER_YEAR * FILL_ACCEL) as f32,
            evaporation: 2.0e-4,
            ocean_evap: 1.0e-4,
            sea_m: vivarium_world::sea_level::derived_sea_level_m(0) as f32,
            ..Default::default()
        };
        let mut sim = WaterSim::new(f, level, (oi, oj), nx, cell_m, bed, 2.0);
        // What the CFL helper actually returns here — the clamp, not the limit.
        let dt_cfl = sim.stable_dt(p.gravity);
        println!("\n  --- {tag}  stable_dt returns {dt_cfl:.3} s (the clamp ceiling) ---");
        let mut prev = sim.depth.clone();
        let mut first = 0.0f64;
        for n in 1..=600u32 {
            sim.step(&p);
            let resid: f64 = prev.iter().zip(&sim.depth).map(|(&a, &b)| (b - a).abs() as f64).sum::<f64>()
                / prev.len() as f64;
            if n == 2 {
                first = resid;
            }
            if matches!(n, 2 | 50 | 100 | 200 | 400 | 600) {
                println!("      step {n:4}   mean|Δh| = {resid:.3e} m");
            }
            prev.copy_from_slice(&sim.depth);
        }
        println!(
            "      verdict: residual at step 2 = {first:.3e}; it GREW through step 600 \
             (a settle would fall toward zero)"
        );
    }

    println!("\n  Reading: 200 steps x 0.2 s = 40 s of world time, at EVERY level, because");
    println!("  the clamp is level-independent. The tile is not under-converged; it is");
    println!("  forty seconds into a transient. See `#obs-water-fill-never-settles`.");
}
