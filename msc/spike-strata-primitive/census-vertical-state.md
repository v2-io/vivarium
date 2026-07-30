# Census: what actually holds vertical state at one horizontal address

*A measurement of the tree as it stands, for the strata-primitive spike. Working document — not a claim segment. Every number below carries the file:line or the probe that produced it, and every inference is marked as one.*

**Kernel era for every measured number in this document: commit `9dd86ab`, clean tree, `SRC_HASH=f0a6800a2f0d920e`** — the same source cohort the built store at `~/.cache/vivarium/globe-world` was last carved under (376–384 water roots and 3552 erosion-field roots at that digest). Measurements were taken in a detached worktree at that commit because the shared tree's `src/erosion.rs` was being edited concurrently and had already moved the digest to `d3ec5278bca48314` mid-session; the store's cohort no longer matches the working tree. Probe: `crates/vivarium-world/examples/vertical_state_census.rs` (read-only store handle; `examples/` is outside the `src/` digest, so running it perturbs nothing — `crates/vivarium-world/build.rs` hashes `src` only).

---

## The answer, first

For one horizontal cell in the live default world the crate can produce **eight different numbers for "the solid surface height"** and **nine for "how much water stands here"**, against **five different values of "sea level."** Of all of that, exactly **five f32 per cell** are durable: one each from `initial-topography`, `uplift-tile`, `climate`, `erosion-tile`, `water-tile`. Nothing else about the vertical is stored at all.

**The parent hypothesis is confirmed, and understated.** `column::Column` is not a materialization view at a sampling boundary — a sampling boundary would imply something on the other side consumes it. Nothing does. Its only non-test consumers in the whole tree are two diagnostic examples, and both read it through the deprecated `gen::SEA_LEVEL_M` datum. The correct description is **read-time-synthesized fiction**: `gen::column_from_surface_at_sea` (`crates/vivarium-world/src/gen.rs:38`) invents two strata from a single height, and no kernel anywhere writes a `Stratum`.

But the sharper finding is not about `Column`. It is that **the tree has no column-shaped object that any kernel writes to, and it has three unrelated things called a column** — and the vertical quantity that *is* real per-cell state, standing water, lives in a struct with eight per-cell arrays of which exactly one survives to the store.

---

## 1. The durable layer: five numbers per cell

Every field root in the store is a flat `Vec<f32>`, one value per cell, encoded by the single codec at `crates/vivarium-world/src/query.rs:1087` (`encode_f32` / `decode_f32`). The store itself is byte-opaque and knows nothing of columns by construction (`crates/vivarium-world/src/store.rs:5-11`).

| stored field | minted at | what one cell holds |
|---|---|---|
| `initial-topography` | `query.rs:290` | solid surface height (m), band-limited to the level's Nyquist |
| `uplift-tile` | `query.rs:326` | rock-uplift rate (m/epoch) |
| `climate` | `query.rs:358` | precipitation (m/yr) |
| `erosion-tile` | `query.rs:437` | carved bed height (m) |
| `water-tile` | `query.rs:942` | standing-water depth (m) |

Non-field roots hold global scalars only: `EpochReduction` is four f64 per $(\text{seed}, T_p)$ (`query.rs:71-80`), `Hydrosphere` three f64 (`hydrosphere.rs:47-54`).

**Store census at the era cohort** (verified by direct scan of 116 955 root files): 80 987 `erosion-tile`, 9275 `initial-topography`, 9253 `climate`, 8760 `water-tile`, 6969 `uplift-tile`, 1528 `mantle-thermal`, 24 `hydrosphere`; of the non-field roots, 27 949 `aspect=stage-residual` and 1554 `aspect=epoch-reduction`. There is **no root kind whose payload is a column**, and no root kind carries more than one number per cell.

So the durable answer to *"what does this world know about the vertical at address $x$?"* is a five-tuple of scalars from five separately-keyed articles, with no object that owns them jointly.

## 2. The three things called a column

