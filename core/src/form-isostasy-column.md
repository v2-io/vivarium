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

7. **Live state (first rung built, 2026-07-24).** `lithosphere.rs` carries the chain's first convicting rung: a per-cell **column** (craton felsic crust + depleted keel over thermally-ramped oceanic crust; the mantle driver `MANTLE_TP_C` a declared dialable constant) and the **Airy isostasy read** — freeboard = buoyancy height − global mass-balance reference, zero-mean by construction on the reference grid (rise here *is* subsidence there, unit-tested exact), spanning basins and land (the range/reachability conviction, unit-tested). The fBm freeboard stand-in is **retired**; the pour reads the column ( #form-derived-sea-level ), land fraction stays in the Abyssal band, and the ordinum's `promise[emerged-land]` is **Claimed** `:kept-by isostasy` — Claimed, not Kept ( #form-ordinum-governs-flux-web ). The `uplift` nomos survives as the rate-field *driver* erosion carves against — a diagnostic-grade article, no longer a freeboard keeper. Cooling-grows-contrast (the buoyancy half) is a unit-tested monotonicity.

8. **Still open modelling (not this formulation).** Differentiation *rate law* (v1 columns are a fated stand-in — declared, physics Low); a mantle-thermal **nomos** (cooling as a process; today a declared constant); water loading in the balance (the $\rho_{sw}$ terms — declared v1 omission); erosion's mass **return** to the column (rock-mass ledger; isostatic rebound); full freeboard coefficients from primary sources not re-derived in-repo; Korenaga et al. 2017 unread. Do not implement plates as a state variable to "get land."

## Epistemic Status

**Max attainable:** **exact** for the architectural claim that elevation must be a mass-conserving reading of a lithosphere column if Abyssal freeboard is to be keepable; **empirical / literature** for the Chowdhury–Cawood–Mulder 2025 mechanism narrative and numeric predicate bands; **heuristic** for any particular rate law until modelled and probed.

**Currently `robust-qualitative`:** nomos shape and "uplift-as-rate-producer is the wrong article for freeboard" are **council-accepted** (`DECISIONS[isostasy-is-the-uplift-nomos-and-the-keel-is-half-of-it]`, `:council 2026-07-24`). **v1 implementation** (column + Airy read + pour wiring + Claimed keeper) is **code- and test-verified** (`lithosphere.rs` convictions; ordinum Claimed test; land-fraction pour still in band). The build ledger entry for this rung is **proposed** (design accepted; build verdict fresh — not yet council-sealed as engineering). Differentiation rate law, mantle-thermal *nomos*, water loading, and rock-mass return remain **open** (FE(8)). Architecture accepted at council; literature citations ride the decision's primary-read chain.

Stage `draft`. **Implementation claim (bounded):** `lithosphere` and `isostasy` are live and registered with the v1 scope FE(7) states — inventory stand-in (physics Low), balance read (physics Med), freeboard may be negative by construction. **Not claimed:** Kept; anything in FE(8); plate emergence.

## Discussion

Sea level already pours a conserved ocean against solid hypsometry. Without a mass-conserving solid column on the other side of that pour, "emerged land" is either baked into a prior (forbidden by the ladder) or faked by a one-sided rate field. The isostasy column is the missing dual of the hydrosphere box: rock inventory with a readable freeboard, so erosion and sea-level both consume truth rather than costume.

## Working Notes

- **Live modules:** `lithosphere.rs` (column + `freeboard_m`); `sea_level.rs` pour; `uplift.rs` **rate only**; nomotheke `LITHOSPHERE` / `ISOSTASY`; erosion consumes `ROCK_UPLIFT_RATE` and emerged-land geometry from isostasy.
- **Do not re-open:** fBm freeboard as land keeper; baking bimodal land into the bathymetry prior; plate state-fields as the path to first freeboard; claiming Kept from Claimed alone.
- **Regression guards (tests):** range spans zero; mean freeboard ≡ 0 at any $T_p$; cooling grows craton/ocean contrast; keel share > 15%; craton-fraction calibration band; ordinum Claimed-by-isostasy (history of unkept-is-the-point in the test comment).
- **Next convicting work:** rock-mass return from erosion; mantle-thermal as a nomos; water loading; generic BrokenKeeper range/reachability machinery; Record-style land-fraction predicate probe (Claimed → Kept path).
- **Primary literature:** Chowdhury, Cawood & Mulder 2025 (relata: chowdhury-2025-subaerial); early-continents survey for Record targets.
- Sibling: #detail-erosion-composition ; #detail-phase-abyssal ; open mantle-thermal; router under #obs-cube-locked-kernel-bias .
