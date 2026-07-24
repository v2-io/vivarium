//! The multiresolution transform: an **area-weighted lifting scheme** on the
//! cube-sphere quadtree.
//!
//! ## What this is, stated exactly
//!
//! One coarsening step takes a parent cell $P$ with children $k \in \{0,1,2,3\}$
//! of areas $a_k$, $A_P = \sum_k a_k$:
//!
//! **Analysis (fine → coarse + detail)**
//! 1. **Update** — the scaling coefficient is the *area-weighted* mean:
//!    $$c_P = \frac{\sum_k a_k h_k}{A_P}$$
//!    ⇒ $A_P\,c_P = \sum_k a_k h_k$ **exactly**: the cell integral is carried up untouched.
//! 2. **Predict** — an arbitrary operator $\hat h_k = \mathcal{P}_k(c_{\text{nbhd}})$ guesses the
//!    children from the *coarse* field (Haar: $\hat h_k = c_P$. Bilinear: interpolate the
//!    coarse neighbourhood. Any order at all.)
//! 3. **Detail** — the residual, with the predictor's own mean error projected out:
//!    $$t_k = h_k - \hat h_k - (c_P - \hat c), \qquad \hat c = \frac{\sum_k a_k \hat h_k}{A_P}$$
//!    which satisfies $\sum_k a_k t_k = 0$ identically. **Only $t_0, t_1, t_2$ are stored**;
//!    $t_3$ is *recovered from that constraint*, so the transform is critically sampled
//!    (4 values in → 1 coarse + 3 details out; $\text{nx}^2$ in, $\text{nx}^2$ out).
//!
//! **Synthesis (coarse + detail → fine)**
//! $$h_k = \hat h_k + (c_P - \hat c) + t_k$$
//!
//! ## The three properties, and which are structural vs contingent
//!
//! - **Perfect reconstruction is BY CONSTRUCTION, for ANY predictor.** Synthesis is
//!   analysis run backwards; no property of $\mathcal{P}$ is used. This is the whole reason
//!   the lifting scheme exists, and it is what lets us have a *good* predictor on an
//!   *irregular* grid without proving anything about it. (`probe_exactness` measures it
//!   anyway.)
//! - **Conservation is BY CONSTRUCTION, for ANY predictor, and — the sharp one —
//!   SURVIVES ARBITRARY LOSSY COMPRESSION.** Zero out a parent's whole detail triple and
//!   $\sum_k a_k h_k = A_P c_P$ *still holds exactly*, because the $(c_P - \hat c)$ update term
//!   carries the mean and the $t_k$ carry zero mean by construction. So thresholded detail
//!   costs you *shape*, never *mass*. This is the property that makes the store lossy-
//!   compressible without a conservation audit. (`probe_compression` measures it.)
//! - **Compression is CONTINGENT on the predictor.** Haar ($\mathcal{P} = c_P$, order 0) has
//!   large details even on a planar ramp, so it compresses badly. A bilinear predictor
//!   reproduces linear fields exactly, so its details vanish on smooth ground. **This is
//!   where the predictor's quality lives — and nowhere else.**
//!
//! > **⇒ The clean separation the lifting scheme buys: PREDICTOR QUALITY ⊥ CONSERVATION.**
//! > A better predictor buys smaller details (compression, refinement criteria, smaller
//! > seams). It cannot buy or cost you conservation, which the update step already owns.
//! > *That* is why the unequal areas are not fatal: the areas enter only the update step,
//! > which is a weighted mean — and a weighted mean does not care that the weights differ.
//!
//! ## Where the areas actually bite (declared, not hidden)
//!
//! The bilinear predictor interpolates coarse *values* at child *index* centres. But a
//! coarse value is an area-weighted **cell average**, whose natural sample point is the
//! area centroid, not the index centre. On a grid with 1.4× area spread those differ by
//! $O(\Delta^2)$. **That is the point-sample-vs-cell-average fork** (`discretisation-and-
//! information.md` §3.1) appearing inside the predictor. It costs *prediction accuracy*
//! (slightly larger details) and **cannot** cost conservation. Declared; not corrected here.

