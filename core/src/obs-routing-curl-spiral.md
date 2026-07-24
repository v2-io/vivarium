---
slug: obs-routing-curl-spiral
type: observation
status: exact
stage: draft
depends:
  - obs-cube-locked-kernel-bias
  - form-declared-structure-tradeoff
  - norm-probe-sensitivity
  - norm-bias-vs-noise
---

# Routing violates the contour-orthogonality identity (the flow spirals)

Measured: ~2% of routed flux runs *around* the hill instead of down it — a level-independent, topological, signed bias. The identity it violates had to be restated before it could be tested, and the obvious probe location is a null test that would have acquitted the router.

## Formal Expression

1. **The identity (restated — the naive form is false).** "Gravity-driven flow is curl-free" is **wrong**: real discharge is $q = -K\nabla\phi$ with varying conveyance $K$, so $\nabla \times q = -\nabla K \times \nabla\phi \neq 0$ — real water on a real hillslope *has* curl. The exact, pointwise, mesh-independent statement is: **flow direction is orthogonal to the contours of $\phi$**, i.e. $$\kappa(C) = \oint \hat d \cdot d\ell \Big/ \oint d\ell = \langle \sin\Delta\rangle = 0$$ on every closed contour. $\kappa$ is the fraction of transport running *along* the contour instead of down the slope; $\kappa \neq 0$ means the routed field spirals — it is not the gradient flow of the potential it claims (an Escher staircase).
2. **Measured violation.** $\kappa \approx 2.0\times 10^{-2}$ on the cone; median $1.9\times 10^{-2}$ over traced contours on real terrain. **Level-independent** (identical L9→L23, a $16\,384\times$ refinement) while the probe's own error falls five orders to machine zero. The spurious vorticity is a smooth, coherent, **signed** field — null island at the face centre, CCW over one half of the face, CW over the other, strengthening outward. Bias, topological; refinement never touches it ( #norm-bias-vs-noise ).
3. **The natural probe location is a null test.** A cone at a face centre is D4-symmetric: mirror symmetry forces $\oint \sin\Delta\, d\ell = 0$ **exactly**. Measured $\kappa = -1.9\times10^{-10}$ at the face centre while a $5.9°$ pointwise bias sits right there. The measurement exists only off-symmetry — where real terrain is worst. A face-centred probe *acquits* a defective router ( #norm-probe-sensitivity ).
4. **The proposed replacement does not fix it.** Gradient-projected edge flux: $\kappa = 4.5\times10^{-3}$ — $4.5\times$ better, **not** zero. The circulation is manufactured by the *split* of mass among discrete receivers, not by the direction estimate alone.
5. **Remedy shape (proposed, unratified).** A first-moment condition on the outflow weights — $\sum_k w_k \sin(\beta_k - \psi) = 0$ (vanishing tangential moment), imposable by a minimum-relative-entropy tilt: measured $\kappa \to 5\times10^{-6}$, deflection $0.02°$, conservation exact to twelve figures, any valence. MFD never imposed the condition; it *assumed* eight nodes $45°$ apart delivered it for free.
6. **Circulation and plume drift are different defects.** They do not rank routers the same way (edge flux: ~$900\times$ the circulation of moment-corrected MFD, yet less than half the drift). Drift is the convergent part; $\kappa$ the solenoidal part (Helmholtz split). The fan probe ( #obs-cube-locked-kernel-bias ) and the curl probe do not subsume each other.
7. **Severity is unpriced.** Both attempts to compose the moment fix with edge flux under-performed on catchment error. The pricing experiment — eroded landscape with and without the moment correction; does the channel network differ? — has not run. Do not act on severity until it does.

## Epistemic Status

**Max attainable: exact** for the measurements under their harness (`examples/curl_probe/`; predictions written before the first run in `msc/spike-curl-probe/PREDICTIONS.md`). **Currently `exact` as observation** of the live router's defect and of the null-test structure. The remedy (FE 5) and any adoption verdict are **proposed** — `DECISIONS[routing-violates-the-potential-identity-and-the-replacement-does-not-fix-it]`, `:by claude`, proposed; the router itself remains open ( #form-grid-equiangular-staggered FE(4)). Priority: correct work, not critical-path — erosion-carving is a Phase-3 gate and `emerged-land` is the queue head (`check-the-ladder` lineage).

Stage `draft`.

## Discussion

Two independent lessons ride on one measurement. First, the identity itself had to be corrected before it could convict anything — a probe built on the brief's "curl-free" would have convicted a perfect router; the LEM survey reached the same correction independently from the literature. Second, the natural probe location was an exact null test — the same species as a green probe sampling below the fault frequency ( #norm-probe-sensitivity ), demonstrated rather than predicted. Together with the fan and uniform-area biases ( #obs-cube-locked-kernel-bias ) this completes the measured catalogue of cube-locked routing defects: attractor fan, fake erodibility, and manufactured spiral.

## Working Notes

- **Instruments:** `crates/vivarium-world/examples/curl_probe/` (identity restatement in `curl.rs` module doc); `msc/spike-curl-probe/PREDICTIONS.md`; DECISIONS long form is history.
- **Pricing experiment owed:** channel-network diff with/without moment correction (~a day) before any severity claim.
- **Self-catch preserved as regression guard:** true-width quadrature was non-deterministic run-to-run (HashMap-order branch cut); its first, flattering numbers were the bug. Re-run new probes 3× before believing them.
- **Do not re-claim:** "gravity-driven flow is curl-free"; "fixing direction fixes magnitude"; the 5.17%-vs-20.76% headline without the null-test caveat.
