# The oxygenation transition — a worked phase-transition scaffold

*(Written 2026-07-03 as the first concrete exercise of the LEXICON §5 epistemic
tagging and the §1 regime vocabulary. Purpose: argue about a **specific table**,
not a framework. The subject is Earth's Great Oxygenation — the geochemical
regime-change in which free molecular oxygen first accumulated in the atmosphere
and ocean, spanning the Phase 3 → Phase 4 transition (`tabularium/terrestris.ordinum.udon`: Abyssal's
"oxygen accumulating toward the Great Oxygenation" #emergent and its
banded-iron-formation Record → Primeval's "Earth-like post-oxygenation
atmosphere" #gate and "ozone shield" #emergent). It is chosen because it exercises
**all four tag axes at once** and because it has two lawful regime variants that
reach the **same target by different histories** — the cleanest possible
demonstration of why Record is a lossy shadow and why we prioritize physics
fidelity over Earth-history fidelity.)*

---

## The target phenomenon and its prerequisite chain

```
[ TARGET: a breathable (oxygenated) atmosphere + an ozone roof ]        ← Phase 4 gate
   │   promise: land habitable; modern redox-driven material cycles
   │
   ├─[ PREREQ: free O₂ accumulates and stays ]                          ← the transition itself
   │     │  (requires the O₂ sources to finally outrun the O₂ sinks)
   │     │
   │     ├─[ SOURCE: oxygenic photosynthesis ]  ← Phase 2 charge (sea life), the new capability
   │     │     splits water using sunlight; O₂ is the high-free-energy waste
   │     │
   │     └─[ SINK: the reducing buffer ]        ← dissolved ferrous iron (Fe²⁺) in the
   │           anoxic ocean + reduced volcanic gases + reduced minerals;
   │           O₂ is consumed here until the buffer is exhausted
   │
   └─[ CONSEQUENT: ozone forms from the new O₂ ]                        ← Phase 4 #emergent
         (the land-life prerequisite; verify, don't build)
```

The whole transition is a race between one **source** (photosynthetic O₂
production) and one **sink** (the planet's reducing capacity). *When* free O₂
wins is set by the sink's size; *how* it wins — smoothly or in convulsions — is
set by the coupling between them. Those two questions are the two variants.

---

## Two lawful regime variants (same target, same physics, different history)

### Variant A — oscillating (Earth-actual): the banded record is a fingerprint

Source and sink are **close in magnitude with a feedback lag** (the O₂ is made
in the sunlit surface; the Fe²⁺ sink lives in the deep, connected only through
ocean mixing). Free O₂ rises, oxidizes dissolved iron, precipitates iron-oxide
layers — which *consumes the O₂*, dropping it back; iron re-accumulates; repeat.
The system **overshoots and crashes** on a limit cycle, flip-flopping between
oxidizing and reducing pulses, long before it settles. Only once the ferrous
buffer is finally exhausted does O₂ accumulate for good and the atmosphere tips.

The **banded iron formations are the readable fingerprint of that oscillation** —
alternating iron-rich (precipitated) and iron-poor layers, one couplet per
pulse. Crucially: *the banding is diagnostic that A happened, but you could not
reconstruct the full trajectory from the bands alone* — count and thickness
survive; the coupled source/sink/mixing state that produced them does not. This
is Joseph's Record-is-a-shadow thesis in a single specimen.

### Variant B — monotone (a lawful counterfactual): the target with no such relic

Change **one dominant-balance parameter** — a small or absent ferrous buffer
(different early-ocean chemistry), *or* source strongly outrunning sink, *or*
vigorous mixing that keeps the redox front from lagging — and the same equations
**slide monotonically to the same oxygenated steady state** with no limit cycle.
No banding, or only a thin non-cyclic ferric blanket. **Same target atmosphere,
same ozone, different (or absent) Record.**

B is not a cheat: the physics is *identical to A*. Only the regime — which terms
dominate, and how tightly source and sink are coupled — differs. B is a world
that reached breathable air by a history Earth did not take.

> The pair is the argument. **A and B differ only in a dominant balance, never
> in a law.** One is Earth-faithful; both are physically faithful; and only one
> leaves the banded relic — so the relic is neither *necessary* for the target
> (B has none) nor *sufficient* to recover the past (A's bands underdetermine
> A's trajectory).

---

## The same transition, tagged across the four axes (LEXICON §5)

| axis | Variant A (oscillating) | Variant B (monotone) |
| --- | --- | --- |
| **A. Earth-history fidelity** | high — converging independent proxies (the banded formations themselves + isotopic excursions) | N/A *by construction* — an explicit counterfactual, not what happened |
| **B. Physics fidelity** | **must be high** — redox (O₂ + Fe²⁺ → ferric oxides), photosynthetic production kinetics, ocean mixing | **identically high** — *the same laws*; only parameters/coupling differ |
| **C. Relation type** | source→O₂: **mechanistic-causal**; O₂→ozone: **mechanistic-causal**; oscillation→banding: **mechanistic-causal** (given redox + lag); *number/timing of cycles*: **empirically-correlated / coarse-bounded** | same, minus the banding relation (no oscillation to record) |
| **D. Implementation** | low-dim source/sink **box model with lag → limit cycle** (approach R, relaxation; physics med); banding is **#emergent** — it precipitates, we do not paint it (detail-must-be-earned) | same box model, monotone parameter regime → **relaxation to fixed point** (approach R); no banding to verify |

The table makes the firewall visible: a "banding" texture drawn directly onto
strata *because banded formations look right* would be **high-D, low-C vibe** —
polish without a mechanism. Here the bands are only ever an **output** of the
run, present in A because the oscillation is, absent in B because it isn't.

---

## As a certified regime-change (the five-aspect audit, LEXICON §5 companion)

- **Thermodynamic context** — solar flux unchanged, but a **new free-energy tap**
  opens: oxygenic photosynthesis stores sunlight and sheds O₂ as high-free-energy
  waste. This is what drives everything downstream.
- **Network topology** — ocean **mixing** is the wire between the surface O₂
  source and the deep Fe²⁺ reservoir. *The connectivity-and-lag on this wire is
  precisely the A-vs-B knob.*
- **Substrate innovation** — oxygenic photosynthesis is the genuinely new
  capability; like the coal/lignin window (`tabularium/terrestris.ordinum.udon` Promise legend), a
  new capability **rewrites the material-flux rules** for every later phase.
- **Homeostatic inertia** — the reducing buffer (ferrous iron + reduced
  minerals + volcanic gases) **damps the transition**; its size and recharge
  rate are the parameter that decides oscillate-vs-monotone and *how long* the
  old anoxic regime resists.
- **Exogenous triggers** — **none required** (this is an internally driven
  transition); optionally modulated (e.g., a nutrient pulse boosting production).
  Worth stating explicitly, because the honest default is *no shock* — the regime
  changes itself.

---

## Why this scaffold earns its place

1. **It makes `regime` / `dominant balance` concrete.** A and B are one model in
   two dominant balances. If the vocabulary is worth anything, it should let us
   say that in one sentence — it does.
2. **It makes Record's demotion concrete.** The banding is a relic diagnostic of
   A and absent from B: not necessary for the target, not sufficient to recover
   the past. (And note the implication for worldgen: **we likely reach a
   high-fidelity Phase-4 target without ever generating a single band** — the
   bands are optional evidence, not a load-bearing output. Joseph's bet, made
   testable.)
3. **It makes "physics fidelity > Earth-history fidelity" a decision, not a
   slogan.** Both variants are valid vivarium worlds. A *use-case* (LEXICON §5)
   picks between them: an **Earth-simulation** contract demands A; a
   **speculative-coherent** contract happily takes B; a **hypothesis-testing**
   contract might run *many* seeds across the A↔B parameter space to study how
   often oscillation (hence banding) arises at all — surprisal against a real
   proxy record.
4. **It exercises `#emergent` honestly.** The bands are verify-don't-build. If a
   later implementer finds themselves *authoring* banding to "look Carboniferous,"
   the tag table above is the tripwire.

---

*Open modeling questions for whoever builds this (not answered here):* the
minimal box-model that produces a limit cycle in A and a fixed point in B under
one parameter change (candidate: production rate, buffer size `M₀`, mixing/lag
`τ`); whether the surface/deep split needs 1-D depth or a two-box lumping
suffices for Phase-3 fidelity; and where the transition's **metric-time**
duration (LEXICON §6) even matters — it may be pure **causal time** until Phase 4
hands the clock to inhabitants who care.
