# Spike — is the stratified column the primitive?

*Spike, not canon. Nothing here is adopted by being written down; claims land in `core/` through their own step and #norm-decision-authority governs who ratifies what. Base `git` short hash `9dd86ab`. Sources read 2026-07-29 unless marked otherwise.*

Joseph's question, verbatim:

> *"I'm genuinely surprised we're talking about the ocean as its own thing at all instead of already being in the realm of [unknown/undefined -> mantel layers -> water layer(s) -> topological layer -> air column / atmosphere layer] strata as our main primitive all 'round..."*

## What this spike found, in seven lines

1. **The column-as-primitive question is already decided in canon** and the code went three other ways; the claim is not the gap. ( #form-column-control-volume FE(1)/(5).)
2. **The ordering question has an authoritative answer too** — a closed water cycle and basic weather gate at phase **3**, before erosion at phase **4** — and *what let the build invert it* is measurable: of phase 3's four gate charges, one has an instrument that can fail; both of phase 4's built gates have one. The `closed-water-cycle` predicate tests an algebraic identity, so on the path its test exercises no world can fail it.
3. **Under Joseph's reduce-back-on-pull test the strata score neither branch — they *synthesize*.** The pull fabricates content (a hardcoded $2\ \mathrm{m}$ of soil) from a scalar height, so there is nothing projected in to reduce back. Measured: **8 surface-height derivations, 9 water-depth derivations, 5 simultaneous sea levels**, against **5 durable f32 per cell**.
4. **Nine separately-named open rungs are one missing object**, and seven declared phase `|record`s are unkeepable without it.
5. **The case is stronger inverted, and this is the finding I had backwards**: the kernels already hold the state (`WaterSim` 8 per-cell arrays, 1 reaching the store; `Fluvial` 6 and 1; `DrainageSurface` 6 and 0). A column would **re-home existing state**, not add structure — the opposite of the FE(11) read-time defect rather than a new instance of it.
6. **The cheapest consequential fix touches no code**: give the phase-3 gates predicates that can fail. It moves no `SRC_HASH` and invalidates no store.
7. **Two findings outrank the spike** and are flagged rather than adjudicated: the committed store holds **NaN heights** in 83 of 3552 erosion roots, silently painting as dry land; and `Coverage` reports `watered_tiles=0` while 384 water roots exist, because it keys at the wrong level.

## The answer, up front

**The surprise is well-founded, and the reason is sharper than a missing feature: the stratified column is already adopted canon, and three parallel vertical representations grew underneath it anyway.** #form-column-control-volume FE(1) already states the column as ordered strata bedrock-toward-surface *with standing water above the solid top*, and FE(5) already states that $(b, d, r)$ "falls out of strata + water depth rather than as a parallel world model." That is Joseph's stack, missing only its two open ends. The claim is not the gap.

So the useful form of the question is not *should the column be the primitive* — that is decided, `status: exact` on the frame, ratified as project stance in `DECISIONS[column-is-a-control-volume-with-sufficient-statistics]`. It is **why the code went three other ways, and what the divergence is now costing.** The answer to that is the body of this spike, and the short version is:

## The ordering question already has an authoritative answer, and the gates that could convict are the ones that got built

Joseph asked, separately: *"Didn't our original phases, and therefore nomos, have atmospheric weather patterns as a pre-abyssal condition / gate?"* **Yes — verified in `tabularium/terrestris.ordinum.udon` at `9dd86ab`, which #detail-phase-abyssal names as the authority and forbids forking.** Primordial is phase **3**; Abyssal is phase **4**. Phase 3's `:tag gate` charges include `atmosphere-reservoir`, `closed-water-cycle` ("Water cycle, closed."), `weather-energy-channels` ("Basic weather fluid dynamics; useful-energy / entropy-gradient channels") and `seeded-asymmetry`. Phase 4 is where `erosion-carving` and `emergent-land` live.

So a closed water cycle and basic weather dynamics gate **before** the erosion phase, by declaration — while erosion, hydrology and lakes are built and `climate.rs` declares itself to have no circulation and no EBM, in its own words *"the PATTERN is fated noise, NOT meteorology."*

**The interesting question is not that the build inverted the order — it is what let it.** I audited every promise in the ordinum for the two things that could report a gate unmet, a `:kept-by` keeper and a `|predicate`. Measured, all phases: **32 promises, 6 with a keeper, 10 with a predicate, 21 with neither.** Restricted to the phases in question:

| phase 3 gate charge | its promise | keeper | predicate |
|---|---|---|---|
| `atmosphere-reservoir` | `three-reservoirs` | — | — |
| `atmosphere-reservoir` | `pre-oxygen-composition` | — | — |
| `closed-water-cycle` | `closed-water-cycle` | — | present, and see below |
| `weather-energy-channels` | `weather-channels` | — | **none** |
| `seeded-asymmetry` | `seeded-asymmetry` | `noise` | present, falsifiable |

| phase 4 gate charge | its promise | keeper | predicate |
|---|---|---|---|
| `emergent-land` | `emerged-land` | `isostasy` | present, rich, with known-bads |
| `erosion-carving` | `erosion-substrate` | `erosion-tile` | present |
| `mineral-systems` | `ore-and-soil` | — | — |

**Of phase 3's four gate charges, one has an instrument that can fail. Both of phase 4's built gates have one.** The build did not defy the declared order so much as follow the instrumented one — which is #norm-declaration-must-convict as a measurement rather than a maxim, and FORMAT's second open question ("a declaration which cannot fail a build is a wish") with a number attached.

**And the water-cycle predicate is worth stating precisely, because it is weaker than "weaker than its promise."** The charge prose says *"Water cycle, closed."* The promise prose says *"A closed, conserved water cycle with its energy-gradient channels."* The predicate says *"global water mass is conserved — nothing pulled from nothing, nothing drained into nothing."* And in `hydrosphere.rs` that conservation is an **algebraic identity**. `Hydrosphere::of` sets `atmosphere_km3 = total × ATMOSPHERE_FRACTION` and then `ocean_km3 = total − atmosphere_km3`, with the source comment saying *"conservation by construction"*; `conservation_residual_km3` returns `total − (ocean + atmosphere)`, documented as *"Exactly 0 by construction."*

Being precise about the scope, since "cannot fail" is the kind of claim that should not be loose: on the construction path the test exercises, the residual is zero by algebra and **no world can make it fail**. The one input that could is a corrupted `from_bytes` deserialization — so the predicate is a store-integrity check wearing a physics predicate's name. A partition with **zero flux between reservoirs** satisfies it perfectly. A conserved inventory is not a closed cycle, and the gate reads satisfied while no water moves.

I want to be careful about the register here, because there is a more dramatic version of this finding that is false. **The cycle is closed by assumption, not unclosed.** `climate.rs` sets precipitation equal to evaporation at steady state, drawn from a conserved stock, and says so honestly. What is absent is the *dynamics*, not the *budget*. The defect is in the gate's instrument, not in anyone's diligence — and generalized: **any charge whose predicate under-tests its prose will let a later phase build on it, silently, and `crate::audit` cannot see the difference** because an unkept promise with no predicate is indistinguishable from a gloss.

**Why this belongs in a spike about columns.** It supplies the ordering answer the column question was reaching for, and from an authority neither Joseph nor I would have had to invent: a closed water cycle requires water to be a **stock that moves between reservoirs** — pore, standing, atmospheric — and those reservoirs are `Stratum::saturation` and the column's standing water. On that reading the strata work is not a refactor competing for rank; **it is what closing the phase-3 gate requires.** That is the version of the unification I set out to break and could not, and §"Loops" below records where I *did* break a neighbouring one — closure does not need *layering*, only per-cell stocks — so this claim is deliberately the narrower one: the column is what the gate needs, and layering is a later rung.

**The cheapest consequential work in this spike lives here, and it touches no code.** Predicates and keepers are ordinum data. Writing a falsifiable predicate for `weather-channels` and strengthening the `closed-water-cycle` one to test a *flux* rather than an identity would convert an implicit deferral into a standing audit finding — and, unlike everything else recommended here, it moves no `SRC_HASH` and invalidates no store.

## Joseph's own test is the right instrument, and nothing in the tree passes it

Joseph supplied the criterion mid-spike, and it is a better instrument than the one I started with:

> *"Honestly every time I hear about work being done that doesn't assume the column as the primitive makes me think 'either this is a temporary hack' or 'they know what they're doing-- this is for algorithmic efficiency, and it reduces back to columns on pull/request...'"*

Two categories: **(a)** a workaround, or **(b)** a deliberate representation change for efficiency that **reduces back to columns on pull**. I first scored the solid strata (b), then (a) after reading the constructor. The census file settles it as **neither**, and its phrasing is better than mine:

> It does not reduce back to columns on pull, because there is nothing to reduce: no kernel state is projected into it. It synthesizes. That is a third thing, and it is the thing to say plainly.

That is the correct call and I am adopting it. A hack *stands in* for something real; this invents. So Joseph's two-way test needs a third branch for what is actually there, and naming it is more useful than forcing it into (a) — because the repair for a hack is to replace it, whereas the repair for a synthesis is to give the thing it pretends to summarize somewhere to live.

**Why the solid case fails, and it is the load-bearing argument in this spike.** There *is* a pull path: `erosion::column_at(seed, cell, regions)` returns a `column::Column`, which looks exactly like (b). Following it through is what settles the question:

```
column_at → gen::column_from_surface(cell, surface_at(…), 2.0)
          → gen::column_from_surface_at_sea(cell, surface_m, soil_m, sea_m)
```

and that constructor, read at `9dd86ab`, assigns: one `Undifferentiated(Igneous)` bedrock stratum of whatever thickness is left over; a **hardcoded $2.0\ \mathrm{m}$** of `Soil`; a saturation of $1.0$ or $0.3$ by whether the surface is under the datum; and

$$d_{\text{water}} = \max(s_{\text{sea}} - h_{\text{surface}},\ 0).$$

So the pull does not *recover* a column. It **manufactures** one from a single float. Erosion can strip three hundred metres of regolith and the pull still reports $2\ \mathrm{m}$ of soil over undifferentiated igneous, because the eroded surface never carried what was removed.

That generalizes into the criterion's sharp form, which I think is the real content of Joseph's test:

> **A representation reduces back to columns on pull only when the projection that produced it was lossless in the dimensions the column exists to carry.** A heightfield is a projection of a stratigraphic column along exactly the axis stratigraphy *is*. No pull function can invert it, so no improvement to `column_at` can move the solid case from (a) to (b) — the information is destroyed at the projection, not at the pull.

This is why the question is not stylistic. Under Joseph's own test, "reduces back on pull" is a **property of the projection**, and the only way to earn category (b) here is for the kernel to carry strata. That is a claim about what is achievable, not a preference.

**And this does not need arguing fresh, because a law landed tonight already convicts it.** The sibling survey `phenomena-reach.md` makes the sharper version of the point and corrects the framing I began with: **the strata are the read-time field, not the water.** `column::Column` has exactly one producer, synthesizing at most two strata from a scalar height at query time, and no kernel writes a `Stratum` at all — while water *is* real per-cell state, in `WaterSim`'s seven physical arrays. So #form-fidelity-ladder FE(11) — a field that acquires a detail increment at read time is invisible to the process it should constrain — **convicts the present vertical arrangement for the whole planet at every level**, not merely the unbuilt submarine-refinement case it names as a caution. The clause was written tonight looking forward at two unbuilt interfaces; it turns out to bite hardest on what is already running.

That also means the two findings are complementary rather than competing: water is bypassed (a wiring fact, repairable by rewiring), and the strata are synthesized at read time (a law violation under FE(11), repairable only by the kernel carrying them).

**Scored, at `9dd86ab`:**

| layer | verdict | why |
|---|---|---|
| solid strata | **synthesized** | the pull fabricates content — constant $2\ \mathrm{m}$ soil, one undifferentiated bedrock category, no erosion history. Measured: two strata, `Undifferentiated(Igneous)` of $2348.135\ \mathrm{m}$ and `Soil` of exactly $2.000\ \mathrm{m}$ |
| standing water | **(a)** | the field exists and the hydrology writes its answer elsewhere; and being a **scalar**, it could not carry layering even if used |
| pore water | **(a)** | `Stratum::saturation` is written at `gen.rs:48` and read by nothing tree-wide; the real pore water is `WaterSim::groundwater`, and the two never meet |
| air | absent | `MaterialId::Void`, carrying air's density under a name meaning *absence* |

**And the census gives the divergence its scale.** For one horizontal cell in the live default world the crate can produce **eight different values for the solid surface height** and **nine for how much water stands here**, against **five simultaneous values of sea level** (spreads of $3.9$, $108.4$, $1214.7$ and $2214.7\ \mathrm{m}$). Durable, per cell, is **five f32** — one each from `initial-topography`, `uplift-tile`, `climate`, `erosion-tile`, `water-tile`. Verified by direct scan of 116 955 store root files: **no root kind's payload is a column, and none carries more than one number per cell.**

**A concrete bug found inside the constructor, currently harmless.** `gen::column_from_surface` (`gen.rs:28-35`) computes its waterline as `derived_sea_level_m(0)` — **seed 0, hardcoded, with no seed parameter to lose it from** — while `erosion::column_at` hands it a seed-dependent surface. Measured on this world the resulting water depth is **$108.388\ \mathrm{m}$ shallower** than the world's own datum gives. It harms nothing today only because `column_at` has zero call sites. If a strata primitive is built from this constructor, that goes first.

**And the threshold count is five, not four — confirmed independently.** The retired elevation law that #form-ocean-is-connectivity-not-elevation replaced is *also* how `gen::column_from_surface_at_sea` sets `water_depth`, so it survives inside the column constructor, which is the one place a reader would most reasonably trust. I found it by following one call path; the census reached five by reading every site (`globe.rs:169`, `paint.rs:302`, `paint.rs:310/345/358`, `water.rs:334`+`:920`, `gen.rs:48/51`) against one connectivity-aware answer at `erosion.rs:774-807`. That is one more than the segment's Working Note records.

**The *submerged* classification — legitimately a threshold — is re-derived at eight further sites** in six near-identical whole-sphere sample loops (`sea_level.rs:217/244/356/394/428`, `erosion_return.rs:182/218/302`), five of which differ only in which surface function they call and which of the two sea values they compare against.

**And the pattern has a second measured instance that is not the ocean mask at all.** `erosion_return` classifies a cell subaerial-versus-submarine from a **hardcoded level 8** (`erosion_return.rs:63`, used at `:101`, `:181`, `:217`) regardless of the cell's actual level, while a reader at level $L$ classifies the same cell with level-$L$ bathymetry. Measured over $55\,296$ cells per level: at level 9 the two agree **to the bit** (bathymetry's octaves saturate there); at level 13 they disagree on **1.02%** of the sphere with a mean surface gap of $105.6\ \mathrm{m}$ and a worst of $1247.8\ \mathrm{m}$; at level 19, **1.06%** and $1283.4\ \mathrm{m}$. So about one percent of the world is a cell the rock-mass ledger erodes or deposits on the opposite verdict from the one a finer reader reaches. Whether the level-8 pin is deliberate pour-grain law or an oversight is *not* settled — `erosion_return.rs`'s module doc calls the ledger a pour-grain article, which reads as deliberate, but nothing states what a finer reader should then do.

