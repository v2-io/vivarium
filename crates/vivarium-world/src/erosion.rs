//! Erosion tier — ported onto the frame ([`ref/erosion-port/NOTES.md`]), as a
//! fidelity-ladder tier on the Cartesian [`Patch`] substrate.
//!
//! **First increment: hillslope diffusion** — the local diffusive term of
//! Davy–Lague — a 5-point Laplacian stencil on a `Patch<f32>` height field (metres).
//! It relaxes slopes (creep / soil transport) and is the simplest *real* erosion on
//! the new substrate, and the proof that the Patch stencil path works end to end.
//!
//! Next increments (`ref/erosion-port/NOTES.md`): stream-power fluvial incision
//! (needs non-local flow accumulation), per-material erodibility (differential
//! erosion), and the multirate water coupling (§4) that lets erosion stay *on*
//! during settling. Strict mass conservation depends on the boundary/halo policy
//! (flux-form + a no-flux or supplied halo); the loader owns halo fill.

use crate::chunk::Patch;
use crate::gen::{self, SEA_LEVEL_M};
use crate::sphere::{CellId, Face};

/// One explicit hillslope-diffusion step: `h' = h + k·∇²h` (5-point Laplacian).
/// `k` is the per-step diffusivity — keep `k ≤ 0.25` for explicit stability. Reads
/// `src` (interior + halo neighbours), writes the interior of `dst`.
pub fn diffuse_step(src: &Patch<f32>, dst: &mut Patch<f32>, k: f32) {
    for y in 0..src.w as isize {
        for x in 0..src.w as isize {
            let c = src.get(x, y);
            let lap = src.get(x - 1, y) + src.get(x + 1, y) + src.get(x, y - 1) + src.get(x, y + 1) - 4.0 * c;
            dst.set(x, y, c + k * lap);
        }
    }
}

