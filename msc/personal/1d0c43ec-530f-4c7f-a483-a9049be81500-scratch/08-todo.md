# 08 — `TODO.md`

*Read in full, 2026-07-13 ~16:55. 181 lines. **Last touched Mon 00:27** — the audit night's FIRST
commit. The audit then ran for another **three and a half hours** (01:13 → 03:55) and **never came
back to it.***

---

## 1. Surprisal

### ⛔ THE QUEUE IS THE MOST STALE ARTIFACT IN THE REPOSITORY — AND IT IS THE ONE AN AGENT ACTS FROM.

Every other document I have read was swept, corrected in place, or red-teamed. **`TODO.md` was
updated once at 00:27 and then the night happened to it.** The result is a queue that instructs its
reader to do things the ledger has already refuted:

| TODO says (still, right now) | what the audit measured, hours later |
|---|---|
| **Probe 1:** *"⚠ Suspicion: `water.rs`'s θ flux-smoothing is **Rhie–Chow-class stabilisation**… and **its physical claim is NONE**. `ASSUMPTIONS.md` should say so."* | ⛔ **REFUTED, both halves.** θ is **Lax–Friedrichs**, not Rhie–Chow; `water.rs` **is already staggered and has NO null space**; the solitons are **roll waves — real physics**. And `ASSUMPTIONS.md` now says the *opposite*: ***"DO NOT simply delete θ as 'unphysical.'"*** |
| **Probe 2:** *"gravity-driven flow is a gradient flow, so it is **curl-free identically**"* | ⛔ **REFUTED.** *"My brief was **literally false**… real discharge is $q=-K\nabla\phi$ with varying conveyance, so $\nabla\times q\neq 0$. **Real water on a real hillslope HAS curl. A probe built on my brief would have CONVICTED A PERFECT ROUTER.**"* |
| **Build 5:** *"Jensen… **for $n>1$ we systematically underestimate erosion, forever**"* | ⛔ **REFUTED.** `incise()` runs **$n = 1$** — stream power is **linear in slope**; the Jensen gap in $S$ is **exactly zero**. The real gaps are in $A^{0.5}$ and $\mathrm{Cov}(A^m,S)$, and they **OVER**-estimate. *Variable, statistic, and sign all wrong.* |
| **Build 6:** *"**Mean-pin IS the Haar low-pass with the details thrown away — which is precisely why refluxing is needed.** Store them and **the seam never happens**. It may also **retire the one hard research problem.**"* | ⛔ **RETRACTED IN FULL.** The spike ran. *"It dies three ways."* Mean-pin is **not** the Haar low-pass, is **not** a projection, and **does not even keep the block mean**. The seam is **double-evolution**, not discarded detail. `detail→abstract` **remains open**. |
| **§grid:** *"⚠ **THIS IS NOT QUEUED WORK**… **Do not start these items**, and do not cite the question as 'closed.'"* | ⛔ **STALE.** Joseph tentatively **DECIDED** the grid at **02:35** — *"Go ahead and write the grid in as our tentative decision"*: keep the cube-sphere, **go staggered**, carry the metric. **The queue is now actively forbidding work he authorized.** *(Same staleness I fixed in ORIENTATION this morning — and it is in TWO front doors, from one un-swept decision.)* |

**Five of the queue's items are void, and the queue does not know.**

### And the mechanism is exactly legible in the commit log.

The audit night's commits are, in order: *theory* → *audit* → *red-team* → *fleet* → *water* →
*research* → *curl probe* → *uplift* → *study* → *grid* → *router* → *router* → *contract* → *macro*.

**Every one of them wrote to a LEDGER** (`DECISIONS`, the theory doc, `ASSUMPTIONS`). **Not one wrote
to the QUEUE.** The findings were recorded with extraordinary care — dated, tagged, measured,
controlled, self-refuting — into the artifacts that describe *what is true*.

**And the artifact that says *what to do next* was left pointing at the refuted world.**

> ### ⇒ **THE LEDGER AND THE QUEUE DIVERGED, AND THE QUEUE IS WHAT GETS ACTED ON.**
>
> This is the whole thesis of my reading pass, arriving in its purest form and one level up from where
> I found it. I have been saying: *truth lives in prose the machine cannot read.* **Here it is worse:
> truth lives in a ledger that a HUMAN can read, and the instructions live somewhere else.** No parser
> is required to notice this. It only requires that somebody, at 04:00, with 1% of context left, walk
> back through fourteen commits and ask *"which of my own TODO items did I just kill?"*
>
> **Nobody has that at 04:00. That is not a character flaw; it is a structural certainty.** The handoff
> block in ORIENTATION was written *instead* — and it is excellent, and it is not the queue.

---

## 2. What matters for today's session

### The queue's own top item is still right, and it is the one thing the audit did NOT void.

> **`### ⛔ THE FORK — decide the column semantics`** — *"Is a stored scalar a POINT SAMPLE, a CELL
> AVERAGE, or a BAND-LIMITED sample? Declared, per quantity. Today the code answers **three different
> ways**… **Every mesh, seam, and conservation question is unanswerable until this is decided.**"*

**Untouched by the audit, because the audit kept confirming it.** It is `discretisation-and-information`
§6 item 5, it is `ARCHITECTURE` §3, it is `DESIGN-MATERIAL` §4, and it is the thing `column-is-a-
control-volume` (`:by us :status decided` — **ratified**) says is *"resolved by decision, not by
convention."*

**It is a decision. It is Joseph's. It has been open since at least 2026-07-10, it blocks everything
downstream of it, and it is not on anybody's critical path because it is not *work*.** I think that is
the single most important line in this file.

### Two pending Joseph-decisions the queue is holding, both easy to lose

