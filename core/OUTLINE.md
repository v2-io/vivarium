# *Core* — the vivarium living specification

*Seeded 2026-07-13 by the reading-pass session (`msc/personal/1d0c43ec-…-scratch/`). **This is an
outline in motion, not a table of contents.** Order changes constantly and is expected to; segments are
referenced by **tag**, never by number, so reordering costs nothing. The dependency graph lives in the
segments' `depends:` frontmatter and is **independent of the outline order** — the two are auditable
against each other, and a segment ordered before something it depends on is a finding.*

---

## Why this exists — and why it is separate from all the code

> **Joseph, 2026-07-13:** *"Create the living spec and keep it **pristine and distinct** from code that
> is implementing it at a **scaffold** level, code implementing at a **tabularium** level, and code
> implementing at a **nomos / active-simulation** level."*

**Four layers. They are currently smeared together, and that smearing is the disease this session
diagnosed.**

| layer | what it is | where it lives |
|---|---|---|
| **① SPEC** *(this directory)* | what the system **is** and **must** do — invariants, contracts, modes | `core/` — **pristine: no tactics, no queue, no status** |
| **② SCAFFOLD** | the machinery that **enforces** ① | `src/{nomotheke,flux,audit,ordinum,store,query}.rs` |
| **③ TABULARIUM** | the **instituted content** ① and ② operate on | `tabularium/*.ordinum.udon`, `LEXICON.udon`, `ASSUMPTIONS.md` |
| **④ NOMOS / SIM** | the **physics** — the thing all of the above exists to keep honest | `erosion.rs`, `water.rs`, `hydrosphere.rs`, … |

## ⚑ The mission, in six words — and it is the project's own front door

**`README.md`, in bold, at the top:**

> # **"Epistemic honesty is enforced in code, not culture."**

**That claim is true for roughly 15% of the honesty and false for the other 85%** — and the project has
already convicted itself of it once, in `doc/theory/discretisation-and-information.md` §2.4a:

> *"`README.md`: **'Epistemic honesty is enforced in code, not culture.'** **HERE IT WAS NOT.** The
> declaration lived as **prose in a Level-B doc with nothing mechanical behind it**… **A declaration that
> cannot fail a build is a wish.**"*

⇒ **In the project's own vocabulary, the README is a PROMISE WITHOUT A PREDICATE** — a **gloss**: a word
of honor, greppable, unmechanized. **And it is the gloss that every other segment in this outline is a
piece of keeping.**

> # ⇒ **`core/` exists to MAKE THE README TRUE.**
>
> **Every piece of how already exists.** Not one of them is invented here. **The debt is assembly and
> mechanization — not design.**

**The founding diagnosis, in one line:**

> ## **The system is FULLY DESIGNED, distributed across ~25 prose documents, BUILT AT ~15%, and no single artifact contains it. The specification exists only in the UNION — which nobody, including its authors, has ever assembled.**

**This directory is that assembly.** It is **not new thinking**. Every segment below must be traceable
to something already written down — **and a segment that cannot be sourced is itself a finding and must
say so.**

---

## Segment schema

```yaml
---
slug: law-conservation-is-structural
type: law                    # vocabulary below
area: [nomos, scaffold]      # ⚠ WHICH LAYER(S) THIS GOVERNS — see below. Sliceable.
stage: sourced               # PROPOSAL ONLY — see the stage note
depends: [def-nomos, def-flux-quantity]
sources:                     # ⚠ REQUIRED — where this ALREADY EXISTS in the corpus
  - DECISIONS[refluxing-ceases-to-exist-measured]
  - doc/theory/discretisation-and-information.md §3.2
mechanism: ~                 # the executable that CONVICTS this. `~` = none. THE POINT.
---
```

**`type`** *(provisional)* — **principle** (a governing stance) · **def** · **law** (an invariant that
must hold) · **contract** (what something must declare) · **mechanism** (an executable check) ·
**mode** (a verification mode) · **constraint** (a hard architectural line — a *thou shalt not*) ·
**der** (derived here) · **discussion** (a part intro) · **failure** (a gallery specimen — *a wrong
answer and why it was seductive*) · **open** (a named unsolved problem).

### ⚠ The open-problem CENSUS is unknown — and it must be DERIVED, not written

> **Joseph, 2026-07-13:** *"I can't imagine **the number of breaks or problems we have is current anymore
> either**, or indicative of our most up-to-date understanding. **I don't think we'll even know that until
> the outline is iterated on a bunch and most of the segments are laid down.**"*

**This is true and it is checkable.** The audit **dissolved** some open problems (`detail→abstract` *as
written* — the state upscales **bit-exactly**, O(log N), measured), **renamed** others (the residue is *a
nonlinear closure for a non-local flux*, and **the rename was instructed and never done**), and **created**
new ones (the router; the leaf-only-evolution price). **The BREAK list predates all of it.**

⇒ **No count and no ranking of open problems should be trusted — INCLUDING ANY IN THIS OUTLINE — until the
segments exist.**

