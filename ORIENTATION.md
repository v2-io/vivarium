# vivarium — orientation (start here)

*Current-state map for a fresh session. Consolidated **2026-07-11** from the
addenda-grown form (the pass ARCHITECTURE got at v0.3): session history lives in
the git log, retirements in `SUPERSEDED.md`, the actionable queue in `TODO.md`.
Keep this file present-tense — when state changes, change the body; don't append
addenda.*

## What vivarium is

A sim game (RimWorld/DF lineage) on a deterministic 3-D voxel planet
(cube-sphere, 0.5 m finest rung), whose real bet is **simulation-grade agents on
the Agentic Systems Framework** — and, dual to that, a high-identifiability
laboratory where ASF/AAT quantities are authored, known, and dialable. The
duality is the point (`README.md`). Vivarium is a member of the Archema program
and a supporting project for ASF: `ASF.md` is Level A — read every session; it
carries the conceptual bridge, the reading gates, and the **standing moratorium
(§0)**.

## Where the thinking lives (read in this order)

1. `doc/ARCHITECTURE.md` — the consolidated frame: **one principle (represent by
   consequence) on three axes** — multiscale substrate machinery / phase-freeze
   developmental ladder / use-case-as-fidelity-contract.
2. `doc/design/DESIGN.md` — purpose, disposition, founding commitments (the three early
   decisions: core/view wall, determinism-as-ontology, the fidelity invariant).
3. `doc/design/DESIGN-REDUX.md` — fidelity philosophy + runtime: the lazy memoized
   query graph, the fidelity ladder, §12 pervasive memoization, §13
   storage-is-the-save.
4. `doc/design/DESIGN-MATERIAL.md` — the matter data model: strata / voxel / body, declared
   cell semantics, the property set, the spatial-key plan (§8).
5. `doc/design/DESIGN-SYSTEMS.md` — the phenomena graph: systems × timescales, coupling
   bands, build order, the fluvial inventory, the instruments.
6. `tabularium/terrestris.ordinum.udon` — the codified developmental ladder (Charge / Promise / Record; the moral
   line at Phase 7).
7. `LEXICON.udon` — canonical vocabulary; `SUPERSEDED.md` — the do-not-reuse
   ledger; `ASSUMPTIONS.md` — the magic-constant ledger (touch a constant,
   touch the ledger).
8. `doc/theory/` — the technical core: `multiscale-seams.md` (position AND time
   as one seam discipline; the resolution-light-cone / dynamic-exponent-$z$
   unification) + `multiscale-methods.md` (the R/L/closure operator algebra).
9. `doc/plan/` — the live build path: `abyssal-parity-plan.md` (the six phases),
   `builder-explorer-decoupling.md` (operational design),
   `regula-conformance-design.md` (world-level conformance, two chapters),
   `framework-to-status-quo.md`, `vivium-operational-workflow.md`,
   `water-parallelism.md`.
10. `ref/` — true reference, consulted on demand: surveys, dossiers, NOTES from
    past work eras, pdfs. `doc/PROCESS.udon` — the working norms;
    `doc/toolchain.md` — the epistemics toolchain.

## Decisions locked (rationale in the design docs)

**Engine** Bevy · **coordinate** cube-sphere, S2-style Hilbert `CellId(u64)` as
the canonical key (curve orders *chunks*; interiors are Cartesian), `f64
CubeCoord` for math only · **time** `i64` deciseconds from Holocene onset ·
**vertical** ~20 km shell · **voxels** cubic 0.5 m · **quantities** rich at
seams (SI-exponent units + exactness), raw `f64` in loops · **storage**
content-addressed, the save *is* the store (git-shaped) · **matter** strata
(storage) / voxel (view) / body (overlay) · **determinism** all stochasticity
is fated noise — a stateless KRNG of (seed, key), never a shared mutable
stream.

## The principled frame (standing and growing — the live architecture)

All committed and tested — **62/62 green in `vivarium-world`** (verified
2026-07-11):

- **Store** (`store.rs`) — content-addressed objects/roots, atomic writes,
  domain-neutral (keys → bytes). The save-file IS the memo store, real.
- **Pull-query** (`query.rs`) — lazy: nomoi check the store, compute on miss,
  memoize. Three systems compose end-to-end purely through pulled/keyed
  artifacts — **spine v2** (fbm3 prior sampled on the 3-D sphere; v1's measured
  ~3 km face-edge cliffs dissolved) → **erosion** → **water** (bounded
  deterministic fill; declared honest limits: tiles hydrologically isolated
  pending flux-BC, per-tile rain stores pending the atmosphere reservoir).
- **Manifest** (`spec.rs`) — a vivium individuated by identity (seed, minted
  once) / label / demand buckets.
- **World-seed threading** — seed-first KRNG convention (`noise.rs`); nomoi are
  methods on `World { store, seed }` so key-seed ≡ compute-seed by
  construction; seed 0 = the legacy world, pinned by golden tests.
- **Nomotheke** (`nomotheke.rs`) — every nomos declares its epistemics, deps,
  promises-with-conservation-claims, and `ASSUMPTIONS.md` anchors as DATA;
  declarations mint the store keys; the ledger compiles into tests; derived
  quality = weakest-link fold. README §"Epistemic honesty is enforced in code"
  is the front-door statement; ARCHITECTURE §9 items 5–6 are the workflow.
