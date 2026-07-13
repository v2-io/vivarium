# Confusion log — a fresh agent onboarding, 2026-07-13

*Written by the agent as it happened, not reconstructed. The instruction was: log every
moment I had to guess, backtrack, or read twice. This is the honest measurement of the
onboarding cost; the agent who wrote the docs cannot produce it, because it knows the
answers.*

**Method.** Followed `CLAUDE.md`'s reading gates exactly — Level A (CLAUDE.md,
ORIENTATION.md, ASF.md), then Level B (doc/ARCHITECTURE.md, LEXICON.udon,
doc/theory/discretisation-and-information.md), then DECISIONS/ASSUMPTIONS/TODO/
SUPERSEDED, then project memory. No shortcuts.

---

## The confusions, in the order I hit them

**C1 — CLAUDE.md's own doc-map does not list the doc it declares required.** (line 11 vs
line 37.) Line 11 maps `theory/` as *"(multiscale-methods, multiscale-seams)"*. Twenty-six
lines later, line 37: *"⇒ `doc/theory/discretisation-and-information.md` — **required**
before authoring or reworking any field nomos."* The map and the gate disagree in the same
file. I read the map first and formed a picture of `theory/` that had to be revised.

**C2 — ORIENTATION.md never mentions the required doc at all.** Item 8 of *"Where the
thinking lives (read in this order)"* — the fresh agent's actual reading list — lists
`multiscale-seams.md` and `multiscale-methods.md` and stops. **A fresh session following
ORIENTATION would never read `discretisation-and-information.md`.** The only reason I read
it is that the task brief named it. *This is the highest-cost defect I found in the
onboarding path: the new principle-carrying document is not reachable from the front door.*

**C3 — `tmptmp.md`, 7 KB, in the repo root.** Not in any doc map, not in `.gitignore`, not
in `.archive/`. It is a raw dialogue excerpt, and it turns out to be the *substrate* of the
theory doc's §4.2 curvature derivation. I could not tell whether it was canon, scratch, or
something I was supposed to act on. I read it to find out — which is exactly the cost the
doc tree is supposed to eliminate.

**C4 — three names for the load-bearing noun.** ORIENTATION says **Manifest** (`spec.rs`).
The code file is `spec.rs`. `LEXICON.udon` has **no `manifest` entry** — while two other
entries carry `|rel :to manifest` edges pointing at it. I had to read `spec.rs`'s doc
comment to learn what a manifest is. Level B says *"`LEXICON.udon` for every term you
touch"*; for this term the LEXICON is silent.

