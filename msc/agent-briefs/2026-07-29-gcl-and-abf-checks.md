# GCL and ABF — resuming the two unrun checks from the structure-preservation survey

*Spike landing note, 2026-07-29, base `8861798`. Agent: opus, structure strand.
Sources: `msc/research-structure-preserving/README.md` §4.8, §4.9, §9 (P3 + "Resolve
before B1" 1–2), `ref/research/grid-comparison-report.md` §5.2,
`core/src/detail-structure-scheme-map.md` WN rows 4–5, `core/src/form-face-flux-register.md` WN row 4.
Primary re-read for this note: `arnold-2005-quadhdiv` PDF (relata), full §1 + §8.*

**Nothing here is decided.** Everything is `:by claude :status proposed`. Segment text is
drafted here for the orchestrator to land; DECISIONS entries are proposed, not appended.

---

## 0. Pre-registration (written before the first run — `#norm-probe-sensitivity`)

Recorded so a later reader can see which of these the measurement overturned.

| # | probe | prediction, written first | what would refute it |
|---|---|---|---|
| **A1** | Non-affineness δ/h of a cube-sphere cell vs level (δ = the bilinear cross-term $\lvert c_{00}-c_{10}+c_{11}-c_{01}\rvert$) | **decays ~O(h), halving each level** | δ/h flat under refinement ⇒ ABF's hypothesis holds on our mesh sequence and the trap is real for us |
| **A2** | Same δ/h on ABF's own trapezoid mesh (α=1, β=1/3) | **flat, level-independent** — that is the construction | decaying ⇒ I have mis-built their mesh |
| **A3** | Two-point-flux Laplacian error on a *uniform sheared parallelogram* lattice (affine ⇒ ABF-exempt by construction, δ ≡ 0) | **does not converge** — the measured defect present where ABF is provably absent | convergence ⇒ the parallelogram separator fails and the correspondence survives |
| **G1** | Spherical-polygon closure identity $\sum_e L_e \hat p_e + 2\!\int_K \hat r\, d\Omega = 0$ on our cells, with $L_e,\hat p_e$ from great-circle edges and the integral by independent quadrature | **holds to ~1e-15** — cube-sphere cell edges *are* great-circle arcs, so this should be a theorem, not luck | failure ⇒ either the edges are not great circles or `cell_solid_angle` is inconsistent with them |
| **G2** | Free-stream test: discrete FV divergence of a solid-body rotation (exactly divergence-free) — (a) exact edge integrals, (b) midpoint rule + true metrics, (c) the uniform-Cartesian set `water.rs` actually uses ($L=d=\texttt{cell\_m}$, $A=\texttt{cell\_m}^2$) | (a) **~1e-16 identically**; (b) **O(h²)**; (c) **O(1), not decaying** | (c) decaying ⇒ the uniform set is better than the area/length measurements suggest |

**A4 — added after the A1–G2 run, pre-registered before its own first run.** A3 came back
*saturating* (order → 0, error plateauing at ~46%), not *growing*. The grid report measured
order **−0.50** on the cube-sphere — error that grows. A uniform lattice has no metric
*variation*, so the growth must come from somewhere A3 cannot see. A4 repeats A3 on a
smoothly **varying-shear** planar lattice (still asymptotically affine, so still ABF-exempt).

| # | probe | prediction, written first |
|---|---|---|
| **A4** | two-point flux Laplacian on a smoothly varying-shear periodic lattice | **honestly uncertain.** My first instinct is that it should still *saturate* — the scheme converges to a wrong *variable*-coefficient operator, which is still an O(1) error, not a growing one. If that is right, the grid report's stated mechanism ("the discarded tangential term scales as $\sin\theta\cdot R/h$, and $R/h$ doubles every refinement") does not by itself explain a *growing* error and the −0.50 has a source none of these probes has isolated. If instead A4 grows, metric variation is the mechanism and the report's account is right in substance. |

**A5 — added after A4, pre-registered before its own first run.** A4 came back *saturating*
too (order → 0 at ~45% error), and its lattice is exactly parallelogram (δ/h at round-off),
so metric variation alone does not produce growth either. Both controls say the two-point
flux converges to **the wrong operator** — an O(1) plateau, not an unbounded error. Meanwhile
`grid_lab` computes the −0.50 from **exactly one pair**, N=32 → N=64 (`main.rs` §7 `pairs`),
which is the coarse regime where both controls show their *largest negative* orders (−0.17,
−0.24) purely from approaching the plateau from below.

| # | probe | prediction, written first |
|---|---|---|
| **A5** | TPFA relative L2 error vs an exact degree-2 spherical harmonic on our own equiangular cube-sphere, **face interior only** (no cross-face adjacency needed; the grid report already refuted "the 24 defects drag the norm"), N = 32 … 1024 | **the −0.50 is pre-asymptotic.** N=32→64 should roughly reproduce it; the order should then climb toward **0** as the error plateaus at O(1). If the order instead stays near −0.5 out to N=1024, the error genuinely grows and this hypothesis is refuted — which would make the sphere's behaviour qualitatively different from both planar controls and would be the more interesting result. |

**Honest prior, separated by register** (the correction the base-level spike asked for):
*measured* = the −0.5 two-point order and the corrected/wide +1.63 (grid report §5.2);
*believed* = that a smooth mapped mesh is asymptotically affine (I have the mechanism, not the number);
*unknown* = everything under G.

**Sensitivity, stated up front.** G1 is a *geometric identity* probe: it can catch an
inconsistent metric set but it cannot catch a metric set that is consistently wrong
(e.g. every length scaled by a common factor — the identity is homogeneous). G2(c) is the
known-bad arm required by `#norm-probe-sensitivity` FE(2): if arm (c) passes, the probe
has no discrimination and nothing here should be believed.

---

## 1. Verdict on the ABF ⇄ order-−0.5 correspondence: **REFUTED**

*Survey §4.8 and §9's "Resolve before B1" #1. The survey called this "the single
highest-value thing in this document to check, and it is checkable on paper." It was
checkable on paper, and the answer is no.*

### 1.1 What ABF actually proves (read primary, `arnold-2005-quadhdiv` §1 and §8)

RT$_r$ spaces on quadrilaterals are built by Piola-transporting a fixed reference space
$\hat V$ off the unit square through a **bilinear** map $F$. For $r=0$ the reference
divergence $\widehat{\operatorname{div}}\hat u$ is a constant, and the Piola transform
carries divergence as $\operatorname{div} u = J_F^{-1}\widehat{\operatorname{div}}\hat u$.
So on each element the *entire achievable divergence space is* $\operatorname{span}\{1/J_F\}$.
On a **parallelogram** $J_F$ is constant, that span is the constants, and everything is
classical. On a **non-affine** quad $J_F$ varies across the element, $1/J_F$ has an
O(1)-shaped wobble the space cannot cancel, and:

> **[P]** *"…while the Raviart–Thomas space of index r achieves order r+1 approximation in
> L² for quadrilateral meshes as for rectangular meshes, **the order of approximation of
> the divergence is only of order r** in the quadrilateral case… Thus, in the case r = 0,
> there is no convergence in H(div, Ω)."* (§1)

**Three facts from the primary that the survey's summary did not carry, and each of them
matters:**

**(i) The non-convergence is a property of a mesh sequence whose non-affineness never
shrinks.** ABF's counterexample is explicit about this:

> **[P]** *"The example is far from pathological. The domain is simply a square, the mesh
> sequence does not degenerate in any sense—in fact **all the elements of all the meshes in
> the sequence are similar to a single right trapezoid**…"* (§1)

Read as a hypothesis rather than as reassurance, *"similar to a single right trapezoid"* is
the load-bearing clause. The trapezoid's shape is fixed as $h\to0$ (their §3 construction
fixes $\alpha=1$, $\beta=\alpha/(1+2\alpha)=1/3$ for every $h$), so the deviation-from-affine
is **h-independent**. That is exactly the hypothesis a smooth mapped mesh violates.

