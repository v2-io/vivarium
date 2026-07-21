---
slug: disc-aat-vivarium-object-map
type: discussion
status: discussion-grade
stage: draft
depends:
  - def-in-vivia
  - post-determinism-as-ontology
  - disc-vivarium-purpose
---

# AAT ↔ vivarium object map

AAT types the agent's epistemic target from the inside; vivarium authors the same object-kinds from the outside. The correspondence below is a proposed reading, not ratified law.

## Formal Expression

The connection is not presented as a proved isomorphism. AAT's root-ontology work (`asf/spikes/epistemic-target-ontology/`) types the epistemic target as $S_t = (\Omega_t, \theta)$ — fast **state** plus persistent **law** — with **chance** $\varepsilon$ as what remains fresh once everything persistent is named into $\theta$, and **compute-shortfall** as ratio-type ignorance (knowing the generator ≠ running it faster than reality). Vivarium constructs worlds whose authored faces are the natural counterparts:

| AAT (the agent infers, from inside) | vivarium (we author, from outside) |
|---|---|
| law $\theta$ — transition / observation structure | physics and constants; each phase's Promise (the box later computation happens inside) |
| state $\Omega_t$ | live world state — terrain, water, weather, populations |
| chance $\varepsilon$ | **fated noise** ( #lexicon/term/fated-noise) — pure function of $(\mathrm{seed}, k)$ ( #post-determinism-as-ontology) |
| compute-shortfall | fidelity ladder and lazy memoized evaluation; the runtime as compute-shortfall manager end to end |
| chronica $\mathcal{C}_t$ ( #asf/1-aat/def-chronica) | a world's **interventional history** ( #lexicon/term/interventional-history) |

**What is settled locally.** The AAT symbols $\theta$, $\Omega$, $\varepsilon$ are the symbols vivarium uses when it speaks of law, state, and chance ( #lexicon/note/collision-ledger · #lexicon/note/aat-handshake). Determinism-as-ontology and the *in vivia* register ( #def-in-vivia) are independently owned claims; this segment does not re-derive them.

**What this segment does not claim.** It does not claim that every AAT quantity is already implemented, dialable, or measured in code. It does not promote the four synthesis consequences listed in Working Notes to proved results. It does not re-prove AAT.

## Epistemic Status

**Max attainable for the table as a correspondence: discussion-grade** (or `conditional` on future ratification). **Do not cite this segment as exact.**

This map is **one session's synthesis** (bridge session 2026-07-04), verified against cited ASF material where noted in the historical bridge prose, marked as hypothesis where not. Joseph has not ratified the mapping as a whole. What is positively settled is narrower: symbol adoption and the local walls (determinism, vivium, *in vivia*), not the full AAT↔vivarium reading of phase ladders, $\rho$-schedules, or two-layer-mind dialability.

Stage `draft`. Upstream root-ontology spike is itself unlanded as ASF canon; that incompleteness is inherited, not hidden.

## Discussion

The laboratory half of #disc-vivarium-purpose needs a vocabulary bridge so world systems are built with $\theta$, $\rho$, and persistence in mind ( #scope-asf-reading-gates Level B). The failure mode is either (a) treating this table as proved AAT, or (b) refusing any map and rebuilding weather as if no agent theory will ever live here. Discussion-grade is the honest middle: usable orientation, not law.

## Working Notes

**Open / follow-on claims — not proved by this segment.** The historical bridge listed four consequences; keep them out of Formal Expression until separately extracted and tiered:

1. **Phase-transition as law-promotion** — converged state promoted into $\theta$ for later dynamics; incremental sunset/spin-up rather than full law swap; "invariance, not slowness" as the cut. (Agent synthesis leaning on AAT spike language; incremental-transition content has separate Joseph history in phase design.)
2. **Phase ladder as $\rho$-schedule** — successive phases as decreasing disturbance rate toward agent-feasible persistence. Hypothesis; unratified; candidate for honest-tier upstream offer to ASF.
3. **Frame-relative aleatoric boundary** — fated noise is chance to the inside agent and lookup to us; housing rule is chosen. Leans on GA-1 twin / freshness material in the epistemic-target spike.
4. **Two-layer mind identifiability as dialable AAT question** — whether LLM perturbations on formal agent state stay legible; bias bound / observation ambiguity $\mathcal{A}$ authored here. Open empirical question in design prose; any experiment clears ETHICS + Level C first ( #scope-agent-seam-constraints).

- When Joseph ratifies any row or consequence, promote that piece (or replace this segment's status for that piece) — do not invent `:by joseph` ratification from the 2026-07-04 bridge session alone.
- Historical section home for this table: former `ASF.md` §2 (now a non-authoritative router).
