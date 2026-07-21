# De novo project audit (B) — canon boundaries, citation integrity, convictability

*2026-07-21. Independent pass over the whole tree by a Claude (Opus 4.8) session at Joseph's request.*

**Independence.** I did not read `audits/2026-07-21-core-segment-floor-audit.md`, `audits/2026-07-21-de-novo-project-audit.md`, or anything under `msc/` before forming these findings — they were written blind to that work, so overlap between the two reports is convergent evidence rather than restatement, and gaps are genuinely separate coverage. I read `audits/README.md` only, for the filing convention.

**Snapshot.** Findings were formed against `3f8754d` and re-verified at `ff89979`. The tree moved four commits during the audit (a parallel session landed `audits/`, ` #ops-audit-integration`, and the clippy determinism bans); items 11 and the segment count reflect `ff89979`. Working files: none retained — every claim below is re-derivable from the commands named in it.

**Method.** Primary sources read directly: all 14 segments, `FORMAT.md`, `core/OUTLINE.md`, `README.md`, `CLAUDE.md`, `ETHICS.md` head, `ASF.md`, `ASSUMPTIONS.md` head, `feedback-from-asf.md`, the `doc/` prose headers, `crates/vivarium-world/src/{spec,nomotheke}.rs`, `crates/vivarium-core/src/lib.rs`, `spikes/worldview/src/main.rs`, the crate manifests, plus `archema-io/charter/INCOHERENCE.md` and `concept-matrix.md`. Executed: `cargo test --workspace`, `cargo clippy -p vivarium-world --lib`, `bin/check`, `vivarium status`, `../asf/bin/lint-md` over the segments and root docs, and mechanical sweeps for cross-reference resolution.

---

## What is sound (verified, not assumed)

Recording this first because a defect list alone misrepresents the state. Each was checked, not inferred.

- **Every `DECISIONS[…]` slug a segment cites exists, with exactly the `:by` and `:status` the segment claims.** All seven checked individually against `DECISIONS.decision-log.udon`. No authority inflation found anywhere in `core/src/`.
- **Cross-references resolve.** 28 of 28 ` #lexicon/…` refs resolve to real entries; 19 of 19 ` #asf/…` refs resolve to real files under `~/src/archema-io/asf` (checked cross-repo, by file). No dangling ` #segment` reference in any canon-live document.
- **14 of 14 segments lint clean** under `../asf/bin/lint-md`.
- **Outline order is consistent with every `depends` edge** — no segment is ordered before one of its prerequisites.
- **`cargo test --workspace`: 112 passed, 0 failed** (87 `vivarium-world` lib + 25 `vivarium-core`).
- **`ETHICS.md` is genuinely path-free** — zero path references, exactly as `3a1c212` claimed. The one document that had to stand alone does.
- **The core/view wall holds where it is checkable in the build graph.** `vivarium-world` and `vivarium-core` each declare zero dependencies; the arrow runs `vivarium-app → vivarium-core` only.
- **`WorldSpec::fresh_seed()`'s `SystemTime::now` is identity minting, not law** — the seed is written to the manifest and everything downstream is pure of it. Consistent with ` #post-determinism-as-ontology` FE(2), and now explicitly `#[allow]`-ed with the reason stated.
- **All 17 declared assumption anchors are present in `ASSUMPTIONS.md`** — the ledger guard passes on real content, not vacuously.
- **The flux web convicts the world, executably, today.** `vivarium status` reports `emerged land` UNMET and states plainly that the world cannot run fluvial erosion. That is ` #norm-declaration-must-convict` working rather than aspiring — the strongest single piece of evidence in the repo that the epistemics are more than prose.
- **` #form-core-view-wall`'s "Known incomplete surface" is accurate.** `spikes/worldview` does read `VIVARIUM_MACRO_EXTRA` / `_FINE_EPOCHS` / `_FINE_PASSES` / `_FILL_CAP` and does run erosion on the explorer path. The disclosure matches the code.
- **`LEXICON.udon` solved the stable-external-citation problem** with `:anchor 7.1` / `7.2` / `7.4` / `7.6` on the terms outside docs cite. `§7.4` resolves to `|term[inhabiting-modes]`. The fix pattern item 1 needs already exists in this repo.

