# Vivarium Methodology Core

The specification of what vivarium is and must do, written as **segments** in `core/src/`. **This file carries the ordering; the slug carries the identity; [`FORMAT.md`](../FORMAT.md) carries the rules.**

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

## I. Guiding principles, purpose, and obligations

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| scope | Project Critical | [#scope-segment-canon](src/scope-segment-canon.md) | Segments are the sole source of truth for project claims | draft |
| definition | Project | [#def-vivium](src/def-vivium.md) | A vivium is one citable world-artifact | draft |
| definition | Project | [#def-in-vivia](src/def-in-vivia.md) | *In vivia*: empirical register between toy model and field data | draft |
| scope | Project Critical | [#scope-moratorium-endogenous-emergence](src/scope-moratorium-endogenous-emergence.md) | No endogenous frontier / emergence-capable minds | draft |
| postulate | Project Critical | [#post-determinism-as-ontology](src/post-determinism-as-ontology.md) | Determinism as ontology; fated noise | draft |
| formulation | Project Critical | [#form-core-view-wall](src/form-core-view-wall.md) | Core / view wall; views observe only | draft |
| postulate | Project Critical | [#post-represent-by-consequence](src/post-represent-by-consequence.md) | Represent by consequence | draft |
| formulation | Project Critical | [#form-fidelity-invariant](src/form-fidelity-invariant.md) | Fidelity is lazy; materializations prove the macro with bounded deficiencies | draft |
| formulation | Model | [#form-reductionist-fallback-cases](src/form-reductionist-fallback-cases.md) | Three reasons to leave pure reductionism; case C needs regime probes first | draft |
| formulation | Model | [#form-fidelity-ladder](src/form-fidelity-ladder.md) | Climb to discover, descend to surrogate; model identity in the key | draft |
| sketch | Model | [#sketch-detail-abstract-reversion](src/sketch-detail-abstract-reversion.md) | Open: reversion / discrete-edit up-invalidation — not primacy | draft |
| sketch | Project | [#sketch-nomos-declaration-boxes](src/sketch-nomos-declaration-boxes.md) | Nomos boxes ②–④ geometry/semantics/structure — exploratory schema | draft |
| discussion | Project | [#disc-vivarium-purpose](src/disc-vivarium-purpose.md) | Dual purpose: play and *in vivia* laboratory; claim truth outranks demo polish this phase | draft |
| sketch | Project | [#sketch-three-project-axes](src/sketch-three-project-axes.md) | Graphics / world / agents as independent budgets — exploratory map | draft |
| formulation | Machine | [#form-engine-bevy](src/form-engine-bevy.md) | Bevy is the view/runtime engine (project decision + spike) | draft |
| sketch | Project | [#sketch-logozoetic-peer-view](src/sketch-logozoetic-peer-view.md) | Typed A/O peer view for logozoetic play — exploratory | draft |
| sketch | Project | [#sketch-memory-as-core-to-agency](src/sketch-memory-as-core-to-agency.md) | Memory core to agency (Joseph research direction) — exploratory | draft |
| discussion | Project | [#disc-aat-vivarium-object-map](src/disc-aat-vivarium-object-map.md) | AAT ↔ vivarium object map (unratified synthesis; not exact) | draft |
| discussion | Project | [#disc-unlawfulness-budget](src/disc-unlawfulness-budget.md) | Every Realization carries an unLawfulness budget; Realized ⟂ Lawful | draft |
| scope | Project Critical | [#scope-asf-reading-gates](src/scope-asf-reading-gates.md) | ASF familiarity gates A/B/C; Level C hard-gates agent-seam work | draft |
| scope | Project | [#scope-agent-seam-constraints](src/scope-agent-seam-constraints.md) | Late-phase agent-seam constraints holder (not primary until Level C) | draft |
| normative | Ops Critical | [#norm-declaration-must-convict](src/norm-declaration-must-convict.md) | A declaration that cannot fail is a wish | draft |
| normative | Ops Critical | [#norm-probes-before-claims](src/norm-probes-before-claims.md) | Probes before claims | draft |
| normative | Ops Critical | [#norm-probe-sensitivity](src/norm-probe-sensitivity.md) | Probe sensitivity is part of the probe; known-bad and scale-separation | draft |
| normative | Ops Critical | [#norm-regime-probes](src/norm-regime-probes.md) | Regime probes ship with every rung; known issues get the probe first | draft |
| normative | Ops Critical | [#norm-declared-violation-is-not-license](src/norm-declared-violation-is-not-license.md) | Declared deficiency is disclosure, not permission; conservation ≠ fidelity rung | draft |
| normative | Ops Critical | [#norm-decision-authority](src/norm-decision-authority.md) | Authority tags; evidence ≠ who decided | draft |
| sketch | Project | [#sketch-two-layer-mind](src/sketch-two-layer-mind.md) | Fast formal / slow LLM-at-aporia agent sketch — exploratory, not primary | draft |
| normative | Model Critical | [#norm-bias-vs-noise](src/norm-bias-vs-noise.md) | Bias versus noise is the decisive discretisation audit | draft |
| discussion | Model Critical | [#disc-algorithms-disguise-physics](src/disc-algorithms-disguise-physics.md) | Port the physical claim, not the paper costume | draft |
| discussion | Ops Critical | [#disc-check-the-ladder](src/disc-check-the-ladder.md) | Check the phase ladder before modern-Earth priors | draft |
| normative | Ops | [#ops-audit-integration](src/ops-audit-integration.md) | Audit reports: verify, adjudicate, integrate, then `.integrated/` | draft |
| | | | *(#gap) Epistemology ladder / weak-tier-in-canon* | missing |
| | | | *(#gap) Strengthen-before-soften as vivarium segment* | missing |
| | | | *(#gap) Full ETHICS segmentation (harm triple, redeemer, …)* | missing |

## II. Principle concepts and project layout

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| definition | Project | [#def-nomos](src/def-nomos.md) | A nomos is one keyed, versioned article of world-law | draft |
| formulation | Machine Critical | [#form-complete-content-addressed-key](src/form-complete-content-addressed-key.md) | Complete key; over-key never under-key | draft |
| formulation | Project | [#form-in-vivia-citation](src/form-in-vivia-citation.md) | Operational *in vivia* citation: seed + versions + phase memo + intervention script | draft |
| formulation | Machine Critical | [#form-nomotheke-registry](src/form-nomotheke-registry.md) | Nomotheke is the registry contract; undeclared law is unlawful | draft |
| formulation | Machine Critical | [#form-flux-web](src/form-flux-web.md) | Nomoi couple by matched flux quantities + statistic/exactness; unmet = unrunnable | draft |
| formulation | Project Critical | [#form-ordinum-governs-flux-web](src/form-ordinum-governs-flux-web.md) | Ladder promises are flux; `:kept-by` is the producer | draft |
| formulation | Project Critical | [#form-kernel-imperative-boundary](src/form-kernel-imperative-boundary.md) | Kernel = only imperative escape hatch; rest is declaration data | draft |
| formulation | Model Critical | [#form-column-control-volume](src/form-column-control-volume.md) | Column is a control volume with sufficient statistics (frame; impl open) | draft |
| observation | Physics | [#obs-hydrosphere-box-nomos](src/obs-hydrosphere-box-nomos.md) | Box nomos proves representation-agnostic contract | draft |
| formulation | Physics Critical | [#form-derived-sea-level](src/form-derived-sea-level.md) | Sea level = pour ocean into solid hypsometry; freeboard earns emerged land (not decreed continents) | draft |
| formulation | Model Critical | [#form-sphere-continuous-surface-fields](src/form-sphere-continuous-surface-fields.md) | Solid surface fields are continuous on $S^2$ (unit-sphere sampling); never per-face noise charts | draft |
| formulation | Project Critical | [#form-manifest-prescribes-vivium](src/form-manifest-prescribes-vivium.md) | Manifest = per-vivium prescription; ordinum = kind floor; no regula artifact for now | draft |
| discussion | Model Critical | [#disc-prime-question](src/disc-prime-question.md) | Prime Question = modified-equation analysis | draft |
| | | | *(#gap) Full Terrestris phase-content segments* | missing |

## III. Runtime, environment, and CLI

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Machine Critical | [#form-store-as-save](src/form-store-as-save.md) | The store is the save; memo store is portable vivium state | draft |
| formulation | Machine Critical | [#form-builder-admission](src/form-builder-admission.md) | Builder admits only flux-allowed work; explorers observe-only pull | draft |
| formulation | Machine Critical | [#form-depend-by-key-never-latest](src/form-depend-by-key-never-latest.md) | Depend on neighbours by complete key only; never “finest available” | draft |
| formulation | Machine Critical | [#form-pull-query-composition](src/form-pull-query-composition.md) | World law composes by pull of keyed nomoi; pattern not generic engine | draft |
| | | | *(#gap) Run-modes carve (thin: LEXICON open referents + root isolation; no fat enum)* | missing |
| | | | *(#gap) Full builder daemon (beacon / demand spool)* | missing |
| formulation | Machine Critical | [#form-three-scoped-runtime](src/form-three-scoped-runtime.md) | Spine / cones / edit-layer decomposition; time-in-key DAG; prefetch is pure optimization | draft |

## IV. Kingdoms, orders, and ordinum

*(#gap) Segment homes for `tabularium/terrestris.ordinum.udon` content — charges, promises, phase gates.*

## V. Physical, mathematical, and algorithmic toolsets

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Math Critical | [#form-rl-closure-algebra](src/form-rl-closure-algebra.md) | Multiscale $U,u,R,L$ + three honesty laws; $R\circ L=\mathrm{id}$ with compliance debt | draft |
| formulation | Math Critical | [#form-seam-flux-exchange](src/form-seam-flux-exchange.md) | Seams exchange fluxes, not states; one discipline on space × time | draft |
| formulation | Math | [#form-scale-separation-directional](src/form-scale-separation-directional.md) | Fast→slow weak block; declare multirate direction (not dual-home multirate package) | draft |
| formulation | Math Critical | [#form-face-flux-register](src/form-face-flux-register.md) | Hanging-node single-valued face flux (Berger–Colella register); refluxing ceases under three conditions | draft |
| observation | Math Critical | [#obs-mean-pin-manufactures-seam](src/obs-mean-pin-manufactures-seam.md) | Mean-pin manufactures seam ridge and mass; not $R\circ L=\mathrm{id}$; zero-physics control | draft |
| formulation | Math Critical | [#form-grid-equiangular-staggered](src/form-grid-equiangular-staggered.md) | Equiangular cube-sphere + Arakawa-C + per-cell metrics (tentative keep); router open; cube-locked control mandatory | draft |
| observation | Model Critical | [#obs-cube-locked-kernel-bias](src/obs-cube-locked-kernel-bias.md) | MFD fan + uniform cell-area A are cube-locked biases that do not converge; cube control mandatory | draft |
| observation | Model Critical | [#obs-routing-curl-spiral](src/obs-routing-curl-spiral.md) | Routing violates contour-orthogonality: ~2% flux spirals, level-independent; face-centre probe is a null test | draft |
| formulation | Model Critical | [#form-declared-structure-tradeoff](src/form-declared-structure-tradeoff.md) | Structure preservation is a declared trade; structures conflict; linear-only seam crossing | draft |
| formulation | Machine | [#form-cellid-chunk-patch](src/form-cellid-chunk-patch.md) | CellId canonical key; curve orders chunks; Cartesian patch + halo in hot loops | draft |
| formulation | Model | [#form-temporal-lod-regimes](src/form-temporal-lod-regimes.md) | Space/time LOD one gradient; four materialization regimes (global / lazy / catch-up / attractor) | draft |
| formulation | Model | [#form-material-property-interface](src/form-material-property-interface.md) | Material property set = stable interface; model ladder behind it; undifferentiated types refine as deterministic prefix | draft |
| sketch | Math | [#sketch-dynamic-exponent-seams](src/sketch-dynamic-exponent-seams.md) | Dynamic exponent $z$ per process; timestep-from-quadtree tactic — exploratory | draft |
| | | | *(#gap) Hydrosphere, climate, erosion, water nomoi (beyond box + #form-derived-sea-level)* — residual: wire pour/freeboard keeper into nomotheke; full field nomos segments | missing |

## VI. Process and contribution

*(FORMAT and DECISIONS already bind; further SOP segments as needed. Candidate inventory: [`core-segment-candidates-2026-07-14.md`](../core-segment-candidates-2026-07-14.md) — not canon.)*

## VII. Tabularium — Terrestris ordinum phases

Phase structure is codified in [`tabularium/terrestris.ordinum.udon`](../tabularium/terrestris.ordinum.udon). Segment homes per phase are **missing** until extracted.

1. Ante-Mundane  
2. Protogenic  
3. Primordial  
4. Abyssal  
5. Primeval  
6. Archaic  
7. Aboriginal  
8. Prehistoric  
9. Scribal  

---

## Transitional note

A large true corpus still lives outside `core/` (`doc/`, `LEXICON.udon`, `DECISIONS`, code, `.archive/`). That corpus is **source material**. Until a claim has a slug, do not treat the old prose as law — extract it, or leave it marked unowned. The Jul 14 candidate list is an unratified scan to speed extraction, not a second outline.

Pacing / residual intuition (not canon): [`CONSOLIDATION-STATUS.md`](../CONSOLIDATION-STATUS.md).
