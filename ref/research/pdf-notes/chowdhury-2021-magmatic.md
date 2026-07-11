# Chowdhury et al. (2021), PNAS — magmatic thickening → subaerial rise of Earth's first continents, 3.3–3.2 Ga

*Primary-source read for the vivarium proto-uplift mechanism. Compiled 2026-07-10.*

**Full cite.** Chowdhury, Mulder, Cawood, Bhattacharjee, Roy, Wainwright, Nebel, Mukherjee (2021), *Magmatic thickening of crust in non–plate tectonic settings initiated the subaerial rise of Earth's first continents 3.3 to 3.2 billion years ago*, PNAS 118(46):e2105746118. DOI 10.1073/pnas.2105746118. Edited by R. L. Rudnick. Data + MATLAB model: github.com/priyadarshi-geo/Continental-Emersion-2021.

> **House tiering.** Every load-bearing item is tagged:
> - **[page-read]** — read directly off the PDF text or a figure/table on the page cited (I read pages 1–8 with the Read tool's PDF renderer, not a summarizer).
> - **[inferred]** — computed, extrapolated, or cross-read by me from page-read numbers; my arithmetic, not theirs.
>
> Caveat one notch below the survey's: the **SI Appendix was not read** (it holds the full isostasy derivation, Figs. S1–S7, and Datasets S1–S5). Everything here is the main-text + main-figure layer. Figure *values* read off plots carry normal plot-reading error (±ticks).

---

## 0. Headline (the question asked)

**Can this mechanism, as published, be the world's first land-maker? — Yes, and it is the *right* one for the pre-plate-tectonic early-Abyssal phase.** It makes **low-relief emergent cratons** (a few hundred m to ~1 km of freeboard) by the **isostatic rise of thickened, silica-*light* crust**, with **no plate collision required** — exactly the "uplift that predates full plate tectonics" that PHASES.md Phase 3 needs to discharge its `#gate` ("meaningful non-volcanic land above sea level"). Two load-bearing qualifications: (a) **only the isostasy is lawful**; the *driver* — where and how fast crust thickens — is a magmatic/mantle-upwelling source term the paper *assumes as given*, so a faithful nomos gets lawful uplift from a **tuned/fated** source field; (b) it **only clears sea level if the ocean is made deep in the right way** — emergence in their own model is *marginal* and flips on ocean depth (4.4 vs 4.7 km) and protocrust thickness (20 vs 30 km). That marginality aligns precisely with the survey's load-bearing "deepen the oceans" fix and with "only a few percent land."

---

## 1. The mechanism, mechanically

**Setting (what drives it):** voluminous **granitoid magmatism** — a tonalite–trondhjemite–granodiorite (TTG) suite maturing to K-granites — emplaced in a **plateau-like crust above a zone of mantle upwelling**. This is **vertical tectonics** (dome-and-keel), *not* horizontal plate collision. The craton has a **"dome-and-keel structure lacking any regional-scale linear tectonic fabric"** and lacks a distinct metamorphic field gradient — the fingerprint of vertical, not collisional, thickening [page-read, p. 2–3, p. 5].

**Two coupled buoyancy sources — the crust rises because it gets both *thicker* and *lighter*, and grows a light mantle keel underneath:**

1. **Crust thickens AND de-densifies simultaneously.** Progressive TTG magmatism thickens the crust *and* makes it more silica-rich, so bulk crustal **density falls from ~2,980 kg/m³ at ~3.5 Ga to ~2,870 kg/m³ at ~3.2 Ga** as thickness grows [page-read, p. 4]. So the thickening column is **doubly buoyant** — more of it, and each unit lighter — giving "large positive buoyancy and thereby a greater isostatic uplift relative to the surrounding thinner and mafic (more-dense) oceanic crust" [page-read, p. 3].
2. **A melt-depleted cratonic lithospheric mantle (CLM) keel forms at the same time**, itself buoyant relative to the asthenosphere (density contrast Δρ_CLM), adding to the standing height [page-read, p. 5; Fig. 5].

**What pulls it back down / caps it (the competition, as *this* paper frames it):** — note this is **not** the eclogite-foundering limit the densification paper (Sci. Adv., survey §3) emphasizes. Here the competition is a **freeboard race against a deep ocean**:
- The craton must out-rise the **Archean ocean depth (~4.4–4.7 km)** — deeper than modern 3.7 km — to become subaerial [page-read, p. 5].
- The **ambient submarine mafic protocrust** sets the reference seafloor: a *thicker* protocrust (30 km) rides *higher* isostatically, *raising the surrounding seafloor* and so *reducing* the craton's relative relief — pushing it back toward submergence; a *thinner* protocrust (20 km) means deeper surroundings and more emergent relief [inferred, from the 20/25/30 km outcome contrast, p. 5–6].
- Result: emergence is **marginal** — the craton barely clears sea level, and small parameter changes flip it. That marginality *is* the paper's answer to "why so little land."

---

## 2. Quantitative core — crustal thickening (Fig. 2, Fig. 3)

Two independent thickness proxies, cross-checked, both from the granitoid trace-element record:

| Proxy | Quantity | ~3.5–3.4 Ga | ~3.3–3.24 Ga | Source |
|---|---|---|---|---|
| **Petrogenetic (P–T) modeling** | Pressure of TTG formation | ~0.9 GPa | ~1.2 GPa | [page-read, p. 4 + Fig. 3B] |
| → converts to | Crustal thickness | **~32 km** | **~45 km** | [page-read, p. 4] |
| **La_N/Yb_N systematics** (Moho-depth proxy, garnet in melt residue) | Crustal thickness | **~35 km** at 3.5 Ga | **~50 km** at 3.24 Ga | [page-read, p. 4 + Fig. 3C] |

- **Net thickening: 13–15 km between ~3.5 and ~3.24 Ga** [page-read, p. 4]. → **~0.05 km/Myr** average over ~260 Myr [inferred].
- Density conversion assumes bulk crust **2,980 kg/m³ at 3.5 Ga → 2,870 kg/m³ at 3.2 Ga** [page-read, p. 4].
- The thickening is monotonic and quadratic-fit through time (Fig. 3C: n = 69 thickness values, median per 10-Myr bin, quadratic fit with 95% CI) [page-read, Fig. 3C caption].
- **K-granites** (the final maturation phase, ~3.2–3.1 Ga) form by *shallow melting* of already-thick felsic crust, and their existence implies the crust was **>45 km thick** by then [page-read, p. 5].
- Both methods "agree well with the global evolution of Archean continental crust thickness" [page-read, p. 4] — i.e. Singhbhum is presented as representative, not a special case.

---

## 3. Quantitative core — the isostatic elevation model (Fig. 4, Methods)

This is the load-bearing part for a world-builder: **elevation is *derived* from the crustal column, not imposed.**

**What's computed:** the craton's **elevation from the seafloor, H**, via isostatic balance **at the base of the CLM**, as a function of time ~3.5→3.1 Ga. Crucially, **H is computed independently of Archean sea level** (which is poorly constrained) — sea level enters only afterward, to decide *whether H clears it* [page-read, p. 5, Methods p. 7].

**Inputs to H (the isostatic column):** thickness + density of (a) cratonic crust, (b) cratonic lithospheric mantle (CLM), (c) ambient submarine mafic protocrust; plus asthenosphere density [page-read, p. 5].

**Parameter values [page-read, p. 5 + Methods p. 7]:**
- Cratonic crust thickness through time (empirical fit, t in Ma, held constant after 3.3 Ga): `thickness(km) = −4.995×10⁻⁴·t² + [b]·t − 5424` — *the middle coefficient rendered as "3306" in the PDF text almost certainly reflects an OCR/scaling artifact; treat the equation's shape as quadratic-in-time, and use the tabulated 32→45 km endpoints as the trustworthy anchor* [page-read w/ flag].
- Cratonic crust density (kg/m³, t in **Ga**): `596.184·t² − 3565.58·t + 8173.967`. **I verified this numerically:** t=3.5 → ~2,998; t=3.2 → ~2,870 kg/m³, matching the stated range [inferred/verified].
- **CLM thickness** = `√(4kt)` (and a second curve at `0.4·√(4kt)`), with **k = 10⁻⁶ m²/s** thermal diffusivity — i.e. CLM grows as a **thermal (conductive-cooling) diffusion length** [page-read, Methods p. 7].
- CLM allowed to thicken to **0–50 km less than the conductive-cooling length scale**; **Δρ_CLM = ρ_asthenosphere − ρ_CLM = 0–50 kg/m³** (headline results use **25 kg/m³**) [page-read, p. 5].
- **Ambient submarine mafic protocrust: density 3,100 kg/m³, thickness 20–30 km** [page-read, p. 5].

**Results — H through time (for Δρ_CLM = 25 kg/m³) [page-read, p. 5]:**

| Time | H (elevation above seafloor) |
|---|---|
| ~3.5 Ga | **~0.8–1.8 km** |
| ~3.3 Ga | **~3.6–4.6 km** |
| ~3.1 Ga | **~4.5–5.5 km** |

→ **Net ΔH ≈ 3.7 km** between 3.5 and 3.1 Ga [page-read, p. 5]. (The band width at each time is the spread over the 20–30 km protocrust range.)

---

## 4. Emergence — does H clear sea level? (Fig. 4)

Sea level enters here. Archean ocean was **deeper** than modern:
- A recent study (their ref.) puts sea level at ~2.94–3.0 Ga **730–1,000 m higher** than present, → **ocean depths 4.4–4.7 km** vs modern **3.7 km** [page-read, p. 5].

Emergence outcomes (this is the "why so little / marginal land" result) [page-read, p. 5–6, Fig. 4]:
- **20 or 25 km protocrust:** craton **emerges between 3.29 and 3.19 Ga**; **subaerial relief 350–1,100 m at ~3.1 Ga** (the "350" and "1100 m" annotations on Fig. 4).
- **30 km protocrust, ocean 4.4 km:** emerges by 3.1 Ga but with only **~140 m** subaerial relief.
- **30 km protocrust, ocean 4.7 km:** **stays submerged ~130 m below sea level** — no land.
- **Most favorable outcome:** ocean **<4.7 km** deep and protocrust **<30 km** at ~3.1 Ga [page-read, p. 6].
- Fig. 4 shows two independent constraints on emergence timing bracketing the same window: the **sedimentological** constraint (Singhbhum cover-sequence depositional ages, 3.29–3.08 Ga) and the **magmatic-isostatic** constraint (pink band), both landing at ~3.2–3.1 Ga — mutual agreement is the paper's evidentiary strength [page-read, Fig. 4 caption].

**So the emergent topography this mechanism produces is: broad, dome-shaped, low-freeboard cratons standing a few hundred metres to ~1 km above a deep ocean — not peaks, not ranges.** Landform character: dome-and-keel, no linear fabric [page-read, p. 2, p. 5].

---

## 5. Timescales & sequence (Fig. 5)

- **Thickening:** ~3.5 → ~3.24 Ga ≈ **~260 Myr** to add 13–15 km [inferred from p. 4 ages].
- **Sustained mantle upwelling/melting:** **>250 Myr** — an explicitly *prolonged* driver, not a pulse [page-read, Fig. 5 caption].
- **Emersion:** ~3.3–3.2 Ga (Fig. 4/5) — "**significantly precedes the commonly inferred estimate of 2.5 Ga**" by **>700 Myr** [page-read, abstract, p. 5, p. 7].
- Fig. 5 full sequence [page-read]: **>3.55 Ga** mafic protocrust + minor felsic → **3.55–3.25 Ga** polyphase TTG + greenstones (older TTGs shallow, younger deep; granites from ~3.32 Ga) → **3.2–3.1 Ga** K-granites, thickening ends, **emersion + paleosols** → **3.1–2.9 Ga** marginal-marine sediments on emerged crust, O₂ build-up, cooling, stable subaerial craton → **2.94–2.78 Ga** accretionary orogenesis, **onset of plate-margin processes**.
- Key ordering claim: **emersion happens *before* plate tectonics**, which only begins at the Paleoarchean–Mesoarchean boundary as the mantle cools to a mobile-lid regime [page-read, p. 5, p. 7].

---

## 6. Limits — why didn't it make MORE land?

Read straight from the paper [page-read, p. 5–7]:
1. **Marginal freeboard.** Even in the favorable case, relief is **350–1,100 m** — the craton barely clears a deep ocean. Unfavorable parameters (thick protocrust + deep ocean) leave it **submerged**.
2. **Localized.** This is a **craton-scale** feature (Singhbhum ~few hundred km), reproduced independently at Kaapvaal/Pilbara/others by ~3 Ga — a *scatter of emergent cratons*, not a global landmass. Consistent with the survey's "few percent land."
3. **Needs a deep ocean anyway.** The mechanism only *reads as land* because the surrounding basin is deep; it doesn't lift a global surface.
4. **More land came later, by a *different* mechanism.** The authors explicitly say emerged area "**likely increased further between ~3.0 Ga and ~2.5 Ga**" as cooling brought on **plate-tectonic compressional thickening** — i.e. magmatic thickening bootstraps the *first* land; plates finish the job [page-read, p. 7].

---

## 7. Load-bearing quotes

- Abstract: *"the entire Singhbhum Craton became subaerial ~3.3 to 3.2 Ga due to progressive crustal maturation and thickening driven by voluminous granitoid magmatism within a plateau-like setting … driven by the isostatic rise of their magmatically thickened (~50 km thick), buoyant, silica-rich crust. The inferred plateau-like tectonic settings suggest that subduction collision–driven compressional orogenesis was not essential in driving continental emersion, at least before the Neoarchean."* [page-read, p. 1]
- *"thick, silica-rich (less-dense) crust, which experiences large positive buoyancy and thereby a greater isostatic uplift relative to the surrounding thin and mafic (more-dense) oceanic crust."* [page-read, p. 3]
- *"we suggest that the emersion of Earth's cratons initiated at ~3.3 to 3.0 Ga due to crustal thickening and plutonism within non–plate tectonic settings … This time estimate significantly precedes the commonly inferred estimate of 2.5 Ga."* [page-read, p. 5–7]
- Methods (the actual model spec): *"crustal density (kg/m³) = 596.184 t² − 3565.58 t + 8173.967 (t in Ga); and CLM thickness = √(4kt) … where k = 10⁻⁶ m²/s is thermal diffusivity."* [page-read, p. 7]

---

## 8. Survey corrections (`early-continents-survey.md`)

The survey's PNAS-2021 entries were **[A]-grade (abstract/secondary)**; this read promotes them to **[V/page-read]** and sharpens several numbers. Suggested edits:

1. **§3 & §8 — upgrade tier and cite.** The survey lists Chowdhury 2021 as `[A]` in both §3 and the source ledger. It can move to `[V]` (full main text + figures read). Attribution note: the survey once calls it *"Bindeman-adjacent / Chowdhury et al."* (§3) — **Bindeman is not an author.** First author **Priyadarshi Chowdhury** (Monash); Bindeman appears only as the *2018 O-isotope 2.4 Ga* land-rise reference elsewhere. Drop "Bindeman-adjacent."

2. **§3 — the "~50 km thick" figure is a maximum, and there are two proxies.** Survey says "magmatic thickening to ~50 km silica-rich crust." Precise: **32→45 km (petrogenetic) / 35→50 km (La/Yb)**; ~50 km is the *late* La/Yb endpoint at 3.24 Ga, and K-granites imply **>45 km**. Net thickening **13–15 km over ~260 Myr**. Worth stating the range, not just the ceiling.

3. **§6 target table — the mechanism's *relief* number is now concrete.** For emergent cratons made this way, **freeboard is 350–1,100 m** (favorable case), or **~140 m** to **submerged** (marginal case). This is a *tighter, page-read* anchor for the table's "continental freeboard: low or slightly negative" row and its "max relief ~3–5 km" row — note the **freeboard** (land above sea) is sub-km even though total column relief **H** above seafloor reaches **4.5–5.5 km**. Survey should distinguish **H-above-seafloor (4.5–5.5 km)** from **freeboard-above-sea-level (0.14–1.1 km)** — the paper is a clean source for that distinction.

4. **§5 / §6 — this paper independently corroborates "deepen the oceans."** Chowdhury uses **Archean ocean depth 4.4–4.7 km** (sea level 730–1,000 m above modern) as a *model input*, and emergence *fails* if the ocean is too deep. This is second-source support (beyond Korenaga/Dong) for the survey's headline claim that early oceans were **deeper** (~5–6 km neighborhood) and that the placeholder's shallow ocean is the load-bearing error. Add Chowdhury 2021 as a corroborating anchor on §5.

5. **§2 — crustal-density contrast is *dynamic*, not static.** The survey treats felsic vs mafic as a fixed two-population contrast. This paper shows the felsic column **actively de-densifies** (2,980→2,870 kg/m³) *as it thickens* — buoyancy is a moving target driven by magmatic differentiation. Minor but relevant to any "two-mode crustal prior."

No survey claim is *contradicted*; these are tier-upgrades and precision sharpenings.

---

## 9. World-builder extraction — a crude *proto-uplift nomos*

A sketch of the smallest honest nomos implementing this mechanism, in ARCHITECTURE §9 / nomotheke language. **This is a design sketch, not a spec** [inferred throughout, grounded in the §1–5 quantities].

**Name (provisional):** `magmatic_uplift` (or `isostatic_emergence`). Discharges PHASES.md Phase-3 `#gate` "meaningful non-volcanic land above sea level" via a `#mech` that is *not* plate tectonics.

**Keyed inputs (the complete content-addressed key):**
- per-column **crustal thickness** field `C(x,t)` and **crustal density** `ρ_c(x,t)`;
- **CLM thickness/density** (or the `√(4kt)` thermal-diffusion closure + Δρ_CLM);
- **ambient protocrust** thickness `T_p` (20–30 km) and density (3,100 kg/m³);
- **asthenosphere density** (reference);
- **sea-level datum / water inventory** — *consumed from the water/hydrology nomos*, not owned here;
- time `t`; the fated **upwelling/magmatism source field** (see driver below).

**Fluxed quantities (fine-grained, §9's "only thing others may depend on"):**
- *Produces:* **bedrock elevation** (or **H = elevation-above-seafloor**), updated **crustal thickness**, updated **crustal density**. Emergent land fraction and freeboard are *derived queries* off H vs sea-level datum — **not imposed** (this is what retires `SEA_LEVEL_M` as an arbitrary relief-setter; ASSUMPTIONS.md line 13).
- *Consumes:* a **crustal-thickening rate** `dC/dt` from a magmatism/heat source; the **water inventory / sea-level datum**.

**The lawful core (high physics-fidelity, axis B):** pointwise **Airy isostatic balance of the column stack** {crust, CLM, protocrust} against asthenosphere → H. This is genuine Archimedes, cheap, algebraic per-column. Good **R** (macro summary = mean-thickness → mean-elevation via the same balance) and trivially memoizable/fated. The density-vs-time and CLM-vs-`√(4kt)` closures are the paper's own and can be used *as intended* (`literature` status in the ledger).

**The un-lawful half — declare it loudly (low B, the honest fudge):** *what thickens the crust, where, and how fast* is **not derived** — the paper *assumes* voluminous granitoid magmatism above a mantle-upwelling zone sustained >250 Myr. A crude nomos supplies this as a **fated source field**: seed "upwelling provinces" per-face, each delivering `dC/dt ≈ 0.05 km/Myr` (their 13–15 km / 260 Myr) with a coupled density drop 2,980→2,870 kg/m³ over the same interval. This source term is **`tuned`/`fated`, not `literature`** — it is the vibe-modeled part (high D on low B/C) and must say so in the nomotheke, or it becomes *undiscoverably* unLawful (ASSUMPTIONS.md rule).

**Execution class & timescale band:** **quasi-static, geological — ~10⁷–10⁸ yr** (thickening over 260 Myr). Decoupled from the hydrology timestep exactly per the house "separate the timescales" lesson (CLAUDE.md): uplift advances on the erosion/tectonic clock; water re-equilibrates against a frozen bedrock snapshot. This nomos is a **slow** producer feeding the water sim's boundary.

**Bequests + conservation claims (§9 item 5):**
- **Bequest: bedrock elevation H** — conservation claim: isostatic balance holds per-column (mass-of-column ↔ displaced-asthenosphere), continuous across tile seams.
- **Bequest: added crustal volume** — conservation claim: crust added = mantle melt extracted (a **mass ledger** the magmatism source must honor; today's rock-mass conservation is `NOT enforced` per ASSUMPTIONS.md — this nomos would need the flux-BC spine before its source term is honest).

**Epistemic tags to declare (the four axes, §7):**
- **A (Earth-history fidelity): Medium.** Real Earth mechanism with real numbers, but craton-specific (Singhbhum), and CLM Δρ / protocrust thickness are poorly constrained inputs the paper sweeps over.
- **B (physics fidelity): split — High for the isostatic closure, Low for the magmatic source term.** The nomos must not let the lawful isostasy launder the tuned driver's low B. Declare per-bequest.
- **C (relation type): `#mech` discharging a `#gate`.** *Tension to flag honestly:* the land is *magmatically built* (igneous granitoid crust) yet rises by **isostasy, not by piling lava above water** — the surface is an eroded felsic craton, not an active volcanic edifice. It plausibly satisfies "**non-volcanic land**" in the sense PHASES.md means (broad emergent craton, not a volcano poking up), but a reviewer could contest the label — surface it, don't bury it.
- **D (implementation status): unbuilt.** New nomos; would follow the store/nomos + flux-BC spine currently gapped (ARCHITECTURE §8).

**Regime probes to write first (§9 item 6):**
1. **Freeboard probe** — emergent land must stand only **~0.1–1.1 km** above sea level, and total **H ≤ ~5.5 km** above seafloor; if the nomos makes km-scale *freeboard* it is wrong (matches paper; guards against re-importing modern relief).
2. **Marginality probe** — emergence must be *sensitive*: flipping ocean depth 4.4↔4.7 km or protocrust 20↔30 km should be able to flip land↔submerged. If emergence is robust to those, the nomos has lost the physics.
3. **Space-seam probe** — H continuous across tile boundaries (isostatic column is local, so this should hold; `seam_ridge`-style).
4. **Time-seam probe** — thickening near-stationary over one water timestep (quasi-static check).
5. **Land-fraction probe** — output should read as a *scatter of low, broad emergent cratons* totalling a few percent of surface, **not** a connected global landmass (matches survey §6 land 2–15%).

**Honest limits the nomos inherits (do not paper over):**
- Makes **only low relief** — cannot produce ranges or peaks. Correct for early-Abyssal; a *feature*.
- Makes **localized patches**, not continents. Correct.
- **Depends on a deep ocean** to read as land — so it is coupled to getting the water inventory right first (the survey's load-bearing fix). If the ocean stays a shallow pan, this nomos produces *no* land no matter how it thickens crust.
- **Its driver is assumed, not derived.** The lawful uplift is real; the source field is tuned. That is the single most important thing for the ledger to state plainly.

---

## 10. One-line for the caller

Yes — publishable, quantitative, and mechanism-complete: isostatic rise of thickened silica-light crust makes the first low-freeboard cratons ~3.3–3.2 Ga with no plates. The isostasy is lawful and directly codeable; the magmatic *driver* is an assumed source term (a fated/tuned field in any nomos); and it only yields land against a **deep** ocean — which independently seconds the survey's "deepen the basins first" verdict.
