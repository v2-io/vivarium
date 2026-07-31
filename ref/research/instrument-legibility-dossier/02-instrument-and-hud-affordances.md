# 02 — Instrument and HUD affordances

*Half-life: a decade. Domain-specific, engine-independent. Alerting, mode indication, unreliability annunciation, overlay practice, always-on vs on-demand.*

## Status of this document

**No primary source was read by the writer.** Chain: primary → extraction agent → synthesis agent → here. Several sources in this area were *paywalled or 403-blocked even for the extraction agents*, meaning some entries are abstract-level or registry-metadata only; where that is so it is stated on the entry.

Tags as in 01: **`[VERIFIED 3/3]`** survived three adversarial passes · **`[CONTESTED nR/nK]`** mixed · **`[REFUTED]`** checked and killed, retained so it is not re-derived · **`[UNVOTED]`** extracted with a verbatim quote, never checked.

Verbatim quotes and URLs: [`appendix/synthesis-practice-safety-critical.md`](appendix/synthesis-practice-safety-critical.md), which covers close to the full unverified pool for this run. The full raw pool (139 mined claims, 29 sources) is [`appendix/harvest/run2-practice-hud-telemetry-safety-critical.md`](appendix/harvest/run2-practice-hud-telemetry-safety-critical.md).

### One source-level warning that governs §1

**14 CFR 25.1322 had three of its five extracted claims put to adversarial vote. All three came back refuted or contested** — and in every case the *quote was verbatim-genuine* (confirmed against Cornell LII, govinfo XML and the eCFR renderer API) while the extracted claim's **gloss** overreached. Its two unvoted claims inherit that risk. This is the clearest instance in the corpus of the general pattern: with regulations, the quote verifies and the sentence after it is where the defect lives.

---

## 1. Flight-deck alerting and regulation

### 1.1 What is binding, and what survived checking

**`[CONTESTED 1R/2K]` — core survives.** 14 CFR 25.1322(c)(2): Warning and Caution alerts must *"Provide timely attention-getting cues through at least two different senses by a combination of aural, visual, or tactile indications."* Single-channel visual is non-compliant for the top two tiers.

Two glosses were killed and the corrections matter: the provenance is **Amdt. 25-131, 75 FR 67209, Nov. 2, 2010** — not "decades ago"; and the **Advisory tier carries no two-senses requirement**, so scope-inflating this into "a truth existing only as one channel is never said" oversteps the clause.

**`[REFUTED 3/3]`** The claim that (b)'s three-tier hierarchy is defined by *required response timing* rather than severity. The quote is genuine — Warning (immediate awareness **and** immediate response) / Caution (immediate awareness, subsequent response) / Advisory (awareness, response possible), under a lead-in reading *"Alerts must conform to the following prioritization hierarchy based on the urgency of flightcrew awareness and response."* What failed: the claim's own falsifiability test ("any transport-category flight deck certified after Nov 2010") is wrong — the **changed-product rule** (14 CFR 21.101) governs applicability, not certification date, and 21.17(a)(1) ties requirements to the date of *application*. The 737 MAX, type-certificated 2017, was exempted from EICAS and granted exceptions to crew-alerting regulations. And "not by severity" is contradicted by 25.1309's severity ladder existing alongside. Also unsupported: the reading that Caution means "must not act yet" — no such prohibition appears in the section.

**`[CONTESTED 1R/1K]`** 25.1322(d)(2) requires a *"clear and unmistakable annunciation"* when an alert is suppressed, with a guarded control. **The over-read that was killed:** this is not "suppression is never silent." The same section's (c)(3) explicitly permits attention-getting cues *"to be acknowledged and suppressed, unless they are required to be continuous."* (d)(2) is scoped to failure of the **alerting function itself**, per the (d) chapeau about false and nuisance alerts — not to routine operator acknowledgment.

**`[UNVOTED]`, lower overreach risk** — a near-literal restatement. 25.1322(e): colour is a reserved namespace. Red = Warning, amber/yellow = Caution, Advisory may use *"any color except red or green"* (green is withheld even though it is not itself an alert colour, protecting its non-alert meaning). Monochromatic displays must substitute another visual dimension.

