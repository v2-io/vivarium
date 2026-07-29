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

**Max attainable: axiomatic** for the project as authored — a founding choice that makes laboratory replay possible, not a claim that nature is deterministic at this grain. Day-one commitment (DESIGN.md founding decisions; reaffirmed throughout). Stage `draft`.

**Known incomplete surfaces (not softens of the postulate):**
1. **Agent-layer RNG** in older `vivarium-core` paths is not fully fated (stepped stream; parallel agents break it). Ethereal exploration does not require that fix; participation does.
2. **No live view paces evolution by wall-clock.** The surviving instance was `spikes/worldview` (telescope re-anchor / settle cadence, contained under its own cache dir); that crate is archived and the sole view crate, `crates/vivarium-explore`, holds a read-only store and cannot advance anything ( #form-core-view-wall FE(6)). Remaining exposure is the archived spikes, which are out of the build and are not a surface anything depends on.

## Discussion

Without fated noise, the store cannot be the save, explorers and builders cannot share state by content address, and a probe that "passes once" is not a probe. Chance $\varepsilon$ for an inside agent is still real chance under a fixed housing rule; outside, it is lookup. That frame-relativity is intentional ( #def-in-vivia).

## Working Notes

- Mechanical bans (`thread_rng`, etc.) in world crates are consequences for the toolchain wave, not substitutes for this postulate.
- **The Level-4 handshake (ASF, verified exact — cite upstream, not re-proved here):** #asf/1-aat/deriv-mechanism-counterfactual-separation establishes that latent-anchored mechanism counterfactuals strictly exceed Pearl's counterfactual layer, and that latent-anchored content is exactly commitment to a coordinatization of the exogenous space. The $(\mathrm{seed}, k)$ scheme *is* such an authored coordinatization, so "the nomos + seed constitute the world's noumenon" is theorem-grounded, not intuition — and a "this very background, different law" query (swap a nomos version, keep seed + keys, rerun) is executable here and in no other artifact the program has. Cheap unrun demonstration: a nomos vs its key-permuted twin — indistinguishable at every observational/interventional layer, divergent under same-seed law-swap. Corollary already absorbed elsewhere: no distributional equivalence test can license cache reuse across a version swap; pointwise-on-keyed-draws is the unique sound rule, now mechanized by the source digest ( #form-complete-content-addressed-key ). ASF-side twin: `asf/doc/vivarium.md` §mechanism-counterfactual handshake.
- **World-side jitter engineering (from graduated DESIGN-REDUX §8 — not second law):** hash quality matters (avalanche finalizer; avoid axis-aligned lattice from naive linear congruential mixes); distribution must match phenomenon (FBM micro-relief, Gaussian scatter, blue-noise placement — uniform is rarely honest); same key family for agent decisions as `(agent_id, tick, purpose)`. Amplitude measurement path for discarded variance lives under #detail-info-theoretic-discretisation FE(5).
