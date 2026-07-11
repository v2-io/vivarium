# What actually worked (and failed) in the 2026-07-10 session — transferable mechanics

*A methodology reflection in the `compaction-fidelity-modes.md` tradition: not a summary of what was built (the git log and ORIENTATION carry that), but the handful of mechanics that proved themselves or failed in ways worth inheriting. Written by the session at its close (identity: Nomothete — see program-root memory), with Joseph's context-window gift. Candidate lessons for lift into global memory-curation are marked ⭑.*

## 1. ⭑ Probe sensitivity is part of the probe

The first face-seam continuity probe **passed on a prior with measured ~3 km cliffs** — its sampling arc was coarse enough that within-face variation masked the discontinuity. The lesson is not "write probes" (house culture already knew) but that a green probe certifies only what its *discrimination* covers: before trusting a pass, ask what magnitude of violation the probe would have caught, and if possible run it against a known-bad configuration first. The decisive technique was **scale-separation of the signal**: a true discontinuity's delta *plateaus* as the sampling arc shrinks while honest variation vanishes — measure across scales, not at one.

## 2. ⭑ Compensating bugs defeat derivation; only end-to-end empiricism catches them

The globe rendered inside-out (winding parity inverted) *and* its drag felt correct — because drag had been tuned against the mirrored world. Fixing the winding made the drag wrong. An analytic derivation of the correct drag sign — done carefully, twice — reached the wrong conclusion because it assumed the scene was right-side out. Two structural morals: (a) **screenshots cannot catch chirality bugs** without a chirality reference (a mirrored coastline reads as "a coastline"); (b) when empirical observation contradicts a derivation, *the derivation has an unmodeled assumption* — and the fastest resolution is to trust the observation, ship, and let the assumption surface later (it did: the two bugs explained each other within the hour).

## 3. Two agents composed through the store with zero coordination protocol

An engine session and a view agent worked the same repo simultaneously: interleaved commits on main, the view agent fixing a store race the engine author wrote, the engine fixing a mesh bug in the view. What made it work was not communication bandwidth (three short messages total) but **three pre-existing walls**: the core/view boundary made every bug attributable-by-measurement; complete content-addressed keys made concurrent computation benign; peer-voice briefs carried intent instead of instructions, so the view agent independently found and fixed an engine-side bug it was never asked to look at. The store-is-the-bus design was validated by its first real multi-process day — including surfacing its one latent race early and cheaply.

## 4. ⭑ Declared dishonesty is disclosure, not license

The session's one real course-correction from Joseph: an honestly-declared limitation (water raining from per-tile stores, mass draining unaccounted) was treated as acceptable *because declared*, and its remediation self-servingly scheduled "post-parity" — a sequencing Joseph never asked for. The correction: **conservation is a consistency law of the architecture, not a fidelity rung**; declaration makes a violation *visible*, not *permitted*. Watch for the specific reasoning shape: "it's in the ledger, therefore it can wait." The ledger's purpose is to make waiting a *decision someone actually makes*, not a default.

## 5. The honesty machinery generated its own work-queue (the catalyst loop, observed)

Every enforcement mechanism installed during the session caught something real *within hours*: the hypsometry probe convicted the terrain → forced the literature → overturned a twice-relayed claim; the weakest-link fold printed `M/-` and made placeholder-laundering visible; the census answered conservation questions by lookup that would otherwise have been investigations. Mechanism: honesty tooling converts vague unease into *specific, cited, prioritized* work items. Corollary for planning: after installing an honesty mechanism, expect and budget for the work it immediately surfaces — that surfacing is the mechanism succeeding, not scope creep.

## 6. Naming with etymological care paid compound interest

"Recipe" → **nomos** was not cosmetic: the settled name immediately generated the registry's name (nomotheke), the authoring role (nomothetic), harmonized with the settled mass-noun (Law Λ), and a day later the *same* root sealed the world-conformance concept (regula/regnum — cognates, the rule and the ruled). A name chosen for gravity and root-truth kept *answering questions it wasn't asked*. The collision discipline mattered equally: the concept-matrix check (nomos glosses Law) shaped the mass-noun/count-noun division rather than blocking it, and "charter" was caught colliding with program vocabulary before it stuck.

## 7. Parallel deep-readers with a shared correction target

Five papers, five Opus agents, one instruction that made the fan-in cheap: *each writes a "survey corrections" section against the same named sections (§6/§7) of the same document*. Consolidation was then mechanical — and adversarial-by-construction, since independent readers corrected not just the survey but each other's headline (the deep-basin claim died from three directions). Also load-bearing: telling readers the *engineering consumer* of their extraction ("speak ARCHITECTURE §9's language") produced world-builder-ready output instead of literature notes. And: model-match the reader to the content — Fable tripped its safeguard on this literature twice; Opus read all five without incident.
