# The epistemics toolchain — what mechanically hardens the honesty, and when to adopt it

*2026-07-11, at Joseph's ask ("packages or methodologies we can adopt earlier that will make the project more wise, strong, and beautiful through truth, and that are harder to retrofit later because they inform our habits"). Every version/capability claim below was verified against current docs.rs / repos on 2026-07-11 by a research agent, not recalled from memory. The organizing principle: adopt early exactly the things that shape muscle memory; defer external harnesses freely — they bolt on whenever.*

## Adopt now (habit-forming, near-zero cost, painful to retrofit)

**1. Clippy disallowed-lists — the determinism bans, mechanized.** ✅ **Wired 2026-07-21.** Root `clippy.toml` carries the bans; `bin/check` runs lib tests + `cargo clippy -p vivarium-world --lib` with `-D clippy::disallowed_methods -D clippy::disallowed_types`. Requires `rustup component add clippy`. Identity minting (`WorldSpec::fresh_seed`) is the sole allowed `SystemTime::now` (annotated). Examples/probes still use `HashMap` — not under this gate yet. Full-workspace `-D warnings` (incl. Bevy spikes) is deferred.

**2. `#[must_use]` on `Quantity`, `Unit`, and every conservation-carrying return.** One attribute; catches silently-dropped computed quantities at compile time.

**3. Typed index collections for new id types.** `typed-index-collections` (v3.5.0, active 2026-06): `TiVec<K, V>` prevents a `CellId` indexing a `RegionId` array — the rustc-internal pattern. Adopt as new index-shaped ids appear; retrofitting typed indices onto grown code is exactly the misery this list exists to preempt.

**4. `indexmap` when map semantics arrive** (v2.14, active): insertion-order-deterministic iteration *by construction*, safe for canonical output; pair with a fixed-seed hasher if cross-platform hash-order independence ever matters.

## Adopt later, deliberately sequenced

- **`cargo-mutants`** (v27.1, active) — mutation testing: the meta-check that answers *"would our probes and tests actually convict a corrupted kernel?"* — which is the nomotheke's falsifiability promise, tested. Sequence it right after the fine-tier probe suite matures (it needs something real to measure). Operational gotcha: always `--package vivarium-world`, or every mutant rebuilds the Bevy graph.
- **`proptest`** (v1.11, active) — property-based testing for the pure algebraic seams (`Unit::mul/div`, `Exactness::and`, Kahan residual bounds). Wrong tool for kernel physics — probes own that. Discipline when adopted: commit `proptest-regressions/` like goldens, knowing the files persist *seeds*, not values — best-effort replay, not exact.
- **`kani`** (AWS, v0.66 line, active) — model checking, **integer kernels only**: `splitmix64`/`hash2`/`hash3` no-panic and distribution-structure properties are realistically checkable. Its float support over-approximates `sin`/`cos`/`sqrt` — it would false-flag the trig-heavy geometry, so keep it away from f64 kernels.

## Skip, with reasons recorded (so the question isn't re-litigated cold)

- **`uom`** — the standing "should we?" answered **no** (2026-07-11). Actively maintained and genuinely zero-cost, but architecturally opposite to our `Quantity`: compile-time phantom types with **no analog of the runtime `Exactness` flag**, which is the load-bearing part (the ubit — *mark the guess as a guess*, mechanical). Wrapping uom to add exactness means maintaining two systems for an SI-macro convenience. The honest next rung for `Quantity` is interval arithmetic (`inari`), which its own doc-comment already anticipates — when a consumer needs guaranteed-vs-approximate *bounds*, not just the flag.
- **`dimensioned`** — stale (last release 2022).
- **`insta`** — active and good, but it snapshots values; our probes assert *physical invariants*. Adopting it now would train a weaker habit (diff-watching) in place of a stronger one (invariant-writing). Revisit only for large structured-output regression diffs.
- **`creusot` / `prusti` / `verus`** — all alive, all demanding annotation burden disproportionate to a research crate of hand-tuned f64 kernels; loop invariants over float accumulation are known-hard in every one of them. Revisit Creusot specifically if a conservation bound ever needs *proving* rather than probe-testing.

## The standing principle

Two kinds of tooling: **structural** (types, lints, key discipline — shapes what code can be written; adopt before habits form) and **harness** (external checkers — measures what was written; adopt when there's something to measure). The queue above sorts by that line. The build-time nomos-version hash (`build.rs` source-hashing, DESIGN-REDUX §12) remains the highest-value *structural* item not yet built — it belongs with the regula.rs wave.
