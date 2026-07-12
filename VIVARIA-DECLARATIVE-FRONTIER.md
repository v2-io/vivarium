# VIVARIA — the declarative frontier (spike 2, 2026-07-12)

*Companion to `VIVARIA-DEFINITIONS.md`. That one mapped the scaffold; this one pushes on the real question — **how much of world-building and world-running can be settled from declarations, before (or entirely without) running the simulation** — and corrects a mistake I made in spike 1.*

**The correction that reframes everything.** In spike 1 I filed "does the predicate actually pass?" and "is the seam reconciliation sound?" under *inherently runtime*. That was too weak. A capable **agentic auditor** reading the kernel source against the declared claim can return a graded verdict — *supported / unlikely / refuted* with reasoning — **statically, without executing anything.** So verification is not a runtime monolith; it's a set of **declared check-modes**, most of which are static (agentic or formal), with numeric probes reserved for the residue an agent genuinely cannot judge from code (emergence, chaos-sensitive magnitudes, opaque model outputs). This widens the pre-run auditable surface enormously, and it's the spine of this pass.

**The frame:** think of the whole world as a **queryable graph of declarations**. The measure of "how declarative" is then just *which questions you can answer from the graph* — with an agent auditor as one of the resolvers. Below: the questions, the udon that makes each answerable, the one gap I close, the residue that genuinely resists, and the attempts I tried and threw away.

---

## Verification is a declared, mostly-static, multi-modal thing

A predicate no longer bottoms out at "run it and see." It declares **how** it's checked, carries its **last verdict**, and names its **verifier** — so "kept" becomes graded and, for most predicates, *established before any kernel runs*.

```udon
|promise[eroded-surface].state
  :quantity "eroded surface (m)"
  :statistic conserved-total            ; <-- the spike-1 GAP, now closed (see below)
  :exactness approx :error-model "sub-ULP deadband at 6km datum, Kahan-compensated"
  |predicate mass out at tile outlets equals incision minus deposition
    :check agentic                      ; an auditor reads the kernel vs this claim
    :verdict supported :confidence high
    :by "opus-auditor@2026-07-12"
    :note "outlet ledger + Kahan accumulation are present in Fluvial::erode; the claim is structurally backed. No probe needed to reach 'likely'."
  |predicate no cell loses increments below ULP at the datum
    :check probe :probe conservation_test :verdict pass    ; numeric — an agent can't eyeball ULP behaviour
  |predicate the eroded field is drainage-coherent (no closed basins off-outlet)
    :check agentic :verdict unlikely :confidence med
    :note "Priority-Flood fills depressions, but the -tile edge policy can strand basins at seams; flagged, would need seam_ridge to confirm."
```

```
--> Rust / harness:
    enum Check { Agentic, Probe(ProbeId), Formal(Tool), Golden }
    struct Verdict { rung: {supported|unlikely|refuted|pass|fail|unrun},
                     confidence, by: VerifierId, note: String, at: Stamp }
    // "kept" = the fold over a promise's predicates' verdicts, weighted by check-mode.
    // An `agentic:supported` predicate is *established pre-run*; a `probe:unrun` is not.
```

The maturity ladder from spike 1 (named→specified→claimed→kept) gains a middle rung it was missing: **assessed** — the claim has a verdict from a static check (agentic/formal) even though no numeric probe has run. Most promises can reach *assessed* the moment the kernel exists, which is exactly the "auditors judge before running" Joseph pointed at.

---

## The declared tier is itself falsifiable (self-certification, checked)

Spike 1 took a nomos's `:physics med` on faith. It shouldn't be. An agentic auditor reads the kernel and returns an **assessed** tier beside the **declared** one — and the discrepancy is the finding.

```udon
|nomos[erosion-tile]
  :physics med                          ; DECLARED (self-asserted by the author)
  :physics-audited med :confidence high ; ASSESSED (agent read the kernel)
    :by "opus-auditor@2026-07-12"
    :note "stream-power (Shields tau_c) + Davy-Lague + talus + creep all present and correctly formed; the physics tier is honest. The weak point is NOT the physics — it's the hardcoded edge policy, which undercuts the *composability* claim, not the tier."
  :relation "mechanistic-causal"
  :relation-audited "mechanistic-causal ON A STAND-IN SUBSTRATE"    ; sharper than declared
```

