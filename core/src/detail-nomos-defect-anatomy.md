---
slug: detail-nomos-defect-anatomy
type: detail
status: robust-qualitative
stage: draft
depends:
  - sketch-nomos-declaration-boxes
  - form-flux-web
  - form-declared-structure-tradeoff
  - disc-prime-question
  - norm-bias-vs-noise
  - norm-declaration-must-convict
  - form-column-control-volume
  - obs-cube-locked-kernel-bias
  - obs-mean-pin-manufactures-seam
---

# Nomos defect anatomy — five boxes from the Jul-13 sample

Complete teaching anatomy of the Jul-13 defect sample: each measured failure maps to a declaration box. Fields live on `NomosDecl`; this segment holds the **specimen map, earning-procedure shape, and failure-gallery seeds** so nothing non-superseded depends on iced prose.

## Formal Expression

1. **Five boxes.** ① quantities · ② geometry (assume vs deliver) · ③ semantics (statistic + exactness) · ④ structure (preserve / approx / sacrifice + conflicts) · ⑤ claim (unphysical term: parity + bias/noise). Same anatomy as #sketch-nomos-declaration-boxes .

2. **Generalization.** Box ② is consumes/produces applied to **geometry**; ③ to **meaning**; ④ to **invariants**; ⑤ to the **lie**. Quantity matching (①) is necessary and shallowest.

3. **Defect → box map (full sample).**

| the defect | box |
|---|---|
| uniform `cell_m²` drainage area (+17.8%, cube-locked, cannot out-resolve) | ② |
| MFD fan (even angular sampling assumed; grid delivers sheared Jacobian) | ② |
| Hardcoded `cell_m` / `cell_m·√2` neighbour distances | ② |
| TPFA orthogonality assumption (~28.79° off on our mesh) | ② |
| Hydrostatic reconstruction proved failure when $\Delta z \ge h$ (common case) | ② |
| Column three-way semantic ambiguity (point / mean / band-limited) | ③ |
| MFD output is a **boundary integral**, not a discharge | ③ |
| Jensen — statistic needed is joint $(A,S)$, not $\mathrm{var}(S)$; at live $n=1$ gap is in $A^{0.5}$ and $\mathrm{Cov}(A,S)$ | ③ |
| mean-pin claims $R\circ L=\mathrm{id}$ on the mean and does not hold | ④ |
| $\theta$ vs entropy condition (structure it actually breaks) | ④ |
| `water.rs` **is** well-balanced, staggered, no null space — **undeclared assets** a rewrite would destroy | ④ |
| Routing enforces acyclicity (strictly-downhill clamp) — structure, not mystery | ④ |
| $\theta$'s physical claim is **NONE** (even/Laplacian diffusion; cannot be the odd/advective term a comment claimed) | ⑤ |
| $p=1.1$ is 45°-periodic first-moment deflection — manufactures the bias it was thought to cancel | ⑤ |
| Jarrett-as-dynamical-law (regression for estimating $n$ from measured slope, used as feedback) | ⑤ |
| Three one-sided clips in `water.rs` (sign-definite ⇒ bias by construction) | ⑤ |
| uplift output **range** cannot reach promise **predicate** | ordinum **reachability** (sibling, not a sixth box) |

4. **Flux web vs truth questions.** The flux web can answer *"can we rain principled water?"* (plumbing). It cannot alone answer bias-vs-noise, number meaning, what was destroyed, or what unphysical term was added — those are boxes ②–⑤ + #norm-bias-vs-noise .

5. **Earning-procedure shape (unwritten files; open intent).** Each box still needs a procedure (not a taxonomy nobody reads) with three disciplines: (i) answers *"how do I find out what's true here?"* — declaration is **output**, procedure is how you **earn** it; (ii) carries its own **probe** ( #norm-declaration-must-convict ); (iii) carries a **failure gallery** including wrong answers and why they were seductive. Shape: question · fundamentals (math done) · procedure · worked specimen · probe · failure gallery · what to write in the declaration. Target layout (unwritten): quantities / geometry / semantics / structure / claim procedure files — prefer core peels over a parallel `doc/design/nomos-contract/` tree.

6. **Richest specimens per box (measured; DECISIONS + probes).**
   - ① rain-without-sky; ordinum unkept promise ⇒ unrunnable; statistic/exactness contract load-bearing after Jensen; reachability (uplift)
   - ② Joseph metric set (area, edge, centre-to-mid-edge arm, angle, valence, joint stencil, curvature split, strata volume); Jacobian shear/det; shear has no $N$; coarse/fine carve (L25 sphere gone; L2 sagitta ~24% of cell); **failure: audited valence instead of angular sampling**
   - ③ anti-alias identity; centroid volume-exact reconstruction; $(1,6,1)/8$ curvature term; Jensen at $n=1$; **failure: semantics declared in prose, code drifted through guard**
   - ④ structure conflicts ( #detail-structure-scheme-map ); linear-only seam commute; **failure: nearly "fixed" water out of its undeclared assets**
   - ⑤ Warming–Hyett modified equation; even=diffusion / odd=advection; sign-definite ⇒ bias; **failure: $\theta$ comment vs paper; $p=1.1$; Jarrett; unprobed clips**

7. **Family census (not one FVM).**

| nomos | family |
|---|---|
| `water.rs` | staggered Arakawa-C, flux-form, well-balanced (lake-at-rest measured) |
| erosion creep | collocated, naive-uniform |
| erosion routing | graph/fan — not FVM; phantom faces ~47.8% of MFD-8 "flux" |
| hydrosphere | reservoir/box — no grid |
| climate | box→field coupling |

Declare family + **between-family seams** ( #form-declared-structure-tradeoff ). Linear structures cross iff restriction commutes; nonlinear (entropy, Jensen) cannot.

8. **Appendix — seductive wrong answers (epistemic completeness).** Re-litigation traps to avoid: diagonal-pipes "fix" against already-staggered water; Rhie–Chow label on $\theta$ (Lax–Friedrichs family; staggered has no null mode for Rhie–Chow); mean-pin as Haar reflux substitute; curl-free $q$ probe target (use contour-orthogonality $\kappa$); Jensen≡aliasing identity; FDT-determined jitter amplitude for driven landscapes; "store details ⇒ seam never happens."

## Epistemic Status

**Max attainable: robust-qualitative** for the defect→box sort as a measured sample restatement. **Currently `robust-qualitative`.** Stage `draft`. Completeness of the five-box set is unproved. Fields on `NomosDecl` 2026-07-24; content conviction still probe-grade.

## Discussion

Without this anatomy in core, agents re-mine iced NOMOS-CONTRACT or re-discover defects cold. Failure galleries earn their keep only if agents pattern-match **why the wrong answer was seductive**.

## Working Notes

- Supersedes treating iced NOMOS-CONTRACT as the Level-B read for field nomos.
- Procedures still open — do not launch five unreviewed agents without review capacity (historical intent, not a standing task list).
