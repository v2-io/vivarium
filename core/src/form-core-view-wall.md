---
slug: form-core-view-wall
type: formulation
status: exact
stage: draft
depends: []
---

# The core / view wall

The world frame never depends on a renderer. Every consumer of the world is a peer that only queries. A view observes; it does not author world-evolution parameters.

## Formal Expression

1. **No reverse dependency.** The world frame (`vivarium-world` and, by the same law, any successor core) has no dependency on rendering, windowing, or view-local types.
2. **Query-only seam.** Views obtain world state only through the sanctioned query path over the store (or an equivalent pure query API). A view may hold camera state, meshes, and HUD state; it may not own authoritative world state.
3. **Peer views.** Human renderer, logozoetic interface, headless logger, ASCII instruments — siblings, never a tower in which one view is the only gateway to the world.
4. **Observe-only evolution.** A view does not expose knobs that choose how the world evolves (for example, how many erosion epochs to run). World-evolution parameters are authored by builder / law / manifest paths, not by the explorer. (`DECISIONS[core-view-wall-observe-only]`, `:by us`, decided.)

## Epistemic Status

**Max attainable: exact** as architecture law — falsified by a rendering dependency into the world crate, or by a view-owned evolution parameter that changes what the world *is*. The law is founding (DESIGN.md; `DECISIONS[core-view-wall-observe-only]`, `:by us`). Stage `draft`.

**Known incomplete surface (compliance debt, not a soften of the law):** `spikes/worldview` still reads view-local evolution parameters (macro/fine erosion epochs and related fill controls) and runs erosion on the explorer path — the same *species* of violation as a view-held epoch knob. That hybrid is a physics testbench debt, not a license to dissolve FE(4). Any path that claims ethereal / moratorium-clear observe-only must satisfy FE(4); the debt must close on that path before the claim is honest.

## Discussion

Headless calibration, human play, and logozoetic play-as-oneself are the same world under different peers. If the sim grows renderer tendrils, the other peers become second-class and *in vivia* citation becomes "whatever that UI did." Observe-only evolution is what keeps an ethereal explorer ethereal ( #scope-moratorium-endogenous-emergence is a consequence gloss, not a well-typing prior of this wall).

## Working Notes

- Worldview store-backed navigation is a separate gap from FE(4); both must close for a true ethereal explorer.
- Workspace `clippy` disallows for rendering crates in world packages remain desirable enforcement, not yet the segment's burden.
- Specimen of FE(4) violation and revert: `DECISIONS[core-view-wall-observe-only]` (history layer).
