# De novo project audit — 2026-07-21

## Scope and method

This was an independent code-and-behaviour audit of the live Rust workspace at
commit `3f8754d` (`core: close segment-floor P0s from independent audit`). The
audit covers the workspace members in `Cargo.toml`:

- `crates/vivarium-core`
- `crates/vivarium-world` (including the `vivarium` lifecycle CLI)
- `crates/vivarium-app`
- the three live `spikes/*` members

The excluded `archive/` experiments were not assessed as production code. I
reviewed the persistence, manifest, query graph, builder, registry/audit, and
spatial-boundary paths; searched for failure-prone APIs and unfinished code;
compiled and ran all workspace test targets; and exercised the lifecycle CLI
against a newly created temporary world.

## Executive summary

The project has a strong deterministic core and an unusually clear internal
model of its epistemic limits. `cargo test --workspace --all-targets` passed.
The persistence layer uses atomic replacement and its same-process concurrent
put regression is covered by a test. The principal concern is that the system's
declared operational law is not yet enforced at the point where state is
materialized. In particular, the CLI explicitly reports that erosion cannot run
because a required quantity has no producer, but then runs and persists erosion
and water artifacts anyway.

Three other issues merit attention before relying on a world directory as a
durable, multi-process save: builder exclusion is racy and can become stale on
ordinary failures; persistence failures are swallowed and then presented as
successful computation; and the store's 64-bit, unchecked hashes can silently
substitute an artifact. The latter is acknowledged in source, but remains a
data-integrity defect rather than merely a future enhancement.

## Findings

### P0 — The builder violates the prerequisite gate that `status` declares

