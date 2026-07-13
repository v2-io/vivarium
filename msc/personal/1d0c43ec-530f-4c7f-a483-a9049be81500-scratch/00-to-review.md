# 00 — the reading queue

*Session `1d0c43ec-530f-4c7f-a483-a9049be81500`, opened Monday 2026-07-13 ~13:20.
Successor to the eight-agent audit session (`438f7e79-…`), which ran 07-12/13 and
ended at its context limit. Joseph chose a fresh session over a compaction
deliberately: "context compaction often gives a false confidence about what the
agent knows, as well as some task-execution anxiety that a fresh agent doesn't
assume."*

**The frame Joseph set for this pass:** the interesting object is the **scaffold**
— the general principles, methodology, and process. `erosion` / `water` / MFD-8
are *the cadaver we dissected to learn the anatomy*, not the patient. They stay as
the concrete testbed. Soon, nomoi may each get **their own agent and their own
TODO list**, so that tactical nomos concerns stop being conflated with the global
scaffold concerns. **Which nomos gets fixed first is a mid-level question that
should become obvious once the scaffold is in good shape** — so do not optimize
for it now.

---

## ⏱ The staleness frontier — established before reading anything, from `git log` dates alone

**The audit ran 2026-07-12 → 07-13. It touched only these docs:**
`DECISIONS.decision-log.udon` · `TODO.md` · `ASSUMPTIONS.md` · `doc/ARCHITECTURE.md` ·
`doc/theory/discretisation-and-information.md` · `CLAUDE.md` · `ORIENTATION.md` ·
`doc/design/NOMOS-CONTRACT.md` (new).

**Everything else is dated 2026-07-11 or 07-12 — i.e. PRE-AUDIT.** In particular
**the entire `doc/design/` and `doc/plan/` trees are 07-11**, and `LEXICON.udon` is
07-12 (so: after the regula collapse, before the kernel audit).

