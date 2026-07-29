# Null-space / eigenvalue probe — graduation note

*Spike, 2026-07-24. Audience: whoever decides the standing probe set, and the next field-nomos author who needs a null-space check before shipping. Not claim canon; the claims live in `#form-declared-structure-tradeoff` / `#detail-fvm-control-volume` and the DECISIONS entries this cites.*

## The headline the brief did not know

**The instrument is already built.** The owed "null-space/eigenvalue probe on each field nomos" (`#form-declared-structure-tradeoff` Working Notes; FE(6) of `#detail-fvm-control-volume`) exists as a full, figure-drawing example at **`crates/vivarium-world/examples/null_space/`** (~130 KB across `main.rs` / `linalg.rs` / `ops.rs` / `water_op.rs` / `draw.rs`), authored during the 07-13 spike that produced `DECISIONS[our-kernels-have-no-null-space-the-solitons-were-roll-waves]`. It is Cardiff's recipe (periodic patch → assemble the operator → count zero eigenvalues beyond rigid modes) with both controls, run against every live field kernel we own.

So this spike is **not** "build the probe." It is two smaller, honest things:

1. **Re-validate** the existing instrument against the *current* (2026-07-24) kernels — `water.rs`/`erosion.rs` have moved since 07-13, and a probe's value is that it can still fail.
2. **Close the one real gap** between an *example that narrates* and a *probe that convicts* — and demonstrate the closure with working code.

## 1. Re-validation — the instrument still runs, and the numbers are bit-stable

`cargo run --release -p vivarium-world --example null_space` on `main` (HEAD 8859e11), captured in `full-instrument-rerun-2026-07-24.out`. Every figure matches the 07-13 DECISIONS record to the digits it printed:

| specimen | result | matches 07-13? |
|---|---|---|
| CTRL-BLIND (collocated central, 2Δ) | 4 zeros, **3 spurious**, λ(checkerboard)=0 — INVISIBLE | ✓ |
| CTRL-CLEAN (compact 5-pt Laplacian) | 1 zero (constant), 0 spurious, λ(checkerboard)=−8 (most-damped) | ✓ |
| erosion creep (real `diffuse_step`) | 1 zero, 0 spurious, gap 3.1e5 — CLEAN | ✓ |
| MFD routing (I−Wᵀ) | nonsingular, no closed circulation | ✓ |
| full erosion epoch | 1 zero (total mass), 0 spurious | ✓ |
| water (staggered SWE) | checkerboard depth SEEN (‖A·χ‖/‖χ‖=5.77); collocated twin blind (8e-16); ratio 6.85e15 | ✓ |
| water roll-wave instability | ρ(J)=**1.04011** at k/π=(0.33,0) — long wave, not checkerboard | ✓ |

**The prior conclusion holds, and now on the current tree:** the live water kernel has no null space — its instability is a resolved long-wave (roll wave), not a grid-scale invisible mode; and `water.rs` is a staggered scheme (its most-damped mode *is* the checkerboard). The instrument agrees with `DECISIONS[our-kernels-have-no-null-space…]` at both poles, so it is validated against both a known-bad (collocated: blind, as it must be) and a known-clean (staggered/compact: sees the checkerboard). Bit-stability across 11 days of kernel evolution is itself a small determinism datum.

## 2. The one real gap: the standing guard cannot fail

The full example has a "standing guard" section that runs both controls — but it only **prints** `✓ INSTRUMENT LIVE` and exits 0 regardless (`main.rs:115`, `if zc == 1 && zb == 4 { println!(...) }` with no assertion). A guard that cannot turn the light red is narration, not a probe — exactly the failure `#norm-probe-sensitivity` FE(2) names ("a probe that cannot fail on a known-bad is not yet a probe"). So the instrument as it stands is a superb **report**, but nothing in CI notices if it goes blind.

**Closed here:** `crates/vivarium-world/examples/null_space_gate.rs` — the same core (`linalg.rs` + `ops.rs`, reused verbatim by `#[path]`, no fork) wired as assertions. It **panics / exits nonzero** if CTRL-BLIND stops showing its 3 spurious modes, if CTRL-CLEAN grows a spurious one, or if the live creep kernel loses its clean count. Output in `gate-2026-07-24.out`; final run is green:

