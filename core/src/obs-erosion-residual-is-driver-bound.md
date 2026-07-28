---
slug: obs-erosion-residual-is-driver-bound
type: observation
status: empirical
stage: draft
depends:
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - form-time-indexed-stage-chains
  - obs-water-fill-never-settles
  - detail-erosion-composition
---

# Erosion's residual is pinned by its driver, and most tiles do no fluvial work at all

A near-stationarity criterion cannot replace erosion's authored epoch count either. Sustained uplift gives the kernel no zero-residual equilibrium to detect, so the per-epoch residual sits at the uplift rate indefinitely; and on a mostly-ocean world the majority of swept tiles are **subaerially empty**, where the kernel does exactly nothing and the epochs buy only accumulated uplift.

## Formal Expression

1. **Measured: the driven residual is the uplift rate.** Mean per-cell $\lvert\Delta h\rvert$ per epoch, level 9, $64^2$ tiles, four footprints, seed `0xf1d242b21d8d89ea` (`examples/erosion_settle_probe`), run in the builder's own kernel configuration (uplift and precipitation weight supplied as `query::World::erosion_tile` supplies them):

   | footprint | land cells | epoch 1 | epoch 40 | epoch 400 | mean uplift (m/epoch) |
   |---|---|---|---|---|---|
   | f0 (0,0) | 0 / 4096 | $4.704\times10^{-1}$ | $4.704\times10^{-1}$ | $4.704\times10^{-1}$ | 0.501 |
   | f2 (64,64) | 3911 / 4096 | $4.933\times10^{-1}$ | $4.808\times10^{-1}$ | $4.380\times10^{-1}$ | 0.498 |
   | f3 (128,0) | 5 / 4096 | $4.703\times10^{-1}$ | $4.703\times10^{-1}$ | $4.703\times10^{-1}$ | 0.500 |
   | f5 (64,128) | 5 / 4096 | $4.691\times10^{-1}$ | $4.691\times10^{-1}$ | $4.691\times10^{-1}$ | 0.500 |

   On three of four footprints the residual is **constant to four significant figures for 400 epochs** and equals the mean uplift rate.

2. **The discriminator: with uplift zeroed, those same tiles change by exactly nothing.** The probe runs each footprint twice. Undriven, f0/f3/f5 report $\lvert\Delta h\rvert = 0$ at *every* epoch — not small, zero — while f2 reports $5.5\times10^{-2}$ falling to $8.7\times10^{-4}$ by epoch 400. Neither configuration alone distinguishes "eroding in exact balance with uplift" from "not eroding"; the pair does. These tiles are not eroding.

3. **Inertness tracks submergence, and that is the kernel behaving correctly.** The three inert footprints hold 0, 5, and 5 subaerial cells of 4096; the one that erodes holds 3911. Fluvial incision is a subaerial process, so a tile with no land above sea level having no fluvial work is right. What follows is not a kernel defect but a **scheduling** fact: a whole-world sweep at an Abyssal land fraction (~5–20 %, `#form-isostasy-column`) spends its authored epochs mostly on tiles where every epoch is a no-op plus one uplift addition.

