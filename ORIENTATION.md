# vivarium — orientation (start here)

*Current-state map for a fresh session. Supersedes the archived
`.archive/HANDOFF.md`. Last updated 2026-07-02 (end of the live-simulation
session: erosion telescope + live water + instruments).*

## What vivarium is
A sim game (RimWorld/DF lineage) on a deterministic 3-D voxel world, whose real
bet is simulation-grade agents on the Agentic Systems Framework. See `CLAUDE.md`
and `DESIGN.md`.

## Where the thinking lives (read in this order)
1. `DESIGN.md` — original decisions (engine = Bevy, geology, LOD-to-horizon).
2. `DESIGN-REDUX.md` §0–15 — the fidelity *philosophy* + runtime: spend
   representation by consequence; lazy memoized **query-graph** runtime; the
   **fidelity ladder**; content-addressed **storage = save**.
3. `DESIGN-MATERIAL.md` — the *matter data model*: strata / voxel / body; declared
   cell semantics; the research-backed **property set**; the nailed **spatial-key**
   plan (§8); undifferentiated materials.
4. `DESIGN-SYSTEMS.md` — the *phenomena graph*: 18 systems × timescales ×
   size-scales, coupling bands, and the **build order**.
5. `ref/research/` — `architecture-audit.md`, `foundation-validation.md`,
   `material-models-survey.md`, `spatial-key-bench.{md,rs}` (all adversarially
   verified / measured).

## The code
- `crates/vivarium-core` — the **working** deterministic voxel world (FBM +
  erosion + hydrology, flat `i32` patch). The proven **physics donor** to port from.
- `crates/vivarium-world` — the **clean-room frame**, zero deps, **26 tests green**:
  `quantity` (rich units + exactness) · `time` (`i64` dsec from Holocene) · `sphere`
  (CubeCoord/Geo + **`CellId`** S2 Hilbert key, `from_face_ij`) · `planet`
  (insolation) · `material` (Material/MaterialId + refinement ladder) · `column`
  (Stratum/Column + derived queries) · `noise` (coordinate-hashed, §8) · `gen`
  (CellId→Column baseline; **two-band prior**: continents λ~1250 km ±1500 m +
  mountains λ~25 km modulated by continental height — slope is what makes terrain
  read, measured 9%/36% mean/max) · `chunk` (Cartesian `Patch<T>` + halo — the
  stencil substrate) · `erosion` (**the fluvial pipeline, ported from core**:
  Priority-Flood fill → D8 receivers → MFD drainage → implicit stream-power n=1 →
  Davy-Lague deposition → talus; plus `ErodedRegion` sampling — bilinear + the
  detail increment — and `column_at`, the ladder dispatch; `examples/erosion_preview`
  is the ASCII instrument) ·
  `sample` (face region → height/water field patches for views). The foundation
  generates a world of columns on the sphere, runs a real erosion stencil on
  materialized patches, and renders through its own view (`spikes/worldview`).
- `spikes/worldview` — **the view over the frame** (depends on `vivarium-world`
  ONLY). **Run it:** `cargo run --release -p vivarium-worldview` — phases run
  automatically (macro erosion ~5 s → fine passes ~15 s → deluge to steady state
  ~2-3 min → living storms); the HUD sim line narrates. Engine at slabs parity: ortho point-mesh + depth-shaded water, auto-pitch
  fan probe + look-up near-clip, floating origin, honest 2 m pawn + reticle +
  pixel-exact scale bar, HUD with relief range + per-rebuild **gen ms** (the
  memoization instrument — ~142 ms/rebuild at defaults is the number §11 caching
  exists to kill). Defaults = slabs' start view (L24 ≈ 0.6 m cells, 130 m
  viewport, 1024-cell ≈ 614 m window). `[`/`]` = live level dial.
  `VIVARIUM_LEVEL/W/FOCUS_I/FOCUS_J/VERT/ZOOM/PITCH/AUTOSHOT/SETTLE`. Good spot:
  `VIVARIUM_FOCUS_I=5308416 VIVARIUM_FOCUS_J=13238272` (mountainside, L24 coords).
  The prior's mountain band is **band-limited to each sample's Nyquist** (≤16
  octaves from a 25 km base) — human-scale *texture* exists; *structure*
  (channels, valleys, cliffs) is still the erosion port's job.
- `crates/vivarium-world/examples/` — **GPU-free world diagnostics** (use these
  FIRST to split world-issues from explorer-issues): `topo` (ASCII elevation map +
  slope stats of any face window), `scan_land` (find peak/coast, prints
  VIVARIUM_FOCUS_I/J).
