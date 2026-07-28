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
use crate::gen;
use crate::sea_level;
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

    /// A view coarser than the build level draws the UNCARVED PRIOR, silently.
    ///
    /// `grid_pos` refuses any cell whose level is below the region's own
    /// (`level < self.level`), so `surface_at` falls through to
    /// `gen::initial_topography_m` — the prior — for every cell. The store can be
    /// full of carved tiles and the picture shows none of them, with nothing in
    /// the return value distinguishing "no tile here" from "tile here, wrong
    /// level to ask." Claim home: #obs-coarse-view-draws-the-uncarved-prior .
    ///
    /// Convicts both halves: at the region's own level the carved surface is
    /// returned (so the region is genuinely covering), and one level coarser the
    /// prior comes back instead.
    #[test]
    fn a_view_coarser_than_the_build_level_silently_draws_the_prior() {
        let (seed, level, nx) = (7u64, 9u8, 8usize);
        let (face, oi, oj) = (Face::ZPos, 64u32, 64u32);
        // A region whose heights are unmistakably NOT the prior: a constant far
        // from any plausible terrain, so "did we get the carved value" is
        // decidable by inspection rather than by tolerance.
        let carved = -12_345.0f32;
        let region = ErodedRegion {
            seed,
            face,
            level,
            oi,
            oj,
            nx,
            h: vec![carved; nx * nx],
        };
        let regions = [region];

        // At the build level: covered, and the carved value comes back. The
        // detail term is prior(level) − prior(self.level) = 0 here, so the value
        // is exactly the carved constant.
        let at_build = CellId::from_face_ij(face, oi + 2, oj + 2, level);
        let got = surface_at(seed, at_build, &regions);
        assert!((got - carved as f64).abs() < 1e-6, "at the build level the carved surface must be returned, got {got}");
        assert_eq!(tier_at(at_build, &regions), Some(level), "and the region must report as covering");

        // One level coarser — the same place on the sphere, asked at L8.
        let coarser = CellId::from_face_ij(face, (oi + 2) / 2, (oj + 2) / 2, level - 1);
        let got_coarse = surface_at(seed, coarser, &regions);
        let prior = gen::initial_topography_m(seed, coarser, level - 1);
        assert_eq!(
            tier_at(coarser, &regions),
            None,
            "the defect: a coarser query is not covered at all, though the tile exists"
        );
        assert!(
            (got_coarse - prior).abs() < 1e-9,
            "and the value returned is the UNCARVED prior ({prior}), not the carved surface — got {got_coarse}"
        );
        assert!(
            (got_coarse - carved as f64).abs() > 1000.0,
            "the two must be far apart, else this test would pass on a coincidence"
        );
    }

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
// implicit solve needs), (4) MFD drainage-area accumulation (Quinn et al.; live p=1.0 —
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
    /// Davy–Lague deposition efficiency `G` (0 = pure detachment).
    pub deposition: f32,
    /// Talus repose slope (rise/run). Slopes beyond this slump (half-excess/epoch).
    pub max_slope: f32,
    /// Hillslope (soil-creep) diffusivity κ, m² per epoch. The missing physics
    /// behind the grid-scale sawtooth anomaly (watched live by Joseph; latent in
    /// old core, which also lacked it): detachment-limited incision leaves
    /// un-drained single-cell peaks standing while cutting everything around
    /// them — without diffusion, minimum valley spacing collapses to the grid
    /// wavelength. A CONSTANT κ gives grid coefficient κ/cell² — negligible on
    /// coarse tiers (19 m: ~1e-4), decisive at walk scale (0.6 m: ~0.14) —
    /// exactly the scale dependence real soil creep has.
    pub diffusivity_m2: f32,
    pub epochs: u32,
}

impl Default for FluvialParams {
    fn default() -> Self {
        // κ = 2 m²/epoch: an "epoch" here carves valleys in ~80 steps (≈ centuries),
        // so per-epoch creep is large. Grid coefficient κ/cell²: L19 0.006 (gentle),
        // L21 0.09 (kills the observed 4.8 m sawteeth), L24 clamped 0.24 (dominant —
        // walk-scale interfluves are creep-smoothed, as in real landscapes).
        Self { k_dt: 0.02, m: 0.5, deposition: 1.0, max_slope: 0.8, diffusivity_m2: 2.0, epochs: 80 }
    }
}