| type | file:line | fields | who writes it |
|---|---|---|---|
| `column::Column` | `column.rs:30` | `strata: Vec<Stratum>`, `water_depth` | `gen.rs:52`, `erosion.rs:2886` — both synthesize it from one height |
| `lithosphere::Column` | `lithosphere.rs:386` | `crust_m`, `crust_rho`, `keel_m`, `sediment_m` | `lithosphere.rs:436`, `erosion_return.rs:220/223` — pure functions of $(\text{seed}, \text{cell}, T_p)$ |
| `vivarium_core::voxel::{Volume, ErodedSurface}` | `voxel.rs:289`, `voxel.rs:163` | sparse voxel edit overlay; `h_m`/`depth_m`/`water_surf_m`/`vx_m`/`vy_m` grids | its own crate's worldgen, reachable from nothing |

The first two share no field, no datum, and no reference to each other — confirming the parent's observation and giving it a number. **Measured at three mid-face addresses (level 9):**

| address | `column::Column` total (m) | `crust_m` (m) | `keel_m` (m) | ratio |
|---|---|---|---|---|
| ZPos $(0.10, 0.10)$ | 2350.1 | 30 000.0 | 0.0 | 0.078 |
| XPos $(-0.30, 0.40)$ | 3900.2 | 32 771.1 | 116 408.3 | 0.026 |
| YNeg $(0.55, -0.20)$ | 1932.2 | 30 000.0 | 0.0 | 0.064 |

The stratigraphic column's "bedrock datum" sits inside the top few percent of the lithospheric column's crust. They are not two views of one object; they are two objects, and the stratigraphic one has no keel, no crust density, and no sediment — the three quantities the isostasy read integrates and the rock-mass ledger conserves.

**What the strata column does agree with, exactly.** Its `solid_thickness_m()` equals `sea_level::tectonic_surface_m` to $0$ m at every address tested (`delta = 0.000\mathrm{e}0`), which is unsurprising once traced: `gen::baseline_column` (`gen.rs:95`) feeds it `initial_topography_m`, and that is a one-line alias of `tectonic_surface_m` (`gen.rs:91-93`). So the column contributes no information about the surface; it re-packages a number computed elsewhere and adds two invented strata around it.

**What the strata column carries, measured** (live column, ZPos): two strata — `Undifferentiated(Igneous)` of 2348.135 m and `Kind(Soil)` of exactly 2.000 m, regolith $r = 2.0$ m. The soil thickness is the literal `2.0` at `gen.rs:100` and `erosion.rs:2886`.

**`Stratum::saturation` is written and never read.** Set at `gen.rs:48` (`1.0` below the waterline, `0.3` above), and a tree-wide grep finds no consumer — the only other occurrences of the identifier are unrelated comments in `water.rs` and `lithosphere.rs`. Pore water has a second, real representation: `WaterSim::groundwater`, a per-cell array with its own capacity, infiltration, baseflow and colmation coupling. The two never meet.

**`Column`'s derived-query API is exercised only by tests.** `overburden`, `material_at` and `regolith_thickness_m` have no call site outside `column.rs`'s own test module (plus one assertion in `gen.rs:118`). `Quantity`/`Exactness` — the exactness-propagating value type `overburden` was built to demonstrate — appears in four files (`gen.rs`, `material.rs`, `column.rs`, `planet.rs`) and reaches no kernel and no store payload.

**`sample.rs` is dead outside its own tests.** `sample_surface` / `sample_surface_with` — the `Patch`-producing bridge that would make the column a real sampling boundary — is called only from `sample.rs:28` and its test module. `erosion::column_at` (`erosion.rs:2885`), the fidelity-ladder swap point its doc comment advertises, has **zero** call sites anywhere. The only live readers of `gen::baseline_column` are `examples/scan_land.rs:23` and `examples/topo.rs:36`, and both subtract `gen::SEA_LEVEL_M` — the datum `gen.rs:19-24` marks `@deprecated` and *"do not use for land/water classification."*

## 3. The eight surface heights

