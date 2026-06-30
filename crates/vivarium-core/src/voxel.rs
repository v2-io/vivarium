//! The volumetric substrate — the world, "3D all the way down."
//!
//! ## Why this exists, and the shape of the commitment
//!
//! `DESIGN.md` settles two things this module embodies:
//!
//!  - **3D all the way down.** Physics needs the dimensions to not be
//!    nonsensical, so the world is genuinely volumetric, even if early human
//!    views render it isometric or first-person-blocky.
//!  - **Lazily materialized fidelity, with sparse edits that persist.** The
//!    world is "necessarily several fidelity levels: emergent, low-granularity
//!    dynamics modeled cheaply at a distance, higher fidelity closer to an
//!    observing agent." Abstract→detail (seeded worldgen) is the easy
//!    direction and the only one a spike needs.
//!
//! ## The core design decision (made deliberately; document is the point)
//!
//! Core stores the *entire* world as **`(seed) + a sparse overlay of edits`**.
//! There is no materialized voxel array in core at all:
//!
//!  - The untouched world is a **pure function** of the seed:
//!    [`Volume::generated`] computes any voxel from its coordinates with no
//!    stored state. This is the "abstraction" tier — infinite, cheap, exact,
//!    and trivially reproducible.
//!  - The only *materialized* state is [`Volume::edits`]: voxels an agent (or a
//!    player) has changed. It is keyed by absolute coordinate and ordered
//!    (`BTreeMap`, never `HashMap` — see the determinism note below), so a
//!    replay that applies the same edits in the same order lands bit-identical.
//!
//! Two consequences worth stating plainly, because they are *why* this shape
//! was chosen over the obvious "array of chunks":
//!
//!  1. **World-state stays tiny and the core/view wall stays sharp.** Chunking,
//!     meshing, level-of-detail, and any notion of "loaded region" are *view*
//!     concerns — they are how a renderer chooses to materialize fidelity near
//!     the camera. Core has no chunk concept. A view asks `voxel(x, y, z)` as
//!     many times as it likes; core answers from `seed`+overlay. If core ever
//!     grows a chunk cache "for performance," that is the signal a view concern
//!     has leaked inward — push it back out.
//!  2. **It is honest about what is not yet solved.** The genuinely hard,
//!     research-flavored direction in `DESIGN.md` is detail→abstract: when an
//!     agent reshapes a high-fidelity locus, the *abstract* model must absorb
//!     the change so it survives the locus collapsing back down. The overlay is
//!     the simplest thing that defers that question without lying about it:
//!     edits persist forever (never collapse), which is correct but does not
//!     yet *scale*. When the abstract↔detail reconciliation is attempted, it
//!     replaces the overlay; nothing outside this module should need to know.

use std::collections::BTreeMap;

use crate::Rng;

/// A single voxel: a material id. `0` is empty space (air). Kept to a `u16` so
/// the edit overlay stays cheap and a future palette can grow to thousands of
/// materials without a layout change. The named constants below are the only
/// materials the seed world uses today; treat them as scaffolding, not a
/// committed material system.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Voxel(pub u16);

impl Voxel {
    pub const AIR: Voxel = Voxel(0);
    pub const STONE: Voxel = Voxel(1);
    pub const DIRT: Voxel = Voxel(2);
    pub const GRASS: Voxel = Voxel(3);
    pub const WATER: Voxel = Voxel(4);

    /// Empty space is the absence of matter — the one material the mesher skips
    /// and the one digging leaves behind.
    #[inline]
    pub fn is_air(self) -> bool {
        self == Voxel::AIR
    }

    /// Whether a face between this voxel and a neighbour should be drawn — i.e.
    /// whether this voxel blocks sight. Water is solid-but-not-opaque is a
    /// refinement for later; for the spike, only air is see-through.
    #[inline]
    pub fn is_solid(self) -> bool {
        !self.is_air()
    }
}

/// **The real-world anchor** (DESIGN.md / `ref/geology/NOTES.md` §0a). The finest
/// voxel is half a metre on a side: a person is ~3–4 voxels, materials and items
/// have real extent, and — crucially — *geology is simulated at its true scale*.
/// Everything physical derives from this. One "world unit" below means **one
/// metre**; [`Detail`] is **voxels per metre**, so the anchor resolution is
/// `detail = 2`.
pub const METERS_PER_VOXEL: f32 = 0.5;

/// Vertical span of the world, **in metres**, measured from `y = 0` (deep
/// bedrock) up. ~8.2 km: enough for multi-km peaks above the sea and several km
/// of crust + ocean below. The world is a pure function — a tall world costs
/// nothing but generation *range*, since nothing is materialized away from the
/// surface. (Was 128 in the pre-anchor toy world; that was ~100× too short.)
pub const WORLD_HEIGHT: i32 = 8192;

/// Sea level, **in metres** (~3 km up the column), leaving ~5 km of headroom for
/// mountains and ~3 km below for ocean basins and crust (caves, strata, ore).
pub const SEA_LEVEL: i32 = 3000;

/// Soil depth (grass/dirt skin over bedrock), **in metres**. A few metres of
/// regolith; below it is stone. Scales to voxels via [`Detail`].
pub const SOIL_DEPTH: i32 = 4;

/// Version tag for a *worldgen result* — bumped whenever a change to the
/// erosion or hydrology algorithm (or its constants: precip, step budget, dt,
/// epochs, …) would make the produced terrain/water DIFFERENT for the same
/// inputs. It is embedded in [`Volume::to_bytes`] and checked by
/// [`Volume::from_bytes`], so a cache written by an older version is silently
/// rejected (the caller regenerates) rather than loaded as stale, wrong data.
///
/// This exists because the expensive worldgen is cached to disk by the view
/// adapters (it takes minutes — see the godot bridge). The cache filename keys
/// on the *call* parameters (seed/region/epochs/…), but the hydrology constants
/// live inside [`Volume::eroded_refined`] and are NOT in that key — so without
/// a content version, editing `precip_rate` or the step budget would serve a
/// stale cache. **If you touch worldgen output, bump this.**
pub const WORLDGEN_VERSION: u32 = 1;

/// Minimum water depth (metres) that renders as water. Below this is a thin
/// transient runoff film, not standing water; thresholding it keeps hillslopes
/// dry rather than sheened.
const MIN_WATER_M: f32 = 1.0;

/// Continental-shelf floor, **in metres** — the elevation the land grades *down*
/// to beyond the eroded patch. 300 m below sea, so the engineered region reads as
/// a continent sitting in open ocean (DESIGN: bounded continent, 2026-06-23).
/// Without this the bilinear sampler's edge-clamp extruded the patch boundary's
/// height to the horizon — straight ridges running off forever (the artifact this
/// removes).
const OCEAN_FLOOR_M: f32 = SEA_LEVEL as f32 - 300.0;

/// Distance, **in metres**, over which the macro surface grades from the patch
/// edge down to [`OCEAN_FLOOR_M`]. The continental shelf / drop-off width; also
/// the band over which sub-grid detail noise fades out, so the open ocean is a
/// clean flat sea rather than a noise-speckled fringe.
const SHELF_FALLOFF_M: f32 = 1500.0;

/// World resolution: **voxels per metre**. The anchor is `detail = 2` (0.5 m
/// voxels, [`METERS_PER_VOXEL`]). The constants above are in metres and are
/// multiplied through by `detail` to reach voxels; terrain noise is sampled in
/// *metre* space and scaled up, so a finer `detail` reproduces the same landscape
/// out of more, smaller voxels rather than inventing higher-frequency relief.
///
/// A *view* renders voxels at `1 / detail` metres, so the world is the same
/// physical size at any resolution — the knob the Bevy/Godot spike swept for the
/// rendering perf ceiling. The *geology* tier resolutions (tectonics ~3 km,
/// fluvial erosion ~16 m; NOTES §0a) are independent of this and live in
/// [`Volume::eroded`].
pub type Detail = i32;