**Evidence.** `audit::render_flux_web` emits the categorical message “THIS
WORLD CANNOT RUN FLUVIAL EROSION” when `emerged land` is unmet
([`crates/vivarium-world/src/audit.rs`](../crates/vivarium-world/src/audit.rs#L167)).
The registry says `erosion-tile` consumes that quantity, yet no nomos produces
it. The builder neither queries the requisite audit nor gates its work: it
unconditionally loops over `initial-topography`, `erosion`, and `water`
([`crates/vivarium-world/src/bin/vivarium.rs`](../crates/vivarium-world/src/bin/vivarium.rs#L216)).

**Reproduction.** In a fresh temporary world, `vivarium status` printed the
unmet `emerged land` requisite and the “cannot run” message. Immediately after,
`vivarium build <dir> --level 6 --epochs 1` completed all three phases. A final
`status` census reported six each of `erosion-tile` and `water-tile` artifacts.

**Impact.** The store is presented as the world save and its artifacts are
presented as lawful materialized state. This mismatch lets the CLI create and
subsequently serve artifacts that the project's own formal contract calls
invalid. A viewer or downstream system cannot distinguish them from
permitted artifacts by looking at the store census.

**Recommendation.** Make prerequisite resolution a builder admission check,
not just a report. Before a phase starts, resolve all `NomosDecl::consumes` for
that phase and its dependency closure. Refuse the phase with a non-zero exit
and leave a clear status when a required quantity is unmet. If exploratory
execution is intentionally allowed, require an explicit `--allow-unmet` flag,
record the waiver and unmet quantities in the artifact key/metadata, and make
`status` label the resulting artifacts provisional/invalid. Add an integration
test that proves the default builder cannot write `erosion-tile` roots while
`EMERGED_LAND` lacks a producer.

### P1 — The single-builder lock is a TOCTOU protocol and is leaked on setup failures

**Evidence.** `cmd_build` reads `builder.lock`, decides whether its PID is
live, and then writes its PID with `std::fs::write`
([`crates/vivarium-world/src/bin/vivarium.rs`](../crates/vivarium-world/src/bin/vivarium.rs#L177)).
The write is not exclusive, so two builders can both observe no lock and both
start; the later writer overwrites the lock. In addition, after the lock is
created, the `Store::open` and `build.log` failure paths return without
removing it ([lines 191–203](../crates/vivarium-world/src/bin/vivarium.rs#L191)).
Only the normal bottom-of-function path removes the lock
([line 250](../crates/vivarium-world/src/bin/vivarium.rs#L250)).

**Impact.** Concurrent builds can write the same roots and status file while
both believe they are the one builder. A mundane setup failure (for example,
disk full or a permission change while opening the log) leaves a stale lock;
the next invocation must rely on platform-specific PID reuse detection. The
current `kill(pid, 0)` probe also treats `EPERM` as dead, so it can reclaim a
lock owned by a live process that the caller may not signal.

**Recommendation.** Acquire the lock atomically with `OpenOptions::create_new`
(or a well-tested file-lock implementation) before any build work. Keep the
open lock handle for the process lifetime; include a random build token and
process start identity in lock/status data rather than using a bare PID. Put
cleanup in an RAII guard so every return and unwind attempts removal only when
the guard still owns the token. Treat `EPERM` from the liveness probe as
“possibly alive,” not stale. Add a two-process integration test proving only
one builder reaches the compute loop and tests for cleanup after each setup
failure.

### P1 — Query methods discard persistence errors but report a successful materialization

**Evidence.** Every query's miss path ignores `Store::put` errors with `let _
= ...` and returns `Source::Computed`: hydrosphere
([`query.rs`](../crates/vivarium-world/src/query.rs#L66)), initial topography
([line 107](../crates/vivarium-world/src/query.rs#L107)), uplift, climate,
erosion, and water ([line 363](../crates/vivarium-world/src/query.rs#L363)).
The builder counts `Source::Computed` as completed work and announces that the
store is the save ([`vivarium.rs`](../crates/vivarium-world/src/bin/vivarium.rs#L229)).

**Impact.** Read-only directories, full disks, I/O errors, and rename failures
yield a successful process exit and a log claiming computed tiles even though
there is no durable artifact. On the next run they recompute. This violates the
published “compute + memoize” and “store is the save” contracts and can conceal
operational data loss.

**Recommendation.** Change public query APIs to return `io::Result<(T,
Source)>` (or a domain error that preserves I/O context) and propagate errors
through the builder as a failed phase/status. If a best-effort cache is desired
for a particular interactive view, expose it as a separately named mode and
never return the same success signal used for persisted results. Add a test
using a deliberately unusable store path that asserts no result is labelled a
persisted computation and that `build` exits non-zero.

### P1 — The content-addressed store can silently return the wrong artifact

**Evidence.** Store object and root names are a 64-bit FNV-1a digest
([`crates/vivarium-world/src/store.rs`](../crates/vivarium-world/src/store.rs#L31)).
`put` writes objects and roots under only those hashes, and `get` reads the
root-selected object without validating the stored canonical key or a
cryptographic digest ([lines 95–119](../crates/vivarium-world/src/store.rs#L95)).
The source itself calls this “not collision-safe at scale” (lines 17–20), but
there is no collision detection or corruption check.

**Impact.** A key-hash collision overwrites another root; a value-hash
collision aliases distinct values. In both cases queries can report `Hit` and
serve incorrect world state with no indication. Accidental bit rot or a
manually modified root can also redirect a key to any existing object. Because
these bytes are treated as saved simulation state, silent substitution is a
correctness issue even in a trusted local directory. FNV-1a is also unsuitable
if world directories ever cross a trust boundary.

**Recommendation.** Use a modern cryptographic digest (for example BLAKE3 or
SHA-256) for both keys and values. Store and verify the complete canonical key,
object digest, artifact format/version, and expected shape before treating a
read as a hit. On a mismatch, surface a corruption error rather than silently
using the bytes. Plan a store-format migration (the existing manifest `FORMAT`
field is a natural anchor) and add collision/corrupt-root tests.

### P2 — Cached float artifacts have no schema or shape validation

**Evidence.** `decode_f32` accepts any byte slice, drops a trailing 1–3 bytes,
and returns however many four-byte chunks it contains
([`crates/vivarium-world/src/query.rs`](../crates/vivarium-world/src/query.rs#L376)).
The initial-topography, uplift, climate, erosion, and water hit paths return
that vector directly as a successful `Source::Hit` without checking `nx * nx`
or finite values ([lines 115–121](../crates/vivarium-world/src/query.rs#L115)).

**Impact.** A truncated/corrupt object can be accepted as a cache hit. Direct
callers receive malformed tiles; dependent kernels can later index them and
panic, or calculate from a malformed field. The failure becomes remote from the
actual corrupted object and is not recoverable by recomputation because the
bad root remains a hit.

**Recommendation.** Give each artifact a small typed envelope containing a
magic, format version, type/nomos version, dimensions, payload length, and
checksum. Decode as `Result`, require the expected `nx * nx` length and finite
floats, and propagate an explicit corrupted-store error (or remove only the
verified-bad root and recompute in an explicitly recoverable mode). Add tests
for truncation, extra bytes, wrong dimensions, NaN/Infinity, and a root pointing
at a wrong artifact type.

### P2 — Manifest compatibility and concurrent creation are not enforced

**Evidence.** `WorldSpec::parse` reads the `format` field but accepts every
integer; it never compares it with `FORMAT` ([`crates/vivarium-world/src/spec.rs`](../crates/vivarium-world/src/spec.rs#L75)).
`save` always uses a fixed sibling `manifest.tmp` name
([lines 112–120](../crates/vivarium-world/src/spec.rs#L112)), and
`load_or_create` performs a non-atomic load-then-save ([lines 124–130](../crates/vivarium-world/src/spec.rs#L124)).

**Impact.** A future incompatible manifest is silently opened by older code,
and two concurrent `new`/implicit-build invocations can each mint different
seeds after both observe no manifest. Their shared fixed temporary file can
also be overwritten between write and rename. This undermines the guarantee
that a directory has one stable world identity.

**Recommendation.** Reject unsupported manifest formats with a precise error
(and provide explicit migrations). Reuse the unique-temp mechanism from the
store, and make creation atomic: create a manifest/identity lock exclusively,
then load the winner's manifest if another process won. Test concurrent
`load_or_create` calls and unsupported future/past format values.

### P3 — Quality gates are incomplete and checked-in targets currently warn

**Evidence.** `cargo test --workspace --all-targets` passed but emitted unused
variable/mutability/dead-field warnings in several `vivarium-world` examples.
It also reported `block v0.1.6` as future-incompatible: the dependency declares
an uninhabited static that a future Rust release will reject. `cargo clippy
--workspace --all-targets -- -D warnings` could not be run in this environment
because the installed `stable-aarch64-apple-darwin` toolchain lacks the clippy
component.

**Impact.** Warnings make regressions easier to miss, and the transitive
future-incompatibility can eventually block builds on a newer compiler.

**Recommendation.** Install and run clippy in CI, decide whether examples are
required quality-gate targets, and make the desired warning policy explicit.
Update or patch the dependency chain bringing in `block 0.1.6` (likely via the
macOS graphics stack used by Bevy) before the compiler change becomes a hard
failure. Pin the Rust toolchain/components in `rust-toolchain.toml` or CI so
contributors reproduce the checks.

## Positive controls observed

- The deterministic-core tests cover same-seed identity, different-seed
  divergence, terrain continuity, conservation checks, and cache round trips.
- Store writes use sibling temporary files and rename; the implementation
  correctly added per-process/per-write temporary names, with a concurrency
  regression test for identical values under distinct keys.
- The query graph puts seed, coordinates, resolution, epochs, and selected
  upstream versions into relevant keys. The registry has useful structural
  tests for names, dependencies, assumptions, and quantity producers.
- The lifecycle smoke test (`new`, `status`, `build`, `info`) worked on a fresh
  temporary world at level 6, including a coherent post-build census.

## Verification record

| Check | Result |
| --- | --- |
| `git status --short` before audit | clean |
| `cargo test --workspace --all-targets` | passed; warnings noted above |
| `cargo clippy --workspace --all-targets -- -D warnings` | not runnable: clippy component absent |
| `cargo report future-incompatibilities --id 1` | reports `block v0.1.6` |
| Fresh-world lifecycle (`new`, `status`, `build --level 6 --epochs 0`, `info`) | passed |
| Erosion-gate reproduction (`status`, then `build --level 6 --epochs 1`, then `status`) | reproduced P0 contradiction |

## Recommended remediation order

1. Enforce the unmet-quantity gate in the builder (or explicitly model and
   key a waiver); add the integration regression test.
2. Replace the builder lock with exclusive acquisition and RAII cleanup;
   make persistence errors fail the operation rather than disappear.
3. Introduce a versioned, verified artifact envelope and cryptographic
   content/key digests, with a migration plan for existing stores.
4. Make manifest format and creation concurrency safe.
5. Establish CI lint/toolchain hygiene and clear the current warnings and
   future-incompatible dependency.

