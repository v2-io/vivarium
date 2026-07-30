# Eighty-three corrupt tiles had been there all along, and the clamp that saved a panic was the cause

*2026-07-30. Frozen. World: `first-light`, seed `17425063241017297386` (`0xf1d242b21d8d89ea`), store `~/.cache/vivarium/globe-world`.*

**Pre-registration** (`msc/lake-connectivity-2026-07-29-prereg.md` §2, before the rebuild): non-finite erosion cells go to zero; **43.75–100%** of erosion payloads change; water changes where its bed did; `mantle-thermal` untouched. Scored below — one hit, one miss low with a clean cause, and one prediction recorded as *undecidable by this run* rather than quietly claimed.

---

## Provenance (required fields)

| Field | Before | After |
| --- | --- | --- |
| **Kernel / memo `src=`** | `d3ec5278bca48314` | **`2dc664edab5e7f9d`** |
| **World-dir git** | `711e68f9a` (pre-rebuild, committed clean) | **`bd9604cb7`** |
| **Source repo** | `19fd43d` (diagnosis) | this entry's commit (repair) |
| **Window** | whole globe, view L8 over the L13 beacon build | same |
| **Commands** | `bin/install vivarium` · `vivarium build` · `cargo run --release -p vivarium-world --example nan_census` | same |

No capture. The change is invisible on screen by construction — corrupt cells were already being drawn as plausible terrain, which is the point of the entry.

---

## What was wrong

Every cohort in this store carried **83 `erosion-tile` roots with non-finite heights**, out of 3552 — the same 83, in seven consecutive cohorts. Every *input* nomos was clean in every cell: `initial-topography`, `uplift-tile`, `climate`, `water-tile`, `hydrosphere`, all zero. So erosion was minting NaN from finite data, deterministically, and had been for as long as the store remembers.

They were not scattered. **73 of 73 distinct positions sat on a region perimeter; none in the interior.** Within one position the count grew monotonically along the stage chain (257 → 686 → … → 2513): it started at the edge and spread through routing.

**The cause was one clamp doing two jobs.** A halo window on a region perimeter asks for cells outside the cube chart. `Fluvial::from_surface` clamps the index — its own comment says *"Clamp rather than panic: true cube-edge resampling for d≥2 is still open"* — and that clamp reached the cell's **geometry** as well as its **data**. The highest window at L9 spans 432…527 against a last index of 511, so **16 rows collapse onto row 511**. Clamped cells share a centre vector *exactly*, so distinct cells sit **0.0 m** apart where a real neighbour is **19 395.8 m** away. Every slope and flux divides by that distance.

Zero distance between distinct cells is not an inaccuracy. It is outside the model — and a defensive clamp, chosen over a panic, is what produced it.

## What changed

The clamp now governs the **data lookup only**. Heights still read from the clamped cell, because heights beyond the chart genuinely are not available yet (cross-face resampling is open work). Geometry is computed from the **unclamped** requested index, because the equiangular formulae extrapolate: over the 16 rows the halo asks for, `cell_center_unit` returns unit vectors with smoothly growing separations (19 395.8 → 19 622.3 m) and `cell_area_m2` returns finite positive areas (283.9 → 274.4 Mm²).

The extrapolated centre is *near*, not identical to, the true neighbouring-face cell — each face carries its own parametrisation. It is a smooth, slightly distorted continuation, and it is not a substitute for real cross-face geometry. It is what makes the metric possible at all.

**The tripwire is on the invariant, not the symptom.** `an_overhanging_window_has_no_zero_length_neighbour_pairs` reported **108** zero-length adjacent pairs before the split and zero after. A test for finite heights would have passed the moment a division happened to be avoided.

## Scoring the pre-registration

| | prediction | result |
| --- | --- | --- |
| P5 | non-finite erosion cells → 0 | **hit** — 83 roots → **0**, 0 positions |
| P6 | 43.75–100% of erosion payloads change | **MISS, low** — **18.4%** (655/3552) |
| P7 | water changes where its bed did | hit — 94/384 changed |
| P8 | `mantle-thermal` untouched | **hit, and sharpened** — 28 → **29** bad roots; still minting, still unexplained |

**P6's miss has a clean cause and it confirms the diagnosis rather than denting it.** I called the affected set "the perimeter," but of the eight window origins `{-16, 48, …, 432}` only **432** overhangs; origins 48–368 span at most 463 < 511. And the low-edge origin `−16` is clamped to 0 *before* `from_surface`, so the geometry split never sees an out-of-chart request there. So the affected set is one row plus one column per face — and 655 roots over a ~9-stage chain is ≈**73 tiles, exactly the 73 positions that held non-finite cells.** The repair moved the corrupted set and nothing else.

## What is still wrong

- **The low edge still slides instead of padding.** A negative window origin is clamped to 0, so the window covers different ground than requested and its interior is no longer at halo offset `d` — publish then writes ground from a 16-cell offset into that tile. **Finite, plausible, wrong**, and nothing catches it because there is no NaN to trip on. This is the more dangerous of the two failures and it is untouched here. Fixing it needs signed origins through `from_surface` and the measure helpers.
- **Nothing guards the store against a non-finite payload** arriving by another route. The new tripwire covers this geometry; it does not cover the store.
- **Statistics measured on a last-row or last-column tile before `2dc664ed` are suspect.** 73 tiles carried non-finite cells and their drainage, χ and basin numbers were computed through them.
- **The +46% NaN inflation** attributed to NaN-acting-as-a-wall under the new ocean mask stays **inferred**. With the mint gone there are no NaN walls left to measure, so this rebuild could not decide it, and it is recorded as undecidable rather than upgraded.

---

## Correction, appended same day (the entry body above is frozen; this is the amendment)

**The `mantle-thermal` defect this entry reported does not exist.** Both the P8 row and the "still wrong" bullet claimed 28 → 29 corrupt `mantle-thermal` roots, actively minting. That was my instrument's error, not the world's: `epoch-reduction` payloads are four `f64`s (`EpochReduction::to_bytes`, 32 bytes) and `examples/nan_census` decoded every payload as `f32` regardless, turning a valid struct into plausible garbage that included a NaN bit pattern. Decoded correctly, `mantle-thermal` is finite in all 6828 cells of all 1707 roots.

So P8 is **void, not hit** — it predicted that a defect would be untouched, and there was no defect.

The corrected census also reports `hydrosphere` (27 roots) as **undecoded**, because the probe does not know its encoding. That is the honest state: those roots are *unverified*, not verified-clean, and an earlier run silently counted them as clean.

**The finding this entry is actually about is unaffected**: the halo-geometry NaN was real, was 83 roots per cohort across seven cohorts, and is now zero. It was the only genuine non-finite defect in the store.

**The lesson is the instrument's, and it is the same shape as the night's other findings.** A census that assumes one encoding for every payload will manufacture defects in whatever does not match, and the manufactured ones look exactly like the real ones. The probe now selects an encoding per nomos and refuses to scan what it cannot identify.
