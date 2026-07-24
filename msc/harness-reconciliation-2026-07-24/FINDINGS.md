# Harness reconciliation — did core absorb the workflow / CLI / harness corpus?

*2026-07-24. **Instrument, not claim canon** — this is an audit record, and per `#scope-segment-canon` it decides nothing. Where it disagrees with `core/`, core wins; where it names a gap, the gap is a candidate for a segment or an OUTLINE `#gap` row, not a claim in itself. Companion per-slice reports sit beside this file.*

**Epistemic marking used throughout.** `[verified]` = the primary source was read by the coordinating agent, at the cited location. `[relayed]` = a slice agent's finding, not independently re-checked. The distinction is load-bearing: a claimed absence is cheap to assert and needs checking in both directions.

---

## 1. Why this was run

Joseph attached a condition before entrusting a builder/demand/observability design to an agent: that `core/` actually carries everything about the workflow / CLI / harness from the historical documents — so the design proceeds from accumulated thinking rather than re-deriving something worse.

**Correction to this document's own framing** `[verified]`. An earlier draft named `#scope-segment-canon` FE(4) as the claim under test. It is not a claim — it is a **prohibition**: "no unique non-superseded meat **may** live only there." A rule can be violated without having been false. The segment's Epistemic Status goes further and explicitly disclaims the reading this audit attributed to it: "The rule does **not** claim that all true content has already been migrated into segments; migration incompleteness is expected."

So **core never asserted the mine was closed.** The overclaim is `CONSOLIDATION-STATUS.md`, in a file whose own first line says "**Not claim canon.**" The claim channel was honest throughout; a non-canon status file was not.

Two consequences worth carrying:

1. FE(4) is a rule that **nothing can convict** — precisely what `#norm-declaration-must-convict` calls a wish. The rule that would have prevented the loss is itself unenforceable, and no instrument could tell anyone it had been broken.
2. `CONSOLIDATION-STATUS.md` line 9 instructs "**do not re-mine ice for claims**" — the sentence that would have prevented this audit. It ran only because Joseph attached a condition that overrode it.

## 2. Method

Two directions, because they find different things.

- **Core-side.** All 83 segments in `core/src/` read in full. This direction cannot find absences — it follows threads outward from what core already contains — and that limit is the reason for the second direction.
- **Source-side.** Four agents read disjoint slices of the historical corpus (`from-plan`, `from-archive`, toolchain/architecture/consolidation-reports, and the live `msc/` adjudication layer), each briefed without the coordinator's expectation, looking for thoughts with no core home.
- **Verification.** Load-bearing claims re-checked against the primary before being reported upward.

A methodological note worth keeping: the initial file inventory was built by grepping the coordinator's own guess at what "workflow / CLI / harness" means. It missed `.super-archive/from-plan/vivium-operational-workflow.md`. Term-list inventories inherit one vocabulary.

## 3. Verdict

Tiered, because the honest answer differs by layer.

- **Law-shaped material survived well, and in several places was strengthened by the peel.** `#form-depend-by-key-never-latest` states build-order independence more crisply than its source `[relayed]`; `water-parallelism` peeled essentially complete `[relayed]`. `git log --diff-filter=D` shows no toolchain- or architecture-class file was deleted; graduations were `git mv` `[relayed]`.
- **One class thinned or vanished:** time, progress, build-sequence, and the observer's experience.
- **No evidence of destroyed thinking.** Everything identified as missing from `core/` still exists in `.super-archive/` and in git history. The cost of the losses is that nothing points at them, not that they are gone.

## 4. Verified losses

**Component E — time-indexed stage chains.** `[verified: .super-archive/from-archive/TODO.md`, §"Component E"`]` The archive note records the loss as accidental in its own words: the component "did **not** carry into [the] six phases, and no deliberate deferral is recorded — it fell through the consolidation crack (found 2026-07-10)." It further names what it is for: making a build stage *addressable* (beacons need it), making intermediate stages monitorable by explorers, playing back "watch erosion happen while floating downstream," and carrying recorded convergence-$\varepsilon$ as the unLawfulness budget in data form.

