---
slug: norm-probe-sensitivity
type: normative
status: exact
stage: draft
depends:
  - norm-probes-before-claims
---

# Probe sensitivity is part of the probe

A green probe certifies only what its discrimination covers. Before trusting a pass, state what magnitude of violation the probe would have caught — and, when possible, run it against a known-bad configuration first.

## Formal Expression

1. **Discrimination, not colour.** A probe that returns green is evidence only inside the class of faults it can resolve. Sensitivity (minimum detectable violation at the chosen footprint, arc, level, and tolerance) is part of the probe's claim, not an optional note.
2. **Known-bad first when available.** When a historical or synthetic broken configuration exists (e.g. the v1 per-face discontinuous prior), the probe should fail on it. A probe that cannot fail on a known-bad is not yet a probe for that fault class ( #norm-probes-before-claims FE(2) specialised).
3. **Scale-separation of the signal.** For continuity / seam faults: a true discontinuity's delta **plateaus** as the sampling arc shrinks; honest continuous variation **vanishes**. Measure across scales (or at an arc known to be finer than the expected cliff), not at a single coarse arc that lets within-face noise mask the edge.
4. **Bit-identical tells.** A ratio or statistic that is bit-identical across independent samples or age gaps is usually a clamp, divide-by-zero floor, or no-op footprint — not a physical measurement. Treat that signature as a probe defect until proven otherwise (companion of #norm-probes-before-claims FE(3)).
5. **Scope.** This norm specialises probes-before-claims; it does not replace it. It does not license fewer probes — it forbids mistaking an insensitive green for a convicting pass.

## Epistemic Status

**Max attainable: exact** as process law. Paid-for lesson 2026-07-10: the first face-seam continuity probe **passed on a prior with measured ~3 km cliffs** because its sampling arc was coarse enough that within-face variation masked the discontinuity (`msc/reflections/session-2026-07-10-mechanics.md` §1; live comments on `gen::prior_is_continuous_across_faces_and_corners`). Companion DECISIONS specimens (seabed no-op footprints, "22888" seam ratio as divide-by-zero) are the same failure species under different costumes.

Stage `draft`. Authority: Joseph-era session correction + `:by joseph` probes-before-claims; this segment names the sensitivity half that was not yet segmented.

## Discussion

House culture already said "write probes." The residual failure mode is subtler: writing a probe that *looks* like it tests the claim while sampling below the fault's frequency. The body-signal is relief at a first green on a hard problem — that is when to ask what the probe would have missed.

## Working Notes

- **Specimen (continuity):** L10 / arc $3\cdot10^{-3}$ green on discontinuous v1; one-cell arc at L12 discriminates ~7× (cross-seam ~3000 m vs within-face ~440 m). Live generator test uses one-cell arc + factor-4 bound.
- **Specimen (seabed / no-op):** `DECISIONS[seam-probes-were-measuring-seabed]` — tests passed by comparing no-ops.
- **Companions in same reflection file (not this claim):** compensating bugs defeat pure derivation (§2); honesty machinery generates its own work queue (§5) — extract later if needed.
- **Design dual-home:** DESIGN-REDUX §2b "regime probes" remains source; this segment owns the *sensitivity* law. Broader "every rung ships regime probes" is residual extract (`#norm-regime-probes` in design-mining remaining list).
