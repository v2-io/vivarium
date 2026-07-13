# Literature reconnaissance — multiresolution store for the cube-sphere quadtree

*Recon pass, 2026-07-13. Scope: the four questions in the spike brief. This file is
evidence, not conclusions-for-the-segment.*

## Verification key

| tag | meaning |
|---|---|
| **[P]** | I read the primary source text and the quote/claim is verbatim or a direct paraphrase of what I read |
| **[S]** | secondary — abstract, another paper's characterization, or a search summary; NOT read in the primary |
| **[D]** | my derivation *from* [P] material; not itself a claim I found stated in the literature |
| **[∅]** | searched for and did not find |

Full texts pulled and read live in the session scratchpad (not committed): Harten ICASE 94-59,
Abgrall–Harten SINUM 1998, Daubechies–Sweldens JFAA 1998, Schröder–Sweldens SIGGRAPH 95,
Lessig–Fiume TOG 2008, Duarte et al. 2015, Deiterding et al. 2016, Gomes et al. 2021,
Bellotti et al. 2022, Deiterding et al. ESAIM Proc 2009.

---

## Q1 — Haar on non-equal weights

### The generalization is *forced*, not chosen, and it is exactly area-weighting

**Abgrall & Harten (1998), SIAM J. Numer. Anal. 35(6):2128–2146, §4** — cell-average
multiresolution on *unstructured* meshes, coarse levels obtained by **agglomeration** of
`q` children of arbitrary shape/size. The decimation (analysis low-pass) operator: **[P]**

> "In any case, the sequence of discretization {Dᵏ} is nested, and it follows from the
> additivity of the integral that
>   (D_{k−1} f)_i = (1/|C_i^{k−1}|) Σ_{ℓ=1}^{q} |C_{i_ℓ}^k| (D_k f)_{i_ℓ} = (D_k^{k−1} D_k f)_i ,
> which directly defines the decimation operator."

That *is* mean-pinning with unequal weights. Harten's own gloss elsewhere in the same
section: **"the only degree of freedom left is the reconstruction"** — i.e. the
area-weighted mean is not a design choice, it is the integral; the *prediction* operator
is the only free part. **[P]**

Identically in **Duarte, Bonaventura, Massot & Bourdon (2015)**, *J. Comput. Phys.* /
arXiv:1311.2488, eqs. (2)–(5): projection `f_γ = |Ω_γ|⁻¹ Σ |Ω_μ| f_μ`, "exact and unique"
for nested grids; and the *dual scaling function* is `φ̃_γ := |Ω_γ|⁻¹ χ_{Ω_γ}`. **[P]**

And in **Schröder & Sweldens (1995)**, SIGGRAPH 95, §3.6 (spherical triangles of *unequal*
spherical area μ(T)): **[P]**

> `φ_{j,k} = χ_{T_{j,k}}` and `φ̃_{j,k} = μ(T_{j,k})⁻¹ χ_{T_{j,k}}`
> "The fact that the scaling function and dual scaling function are biorthogonal follows
> immediately from their disjoint support."

Three independent primary sources agree: **the coarse coefficient is the area-weighted
mean because the dual (analysis) scaling function is `χ_T / area(T)`.** This is not
a generalization anyone had to invent — it falls out of "the coefficient is `⟨f, φ̃⟩`".

### Invertibility / perfect reconstruction — proven, in exactly the unequal-cell case

**Abgrall & Harten (1998) eq. (4.2)** — the prediction error `e` lies in the null space
of the decimation, which for unequal cells reads: **[P]**

> Σ_{ℓ=1}^{q} |C_{i_ℓ}^k| e_{i_ℓ}^k = 0
>
> "This relation shows that we can define the scale-coefficients dᵏ by taking (q−1)
> properly chosen linear combinations of the q prediction errors {e_{i_ℓ}^k} in each cell
> C_i^{k−1}. These linear combinations should be chosen so that together with (4.2) they
> constitute an invertible system of q linear equations for the prediction errors."

