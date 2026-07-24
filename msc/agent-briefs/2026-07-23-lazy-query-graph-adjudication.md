# Adjudication: lazy / demand-driven memoized query graph

*Peer report for parent session. Not claim canon. 2026-07-23.*

> **Parent re-weight (Joseph 2026-07-23):** Live code is the **least** principled surface this phase. Thin promote stands as **architecture law** from design/segments; `query.rs` is **miniature / compliance debt**, not the reason the law is true. Do not read “not a generic engine in code” as weakening FE(1)–(4).

**Sources verified this pass:** DESIGN-REDUX §11 (+ §3/§12 touchpoints); ARCHITECTURE §5 (+ §2 HMM row, §8 gaps, spine invariants); `query.rs` / `store.rs` (read + method walk); segments `#form-store-as-save`, `#form-complete-content-addressed-key`, `#form-depend-by-key-never-latest`, `#form-builder-admission`, `#form-fidelity-invariant`, `#form-nomotheke-registry`, `#def-nomos`, `#post-determinism-as-ontology`, `#post-represent-by-consequence`; OUTLINE §III; candidates inventory notes; promotion-mine residual #1; builder-explorer / abyssal-parity / framework-to-status-quo plan headers; DECISIONS leaf-only-evolution tension (flux-on-face vs independent memo tiers).

---

## 1. Verdict

**Promote thinner claim — not the inventory’s thick Critical formulation.**

| Inventory guess | Independent judgment |
|---|---|
| `#lazy-memoized-query-graph` / `#form-lazy-query-graph` as Machine Critical: “query = (aspect, region, resolution, time); nothing computes until demanded, nothing twice” as a single settled runtime law | **Over-bundled.** That sentence packages (a) a thin evaluation-strategy law that is still true and still unowned, (b) a designed-not-built three-scoped architecture, (c) a generic Salsa-class engine that Architecture itself marks as **gap**, and (d) open research (edit up-invalidation). Promoting the bundle as one exact Critical segment would rubber-stamp design prose. |
| “Already owned?” | **Partially.** Persistence, key completeness, demand≠ontology, observe-only explorer, lazy *fidelity* materialization, and memo soundness under fated keys **are** owned. The **composition strategy** (world law advances by pull of keyed nomos evaluations; deps = recursive pulls) is **not** owned as a single FE surface — and several segments explicitly leave “lazy query-graph runtime shape” as residual sibling. |
| “Do not promote yet / stay source-only until built?” | Wrong for the **thin** law (present practice + architecture doctrine). Right for the **thick** pieces (generic engine, coarse global spine as dependency planner, predictive prefetch, edit-propagation layer, full time-DAG mechanization). |

**Recommended action for parent session:**

1. **Draft a thin segment** (slug suggestion below) under OUTLINE §III Runtime — *if* energy is spent now.
2. **Do not** promote inventory `#runtime-three-scoped-pieces` or `#time-in-every-key-dag` as part of the same FE.
3. **Do not** treat Architecture’s “established paradigm” label as evidence the *engine* is built — the same paragraph says the engine is a gap.
4. Acceptable alternative if session bandwidth is tight: leave residual **dual-home demote only after** the thin segment exists; until then DESIGN-REDUX §11 / ARCHITECTURE §5 remain intentional source, not silent law.

---

## 2. What is actually true vs aspirational vs false

### 2.1 True and already owned (do not re-home under a new slug)

| Claim piece | Home | Evidence |
|---|---|---|
| Demand-shaped work (compute where a consumer depends) | `#post-represent-by-consequence` | FE(1)–(2); Architecture Layer 0 |
| Fine detail exists when demanded; unpulled may stay coarse | `#form-fidelity-invariant` FE(2) | Explicit “Lazy materialization”; Working Notes name query-graph as *sibling*, not content |
| Memo addressed by everything that affects value; over-key | `#form-complete-content-addressed-key` | Live `Key` + `with_dep_versions`; store module docs |
| Save ≡ memo store; hit = matured state; pervasive memo default | `#form-store-as-save` | `store.rs` objects/roots; Joseph 2026-07-09 directive carried here |
| Depend by complete key, never “finest available”; build-order independence | `#form-depend-by-key-never-latest` | builder-explorer §0 law; water fixed-step discipline in `query.rs` comments |
| Builder admits / materializes; explorer observe-only pull | `#form-builder-admission` | `surface_prefer_eroded`; requisite refuse path |
| Same key ⇒ same bytes; memo changes cost not world | `#post-determinism-as-ontology` FE(3) | fated noise + pure keyed compute |
| Nomos = memoizable invocation unit; registry gates undeclared law | `#def-nomos`, `#form-nomotheke-registry` | nomotheke tests; `NomosDecl::key` |

