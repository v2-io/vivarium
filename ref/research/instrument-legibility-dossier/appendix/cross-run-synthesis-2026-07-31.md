# Game / instrument-panel UX research — dossier

*Produced 2026-07-31. **`msc/`-grade: substrate, not claim canon.** Nothing here is a vivarium claim. Where a finding is strong enough to convict or support a segment, that promotion is separate work and is noted in §9.*

> **Read this before §1.** Two independent external audits (2026-07-31, `../AUDIT-2026-07-31-grok.md` §3.3 and `../AUDIT-2026-07-31-grok-2.md` A-1) both flagged this document as sitting in tension with `00-INDEX.md` §5's "makes no claims about vivarium, proposes no changes" fence — this file *does* argue a specific design verdict, and `00-INDEX.md` routes readers here as "closer to the evidence," which is true for the raw per-run material and **not** true for this file's own §1/§9/§12 design inferences, which are a further synthesis hop *beyond* 01–04, not a primary layer beneath them. Read the tier-by-tier breakdown added to §1(a) below before treating "close to the worst available choice" as a finding rather than an argument built from mixed-tier evidence.

## What this is

Joseph asked for the whole gamut of user-facing instrumentation and visualization design principles — first-principles perceptual work through modern game telemetry, with the framing that the interesting layer is *"orthogonal or adjacent to gameplay… traditional HCI/UX. Principled instrument-panels, for example"* — plus a specialized pass on what Bevy offers at primitive and library level.

The immediate consumer is `vivarium explore`'s chrome, whose attention failure is diagnosed in `#disc-explorer-human-chrome` and whose target design is the **status-chip strip** in that segment's FE(5).

**The short version: the research contradicts parts of that design, confirms others, and relocates the highest-leverage fix somewhere neither had considered.**

### Contents

| File | What |
|---|---|
| `README.md` (this) | Cross-run synthesis, contradictions, verification debts |
| `synthesis-perception-attention.md` | Human side — attention capture, clutter, crowding, contrast, typography |
| `synthesis-practice-safety-critical.md` | Practice side — HUD taxonomy, GUR empirics, avionics/nuclear/alarm-management, dashboards |
| `synthesis-bevy-0.18.md` | Bevy 0.18 capability answer, version-stamped, with ecosystem-crate ledger |
| `harvest/` | Added 2026-07-31 — the three raw per-run extraction pools (quote/URL/date/grade for every mined claim) behind the three synthesis files above; closer to the evidence than any of them |

### Epistemic grading used throughout

- **[T]** theoretically principled — derivable model behind it
- **[E]** empirically established — measured; population/context and effect size where known
- **[C]** craft consensus — widely practiced, thinly evidenced
- **[REV]** review/secondary — synthesizes others' data

A claim's *quote* is generally more trustworthy than the sentence introducing it, which is more trustworthy than any "design consequence" appended after it. This is not a stylistic preference; see §10.

---

## 1. What contradicts the proposed chip strip

`#disc-explorer-human-chrome` FE(5) specifies **CARVE** as *"Red / unmissable when N=0 and M>0."* Four independent lines argue against a red chip for that job — **but they are four different evidence tiers, not four replications of one finding, and combining them into "close to the worst available choice" oversteps what any one of them shows. Correction below the table states the honest version.**

**(a) Colour fails peripherally, and the failure is measured** — but under *expected* cues on a static field, not vivarium's case. Bartram, Ware & Calvert (2003), *IJHCS* 58:515–545 [E, verified against primary]:

| Encoding | Undetected @ 7° | Undetected @ 52° | Latency near → far |
|---|---|---|---|
| Colour | 5.5% | **24%** | 2.3 s → 4.6 s |
| Shape | — | — | 2.0 s → 4.4 s |
| Motion | ~0% | **~0%** | **~1.0 s, flat** |

*(Corrected 2026-07-31 from 6%/25%/<2% — see `01-perception-and-attention-fundamentals.md` §2.1 for the two-quotes-in-one-primary explanation.)* The operator's fovea is on the globe. That is the far-field condition — but Bartram's participants expected cues on a static field (§2 caveat below); this is an extension by analogy, not a direct measurement of vivarium's case.