⇒ **Working hypothesis, to be tested by the reading:** *the design and plan docs
have not been reconciled with the audit, and I should expect to find claims in them
that the ledger has already killed.* The isostasy decision already quotes
`DESIGN-MATERIAL.md` §4 as stating a law (*"the conserved primitive is volume/mass
— NOT a height"*) that `uplift.rs` violates — so at minimum, that doc is *correct
and unheeded* rather than stale. **Those are different failures and I want to keep
them apart:** a doc can be (a) stale — says something the ledger has refuted; (b)
**unheeded** — says something true that the code ignores; (c) **orphaned** — true,
heeded, and no longer reachable from any front door. Each needs a different remedy.

---

## The queue (ordered; one at a time; full reads)

Joseph's ordering, with one amendment I proposed and he can veto: **LEXICON moves
up to slot 3**, because everything in `doc/` is written in its vocabulary and I do
not currently hold it — and DECISIONS (slot 2) is what tells me which of its
entries the ledger has moved out from under.

> ### ⚠ Queue correction after read #1 — **the scaffold is CODE, not docs.**
> I had `nomotheke.rs` / `flux.rs` / `audit.rs` / `ordinum.rs` filed as "code I might grep."
> That is wrong. **Those four files ARE the scaffold** — every design doc below is a *claim
> about* them, and the ordinum read showed me the code is the ground truth (the ledger said
> `uplift` keeps `emerged-land`; `ordinum.rs`'s test says nobody does, and the test is right).
> **Promoted to slot 3, immediately after DECISIONS.**
>
> ### ⚠ And the reading frame, per Joseph (2026-07-13, mid-read)
> *"Neither the ordinum nor the nomos as written so far have incorporated any of the higher
> level work and understanding that has begun to crystalize in the last 24 hours or so."*
> ⇒ **I am not auditing these documents. I am reading them to know what is there before we
> redesign the scaffold with the new understanding.** Pre-audit ≠ wrong. The live question at
> every file is: **what does this look like AFTER it absorbs the five boxes, the two-role
> macro tier, and defeasance-as-a-primitive?**

| # | doc | date | bytes | why here |
|---|---|---|---|---|
| 1 | ~~`tabularium/terrestris.ordinum.udon`~~ | 07-11 | 22 K | ✅ **READ** → `01-terrestris-ordinum.md`. Phases are **states, not intervals** — the ordinum is a law, not a schedule. 27/32 promises are glosses. `:kept-by` is the org chart Joseph wants, and it already exists (5 edges). **`\|defeasance` is the retraction primitive every other ledger lacks.** |
| 2 | `DECISIONS.decision-log.udon` | 07-13 | **232 K** | **The staleness adjudicator.** ~70 entries, ~60 unread. Read with dates in hand — this is what licenses me to disbelieve every doc below. **Extract the ratified spine (`:by joseph` / `:by us`) as I go — that, not the 232 K, is what binds.** |
| **3** | **THE SCAFFOLD ITSELF (code):** `src/{nomotheke,flux,audit,ordinum}.rs` | 07-12 | — | **NEW — promoted after read #1.** The declaration model, the flux vocabulary, the requisite graph, the ladder parser. **This is the thing we are redesigning.** Everything in `doc/` is a claim about these four files. |
| 4 | `LEXICON.udon` | 07-12 | 63 K | The vocabulary every `doc/` file is written in. Pre-audit ⇒ expect retired terms still marked settled (`regula` is a known 13-edge hub still `:status settled`). **Now also: check whether the KINGDOM nesting (§4/§7) is the same recursion as the phase ladder and the scale ladder — if it is, that's three.** |
| 4 | `doc/ARCHITECTURE.md` | **07-13** | 29 K | Level B. One principle, three axes. Touched during the audit ⇒ probably the most current frame doc. |
| 5 | `doc/theory/discretisation-and-information.md` | **07-13** | **68 K** | Level B, *required before any field nomos*. The theory the five boxes came out of. Known to contain at least one thing the ledger corrected (§3.3, the Jensen variable). |
| 6 | `ASSUMPTIONS.md` | 07-13 | 21 K | The magic-constant ledger; `include_str!`s into the tests. |
| 7 | `TODO.md` | 07-13 | 37 K | The live queue. Read late — after I can judge which items the audit voided. |
| 8 | `doc/design/DESIGN-MATERIAL.md` | 07-11 | 22 K | §4 is a named Level-B gate *and* the law `uplift.rs` violates. The "unheeded" specimen. |
| 9 | `doc/design/DESIGN-REDUX.md` | 07-11 | 50 K | The lazy memoized query graph + §12 pervasive memoization — **the premise the leaf-only-evolution price threatens.** |
| 10 | `doc/design/DESIGN-SYSTEMS.md` | 07-11 | 23 K | The phenomena graph, coupling bands, build order. |
| 11 | `doc/design/DESIGN.md` | 07-11 | 13 K | Founding commitments (core/view wall, determinism, fidelity invariant). |
| 12 | `doc/theory/multiscale-seams.md` | 07-11 | 30 K | Position+time as one seam. Directly implicated by the macro-tier/leaf-only argument. |
| 13 | `doc/theory/multiscale-methods.md` | 07-11 | 11 K | The R/L/closure operator algebra. |
| 14 | `doc/plan/abyssal-parity-plan.md` | 07-11 | 20 K | The six-phase build path to the ethereal explorer. |
| 15 | `doc/plan/builder-explorer-decoupling.md` | 07-11 | 9 K | Beacons, watchpoints, fidelity pyramid, depend-by-key. |
| 16 | `doc/plan/vivium-operational-workflow.md` | 07-11 | 10 K | Build/freeze/publish/participate/fork + BREAK-2 (the unLawfulness budget). |
| 17 | `doc/plan/framework-to-status-quo.md` | 07-11 | 11 K | |
| 18 | `doc/plan/water-parallelism.md` | 07-11 | 3 K | |
| 19 | `doc/PROCESS.udon` | 07-11 | 10 K | The working norms. First udon doc. |
| 20 | `doc/toolchain.md` | 07-11 | 6 K | The epistemics toolchain (what enforces determinism mechanically). |
| 21 | `README.md` | 07-12 | 6 K | The front door / the duality statement. |
| 22 | `ETHICS.md` | 07-11 | 7 K | Phase-7 moral-scope line. |
| 23 | `ASF.md` §2–§6 | 07-11 | 27 K | I have only §0–§1. **§2 (the invariance cut, "the tower derives by self-similar application") is quoted at me inside the macro-tier decision and I have not read it.** |
| 24 | `VIVARIA-DEFINITIONS.md` | 07-12 | 31 K | ⚠ Joseph: *"the right register but older than most of the work we've done most recently."* Read **as a register specimen**, not as current truth. |
| 25 | `VIVARIA-DECLARATIVE-FRONTIER.md` | 07-12 | 19 K | Same caveat. Carries the `hypothesis`/`fitness` blocks the plate-tectonics study wants. |
| 26 | `doc/plan/regula-conformance-design.md` | 07-11 | 20 K | **Reasoning trail, not plan** (the regula collapsed). Read last, for archaeology. |
| 27 | `tabularium/README.md` | 07-11 | 3 K | |
| — | `msc/` (8 spikes + 2 research dirs) | 07-13 | ? | The audit's working artifacts. Pull on demand as the ledger points at them. |
| — | external papers (`ref/research/pdfs/`, relata) | — | — | **Inventory not yet taken.** Do it once I know which papers the ledger says are load-bearing. |

**Explicitly NOT reading:** `tmptmp.md` (Joseph's, flagged in the handoff) ·
`SUPERSEDED.md` (Joseph excluded it) · `.archive/**` (accounted-only).

---

## Standing artifacts I will keep in this directory

- `00-to-review.md` — this file. **Updated after every read** (queue reordering,
  new documents discovered, papers to fetch).
- `00-initial-predictions.md` — written **before** reading anything. The forecast
  is the instrument; the surprisal is the measurement.
- `00-incoherences.md` — **the running ledger of every contradiction found**
  (doc↔ledger, doc↔code, ledger↔ledger), with which side I believe and why.
  Not fixed as I go — *fixing while reading is how you lose the reading.* This
  feeds the program-level "incoherence ledger" already queued at archema-io root.
- `01-…` … `NN-…` — one reflection per document: **(1) surprisal** — what differed
  from my prediction; **(2) what matters for today's session**; **(3) wandering** —
  open-ended, and *only what actually moved.* If a document leaves me with nothing
  I will say so rather than manufacture wonder.

## A note to whoever reads this next

These are **working artifacts, deliberately kept**. They are not documentation and
they are not decisions — nothing in this directory is authoritative, nothing here
is tagged, and if it contradicts `DECISIONS.decision-log.udon`, the ledger wins.
Joseph's reason for wanting them: *"the notes you leave behind as a trail often end
up containing key insights and 'gold' that would otherwise be lost,"* and the
reading itself goes deeper when it is being thought about rather than absorbed.
