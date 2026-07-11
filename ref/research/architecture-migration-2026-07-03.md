# Architecture migration review (2026-07-03, fresh-eyes architect + session assessment)

> [!note]
> **Terminology (2026-07-10):** “recipe” throughout this dated document = **nomos** (pl. *nomoi*) — the term settled after this was written (LEXICON §2). Kept verbatim as history.


*A Plan-agent with a clean context read Checkpoints.md, all DESIGN-*, ORIENTATION,
both written plans, and the code, and reported; the session agent (full two-day
context) assessed. Verdict: **the physics crate is already the right shape; the
migration list is short and the do-NOT-build list is longer.***

## Migrate now (small verified landings)
1. **Colocate recipe versions with their physics** — `pub const ALGO_VERSION` in
   `water.rs` / `erosion.rs` / `gen.rs` (the prior is a keyed input too); spike
   composes its key from them. Plus a **recipe-canary test** per system: tiny
   fixed sim, golden output hash beside the const — any physics change fails the
   canary; the fix is one commit updating golden + version together (§12's
   key-completeness, mechanized; four manual bumps in one day was the pain).
   *Session note: introduce canaries AFTER the two open water anomalies settle,
   or accept golden churn during active debugging.*
2. **Orchestration crate** (name = Joseph's call; not "epoch") — std-only, deps
   on vivarium-world only; the home for what is neither physics nor view:
   - A: `fill_cache` → `checkpoint` module (keep blob-per-checkpoint; store the
     PLAINTEXT recipe key beside the hashed filename; structured `Recipe` type).
   - B: settle pipeline moves over (TierMsg/WaterMsg are sim vitals, they move);
     worldview shrinks to receivers.
   - C: telescope; then a headless `genesis run` bin — the first peer view
     DESIGN.md promised (and warmers stop competing for the GPU with play).
   *Session note on sequencing: do A early (survives the rewrite); hold B/C
   loosely — the analytic-init builder should own the pipeline shape; moving the
   deluge machinery only to delete it is churn.*
3. **Named couplers, no trait** — the bed write-back (dup'd in settle+telescope)
   becomes `erosion::absorb_bed(&mut ErodedRegion, &WaterSim)` with a
   mean-conservation test, beside `pin_block_means`. §12: flux interfaces stay
   per-quantity; a System trait at n=2 implementors is the monolithic blob.

## Soon
- Analytic hydrological init = the first NAMED phase in the orchestration crate;
  its checkpoint carries a phase identity; near-stationarity probe is its gate.
- Checkpoint round-trip probe (resume vs run-through must agree — promote the
  two-leg cache test from anecdote to instrument).
- Climate (third system) = the rule-of-three moment to revisit shared shapes.
- Store → per-world save dir only when user mutations exist.

## Deliberately NOT yet (docs' own rationale)
Full §13 store (TENTATIVE twice, no mutations to log) · System/tier trait (n=2)
· Phase enum / gate machinery (phases enter code one at a time as runnable
recipes) · lazy query-graph engine (spine+cones+cache already approximates §11)
· per-phase crates (regimes are recipe choices, not module trees) · serde (the
dumb versioned format is fine; consider pure to_bytes/from_bytes beside each
ALGO_VERSION) · GPU/rayon (already planned, placement decided).

## EXECUTION CLASSES (Joseph, same evening — the principle that ties it together)
Each system-in-a-phase declares its compute class in the Recipe:
- **batch-deep / checkpoint-bound**: run long, preemptively, RARELY (re-trigger =
  recipe version change); arbitrarily slow because checkpoints amortize to zero.
  Abyssal geology: tectonics, banding, igneous bodies, uplift.
- **relaxation**: solved/settled per checkpoint, then locally live (water).
- **procedural-tight / call-site-bound**: evaluated constantly, must be fast,
  closed-form or surrogate (insolation is the exemplar; weather aspires to it).

**THE LADDER RUNS BOTH WAYS** (recorded also in DESIGN-REDUX §12): climb to
DISCOVER a system's behaviour (stepwise, expensive, emergent); once its patterns
are characterized and probe-validated, DESCEND to a tight procedural surrogate
reproducing the discovered statistics (§5 sufficient-statistics agreement is the
honesty gate), keeping the expensive rung as reference/calibrator, re-run when
its recipe changes. The batch run is the surrogate's calibration data; the
probes verify the surrogate. (This is HMM upscaling as compute economics.)
The 2026-07-03 symptoms of NOT having this: four cache invalidations in a day,
the eternal-fill UX, warmer choreography by hand.