So for our quadtree (`q = 4`, areas `a₁..a₄`): the **area-weighted** sum of the four
details is zero, leaving **exactly 3 free details per parent**, and *any* 3 linear
combinations completing (4.2) to an invertible 4×4 system give a bijection

  `(4 fine values) ↔ (1 coarse value, 3 details)`.

That is the identity the spike hypothesises, stated in the literature, in the
unequal-cell case, by Harten. Answer to Q1(a) invertible: **yes**. Q1(b) perfect
reconstruction: **yes** — same statement.

### Orthogonality — the hypothesis is *half* right; do not concede Parseval

The suspicion "it must be biorthogonal, and this costs Parseval" is **true of the natural
construction but false as a necessity.**

- **The natural construction is biorthogonal / semi-orthogonal.** Schröder–Sweldens call
  theirs the **Bio-Haar** basis (φ = indicator, φ̃ = indicator/area), with "a set of
  **semi-orthogonal** dual wavelets"; Lessig & Fiume describe it as "a semi-orthogonal and
  symmetric spherical Haar wavelet basis". **[P]**
  Note *semi-orthogonal* ≠ merely biorthogonal: it means `V_j ⊥ W_j` in L², so you **do**
  get the cross-scale Pythagoras/energy split; what you lack is orthonormality *within*
  the 3-dim detail space of each parent. **[P + D]**

- **Orthogonal unbalanced Haar on unequal-measure children exists.**
  - **Girardi & Sweldens (1997)**, *J. Fourier Anal. Appl.* 3(4):457–474, "A new class of
    unbalanced Haar wavelets that form an unconditional basis for L_p on general measure
    spaces": nested partitions of an arbitrary σ-finite measure space; unconditional basis
    for L_p, 1<p<∞; unconditional basis constant `max(p,q)−1`; **"We derive a fast
    algorithm to compute the coefficients."** **[P — abstract only; full text not obtained]**
    Lessig & Fiume characterize it as: *"Girardi and Sweldens [1997] developed **orthogonal**
    Haar wavelet bases over general measure spaces L_p. The scaling functions employed in
    their work are identical to those of the SOHO wavelet basis but their wavelet
    construction does not yield a **symmetric** basis on the sphere."* **[P]**
  - **Lessig & Fiume (2008)**, "SOHO: Orthogonal and Symmetric Haar Wavelets on the Sphere",
    *ACM TOG* 27(1), give the explicit **4-child unequal-area** construction. Scaling
    functions `φ_{j,k} = τ_{j,k}/√α_{j,k}` (α = spherical area), and the semi-orthogonality
    condition `⟨ψ, φ_parent⟩ = 0` is solved as the **nullspace of the Gram matrix**, giving
    (their Eq. 14) the synthesis matrix whose first row is
    `[ −√α₁/√α₀ , −√α₂/√α₀ , −√α₃/√α₀ ]` with the identity below it. **[P]**
    That is *the* unequal-weight generalisation of the 4-child Haar filter bank, written out.

- **What orthogonality costs on a fixed partition.** SOHO could only get orthogonal **and**
  symmetric by *changing the subdivision*: **[P]**
  > "For the SOHO wavelet basis, the vertex positions are chosen so that the areas of the
  > three outer child triangles … **are equal**. This is the key to the derivation of a
  > basis that is both orthogonal and symmetric."

  We cannot move our cell boundaries (the cube-sphere partition is fixed by the projection).
  So: on a fixed unequal-area partition, you may have **orthogonal but not symmetric**
  (Girardi–Sweldens / SOHO Eq. 14 + Gram–Schmidt), or **semi-orthogonal and symmetric with
  physically meaningful coefficients** (Bio-Haar). **[P for the two horns; the "must pick"
  framing is D]**

