# World-Phases & Checkpoints

*(Joseph's draft, 2026-07-03; second pass same day. **Phase** = a span in
which a set of macro systems runs until converged; **Checkpoint** = the gate
between phases — the conditions a phase must deliver before the next can
honestly boot. "Phase" avoids three collisions on "epoch": geology's, the
erosion solver's step unit, and ordinary usage.)*

**The load-bearing structure:** each phase's **Charge** (what it must
establish) IS the next checkpoint's opening condition — *each phase
manufactures the key to the next gate* (Abyssal's photosynthesizers ARE the
mechanism that oxygenates Primeval's sky). A phase boundary is drawn
correctly exactly when this holds. Charges are stated once, in the phase
responsible; a phase "opens at" the delivery of the previous charge.

**Computational character (Joseph):** phases choose their own machinery. Some
are **analytic** — closed-form + stochastic-deterministic seeding (no
simulating 20M years of gas-cloud collapse); some are **relaxation** — solve
or settle to an attractor and checkpoint it; some are **incremental-emergent**
— stepwise simulation on the phase's OWN timescales, spatial scales, and even
data structures. Later phases may be evaluated lazily backwards-from-now
(`DESIGN-REDUX.md` §3/§11); each phase knows which of its parts decompose
independently / in parallel / on demand. **A checkpoint is a world-scale memo
entry (§12–13): the converged state, cached, that the next phase builds on —
and its in-world form is the STRATA it leaves** (chronostratigraphic
signature noted per phase: the save-file is readable in the cliffs). Materials
are therefore phase-dependent: a world that never ran biomineralization has
no limestone.

**Condition labels** (epistemic honesty about the gates): **[gate]** hard
prerequisite · **[earth]** Earth's pattern, causality plausible, kept for
fidelity · **[mech?]** causality likely/known but no simulable mechanism yet
(the "implied" hand-waves, honored as such) · **[emergent]** expected to
arise — verify, don't build.

**Where vivarium stands (2026-07-03):** an Abyssal world wearing a Primeval
sky — geology mid-Phase-3 (erosion/hydrology running), astronomy borrowed
from Phase 4 (crisp sun), biosphere: none yet.

---

## Phase 0 — **Ante-mundane** · *before the world*
Cosmic scale; everything prior to the world. **Analytic: pure parameters**
(element abundances, orbital configuration, seed).
**Charge:** stable orbit in the habitable zone of a viable, protected system
[gate] · proper mass [gate] · Earth-like elemental abundances [gate] ·
protective architecture — moon, outer giants, solar wind shelter [mech?]
(the moon cannot be added non-destructively later, so it is implied here).
*Record: none in-world — only the parameter block itself.*

## Phase 1 — **Protogenic** · *the world is now **Formed***
A sphere orbiting its star, featureless, hot, brooded upon. **Analytic**
(this phase is literally our `Planet` module today: sphere, tilt, spin,
insolation rhythm as pure functions).
**Charge:** axial tilt [gate] · spin [gate] (possibly differential layers —
the magnetic-dynamo mechanism [mech?]) · smooth water-covered surface [earth]
· the rhythm of angle-specific solar insolation [emergent — falls out of
tilt+spin+orbit].
*Record: no preserved strata (molten resurfacing); bulk composition only.*

## Phase 2 — **Primordial** · *the world is now **Divided***
Gas / liquid / solid separate: dense hot pre-oxygenation atmosphere, a sea
level, solid ground beneath. Venus-like but far wetter. Light is ambient and
highly diffused — day and night exist, the sun is not yet seen. **Analytic +
relaxation** (reservoir partitions, water cycle to steady state — the
ancestor of today's fill machinery).
**Charge:** atmosphere as a real reservoir, pre-oxygen composition [gate] ·
water cycle closed [gate] · basic weather fluid dynamics and the
useful-energy / entropy-gradient channels [gate] · magnetic field preserving
the atmosphere [mech?] · enough variation, asymmetry, and seeded noise to
awaken Phase-3 processes [gate — §8's honest stochasticity is this
condition's machinery] · **abiogenesis through PHOTOSYNTHETIC sea life
(microbial at least)** [earth; Joseph: Abyssal opens after this exists].
*Record: primordial basement — the first crust.*

## Phase 3 — ☆ **Abyssal** (Deep Time) · *the world is now **Alive***
The world itself becomes a highly dynamic system across great orders of
temporal magnitude. Runs long; continues beneath all later phases as their
geological substrate. **Incremental-emergent, the heavyweight** — its own
timescale ladder (tectonic Myr → erosive Kyr), its own structures; our
erosion telescope and water physics live here. (☆: this phase will want
internal sub-gates when the details force them — it does billions of years
of work.)
**Charge:** plate tectonics and the Wegenerian cycle [mech? today — uplift
stands in] · meaningful non-volcanic land above sea level [gate] ·
mineral systems and cycles [gate] · erosion processes carving real
landscapes [gate — RUNNING today] · sea life complexity through
BIOMINERALIZATION [earth — and it is the mechanism of the next gate] ·
oxygen accumulating toward the Great Oxygenation [emergent from the
photosynthesizers] · alien proto-biomes hostile to complex life [emergent].
*Record: the deep sedimentary stack; banded-iron formations (oxygen meeting
dissolved iron); the FIRST LIMESTONES — life writing itself into rock.*

## Phase 4 — **Primeval** · *the world is now **Awake***
The clouds part; the world beholds the cosmos for the first time — dawn, the
morning sun, the moon in its phases, and at night the stars. **Coupled
relaxation + emergent weather** (climate tier over the Phase-1 insolation).
**Charge:** Earth-like post-oxygenation atmosphere [gate — delivered by
Abyssal's charge, this checkpoint's key] · ozone shield [emergent from
oxygen; the land-life prerequisite] · sun, moon, stars visible at true
angles, phases, seasons [gate] · modern weather dynamics [emergent] ·
nitrogen and companion biogeochemical cycles [earth] · **stationary land
life** — plants colonize the continents [earth; moved here from the draft's
Phase 3: land is lethal without the ozone this phase's sky provides].
*Record: REDBEDS — the oxidation event rusts the rock record; late, the
first soil horizons under land plants.*

## Phase 5 — **Archaic** · *the world is now **Abundant***
Life as we know it, fully adapted to the world it helped create: agents in
the sea, on land, in the air. **Statistical ecology → agent-based at fine
LOD.**
**Charge:** oxygen-breathing complex mobile sea life, abundant and diverse
[earth] · mobile land life through FLIGHT — birds, fowl, reptiles of every
variety [earth] · resilient, calibrated evolutionary pressures and
energy-gradient-driven diversity mechanisms [mech?].
*Record: the fossil-bearing strata begin.*

## Phase 6 — **Aboriginal** · *the world is now **Recognizable*** — TARGET 1
Earth since shortly after the Cenozoic dawn: the Age of Mammals.
**Charge:** mammals and the recognizable kingdom/taxonomy diversity across
land, air, sea [earth] · modern flora and fungus ecosystems — flowers and
fruit, grain and seed, wood that decays [earth] · modern insect world and
microbiome [earth] · complex intelligence, communication, and agency —
Pleistocene-grade life [gate].
**This is the phase where the primary simulations with real agent
interactions begin — the ASF bet's home.**
*Record: recognizable modern strata; the first anthropic traces only in Phase 7.*

## Phase 7 — **Prehistoric** · *the world now has **People*** — TARGET 2
Not mankind per se, but speech, symbolic thought, emotion, self-determination,
higher-order agency. No creature has yet frozen thought onto matter — none has
a written language that outlives the mind that thought it, that could move a
thought **back out of the timestream** the first phases started. **Agent-based
(the slow ASF/LLM layer engages).**
**Charge:** hominids or an equivalently-ordered agent & intelligence [earth] ·
speech / language — high-enough-fidelity, adaptable, arbitrarily abstractable
person↔person thought transfer [gate] · agriculture (likely) [earth] ·
non-genetic cross-generational transfer: culture, durable familial/tribal
cohabitation [earth].

> ⚠️ **This checkpoint is where obligation begins.** "The world has People" is
> precisely the point at which the builder acquires duties to what is inside
> it: **`ETHICS.md`'s constraints bind from here forward.** The moment an agent
> in this world is driven by the slow (LLM) layer it enters moral scope
> (DESIGN.md §two-layer mind). This line is load-bearing and must survive every
> future revision of this file.

*Record: the first symbolic artifacts, hearths, worked tools — culture entering
the strata, but not yet writing.*

## Phase 8 — **Historical** · *the world now has **Writing** and history* — TARGET 3
From here the space forks wide — real history, fantasy settings, game and story
worlds — anything from the dawn of writing through science-fiction mechanics.
The checkpoints end; authored worlds begin. **Author-driven over the simulated
substrate.**
**Charge:** written language — with whatever agent/intelligence substrate it
requires — and most of what every prior phase built [gate]. Thought moves back
out of the timestream; history becomes possible, and the world's record is no
longer only geological but *documentary*.
*Record: the written layer — the first record the world keeps of itself, on
purpose.*

---

## Design notes

- **Flat, not nested.** No Eon/Era/Period hierarchy — false precision for a
  gate-list. If any phase earns internal structure it is **Abyssal** (☆); its
  sub-gates grow when the details force them, not before.
- **Gate status ≠ sim status.** The `[gate]/[earth]/[mech?]/[emergent]` labels
  say what *kind* of arrow a condition is, never whether it is built.
  Implementation state lives elsewhere (the §12 recipe keys, or a tracking
  layer) so this document never quietly claims the world exists.
- **Phases own their intrusion budget.** Each declares which effects reach
  forward past its checkpoint and which seal at it — and, dually, which of its
  parts decompose independently / in parallel / lazily-backwards-from-now (§3,
  §11). The checkpoint is the multirate coupling interface (§4) at the largest
  scale we have.

---

## Algorithms & approaches ledger *(first pass — accrete as we hit them in practice)*

*A place to park what we learn about HOW each process wants to be modelled, the
day we hit it (today: scouring/armoring). Not a plan — a growing map.*

**Approach codes** (a process may blend them): **A** analytic / closed-form /
functional (+ §8 seeded stochasticity) · **R** relaxation-to-attractor (settle,
then checkpoint-cache) · **S** statistical / empirical (fit, lookup, sub-grid
parameterization) · **T** taxonomy / bestiary import (curated tables — materials,
biomes, species) · **P** procedural-physical stepwise sim *(annotate physics-
understood hi/med/lo + cost)*.

**Reading the "agent-fidelity" column — the prioritization backbone.** It asks:
does an inhabitant of the final three (agent) phases *perceive, depend on, or act
through* this? That is why we may run tectonics crude forever but must get rivers
and biomes right — an agent fords a river and forages a biome; none has ever
touched mantle convection. Work backward from what agents touch.

### Geology & landscape (Phase 3, mostly)
| process | approach | physics | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| terrain prior | A (2-band fBm) | proxy | O(n·oct) | low (sets stage) | built |
| tectonic uplift | A stand-in → P (mantle) | lo | — | low-direct | crude (fBm) |
| stream-power incision (Shields τ_c) | P | hi | O(n)/epoch | **high** (the land) | built |
| Priority-Flood depressions | P/algo | hi | O(n log n) | high | built |
| MFD drainage | P | hi | O(n) | high | built |
| hillslope creep (diffusion) | P | hi | O(n) | med | built |
| glaciation · volcanism · karst · aeolian · coastal | P/S | med | varies | med (landforms) | — |

### Water & sediment (Phase 3 — today's earned detail)
| process | approach | physics | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| shallow-water flow (local-inertial pipes) | P | hi | O(n)/step, CFL | **high** (fording, mills, drink) | built + stabilized |
| suspension + shear-gated settling (Rouse) | P | med | O(n) | med (clarity) | built crude |
| turbulent eddy diffusion | P | hi | O(n) | med | built today |
| armoring + winnowing | P | med | O(n) | med (stony beds) | built, probe-partial |
| colmation (fines seal) | P | med | O(n) | low-med | built |
| **grain-size split + Stokes** (→ saltation, traction, sorting) | P | hi | O(n·k) | med | **planned — top pick** |
| **bank mechanics** (Mohr–Coulomb + undercut → meander, oxbow) | P | hi (geotech) | O(n) | **high** (banks, fords, slumps) | planned |
| Darcy lateral groundwater (→ springs) | P | hi | O(n) | med | planned |
| hyperconcentrated / Bingham + flow-bulking | P | med | O(n) | med (flash-flood hazard) | named |
| analytic hydrological init | A + R | hi | O(n log n) | indirect (clean worlds) | planned |

### Climate & weather (Phase 2 base, Phase 4 modern) — the next system
| process | approach | physics | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| insolation (declination/hour-angle) | A | hi | O(1)/query | med | built |
| climate (T/precip ← lat, elev, continentality) | S + A | med | O(n) | **high** (biomes, seasons) | **next** |
| storms / weather | A jitter → P | med | O(n) | **high** (agents feel it) | crude (jittered) |
| orographic rain, rain shadow | P/S | med | O(n) | high | planned |

### Materials & record (spans all phases)
| process | approach | physics | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| material taxonomy (undifferentiated → refined) | T + A (det. refine) | — | O(1) | **high** (what things are) | schema |
| strata / column | data structure | — | — | high (digging, cliffs) | frame built |
| chronostratigraphic record = the checkpoint artifact | (emerges) | — | — | high (readable history) | conceptual |

### Biosphere (Phases 3→6; final-3-critical) — mostly import + statistics
| process | approach | physics | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| biomes (Whittaker T×P) | T + S lookup | — | O(1)/cell | **high** | — |
| vegetation / succession | S + T | med | O(n) | **high** (food, cover) | — |
| species / fauna | **T (bestiary import)** | — | curated | **high** | — |
| evolution / speciation | S / procedural | lo | — | low-med (flavour) | defer |

### Agents — the real bet (Phases 6→8)
| process | approach | physics | cost | agent-fidelity | status |
|---|---|---|---|---|---|
| fast ASF/AAT layer (formal dynamics) | P | hi (our theory) | O(agents)/tick | **the point** | the bet |
| slow layer (LLM at aporia) | import (the model) | — | event-driven | **the point** | the bet |
| cognitive LOD swap | A (component swap) | — | ~free | high | designed |

*Note on complexity: the per-cell O(n) stencils are the cheap common case —
memory-bound, and the GPU/rayon path (`ref/research/water-parallelism.md`) is
what keeps them affordable as fidelity and area grow. The expensive-but-rare
ones (Priority-Flood O(n log n), any global solve) run once per checkpoint and
cache. The genuinely open-complexity items are the ones marked lo physics
(tectonics, evolution) — where we deliberately run a cheap statistical stand-in
because agent-fidelity does not yet demand more.* 