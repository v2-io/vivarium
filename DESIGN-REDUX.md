# vivarium — design redux (elaboration in progress)

*Started 2026-07-01. This is a **living elaboration** of the multi-fidelity
thinking that `DESIGN.md` §"Multi-fidelity world" opens but does not develop. It
is intended to **eventually supersede** `DESIGN.md`, but does not yet — until it
is complete and adopted, `DESIGN.md` remains authoritative and this is the place
where the harder thinking is worked out. Epistemic status is marked inline:
**established** (a real, mature field), **our stance** (a design commitment we've
reasoned to), **open** (genuinely unsolved / research-flavored). Prior-art names
are field-level and confident; specific author/year attributions should be
verified before any external citation.*

---

## 0. The one principle

**Spend representation — pixels, bits, precision, simulation, cognition — only
where it has consequence, and make the representation carry its own honesty about
what is exact versus approximate.**

This is not a performance trick bolted onto a "real" world model; it *is* the
world model. It descends from the oldest thread in the project — the
`neworld` (2009) *Vision-and-Philosophy* manifesto, which observed that complex
graphics "confine the space of ideas that can be easily implemented within that
graphical style." The lesson generalizes past graphics: **any representation
that is more detailed than its consequences warrant taxes two budgets at once —
the *compute* budget and the *design/creative* budget.** Cheap-where-it-doesn't-
matter is what lets depth and emergence inherit the room that abstraction frees.
(This is why the renderer must stay a disposable *view* behind the core/view
wall — see the workspace `Cargo.toml`. A rigid, expensive view would do to
vivarium exactly what the manifesto warns of.)

The rest of this document is that principle pushed through the practical lenses.

---

## 1. The fidelity invariant, restated and widened

`DESIGN.md` states it once: **fidelity is lazily materialized, and any
materialization must be statistically consistent with the abstraction it
replaces, with known, bounded deficiencies.** *(established as the project's core
commitment.)*

The widening: it is **one invariant over four substrates**, not four problems.

| substrate | "detail where it matters" | driven by |
|---|---|---|
| display geometry | near the observer | spatial distance |
| world simulation | recently, and near agents | spatial + temporal distance |
| numerics / precision | where values live and where error compounds | consequence of the number |
| cognition | at decision-theoretic junctures | agent aporia (the two-layer mind) |

Naming it once is the point: every LOD, precision, and cognitive-swap decision
should cite this invariant and its "bounded deficiencies" clause, so consistency
across tiers is designed in, not retrofitted (retrofitting it is brutal).

**Co-fidelity corollary** *(added 2026-07-03, from the first live violation)*:
the invariant as stated is coarse↔fine *within* one aspect. Its first violation
in practice was *across* aspects: terrain painted to L24 detail while water
simulated on the L21 bed — a consumer (the view) reading the two jointly
manufactured artifacts the sim never contained (level pools shredded into
"bubbles" at the painted-noise wavelength). The rule that resolved it:
**detail must be earned, not decorated — the finest *simulated* tier is
authoritative, and painted (coordinate-noise) detail yields to any simulated
field that overlaps it.** All aspects queried at a place must be mutually
consistent at the coarsest of their materialized resolutions.

---

## 2. When to be reductionist — and when you can't (three cases, not one)

Preferred default: **fine-grained reductionist simulation with all macro
behaviour emergent.** But there are three distinct reasons to fall back to an
imposed/abstract model, and they demand *different* machinery — conflating them
hides which mechanism a given aspect needs:

1. **Laws unknown.** No trustworthy micro-model exists (ecosystem dynamics,
   anything consciousness-adjacent). → You must *impose* a phenomenological macro
   model and be honest that it is imposed.
2. **Laws known but intractable at fine grain.** We have the physics; we cannot
   afford it everywhere (erosion, fluid dynamics from first principles over a
   continent). → Run fine **only** in the observed/active locus; a cheap
   surrogate elsewhere. *This is most of vivarium.*
3. **Laws known and affordable but not yet validated** to integrate to the right
   macro. → You need the reconciliation harness (§5) *before* you trust the
   emergence.

The interesting failures will cluster at (3): the model runs, looks plausible,
and silently fails to reproduce the macro it replaced.

### 2b. The reconciliation harness, concretized: regime probes *(added 2026-07-03, empirically grounded)*

The harness case (3) demands existed for one day before proving itself. The
2026-07-02 "travelling blob" hunt (multi-metre water solitons winding down
channels) was won not by staring at the render but by an **instrument asserting
a falsifiable physical invariant per flow regime**: *subcritical flow must be
smooth* (`crates/vivarium-world/examples/channel_profile.rs`). The subcritical
control passing acquitted the kernel's core; the supercritical case failing
localized the bug to three missing physical terms (θ momentum-diffusion,
sill-depth conveyance, Fr-capped breaking — de Almeida & Bates 2013; Grant
1997). Siblings: `spike_probe` (erosion spire instability), `topo` (prior
slope statistics).

