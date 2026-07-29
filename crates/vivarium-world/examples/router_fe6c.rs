//! # FE(6c) pricing — the Coatléven flux-vector reconstruction on the receiver tree
//!
//! `#obs-routing-curl-spiral` FE(8) priced the routing spiral's landscape severity and
//! left one thing owed: the 2026-07-24 experiment convicted the **receiver/incision
//! tree** diagonal treatment (CUBE +0.07 → +0.26) using a **naive D4 steepest-descent**
//! tree, which its own RESULTS names a strawman. This harness prices the **principled**
//! remedy FE(6)(c) actually names:
//!
//! ```text
//!   Q_K = (1/|K|) Σ_σ F_{K,σ} (x_σ − x_K)     q_K = ‖Q_K‖     Q̂_K = Q_K/‖Q_K‖
//! ```
//!
//! (Coatléven & Chauveau 2025 eq. 12–13; the exactness identity is
//! `|K|·Id = Σ_σ |σ|(x_σ − x_K) ⊗ n̂_{K,σ}`, the divergence theorem on the coordinate
//! functions — Euclidean, and our cells are spherical, so its residual is a **gate**
//! here, not an assumption.)
//!
//! FE(6)(d) — the strongly consistent gradient reconstruction that Coatléven 2020
//! Def. 4.2 makes a **hypothesis** of Thm 6.1/Cor 6.2 — is priced as its own arm
//! (`CoatGrad` vs `CoatTpfa`) rather than folded in, because the segment records it as
//! a *precondition* of (c) and nobody has measured what it costs or buys.
//!
//! Predictions were written first: `msc/spike-router-fe6c/PREDICTIONS.md`.
//!
//! ## Standing limit, carved out rather than ignored
//!
//! Pits/flats/accumulation zones (discrete tell `s_K = 0`) are outside Coatléven's
//! well-posedness theory, and Priority-Flood is *how we make them*. Every metric is
//! therefore reported twice: over all channel cells, and over channel cells **not
//! raised by the fill pass** in the scored epoch.
//!
//! ## Faithfulness
//!
//! The fluvial pipeline is a verbatim port of `erosion::Fluvial`; arm `LiveMfd` must
//! reproduce the live kernel **bit-for-bit** (P0) or nothing here is about the live
//! world. Only the drainage/tree/magnitude steps vary between arms.
//!
//! Run: `cargo run --release -p vivarium-world --example router_fe6c`

// `Instant::now` is on the determinism ban-list for good reason. Here wall-clock
// IS one of the measurements (the affordability open in the census) and it never
// touches a keyed quantity: no arm's `h`, `drainage` or metric depends on it, and
// the determinism gate below re-runs every reconstruction arm 3x bit-for-bit.
#[allow(clippy::disallowed_methods)]
mod timing {
    pub fn now() -> std::time::Instant {
        std::time::Instant::now()
    }
}
use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CellId, CubeCoord, Face};
use vivarium_world::{gen, measure, sample, sea_level};

/// Moore offsets. The first FOUR are the face-sharing (edge) neighbours, in the order
/// the face geometry arrays use: +i, −i, +j, −j.
const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

/// Opposite face index for k in 0..4 (used to pair F_{K,σ} with F_{L,σ}).
const OPP: [usize; 4] = [1, 0, 3, 2];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Router {
    /// Retired kernel: 8-nbr fan, UNIFORM lengths. == erosion.rs before 6c1ad97.
    UniformOld,
    /// Current live kernel. The pricing baseline; must bit-match live erosion.rs.
    LiveMfd,
    /// 4-nbr fan (diagonals cross no face), D8 tree. The 2026-07-24 "fan half".
    EdgeTrue,
    /// 4-nbr fan + naive D4 steepest-descent tree. The 2026-07-24 STRAWMAN, kept here
    /// as the replication check against the prior experiment's CUBE band.
    EdgeFull,
    /// 4-nbr fan with TRANSMISSIVITY weights |σ|·drop/d, D8 tree. Control that keeps
    /// the |σ| factor from being confounded into the Coatléven arms.
    EdgeTau,
    /// FE(6c) without FE(6d): transmissivity face fluxes → reconstructed Q̂ drives the
    /// receiver tree. Two-point (TPFA) face slopes — the estimate that formally wants
    /// mesh orthogonality we do not have.
    CoatTpfa,
    /// FE(6c) WITH FE(6d): face weights from a strongly consistent LSQ gradient
    /// projected on the face normal, |σ|·max(−∇h·n̂, 0). Q̂ drives the tree.
    CoatGrad,
    /// ISOLATION ARM: the corrected gradient FE(6d) in the fan weights, but the
    /// live D8 steepest-descent tree — no reconstruction, no Q̂. Separates "the
    /// corrected gradient fixed the FAN" from "the reconstruction fixed the TREE".
    /// Added after the first seed sweep refuted P4: (d) turned out to carry the
    /// CUBE improvement, and this is the arm that says where (d) acts.
    GradFan,
    /// CoatGrad + the reconstruction's MAGNITUDE consumed downstream:
    /// `A_coat = ‖Q_K‖·√A_K` replaces the raw accumulation in incise/deposit/mask.
    /// (`‖Q‖` is a specific catchment area, units m; `√A_K` is the local cell width,
    /// so the product is an area that reduces to A in the constant-flux limit.)
    CoatMag,
}

use Router::*;

impl Router {
    fn label(self) -> &'static str {
        match self {
            UniformOld => "UniformOld  8-fan, uniform len, D8 tree   [pre-6c1ad97]",
            LiveMfd => "LiveMfd     8-fan, true gc,     D8 tree   [== erosion.rs today]",
            EdgeTrue => "EdgeTrue    4-fan drop/d,      D8 tree   [fan-half diag-kill]",
            EdgeFull => "EdgeFull    4-fan drop/d,      D4 tree   [STRAWMAN, replication]",
            EdgeTau => "EdgeTau     4-fan |s|drop/d,    D8 tree   [transmissivity control]",
            CoatTpfa => "CoatTpfa    4-fan |s|drop/d,   Q-hat tree [FE(6c), no (6d)]",
            CoatGrad => "CoatGrad    4-fan |s|LSQ-grad, Q-hat tree [FE(6c)+(6d)]",
            GradFan => "GradFan     4-fan |s|LSQ-grad, D8 tree   [FE(6d) alone, no (6c)]",
            CoatMag => "CoatMag     CoatGrad + ||Q||*sqrt(A) consumed [full FE(6c)]",
        }
    }
    fn fan_n_nbr(self) -> usize {
        match self {
            UniformOld | LiveMfd => 8,
            _ => 4,
        }
    }
    /// Face weighting inside the 4-fan.
    fn tau_weight(self) -> bool {
        matches!(self, EdgeTau | CoatTpfa | CoatGrad | CoatMag | GradFan)
    }
    fn lsq_weight(self) -> bool {
        matches!(self, CoatGrad | CoatMag | GradFan)
    }
    fn reconstructs(self) -> bool {
        matches!(self, CoatTpfa | CoatGrad | CoatMag)
    }
    fn qhat_tree(self) -> bool {
        matches!(self, CoatTpfa | CoatGrad | CoatMag)
    }
    fn consume_qnorm(self) -> bool {
        matches!(self, CoatMag)
    }
    fn recv_n_nbr(self) -> usize {
        match self {
            EdgeFull => 4,
            _ => 8,
        }
    }
    fn uniform_dist(self) -> bool {
        matches!(self, UniformOld)
    }
}

// ----------------------------- small linear algebra ---------------------------

