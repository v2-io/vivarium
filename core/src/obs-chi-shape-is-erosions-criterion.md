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

3. **Measured: the shape residual falls where the pinned residual cannot help, and it does so under both live drivers.** L13 beacon patch, face 1, origin (640, 5376), 16 tiles of $64^2$ at ~1.22 km/cell, 30 stages at `epochs = 10…300` (`msc/agent-briefs/l13-patch-coords.md`). Four cohorts carry this chain — two under `uplift-2026-07-23a-freeboard` and two under `uplift-2026-07-28a-column-derivative` — and every one of them is measured, each read under its own `src=`, because a silent pick is how a cohort comparison becomes an accident.

   | | prior | stage 10 | stage 300 | trunks, 10 → 300 | $\mathrm{d}z/\mathrm{d}\chi$ ÷ pure-SPL, 10 → 300 | mean $\lvert\Delta h\rvert$, 10 → 300 |
   |---|---|---|---|---|---|---|
   | freeboard driver | 0.472 | 0.439 | 0.340 | 0.435 → 0.161 | 7.9 → 2.5 | 1.115 → 0.475 |
   | column-derivative driver | 0.521 | 0.508 | 0.288 | 0.494 → 0.117 | 4.9 → 2.1 | 1.039 → 0.417 |

   $\sqrt{1-R^2}$, medians over the 16 tiles; "trunks" is the same statistic above 100 median cell areas instead of 10. The uncarved prior is the known-bad the criterion must reject and does, under both drivers. The zero-parameter form of FE(2) falls more consistently than the literature form on the same data — 25 of 29 stage steps against 20 under the current driver — because it fits nothing and so inherits no fit noise.

4. **Measured: 300 epochs is roughly a quarter of the approach.** Carrying the most channelized tile past the ladder's end (computed in the probe, never stored) the statistic keeps falling to ~0.13 and the trunk statistic to ~0.06 by **750–1500 epochs**, then plateaus and jitters. So the beacon's authored 300 — one a-priori response time from `Fluvial::response_census` — leaves the patch visibly short, and the shortfall survived the driver change: under the column-derivative driver stage 300 stands at 0.216 on that tile against a converged ~0.13, with its slope ratio at 1.67 against a converged ~1.4.

5. **The rate half's offset is the deposition term, and switching deposition off makes the criterion exact.** Settling six tiles for 3000 epochs from the same stored stage with only `deposition` changed:

   | $G$ | $\sqrt{1-R^2}$ | fitted ÷ pure-SPL | ÷ $(1+G)$ | per-tile spread |
   |---|---|---|---|---|
   | 0 | **0.009** | **1.00** | 1.000 | 1.00–1.00 |
   | 0.25 | 0.088 | 1.14 | 0.914 | 1.11–1.20 |
   | 0.5 | 0.131 | 1.32 | 0.877 | 1.14–1.41 |
   | 1 (live) | 0.177 | 1.55 | 0.773 | 1.38–1.73 |
   | 2 | 0.155 | 2.25 | 0.751 | 1.72–2.49 |

   At $G = 0$ the criterion is **exact on real terrain**: the shape residual collapses to 0.009 and the fitted slope recovers $U/(k_{dt}A_0^{m})$ to three digits. So neither the criterion nor the instrument carries the residual floor — **deposition does**, and every number in FE(3)–(4) is a measurement of the live composition rather than a limit of the method.

6. **But the offset does not track $(1+G)$, so the rate half stays a diagnostic.** The Davy–Lague reading — sediment re-laid must be re-cut, so the steepening should be $1+G$ — is refuted by the fourth column above, which declines monotonically from 1.00 to 0.75 instead of holding flat. The offset is monotone and tight in $G$ (per-tile spreads do not overlap between rows), so it is *calibratable*; it is not *derived*, and until it is, comparing the fitted slope to a reference is a measurement of the composition and not a gate on the landscape. Dividing instead by $(1+G/2)$ holds within 1.00–1.13 across the same rows — recorded as an **observed regularity with no derivation behind it**, whose candidate mechanism is that only part of the eroded volume is re-laid inside the network while the rest reaches an outlet and is lost to the sea. That mechanism is unverified and the regularity is not a law.

