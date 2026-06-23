# Godot voxel-view spike

One of two throwaway view adapters (Bevy vs Godot) over `vivarium-core`, built to
settle the engine question empirically. See `DESIGN.md` and the
`spike/voxel-view` branch. The simulation is Rust; this project is only a way of
*looking* at it, loaded through a GDExtension bridge (`crates/vivarium-godot`).

## Running

```bash
# 1. Fetch the godot_voxel addon (~105 MB, git-ignored, pinned by sha256).
./fetch-voxel-addon.sh

# 2. Build the Rust bridge, copy + re-sign the cdylib into bin/.
./sync-lib.sh                 # debug; `./sync-lib.sh release` for release

# 3. First time only (per machine): let Godot scan the .gdextensions so the
#    VivariumWorld + Voxel* classes register (writes .godot/extension_list.cfg).
godot --headless --editor --quit --path .   # may print a MoltenVK backtrace on
                                            # macOS headless — harmless.

# 4. Run it (windowed).
godot --path .
```

Controls: WASD move · mouse look · Space/Shift up/down · Ctrl = fast ·
**LMB dig · RMB place** · Esc frees the mouse.

If you change Rust code, re-run `./sync-lib.sh` (it re-signs — see Findings) and
restart Godot.

### Env knobs

| Var | Default | Effect |
|-----|---------|--------|
| `VIVARIUM_DETAIL` | `4` | voxels per world unit; higher = finer voxels (see Findings) |
| `VIVARIUM_VIEWCAP` | `512` | max voxel view distance (physical view = this ÷ detail) |
| `VIVARIUM_AUTOSHOT` | unset | if set: build scene, carve a test crater, screenshot to `user://terrain_shot.png`, quit. For agent verification. |

## State

- **Full pipeline working:** core → gdext bridge → godot_voxel threaded chunk
  generation → VoxelMesherCubes → rendered Minecraft-like terrain, first-person
  fly + raycast dig/place, edits persisted in core and remeshed live.
- **Resolution:** runtime `detail` (voxels/unit), default 4.
- **LOD working:** `VoxelLodTerrain` renders distant terrain coarse, so the view
  reaches 2048 voxels (512 physical units at detail 4) at a vsync-pegged 120 FPS
  — ~4× the non-LOD reach. The generator's `lod` arg threads into
  `generate_block`, which point-samples core at stride 2^lod: **view resolution
  is decoupled from intrinsic resolution.** (Edits don't yet propagate into the
  coarse far view — out of scope for now; see DESIGN.md detail→abstract.)
- **Not yet:** agent visuals, the Bevy half of the comparison, the written
  findings doc.

## Findings (the things that cost time — read before changing the view)

- **Re-sign the dylib after copying (Apple Silicon).** Overwriting a loaded,
  code-signed `.dylib` in place invalidates its signature and the kernel
  SIGKILLs Godot on `dlopen` ("Code Signature Invalid"), with *no* output.
  `sync-lib.sh` does `rm` + `cp` + ad-hoc `codesign` to avoid this.
- **gdext needs `experimental-threads`** because godot_voxel generates on worker
  threads. The bridge holds the `World` behind a `RwLock` and every method takes
  `&self`, so concurrent reads (generation) and a write (dig) never alias.
- **Do NOT scale the VoxelTerrain node.** A non-identity transform breaks its
  streaming/rendering — terrain never appears. So 1 voxel == 1 Godot unit at all
  resolutions; a finer world is a physically larger one, viewed as a local
  bubble. The camera/movement work in voxel space and scale with `detail`.
- **LOD: use `VoxelLodTerrain`, not `VoxelTerrain`.** The plain grid
  (`VoxelTerrain`) has no LOD, so view distance is a hard perf wall (a large
  view at high detail hangs the engine). `VoxelLodTerrain`'s octree meshes far
  terrain coarse and — despite the docs leaning smooth/Transvoxel —
  `VoxelMesherCubes` works with it. Gotcha: `VoxelLodTerrain` rejects
  `material_override`; use `terrain.set_material(mat)` (single arg).

## Layout

| Path | What |
|------|------|
| `project.godot` / `vivarium.gdextension` | project + Rust-bridge extension config |
| `addons/zylann.voxel/` | godot_voxel addon (git-ignored; `fetch-voxel-addon.sh`) |
| `main.tscn` / `main.gd` | scene built in code (terrain, palette, camera, light) |
| `generator.gd` | custom VoxelGenerator sourcing each chunk from core |
| `player.gd` | first-person fly camera + raycast dig/place |
| `sync-lib.sh` / `fetch-voxel-addon.sh` | build+sign the bridge / fetch the addon |
| `bin/` | the copied cdylib (git-ignored, a build product) |