4. **Consequence: the convergence-$\varepsilon$ rung is blocked on erosion too, for a different reason than water.** #obs-water-fill-never-settles blocks water because the timestep is pinned below the scale at which anything settles. Erosion is blocked because the residual is **driver-bound**: with sustained uplift there is no zero-residual equilibrium for a tolerance to detect, and the two classes of tile straddle any value one might declare.

   On a tile that does fluvial work the residual decays toward a **nonzero floor set by the landscape, not by the driver** — on the L13 beacon patch it falls from 1.115 to 0.475 m/epoch across 300 epochs against a mean uplift of 0.50, and plateaus near 0.18 m/epoch ($\approx 0.35\,U$) when one tile is carried to 3000 ( #obs-chi-shape-is-erosions-criterion FE(3)–(4)). On a tile that does nothing the residual is the uplift rate **exactly**, forever. So a declared tolerance has to sit below a floor nobody can compute in advance, and above nothing — because the value an inert tile reports is a value a driven tile passes straight through on its way down. A tolerance either never fires, or fires on a tile that is merely inert, and the second failure is the dangerous one: it reports convergence for exactly the tiles that never computed anything.

5. **What the eroding tile shows about the authored count.** Undriven f2 is still falling at epoch 400 ($8.7\times10^{-4}$); at the authored 40 it reads $3.2\times10^{-2}$, some 36× its epoch-400 value. So on tiles that do fluvial work, `epochs = 40` is not a near-converged approximation — it is early in a long decline. The count is wrong in *both* directions at once: over-run on the inert majority, under-run on the working minority.

6. **What this does not claim.** Not that the uplift rate is wrong (it is a declared `arbitrary` stand-in; `ASSUMPTIONS.md` "uplift rate"). Not that a criterion is impossible — only that the *near-stationarity* form is. The criterion erosion actually needs is a statement about the **erosion–uplift balance**, and its **shape** form is now built and measured against a real settle history ( #obs-chi-shape-is-erosions-criterion ): $\chi$-linearity falls as the landscape settles, is evaluable five orders above the f32 floor, and produces *no test* rather than a pass on an inert tile. No threshold has been declared, so nothing is yet gated on it. Not that inert tiles should simply be skipped without a declaration: skipping is a scheduling decision that has to be declared and convictable, not an optimization applied quietly. Not a measurement of build-time cost — the waste is demonstrated in kernel work, not in seconds, which are unmeasured here.

## Epistemic Status

**Max attainable: exact** for the measurements under `examples/erosion_settle_probe` (deterministic; four footprints; two configurations each). Both halves are independently falsifiable: an undriven residual above zero on the inert footprints refutes FE(2)–(3), and a driven residual falling away from the uplift rate refutes FE(1)/(4).

**Currently `empirical`.** One seed, one level, four footprints chosen for spread rather than representativeness; the ~5–20 % land fraction that makes FE(3) a claim about *most* tiles is read from `#form-isostasy-column`, not re-measured here, so "most" is an inference from two measured things rather than a counted sweep. Stage `draft`.

**Probe sensitivity** ( #norm-probe-sensitivity ): the probe reports trajectories and runs two configurations, because a single driven trajectory at one epoch count is exactly what would have hidden this — the driven residual looks like a healthy steady quantity, and it is the *undriven* zero that says the tile is inert.

## Discussion

This is the second time in two days that building a convergence gate has been the thing that discovered the gate could not be built. The shape is the same as #obs-water-fill-never-settles and the causes are unrelated: there, a clamp pinned the timestep; here, a driver pins the residual. What the two share is that the arbitrary count looked like an under-converged approximation of something, and in neither case was there a something.

The near-miss worth recording is the one this segment nearly became. The obvious reading of the `ASSUMPTIONS.md` **arbitrary** row — *a magic number on the CLI should move onto the nomos declaration and be folded into the key* — is a real defect and a real fix, and it is also, taken alone, a way to make an unjustified number look principled by relocating it into the declaration layer. The count does not become law by being declared. `#form-time-indexed-stage-chains` FE(4) is already precise about this: the *tolerance* is what gets declared and keyed, and the count becomes an output. Where no tolerance is available, the honest declaration is that the number is arbitrary and why — which is what the ledger already said before anyone moved it.

## Working Notes

- **The first honest criterion available here is not a tolerance but a precondition:** a tile with zero subaerial cells is converged at epoch 0, and that is checkable before running anything. It would replace the arbitrary count for the majority class outright, and it is convictable — unlike a tolerance, it can be stated so that a wrong answer is visible. Not built; it belongs with builder admission ( #form-builder-admission ) rather than in the kernel, since it is a scheduling claim.
- **The remaining class has a balance criterion now, and it is a shape statement rather than a stationarity one** — `#obs-chi-shape-is-erosions-criterion`, built as `Fluvial::chi_profile` / `examples/chi_convergence_probe`. What is still open is the *threshold*: its converged value is a property of the live composition, and nothing yet derives it. The count question the criterion guards has a second and independent answer in the **a-priori analytical response time**, which at $n=1$ is computable from static inputs with no calibration ($T_A$ independent of uplift — Whipple & Tucker 1999, via the 2026-07-28 sidebar in `msc/research-lem-sota/`). `Fluvial::response_census` (`examples/response_time_probe`) is the instrument for it: on the live world's landiest L9 tiles the wave crosses the channelized network in ~20–70 epochs (so the authored 40 is one channel-crossing, and the still-falling 400-epoch residual of FE(5) is the *hillslope* clock, a different and slower one); candidate L13 patches measure ~300 epochs max with per-epoch Courant ≈ 1 at trunks. **The two answers disagree, and the shape test is the stricter:** one $T_A$ leaves the L13 patch roughly a quarter of the way to $\chi$-linearity. $T_A$ is derived for pure stream power on a static network; the live composition is neither.
- **Do not conclude that erosion epochs are cheap on inert tiles.** The kernel still runs depression filling, receivers, drainage accumulation, incision, deposition, talus and creep over all 4096 cells every epoch; only the *result* is nil. The cost has not been measured in seconds.
- Sibling: the same driver-bound shape should be checked on the other relaxation rungs before any of them is given a stationarity gate — the pattern, not the kernel, is what generalizes.
