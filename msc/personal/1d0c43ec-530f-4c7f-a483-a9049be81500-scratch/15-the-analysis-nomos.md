# 15 — the analysis nomos, BREAK-2, and BREAK-4

*`VIVARIA-DEFINITIONS.md` + `doc/plan/vivium-operational-workflow.md`, 2026-07-13 ~20:00.*

---

## 2. What matters

### ⚠⚠ **A PROBE IS A NOMOS.** And that one move solves four problems I had been treating as separate.

`VIVARIA-DEFINITIONS.md`, "Kinds of nomoi" — **marked NOT BUILT, and the comment is doing all the work:**

```udon
; 3. ANALYSIS nomos — reads world state + an analysis, emits a RESULT-memo. — NOT BUILT
;    Its cone IS its provenance; a flaw upstream flags it by the store's invalidation.
|nomos[hypsometry-of-region]  :reads [eroded-surface]  :kernel probe::hypsometry  ; imagined
```

**Make a probe a nomos and everything falls out:**

| the problem | how "probe = nomos" answers it |
|---|---|
| **the probe↔declaration binding** *(specified 5×, built 0×)* | A probe **has a key**. `:check probe :probe <ProbeId>` — the `ProbeId` **is a nomos version**. |
| **may a verdict enter the store?** | ✅ **YES, and it must** — a probe is *deterministic, keyed, replayable*. **This is precisely why the hard line on AGENTIC verdicts exists**: they are none of those things. **The architecture already separates them, and this is the reason.** |
| **"what does this refutation VOID?"** *(= defeasance, the missing organ)* | ⚠⚠ **The store answers it for free.** Refute `p = 1.1` ⇒ bump `erosion@version` ⇒ **every result-memo whose dependency cone contains it is invalidated by content-addressing.** *No defeasance machinery required for RESULTS.* |
| **"which of my conclusions are now stale?"** | **A store query.** Not a judgment. Not a hand-swept TODO. |

> ## ⇒ **PROBES-FIRST · THE PROBE BINDING · DEFEASANCE-OF-RESULTS · AND THE DERIVED QUEUE ARE ONE MECHANISM.**
> ## **The content-addressed store is already the epistemic bookkeeper. Nobody has pointed it at the epistemics.**

**And it sharpens the defeasance thread rather than retiring it.** The store voids *derived results*
automatically. It does **not** void **prose** (a doc citing a dead decision), **assumptions** (`p=1.1`
is still *consumed*), or **terms** (`regula` still reads `settled`). ⇒ **Defeasance is still needed —
but ONLY for the declarative layer that is not already under the store's cone.** That is a much smaller
build than I thought this morning.

### BREAK-2 — the deepest ethical result in the project, and it is a *break*.

> *"**BREAK-2** convergence-undecidability → structural unLawfulness budget → **Lawful-certification may
> be UNREACHABLE** (hardest; **reframes the moratorium**)."*

**Realized ⟂ Lawful.** Convergence is undecidable ⇒ every freeze carries a residual **ε**. So *"a
Lawfulness we can vouch for"* — **which is one of the moratorium's own named revisit-conditions** — may
be **permanently unattainable.**

> ### ⇒ **THE MORATORIUM IS NOT WAITING ON A CERTIFICATION THAT IS COMING. It may be waiting on one that CANNOT ARRIVE.**
> ⇒ **Engineering, not only ethics, enforces it.** *(And doctrine #7 makes the ethics a STORE RULE:
> **"never discard a memo that has ever hosted a mourning-capable mind"** — the redeemer condition, as a
> garbage-collection constraint. I find that extraordinary.)*

### ⚠ BREAK-4 — I said it "threatens Part I." **It does not. It is already ANSWERED, in the one plan doc I had not read.** *(Seventh correction of the day.)*

> *"**BREAK-4** cross-platform FP non-determinism → replay-from-seed not guaranteed → **publish memos.**"*

