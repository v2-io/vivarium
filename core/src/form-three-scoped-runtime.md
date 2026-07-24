---
slug: form-three-scoped-runtime
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-pull-query-composition
  - form-depend-by-key-never-latest
  - form-store-as-save
  - form-temporal-lod-regimes
  - form-complete-content-addressed-key
  - sketch-detail-abstract-reversion
---

# The three-scoped runtime: spine, cones, edit layer — and time breaks every cycle

The fully-general pull graph (non-local + coupled + mutable, all aspects at once) is not tractable as one graph; it degrades into three scoped pieces, and **that decomposition is the architecture**: a precomputed coarse-global spine, lazy memoized local cones, and an edit-propagation layer whose open half is now precisely named. Lagged coupling makes the coupled system a DAG — time is in every key, and time breaks every cycle. Prefetch is pure optimization and can never be a correctness surface.

## Formal Expression

1. **Three scopes, not one graph.**
   - **Coarse-global spine** — planet-spanning aspects (the regime-G class of #form-temporal-lod-regimes ) at low resolution, advanced time-uniformly; also the dependency planner (drainage islands, phase targets) that tells local pulls what they need. *Unbuilt as a spanning spine; partial precedents in per-face coarse passes.*
   - **Lazy memoized local cones** — fine detail pulled on demand near the participant, by recursive keyed pull ( #form-pull-query-composition ), reading neighbours only by key ( #form-depend-by-key-never-latest ). *The evaluation pattern is live in hand-written per-nomos miniature; the cone machinery (demand spool, flux-BC tiles) is plan-grade.*
   - **Edit-propagation layer** — absorbing irreducible edits into memoized macros. *Split state (FE 5): state up-propagation is measured solved; the law closure is open research.*
2. **Fan out, then in.** The dependency cone is not local for global aspects: fine work under the participant pulls a bounded local cone **plus** the global-coarse spine (no local uplift without the plate configuration). Recursion fans *out* to global-coarse, then *in* to local-fine; "just enough for under the pawn" holds only for local aspects.
3. **Time breaks every cycle.** Aspects are mutually dependent (water↔climate↔geology), and a pull graph must be acyclic. The cut is lagged coupling: $A$ at $t$ depends on $B$ at $t-\Delta$, never at $t$. The real graph is therefore a DAG over $(\text{aspect}, \text{region}, \text{resolution}, \text{time-index})$ — **the memo key includes time**, and an epoch chain is a memoized chain of timesteps: expensive first pull, cached after. *Doctrine with partial practice: time/epoch indices ride in several live keys; "time in every key" as a mechanized invariant is unbuilt* ( #form-complete-content-addressed-key completeness law governs; this clause names the time component specifically).
4. **Cold-start is real, and mitigated, not eliminated.** The first pull at a deep target time with an empty store pays the full coarse global history. Mitigation is structural: coarse-first (a cheap global spine at world creation) plus epoch checkpoints as first-class memos ( #form-store-as-save ).
5. **The edit layer, stated at present truth.** The *state* half is measured solved: an irreducible single-cell edit propagates to the root of the multiresolution representation **bit-exactly**, $O(\log N)$ (wavelet-store spike, PROBE-grade; `DECISIONS[wavelet-store-spiked-the-seam-is-not-the-details]`, proposed). What remains open is the **nonlinear closure for a non-local flux** — the *law* does not commute with restriction ($\lVert R\circ E - E\circ R\rVert$ signed mean $+5.34\,\mathrm{m}$ against the $7.22\,\mathrm{m}$ the erosion itself carved: a bias the size of the physics), and no local statistic predicts where (detail-RMS vs law-error correlation $-0.027$; drainage area is a non-local accumulation, so coarsening changes the flow network topologically). Claim home for the open problem: #sketch-detail-abstract-reversion (renamed there per the decision's instruction). Downstream invalidation is mature (Salsa/Adapton lineage); it is the **law**, not the plumbing, that is research.
6. **Prefetch is the safe layer.** Prediction (pre-pulling along a likely trajectory) is pure optimization over a correct lazy base: a wrong guess wastes spare cycles; a right one hides latency; it can never cause a correctness bug, because it only chooses *when* keys are computed, never *what* they contain (schedule ≠ ontology, #form-pull-query-composition FE(3)). Build it last; nothing may depend on it for truth.
7. **Out of bounds.** (a) Claiming any scope as a shipped generic engine; (b) letting the spine become a second truth channel (it is keys and memos on the same store bus); (c) absorbing this decomposition back into #form-pull-query-composition (that law is the evaluation pattern; this one is the tractability decomposition on top of it); (d) treating the ethereal-explorer path as blocked by the edit layer — a read-only explorer makes no irreducible edits.

## Epistemic Status

**Max attainable: exact** as architecture once the spine and cone machinery exist to convict it. **Currently `robust-qualitative`:** the decomposition is design stance (DESIGN-REDUX §11 "graceful degradation"; ARCHITECTURE §5), agent-articulated and Joseph-worked-within, with **no DECISIONS ratification row**. FE(3)'s lagged-coupling DAG is doctrine whose full mechanization is unbuilt. FE(5)'s measurements are spike-grade under the wavelet-store harness, `:by claude`, proposed. Honest build inventory: spine **unbuilt**; demand spool / flux-BC tiles **plan-grade** (`#detail-builder-daemon`, `abyssal-parity-plan.md`); pull pattern **live in miniature**; prefetch **deliberately absent**; edit layer **split** per FE(5).

Stage `draft`.

## Discussion

This segment exists so the decomposition stops living as law-shaped prose in two design documents while being formally owned by nobody. The three scopes are how the clean pull-graph ideal survives contact with three facts: global aspects are non-local (scope 1), demand is observer-shaped (scope 2), and mutation must eventually re-summarize upward (scope 3). The leaf-only-evolution tension ( #form-temporal-lod-regimes Working Notes; `the-grid-tentatively-decided…` impact) cuts across scopes 1–2: if summary tiers become derived-from-leaves over materialized ground, the spine's independence over that ground is constrained — unresolved, Joseph's to weigh, and named here so neither scope silently assumes it away.

## Working Notes

- **Dual-home demote:** DESIGN-REDUX §11 (graceful-degradation block + four-seams-of-the-graph + prefetch) and ARCHITECTURE §5 opening get claim-home banners pointing here; teaching texture stays.
- **OUTLINE §III gap row** (three-scoped runtime / time-DAG / prefetch) is closed by this segment; the *builder daemon* gap row remains separate and open.
- Sibling trail: the temporal multiresolution half (time as a third chain group; the flux register as a time-integrated detail coefficient; nests iff the time ladder locks to the quadtree per $z$-sector) is derived-not-built in the wavelet-store decision's impact — lives with #sketch-dynamic-exponent-seams as exploration, not here.
- **Do not claim** Salsa/Adapton adoption, a generic planner, or "the DAG is enforced" — enforcement is future compliance surface under #form-complete-content-addressed-key .
