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
---

# Derived sea level and Abyssal freeboard

Sea level is the waterline that holds the conserved ocean stock against solid hypsometry; emerged land is freeboard above that waterline — never a decreed continent baked into the prior.

## Formal Expression

1. **Waterline from inventory + hypsometry.** Let $V_{\mathrm{ocean}}$ be the hydrosphere's conserved ocean volume ( #obs-hydrosphere-box-nomos ) and let $h_{\mathrm{solid}}(x)$ be the solid surface elevation above a fixed bedrock datum. **Sea level** $s$ is the unique waterline such that the volume of the basins $\lbrace x : h_{\mathrm{solid}}(x) \lt s \rbrace$ equals $V_{\mathrm{ocean}}$ when capacity allows; if the basins cannot hold $V_{\mathrm{ocean}}$, $s$ sits **above** the highest ground and land fraction is zero (a total water-world). That submerged outcome is the Protogenic `water-covered-surface` promise kept, not a generator bug ( #disc-check-the-ladder ). Live: `sea_level::derived_sea_level_m(seed)`.
2. **Not weather bootstrap.** Deriving $s$ does **not** invert insolation, climate, or precipitation. Planet insolation, the hydrosphere box stock, and the climate→precip chain remain independent; $s$ only fixes honest land/sea geometry for later carving.
3. **Solid surface split.** $$h_{\mathrm{solid}}(x) = h_{\mathrm{bathy}}(x) + f(x)$$ where $h_{\mathrm{bathy}}$ is the **bathymetry prior** (`gen::bathymetry_m`) and $f$ is **freeboard** (`uplift::freeboard_m`). Live tectonic surface: `sea_level::tectonic_surface_m` (also exposed as `gen::initial_topography_m` for consumers).
4. **Freeboard may be negative.** Zero-mean isostatic stand-in (v0): long-wavelength fBm, can go negative. Full lithosphere thickness×density isostasy is the later residual — not required for first land (early-continents survey: magmatic/isostatic emergence precedes orogeny).
5. **Emerged land as flux.** The Abyssal gate is the flux quantity `emerged land`. The **uplift-tile** nomos **promises** it; erosion **consumes** it. Live: nomotheke closed; default builder runs erosion/water **without** `--allow-unmet`. Land fraction after pour is small (few-percent Abyssal band), not modern ~29%.
6. **Order of work.** (i) bathymetry prior; (ii) freeboard stand-in; (iii) pour $V_{\mathrm{ocean}}$ on $h_{\mathrm{bathy}}+f$ → $s$; (iv) promise `emerged land`; (v) fluvial erosion without waiver; (vi) later density-column freeboard. Decreed `SEA_LEVEL_M` is retired as land gate (compat residual only).
7. **Record targets, not priors.** Mid-Abyssal land fraction bands from early-continents literature are Record-style checks — not licenses to bake land fraction into Phase-0 topography.

## Epistemic Status

**Max attainable:** **exact** for FE(1)–(3), FE(5) flux architecture, FE(6) order under `DECISIONS[water-world-is-the-promise-not-the-bug]` and `DECISIONS[ordinum-governs-the-flux-web]` (`:by joseph`); **robust-qualitative** for freeboard physics until density-column isostasy; **heuristic** for freeboard amplitude (ASSUMPTIONS).

**Currently `robust-qualitative`:** pour, bathymetry split, freeboard keeper, and closed flux web are **live** under lib tests (95) and smoke build without waiver. Residual: freeboard is not true isostasy; pour uses uniform-area sample (probe retains true spherical excess as gold standard); `SEA_LEVEL_M` still exists as a numeric residual for migrating call sites.

Stage `draft`.

## Discussion

Two independently declared quantities — ante-mundane water inventory and surface relief — meet at one waterline. Freeboard is how Abyssal earns the land erosion may carve. Climate and insolation stay on their own chains so "wet world" is not confused with "weather bootstraps the coastline."

## Working Notes

- Live modules: `sea_level.rs`, `gen::bathymetry_m`, `uplift::freeboard_m`, nomotheke uplift promises `EMERGED_LAND`.
- **Full isostasy residual:** lithosphere column → compensation by mass balance. Literature: early-continents survey; Flament 2008; Chowdhury 2025 (relata keys present).
- Probe: `examples/sea_level_probe.rs` should be retargeted to bathymetry vs tectonic surface when convenient.
- Ordinum Primordial `promise[sea-level-datum]` and Abyssal `promise[emerged-land]` remain phase-content segmentation debt (OUTLINE §VII).
