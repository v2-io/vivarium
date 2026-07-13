//! # THE MOMENT LADDER — what a splitting router is *actually* solving
//!
//! A router that splits mass among neighbours is a **random walk**. Mass at cell `i` goes to
//! neighbour `k` with probability `wₖ`, displacing by `dₖ`. The continuum limit of a random
//! walk is **advection–diffusion**, and its coefficients are the **moments of the weight set**:
//!
//! ```text
//!   m0 = Σ wₖ                       →  MASS.        (every normalised router has it ⇒ all conserve)
//!   m1 = Σ wₖ sin(βₖ − ψ)           →  DRIFT.       a zeroth-order BIAS in the transport direction
//!   m2 = Σ wₖ sin²(βₖ − ψ)          →  DISPERSION.  an artificial TRANSVERSE DIFFUSION of `a`
//! ```
//!
//! The law being discretised — `div(a·v̂) = 1`, `v̂ = −∇φ/‖∇φ‖` — is **pure advection**. It has
//! **no diffusion term at all**. So `m2`'s correct value is *as small as the grid permits*, and
//! **nothing in any router we have constrains it.** `Σw=1` pins `m0`; the moment correction pins
//! `m1`; `m2` is whatever the geometry happens to hand you.
//!
//! **Hypothesis H (under test): the cone catchment error is primarily a function of `m2`.**
//!
//! ## The confound this module exists to break
//!
//! The existing router table conflates **three** error channels, and they are not the same thing:
//!
//! | channel | what it is | fixed by |
//! |---|---|---|
//! | **(A) ψ-estimation** | the reconstructed gradient azimuth ≠ the true one | a better gradient |
//! | **(B) m1 about the router's OWN ψ** | the outflow does not point where the router thinks | `moment_correct` |
//! | **(C) m2** | the outflow spreads | *nothing we have* |
//!
//! `moment_correct` drives **(B)** to zero — it cannot touch **(A)**, because it is *told* ψ.
//! A router can therefore have `m1_self ≈ 0` (looks perfect) and `m1_true` large (is not).
//! **That distinction is invisible in every number measured so far**, and it is why the
//! composition of two "correct" fixes could come out worse than either parent. The **oracle**
//! rows below remove (A) by construction and let (B) and (C) be read off alone.
//!
//! ## The controls
//!
//! Every number here is charged against a control that could have killed it:
//! * **the face-centre band** — the quasi-lattice case, where the cube-sphere Jacobian is
//!   isotropic. A router that is broken *there* indicts the probe, not the sphere.
//! * **`RotWrecked`** — an honest router with its ψ rotated 20°. It **must scream**, on `m1_true`
//!   and on `sca_err` both. If it does not, the instrument is not reading.
//! * **determinism** — the whole sweep is run 3× and compared bit-for-bit. `QuadMoment` has
//!   already been caught once with a HashMap-iteration-order bug whose first numbers were
//!   *flattering*. Drift is a bug in the probe and it is the finding.

#![allow(dead_code)]

use crate::flow::{self, Router};
use crate::grids;
use crate::mesh::*;

/// One row of the ladder: a router, plus the probe-only knobs that isolate a channel.
#[derive(Clone, Copy, PartialEq)]
pub struct Rt {
    pub base: Router,
    /// ⚠ **ORACLE.** Feed the router the EXACT flow azimuth instead of its own reconstruction.
    /// Removes channel (A) entirely. **This is a DIAGNOSTIC, not a proposal — it consults the
    /// analytic answer, which a real kernel cannot do.** Rows using it are marked `@ψ*`.
    pub oracle_psi: bool,
    /// Rotate the (oracle) ψ by this many degrees — the control that must scream.
    pub psi_rot_deg: f64,
    /// MFD's slope exponent, lifted out of its hardcode.
    pub p: f64,
    pub tag: &'static str,
}

