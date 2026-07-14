# core — the vivarium specification

The specification of what vivarium is and must do, written as **claim segments**: one claim per file in `core/src/`, each stating one thing that is defined, follows, or is restricted, given what came before. This file carries the ordering and the rules; the slug carries the identity. **No segments are written yet.** What follows is the contract a segment satisfies, and nothing else.

## The segment ontology is adopted from ASF, and it is expected to change

Vivarium is a member of the Archema program and a supporting project for ASF (`../asf/`). The `type` / `status` / `stage` vocabularies below are ASF's, recapitulated here from [`../asf/doc/sop/format.sop.md`](../../asf/doc/sop/format.sop.md) so that a vivarium session does not have to go read ASF's SOP to write a segment. They are **the currently adopted set, not a settled schema.**

The guidance behind the adoption, which matters more than the tables:

- Where vivarium has **the same concept** as ASF, use **ASF's word**.
- Where vivarium has a concept ASF does not, coining is fine — but **avoid any term that would be easily confused sitting next to ASF's vocabulary**.
- The test is downstream and concrete: **a paper that combines ASF and vivarium concepts or results has to read coherently.** That is what the discipline is for. It is not deference for its own sake, and it is not a gate to be cleared before thinking.

And vivarium is not ASF. ASF is a theory whose claims carry a real partial order — each segment is a proof step, and its promotion gates ride on that order. Vivarium is a simulation and experimentation platform. It does not have that ordering, and it may never want the same gating. So the ladders and gates below are **a starting point carried across from something that demonstrably works over a thousand pages** — not a schema to build tooling against, and not a thing to defend when it stops fitting. Expect evolution; say when you change it, and why. (ASF's own SOP is somewhat stale on its side too.)

---

## Frontmatter

```yaml
---
slug: the-slug-name
type: definition
status: exact
stage: draft
depends:
  - prerequisite-slug-1
  - prerequisite-slug-2
---
```

### `type` — what kind of claim

| Type | Meaning |
|---|---|
| `postulate` | Foundational or tautological — cannot be derived, only accepted |
| `definition` | Introduces a quantity, object, or notation |
| `scope` | Restricts or broadens the domain under discussion |
| `formulation` | A representational or modeling choice (it could have been different) |
| `derived` | Logical consequence of prior claims under stated assumptions |
| `result` | Formally stated, with a detailed derivation |
| `corollary` | Follows directly from a result |
| `hypothesis` | Structurally motivated, needs validation |
| `normative` | Grounded in prior claims but requiring a precondition that must be verified |
| `empirical` | Generalization supported by data or measurement, not fully derived |
| `observation` | Finding from a probe or measurement |
| `discussion` | Conceptual claim used for interpretation |
| `measurement` | Operationalization of a quantity |
| `proposed-schema` | Shape identified, formal content pending |
| `derivation` | Complete formal derivation backing a result or derived claim |
| `worked-example` | End-to-end instantiation validating a chain |
| `detail` | Extended operational or technical material supporting other claims |
| `sketch` | Direction identified, rigor pending |
| `aside` | Tangential; informative but not load-bearing |

ASF chose `postulate` (not *axiom*), `result` (not *theorem*), and `derivation` (not *proof*) to avoid claiming a foundational mathematical originality it does not have. Vivarium inherits the restraint for the same reason. External theorems keep their original names.

A specification plausibly needs kinds a theory does not — something for *what a nomos must declare*, something for *what convicts a claim*. That is a real gap and it is open (§Open questions). It is not resolved by picking a word quickly.

### `status` — epistemic strength

| Status | Meaning |
|---|---|
| `axiomatic` | Foundational or tautological |
| `exact` | Validated under stated assumptions |
| `robust-qualitative` | Survives across assumptions; the specific form is approximate |
| `heuristic` | Useful approximation; the quantitative form may not hold |
| `conditional` | Depends on explicitly named local assumptions |
| `empirical` | Supported by measurement or simulation, not fully derived |
| `discussion-grade` | Argued qualitatively or by analogy, not derived |
| `sketch` | Direction identified, formalization incomplete |

*Solid*, *confident*, *plausible* and *verified* are not strength words. `verified` in particular is a **stage**, not a strength — which is worth saying because the two were collapsed into one column here before, and `verified` then meant both "checked against the code" and "feels solid," with nothing distinguishing them.

`exact` is claimed in the defeasible sense — *valid under stated assumptions, subject to a found error*. A result does not get down-tiered merely for being new.

### `status` and `stage` are different axes

`status` says how strong the claim is. `stage` says how far it has been checked. `status: exact, stage: draft` is a coherent and common state — the argument is exact and nobody has audited it. Keeping both is the point of having two fields.

| Stage | Meaning |
|---|---|
| `missing` | No segment file yet; the row is a `--GAP--` in this outline |
| `draft` | First version written, not yet reviewed |
| `deps-verified` | Dependencies audited |
| `claims-verified` | Content reviewed: derivations valid, labels honest |
| `format-clean` | Mechanical review passed |
| `candidate` | Ready for external challenge; Working Notes resolved |

A segment can move **down** this ladder. When a dependency changes or an error surfaces, it goes back to `draft` rather than to an intermediate rung, because the issue may cascade. A ladder that only promotes accumulates falsehood.

### `depends` — prerequisite slugs

The slugs this claim directly uses. The *kind* of each dependency is derivable from the referenced file's own `type`, so typed edges are unnecessary. Outline order and `depends` are independent, and auditable against each other — a segment ordered before something it depends on is a finding.

### Promotion, and where ASF's model may not fit us

ASF promotes in topological order, leaves first, on the grounds that you cannot verify a derivation whose premises have not been checked. That works because ASF's claims *are* a dependency DAG of proof steps. Vivarium's segments describe a system — some of them will form chains, and many will not. **Carried across as a starting point, with the expectation that it will need rework:**

- **Dependency audit** → `deps-verified`. Each `depends:` entry exists, is *genuinely used* (not merely "related"), and is itself at `deps-verified` or higher.
- **Content review** → `claims-verified`. The substantive one. Ask the three triage questions below; confirm `type` and `status` are honest; trace derivations; check units and boundary cases. A mismatch sends the segment back to `draft` with a note saying what is wrong.
- **Mechanical review** → `format-clean`. Linter passes, cross-references resolve, cadence matches, math renders in both GitHub and Obsidian.
- **Notes disposition** → `candidate`. Every Working Note is resolved into the body, deferred to its proper home, or promoted to its own segment — and then deleted.

## Epistemic triage

Three questions when writing or reviewing a segment. They determine its honest `type` and `status`, and its **maximum attainable status** — the strongest tier it could ever occupy, however much work is invested.

1. **What prior objects make this claim well-typed?** What must exist for the claim to be statable at all? That answer is `depends:`.
2. **What competing formulation would also fit those priors?** If none — this is the only form compatible with them — it may be `derived` or `result`. If several fit and this is the most useful, it is a `formulation`. Most claims have alternatives.
3. **What observation would falsify this in practice?** A concrete falsifier makes it `empirical` or `hypothesis`. If nothing could distinguish it from an alternative and it is not a definition, something is wrong.

Where a ceiling is clear, say so in Epistemic Status: *"Max attainable: X. Currently Y because Z."* It saves effort spent trying to prove something inherently empirical, and it stops a core claim sitting at sketch when a derivation is in reach.

---

## Document cadence

1. **YAML frontmatter**
2. **Title** — `# Heading`, the human-readable form of the slug
3. **One-sentence summary** — plain text, no heading, straight after the title
4. **`## Formal Expression`** — the claim, with equation-level tags
5. **`## Epistemic Status`**
6. **`## Discussion`** — interpretation and connections; brief
7. **`## Findings`** — *optional*
8. **`## Working Notes`** — *optional*

### Epistemic Status is a section, not a caveat

The scope conditions, the limits, and the honest strength of a claim belong **inside the segment that makes it**, in a named section — not in a note beside a table, not in a parenthetical, and not in a separate document. Where the qualifier is adjacent to the claim rather than part of it, the claim travels and the qualifier does not.

State what is derived versus assumed, which conditions are load-bearing, where the claim degrades or overcounts, and what the ceiling is. If the segment's own body states a condition under which its `status` would be wrong, that belongs here, plainly, and the mismatch is a finding rather than a footnote.

### Working Notes

Forward-work space, not canon. A note earns its place only if it **assists future work**:

- **Forward pointers** — open follow-on, gating work, unresolved questions.
- **Regression guards** — a disconfirmed prediction or a deliberately-corrected-away form, recorded so it is not re-attempted. Preserve the dead end; it is what stops the next agent walking it.
- **Dead-end warnings** — an approach found not to work.

Past-work narration (*"previously carried X," "the audit recommended a soften"*) is not a Working Note, even though Working Notes are not canon. That is the history layer's job — the commit message, the decision log. The urge is strongest exactly when the fix was a deletion, because then there is no artifact to point at.

### Segment voice, not diff voice

Formal Expression, Epistemic Status, and Discussion state **what the specification is**, not what changed. Not *"landed 2026-07-13"*, not *"the prior version treated X as…"*, not *"promoted from spike Y."* A future reader has no context for the chronicle of changes, and diff voice forces them to reconstruct a prior state in order to parse the present one. A reader who has never heard of the spike that produced a segment should be able to read it as coherent specification.

### No absolutes

*The most important*, *the deepest*, *the one thing that matters*, *the real finding* — an absolute is a claim with no predicate. Nothing could convict it, so it cannot be marked true at any tier. It gets written at the moment of surprise, which means it reports the author's state as a property of the world, and a future reader inherits one author's surprise-ordering as a ranking with no way to check it.

The fix is precision, not silence. *"Load-bearing for the store, the key, and the seam"* is checkable; *"the load-bearing law of the whole system"* is not. Where the felt sense is itself worth keeping, mark it as one and name its cause — *"when I met this it felt like the deepest thing here, because it unified the seam and the store, which I had been holding separately."* That says where a mind found load, why, and that it is a report rather than a measurement. Bare superlatives belong in personal reflections.

---

## File organization and format

- **One claim per file.** `core/src/{slug}.md`. Filename is the slug; no numbering in filenames.
- **Slug form is `{role-prefix}-{subject-noun}`** — prefix from `type:` (`def-`, `scope-`, `der-`, `deriv-`, `result-`, `hyp-`, `disc-`, `sketch-`, …); the noun names what the segment defines.
- **Ordering lives here**, not in filenames. The linearization will change; the slug is stable.
- **Everything in `core/src/` is a segment** and follows the cadence — drafts included, and segments orphaned from this outline included. Notes, scratch and spikes live in `msc/`; tooling will treat `core/src/*.md` as the authoritative segment set, so a non-conforming file there breaks it silently.
- **Cross-references** are `#slug-name` in prose, `[#slug-name](slug-name.md)` between segments, `[#slug-name](src/slug-name.md)` from this file. Put a space before the `#` — Obsidian only recognizes it as a link when preceded by whitespace. **Forward references to unwritten segments are expected and correct**; they are dependency markers, not broken links.
- **Cross-references carry no path.** A path is a location and it rots; a slug is an identity and it does not — archiving one file dangled three pointers across two repos in a single afternoon. Five forms, all Obsidian-nestable and greppable across the program:

| form | target |
|---|---|
| `#slug-name` | a segment in this project |
| `#lexicon/term/<slug>` · `#lexicon/note/<slug>` | an entry in `LEXICON.udon` |
| `#asf/term/<slug>` | an ASF **terminology entry** — its dictionary-grade definition |
| `#asf/<volume>/<slug>` | an ASF **claim segment**; volumes shorten to `1-aat` · `2-tst` · `3-llm` · `4-eli` |
| `#logos/<paper>` · `#logos/common/<slug>` | the philosophy portfolio |

  **Put a space before every `#`** — Obsidian only registers a link when the `#` is preceded by whitespace, so write `( #asf/1-aat/def-chronica)`, never `(#asf/…)`. **Verify a slug exists before citing it**; a dead cross-reference is worse than none.

- **Do not restate a definition that lives elsewhere.** Link to it and gloss it in one line — enough to recognise the term and gauge relevance, never enough to drift from it. Until terminology tooling is unified across the program, **an inlined definition is a fork waiting to happen** — and it already happened once, in `ETHICS.md`, which restated the LEXICON by hand and thereby used a name the dictionary had retired four days earlier.
- **A missing segment is a `--GAP--` row in this outline**, typed and described, in the same table as the present ones. A named absence is part of the structure; an unnamed one is a hole nobody can see.
- **Line wrapping.** No hard-wrapping to a column. One sentence or clause per line is fine and diff-friendly.
- **Math is LaTeX in every file** — `$…$` inline, `$$…$$` on their own lines — never bare Unicode, never backtick-wrapped Unicode. Unicode math is fine in terminal conversation; the moment text goes to a file it is not.

**Math compatibility.** GitHub's renderer is stricter than Obsidian's and both have to work:

- No space just inside `$`: `$x^2$`, not `$ x^2 $`.
- `\lt` / `\gt`, never raw `<` or `>` — GitHub reads them as HTML and corrupts everything after.
- `\ast`, never a bare `*` inside `$…$` — the emphasis parser runs first and destroys the expression.
- `\lvert…\rvert` and `\lVert…\rVert` for matched delimiters; `\vert` / `\Vert` for single bars. A raw `|` is ambiguous and breaks inside table cells.
- Braces for multi-character sub/superscripts: `$x_{t+1}$`.
- `\begin{aligned}` inside `$$…$$`, never `\begin{align}`.
- No `#slug` inside math — not in `$…$`, not inside `\text{}`, `\boxed{}` or `\underbrace{}_{}`. LaTeX reads `#` as a macro parameter, and a slug is a prose cross-reference, not a mathematical object. Lift it into the surrounding prose.

The linter is ASF's: `../asf/bin/lint-md <file>` (and `--fix`). Run it before calling a file clean — and look at the file anyway, because lint-clean is not the same as renders-well (its bare-Greek check skips code spans, so backtick-wrapped Unicode math passes and still renders badly).

---

## Open questions

*Open, not deferred. None is settled by a Claude session alone.*

1. **Does a specification need kinds a theory does not?** A nomos has *contracts* — what it must declare about its geometry, its semantics, the structure it preserves, and the modified equation it is actually solving — and there is no ASF word for that. Same for whatever convicts a claim. Coining here is allowed; the constraint is only that a coinage must not sit confusably beside ASF's vocabulary, and that a paper drawing on both must still read coherently. `../charter/concept-matrix.md` is where the cross-project mapping lives.

2. **Does a segment need a `mechanism:` field?** Vivarium's governing concern is that *a declaration which cannot fail a build is a wish*. Whether that is a new field naming the executable that convicts the claim (with `~` meaning "none, and that is the point"), or is simply what `claims-verified` already means for a specification rather than a theorem, is undecided.

3. **Does a segment need a `sources:` field?** The specification currently exists only in the union of a large prose corpus, and no single artifact contains it. A field pointing each claim at where it already exists would make the assembly auditable — and might be redundant with `## Findings`.

4. **Does the promotion ladder survive contact with a simulation platform?** ASF's gates ride on a partial order vivarium's segments may not have. This is the part most likely to need rework, and the least worth defending.

5. **Where does the standing moratorium live now?** It was `ASF.md` §0 and it is program law. `ASF.md` is archived; `ETHICS.md` remains; the project memory `moratorium-endogenous-emergence` is currently its primary live carrier. It needs an explicit, unskippable home before any agent-seam work resumes.

## Status

Segments written: **0**. Segments named: **none yet**. The corpus this specification will be assembled from is in `doc/`, `ref/`, `LEXICON.udon`, `DECISIONS.decision-log.udon` and `tabularium/`. The front doors that used to summarize it are in `.archive/` — deliberately, so the assembly is made from the sources rather than from a prior summary of them.
