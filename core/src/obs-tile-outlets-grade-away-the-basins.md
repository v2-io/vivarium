---
slug: obs-tile-outlets-grade-away-the-basins
type: observation
status: exact
stage: draft
depends:
  - detail-erosion-composition
  - obs-water-fill-never-settles
  - obs-cube-locked-kernel-bias
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - form-seam-flux-exchange
  - norm-no-depiction-without-referent
---

# Every builder tile is carved as if surrounded by sea, so it holds no lake and no basin larger than itself

The partial-tile outlet policy makes every edge cell a sink, so each tile grades itself to its own perimeter. **Measured: across the whole planet at L9, 12 of 58 013 subaerial cells sit in a closed depression — 0.02%.** Inland standing water has nowhere to stand, and no drainage basin can exceed one tile.

## Formal Expression

1. **The policy.** `Fluvial::outlets` marks a cell an outlet if it is at or below derived sea **or** — for any tile that is not a whole cube face — if it lies on the tile edge. Every builder tile at L9 (64² of a 512² face) and every beacon tile at L13 is such a partial tile. Priority-Flood then fills to those outlets each epoch, and stream-power incises toward them. The tile is therefore carved against a base level that is an artefact of the tiling, everywhere within $\le 32$ cells of any point.

2. **Measured: the eroded bed holds essentially no closed depression.** Recomputing the depression fill on the *stored* L9 surfaces (`examples/discharge_probe` Part 7, live world `first-light`, 384 tiles): 96 tiles carry land, 58 013 subaerial cells in total, and **3 tiles** contain **12 cells** whose fill depth exceeds 1 m. Depression capacity is a per-cell quantity here — $\text{filled }h - \text{stored }h$ — not an aggregate that could hide a distribution.

3. **This is the explanation for zero inland standing water, and it is a bed property, not a kernel failure.** `water-tile` settles a conserved shallow-water depth on the `erosion-tile` bed. Standing water above the datum requires a closed depression to occupy; the bed has 0.02% of one. So the measured absence of inland standing water at L9 is not the relaxation fill failing to pond — **there is nothing to pond in.** This compounds with, and does not replace, #obs-water-fill-never-settles : that observation shows the fill buys 40 s of world time at any level, so even a bed *with* depressions would not have filled them. Two independent sufficient causes; fixing either alone changes nothing.

4. **Measured: no basin can exceed its tile, and the cost is a factor of four.** On the L13 beacon patch (f1, origin (640, 5376), 4×4 tiles of 64², 30 stored stages) the largest tile-local trunk drains $3.58\times10^{9}\,\mathrm{m^2}$. Re-routing the **assembled** 256² patch — the same stored heights, one field, tile-interior edges no longer sinks — gives $1.38\times10^{10}\,\mathrm{m^2}$, a **3.85×** larger trunk. Per tile, the largest basin collects 18–71% of the tile's runoff and 1–5 basins cover half of it; over the assembled patch the largest collects 17.5% and 7 basins cover half. The network is well integrated *at the scale it is permitted to be*, and that scale is 78 km.

5. **Measured: assembling the tiles manufactures depressions at the seams.** The same assembled patch that had **zero** depression cells tile-locally has **8 532 of 65 536** (13.0%) once the artificial edge sinks are removed, deepest 411 m, capacity $8.7\times10^{11}\,\mathrm{m^3}$. Each tile was graded to its own perimeter; adjacent tiles graded to different perimeter heights; the mismatch is a closed basin. So the depression statistic on an assembled surface is **not evidence of geography** — it is at least partly a seam artefact of the tiling, and the two cannot be separated by this measurement. Both numbers are real and neither alone is the world. This is #form-seam-flux-exchange 's argument arriving in the hydrology: tiles that exchange *states* at a seam rather than *fluxes* disagree about where downhill is.

6. **Measured: the settle history does not integrate the network.** Across all 30 stored stages of one beacon tile (epochs 10 → 300, one full a-priori response time), largest-basin share oscillates between 32.6% and 45.8% with no trend, channel-cell counts hold at ≈1000 / ≈185 / ≈17 at the three thresholds, and discharge-weighted MFD out-degree stays at 1.96–2.09. **The network is set within the first ten epochs and only jitters afterwards.** Under FE(4) this is expected rather than surprising — base level is fixed 32 cells away in every direction, so there is no room for capture or basin growth — but it means the beacon's 300 epochs buy incision *depth*, not network *evolution*, and a scrub of the stage chain should not be described as watching drainage organize.

