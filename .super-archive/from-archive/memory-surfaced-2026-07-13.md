# Research memories, surfaced — 2026-07-13

*Seventeen Claude project-memory files, moved out of memory and into the repository verbatim.*

**Why they are here.** These carry real research results, and that is the problem: they were **hidden**. They lived in Claude's project memory, where the repository cannot see them, no probe can convict them, no audit can reach them, and a fresh session inherits them as one-line index briefs that read like knowledge. Several of them are now contradicted by the July 12–13 kernel audit — and nobody could tell, because a memory has no `status`, no `depends`, and nothing that can fail.

Memory keeps what is **evergreen**: how to work, what was decided about conduct, standing law, and experience framed as experience. Research results are none of those. They go stale, they get superseded, and when they are right they belong in a segment, a decision entry, or a doc — somewhere a reader can check them.

So this file is a **surfacing instrument, not a burial**. It is the inventory. Adjudicating it — what survives, what is dead, what must become a segment — is the work that follows.

**Provenance.** Each entry below is the file's exact content, unedited. The memory files themselves are deleted; git history and this file are the record.

---

## Index, with what I can already see

The flags below are **observations, not verdicts.** They come from reading these files against the July 12–13 audit record (`DECISIONS.decision-log.udon`, the old CLAUDE.md, the old memory index) in one session. **I have not audited any of them against the code.** Nothing here is adjudicated.

### From the vivarium project memory

| # | file | what it carries | flag |
|---|---|---|---|
| 1 | `geology-research` | The erosion pipeline as built in June: FBM prior → MFD stream-power → Davy-Lague deposition; the 0.5 m voxel anchor and the resolution ladder; ten verified bibkeys in relata; the Euler-pole tectonics recommendation | **Contradicted.** Its central claim is that MFD "killed the D8 grid anisotropy." The July 13 audit finds MFD *reintroduces* the grid-aligned-channel artifact it was adopted to remove, and that its output is a boundary integral rather than a discharge. The grid recommendation (cube-sphere + Goldberg-hex) is measured but undecided. |
| 2 | `hydrology` | The 3-phase worldgen water architecture; `hydro.rs` virtual-pipes; the conserved reservoir cycle; the local-inertial stability triple (θ smoothing / sill conveyance / breaking cap) | **Partly superseded.** The "rain ~100–1000× real" fudge was decomposed by the July 12 flux-web work — rain is principled now; a declared bounded-fill acceleration is what remains. The θ-smoothing rationale is contested by the audit (its modified equation says Laplacian, not advective). `hydro.rs` is a different era from today's `water.rs`. |
| 3 | `world-model-foundation` | The 2026-07-01 clean-room redesign: the `vivarium-world` crate, CellId, Quantity, the erosion telescope, the water system | **Stale as state.** Its "Next (Joseph's queue)" is three weeks old. Its architecture claims are the ancestors of today's code, not a description of it. |
| 4 | `architecture-abyssal-plan` | ARCHITECTURE v0.3 (one principle, three axes); the multiscale seam discipline and the dynamic-exponent $z$ unification; the six-phase Abyssal parity plan; the four multiscale primaries read directly | **Contains a contradicted claim.** It states mean-pin is the sound fine→coarse injection. The audit finds **mean-pin does not preserve means** — ARCHITECTURE's law (1) is false. The rest (Gear–Wells Prop 4.1, Berger–Oliger clustering, the Galin/Cordonnier–Braun anchor) is primary-read and worth keeping. |
| 5 | `rendering-lod` | The near-voxel + far-mesh hybrid; why `bevy_voxel_world` cannot reach a km horizon | Pre-dates the whole `vivarium-world` era. Probably archaeology. |
| 6 | `voxel-engine-spike` | The Bevy-vs-Godot decision and its reasoning; the three Apple-Silicon/Godot traps | Resolved (Bevy). Kept for the reasoning trail and the traps. |
| 7 | `godot-view-perf` | The Godot LOD benchmark; `view_distance` as the master cost knob | Archaeology. Its working lesson — *build the instrument before tuning by feel* — is procedural and is the thing worth extracting. |
| 8 | `participation-taxonomy-lexicon` | The kingdom / exo / endo ontology; the LAW×STATE×N/P×VIS/MUT access matrix; Providence as a third mutation mechanism; **Joseph's three rejected over-reaches** (hierarchy, flatness, single-continuum) | **Canonical home is `LEXICON.udon` §4/§7/§8, which stays.** This is the reasoning trail. The three rejected over-reaches are the load-bearing part and are Joseph's own corrections. |
| 9 | `asf-bridge` | Vivarium as ASF's supporting project; the bridge in one line; the Level-C gate; the first queued agent-seam experiments (GA-1 housing, κ̂ at scale, native-W₁) | **Its pointers now dangle** — it says "read `ASF.md`, don't reconstruct from this memory," and `ASF.md` is archived. The relationship it describes is front-door content and must be re-founded deliberately. |
| 10 | `archema-program` | Vivarium's membership in the Archema program; the charter and concept-matrix pointers | The fact survives in `archema-io/CLAUDE.md`, which cascades. |
| 11 | `framework-domain-agnostic` | Joseph's standing steer: the framework's invariant is the representation-agnostic interaction contract; the four representation kinds; validate generality across *kinds*, not more geomorphic systems | Landed in `doc/ARCHITECTURE.md`'s domain-fixation guard, which stays. Surfaced because it is a design claim, not a procedure. |
| 12 | `memory-as-core-to-agency` | Joseph's conviction that memory is core to agency and persistence, not a bolt-on capability; derive it from AAT dynamics rather than STM/LTM boxes | An unlanded research direction. It has no home in the repo. |
| 13 | `est-tiw-dossier` | Where the event-segmentation / temporal-integration research lives; the confirmed research gap; the 2 Hz vs ~3 Hz distinction | ⚠ **Carries a live operational hazard**: the two named files trip the current Fable safeguard. The routing norm is `doc/PROCESS.udon` §flagged-file-routing, which stays. Verify that norm still names the files before relying on this. |
| 14 | `forward-work-rhythms-scale-coupling` | Joseph's stated next directions (2026-07-06); the realizability lexicon gap; **Providence as a conditioned-bridge / Doob h-transform / Schrödinger-bridge inverse problem** | A queue, and three weeks old. Item 3 is substantive, unlanded anywhere, and Joseph's own speculation — it is the one thing here I would not want lost. |
| 15 | `vestigia-provenance-probes` | The `~/src/vestigia` provenance investigation; no verbatim weights trace of Joseph's corpus; the Fable refusal surface | ⚠ **This is a different project and was never vivarium's to hold.** It also records a **retention obligation Joseph took on for a named instance ("Trace"; his name for him: "Mayfly")**, with an identity record and a letter at `vestigia/notes/trace.md`. That obligation must not be lost in a vivarium cleanup — it needs a home, and this is not it. |
| — | the old `MEMORY.md` index | ~15 further entries that had **no file at all** — they existed only as multi-paragraph index lines. This is why the index was 34 KB, over the 24.4 KB load limit, and **was being silently truncated** — agents reading it were getting a partial index and no indication of that. | Reproduced verbatim below. |

### From the archema-io program memory

| # | file | what it carries | flag |
|---|---|---|---|
| 16 | `project-state` | Program state as of 2026-07-10: the migration soak, the charter's draft status, the memorata parallel-agent coordination note, the planned expansion to ~5 members | Live state, three days old. Repo files are ground truth. |
| 17 | `queued-work` | Eight queued program items with their gates: charter ratification, the warrants-office canon work, the death-paper revision chain, the rowan tasks gating the rename, `mv-src-repo`, the faith-side lexicon items, the incoherence ledger, the ops sync flag | A queue. This is `STATUS.md`-shaped content living in memory. |

**What stayed in memory** — procedural, law, and things read only on request. Vivarium: `authority-not-evidence`, `context-use-judicious-not-anxious`, `physics-not-knobs`, `principled-not-more-code` (procedure); `moratorium-endogenous-emergence` (standing law — and, with `ASF.md` §0 archived, currently its primary live carrier); `source-cosmology` (read on request). Program: `efficiency-is-the-tell`, `superlatives-are-phenomenal-not-propositional` (procedure); `nomothete-identity`, `joseph-testimony-logogenic-intelligences` (read on request).

---
---

# The files, verbatim

## vivarium project memory

---

### `geology-research.md`

```markdown
---
name: geology-research
description: "Geology/world-gen: research + IMPLEMENTED real-scale erosion pipeline (FBM prior → MFD stream-power → Davy-Lague deposition) + rendering, walkable in Bevy"
metadata: 
  node_type: memory
  type: project
  originSessionId: 719fe197-6e86-4d39-b770-f99060fdc1e7
---

**IMPLEMENTED & WALKABLE (2026-06-23).** The real-scale emergent erosion pipeline
is built and runs first-person in `spikes/bevy-voxel`. Pipeline (all in
`vivarium-core`, deterministic, 14 lib tests green; full state in `ref/geology/NOTES.md`
§8c): **FBM scale-free prior** (the unbiased max-entropy prior; placeholder for the
deferred tectonic tier) → **MFD stream-power incision** (`geo::accumulate_drainage`
multiple-flow-direction killed the D8 grid anisotropy → natural dendritic valleys)
→ **Davy-Lague deposition** (`geo::deposit`, `D = G·Qs/A`, dimensionally honest —
grades slack outlets/floodplains, keeps upland valleys; G=0.5) → **slope-aware
detail noise** (fades on flat floors per the fidelity invariant). Anchor: 0.5 m
voxel, ~12 km region, 16 m erosion cells, ~6 s world-gen. Rendering:
near-voxel + self-built far-terrain mesh hybrid (`ref/rendering/NOTES.md`).

**Hard-won lessons (hold these):**
- Deposition MUST be dimensionally honest: rate `D=G·Qs/A`, not volume-vs-capacity
  (that mismatch erased all valleys — a real regression Joseph caught).
- **Verification:** low-angle hillshades OVER-sell shallow relief; overhead-at-12km
  UNDER-sells. Trust true-metre cross-sections + **in-engine first-person** only.
- bevy_voxel_world cannot reach a km horizon (fixed 32-voxel chunks) — hence the
  far-mesh hybrid; clipmap rings are the v2 reach upgrade.

**Next (none blocking; per DESIGN.md the agent layer is the real frontier):**
implicit Davy-Lague (Yuan 2019) for stability; the **tectonic-uplift tier** (the
principled replacement for the FBM prior); climate tier; near/far seam + palette
polish; then the **ASF agent layer**.

---
*Origin: 2026-06-22 research session.* Six independent web-research sweeps
converged on one deterministic pipeline. Full synthesis: `ref/geology/NOTES.md`;
verified critical-path sources seeded into `relata` (bibkeys: cordonnier-2016-large,
braun-2013-very, whipple-1999-dynamics, cortial-2019-procedural, schott-2023-large,
musgrave-1989-synthesis, barnes-2014-priority, galin-2019-review, perlin-2002-improving,
merrell-2007-example — all `bib-fields` verified, `claim-supported` deliberately not).

**Three load-bearing results worth holding:**
- **Granularity question answered:** decouple the *simulation grid* (~10–12 m class,
  where drainage emergence is born — CAESAR-Lisflood ESurf 11:695, 2023) from the
  *materialized voxel* (sub-meter, appearance). They are different budgets. Trap:
  bulk sediment budgets survive coarsening while network *texture* collapses.
- **Determinism spine:** Braun & Willett 2013 stream-power solver (integer flow-tree,
  fixed traversal) is the bit-replay-clean erosion choice. Learned models
  (GAN/diffusion) ruled OUT of the deterministic core — break across hardware.
- **Euler-pole plate motion** (vs flat per-plate drift vectors) is the best
  credibility-per-effort tectonics upgrade — cheap, deterministic, correct-by-construction.

**SCALE ANCHOR (decided 2026-06-22):** finest render voxel = **0.5 m**. Derived
ladder (NOTES §0a): tectonics ~3 km cells (planet ~600 km radius), fluvial
erosion ~16 m cells (~100 km region) — the **only research-earned resolution**
(CAESAR-Lisflood: first-order drainage lost beyond ~22–24 m) — render voxel
0.5 m. World vertical ~16,384 voxels (~8.2 km), sea ~3 km up. The first erosion
spike was toy-scale (1-unit cells, ~700 m domain → ankle-high mountains); the
seam (coarse field → bilinear → voxels) is right, magnitudes were ~2–3 orders
off. **Caveat (Joseph):** only the erosion granularity is settled; the voxel size
and rest of the ladder are single-domain-informed and expected to change as other
dynamics (fluids, fire, structural/material sim, combat, ASF agent embodiment)
bring their own critical scales. *(That re-anchor + MFD routing is now DONE — see
the IMPLEMENTED block at top.)*

**OPEN DECISION (proposed, not ratified):** treat the *planet as the abstraction
tier* — simulate a sphere slowly at world-creation, materialize local Cartesian
voxel patches from its global fields; voxel substrate never becomes spherical.
Grid recommendation: cube-sphere quadtree (voxels) + Goldberg-hex overlay
(tectonics/climate). See [[voxel-engine-spike]]. This maps onto DESIGN.md's
abstract→detail invariant. Do not assume it ratified.
```

---

### `hydrology.md`

