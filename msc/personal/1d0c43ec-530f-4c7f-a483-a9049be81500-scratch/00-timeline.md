# 00 — the timeline

*Built 2026-07-13 at Joseph's suggestion, from `git log` (not from memory). **Every date here is a
commit timestamp.** Maintained as the reading pass proceeds.*

> ## ⚠ CORRECTED 2026-07-13 — Joseph caught the method. **Renames hide the real ages.**
>
> My first cut read birth-dates off `git log <path>`, which reports the **reorg** (Jul 11) or the
> **format migration** (`.md`→`.udon`) as a birth. It is not. Traced through the rename chains:
>
> | lineage | **concept born** | **mechanized as data** |
> |---|---|---|
> | `Checkpoints.md` → `PHASES.md` → `tellus.ordinum.udon` → **`terrestris.ordinum.udon`** | **Jul 02** (11 days) | **Jul 11** |
> | `LEXICON.md` → **`LEXICON.udon`** | **Jul 04** (9 days) | Jul 11 |
> | **`ARCHITECTURE.md`** *(the Jul-11 date was the doc-tree reorg)* | **Jul 04** (9 days) | — |
> | **ETHICS** — *"fold ethics + new objectives into docs"* | **Jun 20 — DAY ONE** | moratorium enshrined **Jul 06** |
>
> **⇒ And the correction SHARPENS the finding rather than softening it:**
>
> # **THE CONCEPTS ARE OLD. THEIR MECHANIZATION IS NEW. THE GAP BETWEEN THE TWO DATES *IS* THE PROJECT'S DISEASE.**
>
> The phase ladder waited **9 days** between being *thought* (Jul 2) and being *machine-readable*
> (Jul 11) — and the day it became data, `ordinum.rs` immediately caught **two undeclared nomos** and
> **convicted the world of running erosion without land.** Nine days of that being true and invisible.
>
> **Now apply the same clock to what is still prose:**
>
> | declared | mechanized | **days waiting** |
> |---|---|---|
> | **column semantics** (`DESIGN-MATERIAL` §4 — *"so a later tier can't quietly treat it as a mean"*) | ⛔ **never** | **12 — half the project's life** |
> | **the sufficient-statistic contract** (first named `DESIGN-REDUX`, Jul 1) | ⛔ **never** | **12** |
> | **`ARCHITECTURE` §9's six-clause contract** | ⚠ **2 of 6** | **9** |
> | **the four missing boxes** (②③④⑤) | ⛔ never | 9–12 |
>
> **Every one of them would convict something the moment it became data. That is not a prediction —
> it is what happened to the ordinum, on the day it did.**

---

## ⏱ The real ages (rename-traced)

**The repository is 24 days old.** First commit **2026-06-20**.

| artifact | true age |
|---|---|
| the project · `DESIGN.md` · ethics | **24 days** (day one) |
| `erosion.rs` / `water.rs` — *the kernels the audit convicted* | **12 / 11 days** |
| `DESIGN-MATERIAL/-REDUX/-SYSTEMS` — *incl. §4's semantics declaration* | **12 days** |
| **the phase ladder** (`Checkpoints.md`) | **11 days** |
| `LEXICON` · `ARCHITECTURE` · `ASF.md` · `multiscale-methods` | **9 days** |
| **the moratorium** | **7 days** |
| the **scaffold** (`store` → `nomotheke` → `ASSUMPTIONS`) | **3 days** |
| `flux.rs` · the flux web | **~36 hours** |
| the ordinum **as data** · `ordinum.rs` | **2 days / 26 hours** |
| ⚠ `DECISIONS.decision-log.udon` — *the ledger I had been treating as the constitution* | **26 hours** |
| `doc/theory/discretisation-and-information.md` — *the theoretical core* | **17 hours** |
| `NOMOS-CONTRACT.md` — *the thing we are about to build* | **6 hours** |

**I have been calling documents "stale" that are days old.** They are not stale. **They were
overtaken.** Different diagnosis, different remedy: you do not *sweep* a doc that a leap outran — **you
build the thing that lets leaps propagate.**

