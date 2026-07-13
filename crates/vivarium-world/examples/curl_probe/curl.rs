//! THE CURL PROBE — does our drainage routing violate a TOPOLOGICAL IDENTITY?
//!
//! ## The identity, stated correctly (this is the whole design)
//!
//! Gravity-driven flow is a **gradient flow** on the hydraulic head `φ = z + h`.
//!
//! The naive statement of the consequence — *"the flow field must be curl-free"* — is
//! **FALSE, and building the probe on it would have produced a number that convicts a
//! perfect router.** The discharge is `q = −K∇φ` with a spatially varying conveyance `K`
//! (depth, roughness, discharge). Its curl is
//!
//! ```text
//!   ∇×q = ∇×(−K∇φ) = −∇K × ∇φ    ≠ 0   in general.
//! ```
//!
//! **Real water on a real hillslope has nonzero `∇×q`.** So `∇×q` is not the invariant, and
//! any probe measuring it has no honest control.
//!
//! What IS exact — pointwise, on every mesh, at every resolution, independent of `K` — is:
//!
//! > # **The flow direction is PARALLEL to −∇φ, hence ORTHOGONAL to the contours of φ.**
//!
//! from which the topological invariant follows immediately. For any closed loop `C` that
//! lies **in a level set of φ**, the integrand is *identically zero at every point*, so
//!
//! ```text
//!   κ(C)  =  ∮_C d̂ · dl  /  ∮_C dl   =   0     EXACTLY.        d̂ = the router's unit outflow direction
//! ```
//!
//! and writing `Δ` for the signed angle from the true steepest descent to `d̂`,
//!
//! ```text
//!   d̂ · t̂  =  sin Δ                  ⇒   κ(C) = ⟨ sin Δ ⟩ along C.
//! ```
//!
//! **κ is the fraction of the transport that goes AROUND the hill instead of DOWN it.**
//!
//! ## Why κ ≠ 0 is a violated identity and not merely an error
//!
//! `κ(C) ≠ 0` on a closed contour means the flow has a **net** tangential component around
//! that contour: it *spirals*. A spiralling descent is orthogonal only to a family of
//! spirals, and those are the level sets of a **multivalued** function (the azimuth is not
//! single-valued on an annulus). So:
//!
//! > **κ(C) ≠ 0 ⟺ the router's direction field is the gradient flow of NO single-valued
//! > potential.** The implied head does not close: go around the loop and the water surface
//! > has dropped by a nonzero amount and yet returned to where it started. An Escher
//! > staircase. **No refinement fixes it, because it is not an accuracy statement.**
//!
//! `κ = 0` on every loop ⟹ a potential exists ⟹ the structure survives, and a nonzero
//! *pointwise* deflection is "wrong, but honestly wrong" — an error, not a broken identity.
//!
//! ## ⚠ THE DISTINCTION THIS PROBE EXISTS TO DRAW
//!
//! The measured fan bias makes the cube-face axes **ATTRACTORS**
//! (`DECISIONS[mfd-fan-is-a-bias-and-does-not-converge]`). **An attractor is a CONVERGENT
//! deformation, and convergence is IRROTATIONAL.** A purely convergent field can drift a
//! plume 474 km off its meridian with *exactly zero circulation*. **So the plume drift does
//! NOT imply nonzero curl, and it must not be allowed to stand in as evidence for it.**
//! Whether the deflection field carries a **solenoidal** part on top of its convergent part
//! is a genuinely open question that the fan probe did not answer, and it is the question here.
//!
//! ## The operator, and why its control cannot fail
//!
//! Everything is built on the **error field**, both terms evaluated at the SAME point (the
//! cell centre), both unit vectors:
//!
//! ```text
//!   E_i  :=  d̂_i  −  f̂_i          f̂ = the exact steepest-descent direction of φ (analytic)
//! ```
//!
//! For a router that is exactly right, `E_i` is **the zero vector at every cell** — so every
//! summand of every quadrature below is *identically* zero and the operator returns machine
//! zero **structurally**, on any loop, any mesh, any level, any quadrature rule. That is the
//! control that cannot fail. It is not a tolerance; it is an algebraic identity.

#![allow(dead_code)]

use super::fan::{assumed_dist, wrap180, OFF, P};
use super::grids::{cube_face_basis, cube_to_unit, CubeProj};
use super::mesh::*;

