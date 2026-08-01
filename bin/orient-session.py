#!/usr/bin/env python3
"""Resolve a Grok/Claude session and its compaction generation.

Identity is the **session id**, not "who owns this cwd".

Joseph's sketch (commit gate):
  prove-me stamps this session_id into a pass and prints orient:<session_id>
  commit-msg pulls that id → find harness (claude or grok) → live compaction gen
  if gen drifted (compacted) → re-prove

Generation recipes
------------------
Grok:   sha256 of ~/.grok/sessions/.../<id>/compaction/segment_* listing
        (name\\tsize\\tmtime_ns) — signals.compactionCount can lag
Claude: sha256 of compact_boundary events in
        ~/.claude/projects/<slug>/<id>.jsonl (main session only)

CLI
---
  bin/orient-session.py              → resolve *this* process's session
  bin/orient-session.py --id <uuid>  → resolve by id (commit-msg path)
  ORIENT_SESSION_JSON=...            → override (tests)

Prints JSON: {ok, harness, session_id, compaction_gen, boundary_count, ...}
"""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import urllib.parse
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parent.parent


def project_slug(cwd: Path) -> str:
    return "".join(c if c.isalnum() else "-" for c in str(cwd.resolve()))


def pid_alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    except OSError:
        return False


def parent_pids(start: int | None = None) -> list[int]:
    """Walk parent chain from start (default: this process)."""
    pid = start if start is not None else os.getpid()
    out: list[int] = []
    seen: set[int] = set()
    for _ in range(32):
        if pid <= 1 or pid in seen:
            break
        seen.add(pid)
        out.append(pid)
        try:
            line = subprocess.check_output(
                ["ps", "-o", "ppid=", "-p", str(pid)],
                text=True,
                stderr=subprocess.DEVNULL,
            ).strip()
            pid = int(line)
        except Exception:
            break
    return out


# ── Grok gen ──────────────────────────────────────────────────────────────────


def grok_session_dir(session_id: str) -> Path | None:
    sessions = Path.home() / ".grok" / "sessions"
    if not sessions.is_dir():
        return None
    hits = list(sessions.glob(f"*/{session_id}"))
    return hits[0] if hits else None


def grok_gen(session_dir: Path) -> tuple[str, int, list[str]]:
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
            lines.append(f"{p.name}\t{st.st_size}\t{st.st_mtime_ns}")
            names.append(p.name)
    payload = "\n".join(lines) + ("\n" if lines else "")
    return hashlib.sha256(payload.encode()).hexdigest(), len(names), names


def resolve_grok_id(session_id: str) -> dict | None:
    sdir = grok_session_dir(session_id)
    if sdir is None:
        return None
    gen, n, names = grok_gen(sdir)
    out: dict = {
        "ok": True,
        "harness": "grok",
        "session_id": session_id,
        "compaction_gen": gen,
        "boundary_count": n,
        "boundaries": names,
        "session_dir": str(sdir),
    }
    sig = sdir / "signals.json"
    if sig.is_file():
        try:
            s = json.loads(sig.read_text(encoding="utf-8"))
            out["signals_compaction_count"] = s.get("compactionCount")
            out["turn_count"] = s.get("turnCount")
        except Exception:
            pass
    return out


def find_grok_by_pid(pids: set[int]) -> dict | None:
    active = Path.home() / ".grok" / "active_sessions.json"
    if not active.is_file():
        return None
    try:
        rows = json.loads(active.read_text(encoding="utf-8"))
    except Exception:
        return None
    for row in rows:
        try:
            pid = int(row.get("pid", -1))
        except (TypeError, ValueError):
            continue
        if pid in pids and pid_alive(pid):
            sid = row.get("session_id")
            if not sid:
                continue
            resolved = resolve_grok_id(sid)
            if resolved:
                resolved["pid"] = pid
                return resolved
    return None


# ── Claude gen ────────────────────────────────────────────────────────────────


def claude_jsonl(session_id: str) -> Path | None:
    projects = Path.home() / ".claude" / "projects"
    if not projects.is_dir():
        return None
    hits = list(projects.glob(f"*/{session_id}.jsonl"))
    return hits[0] if hits else None


