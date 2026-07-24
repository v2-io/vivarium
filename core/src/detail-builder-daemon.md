---
slug: detail-builder-daemon
type: detail
status: discussion-grade
stage: draft
depends:
  - form-builder-admission
  - form-store-as-save
  - form-depend-by-key-never-latest
  - form-complete-content-addressed-key
  - form-core-view-wall
  - form-three-scoped-runtime
  - detail-drainage-dependency-planning
---

# Builder daemon design — store as bus, beacons, demand spool

Unbuilt operational design for multi-process builder/explorer coordination. Thin admission law: #form-builder-admission . OUTLINE gap "full builder daemon" is this surface.

## Formal Expression

1. **Store is the bus.** No mediator daemon for truth: immutable content-addressed objects + atomic root swaps; readers safe against writers without protocol. Two processes computing the same key write identical bytes — benign race by construction.

2. **Four pieces.**
   - **Vivium spec** — identity (in every key; change = fork), label (never keyed), demand (target phase, **beacons**, watchpoints — change build *order*, never the world under #form-depend-by-key-never-latest ).
   - **Builder daemon** — owns *scheduling*, never truth: demand frontier (beacons → causal cones → work queue), puts memos, log, status; lockfile-attach (second build attaches, does not fail); pause cheap (completed tiles already in store).
   - **Explorers** — any number, read-only; Hit or instant prior; miss drops **demand file** to spool builder sweeps; work with no builder (coarse); never block.
   - **Instruments** — fidelity pyramid (histogram roots by level×stage); watchpoints (declared place/level/stage snapshots); telemetry by construction (intermediates immutable).

3. **Scheduling.** Unmet frontier of demanded cones; per-beacon depth-first vs breadth-first; Hilbert for storage locality; **dependency structure is drainage-shaped** ( #detail-drainage-dependency-planning ) — two structures, neither substitutes.

4. **Access profiles = process boundaries.** Builder holds write + nomos; explorer holds phenomenal query API — OS boundary instantiates exo access split. Tripwire: two-process store sharing may force mechanism-enforced canon-root guard earlier than first graduation.

5. **Temporal ladder (stage chains).** Macro-erosion epochs → macro water-cycle + alluvial coupling (slow erosion sees time-averaged discharge) → human-scale live water entered warm (settled fill memoized). Each rung a keyed nomos.

6. **Initial slice order.** Spec+seed landed; next: CLI build/status/attach; demand spool + read-only query; time-indexed stages; ethereal explorer (globe-first as spine visible win).

7. **Open.** Demand-spool format; when canon-root guard becomes mechanism; fork UX (BREAK-5); multi-machine later.

## Epistemic Status

**Max attainable: discussion-grade** as unbuilt design. **Currently `discussion-grade`.** Stage `draft`.

## Discussion

Build-order independence is the guarantee; beacons are the UX of demand without a second truth channel.

## Working Notes

- Supersedes builder-explorer-decoupling as design home.
- Do not claim daemon shipped under #form-builder-admission .
