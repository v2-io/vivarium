//! Exact spherical cell areas on the equiangular cube-sphere — and the one
//! geometric fact the whole multiresolution story stands or falls on.
//!
//! ## The claim under test
//!
//! The stated objection to a Haar-style transform on our grid is: *"our cells are
//! NOT equal-area (1.4× spread), and Haar assumes equal weights."*
//!
//! **That objection names the wrong property.** A conservative multiresolution
//! transform does not need equal areas. It needs **area-additivity**:
//!
//! > $A(\text{parent}) = \sum_{k} A(\text{child}_k)$, exactly.
//!
//! Because the conserved quantity is the *integral* $\int h\,dA \approx \sum_i a_i h_i$,
//! and the coarse coefficient that preserves it is the **area-weighted** mean
//! $c_P = (\sum_k a_k h_k) / A_P$ — which telescopes up the pyramid **iff** the areas
//! telescope up the pyramid. Equal weights are *sufficient* for that; they are not
//! *necessary*.
//!
//! ## Why additivity is exact here — a theorem, not a hope
//!
//! On the equiangular cube-sphere a cell is bounded by curves of constant $u$ and
//! constant $v$. For face +Z, $\text{dir}(u,v) \propto (\tan\frac{\pi u}{4},\ \tan\frac{\pi v}{4},\ 1)$.
//! Fix $u = u_0$ and vary $v$: every such point lies in $\text{span}\{(\tan\frac{\pi u_0}{4}, 0, 1),\ (0,1,0)\}$
//! — a plane **through the origin**. So a constant-$u$ curve is a **great circle**, and
//! likewise for constant $v$.
//!
//! ⇒ **Every cube-sphere cell is a spherical quadrilateral with four geodesic edges**
//! (the 24 valence-3 corner cells included — they are quads too; the defect is at a
//! *vertex*, not in the cell).
//!
//! ⇒ A parent's four children are cut from it by the two great circles $u = u_{\text{mid}}$
//! and $v = v_{\text{mid}}$. They **exactly tile** the parent. Therefore their areas sum
//! to the parent's area **identically** — not approximately, not asymptotically.
//!
//! ## ⚠ The control I first wrote was VACUOUS, and it nearly shipped
//!
//! My first "falsification control" subdivided the parent at 0.45 of its span instead of
//! the midpoint, expecting a large residual. **It returned ~1e-13 — the same size as the
//! signal.** Of course it did: an *exact area measure is additive over ANY partition of
//! the parent.* Cutting at 0.45 still partitions it. The control could not fail, so the
//! measurement it "validated" established nothing.
//!
//! The structural facts, restated so they are checkable:
//! - **(a)** The quadtree children *partition* the parent. This is a fact about the index
//!   algebra, and it is checkable **exactly**: `corner_uv(2i, 2j, L+1) == corner_uv(i, j, L)`
//!   and `corner_uv(2i+2, 2j+2, L+1) == corner_uv(i+1, j+1, L)`, bit for bit.
//! - **(b)** Given (a), $\sum A(\text{child}) = A(\text{parent})$ for *any* exact area measure —
//!   a tautology, not a discovery.
//! - **(c)** ⇒ the area-weighted mean telescopes and $\int h\,dA$ rides to the root untouched.
//!   *That* is the load-bearing claim, and `probe_exactness` measures it directly (2.7e-15
//!   across 8 levels).
//!
//! **The control that CAN fail** is therefore a different one: the weighting the codebase
//! uses *today*. `sample::cell_size_m(level, R)` returns **one cell size per level** — a
//! uniform-area assumption. Under uniform weights the transform still telescopes (4 equal
//! children sum to a 4×-area parent), so it is *self*-consistent — **and it computes the
//! WRONG INTEGRAL**, by the full area distortion. That control screams, and it is the one
//! that indicts our current code.

use vivarium_world::sphere::{CubeCoord, Face};

