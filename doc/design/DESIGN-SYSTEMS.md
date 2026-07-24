# vivarium — the world-phenomena graph (systems · timescales · size-scales)

*The concrete aspect-list for the multirate coupling (`.super-archive/from-design/DESIGN-REDUX.md` §4) and a build-order guide. Each system is a tier in the lazy query graph (§11); the ones that must run share the property substrate of `doc/design/DESIGN-MATERIAL.md` §5.*

> **Status: source / inventory, not claim canon.** Multirate coupling, fidelity
> ladder, and column/material substrate have claim homes when segmented
> (`#form-fidelity-invariant`, `#form-column-control-volume`, flux/ordinum
> segments). This file is the phenomena map + build-order judgment + approach
> ledger — extract candidates case-by-case; do not quote status columns as law.

*Epistemic notes: **timescales and size-scales are established** (standard Earth-system values, high confidence); **crude-payoff and high-fidelity-difficulty are judgment** (medium confidence — worth a research-hardening pass if we want rigor).
"vivarium" column is current status.*

## The systems

Ordered slow → fast (the natural coupling order: slow drivers set boundary conditions for fast responders).

| system | timescale | size-scale | crude payoff | hi-fid difficulty | crude version → produces | vivarium |
|---|---|---|---|---|---|---|
| **Tectonics** (uplift, plates) | Myr–100 Myr | global / continental | **high** (sets macro relief) | research (mantle convection) | uplift-rate field → the macro elevation prior | crude ✓ (geology tier) |
| **Climate / atmosphere** | weather hrs–days; climate decades–Myr (orbital) | global | **high** (drives biomes, hydrology, weathering) | research (GCM) | lat + elevation + continentality → temp/precip fields | insolation seed ✓ |
| **Volcanism / magmatism** | eruptions days–yr; provinces Myr | local–regional (10s–100s km) | med (features, ore, basalt) | hard | edifices + intrusions per hotspot/rift field | — |
| **Glaciation / ice sheets** | 10–100 Kyr (Milankovitch) | continental | med–high (distinctive terrain) | hard | ice where temp<thr + simple flow → U-valleys, cirques, fjords, moraines | — |
| **Weathering + pedogenesis** (soil) | 100s–10 Kyr | **local (per-column)** | **high** (soil for agents/farming; differential erosion) | hard (soil chemistry) | rate f(climate, rock, slope, time) → soil depth/type + differential hardness | — |
| **Erosion / sediment** (fluvial + hillslope) | Kyr–Myr | local–regional | **very high** (valleys, canyons, deltas) | med–hard | stream-power + hillslope diffusion | **crude rung ✓** |
| **Karst / dissolution** (caves) | 10 Kyr–Myr | local | med (caves, gameplay) | med–hard | dissolution in carbonate + water table → cave networks, sinkholes | — |
| **Hydrothermal / ore genesis** | Kyr–Myr | local | med (mining) | med | deposition where fluid + heat + host-rock meet → veins, deposits | — |
| **Aeolian** (wind, dunes) | yr–Kyr | regional | med (deserts, dunes) | med | wind field deposition/erosion of loose material → dunes, loess | — |
| **Coastal / sea-level** | erosion decades; eustatic Kyr | local–regional | med (beaches, cliffs, deltas) | med | wave erosion + sea level | partial (flat sea) |
| **Hydrology** (surface + groundwater) | flow days–yr; drainage Kyr | watershed (10s–1000s km) | **high** (rivers, lakes) | hard (full hydrodynamics + groundwater) | flow accumulation + shallow-water | **crude rung ✓** |
| **Biomes** (high-level ecosystems) | shift centuries–millennia | regional–continental | **high** (biosphere character) | easy–med | Whittaker (temp × precip) → biome lookup | — |
| **Ecosystems / vegetation** (succession) | succession yr–centuries | local–regional | **high** (visible flora, agent food) | med | biome → plant community + simple succession | — |
| **Biogeochemical cycles** (C, N, P, O) | biotic days–yr; geologic Myr | local–global | med (fertility, productivity) | hard | nutrient pools per soil/biome | partial (soil params) |
| **Seismicity / faulting** | events (s) on a 100s–1000s yr stress cycle | regional | low–med (scarps, events) | med | fault lines from tectonic stress → scarps, offsets | — |
| **Floral/faunal evolution** (speciation) | 10 Kyr–Myr | regional–global | low–med (flavour) | research | trait drift per biome | defer |
| **Isostasy / rebound** | Kyr–10 Kyr | regional | low–med | med | elastic response to (ice/sediment) load | defer |
| **Ocean circulation** | yr–Kyr (thermohaline ~1000 yr) | global | low (little local-visible) | hard | heat-transport bands → coastal climate | defer |

