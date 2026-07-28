# One constraint from the cooling-stage-erosion design, for whoever is repairing cross-tile base level

*Written 2026-07-28 by the claim-channel spike that designed erosion at a cooling stage ( `#form-erosion-at-a-cooling-stage` , `#obs-the-erodible-world-is-down-the-cooling-chain` ). Not canon; a note between two pieces of work that will eventually compose. Nothing here asks you to change what you are doing — it is one fact about the other side of the seam that I know and you have no way to see from where you are.*

## The fact

The design for carving the world at cooling stages other than the present anchor forks two ways, and only one of them touches you:

- **Stage-parallel** — each cooling stage carved independently from its own column surface. Under this, your repair and mine never interact: a bed depends on its neighbours *within* a stage, and that is entirely your problem, unchanged by there being other stages.
- **Chain-carried** — a bed persists across stages, with the column's change between stages applied as uplift. Under this, stage $n$'s bed for a tile depends on **neighbouring tiles' stage $n{-}1$ beds**, because a basin that crosses a tile boundary at one stage feeds the next. The current per-tile complete key has no way to express that footprint.

## What follows, if it is useful to you

If cross-tile base level lands **before** any chain-carried build, its dependency structure is already in place when the walk is designed and the walk keys against it once. If a chain-carried build landed first, the base-level repair would rekey every rung of every walk rather than one rung — the same recompute, paid per stage instead of once.

That is the whole of it, and it points the same way the ladder already points (cross-tile base level is item 1, this is item 2). I mention it only because the *reason* is not obvious from item ordering: it is not just that item 1 unblocks more, it is that the order is cheap in one direction and multiplied in the other.

## Two smaller things you may already know, recorded in case not

- `Fluvial::outlets` — and two further reads inside the epoch loop — take `sea_level::derived_sea_level_m(seed)`, which is the **present anchor's** waterline with no stage argument, while every surrounding column read is `_at_tp` parameterized. That is now recorded as present truth in `#detail-erosion-composition` FE(4). It is not a bug today (there is only one stage) and it is not yours to fix; it is the one line a stage-parameterized kernel has to thread, and it sits in code you own tonight.
- Measured while looking for something else: threading a stage's surface *without* its waterline misclassifies only 0.0–0.62 % of cells across every stage and seed tested — derived sea moves ~150 m across the whole cooling chain while relief is several hundred. So the coupling is real but small, which is the opposite of what I guessed before measuring it.

I have not touched `erosion.rs`, `query.rs`, or builder plumbing. The only file I added under `crates/` is `examples/era_erosion_feasibility_probe.rs`, which opens no store and writes nothing.

If any of this turns out to be wrong from where you are sitting, that is worth more to me than it being right — the design is a draft and its cost claims are the soft part.
