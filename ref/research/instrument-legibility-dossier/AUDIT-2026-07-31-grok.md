# De novo audit — instrument-legibility dossier

**Auditor:** Grok 4.5 (xAI), session 2026-07-31  
**Target:** `/Users/josephwecker-v2/src/arch/vivarium/ref/research/instrument-legibility-dossier/`  
**Parts under audit:** `00-INDEX.md`, `01`–`04` ladder documents, `appendix/` (four files)  
**Method:** Independent re-read of all nine files; adversarial spot-checks of load-bearing claims against primary sources (PDFs, eCFR/LII regulation text, FAA AC 25-11B, crates.io API, vendored Bevy 0.18.1 source on this machine). Scratch only under the assigned temp directory. **No dossier files other than this audit were modified.**

**Adjudication ideal (stated so findings can be graded against it):** A complete, usable, completely truthful research report on making a real-time instrument legible to a human operator — domain-independent perception, domain-specific instrument/HUD practice, and (separately, perishably) engine implementation — such that a later reader can (a) trust what is marked verified, (b) know the distance of every other claim from primary evidence, (c) re-check any line without reconstructing the production pipeline, and (d) use the report as a map without mistaking map for terrain or design recommendation for measurement.

---

## 0. Executive judgment

**This is one of the better LLM-mediated research dossiers I have seen on method.** The half-life ladder, the quote-over-gloss reading rule, the retention of refuted claims, the explicit “writer read no primary” banner, the verification-debt ledger, and the Bevy source-read discipline are all load-bearing and correctly oriented toward truth rather than persuasiveness.

**It is not yet the ideal.** The ideal fails in three places that matter:

1. **Truth under pressure.** A minority of high-traffic numbers are slightly rounded or scope-inflated; one open question is now closable and currently left open in the wrong place; the cross-run appendix makes design inferences that outrun the measurement base while the index still claims non-position-taking.
2. **Provenance completeness.** The index promises “graded claim pools (~110 unverified claims each)” in `appendix/`; the perception appendix is an explicit half-selection; the original harvest files are not present in this tree; so the bulk of ~339 unvoted claims cannot be re-checked from the dossier alone.
3. **Usable completeness for the stated question.** The report is strong on attention-capture, clutter models, aviation/nuclear annunciation, game-HUD empirics that puncture craft lore, and Bevy 0.18 mechanism. It is thinner or silent on several areas a complete instrument-legibility ideal would own (listed in §6), some of which it correctly flags as gaps and some of which it does not.

**Net grade against the ideal:** **B+ / strong reference-grade substrate, not settlement.** Safe to use as a *map of where to look and what not to re-derive*; unsafe to treat unvoted bulk, design “consequences,” or the cross-run’s vivarium-facing recommendations as established fact. The dossier’s own reading discipline is the correct one — and the dossier does not always obey it in the appendix that people will actually act on.

---

## 1. Scope of what was checked

| Layer | Coverage in this audit |
|---|---|
| Full text of all 9 dossier files | Read end-to-end |
| Bartram, Ware & Calvert 2003 (*IJHCS*) PDF | `pdftotext`; key tables and F-stats checked |
| Wolfe GS6 2021 PDF | Orientation/guidance passage on p.1065 checked verbatim |
| 14 CFR 25.1322 | Cornell LII full text |
| FAA AC 25-11B | Full PDF (`AC_25-11B.pdf`); key clauses §5.8.3, §5.10.3, §5.11.3–4, §6.2.1.7, §6.3.3.1 |
| Bevy 0.18.1 vendored crates | `bevy_text`, `bevy_ui`, `bevy_camera`, `bevy_sprite`, `bevy_ui_widgets`, `bevy_render` on disk |
| crates.io API | `bevy` 0.18.0/0.18.1/0.19.0 dates; ecosystem crate versions & bevy deps |
| EEMUA 191 secondary literature | Multiple industry secondary sources re: dual thresholds |
| APCA / WCAG 3 status | Secondary status pieces through Apr 2026 |
| Rosenholtz 2007 JOV | Abstract + definition confirmed; full-text r-values not re-extracted (paywall/HTML gate) |
| Nikolic et al. 2004 | Identity and abstract-level framing via secondary catalogs (body still paywalled) |
| Harvest source files | **Not found** under vivarium or the dossier tree |