- **CLI** (`crates/vivarium-world/src/bin/vivarium.rs`) — `new` / `build` /
  `status` / `attach`; builder v0 sweeps spine→erosion→water over all six faces
  under `builder.lock` (a second build ATTACHES); `status` renders the fidelity
  pyramid with the declared/derived honesty column. Whole planet at L7:
  ~630 ms cold, ~1 ms warm.

Operational design around it: `doc/plan/builder-explorer-decoupling.md` —
builder daemon / read-only explorers / demand spool / **beacons** (settled,
LEXICON §2) / watchpoints / the fidelity pyramid — and the one invariant that
makes build order irrelevant to results: **depend by key, never "latest
available."**

**Standing directive (Joseph, 2026-07-09) — pervasive disk memoization.**
Memoize every expensive pure computation to disk, at every tier and rate, under
the hard constraint that caching never interferes with algorithm iteration; the
only lawful mechanism for that constraint is the complete content-addressed key
(nomos versions auto-derived from kernel source where possible) — never
caution, never manual cache-clearing. Full statement + named failure modes:
`doc/design/DESIGN-REDUX.md` §12.

## The code (map)

- `crates/vivarium-world` — the frame crate, zero deps: `quantity` · `time` ·
  `sphere` (CubeCoord/Geo + `CellId`) · `planet` (insolation ephemeris) ·
  `material` · `column` · `noise` (KRNG) · `gen` (spine) · `chunk`
  (Patch + halo) · `erosion` (the ported fluvial pipeline: Priority-Flood →
  MFD → implicit stream-power → Davy-Lague → talus + creep) · `water` ·
  `sample` · `store` · `query` · `spec` · `nomotheke`.
- `crates/vivarium-world/examples/` — **GPU-free instruments** (use these FIRST
  to split world-issues from view-issues): `store_explore` (headless store
  walk — the no-GPU quick taste), `topo`, `scan_land`, `hypsometry`,
  `globe_ascii`, `erosion_preview`, and the regime probes `channel_profile`,
  `armor_regimes` (1/3 green, opens in its header), `seam_ridge` (**RED by
  design** — gates the flux-BC seam fix), `spike_probe`, `velocity_histogram`,
  `budget_probe`, `float_probe`, `source_incision`.
- `spikes/globe` — spin/zoom/pick Google-Earth view over the store; real sun
  ephemeris + ethereal time scrub (`,`/`.` hour · `N`/`M` day · `P` play ·
  `Y` headlight — verified against `spikes/globe/src/main.rs`).
  Verified spine v2 globally; found the write-atomic identical-bytes
  race and the `from_unit` edge-tie trap (never sample ON a face edge — cell
  centers only). Wants quadtree LOD past L9.
- `spikes/worldview` — the physics **testbench**: kernels canonical (the full
  multirate stack live — L19 macro erosion → L21/L24 finisher tiers →
  virtual-pipes water with erosion ON), architecture donor-only (one fixed
  patch, non-composable tiles — see the SUPERSEDED watch-list). Run:
  `cargo run --release -p vivarium-worldview`; knobs
  `VIVARIUM_LEVEL/W/FOCUS_I/FOCUS_J/VERT/ZOOM/PITCH/RAIN/LIVE/SEED/…`; a good
  mountainside: `VIVARIUM_FOCUS_I=5308416 VIVARIUM_FOCUS_J=13238272` (L24).
  Known: a cold deluge fill wants ~2 h — *bypassed*, not fixed, by the frame
  path (tiles memoize once, forever); kernel-level plans still on deck are in
  `TODO.md` §Kernel physics.
- `spikes/slabs` — the prior 3-D view, superseded by worldview; kept as
  reference until the old core's remaining physics is fully ported.
- `crates/vivarium-core` — the flat `i32`-patch physics donor; algorithm
  reference until the port completes.
- `archive/*` — superseded spikes (incl. the Godot head-to-head; findings in
  `spikes/FINDINGS.md`).

## Current build target and queue

The first playable milestone: an **ethereal (observe-only, moratorium-clear)
explorer in a Realized-not-Lawful early-Abyssal world** — the six-phase path in
`doc/plan/abyssal-parity-plan.md`, with conservation riding WITH the parity
track, not behind it. Next increments:

- **#10 — fine-tier erosion nomoi**: L21/L24 seeded from coarser tiles;
  walkable-scale ground through the store.
- **#11 — the first-person ethereal explorer**, over the query front-end
  (navigation and persistence fall out; UX intents inherited from the
  testbench era are in `TODO.md` §Explorer intents).
- Alongside: the **reservoir layer + thermal spine** (`mantle-thermal` +
  `climate-ebm`), the **water-system decomposition**, and **regula v0 landing
  as that work's spec** — all specified in `TODO.md`.

## The real bet (don't lose it)

Axes 1–2 (graphics, world dynamics) are proven enough; the highest-**value**
frontier is **axis 3 (the ASF agents)**. Standing prerequisite before agents
step in parallel: the **RNG fix** — per-agent splittable seeds
(`ref/architecture-audit.md` #1). The world-model foundation earns its keep
because agents *live in* this coordinate/time/matter space. Hard gate before
any agent-seam work: the Level-C reading, `ASF.md` §5.

## The one hard research problem (open)

**detail→abstract**: upscaling an irreducible agent edit back into a memoized
macro with correct up-invalidation (`DESIGN-REDUX` §6, `doc/theory/multiscale-seams.md`
§2.4). Not on the ethereal-explorer path (a read-only explorer makes no edits);
plausibly the same shape as the open AAT identifiability bet. Everything else
has prior art.