use vivarium_world::sphere::Face;

use crate::area::cell_area_m2;

/// A square block of a cube-sphere face at one level: row-major `nx × nx` values
/// with the integer origin `(oi, oj)` at that level.
#[derive(Clone)]
pub struct Grid {
    pub face: Face,
    pub level: u8,
    pub oi: u64,
    pub oj: u64,
    pub nx: usize,
    pub v: Vec<f64>,
}

impl Grid {
    pub fn new(face: Face, level: u8, oi: u64, oj: u64, nx: usize, v: Vec<f64>) -> Self {
        assert_eq!(v.len(), nx * nx, "grid data must be nx × nx");
        Grid { face, level, oi, oj, nx, v }
    }

    #[inline]
    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.v[y * self.nx + x]
    }

    /// Clamped access — the halo policy at a tile edge. **This is an approximation,
    /// and it is declared**: a real store would pull the neighbouring coarse cells.
    /// It affects the *predictor* (hence detail size) only, never conservation.
    #[inline]
    pub fn at_clamped(&self, x: isize, y: isize) -> f64 {
        let n = self.nx as isize;
        self.at(x.clamp(0, n - 1) as usize, y.clamp(0, n - 1) as usize)
    }

    /// Exact spherical areas (m²) of this grid's cells.
    pub fn areas(&self, radius_m: f64) -> Vec<f64> {
        let mut a = Vec::with_capacity(self.nx * self.nx);
        for y in 0..self.nx {
            for x in 0..self.nx {
                a.push(cell_area_m2(self.face, self.oi + x as u64, self.oj + y as u64, self.level, radius_m));
            }
        }
        a
    }

    /// The conserved integral $\sum_i a_i h_i$ — the quantity that must telescope.
    pub fn integral(&self, areas: &[f64]) -> f64 {
        self.v.iter().zip(areas).map(|(h, a)| h * a).sum()
    }
}

/// The predict operator. **Perfect reconstruction and conservation hold for any
/// implementation of this trait** — that is the point.
pub trait Predictor {
    fn name(&self) -> &'static str;
    /// Predict the four children of coarse cell `(px, py)`, in child order
    /// `[ (0,0), (1,0), (0,1), (1,1) ]` (local `dx, dy` within the parent).
    fn predict(&self, coarse: &Grid, px: usize, py: usize) -> [f64; 4];
}

/// Order-0: every child inherits the parent. **This is exactly what `mean-pin`
/// assumes** — and it is the Haar scaling function.
pub struct Haar;

impl Predictor for Haar {
    fn name(&self) -> &'static str {
        "haar (order 0: child := parent)"
    }
    fn predict(&self, coarse: &Grid, px: usize, py: usize) -> [f64; 4] {
        let c = coarse.at(px, py);
        [c, c, c, c]
    }
}

/// Order-1: bilinear interpolation of the coarse field at each child's centre.
/// Reproduces linear fields exactly ⇒ **details vanish on planar ground**.
pub struct Bilinear;

impl Predictor for Bilinear {
    fn name(&self) -> &'static str {
        "bilinear (order 1: linear-reproducing)"
    }
    fn predict(&self, coarse: &Grid, px: usize, py: usize) -> [f64; 4] {
        let mut out = [0.0; 4];
        for (k, (dx, dy)) in [(0usize, 0usize), (1, 0), (0, 1), (1, 1)].into_iter().enumerate() {
            // Child (2px+dx) centre, expressed in coarse cell-centre coordinates.
            let gx = px as f64 + if dx == 0 { -0.25 } else { 0.25 };
            let gy = py as f64 + if dy == 0 { -0.25 } else { 0.25 };
            let (x0, y0) = (gx.floor(), gy.floor());
            let (fx, fy) = (gx - x0, gy - y0);
            let (x0, y0) = (x0 as isize, y0 as isize);
            out[k] = coarse.at_clamped(x0, y0) * (1.0 - fx) * (1.0 - fy)
                + coarse.at_clamped(x0 + 1, y0) * fx * (1.0 - fy)
                + coarse.at_clamped(x0, y0 + 1) * (1.0 - fx) * fy
                + coarse.at_clamped(x0 + 1, y0 + 1) * fx * fy;
        }
        out
    }
}

