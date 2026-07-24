---
slug: detail-fvm-control-volume
type: detail
status: robust-qualitative
stage: draft
depends:
  - form-column-control-volume
  - form-declared-structure-tradeoff
  - form-face-flux-register
  - form-core-view-wall
  - disc-prime-question
  - norm-declaration-must-convict
---

# FVM control-volume taxonomy (Cardiff) — constructions, reconstruction, null-space probe

Primary-read support from Cardiff & Demirdžić (2021) for what a control volume *is*, how reconstructions relate to cell averages, why FVM is the default for flux-seam work, and the eigenvalue probe for invisible modes. Supports #form-column-control-volume and structure-preserving declaration; does not re-open the grid decision.

## Formal Expression

1. **FDM vs FVM (one sentence).** FDM operates on **point values** and the **differential** form; FVM balances fluxes on **control volumes** and discretises the **integral** form. A uniform five-point stencil on undeclared values is finite differences wearing FV clothes — conservation only where explicitly enforced.

2. **Four components of every FV variant.** Control-volume construction; face gradient; stabilisation; solution methodology (Cardiff). Axes leak: stabilisation is a computational pathology cured by a mathematical device that silently alters physics — every leak is an unphysical term that must be written down (Prime Question / ASSUMPTIONS depth).

3. **Control-volume constructions.** Cell-centred (unknowns at centroids; CVs = primary cells); vertex-centred duals (unknowns at vertices; CVs on dual); staggered (different components different places); face-centred (rare). Mass-lumped FEM coincides with FV dual assignment — primal/dual through two doors.

4. **Volume-exact reconstruction.** Cell-centred FV: $\boldsymbol{u}(\boldsymbol{x})=\boldsymbol{u}_P+(\boldsymbol{x}-\boldsymbol{x}_P)\cdot(\nabla\boldsymbol{u})_P$. Integrated over the cell, the gradient term vanishes by centroid definition ⇒ **cell average of the reconstruction is exactly $u_P$**. Price: **discontinuous at faces**. The face jump is information (Godunov; refinement indicator), not a claim about continuous ground. Zeroth-order (flat) is Minecraft; add gradient and point-vs-average split. **Column = cell average (+ declared statistics); mesh continuity is a consumer/view choice** ( #form-column-control-volume ; #form-core-view-wall ).

5. **Declared vs enforced.** The project already named mass/volume as conserved primitive and elevation as derived center sample (DESIGN-MATERIAL lineage). Live defect class: declaration without mechanical conviction ( #norm-declaration-must-convict ) — code can drift through prose guards (mean-pin vs declared semantics).

6. **Null-space probe (instrument, not yet built as gate).** Periodic patch of a stencil; count zero eigenvalues of the discrete operator beyond rigid modes; extra zeros + eigenvectors name invisible modes (Cardiff). Run while the kernel can still fail.

7. **Why FVM for vivarium’s flux seams.** Strong local conservation (equal-opposite face fluxes); boundary conditions satisfied exactly independent of mesh density; hanging nodes / overset are first-class in the FV literature — our coarse↔fine tile seam and multi-grid bridges. Unity-weight FEM recovers FVM formally. **Decision is conservation-at-seams, not taste** ( #form-face-flux-register hanging-node bill still applies).

8. **Out of bounds.** Claiming every staggered scheme needs no stabilisation; labeling all artificial diffusion Rhie–Chow without family check; treating this taxonomy as a grid-choice reopening.

## Epistemic Status

**Max attainable: exact** for Cardiff attributions (primary-read 2026-07-12/13). Project mappings `robust-qualitative`. Stage `draft`. `detail` segment.

## Discussion

Field-nomos authors need this taxonomy before inventing a new “mesh meaning.” It is also the mechanical home of the null-space instrument the structure-trade segment demands.

## Working Notes

- **Source peel:** discretisation theory §2 (present-best; §2.4a/§2.5 correction archaeology stays history).
- **Water $\theta$ / Fr validity envelope:** DECISIONS + ASSUMPTIONS residual — not re-litigated here; θ’s claim is artificial diffusion, not a free physical advection stand-in.
- **Images:** Cardiff figures remain under `ref/research/pdfs/…`; this segment does not require them to hold.
