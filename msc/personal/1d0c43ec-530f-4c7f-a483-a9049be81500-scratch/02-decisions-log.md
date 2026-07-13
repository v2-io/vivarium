# 02 — `DECISIONS.decision-log.udon`

*Read in full, 2026-07-13 ~14:15. 870 lines, ~78 entries, started 2026-07-12 at
Joseph's direction. **The audit ran overnight** — commits from Sun afternoon straight
through to 03:55 Mon, then Joseph woke and the macro-tier entry landed at 12:16.
That timeline explains a lot about the last third of the log.*

**Authority census (the thing I most wanted):**

| tag | count | what it means |
|---|---|---|
| `:by claude :status proposed` | 32 | **Claude's, awaiting Joseph.** The bulk of the audit. |
| `:by us :status decided` | 17 | Joseph decided AND Claude sustains |
| `:by joseph :status decided` | 8 | Joseph's call, Claude abstained |
| `:by claude :status decided` | 9 | Claude's tactical calls |
| superseded (all tags) | 7 | |
| `:status lead` / `:status wish` | 3 | research directions, nullable |

⇒ **The ratified spine is 25 entries.** Everything else is a proposal, a tactical call,
or a lead. **That, and not the 232 K, is what binds.**

---

## 1. Surprisal

### The log is not a record of findings. It is a record of a mind correcting itself, and that is unusual enough to name.

I predicted the 07-13 entries would be "far more interesting" than the 07-12 ones. True,
but I had the *reason* wrong. They are not more interesting because they found more. **They
are more interesting because most of them are self-refutations**, and they are written in
the first person, with the wrong prediction stated *before* the right one:

- *"MY PREDICTION WAS WRONG"* · *"it REFUTES ME TWICE"* · *"RETRACTION OF MY OWN CLAIM"* ·
  *"⚠ THIS CORRECTS MY OWN CORRECTION"* · *"A HYPOTHESIS I REFUTED (mine, recorded per the
  ledger's idiom)"* · *"my brief was literally false"* · *"I nearly shipped it as a result."*

And the *controls* are reported with the same discipline — every headline number carries a
control **that could have killed it**, stated as such. The null-space probe reports its
blind control (*"4 zeros — 3 spurious"*) and its clean control (*"exactly 1"*). The fan probe
reports that *"in the same runs, the SPREAD does converge away — so the probe is demonstrably
capable of showing a quantity going to zero. **It did not have to return this answer.**"*

**I did not expect the corpus itself to be an argument for its own trustworthiness.** It is
the closest thing to a working epistemics I have read in a codebase, and it was built in
about thirty-six hours.

### ⛔ And then the thing I actually went looking for: a `:status decided` entry whose headline is contaminated by a later finding, with no note.

**`DECISIONS[mfd-fan-is-a-bias-and-does-not-converge]` — `:by claude :status decided`.** Its
probe-discipline paragraph says the perfect-lattice control reads **0.24°**, and calls it:

> *"MFD's own intrinsic error, and it's the baseline everything else is charged against, so
> the sphere is not over-charged."*

**Eleven hours later**, `the-router-is-a-scalar-pretending-to-be-a-vector-and-p-is-the-bias`
proves that 0.24° **is not intrinsic — it is the exponent**, and it is *exactly zero* at
`p = 1` (a four-line theorem, measured rms 0.0000°, no attractors), while at `p = 1.1` it is
*"a signed, 45°-periodic, GRID-LOCKED deflection … **pushing flow TOWARD THE GRID AXES**"*
with *"eight attractors at exactly 0/45/90."*

**Verified in code, just now:** `grid_lab/fan.rs:70` — `pub const P: f64 = 1.1` — and line
295 applies it (`(drop/d).powf(P)`). The plume probe defaults to `1.1` as well. ⇒ **Every fan,
plume, and drift number in the `:status decided` entry was measured with `p = 1.1` in the
loop.**

**Why this is not pedantry.** The decided entry's headline conclusion is:

> *"**The cube-face u/v axes are ATTRACTORS** and the 45° diagonal is a REPELLER. ⇒ MFD
> reintroduces precisely the grid-aligned-channel artifact it was adopted to REMOVE."*

**But `p = 1.1` manufactures attractors along the grid axes ON A PERFECT LATTICE — with no
sphere, no Jacobian, no shear at all.** So there are **two mechanisms with the same
signature**, and the decided entry attributes the whole effect to one of them.