```udon
; the failure mode this catches — an overclaim, surfaced by the audit:
|nomos[some-tuned-thing]
  :physics med
  :physics-audited low :confidence high
    :note "declares med, but the kernel is a fitted lookup with no conserved quantity; the med tier launders a tuning. DECLARED > AUDITED => overclaim flagged."
```

```
--> fn audit_tiers(nomos, kernel_src) -> {declared, assessed, discrepancy}
    // discrepancy != none  =>  the honesty column shows a * and the reason.
```

This is the strongest single move in the pass: **it makes the epistemic tags — the whole basis of the fidelity pyramid — auditable rather than trusted.** The declared tier is a claim; the agent audit is what could convict it; the two side by side is the honest state. It's the promise/predicate discipline turned back on the nomos's *self-description*.

---

## The gap I close: fluxed quantities carry their sufficient-statistic contract

Spike 1's web declared *what* flows (`"precipitation (m/yr)"`) and its conservation, but not *which statistic it guarantees* — so cross-scale composition couldn't be audited. Fixed: every `produces` declares the statistic(s) it guarantees + exactness; every `consumes` declares the statistic it needs; the match is a graph check.

```udon
|nomos[erosion-tile]
  :consumes [
    { quantity "precipitation (m/yr)"  needs mean        at-least L19 }   ; erosion wants area-mean rain
    { quantity "surface elevation (m)" needs center-sample }
  ]
  |produces "sediment flux (kg/m2/yr)" :statistic conserved-total :exactness approx

|nomos[water-cycle]
  |produces "precipitation (m/yr)" :statistic mean :exactness approx :error-model "sub-grid orographic unresolved"
```

```
--> fn audit_flux_match(consumer, producer) -> Ok | Err(StatMismatch)
    // erosion needs `mean`; water-cycle guarantees `mean` -> OK.
    // If erosion needed `conserved-total` and water-cycle only guaranteed `max`,
    //   the edge is present but UNSOUND -> the exact "store the wrong statistic,
    //   silently corrupt the macro" failure (DESIGN-REDUX §5), caught pre-run.
```

Now the web audits not just *A depends on B* but *A's need for B is cross-scale-sound* — the representativity check that the whole seam discipline exists to enforce, made a lookup.

---

## World-running: a study is auditable before it runs

The part I called weakest in spike 1. An in-vivia experiment is a declarative artifact, and — with agentic audit — its **validity can be assessed before execution**: does the world's fidelity actually support the claim's axes?

```udon
|study[does-the-fbm-spine-drown-the-world]
  :hypothesis "the current spine yields >40% land — wrong for early Earth"
  :world first-light@<hash>              ; the exact vivium (manifest+versions+seed)
  :analysis hypsometry-of-region
  |predicate land-fraction < 0.15
    :check agentic
    :verdict ill-posed :confidence high
    :note "The spine declares physics:none / earth-fidelity:none. A land-fraction claim is an A-axis (Earth-history) claim; the world's A-fidelity is 'none'. So NO land-fraction result from this world can support or refute an Earth claim — the study is ill-posed before it runs. (What it CAN show: that the spine's *own* output is 41.5%, a fact about the algorithm, not about Earth.)"
```

```
--> fn assess_study(study, world) -> Verdict
    // reads: the claim's axes (A/B/C) vs the world's derived fidelity on those axes
    //        (the cone-fold) -> "can this world even support this claim?"
    // A flaw later found in any nomos in the cone re-flags the study by the
    //   store's own invalidation — the spike-1 result — but now the *pre-run*
    //   validity check is agentic and static.
```

This is the real answer to "world-running declaratively": you cannot pre-declare what an agent or user will *do*, but you can **pre-audit whether a run could possibly mean what it claims** — and after the run, the keyed/logged determinism makes it exactly reproducible. Structure and *validity* before; content after; nothing un-auditable in between.

---

## Things that fall out for free (queries over the declared graph)

Once it's all data, these need no new machinery — they're graph queries, some resolved by an agent:

```udon
; the build plan is DERIVED, not authored — topological sort of consumes/produces:
;   spine-tile -> erosion-tile -> surface-water            (target-phase abyssal, today)
;   [atmosphere-reservoir -> water-cycle] -> {erosion, surface-water}   (when built)
; --> fn build_plan(order, manifest) -> Vec<NomosRef>   // a query, not a config file
```

