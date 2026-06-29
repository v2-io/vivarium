//! A dependency-free look at the **water** model (`hydro`) — does rain falling on
//! the eroded terrain actually flow, pool, and stand *flat* like water?
//!
//! Run: `cargo run -p vivarium-core --example water_preview [seed] [steps]`
//!
//! Prints three views, each of which checks a different claim:
//!   1. **plan view** — terrain as a brightness ramp with water overlaid (`░▒▓█`
//!      by depth). You should see water threading the valleys, not sheeting the
//!      slopes.
//!   2. **side cross-section** through the middle row — bed (`▓`) with water
//!      (`≈`) stacked on top. *This is the one that proves it is water:* a pool's
//!      top must be a flat horizontal line, not a surface draped on the bed.
//!   3. **volume trace** — total water vs. step, so you can see it reach a
//!      quasi-steady state (and that it is conserved, not exploding).
//!
//! Reads only the public `geo` + `hydro` APIs.

use vivarium_core::geo::{ErosionParams, Heightfield};
use vivarium_core::hydro::{WaterParams, WaterSim};

fn main() {
    let mut args = std::env::args().skip(1);
    let seed: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(0xC0FFEE);
    let steps: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(6000);

    // A matured landscape to rain on. Strong uplift so there is real relief for
    // water to find, and enough epochs for a dendritic network.
    let nx = 96;
    let ep = ErosionParams { nx, uplift: 0.6, k: 0.30, epochs: 60, ..Default::default() };
    let mut bed = Heightfield::simulate(&ep, seed).h;

    // The eroded field is depression-free (erosion fills pits), so it can show
    // rivers but never a lake. Stamp one real basin — a crater gouged into a
    // valley — so the cross-section can demonstrate flat standing water. (Lithology
    // will earn basins like this for real; this is the instrument, carving one by
    // hand to verify the water responds correctly.)
    let basin_y = nx / 2;
    let (bx, by, br, bdepth) = (nx as f32 * 0.42, basin_y as f32, 9.0_f32, 30.0_f32);
    for y in 0..nx {
        for x in 0..nx {
            let r = ((x as f32 - bx).powi(2) + (y as f32 - by).powi(2)).sqrt();
            if r < br {
                let bowl = (1.0 - (r / br).powi(2)) * bdepth; // smooth dish
                bed[y * nx + x] -= bowl;
            }
        }
    }

    let mut sim = WaterSim::new(nx, bed.clone());
    // Closed test world (no sea). High evaporation keeps the flat-area balance
    // (rain/evap) shallow so slopes shed water, while convergent flow in channels
    // and the basin still stands deep enough to read.
    let p = WaterParams { rain: 0.06, evaporation: 0.5, ..Default::default() };

    println!("vivarium water spike — seed {seed:#x}, {nx}×{nx}, {steps} steps\n");

    // Run in chunks so we can trace convergence to quasi-steady state.
    print!("volume trace (Σ depth): ");
    let chunk = (steps / 12).max(1);
    let mut done = 0;
    while done < steps {
        let n = chunk.min(steps - done);
        sim.run(&p, n);
        done += n;
        print!("{:.0} ", sim.total_water());
    }
    println!("\n");

    plan_view(&sim, &bed);
    println!();
    cross_section(&sim, basin_y);
}

/// Terrain ramp with water overlaid by depth. Water glyphs are visually heavier
/// than the terrain ramp so the network reads at a glance.
fn plan_view(sim: &WaterSim, bed: &[f32]) {
    const LAND: &[u8] = b" .:-=+*#%@";
    const WATER: [char; 4] = ['░', '▒', '▓', '█'];
    let (lo, hi) = bed.iter().fold((f32::MAX, f32::MIN), |(a, b), &v| (a.min(v), b.max(v)));
    let span = (hi - lo).max(1e-6);
    let nx = sim.nx;
    let step = (nx / 72).max(1);
    println!("plan  (land ' '→'@';  water depth ░<▒<▓<█)");
    for y in (0..nx).step_by(step) {
        let mut line = String::new();
        for x in (0..nx).step_by(step) {
            let i = y * nx + x;
            let d = sim.depth[i];
            if d > 0.25 {
                let t = (d / 4.0).clamp(0.0, 1.0);
                line.push(WATER[((t * 3.0).round() as usize).min(3)]);
            } else {
                let t = (bed[i] - lo) / span;
                let c = LAND[((t * (LAND.len() - 1) as f32).round() as usize).min(LAND.len() - 1)];
                line.push(c as char);
            }
        }
        println!("{line}");
    }
}

/// Side view of one row: height up the page, x across. Bed is solid `▓`; water
/// `≈` stacks from the bed to the water surface. A real lake reads as a FLAT
/// water top spanning a basin; a draped surface would step with the bed.
fn cross_section(sim: &WaterSim, y: usize) {
    let nx = sim.nx;
    let row = |x: usize| y * nx + x;
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for x in 0..nx {
        lo = lo.min(sim.bed[row(x)]);
        hi = hi.max(sim.surface(row(x)));
    }
    let rows = 24usize;
    let span = (hi - lo).max(1e-6);
    let step = (nx / 72).max(1);
    println!("cross-section at y={y}  (bed ▓, water ≈;  flat ≈-tops = real pools)");
    for r in 0..rows {
        // Height level for this print row, top of page = high.
        let level = hi - (r as f32 + 0.5) / rows as f32 * span;
        let mut line = String::new();
        for x in (0..nx).step_by(step) {
            let b = sim.bed[row(x)];
            let s = sim.surface(row(x));
            line.push(if level <= b {
                '▓'
            } else if level <= s {
                '≈'
            } else {
                ' '
            });
        }
        println!("{line}");
    }
}
