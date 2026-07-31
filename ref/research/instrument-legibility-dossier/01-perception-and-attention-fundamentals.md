# 01 — Perception and attention fundamentals

*Half-life: decades. Domain-independent. How a human eye acquires a display.*

## Status of this document

**The writer of this document read no primary sources.** Chain: primary → extraction agent → synthesis agent → here. Every line below is at two or three removes, including those carrying page numbers, F-statistics and DOIs.

Each entry is tagged with what actually happened to it:

- **`[VERIFIED 3/3]`** — survived three independent adversarial verification passes, each attempting refutation against the cited primary. Strongest tier available here.
- **`[CONTESTED nR/nK]`** — mixed verdict; the split is stated.
- **`[REFUTED 3/3]`** — checked and killed. **Retained deliberately.** A later reader needs to know what was tested and failed, or the same claim gets re-derived from the same plausible-looking source.
- **`[UNVOTED]`** — extracted from a fetched source with a verbatim quote, never adversarially checked. Roughly a third of comparable claims did not survive when checked. These are leads.

Verbatim quotes and URLs for every entry live in [`appendix/synthesis-perception-attention.md`](appendix/synthesis-perception-attention.md). **Read the appendix before acting on anything here.**

Two sources in this area are burned: Healey's *"Perception in Visualization"* course page went 5-for-5 refuted — do not cite it from this corpus. Wolfe's *Guided Search 6.0* went 3-of-5 refuted; its survivors are discounted below where they appear.

---

## 1. Guidance versus discrimination — two different thresholds

**`[VERIFIED 3/3]`** The threshold for a feature difference to **guide** attention is roughly an order of magnitude coarser than the threshold to **discriminate** it once attended. A 0.5° tilt from vertical is easily detected by early vision but does not guide attention at all; roughly 10–15° of orientation difference is needed before an attention-guiding priority signal is generated (stimulus-dependent). Wolfe states the same holds for colour. Guidance runs on a deliberately coarse representation. Wolfe adds that categorical uniqueness beats mere extremity — a target that is the only steep line is easier to find than one that is merely the steepest, at identical angular separations.

*Wolfe, J.M. (2021), "Guided Search 6.0," Psychon. Bull. Rev. 28:1060–1092, p.1065. Verified by three passes, each extracting the PDF locally.*

**Consequence for reading the rest of this document:** *"discriminable on inspection"* and *"self-announcing"* are separate, quantitatively different bars. Several claims below conflate them; where they do, it is flagged.

---

## 2. What is measured about attention capture

### 2.1 Static colour and shape fail as peripheral alerts; motion does not

**`[VERIFIED 3/3]`** Across a 7°-to-52° eccentricity range:

| Encoding | Undetected, near (7°) | Undetected, far (52°) | Detection latency |
|---|---|---|---|
| Colour | 6% | **25%** | 2.3 s near → 4.6 s far |
| Shape | — | — | 2.0 s near → 4.4 s far |
| Motion | <2% | **<2%** | **~1.0 s, location-independent** |

*Bartram, Ware & Calvert (2003), "Moticons: detection, distraction and task," Int. J. Human-Computer Studies 58:515–545. Verified against the PDF by three independent passes (WebFetch could not parse it; all three ran `pdftotext`).*

**Scope limit carried from the source, not optional:** the degree glyph is dropped by `pdftotext`; "7 to 52" is read as degrees by parallel phrasing elsewhere in the methods.

### 2.2 Distraction is governed by motion *type*, not frequency

**`[VERIFIED 3/3]`** Motion type F(7,84)=86.89, p<0.0001 dominated frequency F(1,12)=40.18 in magnitude. Ranked most-to-least distracting: **travelling motions** (object leaves its origin and traverses several degrees — scrolling tickertape, banners) by far the most; **zoom** next; **slow blink** and **slow linear oscillation** least. The paper's recommended alternative is an **anchored small oscillation** that signals without demanding tracking. Mechanism named: travelling motion adds a cognitive tracking act on top of detection; zoom is attributed to sudden perceptual onset (citing Hillstrom & Yantis 1994).