/// `iters` diffusion steps, ping-ponging `h` and `scratch`; result ends in `h`.
/// The halo is *not* refreshed between steps (fixed-boundary) — a self-contained
/// region relaxes toward its halo values; the full pipeline refreshes halos per
/// step via the loader.
pub fn diffuse(h: &mut Patch<f32>, scratch: &mut Patch<f32>, iters: u32, k: f32) {
    for _ in 0..iters {
        diffuse_step(h, scratch, k);
        std::mem::swap(h, scratch);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Face;

    // origin far from 0 so the halo (origin-1 …) is in-range and `fill` populates it.
    fn patch(w: usize) -> Patch<f32> {
        Patch::new(Face::ZPos, 12, 100, 100, w, 1)
    }

    #[test]
    fn flat_stays_flat() {
        let mut a = patch(8);
        a.fill(|_, _| 5.0);
        let mut b = patch(8);
        b.fill(|_, _| 5.0);
        diffuse_step(&a, &mut b, 0.2);
        for y in 0..8 {
            for x in 0..8 {
                assert!((b.get(x, y) - 5.0).abs() < 1e-5, "flat drifted at ({x},{y})");
            }
        }
    }

    #[test]
    fn spike_spreads() {
        let mut a = patch(8);
        a.fill(|_, _| 0.0);
        a.set(4, 4, 10.0);
        let mut b = patch(8);
        diffuse_step(&a, &mut b, 0.2);
        assert!(b.get(4, 4) < 10.0, "centre didn't fall");
        assert!(b.get(3, 4) > 0.0 && b.get(5, 4) > 0.0, "neighbours didn't rise");
        // symmetric spread
        assert!((b.get(3, 4) - b.get(5, 4)).abs() < 1e-6);
        assert!((b.get(4, 3) - b.get(4, 5)).abs() < 1e-6);
    }

    #[test]
    fn smooths_and_is_stable() {
        let mut a = patch(16);
        a.fill(|i, j| if (i + j) % 2 == 0 { 1.0 } else { -1.0 }); // checkerboard
        let range0 = 2.0f32;
        let mut b = patch(16);
        diffuse(&mut a, &mut b, 30, 0.2);
        let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
        for y in 0..16 {
            for x in 0..16 {
                let v = a.get(x, y);
                assert!(v.is_finite(), "blew up");
                lo = lo.min(v);
                hi = hi.max(v);
            }
        }
        assert!(hi - lo < range0, "did not smooth: range {}", hi - lo);
    }
}

// ---- The fluvial pipeline: a faithful port of vivarium-core's proven geo.rs ----
//
// Per epoch: (1) uplift non-outlets, (2) Priority-Flood depression filling with an
// ε-gradient (Barnes, Lehman & Mulla 2014; deterministic tie-breaks by insertion
// seq, never float chance), (3) D8 steepest-descent receivers (the tree the
// implicit solve needs), (4) MFD drainage-area accumulation (Quinn et al., p=1.1 —
// dissolves D8's grid-locked ribs; this is what decides WHERE channels form),
// (5) implicit stream-power incision, n=1 (Whipple & Tucker 1999 à la Braun &
// Willett 2013 — exact, unconditionally stable, bit-deterministic in fixed order),
// (6) Davy & Lague 2009 deposition D = G·Qs/A routed down the D8 tree (grades
// valley floors without filling upland channels), (7) talus relaxation (Musgrave
// 1989, snapshot+batch). Elevation-sorted order stands in for Braun & Willett's
// O(n) stack — same result, less to get wrong (core's own reasoning; the O(n)
// swap remains available when n log n bites).
//
// Frame-native: heights in METRES, sea level = gen::SEA_LEVEL_M as a real outlet
// set (rivers run to the coast, not just the grid edge), seeded from the
// band-limited two-band prior at the sim level's own Nyquist. Per-material
// erodibility (Material::erodibility / incision_threshold) is the flagged next
// hook — uniform hardness in this first increment.

/// Parameters for a fluvial-erosion run over a region. Metres and epochs; the
/// defaults are tuned for visible dendritic dissection of the two-band prior at
/// ~19 m cells (L19) in under a hundred epochs — a crude-but-honest first rung.
#[derive(Clone, Debug)]
pub struct FluvialParams {
    /// Erodibility `K·dt` lump in `E = K·Aᵐ·Sⁿ` (n = 1, A in m²).
    pub k_dt: f32,
    /// Drainage-area exponent `m`.
    pub m: f32,
    /// Uplift per epoch (m). Zero = decaying landscape (erode the prior as-is).
    pub uplift_m: f32,
    /// Davy–Lague deposition efficiency `G` (0 = pure detachment).
    pub deposition: f32,
    /// Talus repose slope (rise/run). Slopes beyond this slump (half-excess/epoch).
    pub max_slope: f32,
    pub epochs: u32,
}

impl Default for FluvialParams {
    fn default() -> Self {
        Self { k_dt: 0.02, m: 0.5, uplift_m: 0.0, deposition: 1.0, max_slope: 0.8, epochs: 80 }
    }
}

/// A square fluvial simulation field over one face region — the frame's port of
/// core's `Heightfield`. Heights in metres above the bedrock datum.
pub struct Fluvial {
    pub nx: usize,
    pub cell_m: f32,
    pub h: Vec<f32>,
    /// MFD drainage area (m²) from the last epoch — the dendritic-ness instrument.
    pub drainage: Vec<f32>,
    /// Where this field sits (face cells at `level` from `origin`) — identity for
    /// the differential-uplift field and for wrapping back into an ErodedRegion.
    pub face: Face,
    pub level: u8,
    pub origin: (u32, u32),
    /// Cached per-cell uplift weights (built on first uplifting epoch).
    uplift_w: Option<Vec<f32>>,
}

const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

impl Fluvial {
    /// Seed from the band-limited prior over `nx × nx` cells of `face` at `level`
    /// starting at `(oi, oj)` — the honest initial condition (no imposed shapes).
    pub fn from_prior(face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Self {
        Self::from_surface(face, level, oi, oj, nx, |c| gen::surface_prior_m(c, c.level()))
    }

    /// Seed from an arbitrary surface function — how a FINE tier is seeded from
    /// the coarse tiers below it (the §7.2 downscaling seam: the fine sim's
    /// initial condition is the downscaled coarse end-state + detail increment).
    pub fn from_surface(face: Face, level: u8, oi: u32, oj: u32, nx: usize, surf: impl Fn(CellId) -> f64) -> Self {
        let cell_m = crate::sample::cell_size_m(level, crate::planet::Planet::EARTH.radius_m) as f32;
        let mut h = vec![0.0f32; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let cell = CellId::from_face_ij(face, oi + x as u32, oj + y as u32, level);
                h[y * nx + x] = surf(cell) as f32;
            }
        }
        Self { nx, cell_m, h, drainage: vec![0.0; nx * nx], face, level, origin: (oi, oj), uplift_w: None }
    }

    /// Resume a simulation over an existing eroded field (e.g. the startup tier),
    /// so the live loop can keep running epochs without redoing the initial work.
    pub fn from_region(r: &ErodedRegion) -> Self {
        let cell_m = crate::sample::cell_size_m(r.level, crate::planet::Planet::EARTH.radius_m) as f32;
        Self { nx: r.nx, cell_m, h: r.h.clone(), drainage: vec![0.0; r.nx * r.nx], face: r.face, level: r.level, origin: (r.oi, r.oj), uplift_w: None }
    }

    /// Snapshot into a sampleable region.
    pub fn to_region(&self) -> ErodedRegion {
        ErodedRegion { face: self.face, level: self.level, oi: self.origin.0, oj: self.origin.1, nx: self.nx, h: self.h.clone() }
    }

    #[inline]
    fn is_edge(nx: usize, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == nx - 1 || y == nx - 1
    }

    /// Outlets: the grid edge plus every cell at or below sea level — the coast
    /// the rivers run to. Recomputed per epoch (erosion moves the waterline).
    fn outlets(&self) -> Vec<bool> {
        let nx = self.nx;
        let sea = SEA_LEVEL_M as f32;
        let mut out = vec![false; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                out[i] = Self::is_edge(nx, x, y) || self.h[i] <= sea;
            }
        }
        out
    }

    /// Priority-Flood depression filling with an ε-gradient across flats.
    /// Deterministic: min-heap keyed (elevation, insertion seq) — ties break by
    /// integer seq, never float chance.
    fn fill_depressions(&mut self, outlets: &[bool]) {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;
        let nx = self.nx;
        const EPS: f32 = 1e-3; // m; tiny vs. relief, enough to orient flats

        struct Cell {
            elev: f32,
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

        let mut closed = vec![false; nx * nx];
        let mut heap = BinaryHeap::new();
        let mut seq = 0u64;
        for (i, &is_out) in outlets.iter().enumerate() {
            if is_out {
                closed[i] = true;
                heap.push(Cell { elev: self.h[i], seq, i });
                seq += 1;
            }
        }
        while let Some(Cell { elev, i, .. }) = heap.pop() {
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
                self.h[j] = self.h[j].max(elev + EPS);
                heap.push(Cell { elev: self.h[j], seq, i: j });
                seq += 1;
            }
        }
    }

    /// D8 steepest-descent receiver per cell; outlets drain to themselves.
    fn receivers(&self, outlets: &[bool]) -> Vec<usize> {
        let nx = self.nx;
        let mut recv = vec![0usize; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                if outlets[i] {
                    recv[i] = i;
                    continue;
                }
                let hi = self.h[i];
                let (mut best, mut best_slope) = (i, 0.0f32);
                for (dx, dy) in NEIGHBORS {
                    let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                    let dist = if dx != 0 && dy != 0 { self.cell_m * std::f32::consts::SQRT_2 } else { self.cell_m };
                    let slope = (hi - self.h[j]) / dist;
                    if slope > best_slope {
                        best_slope = slope;
                        best = j;
                    }
                }
                recv[i] = best;
            }
        }
        recv
    }

    /// Ascending-elevation order, ties by index — receiver-before-donor.
    fn elevation_order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.h.len()).collect();
        order.sort_by(|&a, &b| self.h[a].total_cmp(&self.h[b]).then_with(|| a.cmp(&b)));
        order
    }

    /// MFD drainage accumulation (high→low, slopeᵖ-weighted to ALL lower
    /// neighbours) — dissolves D8's grid-aligned rib artifact.
    fn accumulate_drainage(&mut self, order: &[usize]) {
        const P: f32 = 1.1;
        let nx = self.nx;
        let cell_area = self.cell_m * self.cell_m;
        for d in self.drainage.iter_mut() {
            *d = cell_area;
        }
        for &i in order.iter().rev() {
            let (x, y) = (i % nx, i / nx);
            let hi = self.h[i];
            let mut weights = [0.0f32; 8];
            let mut total = 0.0f32;
            for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                let drop = hi - self.h[j];
                if drop > 0.0 {
                    let dist = if *dx != 0 && *dy != 0 { self.cell_m * std::f32::consts::SQRT_2 } else { self.cell_m };
                    let w = (drop / dist).powf(P);
                    weights[k] = w;
                    total += w;
                }
            }
            if total > 0.0 {
                let amount = self.drainage[i];
                for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                    if weights[k] > 0.0 {
                        let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                        self.drainage[j] += amount * (weights[k] / total);
                    }
                }
            }
        }
    }

    /// Implicit stream-power incision (n = 1), low→high so each receiver is
    /// already solved: `h ← (h + f·h_r)/(1 + f)`, `f = K·dt·Aᵐ/dist`. Exact and
    /// unconditionally stable.
    fn incise(&mut self, p: &FluvialParams, recv: &[usize], order: &[usize]) {
        let nx = self.nx;
        for &i in order {
            let r = recv[i];
            if r == i {
                continue;
            }
            let (x, y) = (i % nx, i / nx);
            let (rx, ry) = (r % nx, r / nx);
            let dist = if x != rx && y != ry { self.cell_m * std::f32::consts::SQRT_2 } else { self.cell_m };
            let f = p.k_dt * self.drainage[i].powf(p.m) / dist;
            self.h[i] = (self.h[i] + f * self.h[r]) / (1.0 + f);
        }
    }

    /// Davy–Lague deposition: route this epoch's eroded volume down the D8 tree,
    /// laying down `G·Qs/A` per reach; what reaches an outlet is lost to the sea.
    fn deposit(&mut self, p: &FluvialParams, recv: &[usize], order: &[usize], before: &[f32]) {
        let n = self.nx * self.nx;
        let area = self.cell_m * self.cell_m;
        let mut qs = vec![0.0f32; n];
        for i in 0..n {
            let eroded = before[i] - self.h[i];
            if eroded > 0.0 {
                qs[i] = eroded * area;
            }
        }
        for &i in order.iter().rev() {
            let a = self.drainage[i].max(area);
            let deposit_h = p.deposition * qs[i] / a;
            let deposit_vol = (deposit_h * area).min(qs[i]);
            self.h[i] += deposit_vol / area;
            qs[i] -= deposit_vol;
            let r = recv[i];
            if r != i {
                qs[r] += qs[i];
            }
        }
    }

    /// Talus relaxation: half the over-repose excess moves to the steepest lower
    /// neighbour, snapshot+batch (order-independent ⇒ deterministic).
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
                    let dist = if dx != 0 && dy != 0 { self.cell_m * std::f32::consts::SQRT_2 } else { self.cell_m };
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

    /// Run the full pipeline for `p.epochs`.
    pub fn erode(&mut self, p: &FluvialParams) {
        for _ in 0..p.epochs {
            let outlets = self.outlets();
            if p.uplift_m > 0.0 {
                // DIFFERENTIAL uplift (Joseph): weight the rate by low-frequency
                // coordinate fBm (λ ≈ 5 km; its own domain), so blocks rise at
                // different rates and features tilt as in real landscapes. Erosion
                // vs. sustained uplift also gives base-level equilibrium: graded
                // floodplains and flat coastal shelves (deposition near outlets).
                if self.uplift_w.is_none() {
                    let mut w = vec![0.0f32; self.nx * self.nx];
                    for y in 0..self.nx {
                        for x in 0..self.nx {
                            let cell = CellId::from_face_ij(self.face, self.origin.0 + x as u32, self.origin.1 + y as u32, self.level);
                            let c = cell.to_cube();
                            let f = crate::noise::fbm(3, (c.u + 1.0) * 2000.0, (c.v + 1.0) * 2000.0, 3, 2.0, 0.5);
                            w[y * self.nx + x] = (0.25 + 1.5 * f) as f32; // ~0.25×..1.75×
                        }
                    }
                    self.uplift_w = Some(w);
                }
                let w = self.uplift_w.as_ref().unwrap();
                for (i, &o) in outlets.iter().enumerate() {
                    if !o {
                        self.h[i] += p.uplift_m * w[i];
                    }
                }
            }
            self.fill_depressions(&outlets);
            let recv = self.receivers(&outlets);
            let order = self.elevation_order();
            self.accumulate_drainage(&order);
            let before = if p.deposition > 0.0 { Some(self.h.clone()) } else { None };
            self.incise(p, &recv, &order);
            if let Some(b) = before {
                self.deposit(p, &recv, &order, &b);
            }
            self.talus(p);
        }
    }
}