/// A square fluvial simulation field over one face region — the frame's port of
/// core's `Heightfield`. Heights in metres above the bedrock datum.
pub struct Fluvial {
    pub nx: usize,
    /// Characteristic cell length (m) at this level — the face-mean cell size.
    /// Still used for the creep diffusion-number `κ/cell²`; slope/flux **lengths**
    /// now use true per-neighbour great-circle distances ([`Self::dist_m`]), not
    /// this uniform value. **Not** for cell area; see [`Self::cell_area`].
    pub cell_m: f32,
    /// True spherical cell area (m²) per cell — equiangular closed form
    /// (`crate::measure`). Retires uniform `cell_m²` for drainage / deposition
    /// volume (`#obs-cube-locked-kernel-bias`).
    pub cell_area: Vec<f32>,
    /// Unit direction vector at each cell centre (`crate::measure::cell_center_unit`).
    /// Source for true neighbour **lengths** ([`Self::dist_m`]) — retires uniform
    /// `cell_m` / diagonal `cell_m·√2`, a cube-locked length bias that overstates
    /// distances toward the cube corners (`#obs-cube-locked-kernel-bias`).
    centers: Vec<[f64; 3]>,
    pub h: Vec<f32>,
    /// MFD drainage area (m²) from the last epoch — the dendritic-ness instrument.
    pub drainage: Vec<f32>,
    /// Where this field sits (face cells at `level` from `origin`) — identity for
    /// the differential-uplift field and for wrapping back into an ErodedRegion.
    pub face: Face,
    pub level: u8,
    pub origin: (u32, u32),
    /// The world-seed — identity for every fated-noise draw this run makes
    /// (differential uplift today; anything stochastic later).
    pub seed: u64,
    /// Per-cell rock-uplift rate (m/epoch), supplied by the uplift nomos
    /// (`crate::uplift`) via [`Fluvial::set_uplift_rate`]. Zeros (the default) =
    /// no tectonic driver. Erosion CONSUMES this each epoch; it does not compute
    /// it — "what lifts the land" is its own article of law.
    uplift_rate: Vec<f32>,
    /// Per-cell precipitation weight (relative to the tile mean), from the climate
    /// nomos (`crate::climate`) via [`Fluvial::set_precip_weight`]. Runoff seeded
    /// into the drainage accumulation scales by it: discharge ∝ local precip ×
    /// area. Ones (the default) = spatially-uniform rain = no change; when climate
    /// gains geography, wetter ground gathers more discharge.
    precip_weight: Vec<f32>,
    /// Mean |Δh| (m) of the LAST epoch — Joseph's convergence instrument: when
    /// this levels out, further epochs are polishing a steady state.
    pub last_delta_m: f32,
}

const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

impl Fluvial {
    /// Seed from the band-limited prior over `nx × nx` cells of `face` at `level`
    /// starting at `(oi, oj)` — the honest initial condition (no imposed shapes).
    pub fn from_prior(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Self {
        Self::from_surface(seed, face, level, oi, oj, nx, |c| gen::initial_topography_m(seed, c, c.level()))
    }

    /// Seed from an arbitrary surface function — how a FINE tier is seeded from
    /// the coarse tiers below it (the §7.2 downscaling seam: the fine sim's
    /// initial condition is the downscaled coarse end-state + detail increment).
    pub fn from_surface(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize, surf: impl Fn(CellId) -> f64) -> Self {
        let radius = crate::planet::Planet::EARTH.radius_m;
        let cell_m = crate::sample::cell_size_m(level, radius) as f32;
        let mut h = vec![0.0f32; nx * nx];
        let mut cell_area = vec![0.0f32; nx * nx];
        let mut centers = vec![[0.0f64; 3]; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let gi = oi + x as u32;
                let gj = oj + y as u32;
                let cell = CellId::from_face_ij(face, gi, gj, level);
                h[y * nx + x] = surf(cell) as f32;
                cell_area[y * nx + x] =
                    crate::measure::cell_area_m2(face, gi as u64, gj as u64, level, radius) as f32;
                centers[y * nx + x] =
                    crate::measure::cell_center_unit(face, gi as u64, gj as u64, level);
            }
        }
        Self {
            nx,
            cell_m,
            cell_area,
            centers,
            h,
            drainage: vec![0.0; nx * nx],
            face,
            level,
            origin: (oi, oj),
            seed,
            uplift_rate: vec![0.0; nx * nx],
            precip_weight: vec![1.0; nx * nx],
            last_delta_m: f32::INFINITY,
        }
    }

