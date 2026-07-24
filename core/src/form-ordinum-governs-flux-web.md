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

> [!warning] **Code-first trail (not yet claim-complete)** — base `ce55ddf` (2026-07-23). FE(5) still names Abyssal `emerged-land` as an **unkept** queue item; work in progress will add a keeper (freeboard / isostatic stand-in, not full plate tectonics) and must update this segment so "what to build next" matches the nomotheke. Until then, live code ahead of this FE is provisional. Segment backfill required before compact.

The phase ladder's promises *are* flux quantities, and `:kept-by` *is* the producer. A nomos that consumes an unkept promise makes the world mechanically unrunnable for that work — the ladder is not decorative.

## Formal Expression

1. **Ordinum as data.** An **ordinum** is the instituted phase floor for a world-kind (Terrestris: `tabularium/terrestris.ordinum.udon`): phases declare charges, promises, defeasances, and records. Schema is world-kind-agnostic; content is lineage-specific.
2. **Promise ↔ flux.** A promise that later work depends on is expressible as a fluxed quantity. When a nomos **consumes** that quantity and no nomos is registered as keeping it, the flux audit reports **Unmet** ( #form-flux-web ).
3. **`:kept-by`.** The function that makes a promise true is named as a nomos. Absence of `:kept-by` is honest gloss, not a guarantee. A `:kept-by` naming a nomos missing from the nomotheke is **BrokenKeeper** — an error, not a maturity rung (`ordinum.rs`).
4. **Maturity ladder (acceptance-test discipline).** NotStarted → Specified (has `|predicate`) → Claimed (`:kept-by` names a real nomos) → Kept (predicate verified). **A promise with no predicate cannot be marked fulfilled at any level** — nothing could convict it. **Kept is not auto-derived** from the presence of a producer; claiming Kept without a refutable check is plausibility-as-verification ( #norm-probes-before-claims ).
5. **What to build next.** Gaps are look-ups: Specified gate-promises with no keeper (e.g. Abyssal `emerged-land`) are the queue the ladder sets, not session taste ( #disc-check-the-ladder ).

## Epistemic Status

**Max attainable: exact** for the governing join Joseph decided (`DECISIONS[ordinum-governs-the-flux-web]`, `:by joseph`, decided). Engine: `ordinum.rs` maturity computation + flux/audit wiring. Stage `draft`.

**Known incomplete:** CLI does not yet display the maturity report Joseph asked to *see* (engine exists; display does not). Full phase-content segmentation of the Terrestris ordinum is open (OUTLINE §VII).

## Discussion

Before this join, a "closed flux web" could print clean while erosion silently no-op'd on seabed — because land was never declared as a need. The ladder governing the web is what makes under-declaration a failed world rather than a quiet lie.

## Working Notes

- Ice/pointer: DECISIONS ordinum-governs entry remains history of *why*; claim home is this segment. `.archive/PHASES.md` is reportatio, not law.
- Display debt: wire maturity into `vivarium status` (or sibling command) without inventing a second ladder.
