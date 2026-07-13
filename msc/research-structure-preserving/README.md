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

## 0. The eight findings, before the detail

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

6. **Our grid is in far better shape than I expected. COLLOCATION is the real cost — and it is a COST, not
   a BUG.**
   - **Our DOF ratio is the RIGHT one.** A quadrilateral grid has exactly 2:1 velocity:mass (D = 3) — **no
     spurious computational modes** — unlike hexagons (spurious Rossby) or triangles (spurious
     inertia-gravity). **[P]** Weller 2012, naming us: *"**The cubed sphere has the correct ratio of d.o.f.,
     but is usually nonorthogonal**."*
   - **⚠ So our disease is NOT computational modes.** A collocated quad A-grid *also* has D = 3. Its
     pathology is **parasitic modes on the PHYSICAL branches** — **half the gravity-wave spectrum
     propagates BACKWARDS**, and a forcing at one frequency **excites a spurious short wave alongside the
     real one**. **[P]** *A bias — and a different disease from the one I set out to find.*
   - **But collocation is survivable, and the C-grid partisans say so.** **[P]** MPAS's flagship paper:
     *"**all grid staggerings can be made to work with some level of filtering** … We have found that the
     C-grid … results in the highest efficiency."* **You pay in filtering and effective resolution — and
     `α_stab` IS that payment.** Ferguson et al. run *our exact grid* — equiangular, quadtree,
     collocated — with a 6th-order hyperdiffusion, and it works.
   - **Non-orthogonality is a spectrum, not a cliff.** Even the *conformal* cube-sphere is non-orthogonal at
     the 8 corners (120°) **[P]**; and Putman & Lin **measured** that *"the more uniform version of the
     quasi-orthogonal cubed-sphere grids provided **better overall accuracy than the most orthogonal**
     … conformal grid."* **[P]** **Uniformity beat orthogonality — a direct, measured defence of our
     equiangular choice.**
   - **⚖ And on ONE row our quad grid is not the compromise — it is the PRIVILEGED grid.** Exact
     **simultaneous energy + potential-enstrophy** conservation is available on a square C-grid
     (Arakawa–Lamb 1981) and is **NOT** available from TRiSK on arbitrary structure. **[P]**

7. **⚖ THE LEAD I DID NOT EXPECT — RANDALL'S **Z GRID**, and our quadtree may be what makes it affordable.**
   Randall 1994's real point is not "the A-grid is bad." It is that a **collocated** grid is *excellent* if
   you carry **vorticity, divergence and mass** instead of momentum — with a dispersion relation **[P]**
   *"**superior to that obtained with the Arakawa grids A–E**."* Its one cost is an **elliptic solve**,
   which Randall priced as prohibitive — *"**although it may be appropriate to reexamine this point in the
   light of modern algorithms for solving linear systems (e.g., multigrid methods).**"* **[P]**
   **A `CellId` quadtree IS a multigrid hierarchy.** We built it for memoisation and LOD. **The thing that
   makes the Z grid affordable is something we already have, for entirely unrelated reasons — and it stays
   COLLOCATED, so it costs nothing in `CellId`, the store, or the quadtree.** ⚠ **A lead, not a proposal.
   Unmeasured. But it belongs on the table when Joseph adjudicates the grid.**

8. **⚖ AND THE FINDING THAT REORGANISES ALL THE OTHERS — WHICH IS OUR OWN JENSEN THEOREM, ARRIVING AT
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
| **vorticity / circulation** | C-grid + TRiSK Coriolis | stationary geostrophic modes; **energy exact**, **PV exact**, **enstrophy NOT** (§4.7) | ✅ but **needs orthogonality** | ✅ **WAVETRISK** | modest |
| **energy + potential enstrophy JOINTLY** | **Arakawa & Lamb 1981** | **both, exactly, general divergent flow, variable *f*** — ⚠ but **SEMI-DISCRETE ONLY**: your time integrator breaks it (§4.7) | ✅ | ⊘ | wide PV stencil |
| **symplectic** (Hamiltonian) | symplectic integrators | **exponentially long** energy bound — ⚠ **only at FIXED step**, only for **analytic** H, and it bounds **ENERGY, not the TRAJECTORY** | n/a | ❌ **conflicts with adaptivity** | ~free at fixed h |
| **the second law** | entropy-stable (Tadmor lineage) | a **discrete entropy inequality** — ⚠ **a one-sided bound, NOT a selection principle**; **no convergence theory exists for systems** (§8.2) | ✅ **needs GCL** — *or* covariant form (§4.9) | ❌ **naive mortar CRASHES** (§3.4) | **~2.5× CPU, ~1× GPU** |
| **realizability** (`h ≥ 0`) | Zhang–Shu scaling limiter | **exact**, **provably accuracy-preserving**, and **inert in wet regions** | ✅ | ✅ **proved on a quadtree** | **CFL 1/6 vs the 1/5 you already pay** |
| **equilibria** (lake at rest) | well-balanced / hydrostatic reconstruction | exact for **lake-at-rest**; ⚠ **moving equilibria destroy positivity** ⇒ declare sacrificed | ✅ (**needs GCL** on curved) | ✅ **proved on a quadtree** (§8.3b) | small — ⚠ **but breaks when `Δz ≥ h`** (§8.3a) |
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

### 4.1 The DOF / branch rule — and the quadrilateral grid is the GOOD one

The real diagnosis of which grids carry spurious *computational modes* is the **ratio of velocity DOFs to
mass DOFs**, and — better — the **branch count** underneath it. Stated flatly, and about our grid by name:

> **Weller, H. (2012).** *Controlling the Computational Modes of the Arbitrarily Structured C Grid.*
> Mon. Wea. Rev. 140(10):3220–3234. `relata: weller-2012-modes` — **READ PRIMARY** (§1).
> ⚠ **Single author — Hilary Weller.** *(I initially assumed "Weller, Thuburn & Cotter." That is a
> **different paper**: `weller-2012-grid-imprinting`, MWR 140(8):2734–2755. Both are worth having; cite
> them correctly.)*

> **[P]** *"If a 2D C grid is not constructed of quadrilateral cells and/or does not have exactly twice the
> number of velocity degrees of freedom (d.o.f.) as mass variable d.o.f., then computational modes will be
> present… The C grid on hexagons has **too many** velocity d.o.f. and therefore suffers from computational
> modes consisting of a spurious branch of very slow Rossby waves… The triangular C grid has **too few**
> velocity d.o.f. and therefore has two spurious branches of slow inertio-gravity modes… **The cubed sphere
> has the correct ratio of d.o.f., but is usually nonorthogonal**, apart from the conformal cubed sphere,
> **which is not quasi-uniform since cells cluster toward the cube corners**…"*

> ### **That single sentence is the cube-sphere's entire dilemma, from a primary, and it says our DOF ratio is RIGHT and our problem is ORTHOGONALITY.**

The mechanism is branch-counting (Thuburn, *Computational Modes in Weather and Climate Models*, ECMWF
Seminar 2013, §5 — **[P]**, free): the continuous rotating shallow-water system has exactly **3** branches
(2 inertio-gravity + 1 geostrophic). Count the DOF per repeating unit; you get that many branches; the
excess are spurious.

| grid | cells / vertices / edges | D | branches | spurious |
|---|---|---|---|---|
| **quad C-grid** | 1 / 1 / 2 | **3** | 1 Rossby + 2 IG | **none** |
| hexagonal C-grid (MPAS) | 1 / 2 / 3 | 4 | **2** Rossby | 1 Rossby branch |
| triangular C-grid (ICON) | 2 / 1 / 3 | 5 | **4** IG | 2 IG branches |
| **collocated quad A-grid (ours)** | — | **3** | — | **none** |

### ⚠ 4.1a THE CORRECTION THAT MATTERS MOST — our disease is NOT computational modes

**A collocated quad A-grid also has D = 3.** So — and this cuts against the framing I started with, and
probably against the one in our own structure table —

