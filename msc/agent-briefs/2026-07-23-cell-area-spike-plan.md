# Spike plan: true per-cell area into fluvial drainage

*Peer engineer brief for parent session. Not claim canon. 2026-07-23.*

**Intent:** assess whether promoting the already-measured closed-form spherical cell area into `vivarium-world` erosion (or a shared `measure` module) is a clean ~small code spike that **strengthens** `#obs-cube-locked-kernel-bias` / `DECISIONS[drainage-area-uses-a-uniform-cell-area]` without inventing architecture law.

**Code rank:** optional PoC of an owned *measurement* + a *proposed* remedy. Live code is compliance debt / instrument, not the adjudicator. Do not promote a remedy segment as project law from this spike alone.

---

## 1. Verdict

### **GO — small, independent, claim-backed; not “six lines.”**

| Claim in the wild | Independent judgment |
|---|---|
| “~6 lines; closed form exists; independent of open questions” (DECISIONS / ASSUMPTIONS) | **True for the formula.** `Ω = F(X₁,Y₁)−F(X₁,Y₀)−F(X₀,Y₁)+F(X₀,Y₀)` with `F = atan(XY/√(1+X²+Y²))` is already exact in `msc/spike-wavelet-store/src/area.rs`. |
| “Drop it into `erosion.rs:accumulate_drainage` and ship” | **Understates integration.** Real work is: (1) own the formula in-crate, (2) precompute per-cell `A` for the tile, (3) use it in **drainage seed and deposit volume**, (4) bump the erosion nomos version, (5) add a test that can fail. ~**one focused PR / half-day**, not a refactor. |
| “Also fix edge lengths / MFD distances while you’re there” | **NO-GO for *this* spike.** That is the Jacobian *shear* / open router stack (`#obs` FE(1–2), `#form-grid` FE(4)), not the determinant fix. |

**Recommended action:** implement the minimal path below as **code compliance** to a measured bias. Optionally note Working Notes on `#obs-cube-locked-kernel-bias` that the instrument is now in-tree. **Do not** mint a new “true-A is law” segment until Joseph ratifies the DECISIONS `:status proposed` → decided (or a remedy segment with honest `proposed` stage).

---

## 2. What is already true (do not re-discover)

### 2.1 Owned observation

`#obs-cube-locked-kernel-bias` FE(3) — exact as observation:

- Live drainage uses **one** `cell_area = cell_m²` for the whole tile (`sample::cell_size_m`).
- True solid angle varies on the equiangular cube-sphere.
- PROBE 8 / RUN.txt: face centre **0%** overstatement; edge midpoint **+41.20%**; area-weighted mean **+17.810%** bit-identical L5…L13 (no `N`).
- Stream power sees cube-locked fake erodibility up to ~18.8% in `A^m` at `m=0.5`.

Sibling DECISIONS (both `:status proposed`, `:by claude`):

- `drainage-area-uses-a-uniform-cell-area`
- `cell-size-m-is-a-uniform-area-and-erosion-eats-it` (proposes `measure.rs`)

`#form-grid-equiangular-staggered` Working Notes already name the promotion path: *“Closed form … `area.rs` — promote to `measure.rs` when wiring.”*

### 2.2 Live call sites (verified)

| Site | Uses `cell_m²` as area? | In this spike? |
|---|---|---|
| `erosion.rs` `accumulate_drainage` — runoff seed | **Yes** (`cell_area = self.cell_m * self.cell_m`) | **Yes — primary** |
| `erosion.rs` `deposit` — volume `Δh · A`, `G·Qs/A` | **Yes** (same uniform `area`) | **Yes — same PR** (else height↔volume is inconsistent with drainage `A`) |
| `erosion.rs` D8 / MFD / talus **distances** | uses `cell_m` / `cell_m·√2` as **length** | **No** (router / edge metric) |
| `erosion.rs` `creep` — `κ/cell_m²` | length-scale diffusivity coefficient | **No** (different physics; unprobed as bias) |
| `water.rs` `area = l*l` | uniform | **No** (sibling; out of scope) |
| `sea_level.rs` pour areas | uniform sphere/face share (commented “good enough for datum”) | **No** |
| `sample::cell_size_m` | **one length per level** — still legitimate as *characteristic* Δx | **Keep**; stop *squaring* it for cell area |

### 2.3 Formula home today

- Canonical closed form + geodesic edge helpers: `msc/spike-wavelet-store/src/area.rs` (depends on `vivarium_world::sphere::{CubeCoord, Face}` already).
- Duplicate triangle solid-angle in `examples/redteam_probe.rs` (coarser, not the closed form) — do **not** treat as the source to promote.

---

## 3. Minimal file plan

### In scope

