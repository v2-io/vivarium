---
slug: post-determinism-as-ontology
type: postulate
status: axiomatic
stage: draft
depends: []
---

# Determinism as ontology

The world is a pure function of identity: stochastic detail is fated by seed and key, not sampled from wall-clock entropy.

## Formal Expression

1. **Fated noise.** Every aleatoric quantity that enters world law is a pure function of $(\mathrm{seed}, k)$ for a content key $k$. The dictionary form is `#lexicon/term/fated-noise` (settled): a stateless KRNG/DRNG keyed on identity — fixed, order-independent, the same forever. *"Fate"* as a noun is not adopted.
2. **No exogenous entropy in law.** Wall-clock time, OS entropy, shared mutable RNG streams, and scheduler nondeterminism must not affect world state. Views may use wall-clock for camera feel only.
3. **Memoization soundness.** Because lifts and draws are fated, two evaluations of the same complete key agree; content-addressed memoization changes cost, never the world.
4. **Ensembles over seeds.** Distributional claims vary seed (or an explicit key parameter), not re-rolls of a cell under a shared stream.

## Epistemic Status

**Max attainable: axiomatic** for the project as authored — a founding choice that makes laboratory replay possible, not a claim that nature is deterministic at this grain. Day-one commitment (DESIGN.md founding decisions; reaffirmed throughout). **Known incomplete surface:** agent-layer RNG in older `vivarium-core` paths is not fully fated (stepped stream; parallel agents break it); ethereal exploration does not require that fix, but participation does. Stage `draft`.

## Discussion

Without fated noise, the store cannot be the save, explorers and builders cannot share state by content address, and a probe that "passes once" is not a probe. Chance $\varepsilon$ for an inside agent is still real chance under a fixed housing rule; outside, it is lookup. That frame-relativity is intentional ( #def-in-vivia).

## Working Notes

- Mechanical bans (`thread_rng`, etc.) in world crates are consequences for the toolchain wave, not substitutes for this postulate.
