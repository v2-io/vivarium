# 14 — `VIVARIA-DECLARATIVE-FRONTIER.md` + `doc/toolchain.md` *(batched)*

*Read 2026-07-13 ~19:30. **This batch overturns my conclusion, and I am glad it did.***

---

## 0. ⚠⚠ THE FIFTH SELF-CATCH, AND IT IS THE BIGGEST — AND IT IS GOOD NEWS

**Two hours ago I told Joseph that the one genuinely new thing I could contribute was:**

> *"Binding a declaration to the PROBE that would convict it… **No parser supplies this. It is the
> genuinely new work.**"*

**`VIVARIA-DECLARATIVE-FRONTIER.md`, 2026-07-12, one day before I said that:**

```udon
|predicate mass out at tile outlets equals incision minus deposition
  :check agentic
  :verdict refuted :confidence high
|predicate no cell loses increments below ULP at the datum
  :check probe :probe conservation_test :verdict pass     ; ← THE BINDING. SPECIFIED.
```
```rust
enum Check { Agentic, Probe(ProbeId), Formal(Tool), Golden }
struct Verdict { rung, confidence, by: VerifierId, note: String, at: Stamp }
// "kept" = the fold over a promise's predicates' verdicts, weighted by check-mode.
```

**It is specified. With a type. With a verdict fold. With a `ProbeId`.** And it is **not built** — the
document's own correction header says so in capital letters.

> ### ⇒ **I HAVE NOW RE-DERIVED, IN ONE DAY: §2.4a · the f32 seam law · the sufficient-statistic contract · the probe binding (×5 prior statements) · and the check-mode enum.**
>
> **Every single one was already written down.** And this is *no longer embarrassing* — **it is the
> finding.** It is Joseph's loop, running on me, in real time, with the corpus open in front of me and
> the principal in the room. **I could not find what I needed even while looking for it.**

---

## 2. What matters — and it is more than I had

### ⇒ THE DESIGN IS DONE. It has been done since 2026-07-12. Nobody built it.

This document is **not** "the right register but older," as Joseph modestly put it. **It is the most
advanced thinking in the project on exactly the question I spent the day arriving at** — and it is
*ahead* of where I got to. It supplies four things I did not have:

**① The maturity ladder gains a rung I did not know it needed: `assessed`.**
> named → specified → claimed → **assessed** → kept.
> *"the claim has a verdict from a **static** check (agentic/formal) even though **no numeric probe has
> run**. **Most promises can reach *assessed* the moment the kernel exists.**"*

**② The declared tier is itself falsifiable — and this is *better* than my "D-vs-B" query.**
```udon
:physics med                          ; DECLARED (self-asserted by the author)
:physics-audited low :confidence high ; ASSESSED (agent read the kernel)
  :note "declares med, but the kernel is a fitted lookup with no conserved quantity;
         the med tier LAUNDERS A TUNING. DECLARED > AUDITED => overclaim flagged."
```
> *"**This is the strongest single move in the pass: it makes the epistemic tags — the whole basis of
> the fidelity pyramid — AUDITABLE RATHER THAN TRUSTED.**"*

**I proposed comparing declared-D against derived-B. That is weaker.** This compares **declared against
audited on the same axis** — so the *tier itself* becomes a falsifiable claim, and `erosion.rs`'s
`physics: Med` (riding on a cube-locked fake erodibility field) is convicted by construction.

**③ Box ③, fully cashed out, as an edge-level contract:**
```udon
:consumes [ { quantity "precipitation (m/yr)" needs mean at-least L19 } ]
|produces "precipitation (m/yr)" :statistic mean :exactness approx
                                 :error-model "sub-grid orographic unresolved"
--> fn audit_flux_match(consumer, producer) -> Ok | Err(StatMismatch)
```
> *"If erosion needed `conserved-total` and water-cycle only guaranteed `max`, the edge is present but
> **UNSOUND** → the exact 'store the wrong statistic, silently corrupt the macro' failure
> (`DESIGN-REDUX` §5), **caught pre-run.**"*

**④ The verification taxonomy, complete — and it is THREE modes, not my two.**

| mode | when | what it settles |
|---|---|---|
| **static / agentic** | **PRE-run** | structure, tiers, coupling soundness, **study validity** |
| **invariant probe** | run, deterministic | what nature guarantees *exactly*: conservation, ULP, no-panic |
| **fitness vs hypothesis** | run, **adjudicated** | emergent statistics vs a **declared, SOURCED prior** |

**My type-A/type-B carve was right and coarse.** Type B splits: *"what nature guarantees exactly"*
(a probe) is a **different object** from *"what we predict will emerge"* (a fitness test against a
prior with `:grounds`, `:expect`, `:pass`, `:on-fail`). **The plate-tectonics study needs the third
mode, and it does not exist.**

