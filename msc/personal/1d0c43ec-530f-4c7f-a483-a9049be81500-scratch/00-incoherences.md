# 00 — the incoherence ledger

*Running list of contradictions found during the 2026-07-13 reading pass. **Not fixed as
I go** — fixing while reading is how you lose the reading. This is the triage input.*

## ⚠ The distinction that governs this file

Joseph, 2026-07-13, mid-read: *"neither the ordinum nor the nomos as written so far have
incorporated any of the higher level work and understanding that has begun to crystalize
in the last 24 hours or so."*

⇒ **Most of what looks like a defect in this repo right now is a DATE, not a mistake.** The
audit crystallized on 07-12/13; almost everything else was written on 07-11. So each entry
below is typed, and the types get *different remedies*:

| type | meaning | remedy |
|---|---|---|
| **PRE-AUDIT** | Written before the understanding existed. Not wrong — *early*. | **Revise when the scaffold absorbs the new frame.** Not a bug; an input to the redesign. |
| **STALE** | Says something the ledger has since *refuted*, and still reads as current. | Sweep. |
| **UNHEEDED** | Says something **true**, and the code ignores it. | Fix the *code*, not the doc. |
| **FALSE** | Actively asserts something untrue *right now*, that was untrue when written. | **Correct it.** These are the only ones that are actually errors. |
| **ORPHANED** | True, heeded, unreachable from any front door. | Re-link. |

---

## FALSE

### F1 — `DECISIONS[uplift-is-structurally-incapable-of-keeping-its-promise]` asserts a `:kept-by` that does not exist
*Found: reading the ordinum. Confidence: **verified in code**.*

The entry (2026-07-13, `:by claude :status proposed`) says twice that the ordinum assigns
`promise[emerged-land]` to `uplift`:
- *"`vivarium status` will report `uplift` as the keeper of `emerged-land` **forever**"*
- *"the promise the ordinum assigns **to exactly this nomos**"*

**The ordinum assigns it to nobody.** `promise[emerged-land]` has **no `:kept-by`**, and
`ordinum.rs`'s own test asserts it:
`assert_eq!(land.kept_by, None, "and NOTHING keeps it — the gap the ladder is pointing at")`.
ORIENTATION states it correctly (*"nothing keeps it"*); the decision entry does not.

**What survives:** `uplift.rs` really is strictly positive (verified); it really is the
natural keeper; it really could not keep the promise if named.

**What breaks — and it is the entry's proudest claim.** The meta-finding says a
range/reachability check *"would have fired on day one."* **It would not have.** A
reachability check needs a keeper whose range it can check. There is none. ⇒ **The
reachability check is PROPHYLACTIC, not DIAGNOSTIC** — it fires on the *future* failure
(someone writes `:kept-by uplift` and `status` reports `Claimed`). Still worth building.
Not what the entry says it is.

**Remedy:** correct the entry's framing; keep the finding. *(Claude may edit its own
`:by claude` entry — the log's rules explicitly allow in-place refinement.)*

### F2 — the fan/plume drift was measured WITH `p = 1.1` in the loop, and two mechanisms were never separated
*Found: reading DECISIONS end-to-end. Confidence: **the contamination is verified in code; the
MAGNITUDE is unknown and one run would settle it.***

> ⚠ **Scoped down after Joseph's correction (2026-07-13):** *"almost always later decisions are the
> more updated thinking when there is conflict or incoherence, whether or not it specifically
> remembered to say it superseded the prior."* ⇒ **The ledger-hygiene half of this entry is
> WITHDRAWN** — an un-noted supersession is not a defect, it is how the log breathes, and the
> later entry (`p` IS the bias) is the current thinking. **What remains is not bookkeeping. It is
> an unrun experiment.**

`DECISIONS[mfd-fan-is-a-bias-and-does-not-converge]` concludes: *"**the cube-face u/v axes are
ATTRACTORS** … MFD reintroduces precisely the grid-aligned-channel artifact it was adopted to
REMOVE."* Its numbers are charged against a perfect-lattice control reading **0.24°**, called there
*"MFD's own intrinsic error."*

`the-router-is-a-scalar-pretending-to-be-a-vector-and-p-is-the-bias` later proves that 0.24° **is
`p = 1.1`** — exactly zero at `p = 1` (a four-line theorem; measured rms 0.0000°, no attractors),
and at 1.1 *"a signed, 45°-periodic, GRID-LOCKED deflection pushing flow TOWARD THE GRID AXES …
eight attractors at exactly 0/45/90."* **On a perfect square lattice. No sphere. No Jacobian.**

**Verified in code:** `grid_lab/fan.rs:70` — `pub const P: f64 = 1.1` — applied at :295
(`(drop/d).powf(P)`); the plume probe passes `1.1` too.