/// Distance rule: what the router believes about neighbour distances.
#[derive(Clone, Copy, PartialEq)]
pub enum DistRule {
    /// `erosion.rs` today: `cell_m` axial, `cell_m·√2` diagonal.
    Hardcoded,
    /// The one-line fix: the TRUE great-circle centre-to-centre distances.
    True,
}

// ===========================================================================
// Potentials — analytic, so ∇φ is EXACT and the control is exact.
// ===========================================================================

/// A scalar field on the sphere with an analytic gradient.
pub trait Potential: Sync {
    /// Head in metres at a unit-sphere point.
    fn h(&self, p: V3, radius_m: f64) -> f64;
    /// ∇h in the tangent plane at `p`, in metres of head per metre of arc (i.e. a slope).
    fn grad(&self, p: V3, radius_m: f64) -> V3;
    /// Unit steepest-descent direction (the direction water must go). `None` at a flat point.
    fn descent(&self, p: V3, radius_m: f64) -> Option<V3> {
        let g = self.grad(p, radius_m);
        let n = norm(g);
        if n < 1e-12 { None } else { Some(scale(g, -1.0 / n)) }
    }
}

/// **The perfect cone.** `h = −R·θ` about a pole: slope exactly 1 everywhere, flow lines are
/// EXACT MERIDIANS, contours are EXACT circles. The standard drainage test (Tarboton 1997),
/// and the one whose exact answer `flow.rs` already scores routers against.
pub struct Cone {
    pub pole: V3,
}
impl Potential for Cone {
    fn h(&self, p: V3, radius_m: f64) -> f64 {
        -radius_m * geodesic(p, self.pole)
    }
    fn grad(&self, p: V3, _r: f64) -> V3 {
        // ∇(−R·θ) has magnitude 1 (per metre of arc) and points TOWARD the pole.
        let t = sub(self.pole, scale(p, dot(p, self.pole)));
        let n = norm(t);
        if n < 1e-12 { return [0.0; 3] }
        scale(t, 1.0 / n)
    }
}

