# router_lab — MEASUREMENTS

*Data file. Reasoning lives in `FINDINGS.md`; code in `crates/vivarium-world/examples/router_lab/`.*

## Setup

- Equiangular cube-sphere, **n = 32** (6144 cells) for the sweeps; n = 64/128 for level checks.
  ⚠ **Mesh** probes (full adjacency built) ⇒ capped near n=128. The repo's L9–L23
  numbers come from *analytic* fan probes that never build a mesh. Level-independence
  here spans **16×** in cell area, not 16 384×.
- Cone, `h = −geodesic(p, pole)`, pole = unit([0.3,−0.7,0.64]) — generic, incommensurate
  with the cube, so no symmetry hands a router a free zero.
- `sca_err = |(A/W) / (R·tan(θ/2)) − 1|`, θ ∈ [0.35, 2.2] rad; W = contour width against
  the **exact** flow direction (same denominator for all routers).
- Determinism (3 identical sweeps): **PASS**

## §1 P1 — the perfect-lattice baseline vs the slope exponent `p`

MFD-8 on `fan_ideal` (a perfect flat lattice), compass swept at 0.1°.

| p | rms Δ° | max Δ° | captured | attractors (deg) |
|---|---|---|---|---|
| 1.00 | 0.0000 | 0.0000 | 0.0% | 0,1,1,2,2,2,3,3,4,4,5,5,6,7,7,7,8,9,9,10,11,11,12,12,12,13,14,15,16,16,17,17,18,19,19,20,21,21,22,22,23,24,24,25,25,26,26,27,28,28,29,30,30,31,32,33,33,34,35,35,36,37,38,39,39,40,41,41,42,42,43,43,44,45,45,46,46,47,47,48,48,48,49,50,50,51,52,52,53,53,53,54,54,55,55,56,56,57,57,58,58,58,59,59,59,60,60,60,61,61,62,62,63,63,63,64,64,68,68,69,74,77,78,82,83,85,86,87,89,90,92,94,95,96,98,100,101,102,103,104,105,107,108,110,110,112,113,114,115,116,117,117,118,119,119,121,121,122,123,123,124,125,126,126,127,128,131,136,140,145,147,149,152,154,156,158,161,163,165,167,170,172,174,176,179 |
| 1.05 | 0.1301 | 0.1792 | 0.0% | 45,90,135,180,225,270,315,0 |
| 1.10 | 0.2419 | 0.3333 | 0.0% | 0,45,90,135,180,225,270,315 |
| 1.20 | 0.4174 | 0.5758 | 0.0% | 45,90,135,180,225,270,315,0 |
| 1.50 | 0.6614 | 0.9160 | 0.0% | 0,45,90,135,180,225,270,315 |
| 2.00 | 0.5647 | 0.7873 | 0.0% | 0,45,90,135,180,225,270,315 |

## §2 The moment ladder — p = 1.1

| router | conserv | mean\|Δ_fan\|° | mean\|Δ_Q\|° | mean\|m1_true\| | mean m2 | sca_err |
|---|---|---|---|---|---|---|
| MFD-8 (status quo) | 1.0000 | 5.10 | 6.89 | 0.0705 | 0.3270 | 20.76% |
| MFD-8 + true distances | 1.0000 | 3.30 | 4.96 | 0.0455 | 0.3252 | 19.78% |
| edge MFD (no diagonals) | 1.0000 | 5.16 | 4.08 | 0.0696 | 0.3853 | 12.13% |
| gradient-projected edge flux | 1.0000 | 3.10 | 0.95 | 0.0415 | 0.3775 | 5.17% |
| moment-corrected MFD-8 | 1.0000 | 0.02 | 3.56 | 0.0003 | 0.3247 | 18.67% |
| moment + edge flux | 1.0000 | 0.49 | 3.06 | 0.0084 | 0.3772 | 8.25% |
| true-width quadrature + moment | 1.0000 | 0.02 | 3.97 | 0.0003 | 0.3307 | 18.06% |
| ORACLE  grad edge flux @ψ* | 1.0000 | 3.10 | 0.94 | 0.0415 | 0.3776 | 5.16% |
| ORACLE  moment MFD-8   @ψ* | 1.0000 | 0.00 | 3.56 | 0.0000 | 0.3247 | 18.66% |
| ORACLE  moment + edge  @ψ* | 1.0000 | 0.47 | 3.06 | 0.0081 | 0.3773 | 8.23% |
| ORACLE  quad + moment  @ψ* | 1.0000 | 0.00 | 3.97 | 0.0000 | 0.3306 | 18.06% |
| CONTROL grad edge flux @ψ*+20° | 1.0000 | 18.63 | 18.63 | 0.2511 | 0.3865 | 13.59% |

