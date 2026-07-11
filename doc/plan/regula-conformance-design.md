# The regula — world-level conformance, made as auditable as the nomoi

*v2, 2026-07-11 (v1 of 2026-07-10 in git history). From Joseph's proposal (verbatim intent: a "charter that says this vivarium attempts to follow such-and-such… therefore it also models magnetic fields and solar flares and aurora and has high-fidelity ozone and therefore UV insolation simulation… as auditable as the rest… in order to have in-vivia experiment results with rigor and replayability where finding a latent flaw in a nomos can easily be seen to have changed the experimenter's results or not immediately"), strengthened across two dialog rounds. Status: **build-spec** — naming settled (§6); `regula.rs` v0 lands as the reservoir/thermal work's specification (TODO §Regula Terrestris v0). The concept's felt purpose (Joseph, 2026-07-11): the workflow-shaping pressure that stops us "adding various things on a whim like we would if it were just a game engine" — vigilance replaced by structure.*

## 1. What it is — the third grain of one honesty stack

| layer | declares | grain | exists as |
|---|---|---|---|
| **ASSUMPTIONS ledger** (built) | every unprincipled constant | value | prose table, compiled into tests |
| **nomos declaration** (nomotheke, built) | one algorithm's epistemics: tags, deps, bequests-with-conservation, assumption anchors | algorithm | data + minted keys + tests |
| **slot** (new, small) | a named system-role in the world-assembly, identified by its **flux interface** (ARCHITECTURE §9): what any occupant must produce/consume. Nomoi are candidate occupants at different rungs | system-role | today: the nomotheke's `system` field, promoted to first-class |
| **regula** (new) | the world-level conformance profile a vivium declares it follows: which slots MUST be filled at what minimum rigor, which absences are permitted (and on what void condition), and the epistemological posture — the ceiling on what in-vivia results under it may claim | world | data + computed conformance |

One enforcement pattern at three grains: declared as data → declarations mint keys / compile into tests → audit is a computation, never a review of intentions. This reifies **ARCHITECTURE axis 3** (use-case as fidelity contract, LEXICON §5) and gives LEXICON §4's reserved **Primary Invariants** its concrete form. The manifest cites `regula@version` (identity bucket: pre-Realization the law is mutable and the regula may be revised; post-Realization a regula change forks, per §7.2 law-=-identity).

## 2. The Ordinum — the phase floor, pinned and versioned

Joseph's standing assumption, adopted: the phase charges are a **floor until proven otherwise**. Codified: the **Ordinum** (full formal name *Ordinatio Ordinum* — "the appointed arrangement of the orders [of kingdoms]"; cited short as Ordinum, keyed `ordinum@version`) is the per-phase Charge/Bequest floor **as data**, derived from `doc/PHASES.md` — which is thereby the Ordinum's **reportatio**: the prose working text the authorized version is compiled from (the medieval ordinatio-vs-reportatio carve, Scotus precedent). Once codification lands, `doc/PHASES.md` archives as the honest first-guess prose.