---

## ⚑ THE FRAME CORRECTION — Joseph, 2026-07-13. **The project's PURPOSE changed mid-flight.**

> *"The project, notwithstanding the ethics, **has genuinely transformed** from 'strictly casual and for
> fun — a distraction from ASF-rigor work' to **'serious project that gives worlds to ASF that allow
> in-vivia studies'** (and maybe some games as well) — as another leg for the Archema research program
> generally. **So the trend toward rigor wasn't initially expected but is not accidental.**"*

**This is the missing variable, and the timeline shows the promotion to the hour:**

| | |
|---|---|
| **Jun 20 – Jul 3** | **A GAME.** `erosion.rs`, `water.rs`, MFD, the fBm prior. Written to a *game's* bar — **which was the correct bar.** |
| **Jul 4, 09:53** | ⚑ **`ASF.md` — *"vivarium takes its place as a supporting project for ASF/AAT."*** |
| **Jul 6** | ⚖ **The moratorium.** Ethics becomes binding law. |
| **Jul 10 →** | **The rigor machinery**: the frame, the nomotheke, declared-vs-derived, the epistemic axes. |
| **Jul 13** | **The audit.** The kernels measured — for the first time — against the NEW standard. |

> ### ⇒ **THE AUDIT IS NOT A FAILURE REPORT. It is the first honest measurement of a GAME's physics against a RESEARCH INSTRUMENT's requirements — and of course it came back red. The kernels were never built to be measured that way.**
>
> **There is no "technical debt" here, because debt cannot accrue against a standard that did not
> exist.** I wrote a paragraph in `09-…` calling it *"poignant"* — the fast-and-working choice
> outrunning the correct-and-slow one. **That reading is wrong.** Nobody chose speed over rigor. **The
> project was promoted, and the bar moved under finished work.**

### ⇒ AND IT REORDERS WHAT THE SCAFFOLD *IS*.

If the deliverable is **worlds that support in-vivia studies** — LEXICON's third empirical register,
*"between toy models (ground truth, trivial dynamics) and field data (rich dynamics, no ground truth):
richer than the first, more legible than the second"* — then:

> # **A world whose epistemic status you cannot STATE is worthless as an instrument, however beautiful it looks.**

**⇒ The declared-vs-derived machinery, the fidelity pyramid, the maturity ladder, the recorded
convergence-ε / unLawfulness budget — these are not hygiene AROUND the product. THEY ARE THE PRODUCT.**
The world is the medium; **the WARRANT is the deliverable.**

And that is exactly why `DECISIONS[plate-tectonics-as-an-emergent-regime]` makes the **cube control
mandatory** and says the payoff *"would be **uninterpretable**"* until the grid-locked biases are
fixed. **An emergent result from an uncharacterized kernel is not a weak result. It is a NON-result.**
*(And it would be **extremely seductive** — "a planet with a dozen coherent plates and clean boundaries
**looks like a triumph**.")*

**⇒ So the scaffold is not infrastructure for the science. The scaffold IS the science.** Everything in
this directory has been circling that and I did not have the sentence until Joseph gave it to me.

---

## The eras

### ▸ **Jun 20** — day one. *Docs first.*
`DESIGN.md` is in the **first commit**. *"Seed vivarium: docs-first repo for ASF-agent simulation
game."* The three founding commitments (core/view wall · determinism-as-ontology · the fidelity
invariant) are there before the engine is.

### ▸ **Jun 22–23** — the engine spike. *(45 commits)*
Bevy vs Godot head-to-head; voxels; rendering LOD; the km-horizon problem. **Bevy wins.** *(The word
**"bias"** first appears here — Jun 22 — in a rendering context. It will not mean what it comes to mean
for another 21 days.)*

