---
slug: obs-cube-locked-kernel-bias
type: observation
status: exact
stage: draft
depends:
  - form-grid-equiangular-staggered
  - disc-algorithms-disguise-physics
  - norm-bias-vs-noise
  - form-sphere-continuous-surface-fields
---

# Cube-locked kernel biases (MFD fan and uniform cell area)

Two independent, measured defects in erosion/drainage kernels **manufacture structure that follows cube faces**. They are biases that do not converge away under refinement. Sphere-continuous *priors* do not cancel them: they are scheme defects on the lattice ( #form-grid-equiangular-staggered FE(5) control).

## Formal Expression

1. **MFD fan is a directional bias.** Multiple-flow-direction routing that splits outflow over eight Moore neighbours with slope-weighted (or power-weighted) fans treats those eight directions as **evenly spaced quadrature nodes**. On the equiangular cube-sphere they are not. Away from a face centre the eight attractors **collapse into two** ($\Delta_{\mathrm{rms}}$ $16^\circ$ at the corner; $\sim 97\%$ of the compass pushed ${>}1^\circ$). On a perfect cone (exact flow = meridians), a plume ends **$474\,\mathrm{km}$ — twelve cell-widths — off its true meridian.** Refining **worsens** drift: $4.00^\circ$ (L6) $\to$ $5.75^\circ$ (L9), where pure noise would fall. Cube-face $u/v$ axes are **attractors**; the $45^\circ$ diagonal is a **repeller**. MFD thus reintroduces the grid-aligned-channel artifact it was adopted to remove. (`DECISIONS[mfd-fan-is-a-bias-and-does-not-converge]`, measurement-grade; `:by claude`.)
2. **Diagonals were never physics.** Four of eight “neighbours” share only a vertex — zero-length edge for flux. Recovering the physical claim points at **gradient-reconstructed edge flux** (real edges only), not a better eight-way fan. Router replacement is **open** ( #form-grid-equiangular-staggered FE(4)); this observation does not adopt a successor.
3. **Uniform cell area is a second cube-locked bias.** Drainage accumulation uses `cell_area = cell_m²` — **one** area per level for the whole tile (`sample::cell_size_m`). True spherical cell area varies. Measured: correct at face centre; **$+41.2\%$** overstated $A$ at edge midpoints; **area-weighted mean $+17.810\%$** — **bit-identical at L5, L7, L9, L11, L13**. No $N$ in the error. Stream power $E = K A^m S^n$ therefore sees a smooth **cube-locked fake erodibility** (up to $\sim 18.8\%$ in $A^m$ at $m=0.5$). (`DECISIONS[drainage-area-uses-a-uniform-cell-area]`, proposed measurement; PROBE 8 in wavelet-store spike.)
4. **Jacobian siblings.** Theory and measurement agree: the fan is the equiangular map’s **shear** showing through a flat-grid kernel; uniform area is the **determinant**. Closed-form spherical cell area exists (`msc/spike-wavelet-store/src/area.rs`); retiring uniform $A$ is small, exact, and independent of the open router.
5. **Mandatory cube control.** If emergent plate boundaries, major drainage, or orography **correlate with cube-face edges/corners**, treat the result as **void** until these biases (and related routing curl defects) are ruled out. A clean dozen-plate planet on cube edges would be a seductive false triumph ( #form-grid-equiangular-staggered FE(5)).
6. **Not superseded by sphere-continuous priors.** Continuous bathymetry/freeboard ( #form-sphere-continuous-surface-fields ) removes chart cliffs in the *generator*. Kernel bias carves **after** the prior.

## Epistemic Status

**Max attainable: exact** for the quoted measurements under the harnesses that produced them (`grid_lab` / fan probes; PROBE 8; DECISIONS tables).

**Currently `exact` as observation** of live defects. **Remedy stack** (edge-only FV routing, true $A$, moment conditions) is **proposed / unratified** — not project law. Cone-only “4× better router” headlines remain **gated** by a rotation test (Prescott) not yet claimed here.

Stage `draft`.

## Discussion

Bias vs noise ( #norm-bias-vs-noise ): both defects are signed, systematic, and level-fixed — the kind that manufactures fake law. Port the physical claim ( #disc-algorithms-disguise-physics ), not the Moore fan costume.

## Working Notes

- **Summarizes** fan + uniform-$A$ half of cube-locked defects. Sibling spiral measurement home: #obs-routing-curl-spiral . Prime-Question teaching chain: #worked-example-mfd-prime-question .
- **Instruments:** DECISIONS mfd-fan / drainage-area entries; `examples/grid_lab/`; `msc/spike-wavelet-store/` PROBE 8; `crates/vivarium-world/src/measure.rs` + fluvial `cell_area` (2026-07-23 PoC: **true A** for drainage seed and deposit volume; MFD *lengths* still uniform `cell_m` — fan residual remains).
- **Do not claim:** “cube-locked bias fixed” after true-A only; MFD fan is independent. MFD “killed D8 anisotropy” is not current truth.
