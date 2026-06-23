# Geological & world-modeling research — synthesis

> Status: synthesis of a six-sweep research session, 2026-06-22. Goal: a
> defensible, emergent-where-possible, statistically-plausible-where-not world
> engine for vivarium. Sources tracked in `relata`; PDFs under `pdfs/`.
> **Epistemic discipline:** every source carries a verification flag. "metadata
> verified" means a research agent confirmed title/authors/venue/year/DOI
> against the publisher/HAL/arXiv page — it does **not** mean the full text or
> the determinism/cost *claims* were independently audited. Where a claim rests
> on a blocked or undecoded PDF, it is flagged UNVERIFIED. Several recommended
> couplings (ocean-gyre proxy, conservative refinement) are *our* design
> syntheses, not techniques found shipping — flagged DESIGN-HYPOTHESIS.

---

## 0. The headline: six independent sweeps converged on one pipeline

Five research agents (tectonics, erosion, climate, voxel-reduction, PCG-since-
Perlin) plus a deeper tectonics re-run, run without sight of each other,
triangulated onto the *same* deterministic spine and the *same* architectural
conclusion. That convergence is the strongest evidence in this document — it is
not one agent's preference, it is six arriving separately.

**The pipeline (world-creation tier, run slowly, once):**

```
  plate tectonics            erosion                climate              materialize
  (Euler-pole motion   →   (stream-power law,  →  (wind-march      →   (sample fields into
   on a hex overlay)        Braun-Willett O(n))    orographic rain)     cube-sphere voxels)
        │                        │                      │                     │
   uplift/pressure field    drainage-carved        temperature,          per-column
                            ridgelines + rivers     precip, biomes        strata function
```

Every stage is **seed/input-deterministic** and O(N) or O(N log N). The whole
thing fits vivarium's `voxel(x,y,z) + sparse edits` core cleanly (see §6).

---

## 1. Framing decision (PROPOSED 2026-06-22 — ratify or reject before building)

**The planet is the abstraction tier.** A sphere is simulated slowly at
world-creation (plates → uplift → erosion → climate), emitting global fields
that the local Cartesian `seed + edits` voxel world is materialized from as a
tangent patch. The voxel substrate never becomes spherical; the awkward sphere
topology is quarantined in the slow tier where it costs nothing. This is
DESIGN.md's abstract→detail invariant with **geology as the abstraction**, and
it matches Joseph's intuition that world-creation can take far longer than ticks.

The research *confirms* this shape rather than challenging it: every shipped
voxel system (Minecraft density-functions, DF column descriptors, Vintage Story
strata tables) stores geology as a cheap per-coordinate function, never as
materialized voxels.

**Concrete grid recommendation (from the tectonics sweep):**
- **Voxel storage/render: cube-sphere quadtree** — the only sphere grid where
  each of 6 faces is natively locally-Cartesian `(i,j)`; trivial in-face
  neighbors, quadtree LOD mapping directly onto chunked voxel streaming,
  singularities confined to 12 edges + 8 corners (far milder than lat-lon
  poles). Proven at planet scale in NOAA's FV3 weather model.
- **Tectonic/climate/agent overlay: Goldberg-hex (or spherical Voronoi) graph**
  over the fine substrate — clean 6-neighbor boundary-stress propagation
  (Gainey/Red Blob-proven). This is the two-layer fine-world/coarse-overlay
  split DESIGN.md already gestures at.

---

## 2. The granularity question — now has a real number

Joseph's core open question ("how granular must the intrinsic voxels be for the
right emergent complexity?") **partly dissolves**, and the part that remains has
a rigorous anchor.

**The dissolution:** there are *two* resolutions, routinely conflated:
- the **simulation grid** the geology runs on (where emergence is *born*), and
- the **materialized voxel** the player walks on (where *appearance* happens).

Emergent drainage is a property of the *sim grid*, not the render voxel.

**The anchor (rigorous):** a 2023 landscape-evolution-model resolution-
sensitivity study (CAESAR-Lisflood, *Earth Surface Dynamics* 11:695):
- **First-order drainage detail is lost beyond ~22–24 m** cell size.
- **~12 m is the recommended threshold**; finer gives diminishing returns.
- **Critical trap:** bulk sediment budgets stay stable while the network
  *texture* collapses — "the totals match" is **not** "the emergence survived."

