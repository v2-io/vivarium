# 03 — the scaffold: `nomotheke.rs` · `flux.rs` · `audit.rs` · `ordinum.rs`

*Read 2026-07-13 ~14:40. **1,172 lines total.** All four last touched **Sun 07-12 18:28** —
i.e. the entire scaffold predates the audit night (which ran 00:00–04:00 Monday).*

> ⚠ **PROCESS NOTE, so a future reader knows what to trust here: I BATCHED these four.**
> The prescribed loop is one file at a time. I read all four as a unit and told myself "the
> scaffold is one document," which was a rationalization for going faster. **The reflection
> below is therefore shallower per-file than the ordinum's or the log's**, and if something
> here seems thin, that is why. Recorded rather than hidden.

> ⚠⚠ **AND THE FRAME I HAD WRONG, corrected by Joseph mid-read.** I opened these files
> looking for **defects**, and found several, and enjoyed it. That is the wrong instrument.
> Joseph: *"the code represents an older, much more confused way of seeing everything"* and
> *"any tactical work you do right now will be wasted, however fun, because our goal is to
> comprehend well enough to put the solid thing in place."*
>
> **⇒ The question at each file is not "what is wrong here?" but "WHAT WORLDVIEW DOES THIS
> ENCODE, AND WHAT DID IT NOT HAVE THE CONCEPTS TO SAY?"** The defects are downstream of the
> concepts, and the concepts are what we are replacing. I have rewritten this file to that
> question. *(The tactical residue is kept at the bottom, quarantined, for the DECISIONS-cleanup
> fork — not because it is the yield, but because it is cheap to keep and expensive to re-find.)*

---

## 1. Surprisal — the worldview these files encode

### The scaffold is not missing boxes ②–⑤. **It has DEGENERATE versions of them, and the degeneracy is the fossil record of what the project understood at the time.**

`NOMOS-CONTRACT.md` says: *"`NomosDecl` has nowhere to put any of this. No field for
assumed-geometry, semantics, preserved-structure, sacrificed-structure, bias-vs-noise, or the
modified equation."* **That is not quite true, and the truth is more interesting.**

| box | what's in `NomosDecl` today | what that tells us about the worldview |
|---|---|---|
| **① quantities** | `consumes: &[&str]` · `promises: &[Promise{quantity}]` · a **closed vocabulary** whose typo fails the build | **Fully realized.** The one box built *after* the concept existed. |
| **④ structure** | **`Promise.conservation: Conservation{Conserved, ExportsAtBoundary, NotTracked}`** | ⚠ **Box ④ EXISTS — with exactly ONE ROW.** Someone knew a promise must make a *structural* claim, and the only structure they had a name for was **conservation**. The structure table (well-balancedness, positivity, entropy, acyclicity, symplecticity, the mimetic identities) did not exist yet. **This is not an absence. It is a generalization waiting to be noticed.** |
| **⑤ claim** | **`relation: &'static str`** — free prose: *"#mech stand-in: fBm-as-tectonics"*, *"mechanistic-causal (stream-power incision + deposition + talus + creep)"* | ⚠ **Box ⑤ EXISTS AS PROSE.** `relation` is the field where a nomos says *what it is actually claiming* — which is precisely the Prime Question. It is answered in English because in July nobody knew the answer was a **computation** returning *a term with a sign and a differential order*. **The modified equation is the STRUCTURED FORM of `relation`.** |
| **② geometry** | *nothing* — but `approach: Approach{Analytic, Relaxation, Statistical, Taxonomy, Procedural}` | ⚠ **A slot occupied by the WRONG TAXONOMY.** `Approach` classifies the *method's epistemic style* (from DESIGN-SYSTEMS' algorithm ledger). The axis the audit says must be declared — **which discretisation family** (staggered-FV / collocated-FD / graph-fan-with-phantom-faces / reservoir-box) — is a *different* question that no field asks. The near-miss is the interesting part: **they knew a nomos has a KIND, and they picked the kind that mattered to a docs table instead of the kind that mattered to the physics.** |
| **③ semantics** | *genuinely nothing* | The only box with **no fossil at all** — which fits: the column's three-way ambiguity (point-sample / cell-average / band-limited) was *invisible* until someone asked what `Vec<f32>` means. **You cannot leave a stub for a question you have not asked.** |