impl Rt {
    pub fn new(base: Router, tag: &'static str) -> Rt {
        Rt { base, oracle_psi: false, psi_rot_deg: 0.0, p: 1.1, tag }
    }
    pub fn oracle(base: Router, tag: &'static str) -> Rt {
        Rt { base, oracle_psi: true, psi_rot_deg: 0.0, p: 1.1, tag }
    }
    pub fn wrecked(base: Router, deg: f64, tag: &'static str) -> Rt {
        Rt { base, oracle_psi: true, psi_rot_deg: deg, p: 1.1, tag }
    }
    pub fn with_p(mut self, p: f64) -> Rt {
        self.p = p;
        self
    }
}

/// The exact downslope azimuth of a cone about `pole`, expressed in cell `i`'s **router frame**
/// (`flow::frame`) — the same frame the router will interpret a ψ in. `h = −geodesic(·, pole)`,
/// so downhill is *away from* the pole and `f̂ = −tangent(c, pole)`.
pub fn psi_true(g: &Mesh, i: usize, pole: V3) -> f64 {
    let (e0, e1) = flow::frame(g, i);
    let fhat = scale(tangent(g.centers[i], pole), -1.0);
    dot(fhat, e1).atan2(dot(fhat, e0))
}

/// The **contour width**: the cell's outflow face projected PERPENDICULAR to the flow,
/// `W = Σ_σ |σ|·max(0, n̂_σ·f̂)`. This is exactly what `flow::routing` divides by, reproduced
/// here so the two agree by construction rather than by hope.
///
/// `f̂` is the **exact** flow direction, not the router's — so every router is scored against
/// the same denominator and any difference between them is a difference in the routed mass.
pub fn contour_width(g: &Mesh, i: usize, fhat: V3) -> f64 {
    let c = g.centers[i];
    let mut w = 0.0;
    for e in &g.adj[i] {
        let mid = unit(add(g.verts[e.va as usize], g.verts[e.vb as usize]));
        let nh = tangent(c, tangent(mid, e.normal));
        w += e.edge_len_m * dot(nh, fhat).max(0.0);
    }
    w
}

/// The **arm** `x_σ − x_K` as a metre-scale 2-vector in cell `i`'s router frame, and the outward
/// unit normal `n_{K,σ}` in the same frame. Both parallel-transported to the cell centre.
///
/// Used by two things that must agree: the geometric identity check, and Coatléven's flux
/// reconstruction. Sharing the code means a bug in the geometry convicts both, loudly.
fn arm_and_normal(g: &Mesh, i: usize, e: &Edge, e0: V3, e1: V3) -> ([f64; 2], [f64; 2]) {
    let c = g.centers[i];
    let mid = unit(add(g.verts[e.va as usize], g.verts[e.vb as usize]));
    // geodesic arm: direction × arc length. (Not the chord — on the coarse tier they differ.)
    let t = tangent(c, mid);
    let len = geodesic(c, mid) * g.radius_m;
    let arm = [dot(t, e0) * len, dot(t, e1) * len];
    let nh = tangent(c, tangent(mid, e.normal));
    let nrm = [dot(nh, e0), dot(nh, e1)];
    (arm, nrm)
}

