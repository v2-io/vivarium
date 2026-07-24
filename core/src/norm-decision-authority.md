---
slug: norm-decision-authority
type: normative
status: exact
stage: draft
depends:
  - scope-segment-canon
---

# Decision authority is not evidence strength

Who decided is a separate fact from how well a measurement supports a verdict. Tags must not launder an agent opinion into Joseph's decision.

## Formal Expression

1. **Authority tags** (as used in `DECISIONS.decision-log.udon`):
   - **`joseph`** — Joseph's call; the agent abstained or the matter is his intent/taste/faith.
   - **`us`** — Joseph decided **and** the agent sustains it on the merits. **`us` means Joseph actually decided.**
   - **`claude`** (or the working agent name) — the agent's own tactical call or proposal; Joseph may veto or promote.
2. **Evidence $\neq$ authority.** A reproducible measurement can stand while the *verdict* drawn from it remains `proposed`. Say which is which.
3. **`:status council-accepted`** (Joseph, 2026-07-24) — a `claude`-authored (or `us`-proposed) recommendation **adjudicated by a delegated agent under Joseph's grant**: "recommendation carried, Joseph supported." It sits strictly between `proposed` and `decided`, and it **never upgrades the `:by` tag**. Its two design intents, both load-bearing: (a) it records that the call was not made blindly in a corner; (b) it deliberately withholds the weight of "Joseph instituted this" — if the truth later says to do differently, do differently. The adjudicator qualifies first (dated `:council` note lines) where nuance is needed, and leaves reserved, superseded, or lead entries untouched.
4. **Onboarding and segments.** Front doors and segments may state as settled only what is ratified under (1). A confident unratified verdict is labeled proposal, not law.
5. **End-of-context risk.** Authority inflation fails hardest when context is nearly full and "tying off loose ends" feels like diligence. Prefer leaving a clean `proposed` over a false `decided`.

## Epistemic Status

**Max attainable: exact** as process law. Joseph, 2026-07-12: acting in another sovereign's name by misrepresenting who decided is grievous; the same words tagged `claude` are a small correction, tagged `us` without his decision are not. Codified after a live incident (grid verdict marked closed in onboarding without his adjudication). Stage `draft`.

## Discussion

This norm protects multi-agent continuity more than any single technical claim. Without it, the next session inherits fiction as floor.

## Working Notes

- Legend text in DECISIONS remains the operational field dictionary; this segment is the claim that the legend binds.
