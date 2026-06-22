//! A dependency-free ASCII look at the generated world — a cheap sanity check on
//! the voxel substrate before any 3D renderer exists to show it properly.
//!
//! Run: `cargo run -p vivarium-core --example terrain_preview [seed]`
//!
//! Prints two views of the *generated* (pre-edit) world:
//!   1. a top-down height map over a square region, and
//!   2. a vertical west–east cross-section through `z = 0`, showing materials.
//!
//! It reads only the public [`Volume`] API, so it doubles as a worked example of
//! how a view queries core: ask `voxel(x, y, z)` / `surface_height(x, z)`, never
//! reach inside.

use vivarium_core::voxel::{Voxel, Volume, SEA_LEVEL, WORLD_HEIGHT};

fn main() {
    let seed: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x00C0_FFEE);

    let v = Volume::new(seed);
    println!("vivarium terrain preview — seed {seed:#018x}  (sea level y={SEA_LEVEL})\n");

    top_down_heightmap(&v, -40, 40, -40, 40);
    println!();
    cross_section(&v, -60, 60, 0);
}

/// Top-down: each cell is the surface height at that column, mapped onto a ramp
/// from low (dark) to high (bright). Columns at or below sea level are shown as
/// water so coastlines read at a glance.
fn top_down_heightmap(v: &Volume, x0: i32, x1: i32, z0: i32, z1: i32) {
    const RAMP: &[u8] = b" .:-=+*#%@"; // low -> high
    println!("top-down height map  (x: {x0}..{x1}, z: {z0}..{z1}, '~' = water)");

    // First pass: find the height range so the ramp uses its full resolution.
    let (mut lo, mut hi) = (i32::MAX, i32::MIN);
    for z in z0..=z1 {
        for x in x0..=x1 {
            if let Some(h) = v.surface_height(x, z) {
                lo = lo.min(h);
                hi = hi.max(h);
            }
        }
    }
    let span = (hi - lo).max(1) as f32;

    for z in z0..=z1 {
        let mut line = String::new();
        for x in x0..=x1 {
            let h = v.surface_height(x, z).unwrap_or(0);
            let ch = if h <= SEA_LEVEL {
                '~'
            } else {
                let t = (h - lo) as f32 / span;
                RAMP[((t * (RAMP.len() - 1) as f32).round() as usize).min(RAMP.len() - 1)] as char
            };
            line.push(ch);
            line.push(ch); // double-wide so the aspect ratio reads roughly square
        }
        println!("{line}");
    }
    println!("height range: {lo}..{hi}");
}

/// Vertical slice through a fixed `z`: materials column by column, ground at the
/// bottom. Shows the dirt/grass skin, stone underneath, and water filling lows.
fn cross_section(v: &Volume, x0: i32, x1: i32, z: i32) {
    println!("cross-section at z={z}  (#=stone .=dirt \"=grass ~=water, sea level marked)");
    let top = (WORLD_HEIGHT / 2).min(48); // no need to draw empty sky to the ceiling
    for y in (0..top).rev() {
        let mut line = String::new();
        for x in x0..=x1 {
            line.push(glyph(v.voxel(x, y, z)));
        }
        let marker = if y == SEA_LEVEL { " <- sea" } else { "" };
        println!("{line}{marker}");
    }
}

fn glyph(vox: Voxel) -> char {
    match vox {
        Voxel::STONE => '#',
        Voxel::DIRT => '.',
        Voxel::GRASS => '"',
        Voxel::WATER => '~',
        _ => ' ', // air
    }
}