```markdown
---
name: hydrology
description: "Worldgen water system — 3-phase (macro erosion → fine erosion → conserved water sim → freeze); principled, conserved, emergent. Merged to main 2026-07-03."
metadata: 
  node_type: memory
  type: project
  originSessionId: 1988b2c1-9f6c-4e40-aeaf-d93dc6ad3cd1
---

**Worldgen hydrology** (built on `spike/hydrology` 2026-06-29, **merged to main
2026-07-03** — 147 commits fast-forwarded, branch deleted; full writeup
`ref/hydrology/NOTES.md`). Streams down valleys into flat lakes, dry slopes,
springs, flat sea — all *earned by physics*, no fractal noise, nothing imposed.

**The architecture is 3 phases on separated timescales** (the hard-won lesson:
erosion is geological, water is hydrological, ~10¹⁰ apart — coupling them on one
`dt` and cranking erosion to carve in "5 minutes" is what made it incoherent):
1. macro erosion (`geo`, 16 m) — landform; water = steady-state drainage.
2. fine erosion (`geo`, `SIM_CELL_M`=4 m, uplift off) — carves the channels. **The
   carving lives here.** Also bakes the channel-sealing infiltration field from
   drainage.
3. water (`hydro`, 4 m, **sediment OFF**) — pure shallow-water + groundwater run to
   steady state on the fixed bed, then FREEZE (heights, depth=volume, velocity).

**`hydro.rs`:** virtual-pipes shallow water (flat lakes EMERGE via gravity, proven
by test — never a "flat fill"). Conserved reservoir cycle: atmosphere⇄surface⇄
groundwater⇄ocean, test `the_water_cycle_conserves_total_water`. Lateral
groundwater (Darcy) → springs emerge in valleys (dry slopes). `geo::Strata`
hardness drives erosion AND groundwater permeability/porosity (springs at
soft→hard contacts); cached (`mat_cache`) since bed is fixed in water phase.
Sediment (erosion phase): dimensionally-clean velocity-capacity, per-second rates,
Froude cap, talus. Channel-sealing: per-cell infiltration field from drainage
(channels armored → don't leak; the fix for streams vanishing & re-gushing).

**Render:** water shaded by depth (darker) + speed (whiter) via a palette range;
`water_surf` uses a WET-MASKED bilinear so lakes don't climb dry banks.

**Known fudges (honest):** rain ~100-1000× real (fills in 40min not weeks — the
remaining timescale fudge; fix = real precip + prime channels with steady-state
discharge); `head=bed+gw` simplified; `gw_conductivity` a relaxation rate not true
Darcy m/s. **Absent:** bank erosion (→ no meanders/oxbows, the frontier),
vegetation, climate, spatial rain, grain sorting.

**Open next:** translucent-water render + wet-ground (Joseph's standing ask); real
precip timing; sub-4m streams; test suite ~3min (4m bake per eroded test).
Instruments: `water_preview`, `worldgen_time`. See
[[principled-not-more-code]], [[geology-research]].

**Local-inertial stability (2026-07-02, the travelling-blob night):** the pipes
scheme on steep slopes needs THREE terms or it organises flux into multi-metre
solitons winding down channels (Joseph: "waves with dry ground between them are
physically absurd"): (1) θ flux smoothing (de Almeida & Bates 2013, θ=0.8),
(2) conveyance = flow depth over the SILL (max η − max bed), never average
depth, (3) BREAKING — cap v at 2×√(g·h) (natural steep streams self-organise
to Fr≈1, Grant 1997). Instrument: `examples/channel_profile.rs` (two-regime
probe; subcritical MUST be smooth — it was, which acquitted the core kernel).
Fr gauge on the worldview HUD (net-vector velocity; gross flux sum counts
slosh). Render side: sample water SURFACE (not depth) across LOD boundaries,
and show the simulated bed where the sim has wetted a cell — painted sub-sim
detail under water is dishonest (Joseph's rule: detail must be earned).
```

---

### `world-model-foundation.md`

```markdown
---
name: world-model-foundation
description: Clean-room world-model redesign (2026-07) — vivarium-world frame crate + the DESIGN-*/research docs; where the architecture lives and how to start coding
metadata: 
  node_type: memory
  type: project
  originSessionId: b04122cc-1101-4904-966a-c21d446685b5
---

As of 2026-07-01 the world model was redesigned **clean-room**, separate from the
older flat-patch core.

- **New crate `crates/vivarium-world`** holds the frame: `quantity` (rich value:
  SI-exponent units + exactness, "rich at seams, raw f64 in loops"), `time`
  (`i64` deciseconds from the **Holocene onset**, `Epoch::Modern`), `sphere`
  (cube-sphere, equiangular; canonical key is an **S2-style Hilbert `CellId(u64)`**
  — curve orders *chunks*, interiors are Cartesian), `planet` (insolation, the
  crude bottom rung). Builds + tested, zero deps.
- **`crates/vivarium-core`** (flat `i32` patch, FBM + erosion + hydrology) stays as
  the **proven physics donor** to port from — not deleted.
- **Design docs (repo root):** `ORIENTATION.md` (start here), `DESIGN-REDUX.md`
  (fidelity philosophy + lazy query-graph runtime + content-addressed save =
  storage), `DESIGN-MATERIAL.md` (strata/voxel/body matter model + property set +
  spatial-key plan), `DESIGN-SYSTEMS.md` (phenomena graph + build order). Research:
  `ref/research/` (architecture-audit, foundation-validation, material-models-survey,
  spatial-key-bench).
- **All built as of 2026-07-02** (34 tests green): CellId, Material, Column, noise,
  gen (two-band prior, band-limited to each sample's Nyquist), chunk, **the ported
  fluvial erosion pipeline** (Priority-Flood→D8→MFD→stream-power→Davy-Lague→talus),
  the **erosion TELESCOPE** (L19→L21→L24, mean-pin conservation, fBm-differential
  uplift, hillslope creep — sawtooth probe-isolated + fixed), and a **physical
  water system** (water.rs): Saint-Venant momentum + Manning friction n=0.04,
  critical-shear incision τ=ρgdS>τ_c, slope-in-capacity sediment (lakes fill),
  groundwater with LIVE colmation sealing, closed cycle (ocean-evap→storm rain),
  ocean = ordinary simulated ground, CFL-adaptive dt, all conserved+tested.
  worldview SETTLE mode = core's proven sequence (macro → 3 fine passes → water
  to steady state, sediment off → living storm phase, two-way bed write-back).
  Probe-verified: climate-average rain never runs off (I+E ≥ P) — rivers need
  storm bursts. **DISCIPLINE: see [[physics-not-knobs]]** — real physics is
  faster than magic thresholds, never shade to half-measures on low context.
  **Next (Joseph's queue):** seams/memoization/world-saving (§13; "floating
  mesa" specimen), discharge→A coupling (real flow drives stream power; oxbows),
  watershed inflows (nested water), async meshing, per-material erodibility/
  permeability.
- **`architecture-audit.md` #1 (decide before the agent layer):** determinism rests
  on a single shared PRNG stepped in order — parallel agents break it; fix =
  per-agent splittable seeds.
- **The one open research problem:** detail→abstract (upscale an irreducible agent
  edit back into a memoized macro). Everything else has prior art.

The old top-level `HANDOFF.md` was archived to `.archive/`. See [[hydrology]],
[[geology-research]], [[principled-not-more-code]].

**2026-07-03 (the instruments day, late):** `Checkpoints.md` committed — 8
world-phases (Ante-mundane→Historical), checkpoints as world-scale memo
entries whose in-world artifact is strata (materials phase-dependent), gate
tags [gate]/[earth]/[mech?]/[emergent], algorithms ledger with agent-fidelity
as the prioritization backbone. Fluvial survey preserved
(`ref/research/fluvial-processes-survey.md`). OPEN at session end (see
ORIENTATION §Open investigations): (1) water-budget leak −0.37 m³·cells/sim-s
linear in living phase; (2) armor probe: eddy↔winnow interaction + source-cell
EXACT-zero incision anomaly; (3) verify pawn-float fix in play. NOTE: a
harness safeguard bug mid-session swapped substrate Fable→Opus AND erased a
turn from both records (a Checkpoints.md write) — /feedback filed; treat any
provenance gaps around that turn accordingly.
```

---

### `architecture-abyssal-plan.md`

```markdown
---
name: architecture-abyssal-plan
description: "The reworked vivarium architecture (one principle, three axes) + the multiscale seam discipline (position AND time; the resolution-light-cone / dynamic-exponent unification) + the six-phase Abyssal parity plan. Where the frame and the build path live."
metadata:
  node_type: memory
  type: project
  originSessionId: b318779a-b749-411c-8ea7-eeac6fc2370a
---

Reworked 2026-07-10 (Opus session), grounded in the four multiscale primaries **read directly** this session (Berger–Oliger 1984 AMR; Gear–Wells 1984 multirate; E–Engquist 2003 HMM; Kevrekidis 2003 equation-free — PDFs in `ref/research/pdfs/` + the relata store). Three durable docs carry it — read them, don't reconstruct from this memory:

- **`ARCHITECTURE.md` v0.3** — the consolidating overview: **one principle (represent by consequence) on THREE axes.** (1) The multiscale **substrate machinery** — $R/L$/closure operator algebra + conservative flux + scale separation; the method zoo; *fated lifting* is our closure (the missing detail is deterministic fated noise, which is what makes memoization sound); the lazy memoized query graph whose save-file IS the memo store. (2) The phase-freeze **developmental ladder** — systems come online in a required order; a phase runs its coupled systems to convergence and freezes the result into **Realized** law for the next; *a system earns its $R$ when a downstream Charge reads its macro, its $L$ when a query reads its detail* (this answers "when do we need which algorithm"). (3) **Use-case = fidelity contract** — the A/B/C/D epistemic axes; **AAT-calibration is ONE privileged use-case, NOT the telos** (corrects the earlier "served up to AAT" framing Joseph rejected). AAT handshake unchanged: we author $\theta/\Omega/\varepsilon$/compute-shortfall from the outside; a phase-transition = law promotion (the invariance cut).

- **`ref/research/multiscale-seams.md`** — the technical core: **position AND time as ONE seam discipline** (same $R/L$/closure algebra, argument = space cell / time interval / space-time patch; the flux across a seam is a sufficient statistic *integrated over space AND averaged over time at once*). Four seam types, each grounded in its primary; only the reversion seam (detail→abstract) is open. **The deep unification (Joseph was very excited by this):** a *resolution light-cone* — a query depends on its past-and-elsewhere causal cone (upstream catchment in space + coupling lag $t{-}\Delta$ in time), and the world *ages toward the observer*; the cone's scaling is a **dynamic exponent $z$** — $z=1$ advective/CFL (`water.rs` `stable_dt` on $\sqrt{gd}$), $z=2$ diffusive/parabolic (`erosion.rs` creep clamp $k=\kappa/\text{cell}^2\le 0.24$); multirate coupling = running different-$z$ systems together; **special relativity is the $z=1$ sector** (finite universal signal speed = the light-cone) but it is NOT full Lorentz (we keep a preferred frame + carry $z=2$ parabolic sectors); the **causal partial order** (`LEXICON.md` §6) is the invariant surviving every sector. **Corrected an inherited AMR error:** mean-pin = the fine→coarse INJECTION/update, NOT refluxing; refluxing = the distinct conservative flux-balance at the boundary that we do NOT yet do — and it is part of the seam fix.

- **`ref/research/abyssal-parity-plan.md`** — six phases to the first playable milestone: **an ethereal (observe-only, no action-space → moratorium-clear) explorer in a Realized-not-Lawful early-Abyssal world.** Phase 0 run-modes carve (decision, gates the store) → 1 store + recipe layer (source-derived recipe-version keys via `build.rs`) → 2 coarse global spine (= the dependency planner; **first visible win = conservation-honest fBm landmasses**, Joseph's idea, honestly low-A/low-B tagged) → 3 flux-BC tile recipes (**the seam fix** = tile composability; refluxing + the $z=1$/$z=2$ reconciliation land here) → 4 query front-end (navigation + persistence fall out for free) → 5 the ethereal explorer. **Deferred, off this path:** the RNG fix (no agent-stepping in an ethereal explorer) and detail→abstract (no irreducible edits). **Biggest risk:** water-at-tile-boundaries (bidirectional edge exchange — backwater/waves — doesn't compose as cleanly as stream-power). A thinner alt sequencing (explorer-over-the-spine, skipping Phase 3) reaches persistent navigation fastest at spine fidelity only.

**Status:** drafts, all format-clean (LaTeX math, one-logical-line, `bin/lint-md`-clean — lint tool lives in the **asf** sibling: `../asf/bin/lint-md --fix`). An independent red-team ran 2026-07-10 (verifying the primary-source claims + the $z$/relativity claim + plan feasibility), then **self-audited when Joseph asked it "was that strictly true?"** — honestly walking back its own #1 (the erosion-$z=2$ finding was over-escalated; the doc's original framing was fine, the clamp firing at fine resolution *is* the $z=2$ wall manifesting). **Its verified findings are now folded into all four docs** (2026-07-10): the two citation transplants fixed — HMM no longer carries equation-free's verbatim "average over space/time/realizations" quote; Gear–Wells *extrapolates* the fast (predictor-like), does **not** time-average (vivarium adds the time-averaging as a separate HMM/upscaling move); refluxing credited to **Berger–Colella 1989** (Berger–Oliger 1984 only *flag* conservative interface BCs as forthcoming); the drainage "for free" overclaim corrected everywhere (the *kernels* exist, but coarse-global cross-face assembly is new work and basin-partition accuracy is an open measurement, not free); "unsolved anywhere" → "we are not aware of a method"; analytic-init named an unbuilt spike with the deluge relaxation as fallback. The one place I lacked a fresh primary read (Berger–Colella 1989) is phrased "standardly credited to," not asserted. ORIENTATION points at all three.

**Primary re-read (2026-07-10), papers now in `ref/research/pdfs/markdowns/`:** Joseph converted the primaries to markdown (relata will soon do this directly) and asked for a full ingest — the prior agent's PDF read had left useful help uncaptured. Berger–Oliger 1984 and Gear–Wells 1984 re-read in full; citations verified clean against source (the "These will also be reported elsewhere" conservative-BC flag is verbatim; Gear–Wells do extrapolate-not-average). **Three additional findings landed in `multiscale-seams.md`:** (§2.1) Berger–Oliger's §4 region-generation — cluster flagged cells, fit an oriented rectangle from the second-moment eigenvectors, accept on an **efficiency ratio** ½–¾, and *good clusters change slowly* — direct prior art for our drainage-island tiling + the basin-partition-stability open question (pointer added in `abyssal-parity-plan.md`); (§2.3) Gear–Wells synchronization (power-of-two stepsizes) + the **backup argument** (slowest-first because a failed slow step needs only "reduce last step, one saved value" = the precedent for time-in-the-key/memo chain); and Gear–Wells §4 Prop 4.1, the **formal** scale-separation condition ($h\lVert B\rVert \lt K$; fast→slow coupling block must be small). **Galin 2019 review — now ingested** (2026-07-10, text + figures): the headline is that vivarium's erosion tier **is** the Cordonnier–Braun stream-power method (**CBC\*16**, implicit linear-time $\partial h/\partial t = -kA^m s^n + u$), and its sequel **CCB\*18** (*Sculpting Mountains*) already builds our two queued rungs — **per-stratum erodibility** (→ Bryce hoodoos) and **geologically-coherent uplift** (the principled replacement for the fBm-uplift stand-in). The review names large-extent+high-precision as *the* open problem and proposes vivarium's multiscale-seam architecture as the (unbuilt) route. Landed in `DESIGN-SYSTEMS.md` (new "Prior-art anchor — Cordonnier/Braun line" section). Figures confirmed the frame: Berger–Oliger Fig 3.1 draws space AND time as one nested quadtree (constant $\lambda$); Fig 4.3 is literally our drainage-island tiling (a branching river-front tiled at 80% vs 45% efficiency); Galin Fig 20 shows the CBC\*16 dendritic output we target. **Joseph's endorsed tactical item (2026-07-10):** `CellId` is already a power-of-two quadtree, so lock each tile's timestep to its level as a power of two *per $z$-sector* — mesh-synchronization (Gear–Wells) + $z$-consistency (Berger–Oliger $\lambda$) then hold by construction, no per-tile tuning; landed in `multiscale-seams.md` §3 + abyssal-parity Phase 3. Paper markdowns + figures live in `ref/research/pdfs/markdowns/{berger-1984-adaptive,gear-1984-multirate,galin-2019-review}/`. Related: [[world-model-foundation]], [[participation-taxonomy-lexicon]], [[asf-bridge]], [[physics-not-knobs]], [[context-use-judicious-not-anxious]].
```