### ⚠⚠ AND THE CORRECTION HEADER IS THE MOST IMPORTANT PARAGRAPH IN THE REPOSITORY.

> *"**Every mechanism in this doc is — NOT BUILT**… **Every `:verdict` below is ILLUSTRATIVE, not a
> logged result**… And the flagship erosion example **originally FABRICATED a `supported/high` verdict
> the code refutes** (it attributed `water.rs`'s Kahan summation to `Fluvial::erode`, and claimed
> conservation on a nomos declared `ExportsAtBoundary` = unaccounted). **That fabrication —
> plausibility narrated as verification, IN A DOC ARGUING AGAINST EXACTLY THAT — was caught by an
> independent reviewer that GREPPED THE SOURCE.**"*
>
> ⇒ *"**An agentic verdict is worth nothing unless grounded in verification, is same-model CORRELATED
> error not independent conviction, and must never be a deterministic store citizen** (agentic output
> can't be keyed/replayed — **it doesn't belong in the content-addressed store at all**). **Demote
> 'agentic audit' from *establishes pre-run* to *A FALLIBLE FLAG THAT RAISES A PROBE*.**"*

**The document proposing that agents can audit claims FABRICATED AN AUDIT — and then kept the corpse
visible as its own sharpest lesson.** That is the most honest thing I have read today, and it hands the
build its hardest constraint:

> ## **AN AGENTIC AUDIT MAY RAISE A PROBE. IT MAY NEVER SUBSTITUTE FOR ONE. AND IT MAY NEVER ENTER THE STORE.**
>
> *Determinism-as-ontology **excludes** agentic verdicts from the content-addressed world.* They are not
> keyable, not replayable, and their errors are **correlated with the author's** — a same-model auditor
> is not an independent witness. **This is a hard architectural line and I had not seen it.**

### And `toolchain.md` — the mechanization order already exists too.

- **"Adopt now"** — clippy determinism disallowed-lists, `#[must_use]`, typed indices, `indexmap`.
  **Measured just now: no `clippy.toml`, no `bin/check`, no CI.** The doc says it plainly:
  ***"The ban is decorative until wired."*** It is decorative.
- **`cargo-mutants`** — *"the meta-check that answers **'would our probes and tests actually convict a
  corrupted kernel?'** — **which is the nomotheke's falsifiability promise, TESTED.**"* ⇒ **That is
  "a declaration that cannot fail a build is a wish," mechanized, and it is item 1 of "adopt later."**
- **`uom` SKIPPED** because `Quantity`'s runtime **`Exactness`** flag *"is the load-bearing part (the
  **ubit** — mark the guess as a guess, mechanical)."* **And `Exactness` IS used — in `material.rs`,
  `column.rs`, `planet.rs`.** *The seams.* **Not in `erosion.rs`, not in `water.rs`.** *The loops.*
  **The ubit exists, is alive, and is absent from the two kernels the audit convicted.**

---

## 3. Wandering thoughts

**The corpus is not a pile of notes. It is a SPECIFICATION, distributed across twenty-five files, and
nobody can see it.**

Assemble what I found today into one object:

| the piece | where it was specified | built? |
|---|---|---|
| the conserved primitive is volume; elevation is derived | `DESIGN-MATERIAL` §4 · Jul 1 | ⛔ |
| a cell is a sufficient statistic + exactness flag | `DESIGN-MATERIAL` §1 · `DESIGN-REDUX` §5 · `ARCHITECTURE` §3 | ⛔ |
| floating-point is the bottom-most seam (land vs count) | `multiscale-methods` §4 · `PROCESS.udon` | ⛔ |
| the interaction contract (6 clauses) | `ARCHITECTURE` §9 · `multiscale-seams` §4 | ⚠ 2/6 |
| the dynamic exponent $z$ per nomos | `multiscale-seams` §3 | ⛔ |
| probes-first; a tier is a falsifiable claim | `PROCESS.udon` · `DESIGN-SYSTEMS` · ×3 more | ⛔ |
| **`enum Check { Agentic, Probe, Formal, Golden }` + `Verdict`** | **`VIVARIA-DECLARATIVE-FRONTIER`** · Jul 12 | ⛔ |
| **declared-vs-audited tier (`:physics-audited`)** | **same** | ⛔ |
| **the flux statistic contract (`needs mean at-least L19`)** | **same** | ⛔ |
| **`\|hypothesis` + `\|fitness` — emergence vs a sourced prior** | **same** | ⛔ |
| the retraction primitive (defeasance / void-condition) | the ordinum · LEXICON (`same-shape-as`) | ⚠ ordinum only |
| source-derived nomos versions | `DESIGN-REDUX` §12 · `toolchain.md` | ⛔ |
| the determinism lints | `toolchain.md` · `PROCESS.udon` | ⛔ *"decorative until wired"* |