**(ii) On the mixed method — the FE method structurally closest to FV — the defect does not
touch the primary variables at all.**

> **[P]** *"…we show that **despite the lower order of approximation of the divergence by
> Raviart–Thomas quadrilateral elements, the mixed method approximation of the scalar and
> vector variable retain optimal order convergence orders in L²**. By contrast, error
> estimates for the least squares method indicate a possible loss of convergence for both
> the scalar and vector variable."* (§1)

Confirmed by their own numerics (§8, Table 2): *"on a trapezoidal mesh, RT₀ gives a first
order approximation to the scalar and vector variable (the same as on a rectangular mesh),
but there is no convergence of the approximation of the divergence of the vector variable."*

**The ABF defect is a defect of the divergence's *pointwise representation inside a cell*,
given fluxes that are already right.** It degrades a least-squares functional, which reads
that representation; it leaves the mixed solution's rates alone.

**(iii) It sits strictly downstream of where our measured defect lives.** ABF's whole
argument begins *after* the normal-flux degrees of freedom are set — the canonical
interpolant matches face-flux moments exactly, and the Piola transform is chosen precisely
because it preserves $\int_e u\!\cdot\!n$. The grid report's defect is a defect **in that
flux number**: $(u_j-u_i)L/d$ measures $\nabla u$ along the centre-line while the flux needs
it along the edge normal (report §5.2, mechanism [D]). The two statements are about
different arrows in the same diagram.