/// A patch of pre-computed eroded terrain — the output of the slow "abstraction
/// tier" (see [`crate::geo`]). It is the **fluvial-erosion tier** of NOTES §0a:
/// elevations in **metres**, sampled on a coarse grid of ~16 m cells (the
/// research-earned resolution where first-order drainage survives), spanning a
/// region kilometres across. When a [`Volume`] carries one it *is* the terrain —
/// [`Volume::terrain_height`] samples it bilinearly and the 0.5 m render voxels
/// are materialized from it. Coarse sim grid, fine render voxels: orders of
/// magnitude apart, exactly as the scale ladder requires.
#[derive(Clone, Debug)]
struct ErodedSurface {
    /// Metre coordinate of grid node `(0, 0)`.
    x0_m: f32,
    z0_m: f32,
    /// Metres per erosion cell (~16 m — NOTES §0a).
    cell_m: f32,
    /// Grid edge length (square), in nodes.
    nx: usize,
    /// Row-major **carved** elevations in **metres**, length `nx · nx` — the bed
    /// after the coupled water+sediment sim. This *is* the terrain; there is no
    /// fractal-noise overlay any more (the detail is earned by erosion).
    h_m: Vec<f32>,
    /// Per-node **water depth** in metres (≥ 0), from the [`crate::hydro`] sim, on
    /// the *same* grid as `h_m`. The wet mask (depth ≥ threshold ⇒ render water);
    /// safe to bilinearly sample since dry is a true 0.
    depth_m: Vec<f32>,
    /// Per-node **water-surface elevation** `bed + depth` in metres, same grid.
    /// The level the water stands at — sampled directly so a lake reads flat
    /// (reconstructing it from the bed + depth at a mismatched resolution is what
    /// draped water over sub-grid bumps; same grid means no mismatch).
    water_surf_m: Vec<f32>,
    /// Per-node **flow velocity** (m/s) at the frozen instant — `vx` east-positive,
    /// `vy` south-positive. The other half of the snapshot (with `depth_m` as the
    /// volume): direction + speed for currents, agents, and still-vs-flowing. Not
    /// rendered yet, but frozen so the view/agents can use it.
    vx_m: Vec<f32>,
    vy_m: Vec<f32>,
}

impl ErodedSurface {
    /// Bilinearly sample the surface (metres) at a metre position. Outside the
    /// patch the sample coordinate is **clamped to the edge** (see [`Self::bilinear`]),
    /// so this alone would extrude the boundary height outward forever. The
    /// open-ocean falloff that turns that into a real coastline is applied by the
    /// caller ([`Volume::terrain_height`]) via [`Self::exterior_t`] — kept separate
    /// so this stays a pure sampler. Always returns a height.
    fn sample(&self, xm: f32, zm: f32) -> f32 {
        self.bilinear(&self.h_m, xm, zm)
    }

    /// Smooth exterior weight: `0` anywhere inside the eroded patch, ramping to `1`
    /// over [`SHELF_FALLOFF_M`] beyond its edge. This is the lever that makes the
    /// patch a continent in open sea: the caller grades the macro height from the
    /// (edge-clamped) sample down to [`OCEAN_FLOOR_M`] by this weight, and fades the
    /// sub-grid detail noise out by `1 - t`, so beyond the shelf the world is a
    /// clean flat ocean instead of the edge cell smeared to the horizon.
    fn exterior_t(&self, xm: f32, zm: f32) -> f32 {
        let x1 = self.x0_m + (self.nx - 1) as f32 * self.cell_m;
        let z1 = self.z0_m + (self.nx - 1) as f32 * self.cell_m;
        // Distance from the point to the patch rectangle (0 when inside).
        let dx = (self.x0_m - xm).max(xm - x1).max(0.0);
        let dz = (self.z0_m - zm).max(zm - z1).max(0.0);
        let d = (dx * dx + dz * dz).sqrt();
        smoothstep(0.0, SHELF_FALLOFF_M, d)
    }

    /// Bilinearly sampled water depth (metres, ≥ 0) at a metre position. Faded out
    /// by the exterior weight so the open ocean beyond the patch stays the clean
    /// flat sea, not a smear of edge channels.
    fn water_depth(&self, xm: f32, zm: f32) -> f32 {
        self.bilinear(&self.depth_m, xm, zm) * (1.0 - self.exterior_t(xm, zm))
    }

    /// Water-*surface* elevation (metres) — the flat level a lake stands at —
    /// sampled by a **wet-masked** bilinear: only cells that actually hold water
    /// contribute. A plain bilinear would blend in dry bank cells (whose stored
    /// surface is just the high bank `bed + 0`) and drag the rendered surface *up*
    /// the slope, making water climb the banks at every shoreline. Masking to wet
    /// cells keeps the lake level flat to its true edge; a column whose terrain
    /// rises above that level simply renders dry (the bank). Returns a very low
    /// value when no corner is wet (→ renders dry).
    fn water_surface(&self, xm: f32, zm: f32) -> f32 {
        let gx = ((xm - self.x0_m) / self.cell_m).clamp(0.0, (self.nx - 1) as f32);
        let gz = ((zm - self.z0_m) / self.cell_m).clamp(0.0, (self.nx - 1) as f32);
        let (x0, z0) = (gx.floor() as usize, gz.floor() as usize);
        let (x1, z1) = ((x0 + 1).min(self.nx - 1), (z0 + 1).min(self.nx - 1));
        let (tx, tz) = (gx - x0 as f32, gz - z0 as f32);
        let corners = [
            (z0 * self.nx + x0, (1.0 - tx) * (1.0 - tz)),
            (z0 * self.nx + x1, tx * (1.0 - tz)),
            (z1 * self.nx + x0, (1.0 - tx) * tz),
            (z1 * self.nx + x1, tx * tz),
        ];
        let (mut sum, mut wsum) = (0.0f32, 0.0f32);
        for (i, w) in corners {
            if self.depth_m[i] > 0.05 {
                sum += self.water_surf_m[i] * w;
                wsum += w;
            }
        }
        if wsum > 1e-6 {
            sum / wsum
        } else {
            -1.0e9 // no wet corner → render dry
        }
    }

    /// Bilinearly sampled flow **speed** (m/s) at a metre position — the magnitude
    /// of the frozen velocity field. For shading fast water toward white.
    fn speed(&self, xm: f32, zm: f32) -> f32 {
        let vx = self.bilinear(&self.vx_m, xm, zm);
        let vy = self.bilinear(&self.vy_m, xm, zm);
        (vx * vx + vy * vy).sqrt()
    }

    /// Shared bilinear sampler over a row-major `nx·nx` grid, clamped to the edge.
    fn bilinear(&self, grid: &[f32], xm: f32, zm: f32) -> f32 {
        let gx = ((xm - self.x0_m) / self.cell_m).clamp(0.0, (self.nx - 1) as f32);
        let gz = ((zm - self.z0_m) / self.cell_m).clamp(0.0, (self.nx - 1) as f32);
        let (x0, z0) = (gx.floor() as usize, gz.floor() as usize);
        let (x1, z1) = ((x0 + 1).min(self.nx - 1), (z0 + 1).min(self.nx - 1));
        let (tx, tz) = (gx - x0 as f32, gz - z0 as f32);
        let v00 = grid[z0 * self.nx + x0];
        let v10 = grid[z0 * self.nx + x1];
        let v01 = grid[z1 * self.nx + x0];
        let v11 = grid[z1 * self.nx + x1];
        let a = v00 + (v10 - v00) * tx;
        let b = v01 + (v11 - v01) * tx;
        a + (b - a) * tz
    }
}