The compounding property: that note is itself now in `.super-archive/`. The warning was archived along with the thing it warned about, and no `#gap` row was written, so `#disc-open-problem-census` — which derives only from OUTLINE gap rows — is structurally unable to surface it.

**Per-module source attribution.** `[verified: .super-archive/from-plan/abyssal-parity-plan.md`, Phase 1`]` The specified design was "a `build.rs` that hashes **each kernel module** into a compile-time key component — the Nix move," with the acceptance probe "edit a kernel → exactly its dependent cone invalidates and nothing else." What shipped is a whole-crate digest.

**Correction, from the session transcript** `[verified: ~/.claude/projects/-Users-josephwecker-v2-src-arch-vivarium/8145183f-….jsonl]` — this is **not** an unconsidered gap, and an earlier draft of this document framed it as one. The implementing session weighed per-nomos and declined it for a stated reason: "per-nomos attribution needs a hand-maintained source→nomos file list, which *is* a bump-discipline surface — forget to add a newly-shared helper and you under-key again, reintroducing the exact hazard." The brief it answered had already flagged the uncertainty honestly and labelled it "my guess, not your work order." The deliberation was sound and it identified the correct hazard.

What stays open is narrower than "core rebutted a strawman": the session declined a **hand-maintained** map, while `#form-complete-content-addressed-key` KI(3) names the unblock condition as attribution becoming **derivable** from the module graph. Whether those are meaningfully different, or whether the shared-helper problem (`noise.rs`, `measure.rs`, `sphere.rs`, `gen.rs` feed many nomos through no declared `deps` edge) defeats both, is the live question. Under second opinion.

**Method note this produced.** Commits carry `Claude-Session` trailers and DECISIONS entries carry `:session` UUIDs that map onto transcript filenames in the Claude project directory (Joseph, 2026-07-24). The artifacts record *what* was decided; the transcripts record *why*, including options considered and declined that never reach a segment. An audit that reads only artifacts will mistake considered trade-offs for oversights — as this one did until the transcript was read.

**`restart-in-place`.** `[verified]` The name survives in `#form-builder-admission` Known-incomplete (2), which points at `#detail-builder-daemon`; that segment does not contain it. The content lives at `.super-archive/from-plan/builder-explorer-decoupling.md`: "edit the spec → keys change → the builder offers *restart-in-place* (recompute exactly the invalidated cone) or *fork* (new spec; forks may share one object pool — the seed is in every key, so worlds cannot alias — giving cross-fork dedup for free)."

**Legible, steerable time regimes.** `[verified]` A Joseph intent from 2026-07-02 (`.super-archive/from-archive/TODO.md`, "Explorer intents"): "legible, steerable time regimes (in-world clock vs wall clock, sim rate vs framerate — pre-history vs current sim)." A grep for those phrases across `core/` returns zero. `#form-temporal-lod-regimes` is adjacent but answers a different question (what machinery an aspect needs, not what clock the observer reads).

**The round-trip probe.** `[verified: .super-archive/from-archive/architecture-migration-2026-07-03.md]` "Checkpoint round-trip probe (resume vs run-through must agree — promote the two-leg cache test from anecdote to instrument)." `#form-depend-by-key-never-latest` states that law at `status: exact` with no named convictor — a live tension with `#norm-probes-before-claims`.

**`watchpoint` has no LEXICON entry** `[verified]` while `beacon` and `focus` both do — and it is the observer-facing member of the three.

**Two Joseph decisions with no core citation** `[verified]`, both `:topic` values whose whole bucket is uncited:
- `DECISIONS[ascii-globe-in-info-colored-by-state]`, `:by joseph :status wish`, 2026-07-12 — build-state legible at a glance, "coloring regions by what they have reached in SPACE and TIME/PHASE." The **space** half is delivered in `vivarium info`; the time/phase half is not, and needs Component E to be expressible.
- `DECISIONS[new-system-must-reach-the-goal-not-reimplement-poc]`, `:by joseph :status decided`, 2026-07-12.

Also `[relayed]`: "the animation is the memo sequence" and its live-watching mechanism (poll roots, no IPC); worlds-have-genealogy as a derivation; the manifest-template line-to-hold (rigor floors belong on the manifest, never the ordinum); the global-confidence-scalar rejection; the store-as-bus multi-process validation of 2026-07-10.

