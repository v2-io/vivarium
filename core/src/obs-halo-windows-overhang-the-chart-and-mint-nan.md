---
slug: obs-halo-windows-overhang-the-chart-and-mint-nan
type: observation
status: exact
stage: draft
depends:
  - obs-chart-edge-halo-clamps-to-the-face
  - form-same-level-halo-exchange
  - form-cellid-chunk-patch
  - form-ocean-is-connectivity-not-elevation
  - norm-probes-before-claims
---

# Halo windows overhang the cube chart, and the clamp that saves them from panicking mints NaN on one edge and misaligns the carve on the other

Every cohort in the store carries **83 `erosion-tile` roots with non-finite heights**, out of 3552, and they are **not** scattered: 73 of 73 distinct positions sit on a region perimeter and **none** in the interior. The inputs are clean — `initial-topography`, `uplift-tile` and `climate` are finite in every cell of every root — so erosion mints these from finite data, deterministically, at the chart's edge.

**This segment is the measured defect and its repair; the clamp itself is owned by #obs-chart-edge-halo-clamps-to-the-face , which declared it as a boundary contract in force before any of this was measured.** What that segment did not distinguish — and what this one adds — is that clamping a cell's *position* is categorically unlike clamping its *height*: the first is outside the model rather than merely inaccurate.

The cause is one clamp doing two jobs. A halo window on a region perimeter asks for cells outside the cube chart; `Fluvial::from_surface` clamps the index (*"Clamp rather than panic: true cube-edge resampling for d≥2 is still open"*), and that clamp is applied to the cell's **geometry** as well as to its **data**. Clamped cells therefore share an $(i,j)$, share a centre vector, and sit at distance **zero** from a real neighbour — so any slope or flux that divides by that distance is $\Delta h / 0$.

## Formal Expression

1. **Measured: the census, and the high-edge mint is now repaired.** `examples/nan_census` over the whole store. The **current cohort holds zero non-finite `erosion-tile` cells** (0 roots of 3552, 0 positions) since the geometry split landed; the figures below are the seven cohorts carved before it, which remain in the store as archaeology:

   | nomos | roots | roots with non-finite cells | non-finite cells |
   |---|---|---|---|
   | `erosion-tile` (pre-repair cohorts) | 88 926 | 581 | 300 522 |
   | `initial-topography` · `uplift-tile` · `climate` · `water-tile` · `mantle-thermal` | 38 644 | **0** | **0** |
   | `hydrosphere` | 27 | *not decoded* | *encoding unknown to the probe* |

   **The halo geometry was the only genuine non-finite defect in this store.** An earlier version of the census reported 29 corrupt `mantle-thermal` roots; that was the instrument's own error — `epoch-reduction` payloads are four `f64`s (`EpochReduction::to_bytes`, 32 bytes) and the probe decoded them as eight `f32`s, producing plausible garbage including a NaN bit pattern. Decoded correctly they are clean in every cell. The probe now selects an encoding per nomos and reports anything it does not recognise as **undecoded** rather than scanning it under an assumption, which is why `hydrosphere` appears as a gap rather than as a pass.

2. **Measured: it was the same 83 roots in every cohort, and is now none.** Per `src=`, seven consecutive pre-repair cohorts each report 3552 `erosion-tile` roots and exactly **83** with non-finite cells — deterministic, not a flake. The cohort carved after the split reports **0**, which is what convicts the zero-distance metric as the *only* mint on this path.

3. **Measured: perimeter only.** Of the current cohort's distinct `(face, oi, oj, level)` positions holding non-finite cells, **73 are on a region perimeter and 0 are interior**. Within one position the count grows monotonically along the stage chain (257 → 686 → 1093 → 1561 → 1917 → 2136 → 2215 → 2513), so the corruption starts at the edge and spreads through routing.

4. **Measured: the plain carve is clean.** The same footprint (face 0, L9, `(128,448)`, 64², 40 epochs), carved through the per-tile path on its own stored inputs, stays finite for every epoch (`examples/nan_origin`). The mint is in the **region/halo path**, and every convicted root carries `edge=halo d=16`.

5. **Measured: the degenerate geometry.** The region carve computes window origins as `region_oj + tj·tile_n − d`, which for a whole-face L9 region at $d = 16$ gives $\{-16, 48, \ldots, 432\}$ with span 96. The highest window therefore spans $432 \ldots 527$ against a chart whose last index is 511: **17 rows land on $j = 511$, i.e. 16 duplicate rows.** Two clamped cells share a centre vector exactly, so they sit **0.0 m** apart while a genuinely adjacent pair at that level is **19 395.8 m** apart. Division by that distance is the mint.

