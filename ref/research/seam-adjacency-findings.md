# Seam adjacency, sphere geometry, and conservative coupling — findings

*Written 2026-07-12, against the research lead `DECISIONS[seam-grid-leads-snyder-healpix]`
(Joseph, 2026-07-12: "HEALPix and Snyder Equal-Area Cubed Spheres offer mathematically
rigorous, distortion-free vertex distributions"). Sources read directly: Górski et al.
2005 (HEALPix), Westerteiger et al. 2011 (HEALPix terrain rendering), Youngren & Petty
2017 (MRH), Berger & Oliger 1984, plus two chased down in this pass — **Berger & Colella
1989** (the actual refluxing algorithm) and **Putman & Lin 2007** (the operational
cubed-sphere finite-volume core). Code read: `sphere.rs`, `sample.rs`, `erosion.rs`,
`water.rs`, `examples/seam_ridge.rs`. Third-party code read: `~/src-ext/QuadSphere`.*

*Epistemic markers inline: **measured** (I ran it, numbers below reproduce), **primary**
(quoted from a source I read), **derived** (a proof I did here), **secondary** (a source
I did not read primary), **could-not-verify**. Nothing here is asserted from memory.*

> [!important]
> **Two findings in §0 land before any of the source work, and they change what the seam
> problem *is*.** The headline `seam_ridge` number is a divide-by-zero artifact, and the
> erosion kernel's own tests run on terrain where the fluvial core cannot execute. Read
> §0 first. The rest of the doc stands on its own, but the *priority* it implies depends
> on §0.

> [!note]
> **Convergence with `grid_lab` (`cec4057`, landed mid-pass, same session).** A sibling
> instrument attacked the same question **by measurement**, from the code side, while this
> pass worked from the source side. **The two agree without having talked**, which is worth
> more than either alone:
>
> | | `grid_lab` (measured, code) | this doc (measured + primary sources) |
> |---|---|---|
> | equiangular area ratio | 1.41× non-equal-area, converged | 1.4099 at N=256 (§1) — same |
> | naive uniform kernel | **leaks mass: ~10 % at L4, 5.4e-4 at L8** | static geometric error 40–50 % over 56 % of a face (§1) |
> | finite-volume path | **conserves exactly (~1e-15), any level, any placement** | Putman & Lin: *"mass is conserved to machine precision"* (§3.6) |
> | corners under FV | **anisotropy 1.16–1.25×, does NOT shrink with refinement** | 120° coordinate axes; irreducible by Euler (§1, §2) |
>
> **`grid_lab`'s decomposition is the right frame and this doc adopts it**: *conservation is
> a **scheme** property* (finite-volume fixes it exactly, on any grid) — *isotropy/shape is a
> **grid** property* (no scheme fixes it; it never converges away). Everything below is
> consistent with it, and §3 supplies the literature `grid_lab`'s header asks for — see §7.2
> for the direct handoff.

---

## 0. What the code actually does — verified first, before reading anything

### 0.1 `seam_ridge`'s "ratio ≈ 22888" is `seam_median ÷ 1e-9` *(measured)*

`cargo run --release -p vivarium-world --example seam_ridge` reproduces `SEAM RIDGE
RATIO: 22888.18` — **identically for all three age gaps (18 / 60 / 150 fine epochs)**.
That invariance is the tell. Re-running the probe's own arithmetic with full precision
(scratch binary, not committed):

```
seam     n= 190 median 2.288818e-5 m   max 5.7335e-3
interior n=1254 median 0.000000e0  m   max 2.4414e-4   <- the DENOMINATOR
```

The interior median is **exactly zero**. The probe computes
`ratio = ms / mi.max(1e-9)`, so `2.288818e-5 / 1e-9 = 22888.18` — the number is the seam
median divided by the epsilon floor. It carries no information about the seam beyond the
seam median itself, which is **0.023 mm**. The interior *maximum* second difference is
`2.4414e-4 m` = exactly one f32 ULP at a 3712 m datum (2⁻¹² = 2.44140625e-4) — i.e. the
"interior curvature" field is float quantization, not terrain.

**`seam_ridge` is red, but not for the reason it reports, and its magnitude is not
evidence of anything.** (`ORIENTATION.md` and `multiscale-seams.md` §2.1 both treat the
22888 as the measured size of the seam defect. It is not.)

### 0.2 The erosion kernel's fluvial core never executes at its own test footprint *(measured + derived)*

`gen::SEA_LEVEL_M = 4000.0`. `Fluvial::outlets()` marks a cell an outlet if
`is_edge || h[i] <= sea`. At the footprint used by **all three** `erosion.rs`
`fluvial_tests` (`small()` = `from_prior(0, ZPos, 19, 165_800, 413_600, 96)`) **and** by
`seam_ridge` (same origin, `nx=128`):

```
erosion.rs fluvial_tests::small()   L19  96^2 @(165800,413600): h 3709.2..3713.5 m | below sea: 100% | max|dh| after 80 epochs: 0.000 m
seam_ridge macro tier               L19 128^2 @(165800,413600): h 3709.2..3714.9 m | below sea: 100% | max|dh| after 80 epochs: 0.000 m
a LAND footprint, for contrast      L19  96^2 @(400000, 60000): h 4550.2..4593.6 m | below sea:   0% | max|dh| after 80 epochs: 10.215 m
```

Every cell is ~290 m **below** the sea-level datum. The consequence is not "less erosion",
it is **none**, and it follows from the code by inspection: all cells outlets →
`receivers()` sets `recv[i] = i` for every cell → `incise()` hits `if r == i { continue }`
for every cell → `before == h` → `deposit()` routes `qs = 0`. **Priority-Flood, D8, MFD
accumulation, implicit stream-power incision, and Davy–Lague deposition — the entire
fluvial pipeline — provably cannot run there.** Only `talus` (no-op at 0.2 % slopes) and
`creep` execute, and creep's per-epoch increment falls below the f32 ULP at that datum, so
80 epochs move the terrain by **0.000 m — bit-exactly nothing**.

The three unit tests still pass, for reasons worth naming:
- `deterministic_bit_identical` — compares two no-ops.
- `pin_preserves_parent_means` — its comment says `// drifts the means (mass exported to
  outlets)`. **Nothing drifts.** It then pins to the seed surface, which is what `h`
  already is.
- `channels_concentrate_and_stay_finite` — passes because `accumulate_drainage` runs
  regardless of outlets and concentrates on the prior's residual noise. **No channel is
  cut.** The assertion is about the accumulator, not about incision.

Scope of the finding, stated precisely: the four call sites that build a `Fluvial`
**without** uplift are exactly these four (the three tests + `seam_ridge`). The world path
(`query.rs:257`) and the worldview testbench (`spikes/worldview/src/main.rs:414,691,698`)
both drive uplift, and `outlets()` is recomputed per epoch, so sustained uplift can lift a
submarine footprint past the waterline and start incision. **Those paths are not implicated
by this finding.** *(Not verified: whether worldview's `0.05 m/epoch` actually clears the
287 m deficit at this footprint in a normal session — it would take ~5 700 epochs.)*

Related and unresolved: `ORIENTATION.md` advertises "a good mountainside:
`VIVARIUM_FOCUS_I=5308416 VIVARIUM_FOCUS_J=13238272` (L24)". That is the *same
neighbourhood* (÷32 → L19 cell 165 888, 413 696). Its L24 prior is **3713.0–3713.4 m over
256 cells** — 100 % submarine and essentially dead flat (0.4 m of relief across 256 cells).

### 0.3 The three grid assumptions in the brief — confirmed, with one correction

| claim | verdict |
|---|---|
| kernels assume a flat, regular, uniform grid | **confirmed.** `erosion.rs:189` `const NEIGHBORS: [(i32,i32); 8]` + `j = (y+dy)*nx + (x+dx)`; one `cell_m`, one `cell_area = cell_m*cell_m`; diagonals hard-coded `cell_m * SQRT_2`. `water.rs` the same: one `cell_m`, `area = l*l`, pipes at `i±1`, `i±nx`. |
| the world is an **equiangular** cube-sphere | **confirmed.** `sphere.rs:65` — `(self.u * FRAC_PI_4).tan()`. This is exactly Putman & Lin §3.1's `x = a·tan(x′)`. |
| `sample::cell_size_m` returns one uniform size per level | **confirmed.** `sample.rs:20` — `(FRAC_PI_2 * R) / 2^level`. |
| the error "grows toward the 8 cube corners" | **partly wrong — see §1.** The *scale/area* error peaks at the **midpoints of the 12 cube edges**, not the corners; only the *shape/orthogonality* error is a corner phenomenon. `sphere.rs`'s own module doc ("mild area/shape distortion toward the 8 cube corners") is imprecise in the same way. |
| tiles do not compose; zero external discharge | **confirmed.** `water.rs::hold_edge_sea` holds only edge cells at/below the waterline; every other border flux is clamped to 0 (`if x < nx-1 {...} else { 0.0 }`). `ORIENTATION.md` already declares it: "tiles hydrologically isolated pending flux-BC". |

---

## 1. The geometry of the grid we have — measured

Replicating `sphere.rs::to_unit` exactly and integrating spherical quadrilaterals by
spherical excess *(measured; scripts in the session scratchpad, not committed)*:

**Total area is right; the distribution is wrong.** Face area sums to `2.094395 = 4π/6`
exactly at every resolution. The grid is globally area-correct and locally area-wrong.

**`cell_size_m` is the face-centre spacing, and everything else is smaller.**
`cell_size_m(L) = (π/2·R)/2^L` equals, to all printed digits, the true centre-to-centre
arc distance **at the face centre** — and that is the *maximum* over the face.

| quantity, at N=256 cells/face-edge | true ÷ what the code assumes |
|---|---|
| centre-to-centre distance, face centre | 1.000 |
| centre-to-centre distance, worst cell | **0.709** (code overstates by 41 %) |
| cell **area**, face centre | 1.000 |
| cell **area**, face **corner** | 0.770 |
| cell **area**, **face-edge midpoint** ← the worst | **0.716** (code overstates by 40 %) |
| D8 **diagonal** distance (`√2·cell_m`), corner cell | **0.668** (code overstates by 50 %) |
| in-cell **aspect ratio** du:dv, worst | **1.41** (at a face-edge midpoint) |
| fraction of a face where du or dv is >10 % off `cell_size_m` | **56 %** |
| fraction of a face where du or dv is >20 % off | **16 %** |

Two different defects, with two different geographies — this is the correction to the
received framing:

- **Scale / area distortion** peaks at the **midpoints of the 12 cube edges** (0.716), is
  *milder* at the corners (0.770), and is **not localized** — it exceeds 10 % over **56 %
  of every face**. It is a face-wide systematic bias, not a corner artifact.
- **Shape / orthogonality distortion** peaks at the **8 corners**, and only there. The
  angle between the u- and v-coordinate lines is **exactly 90° along both face centrelines
  and along the face edges**, and degenerates to **exactly 120° (and 60°) at the corners**:

  ```
  (0.000000, 0.000000) face centre       90.000 deg
  (0.999999, 0.000000) edge mid          90.000 deg
  (0.900000, 0.900000) near corner      114.947 deg
  (0.999999, 0.999999) AT corner        120.000 deg
  ```

  **This is independently confirmed by a primary.** Putman & Lin 2007 §1: *"the conformal
  mapping is only orthogonal in the interior, with coordinate lines still intersecting at
  the 8 corners at a **120-degree angle**. Therefore, some modifications to the algorithm
  still need to be made to counter the non-orthogonality near the corners."* And §3.2:
  *"The corner singularities remain non-orthogonal with angular deviation from 90 fixed at
  30[°]"*. Two independent derivations, same number.

**What this costs the physics, worst case** *(derived)*. Stream power `E = K·A^m·S^n`
(m=0.5, n=1), at a maximally-distorted cell: drainage area `A` carried as `cell_m²` is
overstated → `A^0.5` off by −16 %; slope `S = Δh/dist` with `dist` overstated → `S` off by
**+41 %**; net `E` off by ≈ +19 %, with the two errors *partially cancelling by accident*.
The MFD/D8 **diagonal** is the worst single term (50 %). And the 5-point Laplacian in
`creep()` and the D8 stencil are simply **the wrong operators at the 8 corners**, where the
axes are 120° apart — no choice of `cell_m` can fix a non-orthogonal stencil.

**And what it costs *dynamically* is worse than the static error suggests** *(measured by
`grid_lab`, `cec4057`)*: run linear diffusion — whose mass invariant is *exactly* known, so
any drift is the scheme lying — and **the naive uniform-grid kernel leaks mass: up to ~9.7e-2
(≈10 %) at L4, 8.5e-3 at L6, 5.4e-4 at L8.** Two things to sit with. First, **the leak is
worst at COARSE levels** — i.e. exactly at early evolution, where the whole developmental
ladder starts. Second, **the error sloshes** (its sign flips with position); it is not a
clean bias that averages out. The finite-volume path conserves to ~1e-15 at every level and
placement.

The verdict on the code's own docstring claim (`sphere.rs:62`, "max/min area ratio ≈ 1.41,
versus ≈ 5.14 for the naive equidistant (gnomonic) grid"): **verified.** I measure 1.4099
and 5.1554 at N=256, both still converging upward. The equiangular choice was correct and
is not the problem. The problem is that *nothing downstream knows about the 1.41*.

---

## 2. The topological defect is irreducible — and HEALPix has **exactly the same one**

**Derived (Euler characteristic).** For any quadrilateral mesh on S²: `V − E + F = 2`, and
every quad has 4 edges each shared by 2 faces, so `E = 2F`, hence `V = F + 2`. The valence
sum is `Σ valence = 2E = 4F`. Therefore

> **Σ_v (4 − valence(v)) = 4V − 2E = 4(F+2) − 4F = 8.**

**Every quad pixelization of the sphere carries exactly 8 units of valence deficiency, at
every refinement level, forever.** The cube-sphere spends it as 8 valence-3 corners. At a
valence-3 vertex, each of the 3 meeting quads loses its diagonal neighbour across that
vertex → 3 cells with 7 neighbours per corner → **24 cells with 7 neighbours**, globally,
at every level.

**HEALPix spends its 8 units the same way.** Empirically verified with `astropy-healpix`
(`neighbours(..., order='nested')`, counting `-1` entries):

```
nside=  2 npix=    48  pixels with a missing neighbour: 24  (total missing links 24)
nside=  4 npix=   192  pixels with a missing neighbour: 24  (total missing links 24)
nside=  8 npix=   768  pixels with a missing neighbour: 24  (total missing links 24)
nside= 32 npix= 12288  pixels with a missing neighbour: 24  (total missing links 24)
```

**24 defective pixels. Identical to the cube-sphere. At every resolution.**

> **This is the single most consequential finding for the grid decision.** Switching to
> HEALPix does **not** buy a defect-free adjacency graph. It buys the *same* 24-cell
> defect, relocated, plus a re-pixelization off our topology, plus the loss of the
> `CellId` quadtree that the whole memoization design keys on. Any kernel written to
> handle HEALPix's 24 special cells could have handled the cube's 24 special cells.

---

## 3. Source by source

### 3.1 Górski et al. 2005 — HEALPix *(primary, read in full)*
`ref/research/pdfs/markdowns/astro-ph0409513/`

**What it says.** Three design requirements (§3): (1) hierarchical, (2) **equal area**,
(3) **iso-latitude**. 12 base curvilinear quadrilaterals, `N_pix = 12·N_side²`, each of
area `Ω = π/(3N_side²)` (§5). Two indexings: RING (for spherical-harmonic transforms) and
NESTED (a quadtree, "essential for the possible database applications"; §5.2).

**The sentence that decides the equal-area question** — §4, and again §5.3:

> *"The curvilinear quadrilateral pixels of this tessellation class **retain equal areas,
> but vary in shape** depending on their positions on the sphere."*
> *"…all pixels have the same surface area but slightly different shape…"*

**Directly usable:** the equal-area property makes `cell_area(level)` a *constant* — one of
the four metrics our kernels need becomes exactly true for free.

**Not usable / not bought:**
- **Requirement 3 (iso-latitude) is the whole point of HEALPix and is worthless to us.**
  It exists to make the associated Legendre recursions cheap (§6) — i.e. for fast spherical
  *harmonic* transforms. We do no spherical harmonics. We are paying HEALPix's entire
  structural cost for a benefit we will never collect.
- **Equal-area does not give us `cell_m`.** `cell_m` is a *length*; equal *area* with
  *varying shape* means neighbour centre-to-centre distances, edge lengths, and the D8
  diagonal **still vary**. Three of our four broken metrics stay broken.
- The paper gives **no neighbour-finding algorithm** — it defers to Wandelt, Hivon &
  Górski 1998 (§5.2). **No PDE content.** "Finite differencing for solving partial
  differential equations" appears once, as an item in a list of operations one might wish
  to perform (§3).

**Honest summary: HEALPix is a data-analysis pixelization for band-limited fields on the
sphere. It is not a numerical-methods grid, it does not claim to be, and no dynamical core
uses it.** *(That last clause is a negative claim — I searched and found none, but a
negative is hard to establish; treat as **likely**, not proven.)*

### 3.2 Westerteiger, Gerndt & Hamann 2011 — spherical terrain rendering on HEALPix *(primary, read in full)*
`ref/research/pdfs/markdowns/Spherical_Terrain_Rendering_using_the_hierarchical/`

**Directly usable — two things, both small:**
- §3.1: *"Due to the equal area property of HEALPix, the **grid resolution for a given
  subdivision level is constant everywhere on the sphere**."* This is the concrete payoff
  of equal-area, and it is exactly the property `sample::cell_size_m` *pretends* to have
  and does not. **But note precisely what it means**: constant *sample density* (cells per
  unit area), which equal-area does guarantee. They need it only to pick a tree depth. It
  is **not** a claim that neighbour spacing is uniform — and it isn't.
- §3: *"Neighboring tiles **overlap by one sample at their shared boundary**, which incurs
  a small storage overhead but allows for gapless C⁰ continuous rendering without needing
  to reference neighboring data."* A 1-cell replicated halo. This is the same idiom
  `spatial-key-bench.md` §5 already commits us to.

**Not usable, and the paper is honest about it:**
- §5, Future work: *"In the algorithm presented, **normal vectors at tile boundaries are
  discontinuous, resulting in small rendering artifacts.**"* **A HEALPix system, with
  equal-area cells, still has a tile seam.** Equal-area does not make seams go away — it
  was never going to, because the seam is a *coupling* problem, not a *projection* problem.
- The LOD scheme (§3.2.2, screen-space bounding-box area) says **nothing about
  differing-resolution neighbours**. There is no T-junction discussion, no crack-fill, no
  2:1 balance. **This paper does not answer the LOD question.** QuadSphere does (§3.5).
- Everything else is rendering (frustum culling, vertex shaders, imagery overlays,
  streaming). No physics, no fluxes, no conservation.

### 3.3 Youngren & Petty 2017 — Multi-Resolution HEALPix (MRH) *(primary, read)*
`ref/research/pdfs/markdowns/1-s2.0-S2405844017304966-main/`

**This source does not apply. Say it plainly.** MRH is a spatial *index* for **sparse point
data** — warhead fragments, weather stations, galaxy catalogues — optimized for disc /
polygon / latitude-strip / neighbour **range queries**. There is no field, no stencil, no
flux, no conservation, no PDE anywhere in it. Its contribution is memory (2–4 orders of
magnitude less) and query time (72 % faster) for sparse points. None of that is our problem.

Two salvageable facts, and one caution:
- **(usable, minor)** Table 1: the **HEALPix neighbour query is O(1)**, worst case, always
  — the only query type that is. Whatever else HEALPix costs, adjacency lookup is cheap.
- **(usable, minor)** MRH demonstrates that a **HEALPix base with different subdivision
  levels in different regions** is a working, published structure. Precedent that
  per-region levels on an equal-area base are fine — *for indexing*. It says nothing about
  whether the *physics* couples across those level boundaries, because MRH has no physics.
- **(caution)** Its Eq. (3) states `N_ring = 4(N_side − 1)`. Górski §5.1 (the primary) says
  the pixels lie on **`4·N_side − 1`** rings. These disagree. **Prefer the primary.** Treat
  this paper as a secondary throughout.

### 3.4 Berger & Oliger 1984 + **Berger & Colella 1989** — conservative coupling *(both primary)*

`multiscale-seams.md` §2.1 is right that **B&O 1984 does not contain refluxing** — it flags
it as forthcoming (*"stable boundary conditions have been derived which maintain the
conservation form of the difference scheme at the interface between the fine and coarse
grid. These will also be reported elsewhere."*, §5 intergrid communication). **I chased the
sequel and read it.** Berger & Colella 1989, *Local Adaptive Mesh Refinement for Shock
Hydrodynamics*, JCP 82:64–84, §3 (pp. 68–71). Here is what refluxing actually requires,
quoted:

Two distinct cases at a coarse/fine boundary:

- **Case (i) — the coarse cell is *covered* by a fine grid.** *"the coarse grid value at
  level l−1 is defined to be the **conservative average** of the fine grid values at level
  l that comprise the coarse cell. After every coarse integration step, the coarse grid
  value is simply replaced by this conservative average, and the value originally
  calculated using (1) is discarded."*
  → **This is what `pin_block_means` approximates** (it pins block means to the parent via
  a bilinearly-upsampled delta, rather than replacing outright — close enough in kind).

- **Case (ii) — the coarse cell *abuts* a fine grid but is not covered.** *"for the
  difference scheme to be conservative on this grid hierarchy, the **fluxes into the fine
  grid across a coarse/fine cell boundary must equal the flux out of the coarse cell**."*
  → **Vivarium does nothing here. This is the missing piece.**

The algorithm, verbatim in structure (their Eqs. 2–3 and the correction step):

1. Integrate the coarse grid normally, computing a **provisional coarse flux**.
2. Allocate a **flux register** `δF` on the coarse-grid edges that bound each fine grid,
   initialized to *minus* the provisional coarse flux.
3. After **each** of the `r` fine sub-steps, accumulate the fine fluxes on that edge:
   `δF_{i+1/2,j} := δF_{i+1/2,j} + (1/r) · Σ_{p=0}^{r-1} F^fine_{...,m+p}`
4. After all `r` fine steps, **correct the abutting coarse cell**:
   `U^coarse_{i+1,j} := U^coarse_{i+1,j} + (Δt_coarse / Δx_coarse) · δF_{i+1/2,j}`
   (with the opposite sign for the cell on the other side).

And the property that makes it composable — *"all fluxes calculated at a given edge and
level are **identical** (up to roundoff error) and are **independent of the particular grid
on which they are calculated**"*.

**Does it generalize to our tile boundaries and face seams?** *(derived, with one honest
caveat)* Yes, structurally — the derivation uses only the divergence theorem on a control
volume and never uses Cartesian-ness. A cube-sphere face seam is just a coarse/fine
interface with a refinement ratio of 1 and a coordinate rotation, and §3.6 shows an
operational core doing exactly that. **But it has a hard prerequisite we do not meet:**

> **Refluxing requires the kernel to be in FLUX FORM — to compute and expose a flux per
> edge, not merely to update a state.**

`water.rs` is already flux-form: `fl/fr/ft/fb` *are* per-edge fluxes, kept between steps.
**Water is ready for refluxing today.** `erosion.rs` is **not**: `incise()` writes
`h[i] = (h[i] + f·h[r])/(1+f)` — a state update with no edge flux; `deposit()` routes `qs`
down the D8 tree (flux-*like*, but along a tree, not across cell faces); `creep()` writes
`h[i] += k·∇²h` (trivially flux-formable as `∇·(κ∇h)`, but currently not). **Putting
erosion in flux form is a precondition of the seam fix, and it is most of the work.**

### 3.5 QuadSphere (`~/src-ext/QuadSphere`) — the LOD question *(primary, source read)*

**Directly usable — the 2:1 balance constraint.** `src/core/quad.ts`, class doc:

> *"if any neighbor is two or more levels of subdivisions different from this quad it
> **MUST** either subdivide or unify until within one level"*

and enforced in `subdivide()` (`quad.ts:567` — `if (this.level - neighbor.level > 0)
neighbor.subdivide(this)`). This is the graphics-world name for AMR's **proper nesting**
(Berger & Oliger §2: *"each point in a fine grid at level l must be in the interior … of a
grid at the next coarser level"*). Two independent literatures, same constraint. **Adopt
it**: it bounds the seam problem to a *single* level jump, which is what makes a flux
register tractable (one coarse edge ↔ exactly 2 fine edges, `r = 2`, known at compile time).

**Directly usable — T-junction handling, but only as geometry.** `quad.ts` `_active` /
`updateSides()` + `quad-utils.ts` `getLeftTriangleIndices(segments, activeSides)`: a coarse
quad whose neighbour has subdivided "activates" that side and re-triangulates it to
*include the neighbour's extra midpoint vertex*, so no crack appears. The ASCII art in the
`Quad` class doc enumerates all 16 active-side configurations. **This is a crack-free
*rendering* fix. It is not a flux fix, and it does not pretend to be** — matching vertex
positions makes the surface C⁰; it says nothing about mass crossing the boundary. **Both
halves are needed and they are different work**: QuadSphere gives us the geometry half,
Berger–Colella gives us the flux half.

**Explicitly NOT usable — the neighbour-finding.** `quad-registry.ts:78–148`
`getNeighbor(side, quad)` **scans every quad at the same level and compares edge vertex
lists with fuzzy float equality** (`_edgeMatches` → `V3.fuzzyEquals(v1, v2,
this._maxDifference / lvl)`, with `_maxDifference = (radius/(segments-2))/3` — a tuned
tolerance). Cross-face adjacency is handled by **hand-enumerated special cases** for
reversed edges ("special case for top of quadsphere", "…for bottom…", "…for back…", with
`.reverse()` sprinkled per case). This is O(N) per query, float-coincidence-based, and
carries no notion of cross-face *orientation* for data (only vertex positions).

**It is, however, a genuinely instructive negative result, and it is worth naming as such:
this is precisely what happens when you have no combinatorial adjacency rule.** It is also
a live warning for us — `sphere.rs::from_unit` already documents the *"edge-tie trap"*
found by the globe view: on a cube edge the dominant-axis comparison is a float tie. Both
are the same disease. **Cross-face adjacency must be integer/combinatorial, never
geometric.**

### 3.6 Putman & Lin 2007 — the source that actually answers the question *(primary, chased and read; not previously in our bibliography)*

W.M. Putman & S.-J. Lin, *Finite-volume transport on various cubed-sphere grids*, **J.
Comput. Phys. 227 (2007) 55–78**. This is the transport core underneath **FV3** (NOAA's
operational model, and NASA GEOS) — which `sphere.rs`'s own module doc already cites as the
validation for choosing a cube-sphere. **It is prior art for exactly our problem, on
exactly our grid, and we had not read it.**

**§3, p. 59 — the whole answer, in one sentence:**

> *"In our implementation, the cubed-sphere grid geometry is completely defined by the
> locations of the vertices (of the finite-volumes). **The cell areas (ΔA), grid lengths in
> the two cartesian directions (Δx, Δy), and a non-orthogonal metric factor (sin α)**, where
> α is the angle formed at the intersection of the coordinates connecting the finite-volume
> vertices, **are derived quantities.**"*

- Cell areas by **spherical excess** — Eq. (19): `ΔA = R²[α₁ + α₂ + α₃ + α₄ − 2π]`.
  *"The cell edges for all grids are prescribed to be great circle arcs and thus this
  formula for the cell area is exact."*
- Grid lengths by **great-circle distance** — Eqs. (20)–(23) (with the haversine form
  recommended for small angles, for rounding).
- The flux-form operator carries the non-orthogonality factor — Eqs. (7)/(13):
  `G(...) = (Δt/ΔA)·δy[ v_e · q(...) · Δx · **sin(α)** ]`, and *"the local metric factor due
  to grid non-orthogonality, sin(α) … **reduces to unity for orthogonal grids**."*

**§ Abstract — the finding that settles the equal-area question empirically:**

> *"slight deviations from orthogonality on the modified cubed-sphere (quasi-orthogonal)
> grids do not negatively impact the accuracy. In fact, **the more uniform version of the
> quasi-orthogonal cubed-sphere grids provided better overall accuracy than the most
> orthogonal (and therefore, much less uniform) conformal grid.** It is also shown that a
> simple non-orthogonal extension to the transport equation enables the use of the highly
> non-orthogonal and computationally more efficient gnomonic grid with acceptable accuracy."*

They compare **five** cubed-sphere grids (equidistant gnomonic, **equiangular gnomonic —
ours**, conformal, elliptic-solver, spring-dynamics). **They never use an equal-area grid at
all.** They keep the analytically-cheap, non-orthogonal grid and **fix the physics with
explicit metrics**, and get better results than the geometrically-purer alternative.

**§Appendix C, "Discontinuous treatment of the 12 face edges" — the cross-face seam
recipe:**

> *"There are 12 interfacing edges between the 6 faces on the cubed-sphere. Regardless of
> the projection methods … these edges where two locally continuous coordinate systems
> intersect are discontinuous, at least at the 8 corners of the cubed-sphere. … we shall
> **treat the 12 face edges as true discontinuities**. We only need to modify the subgrid
> reconstruction scheme for **the two cells nearest to the face edges**."*

The edge value is the average of two independent one-sided second-order extrapolations
(Eq. 47), and — the load-bearing clause — *"**The value computed as above is shared by the
two adjoining faces.**"* One shared edge value ⇒ what leaves one face enters the other ⇒
conservation. And §4: *"**For all schemes tested on all grids, mass is conserved to machine
precision.**"*

**Conservative finite-volume across cube-sphere face seams is a solved problem with a
published recipe, running operationally at NOAA and NASA.** It needs a 2-cell halo, a
one-sided reconstruction, and a shared edge value. It does **not** need a different
projection.

### 3.7 gospl / eSCAPE — our exact physics on an unstructured spherical mesh *(secondary; found by search, not read primary)*

Salles et al., **gospl: Global Scalable Paleo Landscape Evolution**, JOSS 5(56):2804 (2020)
(and its predecessor **eSCAPE**, Salles 2018). Global-scale landscape evolution — **stream
power incision + linear-diffusion hillslope creep, with multiple-flow-direction accumulation
— on an unstructured spherical mesh**, parallel, and used for published 100-Myr Earth
reconstructions (Salles et al., *Science* 2023; *Nature* 2023).

**This is the strongest existence proof for the fully-general alternative** (§4.1): our
exact physics, at global scale, on a sphere, with no regular grid at all — per-cell Voronoi
areas and per-edge lengths carried as mesh data. It proves the unstructured route *works*
for our physics; it says nothing about whether it is the right trade *for us*, and it gives
up the quadtree `CellId` that our entire memoization design keys on.

**Marked secondary and it should stay that way until someone reads the JOSS paper and the
source.** I did not read it primary; I am reporting what the search surfaced.
<https://joss.theoj.org/papers/10.21105/joss.02804> · <https://gospl.readthedocs.io/>

### 3.8 Snyder 1992 — assessed honestly *(secondary; primary not obtained)*

J.P. Snyder, *An Equal-Area Map Projection for Polyhedral Globes*, **Cartographica
29(1):10–21 (1992)**. A modified **Lambert azimuthal equal-area** projection onto polyhedral
faces. Exactly equal-area, by construction.

What I could establish, and could not:
- **(secondary, consistent across sources)** It is *most satisfactory* for the dodecahedron
  and the truncated icosahedron — for the truncated icosahedron, max angular deformation
  ≤ 3.75° and scale variation < 3.3 %. Its dominant real-world use is **ISEA** (Icosahedral
  Snyder Equal Area) in discrete global grid systems. **The cube is the polyhedron it suits
  *worst*** among the ones Snyder discusses, because face angular deformation scales with
  how far the face is from the sphere it covers, and a cube face covers 1/6 of the sphere.
- **(could-not-verify)** I could not obtain **cube-specific** distortion figures — max
  angular deformation, scale variation, or corner behaviour. The one open write-up I fetched
  (brsr.github.io) gives formulas but *"does not include quantitative distortion analysis."*
  **Anyone acting on Snyder must get these numbers first.**
- **(derived, and decisive regardless)** Snyder is **equal-area, therefore not conformal,
  therefore not orthogonal**. It buys `cell_area`. It does **not** buy `cell_m`, edge
  lengths, the D8 diagonal, or orthogonality — and by §2 it cannot remove the 8 corner
  defects, which are topological, not projective. It **trades shape for area**, and *shape*
  is what our stencils depend on.

> **Correction to the lead, offered plainly.** The lead's phrase — *"mathematically rigorous,
> **distortion-free** vertex distributions"* — is not achievable and is not what these
> schemes claim. HEALPix (§3.1) and Snyder both state that they hold area constant **and let
> shape vary**. There is no distortion-free tessellation of the sphere; Górski's own §2 says
> so outright: *"There is no known point set which achieves the analogue of uniform sampling
> in Euclidean space…"* The rigour is real; the distortion-freedom is not, and the distortion
> that survives is precisely the kind our kernels are sensitive to.

---

## 4. The three questions, answered

### 4.1 Adjacency — is Joseph's "adjacency as DATA" right?

**Yes about the metric. Not needed for the topology. And the honest cost is memory, not time.**

Nobody had measured this, so I did. Benchmark (`rustc -O -C target-cpu=native`, same style
as `spatial-key-bench.rs`; scratch, not committed), on the two loops the kernels actually
run:

| | 128² | 256² | 512² | 1024² |
|---|---|---|---|---|
| **A** 5-pt Laplacian, implicit idx *(status quo, `creep`)* | 884 | 980 | 981 | 982 Mcells/s |
| **B** 5-pt Laplacian, **adjacency-as-data** (u32 nbr table) | **1021** | **1019** | **997** | 981 Mcells/s |
| **C** 8-nbr MFD weights, implicit idx *(status quo, `accumulate_drainage`)* | 93 | 92 | 89 | 91 Mcells/s |
| **D** 8-nbr MFD weights, **full FV adjacency-as-data** (nbr + per-edge dist + per-edge length + per-cell area) | 83 | 82 | 80 | 82 Mcells/s |

**Time cost of adjacency-as-data: between −4 % and +11 %.** In the 5-point case it is *at
parity or faster* (the flat neighbour-table loop beats the branchy interior loop). In the
8-neighbour case it costs ~10 % — and note that loop runs at **89 Mcells/s vs 980 for the
Laplacian**, i.e. it is dominated by `powf`, not by addressing. **The prefetcher does not
care that the neighbour index came from a table, because the table is a dense array indexed
by cell.** A pointer-chasing linked list would be a different story — *don't build that.*

**Memory cost, per 512² patch:** field 1.05 MB · nbr8 (u32) **8.39 MB** · +per-edge length
and distance (f32) **16.78 MB** · +per-cell area 1.05 MB. **Carrying the full table costs
8–16× the field it decorates**, and `spatial-key-bench.md` puts the cache cliff at ~17 MB —
so a naive full table walks straight off it.

**Therefore, the honest structure — split adjacency into topology and metric:**

- **Topology stays implicit.** Within a tile, on one face, at one level, the 8-neighbour
  index arithmetic is *exactly correct* for every cell. The exceptions are: (a) the tile /
  face boundary — already solved by the **halo** the loader owns (`spatial-key-bench.md` §5;
  Westerteiger's 1-sample overlap; Putman & Lin's 2-cell edge treatment); and (b) **the 24
  corner cells, world-wide, at each level** (§2). Twenty-four. Not 6 · 4^L. Adjacency-as-data
  for *topology* is a 24-entry exception table, not a graph.
- **The metric must become data.** `ΔA`, `Δx`, `Δy`, the D8 diagonals, and `sin(α)` are what
  is *actually wrong today* (§1) — and they are all **closed-form analytic functions of
  `(face, u, v)`**. So they need not be *stored* per cell at all: compute them once per tile
  into a few f32 planes (or coarser + interpolate — the metric field is smooth and slowly
  varying), key them by `(face, level)` alone (they are **identical for every tile on a
  face**, and by symmetry across all six faces), and let the kernels read them. **This is
  Putman & Lin §3, verbatim: "cell areas, grid lengths, and a non-orthogonal metric factor …
  are derived quantities."**

**The strongest form of Joseph's intuition — a fully general unstructured/CA adjacency graph
— is not a straw man, and gospl (§3.7) proves it works for our exact physics.** Its real
merit is that it gives *one code path* instead of a fast path plus special cases, and it is
correct at corners, seams, and LOD boundaries **by construction**. What it costs us
specifically: the dense Cartesian tile interior that `spatial-key-bench.md` measured at
~6 Gcells/s; the `CellId` quadtree that memo keys, the store, Hilbert locality, and the
fated-noise KRNG all key on; and a new determinism surface (deterministic mesh generation).
**That is a lot to pay for 24 cells.** Both primaries that faced this exact choice —
Berger & Colella (independent structured subgrids, coupled by flux registers) and Putman &
Lin (six structured blocks, discontinuous treatment at the 12 edges) — landed on
**structured interiors + explicit metrics + an explicit interface**, independently. That
convergence is itself evidence.

### 4.2 Equal-area — necessary, or is explicit finite-volume enough?

**Not necessary. Explicit finite-volume is enough, and is strictly more general.**

The physics needs **four** metrics per cell: **area** (drainage area, mass, `∂/∂t` of any
conserved stock), **edge length** (flux face width), **neighbour distance** (slope), and
**the angle between the axes** (whether the stencil is even the right operator).

- **Equal-area gives you exactly one of the four — area — and gives you nothing on the other
  three**, because it holds area constant *by letting shape vary* (Górski §4, quoted in
  §3.1; and Snyder is an equal-area projection, so the same trade by definition).
- **Explicit finite-volume gives you all four**, on the grid we already have, with no
  re-pixelization, no loss of `CellId`, and no loss of the quadtree.
- **And the operational core that had this exact choice chose FV**: Putman & Lin compared
  five cubed-sphere grids and found the **more uniform, less orthogonal** grid *more accurate*
  than the more orthogonal one, because the metric factor `sin(α)` in the flux operator
  absorbs the non-orthogonality (§3.6).

**Recommended: do the finite-volume work now; defer the projection question, and scope it
down.** This is `grid_lab`'s decomposition, and the literature independently supports it:

- **Conservation is a *scheme* property.** Finite-volume fixes it **exactly, on any grid** —
  measured (~1e-15, `grid_lab`) and corroborated by a primary (*"for all schemes tested on
  all grids, mass is conserved to machine precision"*, Putman & Lin §4). **So the FV /
  explicit-metric work is worth doing regardless of which projection we end up on. It is
  never wasted.**
- **Isotropy / shape is a *grid* property.** No scheme fixes it. `grid_lab` measures diffusion
  spreading **1.16–1.25× anisotropically near a corner even with exact geometry, and it does
  not shrink with refinement**. Only a different projection touches this.

So the projection question does not block anything, and it collapses to a narrower, later
one: *how much shape distortion do we accept, and can the area term hoist to a constant for
a fast path?* **On present evidence the answer is still "keep the equiangular cube-sphere"** —
it is already the right choice among cube-spheres (1.41 vs 5.14, verified), it keeps `CellId`
and the quadtree, and equal-area cannot remove the corner defect that is the one thing a
projection *could* help with (§2). But that is now a **measurable** question with an
instrument pointed at it, not a conceptual one.

The one thing equal-area would genuinely buy — a constant `cell_area(level)` — we can have
for the cost of **one f32 plane per (face, level)**, and unlike equal-area it comes with the
*other three* metrics too.

### 4.3 Conservative coupling — what Berger–Colella requires, and whether it generalizes

Answered in §3.4. In brief: **it generalizes** (the derivation is the divergence theorem on
a control volume; nothing in it is Cartesian), and Putman & Lin's Appendix C is the
cube-sphere face-seam instance of it, conserving **to machine precision**. The prerequisite:
**flux form** — a per-edge flux the register can accumulate.

- `water.rs` — **ready.** `fl/fr/ft/fb` already *are* per-edge fluxes retained between steps.
- `erosion.rs` — **not ready.** `incise()` is a state update, not a flux; `deposit()` routes
  along a D8 tree, not across faces; `creep()` is flux-formable but is not in flux form.
  **This is the bulk of the seam work, and it was not previously named as such.**

### 4.4 LOD — differing-resolution neighbours

- **QuadSphere:** enforce **2:1 balance** (never let neighbours differ by more than one
  level; subdivide the neighbour if they would) and re-triangulate the coarse side's edge to
  meet the fine side's midpoint vertex. **Adopt the 2:1 balance — it is AMR proper-nesting
  under a different name, and it reduces the flux register to the fixed, compile-time case
  `r = 2`.** Take the T-junction re-triangulation as the *geometry* half only.
- **Westerteiger:** does not address it. Their tiles overlap by one sample, which is C⁰ for
  *equal-level* neighbours and says nothing about unequal ones.
- **Berger & Colella:** the *flux* half — a coarse edge abutting `r` fine edges accumulates
  `(1/r)·Σ` fine fluxes and corrects the coarse cell. With 2:1 balance, `r = 2`.

**The two halves are different work and both are required. `pin_block_means` is neither** —
it is Berger–Colella *case (i)* (conservative averaging of a *covered* cell), which is a real
and necessary operation, but it is not case (ii), and case (ii) is where the seam lives.
`multiscale-seams.md` §2.1 already says this ("Mean-pin ≈ injection, *not* refluxing"); this
pass confirms it against the primary and supplies the algorithm.

---

## 5. Recommended method, and its honest costs

**Keep the grid. Make the metric explicit. Put the kernels in flux form. Then couple.**

**M1 — a metric plane per `(face, level)`.** Compute, from `sphere.rs`'s own `to_unit`, by
spherical excess and great-circle distance (Putman & Lin Eqs. 19–23): per-cell `area_m2`,
per-edge `dx_m` / `dy_m`, the four D8 diagonal distances, and the non-orthogonality
`sin(α)`. These are analytic in `(u, v)` and *identical for every tile on a face at a given
level*, so they memoize as a single small artifact per level, not per tile. **This alone
removes the errors quantified in §1** (40–50 % on area and distance, over the majority of
every face) and is *independent of every other item here*.
  *Cost:* a few f32 planes; the kernels' `self.cell_m` becomes an indexed read (**measured:
  −4 % to +11 %**, §4.1). *Risk:* low. *This is the highest-value, lowest-risk item and it
  should go first.*

**M2 — a 24-cell corner exception table.** The only cells whose *topology* is irregular.
Give them 7 neighbours and a corrected stencil, or (cheaper and honest) mark them and refuse
to make physics claims there until they are handled. **They are irreducible (§2) — no grid
change removes them.**

**M3 — flux-form erosion.** Rewrite `creep` as `∇·(κ∇h)` with per-edge fluxes; give
`incise`/`deposit` an explicit per-edge sediment flux. **Prerequisite for M4, and the bulk
of the work.**

**M4 — halo + flux register.** Two-cell halo, loader-owned, with **integer/combinatorial**
cross-face orientation (never geometric matching — §3.5). Shared edge value at face seams
(Putman & Lin App. C). Flux register with 2:1 balance (Berger–Colella §3 + QuadSphere) at
tile/level seams. **This is what makes tiles compose.**

**M5 — retire the current `seam_ridge`, and rewrite the probe** (§0). It must be sited on
**land**, must assert on an **absolute** curvature scale (not a ratio against a possibly-zero
denominator), and must **fail loudly if the fluvial core did not fire** (e.g. assert
`max|Δh| > 0` after `erode`).

### Honest costs and what this does *not* fix

- **M1 changes numbers everywhere.** Every golden test, every pinned seed-0 artifact, every
  memoized tile is invalidated. That is *correct* (the content-addressed key exists for
  exactly this) but it is not free, and the ASSUMPTIONS ledger and nomos versions must move
  with it.
- **M3 is a real rewrite of the kernel we have the most invested in**, and it is the item
  most likely to be underestimated.
- **The corners are not *fixed* by any of this — they are made *honest*.** A 120° stencil is
  a genuinely different operator; M2 bounds the damage and makes it declarable, it does not
  eliminate it. **No scheme eliminates it (§2)** — and this is no longer just an argument:
  `grid_lab` measures the residual corner anisotropy at **1.16–1.25× *with exact geometry*,
  not shrinking with refinement.** Finite-volume buys exact conservation and *does not* buy
  isotropy. Say so in the nomotheke rather than hoping refinement washes it out.
- **`pin_block_means` is not deleted** — it is Berger–Colella case (i) and stays. It is just
  no longer expected to do case (ii)'s job.

---

## 6. What I could not resolve

1. **Snyder's cube-specific distortion numbers** (§3.8). Could not obtain the primary. If
   anyone acts on the Snyder lead, get them first — but §4.2's argument does not depend on
   them.
2. **Whether the corner stencil can be made second-order at 120°.** Putman & Lin say
   *"some modifications to the algorithm still need to be made to counter the
   non-orthogonality near the corners"* and do not, in this paper, say what they are.
   Open.
3. **Whether `worldview`'s 0.05 m/epoch uplift actually clears the 287 m sea-level deficit
   at its default focus** (§0.2) — i.e. whether the *testbench* has been showing us real
   fluvial erosion, or the same no-op. **This should be checked before anything else in this
   doc**, because it determines whether the erosion kernel has *ever* been validated on land.
4. **gospl / eSCAPE, primary-unread** (§3.7). Someone should read the JOSS paper and the
   source. If the unstructured route is ever seriously considered, that is where the honest
   comparison lives.
5. **The `z`-reconciliation question** (`multiscale-seams.md` §3, flagged there as our
   coinage) is untouched by this pass. The flux register handles *rates*; whether a `z=1`
   water flux read by a `z=2` consumer is well-posed at a seam remains open, and none of the
   sources read here address it.

---

## 7. Handoffs

### 7.1 To `grid_lab` — the literature it asked for

`grid_lab`'s header states the blocker honestly: *"Snyder and HEALPix plug in behind `Grid`
once their formulae are sourced from the literature — implementing them from memory would
fabricate the very numbers this exists to measure."* That is exactly right, and this pass
resolves it — **asymmetrically**:

- **HEALPix: sourced. Plug it in.** Górski et al. §5.1 gives the closed-form pixel centres
  (Eqs. 2–9: polar cap and equatorial belt, `z` and `φ` from ring index `i` and in-ring
  index `j`); §5.2 gives the NESTED↔`(x,y)` bit-interleave (Eqs. 10–18); §5.3 gives the
  **pixel boundaries** in closed form (`cos θ = a + b·φ` in the equatorial zone,
  `cos θ = a + b/φ²` in the polar caps; Eqs. 19–22) — which is what a `Grid` impl needs to
  compute true edge lengths and centre distances. Area is the constant `Ω = π/(3N_side²)`.
  **Everything the trait needs is in the paper we already have** (`ref/research/pdfs/markdowns/astro-ph0409513/`).
  Expected result, predicted here so it can be falsified: **area ratio 1.000; corner
  anisotropy still non-zero and non-converging, at 24 defective cells (§2).**
- **Snyder: NOT sourced. Do not plug it in yet.** I could not obtain cube-specific distortion
  figures from any source (§3.8), and the primary (Cartographica 29(1)) was not obtainable in
  this pass. **Implementing it from the general Lambert-azimuthal recipe would be exactly the
  fabrication `grid_lab` refuses.** Leave the slot empty and say why.

`grid_lab`'s other stated scope gaps are the seam proper, and this doc supplies the method
for them: **cross-face adjacency** → integer/combinatorial, never geometric (§3.5), with the
shared-edge-value treatment of Putman & Lin App. C (§3.6); **the 8 valence-3 corners** →
irreducible, 24 cells, and the same 24 in HEALPix (§2); **stream-power and shallow-water need
their own rows** → agreed, and §1 gives the terms each leans on (stream power: area^m and
distance; shallow water: edge length and area).

### 7.2 Bibliography deltas

Two items to register (relata is under construction — `BIBLIOGRAPHY.md` says defer, so
these are logged here as pending):

- **`berger-1989-local`** — M.J. Berger & P. Colella, *Local Adaptive Mesh Refinement for
  Shock Hydrodynamics*, J. Comput. Phys. **82**(1):64–84 (1989). **The refluxing algorithm.**
  `multiscale-seams.md` §2.1 already credits it correctly but had not read it; §3.4 above now
  quotes it.
- **`putman-2007-finite`** — W.M. Putman & S.-J. Lin, *Finite-volume transport on various
  cubed-sphere grids*, J. Comput. Phys. **227**(1):55–78 (2007). **The direct prior art for
  flux-conservative finite-volume on a cubed sphere**, including the equiangular grid we use
  and the 12-face-edge seam treatment. It is the paper behind FV3, which `sphere.rs`'s module
  doc already cites as the reason we chose a cube-sphere — we cited the model and not the
  method.

To chase (secondary only, §3.7): **`salles-2020-gospl`** — JOSS 5(56):2804.
