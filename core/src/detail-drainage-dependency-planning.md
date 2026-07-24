---
slug: detail-drainage-dependency-planning
type: detail
status: robust-qualitative
stage: draft
depends:
  - form-seam-flux-exchange
  - form-fidelity-invariant
  - form-three-scoped-runtime
  - form-depend-by-key-never-latest
  - form-pull-query-composition
  - detail-seam-precedents
---

# Drainage-shaped dependency planning

For fluvial (and similar) state, the dependency cone of a point is not a Euclidean halo — it is the drainage island. The coarse spine draws the island map; flux magnitude sets pull fidelity.

## Formal Expression

1. **Island of interdependence.** Upstream catchment closure (discharge/sediment are line-integrals uphill) **plus** downstream path to base level (outlet elevation controls incision). Hillslope diffusion couples across divides only at one-cell range (halos). The island is irregular by nature.

2. **Spine is the planner.** Priority-Flood + receivers + MFD (or successor) at coarse global level produce drainage graph, accumulation, basin partition, base levels — not only elevations. A nomos for "eroded tile $X$ at level $L$" declares inputs = tiles its coarse catchment intersects, discovered from the spine, not assumed from adjacency. Cross-face coarse-global assembly is integration work; basin-partition stability across levels is an **open measurement** ( #detail-seam-precedents region-generation efficiency ratio is prior art for tiling).

3. **Degree is flux magnitude.** Coupling strength across a tile edge **is** the discharge (and sediment) crossing it, read from coarse accumulation. Small creek → coarse BC approximation may suffice; major river → upstream must be evolved or its time-averaged flux memoized. **Flux is the sufficient statistic; magnitude sets required pull fidelity** ( #form-seam-flux-exchange · #form-fidelity-invariant : coarsest rung whose edge flux error stays in tolerance).

4. **Seam fix = composability.** Cosmetic clamps on floating mesas do not replace honest BCs from spine fluxes. `seam_ridge` is a correctness gate, not a cosmetic one.

5. **Status-quo kernel debt (measured).** Patches seed drainage at own area (zero external discharge); outlets hardcoded edge+sea — tiles systematically starved of out-of-window catchment.

6. **Risks.** Bidirectional water exchange (backwater) harder than stream-power composition; multirate quasi-static stage BCs need their own probe; time-index across differentially aged tiles must be in the key; spine too coarse mis-draws divides.

## Epistemic Status

**Max attainable: robust-qualitative** as design. **Currently `robust-qualitative`.** Stage `draft`.

## Discussion

Joseph's dependency question ("what coarse evolution needs to run adjacent, lazily?") has this answer: the drainage graph itself.

## Working Notes

- Supersedes framework-to-status-quo §3 as home.
- Build shape A–E (store, spine, flux-BC tiles, query front-end, time-indexed stages) lives with #detail-abyssal-parity-build and #form-three-scoped-runtime .