6. **The geometry never needed clamping.** `measure::cell_center_unit` is an analytic equiangular formula and **extrapolates cleanly past the chart edge**: over the whole range the halo actually asks for (16 rows out, $j = 527$) it returns unit vectors with smoothly growing arc separations (19 395.8 m at the edge → 19 622.3 m at $j = 527$), and `measure::cell_area_m2` returns finite positive areas throughout (283.9 → 274.4 Mm²). So the clamp conflates two things that are not alike: **data** beyond the chart genuinely requires cross-face resampling and is open work ( #form-cellid-chunk-patch ); **geometry** beyond the chart is available in closed form for free.

   The extrapolated direction is *not* the true neighbouring-face cell centre — each face has its own parametrisation, so continuing this one past its edge lands near but not on the neighbour's cell. It is a smooth, non-degenerate, slightly distorted continuation, and it stays well inside the tangent domain over the range needed (the extended parameter reaches ≈1.06 of the face half-width at $d = 16$). Strictly better than a zero-distance impossibility, and not a substitute for real cross-face geometry.

7. **Measured: what the repair moved.** Computing `centers` and `cell_area` from the **unclamped** requested index — while the height still reads from the clamped cell — changed **655 of 3552** `erosion-tile` payloads (18.4%) and **94 of 384** `water-tile` payloads, with `initial-topography`, `climate`, `uplift-tile`, `hydrosphere` and `mantle-thermal` bit-identical. 655 roots over a ~9-stage chain is ≈73 tiles, which is **exactly the 73 positions that held non-finite cells**: the repair moved the corrupted set and nothing else.

   Only the *last* row and column of tiles overhang at all. Of the eight window origins $\{-16, 48, \ldots, 432\}$ only 432 exceeds the chart; origins 48–368 span at most 463 < 511. So the affected fraction is one row plus one column per face, not a four-sided perimeter.

8. **The low edge failed differently and silently; repaired 2026-07-30.** The region carve computes window origins as `region_oi + ti·tile_n − d`, negative for a low-edge tile, and the builder clamped it to zero because the constructor could not express a negative origin. That slid the window rather than padding it, so the tile's interior left halo offset $d$ and the publish step wrote ground from a **16-cell offset** — finite, plausible, and therefore silent, which is why it outlived the NaN beside it. Origins are now signed through `Fluvial::from_surface_at`, `uplift::uplift_rate_tile_at` and the `measure::*_i` geometry core, with **every field in a window taking the same origin**: padding heights alone would have put terrain and its uplift driver $d$ cells out of register, a defect that did not previously exist. Convicted on *position* by `a_padded_window_places_the_tile_origin_at_the_halo_offset`, whose second arm shows the clamped alternative puts a different cell at the halo offset. Measured: 22.6% of `erosion-tile` payloads moved, the mirror of the high edge's 18.4% (changelog `2026-07-30-03`).

9. **It interacted with the ocean mask, and the interaction was measured.** A non-finite height fails `h ≤ sea`, so a NaN cell is classified **not submerged** — under #form-ocean-is-connectivity-not-elevation it is therefore a *wall* in the reachability flood, able to enclose submerged ground that is not enclosed. Measured across the cohort boundary: non-finite **cells** rise from 37 948 to **55 391** (+46%) in the first cohort carved under connectivity, with the affected root count unchanged at 83 — consistent with NaN walls creating spurious basins whose fill then propagates NaN through the spill maximum.

## Epistemic Status

**Max attainable: `exact`** for FE(1)–(8) — each is a count or a closed-form geometric fact from a deterministic read of the store, reproducible by the two named instruments plus the tripwire `an_overhanging_window_has_no_zero_length_neighbour_pairs`.

**Currently `exact`** for those clauses. FE(9)'s +46% is exact as a measurement; its *attribution* to NaN-as-wall remains **inferred** and this repair could not decide it — with the mint gone there are no NaN walls left to measure, so the clause records a mechanism that existed rather than one now demonstrable. Recorded as undecidable-by-this-run rather than quietly upgraded.

FE(8) (the low-edge slide) is `exact` as a reading of the code and **unmeasured** as a consequence: nobody has quantified how wrong a slid tile's ground is. Stage `draft`.

## Discussion

The clamp's own comment is honest about being provisional and names the right open problem. What it did not anticipate is that a *defensive* clamp — chosen over a panic, which is the humane choice — would silently produce a **geometrically impossible** configuration rather than merely an inaccurate one. Zero distance between distinct cells is not an approximation of anything; it is outside the model. That is the general lesson worth carrying: clamping an index to keep a program alive is safe for *lookup* and unsafe for *metric*, because the metric is what the physics divides by.

It is also the night's recurring shape once more. One accessor served two callers with different rights (geometry and data), and the caller with fewer rights set the behaviour for both — the same structure as `ErodedRegion::surface_m` serving both seeding and depiction ( #form-fidelity-ladder FE(7)).

## Working Notes

- **Both halves of the repair have landed** — unclamped geometry on the high edge, signed origins on the low — and **neither substitutes for cross-face resampling**. Off-chart cells are now *addressed* honestly on both edges while still taking their height from the clamped in-chart cell; sampling the neighbouring face is the open work ( #form-cellid-chunk-patch ).
- **Tripwire landed** as `an_overhanging_window_has_no_zero_length_neighbour_pairs`, and it is on the *geometric invariant* rather than on downstream finiteness: it reported 108 zero-length adjacent pairs before the split and zero after. A finiteness test would have passed the moment a division happened to be avoided. **Still owed:** a store-level guard, since nothing today would catch a non-finite payload arriving by some other route — which is why 83 roots survived seven cohorts.
- The instruments derive window origins from the carve's own formula rather than reconstructing them, because a hand-built origin reports the wrong overhang.
- **Owed:** teach the census `hydrosphere`'s encoding, the one payload shape it currently cannot check. Until then its 27 roots are unverified, not verified-clean.
- **Statistics measured on a last-row or last-column tile before cohort `2dc664ed` are suspect** — 73 tiles carried non-finite cells and their drainage, χ and basin numbers were computed through them.
