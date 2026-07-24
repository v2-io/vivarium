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
| discussion | Project | [#disc-vivarium-purpose](src/disc-vivarium-purpose.md) | Dual purpose: play and *in vivia* laboratory; claim truth outranks demo polish this phase | draft |
| discussion | Project | [#disc-aat-vivarium-object-map](src/disc-aat-vivarium-object-map.md) | AAT ↔ vivarium object map (unratified synthesis; not exact) | draft |
| discussion | Project | [#disc-unlawfulness-budget](src/disc-unlawfulness-budget.md) | Every Realization carries an unLawfulness budget; Realized ⟂ Lawful | draft |
| scope | Project Critical | [#scope-asf-reading-gates](src/scope-asf-reading-gates.md) | ASF familiarity gates A/B/C; Level C hard-gates agent-seam work | draft |
| scope | Project | [#scope-agent-seam-constraints](src/scope-agent-seam-constraints.md) | Late-phase agent-seam constraints holder (not primary until Level C) | draft |
| normative | Ops Critical | [#norm-declaration-must-convict](src/norm-declaration-must-convict.md) | A declaration that cannot fail is a wish | draft |
| normative | Ops Critical | [#norm-probes-before-claims](src/norm-probes-before-claims.md) | Probes before claims | draft |
| normative | Ops Critical | [#norm-decision-authority](src/norm-decision-authority.md) | Authority tags; evidence ≠ who decided | draft |
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
| formulation | Machine Critical | [#form-flux-web](src/form-flux-web.md) | Nomoi couple by matched flux quantities; unmet = unrunnable | draft |
| formulation | Project Critical | [#form-ordinum-governs-flux-web](src/form-ordinum-governs-flux-web.md) | Ladder promises are flux; `:kept-by` is the producer | draft |
| formulation | Model Critical | [#form-column-control-volume](src/form-column-control-volume.md) | Column is a control volume with sufficient statistics (frame; impl open) | draft |
| observation | Physics | [#obs-hydrosphere-box-nomos](src/obs-hydrosphere-box-nomos.md) | Box nomos proves representation-agnostic contract | draft |
| discussion | Model Critical | [#disc-prime-question](src/disc-prime-question.md) | Prime Question = modified-equation analysis | draft |
| | | | *(#gap) Full Terrestris phase-content segments* | missing |
| | | | *(#gap) Manifest / vivium identity* | missing |

## III. Runtime, environment, and CLI

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Machine Critical | [#form-store-as-save](src/form-store-as-save.md) | The store is the save; memo store is portable vivium state | draft |
| formulation | Machine Critical | [#form-builder-admission](src/form-builder-admission.md) | Builder admits only flux-allowed work; explorers observe-only pull | draft |
| | | | *(#gap) Run-modes carve* | missing |
| | | | *(#gap) Full builder daemon (beacon / demand spool)* | missing |

## IV. Kingdoms, orders, and ordinum

*(#gap) Segment homes for `tabularium/terrestris.ordinum.udon` content — charges, promises, phase gates.*

## V. Physical, mathematical, and algorithmic toolsets

| Type | Aspect | Tag | Claim | Stage |
| --- | --- | --- | --- | --- |
| formulation | Math Critical | [#form-rl-closure-algebra](src/form-rl-closure-algebra.md) | Multiscale $U,u,R,L$ + three honesty laws; $R\circ L=\mathrm{id}$ with compliance debt | draft |
| formulation | Math Critical | [#form-seam-flux-exchange](src/form-seam-flux-exchange.md) | Seams exchange fluxes, not states; one discipline on space × time | draft |
| | | | *(#gap) Grid / staggering (measurements vs open verdict)* | missing |
| | | | *(#gap) Hydrosphere, climate, erosion, water nomoi (beyond box observation)* | missing |

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
