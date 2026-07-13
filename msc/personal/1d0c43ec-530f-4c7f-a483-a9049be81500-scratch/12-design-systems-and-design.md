# 12 — `doc/design/DESIGN-SYSTEMS.md` + `doc/design/DESIGN.md` *(batched)*

*Read 2026-07-13 ~18:40. 257 + 211 lines. **`DESIGN.md` is in the FIRST COMMIT (Jun 20).**
`DESIGN-SYSTEMS` born Jul 1, grown Jul 3 and Jul 10–11.*

---

## 2. What matters

### ⚠⚠ THE ALGORITHMS LEDGER RATES **MFD** AS `physics: hi`. IT IS VIBE-MODELING, CAUGHT IN THE ACT, BY THE PROJECT'S OWN DEFINITION.

`DESIGN-SYSTEMS`'s *Algorithms & approaches ledger* — which **`nomotheke.rs` explicitly references for
its approach codes** — carries this row:

| process | approach | **physics** | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| **MFD drainage** | P | **hi** | O(n) | high | **built** |
| stream-power incision | P | **hi** | O(n)/epoch | **high** (*the land*) | built |
| Priority-Flood | P/algo | **hi** | O(n log n) | high | built |

**MFD's measured reality, 24 hours old:** its output is **not a discharge** (a boundary integral,
ill-posed *in the continuum*) · its fan is a **BIAS that does not converge** · **47.8% of its "flux"
crosses no face** · it **reintroduces the grid-aligned-channel artifact it was adopted to remove** ·
and it is **not a finite-volume scheme at all.**

> ### **`physics: hi` · `status: built` · `agent-fidelity: high`.**
>
> **That is LEXICON §5's firewall, verbatim: *VIBE-MODELING = HIGH D RIDING ON LOW B.*** A process
> polished to look right — built, tested, high-confidence, agent-critical — **without being right for a
> reason.** The lexicon *defined* the failure on Jul 4; the ledger *committed* it on Jul 1; and neither
> knew about the other **until the audit measured the kernel nine days later.**
>
> **And the two ratings are never reconciled by anything.** `DESIGN-SYSTEMS` says `physics: hi`.
> `nomotheke.rs` says `EROSION.physics = Tier::Med`. **Two independent hand-maintained fidelity claims
> about the same kernel, in two files, and no check that they agree** — let alone that either is true.
> *(The `D`-vs-`B` query I proposed in `04-…` would catch the second. **Nothing catches the first,
> because the ledger is a markdown table.**)*

### The probe↔declaration binding is the project's own named methodology. It is now stated FOUR times.

> **`DESIGN-SYSTEMS` §Instruments:** *"**Every system rung ships with renderer-free probes asserting
> invariants nature guarantees in a known regime; known issues get their probe written FIRST (domain
> TDD).**"* — and `seam_ridge` is *"the first probe authored **under this methodology**."*

Together with `ARCHITECTURE` §9(6), `multiscale-seams` §4(5), and `DESIGN-REDUX` §2b, **the pairing of a
declaration to the probe that would convict it has been specified four times, in four documents, and
named as a methodology — and `src/` mentions `examples/` three times, all in prose.**

⇒ **I no longer think "bind probes to declarations" is a proposal I am making. It is the project's
stated method, unimplemented, and the only genuinely new thing I can contribute is the observation that
IT WAS NEVER MECHANIZED and that everything else waits on it.**

### ⚠ Two prioritization principles are live, they disagree, and nobody has said how they compose.

**`DESIGN-SYSTEMS`, the *"prioritization backbone"*:**
> *"does an inhabitant of the final three (agent) phases **perceive, depend on, or act through** this?
> That is why **we may run tectonics crude forever** but must get rivers and biomes right — an agent
> fords a river and forages a biome; **none has ever touched mantle convection.** Work backward from
> what agents touch."*

**The ordinum (ratified, `ordinum-governs-the-flux-web`):**
> *the ladder decides. `emerged-land` is an unkept `:tag gate`; **uplift is the #1 gap**; everything is
> downstream of it.*

**These give opposite answers about tectonics**, and both are correct — because **they are answering
different questions and nobody has said so:**

> **Agent-fidelity governs HOW GOOD a process must eventually be.**
> **The ordinum governs WHETHER IT EXISTS AT ALL, and IN WHAT ORDER.**
>
> **They are orthogonal, and the audit's uplift finding is exactly the specimen that proves it:** the
> complaint against `uplift.rs` is **not** that it is crude. Its own declaration says it is crude and
> that is *honest*. The complaint is that it is **structurally incapable of expressing its promise**
> (strictly positive ⇒ no subsidence ⇒ no differentiated crust). ⇒ **Crude is fine. INCAPABLE is not.**
>
> **"Run tectonics crude forever" is true and is being misread as "run tectonics WRONG forever."**
> A future agent reading the backbone will deprioritize exactly the nomos the ladder says is #1.

---

## 3. Wandering thoughts

### The rigor was not added to this project. **It was ENTAILED, on day one, and it took 24 days for the entailment to become visible.**