⇒ **TWO mechanisms manufacture grid-axis attractors, they have the SAME signature, and every
routed number was measured with both of them switched on.** The perfect-lattice control does not
separate them: it was subtracted as a **scalar**, and the p-deflection is **45°-periodic in the
flow direction** — a *field*. On a cone the azimuth sweeps all 360°, so it does not subtract out.

**Scope, precisely — the geometry is p-independent and STANDS:** the 15.0° corner gap, its
invariance across L9/L19/L23, the closed-form Jacobian with no `N` in it, the area-weighted median
6.8°. Those are facts about **where the nodes are**. What is unseparated is everything computed by
**routing water through them with `p`-weights**: the **474 km plume drift**, the **4.00° → 5.75°**
growth-under-refinement, and the **attractor** conclusion.

> **⇒ THE PROBE, AND IT IS ONE CHARACTER: re-run the fan/plume at `p = 1.0`. What SURVIVES is the
> projection's shear. What VANISHES was the exponent. It is cheap, it is decisive, and it can fail.**

**Why it is worth running now rather than later.** `plate-tectonics-as-an-emergent-regime` makes
grid-locking a **precondition for interpretability**: *"FIX THE GRID-LOCKED BIASES FIRST — not
because they block the build, but because until they are fixed, THE PAYOFF RESULT WOULD BE
UNINTERPRETABLE."* If a large share of the measured grid-locking is the exponent, then a
one-character change buys a disproportionate amount of that interpretability. If almost none of it
is, the fan finding is **stronger** than it currently reads — it would have survived a contaminated
control. **Either answer is worth having.**

### F3 — the jitter proposal is RATIFIED at its weakest link
`bias-vs-noise-is-the-decisive-audit` (**`:by us :status decided`** — the ratified layer, cited by
CLAUDE.md) promotes fated jitter as *"a principled move: a BIAS → VARIANCE conversion."* Two later
`:by claude :status proposed` entries qualify it decisively — Mori–Zwanzig says its amplitude is
**derived, not chosen**; and `wavelet-store-…` says *"**JITTER FIGHTS JENSEN** … stream power is
SUPERLINEAR in S: injecting variance into S and applying a convex function **MANUFACTURES A NEW
JENSEN BIAS.**"*

⇒ **Variance may be injected where the law is LINEAR in it (drainage area `A`, a sum) and never
where it is CONVEX (`S`, via `Sⁿ`).** That qualification **exists nowhere in the ratified layer**,
and the ratified layer is what a fresh agent reads first.

---

## PRE-AUDIT (inputs to the redesign, not bugs)

### P1 — the ordinum's schema has outgrown its own `|meta[reading-rules]`
`|meta[reading-rules]` declares **charges · promises · defeasances · record**. The phases
also use `:regime` (all), `:target` (6/7/8), `:sub-gates` (Abyssal), `:opened-by` (×2), and an
entire `|obligation` block (phase 7). Two days old and already growing past its schema — which is
what a live schema does.

> ⚠ **SELF-CORRECTION (after reading LEXICON): I claimed `:target` was "defined nowhere; I cannot
> tell you what it means." That was FALSE.** It is defined — `LEXICON.udon` §1 `|term[target]`:
> *"Phases 6/7/8 as TARGET 1/2/3 — the playable entry points."* **I asserted an absence from one
> file's silence, having not yet read the file where the definitions live.** The real observation
> survives and is much smaller: the *ordinum's own reading-rules* do not carry the field, so the
> artifact is not self-describing. *(Recorded rather than quietly edited — it is the project's own
> failure mode, committed by me, in the ledger built to catch it.)*

### P2 — charge/promise "subset" language does not match the data model
Reading rules: a charge is *"the GATING SUBSET of its promises"*. But they are separate
entities in separate blocks, linked by `:from`; nothing expresses containment. **Latent
better model** (mine, speculative): **a charge is a PROCESS (`emergENT-land`), a promise is
a STATE (`emergED-land`) — the participle map.** If so, "subset" is the wrong metaphor and
the one-letter slug pair is a *feature that should be a lint*, not a trap.

### P3 — the ordinum is a law that cannot currently be evaluated
*"A phase OPENS AT the delivery of the previous phase's charges"* ⇒ phase advance is a pure
function of promise **maturity**. The maturity engine exists (`ordinum.rs`); **the report is
not wired into the CLI** (known, owed). ⇒ **The world cannot presently be asked which phase
it is in** — in a project whose standing guard is *check the ladder, not modern Earth*.

### P4 — 27 of 32 promises are glosses; 23 have no predicate
By the ordinum's own rule (*"ABSENCE = GLOSS: an honest word-of-honor not yet mechanized"*).
**Not a defect — the honest state, mechanically legible.** But it means the derived "nomos
TODO list" Joseph wants is currently **5 edges wide**, and the real backlog is
**specification work**: 23 promises cannot be held to anyone until they have predicates.

