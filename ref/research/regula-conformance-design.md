# The regula — world-level conformance, made as auditable as the nomoi

*2026-07-10, from Joseph's proposal (verbatim intent: a "charter that says this vivarium attempts to follow such-and-such… therefore it also models magnetic fields and solar flares and aurora and has high-fidelity ozone and therefore UV insolation simulation… as auditable as the rest… in order to have in-vivia experiment results with rigor and replayability where finding a latent flaw in a nomos can easily be seen to have changed the experimenter's results or not immediately"), strengthened/refined/critiqued by the session. Status: **design** — the concept is Joseph-initiated and session-endorsed; the name is Ⓝ (see §6); v0 implementation shape in §5.*

## 1. What it is — the third layer, completing a stack that was two-thirds built

| layer | declares | exists as |
|---|---|---|
| **nomos declaration** (nomotheke, built) | one algorithm's epistemics: tags, deps, bequests-with-conservation, assumption anchors | data + tests |
| **slot** (new, small) | a named system-role in the world-assembly, identified by its **flux interface** (ARCHITECTURE §9): what any occupant must produce/consume. Nomoi are candidate occupants at different rungs — the same slot filled at different epistemic rigor/performance | today: the nomotheke's `system` field, promoted to first-class |
| **regula** (new) | the world-level conformance profile a vivium declares it follows: which slots MUST be filled at what minimum rigor, which absences are permitted (and on what condition), and the epistemological posture — what in-vivia results under this regula can and cannot claim | data + computed conformance |

This reifies **ARCHITECTURE axis 3** (use-case as fidelity contract, LEXICON §5 Ⓝ) and gives LEXICON §4's reserved build-stack term **Primary Invariants** its concrete form. The manifest cites `regula@version` (identity bucket: pre-Realization the law is mutable and the regula may be revised; post-Realization a regula change forks, per §7.2 law-=-identity).

## 2. Phase-expectations: PHASES.md is a floor

Joseph's standing assumption, adopted: the phases doc's charges are a **floor until proven otherwise**. Encoded: the regula's per-phase slot minima **must be ≥ the PHASES charges for its target phase**; a regula may extend, never undercut. (A deliberately non-Earthlike regula — a fantasy world — may declare non-conformance to PHASES explicitly; what is forbidden is *silent* undercutting.) Phase-expectations are therefore a **table derived from PHASES.md** — the #gate/#earth/#mech/#emergent tags mapped to slots — and PHASES.md's prose stays the human-readable source.

## 3. Permitted absences — the canceling pairs made auditable, with tripwires

PHASES.md design notes already carry the principle in prose: a canceling pair (magnetic field × solar wind; Jupiter-shepherding × impactor flux; the deferred moon) "stays jointly unbuilt at zero fidelity cost **until a consumer queries the difference or either member alone**." That condition is *mechanically checkable* against the nomotheke:

> **A permit = (absent slots, validity condition).** The validity condition names the fluxed quantities whose consumption voids it. When any nomos declares a dep/input in the permit's void-set (auroras, compass bearing through a real field, UV-at-surface, tides, moonlight), the audit reports the permit **voided** — the regula then demands the slot be filled *before* the consuming nomos may land.

This converts Joseph's "there will be points where we cannot make the law self-consistent without prior world-level requisite nomoi" from a discovered incoherence into a **computed error at declaration time**. The moon is the archetype (declare-causally-materialize-lazily already reserved its parameters); the ozone→UV chain is the worked example (a UV-consuming nomos voids the ozone permit, which voids the atmosphere-composition permit, which is exactly the "high-fidelity ozone and therefore UV insolation" cascade Joseph named).

**The motivating specimen is already in the store**: `water-tile` consumes precipitation with **no atmosphere slot filled** — rain without a sky. Under any Earth-track regula this is a voided-permit condition *today*. The current honest reading: the pre-regula world runs, in effect, permits nobody wrote down; the regula's first job is to write them down (grandfathered, dated, with their retirement conditions — the reservoir layer).

## 4. In-vivia experimental rigor — results are memos with cones

The payoff Joseph specified: find a latent flaw in a nomos → see **immediately** which experimental results it touches. The mechanism is already built, just not yet applied to results: **an experimental result is itself a memo** — a query over the world plus an analysis nomos, with a complete key naming its whole dependency cone (vivium manifest → regula@version → nomoi@versions → seed → the keys it read). Then:

- *Flaw found in `nomos@v`* → invalidation query: which result-memos have `nomos@v` in their cone? → affected results flagged **by the same machinery that invalidates tiles**. Nothing new to build conceptually; the cone is the audit.
- Replayability is inherited: the result's cone is content-addressed, so re-running under the corrected nomos version is *exactly* the recompute the store already knows how to scope.
- This is ASF.md §7.5's cross-bundling ("studies couple to explicit vivia") given its enforcement layer: a result citation isn't a footnote, it's a key.

