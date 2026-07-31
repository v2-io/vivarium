---
slug: disc-explorer-instrument-parity
type: discussion
status: discussion-grade
stage: draft
depends:
  - form-store-as-save
  - form-core-view-wall
  - norm-no-depiction-without-referent
  - form-fidelity-ladder
  - form-column-control-volume
  - disc-known-active-hotspots
  - ops-changelog-is-the-acceptance-check
---

# Explorer instrument parity — when eyes can convict the bed

World-law fidelity and explorer fidelity are **not** the same claim, but they are **not** allowed different honesty standards. A multi-second “updating view…” that re-censuses the save, or an open globe that is 100% uncarved prior while the store is full of L9 carve, is not “UI lag.” It is an **instrument lie**: it makes physics spot-check impossible by mixing **rendering craziness** with **world craziness** until neither can convict the other.

This segment seals a **minimal parity bar** and the **short milestone ladder** that reaches it. It does not plan a full quadtree, a game camera, or Abyssal completion.

## Formal Expression

1. **What parity means here.** *Instrument parity* (for the present phase of work) is the condition under which Joseph (or any agent) can **spot-check physics** in `vivarium explore` without first solving a second research problem about the viewer. Concretely: when something looks wrong on screen, the default hypothesis is allowed to be **the bed / the law**, not “the pull thrashed again.”

2. **What parity is not.** Not full multi-resolution LOD. Not participant/pawn affordances. Not parity of *feature count* with the builder. Not “the explorer is as rigorous as a paper.” The bar is **usable honesty under a zoomable globe**, not cinema.

3. **Joseph 2026-07-31 acceptance (sealed).** Content with a **zoomable (to a limit) globe that is performant**. Some **LoD heterogeneity** is expected *unless* wholesale level switches become so cheap that a single mesh grain is fine. At current thrash, physics spot-check is impossible.

4. **The architectural premise that was already true.** Memoization is the store (`#form-store-as-save`): keyed citizens, not a disposable side cache. RAM is working-set **staging** of store (or pure recompute), never a second truth. The failure mode observed 2026-07-31 is **not** “we forgot to invent caching.” It is: **the hot path never used the memo store as a queryable working set** — full `roots/` re-read (~10⁵ files, multi-second) on every pull, often more than once, before any height sample. A store citizen that cannot be found without scanning the entire save is not yet part of the *hot* architecture.

5. **Parity milestones (minimal ladder — stop when the bar in FE(1) is met).** Each milestone is **done** only when it can fail a probe or a stopwatch on first-light; “implemented in spirit” is not done. **Execution order:** land P0 before spending session attention on P1 thrash policy; P2 can ride with P1; P3 chooses among paths below after thrash is boring; P4 is the declare-done gate, not a separate feature project.

   | ID | Milestone | Done when | Not done while |
   | --- | --- | --- | --- |
   | **P0** | **Index is hot** | `store.roots()` / census after first open is **working-set cheap** — **store-generation counter** (or equivalent), **invalidate on `put`**; pull / census / load share one scan per generation. No multi-second full re-read of ~10⁵ root files per “updating view…”. (Index freshness is **not** content invalidation: `#form-store-as-save` still forbids OS mtime as *truth* for keys; generation is the hot listing’s epoch.) | Every pull re-parses the entire `roots/` tree |
   | **P1** | **Pull thrash is boring** | Steady camera / small orbit after settle does **not** chain multi-second updates; HUD “updating view…” is rare and finishes in **≪ 1 s** on first-light L7 whole-globe present (order-of-magnitude: not “several seconds”) | Latest-wins still rebuilds the wrong request for seconds; roots thrash dominates |
   | **P2** | **Intent is legible** | HUD always states **WHOLE GLOBE vs CLOSE-IN WINDOW**, **mesh level**, and **whether the painted surface is carve / prior / mixed** without requiring archaeology | Postage-stamp windows at cruise altitude; “updating” with no clear target |
   | **P3** | **Open view can show built physics *or* refuse honestly without thrash** | Prefer cheapest honest path that restores spot-check: **(b)** default open lands at a level where the live carve **covers** the mesh, **or** **(c)** open is pure prior but **P0–P2 hold**, HUD “NONE applies” is stable/instant, and zoom-to-carve grain under P1 works. **(a)** far overview samples a **declared coarse product of the live carve** (restriction / overview memo) only if (b)/(c) cannot meet the bar without a product. | 100% prior open + multi-second thrash; cannot tell paint from bed |
   | **P4** | **Spot-check gate (parity)** | With P0–P3: can orbit, zoom to a limit, and **inspect fluvial/ocean/depression** on a built cohort; a weird picture is primarily a **physics or paint-law** question | Still “impossible to differentiate rendering craziness from physics craziness” (Joseph) |

