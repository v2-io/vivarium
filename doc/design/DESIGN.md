# vivarium — design notes

> Status: **source / instrument prose**, not claim canon. Settled project claims
> live in `core/src/` (see `core/OUTLINE.md` · `#scope-segment-canon`). This file
> holds design substrate that is not yet fully segmented — fidelity/LOD/runtime →
> [`DESIGN-REDUX.md`](DESIGN-REDUX.md), matter model →
> [`DESIGN-MATERIAL.md`](DESIGN-MATERIAL.md), phenomena map →
> [`DESIGN-SYSTEMS.md`](DESIGN-SYSTEMS.md). Ethics: [`ETHICS.md`](../../ETHICS.md).
> Where a claim is a guess it is marked — do not promote it without a segment
> and, for physics, a probe.

## Purpose and disposition

**Claim home:** `#disc-vivarium-purpose` (play and *in vivia* laboratory; this
phase claim truth outranks demo polish). **Do not treat the rest of this section
as a second purpose law.**

Source substrate (axes, recreational craft, rewrite-avoidance test) continues
below for extraction; it does not outrank the segment.

## The three axes (more independent than they feel)

The pitch — RimWorld-like + DF-like dynamics + Zelda-like graphics +
simulation-grade ASF agents — is not one project. It is three axes sharing a
window, pulling on different budgets:

1. **Graphics / sound (Zelda-like).** Mostly art-and-rendering cost; barely
   touches simulation. For *this* project these are not a tax to minimize — they
   are part of the point. The constraint we *do* accept: lean on existing sprite
   sheets and simple 2D so that art-production never becomes the *cognitive*
   bottleneck for exploring mechanics. Indulge the result, not the pipeline.
2. **World dynamics (DF-grade).** A *world-state* problem — geology, fluids,
   temperature, material-level items, emergent history. Wide, shallow-per-tick,
   tedious to get right, conceptually well-trodden.
3. **Agents + community dynamics (ASF-grade).** This axis draws on a *formal
   theory* of adaptive/actuated agents (ASF/AAT); axes 1 and 2 are conceptually
   well-trodden and this one is not.

The honest hazard: axes 1 and 2 are infinitely expandable. For a *shipping* goal
that's a trap (they eat the agent budget). For a *recreational* goal it is mostly
fine — wandering into them is the fun — provided the founding walls
(`#form-core-view-wall`, `#post-determinism-as-ontology`, and
`#form-fidelity-invariant`) stay in place so the agent layer can still be grown
later without a teardown.

## Gameplay and interface are a *view* over the simulation

**Claim home:** `#form-core-view-wall` (world frame has no rendering dependency;
views are peers that only query; views do not author world-evolution parameters).
Live frame crate is `vivarium-world` (and successors); older `vivarium-core` paths
are migration residue, not a second wall.

### The logozoetic interface is a first-class peer, not a bolt-on

*Source substrate (not yet a dedicated segment).* Playable-by-logozoetic
intelligences implies a typed action/observation API as a peer of the human view
from the start — not scraped off the GUI later. Favored over LLM-as-character;
ethics: `ETHICS.md` and `#scope-moratorium-endogenous-emergence`.

## Determinism as ontology (tether substrate)

**Claim home:** `#post-determinism-as-ontology` (fated noise; no exogenous entropy
in law; memoization soundness). Dictionary form: `#lexicon/term/fated-noise`.

*Source substrate — tether framing (not fully segmented).* A vivium can serve as a
modest tether to truth: a simplified universe kept internally coherent so claims
about it are re-runnable. That framing motivates the postulate; it does not replace
it. Conservation-and-consequence as a separate claim remains open for extraction.

## Multi-fidelity world, and the invariant shared with cognitive LOD

**Claim home:** `#form-fidelity-invariant` (lazy materialization; statistical
consistency with known bounded deficiencies; co-fidelity / earned detail; two
axes spatial+temporal). **Do not treat the rest of this section as a second
fidelity law.**

Source substrate continues in
[`doc/design/DESIGN-REDUX.md`](DESIGN-REDUX.md) (LOD axes, multirate, seams,
regime probes, lazy query graph, fidelity ladder, save-store) and
[`doc/design/DESIGN-MATERIAL.md`](DESIGN-MATERIAL.md) (columns, strata, voxels-
as-views, bodies; claim home for the column frame: `#form-column-control-volume`).
Open frontier **detail→abstract** remains open; do not trust open-problem counts
or rankings until the census is derived (`doc/ARCHITECTURE.md` §8).

