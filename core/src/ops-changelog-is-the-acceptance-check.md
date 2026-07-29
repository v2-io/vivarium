---
slug: ops-changelog-is-the-acceptance-check
type: normative
status: robust-qualitative
stage: draft
depends:
  - norm-no-depiction-without-referent
  - norm-caught-disciplines-become-mechanisms
  - scope-segment-canon
---

# The changelog entry is the outcome-altitude acceptance check

User-facing change lands with a **frozen, dated entry** in [`changelog/`](../../changelog/) that shows the world doing the new thing — captured from the store, self-labelled with seed, cohort and command. If no entry is writable, that distance is measured and queued: either the work has not reached the surface, or **no instrument can render that scale or layer yet** — two different findings with two different queues.

## Formal Expression

1. **The bar.** An entry is owed when the *external perspective* moved — something visible or doable in `explore` / `watch` / `info`, or world behavior a user would notice. Internal commits skip freely; skipping is fine and good, not a gap (Joseph, 2026-07-28: all the physics and code in the world isn't useful if the rendered world still shows fBm circles passed off as rising land).

2. **Entries are history, not canon.** Frozen at landing, append-only, errata by postscript — a dated snapshot cannot go stale, which is what distinguishes this from the hand-maintained front doors the 2026-07-13 re-founding archived. Current truth stays in `core/`; `#scope-segment-canon` is untouched.

3. **Every shot has a referent** — `#norm-no-depiction-without-referent` extended to documents: renders are *captured* from standard self-labelling commands, never composed or retouched. If the honest capture is unimpressive, the unimpressive capture is the entry. **World-store git** ( #form-store-as-save FE(2) local revision control of a vivium directory) is a valid way to *restore* a prior model for capture — checkout then `explore` / `info` — alongside in-store multi-cohort reads. The entry still names seed, `src=`, and command; the git rev of the world dir is optional extra provenance, not a substitute for the capture.

4. **Pre-register the expected entry** ( `#norm-caught-disciplines-become-mechanisms` FE(4) at outcome altitude): before building, one sentence of "when this lands, an entry can show X" — recorded in a frozen entry or DECISIONS, adjudicated at landing. A landed build whose pre-registered entry cannot be written has measurably not reached the end of the row.

5. **The two-diagnosis clause.** An unwritable entry queues the *work* or the *instrument*; "built, currently invisible, because no view reaches X" is a valid entry and a direct instrument requisition (first exercise: the CLI region-render gap, entry 01).

6. **Out of bounds.** Entry prose as a claim source (segments own claims); entries as marketing (the eye they serve is a calibrated instrument, `#norm-no-depiction-without-referent` FE on decalibration).

## Epistemic Status

**Max attainable: robust-qualitative** — a working norm; its evidence is whether entries keep getting written and whether pre-registrations keep getting adjudicated.

**Currently `robust-qualitative`, one day old:** charter and first entry landed 2026-07-28 with three pre-registrations pending adjudication. Ratified in intent by Joseph (the framing is his); the mechanics are `:by claude, proposed`. Stage `draft`.

## Discussion

The practice closes a measured failure: a day of defensible layer-beneath work once summed to a day where nothing on screen changed, and it took four askings to land the lesson ("deliver the visible thing"). The entry makes that check structural — cheap enough to run every landing, honest enough to fail.

## Working Notes

- Operational detail (capture commands, filename convention, entry form): [`changelog/README.md`](../../changelog/README.md) — the README is the manual; this segment is the law.
- The standing instrument requisition from entry 01: a CLI-level region render (named window, named stage, relief/discharge ASCII) to make before/after pairs one command each.
- **first-light world VCS (2026-07-29):** default store `~/.cache/vivarium/globe-world` is under local git (baseline post-Jacobi). Prefer committing that tree after rebuilds that will need a before/after later; pairs can then be `git checkout` + capture as well as dual-cohort capture under one live store.
