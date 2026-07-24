# vivarium — ARCHITECTURE

*Source / consolidating overview — **not claim canon.** Settled claims live in `core/src/` (`core/OUTLINE.md` · `#scope-segment-canon`). Use this file as extraction substrate and mental-model map; when a segment exists, the segment wins.*

*v0.3, 2026-07-10 (body). Derivations and detail still pointed here: `.super-archive/from-theory/multiscale-seams.md`, `.super-archive/from-design/DESIGN-REDUX.md`, `.super-archive/from-design/DESIGN-MATERIAL.md`, `.super-archive/from-design/DESIGN-SYSTEMS.md`, `tabularium/terrestris.ordinum.udon`, `LEXICON.udon`, `ASF.md`. Graduated teaching zoo: `.super-archive/from-theory/multiscale-methods.md`. Status marked inline: **settled** / **stance** / **open** / **gap**. Vocabulary is `LEXICON.udon`-canonical.*

> [!note]
> Erosion and hydrology appear below only as **two early instances of many more systems to come** — the systems that happen to exist in code, not the subject. The subject is the general machinery that lets an arbitrary number of interdependent systems be developed in parallel, each principled.
>
> **Guard against domain-fixation** (Joseph, 2026-07-10): the framework is a **domain-agnostic simulation substrate**. Interaction contract and representation-agnosticism: claim homes `#form-flux-web`, `#def-nomos`, measured box instance `#obs-hydrosphere-box-nomos`. Field-on-a-grid is not the assumed primitive. Sequencing gradient and full prose remain **source** below / in design docs until further segmented.

---

## Layer 0 — the whole architecture in one paragraph

**Principle claim home:** `#post-represent-by-consequence` — spend simulation, precision, and storage only where a consumer depends on it.

**Three axes** (budget map also `#sketch-three-project-axes`): (1) **substrate machinery** — multiscale, multirate systems joined by conserved fluxes, pull evaluation (`#form-pull-query-composition`, `#form-scale-separation-directional`; §§1–5 source); (2) **developmental ladder** — phases freeze converged state into law the next phase reads (§6; ordinum); (3) **use-case as fidelity contract** — `#disc-vivarium-purpose`, `#def-in-vivia` (§7 source).

The software layer — content-addressed memo store as save file (`#form-store-as-save`) — is how axis 1 is made runnable. **AAT calibration is one privileged use-case on axis 3, not the project's only telos.** View host: `#form-engine-bevy`.

---

# Axis 1 — the substrate machinery

## 1. The general principle — the multiscale operator algebra *(established)*

> **Claim homes:** `#form-rl-closure-algebra` · `#form-seam-flux-exchange` · `#form-face-flux-register` · `#form-scale-separation-directional` · `#obs-mean-pin-manufactures-seam` · `#form-store-as-save` · `#disc-unlawfulness-budget`. This section is **source elaboration** only — do not restate FE here. Teaching zoo (AMR/HMM/multirate map) graduated: `.super-archive/from-theory/multiscale-methods.md`.

## 2. The method zoo — shared vocabulary *(established; primaries read; dual-home collapsed)*

> **Claim homes:** algebra / closure → `#form-rl-closure-algebra`; seam fluxes → `#form-seam-flux-exchange`; face register → `#form-face-flux-register`; ladder climb/descend → `#form-fidelity-ladder`. **Teaching table (method × closure source × vivarium instance)** lives in graduated `.super-archive/from-theory/multiscale-methods.md` §2 — do not restate as parallel law here.

**The ladder runs both ways** (cash-out: `#form-fidelity-ladder`). Climb (stepwise, emergent) to *discover* a system's behaviour; once probe-validated, **descend** to a tight surrogate reproducing the discovered statistics ($R\circ L=\text{id}$ on the chosen statistics is the honesty gate), keeping the expensive rung as calibrator. Each system declares an **execution class** — batch-deep (rare, cache to zero), relaxation (settle per phase, then live), procedural-tight (closed-form/surrogate, evaluated constantly) — residual declaration debt under `#form-kernel-imperative-boundary` / add-a-system checklist (§9), not a second multirate package.

