---
slug: form-manifest-prescribes-vivium
type: formulation
status: exact
stage: draft
depends:
  - def-vivium
  - form-ordinum-governs-flux-web
  - form-nomotheke-registry
  - form-builder-admission
---

# Manifest prescribes the vivium; ordinum describes the kind

World-level conformance uses **two artifacts**, not three: the **ordinum** (descriptive kind-floor — what the *kind* requires) and the **manifest** (per-world prescription — what *this* vivium chose). There is no middle “regula” object until a genuine awkwardness earns a new noun.

## Formal Expression

1. **Ordinum = kind floor.** The ordinum states what a world-*kind* (e.g. Terrestris) requires: phase ladder, charges, promises, defeasances, records. Live floor: `tabularium/terrestris.ordinum.udon`. Claim governance of flux by ladder promises: #form-ordinum-governs-flux-web .
2. **Manifest = this-world prescription.** The **manifest** is the per-vivium surface that pins what *this* instance chose: at minimum **order** (which ordinum / version), **target phase**, **permits** (what is allowed beyond the floor), and **participation** / demand posture for builders and explorers. It is the identity-bearing handle of a citable vivium alongside seed and versions ( #def-vivium , #form-in-vivia-citation ).
3. **No regula artifact (for now).** A separate Regula / Slot object is **not** required for conformance audit. Promise fluxed-quantity + `:kept-by` carries what “slot” reached for; permits live on the manifest. Reach for an in-between noun only if the two-artifact scheme actually strains (`DECISIONS[regula-collapses-to-order-and-manifest]`, `:by us`, decided).
4. **Tracked against nomotheke.** Builder admission and flux web still gate on declared law ( #form-nomotheke-registry , #form-builder-admission ). The manifest does not replace undeclared-is-unlawful; it pins *which* world and *how far* the ladder is driven.
5. **Demand is not identity, and not keyed.** The prescription bucket changes what gets built and in what order, **never what a built artifact contains** — nomos depend on neighbours by complete key, never on "latest available" ( #form-depend-by-key-never-latest ), so demand fields are freely editable mid-build and are deliberately excluded from every key. Convicted incidentally: moving `level` and `erosion_epochs` out of CLI flags and into the manifest produced a rebuild of **0 computed, all hits** — same keys, different home for the request.

   The corollary bounds what may be moved *here*, and it is the reason an arbitrary constant belongs in this bucket rather than on its `NomosDecl`: the manifest records **what this world asked for**, which is true of an arbitrary number; a declaration field would assert it as part of the law, which is not ( #obs-erosion-residual-is-driver-bound Discussion). Relocating an unjustified number into the law layer makes it look principled without making it convictable.

   A second membership test, from the erosion stage chain: **a chain's materialization density is demand exactly when a stage's bytes are stride-independent.** `erosion_stage_stride` (materialize an interior stage every $s$ epochs) qualifies because a stage at `epochs=k` is bit-identical whether built as a chain rung or a one-shot run — convicted by `query::tests::staged_chain_is_bit_identical_to_one_shot` ( #form-time-indexed-stage-chains FE(8) ). The test doubles as the tripwire: if it ever fails, the stride has leaked into artifact content and must move into the key.

6. **Partially built.** `spec.rs` implements identity (`format`, `seed`), label (`name`), and the demand bucket (`order`, `target_phase`, `level`, `frames`, `erosion_epochs`, `erosion_stage_stride`, `water_steps`, `beacon`), with demand keys optional-and-defaulting so a manifest written before the bucket existed still opens as the world it always was. The **first live beacon** (LEXICON `beacon`) is the demand-posture half's opening move: one standing region at its own level with its own response-time-derived erosion demand, swept by the builder after the whole-world floor — erosion-only (fine-level water is blocked on the step-size question, #obs-water-fill-never-settles ), single-beacon, and without per-beacon policy (depth-first vs breadth-first, #detail-builder-daemon ). **Not built:** `permits`, watchpoints, multiple beacons and beacon policy, and any pinning of law/generator versions at Realization. This segment owns the *architecture*, not a claim that every field is executable today.

## Epistemic Status

**Max attainable: exact** for the two-artifact collapse and the refusal to coin regula until earned — Joseph-era `:by us` decision 2026-07-12.

**Currently `exact` for FE(1)–(3)** as project law, and for FE(5)'s not-keyed property (convicted by an all-hits rebuild across a demand move). **`robust-qualitative` / draft for the remainder of FE(2)'s field inventory** — permits and demand posture are designed, not implemented (FE(6)). Vocabulary: LEXICON `manifest` exists; `regula` / `slot` / `permit` words may remain open (deferred, not dead) without reviving the regula *artifact*.

Stage `draft`. Sources: DECISIONS regula-collapse; #detail-regula-design ; #form-ordinum-governs-flux-web .

## Discussion

Ordinum answers “what must any Terrestris-like world eventually satisfy.” Manifest answers “what did *we* pin for *this* run.” Confusing them reintroduces either a third artifact or silent under-keying of vivium identity.

## Working Notes

- OUTLINE §II gap “Manifest / vivium identity” — this segment owns the architecture half; full phase-content segments remain §IV/VII.
- Do not re-plan Slot/Regula builds from old regula-conformance §5 without re-opening the collapse decision.
- After dual-home pointers land, VIVARIA-DEFINITIONS Layer 3 is super-archive-eligible with the rest of that file’s residual peel.