1. **`crates/vivarium-world/src/measure.rs`** (new)
   - Port only what drainage needs:
     - `corner_uv(i, j, level)`
     - `cell_solid_angle(face, i, j, level)` — closed form (face arg unused; keep for API honesty)
     - `cell_area_m2(face, i, j, level, radius_m)`
   - Unit tests *in this module* that re-pin PROBE 8 numbers (see §4).
   - **Do not** port edge-length / MRA / control-subdivision helpers unless a second spike asks.

2. **`crates/vivarium-world/src/lib.rs`**
   - `pub mod measure;`

3. **`crates/vivarium-world/src/erosion.rs`**
   - Add `cell_area: Vec<f32>` on `Fluvial` (or compute once into a private field at `from_surface` / `from_region`).
   - At construction, for each local `(x,y)`:
     - `gi = oi + x`, `gj = oj + y`
     - `cell_area[i] = measure::cell_area_m2(face, gi, gj, level, R) as f32`
   - `accumulate_drainage`: `drainage[i] = self.cell_area[i] * precip_weight[i]`
   - `deposit`: replace every uniform `area` with `self.cell_area[i]` (volume out of incision and volume back into height).
   - Keep `cell_m` for length-using paths (slope, talus, creep) **unchanged**.

4. **`crates/vivarium-world/src/nomotheke.rs`**
   - Bump `EROSION.version` (e.g. `erosion-2026-07-23a-true-cell-area`). Content-addressed memo must miss old tiles; do **not** silently reuse pre-fix erosion bytes under the old key.

### Explicitly out of scope

- MFD `dist = cell_m / cell_m·√2` → geodesic centre-to-centre or edge length.
- Router replacement / edge-only FV (`#form-grid` FE(4) open).
- `water.rs` / sea-level pour metric.
- Wavelet store / leaf-only evolution / `pin_block_means` retirement (those are the big DECISIONS package; true `A` feeds them later).
- New core claim segment for the *remedy* (observation already owns the defect).
- Retiring `sample::cell_size_m` (still a fine characteristic length).

### Optional hygiene (same PR if cheap; else follow-up)

- Spike `area.rs` becomes a thin re-export or comment “canonical home is `vivarium_world::measure`” — only if you touch the spike; not required for go.
- `ASSUMPTIONS.md` drainage-area row: “fix lives in measure; erosion consumes it” — **only if** parent wants bookkeeping; code + nomos version are the durable trail.

---

## 4. Tests that can convict

A test that only checks “channels still form” does **not** convict the bias.

### 4.1 Module tests (`measure`) — geometry

| Assert | Fail condition |
|---|---|
| Face-centre cell at L9 (or any L): `cell_area_m2 / cell_size_m² ≈ 1` within ~1e-4 relative | Wrong formula or wrong corner convention |
| Edge-midpoint `(i=0, j=n/2)`: uniform overstates true A by **~41.2%** (`ua/ta − 1 ≈ 0.412`) | Sign error or `i,j` off-by-one |
| Area-weighted mean overstatement over a whole face at two levels (e.g. L5 and L7) is **equal** to ~**+17.810%** within float noise | “Noise not bias” / level-dependent bug |
| Face independence: `ZPos` vs `XPos` same solid angle for same `(i,j,level)` | Accidental face-dependent path |

These re-home PROBE 8 as an **in-crate failing test**, not a one-off spike binary.

### 4.2 Kernel test (`fluvial_tests`) — drainage seed

Minimal, order-independent of fluvial physics:

1. Build a tiny `Fluvial` (or call a package-visible helper) at a level where the tile spans centre→edge **or** place two single-cell probes:
   - tile origin at face centre region vs edge-midpoint region, `nx` small, flat `h`, uniform precip, **one** accumulate step (or inspect seed before routing).
2. Assert `drainage[i] == cell_area[i]` (flat field, no upstream) — not `cell_m²`.
3. Assert edge-region seed **&lt;** centre-region seed for same level (true A smaller near edge midpoints).

If someone reverts to `cell_m * cell_m`, this fails hard.

### 4.3 Regression (expect pass; not the conviction)

| Test | Expectation |
|---|---|
| `deterministic_bit_identical` | Still bit-identical across two runs |
| `channels_concentrate_and_stay_finite` | Still forms network; update threshold to use `mean(cell_area)` or `max_d > 50 * max(cell_area)` so it is not accidentally coupled to the fake area |
| `pin_preserves_parent_means` | Likely still under 2 m tol; **do not** treat it as a golden of pre-fix heights |
| `test_footprint_is_actually_land` | Unchanged (height seed, not A) |

There are **no bit-golden height vectors** in-tree for fluvial output; the real memo risk is **on-disk store roots** keyed by old `EROSION` version — fixed by the version bump.

---

## 5. Risks