For one cell at level $L$ in the live default world:

| # | derivation | file:line | relation to the others |
|---|---|---|---|
| 1 | `gen::bathymetry_m` | `gen.rs:63` | seafloor prior; level-dependent (Nyquist band-limit) |
| 2 | pre-ledger tectonic surface | `sea_level.rs:76` | $1$ plus the raw isostatic freeboard |
| 3 | live (post-ledger) tectonic surface | `sea_level.rs:67`, `erosion_return.rs:244` | $1$ plus the rebounded freeboard; aliased as `gen::initial_topography_m` |
| 4 | pre-ledger surface **at fixed level 8** | `erosion_return.rs:101/181/217` | what the ledger classifies with, whatever level the cell is |
| 5 | stored carved bed | `erosion-tile` root | the kernel's own field at the level it ran |
| 6 | `ErodedRegion::surface_m` | `erosion.rs` | bilinear over 5 **plus** a fine-minus-coarse prior increment |
| 7 | `DrainageSurface::filled_h` | `erosion.rs` | 5 raised by Priority-Flood spill levels **and** the flat-orienting $\varepsilon$ |
| 8 | `WaterSim::bed` | `water.rs` | a mutable copy of 5 that the settle then edits |

A ninth is arriving as this is written: `ErodedRegion::carved_surface_m` (uncommitted in the shared tree, by a concurrent session) — nearest-cell read of 5 with neither interpolation nor detail increment. Its own doc names the reason cleanly, and it is the sharpest statement of the problem in the tree: `surface_m` returns *"a surface no rung computed."*

### Measured: 3 and 4 disagree, and the disagreement flips land into sea

`erosion_return::crust_eroded_m` and `column_after_erosion_inner` classify a cell subaerial from `tectonic_surface_pre_ledger_at_tp(seed, cell, SAMPLE_LEVEL, tp)` with `SAMPLE_LEVEL = 8` hardcoded (`erosion_return.rs:63`, used at `:101`, `:181`, `:217`), no matter what level the cell is. A reader at level $L$ classifies the same cell with level-$L$ bathymetry. Sampled $6 \times 96^2 = 55\,296$ cells per level:

| reader level | cells where the two disagree on subaerial-vs-submarine | mean $\lvert$surface gap$\rvert$ | worst gap |
|---|---|---|---|
| 9 | 0 (0.000%) | 0.0 m | $+0.0$ m |
| 13 | 566 (1.024%) | 105.6 m | $-1247.8$ m |
| 19 | 588 (1.063%) | 109.4 m | $+1283.4$ m |

Level 9 agrees to the bit because bathymetry's octave count saturates at that grain; the disagreement appears as soon as a reader is finer than the pour. About 1% of the sphere is a cell the ledger erodes-or-deposits on the opposite verdict from the one a level-13-or-finer reader reaches, with surface gaps to 1.28 km.

This is one quantity with one true source re-derived locally — the pattern the brief asked about — and it is not the same instance as the ocean-mask one.

## 4. The nine water depths, and the five sea levels

| # | derivation | file:line |
|---|---|---|
| 1 | `Column::water_depth = \max(\text{sea} - \text{surface}, 0)` | `gen.rs:51` |
| 2 | `WaterSim` initial fill, $\max(\text{sea} - \text{bed}, 0)$ | `water.rs:334` |
| 3 | `WaterSim::depth` after the settle — the stored `water-tile` | `query.rs:999` |
| 4 | `DrainageSurface::standing_water` (spill level, $\varepsilon$ excluded) | `erosion.rs` |
| 5 | `DrainageSurface::fill_depth` (spill level, $\varepsilon$ **included**) | `erosion.rs` |
| 6 | `hold_edge_sea`, rim held at $\text{sea} - \text{bed}$ | `water.rs:920` |
| 7 | explorer `CellFacts::water_m` / `depression_m` | `paint.rs:169-198` |
| 8 | terminal globe `ocean = elev < sea` | `globe.rs:169` |
| 9 | `vivarium_core` `depth_m` / `water_surface` / `water_depth_voxels` | `voxel.rs:222-234`, `voxel.rs:671` |

