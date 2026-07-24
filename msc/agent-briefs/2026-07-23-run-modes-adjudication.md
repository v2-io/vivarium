# Run-modes carve — adjudication

*Audience: parent session. Not claim canon. 2026-07-23.*  
*Inventory row #4 treated as hypothesis, not a task to complete.*

> **Parent re-weight (Joseph 2026-07-23):** Absence of a `RunMode` type in crates is **not** the primary reason a thin carve would overclaim. Overclaim risk is **LEXICON `:status open`**, **no DECISIONS row**, and plan “DECIDED” ≠ ratified authority. Code lag is expected compliance debt once a thin formulation lands.

---

## Verdict

**Thinner claim is warranted; full promote of inventory `#form-run-modes-carve` / `#run-modes-referents-pinned` as settled machine law would overclaim.**

| Option | Finding |
| --- | --- |
| **Already owned** | **No** as a sole home. Pieces are owned elsewhere (see §Already owned). OUTLINE §III gap remains correct. |
| **Full promote** (four named modes + decided isolation + release axis) | **Overclaim.** Names open; no `DECISIONS` entry; no code typing; Phase-0 “provisional banner” ≠ live `provisional` roots. |
| **Thinner claim promote** | **Yes, if Joseph wants a segment now** — referents + orthogonality + convention-only guard + honest non-identity with builder provisional. |
| **Gap stays missing** | **Acceptable** until first graduation or two-process store sharing (the tripwires). LEXICON + `#form-store-as-save` FE(8) already prevent the worst dual-home lie if agents read them. |

**Default recommendation to parent:** do **not** close the OUTLINE gap with a fat FE that sounds enforced. Either (A) land a **thin** formulation under hostile constraints below, or (B) leave the gap and demote plan prose to “pointer + residual,” without inventing a second home that pretends more than code and DECISIONS support.

---

## What is actually true today

### 1. LEXICON (dictionary face)

`|term[run-modes]` in `LEXICON.udon` §3 — **`:status open`**.

Pinned referents (names deliberately not settled):

1. **strictly temporal-causal runs**
2. **replays from pinned generators**
3. **discardable iteration runs**
4. **live play accruing history**

LEXICON’s own map:

- strictly-causal / replay / live-play → **Closed vs Open-with-recorded-forcing** (`|term[closed-open-kingdom]`, settled, §7.2)
- discardable iteration → **pre-participation non-intervention** (`|term[engagement-modes]`, settled, §4)

Related settled (or carved) terms that are **not** run-modes:

| Term | Status | Relation |
| --- | --- | --- |
| `engagement-modes` | settled | Orthogonal registers (Simulation / Explorer / Steward / Avatar…) |
| `interventional-scaffolding` | settled | Pre-participation arrange-then-discard; “never canon” |
| `participation-activation` | settled | Law locks; mutations begin to persist |
| `closed-open-kingdom` | settled | Closed / Open / Open-with-recorded-forcing |
| `realized-kingdom` | settled | Law immutable only |
| `Deployed` | reserved | Explicitly *not* Realized; naming of release head open |

**Truth above “settled” labels:** the *referent carve* is real in the dictionary; the *names* and the *store typing job* are not settled. A segment that says “run-modes are settled” is false. A segment that says “four kinds of ‘running’ are distinct and must not share one root-write policy” can be true.

### 2. DECISIONS

**No** `DECISIONS.decision-log.udon` entry for run-modes, canon-root guard, causal/iterating, or Phase 0.

Implication under `#norm-decision-authority`:

- abyssal-parity-plan’s banner **“DECIDED 2026-07-10”** is **plan-layer attribution**, not a DECISIONS ratification.
- Candidate inventory’s “decided-by-joseph (2026-07-10)” is **not re-verified in DECISIONS** — treat as plan-reported Joseph steer, re-check before any segment tags authority as `us` / `joseph`.
- Do **not** launder plan “DECIDED” into segment “settled law.”

### 3. Plan / design substrate (sources, not canon)

