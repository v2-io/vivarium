---
slug: form-engine-bevy
type: formulation
status: exact
stage: draft
depends:
  - form-core-view-wall
  - post-determinism-as-ontology
---

# Engine: Bevy (Rust)

The runtime **view** engine is **Bevy**. The decision is empirical for this project’s axes (determinism, headless, ECS cognitive LOD, Rust core without FFI seam), not a general graphics ranking.

## Formal Expression

1. **Decision.** Bevy is the chosen engine for playable views and ECS-shaped world/agent components. Resolved 2026-06-20; confirmed 2026-06-22 by feature-parity spike against Godot voxel path.
2. **Why (project-local).** ECS for tiles/items and cognitive LOD component swap; determinism + headless for tether / *in vivia*; no gdext/FFI seam for a Rust world frame; voxel/volumetric path available. Accepted cost: slower art iteration vs Godot — mitigated by simple art pipeline.
3. **Not the world law.** World frame and law live in `vivarium-world` (and successors) with no render dependency ( #form-core-view-wall ). Bevy is a **view/runtime host**, not claim canon for physics.
4. **Instrument.** Comparison confounds and measurements: `spikes/FINDINGS.md`. Godot spike preserved under `archive/` for archaeology.

## Epistemic Status

**Max attainable: exact** as project engine choice (Joseph-era decision + measured spike). **Currently `exact`** for “Bevy is the engine we use.” Stage `draft`. Not a claim that Bevy is best for all games.

## Discussion

Engine choice is a wall only in the sense that thrashing it re-opens the FFI and determinism costs the frame already paid.

## Working Notes

- Peel from DESIGN.md engine section. Do not re-litigate without new measurements.
