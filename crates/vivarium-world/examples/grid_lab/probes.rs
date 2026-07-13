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
    /// The area of the GEODESIC polygon through the corners — what an FV code builds.
    pub area_geo: Stat,
    /// max |geodesic area / exact area − 1|. Non-zero only where the cell boundaries are
    /// not great circles (Snyder, HEALPix) — the hidden tax on the equal-area grids.
    pub area_geo_gap: f64,
    pub edge: Stat,
    pub arm: Stat,
    pub dist: Stat,
    /// TRUE non-orthogonality (deg), at the crossing. 0 on any Voronoi mesh.
    pub nonortho: Stat,
    /// Skewness: |mid-edge → crossing| / edge length. Independent of orthogonality.
    pub skew: Stat,
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
    let mut sk = Vec::new();
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
            sk.push(e.skew);
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

    let gap = (0..n)
        .map(|i| (g.areas_geodesic[i] / g.areas[i] - 1.0).abs())
        .fold(0.0f64, f64::max);
    Geometry {
        area: Stat::of(&areas),
        area_geo: Stat::of(&g.areas_geodesic),
        area_geo_gap: gap,
        edge: Stat::of(&edges),
        arm: Stat::of(&arms),
        dist: Stat::of(&dists),
        nonortho: Stat::of(&no),
        skew: Stat::of(&sk),
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
    /// Mean angle defect at the vertices that ARE the topological defects (the valence-3
    /// cube corners / HEALPix junctions / icosa pentagon centres). Compare with the mean.
    pub defect_at_topo_defects_deg: f64,
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
    // the angle defect AT the topologically-defective vertices
    let mut td: Vec<f64> = Vec::new();
    for v in 0..g.verts.len() {
        let val = g.vcells[v].len();
        let regular = if g.rings[0].len() == 3 { 6 } else { 4 };
        if val != regular {
            td.push(defect[v]);
        }
    }
    let td_mean = if td.is_empty() { f64::NAN } else { td.iter().sum::<f64>() / td.len() as f64 };

    Curvature {
        defect_at_topo_defects_deg: td_mean.to_degrees(),
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
        let word: Vec<[i64; 6]> = nb
            .iter()
            .map(|(_, e)| {
                [
                    q(e.edge_len_m / s),
                    q(e.arm_m / s),
                    q(e.arm_opp_m / s),
                    q(e.dist_m / s),
                    q(e.nonortho_deg / 90.0),
                    q(e.skew),
                ]
            })
            .collect();
        // canonical form over rotation + reflection
        let k = word.len();
        let mut best: Option<Vec<[i64; 6]>> = None;
        for rev in [false, true] {
            let base: Vec<[i64; 6]> =
                if rev { word.iter().rev().cloned().collect() } else { word.clone() };
            for r in 0..k {
                let rot: Vec<[i64; 6]> = (0..k).map(|t| base[(t + r) % k]).collect();
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
    /// PROJECTED ON THE EDGE NORMAL — the correct two-point *transmissibility* on a
    /// non-orthogonal mesh. Still a TWO-POINT scheme, so it still cannot see the
    /// tangential gradient.
    FvArm,
    /// **Finite volume with the non-orthogonality correction** — an LSQ gradient
    /// reconstruction supplying the cross-diffusion term that two-point schemes discard:
    ///
    /// ```text
    ///   ∂u/∂n̂  =  (n̂·ĉ)·(∇u·ĉ)          +   ∇u·(n̂ − (n̂·ĉ)ĉ)
    ///          ≈  (n̂·ĉ)·(u_j − u_i)/d    +   ḡ·(n̂ − (n̂·ĉ)ĉ)
    ///             └── the two-point part ──┘   └─ what TPFA THROWS AWAY ─┘
    /// ```
    ///
    /// `ĉ` is the centre-line direction at the edge, `ḡ = ½(g_i + g_j)` the LSQ gradient.
    /// The correction is ANTISYMMETRIC under `i ↔ j`, so conservation stays exact. This is
    /// Putman & Lin's `sin α` metric factor in mesh form, and it is the thing that makes a
    /// non-orthogonal grid usable at all.
    ///
    /// **The gradient is a QUADRATIC fit over the MOORE neighbourhood.** That detail is not
    /// cosmetic and it took measuring to find: with a linear fit over the 4 edge-neighbours
    /// ([`Scheme::FvLsqNarrow`]) the scheme converges, but only at ~0.5 order. With the wide
    /// quadratic stencil it reaches ~1.6 and the error drops another 30×.
    FvLsq,
    /// The same correction, but with the NARROW gradient (linear fit, edge-neighbours only).
    /// Kept because the gap between this and [`Scheme::FvLsq`] is one of the report's
    /// results: it is the difference between "converges" and "converges usefully".
    FvLsqNarrow,
}

impl Scheme {
    pub fn label(self) -> &'static str {
        match self {
            Scheme::NaiveUniform => "naive-uniform",
            Scheme::FvCentreLine => "FV centre-line",
            Scheme::FvArm => "FV mid-edge arm",
            Scheme::FvLsq => "FV + corrected flux, WIDE quadratic gradient",
            Scheme::FvLsqNarrow => "FV + corrected flux, narrow linear gradient",
        }
    }
}

/// Transmissibility of an edge under a scheme (m — a length ratio × length).
fn trans(s: Scheme, e: &Edge, mean_edge: f64, mean_dist: f64) -> f64 {
    match s {
        Scheme::NaiveUniform => mean_edge / mean_dist,
        Scheme::FvCentreLine => e.edge_len_m / e.dist_m,
        Scheme::FvArm | Scheme::FvLsq | Scheme::FvLsqNarrow => {
            let d = e.arm_normal_m + e.arm_normal_opp_m;
            if d > 0.0 { e.edge_len_m / d } else { e.edge_len_m / e.dist_m }
        }
    }
}

/// **Quadratic** least-squares gradient over the MOORE neighbourhood.
///
/// Why this exists: a linear LSQ fit over the 4 edge-neighbours of a quad cell is only
/// FIRST-order accurate on a distorted grid (the O(h) term cancels only on a symmetric
/// stencil), and a first-order gradient is not good enough to feed a non-orthogonality
/// correction — it caps the whole scheme below second order. A hexagon's 6 near-symmetric
/// neighbours happen to give a second-order gradient for free, which is a large part of why
/// the hex meshes converge and the quad grids do not.
///
/// The fix is not a different GRID, it is a wider STENCIL: fit
/// `Δu = gₓx + g_y y + ½hₓₓx² + hₓ_y xy + ½h_y_y y²` (5 unknowns) over the 8 Moore
/// neighbours. At a valence-3 corner there are 7 of them — still ≥ 5, so the fit is still
/// determined. **That is Addendum A1's "LSQ recovers second order at any valence", made
/// concrete: it does, but only with the wide stencil.**
pub fn lsq_gradients_quadratic(g: &Mesh, u: &[f64]) -> Vec<V3> {
    (0..g.cells())
        .map(|i| {
            let c = g.centers[i];
            let e0 = tangent(c, g.centers[g.adj[i][0].j]);
            let e1 = cross(c, e0);
            let nb = &g.moore[i];
            if nb.len() < 5 {
                return [0.0; 3];
            }
            // normal equations for the 5-parameter quadratic
            let mut ata = [[0.0f64; 5]; 5];
            let mut atb = [0.0f64; 5];
            for &j in nb {
                let d = sub(g.centers[j], c);
                let (x, y) = (dot(d, e0) * g.radius_m, dot(d, e1) * g.radius_m);
                let row = [x, y, 0.5 * x * x, x * y, 0.5 * y * y];
                // inverse-distance weighting — standard, and it keeps the near neighbours
                // (which carry the gradient) from being swamped by the far ones
                let w = 1.0 / (x * x + y * y);
                let b = u[j] - u[i];
                for a in 0..5 {
                    for bb in 0..5 {
                        ata[a][bb] += w * row[a] * row[bb];
                    }
                    atb[a] += w * row[a] * b;
                }
            }
            // Gaussian elimination with partial pivoting
            let mut m = [[0.0f64; 6]; 5];
            for a in 0..5 {
                m[a][..5].copy_from_slice(&ata[a]);
                m[a][5] = atb[a];
            }
            for col in 0..5 {
                let mut piv = col;
                for rr in col + 1..5 {
                    if m[rr][col].abs() > m[piv][col].abs() {
                        piv = rr;
                    }
                }
                if m[piv][col].abs() < 1e-300 {
                    return [0.0; 3];
                }
                m.swap(col, piv);
                for rr in 0..5 {
                    if rr == col {
                        continue;
                    }
                    let f = m[rr][col] / m[col][col];
                    for cc in col..6 {
                        m[rr][cc] -= f * m[col][cc];
                    }
                }
            }
            let gx = m[0][5] / m[0][0];
            let gy = m[1][5] / m[1][1];
            add(scale(e0, gx), scale(e1, gy))
        })
        .collect()
}

/// Least-squares gradient of `u` at each cell, as a 3-D tangent vector ([u]/m).
pub fn lsq_gradients(g: &Mesh, u: &[f64]) -> Vec<V3> {
    (0..g.cells())
        .map(|i| {
            let c = g.centers[i];
            let e0 = tangent(c, g.centers[g.adj[i][0].j]);
            let e1 = cross(c, e0);
            let (mut sxx, mut sxy, mut syy, mut sxz, mut syz) = (0.0, 0.0, 0.0, 0.0, 0.0);
            for e in &g.adj[i] {
                let d = sub(g.centers[e.j], c);
                let (x, y) = (dot(d, e0) * g.radius_m, dot(d, e1) * g.radius_m);
                let z = u[e.j] - u[i];
                sxx += x * x;
                sxy += x * y;
                syy += y * y;
                sxz += x * z;
                syz += y * z;
            }
            let det = sxx * syy - sxy * sxy;
            if det.abs() < 1e-300 {
                return [0.0; 3];
            }
            let gx = (syy * sxz - sxy * syz) / det;
            let gy = (sxx * syz - sxy * sxz) / det;
            add(scale(e0, gx), scale(e1, gy))
        })
        .collect()
}

/// The flux across one edge under a scheme. **Antisymmetric under `i ↔ j` for EVERY scheme
/// here**, which is why they all conserve exactly — conservation is structural, not earned.
fn edge_flux(g: &Mesh, s: Scheme, u: &[f64], grad: &[V3], i: usize, e: &Edge, m: (f64, f64, f64)) -> f64 {
    if s != Scheme::FvLsq && s != Scheme::FvLsqNarrow {
        return (u[e.j] - u[i]) * trans(s, e, m.2, m.1);
    }
    // FULLY CORRECTED FACE GRADIENT — both non-orthogonality AND skew.
    //
    // Work in the tangent plane at the mid-edge `f`. Let `vᵢ`, `vⱼ` be the tangent vectors
    // from `f` to the two cell centres (their lengths are exactly the mid-edge ARMS, which
    // is where the arm earns its keep). Project the cell centres onto the NORMAL LINE
    // through `f`, giving auxiliary points `pᵢ′ = f + aᵢn̂`, `pⱼ′ = f + aⱼn̂`, and use the
    // LSQ gradients to extrapolate `u` to them:
    //
    //     u(pᵢ′) ≈ uᵢ + gᵢ·(aᵢn̂ − vᵢ)        ∂u/∂n̂ ≈ [u(pⱼ′) − u(pᵢ′)] / (aⱼ − aᵢ)
    //
    // The two auxiliary points lie ON the normal, so the difference between them IS a normal
    // derivative — no non-orthogonality error — and they are aligned with the mid-edge, so
    // there is no skew error either. `aⱼ − aᵢ` is exactly `arm_normal + arm_normal_opp`.
    //
    // ⚠ Correcting only ONE of the two (the plain cross-diffusion form) gets you from a
    // DIVERGENT scheme to a merely first-order one. Both corrections are needed for second
    // order, and finding that out took measuring it: see §7's convergence table.
    let f = unit(add(g.verts[e.va as usize], g.verts[e.vb as usize]));
    let nh = tangent(f, e.normal);
    let vi = scale(tangent(f, g.centers[i]), e.arm_m);
    let vj = scale(tangent(f, g.centers[e.j]), e.arm_opp_m);
    let (ai, aj) = (dot(vi, nh), dot(vj, nh));
    let d = aj - ai;
    if d <= 1e-9 {
        return (u[e.j] - u[i]) * trans(Scheme::FvCentreLine, e, m.2, m.1);
    }
    let proj = |v: V3| sub(v, scale(f, dot(f, v)));
    let (gi, gj) = (proj(grad[i]), proj(grad[e.j]));
    let ui = u[i] + dot(gi, sub(scale(nh, ai), vi));
    let uj = u[e.j] + dot(gj, sub(scale(nh, aj), vj));
    (uj - ui) / d * e.edge_len_m
}

/// One explicit diffusion step. Mass `Σ uᵢAᵢ` is conserved EXACTLY by any scheme that
/// uses the true area and a symmetric transmissibility — that is the whole point of the
/// decomposition, and the naive path fails it only because it divides by a FICTIONAL
/// area.
pub fn diffuse_step(g: &Mesh, s: Scheme, u: &mut [f64], k: f64, mean: (f64, f64, f64)) {
    let prev = u.to_vec();
    let grad = match s {
        Scheme::FvLsq => lsq_gradients_quadratic(g, &prev),
        Scheme::FvLsqNarrow => lsq_gradients(g, &prev),
        _ => Vec::new(),
    };
    for i in 0..g.cells() {
        let mut f = 0.0;
        for e in &g.adj[i] {
            f += edge_flux(g, s, &prev, &grad, i, e, mean);
        }
        let a = if s == Scheme::NaiveUniform { mean.0 } else { g.areas[i] };
        u[i] = prev[i] + k * f / a;
    }
}

/// The discrete Laplace–Beltrami operator under a scheme.
pub fn laplacian(g: &Mesh, s: Scheme, u: &[f64], mean: (f64, f64, f64)) -> Vec<f64> {
    let grad = match s {
        Scheme::FvLsq => lsq_gradients_quadratic(g, u),
        Scheme::FvLsqNarrow => lsq_gradients(g, u),
        _ => Vec::new(),
    };
    (0..g.cells())
        .map(|i| {
            let mut f = 0.0;
            for e in &g.adj[i] {
                f += edge_flux(g, s, u, &grad, i, e, mean);
            }
            let a = if s == Scheme::NaiveUniform { mean.0 } else { g.areas[i] };
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
/// Harmonic error split by DISTANCE FROM THE TOPOLOGICAL DEFECTS. Returns
/// (L2 over cells within `ring` hops of a defect, L2 over everything else, n_near).
/// This is the test that decides Addendum A1: is the defect a bounded LOCAL wart, or does
/// it poison the whole grid?
pub fn harmonic_error_split(g: &Mesh, s: Scheme, ell: usize, ring: usize) -> (f64, f64, usize) {
    let e = unit([0.3, -0.7, 0.64]);
    let f = |p: V3| -> f64 {
        let t = dot(p, e);
        if ell == 1 { t } else { 1.5 * t * t - 0.5 }
    };
    let lam = -((ell * (ell + 1)) as f64) / (g.radius_m * g.radius_m);
    let u: Vec<f64> = g.centers.iter().map(|&p| f(p)).collect();
    let du = laplacian(g, s, &u, means(g));

    // BFS out from the defect cells
    let mut near: Vec<bool> = (0..g.cells())
        .map(|i| g.moore[i].len() != 2 * g.adj[i].len())
        .collect();
    for _ in 0..ring {
        let cur = near.clone();
        for i in 0..g.cells() {
            if cur[i] {
                for &j in &g.moore[i] {
                    near[j] = true;
                }
            }
        }
    }
    let (mut nn, mut dn, mut nf, mut df) = (0.0, 0.0, 0.0, 0.0);
    for i in 0..g.cells() {
        let exact = lam * u[i];
        let err = (du[i] - exact).powi(2) * g.areas[i];
        let sig = exact * exact * g.areas[i];
        if near[i] {
            nn += err;
            dn += sig;
        } else {
            nf += err;
            df += sig;
        }
    }
    ((nn / dn).sqrt(), (nf / df).sqrt(), near.iter().filter(|&&b| b).count())
}

pub fn harmonic_error(g: &Mesh, s: Scheme, ell: usize) -> (f64, f64) {
    let e = unit([0.3, -0.7, 0.64]); // deliberately NOT aligned to any grid's axes
    let f = |p: V3| -> f64 {
        let t = dot(p, e);
        match ell {
            1 => t,
            _ => 1.5 * t * t - 0.5, // P₂(t): an ℓ=2 harmonic
        }
    };
    let lam = -((ell * (ell + 1)) as f64) / (g.radius_m * g.radius_m);
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
