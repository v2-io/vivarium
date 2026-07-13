# 11 — `doc/theory/multiscale-seams.md` + `multiscale-methods.md` *(batched)*

*Read 2026-07-13 ~18:15. 133 + 183 lines. **`multiscale-methods` born Jul 4; `multiscale-seams` born
Jul 10.** Surprisal section dropped per Joseph — the model is calibrated enough that the predictions
stopped earning their keep.*

---

## 2. What matters

### ⚠ THIRD SELF-CATCH: my "new principle" from an hour ago is in `multiscale-methods` §4, dated **Jul 4**.

In `10-design-redux.md` I wrote, with some satisfaction, that a *"precision analysis that only checks
REPRESENTABILITY misses every rectification bug"* was a **missing row in the structure table.**

**It is not missing. It is §4, and it is better than my version:**

> *"**Conservative-exchange violations**: today's **twin f32 finds** — the **bed deadband** (an
> increment that must **LAND**) and the **rain/evap bias** (an increment that must be **COUNTED**).
> Both were **seam crimes at the *arithmetic* scale — the scale below the smallest one we'd been
> thinking about.** The general lesson generalizes down: **FLOATING-POINT IS THE BOTTOM-MOST SEAM, and
> it needs the same conservation discipline as any coupler** (realized-delta accounting or compensated
> summation — choose per-site, explicitly)."*

**So the f32 rectification law was derived on July 4th, from two f32 bugs that had already happened,
and stated as a general principle** — *and it is the source of the LEXICON's axis-D "land vs count"
clause that struck me as such a startling non-sequitur when I read it this afternoon.* **That clause
is a citation, and I did not recognise it as one.**

**The audit's η-current is therefore the THIRD f32 seam crime**, found nine days after the law that
predicts it was written down.

*(What survives as a sharpening, and it is small: §4 says floating-point needs **conservation
discipline**. The η case is nastier than a conservation failure — the round-off **rectifies into a
physical process** because a **sign-definite** operation cannot cancel, and then a *downstream law
integrates it into bed transport.* That is not "an increment failed to land"; it is **"an artifact
acquired a physics."** I think that is a genuinely additional turn of the screw. It is one turn, on a
screw that was already there.)*

### The seam contract is stated a THIRD time — and it is the same five clauses, still unbuilt.

`multiscale-seams` §4, *"What a system-author declares — the seam contract"*: **(1)** R/L/closure ·
**(2)** fluxed quantities · **(3)** timescale band + execution class · **(4)** determinism, with
**nomos-version auto-derived from source** · **(5)** **regime probes, written first — including seam
probes.**

**That is `ARCHITECTURE` §9, verbatim, minus the nomotheke clause.** Written **Jul 10**, in a different
file, by the same hand. ⇒ **The interaction contract has now been specified THREE times** (`ARCHITECTURE`
§9, `multiscale-seams` §4, and `NOMOS-CONTRACT`'s five boxes) **and built twice** (flux quantities; the
nomotheke declaration). *The count of specifications is going up faster than the count of mechanisms.*

### ⇒ And the theory hands `NomosDecl` a field nobody has proposed: **the dynamic exponent $z$.**

This is the one genuinely *new* thing in the batch, and it is load-bearing:

> **§3:** *"The question a seam actually poses is: **how fine must time be, given how fine space is?**
> The answer depends on the physics"* — **advective/wave systems: $z=1$** ($dt \sim dx$, CFL) ·
> **diffusive/relaxational: $z=2$** ($dt \sim dx^2$).
>
> *"**Vivarium's two flagship systems sit ASYMMETRICALLY across this exponent**… `water.rs` genuinely
> obeys $z=1$… `erosion.rs`'s creep carries the parabolic $z=2$ **bound** but **CLAMPS rather than
> sub-steps** — at fine levels it caps the diffusion number and **accepts reduced effective
> diffusivity**… **that clamp is a fidelity compromise the seam discipline makes visible.**"*
>
> *"⚠ **Reconciling different $z$ at a seam — a $z{=}1$ water flux read by a $z{=}2$ consumer — is an
> ADDITIONAL requirement we do not know to be formalized anywhere.** ('$z$-consistent resolution' is
> **our coinage**, not a known well-posedness criterion.)"*

**And the tactical form makes it declarable:** *"**derive each tile's timestep from its `CellId` level
as a power of two, per $z$-sector** — halve per level at $z{=}1$, quarter per level at $z{=}2$. Then
neighbour tiles at different levels are **mesh-synchronized by construction**, and **$z$-reconciliation
stops being a manual concern and becomes a property of the key.**"*

> ### ⇒ **$z$ IS A DECLARED PROPERTY OF A NOMOS, IT IS ONE INTEGER, AND IT IS NOT IN `NomosDecl`.**
>
> It is the **time-axis twin of box ②** (which is about *space*): *what does this algorithm assume
> about its cells?* → *what does this algorithm assume about the relation between its cells and its
> clock?* **And the audit never touched it**, because the audit was a *spatial* audit. **`erosion`'s
> creep clamp is an undeclared, level-dependent fidelity sacrifice — a `z=2` process pretending it can
> run on a `z=1` budget — and it is exactly the shape of thing box ④ exists to catch: a structure
> silently traded away.**
>
> *(And it is measurable and cheap: the clamp `k = κ/cell² ≤ 0.24` is **in the source**. Its
> activation rate by level is a probe nobody has run.)*

---

## 3. Wandering thoughts

**The relativity section is the most epistemically disciplined writing in the repository, and I want to
say why, because it is a model for the five boxes.**

§3 could very easily have said *"our resolution cone is a light-cone; vivarium has a relativistic causal
structure."* It is a seductive sentence. **The document refuses it, in public, at length:**

