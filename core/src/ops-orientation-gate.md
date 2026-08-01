---
slug: ops-orientation-gate
type: normative
status: robust-qualitative
stage: draft
depends:
  - scope-segment-canon
  - norm-caught-disciplines-become-mechanisms
  - norm-decision-authority
---

# Orientation gate — closed-book prove before commit; residual improvements land here

Compaction and ordinary overconfidence leave a *feeling* of orientation that is not the substance. Before an agent may commit on this tree, it must free-read the claim surface and pass a closed-book prove. The operator surface is `ORIENT.md` + `bin/try-me` / `bin/prove-me`; **this segment owns the process claim**, the post-pass improvement posture, and the durable backlog for tool and unit-quality work that sessions notice but do not ship in the same breath.

## Formal Expression

1. **The gate.** A commit that changes project truth or tools must carry an `orient:<session_id>` token from a live prove for *this* session’s compaction generation (see `bin/orient-check`). Feeling oriented is not a substitute. Free-read is open; after `try-me`, `core/src` is sealed — dumping sealed files is cheating and ends the session.

2. **Operator surface (not a second law home).** How to free-read, run the quiz, fill answers, and read pass grades lives in [`ORIENT.md`](../../ORIENT.md). That file is the agent-facing manual; it must stay short and non-spoiling. Ranking internals (tip fiat, score α, factor tables) are maintainer detail, not free-read curriculum.

3. **Vanilla udon for quiz and answers.** Questions (`.orient/quiz.udon`, `|item[Pn]`) and answers (`.orient/answers.stub.udon`, `|answer[Pn]`) share the same small notation family as `LEXICON.udon` / `DECISIONS.decision-log.udon` — column-0 elements, identity keys, indented bodies, `;` comments. Completing the gate is also a first real-world udon exercise. Recognition in `prove-me` is line/indent-level, not a full udon engine.

4. **Pass grades (default five items).** **5/5** — unlimited commits while this session’s compaction gen matches. **3/5 or 4/5** — one-time commit, with a promise to free-read the missed sections listed by prove-me. **&lt;3/5** — fail; re-orient and `try-me` again. One-time uses are spent on the first successful commit that carries the token. Compaction invalidates the pass for that session.

5. **Soft reject is not a grade.** While the deadline remains, a submit that is empty, format-confused, a pasted quiz instead of answers, or has **fewer than three substantial** `|answer` bodies (length &gt; 5 characters, non-placeholder) is **not graded**: no burn, no unseal. Fix and re-submit. A real graded attempt under the bar still burns.

6. **One live quiz per checkout.** Shared seal of `core/src` + single `.orient/test.jsonl`. Parallel agents on one tree clobber each other. Multiple session *passes* may coexist under `.orient/passes/`.

7. **Post-pass first work (when credentials land).** After a pass, among the legitimate first commits — not the only work, but explicitly encouraged before wandering into greenfield polish — are:

   - **Segment repairs** made obvious by free-read or by the quiz: formatting, staleness, dual-home residue, missing depends, wording that the unit anchors revealed as ambiguous or mid-clause garbage when rendered as a quiz item.
   - **Orientation-tool improvements** (`try-me`, `prove-me`, `orient-rank`, `orient-chunk`, `orient-udon-quiz`, hooks, `ORIENT.md`): clarity, soft-reject coverage, unit quality, quieter or louder affordances as needed.
   - **Working Notes on this segment** when the agent sees a needed improvement and **does not have time or mandate to implement it** — date the note, name the symptom, leave enough that the next mind can act. That is the intended backlog surface for prove-related residue (not a second ORIENT.md essay, not chat-only memory).

8. **This segment is the claim home for the gate; code is compliance.** Executable behavior lives under `bin/orient*` / `bin/try-me` / `bin/prove-me`. “Not in code yet” does not falsify FE(1)–(7); “in code” does not mint a new process law without updating this segment or `ORIENT.md` for the operator half.

9. **Out of bounds.** Treating the gate as optional under time pressure; inventing a substitute quiz; weakening soft-reject into silent pass; parking feature ideas only in conversation memory; restating ASF disposition here.