**`[UNVOTED]`, inherits the gloss risk above.** 25.1322(a)(2)/(a)(3): simultaneous multiple alerts are named as an explicit certification load case (*"readily and easily detectable… including conditions where multiple alerts are provided"*), and alerts must be removed when the condition no longer exists.

### 1.2 FAA AC 25-11B — guidance, not binding regulation

None of this went to vote. It is an FAA advisory circular (2014-10-07) — the agency's own acceptable-means-of-compliance document, quoted with clause numbers. All **`[UNVOTED]`**.

- **§5.11.3.4:** *"a text change by itself should not be used as an attention-getting cue (for example, to annunciate mode changes)."*
- **§5.11.2:** fixed-location annunciation required, with anything needing immediate awareness in the crew's forward/primary field of view.
- **§5.11.4:** blink escalation bounded at **0.8–4.0 Hz**, with a cancel-means required if blinking persists beyond ~10 s.
- **§4.6.7:** staleness/validity checks required for safety-critical parameters "where possible"; defines **"Coasting Data"** (C.12) as data not updated for a defined period. **§6.2.1.7:** failure flags *"should be presented in the location of the information they reference or replace."* **§4.6.8:** alert latency bounded by criticality and required crew response time.
- **§5.10.3.1:** clutter is defined by *cost* — *"increased flightcrew processing time for display interpretation"* — not by appearance. **§5.8.3:** parameter colour-coding capped at six colours.
- **§6.3.3.1** gives a **six-part admissibility test** for showing information only part-time: not continuously needed; auto-displays when abnormal or relevant to a failure; manually selectable without interference; fails safe per §25.1309; does not create unacceptable clutter on arrival; **and must be accompanied by alerting if its arrival is not self-evident.**
- **Table 4-1/4-7 hazard classification:** a *misleading* attitude or engine display ranks **Catastrophic**; *loss* of the same display ranks **Major-Hazardous**. Appendix C.66 demotes "misleading" only if a monitor auto-detects the fault and annunciates it.
- **§5.9.1** bounds rather than bans smoothing: *"filtering or coasting of data intended to smooth the motion of display elements should not introduce significant positioning errors or create system lag."* Numbers given: update rate **≥15 Hz** for manual-control parameters; attitude display lag **≤100 ms** (first-order equivalent time constant). **§5.11.5.3:** *"image failure, freezing, coasting, or color changes should not be misleading."*

---

## 2. Announcing an instrument's own unreliability

### 2.1 The convergence, and exactly how well-supported it is

Three literatures that do not cite each other arrive at *misleading is worse than absent*:

- **Aviation** — the AC 25-11B hazard tables above. **`[UNVOTED]`**
- **Nuclear** — NUREG-0700 Rev. 3 (NRC staff report, 2020-07). **`[UNVOTED]`**
- **Visualization** — TVCG 2019 dashboard study: field research found the dashboard *form itself* manufactures unearned credibility; people read dashboard data as trustworthy and definitive *because* the visual organization signals objectivity, independent of data quality. **`[UNVOTED]`**, and the paper's tier is qualitative case-study, not controlled experiment.

**All three legs are unvoted.** The convergence is real in the sense that three separately-fetched, mutually non-citing sources say compatible things; it is not verified in the sense that any of them was adversarially checked. Treat the *pattern* as the finding and each leg as a lead.

### 2.2 The nuclear guidance, which is the most specific material in the corpus on staleness

All **`[UNVOTED]`**, NUREG-0700 Rev. 3:

- **Guideline 1.1-23:** freeze mode requires an "obvious reminder," explicitly recommending a flashing message. **2.5.4-6:** frozen displays must be labelled as frozen. **2.5.4-7:** the operator must be advised when a significant *underlying* change has occurred that the frozen display is not showing.
- **Guideline 14.2-2:** sensor/instrument failures must produce *"distinct display changes"* directly indicating invalidity (e.g. offscale indication).
- **§14.3 four-state validity taxonomy**, required on the display face: **valid / invalid / unvalidated / numerical estimate**. 14.3-4 is the notable clause — **"unvalidated"** (checks could not be run) must be its own distinct state, not collapsed into "invalid." 14.3-5 requires a quality indicator with those enumerated values, *"so operators can exercise judgment in interpreting them."*
- Designers must affirmatively analyze whether an instrument's *own degradation* could be mistaken for a real process disturbance — naming human reliability analysis, confusion matrices and misdiagnosis tree analysis as methods, grounded in IEEE Std. 497-2002 §6.5 and IEEE Std. 603-1998.

