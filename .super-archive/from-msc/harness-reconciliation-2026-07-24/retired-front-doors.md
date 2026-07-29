---
scope: .super-archive/from-archive/** (the retired front doors), read against live core/
question: does core carry the workflow / CLI / harness / builder / demand / run-mode / operational thinking from these documents
date: 2026-07-24
status: reconnaissance for a design session — not a claim segment, not canon
---

# Retired front doors — what survived the peel, what thinned, what is gone

Every gap claim below carries a file:line pointer on the archive side and a slug (or a `grep` result) on the core side, so you can convict or acquit it yourself in about a minute each. Where I could not find something, I say what I grepped for — a null result is only as good as its query.

**Headline.** The peel was mostly honest for this slice. The *value laws* came through at full or greater strength. What thinned or vanished is a coherent subset: **the scheduling-and-legibility layer** — everything about time, progress, addressability of intermediate stages, and what an observer is looking at. That is the exact slice your task needs, and I do not think that is a coincidence. See §4 for why, and for the one place I think your framing could be sharpened.

Six things I could not find anywhere (§1). Ten present-but-weakened (§2). A spot-check list of things that are fine so you don't re-verify them (§3). Four incidental findings (§5), one of which is a live cross-segment inconsistency in `#form-in-vivia-citation`.

---

## 1. Nowhere — not in `core/`, not in `LEXICON.udon`, not in `DECISIONS`

Ordered by how much I think each one bites the thing you are about to build.

### 1.1 Component E — time-indexed stage chains ⟵ *the load-bearing one*

**Archive:** `.super-archive/from-archive/TODO.md:92–94` (whole section, three sentences long, worth reading verbatim).

The content: the settle sequence expressed as **explicit chained nomoi with time-indices, each stage a memo carrying its own recorded convergence-$\varepsilon$**. The section then gives four reasons it keeps resurfacing as load-bearing, and every one of them is your brief:

> "it is what makes 'the beginning of Phase 3.3' **addressable** (beacons need it), what makes intermediate stages **monitorable by explorers**, what the '**watch erosion happen while floating downstream**' experience plays back, and its recorded $\varepsilon$ *is* the unLawfulness budget (LEXICON §7.2) as data."

Plus its provenance, which is itself a finding: it was `framework-to-status-quo.md` §4 component **E**, it **did not carry into `abyssal-parity-plan.md`'s six phases, and no deliberate deferral was recorded** — "it fell through the consolidation crack (found 2026-07-10)." Recommendation attached: treat as plan-Phase-2½, build when the builder daemon's stage progression first needs a second time-index.

**Core:** the *name* survives in three places as a pointer with no content behind it —

- `#detail-builder-daemon` FE(5) "Temporal ladder (stage chains)" — but that is the *rung ladder* (macro-erosion epochs → macro water-cycle → human-scale live water), not the time-index / $\varepsilon$-memo mechanism;
- `#disc-unlawfulness-budget` "Not claimed" — "a specific $\varepsilon$ schema in the store (component E / stage-chains still unbuilt)", and Working Notes lists "component-E stage chains" as a sibling;
- `LEXICON.udon:279` — the declared-vs-derived entry ends "Landing order: declaration with the nomotheke (live since 2026-07-10); derivation with **component E**."

`grep -ril "component" core/src/` → one hit (`disc-unlawfulness-budget.md`). **Nothing in core says what component E is, or that it is what makes an intermediate stage addressable and monitorable.** Three segments cite a thing whose definition was dropped.

Worse for findability: there is **no `#gap` row for it** in `core/OUTLINE.md` (I listed all nine gap rows — none matches), so `#disc-open-problem-census`, whose entire method is deriving from OUTLINE gaps plus segment open-residues, is structurally blind to it. By FORMAT.md's own rule, this is "an unnamed [absence] nobody can see."

### 1.2 Explorer intents — especially "legible, steerable time regimes"

**Archive:** `.super-archive/from-archive/TODO.md:173–175`. Twelve intents, attributed to **Joseph, 2026-07-02**, prefaced: *"they are intents about experiencing the world and bind the frame-native explorer's design."* The last one:

> "**legible, steerable time regimes** (in-world clock vs wall clock, sim rate vs framerate — pre-history vs current sim)"

That sentence is the direct antecedent of "watch it in real-time at the 'edge', or watching replays."

**Core:** `grep -ril` in `core/src/` for `"wall clock"` → 0; `"sim rate"` → 0; `"scrub"` → 0; `"time regime"` → 0; `"in-world clock"` → 0.

What core has that is *adjacent but a different axis*: `#form-temporal-lod-regimes` names four **materialization** regimes (G/F/H/E) — that is "what machinery does this aspect need," not "what clock is the observer on and can they steer it." `#detail-vivium-lifecycle` FE(3) has the realizability gate (~2 Hz where a human is the clock). `#form-fidelity-invariant` FE(1) has the temporal axis of the invariant. None of them make the observer's clock legible or steerable, and none of them is about replay.

Also gone: the globe's *actual working* time-scrub UX (`ORIENTATION.md:200–201` — `,`/`.` hour · `N`/`M` day · `P` play · `Y` headlight, marked "verified against `spikes/globe/src/main.rs`"). That is a built replay control surface with no claim home.

The other eleven intents (sediment loop, nested water telescope, HUD clarity, water transparency, pawn under deep water, WASD precision, ground-vs-column-state visualization, wet-ground slumping, …) are partly re-derivable from the physics segments, but as *explorer design intents* they are gone.

### 1.3 `manifest-template` — and the line-to-hold that came with it

**Archive:** `.super-archive/from-archive/VIVARIA-DEFINITIONS.md:280–300` (the artifact) and `:388` (the constraint).

Two pieces, and the second is the one I would not want lost:

1. The artifact — a named, reusable, citable prescriptive profile many worlds share (`:order`, `:rigor-floors`, `:permits`, `:posture`; then `|vivium[…] :extends rigorous-earth-sim`). Called out as *"the ONE thing regula did that isn't yet real."* Deliberately given a plain IT-register placeholder name rather than a Latin coinage.
2. The constraint that survives the regula collapse — verbatim: *"the order stays cleanly descriptive only if **rigor-floors live on the manifest/template, not baked into the order**. The moment someone writes a rigor floor into the ordinum itself, the description/policy separation is gone. **That's the line to hold if we collapse.**"*

**Core:** `#form-manifest-prescribes-vivium` owns the two-artifact collapse well (FE(1)–(3), `exact`, Joseph `:by us`), including "no middle regula object until a genuine awkwardness earns a new noun." It does **not** carry the rigor-floor placement rule, and it does not name `manifest-template` as the deferred third layer with its snap-in seam already drawn. `grep -ril "rigor floor"` / `"rigor-floors"` / `"manifest-template"` in `core/src/` → 0, 0, 0. `#detail-regula-design` FE(9) has "posture overreach (fold wins)" but no floor-placement rule.

**Why this bites you specifically:** the moment a CLI or manifest grows "build me a `rigorous-earth-sim` to phase 4," you are building the manifest-template layer. Its one durable constraint was dropped, and the failure it guards against (a rigor floor drifting into the ordinum) is invisible until the ordinum stops being descriptive.

### 1.4 Declared-vs-audited tier (`:physics` beside `:physics-audited`)

**Archive:** `.super-archive/from-archive/VIVARIA-DECLARATIVE-FRONTIER.md:46–73`. The doc's own framing: *"the strongest single move in the pass: it makes the epistemic tags — the whole basis of the fidelity pyramid — auditable rather than trusted."* Mechanism: `fn audit_tiers(nomos, kernel_src) -> {declared, assessed, discrepancy}`; `DECLARED > AUDITED ⇒ overclaim flagged`; the honesty column shows a `*` and the reason.

**Core:** `grep -ril "declared-vs-audited"` → 0; `"physics-audited"` → 0.

What core has is a *different* axis: `#form-nomotheke-registry` + `LEXICON.udon:279` carry **declared** (self-asserted on the nomos-version) and **derived** (weakest-link fold down the cone). "Audited" is a third strand — an auditor read the kernel against the claim — and it is absent.

**Caveat I owe you, because it may be deliberate:** the same source document is the one that fabricated a verdict and got caught, and it carries the hard constraint that an agentic verdict "may **raise** a probe and **never substitute** for one" and "may never enter the content-addressed store." `#form-kernel-imperative-boundary` Working Notes says exactly: *"Do not re-import five-layer scaffold, kernel-as-DSL, or agentic 'assessed' as store truth."* So agentic-as-store-truth was refused **on purpose**. But a declared-vs-audited *discrepancy flag* is a report, not a keyed store citizen, and I found no record of that distinction being adjudicated. My read: **absent, possibly by conflation with the thing that was correctly refused.** Worth 5 minutes of your judgment, not mine.

### 1.5 Declared hypothesis + fitness blocks — the third verification mode

**Archive:** `.super-archive/from-archive/VIVARIA-DECLARATIVE-FRONTIER.md:156–190`, attributed to **Joseph, 2026-07-12**.

```
|hypothesis[drainage-self-organizes]
  :emergent "channel networks self-organize to Hack's law + fractal drainage density"
  :grounds  "Rigon 1996; Rodriguez-Iturbe & Rinaldo — observed on real DEMs"
  |fitness[hacks-law]
    :measure "fit L = a * A^h over the drainage graph; report h, R^2"
    :expect  "h in [0.55, 0.60]"          ; declared prior, with a source
    :pass    "h in [0.55,0.60] and R^2 > 0.95"
    :on-fail "erosion's B-fidelity-for-drainage is REFUTED (a declared falsification, not a vibe)"
```

and the closing taxonomy: **(1) static/agentic · (2) invariant probe · (3) fitness vs a declared, sourced prior** — plus "even pre-run, an agent audits the hypothesis's WELL-POSEDNESS: is `:expect` grounded in `:grounds`, or a guess dressed as a prior? **An ungrounded `:expect` is itself an audit finding.**"

**Core:** `grep -ril "fitness"` in `core/src/` → 0.

Mode (2) is superbly covered (`#norm-probes-before-claims`, `#norm-probe-sensitivity`, `#norm-regime-probes`). Mode (3) has no home. Partial survivors of the fourth mode the archived outline later added (**control**) exist as *instances* — `#obs-cube-locked-kernel-bias` FE(5) "Mandatory cube control", `#obs-mean-pin-manufactures-seam` FE(4) "Zero-physics control" — but not as a declared mode.

**Why it bites:** "a principled understanding of what exactly they are watching" *is* mode (3). Watching an emergent landscape form means nothing without a declared, sourced expectation it can fail against. That is the whole difference between a screensaver and an instrument, and the machinery for it was designed and then dropped.

### 1.6 Worlds have genealogy — a control is a sibling world

**Archive:** `.super-archive/from-archive/core/OUTLINE.md:392` (`#der-worlds-have-genealogy`, in the retired Jul-13 outline):

> "A world is a pure function of (ante-mundane params, nomoi, seed) ⇒ two worlds differing in **one law** are **SIBLINGS**, and the store localizes their divergence **exactly** ⇒ a **CONTROL is a sibling world** — the instrument for in-vivia counterfactuals already exists."

**Core + LEXICON + DECISIONS:** `grep -rn "sibling world\|genealogy\|counterfactual"` → 0 across all three.

Every ingredient is in core separately: `#form-complete-content-addressed-key` Discussion ("change a nomos version and exactly its dependent cone invalidates; everything else stays"), `#form-depend-by-key-never-latest` FE(1) (build-order independence, byte-identical wherever both materialized), `#detail-vivium-lifecycle` FE(5) (fork-DAG, fork from last memo not seed). The **derivation** — that these compose into a free counterfactual instrument, and that a control run *is* a sibling world — is gone.

**Why it bites:** this is your "restarting at the right points as we iterate on the algorithms" claim in its strongest available form, and it is the one that tells you *what the right points are*: they are the boundary of the invalidated cone, and the two worlds either side of an algorithm change are siblings whose divergence the store already localizes exactly.

---

## 2. Present but weakened, partial, or subtly changed

You said this category is where you expect misjudgement in both directions. I have tried to say precisely what is thinner and why it matters, rather than flagging compression as loss.

### 2.1 The round-trip probe — a law asserted without its convictor

**Archive:** `.super-archive/from-archive/architecture-migration-2026-07-03.md:40–41`, under "Soon":

> "**Checkpoint round-trip probe** (resume vs run-through must agree — promote the two-leg cache test from anecdote to instrument)."

**Core:** `#form-depend-by-key-never-latest` FE(1) states the *law* — two builds along different demand orders converge byte-identical wherever both materialized — at `status: exact`. `grep "round-trip"` in core hits only `#form-store-as-save` FE(6), in a different sense (RAM↔store round-trip cost). **There is no named probe that a resumed build equals a straight-through build.**

Under `#norm-probes-before-claims` FE(1) ("not assertable as established until a probe exists that would fail if the claim were false"), an `exact` behavioral law with no named convictor is a live tension inside core's own norms. This is my sharpest middle-category finding, and it sits directly under "restarting at the right points": that is the claim, and the instrument that would convict it was named in the archive and not carried.

### 2.2 Run-modes carve — honestly gapped, but the gap row drops the ethical tripwire

Core is honest here: `core/OUTLINE.md:110` gap row, `#form-store-as-save` FE(8) (root discipline as frame) + Known-incomplete(2) ("No GC, no full manifest, no run-mode enforcement yet"), `LEXICON.udon:201` (`run-modes :status open`, referents pinned).

What the archived `#open-run-mode-guard-does-not-exist` (`.super-archive/from-archive/core/OUTLINE.md:390`) carried and the live gap does **not** is the **conjunction**:

> "`vivium-operational-workflow` doctrine #7 — *'never discard a memo that has ever hosted a mourning-capable mind'* — has no mechanism. **It holds today only because there is no GC at all.** The moment eviction lands, an ETHICAL invariant needs an implementation that does not exist and is not designed."

Core has both halves, separately and correctly: `#detail-vivium-lifecycle` FE(7) doctrine 7 states the obligation; `#form-store-as-save` FE(5) distinguishes invalidation from eviction and Known-incomplete(2) says no GC. Nobody joins them. A background daemon plus a monotonically growing store is exactly the situation that makes someone reach for GC — which is why I am flagging it to you rather than leaving it.

(The *other* tripwire from this family did survive, and well: `#detail-builder-daemon` FE(4) — "two-process store sharing may force mechanism-enforced canon-root guard earlier than first graduation.")

### 2.3 Execution classes — names survive, scheduling semantics thinned

**Archive:** `.super-archive/from-archive/architecture-migration-2026-07-03.md:53–60`, headed "EXECUTION CLASSES (Joseph, same evening — the principle that ties it together)". Each class gets an operational definition, not just a name:

- **batch-deep / checkpoint-bound** — "run long, **preemptively**, RARELY (re-trigger = recipe version change); **arbitrarily slow because checkpoints amortize to zero**." Exemplars: tectonics, banding, igneous bodies, uplift.
- **relaxation** — "solved/settled per checkpoint, then locally live" (water).
- **procedural-tight / call-site-bound** — "evaluated constantly, must be fast, closed-form or surrogate" (insolation is the exemplar; weather aspires to it).

And the symptom list: *"The 2026-07-03 symptoms of NOT having this: four cache invalidations in a day, the eternal-fill UX, warmer choreography by hand."*

**Core:** `#form-fidelity-ladder` FE(5) and `#form-add-system-contract` FE(3) both carry the three names and the *coupling* purpose ("so coupling treats it quasi-static or time-averaged"). Neither carries the **scheduling** semantics — run preemptively, rarely, arbitrarily slow because memoization amortizes the cost to zero — which is precisely the law a background builder would be built on. The symptom list (the evidence for why the class is load-bearing) is gone.

The fields *are* live: `NomosDecl::execution` / `Timescale` landed 2026-07-24 (`#form-kernel-imperative-boundary` FE(5)). So the mechanism is ahead of the claim here, not behind it.

### 2.4 Watchpoints — the weakest leg of the beacon/focus/watchpoint trio

`beacon` and `focus` both have full `LEXICON.udon` entries (`:141` and `:136`), with beacon explicitly carved against focus ("focus is live and transient; a beacon is standing and spec-persisted") and beacon carrying the order-independence guarantee. **`grep -c watchpoint LEXICON.udon` → 0.** Watchpoint exists only inside `#detail-builder-daemon` FE(2) and FE(4), glossed once as "declared place/level/stage snapshots."

Watchpoints are the observer-facing member of the trio — they are what "watching it at the edge" and "watching replays" would actually be built on. It is the one that got the least definition.

### 2.5 The CLI surface has no claim home at all

**Archive:** `.super-archive/from-archive/README.root.md:79–117` and `ORIENTATION.md:151–157` carry the whole operational surface —

- the verb set `new` / `build` / `status` / `info` / `attach`, each with its one-line contract (`info` = whole-sphere Hammer equal-area oval coloured by build-state; `attach` = follow a running build's log);
- builder v0 sweeps under `builder.lock`, **a second `build` ATTACHES rather than failing**;
- world-dir resolution, stated as a *convention with a reason*: explicit `dir` → `$VIVARIUM_WORLD` → `${XDG_CACHE_HOME:-~/.cache}/vivarium/globe-world`, and the deliberate negative — *"It never scans for 'the only world' — #3 is a fixed convention"*;
- the PATH-symlink-to-release-build workflow, with its two named caveats (tracks the *release* artifact only; `cargo clean` dangles the symlink).

**Core:** `#form-builder-admission` FE(3) names `--allow-unmet`; `#form-core-view-wall` Working Notes names `$VIVARIUM_WORLD`; `#detail-builder-daemon` FE(6) lists "CLI build/status/attach" as an ordering item. **The verb set, the resolution order, the never-scan rule, and the attach semantics are claimed nowhere.** Live `README.md:37–47` carries a quickstart with `new`/`build`/`status` and the default dir, but not `info`/`attach`, not the resolution order as a rule, not the never-scan negative, not the symlink workflow.

Related and also unlanded: `TODO.md:96–98` "**Small UX: unify default world dirs**" — CLI takes explicit `<dir>`, globe defaults to `~/.cache/vivarium/globe-world`, `store_explore` uses `$TMPDIR/vivarium-store-explore`; "a fresh user running all three sees three different worlds"; proposed convention `~/.local/share/vivarium/<name>` + `VIVARIUM_WORLD` honoured everywhere; and the caution **"Don't silently move Joseph's existing `globe-world` — first-light lives there."** Two thirds resolved in practice, `store_explore` still divergent, never claimed.

I flag this as *weakened* rather than *gone* because it is arguably README-grade rather than claim-grade. But you are about to change it, and `#scope-segment-canon` FE(2) says READMEs are not claim homes — so today there is no artifact that a CLI change could contradict.

### 2.6 Analysis nomos / result-memos — survives, filed where a harness designer will not look

**Archive:** `VIVARIA-DEFINITIONS.md:127–129` — "**ANALYSIS nomos** — reads world state + an analysis, emits a RESULT-memo. **Its cone IS its provenance**; a flaw upstream flags it by the store's own invalidation."

**Core:** survives in substance as `#detail-regula-design` FE(4): *"In-vivia results as memos. Result = query + analysis nomos with complete dependency cone; invalidation flags touched results; hash compare flags changed results."* Also FE(3) preserves `vivarium audit --add` as the requisite-closure planning query (the "anti-whim planner").

So: **present, not lost.** The problem is placement. `#detail-regula-design` is `discussion-grade`, sits in §IV (Kingdoms/orders/ordinum), and is about a **largely unbuilt regula**. Nothing in §III (Runtime, machine, and CLI) points at it. If you are designing "principled understanding of what they are watching," the mechanism you want is the analysis-nomos-with-a-cone, and it is two sections away under a header that reads as retired.

(The PROJECTION nomos kind survives as `LEXICON.udon:186` `native-representation`. The three-kinds taxonomy as a taxonomy is gone; I think that is fine — it was flagged as provisional at source.)

### 2.7 Pre-run study validity (`assess_study`) — ingredients kept, gate dropped

**Archive:** `VIVARIA-DECLARATIVE-FRONTIER.md:105–129`. The worked specimen is sharp: a land-fraction claim against a `physics:none / earth-fidelity:none` prior is **ill-posed before it runs** — "NO land-fraction result from this world can support or refute an Earth claim." The move: *"you can pre-audit whether a run could possibly mean what it claims."*

**Core:** `#def-in-vivia` + `#form-in-vivia-citation` FE(3) + `#form-nomotheke-registry`'s weakest-link fold contain all the ingredients. What is absent is the **pre-run gate** as a named check you run *before spending the build*. That is a scheduling concern as much as an epistemic one — it is the difference between a 40-minute L12 build that can mean something and one that cannot.

### 2.8 The declarative-frontier failure record — four of five carried

`VIVARIA-DECLARATIVE-FRONTIER.md:204–214` lists five rejected attempts. Four survive somewhere:

| rejected attempt | survives at |
|---|---|
| kernel-as-DSL | `#form-kernel-imperative-boundary` FE(2) + Working Notes |
| all-static "kept" (drop probes) | `#norm-probes-before-claims`, `#norm-regime-probes` |
| standalone per-coupling policy object | `#form-flux-web` Discussion (arrived at independently, via per-quantity granularity vs "monolithic flux blob") |
| posture as a first-class field | `#detail-regula-design` FE(9) "posture overreach (fold wins)" |
| **a global "confidence" scalar per world** | **nowhere** |

The last one: *"Rejected: it collapses independent axes (A/B/C, per-region, per-aspect) into a lie. The honest object is the per-promise, per-cone fold, not a scalar."* That is directly a status/HUD design rule, and a live "watch the build" surface is exactly what invites one trustworthiness number for the world.

### 2.9 Builder-daemon design — compressed well, one cross-territory note

`#detail-builder-daemon` carries all six named pieces from `ORIENTATION.md:159–163` (daemon / read-only explorers / demand spool / beacons / watchpoints / fidelity pyramid) plus the depend-by-key invariant, the store-as-bus and benign-race-by-construction argument, the four-piece decomposition, per-beacon depth-first-vs-breadth-first policy, and the access-profiles-as-process-boundaries claim. From *my* territory I judge this a real compression, not a loss.

**Cross-territory:** the fuller source is `.super-archive/from-plan/builder-explorer-decoupling.md`, which is another agent's slice — whoever has `from-plan/` should be the one to say whether the segment lost anything there. I deliberately did not read it, to keep this an independent read.

### 2.10 "The ladder runs both ways" — carried, and *strengthened*

For balance: `architecture-migration-2026-07-03.md:62–68` (climb to discover, descend to a tight procedural surrogate, keep the expensive rung as calibrator) lands intact at `#form-fidelity-ladder` FE(2)(3), and FE(6) **adds** a clause the source did not have — "scaffolding has a demolition date" (explicit state that parameterizes what a finer rung would emerge is declared as scaffolding and retires when the rung lands; keeping both is a fork). This is the peel working.

---

## 3. Spot-checked and fine — do not re-verify these

I checked these against core so you can skip them:

| from the retired front doors | lands at |
|---|---|
| Pervasive disk memoization directive; "key, never caution"; never manual cache-clear | `#form-complete-content-addressed-key` FE(3) — verbatim strength, Joseph 2026-07-09 |
| Nomos versions auto-derived from kernel source | FE(4) + `#detail-epistemics-toolchain` FE(5) — **wired**, with probes |
| Depend by key never "latest available"; build-order independence | own segment, `exact` |
| Store shape (objects/roots/mutations), save ≡ memo store, invalidation vs eviction | `#form-store-as-save` FE(2)(4)(5) |
| 64-bit FNV not collision-safe; no GC; no run-mode enforcement | Known-incomplete blocks in both store segments — honest, unsoftened |
| Complete-key-is-unenforced (archived `#open-complete-key-is-UNENFORCED`) | strengthened: the build-time whole-crate source digest closed the forgotten-bump path |
| Flux statistic + exactness contract (`needs mean at-least L19`) | `#form-flux-web` FE(6) — now **mechanized**, strictly stronger than the spike |
| Ordinum governs the flux web; unmet ⇒ mechanically unrunnable; maturity report | `#form-ordinum-governs-flux-web`, `#form-flux-web` FE(3)(5), `#form-builder-admission` |
| Domain-fixation guard; four representation kinds | `#form-add-system-contract` FE(8) |
| BREAK-1…5, capability ladder, the eight standing doctrines | `#detail-vivium-lifecycle` — complete, including "publish memos not just seeds" and fork-from-last-memo |
| Realized ⟂ Lawful; unLawfulness budget; completion-gate retired | `#disc-unlawfulness-budget` |
| Prime Question; bias-vs-noise; probe-that-cannot-fail; check-the-ladder; authority-is-not-evidence | all have segments (`#disc-prime-question`, `#norm-bias-vs-noise`, `#norm-probes-before-claims` FE(2), `#disc-check-the-ladder`, `#norm-decision-authority`) |
| "Build the instrument before tuning by feel" (the godot-perf lesson) | `#norm-probes-before-claims` Discussion |
| Store-as-bus, benign race, lockfile-attach | `#detail-builder-daemon` FE(1)(2) |
| Never-block first light | `#form-builder-admission` Known-incomplete(5) — both halves landed 2026-07-24, with timings |

---

## 4. Where I think your framing could be sharper

You asked to hear it if the question is wrong. I do not think it is wrong, but I think there is a sharper version of it, and finding it changed how I read the corpus.

**The peel was not random about what it kept.** Sorting my findings by *kind* rather than by verdict:

- **Kept at full or greater strength: every law about what a value *is*.** Complete keys, determinism, admission, flux matching, store ontology, order-independence. Every one of these got a segment, most at `status: exact`.
- **Kept, compressed: every law about *roles*.** Builder/explorer, core/view wall, peer views, access profiles as process boundaries.
- **Thinned or dropped: nearly everything about *time, progress, and the observer*.** Component E (stage addressability), explorer intents (steerable time regimes), watchpoint's definition, the round-trip probe, sibling-world genealogy, fitness-against-a-prior, execution-class *scheduling* semantics, the global-confidence-scalar rejection.

That is one coherent layer — call it the **scheduling-and-legibility layer** — and it is exactly the layer your task lives in.

I think the cause is structural, not careless. `FORMAT.md` §1's segment ontology is **claim-shaped**, adopted from ASF: postulate / definition / formulation / observation / result. The scheduling-and-legibility material is mostly *intent* and *mechanism*, which have no natural type in that vocabulary. And this is not my inference — `FORMAT.md` Open Question 1 asks precisely this: *"Does a specification need kinds a theory does not? A nomos has contracts … and there is no ASF word for that. Same for whatever convicts a claim."* The archived Jul-13 outline had coined exactly the missing types — `contract`, **`mode`** (for agentic/probe/fitness/control), `mechanism`, `open` — and they were dropped in the ASF alignment (`.super-archive/from-archive/core/OUTLINE.md:194–198` records that decision and its collision-check gap). **The material typed by them thinned along with them.** Six of my ten weakened items and three of my six absences would have been `mode` or `mechanism` segments.

**So the sharper question might be: not "is it in core," but "does core have a place to put what I am about to design?"**

Today I would say no, not comfortably. `core/OUTLINE.md` §III is titled "Runtime, machine, and CLI," and every row in it is a value-law. The two homes that exist for scheduling/legibility material — `#detail-builder-daemon` and `#detail-vivium-lifecycle` — are both `detail`, both `discussion-grade`, and both explicitly framed as *unbuilt design*. A demand/beacon/frontier/watchpoint/replay layer landing there inherits "unbuilt design" as its epistemic register the day it ships.

That is not an argument for coining a type before thinking (FORMAT is clear that it is not resolved by picking a word quickly). It is an argument that **the placement question is upstream of your design**, and that if you land the design into `#detail-builder-daemon` by default, you will have re-created the shape that let this material thin the first time.

*This section is my read from one corpus slice, not a finding. It is the thing I would most want a second opinion on.*

---

## 5. Incidental

### 5.1 `#form-in-vivia-citation` carries a stale gap — and it is in the segment where it matters most

`#form-in-vivia-citation` Epistemic Status item 2: *"Hand-stamped nomos versions remain present practice; source-derived versions are the target remedy."*

But `#form-complete-content-addressed-key` FE(4) and `#detail-epistemics-toolchain` FE(5) both say the build-time whole-crate source digest is **wired**, folded into every key stem by `NomosDecl::key`, and probed two ways (`source_hash.rs` change-sensitivity; `every_nomos_key_folds_the_source_hash` and `injected_source_hash_matches_live_source`). The same stale phrasing is in `#form-add-system-contract` FE(4) ("hand-stamped today; source-derived is the target").

Two segments describe as *future* a remedy a third describes as *landed*, and one of the two is the citation segment — the one whose whole job is stating what makes a citation exact. Cheap to fix, and it is the kind of drift that gets quoted outward.

### 5.2 Your whole-store-invalidation experiment reproduces a documented, deliberate choice — with the ceiling condition already named

Worth reading before you design around it. `#form-complete-content-addressed-key` Known-incomplete(3): *"editing any `.rs` (including a comment or a test) invalidates the whole store — over-keying, the safe direction."* And FE(4): *"whole-crate covers transitive in-crate deps with no hand-maintained source→nomos map."*

The ceiling condition is stated as a **derivability** constraint, not a backlog item: *"any hand-maintained source→nomos map would reintroduce the bump-discipline this removes, so the ceiling waits until **the module graph makes attribution derivable**."* Anything you design for "restart at the right points" has to beat that bar — it is not enough to be finer-grained; it has to be finer-grained *without* a hand-maintained map. `#detail-epistemics-toolchain` FE(5) is the shortest full statement.

### 5.3 The census cannot see three of these

`#disc-open-problem-census` derives strictly from OUTLINE `#gap` rows plus segment open-residues. I enumerated all nine gap rows in `core/OUTLINE.md`: **none covers component E, the explorer intents, or the round-trip probe.** If you land nothing else from this document, adding gap rows for them makes them visible to the census habit — which is the mechanism core built for exactly this, and it is currently blind to them.

### 5.4 One small structural asymmetry

`watchpoint` has no LEXICON entry while `beacon` and `focus` both do, and beacon's entry carves explicitly against focus. Of the three, watchpoint is the observer-facing one — and it is the undefined one.

---

## 6. Method and limits

- Read fully: live `CLAUDE.md`, `core/OUTLINE.md`, `FORMAT.md`; and in `.super-archive/from-archive/`: `ORIENTATION.md`, `HANDOFF.md`, `TODO.md`, `CLAUDE.md`, `README.root.md`, `core/OUTLINE.md`, `VIVARIA-DECLARATIVE-FRONTIER.md`, `VIVARIA-DEFINITIONS.md`, `architecture-migration-2026-07-03.md`, `memory-surfaced-2026-07-13.md`, `lexicon-notes-and-scratch.md`, plus `PHASES.md` design notes (§190–300) and targeted passes over `taxonomy-bdd-stress-test.md`.
- Read fully on the core side: 22 segments — `detail-builder-daemon`, `form-builder-admission`, `form-store-as-save`, `form-three-scoped-runtime`, `form-manifest-prescribes-vivium`, `detail-vivium-lifecycle`, `form-core-view-wall`, `detail-epistemics-toolchain`, `form-complete-content-addressed-key`, `form-pull-query-composition`, `form-depend-by-key-never-latest`, `form-kernel-imperative-boundary`, `form-flux-web`, `form-fidelity-ladder`, `form-add-system-contract`, `form-in-vivia-citation`, `detail-regula-design`, `detail-abyssal-parity-build`, `disc-unlawfulness-budget`, `disc-open-problem-census`, `scope-segment-canon`, `norm-declaration-must-convict`, `norm-probes-before-claims`, `norm-probe-sensitivity`, `form-temporal-lod-regimes`.
- Every "nowhere" claim was a `grep -ril` over `core/src/` plus, where the concept could plausibly be a term or a decision, over `LEXICON.udon` and `DECISIONS.decision-log.udon`. The queries are quoted inline so you can judge whether I asked the right question. A concept present under vocabulary I did not guess would read as absent to me — that is the residual risk, and it is why I tried to state each finding as a *thought* plus its archive line rather than as a string.
- **Not read, by coordination:** `.super-archive/from-plan/**` (notably `builder-explorer-decoupling.md`, `vivium-operational-workflow.md`, `framework-to-status-quo.md` — the last is component E's origin, so §1.1 is likely to be sharpened by whoever holds that slice), `from-doc/toolchain.md`, `from-design/**`, `from-msc/**`, `msc/agent-briefs/**`. I also did not read live code; per `CLAUDE.md` rule 7 that is not the adjudicator here.
- I did not edit anything outside this file and its directory.

**Confidence.** High on §1.1, §1.2, §1.5, §1.6 and §2.1 (clean archive text, clean null greps, and I checked several plausible synonyms for each). Medium on §1.4 — the absence is verified but the *reason* may be a deliberate refusal I am reading as an oversight. Medium on §2.5 — the absence is real but the claim-grade-vs-README-grade judgment is yours. §4 is a read, not a finding.
