---
slug: worked-example-mfd-prime-question
type: worked-example
status: exact
stage: draft
depends:
  - disc-prime-question
  - disc-algorithms-disguise-physics
  - norm-bias-vs-noise
  - obs-cube-locked-kernel-bias
---

# Worked example: MFD as a disguised geometric claim

End-to-end specimen of the Prime Question applied to geometry: Multiple Flow Direction routing looked like a neighbour stencil; recovering the physical claim exposes a biased quadrature on the cube-sphere. Measurement home remains #obs-cube-locked-kernel-bias ; this segment is the teaching chain from paper costume → claim → bias class → honest kernel shape.

## Formal Expression

1. **Costume.** `erosion.rs` routes with **MFD** (Freeman/Quinn lineage): split outflow among 8 Moore neighbours, slope-weighted. Coordinate assumption: “8 neighbours, 45° apart, diagonals at $\mathrm{cell}\cdot\sqrt{2}$.”

2. **Physical claim recovered.** MFD is a **quadrature** of directional outflow; the eight cells are its **nodes**. The intended physics is: distribute outflow over downhill directions in proportion to slope (continuous angle), not “eight fixed compass bins.”

3. **What the costume smuggled.**
   - Four “neighbours” share only a **vertex** — zero-length edge; no flux crosses a point. Diagonals were a D8-anisotropy hack, not physics.
   - On the equiangular cube-sphere the nodes are **not evenly spaced** ⇒ the quadrature is **biased**, and the bias **does not converge away** ( #obs-cube-locked-kernel-bias FE(1)–(2)).

4. **Bias vs noise.** Same magnitude error would be noise if random; this one **manufactures a preferred direction** (cube $u/v$ attractors) and compounds downstream. Remedy class: not “small at this scale”; recover the claim and rebuild (edge fluxes from reconstructed gradient; real edges only) — successor **open**, not adopted here.

5. **Shape of the work.** This is #disc-algorithms-disguise-physics applied to **geometry** instead of constants: port the claim the algorithm was making, not the flat-grid node set.

## Epistemic Status

**Max attainable / currently `exact`** for the measurement-backed chain under the harnesses cited on #obs-cube-locked-kernel-bias . Teaching narrative extracted from discretisation theory §0. Stage `draft`. `worked-example` — validates the Prime Question method; does not adopt a router.

## Discussion

Without this specimen, the Prime Question stays abstract and MFD stays “a known bias row.” Together they show why valence-auditing never surfaces the defect and why true distances alone do not fix a fan that is not a fan.

## Working Notes

- **Numbers and cube-control rule:** only #obs-cube-locked-kernel-bias .
- **Source peel:** `doc/theory/discretisation-and-information.md` §0 worked example (present-best).
