# vivarium — the world-phenomena graph (systems · timescales · size-scales)

*The concrete aspect-list for the multirate coupling (`DESIGN-REDUX.md` §4) and a
build-order guide. Each system is a tier in the lazy query graph (§11); the ones
that must run share the property substrate of `DESIGN-MATERIAL.md` §5.*

*Status: **timescales and size-scales are established** (standard Earth-system
values, high confidence); **crude-payoff and high-fidelity-difficulty are my
judgment** (medium confidence — worth a research-hardening pass if we want rigor).
"vivarium" column is current status.*

## The systems

Ordered slow → fast (the natural coupling order: slow drivers set boundary
conditions for fast responders).

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

## Coupling structure (multirate, §4)

Four rough bands, each on its own timestep; **fast bands see slow ones as
quasi-static; slow bands see fast ones as time-averaged forcings** (the
separate-the-timescales principle, already the load-bearing hydrology lesson):

- **Deep drivers** (Myr): tectonics, mantle heat → *fluxes* uplift-rate, geothermal
  gradient, base rock type into everything below.
- **Orbital / climate** (10–100 Kyr): Milankovitch → climate; glaciation →
  *fluxes* temperature, precipitation, ice load.
- **Surface process** (Kyr): erosion, weathering, karst, aeolian, coastal →
  *fluxes* elevation change, sediment, soil/regolith.
- **Fast / biological** (yr–centuries): hydrology routing, ecosystems, biogeochem
  → *fluxes* water depth, vegetation cover, nutrients.

## Build order (most fidelity per unit effort, crude rung)

1. **(done)** erosion + hydrology — the biggest visual payoff, already in.
2. **Climate (crude)** — temp/precip from latitude/elevation/continentality; the
   insolation tier is the seed. Unlocks *everything* downstream. Cheap.
3. **Biomes** — a Whittaker lookup from climate. Trivial code, whole-biosphere
   payoff.
4. **Weathering / pedogenesis** — soil (rich/poor/rocky) + differential rock
   hardness. Cheap statistical; unlocks agents/farming *and* Bryce-style relief.
5. **Vegetation / ecosystems** — biome → community. Visible flora, agent food.
6. Then feature-specific, medium cost: glaciation, volcanism, karst, aeolian,
   coastal, hydrothermal (ore).
7. **Defer:** full GCM, ocean circulation, speciation, full biogeochemistry,
   isostasy.

## Feature-driven growth ("want X? ensure these materials + run these systems")

The curiosity path: pick a real-world landform, and it decomposes into *materials
present in quantities* + *systems run*.

- **Bryce Canyon (hoodoos)** = differential erosion of layered sedimentary →
  *sedimentary strata with alternating hard/soft erodibility* + erosion honoring
  per-stratum erodibility. (Exercises strata + `erodibility` directly.)
- **Fjords / U-valleys** = glaciation → ice + glacial carving.
- **Caves** = karst → carbonate rock + dissolution + water table.
- **Deserts / dunes** = aeolian → arid biome (climate) + loose sand + wind field.
- **Gold veins / ore** = hydrothermal → host rock + fluid/heat + the ore flag
  (and the stateless coherent-noise-under-macro materialization, `DESIGN-MATERIAL` §10).
- **Deltas** = erosion + coastal → sediment transport + sea level.

Each is a small recipe over the same substrate — which is the whole point of the
stable property interface (`DESIGN-MATERIAL` §6): features are *compositions*, not
new engines.

## Fluvial ladder — named next rungs (Joseph, 2026-07-03; parentheses = processes rather than attributes)

Within the erosion/hydrology rows above — physics we are knowingly not yet
tracking, listed so the ladder is honest:

- **Armoring** — winnowing of fines leaves a coarse surface lag that caps
  incision; interface state (`DESIGN-MATERIAL` §5), the persistent partner of
  the flow-seal.
- **Colmation as part of the column** — live in the sim (2026-07-02); needs its
  Column interface-state home (`DESIGN-MATERIAL` §5).
- **Aggradation & transient debris flows** — bed *rise* when supply exceeds
  capacity (we deposit, but no supply-driven regime shift); debris flows are the
  high-concentration slurry regime — ties to the μ(I)/Paste rung
  (`DESIGN-MATERIAL` §6).
- **(Traction)** — bedload as a transport *mode* distinct from suspension
  (rolls/slides along the bed, different capacity law, builds bars).
