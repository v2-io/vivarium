---
slug: obs-water-fill-never-settles
type: observation
status: empirical
stage: draft
depends:
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - form-time-indexed-stage-chains
  - form-fidelity-invariant
---

# The water tile's fixed fill is forty seconds of world time, and its residual grows

The live `water-tile` settle does not approach stationarity and stops early — it never approaches stationarity at all. The timestep is pinned three orders of magnitude below the CFL limit by a clamp calibrated for metre-scale cells, so 200 steps buys **40 seconds** of world time on kilometre cells, at every level.

## Formal Expression

1. **Measured: the residual grows.** Mean per-cell $\lvert\Delta h\rvert$ per step, level 9, $64^2$ tiles, two footprints (`examples/water_settle_probe`):

   | step | 2 | 50 | 100 | 200 | 400 | 600 |
   |---|---|---|---|---|---|---|
   | f2-corner (m) | $5.3\times10^{-4}$ | $1.10\times10^{-2}$ | $2.19\times10^{-2}$ | $4.34\times10^{-2}$ | $8.51\times10^{-2}$ | $1.24\times10^{-1}$ |
   | f2-mid (m) | $3.5\times10^{-4}$ | $8.9\times10^{-3}$ | $1.77\times10^{-2}$ | $3.52\times10^{-2}$ | $6.92\times10^{-2}$ | $1.01\times10^{-1}$ |

   Monotone increasing, roughly linear in step count, on both footprints. Total water is constant to five figures throughout, so this is redistribution accelerating, not inventory drift. **A settle would fall toward zero.**

2. **The clamp is the binding constraint, and `stable_dt` is inert at tile scale.** `WaterSim::stable_dt` computes $0.3\,\ell/\sqrt{g\,d_{\max}}$ and then clamps to $[0.005, 0.2]$. At builder tile scale the CFL value is far above the ceiling, so the clamp — not the physics — sets the step:

   | level | cell (m) | CFL $\mathrm{d}t$ at $d{=}100$ m | returned | 200 steps |
   |---|---|---|---|---|
   | 7 | 78 184 | 749 s | 0.200 s | 40 s |
   | 9 | 19 546 | 187 s | 0.200 s | 40 s |
   | 11 | 4 886 | 46.8 s | 0.200 s | 40 s |
   | 13 | 1 222 | 11.7 s | 0.200 s | 40 s |

   At level 9 the CFL limit allows **937×** the returned step. The ceiling of $0.2$ s is the CFL step for cells of roughly **2 m** — the scale the fine testbench works at, carried into a path whose cells are kilometres.

3. **Consequence for the fixed count.** "200 steps" is not an under-converged approximation of an equilibrium. It is a fixed distance into a transient that has barely begun, and the distance is **level-independent** — refining the grid does not buy a longer settle, because the clamp does not move.

4. **Consequence for the convergence-$\varepsilon$ rung.** A near-stationarity criterion cannot simply replace the count here ( #form-time-indexed-stage-chains FE(4)): there is no stationarity to detect at this step size, so any tolerance either fires immediately on the early small residual or never fires. The criterion rung on water is **blocked** on the step-size question, which is a physics decision, not a gate design.

5. **What this does not claim.** Not that the clamp is wrong — it plausibly protects the fine path, and its upper bound may be load-bearing somewhere this probe does not reach. Not that raising it is safe: larger steps change what the kernel's other declared structures (well-balancedness, positivity, the $\theta$ term) are doing, and #form-declared-structure-tradeoff binds any such change. Not that the resulting water field is *useless* — it is a wetted bed that looks plausible; the claim is only that it is not a settled one and must not be described as one.

## Epistemic Status

**Max attainable: exact** for the measurements under `examples/water_settle_probe` (deterministic; two footprints; both halves independently falsifiable — a falling residual refutes FE(1), a `stable_dt` near the CFL limit refutes FE(2)).

**Currently `empirical`.** Measured 2026-07-24 at level 9 on two tiles; not swept across levels, seeds, or footprints, and the linear-in-$n$ shape is a reading of six samples per tile rather than a fitted law. Stage `draft`.

**Probe sensitivity** ( #norm-probe-sensitivity ): the probe reports the residual *trajectory*, not a single green — a pass/fail against one tolerance at one step count is exactly what hid this, since the early residual is genuinely small and a coarse gate would have read that as convergence.

## Discussion

The instruments were honest and still missed it. Every probe, example, spike and unit test in the tree steps water through `stable_dt`; `query::water_tile` alone uses the raw default. That difference turns out not to matter — `stable_dt` returns the clamp ceiling here anyway — which is the more interesting fact: the CFL helper reads as a safety discipline while being a constant function at the scale the world is actually built at. A helper that cannot vary is not a check.

The reason this surfaced now is that a convergence gate was being added, and building the gate required asking what the thing converges to. That is the ordinary shape: the instrument you build to make a claim honest is what finds out the claim was mis-stated.

## Working Notes

- **`ASSUMPTIONS.md` "water fill steps" corrected** from "bounded fill, not converged" to what was measured. The old wording implied an equilibrium being approached.
- **Blocked, and on whose desk:** whether the clamp ceiling should scale with cell size (or whether the tile path should take a level-appropriate step at all) is a physics call with structure-preservation consequences — Joseph's or a spike's, not a session's.
- **Do not** add a near-stationarity gate to water before the step-size question is answered; it would fire on the early small residual and certify a 40-second transient as settled. That is the trap this observation exists to mark.
- **Untested and cheap next:** whether the residual growth is transient start-up (flow accelerating down slopes toward a later equilibrium) or unbounded. Six hundred steps did not reveal a turn; a longer run at a larger step would discriminate.
- Sibling: the analytic hydrological init named at #detail-abyssal-parity-build FE(4) phase 2 solves the equilibrium instead of marching to it, and would sidestep this entirely.
- **This is no longer what stands between the world and standing water.** The spill-level field ( #obs-connectivity-fills-the-basins-the-threshold-drained ) is the wet-limit equilibrium, is a pure function of the stored bed, and needs no steps — so lakes do not wait on the step-size question. What still waits on it: transients, flood propagation, currents, and anything where the *approach* to equilibrium is the subject.
- **The step size is not the binding constraint it was when this was written, and the reason is worth carrying:** the dt-dependent steady state (friction computed from the pre-friction velocity) was repaired at `77b1f5a` — implicit in the updated flux, closed-form quadratic solve, measured flat at Fr 2.0000 from dt 0.4 s down. What binds at dt 0.8 s now is the outflow clamp, which is a volume constraint and a different question.
- **And the framing may be wrong at the root.** Under steady flow the local-inertial system reduces to the diffusion-wave model (de Almeida & Bates 2013 §4), so a stationary solve targets the same answer this march is approaching rather than a cheaper approximation of it. Worse for marching: explicit diffusive stable dt falls *quadratically* under refinement and degrades in near-horizontal water surfaces — a lake is this scheme family's documented worst case. The retired prototype's own record (`ref/hydrology/NOTES.md`) marched to flat lakes at 4 m cells and paid with rain 100–1000× real, and named its own fix as priming channels with steady-state discharge, which is GraphFlood described before knowing it existed. Dossier: `msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md`.
- **Correction owed to the validity envelope:** the `Fr > 1.5` gauge this kernel is measured against is the *Vedernikov* critical, and the probe that measured it refuted that identification. de Almeida & Bates' stated envelope is `Fr < 0.5`; the fraction of wet cells above **0.5** is what the citation licenses and nobody has measured it.
