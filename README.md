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

1. a game world (recreational, fun-led — see [`doc/design/DESIGN.md`](doc/design/DESIGN.md)), and
2. a high-identifiability laboratory in which ASF/AAT agent dynamics can be
   *measured*, because every quantity the theory needs is authored, known,
   and dialable.

That duality is the point, not an accident.

## Epistemic honesty is enforced in code, not culture

Every algorithm that shapes a world — a **nomos** (pl. *nomoi*): one keyed,
versioned, executable article of world-law — must *declare itself* in the
**nomotheke** (`crates/vivarium-world/src/nomotheke.rs`): its epistemic tags
(Earth-fidelity and physics tiers, relation, verification status), its
dependencies, its **bequests** (each with an explicit conservation claim —
"are we conserving mass?" is a lookup, never archaeology), and its
**assumptions** (anchors into [`ASSUMPTIONS.md`](ASSUMPTIONS.md), the canonical
magic-constant ledger). The enforcement is structural:

- **Declarations mint the store keys** — an undeclared algorithm has no path
  into a world's law namespace.
- **Derived quality is computed, not asserted**: a weakest-link fold over the
  dependency graph, so good physics run on a placeholder yields
  placeholder-grade state, and the `vivarium status` pyramid prints both
  (declared/derived) per nomos — the honesty column.
- **The ledger compiles into the test suite** — a magic constant renamed out
  from under a nomos fails the build's tests. An unprincipled value is not the
  sin; an *unaccounted* one is: undeclared constants are latent,
  *undiscoverably* unLawful.
- **Declarations pair with probes** (`crates/vivarium-world/examples/` —
  renderer-free instruments asserting invariants nature guarantees): the
  declared tier is a falsifiable claim, and the probe is what would convict it.

By forcing the epistemology to be asserted, it becomes strictly auditable and
falsifiable — audit is a query over the store, not a review of intentions.

## Status (2026-07-11)

**The principled frame is standing and growing.** Engine = Bevy (decided
2026-06-22 by head-to-head spike). Proven physics — fluvial erosion and a
conserved shallow-water cycle — run in `crates/vivarium-world`, and the
framework proper landed as working code: a content-addressed **store** (the
save *is* the memo store), a lazy memoized **pull-query** layer, and a
**manifest** that individuates each world (its seed is its identity). Walk
away and return: the world persists. Current build target: an **ethereal
(observe-only) explorer in a Realized early-Abyssal world** —
[`doc/plan/abyssal-parity-plan.md`](doc/plan/abyssal-parity-plan.md).

**Start reading at [`ORIENTATION.md`](ORIENTATION.md)** (the current-state
map), then [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) (the consolidated frame).
The tree in one line: root = front doors + ledgers · `doc/` = the durable
thinking (design / plan / theory) · `ref/` = true reference · `.archive/` =
sunset material, indexed (current state lives in the main tree; trails and
provenance live there and in git history). Agent
onboarding tiers: [`CLAUDE.md`](CLAUDE.md); working norms:
[`doc/PROCESS.udon`](doc/PROCESS.udon). Ethics of in-world agents:
[`ETHICS.md`](ETHICS.md); the standing moratorium: [`ASF.md`](ASF.md) §0.

Quick taste, no GPU needed:

```
cargo run --release -p vivarium-world --example store_explore
```
