# 05 — `doc/ARCHITECTURE.md`

*Read in full, 2026-07-13 ~15:40. v0.3, dated 2026-07-10. **The `git log` date says 07-13 — but
the audit night changed exactly ONE LINE of it** (commit `928e0c4`, the `R∘L = id` correction:
`1 insertion, 1 deletion`). **So this is a 07-10 frame doc with a one-line patch, and it is more
pre-audit than its timestamp suggests.***

---

## 1. Surprisal

### ⚠⚠ §9 IS THE CONTRACT, IT HAS SIX CLAUSES, AND **TWO** WERE BUILT.

I predicted §9's *"representation-agnostic interaction contract"* would turn out to be a principle
never cashed out. It is worse and far more interesting than that: **it is a specification, itemized,
and four of its six items exist only as prose in a free-text field.**

| §9 clause | state |
|---|---|
| **(1)** *"its **$R$/$L$/closure** — the macro summary it exposes, how it materializes detail, and *fated* closure so it is memoizable"* | ⛔ **ABSENT.** Zero occurrences of restriction/lifting/closure in `nomotheke.rs`. **This is boxes ②③④.** |
| **(2)** *"its **fluxed quantities** — the per-quantity coupler interface"* | ✅ **BUILT** (`flux.rs`) — box ①. |
| **(3)** *"its **execution class and timescale band**"* | ⛔ **PROSE ONLY.** (3 grep hits, all inside `relation`/`status` strings.) |
| **(4)** *"determinism in its keyed inputs, **nomos-version auto-derived from source**"* | ⚠ **HALF.** Versions are hand-written literals (`"erosion-2026-07-12b-uplift"`). §5 names this the known weak link: *"a stale memo mid-iteration doesn't waste time, it **lies**."* |
| **(5)** *"its **declaration in the nomotheke**"* | ✅ **BUILT.** |
| **(6)** ⚠⚠ *"its **regime probes, written first** — including its **seam probes** — and **paired with the declaration: a declared tier is a falsifiable claim, and the probe is what would convict it.**"* | ⛔ **NOT BUILT.** (7 grep hits, all prose inside `status` strings.) |

> ### **CLAUSE (6) IS THE TYPE-B PROBLEM, AND THE ARCHITECTURE ALREADY SPECIFIED IT.**
>
> From `04-lexicon.md` I derived a carve I thought was mine: *the project has two kinds of
> declaration — those falsified by a **graph query** (built, and they convict) and those falsified
> by a **probe** (English, and 22 of them sit unbound in `examples/`).* **§9 clause (6) is that
> carve, written down on 2026-07-10, as the contract, in the architecture's own words:**
>
> > ***"a declared tier is a falsifiable claim, and the probe is what would convict it."***
>
> **I did not discover this. The architecture states it. Nobody built it.** And the ordinum's
> unreachable `Kept` rung, the 22 orphaned probes, and boxes ②–⑤ are all *one unbuilt clause*.

### And box ③ is NOT greenfield — I was wrong in `03-…`, and §3 is where it lives.

I wrote that semantics was *"the only box with no fossil at all — you cannot leave a stub for a
question you have not asked."* **The question was asked, on 2026-07-10, and answered:**

> **§3, verbatim:** *"the literature fixes one $R$ per method; **we want $R$ per consumer** (hydrology
> needs conserved totals, line-of-sight needs max, display needs mean). So a macro cell honestly
> carries `{mean, min, max, conserved-totals}` **with each field flagged *guaranteed* vs
> *approximate***. **Store the wrong statistic and the fine materialization silently corrupts the
> macro — the one failure mode the whole discipline exists to prevent**, and a systems-theory fact
> (representativity error), not a coding detail."*

**That is the sufficient-statistic + exactness contract, complete, with the failure mode named** —
months before spike-2 "proposed" it, before it was "deferred, not built," and before Jensen made it
*"load-bearing rather than speculative."* **`{guaranteed, approximate}` is the exactness flag.
`R per consumer` is the statistic contract.**

⇒ **Correction to myself: not one of the five boxes is a genuinely new idea. All five are in the
architecture or the lexicon already.** ② is §9(1) + the CHONK *graph* module; ③ is §3; ④ is
§1's three laws + `Promise.conservation`; ⑤ is §2's *"#mech = a fitted statistical closure **missing
its error model***." **The five boxes are not a discovery. They are an INVENTORY OF WHAT WAS
SPECIFIED AND NEVER MECHANIZED.** That reframes the whole build: we are not designing a contract.
**We are executing one that has been sitting here, unexecuted, for three days shy of a week.**

### The two "corrected" docs were not corrected.

