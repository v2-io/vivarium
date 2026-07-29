---
scope: how decisions of various kinds should be represented in and around `core/` — historically and going forward
reader: Joseph (the structural calls are his); the harness-reconciliation coordinator first
status: recommendation — not a claim segment, not canon, not a decision
date: 2026-07-24
---

# Representing decisions in `core/` — a recommendation

**The short answer: the carve-out is right, the schema is right, and nothing about `core/` needs to change. What is broken is smaller and more embarrassing than a structure problem — `DECISIONS` is a write-only channel.** Every property of the file is tuned for cheap append, deliberately and successfully: 101 substantial entries in twelve days, concurrent-safe, not one garbled block. Nothing is tuned for read-back. There is no defined citation form, no linter, no back-index, no projection, and no occasion on which anyone is expected to read it. The invisibility Joseph hit on 2026-07-24 — re-asking for a feature he had specified twelve days earlier — is that single property, not a taxonomy failure.

So the recommendation is a norm, a one-row addition to a table `FORMAT.md` already has, a ~50-line read-only projection, and a list of typo-class repairs. It is **not** a schema for decisions. Where I think a schema change would be a bad trade, I say so and say why.

The one place I do recommend something structural is the opposite of adding: **four of the eleven species I found should carry no `core/` citation at all**, and the obvious reading of the audit finding — "make core cite DECISIONS" — is wrong for about 40% of the file and would damage the segments if executed.

---

## 1. What I measured

All of this is mechanical and reproducible; where a judgment is mine I mark it.

```sh
cd ~/src/arch/vivarium
# entry census (header-line parse — the authoritative one; a bare grep of ':status'
# over the whole file overcounts, because status words appear in body prose too)
grep -E '^\|decision\[' DECISIONS.decision-log.udon \
  | sed -E 's/^\|decision\[([^]]+)\] :date ([0-9-]+) :by ([a-z]+) :status ([a-z-]+) :topic ([a-z-]+).*/\1\t\2\t\3\t\4\t\5/'
# citation cross-tab
grep -rhoE 'DECISIONS\[[^]]+\]' core/ | sort -u
```

**Census.** 101 entries. Topics: `physics` 48 · `architecture` 24 · `process` 16 · `naming` 7 · `view` 3 · `tooling` 2 · `ontology` 1. Authority: `claude` 67 · `joseph` 19 · `us` 15. Status: `council-accepted` 37 · `decided` 36 · `proposed` 14 · `superseded` 10 · `lead` 2 · `deferred` 1 · `wish` 1.

**Citation, by core.** 27 of 101 entries are cited by at least one file in `core/`.

| by status | cited | by topic | cited |
|---|---|---|---|
| `decided` | 15 / 36 | `architecture` | 11 / 24 |
| `council-accepted` | 11 / 37 | `physics` | 11 / 48 |
| `proposed` | 1 / 14 | `process` | 4 / 16 |
| `superseded` | 0 / 10 | `naming` | 1 / 7 |
| `lead` | 0 / 2 | `ontology` | 0 / 1 |
| `wish` | 0 / 1 | `tooling` | 0 / 2 |
| `deferred` | 0 / 1 | `view` | 0 / 3 |

The coordinator's `tooling` 0/2 and `view` 0/3 reproduce exactly. My `process` and `architecture` figures come from the header-line parse; the slice agent's differing numbers are most likely the whole-file grep, which counts status words in prose.

**Direction of linkage.** 28 entries name a `core/` slug somewhere in their body; 27 are named by `core/`. The overlap is 8.

| | core cites the entry | core does not |
|---|---|---|
| **entry names a core slug** | 8 | 20 |
| **entry names none** | 19 | 54 |

**Reference integrity.** `DECISIONS[<slug>]` appears 139 times in the live tree. **21 of those do not resolve to a real entry — a 15% dangle rate — and 2 of the 21 are inside `core/`** (`#form-grid-equiangular-staggered` Primary sources; `#detail-structure-scheme-map` equilibria row). Most are ellipsis truncations (`DECISIONS[the-grid-tentatively-decided-…]`). One is worse: `DECISIONS[column-is-a-control-volume]` appears 5 times, reads as a complete slug, and is not one — the entry is `column-is-a-control-volume-with-sufficient-statistics`.

