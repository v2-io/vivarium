# De-novo audit: fidelity honesty (depiction)

**Date:** 2026-07-31. **Law:** `#form-fidelity-ladder` FE(7)–(9), DECISIONS[a-view-renders-the-physics-and-adds-no-terms].

## Verdict

| Path | Status | Evidence |
|---|---|---|
| Mesh / assemble_surface_tile | **HONEST (FE(8))** | `observe.rs` → `surface_at_carved` only |
| Seam ghost in explore pull | **HONEST** | `pull.rs` ghost → `surface_at_carved` |
| Change-paint baseline | **HONEST** | `prior_at_carve_level` (like with like) |
| Standing water | **HONEST** | region-level field, not drawn surface |
| HUD `tier_line` | **STALE / MISLEADS** | Still claims "prior detail re-added" |
| pull.rs comments (tier census) | **STALE** | Same retired mechanism |
| lens.rs docs | **STALE** | Same |
| Uncovered cells | **NAMED RESIDUAL** | `surface_at_carved` falls to prior at *view* level |
| column_at / seeding surface_at | **OK if not depiction** | Seeding may use detail; depict must not |

## What L14-over-L13 lattice actually is (when FE(8) holds)

Stair-step / nearest-cell sample of the **carve grain** under a finer mesh — *absence visible*, not prior freckles. Tile seams are a separate physics residual.

## Actions

1. Rewrite HUD `tier_line` to FE(8) present tense.
2. Fix pull/lens comments.
3. Unit test: fine query over coarse carve — `surface_at_carved` ≠ `surface_at` (detail path) where detail is nonzero.
4. Segment WN: present-true HUD fixed this pass.
