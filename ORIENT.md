# Agent orientation gate (v2 — random segment completion)

## Problem

After context compaction an agent can feel oriented while being blind to real
segment content. Outline-only confidence is especially dangerous. Fixed quizzes
are gameable; open-book “quote this range” is gameable if tools can re-read
between Q and A.

## Protocol

```
1. Read freely (core/src, Claude.md, FORMAT, ETHICS, …)
2. bin/try-me
     - samples ~12 paragraphs at random (SystemRandom / os.urandom)
       from core/src/**/*.md  WITHOUT REPLACEMENT
     - does NOT use core/OUTLINE.md as a passage source
     - SEALS core/src (chmod a-r) so segments cannot be re-opened
     - writes .orient/test.jsonl  (prefix hints + answer hashes only)
3. Write answers from memory (completion after each prefix)
4. bin/prove-me answers.txt
     - same normalization as try-me (lower, collapse space, strip punct)
     - flexible match (prefix prepending, etc.)
     - wrong <= 1  → pass (provisional if any wrong)
     - else burn test_id, must try-me again
     - prints  orient:<32-hex>   once; stores only the hash
5. git commit -m "orient:TOKEN …"
     - commit-msg hook verifies token, consumes it (one-shot)
```

### Why not “session id on a whitelist”?

Session ids often **survive compaction**. A disk whitelist then re-grants commit
to a mind that no longer holds the segments. Instead:

- **Raw token is only in prove-me stdout / agent context**, not stored on disk.
- Disk keeps `sha256(token)` + uses_left=1.
- Commit message must present the raw token; hook burns the pass.
- After compaction the agent **forgets the token** → must try-me / prove-me again.

### Emergency unseal

If a process crashes mid-seal:

```bash
bin/orient-unseal
```

### Joseph override

```bash
ORIENT_SKIP=1 git commit …
```

## Commands

| Command | Role |
|---|---|
| `bin/try-me` | Random sealed test → `.orient/test.jsonl` |
| `bin/prove-me [answers]` | Grade; print one-shot `orient:TOKEN` |
| `bin/orient-check` | Status / commit-msg verify |
| `bin/orient-unseal` | Restore core/src read bits |

## Residual attacks (honest)

| Attack | Notes |
|---|---|
| Pre-extract answers before try-me | Requires having read; still must pass random items |
| Paste token from a log file | Operational hygiene; token is one-shot |
| ORIENT_SKIP | Joseph only |
| chmod restore during quiz | Parallel tool race; residual |

## Install hooks

```bash
git config core.hooksPath hooks
```