/// **THE GEOMETRIC IDENTITY** — `|K|·Id = Σ_σ |σ|·(x_σ − x_K) ⊗ n_{K,σ}` (Coatléven 2025 eq. 6).
///
/// This is the identity that licenses the whole flux-vector reconstruction: it is what turns a
/// set of scalar face fluxes back into a **vector**. It is a **PLANAR** identity (the divergence
/// theorem applied to the coordinate functions), and our cells are **spherical polygons** — so
/// on our grid it holds only up to a curvature residual, and *that residual is the price of the
/// reconstruction*. Nobody has measured it. Returns the relative Frobenius residual per cell.
pub fn identity_residual(g: &Mesh) -> Vec<f64> {
    (0..g.cells())
        .map(|i| {
            let (e0, e1) = flow::frame(g, i);
            let mut m = [[0.0f64; 2]; 2];
            for e in &g.adj[i] {
                let (arm, nrm) = arm_and_normal(g, i, e, e0, e1);
                for (a, mrow) in m.iter_mut().enumerate() {
                    for (b, mv) in mrow.iter_mut().enumerate() {
                        *mv += e.edge_len_m * arm[a] * nrm[b];
                    }
                }
            }
            let k = g.areas[i];
            let mut f = 0.0;
            for a in 0..2 {
                for b in 0..2 {
                    let target = if a == b { k } else { 0.0 };
                    f += (m[a][b] - target).powi(2);
                }
            }
            f.sqrt() / (k * 2f64.sqrt())
        })
        .collect()
}

/// Per-cell record. Everything the regressions and the azimuth binning need.
#[derive(Clone, Copy)]
pub struct Rec {
    pub theta: f64,
    /// signed (ψ_router − ψ_true), degrees
    pub psi_err: f64,
    /// Σ wₖ sin(βₖ − ψ_TRUE) — the drift error against **truth**
    pub m1_true: f64,
    /// Σ wₖ sin(βₖ − ψ_ROUTER) — what `moment_correct` drives to zero
    pub m1_self: f64,
    /// Σ wₖ sin²(βₖ − ψ_true) — the dispersion. **Unconstrained by every router we have.**
    pub m2_true: f64,
    /// |(A/W) / (R·tan(θ/2)) − 1| — exactly `flow::routing`'s score
    pub sca_err: f64,
    /// the same, but scoring Coatléven's ‖Q_K‖ instead of A/W
    pub sca_err_coat: f64,
    /// ‖Q_K‖ ÷ (A/W). **1.0 ⟺ the two are the same estimator.**
    pub coat_ratio: f64,
    /// flow azimuth relative to the local cube u-axis, folded to [0°, 90°)
    pub azim_face: f64,
    /// fraction of this cell's outflow sent to a Moore neighbour that shares **no edge** —
    /// i.e. mass crossing a face that does not exist. Coatléven's reconstruction cannot see it.
    pub phantom: f64,
}

pub struct Stats {
    pub tag: &'static str,
    pub conservation: f64,
    pub recs: Vec<Rec>,
}

fn mean(xs: impl Iterator<Item = f64>) -> f64 {
    let v: Vec<f64> = xs.collect();
    if v.is_empty() {
        return f64::NAN;
    }
    v.iter().sum::<f64>() / v.len() as f64
}
pub fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

impl Stats {
    pub fn m(&self, f: impl Fn(&Rec) -> f64) -> f64 {
        mean(self.recs.iter().map(|r| f(r)))
    }
    pub fn med(&self, f: impl Fn(&Rec) -> f64) -> f64 {
        median(self.recs.iter().map(|r| f(r)).collect())
    }
}