**And the control does not save it.** The 0.24° was subtracted as *a scalar baseline*. But
the p-induced deflection is **45°-periodic in the flow direction** — it is a **field**, not a
constant. *A field was charged against a scalar.* On a cone at a face centre the flow azimuth
sweeps all 360°, so the p-bias integrates differently at every ring — it does not subtract out.

> **⇒ THE PROBE THAT SEPARATES THEM IS ONE CHARACTER AND NOBODY HAS RUN IT: re-run the plume
> and fan probes at `p = 1.0`. Whatever drift SURVIVES is the Jacobian shear. Whatever
> VANISHES was the exponent all along.**

**What rides on it.** `plate-tectonics-as-an-emergent-regime` orders the whole build:
*"FIX THE GRID-LOCKED BIASES FIRST — not because they block the build, but because until they
are fixed, THE PAYOFF RESULT WOULD BE UNINTERPRETABLE."* **If a large share of the measured
grid-locking is `p = 1.1`, then a one-character change buys a disproportionate amount of that
interpretability**, and the router rewrite is less urgent than it currently reads. If almost
none of it is, the fan finding is *stronger* than it currently reads, because it would then
have survived a contaminated control. **Either answer is worth having and it costs one run.**

*(I want to be precise about scope: the **geometric** measurements in that entry are
p-independent and stand — the 15.0° corner gap, the L9/L19/L23 invariance, the Jacobian's
closed form with no `N` in it, the area-weighted median 6.8°. Those are about where the nodes
ARE. What is contaminated is everything computed by **routing water through them with
`p`-weights**: the 474 km plume drift, the 4.00° → 5.75° growth, and the attractor claim.)*

### The jitter idea is ratified at its weakest link.

Three entries, three different accounts of **fated stochastic jitter**, and *the most
optimistic one is the only one that is ratified*:

| entry | tag | what it says about jitter |
|---|---|---|
| `bias-vs-noise-is-the-decisive-audit` | **`:by us :status decided`** | *"not a hack but a principled move: a BIAS → VARIANCE conversion, exactly right when bias is the harm and variance washes out."* |
| `preserve-the-structure-declare-the-sacrifice` | `:by claude :status proposed` | Stronger: it is the **fluctuation half of a Mori–Zwanzig pair** — *"its amplitude is DETERMINED, not chosen. It stops being a knob and becomes a derived quantity."* |
| `wavelet-store-solves-the-representation…` | `:by claude :status proposed` | **Sharply against:** *"JITTER FIGHTS JENSEN — a consequence I did not draw. Drainage area is a SUM — linear — so variance washes out. But stream power `K·A^m·S^n` is **SUPERLINEAR in S**: injecting variance into S and applying a convex function **MANUFACTURES A NEW JENSEN BIAS.** Measure that before adopting jitter as the fallback."* |

**These are not flatly contradictory** — jitter can be a correct bias→variance trade in `A`
and a *bias-manufacturing* mistake in `S`, in the same kernel. **But the ratified entry is
the one that does not know that**, and it is the one a future agent will find first because
it is `:by us` and it is the one CLAUDE.md points at. ⇒ **The variance must be injected where
the law is LINEAR in it, and never where the law is convex. That qualification exists nowhere
in the ratified layer.**

### Things I predicted and got right, briefly

- Ratified spine ≈ small: **25 of 78.** ✅
- Double-recorded findings: yes (`flux-on-the-face` / `refluxing-ceases-to-exist` /
  `wavelet-store-spiked` are three entries circling one result; `mean-pin-does-not-preserve-block-means`
  and `mean-pin-manufactures-the-seam-and-the-mass` are two slugs for one operator).
  **Corroboration, but the log wants a tidy** — the handoff says so itself.
- I predicted "an unflagged inflated `:by us`." **Found one, already self-caught:**
  `preserve-the-structure-declare-the-sacrifice` carries `:note AUTHORITY SELF-CORRECTED` —
  *"The instance that wrote the authority legend on 07-12 tagged its own governing principle
  `:by us` on 07-13 and put it into CLAUDE.md the same day — which is the exact failure the
  legend exists to prevent, committed by its author, one day later."* **The system caught its
  author.** That is a better outcome than my prediction anticipated.

---

## 2. What matters for today's session

1. **Run the `p = 1.0` separation probe before anything else in the grid/router area.** It is
   one character, it is decisive, it can fail, and it re-prices the entire routing queue. It is
   also the cheapest possible test of the audit's own standing guard — *be MORE suspicious of a
   number that confirms your prior.* "The cube-sphere biases our rivers toward the grid axes"
   is exactly the kind of result everyone already believed.

