# audits/ — audit outputs

Where independent audit reports land. Lighter than ASF's `audits/` tree; same
spirit: findings are instruments that feed segments, DECISIONS, and code — not
a parallel canon (`#scope-segment-canon`, `#ops-audit-integration`).

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

## Live status (pending reports)

| Report | Focus | Status |
|---|---|---|
| [`2026-07-21-core-segment-floor-audit.md`](2026-07-21-core-segment-floor-audit.md) | Segment floor + front door | **Pending (near-closed).** P0s closed. Dual-home demotion largely landed for §I claims. Remaining open as **defer**: full DESIGN-REDUX/theory dual-home sweep; promotion past `draft`. See [`DISPOSITION-2026-07-21-segment-floor.md`](DISPOSITION-2026-07-21-segment-floor.md). |
| [`2026-07-21-de-novo-project-audit.md`](2026-07-21-de-novo-project-audit.md) | Code/behaviour (A) | **In progress.** **P0 builder/flux gate: integrated** — default `build` refuses erosion/water when flux needs unmet; `--allow-unmet` waiver; lock RAII cleanup on early return. Remaining: lock TOCTOU atomicity, query put-error propagation, store hash upgrade, float schema checks. See disposition trail. |
| [`2026-07-21-de-novo-project-audit-b.md`](2026-07-21-de-novo-project-audit-b.md) | Canon boundaries (B, Opus, blind) | **In progress.** Integrated: abyssal-parity moratorium pointer → ETHICS/`#scope-moratorium…`; `#scope-segment-canon` carve-outs extended; worldview clock face named on `#post-determinism…`; dropped inverted probes→declaration depend. Remaining: nomos version source-hash (design), assumptions reverse-guard, program INCOHERENCE row 1 close, more depends hygiene. |

When a report moves to `.integrated/`, replace its row with a one-line pointer.