**Conclusion (single most actionable architectural finding):** *decouple the
simulation grid (~10–12 m class for drainage emergence) from the materialized
voxel size (sub-meter, appearance).* Different budgets, different questions.
Subsurface features (caves, ore veins, ~sub-meter) likely demand a *finer* sim
grid than drainage — another argument for **per-process** sim grids rather than
one global voxel size. (Voxel translation of the 12 m number: medium confidence,
flagged as interpolation. Slope-stability minimum resolution: no rigorous number
found — genuinely open.)

**Caution (verified):** a regular voxel grid with naive single-flow-direction
routing imprints **grid-anisotropy artifacts** (channels biased to axis/diagonal
directions) that masquerade as emergence. Use multiple-flow-direction / D∞
routing. (PNAS arXiv:1911.03519; arXiv:1812.03696.)

---

## 3. Erosion + fluvial geomorphology — the first-feature spine

First target = **mountains pushed up, then eroded into recognizable forms.** The
canonical body of work is the Lyon/Grenoble (Galin–Guérin–Cordonnier) graphics
group, which ported geomorphology's stream-power law into terrain tools.

| Source | What it gives | Determinism | Voxel fit | Verify |
|---|---|---|---|---|
| **Braun & Willett 2013** (Geomorphology 180–181:170–179) | O(n) implicit stream-power solver ("FastScape"); integer flow-tree, fixed traversal → **best determinism story found** | ★★★★★ | heightfield | metadata verified |
| **Cordonnier 2016** (CGF 35(2):165–175) | uplift map → stream graph → stream-power → emergent dendritic drainage + ridgelines | ★★★★ (input-driven) | heightfield | FULLY verified |
| **Schott 2023** (ACM TOG 42(5)) | interactive stream-power authoring; **public C++/GLSL code** (github.com/H-Schott/StreamPowerErosion) | ★★★★ | heightfield | metadata verified |
| **Whipple & Tucker 1999** (JGR 104(B8)) | the stream-power law itself: `E = K·Aᵐ·Sⁿ` (m≈0.5, n≈1); the geomorphology foundation | n/a (theory) | — | metadata verified |
| **Musgrave 1989** (SIGGRAPH CG 23(3):41–50) | thermal/talus erosion (angle of repose ~30–40°); **voxel-native** ("voxels above repose slump") | ★★★★★ | **voxel-native** | metadata verified |
| **Cordonnier 2017** (ACM TOG SIGGRAPH) | erosion ⊗ ecosystem; **models material layers** (rock/sand/humus) — bridge toward strata | ★★★★ | layered heightstack | metadata verified |
| **Cordonnier 2023 glacial** (ACM TOG 42(4)) | U-valleys, fjords, hanging valleys — but uses a **learned ice-flow surrogate** (determinism risk) | ★★ (DL) | heightfield | metadata verified |
| **Mei 2007** (Pacific Graphics) | virtual-pipe / shallow-water GPU hydraulic erosion; real fluid, RNG-free but **GPU float-reduction nondeterminism** | ★★ (GPU) | heightfield+water | metadata verified |
| **Beyer 2015** (BSc thesis, TU München) | the droplet/particle hydraulic erosion algorithm Sebastian Lague & many repos implement | ★★ (RNG) | heightfield | metadata verified |
| **Galin 2019** (CGF 38(2) STAR) | **the Eurographics State-of-the-Art review — start here for the full citation web** | — | — | metadata verified |

**Determinism ranking (best→riskiest):** thermal/talus → stream-power
(Braun-Willett) → virtual-pipe shallow-water → droplet (RNG) → glacial (learned).

**Honest gap:** none of the verified work is *truly volumetric* — all heightfield
or layered height-stacks. **Full-volume voxel erosion with strata is largely open
research.** We are not behind here; this is frontier.

---

## 4. Climate / atmosphere / hydrology — emergent, not painted

The "climate, biomes, rivers fall out of elevation + latitude + wind" property
is achievable as a one-directional pipeline (optionally closed into a slow
erosion feedback loop). All stages deterministic on integer/seeded arithmetic.

