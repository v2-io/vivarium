#!/usr/bin/env python3
"""Resolve the active Grok session for this repo and its compaction generation.

Why this exists: wall-clock TTL and bare session-id both fail the real problem —
detecting *context compaction* (mind discontinuity) vs the same oriented agent
continuing for hours.

Grok persists, under ~/.grok/:

  active_sessions.json                     → session_id + cwd + pid
  sessions/<enc-cwd>/<id>/compaction/segment_*.md
      → one file per completed compact (on-disk, not laggy telemetry)

signals.json also has compactionCount, but it can lag behind a finished compact
(observed: segment_003 written while compactionCount stayed frozen). The
generation id is therefore:

  sha256( sorted listing of segment basenames + size + mtime_ns )

i.e. the equivalent of hashing `ls -1 compaction/segment_*` with enough
metadata that a replace of a segment is also a new generation.

A pass is bound to (session_id, compaction_gen). New segment after compact →
hash moves → re-prove. Same session without compact keeps the pass. New
session id → re-prove.

Prints JSON to stdout:
  {"ok": true, "session_id": "...", "compaction_gen": "…", "segment_count": N, …}
or {"ok": false, "error": "..."}.
"""
from __future__ import annotations

import hashlib
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


def session_dir_for(cwd: Path, session_id: str) -> Path | None:
    """Locate ~/.grok/sessions/<enc>/<session_id>/."""
    enc = urllib.parse.quote(str(cwd.resolve()), safe="")
    direct = Path.home() / ".grok" / "sessions" / enc / session_id
    if direct.is_dir():
        return direct
    sessions = Path.home() / ".grok" / "sessions"
    if not sessions.is_dir():
        return None
    found = list(sessions.glob(f"*/{session_id}"))
    return found[0] if found else None


def compaction_gen(session_dir: Path) -> tuple[str, int, list[str]]:
    """Hash of the segment listing under session_dir/compaction/.

    Empty dir / missing dir → stable empty gen (first compact still moves it).
    """
    comp = session_dir / "compaction"
    lines: list[str] = []
    names: list[str] = []
    if comp.is_dir():
        segs = sorted(
            p
            for p in comp.iterdir()
            if p.is_file() and p.name.startswith("segment_")
        )
        for p in segs:
            st = p.stat()
            # basename + size + mtime_ns — listing shape, not full content
            lines.append(f"{p.name}\t{st.st_size}\t{st.st_mtime_ns}")
            names.append(p.name)
    payload = "\n".join(lines) + ("\n" if lines else "")
    digest = hashlib.sha256(payload.encode("utf-8")).hexdigest()
    return digest, len(names), names


def main() -> None:
    cwd = repo_root()
    if os.environ.get("ORIENT_SESSION_JSON"):
        print(os.environ["ORIENT_SESSION_JSON"])
        return

    row = find_session(cwd)
    if not row:
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
    sdir = session_dir_for(Path(row["cwd"]), sid)
    if sdir is None:
        # last chance: search by id alone
        sdir = session_dir_for(cwd, sid)
    if sdir is None:
        print(
            json.dumps(
                {
                    "ok": False,
                    "error": f"session dir not found for {sid}",
                    "session_id": sid,
                }
            )
        )
        sys.exit(2)

    gen, n_seg, names = compaction_gen(sdir)
    out = {
        "ok": True,
        "session_id": sid,
        "compaction_gen": gen,
        "segment_count": n_seg,
        "segments": names,
        "session_dir": str(sdir),
        "pid": row.get("pid"),
    }
    # Soft corroboration only — may lag; not used for pass validity.
    sig = sdir / "signals.json"
    if sig.is_file():
        try:
            s = json.loads(sig.read_text(encoding="utf-8"))
            out["signals_compaction_count"] = s.get("compactionCount")
            out["turn_count"] = s.get("turnCount")
        except Exception:
            pass
    print(json.dumps(out))


if __name__ == "__main__":
    main()