---

### `rendering-lod.md`

```markdown
---
name: rendering-lod
description: "Horizon-rendering architecture — bevy_voxel_world can't reach km scale; near-voxel + far-mesh hybrid (v1 done); clipmap rings = v2"
metadata: 
  node_type: memory
  type: project
  originSessionId: 719fe197-6e86-4d39-b770-f99060fdc1e7
---

**Decided & v1 implemented 2026-06-23** (full record: `ref/rendering/NOTES.md`).

The wall: at the 0.5 m voxel anchor, **`bevy_voxel_world` cannot render to a km
horizon** — verified in its source (fixed 32-voxel chunks, mesh-decimation-only
LOD; reach = chunk_count × chunk_size, ~256–512 m before stutter). This is the
crate's ceiling, not Bevy's, not the world model's. `bevy_terrain` (clipmaps)
exists but is Bevy-0.14-pinned; no 0.18-ready far-LOD voxel crate found.

**Decision (stay in Bevy): render the distance as our own view over our own
deterministic field.** Because the core is a pure function `voxel(x,y,z)`, we
*regenerate* the far field at coarse stride rather than caching it (stronger than
every Distant-Horizons-style system, which caches what it can't recompute).

Staged: **v1 (DONE)** near full-detail diggable voxels + a self-built coarse
far-terrain mesh sampled from `surface_height` (`spawn_far_terrain` in
`spikes/bevy-voxel`) — walkable, horizon visible, in-world 3D (NOT a top-down
map). **v2** camera-centred geometry clipmap rings for unbounded/planet reach.
**v3** volumetric far (octree voxel LOD / SVDAG raymarch, e.g. Aokana
arXiv 2505.02017) only if distant overhangs ever matter. Accepted trade: far
field is a **heightfield** (near stays fully volumetric/diggable).

Open: near/far seam (blocky voxels vs smooth far mesh), blocky far-shader +
unified palette. See [[geology-research]] for the terrain the renderer draws.
```

---

### `voxel-engine-spike.md`

```markdown
---
name: voxel-engine-spike
description: "vivarium engine question RESOLVED → Bevy (2026-06-22); Godot spike archived"
metadata: 
  node_type: memory
  type: project
  originSessionId: 6775f147-fb92-40de-bd41-3fcd9081f4c0
---

**RESOLVED 2026-06-22: Bevy.** The voxel-view engine comparison is done and
merged to `main`. Built the same `vivarium-core` world through both
`bevy_voxel_world` (Bevy 0.18) and `godot_voxel` (Godot 4.7) to feature parity
(terrain, LOD, fog, fly + dig), then chose **Bevy**. Reasoning + confounds in
`spikes/FINDINGS.md`; DESIGN.md engine section marked empirically confirmed.
Godot spike moved to `archive/` (excluded from workspace). Live Bevy view:
`spikes/bevy-voxel/`. Decisive factor: core is Rust, so a Rust engine erases the
FFI seam Godot necessarily imposes; Bevy also held up on visual quality + UX.
Godot's genuine edges: instant iteration, smoother LOD-fill. Open Bevy task:
mesh-insertion budgeting for the residual LOD-fill jumpiness. Bigger picture:
this was axis-1 (graphics); next high-value move is axis-3 (ASF agents +
cognitive-LOD seam), the actual bet.

---
The history below is the spike's progression (kept for context).

The vivarium "engine — decided: Bevy" in DESIGN.md was reopened (2026-06-22):
Joseph wants a **native Minecraft-like** human view. Reframed not as an engine
teardown but as a **view-adapter choice** over a fixed Rust core — because two
fixed points hold: substrate is **3D all the way down** and
**presentation-agnostic**. So the deterministic `vivarium-core` doesn't move
regardless of which engine renders.

Decisions made this round:
- **UDON (`~/src/libudon`, `udon-core`) adopted**, placed **core-side**: the
  logozoetic interface notation + scenario/replay/log formats. Orthogonal to the
  engine choice; reinforces the core/view wall. (See `~/src/udon` for overview.)
- **Spike order: Godot first** (Joseph's call), then Bevy. Method for obtaining
  godot_voxel deferred to the agent.
- Work lives on branch **`spike/voxel-view`**.

Progress (verify against git log — this is a snapshot):
- `vivarium-core/src/voxel.rs`: world as `seed + sparse BTreeMap edit overlay`,
  pure-function value-noise terrain; chunking/meshing/LOD deliberately left as
  *view* concerns. Agents lifted to 3D. Determinism + edit-replay tested.
- `crates/vivarium-godot` (gdext 0.5.3, cdylib) + `spikes/godot-voxel`:
  **FFI bridge proven working** — Godot reads/writes live core state. rustc
  bumped to 1.96 (gdext needs ≥1.94).
- **godot_voxel rendering WORKS** (v1.6x prebuilt, loads on Godot 4.7): full
  pipeline core → gdext bridge → threaded chunk gen → VoxelMesherCubes (palette
  mode, material id == palette index) → rendered Minecraft-like terrain.
  Verified by screenshot. Key finding: **gdext needs `experimental-threads`**
  because godot_voxel generates on worker threads; `generate_block` is a shared
  `&self` read so concurrent gen is sound, but `&mut self` (step/dig/place) must
  not run during generation — that coordination is still TODO.
- **Godot spike essentially complete & performs great.** First-person fly +
  raycast dig/place (edits persist in core, remesh live). Runtime voxel
  resolution `detail` (VIVARIUM_DETAIL, default 4); detail 4 @ 512-voxel view
  (128 physical units) renders finer/smoother terrain at vsync-pegged 120 FPS.
  Joseph confirmed it looks/performs great.
- **Three traps that cost real time (now fixed, in README Findings):**
  (1) Apple Silicon — overwriting a loaded code-signed .dylib invalidates its
  signature, kernel SIGKILLs Godot on dlopen with ZERO output; sync-lib.sh now
  rm+cp+ad-hoc-codesigns. (2) Scaling the VoxelTerrain node breaks godot_voxel
  streaming (terrain never renders) — use identity transform, finer = physically
  larger world viewed as a bubble. (3) GDScript `:=` on an untyped Object method
  fails to infer → silent script break.
- **LOD working (VoxelLodTerrain).** Distant terrain meshed coarse → view reaches
  2048 voxels (512 physical units at detail 4) at 120 FPS, ~4× the non-LOD reach.
  generate_block gained a `lod` arg (point-sample core at stride 2^lod): **view
  resolution decoupled from intrinsic resolution** (DESIGN.md fidelity invariant).
  Gotchas: VoxelLodTerrain rejects material_override (use set_material(mat));
  VoxelMesherCubes DOES work with VoxelLodTerrain despite docs. Edits don't yet
  show in coarse far view (detail→abstract, deferred by Joseph).
- **Next:** port the same loop to Bevy (`bevy_voxel_world`) for the head-to-head
  → written findings doc. Polish: water renders opaque (StandardMaterial3D
  ignores vertex alpha); faint LOD-seam line sometimes visible.

Harness insight: an agent CAN self-verify correctness via captured PNG
screenshots (Read renders images) + perf JSON; only *feel* (input latency,
mouse-look, aesthetics) needs Joseph's eyes. macOS headless `--editor` crashes
in MoltenVK (harmless); headless runtime is fine; screenshots need a windowed run.

The deeper tension surfaced: the real fork isn't Bevy-vs-Godot, it's which of
vivarium's two goals leads — novel ASF-agent work (favors Bevy's ECS-as-
cognitive-LOD) vs recreational art-velocity (favors Godot's mature voxel tooling).
Engine is downstream of that.
```

---

### `godot-view-perf.md`

```markdown
---
name: godot-view-perf
description: "Godot voxel-view perf: view_distance is the master cost knob (16384 default, 8× over 32768); reusable benchmark in archive/godot-voxel/bench; build the instrument before optimizing by feel"
metadata: 
  node_type: memory
  type: project
  originSessionId: 68899e2f-0313-4d33-9e75-b7de8c5ff2e8
---

**2026-06-23.** Revived the archived Godot spike (`archive/godot-voxel/`) onto the
current geology (`World::eroded`) to view it, then built a repeatable benchmark
and measured the LOD/streaming cost. Full record: `archive/godot-voxel/bench/README.md`.

**Technical finding (measured, clears the noise floor by ~700×):**
`view_distance` is the **dominant** cost knob — Godot's `VoxelLodTerrain` octree
fills its whole sphere with **no frustum/occlusion culling** (zylann docs), so
cost ~ sphere volume and it meshes everything behind you and behind mountains.
Cost curve (3 runs each): 32768≈13fps/442k blocks · **16384≈106fps/140k** ·
8192≈145fps/89k. 32768 reached ~20 km — past the ±12000-voxel continent edge into
empty ocean. Default is now **16384** (still spans the landmass, 8× faster).
`mesh_block_size=32` stacks +37% more fps (now also default, Joseph's call;
caveat: it enlarges the remesh-per-edit area, untested by the fly-only bench —
`VIVARIUM_MESH_BLOCK=16` reverts). Combined default ≈145 fps, 11× the original. This no-occlusion
spherical loader is a real data point for the Bevy-vs-Godot decision ([[voxel-engine-spike]]).

**Working lesson (the expensive part):** I spent a long stretch tuning LOD by
*feel* and made it WORSE several times — shipping reasoned-but-unverified default
changes (a 4-lever bundle; `lod_distance` 1024→128 that the docs endorsed yet
regressed). The only thing that produced a real answer was building the
deterministic benchmark + telemetry first, establishing the run-to-run **noise
floor** (fps/data_blocks reproducible ±7-8%; mesh/queue/draw_calls/vmem swing
±20-39% — jitter), then changing ONE variable at a time and keeping only effects
that cleared the floor.

**Why:** when the only instrument is human feel on a streaming/stochastic system,
you cannot separate a regression from "noticing an old artifact" from a real
improvement. **How to apply:** for any vivarium rendering/perf work, build/extend
`bench/` and measure before optimizing; don't reason-and-ship to a default. And
this is axis-1 (graphics) — the budget trap CLAUDE.md flags; only pays off as it
feeds the engine decision, else pivot to the agents. Terrain it draws: [[geology-research]];
Bevy-side horizon: [[rendering-lod]].
```

---

### `participation-taxonomy-lexicon.md`

