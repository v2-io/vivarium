# Godot voxel-view benchmark

A repeatable instrument for measuring the Godot (`VoxelLodTerrain`) view's
streaming/LOD cost, built after a long stretch of tuning-by-feel made it
impossible to tell a regression from "noticing an old artifact" from a real
improvement. **The point: "better" is now a number, not a memory.**

## How it works

`main.gd` (with `VIVARIUM_BENCH=1`) flies a *fixed, deterministic* camera path —
cruise, a hard 180° whip-around (the motion that exposed the LOD churn), then
cruise — starting at the highest interior land point and clamped above the
waterline, so every run traverses the same terrain and is directly comparable.
It logs one telemetry row per 0.5 s to `user://telemetry<tag>.csv` and grabs a
screenshot every few seconds.

```sh
./run.sh <config-name> <repeats> [ENV=VAL ...]   # archives CSVs into bench/<name>/
./run.sh baseline 3
./run.sh vd16k 3 VIVARIUM_VIEWCAP=16384
python3 analyze.py baseline vd16k vd08k          # steady-state medians + spread
```

Telemetry columns: `t, fps, worst_ms, draw_calls, mesh_blocks, data_blocks,
q_total, q_mesh, q_gen, dropped_mesh, blocked_lod, vmem_mb`. Screenshots are
git-ignored (bulky, regenerable); the CSVs are the durable record.

## Noise floor (read before trusting any comparison)

3 identical baseline runs, **machine under other load** (so an upper bound):

| metric | run-to-run spread | verdict |
|---|---|---|
| **fps** | ±8% (mean ±3%) | **trust** |
| **data_blocks** | ±7% | **trust** |
| worst_ms | ±11% | usable |
| mesh_blocks, q_total, draw_calls, vmem | ±20–39% | noisy — large effect or replicate only |

A config difference is only believable if it clears these bands. `analyze.py`
flags `fps` and `data_blocks` as the trusted metrics.

## Findings (2026-06-23)

### 1. `view_distance` dominates everything — and was set far too high

The octree fills its whole sphere with **no frustum/occlusion culling** (sourced:
zylann docs), so cost scales with sphere volume. Cost curve, 3 runs each:

| view_distance | fps | data_blocks | worst frame | notes |
|---|---|---|---|---|
| 32768 (16 km) | 13 | 442k | 78 ms | old default — painful |
| **16384 (8 km)** | **106** | **140k** | 12 ms | **8× faster, still spans the land** |
| 8192 (4 km) | 145 | 89k | 7 ms | reach too short, lands clip |

The continent is ±12 000 voxels from centre, so 16384 still spans the whole
landmass from a central vantage; 32768 was reaching ~20 km — *past the coast into
empty ocean* — paying 8× the framerate to render nothing. **Default changed
32768 → 16384.** Deltas (+715% fps, −68% data_blocks) clear the noise floor by
~700×; visually confirmed rich (see `vd16k/run_1_t24.png`). This single knob,
my own over-aggressive choice, was the root cause of the session's poor
performance.

### 2. Vertical view ratio is a weak lever here

`VIVARIUM_VRATIO=0.3` moved fps +8% / data_blocks −5% — **within** the noise
floor. Reason: the world is 16 384 voxels tall but view_distance was 32 768, so
the vertical fill was already clamped by world height; trimming the ratio only
shaves a slice. The spherical cost is **horizontal**-dominated. Not worth a
default change.

### 3. Mesh block size 32 is a big further win — but touches an untested axis

At the new 16384 default, `VIVARIUM_MESH_BLOCK=32` (vs the engine's 16):

| metric | mesh_block 16 | mesh_block 32 | Δ |
|---|---|---|---|
| **fps** | 106 | **145** | **+37%** (clears noise) |
| draw_calls | 1338 | 377 | −72% |
| worst frame | 12 ms | 7 ms | better |

So view 16384 + mesh_block 32 ≈ **145 fps — 11× the original baseline** — with no
visible terrain-quality regression (`mb32_vd16k/run_1_t24.png`). Fewer, larger
mesh tasks → far fewer draw calls.

**Not defaulted, on purpose.** Mesh block size also sets how large an area
remeshes per *edit* (dig/place) — core to a voxel game, and the benchmark flies
but never digs. Changing a default on an axis the instrument doesn't measure is
exactly the mistake this campaign exists to avoid. Recommendation: try
`VIVARIUM_MESH_BLOCK=32`, confirm digging still feels responsive, then promote it
to default if good.

## What this means for the engine decision (Bevy vs Godot)

Not a finished verdict, but a real data point: Godot's `VoxelLodTerrain` reaches
the km horizon natively (which `bevy_voxel_world` needed a backdrop-mesh hack
for), **but** its loader is spherical-by-distance with no occlusion/frustum
awareness — so it loads everything behind you and behind mountains, and the only
public lever against that is a smaller `view_distance`. At a sane view distance
it's a smooth 100+ fps; the limitation bites only when you want extreme reach.
