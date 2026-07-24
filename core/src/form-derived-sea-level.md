---
slug: form-derived-sea-level
type: formulation
status: sketch
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

1. **Waterline from inventory + hypsometry.** Let $V_{\mathrm{ocean}}$ be the hydrosphere's conserved ocean volume ( #obs-hydrosphere-box-nomos ) and let $h_{\mathrm{solid}}(x)$ be the solid surface elevation above a fixed bedrock datum. **Sea level** $s$ is the unique waterline such that the volume of the basins $\lbrace x : h_{\mathrm{solid}}(x) \lt s \rbrace$ equals $V_{\mathrm{ocean}}$ when capacity allows; if the basins cannot hold $V_{\mathrm{ocean}}$, $s$ sits **above** the highest ground and land fraction is zero (a total water-world). That submerged outcome is the Protogenic `water-covered-surface` promise kept, not a generator bug ( #disc-check-the-ladder ).
2. **Not weather bootstrap.** Deriving $s$ does **not** invert insolation, climate, or precipitation. Planet insolation, the hydrosphere box stock, and the climate→precip chain remain independent; $s$ only fixes honest land/sea geometry for later carving. Precip still comes from atmosphere stock; sea level is not a climate control knob.
3. **Solid surface split.** $$h_{\mathrm{solid}}(x) = h_{\mathrm{bathy}}(x) + f(x)$$ where $h_{\mathrm{bathy}}$ is the **bathymetry prior** (seafloor relief over seeded asymmetry) and $f$ is **freeboard** (crustal elevation relative to a reference isostatic level). The prior must **not** manufacture continents: bimodal hypsometry as an *initial condition* impersonates Abyssal output and is forbidden as a land-maker. Isostasy / freeboard is the mechanism by which Abyssal *delivers* land.
4. **Freeboard may be negative.** Conservation intuition: relative to a mean isostatic level, some cells ride high and some sit low. A stand-in that is strictly positive everywhere cannot differentiate platforms and basins and cannot keep `emerged-land`. v0 may use a **zero-mean freeboard stand-in** (density-proxy noise) that preserves the flux promise without full lithosphere-column tectonics; the full density/thickness + keel column is the later residual, not the first keeper.
5. **Emerged land as flux.** $$\mathrm{emerged}(x) = \max\bigl(0,\, h_{\mathrm{solid}}(x) - s\bigr)$$ (or an equivalent signed freeboard field the consumer interprets as land). The ordinum's Abyssal gate-charge `emergent-land` — *"meaningful non-volcanic land above sea level — delivered by uplift / proto-tectonic processes, never an initial condition"* — is expressible as the flux quantity `emerged land` ( #form-ordinum-governs-flux-web ). A nomos (freeboard / isostatic keeper) must **promise** that quantity; erosion **consumes** it. Until a keeper is registered, the audit reports **Unmet** and default builder admission refuses fluvial work unless `--allow-unmet` ( #form-builder-admission · #form-flux-web ).
6. **Order of work (locked path).** (i) honest bathymetry prior (no manufactured freeboard); (ii) pour $V_{\mathrm{ocean}}$ → derived $s$ (water-world until lift); (iii) freeboard stand-in earns some emerged land; (iv) promise `emerged land` only when freeboard is real in the nomotheke; (v) fluvial erosion without waiver; (vi) later full lower-density-crust / lithosphere freeboard. Decreed `SEA_LEVEL_M` retires when (ii) is the live datum.
7. **Record targets, not priors.** Mid-Abyssal land fraction and relief bands from early-continents literature are **Record-style checks** on processes that earn land — not licenses to bake modern (or early-Earth) land fraction into Phase-0 topography.

## Epistemic Status

**Max attainable:** **exact** for FE(1)–(3), FE(5) architecture, FE(6) order, and FE(7) reading discipline under `DECISIONS[water-world-is-the-promise-not-the-bug]` (`:by joseph`, decided) and `DECISIONS[ordinum-governs-the-flux-web]` (`:by joseph`, decided); **robust-qualitative** for freeboard physics until a density-column isostasy nomos exists; **heuristic** for any specific zero-mean stand-in amplitude.

**Currently `sketch`** because the claim path is decided and partially instrumented, but the nomotheke / gen path is not yet fully landed as law:

- Live gen still carries decreed `SEA_LEVEL_M = 4000` (ASSUMPTIONS: manufactures forbidden land).
- `flux::EMERGED_LAND` is consumed by erosion; **no registered producer** yet (audit Unmet is the honest state).
- Convicting measurement of the pour on the old unimodal prior: `examples/sea_level_probe.rs` (ocean ~1.37×10⁹ km³ vs capacity to peak ~1.36×10⁹ km³ → derived sea ~23 m above highest ground, land 0.0%).
- WIP pour / tectonic-surface code may exist beside this segment; **do not read unmerged code as this FE's proof**. When pour + freeboard keeper + nomotheke promise are green under tests, raise status for the exact FE items and retire residual trail banners on dependent segments.

Stage `draft`. Authority: FE water-world instance and ladder order are Joseph-decided; freeboard stand-in shape and full lithosphere residual are engineering under those decisions, not a second Joseph stamp.

## Discussion

Two independently declared quantities — ante-mundane water inventory and surface relief — must meet at one waterline. Decreed sea level hid that meeting and manufactured continents the ordinum forbids as initial conditions. Pouring the stock makes the Protogenic water-world visible; freeboard is how Abyssal earns the land erosion is allowed to carve. Climate and insolation stay on their own chains so "wet world" does not get confused with "weather bootstraps the coastline."

## Working Notes

- **Residual until code merge:** retarget trail banners on #disc-check-the-ladder , #form-ordinum-governs-flux-web , #obs-hydrosphere-box-nomos , #form-flux-web to this slug; when nomotheke promises `emerged land` and gen retires `SEA_LEVEL_M`, update FE(5)–(6) Known-incomplete on those segments and drop residual banners.
- **Probe debt:** true spherical-area pour (probe self-test vs $4\pi$) is the gold standard; any coarse uniform-area pour is engineering approximation and must not silently disagree with the probe on water-world / overflow cases.
- **Full isostasy residual:** lithosphere column (crust thickness × density + depleted keel) → compensation constant by mass balance → freeboard that can go negative by conservation. Literature trail: early-continents survey; Chowdhury et al. freeboard equation; Korenaga 2017 unread. Do not smuggle plate tectonics as required for first land (survey / Chowdhury: magmatic + isostatic emergence precedes orogeny).
- **Ordinum content:** Primordial `promise[sea-level-datum]` is the developmental home of derived $s$; Abyssal `promise[emerged-land]` is the home of freeboard-earned land. Full phase-content segmentation still open (OUTLINE §IV / §VII).
- History / dual homes: DECISIONS water-world and ordinum-governs entries remain ratification history; ASSUMPTIONS `SEA_LEVEL_M` row is the magic-constant ledger until retirement.