type V3 = [f64; 3];
#[inline]
fn dot3(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline]
fn cross3(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
#[inline]
fn norm3(a: V3) -> f64 {
    dot3(a, a).sqrt()
}
#[inline]
fn unit3(a: V3) -> V3 {
    let n = norm3(a).max(1e-300);
    [a[0] / n, a[1] / n, a[2] / n]
}
/// Unit tangent at `p` pointing toward `v` (both unit vectors).
#[inline]
fn tangent(p: V3, v: V3) -> V3 {
    let d = dot3(p, v);
    unit3([v[0] - d * p[0], v[1] - d * p[1], v[2] - d * p[2]])
}

/// Per-cell face geometry in the cell's own tangent basis (metres).
#[derive(Clone, Default)]
struct CellGeom {
    /// Bearing to each of the 8 Moore neighbours, unit, in (e1,e2).
    bear: [[f64; 2]; 8],
    /// Face (edge) length |σ| in m, for the 4 face-sharing neighbours.
    elen: [f64; 4],
    /// Lever arm (x_σ − x_K) in m, in (e1,e2).
    arm: [[f64; 2]; 4],
    /// Outward face normal n̂_{K,σ}, unit, in (e1,e2).
    nrm: [[f64; 2]; 4],
    /// Area of the straight-edged tangent-plane quad — the polygon the Euclidean
    /// identity is actually stated on. Kept apart from the exact SPHERICAL
    /// `cell_area`, because the two differ and the difference is a measurement.
    parea: f64,
}

/// The pipeline — verbatim port of `erosion::Fluvial` plus the FE(6c) machinery.
struct Pipe {
    nx: usize,
    cell_m: f32,
    cell_area: Vec<f32>,
    centers: Vec<V3>,
    geom: Vec<CellGeom>,
    h: Vec<f32>,
    drainage: Vec<f32>,
    uplift_rate: Vec<f32>,
    precip_weight: Vec<f32>,
    router: Router,
    // ---- diagnostics, filled during the scored (final) epoch ----
    /// Cells raised by the fill pass in the final epoch (the s_K = 0 manufactory).
    filled: Vec<bool>,
    /// ‖Q_K‖ from the final epoch's reconstruction (m), if the arm reconstructs.
    qnorm: Vec<f32>,
    /// Raw accumulation from the final epoch (before any magnitude swap).
    raw_acc: Vec<f32>,
    /// Angle (deg) between Q̂ and the chosen receiver bearing, final epoch.
    tree_resid_deg: Vec<f32>,
    /// Counters (final epoch).
    n_lsq_fallback: u64,
    n_tree_fallback: u64,
}

impl Pipe {
    fn from_surface(
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        router: Router,
        surf: impl Fn(CellId) -> f64,
    ) -> Self {
        let radius = Planet::EARTH.radius_m;
        let cell_m = sample::cell_size_m(level, radius) as f32;
        let n = nx * nx;
        let mut h = vec![0.0f32; n];
        let mut cell_area = vec![0.0f32; n];
        let mut centers = vec![[0.0f64; 3]; n];
        for y in 0..nx {
            for x in 0..nx {
                let gi = oi + x as u32;
                let gj = oj + y as u32;
                let cell = CellId::from_face_ij(face, gi, gj, level);
                h[y * nx + x] = surf(cell) as f32;
                cell_area[y * nx + x] =
                    measure::cell_area_m2(face, gi as u64, gj as u64, level, radius) as f32;
                centers[y * nx + x] = measure::cell_center_unit(face, gi as u64, gj as u64, level);
            }
        }
        let geom = if router.reconstructs() || router.lsq_weight() || router.tau_weight() {
            build_geom(face, level, oi, oj, nx, radius, &centers)
        } else {
            Vec::new()
        };
        Self {
            nx,
            cell_m,
            cell_area,
            centers,
            geom,
            h,
            drainage: vec![0.0; n],
            uplift_rate: vec![0.0; n],
            precip_weight: vec![1.0; n],
            router,
            filled: vec![false; n],
            qnorm: vec![0.0; n],
            raw_acc: vec![0.0; n],
            tree_resid_deg: vec![f32::NAN; n],
            n_lsq_fallback: 0,
            n_tree_fallback: 0,
        }
    }

    fn set_uniform_uplift(&mut self, rate: f32) {
        self.uplift_rate = vec![rate; self.nx * self.nx];
    }

    #[inline]
    fn is_edge(nx: usize, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == nx - 1 || y == nx - 1
    }

    #[inline]
    fn dist_m(&self, a: usize, b: usize) -> f32 {
        if self.router.uniform_dist() {
            let nx = self.nx;
            let (ax, ay) = (a % nx, a / nx);
            let (bx, by) = (b % nx, b / nx);
            if ax != bx && ay != by {
                self.cell_m * std::f32::consts::SQRT_2
            } else {
                self.cell_m
            }
        } else {
            measure::gc_dist_m(self.centers[a], self.centers[b], Planet::EARTH.radius_m) as f32
        }
    }

    fn outlets(&self, sea: f32) -> Vec<bool> {
        let nx = self.nx;
        let mut out = vec![false; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                out[i] = Self::is_edge(nx, x, y) || self.h[i] <= sea;
            }
        }
        out
    }

    /// Priority-Flood, ported from the live kernel as of `1c1c5a1` — mutates `h`
    /// into the **routing surface** and returns the **standing-water depth** (spill
    /// level minus bed, ε excluded). The ε is a numerical device and is undone
    /// after incision; `water > 0` is a lake and does not incise.
    fn fill_depressions(&mut self, outlets: &[bool]) -> Vec<f32> {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;
        let nx = self.nx;
        const EPS: f32 = 1e-3;
        struct Cell {
            elev: f32,
            spill: f32,
            seq: u64,
            i: usize,
        }
        impl PartialEq for Cell {
            fn eq(&self, o: &Self) -> bool {
                self.elev == o.elev && self.seq == o.seq
            }
        }
        impl Eq for Cell {}
        impl Ord for Cell {
            fn cmp(&self, o: &Self) -> Ordering {
                o.elev.total_cmp(&self.elev).then_with(|| o.seq.cmp(&self.seq))
            }
        }
        impl PartialOrd for Cell {
            fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
                Some(self.cmp(o))
            }
        }
        let bed = self.h.clone();
        let mut water = vec![0.0f32; nx * nx];
        let mut closed = vec![false; nx * nx];
        let mut heap = BinaryHeap::new();
        let mut seq = 0u64;
        for (i, &is_out) in outlets.iter().enumerate() {
            if is_out {
                closed[i] = true;
                heap.push(Cell { elev: self.h[i], spill: self.h[i], seq, i });
                seq += 1;
            }
        }
        while let Some(Cell { elev, spill, i, .. }) = heap.pop() {
            let (x, y) = (i % nx, i / nx);
            for (dx, dy) in NEIGHBORS {
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                if closed[j] {
                    continue;
                }
                closed[j] = true;
                let spill_j = bed[j].max(spill);
                water[j] = spill_j - bed[j];
                self.h[j] = self.h[j].max(elev + EPS);
                heap.push(Cell { elev: self.h[j], spill: spill_j, seq, i: j });
                seq += 1;
            }
        }
        water
    }

    fn elevation_order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.h.len()).collect();
        order.sort_by(|&a, &b| self.h[a].total_cmp(&self.h[b]).then_with(|| a.cmp(&b)));
        order
    }

    // ---------------- FE(6d): strongly consistent gradient reconstruction ---------

    /// Weighted least-squares gradient over the Moore stencil, in the cell's own
    /// tangent basis (m⁻¹ · elevation-units). This is Coatléven 2020 Def. 4.2's
    /// *strongly consistent gradient reconstruction operator* — the hypothesis his
    /// convergence theorem takes, and the reason our 28.79°-non-orthogonal quads do
    /// not need the two-point flux estimate.
    fn lsq_gradients(&mut self) -> Vec<[f64; 2]> {
        let nx = self.nx;
        let mut g = vec![[0.0f64; 2]; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let gm = &self.geom[i];
                let (mut a00, mut a01, mut a11) = (0.0f64, 0.0f64, 0.0f64);
                let (mut b0, mut b1) = (0.0f64, 0.0f64);
                for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                    let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                    if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                        continue;
                    }
                    let j = nyp as usize * nx + nxp as usize;
                    let d = self.dist_m(i, j) as f64;
                    let vx = gm.bear[k][0] * d;
                    let vy = gm.bear[k][1] * d;
                    let w = 1.0 / (d * d);
                    let dh = (self.h[j] - self.h[i]) as f64;
                    a00 += w * vx * vx;
                    a01 += w * vx * vy;
                    a11 += w * vy * vy;
                    b0 += w * vx * dh;
                    b1 += w * vy * dh;
                }
                let det = a00 * a11 - a01 * a01;
                if det.abs() > 1e-12 * (a00 * a11).abs().max(1e-30) {
                    g[i] = [(a11 * b0 - a01 * b1) / det, (a00 * b1 - a01 * b0) / det];
                } else {
                    self.n_lsq_fallback += 1;
                }
            }
        }
        g
    }

    // ---------------- the swapped step -------------------------------------------

    /// Accumulate drainage. Returns the per-face outgoing amounts for the 4 face
    /// neighbours (`out[i][k]`), which the reconstruction consumes; empty for the
    /// 8-fan arms, whose diagonal weights cross no face and have no `F_σ`.
    fn accumulate_drainage(&mut self, order: &[usize], grad: Option<&[[f64; 2]]>) -> Vec<[f32; 4]> {
        const P: f32 = 1.0;
        let nx = self.nx;
        let n_nbr = self.router.fan_n_nbr();
        let tau_w = self.router.tau_weight();
        let want_faces = self.router.reconstructs();
        let n = self.drainage.len();
        for i in 0..n {
            self.drainage[i] = self.cell_area[i] * self.precip_weight[i];
        }
        let mut out = if want_faces { vec![[0.0f32; 4]; n] } else { Vec::new() };
        for &i in order.iter().rev() {
            let (x, y) = (i % nx, i / nx);
            let hi = self.h[i];
            let mut weights = [0.0f32; 8];
            let mut total = 0.0f32;
            for k in 0..n_nbr {
                let (dx, dy) = NEIGHBORS[k];
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                let drop = hi - self.h[j];
                if drop > 0.0 {
                    let dist = self.dist_m(i, j);
                    let w = if let Some(g) = grad {
                        // FE(6d): face weight from the LSQ gradient projected on n̂_σ.
                        // Downhill-restricted (h_j < h_i) so the accumulation stays
                        // acyclic under the elevation order; the gradient decides HOW
                        // MUCH goes through each downhill face, not WHETHER.
                        let gm = &self.geom[i];
                        let s = -(g[i][0] * gm.nrm[k][0] + g[i][1] * gm.nrm[k][1]);
                        (s.max(0.0) * gm.elen[k]) as f32
                    } else if tau_w {
                        (drop / dist).powf(P) * self.geom[i].elen[k] as f32
                    } else {
                        (drop / dist).powf(P)
                    };
                    weights[k] = w;
                    total += w;
                }
            }
            // If the corrected gradient sends nothing through any downhill face
            // (it can: ∇h can point at a face whose neighbour is higher), fall back
            // to the two-point weights rather than trapping the mass.
            if total <= 0.0 && grad.is_some() {
                self.n_lsq_fallback += 1;
                for k in 0..n_nbr {
                    let (dx, dy) = NEIGHBORS[k];
                    let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                    if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                        continue;
                    }
                    let j = nyp as usize * nx + nxp as usize;
                    let drop = hi - self.h[j];
                    if drop > 0.0 {
                        let w = (drop / self.dist_m(i, j)).powf(P) * self.geom[i].elen[k] as f32;
                        weights[k] = w;
                        total += w;
                    }
                }
            }
            if total > 0.0 {
                let amount = self.drainage[i];
                for k in 0..n_nbr {
                    if weights[k] > 0.0 {
                        let (dx, dy) = NEIGHBORS[k];
                        let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                        let send = amount * (weights[k] / total);
                        self.drainage[j] += send;
                        if want_faces && k < 4 {
                            out[i][k] = send;
                        }
                    }
                }
            }
        }
        out
    }

    /// Coatléven eq. (12)–(13): reconstruct the flux VECTOR from the signed face
    /// fluxes, `Q_K = (1/|K|) Σ_σ F_{K,σ} (x_σ − x_K)`. `F` is antisymmetric by
    /// construction (`F_{K,σ} = send(K→L) − send(L→K)`), so the scheme stays
    /// conservative and the influx faces contribute with the opposite sign AND the
    /// opposite lever arm — the factor a naive outflux-only reading loses.
    fn reconstruct_q(&self, out: &[[f32; 4]]) -> Vec<[f64; 2]> {
        let nx = self.nx;
        let mut q = vec![[0.0f64; 2]; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let gm = &self.geom[i];
                let mut acc = [0.0f64, 0.0];
                for k in 0..4 {
                    let (dx, dy) = NEIGHBORS[k];
                    let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                    if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                        continue;
                    }
                    let j = nyp as usize * nx + nxp as usize;
                    let f = (out[i][k] - out[j][OPP[k]]) as f64;
                    acc[0] += f * gm.arm[k][0];
                    acc[1] += f * gm.arm[k][1];
                }
                let a = self.cell_area[i] as f64;
                q[i] = [acc[0] / a, acc[1] / a];
            }
        }
        q
    }

    /// Receiver tree. `qhat` present ⇒ the receiver is the downhill Moore neighbour
    /// whose bearing is closest to the reconstructed direction Q̂_K (the minimal-loss
    /// projection of a continuous direction onto the tree's discrete choice); the
    /// angular residual is recorded because it is the *irreducible* lattice lock a
    /// single-receiver tree imposes even given a perfect direction.
    fn receivers(&mut self, outlets: &[bool], qhat: Option<&[[f64; 2]]>, score: bool) -> Vec<usize> {
        let nx = self.nx;
        let nxi = nx as i32;
        let mut recv = vec![0usize; nx * nx];
        let mut fallback = 0u64;
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                if outlets[i] {
                    recv[i] = i;
                    continue;
                }
                let hi = self.h[i];
                if let Some(q) = qhat {
                    let qn = (q[i][0] * q[i][0] + q[i][1] * q[i][1]).sqrt();
                    if qn > 0.0 {
                        let qx = q[i][0] / qn;
                        let qy = q[i][1] / qn;
                        let gm = &self.geom[i];
                        let (mut best, mut best_cos) = (i, -2.0f64);
                        for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                            let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                            if nx_ < 0 || ny_ < 0 || nx_ >= nxi || ny_ >= nxi {
                                continue;
                            }
                            let j = ny_ as usize * nx + nx_ as usize;
                            if self.h[j] >= hi {
                                continue; // acyclicity: receivers must descend
                            }
                            let c = gm.bear[k][0] * qx + gm.bear[k][1] * qy;
                            if c > best_cos {
                                best_cos = c;
                                best = j;
                            }
                        }
                        if best != i {
                            recv[i] = best;
                            if score {
                                self.tree_resid_deg[i] =
                                    best_cos.clamp(-1.0, 1.0).acos().to_degrees() as f32;
                            }
                            continue;
                        }
                    }
                    fallback += 1;
                }
                // D8 (or D4 for EdgeFull) steepest descent — the live rule.
                let (mut best, mut best_slope) = (i, 0.0f32);
                for k in 0..self.router.recv_n_nbr() {
                    let (dx, dy) = NEIGHBORS[k];
                    let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                    if nx_ < 0 || ny_ < 0 || nx_ >= nxi || ny_ >= nxi {
                        continue;
                    }
                    let j = ny_ as usize * nx + nx_ as usize;
                    let dist = self.dist_m(i, j);
                    let slope = (hi - self.h[j]) / dist;
                    if slope > best_slope {
                        best_slope = slope;
                        best = j;
                    }
                }
                recv[i] = best;
            }
        }
        if score {
            self.n_tree_fallback = fallback;
        }
        recv
    }

    fn incise(&mut self, p: &FluvialParams, recv: &[usize], order: &[usize], water: &[f32]) {
        for &i in order {
            let r = recv[i];
            if r == i || water[i] > 0.0 {
                continue;
            }
            let dist = self.dist_m(i, r);
            let f = p.k_dt * self.drainage[i].powf(p.m) / dist;
            self.h[i] = (self.h[i] + f * self.h[r]) / (1.0 + f);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn deposit(
        &mut self,
        p: &FluvialParams,
        recv: &[usize],
        order: &[usize],
        before: &[f32],
        water: &mut [f32],
        raise: &mut [f32],
    ) {
        let n = self.nx * self.nx;
        let mut qs = vec![0.0f32; n];
        for i in 0..n {
            let eroded = before[i] - self.h[i];
            if eroded > 0.0 {
                qs[i] = eroded * self.cell_area[i];
            }
        }
        for &i in order.iter().rev() {
            let area = self.cell_area[i];
            if water[i] > 0.0 {
                let settled = qs[i].min(water[i] * area) / area;
                water[i] -= settled;
                raise[i] -= settled;
                qs[i] -= settled * area;
            } else {
                let a = self.drainage[i].max(area);
                let deposit_h = p.deposition * qs[i] / a;
                let deposit_vol = (deposit_h * area).min(qs[i]);
                self.h[i] += deposit_vol / area;
                qs[i] -= deposit_vol;
            }
            let r = recv[i];
            if r != i {
                qs[r] += qs[i];
            }
        }
    }

    fn talus(&mut self, p: &FluvialParams) {
        let nx = self.nx;
        let snapshot = self.h.clone();
        let mut delta = vec![0.0f32; nx * nx];
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                let i = y * nx + x;
                let hi = snapshot[i];
                let (mut best, mut best_drop, mut best_dist) = (i, 0.0f32, self.cell_m);
                for (dx, dy) in NEIGHBORS {
                    let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                    let dist = self.dist_m(i, j);
                    let drop = hi - snapshot[j];
                    if drop / dist > best_drop / best_dist {
                        best_drop = drop;
                        best_dist = dist;
                        best = j;
                    }
                }
                if best == i {
                    continue;
                }
                let excess = best_drop - p.max_slope * best_dist;
                if excess > 0.0 {
                    let moved = excess * 0.5;
                    delta[i] -= moved;
                    delta[best] += moved;
                }
            }
        }
        for (h, d) in self.h.iter_mut().zip(delta.iter()) {
            *h += *d;
        }
    }

    fn creep(&mut self, p: &FluvialParams) {
        let k = (p.diffusivity_m2 / (self.cell_m * self.cell_m)).min(0.24);
        if k < 1e-5 {
            return;
        }
        let nx = self.nx;
        let snapshot = self.h.clone();
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                let i = y * nx + x;
                let lap = snapshot[i - 1] + snapshot[i + 1] + snapshot[i - nx] + snapshot[i + nx]
                    - 4.0 * snapshot[i];
                self.h[i] += k * lap;
            }
        }
    }

    fn erode(&mut self, p: &FluvialParams, sea: f32) {
        let nx = self.nx;
        for e in 0..p.epochs {
            let score = e + 1 == p.epochs;
            let outlets = self.outlets(sea);
            for i in 0..nx * nx {
                let rate = self.uplift_rate[i];
                if rate != 0.0 {
                    let (x, y) = (i % nx, i / nx);
                    if !Self::is_edge(nx, x, y) {
                        self.h[i] += rate;
                    }
                }
            }
            let bed = self.h.clone();
            let mut water = self.fill_depressions(&outlets);
            let mut raise: Vec<f32> =
                self.h.iter().zip(bed.iter()).map(|(f, b)| f - b).collect();
            if score {
                for i in 0..nx * nx {
                    // The s_K = 0 manufactory: every cell the fill pass raised,
                    // whether by a spill level or by the ε across a flat.
                    self.filled[i] = raise[i] > 0.0;
                }
            }
            if score {
                self.n_lsq_fallback = 0; // per-epoch, not cumulative over the run
            }
            let grad = if self.router.lsq_weight() { Some(self.lsq_gradients()) } else { None };
            let order = self.elevation_order();
            let faces = self.accumulate_drainage(&order, grad.as_deref());
            let q = if self.router.reconstructs() { Some(self.reconstruct_q(&faces)) } else { None };
            if score {
                self.raw_acc.copy_from_slice(&self.drainage);
                if let Some(qq) = &q {
                    for i in 0..nx * nx {
                        self.qnorm[i] = (qq[i][0] * qq[i][0] + qq[i][1] * qq[i][1]).sqrt() as f32;
                    }
                }
            }
            let recv = self.receivers(
                &outlets,
                if self.router.qhat_tree() { q.as_deref() } else { None },
                score,
            );
            if self.router.consume_qnorm() {
                let qq = q.as_ref().expect("CoatMag reconstructs");
                for i in 0..nx * nx {
                    let qn = (qq[i][0] * qq[i][0] + qq[i][1] * qq[i][1]).sqrt();
                    // ‖Q‖ is a specific catchment area (m); √A is the local cell width.
                    self.drainage[i] = (qn * (self.cell_area[i] as f64).sqrt()) as f32;
                }
            }
            let before = if p.deposition > 0.0 { Some(self.h.clone()) } else { None };
            self.incise(p, &recv, &order, &water);
            if let Some(b) = before {
                self.deposit(p, &recv, &order, &b, &mut water, &mut raise);
            }
            // Undo the fill: what is left is the real bed, with no ε-rock minted.
            for (h, r) in self.h.iter_mut().zip(raise.iter()) {
                *h -= *r;
            }
            self.talus(p);
            self.creep(p);
        }
    }
}