- **Parseval, precisely.** For a piecewise-constant field over cells of area `a_i`, the L²
  norm *is* `Σ a_i h_i²` — the area-weighted energy is not an approximation to the energy,
  it is the energy. **[D]** Semi-orthogonality (`V_j ⊥ W_j`) already gives
  `‖f_{j+1}‖² = ‖f_j‖² + ‖detail part‖²` in that norm. **[D from P]** If you additionally
  want an orthonormal detail basis, Gram–Schmidt the 3-dim per-parent detail space (that is
  exactly what SOHO/Girardi–Sweldens do) — the cost is that the stored numbers are no longer
  "deviation of the child field value from the parent", i.e. they stop being in physical
  units and stop being symmetric under child relabeling. **[D]**

- **Conservation does not depend on orthogonality at all.** It depends only on
  (i) the weighted decimation and (ii) the consistency `D∘P = Id`. **[P — see Q3]**

### ⚠ Cautionary tale directly on point

Lessig & Fiume, on Ma et al. (2006), who used a Haar-like basis over the sphere: **[P]**

> "the authors assumed the subdivision of a partition yields child domains of **equal
> area**. This is in general not true and the pseudo Haar wavelets are therefore **not a
> basis of the space L²(S², dω)**."

That is exactly the failure mode of applying the textbook Haar to a cube-sphere quadtree.

---

## Q2 — Lifting / second-generation wavelets

### Perfect reconstruction by construction — YES, and this is the crux, verbatim

**Daubechies & Sweldens (1998), "Factoring wavelet transforms into lifting steps",
*J. Fourier Anal. Appl.* 4(3):247–269, p. 249:** **[P — exact quote]**

> "This illustrates one of the built-in features of lifting: **no matter how P and U are
> chosen, the scheme is always invertible and thus leads to critically sampled perfect
> reconstruction filter banks.**"

Mechanism: predict is `d = x_o − P(x_e)` (recover `x_o = P(x_e) + d`); update is
`s = x_e + U(d)` (recover `x_e = s − U(d)`). Each is an elementary/triangular polyphase
matrix of determinant 1, so the inverse is obtained by *running the steps backwards with
flipped signs*. Their Theorem 1 (Lifting): "This operation does not change the determinant
of the polyphase matrix." Theorem 4 / abstract: any FIR two-band PR filter bank factors
into lifting steps, because `SL(n; R[z,z⁻¹]) = E(n; R[z,z⁻¹])`. **[P]**

Second-generation restatement, **Schröder & Sweldens (1995) §3.1** — no Fourier, no
translation invariance, arbitrary irregular subdivision: **[P — exact quote]**

> "…and that, **for any choice of {s_{j,k,m}}, the new filters will automatically be
> biorthogonal, and thus lead to an invertible transform.**"

**Caveat worth carrying:** the *completeness* half of Daubechies–Sweldens (every FIR DWT
factors into lifting steps, via the Euclidean algorithm on Laurent polynomials) is a
**first-generation, translation-invariant** result. What carries to the manifold /
irregular-subdivision setting is **invertibility-by-construction**, which is the half the
spike needs. **[D, flagged]**

Nonlinear predictors are explicitly allowed and still perfectly reconstruct — this is the
basis of integer-to-integer wavelet transforms (Calderbank, Daubechies, Sweldens & Yeo,
ICIP 1997: "Invertible wavelet transforms that map integers to integers … based upon the
idea of factoring wavelet transforms into lifting steps"). **[P — Sweldens' own abstract]**

### What the UPDATE step does — it is *exactly* the mean/integral preserver

**Daubechies & Sweldens (1998), p. 248–249:** **[P — exact quote]**

> "The frequency separation is poor since x_e is obtained by simply subsampling so that
> serious aliasing occurs. **In particular, the running average of the x_e is not the same
> as that of the original samples x. To correct this,** we propose a second lifting step,
> which replaces the evens with smoothed values s with the use of an update operator U
> applied to the details: `s = x_e + U(d)`."

So the answer to "is the update step the thing that preserves the mean/integral?" is
**yes, and that is literally why it was introduced.** Confirmed in the manifold setting by
Schröder & Sweldens, who choose the lifting weights precisely to give the wavelet a
vanishing integral: *"The weights s_{j,k,m} are chosen so that the resulting wavelet has a
vanishing integral"* (with `I_{j,k}` = the *spherical area* integrals of the scaling
functions). **[P]** Vanishing wavelet integral ⇔ the scaling coefficient carries the full
integral ⇔ conservation.

