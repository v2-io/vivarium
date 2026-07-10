# vivarium — superseded / deprecated ledger

*A running, lightweight record of terms, doc-sections, and decisions that have been **retired, replaced, or corrected** — so the deprecation is durable (nobody re-adopts a dropped term or trusts a stale section) without a full tidy-up pass.
**Append as you go.** Newest first. This is not a changelog of *additions* (that is the git log + ORIENTATION addenda); it is specifically the *"do not use this anymore, use that"* list. Format: **~~old~~ → new** · *when* · *why / where it moved*.*

> [!note]
> This ledger was started 2026-07-09 during the lexicon-consolidation session; it
> back-fills known supersessions from recent sessions and should be extended
> whenever future work retires or replaces something. When an *entire* doc or
> large section is superseded, note it here **and** add a pointer at the top of
> the stale doc/section itself.

## Terms

- **~~Providence~~ → Lawful-steering** · 2026-07-09 (Joseph's naming call) · The third mutation mechanism (an exo agent choosing a fated-noise realization,
  lawfully and trace-free). "Providence" named the *actor's attributes*
  (providential/divine); "lawful-steering" names *what happens* — the intended focus. *Reconcile:* `taxonomy-formalization-spike.md` §9.2 and the
  `participation-taxonomy-lexicon.md` / `forward-work-rhythms-scale-coupling.md`
  memories still say "Providence" (reasoning-trail; canonical term is now lawful-steering). LEXICON §7.5.
- **~~Incubation / Fine-tune-setup~~ → Brooding** · 2026-07-09 (Joseph's call) ·
  The process that produces the Backstory (Steer / Search). "Brooding" fits the vivarium frame and echoes `PHASES.md` Phase-1 ("brooded upon"). LEXICON §4.
- **~~"Fate" (as a defined noun)~~ → *fated noise* + KRNG/DRNG** · 2026-07-09
  (Joseph's call) · The coordinate-hashed determinism discipline is **fated noise**; the generator is a **KRNG** (keyed) / **DRNG** (deterministic) RNG.
  "Fate" is deliberately *not* adopted as a standalone defined term. LEXICON §3.
- **~~completion gate~~ → *Realized* ⟂ *Lawful* (two orthogonal properties)** ·
  2026-07-06 · The single "completion" moment fused two independent things
  (law-frozen vs law-coherent). Kept only as a loose tactical word;
  **retired from the ontology.** Formally forced by BREAK-2 (completion is an asymptote $\varepsilon \to 0$, not an event). LEXICON §7.2; `vivium-operational-workflow.md`.
- **~~Numen~~ → (no term)** · 2026-07-06 · Smuggled an authority *hierarchy* onto the agent-space; the earned distinctions among exo agents are **access + mode**,
  not authority. The agent ontology imposes **no** default structure. Revisitable only if a genuine authority-distinction ever becomes load-bearing. LEXICON §7 framing principle.
- **~~LTM (Lower Threshold of Motion, as *our* term)~~ → sphere of perception +
  perceptual horizon** · 2026-07-06 · Rejected: collides with Long-Term-Memory,
  is visual-only, and borrows a narrow (maybe-unstudied) term for a general edge. LEXICON §8.
- **~~"backstory = the frozen pre-target phases"~~ → backstory = the
  *scenario-specific realized situation*** · corrected 2026-07-09 · A recurring wrong guess (mine included). The frozen pre-target world is the **Target world**
  (§4 layer 2); **Backstory** is the specific state produced *on top of it* by
  **Incubation**. LEXICON §4.
- **~~"checkpoint" (for phase boundaries)~~ → phase-transition / phase-gate;
  cache sense → memo; verification slice → Record** · 2026-07-04 · "checkpoint"
  ceded to ASF's agent-snapshot sense. `PHASES.md`, LEXICON §1. *(Pending: four
  `DESIGN-REDUX` sites still say "checkpoint" in a runtime-durability sense —
  decide memo vs a distinct persist-boundary name; LEXICON §5 collision ledger.)*

## Doc sections / files

- **~~ARCHITECTURE.md v0.1 (software-architecture register)~~ → v0.2 → v0.3
  (one principle, three axes)** · 2026-07-09 → 2026-07-10 · v0.1 led with the query-graph/keys/store *implementation* and buried the multiscale R/L/closure operator algebra; v0.2 re-centered on it; **v0.3** (2026-07-10) consolidated to
  *one principle (represent by consequence) on three axes* — substrate machinery /
  phase-freeze developmental ladder / use-case-as-fidelity-contract, with AAT-calibration one privileged use-case, not the telos. Grounded in the four multiscale primaries read directly (Berger–Oliger, Gear–Wells, HMM,
  equation-free). *(Still a v0.x draft; §7 AAT bridge is stance.)*
- **~~`framework-to-status-quo.md`: drainage-cone as "center of gravity"~~ →
  drainage cone = one worked instance of consumer-dependent restriction** ·
  2026-07-09 · Corrected per Joseph — multiscale/heterogeneous methods are the general principle; the drainage cone is one instance among many-many systems to come. ARCHITECTURE.md §6.
- **~~LEXICON §4 stubs ("World-as-artifact — Ⓝ")~~ → §4 rewritten (world-artifact lifecycle)** · 2026-07-09 · The 07-03 stubs predated and never captured the
  07-06 vivium-dialog decisions. LEXICON §4 (+ new §7/§8).
- **~~`universal-biological-rhythms.md` (as a rigor source)~~ → superseded-for-
  rigor by `est-tiw-dossier.md`** · 2026-07-06 · Kept for provenance with a correction header (2 Hz/3 Hz conflation, Wittmann miscite, fringe sections);
  the dossier is the verified/tiered successor.
- **~~DESIGN.md §"build the agent seam first, while ugly"~~ → retracted (fun-led exploration guarded by three early decisions)** · pre-2026-07-03 · Already retracted *in* DESIGN.md §Purpose; noted here for completeness.
- **~~ORIENTATION.md "deluge ~2-3 min"~~ → no longer true (~2 h cold fill after the conservation fixes)** · 2026-07-04 · Promotes the analytic hydrological init from elegant to necessary; noted in ORIENTATION's 07-03→07-04 addendum.
- **`spikes/slabs`** → superseded as the SOTA 3-D view by **`spikes/worldview`**;
  kept as reference until the old core's remaining physics is fully ported
  (ORIENTATION "The code").

## Watch-list (superseded-in-practice, not yet cleaned)

- **The whole `spikes/worldview` runtime** is a physics testbench, **not** the target framework (one fixed patch; re-seeds from raw prior on movement;
  non-composable tiles). Not "deprecated" — it is the proving ground — but its
  *architecture* is superseded by the store/recipe/spine design
  (`framework-to-status-quo.md`, ARCHITECTURE.md §5–6). Track so nobody mistakes the testbench for the frame.
- **`vivarium-core`** (flat `i32` patch) is the *physics donor* being ported into
  `vivarium-world`; superseded as the live frame but retained as the algorithm reference until the port is proven (ORIENTATION, `world-model-foundation.md`).
