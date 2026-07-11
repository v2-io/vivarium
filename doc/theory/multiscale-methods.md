# Multiscale & heterogeneous methods — the generalized frame, and where vivarium sits in it

*(Written 2026-07-03 for the lexicon effort. Purpose: the design docs cite the
field — §4 names AMR/HMM/equation-free, §11 says "HMM made lazy" — but never
teach the general structure, so each vivarium construct has looked like a
one-off invention. It isn't: almost everything we've built is an instance of a
small operator algebra that the numerical-analysis and Earth-system communities
matured over ~50 years. Knowing the general frame gives us (a) names, (b) the
invariants every instance must satisfy, (c) a sharp statement of which parts of
vivarium are genuinely novel. **Epistemic note:** the attributions below were
verified against publisher/archive pages on 2026-07-03 and every source is in
relata with a verification event — see `BIBLIOGRAPHY.md` in this directory for
keys, PDF status, and the manual-chase list. The verbatim-read pass (the reason
the PDFs exist) had not yet happened as of this writing.)*

---

## 1. The general structure — every method is four objects

Every multiscale method, whatever its name, is built from:

- **U** — the **macro state**: coarse variables (a 16 m height field, a
  drainage graph, a climate zone map).
- **u** — the **micro state**: fine variables (4 m cells, 0.5 m voxels,
  individual storms).
- **R : u → U** — **restriction** (a.k.a. compression, coarse-graining): how
  fine state summarizes upward. Many-to-one, information-destroying *by
  design*. The whole question is *which* information survives.
- **L : U → u** — **lifting** (a.k.a. reconstruction, downscaling): how coarse
  state materializes downward. One-to-many — U underdetermines u — so every L
  smuggles in a **closure**: a choice of measure over the missing detail.

Two consistency laws make the pair honest:

1. **R ∘ L = identity on U** — reconstruct then summarize must give back what
   the macro claimed. (This is exactly our *fidelity invariant* and the §5
   sufficient-statistic seam; our mean-**pinning** of fine erosion tiers is
   R∘L=id enforced on the mean, live.)
2. **Conservative exchange** — when a conserved quantity crosses a seam
   (scale-to-scale or aspect-to-aspect), exchange **fluxes, not states**, and
   the fluxes must balance exactly. Earth-system couplers stake everything on
   this; so does our water budget.

And one premise underneath it all:

