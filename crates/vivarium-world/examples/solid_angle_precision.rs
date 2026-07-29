//! # Precision of `measure::cell_solid_angle` — before/after, against a real reference
//!
//! Found incidentally while gating the Coatléven reconstruction's geometric identity
//! (`examples/router_fe6c.rs`, P1): the identity `|K|·Id = Σ_σ |σ|(x_σ−x_K)⊗n̂_{K,σ}`
//! holds to 8e-16 against the tangent-plane quad's own area, but disagreed with
//! `cell_area_m2` by ~4e-5 at L19 — far above the ~1e-11 sphericity floor there.
//!
//! ## What this probe measures, and the reference it uses
//!
//! Four routes to the same number:
//!
//! * **`legacy`** — the closed form that shipped until 2026-07-29:
//!   `F(u₁,v₁) − F(u₁,v₀) − F(u₀,v₁) + F(u₀,v₀)` with `F = atan(XY/√(1+X²+Y²))`.
//!   Four O(1) terms whose difference is the tiny solid angle.
//! * **`vos_naive`** — Van Oosterom–Strackee with the triple product taken straight
//!   from the corner components.
//! * **`LIVE`** — whatever `measure::cell_solid_angle` currently does (VOS with the
//!   triple product formed as `a·((b−a)×(c−a))`).
//! * **`quad`** — the reference: tensor Gauss–Legendre on the Jacobian
//!   `dΩ = (π/4)² sec²(πu/4) sec²(πv/4) / (1+X²+Y²)^{3/2} du dv`. This is a sum of
//!   **positive** terms times an **exactly representable** cell size, so it has no
//!   cancellation at any level. Its own convergence is reported (1× vs 2× panel
//!   subdivision) and is ~2e-16 everywhere — that is what licenses it as truth.
//!
//! ## The trap this probe exists to stop being re-fallen-into
//!
//! The first version of this finding used **`vos_naive` as the reference** and
//! reported the legacy error against it. That was wrong in a way only a real
//! reference exposes: `vos_naive` degrades as `4^level` **too** (1.8e-3 at L25),
//! merely ~7× slower than the legacy form. Shipping "VOS is the drop-in fix" would
//! have bought a factor of 7 while claiming a factor of a million. **The difference
//! reformulation of the triple product is the load-bearing part**, and nothing but
//! an independent reference could have said so.
//!
//! ## Why it mattered
//!
//! `cell_area_m2` is consumed by the live fluvial kernel (`erosion.rs` → per-cell
//! runoff in `accumulate_drainage`, volumes in `deposit`). It *replaced* uniform
//! `cell_m²` precisely because per-cell area accuracy was shown to matter
//! (`#obs-cube-locked-kernel-bias`, +17.8% area-weighted bias). At the walk-scale
//! tiers the repo contemplates (L23–L25, `MAX_LEVEL = 25`) the legacy form carried
//! 0.04%–0.5% median and up to 7% worst-case per-cell noise, spatially
//! high-frequency — the shape that reads as sub-grid texture, not as a floor.
//!
//! Run: `cargo run --release -p vivarium-world --example solid_angle_precision`

use std::f64::consts::FRAC_PI_4;
use vivarium_world::measure;
use vivarium_world::sphere::{CubeCoord, Face};

type V3 = [f64; 3];
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn corner(u: f64, v: f64) -> V3 {
    CubeCoord { face: Face::ZPos, u, v }.to_unit()
}

/// The closed form that shipped until 2026-07-29. Kept as the regression guard.
fn legacy(u0: f64, v0: f64, u1: f64, v1: f64) -> f64 {
    let f = |u: f64, v: f64| -> f64 {
        let x = (u * FRAC_PI_4).tan();
        let y = (v * FRAC_PI_4).tan();
        (x * y / (1.0 + x * x + y * y).sqrt()).atan()
    };
    (f(u1, v1) - f(u1, v0) - f(u0, v1) + f(u0, v0)).abs()
}

/// VOS with the triple product straight from the components — NOT the fix.
fn vos_naive(a: V3, b: V3, c: V3) -> f64 {
    2.0 * dot(a, cross(b, c)).abs().atan2(1.0 + dot(a, b) + dot(b, c) + dot(c, a))
}

const GX: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.5255324099163290,
    -0.1834346424956498,
    0.1834346424956498,
    0.5255324099163290,
    0.7966664774136267,
    0.9602898564975363,
];
const GW: [f64; 8] = [
    0.1012285362903763,
    0.2223810344533745,
    0.3137066458778873,
    0.3626837833783620,
    0.3626837833783620,
    0.3137066458778873,
    0.2223810344533745,
    0.1012285362903763,
];

