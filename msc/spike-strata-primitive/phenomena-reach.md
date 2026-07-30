# Phenomena reach — what each phenomenon is gated on, and which gates turn out to be one gate

A reachability survey rather than an inventory: for the phenomena this world will want, what has to exist before each is *statable at all*, which of those requirements are the same requirement under different names, and what becomes unreachable by deferring each one.

Spike material. Nothing here is adopted by being written down, and several verdicts below are mine rather than the project's — they are marked.

## 0. Register discipline for this document

Two registers are kept apart throughout, because a survey that mixes them hands the next reader belief wearing measurement's confidence.

- **Measured / read** — a statement I checked against the artifact, with the artifact named. Everything in §2, §3's collapse claims, and the counts in §9 are of this kind. Code read 2026-07-29 at the tree's then-current `main`; the ordinum, `LEXICON.udon`, `ETHICS.md`, `FORMAT.md`, `core/OUTLINE.md`, ` #detail-phenomena-systems-map`, ` #form-fidelity-ladder`, ` #form-column-control-volume`, ` #disc-known-active-hotspots` and ` #disc-open-problem-census` read in full the same date. `msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md` read by section headings and §0 only — named, partially read.
- **Judgment** — the deferability verdicts (§7), the gate decomposition of coastal erosion and weather (§6), and the mechanism proposal (§9). Argued, not measured.

Where a number is quoted from a segment rather than re-measured here, it is attributed and inherits that segment's era caveat; nothing below re-measures anything, so no new numbers are minted.

## 1. The question I ended up answering

The brief asked what each phenomenon requires in order to be representable, and which requirements are the same requirement wearing different names. I kept that question, with one shift the material forced.

The shift: **the project already owns a reachability mechanism, and it is the flux web, not a document.** `crate::audit` resolves each nomos's `consumes` against the registry's `promises` and reports `Unmet` where no nomos produces the quantity — the "rain without a sky" finding, which is exactly a reachability verdict computed rather than asserted. `flux.rs` carries a deliberate scope discipline in its own words: *"Only quantities that a real nomos today produces or consumes live here… vocabulary is not built ahead of a nomos that speaks it."* So the honest form of this survey is not a parallel list of prerequisites; it is a statement of *which quantities the deferred phenomena would name*, held outside the vocabulary until a nomos speaks them, plus a criterion for the one case where early naming is right. `PRECIPITATION` gives that criterion in the file's own reasoning: it was named ahead of its producer because two live nomoi were already silently assuming it, and naming it converted a hidden assumption into a mechanical audit finding. **Name a future quantity early iff some live nomos already assumes it silently.** §9 applies that test and finds two or three candidates, not twenty.

The second shift is smaller. ` #detail-phenomena-systems-map` already carries the inventory, the timescales and a build order, so this document deliberately does not re-list systems. What it adds is the gate structure and the cost-of-deferral, which that map does not carry.

## 2. What exists to be gated on: four vertical objects, and none of them is stored

The vertical axis is where the brief's suspicion lands, so this is the measured ground everything else rests on.

| object | where it lives | what it carries | status |
|---|---|---|---|
| **stratigraphic column** | `column.rs` — `Column { tile, strata: Vec<Stratum>, water_depth }`, `Stratum { material, thickness, saturation }` | ordered material runs, real-valued thickness; elevation, `b`, `r`, overburden all derived | **assembled at read time, never stored** |
| **lithospheric column** | `lithosphere.rs` — a second type also named `Column { crust_m, crust_rho, keel_m, sediment_m }` | crust + depleted keel + returned sediment as a conserved inventory | **real, keyed, and read by isostasy** — the flux quantity `LITHO_COLUMN` |
| **surface-water state** | `water.rs` — `WaterSim`, seven physical per-cell arrays (`bed`, `depth`, `sediment`, `groundwater`, `sed_bed`, `colmation`, `armor`) plus a Kahan residual | depth-averaged hydraulics, one subsurface bucket, two interface scalars | **real state, but region-local — no vertical structure, and not a column** |
| **atmosphere** | `hydrosphere.rs` — `atmosphere_km3`, one global scalar | a mass of water vapour with no location and no thickness | **no spatial extent at all**, let alone vertical |

