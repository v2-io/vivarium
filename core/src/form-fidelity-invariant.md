---
slug: form-fidelity-invariant
type: formulation
status: exact
stage: draft
depends:
  - post-represent-by-consequence
  - post-determinism-as-ontology
---

# The fidelity invariant

Fidelity is lazily materialized; any materialization must be statistically consistent with the abstraction it replaces, with known, bounded deficiencies — on both spatial and temporal axes, and across co-queried aspects.

## Formal Expression

1. **Invariant (two axes).** Render and simulate at the resolution — **spatial and temporal** — that the most demanding *present* participant needs, and no finer. The dictionary form is #lexicon/term/fidelity-invariant (settled). Space (how fine the field) and time (how far the sim has been pulled / which perceptual band is live) are the same rule, not two independent policies.
2. **Lazy materialization.** Fine detail exists when demanded (a query, a participant, a coupling consumer), not because a uniform finest grid is simpler to code. Unpulled regions may remain coarse or unmaterialized without becoming a second false world.
3. **Statistical consistency.** When fine state replaces (or elaborates) a coarse abstraction, the fine field must **prove the macro true** for the statistics the macro was committed to carry — with **known, bounded** deficiencies. Unknown or unbounded mismatch is a lie about the world, not an LOD optimization.
4. **Co-fidelity corollary (earned detail).** Detail must be **earned**, not decorated. The finest *simulated* tier is authoritative over painted or coordinate-noise detail that overlaps it. **All aspects queried jointly at a place must be mutually consistent at the coarsest of their materialized resolutions.** Decorating terrain finer than the water bed that sits on it manufactures artifacts no sim ever contained (worked failure: painted L24 terrain over L21 water → shredded "bubble" pools).
5. **Four substrates, one rule.** The same invariant binds (a) display geometry, (b) world simulation, (c) numeric precision where error compounds, and (d) cognitive LOD at decision-theoretic junctures. Naming it once is how consistency is designed in rather than retrofitted.
6. **Relation to represent-by-consequence.** This formulation is the fidelity cash-out of #post-represent-by-consequence: spend resolution only where consequence and checkability demand it, and make the representation honest about what is exact versus approximate.

## Epistemic Status

**Max attainable: exact** as project architecture law (founding multi-fidelity commitment; LEXICON settled). Stage `draft`. Co-fidelity is empirically forced by a live 2026-07-03 violation and the repair rule that followed — still formulation-grade law, not a measured theorem about all seams.

**Ceiling / open residue.** Forward materialization and three of four seam types lean on mature prior art (multiscale modeling, couplers, nesting). The hard open half is **detail→abstract**: absorbing irreducible discrete edits back into a memoized macro with correct up-invalidation under the right sufficient statistics. That frontier is **not** solved by restating this invariant; it needs its own segments (sufficient-statistic seams, store up-invalidation). Do not cite this file as closing that problem.

**Not claimed:** posit/ubit numerics as adopted; a specific consumer-statistic schema as built; "the one hard research problem" ranking (forbidden absolute; census must be derived).

## Discussion

Without the invariant, LOD is a bag of performance hacks that silently disagree. With it, a crude rung is honest when it honors the statistics its consumers read, and a fine rung is honest when it does not invent unearned detail next to a coarser authoritative field. Pair with #form-complete-content-addressed-key and #form-store-as-save so fidelity swaps invalidate exactly the right memo cone.

## Working Notes

- **Demote dual homes:** DESIGN.md multi-fidelity paragraph; DESIGN-REDUX §1 (invariant + co-fidelity) and §0 principle pointer via represent-by-consequence; ARCHITECTURE spine bullets that restate the invariant.
- **Siblings not this file:** multirate coupling; sufficient-statistic seam; column control volume; regime-probe methodology; lazy query-graph runtime shape.