If a draft only restates the table above, it is **not** a new claim — it is a consolidating essay and should not ship as segment law.

### 2.2 True, residual, still unowned (the promote-worthy core)

**Evaluation / composition strategy:**

1. **World materialization advances by pull evaluation of declared nomoi under complete keys**, not by a global dense finest-tick and not by explorers inventing evolution on the view path.
2. **Inter-nomos composition is recursion in the pull** (A’s miss pulls B’s keys, memoizes B, then A) — not shared mutable world state between systems.
3. **Present practice implements the pattern hand-written per nomos** on `World` (`query.rs`): hit → return; miss → pull deps → compute → put. Documented recursion: `erosion_tile` → `initial_topography` + `uplift_tile` + `climate_tile`; `water_tile` → `erosion_tile` + `climate_tile`; hydrosphere as box through the same store path (representation-agnostic bus).
4. **The generic declarative pull engine is not built.** `#form-nomotheke-registry` Epistemic Status (5) and Architecture §5 both say so. Code path is methods, not a Salsa/Adapton graph interpreter.

This residual is why dual-home prose keeps being cited as law: OUTLINE §III has store / admission / depend-by-key but **no row for “how the runtime composes evaluation.”**

### 2.3 Aspirational design (true as *design*, false as *shipped law*)

| Piece | Source | Status |
|---|---|---|
| Query identity as full `(aspect, region, resolution, time)` for every memo | REDUX §11, ARCH §5 | **Partial.** Region + resolution + nomos identity live in keys; **time** is ad hoc (`epochs` / `steps` / `eepochs` on some nomoi; hydrosphere and initial-topography have no sim-time coordinate). Not a mechanized DAG over timesteps. |
| Time cuts coupling cycles so the graph is a DAG | REDUX §11 seam 2 | **Design invariant, unbuilt at generic tier.** Live coupling is still shallow hand-written deps, not multirate lagged A(t)←B(t−Δ). |
| Three-scoped degradation: precomputed coarse-global spine + lazy local cones + edit-propagation layer | REDUX §11; ARCH §5; abyssal-parity Phase 2 | **Designed, not built.** Plans still list coarse global spine as a build phase. Edit-propagation is open research (detail→abstract / nonlinear non-local flux closure — ARCH §8 corrected). |
| Predictive prefetch | REDUX §11 | Explicitly “build last / pure optimization.” |
| “HMM made lazy; pull backwards-from-now” | ARCH §2 | Teaching frame / research lineage — not a measured method identity for current code. |
| Full builder demand spool / beacons | builder-explorer plan; OUTLINE gap | Design; `#form-builder-admission` correctly refuses to claim it. |

### 2.4 False, stale, or overclaimed if asserted as present truth

| Assertion | Problem |
|---|---|
| “The whole runtime **is** a demand-driven memoized query graph” (as if engine complete) | Architecture’s own subtitle: *established paradigm; **the engine is a gap***. Code is the paradigm **in miniature**, not the general machine. |
| Inventory Critical tag implying non-negotiable shipped spine | Spine walls already named in ARCH §8: core/view, determinism, complete key — **not** “query graph engine.” |
| “Nothing computes until demanded” without role split | **Builder** demand is lawful materialization; **explorer** must not cold-trigger long evolution (`#form-builder-admission` FE(4)). Absolute “nothing until demanded” without that split reopens the core/view hole. |
| Independent memoized coarse tiers as free architecture | DECISIONS tension: flux-on-the-face as identity can force **leaf-only evolution**, which **cuts against** independent coarse-tier memoization. Unresolved. Any segment that freezes “independent multi-tier evolution is the runtime” is premature. |
| “Nothing twice” as absolute | True under complete keys for pure generative pulls; incomplete under mutation / up-invalidation (frontier). Also true only if keys are complete — under-keying makes “twice” into silent wrong-once. |

### 2.5 What live code convicts