- *"What's the weakest link in erosion's result?"* → the cone-fold. *(built-ish)*
- *"Which promises are un-checkable?"* → promises with no predicate. *(query)*
- *"Which nomoi overclaim their tier?"* → declared > audited. *(agentic)*
- *"If I add nomos X, what does it then require?"* → transitive consumes-closure. *(query — the anti-whim planner)*
- *"Can study S support its claim?"* → assess_study. *(agentic)*
- *"What order do I build in?"* → topological sort. *(query)*
- *"Is this coupling cross-scale sound?"* → flux statistic match. *(query)*

The declarative surface is exactly the union of these. Spike 1 said "world-building composition is auditable pre-run"; spike 2's addition is that **verification, self-certification, and even study-validity join that set once agentic static audit is a declared check-mode.**

---

## The honest residue — what genuinely resists even a good static auditor

Not everything. After all the above, three things remain that need *running* (or are simply opaque), and I want them named sharply so we don't pretend otherwise:

1. **Emergence at scale.** "Does the food web stabilize?" / "Do gliders appear in the CA?" / "Does drainage self-organize into the right channel statistics?" — an auditor reading kernel source *cannot* determine the emergent macro-behaviour of a nonlinear system from the local rules. This is the deepest one, and it's the *point* of a simulation: you run it to discover what the rules do. Declaration + audit tells you the pieces are sound; only running tells you what they *become*.
2. **Chaos-sensitive magnitudes.** An agent can say "this converges" (structure) but not "it converges to 3.47 km of relief" (a sensitive quantitative outcome). Exact values in sensitive regions need the run.
3. **The opaque content — LLM agents, user edits.** The slow-layer's actual decision is not readable from anything. Structure declarable, wrapping auditable, decision opaque — recorded after, never predicted.

Everything else — the couplings, the conservation, the tiers, the requisite chains, the seam soundness, the study validity — I now believe is **statically settleable**, agent-in-the-loop, before the simulation runs. That's a much larger fraction than spike 1 conceded, and Joseph's correction is why.

---

## Attempts I tried and threw away (the failure record)

*(Joseph said retry various things; here's what didn't survive, so no one re-tries them cold.)*

- **A DSL to declare the kernel's numerics themselves.** Tried: push the arithmetic into udon too, so even the kernel is data. Rejected: this just reinvents a programming language, worse than Rust, and the kernel *is* the correct imperative boundary. Declaring the algorithm = writing it in a poorer notation. The win is declaring everything *about* the kernel, not the kernel.
- **Making "kept" fully static (drop probes entirely).** Rejected: emergence and chaos-sensitive magnitudes (residue 1–2) genuinely need running; an all-static audit would launder those into false confidence. Probes stay — but *scoped to what agents can't judge*, not as the default.
- **A per-coupling "policy" object separate from the nomoi.** Tried: declare interpolation/averaging/exchange-cadence as standalone coupling artifacts. Rejected for now: it fragments the flux contract across two places; better to hang the policy on the `consumes`/`produces` edge itself (`needs mean at-least L19`) so the coupling stays one matched pair. Revisit only if a coupling ever needs config no edge can carry.
- **Declaring `posture` (what results may claim) as a first-class field.** Rejected (carried over from spike 1): it's derivable from the cone-fold; "the fold wins." A field that only ever restates a computation is ceremony. Killed, not homed.
- **A global "confidence" scalar per world.** Tried: one number for "how trustworthy is this world." Rejected: it collapses independent axes (A/B/C, per-region, per-aspect) into a lie. The honest object is the per-promise, per-cone fold, not a scalar. (Same mistake as a single "physics tier" for a whole world — the thing the four axes exist to prevent.)

*Two of these failures are the same shape as a real discipline: don't invent a noun/artifact that only restates a computation (posture, global-confidence), and don't move the imperative boundary (kernel-DSL). The declarative frontier is wide, but it has an edge, and pretending past it is the one dishonesty this whole apparatus exists to prevent.*

---

*Spike status: bolder than spike 1, deliberately. The load-bearing new claims — agentic static audit as a declared check-mode, declared-vs-audited tier, flux statistic contracts, pre-run study validity — are proposals I'd defend, but they're proposals; the residue (emergence, chaos, opacity) is where I'm confident the frontier actually ends. If a mechanism here reads as clever-but-hollow, it's the next thing to cut — same rule as spike 1.*
