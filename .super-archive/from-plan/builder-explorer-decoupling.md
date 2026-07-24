# Builder–explorer decoupling — the operational companion to the parity plan

> **Graduated 2026-07-24** to `.super-archive/from-plan/`. Build/reasoning trail only — claim homes in `core/`. Do not re-mine as claim source.


> **Claim homes (not this file).** Store as portable save / bus → `#form-store-as-save`. Builder refuse-unmet / waiver / explorer observe-only pull → `#form-builder-admission`. Core/view wall → `#form-core-view-wall`. Complete key → `#form-complete-content-addressed-key`. This file remains **operational design** for the unbuilt daemon, beacons, demand spool, and UX — not a second home for those laws.

*2026-07-10. Joseph's usage scenario (the "naive imagined scenario," given verbatim in-session) cleaned into an architecture, with the decisions it settled. This is the **operational/UX layer** over `doc/plan/abyssal-parity-plan.md` (which owns the build phases) and `doc/ARCHITECTURE.md` (which owns the frame). Status inline: **settled** (decided this session) / **design** (reasoned, adopt-unless-contradicted) / **open**. Vocabulary: LEXICON-canonical (beacon, focus, memo, vivium, Realized).*

---

## 0. The load-bearing guarantee first — build-order independence

**Claim home:** `#form-depend-by-key-never-latest` (build-order independence; depend by complete key only; fidelity dial in the planner, not the nomos). Elaboration below is source; do not treat this section as a second law home.

Two builds of the same spec, advanced along different demand orders, converge to byte-identical state wherever both have materialized — pure functions of complete keys. Neighbour dependencies are by key from the dependency map, never “finest/latest available.”

Native-representation memos (LEXICON §2) fit unchanged: keys name **artifacts** (nomos outputs — a drainage-graph object, a slab-set), not grid cells; the canonical frame is how *queries* address the world, not how *dependencies* must be shaped. The one tension to manage is **invalidation granularity** — a large native object invalidates all its dependents at its own grain (over-keying: safe, possibly wasteful) — so a native system chooses its memo grain to match its consumers' cones (per-basin, not one global graph).

## 1. The four pieces — and the store is the bus *(settled)*

No mediator daemon: **the store itself is the coordination medium.** Immutable content-addressed objects + atomic root swaps mean any number of readers are safe against a writer with zero protocol. (Two processes computing the same key write identical bytes — a benign race by construction.)

