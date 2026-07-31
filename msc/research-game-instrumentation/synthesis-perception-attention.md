# Synthesis — Run 1 (human perception / attention / typography), unverified claim pool

Source file: `harvest/run1-human-perception-attention-typography.md` (25 sources, 125 mined claims, 42 verification votes over 14 of those claims). This document covers the **111 claims that were never adversarially checked**. It does not re-litigate the 14 that already went through verification — see the correction in §1 for one small factual note about that section, then treat it as settled.

Everything below is graded and flagged by me, not laundered. Read §0 before trusting any single line item — it explains the one pattern that should shape how much weight you give every quote in this file.

---

## 0. The one thing to internalize before reading further

Across the 8 refuted claims in the verification section, the failure was almost never "the quote is fake." **The verbatim quotes were consistently real and accurately transcribed.** What failed, every time, was one of:

1. **The claim's own headline sentence oversteps the quote** ("motion works in the periphery where colour and shape changes *do not*" — the source never says colour/shape are undetectable, only that motion is *faster*).
2. **A number is reassigned from the study that measured it to the finding it's rhetorically attached to** (the 200–310ms RTs belong to a velocity-identification study, not the motion-vs-colour comparison they're glued to).
3. **A source's own explicit hedge is dropped** (Healey: studying a display "**may** offer no assistance," gated to novel/uncommittable-to-memory displays — the claim drops both the "may" and the gate and states it as a flat rule; Callaghan: "there was no dominance of one type... rather, performance was completely predicted by relative discriminability" — the claim converts this into a fixed "dominant channel" hierarchy).
4. **The design "consequence" is the extraction agent's own inference, not the source's**, presented in the same voice as the measured finding.

**Practical rule for reading everything below:** the verbatim quote block is more trustworthy than the sentence that introduces it, and the sentence introducing it is more trustworthy than any "design consequence for vivarium" reasoning appended after it. I've kept that separation explicit per entry. Where a claim's own framing already smells like #1–#4 above, I've marked it 🚩 rather than silently cleaning it up.

**Two sources are structurally exhausted.** `csc2.ncsu.edu/faculty/healey/PP/` (Healey, "Perception in Visualization") had all 5 of its mined claims put to verification and **all 5 were refuted**. There is nothing left from that source to report here — it should not be cited from this corpus at all without an independent re-read. `Wolfe2021_GS6.pdf` had 5 claims verified: 3 refuted, 1 survived, 1 contested-toward-refuted (medium confidence). One Wolfe claim remains unverified below (§3F) and I've discounted it accordingly — same source, same run, a 3-of-5 failure rate on its siblings.

---

## 1. A correction to my own brief, found by doing exactly what it asked

The brief states "10 were refuted and 1 contested — a ~70% kill rate." I recounted directly from the section headers rather than trusting that figure secondhand:

```
REFUTED:    8
contested:  1
SURVIVED:   5
total:     14
```

