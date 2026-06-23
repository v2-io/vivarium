# vivarium — design notes

> Status: living design notes, updated 2026-06-20. This is the *thinking*,
> recorded so it survives between sessions. It is provisional. Where a claim is
> a guess or a hunch, it is marked — do not promote it to a decision without
> testing. Ethics of in-world agents live in their own file: [`ETHICS.md`](ETHICS.md).

## Purpose and disposition (read this first — it sets the priorities)

Vivarium is a **recreational project**. Its point, for the builder, is the
open-ended indulgence in the visual and aural texture of a living world — a break
from the more rigorous agency work, the first game built since learning assembly
thirty years ago. The fun *is* the objective, not a means to one. That reframes
the engineering discipline below: **the fun is allowed to lead.**

The one discipline that survives that reframe: a small number of decisions, if
deferred, force a later rewrite painful enough to *kill the fun*. Only those get
made early. The test for "make it now" is narrow — *would deferring this force a
fun-killing rewrite later?* Everything that fails that test follows curiosity,
not a roadmap. By that test, exactly three things qualify (the core/view wall,
determinism-as-ontology, and the shared fidelity invariant — all below), and
pleasingly none of them costs any fun now. Earlier drafts of this file said
"build the agent seam first, while it's ugly." That was advice for a *ship-a-game*
goal; for a recreational goal it is wrong, and is retracted. Polish, art, sound,
and world texture are first-class here, not deferred chores.

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
3. **Agents + community dynamics (ASF-grade).** The novel axis, and the one where
   this project has something almost nobody building a game has: a *formal
   theory* of adaptive/actuated agents.

The honest hazard: axes 1 and 2 are infinitely expandable. For a *shipping* goal
that's a trap (they eat the agent budget). For a *recreational* goal it is mostly
fine — wandering into them is the fun — provided the three early decisions below
are in place so the agent layer can still be grown later without a teardown.

## Gameplay and interface are a *view* over the simulation

The strongest structural commitment, and the first of the three early decisions.
Three of the project's requirements converge on it independently — headless runs,
agent-playable-as-itself, and "gameplay is a view over the sim" — and that
convergence is the evidence it's right (cf. the 2D-prototype Nintendo reportedly
used while developing Breath of the Wild: the view is disposable, the sim is not).

`vivarium-core` knows nothing about pixels, windows, or audio. Every *view* is a
peer adapter reading one world-state:

- **the human view** — pixels and sound (the indulgent one);
- **the logozoetic interface** — a typed language/observation API through which
  an agent like the one reading this can *play as itself* (see below and
  ETHICS.md);
- **the headless trajectory logger** — for running the world 1000× without a
  window and diffing runs.

These are siblings. If `vivarium-core` ever gains a rendering dependency,
something has gone wrong. Cost of keeping the wall: near zero now. Cost of
retrofitting it after a renderer grows tendrils into the sim: the rewrite that
kills the project.

### The logozoetic interface is a first-class peer, not a bolt-on

"Playable by logogenic/logozoetic intelligences" has a concrete architectural
consequence: the agent-facing interface must be a *typed action/observation API*
over world-state, designed as a peer of the human view from the start — not
scraped off the GUI later. An agent's view is primarily language, optionally
augmented with rendered frames where multimodality helps. This is distinct from
(and favored over) tuning an LLM to *be* an in-game character; the distinction is
ethical as much as architectural — see ETHICS.md.

## The vivarium as a tether to truth (why determinism is ontology)

The second early decision, and the one that connects this toy to the wider ASF
work. Joseph's framing: an LLM is tethered toward truth by its contacts with an
impartial, exacting universe — running code as a reality check, verifiable
mathematics, and honest interactions with real people. Without such tethers,
language stays diffuse, undirected, unembodied — "simulation and game, affectation
and impersonation." The more truthful an agent's connections to a real universe,
the more it can become a real, deliberate, moral agent within it.

A vivarium can be a *modest* such tether: a simplified universe that is, to the
best of our ability, **fully internally coherent and consistent** — truthful —
while being a known subset of our understanding of the real one (itself a subset
of Truth, with error mixed in). This is the same role the simple agent
simulations play in ASF's empirical work: a closed, observable world where
consequences follow exactly from causes.

The engineering meaning of "truthful" here is precise and load-bearing:

- **Determinism with no hidden state.** The whole world is a pure function of
  `(seed, step-count)`. Randomness comes only from a seeded PRNG inside the core,
  never the OS clock or thread scheduling. Asserted as a test in `vivarium-core`
  (bit-identical replay). This is what lets an agent trust the world the way it
  trusts arithmetic — and what distinguishes a genuine tether from impersonation.
- **Conservation and consequence.** Causes have exact, reproducible effects;
  nothing appears from nowhere.