/// The stored details for one coarsening step: 3 coefficients per parent.
/// (The 4th is recovered from $\sum_k a_k t_k = 0$.)
#[derive(Clone)]
pub struct DetailLevel {
    /// Parent-grid size (`coarse.nx`).
    pub pn: usize,
    /// `pn * pn * 3`, row-major by parent then coefficient.
    pub t: Vec<f64>,
}

impl DetailLevel {
    #[inline]
    pub fn get(&self, px: usize, py: usize) -> [f64; 3] {
        let o = (py * self.pn + px) * 3;
        [self.t[o], self.t[o + 1], self.t[o + 2]]
    }
    #[inline]
    pub fn set(&mut self, px: usize, py: usize, t: [f64; 3]) {
        let o = (py * self.pn + px) * 3;
        self.t[o] = t[0];
        self.t[o + 1] = t[1];
        self.t[o + 2] = t[2];
    }
}

/// Child `k` of parent `(px, py)` in the fine grid's local coords.
#[inline]
fn child_xy(px: usize, py: usize, k: usize) -> (usize, usize) {
    let (dx, dy) = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)][k];
    (2 * px + dx, 2 * py + dy)
}

/// One coarsening step. Returns the coarse grid, its areas, and the detail level.
/// `fine_areas` must be the exact spherical areas of `fine`.
///
/// The coarse cell's area is taken as **the sum of its children's** — so the pyramid
/// telescopes by construction. `probe_area_additivity` separately measures whether the
/// *geometry* agrees (it does, to ~1e-16 — the children exactly tile the parent), which
/// is what makes that choice honest rather than a fudge.
pub fn analyze(
    fine: &Grid,
    fine_areas: &[f64],
    pred: &dyn Predictor,
) -> (Grid, Vec<f64>, DetailLevel) {
    assert!(fine.nx % 2 == 0, "nx must be even to coarsen");
    let pn = fine.nx / 2;
    let (coi, coj) = (fine.oi / 2, fine.oj / 2);
    assert_eq!(fine.oi % 2, 0, "origin must be quadtree-aligned");
    assert_eq!(fine.oj % 2, 0, "origin must be quadtree-aligned");

    // --- Step 1: UPDATE. The area-weighted mean; the integral rides up untouched.
    let mut cv = vec![0.0f64; pn * pn];
    let mut ca = vec![0.0f64; pn * pn];
    for py in 0..pn {
        for px in 0..pn {
            let (mut num, mut den) = (0.0, 0.0);
            for k in 0..4 {
                let (x, y) = child_xy(px, py, k);
                let a = fine_areas[y * fine.nx + x];
                num += a * fine.at(x, y);
                den += a;
            }
            cv[py * pn + px] = num / den;
            ca[py * pn + px] = den;
        }
    }
    let coarse = Grid::new(fine.face, fine.level - 1, coi, coj, pn, cv);

    // --- Steps 2 & 3: PREDICT, then DETAIL (mean projected out).
    let mut det = DetailLevel { pn, t: vec![0.0; pn * pn * 3] };
    for py in 0..pn {
        for px in 0..pn {
            let hhat = pred.predict(&coarse, px, py);
            let a_p = ca[py * pn + px];
            let c_p = coarse.at(px, py);
            let mut chat = 0.0;
            let mut a = [0.0f64; 4];
            for k in 0..4 {
                let (x, y) = child_xy(px, py, k);
                a[k] = fine_areas[y * fine.nx + x];
                chat += a[k] * hhat[k];
            }
            chat /= a_p;
            let upd = c_p - chat; // the lifting UPDATE term: the predictor's own mean error
            let mut t = [0.0f64; 3];
            for k in 0..3 {
                let (x, y) = child_xy(px, py, k);
                t[k] = fine.at(x, y) - hhat[k] - upd;
            }
            det.set(px, py, t);
        }
    }
    (coarse, ca, det)
}

