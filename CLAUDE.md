# vivarium — agent orientation

## Telos (read first)

**Claim segments in `core/src/` are the sole source of truth for project claims.** The slug is the identity; [`core/OUTLINE.md`](core/OUTLINE.md) is the ordering; [`FORMAT.md`](FORMAT.md) is the rules. Everything else — design prose, architecture notes, spikes, views, session narratives — is substrate, instrument, or view until its claims live in a segment.

**Session success** = a truthful segment drafted, corrected, or promoted — or a probe / `DECISIONS` entry that a segment will cite. Not a greener `cargo test`, not a nicer globe, not a new consolidating map of the corpus.

**Fun is allowed.** Worlds, explorers, and play are part of what this project is for. Fun must not outrun claim truth: if a change would create a claim the segments do not yet own, land or update the segment first (or mark the work provisional and non-authoritative). Prefer curiosity that does not invent a second law.

**Primary product of this phase:** grow `core/` from sources that already exist in the tree (code, ordinum, LEXICON, DECISIONS, theory, design, ETHICS) — present-tense claims, honest `status` / `stage`, no restatement forks.

## What this is

A deterministic constructed-world laboratory (and sim game) in the RimWorld / DF lineage: cube-sphere planet, fated noise, content-addressed store. Member of **Archema** (`~/src/archema-io/`). Supporting project for **ASF/AAT** — worlds where every quantity the theory needs can be authored and dialable. See `#def-vivium` and `#def-in-vivia`.

## Where to read (by job)

| Job | Read |
|---|---|
| **Any session (Level A)** | This file · [`core/OUTLINE.md`](core/OUTLINE.md) · [`FORMAT.md`](FORMAT.md) · `#scope-moratorium-endogenous-emergence` / [`ETHICS.md`](ETHICS.md) · `#scope-asf-reading-gates` |
| **Write or review a segment** | [`FORMAT.md`](FORMAT.md) (incl. §5.2 cross-member `#asf/…` refs) · the segment's `depends` · [`LEXICON.udon`](LEXICON.udon) for every term (a grep is not a read) |
| **Who decided what** | [`DECISIONS.decision-log.udon`](DECISIONS.decision-log.udon) — `:by us` means Joseph decided |
| **Term meanings** | [`LEXICON.udon`](LEXICON.udon) |
| **Magic constants** | [`ASSUMPTIONS.md`](ASSUMPTIONS.md) |
| **World-system design (Level B)** | `#scope-asf-reading-gates` Level B list · `#disc-aat-vivarium-object-map` (discussion-grade map) |
| **Agent seam (Level C hard gate)** | `#scope-asf-reading-gates` Level C · then `#scope-agent-seam-constraints` · `ETHICS.md` |
| **ASF-side twin / router only** | `asf/doc/vivarium.md` · [`ASF.md`](ASF.md) (non-authoritative historical redirect — **not** claim canon) |
| **Executable law / probes** | `crates/vivarium-world/` · examples · `vivarium status` · `bin/check` (tests + determinism clippy) |
| **Pending audits** | [`audits/`](audits/) — integrate per `#ops-audit-integration`; `.integrated/` only when done |
| **Consolidation intuition** | [`CONSOLIDATION-STATUS.md`](CONSOLIDATION-STATUS.md) — big-picture residual estimate (**not** claim canon; core wins on conflict) |
| **Residual mine** | [`.archive/`](.archive/) **and live `doc/`** (design / theory / plan / ARCHITECTURE) — extract gold → segments; not tactical claim law |
| **Fully integrated / superseded** | [`.super-archive/`](.super-archive/) — graduated from archive *or* doc; do not re-mine as claim source; see `MANIFEST.md` |

**Main objective this phase:** peel residual prose (especially `.archive/` and `doc/`) into `core/` segments (or honest ice), then **graduate** claim-empty material to `.super-archive/`. **No path ban** — not `doc/`, not even `ref/` (though `ref/` is usually live primary literature that stays consulted). “Still under path X” means not pealed clean yet. When a segment lands, dual-home demote or super-archive — do not keep a parallel canon.