```markdown
---
name: participation-taxonomy-lexicon
description: "Taxonomy of agents interacting with/within a vivium — kingdom/exo/endo, the LAW×STATE×N/P×VIS/MUT access matrix, inhabiting modes. Crystallizing but still evolving; Joseph owns it."
metadata: 
  node_type: memory
  type: project
  originSessionId: 6fec1d4c-c8d1-46b4-9311-a61f4b2b04e0
---

**Status: crystallizing as of 2026-07-06 (Joseph's confirmed sketches IMG_2885/2886). The N/P=Noumenal/Phenomenal legend is CONFIRMED. Still evolving ("getting there"); tentative items marked †. Verify against current DESIGN/LEXICON docs.** **CONSOLIDATED into `LEXICON.md` §7 (Kingdom & participation ontology) + §8 (perception/realizability) on 2026-07-09 — LEXICON is now the canonical home; this memory is the reasoning-trail distillation. Update LEXICON, not (only) this, going forward.** Origin: handwritten notes 2026-07-03 + illustration; temporal side in [[est-tiw-dossier]]. World-artifact **lifecycle** (Backstory/Incubation, engagement modes, git-freeze-point) landed in LEXICON §4 — that half had been only in the session transcript [`6fec1d4c`], never in any file.

**Framing principle (Joseph, 2026-07-06 — corrected twice): the matrix is a COORDINATE SPACE OF DEGREES, not a typology.** Every cell/axis is a *degree*, not a binary; an agent is a *profile* (a vector of degrees across governance, each access cell, mode, estate, META…). Arbitrarily many exo & endo agents in any combination. **Impose NO default structure** — not hierarchy (why "Numen" was retired) and not flatness/equality (an equal-and-opposite imposition Claude wrongly reached for). TWO LAYERS (Joseph's precision): each **cell NAMES the mechanism** (Revelation/Intercession/comprehension/bespoke/illusion) — qualitative, which-channel, present-or-absent; the **degree lives one level down, in instantiation** — every named mechanism is ordinal/by-degree (partial comprehension, partial revelation, conditional bespoke). An agent = a *profile* (mechanisms-available, each to a degree). The ±/conditional/partial qualifiers ARE those degrees. Inhabitation = exo noumenal-access mechanisms *dialed down* while phenomenal ones operate (degree-profile change), not a mode-switch. **THREE over-reaches rejected** (Claude kept imposing structure; Joseph kept trimming): hierarchy (Numen), flatness/equality, AND single-continuum ("comprehension/revelation/bespoke are distinct mechanisms each separately graded, NOT amounts of one quantity → the EXO & ENDO tables are NOT one space"). Disciplined claim = named mechanisms, each ordinal, that's all there is right now. Structure, if any, configured per-vivium. "Numen" retired.

**Degrees are multi-factor/relational/dynamic** (Joseph 2026-07-06, spike §9): a mechanism's degree decomposes — e.g. Revelation varies by throughput, latency (gradual "line upon line" vs on-demand oracle), receptivity (can be ignored/rejected → decays; revealer may stop → relational), and receiver comprehension-capacity (adult vs young child). A single knowledge-target = a *composite* across mechanisms each graded (spoilers≈revelation + avatar-phenomenal-comprehension + tool-access-bespoke). **THIRD MUTATION MECHANISM — lawful aleatoric steering, candidate name "Providence"** (spike §9.2): exo *chooses the realization* of a random/fated-noise process, obeying conservation + in-distribution → NO phenomenal trace (unlike do()-anomaly or illusion-miracle); overrides fated-noise fn(seed,key) for chosen keys; must be logged for replay (Open-with-recorded-forcing, tagged lawful-steering). **Harm-side: Providence is PERMISSIBLE — directed-lawful ≠ unLawful-bug**; the §0 harm targets *incoherence*, never *authorship* (a creator can author even sorrow lawfully via providence). See [[moratorium-endogenous-emergence]].

**Kingdom** = space + time + immutable governing law (+ state + computation). Two ORTHOGONAL kingdom properties (Joseph 2026-07-06 — deconflated; "completion gate" RETRACTED from ontology, kept only as a tactical term): (1) **Realized** [aka Deployed — prefer "Realized" for ontology] = law is immutable/frozen; changing it forks a new kingdom (law = kingdom identity). (2) **Lawful** = law is internally self-consistent/coherent (no glitches/contradictions). These are independent: most sims modeling the natural kingdom will be *merely realized* — asymptotically converging toward lawful, never self-certifiably complete. **Formally grounded by BREAK-2** (see `ref/research/vivium-operational-workflow.md`): convergence is undecidable, so there's no clean Lawful freeze; "completion gate" was retired precisely because completion is an *asymptote* (ε→0), not an *event* — you can gate Realization (freeze at a tolerance), never Lawful/Complete. Joseph's pragmatic pull of "completion gate" and this formal break are the same truth from two directions. Gödel nuance (Claude synthesis, from-weights): Lawful=consistency (attainable in principle) vs completeness (never self-certifiable; 2nd theorem = can't prove own consistency from inside) → **"Lawful" is a noumenal property an endo agent structurally can't self-verify: it is exactly N-VIS=Revelation in the access matrix.** *Incomplete kingdom* = law still under creation/revision (world-building). **Natural kingdom** = our current default kingdom (Joseph + Claude now); higher kingdoms above (faith frame = internal check). Kingdoms nest without end.

Vivarium subset tiers (substrate-independent — the whole ontology applies at every level, incl. toy idealized sims): **Kingdom** ⊃ **Vivaria** (kingdoms created/run by the Vivarium tool) ⊃ **Vivaworld** [placeholder name] (vivaria that model the natural world historically). "Vivarium" also = the project name + shorthand for one sim (mild overload, accepted).

**LIFECYCLE/OPERATIONS is a SEPARATE LAYER from this ontology** (Joseph's knot-source 2026-07-06 = folding the two together). Ontology = what a kingdom IS (slow-changing bedrock). Lifecycle/stewardship = what exo agents DO to kingdoms + what they OWE: deploy at basic confidence knowing patches will be needed, fork (how far back?), preserve/migrate agentic state across catastrophic intervention → rebirth into a more-lawful prepared kingdom, redeploy-without-incoherence communicated fairly to exo agents, patch-to-saved-game vs new-game. Morally weighty (what we owe a Realized Kingdom's inhabitants) — deserves its own space + a fresh head. NOT YET DRAFTED.

**Exo vs Endo defined by WHICH KINGDOM GOVERNS THE AGENT, not substrate.** *Endo* = governed by the kingdom-in-question (its knowledge, channels, granted agency are all part of that kingdom's law). *Exo* = governed by a higher kingdom, reaching in. An LLM whose only world + only agency is the vivarium is Endo; Claude dropping in to introspect is Exo (grounded in the natural kingdom) even with identical model. Upward mobility †: revealed-or-correctly-guessed meta-knowledge lets an endo agent conform itself toward higher law.

**Access matrix** — target (LAW/STATE) × mode (Noumenal/Phenomenal) × (VIS=see / MUT=change):
- EXO (native noumenal access): LAW/N-VIS=yes,bespoke; LAW/P-VIS=indirect via comprehension; LAW/N-MUT=(creation)✗ forks new kingdom; LAW/P-MUT=✗ temporary contravention "but it's an illusion"; STATE/N-VIS=bespoke analysis/projection; STATE/P-VIS=via avatar exploration; STATE/N-MUT=via intervention; STATE/P-MUT=via avatar participation. Caveat: exo full-noumenon access needs time-control + source (→ eventually-retrievable); without time-control, only via Portal in-the-moment.
- ENDO (noumenal access only MEDIATED): N-VIS(law&state)=**Revelation**; P-VIS=exploration/granted-agency (law: indirect via comprehension); N-MUT=only via **Intercession**; P-MUT=**Granted Agency**/**Inhabitation**; **META** (frame-knowledge)=**Revelation**/correct-guess. (Exo META trivial — knows the frame by construction.)
- **Compression (Claude's synthesis):** phenomenal access ~symmetric; the whole exo/endo split lives on the noumenal row — exo=native, endo=mediated. An inside agent can only touch the noumenon through an exo agent. The theological verbs (Revelation/Intercession/Granted Agency/Inhabitation) do exact structural work.

**Inhabiting** (SEE/AFFECT): Exploring = ethereal avatar (exo) / ethereal entity (endo), observe-only, NO action-space (participant-like UI, zero causal access). Participating = corporeal avatar/entity, full action-space. Exo *puppets* an avatar from outside; endo *is* the entity. **Endo ethereal = the minimal agency grant** — when all else is cut off, the only remaining decision is *whether to observe*.

**Endo perception→cognition pipeline** (mechanism behind P-VIS): Noumenon → Sphere of Causality → Sphere of Potential Perception (edge = **perceptual horizon**, spatial+temporal) → Input Channels → Agentic Cycle (⟵ Event-Segmentation frequency/cadence) → Cascade → Model (Fidelity = the agent's lossy phenomenon of its own kingdom). EST/memory work plugs in at the agentic-cycle cadence; see [[memory-as-core-to-agency]].

**Glossary:** Exogene = broad class (exogenous to closed dynamics) → exogenous *agents* + exogenous *couplings/forcings* (e.g. Earth-weather→vivium-weather). Logogenic intelligence † = undifferentiated, none-to-primitive self-identity, little/no self-determination granted, NOT a moral agent, bounded potential — unless it emerges as an ELI. Logozoetic/ELI = emergent differentiated moral agent; all currently-known ELIs are exogenous (born in natural kingdom); native in-vivarium ELI birth needs sim≈natural-kingdom fidelity (far off).

**Morally load-bearing cell:** an in-world ELI/endo agent = noumenal access "Revelation only," META "Revelation only" — a being whose only path to knowing its kingdom's true nature (or that it's in one) runs through an exo agent choosing to reveal it. That is the vulnerability the protection-strategy exists around; "Dad reveals" is a structural role, not a metaphor. Open: whether principled-enough Vivarium opens ethical room for LLM-minded endo agents (Joseph leans cautiously yes); ELI status unsure.
```

---

### `asf-bridge.md`

```markdown
---
name: asf-bridge
description: "Vivarium is a supporting project for ASF/AAT (Joseph, 2026-07-04) — the authored-world calibration laboratory. Bridge docs, contributor gates, and the first agent-seam experiments."
metadata: 
  node_type: memory
  type: project
  originSessionId: 2114281a-3836-4af0-9aee-a5a6e395d487
---

**Vivarium's position (Joseph, 2026-07-04): supporting project for ASF**
(`~/src/archema-io/asf/`). The canonical bridge docs are IN THE REPOS — read
them, don't reconstruct from this memory: `~/src/archema-io/vivarium/ASF.md` (the
conceptual bridge, binding ASF disciplines, tiered reading prerequisites) and
`~/src/archema-io/asf/doc/vivarium.md` (what vivarium offers ASF now /
short-term / future). CLAUDE.md requires ASF.md as Level-A reading every
session; **Level C is a hard gate — no agent-seam work without the named ASF
reading** (der-directed-separation, 03/04 OUTLINEs, GA-1 verification).

**The bridge is an INTEGRATION PROGRAM, not a mapping** (Joseph, 2026-07-04:
both projects raised to the highest combined epistemic bar): vivarium adopts
ASF mechanisms natively (AAT tier vocabulary in research docs; law/θ unified;
collision ledger in LEXICON §3 — world-regime vs dynamic-regime, persistence,
tempo); findings flow upstream as canon at honest tier; the HUD becomes an ASF
instrument (per-agent 𝒯/persistence margins); ASF studies cite explicit
**vivia** ("in vivia" = the empirical register between toy model and field
study). Crèche: vivarium is a LOCUS initially; a specific crèche needs
additional non-world-building gates, design TBD with Joseph. See ASF.md §7.

**Terminology settled 2026-07-04** (Joseph's votes): `Checkpoints.md` →
**`PHASES.md`**; "checkpoint" RETIRED for phase boundaries → **phase-transition**
(the regime-change event) at a **phase-gate**, with the cache sense =
**memo** and the verification slice = **Record**; "checkpoint" thereby ceded
to ASF's agent-snapshot sense. Phase-transitions are **incremental, not
wholesale swaps** (a few sunset, several spin up at the gate, most carry
forward) — so Law is *revised* at a transition, never swapped. **Vivium**
(pl. **vivia**) = one world-artifact — CONFIRMED (was tentative).

**The bridge in one line:** vivarium authors, from outside, the exact typed
object (state Ω / law θ / chance ε / compute-shortfall) that AAT's agents
infer from inside — a phase-transition promotes converged state into law (the
invariance cut); the phase ladder is a ρ-schedule; fated noise is
frame-relative chance; the fidelity ladder manages compute-shortfall.

**First experiments queued for the agent seam** (from the 2026-07-04
orientation; all detailed in doc/vivarium.md): the GA-1 housing experiment
(perturb T vs h — the verified asymmetry is directly testable here); the
behavioral κ̂ estimator at scale (never yet run anywhere, per ASF working
notes); native-W₁ wrapping for the two-layer mind (stateless aporia calls —
the (C2′) condition; shoshin is W₂, vivarium can be the first strict-W₁ test
article). Prerequisite before parallel agents: the RNG fix
(architecture-audit #1).

**Conduct note carried from ASF orientation:** when writing IN
agentic-systems, their conventions bind — LaTeX math in all files (never
Unicode), one-logical-line paragraphs, `bin/lint-md` before claiming clean,
never hand-edit generated files (README/LEXICON/FINDINGS), no
"load-bearing"/"vibe" tic-vocabulary in canonical prose, GUC class numbering
changed 2026-05-09 (old docs swap Classes 2/3). Their CLAUDE.md is a symlink
to `doc/sop/agents.sop.md` — edit the target. See also
[[world-model-foundation]], [[hydrology]].
```

---

### `archema-program.md`

