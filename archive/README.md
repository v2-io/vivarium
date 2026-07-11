# archive/

Work preserved for reference, not part of the active build.

## The early view spikes (pre-engine-decision era, 2026-06)

`tilemap/`, `topdown/`, `bevy-iso/`, `iso-tiles/`, `iso-voxels/`, `bevy-voxel/`
— the ladder of 2-D/isometric/voxel view experiments over `vivarium-core` that
preceded and informed the engine decision. Each carries its own README/SPEC;
`iso-tiles` self-declares its retirement. Superseded as views by
`spikes/worldview` (and `spikes/globe`); kept as reference for the rendering
lessons recorded inside them.

## `godot-voxel/` + `vivarium-godot/`

The Godot half of the engine spike (2026-06-22). A full, working voxel view over
`vivarium-core` via a `gdext` Rust bridge: terrain, LOD (`VoxelLodTerrain`),
overcast + distance fog, SSAO, first-person fly + raycast dig/place. The
comparison chose **Bevy** — reasoning and the honest confounds are in
[`../spikes/FINDINGS.md`](../spikes/FINDINGS.md).

Kept because it's a complete, hard-won reference: it documents (in its README and
code) several Godot/gdext/Apple-Silicon potholes worth not re-paying — the
code-signing SIGKILL on dylib overwrite, the no-scale `VoxelTerrain` constraint,
`VoxelLodTerrain` rejecting `material_override`, the `experimental-threads` +
`RwLock` threading model.

To revive: re-add `archive/vivarium-godot` to the workspace `members` in the root
`Cargo.toml` (and fix its `vivarium-core` path), then follow
`godot-voxel/README.md`.
