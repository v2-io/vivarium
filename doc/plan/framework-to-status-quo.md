# Framework-to-status-quo — the honest gap, and the drainage-shaped dependency design

> [!note]
> **Terminology (2026-07-10):** “recipe” throughout this dated document = **nomos** (pl. *nomoi*) — the term settled after this was written (LEXICON §2). Kept verbatim as history.

> **Claim homes for seam / fidelity law restated in §3:** fluxes as sufficient statistics at tile edges → `#form-seam-flux-exchange`; fidelity pull threshold → `#form-fidelity-invariant`; store as save (build shape A) → `#form-store-as-save`. This file remains a **dated status-quo + dependency design trail**, not claim canon.

*2026-07-09. Written after Joseph corrected a session's overclaim that the worldview spike was "the workflow, just manually cranked." It is not, and this file records
(1) the code-verified status quo, (2) what the framework target actually requires that has zero implementation, (3) a design answer to Joseph's dependency question —
"what coarse-pos/coarse-time evolution needs to run on adjacent areas, lazily pulled?" — whose answer is drainage-shaped, and (4) the build shape. Facts below are marked **[code]** (read in source this session) or **[design]** (hypothesis/proposal).*

## 1. The status quo, verified — one patch, no world

What `spikes/worldview` actually is **[code]** (`main.rs` `spawn_settle` /
`spawn_telescope`; `erosion.rs`):

- **Domain: one fixed square patch.** Default (settle mode): a 256² L19 macro region
  ≈ 4.9 km on a side, chosen by the startup focus; fine L21 pass over the same footprint; water fill on that bed; then storms forever *on that patch*. Nothing exists beyond its single deliberate border.
- **Telescope mode discards history on movement.** Tiers re-seed when the pawn drifts past ¼ span — the fine field is thrown away and re-derived from coarser tiers. Crucially the L19 macro tier itself re-seeds the same way, and it has no coarser evolved tier — it re-derives **from the raw fBm prior**. Walking ~2.4 km discards the entire erosion history. Returning does not restore what you saw:
  re-seed runs fresh `init_epochs`, statistically similar, historically unrelated.
- **The only global object is the prior.** `gen` maps any CellId on the sphere to a baseline column — you can *start* anywhere, but evolved state exists only where the current run's patch is, in RAM.
- **The only disk persistence is the fill cache** — a whole-run blob keyed by run parameters + `FILL_ALGO_VERSION`. No per-region objects, no roots, no reuse across different foci or window sizes.
- **The ladder dispatch is run-local.** `erosion::surface_at(c, &tiers)` /
  `column_at` resolve a cell against an in-RAM `Vec<ErodedRegion>` — the embryonic form of the §11 query, with no key, no recursion into dependencies, no store, no invalidation.
- **Tiles are not composable.** `Fluvial::accumulate_drainage` starts every cell at its own area — **a patch receives zero discharge from outside itself** — and outlets are "grid edge + sea," i.e. the boundary condition is "edge is base level, nothing flows in." A tile's rivers are systematically starved of any upstream catchment that lies outside the window. The observed seam pathologies
  (floating mesa, `seam_ridge` red, sim-edge outlets) are the visible signature.

So: the **kernels** are real and proven; the **world** is not. There is no global state, no navigation, no persistence-as-world, no LOD pull. The framework is not
"there, manually cranked" — it is unbuilt, and the spike is a physics testbench wearing a view.

## 2. What survives the move (so we port, not rewrite)

- **The kernels port.** `Fluvial` is already seeded through a surface closure
  (`from_surface`) over an arbitrary (face, level, origin, nx) window **[code]**;
  `WaterSim` similarly takes a bed. The main kernel-side change the framework forces is **parameterized boundary conditions** (upstream flux in, base level out) instead of the hard-coded edge policy — see §3.
- **The prior is already the global L∞ rung.** `gen` + `noise` are pure functions of (seed, key) — the bottom of every recipe chain, already framework-shaped.
- **`surface_at`/`column_at` is the query front-end's seed.** Its signature is right; its backing store is wrong (RAM vec → content-addressed store).
- **The fill cache is the §12 prototype** and its lessons transfer: key completeness (the FILL_CAP incident), version pinning, the two-leg cache test.
  The pervasive-memoization directive (DESIGN-REDUX §12, 2026-07-09) governs.
- **The convergence detector exists** (per-epoch mean |Δh|; the fill plateau logic) — the $\varepsilon$-gate ingredient for phase/stage freezes.

## 3. The dependency question — the answer is the drainage graph itself **[design]**

Joseph's framing: adjacent areas need *some* coarse-pos/coarse-time evolution,
lazily pulled, and the requirement depends on which rivers cross into the area of interest, by what degree — irregular islands of interdependence.

The design answer, in three steps:

1. **For fluvial state, the dependency cone of a point is not a halo — it is (a)
   the upstream closure of its catchment** (discharge and sediment supply are line-integrals over everything uphill) **and (b) the downstream path to base level** (sea/lake outlet elevation controls incision depth). Hillslope diffusion couples across divides only at one-cell range (the existing halo handles it). So the "island of interdependence" = watershed ∪ outlet path —
   irregular by nature, exactly as Joseph anticipated.