Claims not listed in §4–§5 were not independently re-verified. Absence of a finding is not a pass.

---

## 2. What the dossier gets right (keep)

These are not soft compliments; they are structural features that survive the idealization test and should not be “cleaned up” away.

### 2.1 Epistemic machinery

- **Half-life ordering (01 decades → 04 months)** is the correct operational axis. It makes staleness self-announcing and prevents a Bevy API fact from contaminating a Bouma-law claim.
- **Quote > framing sentence > design consequence** is stated, illustrated by actual failure modes in the corpus, and is the single most valuable product of the production method. It is also the rule the cross-run most often breaks (see §3.3).
- **Refuted claims retained** (01 §10, 02 §9, and Bevy refutations in 03) prevent re-derivation of burned material. Healey 5/5 and the GS6 partial burn are correctly handled.
- **Distance-from-source banners** on 00, 01, 02 are honest and rare. The index’s §4a (overlay meta-analysis sign-misprint dropped then restored) is a model self-correction.
- **Verification-debt ledger** (00 § debts 1–6) is the right shape: ranked, none silently resolved, each with a chase path.

### 2.2 Content that survives re-check

Load-bearing survivors I independently confirmed or strengthened:

| Claim region | Status this audit |
|---|---|
| Wolfe GS6 guidance vs discrimination (0.5° detect, ~10–15° guide; categorical uniqueness; coarse representation) | **Confirmed** against PDF p.1065; wording is faithful |
| Bartram 2003 motion vs colour/shape peripheral detection; type ≫ frequency distraction; F(7,84)=86.89 / F(1,12)=40.18 | **Confirmed** against full PDF (minor rounding — §3.1) |
| 14 CFR 25.1322(c)(2) two-senses for Warning/Caution; (e) colour reserved; Amdt. 25-131 Nov 2 2010 | **Confirmed** LII |
| 25.1322 gloss overreach pattern (quote real, claim inflated) | **Confirmed architecture** of the regulation; the dossier’s refutation grounds on (b)/(d)(2) are well-formed |
| AC 25-11B §5.11.3.4 text-change ban; blink 0.8–4.0 Hz; six-part part-time test §6.3.3.1; failure-flag co-location §6.2.1.7 | **Confirmed** against full AC PDF |
| Bevy `detect_text_needs_rerender` / needs_rerender remeasure comment at `text.rs` ~60–67, function ~744 | **Confirmed** vendored 0.18.1 |
| `FontSmoothing::None` pixel-snaps glyph geometry (`pipeline.rs` 399–411) | **Confirmed** |
| `world_to_viewport_core` adds `target_rect.min` (camera.rs ~531) | **Confirmed** |
| `Outline` “do not take up space in the layout”; `UiTransform` exists; `BorderRadius` is a `Node` field not a Component | **Confirmed** |
| `Text2dShadow` exists in `bevy_sprite` 0.18.1; `TABULAR_FIGURES` on `FontFeatureTag` | **Confirmed** |
| `Popover` shipped in `bevy_ui_widgets` 0.18.1 | **Confirmed** file exists |
| Bevy 0.18.0 = 2026-01-13; 0.18.1 = 2026-03-04; 0.19.0 = 2026-06-19 | **Resolved** via crates.io (see §3.2) |
| `bevy_ui_anchor` 0.11.0 → bevy ^0.18; 0.12.0 → ^0.19 | **Confirmed** |
| APCA not adopted W3C standard | **Confirmed and understated** — APCA was exploratory and removed from WCAG 3 drafts; still not the contrast algorithm as of 2026 WCAG 3 Working Drafts |

### 2.3 Honest gaps the dossier owns

Text-reflow unmeasured; globe+chip combination unmeasured; no general density ceiling; EEMUA figure unresolved *as stated*; Gabbard second-remove; Sarter/Woods 65%/15% do-not-cite; Fitts/Miller/10-foot-UI empty. These are correctly labeled gaps, not papered over. That is exactly what the ideal requires.

---

## 3. Truth defects (ordered by severity)

### 3.1 HIGH — Numerical rounding presented as source precision (Bartram)

