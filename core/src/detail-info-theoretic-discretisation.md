---
slug: detail-info-theoretic-discretisation
type: detail
status: robust-qualitative
stage: draft
depends:
  - form-column-control-volume
  - norm-bias-vs-noise
  - post-determinism-as-ontology
  - form-declared-structure-tradeoff
  - form-face-flux-register
  - obs-cube-locked-kernel-bias
  - sketch-detail-abstract-reversion
  - sketch-dynamic-exponent-seams
---

# Information-theoretic discretisation dictionary

Present-best dictionary: discretisation as sampling; cell-average vs point-sample as the anti-aliasing fork; null modes as Nyquist zeros; nonlinearity vs projection (Jensen vs spectral aliasing as siblings, not identity); fated jitter as measurable fluctuation partner (not free FDT amplitude); Noether → isotropy defect; logical depth ≠ LoD; multiresolution store as **representation** identity (not dynamics). Supports column semantics, bias adjudication, and reversion — does not restate those laws.

## Formal Expression

1. **Discretisation is sampling.** Cell size $\Delta$ ⇒ Nyquist $k_N=\pi/\Delta$. Content above $k_N$ is unrepresentable: filter before sample, or let it fold back.

2. **Cell-average = anti-aliased; point-sample = aliased.** $\bar h_i = A^{-1}\int_{\mathrm{cell}} h$ is convolution with a box then a sample. A raw point sample omits the filter. **The column-semantics fork is the anti-aliasing fork** ( #form-column-control-volume FE(4)). Live defect specimen: generator band-limits to Nyquist (anti-alias); mean-pin assumes cell average; bilinear render assumes point sample — three-way disagreement about which filter was applied.

3. **Invisible mode = Nyquist null.** Checkerboard $(-1)^{i+j}$ is in the null space of a centered difference. Cardiff “rank deficiency” ≡ transfer function zero at Nyquist. Staggering halves effective derivative spacing and can push the null out of range. Stabilisation is a high-pass damping filter making the transfer function nonzero on its own null space ($\alpha^{\mathrm{stab}}$ is a gain, not a free fudge) — **when** the pathology is that rank deficiency; staggered hyperbolic schemes can need other stabilisers for other reasons (do not auto-label every $\theta$ as Rhie–Chow).

4. **Nonlinearity does not commute with projection — two different residues.** Shared parent: project-then-nonlinear ≠ nonlinear-then-project. **Not identity:**
   - **Cell-average / Jensen:** sign-definite **bias in the mean**.
   - **Point-sample / spectral aliasing:** fold-back among resolved modes; **not** sign-definite (2/3 rule addresses aliasing, not Jensen).
   - Stream-power specimen at live $n=1$: Jensen gap in $S$ is zero (linear in slope); residual over-estimate paths are **Jensen on $A^{0.5}$** ($m=0.5$ concave) and **negative $\mathrm{Cov}(A^m,S)$**. Honest sufficient statistic needs joint sub-grid $(A,S)$ (at least covariance). **$A$ is what the router computes** — routing bias and coarse-graining bias share a variable ( #obs-cube-locked-kernel-bias precondition).

5. **Fated jitter is the fluctuation half of a dissipation–fluctuation pair — amplitude measured, not free.** Mori–Zwanzig: coarse-graining reversible microdynamics yields memory (dissipation) + orthogonal dynamics (noise). **Second FDT magnitude tie assumes an equilibrium Gibbs measure** — not handed for driven landscape evolution. What survives: (i) discarded variance sets both partners **in principle** and can be **measured from fine runs** (LES dynamic-procedure shape); (ii) fated noise makes the fluctuation half deterministic and replayable ( #post-determinism-as-ontology ); (iii) **jitter fights Jensen** on superlinear laws — inject variance into $S$, apply convex $f$, manufacture new bias; measure before adopting jitter as fallback; (iv) uniform zero sub-grid variance is a **positive claim** (measure-zero), not a safe default.

6. **Noether reads the MFD bias.** Continuous rotational symmetry broken by the grid ⇒ isotropy of flow direction not conserved — theory twin of the measured fan bias ( #obs-cube-locked-kernel-bias ).

7. **Logical depth ≠ level of detail.** Coarse is a low-pass of the **present**, not “old.” LoD couples to timescale only through process-dependent $z$ ( #sketch-dynamic-exponent-seams ); no single LoD↔age map. Bennett logical depth (simple to specify, expensive to compute) is the honest “age”; AAT compute-shortfall is decompression cost — sibling, not this segment’s law.

8. **Multiresolution store: representation identity, not dynamics commute.** $\mathrm{fine}=\mathrm{coarse}\oplus\mathrm{detail}$ makes the **representation** exactly invertible; a **state** delta can propagate to the root in $O(\log N)$. It does **not** make nonlinear evolution commute with restriction — that is the open law half ( #sketch-detail-abstract-reversion ). Injection ≠ refluxing; storing state details does not free flux-identity at hanging faces without a bill ( #form-face-flux-register ). Dyadic ladders are principled for scale-free fields (equal information per octave) and **arbitrary for scale-full processes** — open per phenomenon.

9. **Joseph’s inversion (coarse/fine contract).** What the coarse instrument cannot capture to aliasing is where finer detail is **allowed** to live: coarse value + declared sub-grid statistic define a conditional over admissible fine states. Materialise = sample the conditional (fated); abstract = apply the filter; consistency holds iff the sampler is conditioned on the coarse.

## Epistemic Status

**Max attainable: exact** for classical sampling / null-mode / Noether statements; **robust-qualitative** for project mappings and measured specimens. **Currently `robust-qualitative`:** present-truth after Jul-13 red-team corrections (Jensen≠aliasing identity retracted; FDT amplitude claim retracted; wavelet “retires the seam” retracted). Stage `draft`. `detail` segment — supports column, bias, reversion; invents no NomosDecl schema.

## Discussion

Agents who only hold “declare the structure” still re-litigate anti-aliasing, null modes, and wavelet mythology. This dictionary is the shared language that makes those re-litigations short and stops analogy-in-costume promotions.

## Working Notes

- **Source peel:** `.super-archive/from-theory/discretisation-and-information.md` §3 present-best only; long correction blocks stay history in the graduated source.
- **Sibling measurements:** #obs-mean-pin-manufactures-seam ; wavelet-store DECISIONS; water validity-envelope DECISIONS (not re-proved here).
- **Do not re-promote:** Jensen≡aliasing; FDT-determined jitter amplitude for $\theta$; “store details ⇒ seam never happens.”