#[cfg(test)]
mod fluvial_tests {
    use super::*;

    fn small() -> Fluvial {
        Fluvial::from_prior(Face::ZPos, 19, 165_800, 413_600, 96)
    }

    #[test]
    fn deterministic_bit_identical() {
        let p = FluvialParams { epochs: 8, ..Default::default() };
        let mut a = small();
        let mut b = small();
        a.erode(&p);
        b.erode(&p);
        assert_eq!(a.h, b.h, "two runs diverged");
        assert_eq!(a.drainage, b.drainage);
    }

    #[test]
    fn channels_concentrate_and_stay_finite() {
        let p = FluvialParams { epochs: 12, ..Default::default() };
        let mut f = small();
        f.erode(&p);
        let cell_area = f.cell_m * f.cell_m;
        let max_d = f.drainage.iter().cloned().fold(0.0f32, f32::max);
        assert!(max_d > 50.0 * cell_area, "no channel network formed (max {max_d})");
        assert!(f.h.iter().all(|v| v.is_finite()), "heights blew up");
    }
}

/// A finished erosion run, sampleable at ANY finer level: within the region, a
/// column's surface = **bilinear(eroded field) + the detail increment** — the
/// prior's octaves finer than the erosion grid's Nyquist
/// (`surface_prior_m(cell, cell.level()) − surface_prior_m(cell, region level)`).
/// The carved structure replaces exactly the band the sim simulated; fine texture
/// rides on top; outside the region the caller falls back to the baseline (an
/// honest seam at the region edge — the §7.1 spatial seam, unblended for now).
#[derive(Clone)]
pub struct ErodedRegion {
    pub face: Face,
    pub level: u8,
    pub oi: u32,
    pub oj: u32,
    pub nx: usize,
    pub h: Vec<f32>,
}

