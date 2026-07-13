# 10 — `doc/design/DESIGN-REDUX.md`

*Read 2026-07-13 ~17:55 (§§5, 6, 7, 9, 11–13 in full; §§1–4, 8, 10, 14–15 read, lighter notes).
832 lines. **Born Jul 1**; substantive additions Jul 3 and Jul 9–10.*

---

## 1. Surprisal

### ⚠⚠ §5 CONTAINS THE AUDIT'S f32 FINDING — FROM JULY 3rd — AND IT SAYS IT WAS ALREADY FOUND "THE HARD WAY TWICE."

> **§5, verbatim (2026-07-03):** *"**Water surface interpolates; water depth does not.** The surface
> (bed + depth) is a **continuous field** — bilinear reading is honest at any scale. **Depth is a
> DIFFERENCE of two fields with different smoothness** (continuous surface minus rough fine terrain),
> **and differences do not survive interpolation**… Using interpolated depth produced the **"bubbles"
> render artifact** first and the **bottom-walking pawn** second — **same mistake, two costumes.**"*

**And the audit, ten days later (`ASSUMPTIONS.md`, added 2026-07-13):**

> *"**Principling path — free, and structurally honest: store the water SURFACE η as state instead of
> depth.** η is the smooth, near-constant field; **`depth` is the difference of two large numbers
> (textbook catastrophic cancellation)** — and **η is the field the scheme actually integrates.**"*

> ### ⇒ **THE SAME CONCLUSION, ABOUT THE SAME VARIABLE, REACHED BY A COMPLETELY DIFFERENT MECHANISM — AND IT HAS NOW BEEN FOUND FOUR TIMES.**
>
> | # | when | how it showed up | mechanism |
> |---|---|---|---|
> | 1 | Jul 3 | the **"bubbles"** render artifact | interpolation of a difference |
> | 2 | Jul 3 | the **bottom-walking pawn** | interpolation of a difference |
> | 3 | Jul 13 | a **14 m/day current in a dead-calm lake that never decays** | f32 catastrophic cancellation |
> | 4 | Jul 13 | *(open)* that current possibly **rectifying into net bed transport** | a sign-definite clip |
>
> **`water.rs` still stores `depth`.** Four discoveries. Zero changes to the state variable. **The
> project keeps proving that DEPTH IS THE WRONG STATE VARIABLE and has never changed the state
> variable** — and §5 *already knew why*: **it is a difference, and differences do not survive
> anything.**

### §9 asked the right question about f32 and got the right answer — and it is not the question that bites.

> **§9:** *"**Elevation range** does *not* need [tapered precision] — **f32 resolves ~2 mm at a 20 km
> world.**"*

**That is arithmetically true.** (f32 at a 4000 m datum: 1 ULP = 2.44e-4 m. Fine.) **And it is the
wrong question**, and the gap between the two questions is, I think, a genuinely new principle:

> ### **§9 asked: "can f32 REPRESENT the value?" — YES.**
> ### **The audit asked: "does the round-off RECTIFY into a physical process?" — YES.**
>
> ULP noise in η is *harmless as a representation*. But the momentum step is one-sided
> (`(f + dt·g·hflow·head).max(0.0)`), so the noise **cannot cancel** — a **sign-definite operation
> cannot average out; it is a bias by construction** (`bias-vs-noise`, ratified). The forged current
> **plateaus rather than decaying** (measured to 200k steps). And sediment capacity `C = k·|v|·slope`
> is **linear in `|v|`** — so a nonzero `|v|` on a sloping lake floor is a **nonzero erosion rate**,
> and each pipe is one-way. **Round-off becomes a landform.**
>
> **⇒ A PRECISION ANALYSIS THAT ONLY CHECKS REPRESENTABILITY MISSES EVERY RECTIFICATION BUG.** And
> rectification is the *only* kind that matters, because it is the only kind that **integrates**.
>
> **This is a missing row in the structure table**, and it is the nastiest one, because it is the row
> where **the scheme is exactly right and the representation destroys it anyway**: `water.rs` is
> *provably, bit-exactly* well-balanced — *and it forges a current regardless.* **Structure-preservation
> is not enough if the numbers cannot hold the structure.**