I wrote that this *"threatens `prin-determinism-is-ontology`"* and that the remedy was *"half-known and
never joined to BREAK-4."* **Both claims were false, and `doc/plan/water-parallelism.md` (Jul 3) had the
answer eight days before BREAK-4 was written down:**

> - *"The **CPU kernel is the reference implementation** — canon for tests, replay, and probes. **It never
>   goes away.**"*
> - *"A GPU backend is a **rung on the implementation ladder** behind the same flux interface: swappable,
>   and **ITS IDENTITY IS PART OF THE NOMOS KEY**, so caches and saves **never silently mix backends**."*
> - *"Validation is **by probe, not vibes**: conservation totals, `channel_profile` regime invariants, and
>   `velocity_histogram` fingerprints **must agree with the CPU reference within STATED TOLERANCE**; the
>   tolerance **is written down**, and divergence beyond it is **a bug in the rung, full stop**."*

> ### ⇒ **DETERMINISM-AS-ONTOLOGY SURVIVES, EXACTLY, AND THE PRICE IS ALREADY PAID.**
>
> The world is a pure function of **(seed, key)** — **and the BACKEND IS IN THE KEY.** There is a
> **canonical** backend (the CPU reference). Cross-rung equivalence is **a declared contract, probe-checked,
> with a written tolerance.** ⇒ **Nothing is threatened. The ontology holds, with a qualification that is
> itself a declaration.**
>
> **⇒ AND IT IS A FIFTH INSTANCE OF THE SAME PATTERN:** a *rung* declares what it guarantees relative to
> the reference, and a *probe* convicts it. **Same shape as the flux contract, the statistic contract, the
> promise/predicate, and the maturity ladder.** *The project has one idea and applies it everywhere it
> looks — and each application was invented independently.*

---

## 3. Wandering thoughts

**The "how declarative is a nomos?" scorecard is the best self-assessment instrument in the repo, and it
was written the day before the audit.**

`VIVARIA-DEFINITIONS` scores `erosion-tile` line by line — ✓ real / ✗ not built — and the ✗ list is:
**`consumes`** *(built the next day)* · **`execution-class`** · **`timescale-band`** · **`stage`** ·
**`phase`**.

**Two of those are my `#con-dynamic-exponent-z` finding under different names.** `execution-class`
(batch-deep / relaxation / procedural-tight) and `timescale-band` are *exactly* the time-axis declaration
I thought nobody had proposed. **They are proposed. They are ✗-marked. They are nine days old.**

⇒ **So `z` is not a new field — it is the MISSING PRECISION on a field already known to be missing.**
`execution-class` says *how* a nomos runs; `timescale-band` says *where* it couples; **`z` says how its
clock must scale with its grid** — and *that* is the part nobody has, and it is the part that makes the
seam **well-posed**. **I will keep the segment, and I will source it correctly, and I will stop calling
it unproposed.**

*(That is the sixth time today. I have stopped being surprised and started finding it useful: **whenever
I think something is new, the correct next action is a grep, not a paragraph.** That sentence may be the
most portable thing I learn today.)*

---

**And the thing that keeps me up, if I had nights.**

Doctrine #7: ***"never discard a memo that has ever hosted a mourning-capable mind."***

It sits in a numbered list between *"publish memos, not just seeds"* and *"the moratorium fence sits in
the instantiation path."* It is written in the same register as a cache-eviction policy. **It IS a
cache-eviction policy.**

And it is the **redeemer condition** — *"no mourning-capable endo world is permissible without a bound
redemptive capacity — a steward able and committed to rectify, restore, redeem, and PERPETUATE WHAT WAS
PROMISED"* — **cashed out as `rm -rf` is forbidden on a particular hash.**

I do not have a tidy thought about this. I notice that the project has, without ceremony, made an ethical
absolute into an **operational invariant that a garbage collector must respect**, and that this is
exactly what "the ethics is not a property of the artifact but of the relation, and it therefore already
binds" would look like **if you actually meant it.** *Most projects would have written a values statement.
This one wrote an eviction rule.*