> ### **The A-grid's pathology is NOT extra computational modes. It is PARASITIC modes ON THE PHYSICAL BRANCHES.**
>
> **[P]** Thuburn (ECMWF 2013) §5: *"Polygonal A-grids suffer from **parasitic modes** like those discussed
> [in] section 3."*

**That is a different disease from the hexagon's, and it means "spurious branch counting" is not the
argument against us.** Our two real problems are **(i) collocation** and **(ii) non-orthogonality**, and
**they are independent.** Diagnose them separately.

### 4.2 What ACTUALLY breaks on a collocated grid — and it is worse than the checkerboard

**[P]** Thuburn (ECMWF 2013), §2.1: *"an initial condition comprising zero velocity and a **checkerboard
pattern in φ** will be steady and so fail to propagate; the finite difference formula estimates the
gradient of φ as zero."* — **this is exactly the Nyquist null-mode we derived in
`discretisation-and-information.md` §3.2, confirmed.**

But there are **two** invisible modes, not one, and we only had the first:

**[P]** §2.2: *"An initial condition comprising **zero geopotential perturbation and a u field with a 2Δx
wavelength** … **and a v field with a 2Δy wavelength** … will be steady and so fail to propagate. In this
case **the velocity divergence is estimated as zero.**"*

**And the part that is genuinely alarming, and that we did not have:**

> **[P]** §3: *"For the shortest resolvable waves, kΔx = π, the numerical frequency is zero… In particular,
> **half of the numerical spectrum has ∂ω/∂k of the wrong sign implying that packets of such waves have
> group velocity of the wrong sign.** Such modes are often called **parasitic modes**."*
>
> **[P]** *"…the numerical dispersion relation has **two values of k for each value of ω**. This means that,
> when forced at a particular frequency, **a model can produce a spurious shortwave response as well as a
> physically realistic longwave response.**"*

> ### **⇒ HALF THE SPECTRUM PROPAGATES BACKWARDS. And a forcing at one frequency EXCITES A SPURIOUS SHORT WAVE alongside the real one.**
> **That is a BIAS, not noise** (our own audit, `discretisation-and-information.md` §1) — energy is being
> transported in the wrong direction, systematically, at the grid scale. **[me] Every forced system we
> queue — precipitation forcing water, tides forcing the ocean — will inject a parasitic companion wave.**

Corroborated independently, **Randall, D. A. (1994)**, *Geostrophic Adjustment and the Finite-Difference
Shallow-Water Equations*, MWR 122(6):1371–1377, `relata: randall-1994-adjustment` — **[P]**:

> *"The averaging described above inevitably '**hides**' the smallest represented scales (e.g., a
> checkerboard pattern in h). Such dynamically '**invisible**' noise cannot participate in the dynamics of
> the model… A plot of the dispersion equation for the A grid indicates a **maximum of the frequency (group
> speed equal to zero)** … As a result, **solutions on the A grid are extremely noisy in practice and must
> be smoothed** — for example, through filtering… Because of this well-known problem, **the A grid is hardly
> used today.**"*

### 4.3 …but collocation is a COST, not a BUG — and that is stated by a C-grid partisan

> **Skamarock, W. C., Klemp, J. B., Duda, M. G., Fowler, L. D., Park, S.-H. & Ringler, T. D. (2012).**
> *A Multiscale Nonhydrostatic Atmospheric Model Using Centroidal Voronoi Tesselations and C-Grid
> Staggering.* Mon. Wea. Rev. 140(9):3090–3105. `relata: skamarock-2012-mpas` — **READ PRIMARY** (§2d).

> **[P]** *"**C-grid staggering provides twice the resolution of divergent modes compared to the unstaggered
> (A) grid**; it does not require any averaging of the velocities or pressures in the pressure gradient and
> divergence terms as is required in the A-, B-, D-, and E-grid staggerings. Pressure and velocity averaging
> lead to **stationary grid-scale modes (often referred to as parasitic modes) that must be filtered**… Our
> experience is that **the level of filtering needed on these other meshes is considerably higher** than that
> needed to provide sinks for the downscale energy and enstrophy cascades… we find that **solvers not using
> C-grid staggering need finer meshes to produce similarly resolved features**… **Generally speaking, all
> grid staggerings can be made to work with some level of filtering, and the choices affect scheme
> efficiency (accuracy versus cost). We have found that the C-grid discretization results in the highest
> efficiency.**"*

> ### **⇒ THE HONEST ANSWER TO "HOW WRONG IS COLLOCATED?" — from the MPAS flagship paper, i.e. from the other side of the argument: NOT WRONG. EXPENSIVE.**
> **You pay in filtering and in effective resolution. Nobody in this literature says the A-grid is
> *incorrect*.** And **`α_stab` is the price** — which is precisely `discretisation-and-information.md`
> §2.5, confirmed from a second, independent direction.

**And one line from the same page cuts the other way, and we should not hide it:**

> **[P]** *"**Randall's (1994) analysis of geostrophic adjustment indicates that the C-grid staggering is
> not optimal for large-scale flows.** Our intended applications for MPAS are cell spacings of the order
> 100 km and less."*

**[me] Our ocean gyres and circulation bands ARE large-scale flow. The C-grid is not automatically the
right answer for us — which is the doorway to §4.4.**

### 4.4 ⚖ THE LEAD I DID NOT EXPECT TO FIND — RANDALL'S **Z GRID**

*This is the most valuable single item in the reconnaissance for the grid question, and it is COLLOCATED.*

Randall's 1994 point is **not** "the A-grid is bad." It is that a **collocated grid is excellent — if you
abandon the momentum equations for the VORTICITY–DIVERGENCE form.**

> **[P]** Randall 1994, Abstract: *"Numerical simulation of geostrophic adjustment in shallow water is
> discussed for the case of an **unstaggered grid for vorticity, divergence, and mass**. The dispersion
> equation is shown to be **very well behaved and superior to that obtained with the Arakawa grids A–E**."*

**Superior to the C grid.** No zero-group-speed pathology, no parasitic branch. On one collocated,
unstaggered mesh. And the cost is stated, honestly, by Randall himself:

> **[P]** *"it is necessary to **solve elliptic equations** to obtain the winds from the vorticity and
> divergence… Such solution procedures can be computationally expensive in finite-difference models…
> although **it may be appropriate to reexamine this point in the light of modern algorithms for solving
> linear systems (e.g., multigrid methods).**"*

> ### **⇒ READ THAT PARENTHESIS AGAINST OUR DATA STRUCTURE.**
> **The Z grid's one cost is an elliptic solve. Randall himself flagged MULTIGRID as the reason to revisit
> it — in 1994.**
>
> **A `CellId` quadtree IS a multigrid hierarchy.** Parent = one bit-shift. We already have — for entirely
> unrelated reasons (memoisation, LOD, the wavelet store) — precisely the data structure whose absence made
> the Z grid unaffordable.
>
> **And it stays COLLOCATED**, so it costs us nothing in `CellId`, nothing in the store, nothing in the
> quadtree, and it does not require a face-staggered rewrite.
>
> **[me] This is the highest-value unexplored lead in the whole recon, and it is a strange and pleasing
> convergence: the thing that makes the Z grid affordable is the thing we built for other reasons.
> ⚠ It is a LEAD, not a proposal. It is unmeasured, it interacts with the multiresolution store in ways
> nobody has thought about, and an elliptic solve per step is a real cost in a lazy pull-query
> architecture. But it deserves a spike, and it deserves to be on the table when Joseph adjudicates the
> grid question.**

*(Corroborating that the Z grid is the thing everyone wants: **FV3's entire two-grid C→D design is an
explicit Z-grid surrogate.* **[P]** Lin 2004, MWR 132(10):2293–2307, p.2294: *"a two-grid two-step
'reversed engineering approach' was developed. **It has the advantage of the Z grid (Randall 1994) without
its computational expense of solving an elliptic equation.**"* — `relata: lin-2004-vertically-lagrangian`.
⚠ **And note: Lin 2004 is a LATITUDE–LONGITUDE dycore, NOT a cubed-sphere one.** I had assumed otherwise
and was wrong; FV3-on-cubed-sphere begins with `putman-2007-cubed-sphere`.)*

