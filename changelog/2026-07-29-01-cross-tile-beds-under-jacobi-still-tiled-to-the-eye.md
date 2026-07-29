# Cross-tile beds under Jacobi — different physics, still tiled to the eye

*2026-07-29. Frozen. World: `first-light`, seed `17425063241017297386` (`0xf1d242b21d8d89ea`), store `~/.cache/vivarium/globe-world`.*

**Pre-registration (entry 01, 2026-07-28):** *halo/seam design — claim-channel first; entry when the first cross-tile carve lands.* **This is that entry.**

---

## Provenance (required fields)

| Field | Before (edge-sink cohort) | After (Jacobi production) |
| --- | --- | --- |
| **Kernel / memo `src=`** | `12ea45e5…` (not this binary at capture) | `b129b27ca9b57a95` (this binary) |
| **World-dir git** | *not snapshotted pre-rebuild* (procedure gap — see below) | **`28cb20d`** baseline post-rebuild; HEAD then `d985fa2b6` (README only) |
| **Window** | face 1, L13, beacon 4×4 @ `(640, 5376)`, epoch **300** | same |
| **Commands** | `VIVARIUM_EROSION=29 VIVARIUM_SHOT_DELAY=6 vivarium explore --paint surface` | same after `vivarium build` (Jacobi face + beacon) |

Captures (self-labelling explorer shots):

![[captures/halo-before-after/before-l13-beacon-e300.png]]

![[captures/halo-before-after/after-l13-beacon-e300.png]]

*(Paths relative to `changelog/`. GitHub: [`before`](captures/halo-before-after/before-l13-beacon-e300.png) · [`after`](captures/halo-before-after/after-l13-beacon-e300.png).)*

---

## What changed in the world

Builder erosion no longer carves each partial tile as an isolated edge-sink island by default. **Same-level Jacobi halo exchange** ships: `HaloSchedule` in the complete key (`edge=halo|d|sigma|rho`), face-at-a-time region carve on the global L9 sweep, beacon under the same path, loaders prefer halo memos over plain.

On the L13 beacon (16 tiles, epoch 300), the bed **is not the old bed**:

| Metric | Old (`12ea45e5`) | New (`b129b27c` + halo) |
| --- | --- | --- |
| mean \|\Δh\| vs other | — | **~130 m** (almost every cell differs) |
| max \|\Δh\| | — | **~961 m** |
| mean elevation | ~6093 m | ~6124 m (**+31 m**) |
| seam step ratio (seam / interior) | **0.61** (quiet perimeter crime) | **1.04** (matches exchange-probe “on reference” neighborhood) |

Eye: same landmass footprint, **different physics** — highs/lows rebalanced; new central ridge/divot chain; residual **horizontal structure** and, under change paint, an obvious **4×4 tile lattice**.

---

## What the eye still sees (honest residual)

Jacobi repaired the **structural** tile-seam statistic; it did **not** make fine-grain seams invisible. Change paint (Joseph, same session) shows signed Δh vs uncarved prior with grid-locked blue/orange structure across the beacon. **Flux half** and **spill scalar** of `#form-same-level-halo-exchange` remain open; chart-edge clamp is a separate family (`#form-seam-families`, `#obs-chart-edge-halo-clamps-to-the-face`).

So the ball moved: cross-tile production path **enacted**, beds **changed**, and the remaining distance is **named** — tiled look at L13, not “seams fixed, move on.”

---

## Procedure gap recorded (so it does not recur)

Pre-rebuild store state was **not** committed to world git before `vivarium build`. The *before* image is valid as an **in-store non-current `src=` cohort** capture, not as `git checkout` of a pre-rebuild rev. **Post-rebuild** is `28cb20d`. Landing procedure now requires world-store commits under the same bar as the entry (`#ops-changelog-is-the-acceptance-check` FE(5)).

---

## Rebuild log (abbreviated)

```
erosion: Jacobi halo d=16 σ=5  — 384 L9 tiles in 28.1s
water: 384 tiles in ~534s
beacon: f1 L13 (640,5376) 4×4, 300 epochs, Jacobi d=16 σ=10 — 16 tiles in 11.0s
build complete exit=0
```

Full log: `captures/halo-before-after/first-light-build.log`.