**(b) Red-on-black specifically is a known failure case — conditional on APCA being the more accurate model.** WCAG 5.3:1 (passes AA), APCA Lc=40 (fails) [T, computed]. Both formulas are luminance-only by construction; red carries little luminance against dark terrain. APCA is not an adopted standard (00-INDEX debt 5) — this is a live, non-peer-reviewed contrast model's prediction, not a settled fact. Also note: green fails by the same logic Nikolic's green box does in (d), so this line does not specifically indict red over any other saturated hue.

**(c) The globe's own motion suppresses awareness of colour change — extended by analogy, not measured for this case.** Suchow & Alvarez (2011), *Current Biology* 21(2):140–143 — "motion silences awareness of visual change," measured for hue, luminance, size and shape; extended to orientation by a 2021 replication (N=30, η²p=0.783, ~80 ms cost). A colour flip on a chip beside a rotating planet is analogous to the silenced case, not the same experiment — nobody has measured a chip beside a large, slowly rotating, multi-hued sphere (00-INDEX gaps list).

**(d) There is a paper named after this exact failure — for a *green* outline box, at identity/abstract verification only.** Nikolic, Orr & Sarter (2004), *"Why Pilots Miss the Green Box: How Display Context Undermines Attention Capture"* [E — **identity/abstract verified only**, body text never read by anyone in this chain; retagged 2026-07-31, see 01 §2.5]. The failing artifact is literally a status chip — a green outline box around a mode annunciation on the primary flight display. Three separable degraders, each measured under concurrent load: colour similarity to background, **motion of background elements**, and eccentricity. Abrupt onset and flashing — the mechanism every HUD reaches for — does not reliably capture attention in real data-rich displays.

