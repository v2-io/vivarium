---
slug: norm-no-depiction-without-referent
type: normative
status: exact
stage: draft
depends:
  - form-core-view-wall
  - norm-probes-before-claims
  - norm-declared-violation-is-not-license
---

# Nothing is depicted that has no world referent

A view may only show what the world actually contains. Visual affordances with no world reality are **strongly discouraged**, and admissible only as a **temporary** measure, under **strong caveat**, and only where they illuminate some *other* phenomenon that is real.

## Formal Expression

1. **The rule.** Every visual element that a viewer could read as a fact about the world — a shape, a motion, a boundary, a colour standing for a quantity — must correspond to something the world's law and store actually contain. Smoothing, filling, interpolating, or extrapolating to make a depiction more legible **manufactures world content**, and a view has no authority to do that ( #form-core-view-wall FE(4): views observe; they do not author).

2. **Why the bar is higher here than ordinary honesty: the viewer is an instrument.** A trained eye scanning an evolving world is one of this project's fastest detectors of *missing physics*, and its reach is different in kind from a probe's. A probe convicts one declared thing someone already thought to ask about ( #norm-probes-before-claims ); a viewer's pattern-matching runs against a lifetime of remembered natural landscapes and fires on violations **nobody has thought to declare yet**. Joseph, 2026-07-24, naming the channel: *"my brain is very highly tuned to be able to notice if there's something in the visual evolution that does not seem 'natural' … it was clear that it was one of the fastest ways to reveal missing physics."*

   That is what a depiction without a referent damages. It does not merely mislead once. It **decalibrates the instrument**: invented content produces false detections, and — worse — once a viewer learns the picture contains invention, true detections stop being trustworthy too. The cost is not a wrong frame; it is the loss of a whole epistemic channel.

3. **The narrow exception, and all three conditions bind.** An unreal affordance is admissible when it is (a) **temporary**, (b) **strongly caveated**, and (c) adopted *because* it makes some genuinely real phenomenon legible that would otherwise be invisible. Failing any one of the three, it is not admissible. This is disclosure, not permission — the same shape as #norm-declared-violation-is-not-license : declaring the affordance does not make it lawful evidence, it makes it a named debt.

4. **For a visual instrument, a caveat that lives only in text is not a strong caveat.** A viewer engaged in pattern-matching is not reading the HUD; that is precisely the faculty being used. So "strong" here means **structurally self-announcing** — the affordance must be evident in the picture itself, not merely disclosed beside it. Vertical relief exaggeration passes: ×20 relief is unmistakable as exaggeration to any eye, and the HUD label is a reminder rather than the whole protection. Interpolated intermediate states fail: they are *designed* to look like continuous physics, so no label can stop the eye from reading them as motion.

5. **The failure mode this exists to forbid, stated concretely.** A system with only endpoint states in the store (today: `erosion-tile`, one distinct time-index — see #form-time-indexed-stage-chains FE(2)) must not be animated by interpolating between them. That would synthesize exactly the smooth evolution a viewer is watching *for*, and it would be a renderer's guess wearing the appearance of a result. Where a system has no interior, the honest depiction is a discontinuity plus a statement that there is nothing between — and the correct repair is in the **builder** (materialize the intermediate stages), never in the view.

6. **Out of bounds for this norm.** It does not forbid abstraction, symbol, or projection: a colour ramp for elevation, a build-state palette, an equal-area projection, a legend, an overlay of declared epistemic state. Those are *representations of real quantities*, and they are the view's proper work. The line is not literal-versus-abstract; it is **referent-versus-no-referent**.

## Epistemic Status

**Max attainable: exact** as a project norm — it is a rule we adopt, and it is falsifiable in application: any shipped view element either has a world referent or does not, and any admitted exception either meets all three conditions of FE(3) or does not.

**Currently `exact`.** Articulated by Joseph 2026-07-24 (`DECISIONS[no-depiction-without-a-world-referent]`, `:by joseph`) and distilling practice the tree already followed without a citable home: the globe's own honesty line refuses cosmetic interpolation between epochs; #form-sphere-continuous-surface-fields records that the cube-seam fix had to be a domain change *"not edge blending or cosmetic clamps"*; #detail-drainage-dependency-planning FE(4) refuses cosmetic clamps on floating mesas. FE(4) — that text-only caveats are weak for visual instruments — is the one clause that is a reading rather than a report, and is the most likely place this norm is wrong. Stage `draft`.

## Discussion

The rule looks like ordinary honesty and is actually a claim about *instrumentation*. Vivarium's other instruments are declared: a probe states what it measures and can fail. The viewer's eye is undeclared, unbounded, and enormously sensitive — and it is the only instrument in the project that can detect an absence nobody has named. Everything that keeps it calibrated is therefore infrastructure, not courtesy.

There is a real cost, and it should be paid with open eyes: honest depiction of an incomplete world is *less legible* than a smoothed one. Unbuilt regions look unbuilt; a system with no interior jumps. That ugliness is the instrument working — it is the world telling the viewer where it is thin, which is the same information a `#gap` row carries in the claim channel.

## Working Notes

- **Audit surface, unowned:** no instrument enumerates the live views' unreal affordances. Relief exaggeration and build-state colour are the two known ones and both are declared; whether others exist has not been checked.
- The three-condition exception has no worked example in the tree yet. The first one adopted should be recorded here, because a norm whose exception is never exercised drifts into an absolute, and an absolute gets quietly broken instead of openly invoked.
- Sibling: #form-core-view-wall owns *authority* (who may author world state); this owns *depiction* (what may be shown). A view can satisfy that wall completely and still violate this norm.