### Wavelets on the cube-sphere / spherical quadtrees — what exists

| work | grid | data | unequal area? |
|---|---|---|---|
| **Schröder & Sweldens 1995** (SIGGRAPH 95, 161–172) | spherical triangle **quadtree** (geodesic sphere), 1→4 children | cell averages (Bio-Haar face basis) **and** point values (vertex basis) | **Yes — explicitly**, `μ(T_{j,k})` throughout; "allow fully adaptive subdivisions" **[P]** |
| **Nielson et al. 1997** (Vis '97), **Bonneau 1999**, **Roşca 2005** | spherical triangles | Haar-like | semi-orthogonal / "nearly orthogonal" (orthogonal only in the subdivision limit) **[S, via Lessig & Fiume]** |
| **Lessig & Fiume 2008** (TOG 27(1)) | spherical triangles, *custom* subdivision | Haar | **Yes**, and achieves orthogonal + symmetric by *choosing* vertices for equal outer-child area **[P]** |
| **Chevrot, Martin & Komatitsch 2012** (GJI 191(3):1391–1402) | **the cubed sphere** (6 chunks), lifting scheme, CDF(4,4) + Haar | **point values** on the nodes | **No** — treats each face uniformly in Cartesian face coords; area distortion / metric weighting **not addressed** **[S — WebFetch of the article page; I did not obtain the full PDF]** |
| **Domingues, Gomes, Roussel, Schneider** (Adv. Comput. Math. 2014) | adaptive **spherical geodesic grid**, recursive refinement | multilevel gradient operator | **[S — abstract only]** |

**[∅] Gap found:** I found **no** conservative *cell-average* multiresolution/wavelet
construction on the **equiangular cube-sphere** that carries the ~1.4× area spread. The
closest ancestor is Schröder–Sweldens' Bio-Haar (triangular quadtree, unequal spherical
areas, cell averages) — the construction transfers essentially verbatim to a quad
quadtree; what is *not* in the literature is that transfer. This looks like a genuine
(small, honest) novelty surface, not a reinvention.

---

## Q3 — Multiresolution, AMR, and refluxing

### What Harten's framework actually is

**Harten, "Multiresolution Representation and Numerical Algorithms: A Brief Review",
ICASE Report 94-59 / NASA CR-194949 (Oct 1994)** — Harten's own review; full text read. **[P]**

- Nested discretizations ⇒ a **decimation** operator `D_k^{k−1}` (forced) and a
  **reconstruction/prediction** `R_k` (free), subject to the **consistency** condition
  `D_k R_k = I` — his eq. (2.15) — which **Harten himself calls "conservation"**:
  > "In the context of ENO schemes for the numerical solution of conservation laws we refer
  > to R_k as reconstruction from cell-averages and to (2.15) as **'conservation'**."
- The multiscale transform is **exactly invertible**:
  > "Clearly there is a **one-to-one correspondence** between vᵏ and {dᵏ, v^{k−1}}"
  > … "there is a one-to-one correspondence between the values of the finest level v^L and
  > the sequence {d^L, …, d¹, v⁰} =: v_M."
- In the cell-average setting the details within a parent are **redundant** — his eq.
  (2.17b), `e_{2i−1} + e_{2i} = 0` (equal-size 1-D case; the unequal-cell version is
  Abgrall–Harten (4.2), area-weighted) — so you store `q−1` of them.

**So: Harten's multiresolution representation is LOSSLESS.** The bijection is the whole
point. **[P]**

### What it achieved, and what it did *not*

- **Achieved:** a lossless change of coordinates, plus the observation that thresholding
  small details is a *regularity indicator*, which lets you skip expensive flux
  evaluations. Harten's own §7 Burgers example runs with **"tolerance ε = 10⁻³"** and
  reports "the **efficiency** (i.e. the ratio between the fine grid calculation of 256
  fluxes over the number that we actually had to compute)". **[P]**
