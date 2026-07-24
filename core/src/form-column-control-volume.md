---
slug: form-column-control-volume
type: formulation
status: exact
stage: draft
depends:
  - post-represent-by-consequence
  - form-fidelity-invariant
  - form-complete-content-addressed-key
  - norm-declaration-must-convict
---

# A column is a control volume

A column is a control volume that carries, per quantity, a sufficient statistic plus declared exactness — not a point sample pretending to be every reading at once.

## Formal Expression

1. **Primary vertical unit.** A **column** is the stratigraphic stack at one horizontal address on the world grid (cube-sphere cell / tile): ordered strata (bedrock toward surface), with standing water (if any) above the solid top. It is the unit vertical-integral aspects (hydrology, overburden, energy balance) and inter-column fluxes naturally address.
2. **Control volume, not a point.** The column is a **finite-volume cell** in the horizontal packing: it holds integrated mass/volume (and other conserved quantities), not a mathematical point that "has" a slope. **Slope, curvature, smoothness, and every vector field are reconstructions** over the dual mesh joining cell centres — derived, never stored as if they were column primitives. The manner of reconstruction **is** the scheme.
3. **Sufficient statistic per quantity.** Per stored quantity the column carries a **sufficient statistic** for the laws that will read it at that scale, plus an **exactness** claim (guaranteed vs approximate). A bare mean is not automatically sufficient for a nonlinear law (Jensen): coarse-graining without the statistic the law needs is not loss of precision — it **lies**. Minimum honest shape is consumer-dependent (conserved totals for budgets; max for line-of-sight; moments/variance where the law is nonlinear).
4. **Declared semantics.** For each quantity, the declaration must state whether the stored value is a **point sample**, a **cell average**, or **band-limited** (or another named reading). These are different quantities. Reading one array as all three is a live defect until semantics are declared and convictable ( #norm-declaration-must-convict ). Elevation is always a **derived reading** of mass/volume layout, never the conserved primitive.
5. **Matter representations (frame).** Vertically coherent matter stores as **strata** (run-length, real-valued thickness); **voxels** are a materialized **view**, not the storage primitive; **bodies** (intrusions, voids, structures) overlay the strata substrate. `(b, d, r)` shallow-water/erosion state falls out of strata + water depth rather than as a parallel world model.
6. **Seam duty.** Restriction/lifting between coarse and fine columns must honor the statistics each level declared. Storing the wrong statistic makes fine materialization unable to serve its consumers — silent macro corruption under #form-fidelity-invariant .

## Epistemic Status

**Max attainable: exact** for the **frame** (control volume + sufficient statistic + reconstructions-are-the-scheme). Ratified as project stance: `DECISIONS[column-is-a-control-volume-with-sufficient-statistics]` (`:by us`, decided) — Joseph accepted the frame ("it very much lands"); **implementation is not designed**. Stage `draft`.

**Open (do not smuggle as built):** (a) per-quantity semantic fork still live in code (point vs mean vs band-limited; conservation pins have mixed the readings); (b) flux `:statistic` / `:exactness` contract designed, not built; (c) which moments to carry for which laws; (d) temporal half of averaging (space **and** time) unaddressed here; (e) concrete `Column`/`Stratum` schema in design prose is TENTATIVE. Correct Jensen detail for stream-power at $n=1$ lives in DECISIONS / theory sources — this segment does not re-litigate that specimen.

## Discussion

Calling a column a heightfield pixel collapses conservation, rendering, and collision into one ambiguous float. Control-volume language forces the honest questions: what is conserved, what is reconstructed, and what statistic did we promise the next nomos? Those questions are the same ones the flux web and nomos contract ask at quantity and geometry boxes; this segment names the vertical object they act on.

## Working Notes

- **Dual homes demoted:** DESIGN-MATERIAL graduated (`.super-archive/from-design/DESIGN-MATERIAL.md`); REDUX §14; ARCHITECTURE §5 column sentence. DECISIONS column-is-a-control-volume remains ratification history, not claim home.
- **Do not invent** a fixed property schema or close the point/mean/band-limited fork without Joseph or a convicting design.
- **TENTATIVE schema shapes (from graduated MATERIAL §7/§9 — reserve, do not harden):**
  - `Stratum { material, thickness, saturation, … }` as run-length storage primitive; `Column { tile, strata, water_depth }` with elevation/$(b,d,r)$/overburden **derived**.
  - `Body { shape, material, provenance }` overlay (regenerable worldgen vs irreducible user edit) — law-closure of body *effects* → #sketch-detail-abstract-reversion .
  - **Interface-state slot** on column (not bulk stratum): colmation, armoring, later crusts/snowpack — live partially in `water.rs`; schema placement open.
  - Volume/mass vs hybrid stored primitive unfixed.
- Sibling: #form-material-property-interface property tables; #form-cellid-chunk-patch binding.
