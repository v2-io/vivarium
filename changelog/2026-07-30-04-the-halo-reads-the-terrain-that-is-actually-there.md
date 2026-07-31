# The halo stops repeating the rim and reads the terrain that is actually there

*2026-07-30. Frozen.*

**Pre-registration** (`msc/lake-connectivity-2026-07-29-prereg.md` §4): 30–45% of erosion payloads change; still zero non-finite cells; `initial-topography` unchanged. **P15 and P16 hit; P14 missed low, at 7.0%** — and the miss is the entry's most useful number.

*World: `first-light`, seed `17425063241017297386`, store `/Users/josephwecker-v2/.cache/vivarium/globe-world`. Declared order `terrestris`, target phase `4`.*

## Provenance (required fields)

| Field | Before | After |
| --- | --- | --- |
| **Kernel / memo `src=`** | `925de1ce5820fb9d` | **`4fdc95afbe11c785`** (4786 roots) |
| **World-dir git** | `0896fa3b6` | **`f1b321a14`** |
| **Source repo** | `c9f1053` | this entry's commit |
| **Window** | whole globe, L9 region carves | same |
| **Commands** | `bin/install vivarium` · `vivarium build` · `example nan_census` | same |


*(Provenance from `bin/provenance`; no digest typed by hand.)*

---

## What changed

An off-chart halo cell used to take its height from the clamped in-chart cell — the rim, repeated. It now takes it from **the cell its own extrapolated centre lands in**, which is a real place on a neighbouring face.

This is the *declared resampling* that `#obs-chart-edge-halo-clamps-to-the-face` FE(2) says a $d \ge 2$ halo requires. That clause's measurement is what rules out the cheaper alternative rather than this one: copying neighbour-face **indices** mismatches on 200/256 edge cells at depth 2 and 232/256 by depth 4, because the two grids co-align only on the shared edge. A **direction** does not have that problem — it names a place, not a coordinate in someone else's chart.

It also closes an incoherence introduced by this session's own earlier repair. Once the geometry became honest, an off-chart cell carried an extrapolated *position* and a repeated rim *height*: the value described a different location than the coordinates did. Position and data now name the same point.

**Stated imprecision:** extrapolating one face's parametrisation past its edge distorts spacing, so far out the direction can land a cell off from an ideal correspondence. It is a real neighbouring place rather than exactly the right one, which is strictly better than a repeated rim and still not a resampling declared in the key.

## Scoring, and what the miss teaches

| | prediction | result |
|---|---|---|
| P14 | 30–45% of erosion payloads change | **MISS — 7.0%** (249/3552) |
| P15 | still zero non-finite cells | hit — 0 roots, 0 cells |
| P16 | `initial-topography` unchanged | hit; `climate`, `mantle-thermal`, `hydrosphere` too |

**P14 assumed that a window overhanging the chart implies its published output changes. It does not.** Every off-chart value in ~168 tiles' halo bands moved, and only **35 distinct tiles** produced different published heights — about 21% of those whose windows contain the changed cells. The halo influences the interior only where the difference propagates through routing far enough to reach the published cells, and with $d = 16$ under cone truncation it usually does not. **Overhang is necessary, not sufficient.**

That is a measurement about how much the halo band actually matters to a carve, which the halo design ( #form-same-level-halo-exchange ) has an interest in and which nothing had put a number on.

**And 25 of the 249 changed roots are on INTERIOR tiles**, whose own windows do not overhang at all. They changed because the Jacobi exchange shares halo bands between neighbours, so a perimeter tile's corrected values propagate inward. That mechanism was hypothesised in this session's first pre-registration to explain a different miss, could not be confirmed then, and is confirmed here: 224 perimeter roots, 25 interior.

## What is still wrong

- **The correspondence is approximate**, per the stated imprecision above; an exact cross-face resampling declared in the key remains open ( #form-cellid-chunk-patch ).
- `erosion::column_at` still builds columns from the detailed surface (entry `2026-07-30-02`).
- `hydrosphere`'s payload encoding is unknown to `nan_census`, so its roots are reported unverified rather than clean.