**The scalar-versus-strata distinction for fluids is a separate question from primitivity, and it may be the actual crux of Joseph's bracket** — note his plural, *"water layer(s)"*. A scalar `water_depth` cannot express a stratified ocean, a brine layer, ice over water, or a thermocline, no matter who reads it. So there are two independent findings here that a single "make the column primitive" framing would blur: *water is bypassed* (a wiring fact) and *water could not carry layers anyway* (a type fact). The second is the one his plural is pointing at.

**Nine separately-named open rungs in the tree are one missing object.** Each was found from inside a different nomos, and each reads locally as its own piece of debt. Collected, they are the same absence:

| named open rung | where it is named |
|---|---|
| carved tiles do not debit their columns | #form-isostasy-column FE(2), FE(9) |
| per-material erodibility (differential erosion) | `erosion.rs` header, and again at its line 178 — "the flagged next hook" |
| four independent thresholds answer "is this cell ocean" | #form-ocean-is-connectivity-not-elevation Working Notes; ranked as hotspot **1b** |
| deposition is a uniform submarine blanket, not routed | #form-isostasy-column FE(9) open rungs |
| grain-size split, dissolved load, flocculation absent; armor/colmation is scaffolding awaiting "when grain sizes emerge" | #detail-phenomena-systems-map FE(5); #form-fidelity-ladder FE(6) |
| multi-layer gas mixing up the column, and submarine strata refinement — "neither has been built yet" | #form-fidelity-ladder FE(11) |
| the water cycle is closed by *assumption* (a conserved budget, `precip = evap` asserted at steady state) with no dynamics — so no reservoir a lake, a soil or the atmosphere can exchange with | audited tonight; #obs-water-fill-never-settles for the temporal half |
| a phase-3 gate charge (`weather-energy-channels`) whose promise carries no predicate and no keeper | `tabularium/terrestris.ordinum.udon`, measured above |
| the column semantics fork (point / mean / band-limited), `Statistic::Undeclared` on the registry | #disc-open-problem-census §46; open on #form-column-control-volume |

