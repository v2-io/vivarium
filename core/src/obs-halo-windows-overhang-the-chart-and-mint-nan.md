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

1. **Measured: the census, and the high-edge mint is now repaired.** `examples/nan_census` over the whole store. The **current cohort holds zero non-finite `erosion-tile` cells** (0 roots of 3552, 0 positions) since the geometry split landed; the figures below are the seven cohorts carved before it, which remain in the store as archaeology:

   | nomos | roots | roots with non-finite cells | non-finite cells |
   |---|---|---|---|
   | `erosion-tile` | 85 374 | 581 | 300 522 |
   | `mantle-thermal` | 1 626 | 28 | 28 |
   | `initial-topography` · `uplift-tile` · `climate` · `water-tile` · `hydrosphere` | 36 810 | **0** | **0** |

   The `mantle-thermal` rows carry exactly one bad cell each and are a separate, unexplained defect.

2. **Measured: it was the same 83 roots in every cohort, and is now none.** Per `src=`, seven consecutive pre-repair cohorts each report 3552 `erosion-tile` roots and exactly **83** with non-finite cells — deterministic, not a flake. The cohort carved after the split reports **0**, which is what convicts the zero-distance metric as the *only* mint on this path.

3. **Measured: perimeter only.** Of the current cohort's distinct `(face, oi, oj, level)` positions holding non-finite cells, **73 are on a region perimeter and 0 are interior**. Within one position the count grows monotonically along the stage chain (257 → 686 → 1093 → 1561 → 1917 → 2136 → 2215 → 2513), so the corruption starts at the edge and spreads through routing.

4. **Measured: the plain carve is clean.** The same footprint (face 0, L9, `(128,448)`, 64², 40 epochs), carved through the per-tile path on its own stored inputs, stays finite for every epoch (`examples/nan_origin`). The mint is in the **region/halo path**, and every convicted root carries `edge=halo d=16`.

5. **Measured: the degenerate geometry.** The region carve computes window origins as `region_oj + tj·tile_n − d`, which for a whole-face L9 region at $d = 16$ gives $\{-16, 48, \ldots, 432\}$ with span 96. The highest window therefore spans $432 \ldots 527$ against a chart whose last index is 511: **17 rows land on $j = 511$, i.e. 16 duplicate rows.** Two clamped cells share a centre vector exactly, so they sit **0.0 m** apart while a genuinely adjacent pair at that level is **19 395.8 m** apart. Division by that distance is the mint.