### 2.3 Two mechanism classes, distinct from an annunciator

**`[UNVOTED]`, one 1988 lab dual-task study.** **Likelihood alarm displays** — encode the automated monitor's confidence *in-band on the alert itself* rather than in a separate readout. Findings: LADs did **not** necessarily increase attentional load; the likelihood level entered operators' decision process and improved attention allocation across concurrent tasks; performance improved on both primary and secondary task; tested in two channels (colour-coded visual and synthetic speech), both working. *Sorkin, Kantowitz & Kantowitz 1988. Population is 1988 lab dual-task participants, not field operators.*

**`[UNVOTED]`.** **Legibility-degradation encoding** — make the value itself harder to read in proportion to its uncertainty (blur proportional to positional error, or hue-desaturation toward gray), so the most uncertain values become indistinguishable. Measured to shift how much weight viewers give uncertainty versus a standard bivariate colour map, at a real cost to point-lookup precision. *Padilla, Kay & Hullman 2022, citing Correll/Moritz/Heer.*

### 2.4 A correction that cuts against the intuitive reading

**`[UNVOTED]`, replicated within the cited source.** Viewers substitute an uncertainty *graphic* for a deterministic reading — mistaking 95% CI error-bar caps for high/low temperature bounds — and the error **survives an on-screen key and explicit instruction**. Critically, the same error was **not** found for **textual** descriptions of the same information.

The lesson is not "draw it instead of writing it." It is that **a graphic can install false certainty that a legend cannot remove.** *(Joslyn & LeClerc, cited within Padilla, Kay & Hullman 2022.)*

**`[UNVOTED]`, replicated across map contours, Google Maps' blue dot, and bar charts.** Any hard visual boundary makes viewers treat inside/outside as categorically different populations — subconsciously and instruction-resistant, even when they can correctly state the right interpretation afterward. A threshold rendered as a hard colour flip will be read as physically meaningful whether or not it is.

**`[UNVOTED]`.** Hypothetical Outcome Plots (animated random draws, ~400–500 ms/frame) empirically outperform static error bars, icon arrays, line ensembles and violin plots for lay viewers on uncertainty comprehension — **and** the chapter names the failure mode: viewers may read the animation's frame sequence as an event unfolding in time.

**`[UNVOTED]`.** Displaying uncertainty is not guaranteed to help. In ensemble hurricane displays viewers *overreact* to a single ensemble member striking their location without correspondingly reacting to one narrowly missing — and the overreaction is **stronger with fewer members shown** (1-of-9 worse than 1-of-33), only partially fixed by training.

---

## 3. Mode awareness

**`[VERIFIED 3/3 — IDENTITY/ABSTRACT ONLY]`** The operational failure is a mode-indication failure, and the failing artifact is a status chip: in glass-cockpit flight decks a flight-mode transition is annunciated by the onset of a green outline box drawn around the alphanumeric mode indication on the primary flight display — **and pilots miss it.**

*Nikolic, Orr & Sarter (2004). Publisher 403s; identity confirmed via Crossref, OpenAlex and Semantic Scholar independently by three verifiers. Body text was not read by anyone in this chain.* **Tag corrected 2026-07-31** (external audit finding, mirrored from 01 §2.5): three passes verified identity/abstract, not the empirical claim against a read body — see 01 §2.5 for the full correction note.

**`[UNVOTED]`, secondary-summary-derived — the primary was inaccessible.** Sarter & Woods (1995), 985 citations:

