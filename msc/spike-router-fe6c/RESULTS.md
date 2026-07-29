# FE(6c) pricing — RESULTS

*Ran 2026-07-29. Harness: `crates/vivarium-world/examples/router_fe6c.rs` (new,
single file, outside the source digest). Predictions committed first:
`PREDICTIONS.md`. Raw console of the scoring run: `RESULTS-run.txt`; the earlier
runs that failed gates are kept as `RUN-1..7.txt` because two of them are the
finding.*

## The question

`#obs-routing-curl-spiral` FE(8) convicted the **receiver/incision tree**'s
diagonal treatment as landscape-consequential (CUBE +0.07 → +0.26) using a
**naive D4 steepest-descent** tree, which its own RESULTS names a strawman. The
owed measurement is FE(6)(c): the Coatléven flux-vector reconstruction
`Q_K = (1/|K|) Σ_σ F_{K,σ}(x_σ − x_K)` driving the tree — with FE(6)(d), the
strongly consistent gradient reconstruction that Coatléven 2020 Def. 4.2 makes a
*hypothesis* of Thm 6.1, priced as its own arm rather than folded in.

## Gates (all green in the scoring run — two of them failed first and mattered)

- **P0 — harness `LiveMfd` reproduces live `erosion::Fluvial::erode` bit-for-bit**,
  3× at both sites. *It failed on the first run with max|Δh| ≈ 68 m*, because the
  live kernel had moved since 2026-07-24 (`0780feb` "The fill becomes a routing
  surface" and `1c1c5a1` "the lake claim lands": `fill_depressions` now returns a
  standing-water field, incision skips submerged cells, deposition traps sediment
  in lakes, and the fill is *undone* before talus/creep). **`examples/router_pricing.rs`
  is stale against the live kernel for the same reason** — see Adjacent findings.
- **P1 — the geometric identity** `|K|·Id = Σ_σ |σ|(x_σ−x_K)⊗n̂_{K,σ}`, which is
  what makes the reconstruction exact: **8.0e-16** worst-cell Frobenius residual
  against the tangent-plane quad's own area. *It failed on the first run at 4.5e-1*
  and caught a real bug (unit bearings used where geodesic displacements were
  needed). DERIVATION §6 listed "the identity is EUCLIDEAN and our cells are
  SPHERICAL" as something that could kill the whole approach — **measured, and it
  does not**: at L19 the tangent-plane quad and the spherical cell agree to ~3e-6,
  and what limits that agreement is not sphericity but arithmetic (below).
- **Determinism** — all three reconstruction arms bit-identical across 3 reruns.
- **Not a no-op** — land fraction 88.0–88.5%, channels form at every threshold.
- **Replication** — `EdgeFull` (the strawman) returns **+0.073 / +0.160 / +0.250**
  at τ = 20/50/100 against 2026-07-24's **+0.070 / +0.148 / +0.255**. The prior
  headline replicates across a changed kernel, a corrected terrain and 8 seeds.

## The two validity findings that come before any number

### 1. The metric had never been shown to return zero

CUBE is a difference of differences of a *fraction* over a few thousand channel
cells. Nothing in the prior experiment established how much of a given CUBE value
is terrain realisation. This run adds two controls it lacked:

- **A seed sweep** (8 independent seeds) giving every number a ± sd.
- **A null-pair control**: CUBE computed over **two D4-symmetric footprints**
  (face centres on ZPos and XPos). The cube-locked term vanishes under D4
  symmetry, so every arm must come back at zero. **Every arm does** — |mean| ≤
  0.047 against sd 0.01–0.10, i.e. consistent with zero for all eight arms at all
  three thresholds. *The metric returns nothing when there is nothing to find.*
  That is the first evidence CUBE measures what it is named for.

The null pair also sets the floor: **RMS CUBE below ~0.01–0.03 is not
distinguishable from nothing.**

### 2. The prior experiment's terrain was a bare analytic paraboloid

