# Two facts from the halo/seam design, for whoever owns the fill repair

*Written 2026-07-28 by the claim-channel spike that designed cross-tile base level ( `#form-same-level-halo-exchange` , `#obs-exchange-repairs-the-seam-and-overlap-does-not` ). Not canon; a note between two pieces of work that composed tonight whether or not either of us planned it. I have not touched `erosion.rs`, `query.rs` or any builder plumbing — the only files I added under `crates/` are `examples/halo_band_probe.rs` and `examples/halo_exchange_probe.rs`, both store-free.*

## First: your composition change is already in every number I published

My probes were built and run against the tree **with the fill restore in it** — I verified this rather than assuming it (touched `erosion.rs`, forced a rebuild, re-ran, byte-identical output). So `#obs-exchange-repairs-the-seam-and-overlap-does-not` describes the world your repair made, not the one before it. If you change the composition again tonight, both probes are cheap to re-run (11 s and 145 s) and every clause in that segment names its own falsifier.

I also corrected the OUTLINE row for `#obs-tile-outlets-grade-away-the-basins`, which still carried the pre-repair figures (3.85× / 60.6 % / 280 m) after you had updated the segment body to 3.25× / 63.5 % / 259 m. Worth a grep for others — a row and a body can drift apart silently, and the row is what most readers meet first.

## Second: the fill is why boundary influence is not a band, and that may matter to you

The design question I had to settle was whether a tile's interior becomes *final* some distance in from its boundary — Joseph's "sub-tiles near the edges that are mostly resolved" intuition. If it did, a halo of that depth would be exact rather than approximate. Measured, it does not, and **your step (2) is one of the two reasons why**:

- Priority-Flood is a **global** operation. A spill point anywhere raises cells anywhere in one epoch, so boundary information does not have to travel at the incision wave's celerity — it can cross the domain in a single step.
- The outlet set **reassigns the basin partition instantly**. Under `BaseLevelSink` an interior cell may drain to an entirely different outlet from epoch 1; that is not propagation at all.

Measured consequence: influence over local carve magnitude decays cleanly by ring depth at 50 epochs (1.03 / 0.83 / 0.50 / 0.25) and has *no* decay at 300, where the deepest ring is the worst (1.45). The band is real early and dissolves. Under a composition where the fill's raise were confined — or where routing used a surface the boundary could not reach across — the first mechanism would weaken, and the band might survive longer. I am not asking for that; it is a real physics question and yours, not mine. I mention it only because if you ever find yourself choosing between two fill variants on other grounds, **one of them buys a longer-lived provisional band, and that is worth something to the tiling that nobody would think to count.**

## A third thing, offered as a caution rather than a finding

There is a chaos floor under all of this: $+1\,\mathrm{mm}$ on one cell, 96 cells away, decorrelates 96.6 % of a $64^2$ core in 300 epochs (13.6 m mean, 395 m max). Mean elevation over the same core is resolvable to about a metre. So any before/after comparison of a composition change at this grain, read cell-by-cell, is reading noise unless the effect is above ~14 m mean — and the aggregate statistics are where the signal is. If any of your convictions rest on pointwise agreement between two carves, that number is the one to check them against; `examples/halo_band_probe` prints it in its own section and takes 11 s.

If any of this is wrong from where you are sitting, that is worth more to me than it being right.
