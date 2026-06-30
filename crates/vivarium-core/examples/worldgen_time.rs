//! How long does a full eroded + watered world take to generate? Times
//! `Volume::eroded` at the Godot bridge's defaults (detail 2, ±6 km, 70 epochs),
//! so the number reflects what a launch actually pays. Build release.
//!
//! Run: `cargo run --release -p vivarium-core --example worldgen_time`

use vivarium_core::voxel::{Volume, Voxel};
use std::time::Instant;

fn main() {
    // args: [region_half_m] [epochs]  (defaults: full 6 km world, 70 epochs)
    let mut a = std::env::args().skip(1);
    let region: i32 = a.next().and_then(|s| s.parse().ok()).unwrap_or(6000);
    let epochs: u32 = a.next().and_then(|s| s.parse().ok()).unwrap_or(70);
    let t = Instant::now();
    let v = Volume::eroded(0x00C0_FFEE, 2, region, epochs);
    let dt = t.elapsed();
    println!("worldgen (detail 2, ±{region} m, {epochs} epochs): {:.1} s", dt.as_secs_f32());

    // Cross-section of the ACTUAL rendered voxels along z=0 — terrain (▓) and the
    // water column on top (≈), read only through the public voxel API. A real lake
    // must show a FLAT water top; a draped surface steps with the bed.
    let detail = v.detail() as i32;
    let half = 11_000 * detail / 10; // a bit inside ±12 km (voxels)
    let cols = 150usize;
    let step = (2 * half / cols as i32).max(1);
    let xs: Vec<i32> = (0..cols as i32).map(|i| -half + i * step).collect();

    // Per column: terrain top, and water top (scan up from the ground while WATER).
    let mut ground = Vec::new();
    let mut wtop = Vec::new();
    for &x in &xs {
        let g = v.surface_height(x, 0).unwrap_or(0);
        let mut y = g + 1;
        while v.voxel(x, y, 0) == Voxel::WATER && y < g + 4000 {
            y += 1;
        }
        ground.push(g);
        wtop.push((y - 1).max(g)); // = g when dry
    }
    let lo = *ground.iter().min().unwrap();
    let hi = *wtop.iter().max().unwrap().max(ground.iter().max().unwrap());
    let rows = 30i32;
    let span = (hi - lo).max(1);
    println!("\ncross-section z=0  (terrain ▓, water ≈;  flat ≈-top = real water)");
    for r in 0..rows {
        let level = hi - (span * r) / rows;
        let line: String = (0..cols)
            .map(|c| {
                if level <= ground[c] {
                    '#'
                } else if level <= wtop[c] {
                    '~'
                } else {
                    ' '
                }
            })
            .collect();
        println!("{line}");
    }
    let wet = wtop.iter().zip(&ground).filter(|(w, g)| *w > *g).count();
    println!("wet columns: {wet}/{cols}");

    // Top-down water map: '~' = sea, '#' = inland water (a stream/lake standing
    // above sea level), ' ' = dry land. Shows whether a river network formed.
    let sea = v.sea_level();
    let rows = 50usize;
    let zstep = (2 * half / rows as i32).max(1);
    let xstep = (2 * half / 110).max(1);
    println!("\ntop-down water  (' ' land, '~' sea, '#' inland water):");
    for r in 0..rows {
        let z = -half + r as i32 * zstep;
        let line: String = (0..110)
            .map(|c| {
                let x = -half + c as i32 * xstep;
                let g = v.surface_height(x, z).unwrap_or(0);
                let mut yy = g + 1;
                while v.voxel(x, yy, z) == Voxel::WATER && yy < g + 4000 {
                    yy += 1;
                }
                if yy - 1 <= g {
                    ' ' // dry
                } else if g < sea {
                    '~' // sea
                } else {
                    '#' // inland water above sea
                }
            })
            .collect();
        println!("{line}");
    }
}