7. **The flow field is not a channel thread, measured on live terrain.** MFD at $p=1.0$ spreads to every downhill neighbour ( #obs-cube-locked-kernel-bias FE(1)); discharge-weighted out-degree is **1.96–2.30** across all 16 beacon tiles and 2.00 on the assembled patch — flow leaves a typical cell in two directions, not one. Peak concentration is nonetheless comparable to a single-receiver tree: $\max D8 / \max \mathrm{MFD}$ runs **0.56–1.01**, so the fan spreads *along the way* and re-converges at the trunk rather than smearing the trunk itself. A view painting MFD as a one-cell-wide river is therefore drawing a two-cell fan thin, which #norm-no-depiction-without-referent forbids; a view painting D8 instead is drawing a tree the kernel does not erode with.

8. **The straight runs are a fill artefact, not a router artefact, and only on assembled surfaces.** Longest run of identical D8 flow direction within a tile: **10–20 links**, median 2, with **0%** of long-run cells inside filled ground. On the assembled patch: max 56, 1 740 cells on runs $\ge 8$, and **80% of them inside filled ground** — the Priority-Flood $\varepsilon$-gradient orienting flow across the flats that FE(5)'s seam basins create. The residual table in #detail-erosion-composition names the $\varepsilon$-fill as a *mass* mint; it is also a **directional** one, and that is new here.

## Epistemic Status

**Max attainable: exact** for the quoted measurements under `examples/discharge_probe` — deterministic, read-only, over the stored surfaces of one world (`first-light`, seed 17425063241017297386). Every clause is independently falsifiable: a bed with closed depressions refutes FE(2); an assembled trunk no larger than a tile-local one refutes FE(4); a trending largest-basin share refutes FE(6); an out-degree of 1 refutes FE(7); long runs outside filled ground refute FE(8).

**Currently `exact` as observation.** The store read was **stale-by-`src`** for the binary that measured it (the probe says so on screen and the reason it is admissible is stated there): these are claims about the *shape* of surfaces the store really holds, and a surface carved under a previous source tree is honest terrain for a shape question. Re-running after a rebuild is expected to move the digits and not the clauses; if it moves a clause, that is a finding and this segment returns to `draft`.

**Not measured, and therefore not claimed:** what the depression census looks like under a whole-face tile (coast-only outlets, no edge sinks) — the one configuration where FE(1)'s artefact is absent, and the obvious control; whether any of this changes at L6 or L21; whether the 12 depression cells of FE(2) are geography or deposition pits. The **remedy** is likewise not claimed: halo-exchanged or flux-BC tile boundaries ( #form-seam-flux-exchange , #form-face-flux-register ) are the named direction, and neither is built or costed here.

**Probe sensitivity** ( #norm-probe-sensitivity ): the depression measure is convicted in both directions by unit test — it reports zero on a tile graded to its outlets and fires on a constructed 200 m crater. Without the second half, "no depressions anywhere" would be indistinguishable from a measure that never fires.

Stage `draft`.

## Discussion

The policy is not a mistake. `Fluvial::outlets` documents edge sinks as *"the principled incomplete-tile base level until flux-BC"*, and full-face tiles already refuse them because they carved 300 m trenches around every cube face. What was missing is the price, and the price turns out to be most of the hydrology: no lakes, basins capped at 78 km, a network frozen after ten epochs, and — the moment anyone assembles tiles for a view — 13% of the surface in seam-manufactured pits with straight drainage crawling across them.

It also relocates a question. "Why is there no inland standing water?" reads as a water-kernel question and was filed as one. It is an *erosion boundary-condition* question, and the water kernel is downstream of it. That is the ordinary shape of #norm-probes-before-claims : the instrument built to look at one system convicts the system underneath it.

## Working Notes

- **The control that would sharpen FE(2) and FE(5) most cheaply:** run the depression census over a *whole-face* erosion tile (coast-only outlets). If closed depressions appear there, the tiling is the sole cause and the number is clean; if they do not, some of the absence is the kernel's fill-every-epoch composition and the diagnosis must split. Untried.
- **Do not read FE(5)'s 13% as a lake inventory.** It is the assembled patch's geometric capacity to its spill points, seam artefacts included, with no evaporation, inflow, seepage or residence time in the account. A view that painted it as water would be inventing both the water and the basins.
- **Consumers:** `Fluvial::drainage_surface` is the reader all of this is measured through — a recompute, not a store citizen ( `DECISIONS[drainage-is-a-reader-of-a-stage-not-a-store-citizen]` ). `Fluvial::from_region` leaves `precip_weight` at ones, so any caller that does not supply climate is reading uniform-rain discharge; at the beacon's 313 km span that is worth $<1\%$ (the fated jitter's features are ~1000 km, so it is nearly constant across a patch), but the gap widens with span and it is not a safe default to inherit.
- **Forward:** FE(6) is the measurement the tectonic-driver work wants inverted — if integrated basins are the goal, this segment says the tile boundary, not the epoch count, is what currently bounds them.