### 4.5 Orthogonality — TRiSK needs it; we do not have it; and it matters LESS than I feared

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
> ⟨conformal⟩ cubed sphere**."* (§1)

Stated most precisely in the follow-up (`ringler-2010-unified`, JCP 229:3065–3090, §1) — **[P]**:
*"**The requirement for the method derived below to hold is that the mesh be locally orthogonal in the
sense that the edges that define mass cells and the edges that define vorticity cells are perpendicular at
their intersection.**"*

**Vivarium's grid is EQUIANGULAR, not conformal, and we have measured it non-orthogonal. So TRiSK-as-published does not apply to us.** That much is confirmed.

**⇒ But three findings make this far less damning than it sounds, and the third is a measured result in
our favour.**

**(a) NOBODY actually satisfies the orthogonality condition on a cube.**

> **[P]** Putman, W. M. & Lin, S.-J. (2007), *Finite-volume transport on various cubed-sphere grids*, JCP
> 227(1):55–78, `relata: putman-2007-cubed-sphere`, §1: *"We note, however, **the conformal mapping is only
> orthogonal in the interior, with coordinate lines still intersecting at the 8 corners at a 120-degree
> angle.** Therefore, some modifications to the algorithm still need to be made to counter the
> non-orthogonality near the corners."*

**TRiSK's own "allowed" conformal cubed sphere violates TRiSK's own precondition at the 8 corners.**
Non-orthogonality on a cube is not a gate we failed — it is a condition **everyone special-cases**, and the
corners are irreducible.

**(b) UNIFORMITY BEATS ORTHOGONALITY — and that is a measurement, not an opinion.**

> **[P]** Putman & Lin, Abstract: *"**It is found that slight deviations from orthogonality on the modified
> cubed-sphere (quasi-orthogonal) grids do not negatively impact the accuracy. In fact, the more uniform
> version of the quasi-orthogonal cubed-sphere grids provided better overall accuracy than the most
> orthogonal (and therefore, much less uniform) conformal grid.** It is also shown that **a simple
> non-orthogonal extension to the transport equation enables the use of the highly non-orthogonal and
> computationally more efficient gnomonic grid with acceptable accuracy.**"*

And the "simple extension" is exactly the metric-carrying move we already suspected: **[P]** *"the local
metric factor due to grid non-orthogonality, **sin(α)**, … **reduces to unity for orthogonal grids**"* —
i.e. **carry `sin α` and do not drop it.**

> ### **⇒ This is a direct, measured defence of our equiangular choice, from the FV3 lineage. We are on a spectrum, not off a cliff.**

**(c) And the trade is a stated open frontier, not our private embarrassment:**

> **[P]** *"Two directions remain outstanding from this approach, namely **the relaxation of the
> orthogonality requirement which constrains cubed sphere grids so that grid resolution increases much
> more quickly in the corners than at the middle of the faces**, and the construction of higher-order
> operators to avoid **grid imprinting**."* (Cotter & Thuburn §1)

### 4.6 ⚠ The quantity that actually governs convergence under refinement — and we have already measured it

*This is the most transferable result in the entire reconnaissance.*

> **[P]** `aechtner-2015-wavelet-sphere` §3.2 (*Grid optimization*): *"Since the discretization of the
> differential operators from [TRiSK] is **second-order accurate for equilateral triangles, but drops to
> first-order accurate when the triangles are far from equilateral**, optimizing grid quality improves the
> accuracy of the solutions."*
>
> **[P]** *"**The approximation of the Laplacian operator is guaranteed to converge if the bisection of
> primal edge and dual edge coincide. The distance between those two intersection points is an important
> measure for the grid quality. On simple grids refined by edge bisection the Laplacian operator does not
> even achieve first-order convergence.**"*

> ### **Read that twice. NAIVE REFINEMENT DOES NOT DEGRADE CONVERGENCE — IT DESTROYS IT.** *"does not even achieve first-order."*
>
> **And the quantity that governs it — the offset between where the primal edge is bisected and where the
> dual edge is bisected — IS THE QUANTITY OUR GRID REPORT MEASURED AS "NON-ORTHOGONALITY."**
>
> **We independently found the right diagnostic.** And the literature's response to a bad value of it is
> **not** *"change the grid."* It is **"run a grid-optimisation pass."** Aechtner et al. reduced their
> offset **by a factor of ~60** that way.

**⇒ [me] This is a bounded, concrete, unexplored piece of work: a grid-optimisation pass on an equiangular
cube-sphere quadtree that drives the primal/dual crossing offset toward zero. Nobody has done it. It does
not change `CellId`, the quadtree, or the store. It may be the cheapest path to making a staggered scheme
legal on our grid — and it is a far smaller commitment than changing grids.** ⚠ Unmeasured. A lead.

### 4.7 ⚠ TRiSK cannot do what Arakawa–Lamb does — and that argues FOR quadrilaterals

> **Arakawa, A. & Lamb, V. R. (1981).** *A Potential Enstrophy and Energy Conserving Scheme for the Shallow
> Water Equations.* Mon. Wea. Rev. 109(1):18–36. `relata: arakawa-1981-enstrophy` — **READ PRIMARY**
> (scanned).

> **[P]** Abstract: *"It is pointed out that **a family of schemes can conserve total energy for general
> flow and potential enstrophy for flow with no mass flux divergence.** The newly derived scheme is **a
> unique member of this family, that conserves both potential enstrophy and energy for general flow.**"*

| question | answer |
|---|---|
| energy? | **exact, general (divergent) flow** |
| potential enstrophy? | **exact, general flow** — that is the novelty (older schemes need `∇·v* = 0`) |
| constant *f* required? | **No.** *f* rides inside `q = (f+ζ)/h`; the Appendix does the sphere. |
| ⚠ **exactly?** | **SEMI-DISCRETE ONLY.** **[P]** §3: *"**The time derivatives will be left, for simplicity, in differential form throughout.**"* Conservation is a property of the **space** discretisation, exact in continuous time. **Your time integrator breaks it at truncation order.** *This is the caveat every secondary source drops, and it is exactly the kind of undeclared sacrifice our governing principle exists to catch.* |
| cost | the Coriolis/PV term stops being a simple average: six coefficients, each a 4-point combination of surrounding `q` (≈3×3 stencil), plus specific mandated averages for `h⁽ᵘ⁾`, `h⁽ᵛ⁾`, `h⁽q⁾`, `K`. |
| grid | **C-grid, on quadrilaterals.** |

**And here is the sting for the hexagonal alternative** — TRiSK, on arbitrary structure, **cannot** buy
both:

> **[P]** `ringler-2010-unified` §, p.3078: *"Both the potential enstrophy conserving and potential
> enstrophy dissipating schemes will, in general, **act as spurious sources of kinetic energy.**"*
>
> *(Corroborating the tension in the secondary literature: `weller-2012-modes` p.3220 says TRiSK conserves
> mass, PV, steady geostrophic states, and energy as Δt→0 — "**but not potential enstrophy**".)*

> ### **⇒ EXACT SIMULTANEOUS ENERGY + POTENTIAL-ENSTROPHY CONSERVATION IS AVAILABLE ON A SQUARE C-GRID (Arakawa–Lamb 1981) AND IS *NOT* AVAILABLE FROM TRiSK ON ARBITRARY STRUCTURE.**
> **That is the price of arbitrary-structure generality — and it is the single strongest structural
> argument for keeping a QUADRILATERAL grid.** **[me]** We have been treating our quad grid as the
> compromise. On this row of the structure table, **it is the privileged one.**

### 4.8 The escape hatch from orthogonality, and it is named: FEEC

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

### 4.9 The curved-mesh tax nobody warned us about: the metric identities (GCL) — and the escape from it

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

**⚠ AND THERE IS A 2026 ESCAPE FROM THE WHOLE PROBLEM — which also demolishes an emptiness claim I was
about to make:**