**Where:** 01 §2.1 table; cross-run §1(a) table.

**Source (Bartram 2003, full text):** colour detection error averaged **5.5% near / 24% far**, not 6% / 25%. Motion is described as “nearly a 0% error rate,” not “<2%.” Latencies 2.3 s → 4.6 s (colour) and ~1.0 s (motion) and shape 2.0 → 4.4 s are correct. The F-statistics for distraction type/frequency are correct.

**Severity:** Low absolute error, high traffic — these are the most-quoted numbers in the dossier. Rounding is common but the dossier’s own brand is precision-with-provenance. State 5.5% / 24% or say “about 6% / 25%.”

### 3.2 HIGH — Bevy date inconsistencies left open when they are closed; one crate “latest” wrong

**Where:** 04 §1; appendix/synthesis-bevy §2.

**crates.io ground truth (checked 2026-07-31):**

| Release | `created_at` |
|---|---|
| bevy 0.18.0 | 2026-01-13 |
| bevy 0.18.1 | **2026-03-04** |
| bevy 0.19.0 | **2026-06-19** |

The dossier records “2026-03-02 **or** 2026-03-04” and “2026-06-18 **or** 2026-06-19” as unresolved. That was the right posture mid-run; it is no longer true. Picking one is not inventing a value when the registry is the adjudicator the survey itself names.

**Additionally:** 04’s ledger lists `bevy_feathers` **Latest release = 0.18.1**. crates.io newest is **0.19.0** (2026-06-19). The 0.18-compatible line is 0.18.1; the column conflates “latest absolute” with “latest on our Bevy line.” The survey’s own rule (“three facts: latest, 0.18-compatible, last activity”) is violated by its own feathers row.

### 3.3 HIGH — Scope contradiction: index neutrality vs cross-run design adjudication

**Where:** 00 §5 (“makes no claims about vivarium, proposes no changes… takes no position”) vs `appendix/cross-run-synthesis-2026-07-31.md` §1, §9, §12.

The cross-run:

- Declares the proposed CARVE red chip “close to the worst available choice” on four lines of evidence;
- Relocates “highest-leverage fix” to clutter headroom and `Text` write discipline;
- Lists segment-material candidates including “FE(5) needs revision.”

The cross-run *does* mark itself `msc/`-grade and non-canon, and some of its design inferences are carefully hedged (“probe-shaped gap, not a design answer”). But:

1. **00 points readers at the appendix as closer to the evidence**, which is true for the per-run pools and false for the cross-run’s design sections — those are *farther* from evidence than 01–04 because they are multi-hop syntheses plus vivarium application.
2. The four-line case against “red chip” **mixes tiers without saying so at the point of verdict**:
   - (a) Bartram colour-peripheral failure — [E], verified, but tests expected cues on a static field;
   - (b) red-on-black APCA — [T, computed], from a non-peer-reviewed APCA-advocacy-adjacent article, conditional on “if APCA is more accurate”;
   - (c) motion silencing — [E] for feature change *on moving objects*, extended by analogy to a chip *beside* a rotating globe;
   - (d) Nikolic green-box — [E] for a green outline mode annunciator under concurrent load, not for a red status chip.

The honest claim is: **colour-only peripheral status indication is a poor bet under several independent literatures; the vivarium-specific encoding is unmeasured; red is not specially convicted by Bartram (red and green both failed).** “Close to the worst available choice” is design rhetoric, not a finding.

**This is the same failure shape the dossier documented in itself** (headline oversteps quote). It reappears at the highest-leverage page.

### 3.4 HIGH — Provenance promise not met: full claim pools and harvest files absent

**Where:** 00 § provenance (“Raw material is preserved in `appendix/`: the three per-run graded claim pools (~110 unverified claims each…)”).

**Actual state:**

| Appendix file | What it actually is |
|---|---|
| `synthesis-perception-attention.md` | Explicitly **~55 of 111** selected claims (“strongest half”), not the full pool |
| `synthesis-practice-safety-critical.md` | Richer, closer to a full graded pool for the practice run |
| `synthesis-bevy-0.18.md` | Capability synthesis, not a 110-claim pool |
| `cross-run-synthesis-…` | Design-oriented synthesis |