// ------------------------------- geometry -------------------------------------

/// Face-geometry precompute: |σ|, the lever arm (x_σ − x_K) and the outward normal
/// n̂_{K,σ}, all in the cell's own orthonormal tangent basis, in metres. Everything
/// the reconstruction identity needs, built from the equiangular corner map.
fn build_geom(
    face: Face,
    level: u8,
    oi: u32,
    oj: u32,
    nx: usize,
    radius: f64,
    centers: &[V3],
) -> Vec<CellGeom> {
    let uv = |i: f64, j: f64| -> V3 {
        let n = (1u64 << level) as f64;
        let u = 2.0 * i / n - 1.0;
        let v = 2.0 * j / n - 1.0;
        CubeCoord { face, u, v }.to_unit()
    };
    let mut out = vec![CellGeom::default(); nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            let i = y * nx + x;
            let gi = (oi + x as u32) as f64;
            let gj = (oj + y as u32) as f64;
            let c = centers[i];
            // Grid-aligned tangent basis: e1 toward +i, e2 = c × e1.
            let e1 = tangent(c, uv(gi + 1.5, gj + 0.5));
            let e2 = unit3(cross3(c, e1));
            let proj = |v: V3| -> [f64; 2] { [dot3(v, e1), dot3(v, e2)] };

            let mut g = CellGeom::default();
            for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                let nb = uv(gi + 0.5 + *dx as f64, gj + 0.5 + *dy as f64);
                g.bear[k] = proj(tangent(c, nb));
            }
            // Face-sharing neighbours: +i, −i, +j, −j — corners of the shared edge.
            let faces: [(V3, V3, V3); 4] = [
                (uv(gi + 1.0, gj), uv(gi + 1.0, gj + 1.0), uv(gi + 1.0, gj + 0.5)),
                (uv(gi, gj), uv(gi, gj + 1.0), uv(gi, gj + 0.5)),
                (uv(gi, gj + 1.0), uv(gi + 1.0, gj + 1.0), uv(gi + 0.5, gj + 1.0)),
                (uv(gi, gj), uv(gi + 1.0, gj), uv(gi + 0.5, gj)),
            ];
            // Geodesic DISPLACEMENT of a point into the cell's tangent plane, in
            // metres. (Using the unit bearing here instead of the displacement is
            // the bug the P1 identity gate caught on the first run: bearings to the
            // two edge endpoints do not differ by the edge vector.)
            let disp = |v: V3| -> [f64; 2] {
                let d = proj(tangent(c, v));
                let l = measure::gc_dist_m(c, v, radius);
                [d[0] * l, d[1] * l]
            };
            for (k, (va, vb, mid)) in faces.iter().enumerate() {
                let ea = disp(*va);
                let eb = disp(*vb);
                g.elen[k] = ((eb[0] - ea[0]).powi(2) + (eb[1] - ea[1]).powi(2)).sqrt();
                // x_σ is the edge MIDPOINT of the straight-edged polygon the identity
                // is stated on — the chord midpoint, not the parametric midpoint.
                g.arm[k] = [0.5 * (ea[0] + eb[0]), 0.5 * (ea[1] + eb[1])];
                let arm_dir = g.arm[k];
                let (ex, ey) = (eb[0] - ea[0], eb[1] - ea[1]);
                let el = (ex * ex + ey * ey).sqrt().max(1e-300);
                let mut n = [ey / el, -ex / el];
                if n[0] * arm_dir[0] + n[1] * arm_dir[1] < 0.0 {
                    n = [-n[0], -n[1]];
                }
                g.nrm[k] = n;
            }
            // Shoelace over the four corner displacements.
            let cs = [
                disp(uv(gi, gj)),
                disp(uv(gi + 1.0, gj)),
                disp(uv(gi + 1.0, gj + 1.0)),
                disp(uv(gi, gj + 1.0)),
            ];
            let mut sh = 0.0;
            for k in 0..4 {
                let (a, b) = (cs[k], cs[(k + 1) % 4]);
                sh += a[0] * b[1] - b[0] * a[1];
            }
            g.parea = (sh / 2.0).abs();
            out[i] = g;
        }
    }
    out
}

