---
slug: obs-connectivity-fills-the-basins-the-threshold-drained
type: observation
status: empirical
stage: draft
depends:
  - form-ocean-is-connectivity-not-elevation
  - form-derived-sea-level
  - form-fidelity-ladder
  - norm-probes-before-claims
  - norm-probe-sensitivity
---

# Bounding the ocean by connectivity finds a million cubic kilometres of standing water, and most of what a view draws is still prior detail

Classifying ocean by reachability rather than by height ( #form-ocean-is-connectivity-not-elevation ) turns every landlocked below-datum basin from a drain into a body of water. Measured on the whole-face domain, this world holds **1165 standing bodies, 976 865 km³, deepest 1271 m** — all of it previously ocean-by-threshold, and holding nothing. Each body's surface is **level to the bit**.

The same instrument measures the limit of the picture: on a real carved window the standing-water field is **8.5× larger on the uncarved prior than on the carved surface**, so a view drawing prior detail at a level finer than the carve reports mostly undrained prior dimples rather than drainage-integrated basins.

## Formal Expression

1. **A standing body is exactly level; the routing raise is not.** Every cell of one body shares one spill float, so `bed + standing_water` is bit-identical across it. On the analytic construction — a 200 m cone gouged into a plane falling 5 m per cell — the field returns depth **100.000 m** at surface spread **0.000000 m**, against the constructed spill level; the ε-augmented routing raise returns 100.020 m at spread 0.024 m. (`examples/lake_surface_probe` A.)

2. **The routing raise reports water where none can stand.** On a flat shelf above the waterline joined to a monotone ramp — a surface with no closed depression anywhere — the raise reports **4418 of 9216 cells wet and 0.0605 km³**, all of it the flat-orienting ε read as depth. The standing-water field reports **zero cells**. (`examples/lake_surface_probe` B.) The ε is ~0.02 m and therefore below the 1 m floor used by `depression_cells` and by the depression paint, so no shipped capacity figure moves; what the raise cannot support is any threshold at depth > 0, or any claim of levelness.

3. **A submarine window holds no basin, by construction.** Where every cell lies below the datum and reaches the window rim, every cell is ocean and nothing stands. Since ~95% of this planet is submerged, a footprint chosen without checking is probably one — the canonical L19 water footprint sits 1.7 km under the waterline and reports zero depressions at every threshold. (`examples/lake_surface_probe` D.)

4. **The planet, per whole cube face at L8** (the domain that adjudicates real basins; a window narrower than an enclosed sea reads that sea as ocean — #form-ocean-is-connectivity-not-elevation FE(4)):

   | face | submerged / 65536 | bodies | cells | volume (km³) | deepest (m) | all level |
   |---|---|---|---|---|---|---|
   | XPos | 62 302 | 196 | 758 | 164 242 | 1022.0 | yes |
   | XNeg | 64 149 | 90 | 340 | 104 327 | 1096.5 | yes |
   | YPos | 59 057 | 324 | 1074 | 145 859 | 792.4 | yes |
   | YNeg | 58 966 | 381 | 1691 | 386 230 | 1271.0 | yes |
   | ZPos | 64 587 | 44 | 139 | 11 435 | 461.9 | yes |
   | ZNeg | 63 181 | 130 | 539 | 164 774 | 1081.5 | yes |
   | **total** | | **1165** | **4541** | **976 865** | **1271.0** | **yes** |

   Coarse by declaration: an L8 cell is ~78 km, so only basins larger than that appear at all and the count is a **floor**. For scale, Earth's lakes hold ~180 000 km³; this is a wetter planet read at 78 km cells, so the order is right and the ratio is not a defect.

5. **Carving destroys closed basins; the prior is full of them.** One emerged L13 window (100% land, 1784 m relief), same geography, same reader:

   | surface | wet cells / 9216 | bodies |
   |---|---|---|
   | after 60 epochs of fluvial carve | 210 | 26 |
   | the **uncarved** band-limited prior | 1780 | 59 |

   The carve removes **88% of the prior's standing water**, which is drainage integration doing its work. **Consequence for every view:** the reader runs on the surface *drawn*, and under #form-fidelity-ladder a view finer than the carve draws a coarser carve plus the fine prior's detail re-added — relief that is real law but that no fluvial run at that scale produced. So a fine view's standing-water field is dominated by **undrained prior dimples**, which is why so many painted bodies sit on slopes with an apparent outlet to the sea: at the drawn resolution the dimple genuinely is a pit. The elevation half of this mixing is declared in the HUD; the standing-water number inherits it silently.

6. **The rebuild that carried the classification** (world `first-light`, base `6e4e919`; every key re-derived, 0 hits). Payloads whose bytes changed, against the previous cohort:

   | nomos | changed | unchanged |
   |---|---|---|
   | erosion-tile L9 | 1065 | 2487 |
   | erosion-tile L13 (beacon) | 452 | 28 |
   | water-tile | 384 | 0 |
   | initial-topography · climate · mantle-thermal · hydrosphere | 0 | all |

   Erosion at L9 moved on **30.0%** of tiles and the beacon on **94%**. Topography, climate and the thermal chain are untouched, as they must be — none of them consumes the ocean set.

7. **The marched water field began at the retired datum.** `water_tile` computed the derived waterline for its rim boundary condition and initialised the sim at `gen::SEA_LEVEL_M` instead: a deficit of **1106.2646 m on every interior cell**, measured at `steps = 0` on a footprint below both datums, with only the rim pinned correctly. The bounded fill cannot relax a kilometre ( #obs-water-fill-never-settles ), so the Water paint drew ocean depth wrong by up to that much everywhere except tile rims.

## Epistemic Status

**Max attainable: `exact`** for FE(1)–(3) and FE(5) — deterministic constructions with analytic answers, each falsifiable in both directions (an enclosed basin holding nothing refutes the classification; connected ocean reported as standing water refutes it the other way; both arms are run). FE(7) is exact: one measured number against a constructed prediction.

**Currently `empirical`.** FE(4) is one seed at one coarse level and is a floor, not a census. FE(6) is a single rebuild, and its **geography is unexplained** — see the Discussion. Stage `draft`.

**Probe sensitivity** ( #norm-probe-sensitivity ): the instrument reports per-body tables and a levelness spread rather than a pass, because a 1 m threshold is exactly what hid FE(2) — the ε artefact is real, systematic, and entirely below it.

## Discussion

**The unexplained half, stated as such.** FE(6)'s 30% was pre-registered at 1–10% (`msc/lake-connectivity-2026-07-29-prereg.md`), on the reasoning that only a tile holding *both* land and enclosed below-datum ground can change. The miss is a factor of three, and the two obvious repairs to that reasoning are both refuted by the per-face distribution: changed-tile fraction runs XPos 38.9% · XNeg 32.8% · ZPos 23.4% · YPos 10.9% · ZNeg 10.5% · YNeg 3.1%, which is **inversely** ordered against both land fraction and FE(4)'s body count — the face with the most land and the most lakes changed least. So neither "more land, more coastline" nor "more basins, more change" survives. Something about *where* the reclassified basins sit relative to region-carve domains is doing the work, and nothing here measures it. Naming that gap is worth more than a third guess.

**Why the wet limit is not a knob.** Filling to the sill introduces no parameter: the sill is a property of the bed, and the surface it implies is the hydrologic steady state under positive net supply. What it omits is a water *balance* — and that omission is where the endorheic question lives, not in the fill. Prior art carries the next rung ready-made (Fill–Spill–Merge's closed-form partially-filled lake level; CHONK's evaporation term in the level balance), which is an argument for adopting rather than deriving.

**On the marching kernel.** These bodies exist without any settle: the spill field is a pure function of the stored bed, so it is available at every level the bed is, including a beacon the builder carves erosion-only precisely because water at fine levels is blocked. The step-size question ( #obs-water-fill-never-settles ) is therefore not what stood between this world and standing water, and treating a lake as something to be marched to is the wrong shape of problem — under steady flow the local-inertial scheme reduces to the diffusion-wave model, so a stationary solve targets the same answer, and a near-horizontal water surface is that scheme family's documented worst case.

## Working Notes

- **Owed, and the honest headline for the paint:** FE(5) means the depression paint's number needs a fidelity declaration of its own — either the reader refuses to report standing water above the carve level, or the HUD says the figure is measured on re-added prior detail. Joseph read this off the screen before it was measured (2026-07-29): *"a lot of these have ostensible outlets to the ocean."*
- **Owed:** explain FE(6)'s geography, or record that it resisted one honest attempt.
- **Owed:** a whole-face domain at a level finer than L8, to turn FE(4) from a floor into something closer to a census.
- **Not measured:** what fraction of FE(4)'s volume sits in basins whose enclosure survives a domain wider than one face — cube-face boundaries cut real basins, and FE(4) is blind to any basin straddling one.
- The `#form-derived-sea-level` Caspian note (Joseph, 2026-07-28) named this as elevation-versus-connectivity before any of it was measured; that naming was exact, and the sighting is the observational half of FE(4).
