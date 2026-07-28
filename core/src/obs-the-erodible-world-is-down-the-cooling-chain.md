---
slug: obs-the-erodible-world-is-down-the-cooling-chain
type: observation
status: empirical
stage: draft
depends:
  - form-isostasy-column
  - form-time-indexed-stage-chains
  - detail-erosion-composition
  - obs-erosion-residual-is-driver-bound
  - norm-probes-before-claims
  - norm-probe-sensitivity
---

# The terrain worth eroding is down the cooling chain, and three things about it are not what the ladder assumes

Land area triples and peak stand rises by a kilometre between the present anchor and the canonical chain's cold end, so there is a real target for a fluvial rung that has never been run against it — but the growth **saturates** well inside the chain, the collisional belts stay a **thin** fraction of the globe rather than becoming the world's dominant relief, and the pour-grain rock-mass ledger has **already removed** a substantial share of the stand the fluvial kernel would carve.

## Formal Expression

1. **Method.** All rows are a pure evaluation of the column stack along the mantle-thermal trajectory — `sea_level::tectonic_surface_at_tp` (the live post-ledger surface), `derived_sea_level_at_tp`, `lithosphere::craton_field_at_tp`, and the cooling-stage generalization of `uplift::uplift_rate_m_per_epoch` (the same finite difference $\mathrm{freeboard}(T_p-\delta)-\mathrm{freeboard}(T_p)$ evaluated at *that* stage's $T_p$ rather than at `MANTLE_TP_C`). Level 7, $48^2$ samples per cube face (13 824 cells per stage), seeds 0 / 1 / 7, stages on the canonical micro-Ga grid ( #form-time-indexed-stage-chains FE(9)) at $0.1$ Ga spacing from 3.6 Ga to 1.0 Ga. `examples/era_erosion_feasibility_probe`. **No world is built and no store is opened** — the whole surface stack is law-evaluable in $T_p$ ( #form-time-indexed-stage-chains FE(8)), which is exactly why the target can be measured before the rung that would carve it exists.

2. **Measured: there is a target, and it is substantial.** Between the present anchor ($T_p = 1550$, 3.2 Ga) and the canonical chain's cold end ($T_p = 1464.2$, 2.6 Ga):

   | quantity | seed 0 | seed 1 | seed 7 |
   |---|---|---|---|
   | land fraction, present → cold end | 5.27 % → 14.40 % | 4.61 % → 15.54 % | 6.31 % → 18.66 % |
   | peak subaerial stand (m) | 1934 → 3048 | 1509 → 2976 | 2251 → 3641 |
   | subaerial relief, s.d. (m) | 347 → 434 | 274 → 384 | 414 → 559 |

   Land area roughly triples, peak stand gains about a kilometre, and the relief the kernel would work on gains 25–40 %. The fluvial rung builds only at the present anchor ( #detail-erosion-composition FE(4): its seed surface is `gen::initial_topography_m`, which is `tectonic_surface_m` at `MANTLE_TP_C`), so none of this has been carved.

3. **Measured: the growth saturates inside the chain, so extending the chain colder is not the win.** Continuing on the same grid past the canonical cold end to 1.0 Ga ($T_p = 1375.6$) adds only 2.9 / 2.7 / 3.0 further points of land fraction over 1.6 Ga, against 9.1 / 10.9 / 12.4 points gained over the preceding 0.6 Ga — an emergence rate of ~15 points/Ga falling to ~1.8 points/Ga, an eightfold slowdown. This is the saturating craton growth ( `CRATON_GROWTH_SATURATION` ) that #form-isostasy-column FE(7) relies on for monotonicity, seen from the other side: the same clamp that keeps the cold end in-band also means the interesting window is **already inside the authored chain**. Extending `COOL_END_UGA` would preserve FE(9)'s bit-exact nesting (the grid divides evenly), and buys little.

4. **Measured: the collisional belts are real and thin.** Suture cells ( `craton_field_at_tp` with $w_2 \gt 0$ ) as a fraction of the globe, present anchor → 2.6 Ga → 1.0 Ga: seed 0 **0.00 % → 0.16 % → 0.32 %**, seed 1 **0.00 % → 0.46 % → 0.80 %**, seed 7 **0.89 % → 2.82 % → 3.63 %**. The arrival-in-time #form-isostasy-column FE(5) reports is confirmed at this sampling and is not in question. What the fraction adds is scale: at the cold end the belts occupy roughly 1–15 % of *land* depending on seed, not the majority of it. A world carved at a cold stage is a world with more land, more stand and more relief; describing it as *mountain-belt terrain* would overstate what the column places there, and a view that painted sutures as the dominant feature would be depicting past its referent ( #norm-no-depiction-without-referent ).

5. **Measured: the driver weakens as the target grows.** Median uplift rate over land (m per declared cooling step $\delta$) falls monotonically along the chain — seed 0 **1.297** at 3.5 Ga, **0.823** at the present anchor, **0.650** at 2.6 Ga, **0.550** at 1.0 Ga — and the maximum falls harder, from 7.438 at 3.4 Ga to 3.060 at 2.6 Ga to 1.516 at 1.0 Ga. So relief and driver move in opposite directions, and the erosion–uplift balance a criterion would gate on ( #obs-erosion-residual-is-driver-bound FE(4) and its Working Notes) is a different balance at every stage. The a-priori response time is *not* implicated: at $n=1$ it is independent of uplift ( `Fluvial::response_census` ; Whipple & Tucker via `msc/research-lem-sota/lem-time-interior-prior-art-2026-07-28.md` §1.3), so one authored epoch count remains as defensible at a cold stage as at the present one. What moves is the **equilibrium relief** that count approaches.

6. **Measured: the pour-grain ledger has already removed much of the stand.** Mean difference between the pre-ledger isostatic surface and the live post-ledger surface, over pre-ledger land ( #form-isostasy-column FE(9)): seed 0 **76 m** at the present anchor, **270 m** at 2.6 Ga, **491 m** at 1.0 Ga; seeds 1 and 7 within 20 % of those. Against a subaerial relief standard deviation of 347–563 m, the ledger's single $\varphi = 1$ maturity step is removing between a fifth and a whole standard deviation of stand, and the share **grows along the chain**. The drop is a pure function of $T_p$ applied fresh at each stage rather than accumulated, so it does not compound within the ledger; it does mean that any fluvial history carried from one stage to the next would be carved on top of a surface from which a stage-dependent one-shot erosion has already been subtracted.

7. **Measured: threading a cooling stage without its waterline is a small error, not a large one.** `Fluvial::outlets` reaches `sea_level::derived_sea_level_m(seed)` — the present-anchor waterline, with no stage argument — in three places. Classifying each stage's surface against the present sea instead of its own misclassifies **0.0–0.62 %** of cells across every stage and seed measured (0.24–0.39 % at the cold end). Derived sea moves only ~150 m across the whole chain while relief is several hundred, which is why the error is small. It is nonetheless systematic and located exactly at the coast, which is where a fluvial kernel's base level lives, so it is a correctness item rather than a magnitude one — and naming it small is part of the finding, because the opposite guess is the natural one.

8. **Measured: composing erosion along the chain over-determines the epoch-to-time scale, and the two declared answers differ by three orders of magnitude.** The canonical chain spans 176.5 °C, i.e. 17.7 °C per $0.1$ Ga stage gap; at the declared $\delta = 0.05$ °C/epoch ( `TP_COOLING_PER_EPOCH_C` ) that is **353 erosion epochs per stage gap and 3530 for the canonical chain**. Read as a clock, $\delta$ is worth $2.675\times10^{5}$ years per epoch at the present anchor, against the `ASSUMPTIONS.md` `epoch ↔ years` row's nominal **100** — a factor of $2.675\times10^{3}$. Both rows are declared **arbitrary** today and neither is wrong on its own terms; what the composition removes is the freedom to hold both, because a chain walked in epochs and indexed by $T_p$ has one clock.

9. **What this does not claim.** Not that a cold-stage carve has been run — nothing here builds a tile, and the cost of doing so is unmeasured in seconds. Not that the trajectory's timing is calibrated: the mantle-thermal rate law is declared crude ( #form-isostasy-column FE(8)), so every age label above is order-of-magnitude and the clock arithmetic in FE(8) is arithmetic *on a declared curve*, not a measurement of the world. Not that the implied $2.99\times10^{-6}$ m/yr of freeboard drift is a *rock*-uplift rate — it is a surface rate, and the two coincide only where erosion does no work. Not a claim about level-9 or finer terrain: relief statistics at level 7 are a coarse read of a field whose short-wavelength band grows with level. Not a claim about any seed but these three.

## Epistemic Status

**Max attainable: exact** for every row, in the defeasible sense: each is a deterministic evaluation of pure functions already in the tree, and each is independently falsifiable by re-running the probe — a land fraction that failed to grow, a suture fraction above a few percent, a ledger drop below a tenth of relief, or a mask error above a percent would each refute the clause that reports it.

**Currently `empirical`.** Three seeds at one level and one sampling grain, chosen for continuity with `examples/mantle_cooling_probe` rather than for representativeness. FE(3)'s saturation is read off eleven grid-aligned points, not fitted. FE(6)'s comparison of ledger drop against relief standard deviation compares a mean to a spread, which is a scale comparison and not a decomposition — the ledger's drop is not uniform, and how much of it falls on the cells that carry the relief is unmeasured.

**Probe sensitivity** ( #norm-probe-sensitivity ): the two clauses most at risk of reporting a method artefact are FE(4) and FE(7), and both are guarded by their own null. Suture fraction is zero at the present anchor for two of three seeds and nonzero for the third, so the statistic distinguishes worlds rather than reporting the sampling grid; mask error is **exactly zero** at the present anchor by construction, which is the row that says the metric measures stage separation and not a rounding difference.

Stage `draft`.

## Discussion

The ladder's reason for ranking this work was that the present-anchor world's fragmented drainage is truthful for its age and the realistic-looking world is further down the chain. That holds, and FE(2) is the size of it. The three corrections are worth more than the confirmation, because each redirects the design that would follow.

Saturation (FE(3)) means the work is a **selection within the chain**, not an extension of it — the cheap first build is a single cold stage, and there is no colder frontier to chase afterwards. Thinness (FE(4)) means the payoff is more land under weather, not an orogen to dissect; a design justified by mountain belts would be justified by the smallest of the three effects. And the ledger overlap (FE(6)) is the one that reaches a formulation rather than a build order: a single maturity step of erosion is already applied at every stage, so a fluvial history carried *between* stages is not a new rung beside the ledger but a candidate replacement for it, which is what #form-isostasy-column FE(9) already names as its own open successor.

## Working Notes

- **The clock in FE(8) is over-determined by a third constraint nobody has used.** The kernel's relief is set by the ratio $U/K$ per epoch, so absolute years are interpretive today and no simulated quantity moves when the label changes. That stops being true the moment $K$ is calibrated against real denudation rates — which is exactly what the `ASSUMPTIONS.md` `erosion k_dt, deposition G, talus slope, κ` row defers until `epoch ↔ years` is pinned. So there are three claims on one number and no contradiction *yet*; the composition is what would create one.
- **Not measured, and it decides the first build's cost:** whether a cold-stage sweep is more expensive than the present-anchor sweep. #obs-erosion-residual-is-driver-bound Working Notes record that the kernel runs its full composition on subaerially empty tiles and only the *result* is nil, which predicts cost roughly flat in land fraction — so a cold-stage build should cost about what the present-anchor build cost. That is an inference from an unmeasured premise (cost in seconds was never measured), and one timed sweep would settle it.
- Level-7 relief is a coarse read. FE(2)'s relief column is the quantity most likely to move at level 9, where the short-wavelength band of `gen::bathymetry_m` contributes octaves this sampling does not resolve.
