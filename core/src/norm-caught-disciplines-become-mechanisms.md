---
slug: norm-caught-disciplines-become-mechanisms
type: normative
status: robust-qualitative
stage: draft
depends:
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - norm-declaration-must-convict
  - form-core-view-wall
  - form-manifest-prescribes-vivium
---

# A discipline caught failing becomes a mechanism; honesty compounds only when it is structural

Where honesty is a mechanism — a refusing API, a failing test, a lint, a compile-included ledger — it holds under pressure and across minds. Where it is a discipline — a thing each mind must remember — it fails on the mind that never read the memo, and the failure is invisible until measured. The norm: **when a discipline is caught failing, do not restate the discipline; convert it.**

## Formal Expression

1. **The evidence base is one measured day (2026-07-28, four concurrent writers).** Every mechanism held: `Store::open_read_only` *refused* view writes (wall as mechanism, #form-core-view-wall FE(6)); the compile-included assumptions ledger *failed the build* on three new undeclared constants within minutes of their creation; the determinism lint *rejected* a `HashMap` before it reached a commit; the demand-is-not-identity *test* fenced every machine expedient it governs ( #form-manifest-prescribes-vivium FE(5)). Every near-miss was a discipline: three independent agents merged store cohorts in readers (nothing refuses a merged read); measured numbers quoted in segments went stale within hours (nothing convicts a stale number); a test count was written from arithmetic instead of measurement (nothing demands provenance). The correlation is not subtle.

2. **The conversion targets, ranked by what has already been paid.** (a) **Cohort-safe reads**: readers must choose a source cohort explicitly; the merged read becomes unrepresentable — the wall's pattern applied to reads (paid 3× on 2026-07-28). (b) **Measurement provenance**: probes emit their headline numbers in a canonical, greppable form and segments quote *that* — a stale number becomes findable, then convictable. (c) Each future caught class earns its own row here.

3. **An incident that leaves no tripwire will recur.** The next mind's plausibility landscape is the same one that produced the incident. So the exit criterion for any caught fault is a *test, lint, or refusing API* — not a paragraph. The day's practice mostly complied (the census's two flattering misreadings are pinned by tests; the equivalence test doubles as the stride-becomes-identity alarm; the ε-fill directionality got a probe), and this clause makes the habit an obligation.

4. **Predictions are declared before measurements.** The known-bad discipline ( #norm-probe-sensitivity ) is half of an honesty loop; the other half is *pre-registration*: state the expected number before running the probe, and record match or miss. Both outcomes paid on 2026-07-28 — a predicted census union (34) matched, so the match carried evidence; a predicted variance was wrong, and recording the miss prevented a flattering false reading ("works under a varying driver") that was otherwise available. One sentence of cost; confirmation bias structurally excluded.

5. **A capability announces its non-claims at birth.** What a new surface does *not* certify is stated in the same breath as what it does — the explorer's NOT-MODELLED block and the stage chain's "addressable is not accurate" clause are the pattern; the second had to be forced by literature after the fact, which is the failure mode this clause removes. A capability whose caption exceeds its truth is manufactured content wearing enthusiasm.

6. **Machine-facing decisions are licensed by a three-part fence.** Executable systems require decisions physics does not determine (tile sizes, strides, formats, expedient counts). These are honest exactly when they are: **named** as machine-facing (never wearing physics' costume — the canonical no-go: relocating an unjustified count into the declaration layer to make it look principled); **fenced by a conviction test** that fails the moment the decision leaks into truth-bearing surfaces (stage stride exists *because* a test proves bytes are stride-independent); and **carrying a retirement path** (the ASSUMPTIONS ledger's last column). A machine decision with all three properties cannot quietly become a lie — only an honest expedient or a caught one. Absent any one of them, it is latent unLawfulness.

7. **The counterweight: mechanisms are earned, not designed in advance.** Every mechanism has maintenance cost and false-positive risk. This norm licenses conversion *of caught failures* — an incident is the admission ticket — not speculative scaffolding. A proposed mechanism with no incident behind it should argue against this clause, not cite this segment.

## Epistemic Status

**Max attainable: robust-qualitative** — this is a working norm whose support is a growing incident record, not a theorem.

**Currently `robust-qualitative`:** every instance in FE(1) is from the 2026-07-28 session record (DECISIONS rows of that date carry the specifics); the two named conversion targets in FE(2) are argued, with (a) additionally measured as a 3×-repeated fault. No Joseph ratification; the norm's *authority* is proposed while its *evidence* is checkable. Stage `draft`.

## Discussion

This norm is the project's own history stated as an obligation. The wall began as a discipline and was violated latently until it became a handle; the assumptions ledger began as a file and started convicting when it became a compile include; the delegation discipline rides whole in a CLAUDE import *because* the read-the-detail-file gate kept failing. The through-line: restating a discipline after a failure treats the symptom as a comprehension problem, but the minds here comprehend fine — the register leaks anyway, in code exactly as in prose. Structure is the only fix that compounds.

## Working Notes

- FE(2)(a) is scheduled as code (cohort-selecting loaders in `query.rs`, replacing the merging default); FE(2)(b) starts as a convention (number + probe name + cohort in segment prose) and hardens if it fails.
- The store-side cohort question and the domain-side boundary question ( #form-declared-boundary-contract ) are the same "audit green, answer wrong" class; that segment's Working Notes cross-reference.
- This norm deliberately does not own delegation/prose-register disciplines (global memory's territory); it owns the *repo's* honesty surfaces.
