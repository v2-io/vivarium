# Agent orientation gate — prove Level A before any commit

## Why this exists

After context compaction (and often without it), an agent can feel fully oriented
while being **blind to the largest holes**. Restating "read Claude.md" is a
discipline; disciplines fail under false coherence
(`#norm-caught-disciplines-become-mechanisms`).

This gate **refuses `git commit`** until a **closed-book** prove passes against
the **current** Level-A corpus. Open-book tricks (read a range after the
question appears) are **rejected by protocol**: required files are **unreadable
for the duration of the prove**, so answers must already be in the session's
working memory from a prior full read.

## Protocol (do not skip steps)

### 1. Study (open files; read fully)

Read every path listed by:

```bash
bin/orient-study --list
```

Then record that the bytes you studied match the tree **now**:

```bash
bin/orient-study
```

This writes `.orient/study` with content digests. It does **not** grant commit
permission. It only freezes *which* corpus you claim to have read.

### 2. Prove (closed-book; files sealed)

```bash
bin/orient-prove
```

What happens:

1. Verifies `.orient/study` is present, **fresh** (default ≤ 2 hours), and
   digests still match disk (any edit to Level A invalidates study).
2. **Seals** every required path (`chmod a-r`) so tool reads fail mid-quiz.
3. Prints questions **only after** seal.
4. Reads **all answers in one batch** from stdin (or `--answers FILE` prepared
   **before** invoke only if you already know them — not by opening sealed files).
5. Checks answers against **SHA-256 of normalized text** (plaintext answers are
   **not** in the repo).
6. **Unseals** always (trap on exit).
7. On success writes `.orient/pass` (short-lived token + corpus hash).

If you compacted after study: you still have digests, but not the content —
prove fails. That is the point. Re-read, re-study, prove again.

### 3. Commit

```bash
git commit ...   # pre-commit runs bin/orient-check
```

`bin/orient-check` requires a valid `.orient/pass` whose corpus hash matches
disk and whose age is within the pass TTL (default 60 minutes).

## What this proves / does not prove

| Proves | Does not prove |
|---|---|
| You processed current Level-A bytes (study digests) | Perfect understanding |
| You can answer norm/telos questions **without re-reading during the quiz** | Good taste on every task |
| Compaction amnesia fails the gate | Humans cannot override (use `ORIENT_SKIP=1` only Joseph) |

## Files

| Path | Role |
|---|---|
| `ORIENT.md` | This protocol |
| `bin/orient-study` | List + digest commitment |
| `bin/orient-prove` | Seal → quiz → unseal → pass |
| `bin/orient-check` | Pre-commit / CI gate |
| `bin/orient-quiz.sha` | Question ids + accepted answer hashes (no plaintext) |
| `.orient/` | Local study/pass state (**gitignored**) |

## Joseph override

```bash
ORIENT_SKIP=1 git commit ...
```

Logged to stderr. Not for agents.
