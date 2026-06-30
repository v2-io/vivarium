# iso-tiles — a memoized-tile isometric navigator (spike)

**Status:** spike, in progress (2026-06-30). High-effort build. Standalone — does
not reuse the bevy-voxel / bevy-iso code; this is a different rendering thesis.

## The thesis (why this exists)

Joseph still hasn't seen anything beat a 16-year-old Flash isometric engine
(`~/src/neworld`) for *feel* and *performance*. Archaeology on it
(see the session notes) found the secret is not "it was 2D" — it is:

> **Per-cell appearance is a pure function of a small discrete local key, so it
> is memoized once and every frame is assembled by cheap blits.**

In neworld the key was `material colour + the 8 neighbour height-diffs`, with
heights quantized hard (`/45`, `>>1`). The space of distinct tile appearances is
therefore *small and finite*: the expensive drawing happens once per unique local
configuration; everything after is `copyPixels`. Cost is **O(screen pixels),
independent of world size** — the property that made it scream.

Two more things made it *feel* good, both reproduced here:
- **Continuous relief:** each tile's corners are displaced by the *averaged*
  neighbour heights, so adjacent tiles share corner heights and meet seamlessly —
  no popping, no hard grid. (There is **no fog** in neworld — the archaeology
  confirmed it; the soft "atmospheric" feel is this continuity + per-facet
  shading + height-banded colour. We may add real depth haze later, but the feel
  comes first from continuity.)
- **45° pivots:** neworld alternated two hand-coded tile shapes (iso diamond /
  dimetric) across 8 orientations. We get all 8 *for free* from a real
  orthographic projection at `yaw = orientation·45°` — the tile's parallelogram
  shape simply falls out of the math, no special-casing.

## What this is

An overhead isometric **navigator** over a `vivarium-core` world — the
"strategic/abstract" end of the detail→abstract view ladder (DESIGN.md). It is
*surface-only* (top of each column) with a **Dwarf-Fortress-style Z-slice** for
verticality, which is the right trade for a map view and is exactly what neworld
did (no overhangs in this view; the walk/voxel views own true 3D).

It renders the way neworld did, modernized:
- A single **framebuffer** `Image` the size of the window; one fullscreen sprite
  shows it. The frame is rebuilt **only when the camera/slice changes**
  (event-driven, like neworld's `TimebaseEvent.RENDER`), not every frame.
- A **TileCache**: `key → rasterized RGBA tile`. On rebuild we walk the visible
  cells back-to-front (painter's order), get-or-rasterize each cell's tile, and
  alpha-blit it into the framebuffer. The cache *is* neworld's `RenderedTilePool`.

## Key design choices (each a feature, not a compromise)

- **Discrete zoom levels.** The memoization needs fixed tile sizes; RimWorld-style
  zoom steps feel fine (arguably better). Each level = a `cell` stride in voxels
  (1 m … 32 m at detail 2) and a pixel scale.
- **8 discrete orientations.** `yaw = o·45°`; world sampling basis + tile shape
  come from the projection. Rotation re-keys the cache (shape changes), so we
  clear the tile cache on orientation/zoom change to bound memory.
- **Hard height quantization.** Cell heights are bucketed into bands; corner
  displacements come from averaged *banded* neighbour heights. This is what keeps
  the tile vocabulary finite — the whole performance story. Bands are clamped so
  the key space is bounded.
- **Surface + simple walls.** Each cell is a top quad (corners displaced for
  continuity) plus the two front-facing side walls down to the lower neighbours
  (the "column"), flat-shaded by face. Cliffs read; caves do not (Z-slice
  instead).

## Instrumentation (the thesis is a measurement)

A HUD reports: visible tiles composited, **unique tiles rasterized (cache size)**,
cache hit-rate, framebuffer rebuild time (ms), and FPS. The claim "cost is
independent of world size, because the tile vocabulary is small" is only honest
if we *show* the cache staying small while tiles-drawn is large. If the cache
explodes, the quantization is too fine — that is the dial to watch.

## World source

Loads `Volume` through the **shared worldgen cache** (same scheme as the godot
bridge / bevy-iso), so it reloads Joseph's already-cached 12 km world instantly.
Surface sampling per cell: `surface_height` (band), top `voxel` material,
`water_depth_voxels` / `water_speed` for the water layer.

## Phases
- **P1 (this build):** framebuffer + memoized tiles + continuous relief + flat
  face shading + walls + pan + discrete zoom + 8-orientation rotate + Z-slice +
  instrumentation. Prove cheapness + reproduce the feel.
- **P2:** per-facet 8-triangle Gouraud relief (neworld's exact top shading);
  water shading by depth/flow; optional depth haze.
- **P3:** agents/markers layer; selection/picking; the detail→tactical hand-off.

## Explicitly out of scope
The agent layer (the real bet) — this is a view. Timeboxed accordingly.
