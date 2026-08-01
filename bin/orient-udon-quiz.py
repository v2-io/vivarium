#!/usr/bin/env python3
"""Vanilla-udon surface for orientation quiz + answer stub.

No full udon engine: just emit and recognize the small shape agents already
meet in LEXICON / DECISIONS. Doubles as a first real-world udon exercise.

Quiz document
-------------
  |quiz[<test_id>]
    :items N
    :deadline <unix>
    :git-head <short>
    :stub .orient/answers.stub.udon

    |item[P1]
      :slug …
      :section …
      :start "first words…"
      :end "last words…"     ; only when the completion is long
      :words N               ; approx words to complete after :start

Answers (separate file — fill the bodies)
-----------------------------------------
  |answer[P1]
    completion prose…

  |answer[P2]
    …
"""
from __future__ import annotations

from pathlib import Path


def udon_string(s: str) -> str:
    """Quote a scalar for vanilla udon (no in-string escapes)."""
    s = " ".join(str(s).split())  # collapse whitespace to one line
    if '"' not in s:
        return f'"{s}"'
    if "'" not in s:
        return f"'{s}'"
    # both present: drop " so double-quoted form is legal
    return f'"{s.replace(chr(34), "")}"'


def format_quiz_udon(
    items: list[dict],
    *,
    test_id: str,
    deadline: int | str,
    git_head: str = "",
    stub_path: str = ".orient/answers.stub.udon",
    n: int | None = None,
) -> str:
    """Human-facing quiz as vanilla udon (stdout + .orient/quiz.udon)."""
    n = n if n is not None else len(items)
    lines: list[str] = [
        "; Closed-book orientation quiz — core/src is sealed.",
        "; Vanilla udon (same family as LEXICON.udon / DECISIONS.decision-log.udon).",
        ";",
        "; For each |item[Pn]: write the rest of that unit (after :start, through",
        "; :end if present) as the body of |answer[Pn] in the stub file.",
        "; You may paste the full unit, middle only, or middle+:end — all match.",
        f"; Grade: bin/prove-me --ready {stub_path}",
        "; Re-show: bin/prove-me     Help only: bin/prove-me --help",
        "",
        f"|quiz[{test_id}]",
        f"  :items {n}",
        f"  :deadline {deadline}",
    ]
    if git_head:
        lines.append(f"  :git-head {git_head}")
    lines.append(f"  :stub {stub_path}")
    lines.append("")

    for it in items:
        iid = it.get("id", "?")
        lines.append(f"  |item[{iid}]")
        if it.get("slug"):
            lines.append(f"    :slug {it['slug']}")
        if it.get("breadcrumb"):
            lines.append(f"    :section {udon_string(it['breadcrumb'])}")
        if it.get("title_hint"):
            lines.append(f"    :outline-title {udon_string(it['title_hint'])}")
        lines.append(f"    :start {udon_string(it.get('prefix', ''))}")
        if it.get("suffix"):
            lines.append(f"    :end {udon_string(it['suffix'])}")
        elif it.get("last_word_hint"):
            lines.append(f"    :end {udon_string(it['last_word_hint'])}")
        if it.get("rest_words") is not None:
            lines.append(f"    :words {it['rest_words']}")
        lines.append(
            f"    ; → fill |answer[{iid}] in {stub_path} with the completion"
        )
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def format_answers_stub(
    items: list[dict],
    *,
    test_id: str = "",
    stub_cmd: str = "bin/prove-me --ready .orient/answers.stub.udon",
) -> str:
    """Empty |answer bodies ready for the agent to fill."""
    lines: list[str] = [
        "; Orientation answers — vanilla udon.",
        f"; Fill each |answer body, then: {stub_cmd}",
        "; Middle only, middle+:end, or full unit text — all match.",
        "; Matching ignores case, punctuation, and extra whitespace.",
        "; This file is the counterpart to .orient/quiz.udon (|item ↔ |answer).",
    ]
    if test_id:
        lines.append(f"; quiz {test_id}")
    lines.append("")

    for it in items:
        iid = it.get("id", "?")
        lines.append(f"|answer[{iid}]")
        if it.get("slug") or it.get("breadcrumb"):
            crumb = it.get("breadcrumb") or ""
            slug = it.get("slug") or ""
            lines.append(f"  ; #{slug} — {crumb}".rstrip(" —"))
        if it.get("prefix"):
            lines.append(f"  ; start: {it['prefix']}")
        if it.get("suffix"):
            lines.append(f"  ; end:   {it['suffix']}")
        elif it.get("last_word_hint"):
            lines.append(f"  ; end:   {it['last_word_hint']}")
        if it.get("rest_words") is not None:
            lines.append(f"  ; (~{it['rest_words']} words after start)")
        lines.append("  ")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def write_pair(
    items: list[dict],
    *,
    test_id: str,
    deadline: int | str,
    git_head: str,
    orient_dir: Path,
) -> tuple[Path, Path]:
    """Write quiz.udon + answers.stub.udon under orient_dir."""
    orient_dir = Path(orient_dir)
    orient_dir.mkdir(parents=True, exist_ok=True)
    quiz_path = orient_dir / "quiz.udon"
    stub_path = orient_dir / "answers.stub.udon"
    rel_stub = ".orient/answers.stub.udon"
    quiz_path.write_text(
        format_quiz_udon(
            items,
            test_id=test_id,
            deadline=deadline,
            git_head=git_head,
            stub_path=rel_stub,
        ),
        encoding="utf-8",
    )
    stub_path.write_text(
        format_answers_stub(items, test_id=test_id),
        encoding="utf-8",
    )
    return quiz_path, stub_path