1. **Temperature** ← `cos(lat)` baseline − lapse rate (6.5 °C/km)·elevation
   (+ seasonal/continentality term). *Azgaar 2017.*
2. **Prevailing winds** ← fixed three-cell latitude-band lookup (trade /
   westerly / polar-easterly). *Hadley-cell climatology.*
3. **Precipitation** ← wind-march moisture packets, enhanced by
   **`dot(wind, ∇elevation)`** for orographic lift + leeward rain shadow.
   *DF / Experilous / Roe upslope model.* — **highest-leverage single technique:
   ~50 lines, O(N), deterministic; puts deserts/forests/river-density in the
   right places for the right reasons, which cascades correctly into biomes &
   river weighting.** The `dot(wind, ∇elev)` detail (not raw elevation) is what
   separates plausible from cartoonish.
4. **Biomes** ← Whittaker 2-D lookup (temperature × precipitation); upgrade to
   Köppen when monthly fields exist. *PCG wiki / Azgaar (100×10 → 22 biomes).*
5. **Hydrology** ← Priority-Flood depression fill → D8 (fixed tie-breaks) → flow
   accumulation **weighted by precipitation** → threshold → rivers; recover
   raised cells as lakes; label watersheds. *Barnes et al. 2014.*
6. **(Optional slow loop)** stream-power erosion `∂h/∂t = U − E` via
   Braun-Willett, alternating epochs with 1–5 → the geology↔climate feedback
   (uplift → rain shadow → concentrated erosion → elevation change → shifted
   rain shadow). *The real "statistically-plausible, 3D-all-the-way-down" payoff.*

| Source | Role | Verify |
|---|---|---|
| **Barnes et al. 2014** (Computers & Geosciences 62:117–127) | Priority-Flood depression fill + watershed labeling | metadata verified (pages reconstructed from search) |
| **Azgaar 2017** (blog) | cheap temperature + biome model, widely copied | fetched/verified |
| **Whittaker 1975** (*Communities and Ecosystems*) | temp×precip → biome diagram | secondary-verified |
| Minder & Roe (orographic precip encyclopedia) | rigorous upslope-model backing | UNVERIFIED (PDF not decoded) |
| Red Blob Games 1843 | rivers-on-a-sphere reference impl | fetched/verified |

**DESIGN-HYPOTHESIS (not shipped recipes — spike candidates):** the ocean-gyre
SST proxy and the exact seasonality formula are our syntheses from earth-science
shape, not techniques found in a shipping game.

**Biggest replay-determinism landmines:** (a) flat-region tie-breaking in
D8/Priority-Flood — flats are where nondeterminism sneaks in; fix tie-breaks by
cell index. (b) any GPU/parallel float-reduction erosion. Stay on the integer
flow-tree and seeded replay holds.

---

## 5. Tectonics + spherical planet generation

Memory check: "Procedural Tectonic Planets" is real — lead author **Yann
Cortial** (Galin is co-author). "platec" author is **Lauri Viitanen** (not
"Hirvonen"). "David Mortenson plate generator" — not found, likely a conflation
with tectonics.js. No GDC plate-tectonics talk found.

**Framing theorem:** you cannot tile a sphere with one globally-regular grid
(Euler). Every method just *chooses where the unavoidable distortion goes*
(hex → 12 pentagons; cube-sphere → 8 corners; lat-lon → 2 poles).

| Source | Tier | What it gives | Verify |
|---|---|---|---|
| **Cortial 2019** Procedural Tectonic Planets (CGF 38(2), DOI 10.1111/cgf.13614) | rigorous graphics | sphere-native plates → continents, ridges, **mountain ranges**, island arcs | metadata verified (bit-replay UNVERIFIED, PDF blocked) |
| **Cordonnier 2016** (see §3) | rigorous graphics | the erosion stage downstream of any plate sim — feed collision pressure in as uplift map | FULLY verified |
| **platec / Viitanen 2012** (github Mindwerks/plate-tectonics) | game-grade | continent-collision folds mountain belts; **explicitly seeded** → clearest determinism guarantee in its tier | author verified |
| **Gainey / Experilous 2014** | game-grade | icosphere + Goldberg hex dual; flood-fill plates; boundary-stress mountains | verified |
| **Red Blob 1843** (Amit Patel) | game-grade | Delaunay/Voronoi sphere; convergent boundaries raise mountains | verified |