- Mode proliferation has a cognitive cost that must be paid in the interface; multiplying modes without growing monitoring support creates new error classes.
- Mode awareness is a **continuous attentional** task, not a knowledge task.
- Dual causal attribution for mode errors: operators' buggy mental models **and** opaque, **scattered** interfaces — *"numerous mode indications distributed over multiple displays"* with delayed feedback. This names "state is present but spread out and low-salience" as its own failure mechanism, independent of content correctness. *(This phrasing is from a secondary summary's paraphrase, not the primary.)*

**⚠ Do not cite without finding the primary:** an aborted-takeoff scenario in which 65% of pilots did not realize automation controlled thrust and only 15% could accurately describe the active mode afterward. Sample size, aircraft type, and even *which* Sarter & Woods paper the figures come from could not be verified.

**`[UNVOTED]`.** NUREG-0700 9.4-1 / 9.4-5 / 2.1-34 / 2.7.6-9: mode indication must be always-present and highly salient, with notice both before and at an automatic mode change — *"the user should not have to query the system to determine the current mode."*

---

## 4. Alarm management

**`[VERIFIED 3/3]`** ISA-TR18.2.5-2022 defines alarm-system quality as a *measured* quantity with named metrics — alarm rates, standing alarms, operator response times — and named pathologies, specifically **alarm floods** and **stale alarms**. **The verified finding includes its own dead end:** the public index page carries **no numeric thresholds**; those live in the paywalled standard body and in EEMUA 191.

**`[CONTESTED 1R/2K]`** ISA-TR18.2.8-2023, *"Guidelines for Non-Alarm Notifications,"* is real and correctly designated. Its scope: *"non-alarm notifications in automation systems, providing guidance for managing alerts, prompts and notices directed at operator and non-operator roles."* **The dissent:** the claim's framing that its purpose is preventing "alarm overload" is not supported by the source page — that phrase is absent — and the report is partly aimed at personnel *beyond* the control-room operator (maintenance, engineering, management).

**`[CONTESTED 1R/2K]`** ISA-18.2 treats alarms as a governed lifecycle artifact — identification, rationalization, implementation, maintenance, change management — and bundles HMI design guidance with prioritization and performance monitoring. Quote verified verbatim on the ISA page. **The dissent, which is the operative one:** ISA-18.2 governs **alarms** (signals demanding operator response), not informational indications; the existence of the separate TR18.2.8 for non-alarm notifications is itself evidence that its rationalization discipline does not extend to status indications generally. The claimed harmonization with IEC 62682 is also unsupported by the cited page, which mentions IEC 62682 only in a list of "industry regulations."

### 4.1 The unresolved number

**`[UNVOTED]`, and internally inconsistent.** Two secondary sources cite EEMUA 191 for the same quantity and give **different answers**:

| Reported | Source | Tier |
|---|---|---|
| ~12 alarms/hr long-term average; 18/hr action limit | chemengonline.com, 2016-03 | secondary |
| ≤1 alarm/10 min steady state (=6/hr); <10 alarms in first 10 min post-upset | humanfactors101.com, undated | blog |

Neither is the primary standard. A hypothesis offered by one synthesizer **from training, explicitly not verified in-session**: EEMUA 191 may define acceptability *bands* rather than one ceiling, which would make both real while naming different bands. **Unresolved. Do not cite a figure here.**

Other EEMUA-attributed figures, same secondary source, same caveat: flood entry >10 new alarms/10 min, exit <5/10 min (hysteresis); target priority distribution 80% low / 15% medium / 5% high *as experienced by the operator*; stale alarm ≈24 hr continuous; chattering = 3+ activations/min.

**`[UNVOTED]`, blog-tier, but a real incident:** Milford Haven 1994 — 2,040 configured alarms, more than one every 2–3 s during the upset, 275 in the final 11 minutes, ~£48M damage.

---

## 5. Ecological Interface Design

**`[UNVOTED]` throughout.** Included because it is the tradition that most directly addresses "make the physics legible," and because its own literature is more self-critical than its reputation suggests.

