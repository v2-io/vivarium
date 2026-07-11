# Flament, Coltice & Rey (2008) — primary-source read

*Flament, N., Coltice, N., Rey, P.F. (2008). "A case for late-Archaean
continental emergence from thermal evolution models and hypsometry." Earth
and Planetary Science Letters 275, 326–336. doi:10.1016/j.epsl.2008.08.029.*

PDF: `ref/research/pdfs/flament2008.pdf` (11 pp). Read page-by-page (figures +
Table 1 + Table 2) on 2026-07-10, upgrading this source from **[A]** (abstract/
secondary) to **[V]** (read off the page) for the survey.

> **Tiering used below**
> - **[read]** — a number/word taken verbatim from text, Table 1, or Table 2.
> - **[fig≈]** — read off a figure by eye; approximate, uncertainty noted.
> - **[inf]** — my inference/synthesis, not stated as such by the authors.

---

## Headline

**Yes — the 2–3% survives a real read, and it is if anything slightly
tightened.** The figure is not a loose secondary paraphrase; it is the paper's
central deterministic result and its Monte-Carlo median:

- Deterministic (reduced Archaean mountain-building, h_max ≈ 3615 m, 80% of
  present continental area, T_p = 1430 °C): emerged land = **3.1% of Earth's
  area** for max seafloor age t_max = 180 Ma, **1.8%** for t_max = 104 Ma
  [read, §4.3].
- Monte Carlo (10⁵ runs, t_max = 104 Ma): **medians of emerged area A_f range
  1.79–2.84% of Earth's surface** [read, §4.4].
- The **<12%** number is a *different, weaker* claim: it is the ceiling under
  **constant (modern) hypsometry** — i.e. if you keep modern-height mountains.
  Fig 3's Archaean box spans **1–12%** [read/fig≈, §4.1].

**What drives the 2–3% vs 12% spread:** the **maximum continental elevation
h_max** (equivalently, mountain-building efficiency / continental-lithosphere
strength). Drop h_max from modern (~9 km above shelf) to the Archaean
anorogenic value (~3.6 km) and the emerged area collapses from ≤12% to 2–3%.
That is the whole argument. Mantle temperature and continental-area growth are
secondary levers; the Monte-Carlo says **continental crustal thickness** is the
single largest *uncertainty* source (not the central driver).

**One real correction for our survey:** Flament's Archaean ocean basins are
**shallower than modern, not deeper** (constant ocean volume + thick buoyant
Archaean oceanic crust raises the seafloor). The "deeper early oceans ~5–6 km"
target in survey §5/§6 is a **Korenaga/Dong (ocean-volume) claim and must not
be anchored to Flament** — Flament's flooding mechanism is the *opposite* sign
on basin depth. Details in "Survey corrections" below.

---

## 1. What the model is, and what it holds fixed

The model computes **emerged continental crust area as a function of three
inputs**: mantle potential temperature T_p, continental area A_cc, and
hypsometry (the elevation-vs-area shape) [read, abstract/§1]. It is an
**isostasy + ocean-volume-budget** model, not a dynamic simulation.

**Held constant (the load-bearing assumptions):**
- **Ocean volume constant** through time: V_o = 1.36×10¹⁸ m³ (±2×10¹⁷)
  [read, Table 1]. Explicitly flagged as a limitation: *"there is a poor
  control of ocean volume through time"* [read, Conclusions].
- **Continental crustal thickness constant** over time: Archaean d_cc = 41.1 km
  (±7), identical to present d*_cc = 41.1 km (±6.2) [read, Table 1]. They do
  **not** assume a thinner or thicker Archaean crust in the baseline; the Monte
  Carlo tests sensitivity to it. *"the thickness of the continental crust is
  the key assumption"* [read, Conclusions].
- **Isostatic equilibrium** between continents and oceans [read, §2.1]. Sea-
  level change (Eq 1):
  Δh_f = Δd_r(1−ρ_w/ρ_m) + Δd_oc(1−ρ_oc/ρ_m) − Δd_cc(1−ρ_cc/ρ_m).