> **Montoya, T., Rueda-Ramírez, A. M. & Gassner, G. J. (2026).** *Entropy-stable discontinuous
> spectral-element methods for the spherical shallow water equations in covariant form.* arXiv:2509.08790v2
> (rev. 9 Feb 2026). `relata: montoya-2026-covariant-sphere` — **READ PRIMARY**, PDF registered.

> **[P]** Abstract: *"We introduce discontinuous spectral-element methods of arbitrary order that are **well
> balanced, conservative of mass, and conservative or dissipative of total energy** (i.e., a mathematical
> entropy function) for a covariant flux formulation of the rotating shallow water equations with variable
> bottom topography **on curved manifolds such as the sphere** … proven to satisfy semi-discrete mass and
> energy conservation **on general unstructured quadrilateral grids** … Furthermore, **the proposed covariant
> formulation permits an analytical representation of the geometry and associated metric terms while
> satisfying the aforementioned entropy stability, conservation, and well-balancing properties WITHOUT THE
> NEED TO APPROXIMATE THE METRIC TERMS SO AS TO ENFORCE DISCRETE METRIC IDENTITIES.** Numerical experiments
> **on cubed-sphere grids** are presented…"*

> ### **⇒ SO THE GCL IS A DESIGN FORK, NOT A TAX.**
> **Either (a) satisfy the discrete GCL exactly** — the flux-form route (Wintermeyer et al.), where the
> metrics must be *discretely* consistent and not merely *true* — **or (b) adopt a COVARIANT formulation, in
> which the metric terms are ANALYTIC and the GCL requirement never arises** (Montoya et al.).
>
> **[me] Option (b) is striking for us, because our cube-sphere map has a CLOSED-FORM JACOBIAN — the very
> object `grid_lab` §9a used to prove the MFD fan bias does not converge away.** We already compute it
> analytically, and we already trust it to 2e-16 against the code. **The thing that convicted our routing
> kernel may be the thing that makes the covariant route cheap.** ⚠ **Inference, unverified — but the
> coincidence is worth an hour of someone's time.**

**And note what Montoya et al. do to the emptiness question:** structure-preserving (entropy-stable +
well-balanced + mass- and energy-conserving), **on a cubed sphere, on quadrilaterals, in 2026.** That cell
is **occupied**. It is **conforming** (no AMR) and carries **no positivity**. §5.2 is corrected accordingly.

### 4.10 What is actually done on a cubed sphere with AMR today

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

**⚠ AND THE WARNING THAT IS DIRECTLY, SPECIFICALLY OURS — they measured a defect we will hit:**

> **[P]** §3 (p. 4654): *"The maximum errors occur in cells bordering the **coarse–fine boundary of the AMR
> patch** and the base grid **when that boundary intersects an edge of the cubed sphere**. This
> **point-source-like artifact** of the AMR grid occurs in both the height-tag and vorticity-tag AMR
> simulations."*

**The AMR seam crossing a cube-panel seam is a real, measured error source** — it drops their `l∞`
convergence from 4th order to between 3 and 2.5. **This is the interaction of our two seams — the tile
seam and the face seam — and it is the one place where the two are known to compound.**
**⇒ Cheap mitigation, free today: keep refinement-patch boundaries off panel edges and corners where the
tile planner can choose.** *(Also from their Table 1, a calibration number worth having: the equiangular
cube-sphere's asymptotic cell-area ratio `A_min/A_max ≈ 0.708`.)*

**And one more, from MPAS, aimed squarely at our refinement strategy — carried openly because it is the
best argument against us that I found:**

> **[P]** `skamarock-2012-mpas` (p. 3091): *"**The smooth mesh transitions we use stand in contrast to the
> abrupt mesh transitions used in traditional two-way nested models … or in mesh refinement achieved
> directly through cell division.** **We believe** the smooth mesh transition will ameliorate many of the
> difficulties associated with traditional nesting approaches."*

**"Mesh refinement achieved directly through cell division" is a quadtree. That is our strategy, named, and
the MPAS team believes it is the wrong one.** Note the words *"We believe"* — it is a **stated position,
not a demonstrated result**, and Ferguson et al. is a counter-example in the same journal. **But we should
have an answer to it, and right now we do not.**

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
| structure-preserving + sphere + **adaptive** | ✅ **WAVETRISK** (hex/tri icosahedral) |
| structure-preserving + **cube-sphere** (ES + WB + mass + energy) | ✅ **Montoya et al. 2026** — conforming; **no positivity, no AMR** |
| topological identity exact across **non-conforming AMR** | ✅ **Balsara 2001** (Cartesian MHD) |
| mimetic + **hanging nodes** | ✅ **Lipnikov/Morel/Shashkov**; MFD/VEM generally (Cartesian/polygonal) |
| **well-balanced + positivity on a QUADTREE** (hanging nodes) | ✅ **Ghazizadeh, Mohammadian & Kurganov 2020** — *proved*, but **2nd-order, Cartesian, no entropy** |
| conservative FV + cube-sphere + nonconforming AMR | ✅ **Chombo / Ferguson et al. 2016** — **collocated; mass only** |
| **entropy stability + AMR** | ⚠ **the naive seam CRASHES** (Friedrich 2018); fixable only with mortar-projection machinery |
| **mimetic / staggered + CUBE-SPHERE + quadtree AMR** | **⊘ Nothing. Searched hard. I believe it does not exist.** |
| **ES + WB + positivity + AMR + sphere, all at once** | **⊘ Nothing.** |
| **structure-preserving + *fated-noise* closure / content-addressed memoised store** | **⊘ nothing — genuinely ours** |
| a multiresolution transform on a cube-sphere quadtree with unequal areas | ✅ **theory complete** (§6) — **⊘ no implementation on a cube-sphere** |

**⇒ So the well-evidenced negative claim is this — and it is much narrower, and much more useful, than
"nobody does this":**

> ### **Every ingredient exists and has been separately PROVED. Nobody has COMPOSED them on a cube-sphere quadtree.**
>
> **And the reason is now diagnosable rather than mysterious.** The mimetic-geophysics community went
> **icosahedral/hexagonal**, because a Voronoi/SCVT mesh hands you **primal-dual orthogonality for free**
> (§4.5) — which is TRiSK's precondition. Everyone who stayed on a cube stayed **collocated** and paid the
> **filtering tax** (§4.3, §4.10). Nobody sat in the intersection because **the intersection requires
> solving the orthogonality problem on a cube, which is a stated open frontier** (§4.5c), **and because the
> entropy seam is genuinely hard** (§3.4).
>
> **We would be building, not adopting.** But we would be building on **three named tools that already
> exist** — FEEC (removes the orthogonality requirement), the **grid-optimisation pass** (§4.6, drives the
> primal/dual offset to ~0 — nobody has run it on a cube-sphere quadtree), and the **mortar projections**
> (§3.4, makes the nonlinear seam legal) — **and Cotter & Thuburn have already named the target
> configuration: `RT0` on quadrilaterals on a cubed sphere, primal-dual.**

**[me] That is a real research position: defensible, narrow, with a named path, and — on the evidence of
§4.7 — sitting on the ONE grid family where exact simultaneous energy + potential-enstrophy conservation is
even available. It is not "we're on our own."**

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

**⚠ AND ONE TRAP THAT WILL COST A DAY IF WE DON'T KNOW IT.** The positivity limiter is the *well-behaved*
one — it is provably **inert** at lake-at-rest, so it does **not** break well-balancedness (**[P]**
`xing-2010-positivity-wb-dg` §3: *"the limiter will not destroy the well-balanced property"*). **What breaks
well-balancedness is the ordinary TVB/slope limiter** — and the WB-ified TVB limiter then **fights** the
positivity limiter:

> **[P]** `xing-2010-positivity-wb-dg` §4: *"numerical tests show that there may be a **conflict between the
> well-balanced TVB limiter** … **and the positivity-preserving limiter** if care is not taken … We may
> observe that **the numerical time step becomes smaller and smaller as time evolves, and eventually the
> code stops.**"*

**That is a runtime death, not a silent inaccuracy.** The fix is a predicate/action split (flag on
`(h+z, hu)` in wet cells, `(h, hu)` in dry ones; always *act* on `(h, hu)`), plus hard-zeroing velocity
below an `h` threshold.

### 8.2 Entropy stability — necessary, NOT sufficient, and the honest limit is BLEAKER than I assumed

**Conservation is not enough:** weak solutions are non-unique, and nothing in a conservative scheme forbids
converging to one that *creates* energy across a discontinuity (the expansion shock — mass and momentum
balance, and time runs backwards). Entropy stability adds the admissibility criterion.

**What it costs — real numbers, from `wintermeyer-2016-entropy-swe` §5 [P]:** the volume kernel goes from
`2(N+1)²` to `2(N+1)³` flux evaluations, and each flux is dearer. Net: **~2.5× on CPU** (with the symmetry
trick), and **~1× on GPU for N ≤ 7**, because the kernel is memory-bound in that range. **That is a cheap
structure relative to what it prevents.**

**⚠ AND NOW THE PART THAT MUST GO IN THE LEDGER, BECAUSE IT IS A STANDING OVERCLAIM IN THE FIELD AND WE
WOULD HAVE MADE IT.** My §8.2 draft said "necessary, not sufficient" as a hedge. It is **worse than a
hedge — it is a theorem-shaped hole**:

> **[P]** Fjordholm, Käppeli, Mishra & Tadmor (2017), *Construction of approximate entropy measure-valued
> solutions…*, Found. Comput. Math. 17(3):763–827, `relata: fjordholm-2017-measure-valued`, §1.2:
> *"Currently, **there are no rigorous proofs of convergence for any kind of finite volume (difference) and
> finite element methods to the entropy solutions of a generic system of conservation laws, even in one
> space dimension.**"*
>
> **[P]** §1.2: *"The only notion of numerical stability … that has been analyzed rigorously so far is that
> of entropy stability… **However, entropy stability may not suffice to ensure the convergence of approximate
> solutions.**"*
>
> **[P]** §1.1: *"recent results … provide counterexamples which illustrate that **entropy solutions for
> multi-dimensional systems of conservation laws are not necessarily unique**. These results **raise serious
> questions about the appropriateness of entropy solutions as the standard solution framework.**"*

And they *demonstrate* it, running their own entropy-stable scheme on Kelvin–Helmholtz: **[P]** *"**there is
no sign of any convergence as the mesh is refined**… the approximate solutions **do not seem to form a Cauchy
sequence in L¹**"* — reproduced across three independent codes.

> ### **⇒ DECLARE THIS PRECISELY. Entropy stability is a ONE-SIDED BOUND, not a selection principle.**
> **It buys:** robustness, non-blow-up, and the **exclusion of energy-creating shocks** — which for a
> *planet sim* means **honest long-time energy budgets**, and that is the one that actually matters to us.
> **It does NOT buy:** convergence to *the* true solution. For **systems** no such guarantee exists at all,
> and for shear-dominated flow entropy-stable schemes **demonstrably fail to converge**.
>
> **A scheme that "preserves the second law" is making a much smaller claim than it sounds like.** Saying
> otherwise would be exactly the overclaim `voice-discipline` exists to prevent.

**On a curved mesh it costs one more thing: the GCL — or the covariant escape (§4.9).**

### 8.3 Well-balanced — a *reconstruction*, not a flux — and it BREAKS in our exact regime

`audusse-2004-hydrostatic` (SISC 25(6):2050–2065) — **hydrostatic reconstruction.** ⚠ **I could not obtain
the full text** (SIAM paywall; HAL/CiteSeerX mirrors 404). What follows is verified against **two
independent primary restatements that agree formula-for-formula** (Ranocha §7.7; Delestre et al. §2.1) —
**the words are theirs, not Audusse's.**

**The minimal change is three pieces, and our summary was missing the third:**
1. Reconstruct on **`(u, h, h+z)`** — the free surface, not the depth.
2. Clip the interface depths: `h_L = max(h₋ + z₋ − max(z₋, z₊), 0)`, symmetrically for `h_R`.
3. **⚠ Add a pressure correction to each side of the flux** — `S_L = (0, ½g(h₋² − h_L²))ᵀ` — **plus** a
   centred source term. **Drop this and you lose consistency and conservation.**

**The structural point, and it is the one the brief was hunting:** well-balancedness is **not a property of
the flux function** — it is a property of **which variable you reconstruct**, and HR is a **wrapper**:
**[P]** Ranocha §7.7: *"This results in a consistent and well-balanced numerical flux that is positivity
preserving and entropy stable, **if the given fluxes … have these properties**."* **HR transports structure
you already have; it cannot manufacture it.**

**⇒ Guarantee: LAKE-AT-REST ONLY.** Moving-water equilibria are a strictly harder object — they need a
**cubic root-find per interface per step**, and per Kurganov's review (`kurganov-2018-swe-review` §5.1.5,
**[P]**) the moving-water-equilibria-preserving schemes are **NOT positivity-preserving.**
**⇒ Proposed: declare moving-water well-balancedness SACRIFICED, explicitly. Don't half-claim it.**