/// One refinement step — the exact inverse of [`analyze`] for any predictor.
pub fn synthesize(
    coarse: &Grid,
    coarse_areas: &[f64],
    fine_areas: &[f64],
    det: &DetailLevel,
    pred: &dyn Predictor,
) -> Grid {
    let pn = coarse.nx;
    let nx = pn * 2;
    let mut v = vec![0.0f64; nx * nx];
    for py in 0..pn {
        for px in 0..pn {
            let hhat = pred.predict(coarse, px, py);
            let a_p = coarse_areas[py * pn + px];
            let c_p = coarse.at(px, py);
            let mut chat = 0.0;
            let mut a = [0.0f64; 4];
            for k in 0..4 {
                let (x, y) = child_xy(px, py, k);
                a[k] = fine_areas[y * nx + x];
                chat += a[k] * hhat[k];
            }
            chat /= a_p;
            let upd = c_p - chat;
            let t3 = det.get(px, py);
            // The 4th detail, recovered from Σ aₖ tₖ = 0 — this is what makes the
            // transform critically sampled AND exactly conservative under thresholding.
            let t = [t3[0], t3[1], t3[2], -(a[0] * t3[0] + a[1] * t3[1] + a[2] * t3[2]) / a[3]];
            for k in 0..4 {
                let (x, y) = child_xy(px, py, k);
                v[y * nx + x] = hhat[k] + upd + t[k];
            }
        }
    }
    Grid::new(coarse.face, coarse.level + 1, coarse.oi * 2, coarse.oj * 2, nx, v)
}

/// The full pyramid: the store's actual shape.
///
/// `root` (a `nx >> depth` square of scaling coefficients) + `details[0..depth]`
/// (finest-first: `details[0]` refines `level - 1 → level`).
pub struct Pyramid {
    pub root: Grid,
    pub root_areas: Vec<f64>,
    /// `details[d]` takes level `root.level + d` → `root.level + d + 1`.
    pub details: Vec<DetailLevel>,
    /// `areas[d]` = exact areas of the grid at level `root.level + d`. `areas[0] == root_areas`.
    pub areas: Vec<Vec<f64>>,
}

impl Pyramid {
    /// Total stored floats — must equal the leaf count (critically sampled).
    pub fn stored_floats(&self) -> usize {
        self.root.nx * self.root.nx + self.details.iter().map(|d| d.pn * d.pn * 3).sum::<usize>()
    }
}

/// Decompose a fine grid into the pyramid.
pub fn decompose(fine: &Grid, radius_m: f64, depth: usize, pred: &dyn Predictor) -> Pyramid {
    let mut cur = fine.clone();
    let mut cur_areas = fine.areas(radius_m);
    let mut dets = Vec::new();
    let mut areas_by_level = vec![cur_areas.clone()];
    for _ in 0..depth {
        let (c, ca, d) = analyze(&cur, &cur_areas, pred);
        dets.push(d);
        cur = c;
        cur_areas = ca;
        areas_by_level.push(cur_areas.clone());
    }
    dets.reverse(); // coarsest-first: details[0] refines root → root+1
    areas_by_level.reverse();
    Pyramid { root: cur, root_areas: areas_by_level[0].clone(), details: dets, areas: areas_by_level }
}

/// Rebuild the fine grid from the pyramid.
pub fn reconstruct(p: &Pyramid, pred: &dyn Predictor) -> Grid {
    let mut cur = p.root.clone();
    for (d, det) in p.details.iter().enumerate() {
        let ca = &p.areas[d];
        let fa = &p.areas[d + 1];
        cur = synthesize(&cur, ca, fa, det, pred);
    }
    cur
}