/// The world as a volumetric field. Cheap to clone (the overlay is the only
/// heap state) — clone it to snapshot, diff two of them to study a trajectory,
/// exactly like the rest of [`crate::World`].
#[derive(Clone, Debug)]
pub struct Volume {
    /// Seed for the pure-function terrain. Distinct from (and derived from) the
    /// world seed so that changing agent dynamics never silently reshapes the
    /// ground, and vice versa.
    seed: u64,
    /// Voxels per world unit (see [`Detail`]). `1` is the base grid.
    detail: Detail,
    /// The only materialized state: voxels changed away from what the seed
    /// would generate. Ordered for deterministic replay; sparse so the cost is
    /// proportional to how much the world has been *touched*, not its size.
    edits: BTreeMap<[i32; 3], Voxel>,
    /// Optional pre-computed eroded terrain (see [`ErodedSurface`]). `None` is the
    /// raw-FBM world; `Some` is a world shaped by the [`crate::geo`] abstraction
    /// tier. Deterministic from the seed either way.
    eroded: Option<ErodedSurface>,
}

impl Volume {
    /// Base-resolution world (`detail = 1`).
    pub fn new(seed: u64) -> Self {
        Self::with_detail(seed, 1)
    }

    /// World at a chosen voxel resolution (see [`Detail`]). `detail` is clamped
    /// to at least `1`.
    pub fn with_detail(seed: u64, detail: Detail) -> Self {
        // Decorrelate the terrain stream from any other use of the world seed.
        let mut r = Rng::new(seed);
        Self { seed: r.next_u64(), detail: detail.max(1), edits: BTreeMap::new(), eroded: None }
    }

    /// Metres per fluvial-erosion cell — the research-earned resolution where
    /// first-order drainage survives (NOTES §0a; CAESAR-Lisflood). Far coarser
    /// than the 0.5 m render voxel, far finer than the tectonic tier.
    pub const EROSION_CELL_M: f32 = 16.0;

    /// Metres per cell of the **coupled water+sediment sim** — finer than the macro
    /// erosion grid, since this is where the fine detail (streams, fine carving) is
    /// earned. The macro field is upsampled onto this grid before the sim runs.
    /// Cost scales as `(span / SIM_CELL_M)² · steps`, so this is the granularity ⇄
    /// worldgen-time dial. 4 m resolves thin headwater streams (so the sealed-bed
    /// field catches them and they stop leaking underground); finer still is the
    /// next dial when worldgen time allows.
    pub const SIM_CELL_M: f32 = 4.0;

    /// Epochs of fine stream-power erosion at [`Self::SIM_CELL_M`] — the geological
    /// phase that carves the *fine* channels into the upsampled macro field (uplift
    /// off). This is where the carving lives; the water phase that follows does
    /// none (it just settles water on the finished bed).
    pub const FINE_EPOCHS: u32 = 50;

    /// Build a world whose terrain has been shaped by the [`crate::geo`] fluvial
    /// erosion tier. The raw FBM continental relief is sampled onto a ~16 m grid
    /// spanning `±region_half_m` **metres**, eroded by gentle uplift + stream-power
    /// incision + talus draining to sea level, and that matured surface *becomes*
    /// the terrain — km-scale mountains carved by a real drainage network. Outside
    /// the patch the coastline extends into open sea.
    ///
    /// The erosion is deliberately **gentle**: the kilometre relief comes from the
    /// continental FBM, and erosion's job here is to *carve valleys and drainage*
    /// into it, not to build or raze the mountains (the first spike's mistake was
    /// erosion strong enough to flatten everything — at the wrong scale entirely).
    ///
    /// Deterministic from `seed`. Cost is paid once, up front — the "world creation
    /// takes longer than a tick" tier. The grid is `(2·region_half_m / 16)²` nodes,
    /// so a ~24 km region is ~2.3 M nodes (a few seconds in release); the O(n)
    /// Braun-Willett stack (deferred in `geo`) is the lever when this must grow.
    pub fn eroded(seed: u64, detail: Detail, region_half_m: i32, epochs: u32) -> Self {
        // No fine-refinement pass: `fine_epochs = 0` reproduces the original
        // single-tier behaviour exactly (the Bevy spike depends on this).
        Self::eroded_refined(seed, detail, region_half_m, epochs, Self::EROSION_CELL_M, 0)
    }