ARCHITECTURE's own patched line says: *"The same false claim appears in `multiscale-methods.md` §1
and `multiscale-seams.md` §2.1."* And `DECISIONS[mean-pin-does-not-preserve-block-means]` says:
***"All three corrected 2026-07-13."***

**Checked. One of three.**
- `multiscale-methods.md:38` — *"R∘L=id enforced on the mean, live."* **Still there.**
- `multiscale-seams.md:41` — *"This is what vivarium's `pin_block_means` is … **It enforces R∘L = id
  on the mean**."* **Still there.**
- `multiscale-seams.md:48` — worse: *"`seam_ridge` … measures exactly the R∘L-vs-conservation gap:
  **mean-pin conserves the block mean but not the boundary gradient**"* — **a claim the audit refuted
  twice over** (mean-pin does *not* conserve the block mean; and the zero-physics control shows it
  manufactures a ~2× ridge with **no physics in the world at all**).

*(Tactical, and quarantined below. But the METHODOLOGICAL fact is not tactical: **the ledger asserted
a repair that did not fire.** A `|impact` line saying "done" is a type-B claim — it is about the
world, not about the declarations — and nothing executes it. **The ledger has the same disease as the
nomoi.**)*

---

## 2. What matters for today's session

### ⇒ CHONK already named the module boundary that vivarium violates.

§9's prior-art validation (Gailleton et al. 2024, read 07-10) adopts a four-way decomposition —
**`{fluxes, properties, processes, graph}` as four loosely-coupled modules**, so any one swaps
without the others — and maps it onto us:

| CHONK module | vivarium | built? |
|---|---|---|
| **fluxes** | our fluxed quantities | ✅ `flux.rs` |
| **processes** | our systems / nomoi | ✅ `nomotheke.rs` |
| **properties** | our column / strata | ⚠ *the three-way semantic ambiguity lives here* |
| ⚠⚠ **graph** | **our tile / $R$ / $L$** — *"swap the graph → change grid type"* | ⛔ **NOT A MODULE. It is smeared through every kernel.** |

**And the graph is exactly what the audit convicted.** `cell_m²` hardcoded as an area; `cell_m·√2`
hardcoded as a diagonal distance; MFD's eight fixed `(dx,dy)` offsets. **The grid is not a swappable
module — it is a set of literals inside the processes.** CHONK's stated rule is that *the graph must
be process-agnostic while processes may be domain-specific*, and vivarium has it exactly backwards:
**the processes are grid-specific.**

> **⇒ Box ② (geometry) is not "a new field on `NomosDecl`." It is CHONK's `graph` module, which
> ARCHITECTURE adopted as the contract on 2026-07-10 and never separated out.** Declaring what a
> nomos assumes about its cells is the *interface* of a module that does not exist yet. That is a
> much better description of the work than "add a field."

### ⇒ The demote operator has an honesty gate, and the gate is the law that just failed.

§2: *"**The ladder runs both ways.** Climb to *discover* a system's behaviour; once probe-validated,
**descend** to a tight surrogate reproducing the discovered statistics (**$R\circ L=\text{id}$ on the
chosen statistics is the honesty gate**), keeping the expensive rung as calibrator."*

From `04-…`: *promotion is universal; demotion is the rare and precious operation, and almost nothing
in this project can do it.* **Now I can say why the one ladder that CAN demote does it unsafely: its
honesty gate is `R∘L = id` on the chosen statistics — and `R∘L = id` is the law the audit measured
FALSE in the only place it is implemented.** The demote operator exists, it has a gate, and the gate
is broken. *(And **"the chosen statistics"** is §3's `R`-per-consumer contract — which is not built
either. So the gate is broken **and** underspecified.)*

---

## 3. Wandering thoughts

**This document is not a design. It is a promissory note, and it is almost entirely unredeemed.**

That sounds like a criticism and it is not. Read the status markers it puts on *itself* — the author
was scrupulous: §1 *established* · §3 ***stance*** · §4 ***stance*** · §5 *"established paradigm; **the
engine is a gap**"* · §7 ***stance*** · §9 the contract, with **six clauses**. **The document knows
what it has not built and says so in its own headings.** What it could not know is that *stating* the
contract would feel, for three days, like *having* it.

And that is the failure mode I want to name precisely, because I think it is the deepest one in the
project and it is not dishonesty:

> ### **A SPECIFICATION THAT NOTHING EXECUTES DECAYS INTO A DESCRIPTION OF ITSELF.**
>
> §9 says *"a declared tier is a falsifiable claim, and the probe is what would convict it."* Nothing
> pairs them, so nothing convicts. §3 says *"store the wrong statistic and the fine materialization
> silently corrupts the macro."* Nothing declares the statistic, so it did. §1 says *"$R \circ L =
> \text{id}$."* Nothing checks it, so it isn't. **In every case the architecture was RIGHT, was READ,
> and was CITED — by the very code that violated it.** `pin_block_means`'s docstring cites the law it
> breaks. `DESIGN-MATERIAL §4` declares the semantics the columns drifted through. **Citation is not
> enforcement, and this project has now paid for that lesson at least four separate times.**