**The methodology (Joseph's TDD framing):** every aspect rung ships with
**regime probes** — cheap, renderer-free instruments asserting invariants
nature guarantees in a known regime — and **known issues get their probe
written first**, then the physics is fixed until the probe passes. This is
domain-level test-driven development, and it keeps us *strictly honest at the
seams*, where artifacts hide best. First named seam invariant, currently
failing in the wild: **the differential-aging ridge** — where localized erosion
has run more epochs than an adjacent region, a physically-implausible ridge
can develop along the boundary; a cross-seam slope/curvature-continuity probe
makes that seam's quality measurable instead of anecdotal.

---

## 3. Two axes of level-of-detail

LOD here is not only a *display* concern — it is also *how much simulation has
been run*. Both axes drive both.

- **Spatial distance** (from the observing pawn) → display resolution, *and* how
  many iterations of geology/hydrology/etc. have been materialized there.
  **Caveat (open):** unlike display subsampling, *simulation* LOD does not
  decompose cleanly by region, because many processes are **non-local** — erosion
  depends on the entire upstream watershed. So "iterate this tile to LOD-N" is
  entangled with its neighbours. This is the same non-locality wall the
  architecture audit hit; it is real and unsolved-in-general.
- **Temporal distance** (from the player's "now") → deep past simulated at
  epoch/coarse scale; the recent past re-simulated at finer time- *and* area-
  resolution. (e.g. tectonics at Myr steps deep-time; a 32 m² grid at
  1 iteration ≈ 1 year for the last few epochs.) Time is the *safer* axis to
  coarsen along — see §6.

**The two axes are one gradient for *local* aspects** *(our stance)*. Distance
from interest is really "how far the lazy computation has been pulled" (§11), and
that is *simultaneously* how-fine and how-far-in-sim-time: far = un-pulled = coarse
**and** early (the raw prior); near = fully pulled = fine **and** now. So a world
naturally "ages toward the pawn" — a wavefront of *now* it drags around, with
unvisited regions left young (fine, if never observed). Three regimes govern it:
- **Global aspects (tectonics, climate, sea level) — the exception: cannot be
  time-lagged by distance.** A plate is one planet-spanning object; you can't hold
  half of it in the past. The global spine advances **time-uniformly** (cheap,
  because coarse).
- **History-*free* local aspects** (soil, vegetation, minor relief) — not "behind
  in time," just **un-materialized**: approach evaluates the lazy function at the
  *current* macro + coordinate noise (§10). No replay; "far = raw Perlin" means
  "not yet materialized from the current macro."
- **History-*dependent* local aspects** (a specific carved canyon, a dune field) —
  genuinely at an earlier sim-time; approach triggers a **deterministic catch-up**
  (fast-forward under the time-uniform global forcings), a bounded, memoizable
  temporal-LOD "pop." Reconciling the caught-up state with the *current* macro is
  the §5↔§6 **detail→abstract** problem — i.e. this regime *is* the open frontier.

- **Equilibrium (attractor-seeking) local aspects** *(added 2026-07-03)* — a
  fourth regime the water system forced: history-free *in principle* (steady
  state is a function of the current macro + forcing) but the function is
  **implicit** — often it can only be *relaxed to*; where a direct solve
  exists, prefer it (first planned instance: the analytic hydrological
  initialization replacing the deluge fill —
  `ref/erosion-port/NOTES.md` §Next). Water surfaces, soil moisture, temperature profiles,
  climax vegetation live here. Cheaper than history-dependent replay, dearer
  than lazy evaluation — and it makes reversion nearly free (see §6).

Temporally-different neighbours stay consistent because terrain is quasi-static on
the lag timescale (the same multirate assumption): the lag just scales with
distance so `lag × local-rate-of-change < coupling tolerance`, and locality means
nothing near couples to the raw-prior far side.

---

## 4. Multi-aspect, multi-timescale: coupled multirate, not one grid

The instinct to "settle for one time/area gradation that works for all aspects"
is the **right worry and the wrong answer.** The aspects have wildly different
natural scales — tectonics (Myr), glaciation (10–100 Kyr), erosion (Kyr),
soil/nitrogen (years), hydrology (days), weather (hours), ecology
(years–decades). Forcing them onto a single grid distorts most of them.

**Our stance:** couple them **multirate**, each aspect on its own step —
*fast processes see slow ones as quasi-static; slow processes see fast ones as
time-averaged (aggregated) forcings*, exchanged on a defined coupling schedule.
This is not new to us: it *is* the load-bearing hydrology lesson already banked in
`CLAUDE.md` — "**separate the timescales**… do not couple them on one timestep" —
generalized from two aspects to N.

Where a process is run coarser than its native model, the sub-grid effects are
represented statistically — **sub-grid parameterization** — which is principled
**iff** the parameterization carries a bounded, *known* error (loops to §9's
ubit). Yes, this can mean "adjusting known system-model results"; that is
expected and legitimate when the error is tracked.

*Established prior art to stand on:* **multiscale modeling** — Adaptive Mesh
Refinement (CFD/astrophysics), the Heterogeneous Multiscale Method (E &
Engquist), equation-free/coarse-graining (Kevrekidis); and **Earth-system
modeling** — coupled sub-models exchanging fluxes through a coupler, each on its
own timestep. The forward direction and the coupling are decades mature. Only
§6 is genuinely open.

---

## 5. The seam is a *sufficient statistic*, not a number

When a region resolves macro→micro, the fine detail must **prove the macro true**
(the fidelity invariant, made operational). But "the median of the 8 m voxels
equals the 32 m elevation" only works once you decide **which statistic the macro
stored** — and you cannot preserve all of them cheaply:

- **mean** loses the spire; **max** loses the valley; neither conserves water
  volume; **variance/roughness** is what slope-stability needs.

So the real design decision is **which invariants must survive downscaling**, and
it is **consumer-dependent**: hydrology needs *conserved totals*, line-of-sight
needs *max*, display needs *mean*, geomorphology needs *variance*.

*Worked example (2026-07-03, found the hard way twice):* **water surface
interpolates; water depth does not.** The surface (bed + depth) is a
continuous field — bilinear reading is honest at any scale. Depth is a
DIFFERENCE of two fields with different smoothness (continuous surface minus
rough fine terrain), and differences do not survive interpolation: on a bank
inside a pool, neighbourhood-average depth reads 0.4 m while the surface
stands 3 m over the ground. Fine-scale consumers (the renderer; a 2 m pawn's
buoyancy) must reconstruct depth LOCALLY as surface-minus-local-ground.
Using interpolated depth produced the "bubbles" render artifact first and the
bottom-walking pawn second — same mistake, two costumes. Therefore a
macro cell honestly wants to be not a scalar but **`{mean, min, max, conserved
totals, …}` — an interval-plus-moments, with a flag marking which invariants are
*guaranteed* vs. *approximate*.** That flag is the ubit (§9) applied to terrain.
This is what the "range vs max vs min" intuition is reaching for, and it is why
the numerics question (posits) and the LOD question are the same question.

---

## 6. Reversion — forgetting, done honestly (the open frontier)

The reverse of materialization: when interest moves away, a fully-fleshed region
should be allowed to **revert to a cheaper abstraction**. Upscaling the
*dynamics* here has real prior art (HMM, superparameterization, two-way grid
nesting; and **Vandenbulcke & Barth 2019**, *Upscaling of a local model into a
larger-scale model* — EnKF pseudo-observations, whose "representativity error" is
precisely our §5 sufficient-statistics choice). The genuinely thin part is
narrower, and it is the audit's hard **detail→abstract** direction (see the
"open residue" below).

**Our stance — reversion is cache eviction with regeneration on miss.** Split the
fine state into:
- (i) the part **derivable from seed + macro** — safe to evict, regenerate on
  return; and
- (ii) the **irreducible residual** — agent edits, placed structures, history —
  which *must* be kept.

That split is already the core's shape (**seed + sparse edits**). From it, the
distance-vs-time question resolves rigorously: **evict by probability of
return.**
- **Space** near the player is *reversible* (turn around → high return
  probability) → **keep**. Evicting on spatial distance alone is unwise — it
  risks pop and pays regeneration cost on a likely return.
- **Deep past** is *monotone* (zero return probability) → **forget freely.**

So: **forget along the irreversible axis (time); conserve along the reversible
axis (space).** The instinct was right; this is the reason.

*(2026-07-03)* The §3 equilibrium regime sharpens the split: **attractor-seeking
aspects are evictable almost for free** — their state regenerates by
re-relaxation from the (kept) macro, no history required; only genuinely
path-dependent aspects (erosion's carved record) need checkpoints. A living
region the pawn left does not need "what the river did while we were gone"
stored — re-settling against the absorbed bed *is* the honest regeneration,
provided the macro received the sediment deltas (the §5↔§6 tie again).

**The ratchet is real but slowable.** As irreducible residual accumulates
(permanent structures, long-lived objects, history), the must-keep set grows
monotonically → storage grows (typical of the genre). But the same principle
recurses onto the residual itself: **apply temporal-LOD to the accumulated
history** — a 500-year-old ruin can be stored coarser than yesterday's dig.

**The open residue (narrower than it first looks):** upscaling continuous
*dynamics* is mature (above). What is *not* solved anywhere we've found is
upscaling **irreducible discrete edits** — a dammed stream — into a
**content-addressed, memoized** macro *with correct up-invalidation*: the change
must alter the macro hydrology after the fine locus collapses, and only works if
the macro stored the **right sufficient statistics (§5)** to receive it. Get the
statistics wrong and reversion is lossy in a way that *silently corrupts the
macro*. This §5↔§6 tie — discrete edits + memoized up-invalidation — is the
single sharpest open problem in this document.

---

## 7. The four seams (a maturity map)

"The seams are the big question" is truer as: there are ~four seam *types*, and
only one is open — which is far more encouraging.

1. **Spatial seam** (LOD boundary, one instant) — no cracks/pops + statistical
   match. *Established* (geomorphing, mesh stitching).
2. **Temporal downscaling seam** (macro-epoch → fine-epoch handoff) — the fine
   sim's initial condition must be a valid downscaling of the macro end-state.
   *Established-ish*: this is regional-climate-model nesting, with a **known
   artifact** — spin-up transients near the boundary.
3. **Aspect-coupling seam** (geology↔hydrology↔ecology↔…) — flux exchange on a
   schedule. *Established* (Earth-system couplers; §4).
4. **Reversion / upscaling seam** (fine + edits → macro) — dynamics-upscaling is
   *mature* (HMM, two-way nesting, Vandenbulcke & Barth 2019); the **open** residue
   is discrete-edit up-invalidation into a memoized macro (= §6, = detail→abstract).

Lean on prior art for 1–3 *and* the dynamics half of 4; only the discrete-edit
up-invalidation in 4 is genuinely research.

---

## 8. Honest stochasticity — jitter baked into the coordinates

**Artificial neatness is itself a dishonesty.** Real phenomena carry jitter, and
often heavy-tailed / fractal distributions; a perfectly smooth or regular field
is a *lie* about the world. So honesty *requires* deliberate perturbation, not
just permits it.

**Our stance — all stochasticity is a stateless function of a stable key, never a
draw from a shared mutable stream.** Rather than storing jitter (naive, and it
does not scale) or pulling from a sequential PRNG (which couples order to result
and breaks under parallelism), derive it by **hashing the thing's own
coordinates/identity**:

```
offset = jitter(field_tag, x, y, z, …)   // → deterministic value in a range
```

This is "a world of pseudo-random distributions baked into the coordinates
themselves," and it reconciles three goals at once:
- **honesty** — the field is as irregular as we ask it to be;
- **determinism** — same key ⇒ same value, forever, no stored state;
- **parallel-safety** — no shared mutable RNG; any tile/voxel resolves
  independently, in any order, on any thread.

This is the **world-side of the exact principle the audit flagged on the agent
side.** The audit's #1 finding was that the agents' shared, sequentially-stepped
`World.rng` breaks bit-identical replay under parallel scheduling; the fix there
is *per-agent splittable seeds*. Same family: **every source of randomness derives
from a stable key via a stateless, well-mixed hash** — coordinates for the static
world, `(agent_id, tick, purpose)` for agent decisions. One rule; it is the
computational spine of determinism-as-ontology.

Two correctness notes (these are *correctness*, not polish, because a weak choice
reintroduces the very dishonesty we're removing):
- **Hash quality matters.** Naive `x·P mod Q` schemes have axis-aligned
  correlations — visible lattice artifacts, i.e. *artificial structure*. Use a
  strong avalanche finalizer (SplitMix64's mix, or a murmur3-style `fmix`):
  fold `(field_tag, x, y, z, seed)` together with distinct large odd constants,
  finalize, take the high bits → `[0,1)`, stretch to `[min,max]`. The
  `field_tag` gives infinitely many *independent* fields for free (elevation
  jitter, colour jitter, vegetation placement, …) from one function.
- **The distribution must match the phenomenon.** Uniform jitter is rarely the
  honest choice: micro-relief is **fractal** (FBM of the hash across octaves);
  scatter around a measurement is **Gaussian**; object placement wants
  **blue-noise** (uniform placement clumps *and* over-evens in different ways,
  both dishonest). Choosing the wrong distribution swaps one artificial neatness
  for another.

This also underwrites §5: the sub-voxel truth a macro cell abstracts *is* a
coordinate-keyed distribution, so a macro cell's `{mean,min,max,var}` can be made
**exactly** consistent with the fine field it will later materialize — the fine
detail is a deterministic function of the same coordinates, not an independent
roll that might contradict the stored statistics.

---

## 9. Numerics — precision by consequence, error made honest

The numeric substrate of §0. Unums/posits (Gustafson, *The End of Error*) offer
**tapered precision** (more mantissa where values cluster, near magnitude 1) and,
via the ubit, an **explicit exact-vs-interval flag**. *(established as a
technique; adoption in vivarium is our stance / not yet decided.)*

Where they would earn their keep here — and where plain `f32` is fine:
- **Elevation range** does *not* need them (f32 resolves ~2 mm at a 20 km world).
- **Accumulated sim dynamics** (erosion/sediment/flow summed over many
  iterations) — a quire (exact wide accumulator) makes long sums exact and
  reproducible.
- **Cross-machine determinism** — a single rounding mode and no NaN variance is
  aligned with determinism-as-ontology (and the audit's inferred, unmeasured
  cross-platform float concern).
- **The interval-carrying macro cell (§5)** — the ubit *is* the "guaranteed vs.
  approximate" flag the seam needs.

*(2026-07-03 — first §9-style call made in code before this section was adopted:
the water sim keeps its conserved reservoirs in `f64`, field arrays in `f32`,
with conservation asserted by test as the sentinel. Precision by consequence,
practiced.)*

The through-line: the ubit is the computational analog of the project's epistemic
discipline — *mark the guess as a guess.* A world-model whose values know whether
they are **measured (exact)** or **plausibly filled (interval)** is truth-honoring
made mechanical. (Rust crate to evaluate: `softposit`; a crate literally named
`fast-posit` was mentioned but I could not confirm it from memory — verify before
relying on it.)

---

## 10. Lineage and prior-art coordinates

**Roots (`~/src/neworld`, 2009)** — this is not a fresh idea; three threads are
sixteen years old:
- *Vision-and-Philosophy* — the telos in §0 (representation must serve depth, not
  confine it; "bad causation" as artistic dishonesty → truth-honoring).
- *Column-data* — embryonic relevance-compression geometry ("length of bare
  column… reduces empty space"; per-column vertices + face slopes).
- *Orc-simulation* — the agent-layer seed (observe→actor→order loop; artificial
  life; evolution) — the two-layer mind, early.

**Fields to read (by section):** multiscale modeling — AMR, HMM (E & Engquist),
equation-free (Kevrekidis) [§2,4]; Earth-system / coupled climate modeling and
flux couplers [§4]; regional-climate dynamical downscaling & spin-up artifacts
[§7.2]; sub-grid parameterization [§4]; procedural hashing / value noise and
blue-noise sampling [§8]; unum/posit computing, Gustafson [§9]. Modern
physically-based large-scale terrain (Galin/Cordonnier/Guerrero, ~2016–20) is the
natural next erosion reading and fits the principled-from-physics ethos. For the
reversion frontier [§6]: **Vandenbulcke & Barth 2019**, *Upscaling of a local
model into a larger-scale model* — the closest prior art found to our reversion
problem (EnKF pseudo-observations; their "representativity error" ≈ our §5).

---

## 11. The runtime as a lazy query graph

The operational form of everything above: **the whole runtime is a demand-driven,
memoized query graph.** A query is `(aspect, region, resolution, time)`; a pull
recurses into the queries it depends on; results memoize by key; nothing computes
until demanded and nothing computes twice. *(established paradigm.)*

This is not novel architecture — it is the shape of modern build systems (Nix,
Bazel, Shake; formalized in *Build Systems à la Carte*, Mokhov/Mitchell/Peyton
Jones) and query-based compilers (**Salsa**, which powers rust-analyzer;
**Adapton**; Umut Acar's **self-adjusting computation**). **vivarium is already
this for its base tier** — seed→FBM is a pure memoized function; edits are a
sparse overlay. The whole redux is one sentence in this frame: *extend the
pure-lazy-memoized property the base tier already has, up through the eroded and
coupled tiers.* The audit's eagerly-baked ±6 km patch is exactly where it breaks.

The pawn at (0,0), epoch E, year 1,100,000 poses a query at the finest grain; the
pull recurses down the coupling/abstraction stack — pulling the coarse global
tectonic history it needs (memoized once, cached forever) — until it has what it
takes to feed the fine locus. All pull-based, lazy, streamed, memoized. **This is
the true north; every mechanism below is measured by how close it gets.**

### Where the clean graph meets this world — four seams

1. **The dependency cone isn't local for global aspects.** Fine erosion under the
   pawn pulls a bounded cone; tectonics and climate are *planet-spanning* — no
   local uplift without the global plate configuration. So the recursion fans
   *out* to global-coarse, then *in* to local-fine. Cheap (coarse global is
   small), but "just enough for under the pawn" holds only for *local* aspects.
2. **Coupling makes it cyclic; time makes it a DAG.** Aspects are mutually
   dependent (water↔climate↔geology); a pull-graph must be acyclic. The cut is
   §4's lagged coupling: A at *t* depends on B at *t−Δ*, never *t*. The real graph
   is a DAG over `(aspect, region, resolution, timestep)` — **time breaks every
   cycle** — so the memo key must include time, and the checkpoint chain is part
   of the graph. "Runs tectonics until it's at the right resolution" is a
   *memoized chain of timesteps*: expensive first pull, cached after.
3. **The generative half memoizes cleanly; mutation is the frontier.** Seed→world
   is a pure function of immutable inputs — memoization never invalidates. An
   agent *edit* (dam a stream) must invalidate a non-local downstream cone **and**
   propagate *up* into the abstraction (§5↔§6). Salsa/Adapton do downstream
   invalidation; up-propagation into abstractions is beyond them — the open part.
4. **Cold-start is real.** The first pull at year 1,100,000 with an empty cache
   pays the full coarse global history. Mitigated — not eliminated — by
   coarse-first (a cheap global spine at world creation, refined on demand) plus
   epoch checkpoints.

### Predictive prefetch — the safe layer
Pure optimization over a correct lazy base: a wrong guess wastes spare cycles, a
right one hides latency, and it can *never* cause a correctness bug. So it is
cost-aware caching (Belady-optimal as the unreachable ideal; prefetch the highest
`probability × latency-saved − cost` along the pawn's likely trajectory). Build it
last, enjoy it freely.

### The graceful-degradation decomposition
The fully-general version (non-local + coupled + mutable + all aspects as one pure
graph) is not tractable as a single graph today — but it degrades into three
scoped pieces, and *that decomposition is the actual architecture*:
- a **precomputed coarse-global spine** (planet-spanning aspects at low res);
- **lazy memoized local cones** (fine detail pulled on demand near the pawn);
- a **research-grade edit-propagation layer** (§6 — the open part).

---

## 12. The fidelity ladder — start crude, sharpen any layer later

This is where the query graph pays off as *methodology*, not just performance.

**Most layers can start at a low-fidelity, macro-statistic-matching model, and
that is not a compromise — it is correct, provided the crude model honours the
sufficient statistics its consumers need (§5).** Layers we expect to be fine with
for a long time:
- initial unformed land as **Perlin/FBM**, not early-Earth pre-tectonic
  fluid/gravity/chemistry;
- conserving **mineral totals** rather than element totals; later, element totals
  rather than modelling decay;
- a **simple water cycle**; a **single material hardness** for erosion (today).

So each aspect has a **ladder of models** — crude rung → higher-fidelity rung —
and you may occupy any rung. The fidelity invariant (§1) is what makes a low rung
*honest* rather than a lie: it must reproduce the macro statistics (§5) its
consumers read, with known bounded error (§9's ubit).

### The ladder runs both ways *(Joseph, 2026-07-03)*
Rungs are usually climbed toward more physics — but the economically crucial
move is the DESCENT: climb stepwise-emergent to *discover* a system's
behaviour; once its patterns are characterized and probe-validated, descend to
a tight procedural surrogate that reproduces the discovered statistics (§5
agreement is the honesty gate), keeping the expensive rung as calibrator —
re-run on recipe change, feeding the surrogate's calibration. Insolation
already lives at the bottom (closed-form); weather should end there; Abyssal
geology never needs to descend because checkpoints amortize it to zero. Every
system-in-a-phase declares its execution class (batch-deep / relaxation /
procedural-tight) in its recipe — see
`ref/research/architecture-migration-2026-07-03.md`.

### The payoff: swap a model, rerun only what changed
The requirement that makes the ladder practical: **memoize each aspect's progress
to disk, keyed so that the model's own identity and version are part of the key —
not just its data inputs.** This is the Nix insight (a derivation's hash includes
its *recipe*, so changing the recipe rebuilds exactly that and its dependents).
Then swapping erosion-v1 (single hardness) for erosion-v2 (layered hardness)
changes erosion's key ⇒ **its cached outputs and everything downstream invalidate;
everything else stays cached. No more, no less.** Independent aspects are
untouched; the rerun is minimal *by construction*.

**Accelerated backends are rungs too** *(2026-07-03)*: a GPU implementation of
a kernel is a swappable rung behind the same flux interface — with the CPU
kernel kept forever as the *reference implementation* (canon for tests,
replay, probes), backend identity in the recipe key (GPU bit-reproducibility
is per-device, not universal), and agreement validated by probe within a
written tolerance. Plan: `ref/research/water-parallelism.md`.

**Correctness discipline (this is where it's actually hard):**
- **The key must capture every input that affects the output** — upstream data,
  the model identity+version, coupling parameters, the seed. *Under*-keying ⇒
  stale cache ⇒ silent corruption (the classic build-cache bug); *over*-keying ⇒
  spurious recompute. Key completeness is a *correctness* property, not tidiness.
- **Every model must be deterministic in its keyed inputs** for the cache to be
  sound — which is exactly why §8 (no hidden RNG; all stochasticity is a stateless
  hash of a stable key) is a *precondition* here, not a separate nicety.

### The coupler is the stable seam; models behind it swap freely
"Fluxing through a coupler, lazily" is the Heterogeneous Multiscale Method made
lazy, and it resolves cleanly here:
- The **coupler defines the interface** — the sufficient-statistics fluxed between
  aspects at `t−Δ` (§4). It is the *stable seam*.
- Each aspect's internal model is a swappable implementation *behind* that
  interface. Erosion v1→v2 is free **as long as it still produces the coupler's
  fluxed quantities.**
- A higher-fidelity model often *enriches* the flux (erosion-v2 can flux sediment
  grain-size, not just volume). That is a **new** interface quantity: consumers
  that read grain-size get a new input ⇒ their keys change ⇒ they rerun; consumers
  that don't are untouched. So **keep the flux interface fine-grained
  (per-quantity), never a monolithic blob**, or a model upgrade over-invalidates.
- The macro solver pulls "effective flux" from the micro model on demand and
  memoizes it — HMM's upscaling, computed lazily, cached, reused until the micro
  model or its inputs change.

**The whole document in one image:** *a lattice of swappable,
independently-versioned models, coupled through fine-grained flux interfaces,
evaluated lazily and memoized by complete content-addressed keys — so the world
can start crude everywhere and have any single layer sharpened later at the cost
of exactly the recomputation that layer forces, and no more.*

---

## 13. Storage and save-format (they are the same thing)

The memo store is **not a disposable system cache — it is the world's portable
saved state.** A world evolved and played in a region must be pick-up-able on
another machine without replaying its evolution *or* its user mutations.

**Shipping materialized state is a correctness hedge, not just a speed-up**
*(our stance)*. Determinism means the regenerable part *could* be recomputed —
but a user mutation is bound to the substrate it was made against; regenerating
that substrate under a *newer algorithm version* yields a different world (a
tunnel through what is now solid rock). So a robust save ships the substrate its
mutations depend on, and pins the algo-versions they assume. Determinism doesn't
make shipping-state unnecessary; it makes it **optional and safe** — the
pure-function property is exactly what lets us choose how much to ship.

**Shape** *(our stance)* — a git-repo-shaped, self-contained directory:
```
<world-save>/
  manifest        # store-format-version, world-seed, provenance, pinned algo-versions
  objects/<hash>  # immutable, content-addressed results (§12)
  roots           # (aspect, region, tier, time) -> current hash
  mutations/      # the user-mutation log — first-class (see below)
```
Copy the folder → the world moves, fully usable. Only the **store-format-version**
sits in the path (byte-format compatibility); *algorithm* versions live inside each
object's key (§12), so an app release never invalidates the store. Content-
addressing gives dedup and merge/sync-by-copying-missing-objects (git-fetch-
shaped) — which quietly foreshadows shared/multiplayer worlds.

**The regenerable/irreducible split** *(TENTATIVE — reserve the shape, do not
over-prescribe; future us will have far more tactical wisdom here than we do now)*.
The appealing idea: tag each object by whether it is derivable from seed + macro
(*regenerable*) or its lineage touches a user mutation (*irreducible*). If that bit
holds up, it would buy (a) GC that prunes regenerable objects freely — including
*truly deleting* them on an algo-version bump — while never touching irreducible
ones, and (b) **tunable save size**: a *thin* save (recipe + irreducible + roots,
regenerate the rest) vs. a *fat* save (also ship materialized regenerable state,
for instant and version-drift-robust pickup), with a natural "fat for developed
regions, thin elsewhere" policy. **What is deliberately unfixed:** the exact split,
how the bit is derived, and the GC/eviction policy — settle these against real
usage, not now.

**The mutation log is the primary artifact** *(shape reserved; full design
deferred)*. User mutations (dug tunnels, built structures, history) are the
irreducible truth and will become the larger part of a save over time. The format
should treat the log as first-class — append-only, ordered, keyed by
(region, time), pinning the algo-versions it assumes — with the regenerable object
store as its *supporting cache*, not the reverse. The detail→abstract frontier
(§6) lives right here: absorbing a mutation's effect back into the macro is the
open problem, and the save-format is where it will have to be honest.

**Invalidation vs. eviction** *(established discipline, from §12)*. Invalidation is
*correctness* — by content hash, never by OS mtime/freshness (timestamp-based
staleness is why `make` is fragile and Nix is not). Eviction is *space* — there the
OS/atime LRU is welcome, because content-addressed blobs are immutable, so deleting
one costs only a recompute, never correctness. The ratchet (§6) makes saves grow;
the regenerable/irreducible split (once settled) is what keeps that growth *tunable*
rather than fatal.

---

## 14. Representations of matter, and what a cell value *means* (TENTATIVE)

*Everything in §14 is **TENTATIVE** — reserve the shapes, do not fix them; this is
exactly the kind of thing future us will have far more tactical wisdom for. Marked
so it cannot harden into false certainty.*

### Three representations of matter, chosen by consequence

"Voxel" is not inherently cubic — imaging routinely uses anisotropic voxels; we
*choose* isotropic 0.5 m cubes for the true-scale anchor. But a cube is the wrong
unit for much of what matter does. Likely **three** representations, each matched
to a phenomenon's structure (§0, now applied to the substrate itself):

| representation | for | shape |
|---|---|---|
| **stratum** (bed/layer) | vertically-coherent material (sediment, soil horizons) | run-length interval, **real-valued thickness** — "10.5 m of sand" is *one* stratum, fractional for free |
| **voxel** | adjacency algorithms, rendering, per-cell edits | cubic 0.5 m cell — a *materialized view*, not necessarily the storage |
| **body** (intrusion / vein / cave / boulder / structure) | cross-cutting masses with their own shape | discrete 3D object, **overlaid** on the substrate |

Keep the nouns distinct (stretching "voxel" to cover all of them is the confusion
to avoid): **column** (vertical unit at a tile), **voxel** (cubic materialized
cell), **stratum/bed** (a material run), **body/intrusion** (a cross-cutting mass).
Geology already names the bodies precisely — *dike* (discordant sheet), *sill*
(concordant sheet), *pluton/batholith* (massive; how granite forms). This is the
same **base field + sparse overlay** already in core: strata are the substrate,
bodies are the overlay that overrides it — the geological cousin of a sparse edit
or a user mutation.

### What a stored cell value *means* must be declared — it is plural

The trap: "elevation 8000" means different things to erosion, hydrology,
rendering, and collision, and each will silently assume its own. Three honest
readings, all legitimate for different consumers:

- **center point-sample** — a *finite-difference* node; what a heightfield mesh
  renders (the slabs point-mesh already commits to this — its vertices are column
  centers). Lossy about sub-cell extremes; does **not** conserve volume.
- **mean (area statistic)** — a *finite-volume* cell integral; conserves
  volume/mass (mean × area) — what erosion/hydrology budgets need. Loses the peak.
- **min / max** — conservative bounds; **max** for line-of-sight/occlusion/
  collision-ceiling, **min** for "fully above water." Cheap; they bound how much
  the point-sample lies.

The reframe that dissolves the ambiguity: at voxel scale the honest primitive is
not a *height* but the **volume/mass of material in the cell**, plus the **fill
fraction of the top voxel** (so 10.5 is exact and the surface is sub-voxel — this
is Volume-of-Fluid). **Elevation is *derived*** from that (top of the topmost
solid).

*Tentative recommendation:* make **volume/mass the conserved primitive** (per
column/stratum); **declare** surface elevation as top-of-topmost-solid, *center
point-sample* (matching the renderer, so nothing silently assumes mean/max); carry
**min/max** alongside — a §5 sufficient-statistic tuple, each field tagged with its
exactness (§9), never a bare scalar. Converting between these readings (volume↔
height, sample↔mean) is lossy and directional, so it is **coupler** work (§4/§12);
and a downscale must honor whichever statistic was stored (stored mean ⇒ volume
conserved; stored max ⇒ peak survives). Store the wrong statistic and the fine
materialization cannot serve its consumer — the §5↔§6 silent corruption.

*Prior art (this is not novel):* finite-volume vs finite-difference (the core
fork); GDAL's `AREA_OR_POINT` and DEM grid registration (a cell value's meaning
must be declared); Arakawa grid staggering (quantities at centers vs edges);
Volume-of-Fluid (Hirt & Nichols) for fractional cells; marching cubes / dual
contouring for sub-voxel surfaces.

---

## 15. The material property set (material model as a fidelity ladder)

*Grounded in an adversarially-verified deep-research survey (Dwarf Fortress
material tokens; Mohr–Coulomb / infinite-slope geotechnics; SHALSTAB; shallow-water
+ stream-power erosion; μ(I) granular rheology; DEM). The **spanning claim and the
additions are research-backed**; the exact storage schema and initial rungs stay
TENTATIVE (§14 discipline).*

The material model is its own fidelity ladder (§12), and the property set below is
the **stable coupler interface** every rung fills — so the model swaps without
touching consumers. The survey confirmed the list already implied by core is
**very nearly spanning** (a real validation of the params you'd stowed:
saturation, porosity, channel-ness, flow-direction). Each property tagged by where
it lives; **(NEW)** = surfaced by the survey as missing:

**Per-material-type** (static, keyed by material):
- **density** — but **split solid vs liquid** (a material carries both) **(NEW)** [kg/m³]
- **cohesion** `C` [Pa] and **internal friction angle** `φ` [deg] — Mohr–Coulomb
  `τ = C + σ′·tan φ`; for cohesionless material the **angle of repose *is* `φ`**
  (what makes sand pile and slump).
- **grain size** `d` [m]
- **porosity / permeability**, plus **packing / volume fraction** `φ_pack` **(NEW)**
  (distinct from porosity; governs granular behaviour and the top-voxel fill, §14).
- **phase-state enum** — SOLID / LIQUID / GAS / POWDER / PASTE **(NEW)** — DF's move,
  and the *discrete* form of "fluidity": it governs falls-vs-flows-vs-holds and is
  the crude rung's entire rheology.
- **erodibility** `K` + an **explicit incision threshold** **(NEW)** — stream power
  `I = K·Aᵐ·Sⁿ` above the threshold, nothing below.

**Per-column / per-cell dynamic state** (time-varying):
- **saturation / pore-water pressure** `u` — cuts strength via effective stress
  `σ′ = σ − u`; slope-parallel seepage roughly *halves* the factor-of-safety (the
  dominant landslide trigger).
- **water depth** `d` and **mobile-sediment / regolith thickness** `r` **(NEW)** —
  the loose material available to move, distinct from bedrock; the shallow-water /
  erosion state is just the trio `(b=terrain, d=water, r=regolith)`.
- **temperature** (future: freeze/thaw).

**Derived-geometric** (from the surface field, not stored): surface height (center
point-sample, §14), **slope** `β`, **overburden/normal stress** `σ = ∫ρg dz`.

**The ladder the survey confirmed:**
1. **block/tile (crude):** material = discrete type + phase enum + a scalar strength.
   (DF actually stores six stress-modes × yield/fracture/strain — over-fidelity for
   us; a scalar suffices here. **Caveat: DF's strength numbers are game-scaled, not
   SI — borrow the model, not the values.**)
2. **geotechnical + hydrologic (middle):** Mohr–Coulomb factor-of-safety +
   shallow-water/stream-power erosion, over the `(b, d, r)` and `(C, φ, u)` fields.
3. **granular (high):** continuum μ(I) rheology (μ₁, μ₂, I₀; inertial number from
   `d`, grain density, confining pressure, shear rate), then DEM (per-particle
   contacts calibrated so emergent angle-of-repose ≈ `φ`).

**The load-bearing architectural validation:** **SHALSTAB** couples slope-stability
and water-routing **on the same per-cell state** — direct evidence that one
substrate's property interface serves *both* query families at once. That is the
coupler-interface bet (§4/§12) confirmed on a real coupled model: **no separate
substrate per physics.**

---

## Status — established vs. frontier (so future work knows where to lean)

- **Stand on prior art:** the fidelity invariant's forward direction (§1–2),
  spatial + temporal LOD (§3), multirate multi-aspect coupling (§4), three of the
  four seams (§7.1–7.3), coordinate-hashed stochasticity (§8), posit numerics
  (§9), the lazy-memoized query-graph runtime and content-addressed model-swap
  (§11–12), and the git/Nix-shaped content-addressed save-store (§13) — all real,
  proven architectures. *(Tentative within §13: the regenerable/irreducible split
  and GC policy, and the full mutation-log design — reserved, not fixed.)*
- **The frontier / our research:** *not* dynamics-upscaling — that is mature (HMM,
  superparameterization, two-way nesting, Vandenbulcke & Barth 2019, whose
  "representativity error" is our §5). The genuinely novel residue is narrower and
  singular: **upscaling irreducible discrete agent edits into a content-addressed,
  memoized macro with correct up-invalidation** (§5↔§6, §7.4, §11 seam 3), together
  with the non-locality of simulation-LOD (§3). One thing from several sides:
  **detail→abstract**.
- **Nearest concrete step it implies:** give the erosion tier a complete
  content-addressed memo key (inputs + model-version + seed) and pull-based
  evaluation, so it stops being an eagerly-baked fixed patch (the audit's finding)
  and becomes the first rung of a swappable ladder. That single change exercises
  §5, §8, §11, and §12 at once on a system we already have.
- **Not yet superseding `DESIGN.md`:** this is elaboration; adopt deliberately.