**Supports the thin claim:**

- Coordinate-addressed keys; store get/put; `Source::{Computed,Hit}`.
- Recursive dep pulls inside `erosion_tile` / `water_tile`.
- Module header still points at DESIGN-REDUX §11 (dual-home), while claiming homes for store / admission / complete-key only — residual gap matches the promotion-mine entry.
- Tests exercise hit-on-revisit and dep-hit chaining (erosion then water).

**Does not support the thick claim:**

- No generic query graph / dependency planner object.
- No coarse-global spine memo as world dependency map.
- No time-indexed multirate DAG.
- No invalidation graph for agent edits (mutations/ reserved in store claim only).
- `surface_prefer_eroded` deliberately **refuses** pure lazy “pull finest always” on the view path.

---

## 3. If promote: proposed surface

### 3.1 Slug and placement

- **Preferred slug:** `#form-pull-query-composition`  
  (Names the *composition law*. Avoids implying a built graph engine.)
- **Acceptable alias if parent wants continuity with inventory:** `#form-lazy-query-graph` — only if FE body is the thin law and Epistemic Status forbids the thick package.
- **OUTLINE:** §III Runtime, Machine Critical (formulation), after `#form-depend-by-key-never-latest` or between store and admission.
- **status / stage:** `exact` / `draft` — exact as *evaluation-strategy architecture law* under complete keys + determinism; draft because hostile audit of wording has not run.
- **depends (suggested):**
  - `post-determinism-as-ontology`
  - `form-complete-content-addressed-key`
  - `form-store-as-save`
  - `def-nomos`
  - `form-nomotheke-registry`
  - `form-depend-by-key-never-latest`
  - `form-builder-admission` (role split for who may demand compute)
  - optional soft cite (not necessarily depends): `form-fidelity-invariant` for lazy *resolution*, orthogonal to pull *composition*

### 3.2 FE bullets I would defend under hostile read

1. **Pull composition.** A world-law value is obtained by evaluating a declared nomos at a complete content-addressed key: store Hit returns matured bytes; Miss computes under pure keyed inputs, may **pull** other complete keys for dependencies, then Puts. There is no second channel of world truth outside that bus.
2. **Dependencies are recursive pulls.** Nomoi couple by reading other nomoi’ memoized outputs (or flux objects named by key), not by sharing mutable runtime state. Composition depth is the dep cone of the demanded key.
3. **Demand schedules; keys define.** *Which* keys are scheduled is demand (builder beacons, explorer spool, phase target). *What* a key’s bytes are is fixed by the complete key graph (`#form-depend-by-key-never-latest`). Conflating schedule with ontology is the failure mode.
4. **Role split on who may compute.** Builder (and other admitted materializers) may Miss→compute under admission rules. Explorers / views use observe-only pull: Hit preferred; lawful coarse/instant prior on Miss; no cold long-evolution on the view path (`#form-builder-admission`).
5. **Pattern vs engine.** The law is the evaluation pattern. Present implementation is **hand-written per-nomos methods** on a store-backed `World`. A generic incremental engine (Salsa/Adapton lineage) is a permitted future mechanization, not a present claim and not required for the law to hold.
6. **Out of bounds for this segment.** (a) Three-scoped runtime decomposition as shipped architecture; (b) predictive prefetch; (c) “time is in every key” as fully mechanized; (d) edit up-invalidation / detail→abstract; (e) independent coarse-tier evolution vs leaf-only flux identity (open); (f) restating complete-key or save≡store.

### 3.3 Epistemic Status notes for the draft

- **Max attainable: exact** for FE(1)–(4) as architecture under determinism + complete keys + registry.
- **Present practice:** `query.rs` miniature; probes/tests on hit/revisit; not a generic planner.
- **Compliance / build debt (do not soften the thin law into “wish”):** generic engine; demand spool; time-as-first-class on all nomoi; under-keying bugs still possible.
- **Open tension (name, do not resolve here):** leaf-only evolution price vs independent memoized tiers (DECISIONS flux-on-face thread).

---

## 4. What must NOT be claimed

