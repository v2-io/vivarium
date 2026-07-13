//! The probes. Every number in `ref/research/grid-comparison-report.md` is printed by
//! one of these, and nothing here has a tuning knob.

#![allow(dead_code)]

use super::mesh::*;
use std::collections::HashMap;

// ===========================================================================
// 1. GEOMETRY — the per-cell and per-stencil census.
// ===========================================================================

pub struct Stat {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}
impl Stat {
    pub fn of(xs: &[f64]) -> Stat {
        let (mut lo, mut hi, mut s) = (f64::MAX, f64::MIN, 0.0);
        for &x in xs {
            lo = lo.min(x);
            hi = hi.max(x);
            s += x;
        }
        Stat { min: lo, max: hi, mean: s / xs.len() as f64 }
    }
    /// max/min — the "would this hoist to a constant?" number. 1.000 = yes.
    pub fn spread(&self) -> f64 {
        if self.min.abs() < 1e-300 { f64::INFINITY } else { self.max / self.min }
    }
}

pub struct Geometry {
    pub area: Stat,
    pub edge: Stat,
    pub arm: Stat,
    pub dist: Stat,
    /// Non-orthogonality (deg): angle between the centre-line and the edge normal.
    pub nonortho: Stat,
    /// |arm_i + arm_j − dist| / dist — how far the mid-edge is OFF the centre-line.
    /// Exactly zero iff the centre-line passes through the mid-edge.
    pub arm_deficit: Stat,
    /// Interior angles of the cells (deg).
    pub angle: Stat,
    /// |Σ L_e n̂_e| / Σ L_e — the discrete closure identity. 0 = the stencil is a
    /// consistent control volume; >0 means the divergence theorem does not close on it.
    pub closure: Stat,
    pub valence: HashMap<usize, usize>,
    pub moore: HashMap<usize, usize>,
}

pub fn geometry(g: &Mesh) -> Geometry {
    let n = g.cells();
    let areas: Vec<f64> = g.areas.clone();
    let mut edges = Vec::new();
    let mut arms = Vec::new();
    let mut dists = Vec::new();
    let mut no = Vec::new();
    let mut defi = Vec::new();
    let mut clos = Vec::new();
    let mut angs = Vec::new();
    let mut val: HashMap<usize, usize> = HashMap::new();
    let mut mo: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        let nb = &g.adj[i];
        *val.entry(nb.len()).or_insert(0) += 1;
        *mo.entry(g.moore[i].len()).or_insert(0) += 1;
        let mut sum = [0.0f64; 3];
        let mut per = 0.0;
        for e in nb {
            edges.push(e.edge_len_m);
            arms.push(e.arm_m);
            dists.push(e.dist_m);
            no.push(e.nonortho_deg);
            defi.push(((e.arm_m + e.arm_opp_m - e.dist_m) / e.dist_m).abs());
            sum = add(sum, scale(e.normal, e.edge_len_m));
            per += e.edge_len_m;
        }
        clos.push(norm(sum) / per);
        // interior angles at the cell's corners
        let r = &g.rings[i];
        for k in 0..r.len() {
            let (a, b, c) = (
                g.verts[r[(k + r.len() - 1) % r.len()] as usize],
                g.verts[r[k] as usize],
                g.verts[r[(k + 1) % r.len()] as usize],
            );
            let (u, v) = (tangent(b, a), tangent(b, c));
            angs.push(dot(u, v).clamp(-1.0, 1.0).acos().to_degrees());
        }
    }

    Geometry {
        area: Stat::of(&areas),
        edge: Stat::of(&edges),
        arm: Stat::of(&arms),
        dist: Stat::of(&dists),
        nonortho: Stat::of(&no),
        arm_deficit: Stat::of(&defi),
        angle: Stat::of(&angs),
        closure: Stat::of(&clos),
        valence: val,
        moore: mo,
    }
}

// ===========================================================================
// 2. CURVATURE, split in two — Joseph's carve, kept apart.
// ===========================================================================

pub struct Curvature {
    /// **Arc/metric geometry in the embedding.** Discrete Gaussian curvature = the angle
    /// defect `2π − Σθ` at each vertex. Gauss–Bonnet says these must sum to exactly 4π,
    /// which is a free correctness check; what varies between grids is *where the
    /// curvature is concentrated*.
    pub defect_sum_over_4pi: f64,
    pub defect_max_deg: f64,
    pub defect_mean_deg: f64,
    /// **Flat-via-gravity.** The surface is a gravitational equipotential, so water
    /// locally experiences a flat plane. The question is at what span that stops being
    /// true. This is the max sagitta of a cell — how far its own corners rise off the
    /// plane through them — in metres. That is the error the "flat" shallow-water kernel
    /// commits *inside one cell*, and it is what makes the flat assumption legitimate or
    /// not. It is a length, not a shape, so it is nearly grid-independent at fixed
    /// resolution — which is itself the finding.
    pub sagitta_max_m: f64,
    pub cell_span_mean_m: f64,
}