Referenced harvest files (`harvest/run1-…`, `run2-…`, `run3-…`) are **not in this directory tree** (not under `ref/research/instrument-legibility-dossier`, not found under vivarium by name). Without them, the ~339 unvoted claims cannot be re-audited, and the “verbatim quote is more trustworthy” rule cannot be applied by a later reader for most of the corpus.

**Against the ideal:** a research report that advertises checkability must ship the checkable intermediate, or must not claim it is preserved here.

### 3.5 MEDIUM–HIGH — Frequency range overstated (Bartram)

**Where:** 01 §2.3 “Frequencies across the whole tested 1–3 Hz range were effective.”

**Source:** Experiment motion cues were SLOW (~1 Hz, 30 frames/s path) and FAST (~2 Hz, 15 frames/s path). The guidelines box later mentions “frequencies between 1 and 3 Hz” as a design recommendation region, not as the full tested factorial range. Contested-claim body conflates tested levels with guideline envelope.

### 3.6 MEDIUM — `bevy_egui` screenshot hazard left open where it is closable

**Where:** 03 §7; 04 §5; appendix/synthesis-bevy §7 and ledger.

**Dossier state:** “Whether the Bevy-0.18-compatible `bevy_egui` line postdates that fix was **not** determined — open either way.”

**Checkable fact (crates.io dependencies, 2026-07-31):**

| Crate version | Bevy | egui |
|---|---|---|
| `bevy_egui` 0.39.1 | ^0.18 | **^0.33** |
| `bevy_egui` 0.41.1 | ^0.19 | **^0.35** |

The appendix *already records* “egui pinned two minors behind (0.33 vs 0.35)” for the 0.18 line, but does not close the loop: if the screenshot fix lives in egui 0.35, then **0.39.x is pre-fix**. The open question in 03/04 should be closed: **for Bevy 0.18-compatible bevy_egui, the screenshot-bypass hazard remains the default assumption** unless a backported fix is demonstrated in 0.39.x source (not checked here). Leaving it “open either way” after having the version pin is the same shape as inventing uncertainty.

### 3.7 MEDIUM — Claim-count arithmetic is approximate and internally soft

**Where:** 00: “~388 claims… 49 adjudicated… 25 survived… ~339 never checked.”

**From appendix headers:** perception 125 mined / 14 adjudicated; practice ~133 / 10; Bevy 25 voted (mined total not stated as a clean number). 125+133=258; Bevy would need ~130 mined to reach 388 — plausible but not shown. Vote totals 14+10+25=49 match. Survivor counts 5+4+16=25 match the table.

**Ladder `[VERIFIED 3/3]` tag counts** (grep): 01≈8, 02≈3, 03≈17, 04≈4 — **more tags than 25 surviving claims**, because one adjudicated claim is split into multiple entry sentences. That is fine if stated; unstated, a reader who greps will over-count verified mass.

The dossier already corrected one miscount (perception brief said 10 refuted; recount 8). The aggregate ~388 should be labeled estimate or computed from harvest line counts once harvest is present.

### 3.8 MEDIUM — “No primary was read by the writer” is true of ladder distillation but not of synthesis passes

03 and appendix/synthesis-bevy correctly distinguish `[SOURCE-READ]` and synth-verified vendored reads. 00’s global “no primary was read by the session that assembled this index” is carefully scoped to the index. Risk: a skimming reader applies the weakest banner to the strongest layer (Bevy source-read findings), or the strongest banner to the weakest (cross-run design).

### 3.9 LOW — Minor Bevy API gloss risks

- **`BorderRadius` “no longer a separate component”** — correct (it is a field on `Node`, type still named `BorderRadius`, not `Component`). Fine for implementers; a casual reader might think the type vanished.
- **`TryStableInterpolate` “three implementors”** in the Bevy appendix harvest narrative — trait exists; I did not re-enumerate implementors. Not a defect unless wrong.
- **Default font as stripped FiraCode** — correctly tagged community-lead, not Bevy source.

### 3.10 LOW — Regulatory “binding vs guidance” is handled well; unvoted AC claims inherit lower risk than unvoted 25.1322 glosses

