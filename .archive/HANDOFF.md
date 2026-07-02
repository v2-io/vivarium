# Handoff — `slabs` explorer + architecture audit

*Written 2026-07-01 at the end of a long session, for the next instance (fresh
context). Points to detail files rather than repeating them. The prose docs are
`DESIGN.md` and `CLAUDE.md`; topic notes live in `ref/**/NOTES.md`.*

## Where things stand (TL;DR)

1. **New explorer `spikes/slabs`** — terrain rendered as *real 3D geometry* (a
   continuous point-mesh over the voxel heightfield) through Bevy's actual
   orthographic camera + hardware z-buffer. It reads better than anything prior
   and, unlike the tilemap, is a *physically-defensible* model. This supersedes
   the CPU-framebuffer `tilemap` direction. **The spikes are disposable
   instruments** — the real view belongs in a future `vivarium-app` under the
   core/view wall; don't harden the spikes.
2. **Architecture audit done → `ref/architecture-audit.md`.** Verdict: the
   foundation is *unusually clean for POC work*. One true decide-now item (RNG).
3. **Reviewed `~/src/neworld/` (2009) erosion references** — fundamentals still
   relevant; vivarium already uses *newer* erosion physics than the old papers.
4. **Design direction → [`DESIGN-REDUX.md`](DESIGN-REDUX.md)** (living; will
   eventually supersede `DESIGN.md`). Multi-fidelity world + runtime as one
   principle — *spend representation by consequence*; a lazy, memoized,
   content-addressed **query graph**; the **fidelity ladder** (swap a model, rerun
   only what changed). Punchline: the hard research is a *single* problem in three
   masks — **detail→abstract** — and the nearest concrete step is giving the
   erosion tier a complete content-addressed memo key + pull-based evaluation.

---

## 1. The `slabs` explorer (`spikes/slabs`)

### Why it exists / the lesson
The `tilemap` spike composited four 2-D primitives (tread fill + a face "smear" +
contour lines + a base plane) that never tiled watertight, so it fought endless
gap / occlusion / cut-back bugs. **We stopped compositing 2-D primitives and
built real geometry** — then the GPU does occlusion (z-buffer) and lighting
(real normals) for free. Key lessons, so they aren't re-learned the hard way:
- **Quantized "slabs" are inherently confetti for continuous terrain**: a smooth
  slope crosses a band line constantly, and each crossing becomes a vertical
  wall → walls everywhere. Making each cell a *vertex* (point-mesh) instead of a
  *quad* dissolves this — a slope just slopes.
- Occlusion/lighting belong to the GPU, not hand-rolled depth keys / facing dots.
- Loud debug beats plausible fallbacks (the tilemap's magenta base sentinel:
  "this should never show" — any magenta = a coverage bug, surfaced not hidden).

### The model
Two modes on one giant mesh (rebuilt for the visible footprint on pan):
- **point-mesh** (default, `VIVARIUM_POINTS=1`): one vertex per cell at its
  column height, grid-triangulated → a continuous smooth surface. No vertical
  faces ⇒ no confetti, no gaps.
- **columns** (`VIVARIUM_POINTS=0`, `P` toggles): each cell a real column (top +
  exposed side quads). True voxel look; scaly at overview scale, crisp up close.
- **Water** is a *separate translucent point-mesh* at the waterline; the ground
  mesh is the solid **bed** everywhere (lake/ocean floors included). Water is
  depth-shaded (Beer–Lambert: deeper ⇒ more opaque + darker blue), so the bed
  shows through shallows and deep water occludes.
- **Scale is true-to-world & isotropic**: 1 voxel = 0.5 m on every axis at
  `detail = 2`; `VERT=1` ⇒ no vertical exaggeration.

