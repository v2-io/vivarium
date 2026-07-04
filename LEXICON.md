# vivarium — LEXICON

*(Seeded 2026-07-03 from Joseph's sketch + the term-audit of the design docs.
This is an **inventory, not a set of decisions** — status markers are the
point. Naming discussion in progress; entries move to **settled** only when
Joseph and a session have deliberately agreed. See `~/.claude` memory
[[naming-principles]]: a term earns its place by standalone citability.)*

**Why a lexicon at all.** Two forces: (1) terms we **carved** away from
colliding ordinary/field usage — the carve must be recorded or future sessions
will re-collide; (2) terms **unique to this project** (or uniquely named here
despite analogs) — these carry design decisions inside them, and vague usage
quietly un-decides them. The deeper motivation (Joseph, 2026-07-03): the
tension web of *caching × deterministic seeding × effortful world-project
building × game saves × strictly temporal-causal runs × iterating on
in-between stuff* has already produced slowdowns, confusion, and arguments.
Names are how we stop re-arguing.

**Status markers:**
- ✅ **settled** — in live use, defined, collision-checked.
- 🔨 **carved** — deliberately chosen *against* a recorded collision; the
  collision is part of the entry.
- Ⓝ **open** — needs a noun / name / lexicon decision (Joseph's sketch mark).
  Candidates listed are first tries, loosely held.

---

## 1. World-structure (phases & gates)

- 🔨 **Phase** — a span in which a set of macro systems runs until converged.
  *Carved against "epoch"* (three collisions: geology's, the erosion solver's
  step unit, ordinary usage). `Checkpoints.md`.
- ✅ **Epoch** — reserved exclusively for the erosion solver's step unit.
- ✅ **Checkpoint** — the gate between phases; the conditions a phase must
  deliver before the next can honestly boot; computationally, a world-scale
  memo entry. ⚠️ `Checkpoints.md`'s intro still says a checkpoint's "in-world
  form is the STRATA it leaves" — Joseph (2026-07-03) demoted this: strata
  are one small, lossy evidence among *many* things a phase passes forward;
  pending rewrite with the seams discussion.
- ✅ **Charge** — what a phase must establish; the *gating subset* of its
  bequest; IS the next checkpoint's opening condition.
- ✅ **Bequest** — everything a phase hands forward that persists and shapes
  later dynamics — features, regimes, capabilities *and incapabilities*
  (the coal window). The load-bearing forward concept.
- ✅ **Record** — the bequest's readable-in-rock slice, *demoted to sanity
  probe*: a lossy shadow (a feather in a fossil is a relic, not a fundamental
  output), useful as a consistency check that the right past processes ran —
  never sufficient to reconstruct the past, and not the canonical state.
- ✅ **Gate tags** — #gate (hard prerequisite, we simulate) / #earth (Earth's
  pattern, kept for fidelity) / #mech (causality known, no simulable mechanism
  yet) / #emergent (verify, don't build). Epistemic honesty about *arrows*,
  never about implementation status.
- ✅ **Target** — Phases 6/7/8 as TARGET 1/2/3: the playable entry points;
  ETHICS.md binds from Phase 7's checkpoint.
- ✅ **Canceling pair** — a shield and its threat jointly unbuilt at zero
  fidelity cost until a consumer reads their *difference* (magnetic field ×
  solar wind).
- ✅ **Declare causally, materialize lazily** — parameters committed at their
  causal position, computed on first downstream pull (the moon).
- ✅ **Regime** — a working, deliberately **non-MECE** label for the *dominance
  structure* of the vivarium at a given causal-step: which systems, states,
  and cycles (and, optionally, which scales and shock-generators) currently
  dominate the dynamics. Fuzziness is by design — "dominant" is
  **consumer-relative** (moonlight is negligible dynamics until something
  depends on it: werewolves, or orbital mirrors making all light-sensitive
  systems lunar). Two anchors keep the fuzziness honest rather than sloppy:
  *(a)* **dominant balance** — the asymptotics term of art for "the terms that
  dominate in a given region of the problem"; a regime is a dominant balance
  over the phenomenon set. *(b)* **biome** — precedent that a *useful*
  classification over continuous fields need not have crisp ontological
  boundaries to earn its keep. (Joseph, scratch: regimes/regime-changes may
  stay gloss / non-MECE working labels; *phase* is the crisp one.)