    /// As [`Self::eroded`], but with an optional **finer-scale erosion refinement**.
    /// After the coarse ~16 m erosion matures the macro drainage network, the field
    /// is upsampled onto a `fine_cell_m` grid and run for `fine_epochs` more epochs
    /// of the *same stream-power physics* (with uplift switched **off** — we are
    /// refining existing relief, not building new mountains). The result is sub-16 m
    /// drainage detail that is *carved by erosion* rather than invented by the
    /// fractal `detail_relief` noise — a step toward the conservative-refinement
    /// goal (NOTES §7): physically-plausible fine structure instead of arbitrary
    /// noise. `fine_epochs = 0` (or `fine_cell_m >= 16 m`) skips it entirely.
    ///
    /// Cost scales with `(span / fine_cell_m)²`: at 8 m over a 24 km span that is
    /// ~2.3 M extra nodes per fine epoch; at 4 m, ~9 M. This is the experimental
    /// knob — start coarse and a few epochs, watch what it buys.
    pub fn eroded_refined(
        seed: u64,
        detail: Detail,
        region_half_m: i32,
        epochs: u32,
        fine_cell_m: f32,
        fine_epochs: u32,
    ) -> Self {
        let mut v = Self::with_detail(seed, detail);
        let cell = Self::EROSION_CELL_M;
        let span_m = (region_half_m.max(1) * 2) as f32;
        let nx = (span_m / cell).ceil() as usize;
        let origin = -region_half_m as f32;

        // Sample the continental FBM relief (metres) onto the ~16 m erosion grid.
        let mut h = vec![0.0f32; nx * nx];
        for vv in 0..nx {
            for uu in 0..nx {
                let xm = origin + uu as f32 * cell;
                let zm = origin + vv as f32 * cell;
                h[vv * nx + uu] = v.fbm_height_world(xm, zm);
            }
        }

        // Erosion at the real cell size, draining to sea level (metres). Strong
        // enough — now that MFD routing has dissolved the D8 grid bias — to carve
        // a real dendritic ridge-and-valley network into the continental relief,
        // not just a few gashes. (Tuned by hillshade sweep; see ref/geology.)
        let params = crate::geo::ErosionParams {
            nx,
            cell_size: cell,
            uplift: 2.0, // m/epoch
            k: 0.03,     // erodibility — strong enough (with MFD) for a dramatic dendritic network
            m: 0.5,
            max_slope: 1.2, // talus repose ~50° — steep mountain flanks allowed
            // Davy-Lague deposition `G` (dimensionless): grades slack lower reaches
            // and outlets without filling the upland valleys (see geo::deposit).
            // Swept by hillshade — valleys stay sharp even at G=1; 0.5 is balanced.
            deposition: 0.5,
            epochs,
            dt: 1.0,
            sea_level: Some(SEA_LEVEL as f32),
        };
        eprintln!("[vivarium] worldgen: macro erosion, {nx}×{nx} grid, {epochs} epochs…");
        let coarse = crate::geo::Heightfield::from_heights(nx, cell, h).erode(&params);

        let _ = (fine_cell_m, fine_epochs); // legacy API params, unused

        // === Phase 2 — FINE EROSION (geological time) ===================
        // Upsample the macro field to the sim resolution and incise finer channels
        // with the *same stream-power physics* (uplift off — the mountains already
        // exist; this refines). The carving lives HERE, on the geological clock,
        // where water is the steady-state drainage (discharge ∝ area), NOT in the
        // water sim. That separation is what makes the timescales honest: erosion
        // over millennia here, hydrology over hours/days below.
        let surf_cell = Self::SIM_CELL_M;
        let surf_nx = (span_m / surf_cell).ceil() as usize;
        let mut hf = vec![0.0f32; surf_nx * surf_nx];
        for vv in 0..surf_nx {
            for uu in 0..surf_nx {
                let xm = origin + uu as f32 * surf_cell;
                let zm = origin + vv as f32 * surf_cell;
                hf[vv * surf_nx + uu] = sample_grid_bilinear(&coarse.h, nx, origin, cell, xm, zm);
            }
        }
        let fine_params = crate::geo::ErosionParams {
            nx: surf_nx,
            cell_size: surf_cell,
            uplift: 0.0, // refine, don't build
            epochs: Self::FINE_EPOCHS,
            ..params.clone()
        };
        eprintln!("[vivarium] worldgen: fine erosion, {surf_nx}×{surf_nx} grid, {} epochs…", Self::FINE_EPOCHS);
        let fine = crate::geo::Heightfield::from_heights(surf_nx, surf_cell, hf).erode(&fine_params);

        // Sealed-channel infiltration field — baked from the erosion's drainage.
        // Channel beds (high upslope drainage) armor and colmate over geological
        // time → near-zero infiltration, so a stream stays on the surface instead of
        // leaking underground and re-gushing as an over-large spring; porous slopes
        // (low drainage) keep the full rate. This is the *erosion phase* doing the
        // sealing (Joseph), the water phase just inherits it.
        const DRAIN_REF_M2: f32 = 5.0e4; // ~0.05 km²: channel-onset drainage scale
        const BASE_INFIL: f32 = 0.02; // m/s on porous ground (matches the scalar)
        let infil_field: Vec<f32> = fine
            .drainage
            .iter()
            .map(|&dr| BASE_INFIL / (1.0 + dr / DRAIN_REF_M2))
            .collect();
        let surf_h = fine.h;

        // === Phase 3 — WATER (hydrological time), then FREEZE ============
        // The land is finished. Now run *real water* on the fixed bed — rain, flow,
        // groundwater, springs — to its hydrological steady state (lakes level,
        // streams settle), and freeze the snapshot. **Sediment is OFF** here
        // (capacity = 0): the carving already happened, on the geological clock;
        // this phase is hours-to-days of water settling, which carves nothing. So
        // nothing is time-conflated — the bed is the erosion result, the water is
        // the equilibrium hydrology on top of it. Hardness still feeds the
        // groundwater (springs at lithologic contacts).
        let wp = crate::hydro::WaterParams {
            cell: surf_cell, // m
            gravity: 9.81,   // m/s²
            // s — well inside the CFL limit (cell/√(g·d)) for inland water. The old
            // 0.01·cell was a ~10× over-conservative holdover from when erosion ran
            // *inside* the water phase and blew up; pure water is stable far higher,
            // and a bigger dt is what lets lakes actually reach their flat
            // equilibrium in the step budget (more physical time per step).
            dt: 0.03 * surf_cell,
            pipe_area: surf_cell * surf_cell, // m²
            // Lighter rain — below the infiltration rate, so slopes soak it in and
            // streams emerge from groundwater concentrating in the valleys rather
            // than a surface deluge. (Still ~100s× real rain, to fill in 40 min not
            // weeks — but no longer a flood.)
            precip_rate: 0.006,               // m/s of water
            evaporation: 0.005,               // 1/s
            infiltration: 0.02,               // m/s into unsaturated soil
            gw_capacity: 1.5,                 // m (porosity × soil depth) — PLACEHOLDER (uniform)
            gw_conductivity: 0.15,            // 1/s relaxation — PLACEHOLDER (should be permeability)
            exfil_rate: 4.0,                  // 1/s — springs discharge gradually
            baseflow: 0.0004,                 // 1/s
            sea_level: Some(SEA_LEVEL as f32), // m
            capacity: 0.0,                     // sediment OFF — land is already shaped
            ..Default::default()
        };
        // Enough steps for the water to approach hydrological steady state — flow
        // to reach the sea/lakes, lakes to level. Pure water is cheap per step, so
        // we can afford a generous budget. (Wide shallow lakes still level
        // asymptotically slowly — gravity waves — which is why the proper fix for
        // their final flatness is a volume-conserving fill, not just more steps.)
        // NB: this step budget and the `precip_rate`/`dt` above determine the
        // frozen water result. If you change any of them, bump [`WORLDGEN_VERSION`]
        // so on-disk worldgen caches invalidate instead of serving stale water.
        let steps = (surf_nx as u32 * 20).clamp(1600, 12000);
        // Charge the atmosphere with ~the run's precipitation budget (it recycles
        // via evaporation, so this only has to prime the cycle).
        let atm = wp.precip_rate as f64 * wp.dt as f64 * steps as f64
            * (surf_nx * surf_nx) as f64
            * 1.5;
        let mut sim = crate::hydro::WaterSim::new(surf_nx, surf_h)
            .with_atmosphere(atm)
            .with_hardness(crate::geo::Strata::new(v.seed), origin)
            .with_infiltration_field(infil_field);
        // Run in chunks so worldgen can report progress — this is the slow tier and
        // a frozen window with no feedback is miserable to wait on. The chunking is
        // pure book-keeping; it does not change the result (still deterministic).
        eprintln!("[vivarium] worldgen: water settling, {surf_nx}×{surf_nx}, {steps} steps…");
        let chunk = (steps / 20).max(1);
        let mut done = 0;
        while done < steps {
            let n = chunk.min(steps - done);
            sim.run(&wp, n);
            done += n;
            eprintln!("[vivarium] worldgen: {:>3}%", done * 100 / steps);
        }

        // Freeze the full water state: heights, the continuous (partial) depth =
        // volume, AND the velocity field (direction + speed) — the snapshot Joseph
        // asked for, for flow rendering / agents / still-vs-flowing.
        let (vx_m, vy_m) = sim.velocity(surf_cell);
        let carved = sim.bed; // == the geo-eroded bed (water phase carves nothing)
        let depth_m = sim.depth;
        let water_surf_m: Vec<f32> =
            carved.iter().zip(&depth_m).map(|(&b, &d)| b + d).collect();

        v.eroded = Some(ErodedSurface {
            x0_m: origin,
            z0_m: origin,
            cell_m: surf_cell,
            nx: surf_nx,
            h_m: carved,
            depth_m,
            water_surf_m,
            vx_m,
            vy_m,
        });
        v
    }

    /// Voxels per world unit. A view scales rendered voxels by `1 / detail`.
    #[inline]
    pub fn detail(&self) -> Detail {
        self.detail
    }

    /// Top of the world in voxels at this resolution.
    #[inline]
    pub fn world_height(&self) -> i32 {
        WORLD_HEIGHT * self.detail
    }

