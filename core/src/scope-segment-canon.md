---
slug: scope-segment-canon
type: scope
status: exact
stage: draft
depends: []
---

# Segments are the sole source of truth for project claims

Every settled claim about what vivarium is, must do, or may conclude lives in a claim segment under `core/src/`, identified by its slug. No other document is a parallel canon for those claims.

## Formal Expression

1. **Primary product.** The primary artifacts of this project, for claim-level truth, are the files `core/src/{slug}.md`. Ordering of those claims lives only in `core/OUTLINE.md`. Writing conventions live in `FORMAT.md`.
2. **Sole claim home.** A project claim — a statement about purpose, law, architecture, method, ethics of construction, or what an algorithm asserts — is **authoritative only** when it is the body of a segment (or is a term definition in `LEXICON.udon`, a decision record in `DECISIONS.decision-log.udon`, or the standing moratorium text in `ETHICS.md` as carved below). Design notes, architecture overviews, plans, spikes, READMEs, session maps, and agent memory are **not** claim homes.
3. **Carve-outs (supporting authorities, not rival canons).**
   - **Terms** — headword meaning and carve lines: `LEXICON.udon`.
   - **Decisions** — who decided, status, and the decision statement: `DECISIONS.decision-log.udon`.
   - **Standing moratorium** — full ethical argument: `ETHICS.md` until absorbed; the segment `#scope-moratorium-endogenous-emergence` is the core pointer and scope claim.
   - **Executable state** — code, store roots, and probes *convict or implement* claims; they do not replace the claim text.
4. **Replacement.** When a segment lands or is corrected, it **replaces** prior prose homes for that claim. Old text is deleted, reduced to a one-line pointer, or archived — not kept as a softened twin.
5. **Session success.** Work that does not move a segment (or produce a probe / decision a segment will cite) does not advance claim-truth, however useful it is for play or engineering.

## Epistemic Status

**Max attainable: exact** under the project's self-legislation — this is a scope rule chosen for multi-agent continuity, not a theorem about the world.

Currently `exact` as project law for this phase (Joseph, 2026-07-20: segments shall be the only source of truth going forward). Stage is `draft` until external audit. The rule does **not** claim that all true content has already been migrated into segments; migration incompleteness is expected and does not license treating unmigrated prose as canon.

## Discussion

Hand-maintained "current state" documents grow and go stale by the same property, then get cited as law. A claim channel that is empty while the executable tree is rich invites agents to fill the vacuum with code and new maps. This scope rule closes the vacuum: the door is the outline and the segments, not a summary of them.

Fun and exploration remain legitimate. They are constrained only where they would mint a claim outside a slug.

## Working Notes

- Migration of `doc/ARCHITECTURE.md` and design corpus into segments is open work; until a claim has a slug, treat those files as **sources to extract from**, not as law to quote.
- Whether `ETHICS.md` remains permanently out-of-core or is fully segmented is open; until then the carve-out stands.