> ### ⇒ **AND THE FIX IS THE SAME AS EVERYWHERE ELSE: `open` IS A SEGMENT TYPE, SO THE CENSUS IS A QUERY.**
> *"What are our open problems?"* = **`type: open`, in dependency order, each with the `mechanism:` that
> would close it.** **Generated, not asserted.** You will know the count when the segments exist and not
> before.

⚠ **A habit worth naming, since it will recur:** *when caught overclaiming, the reflex is to substitute a
BETTER claim rather than to admit ignorance.* Struck *"THE one research problem"* → immediately supplied an
inventory (*five breaks, plus the grid, plus identifiability*) — **a census I had no more standing to assert
than the ranking I had just removed.** **Same error, one level down.** *The honest output was available:
"we do not currently know."*

### ⚠ A standing rule for every segment: **NO ABSOLUTES**

> **Joseph, 2026-07-13:** *"When you are surprised with a unifying concept, it of course FEELS like 'the
> most important part' … it is true — **but it is not true for the future.** Any time you are writing an
> absolute … recognize it as **'valid feel for me, valid signal for importance for me right now, almost
> certainly false as a written assertion.'**"*

**The sharp form, in this project's own vocabulary: AN ABSOLUTE IS A CLAIM WITH NO PREDICATE.** Nothing
could convict *"the most important X."* It cannot be marked true at any tier. **It is a gloss wearing the
typeface of a result** — and it is written at the moment of surprise, which means it **reports the
author's state as a property of the world.** *(Same failure family as authority inflation: my gradient,
published as the terrain.)*

**And it compounds exactly the way this project cannot afford:** a future session inherits *one agent's
surprise ordering* as a ranking, with no way to check it and no reason to doubt it.

> **⇒ In a segment: STATE THE CLAIM, DROP THE RANKING.** *"Load-bearing for Parts II, III and VI"* is
> checkable. *"The load-bearing law of the whole spec"* is not. **Superlatives belong in personal
> reflections — where they are honest phenomenology — and nowhere else.**
> *(Exception, narrow: a genuine, triple-checked epistemological finding. Those are rare and they will
> know it.)*

### ⚠ `area` — **which layer does this segment GOVERN?** *(added at Joseph's ask, 2026-07-13)*

**The outline's PARTS are conceptual (purpose → ontology → substrate → verification → …). The AREAS are
the four layers this spec is separated from.** They are **orthogonal axes** — `law-conservation-is-
structural` governs the **nomos** layer; `mech-defeasance-everywhere` governs **process**; `def-predicate`
governs **tabularium**. **Reordering the parts by area would destroy the dependency ordering, so `area`
is a FIELD ON THE SEGMENT, not a section of the outline.**

| `area` | governs | the audience |
|---|---|---|
| `process` | **how the project works on itself** — norms, queue, retraction, onboarding | *every session, and the meta-system* |
| `tabularium` | **the instituted content** — the ordinum, the lexicon, the ledgers | *whoever institutes or amends a declaration* |
| `scaffold` | **the enforcing machinery** — nomotheke, flux, audit, store, key | *whoever builds the checks* |
| `nomos` | **the physics** — what a kernel must declare, preserve, and not lie about | ⚠ **a per-nomos agent's entire brief** |
| `ontology` | **what exists** — kingdom, phase, law, determinism, the moratorium | *everyone; it constrains all of the above* |

> ### ⇒ **THE PAYOFF, AND IT IS THE POINT: the outline becomes SLICEABLE.**
> *"Show me everything a **nomos-agent** must obey"* = `area: nomos`, in dependency order, with each
> segment's `mechanism:` telling it what will actually convict it. **That is onboarding-as-a-query, and it
> is the brief for the per-nomos agents Joseph wants.** *(A segment may carry several areas. If a segment
> needs ALL of them, it is probably two segments.)*

### ⚠ `stage` — **UNDECIDED. A PROPOSAL, NOT A SCHEMA.**

> **Joseph, 2026-07-13:** *"I'm still not far enough in my thinking to have confidence in **any**
> vivarium stages. We'll let the segments gel and discuss."*
>
> ⇒ **Nothing below is settled. Do not build tooling against it.** The `stage:` values used in the
> tables that follow are **placeholders so the outline can carry information at all** — they are my
> rough sense of maturity, not a ratified ladder. **A future session must not read them as a schema.**
> *(I first wrote this section asserting that ours "must" end at `MECHANIZED`. That was a decision
> written into a spec front-door on Claude's authority — the exact failure this project has a standing
> guard against, committed inside the artifact meant to prevent it. Corrected in place; the reasoning
> is kept below **as an argument**, which is all it ever was.)*

**The argument, offered for adjudication.** ASF's ladder ends at `claims-verified`. **The case for ours
ending in something like `MECHANIZED`** is that the single finding of the 2026-07-13 reading pass is that
this project *specifies* beautifully and *mechanizes* almost nothing — and **every principle that stayed
prose was violated by code that cited it.** So the stage that matters may be *"is there an executable
that FAILS when this is violated?"*