/// Route a cone and measure the whole ladder.
///
/// The setup is `flow::routing`'s, exactly — same pole, same field, same elevation-sorted sweep,
/// same θ band, same cell filter — so the `sca_err` column here is directly comparable to the
/// published table and any disagreement is a bug, not a difference of definition.
pub fn run(g: &Mesh, rt: Rt, pole: V3) -> Stats {
    let h: Vec<f64> = g.centers.iter().map(|&p| -geodesic(p, pole)).collect();

    let psi_for = |i: usize| -> Option<f64> {
        if !rt.oracle_psi {
            return None;
        }
        Some(psi_true(g, i, pole) + rt.psi_rot_deg.to_radians())
    };
    let w_at = |i: usize| flow::weights_with_psi(g, rt.base, &h, i, psi_for(i), rt.p);

    // the sweep — `erosion.rs`'s global elevation ordering
    let mut order: Vec<usize> = (0..g.cells()).collect();
    order.sort_by(|&a, &b| h[a].total_cmp(&h[b]).then_with(|| a.cmp(&b)));
    let mut acc: Vec<f64> = g.areas.clone();
    let mut wts: Vec<Vec<(usize, f64)>> = vec![Vec::new(); g.cells()];
    for &i in order.iter().rev() {
        let w = w_at(i);
        let a = acc[i];
        for &(j, x) in &w {
            acc[j] += a * x;
        }
        wts[i] = w;
    }

    let total = g.total_area();
    let sink: f64 = (0..g.cells()).filter(|&i| wts[i].is_empty()).map(|i| acc[i]).sum();

    let mut recs = Vec::new();
    for i in 0..g.cells() {
        let th = geodesic(g.centers[i], pole);
        if !(0.35..=2.2).contains(&th) || wts[i].is_empty() {
            continue;
        }
        let c = g.centers[i];
        let (e0, e1) = flow::frame(g, i);
        let fhat = scale(tangent(c, pole), -1.0);
        let pt = dot(fhat, e1).atan2(dot(fhat, e0));

        let wid = contour_width(g, i, fhat);
        if wid <= 0.0 {
            continue;
        }
        let exact = g.radius_m * (th / 2.0).tan();
        let sca_err = ((acc[i] / wid) / exact - 1.0).abs();

        // the router's OWN ψ — for a router that has none (MooreMfd / EdgeMfd) this is the
        // reconstruction it *would* have used, reported so the ψ column is never blank.
        let ps = psi_for(i)
            .or_else(|| flow::lsq_grad(g, &h, i).map(|(gx, gy, _, _)| (-gy).atan2(-gx)));
        let Some(ps) = ps else { continue };

        // ---- the moments, about TRUE ψ and about the router's OWN ψ -------------------
        let (mut m1t, mut m1s, mut m2t, mut phantom) = (0.0, 0.0, 0.0, 0.0);
        for &(j, wk) in &wts[i] {
            let t = tangent(c, g.centers[j]);
            let beta = dot(t, e1).atan2(dot(t, e0));
            m1t += wk * (beta - pt).sin();
            m1s += wk * (beta - ps).sin();
            m2t += wk * (beta - pt).sin().powi(2);
            if !g.adj[i].iter().any(|e| e.j == j) {
                phantom += wk; // a "neighbour" sharing only a vertex: a face that does not exist
            }
        }

        // ---- Coatléven's flux-vector reconstruction, eq. (12)–(13) --------------------
        //   Q_K = (1/|K|) Σ_σ F_{K,σ} (x_σ − x_K),   F signed OUTWARD.
        // Outflux through the face to j:  acc[i]·w_{i→j}.  Influx from a higher j: acc[j]·w_{j→i}.
        // ⚠ Only EDGE neighbours have a face. Whatever MFD sends to a diagonal has no face to
        // cross and is invisible here — that is `phantom`, and it is a defect of MFD, not of the
        // reconstruction.
        let mut q = [0.0f64; 2];
        for e in &g.adj[i] {
            let out: f64 = wts[i].iter().find(|&&(j, _)| j == e.j).map(|&(_, w)| acc[i] * w).unwrap_or(0.0);
            let inn: f64 = wts[e.j].iter().find(|&&(j, _)| j == i).map(|&(_, w)| acc[e.j] * w).unwrap_or(0.0);
            let f = out - inn;
            let (arm, _) = arm_and_normal(g, i, e, e0, e1);
            q[0] += f * arm[0];
            q[1] += f * arm[1];
        }
        let qn = (q[0] * q[0] + q[1] * q[1]).sqrt() / g.areas[i];

        // flow azimuth relative to the local cube u-axis, folded to [0°,90°) — the grid's own
        // 4-fold symmetry. THIS is the rotation test, taken for free: a router that is
        // rotation-invariant shows no structure across this axis.
        let f_id = g.part[i] as usize;
        let (_, du, _) = grids::cube_face_basis(f_id.min(5));
        let uhat = tangent(c, du);
        let vhat = cross(c, uhat);
        let az = dot(fhat, vhat).atan2(dot(fhat, uhat)).to_degrees().rem_euclid(90.0);

        recs.push(Rec {
            theta: th,
            psi_err: (ps - pt).sin().atan2((ps - pt).cos()).to_degrees(),
            m1_true: m1t,
            m1_self: m1s,
            m2_true: m2t,
            sca_err,
            sca_err_coat: (qn / exact - 1.0).abs(),
            coat_ratio: qn / (acc[i] / wid),
            azim_face: az,
            phantom,
        });
    }
    Stats { tag: rt.tag, conservation: sink / total, recs }
}

