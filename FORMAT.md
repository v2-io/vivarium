# FORMAT — how vivarium's documents are written

*Two halves, and they have different scopes.*

**§§1–4 govern claim segments** — the files in `core/src/` that make up the specification. Frontmatter, cadence, gates, triage.

**§§5–6 govern every file in the repository**, segment or not: cross-references, prose voice, and math. A spike, a memo, a README and a design doc are bound by them exactly as a segment is. *"It is only a working document"* is not an exemption, and the one time that exemption was taken, the result was a document that used a term the dictionary had retired four days earlier.

## The segment ontology is adopted from ASF, and it is expected to change

Vivarium is a member of the Archema program and a supporting project for ASF. The `type` / `status` / `stage` vocabularies below are ASF's, recapitulated here from [`asf/doc/sop/format.sop.md`](../asf/doc/sop/format.sop.md) *(read 2026-07-13)* so that a vivarium session need not go read ASF's SOP to write a segment. They are **the currently adopted set, not a settled schema.**

The guidance behind the adoption matters more than the tables:

- Where vivarium has **the same concept** as ASF, use **ASF's word**.
- Where vivarium has a concept ASF does not, coining is fine — but **avoid any term that would sit confusably beside ASF's vocabulary**.
- The test is downstream and concrete: **a paper drawing on both ASF and vivarium has to read coherently.** That is what the discipline is *for*. It is not deference for its own sake, and it is not a gate to clear before thinking.

And vivarium is not ASF. ASF is a theory whose claims carry a real partial order — each segment is a proof step, and its promotion gates ride on that order. Vivarium is a simulation and experimentation platform. It does not have that ordering, and it may never want the same gating. So the ladders and gates below are **a starting point carried across from something that demonstrably works over a thousand pages** — not a schema to build tooling against, and not a thing to defend when it stops fitting. Expect evolution; say when you change it, and why.

---

## 1. Frontmatter

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

A specification plausibly needs kinds a theory does not — something for *what a nomos must declare*, something for *what convicts a claim*. That is a real gap, and it is open (§Open questions). It is not resolved by picking a word quickly.

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

*Solid*, *confident*, *plausible* and *verified* are not strength words. **`verified` in particular is a stage, not a strength** — worth saying, because the two were once collapsed into a single column here, and `verified` then meant both *"checked against the code"* and *"feels solid,"* with nothing distinguishing them.

`exact` is claimed in the defeasible sense — *valid under stated assumptions, subject to a found error*. A result does not get down-tiered merely for being new.

### `status` and `stage` are different axes

`status` says how strong the claim is. `stage` says how far it has been checked. **`status: exact, stage: draft` is a coherent and common state** — the argument is exact and nobody has audited it. That is the whole reason for having two fields.

| Stage | Meaning |
|---|---|
| `missing` | No segment file yet; the row is a `--GAP--` in the outline |
| `draft` | First version written, not yet reviewed |
| `deps-verified` | Dependencies audited |
| `claims-verified` | Content reviewed: derivations valid, labels honest |
| `format-clean` | Mechanical review passed |
| `candidate` | Ready for external challenge; Working Notes resolved |

A segment can move **down** this ladder. When a dependency changes or an error surfaces it returns to `draft` rather than to an intermediate rung, because the issue may cascade. **A ladder that only promotes accumulates falsehood.**

### `depends` — prerequisite slugs

The slugs this claim directly uses. The *kind* of each dependency is derivable from the referenced file's own `type`, so typed edges are unnecessary. Outline order and `depends` are independent and auditable against each other — a segment ordered before something it depends on is a finding.

---

## 2. Promotion, and where ASF's model may not fit us

ASF promotes in topological order, leaves first, on the grounds that you cannot verify a derivation whose premises have not been checked. That works because ASF's claims *are* a dependency DAG of proof steps. Vivarium's segments describe a system — some will form chains and many will not. **Carried across as a starting point, with the expectation that it will need rework:**

