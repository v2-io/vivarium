# Theory + plan mining report — 2026-07-21

**Slice:** `doc/theory/` (discretisation-and-information, multiscale-methods, multiscale-seams) · `doc/plan/` (abyssal-parity, framework-to-status-quo, builder-explorer, vivium-operational-workflow, water-parallelism, regula-conformance as reasoning trail only).

**Not law.** EXTRACT / DEMOTE / ICE only. Orchestrator owns OUTLINE promotion and commit.

**Parallel-wave collisions noted:** design/decisions agents already landed `#form-store-as-save`, `#norm-bias-vs-noise`, `#form-fidelity-invariant`, `#form-column-control-volume`. Theory-plan wave **does not re-own** those; fork `form-save-is-memo-store.md` marked for **deletion**.

---

## Top 5 extracts (this wave)

| Rank | Slug | Type / status | One-line claim | Primary sources |
|------|------|---------------|----------------|-----------------|
| 1 | **`#form-rl-closure-algebra`** | formulation / robust-qualitative | Four objects $U,u,R,L$ + three laws; $R\circ L=\mathrm{id}$ is law with **compliance debt** (mean-pin falsified) | multiscale-methods §1; ARCHITECTURE §1 (corrected); multiscale-seams §1; DECISIONS mean-pin |
| 2 | **`#form-seam-flux-exchange`** | formulation / robust-qualitative | Seams exchange **fluxes not states**; one discipline on space × time axes | multiscale-seams mental model + §§1–2; ARCHITECTURE §4; framework-to-status-quo §3 |
| 3 | **`#disc-unlawfulness-budget`** | discussion / robust-qualitative | Every Realization carries unLawfulness $\varepsilon$; Realized ⟂ Lawful; BREAK-2 | vivium-operational-workflow Stage 1; ARCHITECTURE §6; LEXICON realized/lawful |
| 4 | *(already owned)* `#form-store-as-save` | — | Save ≡ memo store | DESIGN-REDUX §13; store.rs; LEXICON memo — **do not fork** |
| 5 | *(already owned)* `#norm-bias-vs-noise` | — | Bias vs noise is the decisive audit | discretisation §1; DECISIONS bias-vs-noise `:by us` — **do not fork** |

Draft segments written this wave: **1–3** under `core/src/`. Delete dual-home `core/src/form-save-is-memo-store.md` on integrate.

---

## EXTRACT

Candidates to promote into `core/src/` (slug proposals; not reserved until landed).

### Landed this wave (draft)

| Slug | Sources | Supersession / notes |
|------|---------|----------------------|
| `form-rl-closure-algebra` | multiscale-methods §§1–4; multiscale-seams §1; ARCHITECTURE §1 | Replaces dual-home algebra prose. **Does not** re-claim mean-pin as live $R\circ L=\mathrm{id}$. Fated-lifting closure points at `#post-determinism-as-ontology` rather than forking it. |
| `form-seam-flux-exchange` | multiscale-seams (mental model, §§1–2, §4 contract); ARCHITECTURE §4; framework-to-status-quo §3; abyssal-parity Phase 3 | Distinct from `#form-flux-web` (nomos consume/promise names vs scale/time boundary physics). $z$-reconciliation left out (conjecture). |
| `disc-unlawfulness-budget` | vivium-operational-workflow BREAK-2 + doctrine §3; ARCHITECTURE §6; LEXICON | Structural packaging of Joseph-settled Realized ⟂ Lawful; do not invent a DECISIONS slug Joseph did not write. |

### High confidence next (not drafted — owned elsewhere or needs more care)

| Slug proposal | Sources | Notes |
|---------------|---------|-------|
| *(done)* `form-store-as-save` | DESIGN-REDUX §13; ARCHITECTURE §5; store.rs | Design wave owns. |
| *(done)* `norm-bias-vs-noise` | discretisation §1; DECISIONS | Design/decisions wave owns. |
| *(done)* `form-fidelity-invariant` | LEXICON; multiscale-seams §3 observer-side | Design wave owns. |
| `form-depend-by-key-never-latest` | builder-explorer-decoupling §0 | Settled invariant; build-order independence sibling. OUTLINE §III gap family. |
| `form-store-is-the-bus` | builder-explorer §1; store-as-save | Builder owns scheduling never truth; explorers read-only peers. |
| `form-consumer-dependent-restriction` | multiscale-methods §3; multiscale-seams §5; NOMOS-CONTRACT | $R$ per consumer; statistic exactness on flux edges unbuilt. |
| `form-fated-lifting-closure` | multiscale-methods §3; multiscale-seams §2.2/§5 | Mostly already inside `#post-determinism-as-ontology` — extract **only** if multiscale-closure framing needs a separate home; prefer not to fork. |
| `obs-mean-pin-not-rl-id` | DECISIONS mean-pin; multiscale theory stale lines | Measurement observation segment; strengthens algebra segment's compliance debt. |
| `form-scale-separation-directional` | multiscale-seams §2.3 (Gear–Wells Prop 4.1) | Fast→slow Jacobian block; declare direction. |
| `sketch-dynamic-exponent-z` | multiscale-seams §3; abyssal-parity | **Conjecture** for $z$-reconciliation; keep sketch until Joseph closes. |
| `form-run-modes-carve` | abyssal-parity Phase 0; LEXICON run-modes | Joseph-decided convention-only + provisional banner. OUTLINE §III gap. |
| `form-builder-explorer-decoupling` | builder-explorer whole | Operational companion; large — maybe split bus / beacons / access profiles. |
| `disc-drainage-shaped-dependency` | framework-to-status-quo §3; multiscale-seams §3 | Degree = flux magnitude at edge. |
| `hyp-detail-abstract-rename` | multiscale-seams §2.4; ARCHITECTURE §8 corrected | Open research problem; **rename** nonlinear closure for non-local flux — do not keep stale "detail→abstract" as law. |
| `obs-water-gather-not-scatter` | water-parallelism | Parallelism precondition; engineering, may stay plan. |

