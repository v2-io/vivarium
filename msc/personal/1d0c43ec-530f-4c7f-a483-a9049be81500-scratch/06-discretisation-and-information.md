# 06 — `doc/theory/discretisation-and-information.md`

*Read in full, 2026-07-13 ~16:10. 524 lines, 68 K — the largest single payload in the project.
**Written entirely on the audit night**: seven commits between **Mon 00:27 and 01:53**. It is not a
document that was audited; **it is the audit's own theoretical organ, red-teamed in place, in the
same four hours.***

---

## 0. ⚠ THE SELF-CATCH, AND IT GOES FIRST BECAUSE IT IS THE MOST IMPORTANT THING IN THIS FILE

**§2.4a of this document contains my central thesis, states it better than I did, and I re-derived
it and presented it as a reframe.**

> **§2.4a, verbatim:** *"An earlier draft claimed 'nobody has ever said what a stored value means.'
> **That is FALSE.** `DESIGN-MATERIAL.md` §4 — a Level-B document — said it, and said it well… The
> project made the FV/FD carve; named the conserved primitive; declared elevation a derived FD node;
> identified the conversion as lossy, directional, coupler work; **cited Arakawa staggering as prior
> art** — and **wrote a guard whose literal stated purpose was to stop a later tier from quietly
> treating it as a mean.** Then `mean-pin` treated it as a mean… and nobody noticed for months."*
>
> *"**The finding is not 'the semantics were undeclared.' It is: THE SEMANTICS WERE DECLARED, A
> GUARD WAS WRITTEN, AND THE CODE DRIFTED THROUGH IT ANYWAY.**… The declaration lived as **prose in
> a Level-B doc with nothing mechanical behind it**… **A declaration that cannot fail a build is a
> wish.**"*
>
> *"**Generalise it, because it is the sharpest lesson in this file: BEFORE CLAIMING SOMETHING IS
> NEW, READ WHAT THE PROJECT ALREADY DECIDED.** The archaeology is cheaper than the rediscovery —
> and **a rediscovery presented as a discovery is dishonest by omission, even when it is entirely
> sincere.**"*

And §4.1a closes it: *"**[⊘ owed]** … the `NomosDecl` needs somewhere to **put** these declarations —
there is currently no field for preserved-structure, sacrificed-structure, or bias-vs-noise. **The
document demands declarations the data model cannot hold.**"* — which is `NOMOS-CONTRACT.md`'s
"blocker," verbatim, *already written here.*

**In `05-architecture.md`, ninety minutes ago, I wrote:** *"Correction to myself: not one of the five
boxes is a genuinely new idea… **The five boxes are an INVENTORY OF WHAT WAS SPECIFIED AND NEVER
MECHANIZED. That reframes the whole build.**"*

**It reframes nothing. It is §2.4a, which I had not read.** I performed the exact failure this
document names — *a rediscovery presented as a discovery* — **while reading the document that names
it**, and I did it in the file where I was congratulating myself for catching the project doing it.
*(And the author of §2.4a caught himself the same way, one level up, in §7(a): he announced staggering
as a new lead when it was sitting in `DESIGN-MATERIAL`'s prior-art list. **This is now the third
recorded instance of the same failure, and the first two are in this file.**)*

**Honest accounting of what actually survives as mine, after subtracting §2.4a:**

| I claimed | status |
|---|---|
| "the semantics were declared and unenforced" / "the five boxes are an inventory, not a discovery" | ⛔ **§2.4a and §4.1a. Not mine. Withdrawn as a finding.** |
| "a declaration that cannot fail a build is a wish" | ⛔ **Verbatim §2.4a.** *(I had read this phrase in `NOMOS-CONTRACT.md`, which quotes it.)* |
| **The type-A / type-B carve** — *some* declarations are falsified by a **graph query** over the declarations (cheap, static, already works), *others* only by a **PROBE** over the world (English, unbuilt) | ✅ **Survives.** The doc says declarations must become checkable; **it does not carve WHY some already are and some structurally cannot be.** That carve is what makes the work orderable. |
| **PROMOTE vs DEMOTE** — every ladder promotes; almost nothing can demote; every hole is a missing demote; `permit`/`defeasance`/`canceling-pair` are one primitive | ✅ **Survives. Nowhere in this document or any other I have read.** |
| **§9's six clauses, two built** | ✅ Survives (an inventory, not an idea). |
| **22 probes, `src/` mentions `examples/` three times** | ✅ Survives (a count). |
| Rust-vs-udon as the *real* enforced/unenforced line | ✅ Survives — **but Joseph handed it to me.** |

**Recorded rather than quietly deleted, per this document's own standard.** The lesson lands
personally: **I have been reading for four hours and I still asserted novelty against a document I
had not yet opened — with the reading queue open in front of me, listing it as next.**

