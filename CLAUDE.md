# vivarium — context for Claude (and successors)

**What this is.** A deterministic 3-D voxel planet, built to author worlds whose epistemic status can be *stated* — so that ASF/AAT claims can cite a specific world as evidence (*in vivia*: the register between a toy model, which has ground truth but trivial dynamics, and field data, which has rich dynamics but no ground truth). It is also a simulation game, in the RimWorld/DF lineage. Vivarium is a member of the **Archema program** (`~/src/archema-io/`) and a supporting project for **ASF** (`~/src/archema-io/asf/`).

## Where the project is, as of 13 July 2026

The project is undergoing a serious cleanup. There have been important findings, solutions, and architecture that were getting buried and incoherent. We are now working to give this project the same discipline and epistemological culture that ASF has evolved into.

As a result, a good deal that is cluttered or stale — **and also some of the recent, excellent findings** — is held in `.archive/`. Not because it is wholly deprecated, but so that we have a clean slate from which to rebuild the onboarding and the general processes from the ground up. If you are reading this, that rebuild is somewhere between *"just started"* and *"not quite done."* Ask before assuming which.

## Canon

**[`core/OUTLINE.md`](core/OUTLINE.md)** and the segment files it points to — analogous to ASF's `01-aat-core/OUTLINE.md` and `01-aat-core/src/`. **Read it first.** It carries the epistemology (`type` / `status` / `stage`, adopted from ASF and expected to evolve), the segment cadence, the format rules, and the open questions. It currently points at **zero segments**. That is the work.

Also standing in the main tree, and authoritative for what they record:

| | |
|---|---|
| [`FORMAT.md`](FORMAT.md) | How documents are written. §§1–4 are the segment conventions; **§§5–6 bind every file in the repository** — cross-references, prose voice, math. |
| [`ETHICS.md`](ETHICS.md) | Ethics of in-world agents — **including §0, the standing moratorium on endogenous emergence. That is program law and it governs everything.** Read §0 before any agent-seam or in-world-mind work. |
| [`ASF.md`](ASF.md) | Why this project exists in ASF terms: the AAT ↔ vivarium object mapping (we author from outside the same typed object — law $\theta$, state $\Omega$, chance $\varepsilon$, compute-shortfall — that an AAT agent infers from inside), the agent-seam design constraints, the A/B/C ASF reading gates (**Level C is a hard gate before any agent-seam work**), and the *in vivia* citation recipe. **Its §§2–3 are one session's unratified synthesis and say so.** |
| [`LEXICON.udon`](LEXICON.udon) | The term dictionary — what a word means, and what it was carved against. |
| [`DECISIONS.decision-log.udon`](DECISIONS.decision-log.udon) | Settled calls, with honest authority tags: `joseph` / `us` / `claude`. **`us` means Joseph actually decided.** A finding can be beautifully measured and still be unratified. |
| [`ASSUMPTIONS.md`](ASSUMPTIONS.md) | The magic-constant ledger, compiled into the test suite (`nomotheke.rs`). It carries a banner saying what it still owes. |
| `doc/` · `ref/` · `tabularium/` · `crates/` · `examples/` | The durable thinking, the reference material, the instituted artifacts, the code, the probes. |

## In `.archive/`, with no assurance yet that they are captured in canon

*Not deprecated — set aside so the front door is rebuilt from the sources rather than from a prior summary of them. Much of what is here is good, and some of it is the recent findings. `.archive/README.md` indexes it.*

- **`README.md`** — a minimal replacement is coming.
- **`ORIENTATION.md`** — the current-state map.
- **`TODO.md`** — the hand-written queue.
- **`SUPERSEDED.md`** — the retired-terms ledger.
- **`VIVARIA-DEFINITIONS.md`** · **`VIVARIA-DECLARATIVE-FRONTIER.md`** — the declarative-scaffold spikes: the layer map with build status, and the proposal that a declared epistemic tier be made falsifiable against an audit of the kernel.
- **the previous `core/OUTLINE.md`** — a first assembly of the specification, in 71 rows.
- **the previous `CLAUDE.md`** — the tiered reading gates, the Prime Question and modified-equation analysis, the bias-vs-noise audit.
- **`memory-surfaced-2026-07-13.md`** — seventeen Claude project memories, moved into the repo. They carry real research results that nothing in the repository could see or check.

## ⚖ The gate

> **[`LEXICON.udon`](LEXICON.udon) is read for every term you touch — and a grep is not a read.**

The dictionary carries what a word means, what it was carved *against*, and **what has been retired**. A term in wide use may have been superseded last week, and the entry is the only place that is recorded — in its own `|carve` line, next to the word that replaced it. A hit count tells you a word appears somewhere. It does not tell you the entry says that word is dead.

## While the rebuild is underway

Joseph's global disposition applies with full force — truth-honoring over helpfulness-as-costume, strengthen before softening, mark guesses as guesses, no "complete / comprehensive" overclaim, peer voice when delegating. This project is early; say *"I don't know"* freely, and say *"that has not been decided"* freely.

Work goes directly on `main`; commit often, with messages written for the archaeologist. Two things are worth naming because the project paid for them:

- **Determinism is ontology.** All stochasticity is fated noise — a pure function of (seed, key). No wall-clock, no OS entropy, no shared mutable RNG.
- **A probe that cannot fail is not a probe.** Check that the physics can actually *execute* at a probe's footprint before trusting its number, and be more suspicious of a number that confirms your prior than of one that surprises you.

**Memory bridge.** If you navigated here mid-session, this project's memory did not auto-load — read `~/.claude/projects/-Users-josephwecker-v2-src-archema-io-vivarium/memory/MEMORY.md`. It holds procedure and standing law only; research is in the repo.