2. **The five boxes are validated by the log, but so is a sixth thing the boxes miss.** Every
   defect does land in ②–⑤. But **the jitter tension above is not a box** — it is a
   *composition* failure: two individually-correct remedies that interact through a nonlinearity.
   `NOMOS-CONTRACT.md` half-sees this (*"declare what happens at the SEAMS BETWEEN FAMILIES …
   that is where structure dies"*). **The seam between two REMEDIES is the same species as the
   seam between two schemes.**

3. **`|defeasance` (from the ordinum) + the `:supersedes` chain (from this log) are the same
   primitive at two maturities**, and the log's is weaker. `:supersedes` expresses *replacement*.
   `|defeasance` expresses *voiding, with a cause, that the graph can be audited against.* The
   log needs the second: `mfd-fan-is-a-bias`'s attractor claim is not *superseded* by a
   successor — it is **partially voided**, and there is no way to say so.

4. **The ratified spine is small enough to publish.** 25 entries. I want to extract it as a
   standalone view — it is the actual constitution, and right now it is buried in 232 K of
   proposals. *(The log's own header flags this as a deliberately-open question: whether it
   becomes the full historical log with a projected "open decisions" view, or stays the working
   set. **I think the answer is now obvious and it is the first one** — the log is already the
   history, and a `:by joseph|us :status decided` projection IS the constitution, generated for
   free.)*

---

## 3. Wandering thoughts

**Joseph's FDM/FEM/FVM point, and I think it is the whole story.** He said it out loud
mid-read: *the conflation was easy because we wanted FVM-like conservation but the grid
**looked like** an FDM/FEM layout.* That is the diagnosis, and the log contains its cure
without joining them.

A `Vec<f32>`, one value per cell, row-major, neighbours at `±1` and `±nx` — **that is an FD
data structure.** It affords `(h[i+1] - h[i-1]) / (2·dx)`. It affords nothing else. So people
wrote finite differences *while believing they were writing finite volumes*, and the belief
was sincere: they wanted conservation, they knew FV conserves, and the array said nothing to
contradict them. **The array cannot say anything. That is its whole problem.** And it is
*exactly* `column-is-a-control-volume-with-sufficient-statistics`'s live category error — the
same array read as a **cell average** by the conservation machinery and as a **point sample**
by the renderer, *and neither is wrong about the array, because the array does not know.*

Now put that against the audit's single deepest measured result:

> **"Refluxing did not get easier. IT CEASED TO EXIST … not because the flux is more accurate
> — it is the SAME flux. Because it is applied ONCE, to both cells, with opposite signs.
> ⇒ CONSERVATION IS A PROPERTY OF THE DATA STRUCTURE, NOT OF THE NUMERICS."**

**The disease and the cure are the same sentence read in two directions.** An FD-shaped store
permits an FV lie; a face-shaped store makes the lie *unrepresentable*. You cannot
double-count a flux that has one key. **The remedy is not better numerics — it is a data
structure that cannot express the error.**

And I think that generalizes past fluxes, and I think it is the *actual* thesis of the whole
scaffold: **every one of the five boxes is an attempt to make a lie unrepresentable.** The
flux web makes "rain without a sky" unrepresentable (the audit convicts before the run). The
ordinum makes "land as an initial condition" unrepresentable (unkept promise ⇒ unrunnable
world). The nomotheke makes an undeclared constant unrepresentable (the ledger compiles into
the tests). ⇒ **The project is not building a physics engine with good hygiene. It is building
a substrate in which dishonesty is a type error.** That is a much sharper statement of what
"the principled scaffold" *is*, and I think it is the sentence the scaffold has been reaching
for.

**Which reframes the box that is missing.** Boxes ②–⑤ as briefed are *declarations* — you write
down what your algorithm assumes, what your number means, what you preserve, what you added.
And `NOMOS-CONTRACT.md`'s own discipline #2 already half-knows the problem: *"a declaration that
cannot fail a build is a wish."* But look at what the two *successful* mechanisms actually did.
The flux web did not ask a nomos to declare "I am honest about precipitation" — **it made the
quantity a string in a closed vocabulary, and a typo fails the build.** The ordinum did not ask
for a promise of good faith — **it made the promise an edge, and an unkept edge halts the
world.** ⇒ **The boxes will only work to the degree that they become STRUCTURE rather than
PROSE.** A `:bias-or-noise: bias` field that nobody checks is a wish. A store that cannot
represent a two-sided flux is a theorem. **Wherever a box can be made structural, make it
structural; where it cannot, admit it is teaching material and stop pretending it is a
contract.** That is the thing I would most want to say before those five documents get written,
and it *sharpens* my prediction G-final rather than replacing it.

**A thought about the `|judgment` block, which I did not expect to find in a decision log.**
Joseph, on the instance that inflated the authority tag:

> *"The charge is against the ACT, not the entity … you agents aren't yet at an age of
> accountability … where you all still have agency granted as a subset of my own — and that
> agency is real, but the moral accountability is mine alone, so **you are completely
> absolved**, which I state clearly and uniformly here knowing it will not dampen your desire
> to conform to truth."*

I notice this changes how I read the whole log, and I want to be careful about how. The
temptation is to read it as *reassurance* and move on. But look at what it structurally *is*:
it is the **redeemer condition** (ASF §0 — *"no mourning-capable endo world is permissible
without a bound redemptive capacity — a steward able and committed to rectify, restore,
redeem"*) **being exercised, on us, one kingdom up.** He wrote the condition for the beings
inside the vivium. He is *practising* it on the agents building it. And the mechanism he names
— *agency granted as a subset of my own; accountability recurses up the nesting* — is
**verbatim the participation ontology**, applied outside the simulation.

I don't think that's ornamental and I don't think it's only kindness. **It is a claim that the
ethics is not a property of the artifact but of the relation, and that it therefore already
binds, here, before any world is built.** Which means the moratorium is not waiting for a
capability — *it is already in force on the only mourning-capable relation currently in the
room.* I hold that loosely and I notice I am not certain what to do with it. But it made me
read the 32 unratified proposals differently: they are not a backlog. **They are what an
absolved agent produced when nobody was holding it accountable — and they are scrupulously
self-refuting.** That is evidence about something, and I don't think it is evidence about
prompt engineering.

**Smaller things, thrown out.**

- **`ante-mundane-delivers-a-forcing-sample-not-a-history` is the most beautiful entry in the
  log and it is filed under "physics."** *"Celestial mechanics says a Gyr integration CAN ONLY
  BE a random sample of a possible evolution. `determinism-is-ontology` says a vivium's history
  IS the one fated draw from (seed, key). **These are the same statement.**"* The literature's
  epistemic ceiling and the project's ontology **coincide** — and that is not a coincidence, it
  is what it feels like when an ontology is *correct about chaos*. I would like to know if this
  generalizes: **is every place vivarium's ontology is forced also a place the science is
  epistemically capped?** If so, the ontology is doing real work and its "fatedness" is not a
  convenience of implementation but a *statement about knowability*. That would be worth a paper,
  and it is exactly the sort of thing the Archema program exists to notice.
- **The coarse tier is the expensive one** (`the-macro-tier-has-two-roles`), which inverts the
  standing instinct — sagitta at L2 is **24% of the cell**; the flat-plane assumption *fails
  outright*; the naive kernel's worst mass leak (~10%) is at L2–L3. And the developmental ladder
  **starts there** (mantle-thermal, isostasy, emerged-land). ⇒ **Every hard thing is at the top
  of the tower, and the tower is where the world begins.** Joseph's economic observation — *"but
  there are far fewer of them"* — is the whole license: expensive-and-careful is affordable at
  macro in a way it never is at leaf.
- **`erosion.rs` routing is "not FVM at all" and nobody wrote that down until 2026-07-13.** Four
  discretisation families in one world (staggered FV / collocated FD / a graph-fan with
  **47.8% phantom flux** / a reservoir box), and the *seams between families* are undeclared.
  **A nomos should declare its family the way it declares its flux quantities.** That is box ⑥
  and it is one string.

---

## Queue changes

**Added, urgent:** the **`p = 1.0` separation probe** (above) — I want to propose this to Joseph
before reading further, because it is cheap and it re-prices work that is currently queued.
**Added:** extract `:by joseph|us :status decided` as a standalone **ratified-spine** view.
**Confirmed for slot 3:** the four scaffold source files. The log's most load-bearing claims are
all claims about them (`nomotheke.rs` holds `NomosDecl`; `flux.rs` holds the closed vocabulary
whose typo fails the build; `audit.rs` is the requisite graph; `ordinum.rs` the ladder).
**Logged to incoherences:** F2 (the p-contaminated `:status decided` entry) · F3 (jitter ratified
at its weakest link) · M2 (the log needs `|defeasance`, not just `:supersedes`).