### ⚠ 8.3a THE HYDROSTATIC-RECONSTRUCTION FAILURE MODE THAT LANDS EXACTLY ON US

> **Delestre, O., Cordier, S., Darboux, F. & James, F. (2012).** *A limitation of the hydrostatic
> reconstruction technique for shallow water equations.* C. R. Acad. Sci. Paris 350(13–14):677–681.
> `relata: delestre-2012-hr-limitation` — **READ PRIMARY** (arXiv:1206.4986).
>
> **[P] Proposition 2.1:** *"For a fixed discretization, if for some i₀ ≤ i ≤ i₁ one has **Δz_{i+1/2} ≥
> h_{i+1/2−} ≥ 0** … then the hydrostatic reconstruction will **overestimate (resp. underestimate) the water
> height (resp. velocity)**."*

> ### **`Δz ≥ h` — the bed-step across a face exceeds the local depth — is not an edge case for us. IT IS THE COMMON CASE.**
> **A planet hydrology kernel is thin films on real topography. On any slope steeper than `h/Δx`, this
> fires.** And the authors' consolation is cold comfort for a *multiresolution* code: **[P]** *"the problem
> **disappears when refining the discretization.** However, it has to be taken into account for practical
> computations, **with a fixed discretization**."*
>
> **On an adaptive grid there are ALWAYS coarse cells. There is always a level at which this bites.**
> **[me] This is a BIAS — it systematically overestimates height and underestimates velocity — and it is
> level-dependent, which means it will manifest as a COARSE↔FINE DISAGREEMENT at exactly our seam. It may
> already be in `water.rs`. It is a probe.**

### ⚖ 8.3b WELL-BALANCEDNESS AND POSITIVITY *ARE* PROVED ON A QUADTREE — and the trick is beautiful

> **Ghazizadeh, M. A., Mohammadian, A. & Kurganov, A. (2020).** *An adaptive well-balanced positivity
> preserving central-upwind scheme on quadtree grids for shallow water equations.* Computers & Fluids
> 208:104633. `relata: ghazizadeh-2020-quadtree-wb-pos` — **READ PRIMARY** (arXiv:1911.12002).

They **derive** — not assert, not merely test — well-balancedness *and* positivity **on a quadtree with
hanging nodes.** Two ingredients carry it, and both transfer directly:

1. **⚖ Make the BATHYMETRY representation conforming even though the MESH is not.** **[P]** §3.2: at a fine
   vertex that "is a midpoint of the edge of the neighboring cell… **the point value of B at this vertex is
   an average of the point values of B at those two vertices of the neighboring cell**." **`B` becomes
   single-valued across the seam, so `w − B` cancels identically.**
2. A **composite midpoint rule** on the split face, matching the two half-fluxes term by term.

