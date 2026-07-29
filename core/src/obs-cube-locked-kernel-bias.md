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

1. **MFD fan is a directional bias.** Multiple-flow-direction routing that splits outflow over eight Moore neighbours with slope-weighted (or power-weighted) fans treats those eight directions as **evenly spaced quadrature nodes**. On the equiangular cube-sphere they are not. Away from a face centre the eight attractors **collapse into two** ($\Delta_{\mathrm{rms}}$ $16^\circ$ at the corner; $\sim 97\%$ of the compass pushed $\gt 1^\circ$). On a perfect cone (exact flow = meridians), a plume ends **$474\,\mathrm{km}$ — twelve cell-widths — off its true meridian.** Refining **worsens** drift: $4.00^\circ$ (L6) $\to$ $5.75^\circ$ (L9), where pure noise would fall. Cube-face $u/v$ axes are **attractors**; the $45^\circ$ diagonal is a **repeller**. MFD thus reintroduces the grid-aligned-channel artifact it was adopted to remove. (`DECISIONS[mfd-fan-is-a-bias-and-does-not-converge]`, measurement-grade; `:by claude`.)
2. **Diagonals were never physics.** Four of eight “neighbours” share only a vertex — zero-length edge for flux. Recovering the physical claim points at **gradient-reconstructed edge flux** (real edges only), not a better eight-way fan. Router replacement is **open** ( #form-grid-equiangular-staggered FE(4)); this observation does not adopt a successor.
3. **Uniform cell metrics were cube-locked biases (retired; the *metric* half only).** Two historical defects, both the equiangular map showing through a flat-grid kernel. **Area:** drainage used `cell_area = cell_m²` — **one** area per level — correct at face centre, **$+41.2\%$** overstated $A$ at edge midpoints, **area-weighted mean $+17.810\%$**, **bit-identical at L5–L13** (no $N$). **Lengths:** D8/MFD/incision/talus used uniform neighbour distance `cell_m` (diagonal $\sqrt{2}$) — a corner cell's orthogonal neighbour is **$\sim 6.3\%$** and its diagonal **$\sim 33\%$** *shorter* than the uniform value, so stream power saw both a fake erodibility and a fake slope, signed toward the corners. **Live:** drainage seed and deposit volume use true spherical area per cell (`measure::cell_area_m2`); D8, MFD, incision and talus use true great-circle neighbour distances (`measure::gc_dist_m` over per-cell centres, `Fluvial::dist_m`). This retires the metric bias only — **not** the directional fan of FE(1), which is the map's *shear*, not its metric (true metrics are necessary and insufficient, #norm-bias-vs-noise ). (`DECISIONS[drainage-area-uses-a-uniform-cell-area]`; PROBE 8; measure tests `neighbor_length_vs_uniform_is_cube_locked` / `true_area_vs_uniform_matches_probe8_shape`.)
4. **Jacobian siblings.** The fan is the equiangular map’s **shear** showing through a flat-grid kernel; uniform area was the **determinant** and uniform length its **edge scale**. Closed-form area and true great-circle neighbour lengths are both live; the **shear** (fan) is untouched by them — router successor (edge fluxes; kill diagonals) remains open.
5. **Mandatory cube control.** If emergent plate boundaries, major drainage, or orography **correlate with cube-face edges/corners**, treat the result as **void** until these biases (and related routing curl defects) are ruled out. A clean dozen-plate planet on cube edges would be a seductive false triumph ( #form-grid-equiangular-staggered FE(5)).
6. **Not superseded by sphere-continuous priors.** Continuous bathymetry/freeboard ( #form-sphere-continuous-surface-fields ) removes chart cliffs in the *generator*. Kernel bias carves **after** the prior.

## Epistemic Status

**Max attainable: exact** for the quoted measurements under the harnesses that produced them (`grid_lab` / fan probes; PROBE 8; DECISIONS tables).

**Currently `exact` as observation** of the live directional-fan defect (FE(1)). The **metric** fixes (true $A$, true neighbour lengths) are **live** in the fluvial kernel — no longer part of the open stack. The remaining **remedy stack** (edge-only FV routing; kill diagonals; moment conditions) is **proposed / unratified** — not project law. The rotation gate on cone-only router headlines is **cleared**: the rotation test ran and passed — every router rotation-stable to ≤1.42% pk-pk, with the §5a orientation/position confound caught and the clean §5b form used (`DECISIONS[the-router-is-a-scalar-pretending-to-be-a-vector-and-p-is-the-bias]` ⑥, council-accepted). What still gates those headlines is the null-test caveat ( #obs-routing-curl-spiral FE(3)), not rotation.

Stage `draft`.

## Discussion

Bias vs noise ( #norm-bias-vs-noise ): both defects are signed, systematic, and level-fixed — the kind that manufactures fake law. Port the physical claim ( #disc-algorithms-disguise-physics ), not the Moore fan costume.

## Working Notes

- **Summarizes** fan + uniform-$A$ half of cube-locked defects. Sibling spiral measurement home: #obs-routing-curl-spiral . Prime-Question teaching chain: #worked-example-mfd-prime-question .
- **Instruments:** DECISIONS mfd-fan / drainage-area entries; `examples/grid_lab/`; `msc/spike-wavelet-store/` PROBE 8; `crates/vivarium-world/src/measure.rs` (`cell_area_m2`, `cell_center_unit`, `gc_dist_m`, `neighbor_center_dist_m`) + fluvial `cell_area` / `centers` / `dist_m`: **true $A$** for drainage seed and deposit volume, and **true great-circle neighbour lengths** for D8 / MFD / incision / talus. The fan-drift and curl numbers (FE(1), #obs-routing-curl-spiral ) were measured under the old uniform-length probe harness (`grid_lab` / `curl_probe`) and are **not** re-run by this live-kernel change.
- **Do not claim:** “cube-locked bias fixed” after true metrics — the MFD directional fan (FE(1)) is independent and untouched by honest area/length. MFD “killed D8 anisotropy” is not current truth.