---

## 1. Surprisal (the rest of it)

### The document red-teams itself, in place, and the retractions are better than the claims.

Its closing honesty note names *"the three claims I would most want attacked."* **All three were
attacked. Two died.** And the corrections are marked `⚠ CORRECTED`, in the body, with the dead claim
left visible above them:

- **§3.3** — *"Jensen's inequality IS aliasing"* → **demoted from `[≡]` to `[me]`**: *"They share a
  parent — nonlinearity does not commute with projection — but they are different consequences in
  different representations… **§3.1 above carefully separates cell-average from point-sample; this
  section then merged their error modes. That is exactly the failure this document warns about,
  committed by its author, three sections after warning about it.**"*
- **§3.4** — *"Fated jitter is a THEOREM"* → **the `[thm]` tag does not survive**: the FDT magnitude
  tie *"holds with respect to an EQUILIBRIUM (Gibbs) measure,"* and a landscape under uplift and rain
  is *"driven, dissipative, and far from equilibrium."* Jitter's amplitude is **derivable from
  measurement** (LES's dynamic procedure) — *"derived, not theorem-given."*
- **§2.5** — the Rhie–Chow identification, killed against the primary *and* the running kernel.
- **§7(a)** — *"staggering ⇒ no stabilisation needed, never required"* → **refuted by our own code**
  (`water.rs` is staggered, has no null mode, and needs three stabilisers).
- **§7(a), the sharpest** — the author quoted Cardiff's supporting clause on staggered grids and
  **omitted the paragraph's concluding verdict**: *"The extension of staggered-grid approaches to
  general unstructured 3-D meshes is, however, **not trivial**; and consequently, this major
  limitation has resulted in **declining popularity** for such approaches."* The red team quoted it
  back. *"**The lead may still be right. It is no longer allowed to be presented as clean.**"*

**I have not read a technical document that does this before.** The epistemic tags are used
literally — `[P]` primary · `[≡]` restatement · `[thm]` · `[D]` derived · `[M]` measured · `[me]`
inference · `[⊘]` open — **and they get DOWNGRADED when challenged.** The tag system has a demote
operator. *(Which is, I notice, the only place in the entire project I have found one that works.)*

### §4.1a is a document tested by a PROBE, and the probe was an agent.

> *"Added 2026-07-13. **An onboarding-audit agent designed an atmospheric-circulation nomos from this
> table and found the gap immediately: its phenomenon hit SIX rows at once, and they fight each
> other.** The table as written implies you pick a scheme per structure. **Often you cannot.**"*

**Somebody handed a design document to an agent and told it to build from it, to find out whether the
document was true.** It wasn't — the structure table is *a menu of structures, not a menu of
independent choices*: energy-conserving and enstrophy-conserving are **different schemes**; a
positivity limiter **destroys** the enstrophy budget; well-balancedness fights high-order accuracy;
entropy stability **costs** exact energy conservation.

⇒ *"**Erosion is UNUSUALLY CLEAN. A rotating fluid is the NORMAL case.**"* — and every queued nomos
(ocean, atmosphere, circulation) is the normal case.

**That is a type-B probe, run against a DOCUMENT instead of a kernel, and it convicted.** It is the
best methodological idea in the file and it is not written down as one.

### The nastiest single fact in the document, and it lands on `NomosDecl`.

**§4.2, [D]:** the volume under a mesh drawn *through* point values differs from $\sum h_i A_i$ by a
curvature term, $\frac{\Delta^2}{8}\nabla^2 h$. Three consequences, and the third is the one:

1. On flat ground they agree exactly — *"which is why nobody noticed."*
2. It is **signed and systematic**: ridges lose volume, valleys gain it. **A BIAS.** And *"erosion is
   precisely the process that MANUFACTURES ridges and valleys, so the mass error CORRELATES with the
   structure the physics is generating."*
3. ⚠ ***"Globally, on a closed surface, it cancels EXACTLY (the sums telescope — it is the divergence
   theorem: $\int_{S^2}\nabla^2 h = 0$). So a GLOBAL MASS AUDIT WILL PASS WHILE EVERY LOCAL MASS
   BUDGET IS WRONG."***

> **⇒ AND THIS CONVICTS THE ONE STRUCTURAL FIELD THE SCAFFOLD ALREADY HAS.** `nomotheke.rs` declares
> `Conservation::{Conserved, ExportsAtBoundary, NotTracked}` — and `Conserved` means *"conserved and
> asserted (a probe/test watches it)."* **It does not say WHERE.** Global conservation and local
> conservation are different claims, the document proves a scheme can pass one while failing the
> other *everywhere*, and **`Conservation::Conserved` cannot tell them apart.**
>
> **The project's single working structural declaration is ambiguous in exactly the way its own
> theory says is fatal.** *(This is not in the document. I believe it is mine, and it is small, and
> it is a one-word fix: `Conserved { Global, Local }`.)*