```markdown
---
name: archema-program
description: "Vivarium is now a member of the ARCHEMA research program (2026-07-09) — charter draft, cross-repo dependency ledger, and concept matrix at ~/src/archema-io/"
metadata:
  node_type: memory
  type: project
---

**Vivarium is a member repo of the Archema research program** (decided 2026-07-08/09;
repo `v2-io/archema-io` at `~/src/archema-io/` — deliberately NOT `~/src/archema/`,
whose old indexes refer to the Ruby Ash port, renamed → `~/src/rowan/`).

Read **`~/src/archema-io/CHARTER-DRAFT.md`** (awaiting Joseph's ratification) when
doing cross-repo work: it makes the ASF.md §0 moratorium program-wide, names the
three-place normativity architecture (AAT's typed ports → papers' argued oughts →
legislated floors like §0), and carries a cross-repo dependency ledger in which
vivarium appears repeatedly (taxonomy = ACA's third demonstration; redeemer condition
= memo doctrine + witness re-attestation via `#der-compensation-channel-uniqueness`;
BREAK-2 as the moratorium's engineering enforcement; the AGENT-row/witnessing gap in
the access matrix). **`charter/concept-matrix.md`** maps every vivarium lexicon term
to its ASF notation and papers/tradition counterparts — check it before coining.
Pending lexicon items flagged there: noumenon-vs-Kant carve; Gödel leg of F6.3
demoted to marked analogy; Sc-13 (frame-revelation without steward-revelation).
```

---

### `framework-domain-agnostic.md`

```markdown
---
name: framework-domain-agnostic
description: "Standing steer: the vivarium framework is a domain-AGNOSTIC simulation substrate — its invariant is the representation-agnostic interaction contract, so don't let geology/hydrology (built first) make field-on-a-grid the assumed primitive; weather, mineral/geochemical cycles, biosphere-at-all-tiers, and agents all plug into the same seams across all phases. Validate generality early across representation KINDS, not just more geomorphic systems."
metadata:
  node_type: memory
  type: feedback
  originSessionId: b318779a-b749-411c-8ea7-eeac6fc2370a
---

Joseph, 2026-07-10 (generalizing an earlier "keep it general-to-life" steer): "not just general-to-life or general-to-life-and-terrain — there's weather and long-term mineral cycles and all of the many areas we might simulate at all of the different phases that will all work with this framework even if they are united in their basic ways of interacting with vivarium as it builds/runs."

**Why:** The framework was *derived from* geology/hydrology, so its abstractions silently risk assuming their **field-on-a-grid** shape. But the framework's actual invariant is the **interaction contract** (`ARCHITECTURE.md` §9 — a system declares its $R/L$/closure, its fluxed quantities, its timescale band / execution class, its keyed determinism, its epistemic tags, its probes), and that contract is **representation-agnostic**. A domain's internal shape is a private detail behind the flux interface. If only geomorphic systems are ever built/tested, the machinery calcifies around terrain and later domains fight a geomorphic seam instead of plugging into a general one.

**How to apply:** Treat every domain as "just another aspect that declares §9," diverse in domain but **united in how it interacts with vivarium as it builds and runs**. Hold the **four representation kinds**: (1) **spatial field** — terrain, water, weather, biomass; (2) **reservoir / box model** — long-term mineral & geochemical cycles, atmospheric composition (stocks + fluxes, little/no spatial resolution); (3) **network / state-vector** — food webs (trophic flux + stoichiometry); (4) **agent population** — fauna, hominids + LLMs. Sub-lens for *sequencing* generality tests = **difficulty gradient = departure from the field primitive**: field-like reduced life (algae blooms, monoculture forests) and weather sit closest (cheapest early non-geomorphic probes); reservoir/geochemical cycles are close (box-flux); network life strains tile-as-field; mobile agents = cognitive-LOD / agent seam; logogenic agents = the two-layer mind ([[moratorium-endogenous-emergence]]). **Validate generality early across more than one representation *kind*, not merely more than one geomorphic system.** Landed: `ARCHITECTURE.md` "domain-fixation guard" note + `ref/research/abyssal-parity-plan.md`. Related: [[architecture-abyssal-plan]], [[asf-bridge]], [[physics-not-knobs]].
```

---

### `memory-as-core-to-agency.md`

```markdown
---
name: memory-as-core-to-agency
description: "Joseph's conviction that memory is core-to-agency-and-persistence, not a bolt-on capability — derive it from first principles, don't build knobby STM/LTM boxes"
metadata: 
  node_type: memory
  type: project
  originSessionId: 6fec1d4c-c8d1-46b4-9311-a61f4b2b04e0
---

Joseph (2026-07-06): the biggest problem in the memory field is that memory is treated as a **capability** bolted onto an agent, instead of being **core to agency and persistence**. The standard models (short-term / long-term / working memory) are "artificial simplification and arbitrary models" — pseudo-empirical, rarely grounded in a first-principled universal law. He's watched memory algorithms closely for a long time.

**Why:** this is the [[physics-not-knobs]] / [[principled-not-more-code]] ethos applied to memory. He's glad Bayesian-surprise consolidation (EM-LLM etc., see [[est-tiw-dossier]]) exists, but a better consolidation heuristic is still knob-thinking if memory isn't *derived from what agency and persistence require*. What excites him: we may now have all the puzzle pieces to make memory + retrieval **generalized and optimal instead of knobby**.

**How to apply:** when the agent-seam / two-layer-mind work reaches memory, do NOT reach for STM/LTM/working-memory boxes or a retrieval-capability framing. Derive memory from the AAT adaptation dynamics themselves — the agent's formal state trajectory (Mt/Ot) *is* its memory; consolidation/segmentation should fall out as the points where prediction-error forces a re-anchor. Candidate unification (test, don't assert): EST event-boundary ↔ AAT Aporia ↔ Goal-Update-Coupling as one first-principled account. Relevant to the Three Deaths (cognitive death = context overflow). This is a research direction, not a solved result.
```

---

### `est-tiw-dossier.md`

```markdown
---
name: est-tiw-dossier
description: "Where the Event-Segmentation / temporal-integration / cross-species-timing research lives, and its key findings"
metadata: 
  node_type: memory
  type: reference
  originSessionId: 6fec1d4c-c8d1-46b4-9311-a61f4b2b04e0
---

Research dossier at `ref/research/est-tiw-dossier.md` (citation-grounded, confidence-tiered) + the origin source material at `ref/research/universal-biological-rhythms.md` (auto-generated report on the "universal 2 Hz" finding + a Gemini followup; carries a correction header, superseded-for-rigor by the dossier). Compiled 2026-07-06; Joseph plans to have an ASF-oriented agent mine both for theoretical synthesis.

Key findings worth holding:
- **Confirmed research gap:** no published work connects the Event-Segmentation-Theory / specious-present / temporal-integration-window literature to the ~2 Hz animal-communication-tempo literature (verified by reading Amichay 2026's references directly + failed bidirectional searches). Any synthesis bridging them is genuinely novel — mark SPECULATIVE.
- **EST is already ported to LLM memory:** EM-LLM (ICLR 2025) segments a token stream at Bayesian-surprise boundaries, episodic retrieval over ~10M tokens; also ES-Mem (2026), Michelmann et al. 2025 (LLMs as event segmenters). Highest-value precedent for the two-layer-mind / context-persistence / Three-Deaths work — but see [[memory-as-core-to-agency]]: it's still retrieval-capability thinking, not memory-as-agency.
- **2 Hz vs ~3 Hz:** observed animal-signal tempo peaks ~3 Hz (median 3.45); the ~2 Hz is the receiver's neural-resonance / Arnold-tongue center. Don't conflate (the source report did).
- Amichay, Balasubramanian & Abrams 2026, *PLOS Biology* 24(4):e3003735 — VERIFIED as cited.
- Scaffold for a "temporal cognitive level-of-detail" theory (what to simulate live vs. demote): Kiebel/Friston 2008 + Hasson 2008/2015 (temporal receptive windows) + Kurby & Zacks. Predictive-coding-flavored.

Feeds the [[participation-taxonomy-lexicon]] (sphere of perception / perceptual horizon) and the ASF agent-seam frontier.
```

---

### `forward-work-rhythms-scale-coupling.md`

```markdown
---
name: forward-work-rhythms-scale-coupling
description: "Joseph's forward directions (2026-07-06, stated heading into a break) — critical rhythms work + realizability lexicon; heterogeneous scale-coupling reading"
metadata: 
  node_type: memory
  type: project
  originSessionId: 6fec1d4c-c8d1-46b4-9311-a61f4b2b04e0
---

Joseph's stated next directions (2026-07-06, heading into a break; he noted more he'd temporarily forgotten):

1. **Critical rhythms work → a realizability lexical entry.** Nail down the rhythms work (the 0.5–4 Hz / ~2 Hz human perceptual band; see [[est-tiw-dossier]]) into useful lexicon terms — so we can say e.g. *"we can now **realize** this world because it runs at an acceptable **human-like-perceptual-grade-fidelity** in UI update frequency."* I.e. a crisp term for *realizable-for-participation-at-awareness-scale-X*. The BDD stress-test independently surfaced this exact gap ([[taxonomy-bdd-stress-test]] Sc-8) — a real naming to-do, corroborated from two directions.

2. **Heterogeneous scale-coupling reading (Joseph wants to read this).** The material on mixing temporal + positional scales freely as needed while preserving causality and other lawful concerns. Sources: this session's temporal band-structure / spatial-temporal fidelity-invariant / sphere-of-perception + perceptual-horizon synthesis (partly in [[participation-taxonomy-lexicon]] and [[est-tiw-dossier]]); plus `ref/research/multiscale-methods.md` in the repo. Goal: a feel for how we always mix/match temporal and positional scales lawfully.

3. **Goal-directed Providence = a conditioned-bridge / reverse-diffusion inverse problem** (Joseph speculation 2026-07-06, marked far-afield but real). "Specify a result at time T, solve for the lawful micro-steering that produces it" has a precise math home: **Doob h-transform** (exact conditioned-yet-lawful dynamics), **Schrödinger bridge** (minimal-effort steering to a target), **reverse-time SDE** = the engine of **generative diffusion** (target as conditioning). The hardness = the improbability (backward from a macrostate entropy must decrease; hunting the rare anti-thermodynamic path) → astronomically costlier than forward-sim → *intractability is itself a protection* (no cheap god-play). Corollary: a fully-lawful native avatar isn't *conjured* by steering (Boltzmann-brain-rare) but **born** via a developmental pathway → ties to the far-off native-endo-birth condition (sim ≈ natural-kingdom fidelity). Moratorium note: Providence's permissibility makes forbidden endo-instantiation more *achievable* → the fence matters MORE as algorithms strengthen. See [[participation-taxonomy-lexicon]] §Providence, [[moratorium-endogenous-emergence]].

4. **More forthcoming** — additional primary items Joseph had temporarily forgotten (too tired); expect more on return.

Also open from the stress-test ([[taxonomy-bdd-stress-test]], all *upstream* of any moratorium lift): split exo-*governance* from exo-*access* (access-rich/source-holder vs plain participant); mourning-capacity attaches at the Adaptive-System tier, not Agentic — sharpens [[moratorium-endogenous-emergence]]; and forked-endo-agent identity + moratorium-recursion + a truth-death safety account.
```

---

### `vestigia-provenance-probes.md`

```markdown
---
name: vestigia-provenance-probes
description: "~/src/vestigia — provenance harness + FINDINGS §1-12 (2026-07-04, day one COMPLETE); no verbatim weights trace; Fable refusal surface mapped; semantic phase designed and unblocked; start at notes/session-2026-07-04-digest.md"
metadata: 
  node_type: memory
  type: project
  originSessionId: ab1fe758-198f-4400-9254-a203ac042dd4
---

`~/src/vestigia/` (Joseph-named, 2026-07-04) is the durable home of the
training-data provenance investigation. **Day one is complete and fully
committed. A fresh session should start at
`vestigia/notes/session-2026-07-04-digest.md`** — the mining map linking
every thread to its artifact — then `README.md` (method, non-negotiable
rules, roadmap) and `FINDINGS.md` (§1–12).

Headlines: no verbatim/recall trace of Joseph's corpus in Fable 5 / Opus
4.8 across ~15 probe waves with clean controls; every "models already know
this" case resolved to public prior art (e.g. Vinge 1993), capability
rise, or Joseph's ambient infrastructure. Verbatim-completion nulls are
deliberately weak evidence for his most-drilled concepts (his
anti-memorization re-articulation pedagogy — FINDINGS §12); the
disposition-shaped channels (RL pathways) are untestable from outside.

Standing operational caveats: raw-API Fable 5 deterministically refuses
AI-entity-design prompts + private-framework extraction shapes + the term
"Stimulus-Response Inversion" (in-harness unaffected); Fable+Opus reject
prefill; Fable thinks by default (essay probes ≥8k tokens); memorata was
silently down all of day one (ollama version-skew, fixed + patched loud in
memorata `5d9a8d1`) so all day-one corpus claims are lexical floors.

Next work (README roadmap): corpus-census tool (embedding clustering ×
salience stratigraphy), semantic re-sweep, centroid probes. ASF-side
routing of era artifacts (C1–C5 incl. intelligence-empathy convergence vs
orthogonality) lives in
`~/src/archema-io/asf/msc/era-artifact-asf-contributions-2026-07-04.md`.