- **Vicente & Rasmussen (1992), founding paper.** Formalizes fault detection as constraint-violation detection and derives a hard content requirement: the interface must represent the **complete** set of goal-relevant constraints, because designers cannot know in advance which will break. States explicitly that enumerating fault cases in advance *"cannot, by definition, cope with unanticipated events."*
- **The same paper's self-disclaimer, which is the most important line in this section:** specifying the right *content* does not solve the *attention* problem — *"making the necessary information available does not guarantee that it will be attended to or interpreted correctly."* EID does not supply the attention half.
- Empirical base: DURESS thermal-hydraulic simulation, expert/novice subjects, 25–30 s viewing windows. Physical+Functional interface beat Physical-only on diagnosis, primarily for experts. A "Random" control (a simulated system violating physical law) ruled out a pure visual-form explanation. **No effect sizes in the paper**; authors label it "preliminary." Generalizability beyond process control is explicitly an open question; robustness under sensor noise is named untested.
- **Vicente (2002) review.** EID's measured advantage is **conditional on task regime** — it holds for problem-solving and unanticipated events, and predicts little-to-no advantage for routine well-anticipated operation. As of 2002, ~12 years after founding, **no full-fidelity-simulator evaluation with professional workers had been run**; the evidence base was lab/microworld scale. The framework's originator states significant unresolved challenges block confident industrial adoption.
- **Borst, Flach & Ellerbroek (2015)** argues the opposite of the prevailing minimalist instinct: as automation increases, show *more* — the operator must monitor the machine's rationale, not just its output. It also states that making constraints explicit makes sensor failures **more** detectable, contingent on mapping quality. And it diagnoses modern glass cockpits as still substantially "single-sensor-single-indicator" — one display element per measured value — calling the defect architectural rather than stylistic. The paper is co-authored from within the tradition and reports that its authors' own explorations produced multiple usability concerns and misconceptions about EID's viability.

**A tension worth recording rather than resolving:** Borst/Flach/Ellerbroek's "show more" sits in direct opposition to Few's saliency-budget minimalism, ISA-101's restraint, and NUREG's "minimize density for critical information." None of these sources cite each other. This is a live disagreement between two safety-critical design traditions, not a settled consensus with one deviant.

---

## 6. Layout and clutter in practice

**`[UNVOTED]`, abstract-level only (paywalled).** **Proximity Compatibility Principle** (Wickens & Carswell 1995): elements that must be combined in one mental operation ("task proximity") should be rendered close in perceptual space ("display proximity"). It is explicitly **not** monotonic or single-mechanism — the paper distinguishes multiple forms of task proximity and multiple manipulations of display proximity (spatial nearness, object integration, colour/coding linkage), each running through a different information-processing mechanism. "Put related things together" underspecifies the prescription. Empirical warrant is a review across four domains, not a new experiment; no effect sizes recoverable here.

**`[UNVOTED]`.** NUREG-0700 guideline 1.5-8 caps packing density at **50% overall / 25% for alphanumeric-heavy displays**, and instructs *minimizing* density for critical information. **Its own provenance code traces to Smith & Mosier's 1986 NASA/MITRE handbook** — codified handbook practice, not an experimentally derived threshold.

**`[UNVOTED]`.** A 2015 *Human Factors* clutter review states ergonomics has no established quantitative density ceiling, and decomposes clutter measurement into four checkable families: image-processing algorithms, task-performance evaluation, subjective rating, eye tracking. Its explicit recommendation is to **combine** complementary techniques — a single measure is methodologically insufficient by that review's own standard.

**`[UNVOTED]`, N=16, non-professional sample (students/researchers, not certified controllers).** In an ATC means-ends study, noticing something is wrong and identifying *what* is wrong proved to be **separable capabilities with different display requirements**: both interface conditions let operators sense an anomaly; only the means-ends-linked group could localize it (χ²(2)=4.923, p=.027; χ²(2)=5.208, p=.022). Density degraded fault detection with a named mechanism — overlapping derived glyphs occluding the inconsistency — not merely "clutter is bad." One reported statistic in this paper, F(1,14)=206.249, p=.051, is **internally implausible** and was flagged as likely erroneous by the extraction; the response-time cost it supports should be treated as unconfirmed.

---

## 7. Dashboards