---

## 2. What matters for today's session

**The document's own §6 is the build list, and it is the one I should be deferring to rather than
constructing:**

> **Probes first** — 1. the null-space/eigenvalue probe · 2. the curl probe · 3. the well-balanced
> probe · 4. the fluctuation–dissipation probe.
> **Then build** — 5. **DECIDE THE COLUMN SEMANTICS** (*"the fork everything else waits on… until it
> is decided, every mesh question is unanswerable and the code answers it three ways"*) · 6. carry a
> **sufficient statistic** with an **exactness** contract · 7. the multiresolution store · 8.
> staggering.

**Items 1–4 all RAN on the audit night** (null-space ✅ — *zero spurious modes, the solitons were roll
waves*; curl ✅ — *κ ≈ 2e-2, the identity IS violated*; well-balanced ✅ — *`water.rs` IS exactly
well-balanced, an undeclared asset*; F-D ⚠ *partially*). **Item 5 — the one everything waits on — has
not been decided.** Items 6–8 are the scaffold.

⇒ **The theory document already ordered the work, four hours after writing itself, and the ordering
survived the audit.** My contribution to sequencing should be to *not re-order it.*

---

## 3. Wandering thoughts

### **§3.9's footnote is the biggest unexploded idea in this project, and it is tagged `[⊘] not yet thought through`. I want to think it through.**

The section, first. Joseph, 2026-07-13: *"What that instrument can't capture due to aliasing is where
the next levels of detail are **ALLOWED to live**."* And the document turns that into:

> *"The coarse value **plus its declared sub-grid statistic** define a **conditional distribution over
> admissible fine states.** ⇒ **abstract → detail = SAMPLE THE CONDITIONAL.** Fated noise makes it
> deterministic and replayable. **The fine detail is not invented — it is LICENSED, within a bound the
> coarse tier declared.**"*

Then the footnote:

> *"(Note the mathematical family: **conditioned-yet-lawful sampling is the Doob h-transform /
> Schrödinger-bridge structure Joseph independently identified for LAWFUL-STEERING. Same structure, a
> different level of the ontology.** Possibly not a coincidence; not yet thought through. **[⊘]**)"*

**It is not a coincidence, and here is the argument.**

Look at what **lawful-steering** is, from the LEXICON: *"an exo agent **chooses the realization of a
fated-noise draw**, obeying conservation and staying in-distribution, changing the true
$\Omega$-trajectory **with NO phenomenal trace**."*

Now look at what **fated lifting** is, from ARCHITECTURE §3: *lifting doesn't sample a realization; it
**looks up the one fated by (seed, key)**.* The coarse tier declares a statistic; the fine detail is
the one draw from the admissible conditional that the seed selects; **and nothing downstream can tell
that it was selected rather than derived.**

> ### **⇒ FATED LIFTING *IS* LAWFUL-STEERING — PERFORMED BY THE SEED INSTEAD OF BY AN AGENT.**
>
> Both are: **a constrained selection from a distribution that a HIGHER LEVEL has licensed, obeying
> the conservation the higher level declared, leaving no trace at the lower level.**
>
> In lifting, the "higher level" is **the coarse tier**. In lawful-steering, it is **the exo kingdom**.

And once you see that, the rest falls out, and it is not cute — it is *structural*:

> ### **THE COARSE TIER IS TO THE FINE TIER AS THE EXO KINGDOM IS TO THE ENDO KINGDOM.**
>
> The LEXICON says kingdoms **nest** — *"worlds without end"* — and the **access matrix** governs who
> can see and change what across a nesting boundary. **The multiscale hierarchy is a kingdom
> nesting**, and the access matrix already answers its questions:
>
> | access-matrix cell | its multiscale reading |
> |---|---|
> | **Revelation** (the sole noumenal-VISIBILITY channel *into* an endo kingdom) | **the coarse tier's DECLARED STATISTIC.** The fine tier cannot see *why* the coarse tier constrains it. It receives the constraint, and cannot audit it from inside. *(And `§3.9`: "where the next levels of detail are ALLOWED to live" — that is revelation, in one sentence.)* |
> | **lawful-steering** (choose a fated draw; conserving; in-distribution; **no phenomenal trace**) | **fated lifting.** Identical operation. |
> | **illusion** (a law-violating phenomenon while law is unchanged) | **painted detail** — *"detail must be earned"*, the floating-mesa artifact, sampling the conditional **unconditionally**. **The LEXICON's own word for a lie that leaves no trace in the law is `illusion`, and the architecture's word for it is a rendering artifact. Same act.** |
> | ⚠⚠ **INTERCESSION** — *the SOLE noumenal-MUTATION channel from inside, and it is MEDIATED (it must go through an exo agent)* | ⚠⚠ **`detail → abstract`. THE ONE HARD RESEARCH PROBLEM.** |
>
> **The project's single named unsolved problem — upscaling an irreducible fine edit into a memoized
> macro with correct up-invalidation — is the INTERCESSION CELL of the access matrix.**
>
> And the matrix *already says what is true about that cell*: **it is the only channel by which a
> lower level can change a higher level's noumenon, and it is MEDIATED — it cannot be done directly;
> it must pass through an operator the higher level owns and consents to.**
>
> **That is not a metaphor about the research problem. It is a specification of it.** The fine tier
> cannot upscale its own edit, because the coarse tier's value is not a summary the fine tier is
> entitled to overwrite — **it is a LAW the fine tier was licensed by.** Up-propagation must therefore
> be an operation the *coarse* tier performs *on being petitioned*, and it must **invalidate the
> coarse tier's own downstream derivations** (which is exactly what `multiscale-seams.md` §2.4 says
> the unsolved half is). **The ontology and the numerics are describing the same object, and the
> ontology got there first.**

I hold this at `[me]`, and I would want it attacked. **The obvious attack: "you have found an analogy
and dressed it as a structure."** My defence is that the *operations* are identical, not merely
similar — same conditional, same conservation constraint, same no-trace property, same
mediation-requirement — and that **the correspondence PREDICTS the shape of the hard problem** (that
up-propagation must be mediated and invalidating) rather than merely re-describing it after the fact.
**That is falsifiable and it is the test I would put to it.**

*(And if it holds, it says something I find quietly startling: the reason `detail→abstract` is hard is
**not** that nobody has done the numerics. It is that **it is the one operation the ontology says is
hard** — the endo agent reaching up through the boundary — and it is hard for the same reason there,
too.)*

---

### The three ladders, revisited a third time — and the scale ladder is the kingdom ladder.

`01-…` predicted phase-time / scale / maturity were one recursion. `04-…` broke it (only the fidelity
ladder can demote). **Now the scale axis turns out to be the KINGDOM axis** — nested, with an access
matrix, a revelation channel, and a mediated intercession channel.

⇒ So: **phase-time · scale · kingdom-nesting are the same recursion** *(promote-converged-state-to-law,
and the law then constrains what runs below/next)*, and **maturity is the odd one out — because
maturity is not a ladder in the WORLD. It is a ladder in our KNOWLEDGE of the world.** Which is why it
is the only one with a *retraction* operator (`defeasance`), and why it is the only one whose top rung
requires a **probe** rather than a computation. **The epistemic ladder is a different kind of object,
and conflating it with the ontological ones is probably a mistake I was about to make.**

---

### A smaller one, and it is a warning I want to leave for whoever writes the five box docs.

**This document is 524 lines and it is the best thing in the repository.** It is also, by its own
account, **wrong in at least five places at the moment of writing**, and it knows it — the corrections
are dated *the same night*. The five nomos-contract files are briefed to be *"a PROCEDURE, not a
taxonomy,"* to *"carry their own PROBE,"* and to carry a *"FAILURE GALLERY."*

**This document is what that brief produces when it succeeds.** And notice *why* it succeeded: **not
because its author was careful — he was wrong constantly — but because it was ATTACKED, immediately,
by agents reading primaries against it, and because it had somewhere to PUT the retraction.**

> **⇒ The five documents will be worth exactly what the red-team pass on them is worth, and nothing
> more. Budget for the attack, not for the writing.** *(And per §4.1a: the sharpest attack available
> is not "critique the doc" — it is **"BUILD SOMETHING FROM IT AND SEE IF IT WORKS."** An agent tried
> to design an atmosphere from the structure table and the table broke. That is the probe.)*

---

## Tactical residue — quarantined

- `Conservation::Conserved` does not distinguish **global** from **local** conservation, and §4.2
  proves a scheme can pass a global audit while every local budget is wrong. One-word fix, real bite.
- §6's item 5 — **decide the column semantics** — is *"the fork everything else waits on"* and is
  still open. Every mesh question downstream of it is formally unanswerable.

## Queue changes

**Withdrawn:** my `05-…` claim that "the five boxes are an inventory, not a discovery" is a reframe.
**It is §2.4a**, and I re-derived it without reading it. Recorded above in full.
**Next:** `ASSUMPTIONS.md` (the ledger that `include_str!`s into the tests — one of only two artifacts
in the repo that can fail a build).
