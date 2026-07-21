# audits/ — audit outputs

Where independent audit reports land. Pattern follows ASF's `audits/` tree
(lighter here while the corpus is small).

| Location | Meaning |
|---|---|
| `audits/*.md` | Delivered reports — **pending** verification, adjudication, and integration |
| `audits/.integrated/` | Reports whose findings have been **verified, adjudicated, and integrated** (fixes landed or consciously declined with a record). Move here only then. |

**Not integrated just because the file exists.** Partial work (e.g. closing two
P0s from a longer report) does not earn `.integrated/` — the report stays
pending until the finding set is dispositioned.

Working cognition traces, if any, belong in dated `AUDIT-WORKING-*/` subdirs
here (not under `msc/`), same discipline as ASF: do not casually summarize or
delete first-encounter trails.

Claim canon remains `core/src/`. Audits are instruments that feed segments,
DECISIONS, and code — not a parallel law.
