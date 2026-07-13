# Structure-preserving discretisation — literature reconnaissance

*Written 2026-07-13 against the governing principle adopted the same day
(`DECISIONS[preserve-the-structure-declare-the-sacrifice]`) and the structure table in
[`doc/theory/discretisation-and-information.md`](../../doc/theory/discretisation-and-information.md) §4.1.
The brief: **find out whose principle this actually is, what the mature form looks like, and where our
version is naive.***

> **⚠ NOTHING HERE IS DECIDED.** Every recommendation below is `:by claude :status proposed`. The grid
> question remains Joseph's (`DECISIONS[grid-question-not-closed-authority-was-inflated]`), and this
> document does not adjudicate it. What it does is remove the excuse of not knowing what the literature
> says.

**Epistemic tags, used literally and per-claim.**
**[P]** = read-primary, quoted verbatim from a source I obtained and read ·
**[A]** = read-abstract only ·
**[X]** = cross-checked metadata only (Crossref) ·
**[me]** = my inference from what I read — a claim, not a result ·
**[⊘]** = I could not verify; open.

**Every bibliographic record cited here was constructed from its Crossref DOI record and seeded into
`relata`** (25 entries, `bib-fields` verified, 13 with the OA PDF I actually read registered). Bibkeys
given inline. **No citation in this document was written from memory.** (One was nearly: I fetched
arXiv:1112.4767 believing it was Thuburn & Cotter's mimetic-framework paper; it is a quantum-optics
paper about nitrogen-vacancy centres in diamond. Caught by checking the title line. That is the
failure mode this project has been burned by, and it is one careless `curl` away at all times.)

---

## 0. The six findings, before the detail

1. **The principle is real, mature, and named.** Its umbrella terms are **"compatible discretisation"**,
   **"mimetic"**, and **"structure-preserving"**; the ODE/time half is **"geometric numerical
   integration"**. We are rediscovering a field, not inventing one. **Adopt the vocabulary; do not coin.**

2. **The Prime Question is also named.** *"What is this scheme actually solving, as opposed to what it
   claims to solve"* is **modified equation analysis** (Warming & Hyett 1974) on the PDE side and
   **backward error analysis** on the ODE side. This is not an analogy — it is the same question, and it
   has an answer-shaped machinery attached. **Adopt.**

3. **`curl∘grad ≡ 0` and `div∘curl ≡ 0` are FREE — exactly, at machine precision, on ANY mesh,
   including a sphere, including a non-conforming seam.** They are *combinatorial* identities
   (`d∘d = 0` because the discrete `d` **is** the coboundary operator and `∂∘∂ = 0`), and carry no
   geometry whatsoever. **[P]** All the difficulty — all of it — lives in the **Hodge star**, which is
   where the metric enters. This carve is the single most useful thing in this document: *the topological
   identities cost nothing; the accuracy costs everything.* Our structure table's "potential" row is
   therefore much cheaper to satisfy than we feared, and much less informative than we hoped.

