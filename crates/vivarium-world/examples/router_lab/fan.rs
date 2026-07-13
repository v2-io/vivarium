//! THE MFD FAN — measured at the resolution erosion actually runs, and asked the
//! only question that decides anything: **is the error a BIAS or is it NOISE?**
//!
//! ## Why this file exists
//!
//! `erosion.rs::accumulate_drainage` is **MFD** (Quinn et al., p = 1.1): each cell splits
//! its discharge among its 8 Moore neighbours, weighted `(drop/dist)^p`. It hardcodes
//!
//! ```text
//!   dist = cell_m            for the 4 axial neighbours
//!   dist = cell_m · √2       for the 4 diagonals          (cell_m = sample::cell_size_m)
//! ```
//!
//! and — implicitly, by using eight fixed `(dx,dy)` offsets and nothing else — assumes
//! those 8 neighbours **sample the compass evenly, 45° apart**. That even-sampling premise
//! is MFD's entire reason for existing: it is the fix for D8's grid-aligned-channel
//! artifact. If the premise is false on our grid, MFD's cure has the disease.
//!
//! `grid_lab`'s §9 printed the fan for TWO cells at ONE level (L5, ~313 km cells). Erosion
//! runs at **L19** (~19 m cells) — fourteen levels finer. Two cells is not a survey and L5
//! is not L19, so that number could not answer it.
//!
//! ## The move that makes L19 (and L23) cheap
//!
//! A cell's fan needs only the cell and its 8 Moore neighbours. It does **not** need a
//! mesh. So the fan is computable **analytically at any level** from `CellId::to_cube()` +
//! `CubeCoord::to_unit()` alone — O(1) per sample, level-free. `gate_fan_matches_mesh()`
//! pins the analytic path against the full mesh's *combinatorial* adjacency at L5/L6, so
//! this is a shortcut, not a different quantity.
//!
//! ## And the move that makes convergence-or-not a THEOREM, not a trend
//!
//! As `N = 2^L → ∞` the neighbourhood of a cell at `(u,v)` shrinks onto the map's
//! **Jacobian**: neighbour `(dx,dy)` sits at `c + (2/N)·(dx·∂p/∂u + dy·∂p/∂v) + O(1/N²)`.
//! So the fan converges to the fan of the **lattice sheared by `J(u,v)`** — and `J` is a
//! property of the *projection*, with no `N` in it. The limit is therefore computable in
//! CLOSED FORM ([`fan_limit`]), and the finite-level fans must converge to it rather than
//! to 45°. That is the whole convergence question, answered exactly, with the measured
//! levels as the check rather than the evidence.
//!
//! For the equiangular cube (`x = tan(u·π/4)`) the closed form has two named points:
//!
//! ```text
//!   face CENTRE  (u=v=0):  ∂u ⟂ ∂v, |∂u| = |∂v|      → a perfect 45° fan. MFD is exact.
//!   face EDGE-MID (u=1,v=0): ∂u ⟂ ∂v, |∂u|/|∂v| = √2 → orthogonal but STRETCHED √2:1.
//!   face CORNER  (u=v=1):  ∂u ∠ ∂v = 120°, |∂u|=|∂v| → a RHOMBIC lattice, gaps 60/30.
//! ```
//!
//! ## The bias question
//!
//! [`deflection`] is the decider at the cell level. Impose a field whose steepest descent
//! is EXACTLY azimuth φ (a plane in the tangent plane — the true answer is known and it is
//! φ). Run the **status-quo MFD weights** on the **true** neighbour geometry. Ask where the
//! mass actually went: `Δ(φ) = arg(Σ wₖ d̂ₖ) − φ`. Zero would mean MFD reproduces the true
//! flow direction.
//!
//! The control that makes it a probe that CAN fail: run the identical code on a **perfect
//! square lattice** (bearings exactly k·45°, distances exactly cell/cell·√2). `Δ` there is
//! *MFD's own intrinsic* directional error — nonzero, and it is the baseline. Anything our
//! grid shows above the control is the sphere's doing, and only that part is chargeable
//! here.

#![allow(dead_code)]

use super::grids::{cube_face_basis, cube_to_unit, CubeProj};
use super::mesh::*;
use std::f64::consts::{FRAC_PI_4, PI, SQRT_2};

/// Quinn's exponent, as `erosion.rs::accumulate_drainage` hardcodes it.
pub const P: f64 = 1.1;

