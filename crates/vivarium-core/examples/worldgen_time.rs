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

    // Cross-section of the ACTUAL rendered voxels — terrain (▓) and the water
    // column on top (≈), read through the public voxel API. The point is to see the
    // water SURFACE: a real pool shows a FLAT ≈-top; a thin sheet draped on a slope
    // staircases. We auto-pick the wettest row so the transect actually crosses
    // water, and report the max depth (thin sheet vs real pool).
    let detail = v.detail() as i32;
    let half = region * detail * 9 / 10; // voxels, just inside the patch
    let sea = v.sea_level();
    let cols = 150usize;
    let xstep = (2 * half / cols as i32).max(1);

    let inland_at = |x: i32, z: i32| -> bool {
        let g = v.surface_height(x, z).unwrap_or(0);
        g > sea && v.voxel(x, g + 1, z) == Voxel::WATER
    };
    let probe = |z: i32| (0..cols).filter(|&c| inland_at(-half + c as i32 * xstep, z)).count();
    let zp = (2 * half / 60).max(1);
    let best_z = (0..60)
        .map(|i| -half + i * zp)
        .max_by_key(|&z| probe(z))
        .unwrap_or(0);

    let mut ground = Vec::new();
    let mut wtop = Vec::new();
    for c in 0..cols {
        let x = -half + c as i32 * xstep;
        let g = v.surface_height(x, best_z).unwrap_or(0);
        let mut y = g + 1;
        while v.voxel(x, y, best_z) == Voxel::WATER && y < g + 4000 {
            y += 1;
        }
        ground.push(g);
        wtop.push((y - 1).max(g));
    }
    let lo = *ground.iter().min().unwrap();
    let hi = *wtop.iter().chain(ground.iter()).max().unwrap();
    let rows = 30i32;
    let span = (hi - lo).max(1);
    println!("\ncross-section z={best_z} (wettest row)  (terrain ▓, water ≈; flat ≈-top = pool)");
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
    let maxd = wtop.iter().zip(&ground).map(|(w, g)| w - g).max().unwrap_or(0);
    println!(
        "wet columns: {wet}/{cols};  max water depth this row: {:.1} m (small = thin sheet)",
        maxd as f32 * 0.5
    );

    // Top-down water map: '~' = sea, '#' = inland water, ' ' = dry land.
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