/// P1 gate: Frobenius residual of `|K|·Id = Σ_σ |σ|(x_σ − x_K) ⊗ n̂_{K,σ}` — the
/// identity that makes the reconstruction exact, on Euclidean polygons. Our cells are
/// spherical quads, so this is a measurement, not an assumption (DERIVATION §6 flags
/// it as possibly fatal at the coarse tier).
fn identity_residual(geom: &[CellGeom], area: &[f32]) -> (f64, f64, f64) {
    let mut worst = 0.0f64;
    let mut sum = 0.0f64;
    let mut worst_area = 0.0f64;
    let mut n = 0u64;
    for (g, &asph) in geom.iter().zip(area) {
        let mut m = [[0.0f64; 2]; 2];
        for k in 0..4 {
            for r in 0..2 {
                for cc in 0..2 {
                    m[r][cc] += g.elen[k] * g.arm[k][r] * g.nrm[k][cc];
                }
            }
        }
        let a = g.parea;
        let d = ((m[0][0] / a - 1.0).powi(2)
            + (m[1][1] / a - 1.0).powi(2)
            + (m[0][1] / a).powi(2)
            + (m[1][0] / a).powi(2))
        .sqrt();
        worst = worst.max(d);
        sum += d;
        worst_area = worst_area.max((asph as f64 / a - 1.0).abs());
        n += 1;
    }
    (worst, sum / n.max(1) as f64, worst_area)
}

