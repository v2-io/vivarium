//! # GCL and ABF probe — the two unrun checks from the structure-preservation survey
//!
//! Pre-registered predictions were written **before the first run**, in
//! `msc/agent-briefs/2026-07-29-gcl-and-abf-checks.md` §0 (`#norm-probe-sensitivity`).
//!
//! Two questions, five arms:
//!
//! * **A (ABF)** — is Arnold–Boffi–Falk 2005's "RT0 has no divergence convergence on
//!   non-affine quads" the mechanism behind the grid report's *measured* "two-point flux
//!   inconsistent, order −0.5"? ABF's counterexample mesh holds cell shape **fixed** under
//!   refinement (all elements similar to one right trapezoid). A cube-sphere quadtree cell
//!   is the image of a smooth map, so it should become affine under refinement. A1/A2
//!   measure the same non-affineness diagnostic on both meshes; A3 is the separator —
//!   a uniform sheared **parallelogram** lattice, where ABF is provably exempt (affine ⇒
//!   constant Jacobian) yet the two-point flux is non-orthogonal.
//!
//! * **G (GCL)** — do our metrics satisfy a discrete geometric conservation law? Derived
//!   in the landing note §2.2: cube-sphere cell edges are exactly great-circle arcs, the
//!   outward in-surface normal along such an arc is constant (the arc's pole), and the
//!   surface divergence theorem gives the exact closure identity
//!
//!       Σ_e L_e p̂_e + 2 ∫_K r̂ dΩ = 0      (unit sphere)
//!
//!   which is the spherical replacement for the planar Σ_e L_e n̂_e = 0. G1 tests it with
//!   an independently quadratured ∫_K r̂ dΩ. G2 is the free-stream test on a
//!   solid-body-rotation field (exactly divergence-free on S²) across three metric sets,
//!   including the uniform-Cartesian set `water.rs` actually uses — that arm is the
//!   known-bad required by `#norm-probe-sensitivity` FE(2): if it passes, this probe has
//!   no discrimination.
//!
//! Run: `cargo run --release -p vivarium-world --example gcl_abf_probe`

use vivarium_world::measure;
use vivarium_world::sphere::{CubeCoord, Face};

type V3 = [f64; 3];