`Δ_fan` = deflection of `Σwₖêₖ` (Lagrangian). `Δ_Q` = deflection of `Q̂ = Q_K/‖Q_K‖`
(Eulerian — what the FACE FLUXES imply). ⚠ `ORACLE` rows are handed the analytic ψ:
**diagnostic only, never a router result.**


## §2 The moment ladder — p = 1

| router | conserv | mean\|Δ_fan\|° | mean\|Δ_Q\|° | mean\|m1_true\| | mean m2 | sca_err |
|---|---|---|---|---|---|---|
| MFD-8 (status quo) | 1.0000 | 5.08 | 6.82 | 0.0694 | 0.3373 | 20.03% |
| MFD-8 + true distances | 1.0000 | 3.40 | 5.00 | 0.0465 | 0.3357 | 19.05% |
| edge MFD (no diagonals) | 1.0000 | 5.16 | 3.99 | 0.0696 | 0.3927 | 12.43% |
| gradient-projected edge flux | 1.0000 | 3.10 | 0.95 | 0.0415 | 0.3775 | 5.17% |
| moment-corrected MFD-8 | 1.0000 | 0.02 | 3.40 | 0.0003 | 0.3351 | 17.98% |
| moment + edge flux | 1.0000 | 0.49 | 3.06 | 0.0084 | 0.3772 | 8.25% |
| true-width quadrature + moment | 1.0000 | 0.02 | 3.97 | 0.0003 | 0.3307 | 18.06% |
| ORACLE  grad edge flux @ψ* | 1.0000 | 3.10 | 0.94 | 0.0415 | 0.3776 | 5.16% |
| ORACLE  moment MFD-8   @ψ* | 1.0000 | 0.00 | 3.41 | 0.0000 | 0.3351 | 17.97% |
| ORACLE  moment + edge  @ψ* | 1.0000 | 0.47 | 3.06 | 0.0081 | 0.3773 | 8.23% |
| ORACLE  quad + moment  @ψ* | 1.0000 | 0.00 | 3.97 | 0.0000 | 0.3306 | 18.06% |
| CONTROL grad edge flux @ψ*+20° | 1.0000 | 18.63 | 18.63 | 0.2511 | 0.3865 | 13.59% |

`Δ_fan` = deflection of `Σwₖêₖ` (Lagrangian). `Δ_Q` = deflection of `Q̂ = Q_K/‖Q_K‖`
(Eulerian — what the FACE FLUXES imply). ⚠ `ORACLE` rows are handed the analytic ψ:
**diagnostic only, never a router result.**

## §3 Coatléven ‖Q_K‖ vs A/W  (p = 1.1)

| router | ‖Q‖÷(A/W) | sd | sca_err (A/W) | sca_err (‖Q‖) | phantom flux |
|---|---|---|---|---|---|
| MFD-8 (status quo) | 0.5201 | 0.0509 | 20.76% | 58.17% | 47.8% |
| MFD-8 + true distances | 0.5153 | 0.0453 | 19.78% | 58.35% | 47.8% |
| edge MFD (no diagonals) | 0.9852 | 0.0577 | 12.13% | 10.98% | 0.0% |
| gradient-projected edge flux | 0.9843 | 0.0431 | 5.17% | 5.15% | 0.0% |
| moment-corrected MFD-8 | 0.5135 | 0.0497 | 18.67% | 58.21% | 47.7% |
| moment + edge flux | 0.9894 | 0.0495 | 8.25% | 7.64% | 0.0% |
| true-width quadrature + moment | 0.5135 | 0.0526 | 18.06% | 57.89% | 47.5% |

