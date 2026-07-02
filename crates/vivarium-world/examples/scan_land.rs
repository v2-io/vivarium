//! Scan a cube face for interesting places in the crude baseline — the highest
//! peak and the "best coast" (the window mixing land and sea most evenly). Prints
//! focus coordinates for a view (e.g. worldview's VIVARIUM_FOCUS_I/J), scaled to
//! the requested display level.
//!
//!   cargo run -p vivarium-world --example scan_land [--release]
//!
//! A diagnostic instrument, not worldgen: it just samples `gen::baseline_column`
//! over a coarse grid, so it stays honest to exactly what a view will see.

use vivarium_world::gen::{baseline_column, SEA_LEVEL_M};
use vivarium_world::sphere::{CellId, Face};

fn main() {
    // Scan at a coarse level: 256×256 over the face is plenty to find features.
    const SCAN_LEVEL: u8 = 8;
    const DISPLAY_LEVEL: u8 = 14; // what worldview defaults to
    let n = 1u32 << SCAN_LEVEL;

    let mut height = vec![0.0f64; (n as usize) * (n as usize)];
    for j in 0..n {
        for i in 0..n {
            let col = baseline_column(CellId::from_face_ij(Face::ZPos, i, j, SCAN_LEVEL));
            height[(j as usize) * (n as usize) + (i as usize)] = col.solid_thickness_m() - SEA_LEVEL_M;
        }
    }

    // Highest peak.
    let (mut hi, mut hi_ij) = (f64::MIN, (0u32, 0u32));
    for j in 0..n {
        for i in 0..n {
            let h = height[(j as usize) * (n as usize) + (i as usize)];
            if h > hi {
                hi = h;
                hi_ij = (i, j);
            }
        }
    }

    // Best coast: 16×16 windows scored by how evenly they mix land and sea.
    const W: u32 = 16;
    let (mut best, mut best_ij) = (f64::MIN, (0u32, 0u32));
    for wj in 0..(n / W) {
        for wi in 0..(n / W) {
            let mut land = 0u32;
            for j in 0..W {
                for i in 0..W {
                    let idx = ((wj * W + j) as usize) * (n as usize) + (wi * W + i) as usize;
                    if height[idx] > 0.0 {
                        land += 1;
                    }
                }
            }
            let frac = land as f64 / (W * W) as f64;
            let score = frac * (1.0 - frac); // maximal at a 50/50 mix
            if score > best {
                best = score;
                best_ij = (wi * W + W / 2, wj * W + W / 2);
            }
        }
    }

    let scale = 1u32 << (DISPLAY_LEVEL - SCAN_LEVEL);
    println!("face ZPos, scanned at L{SCAN_LEVEL} ({n}x{n}), coords for L{DISPLAY_LEVEL}:");
    println!(
        "  highest peak : {:>6.0} m   VIVARIUM_FOCUS_I={} VIVARIUM_FOCUS_J={}",
        hi,
        hi_ij.0 * scale,
        hi_ij.1 * scale
    );
    println!(
        "  best coast   : mix {:>4.2}   VIVARIUM_FOCUS_I={} VIVARIUM_FOCUS_J={}",
        best,
        best_ij.0 * scale,
        best_ij.1 * scale
    );
}
