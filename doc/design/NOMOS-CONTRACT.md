# The nomos contract — what a nomos must declare to be true-to-physics

*Written 2026-07-13, after a day auditing the project's kernels. The defects found that day sort into five kinds; the flux web declares only the first. The five boxes were abstracted from those defects, so the sort is a restatement of that sample — not evidence the set is closed.*

> **The purpose is not to fix `erosion.rs`.** It is to **generalize and secure the flux-web structure so that ANY nomos can be true-to-physics** (Joseph, 2026-07-13). MFD-8 is the corpse we dissected to learn the anatomy. This document is the anatomy.

---

## The five boxes

The flux web today declares **one** thing: *"erosion consumes precipitation; climate produces it."* **Quantity matching.** It is necessary, it is real, and it is the **shallowest** layer. Four more are missing.

| # | box | the question it answers | status |
|---|---|---|---|
| **①** | **QUANTITIES** | *What flows in, what flows out?* | ✅ **BUILT** (`flux.rs`, `audit.rs`, and the ordinum governs it) |
| **②** | **GEOMETRY** | *What does this algorithm ASSUME about its cells — and what does the grid actually DELIVER?* | ⛔ **not built** |
| **③** | **SEMANTICS** | *What does this number MEAN? What statistic does it guarantee, at what exactness?* | ⛔ **not built** |
| **④** | **STRUCTURE** | *What does this scheme preserve EXACTLY, what approximately, what does it SACRIFICE — and which of those CONFLICT?* | ⛔ **not built** |
| **⑤** | **CLAIM** | *What unphysical term did this scheme ADD? With a sign and a differential order.* | ⛔ **not built** |

**Box ② is the `consumes`/`produces` pattern applied to GEOMETRY instead of quantities. Box ③ is it applied to MEANING. Box ④ to INVARIANTS. Box ⑤ to the LIE.** The generalization is the same move, four more times.

---

## ⚖ The argument, and it is not a wish-list

**The defects found on 2026-07-13 sort into these five boxes** — the boxes were abstracted from those defects, so this restates the sample rather than showing the set is closed. What the table does show: each defect was found by hand, because nothing declares them. We found each one *by hand*, because **nothing declares them.**

| the defect | the box that would have caught it, mechanically |
|---|---|
| `cell_m²` uniform drainage area (**+17.8%, cube-locked, cannot be out-resolved**) | ② geometry |
| MFD's fan (assumes even angular sampling; grid delivers a sheared Jacobian) | ② geometry |
| Hardcoded `cell_m` / `cell_m·√2` neighbour distances | ② geometry |
| The TPFA's orthogonality assumption (our mesh is 28.79° off) | ② geometry |
| Hydrostatic reconstruction's **proved** failure when `Δz ≥ h` (*our common case*) | ② geometry |
| The column's three-way semantic ambiguity | ③ semantics |
| **MFD's output being a boundary integral, not a discharge** | ③ semantics |
| Jensen — and that the statistic needed is the **joint `(A,S)`**, not `var(S)` | ③ semantics |
| `mean-pin` claiming an invariant (`R∘L = id on the mean`) it **does not hold** | ④ structure |
| θ violating the **entropy condition** (the structure it actually breaks) | ④ structure |
| **`water.rs` IS well-balanced, IS staggered, HAS no null space** — *undeclared assets a rewrite would have destroyed* | ④ structure |
| Routing enforces **acyclicity** (the strictly-downhill clamp) — *recorded as a mystery; it is a structure* | ④ structure |
| θ's physical claim is **NONE** (an **even/Laplacian** diffusion — *which is precisely why it CANNOT be the odd/advective term the file's comment claimed*) | ⑤ claim |
| **`p = 1.1` is a 45°-periodic, grid-locked first-moment deflection** — it *manufactures* the bias it was thought to cancel | ⑤ claim |
| Jarrett-as-a-dynamical-law (a **regression for estimating `n` from a measured slope**, used as a feedback) | ⑤ claim |
| The three **one-sided clips** in `water.rs` (*a sign-definite operation is a bias by construction*) | ⑤ claim |
| `uplift`'s output **range** cannot reach its promise's **predicate** | *(the same idea at the ordinum level — a **reachability** check)* |

> **The flux web can answer *"can we rain principled water?"* — a PLUMBING question.**
> **It cannot answer *"is this a bias or noise?", "what does this number mean?", "what did this scheme destroy?", "what unphysical term did it add?"* — the TRUTH questions.**

---

## ⚠ The cross-cutting rule: every box returns a VERDICT, and the verdict is BIAS or NOISE

This is not a sixth box. It is the **adjudication rule shared by all of them**, and it decides *severity* everywhere:

- **NOISE** washes out under summation. The law of large numbers eats it.
- **BIAS** **manufactures a fake physical law** and **compounds**.

**Same magnitude. Categorically different harm. And it reorders the remedies:**

