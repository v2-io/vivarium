#!/usr/bin/env python3
"""Resolve the active Grok session for this repo and its compaction generation.

Why this exists: wall-clock TTL and bare session-id both fail the real problem —
detecting *context compaction* (mind discontinuity) vs the same oriented agent
continuing for hours. Grok persists:

  ~/.grok/active_sessions.json          → session_id + cwd + pid
  ~/.grok/sessions/<enc-cwd>/<id>/signals.json  → compactionCount

A pass is bound to (session_id, compaction_count). After compact, the count
moves and the pass dies. Same session without compact keeps the pass for as
many commits as needed. New session id → new prove.

Prints JSON to stdout:
  {"ok": true, "session_id": "...", "compaction_count": N, "signals_path": "..."}
or {"ok": false, "error": "..."}.
"""
from __future__ import annotations

import json
import os
import sys
import urllib.parse
from pathlib import Path


def repo_root() -> Path:
    # bin/orient-session.py → parent.parent is repo
    return Path(__file__).resolve().parent.parent


def find_session(cwd: Path) -> dict | None:
    active = Path.home() / ".grok" / "active_sessions.json"
    if not active.is_file():
        return None
    try:
        rows = json.loads(active.read_text(encoding="utf-8"))
    except Exception:
        return None
    cwd_r = cwd.resolve()
    # Prefer exact cwd match; then prefix (subdirs of the repo)
    exact = None
    prefix = None
    for row in rows:
        try:
            rc = Path(row.get("cwd", "")).resolve()
        except Exception:
            continue
        if rc == cwd_r:
            exact = row
            break
        if cwd_r == rc or cwd_r.is_relative_to(rc) or rc.is_relative_to(cwd_r):
            prefix = row
    return exact or prefix


def signals_path(cwd: Path, session_id: str) -> Path:
    enc = urllib.parse.quote(str(cwd.resolve()), safe="")
    return Path.home() / ".grok" / "sessions" / enc / session_id / "signals.json"


def main() -> None:
    cwd = repo_root()
    # Allow override for tests
    if os.environ.get("ORIENT_SESSION_JSON"):
        print(os.environ["ORIENT_SESSION_JSON"])
        return

    row = find_session(cwd)
    if not row:
        # try parent chain (vivarium as submodule path)
        for parent in [cwd, *cwd.parents]:
            row = find_session(parent)
            if row:
                cwd = parent
                break
    if not row:
        print(
            json.dumps(
                {
                    "ok": False,
                    "error": "no active Grok session for this cwd — cannot bind pass to compaction generation",
                }
            )
        )
        sys.exit(2)

    sid = row["session_id"]
    sp = signals_path(Path(row["cwd"]), sid)
    if not sp.is_file():
        # fallback: search under sessions for this id
        sessions = Path.home() / ".grok" / "sessions"
        found = list(sessions.glob(f"*/{sid}/signals.json"))
        if found:
            sp = found[0]
        else:
            print(
                json.dumps(
                    {
                        "ok": False,
                        "error": f"signals.json not found for session {sid}",
                        "session_id": sid,
                    }
                )
            )
            sys.exit(2)

    try:
        sig = json.loads(sp.read_text(encoding="utf-8"))
    except Exception as e:
        print(json.dumps({"ok": False, "error": f"read signals: {e}"}))
        sys.exit(2)

    count = int(sig.get("compactionCount", 0))
    print(
        json.dumps(
            {
                "ok": True,
                "session_id": sid,
                "compaction_count": count,
                "signals_path": str(sp),
                "turn_count": sig.get("turnCount"),
                "pid": row.get("pid"),
            }
        )
    )


if __name__ == "__main__":
    main()
