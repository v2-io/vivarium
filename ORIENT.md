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
# core/src is now sealed — closed book
# edit the stub (multi-line OK):
$EDITOR .orient/answers.stub.txt
bin/prove-me --ready .orient/answers.stub.txt
# put the printed orient:… token in your commit message
```

Useful:

| Command | Effect |
|---------|--------|
| `bin/prove-me` | Re-show the open quiz + answer format (does **not** grade) |
| `bin/prove-me --help` | Help only — never grades, never unseals |
| `bin/prove-me --ready FILE` | Submit answers (`-` = stdin) |
| `bin/prove-me --template` | Refresh `.orient/answers.stub.txt` |
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

Each item is a **unit** from a claim segment (markdown block: FE clause,
Working Notes bullet, or top-level paragraph — not a glued multi-bullet
blob). You see:

- **start:** first few words  
- **end:** last few words (when the completion is long)  
- breadcrumb (segment + section) and slug  

Write what sits between start and the end of that unit (or the full unit).
Matching ignores case, punctuation, and extra whitespace.

## One quiz at a time

There is **one** sealed test per checkout: a single `.orient/test.jsonl` and a
shared chmod seal of `core/src`. Parallel agents on the same tree will clobber
each other. Multiple *passes* (different sessions) can coexist under
`.orient/passes/`; only one live quiz can.

If `try-me` reports a live test still open, finish it or `bin/try-me --force`.