### 1.2 Three separations, any one of which is fatal to the correspondence

**(S1) Different objects.** ABF: (exact fluxes) → (pointwise div field). Measured defect:
(cell values) → (flux). Composition order settles it — you can have either without the other.

**(S2) The parallelogram separator — the clean paper argument.** Take a uniform lattice of
congruent, non-rectangular parallelograms. The map is **affine**, $J_F$ is constant, and by
ABF's own dichotomy RT₀ recovers full order in $H(\mathrm{div})$: **the ABF defect is
provably absent.** But the lattice is non-orthogonal, so the centre-line and the edge normal
differ by a fixed angle and the two-point flux is inconsistent: **the measured defect is
present.** One mesh, defect present, theorem absent. (A3 measures this rather than asserting it.)

**(S3) Opposite h-trends on *our* mesh sequence.** A cube-sphere quadtree cell is the image
of a **smooth** map, so on a cell of size $h$ the map is affine + O($h^2$) — the cell is a
parallelogram perturbed by O($h$) *relatively*, and the perturbation **shrinks under
refinement**. ABF's hypothesis (i) fails for us. Their O(1) divergence defect therefore
decays on our meshes, while the measured two-point defect **grows** (order −0.50). A theorem
whose error decays cannot be the mechanism behind a measurement whose error grows.
(A1/A2 measure this rather than asserting it.)

### 1.3 The second question, and why answering it *yes* is smaller news than the survey hoped

Survey §9 "Resolve before B1" #2 asks whether Bochev–Ridzal's *"natural mimetic divergence
operator"* means an FV divergence is already the rehabilitated one. **Yes** — and the
mechanism is now visible from §1.1: BR's rehabilitation replaces the RT₀ divergence
$c_K/J_F$ with the cell-constant $c_K/\lvert K\rvert$, which is exactly
$(\sum_e F_e)/A_K$, the finite-volume divergence. An FV scheme has *never had the
representation the theorem indicts*; it has no $1/J$ shape factor to be wrong about.

But the survey filed this under *"if it holds it is very good news."* **It is not good news;
it is no news.** It removes an obstacle that §1.2 shows was never on our path. The value of
the check is the opposite of what was expected: it does not make B1 cheaper, it **stops a
false attribution** — and the false attribution was the dangerous half, because "our
measurement has a 2005 theorem behind it, and the theorem says it is not a bug in our
harness" (survey §4.8) would have retired a live defect by citation.

> ### The measured order −0.50 is ours. It has no theorem behind it, it is not exonerated by Bochev–Ridzal, and its fix is the one already measured: corrected normal projection with a **wide** (8-neighbour quadratic) gradient — 2523×, order +1.63, grid report §5.2.

### 1.4 What ABF *does* still say to us — the residue, kept because it is real

- **The FEEC/B1 route is safer than the survey feared, for reason (S3).** Cotter & Thuburn's
  recommended target (`RT0` on quads on a cubed sphere, primal-dual) sits on a smooth mapped
  mesh, so the ABF defect decays there rather than saturating. The survey read this
  recommendation as sitting over a trap; the trap is shallow on smooth meshes and the depth
  is measurable (A1).