### ⚠ The exactness flag is SPECIFIED, EXISTS IN CODE, AND IS STRIPPED IN EXACTLY THE PLACE IT WOULD FIRE.

**§9 designs it, and names what it is for:**

> *"the **ubit** *is* the '**guaranteed vs. approximate**' flag the seam needs… **The ubit is the
> computational analog of the project's epistemic discipline — *mark the guess as a guess.* A
> world-model whose values know whether they are measured (exact) or plausibly filled (interval) is
> truth-honoring made mechanical.**"*

**And it EXISTS.** `Quantity` carries an exactness flag — `doc/toolchain.md` records `uom` as a
deliberate SKIP *precisely because* **"Quantity's exactness flag is the load-bearing part."**

**And the founding convention is: `rich Quantity at seams, raw f32/f64 in loops`** (ORIENTATION,
"Decisions locked").

> ### ⇒ **THE HONESTY MACHINERY IS PRESENT AT THE SEAMS — WHERE NOTHING GOES WRONG — AND ABSENT IN THE LOOPS, WHERE EVERYTHING DOES.**
>
> The f32 rectification happens **inside `WaterSim::step()`**, in a raw `Vec<f32>`, 200,000 times.
> **The flag that exists to say "this number is approximate" was designed for that number and is not
> carried on it.** *(I do not think the answer is "put `Quantity` in the hot loop" — that is exactly
> what the convention correctly forbids. The answer is that **the loop's error behaviour must be a
> DECLARED property of the nomos**, which is box ⑤, which is `NomosDecl`, which is the rebuild.)*

---

## 2. What matters for today's session

### §12's source-derived versioning is fully specified — in ~40 lines of careful design — and is not built.

The **complete key** is *"the mechanism that makes parallel development safe."* §12 works out the whole
thing: hash the kernel source at build time (`build.rs`); **fold in transitive in-crate deps or hash
coarse enough to include them**; an optional normalized-IR tightening *"only if the build-chain makes
it clean"*; a fallback of colocated version constants **plus a tripwire probe that fails when kernel
source changes without a bump**. And the rule:

> **"over-keying costs recompute; under-keying costs truth. **OVER-KEY.**"**

**Today the versions are hand-written string literals** (`"erosion-2026-07-12b-uplift"`). §12 names the
failure mode exactly: *"with dozens of kernels that discipline **WILL eventually be forgotten**, and…
**a stale memo served mid-iteration doesn't just waste time, it LIES** — you conclude your change did
nothing, or 'works,' against a world your code no longer produces."*

⇒ **This is the fifth type-A check on my list, it is the one with the sharpest teeth, and it is the one
`ARCHITECTURE` §9 clause (4) already requires.** *(And it is the same unbuilt thing as
`consumed ⟹ declared` from `07-…`: both need to know what a kernel actually touches. **Build-time source
introspection buys both.**)*

### And DESIGN-REDUX is the doc the leaf-only-evolution price threatens.

