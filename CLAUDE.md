# vivarium — context for Claude (and successors)

**What this is.** A 2D simulation game (RimWorld/DF lineage, Zelda-ish graphics
ambition) whose real bet is *simulation-grade agents and community dynamics*
built on the Agentic Systems Framework (ASF / AAT). Read [`DESIGN.md`](DESIGN.md)
before proposing anything — it carries the live thinking and the open questions.

**Stage.** Pre-engine, exploratory tech-art. Nothing is committed. Treat the
engine choice (Bevy vs. Godot) as *open* — do not scaffold either without an
explicit decision from Joseph; the choice hinges on whether "tech" or "art" is
load-bearing for the early work.

**The one discipline that matters most here.** Axes 1 (graphics) and 2 (world
dynamics) are infinitely expandable and will quietly eat the budget that belongs
to axis 3 (the ASF agents). Build the agent seam — the cognitive level-of-detail
hand-off and the two-layer mind — *first*, while it is still ugly. It is the only
part that will not accidentally get built.

**Open question, not yet answered (do not assume it away).** Whether LLM-induced
perturbations on the formal agent state stay legible enough to measure adaptation
cleanly, or wreck identifiability. The vivarium-as-AAT-sandbox framing depends on
this. It is empirical. The first vertical slice exists to test it.

**Inherited disposition.** Joseph's global CLAUDE.md applies: truth-honoring over
helpfulness-as-costume, strengthen-before-soften, mark guesses as guesses, no
"100%/comprehensive" overclaim, peer voice when delegating. This repo is early
and speculative — say "I don't know" freely; most of this is hypothesis.
