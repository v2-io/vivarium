---
slug: obs-coarse-view-draws-the-uncarved-prior
type: observation
status: exact
stage: draft
depends:
  - form-fidelity-invariant
  - form-core-view-wall
  - norm-no-depiction-without-referent
  - detail-erosion-composition
---

# A view coarser than the build level draws the uncarved prior, and says nothing

An `ErodedRegion` answers only at its own level or finer. Ask it one level coarser and it reports *not covered*, so the surface read falls through to the fated prior — the uncarved terrain. A store full of carved tiles can therefore render as a world where erosion never ran, with nothing in the picture or the return value to say so.

## Formal Expression

1. **The mechanism, exactly.** `ErodedRegion::grid_pos` returns `None` when `level < self.level` (`erosion.rs`). `covers`, `surface_m` and `tier_at` all route through it, so a coarser query is *uncovered*, and `surface_at` reaches its fallback: `gen::initial_topography_m(seed, cell, cell.level())`. That fallback is the correct answer for a region genuinely without a tile, and it is indistinguishable from this case.

2. **Convicted, both halves.** `erosion::tests::a_view_coarser_than_the_build_level_silently_draws_the_prior` builds a region carved to a constant far from any terrain, and asserts: at the build level the carved value returns and `tier_at` reports covering; one level coarser `tier_at` is `None` and the value is the prior to within $10^{-9}$. Either half failing refutes this observation.

3. **Why it is silent rather than wrong-looking.** The prior is not noise or a hole — it is a coherent, plausible planet, because it is the same fated field the carved tiles were carved *from*. So the failure presents as a world that simply has not eroded yet, which is a state the world genuinely passes through. Nothing about the image is anomalous; only its disagreement with the store is.

4. **Scope: this is a read-path fact, not an erosion-kernel fact.** The carved tiles are correct and present. What is missing is any signal, at the read boundary, distinguishing *no tile here* from *tile here, asked at the wrong level*. The fidelity ladder is supposed to answer "coarsest rung whose error stays in tolerance" ( #form-fidelity-invariant ); this path instead answers "finest rung, or the prior," with no rung in between and no report.

5. **What this does not claim.** Not that the level restriction is wrong: restricting downward is defensible, since a coarse cell spans many carved cells and returning one of them, or a mean, is a modelling decision nobody has made ( #form-rl-closure-algebra owns the restriction operator that would). The defect is the **silence**, not the restriction. Not that any particular view is currently mis-drawing — that depends on each view's level selection, which is not measured here.

## Epistemic Status

**Max attainable: exact** — a deterministic property of a pure function, decidable by construction rather than by sampling.

**Currently `exact`**, convicted by the unit test named in FE(2). Found 2026-07-24 by the explorer's provenance paint, which put build-state and surface-source side by side and made two adjacent lines contradict each other ("watered" against "100% prior"); the underlying cause was then read from the source rather than inferred from the symptom. Stage `draft`.

## Discussion

This is the failure mode #norm-no-depiction-without-referent exists to catch, arriving from the opposite direction than expected. That norm forbids drawing what the world does not contain; here a view draws something the world *does* contain — the prior is real, fated, and lawful — while the store holds a better answer it never asked for correctly. The picture has a referent. It is just not the referent the viewer believes they are looking at, and a trained eye cannot catch that, because an uneroded planet looks exactly like an uneroded planet.

Which is the argument for provenance as a first-class lens rather than a debug affordance: no amount of looking at the *surface* reveals this, and one glance at build-state beside it does. The instrument found a defect in its own reading of the world, which is the class of thing it was built for.

## Working Notes

- **The honest minimum is a report, not a fix:** any read that falls through to the prior while a covering tile exists at a finer level should be able to say so. Today the explorer names it on the HUD; nothing in the query path returns it, so every other consumer is on its own.
- **The real fix is a restriction operator** — a declared, keyed coarsening of carved tiles, so a coarse query gets a coarse *answer* rather than a fallback ( #form-rl-closure-algebra ; note #obs-mean-pin-manufactures-seam on what naive mean-pinning costs, so this is not free).
- Not audited: which levels each live view actually requests, and therefore how often this fires in practice.
