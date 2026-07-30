---
slug: obs-halo-windows-overhang-the-chart-and-mint-nan
type: observation
status: exact
stage: draft
depends:
  - form-same-level-halo-exchange
  - form-cellid-chunk-patch
  - form-ocean-is-connectivity-not-elevation
  - norm-probes-before-claims
---

# Halo windows overhang the cube chart, and the clamp that saves them from panicking mints NaN on one edge and misaligns the carve on the other

Every cohort in the store carries **83 `erosion-tile` roots with non-finite heights**, out of 3552, and they are **not** scattered: 73 of 73 distinct positions sit on a region perimeter and **none** in the interior. The inputs are clean — `initial-topography`, `uplift-tile` and `climate` are finite in every cell of every root — so erosion mints these from finite data, deterministically, at the chart's edge.

The cause is one clamp doing two jobs. A halo window on a region perimeter asks for cells outside the cube chart; `Fluvial::from_surface` clamps the index (*"Clamp rather than panic: true cube-edge resampling for d≥2 is still open"*), and that clamp is applied to the cell's **geometry** as well as to its **data**. Clamped cells therefore share an $(i,j)$, share a centre vector, and sit at distance **zero** from a real neighbour — so any slope or flux that divides by that distance is $\Delta h / 0$.

## Formal Expression

1. **Measured: the census.** `examples/nan_census` over 122 810 roots. Non-finite cells appear in exactly two nomoi:

   | nomos | roots | roots with non-finite cells | non-finite cells |
   |---|---|---|---|
   | `erosion-tile` | 85 374 | 581 | 300 522 |
   | `mantle-thermal` | 1 626 | 28 | 28 |
   | `initial-topography` · `uplift-tile` · `climate` · `water-tile` · `hydrosphere` | 36 810 | **0** | **0** |

   The `mantle-thermal` rows carry exactly one bad cell each and are a separate, unexplained defect.

2. **Measured: it is the same 83 roots in every cohort.** Per `src=`, seven consecutive cohorts each report 3552 `erosion-tile` roots and **83** with non-finite cells. Deterministic, not a flake.

3. **Measured: perimeter only.** Of the current cohort's distinct `(face, oi, oj, level)` positions holding non-finite cells, **73 are on a region perimeter and 0 are interior**. Within one position the count grows monotonically along the stage chain (257 → 686 → 1093 → 1561 → 1917 → 2136 → 2215 → 2513), so the corruption starts at the edge and spreads through routing.

4. **Measured: the plain carve is clean.** The same footprint (face 0, L9, `(128,448)`, 64², 40 epochs), carved through the per-tile path on its own stored inputs, stays finite for every epoch (`examples/nan_origin`). The mint is in the **region/halo path**, and every convicted root carries `edge=halo d=16`.

5. **Measured: the degenerate geometry.** For a perimeter window at L9 with $d = 16$, the chart is $512^2$ and the last row asks for $j = 591$: **81 rows collapse onto $j = 511$, i.e. 80 duplicate rows.** Two clamped cells share a centre vector exactly. A genuinely adjacent pair at that level is **19 395.8 m** apart; a duplicated pair is **0.0 m** apart. Division by that distance is the mint.

6. **The geometry never needed clamping.** `measure::cell_center_unit` is an analytic equiangular formula and **extrapolates cleanly past the chart edge** — evaluated 80 rows out ($j = 591$) it returns a unit vector ($\lvert \mathbf{d} \rvert = 1.000000$) that continues smoothly onto the neighbouring face's territory. So the clamp conflates two things that are not alike: **data** beyond the chart genuinely requires cross-face resampling and is open work ( #form-cellid-chunk-patch ); **geometry** beyond the chart is available in closed form for free.

7. **The low edge fails differently, and silently.** The region carve computes window origins as `region_oi + ti·tile_n − d`, which is **negative** for a low-edge tile, and the builder passes `oi.max(0) as u32`. So the window is not padded — it is **slid**. Its interior no longer sits at halo offset $d$, so the publish step (`h[(d+j)·win + (d+i)]`) writes ground from a **16-cell offset** into that tile. No NaN results, which is why nothing caught it; the tile is simply carved from the wrong ground. This is the more dangerous of the two failures because its output is finite and plausible.

8. **It interacts with the ocean mask, and the interaction is measured.** A non-finite height fails `h ≤ sea`, so a NaN cell is classified **not submerged** — under #form-ocean-is-connectivity-not-elevation it is therefore a *wall* in the reachability flood, able to enclose submerged ground that is not enclosed. Measured across the cohort boundary: non-finite **cells** rise from 37 948 to **55 391** (+46%) in the first cohort carved under connectivity, with the affected root count unchanged at 83 — consistent with NaN walls creating spurious basins whose fill then propagates NaN through the spill maximum.

## Epistemic Status

**Max attainable: `exact`** for FE(1)–(7) — each is a count or a closed-form geometric fact from a deterministic read of the store, reproducible by the two named instruments.

**Currently `exact`** for those clauses. FE(8)'s +46% is exact as a measurement; its *attribution* to NaN-as-wall is a strong inference from the mechanism and the timing, not an isolated experiment — the clean test is to repair the mint and re-measure, which is also the repair. Stage `draft`.

## Discussion

The clamp's own comment is honest about being provisional and names the right open problem. What it did not anticipate is that a *defensive* clamp — chosen over a panic, which is the humane choice — would silently produce a **geometrically impossible** configuration rather than merely an inaccurate one. Zero distance between distinct cells is not an approximation of anything; it is outside the model. That is the general lesson worth carrying: clamping an index to keep a program alive is safe for *lookup* and unsafe for *metric*, because the metric is what the physics divides by.

It is also the night's recurring shape once more. One accessor served two callers with different rights (geometry and data), and the caller with fewer rights set the behaviour for both — the same structure as `ErodedRegion::surface_m` serving both seeding and depiction ( #form-fidelity-ladder FE(7)).

## Working Notes

- **The repair, in two independent parts.** (a) Compute `centers` and `cell_area` from the **unclamped** requested index while continuing to clamp the *data* lookup — this removes the degenerate metric and leaves the declared cross-face data gap exactly where it was. (b) Make the low-edge window **pad rather than slide**: `from_surface` would need signed origins, and the measure helpers signed indices, so the requested cell can be named even when it is off-chart on the low side. (a) stops the NaN; (b) stops the misalignment; neither substitutes for cross-face resampling.
- **Owed before the repair lands:** a tripwire that fails on any non-finite stored height. There is no such test today, which is why 83 roots survived seven cohorts.
- **Not investigated:** the 28 `mantle-thermal` roots with one bad cell each. Different nomos, different shape, no hypothesis.
- **Expect payload changes on repair:** every perimeter tile of every region moves, so this is a real-diff cohort with a changelog entry owed, and every drainage-derived statistic measured on a perimeter tile is suspect until then.