**The control that makes that number mean something.** `LEXICON.udon` is the same kind of carve-out at nearly the same size — 102 `|term`/`|note` entries against `DECISIONS`'s 101. `core/` makes 19 distinct `#lexicon/term/<slug>` references. **Zero dangle.** The difference between the two carve-outs is not size, age, or importance: `FORMAT.md` §5.1 defines the LEXICON reference form in a table, and does not define a decision reference form at all. `DECISIONS[...]` appears in `FORMAT.md` exactly once, inside a parenthetical example.

**A second dialect, in the other direction.** The ledger cites `core/` segments **34 times by filesystem path** (`|ref core/src/form-isostasy-column.md`, ×9), across 20 distinct segments — alongside 39 distinct slug-form references. `FORMAT.md` §5 binds every file in the repository and opens with *"References carry no path. A path is a location and it rots."* So the ledger's own home-pointers are half in the form the project forbids.

**Provenance keys.** `:session` resolves to a transcript for **62 of 101 entries**, not the full set:

| session | entries | transcript |
|---|---|---|
| `438f7e79-…` | 39 | present, but under `~/.claude/projects/-Users-josephwecker-v2-src-arch/` — a **different project directory** |
| `8145183f-…` | 17 | present, vivarium project dir |
| `4d13314a-…` | 6 | present, vivarium project dir |
| `01UzCVNNAn6eynKxo2fV74MC` | 31 | older ID format; no file anywhere under `~/.claude/projects/` |
| `01An1fos71L1LmxG3fSF6sE9` | 3 | same |
| `016WWpYtpHx9fAMdyehRxpSX` | 3 | same |
| `spike-corrected-scheme-cost`, `grok-front-door-telos` | 2 | hand-written labels, not session IDs |

I verified the 39-entry transcript by grepping it for `mean-pin-manufactures-the-seam` and finding the entry being drafted (5.9 MB file). The correction matters for the brief's premise: **the session ID alone is not a complete key** — the project directory is part of the identity, and it is a function of the session's cwd, which for the largest block was the program root rather than the member repo. In a repository whose central law is `#form-complete-content-addressed-key` — *complete key; over-key never under-key* — the ledger's provenance field is under-keyed. That is a cheap fix going forward and a pleasing one.

**Three hand-edit defects.**