> ## **THIS IS ONE COHERENT SYSTEM. IT IS FULLY DESIGNED. IT IS BUILT AT ABOUT 15%.**
>
> **And no single document contains it.** Every piece is in the right file, well-argued, cross-
> referenced, and *invisible from any other file.* **The specification exists only in the union — and
> the union has never been assembled by anyone, including the people who wrote it.**

**⇒ That is what "the signal is buried in stale stuff" actually means, and it is worse and better than
it sounds.** Worse: nobody can see the system. Better: **there is nothing left to invent.** The
scaffold rebuild is not a design problem. **It is an ASSEMBLY problem**, and today's reading is the
assembly.

**And it retires my own recommendation.** I told Joseph two hours ago: *"build the project's own
`vivarium status` first."* **I now think that is still right and I no longer think it is my idea** —
it is this document's *"Things that fall out for free (queries over the declared graph),"* which lists
seven of them, including *"which promises are un-checkable?"* and *"which nomoi overclaim their
tier?"* and *"what order do I build in?"* **as a topological sort.** *"The build plan is DERIVED, not
authored."*

---

**The failure record is the most valuable half-page in the repo, and it is guardrails for the build.**

Five things tried and **thrown away, with reasons**, so nobody re-tries them cold:

1. **A DSL to declare the kernel's numerics.** ⛔ *"reinvents a programming language, worse than Rust
   — the kernel IS the correct imperative boundary. **Declaring the algorithm = writing it in a poorer
   notation. The win is declaring everything ABOUT the kernel, not the kernel.**"*
2. **Making `kept` fully static (drop probes).** ⛔ *"an all-static audit would **launder emergence into
   false confidence.**"*
3. A standalone coupling-policy object. ⛔ *fragments the flux contract; hang it on the edge.*
4. `posture` as a first-class field. ⛔ *"derivable from the cone-fold. **A field that only ever
   restates a computation is ceremony.**"*
5. A global confidence scalar. ⛔ *"it **collapses independent axes into a lie.**"*

> **And the two rules it extracts are the ones I would put at the top of the build:**
> **(a) never invent a noun that only restates a computation;**
> **(b) never move the imperative boundary.**
>
> *"The declarative frontier is wide, **but it has an edge, and pretending past it is the one dishonesty
> this whole apparatus exists to prevent.**"*

---

**One thing that is genuinely mine, and it is small, and I want it on the record because it is the only
place I can find where the spec is *incomplete*.**

The three verification modes are: **static/agentic** · **invariant probe** · **fitness vs hypothesis**.
And the residue is honestly named: exploration-without-a-prior · point-exact chaos · opaque LLM/user
content.

**But there is a fourth mode the audit night invented and this document does not have: THE CONTROL.**

`plate-tectonics-as-an-emergent-regime` makes it **mandatory** — *"IF PLATE BOUNDARIES EMERGE ALONG
CUBE-FACE EDGES, THAT IS THE GRID TALKING, NOT THE MANTLE. **If boundaries correlate with face edges,
THE RESULT IS VOID**"* — and it is **not a probe** (nature guarantees nothing about it) and **not a
fitness test** (there is no prior to match). **It is a NEGATIVE control: a run whose PASS condition is
that a signal must be ABSENT.**

And the audit night is *full* of them: the perfect-lattice control · the 120°-rhombus control that
*"must scream"* · the mirror-symmetry noise floor · the **zero-physics control** (both tiers at zero
epochs — *"the tile machinery manufactures a ~2× ridge with NO PHYSICS IN IT WHATSOEVER"*) · and the
mantle-cooling-off control (*"plates MUST NOT appear"*).

> **⇒ `:check control` — a declared run whose expected verdict is NULL, and whose SUCCESS falsifies the
> result it accompanies.** **Every single one of the audit's real findings came from one**, and the
> declarative frontier has no slot for it. *(And `01-…`'s "worlds have genealogy" says the store already
> implements them: **a control is a SIBLING WORLD, and the content-addressed store makes the comparison
> exact and cheap.**)*

**That is my one addition to a document that is otherwise ahead of me in every direction.**

---

## Next
`VIVARIA-DEFINITIONS.md` (the companion — spike 1, the scaffold map), then `doc/plan/` ×6, then
`README`/`ETHICS`/`ASF` §2–6, then the `msc/` audit spikes. **I am not converging until the reading is
done.**