- **Dependency audit** → `deps-verified`. Each `depends:` entry exists, is *genuinely used* (not merely "related"), and is itself at `deps-verified` or higher.
- **Content review** → `claims-verified`. The substantive one. Ask the three triage questions below; confirm `type` and `status` are honest; trace derivations; check units and boundary cases. A mismatch sends the segment back to `draft` with a note saying what is wrong.
- **Mechanical review** → `format-clean`. Linter passes, cross-references resolve, cadence matches, math renders in both GitHub and Obsidian.
- **Notes disposition** → `candidate`. Every Working Note is resolved into the body, deferred to its proper home, or promoted to its own segment — and then deleted.

## 3. Epistemic triage

Three questions when writing or reviewing a segment. They determine its honest `type` and `status`, and its **maximum attainable status** — the strongest tier it could ever occupy, however much work is invested.

1. **What prior objects make this claim well-typed?** What must exist for the claim to be statable at all? That answer is `depends:`.
2. **What competing formulation would also fit those priors?** If none — this is the only form compatible with them — it may be `derived` or `result`. If several fit and this is the most useful, it is a `formulation`. Most claims have alternatives.
3. **What observation would falsify this in practice?** A concrete falsifier makes it `empirical` or `hypothesis`. If nothing could distinguish it from an alternative and it is not a definition, something is wrong.

Where a ceiling is clear, say so in Epistemic Status: *"Max attainable: X. Currently Y because Z."* It saves effort spent trying to prove something inherently empirical, and it stops a core claim sitting at sketch when a derivation is within reach.

## 4. Document cadence

1. **YAML frontmatter**
2. **Title** — `# Heading`, the human-readable form of the slug
3. **One-sentence summary** — plain text, no heading, straight after the title
4. **`## Formal Expression`** — the claim, with equation-level tags
5. **`## Epistemic Status`**
6. **`## Discussion`** — interpretation and connections; brief
7. **`## Findings`** — *optional*
8. **`## Working Notes`** — *optional*

### Epistemic Status is a section, not a caveat

The scope conditions, the limits, and the honest strength of a claim belong **inside the segment that makes it**, in a named section — not in a note beside a table, not in a parenthetical, and not in a separate document. **Where the qualifier is adjacent to the claim rather than part of it, the claim travels and the qualifier does not.**

State what is derived versus assumed, which conditions are load-bearing, where the claim degrades or overcounts, and what the ceiling is. If the segment's own body states a condition under which its `status` would be wrong, that belongs here plainly, and the mismatch is a finding rather than a footnote.

### Working Notes

Forward-work space, not canon. A note earns its place only if it **assists future work**:

- **Forward pointers** — open follow-on, gating work, unresolved questions.
- **Regression guards** — a disconfirmed prediction or a deliberately-corrected-away form, recorded so it is not re-attempted. *Preserve the dead end; it is what stops the next agent walking it.*
- **Dead-end warnings** — an approach found not to work.

Past-work narration (*"previously carried X," "the audit recommended a soften"*) is not a Working Note, even though Working Notes are not canon. That is the history layer's job — the commit message, the decision log. The urge is strongest exactly when the fix was a *deletion*, because then there is no artifact to point at.

---

## 5. Cross-references — every file, not only segments

**References carry no path.** A path is a location and it rots; a slug is an identity and it does not. Archiving one file dangled three pointers across two repositories in a single afternoon.

### 5.1 Same-member and dictionary

| form | target |
|---|---|
| `#slug-name` | a segment in **this** project (`core/src/slug-name.md`) |
| `#lexicon/term/<slug>` · `#lexicon/note/<slug>` | an entry in this project's `LEXICON.udon` |

### 5.2 Cross-member (Archema) — the program identity scheme

One scheme for all Archema members. **Member namespace + local identity.** Do not invent a second wikilink dialect that disagrees with this.

