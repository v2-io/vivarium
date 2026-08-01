# Agent orientation gate

You must be oriented to this project’s claim surface and working norms before
you commit.

## Free-read (before the quiz)

1. **`core/OUTLINE.md` always** — the ordering of claim space.
2. **Starred rows (★)** in that outline — the current high-importance set.
   Most quiz items are drawn from these. Read them **in outline order**, not
   by any score. Stars move when recent work / structural importance moves;
   refresh with `bin/orient-rank --mark-outline` (maintainers).
3. **Whatever you are about to work on**, even if unstarred.

Do **not** invent a substitute quiz. Do **not** dump sealed files after
`try-me` — that is cheating and will get you fired.

## When you believe you are ready

```bash
bin/try-me
# core/src sealed. Quiz prints as vanilla udon; also on disk:
#   .orient/quiz.udon          — the questions (|item[Pn])
#   .orient/answers.stub.udon  — empty |answer[Pn] bodies to fill
$EDITOR .orient/answers.stub.udon
bin/prove-me --ready .orient/answers.stub.udon
# put the printed orient:… token in your commit message
```

**Quiz and answers are the same notation** — vanilla udon (the family of
`LEXICON.udon` / `DECISIONS.decision-log.udon`). No advanced syntax; no full
parser. Completing the quiz is also a first real-world udon exercise.

```udon
; from .orient/quiz.udon (issued by try-me)
|quiz[…]
  |item[P1]
    :slug some-segment
    :section "… > Formal Expression"
    :start "first few words of the unit"
    :end "last few words"          ; when the completion is long
    :words 55

; you fill .orient/answers.stub.udon
|answer[P1]
  the rest of that unit after :start…
```

`;` is a comment. Indent children under their element. Matching ignores case,
punctuation, and extra space. You may paste the full unit, middle only, or
middle+`:end`.

Useful:

| Command | Effect |
|---------|--------|
| `bin/prove-me` | Re-print `.orient/quiz.udon` (does **not** grade) |
| `bin/prove-me --help` | Help only — never grades, never unseals |
| `bin/prove-me --ready FILE` | Submit answers (`-` = stdin) |
| `bin/prove-me --template` | Refresh quiz + answers stub from the open test |
| `bin/orient-unseal` | Emergency unseal if a tool left `core/src` locked |

## Pass grades (default 5 items)

| Score | Token |
|------:|--------|
| **5/5** | Unlimited commits while this session’s compaction gen still matches |
| **3/5 or 4/5** | **One-time** commit; by accepting you **promise to free-read** the sections you missed (listed by prove-me) |
| **&lt;3/5** | Fail — re-orient, `bin/try-me` again |

A one-time pass is spent on the first successful commit that carries its
`orient:<session_id>`. Compaction invalidates the pass for that session —
re-prove after compact.

## Quiz shape

Each `|item` is a **unit** from a claim segment (FE clause, Working Notes
bullet, or top-level paragraph — not a glued multi-bullet blob). `:start` /
`:end` are word anchors so long units stay fillable from free-read memory.

## One quiz at a time

There is **one** sealed test per checkout: a single `.orient/test.jsonl` and a
shared chmod seal of `core/src`. Parallel agents on the same tree will clobber
each other. Multiple *passes* (different sessions) can coexist under
`.orient/passes/`; only one live quiz can.

If `try-me` reports a live test still open, finish it or `bin/try-me --force`.

## Soft rejects (do not burn the test)

If the submit is not yet a real proving attempt, `prove-me --ready` **does not
grade**, **does not unseal**, and **does not burn** while time remains:

- format / empty / misunderstanding (legacy `P1:` / JSON, pasted `|quiz`/`|item`,
  wrong element name, headers with no body, comment-only stub)
- **fewer than 3 substantial answers** (body longer than 5 characters after
  parse) — finish more items, then re-submit

A real graded attempt (≥3 substantial `|answer` bodies) that scores under the
bar still burns as usual.

## After you have commit credentials

A pass is not only a license to leave. Among the **first legitimate uses** of
commit access — before greenfield wander — are improvements the free-read or
quiz made obvious:

1. **Segment repairs** — formatting, staleness, dual-home residue, wording that
   was ambiguous or mid-clause when it showed up as a quiz unit.
2. **Orientation-tool repairs** — `try-me` / `prove-me` / `orient-rank` /
   chunking / udon surface / this manual.
3. **Working Notes on `#ops-orientation-gate`** when you see a needed
   improvement and do **not** have time to implement it — date the note, name
   the symptom, leave enough for the next mind. That segment is the durable
   backlog for prove-related residue (not chat memory alone).

Claim home for the gate process: `#ops-orientation-gate`. This file stays the
short operator manual.
