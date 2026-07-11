# vivarium — context for Claude (and successors)

**What this is.** A simulation game (RimWorld/DF lineage) on a deterministic 3-D voxel planet, whose real bet is *simulation-grade agents and community dynamics* built on the Agentic Systems Framework (ASF/AAT) — and, dual to that, a high-identifiability laboratory where AAT quantities have ground truth by construction. Vivarium is a member of the **Archema program** (`~/src/archema-io/`) and a **supporting project for ASF** (`~/src/archema-io/asf/`).

## Reading gates — tiered, so onboarding is minutes not hours

- **Level A — every session, before any work.** This file · [`ORIENTATION.md`](ORIENTATION.md) (the current-state map) · [`ASF.md`](ASF.md) (the ASF bridge and **the standing moratorium, §0** — program law). Cost: minutes.
- **Level B — before designing or extending any world system.** [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) (the frame: one principle, three axes) · [`LEXICON.udon`](LEXICON.udon) for every term you touch · the specific `doc/design/` + `doc/plan/` docs your work lands in · ASF's own Level B (`ASF.md` §5). A nomos is not designed from an unread architecture.
- **Level C — before touching the agent seam.** Hard gate, unchanged: the named ASF reading in `ASF.md` §5. The agent seam is the project's reason to exist and is not built from an unread theory.

Everything else — `doc/theory/`, `ref/` — is pulled on demand, not read in order. **The doc tree:** root = front doors (this file, README, ORIENTATION, TODO) + the ethics/moratorium unit (ASF.md, ETHICS.md) + the three ledgers (LEXICON, SUPERSEDED, ASSUMPTIONS — the last is `include_str!` into the test suite; it does not move). `doc/` = the durable thinking: `ARCHITECTURE.md` + `PHASES.md` at top, `design/` (founding + the fidelity/matter/systems docs), `plan/` (the live build path: abyssal-parity-plan, builder-explorer-decoupling, regula-conformance-design, framework-to-status-quo, vivium-operational-workflow, water-parallelism), `theory/` (multiscale-methods, multiscale-seams). `ref/` = true reference (surveys, dossiers, NOTES from past eras, pdfs) — consulted, not onboarded. `msc/` = session reflections. `.archive/` = sunset material, indexed (`.archive/README.md`): **archive generously** — the main tree carries current working state only, and must never depend on .archive for *tactical current* needs; historical/provenance/reasoning-trail pointers into it are proper and encouraged (Joseph: "a program unwilling to sunset things becomes unable to attempt things"). Norms for keeping it this way: [`doc/PROCESS.udon`](doc/PROCESS.udon).

## The disciplines that bind here

**The ledgers + the nomotheke.** [`ASSUMPTIONS.md`](ASSUMPTIONS.md) is the canonical accounting of every unprincipled constant; the **nomotheke** (`crates/vivarium-world/src/nomotheke.rs`) is where every nomos *declares* its epistemic tags, deps, promises (with conservation claims), and assumption anchors — declarations mint the store keys, the ledger compiles into the tests, derived quality is a weakest-link fold. **Touch a constant, touch the ledger; add or change a nomos, declare it.** An undeclared magic value is latent, *undiscoverably* unLawful (Joseph, 2026-07-10); a declared placeholder is an honest rung. The world-level third grain — the **regula** (slots, permits, posture; first instance **Regula Terrestris**) — is specified in [`doc/plan/regula-conformance-design.md`](doc/plan/regula-conformance-design.md).

**Determinism is ontology.** All stochasticity is fated noise — a stateless KRNG of (seed, key). No wall-clock, no OS entropy, no shared mutable RNG, no iteration-order leaks into anything canonical. The epistemics toolchain ([`doc/toolchain.md`](doc/toolchain.md)) lists what enforces this mechanically and what is queued.

**Physics, not knobs; probes before claims.** The principled path is rarely more code — just more thought (Joseph). No physics claim without its probe run; known issues get their probe written *first*. Detail must be earned, never painted.

**Timescale separation** is the load-bearing lesson this project paid for: erosion is geological, water is hydrological — never one timestep. Generalized: `doc/ARCHITECTURE.md` §1 (the operator algebra's third law).

**Inherited disposition.** Joseph's global CLAUDE.md applies with full force: truth-honoring over helpfulness-as-costume, strengthen-before-soften, integration-is-replacement, mark guesses as guesses, no "100%/comprehensive" overclaim, peer voice when delegating. This project is early; say "I don't know" freely.

**Documents.** New hybrid prose+structure docs (norms, process maps, the Ordinum, conformance profiles) are authored in **udon** (`~/src/udon/`; safe-subset discipline in `doc/PROCESS.udon`); markdown remains fine for prose-first docs and migrates opportunistically, never as a sweep.

**Git workflow.** Joseph's solo research repo; work goes **directly on `main`** — no branches/worktrees/PRs unless he asks. Commit freely and often with messages written for the archaeologist (Joseph, 2026-07-11: "commit whenever you see fit, as often as you see fit").

## The open question the whole bet rests on (do not assume it away)

Whether LLM-induced perturbations on the formal agent state stay legible enough to measure adaptation cleanly, or wreck identifiability. It is empirical; the vivarium-as-AAT-sandbox framing depends on it; the first vertical slice of the agent seam exists to test it.

## ⚠ Memory bridge (Archema program)

**If you are reading this mid-session because you navigated here from elsewhere, this project's memory did NOT auto-load** — Read `~/.claude/projects/-Users-josephwecker-v2-src-archema-io-vivarium/memory/MEMORY.md` now, before substantive work. (Project memory loads only by exact session-start directory; the program root's CLAUDE.md cascades, memory does not.)
