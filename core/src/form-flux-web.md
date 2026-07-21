---
slug: form-flux-web
type: formulation
status: exact
stage: draft
depends:
  - def-nomos
  - form-complete-content-addressed-key
  - norm-declaration-must-convict
---

# The flux web

Nomoi couple only through a shared vocabulary of fluxed quantities: one produces, another consumes, and that matched pair is the edge. Unmet consumption makes a world mechanically unrunnable for the consumer — when the builder and audit honor the declaration.

## Formal Expression

1. **Edge = matched quantity.** A multiscale world is assembled from nomoi that do not share mutable state. Coupling is a string-identity on a **fluxed quantity**: a producer **promises** a quantity; a consumer **consumes** the same quantity. That pair is the only agreement they need (`flux.rs` vocabulary; #def-nomos declaration surface).
2. **Closed vocabulary.** Consume/promise strings come from one shared vocabulary (or a build fails). A typo is not allowed to look like a missing physics link.
3. **Unmet need.** If a nomos consumes a quantity no registered nomos promises, that edge is **Unmet**. The pre-run audit (`audit.rs`) is a pure graph query over the nomotheke — nothing need run to print the gap.
4. **Admission.** Materializing a consumer while its requisite chain is Unmet is unprincipled unless explicitly waived as provisional. Default builder admission refuses such phases; exploratory override is a named flag, not silence ( #norm-declaration-must-convict ).
5. **Ordinum join.** Promises of the phase ladder and flux quantities are the same grain when the ladder is wired: an unkept ladder promise that a nomos consumes is an Unmet need ( #form-ordinum-governs-flux-web ).

## Epistemic Status

**Max attainable: exact** as architecture of coupling under the nomotheke-as-built. Live: vocabulary module, `consumes`/`promises` on `NomosDecl`, `audit::producer_of` / `unmet_across_registry` / `requisite_chain`, builder gate. Stage `draft`.

**Known incomplete:** (1) statistic + exactness on a consume edge (mean at-least L…) is designed, not built; (2) maturity of ladder promises not yet displayed in CLI; (3) store census does not yet tag provisional artifacts written under `--allow-unmet`.

## Discussion

The flux web is how "rain without a sky" and "erosion without land" stop being oral tradition and become failed builds or loud audits. It is representation-agnostic: a box hydrosphere and a field erosion tile couple by quantity, not by shared mesh.

## Working Notes

- Ice/pointer targets once stable: `flux.rs` module docs; ARCHITECTURE §9 coupler bullets; regula-conformance-design §3 (reasoning trail).
- Sibling segments: #def-nomos (article), #form-ordinum-governs-flux-web (ladder drives web), hydrosphere as worked box instance.