### ▸ **Jun 29 – Jul 02** — **the physics era.** *(141 commits; 92 on Jul 2 alone — the busiest day in the repo's life)*
- **Jul 1, 23:40** — `erosion.rs` is born. FBM prior → MFD → stream power → Davy-Lague.
- **Jul 2, 02:13** — `water.rs` is born. Virtual pipes, local-inertial shallow water.
- **Jul 1, 23:08** — **`DESIGN-MATERIAL.md`, `DESIGN-REDUX.md`, `DESIGN-SYSTEMS.md` are born.**
- ⚠ **The phrase *"sufficient statistic"* first appears — Jul 1.**
- ⚠⚠ **`DESIGN-MATERIAL.md` §4 declares the column semantics — Jul 1.** *"The conserved primitive is
  volume/mass — NOT a height… surface elevation is a derived reading… a finite-difference node… so a
  later tier can't quietly treat it as a mean/max."*

> **⇒ THE TWO DEEPEST UNRESOLVED THINGS IN THE PROJECT WERE BOTH WRITTEN DOWN ON JULY 1st, IN THE SAME
> COMMIT, AND ARE BOTH STILL OPEN TWELVE DAYS LATER.** The column-semantics fork (*"every mesh, seam,
> and conservation question is unanswerable until this is decided"*) and the sufficient-statistic
> contract. **The kernels were born ~24 hours before the document that said how to build them
> honestly.** *(And `physics-not-knobs` was learned this week too: "real physics is FASTER to code than
> magic-knob hacks" — measured Jul 2.)*

### ▸ **Jul 03–04** — orientation outward.
Hydrology merged. `LEXICON` seeded (Jul 3). **`ASF.md` born Jul 4** — vivarium reframed as a
*supporting project for ASF*, and the word **"fated"** enters the vocabulary.

### ▸ **Jul 06** — ⚖ **THE MORATORIUM.**
`ASF.md` §0. *No endogenous instantiation of frontier/emergence-capable LLMs.* **It is derived from
the access matrix, not stipulated** — and it is the oldest thing in the project that has never needed
a correction.

### ▸ **Jul 09–10** — **the frame.** *(38 commits)*
The world-model clean-room. **`store.rs` (Jul 10, 16:14)** · **`nomotheke.rs` (Jul 10, 20:52)** ·
**`ASSUMPTIONS.md` (Jul 10, 20:29)** · **`ARCHITECTURE.md` v0.3** — *"represent by consequence"* and
*"nomos"* both enter the language on **Jul 10**.

⚠ **`ARCHITECTURE` §9 states the six-clause contract on Jul 10.** *Two clauses have been built.*

### ▸ **Jul 11** — **the codification.** *(12 commits)*
The doc-tree reorg. `LEXICON.md` → **`LEXICON.udon`** (dictionary-grade). **`tabularium/` and the
Terrestris ordinum are born (17:20).** `PROCESS.udon`. The regula design. **Bequest → Promise.**

### ▸ **Jul 12** — **the machinery, and the first corrections.** *(37 commits)*
- **02:33** — `flux.rs`. The shared quantity vocabulary; the closure test that fails a build.
- **15:09** — ⚠ **`DECISIONS.decision-log.udon` is born.** *Everything before this has no decision
  record.*
- 15:44 → 16:45 — **`hydrosphere`** (the first non-field nomos) → **`climate`**. The water chain closes.
- 17:22 — **"the probes were measuring seabed."** The first big self-catch.
- 18:08 → 18:15 — the sea-level probe → **Joseph's correction: *the water-world is the promise KEPT.***
- 18:22 — **`ordinum.rs`: the ladder governs the flux web — and it convicts this world.**
- 18:51 → 22:02 — `grid_lab`; nine grids; **the MFD fan is a BIAS and does not converge.**
- 22:04 — the front doors swept to present truth.

### ▸ **Jul 13, 00:27 → 03:55** — ⚑ **THE AUDIT NIGHT.** *(3.5 hours, 14 commits, eight agents)*

| time | what died |
|---|---|
| **00:27** | `discretisation-and-information.md` is born. **The Prime Question is named — today.** |
| **00:36** | flux-on-the-face: *"and Joseph said so first"* — his model had been misheard **twice**. |
| **01:13** | *"the semantics were DECLARED and the code drifted through the guard anyway."* **§2.4a.** |
| **01:22** | **red-team: all three headline claims die.** |
| **01:31** | **fleet: four agents land, and three of them correct the corrections.** |
| **01:38** | `water.rs` is **better AND worse** than we said — staggered, well-balanced, no null space… and outside its validity envelope. |
| **01:45** | the principle **has a name** (modified equation analysis) and **is a COMPUTATION**. The grid verdict **reverses**. |
| **01:53** | the curl identity **IS violated** — and the replacement kernel **does not fix it**. |
| **02:07** | **isostasy is the uplift nomos.** The keel is half the mechanism. |
| **02:19** | plate tectonics as an **emergent regime** — the first real *in vivia* study. |
| **02:35** | ⚖ **THE GRID, TENTATIVELY DECIDED** (Joseph). Keep the cube-sphere. **Stagger it.** |
| **02:56** | MFD's output is **not a discharge** — it is a boundary integral, ill-posed in the continuum. |
| **03:55** | **`p = 1.1` IS the bias, not its cure** — and our own control had been printing the falsification on every run. |

### ▸ **Jul 13, 11:47 – 12:16** — the handoff.
`NOMOS-CONTRACT.md` (the five boxes). Then, **with Joseph awake**: *the macro tier has two roles, and
conservation buys its freedom from the leaves.*

### ▸ **Jul 13, 12:43 →** — this session.
Orientation; the grid reconciled into ORIENTATION; **the reading pass.**

---

## ⇒ What the timeline says, that no single document does

### 1. **Conceptual velocity vastly exceeds integration velocity — and that gap IS the project's disease.**

Count the leaps: *represent-by-consequence* (Jul 10) · *the nomos* (Jul 10) · *the ordinum* (Jul 11) ·
*the flux web* (Jul 12) · *the ordinum governs the web* (Jul 12) · *the governing principle* (Jul 13,
00:27) · *the Prime Question is a computation* (Jul 13, 01:45) · *conservation is a property of the
data structure* (Jul 13, ~03:00).

**Eight conceptual leaps in four days.** And in the same four days, **the number of enforcement
mechanisms built went from 0 to 5** *(flux-vocabulary closure · ASSUMPTIONS anchors · complete-key ·
producer-uniqueness · BrokenKeeper)* — which is genuinely fast, **and it is not fast enough to keep
up with the ideas.**

> **The docs are not rotting. They are being OUTRUN.** Every "stale" thing I found is a place where a
> leap landed in a ledger and never reached the code, the queue, or the sibling doc. **Sweeping is
> treating the symptom.** The disease is that **integration is manual, and thought is faster than
> hands.**

### 2. **The oldest unresolved decision is the one everything waits on.**

**Column semantics: declared Jul 1. Still open Jul 13.** Twelve days — *half the project's life*. It
has been re-derived independently at least four times (`DESIGN-MATERIAL` §4 · `ARCHITECTURE` §3 ·
`column-is-a-control-volume` · `discretisation-and-information` §3.1/§6.5), each time by someone who
did not know it had been decided before, **and it is not "work" — it is a decision, and it is Joseph's.**

**If one thing comes out of today, I think it should be that.**

### 3. **Everything that has never needed correcting is older than everything that has.**

The moratorium (Jul 6). Determinism-as-ontology (Jul 4). The core/view wall (Jun 20). The fidelity
invariant (Jun 20). **The ontology has been stable since the first week.** What has churned violently
is *the numerics* — and the numerics churned because **they were written before the theory that
governs them** (kernels Jul 1–2; the theory Jul 13).

⇒ **This is not a project that got its foundations wrong. It is a project whose foundations were laid
in the right order, and whose walls went up before one floor of it was finished.** The audit did not
find rot. **It found scaffolding that was never removed, holding up a building that is now strong
enough to stand without it.**

---

*Maintained as the pass proceeds. Next: the `doc/design/` tree — all four born Jul 1 or Jun 20, all
pre-frame, all pre-audit, and **`DESIGN-MATERIAL` §4 was right on day twelve of the project and is
still right.***
