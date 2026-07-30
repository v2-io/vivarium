---
slug: form-fidelity-ladder
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-fidelity-invariant
  - form-complete-content-addressed-key
  - def-nomos
  - form-reductionist-fallback-cases
---

# Fidelity ladder: climb to discover, descend to surrogate

Each aspect may occupy a **ladder of models** from crude to high-fidelity. Low rungs are honest when they honour the sufficient statistics consumers need with known bounded error — not a lie dressed as placeholder. The ladder runs **both ways**.

**And each band of detail must enter at a rung that runs.** Structure introduced at a level finer than any executed rung never participates in an action, so it constrains nothing, and it carries no key — there is no run at that level to attribute it to. This is the governing statement of FE(7)–(11), which are its consequences at the surface: *"the noise of the right frequency isn't getting onto the stack at the right rung — it always needs to be present at the level the action is happening, not added at a finer level where there is no action recorded"* (Joseph, 2026-07-29). Climbing the ladder (FE(2)) **is** the mechanism by which a finer band earns the right to act; adding the band without climbing is the defect, and it is a defect in injection order, not in any one consumer.

**Causality alone dictates this** (Joseph, 2026-07-29), and that is a stronger footing than any appeal to honest depiction: every feature of a world must have a cause *within* that world's history. Structure appearing at a level no rung ran has none. This also collapses the clause's two symptoms into one fact — *no action* and *no key* are the same observation, because a complete content-addressed key ( #form-complete-content-addressed-key ) **is** a value's causal provenance, so content that cannot be keyed is content that was not caused. The key discipline is therefore not bookkeeping that happens to catch this; it is the mechanization of causal closure, and this clause is what it looks like when applied to the frequency axis.

## Formal Expression

1. **Rungs.** An aspect has a ladder of models (crude macro-statistic match → higher physics). Occupying a low rung is correct when #form-fidelity-invariant holds for its consumers.
2. **Climb.** Stepwise emergence discovers behaviour; regime probes validate ( #norm-regime-probes , case C of #form-reductionist-fallback-cases ).
3. **Descend.** Once patterns are characterized and probe-validated, **descend** to a tight procedural surrogate that reproduces the discovered statistics; keep the expensive rung as calibrator (re-run on nomos change).
4. **Key identity.** Model identity and version are part of the content-addressed key ( #form-complete-content-addressed-key , #def-nomos ). Swapping a rung invalidates that nomos and its dependents only.
5. **Execution class.** Systems declare execution class (batch-deep / relaxation / procedural-tight) so coupling and cost are honest — carried as `NomosDecl::execution` since 2026-07-24 ( #form-kernel-imperative-boundary FE(5)).
6. **Scaffolding has a demolition date.** Explicit state that parameterizes what a finer rung would emerge (armor field standing in for selective transport of real grain sizes; colmation for fines percolation) is declared as scaffolding: when the finer rung lands and the behaviour re-emerges, the explicit field retires. Keeping both is a fork.

7. **There is no decorative relief: fine detail blocks water.** A metre of relief is a dam, a sill, or a pit — it decides where water stands and where it goes. So relief cannot be added to a picture "for appearance" while being withheld from the physics, because appearance and physics are the *same field*. Any surface a view draws is a claim about drainage, whether or not it is offered as one. Measured consequence of drawing an unreworked one: standing water on the uncarved prior is **8.5×** its value on the same window at the same level after carving, because fluvial reworking is precisely what destroys closed basins ( #obs-connectivity-fills-the-basins-the-threshold-drained FE(5)).

   This retires the appearance defence rather than qualifying it. The band-limited prior *is* declared law, and that is not the issue: at a level the kernel has not run, the prior's detail is an **initial condition**, and drawing an initial condition as a surface asserts a drainage structure the world does not have.

8. **A view renders the physics; it does not synthesize world content.** Descending the ladder to *draw* a level finer than the carve — bilinear over the coarse carve with the fine prior's detail re-added — puts a world layer in the viewer that no nomos produced and no key covers, which is a second, unaddressable surface competing with the store's ( #form-core-view-wall , #norm-no-depiction-without-referent ). Many views of the rendered physics are welcome and are the point of the instrument. A view that adds terms is not a view.

   The honest ways to answer "I want finer detail": carve at that level (`vivarium build` a beacon — the demand posture exists for exactly this), or draw the carve you have and let the absence show. **Joseph, 2026-07-29:** *"No phantom holographic world layers. Different views of the rendered physics: great. Pseudophysics hiding in the viewer — double plus ungood."*

9. **Corollary, weaker than FE(8) and still binding:** no derived physical quantity is computed on a mixed-tier surface, since such a figure measures the prior rather than the world. The tier census is not sufficient cover — a fraction in a text block does not scope a full-field number.

10. **The third path is a sub-grid closure, and it belongs to the physics.** Unresolved relief *should* affect coarse flow — leakage across a rough sill, seepage, wind-driven loss, sub-grid storage — and representing that as a **closure term the coarse kernel carries** is legitimate, keyable and testable, in a way that re-adding the relief afterward is not. The distinction is the ordering: a closure lets the unresolved detail act *on* the flow; re-addition makes the detail arrive *after* the flow, where water cannot see it. **Joseph, 2026-07-29:** *"having water flow without access to the high-frequency and then plopping the high frequency back on like it is invisible to water is silly."* An effective-transmissivity term already exists as a live object on the router side ( #obs-routing-curl-spiral FE(6)(c)), so this is a rung to build rather than a concept to invent — with one measured caution: coarse-only *pointwise* prediction of drainage structure is a no-go at $R^2 \le 0.40$ on the kernel it was measured against ( #obs-coarse-only-closure-nogo , kernel-era-scoped). A leakage/storage closure is a different quantity than that spike's trunk prediction, so the no-go bounds the ambition rather than forbidding the term.

    **And the closure inherits FE(6)'s demolition date, non-negotiably.** Today's finest rung is a statement about available compute, not about the world, so a closure written as though that rung were the bottom becomes **false physics the moment a finer rung is affordable** — it does not degrade gracefully, it turns into a lie sitting at a level that now runs (Joseph, 2026-07-29). A sub-grid term is therefore declared as standing in for a named finer rung and retires when that rung runs; a closure without a stated demolition condition is scaffolding pretending to be law.

11. **This generalizes to every elemental boundary, and the surface is only the first one.** The pattern — run a process at one resolution, add finer structure afterward — lies to any process whose behaviour is set at an *interface*, and interfaces are exactly where unresolved structure matters most. A sill blocks water; a density boundary blocks gas mixing; bed roughness sets what strata lay down and where. So any field that acquires a detail increment at read time is invisible to the process it should be constraining: multi-layer gas mixing up the column and submarine strata refinement inherit this clause as written, and neither has been built yet — which is the cheap moment to get the ordering right (Joseph, 2026-07-29).

## Epistemic Status

**Currently `robust-qualitative`** as design stance (DESIGN-REDUX §12). FE(7)–(8) are newer and narrower. The 8.5× is one window at one level and is the load-bearing measurement in both; what is *argued* rather than measured is the viewer claim — that a smooth surface self-announces its emptiness where textured detail does not — and no probe here convicts it. Stage `draft`.

## Discussion

FE(7) exists because the seeding use and the depicting use share one implementation and diverged in what they license without anyone deciding they should. That is the ordinary way scaffolding outlives its reason: the mechanism was correct where it was built, and the second caller inherited its correctness along with its code.

An earlier draft of FE(7) called the depiction unprincipled outright. That overreached, and the steelman is worth keeping visible so it is not lost to the next pass: the prior is declared law, not noise, and refusing to draw it asserts a flatness that is *also* false. What actually decides the case is not provenance but bias — the missing operator is exactly the one that shapes drainage, so the surface is far more trustworthy about texture than about basins. A claim that re-added detail is "invented" is the wrong objection and can be answered; the bias measurement cannot.

## Working Notes

- Specimens (fBm prior, mineral totals, single hardness) stay teaching in REDUX.
- GPU/backend-as-rung: water-parallelism plan; not claimed built.
- **Settled 2026-07-29 (`DECISIONS[a-view-renders-the-physics-and-adds-no-terms]`):** re-added detail keeps no role in a view. What remains open is the *replacement* — smooth interpolation with the absence visible, versus refusing to draw finer than the carve at all — and the FE(10) closure, which nobody has specified or measured.
- **Owed under FE(8)–(9):** the depression / standing-water reader in the explorer computes on the drawn surface and currently reports across tiers. Either it refuses above the carve level or the figure carries its tier composition. The paint's own fine-view speckle is the visible instance.
- **Why FE(7) pays forward rather than costing** (Joseph, 2026-07-29, on foliage, hippos and beavers): the spill field never asks *what* raised the sill. A beaver dam, a landslide, a lava tongue, foliage-trapped sediment and a wallowed-shut channel all enter the fill identically, because it is a function of the surface alone. Every obstruction kept in the physics is handled by machinery that already exists; an obstruction drawn in the viewer would need a parallel hydrology and would still be false. The agent-side of this is gated — #scope-agent-seam-constraints , ETHICS — but the hydrological seam is already the right shape for it.
