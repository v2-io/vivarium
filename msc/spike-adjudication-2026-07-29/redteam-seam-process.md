# Red-team of the graduate verdicts — seam/store + process/one-off slices

*2026-07-29. Adversarial second pass over the two first-pass reports (`spikes-seam-store.md`, `spikes-process-oneoffs.md`). Posture: try to refute every "covered/absorbed" call and find claims the first pass never mentioned. Method: read every target item directly (not only the reports' trails), then checked both sides of each cited carrier at file+line. The first pass's own flagged residue is conceded and not re-litigated. Coordinator note honored: Tier-1 items landed in core during this pass (store-as-save FE(8), builder-daemon restart/fork-pool, in-vivia/add-system staleness, parity KI(3), rock-mass row, census rows, coriolis pointer) are treated as landed, and the remaining covered-claims judged on their own evidence — one of those landings (fvm FE(6)) is noted where it changes a verdict's condition.*

Bottom line: **most verdicts survive, several after genuine attack** — the null-space DECISIONS absorption in particular is the deepest I have seen in this corpus. Refutation-grade findings: **the nine agent-brief records have two live core citations nobody checked** (§7), **the confusion-log C11 "verified fixed" is factually false** (§5a), **drafted-patches "none needed" is wrong for its three live-ledger patches** (§5b), and **the harness family's residue enumeration misses at least five real items, including the strongest deliverable of one of its own second opinions** (§6).

---

## 1. `msc/spike-null-space/` — verdict "graduate-after-repointing" SURVIVES, but the repoint list is incomplete

**Attacked and held.** I read `probe-output.txt` (405 lines) in full against the two DECISIONS entries and the claimed segment/ledger carriers. Absorption is near-total and often verbatim-plus:

- `DECISIONS[our-kernels-have-no-null-space…]` (line 770) carries controls, pins, Brillouin-zone map, θ sweep, κ findings incl. the χ-gain-below-1-with-creep-off result, nonlinear confirmation, and the §5 staggered/collocated payoff.
- `DECISIONS[jarrett-roughness…]` (line 797) carries the SECOND MECHANISM table near-verbatim, the control-must-match-base-state lesson, and the council-accepted ASSUMPTIONS remedy — which **was executed**: `ASSUMPTIONS.md:56` (Jarrett own row, feedback measured), `:68` (staggering asset row), `:79` (lumped row struck through with both mis-filings named).
- `core/src/form-grid-equiangular-staggered.md:42`, `detail-fvm-control-volume.md`, `form-declared-structure-tradeoff.md:37,42` verified as claimed.

**Refutation of the repoint list's completeness (the actual catch):** the first pass enumerated only the two DECISIONS `|ref` lines (791, 811). It missed that **the live in-crate instrument writes into the graduating directory**:

- `crates/vivarium-world/examples/null_space/main.rs:41` — `PathBuf::from("msc/spike-null-space/fig")` with `create_dir_all` (also doc lines 15, 215). After graduation, any re-run of the full instrument silently **recreates a stray `msc/spike-null-space/` dir** and writes twelve fresh SVGs into it — the graduated copy in `.super-archive/` and a live regenerated twin would then coexist. Not breakage, but exactly the split-home state the sweep exists to end.
- `crates/vivarium-world/examples/null_space_gate.rs:15` — doc-comment cites `msc/spike-null-space-probe/NOTE.md` by live path.

Both need re-pointing (or the outdir moved, e.g. to a scratch/target path) in the same commit as the `git mv`.

**One nuance lost in absorption (Tier-2 grade, not a refutation):** `probe-output.txt:206` — the "cap ALONE" row (θ=1, n=0.04, breaking cap): Fr pinned at 2.00, ρ = 1.00000, **stable**. The Froude-2 breaking cap alone also stabilizes the pre-fix kernel. DECISIONS credits Jarrett solely ("What actually stabilises the shipped kernel is the JARRETT ROUGHNESS… The credit was on the wrong term"). True for the shipped configuration (Jarrett gets there first by dropping Fr to 0.75), but the cap-alone row is a second sufficient stabilizer the ledger does not mention — relevant if anyone ever removes Jarrett per its own artefact conviction and expects instability.

## 2. `msc/spike-null-space-probe/` — verdict "graduate-after-landing-X" SURVIVES; X(a) is now landed

- R4's central claim verified — and **already fixed in core** (coordinator's session or the first pass's landing): `detail-fvm-control-volume.md:31` FE(6) now reads "instrument built, gate built, wiring open… `bin/check` does not run it — a guard nobody runs is narration; wire it or record the decision not to. Library-test lift = moving the ~200-line core to `src/probe/`." That is R4's fix (a) verbatim, including the graduation-cost analysis NOTE.md §3 carried.
- Verified myself: `bin/check` has **no** null_space line (grep, exit 1) — X(b) still open, but the segment now states it honestly, so the claim surface is truthful either way.
- Tolerance lessons verified in gate source as claimed: `null_space_gate.rs:80–85` (`tol_rel`), `:103–109` (≥4 decades, `gap > 1e4`), `:147–149` (f32-grade cut).
- NOTE.md's own "No segment edit is owed by this spike" (line 64) was false — the first pass caught it (R4); concur.
- Remaining pointer: `null_space_gate.rs:15` cites NOTE.md (see §1).

## 3. `msc/claim-channel-cleanup-2026-07-23.md` — "graduate-clean" SURVIVES

Checked its three "judgment calls not made" and its demotion targets myself:

- Judgment 1 (ARCHITECTURE §1 rewrite) and 2 (`.archive/` pruning): mooted — `doc/` is `PROCESS.udon` only; `.archive/` is `README.md` + `SUPERSEDED.md` only (verified `ls`).
- Judgment 3: `core/src/detail-regula-design.md` exists (verified), and `LEXICON.udon:308` `|term[regula] :status open` — the regula question re-opened with a design home, which retires the "out of scope" deferral.
- The demotions it records into live code module docs (`store.rs`, `nomotheke.rs`, `query.rs`) are records of applied edits, not carried claims; nothing in the live tree cites this file (grep clean).

## 4. `msc/promotion-mine-2026-07-23-continuity.md` — "graduate-clean" SURVIVES

- MANIFEST "Provenance audit 2026-07-23" table (`.super-archive/MANIFEST.md:96–108`) verified: mirrors all four bullets including the PHASES-pin honest residual, plus the graduation-checklist hardening.
- All four REDUX landings live as segments (verified `ls core/src/`): `form-reductionist-fallback-cases`, `form-fidelity-ladder`, `sketch-detail-abstract-reversion` (since renamed to the nonlinear-non-local-flux framing — stronger, not lost), `sketch-nomos-declaration-boxes`.

## 5. `msc/audit-onboarding/` — TWO PARTIAL REFUTATIONS, one survives

### 5a. `confusion-log.md` — "all eleven confusions verified fixed" is FALSE for C11

The first pass wrote: "the `:supersedes` header-line defect fixed at line 357." **Line 357 IS the defect, still present:** `DECISIONS.decision-log.udon:354` (`|decision[grid-report-supersedes-two-of-my-claims]` header) does not carry `:supersedes`; it sits on its own line at **:357**, violating the schema rule at **file line 22** ("⚠ WHEN USED, :supersedes GOES ON THE |decision[...] LINE ITSELF — never on its own line"). `grep '^|decision\['` still does not yield that chain link.

What *was* fixed is the downstream harm, via a different mechanism: the supersession *target* `seam-amortization…` (line 307) is re-marked `:status superseded`, and the 07-24 pass (entry at line 979) reconciled statuses to chains. So C11's practical sting is drawn, but the report's stated verification is wrong at the letter, and the schema violation the confusion documents is live. Verdict downgrade: graduate-as-record still defensible, but the "verified fixed" claim should not be relied on as written; either fix line 357 (a one-line move, drafted verbatim in drafted-patches §5) or record the decision that chain-target re-marking supersedes the header rule.

Spot-checks that held: C3 `tmptmp.md` gone; C4 `LEXICON.udon:301` `|term[manifest] :status settled`; C5 `regula :status open` + `#detail-regula-design`; C10/C4-adjacent ASSUMPTIONS rows corrected (see §1).

### 5b. `drafted-patches.md` — "graduate-clean — none applied, none needed: every target file superseded wholesale" is REFUTED for the live-ledger patches

The rationale is true for patches 1–4 and 8 (CLAUDE/ORIENTATION/ARCHITECTURE-era targets superseded; ASSUMPTIONS fixed better than drafted; manifest LEXICON entry exists). It is **false for patches 5–7, whose target is `DECISIONS.decision-log.udon` — live, not superseded:**

- **Patch 5** (move `:supersedes` onto the header): never applied — §5a above.
- **Patch 6(a)** (snyder authority note): `DECISIONS:239` — `snyder-closes-the-projection-lead` still `:by us :status superseded`, **no note added**. The flag's substance stands: a reader grepping `:by us` still finds a grid verdict in Joseph's name that `grid-question-not-closed-authority-was-inflated` records him as not having made; the drafted note itself says "`:status superseded` does not cure it." The 07-24 adjudication pass ran over the *proposed* set and did not touch this.
- **Patch 6(c)** (measured-false "~10 congruence classes" note): `DECISIONS:256–262` — `geometric-contract-metric-set`, still `:by us :status decided`, still asserts "~10 congruence classes" in **both** `|reason` and `|impact`, no in-entry correction. The *content* correction is carried (`DECISIONS:344` two-not-ten + `form-grid-equiangular-staggered.md:42`), so no claim is lost — but the flagged ledger defect (a decided `us`-entry carrying a measured-false number with no pointer to its refutation) is live, and **this graduating file is its only flag-carrier**. Patch 6(c) was explicitly labeled "purely mechanical… safe to apply" and never applied.
- **Patch 7**: half-resolved — `preserve-the-structure…` is now `:by claude :status council-accepted` (line 655) ✓; the weaker sibling flag on `bias-vs-noise-is-the-decisive-audit` (line 392, still `:by us :status decided`, "reaching for it" quote) is unresolved and unmentioned by the first pass.

These are all Joseph's-authority items — none of them are mine or the sweep's to fix — but graduating the only file that flags them, into treated-as-deleted ice, buries live authority-hygiene flags. Cheap fix before graduating: one DECISIONS `:note` (or a census row) carrying the three open flags, then graduate.

### 5c. `nomos-design-sketch-atmospheric-circulation.md` — "graduate-as-record with one pointer landed" SURVIVES

- The coriolis pointer is conceded residue and the coordinator reports it landed.
- I attacked the two claims the first pass never mentioned: the **multi-structure tension gap** ("the table is one-row-per-structure… a rotating fluid is the normal case, and the doc has never walked one") is absorbed **superseded-by-stronger** at `form-declared-structure-tradeoff.md:20` FE(2) — near-verbatim ("Erosion (few structures) is the unusually clean case; a rotating fluid is the normal case") plus the Arakawa–Lamb/Ringler citations the sketch lacked. The **grid-aligned-jets prediction** is proposal-grade design for an unbuilt nomos; its general law (grid-axis attractors → grid-aligned features) is carried at `obs-cube-locked-kernel-bias.md:19` and `detail-structure-scheme-map.md:34`; the atmo-specific transfer rides with the file-as-record. No uncarried claim found.

## 6. `msc/harness-reconciliation-2026-07-24/` — verdict direction survives; the residue enumeration is INCOMPLETE (five real misses)

The Tier-1/Tier-2 lists are good but under-enumerate. Items the first pass called covered, or never mentioned, that fail direct check:

**(a) The segment-ontology second opinion's §5 — the strongest catch in this family.** The first pass: "both second opinions live near-verbatim in `#form-complete-content-addressed-key` KI(3) and `#scope-segment-canon`." True for the invalidation-granularity opinion (KI(3) verified near-verbatim, including the two no-attribution narrowings and the module-separation reframe). **Only half-true for the ontology opinion:** `scope-segment-canon` Working Notes carry the FE(4)-convictor finding and loss list, and `norm-caught-disciplines-become-mechanisms` carries the findability=convictability thesis — but the opinion's §5 deliverable, the **restated-for-Joseph replacement of FORMAT OQ1** ("does a specification need a kind for a *requirement* — and does `status` need a second, non-propositional axis for how firmly a desideratum is held: wished / intended / committed / met"), with its specimen (the twelve explorer intents became zero core hits *because* the format cannot hold desiderata without overclaiming) and its cheap probe (try to land the twelve honestly, see what breaks) — **is carried nowhere**. `FORMAT.md:271` OQ1 is unchanged in the old "contract" framing. This also explains-and-outranks Tier-2 #3 (the eleven intents): the first pass conceded the intents lack a carrier but not the structural *why* or the proposed remedy. Only carrier: `segment-ontology-second-opinion.md` §5, graduating.

**(b) toolchain §4.6 — explorer-legibility has no owner, in either layer.** "The explorer must be able to say, for what it renders: which nomos versions, which fidelity tier, which epoch, lawful or provisional" — the file shows the five pieces exist unjoined and checked ice too ("a gap in both layers, not a peel failure"). Unlisted by the first pass. `#norm-no-depiction-without-referent` (landed since) is adjacent but different: it bans depicting what has no referent; it does not require provenance legibility of what is depicted. Distinct from Tier-2 #6 (confidence-scalar rejection), which is about *how not* to summarize; this is *that nothing owns* the honest summary.

**(c) retired-front-doors §2.5 + plan-corpus — the CLI surface has no claim home.** Verb set, world-dir resolution order, the never-scan negative, attach-not-fail semantics: no OUTLINE §III row (verified — §III header at `core/OUTLINE.md:112` has no CLI gap row), no segment. Arguably README-grade, but the file's own point stands: `#scope-segment-canon` FE(2) says READMEs are not claim homes, so a CLI change today contradicts nothing. Unlisted.

**(d) plan-corpus §3 — the memo-grain-selection rule.** "A native system chooses its memo grain to match its consumers' cones (per-basin, not one global graph)." `form-depend-by-key-never-latest.md:23` FE(5) still carries only "keys may name outputs / over-keying is safe" — the grain-selection design rule is dropped (verified). It is the design answer to whole-store invalidation *from the consumer side*, sibling to KI(3)'s producer-side answer, and it is on no list.

**(e) retired-front-doors §2.7 — the pre-run study-validity gate.** Distinct from the fitness-vs-declared-prior item the first pass called "practiced": this is the *pre-run* check ("you can pre-audit whether a run could possibly mean what it claims" — before spending a 40-minute build). Ingredients in `#def-in-vivia`/`#form-in-vivia-citation` FE(3)/registry fold; the gate as a named pre-spend check exists nowhere. Unlisted.

Smaller unlisted (ice-only after graduation, listed for the record): the ~8× real-time target with its memoize-the-fill budget argument (plan-corpus §3); the `vivarium status` aspiration-dump guard — its "horizon, never failures" half survives only as an ordinum comment (`terrestris.ordinum.udon:26`), the target_phase-advances UX rule is ice-only; access-profiles-attach-at-the-query-front-end; explorers-may-offer-to-start-a-builder; the replay-vocabulary collision (deep-time playback ≠ replay-from-pinned-generators — adjudications §4.3); and adjudications §3.5's authority observation that **the source-digest law itself (complete-key FE(4)/KI(3), `exact`) has no DECISIONS row of its own** — un-ratified law, reopenable, and nothing on any list says so.

**Honest nulls — items I attacked that turn out resolved by events (no action):** FINDINGS §5.4 (`--epochs` placement) — `erosion_epochs` is a manifest field (`spec.rs:126`, doc line 17) and `vivarium build` is a builder path, so core-view-wall FE(4) is satisfied; the beacon-cone degenerate-case note — beacons landed 07-28 (`DECISIONS:1247`); the convergence-detector-is-the-ε-ingredient note — superseded by Component E being built; CONSOLIDATION-STATUS's "do not re-mine ice" self-indictment — the file was rewritten 07-28 and no longer carries the line.

## 7. `msc/agent-briefs/` nine closed records — "graduate as records" REFUTED as unconditional; graduate-after-repointing

**The miss:** two live core segments cite two of the nine by live path, and the first pass checked only DECISIONS-side citations (its reason for keeping the *directory* live):

- `core/src/form-flux-web.md:43` → `msc/agent-briefs/2026-07-23-sufficient-statistic-adjudication.md` (Working Notes, the do-not-mint adjudication provenance)
- `core/src/form-pull-query-composition.md:37` → `msc/agent-briefs/2026-07-23-lazy-query-graph-adjudication.md` (Epistemic Status — "Hostile-read FE from peer adjudication", i.e. the segment's *pedigree* line)

`.super-archive/` is treated-as-deleted for tactical dependence — the same ground on which the seam slice kept wavelet-store live over three instrument citations. These two are provenance-grade, not instrument-grade, so repoint (or drop to git-history reference) rather than keep-live — but the condition must be stated; the first pass's verdict has no repoint clause for the nine.

**Where I attacked execution and it held** (sampled directly, not from the report's trail):

- *multirate-coupling*: kill executed; `form-scale-separation-directional.md` exists, `OUTLINE:154`; bands homed in `#detail-phenomena-systems-map` (OUTLINE:196).
- *archive-residual*: all recommendations executed — `form-kernel-imperative-boundary.md` exists; `doc/PROCESS.udon:96` `|norm[agentic-verdict-is-flag]`; VIVARIA-DEFINITIONS/DECLARATIVE-FRONTIER/PHASES in `.super-archive/from-archive/`; ordinum `:reportatio` repointed with pin preserved (`terrestris.ordinum.udon:7–10`); SUPERSEDED.md stayed in `.archive/` as required.
- *cell-area-spike-plan*: executed beyond plan — `measure.rs` carries the convicting PROBE-8 re-pins (`measure.rs:96–118`: ~0.412 edge, ~17.81% mean asserts), `erosion.rs:267,476–485` per-cell area in drainage+deposit.
- *super-archive-provenance-audit*: MANIFEST mirror verified (§4 above).
- *run-modes / sufficient-statistic / lazy-query*: outcomes anchored in the segments that cite them; the run-modes half-done was Tier-1 #4 (conceded, now landed per coordinator).
- *fable-middle-peel*: unverified — the first pass's own Tier-2 #13 concedes it; conceded here too.

**One override worth a line in the graduation record:** `memory-surfaced-2026-07-13.md` was graduated (MANIFEST batch 4) *against* the archive-residual brief's explicit "Not yet — entry adjudication incomplete." MANIFEST records the override honestly ("Inventory ice; contradicted rows stay dead"), so it is escalated-not-silent — but project memory `MEMORY.md` still points at the **stale path** `.archive/memory-surfaced-2026-07-13.md` and calls the adjudication "open work." Procedural memory fix, not a claim loss.

---

## Scorecard

| Target | First-pass verdict | Red-team result |
|---|---|---|
| spike-null-space | graduate-after-repointing | **Survives** — add `null_space/main.rs:41` outdir + `null_space_gate.rs:15` to the repoint list; cap-alone stabilizer nuance optionally to Jarrett entry |
| spike-null-space-probe | graduate-after-landing-X | **Survives** — X(a) landed (fvm FE(6):31); X(b) open but honestly stated; gate doc pointer to repoint |
| claim-channel-cleanup | graduate-clean | **Survives** (checked, not taken on faith) |
| promotion-mine-continuity | graduate-clean | **Survives** (MANIFEST mirror + four landings verified) |
| confusion-log | graduate-as-record | **Partial refutation** — C11 "verified fixed at line 357" is false; defect live at DECISIONS:357 vs rule at :22; harm mitigated at :307/:979 |
| drafted-patches | graduate-clean, none needed | **Refuted for patches 5/6(a)/6(c)/7b** — target is the live ledger, not superseded docs; three authority flags have this file as sole carrier |
| nomos-design-sketch | graduate-as-record + pointer | **Survives** — both unmentioned claims verified absorbed/proposal-grade |
| harness-reconciliation (8) | graduate after Tier-1 | **Direction survives; enumeration incomplete** — five unlisted residue items, led by the ontology opinion's OQ1 reframing (carried nowhere; "near-verbatim" claim only half-true) |
| agent-briefs nine records | graduate as records | **Refuted as unconditional** — `form-flux-web.md:43` + `form-pull-query-composition.md:37` cite two of the nine; graduate-after-repointing; memory-surfaced override + stale MEMORY.md path noted |

*Staying on the line for follow-ups.*
