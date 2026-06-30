# Hydrology — worldgen water, erosion coupling, and the snapshot

*Authored 2026-06-29 (the hydrology session). The code is `crates/vivarium-core/src/hydro.rs`
(the fluid + groundwater + sediment), with the worldgen wiring in `voxel.rs`
(`Volume::eroded`) and the geological erosion in `geo.rs`. This file is the **why**;
the code is the **what**.*

## The bet

Worldgen produces a walkable landscape with **streams running down valleys into
lakes, dry hillsides, springs, and a flat sea** — all *earned by physics* (erosion
carves, water flows and levels), not painted with fractal noise or imposed by
hand. The frozen result is static ("the way it is by the time you walk in"), but
every feature emerged from a simulation that conserves mass.

## The single most important lesson: **separate the timescales**

Erosion and water are both functions of time, but they live ~10¹⁰ apart: water
finds its level in **hours–days**; landscapes are reshaped over **millennia**.
The first version of this system coupled erosion *into* the water sim on one
timestep and then cranked the erosion rates to carve in a few sim-minutes — which
made the numbers mutually incoherent (5 "minutes" of rain carving canyons; water
spikes that should dissipate in seconds but didn't). **There was no physical time
at all** — every rate was tuned to make a picture in N iterations.

The fix is the standard separation-of-timescales: the *fast* variable (water) is
slaved to the *slow* one (erosion). So worldgen runs **three phases in sequence**,
each on its own honest clock:

1. **Macro erosion** (`geo`, 16 m grid) — the large landform. Geological time;
   water is the steady-state drainage (discharge ∝ upslope area, à la FastScape),
   never time-stepped.
2. **Fine erosion** (`geo`, `SIM_CELL_M` = 4 m, `FINE_EPOCHS`, uplift off) — incises
   the real sub-16 m channels into the upsampled macro field. *This is where the
   carving lives.* Also where the channel network (drainage) is established.
3. **Water** (`hydro`, same 4 m grid, **sediment OFF**) — pure shallow-water +
   groundwater run to hydrological steady state on the **fixed** bed (lakes level,
   streams settle), then **freeze the snapshot**. Carves nothing, so nothing is
   time-conflated.

If you are tempted to put carving back into the water phase: don't. The carving
is geological; the water phase is hours–days and carves nothing real in that time.

## The water model (`hydro::WaterSim`)

**Virtual-pipes shallow water** (Mei, Decaudin & Hu 2007): water depth per cell,
joined to 4 axial neighbours by "pipes" whose flux accelerates under the
water-surface (`bed + depth`) height difference — that acceleration is the
momentum. Stable because flux is non-negative and clamped to the water actually
present, so depth never goes negative and volume is conserved to round-off. A flat
lake **emerges** (gravity dissipates the head difference until the surface is
flat) — it is never imposed by a "fill". The proof is `basin_fills_to_a_flat_lake`.

The water phase is a **conserved cycle of reservoirs** — nothing is created or
destroyed, every "source/sink" is a transfer:

- **atmosphere** (scalar store) → precipitation → **surface**
- **surface** → infiltration → **groundwater**
- **groundwater** → flows *laterally* downhill (Darcy, `head = bed + gw`) → and
  **exfiltrates** where it saturates = **springs** (this is what makes dry-slope /
  wet-valley distribution EMERGE — water soaks the slopes, travels underground,
  surfaces in the valleys)
- **surface** → evaporation → **atmosphere**; **edge/sea** ⇄ **ocean** store

`total_water()` sums all four reservoirs and the test `the_water_cycle_conserves_total_water`
asserts it can't move. If that test ever goes red, a leak was introduced.

**Material coupling (`geo::Strata`).** Hardness is a deterministic 3D field
(strata bands + intrusions). It drives erosion (soft yields, hard resists →
waterfalls, hard-sill lakes) *and* groundwater: permeability and porosity =
`1/hardness`, so groundwater backs up at soft→hard contacts and **springs emerge
at lithologic boundaries**. The `gw_material` field is cached (`mat_cache`) because
the bed is fixed in the water phase — recomputing a hardness sample per cell per
step was the dominant cost.

**Sediment (erosion phase only).** Dimensionally-honest velocity-capacity
transport: equilibrium suspended height `C = capacity·(slope·speed/settling)·depth`
(a concentration × depth), with erosion/deposition relaxing the load toward `C` at
real per-second rates, conservative flux-advection, a Froude speed cap for
stability, and **talus** (angle-of-repose slumping) to stop deposition spikes.
`total_solid()` (bed + suspended) is conserved.

**Channel sealing (baked in the erosion phase).** Streambeds armor/colmate over
geological time, so flowing channels stop leaking into the ground. Implemented as
a per-cell **infiltration field** baked from the erosion's drainage: channel beds
(high drainage) → near-zero infiltration; porous slopes → full rate
(`infil = base/(1 + drainage/~0.05 km²)`). Resolving this needs the channels
resolved — hence 4 m (at 8 m, thin headwater streams fell below the channel
threshold, stayed porous, leaked, and re-gushed downstream).

## The frozen snapshot

`Volume::eroded` freezes onto an `ErodedSurface` (a 4 m grid sampled bilinearly by
the 0.5 m render voxels):

- `h_m` — the carved bed. **This IS the terrain.** No fractal-noise overlay.
- `depth_m` — water depth = the continuous (incl. partial) volume per cell.
- `water_surf_m` — water-surface elevation. Sampled by a **wet-masked** bilinear
  (only wet cells contribute) so the lake level can't climb dry banks — without
  the mask, `bed + depth` on dry bank cells drags the rendered surface up every
  shoreline ("sticky" water).
- `vx_m, vy_m` — flow velocity (direction + speed). Frozen for the view (water
  shading) and future agents/currents; not yet used by physics post-freeze.

## What is principled vs. what is a known fudge (be honest about this)

Principled and tested: conserved water cycle; conserved velocity-capacity sediment
(dimensionally clean, per-second rates); 3D hardness driving erosion AND
groundwater; talus; lateral groundwater (springs emergent); channel sealing from
drainage; flat lakes emergent (not filled); the timescale separation.

Known fudges / placeholders, in order of how much they bother:

- **Rain rate is ~100–1000× real** (`precip_rate ≈ 0.006 m/s`). Tuned to fill the
  basins in ~40 min of physical water time instead of weeks. The *physics of how
  water moves* is time-honest; the *amount delivered per second* is not. The fix
  is real precip + priming the channels with steady-state discharge so they don't
  start from dry.
- **Groundwater `head = bed + gw`** is a simplification of the true water-table
  elevation; `gw_conductivity` is a per-cell relaxation rate, not a true Darcy
  hydraulic conductivity (m/s) — though it's now material-scaled.
- **No bank/lateral erosion** → no meanders or oxbows. That needs a separate
  channel-migration model (Ikeda–Parker–Sawai), not a tweak — the genuine frontier.
- **Uniform precip** — no orographic / rain-shadow spatial field yet.
- Absent entirely: vegetation, climate/snow/ice/glaciers, chemical weathering/karst,
  sediment grain-size sorting, aeolian.

## Parameters (units matter — they're a forcing function for principled physics)

In `WaterParams` (see the field docs). The ones with non-obvious dimensions:
`capacity` is **seconds** (not dimensionless); `erode`/`deposit` are **1/s** (real
per-second rates); `gw_conductivity`, `baseflow`, `exfil_rate`, `evaporation` are
**1/s**; `precip_rate`, `infiltration`, `settling` are **m/s**; `gw_capacity` is
**m**; `min_slope`, `repose` are dimensionless (rise/run). Getting the dimensions
to come out clean is how you know the physics is honest.

## Instruments (headless, dependency-free)

- `cargo run -p vivarium-core --example water_preview` — the fluid in isolation:
  plan view, a **side cross-section** (flat ≈-tops = real pools), volume-vs-step
  convergence trace. The first place to verify sim behaviour.
- `cargo run --release -p vivarium-core --example worldgen_time [region] [epochs]`
  — times a full worldgen and prints a cross-section + top-down water map of the
  *actual rendered voxels*. **Lesson learned:** a binary wet/dry map cannot tell a
  flat lake from a terraced sheet — judge the *surface*, not just "wet".

## Open / next (roughly prioritized)

1. **Renderer**: translucent water (ground visible through it) + wet-ground
   darkening (Joseph's standing request). View-side (`archive/godot-voxel/`).
2. **Real precip timing**: drop rain toward physical and prime channels with the
   steady-state discharge from the drainage network, so it's not "firehose for 40
   minutes".
3. **Sub-4 m streams**: the thinnest trickles still under-resolve; finer grid or a
   sub-grid channel model.
4. **Bank erosion → meanders/oxbows** (the frontier).
5. **Test cost**: the suite is ~3 min because every eroded-world test bakes a 4 m
   sim — add a fast small-grid mode or shrink test regions.
6. **Live water**: the same `step()` kernel can run per-frame in a loaded region
   (not just a worldgen snapshot) when the agent/LOD work needs dynamic water.
