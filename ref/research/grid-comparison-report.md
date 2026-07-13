# Sphere grids, measured — the definitive comparison

*Written 2026-07-12. Nine grids, one measurement harness, every number reproducible by
`cargo run --release -p vivarium-world --example grid_lab`.*

> **Why this document exists.** Joseph, 2026-07-12: the conclusion was probably already
> settled, but the numbers had to be made **visible** — *"so future agents and humans can see
> exactly what the numbers say and distinguish between the metrics that matter to THEM."*
> A reader running a different algorithm will care about different metrics. **This report is
> built to let you find your metric and read off your answer, not to make you inherit ours.**
> §4 is a metric-by-metric index; every row says who should care about it and why.

**Epistemic tags, used inline and meant literally.**

| tag | meaning |
|---|---|
| **[M]** | **Measured** by `examples/grid_lab`. I ran it; the number reproduces; the code is committed. |
| **[P]** | **Primary** — quoted from a source I read directly (Snyder 1992 PDF; Górski et al. 2005). |
| **[D]** | **Derived** — a proof or closed-form done here, stated so you can check it. |
| **[I]** | **Inferred** — follows from measured things but was not itself measured. Treat as a claim, not a result. |
| **[⊘]** | **Could not resolve.** Collected in §10. |

Nothing here was implemented from memory. Where a construction is *ours* rather than a
source's, it says so in the row.

---

## 0. The bottom line, in one table

Every grid at ~24 500 cells (a 3 % spread), on the same sphere, under the same harness. **[M]**

| grid | area spread | non-orth (max) | valence | quadtree? | best accuracy achieved | conserves? |
|---|---|---|---|---|---|---|
| **cube · equiangular — OURS** | 1.397× | 28.79° | 4 (all) | **YES** 1→4 | **3.6e-4** | exact |
| cube · Snyder equal-area | **1.000×** | 15.04° | 4 (all) | **YES** 1→4 | 8.7e-4 | exact |
| cube · gnomonic *(control)* | 5.031× | 29.22° | 4 (all) | **YES** 1→4 | 6.8e-4 | exact |
| rhombic-dodec · gnomonic | 2.735× | 29.45° | 4 (all) | **YES** 1→4 | 5.1e-4 | exact |
| rhombic-dodec · tan-warp *(ours)* | 1.519× | 29.33° | 4 (all) | **YES** 1→4 | 5.7e-4 | exact |
| HEALPix | **1.000×** | 36.73° | 4 (all) | **YES** (NESTED) | 8.4e-4 | exact |
| icosa-tri (geodesic) | 1.931× | 10.05° | 3 (all) | **YES** 1→4 | 3.1e-3 | exact |
| icosa-hex (raw Voronoi dual) | 2.040× | **0.00°** | 6, 12×5 | **NO** | 3.0e-4 | exact |
| icosa-hex (SCVT, 200 Lloyd) | 1.736× | **0.00°** | 6, 12×5 | **NO** | **2.2e-4** | exact |

### The three findings that decide it

**1. Conservation is free, and it is not the question.** Finite volume conserves **exactly**
(~1e-15) on **every** grid here, including the worst one, because the flux across an edge is
literally the same number added to one cell and subtracted from the other. **[M]** No
projection buys it and none is needed.

**2. But conservation does not imply the scheme is solving the right equation — and this
refutes the decomposition the earlier passes landed on.** A **two-point flux** — the naive
meaning of "finite volume with the true geometry" — is **inconsistent** on a non-orthogonal
mesh. Its relative error is **O(1)** and it gets **worse with refinement** (convergence order
**−0.5** on every quad grid). **[M]** The previous framing — *"conservation is a scheme
property; isotropy is a grid property"* (`seam-adjacency-findings.md` §4.2, and
`DECISIONS[seam-grid-leads-snyder-healpix]`) — is **superseded**:

> **Conservation is a scheme property. CONSISTENCY is a property of the scheme and the grid
> TOGETHER.**

**3. And once the scheme is fixed, our grid is competitive with the hexagonal mesh.** Correct
the flux (project both cell centres onto the edge normal through the mid-edge) *and* widen the
gradient stencil (quadratic fit over the Moore neighbours), and our own equiangular cube-sphere
goes from **9.2e-1 → 3.6e-4**, a **2500× gain**, against the best hexagonal mesh's 2.2e-4.
**[M]** Still 1.7× behind — but the same order of magnitude, on the metric hexagons exist to
win, **with no change to `CellId`, the quadtree, the store, the Hilbert ordering, or the KRNG.**

### Recommendation

> ## Keep the equiangular cube-sphere. Change the kernels.
>
> The conclusion survives contact with measurement — but **not for the reason previously
> given**, and the corrective work is bigger and more specific than "carry the metrics."
> Details in §9.

---

## 1. What was built, and what proves it is trustworthy

Nine grids behind one `Mesh` type. Adjacency is **combinatorial** (derived from shared vertex
pairs), never geometric matching — the failure mode `seam-adjacency-findings.md` §3.5 names in
QuadSphere. Every mesh is checked at construction; **a topology bug cannot silently become a
physics result.**

| gate | result |
|---|---|
| our equiangular cube ≡ `sphere.rs::to_unit` | max drift **0.0** — it is the same map, not a lookalike **[M]** |
| Euler `V − E + F = 2`, every grid | asserted at construction; every edge borders exactly 2 cells **[M]** |
| Σ cell areas = 4πR², every grid | to < 1e-9 — all nine actually tessellate the sphere **[M]** |
| **Snyder reproduces its own Table 1** | measured **ω = 25.17°, a = 1.248, b = 0.801** vs paper's **ω = 25.17°, a = 1.248, b = 0.801** **[M vs P]** |
| Snyder equal-area residual | max \|σ₁σ₂ − 1\| = **1.1e-7** (finite-difference floor) — exactly equal-area **[M]** |
| Snyder forward ∘ inverse | max residual **2.0e-15** **[M]** |
| HEALPix ΣΩ / 4π | **1.000000000000**; N_pix = 12·N_side²; each base pixel gets exactly N_side² pixels **[M]** |

### 1.1 Three things the assertions caught that no argument would have

These are worth reading even if you skip the rest, because each is a trap that *looks* fine.

**(a) Snyder's Table 1 rounds a constant, and the rounding breaks the seam.** Table 1 prints
`g = 54.73561032°` for the cube. The exact value is `arctan √2 = 54.735610317…°` — and the paper
supplies that surd itself **[P]**: *"The values of tan g for the various polyhedra may be
expressed as quadratic surds, namely … √2 …"*. The 2.8e-9° rounding perturbs the spherical
excess by ~2.4e-11, the Newton solve for `Az` by ~1.6e-10, and therefore moves the image of the
face *boundary* off the cube edge's great circle — **so the six faces stop welding.** The seam
assertion caught it at 1.3e-10 **[M]**. *Use the surd.*

**(b) A gradient-projected router must be clamped to strictly-downhill edges.** The LSQ gradient
is a *fit*, so its outgoing component through an edge can be positive even when the neighbour
across that edge is **higher**. Route there and the mass goes upstream — and an elevation-ordered
sweep has already passed that cell, so it is stranded and never reaches an outlet. Measured
before the guard: conservation **0.000** instead of 1.000 **[M]**. The guard is one line, but
nothing in the *design* of the scheme tells you it is needed.

**(c) Fixed-topology Lloyd destroys the thing a hexagonal mesh exists for.** Moving the
generators without re-establishing Delaunay lets circumcentres wander outside their triangles;
the "Voronoi" cells built from them are no longer Voronoi cells, and the primal–dual
orthogonality silently dies. Measured: non-orthogonality went **0.66° → 3.70°** — Lloyd made the
grid *worse* on its own headline metric. With Delaunay edge flips it stays at **0.00°**, as the
duality theorem says it must **[M]**.

### 1.2 A structural fact that fell out, and is worth keeping

**HEALPix's base *is* the rhombic dodecahedron.** Not "like" it — *is* it, combinatorially.
Measured vertex census of the N_side = 1 grid **[M]**:

```
HEALPix BASE   : F 12   V 14   valences {3: 8, 4: 6}   Σ(4−valence) = 8
rhombic-dodec  : F 12   V 14   valences {3: 8, 4: 6}   Σ(4−valence) = 8
```

Identical. Both are 12 quadrilateral faces with 8 valence-3 vertices (at the cube-corner
directions) and 6 valence-4 vertices (at the axes). **[M]** So HEALPix is, in effect, *the
equal-area map on the rhombic-dodecahedral base that Snyder cannot give you* (§8).

---

## 2. Provenance of each grid

| grid | where the formulae came from |
|---|---|
| cube · equiangular | `sphere.rs::to_unit`. Verified identical, max drift 0.0 **[M]** |
| cube · gnomonic | radial projection. Snyder Table 1 tabulates its cube distortion as ω=31.08°, a=3.000 **[P]** |
| **cube · Snyder equal-area** | **Snyder 1992, Cartographica 29(1):10–21, eqs (1)–(23), transcribed from the PDF.** Forward (1)–(16), inverse (17)–(23) incl. the Newton–Raphson on `Az`. Constants from Table 1: `G = 60°, θ = 45°`, `g = arctan √2` (§1.1a). **[P]** |
| **HEALPix** | **Górski et al. 2005 §5.1 (centres, eqs 4/5/8/9), §5.3 (boundaries, eqs 19/20/22).** Corners **[D]** by inverting the boundary equations — see below. Area = the paper's exact `Ω = π/(3N_side²)` **[P]** |
| rhombic-dodec · gnomonic | convex hull of cube ∪ octahedron; bilinear in the (planar) rhombus, radial projection. Canonical — no invention. |
| rhombic-dodec · tan-warp | **OURS.** A tan-warp of the parallelogram coordinates, by analogy with the equiangular cube. **Not from any source** — discount accordingly. |
| icosa-tri | class-I geodesic subdivision, radial projection. |
| icosa-hex (raw dual) | the spherical **Voronoi dual**: circumcentres of the Delaunay triangles. Exactly a Voronoi diagram, hence exactly orthogonal **[D]**. |
| icosa-hex (SCVT) | + 200 Lloyd iterations **with Delaunay edge flips** (§1.1c). MPAS-style. |

