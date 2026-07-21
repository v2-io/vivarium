---
slug: def-in-vivia
type: definition
status: exact
stage: draft
depends:
  - def-vivium
---

# In vivia

**In vivia** names the empirical register of results obtained inside a named vivium: between a toy model (ground truth, trivial dynamics) and field data (rich dynamics, no ground truth).

## Formal Expression

A result is obtained **in vivia** when it is produced by running or inspecting a specified vivium ( #def-vivium) under declared law, seed, generator/nomos versions, and intervention script, such that:

1. dynamics are those of the constructed world (not a hand-waved cartoon with no state), and  
2. ground truth for authored quantities is available by construction (not only estimated from outside), and  
3. the citation is in principle reproducible from those declarations.

The register is **stronger in legibility** than field study and **stronger in dynamical richness** than a minimal toy — only when the fidelity and declaration program actually hold for the quantities claimed.

## Epistemic Status

**Max attainable: exact** as a definition of a citation register; **conditional** as a quality claim about any particular study (the study must actually meet (1)–(3)). Joseph's 2026-07-04 bet — simulation results epistemically nearer real-world empirical results than toy simulations — is **conditional on the fidelity program succeeding**, not a theorem. Stage `draft`. Operational citation recipe (seed + versions + phase memo + interventions) is not fully mechanized; do not claim bit-perfect citation infrastructure exists until a segment and tools say so.

## Discussion

ASF's software laboratory is high-identifiability because software exposes state. Vivarium aims at the same identifiability **by construction** for a world that is not only software-process but authored nature-like dynamics. Without declared tiers and probes, "*in vivia*" collapses into marketing language — which is why #norm-declaration-must-convict and #norm-probes-before-claims bind laboratory use.

## Working Notes

- Content-addressed store makes exact citation *possible*; generator-pinning and source-derived nomos versions remain open ( #lexicon/term/generator-pinning).
