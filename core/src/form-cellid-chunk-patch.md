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
4. **Halos carry the seams.** Neighbour values are materialized into a ghost border at patch load, so the inner loop never branches (domain decomposition + halo exchange). **Live:** same-face halo fill (`chunk.rs::fill`). **Designed, unbuilt:** cross-cube-face halo fill with the face axis transform — `chunk.rs` leaves out-of-face halo cells at default and names the loader as the owner. A halo is a *read* surface; it does not make neighbour state a mutable coupling channel ( #form-column-control-volume seam duty still binds).
5. **`CubeCoord{face,u,v}` is a view, never a key.** The `f64` cube coordinate remains the math/conversion form (projection, lat/lon, rendering); identity lives only in `CellId`.
6. **Hexagonal tiling: considered and declined (2026-07-03).** Hexes do not nest (no exact coarse = union-of-children, which the sufficient-statistic ladder needs), do not tile the sphere without 12 pentagons, and privilege the vertical axis in 3-D. Isotropy is purchased where it bites (routing weights, diagonal pipes) rather than by re-tiling. A hex convention stays open for view-side quantization only ( #form-grid-equiangular-staggered keeps the base-grid verdict).

## Epistemic Status

**Max attainable: exact** as the address architecture in force. **Currently `exact`:** `CellId` and the patch/halo structure are live code (`sphere.rs`, `chunk.rs`); the 80× / ~6 Gcells/s figures are the recorded bench rationale (`ref/research/spatial-key-bench.md` + `.rs`). The hex declination is design-recorded reasoning (DESIGN-MATERIAL §8), not a Joseph DECISIONS row — the *base grid* authority chain lives on #form-grid-equiangular-staggered (Joseph-tentative keep), and this segment does not extend it.

**Known incomplete:** cross-cube-face halo fill (FE(4)) is designed, not built — same-face fill only; the two-tier chunk (macro streaming container) is thinner in code than the patch tier; face-keyed flux objects ( #form-grid-equiangular-staggered FE(2)) are not yet store citizens; the ~20 km livable-shell vertical carve is design prose, not a built boundary.

Stage `draft`.

## Discussion

This is the machine-side complement of the grid formulation: the grid segment says what the lattice *is*; this one says how it is *addressed and traversed* without paying curve arithmetic in kernels or float drift in identity. The bench-driven split — identity by curve, work by Cartesian window — is why "Hilbert-ordered" and "6 Gcells/s stencils" are not in tension.

## Working Notes

- **Dual-home demote:** DESIGN-MATERIAL graduated (address block → this segment); bench rationale stays in `ref/research/spatial-key-bench.md`.
- Residual: promote closed-form spherical cell area (`msc/spike-wavelet-store/src/area.rs`) into `measure.rs` per #obs-cube-locked-kernel-bias trail — same wiring wave as face keys.