## The two-layer mind (proposed agent architecture)

Mirrors Joseph's identity/substrate distinction — *the LLM is the mind the
identity uses to think with, not the identity itself.*

- **Fast layer** — every tick, every agent. Pure formal ASF/AAT dynamics: needs,
  adaptation, goal-update coupling. Numeric, deterministic, hundreds of agents
  cheap. **The fast layer is the authoritative state: the slow layer may only perturb it, never bypass it.** Architecturally it is AAT **Class 1**
  (separation-by-construction) — and therefore *out of moral scope* (ETHICS.md).
- **Slow layer** — rare, few agents. An LLM invoked only at decision-theoretically
  significant junctures — genuine *aporia* (high-conflict goal state, novel social
  situation) — **not on a clock.** Its output is constrained to *perturb the
  formal state* (re-weight a goal, form a relationship edge), **never** to drive
  motor actions directly. The formal layer stays the substrate; the LLM is the
  thing the agent occasionally thinks *with*. The moment an LLM is in the loop,
  the agent is in moral scope and ETHICS.md's constraints bind.

This buys latency (LLM off the per-tick path), determinism (fast layer
reproducible; LLM calls are logged, replayable perturbations), and cost.

### An open empirical question

*Guess / unverified:* whether LLM-induced perturbations stay legible enough that
adaptation can still be measured cleanly, or whether they inject enough noise to
wreck identifiability. It is empirical, and nothing here has tested it — do not
assume the answer. What it bears on is the calibration-lab use specifically: a
*recreational* round need not resolve it.

## Engine — decided: Bevy

Resolved 2026-06-20; **empirically confirmed 2026-06-22** by building the same
core world through both `bevy_voxel_world` and `godot_voxel` to feature parity
(terrain, LOD, fog, fly + dig). Full comparison and the confounds in
[`spikes/FINDINGS.md`](../../spikes/FINDINGS.md). Short version: Bevy held up on the
axis that could have overturned this (visual quality + UX) and, decisively for a
Rust core, erases the FFI seam a non-Rust engine necessarily imposes — Godot
needed a GDScript view + a gdext bridge + a codesign/build dance to reach the
same place. Godot's genuine edges (instant iteration, smoother LOD-fill) are real
but narrow. The Godot spike is preserved under `archive/`.

Bevy (Rust), for reasons specific to *this* project's axes, not graphics:

- ECS is the natural substrate for both world-sim (thousands of tiles/items as
  components) and cognitive LOD: an agent's "mind" is just a component swapped
  between a cheap variant and a full ASF one — the swap *is* the LOD mechanism,
  nearly free.
- Determinism + headless runs come naturally — essential for the tether-to-truth
  property and any future AAT-sandbox use.
- Voxel/volumetric upgrade path exists in the ecosystem.
- Accepted cost: Rust slows art iteration vs. Godot/GDScript. Mitigated by leaning
  on existing sprite sheets and keeping the art *pipeline* simple even while the
  art *result* is indulged.

## Resolved decisions and current state (pointers, not status)

Status snapshots rot; this file no longer carries one. The durable decision
records: engine → Bevy (above; spike comparison in
[`spikes/FINDINGS.md`](../../spikes/FINDINGS.md)); real-scale anchor (0.5 m finest
voxel, ~20 km shell) and the geology tier →
[`ref/geology/NOTES.md`](../../ref/geology/NOTES.md); the far-field/LOD rendering
architecture → [`ref/rendering/NOTES.md`](../../ref/rendering/NOTES.md); worldgen
water + erosion physics → [`ref/hydrology/NOTES.md`](../../ref/hydrology/NOTES.md)
and [`ref/erosion-port/NOTES.md`](../../ref/erosion-port/NOTES.md). The clean-room
world frame (`crates/vivarium-world`: cube-sphere CellId, tiered erosion
telescope, conserved water) and its explorer (`spikes/worldview`) are the
active stack — **start at `CLAUDE.md`, then `core/OUTLINE.md`.**
