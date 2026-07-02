# vivarium — the matter data model (columns · strata · voxels · bodies · properties)

*Started 2026-07-01. This is the concrete **data-model** companion to
`DESIGN-REDUX.md` (which carries the general fidelity/LOD/runtime philosophy).
Where REDUX asks "how does representation follow consequence," this asks the
narrow, load-bearing question: **what does a piece of the world actually store,
and what does each stored value mean?** It is grounded in the adversarially-
verified survey at [`ref/research/material-models-survey.md`](ref/research/material-models-survey.md)
and in `DESIGN-REDUX.md` §§5, 8, 11–15.*

*Epistemic status is marked inline: **settled** (a decision already made, often in
code), **our stance** (reasoned, adopt-unless-contradicted), **research-backed**
(confirmed by the survey), **TENTATIVE** (shape reserved, decide with real
usage). The concrete schema in §9 is TENTATIVE throughout — it is a starting
proposal, not a fixed API.*

---

## 1. The one principle, applied to matter

**Representation follows consequence** (`DESIGN-REDUX.md` §0). We do not pick "how
to store elevation" in the abstract; we work backward from the *queries* a
human-scale world must answer — climbability, slumping, foot-placement,
reachability, lateral pressure, fluidity, cohesion — and the properties those
queries read *are* the data model. Corollary, and the hardest thing to keep hold
of: **a cell is a sufficient statistic, not a number** (§5). "Elevation 8000" is
not a fact; it is a *reading*, and different consumers need different readings, so
the honest unit carries a small tuple with declared meaning, not one scalar.

---

## 2. Three representations of matter *(research-backed; the split is our stance)*

Matter is not one shape, and forcing it into one (a grid of cubes) fights most of
what it does. Three representations, each matched to a phenomenon's structure:

| representation | for | form |
|---|---|---|
| **stratum** (bed/layer) | vertically-coherent matter (sediment, soil horizons, bedrock) | run-length interval: `(material, real-valued thickness, per-stratum state)` |
| **voxel** | adjacency algorithms, rendering, per-cell edits | cubic 0.5 m cell — a **materialized view**, not necessarily the storage |
| **body** (intrusion / vein / cave / boulder / structure) | cross-cutting masses with their own shape | discrete 3-D object, **overlaid** on the strata |

Nouns kept distinct (conflating them is the confusion to avoid): **column**
(vertical unit at a tile), **voxel** (cubic materialized cell), **stratum/bed** (a
material run), **body/intrusion** (a cross-cutting mass — geology's *dike* /
*sill* / *pluton*). This is the **base-field + sparse-overlay** pattern already in
core: strata are the substrate; bodies (and user edits) override it in a region.

**Key consequence:** a voxel is *not* the storage primitive — it is a query view.
"10.5 m of sand" is **one** `Stratum{sand, 10.5 m}`, not 21 half-metre cubes;
voxels are materialized from strata only where per-cell detail is asked for. This
is the "view-resolution decoupled from intrinsic-resolution" idea from `DESIGN.md`,
applied to the vertical.

---

## 3. The column is the primary unit *(settled — it's the name in core and in the 2009 neworld notes)*

A **column** at a cube-sphere tile is a **stratigraphic column**: an ordered stack
of strata, bedrock at the bottom, up through soil/regolith, with standing water
(if any) above the solid top. Geology already draws exactly this. The column is
where the vertical-integral aspects (hydrology, overburden, energy balance)
naturally live, and it is the unit a coupler fluxes *between*.

The shallow-water/erosion state the survey found the middle rung needs — the trio
`(b = terrain height, d = water depth, r = regolith thickness)` — **falls out of
the strata model for free**: `b` = sum of solid stratum thicknesses, `r` =
thickness of the topmost *loose* (powder/mobile) strata, `d` = the column's water
depth. The representation subsumes the state; we do not store `(b, d, r)`
separately.

---

## 4. What a stored value *means* must be declared — and it is plural *(our stance; §14)*

The trap the survey and our own reasoning both point at: "elevation" means
different things to erosion (a conserved volume), rendering (a surface height),
and line-of-sight (a max), and each silently assumes its own. So:

- **The conserved primitive is volume / mass of material** — *not* a height.
  Erosion and hydrology move mass; conservation is the thing that must survive
  every LOD crossing. This is **finite-volume** thinking (store the cell-integrated
  conserved quantity), which is the correct frame for the physics.
- **Surface elevation is a *derived* reading: top-of-topmost-solid, sampled at the
  column's center** — a **finite-difference** node. *(Settled by code: the slabs
  renderer already put mesh vertices at column centers, so this convention is
  chosen; §14 just names it so a later tier can't quietly treat it as a mean/max.)*
