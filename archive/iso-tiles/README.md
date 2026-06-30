# vivarium-iso-tiles — ARCHIVED (2026-06-30)

> **Retired.** The memoization thesis held (cost independent of world size, tile
> vocabulary in the hundreds), but a smoothed heightfield-tile look was a
> **non-starter aesthetically** — "neither fun nor interesting" (Joseph). Kept for
> the record. The successor renders *real voxel cubes* in an ortho-iso camera and
> lets the engine do the work (no custom LOD/memoization). What survives from here:
> the worldgen cache (now in the Godot path), the iso projection + 8-orientation
> math, and the pawn-marker idea.

---

# vivarium-iso-tiles

A memoized-tile **isometric navigator** over `vivarium-core` — the strategic/map
end of the view ladder. Reconstructs the secret of a 16-year-old Flash engine
(`neworld`): per-cell appearance is a pure function of a small discrete key, so it
is **memoized once and frames are assembled by cheap blits** — cost is O(screen
pixels), independent of world size. See [`SPEC.md`](SPEC.md) for the full rationale.

## Run

```bash
cargo run --release -p vivarium-iso-tiles
```

Loads the world through the **shared worldgen cache** (same file the godot/bevy
views use), so an already-generated world reloads instantly. `VIVARIUM_REGION_HALF`
shrinks it for a quick first generation.

### Controls
| | |
|---|---|
| WASD / arrows | pan |
| mouse wheel | zoom (6 discrete levels, 1 m → 32 m cells) |
| Q / E | rotate 45° (8 orientations: 4 iso + 4 dimetric) |
| `[` / `]` | Z-slice down / up |
| `\` | clear Z-slice |

A red ~2 m **marker block** stands on the focus cell (the pan/rotate pivot),
depth-sorted with the terrain. The HUD reports fps, rebuild ms, tiles drawn, and —
the load-bearing number — **unique tiles (cache)** and hit-rate.

## How it works (one screen)

- **Framebuffer**: one window-sized image, rebuilt only when the view changes
  (event-driven, like neworld). One fullscreen sprite shows it.
- **TileCache**: `key → rasterised RGBA tile`. A rebuild pre-samples the visible
  cell rectangle once, builds each cell's memoized tile (top face + column walls),
  and composites back-to-front by blitting.
- **Continuity**: tiles anchor at *banded* heights and share neighbour-averaged
  corners, so they meet seamlessly; **column walls** drop to each lower neighbour's
  true height to close risers.
- **All 8 orientations** come from a real ortho projection at `yaw = o·45°` — the
  tile parallelogram shape just falls out, no per-orientation special-casing.

## State (2026-06-30)

Working: continuous blocky terrain over the real 12 km eroded world, water +
coastline, 8-orientation rotate, discrete zoom, Z-slice, the marker block, ~100+
fps at a few ms/rebuild. Verified the thesis: the tile vocabulary stays small
(hundreds, not the 8881 cells drawn), which is *why* it's cheap.

Rough edges / dials: 3 m height banding gives a terraced look; the `drops` field
(true cliff walls) raises vocabulary — coarsen `BAND_VOX` / tint / `MAX_SLOPE` to
trade fidelity for a smaller, neworld-tiny vocabulary. `VIVARIUM_TILES_FLAT=1` is a
flat-tile debug mode. P2: per-facet Gouraud top shading + flow shading; P3:
agents/picking. The agent layer (the real bet) is out of scope — this is a view.
