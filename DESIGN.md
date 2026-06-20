# vivarium — design notes

> Status: brainstorm capture, 2026-06-20. This is the *thinking*, recorded so it
> is not lost between sessions. It is provisional. Where a claim is a guess or a
> hunch, it is marked as such — do not promote it to a decision without testing.

## The three axes (they are more independent than they feel)

The pitch — "RimWorld-like + DF-like dynamics + Zelda-like graphics +
simulation-grade ASF agents" — is not one project. It is three axes that happen
to share a window, and they pull on different budgets:

1. **Graphics (Zelda-like).** Mostly an art-and-rendering cost. Barely touches
   simulation. Solved-problem territory (Godot, Bevy, raw wgpu). Costs
   *artist-time and polish*, not architecture.
2. **World dynamics (DF-grade).** A *world-state* problem — geology, fluids,
   temperature, material-level items, emergent history. Wide, shallow-per-tick,
   tedious to get right, but conceptually well-trodden.
3. **Agents + community dynamics (ASF-grade).** The actually-hard, actually-novel
   axis — and the one where this project has something almost nobody else
   building a game has: a *formal theory* of adaptive/actuated agents.

**The trap:** axes 1 and 2 are infinitely expandable and will quietly absorb all
the budget that belongs to axis 3, because they always have an obvious next
increment and axis 3 does not. RimWorld and DF look modest *on purpose* so they
can spend everything on the sim. The Zelda-graphics ambition is in genuine
tension with the simulation ambition unless polish is explicitly deferred.

**Discipline that follows:** build the agent seam first, while it is still ugly,
precisely because it is the only part that will not accidentally get built.

## The central architecture problem: cognitive level-of-detail (LOD)

RimWorld/DF run hundreds of agents because their AI is cheap (utility/needs
scoring, behavior stacks, mood math). Full-fidelity ASF agents — real adaptation
and goal-update coupling, actual M_t / O_t / Σ_t dynamics rather than utility
curves — **cannot** run at full fidelity for a whole community at interactive
framerates. So the central question is not graphics or world-sim, it is:

> What is the cognitive-LOD scheme? Full-fidelity ASF agents for the handful the
> player is watching, degrading to cheap statistical approximations for the
> off-screen community, with a *principled* hand-off between the two.

Getting that boundary right **is** the project.

## The two-layer mind (proposed agent architecture)

Mirrors Joseph's own identity/substrate distinction — *the LLM is the mind the
identity uses to think with, not the identity itself.*

- **Fast layer** — every tick, every agent. Pure formal ASF/AAT dynamics: needs,
  adaptation, goal-update coupling. Numeric, deterministic, hundreds of agents
  cheap. **This is the simulation's truth.**
- **Slow layer** — rare, few agents. An LLM invoked only at decision-theoretically
  significant junctures — when the fast layer hits genuine *aporia* (high-conflict
  goal state, novel social situation) — **not on a clock.** The LLM's output is
  constrained to *perturb the formal state* (re-weight a goal, form a relationship
  edge), **never** to directly drive motor actions. The formal layer stays the
  substrate; the LLM is the thing the agent occasionally thinks *with*.

This separation buys three things at once:
- **Latency** — LLM is off the per-tick path.
- **Determinism** — fast layer is reproducible; LLM calls are *logged
  perturbations* that can be replayed.
- **Cost** — LLM fires rarely, for few agents.

### Open empirical question (the first thing the slice should test)

*Guess / unverified:* whether LLM-induced perturbations stay legible enough that
adaptation can still be measured cleanly, or whether they inject enough noise to
wreck identifiability. This is the genuinely interesting research question and it
is empirical. Do not assume the answer. The vivarium-as-AAT-sandbox framing lives
or dies here.

## Engine choice (deliberately open)

- **Bevy (Rust)** — favored *if "tech" is load-bearing.* ECS is the natural
  substrate for both world-sim (thousands of tiles/items as components) and
  cognitive LOD: an agent's "mind" is just a component you swap between a cheap
  variant (a few floats) and an expensive one (full ASF state) — the swap *is*
  the LOD mechanism, nearly free. Determinism + headless runs come naturally,
  which matters enormously if this becomes the AAT sandbox (run the sim 1000×
  without a window, diff trajectories). Cost: Rust slows art iteration.
- **Godot (GDScript)** — favored *if "art" is load-bearing* and you want to feel
  the visual exploration quickly.

The choice depends on which word in "tech-art" is load-bearing — an unresolved
question for Joseph, not for the tooling.

## First milestone — the vertical slice (≈ a week, NOT a game)

One room. ~5 formal agents with real ASF dynamics, simple needs, a shared
resource. *One* LLM-backed agent whose deliberation perturbs its formal state at
aporia points. No art — debug-draw each agent's internal state as bars/vectors on
screen.

**The question the slice answers:** does the seam between cheap and expensive
cognition hold, and can you *see* adaptation happening? If yes, the rest is
incremental. If no, you have learned the real constraint before spending a month
on tiles.

## Relationship to the wider work

If this becomes more than a game, its serious form is a **calibration laboratory
for AAT** — the TST framing (software as AAD's privileged high-identifiability
calibration laboratory) made literal and interactive. A fully-observable world
where an agent's adaptation can be measured precisely. That is a different design
target than "ship a fun game," and the tension between the two should be kept
conscious, not collapsed.