**Correction, 2026-07-31 (two independent external audits, both flagging this as the dossier's sharpest scope-fence violation):** the honest claim these four lines jointly support is narrower than "close to the worst available choice" — it is: *colour-only peripheral status indication is a poor bet under several independent, mostly-unrelated literatures; the vivarium-specific encoding (a chip beside a large rotating sphere) is unmeasured by any of them; and red is not specially convicted over other saturated colours — Bartram's colour condition tested red and green together and did not separate them, and (b)/(d) both implicate green too.* Treat "close to the worst available choice" as this document's own rhetorical framing, not a finding the evidence independently establishes.

### The deeper problem: an overlay is in the cost regime by construction

Fadden, Ververs & Wickens (1998), 18-experiment meta-analysis [E, **secondary-within-source** — these numbers come from a 2004 NASA survey reporting the 1998 study, not from the study itself; chase the original]: pooled overlay-vs-separate detection is statistically indistinguishable (p=.215). Split by **expectancy**, a reliable effect appears (Z=1.968, p<.026 — **the survey document itself misprints the sign as "p>.026"**; the flag travels with the number) : overlays *help* detection of events the operator expects and *cost* detection of events they don't.

`vivarium explore` exists to catch violations **nobody declared in advance**. That is the cost regime, by definition of the instrument's purpose. This is the single sharpest boundary condition in the corpus and it applies to the whole idea of chrome-over-globe, not to any particular chip.

---

## 2. What is not the answer either

Motion looks like the fix from §1(a). It mostly isn't:

- **Continuous motion is worse than static** for detection [E, d=.76–1.18]. A perpetually pulsing indicator spends salience continuously for no compounding benefit.
- **A chip embedded in a moving frame pays a cost even while itself motionless** [E, N=31]: pure-static 544 ms < mixed-static 580 ms < mixed-moving 623 ms < pure-moving 635 ms. Being still is necessary, not sufficient.
- **When two attention-getting elements compete, only one wins** — a new object fully suppressed an otherwise-capturing motion onset [E].
- **Bartram's own numbers do not transfer cleanly.** §8 of that paper names perceptual interference with other motions as unaddressed future work; participants *knew* cues were coming; background-motion interference appeared in their own Tetris condition. A rarely-firing freshness alarm is the unexpected-cue regime the study does not cover, and vivarium's background moves.
- **Onset capture is eliminated by concurrent load** [E, n=24–26]: single-task onset slope 42 vs 11 ms/item collapsed to 33 vs 25 (n.s.) under a concurrent task. Meanwhile a *sustained colour singleton* got **stronger** under load (45 vs 30 → 40 vs 12). Mechanism: transients dissipate while the operator is busy; a sustained state difference persists. **This is the one result that argues for a static encoding after all — under load, and in the fovea's vicinity.**

The honest state: colour-only is measurably bad peripherally, motion is better on a *static* field, sustained differences beat transients *under load*, and **nobody measured the case vivarium actually has** — a small chip beside a large, slowly rotating, multi-hued sphere. That is a probe-shaped gap, not a design answer.

---

## 3. What the research relocates: it's clutter headroom, not encoding choice

Two runs, separate corpora, separate scope decompositions, no shared sources, converged on the same answer to *"how much may a display say before it says nothing."*

**Rosenholtz et al. (2007), *Journal of Vision* — Feature Congestion** [T+E, the strongest-tier claim in the pool]. Clutter is **image-computable**: the volume of the local feature-covariance ellipsoid over colour, orientation and contrast-energy at multiple scales. It predicts the contrast a target needs to be found (**r = .93** across 20 map backgrounds) and search-time variance across natural backgrounds (r = .74–.83, ~55–69% of variance).

Feature Congestion is *defined* as exactly the alerting question: **how hard it would be to add a new item that reliably draws attention.**

Three consequences that matter here:

1. **This is runnable.** It takes a rendered frame and returns a number. vivarium already commits capture PNGs to the repo as evidence. Scoring them converts *"is the chrome self-announcing"* from a judgment call into a measurement — which is the shape `#norm-probes-before-claims` demands.
2. **Colour variability is a first-order, separable cost.** With edge density held constant, desaturating the same maps cut search RT from 772 ms → 619 ms (gray) → 552 ms (red), both p<.001 — a 20–29% reduction from colour variability alone. Edge-count metrics miss this entirely. **The number of distinct hues in the globe rendering is a direct cost on chip discoverability**, independent of how many chips exist.
3. **But monochrome is not automatically uncluttered** — and this kills the obvious fix. Quoted directly: *"it may be misleading to label a display as 'uncluttered' simply because it is monochrome; one might not be able to add a target that draws attention because of its colour."* Desaturating the globe to make chips pop would buy edge legibility while spending the exact channel the chip needs.

**The ceiling is local and computable, not panel-wide and fixed.** There is no "N chips" number, and NUREG-0700's 50%/25% packing cap is codified 1986 handbook practice, not a measured threshold.

Corroborating from the other direction, and the cleanest statement of the principle: **Caroux (2022)**, four experiments [E, N=40/36/41+]. HUD *size* (9% vs 15% of screen) and *colour* (red vs blue) produced **near-zero effects** (η²p < .01). Removing task-**relevant** elements produced huge ones (η²p = .41 and .67); removing task-**irrelevant** elements left performance near full-HUD levels. And 78% of experienced players *consciously noticed* the colour change while no performance or affect measure moved.

**Make it bigger / make it redder is measured to be worth approximately nothing.** Relevance is the variable.

---

## 4. The strongest single finding: misleading beats absent, and the redesign raises the stakes

Three literatures that do not cite each other arrive at the same principle:

- **Aviation** — FAA AC 25-11B hazard tables rank a *misleading* attitude/engine display **Catastrophic** and *loss* of the same display **Major-Hazardous**. The only thing that demotes "misleading" is a monitor that auto-detects the fault and annunciates it.
- **Nuclear** — NUREG-0700 Rev. 3 requires sensor failures to produce *"distinct display changes"* and imposes a **four-state validity taxonomy**: valid / invalid / **unvalidated** / numerical estimate. It explicitly refuses to collapse "known bad" into "not checkable." Guideline 1.1-23 further requires a frozen display to carry an obvious reminder, and 2.5.4-7 requires the operator be told when a significant *underlying* change has occurred that the frozen display isn't showing — which is precisely vivarium's stale-cache case, stated by a regulator as mandatory.
- **Visualization** — TVCG 2019 field studies found the dashboard *form itself* manufactures unearned credibility: people read dashboard data as trustworthy *because* the visual organization signals objectivity, independent of data quality.

That third leg is the uncomfortable one. **A clean chip strip will make a stale value look more trustworthy than the current wall of text does.** The redesign raises the stakes on the freshness alarm rather than lowering them.

### And EID cannot help with staleness — a structural blind spot

Ecological Interface Design's mechanism is making *constraint violations* perceptually self-evident: a bad sensor reading becomes visible because it breaks a physical law the display makes legible. That works for **wrong-and-internally-inconsistent** data.

A stale cache is **wrong-but-internally-consistent**. A world that stopped updating three minutes ago is still perfectly lawful; nothing in it contradicts anything else in it. EID's machinery is blind to staleness by construction.

**Consequence:** no amount of physics-legibility improvement substitutes for an explicit clock-based annunciator. EID answers *"how do I show a sensor is lying,"* not *"how do I show a sensor stopped talking."*

Mechanisms that the corpus does say work, for announcing unreliability:

- **Co-location.** AC 25-11B §6.2.1.7: failure flags *"should be presented in the location of the information they reference or replace."* Not a caveat elsewhere.
- **Not a text change.** AC 25-11B §5.11.3.4, verbatim: *"a text change by itself should not be used as an attention-getting cue."* Independently corroborated by games eye-tracking — transient/non-permanent HUD elements drew very few fixations versus permanent ones [E, n=15, small].
- **Two sensory channels, in binding law.** 14 CFR 25.1322(c)(2): Warning and Caution alerts must provide attention-getting cues *"through at least two different senses."* Single-channel visual is non-compliant for the top two tiers. (Amdt. 25-131, 2010 — **not** "decades ago"; that gloss was killed.)
- **Graded rather than binary** [E, one 1988 lab study]: likelihood-coded alerting did *not* measurably increase attentional load, and the confidence level entered operators' decisions rather than acting only as a trigger.
- **Legibility degradation** — the datum announcing its own unreliability by becoming harder to read (blur or desaturation proportional to uncertainty), rather than a separate flag doing the announcing. Cost: degrades point-lookup precision.

---

## 5. Where the corpus checks vivarium's own premises

Joseph asked to be surprised rather than confirmed. These are the findings that cut against the project's stated positions.

**`#norm-no-depiction-without-referent` FE(2) holds that a trained eye is the project's fastest detector of missing physics.** The corpus supplies a hard boundary condition [E, n=16, single-event existence proof]: a mountain adjacent to a checkpoint was deleted from a terrain database; subjects were experienced, actively navigating, holding a paper map, and **explicitly told to report display/map inconsistencies**. **Zero of sixteen explicitly noticed.** At most two may have implicitly noticed, and both attributed the miss to concurrent search load.

Cite that as *"this happened once, dramatically"* — not as a rate. But it is the sharpest available check on the assumption that the eye-instrument is self-correcting, and it says the premise holds **only when the eye is not concurrently loaded**.

**A related result bears on `vivarium watch` versus `vivarium explore`:** passive replay-watchers were markedly more complacent than active navigators (P(A) 0.76 vs 0.84; false alarms 32% vs 15%). A watcher may be a structurally worse detector than an operator actively steering, independent of any HUD design question. `#disc-explorer-instrument-parity` assumes parity of a different kind; this is a distinct axis.

**Cueing produces a criterion shift with no sensitivity gain.** A 100%-reliable "look here" cue over live 3D terrain improved detection of the cued target but *significantly reduced* detection of an uncued, higher-priority anomaly (F(1,12)=6.72, p=0.02). In signal-detection terms sensitivity **dropped**: P(A) 0.88 → 0.64. On the one trial where the cue pointed at a non-target, 63% of operators reported the distractor as a target anyway. Disclosing that the cue was only 75% reliable produced only partial recalibration — sustained false-alarm rate stayed at 45.5%.

**Whatever a chip asserts will be believed over the raw scene, and disclosing the chip's imperfect reliability does not restore calibration.** This is a direct, measured argument for the project's existing instinct that a view must not manufacture content — extended to an area the norm does not currently cover: not just depiction, but *assertion*.

**Also worth knowing:** increasing scene realism did not increase operators' trust in the simulation but *did* increase reliance on the overlay and reduce processing of the raw scene. A prettier globe means more deference to the chrome laid over it.

**And the minimalism instinct is a chosen side, not the uncontested position.** Borst, Flach & Ellerbroek (2015) argue that as automation increases the correct move is to show *more* — the operator must monitor the machine's rationale, not just its output. That sits in direct tension with Few's saliency budget, ISA-101's "gray is good," and NUREG's "minimize density for critical info," and none of these sources cite each other. vivarium's instinct (sparse always-on surface, detail on demand) is defensible but is a live disagreement between two safety-critical traditions, not settled doctrine.

---

## 6. What the research confirms

- **The dump/chrome split is a documented discipline, not an aesthetic preference.** ISA-TR18.2.8-2023 *"Guidelines for Non-Alarm Notifications"* exists specifically to keep informational messages from being rendered as alarms. ISA-TR18.2.5-2022 supplies the vocabulary: alarm rates, standing alarms, response times, **alarm floods**, **stale alarms**.
- **Fixed-position annunciation over reflowing text has direct regulatory endorsement.** AC 25-11B §5.11.2 requires fixed-location annunciation with anything needing immediate awareness in the forward field of view.
- **Moving detail to on-demand is licensed under named conditions.** AC 25-11B §6.3.3.1 gives a **six-part admissibility test** for hiding information part-time: not continuously needed; auto-displays when abnormal; manually selectable without interference; fails safe; doesn't create unacceptable clutter on arrival; **must be accompanied by alerting if its arrival isn't self-evident.** This is the closest thing in the corpus to a principled rule for what may move to a capture dump.
- **Mode indication must be always-present.** NUREG-0700 9.4-1/9.4-5: *"the user should not have to query the system to determine the current mode."* Sarter & Woods (1995) frame mode awareness as a *continuous attentional* task and name **scattered** indication — "numerous mode indications distributed over multiple displays" — as its own failure mechanism, independent of whether the content is correct.
- **The canonical game-visualization taxonomy independently arrives at the same split.** Bowman/Elmqvist/Jankun-Kelly's Temporal Usage dimension separates always-visible continuous status from call-up intermittent display, on distraction grounds.
- **For precision values, a number beats a bar or gauge** [E]: ~76% successful-escape rate for a numeric readout vs ~46% for a bar. Pair with Few's "one separate qualitative cue" — number for precision, colour for is-this-OK.
- **Encode spatial information spatially** [E]: an in-world navigation line beat an abstract compass panel by ~21.6 s vs ~56.4 s to waypoint, ~2% vs ~42% incomplete trials.
- **Reflowing text is in the maximally-distracting motion category.** Distraction is governed by motion *type*, not frequency (type F(7,84)=86.89 vs frequency F(1,12)=40.18): travelling motion — glyphs traversing degrees of visual angle — ranks worst, ahead of zoom, oscillation and flicker. The current dump applies the most attention-capturing motion class to the *least* critical content.

---

## 7. Layout geometry you can design against

The crowding results are the most directly actionable numbers in the corpus, and they constrain chip *layout*, not just chip styling.

- **Bouma's law** [T+E, replicated since 1970]: critical spacing ≈ **0.4–0.5× eccentricity**. A chip at 10° needs ~4–5° of clear space around it to be readable — regardless of its own size or contrast.
- **Crowding is anisotropic, ~2:1 radial vs tangential.** A **vertical stack of chips in a screen corner** is radially aligned relative to a center-screen fixation and therefore crowds itself roughly twice as hard as the same chips arranged along the perpendicular arc. The outermost chip is the worst offender against its inward neighbours, not the reverse.
- **Size buys little peripherally; isolation buys a lot.** A target at 10° needs only ~4× foveal size to be equally resolvable — the popular "peripheral vision is blurry" demo is wrong by roughly a factor of four, and the review names HCI and computer graphics as fields carrying the error forward. Acuity was never the binding constraint; crowding is.
- **Peripheral alarms need scaling in both size *and* exposure time** to equate detectability. Matching pixel dimensions across screen positions is not matching visibility.
- **Foveal load causes genuine tunnel vision; auditory load does not.** The operator's own globe-scanning is precisely what makes a peripheral chip disproportionately hard to detect — a cognitive load alone would not do this.
- **Fixation is not perception.** In an on-road AR study, 87.9% of missed events had >100 ms fixation with no response, and the misses were concentrated in the **central** field of view — refuting the researchers' own pre-registered hypothesis. "Put the alarm where the eye already is" is not sufficient.
- **Overlay text cost is monotonic in volume**: detection odds 3.04× (no text) / 4.45× (two lines) / 5.42× (three lines) relative to one line. The real measured cliff is *no panel vs multi-line panel*.
- **Hick's law is a non-issue here.** Real-task slopes are 32/8/4 ms/bit; going from 2 to 16 on-screen items costs ~12–96 ms. Adding chips does not cost decision latency. If chips cost anything it is via clutter or crowding, not choice-reaction time. (The same CHI 2020 reanalysis notes that "categorize to reduce choices" is not merely unsupported by Hick's law but mathematically contradicted by it.)