**Sphere discretization real-world users:** icosphere → ICON, NICAM, Uber H3;
cube-sphere → **GFDL FV3 (operational NOAA GFS)**, MITgcm; spherical CVT → MPAS;
HEALPix → astronomy (Górski 2005, *ApJ* 622:759) but **no game precedent**.

**Rigorous earth-science sims (for calibration, likely overkill to run):**
ASPECT (Kronbichler 2012, *GJI* 191 — mantle-convection FEM PDE); GPlates
(plate *reconstruction*, Euler-pole rigid bodies); goSPL/Badlands (Salles 2020,
*JOSS* 5:2804 — global landscape evolution on a spherical mesh).

**THE single highest-leverage upgrade — Euler-pole plate motion.** Rigorous
models treat each plate as a rigid body rotating about an **Euler pole**:
velocity is **ω × r** — varies across the plate, *zero at the pole*. Cheap game
sims assign **one flat drift vector per plate**, which is geophysically wrong
globally. Assigning each plate an Euler pole + angular rate instead is **cheap,
deterministic, and correct-by-construction** — the best credibility-per-effort
move short of full mantle PDEs. Exactly Joseph's "defensible and non-arbitrary"
bar: the more-correct model is barely more expensive than the wrong one.

---

## 6. PCG since Perlin — the substrate underneath everything

**Key principle:** determinism comes from *implementation* (integer hashing,
fixed-point or order-fixed float ops, explicit seeded RNG), not from a
technique's identity. Danger zones: (a) parallel/unordered float reductions, and
(b) anything learned.

**A clean architectural fault line (load-bearing for vivarium):** the *noise*
lineage preserves `voxel(x,y,z)` as a **pure O(1) random-access function**; the
*beyond-noise* lineage (WFC/Model-Synthesis, grammars, example-based, agent-sim)
**trades O(1) access away** — those are neighbor-dependent *generation passes*.
Resolution that fits the core exactly: **noise = the pure function; simulation
passes = what writes the sparse edit overlay.** Not a compromise — it is the
`seed + edits` split doing what it is for.

- **Base spine:** improved-Perlin (2002) or **OpenSimplex2** FBM + **domain
  warping** (Quilez) + **analytic derivatives** (free normals & slope for
  erosion-coupling). Pure hash-lattice → bit-identical, O(1).
- **Structured local features:** **Worley/cellular** (caves via F2−F1, ore
  pockets, biome Voronoi); **L-systems/grammars** for vegetation/structures
  stamped by hashed location.
- **Spectral control for the fidelity invariant:** **Gabor noise** (Lagae 2009,
  TOG 28(3)) + **wavelet noise** (Cook & DeRose 2005, TOG 24(3)) are the one
  family that lets you *specify a power spectrum* and anti-alias across LOD — if
  "statistically consistent with the abstraction" means "match a target
  spectrum," this is the technique that speaks its language. (DESIGN-HYPOTHESIS
  on the exact application.)
- **Coherent large structure:** **WFC = Model Synthesis** — credit **Paul
  Merrell 2007** (i3D; TVCG 2011) as origin, Gumin 2016 as popularizer. Run
  per-chunk seeded by `hash(seed, chunkCoord)` with constrained borders.
- **Divergence-free flow fields:** **curl noise** (Bridson 2007) — deterministic
  wind/current/agent-flow fields; flagged for the *agent* layer too.
- **Deepest fit for vivarium's values — agent/simulation-based world history
  (DF-lineage).** Natively the "bit-identical from a seed, no clock, no thread
  randomness" model; produces the most causally-explainable worlds ("this canyon
  exists because this river ran here N steps"); **and shares substrate with the
  ASF agent bet** — worldgen-agents and sim-agents are the same kind of thing.
  Likely the highest-value direction beyond plain noise.
- **Ruled OUT for the deterministic core:** learned models (GAN/diffusion). The
  2025 diffusion "Perlin successor" (arXiv:2512.08309) *itself concedes*
  seed-consistency is only tolerance-bounded under float non-associativity and
  **breaks across hardware** — disqualifying for bit-identical replay. Fine as
  an offline authoring aid baked to static assets; not in the seeded runtime.

