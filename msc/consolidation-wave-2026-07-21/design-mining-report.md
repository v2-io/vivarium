# Design-slice mining report — 2026-07-21 (finished)

**Slice:** `doc/design/` — DESIGN.md · DESIGN-REDUX.md · DESIGN-MATERIAL.md · DESIGN-SYSTEMS.md · NOMOS-CONTRACT.md  
**Claim floor:** `core/OUTLINE.md` + `core/src/*`  
**Not canon.** Pacing only; core wins on conflict. **No commit.**

---

## Completed vs siblings already owned

| Claim | Owner | Design-slice action |
|-------|--------|---------------------|
| `#form-fidelity-invariant` | **This slice** | Drafted + design demotions |
| `#form-column-control-volume` | **This slice** | Drafted + MATERIAL demotions |
| `#form-store-as-save` | Contested mid-wave; **restored as sole home** | Full FE restored; DESIGN-REDUX §13 → this slug; `form-save-is-memo-store` deletion stub |
| `#form-nomotheke-registry` | decisions-code **msc draft only** | **Promoted** to `core/src/` |
| `#form-builder-admission` | decisions-code **msc draft only** | **Promoted** to `core/src/` (depends on store + nomotheke) |
| `#norm-bias-vs-noise` | **Archive slice** | Demoted NOMOS-CONTRACT; do not re-extract |
| `#form-rl-closure-algebra` | **Theory-plan** | Leave |
| `#form-seam-flux-exchange` | **Theory-plan** | Leave (REDUX §5 residual may join later, not forked) |
| `#disc-unlawfulness-budget` | **Theory-plan** | Leave |
| `form-store-is-save` (msc) | decisions-code naming fork | Superseded marker → `#form-store-as-save` |
| `form-save-is-memo-store` | mid-wave dual-home | **DELETE** stub remains for orchestrator `rm` |

---

## Path list

| Path | Role |
|------|------|
| `msc/consolidation-wave-2026-07-21/design-mining-report.md` | This report |
| `core/src/form-fidelity-invariant.md` | **Drafted** (design) |
| `core/src/form-column-control-volume.md` | **Drafted** (design) |
| `core/src/form-store-as-save.md` | **Restored sole store claim** (merged design + decisions-code + theory content) |
| `core/src/form-nomotheke-registry.md` | **Promoted** from msc draft |
| `core/src/form-builder-admission.md` | **Promoted** from msc draft |
| `core/src/form-save-is-memo-store.md` | **DELETE on integrate** (stub only) |
| `doc/design/DESIGN.md` | Demote pointers |
| `doc/design/DESIGN-REDUX.md` | Demote pointers (§0, §1, §2b, §8, §13 → store-as-save) |
| `doc/design/DESIGN-MATERIAL.md` | Demote pointers (§1, §3) |
| `doc/design/DESIGN-SYSTEMS.md` | Source/inventory banner |
| `doc/design/NOMOS-CONTRACT.md` | Boxes ①⑤ + convict + bias-vs-noise pointers |
| `msc/.../draft-form-*.md` | Marked promoted/superseded |

---

## Top 5 extract priorities (remaining)

1. **`#form-lazy-query-graph`** — demand-driven memoized query graph (DESIGN-REDUX §11)
2. **`#form-multirate-coupling`** — multirate aspect coupling (REDUX §4 · DESIGN-SYSTEMS bands)
3. **`#form-sufficient-statistic-seam`** — consumer-dependent restriction (REDUX §5; join carefully with `#form-seam-flux-exchange` / column)
4. **`#form-nomos-declaration-boxes`** — NOMOS-CONTRACT ② geometry · ③ semantics · ④ structure (+ NomosDecl fields first)
5. **`#norm-regime-probes`** — domain TDD / probe-first per rung (REDUX §2b); specializes `#norm-probes-before-claims`

---

## EXTRACT

| Priority | Proposed slug | Type | One-sentence claim | Source | Notes |
|----------|---------------|------|--------------------|--------|-------|
| ✓ | `#form-fidelity-invariant` | formulation | Lazy materialization; statistical consistency with bounded deficiencies; co-fidelity | DESIGN multi-fidelity · REDUX §1 | Drafted |
| ✓ | `#form-column-control-volume` | formulation | Column = control volume + sufficient statistic + declared exactness | MATERIAL §§2–5 · DECISIONS column (`:by us` frame) | Drafted |
| ✓ | `#form-store-as-save` | formulation | Memo store is the portable save | REDUX §13 · ARCHITECTURE §5 · store.rs | Sole home restored |
| ✓ | `#form-nomotheke-registry` | formulation | Registry gates world-law; declaration is key stem; live tests | nomotheke.rs · decisions-code draft | Promoted |
| ✓ | `#form-builder-admission` | formulation | Builder refuses unmet by default; explorers observe-only pull | vivarium.rs · query · plan | Promoted |
| 1 | `#form-lazy-query-graph` | formulation | Runtime is a demand-driven memoized query graph | REDUX §11 | Remaining |
| 2 | `#form-multirate-coupling` | formulation | Fast sees slow as quasi-static; slow sees fast as averages | REDUX §4 · SYSTEMS | Remaining |
| 3 | `#form-sufficient-statistic-seam` | formulation | Seam is sufficient statistic, not one number | REDUX §5 · MATERIAL §4 | Join theory seam, don't fork |
| 4 | `#form-nomos-declaration-boxes` | proposed-schema | Geometry / semantics / structure boxes on nomos | NOMOS-CONTRACT | ①⑤ owned elsewhere |
| 5 | `#norm-regime-probes` | normative | Every rung ships regime probes; issues get probe first | REDUX §2b · SYSTEMS | Remaining |
| 6 | `#form-fidelity-ladder` | formulation | Climb emergent, descend to probe-validated surrogate | REDUX §12 non-key | Remaining |
| 7 | `#form-cellid-chunk-patch` | formulation | CellId keys chunks; Cartesian interiors + halos | MATERIAL §8 | Settled in code |
| 8 | `#disc-two-layer-mind` | discussion | Fast AAT authoritative; LLM only at aporia | DESIGN.md · ETHICS | Level C adjacent |
| 9 | `#form-logozoetic-peer-view` | formulation | Typed A/O API is peer view from day one | DESIGN.md | View wall specialization |
| 10 | `#form-reversion-cache-eviction` | sketch | Reversion = cache eviction; regenerable vs irreducible | REDUX §6 | Open frontier half |