**Varied across the Archaean scenario:**
- Mantle **150–200 °C hotter** than present in the Archaean [read, §4.1; from
  Abbott et al. 1994's 137–187 °C at 2.8 Ga]. Baseline late-Archaean run uses
  T_p = 1430 °C (present T*_p = 1280 °C) [read, Table 1 / Fig 6].
- Continental area **grew from ~20% of present at ~4 Ga to 80% by 2.5 Ga**
  (following Campbell 2003) [read, §4.1]. 80% of present 42.5% ⇒ **~34% of
  Earth's area** at 2.5 Ga [inf/fig≈, Fig 6 step at ~34%].
- **Three thermal-evolution models** for how bathymetry flattens with T_p:
  **BLT** (boundary-layer theory), **K06** (Korenaga 2006), **LJ07** (Labrosse
  & Jaupart 2007, seafloor-age distribution). BLT is most T_p-sensitive; K06 &
  LJ07 are "buffered" (flatter response) [read, §3.1]. The flooding result uses
  **LJ07**.

**Key densities / thicknesses (Table 1) — isostasy inputs [read]:**
ρ_cc = 2800±100, ρ_oc = 3000±100, ρ_m = 3300±100, ρ_w = 1030±10 kg/m³.
Oceanic crust: present d*_oc = 7075±700 m; **Archaean (T_p=1430°C) d_oc =
20,870±2500 m (~21 km)**. Ridge depth d*_r = 2446 m; avg depth below ridges
d*_b = 1859 m. Present continents = **42.5%** of Earth's area; present emerged
crust = **27.5%** [read, §1/§3.2].

---

## 2. The hypsometric shape they ASSUME (a world-builder input in itself)

This is the part most directly reusable as a nomos prior. The continental
hypsometric curve is **prescribed analytically**, fit to eTOPO2 with
**R² = 0.999** [read, §3.2.2], Eq (5):

```
          b · h_max · ( (1 − A/A_sh) / (a + A/A_sh) )^z      for A ≤ A_sh   (emerged + shelf)
f(A) =
          ( (A − A_sh) / (A_cc − A_sh) ) · d_max              for A >  A_sh   (continental slope: linear)
```

with **b = 0.608**, **z = 0.706** [read, Table 1], A_sh = area out to the
shelf edge, A_cc = total continental area, d_max = **2446 m** (max depth of the
continental slope) [read, Table 1], h_max = maximum continental elevation
(the free knob), a = fit constant. Above the shelf the curve is a
Strahler(1952)/Harrison(1981) power-law hump; the shelf-to-slope is **linear**
down to −2446 m [read, §3.2.2].

**The shape is scaled by h_max and bounded by two empirical end-members
(Fig 4, normalized area):**
- **Orogenic end-member = present-day Asia** (steep, tall — built by the last
  50 Myr of collision) [read, §4.2].
- **Anorogenic end-member = present-day Australia** (subdued) [read, §4.2].
- Present *global* hypsometry sits very close to the **orogenic (Asia)**
  member; the Jurassic/Cretaceous flooded world sat closer to **anorogenic
  (Australia)** [read, §4.2].
- **The proposed Neoarchaean orogenic end-member is "very close to the
  present-day anorogenic end-member"** [read, §4.3] — i.e. the *most*
  mountainous Archaean continents look like *flat modern Australia*.

**World-builder read:** the Archaean continental elevation distribution =
Australia-like normalized shape (Eq 5, b=0.608, z=0.706), **vertically scaled
to h_max ≈ 3600 m**, riding **below** sea level for all but a few percent of
its area.

---

## 3. Emerged-area curves — numbers off the figures

**Fig 3** (emerged crust %-of-Earth vs continental area × mantle T_p, *constant
hypsometry*, four models). The Archaean box (T_p 1430–1480 °C ≡ 150–200 °C hot;
continental area 20–80% of present) intersects contours from **~2 up to ~12%**
[fig≈]. This is the source of the "< 12%" ceiling. Contour spacing labeled
2, 6, 8, 10, 12, 14, 16, 18, 20 [fig≈].

**Fig 5** (emerged land % vs maximum elevation h_max; present continental area
x=1). Plain curve (present T_p): ~28% emerged at h_max=10 km, falling to ~0 as
h_max→~500 m [fig≈]. Dashed curves (T_p+150 K): at h_max ≈ 10 km the LJ07/K06
band tops out ~10–12%, BLT ~7.5%; **as h_max drops toward ~3600 m the T_p+150K
emerged area falls to ~2–3%** [fig≈]. This figure *is* the h_max→emergence
lever visualized.

**Fig 6** (the late-Archaean world; LJ07, x=0.8, T_p=1430 °C, h_max=3615 m) —
the single most world-builder-relevant figure. Calculated hypsometry+bathymetry
for late-Archaean (dark) vs present (light):
- Continental platform (cumulative area 0→~34%): sits **at/just below sea
  level**; only a thin sliver pokes above 0 [fig≈] = the ~2–3% emerged.
- Deterministic: **t_max=180 Ma ⇒ 3.1%** emerged; **t_max=104 Ma ⇒ 1.8%**
  [read, §4.3].
- Flooded continental area: **~24% (180 Ma) / ~25.5% (104 Ma) of Earth's area
  is under water ~800 m (~1100 m) deep** — *maximum* depths, sedimentation not
  modeled [read, §4.3]. (This "~800–1100 m" is water over drowned continental
  platform, **not** the abyssal ocean.)
- **Emerged fraction of the continents ≈ 10%** at 2.5 Ga [read, §4.5] — i.e.
  ~3% of Earth ÷ ~34% continental area ≈ 10% of continents emergent; matches
  the ~10% of 2.75–2.5 Ga LIPs that are subaerial (Kump & Barley 2007) [read].

**Table 2 (Monte Carlo, 10⁵ runs, t_max=104 Ma) [read]:** three outcomes —
(a) entirely emerged, (b) partially flooded, (c) entirely flooded.
- **Partial flooding = 64%** of realisations (the most probable); consistent
  with a continental-thickness change of **~2.1 ± 6.1 km** vs present.
- **Entirely flooded = 9%** of trials; needs crust **~10 km thinner** than
  present.
- **Entirely emerged = 27%** of trials; needs crust **~10.2 ± 6.1 km thicker**
  and ocean volume **~10% lower**.
- Distributions positively skewed; std devs up to 100% of mean for A_f, 50%
  for sea level h_f. **A_f medians 1.79–2.84%** of Earth's surface.

---

## 4. Freeboard, sea level, and *why* sea level is high

**Continental freeboard** = mean elevation of emerged continent above sea level
[read, §1]. Its **constancy to ±200 m** (the Phanerozoic observation, extended
to Precambrian by many) is the paper's test object. Their result: constancy
±200 m is achievable **only if the upper mantle potential temperature was never
more than 110–210 °C hotter than present** [read, abstract/Conclusions]. Since
the Archaean mantle was 150–200 °C hotter (at the top of that band), freeboard
in the Archaean is pushed **low / near-zero / slightly negative** → flooding.

**Table 1 freeboard-related parameters [read]:** h*_f (sea level above the
shelf edge) = 200 m; h*_max (max elevation above continental shelf) = 9050 m;
f* = 8850 m (= 9050 − 200, the modern peak-above-sea-level span). *(Notation
note: the paper's f* in Fig 1/Table 1 is this elevation span, distinct from the
"freeboard constancy ±200 m" mean-elevation quantity discussed in prose. Flagged
because the same letter carries two senses.)*

**Why sea level rises (→ flooding) in a hot mantle [read, §3.1]:** two
processes, both raising the seafloor and displacing water onto the continents —
1. **Thickening of buoyant oceanic crust** (hotter mantle → more melt → thicker,
   lighter oceanic crust → floats higher).
2. **Flattening of the seafloor** (younger/hotter ocean floor is shallower).
Plus: **producing new continental crust reduces ocean area**, raising sea level;
*"Removing the continental crust entirely would result in a decrease in sea
level of ~1000 m"* [read, §3.1] (continents displace ~1 km of eustatic sea
level). Net: hot Archaean ⇒ high sea level relative to low-relief continents ⇒
drowned platforms, ~2–3% emergent.