**Primary narrative:** `doc/plan/abyssal-parity-plan.md` Phase 0 (2026-07-10).

Substance of that Phase 0 text (paraphrase, not promote):

1. Store must *eventually* type runs so iteration cannot write canon `roots`.
2. **Now:** convention-only for the **canon-root guard** — nothing graduated yet, so nothing to guard.
3. Stand-in honesty: a **`provisional` status banner** (“all state is *iterating*, not canon”).
4. **Tripwire:** mechanize refuse-at-canon-root-write at **first graduation** (first converged macro frozen as authoritative).
5. **Narrow scope:** convention-only applies *only* to that guard; **complete-key discipline stays mechanism-enforced always**.
6. Placeholder names: run kinds `causal` vs `iterating`; state lifecycle **iterating → candidate → realized**.
7. Refinement: **`causal` ≠ Realized** — causal is *root-write / isolation permission*, fluid; Realized/Deployed is a **separate release axis** (mutation-log compatibility / semver-for-worlds). Deployed naming open.
8. Stack sketch: Incomplete → Realized (law frozen) → Deployed (released + versioned), with causal/iterating orthogonal throughout.

Also:

- `DESIGN-REDUX.md` §12: iteration shares `objects/` safely by complete keys; **only** `roots`/tags are the integrity surface; “iteration must never write a canon root.”
- `doc/ARCHITECTURE.md` §5: run-modes as “store’s typing job” (pointer grade).
- `doc/plan/builder-explorer-decoupling.md` §3/§6: revisit convention-only when **two processes share a store** — tripwire may fire **before** first graduation.
- `doc/plan/framework-to-status-quo.md`: “canon roots vs scratch roots per the run-modes carve.”
- `vivium-operational-workflow.md`: does **not** own the carve; Open-with-logged-forcing appears as workflow doctrine, not run-mode typing.

### 4. Claim segments already in `core/`

| Segment | What it owns re: this topic | What it does **not** own |
| --- | --- | --- |
| `#form-store-as-save` FE(8) | **Frame:** only runs allowed to author authoritative state write canon roots; discardable iteration must not promote by accident; mechanization = compliance debt; “Phase-0 convention + provisional banners are present practice.” | Full taxonomy; naming; enforcement; release axis |
| `#form-store-as-save` Epistemic Status | Honest: **no run-mode enforcement**; provisional roots via `PutOpts`; residual guard debt | Conflates plan banner with builder provisional (see §Drift) |
| `#form-builder-admission` | Who may write; `--allow-unmet` → **provisional roots**; explorers observe-only pull | Run kinds; canon vs scratch modes |
| `#form-core-view-wall` | View does not author evolution; ethereal observe-only | Builder run isolation |
| `#form-complete-content-addressed-key` | Scratch and canon can share `objects/` because keys differ | Root discipline |
| `#disc-unlawfulness-budget` | Lists “run-modes carve” as sibling residual | The carve itself |

**Conclusion on “store-as-save already carries half”:** yes for the **one load-bearing root sentence** (don’t let iteration write canon roots). No for the **taxonomy, axes, lifecycle, naming, and tripwires**. Closing the OUTLINE gap by “already owned” would hide the unowned half.

### 5. Code (what is implemented)

**There is no run-mode mechanism.**

Verified absences (grep + module read):

- No `RunMode` / `run_mode` / `causal` / `iterating` type or CLI flag in `crates/vivarium-world/`.
- No canon-root write refusal.
- No scratch-root namespace separate from canon roots.
- `spec.rs` has no run-mode field (only “causal” as ordinary English in a comment about identity minting).

**What does exist (related, easy to misread as run-modes):**

| Mechanism | Location | Actual meaning |
| --- | --- | --- |
| Complete keys | `store::Key`, query puts | Isolation of *algorithm versions / inputs* — not run kind |
| `PutOpts.provisional` / root third line `provisional` | `store.rs`, `query.rs` | **Waived flux admission** (`--allow-unmet`); not “this process is iterating” |
| `World::set_provisional_writes` | `query.rs` + `bin/vivarium.rs` | Set only when phase unmet and waived; default path writes **non-provisional** roots |
| `vivarium status` provisional census | `bin/vivarium.rs` | Surfaces waived roots; message says “not lawful *in vivia* evidence” |
| Module honesty banner | `store.rs` L19–27 | Explicitly: **no run-mode canon-root guard yet**; provisional = flux waiver metadata |

