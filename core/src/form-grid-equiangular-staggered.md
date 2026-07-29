---
slug: form-grid-equiangular-staggered
type: formulation
status: robust-qualitative
stage: draft
depends:
  - disc-algorithms-disguise-physics
  - norm-bias-vs-noise
  - form-sphere-continuous-surface-fields
---

# Equiangular cube-sphere grid (tentative): keep the cube, change the kernels

The vivium's spatial substrate is the **equiangular cube-sphere** with Hilbert-ordered `CellId`, **Arakawa-C staggering** (scalars at cell centres; fluxes on faces as first-class keys), and **per-cell metrics** precomputed per tile. The router is **not** part of this decision. The grid is tentative in the honest sense: reopening conditions are named; measurements stand either way.

## Formal Expression

1. **Coordinate system (tentative keep).** Retain the equiangular gnomonic cube-sphere and `CellId` quadtree addressing (levels as power-of-two nesting). Do **not** switch base grid to Snyder equal-area cube or HEALPix for seam relief: whole-sphere harness measurements (`grid_lab`, nine grids) show equal-area buys area uniformity and loses shape / accuracy where it matters; HEALPix has the worst non-orthogonality among quads measured here.
2. **Storage arrangement.** **Staggered (Arakawa-C):** cell-centred scalars; **face-stored fluxes** as keyed objects (a face owned by exactly one of its two cells, canonically the lower `CellId`). Collocated flux reconstruction remains a scheme choice; the *identity* of the flux object is face-keyed.
3. **Metric machinery.** Carry continuous metrics **per cell** (area, edge lengths, centre-to-mid-edge arms, non-orthogonality), analytic in position and precomputed per tile/level — derived quantities, not a uniform `cell_m²` for the whole face (Putman & Lin: areas and metric factors are derived). Discrete configuration class (cheap branch) is separate from continuous metrics (evaluate, do not tabulate as if constant).
4. **Not covered.** The **router** (MFD replacement / gradient-reconstructed edge flux / anything that splits outflow) is **open**. Rotation-test gates and curl/potential-identity probes bind any future router claim. This segment does not adopt a drainage algorithm.
5. **Cube-locked structure control (mandatory).** The grid *manufactures* cube-aligned bias when kernels assume a flat uniform Cartesian tile. Measurement homes: #obs-cube-locked-kernel-bias (MFD fan; uniform cell-area $A$) and #obs-routing-curl-spiral (contour-orthogonality / spiral). **If emergent plate boundaries or drainage correlate with cube-face edges/corners, the result is void** until those biases are ruled out.
6. **Reopening conditions.** Reopen the keep-cube decision only if a measured, scheme-independent defect requires a different topology (not because a paper default prefers hex, and not because a kernel was never adapted — #disc-algorithms-disguise-physics ). Leaf-only evolution vs independently memoised coarse tiers is a **price** on the flux-on-face path, not a silent part of this decision (`DECISIONS` wavelet-store / seam family).
7. **Relation to sphere-continuous fields.** #form-sphere-continuous-surface-fields is the prior/domain law on $S^2$. This segment is the *lattice* the schemes live on. Both are required: continuous priors on a biased router still carve cube geography.

## Epistemic Status

**Max attainable: exact** for measured grid comparisons (area ratios, non-orthogonality, accuracy on shared harness — `ref/research/grid-comparison-report.md`, `foundation-validation` equiangular correction). **Currently `robust-qualitative`:** the *verdict* "keep equiangular cube-sphere + stagger + per-cell metric" is **Joseph-tentative** (`DECISIONS[the-grid-tentatively-decided-keep-the-cube-sphere-and-stagger-it]`, `:by us`, decided 2026-07-13 — "write the grid in as our tentative decision"). Per-cell **area** and **neighbour great-circle lengths** are built (`measure::cell_area_m2` / `cell_center_unit` / `gc_dist_m`) and wired into the live fluvial kernel; **edge lengths, centre-to-mid-edge arms and non-orthogonality** (the staggered-FV metric the router successor needs) are **not yet built**; staggered store open; router open. Cube-locked *metric* bias is now paid on the fan path; the directional-fan bias is not (it is not a metric).

**Do not overclaim:** earlier agent prose that closed the grid as settled without Joseph is retracted (`grid-question-not-closed-authority-was-inflated` lineage). This segment states the tentative keep and its measurements, not a frozen forever-grid.

Stage `draft`.

## Discussion

Uniformity beat orthogonality in the measured set; the hexagon envy was misplaced for our quadtree constraints. The load-bearing work is therefore kernel honesty on the cube we keep — not another projection hunt — and a standing control that refuses to read cube-locked "plates" as science.

## Working Notes

- **Primary sources:** `ref/research/grid-comparison-report.md`; `foundation-validation.md` §3; `DECISIONS[the-grid-tentatively-decided-keep-the-cube-sphere-and-stagger-it]`; Putman & Lin 2007; Snyder 1992 (negative lead on equal-area for our metrics); seam-adjacency addenda.
- **Measured cube-locked siblings (own extract later if needed):** `mfd-fan-is-a-bias-and-does-not-converge`; `drainage-area-uses-a-uniform-cell-area`; `routing-violates-the-potential-identity-…`. Closed-form spherical cell **area** and per-cell **centre / great-circle length** helpers are promoted from `msc/spike-wavelet-store/src/area.rs` into `measure.rs` and wired (`cell_area_m2`, `cell_center_unit`, `gc_dist_m`, `neighbor_center_dist_m`). Still in the spike, awaiting the staggered-FV router: the **edge-length** helpers (`arc_len_m`, `east_edge_len_m`, `north_edge_len_m`) and the arm / non-orthogonality metrics.
- **Configuration-class sizing (measured — `DECISIONS[configuration-classes-are-two-not-ten]`):** the discrete branch has **2** arms at every level (a regular cell, and a defective set fixed at 24 cells world-wide forever); geometric/congruence classes at 1% grow as $N^2/8$ (L3 $\to$ 10 is a coincidence of that level, not a law). FE(3)'s evaluate-don't-tabulate is the load-bearing consequence — a fixed dispatch table sized "about ten" would be correct at L3 and wrong at L4.
- **Supersession note for `#disc-algorithms-disguise-physics` Working Notes:** that segment correctly forbade closing the grid by agent fiat; this segment is the later Joseph-tentative keep. Method (port physics, not paper grid) still stands.
- **Price unpaid:** face-keyed store + leaf-only evolution vs double-evolve / mean-pin — see #form-face-flux-register , #form-rl-closure-algebra , #form-seam-flux-exchange .
- **Open leads (from graduated discretisation §7 — not law):** empty cell is *structure-preserving staggered on cube-sphere quadtree* (everyone who went mimetic went icosahedral for free orthogonality); Balsara 2001 area-weighted restriction precedent for AMR face flux; hydrostatic reconstruction fails when $\Delta z \ge h$ (thin films on steep ground — masquerades as seam); panel×AMR seam measured artifact; Randall Z-grid unmeasured multigrid affinity; vertex-centred dual-hex on nesting triangles is a **lead not a proposal** (duals may not nest). Do not re-open grid by agent fiat.
- **Router-spike residue map (verified 2026-07-29, correcting the from-memory guess that it belonged here):** this segment owns only the open-router clause (FE(4)); the spikes' measurements live at #obs-routing-curl-spiral FE(5)–(8) + WN and in the four council-accepted DECISIONS entries (`the-router-is-a-scalar…`, `mfds-output-is-not-a-discharge…`, `routing-spiral-landscape-consequence…`, `corrected-scheme-is-affordable…`), carried at near-verbatim fidelity. Both spikes graduated; paper-of-record PDFs relocated to `ref/research/pdfs/`.