/// The 8 Moore offsets `(dx, dy)` in CCW order from `+u`. `x` is the `i` (u) index and
/// `y` the `j` (v) index — the same sense as `erosion.rs`'s `NEIGHBORS` (order there is
/// irrelevant: the weights are summed).
pub const OFF: [(i64, i64); 8] =
    [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

/// What MFD *assumes* the bearing of offset `k` is: exactly `k · 45°`.
pub fn assumed_bearing(k: usize) -> f64 { k as f64 * 45.0 }

/// What MFD *assumes* the distance of offset `k` is: `cell_m`, or `cell_m·√2` diagonally.
pub fn assumed_dist(k: usize, cell_m: f64) -> f64 {
    let (dx, dy) = OFF[k];
    if dx != 0 && dy != 0 { cell_m * SQRT_2 } else { cell_m }
}

/// `sample::cell_size_m` — the ONE number `erosion.rs` uses for the whole patch.
pub fn cell_m(level: u32, radius_m: f64) -> f64 {
    (PI / 2.0) * radius_m / (1u64 << level) as f64
}

/// Cell centre of face cell `(i, j)` at `level` — literally `CellId::to_cube().to_unit()`.
pub fn cell_center(level: u32, f: usize, i: i64, j: i64) -> V3 {
    let n = (1i64 << level) as f64;
    let c = |k: i64| ((k as f64 + 0.5) / n) * 2.0 - 1.0;
    cube_to_unit(CubeProj::Equiangular, f, c(i), c(j))
}

/// The map's tangent vectors `(∂p/∂u, ∂p/∂v)` at `(u,v)` on face `f` — **exact** (chain
/// rule through `tan` and the normalization), not a finite difference. This is the object
/// the fan converges to, and it contains no `N`.
pub fn jacobian(f: usize, u: f64, v: f64) -> (V3, V3) {
    let (c, a, b) = cube_face_basis(f);
    let (tu, tv) = ((u * FRAC_PI_4).tan(), (v * FRAC_PI_4).tan());
    let d = add(c, add(scale(a, tu), scale(b, tv)));
    let nd = norm(d);
    let p = scale(d, 1.0 / nd);
    // d/dx tan(x·π/4) = (π/4)·sec²(x·π/4)
    let dtan = |x: f64| { let cc = (x * FRAC_PI_4).cos(); FRAC_PI_4 / (cc * cc) };
    // ∂p/∂x = (I − p pᵀ)·(∂d/∂x) / |d|
    let proj = |w: V3| scale(sub(w, scale(p, dot(p, w))), 1.0 / nd);
    (proj(scale(a, dtan(u))), proj(scale(b, dtan(v))))
}

/// One cell's Moore fan, with MFD's assumptions sitting next to the truth.
#[derive(Clone, Debug)]
pub struct Fan {
    pub level: u32,
    pub uv: (f64, f64),
    /// True bearing of offset `k`, degrees CCW, in the tangent frame whose 0° is the true
    /// bearing of offset `(1,0)`. (A gauge choice: the frame cancels out of every error
    /// below, and it makes "relative to the grid axis" the reading.)
    pub bearing: [f64; 8],
    /// True great-circle centre-to-centre distance to offset `k` (m).
    pub dist_m: [f64; 8],
    /// Sorted angular gaps between successive neighbours. An ideal fan is eight 45°.
    pub gaps: [f64; 8],
    pub cell_m: f64,
}

impl Fan {
    /// `max |gap − 45°|` — the headline fan non-uniformity.
    pub fn gap_dev(&self) -> f64 {
        self.gaps.iter().fold(0.0f64, |m, &g| m.max((g - 45.0).abs()))
    }
    /// `max |bearing_k − k·45°|` — how far each direction is from where MFD thinks it is.
    pub fn bearing_err(&self) -> f64 {
        (0..8).fold(0.0f64, |m, k| {
            let d = wrap180(self.bearing[k] - assumed_bearing(k));
            m.max(d.abs())
        })
    }
    /// `max |true_dist / assumed_dist − 1|` — how wrong `cell_m` / `cell_m·√2` are.
    pub fn dist_err(&self) -> f64 {
        (0..8).fold(0.0f64, |m, k| {
            m.max((self.dist_m[k] / assumed_dist(k, self.cell_m) - 1.0).abs())
        })
    }
    pub fn dist_ratio(&self, k: usize) -> f64 { self.dist_m[k] / assumed_dist(k, self.cell_m) }
}

/// Wrap to `(−180, 180]`.
pub fn wrap180(a: f64) -> f64 {
    let mut x = a % 360.0;
    if x > 180.0 { x -= 360.0 }
    if x <= -180.0 { x += 360.0 }
    x
}

fn assemble(level: u32, uv: (f64, f64), c: V3, nb: [V3; 8], radius_m: f64) -> Fan {
    let e0 = tangent(c, nb[0]); // 0° := the true bearing of offset (1,0)
    let e1 = cross(c, e0); // right-handed with the outward normal; +v lies near +90° on every face
    let mut bearing = [0.0f64; 8];
    let mut dist_m = [0.0f64; 8];
    for k in 0..8 {
        let t = tangent(c, nb[k]);
        bearing[k] = dot(t, e1).atan2(dot(t, e0)).to_degrees();
        dist_m[k] = geodesic(c, nb[k]) * radius_m;
    }
    let mut srt = bearing;
    srt.sort_by(f64::total_cmp);
    let mut gaps = [0.0f64; 8];
    for k in 0..8 {
        let d = srt[(k + 1) % 8] - srt[k];
        gaps[k] = if d < 0.0 { d + 360.0 } else { d };
    }
    Fan { level, uv, bearing, dist_m, gaps, cell_m: cell_m(level, radius_m) }
}

/// The **measured** fan at face cell `(i,j)`, level `L`. Requires `1 ≤ i,j ≤ 2^L − 2`
/// (a face-interior cell, so all 8 Moore neighbours are on the same face — which is
/// every cell but the outermost ring).
pub fn fan(level: u32, f: usize, i: i64, j: i64, radius_m: f64) -> Fan {
    let n = 1i64 << level;
    assert!(i >= 1 && j >= 1 && i <= n - 2 && j <= n - 2, "face-interior cells only");
    let c = cell_center(level, f, i, j);
    let mut nb = [[0.0; 3]; 8];
    for (k, &(dx, dy)) in OFF.iter().enumerate() {
        nb[k] = cell_center(level, f, i + dx, j + dy);
    }
    let cu = |k: i64| ((k as f64 + 0.5) / n as f64) * 2.0 - 1.0;
    assemble(level, (cu(i), cu(j)), c, nb, radius_m)
}

/// The **limit** fan at `(u,v)` as `L → ∞`, in closed form from the Jacobian. `level` only
/// sets the length scale (it cancels out of every angle, and out of every distance RATIO).
///
/// This is the claim under test: the fan converges to THIS, not to 45°.
pub fn fan_limit(level: u32, f: usize, u: f64, v: f64, radius_m: f64) -> Fan {
    let (ju, jv) = jacobian(f, u, v);
    let h = 2.0 / (1u64 << level) as f64; // parameter step between adjacent cell centres
    let c = cube_to_unit(CubeProj::Equiangular, f, u, v);
    let mut bearing = [0.0f64; 8];
    let mut dist_m = [0.0f64; 8];
    let e0 = unit(ju); // ∂p/∂u is already tangent at c — it IS offset (1,0)'s limiting direction
    let e1 = cross(c, e0);
    for (k, &(dx, dy)) in OFF.iter().enumerate() {
        let d = add(scale(ju, dx as f64), scale(jv, dy as f64)); // the sheared-lattice arm
        bearing[k] = dot(d, e1).atan2(dot(d, e0)).to_degrees();
        dist_m[k] = norm(d) * h * radius_m; // arc ≈ chord to O(h²)
    }
    let mut srt = bearing;
    srt.sort_by(f64::total_cmp);
    let mut gaps = [0.0f64; 8];
    for k in 0..8 {
        let dd = srt[(k + 1) % 8] - srt[k];
        gaps[k] = if dd < 0.0 { dd + 360.0 } else { dd };
    }
    Fan { level, uv: (u, v), bearing, dist_m, gaps, cell_m: cell_m(level, radius_m) }
}

/// A **perfect square lattice** fan — the control. Bearings exactly `k·45°`, distances
/// exactly `cell_m` / `cell_m·√2`. MFD's premise is TRUE here by construction, so whatever
/// [`deflection`] reports on this fan is MFD's OWN intrinsic error and nothing to do with
/// the sphere. Without this the probe cannot fail honestly: every number it produced would
/// be chargeable to the grid, including the part that isn't.
pub fn fan_ideal(level: u32, radius_m: f64) -> Fan {
    let cm = cell_m(level, radius_m);
    let mut bearing = [0.0f64; 8];
    let mut dist_m = [0.0f64; 8];
    for k in 0..8 {
        bearing[k] = assumed_bearing(k);
        dist_m[k] = assumed_dist(k, cm);
    }
    Fan { level, uv: (0.0, 0.0), bearing, dist_m, gaps: [45.0; 8], cell_m: cm }
}

/// A deliberately **sheared** control (parameter axes at `shear_deg`, aspect `aspect`) —
/// the second half of "can this probe fail?". Feeding it a 120°/1.0 rhombus must reproduce
/// the closed-form corner fan; feeding it 90°/1.0 must reproduce [`fan_ideal`].
pub fn fan_sheared(level: u32, radius_m: f64, shear_deg: f64, aspect: f64) -> Fan {
    let cm = cell_m(level, radius_m);
    let (au, av) = (cm, cm * aspect);
    let mut bearing = [0.0f64; 8];
    let mut dist_m = [0.0f64; 8];
    let (sx, sy) = (shear_deg.to_radians().cos(), shear_deg.to_radians().sin());
    for (k, &(dx, dy)) in OFF.iter().enumerate() {
        let (x, y) = (dx as f64 * au + dy as f64 * av * sx, dy as f64 * av * sy);
        bearing[k] = y.atan2(x).to_degrees();
        dist_m[k] = (x * x + y * y).sqrt();
    }
    let mut srt = bearing;
    srt.sort_by(f64::total_cmp);
    let mut gaps = [0.0f64; 8];
    for k in 0..8 {
        let d = srt[(k + 1) % 8] - srt[k];
        gaps[k] = if d < 0.0 { d + 360.0 } else { d };
    }
    Fan { level, uv: (0.0, 0.0), bearing, dist_m, gaps, cell_m: cm }
}

// ===========================================================================
// THE BIAS PROBE — one cell, exact answer, MFD's own weights.
// ===========================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum DistRule {
    /// `erosion.rs` today: `cell_m` / `cell_m·√2`.
    Hardcoded,
    /// The one-line fix: use the TRUE centre-to-centre distances.
    True,
}

