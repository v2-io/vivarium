//! Fluvial-erosion preview — the GPU-free instrument for the erosion port. Seeds
//! a region from the band-limited prior, runs the ported pipeline, and prints the
//! DRAINAGE map (the dendritic-ness check: rills → tributaries → trunks joining at
//! confluences) plus before/after relief + slope stats.
//!
//!   cargo run --release -p vivarium-world --example erosion_preview \
//!       [level] [oi] [oj] [nx] [epochs]

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::gen::SEA_LEVEL_M;
use vivarium_world::sphere::Face;

const COLS: usize = 128;
const ROWS: usize = 56;

fn stats(h: &[f32], nx: usize, cell_m: f32) -> (f32, f32, f64, f64) {
    let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
    let (mut ssum, mut smax, mut cnt) = (0.0f64, 0.0f64, 0u64);
    for y in 0..nx {
        for x in 0..nx - 1 {
            let a = h[y * nx + x];
            let b = h[y * nx + x + 1];
            lo = lo.min(a);
            hi = hi.max(a);
            let s = ((a - b).abs() / cell_m) as f64;
            ssum += s;
            smax = smax.max(s);
            cnt += 1;
        }
    }
    (lo, hi, ssum / cnt as f64, smax)
}

fn main() {
    let arg = |k: usize, d: f64| std::env::args().nth(k).and_then(|s| s.parse().ok()).unwrap_or(d);
    let level = arg(1, 19.0) as u8;
    let oi = arg(2, 165_888.0 - 192.0) as u32; // default: the known massif, centred
    let oj = arg(3, 413_696.0 - 192.0) as u32;
    let nx = arg(4, 384.0) as usize;
    let epochs = arg(5, 80.0) as u32;

    let t0 = std::time::Instant::now();
    let mut f = Fluvial::from_prior(Face::ZPos, level, oi, oj, nx);
    let seed_ms = t0.elapsed().as_millis();
    let (lo0, hi0, smean0, smax0) = stats(&f.h, nx, f.cell_m);

    let t1 = std::time::Instant::now();
    let params = FluvialParams { epochs, ..Default::default() };
    f.erode(&params);
    let erode_ms = t1.elapsed().as_millis();
    let (lo1, hi1, smean1, smax1) = stats(&f.h, nx, f.cell_m);

    // Drainage map: log-scaled area bands. Channels are cells whose drainage
    // far exceeds one cell's area; sea prints as '~'.
    let cell_area = f.cell_m * f.cell_m;
    let (si, sj) = ((nx / COLS).max(1), (nx / ROWS).max(1));
    println!(
        "fluvial L{level} nx={nx} cell {:.0} m window {:.1} km  epochs {epochs}  seed {seed_ms} ms  erode {erode_ms} ms",
        f.cell_m,
        nx as f32 * f.cell_m / 1000.0
    );
    for r in 0..ROWS.min(nx / sj) {
        let line: String = (0..COLS.min(nx / si))
            .map(|c| {
                // Max drainage within the character's cell block (a thin channel
                // must not be lost to subsampling).
                let mut d = 0.0f32;
                let mut under_sea = false;
                for jj in 0..sj {
                    for ii in 0..si {
                        let idx = (r * sj + jj) * nx + (c * si + ii);
                        d = d.max(f.drainage[idx]);
                        under_sea |= f.h[idx] <= SEA_LEVEL_M as f32;
                    }
                }
                if under_sea {
                    '~'
                } else {
                    let ratio = (d / cell_area).log10();
                    match ratio {
                        r if r < 0.8 => ' ',
                        r if r < 1.4 => '.',
                        r if r < 2.0 => ':',
                        r if r < 2.6 => '=',
                        r if r < 3.2 => '#',
                        _ => '@',
                    }
                }
            })
            .collect();
        println!("{line}");
    }
    println!("channels: ' '<6x cell area  '.'  ':'  '='  '#'  '@'>1600x   '~' sea");
    println!(
        "relief before {lo0:.0}..{hi0:.0} m (slope mean {:.1}% max {:.1}%)  after {lo1:.0}..{hi1:.0} m (slope mean {:.1}% max {:.1}%)",
        smean0 * 100.0,
        smax0 * 100.0,
        smean1 * 100.0,
        smax1 * 100.0
    );
}
