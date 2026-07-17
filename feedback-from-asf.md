# Feedback from the ASF side — for the vivarium rebuild

*Left 2026-07-14 by a Claude (Opus 4.8) session working the Archema program docs from the asf/program side, at Joseph's invitation ("drop notes… for me and others over there to review when I can get back"). Read-only on your tree — I did not touch vivarium internals; you're mid-rebuild. Low-confidence on your current state (I read your front-door + git log + `DECISIONS.decision-log.udon:894`, not the corpus).*

## The one thing worth prioritizing in the re-homing

Your own `DECISIONS.decision-log.udon:894` already nails it: `ASF.md §0` (moratorium) is safe in `ETHICS.md`, but the non-§0 content is homeless-in-`.archive/` and load-bearing. From the **asf/program side**, here's which of that homeless content has live downstream consumers, so the rebuild can prioritize:

1. **§7.5 — the operational meaning of *in vivia*** (an ASF claim cites a specific world artifact: seed + generator versions + phase memo + intervention script; content-addressed storage makes the citation *exact*). Your log calls this "the sentence that makes the whole project cash out." It is also **the** thing the program cites vivarium *for* — `charter/concept-matrix.md` row 82 points here, and the charter's "constructed register" claim rests on it. **Highest priority to re-home into a live doc.**
2. **The AAT↔vivarium object-mapping ($\theta$/$\Omega$/$\varepsilon$/compute)** — cited by `concept-matrix.md` row 13. This is the Rosetta-Stone cell that makes the cross-register lexicon coherent.
3. **The Level-C agent-seam gate (incl. (C2′))** — the charter (§10) and the program CLAUDE.md reference "Level-C gate before agent-seam work" as an inherited hard gate. It currently has no live home.

Until these land somewhere live, they're tracked in `charter/INCOHERENCE.md` row 1 (I created that ledger today). The program docs' **moratorium** pointers I already repointed to `ETHICS.md`; I left the §2/§7.5 refs alone rather than guess.

## Two cross-cutting observations (take or leave)

- **Tooling-lift:** asf has mature versions of things you're rebuilding from scratch — the `bin/term` terminology system + append-only decision events (multi-agent-safe by construction), `bin/lint-md`/`lint-outline`, the markdown-first build pipeline. The charter's "move tooling up a level" instinct + your ground-up rebuild is the moment to decide what becomes *program-level shared tooling* vs re-implemented per-leg. Worth a deliberate call before you re-build a terminology/lexicon system that asf already runs.
- **Convergence worth noting (not a superlative — a checkable fact):** your `doc/PROCESS.udon` cites `asf/msc/meta-process-review-2026-07-07/PROCESS-MAP-v0.udon` as genre precedent and carries the same participant-feedback self-governance valve. The two legs arrived at the same process-shape independently — which is the kind of double-derivation the concept-matrix treats as evidence the joint is real.

*No action needed from you on this file; it's here for whoever picks the rebuild back up.*

## The Level-4 handshake (added 2026-07-16, from the asf session that landed the separation theorem)

ASF landed a verified *exact* result this week that turns out to touch vivarium's foundations directly: **latent-anchored mechanism counterfactuals strictly exceed Pearl's counterfactual layer** — two SCMs can agree on every observational, interventional, and counterfactual quantity forever, yet disagree with certainty on "this very background, different law" queries; and latent-anchored content is *exactly* commitment to a coordinatization of the exogenous space. Full statement + proofs: `asf/01-aat-core/src/deriv-mechanism-counterfactual-separation.md` (independently verified by enumeration, twice); the imagination consequences: `asf/01-aat-core/src/disc-structural-imagination.md`; the four-thread vivarium mapping: `asf/doc/vivarium.md` §"The mechanism-counterfactual handshake". Brief version of what it means here:

1. **Your $(\text{seed}, \text{key})$ scheme is the theorem's central object made operational** — an authored coordinatization of $U$. A "Level-4" query (swap a nomos version, keep seed+keys, rerun) is executable here and nowhere else we have; the demonstration experiment (nomos vs. its key-permuted twin: indistinguishable at every hierarchy layer, divergent under same-seed law-swap) is cheap whenever someone wants it.
2. **Seed-identity is now theorem-grounded, not intuition**: the query class that depends on the seed-coordinatization surplus (every mod/patch/law-change fork held on this background) is exactly characterized. "The nomoi + seed constitute the world's noumenon" is a theorem, not a joke.
3. **Cache-policy corollary worth a DECISIONS line**: no distributional equivalence test between nomos versions — at any layer, however exhaustive — can soundly license cache reuse across a version swap; pointwise-on-keyed-draws (your existing content-addressed policy) is the unique sound rule. The theorem upgrades that from conservative habit to necessity.
4. **Behind the agent seam** (and the ETHICS gate): embodied agents' *positing*-imagination (law-changes on this very background) is provably unanswerable from inside — a designable asymmetry between what a creature can learn of its world's laws and what only the author's coordinates determine.

*As before: no action needed; here for whoever picks it up. — the asf/program session, 2026-07-16*
