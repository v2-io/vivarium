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
}

impl ErodedSurface {
    /// Bilinearly sample the surface (metres) at a metre position, **clamping** to
    /// the patch edge outside it: beyond the eroded region the coastline simply
    /// extends into open sea, rather than presenting a cliff or a seam back to raw
    /// FBM. Always returns a height.
    fn sample(&self, xm: f32, zm: f32) -> f32 {
        let gx = ((xm - self.x0_m) / self.cell_m).clamp(0.0, (self.nx - 1) as f32);
        let gz = ((zm - self.z0_m) / self.cell_m).clamp(0.0, (self.nx - 1) as f32);
        let (x0, z0) = (gx.floor() as usize, gz.floor() as usize);
        let (x1, z1) = ((x0 + 1).min(self.nx - 1), (z0 + 1).min(self.nx - 1));
        let (tx, tz) = (gx - x0 as f32, gz - z0 as f32);
        let h00 = self.h_m[z0 * self.nx + x0];
        let h10 = self.h_m[z0 * self.nx + x1];
        let h01 = self.h_m[z1 * self.nx + x0];
        let h11 = self.h_m[z1 * self.nx + x1];
        let a = h00 + (h10 - h00) * tx;
        let b = h01 + (h11 - h01) * tx;
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
            k: 0.02,     // erodibility — dissects with MFD without razing the relief
            m: 0.5,
            max_slope: 1.2, // talus repose ~50° — steep mountain flanks allowed
            epochs,
            dt: 1.0,
            sea_level: Some(SEA_LEVEL as f32),
        };
        let field = crate::geo::Heightfield::from_heights(nx, cell, h).erode(&params);
        v.eroded = Some(ErodedSurface { x0_m: origin, z0_m: origin, cell_m: cell, nx, h_m: field.h });
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
        let sea = self.sea_level();
        let h = self.terrain_height(x, z);
        if y > h {
            // Above ground: water up to sea level, else air.
            return if y <= sea { Voxel::WATER } else { Voxel::AIR };
        }
        if y == h {
            // The surface skin. Below the waterline it is dirt (lakebed), above
            // it is grass — a cheap, legible bit of texture for the eye.
            return if h < sea { Voxel::DIRT } else { Voxel::GRASS };
        }
        // Soil is a fixed *physical* depth ([`SOIL_DEPTH`] metres), so it scales
        // with resolution; below it is stone.
        if y > h - SOIL_DEPTH * self.detail {
            Voxel::DIRT
        } else {
            Voxel::STONE
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
            // Macro form from the erosion tier, plus sub-grid detail relief so the
            // 0.5 m voxels read as real ground, not a staircase quantization of a
            // smooth ramp (see [`Self::detail_relief`]).
            Some(e) => e.sample(xm, zm) + self.detail_relief(xm, zm),
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
    /// It is **zero-mean**, so it does not shift the macro elevation — the
    /// materialized detail stays statistically consistent with the abstraction it
    /// refines (DESIGN.md's fidelity invariant; a first, simple-noise take on the
    /// conservative-refinement gap, NOTES §7). Coordinates are offset so this
    /// stream is decorrelated from the continental FBM.
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
        const FREQ: f32 = 1.0 / 22_000.0; // ~22 km base wavelength (metres)
        const AMPLITUDE: f32 = 2_300.0; // metres of relief about the base
        const BASE: f32 = 3_200.0; // metres — just above sea level (3 km)
        const OCTAVES: u32 = 6;
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
