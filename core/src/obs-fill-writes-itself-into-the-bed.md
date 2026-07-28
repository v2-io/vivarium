---
slug: obs-fill-writes-itself-into-the-bed
type: observation
status: exact
stage: draft
depends:
  - detail-erosion-composition
  - obs-tile-outlets-grade-away-the-basins
  - form-declared-boundary-contract
  - form-seam-flux-exchange
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - norm-declaration-must-convict
---

# Each epoch's depression fill is written into the stored bed, and that — not the tile grain — is what leaves the world without a lake

`Fluvial::erode` raises every closed depression to its spill point and keeps the raise. The uncarved prior holds 2.3–2.65 % of its land in closed depressions; one epoch of carving leaves 0.12–0.14 %, and the boundary contract makes no difference — a whole-face coast-only carve holds **fewer** depressions than any tiled one.

## Formal Expression

1. **The mechanism, by inspection.** Three readers on `Fluvial` — `drainage_surface`, `chi_profile`, `response_census` — each save `self.h`, call `fill_depressions`, and restore before returning, because a reader must not advance the world. The epoch loop does not. `Fluvial::erode` calls `fill_depressions` as step (2) of the eight-step composition ( #detail-erosion-composition FE(3)) and never restores, so Priority-Flood's raise is carried through incision, deposition, talus and creep into the heights the store receives. **At the instant the fill returns, the domain contains no closed depression by construction.** The only depressions a stored bed can hold are those the four steps *after* the fill created.

2. **Measured: the prior's depressions die in the first epoch.** Whole-face carve at L9 (512², coast-only outlets, 40 epochs, seed 17425063241017297386), depression cells counted as fill depth $\gt 1\,\mathrm{m}$:

   | face | prior cells (% land) | prior capacity | epoch 1 | epochs 2–8 | epoch 40 |
   |---|---|---|---|---|---|
   | f0 | 671 (2.65 %) | $2.182\times10^{13}\,\mathrm{m^3}$ | 31 (0.12 %) | 9–37 | 2 |
   | f1 | 639 (2.65 %) | $2.157\times10^{13}\,\mathrm{m^3}$ | 35 (0.14 %) | 8–60 | 9 |
   | f4 | 309 (2.30 %) | $1.078\times10^{13}\,\mathrm{m^3}$ | 17 (0.13 %) | 3–20 | 5 |
   | f0 at L11 | 27 026 (6.41 %) | $4.351\times10^{13}\,\mathrm{m^3}$ | 520 (0.12 %) | 227–431 | — |

   The capacity column is exactly the rock volume the first fill mints, since fill depth *is* $\text{filled }h - \text{stored }h$ summed over cell areas. It is a **planet-scale** term: $2.2\times10^{13}\,\mathrm{m^3}$ over one L9 face's $\approx 8\times10^{12}\,\mathrm{m^2}$ of land is a mean of about 2.7 m of manufactured rock, concentrated entirely in the basins. The L11 row is the level control: the prior there holds 6.41 % of its land in depressions rather than 2.65 %, and the first epoch lands on **the same 0.12 %**. Where the surface ends up is set by the fill and by what the post-fill steps re-create, not by what the terrain brought.

3. **What remains is a dynamic equilibrium, not a floor.** Epochs 2–8 hold 3–60 depression cells with no trend on any face, and the deepest surviving depression moves between 17 m and 1017 m from epoch to epoch. Deposition, talus and creep create pits after the fill has run; the next epoch's fill erases them. The 0.02 % `#obs-tile-outlets-grade-away-the-basins` FE(2) measured on the live store is one sample of this population — a steady state between two steps of one composition, not a property of the terrain.

4. **Measured: the boundary contract is not the cause, and its sign is the opposite of the expectation.** Carving one face at fixed geography, epochs and prior, sweeping only the tile grain from 8 cells to the whole face (the largest rung takes `Fluvial::outlets`' full-face branch, so it *is* the coast-only control), depression cells fall **monotonically as the grain grows**:

   | grain (cells) | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 |
   |---|---|---|---|---|---|---|---|---|---|
   | f0 L9 | 503 | 335 | 185 | 51 | 48 | 11 | **2** | — | — |
   | f1 L9 | 511 | 275 | 151 | 92 | 26 | 9 | **9** | — | — |
   | f4 L9 | 174 | 90 | 64 | 21 | 6 | 9 | **5** | — | — |
   | f0 L11 | 27 798 | 22 003 | 14 864 | 9 010 | 3 711 | 2 048 | 1 347 | 431 | **260** |
   | f0 L9, 300 ep | 648 | 507 | 357 | 132 | 46 | 11 | **15** | — | — |

   Removing the tile edges removes depressions rather than restoring them. The depressions a tiled carve retains are #obs-tile-outlets-grade-away-the-basins FE(5)'s **seam pits** — adjacent tiles graded to different perimeter heights — now measured as a function of grain rather than at one point. Finer tiling does not buy lakes; it buys manufactured pits that a view would paint as lakes.

5. **Measured: no cap on the assembled bed's trunk appears at any grain, at either level or either epoch count — and the statistic that would show one is contaminated.** Largest MFD drainage on a *subaerial* cell of the assembled bed is non-monotonic in the grain everywhere it was measured. At L9/40 epochs the spread is 6–15 % with no ordering (f0 $5.96$–$7.88\times10^{10}\,\mathrm{m^2}$, f1 $4.08$–$4.55$, f4 $1.50$–$1.80$, all $\times10^{10}$), and there the shipped 64-cell tile spans 1251 km against a basin of equivalent width 279 km, so its edges are never reached. Two sweeps push past that excuse:

   | | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 |
   |---|---|---|---|---|---|---|---|---|---|
   | L11/40, span (km) | 39 | 78 | 156 | 313 | 625 | 1251 | 2502 | 5004 | 10008 |
   | L11/40, land trunk ($10^{10}$) | 5.24 | 6.01 | **15.4** | 7.06 | 6.90 | 6.88 | 6.86 | 6.89 | **6.88** |
   | L9/300, land trunk ($10^{10}$) | 23.6 | 12.8 | 11.6 | 19.2 | 18.7 | 30.0 | **17.9** | — | — |
   | L9/300, largest landmass only ($10^{10}$) | 19.9 | 12.8 | 7.88 | 7.54 | 7.76 | 7.75 | **7.67** | — | — |

   At L11 the finest grain is six times narrower than the basin and suppresses the trunk only to $0.76\times$ the whole-face control, while a 156 km grain *raises* it to $2.2\times$. At L9/300 — a carve mature enough to have grown the trunk fourfold over the prior — the whole-face control is neither the largest nor the smallest value in its row.

   **The last row is why this clause claims an absence rather than a bound.** Restricted to the largest landmass, the trunk is flat at $\approx 7.7\times10^{10}$ for every grain from 32 up and then *rises* to $19.9\times10^{10}$ at grain 8 — the direction opposite to a cap. The mechanism is FE(4): a grain-8 carve leaves 648 seam-pit cells, the whole-face reader fills them, and filled ground merges neighbouring catchments into one accumulation. So the tiling acts on this statistic in **two opposing directions at once** — starving basins through the edge sinks, merging them through the seam pits its fill has to flatten — and the trunk cannot separate them. What can be said is what was measured: over grains spanning 39 km to 10 008 km, two levels and two epoch counts, no ordering in the grain survives. 94.9 % of f0's L9 land sits in landmasses spanning more than a tile, so the defect is reachable throughout.

6. **The 3.85× is not contradicted, because it measures a different quantity.** #obs-tile-outlets-grade-away-the-basins FE(4) compares **tile-local routing to assembled routing on one bed** — how starved the kernel's own incision driver is, which is a property of the carve regardless of what the assembled surface looks like. FE(5) here compares **assembled routing on differently carved beds** — what the world ends up being. Both are true, and the gap between them has a mechanism: a driver starved by $3.85\times$ enters incision as $A^{m}$ with $m=0.5$, so it reaches the bed as a factor of $1.96$ in *rate*, and a rate difference of that size acting for 40 epochs on 2 km of relief does not redraw a routing network that the band-limited prior has already largely set. The two statements diverge in consequence, not in fact — and the consequence gap is the thing a repair has to be justified by, since it is the bed, not the driver, that a world is made of.

7. **Measured: the settle integrates the network by $2.4\times$, and the two contracts track each other rung for rung.** Land trunk over a 300-epoch history on f0 at L9, tiled at grain 64 against the whole-face control:

   | epochs | 50 | 100 | 150 | 200 | 250 | 300 |
   |---|---|---|---|---|---|---|
   | tiled ($10^{10}\,\mathrm{m^2}$) | 7.76 | 8.12 | 9.32 | 13.7 | 14.1 | 19.2 |
   | whole face ($10^{10}\,\mathrm{m^2}$) | 7.75 | 8.07 | 7.08 | 13.2 | 15.2 | 17.9 |

   Largest-basin share rises from 0.009 to 0.013 in both. Over 40 epochs the growth from the prior is $1.66\times$ (f0), $1.53\times$ (f1), $1.27\times$ (f4); over 300 it is $\approx 4\times$ the prior and $2.4\times$ the 50-epoch bed. **The epoch count is what bounds basin integration at this level, and the boundary contract is not** — the arms agree to within 10 % at every rung, including through the two reorganization events near epochs 200 and 300 that both of them undergo.

8. **Measured: the seam step is the tiling's clean signature, and it matures with the carve.** Mean $\lvert\Delta h\rvert$ across 64-cell boundaries over mean $\lvert\Delta h\rvert$ elsewhere, subaerial links only. At 40 epochs the grain-64 carve reads 1.013 / 0.952 / 1.045 against the uncarved prior's null of 0.963 / 0.918 / 1.005 on f0 / f1 / f4 — the tiling raises the ratio by 0.034–0.050, the same sign three times. At **300 epochs on f0 it reads 1.171 against the same 0.963 null**, a rise of 0.208: four times the 40-epoch signature. The step is cumulative, as a tile grading toward its own perimeter for longer should be.

   Across the 300-epoch sweep the signature also peaks where it must — 0.890 / 0.995 / 1.081 / **1.171** / 0.993 / 0.875 / 0.893 at grains 8 through 512 — maximal exactly at the grain the metric's own 64-cell boundaries are drawn on. A carve at grain 8 puts a seam on *every* boundary, so seam and interior links are alike and the ratio returns to the null; a carve at grain 512 has no internal seam at all. That the peak lands on the diagonal is the measure convicting itself.

## Epistemic Status

**Max attainable: exact** for the quoted measurements under `examples/base_level_probe`. The probe opens no store and writes nothing: initial topography, uplift rate and precipitation jitter are pure functions of seed and cell, so every arm is reproducible from the printed header alone and no arm can be contaminated by what a previous build happened to leave behind.

**Currently `exact` as observation.** FE(1) is exact by inspection of `erosion.rs` (three save-restore readers against one loop that does not). FE(2)–(8) are measured on three cube faces at L9 with 40 epochs, replicated in sign and magnitude on all three.

Every clause is falsifiable, and the falsifiers are cheap: a gradual decline of the prior's depressions over the first eight epochs rather than a collapse at epoch 1 refutes FE(2); depression counts *rising* with grain refutes FE(4); a land trunk ordered by grain refutes FE(5); a whole-face history that integrates where the tiled one is flat refutes FE(7); a seam ratio at the prior's null refutes FE(8).

**Probe sensitivity** ( #norm-probe-sensitivity ): the depression measure is the same `DrainageSurface::fill_depth` already convicted in both directions by unit test (zero on a tile graded to its outlets, fires on a constructed 200 m crater). The grain sweep is its own control — the uncarved prior is measured through the identical reader and reports 2.3–2.65 %, so a measure that never fired could not have produced the top row.

**Scope, and therefore not claimed.** Two levels (L9 on three faces, L11 on one), two epoch counts (40 everywhere, 300 on f0 at L9), one seed. The whole-face arm is not an unbounded domain: it still has four cube-face edges, and treats them as a **no-flux wall** (non-outlet, no receiver outside the field) — a different undeclared boundary contract, not an absent one, so FE(5) says the basin cap moves from the tile to the face, never that it is gone. Nothing here measures the beacon regime (L13, 78 km tiles, 300 epochs) where FE(6)'s driver starvation is severe and the grain *is* below the basin scale; the scale argument of FE(5) predicts the cap binds there, and predicting is not measuring. Whether the fill's raise should be restored before incision — the repair FE(1) implies — is neither built nor costed here.

Stage `draft`.

## Discussion

The diagnosis this segment splits was a reasonable one and it was pursued in the right order. "No lakes" was filed as a water-kernel question; a probe relocated it to the bed; the bed's most conspicuous artefact was the tile-edge outlet policy, and that policy is genuinely responsible for a starved incision driver and for manufactured seam pits. What it is not responsible for is the absence of lakes, and the only way to find that out was to build the world both ways — which is why the control the earlier segment named as untried was worth more than another measurement of the status quo.

The shape of the error is worth naming because it is cheap to repeat: two defects sat on the same surface, one loud and one silent, and the loud one absorbed the attribution. The tile edge is visible in a key, in a comment, and in an assembled render. The fill is a single un-restored mutation inside an eight-step loop, adjacent to three functions that do restore. A defect that leaves no artefact to look at will be attributed to whichever neighbouring defect does.

This is also the first measurement to say what a repair to the boundary contract will and will not buy. It un-starves the incision driver, it removes the seam pits, and at fine grain it will restore basin extent. It will not put water in the world, because there is nothing wrong with the water: there is nowhere for it to sit, and the fill is why.

## Working Notes

- **The repair FE(1) implies, unbuilt:** route on the filled surface, incise on the unfilled one. This is standard in the LEM literature (fill supplies flow directions across a lake; the lake's bed keeps its hole) and it is a real physics change, not a bug fix — it changes the mass balance the $\varepsilon$-fill mint row of #detail-erosion-composition FE(6) already names, and it needs deposition into standing water to be given some account. Cost it before adopting it.
- **The measurement that would confirm FE(5)'s scale argument:** the same grain sweep at a level where the grain falls below the basin width — L11 or L12, or a beacon-footprint carve. This needs the boundary contract to become a *declared parameter* rather than a silent function of `origin == (0,0) && nx == 2^level`, since no sub-face window can be carved coast-only today. That is #form-declared-boundary-contract 's first row and this is the probe that wants it.
- **This bears on #obs-tile-outlets-grade-away-the-basins FE(6), and the tension is real.** That clause measured the beacon's network set within ten epochs and only jittering, and attributed it to base level being fixed 32 cells away — "there is no room for capture." FE(7) here runs the same 300 epochs under the same edge-sink policy at L9 and finds the trunk growing $2.4\times$, so the policy does not by itself prevent capture. The two are not yet in contradiction, because a tile is 78 km at L13 and 1251 km at L9, and the mechanism claim may well hold at its own level. But it is a mechanism claim now known not to generalize, and the experiment that would settle it is the same one FE(5) wants: a beacon-footprint carve under a declared non-sink contract.

- **Do not read the grain-8 row as a lake inventory** for the same reason FE(5) of the earlier segment gives: those 503 cells are seam pits, and their monotonic decline with grain is what identifies them as such.
- **Consumers:** `examples/base_level_probe` is the instrument; it takes `VIVARIUM_SEED` / `LEVEL` / `EPOCHS` / `TILE` / `FACE` / `STRIDE` / `GRAIN_MIN` and prints the configuration it ran. A whole-face L9 carve costs ~2.6 s, so the sweep is cheap enough to re-run against any kernel change that claims to affect the bed.
