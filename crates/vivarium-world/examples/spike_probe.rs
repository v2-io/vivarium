//! Spike-anomaly probe (Joseph watched single-cell spikes FORM AND SHARPEN under
//! many fine-scale epochs — also latent in old core). Runs the same fine field
//! under pass-isolation configs and prints a spikiness metric over time:
//! spike = a cell higher than ALL 8 neighbours by more than 2·repose·dist.
//!
//!   cargo run --release -p vivarium-world --example spike_probe

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::sphere::Face;

fn spikes(f: &Fluvial) -> (usize, f32) {
    let nx = f.nx;
    let (mut count, mut worst) = (0usize, 0.0f32);
    for y in 1..nx - 1 {
        for x in 1..nx - 1 {
            let h = f.h[y * nx + x];
            let mut min_drop = f32::INFINITY;
            for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let j = ((y as i32 + dy) as usize) * nx + (x as i32 + dx) as usize;
                min_drop = min_drop.min(h - f.h[j]);
            }
            let threshold = 2.0 * 0.8 * f.cell_m; // 2 × repose × dist
            if min_drop > threshold {
                count += 1;
                worst = worst.max(min_drop);
            }
        }
    }
    (count, worst)
}

/// Sawtooth teeth: cells above their 4-neighbour average by > repose·dist —
/// catches ridge-aligned tooth rows the strict local-max metric misses.
fn teeth(f: &Fluvial) -> usize {
    let nx = f.nx;
    let mut n = 0;
    for y in 1..nx - 1 {
        for x in 1..nx - 1 {
            let i = y * nx + x;
            let avg4 = (f.h[i - 1] + f.h[i + 1] + f.h[i - nx] + f.h[i + nx]) * 0.25;
            if f.h[i] - avg4 > 0.8 * f.cell_m {
                n += 1;
            }
        }
    }
    n
}

fn run(label: &str, p: &FluvialParams) {
    // Fine field straight from the prior (L24, ~0.6 m cells) — the mechanism
    // doesn't need parent tiers.
    let mut f = Fluvial::from_prior(0, Face::ZPos, 24, 5_308_288, 13_238_144, 192);
    print!("{label:<26}");
    for round in 0..6 {
        f.erode(p);
        let (n, worst) = spikes(&f);
        print!("  r{round}:{n}/{}t({worst:.1}m)", teeth(&f));
    }
    println!();
}

/// Joseph's live condition: incremental epochs + the conservation pin re-lifting
/// block means every cycle (sustained relative uplift = the tooth amplifier).
fn run_live(label: &str, level: u8, p: &FluvialParams) {
    let shift = 24 - level as i32;
    let mut f = Fluvial::from_prior(0, Face::ZPos, level, 5_308_288 >> shift, 13_238_144 >> shift, 192);
    let seed_h = f.h.clone();
    let (nx, oi, oj) = (f.nx, f.origin.0, f.origin.1);
    let seed_at = move |c: vivarium_world::sphere::CellId| -> f64 {
        let (_, i, j, _) = c.to_face_ij();
        seed_h[((j - oj) as usize) * nx + (i - oi) as usize] as f64
    };
    print!("{label:<26}");
    for round in 0..6 {
        for _ in 0..20 {
            f.erode(&FluvialParams { epochs: 2, ..p.clone() });
            f.pin_block_means(level - 2, &seed_at); // like live tier-under-parent
        }
        let (n, worst) = spikes(&f);
        print!("  r{round}:{n}/{}t({worst:.1}m)", teeth(&f));
    }
    println!();
}

fn main() {
    let base = FluvialParams { epochs: 40, ..Default::default() };
    println!("spikes/teeth (worst overhang) after rounds of 40 epochs, L24 0.6 m cells, 192^2:");
    run("full (with creep)", &base);
    run("no creep", &FluvialParams { diffusivity_m2: 0.0, ..base.clone() });
    run("no creep, no talus", &FluvialParams { diffusivity_m2: 0.0, max_slope: 1e9, ..base.clone() });
    run("creep, no talus", &FluvialParams { max_slope: 1e9, ..base.clone() });
    println!("LIVE-like (2-epoch cycles + mean pin, 40 epochs/round):");
    run_live("L24 live, no creep", 24, &FluvialParams { diffusivity_m2: 0.0, ..base.clone() });
    run_live("L24 live, with creep", 24, &base);
    run_live("L21 live, no creep", 21, &FluvialParams { diffusivity_m2: 0.0, ..base.clone() });
    run_live("L21 live, with creep", 21, &base);
}