## 5. Verified contradictions inside core

These are not archive losses — they are places where core disagrees with itself, and they were found because the audit went looking.

1. **`#form-complete-content-addressed-key` contradicts itself.** `[verified]` Its Discussion promises "change a nomos version and exactly its dependent cone invalidates; everything else stays." Its FE(4) and Known-incomplete (3) state that editing any `.rs`, comment included, invalidates the whole store. A measured rebuild confirms the FE: one comment appended to an unrelated module took a rebuild from 8 ms (24/24 hits) to 5.1 s (0 hits).
2. **The acceptance probe versus the deferral.** `[verified]` `#detail-abyssal-parity-build` FE(4) Phase 1 makes granular invalidation a probe; `#form-complete-content-addressed-key` KI(3) and `#detail-epistemics-toolchain` FE(5) defer it. Neither cites the other. The stated ground for deferral — that a hand-maintained source→nomos map would reintroduce bump-discipline — answers a weaker proposal than the record made (§4, per-module). *Under second opinion as of this writing.*
3. **Source-derived versions: two segments call future what a third calls landed.** `[verified]` `#form-in-vivia-citation` and `#form-add-system-contract` FE(4) both describe source-derived nomos versions as "the target"; `#form-complete-content-addressed-key` FE(4) and `#detail-epistemics-toolchain` FE(5) describe the build-time digest as wired and probed. One of these is stale, and the stale one is the citation segment.
4. **`--epochs` sits where core says it must not.** `[verified]` `#form-core-view-wall` FE(4) uses "how many erosion epochs to run" as its example of a parameter authored by "builder / law / **manifest** paths." It lives on the global CLI. `#form-manifest-prescribes-vivium` FE(5) is honest that the manifest is "designed; partial implementation."

## 6. Structural diagnosis

Two independent accounts converged, and they are compatible rather than competing.

**By claim shape.** Every loss is the same shape: none of them is a law. The peel routed each idea to the segment owning its law, which is why law-shaped material survived; build-sequence and experience claims had no owning law and no comfortable slot in the Formal-Expression cadence.

**By segment ontology.** `[verified]` The retired 2026-07-13 outline (`.super-archive/from-archive/core/OUTLINE.md`) used types that no longer exist: 7 `contract` rows, 4 `mode` rows, 3 `mechanism` rows. The `mode` rows are a verification taxonomy — static audit, invariant probe, fitness-versus-declared-prior, and **control**, the last noting that every real finding of the 2026-07-13 audit day came from a control run. Those types were dropped when the ontology aligned to ASF's. `FORMAT.md`'s current type table has none of them, and `FORMAT.md` Open Question 1 now asks for one of them by name: "A nomos has *contracts* … and there is no ASF word for that."

**By indexing rather than content.** `[relayed]` The consolidation was prose → segments and was never DECISIONS → segments. `DECISIONS.decision-log.udon` is a `#scope-segment-canon` FE(3) carve-out, so it is authoritative and largely uncited: `:topic tooling` and `:topic view` rows have zero core citations. Two of the findings in §4 were hiding there. Nobody owns that reconciliation.

## 7. Open

- Second opinion in flight on §5.2 — whether per-nomos source attribution is derivable well enough to beat core's stated bar, or whether the `#detail-vivium-lifecycle` BREAK-1 route (phases as memoization boundaries) is the honest path.
- Second opinion in flight on §6's ontology hypothesis — whether the dropped types caused the thinning, and whether a cheaper intervention than schema change would have caught this class.
- Whether `DECISIONS` gets an owner, and on what cadence, is Joseph's call.

## 8. What the coordinator takes from this — marked as one read, not a finding

The four asks Joseph named — background building toward points of interest, restarting at the right points, watching the edge live, watching replays — reduce on this corpus's own account to **one object plus one reader**: time-indices in the key with $\varepsilon$ recorded per stage (Component E), and a poller over roots. Replay and live-watching are the same mechanism, differing only in whether new time-indices are still landing; designed as two features, they become two.

That reduction is the practical payoff of the audit, and it is why the audit was worth running before the design rather than after.
