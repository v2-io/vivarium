# vivarium — TODO

*Tactical open items. Ongoing thinking lives in `ORIENTATION.md` (current state),
the `DESIGN-*` docs (design), and `SUPERSEDED.md` (what's been retired/replaced).
This file is the actionable queue.*

## Lexicon / terminology-system reconciliation (archema-wide) — deferred, tracked

Vivarium's `LEXICON.md` is a **hand-authored** markdown file. ASF has a more mature system we'll want to converge on archema-wide (Joseph, 2026-07-09) — for now this is parked so we can stay focused on lexicon content, architecture, and next steps.

The ASF conventions worth adopting (`~/src/archema-io/asf/`, `doc/sop/format.sop.md`
+ `terminology/README.md`):

- **Generated LEXICON from per-entry files.** ASF's `LEXICON.md` is *generated* by
  `bin/term render` from `terminology/entries/<slug>.md` (frontmatter + prose body),
  with append-only decision events — never hand-edited. Per-entry files make concurrent multi-agent edits merge-clean instead of colliding on table rows.
  *Decision needed:* port `bin/term` into vivarium, or run one shared terminology store across archema members (the charter's cross-member concern).
- **Format/rendering conventions** (partly applied to `LEXICON.md` §4/§7/§8 already):
  - **Math is `$…$` LaTeX**, not bare Unicode, with GitHub/Obsidian-compat rules
    (`\vert`/`\lt`/`\gt`/`\ast`, `\mathcal{}`, no inner spaces). *Done:* bare-math in
    §5/§7/§8 converted. *Pending:* a full pass over §1–§6 (prior-session Unicode).
  - **Segment-voice** — state what *is*, not what *changed*; provenance/retired-term archaeology → history layer (`SUPERSEDED.md`), not the lexicon body. *Done for the sections touched 2026-07-09.*
  - **One-logical-line paragraphs** (no fixed-column hard-wrapping). *Pending:*
    ~230 hard-wrapped lines flagged by `bin/lint-md` — cosmetic (they reflow fine),
    low priority; do alongside the LaTeX pass.
  - **`bin/lint-md`** is the enforcement tool. Vivarium has no local copy; for now run ASF's: `~/src/archema-io/asf/bin/lint-md LEXICON.md`. (Known false-positive:
    its 4-space-indent "bare equation" heuristic misreads nested-list continuation.)

**Next concrete step when we return to this:** decide port-`bin/term`-vs-shared-store,
then either migrate `LEXICON.md` entries into `terminology/entries/` or finish the in-place format pass (§1–§6 LaTeX + de-hard-wrap). Not urgent.

## Native↔canonical representation machinery — deferred, tracked

Joseph (2026-07-10, the session that ran out of context; landed here 2026-07-10 by its successor): systems will often want **domain-specific internal representations** — faster and smaller than columns-on-the-grid — memoized natively and *projected* to the canonical `CellId` frame by lazy keyed recipes. Named examples: tilted-slab bodies computed as horizontal stacks (+ latent dip); the fluvial cells→receivers linked list (CHONK's graph); closed-form coefficient blocks. The interplay with current concepts is recorded in `LEXICON.md` §2 (*native representation / canonical frame*). **Figure out the general machinery when it's tactically intuitive** — i.e. when the first real second-representation system lands (most likely the drainage graph becoming a first-class store object in plan-Phase 2, or slab bodies) — not in advance.

## Session housekeeping

- **The 2026-07-09 batch is committed** (`9129ba5`, 2026-07-10): lexicon consolidation, PDF/relata work, and the architecture set (ARCHITECTURE v0.3,
  `DESIGN-REDUX.md` §12, `ref/research/{framework-to-status-quo,multiscale-seams,abyssal-parity-plan}.md`,
  `SUPERSEDED.md`, `TODO.md`). The `tmp.md` / `tmptmp.md` strays are gone.
- `DESIGN-REDUX.md` still uses "checkpoint" in four runtime-durability sites —
  decide *memo* vs a distinct persist-boundary name (LEXICON §5 collision ledger,
  `SUPERSEDED.md`). **Still open.**
