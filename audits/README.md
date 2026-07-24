# audits/ — audit outputs

Where independent audit reports land. Lighter than ASF's `audits/` tree; same spirit: findings are instruments that feed segments, DECISIONS, and code — not a parallel canon (`#scope-segment-canon`, `#ops-audit-integration`).

## Layout

| Location | Meaning |
|---|---|
| `audits/*.md` | Delivered reports — **pending** verification, adjudication, and integration |
| `audits/.integrated/` | Reports whose finding set has been **verified, adjudicated, and integrated** (fixes landed or consciously declined with a durable record). **Move only then.** |
| `audits/DISPOSITION-*.md` | Running disposition trails for pending reports (optional; may merge into the report header when retiring) |
| `audits/AUDIT-WORKING-*/` | Optional first-encounter cognition trails. Consult Joseph before bulk mining or deletion. |
| `audits/README.md` | This file — process + live status |

**Partial landing does not earn `.integrated/`.**

## Integration process (short)

Full claim form: `#ops-audit-integration`. Spine:

1. **Verify** — re-check findings against present tree.
2. **Adjudicate** — fix · strengthen-then-fix · no-go · decline · defer · not-a-defect. Strengthen before soften.
3. **Integrate** — land disposition or record decline.
4. **Retire** — `git mv` to `audits/.integrated/` when the finding set is fully dispositioned.

## Live status

**No pending reports** (2026-07-24). All three 2026-07-21 reports are retired to [`.integrated/`](.integrated/) with their disposition trails:

| Report | One-line disposition |
|---|---|
| `2026-07-21-core-segment-floor-audit.md` | P0s + follow-ups fixed; dual-home sweep completed by the design/theory graduation; promotion tracked by the stage ladder itself. |
| `2026-07-21-de-novo-project-audit.md` | Refuse gate + provisional honesty landed (incl. `Source::HitProvisional` + lib e2e, 2026-07-24); remaining defers durably homed on `#form-builder-admission` / `#form-complete-content-addressed-key`. |
| `2026-07-21-de-novo-project-audit-b.md` | Canon-boundary fixes landed; depends hygiene executed 2026-07-24 (four cite-or-drop); source-hash / reverse-guard / INCOHERENCE row 1 durably homed on their segments and DECISIONS. |
