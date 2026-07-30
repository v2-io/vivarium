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
// Frame-native: heights in METRES, sea level = `sea_level::derived_sea_level_m`
// (the poured waterline, not the retired `gen::SEA_LEVEL_M` decree) as a real
// outlet set (rivers run to the coast, not just the grid edge), seeded from the
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

/// What a tile's outer boundary is treated as — its **boundary contract**
/// (`#form-declared-boundary-contract`).
///
/// This was never a choice before: [`Fluvial::outlets`] inferred it from
/// geometry, taking the sink branch on any window short of a whole cube face
/// and the coast-only branch on a whole face. That inference is preserved as
/// the *default* ([`Fluvial::inferred_edge_contract`]), so every existing path
/// carves exactly what it carved before and the builder's complete key stays
/// complete — the key must gain a `contract` field on the day a *keyed* caller
/// can choose one, and not before.
///
/// Making it selectable is what lets a sub-face window be carved under the
/// contract it does not have, which is the experiment the L9 grain sweep could
/// not run: nothing measured the beacon regime against its own alternative,
/// because there was no way to ask for it
/// (`#obs-lakes-are-routed-over-not-carved-away` FE(7),
/// `#obs-tile-outlets-grade-away-the-basins` FE(5)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeContract {
    /// **Every perimeter cell is a base-level sink.** The tile grades to its own
    /// perimeter, so no tile-local basin can exceed it and the incision driver
    /// is starved against what the assembled surface carries
    /// (`#obs-tile-outlets-grade-away-the-basins` FE(4)). Today's policy on any
    /// window short of a whole cube face; the code calls it "the principled
    /// incomplete-tile base level until flux-BC."
    BaseLevelSink,
    /// **Only the coast is an outlet; the perimeter is a no-flux wall** — a
    /// perimeter cell has no receiver outside the field, so drainage divides are
    /// forced onto the boundary rather than base level being handed to it. This
    /// is a *different* undeclared contract, not the absence of one, and it is
    /// what a whole cube face has always used (edge sinks there carved ~300 m
    /// trenches around every face, measured).
    ///
    /// A domain with no coast at all would have no sink and Priority-Flood
    /// nothing to flood from, so the lowest cell is made one — stated here
    /// because it is a real part of the contract, not an implementation detail.
    NoFluxWall,
}

/// Jacobi halo-exchange schedule for a same-level tile seam
/// (`#form-same-level-halo-exchange`).
///
/// A tile of `n²` interior cells is carved on an `(n+2d)²` window; every `σ`
/// epochs each tile's halo is overwritten from one **frozen** assembly of all
/// interiors in the region (Jacobi / additive Schwarz). The three fields are
/// **identity**: different `(d, σ, ρ)` produce different beds and must fold into
/// the complete key (`#form-complete-content-addressed-key` FE(6)).
///
/// This is the third boundary-contract value named in
/// `#form-declared-boundary-contract` FE(6) — `Halo{d, σ, ρ}` — carried as its
/// own type so the kernel's two single-tile outlet policies stay a clean enum
/// while the multi-tile schedule keys as a descriptor, not a payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HaloSchedule {
    /// Halo depth in cells (`d`). The window is `(n + 2d)²`; only the interior
    /// `n²` is kept.
    pub depth: u16,
    /// Exchange cadence in epochs (`σ`). Must be ≥ 1. Cadence is the cheap dial
    /// measured at the beacon (`#obs-exchange-repairs-the-seam-and-overlap-does-not`
    /// FE(7)); when exchange is on, `σ` is identity and equals the stage stride
    /// (`#form-same-level-halo-exchange` FE(7)).
    pub cadence: u32,
    /// Cone truncation radius in **tiles** (`ρ`). Beyond `ρ` tiles from a
    /// demand pull, halo values fall back to the prior. `0` means no truncation
    /// inside a swept region (outside the built block, the prior is still used).
    /// Unmeasured as a cost lever; present so the key shape is complete.
    pub cone_rho: u16,
}

impl HaloSchedule {
    /// Values that put seam step and mean elevation on the single-field carve at
    /// the beacon grain in `#obs-exchange-repairs-the-seam-and-overlap-does-not`
    /// FE(7) (`d=16`, `σ=10`). Not law — a measured operating point.
    pub const MEASURED_BEACON: Self = Self { depth: 16, cadence: 10, cone_rho: 0 };

    /// Production schedule for a builder pass: measured halo depth; cadence is
    /// the stage stride when staging ( #form-same-level-halo-exchange FE(7):
    /// exchange cadence and stage stride are the same number), otherwise the
    /// measured beacon cadence capped by `epochs`.
    pub fn for_build(stage_stride: u32, epochs: u32) -> Self {
        let cadence = if stage_stride > 0 && stage_stride < epochs {
            stage_stride
        } else if epochs == 0 {
            1
        } else {
            10u32.min(epochs).max(1)
        };
        Self { depth: 16, cadence, cone_rho: 0 }
    }

    pub fn depth_usize(self) -> usize {
        self.depth as usize
    }
}

/// The **exchange domain** of a Jacobi region carve: which block of tiles
/// exchanged together. Region membership is identity exactly as `(d, σ, ρ)`
/// is: over repeated exchanges, information crosses the whole block, so a tile
/// carved as part of a 1×1 region and the same tile carved inside a 24×24 face
/// sweep hold different interiors. The region therefore folds into the
/// complete key beside the schedule (`#form-same-level-halo-exchange` FE(4);
/// `#form-depend-by-key-never-latest` FE(1) — two demand shapes must never
/// mint two worlds under one key).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HaloRegion {
    /// Region origin in cells (face coordinates at the carve level).
    pub oi: u32,
    pub oj: u32,
    /// Block extent in tiles.
    pub tiles_i: usize,
    pub tiles_j: usize,
}

/// The complete identity of the eroded bed a consumer settles on — the article
/// a `water-tile` (or any future bed consumer) names in its own key, so the
/// consumer's bytes stay a pure function of its key rather than of whichever
/// bed cohort happened to be in the store at compute time
/// (`#form-depend-by-key-never-latest` FE(4)(b)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BedArticle {
    /// The historical single-tile carve: base-level sink on the tile perimeter.
    /// Keys as the absent field, so every existing world stays addressable.
    EdgeSink,
    /// A tile from a Jacobi halo-exchange region carve.
    Halo { schedule: HaloSchedule, region: HaloRegion },
}