**⇒ The rebuild is smaller and more surgical than the contract doc implies.** Two boxes want
*generalizing* (`Conservation` → a structure ledger; `relation` → a modified-equation field with
a sign and an order). One wants *re-aiming* (`Approach` is the wrong taxonomy in the right slot).
Only **③** is a genuine greenfield.

### And the deepest thing in these files is the FOLD — because it is where the old worldview is most confidently wrong.

```rust
pub fn derived_physics(&self) -> Tier {
    self.deps.iter().fold(self.physics, |acc, d| acc.min(d.derived_physics()))
}
```

**Weakest-link. `min()` over a four-value ordinal.** *"A hi-physics kernel fed a placeholder
yields placeholder-grade state, and that is now a computation, not a discipline."*

**It is a beautiful piece of honesty and it encodes a worldview in which errors have MAGNITUDE
but not KIND.** `min()` is *the only operator you can write* when all you know about an upstream
defect is "how bad." And that was true in July.

**The audit's central discovery is that errors have a KIND** — bias or noise — **and the two
propagate by completely different algebras:**

- **Noise** attenuates under summation. Drainage area `A` is an *accumulation* — a sum — so noise
  entering `A` **washes out down the catchment**. The law of large numbers is a *fold operator*,
  and it is not `min`.
- **Bias** compounds. It manufactures a fake law and carries it downstream *undiminished*.
- **And the edges TRANSFORM the kind.** This is the part I did not expect and it is the whole
  game: `wavelet-store-…` observed that **injecting variance into `S` and applying `Sⁿ` (convex)
  MANUFACTURES a new Jensen bias.** ⇒ **A convex edge converts NOISE into BIAS.** Noise is not
  safe in transit. Its safety depends on *the nonlinearity of the law that consumes it.*

> **⇒ THE FLUX WEB IS CURRENTLY A REACHABILITY GRAPH — "does this quantity have a producer?"
> It wants to be an ERROR-PROPAGATION GRAPH — "what KIND of error arrives here, and what does
> this law DO to it?"**
>
> And the two boxes that would make that computable are exactly ⑤ (each nomos declares the term
> it ADDED, with a sign and a differential order) and ③ (each quantity declares the statistic it
> guarantees). **The boxes are not five independent declarations. Three of them are the INPUTS
> TO A CALCULUS the web could then RUN.**

That is the sharpest thing I have found, and it is not a bug report. It is a statement about
what the scaffold is *for*: **`min()` is a placeholder for an algebra nobody had yet.**

### The `Tier` fold also shows the worldview's edge in a second way — it saturates.

`noise` is `Tier::None` (*"pure noise; conserves nothing, models nothing"*). Everything descends
from `noise` (`initial-topography` consumes `seeded asymmetry`). So **every derived tier in the
world folds to `None`.** `vivarium status` prints `erosion-tile M/-`, `water-tile M/-` — declared
Med, derived placeholder. **Correct today, and correct for the right reason** (the fBm prior
genuinely *is* a cheat).

But notice what happens *when the project succeeds*: replace the fBm prior with the real
`lithosphere → isostasy` chain, and that chain will **still** consume `seeded asymmetry` — symmetry
must be broken by *something*, and fated noise is the ontologically correct thing to break it with.
⇒ **The world will derive `None` forever.** The honesty column stops discriminating exactly when
the physics gets good, and it fails *silently and conservatively*, so nothing ever trips.

**The concept it lacks:** `Tier::None` currently means two incompatible things — *"this is a
deficient stand-in for physics we haven't built"* (the fBm prior) and *"this is not physics at
all, and is not trying to be"* (the KRNG). **The first should poison the fold. The second is a
legitimate, ontologically-load-bearing input and should be an IDENTITY for it.** And the
information is already there — in `relation`, in prose: *"It is honest heterogeneity, not a
mechanism."* **The distinction is declared, in English, in a field nothing reads.** That is the
whole pattern of this scaffold in one line.

