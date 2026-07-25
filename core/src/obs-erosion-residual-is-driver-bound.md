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

4. **Consequence: the convergence-$\varepsilon$ rung is blocked on erosion too, for a different reason than water.** #obs-water-fill-never-settles blocks water because the timestep is pinned below the scale at which anything settles. Erosion is blocked because the residual is **driver-bound**: with sustained uplift the landscape approaches a *driven* steady state in which incision balances uplift, and mean $\lvert\Delta h\rvert$ per epoch stays near the uplift rate forever rather than falling to zero. A tolerance on $\lvert\Delta h\rvert$ therefore either never fires, or fires immediately on a tile that is merely inert — and the second failure is the dangerous one, because it would report convergence for exactly the tiles that never computed anything.

5. **What the eroding tile shows about the authored count.** Undriven f2 is still falling at epoch 400 ($8.7\times10^{-4}$); at the authored 40 it reads $3.2\times10^{-2}$, some 36× its epoch-400 value. So on tiles that do fluvial work, `epochs = 40` is not a near-converged approximation — it is early in a long decline. The count is wrong in *both* directions at once: over-run on the inert majority, under-run on the working minority.

6. **What this does not claim.** Not that the uplift rate is wrong (it is a declared `arbitrary` stand-in; `ASSUMPTIONS.md` "uplift rate"). Not that a criterion is impossible — only that the *near-stationarity* form is, and the criterion erosion actually needs is a statement about the **erosion–uplift balance**, which is open. Not that inert tiles should simply be skipped without a declaration: skipping is a scheduling decision that has to be declared and convictable, not an optimization applied quietly. Not a measurement of build-time cost — the waste is demonstrated in kernel work, not in seconds, which are unmeasured here.

## Epistemic Status

**Max attainable: exact** for the measurements under `examples/erosion_settle_probe` (deterministic; four footprints; two configurations each). Both halves are independently falsifiable: an undriven residual above zero on the inert footprints refutes FE(2)–(3), and a driven residual falling away from the uplift rate refutes FE(1)/(4).

**Currently `empirical`.** One seed, one level, four footprints chosen for spread rather than representativeness; the ~5–20 % land fraction that makes FE(3) a claim about *most* tiles is read from `#form-isostasy-column`, not re-measured here, so "most" is an inference from two measured things rather than a counted sweep. Stage `draft`.

**Probe sensitivity** ( #norm-probe-sensitivity ): the probe reports trajectories and runs two configurations, because a single driven trajectory at one epoch count is exactly what would have hidden this — the driven residual looks like a healthy steady quantity, and it is the *undriven* zero that says the tile is inert.

## Discussion

This is the second time in two days that building a convergence gate has been the thing that discovered the gate could not be built. The shape is the same as #obs-water-fill-never-settles and the causes are unrelated: there, a clamp pinned the timestep; here, a driver pins the residual. What the two share is that the arbitrary count looked like an under-converged approximation of something, and in neither case was there a something.

The near-miss worth recording is the one this segment nearly became. The obvious reading of the `ASSUMPTIONS.md` **arbitrary** row — *a magic number on the CLI should move onto the nomos declaration and be folded into the key* — is a real defect and a real fix, and it is also, taken alone, a way to make an unjustified number look principled by relocating it into the declaration layer. The count does not become law by being declared. `#form-time-indexed-stage-chains` FE(4) is already precise about this: the *tolerance* is what gets declared and keyed, and the count becomes an output. Where no tolerance is available, the honest declaration is that the number is arbitrary and why — which is what the ledger already said before anyone moved it.

## Working Notes

- **The first honest criterion available here is not a tolerance but a precondition:** a tile with zero subaerial cells is converged at epoch 0, and that is checkable before running anything. It would replace the arbitrary count for the majority class outright, and it is convictable — unlike a tolerance, it can be stated so that a wrong answer is visible. Not built; it belongs with builder admission ( #form-builder-admission ) rather than in the kernel, since it is a scheduling claim.
- **The remaining class needs a balance criterion**, not a stationarity one — something of the form *incision rate matches uplift rate to within a declared fraction, tile-wide*. That is open, and it is the real content of the `erosion run length` **arbitrary** row.
- **Do not conclude that erosion epochs are cheap on inert tiles.** The kernel still runs depression filling, receivers, drainage accumulation, incision, deposition, talus and creep over all 4096 cells every epoch; only the *result* is nil. The cost has not been measured in seconds.
- Sibling: the same driver-bound shape should be checked on the other relaxation rungs before any of them is given a stationarity gate — the pattern, not the kernel, is what generalizes.
