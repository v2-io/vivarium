# audits/ — audit outputs

Where independent audit reports land. Lighter than ASF's `audits/` tree; same
spirit: findings are instruments that feed segments, DECISIONS, and code — not
a parallel canon (`#scope-segment-canon`, `#ops-audit-integration`).

## Layout

| Location | Meaning |
|---|---|
| `audits/*.md` | Delivered reports — **pending** verification, adjudication, and integration |
| `audits/.integrated/` | Reports whose finding set has been **verified, adjudicated, and integrated** (fixes landed or consciously declined with a durable record). **Move only then.** |
| `audits/AUDIT-WORKING-*/` | Optional first-encounter cognition trails. Not scratch. Consult Joseph before bulk mining, summarization, or deletion. |
| `audits/README.md` | This file — process + live status of pending reports |

**Partial landing does not earn `.integrated/`.** Closing two P0s while P1s remain means the report **stays pending**.

## Integration process (short)

Full claim form: `#ops-audit-integration`. Spine:

1. **Verify** — re-check each load-bearing finding against present tree (code, segments, DECISIONS). Auditors err; code moves.
2. **Adjudicate** — disposition each finding: fix · strengthen-then-fix · no-go · decline-with-reason · defer-with-condition · not-a-defect.  
   **Strengthen before soften** when the finding is an overclaim.
3. **Integrate** — land the disposition (segment / DECISIONS / code+probe / demotion pointer) or record the decline. Authority tags stay honest (`#norm-decision-authority`).
4. **Retire** — when the finding set is dispositioned, `git mv` the report to `audits/.integrated/` and note the date + integrating commit in this README's status table (or a one-line header in the report).

ASF's longer protocol (`asf/doc/audit-routing-instructions.md`) is a reference for hard cases (no-go protocol, spike completion states). Vivarium does not require ASF's full STATUS/WORKING machinery unless the backlog grows enough to need it.

## Live status (pending reports)

| Report | Focus | Status |
|---|---|---|
| [`2026-07-21-core-segment-floor-audit.md`](2026-07-21-core-segment-floor-audit.md) | 13 draft segments + front door | **Pending.** P0-1 (depends/outline) and P0-2 (view evolution-knob incompleteness named) **closed** in `3f8754d` / follow-ups. Dual-home demotion **partially** landed (`5c5c1f2`). Remaining: P1 dual-home completeness, ORIENTATION residue elsewhere, promotion ladder, next extraction wave. |
| [`2026-07-21-de-novo-project-audit.md`](2026-07-21-de-novo-project-audit.md) | Workspace code behaviour | **Pending.** Not dispositioned. Headline P0: builder/`status` flux gate vs actual erosion materialization; also builder lock races, swallowed store errors, 64-bit hash integrity. Do not treat as integrated. |

When a report moves to `.integrated/`, delete or rewrite its row here to a one-line pointer into `.integrated/`.