Default `vivarium build` with met requisites: `set_provisional_writes(false)` → roots look **lawful / non-provisional** even though Phase 0 said “nothing graduated; all state is iterating.”

Builder vs explorer process split, demand spool, and full daemon remain plan-grade (`#form-builder-admission` Epistemic Status already says so).

---

## Critical drift: “provisional banner” ≠ “provisional roots”

This is the main place authority has outrun truth.

| Source | Meaning of “provisional” |
| --- | --- |
| Phase 0 plan (2026-07-10) | Global honesty banner: vivium/store still *iterating*; stand-in until canon-root guard |
| Live code + `#form-builder-admission` | **Per-root** flag for **flux-waiver** materialization |

They share a word and a moral (“do not treat as full law / full evidence”) but **different predicates**. A segment that says “Phase-0 provisional banner is present practice” (current `#form-store-as-save` FE(8) / Epistemic Status gloss) is **partially wrong** if read as “the run-mode stand-in is implemented.” What is present is **builder-admission provisional**, which does not tag iterating algorithm work as non-canon.

**Hostile correction for any promote:** state the non-identity in FE or Epistemic Status. Optionally fix store-as-save residual wording when a run-modes segment lands (or even without one — residual truth cleanup).

---

## Overclaim checklist (what a fat segment would get wrong)

1. “Run-modes are decided / settled” — LEXICON open; DECISIONS silent.
2. “Names: causal / iterating” — Phase 0 *placeholders*, not LEXICON settled terms; LEXICON still uses four referent phrases.
3. “State lifecycle iterating → candidate → realized is instituted” — plan only; not in store, not in DECISIONS.
4. “Provisional banner implements the carve” — false; provisional = flux waiver.
5. “Canon-root guard exists (even as soft convention enforced by tooling)” — **no tooling**; only docs + agent discipline.
6. “Scratch roots and canon roots are separate” — not in store layout; only complete keys separate *objects*.
7. Folding **Deployed / release / mutation-log semver** into the same FE as run kinds — Phase 0 itself says orthogonal; LEXICON keeps Deployed reserved.
8. Claiming Realized is a run-mode — Phase 0 refinement: **causal ≠ Realized**.
9. Treating candidate-inventory three-way split (`#run-modes-referents-pinned` + `#canon-root-guard-convention-only` + `#realized-vs-deployed-release-axis`) as one already-unified claim.

---

## Already owned vs residual (for OUTLINE bookkeeping)

**Owned enough not to re-claim:**

- Save ≡ store; pervasive memo; invalidation ≠ eviction → `#form-store-as-save`
- Complete keys protect shared `objects/` → `#form-complete-content-addressed-key`
- Builder admission + observe-only pull + provisional *flux* tag → `#form-builder-admission`
- View wall / ethereal → `#form-core-view-wall`
- Closed/Open, engagement-modes, scaffolding → LEXICON (dictionary; not segment homes)

**Still missing as claim home:**

- Explicit multi-referent carve (four “running”s) with names open
- Orthogonality: root-write isolation ⟂ Closed/Open ⟂ engagement ⟂ kingdom Realized ⟂ release/Deployed
- Convention-only canon-root guard + tripwires (graduation **or** multi-process share)
- Non-identity: builder `provisional` ≠ run-mode
- Placeholder lifecycle (iterating → candidate → realized) as **open process sketch**, not law

---

## If promote: thin FE under hostile read

Suggested slug: `#form-run-modes-carve` (formulation).  
Alternative if even thinner: keep OUTLINE gap and only tighten `#form-store-as-save` residual language — **not preferred** if the taxonomy keeps resurfacing as a dual-home temptation.

