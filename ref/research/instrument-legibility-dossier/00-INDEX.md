# Instrument legibility — dossier index

*Established 2026-07-31. **Reference-grade.** No vivarium claims live here. This reports what external literature says, with each finding's verification status attached, so a reader can check it rather than take it on trust. What — if anything — is worth carrying further is not decided here (§5).*

## The question

What is actually known — theoretically or empirically — about making a real-time instrument legible to a human operator? Joseph's framing, which is better than any restatement: the elements *"orthogonal or adjacent to gameplay that are 'traditional' HCI/UX. Principled instrument-panels, for example."* Not game feel, onboarding, difficulty, or motivation. The layer an avionics or SCADA designer would recognize, wearing game clothing.

The immediate consumer is `vivarium explore`'s chrome, whose attention failure is diagnosed in `#disc-explorer-human-chrome`.

## The ladder

Four documents, one whole, **ordered by half-life** rather than by abstraction. The two correlate, but half-life is the operational axis: it tells you what to re-check and when, and it makes staleness self-announcing.

| # | Document | Scope | Half-life | Maintenance |
|---|---|---|---|---|
| 01 | [Perception and attention fundamentals](01-perception-and-attention-fundamentals.md) | Domain-independent. How a human eye acquires a display. Crowding, clutter, capture, contrast, typography. | **Decades.** Bouma 1970 is still current. | Re-check only on a genuine replication crisis in vision science |
| 02 | [Instrument and HUD affordances](02-instrument-and-hud-affordances.md) | Domain-specific, engine-independent. Alert tiers, mode indication, staleness annunciation, overlay-over-scene, always-on vs on-demand. | **Decade.** Regulations amend; the mechanisms don't. | Re-check when a cited standard revises |
| 03 | [Implementation concerns](03-implementation-concerns.md) | Engine-level durable knowledge. What Bevy's architecture forces, permits, and forbids — the *why*, not the version. | **Per major release.** | Re-check on Bevy major |
| 04 | [Implementation survey](04-implementation-survey.md) | Dated inventory. Crate versions, compatibility, API surfaces. | **Months.** Explicitly perishable. | It is a table with dates. Re-run the checks; don't reason from it stale |

Read 01 → 04 to learn the domain. Read 04 → 01 to answer "can we build it." Neither order is a dependency graph; `depends:` in core carries that.

## How to read any claim here

Every claim carries a grade:

- **[T]** theoretically principled — a derivable model behind it
- **[E]** empirically established — measured; population, context, and effect size where known
- **[C]** craft consensus — widely practiced, thinly evidenced
- **[REV]** review/secondary — synthesizes others' data rather than measuring

And one reading discipline, which is the most useful thing this corpus produced about itself:

> **The verbatim quote is more trustworthy than the sentence introducing it, and that sentence is more trustworthy than any "design consequence" appended after it.**

Not a platitude — the measured failure shape here. Where claims were checked against their own cited primaries, the quotes were nearly always accurate and the defect sat in the framing sentence or in an inference presented in the same voice as a measured finding.

**Two populations, two confidence levels.** Of ~388 claims extracted across the three runs (≈394 by direct count of `appendix/harvest/`, now that those files exist — see below), **49 went through three-vote adversarial verification** against primary sources; the 25 that survived are marked `[VERIFIED 3/3]` and are the strongest tier available. The remaining **~339 were never checked.** They are the bulk of what follows, they are individually graded and flagged, and **17 of the 49 comparable claims that were checked did not survive.** Treat an unvoted claim as a well-sourced lead, not a settled fact.

**A tag-count caveat, added 2026-07-31 (external audit finding):** `grep`-ing the ladder documents for `[VERIFIED 3/3]` returns more than 25 occurrences (01≈8, 02≈3, 03≈17, 04≈4) — more than the 25 unique adjudicated claims, because one adjudicated claim is often split across multiple entry sentences in the distilled prose. Not a miscount of the 25, but a reader who greps for confidence-mass will over-count it. Two of those 25, the Nikolic et al. (2004) findings in 01 §2.5 and 02 §3, additionally carried the plain tag when only identity/abstract was verified — retagged `IDENTITY/ABSTRACT ONLY` this pass; see those sections.

Refuted claims are **retained and marked**, not deleted — a later reader who does not know what was already tested and killed will re-derive it from the same plausible-looking source.

## Verification debts — one ledger for all four parts

**Deliberately not split across the four documents.** These cut across the ladder, and fragmenting the ledger would make each part look cleaner than the evidence is. Ranked; none known-wrong, all sourced in a shape that produced fabrication elsewhere in this corpus.