## 3. What is distinctively ours — the closure choice *(stance)*

The literature's $L$ *samples* the missing measure (Monte-Carlo, ensemble mean, max-entropy) — equation-free even notes $L$ is non-unique and any choice heals. **Ours is fated lifting** — claim home `#post-determinism-as-ontology`; dictionary `#lexicon/term/fated-noise`. Consequences for this architecture (substrate, not a second law): (1) **memoization is sound** — two evaluations of the same key agree; (2) the world is a pure function of (seed, keys); (3) **ensembles are over seeds**, not re-rolls of a cell.

Three further twists we own: **lazy, backwards-from-now** coupler pulls (the literature pulls forward-in-time); **observer-driven refinement** (attention, not an error estimate); and **consumer-dependent restriction**: the literature fixes one $R$ per method; we want **$R$ per consumer** (hydrology needs conserved totals, line-of-sight needs max, display needs mean). So a macro cell honestly carries `{mean, min, max, conserved-totals}` with each field flagged *guaranteed* vs *approximate*. **Store the wrong statistic and the fine materialization silently corrupts the macro** — the failure mode consumer-dependent restriction exists to prevent, and a systems-theory fact (representativity error), not a coding detail.

## 4. The seams — position AND time as one discipline *(stance; `.super-archive/from-theory/multiscale-seams.md`)*

> **Claim homes:** fluxes-not-states / one seam on two axes → `#form-seam-flux-exchange`. $R$/$L$/closure → `#form-rl-closure-algebra`. Observer-side resolution rule → `#form-fidelity-invariant`. This section is **source elaboration** (drainage-shaped space seam, multirate time seam, participant-as-clock texture).

A world is computed as a **lattice of tiles, each at some resolution in space and some rate in time, coupled only at their boundaries by fluxes.** A *seam* is any such boundary. The recognition §4 is built on: **the discipline at a seam is identical whether the seam is in space or in time** — because both are the same $R/L$/closure algebra with the argument being a space cell, a time interval, or (usually) a space-time patch. What crosses is a **flux of a conserved quantity** (never raw state); what is guaranteed is a **sufficient statistic**, integrated over space and averaged over time at once.

So position and time are **one seam on two axes**, and the two axes interact: the **space seam** is drainage-shaped (a point's dependency is its upstream catchment closure + downstream path to base level — irregular *islands of interdependence* the coarse spine draws with kernels we already have, though coarse-global cross-face assembly is new work and its basin-partition accuracy an open measurement, not free), and the **time seam** is the multirate band (which rate a tile runs, how it reads its neighbours — quasi-static slow / time-averaged fast). A river crossing a tile edge couples in **both**: its upstream catchment (space) *and* the slow erosion tier reading it as time-averaged discharge (time). And the **fidelity invariant is this same rule, observer-side** (claim home `#form-fidelity-invariant`): render/simulate at exactly the resolution — spatial *and* temporal — the most-demanding present participant needs; the participant sets both axes' resolution, and **time couples to the most-demanding participant** (a human clamps the world to ~2 Hz real-time; a pure-agent world has fully elastic logical time). The four seam types (spatial LOD; temporal macro→fine handoff; aspect-coupling; and the one *open* reversion seam) and their primary-source grounding are worked in `.super-archive/from-theory/multiscale-seams.md`.

## 5. Implementation — the lazy memoized query graph and the store *(established paradigm; the engine is a gap)*

*Claim homes: evaluation pattern → `#form-pull-query-composition`; three-scoped decomposition + time-DAG + prefetch → `#form-three-scoped-runtime`. Below is source elaboration.*

This is how the §1 algebra is made runnable. The runtime is a **demand-driven, memoized query graph**: a query is `(aspect, region, resolution, time)`; a pull recurses into its dependencies; results memoize by key; nothing computes until demanded, nothing twice. It degrades into three scoped pieces that *are* the architecture: a **precomputed coarse-global spine** (planet-spanning aspects at low res — also the dependency planner, §4), **lazy memoized local cones** (fine detail pulled near the observer), and an **edit-propagation layer** (state half measured solved; law half open — `#sketch-detail-abstract-reversion`).

**The complete content-addressed key is one of the three walls that make parallel development safe** (§9) *(stance; enforcement is a gap)*: every memo is keyed by *everything that affects it* — upstream input hashes + coupling params + seed + the nomos's **declared version** (hand-stamped today; source-derived is the target, below). So swapping erosion-v1 for erosion-v2 invalidates its cache and its dependent cone **and nothing else**; iteration is cache-transparent; scratch and canon coexist in one store. Under-keying is silent corruption, over-keying only wasted recompute — **the asymmetry is decisive: over-key.** The manual `FILL_ALGO_VERSION` is the known weak link (a stale memo mid-iteration doesn't waste time, it *lies*); source-derived versions are the target (the 2026-07-09 pervasive-memoization directive, `.super-archive/from-design/DESIGN-REDUX.md` §12).