**C5 — the LEXICON's hub node is a ghost, and the LEXICON does not say so.** I looked up
`vivium` — the project's most central noun. Its entry carries `|rel :to regula :kind
declares-conformance-to` and `|rel :to manifest :kind individuated-by`. So I followed
`regula` and read a 20-line `:status settled` entry: etymology, a supremacy clause, a
nesting rule, a lineage discipline. I believed it. **It is retired** — `regula.rs` is not
built and never will be (`DECISIONS[regula-collapses-to-order-and-manifest]`, Joseph).
Nothing in `LEXICON.udon` warns me. The warning exists in `SUPERSEDED.md` and in the
decision log — *the two files Level B did not send me to.* I onboarded onto a dead concept
and only caught it later, from the decision log, by luck of reading order. **13 `|rel`
edges in the LEXICON point at `regula`.** Full assessment in `findings.md` §P3.

**C6 — I could not learn what a column is. The docs told me three different things, and
two of them claim to be settled.** This is the big one, and it stopped my design sketch at
step one.

| source | says | status it claims |
|---|---|---|
| `doc/design/DESIGN-MATERIAL.md` §4, §11 | *"Surface elevation is a derived reading: **top-of-topmost-solid, sampled at the column's center** — a finite-difference node. (Settled by code…)"*, and §11 lists it under **Settled**: *"surface elevation = **center point-sample** (renderer-committed)"* | **settled** |
| `DECISIONS[column-is-a-control-volume-with-sufficient-statistics]` | *"A column is a control volume… **It is NOT a point**… **Nobody has ever said which it is**"* | **`:by us :status decided`** — ratified (Joseph: *"it very much lands"*) |
| `TODO.md` §THE FORK + theory doc §6.5 | *"**Is a stored scalar a POINT SAMPLE, a CELL AVERAGE, or a BAND-LIMITED sample?** … the code answers three ways… **every mesh, seam, and conservation question is unanswerable until this is decided**"* | **open fork** |

I read DESIGN-MATERIAL first (it is Level B, ORIENTATION item 4). It gave me a clean
answer, marked settled. I then read the theory doc, which told me nobody had ever answered
it. I backtracked and re-read DESIGN-MATERIAL twice to check I had not misread. I had not.
**"Nobody has ever said which it is" is false**, and it is asserted in the newest doc and
in a ratified decision.

**C7 — global block, or per-quantity declaration?** Same fork, opposite instructions:

- `TODO.md`: *"**Every mesh, seam, and conservation question is unanswerable until this is
  decided.**"* → reads as a hard block on my nomos.
- `DECISIONS[column-is-a-control-volume]` `|impact` (a): *"the column **DECLARES its
  semantics per quantity**"* → reads as: just declare it and proceed.

**I had to guess.** I proceeded on the second reading (a nomos declares its own quantities'
semantics) because it is the only one that lets any work happen — but I do not know that I
was allowed to.

**C8 — the theory doc tells me to declare things that have no place to be declared.** It
requires a nomos to declare: which structure it preserves, what it sacrifices, whether its
residual is bias or noise, its column semantics per quantity, its reconstruction per
consumer. `DECISIONS[algorithms-are-disguised-physical-claims]` §impact says the geometric
contract *"belongs beside `consumes`/`produces`"* — i.e. in `nomotheke.rs`. **`NomosDecl`
has no such field**, and nothing tells me whether I may add one. This is where the doc
stops being executable.

**C9 — `doc/ARCHITECTURE.md` is Level B and was not swept.** It still states mean-pin as
*"the fidelity invariant… $R \circ L = \mathrm{id}$ **on the mean**, live"* with no flag
that Jensen makes the mean insufficient, and no flag that column semantics is an open fork.
It does not link the theory doc. **`column-is-a-control-volume` cites `ARCHITECTURE.md` (R∘L
= id "on the mean") in its own `|ref` as the *evidence of the error* — and then nobody
annotated ARCHITECTURE.** So the Level-B doc that routes nomos authors is the one document
the correction did not reach. I formed my mental model from it before the theory doc
corrected me.

**C10 — the ledger did not track its own live finding.** Theory doc §2.5 says of
`ASSUMPTIONS.md`'s de Almeida–Bates row: *"`ASSUMPTIONS.md` lists it as 'literature — used
as intended,' which… does not say the true thing: **its physical claim is NONE**."* TODO
says *"`ASSUMPTIONS.md` should say so."* **`ASSUMPTIONS.md` row 42 still says "used as
intended."** Project law is *touch a constant, touch the ledger*; a finding about one of the
ledger's own rows did not reach the ledger.

**C11 — the decision log's supersession chain does not grep in one read, contrary to its own
schema.** The header rule: *"⚠ WHEN USED, `:supersedes` GOES ON THE `|decision[...]` LINE
ITSELF — never on its own line — so that `grep '^|decision\['` yields the whole supersession
chain in one read."* `grid-report-supersedes-two-of-my-claims` puts `:supersedes
seam-amortization-and-the-two-grid-overlay` **on its own line**. I grepped, believed the
result, and concluded `seam-amortization` was superseded by nothing — which was wrong, and
which mattered, because that entry carries a grid verdict in Joseph's name.

---

## What did NOT confuse me — and deserves saying

The **authority discipline landed, completely, before I did any work.** CLAUDE.md's
"evidence is not authority" paragraph → the decision log's authority legend → the
`|judgment` block → `memory/authority-not-evidence.md` are four passes at the same lesson
in escalating depth, and by the end of the second I knew, without being told again: I may
not decide the grid, I may not decide the column semantics, I may not tag anything `:by
us`, and I may not write a verdict into ORIENTATION. **That is the onboarding working
exactly as Joseph wants it to.** It is also the one thing in the repo that is written in
*texture*, not rules — the "it will feel like conscientiousness" line is what makes it
stick, and it is why I went looking for inflated tags rather than assuming they had all
been caught.

The **standing guards fired unprompted.** Before designing anything I went to
`tabularium/terrestris.ordinum.udon` to check which phase licenses my phenomenon — because
*check the ladder, not modern Earth* had been read twice and had landed. It changed the
design (see the sketch: `promise[coriolis]` exists, has no `:kept-by`, and my nomos is its
keeper). **That is inculcation, and it is the thing under test.**

---

## The honest bottom line on cost

Level A cost minutes, as advertised. Level B cost roughly **four times** what it should
have, and every unit of the overrun was one of: a stale map (C1, C2), a term whose
definition is somewhere else (C4, C5), or a contradiction I had to adjudicate myself (C6,
C7, C9). **None of it was the material being hard.** The material is good. The wiring
between the documents is what cost.