Three consequences follow directly, and they are measurements rather than opinions.

**(a) The stratigraphic column is a read-time reconstruction of a scalar height field.** Its sole producer is `gen::column_from_surface_at_sea`, which takes a surface height and a soil thickness and returns at most two strata: one undifferentiated igneous run of `surface − soil`, and one soil run of 2 m. `erosion::column_at` wraps it over the tier telescope. No kernel writes a stratum, and `Stratum` appears in exactly three files (`column.rs`, `gen.rs`, `material.rs`). So the vertical structure of the world is currently *added after the physics ran*, from a level no rung executed — which is the situation ` #form-fidelity-ladder` FE(8) and FE(11) describe, transposed from the horizontal frequency axis onto the vertical. FE(11) predicts this in general terms (*"any field that acquires a detail increment at read time is invisible to the process it should be constraining"*) and names submarine strata refinement as an unbuilt instance. The measurement here is narrower and sharper: it is not only unbuilt, the read-time-increment shape is *already how strata work today*, for the whole planet, at every level.

**(b) Two different objects are both called `Column` in the same crate.** ` #form-column-control-volume` FE(1) defines the column as the stratigraphic stack with standing water above it; `lithosphere::Column` is a crust-and-keel inventory in the same tree under the same word. They are complementary depths of one physical column — the vocabulary has no name for their union, and `LEXICON.udon` carries no entry for *column* or *stratum* at all (its `canonical-frame` and `tile` entries refer to them, pointing at the segments instead). The dictionary gap and the type collision are the same gap.

**(c) The waterline travels outside the flux web.** There is no sea-level nomos and no sea-level flux quantity; `water-tile` lists `SEA_LEVEL_M` among its *assumptions*, and `gen.rs` keeps the retired 4000 m decree as a deprecated constant for migrating call sites. The ordinum's Phase-3 `sea-level-datum` promise has no `:kept-by`. Meanwhile ` #disc-known-active-hotspots` rank 1b exists because four consumers hold four independent thresholds for the same classification. The datum every elevation is read against is the one quantity the coupling contract does not govern.

## 3. The gates, collapsed

Nine enabling objects account for the phenomena in ` #detail-phenomena-systems-map` and the ordinum's charges. The collapse is the useful half: several long lists of "missing features" are one missing object seen from different consumers.

**G1 — a column processes can write into.** *Nine names, one requirement.* Snowpack · soil horizon · alluvium (`sed_bed`) · carbonate blanket · banded iron · coal seam · ore host rock · colmation · armor lag are all *a stratum with a material and a thickness that a process created*. Today three of them exist as parallel ad-hoc per-cell scalars in `WaterSim` because there is no stored column to push a stratum onto, and two of those three (`armor`, `colmation`) already carry demolition dates under ` #form-fidelity-ladder` FE(6). So G1 is not a new subsystem; it is the deduplication of existing fields plus the discharge of two declared scaffoldings. It also retires at least one knob, and the retirement is unusually clean: `gw_capacity` is a bare `0.3` default on `WaterParams` standing in for porosity × thickness of the strata — and `material.rs` **already carries `porosity` per material**, unconsumed. So the physical term is present, the knob is what reads instead of it, and a stored column closes the gap without anyone deciding anything (` #norm-physics-not-knobs`). Two smaller notes ride with it: the constant has no `ASSUMPTIONS.md` row, which is the unbuilt ledger→declared direction that ` #disc-open-problem-census` names as the assumptions reverse-guard; and `gen.rs`'s soil-saturation row *is* ledgered, at **arbitrary**, naming the groundwater/Darcy tier as its cure.

**G2 — a penetration depth.** *Five names, one shape.* Ocean mixed-layer depth · atmospheric boundary layer · photic zone · weathering front / soil production depth · wave closure depth are each *the depth over which a surface flux is distributed before it stops mattering*. This is the cheapest honest form of a column: **one layer whose thickness is set by the physics**, not N layers. Naming G2 separately from G1 matters because it is what lets a water column or an air column enter at a rung that actually runs — the alternative reading, that a column means a vertical discretisation, is what makes every G2 consumer look expensive. Currently: no mixed layer, no boundary layer, and `groundwater` has a *declared* capacity where a derived penetration depth belongs.

