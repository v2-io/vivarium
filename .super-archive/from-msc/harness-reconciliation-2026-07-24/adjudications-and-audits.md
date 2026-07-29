# Harness reconciliation — the adjudication and audit layer

*Peer report, 2026-07-24. Not claim canon. Reader: the harness-design session, today. Every gap claim below names a file and a line/section you can check in under a minute; where I could not verify something I say so.*

---

## Bottom line

**For the `msc/` + `audits/` slice, the peel was clean: I found no thinking that was destroyed.** Every adjudication verdict I could check was either honored or knowingly escalated, and the escalations are honest about it. `audits/README.md`'s "no pending reports" is accurate — the three 2026-07-21 reports have disposition trails whose "landed" claims I spot-checked against segments and they hold.

**But "it's all in core" is false for your slice, for a reason the peel was never scoped to catch.** The consolidation campaign was *prose → segments* (`doc/`, `.archive/`, `msc/` bodies). It was never *DECISIONS → segments*. `DECISIONS.decision-log.udon` is a named carve-out in `#scope-segment-canon` FE(3), so nothing is *wrong* — but it means a whole class of operational decision is authoritative and uncited by core:

| `:topic` | rows | cited by any core segment |
|---|---|---|
| `tooling` | 2 | **0** |
| `view` | 3 | **0** |
| `process` | 15 | 4 |
| `architecture` | 24 | 11 |

Reproduce: the one-liner in §6. The `tooling` and `view` buckets are almost exactly your slice, and they are the two with zero core citations.

So the honest answer to Joseph's condition is: **core carries the workflow/CLI/harness *architecture* well, and carries almost none of the workflow/CLI/harness *conventions and instrument wishes*, which live decided-but-unindexed in DECISIONS.** Three of those are Joseph's own, one is `:status wish` and describes the exact instrument you're about to build.

Second-order finding, and the one I'd act on first: **several "leave it as a gap" verdicts were priced against a world in which nobody was building the builder daemon.** They are conditional on tripwires. Your project trips at least one of them in week one (§4).

---

## 1. What I read

**Fully:** all 10 files in `msc/agent-briefs/` except two scanned by keyword (`2026-07-23-cell-area-spike-plan.md`, `2026-07-23-sufficient-statistic-adjudication.md` — confirmed no workflow/CLI/harness content beyond the flux-statistic grain). `doc/PROCESS.udon` (the whole live `doc/` tree is that one file — verified with `find doc -type f`). `audits/README.md`, `audits/.integrated/2026-07-21-de-novo-project-audit.md`, `…-audit-b.md`, both `DISPOSITION-*.md`; the segment-floor audit scanned by keyword. `msc/build-parameterization-findings-2026-07-24.md`, `msc/promotion-mine-2026-07-23-continuity.md`, `msc/claim-channel-cleanup-2026-07-23.md`.

**Cross-read in core:** `OUTLINE.md`, `scope-segment-canon`, `form-pull-query-composition`, `form-three-scoped-runtime`, `form-builder-admission`, `form-store-as-save`, `form-complete-content-addressed-key`, `form-core-view-wall`, `form-sphere-continuous-surface-fields`, `form-manifest-prescribes-vivium`, `detail-builder-daemon`, `detail-abyssal-parity-build`, `detail-vivium-lifecycle`, `detail-drainage-dependency-planning`, `form-scale-separation-directional`, `ops-audit-integration`, `disc-open-problem-census`.

**Instruments:** `DECISIONS.decision-log.udon` (full slug list; ~10 entries read in full), `LEXICON.udon` `|term[run-modes]`, `.super-archive/` and `.archive/` file inventories.

**Not done:** I did not read the code. Per `CLAUDE.md` §"Code rank" that is correct for adjudicating architecture, but it means where I say "X is unbuilt" I am reporting what the segments and DECISIONS say, not what I compiled.

---

## 2. In core, and well — do not re-derive these

