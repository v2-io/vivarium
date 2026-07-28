# A drainage reader landed in vivarium-world, if the explorer wants rivers (2026-07-28)

*Coordination note, not claim canon and not a work order — the claims are in `#obs-tile-outlets-grade-away-the-basins` and `DECISIONS[drainage-is-a-reader-of-a-stage-not-a-store-citizen]`. Written for whoever is building region rendering and paint modes in `crates/vivarium-explore`; that crate is yours and nothing here touched it.*

## What exists now

`Fluvial::drainage_surface(&mut self) -> DrainageSurface` in `crates/vivarium-world/src/erosion.rs`. It recovers the discharge field the kernel computes every epoch and discards. Same shape as `response_census`: it takes a stored stage's heights, does one fill + receivers + sort + two accumulations, restores the heights, and hands back

- `mfd: Vec<f32>` — MFD drainage area (m²), **the kernel's own field**, and it is *diffused*;
- `d8: Vec<f32>` — single-receiver accumulation down the same tree, which concentrates into threads;
- `recv`, `filled_h`, `fill_depth` — the tree, the surface routing was derived on, and per-cell depression depth;
- `stats` — subaerial count, land runoff, basins / largest-basin share / basins-for-half, channel cells at three thresholds under both routers, `spread_ratio`, `mean_out_degree`, straight-run length, depression cells / volume / depth;
- `in_cells(field)` — the field divided by the tile's median cell area, i.e. "cells drained", which is the only form in which two tiles at different levels compare and is what the thresholds are stated in.

Typical use over a stored region, no store write, ~milliseconds for 64²:

```rust
let mut f = Fluvial::from_region(&region);
f.set_precip_weight(weights);          // see the trap below
let d = f.drainage_surface();
let cells = d.in_cells(&d.mfd);        // paint this
```

`examples/discharge_probe` is a worked end-to-end consumer (read-only, seven parts, ASCII renders of both routers over the L13 beacon patch) if seeing one is faster than reading the docs.

## Three things I would want to know before painting it

**1. MFD and D8 answer different questions and neither is "the river."** MFD at p=1.0 spreads over every downhill neighbour — measured discharge-weighted out-degree **1.96–2.30** across all 16 beacon tiles. So a one-cell-wide painted thread over the MFD field is a two-cell fan drawn thin, and `#norm-no-depiction-without-referent` is exactly about that. D8 concentrates but carries the grid-alignment artifact MFD was adopted to dissolve, and it is *not* what the kernel erodes with. `spread_ratio` (max D8 / max MFD, measured 0.56–1.01) is the gap, per tile, if you want the picture to declare which it is showing. My own instinct after looking at the ASCII was that the honest paint is a **width or intensity ramp on MFD** with the router named on the HUD — but you will see the tradeoff better against a real mesh than I can from a terminal, and I would rather you overrule me than inherit my guess.

**2. There is a live trap in `Fluvial::from_region`.** It rebuilds a field from stored heights alone and leaves `precip_weight` at ones — uniform rain, which is *not* what the kernel ran. Climate carries fated ±50% low-frequency jitter. At the beacon's 313 km span it is worth under 1% (the jitter's features are ~1000 km, so it is near-constant across a patch after normalizing by the tile mean), but that is a property of the span, not a safe default. The weight is a pure function of seed and cell, so you can build it without touching the store:

```rust
let w = climate::precip_jitter_factor(seed, cell) as f32;  // then divide by the tile mean
```

`response_census` has the same gap; its published Courant / response numbers are uniform-rain numbers.

**3. Per-tile and assembled routing give different worlds, and the difference is large.** Every builder tile is carved with its edges as outlets, so no flow path crosses a tile seam. Routing the assembled 4×4 beacon patch as one field gives a trunk **3.85× larger**, and turns 0 depression cells into 8 532 (13% of the patch, deepest 411 m) because adjacent tiles graded to different perimeter heights. Both fields are honest and they are honest about different things: per-tile is what the kernel used, assembled is where water would flow on the surface the store holds. If the explorer paints one, the picture should probably say which — I think per-tile is the one that matches the carving, and assembled is the one that matches the terrain a viewer is looking at, which is an uncomfortable split and may be worth surfacing to Joseph rather than either of us deciding quietly.

## The thing the pictures already say

At full tile resolution the MFD field reads as drainage — branching trunks, dendritic, legible. Downsampled with a **max-pool** it still reads; downsampled with a mean it would not, because a mean dissolves the thread it is drawn to show. That is a real choice a mesh-side consumer has to make too, and it is worth declaring wherever it lands.

Across the beacon's 30 stored stages the network does **not** integrate — largest-basin share oscillates 32.6–45.8% with no trend and channel counts hold flat from epoch 10 to 300 (FE(6) of the segment, and FE(4) says why: base level is 32 cells away). So a scrub of the erosion chain painted with discharge will show incision *deepening*, not drainage *organizing*. Worth knowing before building a scrub whose caption would otherwise overclaim.

## Feedback wanted

If the reader is the wrong shape for a per-frame consumer — if you need it keyed, batched over many tiles, or returning fewer allocations — say so and I will change it rather than have you wrap it. The recompute-not-memoize call is recorded with its reasoning in the decision row precisely so it can be revisited: the condition that would flip it is a cold recompute a view pays *per frame*, and you are the one who will find out whether that is true.