/// The reference: `n×n` panels of tensor Gauss–Legendre-8 on the Jacobian.
/// Positive terms times an exactly-representable cell size — cancellation-free.
fn quad(u0: f64, u1: f64, v0: f64, v1: f64, n: usize) -> f64 {
    let hu = (u1 - u0) / n as f64;
    let hv = (v1 - v0) / n as f64;
    let mut s = 0.0;
    for pu in 0..n {
        for pv in 0..n {
            let (au, av) = (u0 + hu * pu as f64, v0 + hv * pv as f64);
            let mut t = 0.0;
            for (i, gxi) in GX.iter().enumerate() {
                for (j, gxj) in GX.iter().enumerate() {
                    let u = au + hu * 0.5 * (1.0 + gxi);
                    let v = av + hv * 0.5 * (1.0 + gxj);
                    let (cu, cv) = ((u * FRAC_PI_4).cos(), (v * FRAC_PI_4).cos());
                    let (x, y) = ((u * FRAC_PI_4).tan(), (v * FRAC_PI_4).tan());
                    t += GW[i] * GW[j] * FRAC_PI_4 * FRAC_PI_4
                        / (cu * cu * cv * cv * (1.0 + x * x + y * y).powf(1.5));
                }
            }
            s += t * hu * hv * 0.25;
        }
    }
    s
}

fn main() {
    println!("cell_solid_angle precision, relative to Gauss-Legendre quadrature of the Jacobian");
    println!("(legacy = the closed form retired 2026-07-29; LIVE = measure::cell_solid_angle today)\n");
    println!(
        "{:>5} {:>12} {:>12} {:>12} {:>12} {:>11}",
        "level", "cell", "legacy", "vos_naive", "LIVE", "quad conv"
    );
    let radius = vivarium_world::planet::Planet::EARTH.radius_m;
    for level in [4u8, 8, 10, 13, 16, 19, 21, 23, 25] {
        let nn = 1u64 << level;
        // Off-centre, off-symmetry cell — a face centre would be a null test.
        let (i, j) = (nn / 3 + 7, nn / 5 + 3);
        let (u0, v0) = measure::corner_uv(i, j, level);
        let (u1, v1) = measure::corner_uv(i + 1, j + 1, level);
        let (a, b, c, d) = (corner(u0, v0), corner(u1, v0), corner(u1, v1), corner(u0, v1));

        let reference = quad(u0, u1, v0, v1, 2);
        let conv = quad(u0, u1, v0, v1, 1) / reference - 1.0;
        let leg = legacy(u0, v0, u1, v1) / reference - 1.0;
        let vn = (vos_naive(a, b, c) + vos_naive(a, c, d)) / reference - 1.0;
        let live = measure::cell_solid_angle(Face::ZPos, i, j, level) / reference - 1.0;
        println!(
            "{level:>5} {:>10.3} m {:>12.3e} {:>12.3e} {:>12.3e} {:>11.3e}",
            vivarium_world::sample::cell_size_m(level, radius),
            leg,
            vn,
            live,
            conv
        );
    }
    println!("\nReading: `legacy` and `vos_naive` both grow as ~4^level (vos_naive only ~7x");
    println!("better, which is why it is NOT the fix). `LIVE` is flat from L16 on, ~1e-10.");
    println!("`quad conv` ~2e-16 everywhere is what licenses the quadrature as the reference.");
    println!("\nThe legacy->LIVE delta IS the legacy error: ~1e-15 at L4, ~6e-13 at L8,");
    println!("~5e-10 at L13, ~3e-6 at L19, ~3e-4 at L23, ~1e-2 at L25. Every stored world is");
    println!("re-keyed by this change, BY DESIGN (Joseph, 2026-07-29: \"Don't worry about");
    println!("rekeying -- get the truth in place. Let the cache worry about the cache.\").");

    // Regression guard: a future 'simplification' back to a cancelling form fails
    // HERE, loudly, rather than silently at L23 inside a landscape.
    let mut worst = 0.0f64;
    for level in [13u8, 16, 19, 21, 23, 25] {
        let nn = 1u64 << level;
        for (i, j) in [(nn / 3 + 7, nn / 5 + 3), (nn / 2 + 1, nn / 7 + 5), (nn - 3, nn / 11 + 2)] {
            let (u0, v0) = measure::corner_uv(i, j, level);
            let (u1, v1) = measure::corner_uv(i + 1, j + 1, level);
            let r = quad(u0, u1, v0, v1, 2);
            worst = worst.max((measure::cell_solid_angle(Face::ZPos, i, j, level) / r - 1.0).abs());
        }
    }
    println!("\nGUARD: worst |LIVE/quad - 1| over L13..L25, 3 cells each = {worst:.3e}");
    assert!(worst < 1e-8, "cell_solid_angle has lost precision: {worst:.3e} (expected ~4e-10)");
    println!("GUARD PASS (< 1e-8)");
}
