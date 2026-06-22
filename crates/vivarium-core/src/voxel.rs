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

/// Vertical span of the world, in voxels, measured from `y = 0` at the bottom.
/// Generation never produces solid voxels at or above this; it is the ceiling
/// of the abstraction tier, not a wall an agent can hit. Chosen tall enough for
/// hills + headroom and no taller, so the surface-scan helpers terminate fast.
pub const WORLD_HEIGHT: i32 = 128;

/// The `y` below which empty space is filled with water rather than air. Gives
/// the seed world lakes and coastlines, so a first-person spike has something
/// to stand at the edge of. Purely a worldgen choice.
pub const SEA_LEVEL: i32 = 24;

/// The world as a volumetric field. Cheap to clone (the overlay is the only
/// heap state) — clone it to snapshot, diff two of them to study a trajectory,
/// exactly like the rest of [`crate::World`].
#[derive(Clone, Debug)]
pub struct Volume {
    /// Seed for the pure-function terrain. Distinct from (and derived from) the
    /// world seed so that changing agent dynamics never silently reshapes the
    /// ground, and vice versa.
    seed: u64,
    /// The only materialized state: voxels changed away from what the seed
    /// would generate. Ordered for deterministic replay; sparse so the cost is
    /// proportional to how much the world has been *touched*, not its size.
    edits: BTreeMap<[i32; 3], Voxel>,
}

impl Volume {
    pub fn new(seed: u64) -> Self {
        // Decorrelate the terrain stream from any other use of the world seed.
        let mut r = Rng::new(seed);
        Self { seed: r.next_u64(), edits: BTreeMap::new() }
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
        if y < 0 || y >= WORLD_HEIGHT {
            return Voxel::AIR;
        }
        let h = self.terrain_height(x, z);
        if y > h {
            // Above ground: water up to sea level, else air.
            return if y <= SEA_LEVEL { Voxel::WATER } else { Voxel::AIR };
        }
        if y == h {
            // The surface skin. Below the waterline it is dirt (lakebed), above
            // it is grass — a cheap, legible bit of texture for the eye.
            return if h < SEA_LEVEL { Voxel::DIRT } else { Voxel::GRASS };
        }
        if y > h - 4 {
            Voxel::DIRT
        } else {
            Voxel::STONE
        }
    }

    /// Terrain surface height at `(x, z)` — value noise on an integer lattice,
    /// smoothed and scaled into rolling hills. Deterministic and continuous, so
    /// neighbouring columns differ by at most a voxel or two and the mesher gets
    /// coherent surfaces rather than noise.
    fn terrain_height(&self, x: i32, z: i32) -> i32 {
        // One octave is enough for a spike; add octaves here if the hills read
        // as too smooth once you can walk them.
        const FREQ: f32 = 1.0 / 24.0; // larger denominator => broader hills
        const AMPLITUDE: f32 = 18.0;
        const BASE: f32 = 28.0; // mean ground height, a few voxels above sea

        let n = self.value_noise(x as f32 * FREQ, z as f32 * FREQ);
        (BASE + n * AMPLITUDE).round() as i32
    }

    /// 2D value noise in `[-1, 1]`. Bilinear interpolation of per-lattice-point
    /// pseudo-random values, with a smoothstep on the fractional coordinate so
    /// the result is visually smooth (no lattice creases).
    fn value_noise(&self, x: f32, z: f32) -> f32 {
        let x0 = x.floor();
        let z0 = z.floor();
        let fx = smoothstep(x - x0);
        let fz = smoothstep(z - z0);
        let (xi, zi) = (x0 as i64, z0 as i64);

        let v00 = self.lattice(xi, zi);
        let v10 = self.lattice(xi + 1, zi);
        let v01 = self.lattice(xi, zi + 1);
        let v11 = self.lattice(xi + 1, zi + 1);

        let bottom = lerp(v00, v10, fx);
        let top = lerp(v01, v11, fx);
        lerp(bottom, top, fz) * 2.0 - 1.0
    }

    /// A stable pseudo-random value in `[0, 1)` for an integer lattice point.
    /// Hashes `(seed, xi, zi)` through the same SplitMix64 mix the world PRNG
    /// uses, so the noise field is reproducible and shares the determinism
    /// guarantee rather than introducing a second, unaudited source of state.
    fn lattice(&self, xi: i64, zi: i64) -> f32 {
        let mut h = self.seed;
        for w in [xi as u64, zi as u64] {
            h ^= w.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            h = (h ^ (h >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            h = (h ^ (h >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            h ^= h >> 31;
        }
        (h >> 40) as f32 / (1u32 << 24) as f32
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Hermite smoothstep `3t² − 2t³`, mapping `[0,1]→[0,1]` with zero slope at the
/// ends — the standard trick that turns blocky bilinear interpolation into
/// visually smooth noise.
#[inline]
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
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
    fn terrain_is_continuous() {
        // Adjacent columns should not differ wildly — the mesher and any walking
        // agent rely on the surface being walkable, not a cliff per voxel.
        let v = Volume::new(99);
        let mut prev = v.surface_height(0, 0).unwrap();
        for x in 1..200 {
            let h = v.surface_height(x, 0).unwrap();
            assert!((h - prev).abs() <= 3, "height jump of {} at x={}", h - prev, x);
            prev = h;
        }
    }
}