- **It is *not* shallow wherever cell shape is held fixed under refinement.** That is a real
  design constraint with a vivarium address: any grid-optimisation pass (survey §4.6) or any
  future non-smooth mapping that fixes a cell's shape across levels re-arms the hypothesis.
  A quadtree over a smooth map is exempt *because* the map is smooth — the exemption is a
  property of the map, not of the quadtree.
- **The least-squares warning transfers.** Any future scheme that puts the divergence
  *inside a minimised functional* (least-squares FV, some optimisation-based reconstructions)
  inherits ABF's degradation rather than the mixed method's immunity.

---

## 2. The discrete GCL on our metrics

*Survey §4.9 / P3. Status going in: **unknown**, and the survey was right that it was
never checked. What the check found first is that the question needed re-asking before it
could be answered.*

### 2.1 The finding that reframes the probe: there is no metric set to test yet

`crates/vivarium-world/src/measure.rs` carries **areas** (`cell_solid_angle` → `cell_area_m2`,
exact closed form), **cell centres** (`cell_center_unit`), and **centre-to-centre great-circle
distances** (`gc_dist_m`, `neighbor_center_dist_m`). It carries **no edge length, no edge
normal, and no centre-to-mid-edge arm** — `#form-grid-equiangular-staggered` Epistemic Status
already says so ("edge lengths, centre-to-mid-edge arms and non-orthogonality … are **not yet
built**"), and this probe confirms it from the consumer side.

Downstream, the split is sharper than the segment currently states:

- **`erosion.rs`** uses true areas + true centre distances, but its router is an MFD fan —
  a graph weighting, **not** an FV flux — so it has no edge normals to be inconsistent about.
- **`water.rs`** is the flux-form kernel, and it carries a **single scalar `cell_m`** for
  every length, area and flux width (`cell_m`, `cell_m²`) — a **uniform-Cartesian metric set
  on a sphere**.

So the sharp statement of the GCL status, replacing "unknown":

> **The discrete GCL is not violated by our staggered-FV metric set. There is no staggered-FV
> metric set.** The flux-form kernel runs on a uniform-Cartesian stand-in that fails the
> identity by construction and by O(1); the true-metric code path exists but stops short of
> the edge quantities a GCL is *about*.

That is a better answer than a number, and it retires "unknown" honestly. It also converts
P3 from a diagnostic into a **specification** — §2.3.

### 2.2 The identity itself, derived — the part that is now a theorem rather than a probe

Two facts make the spherical GCL exactly checkable in closed form, and both are ours:

**(a) Our cell edges are exactly great-circle arcs.** The gnomonic projection carries great
circles to straight lines; a constant-$u$ (or constant-$v$) line on a cube face is straight in
the face's tangent plane; therefore each cube-sphere cell is a genuine spherical quadrilateral
with great-circle sides. (The equiangular $\tan$ warp moves *where* the lines fall, not what
they are.)

**(b) The outward in-surface normal along a great-circle arc is constant** — it is the arc's
pole $\hat p_e$. So $\oint_{\partial K}\hat n\,ds = \sum_e L_e\,\hat p_e$ exactly, with no
quadrature.

Applying the surface divergence theorem to the tangential part of an ambient constant vector
$c$ (for which $\nabla_S\!\cdot v = -2(c\cdot\hat r)/R$, and on each arc $v\cdot\hat p_e = c\cdot\hat p_e$
exactly because $\hat r\perp\hat p_e$ there) gives, on the unit sphere:

$$\boxed{\;\sum_e L_e\,\hat p_e \;+\; 2\!\int_K \hat r\,d\Omega \;=\; 0\;}$$

**This is the discrete metric identity for a surface FV scheme on the sphere** — the
spherical replacement for the planar closure condition $\sum_e L_e \hat n_e = 0$, with the
curvature source that the plane does not have. Sanity check on the hemisphere: $\sum_e L_e\hat p_e
= 2\pi(-\hat z)$, $2\int \hat r\,d\Omega = 2\pi\hat z$; sum zero. ✓

And the free-stream companion: solid-body rotation $v=\omega\times r$ is exactly
divergence-free on $S^2$, and its exact edge integral telescopes,
$\int_e v\cdot\hat p_e\,ds = \omega\cdot(\hat r_{\text{end}}-\hat r_{\text{start}})$, so the
loop sum is **identically** zero. That half of the GCL is *combinatorial* — the same carve
as the survey's finding 3 (topological identities are free; the metric is where it costs).
**What can fail is not the identity; it is the quadrature-and-metric set that stands in for it.**

### 2.3 What P3 becomes: a specification for `measure.rs`

The identity above is a **build-time acceptance test** for the four edge quantities
`#form-grid-equiangular-staggered` names as unbuilt. Proposed (`src/` need, per the brief's
convention — recorded here, not landed in the tree):

- `edge_pole(face,i,j,level,dir) -> [f64;3]` — the great-circle pole of a cell edge (the exact
  outward in-surface normal, constant along the edge);
- `edge_len_m(...)` — great-circle arc length of the edge (the wavelet-store spike's
  `arc_len_m` / `east_edge_len_m` / `north_edge_len_m`, still unpromoted);
- acceptance: the closure identity above, to ~1e-15 relative, per cell, at several levels and
  positions including the 24 defect cells — this is the discrete GCL, and it is checkable
  *before* any scheme rides on it.

That is P3 discharged in the only form available today: the probe exists, the identity is
derived and exact, and the thing it tests is specified.

### 2.4 The covariant escape (Montoya et al.), priced honestly

The survey's inference — our closed-form Jacobian makes the covariant route cheap — is
**not yet priced**, and this spike did not price it. What can be said with the derivation in
§2.2 in hand is narrower and, I think, more useful: **the flux-form route's GCL requirement is
not expensive for us either**, because our edges are great-circle arcs with constant normals
and our areas are closed-form. The fork the survey drew (satisfy the GCL *or* go covariant)
is real, but the left branch is cheaper than "must approximate the metric terms so as to
enforce discrete metric identities" makes it sound — we are not approximating them.
**Inference, not measurement**; the honest form is: the covariant route's advantage over the
flux-form route *on this specific axis* is smaller for us than for a general curvilinear mesh,
and neither branch is priced.

---

## 3. Probe

`crates/vivarium-world/examples/gcl_abf_probe.rs` (examples are outside the source digest, so
this does not invalidate the shared store). Run:

```
cargo run --release -p vivarium-world --example gcl_abf_probe
```

Results table lands in §4 after the run, against the §0 pre-registration.

---

## 4. Measured results

Two binaries, both in `examples/` (outside the source digest — the shared world store is
untouched). `tpfa_ladder` re-uses `grid_lab`'s own mesh builders and probes by `#[path]`
module inclusion, so the shared instrument was **read, not edited**.

```
cargo run --release -p vivarium-world --example gcl_abf_probe
cargo run --release -p vivarium-world --example tpfa_ladder
```

### 4.1 Scorecard against §0

| # | prediction | outcome |
|---|---|---|
| A1 | δ/h decays O(h) on the cube-sphere | **confirmed** — max ratio 1.77 → 1.998, mean 1.93 → 2.000 across levels 2–10 |
| A2 | δ/h flat on ABF's trapezoid mesh | **confirmed** — 6.346410e-1 at every rung, ratio 1.000 |
| A3 | two-point flux fails on a parallelogram lattice | **confirmed** — 46% relative error, order → 0; orthogonal control 2.000 |
| G1 | closure identity holds ~1e-15 | **confirmed** — relative residual 3e-17 … 7e-15, levels 3–9, centre/edge/corner |
| G2 | (a) round-off, (b) O(h²), (c) O(1) non-decaying | **confirmed** — (a) 6e-16 flat, (b) order 2.00 → 1.83, (c) 0.15 → 0.23, order ≈ 0 |
| A4 | *uncertain* — saturate or grow? | **saturates**, as my first instinct said — 45%, order → 0, on an exactly-parallelogram lattice |
| **A5** | the −0.50 is pre-asymptotic; order climbs to 0 | ⚠ **REFUTED on the real instrument.** On `grid_lab`'s own whole-sphere mesh the order is **−0.521, −0.501, −0.498, −0.498** out to N=256, error 0.45 → 1.83. It genuinely grows. |

**A5 is the honest miss, and chasing it produced the most valuable finding here.** My
face-interior version of A5 *did* plateau (0.176 → 0.192, order → −0.004), which is why the
contradiction with `grid_lab` was worth resolving rather than filing.

### 4.2 ⚖ The finding A5's failure produced: the −0.50 is a **cube-panel-seam** defect

`tpfa_ladder` splits the same whole-sphere L2 by distance from the nearest panel seam
(contributions to the *global* relative norm, so they add in quadrature):

| N/face | band 0 (touching a seam) | band 1–2 | band 3–8 | band 9–32 | band >32 | global |
|---:|---:|---:|---:|---:|---:|---:|
| 32 | **6.278e-1** | 8.33e-2 | 1.36e-1 | 5.10e-2 | 0 | 6.497e-1 |
| 64 | **9.018e-1** | 6.48e-2 | 1.21e-1 | 1.16e-1 | 0 | 9.196e-1 |
| 128 | **1.285e0** | 4.80e-2 | 8.89e-2 | 1.42e-1 | 6.66e-2 | 1.299e0 |
| 256 | **1.825e0** | 3.48e-2 | 6.69e-2 | 1.20e-1 | 1.26e-1 | 1.834e0 |
| **order** | **−0.506** | **+0.466** | **+0.411** | +0.238 | — | −0.498 |

**[M]** Three things read straight off it:

1. **One ring of cells carries the whole defect.** Band 0 is ~99.5% of the global norm in
   quadrature at every rung.
2. **The bulk of the face CONVERGES under the same scheme** — every interior band has a
   *positive* order (+0.47, +0.41), and A5's interior-only run plateaus at 19%. The
   two-point flux in the face interior is a bounded wrong-operator error, not a divergent one.
3. **The local seam error is O(1/h).** Band 0's area fraction is ~2/N, so the local relative
   error is contribution/√(2/N) = 2.5, 5.1, 10.3, 20.6 — **doubling every rung**. And an
   O(1/h) error on a codimension-1 set contributes √h·(1/h) = h^{−1/2} to a 2-D L2 norm:
   **that is where the clean −0.5 comes from, arithmetically.**

**And it is not bad edge geometry.** `seam_edge_metrics` prints what the flux actually
consumes, worst seam edge vs an interior edge:

| N | where | edge_len/h | dist/h | nonortho° | skew |
|---:|---|---:|---:|---:|---:|
| 256 | seam | 1.023 | 1.020 | **29.70** | 0.0005 |
| 256 | interior | 1.011 | 1.013 | 11.30 | 0.0001 |

Lengths, distances and skew at the seam are within ~2% of the interior and **converge to
constants**. The only outlier is non-orthogonality, 29.7° vs 11.3° — and that is an **O(1)**
quantity, which by itself cannot make a 1/h error. **[M]**

**[me] The remaining candidate mechanism, offered as inference, not result:** the seam is
where the metric *jumps*. In the interior the dropped tangential term varies smoothly, so
neighbouring flux errors largely cancel in the divergence and leave the bounded
wrong-operator error A3/A4/A5 all measured. Across a panel seam the term jumps
discontinuously, the cancellation fails, and the divergence picks up
(Δ flux-error)/h = O(1/h). That is testable — freeze the metric at its seam value in a band
either side and the growth should vanish — and it is not tested here.

### 4.3 What this changes, stated plainly

- **The grid report's §5.2 mechanism is right in kind and wrong in location.** *"The
  discarded tangential term scales as sin θ · R/h, and R/h doubles every time you refine"* is
  a **[D]** in that report, not an **[M]**, and three controls here (A3, A4, A5-interior) each
  fail to reproduce *growth* from it — they all saturate. What grows is the seam.
- **The report's own refutation of "the 24 defects are dragging the L2 norm down" stands and
  is not in tension with this.** It split by distance from the **8 corner defects** (24 cells).
  The 12 panel **edges** are a far larger set — ~12N cells — and were not the split tested.
- **The measured 2523× gain from the corrected/wide scheme is, on this evidence, mostly a
  seam fix.** LSQ's own order also drifts down (1.71 → 1.54 across the ladder), so the
  corrected scheme is not clean at the seam either — just vastly better.
- **This is a third, independent reason the seam is broken**, and unlike the two already on
  `#form-face-flux-register` it needs **no hanging node and no refinement mismatch**: it is
  present on a *conforming, same-level* panel seam.

---

## 5. Proposed DECISIONS entries and segment edits

All `:by claude :status proposed`. Segment text drafted, not landed — the orchestrator lands
canon.

### 5.1 Proposed DECISIONS entries

**`abf-quadrilateral-trap-does-not-apply-to-us`** — The Arnold–Boffi–Falk 2005 result that
RT₀ has no convergence of the divergence on non-affine quadrilaterals is **not** the
mechanism behind the grid report's measured two-point-flux inconsistency, and does not
indict a finite-volume scheme on a cube-sphere quadtree. Three separations, one read-primary
and two measured: ABF's statement is about representing the divergence *given exact fluxes*
while ours is about the flux itself; the defect is present on a parallelogram lattice where
ABF is provably absent (measured, 46% error, order 0); and ABF's non-convergence requires a
mesh sequence of fixed cell shape (measured flat δ/h at 6.35e-1) whereas a cube-sphere
quadtree is asymptotically affine (measured δ/h → 0 at order 1). Bochev–Ridzal's
rehabilitation does exempt an FV divergence — the natural mimetic divergence *is* the FV
divergence — but the exemption is **no news, not good news**: it removes an obstacle that was
never on our path. Retires survey §9 "Resolve before B1" items 1 and 2, and survey §10's
first `[⊘]`.

**`the-discrete-gcl-is-a-spec-not-a-defect-we-have-no-metric-set-to-violate-it`** — The
discrete geometric conservation law is not violated by our staggered-FV metric set because
there is no staggered-FV metric set: `measure.rs` carries areas, centres and centre-to-centre
distances and **no edge length, edge normal, or arm**. The flux-form kernel (`water.rs`) runs
on a single scalar `cell_m` for every length and area — a uniform-Cartesian set that fails
free-stream preservation by O(1) and does not decay under refinement (measured). The identity
itself is derived and exact for our grid: cube-sphere cell edges are great-circle arcs, whose
outward in-surface normal is the arc's constant pole, so
Σ_e L_e p̂_e + 2∫_K r̂ dΩ = 0 — verified to ~1e-15 against independent quadrature. Its
free-stream half is combinatorial (exact edge integrals telescope identically), so what can
fail is the quadrature-and-metric stand-in, not the identity. P3 therefore becomes a
**build-time acceptance test** for the four unbuilt edge quantities rather than a diagnostic
of existing ones. Retires survey §10's second `[⊘]`.

**`the-two-point-flux-defect-is-a-panel-seam-defect`** — On our equiangular cube-sphere the
measured order −0.50 of the two-point flux is carried ~99.5% by the single ring of cells
touching a cube-panel seam, where the local relative error is O(1/h) and doubles per
refinement; every interior band converges at ~+0.45 order and the face interior alone
plateaus at 19%. The seam edges' lengths, centre distances and skew are within 2% of interior
values and converge; only non-orthogonality is elevated (29.7° vs 11.3°), and that is an O(1)
quantity. The clean −0.5 is arithmetic: an O(1/h) error on a codimension-1 set contributes
h^{−1/2} to a 2-D L2 norm. **Consequence:** any scheme that keeps a two-point flux across a
panel seam has a *divergent* operator there regardless of its bulk behaviour, and this is
independent of hanging nodes — it is present on conforming, same-level seams. Sharpens
`ref/research/grid-comparison-report.md` §5.2's `[D]` mechanism, whose global "R/h doubles"
account three controls here fail to reproduce.

### 5.2 Proposed segment edits

**`#detail-structure-scheme-map`** — replace WN row 4 (GCL) and row 5 (ABF) with present-tense
outcomes; both currently state open questions this spike closed. Draft:

> - **The discrete GCL is derived, exact, and currently unbuilt.** Cube-sphere cell edges are
>   great-circle arcs, so the outward in-surface normal along an edge is that arc's constant
>   pole and the metric identity is $\sum_e L_e\hat p_e + 2\!\int_K\hat r\,d\Omega = 0$ —
>   verified to ~1e-15 against independent quadrature, and its free-stream half is
>   combinatorial (exact edge integrals telescope). What fails is the *stand-in*: the
>   flux-form kernel carries a uniform-Cartesian metric set whose free-stream residual is
>   O(1) and does not decay. The identity is therefore an acceptance test for the unbuilt
>   edge quantities ( #form-grid-equiangular-staggered Epistemic Status), not a diagnostic of
>   existing ones. The covariant escape (Montoya et al.) remains unpriced, and its advantage
>   *on this axis* is smaller for us than for a general curvilinear mesh: our metric terms are
>   closed-form, not approximated.
> - **The ABF RT₀ trap does not apply to us, and the check's value was stopping a false
>   attribution.** ABF's non-convergence is a statement about representing the divergence
>   given exact fluxes, on a mesh sequence of *fixed* cell shape; a cube-sphere quadtree is
>   asymptotically affine (δ/h → 0 at order 1, measured), and the measured two-point defect is
>   present on a parallelogram lattice where ABF is provably absent. Bochev–Ridzal does exempt
>   an FV divergence, but that exemption buys nothing the measurement needed. The measured
>   inconsistency is ours, and it is a **panel-seam** defect ( #obs-… , §5.3 below).

**`#form-face-flux-register`** — extend WN row 4 ("The seam has two independent reasons to be
broken") to three, and note that the third needs no hanging node:

> - **A third, and it is present without any refinement mismatch.** On a *conforming,
>   same-level* cube-panel seam the two-point flux Laplacian has O(1/h) local relative error
>   (measured, doubling per rung), carrying ~99.5% of the whole-sphere L2 defect while the
>   face interior converges. Lipnikov's coarse–fine non-convergence and the bulk
>   non-orthogonality inconsistency are the other two; this one is a property of the panel
>   seam alone.

**`#form-grid-equiangular-staggered`** — the Epistemic Status already says the edge
quantities are unbuilt; propose adding that the discrete GCL is their **acceptance test**,
with the identity named, so the successor cannot land them unchecked.

**`#disc-open-problem-census`** — *not edited, per the brief.* Two census rows landed this
morning are now closed (survey §4.8 correspondence; §4.9 GCL status) and one new open problem
is opened (the panel-seam O(1/h) mechanism, and its decisive test: freeze the metric across
a band either side of a seam and see whether the growth vanishes).

### 5.3 A slug this spike wants and does not have

The seam finding is an **observation** with a measurement and no home:
`#obs-panel-seam-flux-diverges` (or similar). `#obs-cube-locked-kernel-bias` and
`#obs-routing-curl-spiral` are the shape to copy. Flagged rather than drafted — naming is
Joseph's and the orchestrator's, and `core/OUTLINE.md` is off-limits this hour.

---

## 6. Feedback on the brief, and what I would want next

**On the brief.** The single most useful thing in it was the beliefs-vs-measurements
separation in the fourth paragraph — I used it twice. The first time was writing §0's
"honest prior" block, which is where I noticed the survey's "cheap probe" framing for the GCL
was itself a *belief* about a probe nobody had scoped, and that noticing is what turned P3
from a diagnostic into a spec. The second was A5: the brief's register discipline is why the
landing note says "REFUTED on the real instrument" in a scorecard rather than quietly
dropping a prediction that did not survive.

**What the brief could not have known, and I would not have found had it enumerated less.**
The brief named two items. The most consequential result — the panel-seam localisation — is
neither of them; it came from a discrepancy between my own probe and `grid_lab`, three
follow-ups deep, and it exists only because the brief said "push as hard and far as you can"
and "anything adjacent that looks wrong, name it" rather than scoping the two checks. A
tighter brief would have gotten two clean answers and missed the thing that matters most for
the seam work happening this hour.

**One thing I would change.** The brief says the survey author thought §4.8 was the
highest-value unchecked thing in the document. That is true and it was worth saying — but it
also set an expectation of a *positive* finding ("our measured defect has a 2005 theorem
behind it") that I had to actively resist for the first half-hour, because the shape of the
survey's own `[me]` tags leans that way. The register that would have helped: *"the survey
believes this and flagged it as unproven; the outcome that would most change our plans is
either sign."*

**What I would want next, in order.**
1. The panel-seam mechanism test (freeze the metric in a band either side; ~30 minutes).
2. Promote the wavelet-store spike's `arc_len_m` / `east_edge_len_m` / `north_edge_len_m` into
   `measure.rs` with `edge_pole`, gated on the GCL identity as their acceptance test.
3. Re-read `grid_lab` §7's narrative prose against §4.3 above — several printed lines assert
   the global R/h mechanism, and they now have a measurement pointing elsewhere.

**Standing by** for follow-ups.
