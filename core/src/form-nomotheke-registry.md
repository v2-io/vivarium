---
slug: form-nomotheke-registry
type: formulation
status: exact
stage: draft
depends:
  - def-nomos
  - form-complete-content-addressed-key
  - form-flux-web
  - norm-declaration-must-convict
---

# The nomotheke is the registry contract

World-law algorithms enter the store only through declared nomos. The registry is data the build tests, not optional documentation.

## Formal Expression

1. **Registry.** The **nomotheke** is the closed set of #def-nomos declarations (`NomosDecl` statics and `NOMOTHEKE`). A store root whose nomos name is not registered is a bug against this contract.
2. **Declaration is the key stem.** World-law computations mint keys through `NomosDecl::key` (name + version). Callers fold seed, coordinates, and dependency identities into the complete key ( #form-complete-content-addressed-key ). `Key::new` remains for the domain-neutral store layer and tests; world-law paths go through declarations.
3. **Enforced surfaces (live).** The following fail tests (or make coupling unrunnable) when false:
   - names unique; every listed dep is itself registered;
   - every `assumptions` anchor appears in `ASSUMPTIONS.md` (`include_str!` ledger);
   - consume/promise strings are closed over the flux vocabulary;
   - if a consumed quantity has a producer, that producer is in `deps` (else the complete key would omit its version);
   - a complete key built with `with_dep_versions` embeds each direct dep's name and version;
   - every promise carries an explicit conservation stance;
   - derived physics/earth tiers are weakest-link folds over declared deps.
4. **Flux and ladder join.** `consumes` / `promises` are the quantity half of coupling ( #form-flux-web ); ordinum `:kept-by` names keepers that must exist here ( #form-ordinum-governs-flux-web ). The registry is the common ground both audits read.
5. **Undeclared law is unlawful.** An algorithm that shapes the world without a nomotheke entry has no sanctioned path into the world-law namespace — even if it is called from a demo or a view ( #norm-declaration-must-convict · #form-core-view-wall ).

## Epistemic Status

**Max attainable: exact** for the registry-as-gate under present `nomotheke.rs` tests and query paths. Stage `draft`. Joseph-settled grain: nomos as article of law ( #def-nomos ); plural invariant in LEXICON / DECISIONS.

**Known incomplete (do not soft-close):**

1. **Reverse ASSUMPTIONS coverage.** Ledger rows need not appear on any nomos; kernels can lean on ledger-listed constants without an `assumptions:` anchor (e.g. water frame hardcodes not fully listed on WATER).
2. **Under-declaration of non-flux deps.** Example: uplift kernel uses fated noise with empty `deps` — not caught by consumed⇒in-deps.
3. **Source-derived versions** and full NOMOS-CONTRACT boxes (geometry, structure, modified equation) are designed substrate, not mechanized ( #def-nomos epistemic limits).
4. **Statistic/exactness on flux edges** designed, not built ( #form-flux-web ).
5. Generic pull-query engine beyond hand-written per-nomos methods is unbuilt.

## Discussion

#def-nomos says what one article *is*. This segment says how the set of articles *binds* the executable tree: no undeclared world-law, and several honesty properties are compile-testable. When a new system lands, the admission checklist is "declare here first" — not "add a function and a CLI flag."

## Working Notes

- **Promoted** from `msc/consolidation-wave-2026-07-21/draft-form-nomotheke-registry.md` (decisions-code draft; design wave integration).
- **Dual homes demoted:** ARCHITECTURE §9 "adding a system" registry bullets; `nomotheke.rs` module header. Per-nomos epistemic status strings remain data, not re-prose.
- Sibling gaps: ASSUMPTIONS generated ledger; geometric contract as second declare/match surface; NOMOS-CONTRACT boxes ②–④ fields on `NomosDecl`.