---

## 8. Bevy 0.18 — the capability answer

Full detail and the ecosystem ledger in `synthesis-bevy-0.18.md`. Version-stamped against vendored crate source on this machine (`bevy_*-0.18.1`), not blog prose.

### The finding that changes the fix

`bevy_text-0.18.1/src/text.rs:744` — `detect_text_needs_rerender` sets `needs_rerender` whenever `Changed<Text>` fires. The field's own doc comment is candid:

> "This field currently causes UI to 'remeasure' text, even if the actual changes are non-structural and can be handled by only rerendering and not remeasuring."

**Any write to a `Text` component forces a full Taffy remeasure**, whether or not the content differs. The explorer reassigns a multi-thousand-character `String` every frame, so it pays a complete remeasure every frame unconditionally. A one-character digit change would cost the same.

**This reframes the repair from "stop the box changing size" to "stop writing to the block unless a value actually changed."** Split volatile values into their own small text entities, gate writes with `set_if_neq`. Most frames then touch zero text entities. It is cheap, orthogonal to the chip redesign, and would help even if the layout never changed.

### Capability summary

| Need | Status in 0.18 |
|---|---|
| Fixed-position non-reflowing layout | ✅ first-party |
| Tabular figures (non-jittering numerals) | ✅ `TextFont::font_features` + `FontFeatureTag::TABULAR_FIGURES` — **but needs a shipped `.otf`**; Bevy's default font is not guaranteed to carry the tables |
| Pulse/scale a chip without disturbing siblings | ✅ `UiTransform` is layout-free; `Outline` explicitly takes no layout space |
| Autonomous pulse/blink | ⚠️ `TryStableInterpolate` is a **math helper, not an animator** — use `bevy_easings` (`Loop`/`PingPong`/`Discrete`) |
| Contrast over arbitrary imagery | ✅ `TextShadow`, `Text2dShadow`, `TextBackgroundColor`, `Outline`, `BoxShadow` |
| Glyph-edge text outline | ⛔ none first-party (issue #17076 open; PR #23369 in review) |
| Crisp text at Retina | ✅ automatic; `FontSmoothing::None` also **pixel-snaps** glyph geometry (it is not a no-op — that harvest claim was a `main`-vs-release error) |
| Subpixel/LCD antialiasing | ⛔ not in the API at all |
| World-point → screen anchoring | ✅ `Camera::world_to_viewport`; `bevy_ui_anchor` **pinned to 0.11.0** (0.12.0 needs Bevy 0.19) |
| Off-screen indicator | Hand-roll — clamp + `atan2`; `world_to_viewport` returns `Err(PastNearPlane)` which is itself a useful off-screen signal |
| Toast that never clips off-screen | ✅ `bevy_ui_widgets::popover::Popover` — shipped edge-avoiding auto-placement, nobody listed it |
| Screenshot path | ✅ the current implementation is already on the documented first-party path |

**Two disqualifications with sourced reasons.** `bevy_feathers` self-disclaims product use in three independently-fetched primary documents. And **`bevy_egui` draws outside Bevy's render graph** — its UI was silently absent from screenshots (Bevy issue #16689, `P-Regression`, no app-level workaround). For a project that commits capture PNGs as evidence, an overlay that may not appear in the captured frame is disqualifying rather than inconvenient.

**Dead:** `iyes_perf_ui`, `bevy_screen_diagnostics`, `bevy_mod_billboard` — all 2+ releases behind, effectively abandoned.

**Version fragility:** 0.18 is cosmic-text; **0.19 replaces it with Parley wholesale.** Typography work here sits on a stack the engine already swapped.

**A caution about the project's own verification habit:** Bevy's screenshot-based CI failed to catch the `FontSmoothing` regression before it shipped to `main`. A passing screenshot diff is evidence of gross visual regression, not of subtle rendering-mode defects.

---

## 9. Verification debts — check before any of this enters a segment

Ranked. None are known-wrong; all have sourcing shaped like the sourcing that produced fabrication elsewhere in this corpus.

1. **EEMUA 191 alarm-rate ceiling.** Two secondary sources disagree by a factor of two — ~12/hr long-term average (with an 18/hr action limit) versus ≤1 per 10 min (=6/hr) steady state. Neither is the primary standard. This is precisely the number a density claim would want to cite.
2. **The Gabbard AR contrast numbers** — "billboard beats colour-tuning," 7:1 video-see-through / 1.6:1 optical-see-through floors, "1px outline is enough." Attributed in the harvest to a **2024 review that was never fetched**, not to the dissertation the URL points at. Check Gabbard, Swan & Hix (*Presence* 15(1):16–32, 2006) and Gabbard et al. (IEEE VR 2007, 35–42) directly.
3. **Sarter & Woods' 65% / 15% aborted-takeoff figures.** Entire extraction is secondary-summary-derived; even which of their papers it comes from is unconfirmed. Most quotable number in that subsection, least verified.
4. **The ~8 percentage-point miss-rate increase per +10° eccentricity** — flagged by the extraction itself as coming from downstream citing literature, not the primary.
5. **APCA's status.** It is **not** an adopted W3C standard and had not cleared the WCAG 3 subgroup's own peer-review precondition. Its creator maintains the "independent peer reviews" bibliography, which is overwhelmingly blog posts and conference talks — and one of the "independent reviews" is the same self-published article the contrast numbers in §1(b) come from. Treat APCA as a credible alternative under active contestation, not the settled successor to WCAG.
6. **Two sources are burned.** Healey's *"Perception in Visualization"* page went 5/5 refuted — do not cite it from this corpus. Wolfe's *Guided Search 6.0* went 3/5; its surviving material is discounted accordingly, and its "capacity = 5" figure is a **simulation parameter**, not a measured human limit.

---

## 10. Dead ends and gaps, stated plainly

- **Text reflow / in-place length-change as its own measured variable does not exist in this corpus.** This is the literal defect being fixed, and nobody has run the experiment. The case against reflow is built by extension from motion-type findings — state it as inference, not measurement.
- **No general measured density ceiling exists.** Two separate reviews say so explicitly. NUREG-0700's 50%/25% is inherited handbook practice.
- **The canonical academic game-visualization framework has no axis for urgency, alerting, criticality, or reliability** — verified by full-text term search, zero hits. A 2024 CHI PLAY taxonomy's full 49-item reference list contains **no** "Beyond the HUD," no games-user-research, no human-factors/avionics/alarm-management source. These are genuinely three mutually non-citing literatures, confirmed by bibliography resolution rather than assertion. That is itself the justification for synthesizing across them.
- **Not covered by the fetched sources:** Fitts's/steering-law transfer to gamepad or gaze input; Miller 7±2 vs Cowan 4±1; Pousman & Stasko's ambient-display taxonomy (a different Stasko paper was fetched); 10-foot-UI guidance; tabular-vs-proportional figure research; digit jitter.
- **Game HUD craft lore takes measured hits from inside its own literature.** Diegetic superiority: refuted (no display class won universally). Minimal-HUD-improves-immersion: non-replicated, and the original result was expertise-conditional at p≈.051–.058. Salience fixes: near-zero effects. Bars-over-numbers: recorded by the field's own foundational survey as designer *belief* with no citation and no measurement.

---

## 11. How this was produced, and what that implies

Three `deep-research` workflow runs (scope → parallel search → fetch/extract → 3-vote adversarial verification → synthesis), **stopped mid-verification** at Joseph's direction after ~244 agents. Scope, search and extraction had completed in all three; 49 claims had been fully adjudicated. The journals were harvested, and synthesis was done in-session plus three Sonnet passes over the unverified pools.

**The adjudication outcome is itself a finding worth recording:**

| Run | Refuted | Contested | Survived |
|---|---|---|---|
| Human perception | 8 | 1 | 5 |
| Practice / safety-critical | 1 | 5 | 4 |
| Bevy | 8 | 1 | 16 |

**17 of 49 refuted — 35%.** And the failure mode was consistent across all three runs, identified independently by two synthesizers who did not see each other's work:

> **The verbatim quotes were almost always accurate. The defect lived in the headline sentence and in the appended "design consequence."**

Recurring shapes: a number reassigned from the study that measured it to the finding it was rhetorically attached to; a source's own hedge ("*may* offer no assistance," gated to novel displays) silently converted to a categorical rule; the extraction agent's own inference presented in the same voice as the measured finding; and — dominant in the Bevy run — `main`-branch or PR-body content presented as release fact.

Two claims about primary sources were **fabricated outright**: a "70%" statistic that appears nowhere in the paper it was attributed to, and a categorical about peripheral colour/shape detection that the cited source explicitly declines.

**Practical implication for anyone reading LLM-mediated research, including this dossier:** trust the quote over the framing sentence, and the framing sentence over the design conclusion. Where a source has a refuted sibling, discount its survivors.

One error of the coordinating session's own, recorded because it is the same shape: per-run refutation counts were stated twice from memory and were wrong both times, in the same direction. The aggregate figure, which came from a `grep`, was right. Same session, same lesson as `bin/provenance`: the values copied from output were correct; the one generated was not.

---

## 12. What could become segment material

Not promoted here. Candidates, with what each would need:

- **`#disc-explorer-human-chrome` FE(5) may warrant revision** — colour-only peripheral status indication is a poor bet under several independent literatures, though (per the §1 correction) not specifically a red-vs-other-colour finding, and the vivarium-specific geometry is unmeasured. A probe measuring detection of the CARVE state under realistic globe-scanning would convict either way, and is the actual evidence this candidate needs before revision.
- **A Feature Congestion probe** over committed capture frames (§3) — the one genuinely computable, falsifiable instrument the corpus offers, and the closest fit to `#norm-probes-before-claims` of anything here.
- **`#norm-no-depiction-without-referent` could extend from depiction to assertion** (§5) — the cueing/criterion-shift results say an overlay that *asserts* is believed over the raw scene, and disclosure does not recalibrate. That is the same harm the norm exists to prevent, arriving through a channel the norm does not currently name.
- **The staleness annunciator is architecturally forced** (§4) — EID's blind spot means no physics-legibility work substitutes for an explicit clock-based alarm. Worth stating in a segment because it forecloses a tempting alternative.
- **The `Text`-write remeasure fix** (§8) — a code change with a measurable before/after, independent of any claim.
