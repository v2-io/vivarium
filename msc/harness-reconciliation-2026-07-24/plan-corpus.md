# Plan corpus vs `core/` — workflow / CLI / harness / builder / demand reconciliation

*2026-07-24. Territory: `.super-archive/from-plan/` (six files, the complete historical `doc/plan/` set — verified: `git log --all --pretty=format: --name-only -- 'doc/plan/*'` returns exactly these six, all deleted in `2c77704`). Written for the agent designing the background-builder / watch-the-edge work, to decide what primary material to read before designing. Every verdict below is a pointer you can check; where I could not check something I say so.*

## Bottom line

The peel is better than "assertions made fast over three days" would predict. Of roughly forty substantive workflow / CLI / builder / demand ideas in this corpus, most landed in core with their content intact, several landed **strengthened** (`#form-depend-by-key-never-latest` is a better statement of build-order independence than its source), and one whole file (`water-parallelism.md`) peeled essentially complete.

**But the losses are not scattered — they cluster, and the cluster is exactly your task.** Everything that went missing or got thinned is about *the time axis of a build and the observability of a build in progress*: time-indexed stage chains, stage-addressability, "the animation is the memo sequence," restart-in-place, per-stage recorded $\varepsilon$, a live watching mechanism, and the one stated real-time-factor target in the corpus. Section 2 is those four findings. Section 5 argues the clustering is structural rather than coincidental, and that the argument is itself worth your attention because it predicts the same failure recurring.

So: **`#scope-segment-canon` FE(4) does not hold for this slice**, but the residue is concentrated enough to fix in one sitting.

**Verdict counts** (my judgment; the ledger in §4 is what you should actually read): in core, intact — 24 · in core but thinned or subtly changed — 9 · nowhere in core — 7.

---

## 1. How to read the verdicts

- **IN** — the idea is in core with its load-bearing content. Cited by slug.
- **THIN** — in core, but something a designer would act on differently is gone: a mechanism, a trigger condition, a target number, a placement constraint. This is your "will actually bite" category.
- **OUT** — not in core in any form I could find. Where a live non-core home exists (LEXICON, DECISIONS, code docs) I name it, because "not in core" and "nowhere" are different problems.

Grep alone did not settle any of these; where a concept could be present under other vocabulary I read the candidate segments end to end. Where I report OUT, I checked the plausible synonym set as well as the obvious one (e.g. for live-watching: `animation`, `real-time`, `mtime`, `watch`, `poll`, `replay`, `tail`).

---

## 2. The four that will bite

### F1 — Component E has no home in core, and the record of *why that is a bug* is only in ice

**Source:** `.super-archive/from-plan/framework-to-status-quo.md` §4 component **E** (lines 82–84): *"Time-indexed stage chains. The settle sequence (macro epochs → fine passes → fill → living) becomes explicit chained recipes with time-indices (§11: time breaks cycles), each stage a memo with its convergence-$\varepsilon$ recorded — the workflow doc's Stage-1 freeze machinery at sub-phase grain."*

**Status in core:** OUT as an owned item. Component E is *mentioned* four times and *owned* zero times:

- `#disc-unlawfulness-budget` Epistemic Status "Not claimed" line — "(component E / stage-chains still unbuilt)";
- `#disc-unlawfulness-budget` Working Notes — listed as a sibling;
- `#detail-builder-daemon` FE(6) — "time-indexed stages" as a slice-order item, no content;
- `LEXICON.udon:279` (epistemic-axes) — "derivation with component E," which is the *only* live statement of what it unlocks.

`#detail-abyssal-parity-build` FE(4) enumerates the six abyssal phases (0 run-modes, 1 store, 2 spine, 3 flux-BC tiles, 4 query front-end, 5 explorer). **Component E maps to none of them.** `#form-three-scoped-runtime` FE(3) holds "time in every key" as doctrine, explicitly unmechanized — that is the *invariant*, not the build component.

**And here is the part worth your time.** The finding that component E was dropped *by accident* is recorded, in the live tree, at `.super-archive/from-archive/TODO.md:94`:

