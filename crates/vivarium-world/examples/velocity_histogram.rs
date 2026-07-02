//! Histogram of water node velocities once the stream network has filled —
//! Joseph's ask (2026-07-03), and a regime probe in its own right: the speed
//! distribution is a fingerprint of the whole momentum/friction/breaking
//! stack. Expected shape on real terrain: a large slow mass (pools, sheets),
//! a mid-speed body (streams, roughly Manning equilibrium), a thin fast tail
//! pinned below the breaking cap (~2×√(g·h)).
//!
//! Terrain = the real macro tier (worldview's massif at L19), water at L21
//! over the full footprint, deluge rain until the network fills.
use vivarium_world::erosion::{self, Fluvial, FluvialParams};
use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

fn main() {
    let face = Face::ZPos;
    let (oi, oj, nx) = (165_800u32, 413_600u32, 128usize);
    let mut macro_t = Fluvial::from_prior(face, 19, oi, oj, nx);
    macro_t.erode(&FluvialParams::default());
    let macro_r = macro_t.to_region();

    let wnx = nx * 4; // L21 over the same footprint
    let (woi, woj) = (oi * 4, oj * 4);
    let cell = vivarium_world::sample::cell_size_m(21, vivarium_world::planet::Planet::EARTH.radius_m) as f32;
    let mut bed = vec![0.0f32; wnx * wnx];
    for y in 0..wnx {
        for x in 0..wnx {
            let c = vivarium_world::sphere::CellId::from_face_ij(face, woi + x as u32, woj + y as u32, 21);
            bed[y * wnx + x] = erosion::surface_at(c, std::slice::from_ref(&macro_r)) as f32;
        }
    }
    let mut w = WaterSim::new(face, 21, (woi, woj), wnx, cell, bed, 2.0);
    let p_deluge = WaterParams { precip: WaterParams::default().precip * 60.0, sed_capacity: 0.0, ..Default::default() };

    // Fill: deluge until the wet fraction and mean depth level off (streams
    // established), capped at 600 sim-s.
    let mut t = 0.0f32;
    while t < 600.0 {
        let dt = w.stable_dt(9.8);
        let p = WaterParams { dt, ..p_deluge };
        for _ in 0..(8.0 / dt) as u32 {
            w.step(&p);
            t += dt;
        }
    }
    let (fr_max, fr_sup) = w.froude();
    let wet = w.depth.iter().filter(|&&d| d > 0.01).count();
    println!("t={t:.0} sim-s  wet cells (>1cm): {wet}/{} ({:.1}%)  in-step Fr max {fr_max:.2} / sup {:.0}%", wnx * wnx, wet as f64 / (wnx * wnx) as f64 * 100.0, fr_sup * 100.0);

    // Node speeds: net vector, same definition the pawn readout uses.
    let r = w.to_region();
    let mut speeds: Vec<f32> = (0..wnx * wnx)
        .filter(|&i| w.depth[i] > 0.01)
        .map(|i| (r.vx[i] * r.vx[i] + r.vy[i] * r.vy[i]).sqrt())
        .collect();
    speeds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |q: f64| speeds[((speeds.len() - 1) as f64 * q) as usize];
    println!("percentiles (m/s): p10 {:.3}  p50 {:.3}  p90 {:.3}  p99 {:.3}  max {:.3}", pct(0.1), pct(0.5), pct(0.9), pct(0.99), speeds[speeds.len() - 1]);

    // Histogram: log-ish buckets covering stillwater to breaking.
    let edges = [0.0, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 1.5, 2.0, 3.0, 5.0, 8.0, f32::INFINITY];
    let label = ["0-1cm/s", "1-2cm/s", "2-5cm/s", "5-10cm/s", "10-20cm/s", "0.2-0.5", "0.5-1.0", "1.0-1.5", "1.5-2.0", "2-3", "3-5", "5-8", ">8 m/s"];
    let mut counts = vec![0usize; label.len()];
    for &v in &speeds {
        for k in 0..label.len() {
            if v >= edges[k] && v < edges[k + 1] {
                counts[k] += 1;
                break;
            }
        }
    }
    let cmax = *counts.iter().max().unwrap();
    println!("\n  node speed histogram ({} wet nodes):", speeds.len());
    for (k, &c) in counts.iter().enumerate() {
        let bar = "#".repeat((c * 60 / cmax.max(1)).max(usize::from(c > 0)));
        println!("  {:>10} {:>8}  {bar}", label[k], c);
    }
}