**The HEALPix corner derivation [D], because it is the one place I had to go beyond the paper.**
Górski gives pixel *boundaries*, not corners. Inverting them: in the polar cap, with
`t = N_side·√(3(1−z))`, eq (19) reads `A ≡ t·(2φ_t/π) = k` and eq (20) reads
`B ≡ t·(1 − 2φ_t/π) = k`, with `A + B = t` — so **(A,B) is a square lattice**, centres at
half-integers, corners at `(A±½, B±½)`. In the belt, with `U = z − 8φ/3π` and `V = z + 8φ/3π`,
eq (22)'s two families are `U = const` and `V = const` — **another square lattice**. Every pixel
is therefore a *diamond*. The one place it bites is the cap/belt junction, where a belt pixel's
poleward corner lies in the *cap* zone and must be taken from the (A,B) lattice rather than
extrapolated from the belt's linear-in-`i` formula (which would put it at a latitude no cap
vertex occupies). **The derivation is not trusted, it is checked**: Euler = 2, every edge borders
exactly 2 pixels, every pixel's area matches the paper's `Ω`, ΣΩ = 4π, and each base pixel
receives exactly N_side² pixels. If the lattice were wrong, none of those would hold. **[M]**

---

## 3. Snyder's Table 1 — the polyhedron scorecard, from the paper **[P]**

The table that frames the whole quadtree-vs-quality tension. Reproduced from the primary:

| polyhedron | g | G | θ | **equal-area ω** | a | b | gnomonic ω |
|---|---|---|---|---|---|---|---|
| Trunc. icosahedron (hex faces) | 23.80018260° | 62.15468023° | 60° | **3.75°** | 1.033 | 0.968 | 5.09° |
| Trunc. icosahedron (pent faces) | 20.07675127° | 55.69063953° | 54° | **2.65°** | 1.030 | 0.983 | 3.59° |
| Tetrahedron | 70.52877937° | 60° | 30° | 52.07° | 1.601 | 0.624 | 60.00° |
| **Cube** | **54.73561032°** | **60°** | **45°** | **25.17°** | **1.248** | **0.801** | **31.08°** |
| Octahedron | 54.73561032° | 45° | 30° | 34.45° | 1.357 | 0.737 | 31.08° |
| Dodecahedron | 37.37736814° | 60° | 54° | 10.24° | 1.094 | 0.914 | 13.14° |
| Icosahedron | 37.37736814° | 36° | 30° | 17.27° | 1.163 | 0.860 | 13.14° |

> **The cube is a genuinely poor polyhedron.** ω = 25.17° against the truncated icosahedron's
> 3.75° — nearly **7× worse**. That is real and it is not going away. **But hexagons and
> pentagons do not subdivide into four**, and 1→4 subdivision is what our entire `CellId` / LOD /
> memo-key / Hilbert-locality architecture rests on. **That tension is the actual subject of this
> report**, and §5.2 is where it gets resolved: the geometric penalty turns out to be payable
> *in the scheme*.

Our implementation reproduces the cube row exactly (§1). Note `a·b = 1.248 × 0.801 = 0.9997 ≈ 1`,
as equal-area requires — a free check on the table itself.

---

## 4. Metric by metric — **find yours here**

> **How to use this section.** Each metric says *what it is*, *who should care*, and *what the
> numbers say*. If you are writing a kernel, read the rows your kernel actually leans on and
> ignore the rest. Our conclusion is in §9; you are not obliged to share it.

### 4.1 Cell area — and a tax nobody states

**Who cares:** anything conserving a stock (mass, water, sediment); drainage area in stream
power `E = K·A^m·S^n`; any per-cell volume.

| grid | area spread (TRUE) | area spread (GEODESIC cell) | gap |
|---|---|---|---|
| cube · equiangular (OURS) | 1.3969× | 1.3969× | — |
| cube · Snyder equal-area | **1.0000×** | 1.0584× | **4.96 %** |
| cube · gnomonic | 5.0307× | 5.0307× | — |
| rhombic-dodec · gnomonic | 2.7350× | 2.7350× | — |
| rhombic-dodec · tan-warp (ours) | 1.5185× | 1.5185× | — |
| HEALPix | **1.0000×** | 1.1167× | **9.96 %** |
| icosa-tri | 1.9308× | 1.9308× | — |
| icosa-hex (raw dual) | 2.0398× | 2.0398× | — |
| icosa-hex (SCVT) | 1.7361× | 1.7361× | — |

**[M]** The equiangular 1.41× and gnomonic 5.14× figures from `sphere.rs`'s own docstring are
confirmed (we measure 1.3969 / 5.0307 at this resolution; both still creeping up with N).