1. **The vivium spec** (`spec.rs` manifest — landed, increment #5). Three field buckets, and the carve is load-bearing:
   - **identity** — in every key; change = fork a new world (seed today; pinned law/generator versions at Realization later). Grounding: `doc/plan/vivium-operational-workflow.md` Stage 0 (`hash(seed, T-version, generator-versions)`), LEXICON §4/§7.2.
   - **label** — never keyed; rename freely (name).
   - **demand** — target phase, **beacons**, watchpoints: change build *order*, provably never the world (§0). Editable mid-build, always safe.

2. **The builder daemon** *(design)* — owns *scheduling*, never truth. Walks the demand frontier (beacons → causal cones → work queue), computes memos, appends a log, maintains `status.json`, emits watchpoint snapshots. Lockfile in the world dir; a second `build` invocation on the same spec **attaches** (tails status + log) rather than failing — Joseph's preferred behavior. Pause/stop/kill are all cheap: every completed tile is already in the store; at most one in-flight tile is lost. Nix-style iteration: edit the spec → keys change → the builder offers *restart-in-place* (recompute exactly the invalidated cone) or *fork* (new spec; forks may share one object pool — the seed is in every key, so worlds cannot alias — giving cross-fork dedup for free).

3. **Explorers** *(design)* — read-only peers, any number, fully process-decoupled. Query through the store; on a miss, render the coarsest instantly-computable thing (the prior is a cheap pure function) and drop a **demand file** into a spool directory the builder sweeps. Consequences: explorers work with *no* builder running (coarse world); inform priorities when one is; can *offer* to start one ("demand exists, no builder"); never block or get blocked. Live watching needs no IPC: poll roots / watch mtimes; when time-indexed stages exist (component E), *the animation is the memo sequence* — floating downstream while erosion happens is playing back new time-indices as they land.

4. **Instruments** *(design)* — standing queries over the root census:
   - **fidelity pyramid** — enumerate `roots`, histogram by (level × stage): Joseph's half-population-pyramid, nearly free.
   - **watchpoints** — a declared `(place, level, stage)` that emits an ASCII/image snapshot + stats when the builder reaches it (the `store_explore` renderer, wired to events). *Named against the collision ledger: not "breakpoint," and "checkpoint" is ceded to ASF.*
   - **telemetry answer** (Joseph's dangling question): any part of the world at any stage is monitorable by construction — every intermediate is an immutable object; readers never contend with the writer; the cost is disk reads, not coordination. Precondition for *stage*-addressability: component E (see TODO).

## 2. Scheduling — beacons and the demand frontier *(design)*

The builder's priority is the **unmet frontier of the demanded cones**: least-advanced work items inside the cones beacons demand, computed backward through the dependency map (the spine's drainage islands, plan-Phase-2b). Two per-beacon policies, both visible in the pyramid:

- **depth-first** — push one region to its target fidelity first (an explorer waiting at a beacon);
- **breadth-first** — level the whole cone coarsely first (the globe-spinner: "all of the world to the end of Phase 2").

Hilbert order (`CellId`) makes any region a contiguous id range — *storage/streaming locality* around each beacon. The *dependency* structure is drainage-shaped and does not follow Hilbert adjacency; two different structures, both real, neither substitutes for the other.

## 3. Access profiles = process boundaries *(design; answers taxonomy Sc-9)*

The BDD stress-test's sharpest gap — exo-by-governance splits into *access-rich* (source-holder) vs *plain participant* — is naturally **enforced** by this architecture: a builder process holds nomos + write-capable store access; an explorer process holds a phenomenal-grade query API. The OS process boundary mechanically instantiates the access-profile split the ontology wants. Profiles attach at the **query front-end** (plan-Phase-4), not in the store. Note for the run-modes carve: the moment two processes share a store, the Phase-0 "convention-only" canon-root guard is worth revisiting (the tripwire may fire earlier than "first graduation").

## 4. The temporal ladder *(design; recovered from the context-exhausted 2026-07-10 session)*

The staged physics progression the builder's stage chains implement, each rung a keyed nomos composing on the one below:

1. **Macro-erosion epochs** — coarse geological erosion (today's `erosion_tile`).
2. **Macro water-cycle + alluvial coupling** — water at aggregated scale, sediment fluxed back into erosion; slow erosion reads water as *time-averaged discharge*, water sees terrain as quasi-static (`.super-archive/from-theory/multiscale-seams.md` §2.3). The rung with the real open nuance.
3. **Human-scale live water, full precision, ~8× real-time** — the conserved shallow-water sim, *entered warm*: its settled fill is memoized (eventually the analytic year-zero morning), so entering a tile loads settled water instead of re-filling. Memoizing the fill is simultaneously what kills the old spike's ~2 h re-fill and what frees the budget for 8×.

## 5. The initial slice *(settled order; increments = commits)*

1. ~~**Spec + seed**~~ — **landed** (increment #5): manifest, seed in every key, `World` context so key-seed and compute-seed cannot diverge.
2. **`vivarium` CLI** — `new` / `build` (fg/bg) / `status` / `attach`; builder v0 sweeps a beacon cone at spine+erosion rungs; lockfile-attach; fidelity pyramid.
3. **Demand spool + read-only query path** — the explorer-side API (hit | coarse-fallback + demand file).
4. **Time-indexed erosion stages** — component E minimal: epochs in the key → "watch it age" + stage-addressable beacons.
5. **First-person ethereal explorer** — thin Bevy crate, queries only (core/view wall; moratorium-clear): column-center point mesh, walk, file demand, upgrade as memos land. *(The globe viewer — breadth-first over the spine — is the same machinery with a different camera, and is being built first as plan-Phase-2's visible win.)*

## 6. Open

- **Demand-spool format + sweep cadence** (trivial now, decide in code).
- **When the canon-root guard goes mechanism-enforced** (two-process store sharing is the natural trigger — §3).
- **Fork UX** — naming/browsing the fork-DAG (BREAK-5's world-identity question wears a save-menu costume long before endo agents exist).
- **Multi-machine** — content-addressing gives git-fetch-shaped sync eventually; out of scope until a second machine exists.