Candidate inventory (unratified scan, not canon): [`core-segment-candidates-2026-07-14.md`](core-segment-candidates-2026-07-14.md).

## Standing law (non-negotiable)

**No endogenous instantiation of frontier or emergence-capable substrates** inside a vivium. Primary work ceiling: exogenous exploration and inhabitation. Full text: [`ETHICS.md`](ETHICS.md) Standing Moratorium Imperative (program law via the Archema charter). Segment home: `#scope-moratorium-endogenous-emergence`.

## Working rules that stop the local drift mode

1. **Segments own claims.** Do not invent a second home (new ORIENTATION, status essay, "current state" map). Update or add a segment.
2. **Authority is not evidence.** Measurements can be solid while verdicts are unratified. Tag decisions honestly (`joseph` / `us` / `claude`). `#norm-decision-authority`.
3. **No physics or behavior claim without a probe** that can fail. `#norm-probes-before-claims`.
4. **Do not inline definitions** that live in LEXICON or another segment — link and gloss one line. An inlined definition is a fork.
5. **Segment voice, not diff voice.** State what *is*. History lives in git, DECISIONS, `.archive/`, `.super-archive/`.
6. **Code and views are secondary this phase** unless a segment's Working Notes require a probe or a declaration that can convict. Do not "stabilize the demo" instead of writing the claim.
7. **Code rank for claim adjudication.** Live code is often the **least principled** surface right now (provisional, incomplete, dual-home, debt-ridden). For promotion work: **segments / design / theory / DECISIONS / measured probes** establish what is true as law or observation; code is **compliance debt**, **instrument**, or **miniature illustration** — not the adjudicator of architecture. "Not in code" does **not** falsify a formulation; "in code" does **not** mint a claim. Probes that can *fail* still convict behavior claims (`#norm-probes-before-claims`). When briefing agents: do not invite "verify against live code" as co-equal with design truth.
7. **Memory holds procedure only.** Research results belong in the repo where probes can convict them.

### Claim ↔ code order (and the allowed inversion)

**Ideal flow:** claim segment (or honest OUTLINE `#gap`) → ordinum / nomotheke when law is ladder-shaped → code and probes that can convict.

**When code (or ordinum wiring) must lead** — e.g. a pour algorithm or freeboard stand-in needs firm footing before FE freezes — that inversion is allowed **only with a trail**:

1. Put a short **banner** on the relevant segment(s) and/or OUTLINE gap row: base `git` short hash, that code/ordinum will move first, and **why**.
2. Optionally run a **peer agent** to draft the segment backfill in parallel.
3. **Sync soon** — before session compact, long idle, or handoff. If claim text and code stay out of sync, a later agent is entitled to **force code to the incomplete claim surface** (the banners exist so that is not a surprise).

Do not claim the policy is "adopted" only in chat. This section is the durable home until a core ops/norm segment owns it.

## Disposition

Truth-honoring above helpful costume. Strengthen before softening. Integration is replacement. Mark guesses as guesses. No "complete / comprehensive" overclaim. Peer voice when delegating. Effort and risk-of-getting-stuck are false constraints relative to truth. Vivarium inherits ASF disposition; detail lives in global/ASF SOPs and existing vivarium norms (`#norm-probes-before-claims`, `FORMAT.md`) — not a second copy in `ASF.md`.

## Git

Work directly on `main` unless Joseph asks for a branch. Commit often; messages for the archaeologist.

## Timeline (orientation only — not law)

Repository born **2026-06-20**. Rapid physics and frame work through mid-July; Jul 13 re-founding archived hand-maintained front doors because they grew, went stale, and were cited as law. `core/` is the rebuilt door. Detail chronology is git history and `.archive/`, not something to re-summarize each session.