> **"The error is negligible at this scale" is admissible ONLY for noise. If it is a bias, that defence is NEVER admissible at any scale — a fake law does not become true when you zoom in.**

**A sign-definite operation cannot average out ⇒ it is a bias by construction.** (Every clamp, every clip, every `.max(0.0)`, every one-sided limiter.)

---

## The files — a PROCEDURE, not a taxonomy

Each box gets one file at `doc/design/nomos-contract/`. **The failure mode of a doc-per-box is that it becomes a taxonomy nobody reads.** Three disciplines prevent that, and every file must honour all three:

1. **It answers *"how do I FIND OUT what's true here?"* — not *"here are the categories."*** **The declaration is the output; the file is how you EARN it.**
2. **It carries its own PROBE.** *A declaration that cannot fail a build is a wish.* The file must say how to **validate** the claim, not merely how to write it.
3. **It carries a FAILURE GALLERY** — the specimens, **including the ones where we got it wrong.** Agents pattern-match, and *"here is what the wrong answer looked like, and why it was seductive"* does work a list of correct declarations does not. **Every box has real specimens now, and they were all expensive.**

**Each file's shape:** the question · the fundamentals (the math, done, not gestured at) · the procedure · a worked specimen · **the probe** · the failure gallery · what to write in the declaration.

| file | box |
|---|---|
| [`01-quantities.md`](nomos-contract/01-quantities.md) | ① the flux web — what exists, its vocabulary discipline, and the deferred statistic contract |
| [`02-geometry.md`](nomos-contract/02-geometry.md) | ② what the algorithm assumes about its cells |
| [`03-semantics.md`](nomos-contract/03-semantics.md) | ③ what the number means |
| [`04-structure.md`](nomos-contract/04-structure.md) | ④ what is preserved, sacrificed, and what conflicts |
| [`05-claim.md`](nomos-contract/05-claim.md) | ⑤ the modified equation — the Prime Question, mechanised |

---

## ⚑ INTENT — the five agents I was about to launch, and did not

*I ran out of context at exactly this point. I deliberately **did not** launch them, because a fleet writing five unreviewed documents into a tree I could no longer review is the exact haste this project has a standing guard against. Here is the intent, so a successor can launch them in one action with the full brief — or decide not to.*

**Common brief for all five** (each is one agent, they are independent, run them in parallel):

> Onboard: `CLAUDE.md` (telos: segments sole claim truth) · `core/OUTLINE.md` · **this file** · `doc/theory/discretisation-and-information.md` (Prime Question = modified-equation **computation**, not a disposition) · `DECISIONS.decision-log.udon` — **the authority legend, then ALL of 2026-07-12/13** (source material; ~20 entries, nearly all `:by claude :status proposed`) · `ASSUMPTIONS.md` · project memory (does NOT autoload) · the spikes in `msc/`. Archaeology only: `.archive/ORIENTATION.md` HANDOFF residue — not a tactical current dependency.
>
> **Write ONE file: `doc/design/nomos-contract/0N-<box>.md`. It is a PROCEDURE, NOT A TAXONOMY.** Its shape: **the question · the fundamentals (do the math, do not gesture at it) · the procedure · a worked specimen · THE PROBE · the failure gallery · what to write in the declaration.**
>
> **Three disciplines, non-negotiable:** (1) it answers *"how do I FIND OUT what's true here?"* — the declaration is the OUTPUT, the file is how you EARN it; (2) **it carries its own PROBE** — *a declaration that cannot fail a build is a wish*; (3) **it carries a FAILURE GALLERY, including the times WE got it wrong, and WHY THE WRONG ANSWER WAS SEDUCTIVE.** Agents pattern-match; that gallery is the most valuable teaching content this project owns, and every specimen in it was expensive.
>
> **You may NOT mark anything decided** (`:by claude :status proposed`; Joseph decides). Mark measured / derived / inferred / unread. **Verify every claim against the code or the primary — do not inherit mine.** *(The author of your source material was wrong about every headline claim he made on 2026-07-13, and was corrected by agents reading primaries against him. Assume the same of this brief.)*