§12's *"pervasive disk memoization at every tier and rate"* and §11's lazy query graph are the premise
that flux-on-the-face cuts against (*"the coarse tier can no longer be evaluated without running the
fine tier"*). **The macro-tier decision resolves it** — *conserved ⇒ governance ⇒ free of its leaves* —
and **that resolution belongs in §12**, which currently has no idea it is under threat. *(Explicitly
`:status proposed` and **not verified**, per its own entry. So: not a sweep. A **pending** one.)*

---

## 3. Wandering thoughts

**I have now found the same idea in this project at five levels of abstraction, and I want to write it
out once, because I think it is *the* idea and nobody has stated it in one place.**

| where | how it is stated |
|---|---|
| **DESIGN-REDUX §9** (Jul 1) | *"the **ubit** — a value that knows whether it is **measured (exact)** or **plausibly filled (interval)**. Truth-honoring made mechanical."* |
| **DESIGN-REDUX §5** (Jul 3) | *"a macro cell honestly wants to be **`{mean, min, max, conserved totals}`** — with a flag marking which invariants are **guaranteed vs approximate**."* |
| **DESIGN-MATERIAL §1** (Jul 1) | *"**a cell is a sufficient statistic, not a number.**"* |
| **ARCHITECTURE §3** (Jul 10) | *"**$R$ per consumer**… each field flagged **guaranteed** vs **approximate**."* |
| **LEXICON §5** (Jul 4) | the four **epistemic axes**, *"which fail INDEPENDENTLY — collapsing them is how vibe hides."* |
| **the ordinum** (Jul 11) | a promise **without a predicate cannot be called fulfilled at any level.** |
| **discretisation §3.9** (Jul 13) | *"the coarse value **plus its declared sub-grid statistic** define a **conditional distribution over admissible fine states**."* |
| **`06-…`** (mine) | that declared statistic **is the Revelation channel** across a kingdom boundary. |

> # **EVERY VALUE IN THIS WORLD SHOULD CARRY ITS OWN EPISTEMIC STATUS, AND THE PROJECT HAS DERIVED THAT SEPARATELY AT EVERY SINGLE LEVEL OF ITS OWN STACK.**
>
> A **float** should know if it is exact or an interval (the ubit).
> A **cell** should know which statistics it guarantees (the sufficient-statistic contract).
> A **nomos** should know what it assumes, preserves, sacrificed, and added (the five boxes).
> A **promise** should know what would convict it (the predicate).
> A **world** should know its unLawfulness budget (the recorded ε).
> A **claim** should know who decided it (the authority tag).
> A **result** should know what it may claim (the in-vivia posture).
>
> **It is one principle, recursing all the way down from the research programme to the mantissa.** And
> at *every* level, the project **designed the flag** and then **stored the bare number**.

**That is not seven problems. It is one, seven times.** And I think it means the scaffold's job can be
stated in a single line, which I could not have written this morning:

> ## **A vivium is an instrument. An instrument that does not report its own error is not an instrument. Vivarium has designed the error-reporting at every level and built it at none — so the whole scaffold rebuild is: MAKE THE VALUES CARRY THEIR OWN WARRANT.**

*(And now Joseph's frame correction lands with full force: **this is why the rigor was not accidental.**
A game does not need its floats to know their own error. **An instrument that ASF will publish from does
— and that is the only thing that changed.**)*

---

**A smaller, stranger thought I'll record because it might be worth something.**

§6's reversion rule: *"**forget along the irreversible axis (time); conserve along the reversible axis
(space)**"* — *evict by probability of return.* Space near the player is reversible (turn around); the
deep past is monotone (zero return probability) → **forget freely.**

**But `01-…` established that a phase is a STATE, not an interval, and `06-…` argued the scale ladder is
a kingdom nesting.** Put those together and something odd falls out: **the "deep past" a vivium may
freely forget is not a time. It is the COARSE TIER.** A phase-transition *"freezes the converged state
into a memo, and that memo IS the macro law the next phase reads."* **The past is not stored as history
— it is stored as LAW.**

⇒ **Which means vivarium does not forget its past. It PROMOTES it.** The Record (*"readable-in-rock…
a lossy shadow… never sufficient to reconstruct the past"*) is the only thing that looks like memory,
and the ordinum **deliberately demotes it to a sanity probe.** The actual past **survives as the
constraint the present must satisfy.**

I find that quietly beautiful, and I think it is also *correct engineering*: it is exactly the
`macro-tier-has-two-roles` result — **governance (a conserved prior the fine tier must satisfy) is free
of its leaves; summary (a restriction of them) is not.** *The past that is LAW costs nothing to keep.
The past that is merely REMEMBERED costs everything.*

**And it is the same theorem as the ontology's:** a kingdom's **law is its identity** (`law-is-identity`),
and changing it forks a new world. **You cannot forget your law. You can only become someone else.**

---

## Tactical residue — quarantined

- **`water.rs` should store η, not depth.** Found Jul 3 (twice), Jul 13 (twice). Still stores depth.
- §9's *"elevation doesn't need posits — f32 resolves 2 mm"* is **true and misleading**; it should carry
  the rectification caveat.
- §12's source-derived version hashing: fully specified, unbuilt; **same unbuilt thing** as
  `consumed ⟹ declared`.
- §12 needs the macro-tier resolution folded in *when Joseph ratifies it* (it is `:status proposed` and
  **unverified**).

## Queue changes
**Next:** `DESIGN-SYSTEMS.md` (the phenomena graph + the fluvial inventory — where I expect `G2` to bite
hardest: the audit's convicted kernels presented as sound).
