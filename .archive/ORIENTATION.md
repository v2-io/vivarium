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
8. `doc/theory/` — the technical core. **⚠ REQUIRED before authoring or reworking any
   field nomos: [`discretisation-and-information.md`](doc/theory/discretisation-and-information.md)**
   — *what a nomos is actually claiming*. The **Prime Question** (*"what physical claim
   is this algorithm making?"*), the FV/FD/FE taxonomy, the **structure table** (find
   your phenomenon → find your scheme → **find what it conflicts with**), bias-vs-noise,
   one-reconstruction-per-consumer, and the probes that would convict us. Then
   `multiscale-seams.md` (position AND time as one seam discipline; the
   resolution-light-cone / dynamic-exponent-$z$ unification) + `multiscale-methods.md`
   (the R/L/closure operator algebra).
9. `doc/plan/` — the live build path: `abyssal-parity-plan.md` (the six phases),
   `builder-explorer-decoupling.md` (operational design),
   `framework-to-status-quo.md`, `vivium-operational-workflow.md`,
   `water-parallelism.md`. (`regula-conformance-design.md` is now a **reasoning
   trail, not the plan** — the regula collapsed into ordinum + manifest,
   2026-07-12; its §3 requisite-audit spec is what actually landed.)
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
stream. · **staggering** (2026-07-13, *tentative*) scalars at centres, **fluxes on
faces**, **metric carried per cell** — see §"The grid" below before designing any
field nomos; the **router is NOT covered by it and is NOT decided**.

## The principled frame (standing and growing — the live architecture)

All committed and tested — **87/87 green in `vivarium-world`** (verified
2026-07-12). The water chain runs hydrosphere → climate → precipitation →
erosion + water, conserved end to end.

**The flux web is GOVERNED BY THE ORDINUM — and it currently convicts this
world, which is the machinery working.** Erosion declares that it consumes
`emerged land`; the ordinum's `promise[emerged-land]` is an Abyssal `:tag gate`
that **nothing keeps**; so the requisite audit reports an unmet need and the
world is mechanically unrunnable past that point. An earlier build printed
"unmet flux needs: none" over a world that physically could not run erosion —
because erosion had never *declared* that it needs land. Do not read a closed
web as the goal; read the conviction as the audit doing its job
(`DECISIONS[ordinum-governs-the-flux-web]`).

- **Store** (`store.rs`) — content-addressed objects/roots, atomic writes,
  domain-neutral (keys → bytes). The save-file IS the memo store, real.
- **Pull-query** (`query.rs`) — lazy: a nomos checks the store, computes on
  miss, memoizes. The systems compose end-to-end purely through pulled/keyed
  artifacts — **initial-topography v2** (renamed from "spine" 2026-07-12 to the
  standard landscape-evolution-model term; fbm3 prior sampled on the 3-D sphere,
  which dissolved v1's measured ~3 km face-edge cliffs) + **uplift** (the
  tectonic driver, its own declared nomos — `uplift.rs`, a crude
  constant×low-freq-fBm stub) → **erosion** (consumes the initial surface it
  carves, the uplift field it carves against, and precipitation) → **water**
  (bounded deterministic fill; declared honest limit: tiles hydrologically
  isolated pending the flux-BC seam).
- **Manifest** (`spec.rs`) — a vivium individuated by identity (seed, minted
  once) / label / demand buckets.
- **World-seed threading** — seed-first KRNG convention (`noise.rs`); nomoi are
  methods on `World { store, seed }` so key-seed ≡ compute-seed by
  construction; seed 0 = the legacy world, pinned by golden tests.
- **Nomotheke** (`nomotheke.rs`) — every nomos declares its epistemics, deps,
  **consumes** (the fluxed quantities it needs), promises-with-conservation-
  claims, and `ASSUMPTIONS.md` anchors as DATA; declarations mint the store
  keys; the ledger compiles into tests; derived quality = weakest-link fold.
  README §"Epistemic honesty is enforced in code" is the front-door statement;
  ARCHITECTURE §9 items 5–6 are the workflow.