02 is careful. No new regulatory defect found in the clauses I re-read. The unvoted AC 25-11B material I checked is near-literal. That strengthens 02 §1.2 more than the dossier claims for it.

---

## 4. Completeness against the ideal target

### 4.1 Ideal target (auditor’s construction)

A complete instrument-legibility report for a real-time 3D-instrument + chrome system would cover, at minimum:

| Domain | Ideal coverage | Dossier coverage |
|---|---|---|
| Peripheral detection / capture | Motion, onset, colour, shape; load interactions | **Strong** (Bartram, Boot, Nikolic lineage) |
| Clutter as computable | Feature Congestion et al.; limits of density rules | **Strong** as method; r-values still unvoted |
| Crowding / geometry | Bouma, anisotropy, size vs isolation | **Present**, unvoted review-tier |
| Overlay / HUD over moving scene | Expectancy split, conformal vs screen-fixed | **Present**, mostly secondary-within-NASA-survey |
| Mode awareness / automation opacity | Sarter–Woods, FMA miss | **Partial**; best numbers do-not-cite |
| Alarm vs status vs notification | ISA-18.2 family, EEMUA | **Partial**; thresholds unresolved |
| Unreliability / staleness annunciation | Nuclear four-state, aviation failure flags, uncertainty viz | **Strong** as convergence pattern; legs unvoted |
| EID / constraint-based display | Founding + self-limits | **Good**, self-critical |
| Game HUD empirics | Diegesis, size/colour nulls, numeric vs bar | **Strong** puncture of lore |
| Contrast / typography for HUDs | WCAG vs APCA, AR-specific | **Weak/contested** by design of sources |
| Control / input ergonomics | Fitts, steering, gamepad/gaze | **Absent** (flagged) |
| Working memory / chunking for panels | Miller/Cowan | **Absent** (flagged) |
| Ambient / 10-foot / glance-only displays | Pousman–Stasko, living-room UI | **Absent** (flagged) |
| Temporal design of value change | Digit jitter, tabular figures, reflow | **Partial** (Bevy tabular path; reflow unmeasured) |
| Multi-operator / team instruments | — | **Absent** (not flagged) |
| Accessibility beyond contrast (colour-blind, low vision, motor) | — | **Thin** (one Few colour-blind figure) |
| Auditory/tactile channels for alerts | Required by 25.1322, little design detail | **Named, not developed** |
| Engine implementation (Bevy) | Mechanisms + perishable survey | **Best-evidenced layer** |
| Transfer functions: lab → operational display | Explicitly warned | **Warned, not modeled** |

### 4.2 Completeness score

Against the **stated** scope (external literature on instrument legibility for a chrome-over-globe instrument, plus Bevy 0.18): **high-B**. Against a **complete** idealization of the target: **mid-B**, with the largest holes either already self-flagged or adjacent (aural/tactile design, accessibility, multi-sensory alert construction, transfer models).

The report is more complete on *what not to believe* (game HUD lore, Healey page, main-branch Bevy docs, red-as-unmissable craft instinct) than on *what to build*. That is the right priority for a truth-first research phase — and it leaves the “usable ideal” only half-served.

---

## 5. Usability defects

### 5.1 Two products, one directory

There are effectively **two deliverables**:

1. **Ladder (00–04):** reference-grade, non-prescriptive, half-life ordered.  
2. **Cross-run appendix:** decision-support memo aimed at `#disc-explorer-human-chrome`.

They share a directory and 00 routes readers to the appendix as “closer to the evidence.” For the per-run syntheses that is often true; for the cross-run design sections it is false. **Usability fix:** either (a) move the cross-run to `msc/` with a hard banner that it is application, not evidence, or (b) split “evidence appendix” from “application memo” in the index table.

### 5.2 Actionability without false authority

Best usable artifacts:

- Bevy mechanism table (03 / appendix-bevy capability matrix) — **directly buildable**, source-backed.
- AC 25-11B six-part part-time test — **directly applicable** as a checklist (guidance, not law).
- Feature Congestion as a screenshot probe — **correct shape** for `#norm-probes-before-claims`, implementation not specified.
- Crowding/Bouma numbers — **designable**, still unvoted.

Worst usability failure mode: a future agent opens the cross-run, implements “not red / use motion / Feature Congestion,” and cites this dossier as having *established* those choices. The production method was built to prevent that; the packaging invites it.