`router_pricing.rs`'s `terrain()` — inherited verbatim into the first version of
this harness — documents itself as "broad paraboloid **+ real band-limited prior
detail (natural channel seeding)**". The detail term is

```rust
gen::initial_topography_m(seed, c, c.level()) - gen::initial_topography_m(seed, c, level)
```

and the harness builds every cell **at `level`**, so `c.level() == level` and the
term is **identically zero**. There is no prior detail, and the only seed
dependence left is an additive datum (which is not inert, because the implicit
incision update is not translation-invariant).

Worse, **seed 0 — the seed the prior experiment used — carries no band-limited
relief at all**: measured, the tectonic surface has zero spectral content between
levels 14 and 19 at that seed. So the channels being scored formed on a smooth,
near-radially-symmetric dome, where **the router's own lattice bias is close to
the only symmetry-breaking present**. That is the same species of hazard as the
face-centre cone null test this project has already been bitten by
(`#norm-probe-sensitivity`).

Fixed here (band between `level−5` and `level`, ≈3.5 m sd of relief; seeds 1–8).
**Everything below is on corrected terrain.** The paraboloid runs are kept in
`RUN-5/6.txt` because they produce a *cleaner and different* story than the real
terrain does — which is exactly why they should not be reported as the result.

## The measurement

Arms, all against the live kernel as baseline. `EdgeTrue` is the same-fan control
so a tree comparison is a tree comparison.

Summary statistic: **RMS of the mean CUBE across the three thresholds**, beside
each arm's own **null-pair** RMS. Sorted most cube-safe first.

| arm | what it is | RMS CUBE | null floor | ratio |
|---|---|---|---|---|
| `EdgeTau` | 4-fan, \|σ\|·drop/d, live D8 tree | **0.0326** | 0.0087 | 3.8 |
| `EdgeTrue` | 4-fan, drop/d, live D8 tree | 0.0487 | 0.0156 | 3.1 |
| **`CoatGrad`** | **FE(6c)+(6d): Q̂ tree, LSQ face weights** | **0.0551** | 0.0118 | 4.7 |
| `CoatTpfa` | FE(6c) alone: Q̂ tree, two-point face weights | 0.0571 | 0.0100 | 5.7 |
| `GradFan` | FE(6d) alone: LSQ face weights, live D8 tree | 0.0882 | 0.0270 | 3.3 |
| `UniformOld` | retired pre-`6c1ad97` kernel (scream control) | 0.1250 | 0.0092 | 13.5 |
| `CoatMag` | CoatGrad + ‖Q‖·√A consumed downstream | 0.1619 | 0.0104 | 15.5 |
| `EdgeFull` | **naive D4 tree — the strawman FE(8) priced** | **0.1767** | 0.0192 | 9.2 |

Per-threshold means ± sd are in `RESULTS-run.txt`.

### Three present truths

1. **The principled remedy IS ~3× more cube-safe than the strawman, and the
   belief FE(8) recorded as unmeasured is now measured.** `CoatGrad` 0.055 vs
   `EdgeFull` 0.177; per threshold, |CUBE(CoatGrad)| < 0.5·|CUBE(EdgeFull)| at
   **every** τ (0.026 vs 0.073; 0.034 vs 0.160; 0.085 vs 0.250). The strawman's
   large swing was a property of the naive D4 tree, not of removing diagonals —
   a continuous reconstructed direction does not axis-lock the way four fixed
   azimuths do.

