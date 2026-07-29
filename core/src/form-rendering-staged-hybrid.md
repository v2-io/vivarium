---
slug: form-rendering-staged-hybrid
type: formulation
status: conditional
stage: draft
depends:
  - form-engine-bevy
  - form-core-view-wall
---

# Staged hybrid rendering to the horizon

First-person rendering reaches the horizon by a staged hybrid — a near diggable voxel band plus a far field **regenerated deterministically at coarse stride, never cached** — ratified as the rendering architecture on 2026-06-23.

## Formal Expression

1. **The architecture.** Near band: full-detail diggable voxels (fixed-footprint chunks). Far band: a coarse representation built from the world's surface fields, **regenerated at coarse stride from the deterministic core rather than cached** — the far representation is a declared statistical summary of the near voxels it stands in for (an instance of #form-fidelity-invariant).
2. **The stages.** **v1** — self-built far-heightfield backdrop from `surface_height`, one mesh, no new dependency (implemented 2026-06-23 in the `bevy-voxel` spike, since archived). **v2** — geometry clipmaps (camera-centred rings) when unbounded / planet-scale reach is wanted. **v3** — volumetric far (octree LOD or GPU raymarch) **only if** distant overhangs ever matter; deliberately last because it is the budget sinkhole.
3. **The grounding asymmetry.** Cache-based far-field systems (Distant Horizons lineage) pay staleness, no-replay, and only-terrain-you-visited because they cannot cheaply recompute. A pure-function core sidesteps all three: the far band is a deterministic query, not a stale copy.
4. **Named tension, open.** FE(3) assumed an $O(1)$ analytic `surface_height`. The eroded tier is now **materialized-only** — minutes to recompute, read from the store — so a far band over an eroded world must read store cohorts, re-importing the fidelity and staleness choices FE(3) claimed to sidestep. A far read coarser than the build level draws the uncarved prior ( #obs-coarse-view-draws-the-uncarved-prior) — the far band must therefore declare which cohort and level it summarizes, like any other view.

## Epistemic Status

**Max attainable: exact** for the decision record. **Currently `conditional`**, on two named conditions: (a) first-person view work resuming — the live explorer (`vivarium explore`) is the globe instrument and does not yet reach a first-person horizon, so nothing currently implements this segment; (b) FE(4)'s tension being resolved for the eroded tier, which did not exist when the decision was made.

Authority: **ratified by Joseph 2026-06-23** (recorded in `ref/rendering/NOTES.md`, the synthesis of two research sweeps, with the heightfield-far trade accepted for v1/v2). The crate-level claims there (`bevy_voxel_world`'s fixed-chunk ceiling, verified in its source; `bevy_terrain` pinning) are dated 2026-06-23 — re-verify against current crate versions before building on them.

Stage `draft`.

## Discussion

This is the one ratified architecture decision for what a *player's eye* will see; it fell out of the index when front doors were archived, which is why it lands as a segment now, ahead of any implementation. The core/view wall shapes it: every stage is "our own view over our own field," and the far band is a peer consumer with a declared reconstruction, not a privileged renderer.

## Working Notes

- Full option table, scoring, and the rejected paths (bevy_terrain, SVDAG raymarch with Rust references): `ref/rendering/NOTES.md`.
- v1's open items when archived: the near/far seam (blocky voxels against smooth far mesh), a stepped far-shader with unified palette.
- FE(4) is the item to resolve first when this work resumes; it likely becomes a cohort-keyed far query rather than an analytic one, at which point FE(3)'s wording needs re-truing.