    /// Resume a simulation over an existing eroded field (e.g. the startup tier),
    /// so the live loop can keep running epochs without redoing the initial work.
    pub fn from_region(r: &ErodedRegion) -> Self {
        Self::from_surface(r.seed, r.face, r.level, r.oi, r.oj, r.nx, |_| 0.0).with_heights(r.h.clone())
    }

    /// Replace heights after [`from_surface`] scaffolding (used by [`from_region`]).
    pub(crate) fn with_heights(mut self, h: Vec<f32>) -> Self {
        debug_assert_eq!(h.len(), self.nx * self.nx);
        self.h = h;
        self
    }

    /// Snapshot into a sampleable region.
    pub fn to_region(&self) -> ErodedRegion {
        ErodedRegion { face: self.face, level: self.level, oi: self.origin.0, oj: self.origin.1, nx: self.nx, h: self.h.clone(), seed: self.seed }
    }

    /// Supply the per-cell rock-uplift rate (m/epoch) this run erodes against —
    /// the field produced by the uplift nomos (`crate::uplift`). Length must be
    /// `nx × nx`. This is how the WORLD path drives uplift: erosion consumes a
    /// pulled, keyed uplift tile, never a hidden internal term.
    pub fn set_uplift_rate(&mut self, field: Vec<f32>) {
        debug_assert_eq!(field.len(), self.nx * self.nx, "uplift field must be nx × nx");
        self.uplift_rate = field;
    }

    /// Convenience for INSTRUMENTS probing the kernel: a spatially-uniform uplift
    /// rate (m/epoch). The world path uses [`Fluvial::set_uplift_rate`] with the
    /// nomos's differential field instead — this is a crude probe knob, not law.
    pub fn set_uniform_uplift(&mut self, rate_m_per_epoch: f32) {
        self.uplift_rate = vec![rate_m_per_epoch; self.nx * self.nx];
    }

    /// Supply the per-cell precipitation weight (relative to the tile mean) from
    /// the climate nomos (`crate::climate`). Length must be `nx × nx`. Uniform rain
    /// ⇒ all ones ⇒ discharge unchanged; spatial rain redistributes it (wetter
    /// ground gathers more). Erosion CONSUMES this; it does not model rain.
    pub fn set_precip_weight(&mut self, weight: Vec<f32>) {
        debug_assert_eq!(weight.len(), self.nx * self.nx, "precip weight must be nx × nx");
        self.precip_weight = weight;
    }

    #[inline]
    fn is_edge(nx: usize, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == nx - 1 || y == nx - 1
    }

    /// True great-circle distance (m) between the centres of cells at linear
    /// indices `a` and `b` — the honest slope/flux **length**, retiring uniform
    /// `cell_m` / diagonal `cell_m·√2` for D8, MFD, incision and talus. On the
    /// equiangular cube-sphere the uniform value overstates distance toward the
    /// cube corners (`#obs-cube-locked-kernel-bias`); true metrics are necessary
    /// but not sufficient against the fan's structural bias (`#norm-bias-vs-noise`).
    #[inline]
    fn dist_m(&self, a: usize, b: usize) -> f32 {
        crate::measure::gc_dist_m(self.centers[a], self.centers[b], crate::planet::Planet::EARTH.radius_m) as f32
    }