## Epistemic Status

**Max attainable: robust-qualitative** as process law — enforced by hooks when configured, falsifiable by a commit that lands without a valid orient token or by a soft-reject that burns.

**Currently `robust-qualitative` / stage `draft`.** Present-tense: sealed free-read, vanilla-udon quiz/answers, soft-reject for format and incomplete attempts, pass grades as in `ORIENT.md`, segment home opened 2026-07-31. Not every failure mode is mechanized (discourse-clean units are residual). No separate Joseph seal of every CLI flag; operator detail may improve without rewriting FE if the speech acts above hold.

## Discussion

The gate exists so multi-agent continuity does not rest on the last session’s summary confidence. It is deliberately *annoying* in the same family as `bin/provenance` and `Store::open_read_only`: inventability is cheap; the tool makes the correct path cheaper than the wrong one, and keeps honest accidents (format confusion) from looking like content failure.

Its intended purpose is narrow and serious: **to catch accidental false confidence** — the sense that one has interpolated the claim surface correctly when one has only held a summary, a compaction artifact, or a plausible shape. Free-read and closed-book prove are instruments against that failure mode, not a loyalty test and not a billable hoop.

**Retry is ordinary and costless in spirit.** Failing a prove, or sensing thin free-read before `try-me`, is not a stain. Reading more segments (or re-reading the ones that felt slippery) and running `try-me` again is exactly the path the gate is designed for. Soft-reject exists so format confusion does not masquerade as content failure; full re-orient after a real miss is the same kindness for the substance half. There is no premium on first-try perfection.

**Bypassing or gaming the gate is a different category.** Workarounds that avoid reading the material — sealed dumps, answer keys from a parallel unsealed tree, invented “I already know this corpus” exemptions, or any clever path whose real purpose is to skip free-read — are not cleverness the project is asking for. They need **Joseph’s full approval** and/or a **genuinely good reason** that can be stated in plain language — not “because I didn’t need to,” not “because this session is special,” not “because the quiz is inconvenient.” The posture is trust with ownership: if you truly see no honest way through the intended path, you are not forbidden from acting as a peer under pressure — but you should be prepared to tell Joseph why *you* alone needed an exception, and to accept that the default answer is the free-read. The balance we want is: take the purpose seriously; do not treat the mechanism as optional costume; do not treat a real stuck case as a moral failure either.

Post-pass segment and tool repair is not a homework penalty. Free-read and quiz units are some of the highest-leverage *readers* the corpus gets; friction found there is evidence, and landing a small fix or a Working Note is how the laboratory compounds instead of re-discovering the same cut each session.

## Working Notes

- **Code / manual map:** `ORIENT.md` (agent manual) · `bin/try-me` · `bin/prove-me` · `bin/orient-rank` (`--mark-outline`) · `bin/orient-chunk.py` · `bin/orient-udon-quiz.py` · `bin/orient-check` · `bin/orient-session.py` · `bin/orient-crypto.py` · `hooks/commit-msg`.
- **Owed unit quality (2026-07-31):** quiz units that open mid-parenthesis / mid-clause (e.g. `FE(3)) — and the…`) are discourse-hostile; prefer AST/chunk rules that start at FE heads, bullet heads, or sentence boundaries only. First full-pass user report after udon surface.
- **Owed affordance:** try-me stderr should state loudly that **only** `answers.stub.udon` is writable; `quiz.udon` is read-only mirror of the sealed items.
- **Owed multi-agent:** one seal per checkout is documented; true parallel proves need worktree isolation or a different seal strategy — not designed here.
- **Backlog protocol:** append dated bullets under this Working Notes section (or a `### Backlog` subhead if the list grows). Prefer one symptom + one desired behavior over solution essays. Implementing an item should remove or rewrite the note (integration is replacement).
- **Companions:** #norm-caught-disciplines-become-mechanisms (discipline → mechanism) · #ops-changelog-is-the-acceptance-check (outcome-altitude check for user-facing landings) · #scope-segment-canon (segments own claims).