1. **EEMUA 191 alarm-rate ceiling.** Two secondary sources disagree by 2× — ~12/hr long-term average (18/hr action limit) vs ≤1 per 10 min (=6/hr) steady state. Neither is the primary standard. This is precisely the number a density claim would want. *Hypothesis, unverified:* EEMUA may define acceptability *bands* rather than one ceiling, which would make both real. **Update 2026-07-31:** both external audits independently searched industry secondary literature (ProcessVue, Emerson/ChemEng, ASM, ISA-18.2 KPI tables) and found convergent support for the bands reading — ≤1/10 min as "very likely acceptable," ~12/hr as a separately named "maximum manageable" level. The hypothesis is *strengthened*, not confirmed: the primary standard (EEMUA 191 itself, or ISA-18.2) still has not been opened by anyone in this chain. Check before citing either number.
2. **Gabbard AR contrast numbers** — "billboard beats colour-tuning," 7:1 video-see-through / 1.6:1 optical-see-through floors, "1px outline suffices." Attributed in-corpus to a **2024 review that was never fetched**, not to the dissertation the URL points at. Go to Gabbard, Swan & Hix (*Presence* 15(1):16–32, 2006) and Gabbard et al. (IEEE VR 2007, 35–42).
3. **Sarter & Woods 65% / 15% aborted-takeoff figures.** Entire extraction is secondary-summary-derived; even *which* of their papers it comes from is unconfirmed. Most quotable number in that area, least verified.
4. **~8 percentage-point miss-rate increase per +10° eccentricity** — flagged by the extraction itself as coming from downstream citing literature, not the primary.
5. **APCA's standing.** Not an adopted W3C standard; had not cleared the WCAG 3 subgroup's own peer-review precondition. Its creator maintains the "independent peer reviews" bibliography, which is overwhelmingly blog posts and talks — and one entry is the same self-published article several contrast numbers here come from. Credible alternative under active contestation; **not** the settled successor to WCAG.
6. **Two sources are burned.** Healey's *"Perception in Visualization"* page went 5-for-5 refuted — do not cite it from this corpus at all. Wolfe's *Guided Search 6.0* went 3-of-5; its survivors are discounted, and its "capacity = 5" is a **simulation parameter**, not a measured human limit.

## Gaps — what nobody has measured

- **Text reflow / in-place length-change as its own variable.** This is the literal defect `#disc-explorer-human-chrome` exists to fix, and the experiment has not been run. The case against reflow is built by extension from motion-type findings. State it as inference.
- **A chip beside a large, slowly rotating, multi-hued sphere.** The closest results measured static backgrounds with expected cues, or moving backgrounds without chrome. The specific case is unmeasured.
- **No general density ceiling.** Two separate reviews say so explicitly.
- **Not covered by the fetched sources:** Fitts's/steering-law transfer to gamepad or gaze; Miller 7±2 vs Cowan 4±1; Pousman & Stasko's ambient-display taxonomy; 10-foot-UI guidance; tabular-vs-proportional figure research; digit jitter.

## Provenance, and what the production method implies

Three `deep-research` workflow runs on 2026-07-31 (scope → parallel search → fetch/extract → 3-vote adversarial verification → synthesis), **stopped mid-verification** at ~244 agents by Joseph's direction. Scope, search and extraction had completed in all three; 49 claims were fully adjudicated. Journals were harvested and synthesized in-session plus three Sonnet passes over the unverified pools.

| Run | Refuted | Contested | Survived | Adjudicated |
|---|---|---|---|---|
| Perception / attention | 8 | 1 | 5 | 14 |
| Practice / safety-critical | 1 | 5 | 4 | 10 |
| Bevy | 8 | 1 | 16 | 25 |
| **Total** | **17** | **7** | **25** | **49** |

The refutation rate is *not* uniform across runs, and the variation is informative about which sources to trust here rather than about the method: the perception run leaned on a course web page as though it were primary (that source alone produced 5 of its 8 refutations, and is burned — debt 6 above); the Bevy failures were almost entirely version attribution in a fast-moving codebase, caught by reading vendored crate source; the practice run's sources were regulations, where quotes verify verbatim but scope-glosses are arguable — hence 1 refutation and 5 *contested*.

Two claims were **fabricated outright** against their cited primaries: a "70%" statistic appearing nowhere in the paper it was attributed to, and a categorical about peripheral colour/shape detection that the cited source explicitly declines. Both are excluded; they are recorded because they mark the two source-shapes that produced them — a secondary teaching page cited as primary, and a review cited at one remove.