/// Face-parameter (u, v) of the corner at integer grid coords `(i, j)` at `level`.
/// The cell `(i, j)` spans corners `(i,j) .. (i+1, j+1)`.
#[inline]
pub fn corner_uv(i: u64, j: u64, level: u8) -> (f64, f64) {
    let n = (1u64 << level) as f64;
    (2.0 * (i as f64) / n - 1.0, 2.0 * (j as f64) / n - 1.0)
}

#[inline]
fn unit(face: Face, u: f64, v: f64) -> [f64; 3] {
    CubeCoord { face, u, v }.to_unit()
}

#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// Solid angle of the spherical triangle `(a, b, c)` — Van Oosterom & Strackee (1983):
///
/// $\tan(E/2) = \dfrac{\mathbf{a}\cdot(\mathbf{b}\times\mathbf{c})}{1 + \mathbf{a}\cdot\mathbf{b} + \mathbf{b}\cdot\mathbf{c} + \mathbf{c}\cdot\mathbf{a}}$
///
/// Exact and numerically robust for small triangles (unlike the naive
/// spherical-excess-from-angles form, which cancels catastrophically as the
/// triangle shrinks — and ours shrink to 19 m at L19).
fn tri_solid_angle(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let num = dot(a, cross(b, c));
    let den = 1.0 + dot(a, b) + dot(b, c) + dot(c, a);
    2.0 * num.atan2(den)
}

/// Solid angle (steradians) of the cube-sphere cell `(face, i, j)` at `level`, via the
/// two-geodesic-triangle decomposition. **Numerically poor at fine levels** — the
/// cross-product of near-parallel unit vectors cancels catastrophically (measured: 4e-6
/// relative at L19, 5e-4 at L22). Kept only as an independent cross-check of the exact
/// closed form below, at COARSE levels where it is trustworthy.
pub fn cell_solid_angle_tri(face: Face, i: u64, j: u64, level: u8) -> f64 {
    let (u0, v0) = corner_uv(i, j, level);
    let (u1, v1) = corner_uv(i + 1, j + 1, level);
    let c00 = unit(face, u0, v0);
    let c10 = unit(face, u1, v0);
    let c11 = unit(face, u1, v1);
    let c01 = unit(face, u0, v1);
    (tri_solid_angle(c00, c10, c11) + tri_solid_angle(c00, c11, c01)).abs()
}

/// The **exact closed form** for the solid angle of a cube-sphere cell.
///
/// On a cube face the gnomonic tangent coordinates are $X = \tan\frac{\pi u}{4}$,
/// $Y = \tan\frac{\pi v}{4}$, and the solid-angle element is
/// $d\Omega = \dfrac{dX\,dY}{(1 + X^2 + Y^2)^{3/2}}$, which has the elementary antiderivative
///
/// $$\mathcal{F}(X, Y) \;=\; \arctan\!\left(\frac{X\,Y}{\sqrt{1 + X^2 + Y^2}}\right)$$
///
/// so a cell spanning $[X_0, X_1] \times [Y_0, Y_1]$ has
/// $\Omega = \mathcal{F}(X_1,Y_1) - \mathcal{F}(X_1,Y_0) - \mathcal{F}(X_0,Y_1) + \mathcal{F}(X_0,Y_0)$.
///
/// Exact, no quadrature, and it does not lose the digits the triangle formula loses.
pub fn cell_solid_angle(face: Face, i: u64, j: u64, level: u8) -> f64 {
    let _ = face; // the solid angle is face-independent (all six faces are congruent)
    let (u0, v0) = corner_uv(i, j, level);
    let (u1, v1) = corner_uv(i + 1, j + 1, level);
    let f = |u: f64, v: f64| -> f64 {
        let (x, y) = ((u * std::f64::consts::FRAC_PI_4).tan(), (v * std::f64::consts::FRAC_PI_4).tan());
        (x * y / (1.0 + x * x + y * y).sqrt()).atan()
    };
    (f(u1, v1) - f(u1, v0) - f(u0, v1) + f(u0, v0)).abs()
}

