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
| **Erosion / sediment** (fluvial + hillslope) | Kyr–Myr | local–regional | **very high** (valleys, canyons, deltas) | med–hard | stream-power + hillslope diffusion | **done ✓** |
| **Karst / dissolution** (caves) | 10 Kyr–Myr | local | med (caves, gameplay) | med–hard | dissolution in carbonate + water table → cave networks, sinkholes | — |
| **Hydrothermal / ore genesis** | Kyr–Myr | local | med (mining) | med | deposition where fluid + heat + host-rock meet → veins, deposits | — |
| **Aeolian** (wind, dunes) | yr–Kyr | regional | med (deserts, dunes) | med | wind field deposition/erosion of loose material → dunes, loess | — |
| **Coastal / sea-level** | erosion decades; eustatic Kyr | local–regional | med (beaches, cliffs, deltas) | med | wave erosion + sea level | partial (flat sea) |
| **Hydrology** (surface + groundwater) | flow days–yr; drainage Kyr | watershed (10s–1000s km) | **high** (rivers, lakes) | hard (full hydrodynamics + groundwater) | flow accumulation + shallow-water | **done ✓** |
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
