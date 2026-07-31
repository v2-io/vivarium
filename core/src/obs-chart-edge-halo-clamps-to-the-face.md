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

When Jacobi exchange asks for halo depth $d\ge 1$ on a tile whose enlarged window leaves the face, the fluvial path no longer rim-repeats heights: it **resamples by direction** (2026-07-30). Index-copy across a cube edge is measured to fail for $d\ge 2$, which is why the path uses a direction rather than a neighbour-face index. What remains undeclared is that policy **in the key**, plus residual clamp sites (notably uplift).

## Formal Expression

1. **What production does (present-trued 2026-07-31; path eras `1b028c3` → pad/NaN → direction resample).** On the fluvial Jacobi path, an edge tile is carved on an $(n+2d)^2$ window with a **signed** origin (pad, do not slide — #obs-halo-windows-overhang-the-chart-and-mint-nan FE(8)). When that window's face coordinates leave the chart:

   - **Heights (fluvial window + exchange prior):** **resampled by direction** — the cell's extrapolated centre is a real point on the sphere; height is read from whichever cell contains that point (`CubeCoord::from_unit(dir).cell(level)`). That is the resampling FE(2) calls for, and it is not the index-copy FE(2) measured as failing. Approximate, not exact: extrapolating one face's parametrisation past its edge distorts spacing. Convicting test: `an_off_chart_halo_cell_resamples_the_neighbouring_face`. Changelog `2026-07-30-04`.
   - **Geometry:** unclamped signed indices through `measure::cell_*_i` — clamping the metric minted 83 non-finite roots by putting distinct cells 0.0 m apart ( #obs-halo-windows-overhang-the-chart-and-mint-nan ).
   - **Still clamps data:** `uplift_rate_tile_at` (and any path that has not adopted the direction resample). Chart safety for Hilbert encode still needs either a clamp or a true cross-face `CellId` — the fluvial path uses the latter off-chart; uplift still uses the former.

   Sites: `erosion.rs` (`from_surface_at`), `query.rs` (exchange prior closure), `uplift.rs` (residual clamp).

2. **What was already measured about the geometry.** #form-cellid-chunk-patch FE(4) / cross-face spike PROBE 2: across a cube edge the two grids co-align **only on the shared edge**. Depth-1 halo correspondence is exact; depth 2 already mismatches on 200/256 edge cells (78%); 232/256 by depth 4, worst at cube corners. So for any production schedule with $d\ge 2$ (including `HaloSchedule::for_build`'s measured $d{=}16$), a *true* off-face halo is not a copy of neighbour-face indices — it requires a **declared resampling**. Nothing in the key yet names that resampling.

3. **The edge policy is a boundary contract — and the fluvial half has moved.** #form-declared-boundary-contract : an edge policy that is in force but not in the declaration column is the defect class that made edge-sink tiles look lawful. The chart-edge family ( #form-seam-families ) still needs the **key** to name which policy ran. Production fluvial heights now use direction-resample rather than rim-repeat; uplift still rim-clamps. Until the key carries a `chart_edge: Clamp | Resample{…}` descriptor, two worlds that differ only there can share a key — under-key risk ( #form-complete-content-addressed-key ), not silence about the path.

4. **Relation to same-level tile exchange.** Interior-to-face Jacobi exchange ( #obs-exchange-repairs-the-seam-and-overlap-does-not , L13 4×4 block) never left the face, so its structural budgets do **not** license the chart edge. Production adoption made the edge load-bearing: whole-face sweeps place tiles on every perimeter. Repairing same-level *internal* seams does not automatically repair chart-edge halos.

5. **Relation to the panel two-point defect.** #obs-the-two-point-defect-lives-on-the-panel-seam is an operator defect on conforming panel rings. The clamp is a *sampling* policy for multi-cell halos that leave the chart. Same twelve lines of the cube, different objects ( #form-seam-families ).

6. **The clamp's two halves are different claims, and only one was ever a policy.** Sampling a neighbouring cell's *height* by repetition is a declared deficiency — wrong but well-typed. Repeating its *position* is not a contract at all: it asserts that two distinct cells occupy one point. Geometry never needed to wait for a height policy; the NaN repair proved that. Height resample on the fluvial path is now landed (FE(1)); what FE(2) still owes is the **key descriptor** and residual clamp-site parity (uplift), not "nobody resamples yet."

7. **Falsifiers.** (a) ~~A production path that samples true neighbour-face geometry~~ — **half-met:** fluvial heights do; key does not yet declare it; uplift still clamps. (b) A measurement that depth-$d$ clamp is bit-identical to a principled resample for the schedules we ship would collapse the residual deficiency for those schedules only. (c) Removing every safety path without a replacement returns Hilbert overflow on coarse whole-face builds.

## Epistemic Status

**Max attainable: exact** for "production clamps" (inspect the cited functions) and for "depth ≥ 2 cross-face mismatch was measured" (PROBE 2 numbers on #form-cellid-chunk-patch FE(4); re-quote only under that probe's era or re-run).

**Currently `exact` as observation of present code** (direction resample + residual uplift clamp, verified 2026-07-31 against `from_surface_at` and `uplift_rate_tile_at`) **and of the already-recorded geometric mismatch measurement.** The *normative* claim that the policy must enter the key remains `robust-qualitative`. Stage `draft`.

## Discussion

The failure mode this segment blocks: calling the planet "exchange-repaired" after Jacobi adoption while face rims still run under an *undeclared* edge policy. Exchange repaired the *tile* family inside the face. Chart-edge is a different family: the height path moved to direction resample, and the remaining lie is **under-key ambient policy** plus residual clamp sites — not "rims still always rim-repeat."

## Working Notes

- **Key shape still owed (resample path landed):** extend `HaloSchedule` (or a sibling field) with an explicit `chart_edge: Clamp | Resample{…}` so two worlds that differ only there cannot share a key. Until then, the *policy difference* is ambient and invisible to content-addressing — the same class of under-key risk #form-complete-content-addressed-key exists to prevent.
- **Residual clamp site:** `uplift_rate_tile_at` still clamps data off-chart; fluvial heights do not. A consumer that assumes both share one edge policy is wrong today.
- **chunk.rs default-0 out-of-face** remains a separate specimen (Dirichlet-0 mass sink) on the patch loader path; do not merge that crime with the fluvial edge policy without a dual-arm measure.
