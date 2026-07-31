# Pre-registration — the ocean-connectivity rebuild (2026-07-29, evening)

Written **before** the rebuild, before looking at any rebuilt tile. Base repo rev
`6e4e919`. Not claim canon; the changelog entry that cites it is.

## What changed in law

`outlets` classifies ocean by connectivity to the domain boundary through
below-datum cells, rather than by `h <= sea`. Plus three datum repairs (water-tile
initial fill, ASCII globe waterline, module header) and the reader's
`standing_water` field.

## Predictions

**P1 — full recompute, no hits.** `SRC_HASH` moved, so every nomos key changes and
nothing in the previous store is readable. Expect 0 hits.

**P2 — the null-diff streak breaks, and only for tiles that hold BOTH land and
enclosed below-datum ground.** This is the sharp one. A fully submarine tile has
every cell submerged and every cell reaching its rim, so its outlet set is
unchanged. A fully subaerial tile has no submerged cell at all, so it is unchanged
too. Only a tile containing land *and* below-datum ground that cannot reach the
rim can differ. Since ~95% of the planet is submerged and the land is
concentrated, I predict a **small minority of tiles differ: 1–10%**, and that the
differing tiles are coastal or basin-interior rather than uniformly scattered.

If instead *most* tiles differ, my model of the change is wrong and I should look
for an unintended effect on the submarine interior — most likely the fallback
branch ("a walled domain with no coast makes its lowest cell an outlet") firing
where it did not before.

**P3 — every water tile differs.** The initial fill moves 1106.3 m on every
interior cell. There is no mechanism by which a water tile could come back
unchanged, so a single identical water tile would mean the fix did not reach the
built path.

**P4 — lakes appear in the depression paint where it was previously black**, in
coastal and basin regions specifically, because those cells were classified ocean
and are now landlocked.

## Known limitation carried into the rebuild, not repaired

Under `NoFluxWall` with no ocean in the domain, `outlets` still makes the single
**lowest** cell an outlet so Priority-Flood has a seed — and that cell is always
inside the deepest basin, so the deepest basin in a coastless walled window can
never hold water. The explorer sets `NoFluxWall` per drawn tile, so this affects
the paint on any drawn tile with no submerged cell touching its rim. Found while
building the connectivity tripwire (the first version of that test hit this
instead of the classification). Distinct defect, named, unrepaired.

---

# Pre-registration 2 — the halo-geometry repair (2026-07-30)

Written before the rebuild, before reading any rebuilt payload. Base `19fd43d` + the
`from_surface` geometry split.

**P5 — non-finite `erosion-tile` cells go to zero in the new cohort.** If any
remain, the zero-distance metric was not the only mint and the segment's FE(5)
is incomplete.

**P6 — only perimeter tiles change, plus their exchange neighbours.** The split
only alters cells where the requested index exceeds the chart, i.e. overhanging
windows, i.e. perimeter tiles: 28 of 64 per face = 43.75%. But the Jacobi
exchange shares halo bands, so a perimeter tile's corrected values can propagate
inward over rounds. So: **more than 43.75% and less than 100%** of `erosion-tile`
payloads change. If it comes back at exactly 43.75% the exchange is not
propagating what I think it propagates; if it comes back at 100% something
changed for interior windows, which the diff says it should not.

**P7 — `water-tile` payloads change wherever their bed did**, and not elsewhere.

**P8 — `mantle-thermal`'s 28 bad roots are untouched.** Different nomos, nothing
in this repair reaches it. If they clear, my localization was wrong about them
being a separate defect.

**P9 — the +46% attribution is testable here.** FE(8) inferred that NaN acts as a
wall in the ocean mask and inflates NaN cell counts. With the mint repaired there
are no NaN walls, so that clause loses its subject; what it predicted is that the
*mechanism existed*, which this rebuild cannot confirm retrospectively. Recording
honestly that P9 is **not** decidable by this run — the attribution stays inferred.

---

# Pre-registration 3 — pad, don't slide (2026-07-30)

Written before the rebuild. The high-edge repair moved 18.4% of erosion payloads
(the last row and column of tiles per face). The low edge is the mirror case:
only the **first** row and column have a negative origin.

**P10 — a similar fraction moves, and by the same arithmetic.** 15 of 64 tiles
per face touch the low edge (8 + 8 − 1), so ~23% before exchange propagation.
Predict **15–30%** of `erosion-tile` payloads change. Materially more would mean
the padding reaches windows I think are interior; materially less would mean the
slide was somehow already compensated.

**P11 — `water-tile` changes wherever its bed did**, roughly in proportion.

**P12 — stored `uplift-tile`, `initial-topography`, `climate`, `mantle-thermal`
unchanged.** The signed uplift path is used *inside* the carve; the stored
uplift nomos is a separate call that no longer routes through it.

**P13 — no new non-finite cells.** The geometry is now signed on both edges;
`nan_census` should still report 0 for the new cohort.

---

# Pre-registration 4 — resample the off-chart halo (2026-07-30)

Off-chart cells stop repeating the rim index and read the cell their own
extrapolated direction lands in. Written before the rebuild.

**P14 — 30–45% of `erosion-tile` payloads change.** Every tile whose window
overhangs is affected, which is the union of the two edge sets: 28 of 64 per
face by naive count (43.75%), and both previous edge repairs came in somewhat
under their naive count (high edge 23.4% naive → 18.4% measured), so the union
should land under 43.75% too. Above 45% would mean the resample is reaching
windows that do not overhang.

**P15 — still zero non-finite cells.** A resampled direction always lands in a
real cell, so nothing here can divide by a degenerate metric.

**P16 — `initial-topography` unchanged.** The resample happens inside the carve;
the stored topography nomos does not route through it.
