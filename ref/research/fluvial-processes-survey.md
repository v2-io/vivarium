# Fluvial processes — survey reference (via Joseph, 2026-07-03)

*Provenance: AI-assisted survey passes Joseph ran and pasted into session
2026-07-03, preserved as project reference. External links are AS GIVEN and
UNVERIFIED — verify any before external citation (project discipline). The
distilled inventory with our implementation status lives in
`.super-archive/from-design/DESIGN-SYSTEMS.md` §Sediment & fluvial phenomena; this file keeps the fuller
source material and the reference trail.*

## The process taxonomy (survey 1)

Transport: suspension · saltation · traction · solution (dissolved) ·
flocculation (brackish clay clumping).
Bed/channel: armoring · imbrication · dune/ripple migration · bed-load
piping · colmation.
Erosional mechanisms: abrasion/corrasion (tools) · hydraulic action ·
cavitation · attrition (→ downstream fining) · corrosion/solution · bank
slumping/mass wasting.
Deposition: alluviation · aggradation · splay formation (levee breach) ·
deltaic progradation.
Morphology (emergent): meandering · avulsion · braiding · incision/
degradation · headward erosion.

## Fundamental vs emergent (survey 2) — the load-bearing decomposition

Only the FUNDAMENTALS need simulating; the rest should emerge:
- Suspension: u/u′ (turbulence) vs w_s(d, ρ_s); fluid ρ, μ (T, mud conc!).
- Saltation/traction: Shields τ_c; grain distribution D50/D84; roughness k_s.
- Hydraulic action: boundary τ_b vs cohesion/critical strength τ_s; slope S.
- Abrasion: sediment concentration C (TOOLS); hardness contrast; impact v.
- Mass wasting: pore pressure u_w; internal friction φ; bank geometry H, θ.
- Solution: pH, mineral solubility, T. Flocculation: salinity, clay type.

Worked emergence examples from the survey: armoring = selective Shields
(D10 mobile, D84 not) — hence our explicit armor field is scaffolding with a
demolition date (DESIGN-SYSTEMS); meandering = hydraulic action + abrasion +
alluviation looped through helical flow.

## Flash-flood debris waves (survey 3) — the hyperconcentrated regime

- Flow BULKING: erosion feeds volume/density → ρ, μ rise → shear rises →
  more erosion (a flood can triple its volume; becomes slurry).
- Rheology: Bingham plastic / Voellmy — YIELD STRESS: solid until stress
  threshold, then scouring liquid. (Our mud lakes currently flow like clear
  water — the named missing physics.)
- Debris jams at constrictions → localized vortex scour (needs woody-debris/
  boulder BODIES, §14 overlay).
- Dynamic avulsion: the wave clogs its own channel and jumps.
- Software lineage for this regime: FLO-2D, RAMMS.

## External references (as given; unverified)

Software: HEC-RAS (hec.usace.army.mil/software/hec-ras) · Delft3D ·
FLO-2D (flo-2d.com) · RAMMS.
Papers/reports of likely substance:
- Howard 1998 — sseh.uchicago.edu/doc/Howard_1998.pdf (landscape evolution)
- AGU: agupubs.onlinelibrary.wiley.com/doi/full/10.1029/2011JF002005
- NRCan geomorphic flood mapping: natural-resources.canada.ca/science-data/
  science-research/natural-hazards/geomorphic-considerations-flood-mapping
- FHWA hydraulic scour: fhwa.dot.gov/engineering/hydraulics/pubs/hif12003.pdf
- ScienceDirect S0022169418307625 (debris jam scour)
- PMC10881031 (flow bulking)
- Bulking factor study: s47609.pcdn.co/wp-content/uploads/2019/11/
  BulkingFactorStudy-DraftCombinedReport5-15-11.pdf
- Geological digressions, debris-flow lithofacies:
  geological-digressions.com/debris-flow-lithofacies/
(Weaker/tertiary: fiveable.me erosion lists, britannica fluvial-process,
internetgeography, quizlet/coursehero/mytutor study pages — keep only as
breadcrumbs.)

## Equations the surveys point at (we already stand on the first two)

Shields parameter (τ_c) — our incision gate. Exner equation (bed mass
conservation) — our bed±exchange update, discrete. Rouse number (suspension
profile) — behind our fines suspension threshold. Bingham/Voellmy — the
hyperconcentrated rung, not yet built. μ(I) — §15 rung 3.