/// **Real terrain** — the same band-limited plane-wave sum every grid in `grid_lab` is scored
/// on (`flow::terrain`), re-derived here so that its gradient is ANALYTIC rather than a finite
/// difference. Amplitude scaled to metres.
pub struct Terrain {
    waves: Vec<(f64, f64, V3, f64)>, // (amp, k, dir, phase)
    pub scale_m: f64,
}
impl Terrain {
    pub fn new(scale_m: f64) -> Self {
        let mut st = 0x9E3779B97F4A7C15u64;
        let mut rnd = || {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            (st >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
        };
        let mut waves = Vec::new();
        for oct in 0..6 {
            let k = 2.0f64.powi(oct) * 2.0;
            let amp = 1.0 / k;
            for _ in 0..8 {
                let d = unit([rnd(), rnd(), rnd()]);
                let ph = rnd() * std::f64::consts::PI;
                waves.push((amp, k, d, ph));
            }
        }
        Terrain { waves, scale_m }
    }
}
impl Potential for Terrain {
    fn h(&self, p: V3, _r: f64) -> f64 {
        self.scale_m
            * self.waves.iter().map(|&(a, k, d, ph)| a * (k * dot(p, d) + ph).sin()).sum::<f64>()
    }
    fn grad(&self, p: V3, radius_m: f64) -> V3 {
        // d/dp [ a·sin(k p·d + ph) ] = a·k·cos(...)·d ; project into the tangent plane,
        // then convert from "per unit of the unit sphere" to "per metre of arc" (÷ R).
        let mut g = [0.0f64; 3];
        for &(a, k, d, ph) in &self.waves {
            let c = a * k * (k * dot(p, d) + ph).cos();
            g = add(g, scale(d, c));
        }
        let g = sub(g, scale(p, dot(p, g))); // tangential part
        scale(g, self.scale_m / radius_m)
    }
}

// ===========================================================================
// The cube-sphere: forward, inverse, and the true Moore fan at a cell.
// ===========================================================================

/// `sample::cell_size_m` — the ONE number `erosion.rs` uses for the whole patch.
pub fn cell_m(level: u32, radius_m: f64) -> f64 {
    (std::f64::consts::PI / 2.0) * radius_m / (1u64 << level) as f64
}

/// Cell centre of face cell `(i, j)` at `level` — literally `CellId::to_cube().to_unit()`.
pub fn cell_center(level: u32, f: usize, i: i64, j: i64) -> V3 {
    let n = (1i64 << level) as f64;
    let c = |k: i64| ((k as f64 + 0.5) / n) * 2.0 - 1.0;
    cube_to_unit(CubeProj::Equiangular, f, c(i), c(j))
}

/// **Inverse** of the equiangular map: which face is `p` on, and at what `(u, v)`?
/// `x = tan(u·π/4)` ⇒ `u = (4/π)·atan((p·a)/(p·c))`.
pub fn unit_to_cube(p: V3) -> (usize, f64, f64) {
    let mut best = (0usize, f64::NEG_INFINITY);
    for f in 0..6 {
        let (c, _, _) = cube_face_basis(f);
        let d = dot(p, c);
        if d > best.1 {
            best = (f, d);
        }
    }
    let f = best.0;
    let (c, a, b) = cube_face_basis(f);
    let w = dot(p, c);
    let u = (dot(p, a) / w).atan() * 4.0 / std::f64::consts::PI;
    let v = (dot(p, b) / w).atan() * 4.0 / std::f64::consts::PI;
    (f, u, v)
}

/// The face-cell index containing `p` at `level`, clamped into the face-interior range
/// (so all 8 Moore neighbours exist on the same face — every cell but the outermost ring).
pub fn cell_of(p: V3, level: u32) -> (usize, i64, i64) {
    let (f, u, v) = unit_to_cube(p);
    let n = 1i64 << level;
    let idx = |t: f64| ((t + 1.0) * 0.5 * n as f64).floor() as i64;
    (f, idx(u).clamp(1, n - 2), idx(v).clamp(1, n - 2))
}

// ===========================================================================
// THE ROUTERS — reproduced from `erosion.rs`, run on the TRUE sphere geometry.
// ===========================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Router {
    /// `erosion.rs::accumulate_drainage` verbatim: 8 Moore neighbours, weights
    /// `(drop/dist)^1.1`, `dist` hardcoded to `cell_m` / `cell_m·√2`.
    Mfd8,
    /// The same fan with the TRUE great-circle distances — isolates how much of the defect is
    /// the hardcoded distances and how much is the fan's non-uniform ANGULAR sampling, which
    /// no distance can repair.
    Mfd8TrueDist,
    /// MFD restricted to the 4 real EDGE neighbours. No diagonals.
    Mfd4,
    /// **Gradient-projected edge flux** (D-∞ on a mesh) — the proposed replacement. The flow
    /// DIRECTION is taken from the reconstructed gradient; the split is across the real edges.
    GradEdge,
    // ---- controls (not routers; injected direction fields) ----
    /// CONTROL: route exactly down the true gradient. `E ≡ 0` ⇒ machine zero, structurally.
    ExactControl,
    /// CONTROL: the true gradient rotated by a CONSTANT angle. Known closed-form circulation.
    /// The probe MUST see this, or a zero it reports means nothing.
    RotConstControl(f64),
    /// CONTROL: the true gradient rotated by `A·cos(2ψ)` about the cone pole — a LARGE
    /// pointwise deflection that is ODD around any contour. The probe MUST return ~0 here.
    /// **This is the control that decides whether the probe measures CIRCULATION or merely
    /// DEFLECTION MAGNITUDE** — i.e. whether it is anything more than the fan probe renamed.
    RotQuadControl(f64, V3),
    /// CONTROL: MFD-8 on a PERFECT SQUARE LATTICE (bearings exactly k·45°, distances exactly
    /// cell / cell·√2). MFD's OWN intrinsic circulation floor, with the sphere removed. Every
    /// sphere number below is charged against this.
    Mfd8FlatControl,
}