/// Impose a field whose steepest descent is EXACTLY azimuth `phi_deg` in this cell's
/// tangent plane, run MFD's weights over the TRUE neighbour geometry, and return
/// `Δ = arg(Σ wₖ d̂ₖ) − φ` in degrees — the angle by which MFD *actually* sends the water
/// away from where the water *actually* wants to go.
///
/// Exact by construction: on a plane `h = −(x cos φ + y sin φ)` the drop to a neighbour at
/// `(D, β)` is exactly `D·cos(β − φ)`, so no discretization of `h` enters and the only
/// error measured is the router's.
pub fn deflection(f: &Fan, phi_deg: f64, rule: DistRule) -> f64 {
    let phi = phi_deg.to_radians();
    let (mut rx, mut ry) = (0.0f64, 0.0f64);
    for k in 0..8 {
        let b = f.bearing[k].to_radians();
        let drop = f.dist_m[k] * (b - phi).cos(); // exact, at the TRUE neighbour position
        if drop <= 0.0 {
            continue;
        }
        let d = match rule {
            DistRule::Hardcoded => assumed_dist(k, f.cell_m),
            DistRule::True => f.dist_m[k],
        };
        let w = (drop / d).powf(P);
        rx += w * b.cos();
        ry += w * b.sin();
    }
    if rx == 0.0 && ry == 0.0 {
        return 0.0;
    }
    wrap180(ry.atan2(rx).to_degrees() - phi_deg)
}