---

## Timeline

*Dates are commit timestamps. Ages are traced through rename chains — a birth date read off `git log <path>` reports the reorg or the format migration, not the concept.*

**The repository is 24 days old.** First commit **2026-06-20**.

**Jun 20 — day one, docs first.** `DESIGN.md` is in the first commit. The three founding commitments — the core/view wall, determinism-as-ontology, the fidelity invariant — are there before the engine is. Ethics is named on day one.

**Jun 22–23 — the engine spike** *(45 commits)*. Bevy against Godot, voxels, rendering LOD, the km-horizon problem. Bevy chosen.

**Jun 29 – Jul 2 — the physics era** *(141 commits; 92 on Jul 2 alone, the busiest day in the repository's life)*.

- **Jul 1, 23:08** — `DESIGN-MATERIAL.md`, `DESIGN-REDUX.md`, `DESIGN-SYSTEMS.md`. `DESIGN-MATERIAL` §4 declares the column semantics; the phrase *"sufficient statistic"* first appears the same day.
- **Jul 1, 23:40** — `erosion.rs`. fBm prior → MFD → stream power → Davy-Lague.
- **Jul 2, 02:13** — `water.rs`. Virtual pipes, local-inertial shallow water.

The kernels were written about a day *before* the document that says how to build them honestly. Both of those declarations — the column semantics and the sufficient-statistic contract — are still open.

**Jul 3–4 — orientation outward.** Hydrology merged. `LEXICON` seeded (Jul 3). `ASF.md` (Jul 4) reframes vivarium as a supporting project for ASF; *"fated"* enters the vocabulary.

**Jul 6 — the moratorium.** No endogenous instantiation of frontier or emergence-capable LLMs. Derived from the access matrix rather than stipulated. It is the oldest thing in the project that has never needed a correction.

**Jul 9–10 — the frame** *(38 commits)*. The world-model clean room. `store.rs` (Jul 10, 16:14), `ASSUMPTIONS.md` (20:29), `nomotheke.rs` (20:52). `ARCHITECTURE.md` v0.3 — *"represent by consequence"* and *"nomos"* both enter the language. §9 states a six-clause contract for what a system must declare; two clauses are built.

**Jul 11 — codification** *(12 commits)*. The doc-tree reorg. `LEXICON.md` → `LEXICON.udon`. `tabularium/` and the Terrestris ordinum (17:20). `PROCESS.udon`. Bequest → Promise.

**Jul 12 — the machinery, and the first corrections** *(37 commits)*.

- **02:33** — `flux.rs`: the shared flux-quantity vocabulary, with a closure test that fails the build on a typo.
- **15:09** — `DECISIONS.decision-log.udon`. Nothing before this has a decision record.
- **15:44 → 16:45** — `hydrosphere` (the first reservoir-box nomos) then `climate`. The water chain closes; the old ~9000× rain fudge decomposes into a principled precipitation and a declared fill acceleration.
- **17:22** — the fluvial probes had been measuring seabed. Erosion no-op'd on a fully submarine footprint, so the tests were comparing no-ops, and the "seam ratio 22888" was `0 ÷ 1e-9`.
- **18:15** — Joseph's correction: the submerged world is the Protogenic promise *kept*; `SEA_LEVEL_M` is what manufactures forbidden land.
- **18:22** — `ordinum.rs`. The phase ladder governs the flux web, and convicts this world of running erosion without land. It catches two undeclared nomos on its first two runs.
- **18:51 → 22:02** — `grid_lab`: nine grids measured on one harness. The MFD fan is a bias, and it does not converge away under refinement.

**Jul 13, 00:27 – 03:55 — the audit night** *(14 commits, eight agents)*.

- **00:27** — `doc/theory/discretisation-and-information.md`. The Prime Question is named — *what physical claim is this algorithm actually making?* — and it is a **computation** (modified-equation analysis: Taylor-expand the scheme, read off the PDE it is actually solving), not a disposition.
- **01:22** — a red team kills all three headline claims of the pass that preceded it.
- **01:38** — `water.rs` measures both better and worse than believed: already staggered, exactly well-balanced, no null space — and running outside its published validity envelope.
- **01:45** — the grid verdict reverses under modified-equation analysis.
- **02:07** — isostasy is the uplift nomos.
- **02:35** — the grid, tentatively decided (Joseph): keep the cube-sphere, stagger it.
- **02:56** — MFD's output is a boundary integral, not a discharge — ill-posed in the continuum.
- **03:55** — `p = 1.1` *is* the bias rather than its cure. The falsification had been printing in our own control on every run and was read as a baseline.

**Jul 13, from midday — the reading pass, and this cleanup.** A pass over the corpus established that the specification exists only in the *union* of some twenty-five prose documents, and that no single artifact contains it. The front doors were archived and `core/` was begun.

### What the dates carry

The concepts are older than their mechanization. The phase ladder was thought on Jul 2 and became machine-readable on Jul 11 — and on the day it became *data*, it convicted the world and caught two undeclared nomos. The column semantics were declared on Jul 1 and are still prose.

The purpose also changed mid-flight. Joseph, 2026-07-13:

> *"The project, notwithstanding the ethics, has genuinely transformed from 'strictly casual and for fun — a distraction from ASF-rigor work' to 'serious project that gives worlds to ASF that allow in-vivia studies' (and maybe some games as well) — as another leg for the Archema research program generally. So the trend toward rigor wasn't initially expected but is not accidental."*

The kernels were written to a game's bar, which was the correct bar at the time. The audit is the first measurement of that work against a research instrument's requirements: **the bar moved under finished work.**
