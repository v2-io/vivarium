---
slug: obs-lakes-are-routed-over-not-carved-away
type: observation
status: exact
stage: draft
depends:
  - detail-erosion-composition
  - obs-tile-outlets-grade-away-the-basins
  - obs-water-fill-never-settles
  - obs-chi-shape-is-erosions-criterion
  - form-declared-boundary-contract
  - form-time-indexed-stage-chains
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - norm-declaration-must-convict
  - norm-caught-disciplines-become-mechanisms
---

# The depression fill is a routing surface the epoch restores, so a closed basin survives the carve that routes over it

`Fluvial::erode` fills depressions to derive its drainage network and puts the raise back before the epoch ends. Measured on a whole-face L9 carve: the uncarved prior's 2.65 % of land in closed basins declines to **0.51 % at 40 epochs and 0.47 % at 300**, deepest 212–635 m, where the previous composition — which kept the raise — left 0.12 % after one epoch and 0.02 % after forty. The bed can hold water; whether water *settles* into it is a separate and still-open question ( #obs-water-fill-never-settles ), and at the shipped tile grain the boundary contract still drains most of what the bed retains ( #obs-tile-outlets-grade-away-the-basins ).

## Formal Expression

1. **The composition, by inspection.** Step (2) of the per-epoch composition ( #detail-erosion-composition FE(3)) computes a **routing surface**: every closed depression raised to its spill point, plus an $\varepsilon$-gradient so that D8 has a receiver on the resulting flat. Steps (3)–(6) — receivers, drainage accumulation, incision, deposition — are derived on that surface, because flow that cannot cross a lake and leave by its spill point cannot be routed at all. The epoch then **subtracts the whole raise** before talus and creep, so the heights the store receives are the bed. The epoch loop and the three readers that call the same fill (`drainage_surface`, `chi_profile`, `response_census`) now agree; the asymmetry between them was the defect this replaces.

2. **The fill returns two fields, not one, and separating them is what makes the restore honest.** `fill_depressions` reports the **true spill level** minus the original height — the running maximum of *original* heights along the flood path — carried beside the heap key and never ordering it, so every routed quantity is bit-identical to a fill that does not track it. That depth is exactly zero on any cell at or above its own spill level, **including every cell of a perfectly flat area**, whose spill level is its own height. The $\varepsilon$ is therefore never mistaken for standing water, and the alternative — thresholding total fill depth against a magic depth in metres — is not needed.

3. **Measured: the prior's basins decline gradually and settle at half a percent of land.** Whole-face carve at L9 (512², coast-only outlets, seed 17425063241017297386, f0), depression cells counted as fill depth $\gt 1\,\mathrm{m}$ by the same whole-face reader in every row:

   | | prior | ep 1 | ep 2 | ep 3 | ep 4 | ep 5 | ep 6 | ep 7 | ep 8 | ep 40 | ep 300 |
   |---|---|---|---|---|---|---|---|---|---|---|---|
   | cells | 671 | 613 | 571 | 509 | 465 | 441 | 397 | 377 | 357 | 134 | 228 |
   | % of land | 2.65 | 2.42 | 2.25 | 2.01 | 1.83 | 1.74 | 1.56 | 1.48 | 1.40 | 0.51 | 0.47 |
   | deepest (m) | 688 | 673 | 657 | 640 | 623 | 617 | 616 | 614 | 612 | 635 | 212 |
   | capacity ($10^{13}\,\mathrm{m^3}$) | 2.182 | 1.919 | 1.784 | 1.558 | 1.503 | 1.355 | 1.203 | 1.137 | 1.137 | — | — |

   The same instrument under the previous composition read **31 cells (0.12 %) at epoch 1** and **2 cells at epoch 40** on this face, from the identical prior. The falsifier that segment named for its own central clause — *"a gradual decline of the prior's depressions over the first eight epochs rather than a collapse at epoch 1"* — is what this table now is, because the mechanism it described has been removed.

   **The decline is real erosion, not a residual of the fill.** Capacity falls 48 % over eight epochs while the deepest basin loses 11 % of its depth, which is the signature of many shallow basins being drained by outlet incision and silted by their own catchments while the deep ones persist. Over a full a-priori response time (300 epochs) the deepest surviving basin is 212 m and the count is stable against the 40-epoch reading.

4. **Three physics decisions the restore forces, none of them defaulted.** Routing on a filled surface while writing to an unfilled one leaves three questions the old composition never had to answer, and each is a choice rather than a consequence:

   | question | what the kernel does | what it costs |
   |---|---|---|
   | Does a submerged cell incise? | **No.** $E = K A^{m} S^{n}$ has no channel slope under standing water. A subaerial cell draining *into* a lake keeps its term and takes its receiver's height from the lake surface — the local base level a river entering a lake sees. | Nothing measured. Its purpose is to keep the $\varepsilon$ out of the bed: unmasked, the largest $A$ on the surface multiplies an $\varepsilon$-slope and planes the floor along the flood direction (FE(6)). |
   | Where does sediment entering a lake go? | **Into the bed, up to the remaining capacity, and the surplus spills.** Trapping efficiency is 1 until the basin is full. | Declared as an unphysical term (`nomotheke`, sign-definite): a settling-velocity/residence-time fraction would silt more slowly and starve downstream reaches less. Mass is conserved either way; the error is in the *rate* of infill, not the budget. |
   | Do talus and creep see the water level or the rock? | **The rock.** They are mass-conserving redistributions of rock, and the water surface is not a material surface. | Subaqueous hillslope transport is therefore present (a basin's walls slump into it) while nothing distinguishes it from subaerial transport. Running them on the filled surface instead would silently make the fill's manufactured rock real. |

5. **Measured: the $\varepsilon$-fill's mass term is retired and its directional term is not.** Neither the spill fill nor the $\varepsilon$ reaches the bed, so the $\approx 2\times10^{13}\,\mathrm{m^3}$ per L9 face that the first epoch used to convert into rock is never created — the row that named it a *"sign-definite mass mint"* is replaced by one naming it directional. What survives is that the $\varepsilon$ still decides *where flow goes* across a flat, and incision is applied where flow goes, so the long straight runs #obs-tile-outlets-grade-away-the-basins FE(8) measured in filled ground can still reach the bed through the incision pattern. That the mass half is gone is convicted; that the directional half is unchanged is **argued, not measured**, and the run census that would settle it has not been re-run.

6. **Measured: the restore and the incision mask fail in different sizes, and each has its own conviction.** Four unit tests, each checked against its own known-bad by hand:

   | test | what reverting it does |
   |---|---|
   | `a_crater_survives_the_epoch_that_routes_over_it` | Dropping the restore: 0 depression cells under either boundary contract, from 96 at 197 m. |
   | `an_epoch_over_a_pitted_bed_adds_no_rock` | Dropping the restore: rock volume rises by roughly the crater's own volume — the planet-scale mint at unit scale. |
   | `a_lake_floor_is_not_quietly_planed_by_the_epsilon_gradient` | Dropping only the incision mask: 50 of 144 floor cells move in **one** epoch, by up to $4.9\times10^{-2}\,\mathrm{m}$, oriented along the flood direction. A metre over a settle history, in every lake bed. The claim is bit-exactness rather than a tolerance, because the quantity would hide inside any tolerance. |
   | `a_lake_silts_up_toward_its_spill_point_and_not_past_it` | The trap semantics of FE(4): the bed rises toward the water surface and never past it. |

   The mask's first draft asserted that the crater test would convict it. It does not — the crater test passes with the mask removed, because on the filled surface a lake cell's receiver is at the same lake surface and the unmasked erosion is $\varepsilon$-sized. The assertion was corrected and the conviction built, which is the shape #norm-declaration-must-convict asks for.

7. **Measured: the tile-grain sweep keeps its ordering and gains an order of magnitude.** One geography, tile grain swept from 8 cells to the whole face at fixed prior, uplift and epoch count (f0, L9), depression cells:

   | grain (cells) | 8 | 16 | 32 | 64 | 128 | 256 | 512 | prior |
   |---|---|---|---|---|---|---|---|---|
   | 40 epochs | 571 | 438 | 308 | 178 | 174 | 125 | **134** | 671 |
   | 300 epochs | 757 | 541 | 491 | 255 | 224 | 219 | **228** | 671 |

   Depressions still fall as the grain grows, so the seam pits #obs-tile-outlets-grade-away-the-basins FE(5) identifies are still manufactured at fine grain — but the whole-face control, which has no internal seam to manufacture one, now retains 134–228 cells where it previously retained 2. **The two populations are therefore separable for the first time**: the whole-face row is inherited geography, and the excess at grain 8 (571 against 134) is the seam.

8. **Measured: basin integration and the seam signature are unmoved.** Land trunk on the whole-face carve is $7.733\times10^{10}\,\mathrm{m^2}$ at 40 epochs and $1.930\times10^{11}$ at 300, against the prior's $4.687\times10^{10}$ — inside the spread the previous composition produced on the same face ($5.96$–$7.88\times10^{10}$ at 40; $1.79\times10^{11}$ at 300). The 64-cell seam ratio at grain 64 reads 1.004 at 40 epochs and 1.166 at 300 against the prior's null of 0.963, against 1.013 and 1.171 before. Retaining basins neither integrates the network nor disturbs the tiling's own signature; those remain #obs-tile-outlets-grade-away-the-basins ' business.

9. **Measured: the $\chi$ criterion moves slightly away from linear, which is the expected sign.** L13 beacon, 16 tiles × 30 stages, against the immediately preceding cohort (same prior, same uplift nomos, landed 100 minutes earlier — `examples/chi_convergence_probe`, which selects by cohort so the comparison is not an accident):

   | at stage 300 | before | after |
   |---|---|---|
   | $1-R^2(\sqrt{\ })$ | 0.2882 | 0.3077 |
   | zero-parameter ratio | 0.9193 | 0.9751 |
   | mean $\lvert\Delta h\rvert$ | 0.4172 m/epoch | 0.4109 m/epoch |

   A landscape holding lakes is further from stream-power steady state than one whose basins have been filled flat, so a rise of 7 % in the linearity residual is the direction the criterion should move. It is far below the knickpoint amplitudes that segment convicts at (×1.29 and up — #obs-chi-shape-is-erosions-criterion ), so the criterion's discriminating power is intact. **Not claimed:** that 7 % is *caused* by retained basins rather than by anything else in the same source tree. The cohorts differ by whole-crate digest, and this change is what moved in it.

10. **Measured: the cost is not the obstacle.** The L9 build's erosion sweep runs 384 tiles in 12.0 s against 12.4 s before; a whole-face L9 carve costs 2.7 s at 40 epochs and 20.4 s at 300. The added work per epoch is one height clone and two linear passes.

11. **What the repair does not buy, measured.** On the rebuilt store, a **per-tile** read of the 384 L9 tiles finds 8 tiles holding 23 depression cells, deepest 367 m — against 3 tiles and 12 cells before. That is a small number and it is not the bed's fault twice over: the reader's own edge sinks drain any basin touching a tile edge, and, upstream of the reader, the shipped carve grades each tile to its perimeter so those basins were never carved in the first place ( #obs-tile-outlets-grade-away-the-basins FE(1)). The assembled L13 beacon patch reads 12 893 of 65 536 cells (19.67 %) in closed depressions, deepest 322 m, against 8 532 (13.0 %) before — but that statistic mixes inherited basins with seam pits and cannot separate them, which is exactly what FE(7) can. **The honest summary: this removes the blocker that no bed could hold water anywhere, and leaves the cross-tile base level as the blocker for whether the shipped tiling holds it where geography says it should.**

12. **Out of bounds for this segment.** Any claim that water now stands in these basins — `water-tile` settles 40 s of world time at any level ( #obs-water-fill-never-settles ) and would not fill them; the boundary contract, which is #obs-tile-outlets-grade-away-the-basins ' and #form-declared-boundary-contract 's; lake evaporation, seepage, residence time, or any hydrological account of a basin beyond its geometric capacity; and the deposition law's calibration, which is untouched.

## Epistemic Status

**Max attainable: exact** for the quoted measurements. `examples/base_level_probe` opens no store and writes nothing — initial topography, uplift rate and precipitation jitter are pure functions of seed and cell — so every arm is reproducible from its printed header. FE(9) and FE(11) read the store and say which cohort they read.

**Currently `exact` as observation.** FE(1)–(2) are exact by inspection of `erosion.rs`. FE(3), (7), (8), (10) are measured on one cube face at L9 under one seed at two epoch counts. FE(6) is four unit tests with hand-checked known-bads. FE(5)'s second half and FE(9)'s attribution are explicitly *not* claimed at the tier of the rest.

Falsifiers, all cheap: a collapse of the prior's depressions in the first epoch refutes FE(3); a stored bed whose volume exceeds its pre-epoch volume refutes FE(5); depression counts *rising* with grain refutes FE(7); a land trunk outside the previous composition's spread refutes FE(8); a $\chi$ residual moving *toward* linear refutes FE(9)'s sign.

**Probe sensitivity** ( #norm-probe-sensitivity ): the depression measure is `DrainageSurface::fill_depth`, convicted in both directions by `depression_capacity_fires_on_a_pit_and_not_on_a_graded_dome` — silent on a graded dome, loud on a constructed 200 m crater. The grain sweep carries its own control in the prior row.

**Scope, and therefore not claimed.** One seed, one face, one level for the persistence table; the L11 and multi-face replications the previous composition had are not re-run here. Whether the $\varepsilon$'s directional artefact still reaches the bed (FE(5)) is argued rather than measured. Whether a basin retained at L9 survives at L13 under the shipped contract is FE(11)'s open half.

Stage `draft`.

## Discussion

The two defects that kept water out of this world were serial, and only the first was visible. A bed with no depressions cannot hold a lake no matter what the water kernel does; that is now repaired, and what stands behind it is the boundary contract, which drains at the tile edge what the fill no longer drains at the spill point. Removing a blocker does not produce the phenomenon — it promotes the next blocker to load-bearing, and the useful thing a repair can do is say which one that is with a number attached.

The repair is small and the decisions around it are not. Restoring the raise is four lines. Deciding whether a lake bed erodes, whether it silts, at what efficiency, and whether hillslope processes see rock or water is where the physics lives, and none of those questions existed while the fill was being kept — the old composition answered all four implicitly with *"there are no lakes."* That is the ordinary shape of a defect that removes a degree of freedom: it looks like a bug and it is also a modelling decision nobody made.

Worth naming for the next mind: the tripwire that caught this was written the same day, by an agent who could not land the repair and instead left a failing-when-fixed test whose message named the segments due for replacement. It fired alone out of 177, and the replacement list in its message was correct. That is #norm-caught-disciplines-become-mechanisms working in its intended direction — an incident converted into a mechanism, and the mechanism doing the next mind's bookkeeping.

## Working Notes

- **The directional half of the $\varepsilon$ (FE(5)) is the cheapest open item.** `examples/discharge_probe` already measures longest identical-D8-direction runs and the fraction inside filled ground; re-running it against the new beds says whether restoring the raise also removed the straight-run artefact from the bed or merely from the heights. Prediction, recorded before running: the artefact persists at reduced amplitude, because incision is still *applied* where the $\varepsilon$ sent the flow.
- **Trapping efficiency 1 is the least defended of the three decisions in FE(4).** A residence-time or settling-velocity fraction is the standard alternative and would change how fast the FE(3) capacity column falls. Nothing here measures which is right; the row exists so the choice is visible rather than so it is settled.
- **The L11 and multi-face replications are not re-run.** The previous composition's persistence claim was replicated on three faces at L9 and one at L11, and matching that would make FE(3) as well-supported as what it replaces. `VIVARIUM_LEVEL=11 VIVARIUM_FACE=…` on the same instrument is the whole cost.
- **FE(11) is the handoff to the cross-tile work**, and the number it wants next is a beacon-footprint carve under `NoFluxWall` run as a stored *settle history* rather than an endpoint — which is the same experiment #obs-tile-outlets-grade-away-the-basins names as its own honest gap. One experiment closes both.
- **Consumers:** `examples/base_level_probe` (`VIVARIUM_SEED` / `LEVEL` / `EPOCHS` / `TILE` / `FACE` / `STRIDE` / `GRAIN_MIN`), `examples/chi_convergence_probe`, `examples/discharge_probe` Parts 6–7. A whole-face L9 carve at 40 epochs costs 2.7 s, so the sweep is cheap enough to re-run against any kernel change that claims to affect the bed.
