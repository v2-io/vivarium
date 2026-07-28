---
slug: form-declared-boundary-contract
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-flux-web
  - form-nomotheke-registry
  - sketch-nomos-declaration-boxes
  - obs-tile-outlets-grade-away-the-basins
  - form-seam-flux-exchange
  - norm-declaration-must-convict
---

# A promise holds under a boundary contract; an undeclared edge is an unlawful edge

The flux web types the *quantities* a nomos exchanges; nothing yet types the **domain conditions** under which its promises hold. A kernel can keep every declared flux promise while lying at its own edge — and the edge is where it lies to its *neighbours*, which is exactly the coupling surface the web exists to govern.

## Formal Expression

1. **The gap, measured before it was named.** `erosion-tile` keeps its declared promise (an eroded bed, exports-at-boundary) and passes every flux audit — while `Fluvial::outlets` makes every edge cell of a partial tile a base-level sink, so each tile grades to its own perimeter. Consequences, all measured ( #obs-tile-outlets-grade-away-the-basins ): every tile-local basin is capped at one tile by construction, so the kernel's own incision driver is starved by 3.85× against the assembled routing of the same beacon patch; and 13 % of that assembled patch is seam pits (adjacent tiles graded to different perimeter heights, deepest 411 m), a fraction that rises monotonically as the grain is made finer ( #obs-fill-writes-itself-into-the-bed FE(4)). None of this is expressible in the current declaration: the audit is green because the lie is in a column the schema does not have.

2. **The claim, and its first instance is now in the kernel.** A nomos whose kernel has a spatial domain must **declare its boundary contract**: what its edges are treated as (base level, flux boundary, periodic continuation, halo-from-neighbour, …), and therefore what its promises mean *near* an edge. The declaration follows the pattern of box ③ ( #sketch-nomos-declaration-boxes : statistic + exactness on every promise): a boundary contract on every domain-bearing promise, with `Undeclared` as the honest first-class value — the point is to make the column *exist* so its absence is visible debt, not silence ( #form-nomotheke-registry : undeclared law is unlawful).

   The erosion kernel has taken the first step: `EdgeContract` names the two policies `Fluvial::outlets` used to infer from geometry (`BaseLevelSink`, `NoFluxWall`), and the inference survives verbatim as the default, so naming the contract changed no world and a unit test says so. It is a **kernel** parameter and not yet a **key** field, deliberately: no caller reached through a complete key can select one, so the builder's key stays complete. Declaration and keying are separate obligations and this is only the first — but it is the one that turns a contract from a property of the code into a question a probe can put to the world.

3. **Why this is the flux web's business and not a kernel detail.** Two tiles of the same nomos are *neighbours coupling through a shared boundary* — the same relationship #form-seam-flux-exchange governs across levels ("seams exchange fluxes, not states"). A tile that treats its edge as a base-level sink is asserting a flux boundary condition (everything drains out, nothing enters) that its neighbour's existence contradicts. Same-level tile seams are seams; the seam discipline already in law simply has not been applied to them, and the declaration layer is where the obligation attaches.

4. **The convicting probe shape exists, in two strengths.** A boundary contract can fail ( #norm-declaration-must-convict ). The weaker form re-routes one bed two ways and compares — the 3.85× trunk ratio and the 13 % seam-pit fraction are this, and they convict the *reader's* view of a bed. The stronger form **carves the same geography under each contract** and compares the worlds that result: `examples/base_level_probe` sweeps the tile grain from 8 cells to a whole face at fixed prior, uplift and epoch count, and the contract's signature shows up as a monotonic depression census and a 3–5 % seam step ( #obs-fill-writes-itself-into-the-bed ). Only the second can distinguish a defect the contract causes from one it merely sits beside — which is how the no-lakes attribution was found to belong elsewhere. Declaring the contract turns both into regression guards. **The kernel half of the column now exists** (`EdgeContract` names the two policies, with the geometric inference preserved as the default and tests convicting it), which is what let the stronger probe run at the beacon's own grain; **the keyed half does not** — no caller through `query.rs` can select a contract, deliberately, so the builder's complete key stays complete until a neighbour-dependent contract folds its dependency in. An undeclared contract cannot be *swept by keyed builds*, and that remains the concrete price of the missing declaration column.

5. **Grain boundaries generalize the clause.** The same failure shape recurs wherever two *grains* of one law meet: tile ↔ tile (this segment's measured case), pour-grain ledger ↔ tile-grain fluvial (carved tiles do not debit their columns — #form-isostasy-column FE(9)'s open bridging rung), fine patch ↔ coarse world (two datums at the beacon edge, honestly shown but undeclared). Vertical mechanisms (keys, chains, columns) conserve within a grain; the horizontal joins between grains are where 2026-07-28's three largest defects all lived. A boundary contract is the declaration-layer name for the horizontal join.

6. **Out of bounds for this segment.** Choosing the *right* boundary condition for erosion (that is the cross-tile base-level work, open); implementing the schema field; claiming any existing declaration is complete once the column exists — the first honest state of most rows will be `Undeclared`.

## Epistemic Status

**Max attainable: exact** for "the current declaration schema cannot express the measured defect class" (checkable against `nomotheke.rs` by inspection); **empirical** for the three measured instances FE(1)/(5) cite.

**Currently `robust-qualitative`:** the gap and its instances are measured ( #obs-tile-outlets-grade-away-the-basins , the seam-pit and assembled-routing numbers, and the 60.6 %-versus-0 comparison the declaration itself made runnable). The declaration-box *schema* remains an argued design following an accepted pattern (box ③) and is not built; what is built is one kernel-level contract enum, convicted by two tests — that the inference is preserved exactly, and that the two contracts carve measurably different worlds with the difference largest at the boundary they describe. No Joseph ratification; do not cite as ratified. Stage `draft`.

## Discussion

The declaration boxes exist because the 2026-07-13 audit found "the theory demands declarations the data model cannot hold." This segment is the same finding one layer out: the *practice* produced a defect class the data model cannot hold, three times, in one measured day. The pattern that closed box ③ — add the column, let `Undeclared` be honest, let tests convict mismatches — is the pattern here; what is new is only the recognition that a *domain edge is a coupling surface*, so edge semantics belong to the flux web and not to kernel privacy.

## Working Notes

- First candidate rows when the box lands: `EROSION` (edge = base-level sink on partial tiles; coast-only on full faces; and the full-face case is itself a **no-flux wall** at the cube seam, which is a third contract nobody has named — three contracts, all currently implicit in `Fluvial::outlets`), `WATER` (edge behaviour of the fill), `CLIMATE`/`UPLIFT` (pointwise — boundary-free, and saying so is the honest row).
- **The contract has to become a parameter before the experiment that would choose it can run.** The first consumer of this segment ( #obs-fill-writes-itself-into-the-bed ) could sweep the grain only because a *whole face* takes the coast-only branch by geometric accident; a beacon-footprint window cannot be carved under any contract but edge-sink. Making the contract an explicit keyed field is therefore not decoration on the repair — it is the repair's instrument. Note also that a contract which makes a tile depend on its neighbours must fold that dependency into the tile's key ( #form-complete-content-addressed-key ); the era work's chain-carried fork would multiply that footprint per cooling stage, which is why the base-level dependency story wants to exist first.
- Sibling: the cohort question is the *store-side* twin (which world-history am I reading) of this *domain-side* question (which edge semantics am I computing under); both are "the audit is green while the answer is wrong" classes.
