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

1. **The gap, measured before it was named.** `erosion-tile` keeps its declared promise (an eroded bed, exports-at-boundary) and passes every flux audit — while `Fluvial::outlets` makes every edge cell of a partial tile a base-level sink, so each tile grades to its own perimeter. Consequences, all measured ( #obs-tile-outlets-grade-away-the-basins ): every tile-local basin is capped at one tile by construction, so the kernel's own incision driver is starved by 3.25× against the assembled routing of the same beacon patch; and the seam cost read honestly is **63.5 %** of the beacon window in closed depressions with the two contracts carving beds 259 m apart on the tile rings (the 19.67 % an edge-sink reader reports is an understatement of the same quantity — #obs-tile-outlets-grade-away-the-basins FE(5)), a cost that rises monotonically as the grain is made finer ( #obs-lakes-are-routed-over-not-carved-away FE(7)) and vanishes into a 5 % agreement at L9 planet grain. None of this is expressible in the current declaration: the audit is green because the lie is in a column the schema does not have.

2. **The claim, and its first instance is now in the kernel.** A nomos whose kernel has a spatial domain must **declare its boundary contract**: what its edges are treated as (base level, flux boundary, periodic continuation, halo-from-neighbour, …), and therefore what its promises mean *near* an edge. The declaration follows the pattern of box ③ ( #sketch-nomos-declaration-boxes : statistic + exactness on every promise): a boundary contract on every domain-bearing promise, with `Undeclared` as the honest first-class value — the point is to make the column *exist* so its absence is visible debt, not silence ( #form-nomotheke-registry : undeclared law is unlawful).

   The erosion kernel has taken the first step: `EdgeContract` names the two policies `Fluvial::outlets` used to infer from geometry (`BaseLevelSink`, `NoFluxWall`), and the inference survives verbatim as the default, so naming the contract changed no world and a unit test says so. It is a **kernel** parameter and not yet a **key** field, deliberately: no caller reached through a complete key can select one, so the builder's key stays complete. Declaration and keying are separate obligations and this is only the first — but it is the one that turns a contract from a property of the code into a question a probe can put to the world.

3. **Why this is the flux web's business and not a kernel detail.** Two tiles of the same nomos are *neighbours coupling through a shared boundary* — the same relationship #form-seam-flux-exchange governs across levels ("seams exchange fluxes, not states"). A tile that treats its edge as a base-level sink is asserting a flux boundary condition (everything drains out, nothing enters) that its neighbour's existence contradicts. Same-level tile seams are seams; the seam discipline already in law simply has not been applied to them, and the declaration layer is where the obligation attaches.

4. **The convicting probe shape exists, in two strengths.** A boundary contract can fail ( #norm-declaration-must-convict ). The weaker form re-routes one bed two ways and compares — the 3.25× trunk ratio and the 19.67 % seam-pit fraction are this, and they convict the *reader's* view of a bed. The stronger form **carves the same geography under each contract** and compares the worlds that result: `examples/base_level_probe` sweeps the tile grain from 8 cells to a whole face at fixed prior, uplift and epoch count, and the contract's signature shows up as a monotonic depression census and a 4 % seam step at 40 epochs rising to 20 % at 300 ( #obs-lakes-are-routed-over-not-carved-away FE(7)–(8)). Only the second can distinguish a defect the contract causes from one it merely sits beside — which is how the no-lakes attribution was found to belong elsewhere. Declaring the contract turns both into regression guards. **The kernel half of the column exists** (`EdgeContract` names the two single-tile outlet policies, inference preserved as default). **The keyed Halo path ships in production (2026-07-29):** `HaloSchedule` plus the exchange region fold into `query::erosion_key`; `vivarium build` erodes each face and the beacon under Jacobi exchange; view reads prefer halo over plain, compute consumers name their bed article by key ( #form-same-level-halo-exchange ). Single-tile `BaseLevelSink` / `NoFluxWall` remain kernel parameters for probes and are not silently rekeyed into old plain memos.

5. **Grain boundaries generalize the clause.** The same failure shape recurs wherever two *grains* of one law meet: tile ↔ tile (this segment's measured case), pour-grain ledger ↔ tile-grain fluvial (carved tiles do not debit their columns — #form-isostasy-column FE(9)'s open bridging rung), fine patch ↔ coarse world (two datums at the beacon edge, honestly shown but undeclared). Vertical mechanisms (keys, chains, columns) conserve within a grain; the horizontal joins between grains are where 2026-07-28's three largest defects all lived. A boundary contract is the declaration-layer name for the horizontal join.

6. **A neighbour-dependent contract is keyable, and Halo ships in production.** The two named single-tile contracts are both functions of the tile alone, which is why neither is honest at a coastless window (FE(1)); the third — a halo exchanged with the neighbours the tile actually has — is what supplies a base level that is not a fiction ( #form-same-level-halo-exchange ). Its declaration is a **descriptor** (`HaloSchedule`: depth, cadence, cone truncation), each field identity the moment a caller can vary it. The descriptor keys at constant size ( #form-complete-content-addressed-key FE(6)). Schema shape toward: `Undeclared | BaseLevelSink | NoFluxWall | Halo{d, σ, ρ, chart_edge}` — Halo is the builder default on face/beacon region sweeps; the first two remain probe/kernel-selectable; NomosDecl box schema is still unbuilt.

7. **Chart-edge is a second column on the Halo contract.** When $d\ge 1$ pushes a window off a cube face, production **clamps** samples to the chart ( #obs-chart-edge-halo-clamps-to-the-face ). That policy is in force and was ambient until named — the same defect class FE(1) recorded for edge-sink tiles. Honest schema: `chart_edge ∈ {Clamp, Resample{…}}` folded into the key when Halo is selected. Depth $\ge 2$ true cross-face geometry is measured to need resampling ( #form-cellid-chunk-patch FE(4)); clamp is the deficiency standing in until that field lands ( #norm-declared-violation-is-not-license ). Family map: #form-seam-families .

8. **Out of bounds for this segment.** Full NomosDecl boundary-contract column; choosing operating $(d,\sigma,\rho)$ as law (measured at one grain only — #obs-exchange-repairs-the-seam-and-overlap-does-not ); claiming the clamp *is* the right cube-edge law.

## Epistemic Status

**Max attainable: exact** for "the current declaration schema cannot express the measured defect class" (checkable against `nomotheke.rs` by inspection); **empirical** for the three measured instances FE(1)/(5) cite.

**Currently `robust-qualitative`:** the gap and its instances are measured ( #obs-tile-outlets-grade-away-the-basins , the seam-pit and assembled-routing numbers, and the 63.5 %-versus-one-cell comparison the declaration itself made runnable). Kernel `EdgeContract` + tests remain; **Halo schedule is the builder default** on face/beacon region sweeps; chart-edge clamp is present law and under-declared ( #obs-chart-edge-halo-clamps-to-the-face ). Declaration-box *schema* on NomosDecl is still unbuilt. No Joseph ratification; do not cite as ratified. Stage `draft`.

## Discussion

The declaration boxes exist because the 2026-07-13 audit found "the theory demands declarations the data model cannot hold." This segment is the same finding one layer out: the *practice* produced a defect class the data model cannot hold, three times, in one measured day. The pattern that closed box ③ — add the column, let `Undeclared` be honest, let tests convict mismatches — is the pattern here; what is new is only the recognition that a *domain edge is a coupling surface*, so edge semantics belong to the flux web and not to kernel privacy.

## Working Notes

- First candidate rows when the box lands: `EROSION` (edge = base-level sink on partial tiles; coast-only on full faces; and the full-face case is itself a **no-flux wall** at the cube seam, which is a third contract nobody has named — three contracts, all currently implicit in `Fluvial::outlets`), `WATER` (edge behaviour of the fill), `CLIMATE`/`UPLIFT` (pointwise — boundary-free, and saying so is the honest row).
- **Keyed Halo adopted 2026-07-29.** Builder + view cohort use schedule keys; plain single-tile contracts remain probe-accessible. **Sill-graph library object landed 2026-07-31** (scalar exchange refuted — do not reintroduce a spill scalar); **sill production wire + flux half remain open**. Chart-edge clamp named as FE(7).
- Sibling: the cohort question is the *store-side* twin (which world-history am I reading) of this *domain-side* question (which edge semantics am I computing under); both are "the audit is green while the answer is wrong" classes — map at #form-seam-families .
