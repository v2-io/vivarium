# 00 — initial predictions, written before reading anything

*2026-07-13, ~13:30. Written from: `CLAUDE.md`, `ORIENTATION.md`, `NOMOS-CONTRACT.md`,
`ASF.md` §0–§1, the project memory index, ~10 of ~70 DECISIONS entries, four verified
lines of code, and the `git log` dates. Nothing else.*

**The point of this file is to be WRONG in a legible way.** A prediction I cannot be
caught on is worthless. So each one gets a **confidence** and, where I can manage it,
a **concrete falsifier** — the specific thing I'd expect to see on the page.

**Confidence key:** `[hunch]` — pattern-match, could easily be nothing · `[expect]` —
I'd bet on it · `[strong]` — I'd be genuinely surprised to be wrong · `[?]` — pure
guess, recorded so the guess is on the record.

---

## The global forecast — five things I think I'll find, stated so they can fail

### G1. `[expect]` The scaffold's real gap is not the four missing boxes. It is that the scaffold has no *owner* concept.

Joseph just said it out loud without saying it: *"we might need to already start giving
nomos their own independent todo-lists so we can stop conflating those tactical concerns
with the emerging global concerns."* That is not a filing suggestion. **It is a claim that
the project currently has no boundary between what a nomos OWNS and what the scaffold
owns** — and that's why the audit day's findings arrived as one undifferentiated pile of
twenty decisions spanning "p should be 1.0" and "conservation buys the coarse tier its
freedom."

**Prediction:** nothing in `doc/` names this boundary. `NomosDecl` will turn out to be a
*declaration* (what a nomos claims) with no *jurisdiction* (what a nomos may decide alone,
what it must escalate). **Falsifier:** if `ARCHITECTURE.md` or `DESIGN-SYSTEMS.md` already
carries a nomos-autonomy / scope-of-authority concept, I'm wrong and I should find it and
use it.

**And I think this is the same shape as the five boxes.** Box ① asked *"what flows between
nomoi?"* — quantities. The unasked question is *"what DECISIONS flow between nomoi and the
scaffold?"* Same generalization move, applied to authority instead of physics. If nomoi get
agents, that boundary stops being a documentation nicety and becomes a **protocol**.

### G2. `[strong]` The `doc/design/` and `doc/plan/` trees will contain claims the ledger has already killed, and nobody has swept them.

Dates alone force this: audit on 07-12/13, those docs frozen at 07-11. **Concrete falsifiable
guesses:** `DESIGN-SYSTEMS.md` will describe the fluvial pipeline (MFD, stream power) as
principled or settled, with no hint that MFD's output is ill-posed in the continuum.
`DESIGN-REDUX.md` §12's pervasive-memoization doctrine will state independent-tier evolution
as a premise **without knowing it is under threat** from flux-on-the-face. `abyssal-parity-plan.md`
will still route land through `uplift` as a producer. **If I find sweep markers or
"superseded by" notes in these, I'm wrong about the project's hygiene and should say so.**

### G3. `[expect]` The ordinum will be *better* than I expect and *smaller* than I expect.

Everything I've read treats it with unusual respect — it "governs the flux web," it "validates
its own ladder phase for phase" against a 2025 Annual Reviews paper the project read *after*
writing it. 22 KB is not much for something carrying that much weight. **Prediction:** it is
mostly *charges* and *promises* with terse predicates, and its power comes from being **data
that other machinery reads**, not from being long. **The thing I most want to check:** whether
its phase boundaries are *stated as falsifiable predicates* or as prose. The maturity ladder
(named → specified → claimed → kept) says no fulfillment-claim without a predicate — so
**how many promises actually have predicates?** I'd guess **fewer than half** `[hunch]`, and
that the gap between "promises that exist" and "promises that could ever be checked" is the
ordinum's real weak seam.

### G4. `[expect]` I will find that the project has *two* ladders and has not noticed they are the same ladder.

The macro-tier decision half-says this already — Joseph's scale-ladder is "the phase ladder
drawn on the *scale* axis instead of the *time* axis," and the entry's conclusion is *"no new
machinery is needed — only the noticing that it is the same machinery."* **But I predict there
is a THIRD** `[hunch]`: the **fidelity/maturity ladder** (declared → derived; NotStarted →
Specified → Claimed → Kept) is *also* the same shape — *run until converged, promote the
converged thing to law, the law constrains what runs next.* Time, scale, and **epistemic
status**. If all three are one recursion, the scaffold has one primitive and three instances,
and that is a much smaller thing to build than it looks. **Falsifier:** if the epistemic ladder
turns out to have a genuinely different structure (e.g. it's not monotone, or it has no
"promotion" step), the unification is false and I should drop it fast.

### G5. `[hunch]` The five boxes are not five. They are two, plus a rule.