/// The deflection curve `Δ(φ)` swept over the whole compass, plus what it MEANS.
pub struct Bias {
    /// RMS of `Δ(φ)` over φ (degrees) — the magnitude.
    pub rms: f64,
    pub max: f64,
    /// Mean of `Δ(φ)` over φ — net rotation. ~0 whenever the fan has a reflection symmetry,
    /// and it being ~0 is exactly why "mean error" is the WRONG summary here.
    pub mean: f64,
    /// **Stable fixed points** of `φ ↦ φ + Δ(φ)`: the azimuths flow is pulled TOWARD.
    /// These are the manufactured law — a river near one of these gets steered onto it.
    pub attractors: Vec<f64>,
    /// The fraction of the compass whose flow is deflected TOWARD the nearest attractor by
    /// more than 1° — how much of the direction space the artifact actually captures.
    pub captured: f64,
}

pub fn bias(f: &Fan, rule: DistRule) -> Bias {
    let n = 3600usize; // 0.1° resolution
    let d: Vec<f64> = (0..n)
        .map(|t| deflection(f, t as f64 * 360.0 / n as f64, rule))
        .collect();
    let rms = (d.iter().map(|x| x * x).sum::<f64>() / n as f64).sqrt();
    let max = d.iter().fold(0.0f64, |m, x| m.max(x.abs()));
    let mean = d.iter().sum::<f64>() / n as f64;

    // Stable fixed points: Δ crosses zero downward (Δ>0 below, Δ<0 above ⇒ both sides
    // pushed IN). Those are the directions the router manufactures.
    let mut attractors = Vec::new();
    for t in 0..n {
        let (a, b) = (d[t], d[(t + 1) % n]);
        if a > 0.0 && b <= 0.0 {
            let phi = (t as f64 + a / (a - b)) * 360.0 / n as f64;
            attractors.push(phi % 360.0);
        }
    }
    let captured = d.iter().filter(|x| x.abs() > 1.0).count() as f64 / n as f64;
    Bias { rms, max, mean, attractors, captured }
}