impl ErodedRegion {
    /// Seed from the prior around a centre (face cells at `level`), erode, keep.
    pub fn build(face: Face, level: u8, center_i: u32, center_j: u32, nx: usize, p: &FluvialParams) -> Self {
        Self::build_from(face, level, center_i, center_j, nx, p, |c| gen::surface_prior_m(c, c.level()))
    }

    /// Seed from an arbitrary surface (e.g. the coarser tiers of the telescope),
    /// erode, keep. The nesting primitive for progressive fine-detail erosion.
    pub fn build_from(face: Face, level: u8, center_i: u32, center_j: u32, nx: usize, p: &FluvialParams, surf: impl Fn(CellId) -> f64) -> Self {
        let half = (nx / 2) as u32;
        let oi = center_i.saturating_sub(half);
        let oj = center_j.saturating_sub(half);
        let mut f = Fluvial::from_surface(face, level, oi, oj, nx, surf);
        f.erode(p);
        Self { face, level, oi, oj, nx, h: f.h }
    }

    /// Does this region cover `cell` (same face, level ≥ region's, inside bounds)?
    /// The cheap bounds-only check — the fidelity-debug overlay's query.
    pub fn covers(&self, cell: CellId) -> bool {
        self.grid_pos(cell).is_some()
    }

    /// Cell centre in region-grid coords, if covered (the shared bounds logic).
    fn grid_pos(&self, cell: CellId) -> Option<(f64, f64)> {
        let (face, i, j, level) = cell.to_face_ij();
        if face != self.face || level < self.level {
            return None;
        }
        let scale = (1u64 << (level - self.level)) as f64;
        let gx = (i as f64 + 0.5) / scale - self.oi as f64 - 0.5;
        let gy = (j as f64 + 0.5) / scale - self.oj as f64 - 0.5;
        if gx < 0.0 || gy < 0.0 || gx > (self.nx - 2) as f64 || gy > (self.nx - 2) as f64 {
            return None;
        }
        Some((gx, gy))
    }