**G3 — a free surface with one head per connected body.** *Four names, one requirement.* Sea-level datum · lake level · groundwater head · ocean mask are all *the free surface of a connected water body at hydrostatic equilibrium*. ` #form-ocean-is-connectivity-not-elevation` establishes the connectivity half and ` #obs-connectivity-fills-the-basins-the-threshold-drained` measures what repairing it bought; rank 1b lifts the mask into a shared object. The two halves nobody has named as belonging to the same object are the **datum** (§2c) and the **groundwater head** — a per-cell bucket has no head, so subsurface water cannot flow laterally and cannot have a table that intersects the surface (which is what a spring is).

**G4 — an energy budget per column.** *Three names, one requirement.* Insolation · geothermal / mantle $T_p$ · latent heat of the water cycle are all *energy crossing a column boundary*. Two of the three exist and one is missing, and the missing one is load-bearing: the water cycle as built conserves **mass and not energy**, which is why `ATMOSPHERE_RESIDENCE_YR` exists. That constant is a knob whose real physical term is the surface energy balance — evaporation is set by available energy, not by a residence time chosen to reproduce it. `climate.rs` is candid about the standing: *"a pattern claim from this precipitation would be false; only its global mean is meant."*

**G5 — a surface temperature field.** Distinct from G4 (which is the flux) as state is distinct from forcing. Nothing in the tree carries a surface or air temperature. This is the gate with the least work behind it and the most behind it at once — see §6.

**G6 — solute load.** Water carries one suspended-solid scalar and no dissolved species. Karst · lime and carbonate deposition · evaporites · hydrothermal veins and ore · biomineralization · banded iron · the nitrogen and companion cycles all require it. It is the sharpest reachability result in this survey because the dependents are not wishes: two are ordinum **charges** (Abyssal `mineral-systems` `:tag gate`, `biomineralization` `:tag earth`), one is a Primeval charge (`biogeochemical-cycles`), and Abyssal's `|record` is *banded iron formations and the first limestones* — chemistry written into rock, which additionally needs G1 to have somewhere to write.

**G7 — grain-size classes in transport.** Already named as the highest-leverage absent rung in ` #detail-phenomena-systems-map` FE(5), so it is reported here as known rather than found. Worth one precision: `material.rs` carries `grain_size` as a per-material *property*, so what is absent is the transport-side split (a sediment population with classes that sort), not the property. Gates settling and Stokes · saltation and bedload · sorting · beaches · dunes · flocculation · a legible sedimentary record.

**G8 — a wind field.** A 2-D atmospheric field of column-integrated quantities (mass, vapour, temperature) with horizontal momentum. Gates aeolian transport · wave energy and therefore most of coastal erosion · orographic precipitation · evaporation as a local rather than global flux. Note the split from a multi-level air column: G8 is the *spatial* rung, G2-applied-to-air is the *vertical* rung, and they are independently affordable. `flux.rs` names `INSOLATION` as produced; `nomotheke.rs` names it in no `consumes` — a promised quantity with no consumer, which is the whole G8/G5 branch waiting on its downstream side rather than its upstream side.

**G9 — a time-varying datum with an external forcing.** Tides, and the reason tides is the shape the brief points at. See §5.

Two requirements I expected to find distinct and did not: **coastline as an object** collapses into G3 (a shoreline is the boundary of the connected body, derivable once the mask is one object), and **stratigraphic dating / the record** collapses into G1 plus time-in-the-key, which ` #form-time-indexed-stage-chains` already owns.

## 4. Reachability table

Read the *cost* column as the honest content of a deferral: what stops being reachable, not how hard the thing is.