---

## 5. Relief / topography amplitude

- **Neoarchaean orogenic plateaus ~1800–2200 m** (Rey & Coltice 2008,
  conservative present-day strain rates 10⁻¹⁵–10⁻¹⁴ s⁻¹), **allowing a
  dynamically supported maximum elevation up to ~3600 m** [read, §4.3]. This
  ~3600 m is the h_max = 3615 m used in Fig 6.
- Mechanism for the cap: **greater radiogenic crustal heat + higher mantle heat
  flow → weaker continental lithosphere → less efficient mountain building /
  lower sustainable topography** [read, abstract/§4.3].
- The authors explicitly note this **reduced max elevation is "at odds with
  some previous works"** (England & Bickle 1984, uniformitarian 70-km orogenic
  crust); they rebut that high-grade gneiss pressures don't require 70-km
  Archaean crust [read, §4.3]. So the low-relief assumption is *argued*, not
  assumed away.
- Emerged land at max relief: *"reduced area of emerged continents (up to 3% of
  the Earth's area), which is roughly the size of South America, and maximum
  elevations (approximately 3600 m)"* [read, §4.5].

**Cross-check vs survey §3 ("max Archaean relief ~3–5 km"):** Flament's
h_max ~3.6 km sits at the **low end** of the survey's 3–5 km band and is
consistent with it. Note the definitions differ slightly — Flament's 3615 m is
*max continental elevation above sea level (peak)*, not peak-to-trough relief.