### 5.3 Missing operational tools

The ideal usable report would ship or point to:

- A one-page “do not cite” list (partially present as debts + burned sources).
- A re-run script for 04 (partially present as curl/jq recipes — good).
- Harvest files or a claim database with quote + URL + grade + vote (absent).
- A Feature Congestion runner or reference implementation pointer (Rosenholtz MATLAB exists on MIT DSpace; not linked from the ladder).

---

## 6. Verification-debt ledger — auditor updates

| # | Dossier debt | This audit |
|---|---|---|
| 1 | EEMUA 191 dual figures | **Hypothesis strengthened.** Industry secondary sources (ProcessVue citing EEMUA 191 3rd ed. p.96; Emerson/ChemEng; ASM) support *bands*: ≤1/10 min steady-state as “very likely acceptable,” and ~12/hr long-term as a separate “maximum manageable” style figure. Both can be real without contradiction. **Still do not cite a single ceiling.** Primary still not opened. |
| 2 | Gabbard AR contrast numbers | **Unchanged.** Do not use. Chase 2006/2007 primaries. |
| 3 | Sarter & Woods 65%/15% | **Unchanged.** Do not cite. |
| 4 | ~8 pp miss / +10° eccentricity | **Unchanged.** Second-remove. |
| 5 | APCA standing | **Confirmed and slightly strengthened.** Not adopted; exploratory content removed from WCAG 3 drafts; WCAG 3 still does not settle on APCA as of 2026 status writing. |
| 6 | Healey burned / GS6 discounted | **Confirmed** for GS6 guidance-threshold claim (surviving material is real); capacity=5 is simulation parameter (`asynchronous diffuser has a capacity of five items`) — dossier correct. |
| **New** | bevy_egui 0.39 vs egui 0.35 screenshot fix | **Closable:** 0.39.1 depends on egui ^0.33 → treat hazard as **present** for Bevy 0.18 line until proven otherwise. |
| **New** | Bevy release dates 0.18.1 / 0.19.0 | **Closed:** 2026-03-04 / 2026-06-19. |
| **New** | Bartram 6%/25% | **Correct to 5.5%/24%** (or qualify as approximate). |

---

## 7. Spot-check ledger (compact)

| ID | Claim | Verdict |
|---|---|---|
| W-GS6-guide | 0.5° vs 10–15° guidance; categorical; colour similar | **Pass** (PDF) |
| B-2003-table | Colour 6%/25%, motion <2%, latencies | **Pass with rounding** → 5.5%/24%, ~0% |
| B-2003-F | Type F(7,84)=86.89; freq F(1,12)=40.18 | **Pass** |
| B-2003-Hz | Tested 1–3 Hz | **Fail soft** — tested ~1 and ~2 Hz; 1–3 is guideline envelope |
| CFR-1322-c2 | Two senses Warning/Caution | **Pass** |
| CFR-1322-e | Red/amber/advisory colours | **Pass** |
| CFR-amdt | Amdt 25-131, 75 FR 67209, Nov 2 2010 | **Pass** |
| AC-text | “text change by itself should not…” | **Pass** |
| AC-blink | 0.8–4.0 Hz | **Pass** |
| AC-parttime | Six criteria §6.3.3.1 | **Pass** (accurate paraphrase) |
| AC-flag | Failure flags at location of info | **Pass** |
| Bevy-rerender | needs_rerender + remeasure comment | **Pass** |
| Bevy-smooth | FontSmoothing::None snaps pixels | **Pass** |
| Bevy-viewport | + target_rect.min | **Pass** |
| Bevy-outline | Layout-free | **Pass** |
| Bevy-dates | 0.18.1 / 0.19.0 ambiguous | **Fail (stale open)** — resolvable |
| Bevy-anchor | 0.11.0 for 0.18 | **Pass** |
| Bevy-feathers-latest | Latest = 0.18.1 | **Fail** — latest 0.19.0 |
| Bevy-egui-open | Screenshot fix open | **Fail as left-open** — 0.39 → egui 0.33 |
| APCA-status | Not adopted | **Pass** (understated) |
| EEMUA | Unresolved dual figures | **Pass as posture**; bands hypothesis supported by secondaries |
| Cross-run-red | Red chip near-worst | **Fail as verdict** — over-combined tiers |
| Index-pools | Full ~110 pools in appendix | **Fail** — perception is half-selected; harvest absent |
| Index-neutrality | No vivarium position | **Fail relative to cross-run content** |