3. **Scale separation** — the macro evolves slowly relative to the micro's
   equilibration (a spectral gap between timescales). Where separation holds,
   the fast process may be treated as *equilibrated* (seen from above) and the
   slow one as *frozen* (seen from below). Where it fails, no multiscale
   method saves you — you must resolve, or accept unbounded error. Our
   load-bearing hydrology lesson ("erosion is geological, water is
   hydrological — never one timestep") is this premise, learned empirically.

## 2. The method zoo — one axis: *where does the missing information come from?*

Ordered roughly by how much micro physics each keeps alive:

- **Full nesting / AMR** (Berger & Oliger 1984; CFD/astro). One model,
  locally refined mesh in space *and time* where error indicators demand;
  fine patches substep; coarse–fine interfaces reconciled conservatively
  ("refluxing"). *Vivarium:* the **telescope** is AMR with the refinement
  criterion = observer attention rather than error estimate; **finishers**
  are the fine substeps; mean-pinning is the **injection/update** (averaged
  fine→coarse). *(Corrected 2026-07-10 after the primary read: mean-pin is NOT
  refluxing — refluxing is the distinct conservative flux correction at the
  coarse–fine boundary (Berger–Colella 1989), which vivarium does not yet do;
  that gap is why `seam_ridge` is red. See `doc/theory/multiscale-seams.md` §2.1.)*
- **Multirate time integration** (Gear & Wells 1984 lineage). Split one system by
  timescale; fast components substep with interpolated slow forcings; slow
  components see time-averaged fast output. *Vivarium:* §4's stance verbatim;
  the erosion-ON-while-water-flows schedule.
- **Superparameterization** (Grabowski 2001; Khairoutdinov & Randall 2001). Inside each
  coarse cell, run a *small embedded fine model* instead of a statistical
  closure — pay micro cost only per-cell, in miniature. *Vivarium:* nothing
  yet — but it is the natural template for storm-scale weather inside climate
  cells, and for the **cognitive LOD**: a statistical crowd with a small
  embedded fully-simulated core is superparameterized society.
- **HMM — Heterogeneous Multiscale Method** (E & Engquist 2003). The macro
  solver runs everywhere; wherever it lacks constitutive data (a flux, a
  rate law), it spawns a *constrained, short, small* micro simulation on
  demand, averages the result, and continues. Micro is a subroutine, not a
  layer. *Vivarium:* §11's lazy query-graph pulling through couplers — cited
  there already as "HMM made lazy." The direction of the lazy pull
  (backwards-from-now, §3/§11) is our twist; HMM classically pulls
  forward-in-time.
- **Equation-free / coarse projective integration** (Kevrekidis et al. 2003). No
  macro equations exist at all: lift (L), run micro briefly, restrict (R),
  estimate the macro derivative numerically, take one large macro step,
  repeat. The macro model is *implicitly defined* by short micro bursts.
  *Vivarium:* the §3 **equilibrium regime** (relax-to-attractor when no
  direct solve exists) is its steady-state cousin; the deluge fill was
  equation-free relaxation, and the planned analytic init is its replacement
  by an *explicit* macro solve — the classic upgrade path.
- **Sub-grid parameterization** (all of climate/NWP). Replace the micro
  entirely with a statistical closure — fitted coefficients standing in for
  unresolved physics — legitimate **iff** the closure carries a known error
  budget. *Vivarium:* §4 says exactly this (the ubit, §9); every #mech
  stand-in (fBm-as-tectonics) is a parameterization currently *missing its
  error model* — that's what #mech honestly marks.
- **Coupled heterogeneous models + flux coupler** (Earth-system modeling).
  Different *models* per aspect (ocean, atmosphere, ice, land), each on its
  own grid and step, exchanging conservative fluxes through a coupler on a
  schedule. The coupler — regridding, averaging, lag conventions — is a
  first-class artifact, often the hardest one. *Vivarium:* the phase
  structure is this at world scale; **a phase-transition is a coupler
  interface** (`doc/PHASES.md` design notes say so); DESIGN-SYSTEMS' coupling bands are
  the schedule.
- **Multigrid** (Brandt 1977). Not model reduction — solver acceleration:
  the same equation on a grid hierarchy, using coarse grids to kill
  long-wavelength error fast. Listed to *disambiguate*: our telescope is not
  multigrid, though an analytic-init solver may well want multigrid
  internally.
- **Coarse-graining / renormalization** (statistical physics). Derive the
  effective macro *law* (not just state) by integrating out fine degrees of
  freedom. The systematic way to discover what a phase's macro variables
  even are. Mostly aspirational for us; named for completeness.

## 3. The closure problem is where the bodies are buried

L is one-to-many; every method must choose the missing measure. The standard
choices: sample it randomly (Monte Carlo micro realizations), take the
ensemble mean (loses variance — §5's exact complaint), or maximize entropy
subject to U. **Vivarium's answer is none of these, and it may be our most
distinctive move: the missing detail is a *deterministic pure function of
identity* — the §8 keyed/coordinate-hashed noise.** Lifting doesn't *sample*
a realization; it *looks up the one true realization* fated by (seed, key).
Consequences, in order of importance:

1. **Memoization becomes sound.** If L sampled, two evaluations would
   disagree and caching would change the world. Fated lifting is what lets
   the memoization architecture exist at all. (The seeding discipline and
   the caching architecture are one decision, not two — lexicon §3 should
   say so.)
2. **Reproducibility across the run-modes** (causal runs, replays, iteration
   runs) reduces to key discipline + generator pinning.
3. The cost: our "ensembles" are degenerate (one world per seed). Where we
   ever need distributional claims (calibrating a parameterization's error
   budget), we vary the seed, not the cell.

## 4. The two consistency laws, as we've already paid for them

Worth naming that both laws have already drawn blood here:

- **R∘L=id violations**: painted sub-voxel detail ("detail must be earned"),
  lakes climbing dry banks, the fine tier drifting off its parent's mean
  before pinning landed.
- **Conservative-exchange violations**: today's twin f32 finds — the bed
  deadband (an increment that must *land*) and the rain/evap bias (an
  increment that must be *counted*). Both were seam crimes at the
  *arithmetic* scale — the scale below the smallest one we'd been thinking
  about. The general lesson generalizes down: **floating-point is the
  bottom-most seam, and it needs the same conservation discipline as any
  coupler** (realized-delta accounting or compensated summation — choose
  per-site, explicitly).

## 5. What is genuinely ours vs. what is mature

**Mature, stand on it freely:** multirate coupling, AMR-style nesting,
sub-grid parameterization with error budgets, flux couplers, HMM-style
on-demand micro, relax-to-attractor.

**Our twists (defensible, but we own the risk):** lazy *backwards-from-now*
evaluation of the coupler pulls (§11); observer-driven rather than
error-driven refinement; consumer-dependent sufficient statistics (§5 — the
literature usually fixes one R per method; we want R per consumer);
fated-lookup closure (§3 above); phase-memos as content-addressed storage.

**Genuinely open (the one hard research problem, ORIENTATION):**
**detail→abstract** — an irreducible *micro edit* (an agent digs a canal)
must propagate *upward* through R into every memoized macro that summarized
it, with correct invalidation. The mature methods are all forward/downward;
upward re-summarization after micro perturbation is closest to data
assimilation (state estimation from observations), and nobody does it with
memo-invalidation semantics. This is where vivarium is off the map — and it
is also precisely the AAT question (does a perturbation stay legible), which
is presumably not a coincidence.

---

*Lexicon hooks (for LEXICON.udon when terms settle): restriction/lifting (R/L),
closure, scale separation, refluxing/pinning, coupler, parameterization +
error budget, fated lifting (Ⓝ — the §8 discipline's multiscale role),
detail→abstract (already named).*