## Prior-art anchor — the Cordonnier/Braun line *(Galin et al. 2019, "A Review of Digital Terrain Modeling"; ingested 2026-07-10)*

Vivarium's erosion tier **is** the method the CG-terrain field converged on. **Cordonnier–Braun (CBC\*16)** brought geomorphology's **stream-power law** $\partial h/\partial t = -k\,A^m s^n + u$ (drainage area $A$ approximates the upstream-integrated precipitation flux; slope $s$; uplift $u$) into graphics and **solved it implicitly in linear time** over 100 km at ~100 m — exactly our implicit stream-power ($n{=}1$) tier. The **next two rungs are already published:** (a) **CCB\*18** (*Sculpting Mountains*) adds **per-stratum erodibility** — erosion honoring distinct bedrock strata → a layered $2\tfrac{1}{2}$D model with folds and faults, precisely our queued *per-material erodibility → Bryce hoodoos* rung over the strata/column model; and its **geologically-coherent uplift** (crust as incompressible viscous material, folds from layered sheets) is the principled replacement for our fBm-uplift stand-in (the `tectonic uplift` row's "→ P (mantle)"). (b) The review's verdict on our LOD problem is the positioning statement worth keeping: *"the simultaneous need for large extent and high precision has yet to be adequately addressed,"* and the route it proposes — *"multi-resolution methods operating at varying time and space scales, combined with a careful analysis of the cause-effect relationships in geomorphology"* — **is vivarium's multiscale-seam architecture, named by the field as the open direction and not yet built** (`.super-archive/from-theory/multiscale-seams.md`). Two further confirmations: the standard erosion representation is our exact **layered stack** (bedrock + granular thicknesses — BF01, GGP\*15, = `doc/design/DESIGN-MATERIAL.md` strata); and forward-Euler **thermal/diffusion is a named large-step wall** (*"physically accurate only for small time steps … prevents its use when simulating large scale features"*) — the field's statement of our $z{=}2$ creep clamp (`.super-archive/from-theory/multiscale-seams.md` §3). Terrain **amplification** (coarse→fine, up to 256×, via sparse construction trees preserving drainage coherence — GDGP16/AAC\*17) is prior art for our lift/materialize $L$ and its basin-partition-stability requirement.

## Coupling structure (multirate, §4)

Four rough bands, each on its own timestep; **fast bands see slow ones as quasi-static; slow bands see fast ones as time-averaged forcings** (the separate-the-timescales principle, already the load-bearing hydrology lesson):

- **Deep drivers** (Myr): tectonics, mantle heat → *fluxes* uplift-rate, geothermal gradient, base rock type into everything below.
- **Orbital / climate** (10–100 Kyr): Milankovitch → climate; glaciation →
  *fluxes* temperature, precipitation, ice load.
- **Surface process** (Kyr): erosion, weathering, karst, aeolian, coastal →
  *fluxes* elevation change, sediment, soil/regolith.
- **Fast / biological** (yr–centuries): hydrology routing, ecosystems, biogeochem
  → *fluxes* water depth, vegetation cover, nutrients.

## Build order (most fidelity per unit effort, crude rung)

1. **(done)** erosion + hydrology — the biggest visual payoff, already in.
2. **Climate (crude)** — temp/precip from latitude/elevation/continentality; the insolation tier is the seed. Unlocks *everything* downstream. Cheap.
3. **Biomes** — a Whittaker lookup from climate. Trivial code, whole-biosphere payoff.
4. **Weathering / pedogenesis** — soil (rich/poor/rocky) + differential rock hardness. Cheap statistical; unlocks agents/farming *and* Bryce-style relief.
5. **Vegetation / ecosystems** — biome → community. Visible flora, agent food.
6. Then feature-specific, medium cost: glaciation, volcanism, karst, aeolian,
   coastal, hydrothermal (ore).
7. **Defer:** full GCM, ocean circulation, speciation, full biogeochemistry,
   isostasy.

## Feature-driven growth ("want X? ensure these materials + run these systems")

The curiosity path: pick a real-world landform, and it decomposes into *materials present in quantities* + *systems run*.

- **Bryce Canyon (hoodoos)** = differential erosion of layered sedimentary →
  *sedimentary strata with alternating hard/soft erodibility* + erosion honoring per-stratum erodibility. (Exercises strata + `erodibility` directly.)
- **Fjords / U-valleys** = glaciation → ice + glacial carving.
- **Caves** = karst → carbonate rock + dissolution + water table.
- **Deserts / dunes** = aeolian → arid biome (climate) + loose sand + wind field.
- **Gold veins / ore** = hydrothermal → host rock + fluid/heat + the ore flag
  (and the stateless coherent-noise-under-macro materialization, `DESIGN-MATERIAL` §10).
- **Deltas** = erosion + coastal → sediment transport + sea level.

Each is a small composition over the same substrate — which is the whole point of the stable property interface (`DESIGN-MATERIAL` §6): features are *compositions*, not new engines.

## Fluvial ladder — named next rungs (Joseph, 2026-07-03; parentheses = processes rather than attributes)

Within the erosion/hydrology rows above — physics we are knowingly not yet tracking, listed so the ladder is honest:

- **Armoring** — winnowing of fines leaves a coarse surface lag that caps incision; interface state (`DESIGN-MATERIAL` §5), the persistent partner of the flow-seal.
- **Colmation as part of the column** — live in the sim (2026-07-02); needs its Column interface-state home (`DESIGN-MATERIAL` §5).
- **Aggradation & transient debris flows** — bed *rise* when supply exceeds capacity (we deposit, but no supply-driven regime shift); debris flows are the high-concentration slurry regime — ties to the $\mu(I)$/Paste rung
  (`DESIGN-MATERIAL` §6).
- **(Traction)** — bedload as a transport *mode* distinct from suspension
  (rolls/slides along the bed, different capacity law, builds bars).
- **(Better, finer-tuned bank erosion)** — lateral erosion from cross-channel shear, distinct from bed incision.
- **(→ meandering, oxbows, cutoffs)** — river/stream *evolution* on flatter terrain; emerges from bank erosion + point-bar deposition, not from a meander model.

## Sediment & fluvial phenomena — the full inventory (2026-07-03)

Joseph asked for the complete map ("do we have a feel for ALL the phenomena?").
Status: **✓** crude rung in; **~** partial/emergent; **—** absent. The single suspended pool + alluvium is today's whole grain model — most gaps below resolve into "grain sizes exist" (§15 materials) plus one transport mode.

**Transport modes** (by grain behaviour):
- ✓ wash load / suspension (shear-gated settling, eddy diffusion)
- — saltation (sand, hopping — between suspension and traction)
- — bed-load TRACTION (rolling/sliding: slower than the flow, thalweg-bound;
  builds bars; Joseph's list) — the biggest structural gap: one suspended pool currently stands in for all three modes
- — hyperconcentrated / debris flow ($\mu$(I) regime, §15 rung 3)
- — Stokes settling per grain size (we cap rates instead — fines vs sand settle identically today, which is wrong by ~100×)
- — SOLUTION (dissolved load — moves invisibly, precipitates elsewhere; the transport-mode face of karst/corrosion; also evaporites)
- — FLOCCULATION (clay clumps in brackish water → settles fast: why deltas drop their finest load AT the salt boundary — matters at river mouths)

**Bed / interface:**
- ✓ armoring + winnowing (probe-gated) · ✓ colmation (fines-only)
- — bedforms (ripples/dunes/antidunes) and their ROUGHNESS feedback
- — imbrication/packing ($\varphi_{\text{pack}}$ exists in the schema, unused)
- — bed-load PIPING (subsurface flow drags fines through gravel pores —
  internal restructuring; also the levee/dam internal-failure mechanism)
- ~ pool-riffle / step-pool (emergent at 4.8 m; watch, don't code)
- ~ knickpoint migration (emergent in the erosion tier; unverified)

**Erosional mechanism detail** (our shear-threshold bulk erosion bundles these; worth unbundling when grain sizes exist):
- ~ hydraulic action (= our $\tau \gt \tau_c$ detachment)
- — abrasion TOOLS-AND-COVER effect: erosion needs grinding tools (no load ⇒
  little wear even at high shear) but too much load shields the bed — a non-monotonic dependence our K-only stream power misses (armor/alluvium shield is the cover half; the tools half is absent)
- — attrition → DOWNSTREAM FINING (grains round and shrink in transit: why real rivers grade gravel→sand→silt downstream; needs grain sizes)
- — cavitation (waterfall plunge-pools, extreme-velocity shock — far rung)
- ✓ corrosion/solution as karst (system row); dissolved-load mode above

**Banks / planform:**
- ~ lateral shear erosion (emergent side-carving observed 2026-07-03)
- — bank failure by undercut + cohesive collapse (§15 Mohr–Coulomb rung)
- — meandering (helical secondary flow → point bars → cutoffs → oxbows)
- ~ avulsion (channel-jumping on aggrading fans — the braided delta Joseph photographed suggests it may partially emerge; verify before claiming)
- ~ BRAIDING (named: high load + shallow shifting channels — observed in Joseph's delta shot; same verify-before-claiming caveat)
- ~ headward erosion (channels lengthening upstream — should emerge from the erosion tier's drainage capture; unverified)
- — levees + SPLAY formation (overbank deposition builds levees; breaches drop coarse fans on the floodplain — becomes possible once overbank deposition differentiates by distance-from-channel)

**Hillslope ↔ water:**
- ✓ soil creep (erosion tier) · ~ sheet/rill erosion (storm shear)
- — rain-splash detachment
- — saturation slumping / landslides (Joseph's #10; §15 rung 2, planned)
- — hillslope-initiated debris flows

**Column / still water:**
- ✓ infiltration + colmation seal + baseflow · ✓ eddy mixing
- — Darcy LATERAL groundwater flow (springs at low points need it)
- — turbidity currents (underwater density flows — an 11 m suspended-mud column over a slope SHOULD slide downslope as a bottom current; relevant to the deluge-legacy mud lakes, and how real lakes build turbidites)
- — deposit consolidation / mud cohesion aging
- — closed-basin evaporation → salt flats (ties to the analytic-fill lakes)

**Interfaces to other systems** (rows above): aeolian pickup of dry fines,
coastal/wave transport, karst dissolution, ice/freeze — their own systems.

**Fundamental-vs-emergent discipline** (from Joseph's survey pass, 2026-07-03):
the fundamentals are few — fluid-particle mechanics (Shields/settling/drag),
boundary shear vs cohesion, tools-abrasion, geotechnical failure, chemistry —
and everything else on this page should EMERGE. Corollary worth stating as a rule: **some explicit state is scaffolding with a demolition date.** Our armor field parameterizes what selective transport of real grain sizes would emerge
(D10 mobile, D84 not); when the grain-size split lands, armor should re-emerge and the explicit field retire. Same eventual fate for colmation (fines percolation) and possibly alluvium (it IS the coarse transport pool at rest).

**Hyperconcentrated / debris regime — actively missing physics** (survey 2):
our momentum treats a 95% mud column like clear water. Real mixtures have
(a) concentration-dependent density & viscosity — "flow bulking", the feedback that turns flash floods into debris waves — and (b) a BINGHAM YIELD STRESS: past ~50% solids the mixture is solid until stress exceeds yield,
then a scouring slurry. Two small terms ($\rho$(conc), $\tau_{\text{yield}}$(conc)) bridge the water sim to §15's $\mu(I)$ rung; debris JAMS additionally need woody-debris /
boulder BODIES (the §14 overlay), and jam-forced scour + self-clogging avulsion then emerge. Named candidates, not yet built.

**Highest-leverage next three, by visible-truth-per-effort:** (1) grain-size split (fines/sand/gravel) with Stokes settling — unlocks saltation/traction as behaviours instead of modes, real sorting, honest deltas; (2) bank mechanics (undercut + Mohr–Coulomb) — unlocks meandering/oxbows, Joseph's long-standing wish; (3) Darcy groundwater — springs, honest baseflow geography.

## Instruments (regime probes — see DESIGN-REDUX §2b)

Every system rung ships with renderer-free probes asserting invariants nature guarantees in a known regime; known issues get their probe written FIRST
(domain TDD). Current: `topo` (prior slope statistics), `spike_probe` (erosion spire instability), `channel_profile` (two-regime water: subcritical must be smooth), **`armor_regimes`** (three-regime scour/armor gate: 1/3 green —
fines sorting robust; armor-vs-eddy interaction and source-cell zero-incision anomaly are the two named opens, status in the probe header), and
**`seam_ridge`** — the differential-aging ridge probe (built
2026-07-03, first probe authored UNDER this methodology): cross-seam curvature where a fine tier's age exceeds its surroundings'. **Currently red, as expected**: seam/interior curvature ratio 4.3× at the standard 18-epoch fine pass, growing with the age gap (5.3× @ 60e, 7.1× @ 150e) — the mean-pin conserves block means but not boundary gradients. It now gates the future seam fix; Joseph's ridge sighting is measured, not anecdotal.

---

*The ledger below was relocated here 2026-07-11 from `.super-archive/from-archive/PHASES.md` (now archived as the Terrestris ordinum's reportatio). It is DESIGN-SYSTEMS-native — how each process wants to be modelled — and the `nomotheke` references its approach codes. The per-process status columns are a first-pass snapshot; the live per-nomos status is the nomotheke's declarations.*

## Algorithms & approaches ledger *(first pass — accrete as we hit them in practice)*

*A place to park what we learn about HOW each process wants to be modelled, the
day we hit it (today: scouring/armoring). Not a plan — a growing map.*

**Approach codes** (a process may blend them): **A** analytic / closed-form /
functional (+ fated-noise seeded stochasticity) · **R** relaxation-to-attractor (settle,
then memo-cache) · **S** statistical / empirical (fit, lookup, sub-grid
parameterization) · **T** taxonomy / bestiary import (curated tables — materials,
biomes, species) · **P** procedural-physical stepwise sim *(annotate physics-
understood hi/med/lo + cost)*.

**Reading the "agent-fidelity" column — the prioritization backbone.** It asks:
does an inhabitant of the final three (agent) phases *perceive, depend on, or act
through* this? That is why we may run tectonics crude forever but must get rivers
and biomes right — an agent fords a river and forages a biome; none has ever
touched mantle convection. Work backward from what agents touch.

### Geology & landscape (Phase 4, mostly)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| terrain prior | A (2-band fBm) | proxy | O(n·oct) | low (sets stage) | built |
| tectonic uplift | A stand-in → P (mantle) | lo | — | low-direct | crude (fBm) |
| stream-power incision (Shields τ_c) | P | hi | O(n)/epoch | **high** (the land) | built |
| Priority-Flood depressions | P/algo | hi | O(n log n) | high | built |
| MFD drainage | P | hi | O(n) | high | built |
| hillslope creep (diffusion) | P | hi | O(n) | med | built |
| glaciation · volcanism · karst · aeolian · coastal | P/S | med | varies | med (landforms) | — |

### Water & sediment (Phase 4 — today's earned detail)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| shallow-water flow (local-inertial pipes) | P | hi | O(n)/step, CFL | **high** (fording, mills, drink) | built + stabilized |
| suspension + shear-gated settling (Rouse) | P | med | O(n) | med (clarity) | built crude |
| turbulent eddy diffusion | P | hi | O(n) | med | built today |
| armoring + winnowing | P | med | O(n) | med (stony beds) | built, probe-partial |
| colmation (fines seal) | P | med | O(n) | low-med | built |
| **grain-size split + Stokes** (→ saltation, traction, sorting) | P | hi | O(n·k) | med | **planned — top pick** |
| **bank mechanics** (Mohr–Coulomb + undercut → meander, oxbow) | P | hi (geotech) | O(n) | **high** (banks, fords, slumps) | planned |
| Darcy lateral groundwater (→ springs) | P | hi | O(n) | med | planned |
| hyperconcentrated / Bingham + flow-bulking | P | med | O(n) | med (flash-flood hazard) | named |
| analytic hydrological init | A + R | hi | O(n log n) | indirect (clean worlds) | planned |

### Climate & weather (Phase 3 base, Phase 5 modern) — the next system

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| insolation (declination/hour-angle) | A | hi | O(1)/query | med | built |
| climate (T/precip ← lat, elev, continentality) | S + A | med | O(n) | **high** (biomes, seasons) | **next** |
| storms / weather | A jitter → P | med | O(n) | **high** (agents feel it) | crude (jittered) |
| orographic rain, rain shadow | P/S | med | O(n) | high | planned |

### Materials & record (spans all phases)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| material taxonomy (undifferentiated → refined) | T + A (det. refine) | — | O(1) | **high** (what things are) | schema |
| strata / column | data structure | — | — | high (digging, cliffs) | frame built |
| chronostratigraphic Record = the verification artifact | (emerges) | — | — | high (readable history) | conceptual |

### Biosphere (Phases 3→6; final-3-critical) — mostly import + statistics

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| biomes (Whittaker T×P) | T + S lookup | — | O(1)/cell | **high** | — |
| vegetation / succession | S + T | med | O(n) | **high** (food, cover) | — |
| species / fauna | **T (bestiary import)** | — | curated | **high** | — |
| evolution / speciation | S / procedural | lo | — | low-med (flavour) | defer |

### Agents — the real bet (Phases 6→8)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| fast ASF/AAT layer (formal dynamics) | P | hi (our theory) | O(agents)/tick | **the point** | the bet |
| slow layer (LLM at aporia) | import (the model) | — | event-driven | **the point** | the bet |
| cognitive LOD swap | A (component swap) | — | ~free | high | designed |

*Note on complexity: the per-cell O(n) stencils are the cheap common case —
memory-bound, and the GPU/rayon path (`doc/plan/water-parallelism.md`) is
what keeps them affordable as fidelity and area grow. The expensive-but-rare
ones (Priority-Flood O(n log n), any global solve) run once per phase-transition
and cache. The genuinely open-complexity items are the ones marked lo physics
(tectonics, evolution) — where we deliberately run a cheap statistical stand-in
because agent-fidelity does not yet demand more.*