impl Router {
    pub fn label(self) -> String {
        match self {
            Router::Mfd8 => "MFD-8 (erosion.rs, status quo)".into(),
            Router::Mfd8TrueDist => "MFD-8 + true distances".into(),
            Router::Mfd4 => "edge MFD-4 (no diagonals)".into(),
            // ⚠ NOT the implementable router. This takes the direction STRAIGHT from the
            // reconstructed gradient with NO discrete split among neighbour cells, so it is a
            // REFERENCE (effectively a second control), and its ~1e-13 is a property of the
            // gradient reconstruction, not a promise about gradient-projected edge flux.
            // The implementable router's real number is measured on the mesh in §5 — and it is
            // 4.5e-3, NOT zero, because the SPLIT reintroduces a fan. That gap is the finding.
            Router::GradEdge => "REF: gradient direction, no split".into(),
            Router::ExactControl => "CONTROL exact gradient".into(),
            Router::RotConstControl(a) => format!("CONTROL rotate {a:+.0}° const"),
            Router::RotQuadControl(a, _) => format!("CONTROL rotate {a:.0}°·cos2ψ"),
            Router::Mfd8FlatControl => "CONTROL MFD-8 on a perfect flat lattice".into(),
        }
    }
    pub fn is_control(self) -> bool {
        !matches!(self, Router::Mfd8 | Router::Mfd8TrueDist | Router::Mfd4 | Router::GradEdge)
    }
}

/// Rotate the unit tangent `d` by `deg` CCW about the outward normal at `p`.
pub fn rot(p: V3, d: V3, deg: f64) -> V3 {
    let (s, c) = deg.to_radians().sin_cos();
    let left = cross(p, d); // +90° CCW
    unit(add(scale(d, c), scale(left, s)))
}

/// **The unit outflow direction the router asserts at cell `(f,i,j)`, level `L`.**
///
/// The neighbour geometry is the TRUE sphere geometry (real bearings, real distances); the
/// heights are the ACTUAL potential sampled at the TRUE neighbour cell centres. That is
/// exactly what `erosion.rs` does — it just does not know that its `(dx,dy)` offsets land
/// where they land.
pub fn direction(
    r: Router,
    phi: &dyn Potential,
    level: u32,
    f: usize,
    i: i64,
    j: i64,
    radius_m: f64,
) -> Option<V3> {
    let c = cell_center(level, f, i, j);
    let cm = cell_m(level, radius_m);

    // Controls that inject a direction field directly.
    match r {
        Router::ExactControl => return phi.descent(c, radius_m),
        Router::RotConstControl(a) => return phi.descent(c, radius_m).map(|d| rot(c, d, a)),
        Router::RotQuadControl(a, pole) => {
            let d = phi.descent(c, radius_m)?;
            // ψ = azimuth of the cell about the pole, in an arbitrary but fixed frame.
            let e0 = tangent(pole, if dot(pole, [0.0, 0.0, 1.0]).abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] });
            let e1 = cross(pole, e0);
            let t = tangent(pole, c);
            let psi = dot(t, e1).atan2(dot(t, e0));
            return Some(rot(c, d, a * (2.0 * psi).cos()));
        }
        Router::Mfd8FlatControl => {
            // The same MFD weights on a PERFECT lattice: bearings exactly k·45° about the
            // true flow direction's frame, distances exactly cell / cell·√2. The sphere is
            // gone; only MFD's own quadrature error remains.
            let fdir = phi.descent(c, radius_m)?;
            // Build a local frame whose e0 is the *grid's* u-axis (so the flow azimuth
            // relative to the grid is preserved — that is what MFD's error depends on).
            let e0 = tangent(c, cell_center(level, f, i + 1, j));
            let e1 = cross(c, e0);
            let phi_az = dot(fdir, e1).atan2(dot(fdir, e0));
            let (mut rx, mut ry) = (0.0f64, 0.0f64);
            for k in 0..8 {
                let b = (k as f64 * 45.0).to_radians();
                let dist = assumed_dist(k, cm);
                let drop = dist * (b - phi_az).cos(); // exact plane drop on a flat lattice
                if drop <= 0.0 {
                    continue;
                }
                let w = (drop / dist).powf(P);
                rx += w * b.cos();
                ry += w * b.sin();
            }
            if rx == 0.0 && ry == 0.0 {
                return None;
            }
            let a = ry.atan2(rx);
            return Some(unit(add(scale(e0, a.cos()), scale(e1, a.sin()))));
        }
        _ => {}
    }

    // --- the real routers, on the real geometry ---
    let hc = phi.h(c, radius_m);
    let e0 = tangent(c, cell_center(level, f, i + 1, j));
    let e1 = cross(c, e0);

    match r {
        Router::Mfd8 | Router::Mfd8TrueDist | Router::Mfd4 => {
            let (mut rx, mut ry) = (0.0f64, 0.0f64);
            let mut tot = 0.0f64;
            for (k, &(dx, dy)) in OFF.iter().enumerate() {
                let diag = dx != 0 && dy != 0;
                if r == Router::Mfd4 && diag {
                    continue; // no diagonals: edges only
                }
                let nb = cell_center(level, f, i + dx, j + dy);
                let drop = hc - phi.h(nb, radius_m);
                if drop <= 0.0 {
                    continue;
                }
                let true_d = geodesic(c, nb) * radius_m;
                let dist = match r {
                    Router::Mfd8 => assumed_dist(k, cm),
                    Router::Mfd4 => cm,
                    _ => true_d,
                };
                let w = (drop / dist).powf(P);
                // the mass goes to where the neighbour ACTUALLY is
                let t = tangent(c, nb);
                let b = dot(t, e1).atan2(dot(t, e0));
                rx += w * b.cos();
                ry += w * b.sin();
                tot += w;
            }
            if tot <= 0.0 || (rx == 0.0 && ry == 0.0) {
                return None;
            }
            let a = ry.atan2(rx);
            Some(unit(add(scale(e0, a.cos()), scale(e1, a.sin()))))
        }
        Router::GradEdge => {
            // Least-squares gradient over the 4 edge neighbours, projected onto the real
            // edge normals. (On the cube-sphere's interior the edge normal and the bearing
            // to the neighbour differ; the mesh-based Tier B uses the true normals. Here the
            // interior cell's four edges are well approximated by the bearing bisectors, and
            // the whole-mesh cross-check in §4 is what pins this.)
            let (mut sxx, mut sxy, mut syy, mut sxz, mut syz) = (0.0, 0.0, 0.0, 0.0, 0.0);
            for &(dx, dy) in &[(1i64, 0i64), (-1, 0), (0, 1), (0, -1)] {
                let nb = cell_center(level, f, i + dx, j + dy);
                let d = sub(nb, c);
                let (x, y) = (dot(d, e0) * radius_m, dot(d, e1) * radius_m);
                let z = phi.h(nb, radius_m) - hc;
                sxx += x * x;
                sxy += x * y;
                syy += y * y;
                sxz += x * z;
                syz += y * z;
            }
            let det = sxx * syy - sxy * sxy;
            if det.abs() < 1e-300 {
                return None;
            }
            let gx = (syy * sxz - sxy * syz) / det;
            let gy = (sxx * syz - sxy * sxz) / det;
            let n = (gx * gx + gy * gy).sqrt();
            if n < 1e-14 {
                return None;
            }
            // Downslope direction = −∇h, taken as a CONTINUOUS angle (D-∞'s idea). The
            // outflow SPLIT across edges is a separate matter (conservation); the DIRECTION
            // is what this probe tests, and it is the reconstructed gradient.
            let a = (-gy).atan2(-gx);
            Some(unit(add(scale(e0, a.cos()), scale(e1, a.sin()))))
        }
        _ => unreachable!(),
    }
}

