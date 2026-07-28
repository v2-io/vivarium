---
slug: obs-chi-shape-is-erosions-criterion
type: observation
status: empirical
stage: draft
depends:
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - obs-erosion-residual-is-driver-bound
  - form-time-indexed-stage-chains
  - detail-erosion-composition
  - obs-tile-outlets-grade-away-the-basins
---

# Erosion's criterion is a shape test, and the authored count is a quarter of what it needs

A convergence criterion for the fluvial kernel exists that a residual tolerance cannot supply: at steady state channel elevation is linear in $\chi$, and how far a landscape is from that line is measurable, falsifiable, and absent rather than passed on a tile that never eroded. Measured against a real 30-stage settle history, the criterion fires **late** — the beacon patch at one a-priori response time has covered roughly a quarter of its approach.

## Formal Expression

1. **The criterion.** For channelized cells, integrate $\chi$ upstream from base level along the kernel's own D8 tree and fit elevation against it:

   $$\chi_i = \chi_{r(i)} + \left(\frac{A_0}{A_i}\right)^{m/n} d_i, \qquad z \;=\; z(x_b) + \left(\frac{U}{K A_0^{m}}\right)^{1/n}\chi$$

   (Perron & Royden 2013, *ESPL* 38:570–576, Eqs. 6a/6b.) The reported statistic is $\sqrt{1-R^2}$ over each basin's channel cells — the fraction of channel-elevation *amplitude* $\chi$ leaves unexplained — which is dimensionless and therefore comparable across stages whose relief is growing. Instrument: `Fluvial::chi_profile`, consumed by `examples/chi_convergence_probe`.

2. **The kernel's own discrete form, with no fitted parameter.** One epoch adds $U_i$ and then solves $h_i \leftarrow (h_i + f h_{r})/(1+f)$ with $f = k_{dt}A_i^{m}/d_i$ ( #detail-erosion-composition FE(3.5)). Demanding $h$ unchanged across the epoch gives exactly

   $$h_i - h_{r(i)} = \frac{U_i\,d_i}{k_{dt}\,A_i^{m}}$$

   which is the $\chi$ result specialised to this scheme, integrable up from each basin's base level into a predicted profile with **zero free parameters** — and valid under a *spatially varying* $U$, which the literature form is not. $A$ is the kernel's MFD drainage rather than a D8 accumulation, because the identity is exact only in the $A$ the incision step actually consumes.

3. **Measured: the shape residual falls where the pinned residual cannot help.** L13 beacon patch, face 1, origin (640, 5376), 16 tiles of $64^2$ at ~1.22 km/cell, 30 stages at `epochs = 10…300` (`msc/agent-briefs/l13-patch-coords.md`), read under the cohort's own `src=`:

   | surface | $\sqrt{1-R^2}$, channels | trunks only | fitted $\mathrm{d}z/\mathrm{d}\chi$ ÷ pure-SPL | recorded mean $\lvert\Delta h\rvert$ |
   |---|---|---|---|---|
   | uncarved prior | 0.472 | 0.466 | 8.18 | — |
   | stage 10 | 0.439 | 0.440 | 7.92 | 1.115 |
   | stage 100 | 0.307 | 0.308 | 4.94 | 0.940 |
   | stage 300 | 0.340 | 0.161 | 2.62 | 0.475 |

   Medians over the 16 tiles; "trunks" is the same statistic above 100 median cell areas instead of 10. The uncarved prior is the known-bad the criterion must reject and does.

4. **Measured: 300 epochs is roughly a quarter of the approach.** Carrying the most channelized tile past the ladder's end (computed in the probe, never stored) the statistic keeps falling to ~0.15 and the trunk statistic to ~0.06 by **1200–1500 epochs**, then plateaus. So the beacon's authored 300 — one a-priori response time from `Fluvial::response_census` — leaves the patch visibly short: at stage 300 it stands at 0.24 against a converged 0.15 on that tile, and its fitted slope at 1.74 against a converged ~1.6.

5. **The rate half needs an effective $K$, and the offset is the deposition term.** The fitted $\mathrm{d}z/\mathrm{d}\chi$ does **not** converge to the pure incision–uplift prediction $U/(k_{dt}A_0^{m})$; it converges to about **1.6×** it. With deposition switched off the same instrument recovers the prediction to within 15% (`erosion.rs::the_surface_approaches_the_predicted_chi_profile_under_sustained_uplift`), so the offset is the live composition's Davy–Lague term. Read as interpretation rather than measurement: sediment that is re-laid must be re-cut, so a steeper channel is needed to keep pace with the same $U$, and $G = 1$ brackets the steepening in $[1, 1+G]$ — the measured 1.6 sits inside that bracket.

