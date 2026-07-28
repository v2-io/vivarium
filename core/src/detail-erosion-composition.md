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

2. **The settle history is a materialized stage chain.** A tile's epochs land as keyed interior stages every `erosion_stage_stride` epochs (manifest demand — #form-manifest-prescribes-vivium FE(5)), each seeded from its predecessor stage by complete key and recording its final-epoch mean $\lvert\Delta h\rvert$ as a stage-residual sibling ( #form-time-indexed-stage-chains FE(8): the chain is **materialized-only** — its density is what was built, and asking for more is a build request). Chained ≡ one-shot bit-exactly (`query::tests::staged_chain_is_bit_identical_to_one_shot`), so the stride never touches artifact identity, and a world built endpoint-only gains its interior for the cost of one settle history. The per-epoch step itself is untouched by staging — a rung is exactly `erode(k - k_{prev})` on the predecessor's heights.

3. **Per-epoch composition (order is load-bearing).** For each fluvial epoch on a face tile (`Fluvial::erode`):
   1. **Apply rock uplift** on non-outlet cells from the uplift field erosion consumes (not an internal conjured driver).
   2. **Priority-Flood** depression fill with an $\varepsilon$-gradient (Barnes–Lehman–Mulla lineage; deterministic tie-breaks). **The raise is kept** — steps 3–8 and the stored heights all see the filled surface. The three readers that call the same fill (`drainage_surface`, `chi_profile`, `response_census`) save and restore around it; the epoch loop does not, and that asymmetry is load-bearing ( #obs-fill-writes-itself-into-the-bed ).
   3. **D8** steepest-descent receivers — the tree the implicit solve needs.
   4. **MFD** drainage-area accumulation (Quinn lineage; live outflow exponent $p=1.0$) — decides *where* channels form.
   5. **Implicit stream-power incision** $E = K A^{m} S^{n}$ with **$n=1$** (Braun–Willett exact implicit solve; $m$ default $0.5$).
   6. **Davy–Lague deposition** $D = G\cdot Q_s/A$ routed down the D8 tree.
   7. **Talus** relaxation (repose slope).
   8. **Hillslope creep** — one explicit 5-point Laplacian step per epoch (diffusivity $\kappa$ in m²/epoch; diffusion-number clamp $k\le 0.24$ at fine levels).

4. **Geometry inputs (present tense).** Drainage **seed area** and deposit volume use **true spherical cell area** per cell (`measure::cell_area_m2`); D8 slope, MFD slope-weights, incision length and talus all use **true great-circle neighbour distances** between cell centres (`measure::gc_dist_m` / `Fluvial::dist_m`), not uniform `cell_m` / diagonal $\mathrm{cell}_m\sqrt{2}$. Creep still uses `cell_m²` as its diffusion length² (a face-mean stand-in, not the fan path). Honest metrics do **not** cure the directional fan ( #obs-cube-locked-kernel-bias FE(1)). Outlets are the coast (derived / declared sea) **plus every edge cell of any tile short of a whole face** — which is every builder and beacon tile, so base level is never more than half a tile away and the consequences are large and measured ( #obs-tile-outlets-grade-away-the-basins ). **The coast is the present anchor's coast, and the whole rung is the present anchor's terrain.** The seed surface is `gen::initial_topography_m`, which is `sea_level::tectonic_surface_m` at `MANTLE_TP_C`; `Fluvial::outlets` and the kernel's two further sea reads take `sea_level::derived_sea_level_m(seed)`, which has no cooling-stage argument. The surrounding column stack *is* parameterized in $T_p$ end to end ( #form-isostasy-column FE(7)), so this rung is the one place the world is pinned to one moment — the terrain further down the cooling chain is measured at #obs-the-erodible-world-is-down-the-cooling-chain and has never been carved, and the design that would carve it is #form-erosion-at-a-cooling-stage .

5. **What this rung is for.** Crude-rung dendritic dissection of a land surface under precipitation and differential uplift — Cordonnier/Braun-family CG terrain method family ( #detail-phenomena-systems-map ). Enough for early-Abyssal *look* and for probes that convict scheme defects. **Not:** grain-size split, bank mechanics, meandering, debris flows, rock-mass return to a lithosphere column, or a correct router on the equiangular cube-sphere.

6. **Measured / declared residuals (homes elsewhere; do not re-derive).**
   | Residual | Class | Home |
   |---|---|---|
   | MFD as sheared 8-node fan; diagonals as phantom faces | directional bias | #obs-cube-locked-kernel-bias , #disc-algorithms-disguise-physics |
   | Routing curl / spiral flux | contour-orthogonality sacrifice | #obs-routing-curl-spiral |
   | Mean-pin multiscale "consistency" | manufactures seam + mass | #obs-mean-pin-manufactures-seam |
   | $p=1.1$ first-moment lattice bias | **retired live** ($p=1.0$) | ASSUMPTIONS + nomotheke note |
   | Uniform $A$ for drainage | **retired** for seed/deposit $A$ | #obs-cube-locked-kernel-bias FE(3) present tense |
   | Uniform neighbour **length** (D8/MFD/incision/talus) | **retired** — true great-circle distances | #obs-cube-locked-kernel-bias FE(3) present tense |
   | Partial-tile **edge sinks** as base level | grades every tile to its own perimeter — tile-local basins capped at the tile (driver starved 3.85×), seam pits on assembly rising monotonically as the grain is made finer | #obs-tile-outlets-grade-away-the-basins ; #obs-fill-writes-itself-into-the-bed FE(4) |
   | Priority-Flood fill **written into the bed** | step (2) raises every closed depression to its spill point and the loop never restores, unlike the three readers that do — so the stored bed is depression-free by construction and **nothing can remain a lake**. First epoch mints $\approx 2\times10^{13}\,\mathrm{m^3}$ per L9 face | #obs-fill-writes-itself-into-the-bed FE(1)–(3) |
   | Priority-Flood $\varepsilon$-fill | sign-definite mass mint in sinks; **also directional** — orients flow across flats into long straight runs ( #obs-tile-outlets-grade-away-the-basins FE(8)) | nomotheke unphysical term |
   | Creep $k$ clamp | fidelity compromise, not $z=2$ substeps | nomotheke; #sketch-dynamic-exponent-seams |
   | Fine-grain tile export without column debit | **pour-grain ledger built + adopted as the live default surface** ( #form-isostasy-column FE(9): debit + Airy rebound + submarine-sediment credit, closed-box conserved, LITHO_COLUMN Conserved); the fluvial *tile* pipeline here still exports carved rock without debiting its column — bridging the two grains is a later rung | #form-isostasy-column FE(9); `erosion_return.rs` |
   | Uplift **rate** field strictly positive | diagnostic carving driver only — freeboard is isostasy's job | #form-isostasy-column ; live `uplift.rs` |

7. **Open successor (not adopted).** Kill diagonals → real edge fluxes → Coatléven-style vector reconstruction for magnitude+direction from one object; separate non-orthogonality fix for face gradients ( #obs-routing-curl-spiral remedy stack). Until then, cube-face attractors can still manufacture geography — cube control remains mandatory for any "emergent plates" claim.

8. **Out of bounds for this segment.** Replacing the composition with a new scheme; promoting inventory fluvial next-rungs ( #detail-phenomena-systems-map ) as live; claiming `erosion-substrate` **Kept** without a predicate probe that the eroded bed is the one later phases stand on.

## Epistemic Status

**Max attainable: exact** for "the code runs this composition in this order" (source + tests); **empirical** for channel-forming behaviour under defaults; **robust-qualitative** for residual table as restatement of measured homes.

**Currently `robust-qualitative`:** composition and order verified against `erosion.rs` header and `Fluvial::erode` path; residual rows cite measurement homes; successor open. Stage `draft`. `detail` — operational map of one crude rung, not a new physics postulate.

## Discussion

Without a single composition home, agents re-list steps from memory and confuse "crude live" with "scheme correct." This segment pins the **recipe** so defect observations and successors have something coherent to improve.

## Working Notes

- Live: `crates/vivarium-world/src/erosion.rs`, nomotheke `EROSION`, query spine composition.
- **Do not re-open** mean-pin as Haar substitute; $p=1.1$ as "grid bias cancel"; uniform $A$ as current drainage seed (retired for $A$).
- Next peels: per-material $K$ / threshold; router successor spike integration under probe discipline. (Rock-mass ledger landed at the pour grain — #form-isostasy-column FE(9) / `erosion_return.rs`; bridging it to this fine-grain *tile* pipeline, so carved tiles debit their columns, is a later rung.)