// ------------------------------- terrain -------------------------------------

fn terrain(
    seed: u64,
    _face: Face,
    level: u8,
    oi: u32,
    oj: u32,
    nx: usize,
    dome_m: f64,
) -> impl Fn(CellId) -> f64 {
    let sea = sea_level::derived_sea_level_m(seed);
    move |c: CellId| -> f64 {
        let (_, i, j, _) = c.to_face_ij();
        let di = i as f64 - (oi as f64 + nx as f64 / 2.0);
        let dj = j as f64 - (oj as f64 + nx as f64 / 2.0);
        let r2 = (di * di + dj * dj) / ((nx as f64 / 2.0).powi(2));
        let dome = dome_m * (1.0 - 0.85 * r2).max(-0.3);
        // Band-limited prior detail: the increment the tectonic surface gains
        // between nyquist level `level-5` and `level`. The inherited form of this
        // line read `initial_topography_m(seed, c, c.level()) - ...(seed, c, level)`,
        // which is IDENTICALLY ZERO here because the harness builds every cell at
        // `level`, so `c.level() == level`. That made the terrain a bare analytic
        // paraboloid — see RESULTS: it is the single largest validity finding of
        // this spike, and it applies to the 2026-07-24 experiment too.
        let detail = gen::initial_topography_m(seed, c, level)
            - gen::initial_topography_m(seed, c, level.saturating_sub(5));
        sea + dome + 0.4 * detail
    }
}

// ------------------------------- metrics -------------------------------------

fn channel_mask(drainage: &[f32], cell_area: &[f32], h: &[f32], sea: f32, tau: f32) -> Vec<bool> {
    drainage
        .iter()
        .zip(cell_area)
        .zip(h)
        .map(|((&d, &a), &hh)| hh > sea && d > tau * a)
        .collect()
}

fn jaccard_disagreement(a: &[bool], b: &[bool]) -> f64 {
    let (mut inter, mut union) = (0u64, 0u64);
    for (&x, &y) in a.iter().zip(b) {
        if x || y {
            union += 1;
            if x && y {
                inter += 1;
            }
        }
    }
    if union == 0 {
        return 0.0;
    }
    1.0 - inter as f64 / union as f64
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    for (rank, &i) in idx.iter().enumerate() {
        r[i] = rank as f64;
    }
    r
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    let (mut cov, mut va, mut vb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        cov += (x - ma) * (y - mb);
        va += (x - ma) * (x - ma);
        vb += (y - mb) * (y - mb);
    }
    cov / (va.sqrt() * vb.sqrt()).max(1e-12)
}

fn spearman_log(da: &[f32], db: &[f32], h: &[f32], sea: f32) -> f64 {
    let idx: Vec<usize> = (0..h.len()).filter(|&i| h[i] > sea).collect();
    if idx.len() < 3 {
        return f64::NAN;
    }
    let la: Vec<f64> = idx.iter().map(|&i| (da[i].max(1.0) as f64).ln()).collect();
    let lb: Vec<f64> = idx.iter().map(|&i| (db[i].max(1.0) as f64).ln()).collect();
    pearson(&ranks(&la), &ranks(&lb))
}

