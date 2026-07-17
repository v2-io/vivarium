# World-Phases

> **⚠ ARCHIVED 2026-07-11 — this is the reportatio, not the live artifact.** The
> live, codified phase floor is `tabularium/terrestris.ordinum.udon` (the
> Terrestris ordinum), which supersedes this prose and pins it at
> `:reportatio-pin 42621d5`. This file is kept, its body frozen and unedited (Bequest-era
> terms — the ordinum renamed Bequest→Promise), as the honest first-guess the
> ordinum was authorized from. Its Design notes live on in `LEXICON.udon` §1; its
> Algorithms & approaches ledger moved to `doc/design/DESIGN-SYSTEMS.md`. Do not
> tactically depend on this file — read the ordinum.
>
> **⚠ Diverged from the ordinum since archiving (Joseph, 2026-07-16):** the live
> ladder is now **1-indexed** (Ante-Mundane = Phase 1 … final phase = Phase 9), so
> this file's phase HEADINGS (`Phase 0 — Ante-mundane` … `Phase 8`) each read one
> lower than the live number. Body references are not all in the ordinum's sense —
> line 136 cites *"the draft's Phase 3"* — so check the sense before mapping one.
> The final phase — "Phase 8 — Historical" below — is renamed **Scribal** (the
> banner "Historical" implied accuracy-to-Earth; the boundary, writing-onward,
> is unchanged). This file also carries material that never made it into the
> ordinum (Joseph, 2026-07-16), so it will be revisited; the divergence list above is
> expected to grow until it is.

