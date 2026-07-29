# PREDICTIONS — cross-face seam machinery (written before the first run)

*Spike question: what does it take to make (a) a halo fill that crosses a
cube-face boundary with the correct axis transform, and (b) a face-keyed flux
object as a real store citizen, work together on the live
`sphere.rs`/`chunk.rs`/`store.rs`/`measure.rs` substrate — with a probe that can
convict?*

These are committed **before running**, per `#norm-probe-sensitivity` ("state
what magnitude of violation the probe would have caught; run known-bad first")
and imitating `msc/spike-wavelet-store/RUN.txt`'s predictions-then-results
shape. The whole point of writing them first is to be able to be wrong.

## The central hypothesis

The "face axis transform" that `form-cellid-chunk-patch` FE(4) names as designed
-but-unbuilt is **already latent in the projection**. `spikes/globe`'s
`cell_value` re-homes an out-of-face ghost by `to_unit` (tan runs past the face
edge) → `from_unit` (re-homes the direction onto the neighbour face), landing —
by the equiangular identity `tan(π/4+x)·tan(π/4−x)=1` — exactly half a cell
inside the neighbour, where the face choice is unambiguous. If that is right,
cross-face halo fill needs **no hand-tabulated 24-edge adjacency table**: the
transform is the round-trip, and its correctness is checkable by *involution*
and *geometry*, not by re-deriving it.

I expect the design's real hole to be **halo depth**, not the transform: depth-1
adjacency is exact, but the two face grids are co-aligned *only on the shared
edge*, so a halo of depth ≥ 2 has no exact cross-face cell correspondence.

## Per-probe predictions

**P0 — seam is real (guard).** The chosen footprint straddles the ZPos↔XPos cube
edge (ZPos east edge, u→+1, re-homes to XPos). Both faces evaluate; the two
abutting cells are on *different* faces. A guard that can't fail is not a guard,
so this also asserts the naive same-face fill leaves the out-of-face halo at
default (0.0) — today's `chunk.rs` behaviour.

**P1 — the transform is latent (depth-1 involution + geometry).**
For every cell along the shared edge, round-trip A→B→A.
- *Involution*: returns the original `CellId` **bit-exact** for all edge cells.
- *Geometry*: the cross-face centre distance is a genuine adjacency — ratio to
  the within-face neighbour spacing within a few % of 1 (both ~equal-angle at
  the same level).
- *Known-bad*: a "clamp to edge" transform (no re-home) gives centre distance
  ≈ 0 and fails involution — the probe discriminates.
Prediction: involution exact on all four ZPos edges; distance ratio 0.97–1.03.

**P2 — halo depth is where it breaks (the likely hole).**
Compare, for each edge cell, "extrapolate 2 cells off A" against "extrapolate 1
off A, then step 1 along B's own grid." At depth 1 these agree by construction;
at depth 2 I predict they **diverge by ≥1 cell for a nonzero fraction** of edge
cells (worst near the cube corner), because the grids are co-aligned only on the
edge. Rough guess: depth-2 mismatch on 10–40% of edge cells, growing toward the
corner; depth-3 worse. If instead they agree everywhere, that's a real
surprise and a stronger result than I expect.

**P3 — face-flux identity is canonical, symmetric, and a store citizen.**
A cross-face adjacent pair (idA on ZPos, idB on XPos) yields **one** canonical
face key from either side (symmetric in the unordered pair), owner = lower
`CellId` (global `Ord`: lower face index wins — well-defined *across* faces).
Persisted through the real `store.rs` (`Key`/`put`/`get`), it survives reopen
and the census enumerates it by meaning.
- *Known-bad*: an ownership rule keyed on per-face-local `(i,j)` disagrees across
  the seam (both sides could claim index 0) — the cross-face pair exposes it.
Prediction: symmetric key holds; store round-trips; local-index rule collides.

**P4 — conservation across a real cross-face seam (the payoff).**
Closed box, synthetic conserved scalar, conservative diffusion, **no outlets** —
any mass drift is manufactured at the seam.
- **4a (known-bad, today):** out-of-face halo left at default 0 (today's
  `chunk.rs`). The seam acts as a Dirichlet-0 sink. Predict a **large, growing**
  mass loss (order 1e-2..1e-1 relative over 1000 steps) — this convicts the
  FE(4) gap as a physical, not cosmetic, defect.
- **4b (matching same-level cross-face seam):** round-trip halo, TwoSided vs
  SingleValued. Honest prediction I am unsure of: on a **matching** seam with
  symmetric geometry, *both* may conserve to ~machine epsilon — because the two
  sides read the exact same neighbour value and the shared edge length /
  centre distance are symmetric. If so, the finding is that single-valued's
  value on matching seams is the **data-structure guarantee**, not a numeric
  win. (I'll be watching whether small geometric asymmetry across the seam
  makes TwoSided drift even here — if it does, ~1e-9..1e-7.)
- **4c (cross-face hanging node — coarse ZPos abuts fine XPos):** this is
  `spike-wavelet-store` PROBE 7 lifted onto a genuine cube edge (that probe was
  *same-face*). The shared cube edge subdivides equal-angle on both faces, so a
  coarse edge = exactly 2 fine sub-edges — a clean nested hanging node.
  Predict TwoSided **leaks and grows** (bias, ~1e-8→1e-5 over 1..1000 steps);
  SingleValued **flat at machine epsilon** (~1e-16) at every step count, with no
  correction pass. This is the conviction the face register earns.

**P5 — continuity, known-bad first (`#norm-probe-sensitivity` §3).** A
cross-seam curvature/step statistic: measured across shrinking arcs it
**plateaus** on the default-0 halo (a true cliff) and **vanishes** once the
cross-face halo is filled (honest continuous variation). Predict the default-0
halo shows a step that does not shrink with the arc; the filled halo shows one
that scales down with the arc.

## What would make this spike a success even if I'm wrong

If P2 shows depth-2 halos *do* align, or P4c shows single-valued is *not* needed
across faces, or the round-trip transform fails involution somewhere (a corner
pathology) — any of those is a design-clarifying finding, not a failure. The
named tension I am *not* resolving: leaf-only evolution vs independently
memoised coarse tiers (`form-face-flux-register` FE(7)). P4c picks **double-
evolve with a single-valued register** (the register is exactly what makes that
lawful) and says so.