fn sub(a: V3, b: V3) -> V3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn add(a: V3, b: V3) -> V3 { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn scale(a: V3, s: f64) -> V3 { [a[0] * s, a[1] * s, a[2] * s] }
fn dot(a: V3, b: V3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: V3) -> f64 { dot(a, a).sqrt() }
fn unit(a: V3) -> V3 { scale(a, 1.0 / norm(a)) }

/// Unit vector at face-parameter (u, v).
fn at_uv(face: Face, u: f64, v: f64) -> V3 { CubeCoord { face, u, v }.to_unit() }

/// The four corners of cell (i, j) at `level`, in order c00, c10, c11, c01.
fn corners(face: Face, i: u64, j: u64, level: u8) -> [V3; 4] {
    let (u0, v0) = measure::corner_uv(i, j, level);
    let (u1, v1) = measure::corner_uv(i + 1, j + 1, level);
    [
        at_uv(face, u0, v0),
        at_uv(face, u1, v0),
        at_uv(face, u1, v1),
        at_uv(face, u0, v1),
    ]
}

// ───────────────────────── A: the ABF non-affineness diagnostic ─────────────────────────

/// Bilinear cross-term of a quad, normalised by its mean edge length.
///
/// δ = |c00 − c10 + c11 − c01| is exactly zero for a parallelogram (affine image of the
/// reference square, constant Jacobian) and is the leading non-affine term otherwise.
/// ABF's hypothesis is that δ/h stays **fixed** as h → 0.
fn affine_defect_ratio(c: [V3; 4]) -> f64 {
    let cross_term = add(sub(c[0], c[1]), sub(c[2], c[3]));
    let h = 0.25
        * (norm(sub(c[1], c[0])) + norm(sub(c[2], c[1])) + norm(sub(c[3], c[2])) + norm(sub(c[0], c[3])));
    norm(cross_term) / h
}

/// A1 — δ/h over a cube face, per level. Prediction: decays ~O(h), halving per level.
fn probe_a1() {
    println!("\n## A1 — cube-sphere non-affineness δ/h vs level");
    println!("   (δ = |c00−c10+c11−c01|, normalised by mean edge length; 0 = parallelogram)");
    println!("{:>6} {:>14} {:>14} {:>10} {:>10}", "level", "max δ/h", "mean δ/h", "max ratio", "mean ratio");
    let face = Face::ZPos;
    let mut prev: Option<(f64, f64)> = None;
    for level in 2..=10u8 {
        let n = 1u64 << level;
        let (mut mx, mut sum) = (0.0f64, 0.0f64);
        for j in 0..n {
            for i in 0..n {
                let d = affine_defect_ratio(corners(face, i, j, level));
                mx = mx.max(d);
                sum += d;
            }
        }
        let mean = sum / (n * n) as f64;
        let (rmx, rmean) = match prev {
            Some((pm, pmean)) => (pm / mx, pmean / mean),
            None => (f64::NAN, f64::NAN),
        };
        println!("{level:>6} {mx:>14.6e} {mean:>14.6e} {rmx:>10.3} {rmean:>10.3}");
        prev = Some((mx, mean));
    }
    println!("   ratio ≈ 2.0 ⇒ δ/h = O(h): cells become parallelograms under refinement");
    println!("   ratio ≈ 1.0 ⇒ shape fixed under refinement ⇒ ABF's hypothesis holds for us");
}

/// A2 — the same diagnostic on ABF's own trapezoid mesh (their §3, α = 1, β = 1/3).
///
/// Their T1 tiles the unit square with four congruent trapezoids; T_h applies T1 to each
/// of n² subsquares. We measure δ/h for the archetype element K1 with vertices
/// (0,0), (1/2,0), (1/2, 1−β), (0, β), scaled by the subsquare size 1/n.
fn probe_a2() {
    println!("\n## A2 — ABF's trapezoid mesh (α=1, β=1/3), same diagnostic");
    println!("{:>6} {:>14} {:>10}", "1/h", "δ/h", "ratio");
    let beta = 1.0 / 3.0;
    let mut prev: Option<f64> = None;
    for k in 2..=10u32 {
        let n = 1u32 << k;
        let s = 1.0 / n as f64;
        // K1 corners in the subsquare, lifted to 3-space with z = 0 so the same helper applies.
        let c: [V3; 4] = [
            [0.0, 0.0, 0.0],
            [0.5 * s, 0.0, 0.0],
            [0.5 * s, (1.0 - beta) * s, 0.0],
            [0.0, beta * s, 0.0],
        ];
        let d = affine_defect_ratio(c);
        let r = prev.map_or(f64::NAN, |p| p / d);
        println!("{n:>6} {d:>14.6e} {r:>10.3}");
        prev = Some(d);
    }
    println!("   ratio ≈ 1.0 ⇒ non-affineness is h-independent — ABF's hypothesis, by construction");
}

/// A3 — the separator. A uniform sheared **parallelogram** lattice: affine, so δ ≡ 0 and
/// ABF is provably exempt; non-orthogonal, so the two-point flux should still be wrong.
///
/// Periodic domain, lattice vectors a1 = (1, 0)·h, a2 = (s, 1)·h. Exact field
/// u = sin(2πx) sin(2πy), Δu = −8π² u. Discrete Laplacian by the two-point flux
/// Σ_f (u_nbr − u_i) · L_f / d_f / A, with L_f the true edge length, d_f the true
/// centre-to-centre distance, A the true cell area.
fn probe_a3() {
    println!("\n## A3 — two-point flux on a sheared PARALLELOGRAM lattice (ABF-exempt, δ ≡ 0)");
    println!("   shear s = 0.5 ⇒ lattice angle 63.43°, non-orthogonality 26.57°");
    println!("{:>6} {:>16} {:>10} {:>16} {:>10}", "n", "rel L2 (2-pt)", "order", "rel L2 (orth ctrl)", "order");
    let s = 0.5f64;
    let mut prev: Option<(f64, f64)> = None;
    for k in 3..=9u32 {
        let n = 1u32 << k;
        let h = 1.0 / n as f64;
        // Lattice vectors and the geometry the flux needs.
        let a1 = [h, 0.0];
        let a2 = [s * h, h];
        let area = (a1[0] * a2[1] - a1[1] * a2[0]).abs();
        // Face separating i from i+a1: its edge vector is a2, so L = |a2|; centre distance |a1|.
        let (l1, d1) = ((a2[0] * a2[0] + a2[1] * a2[1]).sqrt(), (a1[0] * a1[0] + a1[1] * a1[1]).sqrt());
        let (l2, d2) = ((a1[0] * a1[0] + a1[1] * a1[1]).sqrt(), (a2[0] * a2[0] + a2[1] * a2[1]).sqrt());
        let tp = |p: i64, q: i64| -> f64 {
            let x = p as f64 * a1[0] + q as f64 * a2[0];
            let y = p as f64 * a1[1] + q as f64 * a2[1];
            (std::f64::consts::TAU * x).sin() * (std::f64::consts::TAU * y).sin()
        };
        let (mut e2, mut r2) = (0.0f64, 0.0f64);
        let (mut e2o, mut r2o) = (0.0f64, 0.0f64);
        for q in 0..n as i64 {
            for p in 0..n as i64 {
                let u0 = tp(p, q);
                let lap_true = -2.0 * std::f64::consts::TAU * std::f64::consts::TAU * u0;
                // Two-point flux over the 4 lattice neighbours.
                let f = (tp(p + 1, q) + tp(p - 1, q) - 2.0 * u0) * l1 / d1
                    + (tp(p, q + 1) + tp(p, q - 1) - 2.0 * u0) * l2 / d2;
                let lap_h = f / area;
                e2 += (lap_h - lap_true).powi(2);
                r2 += lap_true.powi(2);
                // Orthogonal control: the SAME scheme on an unsheared lattice of the same h.
                let tq = |pp: i64, qq: i64| -> f64 {
                    let x = pp as f64 * h;
                    let y = qq as f64 * h;
                    (std::f64::consts::TAU * x).sin() * (std::f64::consts::TAU * y).sin()
                };
                let u0o = tq(p, q);
                let lap_true_o = -2.0 * std::f64::consts::TAU * std::f64::consts::TAU * u0o;
                let lap_ho = (tq(p + 1, q) + tq(p - 1, q) + tq(p, q + 1) + tq(p, q - 1) - 4.0 * u0o) / (h * h);
                e2o += (lap_ho - lap_true_o).powi(2);
                r2o += lap_true_o.powi(2);
            }
        }
        let (err, erro) = ((e2 / r2).sqrt(), (e2o / r2o).sqrt());
        let (o1, o2) = match prev {
            Some((pe, peo)) => ((pe / err).log2(), (peo / erro).log2()),
            None => (f64::NAN, f64::NAN),
        };
        println!("{n:>6} {err:>16.6e} {o1:>10.3} {erro:>16.6e} {o2:>10.3}");
        prev = Some((err, erro));
    }
    println!("   sheared column not converging ⇒ measured defect present where ABF is absent");
    println!("   ⇒ the two defects DISSOCIATE: the correspondence is refuted");
}

/// A4 — the follow-up A3 forced. A3 *saturated* (order → 0); the grid report measured
/// −0.50, which **grows**. A uniform lattice has no metric *variation*, so A4 repeats the
/// same scheme on a smoothly varying-shear lattice: physical position
/// (x, y) = (ξ, η + a·sin(2πξ)/(2π)), periodic in both, cells still asymptotically affine
/// (so still ABF-exempt) but with non-orthogonality that varies from cell to cell.
///
/// Prediction, written first: honestly uncertain — see the landing note §0.
fn probe_a4() {
    println!("\n## A4 — two-point flux on a smoothly VARYING-shear lattice (still ABF-exempt)");
    println!("   (x, y) = (ξ, η + a·sin(2πξ)/(2π)),  a = 0.6");
    println!("{:>6} {:>16} {:>10} {:>14} {:>14}", "n", "rel L2 (2-pt)", "order", "max δ/h", "δ/h ratio");
    let a = 0.6f64;
    let tau = std::f64::consts::TAU;
    let map = |xi: f64, eta: f64| -> [f64; 2] { [xi, eta + a * (tau * xi).sin() / tau] };
    let field = |p: [f64; 2]| -> f64 { (tau * p[0]).sin() * (tau * p[1]).sin() };
    let lap_true = |p: [f64; 2]| -> f64 { -2.0 * tau * tau * field(p) };
    let mut prev: Option<(f64, f64)> = None;
    for k in 3..=9u32 {
        let n = 1u32 << k;
        let h = 1.0 / n as f64;
        // Cell (p, q) spans ξ ∈ [p h, (p+1) h], η ∈ [q h, (q+1) h].
        let corner = |p: i64, q: i64| map(p as f64 * h, q as f64 * h);
        let centre = |p: i64, q: i64| map((p as f64 + 0.5) * h, (q as f64 + 0.5) * h);
        let d2 = |u: [f64; 2], v: [f64; 2]| ((u[0] - v[0]).powi(2) + (u[1] - v[1]).powi(2)).sqrt();
        let quad_area = |c: [[f64; 2]; 4]| -> f64 {
            let mut s = 0.0;
            for e in 0..4 {
                let (u, v) = (c[e], c[(e + 1) % 4]);
                s += u[0] * v[1] - v[0] * u[1];
            }
            0.5 * s.abs()
        };
        let (mut e2, mut r2, mut maxd) = (0.0f64, 0.0f64, 0.0f64);
        for q in 0..n as i64 {
            for p in 0..n as i64 {
                let c = [corner(p, q), corner(p + 1, q), corner(p + 1, q + 1), corner(p, q + 1)];
                let area = quad_area(c);
                // non-affineness of this cell, for the ABF-exemption check
                let cross_term = [c[0][0] - c[1][0] + c[2][0] - c[3][0], c[0][1] - c[1][1] + c[2][1] - c[3][1]];
                let hh = 0.25 * (d2(c[1], c[0]) + d2(c[2], c[1]) + d2(c[3], c[2]) + d2(c[0], c[3]));
                maxd = maxd.max((cross_term[0].powi(2) + cross_term[1].powi(2)).sqrt() / hh);
                let x0 = centre(p, q);
                let u0 = field(x0);
                // four faces: +ξ (edge c1–c2), −ξ (edge c0–c3), +η (edge c3–c2), −η (edge c0–c1)
                let faces: [([f64; 2], [f64; 2], [f64; 2]); 4] = [
                    (c[1], c[2], centre(p + 1, q)),
                    (c[0], c[3], centre(p - 1, q)),
                    (c[3], c[2], centre(p, q + 1)),
                    (c[0], c[1], centre(p, q - 1)),
                ];
                let mut flux = 0.0;
                for (ea, eb, xn) in faces {
                    let l = d2(ea, eb);
                    let d = d2(xn, x0);
                    flux += (field(xn) - u0) * l / d;
                }
                let lh = flux / area;
                let lt = lap_true(x0);
                e2 += (lh - lt).powi(2);
                r2 += lt.powi(2);
            }
        }
        let err = (e2 / r2).sqrt();
        let (o, dr) = match prev {
            Some((pe, pd)) => ((pe / err).log2(), pd / maxd),
            None => (f64::NAN, f64::NAN),
        };
        println!("{n:>6} {err:>16.6e} {o:>10.3} {maxd:>14.6e} {dr:>14.3}");
        prev = Some((err, maxd));
    }
    println!("   δ/h ratio ≈ 2 confirms the lattice is asymptotically affine ⇒ ABF-exempt");
    println!("   order < 0 ⇒ metric VARIATION is the growth mechanism behind the measured −0.50");
    println!("   order ≈ 0 ⇒ growth has a source none of these probes isolates — see note §4");
}

/// A5 — the decisive follow-up A4 forced: is the grid report's −0.50 a *growing* error or
/// the coarse-level approach to an O(1) plateau?
///
/// `grid_lab` §7 computes that order from exactly one pair (N=32 → N=64). Both planar
/// controls saturate, with their largest negative orders in precisely that coarse regime.
/// Here we run the same TPFA scheme on our own equiangular cube-sphere against an exact
/// degree-2 spherical harmonic, **face interior only** — which needs no cross-face
/// adjacency, and which the grid report's own far-field split licenses (the 24 defects were
/// tested and refuted as the cause).
///
/// Exact eigenfunction: p(x) = (x·a)(x·b) − (a·b)|x|²/3 is a harmonic homogeneous
/// polynomial of degree 2, so on the unit sphere Δ_S u = −6u.
fn probe_a5() {
    println!("\n## A5 — TPFA on OUR cube-sphere vs an exact ℓ=2 harmonic (face interior)");
    println!("   grid_lab §7 reports order −0.50 from the single pair N=32 → N=64");
    println!("{:>6} {:>16} {:>10}", "N/face", "rel L2 (TPFA)", "order");
    let face = Face::ZPos;
    // Deliberately not aligned to any cube axis.
    let a = unit([0.31, 0.82, -0.48]);
    let b = unit([-0.65, 0.27, 0.71]);
    let ab = dot(a, b);
    let u_of = |r: V3| -> f64 { dot(r, a) * dot(r, b) - ab / 3.0 };
    let mut prev: Option<f64> = None;
    for k in 5..=10u32 {
        let n = 1u64 << k;
        let level = k as u8;
        let (mut e2, mut r2) = (0.0f64, 0.0f64);
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let c = corners(face, i, j, level);
                let interior = unit(add(add(c[0], c[1]), add(c[2], c[3])));
                let ctr = measure::cell_center_unit(face, i, j, level);
                let area = measure::cell_solid_angle(face, i, j, level);
                let u0 = u_of(ctr);
                // Four edge-neighbours, paired with the shared edge (c[e], c[e+1]).
                let nbrs: [((u64, u64), (V3, V3)); 4] = [
                    ((i + 1, j), (c[1], c[2])),
                    ((i - 1, j), (c[3], c[0])),
                    ((i, j + 1), (c[2], c[3])),
                    ((i, j - 1), (c[0], c[1])),
                ];
                let mut flux = 0.0;
                for ((ni, nj), (ea, eb)) in nbrs {
                    let l = arc_len(ea, eb);
                    let nc = measure::cell_center_unit(face, ni, nj, level);
                    let d = arc_len(ctr, nc);
                    flux += (u_of(nc) - u0) * l / d;
                }
                let lap_h = flux / area;
                let lap_true = -6.0 * u0;
                e2 += (lap_h - lap_true).powi(2);
                r2 += lap_true.powi(2);
                let _ = interior;
            }
        }
        let err = (e2 / r2).sqrt();
        let o = prev.map_or(f64::NAN, |p: f64| (p / err).log2());
        println!("{n:>6} {err:>16.6e} {o:>10.3}");
        prev = Some(err);
    }
    println!("   order climbing toward 0 with err plateauing ⇒ the scheme converges to the WRONG");
    println!("   operator (O(1) inconsistency); the −0.50 is the coarse-level approach to that plateau");
    println!("   order staying ≈ −0.5 out to N=1024 ⇒ the error genuinely grows — hypothesis refuted");
}

