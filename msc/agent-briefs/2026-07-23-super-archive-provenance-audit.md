# Super-archive provenance audit

*Peer auditor report for parent session. Not claim canon. 2026-07-23.*

**Ground truth for this job:** git history + file content (not live code). Scope: integrity of what *sits in* `.super-archive/` relative to what graduation claimed — not re-litigation of whether items should have graduated.

**Method used**

- Full inventory of `.super-archive/{from-archive,from-design,from-msc}/` against `MANIFEST.md`.
- Structure/completeness read of every graduated body (headers, section spine, ends of files — not a skim of titles only).
- DESIGN dual-copy cross-read; sketch Working Notes + session memory of fix `42080b9`.
- Live tree checks: `doc/design/` (DESIGN gone), `.archive/` residual, ordinum `:reportatio`, LEXICON/doc stale paths.
- **Limit:** this environment had no interactive shell, so I could **not** re-run `git show` / `diff` byte-equality against parents. DESIGN “byte-match to `0a5050e^`” is **structure + prior-session verification**, not a fresh hash. Parent should re-run the one-liner below if absolute proof is required.

```
diff -u <(git show 0a5050e^:doc/design/DESIGN.md) .super-archive/from-design/DESIGN.md
# expect empty

# optional pin check (content at pin vs graduated PHASES body)
git show 42621d5:.archive/PHASES.md 2>/dev/null || git show 42621d5:doc/PHASES.md
# compare to .super-archive/from-archive/PHASES.md (headers may differ; body is what matters)
```

---

## Verdict

**Mostly sound — issues found, no second DESIGN-class body loss found.**

The known DESIGN intermediate-peel failure appears **already fixed** in tree: both `from-design/DESIGN.md` and the archaeologist alias hold the long two-layer FE (~20 lines of load-bearing text, not a three-line pointer), 150 lines end-to-end, live pointer-shell DESIGN removed from `doc/design/`.

I did **not** find another graduated file that is an intermediate dual-home peel waved through as “full ice.” Batch 1–3 archive/msc bodies look like complete last-honest specimens (long reportatio, full spikes, full inventory, full session reflection).

What *is* wrong or incomplete is mostly **pointer hygiene and pin/path bookkeeping** after moves — real, but different failure mode from content truncation.

---

## Findings table