| Idea | Home | Note for you |
|---|---|---|
| Pull composition; deps are recursive pulls; **demand schedules, keys define** | `#form-pull-query-composition` FE(1)–(4), `exact` | FE(3) is the sentence to design against: "*which* keys are scheduled is demand (builder beacons, explorer spool, phase target); *what* a key's bytes are is fixed by the complete key graph. Conflating schedule with ontology is the failure mode." |
| **Spine / cones / edit-layer**, time-in-key DAG, prefetch-is-pure-optimization | `#form-three-scoped-runtime`, `robust-qualitative` | This is the architecture of "background building toward known points of interest, watched at the edge." FE(2) fan-out-then-in is the one that will bite a naive cone design: global aspects are non-local, so "just enough under the pawn" is false for uplift. FE(6): prefetch built last, nothing depends on it for truth. |
| Builder refuses unmet flux; `--allow-unmet` → provisional roots; explorers observe-only; **never-block first light** | `#form-builder-admission`, `exact` | Convicted at the binary boundary (`tests/cli_admission.rs`) per its Epistemic Status. KI(5) is the full first-light story, both halves landed. |
| Store is the save; memoized ≡ store object (definitional, `DECISIONS[memoized-means-store-object]` `:by joseph :status decided`) | `#form-store-as-save` FE(6) | "A per-process cache holding results worth keeping past the working set is a **bug against this law**." Binding on anything you build. |
| Complete key; over-key never under-key; **whole-crate source digest** | `#form-complete-content-addressed-key` FE(4) + KI(3) | Your one-comment-line experiment is *exactly* what FE(4)/KI(3) claim. See §3.5 — core also carries a **decision** about the granularity that you should read before designing. |
| Manifest = per-vivium prescription incl. **target phase, permits, demand posture** | `#form-manifest-prescribes-vivium` FE(2), FE(5) admits thin subset live | The designed home for build parameterization. |
| Builder daemon: store-as-bus, **beacons → causal cones → work queue**, demand spool, lockfile-**attach** (second build attaches, does not fail), pause-cheap, fidelity pyramid, watchpoints | `#detail-builder-daemon`, `discussion-grade` | 52 lines. Terse but the named pieces are all there. FE(7) "Open" names demand-spool format as undecided. |
| Six-phase path to ethereal explorer (Phase 4 query front-end; Phase 5 explorer) | `#detail-abyssal-parity-build` | Epistemic Status warns Phase-3 MFD-era assumptions were partly destabilized Jul-13 — reconcile before citing as engineering truth. |
| Dependency cone for fluvial state is the **drainage island**, not a halo; spine is the planner; flux magnitude sets pull fidelity | `#detail-drainage-dependency-planning` FE(1)–(3) | This is what "known points of interest" actually costs for erosion. |
| Realizability/tempo gate (~2 Hz), pre-memo toward attention, BREAK-3 | `#detail-vivium-lifecycle` FE(3), FE(7) | |
| Audit process; agentic verdict is a fallible flag | `#ops-audit-integration`; `doc/PROCESS.udon` `|norm[agentic-verdict-is-flag]` | The archive-residual adjudication recommended process-only-not-a-segment for the latter; that is exactly what landed. |

---

## 3. In core but weakened, partial, or misfiled — the category you asked about

### 3.1 `#form-store-as-save` FE(8) still carries the drift the run-modes adjudication convicted

**Check:** `core/src/form-store-as-save.md` FE(8) last clause vs `msc/agent-briefs/2026-07-23-run-modes-adjudication.md` §"Critical drift" and incidental finding #1.

FE(8) says: *"Mechanization of that guard is compliance debt; Phase-0 convention + provisional banners are present practice."*

The adjudication established that **"provisional banner" and "provisional roots" are different predicates that share a word**:

- *Phase-0 provisional banner* = a global "all state is iterating, not canon" honesty banner, proposed 2026-07-10, **never implemented**.
- *provisional roots* = the per-root third-line flag meaning **flux waiver** (`--allow-unmet`), which **is** implemented.

Reading FE(8) as written, you would conclude a run-isolation stand-in is in present practice. It is not. The segment's own KI(4) describes the correct (waiver) meaning — so the segment contains both readings and never states the non-identity. The adjudication asked for that non-identity to be stated "in FE or Epistemic Status (or even without a run-modes segment — residual truth cleanup)." **Not done.** Severity: low as truth, real as a design trap for you specifically, because you are the first person who will actually have two writers.

### 3.2 The most workflow-relevant fact in the repo is filed under a physics segment

**Check:** `core/src/form-sphere-continuous-surface-fields.md` — Epistemic Status ¶"View-assembly staleness (2026-07-24)" and Working Notes final bullet.