---

## 8. Production-method findings (meta)

The 35% refutation rate on voted claims (17/49) is itself a finding and is honestly reported. Failure modes cluster correctly:

- Perception run: secondary teaching page treated as primary (Healey).
- Bevy run: `main`/PR/docs.rs-latest as release fact.
- Practice run: regulation quote genuine, gloss overreaches.

**What the method did not catch (this audit’s additions):**

1. Rounding inflation on the most-cited table.
2. Leaving registry-resolvable dates and dependency pins “open.”
3. Design-synthesis layer reintroducing the headline-over-quote defect after the method was built to kill it.
4. Provenance claim that full pools live in appendix when they do not.

**Implication:** adversarial three-vote verification works for *individual claim–source pairs*. It does not automatically protect *synthesis narratives* or *inventory tables*. Those need a second pass of the same severity — which this audit partially is.

---

## 9. Recommendations (only if the dossier is revised)

Ordered by truth-impact, not effort:

1. **Correct Bartram table** to 5.5% / 24% (or mark approximate) everywhere it appears.
2. **Close Bevy dates** to crates.io: 0.18.1 = 2026-03-04, 0.19.0 = 2026-06-19.
3. **Close bevy_egui hazard** for the 0.18 line: egui ^0.33 ⇒ pre-0.35 fix; hazard stands.
4. **Fix bevy_feathers “latest”** column (0.19.0 absolute / 0.18.1 on 0.18 line).
5. **Relabel or relocate the cross-run** so it cannot be read as co-equal evidence with 01–04; rewrite the “worst available choice” verdict as a multi-tier inference with named gaps.
6. **Ship harvest files** into the dossier (or amend 00 to stop claiming the full pools are here); expand perception appendix to the full 111 or mark it selected.
7. **State that `[VERIFIED 3/3]` tag count ≠ 25 unique claims** (multi-entry expansion).
8. **Link Rosenholtz MATLAB** (MIT DSpace measures of visual clutter) if Feature Congestion is to be operationalized.
9. **Do not “promote” cross-run design recommendations into core segments** without the probes the cross-run itself names (CARVE detection under globe scan; Feature Congestion on capture frames; reflow isolation experiment).

---

## 10. Bottom line

| Criterion | Verdict |
|---|---|
| **Truthfulness of verified tier** | High — spot-checks largely pass; Bevy source-read layer is the strongest |
| **Truthfulness of unvoted bulk** | Unknown by construction; prior 35% kill rate is the correct prior |
| **Truthfulness of design synthesis** | Mixed — useful, sometimes sharp, repeatedly oversteps its own rules |
| **Completeness vs stated scope** | Strong with self-aware gaps |
| **Completeness vs full ideal** | Incomplete in input ergonomics, multi-sensory alert design, accessibility depth, transfer models |
| **Usability** | High for Bevy implementers and for “what not to trust”; hazardous as silent design authority |
| **Checkability** | Compromised by missing harvest / incomplete pools despite excellent banners |

**Adjudication:** The dossier is a **trustworthy map of a partially verified territory**, produced by a method that correctly distrusts itself. It is not yet a **complete, usable, completely truthful idealization** of instrument legibility — mainly because (1) most claims remain unvoted, (2) the application layer re-introduces synthesis overclaim, and (3) the provenance substrate needed to re-check the bulk is not actually in the tree.

Use 01–04 with the dossier’s own reading rule. Use the cross-run as a smart brief, not as evidence. Re-run 04 before any dependency pin. Do not cite EEMUA, Gabbard-via-review, or Sarter–Woods 65/15 without primary chase. Prefer Bartram’s actual 5.5/24 to the rounded table. Treat Bevy 0.18 + bevy_egui 0.39 as still screenshot-hazardous until proven otherwise.

---

*End of audit. Scratch materials under the assigned temp directory; only this file written into the dossier tree.*