---

## 6. Ocean depth — the one place Flament contradicts the survey's rendering

The survey (§0, §5, §6) says early oceans were **deeper** (~5–6 km, ~50%+ more
water) than modern, and lists Flament among the anchors for a "mostly-drowned,
low-freeboard" world. The *drowned/low-freeboard* part is right and Flament
supports it. But **the mechanism and the sign on ocean depth are opposite**:

- Flament **holds ocean volume constant** and floods continents by **raising
  the seafloor** (thick buoyant Archaean oceanic crust d_oc ~21 km + flattening)
  [read, §3.1]. In **Fig 6 the late-Archaean abyssal floor is ~2 km *shallower*
  than present** (dark curve bottoms ~−4000 m at 100% area; present ~−6000 m)
  [fig≈]. Water that would have filled a deeper basin instead sits on the
  continents.
- The survey's **deeper-ocean claim is Korenaga (2021) / Dong (2021)** — a
  *water-volume* (mantle degassing) argument, an entirely different mechanism.

So the two anchor papers **disagree on ocean-basin depth direction**: Flament =
shallower basins (constant volume), Korenaga/Dong = deeper basins (more water).
Both produce flooding; the survey correctly calls them "not mutually exclusive"
(§1) but should not let readers infer that *Flament* supports deep oceans. It
does not — for a world-builder, **"deep basins" must be sourced to Korenaga/
Dong; "flooded low continents + thick oceanic crust" to Flament.**

Also: Flament's Archaean oceanic-crust thickness (~21 km at T_p=1430 °C) is
**lower than the survey's "25–35 km"** figure (§2/§6, from the Science Advances
melting model). Same ballpark, model-dependent; survey §7 already flags 25–35 km
as uncertain — Flament's ~21 km is a data point at the low end.

---

## 7. Survey corrections (what §6/§7 should change)