**Raw material** is preserved in two layers under [`appendix/`](appendix/), and this index previously overstated what the first layer contains — corrected 2026-07-31 after two independent external audits both caught it (`AUDIT-2026-07-31-grok.md` §3.4, `AUDIT-2026-07-31-grok-2.md` I-3):

- `appendix/synthesis-*.md` — three **curated syntheses**, not full graded pools. The perception synthesis explicitly selects "roughly the strongest half" of its unverified claims (~55 of 111); the practice synthesis covers close to its full unverified pool (~123 of ~133 mined); the Bevy file is a capability answer built from 25 adjudicated claims, not a pool at all. Plus `cross-run-synthesis-2026-07-31.md`, a further synthesis applying findings to vivarium — see its own banner on why it is a further hop, not a primary layer.
- `appendix/harvest/` — added 2026-07-31, previously sitting in a session-scoped temp directory that would have evaporated and been unrecoverable. These three files are the actual raw extraction pools: every source fetched, every claim mined with its verbatim quote, URL, date, and quality tag, before any selection or synthesis — 125 claim-bullets (perception, 25 sources), 139 (practice, 29 sources), 130 (Bevy, 26 sources); ≈394 total, close to the ~388 estimate below and now independently recountable rather than remembered. **This is the layer both external audits identified as the largest checkability gap** — without it, the ~339 unvoted claims could not be re-checked by anyone. It now can be.

For anything load-bearing: read the harvest entry first, then the primary; the synthesis files are useful framing but are one hop further from the evidence than they read as.

## 4a. Distance from source — read the appendix, not the summary

**No human-factors primary source was read by the session that assembled this index.** The chain for human-factors claims is: primary → extraction agent → synthesis agent → this document; every such citation is at two or three removes, including the ones carrying page numbers, F-statistics, DOIs, and source-line references. **This does not describe 03/04's Bevy findings**, which were verified by reading vendored crate source directly on this machine — a skimming reader applying this banner to the whole dossier will under-trust the strongest-evidenced layer (noted 2026-07-31, external audit finding).

That ordering matters because **each hop can shed a qualifier, and at least one demonstrably did.** The overlay-expectancy meta-analysis result was carried by the synthesis layer with an explicit warning that the source document misprints its own significance sign; the summary layer dropped the warning and shipped the statistic clean. It was caught and restored, which is the only reason it is documented here rather than propagating.

Practical consequence: **the `appendix/harvest/` and `appendix/synthesis-*.md` files are closer to the evidence than any summary of them.** For anything load-bearing, read the harvest entry, then the primary. Treat this index and its distillations as a map of where to look, not as the finding. **This does not extend to `appendix/cross-run-synthesis-2026-07-31.md`** — that file is a further synthesis hop applying findings to vivarium, not a primary layer beneath 01–04; see §5 and its own banner.

## 5. Scope of this dossier

**This applies to the ladder (00–04) and to `appendix/synthesis-*.md` and `appendix/harvest/`.** They report research. They make no claims about vivarium, propose no changes to anything, and take no position on what — if any — of it is worth carrying further.

`appendix/cross-run-synthesis-2026-07-31.md` is the one exception, and it is marked as one: it *does* argue a specific design verdict about the CARVE chip specification. Two external audits (2026-07-31) flagged that this dossier previously routed readers to "the appendix" as uniformly closer to the evidence without calling out that the cross-run breaks the neutrality this section states — corrected above and in that file's own banner. Read it as a smart, hedged brief built from this research, not as a finding co-equal with 01–04.

Every finding carries its source, its date, its quality tag, and whether it survived verification. The raw harvest pools in [`appendix/harvest/`](appendix/harvest/) hold the verbatim quotes and URLs behind each one, so any line here can be checked against what the source actually says rather than taken on this document's word.

---

**Audited 2026-07-31** by two independent de novo external reviews (Grok 4.5, run separately, no knowledge of each other): [`AUDIT-2026-07-31-grok.md`](AUDIT-2026-07-31-grok.md) and [`AUDIT-2026-07-31-grok-2.md`](AUDIT-2026-07-31-grok-2.md). Both verified load-bearing claims against primaries (PDFs, eCFR, crates.io, vendored Bevy source) and are the authority behind the corrections dated 2026-07-31 throughout this dossier. [`REPAIR-2026-07-31.md`](REPAIR-2026-07-31.md) records what was fixed, what was deliberately left open, and why.