- **Hydrosphere** (`hydrosphere.rs`, 2026-07-12) — the framework's **first
  non-field nomos: a reservoir/box** (global stocks, no grid — the
  domain-fixation-guard generality probe, ARCHITECTURE §0). It derives a
  **conserved** water inventory from an *ante-mundane* charge (the water-mass
  fraction of planetary mass), partitioned ocean/atmosphere — so precipitation's
  eventual source traces to declared planetary mass, never conjured. Earth budget
  order-checked (~1.37×10⁹ km³ total, ~25 mm atmosphere), conservation exact.
  `vivarium status` prints it in real units. Proves the store/nomotheke/flux-web
  contract is representation-agnostic (a box keys by identity alone, no coords).
- **Climate** (`climate.rs`, 2026-07-12) — the **flow** that makes the hydrosphere
  a *cycle*: precipitation = atmosphere stock ÷ residence time. Conserving (precip
  = evaporation in steady state, so the inventory is untouched), causal (rain
  scales with the stock), and order-correct by construction (~1 m/yr for Earth).
  A **field** nomos fed by a **box** — the first cross-representation coupling.
  Honest limit: v0 is globally UNIFORM (global mean only; no ITCZ, orography, or
  latitude bands — a *pattern* claim from it would be false). Erosion consumes it
  as a relative discharge weight; water rains it at the principled rate, with the
  old ~9000× "rain rate" fudge decomposed into principled-rain × a declared
  `bounded-fill acceleration` (the analytic init retires that).
