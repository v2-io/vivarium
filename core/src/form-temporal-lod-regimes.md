---
slug: form-temporal-lod-regimes
type: formulation
status: robust-qualitative
stage: draft
depends:
  - post-represent-by-consequence
  - form-fidelity-invariant
  - form-pull-query-composition
  - sketch-detail-abstract-reversion
---

# Spatial and temporal LOD are one gradient, in four regimes

For local aspects, distance-from-interest and behind-in-sim-time are the same quantity — how far the lazy pull has gone. The world ages toward the observer. Four materialization regimes govern how, and conflating them assigns an aspect the wrong (and sometimes impossibly expensive) machinery.

## Formal Expression

1. **One gradient (local aspects).** "Far from the observer" = un-pulled = coarse **and** early; "near" = fully pulled = fine **and** now. A world therefore ages toward the participant — a co-moving wavefront of *now*, with unvisited regions left coarse/early. This is the demand side of #form-pull-query-composition read jointly on the space and time axes of #form-fidelity-invariant .
2. **Regime G — global aspects (tectonics, climate, sea level): the exception.** A plate is one planet-spanning object; half of it cannot be held in the past. The global spine advances **time-uniformly** (cheap, because coarse). Global aspects cannot be time-lagged by distance.
3. **Regime F — history-free local aspects** (soil texture, minor relief, vegetation placement): not "behind in time," just **un-materialized**. Approach evaluates the lazy function of the *current* macro + fated noise. No replay; "far = raw prior" means "not yet materialized."
4. **Regime H — history-dependent local aspects** (a carved canyon, a dune field): genuinely at an earlier sim-time. Approach triggers a **deterministic catch-up** — fast-forward under the time-uniform global forcings; bounded, memoizable. Reconciling caught-up state with the current macro is the detail→abstract frontier ( #sketch-detail-abstract-reversion ) — this regime *is* where that problem loads.
5. **Regime E — equilibrium (attractor-seeking) aspects** (water surfaces, soil moisture, temperature profiles, climax vegetation): history-free *in principle* but the function is **implicit** — often it can only be relaxed to; where a direct solve exists, prefer it. Cheaper than replay, dearer than lazy evaluation; makes reversion nearly free.
6. **Consistency condition.** Temporally-lagged neighbours stay consistent because terrain is quasi-static on the lag timescale: the lag scales with distance such that $\text{lag} \times \text{local rate of change} \lt \text{coupling tolerance}$, and locality means nothing near couples to the un-pulled far side.
7. **Known limit (open).** Simulation LOD does not decompose cleanly by region for **non-local** processes — erosion depends on the entire upstream watershed, so "iterate this tile to level N" is entangled with its neighbours. Real and unsolved-in-general; the drainage-island dependency map is the working answer, not a closure of this limit.

## Epistemic Status

**Max attainable: exact** as architecture law if the regime taxonomy survives implementation contact. **Currently `robust-qualitative`:** design stance (DESIGN-REDUX §3, marked *our stance* at source; regime E added 2026-07-03 when the water system forced it). No Joseph DECISIONS row; the taxonomy is agent-articulated design reasoning Joseph has worked within, not a ratified schema. The regime *count* is not claimed closed.

Stage `draft`.

## Discussion

The regime an aspect belongs to dictates its machinery: G gets a time-uniform spine, F gets pure lazy evaluation, H gets memoized catch-up chains, E gets relax-or-solve. Mis-filing is expensive in a specific way — treating F as H buys pointless replay; treating H as F silently drops history; treating E as H replays a path when only the attractor matters. The taxonomy is also why "the world ages toward the pawn" is not a rendering trick but the shape of demand itself.

## Working Notes

- **Dual-home demote:** DESIGN-REDUX §3 gets a claim-home banner; the teaching texture (worked examples, caveat prose) stays as source.
- Sibling: three-scoped runtime / time-DAG remain OUTLINE §III gap (source in REDUX §11); this segment does not claim them.
- Open: whether regime E's "prefer direct solve" earns its own probe class (analytic-init acceptance test is the first instance).
- **Open tension (real, not fatal): leaf-only vs the gradient.** #form-face-flux-register FE(4) prices conservation at hanging nodes as leaf-only evolution (or explicit correction). Over ground where *both* tiers are materialized, leaf-only makes the coarse tier derived, not independently evolvable — which touches this segment's picture only where pulled and un-pulled regions overlap in history (regime H catch-up rejoining the aged spine). Where the far side is genuinely un-materialized (regime F), no double-evolve exists and the gradient is unpriced. Same tension named at #form-pull-query-composition FE(6)(e); resolution (summary-role leaf-only vs governance quantities) undecided.