- **(Better, finer-tuned bank erosion)** — lateral erosion from cross-channel
  shear, distinct from bed incision.
- **(→ meandering, oxbows, cutoffs)** — river/stream *evolution* on flatter
  terrain; emerges from bank erosion + point-bar deposition, not from a
  meander model.

## Sediment & fluvial phenomena — the full inventory (2026-07-03)

Joseph asked for the complete map ("do we have a feel for ALL the phenomena?").
Status: **✓** crude rung in; **~** partial/emergent; **—** absent. The single
suspended pool + alluvium is today's whole grain model — most gaps below
resolve into "grain sizes exist" (§15 materials) plus one transport mode.

**Transport modes** (by grain behaviour):
- ✓ wash load / suspension (shear-gated settling, eddy diffusion)
- — saltation (sand, hopping — between suspension and traction)
- — bed-load TRACTION (rolling/sliding: slower than the flow, thalweg-bound;
  builds bars; Joseph's list) — the biggest structural gap: one suspended
  pool currently stands in for all three modes
- — hyperconcentrated / debris flow (μ(I) regime, §15 rung 3)
- — Stokes settling per grain size (we cap rates instead — fines vs sand
  settle identically today, which is wrong by ~100×)

**Bed / interface:**
- ✓ armoring + winnowing (probe-gated) · ✓ colmation (fines-only)
- — bedforms (ripples/dunes/antidunes) and their ROUGHNESS feedback
- — imbrication/packing (φ_pack exists in the schema, unused)
- ~ pool-riffle / step-pool (emergent at 4.8 m; watch, don't code)
- ~ knickpoint migration (emergent in the erosion tier; unverified)

**Banks / planform:**
- ~ lateral shear erosion (emergent side-carving observed 2026-07-03)
- — bank failure by undercut + cohesive collapse (§15 Mohr–Coulomb rung)
- — meandering (helical secondary flow → point bars → cutoffs → oxbows)
- ~ avulsion (channel-jumping on aggrading fans — the braided delta Joseph
  photographed suggests it may partially emerge; verify before claiming)

**Hillslope ↔ water:**
- ✓ soil creep (erosion tier) · ~ sheet/rill erosion (storm shear)
- — rain-splash detachment
- — saturation slumping / landslides (Joseph's #10; §15 rung 2, planned)
- — hillslope-initiated debris flows

**Column / still water:**
- ✓ infiltration + colmation seal + baseflow · ✓ eddy mixing
- — Darcy LATERAL groundwater flow (springs at low points need it)
- — turbidity currents (underwater density flows — an 11 m suspended-mud
  column over a slope SHOULD slide downslope as a bottom current; relevant
  to the deluge-legacy mud lakes, and how real lakes build turbidites)
- — deposit consolidation / mud cohesion aging
- — closed-basin evaporation → salt flats (ties to the analytic-fill lakes)

**Interfaces to other systems** (rows above): aeolian pickup of dry fines,
coastal/wave transport, karst dissolution, ice/freeze — their own systems.

**Highest-leverage next three, by visible-truth-per-effort:** (1) grain-size
split (fines/sand/gravel) with Stokes settling — unlocks saltation/traction
as behaviours instead of modes, real sorting, honest deltas; (2) bank
mechanics (undercut + Mohr–Coulomb) — unlocks meandering/oxbows, Joseph's
long-standing wish; (3) Darcy groundwater — springs, honest baseflow
geography.

## Instruments (regime probes — see DESIGN-REDUX §2b)

Every system rung ships with renderer-free probes asserting invariants nature
guarantees in a known regime; known issues get their probe written FIRST
(domain TDD). Current: `topo` (prior slope statistics), `spike_probe` (erosion
spire instability), `channel_profile` (two-regime water: subcritical must be
smooth), and **`seam_ridge`** — the differential-aging ridge probe (built
2026-07-03, first probe authored UNDER this methodology): cross-seam curvature
where a fine tier's age exceeds its surroundings'. **Currently red, as
expected**: seam/interior curvature ratio 4.3× at the standard 18-epoch fine
pass, growing with the age gap (5.3× @ 60e, 7.1× @ 150e) — the mean-pin
conserves block means but not boundary gradients. It now gates the future
seam fix; Joseph's ridge sighting is measured, not anecdotal.
