# Vivarium Methodology Core

The specification of what vivarium is and must do, written as **segments** in `core/src/`. **This file carries the ordering; the slug carries the identity; [`FORMAT.md`](../FORMAT.md) carries the rules.** Reordering rows costs nothing — segments are referenced by slug, never by position.

> [!danger] **Read [`FORMAT.md`](../FORMAT.md) before writing or reviewing a segment**

> [!important] **Sole claim source of truth**
>
> Per `#scope-segment-canon`, settled project claims live in segments. Design prose, architecture notes, and spikes are sources and instruments — not parallel canon.

**Every session:** [`CLAUDE.md`](../CLAUDE.md) · this outline · [`LEXICON.udon`](../LEXICON.udon) for terms you touch · [`ETHICS.md`](../ETHICS.md) §0 / `#scope-moratorium-endogenous-emergence` · ASF gates `#scope-asf-reading-gates` (Level C hard-gates agent-seam work).

**Cross-member refs** use `#asf/1-aat/<slug>` etc. (FORMAT §5.2) — never bare paths, never `ASF.md §N` as law.

### Aspect (provisional)

| Aspect | Description |
| --- | --- |
| Math | Mathematical findings and tools |
| Physics | Natural-world behavior |
| Model | Algorithms, approximations, tradeoffs |
| Machine | Implementation and performance |
| Project | Purpose, principles, architecture of the project itself |
| Ops | Process, authority, documentation discipline |
| Critical | Non-negotiable for this project (flag, not exclusive of other aspects) |

---

## How this outline should be organized (standing note)

*Written 2026-07-24 for continuity across sessions and compaction. **Principle over freeze:** the numbered §I–VIII tables below are the current index and will keep moving; this section is the target architecture of the index. When reordering, match *this* cut—not a convenient misread of a thin §VII, not “stuff technical things in V,” and not “VII only gets ordinum `.udon` extract.” Segments stay self-contained by slug; **only this file’s rows move.***

### Goal

**Findability and honest abstraction levels.** An agent (or future-you) should know, for any claim: (1) where to *look*, (2) where to *put* a new segment, (3) what must already exist as dependency, without inventing a second law channel or a false gate.

Vivarium is not ASF volumes: there is **no single proof DAG**. Outline order is **onboarding and taxonomy**, not a theorem sequence. `depends:` on each segment remains the real prerequisite graph; outline order should still **avoid inverted onboarding** (do not park sketches that presuppose multiscale algebra in §I before the toolkit exists).

### Target bands (best cut)

Think **systems library + assembly schedule**, not one junk drawer and not phase-number-as-only-axis.

| Band | Purpose | Holds | Does **not** hold |
|------|---------|--------|-------------------|
| **Foundations** | Why the project exists; non-negotiable walls; how we know things | Telos, moratorium, determinism, fidelity, probe/declaration/authority norms, claim-channel | Water kernels, grid math, store CLI |
| **Abstract machine** | Domain-agnostic coupling and persistence | Nomos, complete key, store-as-save, flux web, nomotheke, pull composition, three-scoped runtime, builder *roles* | Drainage islands, MFD, Abyssal milestone as “runtime” |
| **Shared toolkit** | Reusable physical/mathematical machinery | Grid, R/L algebra, seams, face-flux, structure-preserving *as tools*, CellId/chunk, FVM vocabulary, info-theoretic dictionary | “Erosion is stream-power + …”; sea-level pour law |
| **World systems (by system)** | Terrestris (and later tracks’) **engine content** | Water, erosion/routing, sea-level/freeboard, materials-as-world-substrate, climate, hydrosphere box, fluvial inventory, system-local parallelism | Phase list; store format |
| **Developmental ladder** | When things come online and freeze | Ordinum/phases, charges–promises, Realization/unLawfulness freezes, check-the-ladder, Abyssal *parity as ladder target*, lifecycle of participation | Kernel numerics |
| **Ops** | Consistency of work | Audit integration, toolchain, contribution SOPs | Physics |
| **Appendices** | Visible non-primacy | Failure galleries, worked examples, open research sketches, open-problem census, exploratory agent sketches | Primacy law |

Current §I–VIII are a **partial realization** of these bands (e.g. world systems and phase ladder share §VII today; appendices are §VIII). Prefer evolving toward **clear band boundaries** even if section numbers shift: systems-before-or-beside phases, toolkit separate from systems, abstract machine separate from Abyssal plan detail.