2. **But it buys nothing over the cheapest arm in the ladder.** `EdgeTau` — kill
   the diagonals, weight faces by transmissivity `|σ|·drop/d`, and keep the live
   D8 steepest-descent tree — is the *most* cube-safe arm measured (0.033, closest
   to the metric's own floor), with no reconstruction, no gradient operator, and
   no Q̂. On the metric of record, **FE(6c) is not the cube-safety win; killing
   the diagonals is, and the tree can stay as it is.** This does not refute the
   reconstruction — its justification is consistency of the accumulated quantity
   (Coatléven's `q̃_K` has no mesh-independent limit; §1.1), which CUBE does not
   measure — but it does mean *channel orientation* is no longer an argument for
   building it.

3. **Consuming the reconstruction's MAGNITUDE is landscape-consequential at the
   strawman's scale.** `CoatMag` 0.162 vs `CoatGrad` 0.055 — the direction half
   is benign-ish and the magnitude half is not. **Named confound, and it is
   fatal to reading this arm as a verdict on FE(6c):** ‖Q_K‖ is a *specific*
   catchment area (units m), not a catchment area, so consuming it means the
   stream-power law is being fed a different quantity with `k_dt` unchanged. This
   arm converts via `‖Q_K‖·√A_K` (dimensionally an area, reducing to A in the
   constant-flux limit; measured ratio to the raw accumulation p10 0.52 / median
   0.77 / p90 1.01 — an O(1) reweighting, not a rescale). Coatléven's own claim is
   that `‖Q_K‖` *is* "the correct output of a MFD algorithm", which properly means
   restating stream power in specific-catchment-area form with a re-tuned
   erodibility. **That experiment has not been run. FE(6c)'s magnitude half is
   unpriced; only its direction half is priced here.**

### Supporting numbers

- **Affordability (a named open in the census): answered, and it is cheap.**
  `CoatGrad` 0.17 s vs `LiveMfd` 0.16 s — **1.06×**, well inside the 2.5×
  predicted. The LSQ gradient plus the reconstruction cost ~10–25% over the
  4-neighbour fan and land at parity with the live 8-neighbour fan.
- **Halo (the census's 1→2 open): halo 2, derived, not measured.** `Q_K` consumes
  its neighbours' *outgoing splits*, which need those neighbours' weights, which
  need elevation at distance 2. This holds for `CoatTpfa` too — **halo 2 is the
  price of the reconstruction, not of the LSQ gradient.**
- **The tree's irreducible projection residual.** A single-receiver tree must
  project a continuous Q̂ onto one of 8 lattice bearings; the residual angle is
  median 6.5° (corner) / 11.0° (face centre), p90 16.1° / 20.3°. Q̂ was never
  degenerate (0 fallbacks). The corner/centre asymmetry in this residual is itself
  cube-position-dependent and is the mechanism most likely behind `CoatGrad`'s
  non-zero remainder.
- **LSQ fallbacks are rare**: 42 of 9216 cells in the scored epoch (cells where
  the corrected gradient sent nothing through any downhill face and the two-point
  weights took over). Not a hidden second scheme.

### The standing limit, measured and narrowed

FE(6)(c) carries a standing limit: pits/flats/accumulation zones (`s_K = 0`) are
outside Coatléven's well-posedness theory, and Priority-Flood is *how we make
them*, so the pricing had to carve out exactly the cells the fill manufactures.
**It does carve them out, and it does not matter here**: fill-raised cells are
**0.0–0.2% of the tile**, and excluding them moves every CUBE value by < 0.003.

Two reasons, and the second narrows the limit usefully: the live kernel now
*undoes* the fill each epoch, and this configuration is an uplifting dome with
base-level edge sinks, which closes almost no depressions. **The limit is real
but it is not what constrains FE(6c) pricing on a dendritic uplift landscape. It
would bite on an endorheic / closed-basin configuration, which this experiment
does not test** — that is the condition to name when the limit is next invoked.

## Prediction scorecard

| # | prediction | outcome |
|---|---|---|
| P1 | identity residual < 1e-5 | **Confirmed** (8e-16) — but the mechanism I guessed was wrong; the limit is `cell_solid_angle` precision, not sphericity |
| P2 | \|CUBE(CoatGrad)\| < 0.5·\|CUBE(EdgeFull)\| at every τ | **Confirmed** at all three τ |
| P3 | CoatGrad not benign; \|CUBE\| in 0.02–0.10 | **Confirmed** (0.026 / 0.034 / 0.085; RMS 0.055 vs floor 0.012) |
| P4 | (d)'s CUBE marginal < 0.03; field Spearman < 0.90 | **Split**: field confirmed hard (0.204). CUBE marginal confirmed in aggregate (0.055 vs 0.057 RMS) but **refuted as written** — per-threshold \|Δ\| reaches 0.079. |
| P5 | ratio median in [0.3,3.0]; CoatMag within 0.05 of CoatGrad | **Half confirmed** (median 0.77, long left tail as predicted); **half refuted** — CoatMag is 3× CoatGrad |
| P6 | > 10% of land cells fill-raised; carve-out reduces \|CUBE\| | **Refuted** (0.0–0.2%); the carve-out is inert here |
| P7 | CoatGrad within 2.5× LiveMfd; halo 2 | **Confirmed** (1.06×); halo 2 derived |
| P8 | the no-go I was prepared to report | **Not triggered** — P2 held |
| P9 | distrust triggers | (a) **fired** and caught the stale kernel port; (b) clean; (c) replicated |

The one I most want re-attacked: **P4 on the paraboloid runs said the opposite of
P4 on real terrain.** On the degenerate surface, `CoatTpfa` was strongly
cube-locked (+0.090, z = 22) and `CoatGrad` was the only arm consistent with zero
— a clean story that would have made "(d) is what buys cube-safety" the headline,
and that dissolves once the terrain has relief. Had I not gone looking at the
null-pair control, that is the claim this spike would have shipped.

## Adjacent findings (not what I was sent to do; both convictable)

### A. `examples/router_pricing.rs` is stale against the live kernel

Its `LiveMfd` arm no longer bit-matches `erosion.rs` (max|Δh| ≈ 68 m at the corner,
102 m at the centre) because of the lakes/fill-undo work. Its own
`NOTE-to-coordinator.md` predicted exactly this ("if a peer lands another erosion
change, re-run P0 before committing"). Its numbers are not wrong *as measured*,
but they describe a retired kernel — and `#obs-routing-curl-spiral` FE(8) cites
them as present truth. The strawman band replicates on the current kernel (above),
so the FE(8) conclusion survives; the citation should say which kernel.

### B. `measure::cell_solid_angle` lost relative precision as the level refined — fixed

Probe: `crates/vivarium-world/examples/solid_angle_precision.rs`. **Landed in
`src/measure.rs` on 2026-07-29** under Joseph's explicit grant — *"Don't worry
about rekeying — get the truth in place :-) Let the cache worry about the cache."*

The retired closed form evaluated `F(u₁,v₁) − F(u₁,v₀) − F(u₀,v₁) + F(u₀,v₀)` with
`F = atan(XY/√(1+X²+Y²))` — four O(1) terms whose difference is the tiny solid
angle. Relative cancellation error grew like `4^level`.

**⚠ The first version of this finding named the wrong fix, and the reason is worth
keeping.** It reported the error *against naive Van Oosterom–Strackee* and
proposed naive VOS as the drop-in. Measured against a real reference, **naive VOS
degrades as `4^level` too** — 1.8e-3 at L25, only ~7× better than the form it
would replace. The proposal would have bought a factor of 7 while claiming a
factor of a million. The methodological error: *the probe used as its reference a
formula with the same failure mode as the one under test*, so it could measure a
disagreement but could not attribute it.

The reference now is tensor Gauss–Legendre on the Jacobian — a sum of **positive**
terms times an **exactly representable** cell size, hence cancellation-free by
shape, and self-converged to ~2e-16 (the probe reports that convergence, which is
what licenses it as truth). Against it:

| level | cell | legacy (retired) | naive VOS | **landed** |
|---|---|---|---|---|
| L13 | 1222 m | −4.8e-10 | −3.1e-11 | **−5.6e-13** |
| L16 | 153 m | −2.2e-8 | +7.1e-9 | **+2.6e-13** |
| **L19** | 19.1 m | −3.0e-6 | −5.9e-7 | **−1.9e-11** |
| **L21** | 4.8 m | +3.8e-5 | −7.8e-6 | **−5.9e-11** |
| **L23** | 1.19 m | +2.9e-4 | +6.2e-5 | **−2.9e-10** |
| **L25** | 0.30 m | −1.3e-2 | +1.8e-3 | **−4.0e-10** |

The landed form is VOS with the triple product built as `a·((b−a)×(c−a))` —
algebraically identical to `a·(b×c)` but formed from *differences* of nearby
corner vectors. **The difference reformulation is the load-bearing part**, not VOS
as such. Flat from L16 on; six to seven orders better; no longer `4^level`.

Why it mattered: `cell_area_m2` is consumed by the live fluvial kernel
(`erosion.rs:485` → per-cell runoff in `accumulate_drainage`, volumes in
`deposit`), and it *replaced* uniform `cell_m²` precisely because per-cell area
accuracy was shown to matter (`#obs-cube-locked-kernel-bias`, +17.8%
area-weighted bias). At L23–L25 its replacement carried 0.04%–0.5% median and up
to 7% worst-case per-cell noise, spatially high-frequency — the shape that reads
as sub-grid texture rather than as a numerical floor.

**Independent corroboration.** An unrelated gate in this spike's own harness —
P1's `max |A_spherical/A_planar − 1|` at L19, measuring the cell's genuine
sphericity against the tangent-plane quad — moved **4.44e-5 → 5.44e-8** across the
change. Different file, different quantity, predicted direction and magnitude.

**Cost and verification.** Every stored world cohort is invalidated by design
(SRC_HASH, `#form-complete-content-addressed-key`) — accepted per the grant.
`bin/check` green (180 lib + 3 cli_admission + decision-refs + null-space gate +
determinism clippy). The FE(6c) pricing above was **re-run across the change**:
every conclusion and the entire arm ordering survive, RMS-CUBE shifts < 0.007.
The probe carries a hard guard (worst |live/quad − 1| over L13–L25 < 1e-8;
measured 2.5e-9) so a future simplification back to a cancelling form fails
loudly instead of silently inside a landscape.
`DECISIONS[cell-solid-angle-now-uses-a-difference-formed-spherical-excess]`.

### C. Cross-link to the same session's structure strand

`DECISIONS[the-discrete-gcl-is-a-spec-not-a-defect]` (2026-07-29, proposed) records
that `measure.rs` carries **no edge length, edge normal, or arm** — four unbuilt
quantities — and proposes the discrete GCL as their build-time acceptance test.
`router_fe6c.rs`'s `build_geom` is a working implementation of exactly those four,
and its P1 identity gate is an **independent** exact check on them (8e-16). If
those quantities are landed in `measure.rs`, both tests should ride along.

## Limits

1. **One face-pair, one level (L19), one footprint geometry, one dome, 80 epochs,
   uniform uplift.** The corner footprint is maximum shear — a bound, not a
   face-average. Seeds are now swept (8); faces, levels and forcing are not.
2. **The magnitude half of FE(6c) is unpriced** (limit named above): a fair test
   needs stream power restated in specific-catchment-area form with a re-tuned
   `k_dt`, not a dimensional conversion at fixed `k_dt`.
3. **Metric of record is channel orientation.** Consistency of the accumulated
   quantity — the *actual* argument for the reconstruction — is not measured by
   CUBE at all. An arm can be perfectly cube-safe and still be accumulating a
   quantity with no mesh-independent limit.
4. **The Q̂ tree projects onto 8 lattice bearings.** A genuinely continuous
   receiver (D∞-style two-neighbour split) is not tested, because the incision and
   deposition machinery assumes a tree.
5. The reconstruction divides by the spherical `cell_area`, not the tangent-plane
   quad area the identity is stated on. Difference is now ~5e-8 at L19 (it was
   ~4e-5 before the `cell_solid_angle` fix, and that discrepancy is what exposed
   the fix); irrelevant here, would need care at coarse tiers.

## Reproduce

```
cargo run --release -p vivarium-world --example router_fe6c
cargo run --release -p vivarium-world --example solid_angle_precision
```