> `doc/plan/framework-to-status-quo.md` §4's component **E** … did **not** carry into `doc/plan/abyssal-parity-plan.md`'s six phases, and no deliberate deferral is recorded — it fell through the consolidation crack (found 2026-07-10). It keeps resurfacing as load-bearing: it is what makes "the beginning of Phase 3.3" *addressable* (beacons need it), what makes intermediate stages monitorable by explorers, what the "watch erosion happen while floating downstream" experience plays back, and its recorded ε *is* the unLawfulness budget (LEXICON §7.2) as data. Recommendation: treat it as plan-Phase-2½ — build it when the builder daemon's stage progression first needs a second time-index.

*(That block is quoted byte-exact, including its bare `ε` — which is the file's one deliberate `lint-md` hit. Altering a quotation to satisfy a linter would be the worse trade.)*

That paragraph is a near-verbatim statement of what Joseph said he has been waiting for, written two weeks before he said it, and it now lives only in a directory core instructs you to treat as deleted. `#disc-open-problem-census` cannot surface it (see §5).

**What to read:** `framework-to-status-quo.md` §4 (component E in context of A–E, and the order-of-attack paragraph at lines 86–87, which puts E *last* — worth arguing with, since the TODO's Phase-2½ recommendation and Joseph's ask both pull it earlier). Then `.super-archive/from-archive/TODO.md:94` itself. **Cross-territory:** TODO.md is the from-archive agent's file; expect a duplicate report and dedupe rather than double-count.

### F2 — "The animation is the memo sequence" and the whole live-watching mechanism are gone

**Source:** `builder-explorer-decoupling.md` line 31 (explorers) and line 36 (instruments):

> *"Live watching needs no IPC: poll roots / watch mtimes; when time-indexed stages exist (component E), **the animation is the memo sequence** — floating downstream while erosion happens is playing back new time-indices as they land."*

> *"**telemetry answer** (Joseph's dangling question): any part of the world at any stage is monitorable by construction — every intermediate is an immutable object; readers never contend with the writer; the cost is disk reads, not coordination. Precondition for *stage*-addressability: component E."*

**Status in core:** OUT for the mechanism and the identity claim; THIN for the telemetry claim.

`#detail-builder-daemon` FE(2) renders the second quote as five words — *"telemetry by construction (intermediates immutable)"* — dropping (a) that the cost is disk reads rather than coordination, (b) that readers never contend with the writer, and (c) **the component-E precondition**. The first quote has no representative at all. `animation`, `real-time`, and `stage-addressab` return zero hits across `core/`.

The single live trace is a Working Note, not a claim: `#form-core-view-wall` Working Notes, *"Next strengthen: … periodic store reload while a builder runs."* That is the right instinct expressed as a chore rather than as the architectural payoff it is.

The distinction that got lost matters for your design: *replay* and *live-watching* are **the same mechanism** under this claim, differing only in whether new time-indices are still landing. If you design them as two features you will build two things.

**What to read:** `builder-explorer-decoupling.md` §1.3 and §1.4 whole (lines 31–36). They are short.

### F3 — Restart-in-place is a dangling pointer: named in one segment as living in another that does not contain it

**Source:** `builder-explorer-decoupling.md` line 29:

> *"Nix-style iteration: edit the spec → keys change → the builder offers **restart-in-place** (recompute exactly the invalidated cone) or **fork** (new spec; forks may share one object pool — the seed is in every key, so worlds cannot alias — giving cross-fork dedup for free)."*

**Status in core:** OUT, with a broken cross-reference on top of it.

`restart-in-place` appears exactly once in `core/` — `#form-builder-admission` Known-incomplete (2): *"Full builder daemon (beacon cones, demand spool, restart-in-place) is design-grade in `#detail-builder-daemon`, not this segment's built surface."* Read `#detail-builder-daemon` and there is no restart-in-place, no fork-vs-restart offer, and no shared-object-pool clause. FE(7) lists "fork UX (BREAK-5)" as open, which is the *naming/browsing* problem, not the iteration offer.

This is the literal shape of Joseph's *"restarting at the right points as we iterate on the algorithms."* It is currently a word inside a parenthesis pointing at a file that dropped it.

The shared-object-pool clause is separately OUT (zero hits for `object pool` anywhere live) and is a real design fact: because the seed is in every key, forks cannot alias, so N forks of a spec can share one `objects/` directory and dedupe for free. That is the difference between "iterating on the algorithm costs a new world each time" and "iterating costs the delta."

### F4 — Phase-1's acceptance probe is stated as live in one segment and quietly declared not-owed in another

You measured that one comment line invalidates 24/24 tiles. Core owns both halves of this and they disagree without noticing:

- `#detail-abyssal-parity-build` FE(4), phase 1: *"probe: edit kernel invalidates only dependent cone."* Stated as the plan's acceptance test, with no note that it currently fails.
- `#form-complete-content-addressed-key` FE(4) and Known-incomplete (3): the digest is whole-crate, *"editing any `.rs` (including a comment or a test) invalidates the whole store — over-keying, the safe direction. Finer per-nomos attribution is **deferred, not owed**."*

Neither cites the other. A designer reading §III finds the probe; a designer reading §II finds the deferral.

One substantive wrinkle, offered as a question rather than a verdict. Core's argument for the deferral is that *"any hand-maintained source→nomos map would reintroduce the bump-discipline this removes."* The source proposed something narrower: `abyssal-parity-plan.md` line 52 — *"a `build.rs` that hashes **each kernel module** into a compile-time key component."* Per-module hashing does not require a hand-maintained map if the grain is the file; what it requires is a derivable module→nomos edge set. Core's own ceiling condition says exactly that — *"the ceiling waits until the module graph makes attribution derivable"* — so the two are closer than the rebuttal's framing suggests. Whether the module graph is derivable today is an engineering judgment I did not make; I flag only that core's rebuttal answers a weaker proposal than the one on record.

**What to read:** `abyssal-parity-plan.md` Phase 0 and Phase 1 (lines 48–52). Phase 0's second probe clause — *"a scratch iteration run and canon share `objects/` without polluting `roots`"* — is the run-modes half and is honestly carried as an OUTLINE gap.

---

## 3. Smaller THIN findings worth two minutes each

- **Builder v0 was specified to sweep a beacon cone, not the world.** `builder-explorer-decoupling.md` line 62: *"builder v0 sweeps a **beacon cone** at spine+erosion rungs."* `#detail-builder-daemon` FE(6) compresses this to "CLI build/status/attach," dropping the cone. The live CLI header (`bin/vivarium.rs`) is honest about the divergence — *"no beacon parsing from the manifest (the sweep IS the whole-world beacon)"* — but core no longer records that the whole-world sweep was the *degenerate* case rather than the design.
- **`vivarium status` has a stated UX law that core dropped.** `regula-conformance-design.md` line 76: derive the floor **for the target phase only** — *"the aspiration-dump guard: a regula reaching for Phase 8 makes every report a wall of red, and an ignored dashboard is worse than none. Later phases render as *horizon*, never as failures. The lineage's `target_phase` advances as the work does."* Zero hits in core for the guard or for `target_phase`. This bears directly on "a principled understanding of what exactly they are watching": it is the rule that keeps the instrument readable as the ladder grows.
- **Watchpoints lost their trigger semantics.** Source (line 35): a watchpoint *"emits an ASCII/image snapshot + stats **when the builder reaches it**"* — an event fired by the writer. `#detail-builder-daemon` FE(2) says *"declared place/level/stage snapshots,"* which reads as a standing query. Query and trigger are different builds. Also: `watchpoint` has **no LEXICON entry**, unlike `beacon` and `focus` (`LEXICON.udon:137–140`, settled 2026-07-10), and its naming trail — deliberately not "breakpoint," with "checkpoint" ceded to ASF — survives only in the ice.
- **The one real-time target in the corpus is gone.** Source line 57, temporal-ladder rung 3: *"human-scale live water, full precision, **~8× real-time**, entered warm … memoizing the fill is simultaneously what kills the old spike's ~2 h re-fill and what frees the budget for 8×."* `#detail-builder-daemon` FE(5) keeps "entered warm (settled fill memoized)" and drops the factor and the budget argument. Given your 68%-of-build-time water measurement, the budget argument is the one you would want to argue with.
- **The convergence detector we already have is the $\varepsilon$-gate ingredient.** `framework-to-status-quo.md` line 43: *"The convergence detector exists (per-epoch mean $\lvert\Delta h\rvert$; the fill plateau logic) — the $\varepsilon$-gate ingredient for phase/stage freezes."* Zero hits in core for `convergence detector`. This is component E's cheapest starting point and core does not know it is already in the tree.
- **Invalidation-grain selection.** `builder-explorer-decoupling.md` line 18: a native artifact invalidates all dependents at its own grain, *"so a native system chooses its memo grain to match its consumers' cones (per-basin, not one global graph)."* `#form-depend-by-key-never-latest` FE(5) keeps "keys may name artifacts" and "over-key is safe," dropping the grain-selection rule. Same idea-family as F4 — it is the design answer to "how do I avoid a single object whose invalidation nukes the world."
- **Access profiles attach at the query front-end, not in the store** (source line 49). `#detail-builder-daemon` FE(4) keeps the process-boundary claim and drops the placement constraint. Low urgency, but it is a constraint on where you put the explorer API.
- **Explorers can offer to start a builder** (source line 31: *"can *offer* to start one ('demand exists, no builder')"*). Core keeps "work with no builder," drops the offer. Small, and it is UX you would otherwise re-invent.
- **Phase-transitions are the tag points.** `vivium-operational-workflow.md` line 28 and line 58: *"The phase-transition IS the checkpoint/tag/publishable artifact."* `#detail-vivium-lifecycle` FE(2) keeps "memo to disk, tag" and FE(4) keeps the publish bundle, but the identity — *the transition is the citable artifact* — is not stated in either, nor in `#form-in-vivia-citation`. Worth restoring because it is what makes a build's history navigable rather than a blob.

---

## 4. Full ledger

### `builder-explorer-decoupling.md` — the file most load-bearing for your task

| # | Idea | Verdict | Where |
|---|---|---|---|
| 1 | Build-order independence: different demand orders converge byte-identical | IN (strengthened) | `#form-depend-by-key-never-latest` FE(1) |
| 2 | Keys name artifacts, not only cells | IN | same, FE(5) |
| 3 | Invalidation grain matched to consumers' cones | THIN | §3 above |
| 4 | Store is the bus; no mediator; benign write race | IN | `#detail-builder-daemon` FE(1) |
| 5 | Spec buckets identity / label / demand; demand editable mid-build | IN | `#detail-builder-daemon` FE(2); `#form-manifest-prescribes-vivium` FE(2); `LEXICON.udon:140` |
| 6 | Builder owns scheduling, never truth; frontier → queue; log; `status.json` | IN | `#detail-builder-daemon` FE(2) |
| 7 | Lockfile-attach; second `build` attaches; pause cheap | IN | same; live in `bin/vivarium.rs` |
| 8 | **Restart-in-place vs fork on spec edit** | **OUT** | F3 |
| 9 | **Forks share one object pool; cross-fork dedup free** | **OUT** | F3 |
| 10 | Explorers read-only, process-decoupled, coarse on miss, demand file | IN | `#detail-builder-daemon` FE(2) |
| 11 | Explorers offer to start a builder | THIN | §3 |
| 12 | **Live watching by polling roots / mtimes; animation = memo sequence** | **OUT** | F2 |
| 13 | Fidelity pyramid = roots histogram by level × stage | IN | `#detail-builder-daemon` FE(2); live as nomos × level |
| 14 | Watchpoints fire when the builder reaches them | THIN | §3 |
| 15 | Telemetry by construction; disk reads not coordination; component-E precondition | THIN | F2 |
| 16 | Unmet frontier of demanded cones; per-beacon depth vs breadth | IN | `#detail-builder-daemon` FE(3) |
| 17 | Hilbert = storage locality; drainage = dependency; two structures | IN | same |
| 18 | Access profiles = process boundaries | IN | `#detail-builder-daemon` FE(4) |
| 19 | Profiles attach at query front-end, not store | THIN | §3 |
| 20 | Canon-root guard tripwire on two-process sharing | IN | `#detail-builder-daemon` FE(4); `#form-store-as-save` FE(8) |
| 21 | Temporal ladder, three rungs | IN | `#detail-builder-daemon` FE(5) |
| 22 | Rung 3 at ~8× real-time; memoized fill frees the budget | THIN | §3 |
| 23 | Initial slice: `new`; fg/bg; **beacon cone**; stage-addressable beacons; "watch it age"; explorer upgrades as memos land | THIN (several clauses OUT) | F1, F2, §3 |
| 24 | Open: spool format, guard mechanization, fork UX, multi-machine | IN | `#detail-builder-daemon` FE(7) |

### `framework-to-status-quo.md`

| # | Idea | Verdict | Where |
|---|---|---|---|
| 25 | Status quo: one patch, telescope re-seeds, fill-cache blob, non-composable tiles | IN (numbers dropped, fine) | `#detail-abyssal-parity-build` FE(2); `#detail-drainage-dependency-planning` FE(5) |
| 26 | Kernels port; prior is the global rung; `column_at` is the query seed | IN | `#detail-abyssal-parity-build` FE(2); `#form-pull-query-composition` |
| 27 | Fill cache is the memo prototype; its lessons (key completeness, version pinning, two-leg test) transfer | THIN — the FILL_CAP lesson is in `#form-complete-content-addressed-key`; the **two-leg cache test** is nowhere | — |
| 28 | **Convergence detector already exists; it is the $\varepsilon$-gate ingredient** | **OUT** | §3 |
| 29 | Dependency cone = drainage island, not halo | IN (clean peel) | `#detail-drainage-dependency-planning` FE(1) |
| 30 | Spine is the planner; inputs discovered from spine not adjacency | IN | same, FE(2) |
| 31 | Degree is flux magnitude; sets required pull fidelity | IN | same, FE(3) |
| 32 | Seam fix = composability; `seam_ridge` is a correctness gate | IN | same, FE(4) |
| 33 | Build shape A–D | IN, redistributed | `#detail-abyssal-parity-build` FE(4); `#form-three-scoped-runtime` |
| 34 | **Build shape E — time-indexed stage chains with recorded $\varepsilon$** | **OUT** | F1 |
| 35 | Order of attack: A, B, D early (A+B alone gives persistent navigation), C long pole, E last | THIN — the "thinner milestone" alternative survives at `#detail-abyssal-parity-build` FE(6); the *D-early* reasoning does not | — |
| 36 | Open risks: bidirectional water, time-index across aged tiles, spine coarseness | IN | `#detail-drainage-dependency-planning` FE(6) |

### `vivium-operational-workflow.md`

| # | Idea | Verdict | Where |
|---|---|---|---|
| 37 | Stages 0–4, BREAK-1/3/4/5, capability-ladder table, eight doctrine rules, BREAK-1′ | IN (thorough peel; the table survives intact) | `#detail-vivium-lifecycle` FE(1)–(8) |
| 38 | BREAK-2, unLawfulness budget, Realized ⟂ Lawful, completion-gate retirement | IN | `#disc-unlawfulness-budget` |
| 39 | Convergence gate with stated $\varepsilon$ **as a required artifact before a phase may freeze** | THIN — the budget is claimed; the *gate as a build step* is not | `#disc-unlawfulness-budget` FE(3) |
| 40 | Memo format `{seed, law-versions, generator-versions, frozen-state, provenance}` | IN, split across `#form-store-as-save` FE(2) and `#form-in-vivia-citation` FE(1) | — |
| 41 | **Phase-transition IS the tag / citable artifact** | THIN → effectively OUT | §3 |
| 42 | Realizability gate at ~2 Hz; memoize under horizon; predictive pre-memo | IN | `#detail-vivium-lifecycle` FE(3); `LEXICON.udon:610` |
| 43 | "checkpoint" ceded to ASF; cache/snapshot = memo | IN (LEXICON, not core) | `LEXICON.udon:344` — note the PENDING Joseph call recorded there |

### `regula-conformance-design.md`

Partly *superseded by decision*, correctly: `#form-manifest-prescribes-vivium` FE(3) collapses regula into ordinum + manifest (`DECISIONS[regula-collapses-to-order-and-manifest]`, `:by us`). `#detail-regula-design` preserves the design so it is not only ice. That is integration-as-replacement done right, and I have no complaint about it.

| # | Idea | Verdict | Where |
|---|---|---|---|
| 44 | Honesty stack at three grains; audit is a computation | IN | `#detail-regula-design` FE(1) |
| 45 | Ordinum pinning; PHASES revision re-versions, never silent re-grade | IN | same FE(2); `#form-ordinum-governs-flux-web` |
| 46 | Permits, void conditions, requisite closure as a planning query, `vivarium audit --add` | IN | `#detail-regula-design` FE(3) |
| 47 | Results are memos with cones; touched-in-seconds vs changed-in-minutes by hash compare | IN | same FE(4) |
| 48 | `vivarium status` reports conformance-to-pin and gap-to-head | IN | same FE(5) |
| 49 | Promise maturity ladder; no-predicate cannot be fulfilled; defeased-but-consumed is an error | IN (and built) | `#form-ordinum-governs-flux-web` FE(4); `ordinum.rs` |
| 50 | **Aspiration-dump guard: target-phase-only derivation; later phases as horizon** | **OUT** | §3 |
| 51 | Minima only — regula declares floors; "best occupant" is the registry's answer | OUT (`minima only` returns nothing) | — |
| 52 | Sequencing: declaration-level conformance now, **artifact-level conformance rides component E** | OUT | F1 |
| 53 | Two chapters; participation chapter; regulae nest; moratorium supremacy | IN | `#detail-regula-design` FE(7); `LEXICON.udon:308` |
| 54 | Naming trail (regula / Terrestris / Ordinum; register convention) | IN | same FE(8) |

### `abyssal-parity-plan.md`

| # | Idea | Verdict | Where |
|---|---|---|---|
| 55 | Milestone decomposition; moratorium-clear by construction | IN | `#detail-abyssal-parity-build` FE(1) |
| 56 | Seven-item conformance checklist | IN | same FE(3) |
| 57 | Six phases with deliverable + probe each | IN | same FE(4) |
| 58 | Phase 1 per-**kernel-module** source hashing | THIN | F4 |
| 59 | Phase 1 probe "exactly its cone and nothing else" | IN, but in tension with `#form-complete-content-addressed-key` | F4 |
| 60 | Phase 0 run-modes carve: `causal`/`iterating`, `Incomplete → Realized → Deployed`, mutation-log compatibility as the release contract | THIN — core keeps the carve shape (`#detail-abyssal-parity-build` FE(4.0), `#form-store-as-save` FE(8)); the **mutation-log-compatibility semver-for-worlds** formulation is not restated anywhere I found | — |
| 61 | Seam at Abyssal fidelity; observer resolution cone | IN | same FE(5) |
| 62 | Alternatives: thinner spine-only milestone; analytic init as an early parallel spike; algae-bloom generality probe | IN | same FE(6) |
| 63 | Honest risks | IN | same FE(7) |

### `water-parallelism.md`

Cleanest peel in the corpus. `#detail-water-parallelism` FE(1)–(6) covers gather-not-scatter, staging, the CPU-reference determinism policy with backend identity in the key, what stays CPU, and the placement constraint. The only losses are the measured baseline (~100 steps/s single-threaded at $1024^2$) and the explicit link to time-controls / fast-forward as a *motivating consumer* — core keeps "fast-forward" as a phrase. Given water is 68% of your build time, FE(2) is the live actionable item and it is intact.

---

## 5. The framing note — why these particular things fell out, and why the project cannot see it

You asked whether the framing itself might be wrong. I think the question "is it in core" is right but incomplete, and the missing half predicts recurrence.

**Observation.** Sort the OUT and THIN rows by subject and they are one subject: *time-indexed stages, stage-addressability, per-stage $\varepsilon$, live watching, replay, restart-on-edit, a real-time factor.* Sort them by *claim shape* and they are also one shape: none of them is a **law**. They are build-sequence facts, experience claims, and UX rules. The ideas that survived are the ones that could be phrased as invariants.

**Mechanism.** The peel routed each idea to the segment that owns its law. That is the right discipline and it is why the law-shaped material peeled so well. But an idea with no owning law has nowhere to land, and the two segments that could have absorbed the residue — `#detail-builder-daemon` and `#detail-abyssal-parity-build` — were themselves written in Formal-Expression cadence, which rewards enumerable structural facts and quietly sheds process and experience claims. Compare `builder-explorer-decoupling.md` §1.4 (three sentences of telemetry reasoning) with its five-word rendering in `#detail-builder-daemon` FE(2). Nothing was decided away; the container had no slot.

**Why it is invisible.** `#disc-open-problem-census` FE(1) is explicit that it derives from OUTLINE gap rows and segment open-residues. OUTLINE §III has exactly two gap rows — run-modes carve, builder-daemon implementation. There is no gap row for time-indexed stage chains. So the project's own open-problem instrument **structurally cannot** surface F1, and its honesty about that ("does not claim completeness of unknown unknowns") does not cover it, because this is not an unknown unknown — it was known on 2026-07-10, written down, and then the only carrier was archived.

**What I would do with that, offered as a guess rather than a recommendation.** The cheapest durable fix is a `--GAP--` row in OUTLINE §III, typed, for time-indexed stage chains / stage-addressability / recorded $\varepsilon$ — because a named absence is visible to the census and a paragraph in ice is not. Whether component E then becomes its own segment or content inside `#detail-builder-daemon` is your call and I would not have a better view of it than you.

**One consequence for your design, which is the practical version of all of the above.** Four things Joseph named — background building, restart at the right points, watching the edge in real time, watching replays — are, on this corpus's account, **not four features.** They are one feature (time-indices in the key, with $\varepsilon$ recorded per stage) plus one trivial reader (poll the roots). If you design them as four you will build four. That single claim is the part of the plan corpus I would most want you to have before you start, and it is the part core does not currently carry.

---

## 6. Coordination notes for you

- `.super-archive/from-archive/TODO.md:94` is the from-archive agent's territory and carries F1's provenance. Expect overlap; the finding is the same one.
- `msc/build-parameterization-findings-2026-07-24.md` (live, yours already by your commit history) independently reaches F1's neighborhood from the code side — *"Joseph, in effect, rediscovered why the builder daemon is the highest-leverage unbuilt runtime piece."* Its work item 1 (nested epoch sampling so densification does not invalidate existing memos) is a component-E-shaped problem stated in code vocabulary; worth reading beside `framework-to-status-quo.md` §4E.
- `msc/personal/1d0c43ec-…-scratch/16-the-plan-tree.md` is a 2026-07-13 batched read of exactly my six files. It reached F1 and F2 independently. I found it after forming my own verdicts, and I am reporting the convergence as convergence, not as a source. Its §3 (eight independent instances of claim-paired-with-falsifier) is adjacent to your task rather than in it, but it is a good read.
- I edited nothing outside this directory.

## 7. Feedback on the brief

It worked. Three things that mattered: naming the coordination split so I did not defensively re-read everything; volunteering that your own file list had been wrong an hour earlier, which made me check the corpus boundary myself (`git log --all` on `doc/plan/*`) instead of trusting the six names — that check is the reason I can say the corpus is complete rather than that it is what I was handed; and saying plainly that "yes, it's all in core" was a good answer, which made the IN rows cheap to write honestly rather than something to apologize for.

The one thing I would have wanted earlier: whether `msc/personal/` is in scope. I read it because your brief licensed following the reading, and because it turned out to contain a prior independent read of my exact corpus — but "personal" in the path made me hesitate, and hesitation cost a beat. If personal scratch is fair game for future agents, saying so is worth a clause.

The list of candidate owner segments was accurate and complete for what I found — `#form-core-view-wall` and `#form-complete-content-addressed-key` were the two I would add, and both are one hop from your list.

Happy to stay on the line.