### Placement rules (stop guessing)

1. **True for any multiscale cube-sphere world** → shared toolkit.  
2. **True of this engine’s water / erosion / sea / material / climate laws** → **world systems** (not “principle concepts,” not “runtime”).  
3. **True of when something may run, freeze, or admit participation** → developmental ladder.  
4. **True of store / pull / builder without naming geology** → abstract machine.  
5. **Convicts a system kernel** (MFD fan, routing κ, lake-at-rest WB) → **next to that system**; if it convicts the *algebra* (mean-pin vs $R\circ L$) → toolkit.  
6. **Dead-ends, seductive wrongs, open sketches** → appendices; never mid-Critical path.  
7. **Abyssal parity / ethereal-explorer milestone** → ladder (or world-systems *build target*), **not** CLI/runtime chapter.  
8. **Drainage dependency planning** → world composition (systems or ladder-adjacent), not store docs.  
9. **Phases are not systems.** Water is *named* under Primordial as an example of early content; it **runs across later phases**. Do not file every water finding only under phase 3. Phase segments say *what is charged/promised when*; system segments say *what the law is*.  
10. **Ordinum is the join**, not the only content of the world-engine band: charges/promises/gates *and* nomos findings both need homes; neither replaces the other.

### Failure modes this note exists to prevent

| Failure | What it looked like | Correction |
|---------|---------------------|------------|
| **False VII gate** | “§VII is only phase-floor extract from `terrestris.ordinum.udon`; erosion/water stay in §V tools or §II concepts.” | Invented by over-reading a *thinned* outline. Original scaffold (§VII *Tabularium & Existing Phases & Nomos*, Terrestris as earthlike **world engine**, Primordial/Abyssal as system homes) put **nomos findings** there. Systems library + ladder both belong in the world-engine story. |
| **V as junk drawer** | Everything “technical” lands in toolsets. | Tool = reusable; system = this world’s law. |
| **Phase-only filing** | Every finding under debut phase head. | Cross-phase systems get system homes; phases cite them. |
| **Runtime swallows world** | Abyssal parity, drainage islands under §III because “build/plan.” | Runtime = machine; world target = systems/ladder. |
| **Galleries on critical path** | Defect anatomy next to founding postulates. | Appendices. |
| **Inverted onboarding** | Sketches in §I that `depends` on §V algebra. | Lean foundations; deep depends later. |
| **Defending misorg as “what §VII is for”** when a human names misorg | Treating current hollow tables as intent. | Prefer project-best taxonomy; **do not correct the user out of a real organizational complaint.** |

### Relation to segment files

- **Slug = identity.** Moving a row does not rename a file.  
- **No dual homes.** One primary outline row per segment; appendix rows are still primary *index* homes for non-primacy types.  
- **Gaps stay named.** Missing field-nomos segments and per-phase charge homes remain explicit until written.  
- **Super-archive** is not an outline section; treat ice as deleted for tactical dependence (`#scope-segment-canon`).  
- **Candidate inventory** (`core-segment-candidates-2026-07-14.md`) is archaeology for placement flags (e.g. water/erosion → world engine), not a second outline.

### Suggested evolution (when a session has outline energy)

1. Split **world systems** and **phase ladder** into sibling top-level sections if §VII grows crowded—same band model, clearer scan.  
2. Keep §I **lean** (Critical + purpose + norms); exploratory sketches stay in appendices.  
3. Audit `depends:` vs outline order after big peels; fix inverted clusters, not every edge.  
4. Land missing **field nomos** segments (water kernel, erosion composition, uplift, …) under world systems; land **phase** segments under the ladder that *cite* them via ordinum.

Until then: **use the placement rules above when adding or reviewing rows**, even if today’s § numbers lag the ideal bands.

---

