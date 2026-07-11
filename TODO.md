# vivarium — TODO

*Tactical open items. Ongoing thinking lives in `ORIENTATION.md` (current state),
the `DESIGN-*` docs (design), and `SUPERSEDED.md` (what's been retired/replaced).
This file is the actionable queue.*

## Lexicon / terminology-system reconciliation (archema-wide) — deferred, tracked

Vivarium's `LEXICON.md` is a **hand-authored** markdown file. ASF has a more mature system we'll want to converge on archema-wide (Joseph, 2026-07-09) — for now this is parked so we can stay focused on lexicon content, architecture, and next steps.

The ASF conventions worth adopting (`~/src/archema-io/asf/`, `doc/sop/format.sop.md`
+ `terminology/README.md`):

- **Generated LEXICON from per-entry files.** ASF's `LEXICON.md` is *generated* by
  `bin/term render` from `terminology/entries/<slug>.md` (frontmatter + prose body),
  with append-only decision events — never hand-edited. Per-entry files make concurrent multi-agent edits merge-clean instead of colliding on table rows.
  *Decision needed:* port `bin/term` into vivarium, or run one shared terminology store across archema members (the charter's cross-member concern).
- **Format/rendering conventions** (partly applied to `LEXICON.md` §4/§7/§8 already):
  - **Math is `$…$` LaTeX**, not bare Unicode, with GitHub/Obsidian-compat rules
    (`\vert`/`\lt`/`\gt`/`\ast`, `\mathcal{}`, no inner spaces). *Done:* bare-math in
    §5/§7/§8 converted. *Pending:* a full pass over §1–§6 (prior-session Unicode).
  - **Segment-voice** — state what *is*, not what *changed*; provenance/retired-term archaeology → history layer (`SUPERSEDED.md`), not the lexicon body. *Done for the sections touched 2026-07-09.*
  - **One-logical-line paragraphs** (no fixed-column hard-wrapping). *Pending:*
    ~230 hard-wrapped lines flagged by `bin/lint-md` — cosmetic (they reflow fine),
    low priority; do alongside the LaTeX pass.
  - **`bin/lint-md`** is the enforcement tool. Vivarium has no local copy; for now run ASF's: `~/src/archema-io/asf/bin/lint-md LEXICON.md`. (Known false-positive:
    its 4-space-indent "bare equation" heuristic misreads nested-list continuation.)

**Next concrete step when we return to this:** decide port-`bin/term`-vs-shared-store,
then either migrate `LEXICON.md` entries into `terminology/entries/` or finish the in-place format pass (§1–§6 LaTeX + de-hard-wrap). Not urgent.

## Native↔canonical representation machinery — deferred, tracked

Joseph (2026-07-10, the session that ran out of context; landed here 2026-07-10 by its successor): systems will often want **domain-specific internal representations** — faster and smaller than columns-on-the-grid — memoized natively and *projected* to the canonical `CellId` frame by lazy keyed nomoi. Named examples: tilted-slab bodies computed as horizontal stacks (+ latent dip); the fluvial cells→receivers linked list (CHONK's graph); closed-form coefficient blocks. The interplay with current concepts is recorded in `LEXICON.md` §2 (*native representation / canonical frame*). **Figure out the general machinery when it's tactically intuitive** — i.e. when the first real second-representation system lands (most likely the drainage graph becoming a first-class store object in plan-Phase 2, or slab bodies) — not in advance.

## The water system, decomposed (Joseph's probe, 2026-07-10: "principled water without a cycle or global conservation?" — No)

`water-tile` (increment #9) is only the **routing/settling rung**; its declaration says so. The principled water system is several nomoi, and it **rides WITH the parity track, not behind it** *(corrected 2026-07-10 — "post-parity" was the session's invention, not Joseph's: conservation is consistency law #2 of the operator algebra and DESIGN-REDUX §12's sufficient-statistics clause is the standing constraint; no pulling from nothing, no draining into nothing, even at the crudest rung. fBm remains the ONE acknowledged fundamental cheat, to be undone.)*:

1. **Planetary water inventory** — conserved reservoir set (ocean/atmosphere/groundwater/ice), declared as law; global conservation lives here. Doubles as the framework's first **reservoir/box-model** system → the domain-fixation guard's generality proof (ARCHITECTURE §0 note; four representation kinds).
2. **Sea level derived** from ocean reservoir + hypsometry (`SEA_LEVEL_M` ledger row's inversion).
3. **Crude climate nomos** — insolation (built) + **spin → circulation band structure** (Coriolis enters HERE, parameterized: bands as f(rotation rate)) → geographic precip field → **retires the rain fudge** (water_tile rains climate's output, not ×10 uniform). DESIGN-SYSTEMS build-order #2.
4. **Surface routing** = today's water-tile, re-forced by climate.
5. **Groundwater/Darcy; ocean circulation** — existing DESIGN-SYSTEMS rows (Coriolis: gyres live in #5; storm rotation with weather; tile streams honestly Rossby-negligible).

## Regula Terrestris v0 + the Ordinum — the conformance layer (specs the reservoir/thermal work; drives today)

Design: `doc/plan/regula-conformance-design.md` **v2** (2026-07-11 — naming settled: regula ✅ / **Regula Terrestris** ✅ / **Ordinum** ✅; LEXICON §1/§4/§5). Build order, thin:

1. **`consumes` field on `NomosDecl`** — quantities in, as shared constants referenced by both bequests and consumes (typos fail the build; the permit-voiding prerequisite found 2026-07-11).
2. **`regula.rs`** sibling to the nomotheke: `Slot` / `Permit { absent_slots, void_on, note }` / `Regula`; **`TERRESTRIS_V0`** — three slots filled; grandfathered dated permits: moon, magnetosphere×solar-wind, **atmosphere (retirement = the reservoir layer — the rain-without-a-sky specimen written down at last)**.
3. **Ordinum v0** derived from doc/PHASES.md for the target phase only (later phases = horizon, never failures); doc/PHASES.md stays the prose source (its reportatio) until codification completes, then archives.
4. Conformance report into `vivarium status` (conformance-to-pin + gap-to-head); voided-permit check in tests.
5. Later, when it earns its keep: `vivarium audit --add <nomos>` — the requisite-closure dry-run (the anti-whim planner).

**First forced decision the Terrestris audit will surface (design doc §7.6, Joseph's call):** Phase-2's abiogenesis-through-photosynthetic-sea-life charge vs our zero biosphere — declared low-tier reservoir-grade stand-in (photosynthesis as box-flux; doubles as the first non-field representation kind) or an explicit dated permit.

## The thermal spine — two parallel nomoi, minor boundary exchange (Joseph-confirmed, 2026-07-10)

1. **`mantle-thermal`** — zero-D thermal-evolution ODE: radiogenic decay (U/Th/K — decay constants exact; *abundances are a Phase-0 charge*, tying element conservation into the spine) vs parameterized convection (Nu–Ra; regime-dependent per Cawood's stagnant/squishy/plate modes). Initial $T_p$ page-read (Dong Table 1). **Fated plume/upwelling noise about the secular trend** = the uplift nomos's source term, principled as stochastic realization of a lawful curve (Chowdhury 2021's driver). Sun-irrelevant (flux ratio ~4000×).
2. **`climate-ebm`** — Budyko–Sellers 1-D latitudinal energy balance: annual-mean insolation (built) + declared albedo + greenhouse from the atmosphere reservoir → $T(\mathrm{lat})$ steady state → evaporation/precip geography (retires the rain fudge). Requires: atmosphere reservoir; **solar constant promoted to $S(t)$** in the P0 block (faint young sun ~75–80%; the dense Phase-2 greenhouse is what keeps water liquid).

**The seam** (first aspect-coupling seam between non-field systems; cleanest possible instance): $Q_{geo}(t)$ up (~0.09 W/m² — small, accounted), mean $T_s$ down (mantle upper BC). Two scalars, no shared state; timescale separation Gyr-vs-year satisfies Gear–Wells by nature.

## Prior v3 — phase-correct it (Joseph, 2026-07-10)

The next spine rung is the **Phase-2 Bequest**, not a mid-Abyssal snapshot: a *submerged* world — generous water over **~modern-depth-or-shallower basins** (corrected 2026-07-10: thick buoyant mafic seafloor; basins deepen over time; inventory ~1.3 OM contested) over seeded-asymmetry crust, ~0% land beyond transient volcanic specks ("meaningful non-volcanic land above sea level" is a #gate charge **Abyssal must deliver** via uplift/proto-tectonic nomoi, never an initial condition — doc/PHASES.md). The early-continents survey's §6 table (land 2–15%, compressed bimodal, relief 3–5 km) is the **mid-Abyssal verification target** those processes must *produce* — a Record-style check partway through the phase. Sea level derived from a declared water inventory (ASSUMPTIONS entry), retiring `SEA_LEVEL_M`. Element/stable-chemical abundances belong to the same P0/P2 declaration work and get their own conservation ledger rows when reservoirs exist.

## Cross-member: nomos / regula / ordinum → concept-matrix rows

**nomos/nomoi** settled 2026-07-10; **regula / Regula Terrestris / Ordinum** settled 2026-07-11 (LEXICON §1/§2/§5). The program-level `charter/concept-matrix.md` should gain rows for the executable law-unit (vivarium: nomos; ASF: nearest = the authored $T$/$\theta$ machinery; phil: nomothetic), the world-conformance profile (regula), and the codified phase floor (ordinum) — a root-session/cross-member edit, queued rather than done from inside the member. Note also: "regula" here ≠ the program's CHARTER-DRAFT (the collision that forced the rename — worth a matrix footnote so no one re-collides).

## Phase 0 — declare the Ante-mundane parameter block

Phase 1's charges (tilt, spin, insolation rhythm) exist as exact code (`planet.rs`); Phase 0's charges are mostly **implicit hardcoded constants** (Earth radius, tilt 23.44°, solar constant; orbit/mass/abundances/moon not declared anywhere). Per *declare causally, materialize lazily* (PHASES design notes), the P0 Bequest ("the constants themselves — the box") wants an explicit declared parameter block — law-side identity (manifest identity bucket or a `planet` nomos-version), not view-side. Neither phase is *gated* yet (no phase machinery — fine until component E / the first freeze). Found 2026-07-10 while wiring the globe's sun ephemeris.

## Component E — time-indexed stage chains (un-drop decision, then build)

`doc/plan/framework-to-status-quo.md` §4's component **E** (the settle sequence as explicit chained nomoi with time-indices, each stage a memo with its recorded convergence-ε) did **not** carry into `doc/plan/abyssal-parity-plan.md`'s six phases, and no deliberate deferral is recorded — it fell through the consolidation crack (found 2026-07-10). It keeps resurfacing as load-bearing: it is what makes "the beginning of Phase 3.3" *addressable* (beacons need it), what makes intermediate stages monitorable by explorers, what the "watch erosion happen while floating downstream" experience plays back, and its recorded ε *is* the unLawfulness budget (LEXICON §7.2) as data. Recommendation: treat it as plan-Phase-2½ — build it when the builder daemon's stage progression first needs a second time-index.

## Small UX: unify default world dirs

The CLI takes an explicit `<dir>`; the globe defaults to `~/.cache/vivarium/globe-world`; `store_explore` uses `$TMPDIR/vivarium-store-explore`. A fresh user running all three sees three different worlds. Pick one convention (likely: a `~/.local/share/vivarium/<name>` home + `VIVARIUM_WORLD` respected everywhere) when the FP explorer lands. (Don't silently move Joseph's existing `globe-world` — first-light lives there.)

## Kernel physics on deck (worldview-testbench era — still live, relocated from ORIENTATION 2026-07-11)

The kernels are canonical even though the testbench architecture is donor-only; these carry into the frame's fine-tier nomoi (#10) and beyond. Full context in the referenced docs, not here:

- **Sediment coupling** (Joseph, 2026-07-02): time-averaged discharge → erosion's $A$; deposition into slack water → oxbows, lake→meadow fill — the honest water↔erosion core, currently off.
- **Per-material erodibility** — the strata-honoring rung (DESIGN-SYSTEMS §Cordonnier/Braun: CCB\*18 is the published prior art; → Bryce hoodoos).
- **Highest-leverage three** by visible-truth-per-effort: grain-size split + Stokes settling · bank mechanics (Mohr–Coulomb undercut → meandering) · Darcy lateral groundwater (springs). DESIGN-SYSTEMS §Sediment inventory.
- **Analytic hydrological init** — solve the equilibrium instead of the ~2 h deluge fill (`ref/erosion-port/NOTES.md` §Next); also the principling path for the water-fill ASSUMPTIONS row (convergence-ε via component E).
- **Water parallelism** — gather rewrite → rayon 5–8× → wgpu compute 20–40×, CPU-reference determinism policy (`doc/plan/water-parallelism.md`).
- **⚖ Parked physics decision (Joseph):** Fischer-form eddy diffusivity (`ref/research/eddy-fischer-experiment.diff`) + whether armor should form under supply-rich transients at all (Dietrich 1989: armor expresses supply *deficit* — the `armor_regimes` probe's expectation may be the wrong part).

## Explorer intents (Joseph, 2026-07-02 — inherited by the FP explorer, #11)

Stated for the testbench, but they are intents about *experiencing the world* and bind the frame-native explorer's design (some partially landed in worldview — re-triage when #11 is designed): deepen the sediment loop (suspension/sealing/deposition) · reconcile column state with DESIGN-MATERIAL's strata Column · fine nearby water correctly SEAMED to coarser tiers (the nested water telescope) · HUD clarity + toggleable legend · water transparency · pawn visible even under deep water · WASD step/speed precision · water visualization vs velocity/pitch/suspension · ground visualization vs column state (saturation, exfiltration) · wet-ground slumping (saturated banks fail) · **legible, steerable time regimes** (in-world clock vs wall clock, sim rate vs framerate — pre-history vs current sim).

## Session housekeeping

- `doc/design/DESIGN-REDUX.md` still uses "checkpoint" in four runtime-durability sites —
  decide *memo* vs a distinct persist-boundary name (LEXICON §5 collision ledger,
  `SUPERSEDED.md`). **Still open.**
