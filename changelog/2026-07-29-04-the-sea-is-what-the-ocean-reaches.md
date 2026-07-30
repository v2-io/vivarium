# The sea is what the ocean reaches — and a million cubic kilometres of lakes were hiding behind a height test

*2026-07-29. Frozen. World: `first-light`, seed `17425063241017297386` (`0xf1d242b21d8d89ea`), store `~/.cache/vivarium/globe-world`.*

**Pre-registration** (`msc/lake-connectivity-2026-07-29-prereg.md`, written before the rebuild and before any rebuilt tile was looked at): full recompute with 0 hits; **1–10% of erosion tiles change**; every water tile changes; lakes appear where the paint was black. Scored below — **two hit, one missed by a factor of three, and the miss is the most interesting number in the entry.**

---

## Provenance (required fields)

| Field | Before (src-pin-fix cohort) | After (ocean connectivity) |
| --- | --- | --- |
| **Kernel / memo `src=`** | `defc3c44bfc0b00a` | **`f0a6800a2f0d920e`** (this binary) |
| **World-dir git** | `a50d198ed` (pre-rebuild, committed clean) | **`e0300f43c`** post-rebuild |
| **Source repo** | `1badba9` | `6e4e919` (law) · `5e9fa69` (core) |
| **Window** | whole globe, view L8 over the L13 beacon build | same |
| **Commands** | `bin/install` · `vivarium build` · `VIVARIUM_SHOT=… VIVARIUM_SHOT_DELAY=8 vivarium explore --paint depression` | same |

![[captures/ocean-connectivity/after-globe-depression-l8.png]]

*(Path relative to `changelog/`. GitHub: [`after`](captures/ocean-connectivity/after-globe-depression-l8.png). Shot taken with the pre-fix HUD text, so its caption still reads "CAPACITY … THIS IS NOT WATER" — the corrected caption shipped minutes later in the same session; the *field* in the picture is the new one.)*

---

## What changed

**`Fluvial::outlets` classified ocean as `h <= sea`.** Being under the datum made a cell *the sea*, and `fill_depressions` seeds its Priority-Flood heap from exactly that set — so every basin whose floor dipped below the waterline was told it was already a drain. It held nothing. Not at any settle length, not under any halo depth, not with or without a water balance, because the classification sits **upstream of all of them**.

Now: below the datum makes a cell **submerged**; the ocean has to reach it to make it **sea**. A submerged cell is ocean iff it reaches the domain boundary through submerged cells — eight-connected, matching the flow router's own neighbourhood, boolean reachability, no new parameter. Past that boundary lies the rest of a planet that is ~95% submerged.

**What was standing behind that height test, measured per whole cube face at L8:**

| | bodies | cells | volume | deepest | all level |
| --- | --- | --- | --- | --- | --- |
| **planet** | **1165** | 4541 | **976 865 km³** | **1271 m** | **yes** |

All of it previously ocean-by-threshold, and empty. Earth's lakes hold ~180 000 km³; this is a wetter planet read at 78 km cells, so the order is right and the count is a **floor** — basins smaller than a cell cannot appear at all.

Every body's surface is **level to the bit**. On an analytic construction (a 200 m cone in a plane falling 5 m/cell) the field returns depth **100.000 m** at surface spread **0.000000 m**.

**Three retired-datum sites, same evening.** `water_tile` computed the derived waterline for its rim boundary condition and then initialised the sim at the retired `gen::SEA_LEVEL_M`: **1106.2646 m short on every interior cell**, rim pinned correctly, and a 40 s bounded fill that cannot relax a kilometre. The Water paint reads that field, so ocean depth was wrong by up to a kilometre everywhere but tile rims. The ASCII globe drew its coastline at 4000 m while the explorer used the derived level — two instruments that share an honesty block byte-for-byte disagreeing about where the sea is. And the erosion module header still named the retired constant as its outlet set.

**And the reader's physical field.** `fill_depressions` returns the routing surface *and* the standing-water depth; three reader paths dropped the second, so every view and table ran on the ε-augmented raise. On a flat shelf with no depression anywhere that raise reports **4418 of 9216 cells wet and 0.0605 km³** of water that cannot exist; the standing-water field reports zero.

## Scoring the pre-registration

| | prediction | result |
| --- | --- | --- |
| P1 | full recompute, 0 hits | **hit** |
| P2 | 1–10% of erosion tiles change | **MISS — 30.0%** (1065/3552 at L9; beacon 452/480 = 94%) |
| P3 | every water tile changes | **hit, exactly** — 384/384 |
| P4 | lakes where the paint was black | hit |

`initial-topography`, `climate`, `mantle-thermal` and `hydrosphere` came back bit-identical, as they must: none of them consumes the ocean set.

**P2's miss is not yet explained, and both obvious repairs are refuted.** Changed-tile fraction by face runs XPos 38.9% · XNeg 32.8% · ZPos 23.4% · YPos 10.9% · ZNeg 10.5% · YNeg 3.1% — **inversely** ordered against both land fraction and body count. The face with the most land *and* the most lakes changed least. So neither "more land, more coastline" nor "more basins, more change" survives, and the entry records the gap rather than a third guess.

## What is honest about the picture, and what is not

The violet speckle across the landmass is **mostly not basins.** Measured on one emerged L13 window, same geography and reader: the carved surface holds **210 wet cells in 26 bodies**; the *uncarved prior* holds **1780 in 59**. Fluvial carving destroys 88% of the prior's closed basins — that is drainage integration doing its work — and a view finer than the carve draws the coarse carve with the fine prior's detail re-added, so its standing-water field is dominated by **undrained prior dimples**. Joseph read this off the screen before it was measured: *"a lot of these have ostensible outlets to the ocean."*

That is an instrument defect, not a lake defect, and it produced the evening's other result: a view renders the physics and adds no terms, because **each band of detail must enter at a rung that runs** — structure introduced finer than any executed rung participates in no action and carries no key, and causality alone forbids it. `#form-fidelity-ladder` FE(7)–(11).

## Still open

- P2's geography.
- The explorer still draws mixed-tier surfaces and still computes standing water on them. Either the reader refuses above the carve level or the figure carries its tier composition.
- Four consumers still classify ocean by threshold independently — both paints, the terminal globe reader, the water kernel's initial fill. The mask wants to be one shared world object; crustal columns will want it too.
- A coastless walled domain still seeds Priority-Flood from its own lowest cell, so the deepest basin there can never hold water.
- The wet limit carries no water balance, so an endorheic basin under a dry climate stands lower than drawn and nothing yet says by how much.
