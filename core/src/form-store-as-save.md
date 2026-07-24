---
slug: form-store-as-save
type: formulation
status: exact
stage: draft
depends:
  - form-complete-content-addressed-key
  - post-determinism-as-ontology
  - def-vivium
---

# The store is the save

The content-addressed memo store is the world's portable saved state — not a disposable system cache beside a separate save format. Copy the directory and the vivium moves.

## Formal Expression

1. **Save ≡ memo store.** A #lexicon/term/vivium's durable bytes are the content-addressed store: immutable `objects/<hash>` results and `roots` that map complete keys ( #form-complete-content-addressed-key ) to those objects. There is no second "save game" format that holds a different truth. Dictionary face: #lexicon/term/memo (settled — the save-file *is* the memo store).
2. **Portable directory shape.** Operational form is a self-contained, git-repo-shaped directory:
   - `manifest` — store-format version, world seed, provenance, pinned algorithm versions (target shape; partially present in practice);
   - `objects/<hash>` — immutable memoized results;
   - `roots` — current hash per complete key / (aspect, region, tier, time, …);
   - `mutations/` — append-oriented log of irreducible user/agent edits (shape reserved; the log is first-class, not an afterthought).
   Algorithm identity lives inside each object's key, not only in the app binary, so an app release need not invalidate the store by format alone.
3. **Domain-neutral bus.** Keys and values at the store layer are opaque. Terrain tiles, box reservoirs, and future agent state share one put/get interface; meaning lives in nomos declarations and query paths (`store.rs`).
4. **Hit is matured state.** A store Hit for a complete key is matured world state for that key — not a disposable cache. Miss → compute under pure keyed inputs ( #post-determinism-as-ontology ) → put. Revisits do not re-seed from raw prior when the memo still matches.
5. **Invalidation vs eviction.** **Invalidation** is correctness (content-hash / complete-key mismatch — never OS mtime). **Eviction** is space: immutable blobs may be deleted; cost is recompute under the same keys, never silent wrongness when keys are complete.
6. **Pervasive memoization default.** Memoize to disk wherever expected recompute exceeds store round-trip, at every tier and rate — not only at phase freezes. Safety against iteration is **complete keys**, never cache-disable, manual flush-as-workflow, or "wait until algorithms settle" ( #form-complete-content-addressed-key ; Joseph 2026-07-09 pervasive-memoization directive).
7. **Regenerable vs irreducible (frame only).** State *derivable* from seed + macro + nomos versions may be re-materialized; state whose lineage includes user/agent mutation is **irreducible** and must not be treated as free cache. Thin save (law + irreducible + roots) vs fat save (also ship materialized regenerable state) is policy over that split, not a second store. Exact GC and thin/fat algorithms remain open — this formulation reserves the split.
8. **Run-mode root discipline (frame).** Only runs allowed to author authoritative world state write canon `roots`. Discardable iteration must not promote into canon by accident ( #lexicon/term/run-modes — referents pinned, names open). Mechanization of that guard is compliance debt; Phase-0 convention + provisional banners are present practice.

## Epistemic Status

**Max attainable: exact** as architecture law under complete keys and determinism-as-ontology. Live substrate: `crates/vivarium-world/src/store.rs` (objects + roots; domain-neutral; atomic temp-then-rename put). Stage `draft`. Sources: graduated DESIGN-REDUX §13 (`.super-archive/from-design/`), ARCHITECTURE §5, store module honesty.

**Known incomplete (MVP / compliance debt — not a soften of the law):**

1. **64-bit FNV-1a** is not collision-safe at scale; module docs require a stronger hash before unrecomputable artifacts (BLAKE3 is engineering advice, **not** a DECISIONS entry).
2. **No GC, no full manifest, no run-mode enforcement** yet (honest flags in `store.rs`).
3. **Regenerable/irreducible split and mutation-log schema** remain shape-reserved (TENTATIVE) — do not over-prescribe.
4. **Provisional / waived builds** write a third-line `provisional` flag on roots (`PutOpts`); status census surfaces counts. Still open: run-mode canon-root guard beyond the provisional honesty bit; Hit path does not yet expose the flag in `Source` ( #form-builder-admission ).

## Discussion

Treating the store as "just a cache" reintroduces a second home for world truth and makes pickup-on-another-machine a replay lottery. Identity of save and memo is what lets explorers, builders, and probes share one bus of content-addressed facts. Complete keys are the *correctness* half of memoization; this segment is the *ontology of persistence*. Sibling claims: #form-builder-admission (who may write), run-modes carve, #lexicon/term/generator-pinning.

## Working Notes

- **Sole home.** Do not reintroduce `form-save-is-memo-store` or `form-store-is-save` as parallel slugs.
- **Dual homes demoted:** DESIGN-REDUX §13 (file graduated `.super-archive/from-design/DESIGN-REDUX.md`); ARCHITECTURE §5; `store.rs` / `query.rs` module docs; plan headers.
- **Residual / TENTATIVE shapes reserved from graduated REDUX §13 (not FE):**
  - **Thin vs fat save policy** — thin = nomoi + irreducible + roots (regenerate rest); fat = also ship materialized regenerable for instant / version-drift-robust pickup; natural “fat for developed regions, thin elsewhere.” Exact policy unfixed.
  - **Regenerable bit derivation** — how the regenerable/irreducible tag is computed; GC that prunes regenerable freely on algo-version bump while never touching irreducible.
  - **Mutation-log schema** — append-only, ordered, keyed by (region, time), pins algo-versions; log is primary, object store supporting cache. Full design deferred; law-closure frontier for edit *effects* → #sketch-detail-abstract-reversion .
- **Residual:** run-mode root guard and regenerable/irreducible GC still compliance debt (body Epistemic Status).