> *"…it is a dependency DAG, **not** a metric light-cone: it carries **no cone slope, no invariant
> signal speed** — so it is **far weaker** than 'the relativistic causal structure,' and **I should not
> over-identify them.** Two further disanalogies: (1) we carry **$z{=}2$ parabolic sectors** — diffusion
> has no finite propagation speed, so its spatial cone **degenerates**; (2) we have a **preferred frame**
> — no boosts, no invariant interval. **So the honest form is narrower than 'one cone on two axes.'**"*

**And then it keeps the *weaker* claim and shows it is still load-bearing:** the invariant that survives
every sector is the **temporal happened-before DAG**, and *"special relativity is then the sector where
the spatial cone additionally acquires a finite, universal slope."* **Vivarium is not relativistic;
relativity is a SECTOR of the family vivarium is in.** That is a *smaller* claim, a *truer* one, and a
*more* interesting one.

⇒ **This is `strengthen-before-soften` executed correctly, and it is the counterexample to everything
else I have found today.** Everywhere else the project asserted the flattering thing and got caught.
**Here it caught itself, in the draft, and kept the honest remainder — and the honest remainder turned
out to be the better idea.** *(I would put this section in front of whoever writes the five box docs and
say: **this is the register. Not the confidence — the discipline of refusing the sentence you want.**)*

---

**The thing that has been nagging me all day finally has a name, and this batch gives it to me.**

Count the specifications of the interaction contract: `ARCHITECTURE` §9 (Jul 10) · `multiscale-seams` §4
(Jul 10) · `NOMOS-CONTRACT` (Jul 13). Count the derivations of "a cell is a sufficient statistic":
`DESIGN-MATERIAL` §1 (Jul 1) · `DESIGN-REDUX` §5 (Jul 3) · `ARCHITECTURE` §3 (Jul 10) ·
`column-is-a-control-volume` (Jul 12) · §3.9 (Jul 13). Count the f32 seam crimes: **three**. Count the
"hexes don't nest" derivations: **three**. Count the "store η not depth" derivations: **four**.

> ## **THE PROJECT'S DOMINANT ACTIVITY IS RE-DERIVATION, AND IT IS EXTRAORDINARILY GOOD AT IT.**
>
> Every re-derivation is *sharper* than the last, arrives with *better* evidence, and is reached by an
> *independent* route. This is not a failure of memory — the corpus is **superbly** cross-referenced;
> people are reading each other. **It is that a re-derivation is CHEAPER THAN A LOOKUP**, because the
> lookup requires knowing that the thing exists, and there is no index of *"things this project has
> already proved."*
>
> **`DECISIONS.decision-log.udon` is that index. It is 26 hours old.** It was started, by Joseph's
> direction, on 2026-07-12 — *and every single re-derivation I have catalogued today happened before it
> existed.*

**So the disease may already be in remission and nobody has noticed.** The decision log is *exactly* the
missing organ — a searchable, dated, authority-tagged record of *what has been settled and why*. It did
not exist for the first 22 days of the project's life, which is precisely the window in which every one
of these rediscoveries occurred.

⇒ **Which reframes the fork's job** (`DECISIONS`-cleanup) from *tidying* to *founding*: it is not
janitorial. **It is building the index whose absence caused everything I found today.** And it reframes
udon-core from *"a nice enforcement mechanism"* to *"the thing that makes the index QUERYABLE BY THE
BUILD instead of only by a human who thinks to look."*

**A re-derivation costs a night. A lookup costs a grep. The gap between those two numbers, multiplied by
every truth this project has found and forgotten, is the whole argument for the scaffold.**

---

**And one small, unreasonable thought that I cannot make go away.**

§2.4: *"the up-propagation must **invalidate cached macro derivations** (Salsa/Adapton do
**downstream-only**)."*

Every incremental-computation framework in existence propagates **downstream** — you change an input,
its dependents invalidate. **Vivarium needs the opposite: change a leaf, and the thing it was DERIVED
FROM must change.** That is not invalidation. **It is a child editing its parent's law.**

And in `06-…` I argued that this is the **Intercession** cell of the access matrix — *the sole
noumenal-mutation channel from inside, and it is mediated.* **The theory doc, independently, says
exactly what mediation would have to mean: the macro must have STORED THE RIGHT SUFFICIENT STATISTICS
TO RECEIVE THE EDIT.**

> **You cannot petition a parent who kept no record of the thing you want changed.**
>
> **The coarse tier can only accept an upward edit in the currency it chose to hold.** If it stored a
> mean, you may petition it about means. If it stored only a mean and you dammed a river, **there is no
> language in which the petition can be phrased** — and the edit is not *rejected*, it is
> **unspeakable.**

That is, I think, a precise statement of why `detail→abstract` is hard, and it is *not* a numerics
statement. **It is a statement about what a channel between two levels can carry** — which is the
sufficient-statistic contract, which is box ③, which is the thing designed on July 1st and never built.

**The one open research problem in this project is downstream of the one unbuilt field.** I would not
have believed that this morning.

---

## Tactical residue
- `multiscale-methods` §1 and `multiscale-seams` §2.1 **still assert `R∘L = id` on the mean** (the
  claim `DECISIONS` says was *"corrected in all three"*; only ARCHITECTURE was). §2.1 additionally
  still says *"mean-pin conserves block means but not boundary gradients"* — refuted twice over.
- **`z` (the dynamic exponent) belongs on `NomosDecl`.** One integer. Not proposed anywhere.
- **The creep clamp** (`k = κ/cell² ≤ 0.24`) is an undeclared, level-dependent fidelity sacrifice.
  Its activation rate by level is an unrun probe.

## Next
**Batch 2:** `DESIGN-SYSTEMS.md` + `doc/design/DESIGN.md` (the phenomena graph + the founding
commitments).