That's 8 refuted (57%), or 9/14 (64%) if contested is folded into "did not clear verification" — not 10/14 (~71%). Small difference, doesn't change the takeaway (this run's raw-extraction accuracy is genuinely bad), but it's worth flagging precisely *because* the brief asked me to distrust confident-sounding numbers, and a miscount is the easiest kind of number to launder unchecked. I'd guess the brief's "10" folded the contested claim in with the refuted ones and rounded, but I didn't ask before writing this, so treat "why" as unknown.

---

## 2. Answers to the two questions you flagged as open

### Q1: Is there real evidence about text that reflows / changes length in place, and its cost to attention held elsewhere?

**No direct study.** I looked specifically for something that isolates *reflow/length-change* as its own variable, separate from motion, separate from content-change, separate from onset. It is not in this corpus. That is a genuine hole, and it's the hole that matters most, since it's the literal defect being fixed.

What exists instead, and how far each piece actually reaches:

- **Stasko's ticker/blast/fade study** (§3B below) is the closest analogue and it's a good one, but it tests *traveling motion vs. instantaneous swap vs. fade* — not a panel that reflows/relays out its own content in place. A "blast" (instant full replacement) is not a reflow (progressive relayout with shifting line breaks).
- **Bartram's motion-type ordering** (already verified, SURVIVED) puts "movement over long distances" (their term for scrolling/traveling text) in the single most-distracting category, ahead of zoom, oscillation, and flicker. A reflow event is not literally traveling motion, but it does relocate glyphs and shift line boundaries, which is closer to that category than to an anchored micro-motion.
- **Rosenholtz's Feature Congestion model** (§3D) implies, but does not measure, that a region whose local feature statistics keep changing (which is what reflow does) continuously re-consumes the feature-space headroom other elements would need to pop out. This is my inference from the model, not a finding in the paper.
- **Boot, Brockmole & Simons** (§3F) show onset-capture is eliminated by a concurrent unrelated task — meaning even if reflow behaves like a mild onset each time it fires, an operator genuinely engaged in globe pattern-matching may not have that onset survive the competition anyway. This cuts in an interesting direction: it's not just "reflow is bad," it's "reflow may not even reliably interrupt the thing you're trying to protect," which is a second, independent argument against relying on it for anything important.

Net: the corpus supports "reflow is very likely bad, by extension from adjacent findings," but does not supply a number for it, and nobody has run the specific experiment. State it as inference, not measurement, if it goes in a segment.

### Q2: Is there a principled, measured account of how much a display may say before it effectively says nothing?

**Yes, one — and it's a genuinely strong answer.** Rosenholtz et al.'s **Feature Congestion** clutter model (§3D, `jov.arvojournals.org/article.aspx?articleid=2122001`) is the best fit for exactly the question you're asking, and it's the highest-tier claim in this entire unverified pool: theoretically principled (a computable statistic — the volume of the local feature-covariance ellipsoid over color/orientation/contrast at multiple scales) *and* empirically validated (predicts the contrast a target needs to be found, r = .93 across 20 map backgrounds; predicts search-time variance across natural backgrounds, r = .74–.83, accounting for roughly 55–69% of that variance). It converts "how cluttered is this scene" from a design judgment into a number you can compute off a rendered frame.

Two complementary, lower-tier pieces round it out:

- **Stasko's density/size tradeoff** (§3B): a *smaller* secondary display gave significantly faster detection of changing information but significantly slower primary-task performance — a directly measured density-vs-noticing tradeoff, single study, EE tier.
- **Ververs & Wickens' lowlighting result** (cited inside the NASA survey, §3C): dimming (not removing) task-irrelevant symbology partially recovered detection of far-domain events without losing the task-relevant benefit — evidence that the answer to "how much can it say" is not "less," specifically, but "less *salient*, selectively." Secondary-within-source (a review reporting someone else's numbers), so treat as a pointer to the original Ververs & Wickens 1998 paper if this becomes load-bearing.
- **Matthews et al.'s criticality-budget framing** (§3G) is the closest thing to a general theory, but it's argued, not measured — and the authors say so about their own work: their expert heuristic evaluation *could not determine the correct level of distraction/density*, and a field participant's stated preference (wanting something *more* distracting) directly contradicted what six expert evaluators recommended. That's a real finding, but the finding is "you cannot derive this from heuristics alone; it has to be measured in situ" — which is itself useful (it says don't trust a craft-consensus number here) but is not itself a number.

Net: there is no single universal "N bits" or "N chips" ceiling in this corpus. The best-supported claim is that the ceiling is **local and computable, not panel-wide and fixed** — it is a function of the clutter already present in the specific screen region a chip would occupy, and Feature Congestion is a procedure vivarium could actually run against its own rendered frames rather than adopt a borrowed threshold.

---

## 3. Graded claim pool, by theme

Grading key: **TP** = theoretically principled (derivable model) · **EE** = empirically established (measured; n and replication status given where known) · **CC** = craft/expert consensus (argued or heuristic, not measured) · **REV** = review/secondary (synthesizes others' data, not new measurement). Flags: ⚠ = read the caveat before using · 🚩 = I think this one oversteps its own quote, on the same pattern the verification section caught elsewhere · ✅ = nothing about this one raised my suspicion.

I selected roughly the strongest half of the unverified pool below (~55 of 111) — the other ~56 are lower-signal, redundant with something graded here, or already covered by a dead-end note in §4. I'd rather hand you fewer, checkable lines than pad the count.

### A. Attention capture: onsets, motion, and what defeats them

**A1. Onset capture is eliminated by a concurrent unrelated task; a static color singleton flips the opposite way and becomes* more* prioritized under the same load.** [EE, n=24–26, small, between-subjects] ✅
Single-task search slope for an onset distractor: 42 ms/item (vs. 11 ms/item when it was the target) — strong capture. Under a concurrent auditory 1-back task, this collapsed to 33 vs. 25 ms/item (not reliably different). Meanwhile a color singleton distractor, weakly prioritized single-task (45 vs. 30 ms/item), became strongly prioritized dual-task (40 vs. 12 ms/item). Mechanism offered: onsets are transient — if the secondary task delays search initiation, the transient has already dissipated (intercept differences >130ms observed); a color singleton stays distinct for the whole search, so delay doesn't cost it the same way.
Source: `visualcognition.nd.edu/.../bootbrockmolesimonspbr2005.pdf` · primary · 2005.
Why it matters here: this directly complicates "make the alarm an onset" as a universal fix — an onset's advantage is exactly the thing a loaded operator's engagement erodes. It also names a genuinely useful alternative: a *sustained state difference* may be the more reliable channel under load, not the transient.

**A2. Onset capture survives learned/history-based suppression that defeats a static color cue; it is also fragile to trial-frequency and near-fully defeatable by a valid predictive cue.** [REV/EE mixed — this source is a 2025 review citing older primary experiments, not new data of its own] ⚠ secondary-within-source
Founding result (Yantis & Jonides): only onset targets produced shallow search slopes; luminance/color singletons did not. In a paradigm designed to maximize learned suppression (distractor never the target across trials), color singletons were successfully ignored but onsets kept capturing. Folk & Remington (2015): onset cues on 20% of trials captured attention; on 100% of trials they did not — a low-level (not just operator-discipline) formalization of alarm fatigue. Theeuwes et al. (1998): initial saccades went to an abrupt-onset distractor on ~30–40% of trials. A 100%-valid spatial cue 200ms early "virtually eliminated" onset capture but left a small, consistently positive residual in every experiment.
Source: `pubmed.ncbi.nlm.nih.gov/40029311/` · primary (review article) · 2025-03.
Note: because this is a review citing decades-old primary work, chase the originals (Yantis & Jonides; Theeuwes et al. 1998; Folk & Remington 2015) before treating any specific number as load-bearing — the review's synthesis reads faithful to the underlying literature (consistent with A1 and with the already-verified findings elsewhere in this corpus), but I have not independently pulled those primaries myself.

### B. Motion, animation, and reflow-adjacent display technique (the closest material to Q1)

**B1. Ticker (traveling motion) vs. blast (instant swap) vs. fade — reaction/comprehension tradeoff, no single winner.** [EE, n=70, peer-reviewed IJHCS] ✅ — central to Q1, see §2.
Ticker gave the *worst* detection latency (54.3s vs. 33.6s blast, 35.5s fade — ~65% penalty, F(2,618)=9.53, p<0.01) but the *best* memorability (basic-awareness hit rate 88–91% vs. 79–83% blast vs. 74–78% fade, non-overlapping 95% CIs). The paper's own framing: no animation type wins on all three axes (distraction, reaction, comprehension) simultaneously; the designer has to pick a point on the tradeoff, not optimize globally.
Source: `faculty.cc.gatech.edu/~stasko/papers/ijhcs03.pdf` · primary · 2003.

**B2. Panel size trades detection latency against primary-task speed — a direct, measured density-vs-noticing tradeoff.** [EE, n=91] ✅ — central to Q2, see §2.
Small (840px/~70 chars) vs. normal (1180px/~160 chars) single-line displays: small had significantly *lower* monitoring latency and significantly *higher* basic-awareness hit rate, but significantly *slower* browse completion (up to ~20s within a 2-minute task). Authors attribute this to glance capacity — a bigger readout holds more, but costs more to extract a specific item from.
Source: same as B1.

**B3. A parameter tweak (fade *speed*) reversed the paper's own Experiment 1 conclusion.** [EE, methodological caution] ✅
Exp. 1 concluded motion (ticker) was best for comprehension. Exp. 2's *slow* fade (one shade per 150ms, 9s dwell) beat every ticker variant on detection latency and delivered a ~20-point improvement in detailed awareness (always p<0.01), with no deficiency on any measure. Authors' final recommendation reversed to "slow in-place fade," not motion.
Source: same as B1.
Note: this is a clean, in-paper demonstration that "use a fade" or "use a ticker" as a guideline is underspecified without a rate parameter — worth citing on its own if vivarium ever tunes an animation's timing constant, since it shows the constant, not just the technique, is load-bearing.

**B4. The mere presence of an animated secondary panel did not measurably slow a *reading* primary task — but the authors explicitly refuse to generalize this to visually demanding tasks.** [EE, n≈100s, explicit scope limit stated by the authors] ⚠ scope
Browse times with ticker/fade/blast were statistically indistinguishable from no-panel control (F(3,1625)=1.93, p=0.12; ~4s difference in a 60s task). The authors state directly: "we would not expect these results to generalize to dissimilar primary tasks, especially those that include... intensive cognitive processing." Vivarium's primary task (pattern-matching a globe) is precisely the excluded case.
Source: same as B1.

**B5. Motion ONSET beats offset/static/continuous for detection; a NEW OBJECT beats and fully suppresses motion onset; CONTINUOUS motion is worse than static.** [EE, n=12/experiment, replicated across 2 experiments + real analog motion in Exp. 2, small-n flagged by authors as typical for the paradigm] ✅
Ranking for detection speed: new object > motion onset > motion offset ≈ static > continuous. Continuous motion was *significantly less accurate* than static (d=.76–1.18). Critically, when a new object was present, the identical animated motion-onset stimulus that had captured attention on trials without a new object **failed to capture at all**.
Source: `link.springer.com/article/10.3758/s13414-018-1548-1` · primary · 2018-07-03.
Design consequence stated directly by this reading (not the paper's own framing, so treat as mine): a perpetually spinning/pulsing/scrolling status indicator is spending its salience on an ongoing basis for no compounding benefit, and if two attention-getting elements are present at once, only one wins — the rank order matters when allocating which chip gets which treatment.

**B6. The self-flagged boundary: capture depends on frame rate and per-frame displacement, not "motion" as an abstract category.** [EE, cited within B5's own source, itself citing Sunny & von Mühlenen 2011/2014] ⚠ secondary-within-source, but the citing paper's own limitations section is unusually candid
Prior work found *jerky* (8–17Hz) redraw captures attention; *smooth* (33–100Hz) does not. The 2014 companion isolated the active ingredient as a single abrupt displacement of 0.26°–1.05°, which captures whether or not motion continues afterward; flicker alone did not. On a modern high-refresh display, "smooth" animation of a status chip may not reliably capture at all — the parameter that matters is the size and abruptness of the displacement, not smoothness per se.
Source: same as B5, citing Sunny & von Mühlenen (2011, 2014).
The source's own honesty, worth repeating verbatim because it's a model for how the *rest* of this corpus should have been extracted: despite being titled "Motion onset really does capture attention," the authors' own Limitations section declines the strict definition of capture (context-insensitive, "cognitively impenetrable") because their own Experiment 1 effect was context-dependent (vanished when a new object was present) and top-down goals were never manipulated. **They explicitly downgrade their own headline finding in their own paper.** That is the discipline the failed claims elsewhere in this corpus lacked.

**B7. Motion alerts survive demanding primary tasks, but only when participants know a cue is coming — the paper flags this as a real limitation, not a footnote.** [EE, well-quantified, but with an author-stated ecological-validity caveat] ⚠ scope
Worst-performing of 8 tested motion cues (slow blink) still had mean RT <3s and 89% detection within a 10s window, across genuinely demanding primary tasks (reading-to-learn, FreeCell, Tetris, 32 icons around a full-screen window). But participants *knew cues were coming* and could poll; the authors state this may not transfer to true vigilance settings with unpredictable alarms. A second predicted effect also failed: Tetris was anecdotally the most engaging task, but detection was *slowest* under Solitaire — subjective engagement did not predict alert cost.
Source: `interruptions.net/literature/Bartram-IJHCS03-BW.pdf` · primary · 2003-05.
Directly relevant: a rarely-firing freshness alarm is precisely the unexpected-cue regime this study does not cover — do not borrow the 89%/<3s numbers as a guarantee for that case.

**B8. Motion is an orthogonal coding channel — it doesn't overwrite existing color/shape encodings — but color/shape identification was poor even in sparse, uncrowded test displays.** [EE] ✅
Motion can be layered onto an object without disturbing whatever color/shape already encodes other variables (citing Bartram & Ware 2002). In the same paper's Experiment 2, colour-cue identification accuracy was only ~20% (near) / ~15% (far) even in a sparse display — despite colour being, per the authors, the most commonly used alert code in real interfaces.
Source: same as B7.

### C. Overlay-over-a-moving-scene (the closest structural analogue to a HUD floating over a globe)

**C1. Overlaid symbology delays detection of an unexpected runway incursion, and gives a large real-time-safety-relevant latency penalty for non-conformal symbology specifically.** [EE, but SECONDARY-within-source — this is a 2004 NASA survey reporting Wickens & Long 1995's numbers, not new data] ⚠ chase the 1995 original
Time to initiate a go-around: 5.5–6.7s head-down vs. 7.2–9.1s head-up, a 0.5–3.6s penalty, worst for non-conformal symbology (symbology whose motion doesn't track the scene). Same study: overlay was *better* for the routine tracking task (30% less flightpath deviation, conformal case).
Source: `cs.odu.edu/~mln/ltrs-pdfs/NASA-2004-tm213000.pdf` · primary (survey) · 2004-02.

**C2. The overlay effect reverses sign by expectancy — this is the load-bearing boundary condition for an instrument whose whole purpose is catching undeclared violations.** [EE, meta-analysis of 18 experiments, secondary-within-source] ⚠ chase Fadden, Ververs & Wickens 1998
Pooled across 18 studies, overlay vs. separate-display detection was statistically indistinguishable (p=.215). Splitting the same corpus by expectancy revealed a reliable effect (Z=1.968, p<.026 — note the source document itself misprints this as "p>.026," an error I'm flagging rather than repeating uncritically): overlays *help* detection of events the operator expects, and *cost* detection of events they don't. For an instrument whose entire value proposition is catching things nobody declared in advance, this is squarely the cost regime.
Source: same as C1.

**C3. Screen-fixed (non-scene-linked) overlay symbology performed *worse than no instrument at all* on primary tracking; scene-linked symbology beat both.** [EE, n=14, secondary-within-source] ⚠
Mean path RMSE: 70ft no gauge, 73.5ft superimposed (screen-fixed), ~64ft scene-linked (3 conditions averaged) — held even when gauges weren't placed along the flown path. Proposed mechanism (authors call it speculative): static overlay and moving scene form separate perceptual groupings, and attention divides poorly across groupings.
Source: same as C1, citing Levy, Foyle & McCann 1998.

**C4. Information density, not brightness, is the detection killer for overlaid symbology — and dimming (not removing) irrelevant elements partially recovers the loss.** [EE, licensed pilots, high-fidelity sim, secondary-within-source] ⚠
High-clutter symbology sets significantly reduced event detection in both overlaid and separate-display conditions. "Lowlighting" (dimming) task-irrelevant symbology improved far-domain traffic detection but did not improve detection of commanded changes — a selective, partial recovery, not a full fix.
Source: same as C1, citing Ververs & Wickens 1998 Exp. 2. This is the strongest single piece of evidence for "reduce salience selectively, don't just reduce area" as a design lever — see §2 Q2.

**C5. Symbology *intensity* (brightness) did not disrupt environmental scanning at any tested level — the capture mechanism is clutter/grouping/conformality, not raw luminance.** [EE, secondary-within-source] ⚠ but corroborates C4's mechanism claim
Contrast ratios tested (1.17:1 to 1.6:1, both signed directions depending on background) drove symbology detection speed but never cost far-domain scanning. Authors' direct quote: none of the three intensities "provided sufficient contrast to disrupt the pilot's scan of the environment." Also: these tested ratios (1.17–1.6:1) are far below the WCAG 4.5:1 web threshold, and — because contrast is signed against a variable background — the same fixed chrome luminance flips between positive and negative contrast as the background changes.
Source: same as C1, citing Ververs & Wickens 1998 Exp. 1.

**C6. Attentional tunneling from a "look here" cue is a measured cost, and cueing produces a criterion shift with no sensitivity gain — the operator becomes more willing to assert, not more accurate.** [EE, replicates 3 earlier experiments, n=16 total, small-n flagged] ✅ — see §2 Q2, this is the strongest "an overlay that asserts things has a cost" evidence in the corpus
A 100%-reliable "lock-on" cue over live 3D terrain improved detection of the cued target but *significantly reduced* detection of an uncued, higher-priority anomaly in the same scene (F(1,12)=6.72, p=0.02), and the cost held under 100% reliability but not under 75%. In signal-detection terms: no cueing → P(A)=0.88, hit 84%, false-alarm 8%; 100%-reliable cueing → P(A)=0.64 (sensitivity *dropped*), hit 91%, and on the one trial the cue pointed at a non-target, 63% of operators reported the distractor as a target anyway. Telling operators the cue is only 75% reliable produced only partial recalibration — sustained false-alarm rate stayed at 45.5%.
Source: `web.mit.edu/16.459/Yeh&Wickens.pdf` · primary · 2000-04.
Directly relevant: whatever a status chip *asserts* will be trusted over the raw scene, and disclosing the chip's imperfect reliability does not restore calibration — it only partially limits the damage while giving up most of the chip's benefit.

**C7. 16 of 16 experienced operators missed a large, structurally deleted piece of terrain while actively holding a paper map and being explicitly warned the simulation could be wrong.** [EE, n=16, single-event existence proof, not a rate estimate — say so if you cite it] ⚠ small-n, single event
A mountain adjacent to a checkpoint was deleted from the terrain database. Subjects were told to report display/map inconsistencies. Zero explicitly noticed; at most two may have implicitly noticed (by failing to call the checkpoint), and both attributed the miss to concurrent search load.
Source: same as C6.
This is the sharpest available boundary condition on vivarium's own stated premise that a trained eye is the fastest detector of missing physics — true, per this study, *only when the eye is not concurrently loaded*. Same source: passive replay-watchers were markedly more complacent than active navigators (P(A) 0.76 vs. 0.84; false alarms 32% vs. 15%) — a `vivarium watch` viewer may be a structurally worse detector than a `vivarium explore` operator actively steering, independent of any HUD design question.

**C8. Two evidenced mitigations for tunneling, both about geometry rather than styling — and they partly conflict with each other in the literature.** [CC for the "broaden the cue" recommendation (reasoned, not directly tested here); EE-secondary for scene-linking] ⚠
(1) Cue a *region*, not a point — widens attentional breadth while keeping most of the benefit (design recommendation, not itself tested in this report). (2) Scene-linked (conformally registered) overlay elements reduce or eliminate the divided-attention cost that floating overlay elements incur — but this *disagrees in direction* with a different cited study (Yeh & Wickens 1998 found conformal head-up cueing tunneled *more* than the same cue on a hand-held display). A third finding worth carrying: increasing scene realism did not increase operators' trust in the simulation but *did* increase reliance on the overlay and reduce processing of the raw scene — prettier world, more deference to the chrome laid over it.
Source: same as C6.

### D. Clutter, information density, and glanceability (Q2's best answer)

**D1. Feature Congestion: clutter is a computable statistic that predicts both search cost and the contrast a target needs.** [TP+EE — the strongest-tier claim in this unverified pool] ✅
FC = the volume of the local feature-covariance ellipsoid (color, orientation, contrast-energy) at multiple scales — an operational formalization of "how much headroom is left for something new to draw attention." Contrast a target needs to reach 75%-correct detection correlated r=.93 with FC (4 trained observers, 20 map backgrounds) — better than Subband Entropy (r=.68) or Edge Density (r=.83). Across natural backgrounds, FC/Subband Entropy/Edge Density together explain roughly 55–69% of variance in mean search time (log RT correlations .74–.83, 6 subjects, 3,648 trials).
Source: `jov.arvojournals.org/article.aspx?articleid=2122001` · primary, Journal of Vision · 2007-08-16.

**D2. Color variability, specifically, costs search time independent of how many things are on screen — and edge/element-count-based clutter proxies miss it entirely.** [EE, tight causal isolation] ✅
With edge density held statistically constant across original/gray/red versions of the same maps (12.5–13.0% pixels, n.s.), desaturating cut target-present RT from 772ms (original) to 619ms (gray, t(23)=8.3) and 552ms (red, t(23)=11.8), both p<.001 — a 20–29% reduction from color variability alone. Edge Density predicted no difference; Subband Entropy predicted the *wrong direction* for the red condition; only Feature Congestion tracked the actual pattern.
Source: same as D1.
Practical design lever this implies directly: the number of *distinct hues* in the base globe rendering is a first-order cost on chip discoverability — independent of how many chips or drawn elements are present.

**D3. Monochrome is not automatically "uncluttered" — it removes the color headroom an alarm would use.** [TP, stated explicitly by the authors as a caution against their own metric being misread] ✅
Because Feature Congestion measures fraction of *available* feature space consumed, a desaturated/monochrome display can score as low-clutter while having nothing left in the color channel for a new element to claim. Quoted directly: "it may be misleading to label a display as 'uncluttered' simply because it is monochrome; one might not be able to add a target that draws attention because of its color."
Source: same as D1.
This is a direct, sourced counter to the intuitive fix of desaturating the globe rendering to make chips pop — it may buy edge-based legibility while spending the exact channel (hue/color) a status chip would otherwise use to announce itself.

**D4. Background clutter costs are additive and survive top-down knowledge — an operator who *knows* the background is irrelevant still pays the tax.** [EE, secondary-within-source citing Wolfe et al. 2002] ⚠
Even when targets appeared in predictable locations designed to let observers ignore the background entirely, messier backgrounds still slowed search, tracking the Feature Congestion ordering (empty 3.4, clean 4.3, messy 6.1).
Source: same as D1, citing Wolfe et al. 2002.
Refutes "the operator knows the globe is irrelevant to reading a chip, so they'll suppress it" as a design defense — the cost is paid regardless and has to be engineered down in the rendering, not instructed away.

**D5. Hick's-law slope is small enough in real HCI tasks that added choices don't meaningfully cost decision time — and the "categorize to reduce choices" design principle is not just unsupported by Hick's law, it's mathematically contradicted by it.** [TP+EE, peer-reviewed CHI 2020 reanalysis of 2 published datasets] ✅
Reanalyzed slopes: 32, 8, 4 ms/bit — going from 2 to 16 on-screen items costs roughly 12–96ms, not design-relevant. Because the log function is concave, applying Hick's law honestly to N=32 items recommends showing everything on one page over splitting into categorized subpages (each nesting level costs (m−1)a extra seconds of fixed overhead). Validity envelope: linear only over ~1–4 bits (2–16 alternatives); above that RT is over-estimated by the log model and Fitts & Posner report RT seldom exceeds 1s regardless of alternative count; the envelope is itself task-dependent (Pollack found linearity to ~10 bits in word-naming).
Source: `perso.telecom-paristech.fr/rioul/publis/202001liugoririoulbeaudouinlafonguiard.pdf` · primary, ACM CHI 2020 · 2020-04-25.
Directly answers a sub-question nobody asked but you'll want: adding a few more status chips to a panel is very unlikely to be costing decision latency by way of Hick's law — if chips cost anything, it's via visual clutter (§D1–D4) or crowding (§F), not choice-reaction time. Also names the specific misuse pattern (observing a log-shaped curve and attributing it to Hick's law when many unrelated mechanisms — hierarchical menu search, alphabetized scrolling, divide-and-conquer — also produce logs) as the dominant HCI misapplication of this law, across 8 named studies.

**D6. "Chartjunk" causes no measurable comprehension loss under unconstrained viewing time — but the authors explicitly decline to extend this to safety-critical or time-limited monitoring, naming flight control by name as the excluded case.** [EE, n=20, peer-reviewed CHI 2010] ✅ — this is the most on-the-nose author-stated scope limit in the whole corpus
Heavily embellished (Nigel Holmes) charts produced no accuracy loss vs. plain charts on subject/category/trend description, with viewing time unconstrained. But: only 67% of gaze time landed on data-bearing regions for embellished charts vs. 78% for plain (an 11-point pull-away, still with no task-accuracy cost at this unconstrained-time task); the "no time cost" finding itself failed to reach significance at only p=.082 with the point estimate running *against* embellishment (2.60min vs 2.43min, ~7% slower); and the *only* measured benefit of embellishment was long-term recall after 12–22 days (multiple measures significant), with zero benefit at 5-minute recall.
Source: `vis.csail.mit.edu/classes/6.859/readings/pdfs/Bateman-UsefulJunk.pdf` · primary, CHI 2010 · 2010-04-10.
Do not use this to justify decorative chrome on a real-time instrument — the authors themselves rule that use out, and the gaze-pull-away and marginal time-cost data suggest their own headline finding is softer than "no cost" even within its stated scope.

### E. Legibility, contrast, and typography (numbers you can literally design against)

**E1. WCAG 2.x and APCA give opposite-polarity legibility predictions over a large, specifiable region of color space — computed, not measured on humans.** [TP — this is a computed disagreement-rate between two formulas, not a human-subject finding; the source itself never claims otherwise] ⚠ not peer-reviewed (self-published practitioner article), no human-subject data
Across 8,000 random backgrounds: 47% of backgrounds that pass WCAG 4.5:1 for black text fail APCA Lc>60. For white text, WCAG passes only 2,848/8,000 backgrounds vs. APCA's 4,629 — APCA admits ~63% more valid backgrounds for light text. Author's own framing is explicitly conditional: "if APCA is more accurate, then..." — treat that conditional as live, not settled.
Source: `cedc.tools/article.html` (Sam Waller, Cambridge Engineering Design Centre) · self-published, not peer-reviewed despite the file's "primary" tag · April 2022.

**E2. Both WCAG and APCA are two-color models that ignore surrounding-field luminance — and the same black-text-on-color pair becomes measurably more legible when the surrounding page turns black.** [CC/TP — argued from example, no controlled human study cited] ⚠
The author states no universally applicable two-color model can fix this; it can only be "equally inaccurate" for light and dark surrounds. Directly relevant: this is exactly the regime of a chip floating over a globe whose local surround (ocean, ice, terminator, cloud) changes continuously — a fixed contrast computation against a single assumed background will drift out of validity as the globe rotates.
Source: same as E1.

**E3. Lightness contrast is more robust than hue contrast for readability, partly because peripheral vision is weak on hue — and a concrete failure case: red-on-black passes WCAG AA but fails APCA, and is worst-case for protanopia.** [TP — both formulas are luminance-only by construction (0.2126R^2.4+0.7152G^2.4+0.0722B^2.4), hue-neutral] ✅ as a structural fact about the formulas; ⚠ the color-vision-impairment framing is asserted, not tested here
Pure red on pure black: WCAG 5.3:1 (passes AA), APCA Lc=40 (fails). Directly relevant to any red-glyph alarm styling over a dark globe.
Source: same as E1.

**E4. Legibility ("can the letters be told apart, eyes stationary") and readability ("can the reader hit max reading speed") are distinct thresholds — and the entire WCAG-vs-APCA comparison above is conducted on legibility only.** [definitional/scoping claim, not itself measured] ✅ as a scope note
Neither threshold set (WCAG's 4.5:1/3:1 or APCA's Lc values) has been validated here for glance, peripheral, or moving-imagery viewing — the article defers readability to further research explicitly.
Source: same as E1.
Worth internalizing on its own: nothing in this pair of contrast standards was ever validated for the actual viewing condition (a glance, off-fixation, over a moving image) that a HUD chip lives in.

**E5. WCAG 2's contrast formula is asserted, by APCA's own creator inside the W3C's own standards-track issue tracker, to be polarity-blind and to systematically mis-rate contrast at both extremes.** [CC/advocacy — this is a contested claim from an interested party in an open, unresolved standards dispute, not a settled fact] 🚩 read carefully before citing as fact
Claim: WCAG under-rates contrast for white text (false failures) and severely over-rates it when either color is black (false passes), with the black/white flip point wrong through mid-grays #76–#a3. The issue thread is still open as of the retrieval date (opened 2023-10-24, last comment 2026-04-16). The polarity-blindness of the underlying (L1+0.05)/(L2+0.05) formula is structurally checkable (it's just algebra) and not really in dispute; the *characterization of the magnitude and consequence* of that blindness is the advocacy part.
Source: `github.com/w3c/wcag3/issues/29` · this is a GitHub discussion thread by APCA's creator (Andrew Somers / "Myndex"), not a paper — treat the "primary" tag loosely · opened 2023-10-24, ongoing.

**E6. APCA itself is explicitly NOT an adopted W3C standard, and had not cleared the WCAG 3 subgroup's own stated peer-review precondition as of the most recent activity in this thread.** [meta/status claim] ✅ — this is the single most important caveat for anyone tempted to switch a HUD's contrast math to APCA wholesale
The subgroup's quoted position: APCA "requires extensive peer review from other researchers in color contrast before adoption." APCA's own creator maintains the ~50-entry "independent peer reviews" bibliography cited as evidence of that review — and it is overwhelmingly blog posts, Medium articles, conference talks, and corporate case studies; only a handful are journal/book-published (Ulitin 2021; Ulitin 2023 Springer CCIS; Waller/cedc.tools itself, April 2022 — i.e. one of the "independent reviews" is the same self-published article graded E1–E4 above, which is not independent of the thing it's reviewing).
Source: same as E5.
Recommend treating APCA as a credible research-grade *alternative under active contestation* — worth knowing about, not worth silently substituting for WCAG as if it were the settled successor.

**E7. WCAG 2's only concession to typography is a single text-size breakpoint (24px/18.7px bold), asserted to be misaligned with the actual human contrast-sensitivity function because it derives from physical large-print convention rather than spatial-frequency research; and its non-text 3:1 threshold (SC 1.4.11) is asserted to have no scientific basis at all.** [CC/advocacy, same source and same caveat as E5] 🚩
If true, this bears on chip borders, chevrons, and icon glyphs specifically, since non-text marks span a much wider spatial-frequency range than text. But this is, again, one party's characterization inside an unresolved dispute — the claim that 1.4.11's "cited references are self-referential, out of context, or with qualifications ignored" is a serious accusation that I have not independently checked against the SC 1.4.11 rationale document myself.
Source: same as E5.

**E8. A four-tier contrast/size budget by content criticality is proposed (body / fluent / soft-content / spot-readable), with sizing specified by x-height at expected visual angle rather than nominal font size.** [CC — proposed guideline, not validated] ⚠ proposal, not a finding
Directly transferable framing for a status-chip set: an alarm chip would sit at "body" tier (near-max contrast), a selection readout at "soft-content," an idle-mode label at "spot-readable." The x-height-at-visual-angle sizing recommendation is stated because font metrics are inconsistent across families — nominal point size is not comparable across typefaces.
Source: same as E5, from the same issue thread's proposal section.

**E9. Overlaid AR text — even completely static, non-animated, non-interactive — measurably degrades detection of events in the real scene behind it, and the effect is monotonic in how much text is shown at once.** [EE, N=24, 576 detection events, on-road study, IRB-approved] ⚠ weak model fit on the low-density condition specifically
Odds of detecting a roadway event: 3.04x higher with no text task than one line, 4.45x higher than two lines, 5.42x higher than three lines — but the model only explains 14.18% of deviance (AUC-ROC 0.77) and the no-text-vs-one-line CI [1.05, 8.81] barely excludes 1 (the weakest of the three contrasts). Detection distance fell monotonically: 150.6ft (no text) → 124.7 → 114.8 → 114.0ft. Post-hoc significance held only for baseline-vs-medium and baseline-vs-high — i.e., the real measured cliff is "no panel" vs. "multi-line panel," not "one line" vs. "two lines."
Source: `arxiv.org/pdf/2505.00879` · primary, Virginia Tech IRB #22-294 · 2025-05-01.

**E10. Fixation is not perception: inattentional blindness in this study was *concentrated in the central field of view*, refuting the researchers' own pre-registered hypothesis that periphery would be worse.** [EE, same source as E9] ✅
33 of 55 missed/delayed events had usable eye-tracking data showing >100ms fixation with no response; 87.9% of those were central-FoV, not peripheral. 91.4% of central-FoV misses were complete misses, not delays. Design consequence stated by the source itself (not my inference): "put the alarm where the eye already is" is not sufficient — fixation and perception are separable events.
Source: same as E9.

**E11. Wider letter shapes improve peripheral/parafoveal letter recognition via a crowding-specific mechanism, tested at two separate eccentricities.** [EE, peer-reviewed Ergonomics, causally isolated to one variable within one type family] ✅
Wider fonts (Helvetica Neue Extended vs. Standard vs. Condensed) produced fewer misreadings, and specifically fewer *neighbor-letter* misreports — the diagnostic signature of crowding, not raw stroke-visibility loss. Tested at 2° (parafovea) and 9° (periphery), both showing the effect. Authors explicitly flag that regular (untested-as-optimal) width is simply the unexamined default on most displays, and that generalization beyond this one type family, to words/continuous reading, and to reading speed rather than letter ID, is not established here.
Source: `tandfonline.com/doi/full/10.1080/00140139.2021.1991001` · primary, Ergonomics · 2021-10-27.

**E12. Legibility is not a stable property of a chip's own styling — it drifts with the moving background behind it, and the field-tested remedy is an opaque backing plate ("billboard"), not background-tuned glyph color.** [EE for the base claim (from the dissertation itself); ⚠🚩 provenance gap for the specific remedy numbers, explained below]
Base claim (from the dissertation record directly): an identically-styled AR overlay can flip from fully legible to fully illegible within minutes as lighting/background change, uncontrolled. Falsifiable and directly relevant: a chip color chosen once against one globe state (ocean, one sun angle) should be expected to fail against another (ice, terminator, cloud) with zero change to the chip itself.
Source: `vtechworks.lib.vt.edu/handle/10919/29093` (dissertation) · primary · 2008-09-11; associated primary studies Gabbard, Swan & Hix (Presence 2006) and Gabbard, Swan, Hix, Kim & Fitch (IEEE VR 2007).

⚠🚩 **Provenance flag, stated plainly:** the more specific and more design-useful numbers attached to this source in the original harvest — the "billboard beats color-tuning" verdict, the 7:1 (video-see-through) vs. 1.6:1 (optical-see-through) contrast floors, and the "1px outline is enough, thicker doesn't help" claim — are explicitly annotated in the harvest itself as coming not from the 2008 dissertation but from *"the 2024 multivocal review of this literature, summarizing Gabbard et al. 2006/2007"*. That 2024 review is not among this run's 25 fetched sources, was never itself fetched or verified, and its accuracy is therefore a claim-about-a-claim at one more remove than everything else in this file. Given this run's demonstrated rate of exactly this kind of indirection producing fabrication (the Healey-source refutations were largely citation splices of this shape), I would treat the 7:1/1.6:1 numbers and the "billboard wins" verdict as **plausible but unverified at a second remove** — worth checking against the actual Gabbard 2006/2007 papers (Presence 15(1):16–32; IEEE VR 2007 pp. 35–42) before using either number in a segment, rather than against the dissertation record cited.

**E13. Background texture only degrades readability in the low-contrast regime — a conditional, not unconditional, cost.** [same 2024-review-sourcing caveat as E12] ⚠🚩 same provenance flag as E12
If accurate, this bounds how much effort should go into background-aware chip adaptation: above some contrast threshold, a busy moving globe costs nothing measurable, so buying flat contrast may substitute for buying real-time adaptivity. Treat with the same second-hand-sourcing caution as E12.
Source: same as E12.

### F. Peripheral vision, eccentricity, and crowding (geometry constraints on where a chip can live)

**F1. Concurrent foveal (central) visual load produces genuine tunnel vision — the deficit grows with eccentricity; concurrent auditory working-memory load degrades peripheral sensitivity uniformly without shrinking the field. This adjudicates a real prior dispute in the literature.** [EE, Journal of Vision, methodologically careful — explicitly controls for a known confound in prior UFOV work] ✅
Directly names the mechanism for vivarium's own premise: the operator's *own* central visual task (globe pattern-matching) is what would make a peripheral alarm chip disproportionately hard to detect — an audio/cognitive load alone would not do this. The authors also flag that most prior UFOV/"tunnel vision" literature didn't control for eccentricity-dependent spatial resolution, so older tunnel-vision numbers from other sources may conflate a fixed retinal property with genuine attentional narrowing.
Source: `pubmed.ncbi.nlm.nih.gov/27050950/` · primary, Journal of Vision 16(2):7 · 2016.

**F2. To equate detectability across the visual field, peripheral stimuli must be scaled up in BOTH size and exposure time — equal pixel size is not equal visibility, and this isn't optional, it's how the researchers had to design their own equal-performance baseline.** [TP/methodological, same source as F1] ✅
Source: same as F1.
Direct design consequence: an alarm meant to be equally noticeable off-axis needs to be both larger and shown longer than its foveal equivalent — matching pixel dimensions across screen positions is not matching visibility.

**F3. Any concurrent load — visual or auditory — reduces peripheral sensitivity to some degree, even when it doesn't shrink the field; only the eccentricity-dependent *worsening* is specific to foveal load.** [EE, same source] ✅
Source: same as F1.
So a peripherally placed chip loses some detectability under *any* concurrent cognitive engagement, not only under visual engagement — the modality-specificity only applies to the *tunneling* effect, not to the baseline degradation.

**F4. Crowding, not acuity loss, is the dominant limit on peripheral reading, and it obeys a quantitative law (Bouma's law) that's been replicated across letters, bars, faces, colored circles, and real-world objects since 1970.** [TP+EE, replicated, Annual Review of Vision Science] ✅
Critical center-to-center spacing at which a neighbor destroys recognition ≈ 0.4–0.5x the target's eccentricity. Directly designable: a chip at 10° eccentricity needs ~4–5° of clear space around it to be readable, regardless of its own size or contrast.
Source: `visxvision.com/wp-content/uploads/2017/08/ruth_peripheral.pdf` (Rosenholtz) · primary, Annual Review of Vision Science · 2016.

**F5. Crowding is anisotropic in two separately measured ways that constrain chip *layout geometry*, not just chip size.** [EE] ✅
(a) Radial-tangential: critical spacing is ~2x larger for a flanker aligned radially with the target than for one placed tangentially. (b) Inward-outward: for equal spacing, a more-eccentric flanker interferes more than a more-central one. Concretely: a vertical stack of chips in a screen corner (radially aligned relative to a center-screen fixation) crowds itself roughly twice as hard as the same chips arranged along the arc perpendicular to that radius, and the outermost chip in a stack is the worst offender against its inward neighbors, not the reverse.
Source: same as F4.

**F6. The widely-reproduced "peripheral vision is blurry" demo is quantitatively wrong by roughly a factor of four, and the review names HCI/human-factors/computer-graphics by name as fields that carry the error forward.** [EE, Anstis 1974 / Horton & Hoyt 1991 / Levi et al. 1985, with the exaggeration factor as the review author's own estimate] ✅ but note the last clause is the author's estimate, not an independent measurement
Cortical-magnification scaling means a target at 10° eccentricity needs only ~4x its foveal size to be equally resolvable — and foveal acuity is so far above task requirements that 4x of it is still small. Design consequence stated directly by the source: enlarging a peripheral readout buys little, because acuity was never the binding constraint — isolation, spacing, and reduced clutter (i.e., crowding, per F4/F5) are what actually buy peripheral legibility, not size.
Source: same as F4.

**F7. The standard "the observer failed to attend" explanation for change blindness and inefficient search is directly challenged by a competing account: a largely fixed, lossy, eccentricity-dependent peripheral encoding.** [TP (image-computable summary-statistic model) + EE support] ✅ but author explicitly does not claim this rules out attentional limits too
Freeman & Simoncelli's "metamer" pairs — images sharing the same local summary statistics — remain indistinguishable even when observers *deliberately* attend to the regions that differ most. Operational consequence, stated by the source: if a state change is lost to the peripheral encoding itself, telling the operator to pay attention cannot recover it, and neither can training — only changing the encoding (size, isolation, eccentricity, or a transient that survives pooling) can.
Source: same as F4.
The review nominates exactly vivarium's use case (dashboards, maps) as an application: synthesize a summary-statistic-matched version of a candidate HUD layout at an assumed fixation point, and check whether the alarm chip survives the synthesis. This converts "is it structurally self-announcing" from a judgment call into a testable (if not-yet-built) procedure — flagged as *proposed*, not validated for dashboards specifically, by the source itself.

**F8. Selective attention throughput and capacity: ~20 selections/sec, >150ms/item to bind and recognize, modeled with a working capacity of 5 items.** [TP — this is a simulation architecture's own internal parameters, not an independent measurement of human capacity] ⚠🚩 same-source discount — from Wolfe GS6, whose siblings in this run were 3/5 refuted
Source: `search.bwh.harvard.edu/new/pubs/Wolfe2021_GS6.pdf` · primary · 2021-02-05.
I'm grading this lower than its face-value interest warrants, specifically because it's from the one source in this corpus with the worst track record on independent verification (3 of 5 sibling claims refuted, largely on the "quote is real, headline oversteps it" pattern). The 5-item figure is a *model* parameter (Guided Search 6.0's own internal architecture), not a directly measured empirical capacity limit — worth distinguishing from Miller's 7±2 or Cowan's 4±1 (neither of which, notably, this corpus actually investigated — see §4). If this number becomes load-bearing for a design decision, re-verify it against the primary text yourself rather than trusting this line.

### G. Ambient/peripheral display design theory and field observation

**G1. Peripheral displays go silent when they break, and users can't distinguish "quiet because nothing's happening" from "dead" — a broken display went unnoticed for up to half a day in the field.** [EE, n=4 field users over 2 weeks + CC heuristic] ✅
Directly load-bearing for a freshness/staleness alarm: "nothing is happening" and "the instrument itself has failed" must be visually distinct states, by explicit prior design-heuristic consensus (Mankoff et al. 2003) and by an observed field failure.
Source: `faculty.washington.edu/garyhs/docs/matthews-BOOK-evaluation.pdf` · primary (book chapter) · 2009.

**G2. There's a structural tradeoff between subtlety and criticality — a display quiet enough not to distract is, by that same fact, unable to carry very important information.** [CC — argued from an activity-theory framework, not measured] ⚠ argued, not measured
The chapter's own honest admission: "compelling applications of peripheral displays are difficult to create." Their framework makes criticality an explicit design axis (high criticality → awareness dominates; aesthetics recede) but this is a design-theoretic claim, not a measured result.
Source: same as G1.

**G3. In a controlled lab dual-task study, literal text (a small ticker) beat an abstract color-coded display on recall, with identical self-reported awareness — but this result was contradicted by the same authors' own field study.** [EE, n=26 lab / n=4 field, single small study, explicitly non-replicating across settings by the authors' own account] ⚠ contradicts across settings, say so if cited
Lab: Ticker mean 3.2/5 vs. Orb mean 1.9/5 correct recall (t(24)=−2.19, p=0.038), same self-reported awareness (2.9/5 both). The authors state this laboratory result was contradicted by their own field deployment and speculate it's a learnability/exposure artifact — i.e., literal text may win under short unfamiliar exposure, with the gap closing (or reversing) with practice.
Source: same as G1.

**G4. Two field-observed perceptual failure modes: magnitude-as-luminance was too subtle to read, and a low-contrast text strip habituated away within days.** [EE, n=4 field users, qualitative/interview-derived — genuinely small] ⚠ n=4
Users could not read a count off brightness/color-intensity changes. A separate user reported the status strip "blended in too well with the background" and stopped noticing it after a few days. A third user explicitly asked for the display to be made *more* distracting, because subtle scrolling wasn't capturing her attention at all.
Source: same as G1.
This directly supports the claim that a truth rendered in low-contrast chrome is not "said" — but the failure mode here was contrast and self-announcement, not information content.

**G5. Expert heuristic evaluators preferred event-driven notification over constant ambient cycling — but the same evaluation method could not determine the *correct* level of distraction, and a real field user's stated preference contradicted the experts.** [CC, n=6 evaluators — and the authors name this method's own limit] ✅ as an honest finding about the limits of craft consensus
5 of 6 expert evaluators preferred event-driven notification over an "ambient" design that constantly cycled through five info slots, and recommended reducing animation/flicker/blinking generally. The authors' own stated limitation: "the heuristic evaluation was unable to suggest a correct level of distraction" — and a field participant (G4) wanted things flashier, not calmer. This is direct evidence that craft consensus and measured user need can diverge, and that the corpus does not license treating expert intuition alone as sufficient here.
Source: same as G1.

### H. Motion, distractors, and detection cost in a moving scene (closest lab analogue to "reading a chip while the globe rotates")

**H1. Background/distractor motion delays detection of a feature change in a concurrently monitored item — an ~80ms cost with a large effect size.** [EE, N=30, peer-reviewed] ✅
Mean RT to detect a 30° orientation change: 439ms (SD 58) stationary display vs. 519ms (SD 69) moving display, F(1,27)=97.52, p<0.001, η²p=0.783.
Source: `cognitiveresearchjournal.springeropen.com/articles/10.1186/s41235-021-00312-2` · primary · 2021-06-26.

**H2. Motion of the target item itself is a second, additive cost on top of surrounding motion — the two disruptions don't collapse into one.** [EE, N=31] ✅
Full ordering: pure-static 544ms < mixed-static 580ms < mixed-moving 623ms < pure-moving 635ms, F(3,84)=25.53, p<0.001. Directly designable: an alarm/status chip should itself stay motionless even when everything around it (the globe) is moving — being still is not enough if it's embedded in a moving frame, and being embedded in a moving frame costs something even before the chip itself animates.
Source: same as H1.

**H3. Density and motion interact multiplicatively, not additively — each additional monitored item costs more when the scene moves.** [EE] ✅
Static search slope 9.7ms/item (2–8 items), F(6,162)=23.24, p<0.001; the motion penalty grew at larger set sizes rather than staying constant.
Source: same as H1.

**H4. "Motion silencing" — the mechanism proposed for H1–H3 — is that feature changes on moving objects are perceptually attenuated, extended here from hue/luminance/size/shape to orientation.** [TP/EE, names a mechanism citing Suchow & Alvarez 2011, extends it to a new feature dimension] ✅
Source: same as H1.

**H5. Boundary condition the authors state themselves: this study measured response time only, with no target-absent trials and no accuracy/miss data — it establishes a latency cost, not a miss-rate.** [scope note, authors' own] ✅ — worth keeping attached to H1–H3 whenever they're cited
Source: same as H1.
The paper's own applied recommendation is workstation-level (limit the number of monitored screens), not pixel-level — don't over-read a per-pixel design rule out of a per-workstation recommendation.

---

## 4. Dead ends — stated plainly, per the brief's ask

These are angles the scope decomposition explicitly targeted that produced nothing usable, or produced something adjacent-but-not-the-thing:

- **Text reflow / in-place length-change as its own measured variable.** Not found anywhere in this corpus. This is the single most important gap relative to vivarium's actual defect. See §2 Q1 for the best available proxy evidence, all of it inferential.
- **Fitts's law / steering law transfer to gamepad, joystick, or gaze pointing.** Explicitly named in the scope decomposition's contrarian angle; zero sources in the fetched 25 address it. The Rioul source (§D5) covers Hick's law's misuse and boundary conditions thoroughly but says nothing about Fitts.
- **Miller's 7±2 vs. Cowan's 4±1 working-memory-capacity dispute.** Also explicitly named in scope; zero dedicated sources returned. The closest adjacent number is Wolfe GS6's 5-item search-model capacity (§F8) — a different mechanism (visual search architecture, not verbal/visual working memory) from a source I've already discounted for reliability. Do not treat F8 as answering this question.
- **Pousman & Stasko's own ambient-display taxonomy paper.** Named explicitly in the scope decomposition's query string; what was actually fetched instead was Matthews et al.'s evaluation-heuristics book chapter (§G) and Stasko's own later ticker/blast/fade experimental paper (§B1–B4) — good sources, but not the taxonomy paper that was the original target.
- **10-foot-UI guidance, tabular vs. proportional figures, digit jitter.** Named in the original query design targeting typography research; no fetched source addresses any of the three.
- **Durlach's change-blindness review** (`tandfonline.com/.../s15327051hci1904_10`) — full text returned a 403; only the abstract was accessible. Its claims (change detection failing at visual transients; attention-location, not display-presence, governing detection; the recommendation that change-detection be engineered as a system function rather than left to operator vigilance) are individually plausible and consistent with the rest of this corpus, but this source functions only as a *pointer* to the underlying primary literature, not as substantiated content in its own right — I have not listed its individual claims above for that reason, beyond noting them here.
- **`scholars.unh.edu/ccom/127/`** — a pure bibliographic catalog record for Ware's *Information Visualization*, no chapter content, explicitly self-described in the harvest as containing "no perceptual findings, no numeric thresholds." Its only real finding is a bibliographic hazard: the DOI printed on the record is a chapter-18-specific Elsevier DOI, not the book's, and would be a miscitation if copied as-is. Nothing else to report.

---

## 5. Quick-reference numbers table

Everything below already appears graded above; this is purely for scanning. TP/EE/CC/REV as in §3; ⚠🚩 repeated where relevant.

| Number | What it bounds | Grade | §/Flag |
|---|---|---|---|
| ~4–5° clear space needed around a chip at 10° eccentricity (Bouma's law, 0.4–0.5× eccentricity) | layout spacing | TP+EE | F4 |
| ~2:1 radial vs. tangential crowding anisotropy | chip stack orientation | EE | F5 |
| Peripheral target needs only ~4× foveal size at 10° (not the ~16× the popular blur-demo implies) | chip sizing | EE | F6 |
| FC-vs-required-contrast correlation r=.93; FC-vs-search-time r=.74–.83, ~55–69% of variance explained | how much "room" a background leaves for a chip | TP+EE | D1 |
| Desaturating background cuts search RT 20–29% independent of edge count | color-variability cost, independent lever from clutter-by-element-count | EE | D2 |
| Hick's-law real-task slopes: 32/8/4 ms/bit; envelope ~1–4 bits (2–16 alternatives) | # of chips vs. decision latency (turns out: basically free) | TP+EE | D5 |
| Ticker 54.3s vs. blast 33.6s vs. fade 35.5s monitoring latency (~65% ticker penalty) | traveling-motion cost for a status readout | EE | B1 |
| Small (70-char) vs. normal (160-char) panel: faster detection, slower browse, up to ~20s/2min | panel-size vs. noticing tradeoff | EE | B2 |
| Cueing: P(A) 0.88→0.64 with 100%-reliable "look here" cue; 45.5% sustained FA rate even after disclosing 75% reliability | cost of an overlay that asserts things | EE | C6 |
| Go-around latency 5.5–6.7s (head-down) vs. 7.2–9.1s (head-up, non-conformal) | overlay cost for unexpected events | EE-secondary | C1 |
| Overlay detection: helps expected events, costs unexpected ones (Z=1.968, p<.026, 18-study meta-analysis) | the single sharpest boundary condition for an anomaly-hunting instrument | EE-secondary | C2 |
| Contrast ratios 1.17–1.6:1 (below WCAG 4.5:1) already sufficient to speed symbology detection without costing scene-scanning | brightness ≠ the capture mechanism; clutter/grouping is | EE-secondary | C5 |
| Onset capture: 42 vs 11 ms/item single-task → 33 vs 25 ms/item (n.s.) under concurrent auditory load | onsets are not a free win under operator load | EE | A1 |
| Color singleton: 45 vs 30 ms/item single-task → 40 vs 12 ms/item (stronger) under load | sustained-difference encoding may beat transient under load | EE | A1 |
| Motion onset > new object's suppression of it; continuous motion worse than static (d=.76–1.18) | don't perpetually animate a status widget | EE | B5 |
| Jerky (8–17Hz) redraw captures attention; smooth (33–100Hz) does not; active ingredient = 0.26°–1.05° displacement | frame-rate/step-size design parameters for a deliberate alarm motion | EE-secondary | B6 |
| WCAG/APCA disagree on ~47% of black-text-passing backgrounds; on ~63% more backgrounds pass for white text under APCA | contrast-standard choice materially changes what "passes" | TP (computed) | E1 |
| Red-on-black: WCAG 5.3:1 passes AA, APCA Lc=40 fails | concrete alarm-color failure case | TP (computed) | E3 |
| 7:1 (video-see-through) vs. 1.6:1 (optical-see-through) minimum luminance ratio | applicable floor for a rendered-monitor globe (VST case: 7:1, higher than WCAG 4.5:1) | ⚠🚩 second-hand sourced | E12 |
| 66% of 171 operators missed a salient stimulus in their attentional focus; expertise irrelevant (p=.746); duration irrelevant (2 vs. 43 min, p=.665) | training/experience does not fix this; must be structural | EE | (PMC3897661, cited in A/C discussion) |
| AR text detection odds: 3.04×/4.45×/5.42× higher with no/1/2/3 lines of overlay text | monotonic cost of on-screen text volume | EE | E9 |
| 16/16 experienced operators missed a large deleted terrain feature while actively searching for exactly that class of defect | boundary condition on "a trained eye is the fastest detector" | EE (n=16, single event) | C7 |

---

## 6. Claims I'd flag as most suspect if you're tempted to act on them without a second check

Ranked by how much the framing seems to outrun the evidence, worst first:

1. **E5/E7 (github wcag3 issue 29)** — advocacy from an interested party (APCA's creator) inside an *unresolved* standards dispute, characterizing a competing standard's scientific basis as nonexistent. The structural/algebraic claim (WCAG's formula is polarity-blind) is checkable and probably fine; the characterization of consequence and the "no scientific basis" language for SC 1.4.11 is one side's brief, not a finding.
2. **E12/E13 (Gabbard "billboard"/7:1/1.6:1 numbers)** — sourced to an uncited, unfetched 2024 review rather than to the dissertation the URL points at. This is exactly the indirection shape that produced fabrications elsewhere in this run. Check the actual Gabbard 2006/2007 papers before using the specific numbers.
3. **F8 (Wolfe GS6 capacity=5 model parameter)** — from the source with the worst track record in this run (3/5 siblings refuted); it's a simulation-architecture parameter being read as an empirical human-capacity number.
4. **C7 (16/16 missed the missing mountain)** — a genuinely striking finding, but it's a single-event existence proof (n=16, one deleted-terrain event, one trial), not a rate. Cite it as "this happened once, dramatically," not as "operators miss X% of structural deletions."
5. **G3 (Ticker beats Orb on recall)** — the authors' own field study contradicted their lab result; don't cite the lab numbers without the contradiction attached.
6. **A2/B6/G5** — not suspect exactly, but each is a review or secondary synthesis citing older primary work I have not independently pulled. Treat as "consistent with a real literature," not as directly verified numbers, until someone chases the originals (Yantis & Jonides; Theeuwes et al. 1998; Sunny & von Mühlenen 2011/2014; Mankoff et al. 2003).

None of these are "probably fabricated" the way the 8 refuted claims turned out to be — I have no positive evidence any of them are wrong. They're flagged because the *shape* of their sourcing matches the shape that produced fabrication elsewhere in this exact run, and that pattern-match is worth more than my confidence in any individual reading.

---

## 7. What I'd trust most, if forced to rank

If you only take five things from this whole file into a segment:

1. **Feature Congestion (D1–D4)** — the one genuinely principled, computable answer to "how much may this display say," and it's runnable against vivarium's actual rendered frames rather than borrowed as a threshold.
2. **The tunneling/criterion-shift findings (C6, C7)** — an overlay that asserts something gets believed over the raw scene, doesn't recalibrate when its own unreliability is disclosed, and (existence-proof-grade) a fully-searching, fully-warned, fully-equipped operator still missed a large deleted terrain feature at 16/16. This is the sharpest available check on the assumption that the explorer's human-eye instrument is self-correcting.
3. **Motion-onset vs. continuous-motion vs. new-object ranking (B5, B6)** — falsifies "animate it to make it noticeable" as a blanket rule, and gives frame-rate/displacement-size parameters if a deliberate motion cue is built.
4. **The reflow gap itself (§2 Q1, §4)** — worth stating as an open question in whatever segment cites this work, not papered over with adjacent findings dressed as direct evidence.
5. **The same-source discount pattern (§0)** — a way of reading, not a fact, but the most transferable output of this whole exercise: in this run, distrust the claim's headline sentence and its "design consequence" more than its quote, and treat every claim from a source with a refuted sibling as inheriting some of that source's risk.
