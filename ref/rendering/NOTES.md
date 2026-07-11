# Rendering the world to the horizon — architecture decision

> Status: synthesis of two research sweeps, 2026-06-23, prompted by hitting the
> renderer wall in the Bevy spike (km-scale terrain only visible to ~256 m).
> **RATIFIED by Joseph 2026-06-23**: the staged hybrid below, starting at v1
> (self-built far heightfield backdrop), with the heightfield-far trade accepted
> for v1/v2. Sources + verification flags inline.
>
> **v1 IMPLEMENTED (2026-06-23).** `spikes/bevy-voxel` now renders near full-detail
> diggable voxels (`bevy_voxel_world`, spawn radius 16 ≈ 256 m) plus a self-built
> coarse far-terrain mesh (`spawn_far_terrain`) sampled from `surface_height` —
> the whole landform to the horizon, in-world first-person, deterministic, no new
> dependency. Walkable; you stand in an eroded valley and see the massif + coast
> behind it. **Open:** the near/far seam (blocky voxels vs smooth far mesh), a
> stepped/blocky far-shader + unified palette, and v2 (camera-centred clipmap
> rings) for unbounded/planet reach. v3 (volumetric far) only if distant
> overhangs ever matter.

## The problem, and why it is a *renderer* problem, not an *approach* problem

At the 0.5 m voxel anchor, a renderer that keeps **fixed-size chunks and only
decimates mesh detail** cannot reach a kilometre horizon: reach = chunk_count ×
chunk_size, and chunk_count is bounded by the frame budget. `bevy_voxel_world`
is exactly this — **verified in its source** (`chunk.rs`: `CHUNK_SIZE_U = 32`,
`Chunk::aabb()` always a 32-unit cube, LOD only varies voxel density *inside*
that fixed footprint; no octree/quadtree spatial merge). So ~32 chunks stutters
and the horizon is unreachable. This is an architectural ceiling of *that crate*,
not of voxels, not of Bevy, and not of our world model.

Every viable solution makes the **far cells themselves larger** (cover
exponentially more world per primitive with distance), not just coarser-meshed.

## The asymmetry that is ours alone (the key insight)

Our core is a **pure function** `voxel(x,y,z)` + sparse edits. So we never *cache*
a far field — we **regenerate it deterministically at coarse stride**. Every
Minecraft-derived system (Distant Horizons, etc.) was built to cache a far field
it *could not* cheaply recompute, paying with staleness, no-replay, and
"only-terrain-you-visited." We sidestep all of it. And our core already exposes
the exact reduction the far band needs: **`surface_height(x,z)` is O(1) analytic**
— a coarse far column is just that, sampled on a coarse lattice, with the surface
voxel's material. That *is* a first instance of the fidelity/conservation
invariant (doc/design/DESIGN.md): the far representation is a defined statistical summary of
the near voxels it stands in for.

## Options (both sweeps), scored against the docs' criteria

Criteria: core/view wall · determinism (bit-replay) · 3D-all-the-way-down +
diggable near · cheap-far/detailed-near fidelity invariant · beauty · Rust/Bevy ·
**must not become the sinkhole that starves the ASF agent layer** (doc/design/DESIGN.md's
central discipline — both agents independently flagged this).

| Option | Horizon reach | Far field | Near diggable | Determinism | Effort / risk | Verdict |
|---|---|---|---|---|---|---|
| **bevy_voxel_world alone** | ✗ (~256 m) | — | ✓ | ✓ | none | the wall; keep for *near* only |
| **Self-built coarse far heightfield** (from our field) | ✓ | heightfield | ✓ (near band unchanged) | ✓✓ (regenerated, nothing cached) | **low**, no new deps | **recommended v1** |
| **Geometry clipmaps** (Losasso-Hoppe; rings re-centred on camera) | ✓ unbounded | heightfield | ✓ | ✓✓ | medium, well-trodden | the v2 upgrade for planet/infinite reach |
| **bevy_terrain crate** | ✓ | heightfield | ✗ (backdrop only) | ✓ | **high** (Bevy 0.14-pinned; 0.18 port non-trivial, UNVERIFIED exists) | not turnkey; skip |
| **Octree/quadtree growing-chunk voxel LOD** | ✓ | **volumetric** | ✓ | ✓ | high (seams: Transvoxel / 0fps stable-rounding) | upgrade *if* heightfield-far proves insufficient |
| **GPU SVDAG/brickmap raymarcher** (Aokana arXiv 2505.02017) | ✓ | **volumetric** | ✓ (editing = known-hard) | ✓ | **highest** (from-scratch renderer, off Bevy's mesh path) | eventual target; NOT next — the budget sinkhole |

Rust references for the raymarch path if ever pursued: `voxelis` (Rust SVO-DAG,
rendering WIP), `VoxelHex` (ex-`shocovox`, WGSL SVO-brick). All UNVERIFIED for
Bevy 0.18.

## Recommendation — staged, disciplined

**Bevy stays.** The engine choice holds; only the *voxel-rendering approach*
changes, and it changes toward "our own view over our own field," which is
exactly the core/view-wall philosophy.

- **v1 — self-built far heightfield backdrop.** Keep `bevy_voxel_world` for the
  near, diggable band. Add one coarse 3D heightfield mesh of the region, built
  from `surface_height`, coloured by elevation/material, composited *behind* the
  near voxels. This is an **in-world 3D surface seen first-person** — you look
  toward an 8 km mountain and the mesh draws it — **not** a top-down map. For a
  finite ~12 km region this needs no clipmap machinery (one mesh, ~250 k tris).
  Lowest risk, no new dependency, fully deterministic, ships "see mountains and
  streams to the horizon" soon.
- **v2 — geometry clipmaps** when we want unbounded / planet-scale reach: the same
  field-sampled heightfield, but as camera-centred LOD rings with vertex morphing.
- **v3 — volumetric far** (octree voxel LOD, or the SVDAG raymarcher) **only if**
  the loss of *distant* overhangs/caves proves to genuinely matter. Deferred on
  purpose: it is the highest-effort path and the one most likely to eat the agent
  budget.

**The one trade to decide explicitly:** v1/v2 give a **heightfield far field** —
distant overhangs and caves flatten to a surface. The near field (where you dig,
where agents live, where it matters) stays fully volumetric. So "3D all the way
down" holds *where you interact*; only the un-interactable distance is
approximated — and the upgrade path to volumetric-far is real and reversible.
Distant Horizons sells its entire experience on exactly this trade.

**Known v1 seam:** smooth far mesh vs blocky near voxels is a stylistic
discontinuity; hide it by overlapping the near band generously, and/or render the
far mesh flat-shaded/stepped later. Noted, not solved.

## Sources (verification per the sweeps)
- bevy_voxel_world `chunk.rs` 32-unit ceiling — VERIFIED against source.
- Geometry clipmaps — Losasso & Hoppe 2004; GPU Gems 2 (Asirvatham & Hoppe) — verified.
- Transvoxel (Lengyel, transvoxel.org); 0fps blocky-LOD — verified.
- Aokana SVDAG raymarch — arXiv 2505.02017 (ACM CGIT 2025) — verified.
- bevy_terrain Bevy-0.14 pin; bevy_voxel_world 0.18 compat; clipmap-crate 0.18
  readiness — UNVERIFIED soft spots; confirm against Cargo.toml/git before relying.
- Distant Horizons internal data structure — secondary sources, UNVERIFIED in detail.