### Controls
`WASD` pan · wheel zoom · `Q/E` turn 45° (8 dirs; feels like turning the pawn) ·
`K/J` raise/lower camera angle (`K` toward top-down, `J` down to level and even
*below* level to look up at mountains) · `Y` toggle auto-angle · `P`
point-mesh↔columns.
- **Auto-pitch** keeps the pawn unoccluded by raising the camera just enough to
  clear foreground (fan probe toward the camera; cap ~86°). **Pan/turn
  re-enables auto** and resets to the default iso angle; `K/J` grab manual
  control instantly. (Sun-follows-camera was *rejected* — it scrambles the sense
  of direction; fixed sun is intentional.)
- **Pawn**: human-scale (0.5 × 0.5 × 2 m) red cube at the focus, seated on the
  *max of the 4 voxels* it straddles, held at ~1/5 up the screen at any angle.
- **Look-up** (negative pitch) near-clips the foreground so peaks + pawn show.
- **HUD** (faint white 20% panel): fps · pawn tile `(x,z)` + elevation (m above
  sea) · facing compass (`+Z = N`, `+X = E`) · camera angle · zoom · mesh params.

### Env knobs
`VIVARIUM_POINTS` (1/0) · `VIVARIUM_STRIDE` (voxels per vertex; 1 = finest,
heaviest) · `VIVARIUM_LEVEL_VOX` (elevation band; **1 = raw voxel heights**) ·
`VIVARIUM_VERT` (vertical exaggeration; 1 = true) · `VIVARIUM_SMOOTH` (median
passes on the level field) · `VIVARIUM_FOCUS_X/Z` · `VIVARIUM_ZOOM` ·
`VIVARIUM_PITCH` (radians; negative = look up; for scripted screenshots) ·
`VIVARIUM_AUTO_PITCH` · `VIVARIUM_AUTOSHOT` / `VIVARIUM_SETTLE`.

### Best command (true voxel resolution, smooth)
```
VIVARIUM_POINTS=1 VIVARIUM_STRIDE=1 VIVARIUM_LEVEL_VOX=1 VIVARIUM_VERT=1 VIVARIUM_SMOOTH=0 \
  ./target/release/vivarium-slabs
```
(Shares the worldgen cache with the other adapters — instant when warm.)

---

## 2. Architecture audit (`ref/architecture-audit.md` — read it)

- **#1 decide-now — RNG determinism.** `World` holds one `rng`; `World::step`
  draws from it per-agent *in iteration order*. Bit-identical replay passes only
  because the loop is single-threaded. DESIGN.md pitches Bevy ECS for **parallel**
  agents — the moment they step concurrently the draw *order* is
  scheduler-dependent and determinism dies. Fix: **per-agent splittable seeds**
  (SplitMix64) *before* the agent layer. Worldgen is already parallel-safe
  (stateless coordinate hashing); this is contained to the mutable `World.rng`.
  This is the natural bridge to the agent work (the real bet).
- **#2/#3 — the world is *bounded*; the docs overclaim.** The eroded landscape is
  a finite (~6 km, config at `spikes/bevy-voxel/src/main.rs:50`, not a core
  const), baked, `f32` artifact — not the "infinite pure function of the seed"
  the docs lean on. The overclaim is load-bearing (determinism-as-ontology *and*
  the "regenerate the far field, don't cache it" rendering decision both cite it).
  Cheap fix: **name the boundary honestly**. The hard fix (streamable erosion) is
  the deferred big one — erosion is *non-local* (a cell depends on its whole
  upstream watershed), so it doesn't chunk cleanly.