/// The **signed deflection** Δ at a cell (degrees, CCW positive as seen from outside the
/// sphere): the angle from the true steepest descent to the router's outflow direction.
/// `sin Δ` is exactly the fraction of the flow running ALONG the contour rather than down it.
pub fn deflection_at(
    r: Router,
    phi: &dyn Potential,
    level: u32,
    f: usize,
    i: i64,
    j: i64,
    radius_m: f64,
) -> Option<f64> {
    let c = cell_center(level, f, i, j);
    let fdir = phi.descent(c, radius_m)?;
    let d = direction(r, phi, level, f, i, j, radius_m)?;
    let e1 = cross(c, fdir); // +90° CCW from the flow
    Some(wrap180(dot(d, e1).atan2(dot(d, fdir)).to_degrees()))
}

// ===========================================================================
// THE CIRCULATION OPERATOR
// ===========================================================================

pub struct Circulation {
    /// `κ = ∮ E·dl / ∮ dl` — dimensionless. **Exactly 0 for the truth.** On a contour it is
    /// `⟨sin Δ⟩`: the fraction of the transport going AROUND the hill instead of DOWN it.
    pub kappa: f64,
    /// `∮ E·dl` in metres — the length by which the flow is swept tangentially per unit
    /// transport around the loop. (Multiply by a head scale for the implied head monodromy.)
    pub gamma_m: f64,
    /// Loop length (m).
    pub len_m: f64,
    /// Mean |Δ| around the loop (degrees) — the fan bias, for contrast. **If κ ≈ 0 while this
    /// is large, the deflection CANCELS around the loop: an error, but not a violated identity.**
    pub mean_abs_defl_deg: f64,
    /// Mean signed Δ (degrees).
    pub mean_defl_deg: f64,
    pub samples: usize,
}

