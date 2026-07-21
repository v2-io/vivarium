---
slug: ops-audit-integration
type: normative
status: robust-qualitative
stage: draft
depends:
  - scope-segment-canon
  - norm-decision-authority
  - norm-probes-before-claims
---

# Audit integration

An audit report is an instrument, not a todo list and not law. Findings are verified, adjudicated, and integrated — then the report may move to `audits/.integrated/`.

## Formal Expression

1. **Homes.** Delivered reports live at `audits/*.md` (pending). Working cognition trails, if any, live under `audits/AUDIT-WORKING-*/`. Claim canon remains `core/src/`. Process detail for the tree: `audits/README.md`.
2. **Three gates before `.integrated/`.** A report moves from `audits/` to `audits/.integrated/` only when **all** of the following hold for its finding set:
   - **Verified** — each load-bearing finding was checked against present code/segments/DECISIONS (not inherited from the report alone).
   - **Adjudicated** — each finding is dispositioned: *fix*, *strengthen then fix*, *no-go recorded*, *decline with reason*, *defer with named owner/condition*, or *not-a-defect*. Soften-first is forbidden for overclaims ( #norm-probes-before-claims; strengthen-before-soften).
   - **Integrated** — dispositions that require tree change are landed (segment, DECISIONS entry, code, demotion pointer) **or** explicitly declined with a durable record. Partial progress does not earn the move.
3. **Routing destinations** (pick the real home; do not invent a fourth canon):
   - **Claim truth** → segment in `core/src/` (new or corrected).
   - **Who decided** → `DECISIONS.decision-log.udon` with honest `:by` / `:status` ( #norm-decision-authority).
   - **Executable defect** → code + probe/test that can fail.
   - **Term** → `LEXICON.udon`.
   - **Process** → PROCESS / this family of ops segments / `audits/README.md`.
   - **Decline / defer** → note in the report's disposition trail or a DECISIONS entry; do not leave as silent ignore.
4. **Authority.** Auditor recommendations are evidence. They do not mint `:by joseph` or `:by us` decisions. Unratified verdicts stay `claude` / `proposed` until Joseph decides.
5. **Strengthen first.** When a finding says "downgrade the claim," attempt to make the claim true (or produce a specific no-go) before accepting a softer label.

## Epistemic Status

**Max attainable: exact** as process once practiced and refined; **currently `robust-qualitative`** — shaped by ASF audit-routing experience and Joseph's 2026-07-21 direction for vivarium (`.integrated/` only after verified/adjudicated/integrated), not a long vivarium scar trail yet. Stage `draft`. ASF's full protocol (`asf/doc/audit-routing-instructions.md`) is heavier and more mature; vivarium adopts the spine, not the whole ASF machinery.

## Discussion

The failure modes this process exists to block: (1) treating the audit as a chore list and "closing" items by softening claims; (2) moving a report to `.integrated/` because a couple of P0s landed while the rest still bind; (3) letting audit prose become a parallel law beside segments.

## Working Notes

- Live pending reports and a short status table: `audits/README.md`.
- De-novo `AUDIT-WORKING-*` cognition trails: consult Joseph before mining or deleting (ASF lesson; apply if such dirs appear here).