---

## P1 — load-bearing

### 1. A moratorium-clearance claim is anchored to a section that no longer exists

`doc/plan/abyssal-parity-plan.md:11` reads:

> …it is why the milestone is **moratorium-clear**: no endo agent is instantiated, nothing is *governed by* the world, so `ASF.md` §0 is satisfied by construction.

`ASF.md` has no §0. Its headings run §1–§7; the moratorium moved to `ETHICS.md` §0 in `cf4af2c`. So the one sentence in the plan corpus that declares a milestone clear of the project's standing law cites a dead anchor, and a reader following it lands nowhere.

This is worse than an ordinary dangling pointer, because it was recorded as fixed. `DECISIONS.decision-log.udon:912`:

> …the five `ASF.md §0` pointers were repointed to `ETHICS.md` (ordinum, both plan docs, taxonomy-spike ×2).

Checked: `tabularium/terrestris.ordinum.udon` (3 ETHICS refs), `doc/plan/vivium-operational-workflow.md` (1), `doc/plan/regula-conformance-design.md` (1), and `ref/research/taxonomy-formalization-spike.md` (2) all carry ETHICS references. `doc/plan/abyssal-parity-plan.md` carries **zero**, and still carries the `ASF.md §0` citation. Whether the count of five was wrong or one repoint silently failed, the log records a completed action that did not fully fire — and the half that did not fire is the moratorium.

The same DECISIONS entry adds: *"A slug scheme for this file's sections is the real fix and is not done."* That is still true, and `ASF.md:13` warns about it in its own voice. It is the only live document whose external citations are section *numbers*, and the failure it predicted has already happened once. `LEXICON.udon`'s `:anchor` mechanism is the in-repo precedent.

**Fix:** repoint `abyssal-parity-plan.md:11` to `ETHICS.md`'s Standing Moratorium Imperative (or to ` #scope-moratorium-endogenous-emergence`); correct or annotate `DECISIONS:912` so the record stops asserting a completed sweep; give `ASF.md` anchors or slugs.

### 2. The carve-out list in ` #scope-segment-canon` is smaller than the set of claim homes actually in force

FE(3) carves three documents plus executable state: `LEXICON.udon`, `DECISIONS.decision-log.udon`, `ETHICS.md`. Documents outside that list that are load-bearing for project claims right now:

| Document | Why it is a claim home today |
|---|---|
| `README.md` → `ASSUMPTIONS.md` | README's Authority table names it as one of five authorities; the file itself says it "is authoritative for what it records" |
| `FORMAT.md` | Carries method claims (what a claim is, what convicts one, the triage questions). ` #scope-segment-canon` FE(1) grants it conventions, but FE(3) does not carve it |
| `ASF.md` §5 | The Level-C hard gate. ` #scope-moratorium-endogenous-emergence`'s Working Notes call it "binding" — a binding gate whose only home is a numbered section in an uncarved document |
| `tabularium/terrestris.ordinum.udon` | `core/OUTLINE.md` §VII: "Phase structure is codified in [the ordinum]." The charge/promise floor is data, and it is what `vivarium status` adjudicates against |

Under the segment's own FE(2) — a project claim is "a statement about purpose, law, architecture, method, ethics of construction, or what an algorithm asserts" — these are project claims, and FE(3) says such a claim is authoritative *only* as a segment or a named carve-out. So either they get carved or their claims get extracted. As written, `README.md` and ` #scope-segment-canon` disagree about how many authorities this project has, and the segment's `status: exact` is stated more strongly than the tree supports.

I want to be precise about what this is not: it is not the migration-incompleteness the segment already discloses ("the rule does not claim that all true content has already been migrated"). That disclosure covers *unmigrated prose that nobody treats as law*. These four are cited *as* authority — by README, by a segment's Working Notes, by the outline, and by a running binary.

**Fix (strengthening rather than softening):** extend FE(3)'s carve-out list to name what is genuinely carved, with the same one-line justification each existing carve-out gets, and keep `status: exact`. The alternative — down-tiering the sole-source claim — loses a true rule to fix a bookkeeping gap.