### Medium / needs strengthen-before-soften

| Slug proposal | Sources | Why not drafted |
|---------------|---------|-----------------|
| `result-restriction-must-commute` | discretisation §7; research-structure-preserving | WAVETRISK/Balsara; linear-only; proposed DECISIONS status. |
| `form-unbalanced-haar-store` | discretisation §3.7 (retracted headline) | Spike falsified "seam never happens"; representation vs dynamics split. |
| `disc-column-fv-fd-semantics` | discretisation §2.4–2.4a; DESIGN-MATERIAL §4 | Overlaps `#form-column-control-volume` if design landed that — verify before extract. |
| `norm-declaration-was-unenforced` | discretisation §2.4a | Instance of `#norm-declaration-must-convict` — demote as specimen, don't re-claim. |
| `obs-theta-is-lax-friedrichs` | discretisation §2.5; DECISIONS | Water-kernel measurement; physics nomos home better than theory abstract. |
| `form-timestep-from-quadtree` | multiscale-seams §3 tactical; abyssal Phase 3 | Joseph 2026-07-10 steer endorsed — unbuilt; formulation only with honest status. |

---

## DEMOTE

Pointer / dual-home reductions performed or owed when a segment already owns the claim.

### Done this wave

| File | What | Now points to |
|------|------|---------------|
| `doc/theory/multiscale-methods.md` header + §1 | Algebra + conservative exchange dual-home; false mean-pin live claim | `#form-rl-closure-algebra`, `#form-seam-flux-exchange`; mean-pin corrected |
| `doc/theory/multiscale-seams.md` header + §2.1 injection | Seam law dual-home; mean-pin "enforces R∘L" | `#form-seam-flux-exchange`, `#form-rl-closure-algebra` |
| `doc/ARCHITECTURE.md` §1, §5, §6 | Algebra / save / unLawfulness | `#form-rl-closure-algebra`, `#form-store-as-save`, `#disc-unlawfulness-budget` |
| `doc/plan/vivium-operational-workflow.md` header | BREAK-2 as law | `#disc-unlawfulness-budget` |
| `doc/plan/regula-conformance-design.md` §3 | Flux unmet reasoning | `#form-flux-web`, `#form-ordinum-governs-flux-web` |
| `doc/theory/discretisation-and-information.md` §1 | Bias/noise dual-home | `#norm-bias-vs-noise` |
| `doc/design/DESIGN-REDUX.md` §13 | Already pointed | `#form-store-as-save` (design wave) |
| `doc/theory/discretisation-and-information.md` §0 | Already pointed | `#disc-prime-question` (pre-existing) |

### Still owed (high confidence; not all edited)

| File | Claim already owned by | Action |
|------|------------------------|--------|
| multiscale-methods §3 fated lifting long form | `#post-determinism-as-ontology` | One-line pointer; keep memoization-soundness teaching |
| multiscale-seams §4 seam contract items 2–4 | `#form-flux-web`, `#form-complete-content-addressed-key`, `#norm-probes-before-claims` | Pointer rows only |
| multiscale-seams §3 fidelity observer-side | `#form-fidelity-invariant` | Pointer |
| ARCHITECTURE §9 adding-a-system | split across def-nomos / flux-web / rl-closure | Pointer table when §9 fully segmented |
| abyssal-parity-plan milestone conformance list | core-view-wall, moratorium, seam segments | Checklist cites slugs |
| builder-explorer §3 access profiles / core-view | `#form-core-view-wall` | Pointer |
| framework-to-status-quo §1 status quo | ice as measured gap snapshot (plan, dated) | Keep as plan; no claim home |
| water-parallelism determinism/key | `#post-determinism-as-ontology`, `#form-complete-content-addressed-key` | Pointer header |
| discretisation §2.4a "declaration is a wish" | `#norm-declaration-must-convict` | Specimen pointer only |

### Delete on integrate

| Path | Why |
|------|-----|
| `core/src/form-save-is-memo-store.md` | Dual-home fork of `#form-store-as-save` from parallel mining; marker file only |

---

## ICE

