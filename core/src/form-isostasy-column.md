---
slug: form-isostasy-column
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-column-control-volume
  - form-derived-sea-level
  - form-ordinum-governs-flux-web
  - obs-hydrosphere-box-nomos
  - disc-check-the-ladder
---

# Isostasy reads a lithosphere column; uplift rate is not a driver

Emerged land is earned by mass-conserving lithospheric freeboard — crust and depleted mantle keel as a conserved column, elevation as a derived reading — not by a strictly-positive height-rate field that only ever goes up.

## Formal Expression

1. **Primitive is column mass, not height rate.** The physical object is a **lithospheric column** per cell (or control volume): crustal thickness and density, plus keel (continental lithospheric mantle) thickness and depletion. **Elevation is derived** by isostatic balance over that column. Rock "uplift rate" is $\mathrm{d}e/\mathrm{d}t$ of that reading — a **diagnostic**, not a conserved driver. This is the same discipline as #form-column-control-volume (conserved material, not free height) applied to the solid Earth.

2. **Nomos chain (accepted shape).**  
   $$\mathrm{mantle\text{-}thermal} \rightarrow \mathrm{lithosphere} \rightarrow \mathrm{isostasy} \rightarrow \mathrm{sea\text{-}level} \rightarrow \mathrm{erosion}$$
   - **mantle-thermal** — secular cooling and melt fraction (the control parameter for long-term freeboard evolution).  
   - **lithosphere** — conserved columnar inventory (crust + keel), analogous in pattern to the hydrosphere box on rock ( #obs-hydrosphere-box-nomos ).  
   - **isostasy** — consumes the column, produces **surface elevation**; the compensation constant is fixed by **global mass balance**, so rise here is subsidence there without a second ad-hoc field.  
   - **sea-level** — ocean stock + solid hypsometry ( #form-derived-sea-level ).  
   - **erosion** — consumes elevation and must return **sediment mass** to the column (rock-mass ledger; isostatic rebound bounds relief).  
   Under this shape, a nomos named `uplift` that **produces a height rate** is the wrong article of law.

3. **Keel is half the freeboard, not optional.** Isostatic balance is over the **whole lithosphere**, compensated at the base of the continental lithospheric mantle — not crust alone. Melt that builds thick felsic crust leaves a Mg-rich, Fe-poor **depleted residue** (the buoyant keel). One mass-conserving process yields **two buoyant products stacked in one column**. Thickness typically dominates density contrast for crustal freeboard; the keel's small density contrast acts over great depth.

4. **Subsidence is the same physics with the driver off.** When magmatism ends and the column cools, edifices collapse and freeboard falls — re-submergence is not a bolted-on negative term. A field that is **strictly positive everywhere** cannot express basins, bimodal hypsometry, or "sit low."

5. **Plates are not required for first land.** Early cratonic emergence can be driven by magmatic differentiation + isostasy + cooling; compressional tectonics and orogeny add later relief. The ordinum ladder (water-world → transient volcanic emergence → stable freeboard → later high topography) is the same sequence the freeboard literature narrates ( #disc-check-the-ladder ).

6. **Predicate shape for `emerged-land` (literature-sourced acceptance tests, not modern-Earth bake-in).** When the promise is Specified, candid falsifiers include order-of-magnitude bands such as: land fraction $\gtrsim 10\%$ by ~3.0 Ga-equivalent; craton elevations ~1–1.5 km with peak relief ~1 km under a hot geotherm; $\gtrsim 2$ km topography unlikely while the crust is weak; modern-like freeboard only later (~2.5–2.2 Ga-equivalent). These are **Record-style checks** on earned freeboard, never licenses to bake land fraction into the Protogenic prior ( #form-derived-sea-level FE(7)).

7. **Live residual (honest present).** Code still has a crude `uplift` nomos: differential **strictly-positive** rock-uplift rate for erosion to carve against, plus a **zero-mean freeboard** fBm stand-in that *can* go negative and feeds the pour ( #form-derived-sea-level ). That freeboard is **not** true column isostasy; the rate field **cannot** by itself keep a differentiation predicate. Compliance debt is expected until lithosphere + isostasy replace the stand-in ( #form-ordinum-governs-flux-web maturity: Claimed ≠ Kept).

8. **Still open modelling (not this formulation).** Differentiation *rate law* (what makes felsic melt, how fast, where); full freeboard equation coefficients from primary sources not yet re-derived in-repo; Korenaga et al. 2017 unread as of the council note. Do not implement plates as a state variable to "get land."

## Epistemic Status

**Max attainable:** **exact** for the architectural claim that elevation must be a mass-conserving reading of a lithosphere column if Abyssal freeboard is to be keepable; **empirical / literature** for the Chowdhury–Cawood–Mulder 2025 mechanism narrative and numeric predicate bands; **heuristic** for any particular rate law until modelled and probed.

**Currently `robust-qualitative`:** nomos shape and "uplift-as-rate-producer is the wrong article" are **council-accepted** (`DECISIONS[isostasy-is-the-uplift-nomos-and-the-keel-is-half-of-it]`, `:council 2026-07-24`); strictly-positive live rate field is **code-verified** (`DECISIONS[uplift-is-structurally-incapable-of-keeping-its-promise]`); freeboard may-be-negative is live on the stand-in path. Differentiation rate law and full column implementation are **open**. Not a Joseph `:by joseph` row for every clause — architecture accepted at council; literature citations ride the decision's primary-read chain.

Stage `draft`. **Max for implementation claims: not claimed** — this segment does not assert that lithosphere/isostasy nomoi are built.

## Discussion

Sea level already pours a conserved ocean against solid hypsometry. Without a mass-conserving solid column on the other side of that pour, "emerged land" is either baked into a prior (forbidden by the ladder) or faked by a one-sided rate field. The isostasy column is the missing dual of the hydrosphere box: rock inventory with a readable freeboard, so erosion and sea-level both consume truth rather than costume.

## Working Notes

- **Live modules:** `uplift.rs` (`uplift_rate_m_per_epoch`, `freeboard_m`); pour/sea-level in `sea_level.rs`; erosion consumes `ROCK_UPLIFT_RATE`.
- **Do not re-open:** baking bimodal land into the bathymetry prior; plate state-fields as the path to first freeboard; citing iced DESIGN-MATERIAL as law (column law is #form-column-control-volume ).
- **Next convicting work:** lithosphere column schema (even crude thickness×density with keel stub); isostasy producer of elevation; range/reachability audit for BrokenKeeper (promise predicate vs keeper range); rock-mass return from erosion.
- **Primary literature home on the decision:** Chowdhury, Cawood & Mulder 2025 (relata: chowdhury-2025-subaerial); early-continents survey in `ref/research/` for Record targets.
- Sibling: live erosion recipe #detail-erosion-composition ; Abyssal phase home #detail-phase-abyssal ; open mantle-thermal nomos; router successor under #obs-cube-locked-kernel-bias .
