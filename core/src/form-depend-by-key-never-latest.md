---
slug: form-depend-by-key-never-latest
type: formulation
status: exact
stage: draft
depends:
  - form-complete-content-addressed-key
  - form-store-as-save
  - form-builder-admission
  - post-determinism-as-ontology
---

# Depend by key, never by “finest available”

A nomos may depend on a neighbour only **by complete content-addressed key** — a specific `(tile, level, time-index, …)` chosen by the dependency map — never on “the finest / latest available data.” Build order changes *which* memos exist, never their values.

## Formal Expression

1. **Build-order independence.** Two builds of the same vivium spec, advanced along different demand orders (different explorers, beacons, walking routes), converge to **byte-identical state wherever both have materialized**. Every memo is a pure function of its complete key ( #form-complete-content-addressed-key , #post-determinism-as-ontology ); computation order only selects which keys exist so far.
2. **The invariant that preserves it.** A nomos reads neighbours **only by key** selected by the dependency map (e.g. drainage islands from the coarse spine). It does **not** read “whatever is finest on disk right now.” “Use the best data we happen to have” makes the world a function of the walking route.
3. **Fidelity dial location.** Which keys get **scheduled** lives in the demand planner / builder (beacons, cones, phase target). Which keys get **read** for a given computation is fixed by the nomos key graph. Confusing the two is the failure mode.
4. **Out of bounds.** (a) Implicit “latest root” edges for neighbour tiles. (b) Conditional lifts that upgrade resolution because a finer memo happened to land. (c) View-side or instrument knobs that change which world-evolution keys the kernel uses ( #form-core-view-wall , #form-builder-admission ).
5. **Native artifacts.** Keys may name nomos **outputs** (drainage graph, slab set), not only cells; the invariant is still “named key,” not “best available grain.” Over-keying is safe; under-keying is not.
6. **Seam join.** Flux BCs across tiles still name the **flux objects by key** ( #form-seam-flux-exchange , #form-face-flux-register ) — not “the finest discharge field present.”

## Epistemic Status

**Max attainable: exact** as frame property under complete keys + fated computation. **Currently `exact`** for the invariant as architecture law (`doc/plan/builder-explorer-decoupling.md` §0, settled this-session design 2026-07-10; load-bearing for Phase-3 flux-BC tiles). Production flux-BC tiles still incomplete — debt does not soften the invariant.

Stage `draft`.

## Discussion

Without this rule, content-addressing buys cache reuse and loses the laboratory: two explorers would mint two worlds that only look like one seed. With it, demand is scheduling, not ontology.

## Working Notes

- Source: builder-explorer-decoupling §0; dual-home demote law paragraph there to pointer.
- Sibling gaps: full builder daemon / demand spool remain OUTLINE §III.
- `#form-seam-flux-exchange` OOB (c) “finest/latest available neighbour” — this segment is that rule’s home.