6. **Sensitivity, stated as part of the claim** ( #norm-probe-sensitivity ). Discrimination is *state-dependent*, and improves as the landscape converges. Adding a knickpoint of amplitude $a$ times the channel's elevation standard deviation **to the measured profile**, the smallest one that lifts the statistic clear of the residual the landscape already carries is $a \approx 2$ (≈87 m) at stage 300 and $a \approx 1$ (≈34 m) at 3000 epochs. Below that the criterion is saturated by its own departure and cannot see the step. A landscape twice as steep in $\chi$ passes the shape half at $\sqrt{1-R^2} = 0$ and is caught **only** by the rate half — so the two halves are not redundant.

7. **It is evaluable where a residual tolerance is not.** The rms fit residual is ~27 m at stage 300 and ~10 m converged, against an f32 resolution of ~$1.2\times10^{-4}$ m at kilometre elevations — five orders of margin. The literature's own steady-state metrics run down to $10^{-6}$–$10^{-14}$ m and are unevaluable in this kernel's storage type (`msc/research-lem-sota/lem-time-interior-prior-art-2026-07-28.md` §4.3).

8. **It fails safe.** A tile with no channelized basin produces **no test** rather than a pass. That is the exact inverse of the failure #obs-erosion-residual-is-driver-bound FE(4) names as the dangerous one, where a $\lvert\Delta h\rvert$ tolerance reports convergence for the tiles that never computed anything.

9. **The uniform-$U$ premise was satisfied here, so the departure is not about it.** Over the beacon patch the uplift field this cohort carved against has tile means 0.494–0.515 m/epoch and a within-tile $\sigma/\mu$ of 0.21. The shape residual is therefore not an artifact of spatially varying $U$ on this cohort — which is the only reason FE(3)–(5) can be read as being about the *composition*. This is a property of the cohort and not of the world: the uplift nomos has since become the column's own derivative, signed and coherent at continent wavelength ( #form-isostasy-column FE(7)), and a patch rebuilt under it will not satisfy the premise. FE(2) is the form that survives when it does not, and re-running this probe after that rebuild is what would show whether it does.

10. **What this does not claim.** Not that a threshold has been declared — none has, and picking one is a nomos decision, not a probe's ( #form-time-indexed-stage-chains FE(4)). Not that the criterion is calibrated: its converged value (~0.15 channels, ~0.06 trunks) is a property of the live composition and would move with $G$, $\kappa$, the talus angle, or the level. Not a claim about any cohort but this one: one seed, one patch, one level, 16 tiles. Not that $\chi$-linearity holds for headwaters — it does not, and should not; the statistic improves monotonically with the channelization threshold (0.370 → 0.340 → 0.252 → 0.161 → 0.066 across 3, 10, 30, 100, 300 median cell areas), which is the stream-power law's own domain showing through.

## Epistemic Status

**Max attainable: exact** for FE(2), which is algebra on the incision update and can be checked by reading it; **empirical** for everything measured, which is one cohort at one seed.

**Currently `empirical`.** FE(3)–(6) and (9) are single-cohort measurements from `examples/chi_convergence_probe` against the store, and FE(4)'s extension past the ladder is one tile. FE(5)'s attribution of the 1.6× offset to deposition rests on two configurations (composition on, composition off) rather than on a sweep over $G$, and the $[1,1+G]$ bracket is stated as interpretation. Stage `draft`.

**Falsifiers.** A landscape run far past its response time whose shape residual does not fall refutes FE(3)–(4). A deposition sweep in which the slope ratio does not track $G$ refutes FE(5). An inert tile reported as converged refutes FE(8). A `chi_profile` whose predicted profile is not a near-fixed-point of the incision solve refutes FE(2) — that one is convicted in the lib, not here.

**Probe sensitivity.** FE(6) is the required statement and it is unflattering: on an unconverged landscape this criterion cannot see a knickpoint smaller than about twice the channel's elevation spread. The instrument reports its own discrimination at every call rather than leaving it to be assumed.

## Discussion

The interesting thing is not that a criterion was found. It is that the criterion **disagrees with the count it was meant to justify**, and disagrees in the direction that costs work: the beacon's 300 epochs were chosen as one a-priori response time, and the shape test says one response time buys roughly a quarter of the approach. The a-priori time is derived for pure stream power on a static network; the live composition adds deposition, creep and talus, and reroutes every epoch. Both of the field's own caveats — that the analytic response time is a *minimum* (Gasparini et al. 2024) and that drainage rearrangement dominates transient behaviour (Braun & Willett 2013 §4) — point the same way, and this is what they cost here.

The second thing worth holding is that the two halves of the criterion answer different questions and only one of them is cheap. The shape half needs nothing but the surface. The rate half needs a reference slope, and the reference the literature supplies is the pure-SPL one, which this composition misses by 1.6×. Until that factor is derived rather than measured, the rate half is a *diagnostic of the composition*, not a gate.

## Working Notes

- **The obvious next probe is a $G$ sweep**, because FE(5) is currently an attribution with two points. If the converged slope ratio tracks $1+G$ the composition has a derivable effective $K$ and the rate half becomes a gate; if it does not, the offset is entrained with creep and talus and the rate half stays diagnostic.
- **Per-tile base level is the kernel's own, and measuring against it is deliberate.** Each $64^2$ builder tile treats its whole edge ring as outlets ( #obs-tile-outlets-grade-away-the-basins ), so basins grade to the tiling rather than to a coast, and no path exceeds ~32 cells. Stitching the 4×4 patch into one $256^2$ field would give longer profiles — but the kernel never ran that network, so the fit would test a landscape nobody computed, and re-routing the assembly is separately measured to *manufacture* basins at the seams. What that observation adds to this one is the mechanism behind FE(4): with base level fixed 32 cells away the network is set within ten epochs and only jitters, so the 300 epochs buy incision depth and not network evolution — which is exactly the regime in which a $\chi$ profile relaxes slowly and monotonically toward its line. Whether the criterion behaves the same way on a network still capable of capture is untested and is the more interesting case.
- **The trunk statistic is noisy against basin capture** — the extension run shows it jumping 0.063 → 0.347 → 0.063 across a few hundred epochs on one tile, which is a drainage rearrangement passing through, not instrument noise. A criterion built on it would want either a wider footprint or an explicit tolerance for reorganization events.
- **Not attempted: a declared threshold.** FE(10) says why. The number to declare is roughly "twice the converged value for this composition", and nothing yet derives the converged value.