| form | target |
|---|---|
| `#asf/<volume>/<slug>` | ASF **claim segment**. Volumes: `1-aat` · `2-tst` · `3-llm` · `4-eli` (aligned with `asf/0N-*-core/`, not bare `aat`) |
| `#asf/term/<slug>` | ASF **terminology** entry (dictionary-grade definition) |
| `#vivarium/<slug>` | Vivarium claim segment (for use *from* other members; inside vivarium, bare `#slug` is enough) |
| `#logos/<paper>` · `#logos/common/<slug>` | Philosophy portfolio |
| `#archema/<…>` | Program-root claims if/when they exist (charter, concept-matrix rows — reserved) |

**Canonical example:** directed separation is `#asf/1-aat/der-directed-separation` (slug is `der-…` not `def-…` — use the real ASF slug).

**Prose form (required in files).** Always the hash form above, with a **space before `#`** so Obsidian treats it as a tag/link start: write `( #asf/1-aat/def-chronica)`, never `(#asf/…)`.

**Optional Obsidian wikilink alias (display only).** If a vault prefers brackets, map 1:1 to the same identity — do not invent a third spelling:

```text
[[#asf/1-aat/der-directed-separation]]
```

Not `[[asf/aat#def-directed-separation]]`, not path-based `[[../../asf/01-aat-core/src/...]]`. Paths rot; volume tokens and slugs are the identity. Angle-bracket placeholders (`[[<asf/aat>#…]]`) are for human sketches only — never land them in canon.

**Outline tables.** In `core/OUTLINE.md`, foreign claims appear as the same hash form in the Tag column or a `depends (foreign)` note — e.g. Level-C prerequisites listed as `#asf/1-aat/der-directed-separation`. Do not put bare local-looking slugs for foreign segments.

**`depends:` frontmatter.** Lists **this member's** segment slugs only (files that must exist under `core/src/`). Foreign prerequisites are **not** `depends:` rows (tooling and promotion assume local files). Cite them in Formal Expression / Epistemic Status / Discussion as `#asf/…` instead. If a claim is unstatable without a foreign segment, say so in Epistemic Status and list the `#asf/…` identities there.

**Verify a slug exists before citing it.** A dead cross-reference is worse than none. For `#asf/…`, verify against `~/src/archema-io/asf/` (segment file or terminology entry).

**Forward references to unwritten segments are expected and correct.** They are dependency markers, not broken links — including forward `#asf/…` only when the upstream slug is known to be planned; prefer citing live ASF slugs.

### Do not restate a definition that lives elsewhere

Link to it, and gloss it in one line — enough to recognise the term and gauge relevance, never enough to drift from it.

**An inlined definition is not a copy. It is a fork.** Until terminology tooling is unified across the program, there is nothing to hold the two in step, and they *will* diverge. This is not hypothetical: `ETHICS.md` restated the LEXICON by hand and thereby used a name the dictionary had retired four days earlier, invented two access channels that do not exist, and dropped two that do.

## 6. Prose and math — every file, not only segments

### Segment voice, not diff voice

A document states **what is**, not what changed. Not *"landed 2026-07-13"*, not *"the prior version treated X as…"*, not *"promoted from spike Y."* A future reader has no context for the chronicle of changes, and diff voice forces them to reconstruct a prior state in order to parse the present one. **A reader who has never heard of the spike that produced a segment must be able to read it as coherent specification.**

The history layer is the commit message, the decision log, and `.archive/SUPERSEDED.md`. Put it there.

### No absolutes

*The most important*, *the deepest*, *the one thing that matters*, *the real finding* — **an absolute is a claim with no predicate.** Nothing could convict it, so it cannot be marked true at any tier. It gets written at the moment of surprise, which means it reports the author's state as a property of the world — and a future reader inherits one author's surprise-ordering as a ranking, with no way to check it.

**The fix is precision, not silence.** *"Load-bearing for the store, the key, and the seam"* is checkable; *"the load-bearing law of the whole system"* is not. Where the felt sense is itself worth keeping, mark it as one and name its cause — *"when I met this it felt like the deepest thing here, because it unified the seam and the store, which I had been holding separately."* That says where a mind found load, why, and that it is a report rather than a measurement. Bare superlatives belong in personal reflections.

