---
slug: form-store-is-save
type: aside
status: sketch
stage: draft
depends: []
---

# SUPERSEDED — use `#form-store-as-save` in core

**Not promoted.** Canonical slug is `#form-store-as-save`
(`core/src/form-store-as-save.md`). This draft wording was a third parallel
name; content was merged there. Do not outline or cite this slug.

# The store is the save

The portable world is a content-addressed memo directory: objects and roots, not a separate save blob. Copy the directory and the vivium moves.

## Formal Expression

1. **Save ≡ memo store.** A vivium's durable state on disk is the content-addressed store: immutable `objects/<value-hash>` bytes and `roots/<key-hash>` pointers from complete keys ( #form-complete-content-addressed-key ) to those objects. There is no second "save file" format that holds a different truth.
2. **Domain-neutral bus.** Keys and values at the store layer are opaque. Terrain tiles, box reservoirs, and future agent state share the same put/get interface; meaning lives in nomos declarations and query paths, not in store types (`store.rs`).
3. **Identity of a hit.** A store Hit for a complete key is matured world state for that key — not a disposable cache. Miss → compute under pure keyed inputs ( #post-determinism-as-ontology ) → put. Revisits do not re-seed from raw prior when the memo still matches.
4. **Portability.** Relocating the world directory relocates the vivium (same seeds, same law versions, same objects). Dedup of identical bytes across keys is a consequence of content addressing, not a separate product feature.
5. **What this is not.** Run-mode root discipline (canon vs scratch), GC/eviction policy, mutation logs, and multi-process builder/explorer *scheduling* are sibling claims. This segment asserts only that **truth of computed state is the memo store**, not how admission or explorers are orchestrated ( #form-builder-admission ).

## Epistemic Status

**Max attainable: exact** as architecture law under complete keys and determinism-as-ontology. Live implementation: `crates/vivarium-world/src/store.rs` (objects/roots layout, atomic temp-then-rename put, enumerable root second line = canonical key string). Stage `draft`.

**Known incomplete (compliance / MVP debt — not a soften of the law):**

1. **64-bit FNV-1a** is MVP-grade and not collision-safe at scale (named on `#form-complete-content-addressed-key`; no hash successor is decided).
2. **No GC, no full manifest typing, no mechanized run-mode root guard** — Phase-0 convention and banners; first graduation or multi-process sharing is the tripwire (plan substrate; not decided here).
3. **Provisional / waived builds** can write roots that look like lawful memos (builder `--allow-unmet` residual) — census does not yet tag provisional ( #form-flux-web known incomplete).
4. Design prose still dual-homes this claim (`doc/ARCHITECTURE.md`, DESIGN-REDUX §13); after promotion, those reduce to pointers.

## Discussion

This is the economic half of pervasive memoization: eviction costs recompute, never correctness, so memoization is default rather than "enable when safe." Complete keys are the *correctness* half ( #form-complete-content-addressed-key ); the store-as-save is the *ontology of persistence*. Without both, *in vivia* citation cannot treat a directory as a world artifact ( #def-vivium · #form-in-vivia-citation ).

## Working Notes

- Ice dual-homes once promoted: ARCHITECTURE "save-file *is* the memo store" paragraph; DESIGN-REDUX §13 opening claim; `store.rs` module doc first lines → one-line pointer to this slug.
- Adjacent: `#form-builder-admission` (who may write), run-modes carve (LEXICON / future segment), generator-pinning (version honesty).
- Do not invent: BLAKE3, GC policy, mutation-log primacy — plan/tentative only.