| agent | box | its richest specimens (all measured, all in DECISIONS) |
|---|---|---|
| **1** | **① quantities** | The flux web as built (`flux.rs`, `audit.rs`); the **ordinum governing it** (a nomos consuming an unkept promise ⇒ the world is *mechanically unrunnable*); the *"rain without a sky"* specimen; and the **deferred `:statistic`/`:exactness` contract**, which Jensen has now made **load-bearing, not optional**. Also: the **reachability check** the ordinum lacks (a promise whose keeper's *range* cannot reach its *predicate* — `uplift`). |
| **2** | **② geometry** | Joseph's own metric set (area · edge · **centre-to-mid-edge arm** · angle · valence · **the joint STENCIL** · **curvature SPLIT in two** · strata volume). The **Jacobian**: *the fan is its shear; `cell_m²` is its determinant.* What converges away and what cannot (**the shear has no `N` in it**). The **coarse/fine carve** (at L25 the sphere is *gone*; at L2 the sagitta is **24% of the cell**). ⚠ **Failure gallery: we audited VALENCE (a coordinate fact) instead of ANGULAR SAMPLING (the physical claim) and audited a corner that was never the problem.** |
| **3** | **③ semantics** | point-sample vs cell-average vs band-limited **≡ aliased vs anti-aliased** (a cell average **IS** a box filter — this is an identity, not an analogy). The **centroid identity** (a linear reconstruction about the centroid is **volume-exact by construction**). The **(1,6,1)/8 curvature term**. **Jensen — and that at `n=1` the gap is in `A^0.5` and `Cov(A,S)`, NOT in `S`.** ⚠ **Failure gallery: `DESIGN-MATERIAL` §4 DECLARED the semantics, wrote a guard whose stated purpose was to stop exactly this, and the code drifted through it anyway — *a declaration that cannot fail a build is a wish.*** |
| **4** | **④ structure** | The structure table **and which rows CONFLICT** (energy- vs enstrophy-conserving are *different schemes*; a positivity limiter *destroys* the enstrophy budget — **erosion is unusually clean; a rotating fluid is the normal case**). Noether. The mimetic identities. **The seam law: structures cross IFF the restriction operator COMMUTES — and that covers only the LINEAR ones.** ⚠ **Failure gallery: `water.rs` IS well-balanced, IS staggered, HAS no null space, and routing DOES enforce acyclicity — ALL FOUR UNDECLARED, and we nearly "fixed" a scheme into losing them.** Probes: the **null-space/eigenvalue** probe (*the only instrument we have that shows what we CANNOT see*), still-lake, curl-on-contours, DAG. |
| **5** | **⑤ claim** | **Modified equation analysis** (Warming & Hyett 1974) — Taylor-expand, read off the PDE, get **a term with a sign and a differential order**. **Even operators = diffusion; odd = advection — which is why θ CANNOT be the advective term its own comment claimed.** **Bias vs noise: a sign-definite operation cannot average out ⇒ it is a bias BY CONSTRUCTION.** ⚠ **Failure gallery: θ's comment asserted precisely what its own cited paper denies (¶52) · `p=1.1` manufactures the bias it was believed to cancel · Jarrett (a REGRESSION for estimating `n` from a MEASURED slope) used as a DYNAMICAL LAW, closing a positive feedback · and three one-sided clips nobody had even looked at.** |

**And one more agent I wanted and did not launch:** `NomosDecl` needs the **fields** for ②–⑤ (assumed-geometry, semantics, preserved/sacrificed structure, bias-vs-noise, modified equation). **That is a small, mechanical Rust change and it should probably come FIRST** — the five files describe what to *put* in boxes the data model cannot currently *hold*.

---

## The blocker, and it is mechanical

**`NomosDecl` has nowhere to put any of this.** No field for assumed-geometry, semantics, preserved-structure, sacrificed-structure, bias-vs-noise, or the modified equation.

> **The theory now demands declarations the data model cannot hold.**

That gap is the next build. It is small, and it is the whole point: **`ARCHITECTURE.md` §9's "representation-agnostic interaction contract" has existed as a principle for months and has never been cashed out. The flux web is its first clause. These four are what 2026-07-13 paid for.**

---

## ⚠ And a fact that shapes all of it: we do not have *an* FVM approach

Joseph's plural was doing real work. **We already run four distinct discretisation families in one world:**

| nomos | family |
|---|---|
| `water.rs` | **staggered (Arakawa-C)**, flux-form, exactly well-balanced |
| `erosion.rs` creep | **collocated**, naive-uniform |
| `erosion.rs` routing | **not FVM at all** — a graph/fan scheme with **phantom faces** (47.8% of MFD-8's "flux" crosses no face) |
| `hydrosphere` | **reservoir / box** — no grid whatsoever |
| `climate` | a **box → field** coupling |

⇒ **The formalisation is NOT "pick one FVM."** It is: **declare which family each nomos belongs to — and then declare what happens at the SEAMS BETWEEN FAMILIES.** *That is where structure dies:* the coarse↔fine mortar (**provably not entropy-stable, and it crashes**), the box→field coupling, the collocated↔staggered boundary.

**Structures cross a seam IFF the restriction operator COMMUTES with the operator expressing them — and that law covers only the LINEAR structures.** The nonlinear ones (entropy, and every Jensen-bearing law) **do not commute, and cannot be made to.** *Project-then-square ≠ square-then-project.*

---

*Sources: `DECISIONS.decision-log.udon` (2026-07-12/13, the whole run) · `doc/theory/discretisation-and-information.md` · `msc/` (the eight spikes and audits of 2026-07-13) · `ASSUMPTIONS.md`.*
