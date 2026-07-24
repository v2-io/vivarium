# Material & terrain data-models — survey (deep-research, adversarially verified)

*Preserved from a `/deep-research` workflow: 106 agents, 5 search angles → fetch →
3-vote adversarial verification (needs 2/3 refutes to kill a claim) → cited synthesis.
Feeds `.super-archive/from-design/DESIGN-MATERIAL.md` and `doc/design/DESIGN-REDUX.md` §15. Findings are the survivors;
the `Refuted` section lists what got voted down — do not trust those.*

## Question

What is the minimal, future-proof set of per-cell / per-material / per-column-state PROPERTIES a voxel world-simulation substrate must store to serve human-scale physical queries — and how do different terrain/material simulation approaches represent matter?

Context: a deterministic voxel world at 0.5 m isotropic resolution; columns represented as strata (material runs with real thickness) plus cross-cutting bodies (intrusions/caves); a fidelity-ladder architecture where a stable property "interface" lets the material MODEL behind it be swapped (crude→high) without changing consumers.

Survey these approaches; for EACH, extract the exact per-cell properties and dynamic STATE it assumes/requires, and place it on a crude→high-fidelity ladder:
1. Block/tile material models in games — Minecraft, Dwarf Fortress, Terraria (material as discrete type; per-type attributes like falls-vs-solid, hardness, flammability).
2. Cellular-automata / angle-of-repose granular — falling-sand, Noita, powder-toy; slumping via repose threshold; simple fluid flow.
3. Geotechnical slope-stability / mass-wasting — Mohr–Coulomb shear strength (cohesion c, internal friction angle φ), infinite-slope factor-of-safety, pore-water pressure, bearing capacity, lateral earth pressure (overburden ∫ρg dz, earth-pressure coefficient K).
4. Hydrology + erosion — virtual-pipes shallow-water (Mei et al. 2007), stream-power / Davy–Lague fluvial incision+deposition, particle/droplet erosion, thermal erosion; sediment transport (grain size, transport capacity).
5. Granular / continuum physics — DEM (discrete element method), continuum granular rheology (μ(I)) — the high-fidelity rung.

Human-scale queries the substrate must serve (map each required property to the queries that consume it): surface slope for climbability; slope + cohesion + saturation for slumping; overburden/lithostatic and lateral pressure through the column; foot-placement / bearing capacity / resulting stand-height; reachability/energetics between adjacent cells (potential energy, traversal cost); fluidity / predilection-to-change on seconds-to-weeks timescales; stickiness/cohesion to neighbors.

Deliverable: (a) a per-approach table of assumed properties + state; (b) the MINIMAL SPANNING property set (the union), each property annotated with which queries and which models need it, plus SI units; (c) a fidelity-ladder ordering of the approaches; (d) classify each property as per-material-type vs per-stratum vs per-column-dynamic-state vs derived-geometric; (e) flag any standard property NOT already on our list (current list: density, cohesion, friction angle, grain size, rheology class, porosity/permeability, saturation/pore-pressure, temperature). Cite sources.

## Executive summary

