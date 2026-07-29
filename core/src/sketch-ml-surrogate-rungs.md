---
slug: sketch-ml-surrogate-rungs
type: sketch
status: sketch
stage: draft
depends:
  - form-fidelity-ladder
  - form-complete-content-addressed-key
  - post-determinism-as-ontology
  - norm-regime-probes
---

# Learned surrogates as fidelity-ladder rungs

A machine-learned intermediary — trained on full-simulation corpus, generalizing beyond memoization, far cheaper than the law it stands in for — is a fidelity-ladder rung like any other: model identity in the key, conservation by architecture, convicted by the same probes, with its training distribution as the *declared envelope* and a per-query rarity self-report as the envelope's tripwire.

## Formal Expression (sketch — nothing here is built)

1. **The frame is already law.** #form-fidelity-ladder names the shape: climb to the expensive rung to discover, descend to a probe-validated surrogate, keep the expensive rung as calibrator. A learned surrogate *is* that surrogate. Its identity is the content-addressed weights plus the keyed training-corpus set, folded into the complete key ( #form-complete-content-addressed-key ) — a trained model is a nomos version, and retraining is a version bump that invalidates like any other.
2. **The corpus advantage is structural.** The usual surrogate bottleneck — scarce, noisy labels — vanishes here: a deterministic fated generator produces unlimited, perfectly-labeled, *keyed* training pairs, and the time-indexed stage chains are already labeled trajectories sitting in the store.
3. **Inference must be fated.** A surrogate rung obeys #post-determinism-as-ontology like any kernel: bit-deterministic inference (CPU, or quantized-integer paths; commodity GPU inference is generally *not* bit-stable and is out unless proven otherwise). Weights are data in the key; a draw is a pure function of (seed, key) exactly as before.
4. **Conservation is architectural, never hoped-for.** A surrogate that predicts *states* can leak mass silently. A surrogate that predicts **face fluxes**, applied through the single-valued register, conserves exactly *by construction* — the flux web dictates which outputs to constrain. Same for increments over totals. The laws choose the output representation.
5. **Conviction is unchanged.** A surrogate rung ships with regime probes that can fail it ( #norm-regime-probes ), is judged by structural statistics (the chaos floor already removed pointwise agreement from every method's reach), and its bounded deficiency versus the teacher rung is a declared budget — the fidelity invariant applied to a learned materialization.
6. **The envelope self-reports (Joseph, 2026-07-29 — the load-bearing addition).** The hard failure mode is out-of-distribution *silence*: a surrogate off its training band is wrong without any signal — undiscoverably unlawful. Proposed mechanism: **random forests, with every leaf carrying the fraction of the *test* corpus that fell into it.** Forests bring the flexibility wanted at this stage — insensitive to confounding variables and red herrings, no normalization to get wrong, native feature-importance as a standing red-herring audit — and the leaf-coverage counts turn the envelope into a per-query instrument: a prediction arrives with an aggregate "how rare is this situation to this model," so rarity can gate, flag, or refuse. That converts the declared-envelope *discipline* into a *mechanism* — a declaration that can fail per query, not a paragraph in a ledger.
7. **Rung shapes worth distinguishing (judgment, not law).** (a) **Settle-operators** — one-shot maps from (initial state, drivers, parameters) to the *converged* state — avoid the autoregressive-rollout drift that limits step-emulators, and the erosion settle histories are their ready-made corpus. (b) **Sub-grid closures** — the standard learned-parameterization shape, and the form the nonlocal-closure frontier ( #sketch-detail-abstract-reversion FE(6)) would take if learned. (c) **Field-to-field emulators** (neural-operator class) — the weather shape; the stated intent is that this is where the approach earns its keep when atmospheric systems arrive. Forest-first serves (a) and (b) now; (c) may want different machinery later, under the same rung law.
8. **Moral scope: none.** A trained emulator is a pure function — separation by construction, GUC Class 1 — out of scope by architecture, same as the formal fast layer. Nothing here approaches the agent seam.

## Epistemic Status

**Max attainable: `formulation` (exact)** for the rung law (FE 1, 3–5), once a first surrogate exists and its key/probe/envelope machinery is demonstrated. **Currently `sketch` throughout** — nothing is built, no corpus has been extracted, no forest trained.

Provenance, by register: FE(1)–(5) apply existing project law to a new rung kind (checkable against the cited segments now). FE(6)'s forest-plus-leaf-coverage mechanism is **Joseph's proposal (2026-07-29)**, recorded at proposal strength; the aggregation math (how per-tree leaf fractions compose into one rarity score — geometric mean? minimum? calibrated?) is open, and leaf coverage is a *density proxy*, not a calibrated confidence — the distinction must survive into any HUD that displays it. FE(7)'s field context is **training-knowledge, not read-primary**: the flagship results (weather emulators at ~10³ cost reduction; hybrid cores with learned closures; graph-network simulators; neural operators) are from the assistant's general knowledge and are *leads to register and read, not citable authority* — no paper named here has been read primary in this repository.

Stage `draft`.

## Discussion

Why this is worth a segment before any code exists: the project's hardest-won laws — complete keys, fated draws, flux-form conservation, regime probes, declared envelopes — are precisely the preconditions the surrogate literature struggles to retrofit. Vivarium doesn't need to bolt honesty onto a learned emulator; the emulator inherits it by being forced into the rung shape. The one genuinely new mechanism needed — the envelope tripwire — is FE(6), and it is the difference between a surrogate that fails loudly at its boundary and one that manufactures quiet unlawfulness beyond it.

## Working Notes

- **First-rung candidate (judgment):** the erosion settle-operator — corpus exists (stage chains), conviction criteria exist (χ-shape, structural statistics), payoff lands on the era ladder (stage-parallel cold-stage carves at trivial marginal cost). Not decided; weather remains the stated destination.
- **Open before any build:** leaf-coverage aggregation math; the train/test/probe corpus split under keyed determinism (seeds partition cleanly — use that); where the rarity threshold lives (manifest? NomosDecl envelope field?).
- **Register the leads in relata before citing any of them:** the weather-emulator, hybrid-core, graph-simulator, and neural-operator papers named in session are training-knowledge only.
- The kernel-era expiry of #obs-coarse-only-closure-nogo (2026-07-29) is a live caution for this segment's whole class: a learned closure's validity is a property of the kernel era that trained it — the key machinery (FE 1) handles invalidation *mechanically* only if the training-corpus keys genuinely enter the surrogate's key.