/// Great-circle length (m) of the cell edge from face-parameter `(ua, va)` to `(ub, vb)`.
/// Cube-sphere cell edges ARE geodesics (constant-u and constant-v curves lie in planes
/// through the origin), so this is the exact edge length, not an approximation.
pub fn arc_len_m(face: Face, ua: f64, va: f64, ub: f64, vb: f64, radius_m: f64) -> f64 {
    let a = unit(face, ua, va);
    let b = unit(face, ub, vb);
    let c = cross(a, b);
    let s = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
    s.atan2(dot(a, b)) * radius_m
}

/// Length (m) of the EAST edge of cell `(i, j)` at `level` — the face between it and
/// cell `(i+1, j)`. (The edge at $u = u_1$, running from $v_0$ to $v_1$.)
pub fn east_edge_len_m(face: Face, i: u64, j: u64, level: u8, radius_m: f64) -> f64 {
    let (u1, v0) = corner_uv(i + 1, j, level);
    let (_, v1) = corner_uv(i + 1, j + 1, level);
    arc_len_m(face, u1, v0, u1, v1, radius_m)
}

/// Length (m) of the NORTH edge of cell `(i, j)` at `level` — the face between it and
/// cell `(i, j+1)`.
pub fn north_edge_len_m(face: Face, i: u64, j: u64, level: u8, radius_m: f64) -> f64 {
    let (u0, v1) = corner_uv(i, j + 1, level);
    let (u1, _) = corner_uv(i + 1, j + 1, level);
    arc_len_m(face, u0, v1, u1, v1, radius_m)
}

/// Cell area in m² on a sphere of the given radius.
pub fn cell_area_m2(face: Face, i: u64, j: u64, level: u8, radius_m: f64) -> f64 {
    cell_solid_angle(face, i, j, level) * radius_m * radius_m
}

/// Unit vector at the CENTRE of cell `(i, j)` at `level`.
pub fn cell_center_unit(face: Face, i: u64, j: u64, level: u8) -> [f64; 3] {
    let n = (1u64 << level) as f64;
    let u = 2.0 * (i as f64 + 0.5) / n - 1.0;
    let v = 2.0 * (j as f64 + 0.5) / n - 1.0;
    unit(face, u, v)
}

/// Great-circle distance (m) between two unit vectors.
pub fn gc_dist_m(a: [f64; 3], b: [f64; 3], radius_m: f64) -> f64 {
    let c = cross(a, b);
    let s = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
    s.atan2(dot(a, b)) * radius_m
}

/// FALSIFICATION CONTROL — a subdivision that is *deliberately non-nesting*: the
/// children are cut at `frac` of the parent's span instead of at the midpoint, so
/// they do NOT tile the parent. Their areas must therefore FAIL to sum to it.
/// If this control returns ~0 residual, the additivity probe is measuring nothing.
pub fn control_child_solid_angles(face: Face, i: u64, j: u64, level: u8, frac: f64) -> [f64; 4] {
    let (u0, v0) = corner_uv(i, j, level);
    let (u1, v1) = corner_uv(i + 1, j + 1, level);
    let um = u0 + frac * (u1 - u0);
    let vm = v0 + frac * (v1 - v0);
    let quad = |ua: f64, va: f64, ub: f64, vb: f64| -> f64 {
        let a = unit(face, ua, va);
        let b = unit(face, ub, va);
        let c = unit(face, ub, vb);
        let d = unit(face, ua, vb);
        (tri_solid_angle(a, b, c) + tri_solid_angle(a, c, d)).abs()
    };
    [quad(u0, v0, um, vm), quad(um, v0, u1, vm), quad(u0, vm, um, v1), quad(um, vm, u1, v1)]
}
