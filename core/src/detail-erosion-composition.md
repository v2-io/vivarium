---
slug: detail-erosion-composition
type: detail
status: robust-qualitative
stage: draft
depends:
  - form-flux-web
  - form-derived-sea-level
  - form-isostasy-column
  - form-material-property-interface
  - obs-cube-locked-kernel-bias
  - obs-routing-curl-spiral
  - obs-mean-pin-manufactures-seam
  - disc-algorithms-disguise-physics
  - norm-bias-vs-noise
  - detail-phenomena-systems-map
---

# Live erosion composition — the crude fluvial rung

What the shipped `erosion-tile` nomos **is** today: one composed crude rung (uplift against a land surface, Priority-Flood fill, D8 tree, MFD drainage, implicit stream-power, deposition, talus, creep) with named measured residuals — not a full geomorphology stack and not a second law channel for any single defect.

## Formal Expression

1. **Article.** Live nomos name `erosion-tile` ( #form-flux-web / nomotheke). It **consumes** solid surface / emerged-land geometry, rock-uplift rate, and precipitation; **promises** an eroded bed later phases and views may stand on. Uniform hardness in this increment; per-material erodibility is the next hook on #form-material-property-interface , not claimed built.

2. **Per-epoch composition (order is load-bearing).** For each fluvial epoch on a face tile (`Fluvial::erode`):
   1. **Apply rock uplift** on non-outlet cells from the uplift field erosion consumes (not an internal conjured driver).
   2. **Priority-Flood** depression fill with an $\varepsilon$-gradient (Barnes–Lehman–Mulla lineage; deterministic tie-breaks).
   3. **D8** steepest-descent receivers — the tree the implicit solve needs.
   4. **MFD** drainage-area accumulation (Quinn lineage; live outflow exponent $p=1.0$) — decides *where* channels form.
   5. **Implicit stream-power incision** $E = K A^{m} S^{n}$ with **$n=1$** (Braun–Willett exact implicit solve; $m$ default $0.5$).
   6. **Davy–Lague deposition** $D = G\cdot Q_s/A$ routed down the D8 tree.
   7. **Talus** relaxation (repose slope).
   8. **Hillslope creep** — one explicit 5-point Laplacian step per epoch (diffusivity $\kappa$ in m²/epoch; diffusion-number clamp $k\le 0.24$ at fine levels).

3. **Geometry inputs (present tense).** Drainage **seed area** and deposit volume use **true spherical cell area** per cell (`measure::cell_area_m2`). MFD neighbour **lengths** still use uniform `cell_m` and diagonal $\mathrm{cell}_m\sqrt{2}$. Outlets use derived / declared sea as coast, not only tile edge.

4. **What this rung is for.** Crude-rung dendritic dissection of a land surface under precipitation and differential uplift — Cordonnier/Braun-family CG terrain method family ( #detail-phenomena-systems-map ). Enough for early-Abyssal *look* and for probes that convict scheme defects. **Not:** grain-size split, bank mechanics, meandering, debris flows, rock-mass return to a lithosphere column, or a correct router on the equiangular cube-sphere.

5. **Measured / declared residuals (homes elsewhere; do not re-derive).**
   | Residual | Class | Home |
   |---|---|---|
   | MFD as sheared 8-node fan; diagonals as phantom faces | directional bias | #obs-cube-locked-kernel-bias , #disc-algorithms-disguise-physics |
   | Routing curl / spiral flux | contour-orthogonality sacrifice | #obs-routing-curl-spiral |
   | Mean-pin multiscale "consistency" | manufactures seam + mass | #obs-mean-pin-manufactures-seam |
   | $p=1.1$ first-moment lattice bias | **retired live** ($p=1.0$) | ASSUMPTIONS + nomotheke note |
   | Uniform $A$ for drainage | **retired** for seed/deposit $A$ | #obs-cube-locked-kernel-bias FE(3) present tense |
   | Priority-Flood $\varepsilon$-fill | sign-definite mass mint in sinks | nomotheke unphysical term |
   | Creep $k$ clamp | fidelity compromise, not $z=2$ substeps | nomotheke; #sketch-dynamic-exponent-seams |
   | Rock-mass export without column return | missing conservation | #form-isostasy-column ; ASSUMPTIONS |
   | Strictly-positive uplift rate alone | cannot keep basins/freeboard predicate | #form-isostasy-column |

6. **Open successor (not adopted).** Kill diagonals → real edge fluxes → Coatléven-style vector reconstruction for magnitude+direction from one object; separate non-orthogonality fix for face gradients ( #obs-routing-curl-spiral remedy stack). Until then, cube-face attractors can still manufacture geography — cube control remains mandatory for any "emergent plates" claim.

7. **Out of bounds for this segment.** Replacing the composition with a new scheme; promoting inventory fluvial next-rungs ( #detail-phenomena-systems-map ) as live; claiming `erosion-substrate` **Kept** without a predicate probe that the eroded bed is the one later phases stand on.

## Epistemic Status

**Max attainable: exact** for "the code runs this composition in this order" (source + tests); **empirical** for channel-forming behaviour under defaults; **robust-qualitative** for residual table as restatement of measured homes.

**Currently `robust-qualitative`:** composition and order verified against `erosion.rs` header and `Fluvial::erode` path; residual rows cite measurement homes; successor open. Stage `draft`. `detail` — operational map of one crude rung, not a new physics postulate.

## Discussion

Without a single composition home, agents re-list steps from memory and confuse "crude live" with "scheme correct." This segment pins the **recipe** so defect observations and successors have something coherent to improve.

## Working Notes

- Live: `crates/vivarium-world/src/erosion.rs`, nomotheke `EROSION`, query spine composition.
- **Do not re-open** mean-pin as Haar substitute; $p=1.1$ as "grid bias cancel"; uniform $A$ as current drainage seed (retired for $A$).
- Next peels: per-material $K$ / threshold; rock-mass ledger into #form-isostasy-column ; router successor spike integration under probe discipline.