// ===========================================================================
// E1 — does m2 explain the error? (OLS, and it is allowed to say no)
// ===========================================================================

/// R² of `y` on the given predictors (with intercept). Plain OLS via normal equations; for 1–2
/// predictors that is exact and the conditioning is a non-issue at these magnitudes.
pub fn r2(y: &[f64], xs: &[&[f64]]) -> f64 {
    let n = y.len();
    if n < xs.len() + 2 {
        return f64::NAN;
    }
    let k = xs.len() + 1;
    // design matrix columns: 1, xs...
    let col = |c: usize, i: usize| -> f64 { if c == 0 { 1.0 } else { xs[c - 1][i] } };
    let mut ata = vec![vec![0.0f64; k]; k];
    let mut atb = vec![0.0f64; k];
    for i in 0..n {
        for a in 0..k {
            for b in 0..k {
                ata[a][b] += col(a, i) * col(b, i);
            }
            atb[a] += col(a, i) * y[i];
        }
    }
    // gaussian elimination with partial pivoting
    for c in 0..k {
        let piv = (c..k).max_by(|&a, &b| ata[a][c].abs().total_cmp(&ata[b][c].abs())).unwrap();
        ata.swap(c, piv);
        atb.swap(c, piv);
        if ata[c][c].abs() < 1e-300 {
            return f64::NAN;
        }
        for r in (c + 1)..k {
            let f = ata[r][c] / ata[c][c];
            for cc in c..k {
                ata[r][cc] -= f * ata[c][cc];
            }
            atb[r] -= f * atb[c];
        }
    }
    let mut beta = vec![0.0f64; k];
    for r in (0..k).rev() {
        let mut s = atb[r];
        for cc in (r + 1)..k {
            s -= ata[r][cc] * beta[cc];
        }
        beta[r] = s / ata[r][r];
    }
    let ybar = y.iter().sum::<f64>() / n as f64;
    let (mut ss_res, mut ss_tot) = (0.0, 0.0);
    for i in 0..n {
        let yhat: f64 = (0..k).map(|c| beta[c] * col(c, i)).sum();
        ss_res += (y[i] - yhat).powi(2);
        ss_tot += (y[i] - ybar).powi(2);
    }
    1.0 - ss_res / ss_tot
}

/// Mean of `y` per quantile bin of `x` — the non-parametric read, which cannot be faked by a
/// lucky linear fit.
pub fn binned(x: &[f64], y: &[f64], nb: usize) -> Vec<(f64, f64, usize)> {
    let mut idx: Vec<usize> = (0..x.len()).collect();
    idx.sort_by(|&a, &b| x[a].total_cmp(&x[b]));
    let per = idx.len() / nb.max(1);
    (0..nb)
        .filter_map(|b| {
            let s = b * per;
            let e = if b == nb - 1 { idx.len() } else { (b + 1) * per };
            if e <= s {
                return None;
            }
            let sl = &idx[s..e];
            Some((
                sl.iter().map(|&i| x[i]).sum::<f64>() / sl.len() as f64,
                sl.iter().map(|&i| y[i]).sum::<f64>() / sl.len() as f64,
                sl.len(),
            ))
        })
        .collect()
}
