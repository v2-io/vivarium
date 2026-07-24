---
slug: norm-bias-vs-noise
type: normative
status: exact
stage: draft
depends:
  - disc-algorithms-disguise-physics
  - disc-prime-question
  - norm-probes-before-claims
  - post-determinism-as-ontology
---

# Bias versus noise is the decisive audit

When a discretisation is wrong, the first question is whether the error is **bias** or **noise** — not how large the magnitude is. Same size; categorically different harm.

## Formal Expression

1. **Order of questions.** After the physical claim of a scheme is named ( #disc-prime-question , #disc-algorithms-disguise-physics ), any residual discrepancy between that claim and the scheme-as-written is audited first as:
   - **noise** — zero-mean (or averaging-to-zero under the accumulation the law actually uses), so catchment-scale or ensemble-scale summation can wash it out; or
   - **bias** — signed / directional / topological, so it **manufactures a fake physical law** and **compounds** along the pathway the phenomenon integrates (drainage, transport, energy budgets, seams).
2. **Magnitude is second.** A large noise and a small bias are not comparable by percentage alone. A ~20% unbiased catchment error may largely wash under summation; a ~20% directional bias can lock rivers to grid axes and compound down the network.
3. **Admissible remedies reorder by kind.**
   - *"Negligible at this scale"* is admissible **only for noise**. For bias it is **never** admissible at any scale — zooming does not make a fake law true.
   - Supplying true metrics (distances, areas) is often necessary and often **insufficient** if the scheme’s structure (not the numbers) is the source of the bias.
   - **Recovering the physical claim and rebuilding the kernel** is the principled path when the scheme’s assumption fails the vivium’s topology or multiscale operators.
   - **Fated stochastic jitter** ( #post-determinism-as-ontology ) is a legitimate **fallback**, not a hack: a **bias → variance** conversion that stays deterministic and can stay conservative under normalization. It is strictly weaker than recovering the physics (honest error without small error). Adopting it where the downstream law is **nonlinear** requires a probe first — variance injected into a superlinear term can **manufacture a new Jensen bias**.
4. **Seams inherit the cut.** Coarse-graining a **nonlinear** law is a bias (not mere loss of precision) unless the carried statistics are sufficient for the law being evaluated. That is the same cut applied at multiscale seams, not only inside a single kernel.
5. **Out of bounds.** Treating a measured directional attractor, a level-independent spiral, or a sign-definite clip as “noise that will average out” without a control that could have falsified that reading. Confirming magnitudes without a bias-vs-noise control are not an audit.

## Epistemic Status

**Max attainable: exact** as process/architecture law under the decided decision record. `DECISIONS[bias-vs-noise-is-the-decisive-audit]` — `:by us`, `:status decided` (2026-07-12). Companion chain: `DECISIONS[algorithms-are-disguised-physical-claims]` (impact clause names this cut) and the MFD fan measurement family (bias, not noise; does not converge away) — those are **illustrations and probes**, not re-proved here.

**Not claimed here:** that every nomos declaration already carries a bias-vs-noise field (`NomosDecl` / geometric contract still lacks a storage home — design debt, same class as modified-equation box ⑤). **Present practice lag does not soften the law** (strengthen-before-soften: add the field and the probes; do not demote the cut). Stage `draft`.

## Discussion

This is the severity half of the Prime Question stack: #disc-prime-question recovers *what PDE/ODE the scheme is actually solving*; this norm adjudicates *what kind of wrong* the residual is and which fixes are legal. Without it, “close enough” launders grid-locked attractors into world geography.

## Working Notes

- **Dual homes demoted:** discretisation-and-information.md §1 (teaching); NOMOS-CONTRACT bias adjudication section; `.archive/CLAUDE.md` archive banner + bias paragraph pointer. Worked MFD / θ / clip specimens stay in theory + DECISIONS.
- **`#disc-prime-question` Working Notes** already deferred this cut here — no fork of the rule back into that segment.
- **Unbuilt surface:** geometric-contract / `NomosDecl` field for declared bias-vs-noise class of each residual — `doc/design/NOMOS-CONTRACT.md` and DECISIONS impact; not invented schema here.
- **Related open:** Jensen / sufficient-statistic contract and column cell-average vs point-sample fork need their own segments; this norm only asserts the audit cut they must answer.