**§6 target table:**
- **Emergent land fraction row** — the "Flament floor ~2–3%" is now **[V]-
  verified**, and can be stated more precisely: deterministic **1.8% (t_max
  104 Ma) – 3.1% (t_max 180 Ma)**; Monte-Carlo **median 1.79–2.84%**; constant-
  (modern-)hypsometry ceiling **≤12%** (Fig 3). The overall "2–15%" band is
  fine; just retag the Flament floor to [V] and note the 1.8–3.1 refinement.
- **Continental freeboard row** — Flament supports "low/near-zero/slightly
  negative" and quantifies the gate: **±200 m constancy only if mantle never
  >110–210 °C hotter than present** [V]. Can upgrade confidence and add the
  threshold.
- **Mean ocean depth row** — **do not cite Flament here.** Flament's basins are
  *shallower* than modern (constant volume). Keep this row anchored to
  Korenaga/Dong only, and add a one-line note that Flament floods via seafloor
  *rise*, not ocean deepening. This is the substantive fix.
- **Oceanic crust thickness row** — add Flament's ~21 km (T_p=1430 °C) as a
  low-end data point vs the 25–35 km entry.
- **Continental crust thickness row** — Flament's baseline is **41.1 km,
  *equal* to present** (they do not thin the Archaean crust); the Monte Carlo
  says continents partially flood for a thickness change of only +2.1±6.1 km.
  Worth stating: emergence is gated by *relief/freeboard*, not by assuming
  thinner crust.
- **Hypsometry-shape row** — Flament gives a concrete shape: Eq 5 (b=0.608,
  z=0.706) scaled to h_max~3.6 km, = "flat-Australia" anorogenic end-member.

**§7 (rejected/uncertain):**
- The bullet *"exact 2–3% (Flament) figure … I did not read it off Flament's
  own hypsometry figure … precise percent as [A]"* — **now resolved to [V].**
  Read off Table 2, Fig 6, §4.3 text. Replace with: 1.8–3.1% deterministic /
  1.79–2.84% median, [V].