That segment is about sampling noise continuously on $S^2$. It also carries, in full: the diagnosis of the edit→build→globe staleness loop; `eroded_region_census` (fresh/stale/total by source hash); `load_current_eroded_regions`; the `VIVARIUM_INCLUDE_STALE=1` diagnostic escape; the loud-not-silent census HUD; the still-open 1-cell coverage gap (`ErodedRegion::grid_pos` covered only on `[0, nx-2]`, so the last row/col of every 64-tile falls back to prior even under a matching source hash — named as a builder/region tiling debt, "not fixed here"); and the **verified workflow sentence** from `DECISIONS[globe-ribbons-are-view-assembly-staleness-not-prior-or-kernel]`: *edit `vivarium-world` → `vivarium build` → `vivarium-globe`.*

Nothing in OUTLINE §III (Runtime, machine, and CLI) or §VI (Process) mentions any of it. A person designing the build harness reads §III and §VI. **The content survived; the routing did not.** This is a findability defect, not a truth defect — but it is the single item I'd most expect you to re-derive, because you already re-measured half of it (the store invalidation) from scratch this morning.

The census census also matters directly to your goal: it is the closest thing that exists to "what is fresh vs stale at the frontier," and it is per-region, source-hash-keyed, and already loud.

### 3.3 `#detail-builder-daemon` is thin relative to what it supersedes, and its slice order is dated

