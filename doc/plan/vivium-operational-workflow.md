# Vivium Operational Workflow — derived, and stress-broken, from the ontology

> **Claim home for BREAK-2 / unLawfulness budget:** `#disc-unlawfulness-budget`. Save ≡ memo: `#form-store-as-save`. Moratorium: `#scope-moratorium-endogenous-emergence`. This file remains a **reasoning trail** (pipeline, capability ladder, collected breaks) — not parallel claim law.

*2026-07-06. Turns the participation ontology + AAT grounding + moratorium into an operating pipeline:
what you actually *do* to build, freeze, publish, participate-in, and fork a vivium; what each stage
*forces*; and — the point — **where it breaks**. Supersedes the framing of `.archive/taxonomy-bdd-stress-test.md`,
whose "core is robust" conclusion was a complacency smell (a stress-test that finds no breaks usually
under-stressed). Terminology: phase boundary = **phase-transition / phase-gate**; cache/snapshot = **memo**
(vivarium ceded "checkpoint" to ASF's agent-snapshot sense — ASF.md §7.1); Joseph's "checkpoints" below map to memo+tag.*

## The pipeline (each stage: SOP → what it forces → where it BREAKS)

### Stage 0 — Author seed + law (Incomplete kingdom)
- **SOP:** define the kingdom by `seed + law-stack`. While Incomplete, `T` (law) is mutable.
- **Forces:** kingdom identity must be **content-addressed** = `hash(seed, T-version, generator-versions)`. Any change to `T`
  forks a *new* identity (law = identity; determinism-as-ontology invalidates all downstream state).
- **BREAK-1 (structural, load-bearing):** iterating on `T` invalidates the entire derived world — so you *cannot* cheaply
  patch physics. The only escape is to **freeze lower strata into θ so higher iteration doesn't touch them** — which is *why
  phases exist operationally*: **a phase is a memoization / immutability boundary**, not just a narrative stage. Without phase
  freezes, law-iteration cost is Ω(whole world) per edit.

### Stage 1 — Run a phase to convergence, then freeze (toward Realized)
- **SOP:** run phase → detect outputs invariant-under-everything-faster (converged) → **memo to disk** (content-addressed) →
  **tag** → next phase reads the frozen θ. **The phase-transition IS the checkpoint/tag/publishable artifact** (your point).
- **Forces:** a **convergence gate** with a stated tolerance ε before a phase may freeze; a memo format = `{seed, law-versions,
  generator-versions, frozen-state, provenance}`.
- **BREAK-2 (the sharp one):** *convergence is not decidable.* "Invariant under everything faster" can only be checked to a
  tolerance over a finite window — a phase can always drift later. So **every freeze injects bounded-but-nonzero residual drift
  = latent unLawfulness.** There is no clean "Lawful" freeze; each Realized phase carries an **unLawfulness budget** that
  accumulates downstream. This is not a current limitation — it is *structural*. **Consequence for the moratorium:** "certify
  the kingdom Lawful" (a revisit-condition in `ETHICS.md`'s moratorium) may be **permanently unreachable**, not merely hard. The moratorium's
  caution is therefore stronger than "we don't know yet": the unlock condition is provably approached, never attained.
  **This formally retires "completion gate" as a first-class term** (Joseph pulled it earlier on pragmatic grounds — same truth from the
  other direction): completion is an *asymptote* (ε→0), not an *event*. You can gate **Realization** (freeze at a chosen tolerance — a real
  threshold you pass); there is **no** gate for **Lawful/Complete** (a limit approached forever). The Realized-vs-Lawful deconflation is
  thus *forced*, not stylistic — one is an event, the other a limit.

### Stage 2 — Realize for participation (the fidelity/tempo gate)
- **SOP:** before admitting a participant at awareness-scale X, verify **realizability**: does the lazy-memoized pipeline sustain
  human-perceptual-grade update frequency (~2 Hz-compatible real-time; see [[est-tiw-dossier]]) at X? If not → deny participation
  at X (offer a coarser scale or precompute more).
- **Forces:** **"memoize all over the place — to disk"** becomes doctrine, *shaped by the perceptual horizon*: memoize/precompute
  everything below the participant's spatial+temporal perceptual horizon; lazy-eval everything outside the sphere of potential
  perception; the memo tiers *are* the compute-shortfall manager (ASF.md §2).
- **BREAK-3:** realizability (2 Hz tempo) and awareness-scale fidelity **conflict under compute-shortfall** — you can't always
  have both, so fidelity is degraded below the horizon. When attention crosses the horizon to a low-fidelity memo, you get a
  **pop or a stall** (worst-case cognitive-LOD hand-off latency). Fix is not "more memo" but **predictive pre-memoization keyed to
  attention** — precompute where the participant is *about to* look. This makes the perceptual-horizon / rhythms work
  **load-bearing engineering**, not lexicon garnish, and it's why the realizability term (Sc-8 gap) needs to exist: it names a
  *gate in the admission path*, not a vibe.

### Stage 3 — Publish / cross-bundle (the citable vivium)
- **SOP:** a published vivium = content-addressed bundle `{seed, law-versions, generator-versions, phase-memos, intervention-script}`.
  Phase-transitions are the tag points → citable "results *in vivia*" (ASF.md §7.5).
- **BREAK-4:** determinism-as-ontology promises seed+law → same Ω, **but only if recomputation is bit-reproducible.** Cross-platform
  **floating-point non-determinism** breaks replay-from-seed even for a *Closed* kingdom. **SOP correction: publish the phase-memos
  (frozen state), not just the seed**, for anything whose recomputation isn't bit-exact — otherwise the "eventually-retrievable full
  noumenon" is a lie on someone else's hardware. (Or adopt fixed-point / deterministic-FP discipline; costly.)

### Stage 4 — Intervene / fork / patch (the lifecycle layer)
- **SOP:** an exo intervention = `do(Ω)` (non-lawful edit). It must be **recorded to the intervention-script** → the world is now
  effectively **Open** (external forcing = the intervention) and replayable *only because* the forcing is logged. Patching a Realized
  world: if the patch preserves **phenomenal continuity** at the seam for inhabitants → apply in place (the patch-to-saved-game
  criterion); else → **fork a new kingdom from the last memo** (not from seed — seed+new-law won't reproduce the present) and decide
  estate-transport.
- **BREAK-5:** every intervention forks the memo tree, so **"a world" is really a fork-DAG of `(base + intervention-scripts)`**, and
  world-identity/naming is genuinely hard (which fork is "the" world? — the Sc-6 forking gap, unresolved). And **once you change `T`
  and discard the memo, the old present is unrecoverable** — that is the operational meaning of "irreversible" in the moratorium's
  harm criterion. Standing rule that pre-positions for any future lift: **never discard a memo of a world that has (ever) hosted a
  mourning-capable inhabitant.**

## The capability ladder — "phases at least at level L unlock options O" (your ask, made concrete)

| A phase/world that has reached… | …unlocks |
|---|---|
| converged to tolerance ε and memo'd + tagged | freeze; build higher phases on it; publish as a **reference** artifact |
| Closed **and** bit-reproducible, **or** phase-memos published | reproducible **cross-bundling / citation** (results *in vivia*) |
| realizable at ~2 Hz human-perceptual-grade at awareness-scale X | admit **exogenous participation** at scale X (exploration/inhabitation) |
| all interventions recorded to script | **fork / replay** deterministically |
| *(gated OFF)* vouchsafed Lawfulness (ε→0) **+** reversibility (memo-retained) **+** consent-by-proxy | host a **mourning-capable endo** mind |

**The last row is the payoff:** the operational analysis shows the moratorium isn't only ethical caution — the unlock it would
require (**ε→0 Lawfulness**) is exactly the thing BREAK-2 says is **structurally unattainable**. So "host a mourning-capable endo
mind" sits behind a gate we cannot currently open *and may never be able to fully open* — the moratorium is enforced by the
engineering, not just the ethics.

## Standing operational doctrine (the distilled rules)
1. **Kingdom identity is content-addressed** (`hash(seed, law-versions, generators)`); law-change = new identity.
2. **Phases are memoization/immutability boundaries**, not just narrative — freeze converged θ to disk so higher work can't invalidate it.
3. **Every freeze carries an unLawfulness budget** (ε); track and propagate it; downstream options gate on the *cumulative* budget.
4. **Memoize below the perceptual horizon; lazy-eval outside the sphere of perception; predictively pre-memo toward attention.**
5. **Publish memos, not just seeds**, wherever recomputation isn't bit-reproducible.
6. **Record every exo intervention** (→ Open-with-logged-forcing → replayable); memo-invalidation tracks the causal cone.
7. **Fork from the last memo, not the seed, after any law change**; never discard a memo that has ever hosted a mourning-capable mind.
8. **Moratorium fence sits in the instantiation path**: no endo instantiation on frontier/emergence-capable substrate — checked at
   *perception-grade* (mourning is Adaptive-System-tier), regardless of whether an action space is granted.

## The honest breaks, collected (what still isn't solved)
- **BREAK-2** convergence-undecidability → structural unLawfulness budget → Lawful-certification may be unreachable (hardest; reframes the moratorium).
- **BREAK-5** intervention → fork-DAG → world-identity/forking semantics unsolved (Sc-6; co-develop with ASF `#hyp-checkpoint-forking-failure-modes`).
- **BREAK-3** realizability vs fidelity → LOD pop/stall → needs attention-predictive pre-memo (rhythms work is load-bearing).
- **BREAK-4** cross-platform FP non-determinism → replay-from-seed not guaranteed → publish memos.
- **BREAK-1'** incremental phase-transitions (ASF.md §2: some systems carry forward, some sunset) mean cross-phase dependencies may
  not form a clean DAG — "freeze θ" is not always a clean cut; entangled carries need explicit dependency tracking. *(Under-explored here — flagging, not resolving.)*