Each of those is cheap to describe and expensive to fix *in isolation*, which is the signature the tree already knows to read: #form-declared-boundary-contract FE(1) calls it "the audit is green because the lie is in a column the schema does not have," and #form-ocean-is-connectivity-not-elevation reasons from four independent thresholds to a missing world object. This spike applies the same inference one level up, to the vertical axis rather than to one quantity on it.

**And it is already Joseph's own intent, from three weeks earlier.** *"Reconcile column state with the strata Column"* is item two of the twelve explorer intents (2026-07-02, re-affirmed as binding 2026-07-29), carried in #disc-open-problem-census §III. Tonight's surprise is that intent resurfacing at architecture altitude rather than as a viewer task — which is the right altitude for it, and is the substantive move.

## What actually runs: three vertical representations, and none of them is the claimed one

Verified by reading `crates/vivarium-world/src/`. A sibling file, `census-vertical-state.md`, carries the independently-measured census; this table is the shape I read before commissioning it, and the two should be checked against each other rather than merged.

**1. `column::Column` — the claimed object, running as a fabricated pull product.** `strata: Vec<Stratum>` plus `water_depth: Quantity` plus per-stratum `saturation`. Constructed in `gen.rs` and `erosion.rs::column_at`; read in `sample.rs`. It conserves nothing, no kernel steps it, and every field it carries is invented at construction from a single height (above). The fields for standing water and pore water both exist and both go unused by the hydrology — which is the more interesting fact than their absence would have been, because it means the design was right and the wiring went around it.