**Check:** `core/src/detail-builder-daemon.md` FE(6) against `.super-archive/from-plan/builder-explorer-decoupling.md` (the from-plan agent's territory — coordinate).

FE(6) "Initial slice order" reads: *"Spec+seed landed; next: CLI build/status/attach; demand spool + read-only query; time-indexed stages; ethereal explorer."* Since it was written, `build`/`status` landed and the epoch ladder became store citizens. `attach`, the demand spool, and the explorer did not. The segment does not distinguish these — it presents one undifferentiated "next." The Working Note says it "supersedes builder-explorer-decoupling as design home," which under integration-is-replacement means **the graduated plan is treated as deleted and this 52-line segment is all you get**. I did not diff the two (that's the other agent's slice), but the compression ratio is worth their explicit verdict, and I'd read theirs before designing scheduling.

Also: `#detail-builder-daemon` is `discussion-grade` and has **no DECISIONS row**. Nothing in it is ratified. That is honest, and it also means you are free to reshape it — you are not re-litigating a decision, you are filling in a sketch.

### 3.4 `--epochs` has a note and a DECISIONS row but deliberately no core home

**Check:** `msc/build-parameterization-findings-2026-07-24.md` + `DECISIONS[build-cli-parameterization-is-manifest-and-nomos-debt]` (`:by claude :status proposed`, 2026-07-24).

You've read the note. Two things from the DECISIONS entry worth having: (a) it is explicit that **"no segment changed"** and that a harness owner landing parameter-home decisions "may warrant their own DECISIONS row + a build/manifest ops segment" — i.e. minting that segment is anticipated, not a violation; (b) it carries the near-miss (the `erosion_epochs` / multirate false alarm) into the durable layer, so that one is safely recorded.

### 3.5 Core carries a *decision* on invalidation granularity that runs against your stated goal — and it has no DECISIONS row

**Check:** `core/src/form-complete-content-addressed-key.md` FE(4) and Known-incomplete (3).

FE(4) owns the build-time whole-crate source digest as landed law: *"a kernel edit invalidates its memos whether or not the hand-stamped version string was bumped — human bump-discipline is no longer load-bearing for correctness."* KI(3) then decides the granularity question:

> "The build-time digest is whole-crate: editing any `.rs` (including a comment or a test) invalidates the whole store — over-keying, the safe direction. **Finer per-nomos attribution is deferred, not owed**; any hand-maintained source→nomos map would reintroduce the bump-discipline this removes, so **the ceiling waits until the module graph makes attribution derivable**."

"Restarting at the right points as we iterate on the algorithms" is precisely what per-nomos attribution buys, so this is the clause your design has to meet. Note its exact shape — it is not a prohibition. It rejects one *mechanism* (a hand-maintained map, because it re-opens the bump-discipline hole that FE(4) just closed) and states a *condition* for lifting the ceiling (derivable attribution from the module graph). An argument that derives attribution mechanically — or that routes around the problem entirely, e.g. by retaining superseded generations rather than by keying more finely — is not re-litigating anything.

**Authority check:** I grepped `DECISIONS.decision-log.udon` for `source.digest|src_hash|SRC_HASH|source-derived|whole-crate`. There is **no decision row for the source-digest work at all** — it appears only inside the `|impact` and `|reason` of other entries (`code-to-claim-wave-…`, `epoch-surfaces-are-store-citizens`, `craton-nucleation-growth-built-…`). So FE(4) is `exact` architecture law and KI(3) is an agent-made engineering call, both un-ratified. Under `#norm-decision-authority` that is a decision you may reopen with evidence, not a Joseph wall.

**Related open item that core's own census misses:** "GC of superseded-by-`SRC_HASH` generations" is named as open in `DECISIONS[epoch-surfaces-are-store-citizens]` `|impact` and in `CONSOLIDATION-STATUS.md`, but does **not** appear in `#disc-open-problem-census` — whose whole purpose (FE(1)) is to be the derived-from-core list of named open work. Small census gap; matters to you because GC and restart-points are the same question seen twice.

### 3.6 "Watch the frontier advance" exists only as a Working Note

**Check:** `core/src/form-core-view-wall.md` Working Notes, bullet 3.

> "Next strengthen: demand spool; water-tile depth load into observe path; **periodic store reload while a builder runs**; clippy bans for rendering crates in world packages."

That middle clause is the mechanism for Joseph's "exploration able to watch it in real-time at the edge." It is a to-do line in forward residue, which is the correct place for it under FORMAT — but it means the mechanism is one clause long and nobody has thought it through. Nothing in `#form-builder-admission` FE(4) (observe-only pull) says whether re-reading the store while a builder writes is safe; `#detail-builder-daemon` FE(1) says the store-as-bus makes "readers safe against writers without protocol," which is the claim that would license it. Those two want to meet.

---

## 4. Nowhere — and one of them is your week-one decision

### 4.1 Run-modes: the gap is honest, the *reasoning* is substantial, and your project fires its tripwire

`core/OUTLINE.md` §III carries the gap row. `LEXICON.udon` `|term[run-modes]` is `:status open`. I re-verified the adjudication's central authority finding: **`DECISIONS` has zero run-mode, canon-root, or `iterating` entries** (grep in §6). So the gap is honest and the parent's "leave it" was the recommended default.

What the adjudication holds that you'd otherwise redo (`msc/agent-briefs/2026-07-23-run-modes-adjudication.md`, §"What is actually true today" and §"If promote: thin FE"):

1. Four referents share the phrase "running the world": strictly temporal-causal · replay from pinned generators · **discardable iteration** · live play accruing history. Names deliberately open.
2. **Root-write isolation is the load-bearing half.** Complete keys already make `objects/` safe to share; `roots`/tags are the entire remaining integrity surface.
3. The guard is **convention-only** — the brief verified there is no tooling, no `RunMode` type, no scratch-root namespace, and no refusal path anywhere.
4. Axes are orthogonal: root-write permission (placeholders `causal`/`iterating`) ≠ kingdom `Realized` ≠ `Deployed` release head.
5. The `iterating → candidate → realized` lifecycle is plan-only, never instituted.

**The tripwire is the operative part.** §3 of the brief records **two** tripwires from two different sources, and flags that they disagree:

- Phase-0 (`abyssal-parity-plan`): mechanize the canon-root guard at **first graduation** of authoritative macro state.
- `builder-explorer-decoupling` §3/§6: revisit convention-only when **two processes share a store** — which the brief notes is the **stricter** one and "may fire *before* first graduation."

`#detail-builder-daemon` FE(4) carries the same warning: *"Tripwire: two-process store sharing may force mechanism-enforced canon-root guard earlier than first graduation."*

A background builder plus a live explorer watching the frontier **is** two processes sharing a store. The verdict "leave the gap until a tripwire" was correct on 2026-07-23 because nothing was about to trip it. You are the thing that trips it. That does not mean you must mint the segment — but it means the question arrives with your first design, not later, and the incidental finding #3 of that brief is worth carrying: any thin segment should name **both** tripwires, or say "whichever first."

### 4.2 Five decided/wish DECISIONS rows about your exact slice, uncited by any core segment

These are authoritative under `#scope-segment-canon` FE(3) and invisible from OUTLINE. Two are Joseph's.

| Row | `:by` / `:status` | Why it's yours |
|---|---|---|
| `ascii-globe-in-info-colored-by-state` | **`:by joseph :status wish`** (2026-07-12) | *"render the world's build-state legible AT A GLANCE, geographically — coloring regions by what they have reached in SPACE and TIME/PHASE (which detail level / phase is materialized where)."* This is Joseph's own prior articulation of "watch the frontier." Framed as a **nullable wish** ("a feature to TRY when appropriate, NOT a requirement; drop without guilt"). Names starting points (`globe_ascii` example, `spikes/globe`) and a projection preference (HEALPix *unless* a cube-sphere-native projection maps our cells more faithfully — and notes our data is native cube-sphere `CellId`). The nearest core surface is `#detail-builder-daemon` FE(2) "fidelity pyramid (histogram roots by level×stage)" — which `vivarium status` renders, and which is **not geographic**. |
| `new-system-must-reach-the-goal-not-reimplement-poc` | **`:by joseph :status decided`** (2026-07-12) | The charter of your task, verbatim: *"The point of 'get world-building up to the exploration phase' is to prove the NEW declarative system (nomotheke → pull-query → store, composed by the flux web) can reach an eroded/uplifted, explorable world PRINCIPLED-LY — not to re-implement the worldview proof-of-concept (which already makes eroded worlds ad-hoc)."* `#detail-abyssal-parity-build` FE(2) says something adjacent about the testbench but does not cite this row. |
| `debug-poke-then-consolidate` | `:by us :status decided` | Unprincipled pokes while debugging are legitimate; the discipline is the second half — back the poke out and consolidate the gain. *"A poke left standing (as the answer, or as a view/instrument knob) is the failure, not the poke itself."* Its worked example is literally **the globe epochs knob**, backed out and consolidated into the uplift nomos. Directly on point for `--epochs` and for every knob a harness build will want. |
| `cli-world-dir-default-and-symlink-promotion` | `:by claude :status decided` | The world-dir resolution order (`explicit → $VIVARIUM_WORLD → ~/.cache/vivarium/globe-world`, "resolving as the globe does"), and the promotion rule: symlink `~/.local/bin/vivarium → target/release/vivarium`, **not** `cargo install` (so the installed command tracks rebuilds instead of going stale; `~/.cargo/bin` is not on Joseph's mise PATH). Caveat logged: `cargo clean` breaks the symlink. Small, and exactly the sort of thing re-derived worse. |
| `globe-ribbons-are-view-assembly-staleness-not-prior-or-kernel` | `:by claude :status proposed` | Content **is** in core (§3.2) but the slug is uncited, so `grep DECISIONS\[` from core won't find it. Same shape for `epoch-surfaces-are-store-citizens` (content fully in `#form-builder-admission` KI(5), slug uncited). Bookkeeping, but it means the reverse lookup fails in both directions for the two most recent harness decisions. |

### 4.3 Not found anywhere (core, DECISIONS, msc, PROCESS)

- **"Restart at the right points."** The nearest is `#detail-builder-daemon` FE(2) "pause cheap (completed tiles already in store)." Nothing addresses resuming *after an algorithm change* — which is the case Joseph named, and the case the whole-crate digest currently makes impossible by construction (§3.5).
- **Replay as a run mode.** LEXICON pins "replays from pinned generators" as a referent; `#detail-vivium-lifecycle` FE(4) BREAK-4 says publish phase-memos not just seeds because cross-platform FP breaks replay-from-seed. The *deep-time playback* that exists (globe **T** key, `DECISIONS[deep-time-playback-built-…]` `:topic view`, uncited by core) is a different thing — stepping materialized epoch surfaces, not replaying a run. Worth not conflating; the vocabulary collision is live.
- **Demand-spool format.** `#detail-builder-daemon` FE(7) lists it under "Open." Nobody has proposed one.

---

## 5. Adjudication ledger — what was decided not to mint, and whether it held

This is the "don't accidentally re-litigate" table. `msc/agent-briefs/` verdict vs. present tree, verified.

| Brief | Verdict | Held? |
|---|---|---|
| `run-modes-adjudication` | Thin promote **or** honest gap; **do not** promote fat; do not launder plan "DECIDED"; fix store-as-save drift | Gap held (correct). **Drift fix not done** (§3.1). |
| `lazy-query-graph-adjudication` | Promote thin `#form-pull-query-composition`; **keep three-scoped / time-DAG / prefetch out** — "source-only or OUTLINE gaps" | Thin segment landed, FE close to the brief's draft, out-of-bounds list preserved verbatim as FE(6). **Three-scoped was then promoted anyway** — but not sloppily: `#form-three-scoped-runtime` landed at `robust-qualitative` with "no DECISIONS ratification row," an explicit built/unbuilt inventory, and FE(7)(c) forbidding re-absorption into pull-composition. The `thick-middle-residual-census` (next day) had re-rated it "EXTRACT (thin) or gap-keep" as its #2 candidate. **Read this as an escalation with a paper trail, not a violation** — and as evidence these verdicts are 24-hour-perishable, not durable no's. |
| `multirate-coupling-adjudication` | **Kill** `#form-multirate-coupling` as a dual home; optional narrow `#result-scale-separation-directional` | Kill held. The residue landed as `#form-scale-separation-directional` (`formulation`, `robust-qualitative`) rather than the suggested `result`/`exact`-for-Gear–Wells split. FE(2) explicitly records the rejection. Relevant to you only via the near-miss it protects (§3.4). |
| `archive-residual-adjudication` | Promote thin kernel-boundary; **merge** statistic contract into `#form-flux-web` FE(6); **no** phase-machinery segment; agentic-verdict **process-only** | All four held. `#form-kernel-imperative-boundary` is in OUTLINE §II; flux-web FE(6) exists (per the fable audit F10); no phase-machinery segment; `doc/PROCESS.udon` `|norm[agentic-verdict-is-flag]` present with the correct `.super-archive/` source path. Clean. |
| `super-archive-provenance-audit` | No second DESIGN-class truncation; sweep stale `.archive/` pointers; MANIFEST residual row; PHASES pin note | Accepted and executed per `msc/promotion-mine-2026-07-23-continuity.md` (DESIGN byte-match re-verified exact; path sweep done; pin recorded as honest residual). |
| `thick-middle-residual-census` | Leave teaching alone; graduate `multiscale-methods`; banner fixes; 8 extract candidates | Superseded by events: **the entire `doc/design/` + `doc/theory/` middle graduated**, not just the methods zoo. `find doc -type f` returns exactly one file. Its top-8 is now a map of what got extracted, not a to-do. |
| `fable-middle-peel-audit` | Keep six peels as draft; **10 named dual-home / stale-live defects (F1–F9)**; do not status-up until F1/F2/F5 fixed | I could not verify these — F1–F4 all target `doc/theory/*` and `doc/design/*` files that have since graduated to `.super-archive/`. Whether the fixes landed before graduation or the defects were frozen into the ice is **an open question I am not able to close from my slice**. It matters little for you (they are physics/structure dual-homes) but it is a real loose end: `git log` on those paths before `40479ee`..graduation would settle it. |

**Pattern across all seven:** the recurring self-diagnosis is *"core body careful; dual-home demotion incomplete."* For your slice the mirror image is what bit: **core body careful; routing *into* core incomplete for cross-cutting operational facts** (§3.2, §4.2).

---

## 6. Reproduce any of this

```sh
cd ~/src/arch/vivarium

# The DECISIONS-vs-core cross-tab from §Bottom line
for s in $(grep -oE "^\|decision\[[a-z0-9-]+\]" DECISIONS.decision-log.udon | sed 's/|decision\[//;s/\]//'); do
  t=$(grep -m1 "^|decision\[$s\]" DECISIONS.decision-log.udon | grep -oE ":topic [a-z]+" | cut -d' ' -f2)
  grep -rq "DECISIONS\[$s\]" core/src/ && c=CITED || c=-----
  echo "$c $t $s"
done | sort

# Run-modes has no ratifying authority anywhere
grep -in "run-mode\|canon-root\|iterating" DECISIONS.decision-log.udon      # empty
grep -n -A2 "term\[run-modes\]" LEXICON.udon                                # :status open

# The source digest has no decision row of its own
grep -in "source.digest\|SRC_HASH\|source-derived\|whole-crate" DECISIONS.decision-log.udon

# doc/ really is one file
find doc -type f
```

---

## 7. Where I think your framing was slightly off

You asked "is the thought in core." For the prose-peel slices that is the right question. For **my** slice the higher-yield question turned out to be:

**"What did the adjudications decide *not* to mint, and was that verdict priced against a world that no longer exists?"**

Three of the seven verdicts are explicitly conditional — run-modes on a tripwire, the canon-root guard on the same tripwire, three-scoped on "when a session has claim energy." One of those three was overturned inside 24 hours. Yours is the session that supplies both the claim energy and the tripwire. So the durable output of the adjudication layer is not a list of gaps; it's a list of **conditions**, and you should read them as pre-authorizations rather than as walls.

The second reframe: **the peel's scope and Joseph's condition are not the same set.** The campaign was prose → segments. Joseph's condition is "core carries the accumulated workflow/CLI/harness thinking." Those overlap but the second is strictly larger, because a real fraction of that thinking was never prose — it was decided in dialogue and recorded straight into DECISIONS, where the peel had no reason to look. That is not a failure of the peel and not a defect in core; it is a scope mismatch, and it is the reason "yes, it's all in core" would have been the wrong answer to give you.

---

## 8. Confidence, and what I'd want a second pair of eyes on

**High:** the DECISIONS/core cross-tab (mechanical, reproducible); run-modes authority state (three independent greps agree); §3.1 drift still present (read both texts); §3.2 misfiling (read the segment end to end); the five uncited rows in §4.2 (read each in full).

**Medium:** §3.3 — I judged `#detail-builder-daemon` thin without diffing it against the graduated plan; that is the from-plan agent's call, not mine, and I'd weight theirs over mine.

**Low / open:** the fable-middle-peel F1–F9 disposition (§5) — genuinely unresolved from where I sat.

**One thing I could not do:** I did not read the code, deliberately (project law ranks it last for architecture adjudication). Every "unbuilt" in this report is what the segments and DECISIONS assert. You've measured more of the live system today than I have; where we disagree, your measurement wins and the segment is compliance debt.

---

## 9. Feedback on the brief

It was a good brief and I'll name the two things that actually changed my output.

**The disclosure that your own file list had been wrong an hour ago** was the highest-value sentence in it. It gave me explicit license to treat the territory as a starting point, which is why I went into DECISIONS at all — and the DECISIONS cross-tab is the finding I'd keep if I could keep only one. A brief that had drawn the boundary firmly around `msc/` + `audits/` would have produced a much less useful report: my assigned files are, on their own, *clean*, and I would have handed you "yes, it's all in core" with a straight face.

**"A grep won't settle this one — the question is whether a thought survived, not whether a string did"** was correct as a warning and, as it turned out, incomplete as a diagnosis. In this slice the thoughts all survived. What failed was *indexing* — and that failure mode **is** grep-shaped, which is why the mechanical cross-tab found what careful reading hadn't. Worth holding for the other three agents: if they come back with "no thinking lost," ask them separately whether a person with a specific job would *find* it.

**One thing that would have helped:** naming which of the four agents owns `DECISIONS.decision-log.udon`. It's the largest single instrument in the tree (335 KB, 103 rows), it's a `#scope-segment-canon` carve-out so nothing in it was ever obliged to move to core, and it is now demonstrably the place a chunk of your answer was hiding. If nobody was assigned it, the coverage hole was structural rather than accidental — and if two of us swept it, you'll get the same table twice.

**On the "surprise me" invitation** — the one I genuinely didn't expect: `DECISIONS[ascii-globe-in-info-colored-by-state]` is Joseph asking, in July, for a geographic view of *what has been materialized where, in space and in phase*, and marking it a droppable wish. That is a recognizable early sketch of the thing he told you today he'd been waiting for. It has sat at `:status wish` for twelve days with no core home. I'd read it before you design the frontier view — not because it constrains you, but because it tells you what he was picturing.

---

*Available for follow-up: the fable-audit F1–F9 git question, a deeper read on any single segment above, or the thin run-modes FE if you decide the tripwire has fired.*
