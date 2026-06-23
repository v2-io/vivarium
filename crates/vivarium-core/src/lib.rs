//! vivarium-core — the simulation.
//!
//! This crate knows nothing about pixels, windows, audio, or any particular
//! way of *looking* at the world. It is the world. Renderers, headless
//! loggers, and language interfaces for logozoetic players all sit *outside*
//! this crate and read from it.
//!
//! Two properties are deliberate and load-bearing (see ../../DESIGN.md):
//!
//!  1. **Determinism as ontology.** The entire world is a pure function of
//!     `(seed, number-of-steps-taken)`. There is no hidden, non-reproducible
//!     state — randomness comes only from the seeded [`Rng`] below, never from
//!     the OS clock or thread scheduling. This is what lets the vivarium be a
//!     genuine "tether to truth": consequences follow *exactly* from causes,
//!     and any moment can be re-derived and replayed. An agent can trust this
//!     world the way it trusts arithmetic.
//!
//!  2. **The world is the substrate, the view is disposable.** Everything here
//!     is plain data and plain functions. A view can be thrown away and
//!     rewritten without touching the simulation — the BotW-2D-prototype
//!     insight, made structural.
//!
//! This is a *seed* — a few wandering agents with a single scalar need, walking
//! the surface of a volumetric world — chosen to be the smallest thing that
//! visibly *lives* on screen. It is not the agent model; the ASF/AAT two-layer
//! mind (DESIGN.md) replaces `Agent::step` when we get there. Treat everything
//! here as scaffolding for motion, not as a commitment to mechanics.
//!
//! The world is 3D — see [`voxel`] for the volumetric substrate and the
//! reasoning behind storing the world as `seed + sparse edits`.

pub mod geo;
pub mod voxel;

use voxel::Volume;

/// A small, fast, fully-deterministic PRNG (SplitMix64).
///
/// Chosen over the `rand` crate on purpose: the whole point of the simulation
/// is that nothing varies between two runs of the same seed, and the cheapest
/// way to *prove* that to a future reader is for the generator to be a few
/// lines they can see. SplitMix64 is good enough for world-wandering and
/// trivially seedable; swap it for something stronger only if a mechanic ever
/// needs statistical-test-grade randomness (record the reason if so).
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advance the generator and return 64 raw bits.
    pub fn next_u64(&mut self) -> u64 {
        // SplitMix64 — Steele, Lea & Flood (2014).
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A float in `[0.0, 1.0)`.
    pub fn next_f32(&mut self) -> f32 {
        // Top 24 bits give a uniform value over the unit interval.
        (self.next_u64() >> 40) as f32 / (1u32 << 24) as f32
    }

    /// A float in `[-1.0, 1.0)`. Convenient for symmetric nudges.
    pub fn next_signed(&mut self) -> f32 {
        self.next_f32() * 2.0 - 1.0
    }
}

/// One inhabitant. A 3D position and velocity, plus a single scalar `need` in
/// `[0, 1]` standing in for the whole interior life an ASF agent will later
/// have. Keeping it to one scalar makes the eventual jump to the formal
/// fast-layer state legible by contrast.
///
/// The agent wanders in the horizontal (`x`,`z`) plane and is held on the
/// terrain surface in `y` — it walks the ground rather than flying. Gravity,
/// jumping, and real locomotion are deliberately absent: this is motion to look
/// at, not a physics commitment.
#[derive(Clone, Debug)]
pub struct Agent {
    pub pos: [f32; 3],
    pub vel: [f32; 3],
    pub need: f32,
}

/// The whole world. Plain data — clone it to snapshot, serialize it to persist,
/// diff two of them to study a trajectory.
#[derive(Clone, Debug)]
pub struct World {
    pub seed: u64,
    pub step_count: u64,
    /// Half-extent of the square play area in the horizontal plane; agents
    /// wander within `[-bound, bound]²` in `x`/`z`. The volume itself is
    /// unbounded — this only fences the wandering agents so they stay in view.
    pub bound: f32,
    /// The volumetric world the agents walk on (see [`voxel::Volume`]).
    pub volume: Volume,
    pub agents: Vec<Agent>,
    rng: Rng,
}

impl World {
    /// Build the initial world deterministically from a seed, at base voxel
    /// resolution (`detail = 1`).
    pub fn new(seed: u64, n_agents: usize) -> Self {
        Self::with_detail(seed, n_agents, 1)
    }