4. **The crux question — mimetic + AMR at a non-conforming coarse↔fine interface — is ANSWERED, and it
   answers in our favour.** The literature thins out *in geophysics*, but it is thick in **MHD**:
   Balsara (2001) preserves `div B = 0` **exactly** across a subcycled AMR hierarchy, and the recipe is
   *precisely* what `DECISIONS[flux-on-the-face-makes-refluxing-an-invariant]` proposes — face-staggered
   storage, **area-weighted restriction** (coarse face = sum of its children's faces), and a
   **flux-register-shaped correction** at the seam. Separately, the mimetic-FD community handles hanging
   nodes by the move that dissolves the problem: **a cell with a hanging node is not a defective quad, it
   is a pentagon**, and a mimetic scheme on polygons does not care. **[P]**

5. **The intersection — structure-preserving × spherical × dynamically adaptive — is NOT empty. It has
   exactly one occupant, and it is uncomfortably close to us: WAVETRISK** (Kevlahan & Dubos), which is
   TRiSK (mimetic C-grid) + sphere + **second-generation-wavelet multiresolution adaptivity**. It is
   ~10 years old, in production, open, and published in GMD. **[P]** It also hands us the general
   theorem we were missing: **the restriction operators must COMMUTE with the discrete differential
   operators**, and *that* is what makes a mimetic property survive adaptivity. We independently derived
   one of their three commutation relations. They have three. **The third one — the one we never thought
   of — is the one that prevents spurious vorticity.**

6. **Our grid choice has a real, named cost, and it is not the one we have been auditing.** The
   equiangular cube-sphere is **non-orthogonal**, and the mainstream mimetic C-grid construction (TRiSK)
   *requires* primal-dual orthogonality — it explicitly admits the **conformal** cubed sphere, not the
   equiangular one. **[P]** But the news is far better than that sounds: on the *degree-of-freedom*
   criterion that actually determines whether spurious modes exist, the **quadrilateral C-grid is the
   best of the three** (2:1 velocity:mass — the ideal ratio; hexagons are 3:1 and carry spurious Rossby
   modes; triangles are 3:2 and carry spurious inertia-gravity modes). **[P]** **The reason nobody runs
   a cube-sphere C-grid is orthogonality and corner grid-imprinting — not the DOF ratio.** And the
   escape hatch from the orthogonality requirement already exists and is named: **finite element exterior
   calculus**, which Cotter & Thuburn explicitly recommend applying as **`RT0` on quadrilaterals on a
   cubed-sphere mesh**. **[P]**

7. **⚖ AND THE FINDING THAT REORGANISES ALL THE OTHERS — WHICH IS OUR OWN JENSEN THEOREM, ARRIVING AT
   THE SEAM.** Structures do **not** all cross a coarse↔fine interface on the same terms:

   > ### **LINEAR structures cross the AMR seam for free, via a commuting restriction operator.**
   > ### **NONLINEAR structures DO NOT — because PROJECTION AND NONLINEARITY DO NOT COMMUTE.**

   Mass, `div B`, circulation: **linear** in the state, and a commuting restriction (§3.3) carries them
   across exactly. **The second law is not linear** — and the standard DG mortar at a non-conforming
   interface is provably **not** entropy stable and **empirically crashes** (Friedrich et al. 2018, §8.2).
   *Project-then-square ≠ square-then-project.* **That is exactly `discretisation-and-information.md`
   §3.3 — "Jensen's inequality IS aliasing", "coarse-graining a nonlinear law does not blur, IT LIES" —
   restated at the seam instead of at the cell.** We derived the theorem. We did not notice it applies to
   the seam. **It does, and it is the reason the AMR + entropy-stability intersection is the hard one.**

**The one-sentence version:** *almost everything in our structure table is adopt-not-derive; the seam is
adopt-from-MHD-not-derive **for the linear structures and a genuine research problem for the nonlinear
ones**; the sphere is where we pay; and the only thing genuinely nobody has built is the combination WE
happen to need — structure-preserving on a **cube-sphere quadtree** — for which every individual
ingredient exists and has been proven separately.*

---

## 1. The vocabulary — it is named; adopt it

### 1.1 The governing principle's own name

| our phrasing | the field's name | canonical entry point |
|---|---|---|
| "preserve the structure the physics has" | **compatible discretisation** / **mimetic** / **structure-preserving** | `arnold-2006-feec` (Acta Numerica), `arnold-2010-feec-hodge` (Bull. AMS) |
| the same, for time integration of ODEs | **geometric numerical integration** | `hairer-2003-geometric` (Acta Numerica 12:399–450) |
| "conservation laws, potentials, topological identities…" | the **de Rham complex** and its discrete analogue | FEEC / DEC / MFD |

**[A]** *"Compatible discretizations transform partial differential equations to discrete algebraic
problems that mimic fundamental properties of the continuum equations."* — the standard formulation
(Bochev & Hyman, *Principles of Mimetic Discretizations of Differential Operators*, in the IMA volume
*Compatible Spatial Discretizations*). **We should stop saying "structure-preserving" as if we coined it
and start citing the complex.**

### 1.2 ⚖ THE PRIME QUESTION IS MODIFIED EQUATION ANALYSIS

> **Warming, R.F. & Hyett, B.J. (1974).** *The modified equation approach to the stability and accuracy
> analysis of finite-difference methods.* J. Comput. Phys. 14(2):159–179.
> `relata: warming-1974-modified-equation` **[X]**

The method: take the discrete scheme, Taylor-expand it, and **eliminate the time derivatives** to obtain
the **PDE the scheme is actually solving exactly** (to whatever order you carry). The leading truncation
term is then read off as a *physical* term — an even-order term is **numerical diffusion**, an odd-order
term is **numerical dispersion**. That is, literally and formally, ***"what physical claim is this
algorithm actually making?"***

Its ODE-side twin is **backward error analysis**: the numerical trajectory is the *exact* trajectory of a
*modified* (perturbed) system, and for a symplectic integrator the modified system is *itself Hamiltonian*
— which is the entire reason symplectic integrators do not drift in energy. `hairer-2003-geometric`. **[X]**

**Consequence for us, and it is a real one.** Our Prime Question currently gets answered by *reading* an
algorithm and *arguing* about its claim. Modified equation analysis answers it **by computation**, and
returns a *term* — with a sign, a magnitude, and a differential order. **Our θ flux-smoothing in
`water.rs` has a modified equation, and it will tell us exactly what unphysical term we added.** That is
the probe `discretisation-and-information.md` §6 wants and did not know how to ask for.

**⇒ Proposed: adopt "modified equation" and "backward error analysis" as the standard terms in
`LEXICON.udon`; keep "the Prime Question" as the *pedagogical* framing (it is better teaching), but stop
implying we invented the question.**

---

## 2. The structure table, mapped onto the literature

This is `discretisation-and-information.md` §4.1, with the literature filled in and — the part the brief
actually asked for — **what it guarantees exactly vs approximately, whether it works on a SPHERE, whether
it composes with AMR, and what it costs.**

| structure | the method | guarantee | sphere? | AMR? | cost |
|---|---|---|---|---|---|
| **local conservation** | finite volume / flux form | **exact** (to round-off) | ✅ free | ✅ **Berger–Colella refluxing** | ~free |
| **topological identity** (`∇×∇φ≡0`, `∇·∇×A≡0`) | DEC / FEEC / MFD | **exact, combinatorially** | ✅ **free, any mesh** | ✅ **free (it's a polygon)** | **zero** |
| **the Hodge star / accuracy** of the above | DEC diagonal star; FEEC mass matrix | ⚠ **approximate**, and this is where the sphere bites | ⚠ wants well-centred / orthogonal dual | ⚠ | the real cost |
| **vorticity / circulation** | C-grid + TRiSK Coriolis | stationary geostrophic modes; **energy exact**, **PV exact**, **enstrophy NOT** | ✅ but **needs orthogonality** | ✅ **WAVETRISK** | modest |
| **energy + potential enstrophy jointly** | Arakawa & Lamb 1981 | both — see §4.2 caveats | ✅ (lat-lon origin) | ⊘ | modest |
| **symplectic** (Hamiltonian) | symplectic integrators | **exponentially long** energy bound — **but only at FIXED step** | n/a | ⚠ **conflicts with adaptivity** | ~free at fixed h |
| **the second law** | entropy-stable (Tadmor lineage) | a **discrete entropy inequality** — necessary, *not* sufficient for uniqueness | ✅ (on curved meshes: **needs the GCL**) | ⊘ | 2nd flux eval + dissipation |
| **realizability** (`h ≥ 0`) | Zhang–Shu scaling limiter | **exact**, and **provably accuracy-preserving** | ✅ | ✅ | **a reduced CFL** |
| **equilibria** (lake at rest) | well-balanced / hydrostatic reconstruction | exact for **lake-at-rest**; **moving equilibria are much harder** | ✅ (**needs GCL** on curved) | ✅ (done in practice) | small |
| **rotational symmetry** | isotropic stencils; edge flux, not fans | — | — | — | — |
| **timescale hierarchy** | multirate / AMR subcycling | — (already ours: `multiscale-seams.md`) | — | ✅ | — |
| **⚠ MULTIRESOLUTION CONSISTENCY** | **the commutation relations (§3.3)** | **this row was MISSING from our table** | ✅ | ✅ | — |

**Read the last row.** It is the row our own table did not have, and it is the one that makes every other
row *survive* adaptivity. §3.3.

---

## 3. THE CRUX — structure-preserving across a non-conforming seam

*This is the question the brief flagged as most needed, and the suspicion was that the literature thins
out here. It does thin out — but only in geophysics. Three separate communities have solved it, by three
routes that turn out to be the same route.*

### 3.1 The MHD route — Balsara 2001, and it is our proposal, already built

> **Balsara, D. S. (2001).** *Divergence-Free Adaptive Mesh Refinement for Magnetohydrodynamics.*
> J. Comput. Phys. 174(2):614–648. `relata: balsara-2001-divfree` — **READ PRIMARY**, PDF registered.

MHD must preserve `∇·B = 0` — a **topological identity across a subcycled, non-conforming AMR
hierarchy**. Structurally identical to our seam. The paper's design argument is, almost word for word,
`DECISIONS[flux-on-the-face-makes-refluxing-an-invariant]`:

> **[P]** *"This can be done via a 'staggered mesh magnetic field transport algorithm' where the magnetic
> field components are collocated at the **face-centers** of each zone and the electric field components
> are collocated at the **edge-centers** of the zones. Stoke's law is then applied … Because it follows
> from a discrete version of Stoke's law, the resulting discrete time-update strategy clearly shows that
> **if the magnetic field is divergence-free at the beginning of a time step, it will remain so at the end
> of the time step.**"* (§I)

And here is the sentence that indicts a **collocated** store *specifically for AMR* — this is the
diagnosis we wrote ourselves, from the other side:

> **[P]** *"The issue of control volumes also plays a role in AMR. The reason is that **when the grid
> lines, the lines of all control volumes and the lines that delineate refinement patches in AMR are all
> aligned with each other one can enforce several constraints, including the divergence-free one, quite
> trivially. That is not the case when the control volumes on which the constraint is enforced are not
> aligned with the zones of the mesh.**"* (§III)

**The three operators, and what each guarantees:**

| operator | what it is | guarantee |
|---|---|---|
| **restriction** (fine→coarse) | **[P]** *"making an **area weighted average** of the [field] component that is collocated on the faces of the fine mesh and assigning it to the corresponding face of the coarse mesh"* (§IV) | **exact** — this is *"the coarse face IS the sum of its children"*, verbatim from our own proposal |
| **prolongation** (coarse→fine) | **[P]** *"far more intricate and interesting"* — a naive piecewise-linear face profile has **3 DOF but must match 4 fine faces**, which *"is impossible … in an integral sense"*; so the reconstruction needs an extra **bilinear (`yz`) term** (Eq. 4.1) | div-free **by construction**, but **you must add the cross term** |
| **the seam correction** | the **"electric field correction step"** — replace the coarse edge-EMF with the space-and-time average of the fine edge-EMFs | restores the identity **exactly** |

> **[P]** *"Thus the problem is **entirely analogous to the flux correction problem** that arises in the
> AMR processing of the Euler equations as explained in **Berger and Colella**… Just as the flux
> correction step in Berger and Colella restores the consistency of the **fluxes** at the interface
> between fine and coarse meshes, the electric field correction step described here restores the
> consistency of the **electric fields** at the interface … **the flux correction step ensures
> conservative evolution** … and, in a completely analogous fashion, **the electric field correction step
> ensures divergence-free evolution of the magnetic fields in an AMR hierarchy.**"* (§V)

And the payoff, which is exactly what our seam needs:

> **[P]** *"the time-step can be **sub-cycled on finer meshes without loss of the divergence-free
> property** … each mesh in the AMR hierarchy evolves with the **full Courant number**"* (§VII, conclusion 8)

**⇒ Verdict.** `flux-on-the-face-makes-refluxing-an-invariant` is **correct, and it is 25 years old.**
The two corrections our proposal needs:

- **(a) It is not free.** We wrote *"refluxing stops being an ALGORITHM and becomes an INVARIANT."* That
  is **true for restriction and false for prolongation and for the seam correction.** Restriction is the
  identity (area-weighted sum). **Prolongation still needs a constrained reconstruction with an extra
  cross-derivative DOF, and a subcycled seam still needs an explicit correction step.** The seam does not
  *"never happen"* — it *becomes provable*. That is worth a great deal and it is not the same claim.
- **(b) Balsara's restriction is *area-weighted*, not *mean*.** On our cube-sphere, child areas differ.
  Mean-pinning is already the wrong operator by this criterion, independently of the wavelet argument.

### 3.2 The mimetic-FD route — the hanging node is a category error

> **Lipnikov, K., Morel, J. & Shashkov, M. (2004).** *Mimetic finite difference methods for diffusion
> equations on **non-orthogonal non-conformal** meshes.* J. Comput. Phys. 199(2):589–597.
> `relata: lipnikov-2004-nonconformal` — **READ PRIMARY**, PDF registered.

Two findings, and the first one is a **warning aimed straight at us**:

> **[P]** *"Even if the mesh is orthogonal and the diffusion coefficient is continuous, **one cannot
> accurately compute the flux across the interface between cells i and j in terms of a difference between
> the intensities φᵢ and φⱼ. The intensity φₖ must be included in the difference expression to obtain a
> convergent diffusion discretization.**"* (§1, of the three-cell hanging-node interface)

**⇒ The two-point flux is NOT CONVERGENT at a coarse–fine interface — even on an orthogonal mesh.** This
is the *same defect* the grid report measured on our non-orthogonal mesh (*"a two-point flux is
inconsistent … O(1) error that GROWS under refinement"*), arriving from a completely independent
direction. **Our seam has two independent reasons to be broken, and we had only found one.** The
`seam_ridge` probe's 2.45×→5.79× may not be a refluxing problem at all — or not *only* one.

And the second finding is the dissolution:

> **[P]** *"The local SO method is **particularly amenable to hanging-node meshes** because the
> discretization process occurs in two steps. The first step is to consider each cell in the mesh as an
> independent domain … **Since each cell in a hanging-node mesh is a quadrilateral, the first step … is
> identical to that for standard quadrilateral meshes.**"* (§1)

Corroborated across the MFD literature generally: *because arbitrary polyhedra are naturally permitted,
**the added nodes are not regarded as hanging nodes at all**, and any level of non-conformity can be
implemented without modifying the scheme.* **[A]**

> ### **The hanging node is not a defect of the mesh. It is a coarse cell that has five edges and is
> being mis-declared as having four.**

This is the *exact shape* of the MFD lesson `discretisation-and-information.md` §0 already learned once —
*"the corner was never the problem; the fan was"* — applied to the seam. **A cell's edge count is data,
not a constant.** Our `CellId` quadtree can carry a 5- or 6-edge coarse cell at a refinement boundary with
no change to the key.

*(Note the standard constraint they adopt and we will need: **[P]** "We do not allow any cell to share a
face with more than two cells … a restriction … usually seen in quadtree-based adaptive-mesh algorithms."
That is the **2:1 balance** rule. It is cheap and it is not optional.)*

### 3.3 ⚖ The general theorem — THE COMMUTATION RELATIONS

*This is the most important single thing in this document, and it is the row missing from our structure table.*

> **Kevlahan, N. K.-R. & Dubos, T. (2019).** *WAVETRISK-1.0: an adaptive wavelet hydrostatic dynamical
> core.* Geosci. Model Dev. 12:4901–4921. `relata: kevlahan-2019-wavetrisk` — **READ PRIMARY** (OA), PDF registered.

> **[P]** *"**Mimetic properties (e.g. mass conservation) should be preserved by the adaptivity and the
> discretizations should be unchanged.** The building blocks of the method are one-scale operators (in
> this case, the TRiSK discretization) and two-scale operators between a fine scale j+1 and a coarse
> scale j. **To conserve the mimetic properties of the TRiSK scheme, the restriction operators … and the
> discrete differential operators (div, grad, curl) must satisfy the following commutation properties:**"*
> (§2.1)
>
> $$\mathrm{curl}^j \circ R_u = R_\zeta \circ \mathrm{curl}^{j+1} \qquad \textbf{conserves circulation}$$
> $$R_\mu^j \circ \mathrm{div}^{j+1} = \mathrm{div}^j \circ R_F \qquad \textbf{conserves mass}$$
> $$\mathrm{grad}^j \circ R_B = R_u^j \circ \mathrm{grad}^{j+1} \qquad \textbf{no spurious vorticity}$$
>
> **[P]** *"**The commutation relations (Eqs. 1–3) ensure that the mimetic properties of the
> discretization are also satisfied on the adapted grid.**"* (§2.1)

**Read that as the general law, because it is one.**

> ### **A structure survives multiresolution IFF the restriction operator COMMUTES with the operator that expresses the structure.**

Everything in §3.1 and §3.2 is a *special case*:

- Balsara's area-weighted face restriction + electric-field correction **is** the commutation relation for
  `curl` (Stokes).
- Berger–Colella refluxing **is** the commutation relation for `div` (mass) — relation (2), *exactly*.
- **`DECISIONS[flux-on-the-face-makes-refluxing-an-invariant]` is relation (2), independently derived.**
  We got one of three.

**⚠ And the one we did not have is relation (3), and it is the one that would bite us.** *"no spurious
vorticity"* — **[P]** *"The third commutation relation ensures that a flow with uniform potential
vorticity remains uniform under the advection by an arbitrary velocity field."* We have never asked
whether our coarse↔fine operator preserves a uniform field's uniformity under advection. **That is a
cheap probe and we do not have it.**

**⇒ Proposed for the structure table: a new row — *"multiresolution consistency: restriction must commute
with the operator." Preserved exactly by: choosing R to satisfy the commutation relation. Broken if you
ignore it: every other structure in the table silently stops holding on the adapted grid.***

### 3.4 ⚠ …BUT THE COMMUTATION LAW ONLY COVERS THE **LINEAR** STRUCTURES

*This is the correction that keeps §3.3 from being an overclaim, and it is the deepest thing in this
document.*

> **Friedrich, L., Winters, A. R., Del Rey Fernández, D. C., Gassner, G. J., Parsani, M. & Carpenter, M. H.
> (2018).** *An Entropy Stable h/p Non-Conforming Discontinuous Galerkin Method with the Summation-by-Parts
> Property.* J. Sci. Comput. 77(2):689–725. `relata: friedrich-2018-entropy-nonconforming` — **READ PRIMARY**
> (arXiv:1712.10234).

> **[P]** *"Special attention is given to the coupling between non-conforming elements as we demonstrate
> that **the standard mortar approach for DG methods does not guarantee entropy stability for non-linear
> problems, which can lead to instabilities.**"* (Abstract)

And the mechanism is stated in a way that should stop us cold, because **we already own this theorem**:

> **[P]** *"For **linear** problems, where entropy conservation becomes energy conservation, it is known
> that the mortar method **is sufficient** … because no non-linearities are present … However, for
> **non-linear** problems we replace this simple central numerical flux with a more complicated entropy
> conservative numerical flux that features possible polynomial or rational non-linearities as well as
> strong cross coupling … **it is not obvious how the operation to compute the square of the projection of
> U_L and then L² project the numerical flux back to the left element will change the entropy.**"* (§1.3)

And it is not a theoretical worry — it kills the run:

> **[P]** *"for the mortar method we observe an **unpredictable behavior of the entropy** for t < 1 and note
> that at **t ≈ 1 the approach even crashes**. This has been verified for the CFL numbers CFL = 0.5; 0.25;
> 0.125; 0.0625."* (§3)

> ### **⇒ THE GENERAL LAW, CORRECTED — AND IT IS JENSEN.**
>
> **"Compute the square of the projection … then project back"** *is* $f(\bar S)$ **versus**
> $\overline{f(S)}$. It is `discretisation-and-information.md` §3.3, **verbatim, at the seam**:
> *"Coarse-graining a nonlinear law does not lose precision. IT LIES."*
>
> | structure | linear in the state? | crosses the seam via a commuting R? |
> |---|---|---|
> | mass conservation | ✅ | ✅ **free** (Berger–Colella; WAVETRISK Eq. 2) |
> | `div B = 0` / topological identity | ✅ | ✅ **free** (Balsara) |
> | circulation | ✅ | ✅ **free** (WAVETRISK Eq. 1) |
> | well-balancedness (lake-at-rest) | ✅ *(a linear cancellation)* | ✅ **proved on a quadtree** — §8.3 |
> | positivity | ✅ *(a convex constraint)* | ✅ **proved on a quadtree** — §8.1 |
> | **the second law / entropy** | ❌ **NONLINEAR** | ❌ **NOT free. The naive seam CRASHES.** |
>
> **A structure proved on a conforming mesh is NOT inherited by its adaptive version. It must be
> RE-PROVED at the seam — and for the nonlinear ones, the naive seam is wrong.**

**The fix exists** and is the thing to import if we ever want entropy stability *and* AMR: carefully
constructed **mortar projection operators** (Friedrich et al. 2018; extended to quads/hexes by
`chan-2021-mortar-entropy`, Chan, Del Rey Fernández & Carpenter, J. Sci. Comput. 2021). **[A]**

**⇒ This is the single most load-bearing thing to carry into the seam design.** Our `flux-on-the-face`
proposal is correct **and is a claim about the linear structures only.** It buys mass. It does **not** buy
the second law, and if we ever put an entropy-stable kernel on an adaptive grid without importing the
mortar work, **it will not degrade — it will crash, and it will crash at a CFL number that looks safe.**

---

## 4. The sphere, and what our grid actually costs

### 4.1 The good news first: the quadrilateral C-grid has the BEST degree-of-freedom ratio

The real diagnosis of which staggered grids carry spurious computational modes is the **ratio of velocity
DOFs to mass/pressure DOFs**, and it is stated plainly by the people who built the alternatives:

> **Cotter, C. J. & Thuburn, J. (2014).** *A finite element exterior calculus framework for the rotating
> shallow-water equations.* J. Comput. Phys. 257:1506–1526. `relata: cotter-2014-feec-swe` — **READ PRIMARY**
> (arXiv:1207.3336), PDF registered.

> **[P]** *"…it is now well understood that **the triangular C-grid supports spurious inertia-gravity mode
> branches because of the decreased ratio of velocity degrees of freedom (DOFs) to pressure DOFs relative
> to quadrilaterals (from 2:1 to 3:2)** … **The hexagonal C-grid has an increased ratio of velocity DOFs to
> pressure DOFs (from 2:1 to 3:1), and so does not support spurious inertia-gravity mode branches, but does
> have a branch of spurious Rossby modes.**"* (§1)

| C-grid on | velocity : mass DOF | spurious modes |
|---|---|---|
| **quadrilaterals** | **2 : 1** | **none** — this is the reference ratio |
| hexagons (MPAS) | 3 : 1 | spurious **Rossby** modes |
| triangles (ICON) | 3 : 2 | spurious **inertia-gravity** modes |

**So the answer to "why did MPAS choose hexagons" is NOT "because hexagons are better."** It is: *given
that you have committed to a quasi-uniform unstructured icosahedral mesh with an orthogonal dual (SCVT),
hexagons are the choice that avoids the inertia-gravity disease that kills triangles* — and you then pay
with a spurious Rossby branch, which is argued to be tolerable:

> **[P]** *"There is an argument to be made that spurious Rossby mode branches arising from increasing
> velocity DOFs relative to this ratio are **not harmful since they have very low frequencies and will
> just be passively advected by the flow**."* (§ Discussion)

*(And they are controlled in practice by upwind-biasing the PV advection — Weller 2012, `weller-2012-modes`. **[A]**)*

**⇒ On the DOF criterion — the criterion that determines whether spurious computational modes exist at
all — a cube-sphere C-grid is the best of the three. That is a genuinely encouraging finding and it was
not what I expected to find.**

### 4.2 The bad news: TRiSK needs ORTHOGONALITY, and we do not have it

> **Thuburn, J., Ringler, T. D., Skamarock, W. C. & Klemp, J. B. (2009).** *Numerical representation of
> geostrophic modes on arbitrarily structured C-grids.* J. Comput. Phys. 228(22):8321–8335.
> `relata: thuburn-2009-geostrophic` — **READ PRIMARY**, PDF registered.

The scheme exists to fix a specific, named failure — and note it is a failure of the **naive** C-grid
Coriolis, not of collocation:

> **[P]** *"…with the most obvious discretization of the Coriolis terms, **geostrophic modes, which should
> have zero frequency, in fact have non-zero frequencies, with the largest being of the same order as the
> Coriolis parameter f**."* (§1)

And here is the constraint:

> **[P]** *"**Our procedure is applicable to grids having the property that dual edges are orthogonal to
> primal edges.** The allowed grids include arbitrary Delaunay triangulations and Voronoi diagrams, as well
> as quadrilateral grids based on **orthogonal coordinate systems such as longitude–latitude and
> ⟨**conformal**⟩ cubed sphere**."* (§1)

**Vivarium's grid is the EQUIANGULAR cube-sphere, not the conformal one, and we have measured it to be
non-orthogonal.** So TRiSK-as-published does not apply to us. And the trade is *stated*, which means it
is a known frontier and not our private problem:

> **[P]** *"Two directions remain outstanding from this approach, namely **the relaxation of the
> orthogonality requirement which constrains cubed sphere grids so that grid resolution increases much
> more quickly in the corners than at the middle of the faces**, and the construction of higher-order
> operators to avoid **grid imprinting**."* (Cotter & Thuburn §1)

> ### **That sentence is the cube-sphere's whole dilemma, and we are already living on one horn of it.**
> **Orthogonal (conformal) ⇒ mimetic C-grid works, but cell size clusters badly at the corners.**
> **Equiangular ⇒ quasi-uniform cells, but non-orthogonal, so the standard mimetic C-grid is unavailable.**
> We picked equiangular. **[me] That was, unknowingly, a choice against the off-the-shelf mimetic
> machinery** — and it is a *live* instance of the governing principle: we sacrificed a structure
> silently.

### 4.3 The escape hatch, and it is named: FEEC

Cotter & Thuburn's *entire point* is that **finite element exterior calculus removes the orthogonality
requirement** while keeping the mimetic properties — and their recommended configuration is, of all
things, ours:

> **[P]** *"…which suggests the BDFM1 space on triangles with an icosahedral mesh in the primal
> formulation **or RT0 on quadrilaterals with a cubed sphere mesh in the primal-dual formulation**."*
> (§ Discussion)

**⚠ And there is a trap sitting directly under that recommendation, which we must not walk into:**

> **Arnold, D. N., Boffi, D. & Falk, R. S. (2005).** *Quadrilateral H(div) finite elements.* SIAM J. Numer.
> Anal. 42(6):2429–2451. `relata: arnold-2005-quadhdiv` — **READ PRIMARY**, PDF registered.

> **[P]** *"…while the Raviart–Thomas space of index r achieves order r+1 approximation in L² for
> quadrilateral meshes as for rectangular meshes, **the order of approximation of the divergence is only of
> order r in the quadrilateral case** (but of order r+1 for rectangular meshes). **Thus, in the case r = 0,
> there is no convergence in H(div,Ω).**"* (§1)
>
> **[P]** *"The example is far from pathological. The domain is simply a square, **the mesh sequence does
> not degenerate in any sense** — in fact all the elements of all the meshes in the sequence are similar to
> a single right trapezoid — and the function u is a polynomial."* (§1)

**⇒ `RT0` — the lowest-order H(div) element, which IS "one flux per face" — has NO convergence of the
divergence on general (non-affine) quadrilaterals.** Cube-sphere cells are non-affine quads. **[me] This
is, I strongly suspect, the same animal as the grid report's measured *"two-point flux is inconsistent on
a non-orthogonal mesh; O(1) error that GROWS under refinement (order −0.5 on every quad grid)."* If so,
our measurement has a 2005 theorem behind it, and the theorem says it is not a bug in our harness.**
⚠ **[⊘] I have NOT proven the correspondence** — ABF's statement is about approximating `div u` in L²
under a Piola-mapped FE basis, and ours is about a two-point FV flux. They are not obviously the same
statement. **This is the single highest-value thing in this document to check, and it is checkable on
paper.**

**And the fix exists, and it is pointed at from Cotter & Thuburn's own reference list:**

> **Bochev, P. B. & Ridzal, D. (2008/2009).** *Rehabilitation of the Lowest-Order Raviart–Thomas Element on
> Quadrilateral Grids.* SIAM J. Numer. Anal. 47(1):487–507. `relata: bochev-2009-rehabilitation` **[A]**
>
> **[A]** *"A recent study reveals that convergence of finite element methods using H(div,Ω)-compatible
> finite element spaces **deteriorates on nonaffine quadrilateral grids** … The paper proposes a
> reformulation of finite element methods, **based on the natural mimetic divergence operator**, which
> **restores the order of convergence**."*

**[me] And note what "the natural mimetic divergence operator" IS: it is the finite-volume divergence —
sum of face fluxes over cell volume. Which means a face-flux FV scheme on a cube-sphere quadtree may
ALREADY BE the rehabilitated form, and the ABF defect may be an artifact of the FEM route rather than an
indictment of ours.** ⚠ **This is an inference, not a result. It is the second-highest-value thing to
check, and if it holds it is very good news.**

### 4.4 The curved-mesh tax nobody warned us about: the metric identities (GCL)

> **Wintermeyer, N., Winters, A. R., Gassner, G. J. & Kopriva, D. A.** *An Entropy Stable Nodal
> Discontinuous Galerkin Method for the Two Dimensional Shallow Water Equations on Unstructured Curvilinear
> Meshes with Discontinuous Bathymetry.* (arXiv:1509.07096) — **READ PRIMARY**.

> **[P]** *"The scheme presented here is also well-balanced, **an attribute difficult to guarantee in
> curvilinear coordinates**. We find that **the numerical satisfaction of the metric identities (referred to
> as the geometric conservation law) is critical to prove that the baseline scheme remains entropy
> conservative and well-balanced on arbitrary meshes.**"* (§1)

> ### **On a curved mesh, it is not enough for the metrics to be TRUE. They must satisfy a DISCRETE IDENTITY.**

**⇒ This directly qualifies the grid report's remedy.** *"Supply the true metrics (real distances, real
areas)"* is listed in `discretisation-and-information.md` §1 as *"necessary, usually insufficient."* This
source says something sharper and more actionable: **the discretely-computed metric terms must satisfy the
GCL exactly**, or well-balancedness and entropy conservation are **lost as theorems**, not merely degraded.
A cube-sphere is a curved mapped mesh. **We have never checked a discrete GCL. [⊘] I do not know whether
ours holds.** That is a probe, and it is cheap.

### 4.5 What is actually done on a cubed sphere with AMR today

> **Ferguson, J. O., Jablonowski, C., Johansen, H., McCorquodale, P., Colella, P. & Ullrich, P. A. (2016).**
> *Analyzing the Adaptive Mesh Refinement (AMR) Characteristics of a High-Order 2D Cubed-Sphere
> Shallow-Water Model.* Mon. Wea. Rev. 144(12):4641–4666. `relata: ferguson-2016-amr-cubed` — **READ PRIMARY**, PDF registered.

**This is the closest published system to vivarium's substrate, and it is worth knowing exactly what it
does and does not claim.**

- **[P]** *"an **unstaggered** high-order finite-volume (FV) multiblock approach"* on an **equiangular
  cubed-sphere** grid with **nonconforming** block-structured AMR and **space-and-time** refinement
  (subcycling at constant Courant number).
- **[P]** *"The numerical scheme is **mass conserving to machine precision** and **energy conserving up to
  the temporal truncation order**, when used without limiters or explicit dissipation."*
- **[P]** Seam handling: *"Corrections are applied to the fluxes at coarse–fine interfaces to ensure
  conservation **(Berger and Colella 1989)**."*
- **[P]** *"The adaptive grids are able to track features of interest reliably **without inducing noise or
  visible distortions at the coarse–fine interfaces**."*
- **[P]** ⚠ *"The central difference operators used to obtain the fluxes are **smoothed by an explicitly
  added sixth-order diffusive operator**."*

**⇒ Read that last bullet against `discretisation-and-information.md` §2.5.** A production cubed-sphere
dycore, collocated, carries an **explicit artificial-diffusion stabilisation term** — an `α_stab`, exactly
as Cardiff predicts, exactly as our θ-smoothing is. **The collocated grid's blindness is not our bug; it is
a property of collocation, and everyone who chooses collocation pays this tax.** They just wrote it down.

**And note precisely what they claim on the structure table:** conservation **exact**; energy
**approximate**; **vorticity / PV / geostrophic balance: not claimed at all.** They ran a geostrophic-flow
test and reported errors — they did not claim a preserved structure. **[me] For advection and gravity
waves that is fine. For ocean gyres and persistent circulation bands — which is what our TODO queues — it
is exactly the regime where the unclaimed structure is the one that matters.**

---

## 5. THE META-QUESTION — is the intersection empty?

**No. It has one occupant, and the occupant is excellent.**

### 5.1 WAVETRISK — structure-preserving × spherical × dynamically adaptive, in production

| | |
|---|---|
| `dubos-2013-staggered-wavelet` | Dubos & Kevlahan, *A conservative adaptive wavelet method for the shallow-water equations on staggered grids*, QJRMS 139:1997–2020 (2013) — the β-plane prototype. **[X]** |
| `aechtner-2015-wavelet-sphere` | Aechtner, Kevlahan & Dubos, *…on the sphere*, QJRMS 141:1712–1726 (2015). **READ PRIMARY** (arXiv:1404.0405). |
| `kevlahan-2019-wavetrisk` | Kevlahan & Dubos, *WAVETRISK-1.0: an adaptive wavelet hydrostatic dynamical core*, GMD 12:4901–4921. **READ PRIMARY** (OA). |
| `kevlahan-2022-wavetrisk-ocean` | Kevlahan & Lemarié, *wavetrisk-2.1: an adaptive dynamical core for ocean modelling*, GMD 15:6521–6539. **READ PRIMARY** (OA). |

**What it is:** TRiSK (the mimetic hexagonal C-grid) + **second-generation wavelets** + dynamic adaptivity,
on the sphere, MPI-parallel, with the mimetic properties **proved to survive the adaptivity** via the
commutation relations of §3.3. And — note this — **[P]** its grid is stored as *"a hybrid data structure of
**dyadic quad trees** and patches."* **It is a quadtree.**

**It even has an ocean variant** (`wavetrisk-2.1`) with **[P]** *"Kelvin's circulation theorem, advection of
tracers, conservation of Casimir invariants"* preserved — i.e. the **vorticity row of our structure table,
on an adaptive sphere, in an ocean model, shipping.** That is precisely what our TODO queues as "ocean
gyres."

**The costs, stated by the authors — these are the numbers to plan against:**

- **[P]** *"the overhead due to the multiscale adaptivity is about a factor of 3. We therefore conclude
  that **wavetrisk is about 4 times slower per active grid point** than dynamico [the non-adaptive twin],
  and therefore **a grid compression factor greater than 4 is required** for wavetrisk to be faster."*
- **[P]** *"the **cost per active node is independent of the grid compression ratio**"* — the adaptivity
  overhead is a constant factor, not a scaling penalty. **This is the single most reassuring number in the
  document for our architecture.**
- **[P]** achieved compression: *"4 to 200 or more … depending on the flow"*, up to ~1000× in intermittent
  turbulence; a smooth GCM run gets only ~4.5×. **[P]** *"higher grid compression ratios are achievable at
  higher resolutions and in more turbulent (intermittent) cases."*
- **[P]** (ocean) *"For a well-balanced case with a grid compression of about 10 times, adaptive runs are
  about **1.5 times slower per active node**."*

> **⇒ The honest read for us: adaptivity costs ~2–4× per node and pays for itself only above ~2–4×
> compression. For a *planet mostly at rest being observed by one explorer*, our compression ratio is not
> 4× — it is enormous. The economics that are marginal for a GCM are overwhelming for us.** **[me]**

### 5.2 What is genuinely empty

Having found WAVETRISK, the honest statement of the gap is **much narrower and much more specific** than
"nobody does this":

| combination | status |
|---|---|
| structure-preserving + sphere + adaptive | ✅ **WAVETRISK** (hex/tri icosahedral) |
| topological identity exact across non-conforming AMR | ✅ **Balsara** (Cartesian MHD) |
| mimetic + hanging nodes | ✅ **Lipnikov/Morel/Shashkov**, MFD/VEM generally (Cartesian/polygonal) |
| conservative FV + cubed sphere + nonconforming AMR | ✅ **Chombo / Ferguson et al.** (collocated; mass only) |
| **mimetic / staggered C-grid + CUBE-SPHERE + quadtree AMR** | **⊘ I found nothing.** |
| **structure-preserving + *fated-noise* closure / content-addressed memoised store** | **⊘ nothing, and this is genuinely ours** |
| **a multiresolution transform on a cube-sphere quadtree with unequal cell areas** | ✅ **theory exists** (§6) — **but I found no implementation on a cube-sphere** |

**⇒ So the well-evidenced negative claim is this, and it is narrow enough to be useful:**

> ### **Nobody has built a structure-preserving STAGGERED scheme on a CUBE-SPHERE QUADTREE.** Every ingredient exists and has been separately proved. The reason the combination is empty is almost certainly §4.2 — **the equiangular cube-sphere is not orthogonal, so the cheap mimetic C-grid construction is unavailable, and everyone who wanted mimetic went to icosahedral/hexagonal instead** (where SCVT gives you orthogonality for free). **We would be building, not adopting. But we would be building on FEEC, which is exactly the tool built to remove that obstruction, and Cotter & Thuburn have already named the configuration.**

**[me] That is a real research position — defensible, narrow, and with a named path. It is not "nobody
does this so we're on our own."**

---

## 6. Multiresolution on a cube-sphere quadtree with UNEQUAL cell areas

*Joint with the multiresolution-store spike; this is the literature half.* **Answer: yes, it exists, the
theory is from 1997, and our case is STRICTLY EASIER than the published one.**

### 6.1 The theory: Haar does not assume equal weights — "unbalanced Haar" does the general case

> **Girardi, M. & Sweldens, W. (1997).** *A new class of unbalanced Haar wavelets that form an
> unconditional basis for Lᵖ on general measure spaces.* J. Fourier Anal. Appl. 3(4):457–474.
> `relata: girardi-1997-unbalanced-haar` — **READ PRIMARY**, PDF registered.

> **[P]** *"Given a complete separable σ-finite measure space (X, Σ, µ) **and nested partitions of X**, we
> construct **unbalanced** Haar-like wavelets on X that form an unconditional basis for Lᵖ(X, Σ, µ) …"*
> (Abstract)
>
> **[P]** *"…our wavelets are not the translates and dilates of one function, [but] they still enjoy the
> desirable properties of traditional wavelets, such as a multiresolution structure and an associated fast
> transform algorithm. **Our setting allows for non-translation invariant measures and covers general nested
> partitions of arbitrary subsets of Euclidean spaces.**"* (§1)

**A cube-sphere quadtree IS a nested partition, and cell area IS the measure µ.** The 1.4× area variation
is not an obstacle — it is *literally the case the paper was written for*.

**⇒ Joseph's worry — "Haar assumes equal weights; ours vary 1.4×" — is answered: classical Haar does;
unbalanced Haar does not; and the unbalanced construction is exactly as fast (there is a fast transform).**

**And the authors state the price themselves, which is the honest half:**

> **[P]** *"The use of the unbalanced Haar wavelets in applications is still somehow limited. The reason is
> that **they are non-smooth and that they have only one vanishing moment** … Consequently, **the convergence
> of the expansion is slow for a smooth function f**. In [Sweldens] the **"lifting scheme"** is described,
> which given one initial multiresolution analysis, can allow you to build a second, more performant one …
> **The Haar wavelets constructed in this paper are a perfect example for such an initial multiresolution
> analysis.**"* (§9)

> ### **⇒ THE COMPLETE ANSWER TO THE SPIKE, IN ONE SENTENCE.**
> **An area-weighted (unbalanced) Haar transform on our quadtree is EXACT, EXACTLY CONSERVATIVE, and gives
> you the coarse↔fine identity for free — but it COMPRESSES BADLY on smooth fields (one vanishing moment),
> which is most of a planet. The LIFTING SCHEME is the named, standard, cheap upgrade that buys the
> vanishing moments back, and unbalanced Haar is explicitly the intended starting point for it.**
>
> That is: **build the Haar store first (correctness), then lift (compression).** Two stages, both named,
> neither invented here.

- `sweldens-1998-lifting` — Sweldens, *The Lifting Scheme: A Construction of Second Generation Wavelets*,
  SIAM J. Math. Anal. 29(2):511–546. **PDF registered.** The lifting scheme is **specifically designed for
  irregular samples, weights, and non-Euclidean domains** — it is *why* second-generation wavelets exist.
- `schroder-1995-spherical` — Schröder & Sweldens, *Spherical wavelets: efficiently representing functions
  on the sphere*, SIGGRAPH '95:161–172. **PDF registered.** The sphere case, via lifting on a subdivision
  hierarchy.

### 6.2 The practice: Aechtner et al. did it on a sphere, and their hard part is our free part

**[P]** In `aechtner-2015-wavelet-sphere`, the scaling/wavelet weights on the **non-uniform** spherical
grid are **literally area fractions**:

$$\tilde s^{\,j}_{km} = \frac{A^{j+1}_{km}}{A^{j+1}_m}, \qquad s^{\,j}_{km} = \frac{A^{j+1}_{km}}{A^{j}_{k}}$$

> **[P]** *"where **A^{j+1}_{km} is the area shared by the coarse level hexagon A^j_k and the fine level
> hexagon A^{j+1}_m** … Note that partial areas … cover the fine and coarse scale hexagons, **ensuring that
> s and s̃ are indeed weights**. Thus, **it is necessary to compute the areas of intersection of spherical
> polygons**."* (§3)
>
> **[P]** *"…the construction of a flux restriction operator R_F that guarantees this commutation property
> for a given height restriction operator R_h **poses additional difficulties due to location-dependent
> discrete geometry and due to the problem of overlapping hexagons at successive levels**."*

> ### **⚠ READ THAT AGAIN, BECAUSE IT IS THE BEST NEWS IN THIS DOCUMENT.**
> **Their entire hard part is that HEXAGONS DO NOT NEST.** A coarse hexagon is *not* the union of fine
> hexagons, so they must compute **spherical-polygon intersection areas** to get the restriction weights,
> and they must split the flux restriction into a basic part plus a **corrective** part to recover the
> commutation relation.
>
> **A cube-sphere quadtree NESTS EXACTLY. A coarse cell IS the union of its four children.** The
> intersection areas are trivially the child areas; the overlap problem does not exist; the corrective term
> is not needed.
>
> **On the one axis where this literature is genuinely painful, our grid is strictly, structurally better —
> and it is better for exactly the reason `discretisation-and-information.md` §7(b) worried the hexagonal
> dual would fail: *hexagons do not nest*. That worry was correct, and it is the published state of the
> art's actual tax.**

**⇒ Handoff to the multiresolution-store spike (§7 of this doc's parent):** the transform you want is an
**area-weighted (unbalanced) Haar on the CellId quadtree**, whose restriction weight is `A_child / A_parent`
and whose detail coefficients are the deviations; verify it satisfies the **`div` commutation relation
(§3.3, Eq. 2)** and you have mass conservation across the seam **as an identity**. Then lift for
compression. References: `girardi-1997-unbalanced-haar`, `sweldens-1998-lifting`,
`aechtner-2015-wavelet-sphere` §3–4, `kevlahan-2019-wavetrisk` §2.

---

## 7. The topological identities — free, and cheaper than we hoped

> **Desbrun, M., Hirani, A. N., Leok, M. & Marsden, J. E. (2005).** *Discrete Exterior Calculus.*
> arXiv:math/0508341. — **READ PRIMARY**.

The discrete exterior derivative **is defined as** the coboundary operator `δ`, `δ^k(α) = α ∘ ∂_{k+1}`:

> **[P]** *"This definition of the coboundary operator induces the cochain complex … where it is easy to
> see that **δ^{k+1} ∘ δ^k = 0**."* (Def. 5.6–5.7)
>
> **[P]** *"…the discrete Stokes' theorem, **which is true by definition**, states that ⟨dα, c⟩ = ⟨α, ∂c⟩.
> Furthermore, **it also follows immediately that d^{k+1} d^k = 0**."* (Remark 5.1)

> ### **⇒ ANSWER TO THE BRIEF'S QUESTION 1a, AND IT IS UNCONDITIONAL.**
> **`∇×∇φ ≡ 0` and `∇·∇×A ≡ 0` hold EXACTLY — to machine precision, by construction, with zero effort —
> on ANY mesh: unstructured, spherical, badly distorted, non-conforming.** They are consequences of
> `∂∘∂ = 0` on the chain complex, which is *combinatorics*. **No geometry is involved and none can break
> them.**
>
> **The entire metric content of the method — and therefore the entire accuracy question, and therefore the
> entire sphere question — is concentrated in the HODGE STAR.** DEC's cheap diagonal Hodge star wants a
> **well-centred / Delaunay** mesh (circumcentres inside their simplices); on meshes that fail that, you
> need off-diagonal corrections or a different star (there is active work here — e.g. a
> primal-to-primal star for **general polygonal meshes**, Ptáčková & Velho 2024, arXiv:2401.15436 **[A]**).

**⇒ Three consequences that change what we should probe:**

1. **The `curl` probe in `discretisation-and-information.md` §6.2 will PASS trivially if we adopt DEC/FEEC
   — and that means it is testing the wrong thing.** If we *build* the operator as a coboundary, curl-free
   is free and the probe cannot fail. **A probe that cannot fail is not a probe** (our own standing guard).
   The probe is only meaningful **against our CURRENT kernel**, where the operators are *not* built that way
   — and there it is a genuine test and should be run **now, before any rewrite**, precisely because a
   rewrite would make it vacuous.
2. **The "potential" row of the structure table is mis-stated.** It says mimetic discretisation is *"what
   preserves it exactly."* True, but it undersells: *any* discretisation that defines its operators
   combinatorially gets it, and *no* amount of accuracy in a non-mimetic scheme gets it. **It is a
   yes/no property of how you DEFINE the operator, not a quality of how well you approximate.**
3. **The hard row we do not have is the Hodge star.** That is where the sphere, the non-orthogonality, and
   the accuracy all actually live, and our table has no row for it.

---

## 8. Symplectic, entropy, positivity, well-balanced — the remaining rows

*Investigated in parallel; the detail and the primary quotes are in the companion notes. Headlines:*

### 8.1 Positivity — nearly free, and the price is a CFL

> **Shu, C.-W.** *Positivity-preserving high order schemes for convection dominated equations* (lecture
> notes, cfd.ku.edu/JRV/Shu.pdf) — **READ PRIMARY**; the Zhang–Shu lineage.

The limiter is a **linear scaling of the reconstruction toward the cell mean**,
`p̃(x) = θ(p(x) − ū) + ū` with `θ = min(…, 1)`, evaluated only at pre-chosen quadrature points:

> **[P]** *"We have figured out a way to obtain such reconstruction with **a very simple scaling limiter**,
> which only requires the evaluation of u(x) at certain pre-determined quadrature points **and does not
> destroy accuracy**."*
>
> **[P]** *"The technique has been generalized to the following situations **maintaining uniformly high
> order accuracy** … [including] one and two-dimensional shallow water equations **maintaining
> non-negativity of water height AND well-balancedness for problems with dry areas**."*

**⇒ Answer to the brief's Q5: the positivity limiter costs essentially NOTHING in order of accuracy** —
that was the whole point of Zhang & Shu, and it is provable. **The price is a reduced CFL** (**[P]** *"a
reduced CFL condition λ/ω₀ ≤ λ₀"*, set by the first Gauss–Lobatto quadrature weight). **And it composes
with well-balancedness** — that combination is a published result (Xing, Zhang & Shu 2010), not a hope.
**This row is pure adopt.**

### 8.2 Entropy stability — necessary, and NOT sufficient, and the distinction is the whole point

**Conservation is not enough:** a conservative scheme can converge to a **non-physical weak solution** (an
expansion shock conserves mass and momentum and runs time backwards). Entropy stability is what excludes
those. Tadmor's construction: an **entropy-conservative** flux, plus **explicitly added dissipation** —
because **[A]** *"exact entropy conserving schemes cannot dissipate energy at shocks."*

**⚠ The honest limit, and it must be declared:** a discrete entropy inequality **rules out** entropy-
violating solutions; for nonlinear **systems** it does **not** by itself prove convergence to *the* unique
entropy solution. **Necessary, not sufficient.** *(Stated as my reading of the standard position — see the
companion notes for what was verifiable.)*

**On a curved/spherical mesh it costs one more thing: the GCL (§4.4).**

### 8.3 Well-balanced — the minimal change is small and it is a *reconstruction*, not a flux

`audusse-2004-hydrostatic` (SIAM J. Sci. Comput. 25(6):2050–2065) — **hydrostatic reconstruction**: keep
the free surface `h + z` (not `h`) as the reconstructed variable, and rebuild interface depths so that a
flat surface over a bumpy bed produces **identically cancelling** pressure and bed-slope terms.

**⇒ The important structural point for us, and it is the one the brief was hunting:** well-balancedness is
**not** a property of the flux function — it is a property of **which variable you reconstruct**. It costs
almost nothing. **And our `water.rs` has never been tested for it.** The probe
(`discretisation-and-information.md` §6.3) stands, and it is the cheapest convicting probe we have.

### 8.4 Symplectic — and the collision with adaptivity, which is a genuine no-go

`hairer-2003-geometric`. The guarantee is a **backward-error** result: a symplectic method's numerical
trajectory is the exact trajectory of a **nearby Hamiltonian** system, so the energy error is **bounded,
not secular**, over exponentially long times.

> ### **⚠ AND THE HYPOTHESIS IS: CONSTANT STEP SIZE.**
> **[A]** *"Symplectic methods … have favorable properties concerning long-time integrations … **if applied
> with constant step sizes**, while **all of these properties are lost in a standard variable step size
> implementation**."*

**⇒ This is a first-class instance of the governing principle, and it lands directly on our architecture.**
Our whole seam discipline (`multiscale-seams.md` §3) is built on **deriving Δt from the CellId level** —
i.e. **step size varies**. For the ante-mundane orbital phase that is *exactly the thing that destroys the
structure we would be adopting symplectic integration to get.*

**The ante-mundane phase must therefore either (a) run at fixed step, outside the multirate ladder — which
is fine, because an orbit has no spatial grid — or (b) use a time-symmetric / Poincaré-transformed variable
step, which is the standard workaround.** **⇒ Proposed: the orbital nomos is FIXED-STEP and is explicitly
exempt from the CellId-derived-Δt rule, and that exemption gets DECLARED, not discovered.**

*(Companion note also covers: what symplecticity buys once tidal dissipation makes the system
non-Hamiltonian — the short answer is "less than you think," and it should be declared.)*

---

## 9. What to adopt, what to derive — proposed

**Nothing below is decided. `:by claude :status proposed`.**

### Adopt outright (mature, proven, cheap)

| # | adopt | from |
|---|---|---|
| A1 | **The vocabulary**: compatible/mimetic/structure-preserving; **modified equation analysis**; **backward error analysis**; geometric numerical integration. | §1 |
| A2 | **The commutation relations as the general law of multiresolution consistency**, and a **new row in the structure table**. | §3.3 |
| A3 | **Face-staggered flux store + area-weighted restriction + a Berger–Colella-shaped seam correction.** Our proposal, already built, 25 years old. | §3.1 |
| A4 | **The hanging node is a polygon.** Coarse cells at a refinement boundary have 5–6 edges. Adopt the **2:1 balance** rule. | §3.2 |
| A5 | **Area-weighted (unbalanced) Haar on the quadtree, then lifting.** Correctness first, compression second. | §6 |
| A6 | **The Zhang–Shu positivity limiter.** Accuracy-free; costs a CFL. | §8.1 |
| A7 | **Hydrostatic reconstruction** for well-balancedness. Reconstruct `h+z`, not `h`. | §8.3 |
| A8 | **Fixed-step symplectic for the ante-mundane orbital nomos**, explicitly exempt from the CellId-Δt rule, **declared**. | §8.4 |

### Probe before anything (each is cheap and each could convict us)

| # | probe | why now |
|---|---|---|
| P1 | **The curl probe, on the CURRENT kernel.** | It becomes **vacuous** after a mimetic rewrite. Run it while it can still fail. §7 |
| P2 | **The commutation-relation probe** — does our coarse↔fine operator satisfy `R∘div = div∘R_F`? And relation (3), *uniform PV stays uniform*? | §3.3. We have never asked. |
| P3 | **The discrete GCL probe** — do our cube-sphere metric terms satisfy the metric identities exactly? | §4.4. If not, well-balancedness and entropy stability are **unavailable as theorems**, not merely inaccurate. |
| P4 | **The well-balanced probe.** | §8.3. Cheapest convicting probe we own. |
| P5 | **The two-point-flux-at-a-hanging-node probe.** | §3.2 says it is **non-convergent** even on an orthogonal mesh. Our seam may have a second, independent disease. |

### Derive / build (genuinely ours)

| # | build | why it isn't adoptable |
|---|---|---|
| B1 | **A structure-preserving staggered scheme on a CUBE-SPHERE QUADTREE.** | §5.2 — the one empty cell. Path: **FEEC**, `RT0` on quads, primal-dual, per Cotter & Thuburn's own recommendation — **plus** the Bochev–Ridzal rehabilitation, **plus** the commutation relations for the quadtree. |
| B2 | **Fated-noise closure as the fluctuation half of the FD pair, inside a structure-preserving scheme.** | Nobody does this. It is ours and it is defensible. |
| B3 | **Content-addressed memoised multiresolution store.** | The wavelet transform is adopted; the *store* is ours. |

### ⚠ Resolve before B1 — the two paper-checkable questions

1. **Is ABF's "RT0 has no `div` convergence on non-affine quads" the same animal as our measured
   "two-point flux is inconsistent, order −0.5"?** §4.3. If yes: our measurement has a theorem behind it.
2. **Does Bochev–Ridzal's "natural mimetic divergence operator" mean our FV divergence is already the
   rehabilitated one?** §4.3. If yes, the ABF trap **does not apply to an FV scheme at all**, and B1 gets
   dramatically cheaper. **[me] I think it does. I have not proved it. Do not build on this.**

---

## 10. Gaps, named as gaps

- **[⊘]** The ABF ⇄ two-point-flux correspondence (§4.3). **Highest value; checkable on paper.**
- **[⊘]** Whether a **discrete GCL** holds on our equiangular cube-sphere. Never checked.
- **[⊘]** Whether a mimetic/staggered C-grid can be constructed on a **non-orthogonal** cube-sphere at
  acceptable cost. Cotter & Thuburn call this an **outstanding direction** — so it is a frontier, not an
  oversight, and FEEC is the stated route.
- **[⊘]** No implementation found of a multiresolution transform **on a cube-sphere quadtree**. The theory
  is complete (§6); the code does not exist. That is a build, and an easy one.
- **[⊘]** Entropy stability's exact guarantee for *systems* (necessary vs sufficient) — I state the
  standard position but did not verify it to primary-source standard.
- **[⊘]** Arakawa & Lamb 1981's exact conservation claims and their hypotheses — metadata verified
  (`arakawa-1981-enstrophy`, Mon. Wea. Rev. 109:18–36); **I did not read the paper.** Do not cite its
  guarantees from this document.
- **[⊘]** MPAS's own stated rationale (Ringler et al. 2010, `ringler-2010-unified`) — metadata verified,
  **paper not read**. The "why hexagons" answer in §4.1 comes from Cotter & Thuburn's characterisation,
  which is a *secondary* account of MPAS's reasoning, however authoritative the authors.

---

## 11. Sources — with read status, honestly marked

**All 25 seeded in `relata` with `bib-fields` verified against the Crossref DOI record.** ✅ = OA PDF
obtained and registered in relata.

### READ PRIMARY (I obtained the full text and read the cited sections)

| bibkey | work | ✅ |
|---|---|---|
| `balsara-2001-divfree` | Balsara, *Divergence-Free AMR for MHD*, JCP 174(2):614–648 (2001) | ✅ |
| `lipnikov-2004-nonconformal` | Lipnikov, Morel & Shashkov, *MFD on non-orthogonal non-conformal meshes*, JCP 199(2):589–597 (2004) | ✅ |
| `arnold-2005-quadhdiv` | Arnold, Boffi & Falk, *Quadrilateral H(div) finite elements*, SINUM 42(6):2429–2451 (2005) | ✅ |
| `thuburn-2009-geostrophic` | Thuburn, Ringler, Skamarock & Klemp, *Geostrophic modes on arbitrarily structured C-grids*, JCP 228(22):8321–8335 (2009) | ✅ |
| `cotter-2014-feec-swe` | Cotter & Thuburn, *A FEEC framework for the rotating shallow-water equations*, JCP 257:1506–1526 (2014) | ✅ |
| `ferguson-2016-amr-cubed` | Ferguson, Jablonowski, Johansen, McCorquodale, Colella & Ullrich, *AMR characteristics of a high-order 2D cubed-sphere shallow-water model*, MWR 144(12):4641–4666 (2016) | ✅ |
| `aechtner-2015-wavelet-sphere` | Aechtner, Kevlahan & Dubos, *Conservative adaptive wavelet method for SWE on the sphere*, QJRMS 141(690):1712–1726 (2015) | ✅ |
| `kevlahan-2019-wavetrisk` | Kevlahan & Dubos, *WAVETRISK-1.0*, GMD 12:4901–4921 (2019) | ✅ |
| `kevlahan-2022-wavetrisk-ocean` | Kevlahan & Lemarié, *wavetrisk-2.1: adaptive core for ocean modelling*, GMD 15:6521–6539 (2022) | ✅ |
| `girardi-1997-unbalanced-haar` | Girardi & Sweldens, *Unbalanced Haar wavelets … general measure spaces*, J. Fourier Anal. Appl. 3(4):457–474 (1997) | ✅ |
| `arnold-2010-feec-hodge` | Arnold, Falk & Winther, *FEEC: from Hodge theory to numerical stability*, Bull. AMS 47:281–354 (2010) | ✅ |
| — | Desbrun, Hirani, Leok & Marsden, *Discrete Exterior Calculus*, arXiv:math/0508341 (2005) — **not yet in relata (no DOI)** | |
| — | Shu, *Positivity-preserving high order schemes* (lecture notes, Brown) — **not a citable work; the citable ones are Zhang & Shu (JCP 2010) and Xing, Zhang & Shu (Adv. Water Resour. 2010), not yet seeded** | |
| — | Wintermeyer, Winters, Gassner & Kopriva, *Entropy stable nodal DG for 2D SWE on unstructured curvilinear meshes*, arXiv:1509.07096 — **not yet seeded** | |

### SEEDED + METADATA VERIFIED, NOT READ (do not cite their guarantees from this document)

`arnold-2006-feec` (Acta Numerica 15:1–155) · `bochev-2009-rehabilitation` (SINUM 47:487–507, **abstract
read**) · `ringler-2010-unified` (JCP 229:3065–3090) · `staniforth-2012-grids` (QJRMS 138:1–26) ·
`arakawa-1981-enstrophy` (MWR 109:18–36) · `randall-1994-adjustment` (MWR 122:1371–1377) ·
`weller-2012-modes` (MWR 140:3220–3234, **abstract read** — note: **Hilary Weller, single author**, not
Weller–Thuburn–Cotter as I first assumed) · `dubos-2013-staggered-wavelet` (QJRMS 139:1997–2020) ·
`sweldens-1998-lifting` (SIMA 29(2):511–546) ✅ · `schroder-1995-spherical` (SIGGRAPH '95:161–172) ✅ ·
`berger-1989-refluxing` (JCP 82(1):64–84) · `warming-1974-modified-equation` (JCP 14(2):159–179) ·
`hairer-2003-geometric` (Acta Numerica 12:399–450) · `audusse-2004-hydrostatic` (SISC 25(6):2050–2065).

---

*Author's honesty note. The three claims I would most want attacked, in order:*
***(1)*** *that the ABF non-affine-quadrilateral defect is the same object as our measured two-point-flux
inconsistency, and that Bochev–Ridzal therefore exonerates an FV scheme — this is inference, it is
load-bearing for §9/B1, and it is checkable;* ***(2)*** *that our exact-nesting quadtree makes the
multiresolution problem strictly easier than the published hexagonal case — I believe this firmly and it
follows from Aechtner §3, but it deserves a hostile read;* ***(3)*** *that the WAVETRISK cost economics
transfer to us — their compression ratios are for turbulent GCM flow, ours would be for an observed,
mostly-quiescent planet, and I have assumed that helps us without measuring it.*
*Each has a cheap probe attached. Kill them before building on them.*