**2. `lithosphere::Column` — a second struct with the same name in the same crate.** `crust_m`, `crust_rho`, `keel_m`, `sediment_m`. This is where the *mass* truth lives: `Conservation::Conserved` with a closed-box probe ( #form-isostasy-column FE(9)), erosion debit, Airy rebound, sediment credit. It has no stratigraphic order, its sediment is one lumped thickness, and it runs at the pour grain. Neither `Column` references the other.

**3. `Fluvial::h: Vec<f32>` — a heightfield, and where all the geomorphology happens.** One float per cell. `FluvialParams::k_dt` is a single scalar erodibility for the whole run. The kernel cannot know what it is cutting through, and has not been able to since the port: the module header lists "per-material erodibility (differential erosion)" among *next* increments, and line 178 repeats it as "the flagged next hook — uniform hardness in this first increment."

Plus `water_depth` again in `vivarium-core/src/voxel.rs`, in another crate.

**Two observations I would not have predicted, both from `material.rs`:**

- **`Kind::Water` already exists as a full material** — `Phase::Liquid`, density $1000$, porosity $1.0$, permeability $10^{-9}$. So `Stratum { material: Water, thickness, saturation }` is *already well-typed*. The `water_depth` scalar is a redundant special case of a capability the type carries. Nothing had to be built for water to be a stratum; a parallel scalar was written beside the ability instead.
- **The column's two open ends are the same symbol, and it carries air's density.** `MaterialId::Void` is documented as "Air / cave / excavated space", returns `Phase::Gas` at $1.2\ \mathrm{kg/m^3}$ — air — and is what `material_at` returns above the solid top. One value means *above the world*, *hole in the rock*, and *air*. Below the column there is no sentinel at all: `Column::solid_thickness_m` sums from "the bedrock datum" while `lithosphere::Column` measures crust and keel toward a compensation depth. **Two different bottoms, neither declared.**

That second one is why I think Joseph's bracketed stack is doing real work rather than illustrating. It names both open ends — `unknown/undefined` at the bottom, `air column / atmosphere layer` at the top — and both are exactly where the current representation is silently overloaded.

## The constraint that decides the shape, and it is not the one I expected

#form-fidelity-ladder FE(11), landed tonight, is the governing constraint and it reads at first like an objection:

> *any field that acquires a detail increment at read time is invisible to the process it should be constraining: multi-layer gas mixing up the column and submarine strata refinement inherit this clause as written, and neither has been built yet — which is the cheap moment to get the ordering right.*

Read carelessly, that forbids the proposal: adding water and air strata would multiply the fields that can acquire detail at read time, at two fresh interfaces. Read carefully, it *names this spike's subject as the cheap moment* and states the discipline the shape has to satisfy.

The resolution, and I think it is the load-bearing design point:

> **The number of strata is a rung. The column is the object.**

Run-length strata make the vertical ladder free in a way a fixed-$N$ layer scheme does not. A dry column is one entry. A one-layer mixed-layer ocean is one entry. A single-layer atmosphere with mass is one entry. Refinement adds entries **at build time, when a rung runs**, and the rung's layer count enters the key — which is what #form-complete-content-addressed-key already demands and what makes the causal-closure argument in #form-fidelity-ladder go through on the vertical axis as it does on the frequency axis. `Vec<Stratum>` was chosen for the solid; its ladder-freeness is general, and it is the property that makes the extension legal rather than a fresh instance of the defect.

The sharp near-term consequence, which cuts against the obvious enthusiastic move: **the work is not to add layers.** It is to make the layers that already exist singly *be strata*. One water stratum in place of a `water_depth` scalar buys nothing on its own — and that is the point, because it is a change with no dynamics attached and therefore no ladder violation available, while being the change that makes every later composition legal.

## The strongest form of the case is the one I had backwards

I argued above that a column-shaped world *reaches* things the current one cannot. That is true and it is the weaker half. The census measured the other half, and it inverts my premise:

| struct | per-cell state arrays it holds | how many reach the store |
|---|---|---|
| `WaterSim` | `bed`, `depth`, `sediment`, `groundwater`, `sed_bed`, `bed_res`, `colmation`, `armor` — **eight** | **one** (`depth`) |
| `Fluvial` | `h`, `drainage`, `cell_area`, `centers`, `uplift_rate`, `precip_weight` — **six** | **one** (`h`) |
| `DrainageSurface` | `mfd`, `d8`, `recv`, `filled_h`, `fill_depth`, `standing_water` — **six** | **none**; recomputed per call |

So the sub-surface state a stratified column would carry — pore water, sealing, armour, alluvium, suspended load — **already exists at kernel time.** It has nowhere durable to go because the store's vocabulary is one scalar per field per cell. `colmation`'s own doc says it is *"PERSISTENT — a sealed bed stays sealed between storms"*, and it is discarded when the tile is memoized.

**That changes what the proposal is.** A stratified column primitive is not adding structure the kernels lack; it is giving state that already exists a shared home and a key. Re-homing is a much cheaper claim to defend than inventing, and it means the FE(6) demolition-date discipline applies in the *favourable* direction: two of the fields it would absorb (`armor`, `colmation`) are already declared scaffolding awaiting exactly this.

It also supplies the missing piece of my ladder argument. I claimed run-length strata make the vertical ladder free; the sharper statement is that **the kernels are already running at a fidelity the store cannot express**, so the first strata do not introduce a band no rung ran — they record a band that ran and was thrown away. That is the opposite of the FE(11) defect rather than a new instance of it.

## What a column-shaped world gets that the current one cannot reach

Not a wish list — these are cases where I traced the blocking dependency and it terminated at the same object.

**Carving becomes debiting, with no bridge to build.** Today, removing $3\ \mathrm{m}$ of bed in `Fluvial` and debiting $3\ \mathrm{m}$ of crust in `erosion_return` are two operations at two grains over two representations, and the bridge between them is #form-isostasy-column's open rung. Over strata they are one operation: removing the top $3\ \mathrm{m}$ of a stratum stack *is* the mass debit, *is* the exposure of the next material, *and* produces a typed sediment mass that has to land somewhere. The fork stated at #form-erosion-at-a-cooling-stage FE(4)–(5) — whether the iterated erode/rebound chain replaces the ledger's mass-return role — is a fork about which of two representations owns the ledger. It reads differently if neither does, because the column does.

**Differential erodibility is not a feature, it is what strata are for.** #detail-phenomena-systems-map FE(4) derives landforms as compositions over the material property interface, and its first example is hoodoos = layered erodibility + erosion. The material interface has carried `erodibility` and `incision_threshold` per material since it was written. The kernel reads neither. So the composition law in canon has no executable path, and one change opens it.

**Chemical and mechanical deposition are one operation over different sources.** This is the sharpest test of Joseph's list, and his *"settling lime deposits"* is well chosen: carbonate precipitation is dissolved load in the water column plus a saturation state, precipitating and accreting as a stratum at the bed. Every term is a column property, and a scalar water depth can hold none of them — while `Kind::Water`'s bundle has porosity and permeability but no solute slot. Once deposition is strata-shaped, evaporites, travertine, banded iron and coal are the same rung with different sources rather than four features.

And this one is **not deferrable past the phase the world is in.** The sibling survey establishes it against the ordinum rather than by my guess: `mineral-systems` carries `:tag gate`, and the Abyssal phase's own `|record` names banded iron and first limestones. Its measured companion figure is the sharper statement of this whole spike's thesis that I have seen: **seven phases declare a `|record`, and none of them is keepable without a stored column.** A declared record no representation can satisfy is the visible-debt form of a missing object.

**Ocean-ness stops being a classification.** With a water stratum, *submerged* is a local column property ($d_{\text{water}} \gt 0$) rather than a threshold re-derived in five places. Connectivity does not go away — which of the standing bodies are contiguous is genuinely nonlocal — but it changes from *classifying cells* to *labelling bodies*, which is the shape #form-ocean-is-connectivity-not-elevation FE(2) is already written to permit and its Epistemic Status already names as the better end state ("a *designated* ocean region... would remove FE(4)'s window dependence entirely. That is a better end state and it is not what is built"). Tonight's connectivity repair is correct and is not wasted by this; it is the right computation, and it currently stands in for a state variable that was never stored.

## Weather, and a sharpening of Joseph's hedge

Joseph wrote that warming the weather system up *"will involve some degree of water column up front (not to mention air column)."* I think the instinct is right and the *reason* is worth stating precisely, because it changes what to build. **Reasoned from the physics and from reading `climate.rs`; not measured.**

The next climate rung is not per-cell three-dimensional air columns. `climate.rs` is presently one identity — mean precipitation as atmosphere stock over residence time, times a mean-preserving fated jitter — and its own header names the honest next step as latitudinal structure from insolation, which is a surface field. What a *rationally circulating* atmosphere needs beyond that is:

1. insolation by latitude and obliquity — and the sibling survey measured that `INSOLATION` is already **produced, keyed, and consumed by nothing**, so this branch's driver is paid for and idle;
2. an energy balance giving a temperature field — surface;
3. a **lapse rate** for orographic condensation — a profile parameter, not stored state;
4. **ocean heat capacity** — a mixed-layer depth;
5. atmospheric **mass**, to get pressure, hence wind, hence advection of the water stock.

Items 4 and 5 are each **one stratum**. That is the whole of "some degree of water column and air column up front," and it is the crudest rung of the strata object rather than a separate atmospheric program.

Item 4 carries a #norm-physics-not-knobs finding worth naming on its own. #detail-phenomena-systems-map lists the crude climate product as "lat+elev+continentality → temp/precip" — with **continentality as an input**. Continentality is not a primitive; it is the observable consequence of the ocean's heat capacity against land's. A declared mixed-layer thickness is the physical term, it is one stratum, and it *produces* continentality instead of taking it as a proxy. That is the norm's exact shape: the real term about the same size as the knob, and terminal where the knob is not.

**The ordering claim, which is the part I would defend:** the ocean enters weather as thermal inertia well before it enters as circulation. Heat capacity is what makes land-sea contrast, seasonality, and therefore non-zonal precipitation. Ocean *circulation* — heat-transport bands — is correctly listed as `defer` in the systems map, and this does not move it.

## Loops, not phenomena — and a tested verdict on whether this is one piece of work with the cycle

Joseph asked, separately: *"How do we even have water erosion and lakes without evaporation or even statistical weather?"* The audited answer is that the cycle is closed **by assumption rather than by dynamics**, and every consumer substitutes a proxy: stream power uses drainage area for discharge, the evaporation constants live in a kernel covering $40\ \mathrm{s}$ of world time ( #obs-water-fill-never-settles), the wet-limit lakes carry no balance term, climate is a static fated field, and the conserved hydrosphere inventory sits above all of it exchanging nothing with any of them. The budget is honest and stated; the flux is what is missing. The gate that should have caught this is audited above.

Two things follow, and one of them corrects a framing I was about to write.

**Deferability is often a property of a loop, not of a phenomenon.** Evaporation is individually deferrable and is simultaneously what makes lake level, endorheic basins, rain shadow, salinity, and evaporite deposition reachable — none of which is reachable at *any* fidelity without it. That is a better unit than the phenomenon, and it gives Joseph's *"defer indefinitely unless someone needs to reach a downstream phenomena naturally"* a principled form: **a phenomenon is safely deferrable when it is a leaf of its loop, and structurally blocking when it is the loop's closing edge.** Tides are a leaf. Evaporation is a closing edge. They look similar on a phenomena list and behave oppositely.

**The lake answer is already canon, and it is not "add evaporation to the water kernel."** #form-temporal-lod-regimes **regime E** names exactly this class — "water surfaces, soil moisture, temperature profiles… history-free *in principle* but the function is **implicit**" — and prescribes *relax or solve*, with "where a direct solve exists, prefer it." A lake level under a water balance is a regime-E attractor. What is built is a *geometric* upper bound (fill to the spill point), which is why #obs-connectivity-fills-the-basins-the-threshold-drained can honestly claim only a wet limit. And the direct solve is already named as prior art to adopt in a segment landed tonight: Fill–Spill–Merge carries "a closed-form lake level for the partially-filled endorheic case," and CHONK 1.0 "carries lakes coupled to erosion including an evaporation term in the level balance."

So evaporation does **not** need a new temporal rung. Trying to evaporate anything inside a $40\ \mathrm{s}$ kernel is hopeless by construction, and that is the wrong place to look. This also sharpens the tides case and corrects an over-reach I nearly made: **the split is between attractor-seeking quantities, which need a solve, and genuinely oscillatory ones, which need a rung.** Lake level, soil moisture and salinity are the first kind. Tides, storms, seasonality and diurnal cycles are the second, and *those* are blocked on a missing sub-annual band — nothing runs between $40\ \mathrm{s}$ and an erosion epoch. Blocked in time, not in space.

**Is closing the cycle the same work as making the column primitive? Tested, and: partly — narrower than it looks, and the narrow form is still a reason to do them together.**

Against the unification, and I think this is right: **closure does not require layering.** Three conserved scalar fields per cell — soil moisture, surface water, precipitable water — close a budget perfectly well, and that is what bucket-style land-surface models do. Layering is required for structure *within* a reservoir (thermocline, brine, ice over water, water table under an unsaturated zone), which is a later rung. Asserting that the cycle needs strata would be the over-unification, and it would be false.

For a narrower link, which does hold:

1. **The reservoirs a budget must move water between are already column fields, correctly placed.** `Stratum::saturation` is pore water; `Column::water_depth` is standing water. The cycle's spatial home exists and is unused. So the column is the natural *home* for the reservoirs rather than a *precondition* for the cycle.
2. **Capacity is strata-shaped even at the crudest rung.** How much pore water a column holds is porosity times thickness, summed over strata — and porosity is already a per-material property in the interface. A single lumped soil-moisture field would need an *effective* porosity, which is a knob standing in for the strata that are already there: #norm-physics-not-knobs, with the physical term about the same size as the proxy.
3. **The decisive one: closing the cycle anywhere else reproduces the pattern this spike is about.** A new lumped soil-moisture array would be a sixth independent representation of where the water is, added to the five already counted. The argument for doing these together is not that closure needs strata — it is that closure done *beside* the column manufactures exactly the defect being repaired.

## Deferability, on the phenomena Joseph named

The systematic survey is the sibling file `phenomena-reach.md`. These four are the ones his message named, and they turn out to sort cleanly by the same axis.

**Coastal erosion — gated, and the two halves of this spike disagree about on what.** It is "a big one" for a structural reason rather than a difficulty reason. My read: a crude rung needs a shoreline as a real object (the ocean mask as one world object — hotspot 1b, already ranked), a wave-energy proxy from fetch, and **differential erodibility**, because the difference between a cliff and a beach is a hard stratum undercut at the waterline.

The sibling survey concludes instead that coastal erosion is gated on a **wind field**, not on a water column, and flags it as the verdict it most wants argued with. **The disagreement is left standing rather than resolved, because it is a real fork and not a wording gap.** My position: wind is what makes wave energy non-fake, so it gates the *honest* rung, while fetch-plus-waterline plus differential erodibility gates the *crude* one — which would make erodibility the earlier gate and wind the better one. That reading could be wrong in a way the survey's author is better placed to judge, since they read `flux.rs` and the ordinum promises closely and I did not. Whoever takes this next should read both and pick; the useful thing is that the fork is now visible and named on both sides.

**Tides — deferable indefinitely, and the cost of deferring is nameable.** The equilibrium tide has a closed analytic form from an orbiting body's position: approach code **A** in the systems map, cheap whenever wanted. It is deferable because *nothing currently runs at a diurnal rung at all* — the water kernel covers $40\ \mathrm{s}$ of world time ( #obs-water-fill-never-settles, kernel-era-scoped) and erosion runs in epochs, so there is no temporal band for a tide to act in. What deferral costs is the intertidal: tidal flats, estuarine sedimentation, tidal mixing, and anything ecological keyed to a wet/dry cycle — each of which needs both a sub-daily rung *and* a composition-bearing water column. Joseph's instinct that tides are deferable "unless someone needs to specifically reach a downstream phenomena naturally" is exactly right, and the downstream set is the intertidal one.

**Settling lime deposits — not deferable in the sense that matters**, because it is not a phenomenon to add later; it is a *test* of whether the water column carries composition. Treated as a feature it is expensive and isolated. Treated as a source term into strata deposition it is nearly free and brings its whole family.

**Weather — above.**

The pattern across all four: **the deferability of a phenomenon is mostly a fact about which primitives it needs, and only secondarily about its own difficulty.** Tides are analytically easy and deeply deferred; differential erodibility is a small change and blocks a large family. A build order sorted by phenomenon difficulty will mis-rank both.

**And the sibling survey found the sharper form of this, which I want to carry rather than restate weakly.** Its finding on tides: canon *already* handles them as Joseph is reaching for — `declare-causally-materialize-lazily` is a dictionary-grade term, the moon is declared at Phase 1 with tides named as a lazily-materialized service, and what the mechanism lacks is a *reachability* field saying what stays unreachable until materialization. Then the observation I would not have reached:

> the cost of the tide deferral was not paid in missing tides; it was paid in an undeclared datum

— because the sea level never had to *move*, it never had to become an object, so it stayed a constant outside the flux web and four consumers grew four thresholds for it. Generalized: **the cost of deferring a dynamic is usually paid in the object that dynamic would have forced to exist, and that cost lands whether or not anyone wants the dynamic.** That is a better statement of Joseph's deferability question than mine, and it also explains why this spike's subject and the deferral question are the same subject: a static world does not need its quantities to be objects, so every deferral quietly licenses one more local answer.

## What it costs now, and honestly what it would have cost at the start

Joseph's framing invites *should this have been the primitive from the start*, so: **yes, and the reason it is not is legible and is not a misjudgment.** `Fluvial` arrived as a **port** of an existing heightfield kernel (`ref/erosion-port/NOTES.md`), so the heightfield came in as inherited structure rather than a chosen representation — and the material hook has been flagged in its header since the port.

**The dates say something I did not expect, and it corrects my own recommendation below.** First-commit dates: `column.rs` and `erosion.rs` both land **2026-07-01**, in separate commits on the same day — so neither routed *around* the other; they were born in parallel and never introduced. #form-column-control-volume lands **2026-07-21**, formalizing the strata column as canon. And `lithosphere.rs` — the *second* struct named `Column`, the one that takes the conservation duty — lands **2026-07-24**, three days *after* the segment declaring what a column is.

So the divergence is not ancient debt predating the claim. **It widened three days after the claim landed**, and the new arrival took the mass ledger with it. This is not a charge against that work: `lithosphere::Column` solves crust-and-keel isostasy, where stratigraphic order is genuinely irrelevant, and it solves it well enough to carry a closed-box conservation probe. The point is narrower and it is about mechanism, not diligence — **a `status: exact` segment naming the column primitive did not, on its own, have the force to attract an implementation that needed a column three days later.**

Which is exactly what #norm-declaration-must-convict predicts, and what FORMAT's own second open question asks ("a declaration which cannot fail a build is a wish"). It is also the strongest available argument against the *first* thing I would otherwise have recommended, so it is recorded here rather than in a footnote.

The cost now splits cleanly, and unevenly:

**Cheap now** — no kernel rewrite, each an independent step. I had drafted my own list here and am deferring to the sibling survey's, because its **G1–G9** decomposition is the better carve and a second taxonomy beside it would be the "invent a second home" failure this project warns about. In its terms the cheap set is **G3** (one free surface per connected body — the ocean mask, the datum, and lake level as one object; already ranked hotspot 1b), **G1** at its consolidating end (a column processes write into — which *deduplicates* three existing ad-hoc `WaterSim` fields and discharges two declared FE(6) scaffoldings rather than adding a subsystem), the vertical boundary contract (below), and a solute slot (**G6**), which is the one item that cannot be deferred past Abyssal without failing a declared charge.

One finding inside G1 is worth lifting out because it is #norm-physics-not-knobs in miniature and it argues the direction on its own: `WaterParams::gw_capacity` is a bare `0.3` standing in for porosity times thickness, while `material.rs` **already carries `porosity` per material, unconsumed**. The physical term is present and something else is being read instead of it. A stored column closes that with no decision required — and the knob has no `ASSUMPTIONS.md` row, so it is currently unaccounted as well as unprincipled.

**Expensive, and the real subject** — strata-aware incision. `Fluvial::h` is the hot kernel; making it read a stack per cell is a genuine performance and scheme question, and it interacts with the GPU-as-rung note in #detail-water-parallelism. I have not costed it and would not guess. It is also where the payoff sits, so the honest statement is that the cheap set above is worth doing on its own terms and none of it buys differential erosion.

**A mechanical cost on every step:** any edit under `crates/vivarium-world/src/` moves `SRC_HASH` and invalidates the store, so each of these is a staged change with a rebuild. The tree has the cohort mechanism for exactly this (the keys-only shape recorded as `2026-07-29-02`), so this is scheduling rather than an obstacle.

## The two open ends want a vertical boundary contract

This is the one place I would coin rather than extend, and it is a small coinage.

#form-declared-boundary-contract governs *horizontal* domain edges, with `Undeclared` deliberately first-class "so the column *exists* and its absence is visible debt, not silence." A column has two **vertical** domain edges. Neither is declared, and both are currently overloaded onto `Void` or onto nothing. The extension writes itself:

- **Top:** `OpenToAtmosphere` (today's meaning, honestly named) versus an air stratum with mass. Distinguishing *above the world* from *cave void* is the same repair, and it costs one variant.
- **Bottom:** `Undeclared` — which is the truthful present value — versus `CompensationDepth{z}` (what `lithosphere::Column` means) versus `Unresolved` (Joseph's `unknown/undefined`, and the correct value for a column whose bedrock is a datum rather than a bottom).

Joseph's `unknown/undefined` is doing real work here. A column that bottoms out in a *declared unknown* is honest; one that bottoms out at an undeclared datum reads as a claim that the world ends there. And the same argument that makes the horizontal contract the flux web's business rather than kernel privacy applies verbatim: the vertical ends are coupling surfaces — the top to the atmosphere, the bottom to the mantle — and #form-isostasy-column's chain already crosses the bottom one without naming it.

## Two findings from the census that outrank this spike

Both are incidental to the question and neither is mine to adjudicate. Raising them because a strata primitive built on these beds would inherit the first, and because the second explains a standing puzzle.

**The committed store holds NaN heights.** Measured by direct payload scan: **83 of 3552 current-cohort `erosion-tile` roots contain NaN**, $55\,391$ cells of $14\,548\,992$ stored (0.38%), across 11 distinct tile origins, worst tile 87.9% NaN. The NaN fraction against epochs at that origin runs $7.5, 15.7, 28.7, 46.8, 10.7, 11.0, 11.5, 87.9$ — it grows, partly resets, then blows up, so it is not simple monotone propagation and the census does not explain the pattern. **Two clamps make it silent rather than loud:** `WaterSim::new_at_sea`'s `(sea - b).max(0.0)` returns the non-NaN operand, so a NaN bed becomes $0\ \mathrm{m}$ of water — dry land, without warning; and `globe.rs`'s `unwrap_or(sea)` guards an out-of-range index rather than a NaN value, so NaN reaches `elev < sea`, compares false, and paints as land. This is #norm-probes-before-claims territory and wants its own probe.

**`Coverage` cannot see the built water.** It reports `watered_tiles=0` while **384** current-cohort water roots sit in the store, because it keys at the deepest *surface* level (13 on this world) and every water tile is at level 9. That is the mechanism under `vivarium-explore/src/water.rs`'s standing note that the water field has never been rendered: the census feeding the view reports the field as absent.

**One stale doc, cheap to fix:** `DrainageSurface::standing_water`'s comment still says `Fluvial::outlets` classifies sea by elevation threshold — it became connectivity-aware at `erosion.rs:751-807`, so the paragraph now describes a repaired defect.

## Two smaller gaps, both cheap and both mechanical

**The dictionary does not define the primitive.** There is no `LEXICON.udon` entry for *column* or for *stratum* (measured in the sibling survey). `CLAUDE.md` routes "term meanings" to the LEXICON and #form-column-control-volume FE(4) turns on the difference between a point sample, a cell average and a band-limited reading — so a term that two different structs claim, that **12 segments cite by slug and 37 of 109 mention by name** (measured), and whose reading semantics are an open fork, has no dictionary home. Coining the entry is small and would give the name one place to be pinned.

**The sea-level datum travels outside the flux web.** The sibling survey found it arriving as an *assumption* string on `water-tile` with no keeper on the ordinum promise — and this is where the tide deferral has been charged all along, silently. It also sits next to the known-stale declaration that `9dd86ab` deliberately named rather than guessed at, so the two are one repair with one `SRC_HASH` move.

## A correction owed to a segment

#form-declared-boundary-contract FE(5) reads: *"Vertical mechanisms (keys, chains, columns) conserve within a grain; the horizontal joins between grains are where 2026-07-28's three largest defects all lived."*

The second clause holds. The first is now doubtful as written, and the segment's own sibling supplies the counterexample: *carved tiles do not debit their columns* ( #form-isostasy-column FE(2), FE(9)) is a vertical mechanism failing to conserve **across** grains — pour-grain ledger against tile-grain fluvial — and FE(5) itself lists that pair one clause later as a grain boundary. So the sentence's general claim is contradicted by its own next item. Worth a pass; I have not edited it, and the honest repair is probably to narrow the first clause rather than drop it.

## What I did not verify, and where I would break this

Marked so nobody inherits my confidence for someone else's measurement.

- **Not measured: the cost of strata-aware incision.** The central engineering question, untouched here.
- **Not verified: that the closed-form endorheic lake solve fits regime E as cleanly as I claim.** The prior-art citations are quoted from #form-ocean-is-connectivity-not-elevation's Working Notes; I did not read Barnes et al. 2021 or Gailleton et al. 2024 myself, and the dossier at `msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md` marks what it verified against what it could not get. Treat the regime-E fit as reasoning over a citation, not over a source.
- **The count of five threshold sites is mine and is not a sweep.** I found the constructor instance by following one call path; the sibling census is the better number.
- **Not measured: store size under run-length strata.** I argue it is near-neutral for dry columns because run-length is cheap; I did not measure it, and a world where most columns carry several thin strata could behave differently.
- **Reasoned, not measured: the weather ordering.** The claim that heat capacity precedes circulation is physical reasoning plus a read of `climate.rs`, with no probe.
- **The strongest case against this spike** that I could not dismiss: the flux-web and seam machinery is mature and the vertical machinery is not, so a large vertical change lands against less scaffolding than its horizontal equivalent would. I checked the specific form of this and it came out differently than I expected, so both halves are recorded: the restriction/lifting seam duty that #form-column-control-volume FE(6) places on columns ("must honor the statistics each level declared") has **no implementation at all** — no `restrict` or `lift` function exists in the world crate for any quantity, not merely none for strata. So the asymmetry is real, but the missing piece is a general one rather than a strata-specific gap, which weakens this as an objection to *strata* while strengthening it as a caution about the vertical axis overall. It argues for staged consolidation rather than against the direction.
- **Deliberately not adjudicated:** the boundary `ETHICS.md` §Scope leaves open — whether the exogenous-only ceiling excludes simple non-emergence-capable endo minds or only emergence-capable substrates. It is named as open there, and it stays open here. The moratorium constrains a *method of instantiation*, not planning or representation; modelled intelligent civilizations are intended, with truthfulness to any LLM-substrate inhabitant as the binding companion constraint.

## If I were choosing the next move

Stated as my judgment, not as a plan anyone adopted.

The reframing is worth landing as a claim, because it changes what six existing open rungs *are* and each is currently being reasoned about locally. Concretely: the column as the vertical primitive with **strata count as a rung and layer count in the key**; the vertical boundary contract; and the collection of the rungs under one object. That needs no kernel change to be true.

**But the dates above are evidence that a segment alone will not hold this**, and I would rather say so than recommend the comfortable thing. #form-column-control-volume is already `status: exact` on precisely this frame, and a second `Column` arrived three days later. So the segment is worth landing *and is not the mechanism*. The mechanism the project's own norms point at is a **conviction**: #norm-declaration-must-convict, and FORMAT's second open question — a declaration that cannot fail a build is a wish. The cheapest convicting instruments I can see, offered as candidates rather than a design:

- the ocean mask as **one object** with the threshold sites deleted, so a sixth independent answer has nowhere to live (this is hotspot **1b** and is already ranked);
- a probe that fails when a water quantity is read from anywhere but the column — which is the shape that would have caught tonight's bypass;
- `water_depth` retired in favour of a water stratum, so the *type* stops permitting a scalar answer to a layered question.

Each is small, each stands alone, and each removes a way for the pattern to recur. That ordering — conviction before claim before kernel — inverts what I would have recommended an hour ago, and the reason is the 2026-07-24 date.

**And the first move is cheaper than any of those and sits in the ordinum.** Give the phase-3 gate charges instruments: a falsifiable predicate on `weather-channels` (which has none), and a `closed-water-cycle` predicate that tests a **flux between reservoirs** rather than an algebraic identity that cannot fail. That is data, not library source — so it moves no `SRC_HASH`, invalidates no store, and can land tonight. Its effect is to convert an implicit deferral into a standing audit finding, which is the mechanism the rest of this list is trying to buy. If only one thing from this spike happens, I would make it this one, because it is the only item that makes the *next* deferral visible rather than repairing a past one.

**And one correction the census forces on this list, against the direction I was drifting.** I had been sliding toward *build on `column::Column`*. That is untenable on the measurements: its datum is unrelated to the lithospheric column's (the strata column's "bedrock datum" sits inside the top few percent of `crust_m` — measured $2350\ \mathrm{m}$ against $30\,000\ \mathrm{m}$ at one address), its two strata are invented at read time, its `saturation` is write-only, its derived queries are test-only, its `water_depth` is the retired threshold, and its constructor drops the seed. The census's caution is the one to keep: **none of those is an argument against stratification**, because every one is a property of `column.rs` plus `gen.rs` rather than of the idea. So the object wants designing from the kernels' existing state — the eight arrays above — rather than growing from the existing type.

Then the strata-aware incision question, with a costing spike of its own — of everything here it is what I would least want decided by argument rather than measurement, since the objection to it is a performance claim and no performance claim in this document is measured.

What I would resist is the enthusiastic version: adding water and air *layers* before anything runs at those layers. #form-fidelity-ladder FE(11) is aimed precisely at that move, tonight, in advance.
