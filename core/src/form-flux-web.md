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

Nomos couple only through a shared vocabulary of fluxed quantities: one produces, another consumes, and that matched pair is the edge. Unmet consumption makes a world mechanically unrunnable for the consumer — when the builder and audit honor the declaration.

## Formal Expression

1. **Edge = matched quantity.** A multiscale world is assembled from nomos that do not share mutable state. Coupling is a string-identity on a **fluxed quantity**: a producer **promises** a quantity; a consumer **consumes** the same quantity. That pair is the only agreement they need (`flux.rs` vocabulary; #def-nomos declaration surface).
2. **Closed vocabulary.** Consume/promise strings come from one shared vocabulary (or a build fails). A typo is not allowed to look like a missing physics link.
3. **Unmet need.** If a nomos consumes a quantity no registered nomos promises, that edge is **Unmet**. The pre-run audit (`audit.rs`) is a pure graph query over the nomotheke — nothing need run to print the gap.
4. **Admission.** Materializing a consumer while its requisite chain is Unmet is unprincipled unless explicitly waived as provisional. Default builder admission refuses such phases; exploratory override is a named flag, not silence ( #norm-declaration-must-convict ).
5. **Ordinum join.** Promises of the phase ladder and flux quantities are the same grain when the ladder is wired: an unkept ladder promise that a nomos consumes is an Unmet need ( #form-ordinum-governs-flux-web ).
6. **Statistic + exactness on the edge.** Quantity-name identity is **necessary but not sufficient**. Every promise declares which **statistic** it guarantees (conserved-total, mean, max, center-sample, band-limited, …) and an **exactness** claim; every consume declares the statistic it **needs**. Coupling is sound only when vocabulary **and** statistic compatibility hold. Quantity-matched but statistic-mismatched is **Unsound** (distinct from **Unmet**) — storing the wrong statistic silently corrupts the macro ( #form-column-control-volume , #form-rl-closure-algebra consumer-$R$). **Live:** `Promise{statistic, exactness}` + `Consume{quantity, needs}` on `NomosDecl`; `audit::statistic_match_across_registry` is the match instrument, and a registry test pins **no Unsound edge** while naming the **Undeclared** set exactly. `Statistic::Undeclared` is a first-class honest value: the open column-semantics fork surfaces as named debt on its two elevation-class edges rather than being closed by code default. Do not freeze the statistic set as complete in this FE — it grows with consumers.

## Epistemic Status

**Max attainable: exact** as architecture of coupling under the nomotheke-as-built for FE(1)–(6). FE(6)'s fields and match audit are mechanized (2026-07-24); its *content* is only as strong as the per-quantity declarations, two of which are honestly `Undeclared` pending the column-semantics decision.

Live: vocabulary module, `Consume`/`Promise` with statistic + exactness on `NomosDecl`, `audit::producer_of` / `unmet_across_registry` / `requisite_chain` / `statistic_match_across_registry`, builder gate. Stage `draft`.

**Known incomplete / compliance debt:** (1) a cross-scale *level* component on the statistic contract (needs mean at-least-L19) is deferred until a cross-scale consumer exists; (2) maturity of ladder promises not yet displayed in CLI; (3) provisional root tags under waived admission — end-to-end bin integration residual ( #form-builder-admission ); (4) freeboard / emerged-land producer residual under #form-derived-sea-level when the live graph lags the claim.

## Discussion

The flux web is how "rain without a sky" and "erosion without land" stop being oral tradition and become failed builds or loud audits. It is representation-agnostic: a box hydrosphere and a field erosion tile couple by quantity, not by shared mesh.

Per-quantity granularity is also the upgrade path: a higher-fidelity producer that *enriches* the flux (erosion-v2 fluxing grain-size, not just volume) adds a **new** quantity — consumers that read it gain a key input and rerun ( #form-complete-content-addressed-key ); consumers that do not are untouched. A monolithic flux blob would over-invalidate on every model upgrade, which is why the interface stays fine-grained.

## Working Notes

- Ice/pointer targets once stable: `flux.rs` module docs; ARCHITECTURE §9 coupler bullets; regula-conformance-design §3 (reasoning trail).
- Sibling segments: #def-nomos (article), #form-ordinum-governs-flux-web (ladder drives web), #form-kernel-imperative-boundary (declare vs kernel), hydrosphere box, #form-derived-sea-level .
- **Do not** mint `#form-sufficient-statistic-seam` — dual-home of column + seam + this FE(6). Adjudication: `msc/agent-briefs/2026-07-23-sufficient-statistic-adjudication.md`.
- Residual trail: drop freeboard incomplete item when live graph matches claim.
