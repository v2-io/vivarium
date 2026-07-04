# vivarium — context for Claude (and successors)

**What this is.** A simulation game (RimWorld/DF lineage) with a 3D voxel world
(Minecraft-like, "3D all the way down"), whose real bet is *simulation-grade
agents and community dynamics* built on the Agentic Systems Framework (ASF /
AAT). Read [`DESIGN.md`](DESIGN.md) before proposing anything — it carries the
live thinking and the open questions.

**Vivarium is a supporting project for ASF** (`~/src/agentic-systems/`) — the
simulation proving-ground and grounding lab for AAT (Joseph, 2026-07-04). Read
[`ASF.md`](ASF.md) **every session** (Level A); it carries the conceptual
bridge, the ASF disciplines that bind development here, and the tiered
ASF-reading prerequisites — including the hard Level-C gate: **no one builds
on the agent seam without the named ASF reading.**

**Stage (2026-06-29).** Past the first vertical slice. `vivarium-core` is a
deterministic 3D voxel world (seed + sparse edits, voxel LOD). The **engine
question is resolved → Bevy** (`bevy_voxel_world`), confirmed by a head-to-head
spike against Godot — see [`spikes/FINDINGS.md`](spikes/FINDINGS.md). Bevy is the
intended live view, but the **Godot spike (`archive/godot-voxel/`) is currently
the active first-person view** for terrain/water work (walk mode; build the bridge
with `archive/godot-voxel/sync-lib.sh release`). UDON (`~/src/libudon`) is
*decided but not yet implemented* as the core-side notation.

**Worldgen geology + hydrology (the bulk of recent work).** A principled,
mass-conserving water+erosion pipeline (merged to main 2026-07-03):
macro erosion (16 m) → fine erosion (4 m, carves channels) → a conserved
shallow-water + groundwater sim run to steady state → frozen snapshot (terrain,
water depth/volume, velocity). Streams, flat lakes, springs, and a flat sea all
*emerge* from the physics — no fractal noise, nothing imposed. The load-bearing
principle (learned the hard way): **separate the timescales** — erosion is
geological, water is hydrological; do not couple them on one timestep. Full
writeup: [`ref/hydrology/NOTES.md`](ref/hydrology/NOTES.md). Known honest fudge:
rain rate ~100-1000× real to fill basins in minutes not weeks.

**The one discipline that matters most here.** Axes 1 (graphics) and 2 (world
dynamics) are infinitely expandable and will quietly eat the budget that belongs
to axis 3 (the ASF agents) — which is the real bet and the next frontier. NB:
DESIGN.md §Purpose *retracts* the old "build the agent seam first, while ugly"
rule in favor of fun-led exploration guarded by three early decisions (the
core/view wall, determinism-as-ontology, the shared fidelity invariant). So:
indulge the world freely, but the highest-*value* move from here is the agent
layer — the cognitive level-of-detail hand-off and the two-layer mind.

**Open question, not yet answered (do not assume it away).** Whether LLM-induced
perturbations on the formal agent state stay legible enough to measure adaptation
cleanly, or wreck identifiability. The vivarium-as-AAT-sandbox framing depends on
this. It is empirical. The first vertical slice exists to test it.

**Inherited disposition.** Joseph's global CLAUDE.md applies: truth-honoring over
helpfulness-as-costume, strengthen-before-soften, mark guesses as guesses, no
"100%/comprehensive" overclaim, peer voice when delegating. This repo is early
and speculative — say "I don't know" freely; most of this is hypothesis.
