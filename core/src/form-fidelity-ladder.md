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
5. **Execution class (frame).** Systems declare execution class (batch-deep / relaxation / procedural-tight) so coupling and cost are honest — field residual until nomotheke carries it ( #form-kernel-imperative-boundary debt).

## Epistemic Status

**Currently `robust-qualitative`** as design stance (DESIGN-REDUX §12). Stage `draft`.

## Working Notes

- Specimens (fBm prior, mineral totals, single hardness) stay teaching in REDUX.
- GPU/backend-as-rung: water-parallelism plan; not claimed built.