*Same source as 2.1.*

### 2.3 Designable motion parameters — but the sufficiency claim was contested

**`[CONTESTED 1R/2K]`** Amplitude ≈1° is highly detectable even in pronounced peripheral conditions; amplitude had **no significant effect on detection accuracy** (the authors' own hypothesis that it would was disconfirmed), though sub-1° amplitudes may cost response time (low-amplitude far-field detection nearly doubled, 0.92 s → 1.78 s, in Experiment 1B only). Frequencies across the whole tested 1–3 Hz range were effective with no perceptible difference. Smooth vs "jumpy" cues performed equally. 20–30 frames/s over a few seconds suffices to elicit a single continuous motion percept.

**The dissent, which matters more than the numbers:** the numbers survive verbatim; the claim that they are *sufficient to build against* does not. From the paper's own text — §6.2.1 reports background-motion interference in the Tetris condition; §8 names perceptual interference with other motions as **unaddressed future work**; p.541 records that participants **were expecting cues**; §4.4 p.527 flags the 1A/1B amplitude inconsistency as "puzzling" and unresolved.

*Same source as 2.1. The dissenting verifier extracted the full 1493-line text rather than relying on the guidelines box.*

### 2.4 Onset capture is defeated by concurrent load; sustained colour difference is not

**`[UNVOTED]`** Single-task search slope for an onset distractor: 42 ms/item (vs 11 ms/item when it was the target) — strong capture. Under a concurrent auditory 1-back task this collapsed to 33 vs 25 ms/item (not reliably different). Meanwhile a **colour singleton** distractor, weakly prioritized single-task (45 vs 30 ms/item), became **strongly** prioritized dual-task (40 vs 12 ms/item). Mechanism offered: onsets are transient — if a secondary task delays search initiation the transient has already dissipated (intercept differences >130 ms observed); a colour singleton stays distinct for the whole search.

*Boot, Brockmole & Simons (2005), primary, n=24–26, between-subjects, small.*

**Note the tension with 2.1**, which is not resolved anywhere in this corpus: 2.1 measures colour failing at eccentricity; 2.4 measures colour strengthening under cognitive load. Different paradigms, different failure axes, no source reconciles them.

### 2.5 Abrupt onset does not reliably capture in real data-rich displays

**`[VERIFIED 3/3]`** Flashing and abrupt onset — the canonical lab-validated capture mechanism, and the one most dashboards reach for — does **not** reliably capture attention in real operational displays.

**`[VERIFIED 3/3]`** Three separable display-context properties each measurably degrade detection of an abrupt-onset signal: (a) colour similarity between onset target and background, (b) **motion of background elements**, (c) increasing eccentricity from current fixation. Design crossed 5 backgrounds against 2 eccentricities with participants under an externally paced concurrent visual task — degradation established under *loaded*, not idle, viewing.

*Nikolic, Orr & Sarter (2004), "Why Pilots Miss the Green Box: How Display Context Undermines Attention Capture," Int. J. Aviation Psychology 14(1). Publisher returns 403; identity confirmed independently via Crossref, OpenAlex and Semantic Scholar by three separate passes. **The body text was not read by anyone in this chain** — this is abstract-level plus registry metadata.*

---

## 3. Clutter as a computed quantity

**`[UNVOTED]`, and the highest-tier unvoted claim in this corpus.** Visual clutter is image-computable. Three algorithmic measures — **Feature Congestion**, **Subband Entropy**, **Edge Density** — take an arbitrary raster image and return a clutter score with no knowledge of display semantics.

Reported correlations: contrast a target needs to reach 75%-correct detection correlated **r = .93** with Feature Congestion (4 trained observers, 20 map backgrounds), better than Subband Entropy (r = .68) or Edge Density (r = .83). Across natural backgrounds the three together explain roughly **55–69%** of variance in mean search time (log RT correlations .74–.83, 6 subjects, 3,648 trials).

Feature Congestion is *defined* as: how hard it would be to add a new item that reliably draws attention.

*Rosenholtz et al. (2007), Journal of Vision. Primary. Full text paywalled in one run — a second run reached the correlations; the two runs' figures agree, which is weak independent corroboration but not verification.*

**`[UNVOTED]`** Colour variability costs search time independently of element count. With edge density held statistically constant across original/gray/red versions of the same maps (12.5–13.0% pixels, n.s.), desaturating cut target-present RT from 772 ms (original) to 619 ms (gray, t(23)=8.3) and 552 ms (red, t(23)=11.8), both p<.001 — a 20–29% reduction from colour variability alone. Edge Density predicted no difference; Subband Entropy predicted the *wrong direction* for the red condition.

**`[UNVOTED]`** Monochrome is not automatically uncluttered. Quoted from the source: *"it may be misleading to label a display as 'uncluttered' simply because it is monochrome; one might not be able to add a target that draws attention because of its colour."* Because the measure is fraction of *available* feature space consumed, a desaturated display can score low-clutter while having nothing left in the colour channel.

**`[UNVOTED]`** Background clutter cost survives top-down knowledge. Even where targets appeared in predictable locations designed to let observers ignore the background, messier backgrounds still slowed search, tracking the Feature Congestion ordering (empty 3.4, clean 4.3, messy 6.1). *Secondary-within-source, citing Wolfe et al. 2002.*

**`[UNVOTED]`** Ergonomics has **no established quantitative density ceiling** — stated as an open problem by a 2015 *Human Factors* clutter review. The same review defines clutter operationally: a display is cluttered only if it *measurably degrades performance*, and no single metric is adequate alone.

---

## 4. Crowding and peripheral geometry

These are the most directly designable numbers in this document. All **`[UNVOTED]`**, from Rosenholtz (2016), *Annual Review of Vision Science* — a review, so [REV] tier, reporting replicated primaries.

- **Bouma's law.** Critical center-to-center spacing at which a neighbour destroys recognition ≈ **0.4–0.5× the target's eccentricity**. Replicated across letters, bars, faces, coloured circles and real-world objects since 1970. A target at 10° eccentricity needs ~4–5° of clear space around it, regardless of its own size or contrast.
- **Crowding is anisotropic, two ways.** (a) Radial–tangential: critical spacing is ~2× larger for a flanker aligned *radially* with the target than for one placed tangentially. (b) Inward–outward: at equal spacing, a *more*-eccentric flanker interferes more than a more-central one.
- **Size buys little; isolation buys a lot.** Cortical-magnification scaling means a target at 10° needs only ~4× its foveal size to be equally resolvable — and foveal acuity is far above task requirements, so 4× of it is still small. The widely reproduced "peripheral vision is blurry" demo is quantitatively wrong by roughly a factor of four, and the review names HCI, human factors and computer graphics as fields carrying the error forward. *(The exaggeration factor is the review author's own estimate, not an independent measurement.)*
- **Enlarging a peripheral readout is not the lever.** Acuity was never the binding constraint; crowding is.

**`[UNVOTED]`** Concurrent **foveal** visual load produces genuine tunnel vision, with the deficit growing with eccentricity. Concurrent **auditory** working-memory load degrades peripheral sensitivity uniformly *without* shrinking the field. The authors flag that most prior "useful field of view" literature did not control for eccentricity-dependent spatial resolution, so older tunnel-vision numbers may conflate a fixed retinal property with attentional narrowing. *(Journal of Vision 16(2):7, 2016.)*

**`[UNVOTED]`** To equate detectability across the visual field, peripheral stimuli must be scaled up in **both size and exposure time**. Matching pixel dimensions across screen positions is not matching visibility.

---

## 5. Motion in a moving scene

**`[UNVOTED]`, N=30, peer-reviewed.** Background/distractor motion delays detection of a feature change in a concurrently monitored item: mean RT to detect a 30° orientation change was 439 ms (SD 58) on a stationary display vs 519 ms (SD 69) on a moving display — F(1,27)=97.52, p<0.001, η²p=0.783.

**`[UNVOTED]`, N=31.** Motion of the *target item itself* is a second, additive cost on top of surrounding motion: pure-static 544 ms < mixed-static 580 ms < mixed-moving 623 ms < pure-moving 635 ms, F(3,84)=25.53, p<0.001.

**`[UNVOTED]`** Density and motion interact multiplicatively: static search slope 9.7 ms/item (2–8 items), F(6,162)=23.24, p<0.001, with the motion penalty growing at larger set sizes rather than staying constant.

**Boundary condition stated by the authors themselves, and it must travel with the three above:** this study measured **response time only** — no target-absent trials, no accuracy or miss data. It establishes a latency cost, not a miss rate. Its own applied recommendation is workstation-level (limit the number of monitored screens), not pixel-level.

**`[UNVOTED]`** The proposed mechanism is **motion silencing** — feature changes on moving objects are perceptually attenuated — citing Suchow & Alvarez (2011), *Current Biology* 21(2):140–143, which measured it for hue, luminance, size and shape; this study extends it to orientation.

**`[UNVOTED]`** Motion **onset** beats offset, static and continuous for detection; a **new object** beats and fully suppresses motion onset; **continuous** motion is worse than static (d=.76–1.18 less accurate). When a new object was present, the identical animated motion-onset stimulus that had captured attention on trials without one **failed to capture at all**. *(n=12/experiment, two experiments, small-n flagged by the authors as typical for the paradigm.)*

**`[UNVOTED]`, secondary-within-source.** Capture depends on frame rate and per-frame displacement, not "motion" as a category: *jerky* (8–17 Hz) redraw captures attention, *smooth* (33–100 Hz) does not; the active ingredient is a single abrupt displacement of 0.26°–1.05°, which captures whether or not motion continues afterward. Flicker alone did not. *(Citing Sunny & von Mühlenen 2011/2014. The citing paper's own limitations section declines the strict definition of capture because its own Experiment 1 effect was context-dependent — the authors downgrade their own headline finding.)*

---

## 6. Overlay over a moving scene

All **`[UNVOTED]`** and all **secondary-within-source** — a 2004 NASA survey reporting others' numbers. The originals were not fetched. Treat as pointers.

- **Overlay cost for unexpected events, 18-experiment meta-analysis.** Pooled, overlay vs separate-display detection was statistically indistinguishable (p=.215). Split by **expectancy**, a reliable effect appears (Z=1.968, p<.026 — **the survey document itself misprints the sign as "p>.026"**; the flag travels with the number): overlays *help* detection of events the operator expects and *cost* detection of events they don't. *Chase Fadden, Ververs & Wickens 1998.*
- **Runway-incursion detection.** Time to initiate a go-around: 5.5–6.7 s head-down vs 7.2–9.1 s head-up, worst for non-conformal symbology. The same study found overlay *better* for the routine tracking task (30% less flightpath deviation, conformal case). *Chase Wickens & Long 1995.*
- **Screen-fixed overlay performed worse than no instrument at all** on primary tracking: mean path RMSE 70 ft no gauge, 73.5 ft superimposed screen-fixed, ~64 ft scene-linked. Proposed mechanism, which the authors call speculative: static overlay and moving scene form separate perceptual groupings, and attention divides poorly across groupings. *(n=14; chase Levy, Foyle & McCann 1998.)*
- **Density, not brightness, is the detection killer.** High-clutter symbology significantly reduced event detection in both overlaid and separate-display conditions. "Lowlighting" (dimming, not removing) task-irrelevant symbology improved far-domain traffic detection but did **not** improve detection of commanded changes — a selective, partial recovery. *(Chase Ververs & Wickens 1998 Exp. 2.)*
- **Symbology intensity did not disrupt environmental scanning at any tested level.** Contrast ratios 1.17:1 to 1.6:1 drove symbology detection speed but never cost far-domain scanning; the authors state none of the three intensities *"provided sufficient contrast to disrupt the pilot's scan of the environment."* Note these ratios are far below WCAG's 4.5:1, and that contrast is signed against a variable background — fixed chrome luminance flips between positive and negative contrast as the background changes. *(Chase Ververs & Wickens 1998 Exp. 1.)*

---

## 7. Cueing, tunneling, and what an overlay that asserts costs

**`[UNVOTED]`**, primary, replicates three earlier experiments, n=16 total (small-n flagged).

A 100%-reliable "lock-on" cue over live 3D terrain improved detection of the cued target but **significantly reduced** detection of an uncued, higher-priority anomaly in the same scene, F(1,12)=6.72, p=0.02. The cost held under 100% reliability but not under 75%.

In signal-detection terms sensitivity **dropped**: no cueing → P(A)=0.88, hit 84%, false alarm 8%; 100%-reliable cueing → P(A)=**0.64**, hit 91%. On the one trial where the cue pointed at a non-target, **63% of operators reported the distractor as a target anyway.** Telling operators the cue was only 75% reliable produced only partial recalibration — sustained false-alarm rate stayed at 45.5%.

**`[UNVOTED]`, n=16, single-event existence proof — not a rate.** A mountain adjacent to a checkpoint was deleted from the terrain database. Subjects were experienced, actively navigating, holding a paper map, and explicitly instructed to report display/map inconsistencies. **Zero of sixteen explicitly noticed.** At most two may have implicitly noticed (by failing to call the checkpoint), and both attributed the miss to concurrent search load.

**`[UNVOTED]`, same source.** Passive replay-watchers were markedly more complacent than active navigators: P(A) 0.76 vs 0.84, false alarms 32% vs 15%.

**`[UNVOTED]`, same source.** Increasing scene realism did **not** increase operators' trust in the simulation but **did** increase reliance on the overlay and reduce processing of the raw scene.

**`[UNVOTED]`, mixed tier.** Two mitigations, which partly conflict in the literature: (a) cue a *region* rather than a point, to widen attentional breadth — a design recommendation, not tested in this report; (b) scene-linked (conformally registered) overlay reduces the divided-attention cost that floating overlay incurs — but this **disagrees in direction** with Yeh & Wickens 1998, which found conformal head-up cueing tunneled *more* than the same cue on a hand-held display.

---

## 8. Contrast and typography

Nearly all of this section traces to **one self-published, non-peer-reviewed practitioner article** (Waller, Cambridge Engineering Design Centre, April 2022) and to **an open, unresolved W3C standards dispute**. Tier is low; the algebra is checkable, the characterizations are one side's brief.

**`[UNVOTED]`, computed not measured.** Across 8,000 random backgrounds: 47% of backgrounds that pass WCAG 4.5:1 for black text fail APCA Lc>60. For white text, WCAG passes 2,848/8,000 vs APCA's 4,629 — APCA admits ~63% more valid backgrounds for light text. The author's own framing is explicitly conditional: *"if APCA is more accurate, then…"*

**`[UNVOTED]`, computed.** Pure red on pure black: WCAG 5.3:1 (passes AA), APCA Lc=40 (fails). Both formulas are luminance-only by construction (0.2126R^2.4 + 0.7152G^2.4 + 0.0722B^2.4) and hue-neutral.

**`[UNVOTED]`, argued from example, no controlled study.** Both WCAG and APCA are two-colour models that ignore surrounding-field luminance; the same black-text-on-colour pair becomes more legible when the surrounding page turns black. The author states no universally applicable two-colour model can fix this — only be "equally inaccurate" for light and dark surrounds.

**`[UNVOTED]`, scope note.** Legibility (can letters be told apart, eyes stationary) and readability (can the reader hit maximum reading speed) are distinct thresholds, and the entire WCAG-vs-APCA comparison is conducted on **legibility only**. **Neither threshold set has been validated for glance, peripheral, or moving-imagery viewing** — the exact condition a HUD element lives in.

**`[UNVOTED]`, advocacy — flagged suspect.** APCA's creator asserts, inside the W3C's own issue tracker, that WCAG 2's formula is polarity-blind and systematically mis-rates at both extremes. The polarity-blindness of (L1+0.05)/(L2+0.05) is structurally checkable algebra and not really in dispute; the characterization of magnitude and consequence is the advocacy part. Related assertions that WCAG's 24px/18.7px-bold breakpoint is misaligned with the human contrast-sensitivity function, and that SC 1.4.11's non-text 3:1 threshold has "no scientific basis," are one party's brief in an unresolved dispute and were **not** independently checked.

**`[UNVOTED]`, status fact — the most important caveat in this section.** APCA is **not** an adopted W3C standard and had not cleared the WCAG 3 subgroup's stated peer-review precondition. The subgroup's position: APCA *"requires extensive peer review from other researchers in colour contrast before adoption."* The ~50-entry "independent peer reviews" bibliography cited as evidence is maintained by APCA's creator and is overwhelmingly blog posts, Medium articles, talks and corporate case studies; only a handful are journal-published — and one of them is the same self-published article the numbers above come from.

**`[UNVOTED]`, peer-reviewed *Ergonomics*, causally isolated.** Wider letter shapes improve peripheral/parafoveal letter recognition via a crowding-specific mechanism: wider fonts (Helvetica Neue Extended vs Standard vs Condensed) produced fewer misreadings and specifically fewer **neighbour-letter** misreports — the diagnostic signature of crowding rather than stroke-visibility loss. Tested at 2° and 9°, both showing the effect. The authors flag that generalization beyond this one type family, to words and continuous reading, and to reading speed rather than letter ID, is not established.

**`[UNVOTED]`, N=24, 576 detection events, on-road, IRB-approved.** Overlaid AR text — static, non-animated, non-interactive — degrades detection of real-scene events monotonically in text volume. Detection odds 3.04× higher with no text than one line, 4.45× than two lines, 5.42× than three. Detection distance fell 150.6 → 124.7 → 114.8 → 114.0 ft. **Model fit is weak** (14.18% of deviance, AUC-ROC 0.77) and the no-text-vs-one-line CI [1.05, 8.81] barely excludes 1; post-hoc significance held only for baseline-vs-medium and baseline-vs-high. The real measured cliff is *no panel vs multi-line panel*.

**`[UNVOTED]`, same source.** Fixation is not perception: 33 of 55 missed/delayed events had usable eye-tracking showing >100 ms fixation with no response, and **87.9% of those were central-field-of-view**, not peripheral — refuting the researchers' own pre-registered hypothesis. 91.4% of central-FoV misses were complete misses, not delays.

**`[UNVOTED]` — FLAGGED, second-remove sourcing.** Claims attributed to Gabbard's 2008 dissertation about AR legibility remedies — that an opaque "billboard" backing beats background-adaptive glyph colour, minimum luminance ratios of 7:1 (video-see-through) and 1.6:1 (optical-see-through), and "1px outline is enough" — are annotated **in the harvest itself** as coming from *a 2024 review summarizing Gabbard et al. 2006/2007*, not from the dissertation the URL points at. That review was never fetched. This is the same one-hop indirection that produced fabrication elsewhere in this corpus. **Do not use these numbers.** Chase Gabbard, Swan & Hix (*Presence* 15(1):16–32, 2006) and Gabbard et al. (IEEE VR 2007, 35–42).

The base claim from the dissertation record itself is not subject to that flag: an identically-styled AR overlay can flip from fully legible to fully illegible within minutes as lighting and background change, uncontrolled.

---

## 9. Graphical perception — bounds on how much a ranking can carry

**`[VERIFIED 3/3]`** In a replication of Cleveland & McGill's graphical comparison experiment, the expected performance difference **between individual people** (1.5–3 percentage points) is **larger** than the expected difference between the pie, bubble and stacked-bar encodings themselves (1–1.5 pp).

**`[VERIFIED 3/3]`** The canonical ranking (position > area > angle > volume) is a property of the *average observer* model, not of individuals; a credible fraction of people show accuracy patterns deviating from it. It is an aggregate summary statistic, not a per-viewer prediction.

*Davis, Pu, Ding, Hall, Bonilla, Feng, Kay, Harrison, "The Risks of Ranking: Revisiting Graphical Perception to Model Individual Differences in Visualization Performance," arXiv:2212.10533 / IEEE TVCG. Quote verified verbatim through two independent renderers.*

---

## 10. Checked and killed

Retained so they are not re-derived. Each was refuted 3/3 against its own cited primary.

| Claim as extracted | Why it failed |
|---|---|
| "Motion works in the periphery, where colour and shape changes do not" | The cited source never says colour/shape are undetectable peripherally — only that motion is *faster*. Additionally the 200–310 ms RTs were spliced from a velocity-identification study (Tynan & Sekuler 1982) that had no colour or shape condition. Counter-evidence surfaced during refutation: **Suchow & Alvarez 2011 — motion *silences* awareness of colour/luminance/size/shape change.** |
| "Preattentive iff detection <200–250 ms and set-size independent" | The 200–250 ms figure is a **display exposure duration**, not a detection time. Treisman herself revised the strict parallel/serial dichotomy to a spectrum. The "physiological floor" argument fails — express saccades run ~100 ms. |
| "Prior study of a display confers essentially no search advantage" | The source's hedge ("*may* offer no assistance," gated to novel displays not committable to memory) was dropped. Contradicted by a large replicated literature — Hout & Goldinger 2010 (n>1,100), Chun & Jiang contextual cueing and its 2023 meta-analysis, Võ & Wolfe 2012. **Operators do learn a layout.** |
| "Visual features form an asymmetric interference hierarchy; encode critical state in the dominant channel (luminance)" | Callaghan 1989, the primary the hierarchy rests on, states: *"there was no dominance of one type of property difference over another. Rather, observers' performance was completely predicted by the relative discriminabilities of the two boundaries."* Relative discriminability, not a fixed channel ranking. |
| "Change blindness: 70% of those who missed the actor swap described the original" | **The 70% figure does not exist in Levin & Simons 1997.** Exhaustive grep of the retrieved PDF returns zero hits. Also strips the precondition from Simons & Levin 1997: *"unless a change… produces a localizable change or transient at a specific position on the retina, generally, people will not detect it."* |
| "Attention cannot be guided by semantic identity or importance — only basic features" | Contradicted verbatim by the abstract of the cited paper. Wolfe's GS6 lists **five** guidance sources including scene syntax/semantics and reward/value; scene guidance is called "the most important of these." The quote said "high-level object identity"; the claim silently substituted "semantic identity or importance." |
| "Targets within 2° of fixation are skipped ~30% of the time" | The 70%-fixated figure belongs to the **foraging** task; the actual T-among-L miss rate is 11%, hit rate 87%. The source also states FVF sizes "are not fixed properties of the human search engine." |
| "Shape guidance is effectively a foveal-only channel; encode in colour" | The cited figure was misdescribed — it contains no colour-defined ovals, and the reportable-without-eye-movement cases include an *isolated* oval, i.e. shape guidance succeeding peripherally when uncrowded. Rosenholtz attributes the bulk of search-efficiency variation to crowding and peripheral acuity, and explicitly warns against demo-derived design rules. |

---

## 11. Not established anywhere in this corpus

- **Text reflow / in-place length-change as its own measured variable.** No study isolates it. The nearest analogue tests travelling motion vs instant swap vs fade, which is not the same construct.
- **A small chrome element beside a large, slowly rotating, multi-hued sphere.** The overlay work measured aviation scenes; the motion work measured abstract stimuli; Bartram measured a static field with expected cues. The combination is unmeasured.
- **Fitts's law or steering-law transfer to gamepad, joystick or gaze pointing.** Explicitly targeted by one scope decomposition; zero sources returned.
- **Miller 7±2 vs Cowan 4±1.** Explicitly targeted; zero sources returned. The nearest number in the corpus is a *simulation parameter* from a discounted source and does not answer it.
- **10-foot-UI guidance, tabular vs proportional figures, digit jitter.** Targeted; nothing returned.
- **Hick's law is a non-issue for panel element count** — real-task slopes 32/8/4 ms/bit, so 2→16 items costs ~12–96 ms. `[UNVOTED]`, CHI 2020 reanalysis of two published datasets. The same paper notes "categorize to reduce choices" is not merely unsupported by Hick's law but mathematically contradicted by it, since the log function is concave and each nesting level adds fixed overhead. Validity envelope ~1–4 bits.
