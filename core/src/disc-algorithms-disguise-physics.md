---
slug: disc-algorithms-disguise-physics
type: discussion
status: robust-qualitative
stage: draft
depends:
  - norm-probes-before-claims
---

# Algorithms are disguised physical claims

When a published algorithm's coordinate assumptions do not survive the vivium's topology or multiscale demands, recover the physics the algorithm was asserting and build the algorithm that does that physics here — do not bend the world to the paper's grid habits.

## Formal Expression

1. **Port the claim, not the costume.** An algorithm carries a physical claim (what is conserved, how flux splits, what the stencil approximates). That claim is what transfers. The particular neighbour fan, index tricks, or paper-default grid are not sacred.
2. **No silent violation.** If the algorithm's assumptions fail on the cube-sphere or at a seam, the responses that are **out of bounds** are: (a) patch parameters until the picture looks right, (b) wave the violation through as "close enough," (c) contort the grid solely to host an unmodified paper method.
3. **In-bounds response.** Name the physical claim; derive or select a scheme that asserts that claim under our operators and topology; probe it.

## Epistemic Status

**Max attainable: robust-qualitative** as standing method (Joseph-advocated; `DECISIONS[algorithms-are-disguised-physical-claims]`, `:by us`, decided). Worked examples (MFD-as-quadrature on cube-sphere) support the method but are not re-proved in this segment. Stage `draft`. Individual scheme replacements need their own segments and probes.

## Discussion

This is the anti-pattern to "do what the paper said, plus alterations to work in our system." The paper's authority is for the physics it got right, not for its Fortran neighbourhood.

## Working Notes

- Grid *verdict* (which grid to keep) remains open where Joseph has not closed it; this segment does not close the grid question. See `DECISIONS[grid-question-not-closed-authority-was-inflated]`.
