# Predictions — written BEFORE the first number was measured

*Committed 2026-07-13, before `examples/curl_probe` was run even once. The project was
burned by a "22888" that was believed because it confirmed the prior
(`DECISIONS[seam-probes-were-measuring-seabed]`). The cheapest defence against that is to
write down what you expect **first**, including the shapes in which you expect to be
**wrong**, so that a confirming number has to survive a prediction it could have broken.*

## What the probe is

The physics: gravity-driven flow is a **gradient flow** on the hydraulic head φ. The exact,
topological content of that is **not** "the discharge is curl-free" (it is not — `q = −K∇φ`
with varying conveyance K has `∇×q = −∇K×∇φ ≠ 0`). The exact content is:

> **The flow direction is everywhere PARALLEL to −∇φ, hence everywhere ORTHOGONAL to the
> contours of φ. Therefore the circulation of the flow direction around any closed CONTOUR
> of φ is identically zero — pointwise, so on every mesh, at every resolution, exactly.**

So the invariant is

    κ(C) = ∮_C d̂ · dl / ∮_C dl        C a closed contour of φ,  d̂ the router's unit outflow direction
         = ⟨ sin Δ ⟩                    Δ = signed angle from −∇̂φ to d̂

**κ ≠ 0 ⟺ there is NET tangential transport around the hill ⟺ the router's direction field
admits no single-valued potential** (its implied head is multivalued — an Escher staircase).
That is the violated identity, and it is categorically worse than an accuracy error.

## Predictions

- **P1 — the face-centred cone returns κ ≈ 0, and that is a THEOREM, not a pass.**
  The (equiangular cube face + cone at the face centre) configuration is invariant under the
  face's D4 group. A mirror maps the contour to itself with reversed orientation and flips the
  chirality, so Δ → −Δ. Hence ∮ sin Δ dl = −∮ sin Δ dl = 0 **exactly, by symmetry.**
  ⇒ **This is a NULL TEST. If I had only run the obvious cone I would have measured zero and
  wrongly declared the identity safe.** It is being run as an operator control, not as evidence.

- **P2 — a GENERIC pole breaks the symmetry and κ ≠ 0.** This is the real test.

- **P3 — the local (plaquette) curl density is nonzero almost everywhere, and is
  LEVEL-INDEPENDENT** (it inherits the fan bias's Jacobian limit, which contains no N).
  Zero at the face centre; rising outward.

- **P4 — the discriminating control must return κ ≈ 0 with a LARGE pointwise deflection.**
  Feed the operator a deflection Δ(x) = 15°·cos(2ψ) about the cone axis: big everywhere,
  but odd around the loop. If the probe reports circulation here, it is measuring **deflection
  magnitude**, not circulation, and it is merely re-running the fan probe under a new name.
  **This control is the one that decides whether the probe is a probe at all.**

- **P5 — gradient-projected edge flux ≪ MFD-8.**

## The shape of the honest negative I am prepared to report

The fan bias makes the cube-face axes **attractors**. An attractor is a **convergent**
(irrotational) deformation — it can drift a plume 474 km without any circulation whatsoever.
**So the measured plume drift does NOT imply nonzero curl, and I must not let it stand in as
evidence.** If the deflection field turns out to be (near-)integrable — a pure gradient
perturbation, sign-cancelling around every loop — then the answer is **NO**, the potential
structure survives, the fan bias is real but not topological, and the urgency drops.
I will say so plainly.

## The trap I am most likely to fall into

Believing a nonzero κ that is actually my own quadrature error. Guard: the exact-gradient
control must return **machine zero structurally** (every summand identically zero), and the
operator's own discretisation residual must be shown to **converge away** under refinement
while MFD's signal does not.