pub fn curvature(g: &Mesh) -> Curvature {
    // angle defect at each vertex
    let mut defect = vec![2.0 * std::f64::consts::PI; g.verts.len()];
    for i in 0..g.cells() {
        let r = &g.rings[i];
        for k in 0..r.len() {
            let (a, b, c) = (
                g.verts[r[(k + r.len() - 1) % r.len()] as usize],
                g.verts[r[k] as usize],
                g.verts[r[(k + 1) % r.len()] as usize],
            );
            // the PLANAR angle of the polyhedron (chords), which is what discrete
            // Gaussian curvature is defined on
            let (u, v) = (unit(sub(a, b)), unit(sub(c, b)));
            defect[r[k] as usize] -= dot(u, v).clamp(-1.0, 1.0).acos();
        }
    }
    let sum: f64 = defect.iter().sum();
    let dmax = defect.iter().cloned().fold(f64::MIN, f64::max);
    let dmean = sum / defect.len() as f64;

    // sagitta: max distance of a cell's corners from the best plane through the centre
    let mut sag: f64 = 0.0;
    let mut span = 0.0;
    for i in 0..g.cells() {
        let c = g.centers[i];
        let mut s: f64 = 0.0;
        let mut sp: f64 = 0.0;
        for &v in &g.rings[i] {
            let p = g.verts[v as usize];
            // height of the corner above the tangent plane at the cell centre
            s = s.max((1.0 - dot(p, c)).abs() * g.radius_m);
            sp = sp.max(geodesic(c, p) * g.radius_m);
        }
        sag = sag.max(s);
        span += sp;
    }
    Curvature {
        defect_sum_over_4pi: sum / (4.0 * std::f64::consts::PI),
        defect_max_deg: dmax.to_degrees(),
        defect_mean_deg: dmean.to_degrees(),
        sagitta_max_m: sag,
        cell_span_mean_m: span / g.cells() as f64,
    }
}

/// **Strata volumes — the vertical.** A column's volume is NOT area×height: the shell's
/// volume element scales as r². `V = ∫ A(r/R)² dr` over `[R+z0, R+z1]`. Returns
/// (top/bottom area ratio, true volume ÷ area×height) for a shell. Grid-INDEPENDENT —
/// which is the point: it multiplies whatever area spread the grid already has.
pub fn strata(radius_m: f64, z0_m: f64, z1_m: f64) -> (f64, f64) {
    let (r0, r1) = (radius_m + z0_m, radius_m + z1_m);
    let area_ratio = (r1 / r0).powi(2);
    let v = (r1.powi(3) - r0.powi(3)) / 3.0 / radius_m.powi(2); // ∫(r/R)² dr
    let naive = r1 - r0;
    (area_ratio, v / naive)
}

// ===========================================================================
// 3. THE CONFIGURATION-CLASS CENSUS — the stencil, not the cell.
// ===========================================================================

pub struct Classes {
    /// Cells clustered by **topology** only: (valence, Moore count). This is the cheap
    /// branch — a kernel needs one code path per entry here, and no more.
    pub topo: HashMap<(usize, usize), usize>,
    /// Cells clustered by full **stencil congruence** at a tolerance: the cyclic sequence
    /// of (edge, my arm, its arm, centre-distance, angle) around the cell, canonicalised
    /// over rotation and reflection. This is the count of genuinely distinct local
    /// geometries — and whether it stays small or grows with resolution is the whole
    /// question of whether "class" is a branch or a lookup.
    pub geom: usize,
    pub geom_tol_pct: f64,
}