    /// Build the world at a chosen voxel resolution (see [`voxel::Detail`]). The
    /// agents' wander area scales with `detail` so they roam a comparable
    /// *physical* region regardless of resolution.
    pub fn with_detail(seed: u64, n_agents: usize, detail: voxel::Detail) -> Self {
        let mut rng = Rng::new(seed);
        let volume = Volume::with_detail(seed, detail);
        let bound = 64.0 * volume.detail() as f32;
        let agents = (0..n_agents)
            .map(|_| {
                let x = rng.next_signed() * bound;
                let z = rng.next_signed() * bound;
                Agent {
                    pos: [x, surface_y(&volume, x, z), z],
                    vel: [rng.next_signed() * 8.0, 0.0, rng.next_signed() * 8.0],
                    need: rng.next_f32(),
                }
            })
            .collect();
        Self { seed, step_count: 0, bound, volume, agents, rng }
    }

    /// Advance the world by one fixed step of `dt` seconds.
    ///
    /// Placeholder dynamics: agents wander (a small seeded nudge to velocity),
    /// integrate, and bounce off the bounds; `need` drifts slowly. The only
    /// thing that matters architecturally is that this is a *pure step over
    /// owned state* — when the two-layer mind lands, the per-agent decision is
    /// what changes here, and nothing outside this crate needs to know.
    pub fn step(&mut self, dt: f32) {
        for a in &mut self.agents {
            // Wander in the horizontal plane only; `y` is dictated by the ground.
            a.vel[0] += self.rng.next_signed() * 2.0;
            a.vel[2] += self.rng.next_signed() * 2.0;

            // Gentle speed cap so the wander stays watchable.
            let speed = (a.vel[0] * a.vel[0] + a.vel[2] * a.vel[2]).sqrt();
            let max = 12.0;
            if speed > max {
                a.vel[0] *= max / speed;
                a.vel[2] *= max / speed;
            }

            a.pos[0] += a.vel[0] * dt;
            a.pos[2] += a.vel[2] * dt;

            // Reflect off the square boundary (x and z only).
            for axis in [0, 2] {
                if a.pos[axis] > self.bound {
                    a.pos[axis] = self.bound;
                    a.vel[axis] = -a.vel[axis].abs();
                } else if a.pos[axis] < -self.bound {
                    a.pos[axis] = -self.bound;
                    a.vel[axis] = a.vel[axis].abs();
                }
            }

            // Snap to the terrain surface: the agent walks the ground. (No
            // smoothing — the surface is continuous enough that this reads as
            // walking, not teleporting. Smooth it when it looks wrong, not before.)
            a.pos[1] = surface_y(&self.volume, a.pos[0], a.pos[2]);

            // Need oscillates slowly; later this becomes real ASF dynamics.
            a.need = (a.need + dt * 0.05).fract();
        }
        self.step_count += 1;
    }
}

/// The `y` an agent's feet rest at, standing on the column under `(x, z)`. One
/// voxel above the surface so the agent sits *on* the ground, not embedded in
/// it; falls back to sea level for the (currently impossible) empty column.
fn surface_y(volume: &Volume, x: f32, z: f32) -> f32 {
    let h = volume.surface_height(x.round() as i32, z.round() as i32);
    h.map_or(volume.sea_level() as f32, |h| h as f32 + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tether-to-truth property, asserted as a test: same seed, same world,
    /// forever. If this ever fails, a non-reproducible source of state has crept
    /// in — find it and remove it before doing anything else.
    #[test]
    fn same_seed_is_bit_identical() {
        let mut a = World::new(0xC0FFEE, 16);
        let mut b = World::new(0xC0FFEE, 16);
        for _ in 0..1000 {
            a.step(1.0 / 30.0);
            b.step(1.0 / 30.0);
        }
        assert_eq!(a.step_count, b.step_count);
        for (x, y) in a.agents.iter().zip(b.agents.iter()) {
            assert_eq!(x.pos, y.pos);
            assert_eq!(x.need, y.need);
        }
    }

    /// Agents must never escape the horizontal play area, and must stay glued to
    /// the terrain surface in `y`.
    #[test]
    fn agents_stay_in_bounds_and_on_the_ground() {
        let mut w = World::new(42, 32);
        for _ in 0..2000 {
            w.step(1.0 / 30.0);
        }
        for a in &w.agents {
            assert!(a.pos[0].abs() <= w.bound + 1e-3);
            assert!(a.pos[2].abs() <= w.bound + 1e-3);
            let expected_y = surface_y(&w.volume, a.pos[0], a.pos[2]);
            assert_eq!(a.pos[1], expected_y, "agent left the ground");
        }
    }
}