```
CTRL-CLEAN  5-pt Laplacian        zeros=1 rigid=1 spurious=+0  gap=6.4e15  checker=8.000e0 (seen)
CTRL-BLIND  collocated central    zeros=4 rigid=1 spurious=+3  gap=8.0e15  checker=0.000e0 (INVISIBLE)
LIVE  erosion creep (f32 kernel)  zeros=1 rigid=1 spurious=+0  gap=3.1e5   checker=8.000e0 (seen)
✓ GATE GREEN
```

**It is genuinely can-fail — I watched it fail twice while writing it**, and both failures taught something worth keeping:

- **Tolerance is part of the probe** (`#norm-probe-sensitivity`, literally). My first cut applied an f64-grade zero-threshold (1e-9·λmax) to the *f32* creep kernel. An exact zero in f32 arithmetic returns at ~1e-7·λmax, so the constant mode fell *above* the cut and the gate falsely read "0 zeros" for an operator that provably has one. The full instrument documents this exact trap (`main.rs:174`, the f32 floor at 1e-5·λmax). The gate now carries a per-operator `tol_rel`: f64 controls at 1e-9, f32 kernels at 1e-5. The cut must match the arithmetic that produced the matrix.
- **A clean cut is judged by daylight, not by a fixed magnitude.** The f32 kernel's clean spectral gap is ~3.1e5, not the ~1e15 of the pure-f64 controls; a fixed 1e6 gap floor wrongly rejected it. The gate now requires ≥4 decades of separation (gap > 1e4), which both arithmetics clear honestly.

## 3. What it would take to make this a standing probe (the real graduation cost)

The gate above is an **example** (`cargo run --example null_space_gate`), so it can be wired into `bin/check` as one line today. That is the cheap 80%. The remaining cost, named precisely so it is not a surprise:

- **The reusable core lives under `examples/`**, which is not compiled into the crate library — so a real `#[test] fn` cannot `use` it. To make the probe a first-class **library test** (fails `cargo test -p vivarium-world --lib`, i.e. the default `bin/check` gate), the honest move is to **lift `linalg.rs` and the control ops from `ops.rs` into the crate**, e.g. `crates/vivarium-world/src/probe/null_space.rs`, exposing:
  - `assemble(nx, kernel_fn) -> DenseOp` (the FD-Jacobian periodic-patch assembler),
  - `zero_spectrum(&op, tol_rel) -> { zeros, spurious, gap, checkerboard_response }`,
  - the two controls (`ctrl_compact_laplacian`, `ctrl_collocated_central`) as the built-in known-bad/known-clean pair.
  Then one `#[test]` asserts the controls discriminate, and each field nomos adds a one-closure test (`assemble(nx, my_kernel)` → assert clean). The 130 KB narrative example stays as the readable report; only the ~200-line core moves.
- **Scope of the guarantee, stated honestly.** This probe convicts *linear* checkerboard/rank-deficiency (null-space) faults on a *periodic patch*. It does **not** see: nonlinear structure loss (the Jensen/entropy cut — that is the commute-probe, a different owed instrument in the same Working Notes), boundary/seam faults away from the periodic interior, or the *quantitative* correctness of a mode it does see (the water roll wave is real but its saturation is wrong — `DECISIONS[…roll-waves]` impact (4), still open). A green null-space gate means "no invisible mode," nothing wider.
- **f32 vs f64 discipline** must ride along: any live kernel row needs the f32-grade tolerance, or it will lie about its own arithmetic. Bake `tol_rel` into the API, not the caller's memory.

## Recommendation

- Wire `null_space_gate` into `bin/check` now (one line) — cheap insurance that the instrument stays live.
- When someone next opens `crates/vivarium-world/src/` for other reasons, lift the ~200-line core into `src/probe/` so the gate becomes a library test and the per-nomos check is a one-closure habit, per `#form-declared-structure-tradeoff` FE(6).
- No segment edit is owed by this spike: the claims (`water.rs` staggered; kernels null-space-free; roll waves are real physics) already live in the two segments and the DECISIONS entries. This note is the instrument's home; the DECISIONS ledger is where a future nomos cites the probe.

## Files

- `crates/vivarium-world/examples/null_space_gate.rs` — the can-fail gate (new; reuses the existing core).
- `crates/vivarium-world/examples/null_space/` — the full instrument (pre-existing, 07-13).
- `gate-2026-07-24.out` — green gate run.
- `full-instrument-rerun-2026-07-24.out` — full instrument re-validated on current `main`.
- Prior: `msc/spike-null-space/{probe-output.txt,fig/}` (07-13); `DECISIONS[our-kernels-have-no-null-space-the-solitons-were-roll-waves]`, `[jarrett-roughness-is-a-positive-feedback…]`.
