---
slug: form-ordinum-governs-flux-web
type: formulation
status: exact
stage: draft
depends:
  - form-flux-web
  - def-nomos
  - norm-probes-before-claims
  - disc-check-the-ladder
---

# The ordinum governs the flux web

The phase ladder's promises *are* flux quantities, and `:kept-by` *is* the producer. A nomos that consumes an unkept promise makes the world mechanically unrunnable for that work — the ladder is not decorative.

## Formal Expression

1. **Ordinum as data.** An **ordinum** is the instituted phase floor for a world-kind (Terrestris: `tabularium/terrestris.ordinum.udon`): phases declare charges, promises, defeasances, and records. Schema is world-kind-agnostic; content is lineage-specific.
2. **Promise ↔ flux.** A promise that later work depends on is expressible as a fluxed quantity. When a nomos **consumes** that quantity and no nomos is registered as keeping it, the flux audit reports **Unmet** ( #form-flux-web ).
3. **`:kept-by`.** The function that makes a promise true is named as a nomos. Absence of `:kept-by` is honest gloss, not a guarantee. A `:kept-by` naming a nomos missing from the nomotheke is **BrokenKeeper** — an error, not a maturity rung (`ordinum.rs`).
4. **Maturity ladder (acceptance-test discipline).** NotStarted → Specified (has `|predicate`) → Claimed (`:kept-by` names a real nomos) → Kept (predicate verified). **A promise with no predicate cannot be marked fulfilled at any level** — nothing could convict it. **Kept is not auto-derived** from the presence of a producer; claiming Kept without a refutable check is plausibility-as-verification ( #norm-probes-before-claims ).
5. **What to build next.** Gaps are look-ups: Specified gate-promises with no keeper are the queue the ladder sets, not session taste ( #disc-check-the-ladder ). Abyssal `emerged-land` is **promised** on the freeboard path ( #form-derived-sea-level ); the keepable long-term shape is the lithosphere **column** → isostasy ( #form-isostasy-column ), not a strictly-positive rate field.

## Epistemic Status

**Max attainable: exact** for the governing join Joseph decided (`DECISIONS[ordinum-governs-the-flux-web]`, `:by joseph`, decided). Engine + display: `ordinum.rs` maturity computation (over the flux/audit wiring), rendered by `vivarium status` via `ordinum::render_maturity` — the report reads the same `Promise::maturity()` the engine computes, so there is no second ladder. Stage `draft`.

**Known incomplete:** (1) Full phase-content segmentation of the Terrestris ordinum is open (OUTLINE §VII; Abyssal piloted). (2) `emerged-land` is **Claimed** by isostasy ( #form-isostasy-column ) but not **Kept** until Record-style predicates convict per cycle.

## Discussion

Before this join, a "closed flux web" could print clean while erosion silently no-op'd on seabed — because land was never declared as a need. The ladder governing the web is what makes under-declaration a failed world rather than a quiet lie.

## Working Notes

- DECISIONS ordinum-governs entry is history of *why*; claim home is this segment. Live floor is `tabularium/terrestris.ordinum.udon` data only (historical PHASES reportatio is ice provenance, not a read dependency).
- Emerged-land keeper path extracted to #form-derived-sea-level ; drop residual banner when nomotheke promises the flux.