- Ⓝ **Regime-change** — activation, deactivation, or replacement of the
  dominant systems (or a newly-activated system coming to dominate). A **phase
  boundary is a regime-change we *certify*** — checkpointed, charged, gated;
  most regime-changes we never certify. (Still slightly vague — Joseph flagged
  it; the certify/don't-certify split is the sharpening.)
- Ⓝ **Subphase** *(reserved)* — an intermediate regime *within* a phase; what a
  regime-change becomes when a phase (especially Abyssal ☆) earns internal
  gates.
- ✅ **World** — any simulated-planet vivarium; the primary/initial *kind* of
  vivarium. **(phase-name) world** — a world evolved to that phase *at least in
  the areas of granularity and proximal interest* (an "Abyssal world wearing a
  Primeval sky"): fidelity is local and consumer-relative (ties to the
  telescope, §2), never a global claim.
- 🔨 **Earth / earth** — capital-E **Earth** is the real planet, in the real
  universe, that humans experience (never a generated world, however faithful).
  Lowercase *earth* is ground/soil. *Carve/collision to hold:* three senses
  coexist — **#earth** the gate tag (Earth's *pattern*, kept for fidelity),
  **Earth** the referent planet, *earth* the material; keep them typograph­ically
  distinct.

## 2. Runtime & fidelity

- ✅ **Fidelity ladder** — spend representation by consequence
  (`DESIGN-REDUX`).
- ✅ **Telescope** — nested resolution tiers around a focus, re-anchored on
  drift (the erosion telescope).
- ✅ **Finisher** — a fine tier run for 1–2 animated passes over its parent's
  band, mean-pinned to conserve (Joseph's field observation, 2026-07-02).
- ✅ **Memo** — a cached converged state in the lazy query-graph runtime
  (`DESIGN-REDUX` §11–13); a checkpoint is the largest memo.
- ✅ **Patch / halo** — the Cartesian stencil substrate. **Column / Stratum**,
  **CellId** — the matter and spatial keys (`DESIGN-MATERIAL`).
- ✅ **Detail must be earned** — never paint sub-sim detail the physics
  didn't produce (Joseph's rule; wet-cell rendering).

## 3. Determinism & provenance — Ⓝ the open cluster

*(The cluster the sketch exists to untangle. Names here must serve the
tension web named above.)*

- Ⓝ **the seeding discipline** — currently "§8 honest stochasticity" /
  "coordinate-hashed noise": *all stochasticity is a stateless pure function
  of (world-seed, coordinate/key) — never a shared mutable stream.* Needs a
  real name AND a crisp one-paragraph description (Joseph, 2026-07-03).
  First tries: *keyed noise* (descriptive), *fate* (evocative — a cell's
  fate is fixed by identity, discoverable in any order, same forever).
- Ⓝ **generator pinning** — FILL_ALGO_VERSION generalized: frozen state must
  pin the algorithm version that produced it; a cache key is
  f(inputs, seed, generator-version). Already assumed; unnamed.
- Ⓝ **run modes** — the modes the tension web implies are *different things
  we currently call by one name* ("running the world"): strictly
  temporal-causal runs · replays from pinned generators · iteration runs on
  in-between state (discardable) · live play accruing history. Each likely
  needs a name before the confusion stops.

## 4. World-as-artifact — Ⓝ (the sketch's subject)

- Ⓝ **the named world-project** — a NAME + initial/primary invariants + a
  stack of phase memos at various completeness/coherence/fidelity.
- Ⓝ **pre-target phases / Frozen / backstory** — everything before the chosen
  target, frozen into canon.
- Ⓝ **interventional history** — what accrues after the engine goes live:
  play, agent action, author intervention.
- Ⓝ **"a vivarium instance"?** (Joseph's own question mark) — frozen backstory
  + target world w/ engine + interventional history. First tries: *a
  vivarium* (the project name then honestly names the class); *tenancy*.
- Ⓝ **temporary / discarded / interventional layers** — scaffolding runs
  (today's deluge fill is one); never canon, must be impossible to confuse
  with canon.
- Ⓝ **saved games & participatory worlds & simulations** — forks off an
  instance (cheap under content-addressed storage).
- Ⓝ **state-of-the-art stack vs best-of-class library** — the moving edge vs
  the curated pinned set worlds actually freeze against; and the open
  question written on the sketch: *is the interface stack distinct from the
  algorithm and from the target-machine implementation?* (The CPU-reference
  determinism policy in `ref/research/water-parallelism.md` already implies
  yes for algorithm-vs-machine.)

## 5. Epistemic labels for implementations — Ⓝ (the discipline)

*(Joseph, 2026-07-03: we need discipline in how phenomena are introduced,
checked, and iterated per phase — labeling so honest that accidentally mixing
vibe-modeling with principled simulation becomes very, very difficult.)*

What exists today, partial: the ledger's **physics column** (hi/med/lo/proxy),
the **approach codes** (A/R/S/T/P), the gate tags (§1). What's missing is a
per-phenomenon label — ideally structurally enforced (module boundary or type,
not just prose) so a #mech stand-in cannot silently feed a consumer that
believes it is reading physics.

**The tag is not one number — it is several axes that fail *independently*,**
and the independence is the whole point (collapsing them is how vibe hides).
Draft set:

- Ⓝ **A. Earth-history fidelity** — how well does real-world *science* know
  this happened *this way, on the actual Earth*? A first-draft ordinal scale
  (from Gemini, kept as a starting point): deterministic-causal → converging
  independent proxies → uniformitarian extrapolation (modern process mirrors
  ancient signature) → coarse-bounded (constrained, path unknown) →
  speculative-plausible (no surviving record) → anachronistic-hypothetical.
  **Jettisonable wholesale** for non-Earth world-types.
- Ⓝ **B. Physics fidelity** — is this grounded in the *actual laws of our
  universe*, independent of Earth's particular history? **We prioritize B over
  A** (Joseph, 2026-07-03). The reason they must be separate: the same laws
  admit many histories, so a world can be physically impeccable yet un-Earthlike
  (a *different but lawful* past) — or Earth-mimicking yet physically cheating
  (that is exactly vibe). We want **lawful first, Earth-shaped second.** B is
  jettisonable only for deliberately-experimental-but-internally-coherent-physics
  worlds; for our *current* priorities B runs high and A is a welcome anchor and
  sanity-check, not the master.
- Ⓝ **C. Relation type** — the role a phenomenon plays in *our* causal graph:
  *mechanistic-causal > mechanistic-influential > empirically-correlated >
  evidenced-ordered* (weakest: evidence only that X existed before/at a regime
  boundary). This **refines the coarse gate tags** (#gate/#earth/#mech/#emergent
  are relation-type buckets), not a parallel system.
- Ⓝ **D. Implementation status** — what *we* actually built: approach code
  (A/R/S/T/P), physics tier (hi/med/lo/proxy), **probe-verified?**, and the
  arithmetic-seam discipline (**land vs count** — today's f32 duals: an
  increment that must *land* wants compensated summation; one that may honestly
  not-land must be *counted* — see §6 / the conservation fixes).

**The firewall, stated once:** *vibe-modeling = high D riding on low B/C* — a
phenomenon polished to look right without being right **for a reason.** Making
all four axes explicit per-phenomenon turns that combination from camouflaged
into glaring. (Worked instance: `ref/research/oxygenation-transition-scaffold.md`
tags one real regime-change across all four.)

- Ⓝ **Use-case = fidelity contract.** Each use-case *is* a statement of which
  axes must be honest — it is the **largest-scale consumer** in the
  consumer-dependent sufficient-statistic sense (`DESIGN-REDUX` §5), sitting at
  the very top of the fidelity ladder. Roughly: *natural-world hypothesis
  testing* needs A+B+C all real (you are measuring surprisal against reality);
  *Earth-simulation* pins A hard; *speculative-coherent game* needs only C-
  consistency + B (lawful, not Earth-true); *agentic inhabitation* additionally
  trips ETHICS.md; *ASF-theory simulation* needs the agent layer legible;
  *personal exploration* asks only for coherent beauty. This is also what the
  sketch's **SOTA-stack vs best-of-class-library** split (§4) is *for*:
  different contracts pin against different stacks.

**Companion tool — the regime-change audit checklist.** Distinct from the
per-phenomenon tag above: at each *certified* regime-change (§1), Gemini's
five aspects are a useful "what changed?" checklist — **thermodynamic context**
(energy gradients / free-energy taps), **network topology** (mixing,
connectivity, feedback wiring), **substrate innovation** (a genuinely new
state/information-carrying medium — the capability-bequest, cf. the coal
window), **homeostatic inertia** (the old regime's buffering vs the new
attractor's pull), **exogenous triggers** (shocks crossing a tipping point).
Several already appear in `Checkpoints.md` charges (Phase 2's energy-gradient
channels; "inheritance outside the genome" *is* substrate innovation; the §8
storm/flare schedules *are* the exogenous profile) — mild convergence-evidence
the decomposition is real.

## 6. Time

*(The temporal half of the lexicon. Joseph, scratch: inside phases there is no
intrinsic need to track "seconds since ignition"; that total is a curiosity,
maybe a research target, never a world-building necessity. The load-bearing
invariant is something weaker and truer.)*

- 🔨 **Causal time** *(≈ **logical time**)* — the monotonic, immutable *ordering*
  of events within a local regime: the *happened-before* partial order (Lamport's
  logical clocks are the prior art). **This is the only invariant that must
  survive inside a phase** — what is preserved is the ordering, not any clock —
  and even it is sometimes waived (Phase 0 Ante-mundane, which we don't simulate
  at all). The general principle: *the causal partial order is load-bearing; the
  clock is derived, and often optional.*
- ✅ **Metric time** — quantized, measurable duration in a *world's own*
  seconds/years. A **late-phase luxury**: real only once the world contains
  consumers of it (seasons, calendars, agents that plan against a clock). The
  time datum (`vivarium-world::time`, `i64` deciseconds) is anchored at
  **Holocene onset** precisely because that is where chronology — metric time
  that *matters to inhabitants* — begins; the deep phases run on causal time
  beneath it. *Declare causally, materialize lazily* (§1) is exactly
  causal-position decoupled from evaluation order.
- 🔨 **Physical time** — **reserved for real-world / real-universe metric time**
  (wall-clock, our-universe seconds): the confounding *external* clock, distinct
  from any generated world's metric time. *Carve (Joseph, 2026-07-03):* do **not**
  use "physical time" for in-world metric time — that collision is the whole
  reason the term is reserved. (Note the symmetry with **Earth/earth** in §1:
  the real referent gets the protected name.)
- ✅ **Tick** — a solver's internal quantized `dt`, chosen for stability; a
  numerical detail, **not** a world invariant. (**Epoch**, §1, is the erosion
  solver's tick with a name.) Many algorithms quantize time for stability; that
  quantization is a tick, and must never be mistaken for metric ontology.