    /// Sampled surface (m above bedrock datum) for `cell`, if it lies within the
    /// region (and on the same face, at a level ≥ the region's).
    pub fn surface_m(&self, cell: CellId) -> Option<f64> {
        let (gx, gy) = self.grid_pos(cell)?;
        let level = cell.to_face_ij().3;
        let (x0, y0) = (gx.floor() as usize, gy.floor() as usize);
        let (fx, fy) = (gx - x0 as f64, gy - y0 as f64);
        let at = |x: usize, y: usize| self.h[y * self.nx + x] as f64;
        let base = at(x0, y0) * (1.0 - fx) * (1.0 - fy)
            + at(x0 + 1, y0) * fx * (1.0 - fy)
            + at(x0, y0 + 1) * (1.0 - fx) * fy
            + at(x0 + 1, y0 + 1) * fx * fy;
        let detail = gen::surface_prior_m(cell, level) - gen::surface_prior_m(cell, self.level);
        Some(base + detail)
    }
}

/// Surface through a TELESCOPE of tiers, finest-first: the first region that
/// contains the cell answers (its coarser parents already shaped its seed); the
/// baseline prior answers everywhere else. `regions` is ordered coarse → fine.
pub fn surface_at(cell: CellId, regions: &[ErodedRegion]) -> f64 {
    for r in regions.iter().rev() {
        if let Some(s) = r.surface_m(cell) {
            return s;
        }
    }
    gen::surface_prior_m(cell, cell.level())
}

/// The finest tier level covering `cell`, if any — the fidelity-debug query
/// (bounds checks only; no sampling).
pub fn tier_at(cell: CellId, regions: &[ErodedRegion]) -> Option<u8> {
    regions.iter().rev().find(|r| r.covers(cell)).map(|r| r.level)
}

/// A column through the fidelity ladder: the finest materialized tier that covers
/// the cell, the baseline prior elsewhere.
pub fn column_at(cell: CellId, regions: &[ErodedRegion]) -> crate::column::Column {
    gen::column_from_surface(cell, surface_at(cell, regions), 2.0)
}
