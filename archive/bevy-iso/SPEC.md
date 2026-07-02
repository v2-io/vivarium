# bevy-iso — isometric/orthographic surface renderer (spike)

**Status:** spike, in progress (2026-06-30). The question it exists to answer is
below; nothing here is settled.

## Why this exists

Two earlier findings collide into one experiment:

1. **The voxel-octree path over-renders.** Both view adapters so far drive a
   chunked voxel octree (`bevy_voxel_world` / `godot_voxel`) that streams a
   *sphere* of blocks by distance, with no frustum or occlusion culling. For an
   overhead view that is enormous waste: an orthographic camera's visible set is
   a **bounded box** — the screen rectangle extruded along the view axis,
   intersected with a thin slab around the surface — not an expanding frustum.
   Owning the renderer lets us draw *only* that.

2. **Godot felt more responsive than Bevy — but probably not for the reason the
   engine decision assumed.** Joseph's read (2026-06-30): most of Godot's edge
   was its *mature LOD system decoupled from interaction* — the camera stays
   smooth even when the data being streamed in is just as slow, because loading
   never blocks the interaction loop. That is a property of the *adapter*, not
   the engine. So a fair Bevy-vs-Godot comparison is not "whose voxel addon is
   better" but "which engine, with a renderer we write ourselves, gives a smooth
   decoupled overhead view." This spike builds the Bevy half of that.

The engine decision (`spikes/FINDINGS.md` → Bevy) was made on the *addon*
comparison. This spike deliberately re-opens the question on the fairer footing,
and may confirm or overturn it.

## The load-bearing principle: interaction is decoupled from loading

This is the spine. Everything else serves it.

- **Main thread, every frame, never blocking:** camera (pan / zoom / rotate /
  focus-follow), visible-set computation, and *swapping in* chunk meshes that
  have finished building elsewhere. It must never sample the world in bulk or
  wait on a mesh.
- **Background (`AsyncComputeTaskPool`):** all `vivarium-core` sampling and mesh
  construction. A chunk task takes a clone of `Arc<Volume>`, samples it, and
  returns raw mesh data; the main thread turns that into a `Mesh` asset and
  spawns the entity when it arrives.
- **Consequence:** a frame draws whatever meshes currently exist. A chunk not yet
  built is simply absent or shown at a coarser LOD; the camera keeps moving at
  display rate regardless. *That* is the smoothness Godot got for free from a
  mature decoupled LOD system, made explicit.

If the camera ever stutters because a chunk is generating, the spike has failed
its own thesis — that is the thing to watch.

## Design

### Data source
`Arc<Volume>` from `vivarium-core`, obtained through the **same worldgen cache**
the godot bridge uses (`Volume::to_bytes`/`from_bytes`, keyed by params +
`WORLDGEN_VERSION`, under `$TMPDIR/vivarium-worldcache`). Matching the bridge's
parameters means this app *reloads the same frozen world* the godot view uses —
no regeneration, and a live demonstration of the engine-agnostic core/view wall.
`VIVARIUM_REGION_HALF` etc. mirror the bridge for fast-iteration small worlds.

### Camera
Orthographic, true-iso tilt (≈35.26°), four 90° snap yaws, a focus point that
rides the ground. Pan (keys + drag), zoom (`scale`/viewport-height), rotate.
Identical model to the godot `navigator.gd`, so the two are directly comparable.

### Chunks + progressive LOD
- World partitioned into fixed square chunks (voxels). A chunk's **LOD** is a
  sample stride `2^lod`, chosen by zoom: zoomed in → fine, out → coarse. Coarse
  appears first and refines as finer tasks land — mirroring core's existing
  `lod`-stride point-sampling.
- **Visible set:** from focus + zoom (+ yaw), the chunks covering the screen plus
  a one-chunk margin ring for pan headroom. v1 approximates the screen
  parallelogram with a radius `≈1.3·zoom` square (still **bounded by zoom**, the
  whole point — resident chunk count scales with zoom², not with the world); the
  exact screen-rectangle cull is a later refinement.
- Chunks outside the set are despawned; their tasks, if still pending, are
  dropped.

### Mesh
Per-chunk heightmap mesh sampled from `surface_height`, central-difference
normals, per-vertex material colour — the proven `spawn_far_terrain` pattern from
the bevy-voxel spike, now one mesh per chunk at the chunk's stride. v1 is a smooth
heightmap; stepped voxel-top faces (to keep the blocky identity) are a refinement.

### Phases
- **P1 (this commit):** ortho iso camera + decoupled async chunk meshing of the
  terrain surface with zoom-driven LOD. Prove the smoothness thesis + bounded
  visible set.
- **P2:** water as a second translucent layer from `water_depth_voxels` /
  `water_speed`.
- **P3:** height-slice cutaway — trivial in our own mesher (cap heights at
  `slice_y`); the thing the octree addon could not do cleanly.

## What this spike does NOT settle
- Final aesthetic (smooth vs blocky vs textured).
- Whether Bevy's overhead view actually *feels* as smooth as Godot's — that is
  the empirical question; this is the instrument, not the verdict.
- Anything about the agent layer (DESIGN.md axis 3), which remains the real bet.
