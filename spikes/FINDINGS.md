# Engine spike findings — Bevy vs Godot for the voxel view

> Outcome: **Bevy.** Decided 2026-06-22 after building the *same* deterministic
> `vivarium-core` world through both `bevy_voxel_world` (Bevy 0.18) and
> `godot_voxel` (Godot 4.7), to feature parity, and comparing empirically.
> This records the reasoning — including the confounds — so the decision can be
> re-examined honestly rather than taken on faith.

## What was compared

Two *view adapters* over one shared core. The core (`vivarium-core`) is the
deterministic Rust simulation — seed + sparse edits, Perlin/FBM terrain, voxel
LOD by stride-sampling. It never changed between engines; both views render the
identical world. So this is a fair test of the *view layer*, not the worldgen.

- **Godot side:** `spikes/godot-voxel/` (GDScript view) + `crates/vivarium-godot`
  (a gdext Rust bridge exposing core over FFI). Now archived under
  `archive/` — see below.
- **Bevy side:** `spikes/bevy-voxel/` (one Rust crate; view + core seam together).

Both reached: terrain render, LOD to the horizon, distance fog, first-person
fly + raycast dig/place.

## The axes, and how they landed

### Visual quality — **Bevy**, and not order-dependent
`bevy_voxel_world` bakes ambient occlusion into the mesh (crisp per-block
crevice shading) and handled LOD seams cleanly with skirts. Godot's *cubes*
mesher bakes no AO; screen-space AO (SSAO) + a directional key light + a muted
palette closed *most* of the gap, but not all — matching the baked-AO crispness
would mean switching Godot to the *blocky* (model-library) mesher, real extra
work. Bevy simply looked more polished out of the box.

### Streaming / LOD fill — **Godot**, narrow but real
`godot_voxel` is a mature module; its LOD fill-in and chunk streaming feel
smoother. `bevy_voxel_world` (a younger community crate) applies finished meshes
in clumps, producing occasional 30–100ms frame blips on chunk/LOD bursts (the
"LOD-fill jumpiness"). Steady-state is a flat 120 FPS either way; this is about
the *transients*. Believed mitigable (mesh-insertion time-budgeting), but real
today.

### Performance — **tie at steady state**, with a tuning lesson
Both hold vsync-capped 120 FPS steady. Bevy *appeared* to stutter worse, but
instrumentation showed the cause: `spawning_distance = 32` accumulated ~40k
chunk-entities while flying (110–250ms hitches, a 14fps stall). Cutting to 16
(~Godot's effective view) bounded it to ~4–5.6k entities and the stutter
vanished. **Honest correction:** part of "Godot felt smoother" was a config
asymmetry we introduced (Godot's view distance was tuned conservatively, Bevy's
was not), not pure engine maturity. (Data: `/tmp/bevy_diag.log` during the runs.)

### Code / comprehension — **Bevy**, decisively *for this project*
View-adapter size (core excluded; Rust files are heavily doc-commented, so raw
LOC overstates the gap):

| | Godot | Bevy |
|---|---|---|
| View logic | 353 LOC GDScript (3 files) | 279 LOC Rust (1 file) |
| Core seam | +203 LOC Rust bridge (FFI) | *(same file)* |
| Config/build | 142 LOC, 5 files, 4 formats | 19 LOC (Cargo) |
| Languages | 2 + FFI | 1 |

The structural point matters more than the numbers: **Godot's extra surface —
second language, FFI bridge, the gdext `RwLock`/`experimental-threads` rules, the
Apple-Silicon codesign tax — exists because vivarium's core *must* be Rust.** A
pure-Godot game would be single-language too; that complexity is the tax of
bolting a Rust core onto a non-Rust engine. **Because our core is already Rust, a
Rust engine erases that boundary** — view and core share one language, one type
system, one compiler, no FFI. This is architecture-specific and *not*
order-confounded.

### Iteration ergonomics — mixed
- **Bevy:** errors are compile-time and precise (a fix list). Godot's were
  *silent runtime* aborts (a gray window from a `:=` inference slip or a wrong
  property name), forcing run→introspect→fix loops. For an agent especially,
  machine-readable compile errors are a real edge.
- **Godot:** GDScript iteration is *instant*; Bevy recompiles take minutes. For a
  human tuning look-and-feel, that loop is genuinely better. "Fewer Bevy
  iterations" also partly reflects that Bevy went *second* — the hard design
  (core seam, LOD-via-stride, fog, palette) was worked out during Godot and
  inherited.

## Decision

**Bevy.** It held up on the axis that could have overturned the prior plan
(visual quality + UX), and on our load-bearing axis it collapses the FFI seam a
non-Rust engine necessarily imposes on a Rust core. This *converges with what
`DESIGN.md` hypothesized before any code was written* — Bevy for
ECS-as-cognitive-LOD and a Rust-native deterministic core — now with empirical
backing rather than intuition.

Genuine costs accepted, eyes open:
- LOD-fill is jumpier than Godot today (mitigate via mesh-insertion budgeting).
- Per-block visual polish needs deliberate setup (we got there; it's not free
  the way godot_voxel's baked AO is, but Bevy's *was* the nicer default).
- Slow compile-iteration vs GDScript's instant loop.

## Carried forward / open

- The Bevy spike (`spikes/bevy-voxel/`) is the basis for the real view; it is not
  yet at visual parity with the dialed-in Godot look (palette + matched
  overcast/fog were Godot-side polish not yet ported back).
- Mesh-insertion smoothing for the LOD-fill jumpiness is the first real
  Bevy-side perf task.
- **The bigger point (per `DESIGN.md`'s own warning):** this was axis-1
  (graphics) work — the "infinitely expandable" budget trap. It settled the
  engine question empirically, which was worth doing. The actual bet is axis-3,
  the ASF agents and the cognitive-LOD seam. That's where the next high-value
  work is.
