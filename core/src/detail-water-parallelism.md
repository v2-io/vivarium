---
slug: detail-water-parallelism
type: detail
status: discussion-grade
stage: draft
depends:
  - post-determinism-as-ontology
  - form-complete-content-addressed-key
  - form-fidelity-ladder
  - form-flux-web
  - norm-regime-probes
---

# Water parallelism — gather first; CPU reference; GPU as keyed rung

Intent for water performance: structural precondition, staging, and determinism policy. Not a task list.

## Formal Expression

1. **Why.** Pipes kernel is the hot loop; fine nested water and fast-forward want orders of magnitude more throughput; work is stencil-regular.

2. **Structural precondition: gather, not scatter.** Sediment advection that scatters (push to neighbours) must become gather (pull neighbour fluxes) before rayon or GPU. Rewrite once on the **CPU reference**, determinism-preserving, shared prep for all backends.

3. **Staging.** (1) Gather + row-parallel rayon on CPU. (2) Optional wgpu compute (virtual-pipes lineage is GPU-native); memory-bound stencils; validate against CPU.

4. **Determinism policy.** CPU kernel is the **reference** for tests, replay, probes — never goes away. GPU is a **rung behind the same flux interface** ( #form-fidelity-ladder ); backend identity is in the nomos key so caches never silently mix backends. Validation by probe (conservation, regime invariants, fingerprints) within **written** tolerance; divergence is a rung bug.

5. **What stays CPU.** Priority-Flood and elevation-ordered stream-power are sequential/rare/cached. Far-LOD sampling GPU only after measured need.

6. **Placement.** `vivarium-world` stays dependency-free; wgpu lives in a separate crate (or spike while spike-grade).

## Epistemic Status

**Max attainable: discussion-grade** as performance design. **Currently `discussion-grade`.** Stage `draft`. Speedups unmeasured projections.

## Discussion

Determinism-as-ontology is not negotiable; GPUs are bit-reproducible per device, not across devices — the reference+key policy is the answer.

## Working Notes

- Supersedes water-parallelism plan as home.
