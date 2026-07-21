---
slug: form-complete-content-addressed-key
type: formulation
status: exact
stage: draft
depends:
  - post-determinism-as-ontology
  - def-nomos
---

# Complete content-addressed key

Every memo is keyed by everything that affects it; an incomplete key is silent corruption, so over-key never under-key.

## Formal Expression

1. **Complete key.** A #lexicon/term/memo is addressed by a content key that folds in *everything that affects the value*: upstream input identities (hashes), coupling parameters, seed, and the producing #lexicon/term/nomos's identity and version (plus resolution, time, and other coordinates when they change the bytes). Omitting an input that changes the output is under-keying.
2. **Asymmetry.** Under-keying serves a stale memo that *lies* (silent corruption). Over-keying costs only recompute. When the choice is forced: **over-key.**
3. **Key, never caution.** Memoization must not be "kept safe" by disabling the cache while iterating, by manual cache-clear as a workflow step, or by deferring caching until algorithms "settle." The only lawful mechanism that keeps memoization from interfering with iteration is a complete key. (Joseph, 2026-07-09 pervasive-memoization directive — carried in design source; the over-key cost asymmetry itself is the correctness trade, not a separate named decision.)
4. **Nomos version participates.** Algorithm identity is part of the key ( #lexicon/term/generator-pinning — open as a term, practice already assumed). Hand-stamped version constants are present practice; source-derived versions (build-time hash of kernel source, coarse enough to cover transitive in-crate deps) are the target remedy so human bump-discipline is not load-bearing. Optional IR-normalized hashing is non-dogmatic tightening, not settled law.
5. **Determinism is the precondition.** Memoization is sound only because law and aleatoric detail are pure functions of keyed inputs ( #post-determinism-as-ontology , #lexicon/term/fated-noise ). Unkeyed entropy makes every key incomplete by construction.

## Epistemic Status

**Max attainable: exact** as architecture **law** under determinism-as-ontology. The law is falsified if we treat an incomplete key as complete, or treat cache-disable / manual flush as the iteration safety valve. **Present practice can under-key without falsifying the law** — that is compliance debt, not a warrant to soften the claim (strengthen-before-soften: fix the keys and the guards first).

**Known incomplete surfaces (compliance and MVP debt; not a soften of the law):**

1. **Hand-stamped nomos versions.** Store keys take a version string at construction; fill paths still depend on human-bumped constants of the `FILL_ALGO_VERSION` class. A forgotten bump mid-iteration serves a stale memo that lies. Source-derived nomos-version hashing is designed and queued; it is not yet the mechanism in force.
2. **64-bit FNV-1a is not collision-safe at scale.** The store's content hash is MVP-grade FNV-1a; the module docs state it is fine for demo object counts and **not** collision-safe at scale. Replacing it before the store holds unrecomputable artifacts is named engineering advice in code — **not** a DECISIONS entry. Do not treat BLAKE3 (or any specific successor hash) as decided unless and until DECISIONS records it.
3. **Completeness is doctrine more than full enforcement.** Under-keying is the named unsafe failure. Nomotheke pins consumed⇒in-deps; a dep-identity-in-key test is the next guard. Live under-keying (e.g. a dep version omitted from a consumer key so a bump serves stale hits) is a **bug against this law**, not evidence the law is wrong.

Stage `draft`. Sources for this extraction: ARCHITECTURE §5, DESIGN-REDUX §11–13 (key and memoization clauses), `store.rs` module honesty — none of those remain claim homes once this segment stands.

## Discussion

This is one of the three walls that keep parallel system work and fidelity-ladder swaps cache-transparent: change a nomos version and exactly its dependent cone invalidates; everything else stays. Scratch and canon can share one object store because different keys never collide by construction. Sibling store claims (save-file *is* the memo store; pervasive disk memoization as economic default; run-mode root discipline) are not this segment — they need their own homes. Generator-pinning is the dictionary face of version-in-key; this segment is the completeness law those pins serve.

## Working Notes

- **Ice once this is home.** ARCHITECTURE §5 complete-key paragraph and spine-invariant (3) pointer-to-§5 → one-line claim home ` #form-complete-content-addressed-key `. DESIGN-REDUX §12 key-completeness / over-key / manual-version weak link / "key, never caution" clauses → same. Leave §11 (lazy query-graph runtime shape), §12 ladder methodology that is not key law, and §13 save-directory layout in source until those claims have segments.
- **Adjacent unsegmented.** Save-file-is-memo-store; pervasive disk memoization as directive; #lexicon/term/generator-pinning as definition segment; store MVP restrictions (no GC, no manifest, no run-mode enforcement) as observation; complete-key completeness-unenforced as observation.
- **Do not invent.** BLAKE3 and IR-hash are not decided. Over-key is the cost asymmetry of the formulation, not an extra Joseph seal beyond the 2026-07-09 key-not-caution directive.
