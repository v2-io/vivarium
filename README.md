# vivarium

*An enclosed living world kept for observation.*

A simulation world in the lineage of RimWorld (storytelling colony sim) and
Dwarf Fortress (deep emergent world dynamics): a deterministic 3-D voxel planet
(cube-sphere, 0.5 m at the finest rung) whose real bet is **simulation-grade
agents and community dynamics** built on the Agentic Systems Framework
(ASF/AAT) rather than the cheap utility-AI colony sims normally use. Vivarium
is a member of the **Archema** research program (`~/src/archema-io/`) and a
supporting project for ASF — see [`ASF.md`](ASF.md), read every session.

The name is deliberate: a *vivarium* is an enclosed space for keeping living
things **under observation**. The project is two things wearing one skin —

1. a game world (recreational, fun-led — see [`DESIGN.md`](DESIGN.md)), and
2. a high-identifiability laboratory in which ASF/AAT agent dynamics can be
   *measured*, because every quantity the theory needs is authored, known,
   and dialable.

That duality is the point, not an accident.

## Status (2026-07-10)

**The principled frame is standing and growing.** Engine = Bevy (decided
2026-06-22 by head-to-head spike). Proven physics — fluvial erosion and a
conserved shallow-water cycle — run in `crates/vivarium-world`, and the
framework proper landed as working code: a content-addressed **store** (the
save *is* the memo store), a lazy memoized **pull-query** layer, and a
**manifest** that individuates each world (its seed is its identity). Walk
away and return: the world persists. Current build target: an **ethereal
(observe-only) explorer in a Realized early-Abyssal world** —
[`ref/research/abyssal-parity-plan.md`](ref/research/abyssal-parity-plan.md).

**Start reading at [`ORIENTATION.md`](ORIENTATION.md)** (the current-state
map), then [`ARCHITECTURE.md`](ARCHITECTURE.md) (the consolidated frame).
Ethics of in-world agents: [`ETHICS.md`](ETHICS.md); the standing moratorium:
[`ASF.md`](ASF.md) §0.

Quick taste, no GPU needed:

```
cargo run --release -p vivarium-world --example store_explore
```
