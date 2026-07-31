# The first tile of every region was carving ground sixteen cells away

*2026-07-30. Frozen.*

**Pre-registration** (`msc/lake-connectivity-2026-07-29-prereg.md` §3, before the rebuild): 15–30% of erosion payloads change; water changes in proportion; the input nomos are untouched; no new non-finite cells. **All four hit** — the first clean sweep of the session, and the reason is that the range was reasoned from the builder's own origin arithmetic rather than from a picture of it.

*World: `first-light`, seed `17425063241017297386`, store `/Users/josephwecker-v2/.cache/vivarium/globe-world`. Declared order `terrestris`, target phase `4`.*

## Provenance (required fields)

| Field | Before | After |
| --- | --- | --- |
| **Kernel / memo `src=`** | `8154563432cd9e08` | **`925de1ce5820fb9d`** (4786 roots) |
| **World-dir git** | `d417bc418` | **`0896fa3b6`** |
| **Source repo** | `a7d522d` | this entry's commit |
| **Window** | whole globe, L9 region carves, 8×8 tiles per face | same |
| **Commands** | `bin/install vivarium` · `vivarium build` · `cargo run --release -p vivarium-world --example nan_census` | same |


*(Provenance block produced by `bin/provenance`; no digest in it was typed by hand.)*

---

## What was wrong

A halo window's origin is `region_o + t·tile_n − d`, which is **negative for the first tile in each axis**. The builder passed `oi.max(0) as u32`, because the constructor could not express a negative origin. That does not pad the window — it **slides** it. The tile's interior no longer sits at halo offset `d`, so the publish step wrote ground from **16 cells away** into that tile.

It produced no NaN, no discontinuity, and nothing a test of values could catch: a plausible landscape, in the wrong place. That is why it outlived the NaN it sat beside — entry `2026-07-30-01` repaired the loud sibling on the *high* edge and left this one, and the segment that named it called it the more dangerous of the two.

**The window was also internally inconsistent.** The precip-weight loop already addressed cells with the signed origin, while heights and uplift used the clamped one — so within a single window, rain was aligned to the true origin and terrain was not.

## What changed

`Fluvial::from_surface_at` and `uplift::uplift_rate_tile_at` take a signed origin; `measure`'s `corner_uv_i` / `cell_solid_angle_i` / `cell_area_m2_i` / `cell_center_unit_i` widen the geometry core to `i64` (the unsigned forms now delegate, so no existing caller changed). Off-chart cells still take their *height* from the clamped in-chart cell — cross-face resampling remains open — while geometry comes from the signed index.

**Every field in a window takes the same origin.** Repairing heights alone would have put terrain and its uplift driver `d` cells out of register, which is worse than both being slid together — a partial repair here creates a defect that did not previously exist.

## What convicts it

`a_padded_window_places_the_tile_origin_at_the_halo_offset` checks **position**, not values, because position is the only place the difference is visible. It asserts the cell at window offset `(d,d)` is the tile's own origin cell, and — the arm that makes it a conviction rather than a restatement — that the clamped alternative puts a *different* cell there.

## Measured

| | prediction | result |
|---|---|---|
| P10 | 15–30% of `erosion-tile` payloads change | **22.6%** (803/3552) |
| P11 | `water-tile` changes in proportion | 102/384 (26.6%) |
| P12 | `initial-topography`, `climate`, `uplift-tile`, `mantle-thermal`, `hydrosphere` unchanged | all bit-identical |
| P13 | no new non-finite cells | **0 bad roots, 0 bad cells** |

22.6% is the mirror of the high-edge repair's 18.4%: 15 of 64 tiles per face touch the low edge (8 + 8 − 1) against the 15 that touch the high edge, plus exchange propagation.

## What is still wrong

- **Off-chart heights are still the clamped in-chart cell's.** Both edges now address honestly; neither *samples* the neighbouring face. That is the open cross-face resampling work ( #form-cellid-chunk-patch ), and this entry does not touch it.
- **`erosion::column_at` still builds columns from the detailed surface** (entry `2026-07-30-02`).
- **`hydrosphere`'s payload encoding is unknown to `nan_census`** — 29 roots reported as undecoded rather than clean.
