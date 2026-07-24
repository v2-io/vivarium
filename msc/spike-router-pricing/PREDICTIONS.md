# Predictions — the routing pricing experiment, written BEFORE the first number

*Written before `examples/router_pricing` was run even once, imitating
`msc/spike-curl-probe/PREDICTIONS.md`. A confirming number must survive a
prediction it could have broken. The project has twice believed a flattering
number that confirmed a prior (`DECISIONS[seam-probes-were-measuring-seabed]`,
the "22888" divide-by-zero, the first non-deterministic curl quadrature).*

## What the experiment is (and is NOT)

`DECISIONS[routing-violates-the-potential-identity-and-the-replacement-does-not-fix-it]`
(`#obs-routing-curl-spiral`) measured a level-independent ~2% topological spiral
in routed flux and **quarantined the severity verdict** behind one experiment:

> run an eroded landscape with and without the corrected router and show
> whether the channel network differs — under identical forcing/seed.
> "Everything above is the structure of the error, not its landscape
> consequence. Do not act on severity until that runs."

This spike **prices** that. It does **not** adopt a router successor
(`#form-grid-equiangular-staggered` FE(4) stays open). A genuine *no meaningful
difference* is a first-class result that would honestly de-prioritise router work.

## The design

Two erosion runs, identical in **everything but the drainage router**:

- **Arm L (live):** the drainage accumulation exactly as `erosion.rs`
  `accumulate_drainage` runs today — 8-neighbour Moore fan, weights
  `(drop/dist)^p`, `p = 1.0`, **uniform** lengths (`cell_m`, diagonal `√2·cell_m`).
- **Arm C (corrected):** the same fan **with the named topological defect
  removed** — I build this in the harness from public `measure` geometry:
  1. **C-len:** true great-circle neighbour lengths (`measure::neighbor_center_dist_m`)
     in place of uniform `cell_m`/`√2·cell_m` (the *length* half of the cube-lock).
  2. **C-edge:** additionally **kill the diagonals** — restrict the fan to the four
     face-sharing neighbours (the diagonals cross no face: ~47.8% phantom flux,
     `#obs-cube-locked-kernel-bias` FE(2)), with true edge lengths.
  3. **C-moment (stronger, if C-edge is null):** 8-neighbour with **true bearings**
     and a first-moment (`m1`) correction about the reconstructed gradient azimuth —
     the `router_lab::flow::moment_correct` idea reimplemented on the flat grid.

Everything downstream (Priority-Flood, D8 receivers, implicit stream-power incision,
Davy–Lague deposition, talus, creep, conservation) is **byte-identical** between arms.
Same seed, same footprint, same uplift, same epochs, same `FluvialParams`.

**Faithfulness anchor (the regression guard that makes this convict anything):**
Arm L, reimplemented in the harness, must reproduce the live
`erosion::Fluvial::erode` final `h` and `drainage` **bit-for-bit** on the same
seed/footprint/params. If it does not, the harness is not the live world and no
comparison it reports is about the live world. This is checked first, printed, and
gates the run.

**Off-symmetry footprint (the null-test trap, `#norm-probe-sensitivity`):** a
face-centred D4-symmetric terrain *acquits* a defective router
(`#obs-routing-curl-spiral` FE(3): measured κ = −1.9e−10 at the face centre while a
5.9° pointwise bias sits right there). So the terrain is placed **off face-centre**,
toward a corner where the equiangular Jacobian shear (hence the fan's bearing
distortion) is largest. Reported alongside: the same run **at the face centre** as a
control that *should* show near-zero arm-difference even if off-centre shows a large one.

## The metrics (a channel network "differs" — measured three ways)

Computed on the final eroded fields, arm L vs each corrected arm:

- **M1 — channel-mask disagreement (Jaccard).** Channel = cell whose drainage
  `A > τ·(local cell area)`, τ swept (e.g. 20, 50, 100). Report the fraction of
  channel cells that flip between arms (symmetric-difference / union). This is the
  bluntest "do the rivers go elsewhere" measure.
- **M2 — drainage-field divergence.** Spearman rank-correlation and relative L2 of
  `log(A_L)` vs `log(A_C)` over all land cells. High correlation ⇒ same network,
  reshaped only in magnitude; low ⇒ genuinely different accumulation.