**Sea-level values in simultaneous use, measured on this world:**

| value | m | source |
|---|---|---|
| live derived sea (post-ledger) | 5214.653 | `sea_level.rs:30` |
| pre-ledger derived sea | 5210.768 | `sea_level.rs:118` |
| `derived_sea_level_m(0)` — seed 0, hardcoded | 5106.265 | `gen.rs:33` |
| retired decree `SEA_LEVEL_M` | 4000.000 | `gen.rs:24` |
| `vivarium_core::voxel::SEA_LEVEL` | 3000.000 | `voxel.rs:101` |

Spreads: 3.9 m, 108.4 m, 1214.7 m, 2214.7 m.

**A live seed-dropping bug, in currently-dead code.** `gen::column_from_surface` (`gen.rs:28-35`) computes its waterline as `derived_sea_level_m(0)` — seed 0 — with no seed parameter to lose it from. `erosion::column_at(seed, cell, regions)` (`erosion.rs:2886`) passes it a seed-dependent surface and inherits a seed-0 waterline. Measured on this world: the resulting `water_depth` is **108.388 m** shallower than the world's own datum would give (2756.130 m vs 2864.518 m). It harms nothing today only because `column_at` has no callers. If a strata primitive is built on this constructor, that is the first thing to remove.

### Measured: 1 and 3 disagree by 201.5 m on average; 2 and 3 are bit-identical

Over 6144 cells sampled across 24 built `water-tile` roots (level 9, `eepochs=40`, `steps=200`) at the era cohort:

- mean $\lvert$(datum difference) $-$ (stored kernel depth)$\rvert$ = **201.5 m**
- worst single cell: face 0 $(60, 364)$ — datum says 3226.7 m of water, the kernel says 0.0 m
- 5311 cells both call wet ($\gt 0.5$ m); **243 cells the datum calls wet and the kernel calls dry**; 32 the reverse
- mean $\lvert$tectonic surface $-$ stored eroded bed$\rvert$ = **217.1 m** (over 5767 finite pairs)

So most of the 201.5 m is the *bed* disagreeing, not the water. Separating them gives the more interesting number:

- mean $\lvert(\text{sea} - \text{eroded bed}) - (\text{stored kernel depth})\rvert$ = **0.0 m — exactly, at every sampled cell.**

The stored `water-tile` depth is bit-for-bit the initial fill of `WaterSim::new_at_sea` (`water.rs:334`). Two hundred settle steps changed nothing measurable at any of 6144 sampled cells. That is consistent with `#obs-water-fill-never-settles`, measured here from the other side: the only durable per-cell hydrology in the store is a datum subtraction wearing a kernel's name. Whether the settle is inert *everywhere* or only on the submarine interiors this sample is dominated by is not established here — the sample is 24 of 384 tiles and was not stratified by land fraction.

### The ocean-mask site count: five, confirmed

`#form-ocean-is-connectivity-not-elevation` FE(1) distinguishes the *submerged* set (a threshold, correctly) from the *ocean* set (connectivity). Only one place computes the ocean set: `erosion.rs:774-807`, privately inside `Fluvial::outlets`. The places that answer the same question with a threshold, all verified by reading the site:

1. `globe.rs:169` — `let ocean = elev < sea` (terminal reader)
2. `paint.rs:302` — surface paint, `if f.h_m <= f.sea_m`
3. `paint.rs:310/345/358` — water and shoreline paints, `f.h_m > f.sea_m`
4. `water.rs:334` + `water.rs:920` — the kernel's own initial fill and rim hold
5. `gen.rs:48/51` — the strata column's `saturation` and `water_depth`

That is **five**, matching the sibling agent's count and one more than the segment's Working Note states. The fifth is the column constructor, which is the reason it matters for this spike: the retired elevation threshold is *inside* the object a strata primitive would be built from.