7. **Sensitivity, stated as part of the claim** ( #norm-probe-sensitivity ). Discrimination is *state-dependent*, and improves as the landscape converges. Adding a knickpoint of amplitude $a$ times the channel's elevation standard deviation **to the measured profile**, the smallest one that lifts the statistic clear of the residual the landscape already carries is $a \approx 1$ (≈54 m) at stage 300 under the current driver — and $a \approx 2$ under the previous one, which is the same landscape less far along. Below that the criterion is saturated by its own departure and cannot see the step. A landscape twice as steep in $\chi$ passes the shape half at $\sqrt{1-R^2} = 0$ and is caught **only** by the rate half — so the two halves are not redundant, and FE(6) is why only one of them is currently usable as a gate.

8. **It is evaluable where a residual tolerance is not.** The rms fit residual is ~24 m at stage 300, against an f32 resolution of ~$1.2\times10^{-4}$ m at kilometre elevations — five orders of margin. The literature's own steady-state metrics run down to $10^{-6}$–$10^{-14}$ m and are unevaluable in this kernel's storage type (`msc/research-lem-sota/lem-time-interior-prior-art-2026-07-28.md` §4.3).

9. **It fails safe.** A tile with no channelized basin produces **no test** rather than a pass. That is the exact inverse of the failure #obs-erosion-residual-is-driver-bound FE(4) names as the dangerous one, where a $\lvert\Delta h\rvert$ tolerance reports convergence for the tiles that never computed anything.

10. **Measured: the per-cell form does buy what its algebra says — and the current tiling cannot show it.** The fair head-to-head is the **matched-parameter** form: regress $z$ on the predicted profile with a free intercept and scale, two fitted parameters, exactly as many as the literature form, the scale absorbing FE(5)'s offset. Under a spatially invariant $U$ the predicted profile is an affine function of $\chi$ and the two are the *same fit*, so the ratio between their residuals is the discrimination.

    | footprint | driver $\sigma/\lvert\mu\rvert$ within a fitted tile | $\max U/\min U$ **along a fitted channel** | literature ÷ matched-parameter |
    |---|---|---|---|
    | L13 beacon patch, 313 km | 0.002 | 1.001 | 1.000 |
    | L9 sweep, 1250 km tiles | 0.242 | 1.015 | 0.989 |
    | L9 **whole face**, coast-only base level | — | 1.02 → 2.02 | **1.00 → 1.52** |

    The L9 row is the finding. The driver varies **24% across a tile** and only **1.5% along a channel**, because no basin can exceed one tile ( #obs-tile-outlets-grade-away-the-basins ) and within a tile the basins are a small fraction of it. Two facts compose into one blind spot and neither alone would cause it: a driver coherent at continent wavelength, and an outlet policy that caps every basin far below that wavelength. A channel therefore never crosses a $U$ gradient, and the two forms are not merely indistinguishable — they are the same fit.

11. **Measured: remove the cause and the discrimination appears.** A tile covering a whole cube face is the one configuration `Fluvial::outlets` gives coast-only base levels, so its basins can run continental distances. Assembling the landiest L9 face from its 64 stored tiles and running the kernel on it as one field: at epoch 0, along-channel $\max U/\min U$ is 1.02 and the ratio is **1.003** — the two forms coincide exactly where the algebra says they must, which is the null check. As the network integrates and basins begin to cross the gradient (span rising to 2.0) the ratio rises to **1.31–1.52**, replicated at 1.44–1.89 on a second face. So the per-cell form explains ~1.4× more of the channel profile's amplitude than $\chi$ does, once a channel exists that can tell them apart.

    **This is a controlled experiment about the criterion, not a claim about the world.** The builder never ran this network; assembling stored tiles is separately measured to manufacture basins at the seams; it is computed in memory and never stored. What it establishes is that FE(2) is worth its complexity, and that **the precondition for using it is a tile boundary that is not a base level** — which is `#form-seam-flux-exchange` / `#form-face-flux-register`, not anything in this segment.

11. **What this does not claim.** Not that a threshold has been declared — none has, and picking one is a nomos decision, not a probe's ( #form-time-indexed-stage-chains FE(4)). Not that the criterion is calibrated: FE(5) shows its converged value is set by $G$, and it would also move with $\kappa$, the talus angle, or the level. Not that the rate half is a gate — FE(6). Not a claim about any seed but this one. Not that the whole-face numbers describe any world the builder has built — FE(11) says why. Not that $\chi$-linearity holds for headwaters — it does not, and should not; the statistic improves monotonically with the channelization threshold (0.376 → 0.288 → 0.221 → 0.116 → 0.080 across 3, 10, 30, 100, 300 median cell areas), which is the stream-power law's own domain showing through.

## Epistemic Status

**Max attainable: exact** for FE(2), which is algebra on the incision update and can be checked by reading it, and for FE(5)'s $G = 0$ row, which is a controlled measurement with an analytically known answer that it recovers to three digits; **empirical** for the rest, which is one seed and one patch.

**Currently `empirical`.** FE(3) is four cohorts under two drivers, which is replication across source trees and drivers but not across seeds. FE(4)'s extension past the ladder is one tile per cohort. FE(5)–(6) are six tiles at one epoch count; the $(1+G/2)$ regularity is five points with no derivation and is labelled as such in the segment and in the probe's own output. FE(10) spans three footprints from 313 km to a whole face; FE(11) is two faces of one world at one seed, on assembled networks the builder never ran. Stage `draft`.

**Falsifiers.** A landscape run far past its response time whose shape residual does not fall refutes FE(3)–(4). A $G = 0$ configuration whose fitted slope does not recover $U/(k_{dt}A_0^{m})$ refutes FE(5) and would also convict the instrument. An inert tile reported as converged refutes FE(9). A whole-face network whose channels cross a $U$ gradient and on which the two forms still agree refutes FE(11) — and a network whose channels do *not* cross one on which they disagree would convict the instrument, which is what the epoch-0 null check exists to catch. A `chi_profile` whose predicted profile is not a near-fixed-point of the incision solve refutes FE(2) — that one is convicted in the lib, not here.

**Probe sensitivity.** FE(7) is the required statement and it is unflattering: on an unconverged landscape this criterion cannot see a knickpoint smaller than about the channel's own elevation spread. The instrument reports its own discrimination at every call rather than leaving it to be assumed, and FE(5) says where the saturation comes from — at $G = 0$ it is not there.

## Discussion

The interesting thing is not that a criterion was found. It is that the criterion **disagrees with the count it was meant to justify**, and disagrees in the direction that costs work: the beacon's 300 epochs were chosen as one a-priori response time, and the shape test says one response time buys roughly a quarter of the approach. The a-priori time is derived for pure stream power on a static network; the live composition adds deposition, creep and talus, and reroutes every epoch. Both of the field's own caveats — that the analytic response time is a *minimum* (Gasparini et al. 2024) and that drainage rearrangement dominates transient behaviour (Braun & Willett 2013 §4) — point the same way, and this is what they cost here.

The second thing worth holding is what the deposition sweep did to the reading. Before it, the residual floor looked like a property of the criterion — something χ-linearity could not resolve past. It is not: at $G = 0$ the criterion is exact on real terrain, and the whole floor is one operator in the composition. That reframes the open question from *"how good can this criterion get"* to *"what is the shape of a driven steady state under detachment **plus deposition**"*, which is a physics question with a literature, not an instrument limitation.

The third is a negative worth stating plainly, because it is the discrimination this segment was built to demonstrate and it did not get its test. FE(2) exists to survive a spatially varying $U$. The new column-derivative driver was expected to supply one; measured, it is *smoother* over this patch than the field it replaced ($\sigma/\lvert\mu\rvert$ 0.002 against 0.21), because it is coherent at continent wavelength and the patch is 313 km across. The premise was not stressed, so the two forms were never asked to disagree. That is a fact about the footprint, and the fix is a footprint, not a driver.

## Working Notes

- **The $G$ sweep is done and it came back negative** (FE(6)): the offset is monotone and tight in $G$ but does not track $1+G$, so the rate half did not become a gate. What would make it one is a derivation of the steepening for detachment-plus-deposition under this deposit rule — including the fraction of sediment that leaves the network at an outlet, which is the candidate mechanism behind the $(1+G/2)$ regularity and is unverified. **Do not re-run the sweep expecting a different answer**; the per-tile spreads do not overlap between rows, so the shape of that curve is not noise.
- **The varying-$U$ discrimination is measured and the blocker is named** (FE(10)–(11)). What is *not* done: the whole-face experiment runs on an assembled network, so the honest next step is a builder that can carve a footprint whose basins are not capped — which is the seam work, not this segment's. Nothing here should be read as a case for assembling tiles at build time; the assembly is an instrument, and `#obs-tile-outlets-grade-away-the-basins` measures what it costs.
- **The median along-channel $U$ span is not the right companion to an SS-pooled ratio** and the whole-face table shows it: at 500 epochs the median basin sees a span of 1.029 while the pooled ratio is already 1.42, because the pooling is amplitude-weighted and the large basins that cross gradients dominate it. The columns are still worth reading together — they move together over the run — but a future tightening should weight the span the same way the residual is weighted.
- **Per-tile base level is the kernel's own, and measuring against it is deliberate.** Each $64^2$ builder tile treats its whole edge ring as outlets ( #obs-tile-outlets-grade-away-the-basins ), so basins grade to the tiling rather than to a coast, and no path exceeds ~32 cells. Stitching the 4×4 patch into one $256^2$ field would give longer profiles — but the kernel never ran that network, so the fit would test a landscape nobody computed, and re-routing the assembly is separately measured to *manufacture* basins at the seams. What that observation adds to this one is the mechanism behind FE(4): with base level fixed 32 cells away the network is set within ten epochs and only jitters, so the 300 epochs buy incision depth and not network evolution — which is exactly the regime in which a $\chi$ profile relaxes slowly and monotonically toward its line. Whether the criterion behaves the same way on a network still capable of capture is untested and is the more interesting case.
- **The trunk statistic is noisy against basin capture** — the extension run shows it jumping 0.063 → 0.347 → 0.063 across a few hundred epochs on one tile, which is a drainage rearrangement passing through, not instrument noise. A criterion built on it would want either a wider footprint or an explicit tolerance for reorganization events.
- **Not attempted: a declared threshold.** FE(10) says why. The number to declare is roughly "twice the converged value for this composition", and nothing yet derives the converged value.
