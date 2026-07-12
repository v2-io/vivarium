//! `vivarium-world` — the clean-room world **frame** (see `doc/design/DESIGN-REDUX.md`).
//!
//! A planet on a cube-sphere, time that spans deep-geological to sub-second, and
//! quantities that carry their own meaning. This is the foundation the proven
//! worldgen physics currently in `vivarium-core` (FBM, erosion, hydrology) will
//! migrate onto as fidelity-ladder *tiers* — it does not replace that physics, it
//! gives it an honest coordinate/time/quantity frame it never had.
//!
//! It has **no rendering dependency** — the core/view wall holds. Every module
//! here aims to be the *idiom* later tiers copy: pure, deterministic, and keyable
//! (so results memoize by a complete content-addressed key — `doc/design/DESIGN-REDUX.md`
//! §11–12), with rich [`quantity`] values at seams and raw `f64` in hot loops.
//!
//! **Status: skeleton.** It establishes the shapes and the first tier
//! ([`planet`]'s insolation); it is being validated by a research pass before
//! build-out. Expect churn — do not treat any signature here as settled.

pub mod quantity;
pub mod time;
pub mod sphere;
pub mod planet;
pub mod material;
pub mod column;
pub mod noise;
pub mod gen;
pub mod chunk;
pub mod erosion;
pub mod water;
pub mod sample;
pub mod store;
pub mod query;
pub mod spec;
pub mod flux;
pub mod nomotheke;
pub mod audit;