| phenomenon | gated on | verdict | cost of deferring |
|---|---|---|---|
| Coastal erosion (cliff retreat, beaches, deltas) | G8, G3, G7, G1 | **not gated on a water column** — see §6 | the wave-energy half is unreachable without a wind field; the deposition half without grain sizes |
| Tides | G9 (+ ephemeris, already declared) | **defer indefinitely** | the intertidal band and everything periodic-wet-dry; the tidal-mixing term in a real ocean circulation |
| Weather, nominally circulating | G8, G5, G4, G2-on-water and G2-on-air | **the near-term unlock**; enters at a slab rung | while deferred, no precipitation *pattern* claim is available, so every downstream geography claim inherits the falseness `climate.rs` declares |
| Ocean circulation (dynamical) | G2-on-water, G5, G4, G8, G9 for abyssal mixing | **defer the solve, build the slab** | heat-transport bands; deep-water formation |
| Lime / carbonate settling | G6 × G2-on-water (saturation state with depth) × G1 | **not deferable past Abyssal** without failing declared charges | the Abyssal record; the biomineralization charge; the first limestones |
| Karst, caves, sinkholes | G6, G1 | **defer indefinitely** — cheap once G6 exists, unreachable before | caves and sinkholes only; nothing gates on them |
| Weathering / pedogenesis | G5 (freeze–thaw), G6, G1 | **defer the process, not the object** | differential hardness and soil depth — which erosion already wants |
| Glaciation / ice | G5, G1 (ice as a stratum), G2 | **defer indefinitely** | U-valleys, cirques, fjords, moraines; a large albedo feedback in any later climate |
| Aeolian (dunes, loess) | G8, G7, G1 | **defer indefinitely** | dunes and loess; a sediment pathway that is not fluvial |
| Hydrothermal / ore | G6, G1, G4 | **defer indefinitely** | the ordinum's `ore-and-soil` promise, which is already a gloss |
| Seismicity as events | fault geometry (from the column, partly present via sutures) | **defer indefinitely** | scarps and offsets |
| Isostasy / rebound | built | — | — |
| Biomes | G5, G4 (Whittaker needs temperature and precipitation *fields*) | **reachable the moment G5/G8 land** | the whole biosphere branch, which has no segments at all today |
| Ecosystems, succession, evolution | biomes, plus its own machinery | **defer** | Primeval and later charges; see §10 on why this is not a moratorium question |
| Groundwater as a flow | G3 (a head), G1 (porosity from strata) | **near-term and cheap** | springs, exfiltration, wet-ground slumping — three items on the standing explorer-intent list |
| Submarine strata refinement | G1, G2-on-water | **defer** | named in ` #form-fidelity-ladder` FE(11) as the cheap-now moment to get the ordering right |

## 5. Tides, worked, because it is the shape

The brief points at tides as the pattern: defer indefinitely unless someone needs a downstream phenomenon reachable only through it. Working it through produces a result worth more than the verdict.

**Canon already handles tides exactly this way, and it is the ordinum's own worked example.** Phase 1 `charge[protective-architecture]` declares the moon, with its reason stated: *"The moon cannot be added non-destructively later… so it is declared here — declare-causally-materialize-lazily; its real services are ocean tides, moonlight and phases, and axial-tilt stabilization."* `declare-causally-materialize-lazily` is a settled `LEXICON.udon` term. So the mechanism the brief is reaching for exists, is named, is dictionary-grade, and has tides as its canonical instance.

**What it lacks is the reachability half.** The pattern says *the cause is declared now and the effect is computed later*. It does not say *which downstream phenomena are unreachable until the effect is materialized*. That is the gap this survey actually found: not a missing inventory, a missing second field on an existing mechanism. Concretely, for tides the unreachable set is: the intertidal band as a *place* (periodically wet and dry, which is what mudflats, tidal marsh and the associated ecology are), spring–neap sediment sorting, estuarine tidal currents and bores, and the tidal contribution to abyssal ocean mixing. What is *not* in that set, contrary to a natural guess: coastal erosion, which is wave-driven.

**And deferring tides has been quietly charged already.** Because the datum never had to move, it never had to become an object — so it stayed a constant outside the flux web (§2c) and four consumers grew four thresholds for it. The cost of the tide deferral was not paid in missing tides; it was paid in an undeclared datum. That is the general lesson I would carry out of this: *the cost of deferring a dynamic is usually paid in the object it would have forced to exist*, and that cost lands whether or not anyone wants the dynamic.

## 6. Three phenomena whose gates are not what they look like

