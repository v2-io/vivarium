# The bed can hold water, and nothing yet does

*2026-07-28. Frozen. Seed 17425063241017297386 (`first-light`), cohort `src=12ea45e5d7a1c48e`, against `src=b4b900da6119dabe` (the tree 100 minutes earlier).*

**The pre-registered entry, from `changelog/README.md`:** *"the fill repair's entry will show closed depressions persisting in the depression map where the previous cohort shows none."*

**What actually happened:** the persistence is real and measured, and **no view in this repository can show it yet.** So this is the second diagnosis the README names — built, currently invisible — and it is an instrument requisition as much as a changelog entry.

---

## What changed in the world

Every epoch of fluvial erosion used to fill each closed basin to its spill point and keep the raise. The bed handed to the store was therefore depression-free by construction: a planet that could not hold a lake anywhere, for a reason that had nothing to do with water. The fill is now a **routing surface** — flow still has to cross a basin and leave by its spill point to be routed at all — and the epoch puts the raise back before it ends.

## The capture that shows it

`cargo run --release --example base_level_probe` (store-free; topography, uplift and rain are pure functions of seed and cell, so this reproduces from the header alone). Whole cube face 0 at L9, coast-only outlets, depression cells at fill depth $\gt 1\,\mathrm{m}$:

```
   the prior holds 671 closed-depression cells, capacity 2.182e13 m3, deepest 688 m.
   epoch    dep>1m    dep%    deepest   capacity m3
       1       613    2.42        673      1.919e13
       2       571    2.25        657      1.784e13
       4       465    1.83        623      1.503e13
       8       357    1.40        612      1.137e13
      40       134    0.51        635            —
     300       228    0.47        212            —
```

The same instrument on the same prior under the previous composition read **31 cells at epoch 1 and 2 cells at epoch 40**. The prior's basins used to die in the first epoch; now they erode away over hundreds, and roughly half a percent of the land is still in one after a full response time.

## The globe, honestly

```
reached     initial-topography-only 0 · eroded 16 · watered 0   (build-state from the store census)
```
*(`vivarium info --width 100 --no-color`, and identically at `--level 9`.)*

**Watered: 0.** Unchanged, and expected. Two things stand between a bed with basins and a lake anyone can see, both measured and neither touched by this work:

- **The tile contract.** Every builder tile is carved as if surrounded by sea, so each grades away its own basins at its perimeter. A per-tile read of the rebuilt store finds 8 tiles holding 23 depression cells (deepest 367 m) against 3 tiles and 12 cells before — real movement, and small, for that reason. (Ladder item 1.)
- **The water timestep.** `water-tile` settles 40 s of world time at any level. It would not fill a 200 m basin however good the bed underneath it is. (Ladder item 4.)

So the ball moved one row and the remaining distance is named, which is the weaker guarantee the README asks for rather than the exciting one it hoped for.

## What no instrument can draw

There is no depression or standing-water-capacity paint mode. `vivarium explore --paint water` draws the water nomos's depth, which is zero; the quantity that changed is **the bed's capacity to hold water**, which exists only inside a probe's printout. The three captures this entry could use — globe ASCII, interior census, status — are all build-state instruments, and this was not a build-state change.

**Requisition:** a `--paint depression` mode reading `DrainageSurface::fill_depth` over the assembled surface would have made this entry a picture. It is the same reader the probes already use.

## Also true, and visible in the census

```
erosion-tile               M/-   46302   34 (epochs)
```
The erosion chain's interior survived the rekey: every store went stale when the law changed, the world was rebuilt in ~3 minutes, and the settle history came back with the same shape under new keys. The rebuild cost nothing measurable — the L9 erosion sweep runs 384 tiles in 12.0 s against 12.4 s before.

## Postscript 2026-07-28: the requisition is filled

The instrument this entry asked for exists. `vivarium explore --paint depression` (key **6**) draws standing-water **capacity** — depth to the spill point of every closed basin in the drawn surface — through `Fluvial::drainage_surface`, the same reader `base_level_probe` and the unit tests use, so the picture and the numbers cannot drift apart. The HUD reports cells, deepest, and capacity in km³ beside the count of cells actually holding standing water, because the gap between those two is the honest subject.

Three things the mode had to decide, each declared on screen rather than in a comment:

- **The palette is violet→white, deliberately not the water mode's cyan.** A filled basin has the shape, position and colour of a lake and the eye will take it for one. A unit test holds the two ramps apart in hue, because a caption is not what the eye is reading.
- **The reader treats each drawn unit's rim as a no-flux wall,** set explicitly rather than inferred. The inferred contract for any window short of a whole face makes its own rim an outlet and reports ~0 — the paint would be black for a reason about the reader rather than the world. A test convicts this: the same trench reads 0 cells under the inferred contract and 96 under the declared wall.
- **On a multi-tile surface the number mixes inherited basins with tile-seam pits,** and nothing in the picture separates them. The HUD says so every frame.

This does not change the entry above: capacity is still not water, and `watered 0` still stands. What changed is that the quantity is now visible instead of living in a probe's printout.

*Incident worth recording, because it is a standing open question meeting a real cost:* documenting the new mode in the CLI help text moved `SRC_HASH` and staled every store, because the source digest covers `src/bin/`. A help-string edit cost a full world rebuild (~50 s of sweeps). That is the tradeoff `#form-complete-content-addressed-key`'s open question describes, priced.

## Postscript space

*(Further errata get a dated postscript here, never an edit.)*