| ID | Severity | Path / area | Finding |
|----|----------|-------------|---------|
| F1 | **Fixed / re-checked OK** | `from-design/DESIGN.md` + `DESIGN-pre-pointer-shell-2026-07-23.md` | Known failure: first graduate saved post-peel intermediate (two-layer crushed). Current bodies: full two-layer (fast authoritative / slow-at-aporia / moral scope / latency-determinism-cost / open identifiability), Bevy, axes. Dual copies appear content-identical through EOF (line 150). MANIFEST correctly documents restore from `0a5050e^`. Sketch `#sketch-two-layer-mind` carries FE + notes the fix. Live `doc/design/DESIGN.md` **absent** (shell not left as dual-home). **Fresh byte-diff vs git not re-run here.** |
| F2 | **OK (full body)** | `from-archive/PHASES.md` | Long reportatio, not a stub: archive banners, phases 0–8, Design notes, Algorithms ledger through agent table (~377 lines). Matches “reportatio ice” claim. Ordinum `:reportatio` path correctly points here. |
| F3 | **Medium — pin integrity unverified** | PHASES + `tabularium/terrestris.ordinum.udon` `:reportatio-pin 42621d5` | Path updated; pin not re-diffed. File carries **post-archive editorial** (2026-07-11 archive banner + 2026-07-16 1-index/Scribal divergence). Pin may be body-at-compile; headers may mean file bytes ≠ pin blob. Not content *loss*, but pin can no longer mean “byte-identical to graduated path” without a note. |
| F4 | **OK (full body)** | `from-archive/CLAUDE.md` | Full front-door specimen: Level A/B/C, Prime Question, bias/noise, authority, open identifiability, memory bridge. Archive banner + a few claim-home glosses at top — **annotation, not peel-to-stub**. |
| F5 | **OK (full bodies)** | `VIVARIA-DEFINITIONS.md`, `VIVARIA-DECLARATIVE-FRONTIER.md` | Full spikes: layers 0–4 + web + step-back; frontier correction header + residue + failure record. Not intermediate shells. |
| F6 | **OK (full body)** | `memory-surfaced-2026-07-13.md` | Full inventory (~920+ lines): index table + verbatim memory dumps through archema-io `queued-work`. Not a summary stub. |
| F7 | **OK (full package)** | `from-msc/consolidation-wave-2026-07-21/*` | README + 4 mining reports + 3 draft-form files with SUPERSEDED/PROMOTED banners. Complete wave ice. |
| F8 | **OK (full body)** | `from-msc/session-2026-07-10-mechanics.md` | All 7 mechanics sections present (probe sensitivity … parallel deep-readers). Live `msc/reflections/` no longer holds a copy (only `compaction-fidelity-modes.md`). Core norms cite the super-archive path correctly. |
| F9 | **OK (full bodies)** | Batch 1 ice: ORIENTATION, TODO, README.root, HANDOFF, `core/OUTLINE`, taxonomy-bdd, architecture-migration, lexicon-scratch, universal-biological-rhythms | Complete historical specimens (ORIENTATION still has grid HANDOFF block; OUTLINE seed ends “No segment is written yet”; rhythms keeps correction header + followup). No peel-to-router shape. |
| F10 | **Low — MANIFEST residual incomplete** | `MANIFEST.md` “Still residual mine” | Says `.archive/` = `SUPERSEDED.md` only. Disk also has `.archive/README.md` (index). Not a content-integrity bug. |
| F11 | **Medium — live pointers still name dead `.archive/` paths** | LEXICON, PROCESS, several `doc/` files | Graduated paths still cited as `.archive/PHASES.md`, `.archive/VIVARIA-…`, `.archive/universal-biological-rhythms.md`, etc. **Ordinum + several core segments were updated; LEXICON/doc were not.** Agents following live citations get 404-at-path, not “already iced.” Same class as incomplete graduation bookkeeping, not truncated ice. Notable carriers: `LEXICON.udon` (`phase-transition`, `defeasance`, `ordinum`); `doc/PROCESS.udon` (flagged-file-routing still lists `.archive/universal-biological-rhythms.md`; agentic-verdict source line); `doc/theory/multiscale-methods.md`; `doc/plan/regula-conformance-design.md`; `doc/design/DESIGN-SYSTEMS.md`, `NOMOS-CONTRACT.md`, `DESIGN-REDUX.md`; `core-segment-candidates-2026-07-14.md` (historical inventory — lower urgency). |
| F12 | **Low — intentional intermediate, not loss** | DESIGN body content | Restored DESIGN still has **Claim home** demotion banners (purpose, core-view, determinism, fidelity) with residual substrate under some sections. That is the honest pre-pointer-shell intermediate chosen as graduation artifact — **not** the catastrophic two-layer crush. Axes / two-layer / Bevy remain full FE. Do not “restore older fuller DESIGN” unless someone proves pre-banner body was load-bearing and never pealed into segments/sketches. |
| F13 | **Info — method debt** | This audit | No fresh `git show`/`diff` against parents for non-DESIGN files. Completeness judged by structure + length + expected section spines. Residual risk: a file could be “full looking” but still missing a mid-file deletion only git would catch. Unlikely for the spikes/reportatio/session given shape, but not zero. |

---

## What is sound (short)

| Batch | Paths | Judgment |
|-------|--------|----------|
| 1 re-founding ice | ORIENTATION, TODO, README.root, HANDOFF, core/OUTLINE, taxonomy-bdd, architecture-migration, lexicon-scratch, universal-biological-rhythms | Full specimens; ice claims match content class |
| 2 VIVARIA | DEFINITIONS, DECLARATIVE-FRONTIER | Full spikes with honest NOT-BUILT / correction discipline preserved |
| 3 low-hanging | PHASES, CLAUDE, session-2026-07-10, consolidation-wave package | Full bodies; no intermediate dual-home costume |
| 4 DESIGN + memory | DESIGN (+ alias), memory-surfaced | DESIGN failure already corrected; memory full inventory ice |