Leave as plan, instrument, literature map, or true non-claim mass. Do not segment for its own sake.

| Material | Why ice |
|----------|---------|
| multiscale-methods §2 method zoo table | Teaching map; extract only when a method becomes a project decision |
| multiscale-seams §3 resolution light-cone / relativity analogy | Stance, carefully bounded; high misread risk as physics claim |
| multiscale-seams §6 abyssal-parity buy | Points at plan; redundant with abyssal-parity-plan |
| abyssal-parity-plan full 6-phase build sequence | **Plan**, not claim law; keep until phases complete |
| framework-to-status-quo §1 verified status-quo snapshot | Dated engineering gap report; freezes into false law if segmented |
| builder-explorer §5 initial slice ordering | Implementation queue |
| water-parallelism CPU/GPU staging | Engineering plan; gather-not-scatter may extract later as obs |
| regula-conformance §§1–2, 4–7 | Reasoning trail for regula/permit/slot vocabulary (`DECISIONS[regula-vocabulary-deferred]`); flux claims already owned |
| discretisation §§2.5–2.6, 3.x information dictionary, 4 structure table, 5–8 | Teaching + measurements + open grid leads; feed physics nomos / grid segments, not one mega-segment |
| discretisation MFD worked specimen under §0 | Teaching under `#disc-prime-question`; stay in theory file |
| Cardiff figures / primary long quotes | Literature; cite via BIBLIOGRAPHY |

---

## Proposed OUTLINE rows (not applied)

### §III Runtime, environment, and CLI

| Type | Aspect | Tag | Claim | Stage |
|------|--------|-----|-------|-------|
| formulation | Machine Critical | `#form-store-as-save` | Save file is the memo store | draft *(already in core/src)* |
| | | | *(#gap) Builder / explorer decoupling* | missing |
| | | | *(#gap) Run-modes carve* | missing |
| | | | *(#gap) Depend-by-key never latest / store-as-bus* | missing |

### §V Physical, mathematical, and algorithmic toolsets

| Type | Aspect | Tag | Claim | Stage |
|------|--------|-----|-------|-------|
| formulation | Model Critical | `#form-rl-closure-algebra` | Multiscale $R$/$L$/closure algebra; $R\circ L=\mathrm{id}$ with compliance debt | draft |
| formulation | Model Critical | `#form-seam-flux-exchange` | Seams exchange fluxes not states; one discipline two axes | draft |
| normative | Math Critical | `#norm-bias-vs-noise` | Bias vs noise is the decisive audit | draft *(already in core/src)* |
| formulation | Project Critical | `#form-fidelity-invariant` | Fidelity invariant both axes | draft *(already in core/src)* |
| | | | *(#gap) Grid / staggering* | missing |
| | | | *(#gap) Consumer-dependent restriction / sufficient statistic* | missing |
| | | | *(#gap) Hydrosphere, climate, erosion, water nomoi* | missing |

### §I or §II (ontology / kingdom)

| Type | Aspect | Tag | Claim | Stage |
|------|--------|-----|-------|-------|
| discussion | Project Critical | `#disc-unlawfulness-budget` | Every Realization carries unLawfulness budget; Realized ⟂ Lawful | draft |

Placement note: BREAK-2 is project ontology (near moratorium / vivium lifecycle), not multiscale math — prefer §I after moratorium or §II near vivium identity.

---

## Demotion / extract confidence notes

1. **Strengthen-before-soften on $R\circ L$:** law kept exact-as-aspiration; measured mean-pin failure recorded as compliance debt inside the segment (not a soften of the algebra).
2. **No invented Joseph decisions:** bias-vs-noise and store-as-save use existing DECISIONS/LEXICON; unLawfulness cites Joseph Realized ⟂ Lawful without minting a fake decision id for BREAK-2 packaging.
3. **Prime Question:** theory §0 already demoted to `#disc-prime-question` — no fork.
4. **Foreign claims:** none extracted; ASF checkpoint-forking only mentioned as `#asf/…` forward for BREAK-5 ice.

---

## Files touched this wave

**New segments**

- `/Users/josephwecker-v2/src/archema-io/vivarium/core/src/form-rl-closure-algebra.md`
- `/Users/josephwecker-v2/src/archema-io/vivarium/core/src/form-seam-flux-exchange.md`
- `/Users/josephwecker-v2/src/archema-io/vivarium/core/src/disc-unlawfulness-budget.md`
- `/Users/josephwecker-v2/src/archema-io/vivarium/core/src/form-save-is-memo-store.md` — **DELETE** (fork marker)

**Source demotions / pointer banners**

- `doc/theory/multiscale-methods.md`
- `doc/theory/multiscale-seams.md`
- `doc/theory/discretisation-and-information.md` (§1)
- `doc/ARCHITECTURE.md`
- `doc/plan/vivium-operational-workflow.md`
- `doc/plan/regula-conformance-design.md` (§3)
- `core/src/form-complete-content-addressed-key.md` (Working Notes)
- `core/src/post-represent-by-consequence.md` (Working Notes)

**No commit. No full OUTLINE rewrite.**