Separately, the *submerged* classification — legitimately a threshold — is re-derived at **eight** sites in six near-identical whole-sphere sample loops: `sea_level.rs:217`, `:244`, `:356`, `:394`, `:428` and `erosion_return.rs:182`, `:218`, `:302`. Five of the six loops differ only in which surface function they call and which of the two sea values they compare against.

## 5. The volatile layer: what a kernel holds and then throws away

`WaterSim` (`water.rs`) carries **eight per-cell state arrays** — `bed`, `depth`, `sediment`, `groundwater`, `sed_bed`, `bed_res`, `colmation`, `armor` — plus four pipe-flux arrays kept between steps. Exactly one, `depth`, reaches the store (`query.rs:999`). `colmation`'s own doc says it is *"PERSISTENT — a sealed bed stays sealed between storms"*; it is discarded when the tile is memoized. `Fluvial` carries `h`, `drainage`, `cell_area`, `centers`, `uplift_rate`, `precip_weight`; one, `h`, reaches the store. `DrainageSurface` carries `mfd`, `d8`, `recv`, `filled_h`, `fill_depth`, `standing_water`; none reaches the store, and it is recomputed per call.

**This is the inverted-from-expected result and the one that most changes the spike's question.** The strata are the read-time fiction; the *sub-surface* state — pore water, sealing, armor, alluvium, suspended load — is real per-cell state that already exists at kernel time and has nowhere durable to go, because the store's vocabulary is one scalar per field per cell. A stratified column primitive would not be adding structure the kernels lack. It would be giving eight arrays that already exist a shared home and a key.

## 6. Incidental finding, flagged: the store holds NaN heights

Not sought, found while separating bed disagreement from water disagreement. **83 of 3552 `erosion-tile` field roots at the current source cohort contain NaN heights** (55 391 NaN cells of 14 548 992 stored, 0.38%). All 3552 at this cohort are `edge=halo`, so the contract is not discriminated by this measurement. Eleven distinct tile origins are affected, each across up to 8 epoch rungs; the worst is face 0 $(128, 448)$ at 87.9% NaN.

NaN fraction against epochs at that origin: 7.5, 15.7, 28.7, 46.8, **10.7**, 11.0, 11.5, 87.9 (epochs 5 … 40). It grows, then partly resets, then blows up — so it is not simple monotone propagation from one bad cell, and the pattern is not explained here.

Two clamps make this silent rather than loud:

- `WaterSim::new_at_sea`: `(sea - b).max(0.0)`, and Rust's `f64::max` returns the non-NaN operand — so a NaN bed becomes **0 m of water**, i.e. dry land, with no warning.
- `globe.rs`: `tile.get(...).copied().unwrap_or(sea as f32)` guards an out-of-range index, not a NaN value; NaN reaches `elev < sea`, compares false, and paints as **land**.

This is squarely `#norm-probes-before-claims` territory rather than this spike's, and it wants its own probe. Reported here because it is a fact about what the store holds for a horizontal address, and because a strata primitive built on these beds would inherit it.

## 7. Two more disagreements worth naming

**The coverage census cannot see the built water.** `Coverage::parse` reports one `level` — the deepest surface tile — and `watched` is keyed at that level. On the live store the deepest surface tiles are level 13 while every water tile is level 9, so `Coverage` reports `level=13, nx=64, watered_tiles=0` while 384 current-cohort water roots sit in the store. Verified: the direct root scan finds all 384. This is the mechanism under `crates/vivarium-explore/src/water.rs:1-17`'s statement that the water field has never been rendered — the census that feeds the view reports it as absent.

**A doc that outlived its code.** `DrainageSurface::standing_water`'s doc still says *"`Fluvial::outlets` classifies sea by elevation threshold rather than connectivity, so such a basin is already an outlet and holds nothing here."* `outlets` became connectivity-aware at `erosion.rs:751-807`. The paragraph now describes a defect that was repaired.

## 8. On the framing question

Measurement, not verdict — the adjudication is the spike's.