**The save-file *is* the memo store** *(claim home `#form-store-as-save`; source `.super-archive/from-design/DESIGN-REDUX.md` §13)* — git-repo-shaped, content-addressed: `manifest` · `objects/<hash>` (immutable) · `roots` (current hash per `(aspect, tile, level, time)`) · `mutations/` (append-only, the irreducible truth). Copy the folder → the world moves. Invalidation is correctness (by content hash); eviction is space (LRU on immutable blobs — deleting one costs a recompute, never correctness), so **memoize pervasively**, at every tier and rate. **The run-modes carve** (`LEXICON.udon` §3) is the store's typing job — the strictly-causal / replay / discardable-iteration / live-play distinction maps to **Closed vs Open-with-recorded-forcing** (§6/§7 ontology) plus the pre-participation **non-intervention** register; an iteration run must never write a canon `root`.

The substrate the graph addresses *(settled; `.super-archive/from-design/DESIGN-MATERIAL.md`)*: a cube-sphere **`CellId(u64)`** Hilbert key orders **chunks**; a chunk's interior is a plain Cartesian array (stencils at ~6 Gcells/s, measured); **halos** carry cross-chunk neighbours (same-face live; cross-face unbuilt — #form-cellid-chunk-patch). The **column** (a stack of real-valued **strata**) is the storage unit *(claim home `#form-column-control-volume`)*; **voxels** are a materialized *view*; **bodies** overlay cross-cutting masses; elevation is always *derived*, its meaning *declared* (§3).

---

# Axis 2 — the developmental ladder

## 6. Phases, coupling, and law-promotion *(established; `tabularium/terrestris.ordinum.udon`, `.super-archive/from-theory/multiscale-seams.md`)*

Systems come online in a required order and become law by *freezing*. This is a **co-equal axis** with the substrate machinery, not a detail of it: axis 1 is scale in space-and-fidelity; axis 2 is scale in **developmental time**.

Systems couple **multirate** in four bands, each on its own step: deep drivers (Myr) flux uplift/geothermal; orbital/climate (10–100 Kyr) flux temperature/precip/ice; surface process (Kyr) flux elevation-change/sediment; fast/biological (yr–centuries) flux water-depth/vegetation. Fast sees slow as quasi-static; slow sees fast as time-averaged — the §4 time seam, at world scale.

**A phase-transition is that time seam at the largest scale we have.** A **phase** is a memoization/immutability boundary: it runs its systems to convergence, freezes the converged state into a world-scale **memo**, and that memo *is the macro law* the next phase builds on — the AAT invariance cut. Each phase manufactures the **Charge** to the next gate; its **Promise** is what it hands forward; its **Record** (readable-in-rock) is a lossy sanity-probe, never the canonical state. Phases enter code **one at a time as runnable nomoi** — no phase-enum machinery until they exist.