- **`min` / `max` are carried alongside** for the consumers that need extremes
  (`max` → line-of-sight, occlusion, collision-ceiling; `min` → "fully above
  water"), and they bound how much the center-sample lies about sub-cell relief.
- **The top voxel gets a fill fraction, not a solid/air bit** — so 10.5 is exact
  and the surface is sub-voxel (Volume-of-Fluid). Rendering and collision read the
  fraction per their needs.

Converting between these readings (volume↔height, sample↔mean) is lossy and
directional → it is **coupler** work (§4/§12), and any downscale must honor
whichever statistic was stored (stored mean ⇒ volume conserved; stored max ⇒ peak
survives). Store the wrong one and the fine materialization cannot serve its
consumer — the §5↔§6 silent corruption.

*Prior art (not novel):* finite-volume vs finite-difference; GDAL `AREA_OR_POINT`
and DEM grid registration (a cell value's meaning must be declared); Arakawa
staggering; Volume-of-Fluid (Hirt & Nichols); marching cubes / dual contouring.

---

## 5. The property set *(research-backed; the exact schema is TENTATIVE)*

The survey confirmed the list core already implied is **very nearly spanning**.
Grouped by where each property lives; **(+)** = surfaced by the survey as missing.

**Per-material-type** (static, keyed by material — the physics rungs *fill* these):
- **density** — split **solid** and **liquid** **(+)** [kg/m³]
- **cohesion** `C` [Pa] and **internal friction angle** `φ` [rad] — Mohr–Coulomb
  `τ = C + σ′·tan φ`; for cohesionless matter the **angle of repose *is* φ**
- **grain size** `d` [m]
- **porosity / permeability**, and **packing / volume fraction** `φ_pack` **(+)**
- **phase-state** — Solid / Liquid / Gas / Powder / Paste **(+)** (the *discrete*
  form of "fluidity" — governs falls-vs-flows-vs-holds; the crude rung's whole rheology)
- **erodibility** `K` and an **explicit incision threshold** **(+)** — stream power
  `I = K·Aᵐ·Sⁿ` above threshold, nothing below

**Per-stratum / per-column dynamic state** (time-varying):
- **saturation / pore-water pressure** `u` — cuts strength via effective stress
  `σ′ = σ − u`; slope-parallel seepage ~halves the factor-of-safety (dominant
  landslide trigger, per the survey)
- **water depth** `d`; **regolith/mobile-sediment thickness** `r` **(+)** (both
  derived from strata, §3)
- **temperature** (future — freeze/thaw)

**Derived-geometric** (computed from the surface field, not stored): surface height
(center point-sample), **slope** `β`, **overburden / normal stress** `σ = ∫ρg dz`.

Property *values* at interfaces are **rich `Quantity`** (unit + exactness, §
`DESIGN-REDUX` §12) — a `Material` table is low-volume seam data, so richness is
affordable there; hot per-voxel loops take raw `f64` under the boundary contract.

---

## 6. The material model is a fidelity ladder; the property set is the stable interface *(research-backed)*

The **property set of §5 is the coupler interface** (§4/§12). The *model* behind it
swaps along a ladder without touching consumers:

1. **block/tile (crude):** material = discrete type + phase-state + a scalar
   strength. *(Dwarf Fortress stores six stress-modes × yield/fracture/strain —
   over-fidelity for us; a scalar suffices here. **Its strength numbers are
   game-scaled, not SI — borrow the model shape, take physical values from rung 2.**)*
2. **geotechnical + hydrologic (middle):** Mohr–Coulomb infinite-slope
   factor-of-safety + shallow-water / stream-power erosion, over `(C, φ, u)` and
   the strata-derived `(b, d, r)`.
3. **granular (high):** continuum μ(I) rheology (μ₁, μ₂, I₀; inertial number from
   `d`, grain density, confining pressure, shear rate), then DEM (per-particle
   contacts calibrated so emergent angle-of-repose ≈ `φ`).

**The load-bearing validation:** **SHALSTAB** (Montgomery & Dietrich; USGS) couples
slope-stability and water-routing **on the same per-cell state** — published
evidence that one substrate's property interface serves *both* query families.
**We do not fork the substrate per physics.**

### The material *taxonomy* is a ladder too — undifferentiated materials *(our stance; TENTATIVE)*

A material need not be fully individuated to exist. A type can be
**undifferentiated** — the geology-map term for a unit not yet subdivided (e.g.
"undifferentiated Quaternary sediments") — a coarse category carrying *broad
statistical* properties, refined into finer types later without breaking existing
worlds or saves. The refinement tree:

```
undifferentiated → {igneous, sedimentary, metamorphic} → {granite, basalt, sandstone, …} → {mineral composition}
```

or, cruder still, a **gameplay-attribute bottom rung** — `hard / soft / loose /
slippery` ground — defined directly by the axis the algorithms care about, *before
any geological identity exists*. (Others we'll want early: `rich / poor / rocky`
soil; an "ore" flag on any host rock.)

Two properties make this safe — the same ones the whole doc rests on:
- **Refinement is a deterministic prefix (`DESIGN-REDUX` §8).** "Undifferentiated
  igneous at `(x,y,z)`" refines to "granite" as a *pure function* of (coarse label,
  coordinate, macro) — so a save stores only the coarse label and the finer one is
  *recomputed*, exactly as coarse LOD is a prefix of fine detail. The coarse
  category is not a placeholder to be destructively filled; it is the high-order
  bits of the material identity.
- **The coarse category carries the sufficient statistics the refinement must honor
  (§5).** "Undifferentiated igneous" declares a density/strength *range*; the
  granite/basalt distribution it refines into must integrate back to it — wrong
  statistics corrupt the macro (the §5↔§6 tie).

Crucially this keeps **saves playable across a fidelity bump**: because refinement
only *relabels/individuates* (adds detail) rather than *moves material*, a user
mutation made against "undifferentiated igneous" (a dug tunnel) stays valid when
the rock is later called granite — the tunnel is in the same place regardless of
the label. So **`Material::Undifferentiated(category, stats)` is a first-class
type**, and individuation is a ladder rung, not a rewrite.

---

## 7. Bodies — the overlay *(our stance; TENTATIVE shape)*

Cross-cutting masses (granite plutons, veins, caves = void, boulders, built
structures) are modeled as **bodies overlaid on the strata substrate**, not
decomposed into the columns they intersect. The effective material at
`(tile, altitude)` is *body-if-present, else strata*. This is the same base+overlay
as core's sparse edits, and it is where the **regenerable vs. irreducible**
distinction (§`DESIGN-REDUX` §13, itself TENTATIVE) lands: a worldgen intrusion is
a *regenerable* body; a user-dug tunnel is an *irreducible* one. The detail→abstract
frontier (§6) is exactly "absorb a body's effect back into the macro."

---

## 8. Spatial and material binding *(settled / research-backed)*

- **Address — the nailed-down plan** *(settled; rationale in
  [`ref/research/spatial-key-bench.md`](ref/research/spatial-key-bench.md) and
  `DESIGN-REDUX` §4/#4):*
  - **`CellId(u64)` — a Hilbert-ordered cube-sphere cell id, the S2-geometry
    scheme** (6 faces, quadtree per face, Hilbert-packed with a level sentinel;
    levels 0..25, where level 25 ≈ a ≤0.5 m footprint). Implemented dep-free (~the
    S2 id scheme, not the crate). This is the **canonical key** — column identity,
    memo keys (§11–12), save-store roots (§13), LOD (parent = shift). Exact,
    drift-free (no far-lands), hashable, and Hilbert-ordered so a region is a
    contiguous id range (storage/streaming locality).
  - **The curve orders *chunks*, not cells.** A **chunk** is the storage/streaming/
    LOD unit (keyed by `CellId`); its **interior is a plain Cartesian row-major
    array**. Automata (erosion, slumping, flow) run on the Cartesian interior where
    neighbours are `idx±1`. *Measured: Cartesian neighbours are **80× faster** than
    Hilbert-id neighbours, and the stencil runs at **~6 Gcells/s** while a patch
    fits cache — so curve arithmetic is designed out of every hot loop; per-cell
    encode/decode never happens.*
  - **Two size tiers** (forced by the arithmetic + the measured cache cliff): a
    coarse macro/streaming **chunk** (tens of km — a 75 km chunk at 0.5 m is
    2.25×10¹⁰ cells, undense-able, hence an LOD container), and a fine automata
    **patch** at the cache sweet spot **~0.25–0.5 km** (256²–1024²).
  - **Halos** carry cross-chunk and cross-cube-face neighbours (with the face
    coordinate/axis transform) as a ghost border, filled at patch load, so the
    inner loop never branches (domain-decomposition + halo-exchange).
  - **`f64 CubeCoord{face,u,v}`** stays as the math/conversion view (projection,
    lat/lon, floating-origin rendering) — never a key.
- **Vertical extent:** the **~20 km livable shell** (deepest trench to highest
  peak); the deep interior is a coarse global model fluxing heat/uplift into the
  shell's base. Altitude is an integer voxel level within the shell.
- **Cell geometry:** a column is a **radial frustum** treated as a square prism —
  the taper across a 0.5 m cell at Earth radius is ~1e-7, negligible (noted in
  `sphere.rs`).
- **Isotropy:** voxels are cubic 0.5 m *by choice* (the true-scale geology anchor),
  not by definition — "voxel" is only a 3-D grid sample and can be anisotropic; we
  chose cubic so one length suffices.

---

## 9. A concrete proposed schema *(TENTATIVE — a starting sketch, not an API)*

```rust
// Static, keyed by a small id. Physics rungs FILL these; the crude rung fills
// them coarsely, the geotechnical rung from real (SI) values. Low-volume seam
// data → rich `Quantity` is affordable here.
struct Material {
    id: MaterialId,               // Granite | Sand | Clay | Water | Void | ...
    phase: Phase,                 // Solid | Liquid | Gas | Powder | Paste
    density_solid: Quantity,      // kg/m³
    density_liquid: Quantity,     // kg/m³ (melt / dissolved form)
    cohesion: Quantity,           // Pa   (C)
    friction_angle: Quantity,     // rad  (φ; == angle of repose when cohesionless)
    grain_size: Quantity,         // m    (d)
    porosity: Quantity,           // ratio
    permeability: Quantity,       // m²
    packing_fraction: Quantity,   // ratio (φ_pack)
    erodibility: Quantity,        // K
    incision_threshold: Quantity, // shear/stress below which no incision
    // high rung (deferred): mu_i params, multi-mode strength
}

// A run of one material — the STORAGE primitive for vertically-coherent matter.
// Real-valued thickness ⇒ "10.5 m of sand" is one stratum, fractional for free.
struct Stratum {
    material: MaterialId,
    thickness: Quantity,          // m (real-valued)
    saturation: f32,              // 0..1 pore-water fraction (dynamic state)
    // temperature, disturbance — future
}

// A stratigraphic column at a tile: strata bottom(bedrock)→top, water on top.
// Elevation / (b,d,r) / overburden are DERIVED, never stored as primitives.
struct Column {
    tile: TileId,                 // cube-sphere integer quadtree address (#4)
    strata: Vec<Stratum>,         // bedrock → surface
    water_depth: Quantity,        // m, standing water above the solid top
}

// A cross-cutting mass overlaid on the strata (§7). Void material = a cave.
struct Body {
    shape: BodyShape,             // implicit (SDF) or explicit region
    material: MaterialId,
    provenance: Provenance,       // WorldgenRegenerable | UserIrreducible (§13)
}
```

Derived queries (computed by the physics/query tier, **not stored**) read the
above: `surface_height` = Σ solid thicknesses (center-sample); `slope` = gradient
over neighbour tiles; `overburden(depth)` = Σ ρ·g·thickness above; `factor_of_safety`
= Mohr–Coulomb of (slope, top-material `C`, `φ`, `saturation`); a materialized
**voxel** = the material at `(tile, level)` = body-if-present-else-strata, with the
top voxel's **fill fraction** from the sub-cell surface.

---

## 10. Materialization and determinism *(our stance; §8, §11)*

Fine detail (a gold vein, sub-stratum texture) is materialized **statelessly** as a
pure function of `(coordinate-coherent noise) × (macro-modulated amplitude)` — the
vein *continues* because the noise field is continuous, *tapers* because the macro
field modulates it below threshold. This gives coherent, seam-free, macro-honoring
detail **without** path-dependence. **Do not** grow detail from an already-
materialized neighbour's *state*: that makes the result depend on materialization
order, which breaks determinism (§8) and content-addressed memoization (§11–12).
Genuinely non-local features (river paths from a watershed, fault propagation) are
the exception — they cannot be faked with local noise and require real (costly)
simulation; reaching for cheap propagation there yields a *plausible-but-wrong*
result.

---

## 11. Settled vs. tentative (so future work knows what it may move)

- **Settled (in code or by prior decision):** the column as primary unit; surface
  elevation = center point-sample (renderer-committed); cubic 0.5 m voxels; the
  cube-sphere integer tile as canonical spatial key; the ~20 km shell; rich
  `Quantity` at seams / raw `f64` in loops.
- **Research-backed:** the property set is very nearly spanning; the five additions
  (split density, packing fraction, phase-state enum, incision threshold, regolith
  thickness); the material-model ladder; one substrate serves both slope-stability
  and hydrology (SHALSTAB).
- **TENTATIVE — reserve, don't fix:** the exact schema of §9; the regenerable/
  irreducible split and body representation (§7); whether volume/mass or a hybrid
  is the stored primitive; the full mutation-log design; which high-rung properties
  to carry now vs. defer.
- **Open frontier (unchanged):** absorbing an irreducible body/edit back into the
  macro model with correct up-invalidation (detail→abstract, `DESIGN-REDUX` §6).

*This document does not supersede anything; it is where the matter-model thinking
is worked out. Adopt the schema deliberately, and update this file when usage
teaches us better — future instances inherit what we leave.*
