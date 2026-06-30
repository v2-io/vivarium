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

/// Max edge length of the **water** simulation grid. Water runs on its own grid
/// (see [`ErodedSurface`]); capping it keeps the [`crate::hydro`] sim tractable
/// at continental span — at the cost of coarser water than terrain for now. A
/// dedicated finer water grid is the follow-up.
const WATER_GRID_CAP: usize = 320;

/// Minimum water depth (metres) that renders as water. Below this is a thin
/// transient runoff film, not standing water; thresholding it keeps hillslopes
/// dry rather than sheened. Roughly one render voxel at `detail = 2`.
const MIN_WATER_M: f32 = 0.4;

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
    /// Row-major elevations in **metres**, length `nx · nx`.
    h_m: Vec<f32>,
    /// Per-node **roughness** in `[0, 1]` derived from local slope: ~0 on flat
    /// valley floors and graded reaches, ~1 on steep flanks. Scales the sub-grid
    /// detail noise so it does not add bumps where a river would have smoothed the
    /// ground — the fidelity invariant applied to materialization (DESIGN.md), and
    /// the reason slack valley floors stop reading as a chain of pools.
    rough: Vec<f32>,
    /// Metres per cell of the **water** grid (see `depth_m`). Independent of the
    /// terrain `cell_m` — water runs on its own (currently coarser) resolution.
    water_cell_m: f32,
    /// Water-grid edge length in nodes.
    water_nx: usize,
    /// Per-node **water depth** in metres (≥ 0) on the water grid, the quasi-steady
    /// snapshot of the [`crate::hydro`] shallow-water simulation. The water
    /// *surface* is `bed + depth`; 0 means dry. Bilinearly sampled — safe because
    /// dry is a true 0, so interpolation never smears a sentinel.
    depth_m: Vec<f32>,
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

    /// Bilinearly sampled roughness `[0, 1]` at a metre position (see [`Self::rough`]).
    fn roughness(&self, xm: f32, zm: f32) -> f32 {
        self.bilinear(&self.rough, xm, zm)
    }

    /// Bilinearly sampled water depth (metres, ≥ 0) at a metre position, on the
    /// **water** grid (see [`Self::depth_m`]). Faded out by the exterior weight so
    /// the open ocean beyond the patch stays the clean flat sea, not a smear of
    /// edge channels.
    fn water_depth(&self, xm: f32, zm: f32) -> f32 {
        let d = sample_grid_bilinear(
            &self.depth_m,
            self.water_nx,
            self.x0_m,
            self.water_cell_m,
            xm,
            zm,
        );
        d * (1.0 - self.exterior_t(xm, zm))
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
        let coarse = crate::geo::Heightfield::from_heights(nx, cell, h).erode(&params);

        // Optional finer-scale refinement: upsample the eroded macro field and carve
        // it with a few more epochs of the same physics, uplift off. The surface that
        // *becomes the terrain* is then the finer field (more resolution, real
        // sub-grid drainage). `surf_*` is whichever field we keep.
        let (surf_field, surf_cell, surf_nx) = if fine_epochs > 0 && fine_cell_m < cell {
            let nx_f = (span_m / fine_cell_m).ceil() as usize;
            let mut hf = vec![0.0f32; nx_f * nx_f];
            for vv in 0..nx_f {
                for uu in 0..nx_f {
                    let xm = origin + uu as f32 * fine_cell_m;
                    let zm = origin + vv as f32 * fine_cell_m;
                    hf[vv * nx_f + uu] = sample_grid_bilinear(&coarse.h, nx, origin, cell, xm, zm);
                }
            }
            let fine_params = crate::geo::ErosionParams {
                nx: nx_f,
                cell_size: fine_cell_m,
                uplift: 0.0, // refine, don't build — the mountains already exist
                epochs: fine_epochs,
                ..params.clone()
            };
            let fine = crate::geo::Heightfield::from_heights(nx_f, fine_cell_m, hf).erode(&fine_params);
            (fine, fine_cell_m, nx_f)
        } else {
            (coarse, cell, nx)
        };

        let surf_h = surf_field.h; // move the kept heights out for the surface below

        // Roughness from local slope on the *kept* field: smooth (0) on flat/graded
        // floors, rough (1) on steep flanks — so the sub-grid detail noise textures
        // slopes but leaves valley floors graded (no spurious pools).
        let mut rough = vec![0.0f32; surf_nx * surf_nx];
        for j in 0..surf_nx {
            for i in 0..surf_nx {
                let l = surf_h[j * surf_nx + i.saturating_sub(1)];
                let r = surf_h[j * surf_nx + (i + 1).min(surf_nx - 1)];
                let d = surf_h[j.saturating_sub(1) * surf_nx + i];
                let u = surf_h[(j + 1).min(surf_nx - 1) * surf_nx + i];
                let slope = ((r - l).powi(2) + (u - d).powi(2)).sqrt() / (2.0 * surf_cell);
                rough[j * surf_nx + i] = smoothstep(0.05, 0.40, slope); // flat→0, steep→1
            }
        }

        // --- Water: an actual fluid simulation, run to a quasi-steady snapshot ---
        // The posed water surface is gone; this rains on the matured bed and lets
        // the shallow-water `hydro` model carry it downhill, concentrate it into
        // channels (infiltration soaks the slopes), and pond it flat in any basin.
        // Run on its own grid, capped so worldgen stays tractable at continental
        // span — coarser than the terrain for now; a finer water grid is the
        // follow-up. Deterministic: a fixed bed + fixed step count.
        let water_nx = surf_nx.min(WATER_GRID_CAP);
        let water_cell = span_m / water_nx as f32;
        let mut water_bed = vec![0.0f32; water_nx * water_nx];
        for vv in 0..water_nx {
            for uu in 0..water_nx {
                let xm = origin + uu as f32 * water_cell;
                let zm = origin + vv as f32 * water_cell;
                water_bed[vv * water_nx + uu] =
                    sample_grid_bilinear(&surf_h, surf_nx, origin, surf_cell, xm, zm);
            }
        }
        let wp = crate::hydro::WaterParams {
            cell: water_cell,
            gravity: 9.81,
            dt: 0.01 * water_cell, // ~CFL: scales with cell size
            pipe_area: water_cell * water_cell,
            rain: 0.03,
            evaporation: 0.0,
            infiltration: 0.026, // just under rain → slopes soak, channels run
            sea_level: Some(SEA_LEVEL as f32),
        };
        // Steps ~ a few domain crossings so channels reach the outlets; capped.
        let steps = (water_nx as u32 * 14).clamp(800, 6000);
        let mut sim = crate::hydro::WaterSim::new(water_nx, water_bed);
        sim.run(&wp, steps);
        let depth_m = sim.depth;

        v.eroded = Some(ErodedSurface {
            x0_m: origin,
            z0_m: origin,
            cell_m: surf_cell,
            nx: surf_nx,
            h_m: surf_h,
            rough,
            water_cell_m: water_cell,
            water_nx,
            depth_m,
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
        // Surface = the *smooth* eroded bed + depth, not the noisy terrain_height:
        // roughness is damped to ~0 on channels and floors, so the two agree where
        // water lives, and sampling the smooth bed keeps the surface flat where the
        // sub-grid noise would otherwise ripple it into a false chop.
        let surf_m = e.sample(xm, zm) + depth;
        sea.max((surf_m * d).round() as i32)
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
            // Macro form from the erosion tier, plus sub-grid detail relief —
            // scaled by local roughness so it textures steep flanks but leaves
            // graded valley floors smooth (no pooled bumps; see [`Self::detail_relief`]
            // and [`ErodedSurface::rough`]).
            //
            // Beyond the patch, grade the (edge-clamped) macro height down to the
            // ocean floor and fade the detail noise out, so the engineered region
            // reads as a continent in open sea rather than its boundary extruded to
            // the horizon. `t` is 0 inside the patch, so this is a no-op there.
            Some(e) => {
                let t = e.exterior_t(xm, zm);
                let macro_m = e.sample(xm, zm);
                let shelf_m = macro_m + (OCEAN_FLOOR_M - macro_m) * t;
                shelf_m + e.roughness(xm, zm) * self.detail_relief(xm, zm) * (1.0 - t)
            }
            None => self.fbm_height_world(xm, zm),
        };
        (h_m * d).round() as i32
    }

    /// Sub-erosion-grid relief, in **metres**, added on top of the eroded macro
    /// field. This is the render-voxel tier of the scale ladder (NOTES §0a): the
    /// erosion tier resolves the landscape only to ~16 m, so without this a real
    /// slope materializes as a smooth ramp that *quantizes into terraces* at 0.5 m
    /// — exactly the artifact a first-person view exposes that a macro hillshade
    /// hides. A few octaves of fractal roughness at the tens-of-metres scale give
    /// the surface walkable texture (knolls, gullies, broken ground) and break the
    /// 16 m grid periodicity.
    ///
    /// It is **zero-mean over the plane**, which is *necessary but not sufficient*
    /// for the fidelity invariant — and the earlier comment here overstated it.
    /// DESIGN.md's invariant (sharpened in NOTES §7) is a *per-coarse-cell
    /// conservation* property: the fine voxels inside a given ~16 m cell must sum
    /// back to that cell's aggregate elevation. Plane-zero-mean noise does **not**
    /// guarantee that — within any single cell the noise has a nonzero local
    /// integral, so it shifts that cell's materialized mean off the abstraction.
    /// So this is honestly a **first, non-conservative approximation** — the
    /// "simple-noise start" NOTES §0a explicitly licenses for now — and the
    /// conservative-refinement gap (NOTES §7) remains **open**, not satisfied. The
    /// principled version constrains the noise to conserve each coarse cell's mean
    /// (a multigrid/wavelet detail that integrates to zero *per cell*); that is the
    /// same shape as the AAT fidelity-invariant concern, so it is not throwaway.
    /// Coordinates are offset so this stream is decorrelated from the continental FBM.
    fn detail_relief(&self, xm: f32, zm: f32) -> f32 {
        const FREQ: f32 = 1.0 / 220.0; // ~220 m base wavelength
        const AMPLITUDE: f32 = 28.0; // metres of roughness about the macro slope
        const OCTAVES: u32 = 5; // down to ~14 m features — the erosion-cell scale
        const OFFSET: f32 = 9173.0; // decorrelate from the macro relief stream
        self.fbm((xm + OFFSET) * FREQ, (zm + OFFSET) * FREQ, OCTAVES) * AMPLITUDE
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
        let a = Volume::eroded(0xC0FFEE, 2, 1600, 12);
        let b = Volume::eroded(0xC0FFEE, 2, 1600, 12);
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
        let v = Volume::eroded(0x1234_5678, 1, 2000, 60);
        let w = Volume::eroded(0x1234_5678, 1, 2000, 60);
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
    fn eroded_refined_is_reproducible_and_differs_from_coarse() {
        // The fine-refinement pass must stay deterministic, and a few fine epochs
        // at a sub-16 m cell must actually change the surface vs the coarse-only run
        // (it is carving something). 4 m cells over a 1.6 km region ≈ 800² nodes.
        let coarse = Volume::eroded(0xC0FFEE, 2, 800, 12);
        let a = Volume::eroded_refined(0xC0FFEE, 2, 800, 12, 4.0, 4);
        let b = Volume::eroded_refined(0xC0FFEE, 2, 800, 12, 4.0, 4);
        let mut differs = false;
        for z in -80..80 {
            for x in -80..80 {
                assert_eq!(
                    a.surface_height(x, z),
                    b.surface_height(x, z),
                    "refined terrain diverged between runs at ({x},{z})"
                );
                if a.surface_height(x, z) != coarse.surface_height(x, z) {
                    differs = true;
                }
            }
        }
        assert!(differs, "fine refinement left the surface identical to coarse");
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
}