1. That a **generic lazy query-graph engine is built**.
2. That the **three-scoped graceful-degradation architecture is operational** (spine + local cones + edit layer).
3. That **query = (aspect, region, resolution, time)** is the complete present key schema for every nomos.
4. That **mutation / agent edit up-invalidation** is solved by “the query graph.”
5. That **independent multi-tier evolution** is settled (conflicts with unpaid leaf-only price).
6. That **“nothing computes until demanded”** applies to explorers the same way as builders.
7. That this is a **fourth spine wall** beside core/view, determinism, complete-key — Architecture does not elevate it that high; thin composition law is enough without inflation.
8. That **HMM / Salsa / Nix** are claimed method identities for the live code — lineage and teaching only.
9. Inventory sibling `#pervasive-disk-memoization` as a separate Critical normative if promoted again — already carried inside `#form-store-as-save` FE(6) + complete-key “key never caution.” Do not fork.

---

## 5. Dual-home demote candidates (only after thin segment lands)

| Source | Action after segment |
|---|---|
| DESIGN-REDUX §11 opening paragraph (demand-driven memoized query graph definition) | One-line claim home → thin slug; leave four seams / prefetch / three-scoped *as source design* |
| ARCHITECTURE §5 first sentences (runtime as query graph) | Claim home for thin composition law; keep “engine is a gap” honesty; complete-key / store sentences already pointed at their homes |
| ARCHITECTURE §2 HMM row “§5 lazy query graph” | Soften to “teaching lineage → thin slug”; do not claim method equivalence |
| `query.rs` module docs lines 1–2 | Point claim home to thin slug; stop treating REDUX §11 as law |
| promotion-mine residual #1 / candidates `#lazy-memoized-query-graph` | Mark promoted-thin or retired-as-bundle; split residual three-scoped / time-DAG rows as **still source / gap** |
| `#post-represent-by-consequence` Working Notes “lazy memo graph needs segment” | Cross-link when landed |
| `#form-fidelity-invariant` / `#form-complete-content-addressed-key` Working Notes “sibling runtime shape” | Same |

**Do not demote** REDUX §11 seams 1–4 teaching, three-scoped list, or prefetch into the thin segment — demoting those would smuggle aspirational architecture into exact FE.

---

## 6. Feedback on this brief / incidental findings

### On the brief

- The instruction to **verify over inventory shape** was load-bearing. The inventory line is a reasonable *extraction candidate* and a bad *verbatim FE*. Parent was right that the inventory is a guess from shape.
- Explicit carve-outs in existing segment Working Notes (`form-complete-content-addressed-key`, `form-fidelity-invariant`, `post-represent-by-consequence`) made residual ownership easy to check — good prior discipline.
- “Authority is not evidence” applied cleanly to Architecture’s “established paradigm” and plan “settled” language: both coexist with “engine is a gap” / “designed not built.”

### Incidental findings (for parent triage, not expanded here)

1. **OUTLINE §III gap list** is incomplete relative to residual runtime shape: it has run-modes and builder daemon, but not evaluation-composition. Thin promote fills a real hole; thick promote would fake completeness.
2. **Leaf-only evolution vs memoized independent tiers** is a higher-leverage open problem than re-prosing §11. Any future “three-scoped pieces” segment must confront DECISIONS’ unpaid price before claiming independent coarse evolution.
3. **`#runtime-three-scoped-pieces` and `#time-in-every-key-dag`** should stay **source-only or OUTLINE gaps**, not ride this promotion. They are separate truth-status objects.
4. **Pervasive disk memoization** is already dual-homed into store + complete-key; candidates inventory still lists it `missing` — inventory row is **stale relative to core**.
5. **query.rs still dual-homes** REDUX §11 in the module banner while correctly pointing store/admission/key segments — small fix when thin segment lands.
6. ARCH §8 “status-quo gaps” still lists “store + nomos layer (§5)” as unbuilt in spirit; store/nomotheke/query *exist* in MVP form. That paragraph is **stale relative to code** and should not be cited as current inventory of gaps without re-reading crates.

### Confidence

- High on “do not promote the thick inventory claim as exact Critical.”
- High on “thin composition residual exists and is unowned.”
- Medium on slug naming (`pull-query-composition` vs keep `lazy-query-graph`) — parent aesthetic / LEXICON fit; law content is the thin FE set either way.
- Medium on “promote now vs next after vertical probes” — truth supports either sequencing; I do not treat checklist completion as a reason to draft today.

---

*End of report. Available for short follow-up (slug choice, draft FE polish, or dual-home edit plan).*