6. **LoD policy under the acceptance.** Prefer **performant wholesale level switch** where it stays under the P1 budget. Introduce **heterogeneous LoD** (coarse shell + fine patch, or multi-block pin) only when wholesale switch **cannot** meet P1 at the grain needed to see the carve. Do not build a full quadtree to pass P4 if P0+P1+P3(b|c) already restore spot-check.

7. **World-pieces pipeline after parity.** Once P4 holds, resume the **physics rank** without apology: sill-graph → flux, phase-gate promises, router adopt/park, cold-stage, χ path — as in `#disc-known-active-hotspots`. Until P4, **instrument honesty outranks new world features** when the two conflict for session attention, because uncheckable physics is not progress. Parallel free capacity may touch world pieces only if it does not starve P0–P4.

8. **Relation to older explorer-intent lists.** The 2026-07-02 / re-affirmed 2026-07-29 explorer-intent block in `#disc-open-problem-census` remains **binding intent inventory**, not this ladder. P0–P4 are the **foundation** under which those intents become checkable.

## Epistemic Status

**Max attainable: discussion-grade** (judgment + operational bar, not a theorem).

**Currently `discussion-grade`.** Sealed 2026-07-31 from Joseph’s acceptance (zoomable performant globe; spot-check must distinguish paint from physics) and measured store pathology (~10⁵ roots, multi-second full census). Not a full product roadmap. Stage `draft`.

## Discussion

Physics-first ordering remains defensible: a truthful bed is worth more than a smooth false picture. The error was scoring **explorer maturity** as optional polish after “the truth layer quiets,” while the explorer was already the only way to *see* whether the truth layer held. `#norm-no-depiction-without-referent` and the changelog acceptance bar already require eyes; P0–P4 only name **when those eyes are competent**.

## Working Notes

- **Companion rank surface:** `#disc-known-active-hotspots` FE(5) carries the living order of work; this segment carries the **parity definition** so rank rows can complete without re-arguing the bar.
- **P0 landed 2026-07-31:** `Store::roots_shared` + generation counter on `put`; pull uses shared `Arc` and throttled `roots_invalidate_if_external` (1 s) for builder-beside-explore. First-light: cold ~2.2 s / 146k roots; warm hit ~ns. Done-when (working-set cheap after first open) holds for the thrash path; cold open still pays one scan.
- **P1 landed 2026-07-31:** observe paths use `roots_shared` (no ~10⁵-entry clone per load/census); pull caches coverage/regions/prior faces; `Msg::AlreadyCurrent` when request equals last completed; Frame carries `Arc` roots + pre-parsed coverage (ECS does not re-walk). Steady camera after settle should not chain multi-second “updating view…”. First load still pays Coverage parse + region scan once (~25 ms each on first-light when empty).
- **Face ocean + P3 (2026-07-31):** explore samples face-domain ocean at ≤L9; first-light rebuilt (~3552 fresh erosion, world `7216c8dca`). Open ≤L7 pure prior by coverage — zoom for fluvial.
- **P4 done-for-now (2026-07-31 night, Joseph):** lattice gone; high-fidelity performance acceptable; FOV stamps good; can wonder about L14/prior vs carve without thrash confusion. **Not** certified perfect; **not** fluvial-complete. Closed-water-cycle still honest red. World-pieces (sill-graph → flux) resume without apology.
- **UI half after P4 (2026-07-31 night):** attention failure named — dense HUD/status is not glanceable. Split: #disc-explorer-human-chrome (chips) · #disc-explorer-debug-capture (udon+png pairs; evolve sightings). Not part of P0–P4 definition; does not reopen thrash bar.
- **Do not** expand P0–P4 into full daemon/spool/quadtree without a new parity failure.
- **Peer audit 2026-07-31:** seal-with-fixes applied — generation vocabulary for P0; P3 prefers (b)/(c) before overview product (a); world-rank rows after P4; stale “1b” rank labels retired in hotspots companion.