- `spikes/slabs` — the core-backed 3-D view; superseded as SOTA by worldview
  (kept as reference until the old core's remaining physics is fully ported).
- `archive/*` — superseded spikes.

## Decisions locked (rationale in the design docs)
- **Engine** Bevy · **coordinate** cube-sphere, S2-style Hilbert `CellId(u64)` as
  the canonical key (curve orders *chunks*; interiors are Cartesian — see the
  bench), `f64 CubeCoord` for math only · **time** `i64` deciseconds from Holocene
  onset · **vertical** ~20 km shell · **voxels** cubic 0.5 m · **quantities** rich
  at seams (SI-exponent units + exactness), raw `f64` in loops · **storage**
  content-addressed, the save *is* the store (git-shaped) · **matter** strata
  (storage) / voxel (view) / body (overlay) · **determinism** all stochasticity is
  a stateless coordinate/key hash (never a shared mutable stream).

## Next: build order
**Foundation + substrate done** (all tested + committed): `CellId` · `material` ·
`column` · `noise` · `gen` · `chunk` (Patch + halo). **Erosion port begun**:
hillslope diffusion (`erosion::diffuse`) runs on a Patch. Remaining, in order:

0. **Erosion + WATER SYSTEM: LANDED** (…→ d848979). worldview now runs the full
   multirate stack live: L19 macro tier (1 epoch/cycle + fBm-differential uplift,
   λ≈5 km) → L21/L24 fine tiers (FINISHERS per Joseph's field observation — 1–2
   animated passes ideal; init 4/cap 10 and init 2/cap 6, re-anchor to the pawn
   past ¼-span drift, mean-PINNED to the parent low band = Joseph's conservation
   constraint, §5) → the FAST band: virtual-pipes shallow water (water.rs, ported
   from core hydro; conserved atmosphere/ocean stores) raining onto the live
   L21 bed — **erosion stays ON while water flows** (the §4 schedule replacing
   core's kill-switch). Hillslope creep (κ=2 m²/epoch) added after the sawtooth
   anomaly was probe-isolated (spike_probe: detachment-limited spires at grid
   wavelength without diffusion — also latent in old core).
   **Instruments**: T = fidelity tint (violet=prior/blue=L19/yellow=L21/orange=
   L24); HUD sim line = per-tier epochs + aging speed (~y/s, EPOCH_YEARS=100
   nominal) + per-epoch mean |Δh| (convergence detector) + water rate/steady-
   state; screen newest/oldest sim-age. Env: VIVARIUM_RAIN (default 10×),
   VIVARIUM_LIVE, VIVARIUM_TIERDEBUG, VIVARIUM_ERODE(_NX).
   **Queued for the NEXT session (Joseph, 2026-07-02):** seams/transitions +
   memoization/world-saving (§13; the face-edge "floating mesa" specimen: tile
   clamping + sim-edge outlets + pin's raw-prior fallback), **sediment coupling**
   (time-averaged discharge → erosion's A; deposition into slack water → oxbows,
   lake→meadow fill — the honest water-erosion core turned off), async meshing
   (the 1.5 s rebuild throttle is the visible-water framerate), per-material
   erodibility.
1. `chunk.rs` — ✅ done (`Patch<T>` + halo, API driven by the erosion consumer).
2. **Port erosion** as a *native frame tier*, feeding `gen::column_from_surface` —
   the fidelity ladder made real. **Bridge recommendation (confirm with Joseph):
   port the *algorithm*, not the *data*** — re-implement core's stream-power +
   Davy-Lague stencil in `vivarium-world` on a Cartesian patch seeded by `noise`
   (the FBM prior); `vivarium-core` (flat `i32` patch) stays as the algorithm
   *reference* + the current slabs view's backing until the frame's erosion is
   proven and the view migrates. Keeps the core/view wall clean and drives the
   `chunk` API from a real consumer. *Trade-off:* re-implementation risks
   re-introducing bugs core already solved — the alternative (depend on core, sample
   its output) is safer short-term but bolts the sphere onto a bounded flat patch.

Then, per DESIGN-SYSTEMS build-order: crude climate → biomes → pedogenesis →
vegetation. And before the agent layer: the **RNG fix** (`architecture-audit.md` #1).

## The real bet (don't lose it)
Axes 1–2 (graphics, world dynamics) are proven enough; the highest-**value**
frontier is **axis 3 (the ASF agents)**. The prerequisite is the RNG fix —
per-agent splittable seeds (`architecture-audit.md` #1) — before agents step in
parallel. The world-model foundation earns its keep because agents *live in* this
coordinate/time/matter space.

## The one hard research problem (open)
**detail→abstract**: upscaling an irreducible agent edit back into a memoized
macro with correct up-invalidation (`DESIGN-REDUX` §6, `DESIGN-MATERIAL` §7).
Everything else has prior art.
