# vivarium-iso-voxels

A deliberately-simple **orthographic isometric** view over the **real voxel world**.
The successor to the retired memoized-tile navigator (see `archive/iso-tiles`): no
custom LOD, no memoization, no hand-rolled rasteriser. An ortho camera at the iso
angle, `bevy_voxel_world` meshes actual cubes from `vivarium-core`, and the GPU
depth buffer handles occlusion correctly (the pawn is never clipped by
lower-but-nearer terrain — the failure that killed the tile approach).

## Run

```bash
cargo run --release -p vivarium-iso-voxels
```

Loads through the shared worldgen cache (instant when warm). `VIVARIUM_REGION_HALF`
shrinks the world for a quick first generation.

### Controls
| | |
|---|---|
| WASD / arrows | pan |
| mouse wheel | zoom (orthographic scale) |
| Q / E | rotate 45° |

A red ~2 m cube (the pawn) stands on the focus cell. The ortho camera floats back
at the iso angle; `bevy_voxel_world` streams chunks around it.

## State (2026-06-30, early)

Works: real voxel cubes render in ortho-iso over the 12 km world, pan/zoom/rotate,
correct GPU occlusion. **Rough:** colors are the drab default texture array (flat
per-material palette — stone/dirt/grass/water — not wired yet); framing is oblique;
the pawn is small. All polish, deferred until the approach is judged worth it.

Known knobs to revisit: `STANDOFF` (eye distance — must keep the focus inside the
spawn sphere since streaming follows the camera), `spawning_distance`, the iso
pitch. The streaming-follows-camera coupling is the main constraint on framing.