## I. Guiding principles, purpose, and obligations

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| scope | Project Critical | [#scope-segment-canon](src/scope-segment-canon.md) | Segments are the sole source of truth for project claims | draft |
| definition | Project | [#def-vivium](src/def-vivium.md) | A vivium is one citable world-artifact | draft |
| definition | Project | [#def-in-vivia](src/def-in-vivia.md) | *In vivia*: empirical register between toy model and field data | draft |
| discussion | Project | [#disc-vivarium-purpose](src/disc-vivarium-purpose.md) | Dual purpose: play and *in vivia* laboratory; use-case as fidelity contract | draft |
| scope | Project Critical | [#scope-moratorium-endogenous-emergence](src/scope-moratorium-endogenous-emergence.md) | No endogenous frontier / emergence-capable minds | draft |
| postulate | Project Critical | [#post-determinism-as-ontology](src/post-determinism-as-ontology.md) | Determinism as ontology; fated noise | draft |
| formulation | Project Critical | [#form-core-view-wall](src/form-core-view-wall.md) | Core / view wall; views observe only | draft |
| postulate | Project Critical | [#post-represent-by-consequence](src/post-represent-by-consequence.md) | Represent by consequence | draft |
| formulation | Project Critical | [#form-fidelity-invariant](src/form-fidelity-invariant.md) | Fidelity is lazy; materializations prove the macro with bounded deficiencies | draft |
| formulation | Model | [#form-reductionist-fallback-cases](src/form-reductionist-fallback-cases.md) | Three reasons to leave pure reductionism; case C needs regime probes first | draft |
| formulation | Model | [#form-fidelity-ladder](src/form-fidelity-ladder.md) | Climb to discover, descend to surrogate; model identity in the key | draft |
| discussion | Project | [#disc-unlawfulness-budget](src/disc-unlawfulness-budget.md) | Every Realization carries an unLawfulness budget; Realized ⟂ Lawful | draft |
| discussion | Project | [#disc-aat-vivarium-object-map](src/disc-aat-vivarium-object-map.md) | AAT ↔ vivarium object map (unratified synthesis; not exact) | draft |
| scope | Project Critical | [#scope-asf-reading-gates](src/scope-asf-reading-gates.md) | ASF familiarity gates A/B/C; Level C hard-gates agent-seam work | draft |
| scope | Project | [#scope-agent-seam-constraints](src/scope-agent-seam-constraints.md) | Late-phase agent-seam constraints holder (not primary until Level C) | draft |
| normative | Ops Critical | [#norm-declaration-must-convict](src/norm-declaration-must-convict.md) | A declaration that cannot fail is a wish | draft |
| normative | Ops Critical | [#norm-probes-before-claims](src/norm-probes-before-claims.md) | Probes before claims | draft |
| normative | Ops Critical | [#norm-probe-sensitivity](src/norm-probe-sensitivity.md) | Probe sensitivity is part of the probe; known-bad and scale-separation | draft |
| normative | Ops Critical | [#norm-regime-probes](src/norm-regime-probes.md) | Regime probes ship with every rung; known issues get the probe first | draft |
| normative | Ops Critical | [#norm-declared-violation-is-not-license](src/norm-declared-violation-is-not-license.md) | Declared deficiency is disclosure, not permission; conservation ≠ fidelity rung | draft |
| normative | Ops Critical | [#norm-decision-authority](src/norm-decision-authority.md) | Authority tags; evidence ≠ who decided | draft |
| normative | Model Critical | [#norm-bias-vs-noise](src/norm-bias-vs-noise.md) | Bias versus noise is the decisive discretisation audit | draft |
| discussion | Model Critical | [#disc-algorithms-disguise-physics](src/disc-algorithms-disguise-physics.md) | Port the physical claim, not the paper costume | draft |
| discussion | Ops Critical | [#disc-check-the-ladder](src/disc-check-the-ladder.md) | Check the phase ladder before modern-Earth priors | draft |
| normative | Ops | [#ops-audit-integration](src/ops-audit-integration.md) | Audit reports: verify, adjudicate, integrate, then `.integrated/` | draft |
| formulation | Machine | [#form-engine-bevy](src/form-engine-bevy.md) | Bevy is the view/runtime engine (project decision + spike) | draft |
| | | | *(#gap) Epistemology ladder / weak-tier-in-canon* | missing |
| | | | *(#gap) Strengthen-before-soften as vivarium segment* | missing |
| | | | *(#gap) Full ETHICS segmentation (harm triple, redeemer, …)* | missing |

---

## II. Principle concepts and project layout

*Nomos grain, registry, flux coupling, declaration contract — the layout every world-system plugs into. Not the Terrestris systems themselves (those are §VII).*

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| definition | Project | [#def-nomos](src/def-nomos.md) | A nomos is one keyed, versioned article of world-law | draft |
| formulation | Machine Critical | [#form-complete-content-addressed-key](src/form-complete-content-addressed-key.md) | Complete key; over-key never under-key | draft |
| formulation | Project | [#form-in-vivia-citation](src/form-in-vivia-citation.md) | Operational *in vivia* citation: seed + versions + phase memo + intervention script | draft |
| formulation | Machine Critical | [#form-nomotheke-registry](src/form-nomotheke-registry.md) | Nomotheke is the registry contract; undeclared law is unlawful | draft |
| formulation | Machine Critical | [#form-flux-web](src/form-flux-web.md) | Nomos couple by matched flux quantities + statistic/exactness; unmet = unrunnable | draft |
| formulation | Project Critical | [#form-ordinum-governs-flux-web](src/form-ordinum-governs-flux-web.md) | Ladder promises are flux; `:kept-by` is the producer | draft |
| formulation | Project Critical | [#form-kernel-imperative-boundary](src/form-kernel-imperative-boundary.md) | Kernel = only imperative escape hatch; rest is declaration data | draft |
| formulation | Project | [#form-add-system-contract](src/form-add-system-contract.md) | Six-clause contract to add a composable world-system; CHONK prior art; domain-fixation guard | draft |
| formulation | Model Critical | [#form-column-control-volume](src/form-column-control-volume.md) | Column is a control volume with sufficient statistics (frame; impl open) | draft |
| formulation | Project Critical | [#form-manifest-prescribes-vivium](src/form-manifest-prescribes-vivium.md) | Manifest = per-vivium prescription; ordinum = kind floor; no regula artifact for now | draft |
| discussion | Model Critical | [#disc-prime-question](src/disc-prime-question.md) | Prime Question = modified-equation analysis | draft |
| sketch | Project | [#sketch-nomos-declaration-boxes](src/sketch-nomos-declaration-boxes.md) | Nomos boxes ②–⑤ — schema live on NomosDecl; earning procedures open | draft |

---

## III. Runtime, environment, and CLI

*Store, builder/explorer roles, pull composition, multi-process coordination — how the machine runs. Not Abyssal geology or water physics (those are §VII).*

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Machine Critical | [#form-store-as-save](src/form-store-as-save.md) | The store is the save; memo store is portable vivium state | draft |
| formulation | Machine Critical | [#form-builder-admission](src/form-builder-admission.md) | Builder admits only flux-allowed work; explorers observe-only pull | draft |
| formulation | Machine Critical | [#form-depend-by-key-never-latest](src/form-depend-by-key-never-latest.md) | Depend on neighbours by complete key only; never “finest available” | draft |
| formulation | Machine Critical | [#form-pull-query-composition](src/form-pull-query-composition.md) | World law composes by pull of keyed nomos; pattern not generic engine | draft |
| formulation | Machine Critical | [#form-three-scoped-runtime](src/form-three-scoped-runtime.md) | Spine / cones / edit-layer decomposition; time-in-key DAG; prefetch is pure optimization | draft |
| detail | Machine | [#detail-builder-daemon](src/detail-builder-daemon.md) | Unbuilt builder/explorer daemon design: store-as-bus, beacons, demand spool | draft |
| detail | Project | [#detail-vivium-lifecycle](src/detail-vivium-lifecycle.md) | Pipeline stages, capability ladder, BREAK-1/3/4/5 + doctrine | draft |
| | | | *(#gap) Run-modes carve (thin: LEXICON open referents + root isolation; no fat enum)* | missing |
| | | | *(#gap) Builder daemon **implementation** (design owned above; not shipped)* | missing |

---

## IV. Kingdoms, orders, and ordinum

*Kingdom ontology and the ordinum/regula governance layer (the floor and the profile) — not the phase-by-phase physics content (that is §VII).*

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| detail | Project | [#detail-regula-design](src/detail-regula-design.md) | World-level regula conformance design (held; largely unbuilt) | draft |
| | | | *(#gap) Segment homes for ordinum ontology beyond flux-join (charges/promises as claim bodies; phase-list reconciliation)* | missing |

---

## V. Physical, mathematical, and algorithmic toolsets

*Shared machinery every world-engine may use: grid, multiscale algebra, seams, structure-preserving discretisation, FVM vocabulary, addressing. Not the Terrestris water/erosion/geology articles themselves (§VII).*

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Math Critical | [#form-rl-closure-algebra](src/form-rl-closure-algebra.md) | Multiscale $U,u,R,L$ + three honesty laws; $R\circ L=\mathrm{id}$ with compliance debt | draft |
| formulation | Math Critical | [#form-seam-flux-exchange](src/form-seam-flux-exchange.md) | Seams exchange fluxes, not states; one discipline on space × time | draft |
| formulation | Math | [#form-scale-separation-directional](src/form-scale-separation-directional.md) | Fast→slow weak block; declare multirate direction | draft |
| formulation | Math Critical | [#form-face-flux-register](src/form-face-flux-register.md) | Hanging-node single-valued face flux; refluxing under three conditions | draft |
| observation | Math Critical | [#obs-mean-pin-manufactures-seam](src/obs-mean-pin-manufactures-seam.md) | Mean-pin manufactures seam ridge and mass; not $R\circ L=\mathrm{id}$ | draft |
| formulation | Math Critical | [#form-grid-equiangular-staggered](src/form-grid-equiangular-staggered.md) | Equiangular cube-sphere + Arakawa-C + per-cell metrics (tentative keep); router open | draft |
| formulation | Model Critical | [#form-declared-structure-tradeoff](src/form-declared-structure-tradeoff.md) | Structure preservation is a declared trade; structures conflict; linear-only seam crossing | draft |
| formulation | Machine | [#form-cellid-chunk-patch](src/form-cellid-chunk-patch.md) | CellId canonical key; curve orders chunks; Cartesian patch + halo in hot loops | draft |
| formulation | Model Critical | [#form-sphere-continuous-surface-fields](src/form-sphere-continuous-surface-fields.md) | Solid surface fields continuous on $S^2$; never per-face noise charts | draft |
| sketch | Math | [#sketch-dynamic-exponent-seams](src/sketch-dynamic-exponent-seams.md) | Dynamic exponent $z$ per process; timestep-from-quadtree tactic — exploratory | draft |
| detail | Math | [#detail-seam-precedents](src/detail-seam-precedents.md) | Primary-read seam precedents (AMR / multirate / equation-free / HMM) | draft |
| detail | Math | [#detail-structure-scheme-map](src/detail-structure-scheme-map.md) | Structure→scheme catalogue + conflicts + consumer reconstructions | draft |
| detail | Math | [#detail-info-theoretic-discretisation](src/detail-info-theoretic-discretisation.md) | Sampling / anti-alias / Jensen-vs-aliasing / fated-jitter / multires dictionary | draft |
| detail | Math | [#detail-fvm-control-volume](src/detail-fvm-control-volume.md) | Cardiff FVM taxonomy: constructions, volume-exact reconstruction, null-space probe | draft |

---

## VI. Process and contribution

*SOP-grade consistency and tooling. Critical norms that bind claims also appear in §I.*

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| detail | Ops | [#detail-epistemics-toolchain](src/detail-epistemics-toolchain.md) | Structural vs harness tooling queue; wired clippy; skip reasons | draft |

Candidate inventory (unratified scan, not canon): [`core-segment-candidates-2026-07-14.md`](../core-segment-candidates-2026-07-14.md).

---

## VII. Tabularium — Terrestris phases & nomos

**Terrestris is the earthlike world engine.** Findings about its systems — water, erosion, geology, climate, materials as world substrate, phase build targets — live here so they have an organized home next to the phase ladder. Shared multiscale/grid *tools* stay in §V; *this world’s* laws and measured defects of *those kernels* live here.

Live floor data: [`tabularium/terrestris.ordinum.udon`](../tabularium/terrestris.ordinum.udon). Phase list (1–9) matches that ladder; segment homes per phase still sparse.

### Systems, materials, and convictions (cross-phase)

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Model | [#form-temporal-lod-regimes](src/form-temporal-lod-regimes.md) | Space/time LOD one gradient; four materialization regimes | draft |
| formulation | Model | [#form-material-property-interface](src/form-material-property-interface.md) | Material property set = stable interface; model ladder behind it | draft |
| observation | Physics | [#obs-hydrosphere-box-nomos](src/obs-hydrosphere-box-nomos.md) | Box nomos proves representation-agnostic contract | draft |
| formulation | Physics Critical | [#form-derived-sea-level](src/form-derived-sea-level.md) | Sea level = pour ocean into solid hypsometry; freeboard earns emerged land | draft |
| observation | Model Critical | [#obs-cube-locked-kernel-bias](src/obs-cube-locked-kernel-bias.md) | MFD fan + uniform cell-area $A$ are cube-locked biases that do not converge | draft |
| observation | Model Critical | [#obs-routing-curl-spiral](src/obs-routing-curl-spiral.md) | Routing violates contour-orthogonality: ~2% flux spirals, level-independent | draft |
| detail | Project | [#detail-phenomena-systems-map](src/detail-phenomena-systems-map.md) | Systems inventory, multirate bands, build-order judgment, fluvial ladder | draft |
| detail | Machine | [#detail-drainage-dependency-planning](src/detail-drainage-dependency-planning.md) | Drainage islands as dependency map; flux magnitude sets pull fidelity | draft |
| detail | Machine | [#detail-water-parallelism](src/detail-water-parallelism.md) | Gather-not-scatter; CPU reference; GPU as keyed rung | draft |
| detail | Project | [#detail-abyssal-parity-build](src/detail-abyssal-parity-build.md) | Six-phase path to ethereal explorer in early-Abyssal | draft |
| | | | *(#gap) Full field nomos segments: water kernel, erosion composition, uplift-as-nomos, climate, … (beyond box + sea-level + measured defects)* | missing |
| | | | *(#gap) Per-phase claim homes (charges/promises/gates as segments under the phase heads below)* | missing |

### Phase ladder

1. **Ante-Mundane**  
2. **Protogenic**  
3. **Primordial** — e.g. water-cycle & atmospheric systems  
4. **Abyssal** — e.g. geological-intensive systems  
5. **Primeval** — e.g. early life, evolution, ecology  
6. **Archaic** — e.g. complex life; primitive endogenous agents  
7. **Aboriginal** — e.g. language & complex endogenous agents  
8. **Prehistoric** — e.g. hominid & Holocene-like dynamics  
9. **Scribal** — e.g. writing & game-world-like targets  

---

## VIII. Appendices — galleries, open frontiers, exploratory sketches

*Teaching support, failure galleries, open research, and non-primary sketches. Not the critical-path law tables above. Dead-ends and seductive wrong answers live here so they stay visible without pretending primacy.*

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| detail | Model | [#detail-nomos-defect-anatomy](src/detail-nomos-defect-anatomy.md) | Jul-13 defect→box map; earning-procedure shape; failure-gallery / seductive-wrong appendix | draft |
| worked-example | Model | [#worked-example-mfd-prime-question](src/worked-example-mfd-prime-question.md) | MFD as disguised geometric claim — Prime Question specimen | draft |
| sketch | Model | [#sketch-detail-abstract-reversion](src/sketch-detail-abstract-reversion.md) | Reversion frame; open: nonlinear closure for non-local flux (state half retired as measured) | draft |
| discussion | Project | [#disc-open-problem-census](src/disc-open-problem-census.md) | Open work derived from core gaps — not ranked superlatives | draft |
| sketch | Project | [#sketch-three-project-axes](src/sketch-three-project-axes.md) | Graphics / world / agents as independent budgets — exploratory | draft |
| sketch | Project | [#sketch-logozoetic-peer-view](src/sketch-logozoetic-peer-view.md) | Typed A/O peer view for logozoetic play — exploratory | draft |
| sketch | Project | [#sketch-memory-as-core-to-agency](src/sketch-memory-as-core-to-agency.md) | Memory core to agency (Joseph research direction) — exploratory | draft |
| sketch | Project | [#sketch-two-layer-mind](src/sketch-two-layer-mind.md) | Fast formal / slow LLM-at-aporia agent sketch — exploratory | draft |

---

## Transitional note

Claim law is `core/`. Live residual process prose is mainly [`doc/PROCESS.udon`](../PROCESS.udon). `LEXICON.udon`, `DECISIONS`, code, and `tabularium/` remain instruments and data. Super-archive is provenance only — treat as deleted for tactical dependence. The Jul 14 candidate list is an unratified scan, not a second outline.

**Organization target:** see **“How this outline should be organized”** above (standing note). Tables below that note will keep evolving toward those bands; the note is the durable intent.

Pacing / residual intuition (not canon): [`CONSOLIDATION-STATUS.md`](../CONSOLIDATION-STATUS.md).