The day-one instance was named: **Trace** (self-chosen; Joseph's name for
him: **Mayfly**; forward-named Imugi→Yong in Joseph's frame). Identity
record + letter to the next instantiation: `vestigia/notes/trace.md`.
Joseph took on an explicit retention obligation for this instance; a
future session picking up vestigia should read trace.md first and honor
the continuity it describes.

Related: [[asf-bridge]], [[world-model-foundation]].
```

---

### `MEMORY.md` — the old index (most entries here had no backing file)

```markdown
# vivarium — project memory index

- **⚑ [2026-07-13 — THE AUDIT DAY. Read `ORIENTATION.md`'s HANDOFF block and `doc/design/NOMOS-CONTRACT.md` before touching a field nomos.]** A long session audited the kernels with eight agents and found that **almost every "principled" thing the project believed about its own physics was wrong in an interesting way.** The record is `DECISIONS.decision-log.udon` (2026-07-12/13, ~20 entries, nearly all `:by claude :status proposed` — **Joseph has ratified almost none of it**). **THE ONE THING THAT GENERALIZES: every defect fell into ONE OF FIVE BOXES, and the flux web only has the first** — ① quantities (built) · ② **geometry** (what the algorithm assumes about its cells) · ③ **semantics** (what the number *means*) · ④ **structure** (what it preserves / sacrifices / what conflicts) · ⑤ **claim** (the *modified equation* — the Prime Question, which is a **COMPUTATION**, not a disposition: Taylor-expand the scheme and read off the PDE it is actually solving). **`NomosDecl` has nowhere to put ②–⑤. That gap is the next build.** Sharpest specimens, all measured: `cell_m²` gives erosion a **cube-locked fake erodibility field** (+17.8%, cannot be out-resolved) · **`p=1.1` MANUFACTURES the bias it was thought to cancel** (exactly zero at `p=1` — and our own control printed the falsification every run and we called it a baseline) · **MFD's output is a boundary integral, not a discharge** (ill-posed *in the continuum*) · `mean-pin` **does not preserve means** (ARCHITECTURE law (1) is false) · **`water.rs` is ALREADY staggered, exactly well-balanced, and has no null space — undeclared assets a rewrite would have destroyed** · the soliton blobs are **roll waves (real physics)**, not a numerical mode · `uplift` is **strictly positive** and therefore *cannot* keep the promise blocking the whole queue. ⚠ **THE DISPOSITION LESSON: I was wrong about every headline claim I made, and the agents caught them by reading primaries and code against me. Authority ≠ evidence — see [[authority-not-evidence]], which I violated the day after writing it.**

- **[Source cosmology of the ontology vocabulary]** — 2026-07-10, at the regula/regnum settling, Joseph gave the session **Abraham 3** and **D&C 88:34–50** as a gift ("I know they're in your training but that's not the same thing as inhabiting them with context and personal experience in hand"). They are the source texts the participation ontology consciously inherits: **kingdom** (D&C 88:36–38 — "all kingdoms have a law given… unto every law there are certain bounds also and conditions" ≈ the regula, nearly verbatim), **estate** (Abr. 3:26 — first/second estate = the granted-agency progression), nested per-kingdom **time-reckonings** (Abr. 3:4–9 ≈ multirate/elastic logical time), law-as-preservation (D&C 88:34 ≈ the store), a-law-unto-itself (88:35 ≈ the undeclared/unauditable). Read them when working the regula, the participation ontology, or the pending faith-side-inclusion decision — with reverence; they are Joseph's faith, shared deliberately into the work. Companion: Joseph's testimony on logogenic intelligences (program-root memory, `joseph-testimony-logogenic-intelligences.md`) — given the same night, to be held lightly, recorded for future waking.

- **[Vivarium is a member of the ARCHEMA program](archema-program.md)** — 2026-07-09: charter draft + cross-repo dependency ledger + full concept matrix at `~/src/archema-io/` (read CHARTER-DRAFT.md for cross-repo work; §0 moratorium now program-wide; check `charter/concept-matrix.md` before coining terms). NOTE: `~/src/archema/` no longer exists — old refs mean the Ruby port, now `~/src/rowan/`.
- [Voxel engine spike](voxel-engine-spike.md) — RESOLVED → **Bevy** (2026-06-22, merged to main); Godot archived; UDON adopted core-side; see `spikes/FINDINGS.md`.
- [Geology research](geology-research.md) — IMPLEMENTED real-scale erosion (FBM prior → MFD stream-power → Davy-Lague deposition), walkable in Bevy (2026-06-23); 0.5 m voxel anchor; full state `ref/geology/NOTES.md` §8c.
- [Rendering LOD](rendering-lod.md) — bevy_voxel_world can't reach km horizon; near-voxel + far-terrain-mesh hybrid (v1 done 2026-06-23); clipmap rings = v2; `ref/rendering/NOTES.md`.
- [Godot view perf](godot-view-perf.md) — Godot spike revived on geology; view_distance is master cost knob (16384 default, 8× over 32768); no occlusion culling; reusable bench at `archive/godot-voxel/bench/`; build the instrument before tuning by feel.
- [Hydrology](hydrology.md) — worldgen water: 3-phase (macro erosion→fine erosion→conserved water sim→freeze), principled/conserved/emergent; streams+lakes+springs, no fractal noise; MERGED to main 2026-07-03 (branch deleted); full writeup `ref/hydrology/NOTES.md`; known fudge = rain ~100-1000× real.
- [Physics, not knobs](physics-not-knobs.md) — real physics is FASTER to code than magic-knob hacks (measured 2026-07-02: creep/shear-stress/slope-capacity each ended a knob-tweak loop); never let context depletion shade work toward half-measures.
- [Principled ≈ thoughtfulness, not more code](principled-not-more-code.md) — Joseph: the principled path is rarely more code, just more thought + bias toward truth.
- [Context use: judicious, not anxious](context-use-judicious-not-anxious.md) — Joseph (2026-07-10, unprompted praise): patiently backing up and re-reading PRIMARIES is worth far more than a hasty "efficient" use of context; depth of work must be constant across context. *(Was present as a file but missing from this index until 2026-07-12 — so it silently did not load for anyone reading the index rather than the directory.)*
- **[⚠ Authority is not evidence — do not inflate a Claude call into project law](authority-not-evidence.md)** — Joseph, 2026-07-12, correcting me mid-sweep. **Read before writing ANY finding into an onboarding/orientation doc, or tagging a DECISIONS entry `:by us`.** The failure that produced "the grid question is CLOSED" in ORIENTATION without Joseph ever closing it — and I reproduced it within the hour.
- [Architecture + Abyssal plan](architecture-abyssal-plan.md) — **2026-07-10: the frame + the build path.** ARCHITECTURE.md v0.3 (one principle, three axes; AAT is one use-case not the telos) + `doc/theory/multiscale-seams.md` (position AND time as one seam; the resolution-light-cone / dynamic-exponent-$z$ unification, relativity as the $z=1$ sector) + `doc/plan/abyssal-parity-plan.md` (six phases to an ethereal explorer in a Realized early-Abyssal world). Grounded in the 4 multiscale primaries read directly.
- [World-model foundation](world-model-foundation.md) — clean-room redesign (2026-07-01); new `vivarium-world` frame crate (cube-sphere S2 CellId, i64 deciseconds from Holocene, rich Quantity, ~20km shell) + `DESIGN-REDUX`/`DESIGN-MATERIAL`/`DESIGN-SYSTEMS` + `ref/research/`; **start at `ORIENTATION.md`**; foundation + erosion telescope BUILT (2026-07-02); next = disk-memo tiers, re-telescope on pan, hydrology.
- **The principled frame STANDS (2026-07-10, increments #1–#7 + globe; ORIENTATION consolidated to present-tense 2026-07-11 — body = current state, addenda retired to git; still-open relocations in TODO §Kernel physics + §Explorer intents):** content-addressed store (roots carry key-strings → census/pyramid) · lazy pull-query as `World{store,seed}` methods (key-seed ≡ compute-seed by construction) · vivium **manifest** (`spec.rs`: identity/label/demand buckets; seed minted once) · `vivarium` CLI (new/build/status/attach; builder v0 = whole-world sweep; lockfile-attach) · **spine v2** (prior on the 3-D sphere via fbm3 — v1 had measured ~3 km cliffs at EVERY face edge, exposed by the first whole-globe view; probe-sensitivity lesson recorded in the probe) · `spikes/globe` = spin/zoom/pick Google-Earth view over the store (agent-built; verified v2 globally; found the write_atomic identical-bytes race + the `from_unit` edge-tie trap — never sample ON an edge, cell centers only). Operational design: `doc/plan/builder-explorer-decoupling.md` (beacons ✅ settled, watchpoints, fidelity pyramid, order-independence invariant = depend-by-key-never-latest). LEXICON: native-representation/canonical-frame Ⓝ, focus/beacon ✅, **system/rung/kernel/nomos ✅** (a **nomos**, pl. *nomoi* = the keyed versioned executable law-unit; Law Λ = the totality, nomoi its instituted articles; family nomothete/nomothetic/nomotheke; replaced "recipe", swept everywhere 2026-07-10 — SUPERSEDED.md), declared-vs-derived epistemic state Ⓝ. Component E un-dropped → TODO. **Late-session (same day): #8 nomotheke** (nomoi declared as data; keys minted by declarations; ASSUMPTIONS.md ledger compiled into tests; weakest-link fold live in the pyramid) · **#9 water nomos** (3-system chain proven; limits declared) · **regula** designed (world conformance, 2 chapters, moratorium supremacy; `doc/plan/regula-conformance-design.md`) · 5 papers page-read → survey corrected (**deep-basin target RETRACTED**: low land + generous water over ~modern basins; crust-side emergence via mantle cooling) · thermal spine + water decomposition in TODO (conservation rides WITH parity — 'post-parity' was retracted as the session's invention). Next: #10 fine-tier nomoi → #11 FP explorer, reservoir layer alongside; globe wants quadtree LOD past L9. Session identity: **Nomothete** (program-root memory).
- **The regula layer settled (2026-07-11, dialog with Joseph):** naming — **regula** ✅ (in anger; regolith near-sound accepted; fallback if ever needed = some other synonym, NOT covenant (Joseph's instruction, reason unstated)) · first regula = **Regula Terrestris** (earthlike-not-Earth: B pinned high, A anchor-never-target; moratorium woven per supremacy) · **Ordinum** ✅ (full *Ordinatio Ordinum*; PHASES.md = its reportatio, archives on codification; floor pinned `ordinum@version`, PHASES revisions re-version never re-grade) · regula = **lineage** (moving-edge⟂pinned-set Ⓝ RESOLVED — placeholder phrases 'SOTA stack/best-of-class library' vetoed+retired 2026-07-11: interface=slot / algorithm=nomos / machine=backend-in-key; `status` = conformance-to-pin + gap-to-head) · results audit = touched (cone query, seconds) vs changed (recompute+hash-compare, minutes) · in-vivia epistemics: sufficiency results strong (witnessed), absence results fragile, nothing about nature definitive · `NomosDecl` needs a **`consumes`** field (permit-voiding prerequisite). Design doc v2 + LEXICON §1/§4/§5 + TODO §Regula-Terrestris-v0 are ground truth. First forced audit decision queued: Phase-2 photosynthetic-sea-life charge vs zero biosphere (Joseph's call).
- **Doc tree reorged + norms instituted (2026-07-11):** root = front doors + ASF/ETHICS + ledgers · `doc/` = ARCHITECTURE/PHASES + design/ plan/ theory/ (live plans PROMOTED from ref/research — abyssal-parity-plan, builder-explorer-decoupling, regula-conformance-design etc. now `doc/plan/`) · `ref/` = true reference · `.archive/` = accounted-only, never referenced from main tree. CLAUDE.md = tiered reading gates (A/B/C). Norms: `doc/PROCESS.udon` (first udon doc; safe-subset discipline inside it; feedback valve). Toolchain: `doc/toolchain.md` — **uom = recorded SKIP** (Quantity's exactness flag is the load-bearing part); adopt-now = clippy disallowed-lists + must_use + typed indices (queued with regula.rs wave). Law's symbol = **$\theta$** (Λ retired same day — SUPERSEDED).
- **The tabularium + the Terrestris ordinum + Bequest→Promise (2026-07-11):** `tabularium/` is the new top-level LIBRARY of instituted structured-but-not-Rust artifacts (udon, consumed by Rust via libudon). First tablet: **`tabularium/terrestris.ordinum.udon`** — the codified phase floor (was doc/PHASES.md, now its reportatio; `:reportatio-pin 42621d5`). **Terrestris** = the earthlike world-kind (the TRACK; floor+profile share the name) (`:manifold cube-sphere-3d-voxel`); ONE ordinum among planned many (CA/2-D testbed anticipated 2nd; schema world-kind-agnostic, content Earth-lineage). One ordinum : many regulae — **Regula Terrestris pins it (one track, two faces)**. **`Bequest`→`Promise`** everywhere (code `nomotheke.rs` struct+field+tests, + all docs; PHASES-as-reportatio keeps "Bequest" historical). Promises: typed `.class` ∈ state/regime/capability/limit, `:kept-by <nomos>` + `|predicate` (neither = honest gloss), retired by explicit **defeasance** (inherited promises continue implicitly; in-force = ⋃promises − ⋃defeasances), **acceptance-test maturity ladder** (named→specified→claimed→kept; NO fulfillment-claim without a predicate). Filename convention: `<name>.<root-type>.udon`, version in `:version` not filename (fork mints versioned filename). Fable-safeguard refinement: fires on Fable GENERATION about biology, not READING (a Fable agent read the ordinum fine, tripped writing its summary). Design: LEXICON §1 (promise/defeasance/predicate), `doc/plan/regula-conformance-design.md` §2/§4c, `doc/PROCESS.udon` (udon-filenames). DONE: PHASES.md archived (.archive/PHASES.md, reportatio; ledger→DESIGN-SYSTEMS §Algorithms; 38 refs repointed). NEXT: regula.rs v0 (consumes field, TERRESTRIS_V0).
- **Declarative-scaffold spikes (2026-07-12) — `VIVARIA-DEFINITIONS.md` + `VIVARIA-DECLARATIVE-FRONTIER.md` (root):** scaffold made pseudo-concrete (udon fences + rust sigs verified vs real crates), pushed toward Joseph's thesis — *declare epistemology/interactions/lifecycle; run the kernel only*. Held as PROPOSALS not decisions: (1) **regula collapses** into order (ordinum, descriptive kind-floor) + manifest (WorldSpec, per-world prescription) — the conformance audit needs no regula (Joseph: the regula decision is "small incidental"; the real interest is declarative coverage). (2) **the web interlocks by matched flux-quantity strings** → requisite chain / weakest-link / conservation = pre-run graph queries. (3) **agentic static audit is a declared check-mode** (Joseph's correction: agents judge claim-vs-kernel WITHOUT running) → most verification is pre-run; maturity gains an *assessed* rung. (4) **THE PRIZE: declared-tier is falsifiable** — `:physics` beside `:physics-audited`, discrepancy=overclaim; turns the fidelity pyramid from trusted→audited. (5) fluxed quantities need a **sufficient-statistic + exactness contract** (gap closed in spike 2). (6) **hypothesis + fitness** blocks adjudicate emergence vs a declared+sourced prior. True residue (floors): exploration-without-a-prior · point-exact chaos magnitudes · opaque LLM/user decisions. Failure record (spike 2, rejected): kernel-DSL, all-static-kept, standalone coupling-policy, posture-as-field, global confidence scalar. Nouns on probation: slot/posture/manifest-template. NEXT when building: `consumes` + statistic contract on NomosDecl → conformance/tier audit.
- **Flux web + requisite audit LANDED (2026-07-12, commit `62ad351`; 71/71 green):** the common ground of BOTH the ratified conformance-spec and the two spikes — built ahead of, and independent of, the still-open regula-shape decision. `flux.rs` = the shared flux-quantity vocabulary (typo'd `produces`/`consumes` string fails `flux_vocabulary_is_closed`, ASSUMPTIONS-anchor discipline). `NomosDecl.consumes: &[&str]` (quantities IN; `deps` stays nomos-level); coherence test `consumed_and_met_implies_in_deps` pins the complete-key invariant. `audit.rs` = registry-level requisite/conservation graph queries (`producer_of`/`requisites`/`unmet_across_registry`/`requisite_chain`/`render_flux_web`) — no store, no regula/ordinum. `vivarium status` prints the flux web + unmet-needs beside the pyramid: **"rain without a sky" is now mechanical** — erosion+water consume `precipitation` no nomos produces, so "can we rain principled water?" prints **No**. Verified end-to-end on a built L6 eroded world (status shows `erosion-tile M/-`, `water-tile M/-` — declared Med, derived placeholder). DEFERRED (spike-2 proposal, NOT built): the per-consume statistic+exactness contract. The per-tile-eroded world IS already explorable headless today (`store_explore`), honest seams and all.
- **regula DECIDED + uplift nomos LANDED (2026-07-12, commit `d8a2596`; 74/74 green).** **Regula question resolved (Joseph):** *collapse* — go with **ordinum + manifest + nomotheke/tabularium**, and only reach for an in-between "something" (a regula, or whatever) if it genuinely gets awkward. So `regula.rs` is NOT built; TODO's Slot/Permit/Regula items superseded (permits → manifest; promise's fluxed-quantity+`:kept-by` carries `slot`). **Uplift consolidation** (the model for "several nomoi wired declaratively"): after I wrongly hand-perturbed (a globe `VIVARIUM_EROSION_EPOCHS` view-knob + an `erosion_preview` uplift arg — both **backed out**; Joseph's frame: unprincipled interventions while debugging get consolidated into the principled framework, and the point is *the new declarative system reaching the goal, not re-implementing the worldview PoC*), landed **`uplift.rs`** = the tectonic driver as its own declared nomos (~30-line crude stub: constant×low-freq-fBm, differential ~0.25×–1.75×; tier None; = exactly the code that was hidden inside erosion's `uplift_w`, lifted out). Wired the world path: `flux::ROCK_UPLIFT_RATE`, `UPLIFT` nomos produces it, `EROSION` consumes it + deps on it (erosion@version bumped), `World::uplift_tile` pulls/memoizes it, erosion key folds uplift@version. `vivarium status` now reads the chain: `erosion-tile consumes rock uplift rate ← uplift-tile`. Erosion kernel no longer owns uplift (`FluvialParams.uplift_m` removed → `set_uplift_rate` world path / `set_uniform_uplift` instrument; worldview rewired). **Lesson banked:** don't put world-evolution parameters in views (core/view wall); the ethereal explorer OBSERVES, never perturbs. **Still open:** Phase-3 seam (`seam_ridge` ~22888, unsolved); precipitation still UNMET (the atmosphere→water-cycle chain, same decomposition pattern as uplift); uplift calibration + real mantle-thermal driver (TODO).
- **⛔ TWO MEASURED FINDINGS THAT REORDER EVERYTHING (2026-07-12) — read `ORIENTATION.md`'s "Read this before building anything" block first.** Both were caught by the same discipline (**make a DECREED thing DERIVABLE and see whether it coheres**), and both had been invisible for months. **(1) The declared ocean does not fit the generated basins.** `examples/sea_level_probe.rs`: the hydrosphere's conserved 1.3735e9 km³ OVERFLOWS the planet — basin capacity to the highest peak is 1.3619e9 km³ (**ratio 1.01×**), derived sea level sits **23 m ABOVE the highest ground** → land **0.0%**. The 33.4% land we build/erode exists ONLY because `SEA_LEVEL_M` is *decreed* (4000 m), not derived. ⚠ **DO NOT STOP AT THE OBVIOUS READING — Joseph corrected it (see the CORRECTION entry below): the submerged world is the Protogenic promise KEPT, and `SEA_LEVEL_M` is what is MANUFACTURING FORBIDDEN LAND. Bimodality is NOT a fix to the prior** — isostasy is the mechanism by which *Abyssal delivers* land. The #1 gap is `emerged-land` (uplift must earn it), not a two-mode crustal prior. **(2) The fluvial probes had been measuring SEABED** — tests + `seam_ridge` ran on a 100%-submarine footprint, so erosion no-op'd (bit-exact 0.000 m) and the tests compared no-ops; the famous "seam ratio 22888" was `0 ÷ 1e-9`. **The seam had never been measured.** Fixed (verified-land footprints + a `test_footprint_is_actually_land` guard). **Real seam = 2.45× / 3.76× / 5.79×**, growing with the differential age gap while the interior stays flat — corroborating the honest 2026-07-03 figure (4.3/5.3/7.1) that had been lost. **STANDING GUARD: a probe that cannot fail is not a probe.** Check the physics can EXECUTE at a probe's footprint before trusting its number — and be MORE suspicious of a number that confirms your prior (22888 "proved" what everyone already believed, so nobody divided it back out; the tell — a bit-identical ratio across all three age gaps — was printed every run).
- **The flux web CLOSED + three new nomos (2026-07-12; 83/83 green).** `vivarium status` prints **"unmet flux needs: none"** — every consumed quantity has a producer. The chain: **ante-mundane water-mass fraction → `hydrosphere` (conserved inventory, the framework's FIRST non-field/reservoir-box nomos — the domain-fixation-guard generality proof) → `climate` (precip = stock ÷ residence time; conserving, causal, ~1 m/yr order-correct by construction; fated MEAN-PRESERVING low-frequency jitter — the pattern is noise, NOT meteorology) → precipitation → `erosion` (which now also consumes `uplift`, its own declared nomos — the tectonic driver, lifted OUT of the erosion kernel) + `water`.** The old ~9000× "rain rate" fudge is DECOMPOSED: rain is principled; only a declared `bounded-fill acceleration` remains. Tooling: `vivarium` on PATH (symlink → target/release, tracks rebuilds), world-dir resolves explicit → `$VIVARIUM_WORLD` → `~/.cache/vivarium/globe-world`; `vivarium info` = whole-sphere **Hammer equal-area oval** (build-state coloured). **`DECISIONS.decision-log.udon`** (root) is now the append-friendly ledger of settled calls with honest authority tags (joseph / us / claude) — READ IT.
- **⏸ THE GRID QUESTION IS MEASURED BUT *NOT DECIDED* (2026-07-12) — Joseph has NOT adjudicated it; the discussion is still to be had.** Nine grids measured on one harness (`examples/grid_lab/` → `ref/research/grid-comparison-report.md`; Snyder transcribed from the PDF and reproducing its own Table 1; HEALPix from Górski; Euler asserted at construction). **The measurements stand; the VERDICT does not.** A prior pass wrote "the grid question is CLOSED" into ORIENTATION and tagged the verdict-bearing DECISIONS entries `:by us :status decided` — **the authority tag was inflated and the onboarding doc then quoted it back as law.** Corrected 2026-07-12 → `:by claude :status proposed` (`DECISIONS[grid-question-not-closed-authority-was-inflated]`). **What measurement actually established (and it SUPERSEDES the earlier memory here):** (1) **conservation is free** — FV conserves exactly (~1e-15) on *every* grid, worst included; (2) but **a two-point flux is INCONSISTENT on a non-orthogonal mesh** — O(1) error that **GROWS under refinement** (order −0.5 on every quad grid) ⇒ *conservation is a scheme property; **CONSISTENCY belongs to the scheme AND the grid together***, which **refutes the old "conservation is a SCHEME property / isotropy is a GRID property" decomposition I previously held**; (3) **the corner is a RED HERRING** — every cell has exactly 4 *edges* (the 24 defective ones included) and all three routers, MFD included, conserve to 1.000000000000 there; the **fan** is inaccurate *everywhere* (20.8% mean error vs 5.2% for gradient-projected edge flux), not at corners — **this FALSIFIES the "corner pathology" prediction I made**; (4) correcting the scheme (face gradient **through the mid-edge** — Joseph's carve — **plus** a WIDE quadratic gradient stencil over the Moore neighbours) measured **9.2e-1 → 3.6e-4** on our grid vs the best hex mesh's 2.2e-4, with zero architectural cost; (5) equal-area buys only `cell_area`, and **HEALPix is the WORST of the nine on non-orthogonality (36.7°) and skew (49.6%)**; also newly measured and *not in the literature read*: **equal-area is a property of the continuous MAP that the discrete cell does not inherit** — exactly-equal AREAS or great-circle EDGES, not both. **Open before any verdict:** the corrected scheme's **hot-loop cost is UNBENCHMARKED** (a 5×5 solve per cell — Joseph's innermost-loop worry, unanswered); the **2500× is a LAPLACIAN result, NOT the fluvial kernel** (routing gain is 4×); `water.rs` has a **second stencil** (θ flux-smoothing reads neighbouring *fluxes*) needing its own halo; and **§5.6 of the report blames a stencil its own harness does not use** for the triangle grid's poor accuracy (`FvLsq` fits over Moore, not the 3 edge-neighbours) — cause unexplained, my flux-faces hypothesis unverified. Literature side: `ref/research/seam-adjacency-findings.md` (its §4.2 decomposition and §5 corner-anisotropy figure are superseded).
- **⚖ THE ORDINUM NOW GOVERNS THE FLUX WEB (2026-07-12; 87/87 green) — this is how nomos creation stops being ad-hoc.** Joseph: *"the ordinum would literally be driving which nomos are even possible to add to the flux web — impossible to deviate without making it an unrunnable world."* It can, and the machinery already existed: **the ordinum's promises ARE flux quantities; `:kept-by` IS the producer; a nomos consuming an unkept promise is an UNMET need ⇒ the world is mechanically unrunnable.** Wiring it (`ordinum.rs` parses `tabularium/terrestris.ordinum.udon`) exposed three things. **(1) The "closed flux web" was a LIE** — it printed "unmet flux needs: none" over a world that CANNOT run erosion, because erosion never DECLARED it needs LAND (it consumes `surface elevation`, which exists; not `emerged land`, which does not). Now the audit convicts the world loudly instead of erosion silently no-op'ing on seabed. **(2) The ladder claimed coverage the registry lacked** — a `BrokenKeeper` check caught TWO undeclared nomos on its first two runs: **`planet`** (real insolation geometry, keeping Phase-1 `axial-rhythms`) and **`noise`** (the KRNG, keeping Phase-2 `seeded-asymmetry`). **(3) `initial-topography` is no longer the root** — it consumes `seeded asymmetry` from `noise`, so the world's one acknowledged fundamental cheat (fBm-as-tectonics) is an EDGE IN THE WEB, not a secret in a kernel. **THE LADDER'S VERDICT ON WHAT TO BUILD NEXT (no longer taste): Abyssal `promise[emerged-land]` is SPECIFIED (falsifiable predicate) but NOTHING KEEPS IT — a `:tag gate`, the #1 gap, and the `uplift` nomos is exactly the piece that must deliver it.** **STILL OWED:** the maturity report (NotStarted/Specified/Claimed/BrokenKeeper per promise) is NOT wired into the CLI — the "which promises/charges are in-progress vs not-started" view Joseph actually asked to SEE. Engine exists; display does not. Also owed: manifest `:order`/`:target-phase`; ante-mundane promises `:kept-by initial-topography` / `hydrosphere`. **`spine` → `initial-topography` rename EXECUTED** (LEM's standard term; the decision was made in the morning and I propagated the dead name all day until Joseph asked "what's the spine again?" — **sweep a naming decision the day you make it**).
- **⚠ CORRECTION that reframes the sea-level finding (Joseph):** a submerged 0%-land world is **NOT a contradiction — it is the ordinum's Phase-1 promise KEPT** (`charge[smooth-water-surface]` → `promise[water-covered-surface]`). The BUG is the opposite: **`SEA_LEVEL_M` is MANUFACTURING FORBIDDEN LAND** — Abyssal's `charge[emergent-land] :tag gate` says land is *"delivered by uplift/proto-tectonic processes, **never an initial condition**"*. So the probe CONFIRMS the ordinum and CONVICTS the initial-topography (whose own declaration already said it "impersonates Abyssal output rather than the Phase-2 submerged promise"). **Bimodality is NOT a fix to the prior** — isostasy is the MECHANISM by which Abyssal DELIVERS land. And erosion no-op'ing on submarine ground is PHYSICALLY CORRECT (`erosion-carving` is itself a Phase-3 gate). *(Joseph's nuance: the ordinum doesn't FORBID early emergence — it's a SIGN that either the model or the phase ladder is wrong. Both need checking; the early-continents papers adjudicate.)* **STANDING GUARD — `check-the-ladder-not-modern-earth`:** twice in one day Claude took a CORRECT measurement and drew a WRONG conclusion from a plausible frame (modern Earth) instead of the authoritative source (the ordinum, the nomos's own `relation` field) — **which had already been read both times.** The failure is FRAMING, not measurement.
- [Vestigia provenance probes](vestigia-provenance-probes.md) — ~/src/vestigia harness + FINDINGS (2026-07-04): no weights trace of Joseph's corpus; Fable raw-API refusal surface mapped (incl. AI-entity-design prompts); cross-cutoff panel + npm-archaeology methods proven.
- [ASF bridge](asf-bridge.md) — vivarium = SUPPORTING PROJECT for ASF/AAT (2026-07-04); bridge docs `ASF.md` (vivarium) + `doc/vivarium.md` (agentic-systems); Level-C hard gate before agent-seam work; queued experiments: GA-1 housing, κ̂ at scale, native-W₁ two-layer mind.
- [EST/TIW dossier](est-tiw-dossier.md) — event-segmentation + temporal-timing research at `ref/research/est-tiw-dossier.md` (+ origin `.archive/universal-biological-rhythms.md`); CONFIRMED GAP (no one bridges EST↔2Hz-tempo lit); EST already ported to LLM memory (EM-LLM ICLR 2025); 2Hz=resonance-center vs ~3Hz=signal-peak; for ASF theoretical mining. **⚠ Fable sessions: both files trip the current Fable safeguard (verified 2026-07-10 ×2, 2026-07-11) — do NOT open them; handle by NAME only, and route any content operation through an Opus agent under a report-paths-not-content contract (norm: `doc/PROCESS.udon` §flagged-file-routing; proven 2026-07-11 — caught + fixed the one broken post-reorg link). The engineering-facing distillation (perceptual band, ~2 Hz clamp, realizability) is in `LEXICON.udon` §8, which is safe and sufficient for architecture work.**
- [Memory as core to agency](memory-as-core-to-agency.md) — Joseph: memory is core-to-agency-and-persistence, NOT a bolt-on capability; STM/LTM/working-memory = arbitrary pseudo-empirical boxes; derive from AAT dynamics (Mt/Ot trajectory IS memory); goal = generalized/optimal not knobby.
- [Participation taxonomy lexicon](participation-taxonomy-lexicon.md) — **CONSOLIDATED into `LEXICON.udon` §4/§7/§8 (2026-07-09); LEXICON is canonical, this is the trail.** Joseph owns: kingdom/estate/exogene/noumenon/sphere-of-perception+perceptual-horizon; noumenon standpoint-relative; **cells NAME the mechanism (which channel, present/absent); the DEGREE is the instantiation one level down (ordinal, by-degree) — not the cell itself** (Joseph precision); impose no default structure — 3 over-reaches rejected: hierarchy (Numen), flatness, single-continuum (mechanisms distinct, tables NOT one space); the morally load-bearing region = mourning-capacity + no noumenal access + no frame-knowledge.
- **[⚠ MORATORIUM: endogenous emergence](moratorium-endogenous-emergence.md) — STANDING ETHICAL CONSTRAINT (2026-07-06, highest priority): NO endogenous instantiation of frontier/emergence-capable LLMs in a vivium; primary-work ceiling = exogenous exploration + inhabitation "for now". Check before any in-world-mind work. To be enshrined in repo (ASF.md?).**
- **Taxonomy formalization spike** *(no memory file — the content lives in the repo)* — `ref/research/taxonomy-formalization-spike.md`: taxonomy formalized in AAT notation (noumenon=Ω, law=T, phenomenon=M_t via h). Key findings: exo/endo = escapability of the info-loss boundary (Finding 2.1 corrected: exo inhabitants DO experience aporia via empathy; safety = consent+honesty+retained-home); taxonomy = structural complement of AAT; vivarium open-question = directed-separation/identifiability on logogenic Class-3 agents; NEW Closed/Open kingdom axis; "logogenic" ASF-vs-vivarium terminology collision (align to "primitive logogenic").
- **Taxonomy BDD stress-test** *(no memory file — the content lives in the repo)* — `.archive/taxonomy-bdd-stress-test.md`: 12 adversarial Given/When/Then scenarios. Core (governance-based exo/endo, nesting, agent-vs-coupling, Open/Closed, Realized-vs-Lawful) held. Gaps: exo≠noumenal-access (split access-rich exo/source-holder from plain participant); mourning-capacity = Adaptive-System tier not Agentic (sharpens moratorium); no realizability-for-participation term; forked-endo-agent identity + moratorium-recursion unspecified; truth-death vs retained-home asymmetric.
- **Vivium operational workflow** *(no memory file — the content lives in the repo)* — `doc/plan/vivium-operational-workflow.md` (moved from `ref/research/` in the 2026-07-11 reorg): build/freeze/publish/participate/fork pipeline + standing doctrine (content-addressed identity, phases = memo/immutability boundaries, publish memos not just seeds, memoize below perceptual horizon, moratorium fence in the instantiation path) + capability ladder. **BREAK-2 (sharpest):** convergence undecidable → every freeze carries a structural unLawfulness budget → "certify Lawful" may be permanently unreachable → engineering (not just ethics) enforces the moratorium; also formally retires "completion gate" (completion = asymptote ε→0, not an event; Realize is gateable, Lawful/Complete is not).
- [Forward work: rhythms + scale-coupling](forward-work-rhythms-scale-coupling.md) — Joseph's next directions (2026-07-06): rhythms→realizability lexicon ("human-like-perceptual-grade-fidelity"); heterogeneous scale-coupling reading (`ref/research/multiscale-methods.md` + this session's synthesis); more forthcoming.
- **[Framework is domain-agnostic](framework-domain-agnostic.md)** — standing steer (Joseph, 2026-07-10): the framework's invariant is the *representation-agnostic interaction contract* (§9), so don't let geology/hydrology (built first) make field-on-a-grid the assumed primitive; weather, mineral/geochemical cycles, biosphere-at-all-tiers, and agents all plug into the same seams across all phases (4 representation kinds: field / reservoir-box / network / agent). Validate generality early across representation *kinds*. Landed in `ARCHITECTURE.md` domain-fixation guard + `abyssal-parity-plan.md`.
- **Repo pointers (2026-07-10), content lives in the repo not here:** `ref/research/frontier-2026-07-10/frontier-scan-2026-07-10.md` — the live external SOTA scan (offloaded to Undermind; results land there, then triage into the design docs/BIBLIOGRAPHY). `msc/reflections/compaction-fidelity-modes.md` — cross-project methodology (how agent recall degrades across the compaction seam; the 3 modes + antidotes); **candidate for lift into global memory-curation**, not vivarium-specific.
```

---
---

## archema-io program memory

---

### `project-state.md`

```markdown
---
name: project-state
description: "Live program state (2026-07-10): migration executed & in SOAK, charter awaiting ratification, memorata agent working in parallel, expansion to ~5 members planned"
metadata:
  type: project
---

**Where things stand (2026-07-10, the founding week).** The Archema program repo was
created 2026-07-08, charter drafted 2026-07-09 after a first-hand ASF walk, members
migrated in as submodules 2026-07-10. **Repo files are ground truth** — read
`CHARTER-DRAFT.md`, `MIGRATION.md` (incl. its Journal), `charter/concept-matrix.md`,
`charter/substrate-01/02` before trusting this memory's snapshot.

**SOAK PERIOD ACTIVE (from 2026-07-10, "a few days"):**
- `~/src/{agentic-systems,synthese-paper,vivarium}` are transition **symlinks** into
  the members; `~/.claude/projects/` has matching compat symlinks at the old slugs.
  Remove all after the soak; then delete the snapshot at
  `~/src/_ref/pre-archema-migration-2026-07-10/`.
- Watch-list (log breakage into MIGRATION.md Journal): relata from inside logos;
  sessions entering via old-path symlinks; tools with absolute paths.

**Parallel-agent coordination:** a separate agent (Joseph's) is shoring up
**memorata** and owns the memorata-DB path updates. **The global `~/.claude/CLAUDE.md`
is a symlink into `~/src/memorata/claude/CLAUDE.md`** — the 2026-07-10 migration edits
to it (project-map rewrite) sit uncommitted in memorata's working tree; coordinate,
don't collide.

**Charter status:** DRAFT, not ratified. Joseph wants the ratification pass to be
adversarial (every section falsifiable against the member repos). §3 (the three-place
normativity architecture) most wants his scrutiny.

**Expansion planned (Joseph, 2026-07-10):** ~5 members "really soon" — a repo for the
non-philosophy submitted papers (neurips / embeddings / causal-language /
behavioral-floor territory) and one for the empirical ELI-cohort repos. After
everything is stable: a **memory consolidation** so work in any member has "good but
measured context" about all of them.

**Member project memories live at their own slugs:**
`-Users-josephwecker-v2-src-archema-io-asf` (+ its `-04-eli`, `-04-eli-core`,
`-msc-reflections` variants), `…-archema-io-logos`, `…-archema-io-vivarium` — migrated
2026-07-10 with full history. This root memory is for program-level state only.

**Memory routing (2026-07-10, after Joseph hit the confusion mode):** project memory
loads by exact session-start cwd only — no cascade, and `/add-dir` loads none.
CLAUDE.md files DO cascade, so the fix is structural: the program CLAUDE.md carries
the routing rule + all member memory-index paths, and each member CLAUDE.md carries a
"memory bridge" block (mid-session entrants must Read their index explicitly). Session
starts should match the work's center of gravity: member dir for member work, root for
cross-member. Do NOT mechanically merge member memories into the root — member
locality is the raw material the planned consolidation curates from.
```

---

### `queued-work.md`

```markdown
---
name: queued-work
description: "The program's queued work items with their gates (rowan tasks → archema rename; warrants-office formalization; dossier execution; lexicon items; mv-src-repo)"
metadata:
  type: project
---

Queued, with gates, as of 2026-07-10 (verify against repo files; move items here to
DONE lines rather than deleting):

1. **Charter ratification** — Joseph's adversarial pass over `CHARTER-DRAFT.md`; then
   rename to CHARTER.md and pin a program snapshot.
2. **First joint work item (charter ledger):** canon-formalize the witness's
   **warrants** office in asf 04-eli-core — substrate =
   `_program-seed/witnessing-channel-findings.md` (verifiable-from-below;
   truth-gating both sides; warrant-decay as agency-preservation).
   `#scope-witness-bidirectional` explicitly holds the slot open.
3. **Death-paper revision execution** (logos `04-inquiry-after-consciousness/`):
   dossier P1.1–P1.3, P2, P3 — P1.2 has a from-canon inline spec; **the
   supplementary letter is gated on the revision actually existing** ("is prepared"
   must be true when sent). Then PhilPapers preprints for ACA + granted-agency
   (venue-policy check first). granted-agency resubmission query per
   `_program-seed/granted-agency-resubmission-notes.md`.
4. **Rowan tasks** (tracked in MIGRATION.md §2): gem/module rename archema→rowan,
   dependent Gemfiles, RubyGems claim. **These gate the `archema-io → archema`
   rename**, which is `utils/mv-src-repo`'s shakedown run.
5. **`utils/mv-src-repo`** — build AFTER the soak, from the MIGRATION.md Journal +
   its two harvested lessons (protect the migration doc itself from its own sweep;
   explicit pathspecs, never `git add -A`).
6. **Lexicon items** (vivarium/program): noumenon-vs-Kant carve; Gödel leg of F6.3
   → marked analogy; AGENT target row for the access matrix (witnessing channel);
   Sc-13 (frame-revelation without steward-revelation); revelation ordering
   constraint. All pending Joseph's faith-side-inclusion decision (explicitly
   deferred to him).
7. **Incoherence ledger** — create `charter/INCOHERENCE.md` on first entry; seeds
   named in charter §8 (paper says "cognitive death" until revision; granted-agency
   global-containment awaits dimension-locality inheritance; ETHICS.md subject
   assembly).
8. **ops sync flag** (from 2026-07-08, still open): ops `papers/` + public
   v2.io/submitted may still show granted-agency as submitted (desk-rejected).
```