**This is where axis 1 and axis 2 join — and it answers "when do we need which algorithm."** A system's **$R$** (its macro summary) is what the *next phase* reads as law; its **$L$** (its detail lifting) is what a *consumer's query* materializes. So a system earns its $R$ exactly when a downstream **Charge** reads its macro, and its $L$ exactly when a query reads its detail. Biomineralization is an Abyssal #earth charge that is itself the mechanism of the oxygenation gate, so it needs enough $R$-fidelity *before* Abyssal→Primeval opens; it needs $L$ only when a consumer queries limestone. The #gate/#earth/#mech/#emergent tags already encode how much $R$-fidelity a downstream gate demands.

The honest catch, and the ontology it forces (claim home `#disc-unlawfulness-budget`; `LEXICON.udon` §7.2; source `doc/plan/vivium-operational-workflow.md` BREAK-2): convergence is **undecidable**, so each freeze carries a bounded **unLawfulness budget**. **Realized** (law immutable — frozen at a chosen tolerance) is a threshold you pass; **Lawful** (law self-consistent, no glitches) is an asymptote ($\varepsilon \to 0$) approached forever. Realized ⟂ Lawful; a natural-modeling world is *merely Realized*. This is why the parity target is honestly an early-Abyssal world that is **Realized, not Lawful** — and flagged as such.

---

# Axis 3 — use-case as fidelity contract

## 7. What the world is for, and the AAT bridge *(stance; `LEXICON.udon` §5, §7; `ASF.md`)*

