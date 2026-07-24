---
slug: norm-regime-probes
type: normative
status: exact
stage: draft
depends:
  - norm-probes-before-claims
  - norm-probe-sensitivity
---

# Regime probes ship with every rung

Every aspect rung of the fidelity ladder ships **regime probes** — cheap, renderer-free instruments that assert invariants nature guarantees in a known flow (or process) regime. Known issues get their probe written **first**; physics is then fixed until the probe passes.

## Formal Expression

1. **Regime probe.** A *regime probe* is an instrument that asserts a falsifiable physical (or geometric) invariant that holds in a named regime: e.g. subcritical channel flow is smooth; curl-free gravity-driven routing on a cone; face-seam continuity of solid fields at one-cell arc; closed-box mass at a hanging node.
2. **Ship with the rung.** When a new aspect rung, nomos, or fidelity level is introduced, its regime probes are part of the deliverable — not a post-hoc campaign after the picture looks good.
3. **Known issues first.** When a defect is known or suspected, write the probe that would convict it **before** (or as the first step of) the fix. Domain-level TDD: green means the named regime holds, not that the renderer is pretty.
4. **Sensitivity still binds.** A regime probe that cannot fail, or that is coarse enough to mask the fault, is not yet a probe ( #norm-probe-sensitivity ). Prefer known-bad configurations and scale-separated discrimination.
5. **Relation to general probe law.** #norm-probes-before-claims is the claim-level gate (no physics claim without a convicting probe). This norm is the **engineering rhythm** for building systems: probes are co-delivered with rungs, not only attached when someone writes a segment.
6. **Seam probes included.** Space and time seams are first-class probe targets (`seam_ridge`, continuity across face/tile edges, multirate stationarity). Artifacts hide at seams; regime probes are how honesty stays measurable there.

## Epistemic Status

**Max attainable: exact** as process law under Joseph's TDD framing (DESIGN-REDUX §2b; channel_profile / travelling-blob episode as specimen). **Currently `exact`** for the methodology; individual probes remain instruments with their own pass/fail state.

Stage `draft`. Does not invent new physics claims — only the co-delivery rule.

## Discussion

The travelling-blob hunt was won by an instrument asserting *subcritical flow must be smooth*, not by staring at the render. That is the texture of this norm: invariants first, then code until they hold.

## Working Notes

- Specimens stay in code/examples (`channel_profile`, `seam_ridge`, `gen` continuity test, wavelet-store PROBE 7) — not re-narrated as law here.
- `#norm-probe-sensitivity` owns discrimination; this owns co-delivery with rungs.
- Dual-home: DESIGN-REDUX §2b methodology block → pointer to this slug; keep specimens.
