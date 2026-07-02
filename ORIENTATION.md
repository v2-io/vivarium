# vivarium — orientation (start here)

*Current-state map for a fresh session. Supersedes the archived
`.archive/HANDOFF.md`. Last updated 2026-07-01, at the design→code transition.*

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
- `crates/vivarium-world` — the **clean-room frame**, zero deps, **20 tests green**:
  `quantity` (rich units + exactness) · `time` (`i64` dsec from Holocene) · `sphere`
  (CubeCoord/Geo + **`CellId`**, the S2 Hilbert key) · `planet` (insolation) ·
  `material` (Material/MaterialId + refinement ladder) · `column` (Stratum/Column
  + derived queries) · `noise` (coordinate-hashed, §8) · `gen` (CellId→Column
  baseline). **The foundation is complete** — it can deterministically generate a
  world of columns on the sphere.
- `spikes/slabs` — the current 3-D view (ortho point-mesh); a **disposable
  instrument**, not the real renderer.
- `spikes/tilemap`, `archive/*` — older / superseded.

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
**Foundation done** (all tested + committed): `CellId` · `material` · `column` ·
`noise` · `gen` (CellId→Column). What remains is the design-sensitive **integration**:

1. `chunk.rs` — the Cartesian patch (dense row-major interior + halo, keyed by
   `CellId`) that automata run on. **Drive the API from its first consumer**
   (erosion), not speculatively. (DESIGN-MATERIAL §8; `ref/research/spatial-key-bench.md`.)
2. **Port erosion** from `vivarium-core` onto a materialized patch that feeds
   `gen::column_from_surface` — the fidelity ladder made real. Needs the
   **core↔world bridge decision** (how core's flat-patch FBM/erosion maps onto the
   cube-sphere frame) — worth a design beat, ideally with Joseph, before coding.

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
