---
slug: form-pull-query-composition
type: formulation
status: exact
stage: draft
depends:
  - post-determinism-as-ontology
  - form-complete-content-addressed-key
  - form-store-as-save
  - def-nomos
  - form-nomotheke-registry
  - form-depend-by-key-never-latest
  - form-builder-admission
---

# Pull composition of world law

World-law values are obtained by **pull evaluation** of declared nomoi at complete content-addressed keys: Hit returns matured bytes; Miss computes under pure keyed inputs, may **pull** other complete keys for dependencies, then Puts. Composition is recursive pull on the store bus — not a shared mutable world object and not a global dense finest-tick.

## Formal Expression

1. **Pull composition.** A world-law value is the evaluation of a declared nomos at a complete content-addressed key ( #form-complete-content-addressed-key , #def-nomos ). Store Hit returns matured bytes; Miss computes from pure keyed inputs ( #post-determinism-as-ontology ), may pull other complete keys as dependencies, then Puts ( #form-store-as-save ). There is no second channel of world truth outside that bus.
2. **Dependencies are recursive pulls.** Nomoi couple by reading other nomoi’ memoized outputs (or flux objects named by key), not by sharing mutable runtime state. Composition depth is the dependency cone of the demanded key.
3. **Demand schedules; keys define.** *Which* keys are scheduled is demand (builder beacons, explorer spool, phase target). *What* a key’s bytes are is fixed by the complete key graph ( #form-depend-by-key-never-latest ). Conflating schedule with ontology is the failure mode.
4. **Role split on who may compute.** Builder and other admitted materializers may Miss→compute under admission rules ( #form-builder-admission ). Explorers and views use observe-only pull: Hit preferred; lawful coarse or instant prior on Miss; no cold long-evolution on the view path.
5. **Pattern, not engine.** This law is the **evaluation pattern**. A generic incremental query-graph engine (Salsa/Adapton lineage) is a permitted future mechanization of the same pattern — not required for the law to hold, and not claimed as present.
6. **Out of bounds for this segment.** (a) Three-scoped runtime (coarse spine + local cones + edit-propagation) as shipped architecture; (b) predictive prefetch; (c) “time in every key” as fully mechanized DAG; (d) edit up-invalidation / irreducible-edit closure; (e) independent coarse-tier evolution vs leaf-only flux identity (open tension with #form-face-flux-register ); (f) restating complete-key or save≡store as if new here.

## Epistemic Status

**Max attainable: exact** for FE(1)–(4) as architecture under determinism, complete keys, registry, and store-as-bus — sourced from DESIGN-REDUX §11 and ARCHITECTURE §5 *as doctrine*, not from crate completeness.

**Currently `exact` as evaluation-strategy law.** Present crates implement the pattern in **hand-written per-nomos miniature** form; that is **illustration and compliance surface**, not the warrant for the claim. Architecture’s own honesty still stands: a generic planner/engine is a **gap** (build debt under this law, not a softener of the law).

**Do not read:** “engine unbuilt” as “pattern untrue.” Strengthen-before-soften: keep the thin law; name thick aspirational pieces as residual source (three-scoped, time-DAG, prefetch).

Stage `draft`. Hostile-read FE from peer adjudication `msc/agent-briefs/2026-07-23-lazy-query-graph-adjudication.md` (parent re-weighted: code rank last).

## Discussion

Represent-by-consequence ( #post-represent-by-consequence ) says *where* to spend compute. Fidelity ( #form-fidelity-invariant ) says *how fine* when pulled. This segment says *how law values compose* when something is pulled: through the keyed store, recursively, under role-split admission.

## Working Notes

- **Slug:** pull-query-composition (avoids implying a built graph engine). Inventory “lazy query graph” maps here for the thin law only.
- **Dual-home after land:** DESIGN-REDUX §11 opening; ARCHITECTURE §5 first sentences; `query.rs` module banner → this slug; leave three-scoped / prefetch / time-DAG teaching in place as source.
- **Sibling residuals:** three-scoped pieces; time-in-every-key DAG; leaf-only vs independent tiers; full demand spool (OUTLINE builder daemon gap).
