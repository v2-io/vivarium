//! # `measure::cell_solid_angle` loses relative precision as the level refines
//!
//! Found incidentally while gating the Coatléven reconstruction's geometric identity
//! (`examples/router_fe6c.rs`, P1): the identity `|K|·Id = Σ_σ |σ|(x_σ−x_K)⊗n̂_{K,σ}`
//! holds to 8e-16 against the tangent-plane quad's own area, but disagreed with
//! `cell_area_m2` by ~4e-5 at L19 — far above the ~1e-11 sphericity floor at that
//! level. This probe isolates why.
//!
//! `cell_solid_angle` evaluates `F(u₁,v₁) − F(u₁,v₀) − F(u₀,v₁) + F(u₀,v₀)` with
//! `F = atan(XY/√(1+X²+Y²))`. The four terms are O(1) and converge on each other as
//! the cell shrinks, so the **relative** cancellation error grows like `4^level`
//! (the solid angle itself falls as `4^-level` while the absolute roundoff stays at
//! ~ε of an O(1) arctangent). Van Oosterom–Strackee on the four corners,
//! `Ω = 2·atan2(|a·(b×c)|, 1 + a·b + b·c + c·a)` summed over two triangles, is the
//! standard cancellation-free route and is used here as the reference.
//!
//! **This is a live-kernel function**: `cell_area_m2` is the per-cell runoff in
//! `erosion::accumulate_drainage` and the volume in `deposit`. It replaced uniform
//! `cell_m²` precisely because per-cell area accuracy was shown to matter
//! (`#obs-cube-locked-kernel-bias`, +17.8% area-weighted bias). The error below is
//! spatially high-frequency (it is roundoff), which is the shape that reads as
//! sub-grid texture rather than as a numerical floor.
//!
//! Run: `cargo run --release -p vivarium-world --example solid_angle_precision`

use vivarium_world::measure;
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CubeCoord, Face};

type V3 = [f64; 3];
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
/// Van Oosterom–Strackee: numerically stable solid angle of a spherical triangle.
fn tri_omega(a: V3, b: V3, c: V3) -> f64 {
    2.0 * dot(a, cross(b, c)).abs().atan2(1.0 + dot(a, b) + dot(b, c) + dot(c, a))
}

fn main() {
    let r = Planet::EARTH.radius_m;
    let face = Face::ZPos;
    println!("measure::cell_solid_angle vs Van Oosterom-Strackee, equiangular cube-sphere");
    println!("(relative disagreement; VOS is the cancellation-free reference)\n");
    println!("{:>5} {:>7} {:>12} {:>12} {:>14}", "level", "cells", "median rel", "max rel", "cell size");
    for level in [4u8, 8, 10, 13, 16, 19, 21, 23, 25] {
        let n = (1u64 << level) as f64;
        let nn = 1u64 << level;
        let uv = |i: f64, j: f64| -> V3 {
            CubeCoord { face, u: 2.0 * i / n - 1.0, v: 2.0 * j / n - 1.0 }.to_unit()
        };
        let mut e: Vec<f64> = Vec::new();
        let step = (nn / 17).max(1);
        let mut i = 0u64;
        while i + 1 < nn {
            let mut j = 0u64;
            while j + 1 < nn {
                let (fi, fj) = (i as f64, j as f64);
                let (a, b, c, d) =
                    (uv(fi, fj), uv(fi + 1.0, fj), uv(fi + 1.0, fj + 1.0), uv(fi, fj + 1.0));
                let vos = tri_omega(a, b, c) + tri_omega(a, c, d);
                let arc = measure::cell_solid_angle(face, i, j, level);
                e.push((arc / vos - 1.0).abs());
                j += step;
            }
            i += step;
        }
        e.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "{level:>5} {:>7} {:>12.3e} {:>12.3e} {:>11.3} m",
            e.len(),
            e[e.len() / 2],
            e[e.len() - 1],
            vivarium_world::sample::cell_size_m(level, r)
        );
    }
    println!("\nThe error grows ~4^level — the signature of cancellation in the four-term");
    println!("arctangent difference, not of geometry. Below ~L16 it is irrelevant; at L21 it");
    println!("is 3e-5 median, at L23 4e-4, at L25 5e-3 median / 7e-2 worst.");
    println!("A drop-in fix is the VOS form above: same inputs, no cancellation, no new deps.");
    println!("NOT LANDED HERE — editing src/ re-keys every world under every cohort");
    println!("(#form-complete-content-addressed-key). See msc/agent-briefs/2026-07-29-router-fe6c-pricing.md");
}