### Hostile FE (draft for parent / author — not landed)

```text
1. Referents, not names. At least four different activities share the phrase
   "running the world": (a) strictly temporal-causal evolution under law,
   (b) replay from pinned generators / recorded forcings, (c) discardable
   iteration (algorithm / parameter scratch), (d) live play that accrues
   interventional history. Names for these modes are open
   (#lexicon/term/run-modes). Do not treat a single CLI "run" or a single
   root-write policy as if it were all four.

2. Map, do not collapse. (a)(b)(d) relate to Closed vs Open-with-recorded-
   forcing (#lexicon/term/closed-open-kingdom). (c) relates to pre-
   participation non-intervention / engagement Simulation
   (#lexicon/term/engagement-modes). Interventional scaffolding is discard-
   by-definition (#lexicon/term/interventional-scaffolding) — adjacent,
   not a fifth root-write mode unless later pinned.

3. Root-write isolation is the store load-bearing half. Only a run class
   permitted to author authoritative world state may write canon roots.
   Discardable iteration must not promote into canon by accident. Complete
   keys already keep objects safe to share; roots/tags are the remaining
   integrity surface (#form-complete-content-addressed-key ·
   #form-store-as-save).

4. Convention-only until a tripwire. The refuse-on-canon-root-write guard is
   not mechanized. Present practice is human/plan convention. Tripwires
   (plan substrate, not DECISIONS): first graduation of authoritative macro
   state, or first multi-process share of one vivium store — whichever
   comes first. Complete-key discipline remains mechanism-enforced always.

5. Orthogonal axes. Root-write permission (plan placeholders: causal vs
   iterating) is not kingdom Realized (law frozen) and not Deployed
   (reserved release/compatibility head). Do not synonymize.

6. Non-identity with builder provisional. Root third-line `provisional`
   means waived flux admission (#form-builder-admission), not "this run is
   iterating." Default builds may write non-provisional roots while no
   system has graduated in the Phase-0 sense. Do not cite provisional
   census as run-mode enforcement.
```

### Epistemic Status constraints (must appear if promoted)

- **Max attainable:** `exact` only as architecture *under* “names open + guard convention-only + axes orthogonal.” Prefer `status: exact` with those assumptions explicit, or `discussion-grade` if Joseph has not re-ratified Phase 0 against DECISIONS.
- **Authority:** plan-reported Joseph steer (2026-07-10 Phase 0); **not** a DECISIONS `:by us` until recorded. Tag honestly.
- **Code:** cite `store.rs` honesty banner; admit no `RunMode`, no root refusal, provisional = flux waiver.
- **Stage:** `draft`.
- **Depends (suggested):** `form-store-as-save`, `form-complete-content-addressed-key`, `form-builder-admission` (for non-identity), maybe `form-core-view-wall` (engagement adjacency only).
- **Do not depend on / re-own:** Deployed naming; full builder daemon; mutation-log schema; unLawfulness ε as data (sibling).

### What **not** to put in the first promote

- Frozen English names for the four modes.
- `iterating → candidate → realized` as instituted lifecycle law.
- Mutation-log semver / Deployed as part of the same formal expression.
- Any probe claim that “iteration cannot write roots” (false today).
- “Present practice implements Phase 0” without the provisional drift correction.

---

## Probe / convictability note

`#norm-probes-before-claims` does not force a physics probe for an architecture carve, but:

- The **enforceable** half (canon-root refuse) is currently **unconvictable by construction** — no code path fails an iterating put to a root.
- A segment can still be exact-as-law **with compliance debt**, matching store-as-save’s pattern, **if** it does not claim the debt is paid.
- Strongest near-term convictable sibling remains complete-key isolation (already owned) and provisional flux census (builder-admission).

Do not invent a green test that asserts run-mode enforcement.

---

## Recommendation matrix for parent