> **⚠ THE TWO COLUMNS DIFFER, AND THIS IS A REAL FINDING.** "Equal-area" is a property of the
> continuous **map**. It is **not inherited by the discrete cell you build from it.** Snyder's
> and HEALPix's cell boundaries are *not great circles* (Górski §5.3 says so explicitly **[P]**),
> so the geodesic polygon through a cell's corners — which is the cell a finite-volume code
> actually builds (Putman & Lin: *"the cell edges for all grids are prescribed to be great circle
> arcs"* **[P]**) — is **not** the equal-area cell. The discrepancy is **5.0 % on Snyder and
> 9.9 % on HEALPix** **[M]**.
>
> **You can have exactly-equal AREAS or great-circle EDGES. Not both.** Take the exact area and
> geodesic edges anyway (the pragmatic choice, and what we do) and the scheme is still exactly
> conservative — but it is no longer geometrically *consistent*, and it shows up in §5.2.
>
> Every other grid here is free of this: their boundaries genuinely are great circles, so the two
> columns coincide by construction. **This is a cost that only the equal-area grids pay, and it
> is not in the literature we read.**

### 4.2 Edge length

**Who cares:** shallow-water / any flux-form kernel — the edge is the flux face width.
`water.rs`'s `fl/fr/ft/fb` are exactly this.

| grid | edge spread |
|---|---|
| cube · equiangular (OURS) | 1.4141× |
| cube · Snyder equal-area | 1.5299× |
| rhombic-dodec · tan-warp | **1.3546×** |
| HEALPix | 1.8178× |
| icosa-tri | 1.4561× |
| icosa-hex (raw) | 2.1344× |
| icosa-hex (SCVT) | 1.7857× |

**[M]** **Nothing hoists this to a constant. On any grid here.** Note that **the equal-area grids
are among the WORST on edge length** (Snyder 1.53×, HEALPix 1.82× vs our 1.41×) — they buy area
by *letting shape vary*, and edge length is shape. Górski says so himself **[P]**: *"all pixels
have the same surface area but slightly different shape."*

### 4.3 Centre-to-mid-edge arm, and **skew** — Joseph's carve, and it is the right one

**Who cares:** anyone writing a finite-volume gradient. **The mid-edge arm is the correct FV
gradient arm — not the centre-to-centre distance.**

| grid | arm spread | **skew** (max) | arm-deficit (max) |
|---|---|---|---|
| cube · equiangular (OURS) | 1.3971× | 24.69 % | 14.77 % |
| cube · Snyder equal-area | 1.5356× | **7.53 %** | **1.56 %** |
| rhombic-dodec · tan-warp | 1.3450× | 24.84 % | 15.04 % |
| HEALPix | 2.0350× | **49.62 %** | 25.39 % |
| icosa-tri | 1.5587× | 6.36 % | 1.99 % |
| icosa-hex (raw) | 1.4630× | **1.72 %** | **0.01 %** |
| icosa-hex (SCVT) | 1.3544× | 8.85 % | 0.24 % |

- **skew** = distance from the mid-edge to where the centre-line actually crosses the edge,
  ÷ edge length.
- **arm-deficit** = `|armᵢ + armⱼ − dist| / dist`. Exactly 0 iff the centre-line passes *through*
  the mid-edge.

> **⚠ ORTHOGONALITY AND SKEW ARE INDEPENDENT, and conflating them is the standard mistake — I
> made it myself and the numbers caught me.** The hexagonal Voronoi meshes are **exactly
> orthogonal** (0.00°, §4.4) **and still skew** (1.7 %). Orthogonality asks *does the centre-line
> pierce the edge at 90°*; skew asks *does it pierce it at the MID-EDGE*. **That is precisely why
> the mid-edge arm, and not the centre-to-centre line, is the correct FV gradient arm** — Joseph's
> carve, confirmed. **[M/D]**
>
> Note **HEALPix is the worst grid in this report on skew (49.6 %)** — its diamonds are strongly
> sheared. This is a direct cost of iso-latitude, a requirement that exists to make spherical-
> harmonic transforms cheap (Górski §6 **[P]**) and that **we will never collect on.**

### 4.4 Angle / non-orthogonality

**Who cares:** every gradient, every Laplacian, every flux. **This is the metric that turns out
to matter most (§5.2).**

| grid | non-orth (max) | interior angle min | max |
|---|---|---|---|
| cube · equiangular (OURS) | 28.79° | 60.82° | **120.00°** |
| cube · Snyder equal-area | 15.04° | 68.96° | 128.24° |
| cube · gnomonic | 29.22° | 60.53° | 120.00° |
| rhombic-dodec · gnomonic | 29.45° | 60.38° | 120.00° |
| HEALPix | **36.73°** | 53.11° | 146.01° |
| icosa-tri | 10.05° | 54.01° | 72.00° |
| **icosa-hex (raw dual)** | **0.00°** | 108.00° | 126.00° |
| **icosa-hex (SCVT)** | **0.00°** | 107.99° | 126.61° |

**[M]** The **120.00°** at the cube corners independently reproduces Putman & Lin §3.2 **[P]**:
*"the corner singularities remain non-orthogonal with angular deviation from 90 fixed at 30[°]."*

> **The hexagonal duals are orthogonal EXACTLY, and it is not luck — it is a theorem.** **[D]**
> A Voronoi edge lies on the perpendicular bisector of its two generators, whose plane has normal
> `(cᵢ − cⱼ)`; the centre-line's great circle has pole `cᵢ × cⱼ`, and
> `(cᵢ − cⱼ) · (cᵢ × cⱼ) = 0` identically. So the two great circles meet at exactly 90°, at every
> edge, on any Voronoi mesh, forever. Measured: **0.00°.** This is the single largest structural
> advantage any grid in this report has, and §5.2 is where it cashes in.

### 4.5 Valence / number of sides

| grid | edge-valence census | Moore census |
|---|---|---|
| all three cube grids | **4 : every cell** | 7 : **24 cells** · 8 : the rest |
| rhombic-dodec (both) | **4 : every cell** | 7 : **24 cells** · 8 : the rest |
| HEALPix | **4 : every cell** | 7 : **24 cells** · 8 : the rest |
| icosa-tri | **3 : every cell** | 11 : 60 cells · 12 : the rest |
| icosa-hex (both) | 6 : 24 990 · **5 : exactly 12** | same |

**[M]**

> **The sibling research's claim is CONFIRMED, and by a different method.** `seam-adjacency-
> findings.md` §2 predicted 24 defective pixels for HEALPix at every N_side (verified there with
> `astropy-healpix`). We measure **exactly 24** — from an independent implementation built from
> the paper, with adjacency derived combinatorially. **[M]** And Euler's argument is the reason:
> `Σ(4 − valence) = 4V − 2E = 8` on **any** quad mesh on S² **[D]**. The cube, the rhombic
> dodecahedron and HEALPix all spend that charge identically — as **8 valence-3 vertices ⇒ 24
> cells with 7 Moore neighbours.**
>
> **Every cell on every quad grid here has exactly 4 EDGES — including the 24 corner cells.**
> The defect exists **only** in the diagonal (Moore) stencil. Hold that thought until §6; it is
> the whole resolution of the corner problem.
>
> Hexagons redistribute rather than remove: `Σ(6 − v) = 12` ⇒ **exactly 12 pentagons** — more
> defect cells, each a *milder* perturbation. Measured: 12. **[M]**

### 4.6 The **stencil**, not the cell — the configuration-class census

**Who cares:** anyone deciding whether the per-cell geometry can be a small lookup table with a
cheap branch, or must be computed. **This is the innermost-loop cost question.**

Joseph expected **~10 classes at the coarsest level**. On our own grid, by level **[M]**:

| level | cells | **topological** classes | **geometric** classes @1 % | N²/8 per face |
|---|---|---|---|---|
| L1 | 24 | 1 | 1 | 1 |
| L2 | 96 | 2 | 3 | 2 |
| **L3** | 384 | 2 | **10** | 8 |
| L4 | 1 536 | 2 | 36 | 32 |
| L5 | 6 144 | 2 | 134 | 128 |
| L6 | 24 576 | 2 | 509 | 512 |

> **Joseph's ~10 is EXACT at L3 — and it is a coincidence of that level, not a law.** The
> geometric class count grows as **N²/8** (the face's D₄ symmetry folds an N×N face into an N²/8
> fundamental domain), reaching 509 by L6 and ~10⁵ by L10. **Any design that assumes a fixed
> small table of stencil classes will break.** **[M/D]**
>
> **⚠ ONE NUMBER IN THE COMMISSIONING DECISION NEEDS CORRECTING.**
> `DECISIONS[geometric-contract-metric-set]` records the plan as *"Split DISCRETE configuration
> class (**finite, ~10**, a cheap branch — dispatches the code path) from CONTINUOUS metrics."*
> The **split is exactly right and is the answer** — but the `~10` is attached to the wrong half.
> Measured: the **discrete/topological** class count is **2**, not 10 (better than hoped), and it
> is *genuinely* fixed at every level. The **~10** is the **geometric/congruence** count, and that
> is the half that **does not stay finite** — it grows as N²/8. Reading `~10` as "the discrete
> branch has about ten arms" would build a fixed dispatch table that is correct at L3 and wrong at
> L4. **[M]**

> **With that corrected, the split Joseph named is exactly right, and it is the answer:**
>
> | | count | grows? | what to do |
> |---|---|---|---|
> | **Discrete / topological** class | **2**, on every grid here (regular + defective) | **NO** — 24 cells, world-wide, at *every* level | **a cheap branch.** This is the only branch a kernel needs. |
> | **Continuous** metrics (area, edge, arm, angle) | ~N²/8 and rising | **YES**, quadratically | **compute, don't tabulate** — they are *analytic in position*, so evaluate them (or memoize a few f32 planes per `(face, level)`; they are identical for every tile on a face). |
>
> That is Putman & Lin §3, verbatim **[P]**: *"The cell areas (ΔA), grid lengths … and a
> non-orthogonal metric factor (sin α) … **are derived quantities**."*

At matched cell counts the icosahedral grids have ~2.5× fewer geometric classes (196–215 vs
~510) — their symmetry group is bigger (order 120 vs the cube's 48). **[M]** This is a real but
minor advantage; it does not change the growth law.

### 4.7 Curvature — **split in two, and keep them apart**

Joseph's carve, and it holds up. A grid can be fine for one and wrong for the other.

#### (a) Arc / metric geometry in the embedding — *"geometry in einstein spacetime"*

Governs distances, areas and transport at large spans. Discrete Gaussian curvature = the angle
defect `2π − Σθ` at each vertex (chordal angles — the *polyhedron's*; the spherical angles around
a vertex sum to 2π by construction and say nothing).

| grid | Σdefect / 4π | max defect | mean | **at the topological DEFECTS** |
|---|---|---|---|---|
| cube · equiangular (OURS) | 1.0342 | 0.0392° | 0.0303° | **0.0392°** |
| cube · Snyder equal-area | 1.0161 | 0.0328° | 0.0298° | 0.0249° |
| rhombic-dodec | 1.1195 | 0.0450° | 0.0332° | 0.0331° |
| HEALPix | 1.0935 | 0.0497° | 0.0324° | 0.0497° |
| **icosa-tri** | **1.000000000** | 0.0708° | 0.0588° | 0.0351° |
| icosa-hex (raw) | 1.0000012 | 0.0174° | 0.0144° | 0.0144° |

**[M]**

> **⚠ The first column is NOT the free Gauss–Bonnet check I first took it for, and the reason is
> itself a finding.** Gauss–Bonnet forces `Σ defect = 4π` **only on a polyhedron whose faces are
> PLANAR**. Three points are always coplanar — so it holds *exactly* on the triangle mesh, and it
> does: **1.000000000**. Four points on a sphere are **not** coplanar, so on every quad grid the
> sum overshoots — **and the overshoot IS the non-planarity of the cells.** So the column measures
> *how far your control volumes are from being flat polygons at all*: cube **3.4 %**, HEALPix
> **9.4 %**, rhombic-dodec **12.0 %**, triangles **0.0 %**. **[M/D]**

> **⚠⚠ AND THE LAST COLUMN KILLS A MISCONCEPTION WORTH KILLING.** The curvature at the 8 cube
> corners is **not anomalous — it is ordinary** (0.0392° vs a 0.0303° mean; the same order, not a
> spike). Three cells meet there at ~120° each, summing to ~360°, so the angle *defect* is about
> what it is everywhere else.
>
> **The cube corner is NOT a curvature singularity. It is a CONNECTIVITY singularity.** The
> *surface* is perfectly smooth there; it is the *coordinate system* that is defective. That is
> why no amount of geometric cleverness removes it, and why the fix (§6) is a scheme that does not
> depend on valence. **[M/D]**

#### (b) Flat-via-gravity — *"geometry in euclidean space"*

The surface is a gravitational equipotential, so water **locally experiences a flat plane** —
which is exactly why the shallow-water kernel's flat assumption is legitimate. The number that
decides it is the **sagitta**: how far a cell's own corners rise off the plane through its centre.
It is a *length*, so it shrinks as `h²/2R`:

| level | cell | sagitta | as a fraction of the cell |
|---|---|---|---|
| L2 | 2502 km | 599 km | 2.4e-1 |
| L5 | 313 km | 9.6 km | 3.1e-2 |
| L8 | 39 km | 150 m | 3.9e-3 |
| L11 | 4.9 km | 2.3 m | 4.8e-4 |
| L14 | 611 m | 3.7 cm | 6.0e-5 |
| L17 | 76 m | 0.57 mm | 7.5e-6 |
| L20 | 9.5 m | 8.9 µm | 9.4e-7 |
| **L25** | **0.30 m** | **8.7 nm** | **2.9e-8** |

**[M/D]** (closed form `R(1 − cos(half-diagonal))`; agrees with the full-mesh measurement to 3
significant figures)

> **⇒ THIS METRIC DOES NOT DISCRIMINATE BETWEEN GRIDS AT ALL — and that is a real finding, not a
> null one.** The sagitta is set by the cell size and the planet's radius; **the projection does
> not enter it.** At vivarium's playable rung (L25, 0.5 m cells) it is ~1e-8 m: flat-via-gravity
> is exact to any tolerance that will ever matter, **on every grid here**.
>
> So the shallow-water kernel's flat assumption is **legitimate on all of them**, and the sphere
> enters that kernel **only** through the metric terms (area, edge, arm) — **never** as a "slope of
> the world" the water has to run down. *Geometry-in-euclidean-space: settled, and free.
> Geometry-in-einstein-spacetime: not free, and it is what every other section of this report is
> about.* **A clean "this doesn't apply" — which is a real finding.**

### 4.8 Strata volumes — the vertical

A column's volume is **not** area × height: the shell's volume element scales as `r²`.

| shell | top/bottom cell AREA ratio | true volume ÷ (area × height) |
|---|---|---|
| the ~20 km livable shell (−11 km → +9 km) | **1.00630×** (+0.630 %) | 0.99969 (−0.031 %) |
| 0 → 20 km | 1.00629× (+0.629 %) | **1.00314** (+0.314 %) |
| 0 → 1 km | 1.00031× (+0.031 %) | 1.00016 (+0.016 %) |

**[M/D]** Joseph's ~0.6 % is the **top/bottom area ratio**; the *mean* volume correction over the
shell is half that (+0.31 %).

> **This is a MASS term, and rock mass is what erosion must conserve.** It is entirely **radial**,
> so it is **identical on every grid in this report** — another clean "does not discriminate."
>
> **But it does not vanish, and it composes:** it *multiplies* whatever area spread the grid
> already has. On an equal-area grid a column's volume is still **one constant per (level,
> altitude)** and hoists. On ours it inherits the 1.41×. **[I]**

---

## 5. Per (grid × algorithm) — what survives when the algorithm is applied

### 5.1 CONSERVATION — mass leak under linear diffusion

Linear diffusion is used because **its invariant is exactly known**: nature conserves total mass
exactly, so *any* drift is the scheme lying. No tuning, no opinion.

| grid | naive @ centre | naive @ defect | **finite volume** |
|---|---|---|---|
| cube · equiangular (OURS) | −1.29e-2 | +4.88e-3 | **−2.9e-15** |
| cube · Snyder equal-area | −3.9e-15 | −5.0e-15 | **−3.7e-15** |
| cube · gnomonic | −5.67e-2 | +8.42e-2 | **−4.1e-15** |
| rhombic-dodec · gnomonic | +1.06e-1 | +4.78e-2 | **−4.5e-15** |
| HEALPix | +2.0e-15 | −2.8e-15 | **−9.1e-15** |
| icosa-tri | +4.52e-2 | +4.52e-2 | **−3.7e-15** |
| icosa-hex (raw) | +1.13e-1 | +1.13e-1 | **−1.4e-14** |

**[M]** And the naive leak is **worst at COARSE levels** — i.e. exactly where the developmental
ladder starts:

| level | cells | naive | finite volume |
|---|---|---|---|
| L2 | 96 | **+6.81e-2** | +7.8e-16 |
| L3 | 384 | **+8.73e-2** | −5.8e-16 |
| L4 | 1 536 | +5.27e-2 | +7.3e-16 |
| L5 | 6 144 | +1.46e-2 | −2.6e-15 |
| L6 | 24 576 | +3.01e-3 | +1.2e-16 |
| L7 | 98 304 | +5.25e-4 | −8.4e-15 |

**[M]** *(This supersedes the earlier "9.7e-2 at L4 / 8.5e-3 at L6 / 5.4e-4 at L8" figures, which
were measured on a single face with open boundaries. Same story, whole-sphere: the leak is ~10 %
at the coarsest levels and falls roughly as h².)*

> **CONSERVATION IS A SCHEME PROPERTY.** Finite volume fixes it **exactly** (~1e-15) on **every**
> grid here, including the worst one, **because the flux across an edge is literally the same
> number added to one cell and subtracted from the other.** No projection is needed and none
> helps. Corroborated by a primary — Putman & Lin §4 **[P]**: *"For all schemes tested on all
> grids, mass is conserved to machine precision."*
>
> The two **equal-area** grids conserve even under the *naive* kernel (1e-15) — that is the one
> thing equal-area genuinely buys. §5.2 shows it is not the thing that matters.
>
> **⚠ DO NOT STOP HERE.** The obvious next sentence — *"so conservation is a scheme property and
> isotropy is a grid property"* — is the conclusion the earlier passes drew, and **§5.2 refutes
> it.** Exact conservation does **not** imply the scheme is solving the right equation.

### 5.2 CONSISTENCY / ACCURACY — **the table the whole investigation was for**

A spherical harmonic of degree ℓ is an **exact eigenfunction** of the Laplace–Beltrami operator:
`Δ Yℓ = −ℓ(ℓ+1)/R² · Yℓ`. So the truncation error of a discrete Laplacian is **exactly known** —
no reference solution, no tuning, no opinion. (The test harmonic is deliberately *not* aligned to
any grid's axes.) Relative L2 error, ℓ = 2, at ~24 500 cells:

| grid | naive | **FV two-point** | corrected / narrow grad | **corrected / WIDE grad** | gain |
|---|---|---|---|---|---|
| **cube · equiangular (OURS)** | 9.58e-1 | 9.20e-1 | 4.91e-3 | **3.65e-4** | **2523×** |
| cube · Snyder equal-area | 2.38e0 | 6.65e-1 | 6.53e-3 | 8.66e-4 | 768× |
| cube · gnomonic | 6.91e-1 | 1.14e0 | 2.43e-3 | 6.81e-4 | 1678× |
| rhombic-dodec · gnomonic | 9.50e-1 | 1.19e0 | 3.99e-3 | **5.14e-4** | 2313× |
| rhombic-dodec · tan-warp (ours) | 1.18e0 | 1.09e0 | 6.07e-3 | 5.71e-4 | 1903× |
| HEALPix | 1.84e0 | 1.13e0 | 1.54e-2 | 8.44e-4 | 1333× |
| icosa-tri | 5.49e-1 | 2.11e0 | 5.70e-3 | 3.06e-3 | 690× |
| **icosa-hex (raw dual)** | 6.30e-1 | **4.04e-4** | 3.21e-4 | 2.99e-4 | 1× |
| **icosa-hex (SCVT)** | 2.70e-1 | **6.01e-4** | 2.19e-4 | **2.19e-4** | 3× |

Convergence order (`log₂(err_coarse / err_fine)`; **2.0 = second order; negative = the error
GROWS as you refine**):

| grid | two-point order | **corrected/WIDE order** |
|---|---|---|
| cube · equiangular (OURS) | **−0.50** | **+1.63** |
| cube · Snyder equal-area | **−0.45** | +1.39 |
| rhombic-dodec · gnomonic | **−0.47** | +1.63 |
| HEALPix | **−0.52** | +1.38 |
| icosa-tri | **−1.10** | +1.02 |
| icosa-hex (raw dual) | **+1.74** | +1.93 |
| icosa-hex (SCVT) | **+1.32** | **+1.95** |

**[M]** **Read it left to right; this is the core of the report.**

**1. A two-point flux is INCONSISTENT on a non-orthogonal mesh.** `(u_j − u_i)·L/d` — which is
what "finite volume with the true geometry" naively means, and which §5.1 shows conserves
*exactly* — has **O(1) relative error that grows under refinement.** **[M]** The mechanism is
structural **[D]**: the two-point difference measures `∇u` along the **centre-line**, but the flux
needs `∇u` along the edge **normal**. Where they differ by θ, the discarded tangential term scales
as `sin θ · R/h` — and **R/h doubles every time you refine.**

> ⇒ **Conservation and consistency are different properties, and finite volume buys only the
> first.** This **supersedes** the decomposition in `seam-adjacency-findings.md` §4.2 and
> `DECISIONS[seam-grid-leads-snyder-healpix]`.

**2. The hexagonal duals are the control that proves it.** They are **exactly orthogonal** (§4.4,
0.00°, forced by duality), and there the **same two-point scheme is already consistent** — 4e-4,
converging at 1.7–1.9, with no correction at all. Same scheme, different grid. **[M]**

**3. So correct the scheme.** Project both cell centres onto the edge **normal** through the
**mid-edge** and extrapolate with a reconstructed gradient. That kills the non-orthogonality error
*and* the skew error, and **stays exactly conservative** (the correction is antisymmetric under
`i ↔ j`). This is Putman & Lin's `sin α` metric factor **[P]** in mesh form — and it is why FV3
works on this grid.

**4. And then the part nobody was looking for — the last two columns.** With the gradient fitted
over the **4 edge-neighbours** ("narrow"), the scheme converges, but only at ~0.5 order: a linear
fit over 4 points on a distorted quad is only **first-order** accurate, and a first-order gradient
is not good enough to feed the correction. Fit a **quadratic over the 8 Moore neighbours**
("wide") and the error drops another **~30×** and the order jumps to **~1.6**. **[M]**

> **A hexagon's 6 near-symmetric neighbours give a second-order gradient for free. That — not
> isotropy, not equal area — is the largest single reason the hex mesh wins. And it is a STENCIL
> property, not a GRID property.**

At a valence-3 corner there are **7** Moore neighbours and the quadratic fit needs **5**. It still
works. This is Addendum A1's *"LSQ gradient reconstruction recovers second order at any valence"* —
**confirmed, with a caveat A1 does not state: only with the WIDE stencil.** **[M]**

#### Two hypotheses I tested and had to abandon — recorded because a refuted hypothesis is a result

- **"The cell centre must be the centroid."** Plausible: an SCVT generator *is* the centroid by
  definition, so maybe that is why it converges. Tested — moved every quad cell's centre to its
  true spherical centroid. **Order 0.57 → 0.56. No effect.** **[M]** Refuted.
- **"The 24 defects are dragging the L2 norm down."** Tested — split the error by distance from
  the defects. The **far field** converged at only 0.27 too. **[M]** Refuted. (Both were wrong;
  the gradient stencil was the answer.)

#### ⇒ THE HEADLINE

On our own equiangular cube-sphere the corrected scheme reaches **3.65e-4**, against the best
hexagonal mesh's **2.19e-4**. Not parity — the hex mesh is still **1.7× better** — but the **same
order of magnitude, on the metric hexagonal meshes exist to win.** **[M]**

Set that 1.7× against what switching costs: the `CellId` quadtree, the LOD ladder, the memo keys,
the store's Hilbert locality, and the fated-noise KRNG — all of which key on 1→4 subdivision that a
hexagon **does not have** (§5.6).

**We did not change the grid. We changed the kernel.** The 2500× is entirely scheme-side and costs
nothing architecturally.

### 5.3 ISOTROPY — and does the error converge away?

A symmetric blob diffused 300 steps; ratio of principal second moments (1.0000 = stayed circular).

| grid | anisotropy at a defect, two-point | **corrected** |
|---|---|---|
| cube · equiangular (OURS) | 1.0025 | **1.0000** |
| rhombic-dodec · tan-warp | 1.0056 | 1.0000 |
| HEALPix | **1.1492** | 1.0021 |
| icosa-hex (both) | 1.0000 | 1.0000 |

**[M]**

> **This REVISES the earlier finding.** `seam-adjacency-findings.md` §5 records *"grid_lab
> measures the residual corner anisotropy at 1.16–1.25× with exact geometry, not shrinking with
> refinement — finite-volume buys exact conservation and does not buy isotropy."*
>
> **That was measured with a two-point flux on a single open face.** On the whole sphere with
> correct cross-face adjacency, the two-point anisotropy at a cube corner is **1.0025**, and with
> the corrected scheme it is **1.0000**. **The corner anisotropy was substantially an artifact of
> the scheme and the open boundary, not an irreducible property of the grid.** **[M]**
>
> HEALPix is the one grid with real residual anisotropy (1.149 under two-point) — consistent with
> it being the worst grid here on both skew (49.6 %) and non-orthogonality (36.7°).

### 5.4 Which geometric terms HOIST to constants — the perf lever

**Who cares:** anyone deciding whether a "sunny-day" fast path is even *legal*.

| grid | area | edge | arm | dist | angle | valence |
|---|---|---|---|---|---|---|
| cube · equiangular (OURS) | no | no | no | no | no | **CONST (4)** |
| **cube · Snyder equal-area** | **CONST** | no | no | no | no | **CONST (4)** |
| **HEALPix** | **CONST** | no | no | no | no | **CONST (4)** |
| rhombic-dodec (both) | no | no | no | no | no | **CONST (4)** |
| icosa-tri | no | no | no | no | no | **CONST (3)** |
| icosa-hex (both) | no | no | no | no | no | no (6 + twelve 5s) |

**[M]**

> **Equal-area buys exactly ONE of the four metrics, and it is the one we already get for free.**
> `cell_area(level)` becomes a constant on Snyder and HEALPix — and *nothing else does, on any
> grid here*. Edge length, arm, and distance stay variable everywhere. And per §4.1, even that one
> constant is only exactly true if you accept non-geodesic cell boundaries.
>
> Meanwhile **the metric that actually matters (§5.2) is non-orthogonality, and equal-area does
> not fix it** — Snyder halves it (15.0° vs our 28.8°) and HEALPix makes it **worse** (36.7°).
>
> The one thing equal-area would genuinely buy us, we can have for the cost of **one f32 plane per
> `(face, level)`** — the metrics are analytic in `(u,v)` and *identical for every tile on a face*
> — and unlike equal-area it comes with the **other three metrics too**. **[I]**, following
> `seam-adjacency-findings.md` §4.2.

### 5.5 SEQUENCING / PARALLELISM — Joseph's criterion, and the one with teeth

Diffusion is embarrassingly parallel and therefore **the least informative case** — the old
harness only had that one. **Priority-Flood and drainage accumulation are globally ORDERED**
(elevation-sorted, strictly downstream; `erosion.rs::accumulate_drainage` sorts every cell by `h`
and sweeps the sorted order in reverse). At a seam, a cell's **downstream** neighbour can live in
another partition — a dependency edge that **serialises** the sweep.

Same band-limited terrain on every grid; each partitioned by its **own natural coarse block**.

| grid | parts | boundary cells | **crossing** | partition graph cyclic? | **sync rounds** | longest path |
|---|---|---|---|---|---|---|
| **cube · equiangular (OURS)** | **6** | **6.15 %** | **1.73 %** | YES | **3** | 56 |
| cube · Snyder equal-area | 6 | 6.15 % | **1.70 %** | YES | **2** | 57 |
| cube · gnomonic | 6 | 6.15 % | 1.73 % | YES | 2 | 64 |
| rhombic-dodec · gnomonic | 12 | 8.69 % | 2.11 % | YES | 3 | 49 |
| HEALPix | 12 | 8.69 % | 2.09 % | YES | 3 | 51 |
| icosa-tri | 20 | 8.33 % | 2.53 % | YES | 4 | 53 |
| icosa-hex (raw) | 20 | 11.70 % | **3.52 %** | YES | **5** | 56 |
| icosa-hex (SCVT) | 20 | 11.68 % | 3.42 % | YES | 4 | 52 |

**[M]**

- **crossing** = cells whose steepest-descent **receiver** is in another partition. These *are*
  the dependency edges.
- **cyclic?** = does the **partition-level** dependency graph have cycles? **It does, on every
  grid.** So **no ordering of whole partitions exists** and a partitioned Priority-Flood **must**
  synchronise at cell granularity across the seam. **This is the ordering tooth, and no grid pulls
  it.** **[M]**
- **sync rounds** = the max number of partition transitions along any flow path — the number of
  synchronisation rounds a partitioned accumulation must pay.

> **THE CUBE-SPHERE WINS THIS METRIC, and it wins it for a boring, robust reason: 6 partitions
> beat 12 or 20.** Fewer, larger blocks ⇒ less boundary ⇒ fewer crossings (1.73 % vs the hex
> mesh's 3.52 %, a **2× advantage**) and fewer sync rounds (3 vs 5). **[M]** The hexagonal mesh is
> the *worst* grid here on the criterion Joseph flagged as possibly dominant — it has no coarse
> quad block structure at all, so its natural partition is the 20 icosahedral faces.
>
> **Honest caveat, and it matters:** this is partly an artifact of *how many* partitions each
> grid's natural hierarchy gives, not of the grid's intrinsic quality. At a fixed partition count
> the grids would be closer. But *"how many natural partitions does your hierarchy give you"* **is
> a real property of the grid**, not a nuisance parameter — a grid without a quadtree does not get
> to choose. **[I]**
>
> **And the finding that survives all of that: EVERY grid's partition graph has cycles.** The
> ordering problem is **not solvable by choosing a projection.** It has to be solved by the
> algorithm (a parallel Priority-Flood with a boundary-merge phase). **[M]**

### 5.6 QUADTREE-ABILITY / LOD — what our entire architecture rests on

| grid | 1 → 4 subdivision? |
|---|---|
| cube (all three projections) | **YES** — native |
| **rhombic dodecahedron** | **YES** — a rhombus subdivides into four rhombi |
| HEALPix | **YES** — the NESTED scheme (Górski §5.2 **[P]**: *"essential for the possible database applications"*) |
| icosa-tri (geodesic) | **YES** — a triangle subdivides into four triangles |
| **icosa-hex (both)** | **NO** — a hexagon does not subdivide into four hexagons |

> **The hexagonal mesh — the only grid that beats us on accuracy — is the only grid that cannot
> quadtree.** That is the tension in one line, and it is not a coincidence: the near-isotropy that
> makes it good is the same property that makes it not a quad mesh. **[D]**
>
> Note the **icosahedral triangle** grid *does* quadtree and *is* more orthogonal than the cube
> (10.05° vs 28.79°) — but it is the **worst** grid in the report on corrected accuracy (3.06e-3,
> an order of magnitude behind everything else) **[M]**, because a 3-neighbour edge stencil is a
> poor gradient stencil. **Triangles subdivide beautifully and compute badly.**

---

## 6. The corner — is the valence-3 pathology real?

**The claim under test** (Joseph): *"your up-neighbour's left edge is welded to your
left-neighbour's top edge; the diagonal cell doesn't exist"* — is this **an artifact of MFD's
8-neighbour fan, not of the sphere?**

### The topology, measured on the real whole-sphere cube grid **[M]**

```
cube-sphere, 6·32² = 6144 cells:
  EDGE  adjacency : 4 edges × 6144 cells     ← EVERY cell. No exceptions.
  MOORE adjacency : 7 Moore × 24 cells,  8 Moore × 6120 cells
```

> **Every cell has exactly 4 edges — INCLUDING the 24 corner cells.** The 4-edge flux form has
> **no special case at a cube corner.** The defect exists **only** in the diagonal (Moore) stencil.

**`water.rs` is already in flux form — verified primary, not inherited.** Read directly: `fl/fr/ft/fb`
are *"Outgoing flux per axial pipe (m³/s), **kept between steps**: momentum"* (`water.rs:211`), with a
4-neighbour stencil (`i±1`, `i±nx`). So **water needs no corner special case at all** — it would
simply have 4 edges there, like everywhere else, and conserve exactly. **[M, primary]**

> ⚠ **One caveat the earlier passes did not flag, found by reading it:** `water.rs` also runs a **θ
> flux-smoothing pass** (de Almeida, Bates, Freer & Souvignet 2012) that reads *neighbouring
> fluxes* — `fl[i±1]`, `ft[i±nx]` (`water.rs:362–370`). That is a **second stencil**, on the flux
> field rather than the state field, and **it will need its own halo and its own seam treatment**.
> It is not covered by "water is already flux-form." **[M, primary]**

**Only `erosion.rs`'s MFD wants the diagonal that isn't there** — and its diagonals are themselves a
hack against grid-aligned bias (its own comment says so).

### The fan at a corner, measured **[M]**

```
regular cell : 8 Moore nbrs, gaps  60° 57° 32° 31° 41° 48° 46° 45°
CORNER  cell : 7 Moore nbrs, gaps  60° 48° 43° 59° 59° 43° 48°
```

The missing diagonal is not merely *absent* — **the surviving fan is not a fan.** An 8-neighbour
MFD there weights directions that are not 45° apart, at distances that are not `cell_m·√2`. (Note
the *regular* cell's fan is already badly non-uniform: 31°–60°. The 8-fan was never a good
approximation, corner or no corner.)

### The routing test — three routers, same grid, same perfect cone

A cone is radially symmetric by construction **and has an exact answer**: everything drains
radially, so at angular distance θ the specific catchment area is

`a(θ) = area(cap θ) / circumference(θ) = 2πR²(1 − cos θ) / (2πR sin θ) = R·tan(θ/2)`  **[D]**

So each router can be scored against a **true value**, not merely against "it should look
uniform." (Tarboton 1997 uses exactly this test.)

| router | conservation | mean error | **error at the defects** |
|---|---|---|---|
| **8-nbr Moore MFD (the status quo)** | **1.000000000000** | 20.76 % | **60.59 %** |
| edge MFD (no diagonals) | **1.000000000000** | 12.13 % | 41.56 % |
| **gradient-projected edge flux** | **1.000000000000** | **5.17 %** | **19.67 %** |

**[M]**

### The honest reading — and it is **not** the one the hypothesis predicted

- **MFD does NOT leak at a corner.** All three routers conserve to **1.000000000000**. A
  normalised-weight router conserves on *any* graph, valence-3 included: the weights sum to one
  whether there are 8 neighbours or 7 or 3. **The corner was never a conservation bug.** **[M]**
  This is exactly Addendum A1's argument — *"conservation does not depend on valence; flux is a
  boundary integral; a 3-edge cell closes exactly"* — **confirmed by measurement.**

  > **⚠ AND IT FALSIFIES HALF OF THE PREDICTION THAT COMMISSIONED THIS TEST.**
  > `DECISIONS[corner-pathology-is-a-diagonal-artifact]` (status *proposed*) asked us to
  > *"show edge-flux routing closes at valence 3 **where 8-neighbour MFD cannot**."*
  > **The second clause is false.** 8-neighbour MFD closes at valence 3 perfectly well — it
  > conserves to 1.000000000000, exactly like the others. Nothing about conservation breaks there.
  > The decision's *conclusion* (reformulate as edge flux; the 24 cells stop being special) is
  > **upheld** — but on **accuracy and code structure**, not on closure. Recorded because the
  > difference matters: had we gone looking for a conservation bug at the corners, we would have
  > found nothing and concluded the corner was fine — and missed that the **fan is wrong
  > everywhere**, which is the actual defect (20.76 % mean error on the whole grid). **[M]**
- **What the corner actually breaks** is (a) the **code's** structural assumption of eight fixed
  `(dx,dy)` offsets, which indexes a cell that does not exist, and (b) the fan's **directional
  weighting**.
- **The routers separate on ACCURACY, and they separate over the WHOLE grid, not at the corners.**
  Grid-aligned bias is a **face-wide** defect of the 8-neighbour fan (20.76 % mean error), not a
  corner defect. **The corner is a red herring; the fan is the bug.** **[M]**
- **Joseph's proposed replacement is confirmed on every count.** Gradient-projected edge flux
  ("D-∞ generalised to a mesh") **needs no diagonals, works at any valence, conserves exactly**,
  and is **4× more accurate than the status quo** (5.17 % vs 20.76 %) and **3× better at the
  defects** (19.67 % vs 60.59 %). **[M]**

### And the accuracy verdict on the defect (the A1 test)

Splitting the harmonic error by distance from the 24 defect cells, under the corrected scheme:

| grid | cells near a defect | NEAR: error | order | FAR: error | order |
|---|---|---|---|---|---|
| cube · equiangular (OURS) | 384 / 24 576 | 1.28e-3 | **0.91** | 3.19e-4 | **1.55** |
| cube · Snyder equal-area | 384 | 1.75e-3 | 0.92 | 8.35e-4 | 1.36 |
| HEALPix | 384 | 1.28e-3 | 0.85 | 5.13e-4 | 1.35 |

**[M]**

> **⇒ ADDENDUM A1 IS CONFIRMED, AND THIS IS THE ARGUMENT THAT DECIDES THE GRID DOES NOT NEED
> REPLACING.** The topological defect is **demoted** — from a *conservation failure* (which would
> accumulate globally and destroy the physics) to a **bounded local accuracy wart**: the error near
> the defect is ~4× the far-field error, over 384 of 24 576 cells, and it **still converges**
> (order 0.91, first-order, vs 1.55 in the far field). It degrades; it does not diverge; it does
> not leak. **[M]**
>
> **Those are not comparable harms.** A 24-cell first-order region on a planet is a rounding error
> against re-architecting the world.

---

## 7. The two-grid overlay (cube ∪ dual octahedron) — right mechanism, wrong target

The construction (Joseph's, worked out in `seam-adjacency-findings.md` §A2): a cube's 8 corners
cannot be covered by another cube, but its **dual octahedron** is perfectly complementary — the
octahedron's 6 vertices sit exactly at the cube's face centres, and its 8 faces exactly at the
cube's corners. Delaunay of the 14 singular directions = the **rhombic dodecahedron**; Voronoi =
the **cuboctahedron** (8 triangular cells around the cube corners, 6 square cells around the axes).
Partition by *"use whichever grid is locally regular"* and **no cell is ever evaluated at a
singularity.** The interface irregularity is **fractional** (partial edges, exactly conservative —
a mortar interface), which really is gentler than an integer valence defect. **The intuition is
correct.**

It still loses, and here is the number that decides it. The cuboctahedral interface is the 24 edges
joining the 12 cube-edge-midpoint directions; adjacent ones subtend 60°, so the interface is a
closed 1-D network of total length `24 · (π/3) · R = 8πR ≈ 25.1 R`. **[D]**

| level | cells / planet | **cube DEFECT cells** | **overlay INTERFACE cells** | ratio |
|---|---|---|---|---|
| L4 | 1.5 k | **24** | 256 | 11× |
| L7 | 98 k | **24** | 2.0 k | 85× |
| L10 | 6.3 M | **24** | 16.4 k | 683× |
| L13 | 403 M | **24** | 131 k | 5 461× |
| L16 | 25.8 G | **24** | 1.0 M | 43 691× |
| **L19** | **1 649 G** | **24** | **8.4 M** | **349 525×** |
| L25 | 6.8 P | **24** | 537 M | 22 369 621× |

**[M/D]**

> **O(1) versus O(N).** The cube's defect is **24 cells** — not per face, not per level: 24 cells
> on the whole planet, **forever**, at every resolution. The overlay's interface is a 1-D network
> that grows **linearly** with resolution (`16·2^L` cells). At **L19** — the macro tier vivarium
> already runs — that is **8.4 million interface cells**, each needing partial-edge geometry, a
> neighbour search, and (for the strictly-**ordered** drainage sweep, §5.5) a **cross-grid
> dependency**. Plus two grid types, two addressing schemes, and a broken `CellId` quadtree.
>
> **It pays a SCALING cost to fix something §6 just measured as already nearly free.**
> **Recommend against.**

**But the mechanism is right, and it has a home.** The non-matching-interface / partial-edge
(mortar) formulation is **exactly the correct treatment for our actual seam — the coarse↔fine TILE
boundary**, which is non-matching by nature and unavoidable. That is what Berger–Colella flux
registers *are*. **Right mechanism, right target: apply it where the interface is forced on us, not
where the defect dissolves for free.** (Addendum A3.)

---

## 8. The rhombic-dodecahedral base — the most interesting row in the table

12 quadrilateral faces; a rhombus subdivides into four rhombi, so **it quadtrees**; 12 faces means
half the solid angle per face. Falls out of §7. **[D]** Euler checks: `14 − 24 + 12 = 2`, and
`Σ(4−v) = 8·1 + 6·0 = 8` — **the same irreducible charge as the cube**, confirmed by measurement
**[M]**.

**Does the smaller face pay?** Measured, at matched cell counts:

| | cube · equiangular | **rhombic-dodec · tan-warp** | cube · gnomonic | rhombic-dodec · gnomonic |
|---|---|---|---|---|
| area spread | 1.397× | **1.519×** | 5.031× | **2.735×** |
| edge spread | 1.414× | **1.355×** | 2.099× | **1.713×** |
| non-orthogonality | 28.79° | **29.33°** | 29.22° | **29.45°** |
| skew (max) | 24.69 % | **24.84 %** | 25.00 % | **25.00 %** |
| corrected accuracy | 3.65e-4 | **5.71e-4** | 6.81e-4 | **5.14e-4** |
| sync rounds (§5.5) | **3** | 4 | 2 | 3 |

**[M]** **The honest verdict — and it is the caveat, not the hope, that the numbers support:**

- **Against the GNOMONIC baseline the RD is a large win**: area spread **5.03× → 2.74×**, edge
  spread 2.10× → 1.71×, accuracy 6.8e-4 → 5.1e-4. **Halving the face solid angle really does halve
  the projection distortion.** The mechanism is real. **[M]**
- **Against our EQUIANGULAR cube it is a wash or slightly worse.** Area 1.40× → 1.52×,
  non-orthogonality 28.8° → 29.3°, accuracy 3.65e-4 → 5.71e-4, and it costs a sync round. **[M]**
- **The caveat the coordinator flagged is the one that bites**: the rhombic faces are **not square**
  (diagonals in ratio √2, angles 70.53°/109.47°), and **the baked-in shear hands back exactly what
  the smaller face wins.** Non-orthogonality is *unchanged* (~29°) — the shear replaces the
  face-size distortion one-for-one.
- **Measure, don't assume** was the right instruction, and the measurement says: **the equiangular
  tan-warp on the cube has already collected most of what the RD offers**, by a different route.

**And there is no Snyder equal-area map for it.** Snyder's derivation requires **regular** polygons
(*"any polyhedral globe that has regular polygons"* **[P]**) and a rhombus is not one. So the RD
cannot be given an exactly-equal-area map by Snyder's method; one would need new derivation. **[D]**

> **Except that it already has one, and it is called HEALPix** (§1.2): HEALPix's base *is* the
> rhombic dodecahedron, and HEALPix *is* exactly equal-area with a working NESTED quadtree. So the
> "equal-area 12-quad-face base with LOD" grid **exists**, is **implemented here**, and is
> **measured** — and on the metric that matters (§5.2) it comes **last among the quad grids**
> (8.44e-4), because iso-latitude costs it 36.7° of non-orthogonality and 49.6 % skew. **[M]**
>
> **That closes the RD lead.** Not because the idea was wrong — it was a good idea, and its
> mechanism is sound — but because the two ways of cashing it in have both now been measured and
> neither beats what we already have.

**The RD's own tan-warp is OURS, not sourced.** A genuinely-tuned equal-angle map for the rhombus
might do better than our naive parallelogram tan-warp. That is the one live thread here **[⊘]**;
see §10.

---

## 9. Recommendation

> # Keep the equiangular cube-sphere. Change the kernels.

The conclusion **survives** contact with measurement. **But the reason it was previously given is
wrong, and the corrective work is bigger and more specific than "carry the metrics."**

### What was thought, and what is actually true

| previously | measured |
|---|---|
| *"Conservation is a scheme property; isotropy/shape is a grid property that no scheme fixes."* | **Half right.** Conservation is a scheme property ✓. But **consistency** — solving the right equation at all — is a property of the scheme **and the grid together**, and a two-point FV flux is **inconsistent** on our grid (§5.2). |
| *"Corner anisotropy is 1.16–1.25× with exact geometry and does not shrink with refinement."* | **Superseded.** That was a two-point scheme on a single open face. Whole-sphere: **1.0025**, and **1.0000** with the corrected scheme (§5.3). |
| *"Equal-area trades away the thing we fix for free to worsen the one thing we cannot fix."* | **Right conclusion, wrong mechanism.** Equal-area does buy only `cell_area` (§5.4) — but the thing we "cannot fix" (isotropy) turns out to be **largely fixable in the scheme**, and equal-area's real cost is **shape**: HEALPix has the worst skew and non-orthogonality in the report. |
| *"The 24 valence-3 cells are irreducible and must be declared as a known defect."* | **Confirmed and DEMOTED.** Irreducible ✓ (Euler). But not a conservation failure — a **bounded local accuracy wart** that still converges (§6). |

### ⚠ FIRST — the ladder, not modern Earth

`DECISIONS[check-the-ladder-not-modern-earth]` is a **standing guard**, and it bites this report's
recommendations. I had them in the wrong order until I read it.

**The ordinum says this world is supposed to be SUBMERGED.** `promise[emerged-land]` is an Abyssal
(Phase-3) **`:tag gate`** that **nothing currently keeps**; `charge[erosion-carving]` is itself a
Phase-3 gate. Land is *forbidden* as an initial condition
(`DECISIONS[water-world-is-the-promise-not-the-bug]`).

⇒ **`erosion.rs`'s fluvial routing — the kernel this report's §6 is most excited about — is not
licensed to run yet.** There is nothing to carve until land exists. Fixing its MFD is *correct*
work, and it is **not urgent work**, and I would have ranked it #3 on physics grounds alone.

**`water.rs`, by contrast, is live now** — Phase-1/2 is a water-covered surface, and water is
already in flux form (§6). **The scheme fixes below land on water immediately and on erosion
later.** That reordering is the ladder's, not mine.

### The work, in priority order

0. **The `emerged-land` gate (`uplift`) is upstream of all of this** and is not a grid question.
   The ladder's own verdict, not ours: `DECISIONS[ordinum-governs-the-flux-web]`.

1. **Fix the flux, not the grid — and fix it properly.** A corrected face gradient: project both
   cell centres onto the edge **normal** through the **mid-edge**, extrapolate with a reconstructed
   gradient. **Exactly conservative** (antisymmetric under `i↔j`). **2500× on our grid.** **[M]**
   *This is the highest-value item in the entire investigation and it was not on anyone's list.*
2. **Use the WIDE gradient stencil.** A quadratic LSQ fit over the 8 Moore neighbours. A linear fit
   over 4 edge-neighbours is only first-order and **caps the whole scheme at ~0.5 order** — you get
   a third of the available gain and none of the convergence. This is also what makes the corner
   cells work (7 ≥ 5 points). **[M]**
3. **Replace `erosion.rs`'s 8-neighbour MFD with gradient-projected edge flux.** Needs no
   diagonals, works at any valence, conserves exactly, **4× more accurate**, **3× better at the
   corners**, and the 24-cell special case simply evaporates. **Clamp it to strictly-downhill edges**
   (§1.1b). **[M]** — *but see the ladder note above: this kernel is gated behind `emerged-land`
   and is not on the critical path today.* **Water's θ flux-smoothing halo (§6) is.**
4. **Metric planes per `(face, level)`.** `ΔA`, edge lengths, arms, and the non-orthogonality factor
   are analytic in `(u,v)` and **identical for every tile on a face**. A few f32 planes. This is
   Putman & Lin §3 **[P]** and `seam-adjacency-findings.md` §5's M1 — still right, still cheap,
   still first.
5. **The tile seam** — Berger–Colella flux registers with 2:1 balance, and Putman & Lin's shared
   edge value at the 12 face edges. **This is where the mortar/partial-edge mechanism belongs**
   (§7, Addendum A3). The corrected `seam_ridge` number says *what* is broken, precisely: the seam
   is **2.45× (18e) / 3.76× (60e) / 5.79× (150e)**, **growing with the differential age gap while
   the interior stays flat** — which is exactly the signature of **mean-pin conserving block means
   but NOT boundary gradients** (`DECISIONS[seam-probes-were-measuring-seabed]`, verified primary).
   That is Berger–Colella *case (i)* without *case (ii)*. The flux register is the missing half.
6. **Do NOT** switch projections. Do **NOT** build the two-grid overlay. Do **NOT** adopt hexagons.

### What we would tell someone with a *different* algorithm

- **"I only need conservation."** Any grid. Use finite volume. Done. (§5.1)
- **"I need an accurate Laplacian/diffusion and I can pick my grid freely."** **Hexagonal SCVT** —
  exactly orthogonal, second-order with a plain two-point flux, no correction needed. It is the
  best grid in this report on accuracy and it is not close. (§5.2)
- **"I need an accurate Laplacian and I need LOD."** **Any quad grid + the corrected scheme.** The
  grid barely matters (3.6e-4 to 8.4e-4 across all five); **the scheme matters 2500×.** (§5.2)
- **"I need constant cell area."** Snyder or HEALPix — **but read §4.1 first**: your *discrete*
  cells are not equal-area unless you carry non-geodesic boundaries.
- **"My bottleneck is a globally-ordered sweep (Priority-Flood, drainage)."** **Fewest partitions
  wins: the cube-sphere.** And note **every** grid's partition graph has cycles — that problem is
  not solvable by projection. (§5.5)
- **"I care about shallow water and the flat-plane assumption."** It is legitimate on every grid
  here, at every rung. Not a grid question. (§4.7b)

---

## 9b. The standing guards, applied to **this report**

The project's own guards exist because they were earned. Turning them on my own work:

### "A probe that cannot fail is not a probe." (`DECISIONS[seam-probes-were-measuring-seabed]`)

**Can these probes fail? Demonstrably yes — they did, repeatedly, and that is the best evidence
the numbers mean anything.** During this session:

| the probe | how it failed, before it passed |
|---|---|
| Snyder Table-1 gate | fired 3× — first ω = 5.67° (bad Tissot normalisation), then 41.5° (sampling *on* the projection's cusp), then a bad `a` (couldn't reach the ρ→0 limit). Only then 25.17°. |
| cube seam assertion | fired at 1.3e-10 — and that is how the paper's rounded `g` was found (§1.1a). |
| HEALPix base-pixel inversion | **panicked**: eq (17)/(18) would not yield integer indices, forcing the boundary-lattice derivation. |
| routing conservation | returned **0.000** — the uphill-routing trap (§1.1b). |
| harmonic accuracy | returned **negative convergence orders** — a result nobody wanted and I initially disbelieved. |

**One probe I am NOT confident in, and I will say so:** the **sequencing** cyclicity test returns
**"YES" for all nine grids** (§5.5). A test whose answer is the same for every input is weakly
informative *by construction*, and I cannot rule out that it would say YES for *any* terrain and
*any* partition. It may be measuring "a random flow field on any closed surface has cross-partition
cycles," which is nearly a tautology. **The crossing-% and sync-round columns discriminate and are
sound; the cyclic column should be treated as a null result until someone builds a case it could
fail on.** **[⊘]**

### "An alarming number that CONFIRMS your prior gets less scrutiny than a boring one."

**The 2500× is exactly that shape** — it confirms the answer everyone wanted ("keep the grid,
change the kernels"). So:

- **Is the baseline a strawman?** No. Three independent checks: (a) the **hex mesh gets 4e-4 through
  the identical code path**, so the harness is not broken; (b) the operator's best-fit slope against
  the exact answer is **1.02**, so it is right *on average* and only wrong *locally*; (c) a negative
  convergence order is the **textbook signature** of two-point flux on a non-orthogonal mesh. The
  baseline is real.
- **Is it measured against what vivarium actually runs?** Vivarium runs the **naive uniform** kernel
  (9.58e-1), not two-point FV (9.20e-1). Against the *real* status quo the gain is **2625×** — same
  number, so nothing is hiding there. **[M]**
- **⚠ THE SCOPE LIMIT THAT MATTERS, AND IT IS A REAL ONE:** the 2500× is measured on a **Laplacian**
  (diffusion / `creep`). **It is NOT measured on the fluvial kernel**, which is the kernel we most
  care about. The routing improvement is measured separately and is **4×**, not 2500× (§6).
  **Anyone quoting "2500×" as the payoff for erosion is quoting it wrong.** **[⊘]**

### "No claim without its probe." (`DECISIONS[probes-before-claims-no-plausibility-as-verification]`)

Everything load-bearing is tagged **[M]**. The **[I]** claims — that the metric planes hoist per
`(face, level)`; that a precomputed stencil matrix would make the corrected scheme cheap; that
stream power inherits the area/slope errors — are **inferences, not results**, and §10 lists them.
**The single biggest unprobed claim in this report is that the corrected scheme is affordable
(§10.6). It should be benchmarked before §9 item 1 is committed to.**

---

## 10. What we could not resolve

1. **[⊘] An equal-angle (not merely tan-warped) map for the rhombic dodecahedron.** Our RD tan-warp
   is **ours**, by analogy, not from a source, and it is the weakest-provenance grid in the report.
   A properly-derived map for the rhombus might beat it. The RD's *gnomonic* result (a real win over
   the cube's gnomonic) shows the face-size mechanism is genuine; whether a good intra-face map can
   convert that into a win over the *equiangular* cube is **open**. §8 says probably not — the √2
   shear looks like it eats the gain — but that is one construction, not a proof.
2. **[⊘] Why the corrected order saturates at ~1.6 and not 2.0 on the quad grids** (the hex meshes
   reach 1.93–1.95). Two hypotheses tested and refuted (centroid centres; defect contamination —
   §5.2). Likely the remaining LSQ-gradient error on a distorted stencil, but I did not prove it.
   Practically it does not change any conclusion; theoretically it is unfinished.
3. **[⊘] Stream power end-to-end.** `E = K·A^m·S^n` is **inferred** from the area and slope columns
   (§4.1, §4.3), not simulated. The drainage-area term is measured (§6); the slope term is not
   independently exercised. Someone running the real fluvial kernel on these grids would get a
   sharper answer than our inference.
4. **[⊘] The sequencing numbers depend on the terrain.** One band-limited field. The *ordering* of
   the grids was stable across the seeds tried, but that is not a proof, and the partition-count
   confound (§5.5) is real.
5. **[⊘] HEALPix edge lengths are great-circle chords, not its true boundaries.** Its real
   boundaries are non-geodesic (§4.1); we carry the paper's exact Ω for area and chords for edges.
   The area gap is measured (9.96 %); the *edge-length* error from this is **not** separately
   quantified. HEALPix's poor showing in §5.2 is therefore a mild **under**-estimate of its
   quality — though not nearly enough to change its ranking.
6. **[⊘] Whether the corrected scheme's cost is acceptable in the hot loop. THIS IS THE BIGGEST
   OPEN RISK IN THE REPORT.** The wide quadratic gradient is a 5×5 solve per cell per step, on top
   of a per-edge flux with a projection. **Nobody has benchmarked it, including me.**
   For scale, the two committed measurements it has to live beside:
   - `spatial-key-bench.md` **[P, read]**: the dense Cartesian 5-point stencil — the automata inner
     loop — runs at **~6 G cell-updates/s** while the working set fits L2/L3, with a **cache cliff
     at ~17 MB/field** (a 2 km patch). A full erosion pass on a 512² patch ≈ **44 µs**.
   - The **−4 % to +11 %** figure for adjacency-as-data is **not** from `spatial-key-bench.md` — it
     is from `seam-adjacency-findings.md` §4.1, a **scratch benchmark that was never committed**.
     I have not reproduced it. *(This report cited it wrongly in draft; corrected here.)* **[⊘]**

   The corrected scheme is a **much** bigger ask than either. It plausibly wants a **precomputed
   per-cell stencil matrix** — the geometry is *static*, so the 5×5 normal-equation pseudo-inverse
   can be factored **once per `(face, level)`** and reused, which would make the hot loop a fixed
   set of multiply-adds. That would likely make it cheap. **But that is a claim [I], not a
   measurement, and it should be benchmarked before item 1 of §9 is committed to.**

### Corrections to the record

- **`seam_ridge`'s "22888" is retired.** It was `seam_median ÷ 1e-9` — a divide-by-zero artifact on
  a 100 %-submarine footprint where the fluvial core provably could not execute
  (`seam-adjacency-findings.md` §0). **The real seam is 2.45× (18 epochs) / 3.76× (60e) / 5.79×
  (150e)** — verified primary in `DECISIONS[seam-probes-were-measuring-seabed]` and `ORIENTATION.md`
  §"the seam", and independently corroborated by the honest 2026-07-03 measurement in
  `DESIGN-SYSTEMS` (4.3×/5.3×/7.1×). It **grows with the differential age gap while the interior
  stays flat (0.079 → 0.103)** — the signature of mean-pin conserving block *means* but not boundary
  *gradients*. Any document still citing 22888 as the size of the seam defect is wrong.
  *(I first took these figures on trust from a hand-off message and put them in this report
  unverified — which is the very failure the 22888 lesson names. They are now checked against the
  repo. Recorded so the next reader knows they were checked, not relayed.)*
- **The corner-anisotropy figure (1.16–1.25×, "does not converge away")** is superseded — see §5.3.

---

## 11. Reproducing every number

```
cargo run --release -p vivarium-world --example grid_lab
```

~11 s. Prints every table in this report, in order, with the gates first. If a gate fails, nothing
below it is worth reading — and the gates are assertions, so it will not print anyway.

Code: `crates/vivarium-world/examples/grid_lab/`

| file | what it is |
|---|---|
| `mesh.rs` | the universal `Mesh`: combinatorial adjacency, per-edge joint geometry (edge, both arms, skew, non-orthogonality), Euler asserted at construction |
| `grids.rs` | cube × {equiangular, gnomonic, **Snyder**} + rhombic dodecahedron × {gnomonic, tan-warp}. **Snyder 1992 eqs (1)–(23) transcribed; the exact `g = arctan √2`** |
| `healpix.rs` | Górski §5.1/§5.3. The corner-lattice derivation, and the checks that prove it |
| `icosa.rs` | geodesic triangles; the Voronoi dual; Lloyd/SCVT **with Delaunay edge flips** |
| `probes.rs` | geometry / curvature / strata / class census / the four schemes / the harmonic accuracy test |
| `flow.rs` | drainage sequencing; the three routers; the cone test |
| `main.rs` | the gates, then §1–§10 in order |

### Project documents read for this report

**Read primary:** `CLAUDE.md` · `ORIENTATION.md` · `ref/research/seam-adjacency-findings.md`
(full, incl. **Addendum A1–A4**) · `DECISIONS.decision-log.udon` (the grid/seam/process entries,
in full — incl. the three **standing guards** applied in §9b) · `crates/vivarium-world/src/sphere.rs`
· `erosion.rs` (the MFD / `accumulate_drainage` path) · **`water.rs`** (verified its flux form
directly rather than inheriting the claim — and found the θ-smoothing halo, §6) ·
`ref/research/spatial-key-bench.md` · the project memory index.

**NOT read, and therefore not reflected here [⊘]:** `doc/theory/multiscale-seams.md` and
`multiscale-methods.md` (the operator algebra and the `z`-reconciliation question) ·
`doc/ARCHITECTURE.md` · `ASSUMPTIONS.md` · `LEXICON.udon` · `tabularium/terrestris.ordinum.udon`
(I have the ordinum's phase gating **secondhand**, from the decision log — §9b's ladder note rests
on that, and someone should check it against the artifact itself).

### Sources read directly for this report

- **Snyder, J.P. (1992).** *An Equal-Area Map Projection for Polyhedral Globes.* Cartographica
  **29**(1):10–21. Read in full (PDF supplied by Joseph). Forward eqs (1)–(16), inverse (17)–(23),
  Table 1, and the distortion appendix (A1)–(A23).
- **Górski, K.M. et al. (2005).** *HEALPix — A Framework for High-Resolution Discretization…*
  `ref/research/pdfs/markdowns/astro-ph0409513/`. §5.1 (centres), §5.2 (NESTED), §5.3 (boundaries).
- **Putman & Lin (2007)**, **Berger & Colella (1989)**, **Westerteiger et al. (2011)** — read in the
  sibling pass and quoted from `ref/research/seam-adjacency-findings.md`, not re-read here.
- **Tarboton (1997)** — the cone test for specific catchment area. **Not read primary**; the test is
  reconstructed from its closed form, which is derived here (§6). Cited for provenance of the
  *idea*, not for any number. **[⊘]**

---

*Companion document: `ref/research/seam-adjacency-findings.md` — the literature side, the seam/
refluxing method, and Addenda A1–A4. This report supersedes its §4.2 decomposition (see §5.2) and
its §5 corner-anisotropy figure (see §5.3); everything else in it stands.*