- **Coordinate overflow / Far Lands (the owner's sharpest worry): latent but not
  currently reachable.** `i32` voxel coords are sound (~1e6 km); the `f32`-metre
  precision decay only bites ~3 orders of magnitude *past* the bounded region.
- **Deferrable-but-real**: the edit-overlay's unbounded growth + the detail↔abstract
  reconciliation (already a known deferral); cross-platform float determinism
  (the agent flagged this honestly as *inferred, not measured*).

---

## 3. Elevation / scale facts (verified in `crates/vivarium-core/src/voxel.rs`)

- **Voxel elevation is an integer voxel index** (`i32`) = `round(h_m × detail)`.
  Resolution = `1/detail` m = **0.5 m at detail 2**. No sub-voxel precision; the
  continuous `f32`-metre height collapses to the nearest voxel.
- Physical consts are **metres**: `WORLD_HEIGHT = 8192` m,
  `world_height() = WORLD_HEIGHT × detail` voxels; `sea_level() = SEA_LEVEL × detail`;
  `METERS_PER_VOXEL = 0.5` (anchor at detail 2).
- **Earth-like needs ~20 km peak-to-trench** (Everest ≈ 8.85 km + Mariana ≈ 10.9 km).
  Current 8.2 km world is ~2.5× short. Widening is cheap (pure function) but moves
  the sea datum — decide deliberately.
- **Posits/unums** (owner asked): the elevation *range* doesn't need them (f32
  resolves to ~2 mm at 20 km). Where they'd earn their keep is **accumulated sim
  dynamics** (erosion/sediment/flow sums — a quire gives exact accumulation) and
  **cross-machine determinism** (single rounding mode, no NaN variance). Rust
  crate to check: `softposit` (I couldn't confirm a crate literally named
  `fast-posit` from memory — verify before citing).

---

## 4. `~/src/neworld/` (2009) erosion references — relevance

- `FastErosion_PG07.pdf` = Mei/Decaudin/Hu, *Fast Hydraulic Erosion on GPU* (PG'07),
  the virtual-pipes shallow-water method. Still foundational for *real-time water*,
  but vivarium's erosion tier already uses newer physics (FBM prior → **MFD
  stream-power → Davy-Lague deposition**, ~2009–2014). So: validation, not a step
  forward.
- `terrain_generation.pdf` (Olsen 2004) + `LitReview.pdf` (Sbarski): sound
  algorithms, obsolete "make it fast" tricks.
- Timeless & still used: Perlin/Simplex/Ridged/Voronoi noise, **L-systems**
  (`abop.pdf`) for flora. (Modern successor: OpenSimplex2.)
- **New since 2009, ranked by fit with vivarium's principled-from-physics ethos:**
  (1) **large-scale physically-based terrain** — Galin/Cordonnier/Guerrero
  (~2016–20): tectonic uplift + stream-power + *layered/multi-material* rock →
  the natural next reading; (2) particle/droplet erosion; (3) ML terrain gen
  (data-driven — probably a deliberate *no*).
- His 2009 `terrain.txt` already lists "*Relatively endless*" / "*JIT erosions*"
  and a `continuous-world.odp` — **exactly the audit's bounded-world tension**,
  still open because erosion is non-local. (Also: the `2D Circle Graphic` tileset
  in there is the same one the tilemap spike used.)

---

## 5. Open threads (not done)

- **Real cast shadows** — chosen as the next lighting lever (owner picked #2 over
  sun-follows-camera). *Not implemented.* Note: `STANDOFF = 4000` is large, so
  directional shadow cascades would be low-res; likely reduce `STANDOFF` (ortho ⇒
  no apparent-size change, but watch that the eye still clears tall peaks) and/or
  tune a `CascadeShadowConfig` for the zoom range. No existing spike configures
  cascades — new territory.
- **Pawn beacon** — an always-on-top pole/marker so the 2 px human-scale pawn is
  trackable while panning. Offered, not built.
- **Water edge** — all-4-wet triangulation insets water one cell from the true
  shore; could fade across boundary cells. Minor.
- **Lighting** was tuned (warm grazing key + cool fill + low ambient + deeper
  grass) — good baseline; shadows are the remaining drama lever.

## The bigger arrow (per `CLAUDE.md`)
World rendering (axes 1 & 2) is proven enough. The **highest-value** move is the
**agent layer** (axis 3, the real bet). The RNG fix is the honest bridge — it's an
agent-layer prerequisite, not more world polish.