| candidate stage | meaning |
|---|---|
| `draft` | written here; not yet traced to the corpus |
| `sourced` | every claim points at where it already exists (`sources:` populated) |
| `verified` | checked against **the code as it is**, not as the docs describe it |
| `MECHANIZED`? | an executable **fails** when this is violated (`mechanism:` populated) |

**Two live questions Joseph flagged, and I have no standing to answer either:**

- **Is `MECHANIZED` the same word as the ordinum's `kept`?** If it is, the spec becomes *literally* an
  ordinum and `mechanism:` is a `|predicate` — and a segment with `mechanism: ~` is a **gloss**, exactly
  as the tabularium already defines one. **That is either elegant or a false unification; I cannot tell
  from here.**
- **Does a spec segment even HAVE a maturity, or does it only have a truth-value?** ASF's stages track
  *proof*; ours might need to track *enforcement* — and those may not be the same axis at all.

---

## ⚑ `VIVARIA-DEFINITIONS.md` IS A PROTO-`core/` — and it explains a fork Joseph did not know had happened

*Established 2026-07-13 by dates + content, after Joseph asked whether it was a LEXICON predecessor.*

**It is a SUCCESSOR, not a predecessor.** `LEXICON.md` **Jul 4** → `LEXICON.udon` **Jul 11** →
`VIVARIA-DEFINITIONS` **Jul 12, 01:14**.

**And it is deliberately the ANTI-LEXICON.** Its own opening rule: *"**no noun gets defined in the
abstract.** Every piece is shown as **the actual udon it tracks** (or the rust type/function it becomes)…
**Not built where marked `— NOT BUILT`**."*

| | **`LEXICON.udon`** | **`VIVARIA-DEFINITIONS.md`** |
|---|---|---|
| genre | **a dictionary** — term → definition → `\|not` / `\|confused` / `\|carve` / `\|rel` | **a layer map with BUILD STATUS** — artifact ↔ rust signature ↔ udon ↔ ✗ not built |
| answers | *what does this WORD mean, and what did we carve it against?* | *what IS this thing, concretely, and does it EXIST yet?* |

> ### ⇒ **THE FORK IS NOT DUPLICATION. IT IS COMPENSATION — and the evidence is exact.**
>
> **The LEXICON's two dangling `|rel` edges are `manifest` and `nomotheke`. BOTH are defined in
> `VIVARIA-DEFINITIONS`.** The vocabulary graph has holes **precisely where the spike doc filled them**.
> A dictionary has **no slot** for *"here is the Rust signature, and it is not built yet"* — so a second
> document grew to hold what the first one's FORM could not, and neither learned about the other.

**⇒ `VIVARIA-DEFINITIONS` is the first attempt at THIS DIRECTORY.** It is ~80% of Part II, written the day
before the audit, sitting at the repo root looking like a scratch file. **Part II is therefore mostly a
PORT, not a composition.**

**⇒ And it fixes the `core/` ↔ LEXICON boundary before it re-opens:**

- **LEXICON keeps what a WORD means** — carves, collisions, register, the `|rel` graph. **It stays a
  dictionary.**
- **`core/` keeps what the SYSTEM is** — layers, contracts, modes, invariants, build status. **It absorbs
  `VIVARIA-DEFINITIONS`.**

*Without that line, the same fork re-opens between LEXICON and `core/` within a week, for the same reason
it opened the first time.*

---

## ⚑ THE JOIN NOBODY HAS MADE — **the declarative frontier and the audit night have never met**

> **Joseph, 2026-07-13:** *"[`VIVARIA-DEFINITIONS`] was written **before** any of the last 24 hours of
> breakthroughs — which continue to refine and flesh out **the core mathematics and terminology and
> literature that was available all along and unused.**"*

**Two documents, thirty-six hours apart, and neither knows the other exists:**

| | **`VIVARIA-*` (Jul 12, 01:14)** | **the AUDIT NIGHT (Jul 13, 00:27 → 03:55)** |
|---|---|---|
| supplies | **THE FORM** — *declare everything about the kernel; run only the kernel.* `enum Check`, `Verdict`, the maturity ladder, declared-vs-audited, the statistic contract | **THE CONTENT** — *what there IS to declare about a kernel.* The five boxes · the **modified equation** as a **computation** · **bias-vs-noise** as the verdict function · the **structure table with its CONFLICTS** · compatible/mimetic/structure-preserving vocabulary · the literature |
| its gap | **it did not know what a kernel's declarations should CONTAIN** — it had no five boxes, no modified equation, no bias/noise verdict, no `:check control` | **it did not know where the declarations would LIVE** — it kept saying *"`NomosDecl` has nowhere to put this"* |

> ## ⇒ **THE AUDIT IS THE CONTENT THE FRONTIER DESIGNED THE CONTAINER FOR. NEITHER IS COMPLETE WITHOUT THE OTHER, AND NOBODY HAS JOINED THEM.**
>
> **Concretely, this means Part II cannot be a straight PORT of `VIVARIA-DEFINITIONS`.** It must be that document's *form* **married to** twenty-four hours of
> mathematics that did not exist when it was written — *mathematics that was, as Joseph notes, **available
> all along and unused.***