- **Did NOT achieve (Harten's original scheme):** memory reduction. The solution still
  lives on the full uniform fine grid. **[P — Bellotti et al. 2022:]**
  > "The principle was to use multiresolution to reduce the number of computations to
  > evaluate fluxes at the interfaces, claiming that they constitute the majority of the
  > computational cost. **However, this approach still computes the solution on the full
  > uniform mesh.** The possibilities offered by multiresolution had been further exploited
  > by Cohen et al. who, in the footsteps of Harten, have developed **fully adaptive**
  > schemes with solutions updated only on the reduced grid."
- **Therefore:** the *lossy* part (thresholding, tolerance ε) is a **cost-saving layer
  bolted onto a lossless representation**, and the two are separable. For the spike this is
  the load-bearing fact: **you can take the lossless store and simply not threshold.**
  Nothing in Harten's framework requires the approximation. **[P + D]**

### The refluxing question — the honest answer

The literature *does* directly compare MR and Berger–Colella AMR, in two papers by the
same group:
- Deiterding, Domingues, Gomes, Roussel & Schneider, **ESAIM: Proc. 29 (2009) 28–42**,
  "Adaptive multiresolution or adaptive mesh refinement? A case study for 2D Euler equations".
- Deiterding, Domingues, Gomes & Schneider, **arXiv:1603.05211** (2016) → *SIAM J. Sci.
  Comput.*, "Comparison of adaptive multiresolution and adaptive mesh refinement …".

**AMR (Berger–Colella) — why refluxing is needed** — arXiv:1603.05211 §2.2: **[P — exact quote]**

> "Values of cells covered by finer submeshes are subsequently **overwritten by averaged
> fine mesh values, which, in general, would lead to a loss of conservation on the coarser
> mesh**. A remedy to this problem is to replace the coarse mesh numerical fluxes at
> refinement boundaries with the sum of fine mesh fluxes along the corresponding coarse
> cell boundary."

**MR — how the correction step disappears** — Gomes, Domingues, Mendes, Schneider et al.,
arXiv:2102.11806 (2021), §3: **[P — exact quote]**

> "The conservation properties of the finite volume method are preserved in our adaptive
> multiresolution discretization **taking special care in the flux evaluation**. To ensure
> the balance of ingoing and outgoing fluxes at the cell interfaces on adjacent refinement
> levels … **the ingoing fluxes at level ℓ are computed as the outgoing fluxes of the
> corresponding cells at level ℓ+1**. This is possible due to the graded-tree structure,
> which keeps the nearest cousins of a cell **or creates a virtual leaf for the flux
> computations**. Thus the flux computation is conservative between cells at different
> levels of refinement."

Origin of the virtual-leaf construction: **Roussel, Schneider, Tsigulin & Bockhorn,
"A conservative fully adaptive multiresolution algorithm for parabolic PDEs",
*J. Comput. Phys.* 188 (2003) 493–523.** (Code: CARMEN.) **[S — cited as the source by
both Gomes et al. and Deiterding et al.; I did not obtain the JCP full text.]**

### ⚠ The part of the hypothesis that does NOT hold as stated

**[∅] I found no paper claiming that *storing the detail coefficients* makes the
coarse–fine FLUX mismatch algebraically zero.** Two mismatches must be separated, and the
literature separates them:

1. **Restriction / averaging mismatch — "mean-pinning".** In a cell-average MR store the
   coarse value **is** the area-weighted mean of its children *by definition of the
   decimation operator*. There is nothing to enforce and nothing to repair. **This half of
   the hypothesis is fully supported** — Harten (2.14b)/(2.15), Abgrall–Harten (4.2),
   Duarte et al. (3)/(5). **[P]** It is an identity, not a constraint. ✔

2. **Flux mismatch at a level jump.** This is produced by **time evolution**, not by the
   representation. It is zero iff the interface flux is **single-valued** — i.e. the coarse
   cell does not independently compute its own flux from coarse states, but adopts the sum
   of the fine fluxes (the virtual-leaf discipline above). That is a property of the
   **flux-evaluation rule**, not of the wavelet transform. **[P]**

What the MR store *does* buy against refluxing, precisely: **[D from P]**
- Evolution happens **only on leaves**. A coarse cell under refinement is a *non-leaf*: it
  is never independently evolved and then overwritten by an average. Berger–Colella
  refluxing exists precisely to repair the damage of that double-evolve-then-overwrite; MR
  removes the double-evolve, so there is nothing to repair.
- The graded tree + prediction operator hands you the fine-level states at the interface for
  free, so computing the single-valued interface flux at the finer level is cheap.

**Correct claim shape for the spike:** *the refluxing step disappears in a multiresolution
formulation* — verified, and standard practice in the MR codes (CARMEN, CARMEN-MHD) — *as a
joint consequence of (a) exact weighted decimation, (b) leaf-only evolution, and (c) a
single-valued interface flux rule*; **not** as an algebraic identity of the wavelet
transform alone. Claiming the latter would be an overclaim the literature does not support.

---

## Q4 — Nonlinearity (E = K·Aᵐ·Sⁿ)

### What survives

- **The bijection.** The MR store is a change of coordinates *on the state*. Its
  invertibility, and the exactness of the coarse↔fine identity, are completely indifferent
  to the nonlinearity of the operator that later acts on the state. **[D — trivially implied
  by the Q1/Q2 results; nothing to verify]**
- **Exact conservation.** Harten keeps the scheme in **conservation form** (his (7.13b),
  `vⱼⁿ⁺¹ = vⱼⁿ − λ(f̂_{j+½} − f̂_{j−½})`) and only replaces the *expensive* numerical-flux
  evaluation by a cheap one. Because the scheme stays in conservation form with **one flux
  per interface**, conservation is exact **even where the flux value is approximate**. Only
  *accuracy* degrades. **[P for the conservation-form structure; the "cheap one = polynomial
  interpolation from the coarser grid" characterization is [S] from the CPAM 1995 abstract
  — I could not obtain that full text.]**
  Cited as: **A. Harten, "Multiresolution algorithms for the numerical solution of hyperbolic
  conservation laws", *Comm. Pure Appl. Math.* 48 (1995) 1305–1342.**

### What does NOT survive — and the literature is blunt about it

- **Harten's own warning**, ICASE 94-59 §7, on the Galerkin/wavelet-projection route for
  nonlinear problems: **[P — exact quote]**
  > "We remark that this Galerkin-type scheme **is not suitable for the 'inviscid' Burgers'
  > equation** … in the sense that **it generates spurious oscillations at shocks, and may
  > even become unstable in some cases** — thus some form of artificial viscosity is needed."

  I.e. **projecting a nonlinear PDE onto a wavelet basis is exactly where multiresolution
  breaks.** Harten's fix was to *not* do that: keep the finite-volume conservation-form
  operator, and use MR only as a **representation + decision layer**. That is the design
  lesson for the spike.

- **The perturbation error.** The MR literature's name for "coarse-level dynamics ≠
  projection of fine-level dynamics" is the **perturbation error** (or *additional error*):
  the difference between the adaptive solution and the **reference scheme** run on the full
  finest grid. It is controlled by the threshold ε, **and it accumulates in time.**
  **Bellotti, Gouarin, Graille & Massot (arXiv:2102.12163, 2022), Prop. 4.1** — additional
  error `≤ C_MR · ε · (n+1)` for a non-expansive reference scheme, and **exponential in n**
  otherwise. **[P]**

- **The control rests on an unproved heuristic.** Same paper, on Assumption 1 ("Harten
  heuristics"): **[P — exact quote]**
  > "Observe that we do not rigorously prove that this assumption holds for our refinement
  > strategy H. As for the Finite Volume scheme, **the Harten's approach to construct H has
  > never proved to satisfy the assumption but is widely used in practice.** The only
  > achievement in terms of reliability condition has been obtained in [Cohen et al.] for
  > Finite Volume with **scalar** conservation laws, with a quite sophisticated refinement
  > strategy."

- **Cohen, Kaber, Müller & Postel**, "Fully adaptive multiresolution finite volume schemes
  for conservation laws", *Math. Comp.* 72 (2003) 183–225: the fully-adaptive scheme (solve
  only on the reduced grid) with an error analysis giving the **optimal threshold ε** from
  the reference scheme's L¹-contractivity, CFL condition and spatial order, so the adaptive
  error has the same slope as the reference error. **[S — full text NOT obtained (AMS/Springer
  paywalled); characterized via Bellotti et al. and search summaries. Flag for follow-up.]**

- Also relevant: **Müller, S., *Adaptive Multiscale Schemes for Conservation Laws*, Lecture
  Notes in Comput. Sci. Eng. 27, Springer, 2003** (the book-length treatment). **[S — not
  obtained.]**

### Precise verdict on nonlinearity

| claim | survives E = K Aᵐ Sⁿ? |
|---|---|
| store is exactly invertible; coarse = area-weighted mean is an **identity** | **YES** — property of the representation only |
| conservation is exact (Σ aᵢhᵢ preserved) under evolution | **YES** — *provided* the scheme stays in conservation/flux form with a single-valued interface flux |
| refluxing step is unnecessary | **YES** — but from leaf-only evolution + single-valued flux, not from the store per se |
| coarse-level solution = projection of fine-level solution | **NO** — this is precisely the perturbation error; Jensen is not defeated, it is *bounded* |
| the bound is rigorous | **NOT in general** — proved only for scalar conservation laws (Cohen et al.); elsewhere it is Harten's heuristic + practice |
| the perturbation error is a one-shot O(ε) | **NO** — it accumulates, ≥ linearly in the number of time steps |

### [∅] A gap that matters for us specifically

The whole MR-for-conservation-laws theory assumes a **local, finite-stencil** flux function,
`f̂ⱼ = f̂(v_{j−K+1}, …, v_{j+K})` (Harten's (7.13c)). Our erosion law's drainage area `A` is
a **non-local accumulation over the upslope network**. I found **nothing** in the MR
literature covering non-local flux arguments, and nothing at all connecting
wavelets/multiresolution to landscape-evolution / stream-power models. The regularity
indicator (detail magnitude) is a *local* statement; `A` is not. This is a real hole in the
transfer, not a detail.

---

## Bibliography (ready for `relata import`)

```bibtex
@article{abgrall-harten:sinum98,
  author = {R. Abgrall and A. Harten},
  title = {Multiresolution Representation in Unstructured Meshes},
  journal = {SIAM J. Numer. Anal.}, volume = 35, number = 6, pages = {2128--2146}, year = 1998}

@techreport{harten:icase94-59,
  author = {A. Harten},
  title = {Multiresolution Representation and Numerical Algorithms: A Brief Review},
  institution = {ICASE / NASA Langley}, number = {ICASE 94-59; NASA CR-194949}, year = 1994}

@article{harten:cpam95,
  author = {A. Harten},
  title = {Multiresolution algorithms for the numerical solution of hyperbolic conservation laws},
  journal = {Comm. Pure Appl. Math.}, volume = 48, pages = {1305--1342}, year = 1995}

@article{harten:sinum96,
  author = {A. Harten},
  title = {Multiresolution representation of data: a general framework},
  journal = {SIAM J. Numer. Anal.}, volume = 33, pages = {1205--1256}, year = 1996}

@article{daubechies-sweldens:jfaa98,
  author = {I. Daubechies and W. Sweldens},
  title = {Factoring wavelet transforms into lifting steps},
  journal = {J. Fourier Anal. Appl.}, volume = 4, number = 3, pages = {247--269}, year = 1998}

@article{sweldens:siam98,
  author = {W. Sweldens},
  title = {The lifting scheme: a construction of second generation wavelets},
  journal = {SIAM J. Math. Anal.}, volume = 29, number = 2, pages = {511--546}, year = 1998}

@inproceedings{schroder-sweldens:sig95,
  author = {P. Schr{\"o}der and W. Sweldens},
  title = {Spherical wavelets: efficiently representing functions on the sphere},
  booktitle = {Computer Graphics Proceedings (SIGGRAPH 95)}, pages = {161--172}, year = 1995}

@article{girardi-sweldens:jfaa97,
  author = {M. Girardi and W. Sweldens},
  title = {A new class of unbalanced {H}aar wavelets that form an unconditional basis for $L_p$ on general measure spaces},
  journal = {J. Fourier Anal. Appl.}, volume = 3, number = 4, pages = {457--474}, year = 1997}

@article{lessig-fiume:tog08,
  author = {C. Lessig and E. Fiume},
  title = {{SOHO}: Orthogonal and symmetric {H}aar wavelets on the sphere},
  journal = {ACM Trans. Graph.}, volume = 27, number = 1, year = 2008}

@article{cohen-kaber-muller-postel:mcom03,
  author = {A. Cohen and S. M. Kaber and S. M{\"u}ller and M. Postel},
  title = {Fully adaptive multiresolution finite volume schemes for conservation laws},
  journal = {Math. Comp.}, volume = 72, number = 241, pages = {183--225}, year = 2003}

@book{muller:book03,
  author = {S. M{\"u}ller},
  title = {Adaptive Multiscale Schemes for Conservation Laws},
  series = {Lect. Notes Comput. Sci. Eng.}, volume = 27, publisher = {Springer}, year = 2003}

@article{roussel-etal:jcp03,
  author = {O. Roussel and K. Schneider and A. Tsigulin and H. Bockhorn},
  title = {A conservative fully adaptive multiresolution algorithm for parabolic {PDE}s},
  journal = {J. Comput. Phys.}, volume = 188, number = 2, pages = {493--523}, year = 2003}

@article{deiterding-etal:sisc16,
  author = {R. Deiterding and M. O. Domingues and S. M. Gomes and K. Schneider},
  title = {Comparison of adaptive multiresolution and adaptive mesh refinement applied to simulations of the compressible {E}uler equations},
  journal = {SIAM J. Sci. Comput.}, year = 2016, note = {arXiv:1603.05211}}

@article{gomes-etal:acm21,
  author = {A. K. F. Gomes and M. O. Domingues and O. Mendes and K. Schneider},
  title = {Adaptive two- and three-dimensional multiresolution computations of resistive magnetohydrodynamics},
  journal = {Adv. Comput. Math.}, year = 2021, note = {arXiv:2102.11806}}

@article{bellotti-etal:22,
  author = {T. Bellotti and L. Gouarin and B. Graille and M. Massot},
  title = {Multiresolution-based mesh adaptation and error control for lattice {B}oltzmann methods with applications to hyperbolic conservation laws},
  journal = {SIAM J. Sci. Comput.}, year = 2022, note = {arXiv:2102.12163}}

@article{duarte-etal:15,
  author = {M. Duarte and Z. Bonaventura and M. Massot and A. Bourdon},
  title = {A numerical strategy to discretize and solve the {P}oisson equation on dynamically adapted multiresolution grids for time-dependent streamer discharge simulations},
  journal = {J. Comput. Phys.}, year = 2015, note = {arXiv:1311.2488}}

@article{chevrot-etal:gji12,
  author = {S. Chevrot and R. Martin and D. Komatitsch},
  title = {Optimized discrete wavelet transforms in the cubed sphere with the lifting scheme---implications for global finite-frequency tomography},
  journal = {Geophys. J. Int.}, volume = 191, number = 3, pages = {1391--1402}, year = 2012}
```

## Sources I could not obtain (flag for follow-up)

Harten CPAM 1995 · Harten SINUM 1996 (the canonical "general framework" paper) ·
Cohen–Kaber–Müller–Postel Math. Comp. 2003 · Sweldens SIAM J. Math. Anal. 1998 ·
Girardi–Sweldens JFAA 1997 (abstract only) · Müller's 2003 book · Roussel et al. JCP 2003.
All paywalled; the substantive claims above are carried by sources I *did* read, and the
places where they are not are marked **[S]**.