**`[UNVOTED]`.** The TVCG 2019 dashboard taxonomy is a descriptive framework from qualitative open coding of 83 dashboards, 15 factors → 7 genres, Cohen's κ=0.64 (86.5% agreement). **The authors state they consulted no users or designers and that the framework "may fail to isolate best practices."** Epistemic tier: craft-consensus systematization with one methodology statistic — *not* evidence that any of the 15 factors improves noticing, accuracy or speed.

Two of its statements worth keeping: dashboard design principles are **not genre-portable** (rules tuned for glanceable operational dashboards may fail for narrative ones); and its stated operational-genre rule is to order views by **criticality**, not by temporal or semantic relationship, even when that breaks the data's natural order.

Its density claim rests on a single 1979 accounting-information-systems lab study — a 40-year-old off-domain result standing in for a modern density threshold, flagged as a gap by the paper's own authors.

**`[UNVOTED]`, entirely craft tier.** Stephen Few's dashboard heuristics, recovered from a 2006 Perceptual Edge white paper rather than the book. Few's own evidentiary standard for most of these is personal introspection on a single screenshot. Recorded because they are the craft-consensus ancestor of much instrument-panel practice, not because they are evidenced: single-screen/no-scroll as *definitional*; uniform visual prominence = zero attention direction = display failure; a bare current value with no comparative context and no separate fast qualitative cue is a defect; colour's highlighting power depletes with overuse; skeuomorphic instrument styling (gauges, meters, traffic lights, LED-look readouts) condemned as decoration with a novelty-decay failure mode. One uncited figure: ~10% of males / ~1% of females colour-blind.

---

## 8. Game HUD — what is measured, and what is lore

### 8.1 The canonical academic framework contains no alerting axis

**`[UNVOTED]`, verified by full-text term search within the extraction.** Bowman, Elmqvist & Jankun-Kelly (2012), "Toward Visualization for Games," IEEE TVCG 18(11) — the field's most-cited reference work — **contains no empirical evaluation whatsoever**: no user study, no participants, no eye-tracking. Its sole stated validation is expressive coverage. Every design assertion in it sits at craft tier.

Its design space has **no axis** for urgency, alerting, criticality or data reliability (zero hits for alarm/alert/notification/uncertainty/stale/reliability/salience/clutter/density).

It does independently arrive at one structural split: a Temporal Usage dimension separating always-visible continuous status from call-up "intermittent" display, justified on distraction grounds.

It also records the bars-over-numbers convention as designer *belief* — attributed explicitly to what "the game designers feel," with no citation to perceptual literature and no measurement.

**`[UNVOTED]`, established by resolving all 49 references of a 2024 CHI PLAY in-game-UI taxonomy:** none are Fagerholt & Lorentzon's "Beyond the HUD," none are games-user-research or heuristic sources (Pinelle, Desurvire, Hodent), and none are human-factors, avionics, alarm-management or EID sources. **These are three mutually non-citing literatures as of late 2024** — established by bibliography resolution, not assertion.

### 8.2 Measured results that contradict widely-repeated craft lore

All **`[UNVOTED]`**, all primary studies.