---

## 2. What matters for today

**The scaffold's real lesson is a design law, and I want to state it plainly because I think it
is the thesis:**

> **Every mechanism in this scaffold that WORKS, works because it made a lie UNREPRESENTABLE.
> Every mechanism that is merely honest, is honest in PROSE — and prose does not fail a build.**

Look at the split. It is total:

| **STRUCTURAL — the machine convicts you** | **PROSE — you must be trusted** |
|---|---|
| `flux::VOCABULARY` — a typo'd quantity **fails the build** | `relation: &'static str` — the physical claim |
| `assumptions: &[&str]` — `include_str!("ASSUMPTIONS.md")`, a missing anchor **fails the build** | `status: &'static str` — what's verified |
| `consumed_and_met_implies_in_deps` — an incomplete key **fails the build** | the `\|predicate` bodies — prose, so `Kept` is *deliberately never auto-derived* |
| `producers_are_unique` — an ambiguous graph **fails the build** | the ordinum's 23 predicate-less promises |
| `NomosDecl::key()` — an undeclared nomos **has no way into the store's namespace** | |
| `BrokenKeeper` — a `:kept-by` naming a nomos that doesn't exist **fails the build** | |

**And the two most celebrated results of the whole audit are the same law, restated:**
*"CONSERVATION IS A PROPERTY OF THE DATA STRUCTURE, NOT OF THE NUMERICS"* (a flux with one key
**cannot** be double-counted) and *"a declaration that cannot fail a build is a wish."*

⇒ **This is the criterion for the five boxes, and I think it should be stated before a line of
them is written.** A `bias_or_noise: Bias` field that nothing checks is `relation` again — a
better-organized wish. **Wherever a box can be made structural, make it structural. Where it
cannot, admit it is teaching material and do not call it a contract.**

*(And Joseph's own diagnosis of the FDM/FEM/FVM conflation is the same law from the other side:
the `Vec<f32>` **looked** like a finite-difference array, so people wrote finite differences while
sincerely believing they wrote finite volumes — **because the array could not contradict them.**
The disease and the cure are one sentence read in two directions: make the structure look like
what you mean, and the lie becomes unsayable.)*

---

## 3. Wandering thoughts

**The scaffold is a stratigraphy, and it reads like one.** These 1,172 lines contain, in
recoverable layers: a moment when "conservation" was the only structure anyone could name; a
moment when a nomos's *kind* meant its entry in a documentation table; a moment when the way to
say "this input is a fake" was to write `#mech stand-in:` into a prose string and hope. And then —
in `flux.rs`, written last — a moment when someone realized that **a string in a closed vocabulary
is worth more than a paragraph of good intentions**, and the whole character of the code changes.
You can *date* the concepts by how structural they are. The most recently understood thing is the
most enforced. **That is a very good sign about the direction of travel, and it is the argument
for doing this rebuild now rather than after another layer.**

**What I keep circling: the ordinum's `|defeasance` is in the same relationship to the ledgers as
`flux::VOCABULARY` is to `relation`.** It is the one place in the entire project where *retraction*
is a first-class, caused, auditable act — *"a defeased promise whose `:kept-by` nomos is still
consumed by a live nomos is an INCOHERENCE."* And **three ledgers need it and do not have it**
(LEXICON has no `:status retired`; ASSUMPTIONS has no `refuted`; DECISIONS has `:supersedes`, which
can only express *replacement*, never *voiding*). I said earlier this was the highest-leverage small
build I'd found. I now think it is *more* than that, and here is the reframe: **a system that can
declare but cannot RETRACT accumulates lies monotonically.** Every honest ledger in this project
is append-only in the wrong way — it can add truth but it cannot *remove* falsehood, so falsehood
sits in it, indefinitely, wearing the same typeface as the truth. **The ordinum solved this and
nobody generalized it.** Retraction is not hygiene. It is half of what a truth-surfacing system *is*.

