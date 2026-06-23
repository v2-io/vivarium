# vivarium — context for Claude (and successors)

**What this is.** A simulation game (RimWorld/DF lineage) with a 3D voxel world
(Minecraft-like, "3D all the way down"), whose real bet is *simulation-grade
agents and community dynamics* built on the Agentic Systems Framework (ASF /
AAT). Read [`DESIGN.md`](DESIGN.md) before proposing anything — it carries the
live thinking and the open questions.

**Stage (2026-06-22).** Past the first vertical slice. `vivarium-core` is a
deterministic 3D voxel world (seed + sparse edits, Perlin/FBM terrain, voxel
LOD); the **engine question is resolved → Bevy** (`bevy_voxel_world`), confirmed
by a head-to-head spike against Godot — see [`spikes/FINDINGS.md`](spikes/FINDINGS.md).
The live view is `spikes/bevy-voxel/`; the Godot spike is in `archive/`. UDON
(`~/src/libudon`) is *decided but not yet implemented* as the core-side
notation for the logozoetic interface + scenario/replay formats.

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