② geometry, ③ semantics, ④ structure, ⑤ claim. But ⑤ (the modified equation) is *computed*,
and it **subsumes** a lot of ④ (what did the scheme destroy?) and some of ③ (what did it do
to the meaning?). And ② is *what the algorithm assumes* vs *what the grid delivers* — which is
a **mismatch check between a declaration and a fact**, structurally identical to ① (what a nomos
needs vs what another produces). **So my guess: ① and ② are the same machine (a requisite check,
run on quantities vs run on geometry), and ③④⑤ are the same machine (an analysis you RUN on the
kernel and record the output of).** The bias-vs-noise rule is the verdict function shared by both.
**If true, the build is two mechanisms, not five, and the five docs are teaching material rather
than architecture.** I hold this loosely and expect the reading to complicate it.

---

## Per-document predictions

### 1. `tabularium/terrestris.ordinum.udon` (07-11, 22 K)
- `[expect]` Structure: phases (Protogenic → Abyssal → …7), each with **charges** (what the
  phase must do) and **promises** (what it delivers), typed `state/regime/capability/limit`,
  `:kept-by <nomos>`, `|predicate`.
- `[expect]` The `:tag gate` mechanism will be sparse — a handful of gates, not one per promise.
- `[hunch]` Most promises will have **no predicate**, which is why `Kept` is never auto-derived.
- `[hunch]` It will contain something about **time-reckoning per phase** (the Abraham-3 nested
  time-reckonings are in the memory as source cosmology) — and if it does, that's the *time* axis
  of G4, sitting right there.
- `[?]` I do not know whether phases are *durations* or *states*. If they are states (a phase is
  a conjunction of kept promises, not an interval), then the ladder is already declarative and
  G4 gets much easier.
- **What I most want:** does a phase *end* because promises are kept, or because time passes?
  **That single question decides whether the ordinum is a schedule or a law.**

### 2. `DECISIONS.decision-log.udon` (07-13, 232 K, ~70 entries)
- `[strong]` The 2026-07-13 entries will be far more interesting than the 07-12 ones, and the
  07-12 ones will be substantially superseded by them.
- `[expect]` I will find **more incoherences between entries** than the handoff admits — it
  already confesses "some findings are double-recorded under different slugs."
- `[expect]` The authority tags will be mostly `:by claude :status proposed`, and the handful of
  `:by us`/`:by joseph` will be the actual spine of the project. **I plan to extract exactly those
  and read them as a document in their own right** — *the ratified spine* — because that, and not
  the 232 K, is what binds.
- `[hunch]` There will be at least one decision that contradicts another and nobody noticed, in
  a way that matters. (The three flagged possibly-inflated `:by us` tags are the known ones; I
  predict there are unflagged ones too.)
- **Reading discipline I'm committing to now:** for every entry, note **date, authority, and
  whether it was superseded** — and build the ratified spine as I go.

### 3. `LEXICON.udon` (07-12, 63 K)
- `[strong]` `regula` is still `:status settled` though retired (the handoff says so), and
  `manifest` — load-bearing — has **no entry**. `:status` has no value meaning *retired*.
- `[expect]` It will be *much* richer than a glossary — the memory calls it canonical for the
  participation taxonomy (§4/§7/§8), the perceptual band (§8), in-vivia epistemics (§4). **It is
  probably the closest thing the project has to an ontology, and it is filed as a dictionary.**
- `[hunch]` I will want to add: **`nomos jurisdiction`** (G1), and a `:status retired` value.
- `[?]` Whether it distinguishes *nomos* (the law-unit) from *kernel* (the code) from *scheme*
  (the discretisation) crisply enough to hang the five boxes on. **If it doesn't, that's the first
  scaffold job** — because the five boxes are all about the *scheme*, and the flux web is about
  the *nomos*, and those are not the same object.

### 4. `doc/ARCHITECTURE.md` (07-13, 29 K)
- `[expect]` "One principle (represent by consequence), three axes." The domain-fixation guard.
  §9's "representation-agnostic interaction contract" — which `NOMOS-CONTRACT.md` says *"has
  existed as a principle for months and has never been cashed out."*
- `[expect]` §1's operator algebra "third law" = timescale separation.
- `[hunch]` It was touched on 07-13 but only lightly — I doubt the audit rewrote the frame.
  **Falsifier:** `git log -p` will tell me in seconds; if the 07-13 change is substantial, my
  model of "the audit didn't reach the frame docs" is wrong.
- **The question I'm bringing to it:** *represent by consequence* is a fidelity principle. Does
  it have anything to say about **what a nomos is allowed to assume**? Because that is box ②,
  and if the architecture already implies it, the contract is a *cash-out*, not an addition.

### 5. `doc/theory/discretisation-and-information.md` (07-13, 68 K)
- `[expect]` The biggest single-doc payload in the project and the intellectual core of the
  audit. FV/FD/FE taxonomy, the structure table, bias-vs-noise, one-reconstruction-per-consumer.
- `[strong]` **It contains at least one thing the ledger already corrected** (§3.3 — the Jensen
  variable and sign; the entry says "correct §3.3 … (done 2026-07-13)" so possibly already fixed
  — *check whether the fix actually landed, because "done" claims are exactly what this project
  has learned to distrust*).
- `[hunch]` It will be a *theory* doc where I want a *procedure* doc, and the five nomos-contract
  files exist precisely because someone noticed that. **If so, there is a duplication risk: five
  new files that restate this one.** That's worth flagging *before* anyone writes them.