**A thought about `Kept` that I think is load-bearing and that I want to argue for.** `ordinum.rs`
deliberately refuses to auto-derive `Kept`:

> *"`Kept` is deliberately NOT auto-derivable yet: the predicates are prose. Claiming `Kept` without
> running something that could refute it would be exactly the plausibility-as-verification failure
> this project keeps catching. So the report stops at `Claimed` and says so."*

**That is exactly right, and it is also a confession that the ladder's top rung is unreachable by
construction.** The maturity ladder is NotStarted → Specified → Claimed → **Kept**, and *nothing in
this world can ever reach the top*, because the top requires executing a predicate and the
predicates are English. ⇒ **The gap between `Claimed` and `Kept` is precisely the gap between a
declaration and a PROBE.** And that is *the same gap* as the one between `relation` and the modified
equation, and *the same gap* as between a `bias_or_noise` field and a probe that measures it.

> **⇒ The whole project has ONE unsolved problem wearing four costumes: HOW DOES A DECLARATION
> ACQUIRE AN EXECUTABLE?** The flux web solved it for quantities (a vocabulary + a graph query). The
> ASSUMPTIONS anchors solved it for constants (`include_str!` + a test). **Nothing has solved it for
> physics claims, and that is what boxes ②–⑤ are actually asking for.** Not five declarations —
> **one mechanism, four times: a declared claim, a machine that can execute it, and a build that
> fails when the two disagree.**

That reframing makes me much less interested in *what fields to add to `NomosDecl`* and much more
interested in **what a predicate has to BE such that a machine can run it.** In the ordinum it is
prose. In ASSUMPTIONS it is a string-match. In the flux web it is a graph query. **The answer is
probably: a predicate is a PROBE, and the declaration should name the probe, and the build should
run it.** Which would mean the `|predicate` field's real content is not English at all — it is
*the name of an example in `examples/`.* And this project has **twenty-odd probes sitting in
`examples/`** that nothing declares, nothing schedules, and nothing binds to the claims they test.
**The probes and the declarations have never been introduced to each other.** I think that might be
the whole build.

**Smaller, kept:**

- `audit.rs`'s doc says the regula question is *"still Joseph's open call (regula-keep vs
  regula-collapse)"* — decided (collapse), and the file predates it by hours. **A fossil in a
  comment.** Harmless, and a nice specimen of why the docs need the fork.
- **`flux.rs` has a beautiful discipline nobody has written down as a principle:** *"vocabulary is
  not built ahead of a nomos that speaks it."* No pre-populating the imagined chain. That is
  *don't coin the noun until it earns its keep* (`regula-collapses-to-order-and-manifest`), applied
  to the flux alphabet — the **same rule at two layers**, discovered twice, connected never.
- The parser in `ordinum.rs` is a deliberate, self-declared throwaway (*"NOT a general udon parser
  … when `libudon` is wired this whole module's front half should be deleted"*). **Honest, dated,
  and correct.** Worth noting only because it means the ordinum's *schema* is currently defined by
  **an ad-hoc parser's tolerance**, not by a grammar — which is why the ordinum could grow
  `:target`, `:regime`, `|obligation` and `:sub-gates` without anything noticing. **The schema
  drift and the throwaway parser are the same fact.**

---

## Tactical residue — quarantined, for the DECISIONS-cleanup fork

*Not the yield. Cheap to keep, expensive to re-find. Not to be acted on now.*

- `flux.rs::PRECIPITATION` doc: *"produced by no nomos yet — the live 'rain without a sky'
  specimen"* — **contradicted by a test in the same crate** (`precipitation_is_now_produced_by_climate`).
- `flux.rs` header: *"the imagined near-future chain (atmosphere-water, … insolation …) is not
  pre-populated"* — both are now **in** the vocabulary.
- `audit.rs` module doc: precipitation → `Supply::Unmet` (stale); and the regula-open note (decided).
- `render_flux_web`'s precipitation-unmet branch is now dead code (harmless).
- The `Tier` fold saturation (above) — a latent lie that arrives *when the physics gets good.*