- **M3 — channel-orientation anisotropy (the mechanistically predicted signal).**
  Histogram of channel-segment bearings (steepest-descent step direction on channel
  cells). The bias theory predicts arm L over-populates the **grid axes** (0°/90°)
  and depletes the **45° diagonal repeller**; a corrected arm should *reduce* that
  grid-axis excess. Metric: excess mass within ±ε of {0,90,180,270}° minus expected,
  and the axis/diagonal ratio. **This is the one that could show the defect is
  cosmetic to elevation (M1/M2 small) yet real to river *azimuth* (M3 large) — the
  case the elevation-only metrics would miss.**

Each metric reported **off-centre and at the control centre**, and per corrected arm.

## Predictions

- **P0 — Arm L reproduces live `erosion.rs` bit-for-bit.** If not, stop; the harness
  is wrong. (This is a theorem about my own code, not a finding.)

- **P1 — the marginal landscape effect of the correction is MODEST, quite possibly
  null-grade, at the elevation level (M1, M2).** Honest prior and the shape of the
  negative I am most prepared to report: `p = 1.0` is **already live**, and at `p=1`
  the fan's first moment is *exactly zero on a square lattice* (a theorem,
  `#obs-routing-curl-spiral` FE(6a)) — the largest, cheapest bias fix already shipped.
  The residuals the corrected arm removes (true lengths; diagonal phantom flux) are
  second-order relative to that. So I expect M2 rank-correlation **high** (≳0.9) and
  M1 channel-flip **modest** (order 10–25% at moderate τ, concentrated at channel
  *margins* where a cell is borderline, not at trunk rivers). If M1/M2 come out
  *small*, that is a **real, publishable de-prioritisation** of router work, and I
  will say so plainly — the severity is priced *low*.

- **P2 — the signal, if anywhere, is in M3 (azimuth), off-centre, strongest near the
  corner.** The defect is directional/topological, so its landscape fingerprint is
  channel *orientation*, not channel *elevation*. I predict arm L shows a measurable
  grid-axis excess off-centre that the corrected arms reduce, while at the face-centre
  control both arms look the same (the null-test trap made visible). If M3 also shows
  no arm-difference off-centre, the severity is priced low on **all** axes — the
  strongest possible de-prioritisation, and the most surprising outcome to me.

- **P3 — C-edge (kill diagonals) moves the landscape MORE than C-len (true lengths).**
  Removing 47.8% phantom flux changes *which* cells accumulate; retuning lengths only
  reweights an existing split. If instead C-len dominates, the length bias was the
  larger landscape lever — a finding either way.

- **P4 — determinism: every metric is bit-identical across 3 reruns.** Any drift is a
  bug in the harness (HashMap order, NaN, uninit) and IS the finding — re-run 3× before
  believing anything (the curl-probe self-catch, `#obs-routing-curl-spiral` Working Notes).

## The traps I am most likely to fall into

1. **Believing a flattering M3 that is my own binning artifact.** Guard: M3 must be
   computed identically for both arms from the same channel mask, and the face-centre
   control must show ~0 arm-difference *structurally* (D4 symmetry). If the centre
   control shows a large arm-difference, the metric — not the router — is broken.
2. **Confounding the router with an incidental pipeline difference.** Guard: P0's
   bit-match. If arm L ≠ live, every other number is void.
3. **Declaring "difference!" from a metric that cannot come out null.** Each metric is
   defined so that "no meaningful difference" is a reachable value. A metric that
   always fires is measuring the harness, not the world.
4. **Over-reading a corner-only signal as global severity.** The bias is level-independent
   but *spatially* strongest at corners; a per-region result is reported as such, not
   extrapolated to "all rivers on the planet are grid-locked."

## The shape of the honest negative I am prepared to report

If M1 is modest, M2 correlation high, and M3 shows no arm-difference the face-centre
control doesn't also show — then **the corrected router does not meaningfully change the
channel network**, the ~2% spiral is real in the flux field but **cosmetic to landscape
consequence at these scales/epochs**, and router-successor work (`#form-...` FE(4)) is
**honestly de-prioritised**. That result is exactly as valuable as its opposite and I
will report it without hedging toward "but it still might matter."