impl BedArticle {
    /// Canonical single-field key token, `None` for the historical shape.
    /// One field rather than eight so consumer keys stay legible in a census.
    pub fn key_token(&self) -> Option<String> {
        match self {
            BedArticle::EdgeSink => None,
            BedArticle::Halo { schedule: s, region: r } => Some(format!(
                "halo,d{},s{},r{},o{}+{},t{}x{}",
                s.depth, s.cadence, s.cone_rho, r.oi, r.oj, r.tiles_i, r.tiles_j
            )),
        }
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
    /// The declared boundary contract ([`EdgeContract`]). Set by
    /// [`Fluvial::inferred_edge_contract`] at construction, so an untouched
    /// field behaves exactly as the geometry-inferred policy did; change it with
    /// [`Fluvial::set_edge_contract`].
    edge: EdgeContract,
}

const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

/// See [`Fluvial::chi_profile`] — per-cell fields over the surface as it
/// stands, all of length `nx × nx` and indexed the same way as `h`.
#[derive(Debug, Clone)]
pub struct ChiProfile {
    /// $\chi$ (m) — the upstream integral $\int_{x_b}^{x}(A_0/A)^{m/n}\,dx$
    /// along the D8 path, zero at every base-level cell.
    pub chi: Vec<f32>,
    /// The elevation this kernel's own update implies at driven steady state,
    /// integrated up from each basin's base-level elevation. Zero free
    /// parameters.
    pub z_steady: Vec<f32>,
    /// Linear index of the base-level cell each cell drains to; `u32::MAX`
    /// where the cell reaches none.
    pub basin: Vec<u32>,
    /// The **filled** surface the network was derived on (the kernel's own
    /// working surface, not the stored one).
    pub h: Vec<f32>,
    /// MFD drainage area (m²) — the same $A$ the incision step consumes.
    pub drainage: Vec<f32>,
    /// The outlet (base-level) set: coast plus, on a partial tile, the edge.
    pub outlet: Vec<bool>,
    /// Reference area (m²) $A_0$ that sets $\chi$'s scale.
    pub a0_m2: f32,
    /// Non-outlet cells that reached no base level (a routing defect if > 0).
    pub unrouted: usize,
}

/// See [`Fluvial::response_census`]. All figures over the current surface;
/// response percentiles are over **channelized** cells only (drainage above
/// the stated threshold), Courant over all subaerial non-outlet cells.
#[derive(Debug, Clone, Copy)]
pub struct ResponseCensus {
    pub subaerial: usize,
    pub channel_cells: usize,
    /// Largest subaerial drainage accumulation, in **median-cell-area units** —
    /// "how many cells' worth of runoff does the biggest catchment gather".
    /// The drainage-integration number: fragmented radial nets read ~tens,
    /// integrated basins read ~hundreds and up.
    pub max_catchment_cells: f32,
    pub courant_p50: f32,
    pub courant_max: f32,
    pub response_epochs_p50: f32,
    pub response_epochs_p90: f32,
    pub response_epochs_max: f32,
}

/// Channelization thresholds for [`DrainageSurface`], in multiples of the tile's
/// **median cell area** — i.e. "this cell drains at least N cells' worth of
/// runoff". An instrument choice, stated, not a physics claim: there is no
/// drainage area at which a channel *is*, and the three rungs exist so a reader
/// sees the network's shape rather than one threshold's answer.
pub const CHANNEL_THRESHOLD_CELLS: [f32; 3] = [10.0, 100.0, 1000.0];

/// Summary of a [`DrainageSurface`]. Areas in m²; every count is over the tile
/// the surface came from. Subaerial-only where it says so.
#[derive(Debug, Clone, Copy)]
pub struct DrainageStats {
    /// Cells above derived sea on the *unfilled* surface.
    pub subaerial: usize,
    /// `nx²` — the denominator for `subaerial`.
    pub cells: usize,
    /// Σ over subaerial cells of `cell_area × precip_weight` (m²) — the runoff
    /// the land collects, and the denominator for basin shares.
    pub land_runoff_m2: f64,
    pub median_cell_area_m2: f32,
    /// Largest MFD drainage area on the tile (m²) — the fan's trunk.
    pub max_mfd_m2: f32,
    /// Largest D8 accumulation on the tile (m²) — the thread's trunk.
    pub max_d8_m2: f32,
    /// `max_d8 / max_mfd`. **1.0 would mean the fan concentrates as tightly as a
    /// thread; larger means the fan has smeared the trunk's discharge sideways.**
    /// This is the fan of `#obs-cube-locked-kernel-bias` FE(1) measured on live
    /// terrain rather than on a cone.
    pub spread_ratio: f32,
    /// Discharge-weighted count of downhill neighbours receiving MFD flow. 1.0
    /// is a thread; the theoretical max is 8.
    pub mean_out_degree: f32,
    /// Subaerial cells whose MFD area clears each [`CHANNEL_THRESHOLD_CELLS`] rung.
    pub channel_cells_mfd: [usize; 3],
    /// The same rungs under D8 accumulation.
    pub channel_cells_d8: [usize; 3],
    /// Distinct terminal outlets reached by subaerial cells — the basin count.
    /// **Read this with the outlet policy in hand:** a partial tile makes every
    /// perimeter cell an outlet ([`Fluvial::outlets`]), so a fully subaerial
    /// $64^2$ tile floors at 252 basins for reasons of tiling, not geography.
    /// [`Self::largest_basin_share`] and [`Self::basins_for_half`] are the
    /// integration numbers; this one is mostly a check on that policy.
    pub basins: usize,
    /// Longest run of *identical* D8 flow direction anywhere in the channel
    /// network (links, so 1 = every step turns). A meandering river turns; a
    /// long dead-straight run is a lattice artifact rather than a landform, and
    /// it is the defect a viewer's eye catches first in a painted network.
    pub straight_run_max: usize,
    /// Median straight-run length over channelized cells (rung 1 of
    /// [`CHANNEL_THRESHOLD_CELLS`]).
    pub straight_run_p50: usize,
    /// Of the channel cells sitting on a run of 8 or more identical steps, the
    /// fraction whose cell was **raised by the Priority-Flood fill**. This
    /// discriminates the two candidate causes: near 1 says the straight runs are
    /// the $\varepsilon$-gradient orienting flats (a fill artifact, and a
    /// directional one the residual table does not yet name); near 0 says they
    /// are the router picking lattice axes on real slope
    /// (`#obs-cube-locked-kernel-bias` FE(1)).
    pub straight_in_fill_frac: f32,
    /// Channel cells on a run of 8 or more — the denominator of the above.
    pub straight_cells: usize,
    /// Share of land runoff collected by the single largest basin. **The
    /// integration number**: near 1 is one trunk draining the tile, near 0 is
    /// many small disconnected catchments.
    pub largest_basin_share: f32,
    /// How many basins it takes to cover half the land runoff (1 = one dominant).
    pub basins_for_half: usize,
    /// Cells the Priority-Flood fill had to raise by more than 1 m.
    pub depression_cells: usize,
    /// Σ of that raise × cell area (m³) — the **geometric capacity** of the
    /// surface's closed depressions, filled to their spill points. Not a lake
    /// volume: no evaporation, inflow, seepage or residence time is in this
    /// account, and an endorheic basin under a dry climate holds far less.
    pub depression_volume_m3: f64,
    pub deepest_depression_m: f32,
}

/// The drainage field of one stored stage, recomputed on demand — see
/// [`Fluvial::drainage_surface`]. Row-major `nx × nx`, matching the tile.
pub struct DrainageSurface {
    pub nx: usize,
    /// MFD drainage area (m²), the live kernel's own field. **Diffused** — see
    /// [`Fluvial::drainage_surface`] before painting it as a river.
    pub mfd: Vec<f32>,
    /// D8 single-receiver accumulation (m²) down the same tree. Concentrates
    /// into threads; carries D8's grid-alignment artifact; **not** what the
    /// kernel erodes with.
    pub d8: Vec<f32>,
    /// D8 receiver index per cell; outlets point to themselves.
    pub recv: Vec<usize>,
    /// The depression-filled surface the routing was derived on — the heights
    /// water sees, which are not the heights the store holds.
    pub filled_h: Vec<f32>,
    /// `filled_h − stored h` (m): depression capacity depth, **ε included**. See
    /// [`DrainageStats::depression_volume_m3`] for what this is not, and
    /// [`Self::standing_water`] for the field that is water.
    pub fill_depth: Vec<f32>,
    /// **Standing-water depth (m): where water stands.** The true spill level
    /// minus the stored bed, with the flat-orienting ε *excluded* — the second
    /// return of [`Fluvial::fill_depressions`], which `erode` already consumes
    /// (no incision under water; lake-trap deposition) and which every reader
    /// path used to discard.
    ///
    /// It is the field to paint as a lake, and it differs from [`Self::fill_depth`]
    /// in kind, not only in magnitude: **one standing body shares one spill
    /// float**, so `bed + standing_water` is bit-identical across a lake and its
    /// surface is exactly level, while `bed + fill_depth` is a tilted sheet.
    /// Measured (`examples/lake_surface_probe` B): on a flat shelf with no
    /// depression anywhere, `fill_depth` reports 4418 of 9216 cells wet and
    /// 0.06 km³ of water that cannot exist; `standing_water` reports zero.
    ///
    /// **What it assumes, declared:** the *wet limit*. A depression stands full to
    /// its spill point, which is the hydrologic steady state when net supply is
    /// positive — it carries no water balance, so it cannot produce an endorheic
    /// basin standing below its sill (the Caspian class,
    /// `#form-derived-sea-level` Working Notes). Volume-limited filling is a
    /// further rung and needs a P−E field this project does not yet own.
    ///
    /// **What it cannot express at all:** a landlocked basin whose floor dips
    /// below derived sea level. [`Fluvial::outlets`] classifies sea by elevation
    /// threshold rather than connectivity, so such a basin is already an outlet
    /// and holds nothing here.
    pub standing_water: Vec<f32>,
    pub stats: DrainageStats,
}

impl DrainageSurface {
    /// Discharge in units of "cells drained" — the field divided by the tile's
    /// median cell area, which is what the channel thresholds are stated in and
    /// the only form in which two tiles at different levels compare.
    pub fn in_cells(&self, field: &[f32]) -> Vec<f32> {
        let a = self.stats.median_cell_area_m2.max(1.0);
        field.iter().map(|v| v / a).collect()
    }
}

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
        // Face extent at this level — halo windows for edge tiles can ask past
        // the chart. Clamp rather than panic: true cube-edge resampling for
        // d≥2 is still open (`#form-same-level-halo-exchange` scope; `#form-cellid-chunk-patch`).
        let face_n = 1u32 << level;
        let last = face_n.saturating_sub(1);
        let mut h = vec![0.0f32; nx * nx];
        let mut cell_area = vec![0.0f32; nx * nx];
        let mut centers = vec![[0.0f64; 3]; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let gi = oi.saturating_add(x as u32).min(last);
                let gj = oj.saturating_add(y as u32).min(last);
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
            edge: Self::inferred_edge_contract(level, oi, oj, nx),
        }
    }

    /// The contract geometry implies — a whole cube face (`oi = oj = 0`,
    /// `nx = 2^level`) is a no-flux wall, anything smaller is edge sinks. This
    /// *is* the policy that was in force before the contract was nameable, and
    /// it stays the default so that naming it changes no world.
    pub fn inferred_edge_contract(level: u8, oi: u32, oj: u32, nx: usize) -> EdgeContract {
        if (oi, oj) == (0, 0) && nx == 1usize << level {
            EdgeContract::NoFluxWall
        } else {
            EdgeContract::BaseLevelSink
        }
    }

    /// Declare this field's boundary contract explicitly, overriding the
    /// geometric inference. **Callers reached through a complete key must not
    /// use this** until the contract is in that key — today it is an instrument
    /// affordance, which is why nothing under `query.rs` calls it.
    pub fn set_edge_contract(&mut self, edge: EdgeContract) {
        self.edge = edge;
    }

    /// This field's boundary contract.
    pub fn edge_contract(&self) -> EdgeContract {
        self.edge
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

    /// The per-cell rock-uplift rate (m/epoch) this run is carving against — read
    /// access for instruments that need the driver to say what the surface
    /// *should* look like (see [`Fluvial::chi_profile`]). Zeros until
    /// [`Fluvial::set_uplift_rate`] supplies the nomos's field.
    pub fn uplift_rate(&self) -> &[f32] {
        &self.uplift_rate
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

    /// Outlets: coast (`h ≤ sea`) always; **tile-edge sinks under
    /// [`EdgeContract::BaseLevelSink`]**, which is what geometry infers for any
    /// window short of a whole cube face.
    ///
    /// Edge outlets are the principled incomplete-tile base level until flux-BC
    /// (plan Phase-3), and their price is measured
    /// (`#obs-tile-outlets-grade-away-the-basins`). A **full face** takes
    /// [`EdgeContract::NoFluxWall`] instead, because there those edges *are* cube
    /// seams and treating them as sinks carves artificial trenches around every
    /// face (measured ~300 m edge–interior gap after 20 epochs) — sphere
    /// continuity from the prior would be destroyed.
    fn outlets(&self) -> Vec<bool> {
        let nx = self.nx;
        let sea = sea_level::derived_sea_level_m(self.seed) as f32;
        let sinks = self.edge == EdgeContract::BaseLevelSink;

        // **The sea is where the ocean reaches, not everywhere below its level.**
        // Being under the datum makes a cell *submerged*; it makes it *ocean* only
        // if the ocean can get there. A below-datum basin rimmed by dry land is a
        // lake, and classifying it as sea told Priority-Flood it was already a
        // drain — so it held no water, at any settle length, under any halo depth,
        // with or without a water balance. That was the mechanism under the
        // Caspian gap ( #form-derived-sea-level Working Notes).
        //
        // Connectivity is computed here rather than modelled: a below-datum cell
        // is ocean if it reaches the domain boundary through below-datum cells,
        // because past that boundary lies the rest of a planet that is ~95%
        // submerged. Eight-connected, matching [`NEIGHBORS`] — the same
        // neighbourhood flow itself uses, so a strait water can cross is a strait
        // this agrees is open. Deterministic: a boolean reachability set over a
        // Vec stack, independent of visit order.
        //
        // **Declared scope.** This is as honest as the *window* is wide. An
        // enclosed sea larger than the drawn domain touches the boundary and is
        // read as ocean; only a whole-face domain adjudicates the planet's real
        // basins. Prior art frames the ocean the same way — "a designated sink
        // region or the map edge" (Barnes/Callaghan/Wickert 2021, Fill-Spill-Merge;
        // `msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md`).
        let submerged = |i: usize| self.h[i] <= sea;
        let mut ocean = vec![false; nx * nx];
        let mut stack: Vec<usize> = Vec::new();
        for y in 0..nx {
            for x in 0..nx {
                if Self::is_edge(nx, x, y) {
                    let i = y * nx + x;
                    if submerged(i) && !ocean[i] {
                        ocean[i] = true;
                        stack.push(i);
                    }
                }
            }
        }
        while let Some(i) = stack.pop() {
            let (x, y) = (i % nx, i / nx);
            for (dx, dy) in NEIGHBORS {
                let (xp, yp) = (x as i32 + dx, y as i32 + dy);
                if xp < 0 || yp < 0 || xp >= nx as i32 || yp >= nx as i32 {
                    continue;
                }
                let j = yp as usize * nx + xp as usize;
                if !ocean[j] && submerged(j) {
                    ocean[j] = true;
                    stack.push(j);
                }
            }
        }

        let mut out = vec![false; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let edge_sink = sinks && Self::is_edge(nx, x, y);
                out[i] = edge_sink || ocean[i];
            }
        }
        // A walled domain with no coast has no sink at all, and Priority-Flood
        // needs somewhere to flood from: the lowest cell becomes one.
        if !sinks && !out.iter().any(|&o| o) {
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
    ///
    /// Mutates `self.h` into the **routing surface**: every closed depression
    /// raised to its spill point plus an ε-gradient that orients flow across the
    /// resulting flat. Returns the **standing-water depth** each cell acquired —
    /// the *true* spill level minus the original height, with the ε excluded.
    ///
    /// The two are different fields and the difference is load-bearing
    /// (`#obs-lakes-are-routed-over-not-carved-away`). The ε is a numerical
    /// device: it exists so that D8 has a receiver on a flat, and it is
    /// sign-definite manufactured rock wherever it lands. The spill level is a
    /// physical statement: *this is where water stands*. Returning them apart is
    /// what lets `erode` route on the first and put back the second — and lets
    /// the ε be dropped from the bed entirely rather than minted into it.
    ///
    /// The returned depth is exactly zero on any cell at or above its own spill
    /// level, **including every cell of a perfectly flat area** (its spill level
    /// is its own height), so a flat is not mistaken for a lake merely because
    /// the ε had to raise it.
    fn fill_depressions(&mut self, outlets: &[bool]) -> Vec<f32> {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;
        let nx = self.nx;
        const EPS: f32 = 1e-3; // m; tiny vs. relief, enough to orient flats

        struct Cell {
            elev: f32,
            /// The true spill level flow from this cell has had to climb over —
            /// the running max of ORIGINAL heights along the flood path. Carried
            /// beside `elev` and never used to order the heap, so pop order (and
            /// therefore every routed quantity) is bit-identical to a fill that
            /// does not track it.
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
    ///
    /// **Submerged cells do not incise.** `water[i] > 0` means the routing
    /// surface at `i` is a standing-water level, not ground: there is no channel
    /// there, so `E = K A^m S^n` has no slope to act on and the bed beneath is
    /// left alone. A subaerial cell *draining into* a lake keeps its term, and
    /// its receiver's height is the lake surface — which is exactly the local
    /// base level a river entering a lake sees.
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

    /// Davy–Lague deposition: route this epoch's eroded volume down the D8 tree,
    /// laying down `G·Qs/A` per reach; what reaches an outlet is lost to the sea.
    ///
    /// **A lake is a sediment trap, and it fills from the bed up.** Where
    /// `water[i] > 0` the Davy–Lague reach law does not apply — there is no
    /// channel, only still water — so the cell takes *all* the sediment reaching
    /// it, up to the volume that would raise its bed to the water surface, and
    /// passes the surplus on down the same tree the water spills along. Both
    /// `water` and `raise` are debited by what settles, so the deposit lands on
    /// the bed and the lake gets shallower rather than the water level rising.
    /// Trapping efficiency is therefore **1 until the lake is full**, which is
    /// the standard treatment and the reason a basin in this world silts up
    /// instead of persisting forever at its original depth.
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

    /// **Instrument, not law**: the kinematic-wave numbers for this tile's
    /// *current* surface — the two quantities the stream-power literature says
    /// govern whether an epoch count and an epoch size mean anything.
    ///
    /// At $n=1$ an erosional signal climbs the network at celerity $v = K A^m$,
    /// independent of slope (Whipple & Tucker 1999 Eq. 25). Per epoch our lump
    /// is `k_dt` $= K\,\Delta t$, so:
    ///
    /// - **Courant number** per cell: $C = k_{dt}\,A^m / d$ with $d$ the true
    ///   great-circle step to the D8 receiver — how many cells the wave tries
    ///   to cross per epoch. $C \gg 1$ means the transient (exactly what a
    ///   stage chain materializes) is beyond the scheme's accuracy claim
    ///   (Braun & Willett 2013 §7.2); the *endpoint* remains stable regardless.
    /// - **Response epochs** per cell: the cumulative traversal
    ///   $\sum d_i/(k_{dt}A_i^m)$ down the flow path to base level — the
    ///   discrete form of the analytical response time $T_A$ (Gasparini et
    ///   al. 2024 Eq. 9–11), the a-priori answer to "how many epochs does this
    ///   terrain need" that replaces a runtime convergence gate (which is a
    ///   measured no-go here, `#obs-erosion-residual-is-driver-bound`).
    ///
    /// Computed on the surface as it stands (one fill + network derivation,
    /// heights restored — `&mut` only for scratch): the network is treated as
    /// quasi-static, the same assumption the analytic $T_A$ makes, and the
    /// literature's own caveat applies — network rearrangement is what breaks
    /// it (Gasparini et al. 2024). Headwater cells have tiny $A$ and hence huge
    /// nominal traversal, but they are creep-governed, not stream-power-
    /// governed; the channel-restricted figures use `channel_min_cells` × the
    /// tile's median cell area as the channelization threshold (an instrument
    /// choice, stated, not a physics claim).
    pub fn response_census(&mut self, p: &FluvialParams, channel_min_cells: f32) -> ResponseCensus {
        let saved_h = self.h.clone();
        let outlets = self.outlets();
        self.fill_depressions(&outlets);
        let recv = self.receivers(&outlets);
        let order = self.elevation_order();
        self.accumulate_drainage(&order);

        let n = self.nx * self.nx;
        let sea = sea_level::derived_sea_level_m(self.seed) as f32;
        let median_area = {
            let mut a = self.cell_area.clone();
            a.sort_by(f32::total_cmp);
            a[a.len() / 2]
        };
        let channel_area = channel_min_cells * median_area;

        // Ascending elevation ⇒ receiver visited before donor, so one pass
        // accumulates path totals: epochs_to_base[i] = epochs_to_base[recv[i]] + own leg.
        let mut courant = vec![0.0f32; n];
        let mut epochs_to_base = vec![0.0f32; n];
        for &i in &order {
            if outlets[i] || recv[i] == i {
                continue;
            }
            let d = self.dist_m(i, recv[i]);
            let v = p.k_dt * self.drainage[i].powf(p.m); // metres per epoch
            courant[i] = v / d;
            epochs_to_base[i] = epochs_to_base[recv[i]] + if v > 0.0 { d / v } else { f32::INFINITY };
        }

        let mut c_sub: Vec<f32> = Vec::new();
        let mut resp_channel: Vec<f32> = Vec::new();
        let mut subaerial = 0usize;
        let mut max_drainage = 0.0f32;
        for i in 0..n {
            if self.h[i] <= sea || outlets[i] {
                continue;
            }
            subaerial += 1;
            c_sub.push(courant[i]);
            max_drainage = max_drainage.max(self.drainage[i]);
            if self.drainage[i] >= channel_area {
                resp_channel.push(epochs_to_base[i]);
            }
        }
        self.h = saved_h; // restore — the census must not advance the world
        let pct = |v: &mut Vec<f32>, q: f64| -> f32 {
            if v.is_empty() {
                return 0.0;
            }
            v.sort_by(f32::total_cmp);
            v[((v.len() - 1) as f64 * q) as usize]
        };
        ResponseCensus {
            subaerial,
            channel_cells: resp_channel.len(),
            max_catchment_cells: max_drainage / median_area.max(1.0),
            courant_p50: pct(&mut c_sub, 0.5),
            courant_max: pct(&mut c_sub, 1.0),
            response_epochs_p50: pct(&mut resp_channel, 0.5),
            response_epochs_p90: pct(&mut resp_channel, 0.9),
            response_epochs_max: pct(&mut resp_channel, 1.0),
        }
    }

    /// **Instrument, not law**: the drainage field this tile's *current* surface
    /// carries — the thing the kernel computes every epoch and then discards.
    ///
    /// Same shape as [`Self::response_census`] and for the same reason: drainage
    /// is a **pure function of a stored stage** (heights + cell areas + precip
    /// weight), so it is *recomputed* on demand rather than memoized as a store
    /// citizen. One fill + receivers + sort + two accumulations over `nx²` cells
    /// — cheap enough that a view can pay it per tile, and a memo would need its
    /// own nomos version, its own key, and a world rebuild to gain nothing but
    /// the same numbers. (`DECISIONS[memoized-means-store-object]` gives the
    /// condition that would flip this: a cold recompute expensive enough that
    /// views pay it per *frame*.) Heights are restored — the reader must not
    /// advance the world.
    ///
    /// **Both routers are returned, and that is the point.** They answer
    /// different questions and a view must say which it is painting
    /// (`#norm-no-depiction-without-referent`):
    ///
    /// - [`DrainageSurface::mfd`] is the live kernel's own field (Quinn MFD,
    ///   $p=1.0$, spread over *every* downhill neighbour) — the field that
    ///   decides where incision happens. It is a **diffused** surface, not a
    ///   channel thread: on the equiangular cube-sphere its eight directions are
    ///   a sheared quadrature and the fan does not converge
    ///   (`#obs-cube-locked-kernel-bias` FE(1)). Painted narrow, it would be a
    ///   fan drawn thin — a depiction of something the world does not contain.
    /// - [`DrainageSurface::d8`] is single-receiver accumulation down the same
    ///   D8 tree the implicit solve already uses. It concentrates into threads,
    ///   and carries D8's own grid-alignment artifact — the defect MFD was
    ///   adopted to dissolve. It is **not** what the kernel erodes with.
    ///
    /// [`DrainageStats::spread_ratio`] is the gap between them, measured rather
    /// than argued: how much discharge the fan has smeared off the thread.
    ///
    /// **Precip weight is an input, not a default.** [`Self::from_region`]
    /// rebuilds a field from stored heights alone and leaves `precip_weight` at
    /// ones — uniform rain, which is *not* what the kernel ran (climate carries
    /// fated ±50% low-frequency jitter, `crate::climate`). A reader that wants
    /// the discharge the tile was actually carved under must
    /// [`Self::set_precip_weight`] from the climate tile first.
    pub fn drainage_surface(&mut self) -> DrainageSurface {
        let n = self.nx * self.nx;
        let saved_h = self.h.clone();
        let outlets = self.outlets();
        // Both halves are kept. The fill's physical return — where water stands —
        // was dropped here for as long as this reader existed, which left every
        // view and every table running on the ε-augmented raise instead.
        let standing_water = self.fill_depressions(&outlets);
        let recv = self.receivers(&outlets);
        let order = self.elevation_order();
        self.accumulate_drainage(&order);
        let mfd = self.drainage.clone();
        let filled_h = self.h.clone();

        // D8: the same runoff seed routed down the single steepest receiver.
        let mut d8 = vec![0.0f32; n];
        for i in 0..n {
            d8[i] = self.cell_area[i] * self.precip_weight[i];
        }
        for &i in order.iter().rev() {
            let r = recv[i];
            if r != i {
                let carried = d8[i];
                d8[r] += carried;
            }
        }

        // Discharge-weighted MFD out-degree: 1 is a thread, >1 is a fan. Weighted
        // by the discharge each cell passes on, so trunk behaviour dominates
        // rather than the headwater cells that carry almost nothing.
        let nx = self.nx;
        let (mut deg_num, mut deg_den) = (0.0f64, 0.0f64);
        for i in 0..n {
            let (x, y) = (i % nx, i / nx);
            let hi = filled_h[i];
            let mut k = 0usize;
            for (dx, dy) in NEIGHBORS {
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                if hi - filled_h[nyp as usize * nx + nxp as usize] > 0.0 {
                    k += 1;
                }
            }
            if k > 0 {
                deg_num += k as f64 * mfd[i] as f64;
                deg_den += mfd[i] as f64;
            }
        }

        // Depression capacity: what Priority-Flood had to add to make the surface
        // drain. NOT a lake — it is the geometric volume a lake *could* occupy if
        // filled to its spill point, with no evaporation, inflow or seepage in the
        // account. It is however a referent the water nomos does not depend on.
        let fill_depth: Vec<f32> = (0..n).map(|i| filled_h[i] - saved_h[i]).collect();

        let sea = sea_level::derived_sea_level_m(self.seed) as f32;
        let median_area = {
            let mut a = self.cell_area.clone();
            a.sort_by(f32::total_cmp);
            a[a.len() / 2]
        };

        // Terminal outlet per cell (path-following; the D8 tree is acyclic after
        // the fill, so this halts). Basins are counted over LAND runoff only —
        // every submarine cell is its own outlet and would otherwise swamp the
        // fragmentation number with sea floor.
        let mut terminal = vec![usize::MAX; n];
        for start in 0..n {
            if terminal[start] != usize::MAX {
                continue;
            }
            let mut path = Vec::new();
            let mut cur = start;
            loop {
                if terminal[cur] != usize::MAX {
                    break;
                }
                path.push(cur);
                let r = recv[cur];
                if r == cur {
                    terminal[cur] = cur;
                    break;
                }
                cur = r;
            }
            let end = terminal[cur];
            for &p in &path {
                terminal[p] = end;
            }
        }
        let mut basin_runoff: std::collections::BTreeMap<usize, f64> = std::collections::BTreeMap::new();
        let mut land_runoff = 0.0f64;
        let mut subaerial = 0usize;
        let mut depression_cells = 0usize;
        let mut depression_volume_m3 = 0.0f64;
        let mut deepest_depression_m = 0.0f32;
        for i in 0..n {
            if fill_depth[i] > 1.0 {
                depression_cells += 1;
                depression_volume_m3 += fill_depth[i] as f64 * self.cell_area[i] as f64;
                deepest_depression_m = deepest_depression_m.max(fill_depth[i]);
            }
            if saved_h[i] <= sea {
                continue;
            }
            subaerial += 1;
            let r = self.cell_area[i] as f64 * self.precip_weight[i] as f64;
            land_runoff += r;
            *basin_runoff.entry(terminal[i]).or_insert(0.0) += r;
        }
        let mut basins: Vec<f64> = basin_runoff.into_values().collect();
        basins.sort_by(|a, b| b.total_cmp(a));
        let largest_basin_share = if land_runoff > 0.0 { (basins.first().copied().unwrap_or(0.0) / land_runoff) as f32 } else { 0.0 };
        let mut acc = 0.0f64;
        let mut basins_for_half = 0usize;
        for b in &basins {
            if acc >= land_runoff * 0.5 {
                break;
            }
            acc += b;
            basins_for_half += 1;
        }

        // Straight-run length along the D8 tree: how many consecutive links keep
        // the same direction. Ascending elevation visits the receiver first, so
        // one pass suffices. Rivers turn; a lattice artifact does not.
        let dir_of = |i: usize, r: usize| -> (i32, i32) {
            let (x, y) = ((i % nx) as i32, (i / nx) as i32);
            let (rx, ry) = ((r % nx) as i32, (r / nx) as i32);
            (rx - x, ry - y)
        };
        let mut run = vec![0usize; n];
        for &i in &order {
            let r = recv[i];
            if r == i {
                continue;
            }
            let d = dir_of(i, r);
            let rr = recv[r];
            run[i] = if rr != r && dir_of(r, rr) == d { run[r] + 1 } else { 1 };
        }
        let chan_thresh = CHANNEL_THRESHOLD_CELLS[0] * median_area;
        let mut runs: Vec<usize> = (0..n)
            .filter(|&i| saved_h[i] > sea && mfd[i] >= chan_thresh && recv[i] != i)
            .map(|i| run[i])
            .collect();
        let straight_run_max = runs.iter().copied().max().unwrap_or(0);
        runs.sort_unstable();
        let straight_run_p50 = runs.get(runs.len() / 2).copied().unwrap_or(0);
        let long: Vec<usize> = (0..n)
            .filter(|&i| saved_h[i] > sea && mfd[i] >= chan_thresh && run[i] >= 8)
            .collect();
        let straight_cells = long.len();
        let straight_in_fill_frac = if straight_cells > 0 {
            long.iter().filter(|&&i| fill_depth[i] > 0.01).count() as f32 / straight_cells as f32
        } else {
            0.0
        };

        let count_above = |f: &[f32], mult: f32| -> usize {
            let t = mult * median_area;
            (0..n).filter(|&i| saved_h[i] > sea && f[i] >= t).count()
        };
        let max_of = |f: &[f32]| -> f32 { f.iter().cloned().fold(0.0f32, f32::max) };
        let (max_mfd, max_d8) = (max_of(&mfd), max_of(&d8));
        let channel_cells_mfd = CHANNEL_THRESHOLD_CELLS.map(|m| count_above(&mfd, m));
        let channel_cells_d8 = CHANNEL_THRESHOLD_CELLS.map(|m| count_above(&d8, m));

        self.h = saved_h; // restore — a reader must not advance the world
        DrainageSurface {
            nx,
            mfd,
            d8,
            recv,
            filled_h,
            fill_depth,
            standing_water,
            stats: DrainageStats {
                subaerial,
                cells: n,
                land_runoff_m2: land_runoff,
                median_cell_area_m2: median_area,
                max_mfd_m2: max_mfd,
                max_d8_m2: max_d8,
                spread_ratio: if max_mfd > 0.0 { max_d8 / max_mfd } else { 0.0 },
                mean_out_degree: if deg_den > 0.0 { (deg_num / deg_den) as f32 } else { 0.0 },
                channel_cells_mfd,
                channel_cells_d8,
                basins: basins.len(),
                straight_run_max,
                straight_run_p50,
                straight_in_fill_frac,
                straight_cells,
                largest_basin_share,
                basins_for_half,
                depression_cells,
                depression_volume_m3,
                deepest_depression_m,
            },
        }
    }

    /// **Instrument, not law**: the $\chi$ coordinate and the driven-steady-state
    /// profile for this tile's *current* surface — the raw material for a
    /// convergence criterion that is a statement about **shape** rather than
    /// about a per-epoch residual.
    ///
    /// A residual tolerance cannot work here: sustained uplift pins mean
    /// $\lvert\Delta h\rvert$ at the driver's rate forever, and on an inert tile
    /// it is zero for the wrong reason (`#obs-erosion-residual-is-driver-bound`).
    /// Two shape statements survive that, and this returns the material for both.
    ///
    /// **(a) $\chi$-linearity** (Perron & Royden 2013, *ESPL* 38:570–576,
    /// Eqs. 6a/6b). With
    /// $\chi = \int_{x_b}^{x}(A_0/A)^{m/n}\,\mathrm{d}x$ integrated upstream from
    /// base level, a steady state under spatially invariant $U$ and $K$ has $z$
    /// **exactly linear in $\chi$** with slope $(U/K)^{1/n}/A_0^{m/n}$ and
    /// intercept the base-level elevation. Discretely, along the D8 path,
    /// $\chi_i = \chi_{r(i)} + (A_0/A_i)^{m}\,d_i$.
    ///
    /// **(b) The zero-parameter form, which is exact algebra on *this* kernel's
    /// own update.** One epoch adds $U_i$ and then solves
    /// $h_i \leftarrow (h_i + f h_{r})/(1+f)$ with $f = k_{dt}A_i^{m}/d_i$
    /// ([`Fluvial::incise`]). Demanding $h$ unchanged across the epoch gives
    ///
    /// $$h_i - h_{r(i)} = \frac{U_i\,d_i}{k_{dt}\,A_i^{m}}$$
    ///
    /// — a per-cell identity with **no fitted parameter**, integrated up from
    /// each basin's base-level elevation into `z_steady`. It is the continuum
    /// $\chi$ result specialised to the discrete scheme, and it holds under a
    /// *spatially varying* $U$, which the literature form does not. The price is
    /// that it consumes the uplift field, so it is only meaningful when one has
    /// been supplied ([`Fluvial::set_uplift_rate`]).
    ///
    /// Neither statement is affected by the pinned residual: both describe the
    /// surface at one instant. Both fail safe on an inert tile — no channel means
    /// no $\chi$ to integrate along, so the test is *absent* rather than passed.
    ///
    /// $A$ is the kernel's own MFD drainage, not a D8 accumulation: the identity
    /// above is exact only in the $A$ the incision step actually consumes, so
    /// using anything else would test a textbook kernel instead of this one.
    /// Computed on the surface as it stands (one fill + network derivation,
    /// heights restored — `&mut` only for scratch), and the fill is why `h` comes
    /// back in the result: the returned surface is the filled one the network was
    /// derived on, which is what the profile statements are about.
    pub fn chi_profile(&mut self, p: &FluvialParams, a0_m2: f32) -> ChiProfile {
        let saved_h = self.h.clone();
        let outlets = self.outlets();
        self.fill_depressions(&outlets);
        let recv = self.receivers(&outlets);
        let order = self.elevation_order();
        self.accumulate_drainage(&order);

        let n = self.nx * self.nx;
        let mut chi = vec![0.0f32; n];
        let mut z_steady = vec![0.0f32; n];
        let mut basin = vec![u32::MAX; n];
        let mut unrouted = 0usize;
        // Ascending elevation ⇒ receiver before donor, so one pass integrates
        // both quantities up every flow path (the `response_census` pattern).
        for &i in &order {
            if outlets[i] || recv[i] == i {
                z_steady[i] = self.h[i];
                if outlets[i] {
                    basin[i] = i as u32;
                } else {
                    unrouted += 1;
                }
                continue;
            }
            let r = recv[i];
            let d = self.dist_m(i, r);
            let a = self.drainage[i].max(f32::MIN_POSITIVE);
            let am = a.powf(p.m);
            chi[i] = chi[r] + (a0_m2.powf(p.m) / am) * d;
            let f = p.k_dt * am / d;
            z_steady[i] =
                z_steady[r] + if f > 0.0 { self.uplift_rate[i] / f } else { f32::INFINITY };
            basin[i] = basin[r];
        }
        let filled = std::mem::replace(&mut self.h, saved_h); // restore — the profile must not advance the world
        ChiProfile {
            chi,
            z_steady,
            basin,
            h: filled,
            drainage: self.drainage.clone(),
            outlet: outlets,
            a0_m2,
            unrouted,
        }
    }

    /// Run the full pipeline for `p.epochs`, tracking the last epoch's mean |Δh|.
    ///
    /// **Routing happens on the filled surface; the bed keeps its holes.** Every
    /// epoch fills depressions to derive a drainage network — flow must be able
    /// to cross a lake and leave by its spill point to route at all — and then
    /// puts the raise back before the epoch ends, so a closed basin survives in
    /// the stored bed as a closed basin. What the water does inside it is the
    /// water nomos's business; what erosion owes it is a hole to sit in
    /// (`#obs-lakes-are-routed-over-not-carved-away`).
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
            // The routing surface: the bed with every depression raised to its
            // spill point and an ε across the flats. `bed` is what the store is
            // owed back; `raise` is the whole difference (spill fill AND ε);
            // `water` is the physical half of it.
            let bed = self.h.clone();
            let mut water = self.fill_depressions(&outlets);
            let mut raise: Vec<f32> = self.h.iter().zip(bed.iter()).map(|(f, b)| f - b).collect();
            let recv = self.receivers(&outlets);
            let order = self.elevation_order();
            self.accumulate_drainage(&order);
            let before = if p.deposition > 0.0 { Some(self.h.clone()) } else { None };
            self.incise(p, &recv, &order, &water);
            if let Some(b) = before {
                self.deposit(p, &recv, &order, &b, &mut water, &mut raise);
            }
            // Undo the fill. Incision and subaerial deposition have already been
            // written into `self.h`; lake sedimentation was written into `raise`.
            // What is left after this line is the real bed — depressions intact,
            // and no ε-gradient rock minted into it.
            for (h, r) in self.h.iter_mut().zip(raise.iter()) {
                *h -= *r;
            }
            // Talus and creep move ROCK, so they run on the rock surface, not on
            // a water level: an unfilled bed is what a slope failure and a soil
            // flux see. (They are mass-conserving redistributions either way —
            // running them on the filled surface would silently make the fill's
            // manufactured rock real.)
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

    /// Ordinary least squares of `y` on `x`; returns `(slope, intercept, rms residual)`.
    fn ols(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
        let n = x.len() as f64;
        let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
        let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
        let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
        let s = if sxx > 0.0 { sxy / sxx } else { 0.0 };
        let c = my - s * mx;
        let ss: f64 = x.iter().zip(y).map(|(a, b)| (b - (c + s * a)).powi(2)).sum();
        (s, c, (ss / n).sqrt())
    }

    /// The load-bearing claim: run the incision solve against sustained uplift
    /// long enough and the surface **approaches** the profile `chi_profile`
    /// predicts, with the fitted χ slope recovering $U/(k_{dt}A_0^{m})$ — while
    /// the per-epoch $\lvert\Delta h\rvert$ is doing nothing a tolerance could
    /// read ( #obs-erosion-residual-is-driver-bound ).
    ///
    /// Deposition, talus and creep are switched off here on purpose: the
    /// identity is a balance between uplift and the incision solve alone, and it
    /// is *quantitatively* wrong with the other operators on — the live default
    /// composition settles ~1.6× steeper, which is a finding for a probe and a
    /// segment, not something to hide inside a unit test's parameters
    /// ( #detail-erosion-composition FE(3) lists the operators).
    ///
    /// A small dome so the test is cheap in a debug build; the assertions are
    /// ratios against this same landscape's own epoch-0 state, so none of them
    /// can be re-tuned into a false green.
    #[test]
    fn the_surface_approaches_the_predicted_chi_profile_under_sustained_uplift() {
        let p = FluvialParams {
            epochs: 1,
            deposition: 0.0,
            diffusivity_m2: 0.0,
            max_slope: 1.0e6,
            ..Default::default()
        };
        let uplift = 0.5f32;
        // A 40² dome, small enough that its channel network crosses in a few
        // hundred epochs (`Fluvial::response_census` is the a-priori form).
        let dome = || {
            let seed = 0u64;
            let sea = crate::sea_level::derived_sea_level_m(seed);
            let (level, oi, oj, nx) = (19u8, 108_500u32, 186_350u32, 40usize);
            let mut f = Fluvial::from_surface(seed, Face::ZPos, level, oi, oj, nx, |c| {
                let (_, i, j, _) = c.to_face_ij();
                let (cx, cy) = (
                    i.saturating_sub(oi) as f64 - nx as f64 / 2.0,
                    j.saturating_sub(oj) as f64 - nx as f64 / 2.0,
                );
                sea + 800.0 - 0.05 * (cx * cx + cy * cy)
            });
            f.set_uniform_uplift(uplift);
            f
        };

        // (normalized zero-parameter residual, median fitted χ slope) over the
        // channelized cells of every basin with enough of them to fit.
        let measure = |f: &mut Fluvial| -> (f64, Option<f64>, usize) {
            let a0 = f.cell_area[f.cell_area.len() / 2];
            let prof = f.chi_profile(&p, a0);
            assert_eq!(prof.unrouted, 0, "every non-outlet cell must reach a base level");
            let mut med = f.cell_area.clone();
            med.sort_by(f32::total_cmp);
            let thresh = 10.0 * med[med.len() / 2];
            let mut by_basin = std::collections::BTreeMap::<u32, Vec<usize>>::new();
            for i in 0..prof.chi.len() {
                if prof.basin[i] != u32::MAX && prof.chi[i] > 0.0 && prof.drainage[i] >= thresh {
                    by_basin.entry(prof.basin[i]).or_default().push(i);
                }
            }
            let all: Vec<usize> = by_basin.values().flatten().copied().collect();
            assert!(all.len() >= 32, "too few channel cells to judge ({})", all.len());
            let hs: Vec<f32> = all.iter().map(|&i| prof.h[i]).collect();
            let relief = (hs.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
                - hs.iter().cloned().fold(f32::INFINITY, f32::min)) as f64;
            let rms0 = (all
                .iter()
                .map(|&i| ((prof.h[i] - prof.z_steady[i]) as f64).powi(2))
                .sum::<f64>()
                / all.len() as f64)
                .sqrt();
            let mut slopes: Vec<f64> = by_basin
                .values()
                .filter(|v| v.len() >= 8)
                .map(|cells| {
                    let x: Vec<f64> = cells.iter().map(|&i| prof.chi[i] as f64).collect();
                    let y: Vec<f64> = cells.iter().map(|&i| prof.h[i] as f64).collect();
                    ols(&x, &y).0
                })
                .collect();
            slopes.sort_by(f64::total_cmp);
            // At epoch 0 the dome has no basin with enough channel cells to fit:
            // the shape test is *absent*, not passed — the fail-safe that makes
            // this criterion usable on inert tiles at all.
            let median = slopes.get(slopes.len() / 2).copied();
            (rms0 / relief.max(1e-9), median, all.len())
        };

        let (start, start_slope, _) = measure(&mut dome());
        assert!(start_slope.is_none(), "the undissected dome has no fittable channel — the test would be vacuous if it did");
        let mut settled = dome();
        settled.erode(&FluvialParams { epochs: 400, ..p.clone() });
        let (end, slope, _) = measure(&mut settled);
        let slope = slope.expect("the settled landscape must have a fittable channel network");

        assert!(
            end < 0.1 * start,
            "the surface must approach the predicted steady profile \
             (normalized residual {end:.4} after 400 epochs vs {start:.4} at epoch 0)"
        );
        let a0 = settled.cell_area[settled.cell_area.len() / 2];
        let predicted = uplift as f64 / (p.k_dt as f64 * (a0 as f64).powf(p.m as f64));
        assert!(
            (slope / predicted - 1.0).abs() < 0.15,
            "the settled χ slope must recover U/(k_dt·A₀^m): fitted {slope:.4}, predicted {predicted:.4}"
        );
    }

    /// The literature form (Perron & Royden 2013): under uniform $U$ and $K$ a
    /// steady profile is **linear in χ** with slope $U/(k_{dt}A_0^{m})$. Asserted
    /// on the predicted steady profile, together with the two known-bads the
    /// criterion has to reject — a knickpoint (right slope, wrong shape) and a
    /// doubled slope (right shape, wrong rate). Both halves must be checked or
    /// the criterion passes landscapes it should convict
    /// ( #norm-probe-sensitivity FE(2)).
    #[test]
    fn chi_linearity_passes_the_steady_profile_and_convicts_both_known_bads() {
        let p = FluvialParams {
            epochs: 1,
            deposition: 0.0,
            diffusivity_m2: 0.0,
            max_slope: 1.0e6,
            ..Default::default()
        };
        let uplift = 0.5f32;
        let mut f = small();
        f.set_uniform_uplift(uplift);
        let a0 = f.cell_area[f.cell_area.len() / 2];
        let prof = f.chi_profile(&p, a0);

        // One basin's channel cells: the largest by cell count, channels being
        // cells whose drainage exceeds ten median cell areas.
        let mut area = prof.drainage.clone();
        let mut med = f.cell_area.clone();
        med.sort_by(f32::total_cmp);
        let thresh = 10.0 * med[med.len() / 2];
        area.sort_by(f32::total_cmp);
        let mut counts = std::collections::BTreeMap::<u32, Vec<usize>>::new();
        for i in 0..prof.chi.len() {
            if prof.basin[i] != u32::MAX && prof.chi[i] > 0.0 && prof.drainage[i] >= thresh {
                counts.entry(prof.basin[i]).or_default().push(i);
            }
        }
        let cells = counts.values().max_by_key(|v| v.len()).expect("no channelized basin").clone();
        assert!(cells.len() >= 8, "largest basin has too few channel cells ({})", cells.len());

        let chi: Vec<f64> = cells.iter().map(|&i| prof.chi[i] as f64).collect();
        let z: Vec<f64> = cells.iter().map(|&i| prof.z_steady[i] as f64).collect();
        let relief = z.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            - z.iter().cloned().fold(f64::INFINITY, f64::min);
        let predicted = uplift as f64 / (p.k_dt as f64 * (a0 as f64).powf(p.m as f64));

        let (slope, _, rms) = ols(&chi, &z);
        assert!(
            rms < 0.01 * relief,
            "the steady profile must be linear in χ (rms {rms:.3} m vs relief {relief:.1} m)"
        );
        assert!(
            (slope / predicted - 1.0).abs() < 0.02,
            "fitted χ slope {slope:.4} must recover U/(k_dt·A₀^m) = {predicted:.4}"
        );

        // Known-bad 1 — a knickpoint: the same profile with everything above the
        // median χ lifted. Same slope at both ends, wrong shape.
        let mid = {
            let mut s = chi.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        };
        let kz: Vec<f64> =
            z.iter().zip(&chi).map(|(v, c)| if *c > mid { v + 0.2 * relief } else { *v }).collect();
        let (_, _, krms) = ols(&chi, &kz);
        assert!(
            krms > 0.05 * relief,
            "a knickpoint must fail the shape test (rms {krms:.3} m vs relief {relief:.1} m)"
        );

        // Known-bad 2 — the right shape at the wrong rate: a landscape twice as
        // steep in χ passes the residual test and must be caught by the slope.
        let wz: Vec<f64> = chi.iter().zip(&z).map(|(c, v)| v + slope * c).collect();
        let (wslope, _, wrms) = ols(&chi, &wz);
        assert!(wrms < 0.01 * relief, "the doubled profile is still linear (rms {wrms:.3} m)");
        assert!(
            (wslope / predicted - 1.0).abs() > 0.5,
            "a doubled χ slope must fail the rate test (fitted {wslope:.4} vs predicted {predicted:.4})"
        );
    }

    /// The reader must be a READER: heights come back bit-identical, so a view
    /// or probe calling it cannot advance the world (`#form-core-view-wall`).
    /// The fill genuinely moves them mid-call — `filled_h` differing from the
    /// restored `h` is what proves the restore is doing work rather than the
    /// call being inert.
    #[test]
    fn drainage_surface_restores_the_world_it_read() {
        let mut f = small();
        f.erode(&FluvialParams { epochs: 6, ..Default::default() });
        // Gouge a pit, so the fill has something real to move. On a surface
        // already graded to its outlets the fill is inert (see
        // `depression_capacity_fires_on_a_pit_and_not_on_a_graded_dome`), and a
        // restore test over an inert call proves nothing.
        let nx = f.nx;
        for y in (nx / 2 - 4)..(nx / 2 + 4) {
            for x in (nx / 2 - 4)..(nx / 2 + 4) {
                f.h[y * nx + x] -= 200.0;
            }
        }
        let before = f.h.clone();
        let d = f.drainage_surface();
        assert_eq!(f.h, before, "the reader advanced the world");
        assert!(
            d.filled_h.iter().zip(before.iter()).any(|(a, b)| (a - b).abs() > 1e-4),
            "the fill changed nothing, so the restore proves nothing — this probe would pass on an inert call"
        );
        // And it is idempotent: reading twice gives the same field.
        let d2 = f.drainage_surface();
        assert_eq!(d.mfd, d2.mfd, "two reads of one surface disagreed");
    }

    /// **A lake surface is level; the raise is not.** Every cell of one standing
    /// body shares one spill float, so `bed + standing_water` must be a single
    /// bit-identical value across the body — that is what makes it a water
    /// surface rather than a shaded region. `bed + fill_depth` carries the
    /// flat-orienting ε and is a tilted sheet, and this convicts the difference
    /// rather than asserting it: if the two fields were ever wired to the same
    /// vector, the second half of this test would fail.
    #[test]
    fn a_lake_surface_is_exactly_level_and_the_raise_is_not() {
        let mut f = small();
        let nx = f.nx;
        for y in (nx / 2 - 6)..(nx / 2 + 6) {
            for x in (nx / 2 - 6)..(nx / 2 + 6) {
                f.h[y * nx + x] -= 300.0;
            }
        }
        let bed = f.h.clone();
        let d = f.drainage_surface();

        let wet: Vec<usize> = (0..nx * nx).filter(|&i| d.standing_water[i] > 0.0).collect();
        assert!(!wet.is_empty(), "the gouged pit holds no water — nothing to test");

        let distinct = |field: &[f32]| -> usize {
            let mut v: Vec<u32> = wet.iter().map(|&i| (bed[i] + field[i]).to_bits()).collect();
            v.sort_unstable();
            v.dedup();
            v.len()
        };
        assert_eq!(
            distinct(&d.standing_water),
            1,
            "a standing body must have ONE surface height, bit-identical across every cell"
        );
        assert!(
            distinct(&d.fill_depth) > 1,
            "fill_depth carries the ε and cannot be level — if it is, the two fields have been \
             conflated and the physical one is no longer distinguishable"
        );
    }

    /// **The wet limit reports nothing where nothing can stand.** A perfectly flat
    /// shelf above sea level has no closed depression: every cell *is* its own
    /// spill level. The ε still raises it (that is the ε's job — giving D8 a
    /// receiver on a flat), so `fill_depth` reports a large wet area that holds no
    /// water. Measured on the probe's own construction B: 4418 of 9216 cells and
    /// 0.06 km³ of water that cannot exist (`examples/lake_surface_probe`).
    ///
    /// This is the tripwire for the reader wiring: if `standing_water` is ever
    /// sourced from the raise again, the first assert fires.
    #[test]
    fn standing_water_is_empty_on_a_flat_that_the_epsilon_still_raises() {
        let seed = 0u64;
        let sea = crate::sea_level::derived_sea_level_m(seed) as f32;
        let (level, oi, oj, nx) = (13u8, 6512u32, 1552u32, 96usize);
        let mut f = Fluvial::from_surface(seed, Face::XPos, level, oi, oj, nx, |_| 0.0);
        for y in 0..nx {
            for x in 0..nx {
                // Left half dead flat, right half a monotone ramp to the edge.
                f.h[y * nx + x] =
                    if x < nx / 2 { sea + 500.0 } else { sea + 500.0 - (x - nx / 2) as f32 };
            }
        }
        let d = f.drainage_surface();
        assert!(
            d.standing_water.iter().all(|&w| w == 0.0),
            "water standing on a surface with no depression: {} cells",
            d.standing_water.iter().filter(|&&w| w > 0.0).count()
        );
        // And the ε really is there, so the first assert is not passing by virtue
        // of an inert fill.
        assert!(
            d.fill_depth.iter().filter(|&&r| r > 0.0).count() > nx * nx / 4,
            "the ε raised almost nothing here, so this construction no longer discriminates"
        );
    }

    /// **A landlocked basin below the datum is told it is the sea.** [`Self::outlets`]
    /// classifies ocean by *elevation threshold* rather than connectivity
    /// (`out[i] = edge_sink || h[i] <= sea`), and [`Self::fill_depressions`] seeds
    /// its heap from exactly that set — so a crater whose floor dips below derived
    /// sea level, with a rim of dry land all the way round and no connection to any
    /// ocean, is marked closed at step zero and can hold no water. No settle, no
    /// volume-limited fill and no halo depth can repair it; the classification is
    /// upstream of all of them. This is the mechanism under the Caspian gap
    /// (`#form-derived-sea-level` Working Notes, Joseph 2026-07-28).
    ///
    /// Prior art names the fix: the ocean is *"a designated sink region or the map
    /// edge"* (Barnes 2021, Fill–Spill–Merge), i.e. seeded by connectivity, and
    /// Priority-Flood from real edge/ocean cells then computes that connectivity
    /// for free — the spill field needs no algorithmic change.
    /// See `msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md`.
    ///
    /// This test was written red against the threshold classification and inverted
    /// when connectivity landed, which is what convicts the repair rather than the
    /// wording of it (`#norm-caught-disciplines-become-mechanisms`).
    #[test]
    fn a_landlocked_below_datum_basin_is_misclassified_as_ocean() {
        let seed = 0u64;
        let sea = crate::sea_level::derived_sea_level_m(seed) as f32;
        let (level, oi, oj, nx) = (13u8, 6512u32, 1552u32, 96usize);
        let mut f = Fluvial::from_surface(seed, Face::XPos, level, oi, oj, nx, |_| 0.0);
        // Dry land everywhere, 400 m above the waterline...
        for h in f.h.iter_mut() {
            *h = sea + 400.0;
        }
        // ...a genuine ocean along the left edge, 500 m below it. This is the
        // control that keeps the test about connectivity: real sea exists in the
        // domain, so the "walled domain with no coast" fallback below (which makes
        // the single LOWEST cell an outlet, and would otherwise drain the crater as
        // the deepest thing present) does not fire.
        for y in 0..nx {
            for x in 0..8 {
                f.h[y * nx + x] = sea - 500.0;
            }
        }
        // ...and one crater whose floor is 300 m below the datum, enclosed by dry
        // land in every direction. Nothing here is ocean by any physical reading:
        // the sea cannot reach it.
        let (c0, c1) = (nx / 2, nx / 2 + 16);
        for y in c0..c1 {
            for x in c0..c1 {
                f.h[y * nx + x] = sea - 300.0;
            }
        }
        // A wall contract, so the rim is not a blanket sink — the only thing that
        // can make an interior cell an outlet is its relationship to the sea.
        f.set_edge_contract(EdgeContract::NoFluxWall);
        let d = f.drainage_surface();

        let crater_max = (c0..c1)
            .flat_map(|y| (c0..c1).map(move |x| y * nx + x))
            .fold(0.0f32, |m, i| m.max(d.standing_water[i]));
        assert!(
            crater_max > 0.0,
            "an enclosed crater 300 m below the datum, rimmed by 400 m of dry land, holds no \
             water: its cells were classified ocean by elevation threshold rather than by \
             connectivity to an ocean"
        );
        // And the ocean is not a lake: the connected sea must report no standing
        // water at all, or the repair has merely relabelled everything wet.
        let ocean_max = (0..nx)
            .flat_map(|y| (0..8).map(move |x| y * nx + x))
            .fold(0.0f32, |m, i| m.max(d.standing_water[i]));
        assert_eq!(
            ocean_max, 0.0,
            "the connected ocean is being reported as standing water — a lake field that \
             includes the sea is not a lake field"
        );
    }

    /// Discharge must actually consume the precipitation field. This is the guard
    /// on the honesty note in [`Fluvial::drainage_surface`]: `from_region` leaves
    /// `precip_weight` at ones, so a caller who forgets to supply climate is
    /// reading UNIFORM-rain discharge — and this convicts that the difference is
    /// real rather than cosmetic. A wiring regression that dropped the weight
    /// would make the two runs identical and fail here.
    #[test]
    fn discharge_consumes_the_precipitation_field() {
        let mut f = small();
        f.erode(&FluvialParams { epochs: 6, ..Default::default() });
        let uniform = f.drainage_surface().stats.max_mfd_m2;

        // Rain hard on one half of the tile only — mean-preserving over the tile
        // so this is a redistribution, not more water.
        let nx = f.nx;
        let w: Vec<f32> = (0..nx * nx).map(|i| if i % nx < nx / 2 { 1.8 } else { 0.2 }).collect();
        f.set_precip_weight(w);
        let skewed = f.drainage_surface().stats.max_mfd_m2;

        let rel = (skewed - uniform).abs() / uniform.max(1.0);
        assert!(rel > 0.05, "precip weight moved the trunk by only {:.3}% — is it wired?", 100.0 * rel);
    }

    /// Jacobi exchange is order-independent by construction: every tile erodes
    /// against one frozen snapshot. Re-running the same block twice must agree
    /// bit-for-bit (`#form-same-level-halo-exchange` FE(3);
    /// `#form-depend-by-key-never-latest`).
    #[test]
    fn jacobi_exchange_is_deterministic() {
        let seed = 7u64;
        let face = Face::from_index(2);
        let level = 8u8;
        let (tile_n, epochs) = (12usize, 8u32);
        let schedule = HaloSchedule { depth: 3, cadence: 2, cone_rho: 0 };
        let mk = |oi: i64, oj: i64, nx: usize| {
            let oi_u = oi.max(0) as u32;
            let oj_u = oj.max(0) as u32;
            let mut f = Fluvial::from_surface(seed, face, level, oi_u, oj_u, nx, |c| {
                gen::initial_topography_m(seed, c, level)
            });
            f.set_uplift_rate(crate::uplift::uplift_rate_tile(seed, face, level, oi_u, oj_u, nx));
            f
        };
        let prior = |i: i64, j: i64| {
            let c = CellId::from_face_ij(face, i.max(0) as u32, j.max(0) as u32, level);
            gen::initial_topography_m(seed, c, level) as f32
        };
        let a = carve_region_jacobi_exchange(32, 48, tile_n, 2, 2, epochs, schedule, mk, prior, |_, _| {});
        let b = carve_region_jacobi_exchange(32, 48, tile_n, 2, 2, epochs, schedule, mk, prior, |_, _| {});
        assert_eq!(a.len(), 4);
        for (x, y) in a.iter().zip(b.iter()) {
            assert!(
                x.h.iter().zip(y.h.iter()).all(|(p, q)| p.to_bits() == q.to_bits()),
                "Jacobi region carve must be bit-identical across runs"
            );
        }
    }

    /// The boundary contract must be **inferred exactly as it was before it had a
    /// name**, or naming it silently rekeys every world — `#form-declared-boundary-contract`
    /// FE(2), whose whole point is that the honest first state of a declaration is
    /// the one already in force.
    #[test]
    fn the_inferred_contract_reproduces_the_geometric_policy() {
        // A whole cube face: the walled branch, as `outlets` took it by geometry.
        assert_eq!(Fluvial::inferred_edge_contract(9, 0, 0, 512), EdgeContract::NoFluxWall);
        // Every window short of one, including a face-origin window that is merely
        // too small and a full-width window that is merely offset.
        assert_eq!(Fluvial::inferred_edge_contract(9, 0, 0, 64), EdgeContract::BaseLevelSink);
        assert_eq!(Fluvial::inferred_edge_contract(9, 64, 0, 64), EdgeContract::BaseLevelSink);
        assert_eq!(Fluvial::inferred_edge_contract(13, 640, 5376, 64), EdgeContract::BaseLevelSink);
        // And a constructed field agrees with the free function that set it.
        let f = small();
        assert_eq!(f.edge_contract(), EdgeContract::BaseLevelSink, "a 96-cell window at L19 is a partial tile");
    }

    /// The contract must be able to **fail** — a declaration nothing can convict
    /// is a wish (`#norm-declaration-must-convict`). Carving one window both ways
    /// has to give two different worlds, and the difference has to be at the edge.
    #[test]
    fn the_two_contracts_carve_different_worlds() {
        let p = FluvialParams { epochs: 6, ..Default::default() };

        let mut sink = small();
        assert_eq!(sink.edge_contract(), EdgeContract::BaseLevelSink);
        sink.erode(&p);

        let mut wall = small();
        wall.set_edge_contract(EdgeContract::NoFluxWall);
        wall.erode(&p);

        let nx = sink.nx;
        let (mut edge_gap, mut interior_gap) = (0.0f64, 0.0f64);
        let (mut ec, mut ic) = (0usize, 0usize);
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let d = (sink.h[i] - wall.h[i]).abs() as f64;
                if Fluvial::is_edge(nx, x, y) {
                    edge_gap += d;
                    ec += 1;
                } else {
                    interior_gap += d;
                    ic += 1;
                }
            }
        }
        let (edge_gap, interior_gap) = (edge_gap / ec as f64, interior_gap / ic as f64);
        assert!(
            edge_gap > 1.0,
            "the two contracts must disagree at the boundary they describe; mean |dh| on the edge ring was {edge_gap} m"
        );
        assert!(
            edge_gap > interior_gap,
            "the disagreement must be largest AT the edge ({edge_gap} m) rather than in the interior ({interior_gap} m), \
             or the contract is not what moved"
        );
    }

    /// **A basin survives the epoch that routes over it** — the tripwire for the
    /// bed's capacity to hold water (`#obs-lakes-are-routed-over-not-carved-away`
    /// FE(1)–(2), `#norm-caught-disciplines-become-mechanisms`).
    ///
    /// `Fluvial::erode` fills depressions to derive its network and puts the raise
    /// back before the epoch ends, so a crater gouged into the surface is still
    /// there afterwards, under **either** boundary contract. It is allowed to get
    /// shallower — talus, creep and lake sedimentation all act on it, and a basin
    /// that could never silt up would be its own kind of lie — but it may not
    /// vanish, and it may not vanish by being *raised to its spill point in one
    /// step*, which is what an un-restored Priority-Flood does.
    ///
    /// Reverting the restore fails this test at `depression_cells == 0`. Losing
    /// the incision mask does *not* fail it — the mask's own conviction is
    /// `a_lake_floor_is_not_quietly_planed_by_the_epsilon_gradient`, and the two
    /// are deliberately separate because the two halves of the repair fail in
    /// different sizes.
    #[test]
    fn a_crater_survives_the_epoch_that_routes_over_it() {
        let p = FluvialParams { epochs: 1, ..Default::default() };
        let mut graded = small();
        graded.erode(&FluvialParams { epochs: 6, ..Default::default() });
        let nx = graded.nx;

        let gouge = |f: &mut Fluvial| {
            f.h = graded.h.clone();
            for y in (nx / 2 - 4)..(nx / 2 + 4) {
                for x in (nx / 2 - 4)..(nx / 2 + 4) {
                    f.h[y * nx + x] -= 200.0;
                }
            }
        };

        for contract in [EdgeContract::BaseLevelSink, EdgeContract::NoFluxWall] {
            let mut f = small();
            f.set_edge_contract(contract);
            gouge(&mut f);
            let before = f.drainage_surface().stats;
            assert!(
                before.depression_cells >= 16 && before.deepest_depression_m > 100.0,
                "{contract:?}: the crater must exist before the epoch runs ({} cells, {:.0} m)",
                before.depression_cells,
                before.deepest_depression_m
            );
            f.erode(&p);
            let after = f.drainage_surface().stats;
            assert!(
                after.depression_cells >= 16,
                "{contract:?}: the bed must still hold the crater after an epoch — the fill is a routing device and \
                 the loop restores it; got {} cells at {:.0} m",
                after.depression_cells,
                after.deepest_depression_m
            );
            assert!(
                after.deepest_depression_m > 0.5 * before.deepest_depression_m,
                "{contract:?}: one epoch may silt and slump a basin, not erase it — {:.0} m of a {:.0} m crater left",
                after.deepest_depression_m,
                before.deepest_depression_m
            );
        }
    }

    /// **The fill mints no rock into the bed.** Priority-Flood raises every closed
    /// depression to its spill point and lays an ε-gradient across the flats; both
    /// are sign-definite additions to the height field, and both used to be stored.
    /// Under the repair they are a routing scratch surface that never reaches the
    /// bed, so an epoch run on a pitted surface with the drivers off can only
    /// *remove* rock (incision exports at the boundary) or move it (deposition,
    /// talus, creep) — never add.
    ///
    /// The old composition fails this by roughly the crater's own volume, which is
    /// the unit-scale form of the planet-scale mint that stood at
    /// $\approx 2\times10^{13}\,\mathrm{m^3}$ per L9 face.
    #[test]
    fn an_epoch_over_a_pitted_bed_adds_no_rock() {
        let mut graded = small();
        graded.erode(&FluvialParams { epochs: 6, ..Default::default() });
        let nx = graded.nx;

        let mut f = small();
        f.h = graded.h.clone();
        for y in (nx / 2 - 6)..(nx / 2 + 6) {
            for x in (nx / 2 - 6)..(nx / 2 + 6) {
                f.h[y * nx + x] -= 300.0;
            }
        }
        let volume = |f: &Fluvial| -> f64 {
            f.h.iter().zip(f.cell_area.iter()).map(|(h, a)| *h as f64 * *a as f64).sum()
        };
        let v0 = volume(&f);
        f.set_uniform_uplift(0.0);
        f.erode(&FluvialParams { epochs: 1, ..Default::default() });
        let v1 = volume(&f);
        // Tolerance is f32 summation slop over ~9k cells of ~1e7 m² each, not a
        // physics allowance: the mint this convicts is ~1e11 m³.
        assert!(
            v1 <= v0 + 1e6,
            "an epoch with the drivers off must not add rock: {:.4e} m³ before, {:.4e} m³ after (+{:.3e})",
            v0,
            v1,
            v1 - v0
        );
    }

    /// **Nothing plane a lake floor.** The incision mask's own conviction, and it
    /// has to be a *bit-exactness* claim rather than a magnitude one, because the
    /// quantity it protects is small and would hide inside any tolerance.
    ///
    /// Under water, `E = K A^m S^n` has no slope to act on. What the routing
    /// surface offers instead is the ε-gradient — a numerical device worth
    /// $10^{-3}\,\mathrm{m}$ per cell — multiplied by the drainage area of
    /// everything that drains *into* the lake, which is the largest $A$ anywhere
    /// on the surface. Unmasked, the pit floor therefore erodes at roughly ε per
    /// epoch: invisible in one epoch, a metre over a settle history, and
    /// **oriented along the ε's own flood direction**, so it would write the fill's
    /// measured directional artifact straight into every lake bed.
    ///
    /// With the drivers off and no sediment arriving, a masked floor is untouched
    /// to the last bit. Removing `water[i] > 0.0` from `Fluvial::incise` fails
    /// this immediately.
    #[test]
    fn a_lake_floor_is_not_quietly_planed_by_the_epsilon_gradient() {
        // Drivers off: no deposition (so no siltation), no creep, repose slope
        // high enough that talus cannot reach the floor. Only incision is live.
        let quiet = FluvialParams {
            epochs: 1,
            deposition: 0.0,
            diffusivity_m2: 0.0,
            max_slope: 100.0,
            ..Default::default()
        };
        let mut graded = small();
        graded.erode(&FluvialParams { epochs: 6, ..Default::default() });
        let nx = graded.nx;

        let mut f = small();
        f.set_uniform_uplift(0.0);
        f.h = graded.h.clone();
        for y in (nx / 2 - 6)..(nx / 2 + 6) {
            for x in (nx / 2 - 6)..(nx / 2 + 6) {
                f.h[y * nx + x] -= 300.0;
            }
        }
        let floor: Vec<usize> = {
            let s = f.drainage_surface();
            (0..f.h.len()).filter(|&i| s.fill_depth[i] > 1.0).collect()
        };
        assert!(floor.len() >= 16, "the lake floor must exist to be tested ({} cells)", floor.len());
        let before: Vec<f32> = floor.iter().map(|&i| f.h[i]).collect();
        f.erode(&quiet);
        let after: Vec<f32> = floor.iter().map(|&i| f.h[i]).collect();
        let moved = before.iter().zip(after.iter()).filter(|(a, b)| a != b).count();
        assert_eq!(
            moved,
            0,
            "a submerged bed with no sediment arriving must be bit-identical after an epoch; {moved}/{} cells moved, \
             max |Δh| {:.2e} m — that is the ε-gradient being mistaken for a channel slope",
            floor.len(),
            before
                .iter()
                .zip(after.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, f32::max)
        );
    }

    /// **A lake fills with sediment and then stops** — the deposition semantics the
    /// repair had to choose (`#obs-lakes-are-routed-over-not-carved-away` FE(4)).
    /// A basin fed by an incising catchment silts up monotonically, and its bed
    /// never rises past the spill point that defines the water surface, because the
    /// trap is capped by the remaining capacity and the surplus spills downstream.
    #[test]
    fn a_lake_silts_up_toward_its_spill_point_and_not_past_it() {
        let mut graded = small();
        graded.erode(&FluvialParams { epochs: 6, ..Default::default() });
        let nx = graded.nx;
        let mut f = small();
        f.h = graded.h.clone();
        for y in (nx / 2 - 6)..(nx / 2 + 6) {
            for x in (nx / 2 - 6)..(nx / 2 + 6) {
                f.h[y * nx + x] -= 300.0;
            }
        }
        let spill = {
            let s = f.drainage_surface();
            let i = (0..f.h.len()).max_by(|&a, &b| s.fill_depth[a].total_cmp(&s.fill_depth[b])).unwrap();
            (i, f.h[i] + s.fill_depth[i])
        };
        let (deepest, water_level) = spill;

        let mut depths = Vec::new();
        for _ in 0..8 {
            f.erode(&FluvialParams { epochs: 5, ..Default::default() });
            depths.push(f.drainage_surface().stats.deepest_depression_m);
            assert!(
                f.h[deepest] <= water_level + 1.0,
                "lake sedimentation must stop at the water surface ({water_level:.0} m), not build a mound: bed at {:.0} m",
                f.h[deepest]
            );
        }
        assert!(
            depths.last().unwrap() < depths.first().unwrap(),
            "a basin under an incising catchment must silt up over 40 epochs: {depths:?}"
        );
    }

    /// The depression measure must fire on a real pit and stay quiet on a graded
    /// surface — `#norm-probe-sensitivity`: a capacity number that reported
    /// something everywhere would certify nothing, and one that reported nothing
    /// on a 200 m crater would be the failure it is meant to detect.
    #[test]
    fn depression_capacity_fires_on_a_pit_and_not_on_a_graded_dome() {
        let mut clean = small();
        clean.erode(&FluvialParams { epochs: 6, ..Default::default() });
        let quiet = clean.drainage_surface().stats;
        assert_eq!(
            quiet.depression_cells, 0,
            "a tile graded to its own edge outlets should hold no closed depression, got {}",
            quiet.depression_cells
        );

        // Gouge a crater into the same eroded surface.
        let nx = clean.nx;
        let mut pitted = small();
        pitted.h = clean.h.clone();
        for y in (nx / 2 - 4)..(nx / 2 + 4) {
            for x in (nx / 2 - 4)..(nx / 2 + 4) {
                pitted.h[y * nx + x] -= 200.0;
            }
        }
        let loud = pitted.drainage_surface().stats;
        assert!(loud.depression_cells >= 16, "the crater was not detected ({} cells)", loud.depression_cells);
        assert!(
            loud.deepest_depression_m > 100.0,
            "a 200 m crater must read as a deep depression, got {:.1} m",
            loud.deepest_depression_m
        );
        assert!(loud.depression_volume_m3 > 0.0);
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

/// One tile's kept interior after a Jacobi halo-exchange region carve
/// (`#form-same-level-halo-exchange` FE(2)).
#[derive(Clone, Debug)]
pub struct ExchangedTile {
    pub oi: u32,
    pub oj: u32,
    /// Interior side length `n` (not the enlarged window).
    pub nx: usize,
    pub h: Vec<f32>,
}

/// Carve a rectangular block of same-level tiles under **Jacobi** halo exchange.
///
/// Every `schedule.cadence` epochs, all tile interiors are assembled into one
/// frozen snapshot and every tile's halo is refilled from that snapshot
/// (falling back to `prior` outside the block). Tile order cannot affect the
/// result (`#form-depend-by-key-never-latest`); the probe that measured this is
/// `examples/halo_exchange_probe`.
///
/// Each tile is an `(n+2d)²` window under [`EdgeContract::BaseLevelSink`] on the
/// *window* perimeter (the measured operating form); only the interior `n²` is
/// returned. This is the production path the probe was the instrument for —
/// still region-scoped (not a single-tile pull with a dependency cone).
///
/// `mk_window(oi, oj, nx)` must return a fully driven [`Fluvial`] (surface,
/// uplift, precip) whose origin is `(oi, oj)` and side `nx`. Origins may be
/// slightly negative after halo expansion; the maker is responsible for
/// clamping cell samples the way the shipped path does.
///
/// `on_rung(epochs_reached, interiors)` fires after every cadence chunk (and the
/// final partial chunk), so a staged build can memoize each rung without a
/// second cold settle — `#form-same-level-halo-exchange` FE(7).
pub fn carve_region_jacobi_exchange(
    region_oi: i64,
    region_oj: i64,
    tile_n: usize,
    tiles_i: usize,
    tiles_j: usize,
    epochs: u32,
    schedule: HaloSchedule,
    mut mk_window: impl FnMut(i64, i64, usize) -> Fluvial,
    prior: impl Fn(i64, i64) -> f32,
    mut on_rung: impl FnMut(u32, &[ExchangedTile]),
) -> Vec<ExchangedTile> {
    assert!(tile_n > 0 && tiles_i > 0 && tiles_j > 0, "region must be non-empty");
    assert!(schedule.cadence >= 1, "HaloSchedule.cadence must be ≥ 1 (exchange on)");
    let d = schedule.depth_usize();
    let n_tiles = tiles_i * tiles_j;
    let span_i = tile_n * tiles_i;
    let span_j = tile_n * tiles_j;
    let win = tile_n + 2 * d;

    let mut tiles: Vec<Fluvial> = Vec::with_capacity(n_tiles);
    for tj in 0..tiles_j {
        for ti in 0..tiles_i {
            let oi = region_oi + (ti * tile_n) as i64 - d as i64;
            let oj = region_oj + (tj * tile_n) as i64 - d as i64;
            tiles.push(mk_window(oi, oj, win));
        }
    }

    let mut assembled = vec![0.0f32; span_i * span_j];
    let publish = |assembled: &mut [f32], t: usize, f: &Fluvial| {
        let (ti, tj) = (t % tiles_i, t / tiles_i);
        for j in 0..tile_n {
            for i in 0..tile_n {
                assembled[(tj * tile_n + j) * span_i + (ti * tile_n + i)] =
                    f.h[(d + j) * win + (d + i)];
            }
        }
    };
    let extract = |tiles: &[Fluvial]| -> Vec<ExchangedTile> {
        let mut out = Vec::with_capacity(n_tiles);
        for tj in 0..tiles_j {
            for ti in 0..tiles_i {
                let t = tj * tiles_i + ti;
                let mut h = vec![0.0f32; tile_n * tile_n];
                for j in 0..tile_n {
                    for i in 0..tile_n {
                        h[j * tile_n + i] = tiles[t].h[(d + j) * win + (d + i)];
                    }
                }
                out.push(ExchangedTile {
                    oi: (region_oi + (ti * tile_n) as i64).max(0) as u32,
                    oj: (region_oj + (tj * tile_n) as i64).max(0) as u32,
                    nx: tile_n,
                    h,
                });
            }
        }
        out
    };
    let refill = |assembled: &[f32], t: usize, f: &mut Fluvial| {
        let (ti, tj) = (t % tiles_i, t / tiles_i);
        // Cone truncation: beyond ρ tiles from this tile, treat as outside.
        let rho = schedule.cone_rho as i32;
        let bi = (ti * tile_n) as i64 - d as i64;
        let bj = (tj * tile_n) as i64 - d as i64;
        for j in 0..win {
            for i in 0..win {
                if i >= d && i < d + tile_n && j >= d && j < d + tile_n {
                    continue; // interior is owned
                }
                let gx = bi + i as i64;
                let gy = bj + j as i64;
                let in_block = gx >= 0
                    && gy >= 0
                    && (gx as usize) < span_i
                    && (gy as usize) < span_j;
                let in_cone = if rho == 0 {
                    true
                } else {
                    // Chebyshev distance in tile units from this tile's index.
                    let (gti, gtj) = (
                        (gx.div_euclid(tile_n as i64)) as i32,
                        (gy.div_euclid(tile_n as i64)) as i32,
                    );
                    let di = (gti - ti as i32).abs();
                    let dj = (gtj - tj as i32).abs();
                    di.max(dj) <= rho
                };
                f.h[j * win + i] = if in_block && in_cone {
                    assembled[gy as usize * span_i + gx as usize]
                } else {
                    prior(region_oi + gx, region_oj + gy)
                };
            }
        }
    };

    let mut done = 0u32;
    let mut out = Vec::new();
    while done < epochs {
        let k = schedule.cadence.min(epochs - done);
        for f in tiles.iter_mut() {
            f.erode(&FluvialParams { epochs: k, ..Default::default() });
        }
        done += k;
        // Assemble the frozen snapshot every tile will read.
        for (t, f) in tiles.iter().enumerate() {
            publish(&mut assembled, t, f);
        }
        out = extract(&tiles);
        on_rung(done, &out);
        // Exchange after each chunk except when the settle is finished.
        if done < epochs {
            for (t, f) in tiles.iter_mut().enumerate() {
                refill(&assembled, t, f);
            }
        }
    }
    out
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
    /// The index into [`Self::h`] of the carved cell containing `cell` — no
    /// interpolation, no detail increment. `None` when this region does not
    /// cover it.
    ///
    /// This is the accessor for reading a **field the kernel actually produced**
    /// at the level it ran, and it exists because [`Self::surface_m`] cannot be
    /// that accessor: `surface_m` returns `base + detail`, bilinear over the carve
    /// plus the *fine prior minus coarse prior* increment, so every caller
    /// inheriting it inherits a surface no rung computed
    /// ( #form-fidelity-ladder FE(7)–(9)). Consumers that need a picture want
    /// `surface_m`; consumers computing a derived physical quantity want this.
    pub fn carved_index(&self, cell: CellId) -> Option<usize> {
        let (gx, gy) = self.grid_pos(cell)?;
        let (x, y) = ((gx.round() as usize).min(self.nx - 1), (gy.round() as usize).min(self.nx - 1));
        Some(y * self.nx + x)
    }

    /// The carved surface at `cell` with **no** interpolation and **no** detail
    /// increment — the stored height of the cell that answers for it.
    pub fn carved_surface_m(&self, cell: CellId) -> Option<f64> {
        Some(self.h[self.carved_index(cell)?] as f64)
    }

    /// **Standing water over this region, at this region's own level** — the wet
    /// limit ( #obs-connectivity-fills-the-basins-the-threshold-drained ), read
    /// on the bed the kernel produced rather than on any assembled or detailed
    /// surface. Row-major `nx × nx`, aligned with [`Self::h`] and indexable by
    /// [`Self::carved_index`].
    ///
    /// **The domain is the region, which is the honest unit.** A reader that
    /// re-derives this per *drawn tile* imposes a boundary the carve never had and
    /// manufactures seam pits at every tile rim ( #obs-tile-outlets-grade-away-the-basins );
    /// a region is wider, so fewer basins are cut by its edge and the enclosure
    /// test behind the ocean mask is correspondingly more honest
    /// ( #form-ocean-is-connectivity-not-elevation FE(4)).
    ///
    /// The boundary contract is the one geometry infers for this region — the same
    /// one its own carve ran under — because reading a bed under a contract it was
    /// not made under reports water standing on ground that was graded assuming an
    /// outlet elsewhere. Regions carved with halo exchange had live neighbours that
    /// this reader does not, which is a known and unmeasured understatement at
    /// region rims.
    pub fn standing_water(&self) -> Vec<f32> {
        let mut f = Fluvial::from_region(self);
        f.drainage_surface().standing_water
    }

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