// ───────────────────────── G: the discrete GCL on our metrics ─────────────────────────

/// Pole of the great circle through unit points a and b (the constant outward in-surface
/// normal along that arc), oriented outward from `interior`.
fn edge_pole(a: V3, b: V3, interior: V3) -> V3 {
    let p = unit(cross(a, b));
    if dot(p, interior) > 0.0 { scale(p, -1.0) } else { p }
}

/// Great-circle arc length between two unit vectors.
fn arc_len(a: V3, b: V3) -> f64 { norm(cross(a, b)).atan2(dot(a, b)) }

/// ∫_K r̂ dΩ by tensor Gauss–Legendre quadrature in (u, v) — deliberately an
/// **independent** route from the closed-form solid angle, so G1 is a real cross-check.
fn integral_rhat(face: Face, i: u64, j: u64, level: u8, ng: usize) -> V3 {
    // 8-point Gauss–Legendre nodes/weights on [-1, 1].
    const X: [f64; 8] = [
        -0.9602898564975363, -0.7966664774136267, -0.5255324099163290, -0.1834346424956498,
        0.1834346424956498, 0.5255324099163290, 0.7966664774136267, 0.9602898564975363,
    ];
    const W: [f64; 8] = [
        0.1012285362903763, 0.2223810344533745, 0.3137066458778873, 0.3626837833783620,
        0.3626837833783620, 0.3137066458778873, 0.2223810344533745, 0.1012285362903763,
    ];
    assert_eq!(ng, 8);
    let (u0, v0) = measure::corner_uv(i, j, level);
    let (u1, v1) = measure::corner_uv(i + 1, j + 1, level);
    let (hu, cu) = (0.5 * (u1 - u0), 0.5 * (u1 + u0));
    let (hv, cv) = (0.5 * (v1 - v0), 0.5 * (v1 + v0));
    let mut acc = [0.0; 3];
    for (xu, wu) in X.iter().zip(W.iter()) {
        let u = cu + hu * xu;
        for (xv, wv) in X.iter().zip(W.iter()) {
            let v = cv + hv * xv;
            // Gnomonic tangents and the solid-angle element dΩ = dX dY / (1+X²+Y²)^{3/2},
            // with dX/du = (π/4) sec²(πu/4).
            let (x, y) = ((u * std::f64::consts::FRAC_PI_4).tan(), (v * std::f64::consts::FRAC_PI_4).tan());
            let jx = std::f64::consts::FRAC_PI_4 * (1.0 + x * x);
            let jy = std::f64::consts::FRAC_PI_4 * (1.0 + y * y);
            let dom = jx * jy / (1.0 + x * x + y * y).powf(1.5);
            let r = at_uv(face, u, v);
            let w = wu * wv * hu * hv * dom;
            acc = add(acc, scale(r, w));
        }
    }
    acc
}

