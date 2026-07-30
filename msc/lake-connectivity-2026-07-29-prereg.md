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
