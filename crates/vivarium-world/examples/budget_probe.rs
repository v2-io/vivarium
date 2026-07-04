//! budget_probe — hunt ORIENTATION.md open investigation #2: the water-budget
//! gauge drifting linearly (−0.37 m³·cells/sim-s) in the living phase.
//!
//! Hypothesis under test: at steady state the depth field is STATIONARY, so
//! per-cell f32 rounding on field↔reservoir exchanges (rain debits the f64
//! atmosphere by the INTENDED amount; the f32 depth absorbs a ROUNDED amount)
//! is frozen — the identical signed error repeats every step. Sum over cells =
//! a constant per step = a perfectly linear drift. Deep ocean cells are the
//! worst: a ~1e-6 m transfer is below the ulp of a 100–400 m f32 depth, so the
//! intended and realized amounts diverge wholesale, not marginally.
//!
//! The probe builds a coastal domain (deep ocean shelf → beach → inland bowl),
//! settles it under deluge, then runs living-style episodic storms and reports
//! drift/sim-s with each exchange stage toggled independently.

use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

fn coastal(nx: usize, ocean_depth: f32) -> WaterSim {
    let sea = vivarium_world::gen::SEA_LEVEL_M as f32;
    let mut bed = vec![0.0f32; nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            // Left half: flat deep ocean floor. Right half: rises through a
            // beach to inland hills with a shallow bowl (perched lake).
            let t = x as f32 / nx as f32;
            bed[y * nx + x] = if t < 0.5 {
                sea - ocean_depth
            } else {
                let up = (t - 0.5) * 2.0;
                let bowl = ((y as f32 / nx as f32 - 0.5).powi(2) * -80.0).max(-8.0);
                sea - 2.0 + up * 60.0 + bowl
            };
        }
    }
    WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, 4.8, bed, 1.0)
}

/// Run `sim_s` seconds of living-style stepping; return (drift_delta, rate/s).
fn run(w: &mut WaterSim, p: &WaterParams, sim_s: f32) -> (f64, f64) {
    let d0 = w.budget_drift();
    let mut t = 0.0f32;
    while t < sim_s {
        let dt = w.stable_dt(9.8);
        let wp = WaterParams { dt, ..*p };
        w.step(&wp);
        t += dt;
    }
    let d1 = w.budget_drift();
    (d1 - d0, (d1 - d0) / sim_s as f64)
}

fn main() {
    let nx = 256;
    for ocean_depth in [15.0f32, 120.0, 400.0] {
        println!("=== ocean depth {ocean_depth} m, {nx}x{nx} ===");
        let mut w = coastal(nx, ocean_depth);
        // Settle: deluge to (rough) steady state, like the filling phase.
        let deluge = WaterParams {
            precip: WaterParams::default().precip * 60.0,
            ..Default::default()
        };
        run(&mut w, &deluge, 300.0);
        w.rebaseline_budget();

        // Living-phase storm rain (10x), all stages on — the baseline reading.
        let storm = WaterParams {
            precip: WaterParams::default().precip * 10.0,
            ..Default::default()
        };
        let (_, all) = run(&mut w, &storm, 60.0);
        // Linearity check: a second stretch should repeat the rate.
        let (_, all2) = run(&mut w, &storm, 60.0);

        // Stage isolation, each from the same settled state (clone).
        let stages: [(&str, WaterParams); 4] = [
            ("rain only", WaterParams { evaporation: 0.0, ocean_evap: 0.0, infiltration: 0.0, sed_capacity: 0.0, ..storm }),
            ("evap only", WaterParams { precip: 0.0, ocean_evap: 0.0, infiltration: 0.0, sed_capacity: 0.0, ..storm }),
            ("gw only  ", WaterParams { precip: 0.0, evaporation: 0.0, ocean_evap: 0.0, sed_capacity: 0.0, ..storm }),
            ("pipes/sed", WaterParams { precip: 0.0, evaporation: 0.0, ocean_evap: 0.0, infiltration: 0.0, ..storm }),
        ];
        println!("  all stages : {all:+.4} m·cells/sim-s (repeat {all2:+.4})");
        for (name, p) in stages {
            let mut wc = coastal(nx, ocean_depth);
            run(&mut wc, &deluge, 300.0);
            wc.rebaseline_budget();
            let (_, r) = run(&mut wc, &p, 60.0);
            let (_, r2) = run(&mut wc, &p, 60.0);
            println!("  {name}  : {r:+.4} m·cells/sim-s (repeat {r2:+.4})");
        }
    }
}