The survey confirms a coherent crude-to-high fidelity ladder and a near-complete minimal spanning per-cell property set. At the crude rung, block/tile games (Dwarf Fortress) store matter as a discrete material TYPE carrying fixed per-type attributes: density (separate solid/liquid values in kg/m3), an enumerated phase (SOLID/LIQUID/GAS/POWDER/PASTE/PRESSED that governs falling-vs-flowing), and per-mode yield/fracture/strain-at-yield for six stress modes rather than a scalar hardness. The middle rung is geotechnical plus hydrologic: Mohr-Coulomb shear strength (cohesion C in Pa, internal friction angle phi in degrees) plus normal/overburden stress (sigma'v = gamma*z, gamma = rho*g) and pore-water pressure feed an infinite-slope factor of safety, while shallow-water erosion needs only a small per-column dynamic state (terrain height b, water depth d, mobile-sediment/regolith thickness r) with material erodibility collapsible to a single scalar or the physically-grounded stream-power form I = K*A^m*S^n. Critically, coupled models (SHALSTAB) show slope-stability and water-routing draw on the SAME shared per-cell state, so one substrate serves both query families. The high-fidelity rung is granular physics: continuum mu(I) rheology (mu1, mu2, I0; inertial number from grain diameter d, grain density, confining pressure, shear rate; cohesion via modified pressure p-bar = p + beta*pe), and at the top DEM with per-particle micro-contact parameters calibrated so the emergent angle of repose (approx equal to internal friction angle for cohesionless media) matches measurement. The union largely confirms the project's current property list is spanning; the notable additions to flag are an explicit erosion/incision threshold, packing/volume fraction, and separate solid/liquid density plus a discrete phase-state enum.

## Verified findings

### Crude rung (block/tile games): matter is a discrete material TYPE with fixed per-type attributes. Dwarf Fortress stores per-material density (SOLID_DENSITY and LIQUID_DENSITY in kg/m3, plus unused MOLAR_MASS for gas), an enumerated phase/state with six discrete values (SOLID, LIQUID, GAS, POWDER, PASTE, PRESSED) that distinguishes falling/flowing matter from solid, and mechanical strength as per-mode YIELD/FRACTURE/STRAIN_AT_YIELD for six independent stress modes (impact, compressive, tensile, torsion, shear, bending) rather than a single scalar hardness.

**Confidence:** high · **Vote:** 3-0 (all three sub-claims unanimous)

Density is per-material-type with separate solid/liquid forms in explicit SI kg/m3; MOLAR_MASS marked unused for gas. Six enumerated material states confirmed verbatim (POWDER/PASTE/PRESSED are formally solid variations). Six force types each carry three tokens (_YIELD, _FRACTURE, _STRAIN_AT_YIELD). Note: a refuted companion claim tried to pin exact SI stress units (raw token = Pa) and density in g/cm3 to these tokens; that unit-mapping was voted down 0-3, so treat DF strength values as game-scaled, not directly SI. These map onto the substrate's per-material-type properties: density (kept), a phase-state enum (NEW), and multi-mode strength (crude analog of cohesion/hardness).

**Sources:** https://dwarffortresswiki.org/index.php/DF2014:Material_definition_token  ·  https://dwarffortresswiki.org/index.php/DF2014:Material_science

### Middle rung (geotechnical): soil shear strength follows Mohr-Coulomb, strength = C + sigma*tan(phi), requiring two per-material properties (cohesion C, internal friction angle phi ~30deg sand, 15-25deg clay) plus normal stress sigma. For a dry infinite slope the factor of safety is F = (C + t*gamma*cos(beta)*tan(phi)) / (t*gamma*sin(beta)), needing cohesion C, friction angle phi, unit weight gamma (= density*g), slope-normal thickness t, and slope angle beta; F<1 means impending failure. For cohesionless material (C=0) this reduces to F = tan(phi)/tan(beta), so the angle of repose of a cohesionless pile equals its internal friction angle.

**Confidence:** high · **Vote:** 3-0

Canonical soil-mechanics results (Coulomb 1776/Mohr 1900; Das, Craig). Independent derivation reconciles the slope-normal-thickness form with the standard vertical-depth form via t = z*cos(beta). Maps directly to the substrate's slope (derived-geometric), cohesion and friction angle (per-material-type), density (per-material) and column thickness/saturation (per-column-dynamic-state) for climbability and slumping queries. Rigorously Mohr-Coulomb should be in effective stress tau = c' + (sigma - u)*tan(phi'), which motivates storing pore pressure.

**Sources:** https://web.pdx.edu/~i1kc/programming/slopes/LandslideNotes.pdf

### Pore-water pressure / saturation is a required dynamic-state group (alongside strength and geometry) for slumping/mass-wasting. Slope destabilization is driven by pore-pressure evolution from subsurface runoff (Terzaghi effective stress sigma' = sigma - u reduces Mohr-Coulomb strength). With seepage parallel to the slope the cohesionless factor of safety becomes F = [1 - (gamma_w/gamma_t)]*tan(phi)/tan(beta), reduced from the dry case by the ratio gamma_w/gamma_t (~0.5 for typical soils), making slope-parallel seepage the least-stable condition.

**Confidence:** high · **Vote:** 3-0

Textbook effective-stress mechanics (Craig, Das). Confirms saturation/pore-pressure (per-column-dynamic-state, derivable from saturation + column properties) is load-bearing via water unit weight gamma_w and total unit weight gamma_t. Consistent with the project's existing saturation/pore-pressure property.

**Sources:** https://web.pdx.edu/~i1kc/programming/slopes/LandslideNotes.pdf  ·  https://pubs.usgs.gov/publication/70157567

### The same substrate must serve BOTH slope-stability and water-routing queries from shared per-cell state: shallow landslide susceptibility (SHALSTAB) is a coupled steady-state runoff (hydrology) model plus an infinite-slope stability model, where per-cell slope + saturation/pore-pressure + cohesion + friction angle jointly determine both hydrologic wetness and factor of safety.

**Confidence:** high · **Vote:** 3-0

USGS/Bellugi et al. 2011 and Montgomery & Dietrich 1994 (canonical). This is the strongest architectural evidence for the fidelity-ladder 'shared property interface': overburden/pore-pressure and drainage/wetness are the coupling variables, so slope-stability and hydrology consumers read the same per-column state rather than duplicating it.

**Sources:** https://pubs.usgs.gov/publication/70157567

### Overburden/lithostatic and lateral earth pressure are column integrals of the stored density field. Vertical effective stress accumulates with depth as sigma'v = gamma*z (unit weight times depth), i.e. overburden = integral of density*g over the column; lateral earth pressure = K * vertical effective stress, where K (earth-pressure coefficient) is the ratio of horizontal to vertical effective stress.

**Confidence:** high · **Vote:** 3-0

Standard Rankine/Coulomb earth-pressure theory. Classifies overburden and lateral pressure as DERIVED (per-column integral of density*g) rather than stored, needing only per-material density plus g. Important nuance flagged by verification: sigma'v = gamma*z holds exactly only when pore pressure u = 0 (dry/above water table); below the water table sigma'v = sigma_v - u < integral(rho*g*dz). This effective-vs-total distinction is precisely why saturation/pore-pressure must be a stored/derivable state property.

**Sources:** https://en.wikipedia.org/wiki/Lateral_earth_pressure

### Hydrology/erosion (middle-to-high rung), fluvial incision: the stream-power incision model (SPIM) gives long-term downcutting rate I = K*A^m*S^n (K erodibility, A upstream drainage area, S slope, m/n global exponents), so a detachment-limited incision model needs only per-cell drainage area, local slope, and a material erodibility parameter as minimal state. Crucially, the bare SPIM is NOT universal: all published incising-river datasets away from knickpoints are threshold-dominated, so a realistic model requires an explicit erosion threshold plus stochastic-discharge upscaling beyond K*A^m*S^n.

**Confidence:** high · **Vote:** 3-0

Lague 2014 ESPL, the canonical SPIM review. A is derived per-cell via global flow accumulation; K is per-material erodibility; m,n are global constants (not per-cell state). The threshold/stochastic finding is the key NEW property flag: an explicit incision/erosion threshold (tau_c) is not on the project's current list and should be added as a per-material property.

**Sources:** https://wpg.forestry.oregonstate.edu/sites/default/files/seminars/2014_Lague_ESPL.pdf

### Shallow-water erosion (crude/interactive rung) minimal per-column dynamic state is small: a layered 2D height field storing bottom/terrain height b, water depth d, and mobile-sediment (regolith) thickness r (with water level h = b + r + d and regolith level br = b + r derived). Material erodibility can collapse to a single per-material scalar cr (useful range 0.000-0.01) that sets both maximum erosion/penetration depth and regolith saturation level -- the crude single-parameter end of the material-property ladder, one scalar standing in for grain size / cohesion / erodibility.

**Confidence:** high · **Vote:** 3-0

Benes 2007 (real-time erosion via shallow water). Independent dynamic state = b, d, r; h and br are algebraic. The authors themselves note one constant cr is 'far from reality' -- explicitly framing it as the crude rung of a material-property ladder. Caveat: this wave-equation model stores no explicit velocity/flux or suspended-vs-bed sediment; flux-based virtual-pipes models (Mei et al. 2007) add outflow flux, velocity, and suspended sediment as additional per-cell state, so 'minimal' here is model-specific.

**Sources:** https://www.cs.purdue.edu/cgvlab/www/resources/papers/Benes-2007-Real-Time_Erosion_Using_Shallow_Water_Simulation.pdf

### High-fidelity rung (continuum granular): dense granular flow obeys a local mu(I) rheology where effective friction mu = mu1 + (mu2 - mu1)/(I0/I + 1) is a function of a single dimensionless inertial number I = shear_rate * d / sqrt(sigma_n / rho_p). This requires per-material parameters mu1, mu2, I0 (lower/upper friction bounds, reference inertial number), grain diameter d, particle density rho_p/rho*, packing/volume fraction, and dynamic state (local confining/normal stress and shear rate). Cohesion is encoded not as a single scalar c but as a dimensionless parameter beta acting through a modified pressure p-bar = p + beta*pe, where pe is the isotropic compressive (cohesive) strength.

**Confidence:** high · **Vote:** 3-0

Forterre & Pouliquen ARFM 2008 and Jop/Forterre/Pouliquen (Nature 2006) establish mu(I); the JFM cohesive extension adds beta and pe. mu(I) is explicitly a LOCAL rheology that fails near jamming/where nonlocal effects dominate. Classifies: mu1/mu2/I0/beta/grain-diameter/density = per-material-type; packing fraction = state/output (constant in incompressible mu(I), a phi(I) relation in compressible form); normal stress and shear rate = per-column-dynamic-state. This is the physically-grounded high rung that the crude erodibility scalar cr stands in for. Flags packing/volume fraction as a property NOT on the current list.

**Sources:** https://www.phys.sinica.edu.tw/jctsai/Ray2016/Reference/%5B8%5D_Pouliquen_2008ARFM_Flow_of_Dense_Granular_Media.pdf  ·  https://www.cambridge.org/core/journals/journal-of-fluid-mechanics/article/critical-state-irheology-model-for-cohesive-granular-flows/668CC9AA25AFF9001F756EE02841FA52

### Highest rung (DEM, discrete element method): granular flow is resolved per-particle with a Hertz-Mindlin micro-contact parameter set -- particle density, Young's modulus E, Poisson's ratio, coefficient of restitution / normal+tangential damping, inter-particle and particle-surface sliding (static) friction, and rolling friction -- plus particle radius/size. These micro-inputs map the substrate's density, friction-angle, grain-size, and rheology-class properties onto explicit contact parameters. Inter-particle static (sliding) friction is calibrated against the macroscopic angle of repose (quartz sand ~34deg via funnel-pour cone test) and is the dominant control on emergent repose angle, with rolling friction secondary -- so a friction-angle-like property is primary to store while stiffness parameters can be coarser.

**Confidence:** high · **Vote:** 3-0 on the parameter set; the 'friction dominates repose, stiffness can be coarser' extrapolation was 2-1

PMC11478281 Table 3 gives the exact canonical set (density 2600 kg/m3, E 1 GPa, Poisson 0.22, inter-particle friction 0.6, particle-surface 0.55, rolling 0.1, damping 0.7); Springer 2015 confirms and shows static friction is the primary repose-angle control. Honest qualifications reducing the 2-1 extrapolation confidence: (1) the friction->repose mapping is non-unique (a single repose angle spans a wide range of friction pairs, so one stored parameter under-determines behavior); (2) for perfectly spherical particles rolling friction is ESSENTIAL, not merely secondary -- sliding friction alone saturates. The claim survives because it says rolling friction is 'secondary' (not negligible) and stiffness 'can be coarser' (not omitted). This is the top of the fidelity ladder that a stored friction-angle/grain-size property interface abstracts.

**Sources:** https://pmc.ncbi.nlm.nih.gov/articles/PMC11478281/  ·  https://link.springer.com/article/10.1007/s40571-015-0056-5

## Open questions

- Should cohesion be stored as a single scalar c (Mohr-Coulomb, per-material) or split into a dimensionless cohesion parameter beta plus a pressure-like compressive strength pe (cohesive mu(I) rheology)? The higher rung wants the two-part encoding; the interface question is whether the crude consumers can live with the projection down to a single c.
- Where does the erosion/incision THRESHOLD (tau_c) live on the ladder -- is it a distinct per-material property, or derivable from cohesion + grain size + density already on the list? Lague 2014 shows it is empirically non-optional, but the survey did not settle whether it is independent state or derived.
- Is packing/volume fraction a per-material CONSTANT (incompressible mu(I)) or a per-column DYNAMIC state that evolves with compaction (compressible mu(J)-phi(J) / critical-state form)? This determines whether it belongs in the per-material-type bucket or the per-column-dynamic-state bucket.
- For velocity/flux: the wave-equation shallow-water model (b,d,r) needs no explicit velocity, but flux-based virtual-pipes hydrology (Mei et al. 2007) requires per-cell outflow flux + velocity + suspended sediment. Which hydrology model the substrate commits to changes the minimal per-column dynamic-state set -- unresolved by the surveyed claims.
- How should the many-to-one DEM/repose friction calibration be handled when swapping crude->high fidelity, given that a single stored friction-angle-like scalar under-determines granular behavior (rolling vs sliding friction ambiguity, especially for near-spherical grains)?

## Caveats


## Unverified (surfaced but not confirmed)


## Refuted (voted down — NOT reliable)

- DF stores material yield/fracture strength in stress units (raw token value = Pa; MPa = raw/10^6) and density in g/cm^3, giving the properties concrete SI-mappable units.

## Sources

-  https://dwarffortresswiki.org/index.php/DF2014:Material_definition_token
-  https://dwarffortresswiki.org/index.php/DF2014:Material_science
-  https://www.gamedeveloper.com/design/video-understanding-the-remarkable-tech-and-design-of-i-noita-i-
-  https://blog.okkohakola.com/SandFall/SandFallIntro
-  https://teardown.fandom.com/wiki/Voxels
-  https://mcreator.net/wiki/list-block-flammability-values
-  https://link.springer.com/article/10.1007/s11629-021-7057-z
-  https://pubs.usgs.gov/publication/70157567
-  https://www.geological-digressions.com/mohr-coulomb-failure-criteria/
-  https://en.wikipedia.org/wiki/Lateral_earth_pressure
-  https://www.geoengineer.org/education/earth-retaining-structures/retaining-walls/lateral-earth-pressure-states-and-coefficients
-  https://www.geofem.com/post/explaining-the-mohr-coulomb-failure-criterion
-  https://inria.hal.science/inria-00402079/en/
-  https://agupubs.onlinelibrary.wiley.com/doi/full/10.1029/2008JF001146
-  https://wpg.forestry.oregonstate.edu/sites/default/files/seminars/2014_Lague_ESPL.pdf
-  https://huw-man.github.io/Interactive-Erosion-Simulator-on-GPU/
-  https://en.wikipedia.org/wiki/%CE%9C(I)_rheology
-  https://www.phys.sinica.edu.tw/jctsai/Ray2016/Reference/%5B8%5D_Pouliquen_2008ARFM_Flow_of_Dense_Granular_Media.pdf
-  https://pmc.ncbi.nlm.nih.gov/articles/PMC11478281/
-  https://link.springer.com/article/10.1007/s40571-015-0056-5
-  https://www.cambridge.org/core/journals/journal-of-fluid-mechanics/article/critical-state-irheology-model-for-cohesive-granular-flows/668CC9AA25AFF9001F756EE02841FA52
-  https://www.geotechdata.info/parameter/
-  https://www.cs.purdue.edu/cgvlab/www/resources/papers/Benes-2007-Real-Time_Erosion_Using_Shallow_Water_Simulation.pdf
-  https://web.pdx.edu/~i1kc/programming/slopes/LandslideNotes.pdf