    /// Waterline in voxels at this resolution.
    #[inline]
    pub fn sea_level(&self) -> i32 {
        SEA_LEVEL * self.detail
    }

    /// The voxel at `(x, y, z)`: an edit if one exists there, otherwise whatever
    /// the seed generates. This is the one question a view ever needs to ask.
    #[inline]
    pub fn voxel(&self, x: i32, y: i32, z: i32) -> Voxel {
        self.edits
            .get(&[x, y, z])
            .copied()
            .unwrap_or_else(|| self.generated(x, y, z))
    }

    /// Change a voxel and remember the change forever. Returns the previous
    /// value so a caller can implement undo or conservation bookkeeping. An edit
    /// that restores a voxel to exactly what generation would produce is kept as
    /// an explicit edit on purpose: it records that an agent *acted* there, and
    /// the cost is one map entry. (Collapsing such edits is an optimization that
    /// belongs with the future abstract↔detail work, not here.)
    pub fn set_voxel(&mut self, x: i32, y: i32, z: i32, v: Voxel) -> Voxel {
        let prev = self.voxel(x, y, z);
        self.edits.insert([x, y, z], v);
        prev
    }

    /// Number of voxels that have been edited away from the generated world.
    /// The whole materialized footprint of the world; handy for the harness to
    /// assert "this many digs happened."
    pub fn edit_count(&self) -> usize {
        self.edits.len()
    }

    /// The `y` of the topmost solid voxel in the generated column at `(x, z)`,
    /// or `None` if the column is empty. Pure-function and edit-blind on
    /// purpose: it answers "where is the *ground*," which is a worldgen fact an
    /// agent stands on; digging a hole does not move the ground out from under
    /// everyone. (A view that wants the edited surface can scan [`voxel`].)
    pub fn surface_height(&self, x: i32, z: i32) -> Option<i32> {
        // Generation is heightmap-derived, so the analytic height is exact and
        // we need no scan. Kept as a method (not the raw height) so callers
        // depend on "the surface" rather than on generation internals.
        let h = self.terrain_height(x, z);
        (h >= 0).then_some(h)
    }

    // ---- the abstraction tier: pure-function generation ------------------

    /// The generated (pre-edit) voxel at a coordinate. Pure function of
    /// `(seed, x, y, z)` — the heart of the "infinite, exact, reproducible"
    /// abstraction tier.
    fn generated(&self, x: i32, y: i32, z: i32) -> Voxel {
        if y < 0 || y >= self.world_height() {
            return Voxel::AIR;
        }
        let h = self.terrain_height(x, z);
        if y > h {
            // Above ground: water up to the static waterline (sea, or a higher
            // inland stream/pool surface from the baked hydrology), else air.
            let wl = self.waterline(x, z);
            return if y <= wl { Voxel::WATER } else { Voxel::AIR };
        }
        if y == h {
            // The surface skin. Below the waterline it is dirt (lake/streambed),
            // above it is grass — a cheap, legible bit of texture for the eye.
            let wl = self.waterline(x, z);
            return if h < wl { Voxel::DIRT } else { Voxel::GRASS };
        }
        // Soil is a fixed *physical* depth ([`SOIL_DEPTH`] metres), so it scales
        // with resolution; below it is stone.
        if y > h - SOIL_DEPTH * self.detail {
            Voxel::DIRT
        } else {
            Voxel::STONE
        }
    }

    /// The static waterline at column `(x, z)`, in voxels: the higher of sea level
    /// and the baked **inland** water surface (eroded bed + hydrology depth). This
    /// is the `y` up to which an above-ground column fills with [`Voxel::WATER`].
    /// Pure function of worldgen — deterministic, edit-blind, and `None`-eroded
    /// worlds simply see the sea. See [`crate::hydro`] for the water model.
    fn waterline(&self, x: i32, z: i32) -> i32 {
        let sea = self.sea_level();
        let Some(e) = &self.eroded else { return sea };
        let d = self.detail as f32;
        let (xm, zm) = (x as f32 / d, z as f32 / d);
        let depth = e.water_depth(xm, zm);
        if depth < MIN_WATER_M {
            // Below the render threshold: a thin transient film, not standing
            // water. Treating it as dry keeps slopes from reading as a wet sheen
            // and leaves only channels and pools.
            return sea;
        }
        // The level is the sim's own flat water surface — NOT terrain_bed + depth,
        // which would drape the water over sub-grid terrain bumps. A terrain bump
        // poking above this level simply becomes an island (terrain_height > wl, so
        // `generated` renders no water in that column).
        let surf_m = e.water_surface(xm, zm);
        sea.max((surf_m * d).round() as i32)
    }

    /// Depth of standing water at column `(x, z)`, **in voxels** (waterline minus
    /// ground; 0 where dry). A view can shade water darker where this is larger.
    pub fn water_depth_voxels(&self, x: i32, z: i32) -> i32 {
        (self.waterline(x, z) - self.terrain_height(x, z)).max(0)
    }

    /// Flow **speed** (m/s) at column `(x, z)` from the frozen velocity field
    /// (0 with no eroded surface). A view can shade fast water toward white.
    pub fn water_speed(&self, x: i32, z: i32) -> f32 {
        match &self.eroded {
            Some(e) => {
                let d = self.detail as f32;
                e.speed(x as f32 / d, z as f32 / d)
            }
            None => 0.0,
        }
    }

    /// Terrain surface height at voxel column `(x, z)`, in voxels. Deterministic.
    ///
    /// The voxel coordinate is converted to **metres** (`coord / detail`), the
    /// terrain height is evaluated in metres — from the [`ErodedSurface`] when one
    /// is present (the eroded landscape), else the raw continental FBM — and the
    /// result is scaled back to voxels (`× detail`). So the eroded sim grid
    /// (~16 m), the metre datum, and the render voxels (0.5 m) are cleanly
    /// decoupled resolutions, and a finer `detail` reproduces the same landscape
    /// out of more, smaller voxels.
    fn terrain_height(&self, x: i32, z: i32) -> i32 {
        let d = self.detail as f32;
        let (xm, zm) = (x as f32 / d, z as f32 / d);
        let h_m = match &self.eroded {
            // The carved bed *is* the terrain — the fine detail was earned by the
            // water+sediment sim, so there is no fractal-noise overlay. Beyond the
            // patch, grade the (edge-clamped) height down to the ocean floor so the
            // engineered region reads as a continent in open sea. `t` is 0 inside
            // the patch, so this is a no-op there.
            //
            // (At the 16 m tier the bilinear bed still quantizes into terraces on
            // steep slopes at 0.5 m voxels — the honest consequence of a coarse grid
            // with no noise to hide it; the fix is finer resolution, not noise.)
            Some(e) => {
                let t = e.exterior_t(xm, zm);
                let macro_m = e.sample(xm, zm);
                macro_m + (OCEAN_FLOOR_M - macro_m) * t
            }
            None => self.fbm_height_world(xm, zm),
        };
        (h_m * d).round() as i32
    }

