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

The hydrosphere article is a global reservoir (no grid): conserved water inventory from ante-mundane mass fraction, partitioned ocean/atmosphere. It proves the store/nomotheke/flux contract is representation-agnostic. Ocean stock feeds derived sea level ( #form-derived-sea-level ) — that pour is **not** weather-cycle bootstrap.

## Formal Expression

1. **Shape.** Hydrosphere is a **box / reservoir** nomos: stocks and fluxes without a spatial field. It keys by world identity, not by face/tile coordinates.
2. **Conservation.** Total water inventory is derived from a declared planetary water-mass fraction; partition into ocean (+ice/gw lumped) and atmosphere is exact in the unit tests; not conjured per tile.
3. **Downstream.** Climate consumes atmosphere water and promises precipitation; erosion and water-tile consume precipitation. The chain is visible in the flux web ( #form-flux-web ). Ocean stock is also the inventory poured into hypsometry for derived sea level ( #form-derived-sea-level ) — a separate edge from precip.
4. **Generality.** The same declaration and key discipline that field nomoi use apply here — so "field-on-a-grid" is not the framework primitive (ARCHITECTURE domain-fixation guard as source; this is the measured instance).

## Epistemic Status

**Max attainable: empirical** for the live hydrosphere article; **exact** for "a box can be a nomos" as existence proof once the code and tests hold. Live: `hydrosphere.rs`, nomotheke entry, `vivarium status` water budget. Stage `draft`. Climate geography and ice/groundwater as separate reservoirs remain next rungs, not claimed done. Sea-level pour is not claimed here ( #form-derived-sea-level ).

## Discussion

Without a non-field instance, "representation-agnostic" is a slogan. Hydrosphere is the cheap proof that the flux web is not secretly a grid API.

## Working Notes

- Pointer ice: ARCHITECTURE §0 domain-fixation paragraph can cite this observation instead of only gesturing.
- Do not claim Earth budget fidelity beyond order-of-magnitude checks already in tests.
- Pour path extracted: #form-derived-sea-level . Drop residual banner when that FE's gen/nomotheke Known-incomplete closes.
