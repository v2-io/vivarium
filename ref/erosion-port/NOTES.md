# Erosion + hydrology port — plan (onto vivarium-world, as multirate-coupled tiers)

*The first real physics port onto the clean-room frame, and deliberately the first
real test of `DESIGN-REDUX.md` §4 **multirate coupling** / heterogeneous multiscale
mixing — replacing the current ad-hoc phasing. Bridge decision (confirmed): **port
the algorithm, not the data** (ORIENTATION §next-2).*

## What core does today (the thing being ported)

`vivarium-core`'s worldgen (detail in [`ref/geology/NOTES.md`](../geology/NOTES.md)
and [`ref/hydrology/NOTES.md`](../hydrology/NOTES.md)): FBM prior → **MFD
stream-power erosion** → **Davy–Lague deposition**, run as an **ad-hoc 3-phase
pipeline** on a flat `i32` patch:

1. **coarse-grained epochs** (macro erosion),
2. **fine-grained epochs** (finer erosion, carves channels),
3. **water-settling** to steady state — **with erosion + sediment settling turned
   OFF** (a crude timescale separation; the known "rain ~100–1000× real" fudge).

Turning erosion off during settling is the ad-hoc kludge we want to remove.

## The re-framing: coupled multirate, not phases

Erosion is *geological* (slow); water is *hydrological* (fast). Instead of three
hand-sequenced phases with erosion disabled in the last one, express them as
**coupled tiers on their own timesteps** (§4):

- **water** advances on its fast step, seeing terrain as **quasi-static**;
- **erosion** advances on its slow step, seeing water as a **time-averaged**
  discharge/flux forcing.

Then **erosion + sediment settling can stay ON throughout** — the separation is
achieved by the *coupling schedule*, not by a kill-switch. This is exactly the
"separate the timescales, don't couple them on one timestep" lesson (`CLAUDE.md`),
now expressed as principled HMM coupling rather than manual phasing. It is the
cleanest available test of the whole multirate idea on a system we already
understand.

## Substrate + wiring

- Runs on **Cartesian patches** (`chunk.rs`) — dense row-major fields, `idx±1`
  neighbours, **halos** at patch + cube-face seams (the bench:
  `ref/research/spatial-key-bench.md`). The curve (`CellId`) only addresses the
  patch; the stencil is pure Cartesian.
- Fields are the `(b, d, r)` trio (terrain height, water depth, regolith/sediment)
  from `DESIGN-MATERIAL.md` §3 — separate `Patch<f32>` per field (SoA, cache-friendly).
- **FBM prior** comes from the frame's own `noise` (§8), not core's noise.
- Output feeds **`gen::column_from_surface`**: the eroded surface height + regolith
  → strata. So the eroded result becomes real `Column`s.

## Improvements to fold in while porting

- **Erosion + sediment ON during settling** (the main win above).
- **Per-material erodibility** (`Material::erodibility` / `incision_threshold`,
  already in the property set) → *differential* erosion → layered relief
  (Bryce-style, `DESIGN-SYSTEMS.md`). Core assumed a single hardness.
- Determinism: all stochasticity via coordinate hash (§8), so runs are replayable
  and memoizable (§11–12) — a precondition core's shared-RNG did not meet.

## Open (work out during implementation)

- The exact **coupling schedule** — the fast/slow step ratio and the fluxed
  quantities (discharge in, elevation-change out). Keep the timescales separate.
- How steady-state / termination is decided under continuous coupling (vs. the old
  fixed epoch counts).
- Patch/halo handling at the 8 cube-face **corners** (the mild cube-sphere
  distortion points).

## Status (2026-07-02, end of the water night)
DONE: the fluvial pipeline (ported + hillslope creep added — spike_probe caught
the no-diffusion spire instability, latent in core too); ErodedRegion telescope
sampling; the water FAST band with real physics (Saint-Venant + Manning,
critical-shear incision, slope-capacity sediment, groundwater + live colmation,
closed cycle, ocean-as-ground); mean-conservation pin (fine tiers redistribute,
macro owns elevation); SETTLE mode reproducing core's proven sequence, then a
living storm phase with two-way bed write-back. Erosion stays ON during water —
the multirate goal of this document is REALIZED (as a schedule, not a switch).
OPEN: discharge→A coupling into the erosion tiers (real flow driving stream
power); watershed-boundary inflows (nested water grids); seams/persistence
(§13 store) — the next session's agenda; per-material erodibility/permeability.
