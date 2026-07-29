---
slug: norm-physics-not-knobs
type: normative
status: exact
stage: draft
depends: []
---

# Physics, not knobs

When a symptom appears, write the missing physical term — it is faster to code than the magic-knob proxy, about the same amount of code, and terminal; the knob is not.

## Formal Expression

1. **The rule.** Before reaching for a threshold, clamp, or multiplier, ask *what is the missing physical term or law?* The real term is the default fix; the knob needs a reason.
2. **The cost claim (measured, one evening, 2026-07-02).** Every real-physics fix was small and **terminal**: hillslope creep (~20 lines) ended the sawtooth saga; slope-in-capacity (8 lines) ended gate-fiddling; ocean-as-ordinary-ground deleted a special case; critical shear stress $\tau = \rho g d S$ replaced two ad-hoc gates with one physical parameter that already had a slot in the material property set. Every magic knob (render cutoffs, depth gates, per-step caps, an 0.05 m "atmosphere") cost a full report-symptom → tweak → re-report loop. **The knobs were not cheaper — they only felt cheaper.**
3. **When a knob is truly needed**, prefer one with physical meaning and units ($\tau_c$ in Pa, $\kappa$ in m²/epoch) — it usually already has a home in the material property interface ( #form-material-property-interface) — and it enters the `ASSUMPTIONS.md` ledger like any other unprincipled value: an unprincipled value is not the sin; an unaccounted one is.

## Epistemic Status

**Max attainable: exact** as process law. **Currently `exact`**: Joseph's direction, stated with force 2026-07-02 — *"it only takes a few moments to write in the correct physics rather than go for a quick hack — it ends up being faster to code, about the same amount of code, and saves us from endless back-and-forths where you tweak magical knobs because you're afraid to do the physics"* — and the FE(2) score is that evening's measured record. Segment packaging claude, landed under Joseph's 2026-07-29 delegated grant (`DECISIONS[candidates-adoption-calls-dispositioned]`).

Stage `draft`.

## Discussion

Sibling of #disc-algorithms-disguise-physics, cutting the other way: that segment says do not import a paper's costume in place of its physical claim; this one says do not substitute a knob for the physical claim you already know. Both are the same discipline — the physical term is the unit of truth — applied at import and at repair respectively. The fear that physics is slower is the load-bearing error; FE(2) is the evidence against it.

## Working Notes

- The knob inventory this norm bites on is `ASSUMPTIONS.md` (`tuned` / `arbitrary` rows); each such row's "principling path" column is this norm applied forward.
