---
slug: form-fidelity-ladder
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-fidelity-invariant
  - form-complete-content-addressed-key
  - def-nomos
  - form-reductionist-fallback-cases
---

# Fidelity ladder: climb to discover, descend to surrogate

Each aspect may occupy a **ladder of models** from crude to high-fidelity. Low rungs are honest when they honour the sufficient statistics consumers need with known bounded error — not a lie dressed as placeholder. The ladder runs **both ways**.

## Formal Expression

1. **Rungs.** An aspect has a ladder of models (crude macro-statistic match → higher physics). Occupying a low rung is correct when #form-fidelity-invariant holds for its consumers.
2. **Climb.** Stepwise emergence discovers behaviour; regime probes validate ( #norm-regime-probes , case C of #form-reductionist-fallback-cases ).
3. **Descend.** Once patterns are characterized and probe-validated, **descend** to a tight procedural surrogate that reproduces the discovered statistics; keep the expensive rung as calibrator (re-run on nomos change).
4. **Key identity.** Model identity and version are part of the content-addressed key ( #form-complete-content-addressed-key , #def-nomos ). Swapping a rung invalidates that nomos and its dependents only.
5. **Execution class.** Systems declare execution class (batch-deep / relaxation / procedural-tight) so coupling and cost are honest — carried as `NomosDecl::execution` since 2026-07-24 ( #form-kernel-imperative-boundary FE(5)).
6. **Scaffolding has a demolition date.** Explicit state that parameterizes what a finer rung would emerge (armor field standing in for selective transport of real grain sizes; colmation for fines percolation) is declared as scaffolding: when the finer rung lands and the behaviour re-emerges, the explicit field retires. Keeping both is a fork.

## Epistemic Status

**Currently `robust-qualitative`** as design stance (DESIGN-REDUX §12). Stage `draft`.

## Working Notes

- Specimens (fBm prior, mineral totals, single hardness) stay teaching in REDUX.
- GPU/backend-as-rung: water-parallelism plan; not claimed built.
