//! A dependency-free look at the erosion spike — does uplift + stream-power
//! erosion actually carve a *dendritic* river network, deterministically?
//!
//! Run: `cargo run -p vivarium-core --example erosion_preview [seed] [epochs]`
//!
//! Prints two views of the matured abstraction-tier heightfield:
//!   1. an elevation map (low → high on a ramp), and
//!   2. a drainage map — cells whose accumulated upstream area crosses a
//!      channel threshold, drawn as rivers. *This* is the one to read: a healthy
//!      result is a branching tree of channels (trunks fed by tributaries fed by
//!      rills), not radial spokes, not noise, not a blank.
//!
//! It reads only the public [`geo`] API. See `crates/vivarium-core/src/geo.rs`
//! for the pipeline and its honest limits (this is a spike, not the system).

use vivarium_core::geo::{ErosionParams, Heightfield};

fn main() {
    let mut args = std::env::args().skip(1);
    let seed: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(0xC0FFEE);
    let epochs: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(60);

    let params = ErosionParams { nx: 96, epochs, ..Default::default() };
    let field = Heightfield::simulate(&params, seed);

    println!(
        "vivarium erosion spike — seed {seed:#x}, {epochs} epochs, {n}×{n} grid\n",
        n = params.nx
    );
    elevation_map(&field);
    println!();
    drainage_map(&field, &params);
}

/// Elevation as a brightness ramp. Ridgelines read bright, valleys dark — you
/// should see connected valley floors threading between ridges, not a smooth dome.
fn elevation_map(f: &Heightfield) {
    const RAMP: &[u8] = b" .:-=+*#%@";
    let (lo, hi) = f.elevation_range();
    let span = (hi - lo).max(1e-6);
    println!("elevation  (low ' ' -> high '@', range {lo:.2}..{hi:.2})");
    // Sub-sample to keep the print roughly terminal-sized.
    let step = (f.nx / 72).max(1);
    for y in (0..f.nx).step_by(step) {
        let mut line = String::new();
        for x in (0..f.nx).step_by(step) {
            let t = (f.h[f.idx(x, y)] - lo) / span;
            let c = RAMP[((t * (RAMP.len() - 1) as f32).round() as usize).min(RAMP.len() - 1)];
            line.push(c as char);
        }
        println!("{line}");
    }
}

/// Drainage as a thresholded channel network. Three glyphs by river order so the
/// branching structure is legible: rills `·`, tributaries `+`, trunks `█`.
fn drainage_map(f: &Heightfield, p: &ErosionParams) {
    let cell_area = p.cell_size * p.cell_size;
    let total = ((p.nx - 2) * (p.nx - 2)) as f32 * cell_area;
    // Channel onset at a small fraction of the basin; trunk/tributary tiers above.
    let rill = total * 0.004;
    let trib = total * 0.02;
    let trunk = total * 0.08;
    println!("drainage  (· rill   + tributary   █ trunk river)");
    let step = (f.nx / 72).max(1);
    for y in (0..f.nx).step_by(step) {
        let mut line = String::new();
        for x in (0..f.nx).step_by(step) {
            let a = f.drainage[f.idx(x, y)];
            let c = if a >= trunk {
                '#'
            } else if a >= trib {
                '+'
            } else if a >= rill {
                '.'
            } else {
                ' '
            };
            line.push(c);
        }
        println!("{line}");
    }
}
