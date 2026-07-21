---
slug: norm-declaration-must-convict
type: normative
status: exact
stage: draft
depends:
  - scope-segment-canon
---

# A declaration that cannot fail is a wish

Epistemic honesty about world law is enforced when declarations can break the build or a probe — not when they exist only as prose aspiration.

## Formal Expression

1. **Convictability.** A declaration about what a nomos is, conserves, depends on, or promises is real only if something in the executable path can fail when the declaration is false or incomplete (tests, flux audit, store key discipline, probes).
2. **Prose is not a substitute.** Design documents and README slogans do not become true by force of wording. Where the project claims "honesty is enforced in code," that claim is only as large as the mechanized surface.
3. **Nomotheke direction.** Executable declarations (epistemic tags, deps, consumes, promises, assumption anchors) are the intended home of system self-description; an undeclared algorithm must not silently occupy world law.

## Epistemic Status

**Max attainable: exact** as a normative standard for the project. Self-applied: the project has already recorded that "epistemic honesty enforced in code" was true of some surfaces and a wish for others (`doc/theory/discretisation-and-information.md` and related audit notes). Stage `draft`. Do not read this segment as asserting that *all* declarations are already mechanized — only that unmechanized declarations must not be sold as enforced.

## Discussion

This is the specification dual of #norm-probes-before-claims: probes convict *behavior*; convictable declarations convict *self-description*. Together they stop plausibility from wearing verification's clothes.

## Working Notes

- FORMAT still lists an open question whether segments need a `mechanism:` field naming the convicting executable; this norm states the requirement without closing that schema question.