---

## P2 — real, moderate

### 3. Nomos version declarations are, by the project's own criterion, wishes

` #norm-declaration-must-convict` FE(1): a declaration "is real only if something in the executable path can fail when the declaration is false or incomplete."

`crates/vivarium-world/src/nomotheke.rs` carries eight hand-maintained version strings (`erosion-2026-07-12b-uplift`, `water-2026-07-10a`, …) that mint the store key. The guard is `declarations_mint_the_keys`, which asserts the key **starts with** a hardcoded expected string. That test fails when you bump the version and forget to update the test; it passes when you change the physics and forget to bump the version. The failure mode it does not cover is the only unsafe one: new physics served under a key that claims to describe the old.

`spikes/worldview/src/main.rs:132` is the same shape, with the hazard written into the comment:

> nomos-version hash: BUMP THIS whenever erosion or water physics changes, or stale [cache is served]

Nothing convicts a forgotten bump in either place. This matters more than ordinary cache staleness because the *in vivia* register rests on it — `ASF.md` §7 item 5 and ` #def-in-vivia` both claim content-addressed storage makes a world-citation **exact**. A citation is exact only if the key is a function of the thing it names, and today it is a function of whether someone remembered.

Honestly credited: ` #def-in-vivia`'s Working Note already flags that "generator-pinning and source-derived nomos versions remain open." The gap this audit adds is the sharper form — the risk is *silent*, and by FE(1) the declaration currently does not convict.