- **Flux web + requisite audit** (`flux.rs`, `audit.rs`, 2026-07-12) — the
  coupling contract as data: each nomos `consumes`/`produces` quantities drawn
  from one shared **flux vocabulary** (a typo'd quantity fails the build, same
  discipline as the ASSUMPTIONS anchors), and the audit resolves each consumed
  quantity to its producer **before anything runs**. `vivarium status` prints the
  coupling graph and the unmet-needs list beside the fidelity pyramid. The
  original specimen — *"can we rain principled water?"*, answered **No** because
  erosion and water consumed a precipitation no nomos produced ("rain without a
  sky") — is now **resolved**: the hydrosphere → climate chain produces it. The
  live unmet need is `emerged land` (above). A coherence test pins
  consumed-and-met ⟹ in-deps (the complete key can't omit a producer).
- **Ordinum** (`ordinum.rs`, 2026-07-12) — parses `tabularium/terrestris.ordinum.udon`
  and makes the phase ladder *govern* the flux web: a promise's fluxed quantity
  IS a flux quantity, `:kept-by` IS its producer, so a nomos consuming an unkept
  promise is an unmet need. Computes each promise's maturity (NotStarted /
  Specified / Claimed / BrokenKeeper); `Kept` is deliberately never auto-derived
  (claiming it without running something that could refute it is
  plausibility-as-verification). A `BrokenKeeper` check caught two undeclared
  nomos on its first two runs (`planet`, `noise`). **Still owed: the maturity
  report is not wired into the CLI** — the engine exists, the display does not
  (TODO).
- **CLI** (`crates/vivarium-world/src/bin/vivarium.rs`) — `new` / `build` /
  `status` / `info` / `attach`; builder v0 sweeps initial-topography → erosion →
  water over all six faces under `builder.lock` (a second build ATTACHES);
  `status` renders the fidelity pyramid with the declared/derived honesty column,
  the water budget, and the flux/requisite audit; `info` draws the whole-sphere
  Hammer equal-area oval coloured by build-state. On PATH via a symlink to the
  release build (README). Whole planet at L7: ~630 ms cold, ~1 ms warm.

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
  `material` · `column` · `noise` (KRNG) · `gen` (initial-topography) · `chunk`
  (Patch + halo) · `uplift` (the tectonic driver) · `erosion` (the ported fluvial
  pipeline: Priority-Flood → MFD → implicit stream-power → Davy-Lague → talus +
  creep) · `water` · `hydrosphere` (the reservoir/box nomos) · `climate` ·
  `sample` · `store` · `query` · `spec` · `nomotheke` · `flux` (the shared
  quantity vocabulary) · `audit` (the requisite/conservation graph) · `ordinum`
  (the ladder, parsed from `tabularium/`).
- `crates/vivarium-world/examples/` — **GPU-free instruments** (use these FIRST
  to split world-issues from view-issues): `store_explore` (headless store
  walk — the no-GPU quick taste), `topo`, `scan_land`, `hypsometry`,
  `globe_ascii`, `erosion_preview`, and the regime probes `channel_profile`,
  `armor_regimes` (1/3 green, opens in its header), `seam_ridge` (**RED by
  design** — gates the flux-BC seam fix), `spike_probe`, `velocity_histogram`,
  `budget_probe`, `float_probe`, `source_incision`. **`grid_lab/`** — the sphere-grid
  bench: nine grids (equiangular / Snyder-equal-area / gnomonic cube · rhombic
  dodecahedron ×2 · HEALPix · icosahedral triangles · hexagonal Voronoi dual raw +
  SCVT) on one `Mesh` with combinatorial adjacency and Euler asserted at construction.
  Prints every number in [`ref/research/grid-comparison-report.md`](ref/research/grid-comparison-report.md).
  §9a (`fan.rs`) is the MFD-fan probe: the fan analytically at **any** level (L19/L23 need no mesh —
  it is a local quantity, and its L→∞ limit is the map's Jacobian in closed form), plus the
  bias-vs-noise **plume** on the exact cone, with a perfect-lattice control, a 120°-rhombus control,
  a mirror-symmetry noise floor, and a generic-pole falsification control.
- `spikes/globe` — spin/zoom/pick Google-Earth view over the store; real sun
  ephemeris + ethereal time scrub (`,`/`.` hour · `N`/`M` day · `P` play ·
  `Y` headlight — verified against `spikes/globe/src/main.rs`).
  Verified initial-topography v2 globally; found the write-atomic identical-bytes
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

## ⛔ Read this before building anything (2026-07-12)

Two measured findings reorder the queue. Both were caught the same way — by making a
*decreed* thing **derivable** and checking whether it cohered — and both had been
invisible for months:

1. **`SEA_LEVEL_M` is manufacturing forbidden land, and this world is supposed to be
   submerged.** `examples/sea_level_probe.rs`: pour the hydrosphere's conserved
   1.3735e9 km³ into the generated terrain and it overflows the planet (basin capacity to
   the highest peak = 1.3619e9 km³; **ratio 1.01×**). Derived sea level sits **23 m above
   the highest ground** → land **0.0%**. The 33.4% land we build and erode exists **only**
   because `SEA_LEVEL_M` is *decreed* (4000 m) rather than *derived*.

   **Read that against the ordinum, not against modern Earth.** A water-covered surface is
   the Protogenic **promise KEPT** (`charge[smooth-water-surface]`), and Abyssal's
   `charge[emergent-land] :tag gate` says land is *"delivered by uplift / proto-tectonic
   processes, **never an initial condition**."* So the probe **confirms the ordinum** and
   **convicts the initial-topography**, whose own nomos declaration already admitted it
   "impersonates Abyssal output rather than the Phase-2 submerged promise." Our continents
   are Phase-3 output that was never earned — and erosion no-op'ing on submarine ground is
   **physically correct** (`charge[erosion-carving]` is itself a Phase-3 gate).

   ⇒ **Bimodality is NOT a fix to the prior.** Isostasy is the *mechanism by which Abyssal
   delivers land* (crust differentiates; buoyant continental crust rides high) — it belongs
   to the emergent-land charge, not to the initial condition. **Corrected order of work:**
   (a) derive sea level from the hydrosphere's ocean stock → the world becomes correctly
   submerged, `SEA_LEVEL_M` retires, and the fBm becomes honest **bathymetry** instead of
   fake continents; (b) **Abyssal must EARN the land** — the `uplift` nomos is exactly the
   piece that must deliver `emerged-land`; (c) *then* erosion carves land that was produced
   rather than assumed. (`DECISIONS[water-world-is-the-promise-not-the-bug]`, Joseph.)

   *Joseph's open nuance, not yet adjudicated:* the ordinum does not strictly *forbid* early
   emergence — a model/ladder mismatch is a **sign** that one of the two is wrong, and both
   need checking. The early-continents survey is what adjudicates it. See TODO §Prior v3.

2. **The fluvial probes had been measuring seabed.** The fluvial unit tests and `seam_ridge`
   ran on a 100%-submarine footprint, so erosion no-op'd and the tests compared no-ops;
   `seam_ridge`'s famous "ratio 22888" was `0 ÷ 1e-9`. **The seam had never been measured.**
   Fixed (verified-land footprints + a `test_footprint_is_actually_land` guard). The real
   seam is **2.45× → 5.79×**, growing with the differential age gap while the interior stays
   flat — which corroborates the honest 2026-07-03 figure (4.3/5.3/7.1) that had been lost.

**Two standing guards, both learned the hard way here:**

- *A probe that cannot fail is not a probe.* Check that the physics can even **execute** at
  a probe's footprint before trusting its number — and be MORE suspicious, not less, of a
  number that confirms what you already believed. (The 22888 tell — a bit-identical ratio
  across all three age gaps — was printed every run and nobody divided it back out.)
- *Check the ladder, not modern Earth.* Before calling a world-state result a defect, read
  which **phase** the ordinum says the world is in, and read the nomos's own `relation` /
  `status` declaration. Twice in one day a **correct measurement** produced a **wrong
  conclusion** from a plausible frame (modern Earth) while the authoritative source — already
  read — said otherwise. The failure was framing, not measurement.
  (`DECISIONS[check-the-ladder-not-modern-earth]`.)

## Current build target and queue

The first playable milestone: an **ethereal (observe-only, moratorium-clear)
explorer in a Realized-not-Lawful early-Abyssal world** — the six-phase path in
`doc/plan/abyssal-parity-plan.md`, with conservation riding WITH the parity
track, not behind it. Next increments:

> ## ⚑ HANDOFF — the state at the end of 2026-07-13
>
> *A long session audited the kernels, ran eight agents, and found that **almost every "principled" thing we believed about our own physics was wrong in an interesting way.** Read `DECISIONS.decision-log.udon` for 2026-07-12/13 in full — it is the record. Nothing below is decided except where it says so.*
>
> ### Fix these today. Verified, no decision required, and each is small.
> 1. **`erosion.rs:371` — `cell_area = cell_m * cell_m`.** One uniform area for a grid whose cells vary 1.4×. **Measured +17.8% mean, +41.2% at face edge-midpoints, bit-identical across L5–L13 ⇒ CANNOT be out-resolved.** It hands erosion a **cube-locked fake erodibility field**. ~6 lines; closed form in `msc/spike-wavelet-store/`.
> 2. **`erosion.rs:369` — `const P: f32 = 1.1` → `1.0`.** **One character.** At `p=1` the first moment is **exactly zero** (a theorem); at 1.1 it is a 45°-periodic grid-locked deflection **toward the axes** — *and `grid_lab`'s perfect-lattice control has been printing that 0.24° on every run, and we read it as a baseline.*
> 3. **Kill MFD's diagonals.** They cross **no face** — measured **47.8% phantom flux** — and they are what makes MFD un-correctable by any post-process.
> 4. **`water.rs`: store the surface `η`, not `depth`.** `η = bed + depth` in f32 at a 4000 m datum **forges a 1.6e-4 m/s (≈14 m/day) current in a dead-calm lake that never decays** (f64: 2.9e-13). Free, and structurally honest — `η` is what the scheme integrates.
> 5. **Promote `water.rs`'s hardcoded θ / Froude cap / Jarrett constants to `WaterParams`.** *An assumption you cannot vary is an assumption you cannot test.* **This currently blocks every water probe.**
> 6. **`pin_block_means`: swap the bilinear delta upsample for a block-CONSTANT one** (preserves the mean *exactly*, costs *less*), then **re-run `seam_ridge`.** **Cheap, decisive, and it can fail.** *(And fix the test `pin_preserves_parent_means` — it is named for an invariant it does not test, with a tolerance sized to accommodate the defect.)*
>
> ### The #1 gap — and it now has a sourced mechanism
> **`uplift` is STRICTLY POSITIVE everywhere and is therefore MECHANICALLY INCAPABLE of keeping `promise[emerged-land]`** — the gate the whole queue is blocked on. The fix is not a better uplift field: **uplift should not be a producer at all.** The primitive is the **lithospheric COLUMN** (conserved); **elevation is DERIVED by isostasy**; and *"uplift rate"* is merely `de/dt`. **The depleted mantle keel is half the mechanism** (Chowdhury et al. 2025, read; `relata: chowdhury-2025-subaerial`). **⊘ Korenaga et al. 2017 (the freeboard-modelling reference) is unread and wanted.** See `DECISIONS[isostasy-is-the-uplift-nomos-and-the-keel-is-half-of-it]`.
>
> ### The architectural payload — and it is the thing worth doing
> **[`doc/design/NOMOS-CONTRACT.md`](doc/design/NOMOS-CONTRACT.md) — read it.** Every defect found on 2026-07-13 fell into **one of five boxes, and the flux web only has the first.** ② geometry · ③ semantics · ④ structure · ⑤ the modified equation. **`NomosDecl` has nowhere to put any of them** — that gap is the next build, and it is mechanical. **The five `doc/design/nomos-contract/*.md` files are specified in that document and NOT YET WRITTEN** (Joseph's design: each box gets a file with the math, the procedure, a worked specimen, **its probe**, and a **failure gallery** — *a procedure, not a taxonomy*).
>
> ### Open, and NOT Claude's to decide
> - **The leaf-only-evolution price** on flux-on-the-face (it cuts against the memoised-independent-tier design — the lazy pull-query's whole premise). *A proposed resolution landed after this block was written* (`DECISIONS[the-macro-tier-has-two-roles-and-conservation-buys-its-freedom]`: the macro tier's **governance** role is conserved and therefore free of its leaves; only its **summary** role is leaf-bound) — **but it is a structural argument, explicitly NOT verified. Test it before building on it.**
> - **LEXICON:** `regula` is a **13-edge hub node** still `:status settled` though retired; **`manifest` — the load-bearing noun — has NO entry**; and `:status` has **no value meaning *retired*** (a blocking sub-decision).
> - **Three `:by us` tags flagged as possibly inflated** and deliberately NOT corrected (`snyder-closes-the-projection-lead`, `seam-amortization-and-the-two-grid-overlay`, `geometric-contract-metric-set`) — *correcting an authority tag is not Claude's authority.*
>
> ### Probes that could still convict us — none have run
> The **landscape consequence** of the routing κ (does the channel network actually differ? ~a day — *this prices it*) · **well-balanced + sediment ON** (does the f32 ULP current **rectify into net bed transport**?) · the **roll-wave probe** (raise Manning `n` until `Ve<1` at fixed θ) · an **entropy/shock probe** · and — before believing *any* emergent result — **the CUBE CONTROL** (`DECISIONS[plate-tectonics-as-an-emergent-regime...]`): *if plate boundaries emerge along cube-face edges, that is the grid talking, and the result is void.*
>
> *Housekeeping: two router agents died on API errors (work survived; findings landed by hand). Some findings are double-recorded under different slugs — corroboration, but the log wants a tidy. `relata decide --choose` crashes with a Ruby backtrace. `tmptmp.md` is untracked and is not Claude's.*

**The ladder now says what to build next — it is no longer a matter of taste.**
Abyssal's `promise[emerged-land]` is *specified* (it has a falsifiable predicate)
but **nothing keeps it**, and it is a `:tag gate`. That is the **#1 gap**, and the
`uplift` nomos is the piece that must deliver it. Derived sea level (retiring
`SEA_LEVEL_M`) is its companion. Everything below is downstream of those two.

- **#10 — fine-tier erosion nomoi**: L21/L24 seeded from coarser tiles;
  walkable-scale ground through the store.
- **#11 — the first-person ethereal explorer**, over the query front-end
  (navigation and persistence fall out; UX intents inherited from the
  testbench era are in `TODO.md` §Explorer intents).
- Alongside: the **reservoir layer + thermal spine** (`mantle-thermal` +
  `climate-ebm` — the principling path for the crude `uplift` stub), the
  **water-system decomposition**, and the **kernel/scheme fix the grid report
  ordered** (below) — all specified in `TODO.md`.

## The real bet (don't lose it)

Axes 1–2 (graphics, world dynamics) are proven enough; the highest-**value**
frontier is **axis 3 (the ASF agents)**. Standing prerequisite before agents
step in parallel: the **RNG fix** — per-agent splittable seeds
(`ref/architecture-audit.md` #1). The world-model foundation earns its keep
because agents *live in* this coordinate/time/matter space. Hard gate before
any agent-seam work: the Level-C reading, `ASF.md` §5.

## ⚖ The grid — TENTATIVELY DECIDED (2026-07-13), and the word *tentative* is load-bearing

> **The decision (Joseph, `DECISIONS[the-grid-tentatively-decided-keep-the-cube-sphere-and-stagger-it]`):**
> **(1) KEEP the equiangular cube-sphere and `CellId`.** **(2) GO STAGGERED (Arakawa-C)** — scalars at
> cell centres, **fluxes stored ON FACES as first-class keyed objects**. **(3) CARRY THE METRIC PER CELL**,
> precomputed per tile (**not** per `(face, level)` — that claim in the grid report is false; a face-level
> plane at L19 would be ~23 TB).
>
> **It covers the coordinate system, the storage arrangement, and the metric machinery. Nothing else.**
> ⚠ **The ROUTER is explicitly NOT part of it and is NOT decided** — see the router spike below.
>
> ⚠ **This reverses the earlier posture in two ways, and both matter.** First, an earlier pass wrote the
> grid in as "CLOSED" on Claude's authority alone; that was retracted, and this section then said "NOT
> decided" for a day *after* Joseph had in fact decided — the same failure with the sign flipped. **The
> ledger is ground truth; this file tracks it.** Second, the *reasons* changed: the 2026-07-12 report
> reached the same conclusion from an argument that was subsequently refuted, and the surviving reasons
> make this a **positive case, not a least-bad option** (the quad C-grid's 2:1 DOF ratio admits no spurious
> computational modes; uniformity beats orthogonality — Putman & Lin measured it; the square C-grid is
> *privileged* for simultaneous energy + potential-enstrophy conservation, which a hexagonal TRiSK mesh
> provably cannot do; and 1→4 subdivision is what the entire store/LOD/memo-key architecture rests on).

**What would REOPEN it** (stated so a successor need not guess): (a) if the **leaf-only-evolution price**
proves unpayable against the store's independent-tier premise — flux-on-the-face makes the seam an identity
only if the coarse tier is *restricted from* the leaves rather than run alongside them; a **proposed
resolution exists** (`DECISIONS[the-macro-tier-has-two-roles-and-conservation-buys-its-freedom]` — the macro
tier's *governance* role is conserved and therefore free of its leaves, while only its *summary* role is
leaf-bound) but it is a **structural argument, explicitly NOT verified**; (b) if the **router cannot be made
principled on a quad mesh**; (c) if coarse-tier curvature needs a genuinely different treatment; (d) **Randall's
Z-grid**, whose one cost (an elliptic solve) a `CellId` quadtree may dissolve — **unmeasured, and its elegance
must not substitute for a benchmark.** Agenda + open points: `TODO.md` §grid.

**Also NOT covered and open:** the router · the hydrostatic-reconstruction failure when `Δz ≥ h` (**proved**,
and it is *thin films on steep ground* — our common case; it will masquerade as a seam defect) · the measured
AMR-seam-crossing-a-cube-panel-seam artifact (convergence 4 → 2.5).

**What was measured** (nine grids, `examples/grid_lab/` →
[`ref/research/grid-comparison-report.md`](ref/research/grid-comparison-report.md); Snyder
implemented from the paper and reproducing its own Table 1; Euler asserted at construction, so a
topology bug cannot silently become a physics result). These numbers reproduce; they are the part
that is solid:

- **Conservation is free.** Finite volume conserves exactly (~1e-15) on every grid, worst included.
- **Conservation ≠ consistency.** A *two-point* flux — the naive meaning of "FV with the true
  geometry" — is **inconsistent** on a non-orthogonal mesh: O(1) error that **grows** under
  refinement (order −0.5 on every quad grid). This supersedes the old "conservation is a scheme
  property / isotropy is a grid property" decomposition. ⚠ **And it is not a separate finding from
  the fan: MFD *is* the two-point flux** (Coatléven & Chauveau, read primary) — the two results are
  one result seen through two kernels (`DECISIONS[mfd-is-the-two-point-flux-and-our-router-is-not-d-infinity]`).
  Also measured, and it reframes the whole router question: **MFD's output is not a discharge at all
  — it is a boundary integral, ill-posed *in the continuum*** (its value depends on the shape and
  orientation of the cell boundary). *You cannot fix a scalar's direction.*
- **Correcting the scheme** (face gradient through the **mid-edge**, plus a *wide* quadratic
  gradient stencil over the Moore neighbours) measured **9.2e-1 → 3.6e-4** on our own grid, against
  the best hexagonal mesh's 2.2e-4 — with no change to `CellId`, the quadtree, the store, or the KRNG.
- **The 24 valence-3 cells are not a conservation failure** — every cell has exactly 4 *edges*, and
  all three routers (MFD included) conserve to 1.000000000000 there. The defect lives only in the
  diagonal fan, and the fan is inaccurate *everywhere*, not at corners.
- **The fan defect is a BIAS, it does not converge away, and erosion runs inside it** (2026-07-12,
  report §6a; `grid_lab` §9a). The earlier fan numbers were *two cells at L5* (~313 km) and were
  `|errors|` — erosion runs at **L19** (~19 m), and an absolute value cannot distinguish a bias from
  noise. Measured: the fan converges to the lattice **sheared by the map's Jacobian** — closed-form,
  **no resolution in it** (corner gap error 15.0° at L9, L19 *and* L23; L5→L19 shrinks cells 16 384×
  and moves it ~1°, *the wrong way*). It is a smooth **field**, zero at each face centre and rising
  outward — area-weighted **median 6.8°**, and only **37.8%** of the surface is under 5°. And it
  **accumulates**: on a cone whose exact flow lines are meridians, a plume ends up **474 km** off its
  meridian, with drift **rising** under refinement (4.00° → 5.75° from L6 to L9, where *noise would
  have fallen to 1.4°*) while the *spread* in the same runs converges away. The cube-face axes are
  attractors — **MFD reintroduces the grid-aligned-channel artifact it was adopted to remove.**
  ⇒ This *sharpens* the case for the kernel/scheme work and kills the "it washes out at fine
  resolution" escape hatch.
- **The corrected scheme is affordable — Joseph's innermost-loop worry is ANSWERED** (2026-07-13,
  `msc/spike-corrected-scheme-cost/`). It is an **exact algebraic refactor** (linear in `u`, static
  geometry), so the per-cell 5×5 solve collapses to a **21-point static stencil** precomputed per tile
  (verified 2.7e-13 against the solve): **+2.7% on an erosion epoch**, with 64² tiles sitting 50× below
  the cache cliff. *This retires the standing objection; it does not retire the router question.*

**Known weak points in the report, carried openly:** the **2500× is a Laplacian result, not the fluvial
kernel** (the routing gain is 4×); and one of the report's own causal explanations does not match its
harness (see `TODO.md` §grid, point 3). ⚠ **And its §4.2/§5 corner-anisotropy story is superseded** —
the corner was a red herring; the fan is inaccurate *everywhere*.

## A hard research problem, and it has no known method (open)

⚠ *Previously titled "THE one hard research problem." **Struck twice over (2026-07-13).** (1) The
superlative is a claim with no predicate, and it **suppressed competitors** — `vivium-operational-workflow`'s
BREAK-2 (*"certify Lawful" may be permanently unreachable*) touches the **moratorium's own
revisit-condition**. (2) **The statement below is ALSO stale**: the audit MEASURED the state-upscaling half
as exact and O(log N), and the surviving residue is **a nonlinear closure for a NON-LOCAL flux** —
`DECISIONS[wavelet-store-spiked-the-seam-is-not-the-details]` says to **RENAME it, not close it and not
leave it as written.** It has not been renamed. ⇒ **And we do not currently know what this project's open
problems ARE** — the audit dissolved some and created others, the BREAK list predates it, and **no count or
ranking should be trusted until the `core/` segments are laid down and the census is DERIVED.**

**detail→abstract**: upscaling an irreducible agent edit back into a memoized
macro with correct up-invalidation (`DESIGN-REDUX` §6, `doc/theory/multiscale-seams.md`
§2.4). Not on the ethereal-explorer path (a read-only explorer makes no edits);
plausibly the same shape as the open AAT identifiability bet. Everything else
has prior art.