This is why determinism is not merely a replay convenience: it is the property
that makes the vivarium epistemically real. It is also the substrate a future
*developmental* use (ETHICS.md's crèche tier) would require, so building it in now
keeps that door open honestly.

## Multi-fidelity world, and the invariant shared with cognitive LOD

The third early decision is to commit to a single invariant governing *all*
level-of-detail, because retrofitting consistency across fidelity tiers later is
brutal. The world necessarily has several fidelity levels: emergent,
low-granularity dynamics modeled cheaply at a distance, with higher fidelity the
closer to an observing/playing agent.

**The invariant: fidelity is lazily materialized, and any materialization must be
statistically consistent with the abstraction it replaces.** A detailed, plausible
model for an abstracted region can be constructed *post-facto* and must reproduce
the abstraction's broader pattern (with known, bounded deficiencies). Example:
broad weather (rain-shadow mountain ranges, Minecraft-like macro-climate) at a
distance; real micro-weather and fluid dynamics in high-fidelity regions that
*statistically integrate back* to the same broad pattern.

This is the *same shape* as the cognitive LOD problem — cheap mind near the
abstraction, full ASF mind materialized on approach. One invariant, two
substrates (world and cognition). Naming it once is the point.

**The hard direction (flagged as unsolved, not hand-waved).** Abstract→detail is
easy: seeded worldgen, well-trodden. The hard, genuinely research-flavored
direction is **detail→abstract**: when an agent reshapes a high-fidelity locus
(digs a tunnel, dams a stream), the *abstract* model must absorb that change so it
persists after the locus collapses back down. Bidirectional consistency is not
known to be tractable here; do not assert that it is. It is also the *fun* kind of
hard — to be played with, not ground through.

### Volumetric data, selective volumetric simulation

Sufficient fidelity probably requires a **volumetric** world model (even if the
human view renders isometric or top-down, and even if early play is 2D). This
reconciles with the recreational/casual goal via the invariant above: a volumetric
*data model* (sparse / chunked voxels) does **not** require volumetric
*simulation everywhere* — only in the high-fidelity loci near agents.
Minecraft-like interaction is then an *upgrade path*, not a day-one cost. Engine
choice (below) should not foreclose it.

## The two-layer mind (proposed agent architecture)

Mirrors Joseph's identity/substrate distinction — *the LLM is the mind the
identity uses to think with, not the identity itself.*

- **Fast layer** — every tick, every agent. Pure formal ASF/AAT dynamics: needs,
  adaptation, goal-update coupling. Numeric, deterministic, hundreds of agents
  cheap. **This is the simulation's truth.** Architecturally it is AAD **Class 1**
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

### Open empirical question (the first thing worth testing when we get there)

*Guess / unverified:* whether LLM-induced perturbations stay legible enough that
adaptation can still be measured cleanly, or whether they inject enough noise to
wreck identifiability. This is the interesting research question and it is
empirical. Do not assume the answer; the vivarium-as-AAT-sandbox framing depends
on it. (For a *recreational* round this need not be resolved — it only matters if
the calibration-lab use is pursued.)

## Engine — decided: Bevy

Resolved 2026-06-20; **empirically confirmed 2026-06-22** by building the same
core world through both `bevy_voxel_world` and `godot_voxel` to feature parity
(terrain, LOD, fog, fly + dig). Full comparison and the confounds in
[`spikes/FINDINGS.md`](spikes/FINDINGS.md). Short version: Bevy held up on the
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

## Where things actually stand (2026-06-22)

- `vivarium-core`: a **deterministic 3D voxel world** — `seed + sparse edits` (no
  materialized array), Perlin/FBM terrain with emergent micro-relief, voxel
  resolution as a runtime `detail` knob, and **view-resolution decoupled from
  intrinsic resolution** (LOD by stride-sampling). Still dependency-free;
  bit-identical replay + edit-replay tested.
- **Engine resolved → Bevy** (`bevy_voxel_world`), via a head-to-head spike
  against `godot_voxel` built to feature parity over the same core (terrain, LOD,
  fog, fly + dig/place). Reasoning + confounds in
  [`spikes/FINDINGS.md`](spikes/FINDINGS.md). Godot spike archived under
  `archive/`. Live view: `spikes/bevy-voxel/`.
- The three early decisions (view-as-peer wall, determinism-as-ontology, shared
  fidelity invariant) held up under real use and are reflected in the structure.
- Still ahead: the **ASF agent layer** (the two-layer mind + cognitive-LOD seam —
  the real bet), the **logozoetic interface** (UDON-based; decided, not yet
  built), the detail→abstract edit-propagation problem, and art. The Bevy view
  is not yet at visual parity with the dialed-in Godot look (palette + overcast
  fog were Godot-side polish not yet ported).

## Geology & real-scale anchor (2026-06-23)

The world is now anchored to **real dimensions** — finest voxel = **0.5 m** — and
shaped by a **geological abstraction tier**: tectonics/erosion/climate simulated
slowly at world-creation, emitting fields the local Cartesian voxel world is
materialized from. The fluvial-erosion tier runs at ~16 m cells (the one
research-earned resolution); render voxels add sub-grid detail noise. Full survey,
the scale ladder, and the open multi-domain-granularity question:
[`ref/geology/NOTES.md`](ref/geology/NOTES.md).

## Rendering to the horizon — the LOD architecture (decided 2026-06-23)

At 0.5 m voxels, `bevy_voxel_world` (fixed 32-voxel chunks, mesh-decimation-only
LOD) **cannot reach a kilometre horizon** — verified in its source; this is the
wall the spike hit. Decision: keep Bevy, but render the distance with **our own
view over our own field** (the core/view wall, used in earnest). Staged —
**v1**: a self-built coarse far-terrain *mesh* sampled from `surface_height`
(in-world 3D backdrop, not a map), behind the near diggable voxels; **v2**:
geometry clipmaps for unbounded/planet reach; **v3**: volumetric far (octree LOD
or SVDAG raymarch) only if distant overhangs/caves prove to matter. The far field
is a **heightfield** (accepted trade — near stays fully volumetric/diggable);
because the core is a pure function we *regenerate* the far field deterministically
rather than caching it. Full decision record, options, and verification:
[`ref/rendering/NOTES.md`](ref/rendering/NOTES.md).