6. **The geometry never needed clamping.** `measure::cell_center_unit` is an analytic equiangular formula and **extrapolates cleanly past the chart edge**: over the whole range the halo actually asks for (16 rows out, $j = 527$) it returns unit vectors with smoothly growing arc separations (19 395.8 m at the edge → 19 622.3 m at $j = 527$), and `measure::cell_area_m2` returns finite positive areas throughout (283.9 → 274.4 Mm²). So the clamp conflates two things that are not alike: **data** beyond the chart genuinely requires cross-face resampling and is open work ( #form-cellid-chunk-patch ); **geometry** beyond the chart is available in closed form for free.

   The extrapolated direction is *not* the true neighbouring-face cell centre — each face has its own parametrisation, so continuing this one past its edge lands near but not on the neighbour's cell. It is a smooth, non-degenerate, slightly distorted continuation, and it stays well inside the tangent domain over the range needed (the extended parameter reaches ≈1.06 of the face half-width at $d = 16$). Strictly better than a zero-distance impossibility, and not a substitute for real cross-face geometry.

7. **Measured: what the repair moved.** Computing `centers` and `cell_area` from the **unclamped** requested index — while the height still reads from the clamped cell — changed **655 of 3552** `erosion-tile` payloads (18.4%) and **94 of 384** `water-tile` payloads, with `initial-topography`, `climate`, `uplift-tile`, `hydrosphere` and `mantle-thermal` bit-identical. 655 roots over a ~9-stage chain is ≈73 tiles, which is **exactly the 73 positions that held non-finite cells**: the repair moved the corrupted set and nothing else.

   Only the *last* row and column of tiles overhang at all. Of the eight window origins $\{-16, 48, \ldots, 432\}$ only 432 exceeds the chart; origins 48–368 span at most 463 < 511. So the affected fraction is one row plus one column per face, not a four-sided perimeter — a pre-registration of "43.75–100%" missed low for exactly that reason (`msc/lake-connectivity-2026-07-29-prereg.md`).

8. **The low edge fails differently, and silently — and this repair does not touch it.** The region carve computes window origins as `region_oi + ti·tile_n − d`, which is **negative** for a low-edge tile, and the builder passes `oi.max(0) as u32`. So the window is not padded — it is **slid**. Its interior no longer sits at halo offset $d$, so the publish step (`h[(d+j)·win + (d+i)]`) writes ground from a **16-cell offset** into that tile. No NaN results, which is why nothing caught it; the tile is simply carved from the wrong ground. This is the more dangerous of the two failures because its output is finite and plausible.

9. **It interacted with the ocean mask, and the interaction was measured.** A non-finite height fails `h ≤ sea`, so a NaN cell is classified **not submerged** — under #form-ocean-is-connectivity-not-elevation it is therefore a *wall* in the reachability flood, able to enclose submerged ground that is not enclosed. Measured across the cohort boundary: non-finite **cells** rise from 37 948 to **55 391** (+46%) in the first cohort carved under connectivity, with the affected root count unchanged at 83 — consistent with NaN walls creating spurious basins whose fill then propagates NaN through the spill maximum.

## Epistemic Status

**Max attainable: `exact`** for FE(1)–(8) — each is a count or a closed-form geometric fact from a deterministic read of the store, reproducible by the two named instruments plus the tripwire `an_overhanging_window_has_no_zero_length_neighbour_pairs`.

**Currently `exact`** for those clauses. FE(9)'s +46% is exact as a measurement; its *attribution* to NaN-as-wall remains **inferred** and this repair could not decide it — with the mint gone there are no NaN walls left to measure, so the clause records a mechanism that existed rather than one now demonstrable. Recorded as undecidable-by-this-run rather than quietly upgraded.

FE(8) (the low-edge slide) is `exact` as a reading of the code and **unmeasured** as a consequence: nobody has quantified how wrong a slid tile's ground is. Stage `draft`.

## Discussion

The clamp's own comment is honest about being provisional and names the right open problem. What it did not anticipate is that a *defensive* clamp — chosen over a panic, which is the humane choice — would silently produce a **geometrically impossible** configuration rather than merely an inaccurate one. Zero distance between distinct cells is not an approximation of anything; it is outside the model. That is the general lesson worth carrying: clamping an index to keep a program alive is safe for *lookup* and unsafe for *metric*, because the metric is what the physics divides by.

It is also the night's recurring shape once more. One accessor served two callers with different rights (geometry and data), and the caller with fewer rights set the behaviour for both — the same structure as `ErodedRegion::surface_m` serving both seeding and depiction ( #form-fidelity-ladder FE(7)).

## Working Notes

- **The repair, in two independent parts.** (a) Compute `centers` and `cell_area` from the **unclamped** requested index while continuing to clamp the *data* lookup — this removes the degenerate metric and leaves the declared cross-face data gap exactly where it was. (b) Make the low-edge window **pad rather than slide**: `from_surface` would need signed origins, and the measure helpers signed indices, so the requested cell can be named even when it is off-chart on the low side. (a) stops the NaN; (b) stops the misalignment; neither substitutes for cross-face resampling.
- **Tripwire landed** as `an_overhanging_window_has_no_zero_length_neighbour_pairs`, and it is on the *geometric invariant* rather than on downstream finiteness: it reported 108 zero-length adjacent pairs before the split and zero after. A finiteness test would have passed the moment a division happened to be avoided. **Still owed:** a store-level guard, since nothing today would catch a non-finite payload arriving by some other route — which is why 83 roots survived seven cohorts.
- **Correction, 2026-07-30:** an earlier version of FE(5) reported *80* duplicate rows and a last-row request of $j = 591$. That came from a hand-built window origin in the probe rather than the builder's own formula; the real overhang is **16 rows** from origin 432. The mechanism, the 0.0 m against 19 395.8 m, and the census counts were unaffected — but the magnitude was published as measured when it was reconstructed, and the probe now derives origins from the carve's formula so the same slip cannot recur.
- **Not investigated:** the 28 `mantle-thermal` roots with one bad cell each. Different nomos, different shape, no hypothesis.
- **Statistics measured on a last-row or last-column tile before cohort `2dc664ed` are suspect** — 73 tiles carried non-finite cells and their drainage, χ and basin numbers were computed through them.