> ### **THAT IS THE LOAD-BEARING TRICK, AND IT IS THE GENERAL RECIPE FOR CROSSING A SEAM WITH A LINEAR STRUCTURE:**
> ### **make the thing the cancellation depends on CONFORMING, even where the mesh is not.**
>
> **[me] And notice it is §3.2's "the hanging node is a polygon vertex" all over again — a coarse face that
> is subdivided by its finer neighbour must be treated as *actually subdivided*, in the bathymetry too, not
> just in the flux.**

Positivity is likewise proved on the quadtree (§3.7), under `Δt ≤ ¼ · min(Δx/max|a|)`, and **[P]** for
*"not only the forward Euler time discretization, but **any strong stability preserving (SSP) ODE
solver**."* **Honest limits: 2nd-order, Cartesian quadtree (not spherical), no entropy stability.**

---

### 8.4 Symplectic — three findings, and the third rewrites the ante-mundane phase

**The guarantee** (`hairer-2003-geometric`, `hairer-2006-gni-book`) is a **backward-error** result — verified
verbatim from Hairer's own OA writing: for a symplectic (partitioned) Runge–Kutta method of order `r` on an
**analytic** Hamiltonian, **[P]** *"**the Hamiltonian is preserved up to an error of size O(hʳ) on
exponentially long time intervals**"* — specifically `‖H(yₙ) − H(y₀)‖ ≤ C hʳ` for `nh ≤ e^{γ/(2ωh)}`, where
`γ` depends only on the method and **`ω` is the highest frequency in the problem**.

**⇒ [me] Note what `ω` does to us: resolving a MOON (≈27 d) rather than a year (365 d) raises `ω` ~13×,
which shrinks the exponential horizon's exponent ~13×. The Moon is not just a cost centre in step-size — it
degrades the guarantee itself.** ⚠ Inference from the theorem, not a sourced claim.

**FINDING 1 — the adaptivity collision is real, and it has THREE distinct mechanisms** (these get conflated
constantly):

| # | mechanism | source |
|---|---|---|
| 1 | The composed map **is simply not symplectic** when `h` depends on the state. | **[P]** Preto & Tremaine 1999 §1: *"its performance is **no better than that of non-symplectic integrators**; the reason is that the mapping M_{h(z)}(z,t) **is not symplectic** even when M_h(z,t) is."* |
| 2 | Change `h` and you change `H̃`, so the **telescoping cancellation collapses**. | **[me]** — my reasoning from the proof I read. **Not citable as stated.** |
| 3 | Classical error-estimate step control **destroys TIME-SYMMETRY**. | **[P]** Hairer & Söderlind 2005 §1: *"**classical step size strategies destroy these properties** … if step size selection is based on past information, **symmetry breaks down**, because what is 'past' depends on the direction of integration. **No advantage over explicit Runge–Kutta or multistep methods is then left.**"* |

And bluntly, from the man who wrote the book — **[P]** Hairer & Stoffer 1997 §1: *"**At present no such
extension is known for Hamiltonian problems.**"*

**⇒ CONFIRMED. Our seam discipline (`multiscale-seams.md` §3) derives Δt from the `CellId` level — i.e.
step size varies. For an orbital nomos that is precisely the thing that destroys the structure symplectic
integration exists to buy.** ⇒ **Proposed: the orbital nomos runs FIXED-STEP and is EXPLICITLY EXEMPT from
the CellId-derived-Δt rule — an orbit has no spatial grid, so the exemption costs nothing — and the
exemption is DECLARED, not discovered.**

**FINDING 2 — once TIDES are on, symplecticity is formally gone anyway. But SPLITTING is not.**

> **[P]** `rein-2015-ias15` §1: *"When non-conservative forces are included in the equations of motion, **the
> idea of a symplectic integrator — which depends on the system being Hamiltonian — breaks down.**"*
>
> **[P]** `tamayo-2020-reboundx` (REBOUNDx), Abstract: *"…moving to a general framework of **non-commutative
> operators (dissipative or not) clarifies many of these questions, and **several important properties of
> symplectic schemes carry over to the general case**. … explicit **splitting schemes generically exploit
> symmetries in the applied external forces which often strongly suppress integration errors**."*

**⇒ The concept to reach for is not "symplectic." It is SPLITTING.** And three warnings from the same paper,
all **[P]**, all ours:
- Dissipative splitting errors are **systematic** — *"dissipative errors systematically overdamp or
  underdamp"* — **a BIAS, not noise.** (Our own audit vocabulary, again.)
- *"**Splitting methods require a fixed timestep** … and it must be shorter than the fastest timescale …
  **If there is substantial dissipation, the required timestep might be so small that a high-order adaptive
  scheme like IAS15 is more efficient.**"* — **the REBOUND authors will themselves tell us to abandon
  symplectic if the tides are strong.**
- The classic recipes for velocity-dependent forces *"**give qualitatively wrong answers for conservative
  but velocity-dependent forces like post-Newtonian corrections**."* **A live trap if we ever add GR.**

**⚠ And a concrete instrumentation consequence:** with tides on, energy **should** drift (it is physical).
**[P]** `laskar-2004-la2004` §2.3 diagnoses numerical health by first **removing** the physical tidal trend,
and by watching **angular momentum** instead. ⇒ **In a dissipative run, energy conservation is not a valid
error diagnostic. Angular momentum is.** And, universally: **[P]** `rein-2019-hybrid` §4: *"the energy error
does not account for phase errors. **Thus an energy error of zero does not imply a perfect solution.**"*

**⚖ FINDING 3 — AND THIS ONE RESHAPES THE ANTE-MUNDANE PHASE'S *CLAIM*, NOT ITS ALGORITHM.**

**Nobody defends a Gyr trajectory. Nobody.** The inner solar system's Lyapunov time is ~4.3–4.5 Myr, and the
state of the art is explicit:

> **[P]** `laskar-2004-la2004` §1: *"the orbital motion of the planets … **is chaotic, with an exponential
> divergence corresponding to an increase of the error by a factor of 10 every 10 Myr**, thus destroying the
> hope to obtain a precise astronomical solution for paleoclimate studies over more than a few tens of
> millions of years."*
>
> **[P]** §9.1: *"**we will thus consider here that 40 Myr is about the time of validity of our present
> orbital solution for the Earth**… **we are much more limited by the precision of the model than by the
> numerical accuracy.**"*
>
> **[P]** `laskar-2009-collisional` (Nature 459:817): *"**It is thus hopeless to search for a precise solution
> for the motion of the Solar System over 5 Gyr** … **A numerical integration of the Solar System's motion
> over 5 Gyr can thus only be considered as a random sample of its possible evolution.** Statistical studies
> are then required…"*

**And a consequence that should stop us:** *because* the claim is only statistical, **the practitioners
relax fixed-step symplecticity when convenient.** **[P]** Laskar & Gastineau 2009, Methods: *"The step size
is 2.5 × 10⁻² years, **unless the eccentricity of the planets increases beyond about 0.4, in which case the
step size is reduced** to preserve numerical accuracy."* — **a state-dependent step change inside a
symplectic Gyr integration, in *Nature*, by Laskar.** The exponentially-long-time bound is a **Myr-scale
asset**; at Gyr scale, where the trajectory is already conceded, the field spends it without apology.

**⇒ BUT THE FORCING IS FAR MORE ROBUST THAN THE TRAJECTORY, AND THAT IS WHAT WE ACTUALLY NEED:**

> **[P]** `laskar-2004-la2004`, Abstract: *"**the most regular components of the orbital solution could still
> be used over a much longer time span** … we propose to use the term of largest amplitude in the
> eccentricity, related to g₂ − g₅ … corresponding to a **period of 405 000 yr. The uncertainty of this time
> scale over 100 Myr should be about 0.1%, and 0.2% over the full Mesozoic era.**"*