/// Circulation of the ERROR field `E = d̂ − f̂` around a closed **contour of the cone**
/// (a circle of geodesic radius `theta` about `pole`), sampled at `m` points.
///
/// Both `d̂` and `f̂` are evaluated at the SAME point (the containing cell's centre), so for a
/// router that is exactly right `E` is the zero vector and every summand vanishes
/// **identically** — the control returns machine zero structurally, not to a tolerance.
pub fn contour_circulation(
    r: Router,
    phi: &dyn Potential,
    pole: V3,
    theta: f64,
    level: u32,
    radius_m: f64,
    m: usize,
) -> Circulation {
    let e0 = tangent(pole, if dot(pole, [0.0, 0.0, 1.0]).abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] });
    let e1 = cross(pole, e0);

    let (mut num, mut len, mut sabs, mut ssgn, mut n) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0usize);
    for k in 0..m {
        let psi = 2.0 * std::f64::consts::PI * (k as f64 + 0.5) / m as f64;
        let p = unit(add(
            scale(pole, theta.cos()),
            scale(add(scale(e0, psi.cos()), scale(e1, psi.sin())), theta.sin()),
        ));
        // CCW tangent to the contour at p, as seen from outside the sphere, oriented so that
        // `d̂ · t̂ = sin Δ` with Δ the CCW-positive deflection from the flow. The flow on this
        // cone runs AWAY from the pole (f̂ = −tangent(p, pole)), so t̂ = p̂ × f̂.
        //
        // ⚠ Caught by control C2: the first version used `p̂ × tangent(p, pole)` — the same
        // vector NEGATED — which flipped the orientation of every loop. It changed no
        // magnitude and no verdict, and it would have been invisible without a control that
        // has a SIGNED closed-form answer. That is what C2 is for.
        let that = unit(cross(p, scale(tangent(p, pole), -1.0)));
        let dl = radius_m * theta.sin() * (2.0 * std::f64::consts::PI / m as f64);

        let (f, i, j) = cell_of(p, level);
        let c = cell_center(level, f, i, j);
        let (Some(fdir), Some(d)) = (phi.descent(c, radius_m), direction(r, phi, level, f, i, j, radius_m))
        else {
            continue;
        };
        // E = d̂ − f̂, evaluated at the cell centre; project into the tangent plane at p.
        let e = sub(d, fdir);
        let e = sub(e, scale(p, dot(p, e)));
        num += dot(e, that) * dl;
        len += dl;

        let e1c = cross(c, fdir);
        let dfl = wrap180(dot(d, e1c).atan2(dot(d, fdir)).to_degrees());
        sabs += dfl.abs();
        ssgn += dfl;
        n += 1;
    }
    Circulation {
        kappa: if len > 0.0 { num / len } else { 0.0 },
        gamma_m: num,
        len_m: len,
        mean_abs_defl_deg: if n > 0 { sabs / n as f64 } else { 0.0 },
        mean_defl_deg: if n > 0 { ssgn / n as f64 } else { 0.0 },
        samples: n,
    }
}

// ---------------------------------------------------------------------------
// Real terrain has no exact contour to walk. So walk one.
// ---------------------------------------------------------------------------