What the census supports: there is no per-address object in this tree today. There are five separately-keyed scalar fields, three unrelated column types of which none is written by a kernel, eight surface derivations, nine water derivations, five sea levels, and eight submerged-classification sites. The two measured disagreements that a shared vertical object would remove by construction are the level-8/level-$L$ ledger split (1% of the sphere, gaps to 1.28 km) and the five-way ocean-mask fork.

What it does **not** support: that `column::Column` is the object to build on. Its datum is unrelated to the lithospheric column's, its two strata are invented at read time, its `saturation` is written and never read, its derived queries are test-only, its `water_depth` is the retired elevation threshold, and its constructor drops the seed. Every one of those is a property of `column.rs` + `gen.rs`, not of stratification as an idea.

Against Joseph's two-way test — *"either this is a temporary hack, or they know what they're doing, this is for algorithmic efficiency and it reduces back to columns on pull/request"* — the measured answer for `column::Column` is neither branch. It does not reduce back to columns on pull, because there is nothing to reduce: no kernel state is projected into it. It synthesizes. That is a third thing, and it is the thing to say plainly.

---

## Method, and what is not established

**Verified by running code at the stated era:** every number in §§2–4, §6, §7's first item. Probe `examples/vertical_state_census.rs`; store scanned read-only via `Store::open_read_only`; NaN census by direct read of `objects/` payloads as little-endian f32 (matching `query.rs:1087`).

**Verified by reading the site:** every file:line citation, every site count, the write-only status of `saturation`, the test-only status of `overburden`/`material_at`/`regolith_thickness_m`, the zero-call-site status of `erosion::column_at`, the absence of `vivarium-core` from any dependency of `vivarium-world` or `vivarium-explore` (only its own `examples/` reference it).

**Inferred, not measured:** that the settle's inertness in §4 generalizes beyond the 24-tile sample; that the NaN blow-up is specific to halo exchange (the cohort offers no edge-sink control); that the 1% level-split disagreement translates into a visible artifact rather than being absorbed downstream.

**Not attempted:** the agent-seam and voxel tiers (Level C gate, out of scope here); whether the level-8 hardcode in the ledger is deliberate pour-grain law or an oversight — `erosion_return.rs`'s module doc calls the ledger a pour-grain article, which reads as deliberate, but nothing states what a finer reader should then do; the `#form-fidelity-ladder` FE(7)–(9) detail-increment question that `carved_surface_m` is arriving to address.

## Feedback on the brief

Useful and mostly right. Three notes:

The brief named `tabularium/*.ordinum.udon` as *"the closest thing to a declared field inventory."* It is not — it declares phases, charges and promises, and the only per-address noun in it is `record: "base columns"` under Phase 1. The actual declared inventory is `flux.rs` (11 quantity strings, closed vocabulary, test-enforced) plus `nomotheke.rs`'s `promises`/`consumes`. Reading the ordinum first cost time; reading `flux.rs` first would have oriented the whole census in one file. Worth knowing: `LITHO_COLUMN` is the only column-shaped entry in that vocabulary, and it names the *lithospheric* column, not the stratigraphic one.

The hypothesis was framed as "nominal, present as a type" — which invited me to check whether it is storage. The more productive question turned out to be the inverse one, and it is the one I would put in a brief like this next time: *which per-cell state does a kernel already hold that the store has no vocabulary for?* That is what surfaced `WaterSim`'s eight arrays, and it is the finding that bears on whether a column primitive adds structure or merely re-homes it.

The "don't edit `src/`" constraint was correct and the reason given was accurate, but it is worth knowing that `examples/` is outside the digest (`build.rs` hashes `src` only), so a probe can be written and run freely — I would not have found the NaN heights without one. Separately: the shared tree's `src/erosion.rs` was modified by a concurrent session while I measured, which moved `SRC_HASH` and de-synchronized the store cohort mid-run. A worktree at HEAD fixed it. If more agents are running against this tree tonight, a brief that says which commit to measure at would save the rediscovery.