**⇒ Which gives the scaffold rebuild its actual criterion, and it is not "add the four missing
boxes."** It is:

> **Every clause of §9 must acquire an executable, or be struck.** Six clauses. Two have one. The
> other four are *prose in a `&'static str`*, and prose in a string is indistinguishable — to a
> machine, and eventually to a reader — from prose in a document. **The `relation` field is
> `ARCHITECTURE.md` with worse formatting.**

---

**A thing I find genuinely lovely, and it is the strongest structural claim in the document.**

§3, the closure choice. The literature's lifting operator $L$ *samples* the missing detail —
Monte-Carlo, ensemble mean, max-entropy — and equation-free explicitly notes that $L$ is non-unique
and *"any choice heals."* Vivarium's is **none of these**: the missing detail is *fated* — a pure
function of (seed, key). And then:

> *"**memoization becomes sound** — if $L$ sampled, two evaluations would disagree and caching would
> change the world; **fated lifting is what lets the entire memo architecture exist** (the seeding
> discipline and the caching architecture are **one decision**)."*

**Determinism-as-ontology is not a philosophical preference that happens to be convenient. It is the
enabling condition of the memo store, and therefore of the entire runtime.** You cannot cache a
sampled lifting. The moment you decide that a world's unseen detail is *fated rather than drawn*, the
save-file can be the memo store, the store can be content-addressed, the query can be lazy, and the
whole architecture falls out. **One metaphysical commitment; the entire engineering follows.**

And it *keeps* paying out, in registers that have no business agreeing:
- **`ante-mundane-delivers-a-forcing-sample-not-a-history`:** a Gyr solar-system integration *"can
  only be considered as a random sample of its possible evolution"* — **the literature's epistemic
  ceiling IS fated noise**, arrived at from celestial mechanics.
- **The AAT handshake:** fated noise = authored $\varepsilon$ — *"genuine chance to the inside agent,
  a deterministic lookup to us."* **The aleatoric boundary is frame-relative**, which is a *theorem*
  about kingdoms and also just how the KRNG works.
- **`preserve-the-structure`:** Mori–Zwanzig says the fluctuation you must add back is *determined*
  by the variance you discarded — **so even the noise is not free.**

**Four independent derivations of one commitment, from numerical analysis, celestial mechanics,
agent theory, and statistical mechanics.** I said in `04-…` that I wanted to know whether *every
place vivarium's ontology is forced is a place the science is epistemically capped.* This is the
fourth instance and I have not yet found a counterexample. **If that holds, "determinism-as-ontology"
is not a design choice at all — it is the only honest position available to anyone building a world,
and vivarium arrived at it early and for the wrong-but-lucky reason (caching).**

---

**The domain-fixation guard, and the thing it quietly admits.**

The header note is emphatic: four **representation kinds** — spatial field · reservoir/box · network
/ state-vector · agent population — and *"validate generality early across more than one
representation **kind***, not merely more than one geomorphic system."

**Score, as of today: field ✅ (many), box ✅ (`hydrosphere` — built explicitly as the generality
probe), network ⛔, agent ⛔.** And the guard supplies its own difficulty gradient: *"network life
(food webs) **strains the tile-as-field assumption**; mobile agents are the cognitive-LOD / agent
seam."*

⇒ **The framework has been validated on the two representation kinds that are easiest for it, and
the two it has NOT been tested on are the two that the project's actual bet depends on.** (The bet:
*simulation-grade agents on ASF.*) That is not an indictment — `hydrosphere` was a real, deliberate,
successful probe and it *did* prove the store/nomotheke/flux contract is coordinate-free. But it is
worth saying plainly that **the generality claim is currently 2-for-4, and the untested half is the
half the project exists for.**

**And here is the thought that will not leave me alone:** a **network** nomos (a food web — trophic
flux + stoichiometry) would be *cheap*, it has **no grid at all**, and it would test the exact seam
that box ② is about — *what happens when a nomos's `graph` is not a lattice?* We have one
representation-kind probe (`hydrosphere`) and it was worth its weight several times over: it proved
generality *and* it produced the framework's only conserved reservoir *and* it became the pattern the
coming `lithosphere` will copy. **A second one, chosen to break the field assumption rather than to
confirm it, might be the cheapest scaffold validation available** — and it would fall exactly on the
untested side.

---

## Tactical residue — quarantined for the DECISIONS-cleanup fork

