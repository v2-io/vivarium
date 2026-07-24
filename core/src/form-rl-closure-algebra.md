---
slug: form-rl-closure-algebra
type: formulation
status: robust-qualitative
stage: draft
depends:
  - post-determinism-as-ontology
  - post-represent-by-consequence
---

# Multiscale R / L / closure algebra

Every multiscale method is four objects and three honesty laws: macro $U$, micro $u$, restriction $R$, lifting $L$ with a closure. $R \circ L = \mathrm{id}$ on $U$ is the fidelity gate — currently an architecture law the code does not satisfy.

## Formal Expression

1. **Four objects.** A multiscale article of world work is built from:
   - **$U$** — the **macro state** (coarse field, drainage graph, climate zone map, …);
   - **$u$** — the **micro state** (fine cells, storms, sub-grid detail);
   - **$R : u \to U$** — **restriction** (coarse-graining): many-to-one; information-destroying *by design*;
   - **$L : U \to u$** — **lifting** (reconstruction): one-to-many, so every $L$ carries a **closure** — a chosen measure over the missing detail.

2. **Three honesty laws.**
   1. **$R \circ L = \mathrm{id}$ on $U$** — reconstruct then summarize must return what the macro claimed (fidelity invariant for the chosen statistics).
   2. **Conservative flux exchange** — when a conserved quantity crosses a seam (scale-to-scale or aspect-to-aspect), exchange **fluxes, not raw states**, and balance them exactly (claim home: #form-seam-flux-exchange ).
   3. **Scale separation** — the fast process is *equilibrated* when seen from above; the slow one is *quasi-static* when seen from below. Where separation fails, no multiscale method saves you — resolve, or accept unbounded error. Operational specimen: erosion is geological, water hydrological — never one shared timestep.

3. **Closure is fated, not sampled.** Literature $L$ often samples the missing measure (Monte Carlo, ensemble mean, max-entropy). Vivarium's closure is **fated lifting**: the missing detail is a pure function of $(\mathrm{seed}, k)$ ( #post-determinism-as-ontology , #lexicon/term/fated-noise ). That is what makes content-addressed memoization sound: two lifts of the same key agree, so caching changes cost, never the world.

4. **Consumer-dependent $R$ (stance).** The literature often fixes one $R$ per method; vivarium wants **$R$ per consumer** (hydrology needs conserved totals; line-of-sight needs max; display needs mean). Store the wrong statistic and the fine materialization silently corrupts the macro.

5. **What this is not.** This segment does **not** claim that live kernels already implement honest $R \circ L = \mathrm{id}$, refluxing, or consumer-dependent $R$. It names the algebra the stack must speak. Method-zoo taxonomy (AMR, HMM, multirate, equation-free, …) and seam-type catalogue stay in theory sources until they earn their own segments.

## Epistemic Status

**Max attainable: exact** for the operator vocabulary as shared mathematical structure (HMM $\mathcal{Q}\mathcal{R}=I$, equation-free $\mathcal{M}\mu=I$, Berger–Oliger injection — primaries read; see `doc/theory/multiscale-methods.md`, `doc/theory/multiscale-seams.md`, ARCHITECTURE §1).

**Currently `robust-qualitative`:** the four objects and three laws are the project formulation of that structure; consumer-dependent $R$ and fated-lifting-as-closure-stance are design commitment under determinism, not Joseph-stamped as a separate DECISIONS row.

**Compliance debt (strengthen first — do not soften the law):**
1. **Mean-pin is not $R \circ L = \mathrm{id}$ on the mean.** `erosion.rs::pin_block_means` is a smoothed delta injection (bilinear upsample of a block delta), not a block-constant injection. Measured residual on a real L19 eroded tile: mean $0.43\,\mathrm{m}$, max $2.97\,\mathrm{m}$ at 30 epochs, growing with epoch count and curvature-correlated ($r=-0.42$). The in-tree guard passes only because its tolerance ($2.0\,\mathrm{m}$) was sized to the defect (`DECISIONS[mean-pin-does-not-preserve-block-means]`, `:by claude`, proposed measurement — the falsification of the *implementation claim* is measured; the algebra law stands).
2. **Injection $\neq$ refluxing.** Mean-pin (when corrected) is Berger–Oliger fine→coarse *injection/update*. Conservative interface flux correction (*refluxing*, Berger–Colella 1989) is distinct and still missing at tile seams.

Stage `draft`. Sources for extraction: ARCHITECTURE §1 (corrected), multiscale-methods §§1–4, multiscale-seams §§1–2, #post-represent-by-consequence Working Notes (do not treat uncorrected $R \circ L$ prose as this postulate).

## Discussion

Without this algebra named, every multiscale construct looks like a one-off invention. With it named, a system author declares $R$, $L$, and closure the way a nomos declares consumes/promises — and probes can convict whether the pair is honest. Represent-by-consequence ( #post-represent-by-consequence ) is the demand-side twin: compute only where a consumer needs the result; this segment is the scale-crossing machinery that makes that demand honest.

## Working Notes

- **Dual homes demoted (prior wave + this):** multiscale-methods header + §1; ARCHITECTURE §1; multiscale-seams header + §1; DESIGN-REDUX §5 algebra join. Method-zoo rows, dynamic-exponent $z$, and detail→abstract open problem remain source until segmented.
- **Sibling claims (not this segment):** #form-seam-flux-exchange (fluxes-not-states); #form-store-as-save; consumer-dependent statistic contract as flux-edge field; wavelet/Haar store spike findings.
- **Do not re-assert** "mean-pin enforces $R \circ L = \mathrm{id}$ live" in any file — the corrected law is aspiration + compliance debt.