## 5. Implementation v0 (thin, same pattern as the nomotheke)

A `regula.rs` sibling to `nomotheke.rs`: `Slot { name, fluxed_interface }`; `Permit { absent_slots, void_on: &[quantity], note }`; `Regula { name, version, target_phase, slot_minima: &[(Slot, Tier)], permits, posture: &str }`; static `EARTH_TRACK_V0` declaring today's honest state (three slots filled; permits for moon, magnetosphere×solar-wind, atmosphere — the last grandfathered-with-retirement-note). Conformance = a function `(Regula, NOMOTHEKE, store census) → report`, printed by `vivarium status`; permit-voiding checked in tests the same way ledger anchors are. **Do not build the full machinery ahead of the fourth nomos** — land the type + `EARTH_TRACK_V0` + the voided-permit test when the reservoir/thermal work starts (it is that work's specification).

## 5b. The second chapter — the exo-facing constitution (Joseph, 2026-07-10: "the ultimate claim on interaction potential with other kingdoms")

The regula is a kingdom's constitution in **two chapters**, both auditable:

**Chapter 1 — fidelity (internal):** §§1–5 above — slots, rigor minima, permits, posture.

**Chapter 2 — participation (exo-facing):** the declared interaction surface of the regnum:
- **Access-matrix configuration** — LEXICON §7's "any structure is configured per-vivium" finally has its *where*: engagement modes admitted (§7.4), access profiles grantable (§7.5, incl. the Sc-9 access-rich/participant split), realizability requirements for admission at each awareness scale (§8).
- **Regula nesting = the Sc-5 answer**: kingdoms nest, so regulae nest — **a child kingdom's regula is constrained by its parent's** (an endo author can only issue a regula within the envelope its own kingdom's regula grants). Moratorium-recursion resolved by inheritance-by-constraint, auditable per level.
- **Stewardship duties as clauses**: memo-retention (never discard a memo of a world that ever hosted a mourning-capable inhabitant — workflow rule 7), the redeemer condition, fork/patch semantics (mutation-log compatibility), all declared and versioned.

**Supremacy clause (load-bearing, stated once):** the **moratorium is not within the regula's gift** — it is program law (ASF.md §0) sitting *above* the regula layer, a constraint on the space of valid regulae. No regula can permit endo instantiation of emergence-capable substrates; the agent-seam slots are unoccupiable by such in any valid regula, and the conformance audit checks it mechanically. The regula is the moratorium's *enforcement locus*, never its negotiation surface.

**Etymology sealing the naming (Joseph's find):** *regula* (the straightedge — the rule) and *regnum* (the realm — the ruled) are cognates from PIE *h₃reǵ- via* regere*: the regula is the rule by which a regnum is a regnum. "Kingdom," the settled LEXICON term, is English for regnum — the pairing is the word-family's own division of labor.

## 6. Naming — Ⓝ, Joseph's call

"Charter" collides with the Archema program's `CHARTER-DRAFT.md` (the program constitution) — a real cross-member confusion risk. Leading candidate: **regula** (pl. *regulae*) — the monastic *rule a house declares it lives under* (Regula Benedicti): a community-adopted, versioned, conformance-audited law-profile; classically harmonious beside *nomos* (the laws) and *nomotheke* (their registry); clean English mouthfeel; zero collisions in-project — and now with the §5b cognate seal: regula/regnum are one root, so the term binds the artifact to **Kingdom** itself. Alternatives: *covenant* (heavier relational/faith register — may be right, Joseph's call given the faith-side-inclusion question is open program-wide), *conformance profile* (plain, cold). Descriptive gloss either way: "the world's conformance charter."

## 7. Critiques (honest, held against the design)

1. **Prose-wish risk**: a regula that isn't computed against the census is PHASES.md with extra steps. Mitigation is constitutive: it ships only as data + conformance function + tests, like the nomotheke. No prose-only regulae.
2. **Prematurity risk**: three nomoi, one representation kind — is a conformance layer YAGNI? Countered by the live specimen (§3): the rain-without-atmosphere incoherence exists *now*, and the reservoir/thermal work needs a specification *anyway* — the regula is that spec. Thin v0, no machinery beyond the first real check.
3. **Slot-granularity ambiguity**: is "climate" one slot or five? Rule adopted from ARCHITECTURE §9: **a slot is identified by its flux interface** — if consumers couple to it as one interface, it is one slot; internal decomposition (EBM vs GCM) is rungs within it; separate interfaces (atmosphere-reservoir vs circulation) are separate slots.
4. **Double-bookkeeping risk**: phase-expectations vs PHASES.md drift. Mitigation: the regula's floor table cites PHASES.md charge-by-charge; a lint (later, with `bin/term`-style tooling) can diff them.
5. **Posture overreach**: "what in-vivia results can claim" must not become a blanket stamp — a result's *actual* epistemic state is its cone-fold (§4), and the regula posture is only the ceiling. Both get stated; the fold wins.