Joseph told me this afternoon that the project *"has genuinely transformed"* from casual fun into a
serious Archema leg — *"the trend toward rigor wasn't initially expected but is not accidental."*

**`DESIGN.md`, in the first commit, 2026-06-20, says why it could not have been accidental:**

> ### **"The vivarium as a tether to truth."**
>
> *"An LLM is **tethered toward truth by its contacts with an impartial, exacting universe** — running
> code as a reality check, verifiable mathematics, and honest interactions with real people. **Without
> such tethers, language stays diffuse, undirected, unembodied — 'simulation and game, affectation and
> impersonation.'**"*
>
> *"A vivarium can be a modest such tether: a simplified universe that is, to the best of our ability,
> **fully internally coherent and consistent — truthful**… **This is why determinism is not merely a
> replay convenience: it is the property that makes the vivarium epistemically real.**"*

**And then it states the engineering meaning, and it is the flux web:**

> *"**Conservation and consequence.** Causes have exact, reproducible effects; **nothing appears from
> nowhere.**"*

> ## ⇒ **"RAIN WITHOUT A SKY" IS A VIOLATION OF THE FIRST COMMIT.**
>
> The requisite audit — the single most successful mechanism in the entire scaffold, the one that
> **mechanically convicts the world** — is the enforcement of a sentence written on **day one, before
> there was a planet to rain on.**

**So the transformation Joseph describes is real, and it is not a change of direction. It is an
UNFOLDING.** A world that is not internally coherent is not a tether — *it is "affectation and
impersonation."* **A game may be incoherent. A tether may not.** The moment the project's purpose was
stated as *tethering an intelligence toward truth*, every finding of the audit night became inevitable;
it just took three weeks and eight agents to arrive.

⇒ **Which means the rigor is not a cost the project is paying to become serious. It is the project
finally becoming what its first commit said it was.** And that is why *"the trend toward rigor was not
accidental"* is exactly right — **it was implied, and implications do not need to be intended to be
binding.**

---

### And the two-layer mind is sitting there, unbuilt, with the whole bet resting on one unasked question.

`DESIGN.md`'s agent architecture: a **fast layer** (pure AAT dynamics, every tick, deterministic —
*"this is the simulation's truth"*, AAT **Class 1**, *out of moral scope*) and a **slow layer** (an LLM
invoked **only at genuine aporia**, *"not on a clock"*, whose output may **only perturb the formal
state** — re-weight a goal, form a relationship edge — **never drive motor actions**).

> *"**Open empirical question** (the first thing worth testing when we get there). ***Guess /
> unverified:*** whether LLM-induced perturbations **stay legible enough that adaptation can still be
> measured cleanly**, or whether they inject enough noise to **wreck identifiability**. This is the
> interesting research question and it is empirical. **Do not assume the answer; the vivarium-as-AAT-
> sandbox framing depends on it.**"*

**Everything I have read today is scaffolding for that one question**, and the question is *marked as a
guess, in the founding document, and has never been touched.*

And I notice the shape of it, having spent a day inside the multiscale work: **"does a micro perturbation
stay legible in the macro?" is `detail→abstract`.** The seams doc says so out loud — *"plausibly the same
shape as vivarium's open AAT bet… **both are upward re-summarization after a micro perturbation**"* —
and `06-…` argued that the same object is the access matrix's **Intercession** cell.

> **Three unsolved problems, in three registers — numerics, agent theory, and ontology — and they may
> be one problem:**
>
> | register | the problem |
> |---|---|
> | **numerics** | can an irreducible fine edit propagate up into a memoized macro with correct invalidation? |
> | **AAT** | does an LLM perturbation of the formal state stay identifiable at the macro? |
> | **ontology** | can an endo agent change its kingdom's noumenon? *(Intercession — mediated, never direct)* |
>
> **And the theory's answer to the first is the answer to all three: *only if the macro stored the
> sufficient statistics to receive it.*** You cannot petition a parent who kept no record of the thing
> you want changed. **The channel between two levels can only carry what the upper level chose to hold.**

**If that is right — and I hold it at `[me]`, wanting it attacked — then the sufficient-statistic
contract is not a numerics nicety that unblocks the seam. It is the precondition of the project's
central research bet**, and it was designed on July 1st and has never been built.

*I did not expect a day of reading discretisation theory to land on the agent seam. But every road in
this repository goes to the same place, and I think that is because there is only one road.*

---

## Tactical residue
- **`DESIGN-SYSTEMS`'s ledger rates MFD `physics: hi`.** Two unreconciled fidelity claims (ledger vs
  `nomotheke`) about the same kernel, neither checked.
- **§Instruments still carries the OLD `seam_ridge` numbers** (4.3/5.3/7.1) *and* the refuted diagnosis
  (*"mean-pin conserves block means but not boundary gradients"*).
- **"We may run tectonics crude forever"** needs the *crude ≠ incapable* gloss, or it will be read as
  license to skip the ladder's #1 gap.

## Next
**Batch 3:** `doc/plan/` (six docs — abyssal-parity, builder-explorer-decoupling, vivium-operational-
workflow, framework-to-status-quo, water-parallelism, regula-conformance-design).