Pinning discipline (this is what makes the floor survive PHASES' expected fluidity): a regula pins the `ordinum@version` it was audited against; **a PHASES revision produces a new ordinum version, never a silent re-grading of existing regulae** — they opt in by re-versioning. Integration-is-replacement, applied to the constitution itself. A regula's slot minima must be ≥ its pinned ordinum's charges for its target phase; it may extend, never silently undercut (a deliberately non-Earthlike regula declares non-conformance explicitly — what is forbidden is *silence*).

## 3. Permitted absences and the requisite closure — incoherence as a computed error

doc/PHASES.md design notes carry the principle in prose: a canceling pair (magnetic field × solar wind; Jupiter-shepherding × impactor flux; the deferred moon) "stays jointly unbuilt at zero fidelity cost **until a consumer queries the difference or either member alone**." That condition is mechanically checkable against the nomotheke:

> **A permit = (absent slots, validity condition).** The validity condition names the fluxed quantities whose consumption voids it. When any nomos declares a consumed quantity in the permit's void-set (auroras, compass bearing through a real field, UV-at-surface, tides, moonlight), the audit reports the permit **voided** — the regula then demands the slot be filled *before* the consuming nomos may land.

**Structural prerequisite found in code (2026-07-11):** `NomosDecl` declares deps (nomos→nomos) and bequests (quantities out) but has **no `consumes` field** (quantities in) — and permit-voiding is defined over consumed quantities. v0 adds `consumes: &[&'static str]`, with quantity names as shared constants referenced by both bequests and consumes, so a typo fails the build (same pattern as the ASSUMPTIONS anchors).

**The requisite closure is a planning query, not a discovered wall.** The audit computes the transitive closure of a *proposed* nomos before it lands: every consumed quantity must be either bequested by an occupied slot or covered by a live permit, and the report prints the full requisite tree (aurora → magnetic field + solar-activity cycle; UV-at-surface → ozone → atmosphere composition — Joseph's named cascade). Tool form, later: `vivarium audit --add <nomos>` — "if I add this, here is what the regula then demands." This converts "there will be points where we cannot make the law self-consistent without prior world-level requisite nomoi" from an incoherence someone trips over into a build planner.

**The motivating specimen is already in the store**: `water-tile` consumes precipitation with **no atmosphere slot filled** — rain without a sky. Under any Earth-track regula this is a voided-permit condition *today*. The pre-regula world runs, in effect, permits nobody wrote down; the regula's first job is to write them down (grandfathered, dated, with their retirement conditions — the reservoir layer).

## 4. In-vivia experimental rigor — results are memos with cones

The payoff Joseph specified: find a latent flaw in a nomos → see which experimental results it touches, at better-than-academic latency (his bar: patch, rerun, analyze next day). The mechanism is already built, just not yet applied to results: **an experimental result is itself a memo** — a query over the world plus an analysis nomos, with a complete key naming its whole dependency cone (vivium manifest → regula@version → nomoi@versions → seed → the keys it read). Two latencies fall out:

- **Touched — seconds.** Invalidation query: which result-memos have `nomos@v` in their cone? Flagged by the same machinery that invalidates tiles.
- **Changed — minutes.** Recompute the result under the corrected nomos and **compare hashes**: determinism + content-addressing make "did the flaw alter this result" an exact byte-level answer. A flaw that touched forty results and changed three is a different situation, and the store can tell them apart. The full corrected study is then exactly the scoped recompute the store already knows.

Replayability is inherited (the cone is content-addressed), and this is ASF.md §7.5's cross-bundling given its enforcement layer: a result citation isn't a footnote, it's a key.

**What results may claim (the posture, held precisely).** The regula posture only *caps* claims — a result's actual epistemic state is its cone-fold, and the fold wins. Split by what a claim is about: *about the model*, determinism makes reproducible results definitive, positive and negative alike; *about nature*, nothing is definitive — every claim travels only as far as the cone-fold plus stated transfer assumptions. The asymmetry runs by claim-type, and it favors the positives: **sufficiency results ("these laws can produce X") are witnessed by a running world; absence results ("X doesn't arise") are only as strong as fidelity + coverage** — absence may be our infidelity, not nature's verdict. (Settled with Joseph 2026-07-11, correcting the naive only-disprove reading.)

## 4b. The regula is a lineage — the moving-edge / pinned-set question, resolved

"Contains our best state-of-the-art nomoi as we go" and "citable/replayable" are reconciled by versioning, not compromise: **each `regula@version` is immutable and citable; the lineage head carries the current best.** This resolves LEXICON §4's open moving-edge-vs-curated-set entry completely (the sketch's placeholder phrases for it are retired — `SUPERSEDED.md` 2026-07-11), including its three-way sub-question — **interface stack = the slot set (regula) · algorithm = the nomos occupying a slot · target-machine = backend identity in the memo key** (the CPU-reference determinism policy already answered the third leg).

The big-picture instrument Joseph asked for falls out as a computed diff: `vivarium status` reports **conformance-to-pin** (is this world sound under the regula it declares?) and **gap-to-head** (how far behind current SOTA, slot by slot?). Big-picture legibility becomes a query, not a memory.

## 5. Implementation v0 (thin, same pattern as the nomotheke)

A `regula.rs` sibling to `nomotheke.rs`:

- `Slot { name, fluxed_interface }` · `Permit { absent_slots, void_on: &[quantity], note }` · `Regula { name, version, target_phase, ordinum, slot_minima: &[(Slot, Tier)], permits, posture: &'static str }`.
- `consumes` added to `NomosDecl` (§3); quantity names as shared constants.
- **`TERRESTRIS_V0`** (§6) declaring today's honest state: three slots filled (surface-prior, fluvial-erosion, surface-water); grandfathered dated permits — moon, magnetosphere × solar-wind, **atmosphere-with-retirement-note** (retired by the reservoir layer).
- The Ordinum derived from doc/PHASES.md **for the target phase only** — the aspiration-dump guard: a regula reaching for Phase 8 makes every report a wall of red, and an ignored dashboard is worse than none. Later phases render as *horizon*, never as failures. The lineage's `target_phase` advances as the work does.
- Conformance = `(Regula, NOMOTHEKE, store census) → report`, printed by `vivarium status`; permit-voiding checked in tests the same way ledger anchors are.
- **Minima only** (adopted 2026-07-11, session recommendation unopposed): the regula declares floors; "best available occupant" is the registry's answer, not the regula's — "best" is not total-ordered once performance and capability trade against rigor. Revisit only if a slot ever genuinely needs a recommended-occupant pointer.
- Sequencing: declaration-level conformance now; **artifact-level** conformance (folds over actual cones + convergence-ε) rides component E. Do not build machinery ahead of the fourth nomos — v0 lands *as* the reservoir/thermal work's spec.

## 5b. The second chapter — the exo-facing constitution

The regula is a kingdom's constitution in **two chapters**, both auditable:

**Chapter 1 — fidelity (internal):** §§1–5 above — slots, rigor minima, permits, posture.

**Chapter 2 — participation (exo-facing):** the declared interaction surface of the regnum: access-matrix configuration (LEXICON §7's "configured per-vivium" finally has its *where*), engagement modes admitted, realizability gates per awareness scale, stewardship duties as clauses (memo-retention, the redeemer condition, fork/patch semantics). **Regulae nest as kingdoms nest** — a child kingdom's regula is constrained by its parent's (the Sc-5 moratorium-recursion answer, inheritance-by-constraint, auditable per level).

**Supremacy clause (load-bearing, stated once):** the **moratorium is not within the regula's gift** — it is program law (ASF.md §0) sitting *above* the regula layer, a constraint on the space of valid regulae. No regula can permit endo instantiation of emergence-capable substrates; the conformance audit checks it mechanically. The regula is the moratorium's *enforcement locus*, never its negotiation surface. Terrestris carries it explicit-and-compliant; ASF §0's "for now" is load-bearing and names its own revisit conditions — lifting is future-us's to *earn* (vouchable Lawfulness, reversibility, consent-by-proxy stewardship), and no work here forecloses or presumes it.

## 6. Naming — settled 2026-07-11 (Joseph's calls; carve trail for the ledger)

- **regula** (pl. *regulae*) ✅ — adopted for in-anger use. The monastic *rule a house declares it lives under* (Regula Benedicti); cognate with **regnum** (PIE \*h₃reǵ- via *regere*) — the rule by which a regnum is a regnum, sealing the term to the Kingdom ontology. Known cost, accepted with open eyes: the *regolith* near-sound (unrelated roots — Gk. *rhēgos* "blanket" + *lithos*). If the association still grates after real use, the fallback is some other synonym — **explicitly not covenant** (Joseph, 2026-07-11). Rejected: *charter* (collides with the program CHARTER-DRAFT), *norma* (fatal normal/normative overload in a distribution-saturated codebase), *syntagma* (names the assembled thing, not the norm it is held to — that role is Law's ($\theta$); and *syntagmatic* is live linguistics vocabulary in a language-heavy program).
- **Regula Terrestris** ✅ — the first regula (slug `terrestris`): essentially-Earthlike-without-modeling-Earth — **B (physics) pinned high as slots fill; A (Earth-history) an anchor and sanity-check, never a target** (LEXICON §5's lawful-first-Earth-shaped-second, made a posture). *Terrestris* from *terra* as **the world** — earthly in full, sky and sea included, not soil. Collisions checked: species epithets only (*Tribulus terrestris*, *Bombus terrestris*) — lowercase-after-genus, no genuine confusion. Preferred over *Tellurian* (Joseph: wants more than solid ground).
- **Ordinum** ✅ — full formal name **Ordinatio Ordinum**; cited short as Ordinum, keyed `ordinum@version`. Grammar held honestly: standalone *ordinum* is a bare genitive plural ("of the orders"); the elided head noun makes the full form exact — deliberate ellipsis, Joseph's euphony call, and it lands his own phrase "orders of kingdoms" inside the name. Rejected: *taxis* (veto-grade in-domain collision — chemotaxis/phototaxis will someday *run in the world*), *ordinatium* (not a Latin form; corrected to *ordinatio*, then shortened per above). **Guard:** within the artifact the stages remain **phases** — "order" is not adopted as a stage-synonym (it would re-blur the sequence-vs-class senses the ordinum/regula split just separated; class-of-regna stays extensional: "Terrestris regna").
- **Register convention** (observed, kept deliberately): **Greek** names the law-article machinery (*nomos, nomoi, nomothete, nomothetic, nomotheke*); **Latin** the world-governance layer (*regula, regnum, ordinum*); **plain English** the lived concepts (Kingdom, Charge, Bequest, Record, slot, permit). Register signals layer; a proposal that crosses registers should notice it is doing so.

## 7. Critiques (honest, held against the design)

1. **Prose-wish risk**: a regula that isn't computed against the census is doc/PHASES.md with extra steps. Mitigation is constitutive: it ships only as data + conformance function + tests, like the nomotheke. No prose-only regulae.
2. **Prematurity risk**: three nomoi, one representation kind — is a conformance layer YAGNI? Countered by the live specimen (§3): the rain-without-atmosphere incoherence exists *now*, and the reservoir/thermal work needs a specification anyway — the regula is that spec. Thin v0, no machinery beyond the first real check.
3. **Slot-granularity ambiguity**: is "climate" one slot or five? Rule adopted from ARCHITECTURE §9: **a slot is identified by its flux interface** — one interface consumers couple to = one slot; internal decomposition (EBM vs GCM) is rungs within it; separate interfaces (atmosphere-reservoir vs circulation) are separate slots.
4. **Floor drift** — *resolved by §2*: the Ordinum is versioned and pinned; PHASES revisions re-version it; no silent re-grading. (v1's lint-the-diff mitigation is subsumed.)
5. **Posture overreach**: "what in-vivia results can claim" must never become a blanket stamp — the cone-fold is the result's actual state; the posture is only the ceiling; the fold wins (§4).
6. **The first audit forces a real decision on day one — which is the concept working, not a bug.** Terrestris' floor derivation for an early-Abyssal target reaches back through Phase 2's charges: atmosphere reservoir, closed water cycle, *and abiogenesis-through-photosynthetic-sea-life* (Abyssal's opening condition). We have zero biosphere. Either a declared low-tier reservoir-grade stand-in (photosynthesis as a box-flux in the reservoir layer — which would double as the domain-fixation guard's first non-field representation kind) or an explicit dated permit. Queued as the first output of the Terrestris audit; the decision is Joseph's.
