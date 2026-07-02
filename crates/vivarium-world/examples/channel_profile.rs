//! Longitudinal depth profile of a steep synthetic channel under deluge rain —
//! the instrument for Joseph's "multi-metre blobs winding down the channels".
//! A pulsing sim shows alternating deep/shallow cells along the thalweg; honest
//! flow shows a smooth profile. No renderer involved.
use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

fn main() {
    // Two regimes: steep+deluge is HONESTLY roll-wave unstable (Fr_eq ~2.7 —
    // surges there may be real); gentle+moderate is subcritical (Fr_eq ~0.65)
    // where nature guarantees smooth flow — lumps THERE are a scheme bug.
    run("STEEP 10% + 60x deluge (roll waves plausible)", 0.10, 60.0, 150.0);
    run("GENTLE 1% + 10x rain (must be smooth)", 0.01, 10.0, 400.0);
}

fn run(label: &str, grade: f32, rain: f32, t_end: f32) {
    println!("--- {label} ---");
    let nx = 256usize;
    let cell = 4.8f32;
    let mut bed = vec![0.0f32; nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            let across = (x as f32 - nx as f32 / 2.0) * cell;
            bed[y * nx + x] = 6000.0 - y as f32 * cell * grade + (across / 40.0).powi(2);
        }
    }
    let mut w = WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, cell, bed, 5.0);
    let p = WaterParams { precip: WaterParams::default().precip * rain, sed_capacity: 0.0, ..Default::default() };
    let mut t = 0.0f32;
    while t < t_end {
        w.step(&p);
        t += p.dt;
    }
    let (fr_max, fr_sup) = w.froude();
    println!("t={t:.0} sim-s  Fr max {fr_max:.2}  supercritical {:.1}%", fr_sup * 100.0);
    // Depth down the channel centreline (skip inflow edge), 4 cells per row.
    let cx = nx / 2;
    let profile: Vec<f32> = (8..nx - 8).map(|y| w.depth[y * nx + cx]).collect();
    let mean: f32 = profile.iter().sum::<f32>() / profile.len() as f32;
    // Pulse metric: relative std of depth + count of >30% swings between cells.
    let var: f32 = profile.iter().map(|d| (d - mean).powi(2)).sum::<f32>() / profile.len() as f32;
    let mut swings = 0;
    for k in 1..profile.len() {
        let a = profile[k - 1].max(1e-3);
        if ((profile[k] - profile[k - 1]).abs() / a) > 0.3 {
            swings += 1;
        }
    }
    println!("channel depth: mean {mean:.3} m  rel-std {:.1}%  cell-to-cell >30% swings: {swings}/{}", (var.sqrt() / mean.max(1e-6)) * 100.0, profile.len() - 1);
    let bars: String = profile.iter().step_by(4).map(|d| {
        let x = (d / (2.0 * mean + 1e-6) * 8.0) as usize;
        [' ', '.', ':', '-', '=', '+', '*', '#', '@'][x.min(8)]
    }).collect();
    println!("profile (downhill →): [{bars}]");
}
