---
slug: form-cellid-chunk-patch
type: formulation
status: exact
stage: draft
depends:
  - form-grid-equiangular-staggered
  - form-complete-content-addressed-key
  - form-column-control-volume
---

# CellId is the canonical key; the curve orders chunks, patches stay Cartesian

The Hilbert-ordered cube-sphere `CellId(u64)` is the one canonical spatial address — for column identity, memo keys, store roots, and LOD. The space-filling curve orders **containers**, never hot loops: a patch's interior is a plain Cartesian array with a halo, so per-cell curve arithmetic never happens.

## Formal Expression

1. **Canonical key.** `CellId(u64)` — S2-style packing of (face, level, Hilbert distance); 6 faces, quadtree per face, levels to ~25 (≈ 0.5 m footprint). Exact, drift-free (no far-lands float origin), hashable, and Hilbert-ordered so a region **within a face** is a contiguous id range (storage/streaming locality; the curve is plain per-face — S2's cross-face continuity is deliberately not carried). Parent = bit-shift, so the LOD ladder is structural. It is the spatial component of every complete content-addressed key ( #form-complete-content-addressed-key ). Live: `sphere.rs`.
2. **The curve orders chunks, not cells.** The Hilbert curve's job is container ordering. A **patch** — the automata/storage working unit — has a plain Cartesian row-major interior where neighbours are `idx±1`. Measured basis (spatial-key bench; the bench's own caveat: ratios are the durable result, absolutes are machine-local): Cartesian neighbours ~80× faster than Hilbert-id neighbour arithmetic; the stencil ran ~6 Gcells/s on the bench machine while a patch fits cache. Curve encode/decode is designed out of every hot loop.
3. **Two size tiers (design).** A coarse macro/streaming **chunk** (tens of km — LOD container) and a fine automata **patch** at the cache sweet spot (~0.25–0.5 km, 256²–1024²). **Live:** patch + same-face halo (`chunk.rs` `Patch`). **Macro chunk tier** is thinner in code than the design pair — design residual, not a second live citizen.
4. **Halos carry the seams.** Neighbour values are materialized into a ghost border at patch load, so the inner loop never branches (domain decomposition + halo exchange). **Live:** same-face halo fill (`chunk.rs::fill`). **Cross-cube-face fill: spiked and measured, not built.** The face axis transform needs no adjacency table — the `to_unit`/`from_unit` projection round-trip *is* the transform, verified as a bit-exact involution on every edge cell at L4/L6/L8 (64/64, 256/256, 1024/1024), with genuine adjacency geometry (cross/within distance ratio $\in [0.87, 1.41]$) and a known-bad clamp control that collapses to $\approx 0$ distance, so the check is not vacuous. Correspondence is **exact at depth 1 only**: the two cross-face grids co-align only on the shared edge, and a depth-2 halo already mismatches on 200/256 edge cells (78%; 232/256 by depth 4, worst at cube corners) — so any $d \ge 2$ cross-face fill requires a **declared resampling**, which nothing declares today ( #form-same-level-halo-exchange carries the design-side consequence). Meanwhile the shipped default-0 out-of-face halo is a **convicted physical defect**, not a cosmetic TODO: a Dirichlet-0 mass sink with linear-growing drift ($-4.1{\times}10^{-14} \to -4.1{\times}10^{-11}$ over $1 \to 1000$ steps) and a manufactured $\sim$121 m seam cliff that plateaus across L6/L8/L10 while a filled halo tracks the within-face step ($0.507 \to 0.032$ m) — a discontinuity, not physics (`chunk.rs:74` still ships it; cross-face spike PROBEs 1/2/4a/5). A halo is a *read* surface; it does not make neighbour state a mutable coupling channel ( #form-column-control-volume seam duty still binds).
5. **`CubeCoord{face,u,v}` is a view, never a key.** The `f64` cube coordinate remains the math/conversion form (projection, lat/lon, rendering); identity lives only in `CellId`.
6. **Hexagonal tiling: considered and declined (2026-07-03).** Hexes do not nest (no exact coarse = union-of-children, which the sufficient-statistic ladder needs), do not tile the sphere without 12 pentagons, and privilege the vertical axis in 3-D. Isotropy is purchased where it bites (routing weights, diagonal pipes) rather than by re-tiling. A hex convention stays open for view-side quantization only ( #form-grid-equiangular-staggered keeps the base-grid verdict).

## Epistemic Status

**Max attainable: exact** as the address architecture in force. **Currently `exact`:** `CellId` and the patch/halo structure are live code (`sphere.rs`, `chunk.rs`); the 80× / ~6 Gcells/s figures are the recorded bench rationale (`ref/research/spatial-key-bench.md` + `.rs`). The hex declination is design-recorded reasoning (DESIGN-MATERIAL §8), not a Joseph DECISIONS row — the *base grid* authority chain lives on #form-grid-equiangular-staggered (Joseph-tentative keep), and this segment does not extend it.

**Known incomplete:** cross-cube-face halo fill (FE(4)) is spiked-and-measured, not built as true neighbour-face geometry — same-face fill only on `chunk.rs`; the depth-$\ge$2 resampling choice FE(4) names is undeclared. **Fluvial production path (era `1b028c3`):** multi-cell Jacobi windows that leave the chart **clamp** to face indices rather than panic ( #obs-chart-edge-halo-clamps-to-the-face ) — a chart-edge boundary contract in force, not the principled resample FE(4) still owes. The two-tier chunk (macro streaming container) is thinner in code than the patch tier; face-keyed flux objects ( #form-grid-equiangular-staggered FE(2)) are not yet store citizens; the ~20 km livable-shell vertical carve is design prose, not a built boundary.

Stage `draft`.

## Discussion

This is the machine-side complement of the grid formulation: the grid segment says what the lattice *is*; this one says how it is *addressed and traversed* without paying curve arithmetic in kernels or float drift in identity. The bench-driven split — identity by curve, work by Cartesian window — is why "Hilbert-ordered" and "6 Gcells/s stencils" are not in tension.

## Working Notes

- **Dual-home demote:** DESIGN-MATERIAL graduated (address block → this segment); bench rationale stays in `ref/research/spatial-key-bench.md`.
- Closed-form spherical cell area is promoted (`measure.rs::cell_area_m2`, from the wavelet spike) and wired into the live fluvial kernel; the edge-length/arm helpers deliberately remain in `msc/spike-wavelet-store/` awaiting the staggered-FV router ( #form-grid-equiangular-staggered WN owns that parts-shelf).
- The cross-face spike's pre-registered P4a predicted relative mass loss $10^{-2}..10^{-1}$ and measured $4{\times}10^{-11}$ — right sign and shape, nine orders off on magnitude (the probe's diffusion flux is tiny against total mass). Recorded as a probe-sensitivity specimen; the sink's *linearity in steps*, not its magnitude, is the conviction.
