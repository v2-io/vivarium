# The world did not change — and now that is provable

*2026-07-29. Frozen. World: `first-light`, seed `17425063241017297386` (`0xf1d242b21d8d89ea`), store `~/.cache/vivarium/globe-world`.*

**Pre-registration (DECISIONS `compute-names-its-article-water-bed-and-exchange-region-into-the-key`, same day, before the rebuild):** *when this lands and first-light rebuilds, the entry can show the world did NOT change — before/after captures indistinguishable, only the keys became complete.* **This is that entry, and the prediction held exactly.**

---

## Provenance (required fields)

| Field | Before (morning halo cohort) | After (key-truthification) |
| --- | --- | --- |
| **Kernel / memo `src=`** | `b129b27ca9b57a95` | `defc3c44bfc0b00a` (this binary) |
| **World-dir git** | `d985fa2b6` (pre-rebuild, committed clean) | **`e67fbaeb9`** post-rebuild |
| **Window** | face 1, L13, beacon 4×4 @ `(640, 5376)`, epoch 300 | same |
| **Commands** | `vivarium build` · `VIVARIUM_EROSION=29 VIVARIUM_SHOT=… VIVARIUM_SHOT_DELAY=6 vivarium explore --paint surface` | same |

Capture (self-labelling explorer shot, new cohort):

![[captures/key-truthification/after-l13-beacon-e300.png]]

*(Path relative to `changelog/`. GitHub: [`after`](captures/key-truthification/after-l13-beacon-e300.png). The morning cohort's shot is entry `2026-07-29-01`'s `after` — the honest "before" here.)*

---

## What changed — in the keys, not the world

The morning's halo adoption carried two incomplete keys and one law violation, found in review and repaired the same day (`#form-depend-by-key-never-latest` FE(4)(b); DECISIONS entry above):

- **The exchange region is now identity.** A tile carved inside a 1×1 block and the same tile inside a whole-face sweep hold different interiors — they now hold different keys (`edge=halo|d|sigma|rho|roi|roj|rti|rtj`). Convicted by `halo_key_carries_the_exchange_region`.
- **Water names its bed.** `water_tile` takes a `BedArticle` folded into its key as a `bed=` token and computes exactly the named article on a miss. Before, water settled on *whichever bed cohort was in the store at compute time* under a key naming neither — one key, two possible worlds by demand order. Convicted by `water_bytes_are_a_function_of_the_key_not_build_order` (both demand orders, separate stores, bit-identical depths).
- **`erosion_tile` is pure again**; the halo-over-plain preference retreated to view reads only, with a deterministic tie-break.

## The null diff, adjudicated

**Store level:** across the two cohorts, 3936 of 3936 shared-identity payloads (3552 erosion tiles incl. settle rungs + beacon, 384 water tiles) are **bit-identical** — same content-addressed object hashes; zero differing, zero orphans on either side.

**Eye level:** the before/after L13 beacon captures differ in **zero terrain pixels** — a pixel diff isolates every differing pixel to the HUD text overlay (root counts and line layout). The picture of the planet is the same picture.

That is what a key-completeness repair should look like: bytes untouched, addressability repaired. The same evidence would have *convicted* the morning build had the physics accidentally moved.

## Incidental: the water phase got 12× faster

`water: swept 384 tiles in 42.9s` vs the morning's `534.1s`. The morning path scanned all ~94k store roots once per tile (the store-preference read inside a compute loop); pulling the bed by exact key removed the scan. The truthful path and the fast path turned out to be the same path.

## Rebuild log (abbreviated)

```
initial-topography: 384 tiles in 659ms (all computed)
erosion: Jacobi halo d=16 σ=5 ρ=0 — 384 L9 tiles in 27.2s
water: 384 tiles in 42.9s
cooling stages: 81 in 27.7s
beacon: f1 L13 (640,5376) 4×4, 300 epochs, Jacobi d=16 σ=10 — 16 tiles in 10.8s
build complete exit=0
```

Full log: `captures/key-truthification/first-light-build.log`. Explorer log: `captures/key-truthification/after-explore.log`.
