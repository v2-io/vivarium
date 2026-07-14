# core — the vivarium specification

The specification of what vivarium is and must do, written as **claim segments**: one claim per file in `core/src/`, each stating one thing that is defined, follows, or is restricted, given what came before. **This file carries the ordering. The slug carries the identity. [`FORMAT.md`](../FORMAT.md) carries the rules.**

> ### ⚖ Read [`FORMAT.md`](../FORMAT.md) before writing or reviewing a segment.
>
> It holds the frontmatter (`type` / `status` / `stage` / `depends`), the promotion gates, the epistemic triage, and the document cadence — and, in its second half, the cross-reference scheme, the prose voice, and the math rules, **which bind every file in this repository and not only segments**.

---

## The outline

**Segments written: 0. Segments named: none yet.** The tables belong here, ordered conceptually; the order will change often and costs nothing to change, because a segment is referenced by slug and never by position.

**A missing segment is a `--GAP--` row** — typed and described, in the same table as the present ones. A named absence is part of the structure; an unnamed one is a hole nobody can see.

## Where the specification currently lives

It does not live in one place, and that is the problem this directory exists to solve. It is distributed across the corpus — `doc/`, `ref/`, `LEXICON.udon`, `DECISIONS.decision-log.udon`, `tabularium/` — and no single artifact contains it.

The front doors that used to summarize that corpus are in `.archive/`, deliberately: **the assembly is to be made from the sources, not from a prior summary of them.**
