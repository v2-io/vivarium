---
slug: disc-prime-question
type: discussion
status: robust-qualitative
stage: draft
depends:
  - disc-algorithms-disguise-physics
  - norm-probes-before-claims
---

# The Prime Question is a computation

Before a scheme may stand as a nomos claim, answer *"what physical claim is this algorithm actually making?"* by modified-equation analysis (or its ODE twin) — a computation that returns a term with a sign and a differential order, not a rhetorical disposition.

## Formal Expression

1. **The question.** For any discrete scheme under consideration as world law or as a port of a published algorithm: **what physical claim is this algorithm actually making?** (Pedagogical name: the *Prime Question*.)
2. **The answer is a computation, not a mood.** On the PDE side the standard method is **modified-equation analysis** (Warming & Hyett 1974): Taylor-expand the discrete scheme, eliminate time derivatives to the order carried, and read the **PDE the scheme actually solves**. The leading truncation term is a *physical* term with a **sign** and a **differential order** — classically, even-order residuals read as numerical diffusion; odd-order residuals as numerical dispersion / spurious advection. On the ODE / time-integrator side the twin is **backward error analysis**: the numerical trajectory is the exact trajectory of a modified system.
3. **What a nomos claim requires.** A scheme is not assertable as the physics it was sold as until that computation (or an honest equivalent that still returns term + sign + order, not free-hand gloss) can be stated for the scheme *as written*. If the answer cannot be produced, the compromises are unknown — and so are they for anyone downstream.
4. **Port the claim you named, not the paper costume.** This is the operational half of #disc-algorithms-disguise-physics: that segment forbids bending the vivium to paper grid habits; this segment names the instrument that recovers *which* physical claim the algorithm was asserting, including unphysical terms the author never wrote down.
5. **Out of bounds.** Treating the Prime Question as a vibe-check ("it looks like advection"), as a comment in the kernel, or as the paper's *intended* PDE without expanding the scheme actually running, is not an answer. Intent is not the modified equation.

## Epistemic Status

**Max attainable: robust-qualitative** as standing method for nomos work; the classical FD/linear modified-equation procedure is standard numerical analysis (not a project invention). Pedagogical framing "Prime Question" is project teaching language for that computation (`DECISIONS[structure-preserving-is-a-rediscovery-adopt-the-field]`, `:by claude`, *proposed* for LEXICON adoption — do not treat that entry as Joseph-settled law). The requirement that the claim be *answerable before the scheme is sold as physics* rides with `DECISIONS[algorithms-are-disguised-physical-claims]` (`:by us`, decided) and with #norm-probes-before-claims (no behavior/physics claim without something that can fail).

**Honest limits.** (1) Warming–Hyett is classical for finite-difference / smooth linear(ish) settings; nonlinear, clipped, adaptive, or multiscale kernels may lack a clean closed-form expansion — then the duty is still to produce the best available term-level characterization (analysis where possible, measured residual structure where not), not to waive the question. (2) This segment does **not** assert that every live kernel already has a written modified equation, nor that `NomosDecl` can store box ⑤ yet. (3) After the modified equation is known, severity adjudication is the **bias vs noise** cut (`DECISIONS[bias-vs-noise-is-the-decisive-audit]`, `:by us`, decided) — a separate question; magnitude alone is never the first question. (4) Worked specimens (MFD fan, $\theta$ as even/Laplacian diffusion, $p$-exponent first-moment bias) live in DECISIONS and probes; they illustrate the method and are not re-proved here.

Stage `draft`.

## Discussion

The failure mode the method blocks is arguing about the algorithm one *imagines* (or the paper's abstract PDE) instead of the operator in the file. Even-order vs odd-order residuals matter operationally: an even residual cannot be the "stand-in for neglected advection" a comment might claim. Pair with #disc-algorithms-disguise-physics for the porting rule; with #norm-probes-before-claims so the answer is convictable rather than narrative.

## Working Notes

- **Pointer-only:** law is this segment; MFD teaching chain → #worked-example-mfd-prime-question ; measurements → #obs-cube-locked-kernel-bias ; residual theory §0 dual-home demoted. Bias-vs-noise cut is `#norm-bias-vs-noise`.
- **`msc/research-structure-preserving/README.md` §1.2** (Prime Question ≡ modified-equation analysis; Warming & Hyett) becomes a literature pointer into this segment for project law; survey detail stays in the recon doc.
- **`doc/design/NOMOS-CONTRACT.md` box ⑤** still needs procedure + `NomosDecl` fields; this segment states the claim-side requirement only — does not invent schema.
- **LEXICON:** `modified-equation` / `backward-error-analysis` still unlanded; proposed under the structure-preserving DECISION, not ratified here.