**Coastal erosion is gated on wind, not on water depth.** Joseph names it as a big one, and the natural reading is that it needs the sea to be real in the vertical. Decomposed: cliff retreat is wave energy delivered along a shoreline against a failure criterion; the shoreline is G3 (already half-built, and rank 1b is lifting the mask); the failure criterion is Mohr–Coulomb over $C$ and $\varphi$, and `material.rs` already carries a property interface for exactly those; the beach and delta half is G7. The one gate with nothing behind it is **wave energy, which needs a wind field and a fetch** — G8. So coastal erosion is a G8 dependent wearing a hydrology costume. This is judgment, not measurement, and it is the claim in this document I would most want argued with.

**Weather does need a water column up front, and the honest rung is one layer.** The brief's parenthetical suspicion holds, with a sharpening. For circulation to be *rational* rather than merely present, the ocean has to have a heat capacity — sea-surface temperature must respond to insolation on a physical timescale, which requires a mixed-layer depth. That is G2, one layer with a physically-set thickness, not a stratified ocean. Same on the air side: a 2-D atmospheric field with a boundary-layer thickness, not a multi-level column. So the sequencing that ` #form-fidelity-ladder` demands — enter at a rung that runs — is available cheaply here, and the expensive readings (a layered ocean, a baroclinic atmosphere) are the ones to refuse for now. The measured fact that makes this the near-term unlock rather than a large build: **`INSOLATION` is already produced, keyed, and consumed by nothing.** The driver of the whole branch is paid for.

**Settling lime deposits is three requirements, not one.** Joseph's aside names it as though it were one phenomenon. It is G6 (a dissolved species) × G2-on-water (a saturation state that varies with depth, since carbonate solubility is pressure- and temperature-dependent — which is *why* the carbonate compensation depth is a depth) × G1 (a column that can receive the resulting stratum). It is the best specimen in this survey for the brief's question read in reverse: the interesting collapses are not only many names for one gate, but also one name for several. A deferability verdict on "lime" alone is not statable.

## 7. Verdicts I will defend

**Defer indefinitely, and I would say so in canon:** tides · karst · aeolian · glaciation · hydrothermal and ore · seismicity-as-events · dynamical ocean circulation · speciation. Each cost is in §4, and none of those costs blocks a declared charge before Primeval.

**Do not defer, because a declared charge fails without it:** solute load (G6), by Abyssal — `mineral-systems` is `:tag gate`, meaning a hard prerequisite the project intends to simulate, and Abyssal's record is a chemistry record.

**Do not defer, because the deferral is being paid for anyway:** the stored column (G1) and the single free surface (G3). G1's cost is currently paid as three parallel ad-hoc fields, two declared scaffoldings, one knob, and a read-time-detail shape the ladder law forbids elsewhere. G3's is paid as four thresholds and an undeclared datum. Neither is a new subsystem; both are consolidations that reduce the surface they replace.

**Build next, on leverage rather than on obligation:** G5 and G8 at the slab rung. The input exists and is unconsumed; the outputs unlock biomes, weathering, evaporation-as-physics, wave energy, and the retirement of the residence-time knob. I am aware this sits outside the current rank order in ` #disc-known-active-hotspots`, whose rank 1 is the mixed-tier reader — and I think that ordering is right and this does not displace it. Rank 1 is a *truth* repair; this is a *reach* expansion, and the repair should land first for the reason that segment gives.

**Where I would push back on the brief's framing:** the phrase *"phenomena we'll need to represent at some point"* invites a list, and a list is the artifact this project has repeatedly found to go stale. The durable form of the answer is §9.

## 8. What the fidelity-ladder law forbids here

` #form-fidelity-ladder`'s governing clause — each band of detail enters at a rung that runs, because structure introduced at a level no rung ran participates in no action and carries no key — constrains this survey more than any other consideration, and it constrains it *twice*.

**On the vertical axis it convicts the present arrangement, not only a future one.** §2a is a read-time detail increment: strata are synthesized from a scalar height at query time, and nothing that runs can see them. By the clause's own logic (*no action* and *no key* are the same observation, because a complete key is a value's causal provenance) the strata in a rendered column have no cause inside the world. The clause was stated for the frequency axis and for the viewer; the vertical case is the same defect in the same shape, and it is currently the default path for every column in the world.

