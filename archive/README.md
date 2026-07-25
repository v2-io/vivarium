# archive/

Work preserved for reference, not part of the active build.

## The early view spikes (pre-engine-decision era, 2026-06)

`tilemap/`, `topdown/`, `bevy-iso/`, `iso-tiles/`, `iso-voxels/`, `bevy-voxel/`
— the ladder of 2-D/isometric/voxel view experiments over `vivarium-core` that
preceded and informed the engine decision. Each carries its own README/SPEC;
`iso-tiles` self-declares its retirement. Superseded as views by
`spikes/worldview` (and `spikes/globe`); kept as reference for the rendering
lessons recorded inside them.

## `globe-spike/` + `worldview-spike/` (retired 2026-07-24)

The two Bevy view spikes over `vivarium-world`, retired when
`crates/vivarium-explore` replaced them. Kept because `globe-spike` in
particular is a good reference and several of its comments record bugs that cost
a live sighting to find — the inverted winding probe (the globe rendered as its
own far shell, seen from inside), the drag-sign history behind it, and the
half-cell ghost sampling that avoids the `from_unit` edge tie. Those comments
travelled into `crates/vivarium-explore/src/mesh.rs` verbatim; the code did not.

What did **not** travel, deliberately:

- `globe-spike`'s deep-time warmer called `World::epoch_reduction`, which on a
  miss computes **and puts** — a view authoring store citizens
  ( #form-core-view-wall ). The explorer opens the store read-only and counts
  its refused writes on the HUD.
- `worldview-spike`'s `VIVARIUM_ALLOW_VIEW_EVOLUTION=1` path let the view own
  epoch counts and run erosion/water workers itself — the FE(4) compliance debt
  that segment records. The explorer has no such path and no such flag.

Both still build if re-added to the workspace `members`, against whatever
`vivarium-world` looks like at that point.

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

## `slabs/` + `vivarium-app/` (archived 2026-07-24)

Both were live workspace members until the explorer consolidation. `slabs/` is the
stacked level-slab 3-D terrain experiment (last touched 2026-07-01); `vivarium-app/`
is the 2-D debug view (last touched 2026-06-22). Both render over **`vivarium-core`**,
the legacy pre-frame crate, not over `vivarium-world` — which is why neither could
grow into the principled explorer: they observe a substrate the world no longer
runs on. Kept because `vivarium-core` remains the provenance for the ported
hydrology and fluvial kernels, and these are its only worked view examples.