*(Joseph's draft, 2026-07-03; second pass same day; boundary terminology settled 2026-07-04. **Phase** = a span in which a set of macro systems runs until converged; **phase-transition** = the regime-change at a **phase-gate**, where a phase's **Charge** (what it must deliver) is met and the next phase's systems spin up. "Phase" avoids three collisions on "epoch": geology's, the erosion solver's step unit, and ordinary usage. The boundary's earlier working name — "checkpoint" — is retired: it bundled three distinct things now named apart (the gate, the cached **memo**, the **Record**) and collided with ASF's agent-state checkpoint. See the plumbing note in Design notes.)*

**The load-bearing structure:** each phase's **Charge** (what it must establish) IS the next phase-gate's opening condition — *each phase manufactures the key to the next gate* (Abyssal's photosynthesizers ARE the mechanism that oxygenates Primeval's sky). A phase boundary is drawn correctly exactly when this holds. Charges are stated once, in the phase responsible; a phase "opens at" the delivery of the previous charge.

**Computational character (Joseph):** phases choose their own machinery. Some are **analytic** — closed-form + stochastic-deterministic seeding (no simulating 20M years of gas-cloud collapse); some are **relaxation** — solve or settle to an attractor and memoize it; some are **incremental-emergent** — stepwise simulation on the phase's OWN timescales, spatial scales, and even data structures. Later phases may be evaluated lazily backwards-from-now (`doc/design/DESIGN-REDUX.md` §3/§11); each phase knows which of its parts decompose independently / in parallel / on demand. **A phase-transition leaves a world-scale memo (§12–13): the converged state, cached, that the next phase builds on.** Its in-world traces are the whole Bequest — regimes, features, capabilities; the strata are only the Record, the readable-in-rock *slice*, kept as a verification target (never sufficient to reconstruct the phase — see `ref/research/oxygenation-transition-scaffold.md` for the worked proof). Materials are therefore phase-dependent: a world that never ran biomineralization has no limestone.

**Condition tags** (epistemic honesty about the gates; real Obsidian tags, searchable):
- #gate — hard prerequisite we intend to simulate; the next phase cannot boot without it
- #earth — Earth's pattern; causality plausible; kept for fidelity
- #mech — causality likely or known, but **no simulable mechanism yet** (the honored hand-waves; often *declare-causally, materialize-lazily* — see design notes)
- #emergent — expected to arise from the phase's own dynamics; **verify, don't build**

**Bequest vs Charge vs Record:** a phase's **Bequest** is everything it hands
forward that persists and shapes later dynamics — features, regimes,
capabilities (and *incapabilities*: coal exists only because Carboniferous
ecosystems could not yet digest lignin — a phase-window material born of a
temporary inability). The **Charge** is only the *gating subset* of the
bequest. The **Record** is the bequest's *readable-in-rock slice* — the
chronostratigraphic signature, doubling as a verification target: drill a
core through a finished world and the phases must appear in order, or the
machinery is lying.

**Where vivarium stands (2026-07-03):** an Abyssal world wearing a Primeval sky — geology mid-Phase-3 (erosion/hydrology running), astronomy borrowed from Phase 4 (crisp sun), biosphere: none yet.

---

## Phase 0 — **Ante-mundane** · *before the world*

Cosmic scale; everything prior to the world.

**Regime:** analytic — pure parameters (element abundances, orbital configuration, seed). Not a process; a parameter block.

**Charge:**
- Stable orbit in the habitable zone of a viable, protected system #gate
- Proper mass #gate
- Earth-like elemental abundances #gate
- Protective architecture (some implicit) — moon, outer giants, solar-wind shelter #mech
  *(the moon cannot be added non-destructively later, at least in real life, so it is declared here — see design notes)*

**Bequest:** the constants themselves — orbit, mass, chemistry, seed: the box
every later computation happens inside.
*Geological Record: proportions and totals of elements & base columns & most conserved values*

## Phase 1 — **Protogenic** · *the world is now **Formed***

A sphere orbiting its star, featureless, hot, brooded upon.

**Regime:** analytic — this phase is literally our `Planet` module today: sphere, tilt, spin, insolation rhythm as pure functions.

**Charge:**
- Axial tilt #gate
- Spin #gate — possibly differential layers (the magnetic-dynamo mechanism #mech)
- Smooth, water-covered surface #earth
- The rhythm of angle-specific solar insolation #emergent *(falls out of tilt + spin + orbit)*

**Bequest:** the sphere and its rhythms — tilt (every season, forever), spin
(day length; Coriolis, hence every future wind belt and ocean gyre), the
insolation gradient (the skeleton of all climate zones). *Record: none
preserved (molten resurfacing); bulk composition only.*

## Phase 2 — **Primordial** · *the world is now **Divided***

Gas, liquid, and solid separate: a dense, hot, pre-oxygenation atmosphere; a sea level; solid ground beneath. Venus-like but far wetter. Light is ambient and highly diffused — day and night exist, but the sun is not yet seen.

**Regime:** analytic + relaxation — reservoir partitions; water cycle settled to steady state (the ancestor of today's fill machinery).

**Charge:**
- Atmosphere as a real reservoir, pre-oxygen composition #gate
- Water cycle, closed #gate
- Basic weather fluid dynamics; useful-energy / entropy-gradient channels #gate
- Magnetic field preserving the atmosphere #mech *(a canceling pair with the solar wind — see design notes)*
- Enough variation, asymmetry, and seeded noise to awaken the Phase-3 processes #gate
  *(§8's honest stochasticity is this condition's machinery)*
- **Abiogenesis, through photosynthetic sea life** — microbial at least #earth
  *(Joseph: Abyssal opens after this exists)*

**Bequest:** the three reservoirs and their interfaces; the sea-level datum;
a closed water cycle with its energy-gradient channels; the seeded
asymmetries every later phase amplifies. *Record: primordial basement — the
first crust.*

## Phase 3 — ☆ **Abyssal** (Deep Time) · *the world is now **Alive***

The world itself becomes a highly dynamic system across great orders of temporal magnitude. Runs long; continues beneath all later phases as their geological substrate. (☆: this phase will want internal sub-gates when the details force them — it does billions of years of work.)

**Regime:** incremental-emergent, the heavyweight — its own timescale ladder (tectonic Myr → erosive Kyr), its own structures. Our erosion telescope and water physics live here.

**Charge:**
- Plate tectonics and the Wegenerian cycle #mech *(today: fBm uplift stands in)*
- Meaningful non-volcanic land above sea level #gate
- Mineral systems and cycles #gate
- Erosion processes carving real landscapes #gate *(RUNNING today)*
- Sea-life complexity through **biomineralization** #earth *(and it is the mechanism of the next gate)*
- Oxygen accumulating toward the Great Oxygenation #emergent *(from the photosynthesizers)*
- Alien proto-biomes, hostile to complex life #emergent

**Bequest:** THE LAND ITSELF — continents, relief, and drainage networks
(every future rain-shadow, watershed, and harbor is drawn here); mineral and
ore bodies; soil parent material; an ocean-atmosphere partway to breathable.
*Record: the deep sedimentary stack; banded-iron formations (oxygen meeting
dissolved iron); the FIRST LIMESTONES — life writing itself into rock.*

## Phase 4 — **Primeval** · *the world is now **Awake***

The clouds part; the world beholds the cosmos for the first time — dawn, the morning sun, the moon in its phases, and at night the stars.

**Regime:** coupled relaxation + emergent weather — the climate tier over the Phase-1 insolation.

**Charge:**
- Earth-like post-oxygenation atmosphere #gate *(delivered by Abyssal's charge — this gate's key)*
- Ozone shield #emergent *(from the new oxygen; the land-life prerequisite)*
- Sun, moon, and stars visible at true angles, phases, and seasons #gate
- Modern weather dynamics #emergent
- Nitrogen and companion biogeochemical cycles #earth
- **Stationary land life** — plants colonize the continents #earth
  *(moved here from the draft's Phase 3: land is lethal without the ozone this phase's sky provides)*

**Bequest:** a breathable sky and its ozone roof; climate zones and modern
circulation — including the coupled ocean-atmosphere oscillations (the
ENSO-analogs, droughts, monsoons later phases will live by); vegetated
continents that change albedo and slow erosion (roots are a geological
force); the visible cosmos — the clock later navigators and calendars read.
*Record: REDBEDS — the oxidation event rusts the rock record; late, the
first soil horizons under land plants.*

## Phase 5 — **Archaic** · *the world is now **Abundant***

Life as we know it, fully adapted to the world it helped create: agents in the sea, on land, in the air.

**Regime:** statistical ecology → agent-based at fine LOD.

**Charge:**
- Oxygen-breathing, complex, mobile sea life — abundant and diverse #earth
- Mobile land life, through FLIGHT — birds, fowl, reptiles of every variety #earth
- Resilient, calibrated evolutionary pressures; energy-gradient-driven diversity mechanisms #mech

**Bequest:** the food web and its capability set — where what life CANNOT yet
do matters as much as what it can (until lignin-digesters arrive, wood
buries instead of rotting: the coal-seam window). Ecological capabilities
set the material-flux rules — decay vs burial — for everything after.
*Record: the fossil-bearing strata begin; coal seams, the signature of an
inability.*

## Phase 6 — **Aboriginal** · *the world is now **Recognizable*** — TARGET 1

Earth since shortly after the Cenozoic dawn: the Age of Mammals.

**Regime:** the substrate stabilizes beneath the agent seam — **this is the phase where the primary simulations with real agent interactions begin: the ASF's domain.**

**Charge:**
- Mammals; recognizable kingdom/taxonomy diversity across land, air, and sea #earth
- Modern flora and fungus ecosystems — flowers and fruit, grain and seed, wood that decays #earth
- Modern insect world and microbiome #earth
- Complex intelligence, communication, and agency — Pleistocene-grade life #gate

**Bequest:** the agent-legible resource world — food and forage, timber and
fiber, prey and pasture, workable stone and ore: the inventory the ASF
agents will perceive, value, and act through. *Record: recognizable modern
strata; anthropic traces only from Phase 7.*

## Phase 7 — **Prehistoric** · *the world now has **People*** — TARGET 2

Not mankind per se, but speech, symbolic thought, emotion, self-determination, higher-order agency. No creature has yet frozen thought onto matter — none has a written language that outlives the mind that thought it, that could move a thought **back out of the timestream** the first phases started.

**Regime:** agent-based — the slow ASF/LLM layer engages.

**Charge:**
- Hominids, or an equivalently-ordered agent and intelligence #earth
- Speech / language — high-enough-fidelity, adaptable, arbitrarily abstractable person↔person thought transfer #gate
- Agriculture (likely) #earth
- Non-genetic cross-generational transfer: culture; durable familial/tribal cohabitation #earth

> ⚠️ **This phase-transition is where obligation begins.** "The world has People" is
> precisely the point at which the builder acquires duties to what is inside
> it: **`ETHICS.md`'s constraints bind from here forward.** The moment an agent
> in this world is driven by the slow (LLM) layer it enters moral scope
> (doc/design/DESIGN.md §two-layer mind). This line is load-bearing and must survive every
> future revision of this file.

**Bequest:** culture itself — language, names on places, domesticated
lineages, managed landscapes, paths that become roads: inheritance that now
travels OUTSIDE the genome. *Record: symbolic artifacts, hearths, worked
tools — culture entering the strata, but not yet writing.*

## Phase 8 — **Historical** · *the world now has **Writing** and history* — TARGET 3

From here the space forks wide — real history, fantasy settings, game and story worlds — anything from the dawn of writing through science-fiction mechanics. The phase-transitions end; authored worlds begin.

**Regime:** author-driven, over the simulated substrate.

**Charge:**
- Written language — with whatever agent/intelligence substrate it requires, and most of what every prior phase built #gate
  *(Thought moves back out of the timestream; history becomes possible, and the world's record is no longer only geological but documentary.)*

**Bequest:** history — a documentary record, institutions, accumulated and
correctable knowledge; the world can now inherit INTENTIONS. *Record: the
written layer — the first record the world keeps of itself, on purpose.*

---

## Design notes

- **What a phase-transition actually names** *(the retired "checkpoint,"
  decomposed)*. Four distinct things meet at a boundary, and one word was
  hiding all of them: the **phase-gate** (the threshold itself), the **Charge**
  (the conditions *checked* there — #gate/#earth/#mech/#emergent), the **memo**
  (the converged state *cached* and carried forward — the save/cache sense),
  and the **Record** (the readable-in-rock slice that later *verifies* the
  phase ran — "what is checked" against a finished world). "Checkpoint"
  bundled the last three and collided with ASF's agent-state checkpoint;
  naming them apart removes both problems. The regime-change *event* at the
  gate is the **phase-transition**.
- **Phase-transitions are incremental, not wholesale swaps** *(Joseph,
  2026-07-04)*. A transition rarely trades one system set for another. In
  practice the next phase's active set is: **all still-necessary prior
  systems** (a carried-forward core — small at first, growing across phases)
  **+ a few systems introduced mid-phase** that keep running, **− the older
  systems no longer needed** (sunset at the gate), **+ a larger group** of
  genuinely new systems and finer-scale continuations of the intra-phase and
  older ones (usually the biggest group, spun up at the gate). So each
  transition *adds a few, removes a few, and spins up a bunch at the gate* —
  most machinery perpetuates. This is why "Law" (LEXICON) is *revised* at a
  transition, never *swapped*: the world's governing set is mostly continuous
  across gates.
- **Flat, not nested.** No Eon/Era/Period hierarchy — false precision for a
  gate-list. If any phase earns internal structure it is **Abyssal** (☆); its
  sub-gates grow when the details force them, not before.
- **Gate status ≠ sim status.** The #gate/#earth/#mech/#emergent tags say what
  *kind* of arrow a condition is, never whether it is built. Implementation
  state lives elsewhere (the §12 nomos keys, or a tracking layer) so this
  document never quietly claims the world exists.
- **Declare causally, materialize lazily** *(Joseph, resolving the moon
  fuzziness)*. Physical universes must BUILD prerequisites at their causal
  moment (the moon-forming impact resets everything, so it must come first);
  we need only DECLARE them there — parameters committed at their causal
  position in Phase 0/1 — and materialize on first downstream pull (§11
  applied to gate conditions). The moon is the archetype: its charge is one
  parameter block + a closed-form cached ephemeris (small, well-established
  math, same analytic regime as insolation), sitting there henceforth with
  its gravity, light, and cheese for whatever pulls on it. Its REAL services
  (corrected list — tidal tectonics/mantle-heat is folklore): ocean tides
  (→ intertidal ecology, Phase 5), moonlight & phases (Phase 4's sky), and
  the sleeper: AXIAL-TILT STABILIZATION — without it obliquity wanders
  chaotically over Myr and climate stability goes with it. Note: our Phase 1
  already silently assumes this service by asserting a fixed 23.44° tilt.
- **Canceling pairs — paired non-simulation** *(Joseph, the magnetic field)*.
  Some #mech conditions are a shield and its threat whose only world-visible
  product is their BALANCE: magnetic field × solar wind ("its job of keeping
  deadly solar winds from stripping your atmosphere is simulated by us also
  not simulating those winds"), Jupiter/moon shepherding × impactor flux. A
  pair stays jointly unbuilt at zero fidelity cost until a consumer queries
  the DIFFERENCE or either member alone — polarity stripes in ridge basalt
  (the unseen field still leaves readable rock — the Record
  principle), auroras, flare blackouts, an agent's compass. Counter-example
  proving the rule cuts both ways: ozone × UV was deliberately UN-canceled
  because a gate (land life) reads their difference. And when a pair is
  opened, it opens as PHYSICS, not events: auroras come from a keyed solar
  activity cycle (§8 temporal stochasticity, storm-schedule machinery at an
  ~11-year-analog scale, heavy-tailed flares) consumed as f(activity,
  magnetic latitude, night) — never "Event 7, probability 1/40/day".
- **Phases own their intrusion budget.** Each declares which effects reach
  forward past its phase-gate and which seal at it — and, dually, which of its
  parts decompose independently / in parallel / lazily-backwards-from-now (§3,
  §11). The phase-transition is the multirate coupling interface (§4) at the
  largest scale we have.

---

## Algorithms & approaches ledger *(first pass — accrete as we hit them in practice)*

*A place to park what we learn about HOW each process wants to be modelled, the
day we hit it (today: scouring/armoring). Not a plan — a growing map.*

**Approach codes** (a process may blend them): **A** analytic / closed-form /
functional (+ §8 seeded stochasticity) · **R** relaxation-to-attractor (settle,
then memo-cache) · **S** statistical / empirical (fit, lookup, sub-grid
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
| --- | --- | --- | --- | --- | --- |
| terrain prior | A (2-band fBm) | proxy | O(n·oct) | low (sets stage) | built |
| tectonic uplift | A stand-in → P (mantle) | lo | — | low-direct | crude (fBm) |
| stream-power incision (Shields τ_c) | P | hi | O(n)/epoch | **high** (the land) | built |
| Priority-Flood depressions | P/algo | hi | O(n log n) | high | built |
| MFD drainage | P | hi | O(n) | high | built |
| hillslope creep (diffusion) | P | hi | O(n) | med | built |
| glaciation · volcanism · karst · aeolian · coastal | P/S | med | varies | med (landforms) | — |

### Water & sediment (Phase 3 — today's earned detail)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
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
| --- | --- | --- | --- | --- | --- |
| insolation (declination/hour-angle) | A | hi | O(1)/query | med | built |
| climate (T/precip ← lat, elev, continentality) | S + A | med | O(n) | **high** (biomes, seasons) | **next** |
| storms / weather | A jitter → P | med | O(n) | **high** (agents feel it) | crude (jittered) |
| orographic rain, rain shadow | P/S | med | O(n) | high | planned |

### Materials & record (spans all phases)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| material taxonomy (undifferentiated → refined) | T + A (det. refine) | — | O(1) | **high** (what things are) | schema |
| strata / column | data structure | — | — | high (digging, cliffs) | frame built |
| chronostratigraphic Record = the verification artifact | (emerges) | — | — | high (readable history) | conceptual |

### Biosphere (Phases 3→6; final-3-critical) — mostly import + statistics

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| biomes (Whittaker T×P) | T + S lookup | — | O(1)/cell | **high** | — |
| vegetation / succession | S + T | med | O(n) | **high** (food, cover) | — |
| species / fauna | **T (bestiary import)** | — | curated | **high** | — |
| evolution / speciation | S / procedural | lo | — | low-med (flavour) | defer |

### Agents — the real bet (Phases 6→8)

| process | approach | physics | cost | agent-fidelity | status |
| --- | --- | --- | --- | --- | --- |
| fast ASF/AAT layer (formal dynamics) | P | hi (our theory) | O(agents)/tick | **the point** | the bet |
| slow layer (LLM at aporia) | import (the model) | — | event-driven | **the point** | the bet |
| cognitive LOD swap | A (component swap) | — | ~free | high | designed |

*Note on complexity: the per-cell O(n) stencils are the cheap common case —
memory-bound, and the GPU/rayon path (`doc/plan/water-parallelism.md`) is
what keeps them affordable as fidelity and area grow. The expensive-but-rare
ones (Priority-Flood O(n log n), any global solve) run once per phase-transition
and cache. The genuinely open-complexity items are the ones marked lo physics
(tectonics, evolution) — where we deliberately run a cheap statistical stand-in
because agent-fidelity does not yet demand more.*