**On the multiplication risk the brief raises, the clause gives a clean answer.** If a stratified column becomes the primitive, every field it holds — temperature, saturation, solute concentration, grain size, ice fraction — can acquire detail at read time, and the clause applies to each. The answer is not to hold back the primitive; it is that **each field enters at the rung its own process runs at, and G2 is why that is affordable**: a field whose process runs as a surface flux enters as *one layer with a physically-set thickness*, and refines only when a rung runs finer. Under that discipline the strata primitive does not multiply the read-time-detail surface — it is what lets each band be attributed, because a stratum a process wrote is a stratum with a key.

The corollary I would hold anyone (including this document) to: a stored column with a *declared* layer count is scaffolding, and under FE(6) it needs a stated demolition condition at birth. The layer count is a statement about compute, not about the world.

## 9. The mechanism this wants to become

The durable form of this survey is not a list. Three moves, in increasing cost.

**(a) Give `declare-causally-materialize-lazily` a reachability field.** The mechanism already declares the cause; it should also declare *what stays unreachable until materialization*. For the moon that is one line naming the intertidal band and the mixing term. This is the smallest change that makes every future deferral self-documenting, and it is a dictionary edit plus an ordinum field rather than a new artifact.

**(b) Read the gloss set as the deferability ledger it already is.** Measured: the ordinum carries **32 promises, of which 6 have a `:kept-by`** — so 26 are what the reading rules call glosses, *"an honest word-of-honor not yet mechanized… greppable, auditable for epistemic honesty, never mistaken for a guarantee."* Seven phases declare a `|record`, and no record is keepable without G1. The ordinum therefore already contains the deferral ledger this spike was asked to produce; what it lacks is the *cost* column, which §4 is a first pass at. One concrete discrepancy found while counting: `promise[closed-water-cycle]` has a `|predicate` (global water mass conserved) and no `:kept-by`, while `hydrosphere.rs` carries a passing test for exactly that predicate and is a registered nomos. Either the row wants a `:kept-by hydrosphere` or the promise means more than the test — worth a look, and small.

**(c) Name a future flux quantity iff a live nomos already assumes it silently.** Applying `flux.rs`'s own `PRECIPITATION` criterion, and against its stated refusal to pre-populate, three candidates pass and the rest do not:

- **evaporation** — `climate.rs` asserts precipitation equals evaporation in steady state, so an evaporation flux with no producer is already load-bearing in a live kernel.
- **the sea-level datum** — consumed by water, erosion's classification, and the readers; currently an *assumption* string rather than a quantity, which is the mechanism for a silent coupling edge (§2c). This is the strongest of the three: naming it is what makes rank 1b's four-thresholds problem an audit finding instead of a discovery.
- **column porosity** (weakest, and probably a no) — `gw_capacity` stands in for it, so something *is* assumed; but `material.rs` already holds `porosity`, so this is a property waiting to be read rather than a coupling edge waiting to be named. It belongs to ` #form-material-property-interface`, not to the vocabulary.

Nothing else on the G-list passes: no live nomos silently assumes a wind field, a surface temperature, a solute concentration or a grain-size distribution. They are absent, not assumed, and absence is honest. Under the file's discipline they stay out of the vocabulary until a nomos speaks them — which is the correct outcome and worth saying plainly, since the tempting move after a survey like this is to write the whole future alphabet down.

## 10. Intelligence-bearing phenomena

Held as an open region, not adjudicated, per the correction I received mid-survey and per `ETHICS.md` §Scope's own words.

Nothing in the *physical* phenomena inventory reaches the moratorium. The moratorium turns on **governance × substrate** — an endogenous mind, living inside the vivium's information-loss boundary, on a frontier or emergence-capable substrate — and `ETHICS.md`'s "Guardrail against overreach" plus Appendix B.1's per-cell table make clear that the axis is governance rather than estate or biological complexity. So biological complexity, ecology, food webs, speciation, and modelled civilizations at game-like sophistication are ordinary phenomena for a survey like this, and the ordinum already runs its phase ladder through them: `charge[complex-agency]`, `charge[language]`, `charge[culture]`.

Two things belong in the record rather than in a verdict.