## §4 P3 — the geometric identity on spherical cells

`|K|·Id = Σ_σ |σ|(x_σ − x_K) ⊗ n_{K,σ}` — Euclidean identity, spherical cells.

| n | cells | mean rel. resid | max rel. resid |
|---|---|---|---|
| 4 | 96 | 7.777e-3 | 1.118e-2 |
| 8 | 384 | 2.059e-3 | 3.784e-3 |
| 16 | 1536 | 5.224e-4 | 1.087e-3 |
| 32 | 6144 | 1.311e-4 | 2.906e-4 |
| 64 | 24576 | 3.280e-5 | 7.504e-5 |
| 128 | 98304 | 8.202e-6 | 1.906e-5 |

## §5a Rotation sensitivity — sca_err by flow azimuth rel. cube u-axis (p=1.1)

| router | 0-15 | 15-30 | 30-45 | 45-60 | 60-75 | 75-90 | pk-pk |
|---|---|---|---|---|---|---|---|
| MFD-8 (status quo) | 16.94 | 22.61 | 20.26 | 21.64 | 20.91 | 19.72 | **5.67%** |
| MFD-8 + true distances | 13.47 | 21.22 | 20.78 | 21.76 | 19.63 | 15.64 | **8.29%** |
| edge MFD (no diagonals) | 15.25 | 14.34 | 9.66 | 9.17 | 13.35 | 16.78 | **7.61%** |
| gradient-projected edge flux | 14.36 | 4.46 | 1.96 | 1.91 | 4.46 | 15.99 | **14.07%** |
| moment-corrected MFD-8 | 8.78 | 19.05 | 21.90 | 22.40 | 18.56 | 9.51 | **13.62%** |
| moment + edge flux | 15.42 | 7.55 | 5.56 | 5.84 | 8.09 | 16.27 | **10.71%** |
| true-width quadrature + moment | 8.70 | 18.48 | 21.08 | 21.52 | 18.09 | 9.36 | **12.82%** |

⚠ **CONFOUND — do not read §5a as a clean rotation test.** On a cone about a fixed pole,
a cell's flow azimuth relative to the cube axes is CORRELATED WITH ITS POSITION on the
face — and the Jacobian shear is a function of position. So §5a mixes orientation with
location and cannot separate them. **§5b is the clean test** (same landform, same
distance, only the orientation changes), and there every router is rotation-stable to
≤1.42% pk-pk. §5a's structure is real and large, but it is evidence that the error is a
COHERENT FIELD over the face — not that the routers are orientation-dependent.

(§5b, the literal rotation test at 0°–45° over 7 angles, is in the run log.)

## §6 Level dependence (n = 32 → 128, 16× in cell area, p=1.1)

| router | err n=32 | err n=64 | err n=128 | \|Δfan\| 32 | 64 | 128 |
|---|---|---|---|---|---|---|
| MFD-8 (status quo) | 20.76% | 22.48% | 23.53% | 5.10 | 5.24 | 5.30 |
| MFD-8 + true distances | 19.78% | 21.34% | 22.25% | 3.30 | 3.39 | 3.43 |
| edge MFD (no diagonals) | 12.13% | 13.35% | 14.33% | 5.16 | 5.34 | 5.43 |
| gradient-projected edge flux | 5.17% | 4.68% | 4.73% | 3.10 | 3.00 | 2.96 |
| moment-corrected MFD-8 | 18.67% | 19.83% | 20.49% | 0.02 | 0.01 | 0.00 |
| moment + edge flux | 8.25% | 8.00% | 8.20% | 0.49 | 0.48 | 0.48 |
| true-width quadrature + moment | 18.06% | 19.20% | 19.80% | 0.02 | 0.01 | 0.00 |

## §7 Secondary — R² of sca_err on the moments (pooled, 7 honest routers, p=1.1)

| predictor | R² |
|---|---|
| m2_true | 0.039 |
| \|m1_true\| | 0.031 |
| both | 0.063 |
