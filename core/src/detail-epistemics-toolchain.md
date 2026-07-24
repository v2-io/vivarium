---
slug: detail-epistemics-toolchain
type: detail
status: robust-qualitative
stage: draft
depends:
  - post-determinism-as-ontology
  - form-complete-content-addressed-key
  - form-store-as-save
  - norm-probes-before-claims
---

# Epistemics toolchain — structural vs harness tooling

What to adopt early because it shapes habits (structural) vs what to bolt on when there is something to measure (harness). Queue and skip-reasons from the 2026-07-11 package verification; status of wired items updated when known.

## Formal Expression

1. **Standing principle.** Two kinds of tooling: **structural** (types, lints, key discipline — shapes what can be written; adopt before habits form) and **harness** (external checkers — measures what was written; adopt when there is something to measure).

2. **Adopted / adopt now.**
   - **Clippy disallowed-lists** for determinism bans — **wired** (`clippy.toml`; `bin/check` with `-D clippy::disallowed_methods/types` on `vivarium-world` lib). Identity minting is the sole allowed `SystemTime::now`.
   - **`#[must_use]`** on conservation-carrying returns.
   - **Typed index collections** as new id types appear (`TiVec`).
   - **`indexmap`** when map semantics need insertion-order-deterministic iteration.

3. **Adopt later (sequenced).** `cargo-mutants` after probe suite matures; `proptest` for pure algebraic seams (not kernel physics); `kani` for integer kernels only (avoid float geometry).

4. **Skip (recorded).** `uom` (no runtime Exactness analog; opposite to `Quantity`); `dimensioned` (stale); `insta` (value snapshots train wrong habit vs invariant probes); heavy proof tools (`creusot`/`prusti`/`verus`) disproportionate for hand-tuned f64 kernels.

5. **Highest-value unbuilt structural item.** Build-time nomos-version hash (`build.rs` source-hashing into keys) under #form-complete-content-addressed-key / #form-store-as-save .

## Epistemic Status

**Max attainable: robust-qualitative** for adoption judgment; package versions were verified 2026-07-11 and may drift. **Currently `robust-qualitative`.** Stage `draft`. 
## Discussion

PROCESS norms bind ahead of tooling; this segment is the package map those norms point at, so cold re-litigation of "should we use uom?" stops.

## Working Notes

- Live PROCESS may cite this slug instead of `#detail-epistemics-toolchain`.
- `fmt-md` is project-wide Archema markdown discipline (unwrap + optional `--math`); not listed in the 07-11 queue — use on prose edits.