    /// Outlets: coast (`h ≤ sea`) always; **tile-edge sinks only when this tile
    /// is a proper partial domain** (does not cover a whole cube face).
    ///
    /// Edge outlets are the principled incomplete-tile base level until flux-BC
    /// (plan Phase-3). When the builder sweeps a **full face** as one tile
    /// (`oi=oj=0`, `nx = 2^level`), those edges *are* cube seams — treating them
    /// as sinks carves artificial trenches around every face (measured ~300 m
    /// edge–interior gap after 20 epochs). Full-face tiles therefore use
    /// coast-only outlets so sphere continuity from the prior is not destroyed.
    fn outlets(&self) -> Vec<bool> {
        let nx = self.nx;
        let sea = sea_level::derived_sea_level_m(self.seed) as f32;
        let face_n = 1usize << self.level;
        let full_face = self.origin == (0, 0) && nx == face_n;
        let mut out = vec![false; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let edge_sink = !full_face && Self::is_edge(nx, x, y);
                out[i] = edge_sink || self.h[i] <= sea;
            }
        }
        // All-land full-face tile: need at least one sink for Priority-Flood.
        if full_face && !out.iter().any(|&o| o) {
            let mut best = (f32::INFINITY, 0usize);
            for (i, &h) in self.h.iter().enumerate() {
                if h < best.0 {
                    best = (h, i);
                }
            }
            out[best.1] = true;
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
        let nxi = nx as i32;
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
        const P: f32 = 1.0; // directional first-moment unbiased on square lattice; DECISIONS[the-router-is-a-scalar-pretending-to-be-a-vector-and-p-is-the-bias] / #obs-cube-locked-kernel-bias
        let nx = self.nx;
        // Local runoff = true spherical cell area × local precip weight.
        // (Uniform cell_m² was a cube-locked bias — `#obs-cube-locked-kernel-bias`.)
        for i in 0..self.drainage.len() {
            self.drainage[i] = self.cell_area[i] * self.precip_weight[i];
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
                    let dist = self.dist_m(i, j);
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
        for &i in order {
            let r = recv[i];
            if r == i {
                continue;
            }
            let dist = self.dist_m(i, r);
            let f = p.k_dt * self.drainage[i].powf(p.m) / dist;
            self.h[i] = (self.h[i] + f * self.h[r]) / (1.0 + f);
        }
    }

    /// Davy–Lague deposition: route this epoch's eroded volume down the D8 tree,
    /// laying down `G·Qs/A` per reach; what reaches an outlet is lost to the sea.
    fn deposit(&mut self, p: &FluvialParams, recv: &[usize], order: &[usize], before: &[f32]) {
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

    /// CONVICTED OPERATOR — retained only as the instrument that demonstrates its own
    /// defects (`#obs-mean-pin-manufactures-seam`; council `mean-pin-manufactures-the-seam-and-the-mass`,
    /// retire-or-replace). **No production caller**: world-gen composition (`query.rs`) seeds a
    /// fine tier from the coarse surface and never pins. It was written to enforce
    /// `#form-rl-closure-algebra` law (1) `R∘L = id` on the block mean — and it does NOT: it
    /// computes a per-block delta (coarse point-sample − fine block mean) and BILINEARLY
    /// upsamples it, and a bilinear upsample of a piecewise-constant field does not preserve
    /// block means (residual median ~0.5 m, max ~3 m). It is also a mass source (~+0.22% at
    /// 150 fine epochs on high-relief land) and the single largest manufacturer of the tile
    /// seam it was meant to prevent (removing it takes the zero-physics seam ratio ~1.96→1.17,
    /// the worst-case ~12.8→3.8). Do NOT re-introduce it into a live surface, and do NOT
    /// re-assert the `R∘L = id` claim in its name. See `pin_block_means_const` for the honest
    /// injection form (pins the mean exactly but keeps the mass source and washboards the
    /// interior — also convicted); the admissible fix is leaf-only + `#form-face-flux-register`.
    pub fn pin_block_means(&mut self, parent_level: u8, parent: impl Fn(CellId) -> f64) {
        debug_assert!(parent_level < self.level);
        let b = 1usize << (self.level - parent_level);
        let nb = self.nx / b;
        if nb < 2 {
            return;
        }
        let mut delta = vec![0.0f32; nb * nb];
        for by in 0..nb {
            for bx in 0..nb {
                let mut sum = 0.0f64;
                for y in 0..b {
                    for x in 0..b {
                        sum += self.h[(by * b + y) * self.nx + bx * b + x] as f64;
                    }
                }
                let mean = sum / (b * b) as f64;
                let cx = self.origin.0 + (bx * b + b / 2) as u32;
                let cy = self.origin.1 + (by * b + b / 2) as u32;
                let target = parent(CellId::from_face_ij(self.face, cx, cy, self.level));
                delta[by * nb + bx] = (target - mean) as f32;
            }
        }
        // Bilinearly upsample the block-delta field over the fine cells.
        for y in 0..self.nx {
            for x in 0..self.nx {
                let gx = ((x as f64 + 0.5) / b as f64 - 0.5).clamp(0.0, (nb - 1) as f64);
                let gy = ((y as f64 + 0.5) / b as f64 - 0.5).clamp(0.0, (nb - 1) as f64);
                let (x0, y0) = (gx.floor() as usize, gy.floor() as usize);
                let (x1, y1) = ((x0 + 1).min(nb - 1), (y0 + 1).min(nb - 1));
                let (fx, fy) = ((gx - x0 as f64) as f32, (gy - y0 as f64) as f32);
                let d = delta[y0 * nb + x0] * (1.0 - fx) * (1.0 - fy)
                    + delta[y0 * nb + x1] * fx * (1.0 - fy)
                    + delta[y1 * nb + x0] * (1.0 - fx) * fy
                    + delta[y1 * nb + x1] * fx * fy;
                self.h[y * self.nx + x] += d;
            }
        }
    }

    /// EXPERIMENT CANDIDATE (council `mean-pin-manufactures-the-seam-and-the-mass`,
    /// retire-or-replace, replacement judged against FE(8) — can fail). Block-constant
    /// injection `h ← h + Δ_block`: the honest form of the operator #obs-mean-pin-manufactures-seam
    /// FE(1) names. Same per-block delta (coarse point-sample target − fine block mean) but
    /// added as a CONSTANT over the block instead of bilinearly upsampled — so it genuinely
    /// preserves block means (R∘L = id on the mean becomes TRUE), unlike the bilinear form.
    /// It does NOT claim to fix the seam or the mass source: it still coerces the fine tier
    /// toward the stale coarse target (injection alone, no refluxing partner — that lives on
    /// #form-face-flux-register). Measured, not shipped-by-default. Not in the live pipeline.
    pub fn pin_block_means_const(&mut self, parent_level: u8, parent: impl Fn(CellId) -> f64) {
        debug_assert!(parent_level < self.level);
        let b = 1usize << (self.level - parent_level);
        let nb = self.nx / b;
        if nb < 2 {
            return;
        }
        for by in 0..nb {
            for bx in 0..nb {
                let mut sum = 0.0f64;
                for y in 0..b {
                    for x in 0..b {
                        sum += self.h[(by * b + y) * self.nx + bx * b + x] as f64;
                    }
                }
                let mean = sum / (b * b) as f64;
                let cx = self.origin.0 + (bx * b + b / 2) as u32;
                let cy = self.origin.1 + (by * b + b / 2) as u32;
                let target = parent(CellId::from_face_ij(self.face, cx, cy, self.level));
                let d = (target - mean) as f32;
                for y in 0..b {
                    for x in 0..b {
                        self.h[(by * b + y) * self.nx + bx * b + x] += d;
                    }
                }
            }
        }
    }

    /// Run the full pipeline for `p.epochs`, tracking the last epoch's mean |Δh|.
    pub fn erode(&mut self, p: &FluvialParams) {
        for e in 0..p.epochs {
            let track_before = if e + 1 == p.epochs { Some(self.h.clone()) } else { None };
            let outlets = self.outlets();
            // Tectonic uplift, CONSUMED from the uplift nomos (`crate::uplift`)
            // via set_uplift_rate — differential (per-cell) and pre-computed, so
            // erosion carries no uplift model of its own. It applies to all
            // interior ground, submarine included (a seamount may rise past the
            // waterline — the seabed is not a special case, Joseph); only the grid
            // edge is pinned. Sustained uplift vs. erosion is what gives base-level
            // equilibrium — graded floodplains, flat coastal shelves — and macro
            // relief at all (zero uplift → the landscape planes to a peneplain).
            // Zeros (the default field) make this loop a no-op.
            let nx = self.nx;
            for i in 0..nx * nx {
                let rate = self.uplift_rate[i];
                if rate != 0.0 {
                    let (x, y) = (i % nx, i / nx);
                    if !Self::is_edge(nx, x, y) {
                        self.h[i] += rate;
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
            self.creep(p);
            if let Some(tb) = track_before {
                let sum: f64 = self.h.iter().zip(tb.iter()).map(|(a, b)| (a - b).abs() as f64).sum();
                self.last_delta_m = (sum / self.h.len() as f64) as f32;
            }
        }
    }

    /// Hillslope diffusion (soil creep): one explicit 5-point Laplacian step per
    /// epoch, `k = κ/cell²` clamped to the stability bound. Interior cells only
    /// (outlets/edges are base level). This is what keeps interfluves smooth at
    /// fine scales and stops incision sharpening single-cell teeth.
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
                let lap = snapshot[i - 1] + snapshot[i + 1] + snapshot[i - nx] + snapshot[i + nx] - 4.0 * snapshot[i];
                self.h[i] += k * lap;
            }
        }
    }
}

#[cfg(test)]
mod fluvial_tests {
    use super::*;

    /// A SUBAERIAL test footprint — and the word is load-bearing.
    ///
    /// **These tests were vacuous until 2026-07-12.** The old footprint
    /// (`165_800, 413_600`) sits at 3709–3715 m, entirely below `SEA_LEVEL_M`
    /// (4000). Every cell is therefore an outlet → `recv[i] = i` → `incise()`
    /// skips every cell → Priority-Flood, D8, MFD, stream-power and Davy–Lague
    /// all no-op. Measured `max|Δh|` after 80 epochs: **0.000 m, bit-exactly.**
    /// All three tests below passed anyway, because they were comparing no-ops
    /// to no-ops. (`seam_ridge` shared the footprint, which is why its "ratio
    /// 22888" was really `0 ÷ 1e-9` — a divide-by-zero against the epsilon
    /// floor, not a seam measurement. The tell was printed all along: the ratio
    /// was bit-identical across every age gap the probe swept.)
    ///
    /// Constructed **subaerial** bowl above derived sea — not a prior sample
    /// (which may be mostly submarine at seed 0 after the pour). Relief is high
    /// enough that outlets are the rim, not the coast, so incision actually runs.
    fn small() -> Fluvial {
        let seed = 0u64;
        let sea = crate::sea_level::derived_sea_level_m(seed);
        let face = Face::ZPos;
        let (level, oi, oj, nx) = (19u8, 108_500u32, 186_350u32, 96usize);
        Fluvial::from_surface(seed, face, level, oi, oj, nx, |c| {
            let (f, i, j, _) = c.to_face_ij();
            let di = i.saturating_sub(oi) as f64;
            let dj = j.saturating_sub(oj) as f64;
            let cx = di - nx as f64 / 2.0;
            let cy = dj - nx as f64 / 2.0;
            // Dome peak well above sea; rim still subaerial.
            sea + 800.0 - 0.05 * (cx * cx + cy * cy) + if f == face { 0.0 } else { 0.0 }
        })
    }

    /// Guards the guard: if a future prior change drowns this footprint, every
    /// fluvial test below silently becomes a no-op again. Fail loudly instead.
    #[test]
    fn test_footprint_is_actually_land() {
        let f = small();
        // Seed 0, derived sea after freeboard — not the retired decreed 4000 m datum.
        let sea = crate::sea_level::derived_sea_level_m(0) as f32;
        let above = f.h.iter().filter(|&&h| h > sea).count();
        assert!(
            above * 2 > f.h.len(),
            "the fluvial test footprint must be mostly LAND (>{sea} m derived) or these tests test a no-op — \
             only {above}/{} cells are subaerial",
            f.h.len()
        );
    }

    /// The fluvial kernels read TRUE spherical neighbour lengths, not uniform
    /// `cell_m` / `cell_m·√2`. Anchored at the face corner (origin 0,0) where the
    /// equiangular map shrinks cells: uniform would make both ratios exactly 1.0,
    /// so reverting `dist_m` to the old constants fails this probe
    /// (`#obs-cube-locked-kernel-bias`, `#norm-probe-sensitivity` known-bad).
    #[test]
    fn neighbor_lengths_are_true_spherical_not_uniform() {
        let nx = 8usize;
        let f = Fluvial::from_surface(0, Face::ZPos, 7, 0, 0, nx, |_| 0.0);
        let idx = |x: usize, y: usize| y * nx + x;
        // Corner cell (0,0) east neighbour: ~6.3% shorter than uniform cell_m.
        let ratio_e = f.dist_m(idx(0, 0), idx(1, 0)) / f.cell_m;
        assert!(
            (ratio_e - 0.9372).abs() < 0.01,
            "corner east ratio {ratio_e} expected ~0.937 (uniform would be 1.0)"
        );
        // Corner diagonal: ~33% shorter than the uniform diagonal cell_m·√2.
        let ratio_d = f.dist_m(idx(0, 0), idx(1, 1)) / (f.cell_m * std::f32::consts::SQRT_2);
        assert!(
            (ratio_d - 0.672).abs() < 0.01,
            "corner diagonal ratio {ratio_d} expected ~0.672 (uniform would be 1.0)"
        );
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

    /// CONVICTING probe (was `pin_preserves_parent_means`, a green with a 2 m tolerance
    /// "sized to the residual" — the `#norm-probe-sensitivity` failure species that certified
    /// the very lie `#obs-mean-pin-manufactures-seam` retracts). Restated to convict:
    /// `pin_block_means` (BILINEAR) does NOT pin the block mean, while `pin_block_means_const`
    /// (the honest injection form) does — to machine precision. The relative gap is asserted,
    /// self-calibrating, so it cannot be re-tuned into a false green. Hermetic: an analytic
    /// rough fine surface pinned to an analytic (curved) coarse parent — no ocean pour, no
    /// erosion, so the probe isolates the OPERATOR, not the machinery around it.
    #[test]
    fn bilinear_pin_does_not_preserve_means_block_const_does() {
        let (face, level, oi, oj, nx) = (Face::ZPos, 19u8, 100_000u32, 180_000u32, 64usize);
        // Rough fine surface: within-block variation (sinusoid) + between-block curvature.
        let fine_surf = |c: CellId| -> f64 {
            let (_, i, j, _) = c.to_face_ij();
            let (di, dj) = ((i - oi) as f64, (j - oj) as f64);
            100.0 * (di * 0.30).sin() * (dj * 0.25).cos() + 0.02 * (di * di + dj * dj)
        };
        // Coarse parent: a smooth, CURVED field — so the per-block delta varies between
        // blocks and a bilinear upsample of it cannot preserve block means.
        let parent = |c: CellId| -> f64 {
            let (_, i, j, _) = c.to_face_ij();
            let (di, dj) = ((i - oi) as f64, (j - oj) as f64);
            500.0 + 0.05 * (di * di + dj * dj) - 3.0 * di
        };
        let block_residual = |f: &Fluvial| -> f64 {
            let b = 4usize; // pin to level-2
            let nb = nx / b;
            let mut worst = 0.0f64;
            for by in 0..nb {
                for bx in 0..nb {
                    let mut m = 0.0f64;
                    for y in 0..b {
                        for x in 0..b {
                            m += f.h[(by * b + y) * nx + bx * b + x] as f64;
                        }
                    }
                    m /= (b * b) as f64;
                    let cx = oi + (bx * b + b / 2) as u32;
                    let cy = oj + (by * b + b / 2) as u32;
                    let t = parent(CellId::from_face_ij(face, cx, cy, level));
                    worst = worst.max((m - t).abs());
                }
            }
            worst
        };

        let mut fb = Fluvial::from_surface(0, face, level, oi, oj, nx, fine_surf);
        fb.pin_block_means(level - 2, parent);
        let bilinear_worst = block_residual(&fb);

        let mut fc = Fluvial::from_surface(0, face, level, oi, oj, nx, fine_surf);
        fc.pin_block_means_const(level - 2, parent);
        let const_worst = block_residual(&fc);

        // Block-const pins to ~machine precision; bilinear does not, by orders of magnitude.
        // Absolute thresholds are avoided so this can never be re-tuned into a false green.
        assert!(
            const_worst < 1e-2,
            "block-const injection must pin the mean (worst |mean−target| = {const_worst:.5} m)"
        );
        assert!(
            bilinear_worst > 50.0 * const_worst.max(1e-4),
            "bilinear pin must be convicted as NOT pinning: bilinear worst {bilinear_worst:.4} m \
             vs block-const worst {const_worst:.6} m (expected orders larger)"
        );
    }

    #[test]
    fn channels_concentrate_and_stay_finite() {
        let p = FluvialParams { epochs: 12, ..Default::default() };
        let mut f = small();
        f.erode(&p);
        let cell_area = f.cell_area.iter().cloned().fold(0.0f32, f32::max);
        let max_d = f.drainage.iter().cloned().fold(0.0f32, f32::max);
        assert!(max_d > 50.0 * cell_area, "no channel network formed (max {max_d})");
        assert!(f.h.iter().all(|v| v.is_finite()), "heights blew up");
    }
}

/// A finished erosion run, sampleable at ANY finer level: within the region, a
/// column's surface = **bilinear(eroded field) + the detail increment** — the
/// prior's octaves finer than the erosion grid's Nyquist
/// (`initial_topography_m(cell, cell.level()) − initial_topography_m(cell, region level)`).
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
    /// The world-seed the run was made under — needed to sample honestly (the
    /// detail increment re-derives prior octaves, which are seed-dependent).
    pub seed: u64,
}

impl ErodedRegion {
    /// Seed from the prior around a centre (face cells at `level`), erode, keep.
    pub fn build(seed: u64, face: Face, level: u8, center_i: u32, center_j: u32, nx: usize, p: &FluvialParams) -> Self {
        Self::build_from(seed, face, level, center_i, center_j, nx, p, |c| gen::initial_topography_m(seed, c, c.level()))
    }

    /// Seed from an arbitrary surface (e.g. the coarser tiers of the telescope),
    /// erode, keep. The nesting primitive for progressive fine-detail erosion.
    pub fn build_from(seed: u64, face: Face, level: u8, center_i: u32, center_j: u32, nx: usize, p: &FluvialParams, surf: impl Fn(CellId) -> f64) -> Self {
        let half = (nx / 2) as u32;
        let oi = center_i.saturating_sub(half);
        let oj = center_j.saturating_sub(half);
        let mut f = Fluvial::from_surface(seed, face, level, oi, oj, nx, surf);
        f.erode(p);
        Self { face, level, oi, oj, nx, h: f.h, seed }
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

    /// Bilinear-only sample (no detail increment) — the LOW band, used as the
    /// pin target for fine-tier mean conservation.
    pub fn surface_bilinear_m(&self, cell: CellId) -> Option<f64> {
        let (gx, gy) = self.grid_pos(cell)?;
        let (x0, y0) = (gx.floor() as usize, gy.floor() as usize);
        let (fx, fy) = (gx - x0 as f64, gy - y0 as f64);
        let at = |x: usize, y: usize| self.h[y * self.nx + x] as f64;
        Some(at(x0, y0) * (1.0 - fx) * (1.0 - fy)
            + at(x0 + 1, y0) * fx * (1.0 - fy)
            + at(x0, y0 + 1) * (1.0 - fx) * fy
            + at(x0 + 1, y0 + 1) * fx * fy)
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
        let detail = gen::initial_topography_m(self.seed, cell, level) - gen::initial_topography_m(self.seed, cell, self.level);
        Some(base + detail)
    }
}

/// Surface through a TELESCOPE of tiers, finest-first: the first region that
/// contains the cell answers (its coarser parents already shaped its seed); the
/// baseline prior answers everywhere else. `regions` is ordered coarse → fine.
pub fn surface_at(seed: u64, cell: CellId, regions: &[ErodedRegion]) -> f64 {
    // Ordering is a CONTRACT, and passing fine-first fails silently (the coarse
    // tier answers everything, the fine tier is dead weight — a probe lost an
    // hour to this). Cheap guard in debug builds.
    debug_assert!(regions.windows(2).all(|w| w[0].level <= w[1].level), "surface_at: regions must be ordered coarse -> fine");
    for r in regions.iter().rev() {
        if let Some(s) = r.surface_m(cell) {
            return s;
        }
    }
    gen::initial_topography_m(seed, cell, cell.level())
}

/// The finest tier level covering `cell`, if any — the fidelity-debug query
/// (bounds checks only; no sampling).
pub fn tier_at(cell: CellId, regions: &[ErodedRegion]) -> Option<u8> {
    regions.iter().rev().find(|r| r.covers(cell)).map(|r| r.level)
}

/// A column through the fidelity ladder: the finest materialized tier that covers
/// the cell, the baseline prior elsewhere.
pub fn column_at(seed: u64, cell: CellId, regions: &[ErodedRegion]) -> crate::column::Column {
    gen::column_from_surface(cell, surface_at(seed, cell, regions), 2.0)
}
