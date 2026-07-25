---
slug: form-time-indexed-stage-chains
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-complete-content-addressed-key
  - form-store-as-save
  - form-depend-by-key-never-latest
  - form-temporal-lod-regimes
  - disc-unlawfulness-budget
  - detail-seam-precedents
  - norm-declaration-must-convict
  - form-fidelity-invariant
  - detail-phenomena-systems-map
---

# Time-indexed stage chains: a settle sequence is addressable memos, each carrying its own $\varepsilon$

A world does not reach a state, it walks to one. That walk is a chain of keyed nomos invocations, each carrying a **time-index in its complete key** and recording the **convergence-$\varepsilon$** it actually reached — so a stage is a thing you can point at, resume from, watch, and replay, rather than a number of iterations someone chose.

## Formal Expression

1. **The unit.** A settle sequence is not one long computation with a step count. It is a **chain of stages**, each an independently keyed memo whose complete key includes a **time-index** ( #form-complete-content-addressed-key ; the time component named at #form-three-scoped-runtime FE(3)). Stage $n{+}1$ depends on stage $n$ **by key**, never by "the latest state on disk" ( #form-depend-by-key-never-latest ).

2. **Addressability is the point.** With a time-index in the key, *"the beginning of the third fluvial stage"* is a **key**, not a description. That is what a beacon can aim at, what a watchpoint can name, and what a builder can resume from. Without it, a build has no interior: it is opaque between "started" and "finished," and the only addressable states are the endpoints.

3. **Each stage records the $\varepsilon$ it reached.** A stage's memo carries not only its result but the convergence residual it achieved and the criterion it was run against. That number is not diagnostics — it is **the unLawfulness budget as data** ( #disc-unlawfulness-budget FE(3): every Realized freeze injects bounded-but-nonzero residual drift, and downstream honesty gates on the *cumulative* budget). An $\varepsilon$ that is never recorded is an unLawfulness budget asserted to be zero.

4. **A criterion replaces an iteration count — and the count becomes a measurement.** A relaxation-to-attractor process ( #detail-phenomena-systems-map FE(6) approach **R**) has no natural step count; it has a **residual tolerance**. The tolerance is declared on the nomos and folded into its key; the steps actually taken are then an *output* a probe can convict ("did it converge, and in how many"). An authored step count is a number nothing can falsify, and it silently changes meaning whenever the kernel or the terrain does. Three `ASSUMPTIONS.md` rows currently marked **arbitrary** name this mechanism as their cure: `epoch ↔ years` ("component E must pin it"), erosion run length ("convergence-$\varepsilon$ gate replaces fixed counts"), and water fill steps ("component E records convergence-$\varepsilon$"). Two live `NomosDecl` `status` strings declare the same absence.

5. **Replay and live-watching are one mechanism, not two features.** An explorer watching a build progress and an explorer scrubbing a finished world's history are both **reading a chain of time-indexed memos**. They differ only in whether new indices are still landing. Building them as two features produces two, and they will disagree. The reader is a poller over roots; the store is already the bus ( #form-store-as-save , #detail-builder-daemon FE(1)).

6. **Time-uniform where the aspect demands it.** Stage chains do not make every aspect demand-shaped. Regime G aspects — tectonics, sea level, the mantle-thermal trajectory — advance **time-uniformly** because a planet-spanning object cannot be held half in the past ( #form-temporal-lod-regimes FE(2)). For those, the chain *is* the spine and its density is set by what a consumer must resolve, not by a placed beacon. Regime H (history-dependent local) is where a chain is walked on demand as catch-up.

7. **Retention is bounded, and the bound has a primary.** A chain does not require keeping every stage. Gear–Wells order integration **slowest-component-first** precisely so a failed step needs only "reduce the last step," retaining **one prior value per component** — fastest-first would demand unstable reverse integration or prohibitive storage — and power-of-two stepsizes make every slower mesh a subset of every faster one ( #detail-seam-precedents FE(5), primary-read). That is the retention law for stage chains: enough history to back up one coupling step, no more.

8. **Live partial state.** The mantle-thermal epoch ladder is a stage chain in every respect but $\varepsilon$: `query::World::epoch_reduction` keys by `(mantle-thermal identity, seed, tp_bits, chain versions)`, is a store citizen, and is resumable — the round-trip probe convicts its order-independence and its resume-equals-run-through property on exactly that chain ( #form-depend-by-key-never-latest ). What no stage in the tree records is its convergence-$\varepsilon$, and no kernel yet runs to a criterion instead of a count.

9. **Out of bounds for this segment.** Claiming any $\varepsilon$-gated kernel is built; fixing the epoch ladder's density (that is a demand question, FE(6)); specifying the store schema for a recorded $\varepsilon$; treating the time-index as an ordering over wall-clock rather than over world-time.

## Epistemic Status

**Max attainable: exact** as architecture, once a stage records an $\varepsilon$ and something can fail when it does not.

**Currently `robust-qualitative`.** The formulation is a 2026-07-10 design (`framework-to-status-quo` §4, component **E**) whose retention half is primary-read numerics (Gear–Wells 1984), and whose necessity is attested from four independent live surfaces — three `ASSUMPTIONS.md` **arbitrary** rows, two `NomosDecl` `status` strings, a `LEXICON.udon` note on derived epistemic state, and `#disc-unlawfulness-budget`'s "not claimed" clause. **No Joseph DECISIONS row**; do not cite as ratified.

**Built:** time-index in key, store residency, and resumability, on the mantle-thermal chain only — convicted by `query::tests::{epoch_ladder_is_order_independent, resume_equals_run_through}`. **Unbuilt:** recorded $\varepsilon$ anywhere; any kernel running to a declared criterion rather than a count; stage naming beyond a raw time-index; the poller of FE(5).

**Provenance worth keeping, because the failure recurs.** This component did not carry from its source plan into the six-phase build plan, and no deliberate deferral was ever recorded — the loss was noticed on 2026-07-10 and written down, and then the note recording it was itself archived. It survived only in instruments nobody reads for open work (an assumptions ledger, two status strings) and as a parenthetical in a segment about something else. Its absence had no `#gap` row until 2026-07-24, which is why the project's own open-problem census could not see it. Stage `draft`.

## Discussion

The four things most often asked of this project's runtime — build in the background toward somewhere, restart at the right point after a change, watch the edge live, watch a replay — look like four features and are one object plus one reader. All four need the same thing: **the interior of a build has to be addressable**. A time-index in the key is what makes it so, and a recorded $\varepsilon$ is what makes a stage's honesty travel with it instead of living in a comment.

The reason this matters beyond convenience: a build with no addressable interior cannot be *believed* either. "It converged" is a claim, and without the residual it reached, nothing can convict it ( #norm-declaration-must-convict ). The same construction that makes a build watchable makes it honest — which is not a coincidence, because both are the demand that a computation say where it actually got to.

## Working Notes

- **First convicting step, and the cheapest:** give one relaxation kernel a declared residual tolerance and have it record the steps it took. Water is the natural first — it carries the most arbitrary constant (a hardcoded `200` at the call site, not even a declared parameter) and dominates build cost (26.4 s of a 38.8 s L9 build, measured 2026-07-24), so a criterion is likely to be *cheaper* as well as honest, since ocean and flat tiles should converge early. Measure that rather than assuming it.
- **Do not conflate the two "epochs."** Erosion iterations and mantle-thermal cooling snapshots share the word and nothing else; the collision has already cost one confused exchange. Renaming belongs with this work (`msc/build-parameterization-findings-2026-07-24.md`).
- **Densification must nest.** If the epoch ladder's density becomes a demand parameter, sample so the finer set *contains* the coarser one — each snapshot's $T_p$ is in its key, so a non-nested refinement silently invalidates every existing epoch memo. This is the same power-of-two subset property as FE(7).
- **Where the $\varepsilon$ goes is open**, deliberately: alongside the value in the memo, or as a sibling keyed artifact. Do not harden a schema before a second consumer exists ( #form-store-as-save FE(3) reserves the shape).
- Sibling gaps: the operated-instrument section and the demand spool (OUTLINE §III); `watchpoint` has no `LEXICON` entry while `beacon` and `focus` do.
