# Ocean is one shared mask — below the datum is not automatically sea

*2026-07-31. Frozen.*

Connectivity classification left the router and became the single pure function every consumer that means "is this the sea?" must call. Landlocked floors under the waterline stop painting and filling as open ocean.

*World: `first-light`, seed `17425063241017297386`, store `/Users/josephwecker-v2/.cache/vivarium/globe-world`. Declared order `terrestris`, target phase `4`.*

## Provenance (required fields)

| Field | Before | After |
| --- | --- | --- |
| **Kernel / memo `src=`** | `4fdc95afbe11c785` | **`f0a56097577e8971`** (4786 roots at this cohort) |
| **World-dir git** | `f1b321a14` | **`6ca02be83`** |
| **Source repo** | `acc6eea` | this entry's commit |
| **Window** | whole globe L9 rebuild + L13 beacon | same |
| **Commands** | `bin/install vivarium` · `vivarium build` · `vivarium info --width 100 --no-color` | same |

*(Provenance from `bin/provenance` after rebuild; digests not typed by hand.)*

## What changed

**One pure function:** `sea_level::ocean_mask(h, nx, sea)` — submerged cells that reach the domain boundary through submerged cells, eight-connected ( #form-ocean-is-connectivity-not-elevation ). Dual-arm unit test: an enclosed crater below the datum is *not* ocean; rim-connected sea *is*.

**Consumers rewired to that function** (no second independent threshold):

| Consumer | Was | Now |
| --- | --- | --- |
| Fluvial router outlets | private flood-fill (already correct) | calls the shared function |
| Water kernel initial fill | every cell with `bed ≤ sea` full to sea | only **ocean** cells filled |
| Water rim hold | every edge cell `bed ≤ sea` | only ocean edge cells |
| Terminal `vivarium info` globe | `elev < sea` → ocean glyph | connectivity on eroded tiles |
| Explorer surface / water / provenance paints | blue / dark for below-datum | blue only for **is_ocean** |
| Explorer mesh freeboard | flattened everything below sea to the waterline sphere | open ocean flat; landlocked floors keep true freeboard (visible pits) |
| `column_at` / classified column assembly | threshold water depth | ocean water only when the covering region's mask says so |

**Still residual (named, not sold as closed):** store-memoized planet/face ocean article for plate work (today the mask is recomputed from heights); FE(4) window bias when the domain is smaller than a real enclosed sea; legacy `column_from_surface_at_sea` still thresholds for pour-style tests.

## What you can see

- **Surface paint:** landlocked basins under the waterline read as *low land*, not open-ocean blue. Caption: *"blue = OCEAN (connectivity, not every cell below the datum)"*.
- **Mesh:** endorheic floors can sit as geometric pits rather than false sea surface.
- **Depression paint (6):** unchanged law — wet-limit lakes on the carved bed — still the place to read standing water.
- **ASCII globe** (`vivarium info`): [after-info.txt](captures/ocean-mask/after-info.txt) — post-rebuild L13 overview; beacon tiles eroded; coast is connectivity on built tiles.

```
![[captures/ocean-mask/after-info.txt]]
```

## What this is not

- Not a longer water settle (still not the lake path).
- Not the sill/flux seam object (next physics open on the same-level family).
- Not a store-keyed planet ocean nomos yet.

## Epistemic note

The rebuild was required because the library edit moves `SRC_HASH`; explorers only load the current source cohort. View-side classification is pure of the stored bed, so the *meaning* of the paint is independent of water-tile recompute — the rebuild also re-keys water under the new fill rule.