pub fn classes(g: &Mesh, tol_pct: f64) -> Classes {
    let mut topo: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..g.cells() {
        *topo.entry((g.adj[i].len(), g.moore[i].len())).or_insert(0) += 1;
    }

    // Geometric signature. Scale-free: everything is normalised by the cell's own
    // sqrt(area), so two cells in the same class are congruent, not merely similar in
    // size. Quantise at `tol_pct`, then canonicalise the cyclic word.
    let q = |x: f64| ((x / (tol_pct / 100.0)).round()) as i64;
    let mut sigs: std::collections::HashSet<Vec<i64>> = std::collections::HashSet::new();
    for i in 0..g.cells() {
        let s = g.areas[i].sqrt();
        // order the neighbours cyclically (by azimuth around the cell centre)
        let c = g.centers[i];
        let e0 = tangent(c, g.centers[g.adj[i][0].j]);
        let e1 = cross(c, e0);
        let mut nb: Vec<(f64, &Edge)> = g.adj[i]
            .iter()
            .map(|e| {
                let d = tangent(c, g.centers[e.j]);
                (dot(d, e1).atan2(dot(d, e0)), e)
            })
            .collect();
        nb.sort_by(|a, b| a.0.total_cmp(&b.0));
        let word: Vec<[i64; 5]> = nb
            .iter()
            .map(|(_, e)| {
                [
                    q(e.edge_len_m / s),
                    q(e.arm_m / s),
                    q(e.arm_opp_m / s),
                    q(e.dist_m / s),
                    q(e.nonortho_deg / 90.0),
                ]
            })
            .collect();
        // canonical form over rotation + reflection
        let k = word.len();
        let mut best: Option<Vec<[i64; 5]>> = None;
        for rev in [false, true] {
            let base: Vec<[i64; 5]> =
                if rev { word.iter().rev().cloned().collect() } else { word.clone() };
            for r in 0..k {
                let rot: Vec<[i64; 5]> = (0..k).map(|t| base[(t + r) % k]).collect();
                if best.as_ref().map_or(true, |b| rot < *b) {
                    best = Some(rot);
                }
            }
        }
        sigs.insert(best.unwrap().into_iter().flatten().collect());
    }
    Classes { topo, geom: sigs.len(), geom_tol_pct: tol_pct }
}

// ===========================================================================
// 4. THE SCHEMES. Conservation is a property of these; isotropy is not.
// ===========================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum Scheme {
    /// What `erosion.rs`/`water.rs` do today: ONE cell size, ONE area, hoisted out of
    /// the loop. Fast; and on a non-uniform grid it is solving a slightly different
    /// equation in every cell.
    NaiveUniform,
    /// Finite volume with the true geometry, transmissibility `L / dist` (centre-to-centre).
    FvCentreLine,
    /// Finite volume with the **mid-edge arm**: `L / (d_i + d_j)` where `d` is the arm
    /// PROJECTED ON THE EDGE NORMAL. This is the correct two-point flux on a
    /// non-orthogonal mesh — the difference from `FvCentreLine` IS the non-orthogonality
    /// correction, made visible.
    FvArm,
}

impl Scheme {
    pub fn label(self) -> &'static str {
        match self {
            Scheme::NaiveUniform => "naive-uniform",
            Scheme::FvCentreLine => "FV centre-line",
            Scheme::FvArm => "FV mid-edge arm",
        }
    }
}

/// Transmissibility of an edge under a scheme (m — a length ratio × length).
fn trans(s: Scheme, e: &Edge, mean_edge: f64, mean_dist: f64) -> f64 {
    match s {
        Scheme::NaiveUniform => mean_edge / mean_dist,
        Scheme::FvCentreLine => e.edge_len_m / e.dist_m,
        Scheme::FvArm => {
            let d = e.arm_normal_m + e.arm_normal_opp_m;
            if d > 0.0 { e.edge_len_m / d } else { e.edge_len_m / e.dist_m }
        }
    }
}

/// One explicit diffusion step. Mass `Σ uᵢAᵢ` is conserved EXACTLY by any scheme that
/// uses the true area and a symmetric transmissibility — that is the whole point of the
/// decomposition, and the naive path fails it only because it divides by a FICTIONAL
/// area.
pub fn diffuse_step(g: &Mesh, s: Scheme, u: &mut [f64], k: f64, mean: (f64, f64, f64)) {
    let (mean_area, mean_dist, mean_edge) = mean;
    let prev = u.to_vec();
    for i in 0..g.cells() {
        let mut f = 0.0;
        for e in &g.adj[i] {
            f += (prev[e.j] - prev[i]) * trans(s, e, mean_edge, mean_dist);
        }
        let a = if s == Scheme::NaiveUniform { mean_area } else { g.areas[i] };
        u[i] = prev[i] + k * f / a;
    }
}

