---
slug: form-sphere-continuous-surface-fields
type: formulation
status: exact
stage: draft
depends:
  - post-determinism-as-ontology
  - form-derived-sea-level
  - norm-probe-sensitivity
---

# Sphere-continuous solid surface fields

Solid scalar fields on the planet — bathymetry, freeboard, tectonic surface, and any chart-independent prior — are continuous functions of position on the unit sphere. They are never independent noise charts per cube face. A globe that shows cube-face Y-structure in the *prior* is a generator bug, not geography.

## Formal Expression

1. **Chart independence.** Let $h$ be a solid surface field (bathymetry, freeboard, tectonic $h_{\mathrm{bathy}}+f$, or any other sphere-scalar used as solid height). For points $p,q$ on the unit sphere that are arc-close, $|h(p)-h(q)|$ must be the same *order* as within-face variation at the same arc — not $O(\text{amplitude})$ cliffs at cube edges or corners.
2. **Sampling law.** Evaluate $h$ by: `CellId` → cube $(u,v)$ → **unit vector** $p$ → sphere-domain fated noise (`noise::fbm3` / `hash3` on $(p_x,p_y,p_z)$). Per-face 2-D noise in face $(u,v)$ is **out of bounds** for solid priors: independent charts disagree on shared edges by construction.
3. **What this is not.** Continuity of the *prior* does not by itself make *evolved* or *assembled* surfaces continuous. Independent full-face evolution, dual-valued edge fluxes, hard edge outlets, and view-side charting can re-manufacture cube structure even when $h$ is sphere-honest. Those are seam / kernel / view debts ( #form-seam-flux-exchange , #form-grid-equiangular-staggered ), not licenses to abandon sphere sampling. **A measured instance ( #form-builder-admission , view-assembly): the store-assemble path blends memoized eroded tiles with the instant prior; when the tiles are carved under a *stale source hash* ( #form-complete-content-addressed-key ) or leave 1-cell coverage gaps, the covered↔fallback boundary is a datum step that paints cube-face-edge ribbons even though the prior itself is continuous.** The face-seam probe (FE(6)) measures cross-*face* prior continuity and is blind to this within-face coverage-boundary offset — the ribbon can appear while `cross ≲ within`.
4. **Freeboard is in scope.** Freeboard $f$ is a solid height contribution ( #form-derived-sea-level ). It obeys the same sampling law. Continuity on bathymetry alone with discontinuous freeboard still paints cube faces on the globe.
5. **Placeholder honesty.** Sphere-continuity is a *topology* law, not a geology claim. Freeboard is the isostasy read of a column ( #form-isostasy-column ) — continuity does not launder a fated craton field into a differentiation rate law, and does not by itself earn Record freeboard.
6. **Convicting probe.** A face-seam continuity probe is part of the generator contract: cross-edge and near-corner elevation deltas at one-cell arc must not exceed within-face deltas by more than a stated factor (live: `gen::tests::prior_is_continuous_across_faces_and_corners`). Probe discrimination is load-bearing ( #norm-probe-sensitivity ).

## Epistemic Status

**Max attainable: exact** for the sampling law as architecture (sphere is the continuous domain; cube faces are charts). Paid-for empirically 2026-07-10: v1 per-face prior produced multi-kilometre edge cliffs visible on the first whole-globe view; sphere-sampled `fbm3` retired that class of cliff in the prior.

**Currently `exact` for FE(1)–(2) and (4) as live generator law** under `bathymetry_m` / `lithosphere::freeboard_m` / tectonic surface + the continuity unit test. **FE(3) compliance debt is open and named:** full-face / tile evolution and some view assembly paths can still show cube-aligned structure (drainage sinks, dual-valued boundaries, cube-locked kernel bias — #form-grid-equiangular-staggered ). Debt does not soften the prior law.

**View-assembly staleness (2026-07-24, diagnosed + remedied view-side):** the whole-globe ribbons were convicted as an assembly artifact, *not* a prior/kernel defect. `query::World::load_eroded_regions` filtered store roots by the `erosion-tile@` prefix only and ignored the `src=` source hash, so tiles carved under a superseded source tree loaded and blended silently with current-source prior fallback — a two-datum surface whose coverage boundaries are the cube-edge ribbons (Joseph's edit→globe loop: any `vivarium-world` edit moves every key's source hash, orphaning yesterday's store). Remedy landed in the view seam: `eroded_region_census` (fresh/stale/total by source hash) + `load_current_eroded_regions` (current-source only); the globe defaults to current-only and prints the fresh/stale/prior-fallback census (silent fallback made loud), with `VIVARIUM_INCLUDE_STALE=1` restoring the old blend for diagnosis. Convicted by `query::tests::stale_source_tiles_are_loaded_silently_but_the_census_and_current_loader_separate_them` and `examples/coverage_seam_probe`. **Still-open, named:** a residual 1-cell coverage gap — `ErodedRegion::grid_pos` reports covered only on `[0, nx-2]` (bilinear needs `x0+1`), so the last row/column of every tile falls back to prior even under matching source; that is a builder/region tiling debt in `erosion.rs` (a separate fix wave — tile halo/overlap), named here, not fixed here.

Stage `draft`. No separate DECISIONS row; the law is the Jul-10 regime-probe correction carried in code comments and reflections, promoted here so it cannot be re-lost when kernel work thrash resumes.

## Discussion

The cube-sphere is a *covering* of $S^2$, not six independent planets. Chart seams are numerical objects; they must not appear as tectonic or hypsometric objects unless a process *on the sphere* puts them there. The first globe view made the opposite mistake obvious: independent face noise looks like a cube. The correct fix was domain change (sample on $S^2$), not edge blending or cosmetic clamps.

## Working Notes

- **Live:** `noise::fbm3`, `gen::bathymetry_m`, `lithosphere::freeboard_m` (sphere-continuous craton field), `sea_level::tectonic_surface_m`, continuity test in `gen.rs`.
- **Source / ice:** `.super-archive/from-msc/session-2026-07-10-mechanics.md` §1; `#norm-regime-probes`; globe spike comments on discontinuous prior.
- **Do not fold in:** MFD fan / uniform cell-area / mean-pin seam ridge — those are scheme and multiscale debts with their own homes (grid segment, #form-rl-closure-algebra , #form-seam-flux-exchange ).
- **Visual regression residual (2026-07-23) — diagnosed 2026-07-24:** the globe's cube-face-edge ribbons + edge land speckles were the **view-assembly staleness** artifact above (silent stale-source tile load blended with current-source prior fallback), *not* a prior discontinuity (FE(1)–(2) hold; seam probe green: fresh store measured cross 179 m mean / within 192 m mean) and *not* cube-locked kernel bias ( #obs-cube-locked-kernel-bias FE(5) ruled out for this artifact: the offset is at coverage boundaries, not carved by the router). Remedied view-side (census + current-source-only load); the outlet-hack temptation was correctly avoided. Instruments: `query::tests::stale_source_tiles_are_loaded_silently_…`, `examples/coverage_seam_probe`, and the globe HUD census line.