The ordinum has already drawn its own line, and it is upstream of anything a phenomena survey would decide: Phase 8 carries `:binds ETHICS.md` and an `|obligation` block at severity `load-bearing-must-survive-every-revision`, stating that obligation begins at the transition where *the world has People*. That is the boundary, it is already in the data, and no phenomenon-level judgment should be read as relocating it.

And `ETHICS.md` §Scope leaves deliberately open whether the exogenous-only ceiling on primary work also excludes simple non-emergence-capable endo minds. Laid out as strata, one observation offers itself without adjudicating: the phenomena in §4 are gated on *objects*, and the gates get sharper as the object gets more specific. The endo-mind question has no such gate — its distinctions are governance, honesty, and retained estate, none of which is a world object at all. That is a reason to expect the boundary to stay where `ETHICS.md` puts it (in the agent seam, under Level C) rather than to fall out of the world-system layer, and it is an argument for not letting a phenomena roadmap be the place it gets settled.

## 11. Feedback on the brief

**What landed.** Naming ` #detail-phenomena-systems-map` and saying plainly that a fresh list would duplicate it saved the whole survey from being the wrong artifact. Handing me the fidelity-ladder clause as *likely to constrain the answer more than anything else* was accurate, and it turned out to convict the present arrangement rather than only a future one — I would not have looked at the vertical axis through it without the pointer.

**Where the frame nearly cost something.** The brief's hypothesis — that ` #form-column-control-volume` claims water lives in the column while the code treats it as a derived render-time field — is close to right but off in a way that matters. Water is *not* the read-time field; water is real state in `WaterSim`. **The strata are the read-time field.** The segment's claim about water holds better than the brief credits, and the defect is one layer down and much broader than water. Holding the hypothesis loosely, as invited, is what let that surface.

**The correction was right to arrive.** The original moratorium sentence would have narrowed §10 to a paragraph of omissions. It arrived before I had written anything, and I had already read `ETHICS.md` in full including the guardrail — so the wall had not been acted on. Worth knowing for calibration: the sentence *did* shape my reading order (I read `ETHICS.md` early and closely because of it), so it was not inert even though it changed no output.

**One structural note.** The brief's list of things-to-read was accurate and the most useful things I found were on it — but the single highest-value artifact was one nobody named: `crates/vivarium-world/src/flux.rs`, whose eleven-quantity vocabulary and stated scope discipline reframed the deliverable from a document into a query. If a future brief on this question exists, that file is the entry point.

## 12. What I did not verify

- **The gate decomposition of coastal erosion (§6) is unmeasured.** No probe here; it rests on reading `material.rs`'s property set and the shoreline's derivability from the mask.
- **No probe was run and no number was minted.** Every quantity quoted is attributed to a segment or to code, and inherits its era caveat. In particular I did not re-check whether any drainage-derived statistic I quote crossed the 2026-07-29 ocean-connectivity boundary; I quoted only figures the segments themselves present as current.
- **The 26-of-32 gloss count is mechanical** (`grep` over `|promise[` for `:kept-by`), so it counts syntax rather than adjudicating whether each keeper genuinely keeps its promise. `promise[emerged-land]` is the known case where the ordinum itself says Claimed rather than Kept.
- **`msc/research-lem-sota/lake-and-settle-sota-2026-07-29.md` is partially read** — headings and §0. It plausibly bears on G2 and G3 (its §2.2 on steady-state equivalence between the local-inertial scheme and a stationary solve sounds directly relevant to whether a slab/penetration-depth rung is defensible), and I did not use it. Anyone extending this should read it before making a prior-art claim about the water column.
- **I did not check the `crates/vivarium-explore/` side at all**, so nothing here says what the instrument can or cannot show of a stratified column.
- **The ordinum's reportatio was not read** (`.super-archive/from-archive/PHASES.md`, pin `42621d5`). ` #disc-open-problem-census` Working Notes say it carries decided-by-Joseph phase doctrine with no claim homes yet, including `declare-causally-materialize-lazily` and the corrected moon service list. §5 and §9(a) build on that mechanism from the LEXICON entry and the ordinum comment only, so the reportatio may already say more about it than I have credited.
