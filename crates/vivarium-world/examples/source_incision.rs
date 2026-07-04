//! Single-column probe for armor_regimes regime 3's EXACT-zero incision
//! (ORIENTATION open investigation #1b). One narrow domain, one source cell,
//! instrumented at the arithmetic level: what e does the erosion branch
//! compute, and does `bed -= e` actually change the f32?
//!
//! Hypothesis under test: f32 ABSORPTION. bed sits at a ~6000 m datum where
//! one ULP is ~0.0005 m; any per-step erosion increment below the half-ULP
//! (~0.00024 m) rounds away entirely — the branch fires, `sediment += e`
//! receives mass (created from nothing), armor grows via into_parent, and
//! the bed is EXACTLY unchanged forever. Continuous physics can't make an
//! exact zero; float rounding can.
use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

fn main() {
    // The exact arithmetic claim, isolated:
    let bed0 = 5995.2f32;
    let e = 1.5e-4f32;
    println!("arithmetic: {bed0} - {e} == {bed0} ? {}", bed0 - e == bed0);
    println!("f32 ULP at {bed0}: {:e}\n", f32::from_bits(bed0.to_bits() + 1) - bed0);

    // Same ramp geometry as armor_regimes, narrow strip.
    let nx = 96usize;
    let cell = 4.8f32;
    let mut bed = vec![0.0f32; nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            let across = (x as f32 - nx as f32 / 2.0) * cell;
            let down = y as f32 * cell;
            let ramp_end = nx as f32 * cell * 0.66;
            let h = if down < ramp_end { 6000.0 - down * 0.05 } else { 6000.0 - ramp_end * 0.05 };
            bed[y * nx + x] = h + (across / 40.0).powi(2).min(30.0);
        }
    }
    let mut w = WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, cell, bed, 5.0);
    let p0 = WaterParams::default();
    let p = WaterParams { precip: p0.precip * 10.0, ..p0 };
    let (cx, y_top) = (nx / 2, 20usize);
    let i = y_top * nx + cx;
    let bed_start = w.bed[i];
    let solid_start = w.total_solid();
    let mut t = 0.0f32;
    let mut printed = 0;
    while t < 600.0 {
        let dt = w.stable_dt(9.8);
        let pp = WaterParams { dt, ..p };
        // Reproduce the erosion branch's own arithmetic for the source cell,
        // BEFORE stepping, so we see the e the kernel is about to apply.
        let d = w.depth[i];
        if d >= 1e-3 && printed < 8 && t > 60.0 {
            let l = cell;
            let out = 0.0f32.max(w.depth[i]); // depth known; recompute slope/tau as the kernel does
            let _ = out;
            let mut slope = 0.0f32;
            for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let j = ((y_top as i32 + dy) as usize) * nx + (cx as i32 + dx) as usize;
                slope = slope.max((w.bed[i] - w.bed[j]) / l);
            }
            let tau = 1000.0 * 9.8 * d * slope;
            // capacity needs speed; approximate with depth-normalized pipe sum
            // (post-hoc — indicative only, the kernel's own value is what acts).
            println!(
                "t={t:>5.1} d={d:.4} slope={slope:.3} tau={tau:.2} sed={:.2e} bed={:.4} (Δbed so far {:+.6})",
                w.sediment[i], w.bed[i], w.bed[i] - bed_start
            );
            printed += 1;
        }
        for _ in 0..(8.0 / dt) as u32 {
            w.step(&pp);
            t += dt;
        }
    }
    println!("\nafter 600 ss: bed Δ = {:+.6} m (exact zero? {})", w.bed[i] - bed_start, w.bed[i] == bed_start);
    println!("armor at source: {:.2}  suspended: {:.2e}  sed_bed: {:.4}", w.armor[i], w.sediment[i], w.sed_bed[i]);
    // Solid-mass audit: pre-fix, the bed absorbed the subtraction while
    // sediment received it, so total solid GREW (and a relative-1e-6 test
    // tolerance hid it — since replaced by an absolute bound in water.rs).
    println!("total_solid drift: {:+.6} m·cells (absolute; conserves test bounds this at 1e-3).", w.total_solid() - solid_start);
}