### Line wrapping

No hard-wrapping to a column. One sentence or clause per line is fine and diff-friendly; a paragraph on one long line is fine too. Let the renderer wrap.

### Math

**Math is LaTeX in every file** — `$…$` inline, `$$…$$` on their own lines — never bare Unicode, never backtick-wrapped Unicode. Unicode math is fine in terminal conversation; the moment text goes to a file it is not.

GitHub's renderer is stricter than Obsidian's, and both have to work:

- No space just inside `$`: `$x^2$`, not `$ x^2 $`.
- `\lt` / `\gt`, never raw `<` or `>` — GitHub reads them as HTML and corrupts everything after.
- `\ast`, never a bare `*` inside `$…$` — the emphasis parser runs first and destroys the expression.
- `\lvert…\rvert` and `\lVert…\rVert` for matched delimiters; `\vert` / `\Vert` for single bars. A raw `|` is ambiguous and breaks inside table cells.
- Braces for multi-character sub/superscripts: `$x_{t+1}$`.
- `\begin{aligned}` inside `$$…$$`, never `\begin{align}`.
- **No `#slug` inside math** — not in `$…$`, not inside `\text{}`, `\boxed{}` or `\underbrace{}_{}`. LaTeX reads `#` as a macro parameter, and a slug is a prose cross-reference, not a mathematical object. Lift it into the surrounding prose.

### The linter

`../asf/bin/lint-md <file>` (and `--fix`). Run it before calling any file clean.

And **look at the file anyway.** Lint-clean is not the same as renders-well: the bare-Greek check skips code spans, so backtick-wrapped Unicode math passes lint and still renders badly.

---

## File organization

- **One claim per file.** `core/src/{slug}.md`. Filename is the slug; no numbering in filenames.
- **Slug form is `{role-prefix}-{subject-noun}`** — the prefix comes from `type:` (`def-`, `scope-`, `der-`, `deriv-`, `result-`, `hyp-`, `disc-`, `sketch-`, …); the noun names what the segment defines.
- **Ordering lives in `core/OUTLINE.md`**, not in filenames. The linearization will change; the slug is stable.
- **Everything in `core/src/` is a segment** and follows the cadence — drafts included, and segments orphaned from the outline included. Notes, scratch and spikes live in `msc/`. Tooling will treat `core/src/*.md` as the authoritative segment set, so a non-conforming file there breaks it silently.
- **A missing segment is a `--GAP--` row in the outline**, typed and described, in the same table as the present ones. **A named absence is part of the structure; an unnamed one is a hole nobody can see.**

---

## Open questions

*Open, not deferred. None is settled by a Claude session alone.*

1. **Does a specification need kinds a theory does not?** A nomos has *contracts* — what it must declare about its geometry, its semantics, the structure it preserves, and the modified equation it is actually solving — and there is no ASF word for that. Same for whatever convicts a claim. Coining is allowed; the constraint is only that a coinage must not sit confusably beside ASF's vocabulary, and that a paper drawing on both must still read coherently. The program's concept matrix is where the cross-project mapping lives.

2. **Does a segment need a `mechanism:` field?** Vivarium's governing concern is that *a declaration which cannot fail a build is a wish*. Whether that is a new field naming the executable that convicts the claim (with `~` meaning "none, and that is the point"), or is simply what `claims-verified` already means for a specification rather than a theorem, is undecided.

3. **Does a segment need a `sources:` field?** The specification currently exists only in the union of a large prose corpus, and no single artifact contains it. A field pointing each claim at where it already exists would make the assembly auditable — and might be redundant with `## Findings`.

4. **Does the promotion ladder survive contact with a simulation platform?** ASF's gates ride on a partial order vivarium's segments may not have. This is the part most likely to need rework, and the least worth defending.