1. **An orphaned `:council` note** at line 1035 sits *above* its `|decision[routing-spiral-…]` header, separated from the previous entry by a blank line. Under the file's own invariant — each decision is a self-contained block starting at column 0 — that note belongs to no block. A `grep '^|decision\['` reader loses it; a block parser attaches it to the wrong entry.
2. **One `:supersedes` on its own line** (line 353, `grid-report-supersedes-two-of-my-claims`), against the header's explicit rule that it goes on the `|decision[...]` line *so that the whole supersession chain greps in one read*. That entry's chain is invisible to the documented grep.
3. **One slug carries uppercase** (`the-grid-recon-argues-FOR-the-quad-grid`) against "slugs are unique and kebab-case," and three entries are indented four spaces rather than two.

**Content genuinely absent from the live tree** (not merely uncited — I grepped `core/`, `doc/`, `LEXICON.udon`, `FORMAT.md`, `CLAUDE.md`):

- `terminology-register-by-layer` — `:by us :status decided`. The **governing** naming rule that every specific naming call is an instance of (ontological register for architecture, the subfield's literature term for the nomos, plain code terms for implementation; every word kept honest). Nowhere live.
- `new-system-must-reach-the-goal-not-reimplement-poc` — `:by joseph :status decided`. Nowhere live. (Confirms the coordinator's finding.)
- `debug-poke-then-consolidate` — `:by us :status decided`. The poke/consolidate discipline. Nowhere live.
- `ascii-globe-in-info-colored-by-state` — `:by joseph :status wish`. The specimen with a measured cost: restated by Joseph twelve days later, unaware.

---

## 2. The species — "of various kinds" is the load-bearing part of the question

Reading all 101 entries, they sort into eleven kinds by *what the entry is for*. The treatments differ, and they differ most on the question the audit raised: **does this want a `core/` citation at all?** Four of the eleven do not, and that is not a defect.

| # | species | count (approx) | specimen | wants a claim home in `core/`? |
|---|---|---:|---|---|
| 1 | **Ratified law** — Joseph decided something about the project or the world | ~30 | `memoized-means-store-object`, `ordinum-governs-the-flux-web`, `water-world-is-the-promise-not-the-bug` | **Yes, always.** Highest cost when missing. |
| 2 | **Measured observation** — a number with a probe that could have failed | ~25 | `mfd-fan-is-a-bias-and-does-not-converge`, `mean-pin-manufactures-the-seam-and-the-mass` | **Yes** — an `obs-` segment states it; the entry is the provenance. Best-served species today. |
| 3 | **Retraction / self-correction** — the subject *is* a previous entry being wrong | ~12 | `grid-question-not-closed-authority-was-inflated`, `the-jensen-variable-was-wrong-and-so-was-the-sign`, `theta-is-lax-friedrichs-not-rhie-chow` | **No.** Its value is entirely historical. |
| 4 | **Supersession chain link** | 10 | `conservation-is-not-consistency` | **No** — correctly 0/10 cited. Wants chain integrity, not visibility. |
| 5 | **Build record** — "BUILT / EXECUTED / LANDED" | ~12 | `rock-mass-ledger-built-…`, `epoch-surfaces-are-store-citizens` | **No** — it is changelog. The segment already states the result. |
| 6 | **Scope ruling / work grant** | ~5 | `code-to-claim-wave-council-fronts-…`, `mantle-thermal-nomos-ruled-in-scope` | **No** — spent on execution. |
| 7 | **Research lead** | 2 | `the-z-grid-lead` | No, but wants re-reading when its area opens. |
| 8 | **Wish** | 1 | `ascii-globe-in-info-colored-by-state` | No — wants a *reading occasion*. |
| 9 | **Deferral** — an explicit non-decision | 1 | `regula-vocabulary-deferred` | No — wants to stay distinguishable from a decision. |
| 10 | **Naming / terminology call** | 7 | `terminology-register-by-layer`, `tile-as-honest-flat-artifact` | **`LEXICON`, not `core/`.** A routing question, not a segment gap. |
| 11 | **Operator / tooling / view decision** | 5 | `cli-world-dir-default-…`, `globe-ribbons-are-view-assembly-staleness` | **Has nowhere to go today** — see §3.4. |

Three consequences worth pulling out.

**(a) The raw 27/101 overstates the problem.** Species 3–9 are 43 entries for which a `core/` citation would be wrong or pointless, and `superseded` scoring 0/10 is the canon rule working, not failing. The honest denominator is closer to 58, of which 27 are cited — still a real gap, but half the alarm.

**(b) `proposed` is doing two different jobs, and the 1/14 rate conflates them.** Some `proposed` entries are unratified *verdicts* awaiting Joseph (`energy-probe-clamp-rates-and-a-friction-anomaly`). Others are *build reports* whose own text says "build verdict fresh — proposed, not council-sealed" — meaning the code landed and the segments were updated, and what is unsealed is the review, not the claim. Those two want opposite handling: the first should never be cited by core; the second already *changed* core and is simply not named as the provenance. If any status vocabulary work is ever done, this is the seam I would name first — and I would not do it now, because the entries themselves disambiguate in one line of prose and a reader can tell.

**(c) Species 3 answers the file's own open question, and answers it against both offered options.** The header leaves undecided whether the file becomes the full historical log with a projected open view, or stays a working set with superseded items swept elsewhere. The retraction species makes the second option costly: because `#scope-segment-canon` FE(4) requires that core state present truth only, **the correction trail cannot live in a segment, and this file is its only home.** `grid-question-not-closed-authority-was-inflated` — with Joseph's `|judgment` block addressed explicitly to future readers — is not sweepable material at any horizon. Meanwhile species 5 and 6 (build records, scope rulings) genuinely *are* spent, and sweeping them would be pure gain. **The file has at least three populations with opposite retention needs, so a single sweep policy will be wrong for two of them.** My recommendation on that adjacent question, in §3.3: project the views and move nothing.

---

## 3. Recommendations — policy, going forward

Ordered by cost. The first two are habit and a table row; the third is a small read-only tool; the fourth is yours.

### 3.1 Make the reference form real (cheapest, and it is the LEXICON comparison cashing out)

Add one row to the `FORMAT.md` §5.1 table:

| form | target |
|---|---|
| `DECISIONS[<slug>]` | a decision record in this project's `DECISIONS.decision-log.udon` |

and stop using `core/src/<slug>.md` paths inside the ledger, per §5's existing rule.

Then lint both directions. This is ten lines in `bin/check` or beside it:

```sh
# every DECISIONS[x] in the live tree resolves to a |decision[x]
# every #<slug> in DECISIONS resolves to core/src/<slug>.md
```

**Why this is the highest-confidence item in the report:** two carve-outs of nearly identical size, one with a defined reference form and one without, sitting side by side in the same repository for the same twelve days, with a 0% and a 15% error rate. It is as close to a controlled comparison as this corpus offers. And it is not a schema change to `DECISIONS` at all — it names a convention 139 uses old.

Cost: one table row, one grep loop. It converts 21 silently dead references into a build finding today and stops the next one.

### 3.2 The claim-home line — one direction, entry-side (a norm, not a mechanism)

Seven entries already carry a prose "Claim home: `#slug`" line, and it is the right convention. I recommend making it the habit for species 1, 2, and 10 — the claim-bearing kinds — with `#gap` as an honest answer where no segment exists yet.

**The direction matters and should be stated, because it is the actual failure.** Today the linkage is one-way in one direction or the other and almost never both (8 of 101). Nobody said which way it should point, so both partial conventions grew.

It should point **entry → home**, and not the reverse, for three reasons:

1. The entry is written **once**, at decision time, by the agent that knows the answer. A segment is edited many times by agents that do not.
2. A segment listing every decision that touched it is **diff voice**, which `FORMAT.md` §6 forbids in every file. A segment should cite a decision only where the decision *is* the present-tense evidence for the claim — which is what most of the existing 27 citations correctly do.
3. It is the direction that scales. The claim channel stays clean; the history channel carries the index.

I would **not** add a `:home` field. The prose line works, the header already invites field invention (`:council` and `:superseded-by` were both added in the field, correctly), and one more field is one more thing to be inconsistent about. If a machine-readable form is ever wanted, fixing the *prose* form (always `Claim home: #slug`, case-insensitively greppable) costs nothing and is reversible; a field is neither.

### 3.3 A projection, not a reorganization — and it dissolves the open question

Roughly fifty lines, read-only, no state, computing three views over the file as it stands:

- **open** — `proposed` · `lead` · `wish` · `deferred`, which is the set nobody currently reads;
- **unhomed** — claim-bearing entries with no claim-home line and no `core/` citation;
- **dangling** — dead `DECISIONS[...]` references anywhere in the live tree (this is 3.1's linter, surfaced).

This is the whole of the read-back fix. It would have surfaced `ascii-globe-in-info-colored-by-state` on the first run.

It also **dissolves the header's open question rather than answering it**: you get the historical log *and* the working view, and nothing moves — which matters because §2(c) shows any single sweep policy is wrong for two of the three populations.

**The honest cost, named because the brief was right to warn about it:** this is a mechanism, and mechanisms rot. The mitigation is that this one is a pure projection with no state and nothing depending on it — if it rots, delete it and lose nothing. That is categorically different from a schema change, which cannot be deleted once a hundred rows carry it. If the choice is between one throwaway script and one permanent field, I would take the script every time, and if the choice is between the script and neither, I would still take the script — but it is the item on this list I hold least firmly.

### 3.4 Two things that are Joseph's, and one that is not mine to write

**(a) A reading occasion for the wish set.** One row in `CLAUDE.md`'s "Where to read (by job)" table:

> | **What Joseph asked for that isn't built** | `DECISIONS` `:status wish` / `lead` / `deferred` |

That single line is the cheapest possible fix for the highest-cost specimen in the whole finding. It costs a row and needs no tooling.

**(b) Complete the provenance key.** Going forward, record what makes `:session` resolvable — the project directory, or equivalently the session's cwd. `:session 438f7e79-… :proj -Users-josephwecker-v2-src-arch` would have made the largest block of the ledger's provenance findable without a search. The 37 older-format entries are probably not recoverable and I would not spend effort trying; I would note it in the header so a future reader does not repeat my search.

**(c) The operator/tooling subject.** Species 11 has no home because `core/` has no subject for the vivium as *a thing a person builds, watches, steers, and reads a report about*. I reached this from the decision side; the parallel second opinion reached the same place from the ontology side and traces it to an explicit unanswered question in the retired outline (*"Does Part VI belong in `core/` at all? It is about the project, not the world. — Joseph's call."*). **Two independent routes to one finding, and it is a question you already deferred once.** I have nothing to add to their §6D except the corroboration, and I would not have either of us answer it for you.

---

## 4. Remediation of the ~100 existing rows

Separated from §3 as asked, because the costs differ sharply. R1 is typo repair; R2 is an afternoon of indexing; R3 is yours; R4 is the thing I recommend against.

**R1 — mechanical repairs, no judgment, safe to do now (minutes).**

- Fix the 21 dangling `DECISIONS[...]` references — 2 in `core/`, 19 in `msc/`. Each is an ellipsis or truncation with a unique real target.
- Move the orphaned `:council` note (line 1035) inside its entry.
- Move the stray `:supersedes` (line 353) onto its header line, so the chain greps in one read as the header's own rule intends.
- Normalize the uppercase slug and the three four-space indents.

None of these changes what any entry says.

**R2 — the indexing pass the consolidation never ran (an afternoon, judgment, still safe).**

Add a claim-home line to the claim-bearing entries that lack one — species 1, 2, and 10 — naming `#slug` where the segment exists and `#gap` where it does not. **This is indexing, not authoring: no segment is written, no claim is minted, no status changes.** It is precisely the work the 2026-07-21→24 peel did for prose and never did for this file, and it is the step that makes §3.3's "unhomed" view converge to something small.

Two things fall out of it for free: the naming entries route to `LEXICON` rather than `core/`, and the entries that turn out to have no possible home are exactly the specimens for §3.4(c).

**R3 — the four orphans, which are yours (each is a small decision).**

Three `decided` entries and one `wish` have no restatement anywhere in the live tree:

- `terminology-register-by-layer` — I would rank this first. It is the *governing* rule the seven naming decisions instantiate, it is `:by us :status decided`, and `FORMAT.md`'s own "the segment ontology is adopted from ASF, and it is expected to change" section is arguing adjacent to it without citing it. Its natural home is `LEXICON.udon` or `FORMAT.md`, not a segment.
- `new-system-must-reach-the-goal-not-reimplement-poc` — the charter of the current build wave.
- `debug-poke-then-consolidate` — a live methodological norm; the nearest existing kin are `#norm-declaration-must-convict` and `#norm-probes-before-claims`.
- `ascii-globe-in-info-colored-by-state` — the space half shipped; the time/phase half is now newly *cheap*, because `mantle_thermal::abyssal_epochs` and `World::epoch_reduction` landed today and the playback view already colors by epoch. This may be closer to done than it was when written.

I have deliberately not drafted any of them. They are `:by joseph` / `:by us`, and writing them would be exactly the authority inflation `#norm-decision-authority` exists to stop.

**R4 — what I recommend against: a back-sweep adding `DECISIONS[...]` citations into segments.**

This is the obvious reading of "core doesn't cite DECISIONS," and I think it is the wrong move. It would be ~60 edits across 79 segments, in diff voice, to make segments narrate their own provenance — and `FORMAT.md` §6 forbids that shape in every file for good reasons. The one-way entry→segment link in §3.2 buys the same discoverability at a fraction of the cost and with zero segment churn. A segment should cite a decision where the decision is *evidence for the present claim*, and that is a judgment made per segment when the segment is written, not a sweep.

---

## 5. What should not change, and why I want to say so explicitly

The brief invited "nothing should change" as a legitimate answer, and while my answer is not quite that, four things in the current design are working and could plausibly be damaged by a well-meaning reorganization.

**The fast-append design.** 101 substantial entries in twelve days across nine sessions, concurrent-safe, no interleaving, no garbled block. The one structural defect I found (the orphan) came from a *hand edit* during a later pass, not from the append path. Any proposal that makes appending require reading or parsing the file first should be measured against this record.

**The authority legend.** It caught its own author one day after he wrote it (`preserve-the-structure-declare-the-sacrifice`'s `:note`), and `council-accepted` is a genuinely good invention — a tier that records deliberation without borrowing Joseph's weight, with its non-inviolability stated in the definition. I would not touch either.

**The carve-out.** A decision is not a claim. Who decided, when, under what authority, and what was considered and declined is a different kind of fact from what is true. Forcing decisions into segments would produce exactly the fork `FORMAT.md` §5 warns about, and would put diff voice inside the claim channel. `#scope-segment-canon` FE(3) has this right.

**The retraction trail.** It is the most valuable content in the file and it has nowhere else to live, precisely *because* integration is replacement. The species work in §2 is largely a defense of it.

---

## 6. Relation to the parallel second opinion

The two questions touch and are not the same — theirs is about segment *types*, mine about the relationship between two *authorities* — and I do not think they are one question. But they intersect twice, and both intersections are worth having.

**Their §8 leaves one thread explicitly open and hands it to this report:** whether their §5 (does `status` need a non-propositional axis for desiderata?) dissolves once you look at how `DECISIONS[ascii-globe-in-info-colored-by-state]` has actually functioned as a wish home. They write that it "worked, in the sense that the row still exists."

**The measured answer is: it worked as storage and failed as circulation.** The row exists, is correct, is `:by joseph`, and was found by an audit — and Joseph restated the same desire twelve days later, to me, not knowing it was written down. So their §5 does **not** dissolve, but it relocates: **the missing thing is not a `status` axis in `FORMAT.md`, it is a reading occasion for a channel that already holds desiderata correctly.** `:status wish` — "a feature to TRY when appropriate, NOT a requirement; drop without guilt" — is a better-shaped home for Joseph's twelve explorer intents than any new tier in a propositional ladder would be, because it was never propositional to begin with. What it lacks is §3.3's projection and §3.4(a)'s one-line reading occasion.

That is a conclusion neither of us reaches alone, and it argues for the cheaper remedy on both sides: **no `FORMAT.md` change, no new type, no new status tier** — route desiderata to the channel that already fits them, and make that channel readable.

**The second intersection is species 11 / their C1**, which I described in §3.4(c). Same finding, two routes, and their tracing of it to the retired outline's unanswered Open Question 1 is the better provenance.

I agree with their §6A and §6B and would not restate them here; the `CONSOLIDATION-STATUS.md:9` correction in particular is one line and is not anybody's call to make.

---

## 7. Where I could be wrong

**High confidence** (mechanical, re-runnable): every count in §1 — the census, the citation cross-tab, the 139/21 reference figures, the LEXICON control, the 34 path-form refs, the session-to-transcript mapping including the different-project-directory finding, which I verified by reading the transcript. The three hand-edit defects (read in place). The four content-absences (grepped `core/`, `doc/`, `LEXICON`, `FORMAT`, `CLAUDE`).

**Medium** — **the species taxonomy in §2 is mine, and it is a judgment.** I read all 101 entries and sorted them; another reader would draw some lines differently, and the counts are approximate where an entry does two things at once (several build records also carry measurements). What I would defend more firmly than the exact partition is the claim that *the file has multiple populations with opposite needs* — the specimen that persuaded me is the contrast between `grid-question-not-closed-authority-was-inflated` (never sweepable; its whole value is the record of an error and Joseph's judgment on it) and `mantle-thermal-nomos-ruled-in-scope` (a work grant, entirely spent, three entries later). If you think that contrast is doing too much work, that is the right place to push.

**Medium** — the honest denominator of 58 in §2(a) inherits the taxonomy's softness.

**Lower** — **§3.3 is the one item where I am recommending a mechanism, and the brief's hazard applies to it squarely.** My defense is that it is a stateless projection nothing depends on. But I notice the pull: "write a small tool" is the most satisfying-feeling recommendation on this list and therefore the one most likely to be motivated reasoning. If you want to take §3.1, §3.2, and §3.4(a) and leave §3.3, I think that gets most of the value — the reading occasion plus a working reference form is the substance, and the projection is convenience.

**A way the whole report could be wrong.** I have argued that the carve-out is correct and the problem is circulation. Someone could reply that a decision log which needs a tool to be readable is a log that has outgrown its format, and that the real answer is fewer, better entries — that 101 entries in twelve days is itself the finding, and several of the physics entries are long enough to be segments that were written in the wrong file. I do not think that is right (the length is where the reasoning and the self-corrections live, and it is the best content in the repository), but I have not refuted it, and it is the argument I would want to hear against this report.

---

## 8. Feedback on the brief

It was unusually good to receive, and two specific things did work.

**The measurement invitation was load-bearing.** "A slice agent's counts differed slightly from mine — treat both as indicative and measure it yourself if the number matters." I did, found the likely cause of the discrepancy (whole-file grep vs. header-line parse), and that same parse is what produced the status cross-tab that turned "27/101" into a much more useful "and 43 of the uncited should be uncited." A brief that had handed me a number would have gotten a report built on it.

**Giving me the parallel agent's landing path and the words "coordinate rather than duplicate, and if you conclude they're really one question, say so"** left both answers open. Their report landed while I was reading the log, and their §8 turned out to contain a question addressed to my data. Neither of us would have gotten §6 from a brief that partitioned the work more tightly.

**One thing in the brief needed correcting, and it is the kind you asked for.** The transcript-recovery premise — that UUID-form `:session` values map onto filenames in the vivarium project directory — is true for 2 of the 4 UUID sessions and false for the largest. `438f7e79-…` (39 entries) lives under `-Users-josephwecker-v2-src-arch/`, because that session was started at the program root. I only found it because the mapping failed and the failure was surprising. Had the brief been less specific I might have searched more broadly sooner; had I trusted it, I would have written "not recoverable" about the richest session in the ledger. The generalization is §3.4(b), and it is the finding I am most pleased with, because the repository's own central law names the defect exactly.

**One thing I would have wanted.** The brief says "the interesting ones are unlikely to be where I'd guess," which was true and which I appreciated — but it left me to discover on my own that `LEXICON.udon` is a same-size carve-out sitting right beside `DECISIONS` with the opposite outcome. That comparison is the strongest single piece of evidence in this report and I found it by accident, three quarters of the way in, while checking something else. A brief for a question about *one* carve-out might productively name the *other* carve-outs as the natural controls.

I am glad to stay on the line, and there are three threads I would expect to be useful: whether the retraction-vs-work-grant contrast in §2(c) carries the weight I put on it; whether §3.3 survives your read of the mechanism hazard or should be dropped to leave §3.1/§3.2/§3.4(a) standing alone; and R2, which I would be happy to execute as pure indexing if you want it, since it changes nothing any entry says and is the step that makes everything else converge.