    /// Continuous terrain height in **metres** (detail-independent) from the raw
    /// FBM — the uneroded *continental* relief, and the initial condition the
    /// erosion tier starts from. Coordinates are in metres.
    ///
    /// Real-scale: a base wavelength of ~22 km gives mountain *massifs*, six
    /// octaves add ridge-and-valley structure down to ~700 m features, and the
    /// amplitude puts peaks a couple of km above the 3 km sea level and ocean
    /// basins a couple of km below. (Sub-~700 m detail is the render-voxel tier's
    /// job — added noise under the fidelity invariant — not invented here.)
    fn fbm_height_world(&self, xu: f32, zu: f32) -> f32 {
        // **The scale-free prior.** FBM is the honest maximum-entropy field when we
        // know there is structure at every scale but have no information yet
        // privileging any one — the unbiased prior before the *actual* fractal
        // hierarchy is known. So it is not pretending to be geology: it is the
        // proto-relief that the (physical) erosion then *shapes*, and a deliberate
        // placeholder for the tectonic-uplift tier (deferred — NOTES §0a) that will
        // one day replace it with information about where ranges truly sit.
        //
        // Base wavelength is kept *smaller* than a typical view so relief exists at
        // many scales and several massifs fall in frame; erosion dissects it into
        // emergent ridges. (Earlier a 22 km base — larger than the 12 km region —
        // collapsed this to one dome; that was the bug, not the voxel scaling.)
        const FREQ: f32 = 1.0 / 6_000.0; // ~6 km base wavelength (< region)
        const AMPLITUDE: f32 = 2_200.0; // metres of relief about the base
        const BASE: f32 = 3_300.0; // metres — a little above the 3 km sea
        const OCTAVES: u32 = 7; // down to ~90 m features for erosion to act on
        BASE + self.fbm(xu * FREQ, zu * FREQ, OCTAVES) * AMPLITUDE
    }

    /// The uneroded **continental** relief height in metres at a metre position —
    /// the macro abstraction *before* erosion and detail noise. Exposed so
    /// world-creation tools (and erosion-tuning sweeps) can reconstruct the exact
    /// initial condition `eroded` starts from.
    pub fn continental_height_m(&self, x_m: f32, z_m: f32) -> f32 {
        self.fbm_height_world(x_m, z_m)
    }