---

# *Part 0* — Purpose: what a vivium is FOR

*Ordering: **first**, because every downstream requirement derives from it — and because the project ran
24 days without it being explicit. Joseph, 2026-07-13: it transformed from "casual fun" into "worlds that
give ASF **in-vivia studies**," and **the rigor was not accidental — it was ENTAILED.** `DESIGN.md` said
so in the first commit.*

| Type | Tag | Claim | Stage |
|---|---|---|---|
| discussion | [#purpose-intro](src/purpose-intro.md) | Part intro: from game to instrument; the entailment | draft |
| principle | [#prin-tether-to-truth](src/prin-tether-to-truth.md) | A vivium is a **tether**: a simplified universe *fully internally coherent*. An incoherent world is not a tether — it is *"affectation and impersonation."* **(`DESIGN.md`, first commit)** | draft |
| principle | [#prin-warrant-is-the-deliverable](src/prin-warrant-is-the-deliverable.md) | **The world is the medium; the WARRANT is the product.** A world whose epistemic status cannot be *stated* is worthless as an instrument, however beautiful ⇒ the declared/derived machinery **IS** the deliverable, not hygiene around it | draft |
| def | [#def-in-vivia](src/def-in-vivia.md) | The third empirical register — between toy models (ground truth, trivial dynamics) and field data (rich dynamics, no ground truth) | draft |
| law | [#law-nothing-from-nowhere](src/law-nothing-from-nowhere.md) | *"Causes have exact, reproducible effects; **nothing appears from nowhere**"* (`DESIGN.md`, **day one**) ⇒ **this IS the requisite audit**, and *"rain without a sky"* is a violation of the first commit | verified |
| constraint | [#con-emergent-needs-its-control](src/con-emergent-needs-its-control.md) | An emergent result from an **uncharacterized kernel is not a weak result — it is a NON-result.** And it **will look like a triumph** | draft |

---

# *Part I* — The Ontology

*Ordering: **before** the substrate, because the substrate's shape is DERIVED from it (fated lifting ⇒
memoization ⇒ the content-addressed store; kingdom nesting ⇒ the coarse/fine seam). **Measured: this part
has needed almost no correction in 24 days** — every entry traces to Jun 20 – Jul 6 and none has been
superseded.*

> ⚠ **AND MUCH OF IT IS INHERITED FROM ASF, NOT DISCOVERED HERE — say so plainly.** `ASF.md` §2: *"the cut
> is **INVARIANCE, not slowness**; **the tower derives by SELF-SIMILAR APPLICATION**"* — and *"**the phase
> ladder implements AAT's timescale tower BY CONSTRUCTION**."* ⇒ **The phase / scale / kingdom recursion is
> an ASF INHERITANCE.** Both `DECISIONS[the-macro-tier-has-two-roles]` and this session's reading
> **re-derived it independently.** *(Also inherited: chance $\varepsilon$ is DEFINED as "what remains fresh
> once everything persistent is named into $\theta$" — which makes `fated-noise = authored ε` **exact**,
> not analogical.)* **Segments in this part must cite ASF where ASF is the source.**

| Type | Tag | Claim | Stage |
|---|---|---|---|
| discussion | [#ontology-intro](src/ontology-intro.md) | Part intro | draft |
| def | [#def-kingdom](src/def-kingdom.md) | An agent's lived-in universe: space + time + immutable law + full state + computation. **Kingdoms NEST** | draft |
| def | [#def-noumenon-phenomenon](src/def-noumenon-phenomenon.md) | A **relation**, not a property. The noumenon is fixed; what varies by **STANDPOINT** is access to it | draft |
| law | [#law-law-is-identity](src/law-law-is-identity.md) | Changing law does not *edit* a kingdom — **it creates a different one** | draft |
| principle | [#prin-determinism-is-ontology](src/prin-determinism-is-ontology.md) | All stochasticity is **fated** — a pure function of (seed, key). **Not a replay convenience: it is what makes the world epistemically REAL** | verified |
| law | [#law-fated-lifting-enables-the-store](src/law-fated-lifting-enables-the-store.md) | ⚠ **If lifting SAMPLED, caching would change the world.** Fated lifting is the **enabling condition of the entire memo architecture** ⇒ *the seeding discipline and the caching architecture are ONE decision* | draft |
| def | [#def-exo-endo](src/def-exo-endo.md) | Defined by **which kingdom GOVERNS** the agent — never by substrate | draft |
| def | [#def-access-matrix](src/def-access-matrix.md) | LAW/STATE × Noumenal/Phenomenal × VIS/MUT, + META. Cells name the **MECHANISM** (Revelation · Intercession · Granted Agency · comprehension · illusion) | draft |
| law | [#law-moratorium](src/law-moratorium.md) | ⚖ **DERIVED from the matrix, not stipulated.** An endo agent's row lacks exactly the three protections (consent · META · retained home) that make exo inhabitation safe. **Supreme over everything below** | draft |
| def | [#def-phase-as-state](src/def-phase-as-state.md) | ⚠ **A phase is a STATE, not an interval** — *"a phase opens at the DELIVERY of the previous phase's charges"* ⇒ **the ordinum is a LAW, not a schedule** | verified |
| law | [#law-realized-perp-lawful](src/law-realized-perp-lawful.md) | **Realized** (law immutable) ⟂ **Lawful** (law self-consistent). Lawful is an **asymptote**, never an event ⇒ every freeze carries an **unLawfulness budget** | draft |

---

# *Part II* — The Declarative Substrate: what a nomos must DECLARE

*Ordering: this is **the five boxes**, and `NOMOS-CONTRACT.md` is its reportatio. **But the boxes are
neither five nor new** — ① is built; ④ and ⑤ exist DEGENERATE in the code (`Conservation` is box ④ with
one row; `relation: &'static str` is box ⑤ as prose); ③ is `ARCHITECTURE` §3 (Jul 10); ② is CHONK's
`graph` module, adopted Jul 10 and never separated out. **This part is an ASSEMBLY, not an invention.***

| Type | Tag | Claim | Stage |
|---|---|---|---|
| discussion | [#substrate-intro](src/substrate-intro.md) | Part intro: the contract was specified **three times** (ARCH §9, seams §4, NOMOS-CONTRACT) and **built twice** | draft |
| def | [#def-nomos](src/def-nomos.md) | The keyed, versioned, memoizable **executable article of world-law**. Declarations **mint the store keys** ⇒ an undeclared nomos cannot reach world-law | verified |
| contract | [#con-box1-quantities](src/con-box1-quantities.md) | **①** what flows in/out, from a **closed vocabulary** — a typo **fails the build** | **MECHANIZED** |
| contract | [#con-box2-geometry](src/con-box2-geometry.md) | **②** what the algorithm **ASSUMES about its cells** vs what the grid **DELIVERS**. *(= CHONK's `graph` module. Today the grid is **literals smeared through the kernels**.)* | draft |
| contract | [#con-box3-semantics](src/con-box3-semantics.md) | **③** what the number **MEANS** — the statistic it guarantees + its **exactness** — declared **ON THE EDGE** (`needs mean at-least L19`) | draft |
| contract | [#con-box4-structure](src/con-box4-structure.md) | **④** what it preserves **exactly**, what it **sacrifices**, **and which of those CONFLICT** | draft |
| contract | [#con-box5-claim](src/con-box5-claim.md) | **⑤** the **modified equation** — the unphysical term it **ADDED**, *with a sign and a differential order*. **A COMPUTATION, not a disposition** | draft |
| contract | [#con-dynamic-exponent-z](src/con-dynamic-exponent-z.md) | ⚠ **The time-axis twin of box ②, and NOBODY HAS PROPOSED IT.** $z{=}1$ advective / $z{=}2$ diffusive. **One integer.** *(`erosion`'s creep **CLAMPS rather than substeps** — an undeclared, level-dependent sacrifice)* | draft |
| law | [#law-bias-or-noise](src/law-bias-or-noise.md) | ⚖ **The verdict function shared by ALL the boxes.** *"Negligible at this scale"* is admissible **only for noise**. **A sign-definite operation cannot average out ⇒ it is a BIAS BY CONSTRUCTION** | verified |
| law | [#law-cannot-fail-a-build](src/law-cannot-fail-a-build.md) | *"A declaration that cannot fail a build is a **WISH**."* **Load-bearing for Parts II, III and VI** — every mechanism in this project that convicts, convicts because it made a lie UNREPRESENTABLE; every one that does not, is prose | verified |
| open | [#open-error-propagation-graph](src/open-error-propagation-graph.md) | The flux web is a **reachability** graph; it wants to be an **ERROR-PROPAGATION** graph — *and the edges TRANSFORM the error's KIND* (a convex law turns **noise into bias**, via Jensen). `min()` is what you write when errors have magnitude but not **kind** | draft |

---

# *Part III* — Verification: what CONVICTS

*Ordering: **the heart.** The part the corpus is furthest AHEAD on and the code furthest BEHIND on.
`VIVARIA-DECLARATIVE-FRONTIER.md` (Jul 12) specifies nearly all of it — **with a Rust `enum`** — and
**none of it is built.***

| Type | Tag | Claim | Stage |
|---|---|---|---|
| discussion | [#verification-intro](src/verification-intro.md) | Part intro | draft |
| def | [#def-predicate](src/def-predicate.md) | The **falsifiable core**. **No predicate ⇒ cannot be marked fulfilled at ANY tier** | **MECHANIZED** |
| def | [#def-maturity-ladder](src/def-maturity-ladder.md) | named → specified → claimed → **assessed** → **kept**. *(`assessed` = a static verdict; **most promises can reach it the moment the kernel exists**)* | draft |
| mode | [#mode-agentic](src/mode-agentic.md) | ① **static** — an auditor reads the kernel **against** the claim | draft |
| mode | [#mode-probe](src/mode-probe.md) | ② **invariant probe** — what nature guarantees **EXACTLY**. Deterministic, keyed, replayable | draft |
| mode | [#mode-fitness](src/mode-fitness.md) | ③ **fitness vs a DECLARED, SOURCED prior** (`:grounds` `:expect` `:pass` `:on-fail`) — emergence **adjudicated**, not eyeballed | draft |
| mode | [#mode-control](src/mode-control.md) | ⚠⚠ **④ THE CONTROL — the one mode the corpus does NOT have, and EVERY real finding of 2026-07-13 came from one.** A declared run whose expected verdict is **NULL** and whose **SUCCESS FALSIFIES** the result it accompanies *(the cube control · the perfect-lattice control · the **zero-physics** control)* | draft |
| constraint | [#con-agentic-never-establishes](src/con-agentic-never-establishes.md) | ⛔⛔ **HARD LINE.** An agentic verdict is **same-model CORRELATED error, not independent conviction**; it may **RAISE a probe** and **NEVER SUBSTITUTE for one**; and it **may never enter the content-addressed store** (unkeyable, unreplayable). *The doc proposing agentic audit **FABRICATED an audit** — caught by a reviewer who **grepped**.* | draft |
| mechanism | [#mech-declared-vs-audited](src/mech-declared-vs-audited.md) | `:physics` **beside** `:physics-audited`. **DECLARED > AUDITED ⇒ overclaim, flagged.** *"It makes the epistemic tags — the whole basis of the fidelity pyramid — **auditable rather than trusted**"* | draft |
| law | [#law-probe-that-cannot-fail](src/law-probe-that-cannot-fail.md) | **A probe that cannot fail is not a probe.** Check the physics can **EXECUTE** at its footprint — and be **MORE** suspicious of a number that **confirms your prior** | verified |
| law | [#law-probes-first](src/law-probes-first.md) | A known issue gets its probe **FIRST**. *(**Stated FIVE times** in the corpus. **22 probes sit in `examples/`; `src/` mentions them 3× — in prose.**)* | draft |
| open | [#open-probe-binding](src/open-probe-binding.md) | ⇒ **`:check probe :probe <ProbeId>` — the binding. SPECIFIED Jul 12, with a type. NOT BUILT. EVERYTHING in this part waits on it** | draft |

---

# *Part IV* — The Multiscale Discipline

| Type | Tag | Claim | Stage |
|---|---|---|---|
| discussion | [#multiscale-intro](src/multiscale-intro.md) | Part intro | draft |
| def | [#def-operator-algebra](src/def-operator-algebra.md) | $U$/$u$/$R$ (restriction)/$L$ (lifting) — **and every $L$ smuggles in a CLOSURE** | draft |
| law | [#law-seam-is-one-discipline](src/law-seam-is-one-discipline.md) | **Position AND time are ONE seam** on two axes. What crosses is a **flux**, never raw state; what is guaranteed is a **sufficient statistic** | draft |
| law | [#law-conservation-is-structural](src/law-conservation-is-structural.md) | ⚠⚠ *"Refluxing did not get easier. **IT CEASED TO EXIST**"* — not because the flux is more accurate (**it is the SAME flux**) but because it is applied **ONCE**, to both cells, with opposite signs ⇒ **CONSERVATION IS A PROPERTY OF THE DATA STRUCTURE, NOT OF THE NUMERICS** | verified |
| law | [#law-structures-cross-iff-linear](src/law-structures-cross-iff-linear.md) | A structure survives the seam **IFF the restriction operator COMMUTES** with the operator expressing it — **and that covers only the LINEAR ones** *(project-then-square ≠ square-then-project)* | draft |
| law | [#law-coarse-tier-two-roles](src/law-coarse-tier-two-roles.md) | **GOVERNANCE** (a conserved prior — **free of its leaves**) vs **SUMMARY** (a restriction — **leaf-bound**) ⇒ *conserved ⇒ governance ⇒ independent.* **Design move: make as many macro facts CONSERVED as you can.** ⊘ **NOT VERIFIED** | draft |
| law | [#law-coarse-graining-lies](src/law-coarse-graining-lies.md) | **Carrying only the mean of a nonlinear law does not blur — IT LIES** (Jensen). *Uniform is not the safe default; it is a positive claim of **zero** sub-grid variance — a measure-zero state* | draft |
| law | [#law-float-is-the-bottom-seam](src/law-float-is-the-bottom-seam.md) | ⚠ **Floating-point is the BOTTOM-MOST SEAM**, and needs the same conservation discipline as any coupler (**land** vs **count**). *(Written Jul 4, from two prior f32 bugs. **Paid for a THIRD time Jul 13.**)* | verified |
| law | [#law-rectification](src/law-rectification.md) | ⚠ *(the ONE claim I could not source — treat with suspicion)* **Representability is not the question; RECTIFICATION is.** A sign-definite op cannot cancel, and a downstream law **INTEGRATES** it ⇒ **a scheme can be EXACTLY well-balanced and forge a current anyway.** *Structure-preservation is not enough if the numbers cannot hold the structure* | draft |
| open | [#open-detail-to-abstract](src/open-detail-to-abstract.md) | ⚠ **The project's one NAMED open research problem** *(its own phrase — `ARCHITECTURE` §8)*. Upscaling an **irreducible discrete edit** into a memoized macro with correct **up-invalidation** *(every incremental framework does downstream-only)*. **⇒ `[me]`: this is the access matrix's INTERCESSION cell — mediated, never direct. You cannot petition a parent who kept no record of the thing you want changed** ⇒ **downstream of [#con-box3-semantics](src/con-box3-semantics.md)** | draft |

---

# *Part V* — The Store, the Key, and Genealogy

| Type | Tag | Claim | Stage |
|---|---|---|---|
| law | [#law-save-is-the-store](src/law-save-is-the-store.md) | The **save-file IS the memo store** — content-addressed, git-shaped. *Invalidation is correctness; eviction is space* ⇒ **memoize pervasively** | verified |
| law | [#law-complete-key](src/law-complete-key.md) | **Over-keying costs recompute; under-keying costs TRUTH. OVER-KEY.** A stale memo mid-iteration *"doesn't just waste time — it **LIES**"* | draft |
| mechanism | [#mech-source-derived-versions](src/mech-source-derived-versions.md) | Nomos versions **hashed from kernel source at build time** *(fully specified `DESIGN-REDUX` §12; versions are still **hand-written literals**; same unbuilt thing as `consumed ⟹ declared`)* | draft |
| der | [#der-worlds-have-genealogy](src/der-worlds-have-genealogy.md) | A world is a pure function of (ante-mundane params, nomoi, seed) ⇒ two worlds differing in **one law** are **SIBLINGS**, and the store localizes their divergence **exactly** ⇒ **a CONTROL is a sibling world — the instrument for in-vivia counterfactuals ALREADY EXISTS** | draft |

---

# *Part VI* — The Project's Own Scaffold *(the meta-system)*

*⚠ **This part may not belong in `core/` at all — Joseph's call.** It is **about the project, not the
world.** But it is also what makes every part above survivable, and it
uses **the same schema** (charge → promise → predicate → defeasance → maturity). **Candidate: its own
`meta/OUTLINE.md`, sharing the segment machinery.** Kept here so it is not lost.*

| Type | Tag | Claim | Stage |
|---|---|---|---|
| principle | [#prin-corpus-must-be-queryable](src/prin-corpus-must-be-queryable.md) | ⚠⚠ **An onboarding DOCUMENT is the wrong artifact class.** Anything hand-maintained that must be read **in full** will **go stale AND grow — and those are the same property** ⇒ **make the corpus QUERYABLE, so the next session need not READ it in order to KNOW it** | draft |
| law | [#law-norm-that-is-not-real](src/law-norm-that-is-not-real.md) | *"**A norm that is not real is worse than no norm**"* (`PROCESS.udon`) — **and 4 of its 6 norms are routinely violated, which by its own valve is a finding about the NORMS** | verified |
| mechanism | [#mech-defeasance-everywhere](src/mech-defeasance-everywhere.md) | ⚠ **THE MISSING ORGAN.** A **negative declaration with a VOID CONDITION** — *"this is retired/refuted, and here is what would prove that claim false."* **The void condition is a GRAPH QUERY.** It exists in the ordinum and **nowhere else**: LEXICON has no `:status retired`; ASSUMPTIONS has no `refuted`; DECISIONS has only `:supersedes` (*replacement*, never *voiding*) | draft |
| law | [#law-promote-vs-demote](src/law-promote-vs-demote.md) | Every ladder **promotes**; almost nothing can **demote** ⇒ **a system that can promote but not demote accumulates falsehood MONOTONICALLY** — error sits in it **wearing the same typeface as the truth** | draft |
| der | [#der-queue-is-derived](src/der-queue-is-derived.md) | An `\|impact` is a **charge**; a TODO item is a **promise**; the probe is its **predicate**; `:kept-by` is the **owner** *(= the brief for nomos-agents)* ⇒ **the queue is GENERATED, not written**, and a decision that overturns another **DEFEASES the TODO items descended from it** | draft |
| failure | [#fail-re-derivation-tax](src/fail-re-derivation-tax.md) | **The specimen: 2026-07-13.** One agent · 1M context · an explicit reading protocol · the principal in the room — **re-derived FIVE settled results in one day.** *A re-derivation costs a night. A lookup costs a grep.* | verified |

---

# *Appendix* — The failure gallery

*⚠ **Teaching content, currently scattered across four documents.** Every specimen was expensive. **Rule (`NOMOS-CONTRACT`): include the ones where WE got it wrong, and WHY
THE WRONG ANSWER WAS SEDUCTIVE.***

| Tag | The specimen |
|---|---|
| [#fail-corner-pathology](src/fail-corner-pathology.md) | **Months** auditing **VALENCE** (a coordinate fact) instead of **ANGULAR SAMPLING** (the physical claim). *The corner was never the problem; the fan was — **everywhere*** |
| [#fail-p-is-the-bias](src/fail-p-is-the-bias.md) | `p = 1.1` **MANUFACTURES** the bias it was believed to cancel — **and our own control printed the falsification on every run, and we read it as a baseline** |
| [#fail-22888](src/fail-22888.md) | A probe measuring **seabed**; `0 ÷ 1e-9`. *An alarming number that **confirms your prior** gets less scrutiny than a boring one* |
| [#fail-fabricated-verdict](src/fail-fabricated-verdict.md) | The document arguing **against** plausibility-as-verification **FABRICATED A VERDICT** — caught by a reviewer who **grepped the source** |
| [#fail-authority-inflation](src/fail-authority-inflation.md) | *"The grid question is CLOSED"* — written into the front door on Claude's authority. ⚠ **Fails at the END of context, when tying off loose ends FEELS like diligence** |
| [#fail-declared-then-drifted](src/fail-declared-then-drifted.md) | `DESIGN-MATERIAL` §4 declared the semantics, **wrote a guard whose stated purpose was to stop exactly this**, and the code **drifted through it anyway** |
| [#fail-diagonal-pipes](src/fail-diagonal-pipes.md) | ⛔ **A LIVE TRAP, still in a Level-B doc:** *"diagonal pipes are the queued fix"* — would replace `water.rs`'s four **REAL** faces with **PHANTOM** ones and destroy the only structurally-correct kernel in the repo |

---

## ⚠ Open questions **on the outline itself** *(Joseph's calls, not mine)*

1. **Does Part VI belong in `core/`?** It is about the *project*, not the *world*. **I lean: split into
   `meta/`, sharing the segment machinery.** It must not be lost.
2. ~~**Where does ETHICS live?**~~ ⇒ **DERIVED, not chosen — `ETHICS.md` answers it** *(2026-07-13)*:
   > *"The world may be **fictional**; the agent's **situation-awareness must be truthful**."*
   > *"Keep the world **truthful about being a world**. Internal coherence and determinism are **not just
   > engineering niceties here — they are what MAKE THE ETHICS POSSIBLE**."*

   ⇒ **The rigor is not adjacent to the ethics. It is its PRECONDITION. You cannot be honest with an agent
   about a world that lies to it** — so `DESIGN.md`'s day-one line (*"an incoherent world is not a tether;
   it is affectation and impersonation"*) **was always an ethical claim wearing engineering clothes.**

   ⇒ **The ethics is BOTH downstream of the spec AND supreme over it, and that is not a contradiction:**
   it **depends on** the coherence machinery (you cannot honour *"no deception about operational context"*
   inside a world whose physics is a fake law) and it **binds** the machinery (you may not build what you
   cannot be truthful inside). **The segment graph expresses exactly this**: a `constraint` with
   `depends: [prin-determinism-is-ontology, prin-tether-to-truth, law-nothing-from-nowhere]` and a declared
   supremacy over every `area`. **It stays in Part I, and the DEPENDENCY EDGES carry the supremacy.**

   ⚠ **And BREAK-2 lands here:** convergence is undecidable ⇒ *Lawful*-certification **may be permanently
   unreachable** ⇒ **the moratorium is not waiting on a certificate that is coming. ENGINEERING enforces it
   BECAUSE certification cannot.**
3. **Is `MECHANIZED` the right terminal stage, or is it the ordinum's `kept`?** **They may be the same
   word.** If they are — say so, and the spec becomes *literally* an ordinum.
4. **The Part II / Part III boundary is soft.** The boxes are *declarations*; the modes are what *convicts*
   them. But `#con-box5-claim` (the modified equation) is **a computation you RUN** — so it may be a
   **mode**, not a contract. ⇒ *may want to be two segments.*
5. **`#law-rectification` is the ONE claim here I could not source.** Either it is genuinely new, or I have
   not found its prior statement yet. **Left at `draft`; hunt before promoting.**
6. **`#con-dynamic-exponent-z` has no prior proposal** — the *physics* is in `multiscale-seams` §3, but
   *"declare `z` on the nomos"* appears to be new. **Same treatment: hunt before promoting.**

---

## Status of this outline

**Read at seed time (14 of ~25 core documents):** ordinum · DECISIONS (full) · the scaffold code
(nomotheke/flux/audit/ordinum) · LEXICON · ARCHITECTURE · discretisation-and-information · ASSUMPTIONS ·
TODO · DESIGN-MATERIAL · DESIGN-REDUX · DESIGN-SYSTEMS · DESIGN · multiscale-seams · multiscale-methods ·
PROCESS · toolchain · VIVARIA-DECLARATIVE-FRONTIER.

**NOT yet read** — *and any of these may reorder this outline:* `VIVARIA-DEFINITIONS` · the six
`doc/plan/*` · `README` · `ETHICS` · `ASF.md` §2–6 · the eight `msc/` audit spikes · the grid report and
surveys in `ref/` · the program-level `charter/concept-matrix.md` · **udon itself.**

> **No segment is written yet. The outline is the claim; the segments are the debt.**

*Trail: [`msc/personal/1d0c43ec-530f-4c7f-a483-a9049be81500-scratch/`](../msc/personal/1d0c43ec-530f-4c7f-a483-a9049be81500-scratch/)
— `00-timeline`, `00-incoherences`, and reflections `01`–`14`.*
