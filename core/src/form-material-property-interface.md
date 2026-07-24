---
slug: form-material-property-interface
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-column-control-volume
  - form-fidelity-ladder
  - form-flux-web
  - post-determinism-as-ontology
---

# The material property set is the stable interface; the model behind it is a ladder

Matter carries one declared property set — the coupler interface every physics rung fills — so the material model swaps along its fidelity ladder without touching consumers. Material *taxonomy* is a ladder too: an undifferentiated type refines deterministically, honoring the statistics it declared.

## Formal Expression

1. **One property interface.** Material behaviour is queried through a declared property set — per-material statics (density split solid/liquid, cohesion $C$, friction angle $\varphi$, grain size, porosity/permeability, packing fraction, phase-state, erodibility $K$ + incision threshold) and per-column/stratum dynamics (saturation/pore pressure, water depth, regolith thickness) — not through model internals. The set is the coupler interface in the sense of #form-flux-web : rungs fill it; consumers read it.
2. **The model is a ladder behind the interface** ( #form-fidelity-ladder ): (i) crude — discrete type + phase-state + scalar strength; (ii) geotechnical/hydrologic — Mohr–Coulomb factor-of-safety + shallow-water/stream-power over $(C,\varphi,u)$ and strata-derived $(b,d,r)$; (iii) granular — $\mu(I)$ rheology, then DEM. Swapping a rung must not touch consumers that read only the interface.
3. **One substrate serves all query families.** SHALSTAB (published, coupled slope-stability + water-routing on the same per-cell state) is the external evidence that one property substrate serves both mechanical and hydrological queries. **Do not fork the substrate per physics.**
4. **Undifferentiated materials are first-class.** A material may exist as a coarse category ("undifferentiated igneous"; or a gameplay-attribute rung: hard/soft/loose) carrying **broad statistical properties**, refined later into finer types. Two conditions make this safe:
   - **Refinement is a deterministic prefix.** "Undifferentiated igneous at $x$" refines to "granite" as a pure function of (coarse label, coordinate, macro) — fated ( #post-determinism-as-ontology ), so a save stores the coarse label and recomputes the fine one. The coarse category is the high-order bits of material identity, not a placeholder destructively filled.
   - **The category declares the statistics refinement must honor.** The declared density/strength *range* of the coarse type bounds the distribution it refines into, which must integrate back to it ( #form-column-control-volume sufficient-statistic discipline). Wrong statistics corrupt the macro.
5. **Saves survive fidelity bumps.** Because refinement relabels/individuates rather than moving material, a user mutation made against the coarse label (a dug tunnel) stays valid when the rock is later individuated — the tunnel is in the same place regardless of the name.
6. **Not this segment.** The concrete Rust schema (`Material`/`Stratum`/`Column`/`Body` field lists) is TENTATIVE source, deliberately unfixed. Interface-state placement (colmation, armoring — state of the water/stratum *surface*, not any bulk) is a named residual, not settled schema.

## Epistemic Status

**Max attainable: exact** for the interface-stability architecture under the fidelity ladder. **Currently `robust-qualitative`:** the property list is **research-backed** — a survey run under a self-described 3-vote adversarial protocol (`ref/research/material-models-survey.md`) found the project-implied list near-spanning; the survey itself flags four missing members (split density, packing fraction, phase-state enum, incision threshold), and DESIGN-MATERIAL's tagging adds regolith thickness as a fifth, inferable from the survey's $(b,d,r)$ minimal-state finding. SHALSTAB is read prior art. The undifferentiated-refinement construction is design stance (DESIGN-MATERIAL §6, *our stance / TENTATIVE* at source) — coherent under fated noise + declared statistics, **unbuilt and unprobed**. No Joseph DECISIONS row for the interface as standing law.

**Not claimed:** any rung beyond crude erosion/water as built; DF-derived strength values (game-scaled, not SI — borrow the model shape only); the schema.

Stage `draft`.

## Discussion

The interface bet is what keeps "features are compositions, not new engines" true: Bryce hoodoos are strata + per-stratum erodibility; ore veins are host rock + the materialization law — small compositions over one substrate. The undifferentiated rung is the same bet applied to identity itself: worlds and saves stay playable across a taxonomy refinement for exactly the reason coarse LOD is a prefix of fine detail.

## Working Notes

- **Dual-home demote:** DESIGN-MATERIAL §§5–6 and DESIGN-REDUX §15 get claim-home banners; property tables and schema sketch stay TENTATIVE source.
- **Interface-state slot:** colmation **and** armoring/colmation depth are live in `water.rs` (`armor`, `armor_depth`, `armor_shield`); schema-as-interface placement still residual — do not claim the full Material/Stratum schema is fixed.
- Sibling: scaffolding-with-a-demolition-date (explicit fields that parameterize what finer rungs will emerge — armor, colmation) belongs to #form-fidelity-ladder 's climb story; noted there.
