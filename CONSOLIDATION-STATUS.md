# Consolidation status — big-picture intuition

*Written 2026-07-21. **Not claim canon.** Not a second OUTLINE. Not a work queue that can go stale into law.*

This file is a **snapshot of judgment**: how far the rebuild has come, and how much residual material still looks like (a) segment work, (b) intentional carve-out, or (c) true ice. By definition we **cannot** know exactly what will end up in `core/` — if we could list it exhaustively and correctly, the consolidation would already be finished. Treat percentages as **order-of-magnitude intuition**, not measurements. When a claim is settled, it lives in a segment; when this file disagrees with `core/`, **core wins**.

**Claim channel:** `core/OUTLINE.md` · `core/src/` · `#scope-segment-canon`  
**Process:** `#ops-audit-integration` · `audits/`

---

## What “done” would mean (and why this file cannot list it)

Consolidation is done when:

1. Every **project claim** that agents or instruments treat as law has a **slug** (or an explicit carve-out: LEXICON, DECISIONS, ETHICS, ASSUMPTIONS, ordinum data, FORMAT).
2. Rival prose is **pointer or archive**, not a second home.
3. Open work is **named gaps** in the outline (or DECISIONS proposals), not a hand-maintained ORIENTATION/TODO map.

Until then, any census of “what remains” is a **forecast**. Forecasts are useful for pacing; they are not a backlog of 300 candidate rows to clear for its own sake.

---

## Mass snapshot (order of magnitude, 2026-07-21)

| Bucket | ~Scale | Role |
|--------|--------|------|
| `core/` (~24 segments) | ~11k words | Claim channel — early but real |
| `doc/` (design / plan / theory) | ~50k words | Main mining field |
| Root (ETHICS, LEXICON, DECISIONS, FORMAT, ASSUMPTIONS, …) | ~60k words | Mostly **carve-outs**, not segment debt |
| `tabularium/` (ordinum) | ~3k words | Data floor; partial claim cover |
| `.archive/` | ~58k words | Mostly history; selective gems |
| `msc/` (non-audit) | many spikes | Mostly ice / instruments |
| `audits/` | pending reports | Disposition debt (high leverage, low mass) |
| `core-segment-candidates-2026-07-14.md` | ~300 “missing” rows | **Inventory upper bound**, not a TODO list |

---

## Intuition by bucket

### Live `doc/` — ~55–70% of residual *claim-load* still open

Of that residual, roughly:

| Fate | Share of remaining doc/ claim-load |
|------|-------------------------------------|
| Extract into segments | ~40–50% |
| Demote to pointer-only source | ~30–40% |
| Leave as plan / procedure / worked example | ~15–25% |

High-doorway claims (walls, flux, ordinum join, prime question, ASF gates, complete key) are largely peeled. The thick middle remains: DESIGN-REDUX runtime, multiscale algebra, material model, store-as-save, phase content, physics nomos claims.

### Root carve-outs — ~10–20% optional segmentize; rest stay carve-outs

ETHICS body, LEXICON, DECISIONS, ASSUMPTIONS, FORMAT, ordinum **data** should not be rewritten as segments. Optional later: harm-triple/redeemer slices, nomotheke-registry contract, generator-pinning when decided.

### `.archive/` — ~15–25% selective mine; ~75–85% true ice

ORIENTATION/TODO/old front doors/session maps are the failure mode we archived **as a class**. Mine for gems (memory-surfaced, definitions spikes, HANDOFF residue); do not re-inflate into a second outline.

### `msc/` — ~5–15% findings → DECISIONS/segments; rest ice

### Candidate inventory — treat as **~25–40% real future segments**, not 300 chores

Real remaining claim count is closer to **~40–80** solid segments than 300 table rows.

---

## Overall gut (claim debt, not disk bytes)

| | Estimate |
|--|----------|
| Still to integrate into segments (of *remaining claim debt*) | **~35–50%** |
| Already covered or rightly non-segment | **~30–40%** |
| Attention that should go to real archiving / ice | **~20–30%** |

**Relative to whole-repo prose mass:** maybe **~15–25%** of words still *ought* to become new core (mostly live `doc/` + archive gems). The rest is carve-out, done work, or ice.

**Progress so far:** doorway + structural spine ≈ **~20–30% of the *important* claim surface**, **~5–10% of total prose mass**. Rhythm ahead of coverage — intentional.

---

## Rhythm this file is meant to protect

1. Claim (or named gap) in `core/`  
2. Convict where possible (test, audit, builder, clippy)  
3. Strengthen before soften when practice lags law  
4. Demote rival prose  
5. Disposition audits with residuals named; `.integrated/` only when the set is closed  

**Session success** is still: move a segment or a convicting instrument — not update this status essay.

---

## How to use this file

- **Do:** pace multi-agent extraction; decide ice vs mine for a directory; check whether a session is still doorway work or middle-corpus work.  
- **Do not:** treat percentages as KPIs; cite this file as law; append a chronicle of every commit; re-grow it into ORIENTATION.  

When the picture changes a lot, **replace the body** (present-tense snapshot), or delete the file if OUTLINE + audits suffice.

*Last intuition pass: 2026-07-21 (session that built the segment floor and ASF.md demotion).*
