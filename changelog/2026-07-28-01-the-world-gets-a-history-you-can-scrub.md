# 2026-07-28 — The world gets a history you can scrub

*World: `first-light`, seed 17425063241017297386, `~/.cache/vivarium/globe-world`. Newest cohort at capture time: `src=12ea45e5d7a1c48e`. Frozen entry; errata by postscript only.*

## What you can do today that you couldn't yesterday

- **Scrub erosion in world-time.** `vivarium explore`, press `E`: every tile on screen steps through the same epoch together (`J`/`L` step, `K` play). Yesterday, erosion had exactly one addressable moment; today it has a settle history — 8 stages per tile globally, 30 on the beacon patch.
- **Zoom past the globe into a region window.** Levels now reach **L14** (was L9): zooming in replaces the six-face globe with a 384×384 window into one face. `B` flies to the selected chain's extent; `G` cycles chains. At L13 over the beacon, epoch 10 is scattered blobs and epoch 300 is organized ridge-and-valley — the first picture in this tree at a scale where fluvial form is a shape.
- **See change instead of color.** Paint mode 5 draws the signed change field — necessary because 40 L9 epochs move ~25 m against kilometres of relief, invisible in hypsometric color. Under today's new driver you will see the ocean floor *subsiding* while land rises; yesterday's driver raised the entire seafloor every epoch, and the paint is what caught it.
- **Aim the builder at a place.** The manifest carries the project's first standing beacon:

  ```udon
  beacon = "face=1 level=13 oi=640 oj=5376 tiles=4 epochs=300 stride=10"
  ```

  — a ~310 km square at ~1.2 km/cell on the largest landmass, 300 epochs (the first *derived* epoch count: one measured response time), 30 scrubbable stages per tile.

## The interior, measured (capture: `vivarium watch --frames 1`)

```
nomos                B dcl/drv   roots   interior (distinct time-index)
climate                    L/L    6254   — no time-index in key
erosion-tile               M/-   46302   34 (epochs)
hydrosphere                L/L      17   — no time-index in key
initial-topography         -/-    6254   — no time-index in key
mantle-thermal             L/L     897   81 (tp_bits)
uplift-tile                L/-    6254   — no time-index in key
water-tile                 M/-    5790   1 (steps) — endpoint only, no interior
```

Yesterday `erosion-tile` read **1** — endpoint only, no interior. Water is now the tree's only relaxation kernel with two ends and nothing between.

## What the world honestly is at this date (so nobody reads the scrub as more)

- The landmasses are nucleation-growth cratons — the "circles" are real objects, young ones. Mountain belts don't exist yet at the present anchor and *shouldn't*: sutures arrive down the cooling chain as cratons grow into collision (scrub the deep-time timeline to colder stages to watch continents assemble).
- **There are no lakes and no rivers of standing water anywhere.** Measured today to be a *bed* property: the erosion loop keeps its depression-fill in the stored surface, so every bed is depression-free by construction. This is queued work (see pre-registrations), not a rendering gap.
- The beacon's 30-stage scrub shows incision *deepening*, not drainage *reorganizing* — under the current tile policy at that grain, the network is set in the first ten epochs. Honest caption: watch valleys cut, not rivers wander.

## Pre-registered expected entries (declared before their work lands)

1. **Fill repair** (in flight): its entry will show a depression map with closed basins *persisting* across a settle history where this cohort's map shows essentially none — the first beds that could hold a lake.
2. **Halo/seam design** (in flight): claim-channel work — likely **no entry**, and that's the system working; its entry comes when the first cross-tile carve lands.
3. **Era carve** (queued): its entry will show a cold-stage world with roughly triple the land and, at the coldest stages, the first suture belts — side-by-side with the present anchor from the same seed.

## Instrument gaps this entry exposes (the requisition clause)

The whole-globe ASCII (`vivarium info`) can't show any of today's change — its glyphs are build-state, not relief, and at ~600 km/character the carving is sub-glyph. The 3D explorer is currently the *only* surface on which today's work is visible, and no capture path exists from it short of the sighting key. A CLI-level region render (ASCII relief/discharge shot of a named window at a named stage) would make future entries' before/after pairs one command each.

## Postscript 2026-07-29: pre-registrations landed

1. **Fill repair** — entry `2026-07-28-02` (and adjudication in `2026-07-28-03`).
2. **Halo/seam** — production Jacobi path landed; user-visible cross-tile entry is `2026-07-29-01-cross-tile-beds-under-jacobi-still-tiled-to-the-eye` (residual 4×4 lattice + flux/spill open). Explorer capture path also exists (`VIVARIUM_SHOT` / delay).
3. **Era carve** — still queued (hotspots rank 3).
