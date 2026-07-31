---
slug: obs-chart-edge-halo-clamps-to-the-face
type: observation
status: exact
stage: draft
depends:
  - form-cellid-chunk-patch
  - form-same-level-halo-exchange
  - form-declared-boundary-contract
  - norm-declared-violation-is-not-license
  - norm-probes-before-claims
---

# A multi-cell fluvial halo at a cube face edge clamps to the chart; that is a boundary contract in force, not a missing comment

When Jacobi exchange asks for halo depth $d\ge 1$ on a tile whose enlarged window leaves the face, production samples **clamp** face indices to $[0, 2^L-1]$. The cross-face correspondence that would make the halo geometrically honest is measured to fail for $d\ge 2$. The clamp is therefore present-tense law at the chart edge until a declared resampling lands in the key.

## Formal Expression

1. **What production does (era `1b028c3`; the geometry half repaired 2026-07-30).** On the fluvial Jacobi path, an edge tile is carved on an $(n+2d)^2$ window. When that window's face coordinates would leave the chart, `uplift_rate_tile` **clamps** each index to the face range rather than panic. The window constructor and the exchange path's prior closure no longer do: off-chart cells are **resampled by direction** (2026-07-30) — the cell's extrapolated centre is a real point on the sphere, and its height is read from whichever cell contains that point. That is the declared resampling FE(2) calls for, and it is not the index-copy FE(2) measured as failing, because a direction names a place rather than a coordinate in another chart. Approximate, not exact: extrapolating one face's parametrisation past its edge distorts spacing, so far out the direction can land a cell off from ideal correspondence.

   **What it no longer clamps is the cell's geometry.** Clamping the metric is not a coarser version of clamping the data: it made distinct cells share a centre vector and therefore sit **0.0 m** apart, which every slope and flux then divides by, minting non-finite heights in 83 stored roots per cohort across seven cohorts. Centres and areas are now taken from the *unclamped* requested index — the equiangular formulae are analytic and extrapolate — while the height still reads from the clamped cell. Measured and repaired at #obs-halo-windows-overhang-the-chart-and-mint-nan .

   The split preserves FE(6)(c): chart safety comes from the **data** clamp, since it is `CellId::from_face_ij` that Hilbert-encodes. `measure::cell_center_unit` and `cell_area_m2` take raw indices and construct no `CellId`, so the L6 whole-face $d{=}16$ admission specimen still passes. Sites: `erosion.rs` (`from_surface`), `uplift.rs` (`uplift_rate_tile`), `query.rs` (`erosion_region_exchanged_staged` window makers). The clamp is not optional instrumentation — it is what keeps a whole-face planet build from overflowing Hilbert encode at coarse levels (CLI admission specimen: L6, one $64^2$ tile per face, $d{=}16$).

2. **What was already measured about the geometry.** #form-cellid-chunk-patch FE(4) / cross-face spike PROBE 2: across a cube edge the two grids co-align **only on the shared edge**. Depth-1 halo correspondence is exact; depth 2 already mismatches on 200/256 edge cells (78%); 232/256 by depth 4, worst at cube corners. So for any production schedule with $d\ge 2$ (including `HaloSchedule::for_build`'s measured $d{=}16$), a *true* off-face halo is not a copy of neighbour-face indices — it requires a **declared resampling**. Nothing in the key yet names that resampling.

3. **The clamp is a boundary contract.** #form-declared-boundary-contract : an edge policy that is in force but not in the declaration column is the defect class that made edge-sink tiles look lawful. The chart-edge clamp is the same shape at a different family ( #form-seam-families "cube chart-edge"): every promise near a face perimeter under multi-cell halo is evaluated under "repeat the rim cell" (or nearest in-chart cell), not under "neighbour face via involution" or "resampled band." Until the key carries a resampling descriptor, the honest label is a **declared deficiency** ( #norm-declared-violation-is-not-license ), not silence.

4. **Relation to same-level tile exchange.** Interior-to-face Jacobi exchange ( #obs-exchange-repairs-the-seam-and-overlap-does-not , L13 4×4 block) never left the face, so its structural budgets do **not** license the chart edge. Production adoption made the edge load-bearing: whole-face sweeps place tiles on every perimeter. Repairing same-level *internal* seams does not automatically repair chart-edge halos.

5. **Relation to the panel two-point defect.** #obs-the-two-point-defect-lives-on-the-panel-seam is an operator defect on conforming panel rings. The clamp is a *sampling* policy for multi-cell halos that leave the chart. Same twelve lines of the cube, different objects ( #form-seam-families ).

6. **The clamp's two halves are different claims, and only one is a policy.** Sampling a neighbouring cell's *height* by repetition is a declared deficiency — a stated boundary contract, wrong but well-typed, exactly what this segment exists to name. Repeating its *position* is not a contract at all: it asserts that two distinct cells occupy one point, which no resampling choice could ever be. So the resampling work FE(2) calls for remains open for the data, and there was never a version of it that the geometry needed to wait for.

7. **Falsifiers.** (a) A production path that samples true neighbour-face geometry (with declared $d\ge 2$ resampling in the key) retires the clamp as present law for that path. (b) A measurement that depth-$d$ clamp is bit-identical to a principled resample for the schedules we ship would collapse the deficiency claim for those schedules only — unexpected and must be shown, not assumed. (c) Removing the clamp without a replacement returns the overflow panic — the clamp is load-bearing for chart safety.

## Epistemic Status

**Max attainable: exact** for "production clamps" (inspect the cited functions) and for "depth ≥ 2 cross-face mismatch was measured" (PROBE 2 numbers on #form-cellid-chunk-patch FE(4); re-quote only under that probe's era or re-run).

**Currently `exact` as observation of present code and of the already-recorded geometric measurement.** The *normative* claim that resampling must enter the key is `robust-qualitative` formulation shared with #form-same-level-halo-exchange scope and #form-declared-boundary-contract . No new geometric re-measure in this segment's era (`1b028c3`); the L6 overflow that forced the clamp into the fluvial path is a build-path specimen, not a new PROBE table.

Stage `draft`.

## Discussion

The failure mode this segment blocks: calling the planet "exchange-repaired" after Jacobi adoption while face rims still run under an undeclared rim-repeat policy. Exchange repaired the *tile* family inside the face; the *chart-edge* family was made louder, not smaller.

## Working Notes

- **Key shape when resampling lands:** extend `HaloSchedule` (or a sibling field) with an explicit `chart_edge: Clamp | Resample{…}` so two worlds that differ only there cannot share a key. Until then, clamp is ambient and invisible to content-addressing — the same class of under-key risk #form-complete-content-addressed-key exists to prevent.
- **chunk.rs default-0 out-of-face** remains a separate specimen (Dirichlet-0 mass sink) on the patch loader path; do not merge that crime with the fluvial clamp without a dual-arm measure.