### P5 — the ordinum's generality claim has never been probed
It claims *"the SCHEMA is world-kind-agnostic; only the CONTENT is Earth-lineage"* and names
the intended second instance (a CA / 2-D testbed, `:manifold euclidean-plane-2d`). **Nobody
has written it.** This project's own discipline (`hydrosphere` was built as a reservoir-box
*specifically* to prove the framework wasn't secretly field-on-a-grid — the domain-fixation
guard) says an unexercised generality claim is a wish. **Cheap decisive probe: write a
30-line second ordinum and see what breaks.** *(My guess at what breaks: `:record` — a
**geological** concept sitting in the SCHEMA rather than the content.)*

---

## The missing primitive (appears three times — same hole)

### M1 — there is no way to say "this is VOID" in any ledger except the ordinum
- `LEXICON.udon` — `:status` **has no value meaning *retired***. `regula` is retired and
  still reads `settled` (13-edge hub node). *Known blocking sub-decision.*
- `ASSUMPTIONS.md` — **no "refuted" state.** `p = 1.1` is no longer merely unprincipled; it
  is **known wrong** (exactly-zero first moment at `p=1`, a theorem). The ledger cannot say
  so — **and `erosion.rs` still consumes it.**
- `DECISIONS.decision-log.udon` — has `:supersedes`, which is *replacement by a successor*.
  It cannot express **"void, and nothing takes its place."**

**⇒ `|defeasance` — already built, in the ordinum, on 07-11 — is exactly this primitive**,
and it comes with the audit rule that makes it bite: *"a defeased promise whose `:kept-by`
nomos is still consumed by a live nomos is an INCOHERENCE."* **Generalize it to every
ledger.** A defeasance-aware ASSUMPTIONS would **fail the build on `p = 1.1` today.**

*(This is my candidate for the highest-leverage small scaffold build found so far. Same
shape as the five boxes: a primitive that exists in one place, generalized to the places it
was always about.)*

---

## STALE

*(none yet beyond what I already fixed in ORIENTATION this morning — the grid decision, the
"unbenchmarked" hot-loop cost, the corner-anisotropy story, and the conservation≠consistency
framing. Committed `d904994`.)*

## UNHEEDED

### U1 — `DESIGN-MATERIAL.md` §4 states the conserved-primitive law; `uplift.rs` violates it
*"The conserved primitive is volume/mass of material — NOT a height."* `uplift.rs` produces a
height rate. **The doc is right and old (07-11, and §4 is older); the code ignored it.** Not
stale — **unheeded.** *(Not yet read in full — flagged from the isostasy decision, which
quotes it. Verify on read.)*

### M2 — the decision log needs `|defeasance`, not just `:supersedes`
`:supersedes` expresses **replacement by a named successor**. It cannot express **"this claim is
partially VOID and nothing replaces it"** — which is exactly F2's situation (the attractor claim is
not superseded; it is *contaminated*). The ordinum's `|defeasance :voids "<x>" :by <cause>` is the
stronger primitive and it already exists. **Same hole as M1, fourth instance.**

### M3 — the LEXICON's `|rel` graph has 2 DANGLING EDGES and no closure test
84 terms, 58 `|rel :to <slug>` edges. **Two point at terms that do not exist:** `manifest`
(pointed at by `beacon` *lives-in* and `vivium` *individuated-by* — and it is the noun the **regula
collapsed INTO**) and `nomotheke` (pointed at by `promise` and `nomos`, both *declared-in* — and it
is **the scaffold itself**).

⇒ **The two undefined terms in this vocabulary are the thing the architecture just collapsed into,
and the thing the architecture IS.**

**And the mechanism that would have caught this exists thirty lines away**: `flux.rs` has
`is_in_vocabulary()` + `flux_vocabulary_is_closed`, which **fails the build** if a nomos names a
quantity that is not a term. **`|rel :to X` has no such check.** Same vocabulary-closure discipline,
applied to the flux alphabet but never to the vocabulary itself.

### M4 — `permit` / `defeasance` / `canceling-pair` are ONE mechanism, and the LEXICON already says so
`|term[defeasance]` carries `|rel :to permit :kind same-shape-as`. `|term[canceling-pair]` carries
`|rel :to permit :kind audited-as`. The general form is **a NEGATIVE DECLARATION WITH A VOID
CONDITION** — *"this is absent/retired/refuted, and here is what would prove that claim false"* —
and **the void condition is a GRAPH QUERY, i.e. it is executable and free.**

This is M1/M2 again (the missing retraction primitive), now with the project's own edge already
drawn between the three instances. **Nobody walked the edge.** And `permit` was itself demoted into
the manifest by the regula collapse three hours before the LEXICON's last edit — so the mechanism
got relocated exactly when it was about to be recognized as general.