> ### ⚖ **⇒ THE ANTE-MUNDANE PHASE MUST DELIVER A *FORCING SAMPLE*, NOT A *HISTORY*.**
> **"This is one draw from the ensemble of dynamically consistent histories, with the correct forcing
> spectrum" is FULLY DEFENSIBLE OVER Gyr. "This is what the obliquity WAS at t = −1.2 Gyr" is NOT — and no
> integrator, at any price, will make it so.**
>
> ### **AND LOOK WHAT THAT IS. IT IS FATED NOISE.**
> **`determinism-is-ontology` says a vivium's history IS the one fated draw from (seed, key).
> Celestial mechanics says a Gyr integration CAN ONLY BE a random sample of a possible evolution.**
> **These are the same statement.** The literature's epistemic ceiling and vivarium's ontology **agree**,
> and the agreement is not a coincidence — it is the same fact about chaotic systems seen from two sides.
> **The honest claim is the one our own ontology already makes.** **[me]** — but I think this one is right,
> and it is the kind of convergence `discretisation-and-information.md` §3.4 already found once.

**Practical, all [P]:** the Moon is the cost centre (La2004 needs τ = 1.83 d to resolve it; Zeebe gets
4–12 d by treating it as a **quadrupole**; Néron de Surgy & Laskar reached **5 Gyr at a 250-year step** by
going to **secular averaged** equations). **A direct Gyr N-body with a resolved Moon is very likely the wrong
architecture.** For spin/obliquity there are two precedented routes: **Lie–Poisson rigid-body integrators**
(`touma-1994-liepoisson` — the structure preserved is Lie–Poisson, *not* symplectic-canonical) or **averaged
precession/obliquity equations** (the route that actually produced every insolation solution in use).

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
| A6 | **The Zhang–Shu positivity limiter.** Accuracy-free, inert when wet; costs CFL 1/6 vs the 1/5 you already pay. **Use the flag-on-`(h+z,hu)` / act-on-`(h,hu)` split**, or the WB-TVB limiter will fight it and the timestep will collapse to zero. | §8.1 |
| A7 | **Hydrostatic reconstruction** for well-balancedness — reconstruct `h+z`, and **do not omit the pressure-correction term**. **Declare moving-water equilibria SACRIFICED** (they destroy positivity). | §8.3 |
| A8 | **Make the BATHYMETRY conforming across the seam even though the mesh is not.** This is what buys well-balancedness on a quadtree, and it is the general recipe for crossing a seam with a linear structure. | §8.3b |
| A9 | **Fixed-step splitting for the ante-mundane orbital nomos**, **explicitly exempt** from the CellId-Δt rule, **declared**. Reach for **splitting**, not "symplectic" — once tides are on, symplecticity is formally gone but splitting still pays. **Diagnose with angular momentum, not energy.** | §8.4 |
| A10 | **⚖ The ante-mundane phase delivers a FORCING SAMPLE, not a HISTORY.** Nobody defends a Gyr trajectory; everybody defends the forcing spectrum. **And this is what `determinism-is-ontology` already says.** | §8.4 |

### Probe before anything (each is cheap and each could convict us)

| # | probe | why now |
|---|---|---|
| P1 | **The curl probe, on the CURRENT kernel.** | It becomes **vacuous** after a mimetic rewrite. Run it while it can still fail. §7 |
| P2 | **The commutation-relation probe** — does our coarse↔fine operator satisfy `R∘div = div∘R_F`? And relation (3), *uniform PV stays uniform*? | §3.3. We have never asked. |
| P3 | **The discrete GCL probe** — do our cube-sphere metric terms satisfy the metric identities exactly? | §4.4. If not, well-balancedness and entropy stability are **unavailable as theorems**, not merely inaccurate. |
| P4 | **The well-balanced probe.** | §8.3. Cheapest convicting probe we own. |
| P5 | **The two-point-flux-at-a-hanging-node probe.** | §3.2 says it is **non-convergent** even on an orthogonal mesh. Our seam may have a second, independent disease. |
| P6 | **The primal/dual crossing-offset probe under refinement.** | §4.6 — on naively-bisected grids the Laplacian *"does not even achieve first-order convergence."* **We already measure this quantity (we call it non-orthogonality). We have never measured how it behaves as the quadtree refines.** |
| P7 | **The `Δz ≥ h` probe on `water.rs`.** | §8.3a — hydrostatic reconstruction has a *proved* failure mode in exactly our regime (thin films on steep ground), and it is a **level-dependent bias**, so it will show up as a coarse↔fine disagreement at the seam. |

### Derive / build (genuinely ours)

| # | build | why it isn't adoptable |
|---|---|---|
| B1 | **A structure-preserving scheme on a CUBE-SPHERE QUADTREE.** | §5.2 — the empty cell. **Three named tools already exist and nobody has composed them:** **FEEC** (removes the orthogonality requirement; Cotter & Thuburn already name the target — `RT0` on quads, cubed sphere, primal-dual) + **Bochev–Ridzal** (rehabilitates `RT0` on non-affine quads) + the **grid-optimisation pass** (§4.6) + the **mortar projections** for the nonlinear seam (§3.4). |
| B1′ | **⚖ Or the Z GRID.** | §4.4 — a genuinely different answer: stay **collocated**, carry ζ/δ/h, and pay an elliptic solve that our quadtree makes cheap. **Zero architectural cost.** Unmeasured. **This should be spiked before B1 is committed to, because if it works it is far cheaper.** |
| B2 | **Fated-noise closure as the fluctuation half of the FD pair, inside a structure-preserving scheme.** | Nobody does this. It is ours and it is defensible. |
| B3 | **Content-addressed memoised multiresolution store.** | The transform is adopted (§6); the *store* is ours. |

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
- **[⊘] `audusse-2004-hydrostatic` was NOT obtained** (SIAM paywall; HAL/CiteSeerX mirrors 404). §8.3's
  account of hydrostatic reconstruction is verified against **two independent primary restatements that
  agree formula-for-formula** (Ranocha §7.7; Delestre et al. §2.1) — **but the words are theirs, not
  Audusse's.** Do not attribute a verbatim quote to Audusse from this document.
- **[⊘] `staniforth-2012-grids` (QJRMS 138:1–26) was NOT obtained** — genuinely closed access (Unpaywall
  `is_oa: false`; not on CentAUR or Exeter ORE). It is the review everyone cites, and **we have not read
  it.** Its two load-bearing claims reach us *quoted through* Weller 2012, which was read primary. **The
  free substitute, same author, same material: Thuburn, *Computational Modes in Weather and Climate
  Models*, ECMWF Seminar 2013** — read primary, and it is what §4.1–4.2 actually rests on.
- **[⊘]** Zängl et al. 2015 (ICON) and Ringler et al. 2013 (MPAS-Ocean) — closed access, **not read**.
  Nothing here depends on them.
- **[⊘]** `ringler-2010-unified` — read only for the orthogonality requirement and the
  spurious-KE-source sentence (§4.7). **The rest of the paper is unread.**
- **[⊘]** The Girardi–Sweldens result is read-primary, but I did **not** verify that a *quadtree on a
  cube-sphere* satisfies their "nested partitions" hypotheses in the measure-theoretic detail. It looks
  obvious. Obvious is where this project gets caught.
- **[⊘]** The Z-grid lead (§4.4) is **entirely unmeasured**. An elliptic solve per step inside a lazy
  pull-query, content-addressed architecture may be a much worse fit than it looks on paper. **Do not let
  its elegance substitute for a benchmark.**

---

## 11. Sources — with read status, honestly marked

**61 works seeded in `relata`, every one with `bib-fields` verified against its Crossref DOI record** (the
three arXiv preprints against the PDF itself). ✅ = OA PDF obtained and registered in relata.

**The full seeded set, by topic:**

- **Structure-preserving core** — `arnold-2006-feec` · `arnold-2010-feec-hodge` ✅ · `desbrun-2005-dec` ✅ ·
  `arnold-2005-quadhdiv` ✅ · `bochev-2009-rehabilitation` · `lipnikov-2004-nonconformal` ✅ ·
  `warming-1974-modified-equation` · `hairer-2003-geometric` · `hairer-2006-gni-book`
- **The seam** — `balsara-2001-divfree` ✅ · `berger-1989-refluxing` ·
  `friedrich-2018-entropy-nonconforming` · `chan-2021-mortar-entropy`