def claude_gen(jsonl: Path) -> tuple[str, int, list[str]]:
    lines: list[str] = []
    uuids: list[str] = []
    if not jsonl.is_file():
        return hashlib.sha256(b"").hexdigest(), 0, []
    with jsonl.open("r", encoding="utf-8", errors="replace") as f:
        for line in f:
            if "compact_boundary" not in line:
                continue
            try:
                o = json.loads(line)
            except json.JSONDecodeError:
                continue
            if o.get("type") != "system" or o.get("subtype") != "compact_boundary":
                continue
            if o.get("isSidechain") or o.get("agentId"):
                continue
            cm = o.get("compactMetadata") or o.get("compact_metadata") or {}
            uid = str(o.get("uuid") or "")
            ts = str(o.get("timestamp") or "")
            trigger = str(cm.get("trigger") or "")
            pretok = cm.get("preTokens", cm.get("pre_tokens", ""))
            lines.append(f"{uid}\t{ts}\t{trigger}\t{pretok}")
            uuids.append(uid or ts)
    payload = "\n".join(lines) + ("\n" if lines else "")
    return hashlib.sha256(payload.encode()).hexdigest(), len(uuids), uuids


def resolve_claude_id(session_id: str) -> dict | None:
    jsonl = claude_jsonl(session_id)
    if jsonl is None:
        return None
    gen, n, uuids = claude_gen(jsonl)
    return {
        "ok": True,
        "harness": "claude",
        "session_id": session_id,
        "compaction_gen": gen,
        "boundary_count": n,
        "boundaries": uuids,
        "transcript": str(jsonl),
    }


def find_claude_by_pid(pids: set[int]) -> dict | None:
    sessions = Path.home() / ".claude" / "sessions"
    if not sessions.is_dir():
        return None
    for p in sessions.glob("*.json"):
        if not p.stem.isdigit():
            continue
        try:
            row = json.loads(p.read_text(encoding="utf-8"))
        except Exception:
            continue
        pid = int(row.get("pid", p.stem))
        if pid not in pids or not pid_alive(pid):
            continue
        sid = row.get("sessionId") or row.get("session_id")
        if not sid:
            continue
        resolved = resolve_claude_id(sid)
        if resolved:
            resolved["pid"] = pid
            return resolved
    return None


# ── public resolve ────────────────────────────────────────────────────────────


def resolve_by_id(session_id: str) -> dict:
    """Find session_id under Grok or Claude (either order)."""
    sid = session_id.strip()
    # Prefer exact presence: grok dir vs claude jsonl
    g = resolve_grok_id(sid)
    c = resolve_claude_id(sid)
    if g and c:
        # Ambiguous rare collision — prefer whichever has more boundaries / newer
        # Prefer grok if both (different namespaces almost always)
        return g
    if g:
        return g
    if c:
        return c
    return {
        "ok": False,
        "error": f"session id not found under ~/.grok or ~/.claude: {sid}",
        "session_id": sid,
    }


def resolve_this() -> dict:
    """This process: walk parents → match live harness pid → that session."""
    pids = set(parent_pids())
    forced = (os.environ.get("ORIENT_HARNESS") or "").strip().lower()
    if forced == "claude":
        return find_claude_by_pid(pids) or {
            "ok": False,
            "harness": "claude",
            "error": "no live Claude session for this process tree",
        }
    if forced == "grok":
        return find_grok_by_pid(pids) or {
            "ok": False,
            "harness": "grok",
            "error": "no live Grok session for this process tree",
        }
    # Prefer Grok if GROK_AGENT set (this shell is Grok Build)
    if os.environ.get("GROK_AGENT"):
        g = find_grok_by_pid(pids)
        if g:
            return g
    c = find_claude_by_pid(pids)
    if c:
        return c
    g = find_grok_by_pid(pids)
    if g:
        return g
    return {
        "ok": False,
        "error": "no live Grok/Claude session matched this process tree",
        "pids_sampled": sorted(pids)[:12],
    }


def main() -> None:
    if os.environ.get("ORIENT_SESSION_JSON"):
        print(os.environ["ORIENT_SESSION_JSON"])
        return

    args = sys.argv[1:]
    if args and args[0] in ("--id", "-i") and len(args) >= 2:
        out = resolve_by_id(args[1])
    elif args and not args[0].startswith("-"):
        out = resolve_by_id(args[0])
    else:
        out = resolve_this()

    print(json.dumps(out))
    if not out.get("ok"):
        sys.exit(2)


if __name__ == "__main__":
    main()