- **The thing I most want to know:** does it already contain the **modified-equation computation**
  worked on our kernels, or only the recommendation to do it? `CLAUDE.md` says the tool "would have
  short-circuited most of 2026-07-13's archaeology in an afternoon" — which implies **it was not
  used**, which implies the doc *recommends* it rather than *carries* it.

### 6. `ASSUMPTIONS.md` (07-13, 21 K)
- `[expect]` A table of magic constants with anchors compiled into tests.
- `[hunch]` The audit added entries but **did not remove any** — and I predict several constants
  are now known to be *not merely unprincipled but actively wrong* (`p = 1.1`, the θ smoothing,
  Jarrett's constants), and the ledger has no way to say that. **An assumption ledger with no
  "refuted" state is the same defect as `:status` having no "retired."** If I find that, it's the
  same missing primitive twice, and that's a real finding.

### 7. `TODO.md` (07-13, 37 K)
- `[expect]` Large, sectioned, partly voided by the audit.
- `[strong]` It will conflate exactly the two things Joseph wants separated — nomos tactics and
  scaffold work — because that's *why* he said it. **This is the doc his instruction is really
  about.** I expect to propose splitting it.

### 8–11. `doc/design/*` (07-11)
- `DESIGN-MATERIAL` §4: `[strong]` states the conserved-primitive law correctly; `[expect]` the
  guard it describes is the one the code "drifted through anyway."
- `DESIGN-REDUX` §12: `[expect]` pervasive memoization as *standing directive*, independent-tier
  as premise, **no awareness of the leaf-only threat.**
- `DESIGN-SYSTEMS`: `[expect]` the fluvial inventory presented as sound. **This is where G2 should
  bite hardest.**
- `DESIGN`: `[expect]` the three founding commitments; probably still true and probably the least
  stale of the four.

### 12–13. `doc/theory/multiscale-*` (07-11)
- `[expect]` `multiscale-seams` will already contain the seam law ("structures cross iff the
  restriction operator commutes") **or** will be the doc that the audit's version of it contradicts.
- `[hunch]` The **resolution-light-cone / dynamic-exponent-$z$** unification will turn out to be the
  same recursion as G4, a third time. If position-and-time are one seam, and the phase ladder and
  the scale ladder are one ladder, then *these are the same claim* and the project has derived it
  twice from two directions without joining them.

### 14–18. `doc/plan/*` (07-11)
- `[expect]` Mostly still-valid operational design (beacons, depend-by-key, the fidelity pyramid).
- `[expect]` `vivium-operational-workflow`'s **BREAK-2** (convergence undecidable ⇒ every freeze
  carries a structural unLawfulness budget ⇒ "certify Lawful" may be permanently unreachable) is
  the deepest thing in the plan tree and connects **directly to the moratorium's redeemer condition.**
  I want to sit with that one.

### 19–27. norms, front doors, ASF §2+, the VIVARIA-* spikes
- `ASF.md §2` `[expect]`: the invariance cut, and "the tower derives by self-similar application"
  — **which is G4 again, stated in ASF's own voice, before vivarium re-derived it.** If so, the
  scale/phase/epistemic unification is not a discovery, it's an *inheritance*, and I should say so
  plainly rather than dress it as new.
- `VIVARIA-*` `[expect]`: right register, stale content (Joseph said so). Read for the *move*, not
  the claims.

---

## The papers — I don't know what we have

**I have taken no inventory.** Known-wanted from the ledger: **Korenaga et al. 2017** (freeboard
modelling, `⊘ unread`, blocking the isostasy predicate) · **van Hunen & van den Berg 2008**
(`⊘ unread`, the strength/buoyancy half of the plate hypothesis) · **Johnson et al. 2017 Nature
543:239** (`⊘`) · **Coatléven 2020 [11]** (`⊘`, the cell-to-cell proof). Read primaries include
Chowdhury 2025, Coatléven & Chauveau 2024/2025, Prescott 2025, Armitage 2019, Bonetti 2018.
**Task: inventory `ref/research/pdfs/` and relata before deciding what to fetch.**

---

## The one prediction I'd most like to be wrong about

`[hunch, and I hope it's wrong]` **That the five nomos-contract docs, if written as briefed, will
be read once and never again.** The brief is *excellent* — procedure not taxonomy, carry your own
probe, carry a failure gallery. But `DESIGN-MATERIAL.md` §4 **also declared the semantics, also
wrote a guard whose stated purpose was to stop exactly the drift that happened, and the code drifted
through it anyway.** The project's own failure gallery contains a *previous doc that did everything
these five are supposed to do.* **The lesson I'd draw before writing a line of them: a document did
not fail because it was a taxonomy. It failed because nothing EXECUTED it.** ⇒ **Build the
`NomosDecl` fields and the checks FIRST; let the docs be the thing that explains the checks that
already fail the build.** That is the same conclusion `NOMOS-CONTRACT.md` reaches in its last line
— but for a *different reason*, and I think mine is the load-bearing one.