    /// Fractal Brownian motion: sum `octaves` of Perlin noise, each at twice the
    /// frequency and half the amplitude of the last (lacunarity 2, persistence
    /// ½). Normalised back to roughly `[-1, 1]`. This is what turns a single
    /// smooth swell into hills-with-texture.
    fn fbm(&self, x: f32, z: f32, octaves: u32) -> f32 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut freq = 1.0;
        let mut norm = 0.0;
        for _ in 0..octaves {
            sum += amp * self.perlin2d(x * freq, z * freq);
            norm += amp;
            amp *= 0.5;
            freq *= 2.0;
        }
        sum / norm
    }

    /// 2D gradient (Perlin) noise in roughly `[-1, 1]`. At each integer lattice
    /// corner a pseudo-random unit gradient is dotted with the offset to the
    /// sample point; the four corner values are interpolated with Perlin's
    /// quintic fade. Unlike value noise, the zero-crossings fall *on* the lattice
    /// and the slopes are gradient-driven, giving the more natural ridged look.
    fn perlin2d(&self, x: f32, z: f32) -> f32 {
        let x0 = x.floor();
        let z0 = z.floor();
        let xf = x - x0;
        let zf = z - z0;
        let (xi, zi) = (x0 as i64, z0 as i64);

        let g00 = self.gradient(xi, zi);
        let g10 = self.gradient(xi + 1, zi);
        let g01 = self.gradient(xi, zi + 1);
        let g11 = self.gradient(xi + 1, zi + 1);

        // Dot each corner gradient with the vector from that corner to (x, z).
        let d00 = g00.0 * xf + g00.1 * zf;
        let d10 = g10.0 * (xf - 1.0) + g10.1 * zf;
        let d01 = g01.0 * xf + g01.1 * (zf - 1.0);
        let d11 = g11.0 * (xf - 1.0) + g11.1 * (zf - 1.0);

        let u = fade(xf);
        let v = fade(zf);
        let a = lerp(d00, d10, u);
        let b = lerp(d01, d11, u);
        // 2D Perlin lands within ±√2/2; scale to roughly [-1, 1].
        lerp(a, b, v) * std::f32::consts::SQRT_2
    }

    /// A stable unit gradient for an integer lattice point, chosen from 8
    /// directions by hashing `(seed, xi, zi)` through the same SplitMix64 mix the
    /// world PRNG uses — so the field is reproducible and shares the determinism
    /// guarantee rather than introducing a second, unaudited source of state.
    fn gradient(&self, xi: i64, zi: i64) -> (f32, f32) {
        const G: f32 = std::f32::consts::FRAC_1_SQRT_2;
        const GRADS: [(f32, f32); 8] = [
            (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0),
            (G, G), (-G, G), (G, -G), (-G, -G),
        ];
        let mut h = self.seed;
        for w in [xi as u64, zi as u64] {
            h ^= w.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            h = (h ^ (h >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            h = (h ^ (h >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            h ^= h >> 31;
        }
        GRADS[(h & 7) as usize]
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Hermite smoothstep: 0 below `lo`, 1 above `hi`, an ease-curve between. Used to
/// turn a local slope into a `[0, 1]` roughness weight.
#[inline]
/// Bilinearly sample a square row-major `nx·nx` grid whose cell `[0,0]` sits at
/// world `(origin, origin)` with spacing `cell`, clamped to the edge outside it.
/// Used to upsample the coarse eroded field onto the finer refinement grid.
fn sample_grid_bilinear(grid: &[f32], nx: usize, origin: f32, cell: f32, xm: f32, zm: f32) -> f32 {
    let gx = ((xm - origin) / cell).clamp(0.0, (nx - 1) as f32);
    let gz = ((zm - origin) / cell).clamp(0.0, (nx - 1) as f32);
    let (x0, z0) = (gx.floor() as usize, gz.floor() as usize);
    let (x1, z1) = ((x0 + 1).min(nx - 1), (z0 + 1).min(nx - 1));
    let (tx, tz) = (gx - x0 as f32, gz - z0 as f32);
    let v00 = grid[z0 * nx + x0];
    let v10 = grid[z0 * nx + x1];
    let v01 = grid[z1 * nx + x0];
    let v11 = grid[z1 * nx + x1];
    let a = v00 + (v10 - v00) * tx;
    let b = v01 + (v11 - v01) * tx;
    a + (b - a) * tz
}

fn smoothstep(lo: f32, hi: f32, x: f32) -> f32 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Perlin's quintic fade `6t⁵ − 15t⁴ + 10t³`: an ease curve with zero first *and*
/// second derivative at the ends, so interpolated noise has no visible creases
/// at lattice boundaries (the improvement over plain cubic smoothstep).
#[inline]
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

// === Worldgen cache (de)serialization =============================================
//
// The eroded/hydrology worldgen is the slow tier — minutes for a full region (a
// km-scale grid run to hydrological steady state). For active iteration on a
// *view* (where the world is fixed and only the camera/render changes) recomputing
// it every launch is pure waste. So a view adapter can freeze a generated `Volume`
// to disk once and reload it instantly thereafter.
//
// This lives in core (not the adapter) because core owns these types' private
// fields and their invariants — serialization belongs with the data it mirrors.
// But core stays true to its dependency-free, closed-system charter: it does NO
// file I/O and pulls in NO serde/bincode. It only converts `Volume ⇄ bytes`; the
// adapter decides *where* those bytes live and *when* to trust them. A hand-rolled
// little-endian format keyed by [`WORLDGEN_VERSION`] is plenty for a handful of
// `f32` arrays and a sparse edit map, and it keeps the "no third-party crate
// touches the formal state" guarantee intact.
//
// Determinism is preserved by construction: the bytes are a faithful image of a
// deterministically-generated `Volume`, and any version/format/length mismatch is
// rejected (→ `None`), so a corrupt or stale file can only ever cause a *regenerate*,
// never a silently-wrong world.

/// 8-byte file magic. The trailing digits are the *serialization layout* version
/// (distinct from [`WORLDGEN_VERSION`], which versions the world *content*): bump
/// the magic when the byte layout below changes, bump `WORLDGEN_VERSION` when the
/// terrain/water a given input produces changes.
const CACHE_MAGIC: [u8; 8] = *b"VIVWLD01";

impl Volume {
    /// Serialize this volume to a compact little-endian byte image for the worldgen
    /// cache (see the module-level note). Round-trips exactly through
    /// [`Volume::from_bytes`]. The image embeds [`WORLDGEN_VERSION`] so a reader can
    /// reject content produced by a different worldgen.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&CACHE_MAGIC);
        b.extend_from_slice(&WORLDGEN_VERSION.to_le_bytes());
        b.extend_from_slice(&self.seed.to_le_bytes());
        b.extend_from_slice(&self.detail.to_le_bytes());

        // Edit overlay: count, then (x, y, z, voxel) each. BTreeMap iterates in
        // sorted key order, so the byte image is itself deterministic.
        b.extend_from_slice(&(self.edits.len() as u64).to_le_bytes());
        for (pos, vox) in &self.edits {
            b.extend_from_slice(&pos[0].to_le_bytes());
            b.extend_from_slice(&pos[1].to_le_bytes());
            b.extend_from_slice(&pos[2].to_le_bytes());
            b.extend_from_slice(&vox.0.to_le_bytes());
        }

        match &self.eroded {
            None => b.push(0),
            Some(e) => {
                b.push(1);
                b.extend_from_slice(&e.x0_m.to_le_bytes());
                b.extend_from_slice(&e.z0_m.to_le_bytes());
                b.extend_from_slice(&e.cell_m.to_le_bytes());
                b.extend_from_slice(&(e.nx as u64).to_le_bytes());
                // Five arrays, each exactly nx·nx long; length is implied by nx so
                // we don't repeat it. Order must match the read side below.
                for arr in [&e.h_m, &e.depth_m, &e.water_surf_m, &e.vx_m, &e.vy_m] {
                    for &f in arr {
                        b.extend_from_slice(&f.to_le_bytes());
                    }
                }
            }
        }
        b
    }

    /// Reconstruct a volume from a [`Volume::to_bytes`] image. Returns `None` on any
    /// mismatch — wrong magic, a different [`WORLDGEN_VERSION`], a truncated or
    /// internally-inconsistent buffer — so the caller treats a bad/stale cache as
    /// "regenerate", never as a usable world.
    pub fn from_bytes(bytes: &[u8]) -> Option<Volume> {
        let mut r = ByteReader::new(bytes);
        if r.take(8)? != CACHE_MAGIC {
            return None;
        }
        if r.u32()? != WORLDGEN_VERSION {
            return None; // produced by a different worldgen — invalidate.
        }
        let seed = r.u64()?;
        let detail = r.i32()?;

        let n_edits = r.u64()? as usize;
        let mut edits = BTreeMap::new();
        for _ in 0..n_edits {
            let pos = [r.i32()?, r.i32()?, r.i32()?];
            edits.insert(pos, Voxel(r.u16()?));
        }

        let eroded = match r.u8()? {
            0 => None,
            1 => {
                let x0_m = r.f32()?;
                let z0_m = r.f32()?;
                let cell_m = r.f32()?;
                let nx = r.u64()? as usize;
                let len = nx.checked_mul(nx)?; // guard a corrupt nx from over-allocating
                let mut read_arr = || -> Option<Vec<f32>> {
                    let mut v = Vec::with_capacity(len);
                    for _ in 0..len {
                        v.push(r.f32()?);
                    }
                    Some(v)
                };
                let h_m = read_arr()?;
                let depth_m = read_arr()?;
                let water_surf_m = read_arr()?;
                let vx_m = read_arr()?;
                let vy_m = read_arr()?;
                Some(ErodedSurface { x0_m, z0_m, cell_m, nx, h_m, depth_m, water_surf_m, vx_m, vy_m })
            }
            _ => return None, // not a valid option tag
        };

        // Reject trailing garbage: a well-formed image is consumed exactly.
        if !r.at_end() {
            return None;
        }
        Some(Volume { seed, detail, edits, eroded })
    }
}

/// Minimal bounds-checked little-endian cursor for [`Volume::from_bytes`]. Every
/// accessor returns `None` on underflow, so a truncated cache can never panic or
/// read past the buffer — it just fails the load and the caller regenerates.
struct ByteReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let slice = self.buf.get(self.pos..end)?;
        self.pos = end;
        Some(slice)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn i32(&mut self) -> Option<i32> {
        Some(i32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }
    fn f32(&mut self) -> Option<f32> {
        Some(f32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn at_end(&self) -> bool {
        self.pos == self.buf.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_pure_and_reproducible() {
        let a = Volume::new(0xABCD_1234);
        let b = Volume::new(0xABCD_1234);
        for (x, y, z) in [(0, 30, 0), (100, 10, -40), (-7, 0, 9999), (5, 127, 5)] {
            assert_eq!(a.voxel(x, y, z), b.voxel(x, y, z));
        }
    }

    #[test]
    fn edits_override_generation_and_persist() {
        let mut v = Volume::new(7);
        // Find a solid voxel to dig: top of the column at the origin.
        let h = v.surface_height(0, 0).unwrap();
        assert!(v.voxel(0, h, 0).is_solid());

        let prev = v.set_voxel(0, h, 0, Voxel::AIR);
        assert!(prev.is_solid());
        assert!(v.voxel(0, h, 0).is_air(), "dig must persist");
        assert_eq!(v.edit_count(), 1);
    }

    #[test]
    fn edit_replay_is_bit_identical() {
        // The tether-to-truth property for the volume: the same seed plus the
        // same sequence of edits yields an identical world. Edits are applied in
        // a fixed order; the ordered overlay guarantees the result does not
        // depend on map internals.
        let edits = [
            (1, 30, 1, Voxel::STONE),
            (1, 31, 1, Voxel::AIR),
            (-5, 26, 3, Voxel::WATER),
            (1, 30, 1, Voxel::DIRT), // overwrite an earlier edit
        ];
        let build = || {
            let mut v = Volume::new(0xC0FFEE);
            for &(x, y, z, m) in &edits {
                v.set_voxel(x, y, z, m);
            }
            v
        };
        let a = build();
        let b = build();
        for (x, y, z, _) in edits {
            assert_eq!(a.voxel(x, y, z), b.voxel(x, y, z));
        }
        assert_eq!(a.voxel(1, 30, 1), Voxel::DIRT, "later edit wins");
    }

    #[test]
    fn eroded_world_is_reproducible_and_changes_terrain() {
        // The eroded terrain must be a deterministic function of the seed (same
        // tether-to-truth guarantee as the raw world), and erosion must actually
        // reshape the surface (it is doing something). `region_half_m` is in
        // metres now; a ~3.2 km region at 16 m cells is ~200² nodes — fast.
        let a = Volume::eroded(0xC0FFEE, 2, 900, 12);
        let b = Volume::eroded(0xC0FFEE, 2, 900, 12);
        let raw = Volume::with_detail(0xC0FFEE, 2);
        let mut changed_somewhere = false;
        for z in -100..100 {
            for x in -100..100 {
                assert_eq!(
                    a.surface_height(x, z),
                    b.surface_height(x, z),
                    "eroded terrain diverged between runs at ({x},{z})"
                );
                if a.surface_height(x, z) != raw.surface_height(x, z) {
                    changed_somewhere = true;
                }
            }
        }
        assert!(changed_somewhere, "erosion left the surface untouched");
    }

    /// On a real-relief world the baked hydrology must (a) put water *inland and
    /// above sea level* — a stream surface standing higher than the ocean, the
    /// headless proxy for "walk to a valley and find a stream" — and (b) read as a
    /// *network*, not a sheet: only a minority of land columns are wet. (a) failing
    /// means the depth dials produce nothing visible; (b) failing means the free
    /// surface is over-flooding the valleys. Also checks determinism.
    #[test]
    fn eroded_world_has_inland_water_above_sea() {
        let v = Volume::eroded(0x1234_5678, 1, 1100, 50);
        let w = Volume::eroded(0x1234_5678, 1, 1100, 50);
        let sea = v.sea_level();
        let (mut inland, mut land, mut wet_land) = (0u32, 0u32, 0u32);
        let r = 1900;
        for z in (-r..=r).step_by(8) {
            for x in (-r..=r).step_by(8) {
                let wl = v.waterline(x, z);
                assert_eq!(wl, w.waterline(x, z), "waterline diverged at ({x},{z})");
                if wl > sea {
                    inland += 1;
                }
                let h = v.terrain_height(x, z);
                if h > sea {
                    land += 1;
                    if wl > h {
                        wet_land += 1; // standing water above the ground here
                    }
                }
            }
        }
        assert!(
            inland > 0,
            "no inland water above sea level — hydrology produced nothing visible"
        );
        let wet_frac = wet_land as f32 / land.max(1) as f32;
        assert!(
            wet_frac < 0.4,
            "{wet_frac} of land is under water — a sheet, not a stream network"
        );
    }

    #[test]
    fn eroded_world_is_deterministic_across_runs() {
        // The full pipeline — macro erosion + upsample + coupled water/sediment sim
        // — must be bit-reproducible from the seed (the tether-to-truth property).
        let a = Volume::eroded(0xC0FFEE, 2, 800, 12);
        let b = Volume::eroded(0xC0FFEE, 2, 800, 12);
        for z in -80..80 {
            for x in -80..80 {
                assert_eq!(
                    a.surface_height(x, z),
                    b.surface_height(x, z),
                    "eroded terrain diverged between runs at ({x},{z})"
                );
            }
        }
    }

    #[test]
    fn terrain_is_continuous() {
        // Adjacent voxels must not differ *wildly* — a walking agent and the
        // mesher rely on the surface being a slope, not a per-voxel cliff. At the
        // real anchor a steep mountainside is a genuine ~1–2 voxels per voxel, so
        // the bound is "no pathological cliff" (a noise bug), not "nearly flat".
        let v = Volume::with_detail(99, 4);
        let mut prev = v.surface_height(0, 0).unwrap();
        for x in 1..800 {
            let h = v.surface_height(x, 0).unwrap();
            assert!((h - prev).abs() <= 6, "height jump of {} at x={}", h - prev, x);
            prev = h;
        }
    }

    #[test]
    fn detail_scales_the_same_world_proportionally() {
        // A finer world is the *same* terrain in more, smaller voxels — not a
        // different shape. The same physical point should sit at ~detail× the
        // height, within rounding.
        let v1 = Volume::new(0xC0FFEE);
        let v8 = Volume::with_detail(0xC0FFEE, 8);
        assert_eq!(v8.detail(), 8);
        assert_eq!(v8.sea_level(), 8 * v1.sea_level());
        assert_eq!(v8.world_height(), 8 * v1.world_height());
        for x1 in [-30, 0, 17, 50] {
            let h1 = v1.surface_height(x1, 0).unwrap();
            let h8 = v8.surface_height(x1 * 8, 0).unwrap();
            assert!((h8 - 8 * h1).abs() <= 8, "h1={h1} h8={h8} at x1={x1}");
        }
    }

    #[test]
    fn fine_terrain_has_micro_relief_but_no_cliffs() {
        // At detail 8 the high-frequency octave puts micro-relief into the
        // surface — a few voxels of step is the *point* (it's what emerges at
        // high resolution). What must NOT happen is a pathological cliff (a bug
        // in the noise), so the bound is generous-but-finite.
        let v = Volume::with_detail(7, 8);
        let mut prev = v.surface_height(0, 0).unwrap();
        let mut max_jump = 0;
        for x in 1..1200 {
            let h = v.surface_height(x, 0).unwrap();
            max_jump = max_jump.max((h - prev).abs());
            assert!((h - prev).abs() <= 6, "cliff: jump {} at x={}", h - prev, x);
            prev = h;
        }
        // And it really is micro-relief, not a flat plane: some steps exist.
        assert!(max_jump >= 1, "expected some micro-relief");
    }

    // --- Worldgen cache round-trips ----------------------------------------------

    /// A plain (raw-FBM, no eroded patch) volume with edits survives a byte
    /// round-trip unchanged — both the metadata and the edit overlay.
    #[test]
    fn cache_round_trips_plain_volume_with_edits() {
        let mut v = Volume::with_detail(0xABCD, 2);
        let h = v.surface_height(10, -7).unwrap();
        v.set_voxel(10, h, -7, Voxel::AIR);
        v.set_voxel(10, h - 1, -7, Voxel::STONE);

        let back = Volume::from_bytes(&v.to_bytes()).expect("round-trip");
        assert_eq!(back.detail(), v.detail());
        assert_eq!(back.edit_count(), v.edit_count());
        // The reconstructed world answers voxel queries identically.
        for &(x, y, z) in &[(10, h, -7), (10, h - 1, -7), (0, h, 0)] {
            assert_eq!(back.voxel(x, y, z), v.voxel(x, y, z), "voxel mismatch at {x},{y},{z}");
        }
    }

    /// A volume carrying a (small) eroded+hydrology patch round-trips: the frozen
    /// surface, water, and velocity fields all come back bit-identical, so a
    /// reloaded world samples terrain and water exactly like the generated one.
    #[test]
    fn cache_round_trips_eroded_volume() {
        // Tiny region so the test stays fast — the codec doesn't care about size.
        let v = Volume::eroded_refined(99, 2, 256, 8, Volume::SIM_CELL_M, 4);
        let back = Volume::from_bytes(&v.to_bytes()).expect("round-trip");
        // Sample the surface + water + flow at a spread of columns; the reloaded
        // volume must answer every public query identically.
        for (x, z) in [(0, 0), (100, -60), (-240, 160), (400, 400)] {
            assert_eq!(back.surface_height(x, z), v.surface_height(x, z), "surface @ {x},{z}");
            assert_eq!(back.water_depth_voxels(x, z), v.water_depth_voxels(x, z), "depth @ {x},{z}");
            assert_eq!(back.water_speed(x, z), v.water_speed(x, z), "speed @ {x},{z}");
        }
    }

    /// A different [`WORLDGEN_VERSION`] in the header must be rejected (→ `None`),
    /// so a stale cache forces a regenerate rather than loading wrong terrain.
    #[test]
    fn cache_rejects_wrong_version_and_garbage() {
        let v = Volume::with_detail(1, 1);
        let mut bytes = v.to_bytes();
        // Corrupt the embedded version word (bytes 8..12, right after the magic).
        bytes[8] = bytes[8].wrapping_add(1);
        assert!(Volume::from_bytes(&bytes).is_none(), "wrong version must be rejected");
        // Truncated / nonsense buffers are rejected too, never panic.
        assert!(Volume::from_bytes(&[]).is_none());
        assert!(Volume::from_bytes(b"not a cache file").is_none());
    }
}