**Do not invent:** grid/stagger, BLAKE3, GC algorithms, NomosDecl fields for ②–④ as built.

---

## DEMOTE (high-confidence applied ✓)

| Source | Claim home | Action |
|--------|------------|--------|
| ✓ DESIGN.md Purpose | `#disc-vivarium-purpose` | Already pointer |
| ✓ DESIGN.md Interface | `#form-core-view-wall` | Already pointer |
| ✓ DESIGN.md Determinism | `#post-determinism-as-ontology` | Already pointer |
| ✓ DESIGN.md Multi-fidelity | `#form-fidelity-invariant` | Claim-home banner |
| ✓ DESIGN-REDUX §0 | `#post-represent-by-consequence` | Demoted |
| ✓ DESIGN-REDUX §1 | `#form-fidelity-invariant` | Demoted |
| ✓ DESIGN-REDUX §2b | `#norm-probes-before-claims` (+ residual extract) | Pointer |
| ✓ DESIGN-REDUX §8 | `#post-determinism-as-ontology` | Demoted |
| ✓ DESIGN-REDUX §12 key | `#form-complete-content-addressed-key` | Already mid-section |
| ✓ DESIGN-REDUX §13 | `#form-store-as-save` | Demoted (slug fixed this pass) |
| ✓ DESIGN-MATERIAL §1, §3 | column + fidelity + represent-by-consequence | Demoted |
| ✓ NOMOS-CONTRACT header | flux · prime · convict · nomos | Banner |
| ✓ NOMOS-CONTRACT bias | `#norm-bias-vs-noise` | Pointer (archive slice) |
| ✓ DESIGN-SYSTEMS | inventory only | Source banner |

---

## ICE

| Item | Why |
|------|-----|
| Bevy engine decision + Godot spike | Historical / `spikes/FINDINGS.md` |
| REDUX §9 posits, §10 bibliography, prefetch | Undecided / literature / opt |
| MATERIAL §9 concrete Rust schema | TENTATIVE |
| SYSTEMS status columns, fluvial checklists, approach ledger | Judgment inventory / procedure |
| NOMOS-CONTRACT unlaunched five-agent brief | Process intent |
| msc `draft-form-store-is-save` as a *third slug* | Naming fork — content merged |

---

## Drafted / promoted segments (design integration)

### Own drafts
1. **`#form-fidelity-invariant`** — formulation · exact · draft  
2. **`#form-column-control-volume`** — formulation · exact · draft (frame only; impl open)

### Store (sole home after collision cleanup)
3. **`#form-store-as-save`** — formulation · exact · draft — restored full FE (directory shape, domain-neutral bus, Hit-as-state, invalidation/eviction, pervasive memo, regenerable/irreducible frame, run-mode root frame)

### Promoted from decisions-code msc drafts
4. **`#form-nomotheke-registry`** — OUTLINE §II gap  
5. **`#form-builder-admission`** — OUTLINE §III gap  

**Orchestrator cleanup:** `rm core/src/form-save-is-memo-store.md`

---

## Proposed OUTLINE rows (append only)

### §I (near represent-by-consequence)
| Type | Aspect | Tag | Claim | Stage |
|------|--------|-----|-------|-------|
| formulation | Project Critical | [#form-fidelity-invariant](src/form-fidelity-invariant.md) | Fidelity invariant + co-fidelity | draft |

### §II
| Type | Aspect | Tag | Claim | Stage |
|------|--------|-----|-------|-------|
| formulation | Model Critical | [#form-column-control-volume](src/form-column-control-volume.md) | Column is a control volume with sufficient statistics | draft |
| formulation | Machine Critical | [#form-nomotheke-registry](src/form-nomotheke-registry.md) | Nomotheke is the registry contract | draft |

*(Replace gap row "Nomotheke as registry contract".)*

### §III
| Type | Aspect | Tag | Claim | Stage |
|------|--------|-----|-------|-------|
| formulation | Machine Critical | [#form-store-as-save](src/form-store-as-save.md) | Store is the save | draft |
| formulation | Machine Critical | [#form-builder-admission](src/form-builder-admission.md) | Builder admission + explorer observe-only pull | draft |

*(Replace gaps "Store as save file" and partially "Builder / explorer decoupling" — full daemon remains gap.)*

Remaining §III gap: run-modes carve. Remaining §II gaps: phase content, manifest.

---

## Method

- OUTLINE + core skimmed; DECISIONS column frame and authority tags checked; no invented Joseph decisions.  
- Parallel-wave store triple-slug collapsed to `#form-store-as-save` (matches ARCHITECTURE, theory-plan, disc-unlawfulness, complete-key notes).  
- Present tense; stage `draft`; strengthen-before-soften on compliance debt vs law.

*End design-slice report (finished).*