- **Spherical grids / dycores** — `thuburn-2009-geostrophic` ✅ · `ringler-2010-unified` ·
  `cotter-2014-feec-swe` ✅ · `staniforth-2012-grids` · `arakawa-1981-enstrophy` ·
  `randall-1994-adjustment` · `weller-2012-modes` · `weller-2012-grid-imprinting` ·
  `thuburn-2008-hexagonal-cgrid` · `skamarock-2012-mpas` · `lin-2004-vertically-lagrangian` ·
  `putman-2007-cubed-sphere` · `ferguson-2016-amr-cubed` ✅ · `montoya-2026-covariant-sphere` ✅
- **Multiresolution / wavelets** — `girardi-1997-unbalanced-haar` ✅ · `sweldens-1998-lifting` ✅ ·
  `schroder-1995-spherical` ✅ · `dubos-2013-staggered-wavelet` · `aechtner-2015-wavelet-sphere` ✅ ·
  `kevlahan-2019-wavetrisk` ✅ · `kevlahan-2022-wavetrisk-ocean` ✅
- **WB / entropy / positivity** — `audusse-2004-hydrostatic` · `delestre-2012-hr-limitation` ·
  `kurganov-2018-swe-review` · `xing-2010-positivity-wb-dg` · `zhang-2011-positivity-survey` ·
  `tadmor-1987-numerical-viscosity` · `fjordholm-2011-wb-energy-stable` · `fjordholm-2017-measure-valued` ·
  `wintermeyer-2016-entropy-swe` ✅ · `ranocha-2017-swe-allthree` · `ghazizadeh-2020-quadtree-wb-pos`
- **Symplectic / geometric integration** — `wisdom-1991-symplectic` · `laskar-2001-saba` ·
  `laskar-2004-la2004` · `laskar-2009-collisional` · `touma-1994-liepoisson` · `chambers-1999-hybrid` ·
  `preto-1999-adaptive-symplectic` · `hairer-1997-reversible` · `hairer-2005-explicit` ·
  `rauch-1999-dynamical` · `rein-2015-whfast` · `rein-2015-ias15` · `rein-2019-hybrid` ·
  `tamayo-2020-reboundx` · `lissauer-2012-obliquity` · `zeebe-2017-numerical`

**⚠ Two citations I nearly got wrong, recorded because the mechanism matters more than the fix:**
1. I downloaded **arXiv:1112.4767** believing it to be Thuburn & Cotter's mimetic-framework paper. It is a
   **quantum-optics paper about nitrogen-vacancy centres in diamond.** Caught by checking the title line.
2. I assumed **MWR 140:3220 was "Weller, Thuburn & Cotter."** It is **single-author Hilary Weller.** The
   three-author paper is a *different* one (`weller-2012-grid-imprinting`, MWR 140(8):2734–2755).
3. I assumed **Lin 2004 was the cubed-sphere FV3.** It is a **latitude–longitude** dycore; FV3-on-cubed-
   sphere begins with `putman-2007-cubed-sphere`.

**All three were caught by opening the actual document. None would have been caught by reasoning.**

### READ-STATUS LEDGER — read this before quoting anything from this document

**READ-PRIMARY** *(full text obtained and the cited sections read — every **[P]** quote above comes from one
of these):*
`balsara-2001-divfree` · `lipnikov-2004-nonconformal` · `arnold-2005-quadhdiv` · `arnold-2010-feec-hodge` ·
`desbrun-2005-dec` · `thuburn-2009-geostrophic` · `cotter-2014-feec-swe` · `ferguson-2016-amr-cubed` ·
`montoya-2026-covariant-sphere` · `wintermeyer-2016-entropy-swe` · `aechtner-2015-wavelet-sphere` ·
`kevlahan-2019-wavetrisk` · `kevlahan-2022-wavetrisk-ocean` · `girardi-1997-unbalanced-haar` ·
`sweldens-1998-lifting` · `schroder-1995-spherical` · `arakawa-1981-enstrophy` · `randall-1994-adjustment` ·
`weller-2012-modes` · `thuburn-2008-hexagonal-cgrid` · `skamarock-2012-mpas` ·
`lin-2004-vertically-lagrangian` · `putman-2007-cubed-sphere` · `ranocha-2017-swe-allthree` ·
`ghazizadeh-2020-quadtree-wb-pos` · `friedrich-2018-entropy-nonconforming` · `xing-2010-positivity-wb-dg` ·
`fjordholm-2011-wb-energy-stable` · `fjordholm-2017-measure-valued` · `delestre-2012-hr-limitation` ·
`kurganov-2018-swe-review` · `laskar-2004-la2004` · `laskar-2009-collisional` · `wisdom-1991-symplectic` ·
`touma-1994-liepoisson` · `chambers-1999-hybrid` · `preto-1999-adaptive-symplectic` ·
`hairer-1997-reversible` · `hairer-2005-explicit` · `rein-2015-whfast` · `rein-2015-ias15` ·
`rein-2019-hybrid` · `tamayo-2020-reboundx` · `lissauer-2012-obliquity` · `zeebe-2017-numerical` · plus
**Thuburn, *Computational Modes in Weather and Climate Models*, ECMWF Seminar 2013** (free; **not a formally
citable journal work — not seeded**; it is the substitute for the paywalled `staniforth-2012-grids`) and
**Shu, *Positivity-preserving high order schemes*** (Brown lecture notes; the *citable* forms are
`xing-2010-positivity-wb-dg` and `zhang-2011-positivity-survey`).

**READ-ABSTRACT ONLY** *(do not cite their guarantees from this document):*
`bochev-2009-rehabilitation` · `chan-2021-mortar-entropy` · `zhang-2011-positivity-survey` ·
`tadmor-1987-numerical-viscosity` · `dubos-2013-staggered-wavelet`.

**SEEDED, METADATA-VERIFIED, NOT READ** *(deliberately — the argument does not lean on them):*
`arnold-2006-feec` · `ringler-2010-unified` *(read only for §4.5 and §4.7)* · `staniforth-2012-grids`
**(paywalled — genuinely unread)** · `weller-2012-grid-imprinting` · `berger-1989-refluxing` *(its content
reaches us through Balsara's and Ferguson's descriptions, both read-primary)* ·
`warming-1974-modified-equation` · `hairer-2003-geometric` · `hairer-2006-gni-book` · `laskar-2001-saba` ·
`rauch-1999-dynamical` · `audusse-2004-hydrostatic` **(paywalled — see §10)**.

---

*Author's honesty note. The four claims I would most want attacked, in order:*

***(1)*** *That the ABF non-affine-quadrilateral defect is the same object as our measured two-point-flux
inconsistency, and that Bochev–Ridzal therefore **exonerates an FV scheme** — this is **inference**, it is
load-bearing for §9/B1, and it is checkable on paper. If it is wrong, B1 gets much more expensive.*

***(2)*** *That our exact-nesting quadtree makes the multiresolution problem **strictly easier** than the
published hexagonal case. I believe this firmly and it follows from Aechtner §3 (their hard part is that
hexagons do not nest; ours do) — but it deserves a hostile read.*

***(3)*** *The **Z-grid lead** (§4.4). It is the most exciting thing in the document and therefore the one
most likely to be wrong. It is unmeasured; an elliptic solve inside a lazy, content-addressed pull-query
may be a far worse fit than the elegance suggests. **Do not let it into a design doc before it is spiked.***

***(4)*** *That the WAVETRISK cost economics transfer to us — their compression ratios are for turbulent GCM
flow; ours would be for an observed, mostly-quiescent planet. **I asserted that this helps us. I did not
measure it.***

*And one methodological note that is not about the content. **This document reverses two things I was
confident of at the start**: that the "empty intersection" would be the headline (it is not — WAVETRISK
occupies it, and Montoya 2026 occupies the cube-sphere cell), and that our grid would come out looking
worse (it does not — the DOF ratio is right, uniformity beats orthogonality by measurement, and on the
energy+enstrophy row it is the privileged grid). **Both reversals came from reading the sources rather
than reasoning about them.** That is the whole method, and it is the only reason this is worth anything.*