/// Trace a closed **contour of φ** by RK4 integration along the level-set tangent
/// `t̂ = p̂ × f̂` — which is, by construction, perpendicular to ∇φ, so the path stays on the
/// level set. Returns the closed polyline, or `None` if it does not close in `max_steps`.
///
/// **Its own control is free and is checked by the caller:** `h` must be CONSTANT along the
/// returned path. If the traced curve has drifted off the level set, it is not a contour and
/// `∮ f̂·dl` is no longer exactly zero — so the invariant would not be an invariant. The
/// reported `h` drift is therefore a gate, not a diagnostic.
pub fn trace_contour(
    phi: &dyn Potential,
    seed: V3,
    radius_m: f64,
    step_m: f64,
    max_steps: usize,
) -> Option<Vec<V3>> {
    let tan_at = |p: V3| -> Option<V3> { phi.descent(p, radius_m).map(|f| unit(cross(p, f))) };
    let ds = step_m / radius_m; // arc in radians
    let mut pts = vec![seed];
    let mut p = seed;
    for step in 0..max_steps {
        let k1 = tan_at(p)?;
        let p2 = unit(add(p, scale(k1, ds * 0.5)));
        let k2 = tan_at(p2)?;
        let p3 = unit(add(p, scale(k2, ds * 0.5)));
        let k3 = tan_at(p3)?;
        let p4 = unit(add(p, scale(k3, ds)));
        let k4 = tan_at(p4)?;
        let d = scale(add(add(k1, scale(k2, 2.0)), add(scale(k3, 2.0), k4)), ds / 6.0);
        p = unit(add(p, d));
        pts.push(p);
        // closed? (only after enough travel that we cannot trivially match the seed)
        if step > 20 && geodesic(p, seed) * radius_m < step_m {
            pts.push(seed);
            return Some(pts);
        }
    }
    None
}

/// Circulation of `E = d̂ − f̂` around an arbitrary closed polyline (used for traced contours).
pub fn polyline_circulation(
    r: Router,
    phi: &dyn Potential,
    pts: &[V3],
    level: u32,
    radius_m: f64,
) -> Circulation {
    let (mut num, mut len, mut sabs, mut ssgn, mut n) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0usize);
    for w in pts.windows(2) {
        let (a, b) = (w[0], w[1]);
        let mid = unit(add(a, b));
        let seg = scale(sub(b, a), radius_m);
        let dl = norm(seg);
        if dl <= 0.0 {
            continue;
        }
        let (f, i, j) = cell_of(mid, level);
        let c = cell_center(level, f, i, j);
        let (Some(fdir), Some(d)) = (phi.descent(c, radius_m), direction(r, phi, level, f, i, j, radius_m))
        else {
            continue;
        };
        let e = sub(d, fdir);
        let e = sub(e, scale(mid, dot(mid, e)));
        num += dot(e, seg);
        len += dl;
        let e1c = cross(c, fdir);
        let dfl = wrap180(dot(d, e1c).atan2(dot(d, fdir)).to_degrees());
        sabs += dfl.abs();
        ssgn += dfl;
        n += 1;
    }
    Circulation {
        kappa: if len > 0.0 { num / len } else { 0.0 },
        gamma_m: num,
        len_m: len,
        mean_abs_defl_deg: if n > 0 { sabs / n as f64 } else { 0.0 },
        mean_defl_deg: if n > 0 { ssgn / n as f64 } else { 0.0 },
        samples: n,
    }
}

/// **Local discrete curl** — the plaquette circulation of `E = d̂ − f̂` around the 2×2 loop of
/// cells `(i,j) → (i+1,j) → (i+1,j+1) → (i,j+1)`, normalised to a level-independent number.
///
/// Returns `Ω·R / A` (dimensionless): the spurious rotation per radian of arc, relative to the
/// local slope. Zero for a perfect router, by construction.
pub fn plaquette_curl(
    r: Router,
    phi: &dyn Potential,
    level: u32,
    f: usize,
    i: i64,
    j: i64,
    radius_m: f64,
) -> Option<f64> {
    let quad = [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)];
    let mut e = [[0.0f64; 3]; 4];
    let mut x = [[0.0f64; 3]; 4];
    for (k, &(a, b)) in quad.iter().enumerate() {
        let c = cell_center(level, f, a, b);
        let fdir = phi.descent(c, radius_m)?;
        let d = direction(r, phi, level, f, a, b, radius_m)?;
        e[k] = sub(d, fdir);
        x[k] = scale(c, radius_m);
    }
    let mut gamma = 0.0;
    for k in 0..4 {
        let l = (k + 1) % 4;
        let seg = sub(x[l], x[k]);
        gamma += 0.5 * dot(add(e[k], e[l]), seg);
    }
    // plaquette area ≈ |diag1 × diag2| / 2
    let area = 0.5 * norm(cross(sub(x[2], x[0]), sub(x[3], x[1])));
    if area <= 0.0 {
        return None;
    }
    Some(gamma * radius_m / area)
}