**§1 Flament bullet** — the current gloss ("felsic crust buried under thick
basaltic cover, continents flooded by a near-global ocean, crust too weak to
support high mountains") is **broadly faithful** but slightly over-eggs the
"near-global ocean / thick basaltic cover" imagery. Flament's actual causal
chain is narrower and cleaner: *constant ocean volume + hot mantle → thick/
buoyant + flattened seafloor → high sea level; + weak lithosphere → low h_max →
little land above that high sea level.* The "submarine flood basalts / felsic
fingerprint at 2.5 Ga" material is *corroborating evidence* Flament cites
(§4.5), not the mechanism. Minor tightening, not an error.

---

## 8. World-builder extraction (parameters / shapes for a prior / uplift nomos)

Stated as reusable inputs. Tier as marked; all are **Earth-referenced** (survey
§ status `literature`), jettisonable for non-Earth worlds.

1. **Continental hypsometric shape (prior):** Eq 5 — power-law hump
   `b·h_max·((1−A/A_sh)/(a+A/A_sh))^z` above the shelf, **linear** slope to
   −2446 m below it. **b = 0.608, z = 0.706** [read]. Use the **anorogenic
   (Australia-like)** normalized profile for an Archaean world, not the
   orogenic (Asia) one.
2. **Vertical scale h_max ≈ 3600 m** [read] — max continental elevation above
   sea level in the late Archaean (1800–2200 m plateaus + dynamic support to
   ~3600 m). This is *the* knob that sets emergent fraction. Modern equivalent
   ~9050 m above shelf.
3. **Emergent land fraction target: ~2–3% of surface** (1.8–3.1%), median
   ~1.8–2.8% [read] — for a ~2.5 Ga world with 80%-of-present continental area.
   Constant-modern-relief ceiling ≤12%.
4. **Continental area ~34% of surface** at 2.5 Ga (80% of modern 42.5%) [inf/
   read]; grows from ~20%-of-present at 4 Ga.
5. **Freeboard near-zero / slightly negative** — continental platform mode sits
   *at/just below* sea level; drowned platforms under **~800–1100 m** of water
   over ~24–25% of Earth's area [read].
6. **Isostasy constants** [read, Table 1]: ρ_cc 2800, ρ_oc 3000, ρ_m 3300,
   ρ_w 1030 kg/m³; continental crust 41.1 km; **Archaean oceanic crust ~21 km**
   (vs modern ~7 km) — the thick oceanic crust is what raises the seafloor.
   Sea-level sensitivity (Eq 1): Δh_f = Δd_r(1−ρ_w/ρ_m) + Δd_oc(1−ρ_oc/ρ_m) −
   Δd_cc(1−ρ_cc/ρ_m).
7. **Sea level is a derived quantity, not a datum** [inf, but this is exactly
   the paper's method] — it is solved from an ocean-volume budget + isostasy.
   This is the principled path ASSUMPTIONS.md wants for `SEA_LEVEL_M`: fill the
   hypsometry with a declared water inventory and *derive* sea level, rather
   than fixing a datum. Flament's Eq (2)–(4) is a worked template: V_o = V_a +
   V_b + V_s + V_l (ridge reservoir + bathymetric reservoir + over-continent
   reservoir + isostatic-adjustment term), iterated to constant volume.
8. **Caveat for the uplift nomos:** Flament's world floods because relief is
   *capped low*, not because crust is thin or oceans are deep. An uplift model
   should gate emergence on **lithospheric strength / h_max**, letting land
   emerge *during* cooling as h_max rises toward modern values through
   2.5–2.2 Ga — matching the "land emerges during the Abyssal phase via uplift"
   framing in PHASES.md.
9. **Ocean depth:** take deep basins (~5–6 km) from **Korenaga/Dong**, *not*
   Flament — Flament's constant-volume basins are ~2 km shallower than modern
   (Fig 6). If the vivarium water inventory is set larger (deeper oceans), that
   is the Korenaga/Dong branch and reinforces flooding independently of
   Flament's seafloor-rise mechanism.

---

## 9. Load-bearing verbatim quotes

- Abstract: *"the continents were mostly flooded until the end of the Archaean
  and … only 2–3% of the Earth's area consisted of emerged continental crust by
  around 2.5 Ga."*
- §1: *"For a late-Archaean balance of parameters (mantle potential temperature
  150 °C hotter than present and 80% of present continental area) and for
  constant hypsometry, we calculate that the area of emerged crust would be less
  than 12% of the Earth's area."*
- §4.1: *"assuming constant hypsometry, the calculated area of emerged
  continents in the Archaean is between 1 and 12% of the Earth's area."*
- §4.3: *"For a maximum oceanic floor age t_max of 180 Ma, the calculated
  emerged crustal area is 3.1% of the Earth's area. For t_max = 104 Ma, it is
  1.8%."*
- §4.3: *"For a maximum oceanic floor age of 180 Ma (104 Ma) we calculate that
  ~24% (~25.5%) of the Earth's area is covered by water ~800 m (~1100 m) deep.
  Note that these are maximum depths, as sedimentation processes are not taken
  into account here."*
- §4.4: *"The medians of A_f range between 1.79 and 2.84% of the Earth's
  surface."*
- §4.3: *"the maximum elevation of orogenic plateaus during the Neoarchaean
  would have been 1800–2200 m … allowing for a dynamically supported maximum
  elevation up to ~3600 m."*
- Conclusions: *"constancy of the continental freeboard (±200 m) can be achieved
  if the upper mantle potential temperatures have never been 110–210 °C hotter
  than present."*
- Conclusions/§4.4: *"the thickness of the continental crust is the key
  assumption."*
- Conclusions (limitation): *"there is a poor control of ocean volume through
  time … we assume a constant thickness of continental crust over time."*
