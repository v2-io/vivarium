//! How long does a full eroded + watered world take to generate? Times
//! `Volume::eroded` at the Godot bridge's defaults (detail 2, ±6 km, 70 epochs),
//! so the number reflects what a launch actually pays. Build release.
//!
//! Run: `cargo run --release -p vivarium-core --example worldgen_time`

use std::time::Instant;
use vivarium_core::voxel::Volume;

fn main() {
    let t = Instant::now();
    let v = Volume::eroded(0x00C0_FFEE, 2, 6000, 70);
    let dt = t.elapsed();
    // Touch the world so the build can't be optimised away, and report.
    let h = v.surface_height(0, 0).unwrap_or(0);
    println!("worldgen (detail 2, ±6 km, 70 epochs): {:.1} s  [surface@origin = {h}]", dt.as_secs_f32());
}
