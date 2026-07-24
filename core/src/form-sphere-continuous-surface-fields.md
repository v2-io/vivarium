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
3. **What this is not.** Continuity of the *prior* does not by itself make *evolved* or *assembled* surfaces continuous. Independent full-face evolution, dual-valued edge fluxes, hard edge outlets, and view-side charting can re-manufacture cube structure even when $h$ is sphere-honest. Those are seam / kernel / view debts ( #form-seam-flux-exchange , #form-grid-equiangular-staggered ), not licenses to abandon sphere sampling.
4. **Freeboard is in scope.** Freeboard $f$ is a solid height contribution ( #form-derived-sea-level ). It obeys the same sampling law. Continuity on bathymetry alone with discontinuous freeboard still paints cube faces on the globe.
5. **Placeholder honesty.** Sphere-continuity is a *topology* law, not a geology claim. Long-wavelength fBm freeboard remains a stand-in (low Record / low Lawfulness until density-column isostasy). Continuity does not launder the prior into principled continents.
6. **Convicting probe.** A face-seam continuity probe is part of the generator contract: cross-edge and near-corner elevation deltas at one-cell arc must not exceed within-face deltas by more than a stated factor (live: `gen::tests::prior_is_continuous_across_faces_and_corners`). Probe discrimination is load-bearing ( #norm-probe-sensitivity ).

## Epistemic Status

**Max attainable: exact** for the sampling law as architecture (sphere is the continuous domain; cube faces are charts). Paid-for empirically 2026-07-10: v1 per-face prior produced multi-kilometre edge cliffs visible on the first whole-globe view; sphere-sampled `fbm3` retired that class of cliff in the prior.

**Currently `exact` for FE(1)–(2) and (4) as live generator law** under `bathymetry_m` / `uplift::freeboard_m` / tectonic surface + the continuity unit test. **FE(3) compliance debt is open and named:** full-face / tile evolution and some view assembly paths can still show cube-aligned structure (drainage sinks, dual-valued boundaries, cube-locked kernel bias — #form-grid-equiangular-staggered ). Debt does not soften the prior law.

Stage `draft`. No separate DECISIONS row; the law is the Jul-10 regime-probe correction carried in code comments and reflections, promoted here so it cannot be re-lost when kernel work thrash resumes.

## Discussion

The cube-sphere is a *covering* of $S^2$, not six independent planets. Chart seams are numerical objects; they must not appear as tectonic or hypsometric objects unless a process *on the sphere* puts them there. The first globe view made the opposite mistake obvious: independent face noise looks like a cube. The correct fix was domain change (sample on $S^2$), not edge blending or cosmetic clamps.

## Working Notes

- **Live:** `noise::fbm3`, `gen::bathymetry_m`, `uplift::freeboard_m`, `sea_level::tectonic_surface_m`, continuity test in `gen.rs`. Nomotheke uplift status string already says sphere-continuous bathymetry.
- **Source / ice:** `.super-archive/from-msc/session-2026-07-10-mechanics.md` §1; `#norm-regime-probes`; globe spike comments on discontinuous prior.
- **Do not fold in:** MFD fan / uniform cell-area / mean-pin seam ridge — those are scheme and multiscale debts with their own homes (grid segment, #form-rl-closure-algebra , #form-seam-flux-exchange ).
- **Visual regression residual (2026-07-23):** globe still shows face/Y structure under some seeds and assembly paths; diagnose against FE(3) debt and cube-locked kernels before re-inventing outlet hacks.
