---
slug: obs-routing-curl-spiral
type: observation
status: exact
stage: draft
depends:
  - obs-cube-locked-kernel-bias
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
5. **What $\kappa$ actually measured (bound on FE(1)–(4)).** The probe read the tangential component of the **fan-weighted transport direction** — a scalar accumulation's implied direction, which is **not** the physical flow direction. The router's output is a boundary integral, ill-posed *in the continuum* (its value depends on cell shape and orientation — Coatléven & Chauveau, read primary); the physical direction is the **reconstructed flux vector** $\hat Q_K = Q_K/\lVert Q_K\rVert$. A first-moment tilt on the fan weights drove $\kappa \to 5\times10^{-6}$ but broke magnitude — because it forced *the wrong object* to have the right first moment. The apparent two-fixes-interfere paradox is dissolved: it was one fix stated in the wrong coordinates (`DECISIONS[the-router-is-a-scalar-pretending-to-be-a-vector-and-p-is-the-bias]` ④, `DECISIONS[mfds-output-is-not-a-discharge-it-is-a-boundary-integral]`).
6. **Remedy stack (proposed, unratified — composes in order).** (a) Set $p=1$ (weights $\propto \cos^q$ have exactly zero first moment at $q=1$ — a theorem; the measured $0.24°$ "intrinsic" lattice baseline *is* the $p=1.1$ exponent, vanishing at $p=1$); (b) **kill the diagonals** (they cross no face — 47.8% phantom flux — and make MFD un-correctable as an FV scheme); (c) **then** apply the Coatléven vector reconstruction as a no-solve post-process on real face fluxes (direction and magnitude from the same object, so they cannot fight; the identity needs no orthogonality and converges at 2nd order on our cells); (d) separately, the non-orthogonality-corrected face gradient — a distinct error with a distinct fix. The superseded moment-tilt is retained only as the instrument that located the disease.
7. **Circulation and plume drift are different defects.** They do not rank routers the same way (edge flux: ~$900\times$ the circulation of moment-corrected MFD, yet less than half the drift) — convergent vs solenoidal parts (Helmholtz split). The fan probe ( #obs-cube-locked-kernel-bias ) and the curl probe do not subsume each other. Under refinement, every fan-based variant's catchment error **grows**; only the real-face FV router is flat.
8. **Severity is unpriced.** The pricing experiment — eroded landscape with and without the corrected router; does the channel network differ? — has not run. Do not act on severity until it does.

## Epistemic Status

**Max attainable: exact** for the measurements under their harness (`crates/vivarium-world/examples/curl_probe/`; predictions written before the first run in `msc/spike-curl-probe/PREDICTIONS.md`; router-spike measurements in `msc/spike-principled-router/` + `examples/router_lab/`). **Currently `exact` as observation** of the live router's defect and of the null-test structure — with the FE(5) bound: $\kappa$ convicts the fan-weighted direction, not the reconstructed physical vector. The remedy stack (FE 6) and any adoption verdict are **proposed** — DECISIONS family `routing-violates-the-potential-identity…`, `the-router-is-a-scalar…`, `mfds-output-is-not-a-discharge…`, all `:by claude`, proposed; the router itself remains open ( #form-grid-equiangular-staggered FE(4)). Priority: correct work, not critical-path — erosion-carving is a Phase-3 gate and `emerged-land` is the queue head (`check-the-ladder` lineage).

Stage `draft`.

## Discussion

Two independent lessons ride on one measurement. First, the identity itself had to be corrected before it could convict anything — a probe built on the brief's "curl-free" would have convicted a perfect router; the LEM survey reached the same correction independently from the literature. Second, the natural probe location was an exact null test — the same species as a green probe sampling below the fault frequency ( #norm-probe-sensitivity ), demonstrated rather than predicted. Together with the fan and uniform-area biases ( #obs-cube-locked-kernel-bias ) this completes the measured catalogue of cube-locked routing defects: attractor fan, fake erodibility, and manufactured spiral.

## Working Notes

- **Instruments:** `crates/vivarium-world/examples/curl_probe/` (identity restatement in `curl.rs` module doc); `msc/spike-curl-probe/PREDICTIONS.md`; DECISIONS long form is history.
- **Pricing experiment owed:** channel-network diff with/without the corrected router (~a day) before any severity claim.
- **Self-catch preserved as regression guard:** true-width quadrature was non-deterministic run-to-run (HashMap-order branch cut); its first, flattering numbers were the bug. Re-run new probes 3× before believing them.
- **Do not re-claim:** "gravity-driven flow is curl-free"; "fixing direction fixes magnitude"; the moment tilt as the remedy (superseded — wrong object); the 5.17%-vs-20.76% headline without the null-test caveat.
