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
macro owns elevation); FILL mode (deluge to steady state) then a living
storm phase with two-way bed write-back.
**Correction (2026-07-03):** the 07-02 claim that the kill-switch goal was
"REALIZED" was premature — parity night had quietly re-imported core's
sediment-off switch into the fill phase, and the living phase (sediment on)
was in practice unreachable (fill takes tens of minutes and restarts every
launch). Joseph's probing surfaced it. The momentum fixes (θ-smoothing, sill
conveyance, breaking cap, rate caps) removed the instabilities that justified
core's switch — probe-verified (channel_profile sediment regimes: zero bed
kinks, work bounded by sed_max_rate) — so **sediment now runs through the
fill as well**: the kill-switch is deleted for real, and the fill matures the
beds (alluvium, colmation, armor) before the living phase begins.
OPEN: discharge→A coupling into the erosion tiers (real flow driving stream
power); watershed-boundary inflows (nested water grids); seams/persistence
(§13 store) — the next session's agenda; per-material erodibility/permeability.

## Next: analytic hydrological initialization (planned 2026-07-03; replaces the fill)

*Intent, constraints, and shape — not a prescription. Whoever builds this will
see things we can't from here.*

**The realization (Joseph + the day's fill pain):** the deluge fill was never
about which physics ran during it — it was a *time-integration through a false
regime* to reach a state that is actually a **function, not a history**. The
water distribution is an equilibrium aspect (`DESIGN-REDUX` §3, the 07-03
amendment): steady state = f(terrain, climatological forcing). Functions get
solved. Twenty minutes of unearthly rain (with either sediment switch state)
was us evaluating that function by the most expensive and least honest method
available.

**The shape of the solve** — every piece already exists in the erosion tier:
- Lakes: Priority-Flood with spill volumes (basins fill to their outlets).
- Rivers: steady discharge from drainage area × MEAN living forcing (rain ×
  storm duty cycle — the forcing the world will actually experience), then
  Manning normal depth per reach — the live sim's own friction law, inverted.
  Use the same slope-dependent (Jarrett) roughness or the estimate will
  disagree with the sim it seeds.
- Groundwater: recharge/baseflow balance, one line of algebra.
- Interface state (the path-dependent part): handed over by the SLOW tier,
  which already time-integrated the eons — alluvium where Davy–Lague
  net-deposited, armor where stream-power net-incised, colmation from
  equilibrium fines flux in channels. This is §4's multirate contract done
  properly: slow provides time-averaged state to fast; fast never fudges eons.
- Then a SHORT true relaxation (~a minute of sim-time) from the analytic seed,
  because the estimates are estimates. Cache the result: "an ordinary morning
  of year zero." Live + storms from second one, every launch.

**What dies:** the deluge, the FILLING phase, the plateau detector, the fill's
compressed-clock geomorphic work (which sealed/washed the whole world in a
regime that never existed). **What keeps us honest:** a new regime probe —
*the analytic seed must be near-stationary under the live sim* (small
per-burst delta from step one). If the seed drifts hard, the solver and the
sim disagree about equilibrium, and that disagreement is a finding, not a
nuisance.

**Known unknowns:** closed-basin lake levels want an evaporation balance, not
just spill; braided/multi-thread reaches violate the single-normal-depth
assumption (probably fine at 4.8 m cells; check); how much interface-state
detail the slow tier can honestly claim to know (alluvium yes, colmation is
shakier — mark exactness accordingly, §9).