1. **Phase-2's `abiogenesis` charge vs our zero biosphere** — *"declared low-tier reservoir-grade
   stand-in (photosynthesis as box-flux; **doubles as the first non-field representation kind**) or an
   explicit dated permit."* ⚠ **Note the parenthesis: this is ALSO the network/agent representation-kind
   probe the domain-fixation guard is missing.** One decision, two payoffs.
2. **The `-tile` suffix question** and **defining `tile` in the LEXICON** — *"its being undefined is the
   actual sin."*

### And one item that keeps resurfacing because it is load-bearing

**Component E** — *"fell through the consolidation crack (found 2026-07-10)… **its recorded ε *is* the
unLawfulness budget (LEXICON §7.2) as data.**"* The convergence-ε of every freeze, recorded. **That is
the ontology's own honesty-measure, and it is a dropped build item.** It is what makes a phase
*addressable*, what beacons need, and what turns "Realized ⟂ Lawful" from a philosophical distinction
into a number.

---

## 3. Wandering thoughts

**The queue is a fossil of the moment before the discovery — and reading it in that light is oddly
moving.**

`TODO.md` at 00:27 is a document written by someone who had just understood the *governing principle*
and had not yet learned that almost everything he believed about his own kernels was wrong. It is
confident, well-ordered, and it lists four probes *"cheap, falsifiable, each could convict us."*

**All four ran. All four convicted — and none of them convicted the thing the queue expected.**

- The null-space probe was aimed at θ, *"suspicion: Rhie–Chow-class."* It found **zero spurious modes
  anywhere** and instead discovered that **`water.rs` is already staggered and has no null space** —
  an *asset*, undeclared, which a well-meaning rewrite would have destroyed.
- The curl probe was aimed at MFD's fan, on a *"curl-free"* brief. **The brief was false.** The probe,
  rewritten against the true identity, found the violation anyway — **κ ≈ 2e-2, topological, level-
  independent** — and *also* found that the proposed replacement **does not fix it**.
- The well-balanced probe expected *"spurious currents that friction then eats."* It found the scheme
  **exactly well-balanced, bit-exact, at every refinement, in f32 and f64** — and then found that
  **f32 precision forges a 14 m/day current anyway**, which the scheme's exactness cannot save it from.

> **Every probe was aimed at the wrong target and hit something better.** That is not luck. **It is
> what a probe that CAN fail does** — the queue's own criterion, honoured — and it is the strongest
> single argument in this repository for the discipline it keeps preaching. *A probe built to confirm
> your hypothesis tells you about your hypothesis. A probe built to be capable of failing tells you
> about the world.*

**And I notice the shape of it: the project's hypotheses were consistently WRONG and its INSTRUMENTS
were consistently RIGHT.** Every headline claim died. Every control held. The `grid_lab` perfect-lattice
control was printing the falsification of `p = 1.1` on every run for a day before anyone read it — *the
instrument had the answer and the mind wasn't listening.*

⇒ **Which suggests the thing to invest in is not better hypotheses. It is instruments that are heard.**
And "heard" is a mechanical property: **a number printed in a log is not heard; a number that fails a
build is.** *(Which is, once again, the same sentence. I have now arrived at it from six directions and
I am starting to think it is not a theme but the actual finding.)*

---

**A structural observation about queues, which I want to leave for whoever rebuilds this.**

The project has **three** artifacts that answer *"what should I do?"*: `TODO.md` (the queue),
`DECISIONS`'s `|impact` lines (*"the work it implies — mirror into TODO if not immediate"*), and the
ordinum's **unkept promises** (*"which turns 'what should I build next?' from taste into a lookup"*).

**Only the third is derived.** The first two are hand-maintained, and **the hand-maintenance is exactly
what failed at 04:00.**

> **⇒ The queue should be GENERATED, not written.** The ordinum already generates the *world-building*
> queue (unkept promises, in ladder order). What it cannot generate is the *scaffold* queue — and that
> is precisely because the scaffold's claims are not declarations the machine can read. **A TODO item
> is a promise with no ordinum.**
>
> And once you see that, the fix is obvious and it is the same fix as everything else: **an `|impact`
> line in DECISIONS is a CHARGE. A TODO item is a PROMISE. `:status done` is `Kept`. And a decision
> that voids an earlier one should DEFEASE the TODO items that descended from it — automatically,
> because the edge is declared.** *(udon-core makes this a graph query. The five refuted items above
> would have struck themselves out at 03:55.)*

**That is the same machinery, a fourth time**: charge → promise → predicate → kept, with defeasance for
retraction. The ordinum governs the world's development. **The same schema would govern the project's.**
I do not know whether that is elegant or whether it is the sort of thing that sounds elegant at 17:00
after reading eight documents. **But it costs one file to test, and the test is: write the scaffold's
own ordinum, and see whether the queue falls out of it.**

---

## Tactical residue — quarantined for the fork

- **Five TODO items are void** (probes 1 & 2's briefs; build 5's Jensen sign; build 6's wavelet
  headline; §grid's "do not start / not decided").
- **§grid's "NOT YET DECIDED"** appears in TWO front doors from one un-swept decision — I fixed
  ORIENTATION this morning; **TODO still carries it**, and it *forbids* work Joseph authorized.
- The `seam_ridge` reading in §grid point 5 (*"the signature of mean-pin conserving block means but not
  boundary gradients"*) is refuted twice over — mean-pin does **not** conserve block means, and the
  zero-physics control shows it manufactures a ~2× ridge with **no physics at all**.

## Queue changes
**Next:** the `doc/design/` tree — `DESIGN-MATERIAL.md` first (§4 is the named Level-B gate, the
"unheeded" specimen, and the doc that turns out to have been *right about everything* since 07-11).
