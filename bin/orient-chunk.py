#!/usr/bin/env python3
"""Quiz-unit chunking for try-me — markdown block AST (markdown-it), not blank-line glue.

Units:
  - top-level paragraphs (not inside a list)
  - top-level list items (FE numbered clauses, Working Notes bullets, …)
    Nested sub-bullets stay *inside* the parent item (not separate quiz units).
  - tables, fences, pure headings: never units

Long units (> MAX_UNIT_WORDS) are split on blank lines, then on sentence
boundaries if still oversized — so a 900-word FE clause is not one completion.
"""
from __future__ import annotations

import re
from pathlib import Path

MIN_WORDS = 12
MAX_UNIT_WORDS = 120  # soft: split when longer
MAX_UNIT_HARD = 220  # never emit a larger completion target

try:
    from markdown_it import MarkdownIt
except ImportError as e:  # pragma: no cover
    raise SystemExit(
        "orient-chunk: needs markdown-it-py (markdown_it). "
        f"Import failed: {e}"
    ) from e


def _strip_frontmatter(text: str) -> str:
    if not text.startswith("---"):
        return text
    m = re.search(r"\n---\s*\n", text[3:])
    if not m:
        return text
    return text[3 + m.end() :]


def _word_count(s: str) -> int:
    return len(s.split())


def _split_oversized(chunk: str) -> list[str]:
    """Break a long block into quiz-sized pieces without inventing text."""
    chunk = chunk.strip()
    if not chunk:
        return []
    if _word_count(chunk) <= MAX_UNIT_WORDS:
        return [chunk]

    parts = [p.strip() for p in re.split(r"\n\s*\n", chunk) if p.strip()]
    if len(parts) == 1:
        # sentence-ish split (keep delimiters attached to the left)
        parts = [
            p.strip()
            for p in re.split(r"(?<=[.!?])\s+(?=[A-Z0-9*`\"'(])", chunk)
            if p.strip()
        ]

    out: list[str] = []
    buf: list[str] = []
    buf_w = 0
    for p in parts:
        w = _word_count(p)
        if w > MAX_UNIT_HARD:
            # last resort: fixed word windows with overlap of 0
            words = p.split()
            step = MAX_UNIT_WORDS
            for i in range(0, len(words), step):
                window = " ".join(words[i : i + step]).strip()
                if _word_count(window) >= MIN_WORDS:
                    out.append(window)
            continue
        if buf and buf_w + w > MAX_UNIT_WORDS:
            out.append("\n\n".join(buf) if len(buf) > 1 else buf[0])
            buf, buf_w = [], 0
        buf.append(p)
        buf_w += w
    if buf:
        out.append("\n\n".join(buf) if len(buf) > 1 else buf[0])
    return [c for c in out if _word_count(c) >= MIN_WORDS]


def parse_segment(path: Path, root: Path | None = None) -> list[dict]:
    """Return quiz units with breadcrumb context for one segment file."""
    path = Path(path)
    root = Path(root) if root else path.parent.parent.parent
    try:
        rel = str(path.relative_to(root))
    except ValueError:
        rel = str(path)
    slug = path.stem
    raw = path.read_text(encoding="utf-8", errors="replace")
    body = _strip_frontmatter(raw)
    lines = body.splitlines()
    md = MarkdownIt("commonmark")
    tokens = md.parse(body)

    heads = ("", "", "")
    units: list[dict] = []
    seen: set[str] = set()

    def crumb() -> str:
        return " > ".join(x for x in heads if x) or slug

    def emit(text: str) -> None:
        text = text.strip()
        if not text:
            return
        # Drop table rows / fence lines so a FE clause with a table is prose-only.
        cleaned_lines = []
        in_fence = False
        for ln in text.splitlines():
            s = ln.strip()
            if s.startswith("```"):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            if s.startswith("|"):
                continue
            cleaned_lines.append(ln)
        text = "\n".join(cleaned_lines).strip()
        if not text:
            return
        stripped = text.lstrip()
        if stripped.startswith("#"):
            return
        key = re.sub(r"\s+", " ", text)
        if key in seen:
            return
        for piece in _split_oversized(text):
            k2 = re.sub(r"\s+", " ", piece)
            if k2 in seen:
                continue
            if _word_count(piece) < MIN_WORDS:
                continue
            seen.add(k2)
            units.append(
                {
                    "path": rel,
                    "slug": slug,
                    "breadcrumb": crumb(),
                    "paragraph": piece,
                }
            )

    i = 0
    n = len(tokens)
    while i < n:
        t = tokens[i]
        if t.type == "heading_open":
            level = len(t.markup or "#")
            content = ""
            if i + 1 < n and tokens[i + 1].type == "inline":
                content = tokens[i + 1].content.strip()
            if level <= 1:
                heads = (content, "", "")
            elif level == 2:
                heads = (heads[0], content, "")
            else:
                heads = (heads[0], heads[1], content)
            i += 1
            continue

        if t.type in ("fence", "code_block", "html_block"):
            i += 1
            continue
        if t.type == "table_open":
            # skip until table_close
            i += 1
            while i < n and tokens[i].type != "table_close":
                i += 1
            i += 1
            continue

        # Top-level list items only (level 1): FE clauses, WN bullets
        if t.type == "list_item_open" and t.level == 1 and t.map:
            a, b = t.map
            a = max(0, a)
            b = min(len(lines), b)
            emit("\n".join(lines[a:b]))
            i += 1
            continue

        # Top-level paragraphs only (not nested inside lists)
        if t.type == "paragraph_open" and t.level == 0 and t.map:
            a, b = t.map
            a = max(0, a)
            b = min(len(lines), b)
            emit("\n".join(lines[a:b]))
            i += 1
            continue

        i += 1

    return units


def collect_pool(src: Path, root: Path | None = None) -> list[dict]:
    src = Path(src)
    root = Path(root) if root else src.parent.parent
    pool: list[dict] = []
    for p in sorted(src.rglob("*.md")):
        pool.extend(parse_segment(p, root=root))
    return pool


if __name__ == "__main__":
    import sys

    root = Path(__file__).resolve().parent.parent
    src = root / "core" / "src"
    pool = collect_pool(src, root=root)
    print(f"units={len(pool)}", file=sys.stderr)
    if len(sys.argv) > 1:
        slug = sys.argv[1]
        for u in pool:
            if u["slug"] == slug:
                w = _word_count(u["paragraph"])
                print(f"{w:4d}  {u['breadcrumb'][:50]}  |  {u['paragraph'][:70]!r}")