| Goal | Move |
| --- | --- |
| Stop dual-home temptation; keep mass low | **Leave OUTLINE gap**; fix store-as-save residual wording that over-identifies provisional banner with run-modes; point to LEXICON. |
| Give residual EXTRACT #4 a real home | **Thin promote** with hostile FE above; OUTLINE row → draft segment; demote Phase 0 dual-home to pointer. |
| Close “Joseph decided it” anxiety | Open a **DECISIONS** entry (or explicit non-entry): restate Phase 0 with authority tag Joseph actually owns; separate “names still open.” |
| Wait for reality | **Defer** until tripwire (graduation or multi-process store); then promote with mechanism + probe together. |

Parent intent (“truth above authority — including if a Joseph convention is incomplete, misnamed, or not yet real in code”) favors either **thin promote with teeth** or **honest gap**. It does **not** favor “promoted because inventory said Joseph convention.”

---

## Feedback on this brief

What worked:

- Pointing at store-as-save / builder-admission / core-view-wall as partial owners prevented a pure greenfield hallucination.
- Flagging inventory row #4 as hypothesis matched the actual authority gap (plan “DECIDED” without DECISIONS).
- Asking for code truth forced the **provisional drift** finding — the highest-value result of the pass.

What could be sharper next time:

- Explicit ask: “Is provisional in code the Phase-0 banner?” would have been the first check; it is the load-bearing ambiguity.
- Split candidate-inventory rows (referents / guard / release axis) in the brief so agents don’t re-merge three claims by slug habit.
- Note that DECISIONS silence is itself evidence, not “search harder.”

Incidental findings (not scoped, recorded for parent):

1. `#form-store-as-save` FE(8) / Epistemic Status slightly overclaim “provisional banners are present practice” as run-mode stand-in.
2. Candidate inventory still labels Phase 0 “decided-by-joseph” without DECISIONS verification — same class of authority risk as this row.
3. Builder-explorer plan’s earlier tripwire (multi-process share) is **stricter** than Phase 0’s “first graduation only” — any thin segment should name **both** tripwires or say “whichever first.”
4. `Deployed` remains reserved in LEXICON; release-axis candidate is even thinner than run-modes — do not promote with this carve.

---

## Sources read (absolute)

- `/Users/josephwecker-v2/src/arch/vivarium/LEXICON.udon` (`|term[run-modes]`, §4 lifecycle, §7.2 Closed/Open/Realized/Deployed)
- `/Users/josephwecker-v2/src/arch/vivarium/DECISIONS.decision-log.udon` (no run-mode hits)
- `/Users/josephwecker-v2/src/arch/vivarium/core/OUTLINE.md` §III
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-store-as-save.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-builder-admission.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-core-view-wall.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-complete-content-addressed-key.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/disc-unlawfulness-budget.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/norm-decision-authority.md`
- `/Users/josephwecker-v2/src/arch/vivarium/doc/plan/abyssal-parity-plan.md` Phase 0
- `/Users/josephwecker-v2/src/arch/vivarium/doc/plan/builder-explorer-decoupling.md`
- `/Users/josephwecker-v2/src/arch/vivarium/doc/design/DESIGN-REDUX.md` §12–13
- `/Users/josephwecker-v2/src/arch/vivarium/doc/ARCHITECTURE.md` (store / run-modes)
- `/Users/josephwecker-v2/src/arch/vivarium/crates/vivarium-world/src/store.rs`
- `/Users/josephwecker-v2/src/arch/vivarium/crates/vivarium-world/src/query.rs`
- `/Users/josephwecker-v2/src/arch/vivarium/crates/vivarium-world/src/bin/vivarium.rs`
- `/Users/josephwecker-v2/src/arch/vivarium/core-segment-candidates-2026-07-14.md` (rows + note 12)
- `/Users/josephwecker-v2/src/arch/vivarium/msc/promotion-mine-2026-07-23-continuity.md` residual #4

---

## One-line for the parent session

**Promote only a thin, names-open, convention-only, axes-orthogonal formulation — or leave the gap; do not treat inventory #4 or Phase-0 “DECIDED” as settled code law, and do not equate builder `provisional` with the run-modes carve.**
