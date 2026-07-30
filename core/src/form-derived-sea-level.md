---
slug: form-derived-sea-level
type: formulation
status: robust-qualitative
stage: draft
depends:
  - disc-check-the-ladder
  - obs-hydrosphere-box-nomos
  - form-flux-web
  - form-ordinum-governs-flux-web
  - form-isostasy-column
---

# Derived sea level and Abyssal freeboard

Sea level is the waterline that holds the conserved ocean stock against solid hypsometry; emerged land is freeboard above that waterline — never a decreed continent baked into the prior.

## Formal Expression

1. **Waterline from inventory + hypsometry.** Let $V_{\mathrm{ocean}}$ be the hydrosphere's conserved ocean volume ( #obs-hydrosphere-box-nomos ) and let $h_{\mathrm{solid}}(x)$ be the solid surface elevation above a fixed bedrock datum. **Sea level** $s$ is the unique waterline such that the volume of the basins $\lbrace x : h_{\mathrm{solid}}(x) \lt s \rbrace$ equals $V_{\mathrm{ocean}}$ when capacity allows; if the basins cannot hold $V_{\mathrm{ocean}}$, $s$ sits **above** the highest ground and land fraction is zero (a total water-world). That submerged outcome is the Protogenic `water-covered-surface` promise kept, not a generator bug ( #disc-check-the-ladder ). Live: `sea_level::derived_sea_level_m(seed)`.
2. **Not weather bootstrap.** Deriving $s$ does **not** invert insolation, climate, or precipitation. Planet insolation, the hydrosphere box stock, and the climate→precip chain remain independent; $s$ only fixes honest land/sea geometry for later carving.
3. **Solid surface split.** $$h_{\mathrm{solid}}(x) = h_{\mathrm{bathy}}(x) + f(x)$$ where $h_{\mathrm{bathy}}$ is the **bathymetry prior** (`gen::bathymetry_m`) and $f$ is **freeboard** — since 2026-07-24 the mass-conserving isostasy read of the lithosphere column (`lithosphere::freeboard_m`, #form-isostasy-column ), replacing the retired fBm stand-in. Live tectonic surface: `sea_level::tectonic_surface_m` (also exposed as `gen::initial_topography_m` for consumers).
4. **Freeboard may be negative.** Live freeboard is the **isostasy read** of the lithosphere column ( #form-isostasy-column ): zero-mean by mass-balance construction on the reference grid; range spans basins and land (reachability tested). The retired fBm freeboard stand-in is not on the pour path.
5. **Emerged land as flux.** The Abyssal gate is the flux quantity `emerged land`. The **isostasy** nomos **promises** it (`:kept-by isostasy` in the ordinum — **Claimed**, not Kept); erosion **consumes** it. Live: nomotheke closed; default builder runs erosion/water **without** `--allow-unmet`. Land fraction after pour stays in the few-percent Abyssal band under present defaults (existing pour test), not modern ~29%.
6. **Order of work (live).** (i) bathymetry prior; (ii) lithosphere column → isostasy freeboard; (iii) pour $V_{\mathrm{ocean}}$ on $h_{\mathrm{bathy}}+f$ → $s$; (iv) fluvial erosion without waiver on emerged land. Decreed `SEA_LEVEL_M` is retired as land gate (compat residual only). Open residual on the freeboard side: rate law, mantle-thermal nomos, water loading, rock-mass return ( #form-isostasy-column FE(8)).
7. **Record targets, not priors.** Mid-Abyssal land fraction bands from early-continents literature are Record-style checks for **Kept** — not licenses to bake land fraction into Phase-0 topography, and not satisfied merely by Claimed.

## Epistemic Status

**Max attainable:** **exact** for FE(1)–(3), FE(5) flux architecture under `DECISIONS[water-world-is-the-promise-not-the-bug]` and `DECISIONS[ordinum-governs-the-flux-web]` (`:by joseph`); **robust-qualitative** for freeboard-as-isostasy-read (v1 balance real; inventory stand-in); **heuristic** for absolute freeboard amplitudes pending rate-law work.

**Currently `robust-qualitative`:** pour, bathymetry split, **column freeboard**, Claimed isostasy keeper, and closed flux web are **live** under lib tests. Residual: pour area sampling; `SEA_LEVEL_M` numeric compat; open FE(8) on #form-isostasy-column ; Claimed ≠ Kept for `emerged-land`.

Stage `draft`.

## Discussion

Two independently declared quantities — ante-mundane water inventory and mass-conserving solid freeboard — meet at one waterline. Climate and insolation stay on their own chains so "wet world" is not confused with "weather bootstraps the coastline."

## Working Notes

- Live: `sea_level.rs`, `gen::bathymetry_m`, `lithosphere::freeboard_m`, nomotheke `ISOSTASY` promises `EMERGED_LAND`.
- Freeboard law home: #form-isostasy-column . Abyssal phase maturity: #detail-phase-abyssal .
- Probe: `examples/sea_level_probe.rs` retarget when convenient; **Record land-fraction predicate now built** — `sea_level::emerged_land_verdict` + `examples/emerged_land_probe.rs` ( #form-isostasy-column FE(6)), the Claimed → Kept instrument (hard clauses convict live; still Claimed, not Kept).
- Ordinum Primordial `promise[sea-level-datum]` phase home still missing (OUTLINE).
- **Sea is classified by connectivity, not by elevation threshold** — the datum this segment derives says which cells are *submerged*, and #form-ocean-is-connectivity-not-elevation says which of those are *ocean*. A landlocked below-datum basin is standing water, and this world expresses the Caspian class: 1165 bodies, 976 865 km³ at whole-face L8 ( #obs-connectivity-fills-the-basins-the-threshold-drained FE(4)). Joseph's eye caught the consequence of the threshold reading first (2026-07-28, first depression-paint session, F0 L14 region window) and named the cause exactly.
- **Still threshold-classified, and therefore still wrong in the same way:** the explorer's surface and water paints, the terminal globe reader, and the water kernel's own initial fill each derive "is this ocean" independently from the datum. Only the router's answer is connectivity-aware. The ocean set wants to be one shared world object rather than four thresholds — #form-ocean-is-connectivity-not-elevation Working Notes carries that as owed work, and crustal columns will want the same mask for plate work (Joseph, 2026-07-29).
