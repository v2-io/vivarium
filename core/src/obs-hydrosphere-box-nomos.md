---
slug: obs-hydrosphere-box-nomos
type: observation
status: empirical
stage: draft
depends:
  - def-nomos
  - form-flux-web
  - form-complete-content-addressed-key
---

# Hydrosphere is a box nomos on the same contract

> [!warning] **Code-first trail (not yet claim-complete)** — base `ce55ddf` (2026-07-23). **Derived sea level** (ocean stock poured into solid hypsometry) is being implemented in `sea_level.rs` / gen; it is **not** weather-cycle bootstrap (insolation and climate already have their chains). This observation still only claims the box stock + precip edge. A sibling formulation for derived sea level / bathymetry vs freeboard will backfill; do not read unfinished pour code as canon of this segment.

The hydrosphere article is a global reservoir (no grid): conserved water inventory from ante-mundane mass fraction, partitioned ocean/atmosphere. It proves the store/nomotheke/flux contract is representation-agnostic.

## Formal Expression

1. **Shape.** Hydrosphere is a **box / reservoir** nomos: stocks and fluxes without a spatial field. It keys by world identity, not by face/tile coordinates.
2. **Conservation.** Total water inventory is derived from a declared planetary water-mass fraction; partition into ocean (+ice/gw lumped) and atmosphere is exact in the unit tests; not conjured per tile.
3. **Downstream.** Climate consumes atmosphere water and promises precipitation; erosion and water-tile consume precipitation. The chain is visible in the flux web ( #form-flux-web ).
4. **Generality.** The same declaration and key discipline that field nomoi use apply here — so "field-on-a-grid" is not the framework primitive (ARCHITECTURE domain-fixation guard as source; this is the measured instance).

## Epistemic Status

**Max attainable: empirical** for the live hydrosphere article; **exact** for "a box can be a nomos" as existence proof once the code and tests hold. Live: `hydrosphere.rs`, nomotheke entry, `vivarium status` water budget. Stage `draft`. Climate geography and ice/groundwater as separate reservoirs remain next rungs, not claimed done.

## Discussion

Without a non-field instance, "representation-agnostic" is a slogan. Hydrosphere is the cheap proof that the flux web is not secretly a grid API.

## Working Notes

- Pointer ice: ARCHITECTURE §0 domain-fixation paragraph can cite this observation instead of only gesturing.
- Do not claim Earth budget fidelity beyond order-of-magnitude checks already in tests.
