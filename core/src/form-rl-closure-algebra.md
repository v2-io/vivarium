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

5. **What this is not.** This segment does **not** claim that live kernels already implement honest $R \circ L = \mathrm{id}$, refluxing, or consumer-dependent $R$. It names the algebra the stack must speak. Method-zoo taxonomy (AMR, HMM, multirate, equation-free, …) is graduated teaching (`.super-archive/from-theory/multiscale-methods.md`); seam-type catalogue remains teaching in `doc/theory/multiscale-seams.md` until further segmented.

## Epistemic Status

**Max attainable: exact** for the operator vocabulary as shared mathematical structure (HMM $\mathcal{Q}\mathcal{R}=I$, equation-free $\mathcal{M}\mu=I$, Berger–Oliger injection — primaries read; see `.super-archive/from-theory/multiscale-methods.md` (graduated teaching zoo), `doc/theory/multiscale-seams.md`, ARCHITECTURE §1).

**Currently `robust-qualitative`:** the four objects and three laws are the project formulation of that structure; consumer-dependent $R$ and fated-lifting-as-closure-stance are design commitment under determinism, not Joseph-stamped as a separate DECISIONS row.

**Compliance debt (strengthen first — do not soften the law):**
1. **Mean-pin is not $R \circ L = \mathrm{id}$ on the mean** — and it **manufactures seam ridge and mass**. Measurement home: #obs-mean-pin-manufactures-seam (zero-physics ridge $\sim 2\times$; pin-on seam $5.77\to 2.03$ pin-off; mass $+0.136\%$ at $150$ fine epochs; residual mean/max on L19 also in `DECISIONS[mean-pin-does-not-preserve-block-means]`). Algebra law stands; implementation convicted.
2. **Injection $\neq$ refluxing.** Mean-pin (when corrected) is Berger–Oliger fine→coarse *injection/update*. Conservative interface flux identity lives on a **face flux register** at hanging nodes ( #form-face-flux-register ); a separate refluxing *pass* is only needed if double-evolve is retained.

Stage `draft`. Sources for extraction: ARCHITECTURE §1 (corrected), graduated multiscale-methods §§1–4 (`.super-archive/from-theory/`), multiscale-seams §§1–2, #post-represent-by-consequence Working Notes (do not treat uncorrected $R \circ L$ prose as this postulate).

## Discussion

Without this algebra named, every multiscale construct looks like a one-off invention. With it named, a system author declares $R$, $L$, and closure the way a nomos declares consumes/promises — and probes can convict whether the pair is honest. Represent-by-consequence ( #post-represent-by-consequence ) is the demand-side twin: compute only where a consumer needs the result; this segment is the scale-crossing machinery that makes that demand honest.

## Working Notes

- **Dual homes demoted (prior wave + this):** multiscale-methods **graduated** (`.super-archive/from-theory/multiscale-methods.md`, 2026-07-24); ARCHITECTURE §1; multiscale-seams header + §1; DESIGN-REDUX §5 algebra join. Dynamic-exponent $z$ and reversion residue remain sketch/source until further segmented.
- **Sibling claims:** #form-seam-flux-exchange ; #form-face-flux-register ; #obs-mean-pin-manufactures-seam (measurement home for pin defects); consumer-dependent statistic contract; wavelet store representation findings.
- **Do not re-assert** "mean-pin enforces $R \circ L = \mathrm{id}$ live" in any file — the corrected law is aspiration + compliance debt.