/// G1 — the closure identity Σ_e L_e p̂_e + 2 ∫_K r̂ dΩ = 0, per cell.
fn probe_g1() {
    println!("\n## G1 — spherical closure identity  Σ_e L_e p̂_e + 2∫_K r̂ dΩ = 0");
    println!("   (edges as great-circle arcs; ∫ r̂ by independent 8×8 Gauss–Legendre)");
    println!("{:>6} {:>10} {:>16} {:>16} {:>14}", "level", "cell", "|residual|", "Σ L_e", "relative");
    let face = Face::ZPos;
    for level in [3u8, 5, 7, 9] {
        let n = 1u64 << level;
        // Three positions: face centre, edge midpoint, corner (a defect cell).
        for (label, i, j) in [("centre", n / 2, n / 2), ("edge-mid", n / 2, 0), ("corner", 0, 0)] {
            let c = corners(face, i, j, level);
            let interior = unit(add(add(c[0], c[1]), add(c[2], c[3])));
            let mut sum: V3 = [0.0; 3];
            let mut lsum = 0.0;
            for e in 0..4 {
                let (a, b) = (c[e], c[(e + 1) % 4]);
                let l = arc_len(a, b);
                lsum += l;
                sum = add(sum, scale(edge_pole(a, b, interior), l));
            }
            let rhat = integral_rhat(face, i, j, level, 8);
            let resid = norm(add(sum, scale(rhat, 2.0)));
            println!("{level:>6} {label:>10} {resid:>16.6e} {lsum:>16.6e} {:>14.6e}", resid / lsum);
        }
    }
    println!("   relative ~1e-15 ⇒ the identity holds: our cells are honest spherical polygons");
    println!("   NOTE sensitivity: the identity is homogeneous — a metric set that is");
    println!("   uniformly mis-scaled passes. It catches INCONSISTENCY, not common-mode error.");
}

