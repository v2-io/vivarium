---
slug: def-nomos
type: definition
status: exact
stage: draft
depends:
  - post-determinism-as-ontology
  - norm-declaration-must-convict
---

# Nomos

A **nomos** is one keyed, versioned, executable article of world-law — the grain at which an algorithm has identity, epistemic tags, and a store key.

## Formal Expression

A **nomos** (pl. *nomos*, invariant — #lexicon/term/nomos; `DECISIONS[nomos-plural-invariant]`, `:by joseph`) is the memoizable invocation unit that embodies one article of world-law. Concretely:

1. **Identity.** It has a stable name and a version string. The pair mints the **key stem** (`NomosDecl::key`); callers fold in seed, coordinates, and upstream identities to form the complete key ( #form-complete-content-addressed-key ). An undeclared algorithm has no sanctioned path into the world-law namespace.
2. **Executability.** It runs a **kernel** (the numerical code) for one **system** (the phenomenon-level role) at a chosen **rung** of that system's fidelity ladder. Dictionary forms: #lexicon/term/kernel, #lexicon/term/system, #lexicon/term/rung.
3. **Memoizability under fated keys.** Evaluation is pure in its keyed inputs ( #post-determinism-as-ontology): same complete key ⇒ same bytes; content-addressed memoization changes cost, never the world.
4. **Declaration.** Every nomos declares itself in the **nomotheke** (the registry of nomos-as-data). The declaration is the load-bearing self-description, not optional documentation. At minimum it states:
   - **name**, **version** — identity and key stem;
   - **system** — which phenomenon-role this article occupies;
   - **deps** — upstream nomos whose outputs feed this one (nomos→nomos);
   - **consumes** — fluxed quantities required in (role-level needs, independent of who supplies them);
   - **promises** — quantities handed forward, each with an explicit **conservation** claim (conserved / exports-at-boundary / not-tracked);
   - **epistemic tags** — Earth-history fidelity, physics fidelity, relation type, and implementation status (including approach code), as fields on the declaration; the dictionary form #lexicon/term/epistemic-axes is still `:status open` (do not treat that entry as settled law);
   - **assumptions** — verbatim anchors into `ASSUMPTIONS.md` for every magic constant the article leans on.
5. **Composition.** A phase is a choreography of nomos; **law** ( #lexicon/term/law) is composed of them. A nomos is not the whole phase ladder, not the store, and not a view.

**Version honesty.** Versions are part of the complete content-addressed key. Hand-stamped version strings are the present practice and remain a **wish** under #norm-declaration-must-convict until they are derived from kernel source (or an equivalent build-time mechanism that fails when the algorithm changes without a new key). Source-derived versions are open design ( #lexicon/term/generator-pinning; DESIGN-REDUX pervasive-memoization target); they are not claimed as built.

**Representation.** The contract is representation-agnostic: a field on a grid, a global reservoir/box, a network, or another lawful shape may be a nomos so long as identity, declaration, and keyed purity hold.

## Epistemic Status

**Max attainable: exact** as a project definition. The headword and plural are Joseph-settled ( #lexicon/term/nomos `settled`; `DECISIONS[nomos-plural-invariant]`, `:by joseph`). The declaration surface is live code in `crates/vivarium-world/src/nomotheke.rs` and is already enforced on several paths (unique names, registered deps, assumption anchors, consumed⇒in-deps). Stage `draft`.

**Not claimed here:** the true-to-physics declaration boxes as *earned content* — their **fields** are mechanized on `NomosDecl` with structural tests ( #sketch-nomos-declaration-boxes FE(3), 2026-07-24), but per-box earning procedures and content conviction beyond shape remain open; the generic pull-query engine beyond hand-written per-nomos paths; bit-perfect source-derived versioning.

## Discussion

Calling something a nomos is the claim that it is *one addressable article of law*, not a free-floating routine. The store, the flux audit, and the fidelity pyramid all read that grain. Confusing a nomos with a phase, a system, or a kernel blurs three different units (choreography / role / code). Confusing it with "whatever is in the demo loop" is how undeclared algorithms re-enter world-law without a key or a conservation claim.

## Working Notes

- Follow-on segments (not this definition): nomotheke as registry claim; complete content-addressed key; flux web / consumes–promises audit; declared-vs-derived weakest-link fold; source-derived versions; NOMOS-CONTRACT boxes ②–⑤ as convictable surfaces.
- **Ice once this lands (definition dual-homes only):**
  - `.archive/VIVARIA-DEFINITIONS.md` §"Layer 1 — the nomos" — primary archive prose of "keyed, versioned, executable article of world-law"; ice the definitional gloss (scaffold udon sketch may still inform a later nomotheke/udon segment).
  - `.archive/README.root.md` opening nomos paragraph — dual-home of the same gloss for the old front door.
  - `doc/ARCHITECTURE.md` §9 clause (5) (nomotheke declaration fields) — reduce to a one-line pointer to this slug when the §9 "adding a system" contract is segmented; leave R/L/closure and probe clauses until their own homes.
  - `doc/design/DESIGN-REDUX.md` §12 Nix/"recipe" = nomos gloss — definitional identity only; keep the memoization and over-key discipline until the complete-key segment lands.
