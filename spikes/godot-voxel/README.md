# Godot voxel-view spike

One of two throwaway view adapters (Bevy vs Godot) over `vivarium-core`, built to
settle the engine question empirically. See `DESIGN.md` and the
`spike/voxel-view` branch. The simulation is Rust; this project is only a way of
*looking* at it, loaded through a GDExtension bridge (`crates/vivarium-godot`).

## Running

```bash
# 1. Build the Rust bridge and copy the cdylib into bin/.
./sync-lib.sh                 # debug; `./sync-lib.sh release` for release

# 2. First time only (per machine): let Godot scan the .gdextension so the
#    VivariumWorld class registers. This writes .godot/extension_list.cfg.
godot --headless --editor --quit --path .   # may print a MoltenVK backtrace on
                                            # macOS headless — harmless; the
                                            # extension list is still written.

# 3. Run it.
godot --headless --path . --quit-after 3    # prints the bridge self-check
godot --path .                              # windowed (for the eventual walk)
```

If you change Rust code, re-run `./sync-lib.sh` and restart Godot. `reloadable`
is on in `vivarium.gdextension`, so the editor can hot-reload in many cases.

## State

- **Bridge (read + write seam): working.** `VivariumWorld` exposes
  `voxel_at`, `surface_height`, `dig`, `place`, `step`, `agent_count`,
  `agent_position` to Godot, and `ready()` prints a live-core self-check.
- **Not yet:** godot_voxel terrain rendering from core, first-person controller,
  raycast dig/place, the screenshot+perf harness.

## Layout

| Path | What |
|------|------|
| `project.godot` | Godot project config |
| `vivarium.gdextension` | tells Godot to load the Rust cdylib from `bin/` |
| `main.tscn` | scene whose root is the Rust `VivariumWorld` node |
| `sync-lib.sh` | build the bridge + copy the cdylib into `bin/` |
| `bin/` | the copied cdylib (git-ignored, a build product) |