- **`DECISIONS[mean-pin-does-not-preserve-block-means]` claims "All three corrected 2026-07-13."
  One of three.** `multiscale-methods.md:38` and `multiscale-seams.md:41` still assert
  `R∘L = id on the mean`; `multiscale-seams.md:48` additionally still asserts *"mean-pin conserves
  the block mean but not the boundary gradient,"* refuted twice over.
- ARCHITECTURE §9's CHONK caveat is fine; the *body* is fine. The one-line patch is well-made and
  scrupulous — it states the measurement, names the mechanism, and points at its own siblings. **It
  did the honest thing and then did not sweep.**

## Queue changes

**Correction to `03-the-scaffold-code.md`:** I claimed box ③ (semantics) was *"genuine greenfield."*
**False** — it is `ARCHITECTURE §3`, complete with the `{guaranteed, approximate}` exactness flag.
**None of the five boxes is a new idea; all five are specified somewhere and mechanized nowhere.**
**Next:** `doc/theory/discretisation-and-information.md` (68 K — the largest single payload, and the
one document the audit actually rewrote).

---

## ⚑ ADDENDUM — Joseph, mid-read (2026-07-13): udon-core / udon-util land TODAY

> *"We've been waiting for a solid **udon-core** and **udon-util** crates in order to truly get the
> udon docs integrated in a more deterministic and **code-generating or checking** way. That is
> *almost* done (probably today)."*

**This corrects my central thread, and it corrects it in the good direction.** I had been reading the
project's split as **STRUCTURAL vs PROSE** — a discipline gap. **It is not. It is RUST vs UDON — a
TOOLCHAIN gap, and the toolchain arrives today.**

| | **type A** — falsifier is a **graph query** over the declarations | **type B** — falsifier is a **PROBE** over the world |
|---|---|---|
| **declared in RUST** | ✅ **ENFORCED.** flux-vocabulary closure · ASSUMPTIONS anchors (`include_str!`) · `consumed_and_met_implies_in_deps` · `producers_are_unique` · `BrokenKeeper` · declarations-mint-the-keys | ⛔ nothing |
| **declared in UDON** | ⛔ **NOT YET — and this is why.** LEXICON's 2 dangling `\|rel` edges · the ordinum's schema drift past its own reading-rules · DECISIONS' unwalked supersession chains · no `:status retired` · no `refuted` in ASSUMPTIONS | ⛔ nothing |

**⇒ The udon docs were not unenforced through neglect. They were written in a structured format
AHEAD OF THE PARSER, on the bet that the parser would come** — and `ordinum.rs` says so in its own
header, in as many words: *"Deliberately NOT a general udon parser … when `libudon` is wired as a
dependency **this whole module's front half should be deleted** in favour of it. Marked so nobody
mistakes it for one."* **The hand-rolled parser is a placeholder for precisely what lands today.**

### What udon-core closes, and what it does not

**CLOSES (the bottom-left cell) — and this is most of what I have been flagging for three documents:**
- **LEXICON `|rel :to X` closure** — the same test `flux.rs` already has, applied to the vocabulary
  itself. The 2 dangling edges (`manifest`, `nomotheke`) become build failures.
- **The ordinum's grammar** — `:target` / `:regime` / `:sub-gates` / `|obligation` stop being schema
  drift and become either declared or rejected.
- **`|defeasance` as a REAL, general primitive** across every ledger — the void-condition is a graph
  query, and a graph query over udon is exactly what udon-core makes possible. ⇒ **The missing DEMOTE
  operator (`04-…`) is a udon-core deliverable.** A refuted `ASSUMPTIONS` row that `erosion.rs` still
  consumes could fail the build. **`p = 1.1` would have been convicted the morning after it was
  refuted.**
- **The D-vs-B vibe-modeling query** (`04-…`) — free, once the axes are queryable.

**DOES NOT CLOSE (the entire right-hand column):**
> **Binding a declaration to the PROBE that would convict it.** That is `ARCHITECTURE §9(6)` —
> *"a declared tier is a falsifiable claim, and the probe is what would convict it"* — it is the
> ordinum's structurally-unreachable `Kept` rung, and it is **22 probes in `examples/` that `src/`
> mentions three times, all in prose.** No parser supplies this. **It is the genuinely new work, and
> it is now cleanly separable from what the toolchain is about to hand us.**

**⇒ Revised read of the scaffold build, and it is much smaller than it looked an hour ago:**
1. **udon-core lands** → the type-A/udon cell closes, largely for free. *Do not hand-build any of it.*
2. **The one thing to actually design: the probe↔declaration binding.** A `|predicate` should not be
   English — it should **name an executable**, and the build should run it. The raw material exists
   (22 probes) and the contract was specified on 2026-07-10 (§9 clause 6).
3. **Then** boxes ②–⑤ are *fields whose falsifiers already exist* — not new machinery.