/// The discrete Laplace–Beltrami operator under a scheme.
pub fn laplacian(g: &Mesh, s: Scheme, u: &[f64], mean: (f64, f64, f64)) -> Vec<f64> {
    let (mean_area, mean_dist, mean_edge) = mean;
    (0..g.cells())
        .map(|i| {
            let mut f = 0.0;
            for e in &g.adj[i] {
                f += (u[e.j] - u[i]) * trans(s, e, mean_edge, mean_dist);
            }
            let a = if s == Scheme::NaiveUniform { mean_area } else { g.areas[i] };
            f / a
        })
        .collect()
}

pub fn means(g: &Mesh) -> (f64, f64, f64) {
    let ma = g.areas.iter().sum::<f64>() / g.cells() as f64;
    let (mut sd, mut se, mut n) = (0.0, 0.0, 0usize);
    for i in 0..g.cells() {
        for e in &g.adj[i] {
            sd += e.dist_m;
            se += e.edge_len_m;
            n += 1;
        }
    }
    (ma, sd / n as f64, se / n as f64)
}

/// **The accuracy backbone.** A spherical harmonic of degree ℓ is an exact eigenfunction
/// of the Laplace–Beltrami operator: `Δ Yℓ = −ℓ(ℓ+1)/R² · Yℓ`. So the truncation error of
/// a discrete Laplacian is *exactly known*, with no reference solution, no tuning, and no
/// opinion. Returns the area-weighted relative L2 error and the L∞ error.
pub fn harmonic_error(g: &Mesh, s: Scheme, ell: usize) -> (f64, f64) {
    let e = unit([0.3, -0.7, 0.64]); // deliberately NOT aligned to any grid's axes
    let f = |p: V3| -> f64 {
        let t = dot(p, e);
        match ell {
            1 => t,
            _ => 1.5 * t * t - 0.5, // P₂(t): an ℓ=2 harmonic
        }
    };
    let lam = -(ell * (ell + 1)) as f64 / (g.radius_m * g.radius_m);
    let u: Vec<f64> = g.centers.iter().map(|&p| f(p)).collect();
    let du = laplacian(g, s, &u, means(g));
    let (mut num, mut den, mut linf, mut lmax) = (0.0, 0.0, 0.0f64, 0.0f64);
    for i in 0..g.cells() {
        let exact = lam * u[i];
        let err = du[i] - exact;
        num += err * err * g.areas[i];
        den += exact * exact * g.areas[i];
        linf = linf.max(err.abs());
        lmax = lmax.max(exact.abs());
    }
    ((num / den).sqrt(), linf / lmax)
}

/// Conservation + isotropy from a symmetric blob. Returns (mass drift, spread anisotropy).
pub fn blob(g: &Mesh, s: Scheme, src: V3, steps: usize) -> (f64, f64) {
    let m = means(g);
    let k = 0.15 * m.0 * m.1 / m.2 / 4.0;
    let mut u: Vec<f64> =
        g.centers.iter().map(|&p| (-(geodesic(p, src) * 8.0).powi(2)).exp()).collect();
    let m0 = g.mass(&u);
    for _ in 0..steps {
        diffuse_step(g, s, &mut u, k, m);
    }
    let drift = (g.mass(&u) - m0) / m0;
    (drift, anisotropy(g, &u))
}

/// Ratio of the largest to smallest principal second moment of the field about its own
/// centroid. 1.0000 = the spread stayed circular; >1 means the grid bent the physics.
pub fn anisotropy(g: &Mesh, u: &[f64]) -> f64 {
    let tot: f64 = (0..g.cells()).map(|i| u[i].max(0.0) * g.areas[i]).sum();
    let mut c = [0.0f64; 3];
    for i in 0..g.cells() {
        let w = u[i].max(0.0) * g.areas[i] / tot;
        c = add(c, scale(g.centers[i], w));
    }
    let c = unit(c);
    let t1 = {
        let a = if c[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
        unit(cross(c, a))
    };
    let t2 = cross(c, t1);
    let (mut m11, mut m22, mut m12) = (0.0, 0.0, 0.0);
    for i in 0..g.cells() {
        let w = u[i].max(0.0) * g.areas[i] / tot;
        let d = sub(g.centers[i], c);
        let (a, b) = (dot(d, t1), dot(d, t2));
        m11 += w * a * a;
        m22 += w * b * b;
        m12 += w * a * b;
    }
    // principal moments of the 2×2 second-moment tensor
    let tr = m11 + m22;
    let det = m11 * m22 - m12 * m12;
    let disc = ((tr * tr / 4.0 - det).max(0.0)).sqrt();
    let (l1, l2) = (tr / 2.0 + disc, tr / 2.0 - disc);
    (l1 / l2).sqrt()
}