---

## 7. Two genuine research gaps (open everywhere we looked — vivarium-shaped)

1. **Conservative / statistical refinement.** DESIGN.md's shared-fidelity
   invariant ("any materialization must be statistically consistent with the
   abstraction it replaces") is, formally, a **multigrid/wavelet conservation
   property** — fine voxels in a coarse cell must sum back to the coarse cell's
   aggregate (mean elevation, material fractions, water volume). Standard in
   numerical PDEs; **absent from every voxel game found.** This is plausibly
   novel territory and maps directly onto the AAT fidelity-invariant concern.
2. **Detail→abstract edit propagation** (already flagged in DESIGN.md/voxel.rs):
   when an agent reshapes a high-fidelity locus, the abstract model must absorb
   it. No prior art found. Both gaps are the *fun* kind of hard.

---

## 8. Proposed first spike (for discussion — not yet decided)

A minimal, end-to-end, deterministic **uplift→erode** slice on a flat heightfield
patch (defer the sphere; it's the abstraction tier and can wait):

1. Coarse uplift field (hand-painted or a few Euler-pole-ish blobs) at ~10 m sim
   grid.
2. **Braun-Willett stream-power** erosion to steady-ish state (integer flow-tree,
   fixed traversal — the determinism-clean choice).
3. **Thermal/talus** slope cap — implemented **natively in voxels** (its most
   natural home).
4. Sample the eroded heightfield into the existing `voxel(x,y,z)` core; assert
   bit-identical replay.
5. Look at it. Is the drainage dendritic? Do ridgelines emerge? Does it survive
   the "texture not just budget" test from §2?

This exercises the determinism invariant, the two-resolutions split, and the
mountain-first target in one coherent step — and produces something to *look at*,
which DESIGN.md's recreational disposition wants.

---

## 8a. Artifacts on disk + implementation-readiness (2026-06-22)

Pulled toward the §8 spike. Both gitignored (`ref/.gitignore`) — read locally,
not vendored into vivarium history.

- **`ref/geology/StreamPowerErosion/`** — Schott 2023's reference implementation
  (cloned; `.git` removed). **This largely closes the implementation gap.** The
  whole uplift→erode loop is in `data/shaders/spe_shader.glsl` (~200 lines,
  readable): steepest-descent D8 flow (`GetFlowSteepest`), stream-area
  accumulation, stream-power erosion `k·A^p_sa·slope^p_sl` (defaults
  p_sa=0.8, p_sl=2, k=5e-4) applied **explicitly** with `dt`, plus toggleable
  hillslope (Laplacian) and debris/talus-slope modes.
- **Key distinction discovered by reading the code:** Schott's method is an
  **explicit per-cell GPU scheme**, *not* Braun & Willett's implicit O(n) stack.
  That means it is precisely the **GPU-float-reduction determinism risk** flagged
  in §3/§4. For vivarium's bit-identical core we want the **implicit stack**
  variant (same stream-power physics, deterministic-by-construction) — so Schott
  = the readable worked shape; **Braun & Willett 2013 = the determinism-clean
  solver we still need in hand.** The two are complementary, not redundant.
- **`ref/geology/pdfs/`** — intended home for PDFs. Empty: see §9 fetch status.

## 9. Source ledger → relata

Spine seeded into `relata` this session (verified critical-path): galin-2019,
cordonnier-2016, braun-2013, whipple-1999, cortial-2019, schott-2023,
musgrave-1989, barnes-2014, perlin-2002, merrell-2007.

Follow-up batch (metadata in the tables above; add when convenient):
Cordonnier 2017 & 2023-glacial, Mei 2007, Beyer 2015, Lagae 2009, Cook 2005,
Bridson 2007, Summerville 2018 (PCGML), Aokana 2025 (arXiv:2505.02017),
**CAESAR-Lisflood ESurf 11:695 2023** (granularity anchor — needs author lookup
before adding), HEALPix/Górski 2005, goSPL/Salles 2020, ASPECT/Kronbichler 2012,
Whittaker 1975, Viitanen 2012, Gainey 2014, Patel/Red Blob 1843.
</content>