**MANIFEST path inventory:** every path listed exists on disk; no orphan super-archive files outside MANIFEST (alias is listed). Layout matches README (`from-archive` / `from-design` / `from-msc`).

---

## Recommended restores / corrections

### Do now (parent session, cheap)

1. **Re-run DESIGN byte-diff** (one-liner above). If empty → close F1 as verified this session; if not empty → restore again from `0a5050e^` and stop trusting “structure looks right.”
2. **Sweep live tactical pointers** from `.archive/<graduated>` → `.super-archive/from-archive/<same>` (or to the owning segment / SUPERSEDED ledger where the citation was only historical color):
   - **Must:** `LEXICON.udon` (3× PHASES reportatio citations), `doc/PROCESS.udon` flagged-file-routing + agentic source line, `tabularium` already OK.
   - **Should:** `doc/plan/regula-conformance-design.md`, `doc/theory/multiscale-methods.md`, `doc/design/DESIGN-SYSTEMS.md` (ledger origin trail), `NOMOS-CONTRACT.md` archaeology line, `DESIGN-REDUX.md` migration trail.
   - **May leave:** dated candidate inventory / audits that are themselves ice-adjacent.
3. **MANIFEST residual row:** `.archive/` = `SUPERSEDED.md` + `README.md`.
4. **Reportatio-pin note (if pin ≠ current file bytes):** either (a) document in PHASES header / ordinum comment that pin is pre-banner body hash / commit, or (b) pin a commit that includes the frozen graduated body and state that explicitly. Do **not** silently re-pin without reading whether 42621d5 was body-only.

### Do not restore (no evidence of loss)

- PHASES “from some longer pre-archive draft” — graduated file is the long reportatio with algorithms ledger; not a three-line shell.
- VIVARIA spikes “pre-correction” — correction header is load-bearing honesty; stripping it would be worse provenance.
- CLAUDE “pre-archive banner” — banner is correct demotion; body intact.
- Pre-claim-home DESIGN (unless git shows material never pealed into segments/sketches) — would fight integration-is-replacement.

### Optional process hardening (so DESIGN cannot recur)

Add a graduation checklist line to PROCESS / super-archive README (parent drafts; peer voice):

> Before `git mv` to super-archive: **diff against the last pre-peel commit** (or `0a5050e^`-style known-good parent). “Looks complete” and “claim homes exist” are not provenance. Prefer byte-match or named intentional intermediate with section checklist (e.g. two-layer still has FE 1–5).

---

## Feedback on the brief

**Helped**

- Named failure mode (intermediate peel waved through) + known-good DESIGN fix anchors.
- Explicit “integration is replacement / not pointer tables” so audit didn’t re-argue graduation.
- Concrete hot list (PHASES, CLAUDE, VIVARIA, memory, consolidation-wave, session, DESIGN) focused effort.
- Severity + restores as deliverable shape.

**Would help next time**

- Shell / `git show` availability for the auditor (byte-diff is the actual instrument for this class of bug).
- Optional: `git log --follow --stat` one-liners already computed for each graduated path at brief time — saves rediscovery of batch commits (`f2e37cb` intro, `c9110d1` batch 2 peels, `0a5050e` batch 3, `c5fe226` batch 4, `42080b9` DESIGN fix).
- Clarify whether stale **live** `.archive/…` citations are in-scope (I treated them as related graduation bookkeeping, medium severity) or out-of-scope pure-ice audit.

---

## Stay available

I can: re-run git diffs if shell appears; draft the LEXICON/PROCESS path sweep patch; or pin-integrity report for PHASES once `git show 42621d5:…` is available. Not claiming this audit was comprehensive against every blob parent — only that **no second DESIGN-shaped truncation was found on content inspection**, and that **path/pin hygiene is the live residual defect class**.
