# The globe's million cubic kilometres of lakes were the prior, and the paint now says so

*2026-07-30. Frozen. World: `first-light`, seed `17425063241017297386` (`0xf1d242b21d8d89ea`), store `~/.cache/vivarium/globe-world`.*

**No pre-registration.** This one implements a law stated hours earlier rather than testing a hypothesis, and the honest note is that I expected it to make lakes *appear* and it made them disappear.

---

## Provenance (required fields)

| Field | Before | After |
| --- | --- | --- |
| **Kernel / memo `src=`** | `2dc664edab5e7f9d` | **`f0e5cb32e1b8b7de`** (this binary; keys only) |
| **World-dir git** | `bd9604cb7` | **`d417bc418`** — payloads **bit-identical** (0 new objects, 4786 new roots) |
| **Source repo** | `4d7f9d3` | `9c35ac9` |
| **Window** | whole globe, view L8 over the L13 beacon build | same |
| **Commands** | `bin/install` · `vivarium build` · `VIVARIUM_SHOT=… VIVARIUM_SHOT_DELAY=9 vivarium explore --paint depression` | same |

![[captures/ocean-connectivity/after-carved-view-globe.png]]

*(Path relative to `changelog/`. GitHub: [`after`](captures/ocean-connectivity/after-carved-view-globe.png). The honest "before" is entry `2026-07-29-04`'s capture, same view, same paint.)*

---

## What changed on screen

The whole-globe depression paint used to report **3761 wet cells, deepest 1387 m, 1.258 × 10⁶ km³** — dense violet speckle across every landmass. It now reports **0 cells, 0.000e0 km³** and the speckle is gone.

The old number was computed on the surface being *drawn*, and that surface was — by the HUD's own adjacent line — **100% uncarved prior**: *"400 fluvial tiles exist but NONE applies at view L8: an eroded region answers only at its own level (L13) or finer."* The figure and the caption contradicted each other in the same frame, and nothing flagged it.

## What changed underneath

`ErodedRegion::surface_m` returns `base + detail` — bilinear over the carve plus the prior's own detail increment at whatever level is asked for. That is right for **seeding** a finer carve, where the prior below the coarse run's Nyquist is declared law and the fine kernel then acts on it. It is wrong for **depicting**, because the detail belongs to no rung that ran: no key, no action, and a drainage structure the world does not have.

So there are now two accessors instead of one:

- **`surface_at_carved`** — the covering region's own stored cell, unblended and undecorated. Feeds the mesh and the explorer's seam ghost. Visibly stair-stepped at the carve's grain, which is the honest picture.
- **`prior_at_carve_level`** — the uncarved prior at the band the covering carve ran on. The change paint's baseline, because differencing a carve-level surface against a *finer* prior would report the missing detail band as erosion and swamp the signal that mode exists to show.

This is `#form-fidelity-ladder` FE(8) — *a view renders the physics and adds no terms* — and the law behind it, stated by Joseph the previous evening: **each band of detail must enter at a rung that runs**, because structure appearing where no rung ran has no cause and no key, and causality alone forbids it.

## The measurement that convicts it

Probe construction F (`examples/lake_surface_probe`), one L14 view over an L13 carve, 192² cells — same geography, same water, three readers:

| | wet cells | bodies | surface spread |
|---|---|---|---|
| **(b)** old reader, on the drawn surface | 4007 / 36864 | 282 | **0.000000 m** — level |
| **(c)** reader fixed only | 806 / 36864 | 26 | **~81–95 m** — tilted |
| **(d)** reader **and** surface fixed | 806 / 36864 | 26 | **0.000000 m** — level |

**(b) is level because bed and water came from the same phantom.** Internal consistency with a fabrication is free. (c) holds the right water over the wrong ground — fixing one consumer relocated the lie into the disagreement between consumers. (d) is (c)'s water over ground from the same rung.

The levelness test — the discriminator that convicted the ε-augmented raise the night before — **cannot tell (b) from the truth.** A consistency check bounds arithmetic; only the key bounds provenance.

## The zero needed its own honesty, and I got it wrong first

A zero here is the reader **declining**, not a world without basins. The prior's relief genuinely does contain closed basins — probe construction E measures **1165** of them at whole-face L8 — but they are an initial condition no kernel has processed, not an answer. My first version of this change reported the zero with the *water-census* explanation it had been borrowing, which is now the wrong cause. The depression line now distinguishes "no carve covers this view" from that, and reports the partial case with the fallback fraction.

## What is still wrong

- **Uncovered cells still fall back to the prior at their own level**, which reintroduces exactly the bands this change removes elsewhere. Choosing a coarser band, or refusing to draw at all, is a policy decision nobody has taken; it is named in `surface_at_carved`'s own doc rather than silently settled.
- **`erosion::column_at` still builds columns from the detailed surface** — the same defect one layer down, in the object Joseph identifies as where every system meets. Wants its own decision, not a quiet edit.
- **The low-edge halo window still slides instead of padding** (entry `2026-07-30-01`), untouched here.
