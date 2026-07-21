---
slug: form-core-view-wall
type: formulation
status: exact
stage: draft
depends:
  - scope-moratorium-endogenous-emergence
---

# The core / view wall

The world frame never depends on a renderer. Every consumer of the world is a peer that only queries. A view observes; it does not author world-evolution parameters.

## Formal Expression

1. **No reverse dependency.** The world frame (`vivarium-world` and, by the same law, any successor core) has no dependency on rendering, windowing, or view-local types.
2. **Query-only seam.** Views obtain world state only through the sanctioned query path over the store (or an equivalent pure query API). A view may hold camera state, meshes, and HUD state; it may not own authoritative world state.
3. **Peer views.** Human renderer, logozoetic interface, headless logger, ASCII instruments — siblings, never a tower in which one view is the only gateway to the world.
4. **Observe-only evolution.** A view does not expose knobs that choose how the world evolves (for example, how many erosion epochs to run). World-evolution parameters are authored by builder / law / manifest paths, not by the explorer. (`DECISIONS[core-view-wall-observe-only]`, `:by us`, decided.)

## Epistemic Status

**Max attainable: exact** as architecture — falsified by a rendering dependency into the world crate, or by a view-owned evolution parameter that changes what the world *is*. Founding commitment in DESIGN.md; re-asserted after a live violation (view-held erosion-epoch knob, reverted). Stage `draft`. The ethereal explorer milestone is moratorium-clear partly *because* the observer has no causal access; breaking (4) would re-open that.

## Discussion

Headless calibration, human play, and logozoetic play-as-oneself are the same world under different peers. If the sim grows renderer tendrils, the other peers become second-class and *in vivia* citation becomes "whatever that UI did."

## Working Notes

- `spikes/worldview` still behaves partly as a physics testbench (not fully store-backed navigation). That is a **view gap**, not a license to dissolve the wall.
- Workspace `clippy` disallows for rendering crates in world packages remain desirable enforcement, not yet the segment's burden.