- **Caroux (2022)**, four experiments. HUD *size* (9% vs 15% of screen, The Witcher III, N=40) and *colour* (red vs blue, CS:GO, N=36) produced **near-zero effects** on performance and subjective experience (η²p < .01 in most tests). Removing task-**relevant** elements (avatar status, mini-map orientation; League of Legends) caused large decrements (η²p = **.41** and **.67**); removing task-**irrelevant** elements (score, pseudonyms, item counts) left performance near full-HUD levels. Removing control-mapping elements (Zelda, N=41) changed neither progress (F(1,39)=1.11, p=.30) nor immersion (F(1,39)=2.41, p=.13) — a direct **non-replication** of the belief that minimal HUD improves expert immersion. And 78% of experienced players *consciously noticed* the colour change (vs 6% of less-experienced) while **no** performance or affect measure moved; the salience/effort arm of Wickens' SEEV model failed in 3 of 4 experiments while its value/expectancy arm held.
- **A 2018 four-experiment HUD taxonomy test** (expert FPS players, custom Unity game): display *type* (HUD / diegetic / spatial) is **not** the determining variable — no class won universally; performance was determined by properties of the specific display. The authors state this contradicts the prevailing "Beyond the HUD" recommendation literature. For a precision task, **numeric readouts beat bar and icon encodings by a large margin** (~76% vs ~46% successful-escape rate). For a spatial task, an in-world navigation line massively outperformed an abstract compass panel (~21.6 s vs ~56.4 s to waypoint; ~2% vs ~42% incomplete trials).
- **CHI PLAY 2015 diegesis/immersion study.** Removing the entire non-diegetic HUD raised immersion **for experts only**; novices unchanged. Neither main effect was significant alone; the effect was carried entirely by an interaction at **F(1,20)=4.32, p=.051**, with the expert simple effect at p=.058 — i.e. above the conventional threshold, interpreted via effect size. The measured cost was specifically **attention and control**, not difficulty or affect. The same paper's Study 1 (N=9) trended in the *opposite* direction on the Core Elements of Gaming Experience scale (t=-1.97, p=.08, d=.66, favouring keeping the HUD). The authors admit their diegetic/non-diegetic taxonomy is unstable, and note that as of 2015 the industry doctrine that diegetic integration increases immersion had **no empirical backing** — which this paper does not supply either, since it manipulated *removal*, never diegetic re-presentation.
- **Caroux & Isbister (2016) eye-tracking**, n=15 (3–5 per cell), authors flag as needing dedicated study: transient/non-permanent HUD elements drew **very few eye fixations** compared with permanently displayed elements. Separately (N=48, rated screenshots): the unmodified full HUD was rated **highest** and HUDs with permanent elements removed **lowest**. And PCP-conforming re-layouts were rated **lower** than unmodified commercial originals — the authors' own hypothesis refuted; PCP only won among modified layouts once the familiar original was off the table.
- **Same source, methodologically important:** player self-report about HUD use is unreliable and can be flatly contradicted by gaze data. Novices said in interviews they did not use the HUD; eye-tracking showed they fixated it no differently from experts.
- **Kriglstein, Wallner & Pohl (CHI 2014)**, N=29: encoding choice for gameplay telemetry is task-dependent, not globally rankable — heatmaps for hot-spot detection, cluster representations for comparing variables, with relationship discovery attributed specifically to the cluster representations rather than the heatmap.

---

## 9. Checked and killed

**`[REFUTED 3/3]`** — 14 CFR 25.1322(b)'s three-tier hierarchy as a response-timing taxonomy. See §1.1 for the three independent grounds.

**`[CONTESTED 1R/0K]` — treat as failed.** The claim that peripheral automation displays "did not degrade concurrent visual task performance," advanced as a measured refutation of the attention-cost tradeoff. The source (Nikolic & Sarter 2001, *Human Factors* 43(1)) is real and the abstract quote is verbatim, but the publisher page 403s so the method, N and interference statistics **could not be checked** — which is precisely why "measured refutation" cannot be claimed. Semantic Scholar's citation-context corpus surfaced competing findings from the same lineage: *"pilots missed over 10% of changes in FMS modes, and this rate substantially increased with competition for visual attention."*

The unvoted siblings of that claim, carrying the same paywall caveat: two peripheral-feedback implementations beat the current foveal Flight Mode Annunciator on detection rate and response time in a piloted simulator study; the paper's causal thesis is that foveal-only feedback structurally cannot support tracking system-induced change alongside a concurrent task; and it names the hazard class **high autonomy + low observability**. The authors' generalization beyond aviation is *asserted*, not tested.

---

## 10. Not established anywhere in this corpus

- **A general measured density ceiling.** Two separate reviews state it does not exist.
- **The EEMUA alarm-rate number.** Two secondary sources differ by 2×; unresolved (§4.1).
- **Whether diegetic presentation beats non-diegetic presentation of the same information.** Genuinely untested by the study usually cited for it, by that study's own admission.
- **Any measured design that *succeeds* at conveying uncertainty in a dashboard.** The TVCG paper reports none.
- **Transfer of EID beyond process control.** Named an open question by its own founders; aviation applications called "anecdotal."
- **Whether alarm-management rationalization discipline extends to non-alarm status indications.** The existence of a separate technical report for non-alarm notifications suggests it does not; this was the operative dissent in §4.
