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

3. **Every shot has a referent** — `#norm-no-depiction-without-referent` extended to documents: renders are *captured* from standard self-labelling commands, never composed or retouched. If the honest capture is unimpressive, the unimpressive capture is the entry. The entry still names seed, `src=`, and command.

4. **Pre-register the expected entry** ( `#norm-caught-disciplines-become-mechanisms` FE(4) at outcome altitude): before building, one sentence of "when this lands, an entry can show X" — recorded in a frozen entry or DECISIONS, adjudicated at landing. A landed build whose pre-registered entry cannot be written has measurably not reached the end of the row.

5. **World-store snapshot is part of the landing, not optional taste.** When FE(1) is owed and the path to the visible change **rebuilds** the default (or named) vivium store, commit that store's local git **as part of the same procedure** — typically: commit (or tag) the pre-rebuild tree for a reproducible *before*, rebuild, commit the post-rebuild tree for *after*, then capture. first-light's home is `~/.cache/vivarium/globe-world` ( #form-store-as-save FE(2) ). This is not a second save format and not vivarium source git; it freezes whole-directory model state so before/after does not depend on memory or on an accidental multi-`src` pile. Checkout then capture is as valid as dual-cohort capture under one live store. **Skip only when FE(1) skips** — not when the rebuild "doesn't feel important enough."

6. **The frozen entry carries provenance in the body, not only in chat.** Required fields when FE(1) lands (a landing without them is incomplete — same as an unwritable capture):

   | Field | Content |
   | --- | --- |
   | Seed + world name/path | e.g. first-light, seed, `~/.cache/vivarium/globe-world` |
   | `src=` (or dual before/after) | kernel era of the captured surface(s) |
   | World-dir git rev(s) | pre- and/or post-rebuild commits when FE(5) applied; if missing, **say so** (specimen: 2026-07-29-01) |
   | Commands | exact explorer/build/probe lines that produced the shots |
   | Captures | **embedded** as Obsidian wikilinks `![[captures/…/name.png]]` under `changelog/`, plus relative markdown links for GitHub |

7. **Landing checklist (agent must not soft-skip).** When FE(1) is owed, finish **all** of the following in the same cycle or leave an explicit incomplete queue — do not claim the work "landed" on segments/code alone:

   1. Pre-register the expected entry sentence (FE(4)).
   2. If rebuilding: **commit world store** (FE(5)) pre-rebuild; rebuild; **commit** post-rebuild.
   3. Capture before/after (or one-shot + named residual) into `changelog/captures/…`.
   4. Write `changelog/DATE-NN-slug.md` with FE(6) fields + embeds; freeze.
   5. Commit captures + entry on the **vivarium source** repo.
   6. Clear “entry owed” language from hotspots / working notes.

8. **The two-diagnosis clause.** An unwritable entry queues the *work* or the *instrument*; "built, currently invisible, because no view reaches X" is a valid entry and a direct instrument requisition (first exercise: the CLI region-render gap, entry 01).

9. **Out of bounds.** Entry prose as a claim source (segments own claims); entries as marketing (the eye they serve is a calibrated instrument, `#norm-no-depiction-without-referent` FE on decalibration).

## Epistemic Status

**Max attainable: robust-qualitative** — a working norm; its evidence is whether entries keep getting written and whether pre-registrations keep getting adjudicated.

**Currently `robust-qualitative`:** charter 2026-07-28; FE(5)–(7) tightened 2026-07-29 after an agent soft-skipped world-git + entry until asked (specimen entry `2026-07-29-01`). Ratified in intent by Joseph; mechanics `:by claude, proposed`. Stage `draft`.

## Discussion

The practice closes a measured failure: a day of defensible layer-beneath work once summed to a day where nothing on screen changed, and it took four askings to land the lesson ("deliver the visible thing"). The entry makes that check structural — cheap enough to run every landing, honest enough to fail.

## Working Notes

- Operational detail (capture commands, filename convention, entry form): [`changelog/README.md`](../../changelog/README.md) — the README is the manual; this segment is the law.
- The standing instrument requisition from entry 01: a CLI-level region render (named window, named stage, relief/discharge ASCII) to make before/after pairs one command each.
- **first-light world VCS (2026-07-29):** default store `~/.cache/vivarium/globe-world` is under local git (baseline `28cb20d` post-Jacobi). FE(5)–(7): commit + entry fields + checklist. Ops detail in that dir's `README.md`.