/// M3: channel-orientation axis-fraction. Terrain steepest-descent azimuth binned
/// axis-vs-diagonal over channel cells. `exclude` masks out cells the fill pass
/// manufactured (the s_K = 0 carve-out) when requested.
fn axis_fraction(mask: &[bool], h: &[f32], nx: usize, exclude: Option<&[bool]>) -> f64 {
    let (mut axis, mut diag) = (0u64, 0u64);
    for y in 0..nx {
        for x in 0..nx {
            let i = y * nx + x;
            if !mask[i] {
                continue;
            }
            if let Some(e) = exclude {
                if e[i] {
                    continue;
                }
            }
            let hi = h[i];
            let mut best_drop = 0.0f32;
            let mut best_is_diag = None;
            for (dx, dy) in NEIGHBORS {
                let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                if nx_ < 0 || ny_ < 0 || nx_ >= nx as i32 || ny_ >= nx as i32 {
                    continue;
                }
                let j = ny_ as usize * nx + nx_ as usize;
                let d = if dx != 0 && dy != 0 { std::f32::consts::SQRT_2 } else { 1.0 };
                let drop = (hi - h[j]) / d;
                if drop > best_drop {
                    best_drop = drop;
                    best_is_diag = Some(dx != 0 && dy != 0);
                }
            }
            match best_is_diag {
                Some(true) => diag += 1,
                Some(false) => axis += 1,
                None => {}
            }
        }
    }
    let tot = (axis + diag).max(1) as f64;
    axis as f64 / tot
}

fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

// ------------------------------- driver --------------------------------------

/// Everything one arm at one site produces.
struct ArmOut {
    h: Vec<f32>,
    drainage: Vec<f32>,
    raw_acc: Vec<f32>,
    qnorm: Vec<f32>,
    cell_area: Vec<f32>,
    filled: Vec<bool>,
    tree_resid_deg: Vec<f32>,
    n_lsq_fallback: u64,
    n_tree_fallback: u64,
    secs: f64,
    geom_ok: Option<(f64, f64, f64)>,
}

#[derive(Clone, Copy)]
struct Site {
    name: &'static str,
    face: Face,
    oi: u32,
    oj: u32,
}