**Direction:** a source-derived version (hash of the kernel's source or its declared parameter set) folded into the key would make the declaration convictable. That is a real design question, not a cleanup, and it is probably the highest-value single mechanism this repo could add.

### 4. The assumptions guard covers the direction that was already honest

`every_assumption_is_in_the_ledger` checks *declared anchor → present in `ASSUMPTIONS.md`*. Three things it does not check:

1. **Code constant → declared.** A magic constant in a nomos that is simply never declared stays invisible. That is precisely the failure the ledger was created for — Joseph's "undiscoverably unlawful."
2. **Ledger row → still declared.** Retired anchors accumulate as rows nothing uses.
3. The check is `LEDGER.contains(a)` — a substring match anywhere in a 22 KB markdown file, including inside a sentence *retiring* the anchor.

The guard is real and worth keeping; the claim in `ASSUMPTIONS.md`'s preamble ("that guard is one of the few things in the repo that actually convicts") is true as stated. The finding is only that its coverage is asymmetric in the direction where the risk lives.

### 5. ` #post-determinism-as-ontology` names one incomplete surface; there are two

The segment's "Known incomplete surface" names agent-layer RNG in older `vivarium-core` paths — verified accurate (`crates/vivarium-core/src/lib.rs:183` steps a shared `World.rng` inside the agent loop, so the stream is order-dependent).

Undisclosed: in `spikes/worldview`, world evolution is paced by wall clock and viewer motion. Under `VIVARIUM_MODE=telescope` the worker runs erosion epochs on a ~0.5 s cadence and re-anchors tiers when the pawn moves past a quarter of a tier's span, so the evolved field is a function of how long you watched and where you walked. The default SETTLE path fixes the epoch counts but then runs water "forever" on the same cadence.

Scope, stated honestly: store roots stay clean — the worldview cache writes to `~/.cache/vivarium/worldview`, not to a canon root — so this does not corrupt any citable vivium. And ` #form-core-view-wall` already discloses the *view-wall* face of this same code. What is missing is that the same code has a *determinism* face, and the postulate's disclosure does not mention it. FE(2) says "Wall-clock time … must not affect world state. Views may use wall-clock for camera feel only," and this is past camera feel.

**Fix:** one clause in ` #post-determinism-as-ontology`'s Known-incomplete-surface paragraph, naming the spike and the containment (cache-only, never a canon root).

### 6. Dependency edges: five uncited, one probably inverted

Five of fourteen segments declare a `depends` entry their body never cites:

| Segment | Uncited dependency |
|---|---|
| ` #disc-algorithms-disguise-physics` | `norm-probes-before-claims` |
| ` #norm-decision-authority` | `scope-segment-canon` |
| ` #norm-declaration-must-convict` | `scope-segment-canon` |
| ` #norm-probes-before-claims` | `norm-declaration-must-convict` |
| ` #ops-audit-integration` | `scope-segment-canon` |

FORMAT §2 requires, for `deps-verified`, that each entry be "genuinely used (not merely 'related')." Nothing has been promoted, so no gate has been jumped — this is the list the dependency audit will produce, delivered early.

One of them is more than bookkeeping. ` #norm-probes-before-claims` (`status: exact`) declares `depends: norm-declaration-must-convict` (`status: robust-qualitative`), while the bodies describe the two as **duals** — ` #norm-declaration-must-convict`'s Discussion: "This is the specification dual of ` #norm-probes-before-claims`: probes convict *behavior*; convictable declarations convict *self-description*." The prose citation runs the opposite direction from the `depends` edge, and duals are peers, not premise and consequent.

The edge as declared also puts an `exact` claim on top of a `robust-qualitative` one. ` #norm-probes-before-claims` is grounded directly in `DECISIONS[probes-before-claims-no-plausibility-as-verification]` (`:by joseph`, decided — verified), which makes it well-typed standing alone. **So the resolution is to drop the edge, not to down-tier the `exact`** — the claim is as strong as it says; the graph was drawing a relation the bodies do not assert.

### 7. The program's incoherence ledger is stale in vivarium's favour

`archema-io/charter/INCOHERENCE.md` row 1 (verified first-hand 2026-07-14) states that vivarium's `ASF.md` is "absent from main tree," that the AAT object-mapping and §7.5 live "only in `.archive/ASF.md`," and that "the Level-C-gate detail has no live home." `archema-io/CLAUDE.md` says `ASF.md` "was dissolved 2026-07-11."

`ASF.md` returned to the live tree on 2026-07-16 (`6b669d3`), with §2 (the conceptual bridge), §5 (the Level A/B/C gates, including the Level-C hard gate), and §7 (the integration program, whose item 5 is the *in vivia* cross-bundling) all present — verified by reading the file. `feedback-from-asf.md` at the repo root, which prioritized re-homing exactly those three items, is therefore substantially answered and has been sitting unacknowledged for five days.

`concept-matrix.md` row 13 cites `ASF.md §2` — resolves. Row 82 cites `ASF.md §7.5` — resolves only if the reader knows §7 is a numbered list and §7.5 means its fifth item; there is no `§7.5` heading. That is item 1's fragility, seen from the program side.

Charter §8: *"incoherence is tolerated exactly when it is tracked; silent divergence is the only prohibited state."* A tracked divergence that was closed five days ago and is still on the open ledger is the ledger failing at its one job — and the close is vivarium's to report.

**Fix (vivarium-side, cheap):** tell the program that row 1's remainder is closed, and reply to `feedback-from-asf.md` where its author will find it.

---

## P3 — hygiene, all verified

8. **`core/OUTLINE.md`'s gap markers fail both the linter and FORMAT.** FORMAT (*File organization*) says a missing segment is a `--GAP--` row; the outline uses `*(#gap) …*` in 16 places. `../asf/bin/lint-md core/OUTLINE.md` reports all 16 ("missing space before # for Obsidian"). `#gap` also reads as a cross-reference to a segment slugged `gap`. It is the only file in `core/` that does not lint clean.

9. **`ASSUMPTIONS.md` fails FORMAT §6 in 17 places** — bare `λ`, `ε`, `θ`, `η` outside `$…$`, where §§5–6 bind every file in the repository. The file's own preamble already books this debt ("the register needs scrubbing"); this is its mechanical half, and it is a `--fix`-scale change.

10. **Live source still points at archived documents.** `crates/vivarium-world/src/{flux.rs:90, uplift.rs:22, climate.rs:23}` cite `TODO.md`; `examples/budget_probe.rs:1`, `examples/source_incision.rs:2`, and `examples/null_space/ops.rs:15` cite `ORIENTATION.md`. Both now live under `.archive/`, and all six citations are bare filenames, so a reader cannot even locate the target. This is FORMAT §5's own worked example ("archiving one file dangled three pointers") recurring in the code layer, which the 2026-07-13 sweep did not reach.

11. **`clippy.toml`'s documented invocation is red.** Its header says: *"Enforced via: `cargo clippy -p vivarium-world --lib -- -D warnings` (or `bin/check`)."* That command exits 101 with nine unrelated style errors — `too_many_arguments` ×5 (`noise.rs:121`, `erosion.rs:726`, `water.rs:404`, `query.rs:291`, `query.rs:317`), `inherent_to_string` (`spec.rs:67`), `useless_format` (`globe.rs:324`), `unnecessary_cast` (`noise.rs:60`), `drop_non_drop` (`water.rs:457`). `bin/check`, which denies only `clippy::disallowed_methods` and `clippy::disallowed_types`, runs green — verified. An agent reading `clippy.toml` will run the first form, meet nine errors, and either "fix" unrelated style in world-law code or quietly stop trusting the gate. Either fix the nine or correct the comment to point only at `bin/check`.

12. **`.archive/` is a live dependency in two places** while `CLAUDE.md` calls it "not tactical current dependency." `tabularium/terrestris.ordinum.udon:7` declares `:reportatio .archive/PHASES.md`; three `LEXICON.udon` entries (`phase-transition`, `defeasance`, `ordinum`) ground their definitions in `.archive/PHASES.md` design notes; `doc/theory/multiscale-methods.md:106` cites it as live support ("`.archive/PHASES.md` design notes say so"). The reportatio relation is defensible — the compile already happened — but a LEXICON definition resting on an archived file is citing archaeology as evidence, and `CLAUDE.md`'s characterization should either acquire that exception or the groundings should move.

13. **A rank claim survives inside a segment.** ` #norm-decision-authority`'s Discussion: *"This norm protects multi-agent continuity more than any single technical claim."* Nothing could convict that comparison. FORMAT §6: *an absolute is a claim with no predicate.* Same class as those struck in `674be86` and `1ffdd65`. The precise form is available and stronger: name what it protects and against what failure.

14. **A self-labelled non-probe in the test suite.** `nomotheke.rs:457 every_promise_makes_a_conservation_claim` — its own comment reads "Vacuously structural (the type forces it)." ` #norm-probes-before-claims` FE(2): "A probe that cannot fail is not a probe." The labelling is honest, so this is not a truth defect; it does inflate the passing-test count by one, and the discipline says delete it or give it a predicate.

15. **Two empty directories under `msc/`** (`research-lem-sota`, `spike-water-structures`), and `msc/spike-wavelet-store/target/` carries build artifacts in-tree.

---

## The one thing I would do first

Not item 1 — that is a five-minute repoint and it should just happen. **Item 3.** Every other finding here is a pointer, a label, or a list that got out of step, and all of them are recoverable by reading. Item 3 is the one place where the project's central external claim — that a result can cite a world *exactly* — depends on a declaration that nothing can convict, in a repo whose distinguishing discipline is that declarations must be convictable. It is also the finding with the most interesting work behind it, and the least likely to be found by anyone who is not looking at the key and the physics at the same time.

## Feedback on the audit itself

Two things I could not do well, so the next auditor knows the shape of the hole:

- **I did not audit the physics.** Whether `erosion.rs`, `water.rs`, and `climate.rs` assert the physical claims they are declared to assert is the question ` #disc-algorithms-disguise-physics` and ` #norm-probes-before-claims` actually care about, and answering it means reading the kernels against the primary literature — a different and larger pass than this one. `doc/theory/discretisation-and-information.md` is where that auditor should start, and `msc/redteam-discretisation/` suggests some of it exists already.
- **The tree moved four commits while I worked.** Findings were re-verified at `ff89979`, but a parallel session was landing work in exactly this area throughout, and items 11 and 14 concern files that were hours old when I read them. Re-check before dispositioning.

I stayed available after filing this and can go deeper on any finding — particularly item 3, where the useful next step is a design conversation rather than a fix.