| Risk | Severity | Mitigation |
|---|---|---|
| **Stale erosion memos** look like “physics didn’t change” | High if version not bumped | Bump `EROSION.version`; old roots orphan harmlessly |
| **Absolute incision rates shift** (edges less over-eroded) | Expected, good | Re-eye demos; do not retune `k_dt` in the same PR unless a probe demands it |
| **`deposit` left on uniform A** while drainage uses true A | Medium inconsistency | Same PR, both sites |
| **`channels_concentrate` threshold** uses `cell_m²` | Low false-negative risk | Threshold on true area stats |
| **`pin_preserves_*`** height field shifts | Low (tol 2 m; test already approximate) | Run suite; do not tighten tol to freeze the bug |
| **f32 storage of area** | Negligible at L19 | Compute f64, store f32 |
| **Perf** (tan/atan per cell once) | Negligible vs MFD epochs | Precompute at construct; never inside epoch loop |
| **Scope creep into edge metric** | High social risk | Hard no in this PR; see §6 |
| **Claim overshoot** (“we fixed cube-locked bias”) | Epistemic | Spike fixes **determinant** only; fan/router remain; obs FE(1) still live |

### Goldens / `pin_preserves` / fluvial suite — honest read

- **No float golden** of full erosion tiles in unit tests.
- `pin_preserves_parent_means` does **not** assert the drainage-area invariant; it asserts approximate block means after a known-defective bilinear pin (own DECISIONS: `mean-pin-does-not-preserve-block-means`). True-A will change the eroded field slightly; test should still pass. If it fails, the failure is informative about pin residual growth, not a reason to keep fake A.
- Builder smoke / on-disk worlds: expect **recompute** after nomos version bump.

---

## 6. Face-edge length: wait?

### **Yes — wait.**

Reasons (truth over completeness):

1. **Different defect.** Uniform `A` is the Jacobian **determinant**. Flat-grid `dist` / MFD eight-way fan is **shear + wrong neighbour set**. `#obs` already separates them (FE(1–2) vs FE(3)).
2. **Open router.** Replacing `cell_m`/`√2` with geodesic edge or centre distances **without** retiring Moore diagonals and moving to edge-only flux partially reshapes the fan bias instead of closing the physical claim. Router replacement is explicitly **open** (`#form-grid` FE(4)).
3. **Independence claim is load-bearing.** DECISIONS says true A is independent of every open question — that is only true if we **do not** entangle it with the router rewrite in one spike.
4. **`measure.rs` can grow later.** Porting `east_edge_len_m` / `north_edge_len_m` into the module *API* is fine as dead code only if unused; better leave them in the spike until a router PR consumes them, so this PR stays reviewable.

When edge length *should* land: with gradient-reconstructed **edge** flux (or a declared FV metric package), not as a silent swap inside MFD.

---

## 7. Effort honesty (“six lines?”)

| Piece | Size |
|---|---|
| Closed-form solid angle | ~15 lines (already written) |
| `measure` module + convicting geometry tests | ~80–120 lines |
| `Fluvial` area field + two call sites | ~25–40 lines |
| Nomos version bump | 1 line + comment |
| Fluvial drainage-seed test | ~30–50 lines |
| **Total** | **Small PR**, roughly **~150–250 LOC**, not six lines and not a multi-day architecture project |

If a future agent expands this into “full geometric contract + water + sea_level + wavelet,” that is a **different** program; send them back to this brief.

---

## 8. Success criteria (for the parent session)

Spike **succeeds** when:

1. In-crate tests re-pin PROBE 8 centre / edge / mean numbers and fail if uniform area returns.
2. `accumulate_drainage` + `deposit` consume per-cell true `A`.
3. `EROSION` version bumped; `cargo test -p vivarium-world` green.
4. No claim text asserts the MFD fan is fixed.
5. No new architecture law segment required for merge.

Spike **fails / no-go abort** if:

- Implementation requires changing face topology, projection, or router to get area right (it should not — formula is face-local closed form).
- Review pressure forces edge-length / water into the same change set without a separate brief.

---

## 9. Sources read for this plan

- `core/src/obs-cube-locked-kernel-bias.md`
- `core/src/form-grid-equiangular-staggered.md` (Working Notes promotion path)
- `msc/spike-wavelet-store/src/area.rs`, `probes.rs` PROBE 8, `RUN.txt`
- `crates/vivarium-world/src/erosion.rs` (accumulate / deposit / creep / fluvial_tests)
- `crates/vivarium-world/src/sample.rs` (`cell_size_m`)
- `crates/vivarium-world/src/nomotheke.rs` (`EROSION` version)
- `DECISIONS` entries `drainage-area-uses-a-uniform-cell-area`, `cell-size-m-is-a-uniform-area-and-erosion-eats-it`
- `ASSUMPTIONS.md` drainage cell area row

---

## 10. Bottom line for the parent

**Do the spike.** It is the clean unpaid half of a measured cube-locked bias, already pointed at by segments and DECISIONS, with a formula that does not depend on open grid politics.

**Call it what it is:** ~half-day engineering, not six lines; **area only**; edge length waits for the router. Code is secondary — the win is an instrument and a compliance path under an observation that is already exact.
)