#[allow(clippy::too_many_arguments)]
fn run_arm(
    router: Router,
    seed: u64,
    _face_ignored: Face,
    level: u8,
    site: Site,
    nx: usize,
    dome_m: f64,
    uplift: f32,
    p: &FluvialParams,
) -> ArmOut {
    let face = site.face;
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    let surf = terrain(seed, face, level, site.oi, site.oj, nx, dome_m);
    let mut pipe = Pipe::from_surface(face, level, site.oi, site.oj, nx, router, &surf);
    if uplift != 0.0 {
        pipe.set_uniform_uplift(uplift);
    }
    let geom_ok = if pipe.geom.is_empty() {
        None
    } else {
        Some(identity_residual(&pipe.geom, &pipe.cell_area))
    };
    let t0 = timing::now();
    pipe.erode(p, sea);
    let secs = t0.elapsed().as_secs_f64();
    ArmOut {
        h: pipe.h,
        drainage: pipe.drainage,
        raw_acc: pipe.raw_acc,
        qnorm: pipe.qnorm,
        cell_area: pipe.cell_area,
        filled: pipe.filled,
        tree_resid_deg: pipe.tree_resid_deg,
        n_lsq_fallback: pipe.n_lsq_fallback,
        n_tree_fallback: pipe.n_tree_fallback,
        secs,
        geom_ok,
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_bitmatch(
    seed: u64,
    _face_ignored: Face,
    level: u8,
    site: Site,
    nx: usize,
    dome_m: f64,
    uplift: f32,
    p: &FluvialParams,
) -> bool {
    let face = site.face;
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    let surf = terrain(seed, face, level, site.oi, site.oj, nx, dome_m);
    let mut live = Fluvial::from_surface(seed, face, level, site.oi, site.oj, nx, &surf);
    if uplift != 0.0 {
        live.set_uniform_uplift(uplift);
    }
    live.erode(p);
    let mut mine = Pipe::from_surface(face, level, site.oi, site.oj, nx, LiveMfd, &surf);
    if uplift != 0.0 {
        mine.set_uniform_uplift(uplift);
    }
    mine.erode(p, sea);
    let ok = live.h == mine.h && live.drainage == mine.drainage;
    if !ok {
        let hd = live.h.iter().zip(&mine.h).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
        eprintln!("  BITMATCH FAIL at {}: max|Δh|={hd:.6e}", site.name);
    }
    ok
}

const ARMS: [Router; 9] =
    [UniformOld, LiveMfd, EdgeTrue, EdgeFull, EdgeTau, CoatTpfa, CoatGrad, GradFan, CoatMag];
const TAUS: [f32; 3] = [20.0, 50.0, 100.0];

fn main() {
    let p = FluvialParams::default();
    // NOT seed 0: the tectonic surface at seed 0 has no spectral content between
    // levels 14 and 19 at all (measured), so it would reproduce the bare-paraboloid
    // degeneracy this harness just found. Seed 1 has ~3.5 m sd of band-5 detail.
    let seed = 1u64;
    let face = Face::ZPos;
    let level = 19u8;
    let nx = 96usize;
    let n = 1u32 << level;
    let dome_m = 1200.0;
    let uplift = 0.02f32;

    let corner =
        Site { name: "CORNER (sheared Jacobian)", face, oi: nx as u32 + 4, oj: nx as u32 + 4 };
    let centre = Site {
        name: "FACE CENTRE (D4 acquittal control)",
        face,
        oi: n / 2 - nx as u32 / 2,
        oj: n / 2 - nx as u32 / 2,
    };
    // NULL PAIR: two footprints that are BOTH D4-symmetric (face centres on two
    // different faces). The cube-locked term vanishes at a face centre, so CUBE
    // computed over this pair must come back ~0 for every arm. Nothing in the
    // 2026-07-24 experiment established that the metric returns zero when there is
    // nothing to find — this is that control.
    let centre_x = Site {
        name: "FACE CENTRE, XPos (null-pair partner)",
        face: Face::XPos,
        oi: n / 2 - nx as u32 / 2,
        oj: n / 2 - nx as u32 / 2,
    };

    println!("######## FE(6c) PRICING — Coatleven flux-vector reconstruction on the tree ########");
    println!("Prices #obs-routing-curl-spiral FE(6)(c)+(d) against the FE(8) strawman.");
    println!("Predictions (pre-registered): msc/spike-router-fe6c/PREDICTIONS.md");
    println!("level={level} nx={nx} epochs={} uplift={uplift} dome={dome_m}m seed={seed}", p.epochs);

    // ---- P0: bit-match anchor ----
    println!("\n---- P0: LiveMfd arm == live erosion.rs, bit-for-bit, 3x, both sites ----");
    let mut all_ok = true;
    for trial in 0..3 {
        let a = validate_bitmatch(seed, face, level, corner, nx, dome_m, uplift, &p);
        let b = validate_bitmatch(seed, face, level, centre, nx, dome_m, uplift, &p);
        println!("  trial {trial}: corner={a} centre={b}");
        all_ok &= a && b;
    }
    println!(
        "  P0 {}",
        if all_ok { "PASS" } else { "FAIL — nothing below is about the live world" }
    );

    // ---- run every arm at every site, once ----
    let mut res: Vec<Vec<ArmOut>> = Vec::new();
    for site in [corner, centre] {
        let mut row = Vec::new();
        for &r in ARMS.iter() {
            row.push(run_arm(r, seed, face, level, site, nx, dome_m, uplift, &p));
        }
        res.push(row);
    }
    let sea = sea_level::derived_sea_level_m(seed) as f32;

    // ---- P1 gate: the Euclidean identity on spherical quads ----
    println!("\n---- P1 GATE: geometric identity |K|Id = Sum |s|(x_s-x_K) (x) n_hat ----");
    if let Some((worst, mean, warea)) = res[0][5].geom_ok {
        println!("  Frobenius residual ||M/|K|_planar - I||   worst={worst:.3e}  mean={mean:.3e}");
        println!(
            "  {}",
            if worst < 1e-9 {
                "PASS — the identity holds exactly on the tangent-plane quad"
            } else {
                "FAIL — normals / face-centres / lever arms are wrong. Fix before reading on."
            }
        );
        println!("  sphericity: max |A_spherical/A_planar - 1| = {warea:.3e}");
        println!("  (DERIVATION SS6 flagged the Euclidean identity on spherical cells as possibly");
        println!("   fatal at the coarse tier. At this level it is not the identity that limits");
        println!("   agreement -- it is the PRECISION of measure::cell_solid_angle. See RESULTS.)");
    }

    // ---- determinism ----
    println!("\n---- determinism: reconstruction arms bit-identical across 3 reruns ----");
    let mut det = true;
    for &arm in &[CoatTpfa, CoatGrad, CoatMag] {
        let base = run_arm(arm, seed, face, level, corner, nx, dome_m, uplift, &p).drainage;
        for t in 1..3 {
            let r = run_arm(arm, seed, face, level, corner, nx, dome_m, uplift, &p).drainage;
            if r != base {
                det = false;
                println!("  {arm:?} rerun {t}: DRIFT — nondeterministic, and that IS the finding");
            }
        }
    }
    println!("  determinism {}", if det { "PASS" } else { "FAIL" });

    // ---- per-site descriptive ----
    for (s, site) in [corner, centre].iter().enumerate() {
        println!("\n================ SITE: {} ================", site.name);
        let live = &res[s][1];
        let land = live.h.iter().filter(|&&h| h > sea).count();
        println!(
            "  land fraction {:.1}%   fill-raised cells (final epoch, live arm) {:.1}%",
            100.0 * land as f64 / (nx * nx) as f64,
            100.0 * live.filled.iter().filter(|&&b| b).count() as f64 / (nx * nx) as f64
        );
        println!("  {:<52} {:>7} {:>8} {:>8} {:>8}", "arm", "secs", "tau=20", "tau=50", "tau=100");
        for (a, &r) in ARMS.iter().enumerate() {
            let o = &res[s][a];
            let af: Vec<f64> = TAUS
                .iter()
                .map(|&t| {
                    axis_fraction(
                        &channel_mask(&o.drainage, &o.cell_area, &o.h, sea, t),
                        &o.h,
                        nx,
                        None,
                    )
                })
                .collect();
            println!(
                "  {:<52} {:>7.2} {:>8.4} {:>8.4} {:>8.4}",
                r.label(),
                o.secs,
                af[0],
                af[1],
                af[2]
            );
        }
        // M1/M2 vs the live baseline, plus vs EdgeTrue (the same-fan tree control).
        println!("  -- vs LiveMfd: M1 Jaccard(tau=50) / M2 log-drainage Spearman --");
        for (a, &r) in ARMS.iter().enumerate() {
            if a == 1 {
                continue;
            }
            let o = &res[s][a];
            let m1 = jaccard_disagreement(
                &channel_mask(&live.drainage, &live.cell_area, &live.h, sea, 50.0),
                &channel_mask(&o.drainage, &o.cell_area, &o.h, sea, 50.0),
            );
            let m2 = spearman_log(&live.drainage, &o.drainage, &live.h, sea);
            println!("     {:<52} M1={m1:.4} M2={m2:.3}", r.label());
        }
    }

    // ---- P4: FE(6d)'s own marginal, field-level ----
    {
        let a = &res[0][5]; // CoatTpfa
        let b = &res[0][6]; // CoatGrad
        println!("\n---- P4: FE(6d) marginal on the FIELD (corner) ----");
        println!(
            "  CoatTpfa vs CoatGrad: M2 log-drainage Spearman={:.3}  M1 Jaccard(tau=50)={:.4}",
            spearman_log(&a.drainage, &b.drainage, &a.h, sea),
            jaccard_disagreement(
                &channel_mask(&a.drainage, &a.cell_area, &a.h, sea, 50.0),
                &channel_mask(&b.drainage, &b.cell_area, &b.h, sea, 50.0)
            )
        );
        println!(
            "  LSQ downhill-fallback events, FINAL EPOCH ONLY, corner = {} of {} cells   (the corrected",
            b.n_lsq_fallback, nx * nx
        );
        println!("  gradient sent nothing through any downhill face and the two-point weights took over)");
    }

    // ---- P5: the magnitude the reconstruction returns ----
    {
        let o = &res[0][6]; // CoatGrad — reconstructs, but consumes raw acc
        let mut ratio: Vec<f64> = (0..nx * nx)
            .filter(|&i| o.h[i] > sea && o.raw_acc[i] > 0.0 && o.qnorm[i] > 0.0)
            .map(|i| {
                (o.qnorm[i] as f64 * (o.cell_area[i] as f64).sqrt()) / o.raw_acc[i] as f64
            })
            .collect();
        let nrat = ratio.len();
        let med = median(&mut ratio);
        let p10 = if nrat > 0 { ratio[nrat / 10] } else { f64::NAN };
        let p90 = if nrat > 0 { ratio[9 * nrat / 10] } else { f64::NAN };
        println!("\n---- P5: ||Q||*sqrt(A) / raw accumulation, land cells (corner) ----");
        println!("  n={nrat}  p10={p10:.3}  median={med:.3}  p90={p90:.3}");
    }

    // ---- the tree's irreducible projection residual ----
    {
        for (s, site) in [corner, centre].iter().enumerate() {
            let o = &res[s][6];
            let mut v: Vec<f64> =
                o.tree_resid_deg.iter().filter(|d| d.is_finite()).map(|&d| d as f64).collect();
            let nv = v.len();
            let med = median(&mut v);
            let p90 = if nv > 0 { v[9 * nv / 10] } else { f64::NAN };
            println!(
                "  tree projection residual angle(Q-hat, chosen receiver) @{:<32} n={nv} median={med:.2}deg p90={p90:.2}deg  Q-hat-degenerate fallbacks={}",
                site.name, o.n_tree_fallback
            );
        }
    }

    // ---- THE MEASUREMENT: null-test differential, with and without the carve-out ----
    for &carve in &[false, true] {
        println!(
            "\n======= NULL-TEST DIFFERENTIAL{} =======",
            if carve { " — s_K=0 CARVE-OUT (fill-raised cells excluded)" } else { "" }
        );
        println!("  CUBE = (arm-live axis-frac at CORNER) - (arm-live at FACE CENTRE)");
        println!("  {:<52} {:>9} {:>9} {:>9}", "arm", "tau=20", "tau=50", "tau=100");
        let af = |s: usize, a: usize, t: f32| -> f64 {
            let o = &res[s][a];
            axis_fraction(
                &channel_mask(&o.drainage, &o.cell_area, &o.h, sea, t),
                &o.h,
                nx,
                if carve { Some(&o.filled) } else { None },
            )
        };
        for (a, &r) in ARMS.iter().enumerate() {
            if a == 1 {
                continue;
            }
            let c: Vec<f64> = TAUS
                .iter()
                .map(|&t| (af(0, a, t) - af(0, 1, t)) - (af(1, a, t) - af(1, 1, t)))
                .collect();
            println!("  {:<52} {:>+9.4} {:>+9.4} {:>+9.4}", r.label(), c[0], c[1], c[2]);
        }
        // Same-fan tree comparison: everything measured against EdgeTrue instead of live,
        // which removes the fan diagonal-kill from both sides and leaves the TREE.
        println!("  -- same-fan (baseline = EdgeTrue): isolates the TREE treatment alone --");
        for &a in &[3usize, 5, 6, 7, 8] {
            let c: Vec<f64> = TAUS
                .iter()
                .map(|&t| (af(0, a, t) - af(0, 2, t)) - (af(1, a, t) - af(1, 2, t)))
                .collect();
            println!("  {:<52} {:>+9.4} {:>+9.4} {:>+9.4}", ARMS[a].label(), c[0], c[1], c[2]);
        }
    }

    // ---- THE NOISE FLOOR (the thing nobody had measured) ----
    //
    // CUBE is a difference of differences of a FRACTION over a few thousand channel
    // cells. Nothing in the 2026-07-24 experiment established how much of a given
    // CUBE value is terrain realisation rather than router. One seed cannot say.
    // So: re-run the whole ladder over independent seeds and report mean ± sd.
    // A per-arm |mean| that is not several sd above zero is not a measurement.
    //
    // UniformOld is the SCREAM CONTROL: it carries the retired length bias, which
    // the prior experiment measured at CUBE +0.10..+0.20. If it does not separate
    // from zero here, the instrument is not reading and nothing else in the table
    // means anything.
    let seeds: Vec<u64> = (1..=8u64).collect();
    println!("\n\n######## SEED SWEEP — CUBE mean +/- sd over {} seeds ########", seeds.len());
    println!("  (same corner/centre footprints, same level/epochs; only the terrain realisation moves)");
    let mut cube_by_arm: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); TAUS.len()]; ARMS.len()];
    let mut cube_vs_edge: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); TAUS.len()]; ARMS.len()];
    for &sd in &seeds {
        let mut rr: Vec<Vec<ArmOut>> = Vec::new();
        for site in [corner, centre] {
            let mut row = Vec::new();
            for &r in ARMS.iter() {
                row.push(run_arm(r, sd, face, level, site, nx, dome_m, uplift, &p));
            }
            rr.push(row);
        }
        let seas = sea_level::derived_sea_level_m(sd) as f32;
        let af = |s: usize, a: usize, t: f32| -> f64 {
            let o = &rr[s][a];
            axis_fraction(&channel_mask(&o.drainage, &o.cell_area, &o.h, seas, t), &o.h, nx, None)
        };
        for a in 0..ARMS.len() {
            for (ti, &t) in TAUS.iter().enumerate() {
                cube_by_arm[a][ti].push((af(0, a, t) - af(0, 1, t)) - (af(1, a, t) - af(1, 1, t)));
                cube_vs_edge[a][ti].push((af(0, a, t) - af(0, 2, t)) - (af(1, a, t) - af(1, 2, t)));
            }
        }
        print!("  seed {sd} done. ");
    }
    println!();
    let stat = |v: &[f64]| -> (f64, f64) {
        let n = v.len() as f64;
        let m = v.iter().sum::<f64>() / n;
        let sd = (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0)).sqrt();
        (m, sd)
    };
    for (name, tab, skip) in
        [("baseline = LiveMfd", &cube_by_arm, 1usize), ("baseline = EdgeTrue (tree alone)", &cube_vs_edge, 2usize)]
    {
        println!("\n  ---- CUBE, {name} ----");
        println!("  {:<52} {:>18} {:>18} {:>18}", "arm", "tau=20", "tau=50", "tau=100");
        for (a, &r) in ARMS.iter().enumerate() {
            if a == skip || a == 1 && skip == 2 {
                continue;
            }
            let c: Vec<(f64, f64)> = (0..TAUS.len()).map(|ti| stat(&tab[a][ti])).collect();
            println!(
                "  {:<52} {:>+8.4}+/-{:<7.4} {:>+8.4}+/-{:<7.4} {:>+8.4}+/-{:<7.4}",
                r.label(), c[0].0, c[0].1, c[1].0, c[1].1, c[2].0, c[2].1
            );
        }
        println!("  {:<52} {:>18} {:>18} {:>18}", "  |mean| / sem  (>2 = separated from zero)", "", "", "");
        for (a, &r) in ARMS.iter().enumerate() {
            if a == skip || a == 1 && skip == 2 {
                continue;
            }
            let n = seeds.len() as f64;
            let z: Vec<f64> = (0..TAUS.len())
                .map(|ti| {
                    let (m, s) = stat(&tab[a][ti]);
                    m.abs() / (s / n.sqrt()).max(1e-12)
                })
                .collect();
            println!("  {:<52} {:>18.2} {:>18.2} {:>18.2}", r.label(), z[0], z[1], z[2]);
        }
    }

    // ---- THE METRIC'S OWN NULL CONTROL ----
    println!("\n\n######## NULL-PAIR CONTROL — CUBE over TWO D4-symmetric footprints ########");
    println!("  Both sites are face centres (ZPos and XPos). The cube-locked term vanishes under");
    println!("  D4 symmetry, so every arm's CUBE must come back consistent with zero. Any arm that");
    println!("  does not is measuring terrain realisation, not cube-lock -- and would convict the");
    println!("  metric rather than the router.");
    let mut null_tab: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); TAUS.len()]; ARMS.len()];
    for &sd in &seeds {
        let mut rr: Vec<Vec<ArmOut>> = Vec::new();
        for site in [centre, centre_x] {
            let mut row = Vec::new();
            for &r in ARMS.iter() {
                row.push(run_arm(r, sd, face, level, site, nx, dome_m, uplift, &p));
            }
            rr.push(row);
        }
        let seas = sea_level::derived_sea_level_m(sd) as f32;
        let af = |s: usize, a: usize, t: f32| -> f64 {
            let o = &rr[s][a];
            axis_fraction(&channel_mask(&o.drainage, &o.cell_area, &o.h, seas, t), &o.h, nx, None)
        };
        for a in 0..ARMS.len() {
            for (ti, &t) in TAUS.iter().enumerate() {
                null_tab[a][ti].push((af(0, a, t) - af(0, 1, t)) - (af(1, a, t) - af(1, 1, t)));
            }
        }
    }
    println!("  {:<52} {:>18} {:>18} {:>18}", "arm", "tau=20", "tau=50", "tau=100");
    for (a, &r) in ARMS.iter().enumerate() {
        if a == 1 {
            continue;
        }
        let c: Vec<(f64, f64)> = (0..TAUS.len()).map(|ti| stat(&null_tab[a][ti])).collect();
        println!(
            "  {:<52} {:>+8.4}+/-{:<7.4} {:>+8.4}+/-{:<7.4} {:>+8.4}+/-{:<7.4}",
            r.label(), c[0].0, c[0].1, c[1].0, c[1].1, c[2].0, c[2].1
        );
    }

    // ---- THE SUMMARY THAT MATTERS: signal against the metric's own floor ----
    //
    // RMS of the mean CUBE across the three thresholds, for the CUBE-carrying pair
    // (corner vs face centre) and for the NULL pair (two face centres). The null
    // column is what this metric returns when there is nothing to find; an arm
    // whose signal column is not clearly above its own null column has no measured
    // cube-locked landscape consequence.
    println!("\n\n######## SUMMARY — RMS mean-CUBE across thresholds, against the null floor ########");
    println!("  {:<52} {:>10} {:>10} {:>8}", "arm", "RMS CUBE", "RMS null", "ratio");
    let rms = |t: &Vec<Vec<Vec<f64>>>, a: usize| -> f64 {
        let v: Vec<f64> = (0..TAUS.len()).map(|ti| stat(&t[a][ti]).0).collect();
        (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt()
    };
    let mut rows: Vec<(f64, String)> = Vec::new();
    for (a, &r) in ARMS.iter().enumerate() {
        if a == 1 {
            continue;
        }
        let sig = rms(&cube_by_arm, a);
        let nul = rms(&null_tab, a);
        rows.push((sig, format!("  {:<52} {sig:>10.4} {nul:>10.4} {:>8.1}", r.label(), sig / nul.max(1e-9))));
    }
    rows.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (_, line) in &rows {
        println!("{line}");
    }
    println!("  (sorted most cube-safe first; LiveMfd is the baseline and is 0 by construction)");
}