/// G2 — free-stream test. Discrete FV divergence of a solid-body rotation field
/// v = ω × r, which is exactly divergence-free on S². Three metric sets.
fn probe_g2() {
    println!("\n## G2 — free-stream: discrete div of solid-body rotation (true div ≡ 0)");
    println!("   arms: (a) exact edge integrals  (b) midpoint rule + true metrics");
    println!("         (c) uniform-Cartesian set — what water.rs actually carries [KNOWN-BAD]");
    println!(
        "{:>6} {:>16} {:>8} {:>16} {:>8} {:>16} {:>8}",
        "level", "(a) exact", "order", "(b) midpoint", "order", "(c) uniform", "order"
    );
    let face = Face::ZPos;
    let omega: V3 = unit([0.3, -0.7, 0.5]); // deliberately not aligned to any face axis
    let mut prev: Option<(f64, f64, f64)> = None;
    for level in 3..=9u8 {
        let n = 1u64 << level;
        // A modest window in the face interior plus the three special positions, to keep
        // cost flat while still sampling defect geometry.
        let lo = 0u64;
        let hi = n.min(64);
        let (mut sa, mut sb, mut sc, mut cnt) = (0.0f64, 0.0f64, 0.0f64, 0usize);
        // Uniform-Cartesian stand-in: one cell size for every length, its square for area.
        let cell_ang = 2.0 / n as f64 * std::f64::consts::FRAC_PI_2; // rough face-centre cell arc
        for j in lo..hi {
            for i in lo..hi {
                let c = corners(face, i, j, level);
                let interior = unit(add(add(c[0], c[1]), add(c[2], c[3])));
                let area = measure::cell_solid_angle(face, i, j, level);
                let (mut fa, mut fb, mut fc) = (0.0f64, 0.0f64, 0.0f64);
                for e in 0..4 {
                    let (a, b) = (c[e], c[(e + 1) % 4]);
                    let l = arc_len(a, b);
                    let p = edge_pole(a, b, interior);
                    // (a) exact: ∫_e (ω×r)·p ds = ω·(r_end − r_start)  (telescopes to 0)
                    fa += dot(omega, sub(b, a));
                    // (b) midpoint rule with the true L and p
                    let m = unit(scale(add(a, b), 0.5));
                    fb += l * dot(cross(omega, m), p);
                    // (c) uniform-Cartesian: same normal-component sample, but every edge
                    //     given the same nominal length — the metric set water.rs carries.
                    fc += cell_ang * dot(cross(omega, m), p);
                }
                sa += (fa / area).powi(2);
                sb += (fb / area).powi(2);
                sc += (fc / (cell_ang * cell_ang)).powi(2);
                cnt += 1;
            }
        }
        let (ra, rb, rc) = ((sa / cnt as f64).sqrt(), (sb / cnt as f64).sqrt(), (sc / cnt as f64).sqrt());
        let ord = |p: f64, c: f64| (p / c).log2();
        let (oa, ob, oc) = match prev {
            Some((pa, pb, pc)) => (ord(pa, ra), ord(pb, rb), ord(pc, rc)),
            None => (f64::NAN, f64::NAN, f64::NAN),
        };
        println!("{level:>6} {ra:>16.6e} {oa:>8.2} {rb:>16.6e} {ob:>8.2} {rc:>16.6e} {oc:>8.2}");
        prev = Some((ra, rb, rc));
    }
    println!("   (a) at round-off and flat ⇒ the GCL's exact half is COMBINATORIAL, not metric");
    println!("   (c) failing to converge is the known-bad arm: if it passed, this probe is blind");
}

fn main() {
    println!("# GCL and ABF probe — predictions pre-registered in");
    println!("#   msc/agent-briefs/2026-07-29-gcl-and-abf-checks.md §0");
    probe_a1();
    probe_a2();
    probe_a3();
    probe_a4();
    probe_a5();
    probe_g1();
    probe_g2();
}