A finished world (a **vivium**) is consumed by many use-cases, and **the use-case is a statement of which fidelity axes must be honest** (`LEXICON.udon` §5). The working menu (Joseph's 2026-07-03 enumeration, recorded here as the canonical set): *natural-world hypothesis testing & experimentation* · *world-building algorithm/process advancement* (the implementation-facing use-case: fidelity, speed, memory) · *Earth-simulation* · *the simple joy of personal exploration and discovery* · *AAT/ASF-theory simulation* · *agentic inhabitation* · *realistic games* · *speculative-coherent games* — an open list, not a closed one. Each phenomenon is tagged on four independently-failing axes: **A** Earth-history fidelity, **B** physics fidelity (*lawful first, Earth-shaped second*), **C** relation type (the #gate/#earth/#mech/#emergent buckets refined), **D** implementation status. *Vibe-modeling = high D on low B/C* — making the axes explicit turns that from camouflaged into glaring. Then: *speculative-coherent game* needs C-consistency + B (lawful, not Earth-true); *Earth-simulation* pins A hard; *hypothesis-testing* needs A+B+C real; *inhabitation / agentic participation* trips ETHICS.md and the moratorium (exploration — ethereal, observe-only — is moratorium-clear); *AAT-calibration* additionally needs the agent layer legible. The mediation between how systems *produce/store* (domain-centric) and how consumers *query* (consumer-centric) is the memoized query graph of §5, and the seam between them is a chosen sufficient statistic (§3) — the `⟷` the design has circled since the first sketch.

**AAT-calibration is uniquely well-served** — because vivarium authors from the outside exactly the typed object an adaptive agent infers from inside:

| AAT (agent infers, from inside) | vivarium (we author, from outside) |
|---|---|
| law **$\theta$** — transition/observation structure | the physics + constants; each phase's **Promise** |
| state **$\Omega$** | the live world state (terrain, water, weather, populations) |
| chance **$\varepsilon$** | **fated noise** (§3) — a pure function of (seed, key) |
| **compute-shortfall** (a *ratio* — knowing the generator ≠ running it faster than reality) | the **fidelity ladder / lazy memoized runtime** — our entire runtime *is* a compute-shortfall manager |

A phase-transition promotes converged state into law (the invariance cut, §6); the aleatoric boundary is frame-relative (fated noise is genuine chance to the inside agent, a deterministic lookup to us — we are the housing rule); and the developer is itself an AAT agent whose observation channel is the instruments (TST live here) — which is *why* "build the instrument before tuning by feel" is architecture, not preference. But this is **one column of axis 3**; the calibration bridge does not make the machinery *for* AAT any more than a telescope's optics are *for* one galaxy.

---

## 8. Invariants, gaps, and the open-problem inventory

**Three spine invariants the implementation may never trade away:** (1) **core/view wall** — claim home `#form-core-view-wall`; (2) **determinism-as-ontology via fated lifting** — claim home `#post-determinism-as-ontology`; (3) **complete content-addressed key** — claim home `#form-complete-content-addressed-key` (§5 remains source elaboration).

**Status-quo gaps** (designed, not built — the build path in `doc/plan/abyssal-parity-plan.md`): the **store + nomos layer** (§5); the **coarse global spine** as dependency planner (§4, §6); **tile nomoi with flux boundary conditions** replacing today's hardcoded edge policy — *this is also the seam fix*: the present code seeds every patch's drainage at its own area and hardcodes edge-outlets, so tiles are non-composable and the seam pathologies (`seam_ridge` red) are the visible signature; the **query front-end** the view queries through, so navigation and persistence fall out; the **RNG fix** before agents. (Today's `spikes/worldview` is a physics testbench, not this runtime — one fixed patch, re-seeds from the raw prior on movement. The kernels are proven; the world-frame around them is unbuilt.)

**detail→abstract** *(open — and BOTH its ranking and its statement are now suspect)*.

⚠ **CORRECTED 2026-07-13, twice, and the second correction matters more than the first.**

**(a) The superlative is struck.** This was written as *"THE one open research problem."* **A superlative is a claim with no predicate** — nothing can convict it — and it encodes the author's sense of importance rather than a measurement. Worse, it **suppresses competitors**: `doc/plan/vivium-operational-workflow.md` collects several honest BREAKs, and BREAK-2 (*convergence is undecidable ⇒ every freeze carries a structural unLawfulness budget ⇒ **"certify Lawful" may be permanently unreachable***) touches the **moratorium's own revisit-condition** — which the front door was quietly telling readers did not exist.

**(b) ⚠⚠ AND THE STATEMENT BELOW IS ITSELF STALE.** `DECISIONS[wavelet-store-spiked-the-seam-is-not-the-details]` (measured): ***"`detail→abstract` AS THIS DOC WRITES IT is RETIRED — an irreducible single-cell edit propagates to the root BIT-EXACTLY, O(log N), ~60 lines."*** **The state upscales exactly. What does NOT upscale is the LAW** (‖R∘E − E∘R‖ measured at **+5.34 m** against the 7.22 m erosion itself carved — *the law-upscaling error is the size of the physics, and it is a BIAS*). ⇒ **The open problem is the NONLINEAR CLOSURE FOR A NON-LOCAL FLUX — which "detail→abstract" never actually said.** The decision's own instruction: ***"RENAME it to what it really is; do not close it and do not leave it as written."*** **Executed 2026-07-24 in `core/`:** `#sketch-detail-abstract-reversion` now carries the split (state half retired as measured; open problem renamed to the nonlinear closure for a non-local flux).

**(c) ⇒ AND THE HONEST POSITION ON THE INVENTORY AS A WHOLE:** *we do not currently know what this project's open problems are.* The audit dissolved some, renamed others, and created new ones (the router; the leaf-only-evolution price); the BREAK list predates it. **Do not trust any count or ranking of open problems — including this one — until the `core/` segments are laid down and the census is derived rather than asserted.** *(Joseph, 2026-07-13.)*

What can still be said about this problem without ranking or census: it **blocks agent EDITS**, it is **not on the ethereal-explorer path** (a read-only explorer makes none), and the surviving residue is **plausibly the same shape as the AAT identifiability bet** (§7) — *both are upward re-summarization after a micro perturbation.*

Forward/downward is mature (all four methods do $R$ and $L$ forward; Vandenbulcke & Barth 2019 upscale by assimilation). What we are not aware of a method for is **upward $R$ of an irreducible discrete edit** (a dammed stream, a placed structure — not a statistical closure) **into a content-addressed, memoized macro with correct up-invalidation** (`.super-archive/from-theory/multiscale-seams.md` §2.4). It is **not on the ethereal-explorer path** — a read-only explorer makes no edits — and is plausibly the same shape as the open AAT bet (does an $\Omega$-perturbation stay legible): both are *upward re-summarization after a micro perturbation*.

## 9. Adding a system — the contract

> **Claim homes:** registry gate / declaration-as-key-stem → `#form-nomotheke-registry` (+ `#def-nomos`). Algebra objects → `#form-rl-closure-algebra`. Flux interface → `#form-flux-web`. Complete key → `#form-complete-content-addressed-key`. This checklist is **source procedure** for authors; the registry law is the segment.

To add a world-system and have the machine compose it, declare its place in the algebra: (1) **its $R$/$L$/closure** — the macro summary it exposes, how it materializes detail, and *fated* closure so it is memoizable; (2) **its fluxed quantities** — the per-quantity coupler interface it produces/consumes, the *only* thing others may depend on (fine-grained, never a monolithic blob); (3) **its execution class and timescale band** (so coupling treats it quasi-static or time-averaged); (4) **determinism in its keyed inputs**, with a declared nomos-version (hand-stamped today; source-derived is the target, §5); (5) **its declaration in the nomotheke** (`nomotheke.rs` — since 2026-07-10 this is code, not prose; claim home `#form-nomotheke-registry`): the four epistemic tags (§7) as data, deps (from which derived quality is computed by weakest-link fold), **promises each with an explicit conservation claim**, and assumption anchors into `ASSUMPTIONS.md` (test-enforced; the declaration mints the store keys, so an undeclared nomos cannot reach world-law); (6) **its regime probes, written first** — including its **seam probes** (a space-seam continuity probe like `seam_ridge`; a time-seam near-stationarity probe) — and paired with the declaration: a declared tier is a falsifiable claim, and the probe is what would convict it. If those hold, three walls keep two authors from breaking each other — the **flux interface**, the **complete key**, and **execution-class + multirate coupling** — so interdependence is *mediated*, never *shared-mutable*, and the count of systems can grow without the coupling cost growing with it.

**Prior-art validation — CHONK** *(Gailleton et al. 2024, `gmd-17-71`; read 2026-07-10)*: the geoscience-LEM world hit this exact wall — process laws combined by *superposing independently-solved flux grids* break the moment processes are interdependent (a lake's water/sediment budget depends on *all* upstream processes; erosion efficiency depends on the sediment flux's lithology) — and answered it with a decomposition worth adopting: **numerically separate `{fluxes, properties, processes, graph}` as four loosely-coupled modules**, so any one swaps without the others (swap the *graph* → change grid type / 1-D / Voronoi; swap a *process* → change a law; add a *flux* → track a new quantity). That is our contract, sharpened: our fluxed quantities = their *fluxes* (separate from processes, fine-grained), our systems = their *processes*, our tile / $R$ / $L$ = their *graph*, our column/strata = their *properties*. Two transfers land directly: (a) the graph must be **process-agnostic** while processes may be **domain-specific** (marine / fluvial / lake / glacier) — CHONK's own statement of the domain-fixation guard above, done for geomorphic sub-domains; and (b) their load-bearing rule — **every interdependent process at a node runs to *definitive* fluxes before transmitting downstream** — is our sufficient-statistic seam at cell grain, and is exactly how they solve the lake/interdependence case that our water-at-tile-boundaries risk mirrors. Caveat: CHONK is single-resolution with no memoization/LOD, so it informs the *coupling/seam* layer only — the multiscale and detail→abstract concerns stay ours.

---

*Founding DESIGN and DESIGN-REDUX are graduated (`.super-archive/from-design/`). Claim law is `core/`. Live design residual: NOMOS-CONTRACT (procedure + defect table). DESIGN-MATERIAL / SYSTEMS / REDUX / multiscale theory / discretisation graduated. Build sequence: `doc/plan/abyssal-parity-plan.md`. Future instances inherit what we leave.*