2. **The coarse spine draws the island map with kernels we already have** —
   though running them coarse-global across cube-faces is new integration work and the resulting basin-partition stability across levels is an open measurement, not a free readout. Priority-Flood + D8 receivers + MFD accumulation — already in the kernel at every level **[code]** — produce, at a coarse global level, precisely: the drainage graph, flow accumulation, basin partition, and base levels. So the coarse tier is not just an elevation prior for seeding fine tiers; **it is the query planner's dependency map.** A recipe for "eroded tile X at level L" declares as inputs the tiles its coarse-level catchment intersects — discovered from the spine,
   not assumed from adjacency.

3. **Degree is quantitative, not heuristic: the cut is flux magnitude.** The coupling strength across a tile edge *is* the discharge (and sediment flux)
   crossing it, read off the coarse MFD accumulation. Small creek in → the neighbor can be closed with a coarse boundary-condition approximation; major river in → the upstream tile must be evolved (or at least its time-averaged discharge/sediment flux memoized — the already-queued "sediment coupling:
   time-averaged discharge → erosion's A" item is this) before X is honest. This is the sufficient-statistic seam applied to tile composition (claim home `#form-seam-flux-exchange`): **the flux across the boundary is the sufficient statistic, and its magnitude sets the required fidelity of the pull.** The fidelity invariant (`#form-fidelity-invariant`) gives the threshold form: pull the neighbor at the coarsest rung whose flux error at the shared edge stays within the consumer's tolerance.

Corollary: the seam fix and the framework are the same work. "Fix the floating mesa" by clamping is cosmetic; giving tiles honest boundary conditions *from the spine's fluxes* makes tiles composable, which is what makes lazy regional pull correct at all. The `seam_ridge` probe graduates from cosmetic gate to correctness gate.

## 4. Build shape — five components to status-quo-in-framework **[design]**

Target restated: the current physics, running *inside* a frame where dropping in a new algorithm/simulation does the right thing (keyed, memoized, invalidated minimally), where saves/queries on vivia state know what stack built them, and where the pawn can walk off the patch and the world materializes lazily and persists.

- **A. Store + recipe layer** (`vivarium-world`, new module; §13 shape). Objects =
  content-addressed immutable results; roots = (aspect, face, tile, level,
  time-index) → hash; manifest pins store-format + seed + provenance. Key =
  complete input set: upstream object hashes + params + seed + **recipe version auto-derived from kernel source** (build-script hash per the 2026-07-09 directive). Canon roots vs scratch roots per the run-modes carve (LEXICON §3).
- **B. Coarse global spine.** Macro erosion + hydrological equilibrium at a coarse level, per-face, on demand, memoized once per (seed, recipe-version): produces elevations + drainage graph + basin partition + base levels — the world's
  "sense of where the global state is," and the dependency map for §3. The planned analytic hydrological init (`ref/erosion-port/NOTES.md` §Next) slots here as the equilibrium solver; the spine memo is the first real phase-shaped memo.
- **C. Tile recipes with flux boundary conditions.** Re-express Fluvial/WaterSim runs as tile-sized recipes whose BCs (upstream discharge + sediment in, base level out) are pulled from the spine and neighbor memos. Kernel change:
  parameterize the edge policy (today: hardcoded edge-outlet, zero-inflow
  **[code]**). This is the bulk of the porting work and where the seam fix lives.
- **D. Query front-end.** `column_at`-shaped queries route through the store:
  hit → load; miss → recurse per the dependency map → compute → store → serve.
  The view stops owning tiers and just queries; navigation falls out (walk anywhere; matured state persists; returning is a cache hit). The telescope becomes a *policy* over queries (what to pull eagerly around the pawn), not a state-owner.
- **E. Time-indexed stage chains.** The settle sequence (macro epochs → fine passes → fill → living) becomes explicit chained recipes with time-indices
  (§11: time breaks cycles), each stage a memo with its convergence-$\varepsilon$ recorded —
  the workflow doc's Stage-1 freeze machinery at sub-phase grain.

Order of attack **[design, loosely held]**: A first (small, everything needs it);
B next (unlocks §3's map + is independently the analytic-init home); C is the long pole; D wires it to the view early (even backed by only A+B it already gives persistent navigation over spine-fidelity terrain); E last (formalizes what settle-mode already sequences by hand).

## 5. Open questions / honest risks

- **Water at tile boundaries is harder than erosion at tile boundaries.** Erosion composes through fluxes reasonably (stream-power is local given A and base level); the shallow-water sim has bidirectional edge exchange (backwater,
  waves). The multirate answer — fast water sees slow neighbors as quasi-static stage/discharge BCs — needs a probe of its own. Unproven here.
- **Time-index semantics across tiles**: neighbor tiles evolved to different sim-ages is the differential-aging problem generalized (seam_ridge measured it at 4.3×). The spine's causal-time bookkeeping (which tile is at which epoch,
  what flux era it exported) must be part of the key or the islands go incoherent.
- **How coarse can the spine be** before its drainage topology is wrong enough to mis-draw the dependency islands (a divide misplaced at L12 puts a tile in the wrong catchment)? Wants a measurement: basin-partition stability across levels.
- The detail→abstract frontier (§6) is *still* not on this path — tiles here are generated, not agent-edited. It stays the one open research problem, deferred.
