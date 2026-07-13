# Bibliography with abstracts — vivarium

*Compiled 2026-07-13. Scope: (1) everything cited by or held in `ref/research/` and the surrounding `ref/` tree; (2) the vivarium-topical subset of `relata` (which holds ~2267 works across all of Joseph's projects — most of it is not vivarium); (3) state-of-the-art and tangential work surfaced by web search and verified against a primary record, including work we hold nothing for.*

**This document is a bibliography and nothing else.** No relevance ratings, no implications, no recommendations. Citation, abstract, availability.

---

## How to read an entry

Every entry carries:

- **Citation** — authors, year, title, venue, volume/pages, DOI. Bibliographic fields come from the relata record, Crossref, or the PDF itself, not from memory.
- **Abstract** — the real abstract, quoted. The source of each abstract is named. Where the abstract could not be obtained, the entry says **ABSTRACT NOT RETRIEVED** rather than paraphrasing the title. There are no reconstructed abstracts in this document.
- **Availability** — one of:
  - **in-repo PDF + markdown-converted** — PDF under `ref/**/pdfs/`, and a markdown conversion under `ref/research/pdfs/markdowns/`
  - **in-repo PDF** — PDF under `ref/**/pdfs/` (git-ignored reading copies)
  - **PDF in relata store** — no repo copy, but the PDF is registered in relata's external store (`relata inspect <key>` reads it)
  - **in relata — metadata only, no PDF held**
  - **not held anywhere** — verified bibliographic record only

Bibkeys are relata keys. `relata show <key>` gives full metadata; `relata inspect <key>` reads the work as markdown where a PDF is registered.

---

## Counts

**Part I — the held corpus (what the repo and relata contain): 242 entries.**

| Area | Entries | in-repo PDF + MD | in-repo PDF | relata + PDF | relata metadata only / not held | abstract not retrieved |
|---|--:|--:|--:|--:|--:|--:|
| **A** — Structure-preserving / compatible / mimetic discretisation (FEEC · DEC · MFD) | 8 | 1 | 0 | 5 | 2 | 0 |
| **B** — Modified-equation & backward-error analysis · geometric numerical integration | 5 | 0 | 0 | 0 | 5 | 1 |
| **C** — Finite volume: entropy stability, positivity, well-balancedness, shallow water | 28 | 0 | 0 | 7 | 21 | 0 |
| **D** — Spherical grids, projections and staggering (cube-sphere · HEALPix · Arakawa · TRiSK) | 18 | 3 | 0 | 3 | 12 | 0 |
| **E** — AMR, refluxing, multiresolution/wavelets, multirate & multiscale methods | 60 | 2 | 0 | 25 | 33 | 1 |
| **F** — Landscape evolution models & drainage routing | 69 | 1 | 4 | 27 | 37 | 2 |
| **G** — Surface hydraulics & sediment transport | 9 | 0 | 2 | 1 | 6 | 2 |
| **H** — Terrain synthesis, erosion authoring & the graphics lineage | 18 | 1 | 2 | 3 | 12 | 1 |
| **I** — Early-Earth geology: continental emergence, cratons, tectonics, oxygenation | 5 | 0 | 5 | 0 | 0 | 0 |
| **J** — Celestial mechanics & long-term orbital forcing | 14 | 0 | 0 | 0 | 14 | 0 |
| **K** — Simulation architecture: incremental computation, determinism, exact arithmetic | 8 | 0 | 0 | 2 | 6 | 0 |
| **TOTAL** | **242** | **8** | **13** | **73** | **148** | **7** |

Abstract provenance across those 242: **126** from the relata record · **38** Crossref · **21** extracted from a PDF we hold (read primary) · **20** fetched from a publisher page / author copy / NASA ADS / PubMed · **17** OpenAlex · **13** Semantic Scholar · **7** not retrieved.

**Part II — state of the art and tangential work from web search: 54 entries**, each verified against a primary record (Crossref / arXiv / publisher page). Mostly work the project holds nothing for — which is a large part of the point. Grouped II-A through II-I along the same subject axis.

**Part III — the four known-wanted items** that `DECISIONS.decision-log.udon` marks `⊘ unread`. All four bibliographic records are now verified; three of the four abstracts obtained.

**Part IV — records that need fixing** (duplicate relata keys, year mismatches, works held but unregistered).

**Total: 300 entries. 282 carry a real abstract; 18 are marked ABSTRACT NOT RETRIEVED, each with the reason stated.** No abstract in this document was paraphrased from a title or reconstructed from memory. One entry (`thuburn-2008-hexagonal-cgrid`) carries its abstract with an explicit **⚠ UNVERIFIED** marker because it came from a search-engine rendering that could not be confirmed against a fetched primary page.

---

# Part I — The held corpus (repo + relata)

*242 entries: everything cited by or held under `ref/`, plus the vivarium-topical subset of relata. Ordered by year within each area.*

---

## A. Structure-preserving / compatible / mimetic discretisation (FEEC · DEC · MFD)

*8 entries.*

### `lipnikov-2004-nonconformal`

Lipnikov, Konstantin, Morel, Jim, Shashkov, Mikhail (2004). *Mimetic finite difference methods for diffusion equations on non-orthogonal non-conformal meshes.* Journal of Computational Physics 199(2):589--597. DOI: [10.1016/j.jcp.2004.02.016](https://doi.org/10.1016/j.jcp.2004.02.016).

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> Mimetic discretizations based on the support-operators methodology are derived for non-orthogonal locally refined quadrilateral meshes. The second-order convergence rate on non-smooth meshes is verified with numerical examples.


### `arnold-2005-quadhdiv`

Arnold, Douglas N., Boffi, Daniele, Falk, Richard S. (2005). *QuadrilateralH(div) Finite Elements.* SIAM Journal on Numerical Analysis 42(6):2429--2451. DOI: [10.1137/s0036142903431924](https://doi.org/10.1137/s0036142903431924).

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We consider the approximation properties of quadrilateral finite element spaces of vector fields defined by the Piola transform, extending results previously obtained for scalar approximation. The finite element spaces are constructed starting with a given finite dimensional space of vector fields on a square reference element, which is then transformed to a space of vector fields on each convex quadrilateral element via the Piola transform associated to a bilinear isomorphism of the square onto the element. For affine isomorphisms, a necessary and sufficient condition for approximation of order r + 1 in L2 is that each component of the given space of functions on the reference element contain all polynomial functions of total degree at most r. In the case of bilinear isomorphisms, the situation is more complicated and we give a precise characterization of what is needed for optimal order L2 -approximation of the function and of its divergence. As applications, we demonstrate degradation of the convergence order on quadrilateral meshes as compared to rectangular meshes for some standard finite element approximations of H(div). We also derive new estimates for approximation by quadrilateral Raviart–Thomas elements (requiring less regularity) and propose a new quadrilateral finite element space which provides optimal order approximation in H(div). Finally, we demonstrate the theory with numerical computations of mixed and least squares finite element approximations of the solution of Poisson's equation.


### `desbrun-2005-dec`

Desbrun, Mathieu, Hirani, Anil N., Leok, Melvin, Marsden, Jerrold E. (2005). *Discrete Exterior Calculus.* <https://arxiv.org/abs/math/0508341>

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We present a theory and applications of discrete exterior calculus on simplicial complexes of arbitrary finite dimension. This can be thought of as calculus on a discrete space. Our theory includes not only discrete differential forms but also discrete vector fields and the operators acting on these objects. This allows us to address the various interactions between forms and vector fields (such as Lie derivatives) which are important in applications. Previous attempts at discrete exterior calculus have addressed only differential forms. We also introduce the notion of a circumcentric dual of a simplicial complex. The importance of dual complexes in this field has been well understood, but previous researchers have used barycentric subdivision or barycentric duals. We show that the use of circumcentric duals is crucial in arriving at a theory of discrete exterior calculus that admits both vector fields and forms.


### `arnold-2006-feec`

Arnold, Douglas N., Falk, Richard S., Winther, Ragnar (2006). *Finite element exterior calculus, homological techniques, and applications.* Acta Numerica 15:1--155. DOI: [10.1017/s0962492906210018](https://doi.org/10.1017/s0962492906210018).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Finite element exterior calculus is an approach to the design and understanding of finite element discretizations for a wide variety of systems of partial differential equations. This approach brings to bear tools from differential geometry, algebraic topology, and homological algebra to develop discretizations which are compatible with the geometric, topological, and algebraic structures which underlie well-posedness of the PDE problem being solved. In the finite element exterior calculus, many finite element spaces are revealed as spaces of piecewise polynomial differential forms. These connect to each other in discrete subcomplexes of elliptic differential complexes, and are also related to the continuous elliptic complex through projections which commute with the complex differential. Applications are made to the finite element discretization of a variety of problems, including the Hodge Laplacian, Maxwell's equations, the equations of elasticity, and elliptic eigenvalue problems, and also to preconditioners.


### `bochev-2009-rehabilitation`

Bochev, Pavel B., Ridzal, Denis (2009). *Rehabilitation of the Lowest-Order Raviart–Thomas Element on Quadrilateral Grids.* SIAM Journal on Numerical Analysis 47(1):487--507. DOI: [10.1137/070704265](https://doi.org/10.1137/070704265).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> A recent study [D. N. Arnold, D. Boffi, and R. S. Falk, SIAM J. Numer. Anal., 42 (2005), pp. 2429–2451] reveals that convergence of finite element methods using $H(\mathrmdiv\,,\Omega)$-compatible finite element spaces deteriorates on nonaffine quadrilateral grids. This phenomena is particularly troublesome for the lowest-order Raviart–Thomas elements, because it implies loss of convergence in some norms for finite element solutions of mixed and least-squares methods. In this paper we propose reformulation of finite element methods, based on the natural mimetic divergence operator [M. Shashkov, Conservative Finite Difference Methods on General Grids, CRC Press, Boca Raton, FL, 1996], which restores the order of convergence. Reformulations of mixed Galerkin and least-squares methods for the Darcy equation illustrate our approach. We prove that reformulated methods converge optimally with respect to a norm involving the mimetic divergence operator. Furthermore, we prove that standard and reformulated versions of the mixed Galerkin method lead to identical linear systems, but the two versions of the least-squares method are veritably different. The surprising conclusion is that the degradation of convergence in the mixed method on nonaffine quadrilateral grids is superficial, and that the lowest-order Raviart–Thomas elements are safe to use in this method. However, the breakdown in the least-squares method is real, and there one should use our proposed reformulation.


### `arnold-2010-feec-hodge`

Arnold, Douglas, Falk, Richard, Winther, Ragnar (2010). *Finite element exterior calculus: from Hodge theory to numerical stability.* Bulletin of the American Mathematical Society 47(2):281--354. DOI: [10.1090/s0273-0979-10-01278-4](https://doi.org/10.1090/s0273-0979-10-01278-4).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> This article reports on the confluence of two streams of research, one emanating from the fields of numerical analysis and scientific computation, the other from topology and geometry. In it we consider the numerical discretization of partial differential equations that are related to differential complexes so that de Rham cohomology and Hodge theory are key tools for exploring the well-posedness of the continuous problem. The discretization methods we consider are finite element methods, in which a variational or weak formulation of the PDE problem is approximated by restricting the trial subspace to an appropriately constructed piecewise polynomial subspace. After a brief introduction to finite element methods, we develop an abstract Hilbert space framework for analyzing the stability and convergence of such discretizations. In this framework, the differential complex is represented by a complex of Hilbert spaces, and stability is obtained by transferring Hodge-theoretic structures that ensure well-posedness of the continuous problem from the continuous level to the discrete. We show stable discretization is achieved if the finite element spaces satisfy two hypotheses: they can be arranged into a subcomplex of this Hilbert complex, and there exists a bounded cochain projection from that complex to the subcomplex. In the next part of the paper, we consider the most canonical example of the abstract theory, in which the Hilbert complex is the de Rham complex of a domain in Euclidean space. We use the Koszul complex to construct two families of finite element differential forms, show that these can be arranged in subcomplexes of the de Rham complex in numerous ways, and for each construct a bounded cochain projection. The abstract theory therefore applies to give the stability and convergence of finite element approximations of the Hodge Laplacian. Other applications are considered as well, especially the elasticity complex and its application to the equations of elasticity. Background material is included to make the presentation self-contained for a variety of readers.


### `cotter-2014-feec-swe`

Cotter, C.J., Thuburn, J. (2014). *A finite element exterior calculus framework for the rotating shallow-water equations.* Journal of Computational Physics 257:1506--1526. DOI: [10.1016/j.jcp.2013.10.008](https://doi.org/10.1016/j.jcp.2013.10.008).

**Availability:** PDF in relata store

**Abstract** *(source: Semantic Scholar)*

> We describe discretisations of the shallow-water equations on the sphere using the framework of finite element exterior calculus, which are extensions of the mimetic finite difference framework presented in Ringler (2010) [11]. The exterior calculus notation provides a guide to which finite element spaces should be used for which physical variables, and unifies a number of desirable properties. We present two formulations: a ''primal'' formulation in which the finite element spaces are defined on a single mesh, and a ''primal-dual'' formulation in which finite element spaces on a dual mesh are also used. Both formulations have velocity and layer depth as prognostic variables, but the exterior calculus framework leads to a conserved diagnostic potential vorticity. In both formulations we show how to construct discretisations that have mass-consistent (constant potential vorticity stays constant), stable and oscillation-free potential vorticity advection.


### `cardiff-2021-thirty`

Cardiff, P., Demirdžić, I. (2021). *Thirty Years of the Finite Volume Method for Solid Mechanics.* Archives of Computational Methods in Engineering 28(5):3721-3780. DOI: [10.1007/s11831-020-09523-0](https://doi.org/10.1007/s11831-020-09523-0).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/cardiff-2021-thirty.pdf`, `ref/research/pdfs/markdowns/` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> Since early publications in the late 1980s and early 1990s, the finite volume method has been shown suitable for solid mechanics analyses. At present, there are several flavours of the method, which can be classified in a variety of ways, such as grid arrangement (cell-centred vs. staggered vs. vertex-centred), solution algorithm (implicit vs. explicit), and stabilisation strategy (Rhie–Chow vs. Jameson–Schmidt–Turkel vs. Godunov upwinding). This article gives an overview, historical perspective, comparison and critical analysis of the different approaches where a close comparison with the de facto standard for computational solid mechanics, the finite element method, is given. The article finishes with a look towards future research directions and steps required for finite volume solid mechanics to achieve more widespread acceptance.



---

## B. Modified-equation & backward-error analysis · geometric numerical integration

*5 entries.*

### `warming-1974-modified-equation`

Warming, R.F, Hyett, B.J (1974). *The modified equation approach to the stability and accuracy analysis of finite-difference methods.* Journal of Computational Physics 14(2):159--179. DOI: [10.1016/0021-9991(74)90011-4](https://doi.org/10.1016/0021-9991(74)90011-4).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract:** ABSTRACT NOT RETRIEVED. Elsevier deposits no abstract for this record to Crossref; Semantic Scholar and OpenAlex both return null; ScienceDirect 403s. The 1974 JCP print abstract exists but could not be reached from any open channel.


### `hairer-1997-reversible`

Hairer, Ernst, Stoffer, Daniel (1997). *Reversible Long-Term Integration with Variable Stepsizes.* SIAM Journal on Scientific Computing 18(1):257--269. DOI: [10.1137/s1064827595285494](https://doi.org/10.1137/s1064827595285494).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> The numerical integration of reversible dynamical systems is considered. A backward analysis for variable stepsize one-step methods is developed, and it is shown that the numerical solution of a symmetric one-step method, implemented with a reversible stepsize strategy, is formally equal to the exact solution of a perturbed differential equation, which again is reversible. This explains geometrical properties of the numerical flow, such as the nearby preservation of invariants. In a second part, the efficiency of symmetric implicit Runge--Kutta methods (linear error growth when applied to integrable systems) is compared with explicit nonsymmetric integrators (quadratic error growth).


### `hairer-2003-geometric`

Hairer, Ernst, Lubich, Christian, Wanner, Gerhard (2003). *Geometric numerical integration illustrated by the Störmer–Verlet method.* Acta Numerica 12:399--450. DOI: [10.1017/s0962492902000144](https://doi.org/10.1017/s0962492902000144).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> The subject of geometric numerical integration deals with numerical integrators that preserve geometric properties of the flow of a differential equation, and it explains how structure preservation leads to improved long-time behaviour. This article illustrates concepts and results of geometric numerical integration on the important example of the Störmer–Verlet method. It thus presents a cross-section of the recent monograph by the authors, enriched by some additional material.After an introduction to the Newton–Störmer–Verlet–leapfrog method and its various interpretations, there follows a discussion of geometric properties: reversibility, symplecticity, volume preservation, and conservation of first integrals. The extension to Hamiltonian systems on manifolds is also described. The theoretical foundation relies on a backward error analysis, which translates the geometric properties of the method into the structure of a modified differential equation, whose flow is nearly identical to the numerical method. Combined with results from perturbation theory, this explains the excellent long-time behaviour of the method: long-time energy conservation, linear error growth and preservation of invariant tori in near-integrable systems, a discrete virial theorem, and preservation of adiabatic invariants.


### `hairer-2005-explicit`

Hairer, Ernst, Söderlind, Gustaf (2005). *Explicit, Time Reversible, Adaptive Step Size Control.* SIAM Journal on Scientific Computing 26(6):1838--1851. DOI: [10.1137/040606995](https://doi.org/10.1137/040606995).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> Adaptive step size control is difficult to combine with geometric numerical integration. As classical step size control is based on "past" information only, time symmetry is destroyed and with it the qualitative properties of the method. In this paper we develop completely explicit, reversible, symmetry-preserving, adaptive step size selection algorithms for geometric numerical integrators such as the Störmer--Verlet method. A new step density controller is proposed and analyzed using backward error analysis and reversible perturbation theory. For integrable reversible systems we show that the resulting adaptive method nearly preserves all action variables and, in particular, the total energy for Hamiltonian systems. It has the same excellent long-term behavior as that obtained when constant steps are used. With variable steps, however, both accuracy and efficiency are greatly improved.


### `hairer-2006-gni-book`

Hairer, Ernst, Lubich, Christian, Wanner, Gerhard (2006). *Geometric Numerical Integration: Structure-Preserving Algorithms for Ordinary Differential Equations.* Springer Series in Computational Mathematics, vol. 31, 2nd ed.. DOI: [10.1007/3-540-30666-8](https://doi.org/10.1007/3-540-30666-8).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Springer PUBLISHER DESCRIPTION (a book — not a journal abstract))*

> Numerical methods that preserve properties of Hamiltonian systems, reversible systems, differential equations on manifolds and problems with highly oscillatory solutions are the subject of this book. A complete self-contained theory of symplectic and symmetric methods, which include Runge-Kutta, composition, splitting, multistep and various specially designed integrators, is presented and their construction and practical merits are discussed. The long-time behaviour of the numerical solutions is studied using a backward error analysis (modified equations) combined with KAM theory. The book is illustrated by many figures, it treats applications from physics and astronomy and contains many numerical experiments and comparisons of different approaches. The second edition is substantially revised and enlarged, with many improvements in the presentation and additions concerning in particular non-canonical Hamiltonian systems, highly oscillatory mechanical systems, and the dynamics of multistep methods.



---

## C. Finite volume: entropy stability, positivity, well-balancedness, shallow-water schemes

*28 entries.*

### `tadmor-1987-numerical-viscosity`

Tadmor, Eitan (1987). *The numerical viscosity of entropy stable schemes for systems of conservation laws. I.* Mathematics of Computation 49(179):91--103. DOI: [10.1090/s0025-5718-1987-0890255-3](https://doi.org/10.1090/s0025-5718-1987-0890255-3).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Discrete approximations to hyperbolic systems of conservation laws are studied. We quantify the amount of numerical viscosity present in such schemes, and relate it to their entropy stability by means of comparison . To this end, conservative schemes which are also entropy conservative are constructed. These entropy conservative schemes enjoy second-order accuracy; moreover, they can be interpreted as piecewise linear finite element methods, and hence can be formulated on various mesh configurations. We then show that conservative schemes are entropy stable, if and—for three-point schemes—only if they contain more viscosity than that present in the above-mentioned entropy conservative ones.


### `audusse-2004-hydrostatic`

Audusse, Emmanuel, Bouchut, François, Bristeau, Marie-Odile, Klein, Rupert, Perthame, Benoit (2004). *A Fast and Stable Well-Balanced Scheme with Hydrostatic Reconstruction for Shallow Water Flows.* SIAM Journal on Scientific Computing 25(6):2050--2065. DOI: [10.1137/s1064827503431090](https://doi.org/10.1137/s1064827503431090).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher record (author repository))*

> We consider the Saint-Venant system for shallow water flows, with nonflat bottom. It is a hyperbolic system of conservation laws that approximately describes various geophysical flows, such as rivers, coastal areas, and oceans when completed with a Coriolis term, or granular flows when completed with friction. Numerical approximate solutions to this system may be generated using conservative finite volume methods, which are known to properly handle shocks and contact discontinuities. However, in general these schemes are known to be quite inaccurate for near steady states, as the structure of their numerical truncation errors is generally not compatible with exact physical steady state conditions. This difficulty can be overcome by using the so-called well-balanced schemes. We describe a general strategy, based on a local hydrostatic reconstruction, that allows us to derive a well-balanced scheme from any given numerical flux for the homogeneous problem. Whenever the initial solver satisfies some classical stability properties, it yields a simple and fast well-balanced scheme that preserves the nonnegativity of the water height and satisfies a semidiscrete entropy inequality.


### `berger-2010-geoclaw`

Berger, M., George, D., LeVeque, R., Mandli, K. (2010). *The GeoClaw software for depth-averaged flows with adaptive refinement.* Advances in Water Resources 34(9):1195--1206. DOI: [10.1016/j.advwatres.2011.02.016](https://doi.org/10.1016/j.advwatres.2011.02.016).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Many geophysical flow or wave propagation problems can be modeled with two-dimensional depth-averaged equations, of which the shallow water equations are the simplest example. We describe the GeoClaw software that has been designed to solve problems of this nature, consisting of open source Fortran programs together with Python tools for the user interface and flow visualization. This software uses high-resolution shock-capturing finite volume methods on logically rectangular grids, including latitude–longitude grids on the sphere. Dry states are handled automatically to model inundation. The code incorporates adaptive mesh refinement to allow the efficient solution of large-scale geophysical problems. Examples are given illustrating its use for modeling tsunamis and dam-break flooding problems. Documentation and download information is available at www.clawpack.org/geoclaw.


### `xing-2010-positivity-wb-dg`

Xing, Yulong, Zhang, Xiangxiong, Shu, Chi-Wang (2010). *Positivity-preserving high order well-balanced discontinuous Galerkin methods for the shallow water equations.* Advances in Water Resources 33(12):1476--1493. DOI: [10.1016/j.advwatres.2010.08.005](https://doi.org/10.1016/j.advwatres.2010.08.005).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: published PDF (author copy, OSU))*

> Shallow water equations with a non-flat bottom topography have been widely used to model flows in rivers and coastal areas. An important difficulty arising in these simulations is the appearance of dry areas where no water is present, as standard numerical methods may fail in the presence of these areas. These equations also have still water steady state solutions in which the flux gradients are nonzero but exactly balanced by the source term. In this paper we propose a high order discontinuous Galerkin method which can maintain the still water steady state exactly, and at the same time can preserve the non-negativity of the water height without loss of mass conservation. A simple positivity-preserving limiter, valid under suitable CFL condition, will be introduced in one dimension and then extended to two dimensions with rectangular meshes. Numerical tests are performed to verify the positivity-preserving property, well-balanced property, high order accuracy, and good resolution for smooth and discontinuous solutions.


### `fjordholm-2011-wb-energy-stable`

Fjordholm, Ulrik S., Mishra, Siddhartha, Tadmor, Eitan (2011). *Well-balanced and energy stable schemes for the shallow water equations with discontinuous topography.* Journal of Computational Physics 230(14):5587--5609. DOI: [10.1016/j.jcp.2011.03.042](https://doi.org/10.1016/j.jcp.2011.03.042).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: author preprint (Tadmor, UMD))*

> We consider the shallow water equations with non-flat bottom topography. The smooth solutions of these equations are energy conservative, whereas weak solutions are energy stable. The equations possess interesting steady states of lake at rest as well as moving equilibrium states. We design energy conservative finite volume schemes which preserve (i) the lake at rest steady state in both one and two space dimensions, and (ii) one-dimensional moving equilibrium states. Suitable energy stable numerical diffusion operators, based on energy and equilibrium variables, are designed to preserve these two types of steady states. Several numerical experiments illustrating the robustness of the energy preserving and energy stable well-balanced schemes are presented.


### `zhang-2011-positivity-survey`

Zhang, Xiangxiong, Shu, Chi-Wang (2011). *Maximum-principle-satisfying and positivity-preserving high-order schemes for conservation laws: survey and new developments.* Proceedings of the Royal Society A: Mathematical, Physical and Engineering Sciences 467(2134):2752--2776. DOI: [10.1098/rspa.2011.0153](https://doi.org/10.1098/rspa.2011.0153).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> In an earlier study (Zhang & Shu 2010 b J. Comput. Phys. 229 , 3091–3120 ( doi:10.1016/j.jcp.2009.12.030 )), genuinely high-order accurate finite volume and discontinuous Galerkin schemes satisfying a strict maximum principle for scalar conservation laws were developed. The main advantages of such schemes are their provable high-order accuracy and their easiness for generalization to multi-dimensions for arbitrarily high-order schemes on structured and unstructured meshes. The same idea can be used to construct high-order schemes preserving the positivity of certain physical quantities, such as density and pressure for compressible Euler equations, water height for shallow water equations and density for Vlasov–Boltzmann transport equations. These schemes have been applied in computational fluid dynamics, computational astronomy and astrophysics, plasma simulation, population models and traffic flow models. In this paper, we first review the main ideas of these maximum-principle-satisfying and positivity-preserving high-order schemes, then present a simpler implementation which will result in a significant reduction of computational cost especially for weighted essentially non-oscillatory finite-volume schemes.


### `delestre-2012-hr-limitation`

Delestre, Olivier, Cordier, Stéphane, Darboux, Frédéric, James, François (2012). *A limitation of the hydrostatic reconstruction technique for Shallow Water equations.* Comptes Rendus. Mathématique 350(13-14):677--681. DOI: [10.1016/j.crma.2012.08.004](https://doi.org/10.1016/j.crma.2012.08.004).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Because of their capability to preserve steady states, well-balanced schemes for Shallow Water equations are becoming popular. Among them, the hydrostatic reconstruction proposed in Audusse et al. (2004) [1], coupled with a positive numerical flux, allows to verify important mathematical and physical properties like the positivity of the water height and, thus, to avoid instabilities when dealing with dry zones. In this note, we prove that this method exhibits an abnormal behavior for some combinations of slope, mesh size and water height.


### `fjordholm-2017-measure-valued`

Fjordholm, Ulrik S., Käppeli, Roger, Mishra, Siddhartha, Tadmor, Eitan (2015). *Construction of Approximate Entropy Measure-Valued Solutions for Hyperbolic Systems of Conservation Laws.* Foundations of Computational Mathematics 17(3):763--827. DOI: [10.1007/s10208-015-9299-z](https://doi.org/10.1007/s10208-015-9299-z).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> Entropy solutions have been widely accepted as the suitable solution framework for systems of conservation laws in several space dimensions. However, recent results in De Lellis and Székelyhidi Jr (Ann Math 170(3):1417–1436, 2009) and Chiodaroli et al. (2013) have demonstrated that entropy solutions may not be unique. In this paper, we present numerical evidence that state-of-the-art numerical schemes need not converge to an entropy solution of systems of conservation laws as the mesh is refined. Combining these two facts, we argue that entropy solutions may not be suitable as a solution framework for systems of conservation laws, particularly in several space dimensions. We advocate entropy measure-valued solutions, first proposed by DiPerna, as the appropriate solution paradigm for systems of conservation laws. To this end, we present a detailed numerical procedure which constructs stable approximations to entropy measure-valued solutions, and provide sufficient conditions that guarantee that these approximations converge to an entropy measure-valued solution as the mesh is refined, thus providing a viable numerical framework for systems of conservation laws in several space dimensions. A large number of numerical experiments that illustrate the proposed paradigm are presented and are utilized to examine several interesting properties of the computed entropy measure-valued solutions.


### `mccorquodale-2015-adaptive`

McCorquodale, P., Ullrich, Paul A., Johansen, H., Colella, P. (2015). *An adaptive multiblock high-order finite-volume method for solving the shallow-water equations on the sphere.* Communications in Applied Mathematics and Computational Science 10(2):121--162. DOI: [10.2140/camcos.2015.10.121](https://doi.org/10.2140/camcos.2015.10.121).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> We present a high-order finite-volume approach for solving the shallow-water equations on the sphere, using multiblock grids on the cubed-sphere. This approach combines a Runge--Kutta time discretization with a fourth-order accurate spatial discretization, and includes adaptive mesh refinement and refinement in time. Results of tests show fourth-order convergence for the shallow-water equations as well as for advection in a highly deformational flow. Hierarchical adaptive mesh refinement allows solution error to be achieved that is comparable to that obtained with uniform resolution of the most refined level of the hierarchy, but with many fewer operations.


### `popinet-2015-quadtree-adaptive`

Popinet, S. (2015). *A quadtree-adaptive multigrid solver for the Serre-Green-Naghdi equations.* Journal of Computational Physics 302:336--358. DOI: [10.1016/j.jcp.2015.09.009](https://doi.org/10.1016/j.jcp.2015.09.009).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: NASA ADS (bibcode 2015JCoPh.302..336P))*

> The Serre-Green-Naghdi (SGN) equations, also known as the fully-nonlinear Boussinesq wave equations, accurately describe the behaviour of dispersive shoaling water waves. This article presents and validates a novel combination of methods for the numerical approximation of solutions to the SGN equations. The approach preserves the robustness of the original finite-volume Saint-Venant solver, in particular for the treatment of wetting/drying and equilibrium states. The linear system of coupled vector equations governing the dispersive SGN momentum sources is solved simply and efficiently using a generic multigrid solver. This approach generalises automatically to adaptive quadtree meshes. Adaptive mesh refinement is shown to provide orders-of-magnitude gains in speed and memory when applied to the dispersive propagation of waves during the Tohoku tsunami. The source code, test cases and examples are freely available.


### `ranocha-2017-swe-allthree`

Ranocha, Hendrik (2016). *Shallow water equations: split-form, entropy stable, well-balanced, and positivity preserving numerical methods.* GEM - International Journal on Geomathematics 8(1):85--133. DOI: [10.1007/s13137-016-0089-9](https://doi.org/10.1007/s13137-016-0089-9).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> For the first time, a general two-parameter family of entropy conservative numerical fluxes for the shallow water equations is developed and investigated. These are adapted to a varying bottom topography in a well-balanced way, i.e. preserving the lake-at-rest steady state. Furthermore, these fluxes are used to create entropy stable and well-balanced split-form semidiscretisations based on general summation-by-parts (SBP) operators, including Gauß nodes. Moreover, positivity preservation is ensured using the framework of Zhang and Shu (Proc R Soc Lond A Math Phys Eng Sci 467: 2752–2776, 2011). Therefore, the new two-parameter family of entropy conservative fluxes is enhanced by dissipation operators and investigated with respect to positivity preservation. Additionally, some known entropy stable and positive numerical fluxes are compared. Furthermore, finite volume subcells adapted to nodal SBP bases with diagonal mass matrix are used. Finally, numerical tests of the proposed schemes are performed and some conclusions are presented.


### `wintermeyer-2016-entropy-swe`

Wintermeyer, Niklas, Winters, Andrew R., Gassner, Gregor J., Kopriva, David A. (2017). *An Entropy Stable Nodal Discontinuous Galerkin Method for the Two Dimensional Shallow Water Equations on Unstructured Curvilinear Meshes with Discontinuous Bathymetry.* <https://arxiv.org/abs/1509.07096>

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We design an arbitrary high-order accurate nodal discontinuous Galerkin spectral element approximation for the nonlinear two dimensional shallow water equations with non-constant, possibly discontinuous, bathymetry on unstructured, possibly curved, quadrilateral meshes. The scheme is derived from an equivalent flux differencing formulation of the split form of the equations. We prove that this discretisation exactly preserves the local mass and momentum. Furthermore, combined with a special numerical interface flux function, the method exactly preserves the mathematical entropy, which is the total energy for the shallow water equations. By adding a specific form of interface dissipation to the baseline entropy conserving scheme we create a provably entropy stable scheme. That is, the numerical scheme discretely satisfies the second law of thermodynamics. Finally, with a particular discretisation of the bathymetry source term we prove that the numerical approximation is well-balanced. We provide numerical examples that verify the theoretical findings and furthermore provide an application of the scheme for a partial break of a curved dam test problem.


### `friedrich-2018-entropy-nonconforming`

Friedrich, Lucas, Winters, Andrew R., Del Rey Fernández, David C., Gassner, Gregor J., Parsani, Matteo, Carpenter, Mark H. (2018). *An Entropy Stable h / p Non-Conforming Discontinuous Galerkin Method with the Summation-by-Parts Property.* Journal of Scientific Computing 77(2):689--725. DOI: [10.1007/s10915-018-0733-7](https://doi.org/10.1007/s10915-018-0733-7).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> This work presents an entropy stable discontinuous Galerkin (DG) spectral element approximation for systems of non-linear conservation laws with general geometric (h) and polynomial order (p) non-conforming rectangular meshes. The crux of the proofs presented is that the nodal DG method is constructed with the collocated Legendre–Gauss–Lobatto nodes. This choice ensures that the derivative/mass matrix pair is a summation-by-parts (SBP) operator such that entropy stability proofs from the continuous analysis are discretely mimicked. Special attention is given to the coupling between non-conforming elements as we demonstrate that the standard mortar approach for DG methods does not guarantee entropy stability for non-linear problems, which can lead to instabilities. As such, we describe a precise procedure and modify the mortar method to guarantee entropy stability for general non-linear hyperbolic systems on h / p non-conforming meshes. We verify the high-order accuracy and the entropy conservation/stability of fully non-conforming approximation with numerical examples.


### `kurganov-2018-swe-review`

Kurganov, Alexander (2018). *Finite-volume schemes for shallow-water equations.* Acta Numerica 27:289--351. DOI: [10.1017/s0962492918000028](https://doi.org/10.1017/s0962492918000028).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Shallow-water equations are widely used to model water flow in rivers, lakes, reservoirs, coastal areas, and other situations in which the water depth is much smaller than the horizontal length scale of motion. The classical shallow-water equations, the Saint-Venant system, were originally proposed about 150 years ago and still are used in a variety of applications. For many practical purposes, it is extremely important to have an accurate, efficient and robust numerical solver for the Saint-Venant system and related models. As their solutions are typically non-smooth and even discontinuous, finite-volume schemes are among the most popular tools. In this paper, we review such schemes and focus on one of the simplest (yet highly accurate and robust) methods: central-upwind schemes. These schemes belong to the family of Godunov-type Riemann-problem-solver-free central schemes, but incorporate some upwinding information about the local speeds of propagation, which helps to reduce an excessive amount of numerical diffusion typically present in classical (staggered) non-oscillatory central schemes. Besides the classical one- and two-dimensional Saint-Venant systems, we will consider the shallow-water equations with friction terms, models with moving bottom topography, the two-layer shallow-water system as well as general non-conservative hyperbolic systems.


### `ghazizadeh-2020-quadtree-wb-pos`

Ghazizadeh, Mohammad A., Mohammadian, Abdolmajid, Kurganov, Alexander (2020). *An adaptive well-balanced positivity preserving central-upwind scheme on quadtree grids for shallow water equations.* Computers & Fluids 208:104633. DOI: [10.1016/j.compfluid.2020.104633](https://doi.org/10.1016/j.compfluid.2020.104633).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> We present an adaptive well-balanced positivity preserving central-upwind scheme on quadtree grids for shallow water equations. The use of quadtree grids results in a robust, efficient and highly accurate numerical method. The quadtree model is developed based on the well-balanced positivity preserving central-upwind scheme proposed in [A. Kurganov and G. Petrova, Commun. Math. Sci., 5 (2007), pp. 133--160]. The designed scheme is well-balanced in the sense that it is capable of exactly preserving "lake-at-rest" steady states. In order to achieve this as well as to preserve positivity of water depth, a continuous piecewise bilinear interpolation of the bottom topography function is utilized. This makes the proposed scheme capable of modelling flows over discontinuous bottom topography. Local gradients are examined to determine new seeding points in grid refinement for the next timestep. Numerical examples demonstrate the promising performance of the central-upwind quadtree scheme.


### `berger-2021-towards`

Berger, M., LeVeque, R. (2021). *Towards Adaptive Simulations of Dispersive Tsunami Propagation from an Asteroid Impact.* abs/2110.01420. <https://www.semanticscholar.org/paper/55c38a0628664d7ee644bdb683fa2ce4b9809200>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> The long-term goal of this work is the development of high-fidelity simulation tools for dispersive tsunami propagation. A dispersive model is especially important for short wavelength phenomena such as an asteroid impact into the ocean, and is also important in modeling other events where the simpler shallow water equations are insufficient. Adaptive simulations are crucial to bridge the scales from deep ocean to inundation, but have difficulties with the implicit system of equations that results from dispersive models. We propose a fractional step scheme that advances the solution on separate patches with different spatial resolutions and time steps. We show a simulation with 7 levels of adaptive meshes and onshore inundation resulting from a simulated asteroid impact off the coast of Washington. Finally, we discuss a number of open research questions that need to be resolved for high quality simulations.


### `chan-2021-mortar-entropy`

Chan, Jesse, Bencomo, Mario J., Del Rey Fernández, David C. (2021). *Mortar-based Entropy-Stable Discontinuous Galerkin Methods on Non-conforming Quadrilateral and Hexahedral Meshes.* Journal of Scientific Computing 89(2). DOI: [10.1007/s10915-021-01652-3](https://doi.org/10.1007/s10915-021-01652-3).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> High-order entropy-stable discontinuous Galerkin (DG) methods for nonlinear conservation laws reproduce a discrete entropy inequality by combining entropy conservative finite volume fluxes with summation-by-parts (SBP) discretization matrices. In the DG context, on tensor product (quadrilateral and hexahedral) elements, SBP matrices are typically constructed by collocating at Lobatto quadrature points. Recent work has extended the construction of entropy-stable DG schemes to collocation at more accurate Gauss quadrature points (Chan et al. in SIAM J Sci Comput 41(5):A2938–A2966, 2019) . In this work, we extend entropy-stable Gauss collocation schemes to non-conforming meshes. Entropy-stable DG schemes require computing entropy conservative numerical fluxes between volume and surface quadrature nodes. On conforming tensor product meshes where volume and surface nodes are aligned, flux evaluations are required only between "lines" of nodes. However, on non-conforming meshes, volume and surface nodes are no longer aligned, resulting in a larger number of flux evaluations. We reduce this expense by introducing an entropy-stable mortar-based treatment of non-conforming interfaces via a face-local correction term, and provide necessary conditions for high-order accuracy. Numerical experiments for the compressible Euler equations in two and three dimensions confirm the stability and accuracy of this approach.


### `huang-2022-adaptive`

Huang, Pei, Chen, Chungang, Li, Xingliang, Shen, Xueshun, Xiao, F. (2022). *An adaptive nonhydrostatic dynamical core using a multimoment finite‐volume method on a cubed sphere.* Quarterly Journal of the Royal Meteorological Society 148(749):3814--3831. DOI: [10.1002/qj.4389](https://doi.org/10.1002/qj.4389).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract: An adaptive nonhydrostatic atmospheric dynamical core was developed on a Cartesian grid and extended to spherical geometry with the application of a cubed‐sphere grid. To assure a practical Courant‐Friedrichs‐Lewy number for stability, a horizontally explicit and vertically implicit algorithm was applied in this model through a third‐order implicit–explicit Runge–Kutta scheme. A two‐dimensional adaptive grid was applied in the horizontal directions to generate the computational meshes, with variable resolutions according to the evolution of the predicted variables during the simulations, while the vertical grid is fixed considering the characteristics of atmospheric flows and the computational efficiency in practical applications. The Berger–Oliger block‐structured adaptive algorithm was adopted in this study. Blocks with different resolutions can be constructed straightforwardly on each patch of the cubed sphere and an algorithm to exchange both solution and block information between adjacent patches was designed in order to implement a global model. The nonhydrostatic governing equations are solved by a three‐point multimoment constrained finite‐volume scheme. Using the compact stencil for spatial reconstructions, the interpolation operations between coarse–fine blocks can be implemented efficiently and the local‐based scheme is also helpful to suppress the computational modes around the coarse–fine interfaces due to the abrupt change of grid resolution. Additionally, flux corrections were conducted along the block boundaries and the resulting dynamical core is rigorously conservative. The proposed model was evaluated by calculating several idealized benchmark tests, and the effectiveness of the adaptive model in saving computational costs was verified in this study.


### `huang-2022-adaptive-nonhydrostatic`

Huang, Pei, Chen, Chungang, Li, Xingliang, Shen, Xueshun, Xiao, F. (2022). *An Adaptive Nonhydrostatic Atmospheric Dynamical Core Using a Multi-Moment Constrained Finite Volume Method.* Advances in Atmospheric Sciences 39(3):1--15. DOI: [10.1007/s00376-021-1185-9](https://doi.org/10.1007/s00376-021-1185-9).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: journal article page (Advances in Atmospheric Sciences))*

> An adaptive 2D nonhydrostatic dynamical core is proposed by using the multi-moment constrained finite-volume (MCV) scheme and the Berger-Oliger adaptive mesh refinement (AMR) algorithm. The MCV scheme takes several point-wise values within each computational cell as the predicted variables to build high-order schemes based on single-cell reconstruction. Two types of moments, such as the volume-integrated average (VIA) and point value (PV), are defined as constraint conditions to derive the updating formulations of the unknowns, and the constraint condition on VIA guarantees the rigorous conservation of the proposed model. In this study, the MCV scheme is implemented on a height-based, terrain-following grid with variable resolution to solve the nonhydrostatic governing equations of atmospheric dynamics. The AMR grid of Berger-Oliger consists of several groups of blocks with different resolutions, where the MCV model developed on a fixed structured mesh can be used directly. Numerical formulations are designed to implement the coarse-fine interpolation and the flux correction for properly exchanging the solution information among different blocks. Widely used benchmark tests are carried out to evaluate the proposed model. The numerical experiments on uniform and AMR grids indicate that the adaptive model has promising potential for improving computational efficiency without losing accuracy.


### `avgerinos-2023-semi-implicit`

Avgerinos, S., Castro, M., Macca, E., Russo, G. (2023). *A semi-implicit finite volume method for the Exner model of sediment transport.* abs/2303.03801. DOI: [10.48550/arxiv.2303.03801](https://doi.org/10.48550/arxiv.2303.03801).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> The aim of this work is to construct efficient finite volume schemes for the numerical study of sediment transport in shallow water, in the framework of the Exner model.In most cases, the velocity related to the sediment is much lower that the fluid velocity, which, in turn, may be much lower that the free-surface wave speed. Explicit methods that resolve all waves require small time steps due to the CFL stability restriction because of fast surface waves. Furthermore, if Rusanov flux is adopted, slow sediment waves may be affected by the large numerical diffusion. The objective of the present work is to drastically improve the efficiency in the computation of the evolution of the sediment by treating water waves implicitly, thus allowing much larger time steps than the one required by fully explicit schemes. The goal is reached by suitably semi-implicit schemes obtained by the use of implicit-explicit Runge-Kutta methods.


### `berger-2023-implicit`

Berger, M., LeVeque, R. (2023). *Implicit Adaptive Mesh Refinement for Dispersive Tsunami Propagation.* SIAM Journal on Scientific Computing 46(4):554-. DOI: [10.1137/23m1585210](https://doi.org/10.1137/23m1585210).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We present an algorithm to solve the dispersive depth-averaged Serre-Green-Naghdi (SGN) equations using patch-based adaptive mesh refinement. These equations require adding additional higher derivative terms to the nonlinear shallow water equations. This has been implemented as a new component of the open source GeoClaw software that is widely used for modeling tsunamis, storm surge, and related hazards, improving its accuracy on shorter wavelength phenomena. We use a formulation that requires solving an elliptic system of equations at each time step, making the method implicit. The adaptive algorithm allows different time steps on different refinement levels, and solves the implicit equations level by level. Computational examples are presented to illustrate the stability and accuracy on a radially symmetric test case and two realistic tsunami modeling problems, including a hypothetical asteroid impact creating a short wavelength tsunami for which dispersive terms are necessary.


### `but-2024-adaptive`

But, I., Kiryushina, M. A., Elistratov, S., Elizarova, T., Tiniakov, A. D. (2024). *An Adaptive Mesh Refinement Solver for Regularized Shallow Water Equations.* Computational Mathematics and Information Technologies 8(2):9-23. DOI: [10.23947/2587-8999-2024-8-2-9-23](https://doi.org/10.23947/2587-8999-2024-8-2-9-23).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Introduction. We present a novel adaptive mesh refinement (AMR) solver, SWqgdAMR, based on the open software platform AMReX. The new solver is grounded in regularized shallow water equations. This paper details the equations, their discretization, and implementation features within AMReX. The efficacy of SWqgdAMR is demonstrated through two test cases: a two-dimensional circular dam break (collapse of a liquid column) and the collapse of two liquid columns of different heights.Materials and Methods. The SWqgdAMR solver is developed to extend the applicability of regularized equations in problems requiring high computational power and adaptive grids. SWqgdAMR is the first solver based on the quasigas dynamic (QGD) algorithm within the AMReX framework. The implementation and validation of SWqgdAMR represent a crucial step towards the further expansion of the QGD software suite.Results. The AMReX-based shallow water equations solver SWqgdAMR with adaptive mesh refinement is described and tested in detail. Validation of SWqgdAMR involved two-dimensional problems: the breach of a cylindrical dam and the breach of two cylindrical dams of different heights. The presented solver demonstrated high efficiency, with the use of adaptive mesh refinement technology accelerating the computation by 56 times compared to a stationary grid calculation.Discussion and Conclusions. The algorithm can be expanded to include bathymetry, external forces (wind force, bottom friction, Coriolis forces), and the mobility of the shoreline during wetting and drying phases, as has been done in individual codes for regularized shallow water equations (RSWE). The current implementation of the QGD algorithm did not test the potential for parallel computing on graphical cores.


### `ishimwe-2024-multi-scale`

Ishimwe, A. P., Deleersnijder, É., Legat, V., Lambrechts, J. (2024). *A multi-scale IMEX second order Runge-Kutta method for 3D hydrodynamic ocean models.* Journal of Computational Physics 520:113482. DOI: [10.1016/j.jcp.2024.113482](https://doi.org/10.1016/j.jcp.2024.113482).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: open-access copy (UCLouvain DIAL repository))*

> Understanding complex physical phenomena often involves dealing with partial differential equations (PDEs) where different phenomena exhibit distinct timescales. Fast terms, associated with short characteristic times, coexist with slower ones requiring relatively longer time steps for resolution. The challenge becomes more manageable when, despite the varying characteristic times of fast and slow terms, the computational cost associated with faster terms is significantly lower than that of slower terms. Additionally, slower terms can also exhibit two distinct longer characteristic times, adding complexity to the system and resulting in a total of three characteristic timescales. In this paper, an innovative split second-order IMEX (IMplicit-EXplicit) temporal scheme is introduced to address this temporal complexity. It is used to solve the primitive equation ocean model. Extremely short times are handled explicitly with small time steps, while longer timescales are managed explicitly and semi-implicitly using larger time steps. The decision to solve a portion of the slower terms semi-implicitly is due to the fact that it does not significantly increase the total computational cost, allowing for greater flexibility in the time step without imposing a substantial burden on the overall computational efficiency. This strategy enables efficient management of the various temporal scales present in the equations, thereby optimizing computational resources. The proposed scheme is applied to solve 3D hydrodynamics equations encompassing three time scale: fast terms representing wave phenomena, slow terms describing horizontal aspects and stiff terms for vertical ones. Furthermore, the scheme is designed to respect crucial physical properties, namely global and local conservation. The obtained results on different test cases demonstrate the robustness and efficiency of the IMEX approach in simulating these complex systems.


### `fernndeznieto-2025-third-order`

Fernández-Nieto, E., Garres-Díaz, J., Macca, E., Russo, G. (2025). *A Third-Order Finite Volume Semi-Implicit Method for the Shallow Water-Exner Model.* Journal of Scientific Computing 105(3). DOI: [10.1007/s10915-025-03093-8](https://doi.org/10.1007/s10915-025-03093-8).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> In this work, third-order semi-implicit schemes on staggered meshes for the shallow water and Saint-Venant-Exner systems are presented. They are based on a third-order extension of the technique introduced in Casulli & Cheng [15]. The stability conditions for these schemes depend on the velocity and not on the celerity, allowing us to reduce computational efforts, especially in subcritical flow simulations, which is the regime we are mainly interested in. The main novelty consists in the third-order approximation of the pressure gradient term in the momentum equation through appropriate polynomial reconstructions. Concretely, CWENO conservative reconstruction is considered for the water thickness h and a centered fourth-degree polynomial is adopted interpolating the cell averages of the free surface η\documentclass[12pt]minimal \usepackageamsmath \usepackagewasysym \usepackageamsfonts \usepackageamssymb \usepackageamsbsy \usepackagemathrsfs \usepackageupgreek \setlength\oddsidemargin-69pt \begindocument$$\eta $$\enddocument. For time discretization, a third-order IMEX scheme is applied. In addition, a novel time-dependent semi-analytical solution for Saint-Venant-Exner system is introduced and compared with the numerical ones. Several tests are performed, including accuracy tests showing third-order accuracy, well-balance tests, and simulations of slow bedload processes for large time.


### `macca-2025-semi-implicit`

Macca, E., Boscheri, W., Ricchiuto, Mario (2025). *Semi-implicit strategies for the Serre-Green-Naghdi equations in hyperbolic form. Is hyperbolic relaxation really a good idea?.* abs/2510.07539. DOI: [10.48550/arxiv.2510.07539](https://doi.org/10.48550/arxiv.2510.07539).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> The Serre-Green-Naghdi (SGN) equations provide a valuable framework for modelling fully nonlinear and weakly dispersive shallow-water flows. However, their elliptic formulation can considerably increase the computational cost compared to the Saint-Venant equations. To overcome this difficulty, hyperbolic models (hSGN) have been proposed that replace the elliptic operators with first-order hyperbolic formulations augmented by relaxation terms, which recover the original elliptic formulation in the stiff limit. Yet, as the relaxation parameter \lambda increases, explicit schemes face restrictive stability constraints that may offset these advantages. To mitigate this limitation, we introduce a semi-implicit (SI) integration strategy for the hSGN system, where the stiff acoustic terms are treated implicitly within an IMEX Runge-Kutta framework, while the advective components remain explicit. The proposed approach mitigates the CFL stability restriction and maintains dispersive accuracy at a moderate computational cost. Numerical results confirm that the combination of hyperbolization and semi-implicit time integration provides an efficient and accurate alternative to both classical SGN and fully explicit hSGN solvers.


### `parvin-2025-hyperbolic`

Parvin, Afroja, Samaey, G., Koellermeier, Julian (2025). *A Hyperbolic Moment Based Shallow Water Model for Coupled Bedload Suspended Load Morphodynamics with Variable Density.* <https://www.semanticscholar.org/paper/81ae32b04092222b32c88b03efec0c845be2b14f>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> In this paper, we develop the Hyperbolic Shallow Water Exner Moment model with Erosion and Deposition (HSWEMED), extending the shallow water moment framework to capture coupled morphodynamics with erosion and deposition. HSWEMED introduces a suspended-sediment concentration equation, couples concentration-dependent mixture density with the momentum and higher-order moment equations, and includes source terms due to erosion and deposition. Starting from the incompressible Navier-Stokes equations for a water-sediment mixture, we derive a coupled system consisting of the shallow water equations, moment equations for polynomial velocity coefficients, a depth-averaged suspended-sediment equation, and an Exner equation for bedload transport with erosion-deposition coupling. Although the transported scalar is depth-averaged, we reconstruct a low-order vertical concentration profile consistent with the moment representation of velocity, providing the near-bed concentration needed in the closure. We prove hyperbolicity through hyperbolic regularization and derive dissipative energy balance relations for lower-order models. Numerical results are obtained with a path-conservative finite-volume scheme based on a Lax-Friedrichs-type flux. Several dam-break tests, including wet/dry front cases, are validated against laboratory experiments, showing improved accuracy over existing shallow water moment models. The proposed HSWEMED provides a mathematically well-posed and computationally efficient framework for morphodynamic simulations.


### `montoya-2026-covariant-sphere`

Montoya, Tristan, Rueda-Ram\'irez, Andr\'es M., Gassner, Gregor J. (2026). *Entropy-stable discontinuous spectral-element methods for the spherical shallow water equations in covariant form.* <https://arxiv.org/abs/2509.08790>

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We introduce discontinuous spectral-element methods of arbitrary order that are well balanced, conservative of mass, and conservative or dissipative of total energy (i.e., a mathematical entropy function) for a covariant flux formulation of the rotating shallow water equations with variable bottom topography on curved manifolds such as the sphere. The proposed methods are based on a skew-symmetric splitting of the tensor divergence in covariant form, which we implement and analyze within a general flux-differencing framework using tensor-product summation-by-parts operators. Such schemes are proven to satisfy semi-discrete mass and energy conservation on general unstructured quadrilateral grids in addition to well balancing for arbitrary continuous bottom topographies, with energy dissipation resulting from a suitable choice of numerical interface flux. Furthermore, the proposed covariant formulation permits an analytical representation of the geometry and associated metric terms while satisfying the aforementioned entropy stability, conservation, and well-balancing properties without the need to approximate the metric terms so as to enforce discrete metric identities. Numerical experiments on cubed-sphere grids are presented in order to verify the schemes' structure-preservation properties as well as to assess their accuracy and robustness within the context of several standard test cases characteristic of idealized atmospheric flows. Our theoretical and numerical results support the further development of the proposed methodology towards a full dynamical core for numerical weather prediction and climate modelling, as well as broader applications to other hyperbolic and advection-dominated systems of partial differential equations on curved manifolds.


### `muozmoncayo-2026-adaptive`

Muñoz-Moncayo, Carlos, Ketcheson, D. (2026). *Adaptive, efficient, and scalable water wave modeling with dispersive hyperbolic systems.* <https://www.semanticscholar.org/paper/14af6ba549c4e363840d5029c6c919278e42d754>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Accurate modeling of tsunamis (such as those generated by landslides) requires capturing both wave dispersion in the deep ocean and wave breaking near the shore. The shallow water equations are often preferred for working with tsunamis, but neglect dispersion and may be inaccurate in scenarios where dispersive effects are significant. In this work, we develop an approach that seeks to incorporate the best aspects of both hyperbolic and dispersive models by combining either of two hyperbolic reformulations of the Serre-Green-Naghdi equations away from the shore with the non-dispersive shallow water equations near the shore. The model is discretized and implemented within the GeoClaw software, and incorporates adaptive mesh refinement as well as shared-memory parallelism. We validate it through comparison with benchmarks and real tsunami data. The results and performance compare favorably with the existing dispersive water wave solvers, including a speedup of about 2x relative to GeoClaw's existing dispersive solver for a large-scale tsunami simulation.



---

## D. Spherical grids, projections and staggering (cube-sphere · HEALPix · Arakawa · TRiSK)

*18 entries.*

### `arakawa-1981-enstrophy`

Arakawa, Akio, Lamb, Vivian R. (1981). *A Potential Enstrophy and Energy Conserving Scheme for the Shallow Water Equations.* Monthly Weather Review 109(1):18--36. DOI: [10.1175/1520-0493(1981)109<0018:apeaec>2.0.co;2](https://doi.org/10.1175/1520-0493(1981)109<0018:apeaec>2.0.co;2).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> To improve the simulation of nonlinear aspects of the flow over steep topography, a potential enstrophy and energy conserving scheme for the shallow water equations is derived. It is pointed out that a family of schemes can conserve total energy for general flow and potential enstrophy for flow with no mass flux divergence. The newly derived scheme is a unique member of this family, that conserves both potential enstrophy and energy for general flow. Comparison by means of numerical experiment with a scheme that conserves (potential) enstrophy for purely horizontal nondivergent flow demonstrated the considerable superiority of the newly derived potential enstrophy and energy conserving scheme, not only in suppressing a spurious energy cascade but also in determining the overall flow regime. The potential enstrophy and energy conserving scheme for a spherical grid is also presented.


### `snyder-1992-equal-area`

John P Snyder (1992). *An Equal-Area Map Projection For Polyhedral Globes.* Cartographica 29(1):10-21. DOI: [10.3138/27H7-8K88-4882-1752](https://doi.org/10.3138/27H7-8K88-4882-1752).

**Availability:** **not held anywhere** — not in relata, no PDF (cited repeatedly in `DECISIONS.decision-log.udon` for the equal-area cube-sphere Table 1)

**Abstract** *(source: Crossref)*

> Numerous polyhedral shapes have been proposed as approximations for globes, and the projection most often used is the Gnomonic, with considerable scale and area distortion. Complicated conformal projections have been designed, but an equal-area projection has been used only once, for the icosahedron. The Lambert Azimuthal Equal-Area projection can be modified to provide an exactly fitting, perfectly equal-area projection for any polyhedral globe that has regular polygons, but is most satisfactory for the dodecahedron with 12 pentagons and for the truncated icosahedron with 20 hexagons and 12 pentagons. On the application to the truncated icosahedron, the angular deformation does not exceed 3.75°, and the scale variation is less than 3.3 percent. These advantages are at the expense of increased interruptions at the polygon edges when the polyhedral globe is unfolded. On a proposé de nombreuses formes polyédriques comme approximation de globes et la projection gnomonique est la plus souvent utilisée, avec de considérables distortions d'échelles et de superficies. On a conçu des projections conformes compliquées, mais on n'a utilisé qu'une seule fois une projection équivalente pour l'icosaèdre. La projection équivalente azimutale de Lambert peut être modifiée pour permettre un ajustement exact, parfaitement équivalent pour tout globe polyédrique dote de polygones réguliers, mais elle est plus que satisfaisante pour le dodécaèdre à douze pentagones et pour l'icosaèdre tronqué à vingt hexagones et douze pentagones. Dans l'application de l'icosaèdre tronqué, la déformation angulaire ne dépasse pas 3.75° et la variation d'échelle est inférieure à 3.3 pour cent. Ces avantages s'obtiennent aux dépens d'une augmentation des interruptions aux limites des polygones, lorsque l'on déplie le globe polyédrique.


### `randall-1994-adjustment`

Randall, David A. (1994). *Geostrophic Adjustment and the Finite-Difference Shallow-Water Equations.* Monthly Weather Review 122(6):1371--1377. DOI: [10.1175/1520-0493(1994)122<1371:gaatfd>2.0.co;2](https://doi.org/10.1175/1520-0493(1994)122<1371:gaatfd>2.0.co;2).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> Numerical simulation of geostrophic adjustment in shallow water is discussed for the case of an unstaggered grid for vorticity, divergence, and mass. The dispersion equation is shown to be very well behaved and superior to that obtained with the Arakawa grids A–E.


### `gorski-2004-healpix`

Gorski, K. M., Hivon, E., Banday, A. J., Wandelt, B. D., Hansen, F. K., Reinecke, M., et al. (2004). *HEALPix -- a Framework for High Resolution Discretization, and Fast Analysis of Data Distributed on the Sphere.*

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/astro-ph0409513.pdf`, `ref/research/pdfs/markdowns/` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> HEALPix – the Hierarchical Equal Area iso-Latitude Pixelization – is a versatile data structure with an associated library of computational algorithms and visualization software that supports fast scientific applications executable directly on very large volumes of astronomical data and large area surveys in the form of discretized spherical maps. Originally developed to address the data processing and analysis needs of the present generation of cosmic microwave background (CMB) experiments (e.g. BOOMERanG, WMAP), HEALPix can be expanded to meet many of the profound challenges that will arise in confrontation with the observational output of future missions and experiments, including e.g. Planck, Herschel, SAFIR, and the Beyond Einstein CMB polarization probe. In this paper we consider the requirements and constraints to be met in order to implement a sufficient framework for the efficient discretization and fast analysis/synthesis of functions defined on the sphere, and summarise how they are satisfied by HEALPix.


### `lin-2004-vertically-lagrangian`

Lin, Shian-Jiann (2004). *A Vertically Lagrangian Finite-Volume Dynamical Core for Global Models.* Monthly Weather Review 132(10):2293--2307. DOI: [10.1175/1520-0493(2004)132<2293:avlfdc>2.0.co;2](https://doi.org/10.1175/1520-0493(2004)132<2293:avlfdc>2.0.co;2).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> A finite-volume dynamical core with a terrain-following Lagrangian control-volume discretization is described. The vertically Lagrangian discretization reduces the dimensionality of the physical problem from three to two with the resulting dynamical system closely resembling that of the shallow water system. The 2D horizontalto-Lagrangian-surface transport and dynamical processes are then discretized using the genuinely conservative flux-form semi-Lagrangian algorithm. Time marching is split-explicit, with large time steps for scalar transport, and small fractional steps for the Lagrangian dynamics, which permits the accurate propagation of fast waves. A mass, momentum, and total energy conserving algorithm is developed for remapping the state variables periodically from the floating Lagrangian control-volume to an Eulerian terrain-following coordinate for dealing with ''physical parameterizations'' and to prevent severe distortion of the Lagrangian surfaces. Deterministic baroclinic wave-growth tests and long-term integrations using the Held-Suarez forcing are presented. Impact of the monotonicity constraint is discussed.


### `putman-2007-cubed-sphere`

Putman, William M., Lin, Shian-Jiann (2007). *Finite-volume transport on various cubed-sphere grids.* Journal of Computational Physics 227(1):55--78. DOI: [10.1016/j.jcp.2007.07.022](https://doi.org/10.1016/j.jcp.2007.07.022).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: published PDF (NASA GSFC author copy))*

> The performance of a multidimensional finite-volume transport scheme is evaluated on the cubed-sphere geometry. Advection tests with prescribed winds are used to evaluate a variety of cubed-sphere projections and grid modifications including the gnomonic and conformal mappings, as well as two numerically generated grids by an elliptic solver and spring dynamics. We explore the impact of grid non-orthogonality on advection tests over the corner singularities of the cubed-sphere grids, using some variations of the transport scheme, including the piecewise parabolic method with alternative monotonicity constraints. The advection tests revealed comparable or better accuracy to those of the original latitudinal-longitudinal grid implementation. It is found that slight deviations from orthogonality on the modified cubed-sphere (quasi-orthogonal) grids do not negatively impact the accuracy. In fact, the more uniform version of the quasi-orthogonal cubed-sphere grids provided better overall accuracy than the most orthogonal (and therefore, much less uniform) conformal grid. It is also shown that a simple non-orthogonal extension to the transport equation enables the use of the highly non-orthogonal and computationally more efficient gnomonic grid with acceptable accuracy.


### `thuburn-2008-hexagonal-cgrid`

Thuburn, J. (2008). *Numerical wave propagation on the hexagonal C-grid.* Journal of Computational Physics 227(11):5836--5858. DOI: [10.1016/j.jcp.2008.02.010](https://doi.org/10.1016/j.jcp.2008.02.010).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: ⚠ UNVERIFIED — reproduced from a search-engine rendering of the publisher abstract; every direct fetch (ScienceDirect, Exeter ORE, Semantic Scholar) returned 403 or empty, so this text is NOT confirmed verbatim against a primary page)*

> Inertio-gravity mode and Rossby mode dispersion properties are examined for discretizations of the linearized rotating shallow water equations on a regular hexagonal C-grid in planar geometry. It is shown that spurious non-zero Rossby mode frequencies found by previous authors in the f-plane case can be avoided by an appropriate discretization of the Coriolis terms. Three generalizations of this discretization that conserve energy even for non-constant Coriolis parameter are presented. A quasigeostrophic beta-plane analysis is carried out to investigate the Rossby mode dispersion properties of these three schemes. The Rossby mode dispersion relation is found to have two branches. The primary branch modes are good approximations, in terms of both structure and frequency, to corresponding modes of the continuous governing equations, and offer some improvements over a quadrilateral C-grid scheme.


### `thuburn-2009-geostrophic`

Thuburn, J., Ringler, T.D., Skamarock, W.C., Klemp, J.B. (2009). *Numerical representation of geostrophic modes on arbitrarily structured C-grids.* Journal of Computational Physics 228(22):8321--8335. DOI: [10.1016/j.jcp.2009.08.006](https://doi.org/10.1016/j.jcp.2009.08.006).

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> A C-grid staggering, in which the mass variable is stored at cell centers and the normal velocity component is stored at cell faces (or edges in two dimensions) is attractive for atmospheric modeling since it enables a relatively accurate representation of fast wave modes. However, the discretization of the Coriolis terms is non-trivial. For constant Coriolis parameter, the linearized shallow water equations support geostrophic modes: stationary solutions in geostrophic balance. A naive discretization of the Coriolis terms can cause geostrophic modes to become non-stationary, causing unphysical behaviour of numerical solutions. Recent work has shown how to discretize the Coriolis terms on a planar regular hexagonal grid to ensure that geostrophic modes are stationary while the Coriolis terms remain energy conserving. In this paper this result is extended to arbitrarily structured C-grids. An explicit formula is given for constructing an appropriate discretization of the Coriolis terms. The general formula is illustrated by showing that it recovers previously known results for the planar regular hexagonal C-grid and the spherical longitude–latitude C-grid. Numerical calculation confirms that the scheme does indeed give stationary geostrophic modes for the hexagonal–pentagonal and triangular geodesic C-grids on the sphere.


### `ringler-2010-unified`

Ringler, T.D., Thuburn, J., Klemp, J.B., Skamarock, W.C. (2010). *A unified approach to energy conservation and potential vorticity dynamics for arbitrarily-structured C-grids.* Journal of Computational Physics 229(9):3065--3090. DOI: [10.1016/j.jcp.2009.12.007](https://doi.org/10.1016/j.jcp.2009.12.007).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: NASA ADS (bibcode 2010JCoPh.229.3065R))*

> A numerical scheme applicable to arbitrarily-structured C-grids is presented for the nonlinear shallow-water equations. By discretizing the vector-invariant form of the momentum equation, the relationship between the nonlinear Coriolis force and the potential vorticity flux can be used to guarantee that mass, velocity and potential vorticity evolve in a consistent and compatible manner. Underpinning the consistency and compatibility of the discrete system is the construction of an auxiliary thickness equation that is staggered from the primary thickness equation and collocated with the vorticity field. The numerical scheme also exhibits conservation of total energy to within time-truncation error. Simulations of the standard shallow-water test cases confirm the analysis and show convergence rates between 1st- and 2nd-order accuracy when discretizing the system with quasi-uniform spherical Voronoi diagrams. The numerical method is applicable to a wide class of meshes, including latitude-longitude grids, Voronoi diagrams, Delaunay triangulations and conformally-mapped cubed-sphere meshes.


### `chen-2011-adaptive`

Chen, Chungang, Xiao, F., Li, Xingliang (2011). *An Adaptive Multimoment Global Model on a Cubed Sphere.* Monthly Weather Review 139(2):523--548. DOI: [10.1175/2010mwr3365.1](https://doi.org/10.1175/2010mwr3365.1).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> An adaptive global shallow-water model is proposed on cubed-sphere grid using the multimoment finite volume scheme and the Berger-Oliger adaptive mesh refinement (AMR) algorithm that was originally designed for a Cartesian grid. On each patch of the cubed-sphere grid, the curvilinear coordinates are constructed in a way that the Berger-Oliger algorithm can be applied directly. Moreover, an algorithm to transfer data across neighboring patches is proposed to establish a practical integrated framework for global AMR computation on the cubed-sphere grid. The multimoment finite volume scheme is adopted as the fluid solver and is essentially beneficial to the implementation of AMR on the cubed-sphere grid. The multimoment interpolation based on both volume-integrated average (VIA) and point value (PV) provides the compact reconstruction that makes the present scheme very attractive not only in dealing with the artificial boundaries between different patches but also in the coarse fine interpolations required in the AMR computations. The single-cell-based reconstruction avoids involving more than two nesting levels during interpolations. The reconstruction profile of constrained interpolation profile-conservative semi-Lagrangian scheme with third-order polynomial function (CIP-CSL3) is adopted where the slope parameter provides a flexible and convenient switching to get the desired numerical properties, such as high-order (fourth order) accuracy, monotonicity, and positive definiteness. Numerical experiments with typical benchmark tests for both advection equation and shallow-water equations are carried out to evaluate the proposed model. The numerical errors and the corresponding CPU times of numerical experiments on uniform and adaptive meshes verify the performance of the proposed model. Compared to the uniformly refined grid, the AMR technique is able to achieve the similar numerical accuracy with less computational cost.


### `staniforth-2012-grids`

Staniforth, Andrew, Thuburn, John (2011). *Horizontal grids for global weather and climate prediction models: a review.* Quarterly Journal of the Royal Meteorological Society 138(662):1--26. DOI: [10.1002/qj.958](https://doi.org/10.1002/qj.958).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> A latitude–longitude grid is used by almost all operational atmospheric forecast models, and many research models. However, it is expected that the advantages of a latitude–longitude grid will become outweighed on massively parallel computers by data‐communication bottlenecks. There is therefore renewed interest in quasi‐uniform alternatives. This review surveys and assesses previously proposed horizontal grids for modelling the atmosphere over the sphere. Aspects of numerical accuracy likely to be affected by grid structure are discussed; particular attention is paid to computational modes and grid imprinting. Computational modes are potentially very serious, since they may be excited in realistic applications by boundary conditions, nonlinearity, physical forcing, and data assimilation. The geometry of polyhedra is reviewed due to its relation to numerical degrees of freedom, and hence to numerical wave dispersion and the possible existence of computational modes.All grids proposed to date have known problems or issues that merit further investigation. Orthogonal logically rectangular grids may be generated using conformal maps, but these suffer from singularities and resolution clustering. Resolution clustering may be avoided by using overset grids, but there are potential issues associated with the overlap regions. Alternatively, resolution clustering may be avoided, whilst retaining a logically rectangular grid, by giving up orthogonality; however, existing numerical schemes exploit orthogonality to obtain various properties thought to be important for accuracy, and it is not yet known whether these can also be obtained on non‐orthogonal grids. Quasi‐uniformity and orthogonality can be obtained without resolution clustering or overlaps by using non‐quadrilateral grid cells, such as triangles, or pentagons and hexagons. However, when a staggered placement of variables is used to minimise dispersion errors for fast waves, non‐quadrilateral grids support computational modes.In view of the lack of a single ideal grid, several topics meriting further investigation are identified. Copyright © 2011 Royal Meteorological Society and British Crown Copyright, the Met Office


### `skamarock-2012-mpas`

Skamarock, William C., Klemp, Joseph B., Duda, Michael G., Fowler, Laura D., Park, Sang-Hun, Ringler, Todd D. (2012). *A Multiscale Nonhydrostatic Atmospheric Model Using Centroidal Voronoi Tesselations and C-Grid Staggering.* Monthly Weather Review 140(9):3090--3105. DOI: [10.1175/mwr-d-11-00215.1](https://doi.org/10.1175/mwr-d-11-00215.1).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> The formulation of a fully compressible nonhydrostatic atmospheric model called the Model for Prediction Across Scales–Atmosphere (MPAS-A) is described. The solver is discretized using centroidal Voronoi meshes and a C-grid staggering of the prognostic variables, and it incorporates a split-explicit time-integration technique used in many existing nonhydrostatic meso- and cloud-scale models. MPAS can be applied to the globe, over limited areas of the globe, and on Cartesian planes. The Voronoi meshes are unstructured grids that permit variable horizontal resolution. These meshes allow for applications beyond uniform-resolution NWP and climate prediction, in particular allowing embedded high-resolution regions to be used for regional NWP and regional climate applications. The rationales for aspects of this formulation are discussed, and results from tests for nonhydrostatic flows on Cartesian planes and for large-scale flow on the sphere are presented. The results indicate that the solver is as accurate as existing nonhydrostatic solvers for nonhydrostatic-scale flows, and has accuracy comparable to existing global models using icosahedral (hexagonal) meshes for large-scale flows in idealized tests. Preliminary full-physics forecast results indicate that the solver formulation is robust and that the variable-resolution-mesh solutions are well resolved and exhibit no obvious problems in the mesh-transition zones.


### `weller-2012-grid-imprinting`

Weller, Hilary, Thuburn, John, Cotter, Colin J. (2012). *Computational Modes and Grid Imprinting on Five Quasi-Uniform Spherical C Grids.* Monthly Weather Review 140(8):2734--2755. DOI: [10.1175/mwr-d-11-00193.1](https://doi.org/10.1175/mwr-d-11-00193.1).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Currently, most operational forecasting models use latitude–longitude grids, whose convergence of meridians toward the poles limits parallel scaling. Quasi-uniform grids might avoid this limitation. Thuburn et al. and Ringler et al. have developed a method for arbitrarily structured, orthogonal C grids called TRiSK, which has many of the desirable properties of the C grid on latitude–longitude grids but which works on a variety of quasi-uniform grids. Here, five quasi-uniform, orthogonal grids of the sphere are investigated using TRiSK to solve the shallow-water equations. Some of the advantages and disadvantages of the hexagonal and triangular icosahedra, a "Voronoi-ized" cubed sphere, a Voronoi-ized skipped latitude–longitude grid, and a grid of kites in comparison to a full latitude–longitude grid are demonstrated. It is shown that the hexagonal icosahedron gives the most accurate results (for least computational cost). All of the grids suffer from spurious computational modes; this is especially true of the kite grid, despite it having exactly twice as many velocity degrees of freedom as height degrees of freedom. However, the computational modes are easiest to control on the hexagonal icosahedron since they consist of vorticity oscillations on the dual grid that can be controlled using a diffusive advection scheme for potential vorticity.


### `weller-2012-modes`

Weller, Hilary (2012). *Controlling the Computational Modes of the Arbitrarily Structured C Grid.* Monthly Weather Review 140(10):3220--3234. DOI: [10.1175/mwr-d-11-00221.1](https://doi.org/10.1175/mwr-d-11-00221.1).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> The arbitrarily structured C grid, Thuburn–Ringler–Skamarock–Klemp (TRiSK), is being used in the Model for Prediction Across Scales (MPAS) and is being considered by the Met Office for their next dynamical core. However, the hexagonal C grid supports a branch of spurious Rossby modes, which lead to erroneous grid-scale oscillations of potential vorticity (PV). It is shown how these modes can be harmlessly controlled by using upwind-biased interpolation schemes for PV. A number of existing advection schemes for PV are tested, including that used in MPAS, and none are found to give adequate results for all grids and all cases. Therefore a new scheme is proposed; continuous, linear-upwind stabilized transport (CLUST), a blend between centered and linear-upwind with the blend dependent on the flow direction with respect to the cell edge. A diagnostic of grid-scale oscillations is proposed that gives further discrimination between schemes than using potential enstrophy alone. Indeed, some schemes are found to destroy potential enstrophy while grid-scale oscillations grow. CLUST performs well on hexagonal-icosahedral grids and unrotated skipped latitude–longitude grids of the sphere for various shallow-water test cases. Despite the computational modes, the hexagonal icosahedral grid performs well since these modes are easy and harmless to filter. As a result, TRiSK appears to perform better than a spectral shallow-water model.


### `westerteiger-2012-spherical`

Westerteiger, Rolf, Gerndt, Andreas, Hamann, Bernd (2012). *Spherical Terrain Rendering using the hierarchical HEALPix grid.* DOI: [10.4230/oasics.vluds.2011.13](https://doi.org/10.4230/oasics.vluds.2011.13).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/Spherical_Terrain_Rendering_using_the_hierarchical.pdf`, `ref/research/pdfs/markdowns/` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We present an interactive spherical terrain rendering system employing a hierarchical subdivision of the HEALPix coordinate system using quadtrees. Compared to other parameterizations, the scheme avoids singularities and allows for efficient fusion of mixed-resolution digital elevation models and imagery. A Level-of-Detail heuristic is used to guarantee both high performance and visual fidelity. Unified treatment of DEM and imagery data is achieved by performing the HEALPix projection within a GPU shader. The system is applied to the exploration of Mars, using both MOLA (NASA) and HRSC (German Aerospace Center) data sets.


### `ferguson-2016-amr-cubed`

Ferguson, Jared O., Jablonowski, Christiane, Johansen, Hans, McCorquodale, Peter, Colella, Phillip, Ullrich, Paul A. (2016). *Analyzing the Adaptive Mesh Refinement (AMR) Characteristics of a High-Order 2D Cubed-Sphere Shallow-Water Model.* Monthly Weather Review 144(12):4641--4666. DOI: [10.1175/mwr-d-16-0197.1](https://doi.org/10.1175/mwr-d-16-0197.1).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> Adaptive mesh refinement (AMR) is a technique that has been featured only sporadically in atmospheric science literature. This paper aims to demonstrate the utility of AMR for simulating atmospheric flows. Several test cases are implemented in a 2D shallow-water model on the sphere using the Chombo-AMR dynamical core. This high-order finite-volume model implements adaptive refinement in both space and time on a cubed-sphere grid using a mapped-multiblock mesh technique. The tests consist of the passive advection of a tracer around moving vortices, a steady-state geostrophic flow, an unsteady solid-body rotation, a gravity wave impinging on a mountain, and the interaction of binary vortices. Both static and dynamic refinements are analyzed to determine the strengths and weaknesses of AMR in both complex flows with small-scale features and large-scale smooth flows. The different test cases required different AMR criteria, such as vorticity or height-gradient based thresholds, in order to achieve the best accuracy for cost. The simulations show that the model can accurately resolve key local features without requiring global high-resolution grids. The adaptive grids are able to track features of interest reliably without inducing noise or visible distortions at the coarse–fine interfaces. Furthermore, the AMR grids keep any degradations of the large-scale smooth flows to a minimum.


### `ferguson-2016-analyzing`

Ferguson, J. (2016). *Analyzing the Adaptive Mesh Refinement (AMR) Characteristics of a High-Order 2D Cubed-Sphere Shallow-Water Model.* Monthly Weather Review 144(12):4641--4666. DOI: [10.1175/mwr-d-16-0197.1](https://doi.org/10.1175/mwr-d-16-0197.1).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> AbstractAdaptive mesh refinement (AMR) is a technique that has been featured only sporadically in atmospheric science literature. This paper aims to demonstrate the utility of AMR for simulating atmospheric flows. Several test cases are implemented in a 2D shallow-water model on the sphere using the Chombo-AMR dynamical core. This high-order finite-volume model implements adaptive refinement in both space and time on a cubed-sphere grid using a mapped-multiblock mesh technique. The tests consist of the passive advection of a tracer around moving vortices, a steady-state geostrophic flow, an unsteady solid-body rotation, a gravity wave impinging on a mountain, and the interaction of binary vortices. Both static and dynamic refinements are analyzed to determine the strengths and weaknesses of AMR in both complex flows with small-scale features and large-scale smooth flows. The different test cases required different AMR criteria, such as vorticity or height-gradient based thresholds, in order to achieve the...


### `youngren-2017-multiresolution`

Youngren, Robert W., Petty, Mikel D. (2017). *A multi-resolution HEALPix data structure for spherically mapped point data.* Heliyon 3(6):e00332. DOI: [10.1016/j.heliyon.2017.e00332](https://doi.org/10.1016/j.heliyon.2017.e00332).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/1-s2.0-S2405844017304966-main.pdf`; **not in relata**

**Abstract** *(source: extracted from the PDF we hold)*

> Data describing entities with locations that are points on a sphere are described as spherically mapped. Several data structures designed for spherically mapped data have been developed. One of them, known as Hierarchical Equal Area iso-Latitude Pixelization (HEALPix), partitions the sphere into twelve diamond-shaped equalarea base cells and then recursively subdivides each cell into four diamond-shaped subcells, continuing to the desired level of resolution. Twelve quadtrees, one associated with each base cell, store the data records associated with that cell and its subcells. HEALPix has been used successfully for numerous applications, notably including cosmic microwave background data analysis. However, for applications involving sparse point data HEALPix has possible drawbacks, including inefficient memory utilization, overwriting of proximate points, and return of spurious points for certain queries. A multi-resolution variant of HEALPix specifically optimized for sparse point data was developed. The new data structure allows different areas of the sphere to be subdivided at different levels of resolution. It combines HEALPix positive features with the advantages of multi-resolution, including reduced memory requirements and improved query performance. An implementation of the new Multi-Resolution HEALPix (MRH) data structure was tested using spherically mapped data from four different scientific applications



---

## E. AMR, refluxing, multiresolution/wavelets, multirate & multiscale methods

*60 entries.*

### `brandt-1977-multilevel`

Brandt, Achi (1977). *Multi-Level Adaptive Solutions to Boundary-Value Problems.* Mathematics of Computation 31(138):333--390. DOI: [10.1090/S0025-5718-1977-0431719-X](https://doi.org/10.1090/S0025-5718-1977-0431719-X).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> The boundary-value problem is discretized on several grids (or finite-element spaces) of widely different mesh sizes. Interactions between these levels enable us (i) to solve the possibly nonlinear system of n discrete equations in O ( n ) O(n) operations (40 n additions and shifts for Poisson problems); (ii) to conveniently adapt the discretization (the local mesh size, local order of approximation, etc.) to the evolving solution in a nearly optimal way, obtaining " ∞ \infty -order" approximations and low n , even when singularities are present. General theoretical analysis of the numerical process. Numerical experiments with linear and nonlinear, elliptic and mixed-type (transonic flow) problems-confirm theoretical predictions. Similar techniques for initial-value problems are briefly discussed.


### `berger-1984-adaptive`

Berger, Marsha J., Oliger, Joseph (1984). *Adaptive Mesh Refinement for Hyperbolic Partial Differential Equations.* Journal of Computational Physics 53(3):484--512. DOI: [10.1016/0021-9991(84)90073-1](https://doi.org/10.1016/0021-9991(84)90073-1).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/berger-1984-adaptive.pdf`, `ref/research/pdfs/markdowns/` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> An adaptive method based on the idea of multiple component grids for the solution of hyperbolic partial differential equations using finite difference techniques is presented. Based upon Richardson-type estimates of the truncation error, refined grids are created or existing ones removed to attain a given accuracy for a minimum amount of work. The approach is recursive in that tine grids can contain even finer grids. The grids with finer mesh width in space also have a smaller mesh width in time. making this a mesh refinement algorithm in time and space. We present the algorithm, error estimation procedure, and the data structures, and conclude with numerical examples in one and two space dimensions.


### `gear-1984-multirate`

Gear, C. William, Wells, D. R. (1984). *Multirate Linear Multistep Methods.* BIT Numerical Mathematics 24:484--502. DOI: [10.1007/BF01934907](https://doi.org/10.1007/BF01934907).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/gear-1984-multirate.pdf`, `ref/research/pdfs/markdowns/` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> The design of a code which uses different stepsizes for different components of a system of ordinary differential equations is discussed. Methods are suggested which achieve moderate efficiency for problems having some components with a much slower rate of variation than others. Techniques for estimating errors in the different components are analyzed and applied to automatic stepsize and order control. Difficulties, absent from non-multirate methods, arise in the automatic selection of stepsizes, leading to a suggested organization of the code that is counter-intuitive. An experimental code and some initial experiments are described.


### `berger-1989-refluxing`

Berger, M.J., Colella, P. (1989). *Local adaptive mesh refinement for shock hydrodynamics.* Journal of Computational Physics 82(1):64--84. DOI: [10.1016/0021-9991(89)90035-1](https://doi.org/10.1016/0021-9991(89)90035-1).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: published PDF)*

> The aim of this work is the development of an automatic, adaptive mesh refinement strategy for solving hyperbolic conservation laws in two dimensions. There are two main difficulties in doing this. The first problem is due to the presence of discontinuities in the solution and the effect on them of discontinuities in the mesh. The second problem is how to organize the algorithm to minimize memory and CPU overhead. This is an important consideration and will continue to be important as more sophisticated algorithms that use data structures other than arrays are developed for use on vector and parallel computers.


### `schroder-1995-spherical`

Schröder, Peter, Sweldens, Wim (1995). *Spherical wavelets.* Proceedings of the 22nd annual conference on Computer graphics and interactive techniques - SIGGRAPH '95:161--172. DOI: [10.1145/218380.218439](https://doi.org/10.1145/218380.218439).

**Availability:** PDF in relata store

**Abstract** *(source: OpenAlex)*

> Article Spherical wavelets: efficiently representing functions on the sphere Share on Authors: Peter Schröder Department of Mathematics, University of South Carolina Department of Mathematics, University of South CarolinaView Profile , Wim Sweldens Department of Mathematics, Department of Computer Science, Katholieke Universiteit Leuven, Belgium Department of Mathematics, Department of Computer Science, Katholieke Universiteit Leuven, BelgiumView Profile Authors Info & Claims SIGGRAPH '95: Proceedings of the 22nd annual conference on Computer graphics and interactive techniquesSeptember 1995 Pages 161–172https://doi.org/10.1145/218380.218439Online:15 September 1995Publication History 338citation2,133DownloadsMetricsTotal Citations338Total Downloads2,133Last 12 Months44Last 6 weeks8 Get Citation AlertsNew Citation Alert added!This alert has been successfully added and will be sent to:You will be notified whenever a record that you have chosen has been cited.To manage your alert preferences, click on the button below.Manage my AlertsNew Citation Alert!Please log in to your account Save to BinderSave to BinderCreate a New BinderNameCancelCreateExport CitationPublisher SiteGet Access


### `minion-1996-projection`

Minion, M. (1996). *A Projection Method for Locally Refined Grids.* Journal of Computational Physics 127(1):158--178. DOI: [10.1006/jcph.1996.0166](https://doi.org/10.1006/jcph.1996.0166).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> A numerical method for the solution of the two-dimensional Euler equatons for incompressible flow on locally refined grids is presented. The method is a second-order Godunov-projection method adapted from Bell, Colella, and Glaz. Second-order accuracy of the numerical method in time and space is established through numerical experiments. The main contributions of this work concern the formulation and implementation of a projection for refined grids. A discussion of the adjointness relation between gradient and divergence operators for a refined grid MAC projection is presented, and a refined grid approximate projection is developed. An efficient multigrid method which exactly solves the projection is developed, and a method for casting certain approximate projections as MAC projections on refined grids is presented.


### `girardi-1997-unbalanced-haar`

Girardi, Maria, Sweldens, Wim (1997). *A new class of unbalanced haar wavelets that form an unconditional basis for Lp on general measure spaces.* The Journal of Fourier Analysis and Applications 3(4):457--474. DOI: [10.1007/bf02649107](https://doi.org/10.1007/bf02649107).

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> Given a complete separable σ-finite measure space (X, Σ, µ) and nested partitions of X, we construct unbalanced Haar-like wavelets on X that form an unconditional basis for Lp (X, Σ, µ) where 1 < p < ∞. Our construction and proofs build upon ideas of Burkholder and Mitrea. We show that if (X, Σ, µ) is not purely atomic, then the unconditional basis constant of our basis is (max(p, q) − 1). We derive a fast algorithm to compute the coefficients.


### `sweldens-1998-lifting`

Sweldens, Wim (1998). *The Lifting Scheme: A Construction of Second Generation Wavelets.* SIAM Journal on Mathematical Analysis 29(2):511--546. DOI: [10.1137/s0036141095289051](https://doi.org/10.1137/s0036141095289051).

**Availability:** PDF in relata store

**Abstract** *(source: OpenAlex)*

> We present the lifting scheme, a simple construction of second generation wavelets; these are wavelets that are not necessarily translates and dilates of one fixed function. Such wavelets can be adapted to intervals, domains, surfaces, weights, and irregular samples. We show how the lifting scheme leads to a faster, in-place calculation of the wavelet transform. Several examples are included.


### `balsara-2001-divfree`

Balsara, Dinshaw S. (2001). *Divergence-Free Adaptive Mesh Refinement for Magnetohydrodynamics.* Journal of Computational Physics 174(2):614--648. DOI: [10.1006/jcph.2001.6917](https://doi.org/10.1006/jcph.2001.6917).

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> Several physical systems such as non-relativistic and relativistic magnetohydrodynamics (MHD), radiation MHD, electromagnetics and incompressible hydrodynamics satisfy Stoke's law type equations for the divergence-free evolution of vector fields. In this paper we present a full-fledged scheme for the second order accurate, divergence-free evolution of vector fields on an adaptive mesh refinement (AMR) hierarchy. We focus here on adaptive mesh MHD. However, the scheme has applicability to the other systems of equations mentioned above. The scheme is based on making a significant advance in the divergence-free reconstruction of vector fields. In that sense, it complements the earlier work of Balsara and Spicer (1999) where we discussed the divergence-free time-update of vector fields which satisfy Stoke's law type evolution equations. Our advance in divergence-free reconstruction of vector fields is such that it reduces to the total variation diminishing (TVD) property for one-dimensional evolution and yet goes beyond it in multiple dimensions. For that reason, it is extremely suitable for the construction of higher order Godunov schemes for MHD. Both the two dimensional and three dimensional reconstruction strategies are developed. A slight extension of the divergence-free reconstruction procedure yields a divergence-free prolongation strategy for prolonging magnetic fields on AMR hierarchies. Divergencefree restriction is also discussed. Because our work is based on an integral formulation, divergence-free restriction and prolongation can be carried out on AMR meshes with any integral refinement ratio, though we specialize the expressions for the most popular situation where the refinement ratio is two. Furthermore, we pay attention to the fact that in order to efficiently evolve the MHD equations on AMR hierarchies, the refined meshes must evolve in time with time steps that are a fraction of their parent mesh's time step. An electric field correction strategy is presented for use on AMR meshes. The electric field correction strategy helps preserve the divergence-free evolution of the magnetic field even when the time steps are sub-cycled on refined meshes. The above-mentioned innovations have been implemented in Balsara's RIEMANN framework for parallel, selfadaptive computational astrophysics which supports both non-relativistic and relativistic MHD. Several rigorous, three dimensional AMR-MHD test problems with strong discontinuities have been run with the RIEMANN framework showing that the strategy works very well. In our AMR-MHD scheme the adaptive mesh hierarchy can change in response to discontinuities that move rapidly with respect to the mesh. Time-step subcycling permits efficient processing of the AMR hierarchy. Our AMR-MHD scheme parallelizes very well as shown by Balsara and Norton (Parallel Computing, 2001, vol. 27, pg. 37).


### `grabowski-2001-coupling`

Grabowski, Wojciech W. (2001). *Coupling Cloud Processes with the Large-Scale Dynamics Using the Cloud-Resolving Convection Parameterization (CRCP).* Journal of the Atmospheric Sciences 58(9):978--997. DOI: [10.1175/1520-0469(2001)058<0978:CCPWTL>2.0.CO;2](https://doi.org/10.1175/1520-0469(2001)058<0978:CCPWTL>2.0.CO;2).

**Availability:** PDF in relata store

**Abstract** *(source: OpenAlex)*

> A formal approach is presented to couple small-scale processes associated with atmospheric moist convection with the large-scale dynamics. The approach involves applying a two-dimensional cloud-resolving model in each column of a three-dimensional large-scale model. In the spirit of classical convection parameterization, which assumes scale separation between convection and the large-scale flow, the cloud-resolving models from neighboring columns interact only through the large-scale dynamics. This approach is referred to as Cloud-Resolving Convection Parameterization (CRCP). In short, CRCP involves many two-dimensional cloud-resolving models interacting in a manner consistent with the large-scale dynamics.


### `khairoutdinov-2001-cloud`

Khairoutdinov, Marat F., Randall, David A. (2001). *A Cloud Resolving Model as a Cloud Parameterization in the NCAR Community Climate System Model: Preliminary Results.* Geophysical Research Letters 28(18):3617--3620. DOI: [10.1029/2001GL013552](https://doi.org/10.1029/2001GL013552).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Preliminary results of a short climate simulation with a 2‐D cloud resolving model (CRM) installed into each grid column of an NCAR Community Climate System Model (CCSM) are presented. The CRM replaces the conventional convective and stratiform cloud parameterizations, and allows for explicit computation of the global cloud fraction distribution for radiation computations. The extreme computational cost of the combined CCSM/CRM model has thus far limited us to a two‐month long climate simulation (December‐January) using 2.8° × 2.8° resolution. The simulated geographical distributions of the total rainfall, precipitable water, cloud cover, and Earth radiation budget, for the month of January, look very reasonable.


### `e-2003-heterogeneous`

E, Weinan, Engquist, Bjorn (2003). *The Heterogeneous Multiscale Methods.* Communications in Mathematical Sciences 1(1):87--132. DOI: [10.4310/CMS.2003.v1.n1.a8](https://doi.org/10.4310/CMS.2003.v1.n1.a8).

**Availability:** PDF in relata store

**Abstract** *(source: OpenAlex)*

> The heterogenous multiscale method (HMM) is presented as a general\nmethodology for the efficient numerical computation of problems with\nmultiscales and multiphysics on multigrids. Both variational and\ndynamic problems are considered. The method relies on an efficent\ncoupling between the macroscopic and microscopic models. In cases\nwhen the macroscopic model is not explicity available or invalid,\nthe microscopic solver is used to supply the necessary data for the\nmicroscopic solver. Besides unifying several existing multiscale\nmethods such as the ab initio molecular dynamics [13],\nquasicontinuum methods [73,69,68] and projective methods for systems\nwith multiscales [34,35], HMM also provides a methodology for\ndesigning new methods for a large variety of multiscale problems. A\nframework is presented for the analysis of the stability and accuracy\nof HMM. Applications to problems such as homogenization, molecular\ndynamics, kinetic models and interfacial dynamics are discussed.


### `kevrekidis-2003-equation-free`

Kevrekidis, Ioannis G., Gear, C. William, Hyman, James M., Kevrekidis, Panagiotis G., Runborg, Olof, Theodoropoulos, Constantinos (2003). *Equation-Free, Coarse-Grained Multiscale Computation: Enabling Microscopic Simulators to Perform System-Level Analysis.* Communications in Mathematical Sciences 1(4):715--762. DOI: [10.4310/CMS.2003.v1.n4.a5](https://doi.org/10.4310/CMS.2003.v1.n4.a5).

**Availability:** PDF in relata store

**Abstract** *(source: OpenAlex)*

> We present and discuss a framework for computer-aided multiscale analysis, which enables models at a fine (microscopic/stochastic) level of description to perform modeling tasks at a coarse (macroscopic, systems) level. These macroscopic modeling tasks, yielding information over long time and large space scales, are accomplished through appropriately initialized calls to the microscopic simulator for only short times and small spatial domains. Traditional modeling approaches first involve the derivation of macroscopic evolution equations (balances closed through constitutive relations). An arsenal of analytical and numerical techniques for the efficient solution of such evolution equations (usually Partial Differential Equations, PDEs) is then brought to bear on the problem. Our equation-free (EF) approach, introduced in [1], when successful, can bypass the derivation of the macroscopic evolution equations when these equations conceptually exist but are not available in closed form. We discuss how the mathematics-assisted development of a computational superstructure may enable alternative descriptions of the problem physics (e.g. Lattice Boltzmann (LB), kinetic Monte Carlo (KMC) or Molecular Dynamics (MD) microscopic simulators, executed over relatively short time and space scales) to perform systems level tasks (integration over relatively large time and space scales,"coarse" bifurcation analysis, optimization, and control) directly. In effect, the procedure constitutes a system identification based, "closure-on-demand" computational toolkit, bridging microscopic/stochastic simulation with traditional continuum scientific computation and numerical analysis. We will briefly survey the application of these "numerical enabling technology" ideas through examples including the computation of coarsely self-similar solutions, and discuss various features, limitations and potential extensions of the approach.


### `jenny-2005-adaptive`

Jenny, P., Lee, Seong-Hyeok, Tchelepi, H. (2005). *Adaptive Multiscale Finite-Volume Method for Multiphase Flow and Transport in Porous Media.* Multiscale Modeling & Simulation 3(1):50--64. DOI: [10.1137/030600795](https://doi.org/10.1137/030600795).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We present a multiscale finite-volume (MSFV) method for multiphase flow and transport in heterogeneous porous media. The approach extends our recently developed MSFV method for single-phase flow. We use a sequential scheme that deals with flow (i.e., pressure and total velocity) and transport (i.e., saturation) separately and differently. For the flow problem, we employ two different sets of basis functions for the reconstruction of a conservative fine-scale total velocity field. Our basis functions are designed to have local support, and that allows for adaptive computation of the flow field. We use a criterion based on the time change of the total mobility field to decide when and where to recompute our basis functions. We show that at a given time step, only a small fraction of the basis functions needs to be recomputed. Numerical experiments of difficult two-dimensional and three-dimensional test cases demonstrate the accuracy, computational efficiency, and overall scalability of the method.


### `tchelepi-2005-adaptive`

Tchelepi, H., Jenny, P., Lee, Seong H., Wolfsteiner, Christian (2005). *An Adaptive Multiphase Multiscale Finite Volume Simulator for Heterogeneous Reservoirs.* SPE Reservoir Simulation Symposium. DOI: [10.2118/93395-ms](https://doi.org/10.2118/93395-ms).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We developed an adaptive reservoir simulator for accurate modeling of multiphase flow and transport in large scale heterogeneous reservoirs. The simulator is based on a multiscale finite volume (MSFV) method. We describe both IMPES and sequential implicit formulations. The algorithms are sensitive to the specific characteristics of flow (i.e., pressure and total velocity) and transport (i.e., saturation). To obtain the fine scale (i.e., fine grid) flow field, two sets of basis functions - dual and primal - are constructed. The dual basis functions, which are associated with the dual coarse grid, are used to calculate the coarse scale transmissibilities. The fine scale pressure field is computed from the coarse grid pressure via superposition of the dual basis functions. Having a locally conservative fine scale velocity field is essential for accurate solution of the saturation equations (i.e., transport). The primal basis functions, which are associated with the primal coarse grid, are constructed for that purpose. The dual basis functions serve as boundary conditions to the primal basis functions. To resolve the fine scale flow field in and around wells, a special well basis function is devised. As with the other basis functions, we ensure that the support for the well basis is local. Our MSFV simulator is designed for adaptive computation of both flow and transport in the course of a simulation run. Adaptive computation of the flow field is based on the time change of the total mobility field and triggers selective updates of basis functions. The key to achieving scalable (efficient for large problems) adaptive computation of flow and transport is the use of high fidelity basis functions with local support. We demonstrate the robustness and computational efficiency of the MSFV simulator using a variety of large heterogeneous reservoir models, including the SPE 10 comparative solution problem.


### `lambers-2006-specialized`

Lambers, J. (2006). *A SPECIALIZED UPSCALING METHOD FOR ADAPTIVE GRIDS: TIGHT INTEGRATION OF LOCAL-GLOBAL UPSCALING AND ADAPTIVITY LEADS TO ACCURATE SOLUTION OF FLOW IN HETEROGENEOUS FORMATIONS.* <https://www.semanticscholar.org/paper/005831318e3f40a261d680496b63759246a5a2d7>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We propose a methodology, called multi-level local-global upscaling, for generating accurate upscaled models of permeabilities or transmissibilities for flow simulation in highly heterogeneous subsurface formations. The method generates an initial adapted grid based on the given fine scale reservoir heterogeneity and potential flow paths. It then applies local-global upscaling [1], along with adaptivity, in an iterative manner. For highly heterogeneous (e.g., channelized) systems, this integration of grid adaptivity and upscaling is shown to consistently provide more accurate coarse scale models for global flow, relative to reference fine scale results, than do existing upscaling techniques applied to uniform grids of similar densities. The algorithm allows for rapid updating of the coarse scale model when the grid is adapted dynamically during transport to resolve important features, or as a result of changing boundary conditions. There is no need for downscaling and both refinement and coarsening can take place dynamically without destroying accurate global flow resolution on the adapted grid.


### `dubos-2010-conservative`

Dubos, T. (2010). *Conservative adaptivity and two-way self-nesting using discrete wavelets.* <https://www.semanticscholar.org/paper/1844a4daedf3dddaee80b8d553e82b9ed6219e33>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> In simulating atmosphere and oceans, multiscale modelling is desirable to track high-intensity weather patterns, to investigate the interactions between the various spatio-temporal scales of the climate system, and to perform assessments of climate change at scales small enough to derive impacts on society and ecosystems. The mainstream approach to multiscale modelling is to nest a fine, limited-area model into a coarse, global model. These models are then coupled, either one-way or two-way, in order to combine the global coverage of the global model and the fine details of the fine model.


### `e-2011-principles`

E, Weinan (2011). *Principles of Multiscale Modeling.* Cambridge University Press.

**Availability:** PDF in relata store

**Abstract:** ABSTRACT NOT RETRIEVED. A book (Cambridge University Press). No journal-style abstract exists; the publisher's description was not treated as an abstract.


### `dubos-2013-staggered-wavelet`

Dubos, T., Kevlahan, N. K‐R. (2013). *A conservative adaptive wavelet method for the shallow‐water equations on staggered grids.* Quarterly Journal of the Royal Meteorological Society 139(677):1997--2020. DOI: [10.1002/qj.2097](https://doi.org/10.1002/qj.2097).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> This article presents the first dynamically adaptive wavelet method for the shallow‐water equations (SWEs) on a staggered hexagonal C‐grid. Pressure is located at the centres of the primal grid (hexagons) and velocity is located at the edges of the dual grid (triangles). Distinct bi‐orthogonal second‐generation wavelet transforms are developed for the pressure and the velocity. These wavelet transforms are based on second‐order accurate interpolation and restriction operators. Together with compatible restriction operators for the mass flux and Bernoulli function, they ensure that mass is conserved and that there is no numerical generation of vorticity when solving the SWEs. Grid refinement relies on appropriate thresholding of the wavelet coefficients, allowing error control in both the quasi‐geostrophic and inertia–gravity wave regimes. The SWEs are discretized on the dynamically adapted multiscale grid using a mass and potential‐enstrophy‐conserving finite‐difference scheme. The conservation and error control properties of the method are verified by applying it to a propagating inertia–gravity wave packet and to rotating shallow‐water turbulence. Significant savings in the number of degrees of freedom are achieved even in the case of rotating shallow‐water turbulence. The numerical dissipation introduced by the grid adaptation is quantified. The method has been designed so it can be extended easily to the icosahedral subdivision of the sphere. This work provides important building blocks for the development of fully adaptive general circulation models.


### `aechtner-2015-wavelet-sphere`

Aechtner, M., Kevlahan, N. K.‐R., Dubos, T. (2014). *A conservative adaptive wavelet method for the shallow‐water equations on the sphere.* Quarterly Journal of the Royal Meteorological Society 141(690):1712--1726. DOI: [10.1002/qj.2473](https://doi.org/10.1002/qj.2473).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> We introduce an innovative wavelet‐based approach to adjust local grid resolution dynamically to maintain a uniform specified error tolerance. Extending the work of Dubos and Kevlahan, a wavelet multiscale approximation is used to make the Thuburn‐Ringler‐Skamarock‐Klemp (TRiSK) model dynamically adaptive for the rotating shallow‐water equations on the sphere. This article focuses on the challenges encountered when extending the adaptive wavelet method to the sphere and ensuring an efficient parallel implementation using message passing interface (MPI). The wavelet method is implemented in Fortran 95 with an emphasis on computational efficiency and scales well up to processors for load‐unbalanced scenarios and up to at least processors for load‐balanced scenarios. The method is verified using standard smooth test cases and a nonlinear test case proposed by Galewsky et al. The dynamical grid adaption provides compression ratios of up to 50 times in a challenging homogenous turbulence test case. The adaptive code is about three times slower per active grid point than the equivalent non‐adaptive TRiSK code and about four times slower per active grid point than an equivalent spectral code. This computationally efficient adaptive dynamical core could serve as the foundation on which to build a complete climate or weather model.


### `liu-2014-acceleration`

Liu, Ping, Samaey, G., Gear, C., Kevrekidis, I. (2014). *On the acceleration of spatially distributed agent-based computations.* Applied Numerical Mathematics 92:54--69. DOI: [10.1016/j.apnum.2014.12.007](https://doi.org/10.1016/j.apnum.2014.12.007).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> In recent years, individual-based/agent-based modeling has been applied to study a wide range of applications, ranging from engineering problems to phenomena in sociology, economics and biology. Simulating such agent-based models over extended spatiotemporal domains can be prohibitively expensive due to stochasticity and the presence of multiple scales. Nevertheless, many agent-based problems exhibit smooth behavior in space and time on a macroscopic scale, suggesting that a useful coarse-grained continuum model could be obtained. For such problems, the equation-free framework 16-18 can significantly reduce the computational cost. Patch dynamics is an essential component of this framework. This scheme is designed to perform numerical simulations of an unavailable macroscopic equation on macroscopic time and length scales; it uses appropriately initialized simulations of the fine-scale agent-based model in a number of small "patches", which cover only a fraction of the spatiotemporal domain. In this work, we construct a finite-volume-inspired conservative patch dynamics scheme and apply it to a financial market agent-based model based on the work of Omurtag and Sirovich 22. We first apply our patch dynamics scheme to a continuum approximation of the agent-based model, to study its performance and analyze its accuracy. We then apply the scheme to the agent-based model itself. Our computational experiments indicate that here, typically, the patch dynamics-based simulation needs to be performed in only 20% of the full agent simulation space, and in only 10% of the temporal domain.


### `myner-2014-construction`

Møyner, O. (2014). *Construction of Multiscale Preconditioners on Stratigraphic Grids.* Proceedings 2014:1--20. DOI: [10.3997/2214-4609.20141775](https://doi.org/10.3997/2214-4609.20141775).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> A large number of multiscale methods have been developed based on the same basic concept: Solve localized flow problems to estimate the local effects of fine-scale petrophysical properties. Use the resulting multiscale basis functions to pose a global flow problem a coarser grid. Reconstruct conservative fine-scale approximations from the coarse-scale solution. By extending the basic concept with iteration cycles and additional local stages, one can systematically drive the fine-scale residual towards machine precision. Posed algebraically, this can be seen as a set of restriction operators for computing a reduced global problem and a set of prolongation operators for constructing conservative fine-scale approximations. Such multiscale finite-volume methods have been extensively developed for Cartesian grids in the literature. The industry, however, uses very complex with unstructured connections and degenerate cell geometries to represent realistic structural frameworks and stratigraphic architectures. A successful multiscale method should therefore be able to handle unstructured polyhedral grids, both on the fine and coarse scale, and be as flexible as possible to enable automatic coarse partitionings that adapt to wells and geological features in a way that ensures optimal accuracy for a chosen level of coarsening. Herein, we will discuss a compare a set of prolongation operators that can be combined with finite-element or finite-volume restriction operators to form different multiscale finite-volume methods. We consider the MsFV prolongation operator (developed on a dual coarse grid with unitary at coarse block vertices), the more recent MsTPFA operator (developed on primal grid with unitary flux across coarse block faces), as well as a simplified constant prolongation operator. The methods will be compared on a variety of test cases ranging from simple synthetic grids to highly complex, real-world, field models. Our discussion will focus on flexibility wrt (coarse) grids and tendency of creating oscillatory approximations. In addition, we will look at various methods for improving the methods' convergence properties when used as preconditioners, as well as for generating novel prolongation operators. This is relevant for oil recovery because: - Multiscale methods may provide a way to significantly speed up reservoir simulation and make previously intractable problems possible to solve. - The extension of such methods to industry standard grids used for reservoir modelling enables the evaluation of the methods on real world models - The construction of basis functions for multiscale methods may have direct connections to the process of upscaling rock derived properties such as transmissibility


### `buhr-2015-interactive`

Buhr, A., Ohlberger, Mario (2015). *Interactive Simulations Using Localized Reduced Basis Methods.* IFAC-PapersOnLine 48(1):729--730. DOI: [10.1016/j.ifacol.2015.05.134](https://doi.org/10.1016/j.ifacol.2015.05.134).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract An interactive simulation tool should allow its user to change the geometry of the simulation and present an updated solution within a very short time span. To achieve this, the Reduced Basis Method can be used. For problems described by parametrized partial differential equations, it allows for very fast recomputation of the solution after parameter changes. In many cases, changes in the geometry can be accounted for by parametrization. However, this approach has two drawbacks: First, not all geometric variations can be described efficiently by parametrization. Second, the parametrization and thereby the type of changes possible has to be specified before the setup phase. The user is then restricted to these. To overcome these limitations, we propose to localize the basis generation in the Reduced Basis Method. Using basis functions having support only on a small subset of the domain, one can react to arbitrary local geometry modifications by recreating only the basis functions in an environment of the modification.


### `myner-2016-multiscale`

Møyner, O., Lie, Knut-Andreas (2016). *A multiscale restriction-smoothed basis method for high contrast porous media represented on unstructured grids.* Journal of Computational Physics 304:46--71. DOI: [10.1016/j.jcp.2015.10.010](https://doi.org/10.1016/j.jcp.2015.10.010).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> A wide variety of multiscale methods have been proposed in the literature to reduce runtime and provide better scaling for the solution of Poisson-type equations modeling flow in porous media. We present a new multiscale restricted-smoothed basis (MsRSB) method that is designed to be applicable to both rectilinear grids and unstructured grids. Like many other multiscale methods, MsRSB relies on a coarse partition of the underlying fine grid and a set of local prolongation operators (multiscale basis functions) that map unknowns associated with the fine grid cells to unknowns associated with blocks in the coarse partition. These mappings are constructed by restricted smoothing: Starting from a constant, a localized iterative scheme is applied directly to the fine-scale discretization to compute prolongation operators that are consistent with the local properties of the differential operators.The resulting method has three main advantages: First of all, both the coarse and the fine grid can have general polyhedral geometry and unstructured topology. This means that partitions and good prolongation operators can easily be constructed for complex models involving high media contrasts and unstructured cell connections introduced by faults, pinch-outs, erosion, local grid refinement, etc. In particular, the coarse partition can be adapted to geological or flow-field properties represented on cells or faces to improve accuracy. Secondly, the method is accurate and robust when compared to existing multiscale methods and does not need expensive recomputation of local basis functions to account for transient behavior: Dynamic mobility changes are incorporated by continuing to iterate a few extra steps on existing basis functions. This way, the cost of updating the prolongation operators becomes proportional to the amount of change in fluid mobility and one reduces the need for expensive, tolerance-based updates. Finally, since the MsRSB method is formulated on top of a cell-centered, conservative, finite-volume method, it is applicable to any flow model in which one can isolate a pressure equation. Herein, we only discuss single and two-phase incompressible models. Compressible flow, e.g., as modeled by the black-oil equations, is discussed in a separate paper.


### `weinzierl-2016-quasi-matrix-free`

Weinzierl, Marion, Weinzierl, T. (2016). *Quasi-matrix-free Hybrid Multigrid on Dynamically Adaptive Cartesian Grids.* ACM Transactions on Mathematical Software 44(3):1--44. DOI: [10.1145/3165280](https://doi.org/10.1145/3165280).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> We present a family of spacetree-based multigrid realizations using the tree's multiscale nature to derive coarse grids. They align with matrix-free geometric multigrid solvers as they never assemble the system matrices, which is cumbersome for dynamically adaptive grids and full multigrid. The most sophisticated realizations use BoxMG to construct operator-dependent prolongation and restriction in combination with Galerkin/Petrov-Galerkin coarse-grid operators. This yields robust solvers for nontrivial elliptic problems. We embed the algebraic, problem-dependent, and grid-dependent multigrid operators as stencils into the grid and evaluate all matrix-vector products in situ throughout the grid traversals. Such an approach is not literally matrix-free as the grid carries the matrix. We propose to switch to a hierarchical representation of all operators. Only differences of algebraic operators to their geometric counterparts are held. These hierarchical differences can be stored and exchanged with small memory footprint. Our realizations support arbitrary dynamically adaptive grids while they vertically integrate the multilevel operations through spacetree linearization. This yields good memory access characteristics, while standard colouring of mesh entities with domain decomposition allows us to use parallel many-core clusters. All realization ingredients are detailed such that they can be used by other codes.


### `lie-2017-feature-enriched`

Lie, Knut-Andreas, Møyner, O., Natvig, J. (2017). *A Feature-Enriched Multiscale Method for Simulating Complex Geomodels.* SPE Reservoir Simulation Conference. DOI: [10.2118/182701-ms](https://doi.org/10.2118/182701-ms).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Multiscale methods have been shown to offer an order-of-magnitude increase in the speed of reservoir simulators. This may enable users to model complex fluid flow and geology with greater speed and flexibility than is available with the current computational technologies. Contemporary multiscale methods typically use a restriction operator to construct a reduced system of flow equations and a prolongation operator to map pressure unknowns from the reduced flow equations back to the original simulation grid. When combined with a local smoother, this gives an iterative solver that can efficiently compute approximate pressures to within a prescribed accuracy and still provide mass-conservative fluxes. We present an adaptive and flexible framework for combining multiple sets of such multiscale approximations. Each multiscale approximation can target a certain scale; geological features like faults, fractures, facies, or other geobodies; or a particular computational challenge like propagating displacement and chemical fronts, wells being turned on or off, etc. Multiscale methods that fit the framework are characterized by three features. First, the prolongation and restriction operators are constructed using a non-overlapping partition of the fine grid. Second, the prolongation operator is composed of a set of basis functions, each of which has compact support within a support region that contains a coarse grid block. Finally, the basis functions form a partition of unity. Through a series of numerical examples that include idealized geology and flow physics as well as geological models of real assets, we demonstrate that the new framework increases the accuracy and efficiency of multiscale technology. In particular, we show how it is possible to combine multiscale approximations with different resolution as well as multiscale approximations targeting, among others, high-contrast fluvial sands; fractured carbonate reservoirs; challenging grids including faults, pinchouts and inactive cells; and complex wells. Introduction In reservoir simulation, a system of mass balance equations needs to be solved to determine the reservoir pressure and fluid composition. Each mass balance equation describes the evolution of one fluid species α in a porous medium Ω, in which multiple fluid species exist in M phases. When discretized in time and space, these equations form a system of nonlinear algebraic equations F α(p, S1, . . . , SM , xα,1, . . . , xα,M ) = qα. (1) Given a known pressure and fluid distribution at time t, Eq. 1 can be solved to determine the reservoir pressure p and distribution of fluid species (in terms of phase saturations S` and molar fractions xα,`) at time t + ∆t. In particular, by manipulating the equation system Eq. 1, it is possible to form a nonlinear system of equations for the reservoir pressure p at time t+ ∆t,


### `moraes-2017-multiscale`

Moraes, R., Rodrigues, J., Hajibeygi, H., Jansen, Jan Dirk (2017). *Multiscale Gradient Computation for Multiphase Flow in Porous Media.* SPE Reservoir Simulation Conference. DOI: [10.2118/182625-ms](https://doi.org/10.2118/182625-ms).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> A multiscale gradient computation method for multiphase flow in heterogeneous porous media is developed. The method constructs multiscale primal and dual coarse grids, imposed on the given fine-scale computational grid. Local multiscale basis functions are computed on (dual-) coarse blocks, constructing an accurate map (prolongation operator) between coarse- and fine-scale systems. While the expensive operations involved in computing the gradients are performed at the coarse scale, sensitivities with respect to uncertain parameters (e.g., grid block permeabilities) are expressed in the fine scale via the partial derivatives of the prolongation operator. Hence, the method allows for updating of the geological model, rather than the dynamic model only, avoiding upscaling and the inevitable loss of information. The formulation and implementation are based on automatic differentiation (AD), allowing for convenient extensions to complex physics. An IMPES coupling strategy for flow and transport is followed, in the forward simulation. The flow equation is computed using a multiscale finite volume (MSFV) formulation and the transport equation is computed at the fine scale, after reconstruction of mass conservative velocity field. To assess the performance of the method, a synthetic multiphase flow test case is considered. The multiscale gradients are compared against those obtained from a fine-scale reference strategy. Apart from its computational efficiency, the benefits of the method include flexibility to accommodate variables expressed at different scales, specially in multiscale data assimilation and reservoir management studies.


### `kevlahan-2019-wavetrisk`

Kevlahan, Nicholas K.-R., Dubos, Thomas (2019). *WAVETRISK-1.0: an adaptive wavelet hydrostatic dynamical core.* Geoscientific Model Development 12(11):4901--4921. DOI: [10.5194/gmd-12-4901-2019](https://doi.org/10.5194/gmd-12-4901-2019).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> This paper presents the new adaptive dynamical core wavetrisk. The fundamental features of the wavelet-based adaptivity were developed for the shallow water equation on the β plane and extended to the icosahedral grid on the sphere in previous work by the authors. The three-dimensional dynamical core solves the compressible hydrostatic multilayer rotating shallow water equations on a multiscale dynamically adapted grid. The equations are discretized using a Lagrangian vertical coordinate version of the dynamico model. The horizontal computational grid is adapted at each time step to ensure a user-specified relative error in either the tendencies or the solution. The Lagrangian vertical grid is remapped using an arbitrary Lagrangian–Eulerian (ALE) algorithm onto the initial hybrid σ-pressure-based coordinates as necessary. The resulting grid is adapted horizontally but uniform over all vertical layers. Thus, the three-dimensional grid is a set of columns of varying sizes. The code is parallelized by domain decomposition using mpi, and the variables are stored in a hybrid data structure of dyadic quad trees and patches. A low-storage explicit fourth-order Runge–Kutta scheme is used for time integration. Validation results are presented for three standard dynamical core test cases: mountain-induced Rossby wave train, baroclinic instability of a jet stream and the Held and Suarez simplified general circulation model. The results confirm good strong parallel scaling and demonstrate that wavetrisk can achieve grid compression ratios of several hundred times compared with an equivalent static grid model.


### `klemetsdal-2019-accelerating`

Klemetsdal, Ø., Møyner, O., Lie, Knut-Andreas (2019). *Accelerating multiscale simulation of complex geomodels by use of dynamically adapted basis functions.* Computational Geosciences 24(2):459--476. DOI: [10.1007/s10596-019-9827-z](https://doi.org/10.1007/s10596-019-9827-z).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> A number of different multiscale methods have been developed as a robust alternative to upscaling and as a means for accelerated reservoir simulation of high-resolution geomodels. In their basic setup, multiscale methods use a restriction operator to construct a reduced system of flow equations on a coarser grid, and a prolongation operator to map pressure unknowns from the coarse grid back to the original simulation grid. The prolongation operator consists of basis functions computed numerically by solving localized flow problems. One can use the resulting multiscale solver both as a CPR preconditioner in fully implicit simulators or as an efficient approximate iterative linear solver in a sequential setting. The latter approach has been successfully implemented in a commercial simulator. Recently, we have shown that you can obtain significantly faster convergence if you instead of using a single pair of prolongation-restriction operators apply a sequence of such operators, where some of the operators adapt to faults, fractures, facies, or other geobodies. Herein, we present how you can accelerate the convergence even further, if you also include additional basis functions that capture local changes in the pressure.


### `luan-2019-class`

Luan, Vu Thai, Chinomona, Rujeko, Reynolds, Daniel R. (2019). *A New Class of High-Order Methods for Multirate Differential Equations.* SIAM Journal on Scientific Computing 42(2):A1245-A1268. DOI: [10.1137/19m125621x](https://doi.org/10.1137/19m125621x).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> This work focuses on the development of a new class of high-order accurate methods for multirate time integration of systems of ordinary differential equations. The proposed methods are based on a specific subset of explicit one-step exponential integrators. More precisely, starting from an explicit exponential Runge--Kutta method of the appropriate form, we derive a multirate algorithm to approximate the action of the matrix exponential through the definition of modified "fast" initial-value problems. These fast problems may be solved using any viable solver, enabling multirate simulations through use of a subcycled method. Due to this structure, we name these Multirate Exponential Runge--Kutta (MERK) methods. In addition to showing how MERK methods may be derived, we provide rigorous convergence analysis, showing that for an overall method of order $p$, the fast problems corresponding to internal stages may be solved using a method of order $p-1$, while the final fast problem corresponding to the time-evolved solution must use a method of order $p$. Numerical simulations are then provided to demonstrate the convergence and efficiency of MERK methods with orders three through five on a series of multirate test problems.


### `vandenbulcke-2019-upscaling`

Vandenbulcke, Luc, Barth, Alexander (2019). *Upscaling of a Local Model into a Larger-Scale Model.* Ocean Science 15(2):291--305. DOI: [10.5194/os-15-291-2019](https://doi.org/10.5194/os-15-291-2019).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> Traditionally, in order for lower-resolution, global- or basin-scale (regional) models to benefit from some of the improvements available in higher-resolution subregional or coastal models, two-way nesting has to be used. This implies that the parent and child models have to be run together and there is an online exchange of information between both models. This approach is often impossible in operational systems where different model codes are run by different institutions, often in different countries. Therefore, in practice, these systems use one-way nesting with data transfer only from the parent model to the child models. In this article, it is examined whether it is possible to replace the missing feedback (coming from the child model) by data assimilation, avoiding the need to run the models simultaneously. Selected variables from the high-resolution simulation will be used as pseudo-observations and assimilated into the low-resolution models. This method will be called "upscaling". A realistic test case is set up with a model covering the Mediterranean Sea, and a nested model covering its north-western basin. Under the hypothesis that the nested model has better prediction skills than the parent model, the upscaling method is implemented. Two simulations of the parent model are then compared: the case of one-way nesting (or a stand-alone model) and a simulation using the upscaling technique on the temperature and salinity variables. It is shown that the representation of some processes, such as the Rhône River plume, is strongly improved in the upscaled model compared to the stand-alone model.


### `bunder-2020-equation-free`

Bunder, J., Kevrekidis, I., Roberts, A. J. (2020). *Equation-free patch scheme for efficient computational homogenisation via self-adjoint coupling.* Numerische Mathematik 149(2):229--272. DOI: [10.1007/s00211-021-01232-5](https://doi.org/10.1007/s00211-021-01232-5).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Equation-free macroscale modelling is a systematic and rigorous computational methodology for efficiently predicting the dynamics of a microscale complex system at a desired macroscale system level. In this scheme, a given microscale model is computed in small patches spread across the space-time domain, with patch coupling conditions bridging the unsimulated space. For accurate predictions, care must be taken in designing the patch coupling conditions. Here we construct novel coupling conditions which preserve self-adjoint symmetry, thus guaranteeing that the macroscale model maintains some important conservation laws of the original microscale model. Consistency of the patch scheme's macroscale dynamics with the original microscale model is proved for systems in 1D and 2D space, and these proofs immediately extend to higher dimensions. Expanding from a system with a single configuration to an ensemble of configurations establishes that the proven consistency also holds for cases where the microscale periodicity does not integrally fill the patches. This new self-adjoint patch scheme provides an efficient, flexible, and accurate computational homogenisation, as demonstrated here with canonical examples in 1D and 2D space based on heterogenous diffusion, and is applicable to a wide range of multiscale scenarios of interest to scientists and engineers.


### `chinomona-2020-implicit-explicit`

Chinomona, Rujeko, Reynolds, Daniel R. (2020). *Implicit-Explicit Multirate Infinitesimal GARK Methods.* SIAM Journal on Scientific Computing 43(5):A3082-A3113. DOI: [10.1137/20m1354349](https://doi.org/10.1137/20m1354349).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> This work focuses on the development of a new class of high-order accurate methods for multirate time integration of systems of ordinary differential equations. Unlike other recent work in this area, the proposed methods support mixed implicit-explicit (IMEX) treatment of the slow time scale. In addition to allowing this slow time scale flexibility, the proposed methods utilize a so-called `infinitesimal' formulation for the fast time scale through definition of a sequence of modified `fast' initial-value problems, that may be solved using any viable algorithm. We name the proposed class as implicit-explicit multirate infinitesimal generalized-structure additive Runge--Kutta (IMEX-MRI-GARK) methods. In addition to defining these methods, we prove that they may be viewed as specific instances of GARK methods, and derive a set of order conditions on the IMEX-MRI-GARK coefficients to guarantee both third and fourth order accuracy for the overall multirate method. Additionally, we provide three specific IMEX-MRI-GARK methods, two of order three and one of order four. We conclude with numerical simulations on two multirate test problems, demonstrating the methods' predicted convergence rates and comparing their efficiency against both legacy IMEX multirate schemes and recent third and fourth order implicit MRI-GARK methods.


### `luan-2021-multirate`

Luan, Vu Thai, Chinomona, Rujeko, Reynolds, Daniel R. (2021). *Multirate Exponential Rosenbrock Methods.* SIAM Journal on Scientific Computing 44(5):3265-. DOI: [10.1137/21m1439481](https://doi.org/10.1137/21m1439481).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> In this paper we propose a novel class of methods for high order accurate integration of multirate systems of ordinary differential equation initial-value problems. The proposed methods construct multirate schemes by approximating the action of matrix $\varphi$-functions within explicit exponential Rosenbrock (ExpRB) methods, thereby called Multirate Exponential Rosenbrock (MERB) methods. They consist of the solution to a sequence of modified"fast"initial-value problems, that may themselves be approximated through subcycling any desired IVP solver. In addition to proving how to construct MERB methods from certain classes of ExpRB methods, we provide rigorous convergence analysis of these methods and derive efficient MERB schemes of orders two through six (the highest order ever constructed infinitesimal multirate methods). We then present numerical simulations to confirm these theoretical convergence rates, and to compare the efficiency of MERB methods against other recently-introduced high order multirate methods.


### `maclean-2021-adaptively`

Maclean, J., Bunder, J., Kevrekidis, I., Roberts, A. (2021). *Adaptively detect and accurately resolve macro-scale shocks in an efficient Equation-Free multiscale simulation.* SIAM Journal on Scientific Computing 44(4):2557-. DOI: [10.1137/21m1437172](https://doi.org/10.1137/21m1437172).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> The Equation-Free approach to efficient multiscale numerical computation marries trusted micro-scale simulations to a framework for numerical macro-scale reduction -- the patch dynamics scheme. A recent novel patch scheme empowered the Equation-Free approach to simulate systems containing shocks on the macro-scale. However, the scheme did not predict the formation of shocks accurately, and it could not simulate moving shocks. This article resolves both issues, as a first step in one spatial dimension, by embedding the Equation-Free, shock-resolving patch scheme within a classic framework for adaptive moving meshes. Our canonical micro-scale problems exhibit heterogeneous nonlinear advection and heterogeneous diffusion. We demonstrate many remarkable benefits from the moving patch scheme, including efficient and accurate macro-scale prediction despite the unknown macro-scale closure. Equation-free methods are here extended to simulate moving, forming and merging shocks without a priori knowledge of the existence or closure of the shocks. Whereas adaptive moving mesh equations are typically stiff, typically requiring small time-steps on the macro-scale, the moving macro-scale mesh of patches is typically not stiff given the context of the micro-scale time-steps required for the sub-patch dynamics.


### `maclean-2021-equation`

Maclean, J., Bunder, J., Kevrekidis, I., Roberts, A. (2021). *An Equation Free Algorithm Accurately Simulates Macroscale Shocks Arising From Heterogeneous Microscale Systems.* IEEE Journal on Multiscale and Multiphysics Computational Techniques 6:8--15. DOI: [10.1109/jmmct.2021.3054012](https://doi.org/10.1109/jmmct.2021.3054012).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Scientists and engineers often create accurate, trustworthy, computational simulation schemes—but all too often these are too computationally expensive to execute over the time or spatial domain of interest. The equation-free approach is to marry such trusted simulations to a framework for numerical macroscale reduction—the patch dynamics scheme. This article extends the patch scheme to scenarios in which the trusted simulation resolves abrupt state changes on the microscale that appear as shocks on the macroscale. Accurate simulation for problems in these scenarios requires capturing the shock within a novel patch, and also modifying the patch coupling rules in the vicinity in order to maintain accuracy. With these two extensions to the patch scheme, straightforward arguments derive consistency conditions that match the usual order of accuracy for patch schemes. The new scheme is successfully tested to simulate a heterogeneous microscale partial differential equation. This technique will empower scientists and engineers to accurately and efficiently simulate, over large spatial domains, multiscale multiphysics systems that have rapid transition layers on the microscale.


### `divahar-2022-novel`

Divahar, J., Roberts, A., Mattner, T. W., Bunder, J., Kevrekidis, I. (2022). *Two novel families of multiscale staggered patch schemes efficiently simulate large-scale, weakly damped, linear waves.* Computer Methods in Applied Mechanics and Engineering abs/2210.15823:116133. DOI: [10.1016/j.cma.2023.116133](https://doi.org/10.1016/j.cma.2023.116133).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Many multiscale wave systems exhibit macroscale emergent behaviour, for example, the fluid dynamics of floods and tsunamis. Resolving a large range of spatial scales typically requires a prohibitively high computational cost. The small dissipation in wave systems poses a significant challenge to further developing multiscale modelling methods in multiple dimensions. This article develops and evaluates two families of equation-free multiscale methods on novel 2D staggered patch schemes, and demonstrates the power and utility of these multiscale schemes for weakly damped linear waves. A detailed study of sensitivity to numerical roundoff errors establishes the robustness of developed staggered patch schemes. Comprehensive eigenvalue analysis over a wide range of parameters establishes the stability, accuracy, and consistency of the multiscale schemes. Analysis of the computational complexity shows that the measured compute times of the multiscale schemes may be 10^5 times smaller than the compute time for the corresponding full-domain computation. This work provides the essential foundation for efficient large-scale simulation of challenging nonlinear multiscale waves.


### `he-2022-fast`

He, Zhong-Quan, Pérez, Jesús, Otaduy, M. (2022). *Fast Numerical Coarsening with Local Factorizations.* Computer Graphics Forum 41(8):9-17. DOI: [10.1111/cgf.14619](https://doi.org/10.1111/cgf.14619).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Numerical coarsening methods offer an attractive methodology for fast simulation of objects with high‐resolution heterogeneity. However, they rely heavily on preprocessing, and are not suitable when objects undergo dynamic material or topology updates. We present methods that largely accelerate the two main processes of numerical coarsening, namely training data generation and the optimization of coarsening shape functions, and as a result we manage to leverage runtime numerical coarsening under local material updates. To accelerate the generation of training data, we propose a domain‐decomposition solver based on substructuring that leverages local factorizations. To accelerate the computation of coarsening shape functions, we propose a decoupled optimization of smoothness and data fitting. We evaluate quantitatively the accuracy and performance of our proposed methods, and we show that they achieve accuracy comparable to the baseline, albeit with speed‐ups of orders of magnitude. We also demonstrate our methods on example simulations with local material and topology updates.


### `kevlahan-2022-wavetrisk-ocean`

Kevlahan, Nicholas K.-R., Lemarié, Florian (2022). *wavetrisk-2.1: an adaptive dynamical core for ocean modelling.* Geoscientific Model Development 15(17):6521--6539. DOI: [10.5194/gmd-15-6521-2022](https://doi.org/10.5194/gmd-15-6521-2022).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> This paper introduces wavetrisk-2.1 (i.e. wavetrisk-ocean), an incompressible version of the atmosphere model wavetrisk-1.x with free surface. This new model is built on the same wavelet-based dynamically adaptive core as wavetrisk, which itself uses dynamico's mimetic vector-invariant multilayer rotating shallow water formulation. Both codes use a Lagrangian vertical coordinate with conservative remapping. The ocean variant solves the incompressible multilayer shallow water equations with inhomogeneous density layers. Time integration uses barotropic–baroclinic mode splitting via an semi-implicit free surface formulation, which is about 34–44 times faster than an unsplit explicit time-stepping. The barotropic and baroclinic estimates of the free surface are reconciled at each time step using layer dilation. No slip boundary conditions at coastlines are approximated using volume penalization. The vertical eddy viscosity and diffusivity coefficients are computed from a closure model based on turbulent kinetic energy (TKE). Results are presented for a standard set of ocean model test cases adapted to the sphere (seamount, upwelling and baroclinic turbulence). An innovative feature of wavetrisk-ocean is that it could be coupled easily to the wavetrisk atmosphere model, thus providing a first building block toward an integrated Earth system model using a consistent modelling framework with dynamic mesh adaptivity and mimetic properties.


### `loffeld-2022-performance`

Loffeld, J., Nonaka, A., Reynolds, Daniel R., Gardner, D., Woodward, C. (2022). *Performance of explicit and IMEX MRI multirate methods on complex reactive flow problems within modern parallel adaptive structured grid frameworks.* The International Journal of High Performance Computing Applications 38(4):263--281. DOI: [10.1177/10943420241227914](https://doi.org/10.1177/10943420241227914).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Large-scale multiphysics simulations are computationally challenging due to the coupling of multiple processes with widely disparate time scales. The advent of exascale computing systems exacerbates these challenges since these systems enable ever-increasing size and complexity. In recent years, there has been renewed interest in developing multirate methods as a means to handle the large range of time scales, as these methods may afford greater accuracy and efficiency than more traditional approaches of using implicit-explicit (IMEX) and low-order operator splitting schemes. However, to date there have been few performance studies that compare different classes of multirate integrators on complex application problems. In this work, we study the performance of several newly developed multirate infinitesimal (MRI) methods, implemented in the SUNDIALS solver package, on two reacting flow model problems built on structured mesh frameworks. The first model revisits prior work on a compressible reacting flow problem with complex chemistry that is implemented using BoxLib but where we now include comparisons between a new explicit MRI scheme with the multirate spectral deferred correction (SDC) methods in the original paper. The second problem uses the same complex chemistry as the first problem, combined with a simplified flow model, but runs at a large spatial scale where explicit methods become infeasible due to stability constraints. Two recently developed IMEX MRI multirate methods are tested. These methods rely on advanced features of the AMReX framework on which the model is built, such as multilevel grids and multilevel preconditioners. The results from these two problems show that MRI multirate methods can offer significant performance benefits on complex multiphysics application problems and that these methods may be combined with advanced spatial discretization to compound the advantages of both.


### `ltjens-2022-multiscale`

Lütjens, Björn, Crawford, Catherine H., Watson, C., Hill, C., Newman, Dava (2022). *Multiscale Neural Operator: Learning Fast and Grid-independent PDE Solvers.* abs/2207.11417. DOI: [10.48550/arxiv.2207.11417](https://doi.org/10.48550/arxiv.2207.11417).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Numerical simulations in climate, chemistry, or astrophysics are computationally too expensive for uncertainty quantification or parameter-exploration at high-resolution. Reduced-order or surrogate models are multiple orders of magnitude faster, but traditional surrogates are inflexible or inaccurate and pure machine learning (ML)-based surrogates too data-hungry. We propose a hybrid, flexible surrogate model that exploits known physics for simulating large-scale dynamics and limits learning to the hard-to-model term, which is called parametrization or closure and captures the effect of fine- onto large-scale dynamics. Leveraging neural operators, we are the first to learn grid-independent, non-local, and flexible parametrizations. Our \textitmultiscale neural operator is motivated by a rich literature in multiscale modeling, has quasilinear runtime complexity, is more accurate or flexible than state-of-the-art parametrizations and demonstrated on the chaotic equation multiscale Lorenz96.


### `reynolds-2022-arkode`

Reynolds, Daniel R., Gardner, D., Woodward, C., Chinomona, Rujeko (2022). *ARKODE: A Flexible IVP Solver Infrastructure for One-step Methods.* ACM Transactions on Mathematical Software 49(2):1--26. DOI: [10.1145/3594632](https://doi.org/10.1145/3594632).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We describe the ARKODE library of one-step time integration methods for ordinary differential equation (ODE) initial-value problems (IVPs). In addition to providing standard explicit and diagonally implicit Runge–Kutta methods, ARKODE supports one-step methods designed to treat additive splittings of the IVP, including implicit-explicit (ImEx) additive Runge–Kutta methods and multirate infinitesimal (MRI) methods. We present the role of ARKODE within the SUNDIALS suite of time integration and nonlinear solver libraries, the core ARKODE infrastructure for utilities common to large classes of one-step methods, as well as its use of "time stepper" modules enabling easy incorporation of novel algorithms into the library. Numerical results show example problems of increasing complexity, highlighting the algorithmic flexibility afforded through this infrastructure, and include a larger multiphysics application leveraging multiple algorithmic features from ARKODE and SUNDIALS.


### `fish-2023-implicit-explicit`

Fish, Alex C., Reynolds, Daniel R., Roberts, S. (2023). *Implicit-Explicit Multirate Infinitesimal Stage-Restart Methods.* Journal of Computational and Applied Mathematics abs/2301.00865:115534. DOI: [10.1016/j.cam.2023.115534](https://doi.org/10.1016/j.cam.2023.115534).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Implicit-Explicit (IMEX) methods are flexible numerical time integration methods which solve an initial-value problem (IVP) that is partitioned into stiff and nonstiff processes with the goal of lower computational costs than a purely implicit or explicit approach. A complementary form of flexible IVP solvers are multirate infinitesimal methods for problems partitioned into fast- and slow-changing dynamics, that solve a multirate IVP by evolving a sequence of ``fast''IVPs using any suitably accurate algorithm. This article introduces a new class of high-order implicit-explicit multirate methods that are designed for multirate IVPs in which the slow-changing dynamics are further partitioned in an IMEX fashion. This new class, which we call implicit-explicit multirate stage-restart (IMEX-MRI-SR), both improves upon the previous implicit-explicit multirate generalized-structure additive Runge Kutta (IMEX-MRI-GARK) methods, and extends multirate exponential Runge Kutta (MERK) methods into the IMEX context. We leverage GARK theory to derive conditions guaranteeing orders of accuracy up to four. We provide second-, third-, and fourth-order accurate example methods and perform numerical simulations demonstrating convergence rates and computational performance in both fixed-step and adaptive-step settings.


### `guilherme-2023-application`

Guilherme, J., Steinstraesser, Caldas, Guinot, V., Rousseau, A. (2023). *Application of a modified parareal method for speeding up the numerical resolution of the 2D shallow water equations.* <https://www.semanticscholar.org/paper/3056925d3f3d804f13d0c587d638c0d91a741662>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> In this work, we implement some variations of the parareal method for speeding up the numerical resolution of the two-dimensional nonlinear shallow water equations (SWE). This method aims to reduce the computational time required for a fine and expensive model, by using alongside a less accurate, but much cheaper, coarser one, which allows to parallelize in time the fine simulation. We consider here a variant of the method using reduced-order models and suitable for treating nonlinear hyperbolic problems, being able to reduce stability and convergence issues of the parareal algorithm in its original formulation. We also propose a modification of the ROM-based parareal method consisting in the enrichment of the input data for the model reduction with extra information not requiring any additional computational cost to be obtained. Numerical simulations of the 2D nonlinear SWE with increasing complexity are presented for comparing the configurations of the model reduction techniques and the performance of the parareal variants. Our proposed method presents a more stable behavior and a faster convergence towards the fine, referential solution, providing good approximations with a reduced computational cost. Therefore, it is a promising tool for accelerating the numerical simulation of problems in hydrodynamics.


### `roberts-2023-accurate`

Roberts, A., Tran-Duc, T., Bunder, J., Kevrekidis, Y. (2023). *Accurate and efficient multiscale simulation of a heterogeneous elastic beam via computation on small sparse patches.* abs/2301.13145. DOI: [10.48550/arxiv.2301.13145](https://doi.org/10.48550/arxiv.2301.13145).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Modern `smart' materials have complex microscale structure, often with unknown macroscale closure. The Equation-Free Patch Scheme empowers us to non-intrusively, efficiently, and accurately simulate over large scales through computations on only small well-separated patches of the microscale system. Here the microscale system is a solid beam of random heterogeneous elasticity. The continuing challenge is to compute the given physics on just the microscale patches, and couple the patches across un-simulated macroscale space, in order to establish efficiency, accuracy, consistency, and stability on the macroscale. Dynamical systems theory supports the scheme. This research program is to develop a systematic non-intrusive approach, both computationally and analytically proven, to model and compute accurately macroscale system levels of general complex physical and engineering systems. References R. A. Biezemans, C. Le Bris, F. Legoll, and A. Lozinski. Non-intrusive implementation of a wide variety of Multiscale Finite Element Methods. Comptes Rendus. Mécanique 351 (2023), pp. 1–46. doi: 10.5802/crmeca.178 M. P. Brenner and P. Koumoutsakos. Editorial: Machine learning and Physical Review Fluids: An editorial perspective. Phys. Rev. Fluids 6.7 (2021), p. 070001. doi: 10.1103/PhysRevFluids.6.070001 J. E. Bunder, I. G. Kevrekidis, and A. J. Roberts. Equation-free patch scheme for efficient computational homogenisation via self-adjoint coupling. Numer. Math. 149.2 (2021), pp. 229–272. doi: 10.1007/s00211-021-01232-5 J. E. Bunder, A. J. Roberts, and I. G. Kevrekidis. Good coupling for the multiscale patch scheme on systems with microscale heterogeneity. J. Comput. Phys. 337 (2017), pp. 154–174. doi: 10.1016/j.jcp.2017.02.004 References C175 M. Cao and A. J. Roberts. Multiscale modelling couples patches of nonlinear wave-like simulations. IMA J. Appl. Math. 81.2 (2016), pp. 228–254. doi: 10.1093/imamat/hxv034 J. Divahar, A. J. Roberts, T. W. Mattner, J. E. Bunder, and I. G. Kevrekidis. Two novel families of multiscale staggered patch schemes efficiently simulate large-scale, weakly damped, linear waves. Comput. Meth. Appl. Mech. Eng. 413 (2023), p. 116133. doi: 10.1016/j.cma.2023.116133. (Cit. on pp. C163, C165, C172). S. Lucarini, M. V. Upadhyay, and J. Segurado. FFT based approaches in micromechanics: fundamentals, methods and applications. Model. Sim. Mat. Sci. Eng. 30.2 (2021), p. 023002. doi: 10.1088/1361-651X/ac34e1 J. Maclean, J. E. Bunder, and A. J. Roberts. A toolbox of Equation-Free functions in Matlab/Octave for efficient system level simulation. Numer. Alg. 87 (2021), pp. 1729–1748. doi: 10.1007/s11075-020-01027-z J. Maclean, J. E. Bunder, I. G. Kevrekidis, and A. J. Roberts. An equation free algorithm accurately simulates macroscale shocks arising from heterogeneous microscale systems. IEEE J. Multiscale Multiphys. Comput. Tech. 6 (2021), pp. 8–15. doi: 10.1109/JMMCT.2021.3054012 A. J. Majda and I. Grooms. New perspectives on superparameterization for geophysical turbulence. J. Comput. Phys. Frontiers in Computational Physics 271 (2014), pp. 60–77. doi: 10.1016/j.jcp.2013.09.014 K. Matouš, M. G. D. Geers, V. G. Kouznetsova, and A. Gillman. A review of predictive nonlinear theories for multiscale modeling of heterogeneous materials. J. Comput. Phys. 330 (2017), pp. 192–220. doi: 10.1016/j.jcp.2016.10.070 K. Raju, T.-E. Tay, and V. B. C. Tan. A review of the FE2 method for composites. Multiscale Multidisc. Model. Exp. Design 4 (2021), pp. 1–24. doi: 10.1007/s41939-020-00087-x A. J. Roberts. Macroscale, slowly varying, models emerge from the microscale dynamics in long thin domains. IMA J. Appl. Math. 80.5 (2015), pp. 1492–1518. doi: 10.1093/imamat/hxv004 A. J. Roberts and I. G. Kevrekidis. General tooth boundary conditions for equation free modelling. SIAM J. Sci. Comput. 29.4 (2007), pp. 1495–1510. doi: 10.1137/060654554 A. J. Roberts, T. MacKenzie, and J. Bunder. A dynamical systems approach to simulating macroscale spatial dynamics in multiple dimensions. J. Eng. Math. 86.1 (2014), pp. 175–207. doi: 10.1007/s10665-013-9653-6 A. J. Roberts, J. Maclean, and J. E. Bunder. Equation-Free function toolbox for Matlab/Octave. Tech. rep. https://github.com/uoa1184615/EquationFreeGit, 2019–2024 G. Samaey, A. J. Roberts, and I. G. Kevrekidis. Equation-free computation: An overview of patch dynamics. Multiscale methods: bridging the scales in science and engineering. Ed. by J. Fish. Oxford University Press, 2010. Chap. 8, pp. 216–246. doi: 10.1093/acprof:oso/9780199233854.003.0008 J. Somnic and B. W. Jo. Status and challenges in homogenization methods for lattice materials. Materials 15.2 (2022), p. 605. doi: 10.3390/ma15020605 H. Whitney. Differentiable manifolds. Annal. Math. 37.3 (1936), pp. 645–680. doi: 10.2307/1968482


### `abdi-2024-comparison`

Abdi, Daniel S., Almgren, Ann S., Giraldo, Francis X., Jankov, Isidora (2024). *Comparison of adaptive mesh refinement techniques for numerical weather prediction.* abs/2404.16648. DOI: [10.48550/arxiv.2404.16648](https://doi.org/10.48550/arxiv.2404.16648).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> This paper examines the application of adaptive mesh refinement (AMR) in the field of numerical weather prediction (NWP). We implement and assess two distinct AMR approaches and evaluate their performance through standard NWP benchmarks. In both cases, we solve the fully compressible Euler equations, fundamental to many non-hydrostatic weather models. The first approach utilizes oct-tree cell-based mesh refinement coupled with a high-order discontinuous Galerkin method for spatial discretization. In the second approach, we employ level-based AMR with the finite difference method. Our study provides insights into the accuracy and benefits of employing these AMR methodologies for the multi-scale problem of NWP. Additionally, we explore essential properties including their impact on mass and energy conservation. Moreover, we present and evaluate an AMR solution transfer strategy for the tree-based AMR approach that is simple to implement, memory-efficient, and ensures conservation for both flow in the box and sphere. Furthermore, we discuss scalability, performance portability, and the practical utility of the AMR methodology within an NWP framework -- crucial considerations in selecting an AMR approach. The current de facto standard for mesh refinement in NWP employs a relatively simplistic approach of static nested grids, either within a general circulation model or a separately operated regional model with loose one-way synchronization. It is our hope that this study will stimulate further interest in the adoption of AMR frameworks like AMReX in NWP. These frameworks offer a triple advantage: a robust dynamic AMR for tracking localized and consequential features such as tropical cyclones, extreme scalability, and performance portability.


### `guenter-2024-pythos`

Guenter, Victoria, Wei, Siqi, Spiteri, Raymond J. (2024). *pythOS: A Python library for solving IVPs by operator splitting.* abs/2407.05475. DOI: [10.48550/arxiv.2407.05475](https://doi.org/10.48550/arxiv.2407.05475).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Operator-splitting methods are widespread in the numerical solution of differential equations, especially the initial-value problems in ordinary differential equations that arise from a method-of-lines discretization of partial differential equations. Such problems can often be solved more effectively by treating the various terms individually with specialized methods rather than simultaneously in a monolithic fashion. This paper describes \pythOS, a Python software library for the systematic solution of differential equations by operator-splitting methods. The functionality of \pythOS\ focuses on fractional-step methods, including those with real and complex coefficients, but it also implements additive Runge--Kutta methods, generalized additive Runge--Kutta methods, and multi-rate, and multi-rate infinitesimal methods. Experimentation with the solution of practical problems is facilitated through an interface to the \Firedrake\ library for the finite element spatial discretization of partial differential equations and further enhanced by the convenient implementation of exponential time-integration methods and fully implicit Runge--Kutta methods available from the \Irksome\ software library. The functionality of \pythOS\ as well as some less generally appreciated aspects of operator-splitting methods are demonstrated by means of examples.


### `karmakar-2024-generalised`

Karmakar, T. K., Dalal, D. C. (2024). *A Generalised Curvilinear Coordinate system-based Patch Dynamics Scheme in Equation-free Multiscale Modelling.* <https://www.semanticscholar.org/paper/75524c9fc18189959d95f0edaa2e47e36bfde237>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> The patch dynamics scheme in equation-free multiscale modelling has the potential to efficiently predict the macroscopic behaviours by simulating the microscale problem in a fraction of the space-time domain. The patch dynamics schemes developed so far are mainly on rectangular domains with uniform grids and uniform rectangular patches. In real-life problems, the geometry of the domain is not regular or simple, where rectangular and uniform grids or patches may not be useful. To address this kind of complexity, for the first time, a generalised orthogonal curvilinear coordinate system is employed in the patch dynamics scheme, applicable to both rectangular domains with non-uniform grids and non-rectangular domains; while applying this, the concept of non-uniform and non-rectangular patch configurations in the physical domain is also adopted for the first time. An explicit representation of a patch dynamics scheme on a generalised curvilinear coordinate system in a two-dimensional domain is proposed for unsteady, linear, heterogeneous convection-diffusion-reaction (CDR) problems. The proposed scheme is validated through heterogeneous convection-diffusion-reaction and non-axisymmetric diffusion problems on generalised curvilinear coordinate systems. The results demonstrate excellent accuracy and show that the method significantly outperforms full-domain simulations in terms of computational efficiency, memory usage and overall performance.


### `karmakar-2024-generalized`

Karmakar, T. K., Dalal, D. C. (2024). *Generalized Patch Dynamics Scheme in Equation-free Multiscale Modelling.* <https://www.semanticscholar.org/paper/15bef289901b62472fdec22c5034b7562cab203e>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> There is a class of problems that exhibit smooth behavior on macroscopic scales, where only a microscopic evolution law is known. Patch dynamics scheme of `equation-free multiscale modelling' is one of the techniques, which aims to extract the macroscopic information using such known time-dependent microscopic model simulation in patches (which is a fraction of the space-time domain) that reduces the computational complexity. Here, extrapolation time step has an important role to reduce the error at macroscopic level. In this study, a generalized patch dynamics (GPD) scheme is proposed by distributing the gap-tooth timesteppers (GTTs) within each long (macroscopic) time step. This distribution is done in two ways, namely, GPD schemes of type-I and type-II. The proposed GPD scheme is based on three different time scales namely, micro, meso and macro to predict the system level behaviours. The GPD scheme of both types are capable of providing better accuracy with less computation time compared to the usual patch dynamics (UPD) scheme. The physical behaviours of the problems can be more appropriately addressed by the GPD scheme as one may use a non-uniform (variable) distribution of gap-tooth timesteppers (GTTs), as well as the extrapolation times based on the physics of the problem. Where the UPD scheme fails to converge for a long extrapolation time, both types of GPD schemes can be successfully applied. The whole method has been analyzed successfully for the one-dimensional reaction-diffusion problem.


### `liu-2024-robust`

Liu, Cheng, Hu, Yiding, Gao, Ruoqing, Hu, Changhong (2024). *Robust treatment for the coarse/fine interface of adaptive mesh in the simulation of two-phase flow.* Journal of Computational Physics 520:113485. DOI: [10.1016/j.jcp.2024.113485](https://doi.org/10.1016/j.jcp.2024.113485).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher landing page (ScienceDirect, via reader proxy))*

> We propose a numerical framework for simulating two-phase flow considering gravity and surface tension, and implement it with adaptive mesh. The framework employs a pressure compensation method and a jump model, ensuring accurate modeling of gravity and surface tension when the interface crosses between adaptive meshes of different refinement levels. Additional treatments have been developed to accommodate existing interface capturing and curvature estimation approaches for the adaptive mesh. The method is straightforward to implement and significantly reduces the unnecessary refinement around the free surface. Numerical validations demonstrate the robustness of the treatment for coarse/fine interfaces, with overall accuracy converging and comparable to results obtained with a uniform mesh. Furthermore, the numerical approach accurately reproduces the physics of bubble rising and jet capillary breakup.


### `prempeh-2024-exact`

Prempeh, K. O., George, Parker William, Bedrikovetsky, Pavel (2024). *Exact Solutions and Upscaling for 1D Two‐Phase Flow in Heterogeneous Porous Media.* Water Resources Research 60(11). DOI: [10.1029/2024wr037917](https://doi.org/10.1029/2024wr037917).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Upscaling of 1D two‐phase flows in heterogeneous porous media is important in interpretation of laboratory coreflood data, streamline quasi 3D modeling, and numerical reservoir simulation. In 1D heterogeneous media with properties varying along the flow direction, phase permeabilities are coordinate‐dependent. This yields the Buckley‐Leverett equation with coordinate‐dependent fractional flow f = f(s, x), which reflects the heterogeneity. So, an x‐dependency is considered to reflect microscale heterogeneity and averaging over x—upscaling. This work aims to average or upscale the heterogeneous system to obtain the homogenized media with such fractional flow function F(S) that provides the same water‐cut history at the reservoir outlet, x = 1. Thus, F(S) is an equivalent property of the medium. So far, the exact upscaling for 1D micro heterogeneous systems has not been derived. With the x‐dependency of fractional flow, the Riemann invariant is flux f, which yields exact integration of 1D flow problems. The novel exact solutions are derived for flows with continuous saturation profile, transition of shock into continuous wave, transition of continuous wave into shock, and transport in heterogeneous piecewise‐uniform rocks. The exact procedure of upscaling from f = f(s, x) to F(S) is as follows: the inverse function to the upscaled F(S) is equal to the averaged saturation over x of the inverse microscale function s = f −1(f, x). It was found that the Welge's method as applied to heterogeneous cores provides the upscaled F(S). For characteristic finite‐difference scheme, the fluxes for microscale and upscaled‐numerical‐cell systems, coincide in all grid nodes.


### `tranduc-2024-efficient`

Tran-Duc, T., Bunder, J., Roberts, A. (2024). *Efficient prediction of static and dynamical responses of functional graded beams using sparse multiscale patches.* Computational Mechanics 76(3):567--588. DOI: [10.1007/s00466-025-02614-4](https://doi.org/10.1007/s00466-025-02614-4).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We develop a multiscale patch scheme for studying the system level characteristics of heterogeneous functional graded beams in 3D via accurate computational homogenisation. The algorithm is an extension of our previous work for 2D beams (Tran-Duc et al. in Int. J. Solids Struct. 292:112719, 2024) to explore out-of-plane dynamics of 3D beams of functional graded materials. The scheme computes the detailed microscale elastic equations only in sparsely spaced, small patches of the domain (akin to fe2\documentclass[12pt]minimal \usepackageamsmath \usepackagewasysym \usepackageamsfonts \usepackageamssymb \usepackageamsbsy \usepackagemathrsfs \usepackageupgreek \setlength\oddsidemargin-69pt \begindocument$$ ^2$$\enddocument), and via symmetry-preserving interpolation between these patches. We develop new applications of the scheme to two classes of functionally graded beams, namely cross-sectionally graded and axially graded. Our approach accurately and provably predicts the macroscale system-wide behaviour. Beam deflection and natural frequencies from the patch computations agree very well with both existing experimental data and the full-domain computations, which provides a new validation of the approach and a new characterisation of the interaction between bending and twisting in graduated beams. The scheme is stable and robust, with errors consistently small and controllable by varying the number of patches. The reduction in the spatial domain of computation substantially improves the computational efficiency, with the computational time reducing by a factor of up to 17\documentclass[12pt]minimal \usepackageamsmath \usepackagewasysym \usepackageamsfonts \usepackageamssymb \usepackageamsbsy \usepackagemathrsfs \usepackageupgreek \setlength\oddsidemargin-69pt \begindocument$$17$$\enddocument when the patches cover 27%\documentclass[12pt]minimal \usepackageamsmath \usepackagewasysym \usepackageamsfonts \usepackageamssymb \usepackageamsbsy \usepackagemathrsfs \usepackageupgreek \setlength\oddsidemargin-69pt \begindocument$$27\%$$\enddocument of the beam. The scheme also accurately predicts the homogenised dynamics of periodic micro-structured materials, such as metamaterials, by simply ensuring patches are a multiple of the micro-period. Localised phenomena, such as material failures or cracks or boundary layers, may also be accurately encompassed by fully resolving them within a patch.


### `wang-2024-coarse`

Wang, Chuwei (2024). *Coarse Graining with Neural Operators for Simulating Chaotic Systems.* <https://www.semanticscholar.org/paper/a30523b227c7ca5b88474db415d685cbbf5c35a8>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Accurately predicting the long-term behavior of chaotic systems is crucial for various applications such as climate modeling. However, achieving such predictions typically requires iterative computations over a dense spatiotemporal grid to account for the unstable nature of chaotic systems, which is expensive and impractical in many real-world situations. An alternative approach to such a full-resolved simulation is using a coarse grid and then correcting its errors through a \textitclosure model, which approximates the overall information from fine scales not captured in the coarse-grid simulation. Recently, ML approaches have been used for closure modeling, but they typically require a large number of training samples from expensive fully-resolved simulations (FRS). In this work, we prove an even more fundamental limitation, i.e., the standard approach to learning closure models suffers from a large approximation error for generic problems, no matter how large the model is, and it stems from the non-uniqueness of the mapping. We propose an alternative end-to-end learning approach using a physics-informed neural operator (PINO) that overcomes this limitation by not using a closure model or a coarse-grid solver. We first train the PINO model on data from a coarse-grid solver and then fine-tune it with (a small amount of) FRS and physics-based losses on a fine grid. The discretization-free nature of neural operators means that they do not suffer from the restriction of a coarse grid that closure models face, and they can provably approximate the long-term statistics of chaotic systems. In our experiments, our PINO model achieves a 330x speedup compared to FRS with a relative error $\sim 10\%$. In contrast, the closure model coupled with a coarse-grid solver is $60$x slower than PINO while having a much higher error $\sim186\%$ when the closure model is trained on the same FRS dataset.


### `buvoli-2025-multirate`

Buvoli, Tommaso, Tran, Brian K., Southworth, Benjamin (2025). *Multirate Runge-Kutta for Nonlinearly Partitioned Systems.* abs/2504.03257. DOI: [10.48550/arxiv.2504.03257](https://doi.org/10.48550/arxiv.2504.03257).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Multirate integration is an increasingly relevant tool that enables scientists to simulate multiphysics systems. Existing multirate methods are designed for equations whose fast and slow variables can be linearly separated using additive or component-wise partitions. However, in realistic applications, this assumption is not always valid. Building on the recently developed class of nonlinearly partitioned Runge-Kutta (NPRK) methods, we develop a framework for multirate NPRK (MR-NPRK) that allows for arbitrary nonlinear splittings of the evolution operator. We discuss order conditions, formalize different types of coupling between timescales, and analyze joint linear stability of MR-NPRK methods. We then introduce a class of 2nd- and 3rd-order methods, referred to as ``implicitly-wrapped'' multirate methods, that combine a user-specified explicit method for integrating the fast timescale with several slow implicit stages. These methods are designed to be algorithmically simple with low memory costs and minimal operator evaluations. Lastly, we conduct numerical experiments to validate our proposed methods and show the benefits of multirating a nonlinear partition.


### `karmakar-2025-generalised`

Karmakar, T. K., Dalal, D. C. (2025). *Generalised patch dynamics schemes in equation-free multiscale modelling.* Journal of Computational Physics 548:114560. DOI: [10.1016/j.jcp.2025.114560](https://doi.org/10.1016/j.jcp.2025.114560).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher landing page (ScienceDirect, via reader proxy))*

> There is a class of problems that exhibits smooth behaviours on macroscopic scales, where only microscopic evolution laws are known. Patch dynamics scheme of 'equation-free multiscale modelling' is one of the techniques that aims to extract the macroscopic information using such known time-dependent microscopic model simulation in the patches (which is a fraction of the space-time domain) that reduce the computational complexity. In this study, we propose generalised patch dynamics (GPD) schemes by distributing the gap-tooth time-steppers (GTTs) within each long (macroscopic) time step. Based on the distribution of GTTs, the proposed GPD schemes are classified as GPD scheme of type-I (GPD-I) and of type-II (GPD-II). The proposed GPD schemes are based on three different time scales, namely, micro, meso and macro, to predict the system-level behaviours. In GPD schemes, number of gap-tooth time-steppers, number of scale bridgings, and the extrapolation time step sizes have important roles in reducing the error at the macroscopic level. The GPD schemes of both types are capable of providing a better accuracy with less computational time compared to the usual patch dynamics (UPD) scheme. Physical behaviours of practical problems could be more appropriately addressed by the GPD schemes as one may use a non-uniform (variable) distribution of gap-tooth time-steppers (GTTs), as well as the extrapolation time step sizes based on the physical behaviours of the problem. We successfully applied both types of GPD schemes to the problems in which the UPD scheme fails to converge over a long extrapolation time step size. We analysed the whole method for a one-dimensional reaction-diffusion problem. Along with this, we effectively solved advection-diffusion-reaction equation and nonlinear heterogeneous problem using the proposed GPD schemes.


### `reynolds-2025-efficient`

Reynolds, Daniel R., Amihere, Sylvia, Mitchell, D., Luan, Vu Thai (2025). *Efficient and Flexible Multirate Temporal Adaptivity.* abs/2510.14964. DOI: [10.48550/arxiv.2510.14964](https://doi.org/10.48550/arxiv.2510.14964).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> In this work we present two new families of multirate time step adaptivity controllers, that are designed to work with embedded multirate infinitesimal (MRI) time integration methods for adapting time steps when solving problems with multiple time scales. We compare these controllers against competing approaches on two benchmark problems, showing that the proposed methods offer dramatically improved performance and flexibility. The combination of embedded MRI methods and the proposed controllers enable adaptive simulations of problems with a potentially arbitrary number of time scales, achieving high accuracy while maintaining low computational cost. Additionally, we introduce a new set of embeddings for the family of explicit multirate exponential Runge--Kutta (MERK) methods of orders 2 through 5, resulting in the first-ever fifth-order embedded MRI method. Finally, we compare the performance of a wide range of embedded MRI methods on our benchmark problems to provide guidance on how to select an appropriate MRI method and multirate controller.


### `roberts-2025-time`

Roberts, S. (2025). *New Time Integrators and Capabilities in SUNDIALS Versions 6.2.0-7.4.0.* ACM Transactions on Mathematical Software 52(1):1--14. DOI: [10.1145/3797888](https://doi.org/10.1145/3797888).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> SUNDIALS is a well-established numerical library that provides robust and efficient time integrators and nonlinear solvers. This article overviews several significant improvements and new features added over the last 3 years to support scientific simulations run on high-performance computing systems. Notably, three new classes of one-step methods have been implemented: low storage Runge–Kutta, symplectic partitioned Runge–Kutta, and operator splitting. In addition, we describe new timestep adaptivity support for multirate methods, adjoint sensitivity analysis capabilities for explicit Runge–Kutta methods, additional options for Anderson acceleration in nonlinear solvers, and improved error handling and logging.


### `jung-2026-learning`

Jung, Ju-Won, Constantinescu, Emil (2026). *Learning Differentiable Weak-Form Corrections to Accelerate Finite Element Simulations.* <https://www.semanticscholar.org/paper/e228452bcb66980c953a44082515683f08c78a1b>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We present a differentiable weak-form learning approach for accelerating finite element simulations. Rather than introducing black-box source terms in the strong form of the governing equations, we augment the momentum equation directly in the variational (weak) form with parameterized bilinear operators. The coefficients of these operators are learned from high-resolution simulations so that unresolved small-scale dynamics can be represented on coarse grids. Applying the correction at the weak-form level aligns the learned model with the finite element discretization, preserving key numerical structure and better respecting the fundamental properties of incompressible flow. In the same setting, the approach yields solutions that are more accurate and more stable over long time horizons than comparable strong-form corrections. We implement the proposed method in the Firedrake finite element solver and evaluate it on benchmark problems, including the one-dimensional convection-diffusion equation and the two-dimensional incompressible Navier-Stokes equations. End-to-end differentiable training is enabled by coupling PyTorch with the Firedrake adjoint framework. Across these tests, the learned variational operators improve long-term accuracy while reducing computational cost. Overall, our results suggest that weak-form learning provides a principled, structure-preserving route to accurate and stable coarse-grid simulations of incompressible flows.


### `li-2026-physics-informed`

Li, Xingkai, Jiang, Yuanjun, Yu, Zhuang, Hu, Xiaobo, Zhao, Binbin (2026). *A physics-informed multi-scale fourier neural operator framework for snow avalanche dynamics simulation.* Journal of Hydrology 675:135573. DOI: [10.1016/j.jhydrol.2026.135573](https://doi.org/10.1016/j.jhydrol.2026.135573).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher landing page (ScienceDirect, via reader proxy))*

> Avalanches are typical natural disasters in alpine watersheds. Near-real-time early warning and multi-scenario risk assessment require a balance between physical consistency and computational efficiency, yet traditional numerical solvers often suffer from severe computational bottlenecks in high-resolution simulations or large-scale scenario ensemble computations. To address this issue, this study proposes a Physics-Informed Neural Operator (PINO) framework for the efficient prediction of avalanche dynamic processes. The framework uses the Fourier Neural Operator as the backbone network and incorporates residual constraints of governing equations. By introducing architectural enhancements including dimensionless physical loss, a spatial feature extractor, and a channel attention mechanism, the model's ability to learn complex avalanche evolution processes under varying terrain and physical parameters is significantly improved. Meanwhile, an active-region-prioritized sampling scheme and a progressive training strategy further enhance training efficiency. The model is trained and evaluated solely on 12 heterogeneous multi-scenario datasets generated by the RAMMS software. Results demonstrate that the model achieves relatively stable accuracy under cross-parameter and cross-terrain conditions, and its performance is significantly superior to traditional Physics-Informed Neural Networks (PINNs). Its zero-shot super-resolution performance also outperforms the standard FNO. Benchmark tests show that the inference time reaches the near-real-time level of seconds to tens of seconds, providing a feasible and novel paradigm for multi-scenario risk assessment and near-real-time prediction of avalanches and other shallow surface flow dynamic processes in alpine watersheds.


### `saurabh-2026-field`

Saurabh, K., Khanwale, Makrand A., Ishii, Masado, Sundar, Hari, Ganapathysubramanian, B. (2026). *Field conserving adaptive mesh refinement (AMR) scheme on massively parallel adaptive octree meshes.* abs/2602.07817. DOI: [10.48550/arxiv.2602.07817](https://doi.org/10.48550/arxiv.2602.07817).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Adaptive mesh refinement (AMR) is widely used to efficiently resolve localized features in time-dependent partial differential equations (PDEs) by selectively refining and coarsening the mesh. However, in long-horizon simulations, repeated intergrid interpolations can introduce systematic drift in conserved quantities, especially for variational discretizations with continuous basis functions. While interpolation from parent-to-child during refinement in continuous Galerkin (CG) discretizations is naturally conservative, the standard injection-based child-to-parent coarsening interpolation is generally not. We propose a simple, scalable field-conserving coarsening operator for parallel, octree-based AMR. The method enforces discrete global conservation during coarsening by first computing field conserving coarse-element values at quadrature points and then recovering coarse nodal degrees of freedom via an $L^2$ projection (mass-matrix solve), which simultaneously controls the $L_2$ error. We evaluate the approach on mass-conserving phase-field models, including the Cahn--Hilliard and Cahn--Hilliard--Navier--Stokes systems, and compare against injection in terms of conservation error, solution quality, and computational cost.



---

## F. Landscape evolution models & drainage routing

*69 entries.*

### `montgomery-1994-physically`

Montgomery, David R., Dietrich, William E. (1994). *A Physically Based Model for the Topographic Control on Shallow Landsliding.* Water Resources Research 30(4):1153--1171. DOI: [10.1029/93WR02979](https://doi.org/10.1029/93WR02979).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> A model for the topographic influence on shallow landslide initiation is developed by coupling digital terrain data with near‐surface through flow and slope stability models. The hydrologic model TOPOG (O'Loughlin, 1986) predicts the degree of soil saturation in response to a steady state rainfall for topographic elements defined by the intersection of contours and flow tube boundaries. The slope stability component uses this relative soil saturation to analyze the stability of each topographic element for the case of cohesionless soils of spatially constant thickness and saturated conductivity. The steady state rainfall predicted to cause instability in each topographic element provides a measure of the relative potential for shallow landsliding. The spatial distribution of critical rainfall values is compared with landslide locations mapped from aerial photographs and in the field for three study basins where high‐resolution digital elevation data are available: Tennessee Valley in Marin County, California; Mettman Ridge in the Oregon Coast Range; and Split Creek on the Olympic Peninsula, Washington. Model predictions in each of these areas are consistent with spatial patterns of observed landslide scars, although hydrologic complexities not accounted for in the model (e.g., spatial variability of soil properties and bedrock flow) control specific sites and timing of debris flow initiation within areas of similar topographic control.


### `whipple-1999-dynamics`

Whipple, Kelin X., Tucker, Gregory E. (1999). *Dynamics of the stream-power river incision model: Implications for height limits of mountain ranges, landscape response timescales, and research needs.* Journal of Geophysical Research: Solid Earth 104(B8):17661--17674. DOI: [10.1029/1999JB900120](https://doi.org/10.1029/1999JB900120).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> The longitudinal profiles of bedrock channels are a major component of the relief structure of mountainous drainage basins and therefore limit the elevation of peaks and ridges. Further, bedrock channels communicate tectonic and climatic signals across the landscape, thus dictating, to first order, the dynamic response of mountainous landscapes to external forcings. We review and explore the stream‐power erosion model in an effort to (1) elucidate its consequences in terms of large‐scale topographic (fluvial) relief and its sensitivity to tectonic and climatic forcing, (2) derive a relationship for system response time to tectonic perturbations, (3) determine the sensitivity of model behavior to various model parameters, and (4) integrate the above to suggest useful guidelines for further study of bedrock channel systems and for future refinement of the streampower erosion law. Dimensional analysis reveals that the dynamic behavior of the stream‐power erosion model is governed by a single nondimensional group that we term the uplift‐erosion number, greatly reducing the number of variables that need to be considered in the sensitivity analysis. The degree of nonlinearity in the relationship between stream incision rate and channel gradient (slope exponent n) emerges as a fundamental unknown. The physics of the active erosion processes directly influence this nonlinearity, which is shown to dictate the relationship between the uplift‐erosion number, the equilibrium stream channel gradient, and the total fluvial relief of mountain ranges. Similarly, the predicted response time to changes in rock uplift rate is shown to depend on climate, rock strength, and the magnitude of tectonic perturbation, with the slope exponent n controlling the degree of dependence on these various factors. For typical drainage basin geometries the response time is relatively insensitive to the size of the system. Work on the physics of bedrock erosion processes, their sensitivity to extreme floods, their transient responses to sudden changes in climate or uplift rate, and the scaling of local rock erosion studies to reach‐scale modeling studies are most sorely needed.


### `davy-2009-fluvial`

Davy, Philippe, Lague, Dimitri (2009). *Fluvial Erosion/Transport Equation of Landscape Evolution Models Revisited.* Journal of Geophysical Research: Earth Surface 114:F03007. DOI: [10.1029/2008JF001146](https://doi.org/10.1029/2008JF001146).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> We present a mesoscale erosion/deposition model, which differs from previous landscape evolution models equations by taking explicitly into account a mass balance equation for the streamflow. The geological and hydrological complexity is lumped into two basic fluxes (erosion and deposition) and two averaged parameters (unit width discharge q and stream slope s). The model couples the dynamics of streamflow and topography through a sediment transport length function ξ(q), which is the average travel distance of a particle in the flow before being trapped on topography. This property reflects a time lag between erosion and deposition, which allows the streamflow not to be instantaneously at capacity. The so‐called ξ‐q model may reduce either to transport‐limited or to detachment‐limited erosion modes depending on ξ. But it also may not. We show in particular how it does or does not for steady state topographies, long‐term evolution, and high‐frequency base level perturbations. Apart from the unit width discharge and the settling velocity, the ξ(q) function depends on a dimensionless number encompassing the way sediment is transported within the streamflow. Using models of concentration profile through the water column, we show the dependency of this dimensionless coefficient on the Rouse number. We discuss how consistent the ξ‐q model framework is with bed load scaling expressions and Einstein's conception of sediment motion.


### `pelletier-2010-minimizing`

Pelletier, Jon D. (2010). *Minimizing the grid-resolution dependence of flow-routing algorithms for geomorphic applications.* Geomorphology 122:91--98. DOI: [10.1016/j.geomorph.2010.06.001](https://doi.org/10.1016/j.geomorph.2010.06.001).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract:** ABSTRACT NOT RETRIEVED. Elsevier (Geomorphology) deposits no abstract; Crossref, Semantic Scholar and OpenAlex all return null.


### `braun-2013-very`

Braun, Jean, Willett, Sean D. (2013). *A very efficient O(n), implicit, and parallel method to solve the stream power equation governing fluvial incision and landscape evolution.* Geomorphology 180--181:170--179. DOI: [10.1016/j.geomorph.2012.10.008](https://doi.org/10.1016/j.geomorph.2012.10.008).

**Availability:** **in-repo PDF** — `ref/geology/pdfs/braun-2013-very.pdf` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We present a new algorithm to solve the basic stream power equation, which governs channel incision and landscape evolution in many geomorphic settings. The algorithm is highly efficient because computation time increases linearly with the number of points used to discretize the landscape and is ideally suited to parallelization. It is also unconditionally stable because it uses an implicit scheme for the time integration of the landscape evolution equation, which means that large time steps can be used without sacrificing accuracy. In this paper we describe the algorithm and present results that demonstrate its efficiency and accuracy.


### `barnes-2014-priority`

Barnes, Richard, Lehman, Clarence, Mulla, David (2014). *Priority-Flood: An Optimal Depression-Filling and Watershed-Labeling Algorithm for Digital Elevation Models.* Computers and Geosciences 62:117--127. DOI: [10.1016/j.cageo.2013.04.024](https://doi.org/10.1016/j.cageo.2013.04.024).

**Availability:** PDF in relata store

**Abstract** *(source: Semantic Scholar)*

> Depressions (or pits) are areas within a digital elevation model that are surrounded by higher terrain, with no outlet to lower areas. Filling them so they are level, as fluid would fill them if the terrain was impermeable, is often necessary in preprocessing DEMs. The depression-filling algorithm presented here - called Priority-Flood - unifies and improves the work of a number of previous authors who have published similar algorithms. The algorithm operates by flooding DEMs inwards from their edges using a priority queue to determine the next cell to be flooded. The resultant DEM has no depressions or digital dams: every cell is guaranteed to drain. The algorithm is optimal for both integer and floating-point data, working in O(n) and O(nlog"2n) time, respectively. It is shown that by using a plain queue to fill depressions once they have been found, an O(mlog"2m) time-complexity can be achieved, where m does not exceed the number of cells n. This is the lowest time complexity of any known floating-point depression-filling algorithm. In testing, this improved variation of the algorithm performed up to 37% faster than the original. Additionally, a parallel version of an older, but widely used, depression-filling algorithm required six parallel processors to achieve a run-time on par with what the newer algorithm's improved variation took on a single processor. The Priority-Flood Algorithm is simple to understand and implement: the included pseudocode is only 20 lines and the included C++ reference implementation is under a hundred lines. The algorithm can work on irregular meshes as well as 4-, 6-, 8-, and n-connected grids. It can also be adapted to label watersheds and determine flow directions through either incremental elevation changes or depression carving. In the case of incremental elevation changes, the algorithm includes safety checks not present in prior works.


### `chen-2014-landscape`

Chen, A., Darbon, J., Morel, J.-M. (2014). *Landscape evolution models: A review of their fundamental equations.* Geomorphology 219:68--86. DOI: [10.1016/j.geomorph.2014.04.037](https://doi.org/10.1016/j.geomorph.2014.04.037).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract:** ABSTRACT NOT RETRIEVED. Elsevier (Geomorphology) deposits no abstract; Crossref, Semantic Scholar and OpenAlex all return null.


### `lague-2014-stream`

Lague, Dimitri (2014). *The Stream Power River Incision Model: Evidence, Theory and Beyond.* Earth Surface Processes and Landforms 39(1):38--61. DOI: [10.1002/esp.3462](https://doi.org/10.1002/esp.3462).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> ABSTRACTThe stream power incision model (SPIM) is a cornerstone of quantitative geomorphology. It states that river incision rate is the product of drainage area and channel slope raised to the power exponents m and n, respectively. It is widely used to predict patterns of deformation from channel long profile inversion or to model knickpoint migration and landscape evolution. Numerous studies have attempted to test its applicability with mixed results prompting the question of its validity. This paper synthesizes these results, highlights the SPIM deficiencies, and offers new insights into the role of incision thresholds and channel width. By reviewing quantitative data on incising rivers, I first propose six sets of field evidence that any long‐term incision model should be able to predict. This analysis highlights several inconsistencies of the standard SPIM. Next, I discuss the methods used to construct physics‐based long‐term incision laws. I demonstrate that all published incising river datasets away from knickpoints or knickzones are in a regime dominated by threshold effects requiring an explicit upscaling of flood stochasticity neglected in the standard SPIM and other incision models. Using threshold‐stochastic simulations with dynamic width, I document the existence of composite transient dynamics where knickpoint propagation locally obeys a linear SPIM (n=1) while other part of the river obey a non‐linear SPIM (n&gt;1). The threshold‐stochastic SPIM resolves some inconsistencies of the standard SPIM and matches steady‐state field evidence when width is not sensitive to incision rate. However it fails to predict the scaling of slope with incision rate for cases where width decreases with incision rate. Recent proposed models of dynamic width cannot resolve these deficiencies. An explicit upscaling of sediment flux and threshold‐stochastic effects combined with dynamic width should take us beyond the SPIM which is shown here to have a narrow range of validity. Copyright © 2013 John Wiley & Sons, Ltd.


### `barnes-2016-parallel`

Barnes, Richard (2016). *Parallel non-divergent flow accumulation for trillion cell digital elevation models on desktops or clusters.* Environmental Modelling & Software 92:202--212. DOI: [10.1016/j.envsoft.2017.02.022](https://doi.org/10.1016/j.envsoft.2017.02.022).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Continent-scale datasets challenge hydrological algorithms for processing digital elevation models. Flow accumulation is an important input for many such algorithms; here, I parallelize its calculation. The new algorithm works on one or many cores, or multiple machines, and can take advantage of large memories or cope with small ones. Unlike previous parallel algorithms, the new algorithm guarantees a fixed number of memory access and communication events per raster cell. In testing, the new algorithm ran faster and used fewer resources than previous algorithms, exhibiting 30% strong and weak scaling efficiencies up to 48 cores and linear scaling across datasets ranging over three orders of magnitude. The largest dataset tested had two trillion (21012) cells. With 48 cores, processing required 24min wall-time (14.5 compute-hours). This test is three orders of magnitude larger than any previously performed in the literature. Complete, well-commented source code and correctness tests are available on Github. I present a parallelized ow accumulation algorithm for large DEMs.The algorithm guarantees fixed numbers of memory and communication events.The algorithm runs in serial on a standard desktop or in parallel on a supercomputer.The algorithm performs tests three orders of magnitude larger than any previous work.Well-commented source code and an automated test suite are available on Github.


### `campforts-2016-ttlem`

Campforts, B., Schwanghart, W. (2016). *TTLEM - an implicit-explicit (IMEX) scheme for modelling landscape evolution in MATLAB.* 18. <https://www.semanticscholar.org/paper/e4f8fa6b1a612996b73af36200e2c6d7df054a18>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Landscape evolution models (LEM) are essential to unravel interdependent earth surface processes. They are proven very useful to bridge several temporal and spatial timescales and have been successfully used to integrate existing empirical datasets. There is a growing consensus that landscapes evolve at least as much in the horizontal as in the vertical direction urging for an efficient implementation of dynamic drainage networks. Here we present a spatially explicit LEM, which is based on the object-oriented function library TopoToolbox 2 (Schwanghart and Scherler, 2014). Similar to other LEMs, rivers are considered to be the main drivers for simulated landscape evolution as they transmit pulses of tectonic perturbations and set the base level of surrounding hillslopes. Highly performant graph algorithms facilitate efficient updates of the flow directions to account for planform changes in the river network and the calculation of flow-related terrain attributes. We implement the model using an implicit-explicit (IMEX) scheme, i.e. different integrators are used for different terms in the diffusion-incision equation. While linear diffusion is solved using an implicit scheme, we calculate incision explicitly. Contrary to previously published LEMS, however, river incision is solved using a total volume method which is total variation diminishing in order to prevent numerical diffusion when solving the stream power law (Campforts and Govers, 2015). We show that the use of this updated numerical scheme alters both landscape topography and catchment wide erosion rates at a geological time scale. Finally, the availability of a graphical user interface facilitates user interaction, making the tool very useful both for research and didactical purposes.


### `salles-2016-badlands`

Salles, T. (2016). *Badlands: A parallel basin and landscape dynamics model.* SoftwareX 5:195--202. DOI: [10.1016/j.softx.2016.08.005](https://doi.org/10.1016/j.softx.2016.08.005).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Over more than three decades, a number of numerical landscape evolution models (LEMs) have been developed to study the combined effects of climate, sea-level, tectonics and sediments on Earth surface dynamics. Most of them are written in efficient programming languages, but often cannot be used on parallel architectures. Here, I present a LEM which ports a common core of accepted physical principles governing landscape evolution into a distributed memory parallel environment. Badlands (acronym for BAsin anD LANdscape DynamicS) is an open-source, flexible, TIN-based landscape evolution model, built to simulate topography development at various space and time scales.


### `salles-2016-badlands-open-source`

Salles, T., Hardiman, L. (2016). *Badlands: An open-source, flexible and parallel framework to study landscape dynamics.* Computers & Geosciences 91:77--89. DOI: [10.1016/j.cageo.2016.03.011](https://doi.org/10.1016/j.cageo.2016.03.011).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher-affiliated mirror (EarthByte))*

> In this paper, we propose a minimal numerical model which governing equations describe the following processes: erosion, sedimentation, diffusion and flexure. The model respects conservation laws for water and sediment. The implementation is based on a finite volume approach and the explicit solution stability is ensured by a CFL-like condition. This common core of accepted physical principles governing landscape evolution is ported into a distributed memory parallel environment. Badlands (acronym for BAsin anD LANdscape DynamicS) is an open-source, flexible, TIN-based landscape evolution model, built to simulate landform development and test source-to-sink concepts at regional to continental scale over thousands to millions of years. To illustrate the model capabilities, we first present an example of delta evolution under sea-level fluctuations. The model predicts the successive progradation and transgression phases, the development of depositional and erosional patterns as well as the associated stratigraphic formation. Then, we investigate the importance of climate, and in particular the spatial pattern of precipitation, on the topographic evolution of mountain belts. The simulation and associated quantitative analyses suggest that the main drainage divide migrates and asymmetric topography develops in response to orographic precipitation. This mechanism, documented in recent analogue and numerical experiments, results in a complex reorganisation of drainage networks that our model is able to reproduce.


### `barnes-2018-accelerating`

Barnes, Richard (2018). *Accelerating a fluvial incision and landscape evolution model with parallelism.* Geomorphology abs/1803.02977:28-39. DOI: [10.1016/j.geomorph.2019.01.002](https://doi.org/10.1016/j.geomorph.2019.01.002).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract Solving inverse problems, performing sensitivity analyses, and achieving statistical rigour in landscape evolution models require running many model realizations. Parallel computation is necessary to achieve this in a reasonable time. However, no previous landscape evolution algorithm is able to leverage modern parallelism. Here, I describe an algorithm that can utilize the parallel potential of GPUs and many-core processors, in addition to working well in serial. The new algorithm runs 43× faster (70 s vs. 3000 s on a 10,000×10,000 input) than the previous state-of-the-art and exhibits sublinear scaling with input size. I also identify key techniques for multiple flow direction routing and quickly eliminating landscape depressions and local minima. Complete, well-commented, easily adaptable source code for all versions of the algorithm is available on Github and Zenodo.


### `barnes-2018-richdem`

Barnes, Richard (2018). *RichDEM: High-performance terrain analysis.* DOI: [10.7287/peerj.preprints.27099v1](https://doi.org/10.7287/peerj.preprints.27099v1).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> To answer geomorphological questions at unprecedented spatial and temporal scales, we need to (a) parse terabyte-scale datasets (DEMs), (b) perform millions of model realizations to pinpoint the parameters which govern landscape evolution, and (c) do so with statistical rigor, which may require thousands of additional realizations. A core set of operations underpin many geomorphic models. These include determination of terrain attributes such as slope and curvature; flow routing; depression flooding and breaching; flat resolution; and flow accumulation. Here, I present RichDEM, a high-performance C++ library and set of wrappers for performing these operations. The library incorporates a number of options for performing each operation and makes full use of modern high-performance capabilities. The library can scale to process DEMs of over one trillion cells and operates effectively on laptops or supercomputers.


### `bonetti-2018-theory`

Bonetti, S., Bragg, A. D., Porporato, A. (2018). *On the theory of drainage area for regular and non-regular points.* Proceedings of the Royal Society A 474(2211):20170693. DOI: [10.1098/rspa.2017.0693](https://doi.org/10.1098/rspa.2017.0693).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> The drainage area is an important, non-local property of a landscape, which controls surface and subsurface hydrological fluxes. Its role in numerous ecohydrological and geomorphological applications has given rise to several numerical methods for its computation. However, its theoretical analysis has lagged behind. Only recently, an analytical definition for the specific catchment area was proposed (Gallant & Hutchinson. 2011 Water Resour. Res. 47 , W05535. ( doi:10.1029/2009WR008540 )), with the derivation of a differential equation whose validity is limited to regular points of the watershed. Here, we show that such a differential equation can be derived from a continuity equation (Chen et al. 2014 Geomorphology 219 , 68–86. ( doi:10.1016/j.geomorph.2014.04.037 )) and extend the theory to critical and singular points both by applying Gauss's theorem and by means of a dynamical systems approach to define basins of attraction of local surface minima. Simple analytical examples as well as applications to more complex topographic surfaces are examined. The theoretical description of topographic features and properties, such as the drainage area, channel lines and watershed divides, can be broadly adopted to develop and test the numerical algorithms currently used in digital terrain analysis for the computation of the drainage area, as well as for the theoretical analysis of landscape evolution and stability.


### `braun-2018-highly`

Braun, J., Cordonnier, G., Bovy, B., Yuan, Xiaoping (2018). *Highly efficient methods to solve the Stream Power Law including sediment transport, local minima resolution and multi-direction flow.* :17723. <https://www.semanticscholar.org/paper/7bdac2f49d01e3b40ea2ebfc8e5cba00d8d4c9fd>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Over the past few years we have continued the development of efficient methods and algorithms to model landscape evolution. The main purpose of our efforts is to obtain methods that can be inserted into an optimization (Bayesian) scheme to invert geological observations such as present-day landform, thermochronological and barometric data or sedimentary flux data, in order to obtain relevant constraints on local uplift rate, its evolution through time, as well as the value of model parameters. This often requires that hundreds of thousands to millions of forward model runs be performed to explore parameter space. This can only be achieved if the forward model run takes a few minutes of compute time, at most, to simulate tens of millions of years of landscape evolution at a spatial resolution that is relevant for the process being modeled (i.e., grid size or number of points used to discretize the model, n, of 1000 × 1000 or more). This is why we are currently developing methods that are implicit in time and thus allow for very large time step lengths (10 − 10 yrs), and are O(n), i.e. where the number of operations increases linearly with n.


### `chandra-2018-bayeslands`

Chandra, Rohitash, Azam, Danial, Müller, R., Salles, T., Cripps, Sally (2018). *BayesLands: A Bayesian inference approach for parameter uncertainty quantification in Badlands.* Computers & Geosciences abs/1805.03696:89-101. DOI: [10.1016/j.cageo.2019.06.012](https://doi.org/10.1016/j.cageo.2019.06.012).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract Bayesian inference provides a rigorous methodology for estimation and uncertainty quantification of parameters in geophysical forward models. Badlands (basin and landscape dynamics model) is a landscape evolution model that simulates topography development at various space and time scales. Badlands consists of a number of geophysical parameters that needs estimation with appropriate uncertainty quantification; given the observed present-day ground truth such as surface topography and the stratigraphy of sediment deposition through time. The inference of unknown parameters is challenging due to the scarcity of data, sensitivity of the parameter setting and complexity of the Badlands model. In this paper, we take a Bayesian approach to provide inference using Markov chain Monte Carlo sampling (MCMC). We present Bayeslands ; a Bayesian framework for Badlands that fuses information obtained from complex forward models with observational data and prior knowledge. As a proof-of-concept, we consider a synthetic and real-world topography with two parameters for Bayeslands inference, namely precipitation and erodibility. The results of the experiments show that Bayeslands yields a promising distribution of the parameters. Moreover, we demonstrate the challenge in sampling irregular and multi-modal posterior distributions using a likelihood surface that has a range of sub-optimal modes.


### `cordonnier-2018-versatile`

Cordonnier, G., Bovy, B., Braun, J. (2018). *A versatile, linear complexity algorithm for flow routing in topographies with depressions.* Earth Surface Dynamics 7(2):549-562. DOI: [10.5194/esurf-7-549-2019](https://doi.org/10.5194/esurf-7-549-2019).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. We present a new algorithm for solving the common problem of flow trapped in closed depressions within digital elevation models, as encountered in many applications relying on flow routing. Unlike other approaches (e.g., the Priority-Flood depression filling algorithm), this solution is based on the explicit computation of the flow paths both within and across the depressions through the construction of a graph connecting together all adjacent drainage basins. Although this represents many operations, a linear time complexity can be reached for the whole computation, making it very efficient. Compared to the most optimized solutions proposed so far, we show that this algorithm of flow path enforcement yields the best performance when used in landscape evolution models. In addition to its efficiency, our proposed method also has the advantage of letting the user choose among different strategies of flow path enforcement within the depressions (i.e., filling vs. carving). Furthermore, the computed graph of basins is a generic structure that has the potential to be reused for solving other problems as well, such as the simulation of erosion. This sequential algorithm may be helpful for those who need to, e.g., process digital elevation models of moderate size on single computers or run batches of simulations as part of an inference study.


### `omalley-2018-large`

O'Malley, C., White, N. J., Stephenson, S., Roberts, G. (2018). *Large‐Scale Tectonic Forcing of the African Landscape.* Journal of Geophysical Research: Earth Surface 126(12). DOI: [10.1029/2021jf006345](https://doi.org/10.1029/2021jf006345).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Successful inverse modeling of observed longitudinal river profiles suggests that fluvial landscapes are responsive to continent‐wide tectonic forcing. However, inversion algorithms make simplifying assumptions about landscape erodibility and drainage planform stability that require careful justification. For example, precipitation rate and drainage catchment area are usually assumed to be invariant. Here, we exploit a closed‐loop modeling strategy by inverting drainage networks generated by dynamic landscape simulations in order to investigate the validity of these assumptions. First, we invert 4,018 African river profiles to determine an uplift history that is independently calibrated, and subsequently validated, using separate suites of geologic observations. Second, we use this tectonic forcing to drive landscape simulations that permit divide migration, interfluvial erosion and changes in catchment size. These simulations reproduce large‐scale features of the African landscape, including growth of deltaic deposits. Third, the influence of variable precipitation is investigated by carrying out a series of increasingly severe tests. Inverse modeling of drainage inventories extracted from simulated landscapes can largely recover tectonic forcing. Our closed‐loop modeling strategy suggests that large‐scale tectonic forcing plays the primary role in landscape evolution. One corollary of the integrative solution of the stream‐power equation is that precipitation rate becomes influential only if it varies on time scales longer than ∼1 Ma. We conclude that calibrated inverse modeling of river profiles is a fruitful method for investigating landscape evolution and for testing source‐to‐sink models.


### `salles-2018-pybadlands`

Salles, T., Ding, Xuesong, Brocard, G. (2018). *pyBadlands: A framework to simulate sediment transport, landscape dynamics and basin stratigraphic evolution through space and time.* PLOS ONE 13(4):e0195557. DOI: [10.1371/journal.pone.0195557](https://doi.org/10.1371/journal.pone.0195557).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Understanding Earth surface responses in terms of sediment dynamics to climatic variability and tectonics forcing is hindered by limited ability of current models to simulate long-term evolution of sediment transfer and associated morphological changes. This paper presents pyBadlands, an open-source python-based framework which computes over geological time (1) sediment transport from landmasses to coasts, (2) reworking of marine sediments by longshore currents and (3) development of coral reef systems. pyBadlands is cross-platform, distributed under the GPLv3 license and available on GitHub (http://github.com/badlands-model). Here, we describe the underlying physical assumptions behind the simulated processes and the main options already available in the numerical framework. Along with the source code, a list of hands-on examples is provided that illustrates the model capabilities. In addition, pre and post-processing classes have been built and are accessible as a companion toolbox which comprises a series of workflows to efficiently build, quantify and explore simulation input and output files. While the framework has been primarily designed for research, its simplicity of use and portability makes it a great tool for teaching purposes.


### `anand-2019-linear`

Anand, S., Hooshyar, M., Porporato, A. (2019). *Linear layout of multiple flow-direction networks for landscape-evolution simulations.* Environmental Modelling & Software 133:104804. DOI: [10.1016/j.envsoft.2020.104804](https://doi.org/10.1016/j.envsoft.2020.104804).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> We present an algorithm that is well suited to find the linear layout of the multiple flow-direction network (directed acyclic graph) for an efficient implicit computation of the erosion term in landscape evolution models. The time complexity of the algorithm varies linearly with the number of nodes in the domain, making it very efficient. The resulting numerical scheme allows us to achieve accurate steady-state solutions in conditions of high erosion rates leading to heavily dissected landscapes. We also establish that contrary to single flow-direction methods such as D8, D$\infty$ multiple flow-direction method follows the theoretical prediction of the linear stability analysis and correctly captures the transition from smooth to the channelized regimes. We finally show that the obtained numerical solutions follow the theoretical temporal variation of mean elevation.


### `armitage-2019-short`

Armitage, John J. (2019). *Short communication: flow as distributed lines within the landscape.* Earth Surface Dynamics 7(1):67-75. DOI: [10.5194/esurf-7-67-2019](https://doi.org/10.5194/esurf-7-67-2019).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Landscape evolution models (LEMs) aim to capture an aggregation of the processes of erosion and deposition within the earth's surface and predict the evolving topography. Over long timescales, i.e. greater than 1 million years, the computational cost is such that numerical resolution is coarse and all small-scale properties of the transport of material cannot be captured. A key aspect, therefore, of such a long timescale LEM is the algorithm chosen to route water down the surface. I explore the consequences of two end-member assumptions of how water flows over the surface of an LEM – either down a single flow direction (SFD) or down multiple flow directions (MFDs) – on model sediment flux and valley spacing. I find that by distributing flow along the edges of the mesh cells, node to node, the resolution dependence of the evolution of an LEM is significantly reduced. Furthermore, the flow paths of water predicted by this node-to-node MFD algorithm are significantly closer to those observed in nature. This reflects the observation that river channels are not necessarily fixed in space, and a distributive flow captures the sub-grid-scale processes that create non-steady flow paths. Likewise, drainage divides are not fixed in time. By comparing results between the distributive transport-limited LEM and the stream power model "Divide And Capture", which was developed to capture the sub-grid migration of drainage divides, I find that in both cases the approximation for sub-grid-scale processes leads to resolution-independent valley spacing. I would, therefore, suggest that LEMs need to capture processes at a sub-grid-scale to accurately model the earth's surface over long timescales.


### `salles-2019-escape`

Salles, T. (2019). *eSCAPE: Regional to Global Scale Landscape Evolution Model v2.0.* Geoscientific Model Development 12(9):4165-4184. DOI: [10.5194/gmd-12-4165-2019](https://doi.org/10.5194/gmd-12-4165-2019).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. The eSCAPE model is a Python-based landscape evolution model that simulates over geological time (1) the dynamics of the landscape, (2) the transport of sediment from source to sink, and (3) continental and marine sedimentary basin formation under different climatic and tectonic conditions. The eSCAPE model is open-source, cross-platform, distributed under the GPLv3 licence, and available on GitHub (http://escape.readthedocs.io, last access: 23 September 2019). Simulated processes rely on a simplified mathematical representation of landscape processes – the stream power and creep laws – to compute Earth's surface evolution by rivers and hillslope transport. The main difference with previous models is in the underlying numerical formulation of the mathematical equations. The approach is based on a series of implicit iterative algorithms defined in matrix form to calculate both drainage area from multiple flow directions and erosion–deposition processes. The eSCAPE model relies on the PETSc parallel library to solve these matrix systems. Along with the description of the algorithms, examples are provided to illustrate the model current capabilities and limitations. It is the first landscape evolution model able to simulate processes at the global scale and is primarily designed to address problems on large unstructured grids (several million nodes).


### `yuan-2019-linking`

Yuan, Xiaoping (2019). *Linking continental erosion to marine sediment transport and deposition: A new implicit and O(N) method for inverse analysis.* Earth and Planetary Science Letters 524:115728. DOI: [10.1016/j.epsl.2019.115728](https://doi.org/10.1016/j.epsl.2019.115728).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract The marine sedimentary record contains unique information about the history of erosion, uplift and climate of the adjacent continent. Inverting this record has been the purpose of many numerical studies. However, limited attention has been given to linking continental erosion to marine sediment transport and deposition in large-scale surface process evolution models. Here we present a new numerical method for marine sediment transport and deposition that is directly coupled to a landscape evolution algorithm solving for the continental fluvial and hillslope erosion equations using implicit and O ( N ) algorithms. The new method takes into account the sorting of grain sizes (e.g., silt and sand) in the marine domain using a non-linear multiple grain-size diffusion equation and assumes that the sediment flux exported from the continental domain is proportional to the bathymetric slope. Specific transport coefficients and compaction factors are assumed for the two different grain sizes to simulate the stratigraphic architecture. The resulting set of equations is solved using an efficient ( O ( N ) and implicit) algorithm. It can thus be used to invert stratigraphic geometries using a Bayesian approach that requires a large number of simulations. This new method is used to invert the sedimentary geometry of a natural example, the Ogooue Delta (Gabon), over the last ∼5 Myr. The objective is to unravel the set of erosional histories of the adjacent continental domain compatible with the observed geometry of the offshore delta. For this, we use a Bayesian inversion scheme in which the misfit function is constructed by comparing four geometrical parameters between the natural and the simulated delta: the volume of sediments stored in the delta, the surface slope, the initial and the final shelf lengths. We find that the best-fit values of the transport coefficients for silt in the marine domain are in the range of 300 − 500 m2/yr, in agreement with previous studies on offshore diffusion. We also show that, in order to fit the sedimentary geometry, erosion rate on the continental domain must have increased by a factor of 6 to 8 since 5.3 Ma.


### `yuan-2019-new`

Yuan, X. P., Braun, Jean, Guerit, Laure, Rouby, Delphine, Cordonnier, Guillaume (2019). *A New Efficient Method to Solve the Stream Power Law Model Taking Into Account Sediment Deposition.* Journal of Geophysical Research: Earth Surface 124(6):1346--1365. DOI: [10.1029/2018JF004867](https://doi.org/10.1029/2018JF004867).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> The stream power law model has been widely used to represent erosion by rivers but does not take into account the role played by sediment in modulating erosion and deposition rates. Davy and Lague (2009, https://doi.org/10.1029/2008JF001146) provide an approach to address this issue, but it is computationally demanding because the local balance between erosion and deposition depends on sediment flux resulting from net upstream erosion. Here, we propose an efficient (i.e., O(N) and implicit) method to solve their equation. This means that, unlike other methods used to study the complete dynamics of fluvial systems (e.g., including the transition from detachment‐limited to transport‐limited behavior), our method is unconditionally stable even when large time steps are used. We demonstrate its applicability by performing a range of simulations based on a simple setup composed of an uplifting region adjacent to a stable foreland basin. As uplift and erosion progress, the mean elevations of the uplifting relief and the foreland increase, together with the average slope in the foreland. Sediments aggrade in the foreland and prograde to reach the base level where sediments are allowed to leave the system. We show how the topography of the uplifting relief and the stratigraphy of the foreland basin are controlled by the efficiency of river erosion and the efficiency of sediment transport by rivers. We observe the formation of a steady‐state geometry in the uplifting region, and a dynamic steady state (i.e., autocyclic aggradation and incision) in the foreland, with aggradation and incision thicknesses up to tens of meters.


### `bovy-2020-fastscape`

Bovy, B., Braun, J., Cordonnier, G., Lange, R., Yuan, Xiaoping (2020). *The FastScape software stack: reusable tools for landscape evolution modelling.* DOI: [10.5194/egusphere-egu2020-9474](https://doi.org/10.5194/egusphere-egu2020-9474).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> ' <div> <div> <div> <div> <p>The name "FastScape" has been used to describe a landscape evolution model as well as a set of efficient algorithms to simulate various processes of erosion, transport and deposition (e.g., fluvial, hillslope and marine). We also use this name for a set of software components (https://github.com/fastscape-lem) aimed at making those models and algorithms readily accessible to a wide range of users, from experts in landscape evolution modelling to scientists, researchers and teachers in the broader Earth science community. Those software components are organised as a stack where each level has a distinct scope. At the bottom of this stack, "fastscapelib-fortran" is the original, full-featured implementation of the FastScape model, which provides a Fortran API as well as Python bindings. Its successor "fastscapelib" is a library written in modem C++ that directly exposes the FastScape algorithms (e.g., flow-routing, depression-resolving, channel erosion, hillslope diffusion) through basic APIs in C++, Python and potentially other languages such as R or Julia in the future. Built on top of those core libraries, "fastscape" is a high-level yet flexible tool that helps anyone who wants to quickly build, extend or simply run FastScape model variants in a user-friendly, interactive environment. Through its xarray-centric interface, it is deeply integrated with the rest of the Python scientific ecosystem, therefore offering great capabilities at user's fingertips for pre/post-processing, visualisation and simulation management. One of our primary concern is following good practices (API design, testing, documentation, distribution...) while developing each of these tools. We show through a gallery of examples how the FastScape software stack has been used in research and outreach projects. We plan to provide better integration with other tools for topographic analysis/modelling (e.g., Landlab, LSDTopotools) in the future and we also greatly encourage contributions from the broader community.</p> </div> </div> </div> </div>


### `hergarten-2020-transport-limited`

Hergarten, S. (2020). *Transport-limited fluvial erosion – simple formulation and efficient numerical treatment.* DOI: [10.5194/esurf-2020-39](https://doi.org/10.5194/esurf-2020-39).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract. Most of the recent studies modeling fluvial erosion in the context of tectonic geomorphology focus on the detachment-limited regime. One reason for this simplification is the simple relationship of the constitutive law used here – often called stream-power law – to empirical results on longitudinal river profiles. Another no less important reason lies in the numerical effort that is much higher for transport-limited models than for detachment-limited models. This study proposes a formulation of transport-limited erosion where the relationship to empirical results on river profiles is almost as simple as it is for the stream-power law. As a central point, a direct solver for the fully implicit scheme is presented. This solver requires no iteration for the linear version of the model, allows for arbitrarily large time increments, and is almost as efficient as the established implicit solver for detachment-limited erosion. The numerical scheme can also be applied to linear hybrid models that cover the range between the two end-members of detachment-limited and transport-limited erosion.


### `salles-2020-gospl`

Salles, T., Mallard, C., Zahirovic, S. (2020). *gospl: Global Scalable Paleo Landscape Evolution.* Journal of Open Source Software 5(56):2804. DOI: [10.21105/joss.02804](https://doi.org/10.21105/joss.02804).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> The source-to-sink (S2S) concept quantifies the different components of sedimentary systems: from source areas, through dispersal systems, to deposition in a number of sedimentary sinks. When applied to ancient sedimentary systems, it can be used to make inferences or predictions about upstream control, or downstream evolution of paleo-landscapes and stratigraphic record (Helland-Hansen et al., 2016). Such a concept is of keen interest to Earth system scientists studying the role of atmospheric circulation on physical denudation, the influence of mantle convection on erosion and deposition patterns, the location and abundance of natural resources or the implications of morphological changes and catchments dynamics on the evolution of life.


### `barnes-2021-computing`

Barnes, Richard, Callaghan, K., Wickert, A. (2021). *Computing water flow through complex landscapes – Part 3: Fill–Spill–Merge: flow routing in depression hierarchies.* Earth Surface Dynamics 9(1):105--121. DOI: [10.5194/esurf-9-105-2021](https://doi.org/10.5194/esurf-9-105-2021).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Depressions – inwardly-draining regions – are common to many landscapes. When there is sufficient moisture, depressions take the form of lakes and wetlands; otherwise, they may be dry. Hydrological flow models used in geomorphology, hydrology, planetary science, soil and water conservation, and other fields often eliminate depressions through filling or breaching; however, this can produce unrealistic results. Models that retain depressions, on the other hand, are often undesirably expensive to run. In previous work we began to address this by developing a depression hierarchy data structure to capture the full topographic complexity of depressions in a region. Here, we extend this work by presenting a Fill-Spill-Merge algorithm that utilizes our depression hierarchy to rapidly process and distribute runoff. Runoff fills depressions, which then overflow and spill into their neighbors. If both a depression and its neighbor fill, they merge. We provide a detailed explanation of the algorithm as well as results from two sample study areas. In these case studies, the algorithm runs 90–2600× faster (with a 2000–63 000× reduction in compute time) than the commonly-used Jacobi iteration and produces a more accurate output. Complete, well-commented, open-source code is available on Github and Zenodo.


### `gailleton-2021-dynamic`

Gailleton, B., Malatesta, L., Braun, J., Cordonnier, G. (2021). *Dynamic modelling framework to track sediment provenance and solve lakes in long-term landscape evolution models.* DOI: [10.5194/egusphere-egu21-9392](https://doi.org/10.5194/egusphere-egu21-9392).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> <p>Many laws have been developed to describe the different aspects of landscape evolution at large spatial and temporal scales. Natural landscapes have heterogeneous properties (lithologies, climates, tectonics, etc.) that are associated with multiple coexisting processes. In turn, this can demand different mathematical expressions to model landscape evolution as a function of time and or space. Landscape Evolution Models are mostly designed to facilitate the combination of different landscape-wide laws in a plug-and-play way and many frameworks are being developed in this aim. However, most current frameworks cannot capture important landscape processes such as lake dynamics and full sediment tracing because they are optimized for speed and handle fluxes separately. Several processes require information from more than the immediate neighboring cells within a time step and demand an integrated knowledge from the entire upstream trajectory. Lakes for example require knowledge of all upstream water and sediment fluxes to be filled. These can only be known if all the laws controlling those have been processed. Tackling these situation with a grid logic requires substantial amount of numerical refactoring from existing models.</p><p>We present an alternative method to tackle landscape evolution modelling in heterogeneous landscapes with a framework inspired from Lagrangian and cellular automaton methods. Our framework only relies on the assumption that upstream nodes needs to be processed before the downstream ones, including lakes with outlets, in order to process all selected governing equations on a pixel-to-pixel basis. This way, we ensure that the true content of sediment and water fluxes can be known and tracked at any points. We first utilise graph theory to (i) find the most comprehensive path to reroute water through depressions and (ii) determine a generic multiple flow topological order (any node is processed after all potential upstream ones). Particles that register and track all fluxes simultaneously can then "roll" on the landscape and merge between each other while interacting with the grid.</p><p>This formulation makes possible a number of generic features. (i) The laws can be dynamically adapted to the environment (e.g. switching from single to multiple flow function of water content, adapting erodibility function of the sediment composition and quantity), (ii) Depressions can be explicitly managed, filled (or not) and separated from the rest of the landscape (e.g. sedimentation or evaporation in lakes) as a function function of inputted fluxes and parameters, (iii) full provenance, transport time, and deposition tracking as the particle can always keep in memory where the fluxes are from and in what proportions. In this contribution, we demonstrate the impact the importance of considering these additional elements in landscape evolution. In particular, lake dynamic can significantly impact the long-term signal propagation from source to sink.</p>


### `gallen-2021-data`

Gallen, S., Fernández‐Blanco, D. (2021). *A New Data‐Driven Bayesian Inversion of Fluvial Topography Clarifies the Tectonic History of the Corinth Rift and Reveals a Channel Steepness Threshold.* Journal of Geophysical Research: Earth Surface 126(3). DOI: [10.1029/2020jf005651](https://doi.org/10.1029/2020jf005651).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Landscape evolution models that invert topography for rock uplift can improve our understanding of both tectonic and geomorphic processes when properly constrained with data. Here we present a flexible, data‐driven Bayesian approach to invert fluvial topography for tectonic and geomorphic model parameters and apply it to a case study, the uplifting footwall of the Corinth Rift, Greece. We invert transient river profiles and up‐flexed marine terraces to resolve seven unknown parameters in a regional‐to‐flexural uplift tectonic model and the stream power incision model. The best‐fit tectonic parameters are consistent with independent data and predict block uplift rates of ∼0.1 mm yr−1 that changed to flexural uplift rates of ∼1.6 mm yr−1 at ∼0.6 Ma, as the master normal fault initiated. Similarly, the best‐fit geomorphic parameters predict sediment flux consistent with the offshore record and erodibility consistent with previous studies. However, the drainage area exponent, m , of ∼2, and slope exponent, n , of ∼7, are unusually high, indicating a threshold channel steepness where fluvial topography is largely insensitive to rock uplift rate >0.05 mm yr−1. Analysis indicates channels narrow to accommodate enhanced uplift rates, but channel narrowing only partially explains our results, suggesting that other processes not accounted for in the generic stream power model are also relevant to bedrock river incision in Corinth. Our results help clarify the tectonic and geomorphic evolution of the Corinth Rift, have important implications for studies that invert topography for rock uplift histories, and provide insight into potential limitations of some long‐term river incision models.


### `acevedotrejos-2023-adascape`

Acevedo‐Trejos, E., Braun, J., Kravitz, Katherine, Raharinirina, N. A., Bovy, B. (2023). *AdaScape 1.0: a coupled modelling tool to investigate the links between tectonics, climate, and biodiversity.* Geoscientific Model Development 16(23):6921-6941. DOI: [10.5194/gmd-16-6921-2023](https://doi.org/10.5194/gmd-16-6921-2023).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. The interplay between tectonics and climate is known to impact the evolution and distribution of life forms, leading to present-day patterns of biodiversity. Numerical models that integrate the co-evolution of life and landforms are ideal tools to investigate the causal links between these earth system components. Here, we present a tool that couples an ecological–evolutionary model with a landscape evolution model (LEM). The former is based on the adaptive speciation of functional traits, where these traits can mediate ecological competition for resources, and includes dispersal and mutation processes. The latter is a computationally efficient LEM (FastScape) that predicts topographic relief based on the stream power law, hillslope diffusion, and orographic precipitation equations. We integrate these two models to illustrate the coupled behaviour between tectonic uplift and eco-evolutionary processes. Particularly, we investigate how changes in tectonic uplift rate and eco-evolutionary parameters (i.e. competition, dispersal, and mutation) influence speciation and thus the temporal and spatial patterns of biodiversity.


### `braun-2023-implicit`

Braun, J., Deal, E. (2023). *Implicit Algorithm for Threshold Stream Power Incision Model.* Journal of Geophysical Research: Earth Surface 128(10). DOI: [10.1029/2023jf007140](https://doi.org/10.1029/2023jf007140).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We present an O(n) complexity and implicit algorithm for the two‐dimensional solution of the stream power incision model (SPIM) enriched by a discharge threshold term and taking into account variability in rainfall and thus discharge. The algorithm is based on the formulation developed by Deal et al. (2018, https://doi.org/10.1002/2017jf004393) and the generalization of the FastScape algorithm (Braun & Willett, 2013, https://doi.org/10.1016/j.geomorph.2012.10.008) where the slope is approximated by first‐order accurate finite difference. We consider a variety of discharge thresholds that vary in their dependence on channel slope. The algorithm requires finding the root of a non‐linear equation using a Newton‐Raphson iterative scheme. We show that the convergence of this scheme is unconditional, except for a narrow range of model parameters where the threshold increases with the slope and for low discharge variability. We also show that the rate of convergence of the iterative scheme is directly proportional to the slope exponent n in the SPIM. We compare the algorithm to analytical solutions and to numerical solutions obtained using a higher‐order finite difference scheme. We show that the accuracy of the FastScape algorithm and its generalization presented here is comparable to other schemes for values of n > 1. We also confirm that the FastScape algorithm and its generalization to variable discharge + threshold conditions does not need to satisfy the CFL condition and provides an accurate solution for both small and very long time steps. We finally use the new algorithm to quantify how the existence of an erosional threshold strongly affects the length of the post‐orogenic decay of mountain belts.


### `coatlven-2023-large`

Coatléven, J., Chauveau, B. (2023). *Large structures simulation for landscape evolution models.* <https://www.semanticscholar.org/paper/d4799fe03cf3001953bcf338a98fabdd6605b023>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Because of the chaotic behavior of the coupling between water flow and sediment erosion and transport, without any special treatment the practical results of landscape evolution models (LEM) are likely to be dominated by numerical errors. This paper describes two areas of improvement that we believe are necessary for the successful simulation of landscape evolution models. The first one concerns the expression of the water flux that was initially rebuilt in Coatléven (2020) in a mathematically consistent way for the cell-to-cell multiple flow direction algorithms, thanks to a reinterpretation as a well 5 chosen discretization of the Gauckler-Manning-Strickler continuous equation. Building on those results, we introduce here a general framework allowing to derive consistent expressions of the water flux for the most commonly used multiple/single flow direction (MFD/SFD) water flow routines, including node-to-node versions. If having a consistent water flux is crucial to avoid any mesh size dependence in a LEM and controlling the consistency error, the expected non-linear self amplification mechanisms of the water and sediment coupling can still lead to simulations blurred by numerical errors. Those numerical instabilities 10 being highly reminiscent of turbulence induced instabilities in computational fluid dynamics (CFD), in the second part of our paper we present a "large structure simulation" (LSS) approach for LEM, mimicking the large-eddy simulations (LES) used for turbulent CFD. The LSS allows to control numerical errors while preserving the major physical based geomorphic patterns.


### `gatti-2023-fully`

Gatti, F., Bonaventura, Luca, Menafoglio, A., Papini, M., Longoni, L. (2023). *A fully coupled superficial runoff and soil erosion basin scale model with efficient time stepping.* Computers & Geosciences 177:105362. DOI: [10.1016/j.cageo.2023.105362](https://doi.org/10.1016/j.cageo.2023.105362).

**Availability:** PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> We present a numerical model of soil erosion at the basin scale that allows one to describe surface runoff without a priori identifying drainage zones, river beds and other water bodies. The model is based on robust semi-implicit numerical techniques and guarantees exact mass conservation and positivity of the surface and subsurface water layers. Furthermore, the method is equipped with a geostatistical preprocessor that can perform downscaling of data retrieved from digital databases at coarser resolutions. Numerical experiments on both idealized and realistic configurations demonstrate the effectiveness of the proposed method in reproducing transient high resolution features at a reduced computational cost and to reproduce correctly the main hydrographic features of the considered catchment. Furthermore, probabilistic forecasts can be carried out based on soil data maps automatically generated by the geostatistical preprocessor that are distributed among the available MPI ranks to carry out simulations independently thus reducing the total cost of the simulation. The numerical experiments show the ability of the model to provide robust estimates of water levels, discharge and of the order of magnitude of the total sediment yield.


### `hergarten-2023-self-organization`

Hergarten, S., Pietrek, A. (2023). *Self-organization of channels and hillslopes in models of fluvial landform evolution and its potential for solving scaling issues.* Earth Surface Dynamics 11(4):741-755. DOI: [10.5194/esurf-11-741-2023](https://doi.org/10.5194/esurf-11-741-2023).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Including hillslope processes in models of fluvial landform evolution is still challenging. Since applying the respective models for fluvial and hillslope processes to the entire domain causes scaling problems and makes the results dependent on the spatial resolution, the domain is explicitly subdivided into channels and hillslopes in some models. The transition from hillslopes to channels is typically attributed to a given threshold catchment size as a proxy for a minimum required discharge. Here we propose a complementary approach for delineating channels based on the discrete representation of the topography. We assume that sites with only one lower neighbor are channelized. In combination with a suitable model for hillslope processes, this concept initiates the self-organization of channels and hillslopes. A numerical analysis with a simple model for hillslope dynamics reveals no scaling issues, so the results appear to be independent of the spatial resolution. The approach predicts a break in slope in the sense that all channels are distinctly less steep than hillslopes. On a regular lattice, the simple D8 flow-routing scheme (steepest descent among the eight nearest and diagonal neighbors) harmonizes well with the concept proposed here. The D8 scheme works well even when applied to the hillslopes. This property simplifies the numerical implementation and increases its efficiency.


### `morris-2023-towards`

Morris, M. J., Lipp, A., Roberts, G. (2023). *Towards Inverse Modeling of Landscapes Using the Wasserstein Distance.* Geophysical Research Letters 50(14). DOI: [10.1029/2023gl103880](https://doi.org/10.1029/2023gl103880).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Extricating histories of uplift and erosion from landscapes is crucial for many branches of the Earth sciences. An objective way to calculate such histories is to identify calibrated models that minimize misfit between observations (e.g., topography) and predictions (e.g., synthetic landscapes). In the presence of natural or computational noise, widely used Euclidean measures of similarity can have complicated objective functions, obscuring the search for optimal models. Instead, we introduce the Wasserstein distance as a means to measure misfit between observed and theoretical landscapes. Our results come in two parts. First, we show that this approach can generate much smoother objective functions than Euclidean measures, simplifying the search for optimal models. Second, we show how locations and amplitudes of uplift can be accurately recovered from synthetic landscapes even when seeded with different noisy initial conditions. We suggest that this approach holds promise for inverting real landscapes for their histories.


### `salles-2023-hundred`

Salles, T. (2023). *Hundred million years of landscape dynamics from catchment to global scale.* Science 379(6635):918--923. DOI: [10.1126/science.add2541](https://doi.org/10.1126/science.add2541).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Our capability to reconstruct past landscapes and the processes that shape them underpins our understanding of paleo-Earth. We take advantage of a global-scale landscape evolution model assimilating paleoelevation and paleoclimate reconstructions over the past 100 million years. This model provides continuous quantifications of metrics critical to the understanding of the Earth system, from global physiography to sediment flux and stratigraphic architectures. We reappraise the role played by surface processes in controlling sediment delivery to the oceans and find stable sedimentation rates throughout the Cenozoic with distinct phases of sediment transfer from terrestrial to marine basins. Our simulation provides a tool for identifying inconsistencies in previous interpretations of the geological record as preserved in sedimentary strata, and in available paleoelevation and paleoclimatic reconstructions. Description Shaping Earth's surface How has sediment transfer and accumulation from lands to oceans affected Earth's geomorphology? Salles et al. present results from a high-resolution model of surface physiography, validated by independent observations from the geological record, which simulates landscape evolution caused by erosion and deposition over the past 100 million years (see the Perspective by Ehlers). Their results should help us better understand the apparent contradiction between the observed Late Cenozoic pulse in marine sedimentation and the constancy of the global weathering flux. —HJS A 100-million-year simulation of Earth's surface evolution clarifies the role of sediment transfer and accumulation on a global scale.


### `salles-2023-landscape`

Salles, T., Husson, L., Lorcery, Manon, Boggiani, Beatriz Hadler (2023). *Landscape dynamics and the Phanerozoic diversification of the biosphere.* Nature 624(7990):115--121. DOI: [10.1038/s41586-023-06777-z](https://doi.org/10.1038/s41586-023-06777-z).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> A model of sediment flux from the land to the oceans over the Phanerozoic eon explains differences in the fossil records of marine animal genera and land plant genera. The long-term diversification of the biosphere responds to changes in the physical environment. Yet, over the continents, the nearly monotonic expansion of life started later in the early part of the Phanerozoic eon^ 1 than the expansion in the marine realm, where instead the number of genera waxed and waned over time^ 2 . A comprehensive evaluation of the changes in the geodynamic and climatic forcing fails to provide a unified theory for the long-term pattern of evolution of life on Earth. Here we couple climate and plate tectonics models to numerically reconstruct the evolution of the Earth's landscape over the entire Phanerozoic eon, which we then compare to palaeo-diversity datasets from marine animal and land plant genera. Our results indicate that biodiversity is strongly reliant on landscape dynamics, which at all times determine the carrying capacity of both the continental domain and the oceanic domain. In the oceans, diversity closely adjusted to the riverine sedimentary flux that provides nutrients for primary production. On land, plant expansion was hampered by poor edaphic conditions until widespread endorheic basins resurfaced continents with a sedimentary cover that facilitated the development of soil-dependent rooted flora, and the increasing variety of the landscape additionally promoted their development.


### `skinner-2023-testing`

Skinner, Christopher J., Coulthard, Thomas J. (2023). *Testing the sensitivity of the CAESAR-Lisflood landscape evolution model to grid cell size.* Earth Surface Dynamics 11(4):695-711. DOI: [10.5194/esurf-11-695-2023](https://doi.org/10.5194/esurf-11-695-2023).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Landscape evolution models (LEMs) are useful for understanding how large-scale processes and perturbations influence the development of the surface of the Earth and other planets. With their increasing sophistication and improvements in computational power, they are finding greater uptake in analyses at finer spatial and temporal scales. For many LEMs, the land surface is represented by a grid of regularly spaced and sized grid cells, or pixels, referred to as a digital elevation model (DEM), yet despite the importance of the DEM to LEM studies, there has been little work to understand the influence of grid cell size (i.e. resolution) on model behaviour. This is despite the choice of grid cell size being arbitrary for many studies, with users needing to balance detail with computational efficiency. Using the Morris method (MM) for global sensitivity analysis, the sensitivity of the CAESAR-Lisflood LEM to the grid cell size is evaluated relative to a set of influential user-defined parameters, showing that it had a similar level of influence as a key hydrological parameter and the choice of sediment transport law. Outputs relating to discharge and sediment yields remained stable across different grid cell sizes until the cells became so large that the representation of the hydrological network degraded. Although total sediment yields remained steady when changing the grid cell sizes, closer analysis revealed that using a coarser grid resulted in it being built up from fewer yet more geomorphically active events, risking outputs that are "the right answer but for the wrong reasons". These results are important considerations for modellers using LEMs and the methodologies detailed provide solutions to understanding the impacts of modelling choices on outputs.


### `bernard-2024-estimation`

Bernard, Thomas (2024). *Estimation of Denudation Parameters and River Capture Events From Neural Network Inverse Modeling of River Profiles and Thermo‐ and Geochronology Data.* Journal of Geophysical Research: Earth Surface 129(10). DOI: [10.1029/2024jf007636](https://doi.org/10.1029/2024jf007636).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Earth's topography represents the cumulative effects of tectonics and surface processes modulated by climate and lithology. These factors shape landscapes through time. River profiles can be inverted to estimate the rock uplift histories or lithology‐specific erodibilities. However, river systems are dynamic and evolve in response to spatial and temporal internal dynamics, such as river capture events. Here, we present a modeling framework to infer denudation rates from the inversion of river profiles and thermo‐ and geochronology data. We achieve this by coupling a landscape evolution model and an efficient inverse modeling scheme to infer poorly resolved erosional and tectonic parameters. An application of the approach is presented for the Neckar catchment, southwest Germany, characterized by stark lateral variation in bedrock erodibility and rock uplift, and that have demonstrably undergone multiple river capture events. Different end‐member scenarios are explored in the simulations. First, we test uniform and spatial variability in rock uplift rate and bedrock erodibility, and second, temporal variations in rock uplift rate and base level. Finally, we simulate river capture events by adding upstream sections (drainage area) at specific times and locations within the fluvial network. We find that spatial variation in rock uplift rate is necessary to reproduce the Neckar's river profile while honoring analytical observations. Simulations integrating river captures allow improved river profile predictions of specific tributaries of the Neckar catchment, leading to potentially more realistic erodibility and rock uplift history estimates. The time and location of the capture events determined from the modeling agree with previous estimations from geological evidence.


### `coatleven-2024-large`

Coatl\'even, Julien, Chauveau, Beno\^it (2024). *Large structure simulation for landscape evolution models.* Earth Surface Dynamics 12:995--1026. DOI: [10.5194/esurf-12-995-2024](https://doi.org/10.5194/esurf-12-995-2024).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> The aim of this paper is to discuss the efficiency of a new methodology to maintain the accuracy of numerical solutions obtained from our landscape evolution model (LEM). As in every LEM, the tricky part is the coupling between water and sediment flows that drives the nonlinear self-amplification mechanisms. But this coupling is also responsible for the emergence and amplification of numerical errors, as we illustrate here. These numerical instabilities being strongly reminiscent of turbulence-induced instabilities in computational fluid dynamics (CFD), we introduce a "large structure simulation" (LSS) approach for LEM, mimicking the large eddy simulation (LESs) used for turbulent CFD. In practice, this treatment consists in a filtering strategy that controls small-scale perturbations in the solution. We demonstrate the accuracy of the LSS approach in the context of our LEM.


### `gailleton-2024-chonk`

Gailleton, B., Malatesta, L., Cordonnier, G., Braun, J. (2024). *CHONK 1.0: landscape evolution framework: cellular automata meets graph theory.* Geoscientific Model Development 17(1):71-90. DOI: [10.5194/gmd-17-71-2024](https://doi.org/10.5194/gmd-17-71-2024).

**Availability:** **in-repo PDF** — `ref/research/pdfs/gmd-17-71-2024.pdf` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> Landscape evolution models (LEMs) are prime tools for simulating the evolution of source-to-sink systems through ranges of spatial and temporal scales. A plethora of various empirical laws have been successfully applied to describe the different parts of these systems: fluvial erosion, sediment transport and deposition, hillslope diffusion, or hydrology. Numerical frameworks exist to facilitate the combination of different subsets of laws, mostly by superposing grids of fluxes calculated independently. However, the exercise becomes increasingly challenging when the different laws are inter-connected: for example when a lake breaks the upstream–downstream continuum in the amount of sediment and water it receives and transmits; or when erosional efficiency depends on the lithological composition of the sediment flux. In this contribution, we present a method mixing the advantages of cellular automata and graph theory to address such cases. We demonstrate how the former ensure interoperability of the different fluxes (e.g. water, fluvial sediments, hillslope sediments) independently of the process law implemented in the model, while the latter offers a wide range of tools to process numerical landscapes, including landscapes with closed basins. We provide three scenarios largely benefiting from our method: (i) one where lake systems are primary controls on landscape evolution, (ii) one where sediment provenance is closely monitored through the stratigraphy and (iii) one where heterogeneous provenance influences fluvial incision dynamically. We finally outline the way forward to make this method more generic and flexible.


### `gailleton-2024-graphflood`

Gailleton, B., Steer, P., Davy, P., Schwanghart, W., Bernard, T. (2024). *GraphFlood 1.0: an efficient algorithm to approximate 2D hydrodynamics for landscape evolution models.* Earth Surface Dynamics 12(6):1295-1313. DOI: [10.5194/esurf-12-1295-2024](https://doi.org/10.5194/esurf-12-1295-2024).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Computing hydrological fluxes at the Earth's surface is crucial for landscape evolution models, topographic analysis, and geographic information systems. However, existing formalisms, like single or multiple flow algorithms, often rely on ad hoc rules based on local topographic slope and drainage area, neglecting the physics of water flow. While more physics-oriented solutions offer accuracy (e.g. shallow-water equations), their computational costs limit their use in terms of spatial and temporal scales. In this contribution, we introduce GraphFlood, a novel and efficient iterative method for computing river depth and water discharge in 2D with a digital elevation model (DEM). Leveraging the directed acyclic graph structure of surface water flow, GraphFlood iteratively solves the 2D shallow-water equations. This algorithm aims to find the correct hydraulic surface by balancing discharge input and output over the topography. At each iteration, we employ fast-graph-theory algorithms to calculate flow accumulation on the hydraulic surface, approximating discharge input. Discharge output is then computed using the Manning flow resistance equation, similar to the River.lab model (Davy and Lague, 2009). The divergence of discharges iteratively increments flow depth until reaching a stationary state. This algorithm can also solve for flood wave propagation by approximating the input discharge function of the immediate upstream neighbours. We validate water depths obtained with the stationary solution against analytical solutions for rectangular channels and the River.lab and CAESAR-Lisflood models for natural DEMs. GraphFlood demonstrates significant computational advantages over previous hydrodynamic models, an with approximately 10-fold speed-up compared to the River.lab model (Davy and Lague, 2009). Additionally, its computational time scales slightly more than linearly with the number of cells, making it suitable for large DEMs exceeding 106–108 cells. We demonstrate the versatility of GraphFlood by integrating realistic hydrology into various topographic and morphometric analyses, including channel width measurement, inundation pattern delineation, floodplain delineation, and the classification of hillslope, colluvial, and fluvial domains. Furthermore, we discuss its integration potential in landscape evolution models, highlighting its simplicity of implementation and computational efficiency.


### `ghelichkhan-2024-automatic`

Ghelichkhan, S., Gibson, A., Davies, D., Kramer, Stephan C., Ham, D. A. (2024). *Automatic adjoint-based inversion schemes for geodynamics: reconstructing the evolution of Earth's mantle in space and time.* Geoscientific Model Development 17(13):5057-5086. DOI: [10.5194/gmd-17-5057-2024](https://doi.org/10.5194/gmd-17-5057-2024).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Reconstructing the thermo-chemical evolution of Earth's mantle and its diverse surface manifestations is a widely recognised grand challenge for the geosciences. It requires the creation of a digital twin: a digital representation of Earth's mantle across space and time that is compatible with available observational constraints on the mantle's structure, dynamics and evolution. This has led geodynamicists to explore adjoint-based approaches that reformulate mantle convection modelling as an inverse problem, in which unknown model parameters can be optimised to fit available observational data. Whilst there has been a notable increase in the use of adjoint-based methods in geodynamics, the theoretical and practical challenges of deriving, implementing and validating adjoint systems for large-scale, non-linear, time-dependent problems, such as global mantle flow, has hindered their broader use. Here, we present the Geoscientific ADjoint Optimisation PlaTform (G-ADOPT), an advanced computational modelling framework that overcomes these challenges for coupled, non-linear, time-dependent systems by integrating three main components: (i) Firedrake, an automated system for the solution of partial differential equations using the finite-element method; (ii) Dolfin-Adjoint, which automatically generates discrete adjoint models in a form compatible with Firedrake; and (iii) the Rapid Optimisation Library, ROL, an efficient large-scale optimisation toolkit; G-ADOPT enables the application of adjoint methods across geophysical continua, showcased herein for geodynamics. Through two sets of synthetic experiments, we demonstrate the application of this framework to the initial condition problem of mantle convection, in both square and annular geometries, for both isoviscous and non-linear rheologies. We confirm the validity of the gradient computations underpinning the adjoint approach, for all cases, through second-order Taylor remainder convergence tests and subsequently demonstrate excellent recovery of the unknown initial conditions. Moreover, we show that the framework achieves theoretical computational efficiency. Taken together, this confirms the suitability of G-ADOPT for reconstructing the evolution of Earth's mantle in space and time. The framework overcomes the significant theoretical and practical challenges of generating adjoint models and will allow the community to move from idealised forward models to data-driven simulations that rigorously account for observational constraints and their uncertainties using an inverse approach.


### `guryan-2024-sediment`

Guryan, G. J., Johnson, J. P. L., Gasparini, N. (2024). *Sediment Cover Modulates Landscape Erosion Patterns and Channel Steepness in Layered Rocks: Insights From the SPACE Model.* Journal of Geophysical Research: Earth Surface 129(7). DOI: [10.1029/2023jf007509](https://doi.org/10.1029/2023jf007509).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Erosional perturbations from changes in climate or tectonics are recorded in the profiles of bedrock rivers, but these signals can be challenging to unravel in settings with non‐uniform lithology. In layered rocks, the surface lithology at a given location varies through time as erosion exposes different layers of rock. Recent modeling studies have used the Stream Power Model (SPM) to highlight complex variations in erosion rates that arise in bedrock rivers incising through layered rocks. However, these studies do not capture the effects of coarse sediment cover on channel evolution. We use the "Stream Power with Alluvium Conservation and Entrainment" (SPACE) model to explore how sediment cover influences landscape evolution and modulates the topographic expression of erodibility contrasts in horizontally layered rocks. We simulate river evolution through alternating layers of hard and soft rock over million‐year timescales with a constant and uniform uplift rate. Compared to the SPM, model runs with sediment cover have systematically higher channel steepness values in soft rock layers and lower channel steepness values in hard rock layers. As more sediment accumulates, the contrast in steepness between the two rock types decreases. Effective bedrock erodibilities back‐calculated assuming the SPM are strongly influenced by sediment cover. We also find that sediment cover can significantly increase total relief and timescales of adjustment toward landscape‐averaged steady‐state topography and erosion rates.


### `jain-2024-fastflow`

Jain, Aryamaan, Kerbl, Bernhard, Gain, James, Finley, Brandon, Cordonnier, G. (2024). *FastFlow: GPU Acceleration of Flow and Depression Routing for Landscape Simulation.* Computer Graphics Forum 43(7). DOI: [10.1111/cgf.15243](https://doi.org/10.1111/cgf.15243).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Terrain analysis plays an important role in computer graphics, hydrology and geomorphology. In particular, analyzing the path of material flow over a terrain with consideration of local depressions is a precursor to many further tasks in erosion, river formation, and plant ecosystem simulation. For example, fluvial erosion simulation used in terrain modeling computes water discharge to repeatedly locate erosion channels for soil removal and transport. Despite its significance, traditional methods face performance constraints, limiting their broader applicability.


### `keck-2024-landslide`

Keck, Jeffrey, Istanbulluoglu, E., Campforts, B., Tucker, G., Horner-Devine, Alexander (2024). *A landslide runout model for sediment transport, landscape evolution, and hazard assessment applications.* Earth Surface Dynamics 12(5):1165-1191. DOI: [10.5194/esurf-12-1165-2024](https://doi.org/10.5194/esurf-12-1165-2024).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract. We developed a new rule-based, cellular-automaton algorithm for predicting the hazard extent, sediment transport, and topographic change associated with the runout of a landslide. This algorithm, which we call MassWastingRunout (MWR), is coded in Python and implemented as a component for the package Landlab. MWR combines the functionality of simple runout algorithms used in landscape evolution and watershed sediment yield models with the predictive detail typical of runout models used for landslide inundation hazard mapping. An initial digital elevation model (DEM), a regolith depth map, and the location polygon of the landslide source area are the only inputs required to run MWR to model the entire runout process. Runout relies on the principle of mass conservation and a set of topographic rules and empirical formulas that govern erosion and deposition. For the purpose of facilitating rapid calibration to a site, MWR includes a calibration utility that uses an adaptive Bayesian Markov chain Monte Carlo algorithm to automatically calibrate the model to match observed runout extent, deposition, and erosion. Additionally, the calibration utility produces empirical probability density functions of each calibration parameter that can be used to inform probabilistic implementation of MWR. Here we use a series of synthetic terrains to demonstrate basic model response to topographic convergence and slope, test calibrated model performance relative to several observed landslides, and briefly demonstrate how MWR can be used to develop a probabilistic runout hazard map. A calibrated runout model may allow for region-specific and more insightful predictions of landslide impact on landscape morphology and watershed-scale sediment dynamics and should be further investigated in future modeling studies.


### `roberts-2024-theory`

Roberts, G., Wani, O. (2024). *A theory of stochastic fluvial landscape evolution.* Proceedings of the Royal Society A: Mathematical, Physical and Engineering Sciences 480(2283). DOI: [10.1098/rspa.2023.0456](https://doi.org/10.1098/rspa.2023.0456).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Geometries of eroding landscapes contain important information about geologic, climatic, biotic and geomorphic processes. They are also characterized by variability, which makes disentangling their origins challenging. Observations and physical models of fluvial processes, which set the pace of erosion on most continents, emphasize complexity and variability. By contrast, the spectral content of longitudinal river profiles and similarity of geometries at scales greater than approximately 100 km highlight relatively simple emergent properties. A general challenge then, addressed in this manuscript, is development of a theory of landscape evolution that embraces such scale-dependent insights. We do so by incorporating randomness and probability into a theory of fluvial erosion. First, we explore the use of stochastic differential equations of the Langevin type, and the Fokker–Planck equation, for predicting migration of erosional fronts. Second, analytical approaches incorporating distributions of driving forces, critical thresholds and associated proxies are developed. Finally, a linear programming approach is introduced, that, at its core, treats evolution of longitudinal profiles as a Markovian stochastic problem. The theory is developed essentially from first principles and incorporates physics governing fluvial erosion. We explore predictions of this theory, including the natural growth of discontinuities and scale-dependent evolution, including local complexity and emergent simplicity.


### `yuan-2024-coordination`

Yuan, Xiaoping (2024). *Coordination between deformation, precipitation, and erosion during orogenic growth.* Nature Communications 15(1). DOI: [10.1038/s41467-024-54690-4](https://doi.org/10.1038/s41467-024-54690-4).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Crustal thickening associated with orogenic growth elevates topography, causing orographic enhancement of precipitation, which in turn facilitates local erosion and possibly intensifies localization of deformation. How these three processes—deformation, precipitation, and erosion—coordinate during orogenic growth remains unknown. Here, we present a numerical model where tectonics, surface processes, and orographic precipitation are tightly coupled, and explore the impact on low, intermediate, and high erodibility orogens. We show that, for intermediate erodibility models, rock uplift rates and precipitation rates correlate well with erosion rates during the formation of orogenic plateaus with high correlation coefficients of ~0.9 between rock uplift and erosion rates, and ~0.8 between precipitation and erosion rates. We demonstrate a cyclicity of correlation evolution among uplift, precipitation, and erosion rates through the development of new faults propagating outward. These results shed insights into the relative tectonic or climatic control on erosion in active orogens (e.g., Himalayas, Central Andes, and Southern Alps of New Zealand), and provide a plausible explanation for several conflicting data and interpretations in the Himalayas, which depend on the stage of maturity of the newest fault and the relative locations to old faults. This study presents a model showing how tectonics, climate, and erosion interact during orogen growth. It reveals a cyclic correlation between uplift, precipitation, and erosion, offering insights into exogenic controlled orogens like the Himalayas.


### `coatleven-2025-postprocessing`

Coatl\'even, Julien, Chauveau, Beno\^it (2025). *A post-processing solution to restore numerical consistency for classical flow routing algorithms.* Computational Geosciences. DOI: [10.1007/s10596-025-10359-5](https://doi.org/10.1007/s10596-025-10359-5).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/CorrectedMFD.pdf`, `ref/research/pdfs/markdowns/`

**Abstract** *(source: extracted from the PDF we hold)*

> In a recent paper, a consistency correction for the water flux using multiple flow direction (MFD) algorithms that account for exchanges between a cell and its neighbors was proposed, thanks to a reinterpretation of the MFD as a well chosen discretization of the Gauckler-Manning-Strickler continuous equation. Building on those results, we introduce here a general framework allowing to derive consistent expressions of the water flux for the most commonly used multiple/single flow direction (MFD/SFD) water flow routines, including versions in which water is flowing from a node to its neighbors. This general framework is shown to be sufficiently general to encompass the alternative continuous definition of the unit catchment area of the literature. Numerical examples illustrate the consistency and convergence of the proposed water flux reconstructions.


### `curry-2025-exploring`

Curry, M., Beek, P. A. van der (2025). *Exploring Controls on Post‐Orogenic Topographic Stasis of the Pyrenees Mountains With Inverse Landscape Evolution Modeling.* Journal of Geophysical Research: Earth Surface 130(2). DOI: [10.1029/2024jf007759](https://doi.org/10.1029/2024jf007759).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> How high topography can be sustained over long timescales in post‐orogenic mountain belts is a longstanding research question in tectonic geomorphology and geodynamics. Here we utilize the well‐documented orogenic paleo‐topography and spatial‐temporal exhumation patterns of the Pyrenee Mountains in a numerical modeling study investigating controls on post‐orogenic topographic stasis. Orogenic activity in the Pyrenees Mountains ceased at ca. 25‐20 Ma, but topographic decay has only been on the scale of hundreds of meters since that time. We use the landscape‐evolution model FastScape coupled with the neighborhood‐algorithm inversion method to explore the influence of precipitation, lithology, and stream power parameters on post‐orogenic topographic stability. The inversions are constrained using topography (elevation, slope) and low‐temperature thermochronology data. We find that incorporation of an erodibility threshold is required for moderating post‐orogenic topographic decay, without which post‐orogenic topography declines significantly on Myr timescales. While other evaluated parameters such as lithology and precipitation also contribute to topographic stability, they are secondary to the erodibility threshold in maintaining long‐term post‐orogenic topography. Our results provide valuable insights into the mechanisms governing post‐orogenic landscape evolution and emphasize the importance of thresholds in landscape evolution modeling of mountain belts.


### `du-2025-jax-mpm`

Du, Honghui, He, QiZhi (2025). *JAX-MPM: a learning-augmented differentiable meshfree framework for GPU-accelerated Lagrangian simulation and geophysical inverse modeling.* Engineering with Computers 42(3). DOI: [10.1007/s00366-026-02320-6](https://doi.org/10.1007/s00366-026-02320-6).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Differentiable programming has emerged as a powerful paradigm in scientific computing, enabling automatic differentiation through simulation pipelines and naturally supporting both forward and inverse modeling. We present JAX-MPM, a general-purpose differentiable meshfree solver based on the material point method (MPM) and implemented in the modern JAX ecosystem. The framework adopts a hybrid Eulerian–Lagrangian formulation to capture large deformations, frictional contact, and inelastic material behavior, with emphasis on geomechanics and geophysical hazard applications. Leveraging GPU acceleration and automatic differentiation, JAX-MPM enables efficient gradient-based optimization directly through its time-stepping solvers and supports joint training of physical models with deep learning to infer unknown system conditions and uncover hidden constitutive parameters. We validate JAX-MPM through a series of 2D and 3D benchmark simulations, including dam-break and granular collapse problems, demonstrating both numerical accuracy and GPU-accelerated performance. Results show that a high-resolution 3D granular cylinder collapse with 2.7 million particles completes 1000 time steps in approximately 22 s (single precision) and 98 s (double precision) on a single GPU. Beyond high-fidelity forward modeling, we demonstrate the framework's inverse modeling capabilities through tasks such as velocity field reconstruction and the estimation of spatially varying friction from sparse data. In particular, JAX-MPM introduces a differentiable observation layer that unifies data assimilation from both Lagrangian (particle-based) and Eulerian (region-based) observations, and can be seamlessly coupled with neural network representations. These results establish JAX-MPM as a unified and scalable differentiable meshfree platform that advances fast physical simulation and data assimilation for complex solid and geophysical systems.


### `li-2025-hillslope`

Li, Tingan, Sklar, L., Gasparini, N. (2025). *Hillslope grain size variation across evolving landscapes linked to climate, tectonics and lithology.* Earth Surface Processes and Landforms 50(8). DOI: [10.1002/esp.70111](https://doi.org/10.1002/esp.70111).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> The grain size of bedload sediment regulates rates of river incision into bedrock and thus influences topographic response to temporal and spatial variations in climate, tectonics and lithology. Grain size in river networks, in turn, depends on the size distributions of rock particles produced by weathering on hillslopes, which vary with local climate, erosion rate and rock properties. Hence, understanding the evolution of erosional landscapes requires consideration of the role of grain size as both a driver and a response to topographic change. However, conventional landscape evolution models do not explicitly account for the role of grain size, in part because algorithms for predicting hillslope grain size have been lacking. Here, we couple a recently proposed model for grain size production on hillslopes with a conventional landscape evolution model, to explore the controls on grain size at the landscape scale. We conducted a series of numerical experiments, varying rock uplift rate, temperature, precipitation and rock properties, to evolve a suite of steady‐state and transient landscapes. Model simulations suggest that rock uplift rate, through its effect on erosion rate and hillslope residence time, is more influential than climate in controlling the variation in hillslope grain size distributions in tectonically active landscapes. Overall, coarser size distributions result from faster rates of uplift, as well as from colder and drier conditions, and lithologies with lower erodibility and weathering susceptibility. These results are broadly consistent with patterns of hillslope grain size variation reported in field studies but likely underpredict the potential magnitude of variation because of the limitations of the model linking grain size and hillslope weathering. This work is a first step toward incorporating grain‐sized explicit algorithms for bedrock incision into landscape evolution models to capture the potential for feedback among grain size, climate, tectonics and lithology in evolving landscapes.


### `monsalve-2025-riverbeddynamics`

Monsalve, Angel D., Anderson, Samuel R., Gasparini, Nicole M., Yager, E. (2025). *RiverBedDynamics v1.0: a Landlab component for computing two-dimensional sediment transport and river bed evolution.* Geoscientific Model Development 18(11):3427-3451. DOI: [10.5194/gmd-18-3427-2025](https://doi.org/10.5194/gmd-18-3427-2025).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Computational landscape evolution models (LEMs) typically comprise at least two interacting components: a flow hydraulic solver that routes water across a landscape and a fluvial geomorphological model that modifies terrain properties, primarily bed surface elevation. LEMs used in long-term simulations over large watersheds, including some available in the Landlab library, often assume that only erosive processes occur in rivers and that terrain elevation increases solely due to tectonic uplift. Consequently, these models cannot capture the dynamics of gravel-bedded rivers, lacking the capacity to include sediment mixtures, simulate sediment deposition, and track textural changes in substrate stratigraphy that result from varying flow characteristics. To address this limitation, we developed, implemented, and tested RiverBedDynamics, a new Landlab component that simulates the evolution of bed surface elevation and grain size distribution in 2D grids based on the Exner equation for sediment mass balance. By dynamically coupling RiverBedDynamics with Landlab's hydrodynamic flow solver, OverlandFlow, we created a new LEM capable of simulating the dynamics of local shear stresses, bed load transport rates, and grain size distributions. Comparisons of our LEM results with analytical and previously reported solutions demonstrate its ability to accurately predict time-varying local changes in bed surface elevation, including erosion and deposition, as well as grain size distribution. Furthermore, application of our LEM to a synthetic watershed illustrates how spatially variable rainfall intensity leads to varying discharge patterns, which in turn drive changes in bed elevation and grain size distribution across the domain. This approach provides a more comprehensive representation of the complex interactions between flow dynamics and sediment transport in gravel-bedded rivers at timescales ranging from individual flood events to yearly morphological changes, enhancing our ability to model landscape evolution across diverse geomorphic settings.


### `morris-2025-impact`

Morris, M., Roberts, G. (2025). *Impact of noise on landscapes and metrics generated with stream power models.* Earth Surface Dynamics 13(5):1003-1038. DOI: [10.5194/esurf-13-1003-2025](https://doi.org/10.5194/esurf-13-1003-2025).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. The Stream Power Model (SPM) has become a cornerstone of quantitative geomorphology, widely used to predict landscape evolution including the generation, moderation, and lowering of Earth's topography, sedimentary flux and biogeochemical processes. It is well known that landscape geometries predicted by the SPM can be strongly influenced by noise. However, its impact on the uncertainties or probabilities of, for instance, drainage planform geometries and widely used metrics is poorly understood. Noise can be incorporated into SPM simulations in a variety of ways. For instance, random, low amplitude, topographic anomalies are often inserted into starting conditions to enhance the realism of calculated drainage networks. Spatio-temporal or quenched (frozen) noise also influence the trajectories of evolving landscapes. Our goal with this paper is to establish how noise impacts the probabilities of landscape geometries and the reliability of tectonic and erosional information recovered from them. A series of landscape evolution models are run in which different arrangements, distributions, and implementations of noise are added to models evolving under the same tectonic and erosional forcings to an equilibrium state. We quantify uncertainties that arise from incorporating different arrangements of typical (uniform; white) and naturalistic initial, quenched and spatio-temporal noise. We focus on three conclusions. First, tectonic rates and values of erosional-geometric parameters (e.g., concavity and steepness indices, Hack exponents) recovered via metrics-based approaches (e.g., slope-area, χ, length-area) are uncertain in the presence of noisy initial conditions. Recovered values from individual landscapes generated with the same distribution but different specific arrangement of noise are at least as uncertain as ranges attributed to, for instance, changes in aridity. In fact, even noise with amplitudes that are <1 % of cumulative uplift can cause tectonic rates to no longer be recoverable to within a factor of two of true values. These results emphasise the sensitivity of metrics that rely on calculating derivatives (e.g., slope-area, χ) to noise. Secondly, whilst noise can make landscape geometries highly uncertain (different in different simulations), the distributions of their geomorphic properties (e.g., hypsometries, channel length-area relationships, Hack exponents) appear to have well defined statistical properties (e.g., expected values and variance). Finally, we suggest that a useful way to assess the impact of noise on SPM predictions is to generate ensembles of hundreds to thousands of models in which different arrangements of the chosen distribution of noise are inserted. Doing so can provide means to quantify uncertainty in predicted geometries and derived metrics, which can be substantial.


### `nava-2025-neural`

Nava, Lorenzo, Chen, Ye, Vries, M. V. W. D. (2025). *Neural emulation of gravity-driven geohazard runout.* abs/2512.16221. DOI: [10.48550/arxiv.2512.16221](https://doi.org/10.48550/arxiv.2512.16221).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Predicting geohazard runout is critical for protecting lives, infrastructure and ecosystems. Rapid mass flows, including landslides and avalanches, cause several thousand deaths across a wide range of environments, often travelling many kilometres from their source. The wide range of source conditions and material properties governing these flows makes their runout difficult to anticipate, particularly for downstream communities that may be suddenly exposed to severe impacts. Accurately predicting runout at scale requires models that are both physically realistic and computationally efficient, yet existing approaches face a fundamental speed-realism trade-off. Here we train a machine learning model to predict geohazard runout across representative real world terrains. The model predicts both flow extent and deposit thickness with high accuracy and 100 to 10,000 times faster computation than numerical solvers. It is trained on over 100,000 numerical simulations across over 10,000 real world digital elevation model chips and reproduces key physical behaviours, including avulsion and deposition patterns, while generalizing across different flow types, sizes and landscapes. Our results demonstrate that neural emulation enables rapid, spatially resolved runout prediction across diverse real world terrains, opening new opportunities for disaster risk reduction and impact-based forecasting. These results highlight neural emulation as a promising pathway for extending physically realistic geohazard modelling to spatial and temporal scales relevant for large scale early warning systems.


### `oryan-2025-inferring`

Oryan, B., Gailleton, B., Olive, J., Malatesta, L., Jolivet, R. (2025). *Inferring Long‐Term Tectonic Uplift Patterns From Bayesian Inversion of Fluvially‐Incised Landscapes.* Journal of Geophysical Research: Solid Earth 130(12). DOI: [10.1029/2024jb030819](https://doi.org/10.1029/2024jb030819).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Earth surface processes encode the combined forcing of tectonics and climate in topography. Separating their contributions is essential for using landscapes as quantitative records of crustal deformation. Here, we develop a method to invert non‐dimensionalized, spatially variable fields of long‐term rock uplift and rock erodibility from fluvially incised landscapes, using an extended χ‐coordinate system that accounts for variability in uplift, erodibility, and precipitation ( χUKQ) $\chi _\textUKQ)$ . We invert 170 synthetically‐generated landscapes and demonstrate that our method accurately recovers the spatial variability of rock uplift and rock erodibility, even when applied to settings that deviate from the ideal model of equilibrated, detachment‐limited channels, which underpins the χUKQ $\chi _\textUKQ$ framework. We subsequently apply our inversion to six natural landscapes shaped by normal faults (half‐grabens), and to a 200‐km wide region of the central Himalayas. We show that our inversion can resolve the effect of climate and lithology while extracting uplift fields that are consistent with patterns expected from upper crustal flexure and previous estimates derived from geomorphological markers. The success of our method in recovering rock uplift patterns, isolated from the effects of climate and erodibility, highlights its applicability to settings where long‐term uplift trends are unknown, paving the path to deciphering tectonic fingerprints recorded in landscapes over tens of thousands of years.


### `petit-2025-reconstructing`

Petit, Carole, Jourdon, Anthony, Coltice, N. (2025). *Reconstructing landscapes: an adjoint model of the stream power and diffusion erosion equation.* Earth Surface Dynamics 13(6):1263-1280. DOI: [10.5194/esurf-13-1263-2025](https://doi.org/10.5194/esurf-13-1263-2025).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. We simulate landscape evolution using a diffusion-advection equation with a source term, where the advection velocity is derived from the classical parametrization of the Stream Power Law. This formulation allows for forward modeling of uplift, hillslope and fluvial erosion within a finite-element framework, and enables the use of adjoint methods for sensitivity analysis and parameter inversion. When considered individually, model parameters such as the diffusion coefficient, fluvial erodibility, initial topography, and time-dependent uplift can be inverted using constraints from final topography, sediment flux, or cumulative denudation at specific locations. Sensitivity analysis on a real landscape reveals that sensitivity to erosion parameters is higher in steep, high-relief areas and that hillslope diffusion and fluvial incision affect the model differently. After a series of tests on synthetic topographies, we apply the adjoint model to two natural cases: (1) reconstructing the pre-incision topography of the southeastern French Massif Central, which appears as a smooth, flat footwall bounded by a linear escarpment along a major lithological boundary; and (2) estimating the Quaternary uplift rate along the Wasatch Range, USA, where our model suggests a significant increase in uplift from 0.2 to 1 mm yr−1 over the last ∼ 2 million years.


### `prescott-2025-evaluation`

Prescott, Alexander B., Pelletier, Jon D., Chataut, Satya, Ananthanarayan, Sriram (2025). *An evaluation of flow-routing algorithms for calculating contributing area on regular grids.* Earth Surface Dynamics 13(2):239-256. DOI: [10.5194/esurf-13-239-2025](https://doi.org/10.5194/esurf-13-239-2025).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Abstract. Calculating contributing area (often used as a proxy for surface water discharge) within a digital elevation model (DEM) or landscape evolution model (LEM) is a fundamental operation in geomorphology. Here we document the fact that a commonly used multiple-flow-direction algorithm for calculating contributing area, i.e., D∞ of Tarboton (1997), is sufficiently biased along the cardinal and ordinal directions that it is unsuitable for some standard applications of flow-routing algorithms. We revisit the purported excess dispersion of the multiple-flow-direction (MFD) algorithm of Freeman (1991) that motivated the development of D∞ and demonstrate that MFD is superior to D∞ when tested against analytic solutions for the contributing areas of idealized landforms and the predictions of the shallow-water equation solver FLO-2D for more complex landforms in which the water surface slope is closely approximated by the bed slope. We also introduce a new flow-routing algorithm entitled IDS (in reference to the iterative depth- and slope-dependent nature of the algorithm) that is more suitable than MFD for applications in which the bed and water surface slopes differ substantially. IDS solves for water flow depths under steady hydrologic conditions by distributing the discharge delivered to each grid point from upslope to its downslope neighbors in rank order of elevation (highest to lowest) and in proportion to a power-law function of the square root of the water surface slope and the five-thirds power of the water depth, mimicking the relationships among water discharge, depth, and surface slope in Manning's equation. IDS is iterative in two ways: (1) water depths are added in small increments so that the water surface slope can gradually differ from the bed slope, facilitating the spreading of water in areas of laterally unconfined flow, and (2) the partitioning of discharge from high to low elevations can be repeated, improving the accuracy of the solution as the water depths of downslope grid points become more well approximated with each successive iteration. We assess the performance of IDS by comparing its results to those of FLO-2D for a variety of real and idealized landforms and to an analytic solution of the shallow-water equations. We also demonstrate how IDS can be modified to solve other fluid-dynamical nonlinear partial differential equations arising in Earth surface processes, such as the Boussinesq equation for the height of the water table in an unconfined aquifer.


### `cardosobihlo-2026-solver-in-the-loop`

Cardoso-Bihlo, Elsa, Bihlo, Alex (2026). *A solver-in-the-loop framework for end-to-end differentiable coastal hydrodynamics.* abs/2604.07129. DOI: [10.48550/arxiv.2604.07129](https://doi.org/10.48550/arxiv.2604.07129).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Numerical simulation of wave propagation and run-up is a cornerstone of coastal engineering and tsunami hazard assessment. However, applying these forward models to inverse problems, such as bathymetry estimation, source inversion, and structural optimization, remains notoriously difficult due to the rigidity and high computational cost of deriving discrete adjoints. In this paper, we introduce AegirJAX, a fully differentiable hydrodynamic solver based on the depth-integrated, non-hydrostatic shallow-water equations. By implementing the solver entirely within a reverse-mode automatic differentiation framework, AegirJAX treats the time-marching physics loop as a continuous computational graph. We demonstrate the framework's versatility across a suite of scientific machine learning tasks: (1) discovering regime-specific neural corrections for model misspecifications in highly dispersive wave propagation; (2) performing continuous topology optimization for breakwater design; (3) training recurrent neural networks in-the-loop for active wave cancellation; and (4) inverting hidden bathymetry and submarine landslide kinematics directly from downstream sensor data. The proposed differentiable paradigm fundamentally blurs the line between forward simulation and inverse optimization, offering a unified, end-to-end framework for coastal hydrodynamics.


### `duretz-2026-automatic`

Duretz, Thibault, de Montserrat, Albert, Sevilla, Rubén, Räss, Ludovic, Utkin, Ivan, Spang, Arne (2026). *Automatic tuning of iterative pseudo-transient solvers for modeling the deformation of heterogeneous media.* Geoscientific Model Development 19(12):5343-5362. DOI: [10.5194/gmd-19-5343-2026](https://doi.org/10.5194/gmd-19-5343-2026).

**Availability:** **in-repo PDF** — `ref/gmd-19-5343-2026.pdf` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> Geodynamic modeling has become a crucial tool for investigating the dynamics of Earth deformation across various scales. Such simulations often involve solving mechanical problems with significant material heterogeneities (e.g. strong viscosity contrasts) under nearly incompressible conditions. Recent advancements have enabled the development of iterative solvers based on Dynamic Relaxation (DR) or Pseudo-Transient schemes, which require minimal global communication and exhibit quasi-linear scaling on GPU and supercomputing architectures. These solvers incorporate automatic tuning of iterative parameters, including pseudo-time steps and damping coefficients, based on spectral estimates of the discrete operators, ensuring both robust and rapid convergence. We demonstrate the effectiveness of this approach on discretized problems with finite-difference and face-centered finite volume methods, including heterogeneous incompressible Stokes flows. Moreover, the relative algorithmic simplicity of DR-based methods allows straightforward extensions to compressible flow, multiphase flow, and nonlinear constitutive laws, opening promising avenues for large-scale, high-resolution simulations of geoscientific problems.


### `gauvain-2026-global`

Gauvain, Alexandre (2026). *A Global High-Resolution Hydrological Model to Simulate the Dynamics of Surface Liquid Reservoirs: Application on Mars.* DOI: [10.5194/egusphere-2025-4992](https://doi.org/10.5194/egusphere-2025-4992).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Surface runoff shapes planetary landscapes, but global hydrological models often lack the resolution and flexibility to simulate dynamic surface water bodies beyond Earth. Recent studies of Mars have revealed abundant geological and mineralogical evidence for past surface water, including valley networks, crater lakes, deltas and possible ocean margins dating from late Noachian to early Hesperian times. These features suggest that early Mars experienced periods allowing liquid water stability, runoff and sediment transport. To investigate where surface water could accumulate and how it may have been redistributed, we developed a global high-resolution (km-scale) surface hydrological model. The model uses a pre-computed hydrological database that maps topographic depressions, their spillover points, hierarchical connections between basins, and lake volume-area-elevation relationships. This database approach greatly accelerates simulations by avoiding repeated geomorphic processing. The model dynamically forms, grows, merges and dries lakes and putative seas without prescribing fixed coastlines, by transferring water volumes between depressions according to their storage capacities and overflow rules. We explore model behavior over the present-day Mars'topography measured by MOLA (Mars Orbiter Laser Altimeter) topography for a range of evaporation rates (from 0.1 m/yr to 10 m/yr) and total water inventories expressed as Global Equivalent Layer (from 1 mGEL to 1000 mGEL). 48 Simulations are iterated to reach the steady state. The model outputs the extent and depth of surface water bodies and identifies main drainage pathways using overflow fluxes as runoff indicators. Results show a transition toward a contiguous northern ocean between low (1-10 m) GEL values and increasing concentration of water in northern lowlands and major impact basins at higher GEL.


### `kern-2026-simultaneous`

Kern, Skyler, McGuinn, Mary E., Smith, Katherine M., Pinardi, Nadia, Niemeyer, Kyle E., Lovenduski, Nicole S., et al. (2026). *Simultaneous versus sequential estimation of biogeochemical and physical parameters in coupled marine ecosystem models.* Geoscientific Model Development 19(12):5601-5622. DOI: [10.5194/gmd-19-5601-2026](https://doi.org/10.5194/gmd-19-5601-2026).

**Availability:** **in-repo PDF** — `ref/gmd-19-5601-2026.pdf` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> As computational resources have increased in availability and capability, so has the complexity of the models used to represent biogeochemical (BGC) processes in ocean simulations. To effectively calibrate the increasingly large number of uncertain parameters in these models, efficient parameter estimation methods are needed to ensure that the models can accurately represent the BGC processes under investigation. In this study, we address this challenge using a multistage automatic parameter estimation methodology that sequentially applies global sampling and local optimization to calibrate both the BGC model parameters and the parameters associated with a one-dimensional physical ocean model. We quantitatively compare the accuracy of sequential and simultaneous parameter estimations of moderately complex BGC and physical models at locations corresponding to the Bermuda Atlantic time series and the Hawaii Ocean time series. The results show that the best overall agreement with the observed mean seasonal cycles is obtained when BGC, advection, boundary condition, and turbulent diffusion parameters are estimated simultaneously, rather than sequentially. Simultaneous estimation of all these parameters results in closer agreement with mean seasonal cycles for oxygen and particulate organic nitrogen. Moreover, the agreement is improved in general when the advection, boundary condition, and turbulent diffusion parameters are included in the estima- tion, as opposed to calibrating the BGC model alone. This study also serves as a demonstration of a meta-algorithm for parameter estimation in high-dimensional models using a truncated global search with local optimizations.


### `morris-2026-seeing`

Morris, M. J. (2026). *Seeing Through Geomorphic Complexity to Recover Tectonics From Topography: Inverting Landscapes for Uplift Histories Using the Wasserstein Distance.* Journal of Geophysical Research: Earth Surface 131(4). DOI: [10.1029/2025jf008966](https://doi.org/10.1029/2025jf008966).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> An important problem in the Earth sciences is extracting information about tectonic and other processes from topography. A general challenge is that geomorphic activity that we typically have little information about during the lifetime of a landscape can introduce geomorphic "noise". Such noise, producing changes in elevation at scales typically ≪100 $\ll 100$ m, could be introduced by variations in lithology, biology, climate, and sedimentology, for instance. It can dramatically impact the way in which landscapes evolve and their form, including shapes and positions of drainage networks. We seek to establish how information about uplift rate histories can be extracted from entire landscapes despite the presence of noise. The sensitivity of single landscape simulations to noise suggests that statistical and inverse modeling approaches utilizing model ensembles may be required. We establish the use of Wasserstein distances in an inverse modeling framework to recover uplift rate histories. We test optimization techniques that automate the search for optimal models (i.e., uplift rate histories) including direction‐based and ameba‐simplex algorithms, confirming that the Neighborhood algorithm is well suited to the task. This approach works even when noise demonstrably plays an important role in determining landscape form but is poorly constrained. It is developed and tested using synthetic landscapes generated with the stream power erosional model and increasingly complex uplift and noise scenarios. The results indicate that it is possible to recover the history of uplift from natural landscapes in which the origin of the specific arrangements of channels, valleys, interfluves, etc. are poorly understood.


### `moses-2026-dj4earth`

Moses, William S. (2026). *DJ4Earth: Differentiable, and Performance‐Portable Earth System Modeling via Program Transformations.* Journal of Advances in Modeling Earth Systems 18(5). DOI: [10.1029/2025ms005615](https://doi.org/10.1029/2025ms005615).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> ' Differentiable Earth system models (ESMs) enable powerful applications such as sensitivity analysis, gradient‐based calibration, state estimation, boundary flux inversions, uncertainty quantification, and online machine learning. Reverse‐mode automatic differentiation (AD) efficiently provides gradients for such tasks, yet models have rarely included this capability because of complex, bespoke numerical algorithms. As part of the Differentiable programming in Julia for Earth system modeling (DJ4Earth) initiative, we present improved capabilities of the AD tool Enzyme.jl and the new compiler transpilation tool Reactant.jl, augmented by sophisticated checkpointing algorithms, which, together make general‐purpose AD tractable and efficient for full‐fledged ESM components written in Julia. Operating at the low‐level virtual machine intermediate representation or multi‐level intermediate representation compiler levels, these frameworks support mutable memory, custom kernels, and compiler optimizations before and after differentiation. Julia‐specific challenges related to just‐in‐time compilation and garbage collection are handled efficiently. Reactant further enables automatic performance portability across central processing units, graphics processing units, and tensor processing units, facilitating use of emerging AI‐customized high‐performance computing architectures. We demonstrate these frameworks on four Julia‐based ESM components featuring diverse spatial discretizations and numerical algorithms: the rotating‐sphere shallow water model ShallowWaters.jl, the finite‐volume ocean model Oceananigans.jl, the finite‐element ice sheet model DJUICE.jl, and the spectral atmospheric model SpeedyWeather.jl. Across these ESM components, our tools compute efficient and correct gradients. These results establish a foundation for differentiable, high‐performance and performance‐portable ESMs that can integrate neural networks for unresolved processes, trained online, enabling next‐generation hybrid physics–machine learning ESMs constrained by physical dynamics and observations.


### `ren-2026-modular`

Ren, Yuhang (2026). *A Modular Implicit Numerical Method for Hillslope Sediment Transport Laws.* Journal of Geophysical Research: Earth Surface 131(1). DOI: [10.1029/2025jf008675](https://doi.org/10.1029/2025jf008675).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Long‐term hillslope evolution involves complex continuous (i.e., soil creep) and discontinuous (i.e., landslide) processes. The nonlinearity introduced using discontinuous processes poses significant challenges to the numerical stability of long‐term hillslope numerical simulations. Geomorphologists have proposed numerous sediment transport laws related to local slopes, each tailored to specific geomorphic processes. However, many complex hillslope sediment transport models, such as the depth–slope product model and the nonlinear depth‐ and slope‐dependent model, lack available implicit solution schemes. We address this issue by proposing an implicit numerical scheme named "Modular Implicit Method." Taking the nonlinear slope‐dependent transport law as an example, our method splits the sediment transport law into gradient and coefficient functions and then directly computes the coefficient functions using a lagged strategy. Numerical experiments demonstrate that our method performs comparably to existing implicit methods in solving nonlinear transport models. Long‐term evolution simulations on realistic hillslopes show that our method enables flexible switching and co‐existence of multiple transport laws, demonstrating strong extensibility. Furthermore, the framework seamlessly couples with the stream‐power incision model, exhibiting a more general compatibility.


### `roberge-2026-concentrationtracker`

Roberge, L., Gasparini, N., Campforts, B., Tucker, Gregory E. (2026). *ConcentrationTracker: Landlab components for tracking material concentrations in sediment.* Geoscientific Model Development 19(3):1387-1404. DOI: [10.5194/gmd-19-1387-2026](https://doi.org/10.5194/gmd-19-1387-2026).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract. We present a set of new Landlab numerical model components that allow users to track sediment properties across a landscape grid. The components use a mass-balance approach to partition the mass concentration of each property based on sediment fluxes calculated by various Landlab flux components. The methods are generic, allowing the user to assign any sediment property that can be expressed as a mass, volume, or number concentration (for example, mass of magnetite, volume of quartz, number of zircons, number of radiogenic 10Be atoms, "equivalent dose" of luminescence). Several properties can be tracked at once, each with concentration tracked in both sediment and bedrock at every location on the grid. Two ConcentrationTracker components have been formulated; one for distributed, space- and time-varying hillslope regolith movement and another for transport in fluvial networks, allowing for interaction between sediment in the water column and on the channel bed. These components can be used individually to study a single process or coupled to study the interactions of multiple processes acting on a dynamic landscape. We present two examples that illustrate the diverse uses of the ConcentrationTracker components: colour banding in hillslope regolith and provenance tracking of fluvial sediments.


### `zhao-2026-deciphering`

Zhao, Xiang, Shen, Yangen, Zhang, Guihong, Han, Xu, Sun, Mengdi (2026). *Deciphering the field of uplift from landscape: A genetic algorithm‐based inversion method.* Earth Surface Processes and Landforms 51(3). DOI: [10.1002/esp.70266](https://doi.org/10.1002/esp.70266).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Landscape evolution model (LEM) is an effective numerical simulation tool to predict topographic changes in response to tectonic uplift and surface erosion. Combining with an inverse algorithm, the LEM have been used for deducing the tectonic and climatic factors from the current landforms. However, the predictive capability of LEMs is fundamentally limited by uncertainty in boundary conditions and parameters, particularly the spatially variable rock uplift rate field, which serves as the primary driving force in landscape evolution. This study introduces a novel, data‐driven method to invert for the initial uplift field using a genetic algorithm (GA). By coupling a GA with the LEM Fastscape and employing present‐day topography as the target, our method iteratively optimizes candidate uplift rate fields. Key features of this work include a multi‐dimensional fitness function integrating traditional geomorphic metrics with perceptual similarity (LPIPS) to better capture realistic landscape features, and an effective dimensionality reduction strategy for parameterizing the uplift field, enabling efficient inversion of complex spatial patterns using a GA. The method's performance is rigorously evaluated through a series of synthetic experiments with progressively complex uplift patterns, demonstrating the robust performance for simple Unimodal and medium complexity bimodal uplift patterns and a modest degradation for Complex Sinusoidal Patterns. Applying the method to the tectonically active Taiwan Central Range, our method effectively converges to an optimal uplift rate field that aligns with independent thermochronological constraints. This GA‐based inversion provides a robust and objective approach for determining crucial LEM initial conditions, enhancing our ability to decipher the interactions between tectonic and Earth surface processes in tectonically active regions.



---

## G. Surface hydraulics & sediment transport

*9 entries.*

### `rouse-1937-modern`

Rouse, Hunter (1937). *Modern Conceptions of the Mechanics of Fluid Turbulence.* Transactions of the American Society of Civil Engineers 102:463--543.

**Availability:** **in relata — metadata only, no PDF held**

**Abstract:** ABSTRACT NOT RETRIEVED. A 1937 ASCE Transactions paper. **No abstract exists** — pre-abstract-era publication. Crossref resolves the DOI and confirms all bibliographic fields; there is nothing to retrieve.


### `fischer-1979-mixing`

Fischer, Hugo B., List, E. John, Koh, Robert C. Y., Imberger, J\"org, Brooks, Norman H. (1979). *Mixing in Inland and Coastal Waters.* Academic Press.

**Availability:** **in relata — metadata only, no PDF held**

**Abstract:** ABSTRACT NOT RETRIEVED. A book (Academic Press, ISBN 9780122581502). **No abstract exists.**


### `jarrett-1984-hydraulics`

Jarrett, Robert D. (1984). *Hydraulics of High-Gradient Streams.* Journal of Hydraulic Engineering 110(11):1519--1539. DOI: [10.1061/(ASCE)0733-9429(1984)110:11(1519)](https://doi.org/10.1061/(ASCE)0733-9429(1984)110:11(1519)).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher record (ASCE))*

> Onsite surveys and 75 measurements of discharge were made on 21 high-gradient streams (slopes greater than 0.002) for the purpose of computing the Manning roughness coefficient, n, and to provide data on the hydraulics of these streams. These data show that: (1) n varies inversely with depth; (2) n varies directly with slope; and (3) streams thought to be in the supercritical flow range were actually in the subcritical range. A simple and objective method was employed to develop an equation for predicting the n of high-gradient streams by using multiple-regression techniques and measurements of the slope and hydraulic radius. The average standard error of estimate of this prediction equation was 28% when tested with Colorado data. The equation was verified using other data available for high-gradient streams. Regime-flow equations for velocity and discharge also were developed.


### `dietrich-1989-sediment`

Dietrich, William E., Kirchner, James W., Ikeda, Hiroshi, Iseya, Fujiko (1989). *Sediment Supply and the Development of the Coarse Surface Layer in Gravel-Bedded Rivers.* Nature 340:215--217. DOI: [10.1038/340215a0](https://doi.org/10.1038/340215a0).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher record)*

> The bed surface of most gravel rivers is considerably coarser than the sub-surface or the gravel load transported over it, a phenomenon affecting river dynamics, morphology and ecology. The coarse surface layer, often called an armour or pavement, has been attributed to an inherent tendency for small grains to settle between larger ones during active transport of all sizes; and to selective erosion or trapping of finer particles when the coarse grains are relatively immobile. Where bedload supply is cut off below dams, selective erosion causes coarsening. Using a simple quantitative model, we propose that surface coarsening develops in gravel-bedded rivers when local bedload supply from upstream is less than the ability of the flow to transport that load. We present laboratory results which support this hypothesis, and show that supply reduction causes changes in bedforms and progressive confinement of active bedload transport to a narrow band of finer bed surface bordered by a coarse, less active bed. It may therefore be possible to relate the degree of river-bed surface coarsening to sediment supply resulting from land use.


### `grant-1997-critical`

Grant, Gordon E. (1997). *Critical Flow Constrains Flow Hydraulics in Mobile-Bed Streams: A New Hypothesis.* Water Resources Research 33(2):349--358. DOI: [10.1029/96WR03134](https://doi.org/10.1029/96WR03134).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> A new hypothesis predicts that in mobile‐bed river channels, interactions between the channel hydraulics and bed configuration prevent the Froude number (Fr) from exceeding 1 for more than short distances or periods of time. Flow conditions in many steep, competent streams appear to be close to critical. Froude numbers of steep (slope ≈ 0.01) sand‐bed streams with considerable freedom to adjust boundaries oscillate between 0.7 and 1.3 over 20‐ to 30‐s cycles, with an average of 1.0 at the channel thalweg. Critical flow in these streams is maintained by the interaction between the mobile bed and free water surface at high Fr, which results in a cyclical pattern of creation and destruction of bed forms. Field observations support that a similar mechanism of flow–bed form interaction constrains Fr ≤ 1 in active‐bed braided gravel rivers, step–pool streams, laboratory rills, lahar‐runout channels, and even some bedrock channels. Empirical and analytical results show that as slope increases, competent flows tend to asymptotically approach critical flow. An assumption of critical flow would dramatically simplify paleohydraulic flow reconstructions and modeling of flow hydraulics in high gradient streams.


### `mei-2007-fast`

Mei, Xing, Decaudin, Philippe, Hu, Bao-Gang (2007). *Fast Hydraulic Erosion Simulation and Visualization on GPU.* 15th Pacific Conference on Computer Graphics and Applications (PG'07):47--56. DOI: [10.1109/PG.2007.15](https://doi.org/10.1109/PG.2007.15).

**Availability:** PDF in relata store

**Abstract** *(source: OpenAlex)*

> Natural mountains and valleys are gradually eroded by rainfall and river flows. Physically-based modeling of this complex phenomenon is a major concern in producing realistic synthesized terrains. However, despite some recent improvements, existing algorithms are still computationally expensive, leading to a time-consuming process fairly impractical for terrain designers and 3D artists. In this paper, we present a new method to model the hydraulic erosion phenomenon which runs at interactive rates on today's computers. The method is based on the velocity field of the running water, which is created with an efficient shallow-water fluid model. The velocity field is used to calculate the erosion and deposition process, and the sediment transportation process. The method has been carefully designed to be implemented totally on GPU, and thus takes full advantage of the parallelism of current graphics hardware. Results from experiments demonstrate that the proposed method is effective and efficient. It can create realistic erosion effects by rainfall and river flows, and produce fast simulation results for terrains with large sizes.


### `bates-2010-simple`

Bates, Paul D., Horritt, Matthew S., Fewtrell, Timothy J. (2010). *A Simple Inertial Formulation of the Shallow Water Equations for Efficient Two-Dimensional Flood Inundation Modelling.* Journal of Hydrology 387(1--2):33--45. DOI: [10.1016/j.jhydrol.2010.03.027](https://doi.org/10.1016/j.jhydrol.2010.03.027).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher record (NASA ADS))*

> This paper describes the development of a new set of equations derived from 1D shallow water theory for use in 2D storage cell inundation models where flows in the x and y Cartesian directions are decoupled. The new equation set is designed to be solved explicitly at very low computational cost, and is here tested against a suite of four test cases of increasing complexity. In each case the predicted water depths compare favourably to analytical solutions or to simulation results from the diffusive storage cell code of Hunter et al. (2005). For the most complex test involving the fine spatial resolution simulation of flow in a topographically complex urban area the Root Mean Squared Difference between the new formulation and the model of Hunter et al. is ~1 cm. However, unlike diffusive storage cell codes where the stable time step scales with (1/dx)^2, the new equation set developed here represents shallow water wave propagation and so the stability is controlled by the Courant-Freidrichs-Lewy condition such that the stable time step instead scales with 1/dx. This allows use of a stable time step that is 1-3 orders of magnitude greater for typical cell sizes than that possible with diffusive storage cell models and results in commensurate reductions in model run times. For the tests reported in this paper the maximum speed up achieved over a diffusive storage cell model was 1120x, although the actual value seen will depend on model resolution and water surface gradient. Solutions using the new equation set are shown to be grid-independent for the conditions considered and to have an intuitively correct sensitivity to friction, however small instabilities and increased errors on predicted depth were noted when Manning's n = 0.01. The new equations are likely to find widespread application in many types of flood inundation modelling and should provide a useful additional tool, alongside more established model formulations, for a variety of flood risk management studies.


### `almeida-2012-improving`

de Almeida, Gustavo A. M., Bates, Paul, Freer, James E., Souvignet, Maxime (2012). *Improving the Stability of a Simple Formulation of the Shallow Water Equations for 2-D Flood Modeling.* Water Resources Research 48(5):W05528. DOI: [10.1029/2011WR011570](https://doi.org/10.1029/2011WR011570).

**Availability:** **in-repo PDF** — `ref/hydrology/pdfs/almeida-2012-improving.pdf` · PDF in relata store

**Abstract** *(source: Crossref)*

> The ability of two‐dimensional hydrodynamic models to accurately and efficiently predict the propagation of floods over large urban areas is of paramount importance for flood risk assessment and management. Paradoxically, it is in these highly relevant urban domains where flood modeling faces some of the most challenging obstacles. This is because of the very high‐resolution topography that is typically required to capture key hydraulic features, which significantly increases the computational time of the model. One particularly interesting solution to this difficulty was recently proposed in the form of a numerical scheme for the solution of a simplified version of the shallow water equations, which yields a system of two explicit equations that captures the most relevant hydraulic processes at very high computational efficiency. However, some stability problems were reported, especially when this formulation is applied to low friction areas. This is of particular importance in urban areas, where smooth surfaces are usually abundant. This paper proposes and tests two modifications of this previous numerical scheme that considerably improves the numerical stability of the model. Model improvements were assessed against a structured set of idealized test cases and finally in the simulation of flood propagation over complex topography in a highly urbanized area in London, United Kingdom. The enhanced stability achieved by the new formulation comes at no significant additional computational cost and, in fact, the model performance can benefit from the longer time steps that are allowed by the new scheme.


### `almeida-2013-applicability`

de Almeida, Gustavo A. M., Bates, Paul (2013). *Applicability of the Local Inertial Approximation of the Shallow Water Equations to Flood Modeling.* Water Resources Research 49(8):4833--4844. DOI: [10.1002/wrcr.20366](https://doi.org/10.1002/wrcr.20366).

**Availability:** **in-repo PDF** — `ref/hydrology/pdfs/almeida-2013-applicability.pdf` · PDF in relata store

**Abstract** *(source: Crossref)*

> Recent studies have demonstrated the improved computational performance of a computer algorithm based on a simplification of the shallow water equations—the so‐called local inertial approximation—which has been observed to provide results comparable to the full set of equations in a range of flood flow problems. This study presents an extended view on the local inertial system, shedding light on those key elements necessary to understand its applicability to flows of practical interest. First, the properties of the simplified system with potential impact on the accuracy of the solutions are described and compared to the corresponding full‐dynamic counterparts. In light of this discussion, the behavior of the solutions is then analyzed through a set of rigorously designed test cases in which analytical solutions to the shallow water system are available. Results show a general good agreement between the local inertial and full‐dynamic models, especially in the lower range of subcritical flows (Fr &lt; 0.5). In terms of steady nonuniform flow water profiles, the error introduced by the local inertial approximation leads to milder water depth gradients, which results in attenuated spatial changes in depth. In unsteady problems, the local inertial approximation leads to slower flood propagation speeds than those predicted by the full‐dynamic equations. Even though our results suggest that the magnitude of these errors is small in a range of floodplain and lowland channels, it becomes increasingly relevant with increasing Fr and depth gradients.



---

## H. Terrain synthesis, erosion authoring & the graphics lineage

*18 entries.*

### `musgrave-1989-synthesis`

Musgrave, F. Kenton, Kolb, Craig E., Mace, Robert S. (1989). *The Synthesis and Rendering of Eroded Fractal Terrains.* ACM SIGGRAPH Computer Graphics 23(3):41--50. DOI: [10.1145/74334.74337](https://doi.org/10.1145/74334.74337).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> In standard fractal terrain models based on fractional Brownian motion the statistical character of the surface is, by design, the same everywhere. A new approach to the synthesis of fractal terrain height fields is presented which, in contrast to previous techniques, features locally independent control of the frequencies composing the surface, and thus local control of fractal dimension and other statistical characteristics. The new technique, termed noise synthesis , is intermediate in difficulty of implementation, between simple stochastic subdivision and Fourier filtering or generalized stochastic subdivision, and does not suffer the drawbacks of creases or periodicity. Varying the local crossover scale of fractal character or the fractal dimension with altitude or other functions yields more realistic first approximations to eroded landscapes. A simple physical erosion model is then suggested which simulates hydraulic and thermal erosion processes to create gloabl stream/valley networks and talus slopes. Finally, an efficient ray tracing algorithm for general height fields, of which most fractal terrains are a subset, is presented.


### `perlin-2002-improving`

Perlin, Ken (2002). *Improving Noise.* Proceedings of SIGGRAPH 2002. DOI: [10.1145/566570.566636](https://doi.org/10.1145/566570.566636).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> Two deficiencies in the original Noise algorithm are corrected: second order interpolation discontinuity and unoptimal gradient computation. With these defects corrected, Noise both looks better and runs faster. The latter change also makes it easier to define a uniform mathematical reference standard.


### `cordonnier-2016-large`

Cordonnier, Guillaume, Braun, Jean, Cani, Marie-Paule, Benes, Bedrich, Galin, Eric, Peytavie, Adrien, et al. (2016). *Large Scale Terrain Generation from Tectonic Uplift and Fluvial Erosion.* Computer Graphics Forum 35(2):165--175. DOI: [10.1111/cgf.12820](https://doi.org/10.1111/cgf.12820).

**Availability:** **in-repo PDF** — `ref/geology/pdfs/cordonnier-2016-large.pdf` · PDF in relata store

**Abstract** *(source: Crossref)*

> At large scale, landscapes result from the combination of two major processes: tectonics which generate the main relief through crust uplift, and weather which accounts for erosion. This paper presents the first method in computer graphics that combines uplift and hydraulic erosion to generate visually plausible terrains. Given a user‐painted uplift map, we generate a stream graph over the entire domain embedding elevation information and stream flow. Our approach relies on the stream power equation introduced in geology for hydraulic erosion. By combining crust uplift and stream power erosion we generate large realistic terrains at a low computational cost. Finally, we convert this graph into a digital elevation model by blending landform feature kernels whose parameters are derived from the information in the graph. Our method gives high‐level control over the large scale dendritic structures of the resulting river networks, watersheds, and mountains ridges.


### `cortial-2019-procedural`

Cortial, Yann, Peytavie, Adrien, Galin, Eric, Guerin, Eric (2019). *Procedural Tectonic Planets.* Computer Graphics Forum 38(2). DOI: [10.1111/cgf.13614](https://doi.org/10.1111/cgf.13614).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> We present a procedural method for authoring synthetic tectonic planets. Instead of relying on computationally demanding physically‐based simulations, we capture the fundamental phenomena into a procedural method that faithfully reproduces large‐scale planetary features generated by the movement and collision of the tectonic plates. We approximate complex phenomena such as plate subduction or collisions to deform the lithosphere, including the continental and oceanic crusts. The user can control the movement of the plates, which dynamically evolve and generate a variety of landforms such as continents, oceanic ridges, large scale mountain ranges or island arcs. Finally, we amplify the large‐scale planet model with either procedurally‐defined or real‐world elevation data to synthesize coherent detailed reliefs. Our method allows the user to control the evolution of an entire planet interactively, and to trigger specific events such as catastrophic plate rifting.


### `galin-2019-review`

Galin, Eric, Guerin, Eric, Peytavie, Adrien, Cordonnier, Guillaume, Cani, Marie-Paule, Benes, Bedrich, et al. (2019). *A Review of Digital Terrain Modeling.* Computer Graphics Forum 38(2). DOI: [10.1111/cgf.13657](https://doi.org/10.1111/cgf.13657).

**Availability:** **in-repo PDF + markdown-converted** — `ref/research/pdfs/galin-2019-review.pdf`, `ref/research/pdfs/markdowns/` · PDF in relata store

**Abstract** *(source: Crossref)*

> Terrains are a crucial component of three‐dimensional scenes and are present in many Computer Graphics applications. Terrain modeling methods focus on capturing landforms in all their intricate detail, including eroded valleys arising from the interplay of varied phenomena, dendritic mountain ranges, and complex river networks. Set against this visual complexity is the need for user control over terrain features, without which designers are unable to adequately express their artistic intent. This article provides an overview of current terrain modeling and authoring techniques, organized according to three categories: procedural modeling, physically‐based simulation of erosion and land formation processes, and example‐based methods driven by scanned terrain data. We compare and contrast these techniques according to several criteria, specifically: the variety of achievable landforms; realism from both a perceptual and geomorphological perspective; issues of scale in terms of terrain extent and sampling precision; the different interaction metaphors and attendant forms of user‐control, and computation and memory performance. We conclude with an in‐depth discussion of possible research directions and outstanding technical and scientific challenges.


### `hu-2023-terrain`

Hu, Zexin, Hu, Kun, Mo, C., Pan, Lei, Wang, Zhiyong (2023). *Terrain Diffusion Network: Climatic-Aware Terrain Generation with Geological Sketch Guidance.* abs/2308.16725. DOI: [10.48550/arxiv.2308.16725](https://doi.org/10.48550/arxiv.2308.16725).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Sketch-based terrain generation seeks to create realistic landscapes for virtual environments in various applications such as computer games, animation and virtual reality. Recently, deep learning based terrain generation has emerged, notably the ones based on generative adversarial networks (GAN). However, these methods often struggle to fulfill the requirements of flexible user control and maintain generative diversity for realistic terrain. Therefore, we propose a novel diffusion-based method, namely terrain diffusion network (TDN), which actively incorporates user guidance for enhanced controllability, taking into account terrain features like rivers, ridges, basins, and peaks. Instead of adhering to a conventional monolithic denoising process, which often compromises the fidelity of terrain details or the alignment with user control, a multi-level denoising scheme is proposed to generate more realistic terrains by taking into account fine-grained details, particularly those related to climatic patterns influenced by erosion and tectonic activities. Specifically, three terrain synthesisers are designed for structural, intermediate, and fine-grained level denoising purposes, which allow each synthesiser concentrate on a distinct terrain aspect. Moreover, to maximise the efficiency of our TDN, we further introduce terrain and sketch latent spaces for the synthesizers with pre-trained terrain autoencoders. Comprehensive experiments on a new dataset constructed from NASA Topology Images clearly demonstrate the effectiveness of our proposed method, achieving the state-of-the-art performance. Our code is available at https://github.com/TDNResearch/TDN.


### `lochner-2023-interactive`

Lochner, Joshua (2023). *Interactive Authoring of Terrain using Diffusion Models.* Computer Graphics Forum 42(7). DOI: [10.1111/cgf.14941](https://doi.org/10.1111/cgf.14941).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Generating heightfield terrains is a necessary precursor to the depiction of computer‐generated natural scenes in a variety of applications. Authoring such terrains is made challenging by the need for interactive feedback, effective user control, and perceptually realistic output encompassing a range of landforms. We address these challenges by developing a terrain‐authoring framework underpinned by an adaptation of diffusion models for conditional image synthesis, trained on real‐world elevation data. This framework supports automated cleaning of the training set; authoring control through style selection and feature sketches; the ability to import and freely edit pre‐existing terrains, and resolution amplification up to the limits of the source data. Our framework improves on previous machine‐learning approaches by: expanding landform variety beyond mountainous terrain to encompass cliffs, canyons, and plains; providing a better balance between terseness and specificity in user control, and improving the fidelity of global terrain structure and perceptual realism. This is demonstrated through drainage simulations and a user study testing the perceived realism for different classes of terrain. The full source code, blender add‐on, and pre‐trained models are available.


### `schott-2023-large`

Schott, Hugo, Paris, Axel, Fournier, Lucie, Guerin, Eric, Galin, Eric (2023). *Large-scale Terrain Authoring through Interactive Erosion Simulation.* ACM Transactions on Graphics 42(5). DOI: [10.1145/3592787](https://doi.org/10.1145/3592787).

**Availability:** **in-repo PDF** — `ref/geology/pdfs/schott-2023-large.pdf` · PDF in relata store

**Abstract** *(source: relata record)*

> Large-scale terrains are essential in the definition of virtual worlds. Given the diversity of landforms and the geomorphological complexity, there is a need for authoring techniques offering hydrological consistency without sacrificing user control. In this article, we bridge the gap between large-scale erosion simulation and authoring into an efficient framework. We set aside modeling in the elevation domain in favour of the uplift domain and compute emerging reliefs by simulating the stream power erosion. Our simulation relies on a fast yet accurate approximation of drainage area and flow routing to compute the erosion interactively, which allows for incremental authoring. Our model provides landscape artists with tools for shaping mountain ranges and valleys, such as copy-and-paste operations; warping for imitating folds and faults; and point and curve elevation constraints to precisely sculpt ridges or carve river networks. It also lends itself to inverse procedural modeling by reconstructing the uplift from an input digital elevation model and allows hydrologically consistent blending between terrain patches.


### `cao-2024-integrating`

Cao, Haoyu, Xiong, Liyang, Wang, Hongen, Zhao, Fei, Strobl, Josef (2024). *Integrating hydrological knowledge into deep learning for DEM super-resolution.* International Journal of Geographical Information Science 39(2):301--325. DOI: [10.1080/13658816.2024.2410345](https://doi.org/10.1080/13658816.2024.2410345).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Abstract Deep learning-based super-resolution methods have been successfully applied to digital elevation model (DEM) downscaling studies by designing structures and loss functions of the model. However, little attention has been paid to the design of super-resolution models that can maintain the hydrological characteristics of the DEM, which is important for hydrological studies. This study introduces a super-resolution model that integrates hydrologic knowledge (HKSRCGAN), with the aim to effectively maintain topographic features as well as the hydrologic connectivity of the DEM. The hydrological knowledge derived from surface flow direction and hydrological features are integrated into a deep learning algorithm to guide model training. The 30 m spatial resolution FABDEM is used to demonstrate the utility of the proposed method. Results show that the HKSRCGAN outperforms the bicubic interpolation, SRCNN, SRGAN, SRResNet and TfaSR methods in reducing topographic errors and maintaining hydrologic characteristics. In the test area, the entropy difference analysis shows that the DEM generated by HKSRCGAN is similar to the information contained in the reference DEM. Furthermore, super-resolution models integrating hydrological knowledge are valuable for modeling terrain primarily shaped by gravity and surface water flows. In the future, deep learning-based models integrating hydrologic knowledge are expected to be applied in DEM upscaling to maintain consistent hydrological characteristics.


### `lo-2024-diff-dem`

Lo, Kyle Shih-Huang, Peters, Jorg (2024). *Diff-DEM: A Diffusion Probabilistic Approach to Digital Elevation Model Void Filling.* IEEE Geoscience and Remote Sensing Letters 21:1--5. DOI: [10.1109/lgrs.2024.3403835](https://doi.org/10.1109/lgrs.2024.3403835).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Digital elevation models (DEMs) are crucial for modeling and analyzing terrestrial environments, but voids in DEMs can compromise their downstream use. Diff-DEM is a self-supervised method for filling DEM voids that leverages a Denoising Diffusion Probabilistic Model (DDPM). Conditioned on a void-containing DEM, the DDPM acts as a transition kernel in the diffusion reversal, progressively reconstructing a sharp and accurate DEM. Both qualitative and quantitative assessments demonstrate that Diff-DEM outperforms existing DEM inpainting, including generative adversarial network (GAN) methods, inverse distance weighting (IDW), Kriging, LR B-spline, and Perona-Malik diffusion. The comparison is on Gavriil's and on our benchmark that expands Gavriil's dataset from 63 to 217 full-size ( $5051 \times 5051$ ) 10-m GeoTIFF images sourced from the Norwegian Mapping Authority; and from 50 DEMs to three groups of 1 k each of increasing void size. The code and dataset are available at https://github.com/kylelo/Diff-DEM.


### `nilles-2024-real-time`

Nilles, A., Günther, Lars, Wagner, Tobias, Müller, Stefan (2024). *3D Real-Time Hydraulic Erosion Simulation using Multi-Layered Heightmaps.* DOI: [10.2312/vmv.20241211](https://doi.org/10.2312/vmv.20241211).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> We present a novel method for real-time 3D hydraulic erosion simulation of large-scale terrain. Existing hydraulic erosion methods based on heightmaps and the virtual pipes method are extended to multi-layered heightmaps that can represent more complex 3D features. Our new method for horizontal erosion is able to create overhangs, arches and to some extent caves by allowing water to erode horizontally adjacent bedrock, eventually splitting a column into two new columns. Additionally, we developed an iterative method for bedrock support check that efficiently prevents floating terrain and unrealistic overhangs by propagating bedrock connectivity while incorporating the weight of each column. We implement our method in CUDA, using only features that are also available in standard compute shaders. On a RTX 3070, the resulting method runs at approximately 6ms and 23ms per simulation step for resolutions of 2048 2 and 4096 2 , respectively.


### `sharma-2024-earthgen`

Sharma, Ansh (2024). *EarthGen: Generating the World from Top-Down Views.* abs/2409.01491. DOI: [10.48550/arxiv.2409.01491](https://doi.org/10.48550/arxiv.2409.01491).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> In this work, we present a novel method for extensive multi-scale generative terrain modeling. At the core of our model is a cascade of superresolution diffusion models that can be combined to produce consistent images across multiple resolutions. Pairing this concept with a tiled generation method yields a scalable system that can generate thousands of square kilometers of realistic Earth surfaces at high resolution. We evaluate our method on a dataset collected from Bing Maps and show that it outperforms super-resolution baselines on the extreme super-resolution task of 1024x zoom. We also demonstrate its ability to create diverse and coherent scenes via an interactive gigapixel-scale generated map. Finally, we demonstrate how our system can be extended to enable novel content creation applications including controllable world generation and 3D scene generation.


### `tzathas-2024-physically`

Tzathas, Petros, Gailleton, B., Steer, P., Cordonnier, G. (2024). *Physically‐based analytical erosion for fast terrain generation.* Computer Graphics Forum 43(2). DOI: [10.1111/cgf.15033](https://doi.org/10.1111/cgf.15033).

**Availability:** PDF in relata store

**Abstract** *(source: relata record)*

> Terrain generation methods have long been divided between procedural and physically‐based. Procedural methods build upon the fast evaluation of a mathematical function but suffer from a lack of geological consistency, while physically‐based simulation enforces this consistency at the cost of thousands of iterations unraveling the history of the landscape. In particular, the simulation of the competition between tectonic uplift and fluvial erosion expressed by the stream power law raised recent interest in computer graphics as this allows the generation and control of consistent large‐scale mountain ranges, albeit at the cost of a lengthy simulation. In this paper, we explore the analytical solutions of the stream power law and propose a method that is both physically‐based and procedural, allowing fast and consistent large‐scale terrain generation. In our approach, time is no longer the stopping criterion of an iterative process but acts as the parameter of a mathematical function, a slider that controls the aging of the input terrain from a subtle erosion to the complete replacement by a fully formed mountain range. While analytical solutions have been proposed by the geomorphology community for the 1D case, extending them to a 2D heightmap proves challenging. We propose an efficient implementation of the analytical solutions with a multigrid accelerated iterative process and solutions to incorporate landslides and hillslope processes – two erosion factors that complement the stream power law.


### `yang-2024-unerosion`

Yang, Zhanyu, Cordonnier, G., Cani, Marie-Paule, Perrenoud, Christian, Benes, Bedrich (2024). *Unerosion: Simulating Terrain Evolution Back in Time.* Computer Graphics Forum 43(8). DOI: [10.1111/cgf.15182](https://doi.org/10.1111/cgf.15182).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> While the past of terrain cannot be known precisely because an effect can result from many different causes, exploring these possible pasts opens the way to numerous applications ranging from movies and games to paleogeography. We introduce unerosion, an attempt to recover plausible past topographies from an input terrain represented as a height field. Our solution relies on novel algorithms for the backward simulation of different processes: fluvial erosion, sedimentation, and thermal erosion. This is achieved by re‐formulating the equations of erosion and sedimentation so that they can be simulated back in time. These algorithms can be combined to account for a succession of climate changes backward in time, while the possible ambiguities provide editing options to the user. Results show that our solution can approximately reverse different types of erosion while enabling users to explore a variety of alternative pasts. Using a chronology of climatic periods to inform us about the main erosion phenomena, we also went back in time using real measured terrain data. We checked the consistency with geological findings, namely the height of river beds hundreds of thousands of years ago.


### `bornepons-2025-mesa`

Borne--Pons, Paul, Czerkawski, Mikolaj, Martin, Rosalie, Rouffet, Romain (2025). *MESA: Text-Driven Terrain Generation Using Latent Diffusion and Global Copernicus Data.* 2025 IEEE/CVF Conference on Computer Vision and Pattern Recognition Workshops (CVPRW):3058--3066. DOI: [10.1109/cvprw67362.2025.00289](https://doi.org/10.1109/cvprw67362.2025.00289).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Terrain modeling has traditionally relied on procedural techniques, which often require extensive domain expertise and handcrafted rules. In this paper, we present MESA - a novel data-centric alternative by training a diffusion model on global remote sensing data. This approach leverages large-scale geospatial information to generate high-quality terrain samples from text descriptions, showcasing a flexible and scalable solution for terrain generation. The model's capabilities are demonstrated through extensive experiments, highlighting its ability to generate realistic and diverse terrain landscapes. The dataset produced to support this work, the Major TOM Core-DEM extension dataset, is released openly as a comprehensive resource for global terrain data. The results suggest that data-driven models, trained on remote sensing data, can provide a powerful tool for realistic terrain modeling and generation.


### `goslin-2025-infinitediffusion`

Goslin, Alexander (2025). *InfiniteDiffusion: Bridging Learned Fidelity and Procedural Utility for Open-World Terrain Generation.* <https://www.semanticscholar.org/paper/19f256a5c98a30b718a9f6201546f329a90edb35>

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> For decades, procedural worlds have been built on procedural noise functions such as Perlin noise, which are fast and infinite, yet fundamentally limited in realism and large-scale coherence. Conversely, diffusion models offer unprecedented fidelity but remain generally confined to bounded canvases. We introduce InfiniteDiffusion, a training-free algorithm that reformulates diffusion sampling for lazy and unbounded generation, bridging the fidelity of diffusion models with the properties that made procedural noise indispensable: seamless infinite extent, seed-consistency, and constant-time random access. To demonstrate the utility of this approach, we present Terrain Diffusion, a framework for learned procedural terrain generation with a procedural noise-like interface. Our framework outpaces orbital velocity by 9 times on a consumer GPU, enabling realistic terrain generation at interactive rates. We integrate a hierarchical stack of diffusion models to couple planetary context with local detail, a compact Laplacian encoding to stabilize outputs across Earth-scale dynamic ranges, and an open-source infinite-tensor framework for constant-memory manipulation of unbounded tensors. Together, these components position diffusion models as a practical foundation for the next generation of infinite virtual worlds.


### `huo-2026-spatial`

Huo, Ziqiang (2026). *Spatial Interpolation of DEM With Stochastic Differential Equations-Based Diffusion Model.* IEEE Journal of Selected Topics in Applied Earth Observations and Remote Sensing 19:12480--12493. DOI: [10.1109/jstars.2026.3680214](https://doi.org/10.1109/jstars.2026.3680214).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Despite the availability of high-precision surveying technologies, digital elevation models (DEMs) may still contain missing or degraded elevation values due to complex acquisition conditions, terrain occlusions, imaging geometry effects, and subsequent data processing procedures. Spatial interpolation methods provide a technical means to recover complete DEM data from incomplete DEM data. However, traditional interpolation methods often face problems such as insufficient model expressiveness and limited interpolation accuracy. In this study, we innovatively introduce a diffusion model based on the mean-reverting stochastic differential equation (SDE) to the DEM spatial interpolation task and propose a novel data-driven interpolation framework. This method constructs a mean-reverting SDE to map high-quality DEM data to a degraded state with fixed noise, and uses the reverse-time SDE to achieve high-precision reconstruction of missing areas. We also specifically design a unique joint loss function for DEM interpolation and reconstruction, which includes content loss, perceptual loss and terrain feature loss. In addition, we propose a maximum likelihood estimation strategy to optimize the training objective, which significantly improves the training stability of the model and the reliability of the interpolation results. Experimental results show that, compared to traditional interpolation and typical deep learning-based reconstruction models, this method performs excellently in interpolating DEM data for complex terrains (such as steep slopes, gullies and ridges). In particular, it significantly reduces interpolation errors and produces terrain details that are more consistent with the actual terrain.


### `zhao-2026-structure-preserving`

Zhao, Ji (2026). *Structure-preserving DEM void filling via optical imagery-guided diffusion.* Remote Sensing of Environment 344:115515. DOI: [10.1016/j.rse.2026.115515](https://doi.org/10.1016/j.rse.2026.115515).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract:** ABSTRACT NOT RETRIEVED. Elsevier (Remote Sensing of Environment) deposits no abstract; Crossref, Semantic Scholar and OpenAlex all return null; ScienceDirect 403s.



---

## I. Early-Earth geology: continental emergence, cratons, tectonics, oxygenation

*5 entries.*

### `flament-2008-case`

Flament, Nicolas, Coltice, Nicolas, Rey, Patrice F. (2008). *A case for late-Archaean continental emergence from thermal evolution models and hypsometry.* Earth and Planetary Science Letters 275(3-4):326-336. DOI: [10.1016/j.epsl.2008.08.029](https://doi.org/10.1016/j.epsl.2008.08.029).

**Availability:** **in-repo PDF** — `ref/research/pdfs/flament-2008-case.pdf`; reading notes `ref/research/pdf-notes/flament-2008.md` · PDF in relata store

**Abstract** *(source: extracted from the PDF we hold)*

> The secular cooling of the Earth's mantle and the growth of the continental crust together imply changes in the isostatic balance between continents and oceans, in the oceanic bathymetry and in the area of emerged continental crust. The evolution of these variables is of fundamental importance to the geochemical coupling of mantle, continental crust, atmosphere and ocean. To explore this further, we developed a model that evaluates the area of emerged continental crust as a function of mantle temperature, continental area and hypsometry. In this paper, we investigate the continental freeboard predicted using different models for the cooling of the Earth. We show that constancy of the continental freeboard (±200 m) is possible throughout the history of the planet as long as the potential temperature of the upper mantle was never more than 110–210 °C hotter than present. Such numbers imply either a very limited cooling of the planet or, most likely, a change in continental freeboard since the Archaean. During the Archaean a greater radiogenic crustal heat production and a greater mantle heat flow would have reduced the strength of the continental lithosphere, thus limiting crustal thickening due to mountain building processes and the maximum elevation in the Earth's topography [Rey, P. F., Coltice, N., Neoarchean strengthening of the lithosphere and the coupling of the Earth's geochemical reservoirs, Geology 36, 635–638 (2008)]. Taking this into account, we show that the continents were mostly flooded until the end of the Archaean and that only 2–3% of the Earth's area consisted of emerged continental crust by around 2.5 Ga. These results are consistent with widespread Archaean submarine continental flood basalts, and with the appearance and strengthening of the geochemical fingerprint of felsic sources in the sedimentary record from ∼ 2.5 Ga. The progressive emergence of the continents as shown by our models from the late-Archaean onward had major implications for the Earth's environment, particularly by contributing to the rise of atmospheric oxygen and to the geochemical coupling between the Earth's deep and surface reservoirs.


### `chowdhury-2021-magmatic`

Chowdhury, Priyadarshi, Mulder, Jacob A., Cawood, Peter A., Bhattacharjee, Surjyendu, Roy, Subhajit, Wainwright, Ashlea N., et al. (2021). *Magmatic thickening of crust in non–plate tectonic settings initiated the subaerial rise of Earth's first continents 3.3 to 3.2 billion years ago.* Proceedings of the National Academy of Sciences 118(46). DOI: [10.1073/pnas.2105746118](https://doi.org/10.1073/pnas.2105746118).

**Availability:** **in-repo PDF** — `ref/research/pdfs/chowdhury-2021-magmatic.pdf`; reading notes `ref/research/pdf-notes/chowdhury-2021-magmatic.md` · PDF in relata store

**Abstract** *(source: relata record)*

> Significance Understanding when and how subaerial continental crust first formed is crucial, as it likely played a critical role in establishing Earth's habitability. Although debated, the broad consensus is that the subaerial rise of continents began ∼2.5 billion years ago and was driven by plate tectonics. Here, we integrate the igneous and sedimentary history of Archean cratons to demonstrate that stable continental landmasses started to emerge above sea level 3.3 to 3.2 billion years ago (i.e., over 700 million years earlier than most models predict). We also demonstrate that these initial episodes of continental emersion were driven by voluminous granitoid magmatism in non–plate tectonic settings that formed ∼50-km-thick, silica-rich crust, which rose above the oceans due to isostasy.


### `dong-2021-constraining`

Dong, Junjie, Fischer, Rebecca A., Stixrude, Lars P., Lithgow‐Bertelloni, Carolina R. (2021). *Constraining the Volume of Earth's Early Oceans With a Temperature‐Dependent Mantle Water Storage Capacity Model.* AGU Advances 2(1). DOI: [10.1029/2020av000323](https://doi.org/10.1029/2020av000323).

**Availability:** **in-repo PDF** — `ref/research/pdfs/dong-2021-constraining.pdf`; reading notes `ref/research/pdf-notes/dong-2021.md` · PDF in relata store

**Abstract** *(source: relata record)*

> Abstract The water content in Earth's mantle today remains poorly constrained, but the bulk water storage capacity in the solid mantle can be quantified based on experimental data and may amount to a few times the modern surface ocean mass (OM). An appreciation of the mantle water storage capacity is indispensable to our understanding of how water may have cycled between the surface and mantle reservoirs and changed the volume of the oceans through time. In this study, we parameterized high pressure‐temperature experimental data on water storage capacities in major rock‐forming minerals to track the bulk water storage capacity in Earth's solid mantle as a function of temperature. We find that the mantle water storage capacity decreases as mantle potential temperature ( T p ) increases, and its estimated value depends on the water storage capacity of bridgmanite in the lower mantle: 1.86–4.41 OM with a median of 2.29 OM for today ( T p = 1600 K), and 0.52–1.69 OM with a median of 0.72 OM for the early Earth's solid mantle (for a T p that was 300 K higher). An increase in T p by 200–300 K results in a decrease in the mantle water storage capacity by – OM. We explored how the volume of early oceans may have controlled sea level during the early Archean (4–3.2 Ga) with some additional assumptions about early continents. We found that more voluminous surface oceans might have existed if the actual mantle water content today is > 0.3–0.8 OM and the early Archean T p was ≥1900 K.


### `cawood-2022-secular`

Cawood, Peter A., Chowdhury, Priyadarshi, Mulder, Jacob A., Hawkesworth, Chris J., Capitanio, Fabio A., Gunawardana, Prasanna M., et al. (2022). *Secular Evolution of Continents and the Earth System.* Reviews of Geophysics 60(4). DOI: [10.1029/2022rg000789](https://doi.org/10.1029/2022rg000789).

**Availability:** **in-repo PDF** — `ref/research/pdfs/cawood-2022-secular.pdf`; reading notes `ref/research/pdf-notes/cawood-2022.md` · PDF in relata store

**Abstract** *(source: relata record)*

> Abstract Understanding of secular evolution of the Earth system is based largely on the rock and mineral archive preserved in the continental lithosphere. Based on the frequency and range of accessible data preserved in this record, we divide the secular evolution into seven phases: (a) " Proto‐Earth " (ca. 4.57–4.45 Ga); (b) " Primordial Earth " (ca. 4.45–3.80 Ga); (c) " Primitive Earth " (ca. 3.8–3.2 Ga); (d) "Juvenile Earth " (ca. 3.2–2.5 Ga); (e) " Youthful Earth " (ca. 2.5–1.8 Ga); (f) " Middle Earth " (ca. 1.8–0.8 Ga); and (g) " Contemporary Earth " (since ca. 0.8 Ga). Integrating this record with knowledge of secular cooling of the mantle and lithospheric rheology constrains the changes in the tectonic modes that operated through Earth history. Initial accretion and the Moon forming impact during the Proto‐Earth phase likely resulted in a magma ocean. The solidification of this magma ocean produced the Primordial Earth lithosphere, which preserves evidence for intra‐lithospheric reworking of a rigid lid, but which also likely experienced partial recycling through mantle overturn and meteorite impacts. Evidence for craton formation and stabilization from ca. 3.8 to 2.5 Ga, during the Primitive and Juvenile Earth phases, likely reflects some degree of coupling between the convecting mantle and a lithosphere initially weak enough to favor an internally deformable, squishy‐lid behavior, which led to a transition to more rigid, plate like, behavior by the end of the early Earth phases. The Youthful to Contemporary phases of Earth, all occurred within a plate tectonic framework with changes between phases linked to lithospheric behavior and the supercontinent cycle.


### `chowdhury-2025-subaerial`

Chowdhury, Priyadarshi, Cawood, Peter A., Mulder, Jacob A. (2025). *Subaerial Emergence of Continents on Archean Earth.* Annual Review of Earth and Planetary Sciences 53(1):443-478. DOI: [10.1146/annurev-earth-040722-093345](https://doi.org/10.1146/annurev-earth-040722-093345).

**Availability:** **in-repo PDF** — `ref/research/pdfs/annurev-earth-040722-093345.pdf`; reading notes `ref/research/pdf-notes/chowdhury-2025-review.md` · PDF in relata store

**Abstract** *(source: relata record)*

> The emergence of continental crust above sea level influences Earth's surface environments and climate patterns, and it creates diverse habitats that promote biodiversity. Earth exhibits bimodal hypsometry with elevated continents and a submerged seafloor. However, it remains elusive when and how this unique feature was first established. The geological record suggests the presence of subaerial landmasses between ca. 3.8 and 2.4 billion years ago (Ga), but their spatial extent and longevity remain unclear. Further, the tectonic processes governing the proportion of continental land to ocean basins and topography during this period are poorly understood. Here, we synthesize a variety of geological and geochemical proxies to suggest that crustal emergence did occur in the early-to-mid Archean, primarily exposing precratonized volcanic crust for brief time periods. Stable continental crust on a regional scale (as cratons) began emerging around ca. 3.2–3.0 Ga, facilitated by the development of thick, stable cratonic lithospheres. Over hundreds of millions of years, voluminous magmatism within a plateau-type setting led to the formation of thick, felsic crust and depleted mantle keels, allowing cratons to rise above sea level via isostatic adjustment. The areal extent of emergent land increased from ca. 3.0 to 2.5 Ga owing to the formation of more cratons, likely coinciding with the onset of plate tectonics, and culminated around ca. 2.5–2.2 Ga when land surface area and freeboard conditions resembled those observed today. These newly emerged landmasses possibly played a critical role in oxygenating the atmosphere and oceans, cooling the climate, and promoting biodiversity during the late Archean to early Paleoproterozoic. ▪ Continental emergence marks a pivotal moment in Earth's history, impacting the planet's atmosphere, oceans, climate, and life evolution. ▪ We review the rock record to infer the timing, nature, and tectonic drivers of continental emergence on early Earth. ▪ Emergence on early Archean Earth was mostly transient, exposing primarily volcanic crust. ▪ The first stable continental land formed at ca. 3.2–3.0 Ga due to the development of thick cratons and their isostatic adjustment. ▪ Emergent land area increased from ca. 3.0 to 2.5 Ga as more felsic crust formed and plate tectonics began.



---

## J. Celestial mechanics & long-term orbital forcing

*14 entries.*

### `wisdom-1991-symplectic`

Wisdom, Jack, Holman, Matthew (1991). *Symplectic maps for the n-body problem.* The Astronomical Journal 102:1528. DOI: [10.1086/115978](https://doi.org/10.1086/115978).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> The present study generalizes the mapping method of Wisdom (1982) to encompass all gravitational n-body problems with a dominant central mass. The rationale for the generalized mapping method is discussed as well as details for the mapping for the n-body problem. Some refinements of the method are considered, and the relationship of the mapping method to other symplectic integration methods is shown. The method is used to compute the evolution of the outer planets for a billion years. The resulting evolution is compared to the 845 million year evolution of the outer planets performed on the Digital Orerry using standard numerical integration techniques. This calculation provides independent numerical confirmation of the result of Sussman and Wisdom (1988) that the motion of the planet Pluto is chaotic.


### `touma-1994-liepoisson`

Touma, J., Wisdom, J. (1994). *Lie-Poisson integrators for rigid body dynamics in the solar system.* The Astronomical Journal 107:1189. DOI: [10.1086/116931](https://doi.org/10.1086/116931).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> The n-body mapping method of Wisdom & Holman (1991) is generalized to encompass rotational dynamics. The Lie-Poisson structure of rigid body dynamics is discussed. Integrators which preserve that structure are derived for the motion of a free rigid body and for the motion of rigid bodies interacting gravitationally with mass points.


### `chambers-1999-hybrid`

Chambers, J. E. (1999). *A hybrid symplectic integrator that permits close encounters between massive bodies.* Monthly Notices of the Royal Astronomical Society 304(4):793--799. DOI: [10.1046/j.1365-8711.1999.02379.x](https://doi.org/10.1046/j.1365-8711.1999.02379.x).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> Mixed-variable symplectic integrators exhibit no long-term accumulation of energy error, beyond that owing to round-off, and they are substantially faster than conventional N-body algorithms. This makes them the integrator of choice for many problems in Solar system astronomy. However, in their original formulation, they become inaccurate whenever two bodies approach one another closely. This occurs because the potential energy term for the pair undergoing the encounter becomes comparable to the terms representing the unperturbed motion in the Hamiltonian. The problem can be overcome using a hybrid method, in which the close encounter term is integrated using a conventional integrator, whilst the remaining terms are solved symplectically. In addition, using a simple separable potential technique, the hybrid scheme can be made symplectic even though it incorporates a non-symplectic component.


### `preto-1999-adaptive-symplectic`

Preto, Miguel, Tremaine, Scott (1999). *A Class of Symplectic Integrators with Adaptive Time Step for Separable Hamiltonian Systems.* The Astronomical Journal 118(5):2532--2541. DOI: [10.1086/301102](https://doi.org/10.1086/301102).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> Symplectic integration algorithms are well suited for long-term integrations of Hamiltonian systems, because they preserve the geometric structure of the Hamiltonian flow. However, this desirable property is generally lost when adaptive time step control is added to a symplectic integrator. We describe an adaptive time step, symplectic integrator that can be used if the Hamiltonian is the sum of kinetic and potential energy components and the required time step depends only on the potential energy (e.g., test-particle integrations in fixed potentials). In particular, we describe an explicit, reversible, symplectic, leapfrog integrator for a test particle in a near-Keplerian potential; this integrator has a time step proportional to distance from the attracting mass and has the remarkable property of integrating orbits in an inverse-square force field with only "along-track" errors; i.e., the phase-space shape of a Keplerian orbit is reproduced exactly, but the orbital period is in error by O(N-2), where N is the number of steps per period.


### `rauch-1999-dynamical`

Rauch, Kevin P., Holman, Matthew (1999). *Dynamical Chaos in the Wisdom-Holman Integrator: Origins and Solutions.* The Astronomical Journal 117(2):1087--1102. DOI: [10.1086/300720](https://doi.org/10.1086/300720).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> We examine the nonlinear stability of the Wisdom-Holman (WH) symplectic mapping applied to the integration of perturbed, highly eccentric (e ≳ 0.9) two-body orbits. We find that the method is unstable and introduces artificial chaos into the computed trajectories for this class of problems, unless the step size chosen is small enough that periapse is always resolved, in which case the method is generically stable. This "radial orbit instability" persists even for weakly perturbed systems. Using the Stark problem as a fiducial test case, we investigate the dynamical origin of this instability and argue that the numerical chaos results from the overlap of step-size resonances; interestingly, for the Stark problem many of these resonances appear to be absolutely stable. We similarly examine the robustness of several alternative integration methods: a time-regularized version of the WH mapping suggested by Mikkola; the potential-splitting (PS) method of Duncan, Levison, & Lee; and two original methods incorporating approximations based on Stark motion instead of Keplerian motion (compare Newman et al.). The two fixed point problem and a related, more general problem are used to conduct a comparative test of the various methods for several types of motion. Among the algorithms tested, the time-transformed WH mapping is clearly the most efficient and stable method of integrating eccentric, nearly Keplerian orbits in the absence of close encounters. For test particles subject to both high eccentricities and very close encounters, we find an enhanced version of the PS method—incorporating time regularization, force-center switching, and an improved kernel function—to be both economical and highly versatile. We conclude that Stark-based methods are of marginal utility in N-body type integrations. Additional implications for the symplectic integration of N-body systems are discussed.


### `laskar-2001-saba`

Laskar, Jacques, Robutel, Philippe (2001). *High order symplectic integrators for perturbed Hamiltonian systems.* Celestial Mechanics and Dynamical Astronomy 80(1):39--62. DOI: [10.1023/a:1012098603882](https://doi.org/10.1023/a:1012098603882).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> A family of symplectic integrators adapted for the integration of perturbed Hamiltonian systems of the form H = A + εB was given in (McLachlan, 1995). We give here a constructive proof that for all integer p, such integrator exists, with only positive steps, and with a remainder of order O(τpε + τ2ε2), where τ is the stepsize of the integrator. Moreover, we compute the analytical expressions of the leading terms of the remainders at all orders. We show also that for a large class of systems, a corrector step can be performed such that the remainder becomes O(τpε +τ4ε2). The performances of these integrators are compared for the simple pendulum and the planetary three-body problem of Sun–Jupiter–Saturn.


### `laskar-2004-la2004`

Laskar, J., Robutel, P., Joutel, F., Gastineau, M., Correia, A. C. M., Levrard, B. (2004). *A long-term numerical solution for the insolation quantities of the Earth.* Astronomy & Astrophysics 428(1):261--285. DOI: [10.1051/0004-6361:20041335](https://doi.org/10.1051/0004-6361:20041335).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: OpenAlex)*

> We present here a new solution for the astronomical computation of the insolation quantities on Earth spanning from -250 Myr to 250 Myr. This solution has been improved with respect to La93 (Laskar et al. [CITE]) by using a direct integration of the gravitational equations for the orbital motion, and by improving the dissipative contributions, in particular in the evolution of the Earth–Moon System. The orbital solution has been used for the calibration of the Neogene period (Lourens et al. [CITE]), and is expected to be used for age calibrations of paleoclimatic data over 40 to 50 Myr, eventually over the full Palaeogene period (65 Myr) with caution. Beyond this time span, the chaotic evolution of the orbits prevents a precise determination of the Earth's motion. However, the most regular components of the orbital solution could still be used over a much longer time span, which is why we provide here the solution over 250 Myr. Over this time interval, the most striking feature of the obliquity solution, apart from a secular global increase due to tidal dissipation, is a strong decrease of about 0.38 degree in the next few millions of years, due to the crossing of the resonance (Laskar et al. [CITE]). For the calibration of the Mesozoic time scale (about 65 to 250 Myr), we propose to use the term of largest amplitude in the eccentricity, related to , with a fixed frequency of /yr, corresponding to a period of 405 000 yr. The uncertainty of this time scale over 100 Myr should be about , and over the full Mesozoic era.


### `laskar-2009-collisional`

Laskar, J., Gastineau, M. (2009). *Existence of collisional trajectories of Mercury, Mars and Venus with the Earth.* Nature 459(7248):817--819. DOI: [10.1038/nature08096](https://doi.org/10.1038/nature08096).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: publisher-deposited author abstract (PubMed, PMID 19516336))*

> It has been established that, owing to the proximity of a resonance with Jupiter, Mercury's eccentricity can be pumped to values large enough to allow collision with Venus within 5 Gyr. This conclusion, however, was established either with averaged equations that are not appropriate near the collisions or with non-relativistic models in which the resonance effect is greatly enhanced by a decrease of the perihelion velocity of Mercury. In these previous studies, the Earth's orbit was essentially unaffected. Here we report numerical simulations of the evolution of the Solar System over 5 Gyr, including contributions from the Moon and general relativity. In a set of 2,501 orbits with initial conditions that are in agreement with our present knowledge of the parameters of the Solar System, we found, as in previous studies, that one per cent of the solutions lead to a large increase in Mercury's eccentricity - an increase large enough to allow collisions with Venus or the Sun. More surprisingly, in one of these high-eccentricity solutions, a subsequent decrease in Mercury's eccentricity induces a transfer of angular momentum from the giant planets that destabilizes all the terrestrial planets approximately 3.34 Gyr from now, with possible collisions of Mercury, Mars or Venus with the Earth.


### `lissauer-2012-obliquity`

Lissauer, Jack J., Barnes, Jason W., Chambers, John E. (2012). *Obliquity variations of a moonless Earth.* Icarus 217(1):77--87. DOI: [10.1016/j.icarus.2011.10.013](https://doi.org/10.1016/j.icarus.2011.10.013).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: NASA ADS (bibcode 2012Icar..217...77L))*

> We numerically explore the obliquity (axial tilt) variations of a hypothetical moonless Earth. Previous work has shown that the Earth's Moon stabilizes Earth's obliquity such that it remains within a narrow range, between 22.1 deg and 24.5 deg. Without lunar influence, a frequency map analysis by Laskar et al. (Laskar, J., Joutel, F., Robutel, P. [1993]. Nature 361, 615-617) showed that the obliquity could vary between 0 deg and 85 deg. This has left an impression in the astrobiology community that a big moon is necessary to maintain a habitable climate on an Earth-like planet. Using a modified version of the orbital integrator mercury, we calculate the obliquity evolution for moonless Earths with various initial conditions for up to 4 Gyr. We find that while obliquity varies significantly more than that of the actual Earth over 100,000 year timescales, the obliquity remains within a constrained range, typically 20-25 deg in extent, for timescales of hundreds of millions of years. None of our Solar System integrations in which planetary orbits behave in a typical manner show obliquity accessing more than 65% of the full range allowed by frequency-map analysis. The obliquities of moonless Earths that rotate in the retrograde direction are more stable than those of prograde rotators. The total obliquity range explored for moonless Earths with rotation periods less than 12 h is much less than that for slower-rotating moonless Earths. A large moon thus does not seem to be needed to stabilize the obliquity of an Earth-like planet on timescales relevant to the development of advanced life.


### `rein-2015-ias15`

Rein, Hanno, Spiegel, David S. (2014). *ias15: a fast, adaptive, high-order integrator for gravitational dynamics, accurate to machine precision over a billion orbits.* Monthly Notices of the Royal Astronomical Society 446(2):1424--1437. DOI: [10.1093/mnras/stu2164](https://doi.org/10.1093/mnras/stu2164).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> We present IAS15, a 15th-order integrator to simulate gravitational dynamics. The integrator is based on a Gaus-Radau quadrature and can handle conservative as well as non-conservative forces. We develop a step-size control that can automatically choose an optimal timestep. The algorithm can handle close encounters and high-eccentricity orbits. The systematic errors are kept well below machine precision and long-term orbit integrations over 10 9 orbits show that IAS15 is optimal in the sense that it follows Brouwer's law, i.e. the energy error behaves like a random walk. Our tests show that IAS15 is superior to a mixed-variable symplectic integrator (MVS) and other high-order integrators in both speed and accuracy. In fact, IAS15 preserves the symplecticity of Hamiltonian systems better than the commonly-used nominally symplectic integrators to which we compared it. We provide an open-source implementation of IAS15. The package comes with several easy-to-extend examples involving resonant planetary systems, Kozai-Lidov cycles, close encounters, radiation pressure, quadrupole moment, and generic damping functions that can, among other things, be used to simulate planet-disc interactions. Other non-conservative forces can be added easily.


### `rein-2015-whfast`

Rein, Hanno, Tamayo, Daniel (2015). *whfast: a fast and unbiased implementation of a symplectic Wisdom–Holman integrator for long-term gravitational simulations.* Monthly Notices of the Royal Astronomical Society 452(1):376--388. DOI: [10.1093/mnras/stv1257](https://doi.org/10.1093/mnras/stv1257).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Semantic Scholar)*

> We present WHFast, a fast and accurate implementation of a Wisdom-Holman symplectic integrator for long-term orbit integrations of planetary systems. WHFast is significantly faster and conserves energy better than all other Wisdom-Holman integrators tested. We achieve this by significantly improving the Kepler-solver and ensuring numerical stability of coordinate transformations to and from Jacobi coordinates. These refinements allow us to remove the linear secular trend in the energy error that is present in other implementations. For small enough timesteps we achieve Brouwer's law, i.e. the energy error is dominated by an unbiased random walk due to floating-point round-off errors. We implement symplectic correctors up to order eleven that significantly reduce the energy error. We also implement a symplectic tangent map for the variational equations. This allows us to efficiently calculate two widely used chaos indicators the Lyapunov characteristic number (LCN) and the Mean Exponential Growth factor of Nearby Orbits (MEGNO). WHFast is freely available as a flexible C package, as a shared library, and as an easy-to-use python module.


### `zeebe-2017-numerical`

Zeebe, Richard E. (2017). *Numerical Solutions for the Orbital Motion of the Solar System over the Past 100 Myr: Limits and New Results*.* The Astronomical Journal 154(5):193. DOI: [10.3847/1538-3881/aa8cce](https://doi.org/10.3847/1538-3881/aa8cce).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> I report results from accurate numerical integrations of solar system orbits over the past 100 Myr with the integrator package HNBody. The simulations used different integrator algorithms, step sizes, and initial conditions, and included effects from general relativity, different models of the Moon, the Sun's quadrupole moment, and up to 16 asteroids. I also probed the potential effect of a hypothetical Planet 9, using one set of possible orbital elements. The most expensive integration (Bulirsch–Stoer) required 4 months of wall-clock time with a maximum relative energy error . The difference in Earth's eccentricity ( ) was used to track the difference between two solutions, considered to diverge at time τ when max irreversibly crossed ∼10% of mean ( ). The results indicate that finding a unique orbital solution is limited by initial conditions from current ephemerides and asteroid perturbations to ∼54 Myr. Bizarrely, the 4-month Bulirsch–Stoer integration and a symplectic integration that required only 5 hr of wall-clock time (12-day time step, with the Moon as a simple quadrupole perturbation), agree to ∼63 Myr. Internally, such symplectic integrations are remarkably consistent even for large time steps, suggesting that the relationship between time step and τ is not a robust indicator of the absolute accuracy of symplectic integrations. The effect of a hypothetical Planet 9 on becomes discernible at ∼65 Myr. Using τ as a criterion, the current state-of-the-art solutions all differ from previously published results beyond ∼50 Myr. I also conducted an eigenmode analysis, which provides some insight into the chaotic nature of the inner solar system. The current study provides new orbital solutions for applications in geological studies.


### `rein-2019-hybrid`

Rein, Hanno, Hernandez, David M, Tamayo, Daniel, Brown, Garett, Eckels, Emily, Holmes, Emma, et al. (2019). *Hybrid symplectic integrators for planetary dynamics.* Monthly Notices of the Royal Astronomical Society 485(4):5490--5497. DOI: [10.1093/mnras/stz769](https://doi.org/10.1093/mnras/stz769).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> Hybrid symplectic integrators such as MERCURY are widely used to simulate complex dynamical phenomena in planetary dynamics that could otherwise not be investigated. A hybrid integrator achieves high accuracy during close encounters by using a high-order integration scheme for the duration of the encounter while otherwise using a standard second-order Wisdom–Holman scheme, thereby optimizing both speed and accuracy. In this paper we reassess the criteria for choosing the switching function that determines which parts of the Hamiltonian are integrated with the high-order integrator. We show that the original motivation for choosing a polynomial switching function in MERCURY is not correct. We explain the nevertheless excellent performance of the MERCURY integrator and then explore a wide range of different switching functions including an infinitely differentiable function and a Heaviside function. We find that using a Heaviside function leads to a significantly simpler scheme compared to MERCURY , while maintaining the same accuracy in short-term simulations.


### `tamayo-2020-reboundx`

Tamayo, Daniel, Rein, Hanno, Shi, Pengshuai, Hernandez, David M (2019). *REBOUNDx: a library for adding conservative and dissipative forces to otherwise symplectic N-body integrations.* Monthly Notices of the Royal Astronomical Society 491(2):2885--2901. DOI: [10.1093/mnras/stz2870](https://doi.org/10.1093/mnras/stz2870).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> ABSTRACT Symplectic methods, in particular the Wisdom–Holman map, have revolutionized our ability to model the long-term, conservative dynamics of planetary systems. However, many astrophysically important effects are dissipative. The consequences of incorporating such forces into otherwise symplectic schemes are not always clear. We show that moving to a general framework of non-commutative operators (dissipative or not) clarifies many of these questions, and that several important properties of symplectic schemes carry over to the general case. In particular, we show that explicit splitting schemes generically exploit symmetries in the applied external forces, which often strongly suppress integration errors. Furthermore, we demonstrate that so-called 'symplectic correctors' (which reduce energy errors by orders of magnitude at fixed computational cost) apply equally well to weakly dissipative systems and can thus be more generally thought of as 'weak splitting correctors'. Finally, we show that previously advocated approaches of incorporating additional forces into symplectic methods work well for dissipative forces, but give qualitatively wrong answers for conservative but velocity-dependent forces like post-Newtonian corrections. We release REBOUNDx, an open-source C library for incorporating additional effects into REBOUNDN-body integrations, together with a convenient python wrapper. All effects are machine independent and we provide a binary format that interfaces with the SimulationArchive class in REBOUND to enable the sharing and reproducibility of results. Users can add effects from a list of pre-implemented astrophysical forces, or contribute new ones.



---

## K. Simulation architecture: incremental computation, determinism, exact arithmetic

*8 entries.*

### `leiserson-2012-deterministic`

Leiserson, C., Schardl, T., Sukha, Jim (2012). *Deterministic parallel random-number generation for dynamic-multithreading platforms.* Proceedings of the 17th ACM SIGPLAN symposium on Principles and Practice of Parallel Programming:193--204. DOI: [10.1145/2145816.2145841](https://doi.org/10.1145/2145816.2145841).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Existing concurrency platforms for dynamic multithreading do not provide repeatable parallel random-number generators. This paper proposes that a mechanism called pedigrees be built into the runtime system to enable efficient deterministic parallel random-number generation. Experiments with the open-source MIT Cilk runtime system show that the overhead for maintaining pedigrees is negligible. Specifically, on a suite of 10 benchmarks, the relative overhead of Cilk with pedigrees to the original Cilk has a geometric mean of less than 1%. We persuaded Intel to modify its commercial C/C++ compiler, which provides the Cilk Plus concurrency platform, to include pedigrees, and we built a library implementation of a deterministic parallel random-number generator called DotMix that compresses the pedigree and then "RC6-mixes" the result. The statistical quality of DotMix is comparable to that of the popular Mersenne twister, but somewhat slower than a nondeterministic parallel version of this efficient and high-quality serial random-number generator. The cost of calling DotMix depends on the "spawn depth" of the invocation. For a naive Fibonacci calculation with n=40 that calls DotMix in every node of the computation, this "price of determinism" is a factor of 2.65 in running time, but for more realistic applications with less intense use of random numbers -- such as a maximal-independent-set algorithm, a practical samplesort program, and a Monte Carlo discrete-hedging application from QuantLib -- the observed "price" was less than 5%. Moreover, even if overheads were several times greater, applications using DotMix should be amply fast for debugging purposes, which is a major reason for desiring repeatability.


### `steele-2014-fast`

Steele, G., Lea, D., Flood, Christine H. (2014). *Fast splittable pseudorandom number generators.* ACM SIGPLAN Notices 49(10):453--472. DOI: [10.1145/2714064.2660195](https://doi.org/10.1145/2714064.2660195).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: Crossref)*

> We describe a new algorithm SplitMix for an object-oriented and splittable pseudorandom number generator (PRNG) that is quite fast: 9 64-bit arithmetic/logical operations per 64 bits generated. A conventional linear PRNG object provides a generate method that returns one pseudorandom value and updates the state of the PRNG, but a splittable PRNG object also has a second operation, split , that replaces the original PRNG object with two (seemingly) independent PRNG objects, by creating and returning a new such object and updating the state of the original object. Splittable PRNG objects make it easy to organize the use of pseudorandom numbers in multithreaded programs structured using fork-join parallelism. No locking or synchronization is required (other than the usual memory fence immediately after object creation). Because the generate method has no loops or conditionals, it is suitable for SIMD or GPU implementation. We derive SplitMix from the DotMix algorithm of Leiserson, Schardl, and Sukha by making a series of program transformations and engineering improvements. The end result is an object-oriented version of the purely functional API used in the Haskell library for over a decade, but SplitMix is faster and produces pseudorandom sequences of higher quality; it is also far superior in quality and speed to java.util.Random, and has been included in Java JDK8 as the class java.util.SplittableRandom. We have tested the pseudorandom sequences produced by SplitMix using two standard statistical test suites (DieHarder and TestU01) and they appear to be adequate for "everyday" use, such as in Monte Carlo algorithms and randomized data structures where speed is important.


### `gustafson-2017-beating`

Gustafson, John L., Yonemoto, Isaac T. (2017). *Beating Floating Point at its Own Game: Posit Arithmetic.* Supercomputing Frontiers and Innovations 4(2):71--86. DOI: [10.14529/jsfi170206](https://doi.org/10.14529/jsfi170206).

**Availability:** PDF in relata store

**Abstract** *(source: Semantic Scholar)*

> A new data type called a posit is designed as a direct drop-in replacement for IEEE Standard 754 floating-point numbers (floats). Unlike earlier forms of universal number (unum) arithmetic, posits do not require interval arithmetic or variable size operands; like floats, they round if an answer is inexact. However, they provide compelling advantages over floats, including larger dynamic range, higher accuracy, better closure, bitwise identical results across systems, simpler hardware, and simpler exception handling. Posits never overflow to infinity or underflow to zero, and "Not-a-Number" (NaN) indicates an action instead of a bit pattern. A posit processing unit takes less circuitry than an IEEE float FPU. With lower power use and smaller silicon footprint, the posit operations per second (POPS) supported by a chip can be significantly higher than the FLOPS using similar hardware resources. GPU accelerators and Deep Learning processors, in particular, can do more per watt and per dollar with posits, yet deliver superior answer quality. A comprehensive series of benchmarks compares floats and posits for decimals of accuracy produced for a set precision. Low precision posits provide a better solution than "approximate computing" methods that try to tolerate decreased answer quality. High precision posits provide more correct decimals than floats of the same size; in some cases, a 32-bit posit may safely replace a 64-bit float . In other words, posits beat floats at their own game.


### `mokhov-2018-build`

Mokhov, Andrey, Mitchell, Neil, Peyton Jones, Simon (2018). *Build Systems \`a la Carte.* Proceedings of the ACM on Programming Languages 2(ICFP):79:1--79:29. DOI: [10.1145/3236774](https://doi.org/10.1145/3236774).

**Availability:** PDF in relata store

**Abstract** *(source: Crossref)*

> Build systems are awesome, terrifying -- and unloved. They are used by every developer around the world, but are rarely the object of study. In this paper we offer a systematic, and executable, framework for developing and comparing build systems, viewing them as related points in landscape rather than as isolated phenomena. By teasing apart existing build systems, we can recombine their components, allowing us to prototype new build systems with desired properties.


### `hanai-2019-exact-differential`

Hanai, Masatoshi, Suzumura, T., Liu, Elvis S., Theodoropoulos, G., Perumalla, K. (2019). *Exact-Differential Simulation.* ACM Transactions on Modeling and Computer Simulation 29(3):1--25. DOI: [10.1145/3301499](https://doi.org/10.1145/3301499).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Using computer simulation to analyze large-scale discrete event systems requires repeated executions with various scenarios or parameters. Such repeated executions can induce significant redundancy in event processing when the modification from a prior scenario to a new scenario is relatively minor, and when the altered scenario influences only a small part of the simulation. For example, in a city-scale traffic simulation, an altered scenario of blocking one junction may only affect a small part of the city for considerable length of time. However, traditional simulation approaches would still repeat the simulation for the whole city even when the changes are minor. In this article, we propose a new redundancy reduction technique for large-scale discrete event simulations, called exact-differential simulation, which simulates only the altered portions of scenarios and their influences in repeated executions while still achieving the same results as the re-execution of entire simulations. This article presents the main concepts of the exact-differential simulation, the design of its algorithm, and an approach to build an exact-differential simulation middleware that supports multiple applications of discrete event simulation. We also evaluate our approach by using two case studies, PHOLD benchmark and a traffic simulation of Tokyo.


### `marotta-2024-out-of-order`

Marotta, Romolo, Quaglia, Francesco (2024). *Out-of-Order Discrete Event Simulation: Fighting Memory Boundedness while Running DES Models.* 2024 28th International Symposium on Distributed Simulation and Real Time Applications (DS-RT):110--119. DOI: [10.1109/ds-rt62209.2024.00024](https://doi.org/10.1109/ds-rt62209.2024.00024).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> In this article we present Out-of-order Discrete Event Simulation (ODES), a solution for sequential style execution of DES models not following timestamp order. ODES ensures anyway the same identical simulation results as timestamp ordered execution, thanks to fully correct maintenance of the simulation model data flow. At the same time, it drastically reduces the memory boundedness—namely, the impact of cache misses and of stale CPU cycles—on the simulation model execution speed. Beyond presenting foundational concepts, we also discuss our ODES-engine implementation, based on the c programming language. Additionally, we report experimental data for comparing ODES with the classical timestamp ordered execution of simulation models according to conventional sequential simulation. The relevance of ODES compared to classical timestamp ordered sequential DES not only stands in its benefits on performance, rather ODES can also assume the role of new reference for determining the speedup achievable via parallel/distributed discrete event simulation systems, compared to the single thread execution. Also, thanks to its improvements in the interaction with RAM, ODES constitutes a new framework for effective parallel replication of simulation experiments on multi-processor/multi-core machines.


### `yoginath-2024-specsims`

Yoginath, Srikanth B., Shukla, Pratishtha, Nutaro, James J., Seal, Sudip K. (2024). *SpecSims: A Scalable Speculative Tree-based Simulation Cloning Framework for Finite Memory Machines.* ACM Transactions on Modeling and Computer Simulation 35(3):1--21. DOI: [10.1145/3708885](https://doi.org/10.1145/3708885).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Simulation cloning is a technique in which cloned simulations whose state spaces differ partially from their parent simulation due to intervening events are spawned at runtime and concurrently advanced. It is a powerful method to carry out what-if analysis by speculatively exploring and evaluating the impact of various permutations of intervening cascade of events. Due to the exponential growth in the number of possible clones even for a small number of distinct intervening events, the practical efficacy of the approach is often severely limited by the maximum available memory of the computing host. In this paper, we introduce a novel speculative simulation cloning framework that executes a simulation cloning campaign capable of efficiently exploring an exponentially large space of clone simulations created by permutation of intervening events under a finite memory constraint. We provide a theoretical analysis of the runtime characteristics of our proposed approach and highlight its novel advantages such as memory-aware and as-long-as-needed execution. In support of our analytical findings and to demonstrate its practical feasibility, we implement a prototype of the cloning framework on a shared memory system and report its performance characteristics in the context of a heat diffusion simulation, and a power grid simulation subject to cascading disruptions from geomagnetic disturbances.


### `jensen-2025-order`

Jensen, Erik J., Leathrum, James F., Lynch, Christopher J., Smith, Katherine, Gore, Ross J. (2025). *Out of Order and Causally Correct: Ready-Event Discovery through Data-Dependence Analysis.* Proceedings of the 39th ACM SIGSIM Conference on Principles of Advanced Discrete Simulation:88-98. DOI: [10.1145/3726301.3728416](https://doi.org/10.1145/3726301.3728416).

**Availability:** **in relata — metadata only, no PDF held**

**Abstract** *(source: relata record)*

> Data-dependence analysis can identify causally-unordered events in a pending event set. The execution of these events is independent from all other scheduled events, making them ready for execution. These events can be executed out of order or in parallel. This approach may find and utilize more parallelism than spatial-decomposition parallelization methods, which are limited by the number of subdomains and by synchronization methods. This work provides formal definitions that use data-dependence analysis to find causally-unordered events and uses these definitions to measure parallelism in several discrete-event simulation models. A variant of the event-graph formalism is proposed, which assists with identifying ready events, by more clearly visualizing data dependencies between event types. Data dependencies between two event types may be direct or indirect, where the latter case considers the scheduling of intermediate events. Data dependencies and scheduling dependencies in a discrete-event simulation model are used to define time-interval limits that support the identification of events that are ready for execution. Experimental results from serial simulation testing demonstrate the availability of numerous events that are ready for execution, depending on model characteristics. The mean size of the ready-event set varies from about 1.5 to 110 for the tested models, depending on the model type, the size of the model, and delay distribution parameters. These findings support future work to develop a parallel capability to dynamically identify and execute ready events in a multi-threaded environment.


---

# Part II — State of the art and tangential work (web search)

*Work surfaced by web search and verified against a primary record. **Every DOI in this Part was independently re-resolved against the Crossref API** after the searches returned it; where Crossref disagreed with the search result on volume/issue/year, Crossref's values are used. Most of these are works the project holds nothing for — that is the point of this Part.*

*Availability for everything in Part II is **not held anywhere** unless the entry says otherwise.*

---

## II-A. Structure-preserving / compatible / mimetic discretisation

### Arnold, Falk & Winther 2006 — FEEC, the canonical anchor

**Arnold, D. N., Falk, R. S. & Winther, R. (2006).** *Finite element exterior calculus, homological techniques, and applications.* Acta Numerica 15:1–155. DOI: [10.1017/S0962492906210018](https://doi.org/10.1017/S0962492906210018).

**Availability:** in relata as `arnold-2006-feec` (metadata only) — see Part I §A.

> Finite element exterior calculus is an approach to the design and understanding of finite element discretizations for a wide variety of systems of partial differential equations. This approach brings to bear tools from differential geometry, algebraic topology, and homological algebra to develop discretizations which are compatible with the geometric, topological, and algebraic structures which underlie well-posedness of the PDE problem being solved. In the finite element exterior calculus, many finite element spaces are revealed as spaces of piecewise polynomial differential forms. These connect to each other in discrete subcomplexes of elliptic differential complexes, and are also related to the continuous elliptic complex through projections which commute with the complex differential. Applications are made to the finite element discretization of a variety of problems, including the Hodge Laplacian, Maxwell's equations, the equations of elasticity, and elliptic eigenvalue problems, and also to preconditioners.

---

### Cotter 2023 — compatible finite elements for geophysical fluid dynamics

**Cotter, C. J. (2023).** *Compatible finite element methods for geophysical fluid dynamics.* Acta Numerica 32:291–393. DOI: [10.1017/S0962492923000028](https://doi.org/10.1017/S0962492923000028). Preprint: arXiv:2302.13337.

> This article surveys research on the application of compatible finite element methods to large scale atmosphere and ocean simulation. Compatible finite element methods extend Arakawa's C-grid finite difference scheme to the finite element world. They are constructed from a discrete de Rham complex, which is a sequence of finite element spaces which are linked by the operators of differential calculus. The use of discrete de Rham complexes to solve partial differential equations is well established, but in this article we focus on the specifics of dynamical cores for simulating weather, oceans and climate. The most important consequence of the discrete de Rham complex is the Hodge-Helmholtz decomposition, which has been used to exclude the possibility of several types of spurious oscillations from linear equations of geophysical flow. This means that compatible finite element spaces provide a useful framework for building dynamical cores. In this article we introduce the main concepts of compatible finite element spaces, and discuss their wave propagation properties. We survey some methods for discretising the transport terms that arise in dynamical core equation systems, and provide some example discretisations, briefly discussing their iterative solution. Then we focus on the recent use of compatible finite element spaces in designing structure preserving methods, surveying variational discretisations, Poisson bracket discretisations, and consistent vorticity transport.

---

### Pitassi et al. 2023 — MFD on grids with curved faces

**Pitassi, S., Ghiloni, R., Petretti, I., Trevisan, F. & Specogna, R. (2023).** *The curved mimetic finite difference method: allowing grids with curved faces.* Journal of Computational Physics 491:112294. DOI: [10.1016/j.jcp.2023.112294](https://doi.org/10.1016/j.jcp.2023.112294). Preprint: arXiv:2203.13105.

> We present a new mimetic finite difference method for diffusion problems that converges on grids with curved (i.e., non-planar) faces. Crucially, it gives a symmetric discrete problem that uses only one discrete unknown per curved face. The principle at the core of our construction is to abandon the standard definition of local consistency of mimetic finite difference methods. Instead, we exploit the novel and global concept of P₀-consistency. Numerical examples confirm the consistency and the optimal convergence rate of the proposed mimetic method for cubic grids with randomly perturbed nodes as well as grids with curved boundaries.

---

### Mantravadi, Jagad & Samtaney 2022 — DEC on spherical shells

**Mantravadi, B., Jagad, P. & Samtaney, R. (2022).** *A hybrid discrete exterior calculus and finite difference method for Boussinesq convection in spherical shells.* arXiv:2210.00861 [physics.flu-dyn].

> We present a new hybrid discrete exterior calculus (DEC) and finite difference (FD) method to simulate fully three-dimensional Boussinesq convection in spherical shells subject to internal heating and basal heating, relevant in the planetary and stellar phenomenon. We employ DEC to compute the surface spherical flows, taking advantage of its unique features of structure preservation (e.g., conservation of secondary quantities like kinetic energy) and coordinate system independence, while we discretize the radial direction using FD method. The grid employed for this novel method is free of problems like the coordinate singularity, grid non-convergence near the poles, and the overlap regions. We have developed a parallel in-house code using the PETSc framework to verify the hybrid DEC-FD formulation and demonstrate convergence. We have performed a series of numerical tests which include quantification of the critical Rayleigh numbers for spherical shells characterized by aspect ratios ranging from 0.2 to 0.8, simulation of robust convective patterns in addition to stationary giant spiral roll covering all the spherical surface in moderately thin shells near the weakly nonlinear regime, and the quantification of Nusselt and Reynolds numbers for basally heated spherical shells. The method exhibits slightly better than second order error convergence with the mesh size.

---

### Luesink et al. 2024 — structure-preserving quasi-geostrophy on the sphere

**Luesink, E., Franken, A., Ephrati, S. & Geurts, B. (2024).** *Geometric derivation and structure-preserving simulation of quasi-geostrophy on the sphere.* arXiv:2402.13707 [physics.flu-dyn].

> We present a geometric derivation of the quasi-geostrophic equations on the sphere, starting from the rotating shallow water equations. We utilise perturbation series methods in vorticity and divergence variables. The derivation employs asymptotic analysis techniques, leading to a global quasi-geostrophic potential vorticity model on the sphere without approximation of the Coriolis parameter. The resulting model forms a closed system for the evolution of potential vorticity with a rich mathematical structure, including Lagrangian and Hamiltonian descriptions. Formulated using the Lie-Poisson bracket reveals the geometric invariants of the quasi-geostrophic model. Motivated by these geometric results, simulations of quasi-geostrophic flow on the sphere are presented based on structure-preserving Lie-Poisson time-integration. We explicitly demonstrate the preservation of Casimir invariants and show that the hyperbolic quasi-geostrophic equations can be simulated in a stable manner over long time. We show the emergence of longitudonal jets, wrapped around the circumference of the sphere in a general direction that is perpendicular to the axis of rotation.

---

### Lee 2021 — energy-balanced Hamiltonian integrator on the sphere

**Lee, D. (2021).** *An energetically balanced, quasi-Newton integrator for non-hydrostatic vertical atmospheric dynamics.* Journal of Computational Physics 429:109988. DOI: [10.1016/j.jcp.2020.109988](https://doi.org/10.1016/j.jcp.2020.109988). Preprint: arXiv:2004.14090.

> An energetically balanced, implicit integrator for non-hydrostatic vertical atmospheric dynamics on the sphere is presented. The integrator allows for the exact balance of energy exchanges in space and time for vertical atmospheric motions by preserving the skew-symmetry of the non-canonical Hamiltonian formulation of the compressible Euler equations. The performance of the integrator is accelerated by a preconditioning strategy that reduces the dimensionality of the inner linear system. Here we reduce the four component velocity, density, density weighted potential temperature and Exner pressure system into a single equation for the density weighted potential temperature via repeated Schur complement decomposition and the careful selection of coupling terms. As currently formulated, the integrator is based on a horizontal-vertical spatial splitting that does not permit bottom topography. The integrator is validated for standard test cases for baroclinic instability and a non-hydrostatic gravity wave on the sphere and a rising bubble in a high-resolution planar geometry, and shows robust convergence across all of these regimes.

---

### Lee et al. 2022 — potential-enstrophy-conserving upwinding in compatible FE

**Lee, D., Martín, A. F., Bladwell, C. & Badia, S. (2022).** *A comparison of variational upwinding schemes for geophysical fluids, and their application to potential enstrophy conserving discretisations.* arXiv:2203.04629 [math.NA].

> Methods for upwinding the potential vorticity in a compatible finite element discretisation of the rotating shallow water equations are studied. These include the well-known anticipated potential vorticity method (APVM), streamwise upwind Petrov-Galerkin (SUPG) method, and a recent approach where the trial functions are evaluated downstream within the reference element. In all cases the upwinding scheme conserves both potential vorticity and energy, since the antisymmetric structure of the equations is preserved. The APVM leads to a symmetric definite correction to the potential enstrophy that is dissipative and inconsistent, resulting in a turbulent state where the potential enstrophy is more strongly damped than for the other schemes. While the SUPG scheme is widely known to be consistent, since it modifies the test functions only, the downwinded trial function formulation results in the advection of downwind corrections. Results of the SUPG and downwinded trial function schemes are very similar in terms of both potential enstrophy conservation and turbulent spectra. The main difference between these schemes is in the energy conservation and residual errors. If just two nonlinear iterations are applied then the energy conservation errors are improved for the downwinded trial function formulation, reflecting a smaller residual error than for the SUPG scheme. We also present formulations by which potential enstrophy is exactly integrated at each time level. Results using these formulations are observed to be stable in the absence of dissipation, despite the uncontrolled aliasing of grid scale turbulence.

---

### Sharma, Patil & Woolsey 2020 — review of structure-preserving methods

**Sharma, H., Patil, M. & Woolsey, C. (2020).** *A review of structure-preserving numerical methods for engineering applications.* Computer Methods in Applied Mechanics and Engineering 366:113067. DOI: [10.1016/j.cma.2020.113067](https://doi.org/10.1016/j.cma.2020.113067).

> Accurate numerical simulation of dynamical systems is essential in applications ranging from particle physics to geophysical fluid flow to space hazard analysis. However, most traditional numerical methods do not account for the underlying geometric structure of the physical system, leading to simulation results that may suggest nonphysical behavior. The field of geometric numerical integration (GNI) is concerned with numerical methods that respect the fundamental physics of a problem by preserving the geometric properties of the governing differential equations. Research over the past two decades has produced GNI methods that are so accurate that they are now used for benchmarking purposes for long-time simulation of conservative dynamical systems. However, their utility for large-scale engineering problems is still an open question. This paper presents a review of structure-preserving numerical methods with focus on their engineering applications. The purpose of this paper is to provide an overview of different classes of GNI methods for mechanical systems while providing a survey of practical examples from numerical simulation of realistic engineering problems.

---

## II-B. Modified-equation & backward-error analysis; geometric integration

### McLachlan & Offen 2023 — backward error analysis for conjugate symplectic methods

**McLachlan, R. I. & Offen, C. (2023).** *Backward error analysis for conjugate symplectic methods.* Journal of Geometric Mechanics 15(1):98–115. DOI: [10.3934/jgm.2023005](https://doi.org/10.3934/jgm.2023005). Preprint: arXiv:2201.03911.

> The numerical solution of an ordinary differential equation can be interpreted as the exact solution of a nearby modified equation. Investigating the behaviour of numerical solutions by analysing the modified equation is known as backward error analysis. If the original and modified equation share structural properties, then the exact and approximate solution share geometric features such as the existence of conserved quantities. Conjugate symplectic methods preserve a modified symplectic form and a modified Hamiltonian when applied to a Hamiltonian system. We show how a blended version of variational and symplectic techniques can be used to compute modified symplectic and Hamiltonian structures. In contrast to other approaches, our backward error analysis method does not rely on an ansatz but computes the structures systematically, provided that a variational formulation of the method is known. The technique is illustrated on the example of symmetric linear multistep methods with matrix coefficients.

---

### Engel & Gottwald 2023 — modified equations explain discretisation artifacts

**Engel, M. & Gottwald, G. A. (2023).** *Canards in modified equations for Euler discretizations.* arXiv:2304.08797 [math.DS].

> Canards are a well-studied phenomenon in fast-slow ordinary differential equations implying the delayed loss of stability after the slow passage through a singularity. Recent studies have shown that the corresponding maps stemming from explicit Runge-Kutta discretizations, in particular the forward Euler scheme, exhibit significant distinctions to the continuous-time behavior: for folds, the delay in loss of stability is typically shortened whereas, for transcritical singularities, it is arbitrarily prolonged. We employ the method of modified equations, which correspond with the fixed discretization schemes up to higher order, to understand and quantify these effects directly from a fast-slow ODE, yielding consistent results with the discrete-time behavior and opening a new perspective on the wide range of (de-)stabilization phenomena along canards.

---

### Foti & Duraisamy 2021 — modified equation analysis as sub-grid-scale characterisation

**Foti, D. & Duraisamy, K. (2021).** *Sub-grid scale characterization and asymptotic behavior of multi-dimensional upwind schemes for the vorticity transport equations.* arXiv:2102.02277 [physics.flu-dyn].

> We study the sub-grid scale characteristics of a vorticity-transport-based approach for large-eddy simulations. In particular, we consider a multi-dimensional upwind scheme for the vorticity transport equations and establish its properties in the under-resolved regime. The asymptotic behavior of key turbulence statistics of velocity gradients, vorticity, and invariants is studied in detail. Modified equation analysis indicates that dissipation can be controlled locally via non-linear limiting of the gradient employed for the vorticity reconstruction on the cell face such that low numerical diffusion is obtained in well-resolved regimes and high numerical diffusion is realized in under-resolved regions. The enstrophy budget highlights the remarkable ability of the truncation terms to mimic the true sub-grid scale dissipation and diffusion. The modified equation also reveals diffusive terms that are similar to several commonly employed sub-grid scale models including tensor-gradient and hyper-viscosity models. Investigations on several canonical turbulence flow cases show that large-scale features are adequately represented and remain consistent in terms of spectral energy over a range of grid resolutions. Numerical dissipation in under-resolved simulations is consistent and can be characterized by diffusion terms discovered in the modified equation analysis. A minimum state of scale separation necessary to obtain asymptotic behavior is characterized using metrics such as effective Reynolds number and effective grid spacing. Temporally-evolving jet simulations, characterized by large-scale vortical structures, demonstrate that high Reynolds number vortex-dominated flows are captured when criteria is met and necessitate diffusive non-linear limiting of vorticity reconstruction be employed to realize accuracy in under-resolved simulations.

---

### Celledoni et al. 2022 — Lie-group integrators for mechanical systems

**Celledoni, E., Çokaj, E., Leone, A., Murari, D. & Owren, B. (2022).** *Lie group integrators for mechanical systems.* International Journal of Computer Mathematics 99(1):58–88. DOI: [10.1080/00207160.2021.1966772](https://doi.org/10.1080/00207160.2021.1966772). Preprint: arXiv:2102.12778.

> Since they were introduced in the 1990s, Lie group integrators have become a method of choice in many application areas. These include multibody dynamics, shape analysis, data science, image registration and biophysical simulations. Two important classes of intrinsic Lie group integrators are the Runge–Kutta–Munthe–Kaas methods and the commutator free Lie group integrators. We give a short introduction to these classes of methods. The Hamiltonian framework is attractive for many mechanical problems, and in particular we shall consider Lie group integrators for problems on cotangent bundles of Lie groups where a number of different formulations are possible. There is a natural symplectic structure on such manifolds and through variational principles one may derive symplectic Lie group integrators. We also consider the practical aspects of the implementation of Lie group integrators, such as adaptive time stepping. The theory is illustrated by applying the methods to two nontrivial applications in mechanics. One is the N-fold spherical pendulum where we introduce the restriction of the adjoint action of the group SE(3) to TS², the tangent bundle of the two-dimensional sphere. Finally, we show how Lie group integrators can be applied to model the controlled path of a payload being transported by two rotors.

---

### Demoures & Gay-Balmaz 2021 — multisymplectic variational integrators for fluids

**Demoures, F. & Gay-Balmaz, F. (2021).** *Multisymplectic variational integrators for barotropic and incompressible fluid models with constraints.* arXiv:2102.10907 [math.NA].

> We present a structure preserving discretization of the fundamental spacetime geometric structures of fluid mechanics in the Lagrangian description in 2D and 3D. Based on this, multisymplectic variational integrators are developed for barotropic and incompressible fluid models, which satisfy a discrete version of Noether theorem. We show how the geometric integrator can handle regular fluid motion in vacuum with free boundaries and constraints such as the impact against an obstacle of a fluid flowing on a surface. Our approach is applicable to a wide range of models including the Boussinesq and shallow water models, by appropriate choice of the Lagrangian.

---

## II-C. Entropy stability, positivity, well-balancedness — the frontier at the non-conforming seam

### Del Rey Fernández et al. 2019 — entropy-stable p-nonconforming on curvilinear grids

**Del Rey Fernández, D. C., Carpenter, M. H., Dalcin, L., Friedrich, L., Rojas, D., Winters, A. R., Gassner, G. J., Zampini, S. & Parsani, M. (2019).** *Entropy stable p-nonconforming discretizations with the summation-by-parts property for the compressible Euler equations.* arXiv:1909.12536 [math.NA].

> The entropy conservative/stable algorithm of Friedrich et al. (2018) for hyperbolic conservation laws on nonconforming p-refined/coarsened Cartesian grids, is extended to curvilinear grids for the compressible Euler equations. The primary focus is on constructing appropriate coupling procedures across the curvilinear nonconforming interfaces. A simple and flexible approach is proposed that uses interpolation operators from one element to the other. On the element faces, the analytic metrics are used to construct coupling terms, while metric terms in the volume are approximated to satisfy a discretization of the geometric conservation laws. The resulting scheme is entropy conservative/stable, elementwise conservative, and freestream preserving. The accuracy and stability properties of the resulting numerical algorithm are shown to be comparable to those of the original conforming scheme (~p+1 convergence) in the context of the isentropic Euler vortex and the inviscid Taylor-Green vortex problems on manufactured high order grids.

---

### Chan 2019 — entropy-stable DG without a strong SBP property

**Chan, J. (2019).** *Skew-symmetric entropy stable modal discontinuous Galerkin formulations.* Journal of Scientific Computing 81(1):459–485. DOI: [10.1007/s10915-019-01026-w](https://doi.org/10.1007/s10915-019-01026-w). Preprint: arXiv:1902.01828.

> High order entropy stable discontinuous Galerkin (DG) methods for nonlinear conservation laws satisfy an inherent discrete entropy inequality. The construction of such schemes has relied on the use of carefully chosen nodal points or volume and surface quadrature rules to produce operators which satisfy a summation-by-parts (SBP) property. In this work, we show how to construct "modal" skew-symmetric DG formulations which are entropy stable for volume and surface quadratures under which a traditional SBP property does not hold. These skew-symmetric formulations avoid the use of a "strong" matrix-based SBP property, and instead rely on the assumption that discrete operators exactly differentiate constants and satisfy a discrete form of the fundamental theorem of calculus. We conclude with numerical experiments verifying the accuracy and stability of the proposed formulations, and discuss an application of skew-symmetric formulations for entropy stable DG schemes on mixed quadrilateral-triangle meshes.

---

### Shadpey & Zingg 2020 — entropy-stable SBP on hp-adaptive curvilinear grids

**Shadpey, S. & Zingg, D. W. (2020).** *Entropy-stable multidimensional summation-by-parts discretizations on hp-adaptive curvilinear grids for hyperbolic conservation laws.* Journal of Scientific Computing 82(3):70. DOI: [10.1007/s10915-020-01169-1](https://doi.org/10.1007/s10915-020-01169-1).

**Abstract:** ABSTRACT NOT RETRIEVED. Crossref confirms the record; Semantic Scholar reports the abstract field as elided by the publisher; no preprint located.

---

### Wu, Trask & Chan 2021 — entropy stability + positivity + well-balancing in one shallow-water DG scheme

**Wu, X., Trask, N. & Chan, J. (2021).** *Entropy stable discontinuous Galerkin methods for the shallow water equations with subcell positivity preservation.* arXiv:2112.07749 [math.NA].

> High order schemes are known to be unstable in the presence of shock discontinuities or under-resolved solution features, and have traditionally required additional filtering, limiting, or artificial viscosity to avoid solution blow up. Entropy stable schemes address this instability by ensuring that physically relevant solutions satisfy a semi-discrete entropy inequality independently of discretization parameters. However, additional measures must be taken to ensure that solutions satisfy physical constraints such as positivity. In this work, we present a high order entropy stable discontinuous Galerkin (ESDG) method for the nonlinear shallow water equations (SWE) on two-dimensional (2D) triangular meshes which preserves the positivity of the water heights. The scheme combines a low order positivity preserving method with a high order entropy stable method using convex limiting. This method is entropy stable and well-balanced for fitted meshes with continuous bathymetry profiles.

---

### Zhang & Shu 2010 — the positivity-preserving limiter, canonical anchor

**Zhang, X. & Shu, C.-W. (2010).** *On positivity-preserving high order discontinuous Galerkin schemes for compressible Euler equations on rectangular meshes.* Journal of Computational Physics 229(23):8918–8934. DOI: [10.1016/j.jcp.2010.08.016](https://doi.org/10.1016/j.jcp.2010.08.016).

**Abstract:** ABSTRACT NOT RETRIEVED. Elsevier deposits no abstract; Crossref, Semantic Scholar and OpenAlex all return null; no open-access copy found via Unpaywall.

---

### Kurganov, Qu & Wu 2022 — well-balanced positivity-preserving on adaptive moving meshes

**Kurganov, A., Qu, Z. & Wu, T. (2022).** *Well-balanced positivity preserving adaptive moving mesh central-upwind schemes for the Saint-Venant system.* ESAIM: Mathematical Modelling and Numerical Analysis 56(4):1327–1360. DOI: [10.1051/m2an/2022041](https://doi.org/10.1051/m2an/2022041).

> We extend the adaptive moving mesh (AMM) central-upwind schemes recently proposed in Kurganov et al. [Commun. Appl. Math. Comput. 3 (2021) 445–479] in the context of one- (1-D) and two-dimensional (2-D) Euler equations of gas dynamics and granular hydrodynamics, to the 1-D and 2-D Saint-Venant system of shallow water equations. When the bottom topography is nonflat, these equations form hyperbolic systems of balance laws, for which a good numerical method should be capable of preserving a delicate balance between the flux and source terms as well as preserving the nonnegativity of water depth even in the presence of dry or almost dry regions. Therefore, in order to extend the AMM central-upwind schemes to the Saint-Venant systems, we develop special positivity preserving reconstruction and evolution steps of the AMM algorithms as well as special corrections of the solution projection step in (almost) dry areas. At the same time, we enforce the moving mesh to be structured even in the case of complicated 2-D computational domains. We test the designed method on a number of 1-D and 2-D examples that demonstrate robustness and high resolution of the proposed numerical approach.

---

### Wu, Li & Weng 2025 — positivity-preserving well-balanced shallow flow on quadtree AMR

**Wu, G., Li, Z. & Weng, H. (2025).** *A robust two-dimensional shallow flow model with adaptive quadtree mesh.* Journal of Marine Science and Engineering 13(10):1834. DOI: [10.3390/jmse13101834](https://doi.org/10.3390/jmse13101834).

> A two-dimensional shallow flow model is developed by integrating a positivity-preserving, well-balanced central-upwind scheme with a block-structured quadtree AMR grid implemented in the Afivo open-source framework. A non-negative water depth reconstruction ensures second-order spatial accuracy and the robust treatment of wetting and drying, while coarse-grid fluxes at refinement boundaries are obtained by summing the corresponding fine-grid fluxes, thereby guaranteeing strict mass conservation between refinement levels. Mesh refinement is driven by gradients in water surface elevation, which focus resolution on regions of rapid flow variation, thereby improving both accuracy and computational efficiency. Model validation through benchmark problems and the Malpasset dam-break event show close agreement with analytical solutions, laboratory measurements, and previous numerical simulations, while achieving substantial reductions in computational cost.

---

## II-D. Spherical grids, projections, staggering

### Mouallem, Harris & Chen 2023 — the Duo-Grid: eliminating cube-sphere grid imprinting in FV3

**Mouallem, J., Harris, L. & Chen, X. (2023).** *Implementation of the novel Duo-Grid in GFDL's FV3 dynamical core.* Journal of Advances in Modeling Earth Systems 15(12):e2023MS003712. DOI: [10.1029/2023MS003712](https://doi.org/10.1029/2023MS003712).

> The gnomonic cubed-sphere grid has excellent accuracy and uniformity, but the "kink" in the coordinates at the cube edges in the halo region can leave an imprint of the cube in the solution, and requires special edge handling. To reduce grid imprinting, we implement the novel "Duo-Grid" within the Geophysical Fluid Dynamics Laboratory's (GFDL) Finite-Volume Cubed-Sphere Dynamical Core (FV3). The Duo-Grid remaps a cube face's data from neighboring face from kinked to natural locations along great circle lines using 1D piecewise linear interpolation. A 2D interpolation algorithm is used to fill correct data at the eight corners of the cubed-sphere needed for FV3's 2D advection scheme. The Duo-Grid was tested in idealized tests using the 2D shallow water solver and the 3D hydrostatic and non-hydrostatic solvers. We found that error norms are greatly reduced and grid imprinting is practically eliminated when employing the Duo-Grid. These results indicate that FV3's accuracy and robustness have improved.

---

### Arakawa & Lamb 1977 — the A–E staggering taxonomy

**Arakawa, A. & Lamb, V. R. (1977).** *Computational design of the basic dynamical processes of the UCLA general circulation model.* Methods in Computational Physics: Advances in Research and Applications 17:173–265. DOI: [10.1016/B978-0-12-460817-7.50009-4](https://doi.org/10.1016/B978-0-12-460817-7.50009-4).

**Abstract:** ABSTRACT NOT RETRIEVED. No abstract is deposited anywhere reachable; Semantic Scholar returns null and ScienceDirect is subscription-gated. This is the paper that defines the Arakawa A/B/C/D/E grid taxonomy. (Note: distinct from `arakawa-1981-enstrophy` in Part I §D, which is the potential-enstrophy-and-energy-conserving scheme.)

---

## II-E. AMR frameworks, multiresolution, multirate

### Burstedde, Wilcox & Ghattas 2011 — p4est

**Burstedde, C., Wilcox, L. C. & Ghattas, O. (2011).** *p4est: Scalable algorithms for parallel adaptive mesh refinement on forests of octrees.* SIAM Journal on Scientific Computing 33(3):1103–1133. DOI: [10.1137/100791634](https://doi.org/10.1137/100791634).

> We present scalable algorithms for parallel adaptive mesh refinement and coarsening (AMR), partitioning, and 2:1 balancing on computational domains composed of multiple connected two-dimensional quadtrees or three-dimensional octrees, referred to as a forest of octrees. By distributing the union of octants from all octrees in parallel, we combine the high scalability proven previously for adaptive single-octree algorithms with the geometric flexibility that can be achieved by arbitrarily connected hexahedral macromeshes, in which each macroelement is the root of an adapted octree. A key concept of our approach is an encoding scheme of the interoctree connectivity that permits arbitrary relative orientations between octrees. Based on this encoding we develop interoctree transformations of octants. These form the basis for high-level parallel octree algorithms, which are designed to interact with an application code such as a numerical solver for partial differential equations. We have implemented and tested these algorithms in the p4est software library. We demonstrate the parallel scalability of p4est on its own and in combination with two geophysics codes. Using p4est we generate and adapt multioctree meshes with up to 5.13×10¹¹ octants on as many as 220,320 CPU cores and execute the 2:1 balance algorithm in less than 10 seconds per million octants per process.

---

### Zhang et al. 2021 — AMReX

**Zhang, W., Myers, A., Gott, K., Almgren, A. & Bell, J. (2021).** *AMReX: Block-structured adaptive mesh refinement for multiphysics applications.* The International Journal of High Performance Computing Applications 35(6):508–526. DOI: [10.1177/10943420211022811](https://doi.org/10.1177/10943420211022811). Preprint: arXiv:2009.12009.

> Block-structured adaptive mesh refinement (AMR) provides the basis for the temporal and spatial discretization strategy for a number of ECP applications in the areas of accelerator design, additive manufacturing, astrophysics, combustion, cosmology, multiphase flow, and wind plant modelling. AMReX is a software framework that provides a unified infrastructure with the functionality needed for these and other AMR applications to be able to effectively and efficiently utilize machines from laptops to exascale architectures. AMR reduces the computational cost and memory footprint compared to a uniform mesh while preserving accurate descriptions of different physical processes in complex multi-physics algorithms. AMReX supports algorithms that solve systems of partial differential equations (PDEs) in simple or complex geometries, and those that use particles and/or particle-mesh operations to represent component physical processes. In this paper, we will discuss the core elements of the AMReX framework such as data containers and iterators as well as several specialized operations to meet the needs of the application projects. In addition we will highlight the strategy that the AMReX team is pursuing to achieve highly performant code across a range of accelerator-based architectures for a variety of different applications.

---

### Weinzierl 2019 — Peano

**Weinzierl, T. (2019).** *The Peano software — parallel, automaton-based, dynamically adaptive grid traversals.* ACM Transactions on Mathematical Software 45(2):Article 14. DOI: [10.1145/3319797](https://doi.org/10.1145/3319797).

> We discuss the design decisions, design alternatives, and rationale behind the third generation of Peano, a framework for dynamically adaptive Cartesian meshes derived from spacetrees. Peano ties the mesh traversal to the mesh storage and supports only one element-wise traversal order resulting from space-filling curves. The user is not free to choose a traversal order herself. The traversal can exploit regular grid subregions and shared memory as well as distributed memory systems with almost no modifications to a serial application code. We formalize the software design by means of two interacting automata — one automaton for the multiscale grid traversal and one for the application-specific algorithmic steps. This yields a callback-based programming paradigm. We further sketch the supported application types and the two data storage schemes realized before we detail high-performance computing aspects and lessons learned. Special emphasis is put on observations regarding the used programming idioms and algorithmic concepts. This transforms our report from a "one way to implement things" code description into a generic discussion and summary of some alternatives, rationale, and design decisions to be made for any tree-based adaptive mesh refinement software.

---

### Vasilyev & Bowman 2000 — the adaptive wavelet collocation method

**Vasilyev, O. V. & Bowman, C. (2000).** *Second-generation wavelet collocation method for the solution of partial differential equations.* Journal of Computational Physics 165(2):660–693. DOI: [10.1006/jcph.2000.6638](https://doi.org/10.1006/jcph.2000.6638).

**Abstract:** ABSTRACT NOT RETRIEVED. Elsevier deposits no abstract; Crossref, Semantic Scholar and OpenAlex all return null; ScienceDirect 403s.

---

### Sandu 2019 — MRI-GARK, the modern multirate method of record

**Sandu, A. (2019).** *A class of multirate infinitesimal GARK methods.* SIAM Journal on Numerical Analysis 57(5):2300–2327. DOI: [10.1137/18M1205492](https://doi.org/10.1137/18M1205492). Preprint: arXiv:1808.02759.

> Differential equations arising in many practical applications are characterized by multiple time scales. Multirate time integration seeks to solve them efficiently by discretizing each scale with a different, appropriate time step, while ensuring the overall accuracy and stability of the numerical solution. In a seminal paper Knoth and Wolke (APNUM, 1998) proposed a hybrid solution approach: discretize the slow component with an explicit Runge-Kutta method, and advance the fast component via a modified fast differential equation. The idea led to the development of multirate infinitesimal step (MIS) methods by Wensch et al. (BIT, 2009.) Günther and Sandu (BIT, 2016) explained MIS schemes as a particular case of multirate General-structure Additive Runge-Kutta (MR-GARK) methods. The hybrid approach offers extreme flexibility in the choice of the numerical solution process for the fast component. This work constructs a family of multirate infinitesimal GARK schemes (MRI-GARK) that extends the hybrid dynamics approach in multiple ways. Order conditions theory and stability analyses are developed, and practical explicit and implicit methods of up to order four are constructed. Numerical results confirm the theoretical findings. We expect the new MRI-GARK family to be most useful for systems of equations with widely disparate time scales, where the fast process is dispersive, and where the influence of the fast component on the slow dynamics is weak.

---

### Roberts et al. 2021 — implicit multirate GARK

**Roberts, S., Loffeld, J., Sarshar, A., Woodward, C. S. & Sandu, A. (2021).** *Implicit multirate GARK methods.* Journal of Scientific Computing 87(1):4. DOI: [10.1007/s10915-020-01400-z](https://doi.org/10.1007/s10915-020-01400-z). Preprint: arXiv:1910.14079.

> This work considers multirate generalized-structure additively partitioned Runge-Kutta (MrGARK) methods for solving stiff systems of ordinary differential equations (ODEs) with multiple time scales. These methods treat different partitions of the system with different timesteps for a more targeted and efficient solution compared to monolithic single rate approaches. With implicit methods used across all partitions, methods must find a balance between stability and the cost of solving nonlinear equations for the stages. In order to characterize this important trade-off, we explore multirate coupling strategies, problems for assessing linear stability, and techniques to efficiently implement Newton iterations for stage equations. Unlike much of the existing multirate stability analysis which is limited in scope to particular methods, we present general statements on stability and describe fundamental limitations for certain types of multirate schemes. New implicit multirate methods up to fourth order are derived, and their accuracy and efficiency properties are verified with numerical tests.

---

### Günther & Sandu 2021 — multirate linearly-implicit GARK

**Günther, M. & Sandu, A. (2021).** *Multirate linearly-implicit GARK schemes.* BIT Numerical Mathematics 62(3):869–901. DOI: [10.1007/s10543-021-00898-5](https://doi.org/10.1007/s10543-021-00898-5). Preprint: arXiv:2102.10203.

> Many complex applications require the solution of initial-value problems where some components change fast, while others vary slowly. Multirate schemes apply different step sizes to resolve different components of the system, according to their dynamics, in order to achieve increased computational efficiency. The stiff components of the system, fast or slow, are best discretized with implicit base methods in order to ensure numerical stability. To this end, linearly implicit methods are particularly attractive as they solve only linear systems of equations at each step. This paper develops the Multirate GARK-ROS/ROW (MR-GARK-ROS/ROW) framework for linearly-implicit multirate time integration. The order conditions theory considers both exact and approximative Jacobians. The effectiveness of implicit multirate methods depends on the coupling between the slow and fast computations; an array of efficient coupling strategies and the resulting numerical schemes are analyzed. Multirate infinitesimal step linearly-implicit methods, that allow arbitrarily small micro-steps and offer extreme computational flexibility, are constructed. The new unifying framework includes existing multirate Rosenbrock(-W) methods as particular cases, and opens the possibility to develop new classes of highly effective linearly implicit multirate integrators.

---

## II-F. Landscape evolution & drainage routing

### Whipple & Tucker 2002 — sediment-flux-dependent incision

**Whipple, K. X. & Tucker, G. E. (2002).** *Implications of sediment-flux-dependent river incision models for landscape evolution.* Journal of Geophysical Research: Solid Earth 107(B2):2039. DOI: [10.1029/2000JB000044](https://doi.org/10.1029/2000JB000044).

> Developing a quantitative understanding of the factors that control the rate of river incision into bedrock is critical to studies of landscape evolution and the linkages between climate, erosion, and tectonics. Current models of long-term river network incision differ significantly in their treatment of the role of sediment flux. We analyze the implications of various sediment-flux-dependent incision models for large-scale topography, in an attempt (1) to identify quantifiable and diagnostic differences between models that could be detected from topographic data or from the transient responses of perturbed systems and (2) to explain the apparent ubiquity of mixed bedrock-alluvial channels in active orogens. Although certain forms of the various models can be discarded as inconsistent with morphological data, we find that the relative intrinsic concavity indices of detachment- and transport-limited systems (defined herein) largely dictate whether the various models can be tied to distinctive steady state morphologies. Preliminary data suggest that no such diagnostic differences may exist, and other methods must be developed to test models. Accordingly, we develop and explore differences in the scaling behavior of topographic relief and the extent of detachment- versus transport-limited channels as a function of rock uplift rate that may allow discrimination among various models. Further, we explore potentially diagnostic differences in the rates and patterns of transient channel response to changes in rock uplift rate. In addition to general differences between detachment- and transport-limited systems our analysis identifies an interesting hysteresis in landscape evolution: 'hybrid' channels at the threshold between detachment- and transport-limited conditions are expected to act as detachment-limited systems in response to an increase in rock uplift rate (or base level fall) and as transport-limited systems in response to a decrease in rock uplift rate, especially during postorogenic topographic decline. The analyses presented set the stage for field studies designed to test quantitatively the various river incision models that have been proposed.

---

### DiBiase & Whipple 2011 — thresholds and runoff variability (stochastic-threshold SPIM)

**DiBiase, R. A. & Whipple, K. X. (2011).** *The influence of erosion thresholds and runoff variability on the relationships among topography, climate, and erosion rate.* Journal of Geophysical Research: Earth Surface 116(F4):F04036. DOI: [10.1029/2011JF002095](https://doi.org/10.1029/2011JF002095).

> Bedrock river incision occurs only during floods large enough to mobilize sediment and overcome substrate detachment thresholds. New data relating channel steepness and erosion rate provide the opportunity to evaluate the role of thresholds and discharge variability in landscape evolution. We augment an extensive erosion rate data set in the San Gabriel Mountains, California, with analysis of streamflow records and observations of channel width and sediment cover to evaluate the importance of climate and erosion thresholds on incision rates. We find the relationship between channel steepness and erosion rate in the San Gabriel Mountains can be explained using a simple stochastic-threshold incision model where the distribution of large floods follows an inverse power law, suggesting that details of incision mechanics, sediment effects, width adjustment, and debris flows do not significantly influence the steady state relationship between steepness and erosion rate. Using parameters tuned to this case, we vary climate parameters to explore a range of behavior for the steepness-erosion relationship. Erosion is enhanced by both increases in mean runoff and discharge variability. We explore the implications of an empirical relationship between mean runoff and variability to test whether dry, variable climates can erode more efficiently than wet, stable climates. For channels with high thresholds or low steepness, modeled erosion rate peaks at a mean runoff of 200-400 mm/yr. For much of the parameter space tested, erosion rates are predicted to be insensitive to increases in runoff above ~500 mm/yr, with important implications for the hypothesized influence of climate on tectonics.

---

### O'Callaghan & Mark 1984 — D8

**O'Callaghan, J. F. & Mark, D. M. (1984).** *The extraction of drainage networks from digital elevation data.* Computer Vision, Graphics, and Image Processing 28(3):323–344. DOI: [10.1016/S0734-189X(84)80011-0](https://doi.org/10.1016/S0734-189X(84)80011-0).

**Abstract:** ABSTRACT NOT RETRIEVED. Crossref confirms all bibliographic fields but carries no abstract; ScienceDirect, Semantic Scholar and ResearchGate all 403'd or returned null.

---

### Tarboton 1997 — D-infinity

**Tarboton, D. G. (1997).** *A new method for the determination of flow directions and upslope areas in grid digital elevation models.* Water Resources Research 33(2):309–319. DOI: [10.1029/96WR03137](https://doi.org/10.1029/96WR03137).

> A new procedure for the representation of flow directions and calculation of upslope areas using rectangular grid digital elevation models is presented. The procedure is based on representing flow direction as a single angle taken as the steepest downward slope on the eight triangular facets centered at each grid point. Upslope area is then calculated by proportioning flow between two downslope pixels according to how close this flow direction is to the direct angle to the downslope pixel. This procedure offers improvements over prior procedures that have restricted flow to eight possible directions (introducing grid bias) or proportioned flow according to slope (introducing unrealistic dispersion). The new procedure is more robust than prior procedures based on fitting local planes while retaining a simple grid based structure. Detailed algorithms are presented and results are demonstrated through test examples and application to digital elevation data sets.

---

### Quinn et al. 1991 — MFD / FD8, the multiple-flow-direction classic

**Quinn, P., Beven, K., Chevallier, P. & Planchon, O. (1991).** *The prediction of hillslope flow paths for distributed hydrological modelling using digital terrain models.* Hydrological Processes 5(1):59–79. DOI: [10.1002/hyp.3360050106](https://doi.org/10.1002/hyp.3360050106).

> The accuracy of the predictions of distributed hydrological models must depend in part on the proper specification of flow pathways. This paper examines some of the problems of deriving flow pathways from raster digital terrain data in the context of hydrological predictions using TOPMODEL. Distributed moisture status is predicted in TOPMODEL on the basis of spatial indices that depend on flow path definition. The sensitivity of this index to flow path algorithm and grid size is examined for the case where the surface topography is a good indicator of local hydraulic gradients. A strategy for the case where downslope subsurface flow pathways may deviate from those indicated by the surface topography is described with an example application.

---

### Gallant & Hutchinson 2011 — the analytical definition of specific catchment area

**Gallant, J. C. & Hutchinson, M. F. (2011).** *A differential equation for specific catchment area.* Water Resources Research 47(5):W05535. DOI: [10.1029/2009WR008540](https://doi.org/10.1029/2009WR008540).

**Abstract:** ABSTRACT NOT RETRIEVED. Crossref and Semantic Scholar confirm the record but expose no abstract; AGU/Wiley returned 403 and NASA ADS rendered no content. *(This is the paper Bonetti et al. 2018 and Coatléven & Chauveau build on — the SCA differential equation valid at regular points.)*

---

### Barnhart et al. 2020 — Landlab v2.0

**Barnhart, K. R., Hutton, E. W. H., Tucker, G. E., Gasparini, N. M., Istanbulluoglu, E., Hobley, D. E. J., Lyons, N. J., Mouchene, M., Nudurupati, S. S., Adams, J. M. & Bandaragoda, C. (2020).** *Short communication: Landlab v2.0: a software package for Earth surface dynamics.* Earth Surface Dynamics 8(2):379–397. DOI: [10.5194/esurf-8-379-2020](https://doi.org/10.5194/esurf-8-379-2020).

> Numerical simulation of the form and characteristics of Earth's surface provides insight into its evolution. Landlab serves researchers by providing modularized components that reduce implementation time for new or existing models. The package includes a gridding engine supporting structured and irregular mesh representations, 58 components implementing discrete Earth surface processes, and supporting utilities. This contribution documents developments since version 1.0, including grid refactoring, enhanced component interfaces, Python 2 discontinuation, 31 additional components, and revised documentation.

---

### Schwanghart & Scherler 2014 — TopoToolbox 2

**Schwanghart, W. & Scherler, D. (2014).** *Short Communication: TopoToolbox 2 — MATLAB-based software for topographic analysis and modeling in Earth surface sciences.* Earth Surface Dynamics 2(1):1–7. DOI: [10.5194/esurf-2-1-2014](https://doi.org/10.5194/esurf-2-1-2014).

> TopoToolbox is a MATLAB program for the analysis of digital elevation models (DEMs). With the release of version 2, the software adopts an object-oriented programming (OOP) approach to work with gridded DEMs and derived data such as flow directions and stream networks. The introduction of a novel technique to store flow directions as topologically ordered vectors of indices enables calculation of flow-related attributes such as flow accumulation ~20 times faster than conventional algorithms while at the same time reducing memory overhead to 33% of that required by the previous version. Graphical user interfaces (GUIs) enable visual exploration and interaction with DEMs and derivatives and provide access to tools targeted at fluvial and tectonic geomorphologists. With its new release, TopoToolbox has become a more memory-efficient and faster tool for basic and advanced digital terrain analysis that can be used as a framework for building hydrological and geomorphological models in MATLAB.

---

### Cordonnier, Bovy & Braun 2019 — linear-complexity flow routing through depressions

**Cordonnier, G., Bovy, B. & Braun, J. (2019).** *A versatile, linear complexity algorithm for flow routing in topographies with depressions.* Earth Surface Dynamics 7(2):549–562. DOI: [10.5194/esurf-7-549-2019](https://doi.org/10.5194/esurf-7-549-2019).

**Availability:** in relata as `cordonnier-2018-versatile` (metadata + PDF) — see Part I §F. *(The relata key says 2018; the published record is 2019.)*

> We present a new algorithm for solving the common problem of flow trapped in closed depressions within digital elevation models, as encountered in many applications relying on flow routing. Unlike other approaches (e.g., the Priority-Flood depression filling algorithm), this solution is based on the explicit computation of the flow paths both within and across the depressions through the construction of a graph connecting together all adjacent drainage basins. Although this represents many operations, a linear time complexity can be reached for the whole computation, making it very efficient. Compared to the most optimized solutions proposed so far, we show that this algorithm of flow path enforcement yields the best performance when used in landscape evolution models. In addition to its efficiency, our proposed method also has the advantage of letting the user choose among different strategies of flow path enforcement within the depressions (i.e., filling vs. carving). Furthermore, the computed graph of basins is a generic structure that has the potential to be reused for solving other problems as well, such as the simulation of erosion. This sequential algorithm may be helpful for those who need to, e.g., process digital elevation models of moderate size on single computers or run batches of simulations as part of an inference study.

---

## II-G. Surface hydraulics & sediment

### Guidolin et al. 2016 — weighted cellular-automata 2D inundation (the closest verified analogue to virtual-pipes)

**Guidolin, M., Chen, A. S., Ghimire, B., Keedwell, E. C., Djordjević, S. & Savić, D. A. (2016).** *A weighted cellular automata 2D inundation model for rapid flood analysis.* Environmental Modelling & Software 84:378–394. DOI: [10.1016/j.envsoft.2016.07.008](https://doi.org/10.1016/j.envsoft.2016.07.008).

> To achieve fast flood modelling for large-scale problems, a two-dimensional cellular automata based model was developed. This model employs simple transition rules and a weight-based system rather than complex Shallow Water Equations. The simplified feature of cellular automata allows the model to be implemented in parallel environments, resulting in significantly improved modelling efficiency. The model has been tested using an analytical solution and four case studies and the outputs were compared to those from a widely-used commercial physically-based hydraulic model. Results show that the model is capable of simulating water-depth and velocity variables with reasonably good agreement with the benchmark model, using only a fraction of the computational time and memory. In the case of the real world example, the proposed model run times are up to 8 times faster. The rapid and accurate attributes of the model have demonstrated its applicability for quick flood analysis in large modelling systems.

---

### Balmforth & Mandre 2004 — roll waves

**Balmforth, N. J. & Mandre, S. (2004).** *Dynamics of roll waves.* Journal of Fluid Mechanics 514:1–33. DOI: [10.1017/S0022112004009930](https://doi.org/10.1017/S0022112004009930).

> Shallow-water equations with bottom drag and viscosity are used to study the dynamics of roll waves. First, we explore the effect of bottom topography on linear stability of turbulent flow over uneven surfaces. Low-amplitude topography is found to destabilize turbulent roll waves and lower the critical Froude number required for instability. At higher amplitude, the trend reverses and topography stabilizes roll waves. At intermediate topographic amplitude, instability can be created at much lower Froude numbers due to the development of hydraulic jumps in the equilibrium flow. Second, the nonlinear dynamics of the roll waves is explored, with numerical solutions of the shallow-water equations complementing an asymptotic theory relevant near onset. We find that trains of roll waves undergo coarsening due to waves overtaking one another and merging, lengthening the scale of the pattern. Unlike previous investigations, we find that coarsening does not always continue to its ultimate conclusion (a single roll wave with the largest spatial scale). Instead, coarsening becomes interrupted at intermediate scales, creating patterns with preferred wavelengths. We quantify the coarsening dynamics in terms of linear stability of steady roll-wave trains.

---

### Chow 1959 — Manning roughness, the standard reference

**Chow, V. T. (1959).** *Open-Channel Hydraulics.* McGraw-Hill, New York. xviii + 680 pp.

**Abstract:** ABSTRACT NOT RETRIEVED — a pre-abstract-era textbook. **No abstract exists.** Bibliographic record confirmed against an archive.org catalogue record.

---

### Wild et al. 2025 — grain-size tracking inside a FastScape-lineage LEM

**Wild, A. L., Braun, J., Whittaker, A. C. & Castelltort, S. (2025).** *Grain size dynamics using a new planform model – Part 1: GravelScape description and validation.* Earth Surface Dynamics 13(5):875–887. DOI: [10.5194/esurf-13-875-2025](https://doi.org/10.5194/esurf-13-875-2025).

> The grain size preserved within the stratigraphic record over thousands to millions of years has several important applications. In particular, it can serve as a record of significant climatic, eustatic, or tectonic events. Here we present a new model for grain size fining predictions that combines a landscape evolution model based on the Stream Power Law but modified for sedimentation (Yuan et al., 2019) with an extension of the self-similar grain size model (Fedele and Paola, 2007). The new model, which we called GravelScape, includes the effects on grain size fining of lateral heterogeneities in deposition rate caused by dynamically evolving channels. We show that, when multi-channel dynamics (i.e. avulsions) are prevented, by reducing the planform model to a single downstream dimension, our new model can reproduce results obtained by other methods that assume that fining is controlled by subsidence only. We demonstrate that including across-basin (two-dimensional) effects can lead to deviations from previous subsidence predictions for grain size fining. The magnitude of these deviations correlates with the extent of sediment bypass and the configuration of surface topography, both of which influence the amplitude of across-basin variability within the sedimentary system.
---

## II-H. Early-Earth geology: continental emergence, cratons, tectonic onset, oxygenation

*This cluster was compiled directly against the Crossref API and PubMed; every record and abstract below was fetched, not recalled.*

### Korenaga 2013 — initiation and evolution of plate tectonics

**Korenaga, J. (2013).** *Initiation and evolution of plate tectonics on Earth: theories and observations.* Annual Review of Earth and Planetary Sciences 41(1):117–151. DOI: [10.1146/annurev-earth-050212-124208](https://doi.org/10.1146/annurev-earth-050212-124208).

> The inception of plate tectonics on Earth and its subsequent evolution are discussed on the basis of theoretical considerations and observational constraints. The likelihood of plate tectonics in the past depends on what mechanism is responsible for the relatively constant surface heat flux that is indicated by the likely thermal history of Earth. The continuous operation of plate tectonics throughout Earth's history is possible if, for example, the strength of convective stress in the mantle is affected by the gradual subduction of surface water. Various geological indicators for the emergence of plate tectonics are evaluated from a geodynamical perspective, and they invariably involve certain implicit assumptions about mantle dynamics, which are either demonstrably wrong or yet to be explored. The history of plate tectonics is suggested to be intrinsically connected to the secular evolution of the atmosphere, through sea-level changes caused by ocean-mantle interaction.

---

### Cawood et al. 2018 — the geological archive of plate-tectonic onset

**Cawood, P. A., Hawkesworth, C. J., Pisarevsky, S. A., Dhuime, B., Capitanio, F. A. & Nebel, O. (2018).** *Geological archive of the onset of plate tectonics.* Philosophical Transactions of the Royal Society A 376(2132):20170405. DOI: [10.1098/rsta.2017.0405](https://doi.org/10.1098/rsta.2017.0405).

> Plate tectonics, involving a globally linked system of lateral motion of rigid surface plates, is a characteristic feature of our planet, but estimates of how long it has been the modus operandi of lithospheric formation and interactions range from the Hadean to the Neoproterozoic. In this paper, we review sedimentary, igneous and metamorphic proxies along with palaeomagnetic data to infer both the development of rigid lithospheric plates and their independent relative motion, and conclude that significant changes in Earth behaviour occurred in the mid- to late Archaean, between 3.2 Ga and 2.5 Ga. These data include: sedimentary rock associations inferred to have accumulated in passive continental margin settings, marking the onset of sea-floor spreading; the oldest foreland basin deposits associated with lithospheric convergence; a change from thin, new continental crust of mafic composition to thicker crust of intermediate composition, increased crustal reworking and the emplacement of potassic and peraluminous granites, indicating stabilization of the lithosphere; replacement of dome and keel structures in granite-greenstone terranes, which relate to vertical tectonics, by linear thrust imbricated belts; the commencement of temporally paired systems of intermediate and high dT/dP gradients, with the former interpreted to represent subduction to collisional settings and the latter representing possible hinterland back-arc settings or ocean plateau environments. Palaeomagnetic data from the Kaapvaal and Pilbara cratons for the interval 2780–2710 Ma and from the Superior, Kaapvaal and Kola-Karelia cratons for 2700–2440 Ma suggest significant relative movements. We consider these changes in the behaviour and character of the lithosphere to be consistent with a gestational transition from a non-plate tectonic mode, arguably with localized subduction, to the onset of sustained plate tectonics.

---

### Palin et al. 2020 — secular change and the onset of plate tectonics

**Palin, R. M., Santosh, M., Cao, W., Li, S.-S., Hernández-Uribe, D. & Parsons, A. (2020).** *Secular change and the onset of plate tectonics on Earth.* Earth-Science Reviews 207:103172. DOI: [10.1016/j.earscirev.2020.103172](https://doi.org/10.1016/j.earscirev.2020.103172).

> The Earth as a planetary system has experienced significant change since its formation c. 4.54 Gyr ago. Some of these changes have been gradual, such as secular cooling of the mantle, and some have been abrupt, such as the rapid increase in free oxygen in the atmosphere at the Archean–Proterozoic transition. Many of these changes have directly affected tectonic processes on Earth and are manifest by temporal trends within the sedimentary, igneous, and metamorphic rock record. Indeed, the timing of global onset of mobile-lid (subduction-driven) plate tectonics on our planet remains one of the fundamental points of debate within the geosciences today, and constraining the age and cause of this transition has profound implications for understanding our own planet's long-term evolution, and that for other rocky bodies in our solar system. Interpretations based on various sources of evidence have led different authors to propose a very wide range of ages for the onset of subduction-driven tectonics, which span almost all of Earth history from the Hadean to the Neoproterozoic, with this uncertainty stemming from the varying reliability of different proxies. Here, we review evidence for paleo-subduction preserved within the geological record, with a focus on metamorphic rocks and the geodynamic information that can be derived from them. First, we describe the different types of tectonic/geodynamic regimes that may occur on Earth or any other silicate body, and then review different models for the thermal evolution of the Earth and the geodynamic conditions necessary for plate tectonics to stabilize on a rocky planet. The community's current understanding of the petrology and structure of Archean and Proterozoic oceanic and continental crust is then discussed in comparison with modern-day equivalents, including how and why they differ. We then summarize evidence for the operation of subduction through time, including petrological (metamorphic), tectonic, and geochemical/isotopic data, and the results of petrological and geodynamical modeling. The styles of metamorphism in the Archean are then examined and we discuss how the secular distribution of metamorphic rock types can inform the type of geodynamic regime that operated at any point in time. In conclusion, we argue that most independent observations from the geological record and results of lithospheric-scale geodynamic modeling support a global-scale initiation of plate tectonics no later than c. 3 Ga, just preceding the Archean–Proterozoic transition. Evidence for subduction in Early Archean terranes is likely accounted for by localized occurrences of plume-induced subduction initiation, although these did not develop into a stable, globally connected network of plate boundaries until later in Earth history. Finally, we provide a discussion of major unresolved questions related to this review's theme and provide suggested directions for future research.

---

### Pearson et al. 2021 — deep continental roots and cratons

**Pearson, D. G., Scott, J. M., Liu, J., Schaeffer, A., Wang, L. H., van Hunen, J., Szilas, K., Chacko, T. & Kelemen, P. B. (2021).** *Deep continental roots and cratons.* Nature 596(7871):199–210. DOI: [10.1038/s41586-021-03600-5](https://doi.org/10.1038/s41586-021-03600-5).

**Abstract** *(source: PubMed, PMID 34381239)*

> The formation and preservation of cratons — the oldest parts of the continents, comprising over 60 per cent of the continental landmass — remains an enduring problem. Key to craton development is how and when the thick strong mantle roots that underlie these regions formed and evolved. Peridotite melting residues forming cratonic lithospheric roots mostly originated via relatively low-pressure melting and were subsequently transported to greater depth by thickening produced by lateral accretion and compression. The longest-lived cratons were assembled during Mesoarchean and Palaeoproterozoic times, creating the stable mantle roots 150 to 250 kilometres thick that are critical to preserving Earth's early continents and central to defining the cratons, although we extend the definition of cratons to include extensive regions of long-stable Mesoproterozoic crust also underpinned by thick lithospheric roots. The production of widespread thick and strong lithosphere via the process of orogenic thickening, possibly in several cycles, was fundamental to the eventual emergence of extensive continental landmasses — the cratons.

---

### Bindeman et al. 2018 — rapid emergence of subaerial landmasses at 2.5 Ga

**Bindeman, I. N., Zakharov, D. O., Palandri, J., Greber, N. D., Dauphas, N., Retallack, G. J., Hofmann, A., Lackey, J. S. & Bekker, A. (2018).** *Rapid emergence of subaerial landmasses and onset of a modern hydrologic cycle 2.5 billion years ago.* Nature 557(7706):545–548. DOI: [10.1038/s41586-018-0131-1](https://doi.org/10.1038/s41586-018-0131-1).

**Abstract** *(source: PubMed, PMID 29795252)*

> The history of the growth of continental crust is uncertain, and several different models that involve a gradual, decelerating, or stepwise process have been proposed. Even more uncertain is the timing and the secular trend of the emergence of most landmasses above the sea (subaerial landmasses), with estimates ranging from about one billion to three billion years ago. The area of emerged crust influences global climate feedbacks and the supply of nutrients to the oceans, and therefore connects Earth's crustal evolution to surface environmental conditions. Here we use the triple-oxygen-isotope composition of shales from all continents, spanning 3.7 billion years, to provide constraints on the emergence of continents over time. Our measurements show a stepwise total decrease of 0.08 per mille in the average triple-oxygen-isotope value of shales across the Archaean–Proterozoic boundary. We suggest that our data are best explained by a shift in the nature of water–rock interactions, from near-coastal in the Archaean era to predominantly continental in the Proterozoic, accompanied by a decrease in average surface temperatures. We propose that this shift may have coincided with the onset of a modern hydrological cycle owing to the rapid emergence of continental crust with near-modern average elevation and aerial extent roughly 2.5 billion years ago.

---

### Rosas & Korenaga 2021 — Archaean seafloors shallowed with age

**Rosas, J. C. & Korenaga, J. (2021).** *Archaean seafloors shallowed with age due to radiogenic heating in the mantle.* Nature Geoscience 14(1):51–56. DOI: [10.1038/s41561-020-00673-1](https://doi.org/10.1038/s41561-020-00673-1).

**Abstract:** ABSTRACT NOT RETRIEVED. Springer Nature deposits no abstract to Crossref; the record is absent from PubMed; OpenAlex returns no inverted index. Bibliographic record confirmed against Crossref. *(This is the paper that argues Archaean ocean basins were shallower, which bears directly on freeboard and the emerged-land fraction.)*

---

### Lyons, Reinhard & Planavsky 2014 — the rise of oxygen

**Lyons, T. W., Reinhard, C. T. & Planavsky, N. J. (2014).** *The rise of oxygen in Earth's early ocean and atmosphere.* Nature 506(7488):307–315. DOI: [10.1038/nature13068](https://doi.org/10.1038/nature13068).

**Abstract** *(source: PubMed, PMID 24553238)*

> The rapid increase of carbon dioxide concentration in Earth's modern atmosphere is a matter of major concern. But for the atmosphere of roughly two-and-half billion years ago, interest centres on a different gas: free oxygen (O2) spawned by early biological production. The initial increase of O2 in the atmosphere, its delayed build-up in the ocean, its increase to near-modern levels in the sea and air two billion years later, and its cause-and-effect relationship with life are among the most compelling stories in Earth's history.

---

### Catling & Zahnle 2020 — the Archean atmosphere

**Catling, D. C. & Zahnle, K. J. (2020).** *The Archean atmosphere.* Science Advances 6(9):eaax1420. DOI: [10.1126/sciadv.aax1420](https://doi.org/10.1126/sciadv.aax1420).

> What was the early atmosphere made of? We review what is known during the Archean eon, 4 to 2.5 billion years ago.

*(Note: that one sentence is the entirety of the abstract Crossref carries for this record. The paper itself is a long review.)*

---

## II-I. Celestial mechanics & long-term orbital forcing

### Laskar et al. 2004 — La2004, the canonical insolation solution

**Laskar, J., Robutel, P., Joutel, F., Gastineau, M., Correia, A. C. M. & Levrard, B. (2004).** *A long-term numerical solution for the insolation quantities of the Earth.* Astronomy & Astrophysics 428(1):261–285. DOI: [10.1051/0004-6361:20041335](https://doi.org/10.1051/0004-6361:20041335).

**Availability:** in relata as `laskar-2004-la2004` — see Part I §J. *(Reproduced here because it is the anchor of this cluster.)*

> We present here a new solution for the astronomical computation of the insolation quantities on Earth spanning from −250 Myr to 250 Myr. This solution has been improved with respect to La93 by using a direct integration of the gravitational equations for the orbital motion, and by improving the dissipative contributions, in particular in the evolution of the Earth–Moon System. The orbital solution has been used for the calibration of the Neogene period, and is expected to be used for age calibrations of paleoclimatic data over 40 to 50 Myr, eventually over the full Palaeogene period (65 Myr) with caution. Beyond this time span, the chaotic evolution of the orbits prevents a precise determination of the Earth's motion. However, the most regular components of the orbital solution could still be used over a much longer time span, which is why we provide here the solution over 250 Myr. Over this time interval, the most striking feature of the obliquity solution, apart from a secular global increase due to tidal dissipation, is a strong decrease of about 0.38 degree in the next few millions of years, due to the crossing of a resonance. For the calibration of the Mesozoic time scale (about 65 to 250 Myr), we propose to use the term of largest amplitude in the eccentricity, with a fixed frequency corresponding to a period of 405 000 yr. The uncertainty of this time scale over 100 Myr should be small, and over the full Mesozoic era it remains bounded.

---

### Zeebe & Lourens 2019 — solar-system chaos constrained by geology

**Zeebe, R. E. & Lourens, L. J. (2019).** *Solar System chaos and the Paleocene–Eocene boundary age constrained by geology and astronomy.* Science 365(6456):926–929. DOI: [10.1126/science.aax0612](https://doi.org/10.1126/science.aax0612).

**Abstract** *(source: Crossref — this is the journal's editor's summary, which is what Crossref carries for this record; the paper's own abstract was not separately obtainable)*

> Filling a dating hole. The periodic nature of Earth's orbit around the Sun produces cycles of insolation reflected in climate records. Conversely, these climate records can be used to infer changes in the dynamics of the Solar System, which is inherently chaotic and not always similarly periodic. A particular obstacle is the lack of well-defined planetary orbital constraints between 50 and 60 million years ago. Zeebe and Lourens found an astronomical solution for that interval showing that the Solar System experienced a specific resonance transition pattern. These data provide a measure of the duration of the Paleocene-Eocene Thermal Maximum.

---

### Rein & Liu 2012 — REBOUND

**Rein, H. & Liu, S.-F. (2012).** *REBOUND: an open-source multi-purpose N-body code for collisional dynamics.* Astronomy & Astrophysics 537:A128. DOI: [10.1051/0004-6361/201118085](https://doi.org/10.1051/0004-6361/201118085).

> REBOUND is a new multi-purpose N-body code which is freely available under an open-source license. It was designed for collisional dynamics such as planetary rings but can also solve the classical N-body problem. It is highly modular and can be customized easily to work on a wide variety of different problems in astrophysics and beyond.

---

### Farhat et al. 2022 — the resonant tidal evolution of the Earth–Moon distance

**Farhat, M., Auclair-Desrotour, P., Boué, G. & Laskar, J. (2022).** *The resonant tidal evolution of the Earth-Moon distance.* Astronomy & Astrophysics 665:L1. DOI: [10.1051/0004-6361/202243445](https://doi.org/10.1051/0004-6361/202243445).

> Due to tidal interactions in the Earth-Moon system, the spin of the Earth slows down and the Moon drifts away. This recession of the Moon can now be measured with great precision, but it was noticed more than fifty years ago that simple tidal models extrapolated back in time lead to an age of the Moon that is largely incompatible with the geochronological and geochemical evidence. In order to evade this problem, more elaborate models have been proposed, taking into account the oceanic tidal dissipation. However, these models have not been able to fit both the estimated lunar age and the present rate of lunar recession simultaneously. In the present work, we present a physical model that reconciles these two constraints and yields a unique solution for the tidal history. This solution fits the available geological proxies for the history of the Earth-Moon system well and it consolidates the cyclostratigraphic method. Our work extends the lineage of earlier works on the analytical treatment of fluid tides on varying bounded surfaces that is further coupled with solid tidal deformations. This allows us to take into account the time-varying continental configuration on Earth by considering hemispherical and global ocean models. The resulting evolution of the Earth-Moon system involves multiple crossings of resonances in the oceanic dissipation that are associated with significant and rapid variations in the lunar orbital distance, the length of an Earth day and the Earth's obliquity.

---

### Waltham 2015 — Milankovitch period uncertainties

**Waltham, D. (2015).** *Milankovitch period uncertainties and their impact on cyclostratigraphy.* Journal of Sedimentary Research 85(8):990–998. DOI: [10.2110/jsr.2015.66](https://doi.org/10.2110/jsr.2015.66).

> Astronomically calibrated cyclostratigraphy relies on correct matching of observed sedimentary cycles to predicted astronomical drivers such as eccentricity, obliquity, and climate precession. However, the periods of these astronomical cycles, in the past, are not perfectly known because: (i) they drift through time; (ii) they overlap; and (iii) they are affected by the poorly constrained recession history of the Moon. This paper estimates the resulting uncertainties in ancient Milankovitch cycle periods and shows that they lead to: (i) problems with using Milankovitch cycles for accurate measurement of durations (potential errors are around 25% by the start of the Phanerozoic); (ii) problems with correctly identifying the Milankovitch cycles responsible for observed period ratios (e.g., the ratio for long-eccentricity/short-eccentricity overlaps, within error, with the ratio for short-eccentricity/precession); and (iii) problems with verifying that observed cycles are Milankovitch driven at all (the probability of a random period ratio matching a predicted Milankovitch ratio, within error, is 20–70% in the Phanerozoic). Milankovitch-derived ages and durations should therefore be treated with caution unless supported by additional information such as radiometric constraints.
---

# Part III — The known-wanted items (the `⊘ unread` list)

*These four are marked `⊘ unread` in `DECISIONS.decision-log.udon` and were not in relata at all. All four bibliographic records are now verified against Crossref; three of the four abstracts were obtained.*

### Coatléven 2020 — the cell-to-cell MFD proof

**Coatléven, Julien (2020).** *Some multiple flow direction algorithms for overland flow on general meshes.* ESAIM: Mathematical Modelling and Numerical Analysis (M2AN) 54(6):1917–1949. DOI: [10.1051/m2an/2020025](https://doi.org/10.1051/m2an/2020025).

**Availability:** **not held anywhere** — not in relata, no PDF. This is reference [11] of Coatléven & Chauveau 2024 (ESurf 12:995), and `DECISIONS[…]` line 479 marks it `⊘ unread`.

**Abstract** *(source: Crossref)*

> After recalling the most classical multiple flow direction algorithms (MFD), we establish their equivalence with a well chosen discretization of Manning–Strickler models for water flow. From this analogy, we derive a new MFD algorithm that remains valid on general, possibly non conforming meshes. We also derive a convergence theory for MFD algorithms based on the Manning–Strickler models. Numerical experiments illustrate the good behavior of the method even on distorted meshes.

---

### Korenaga, Planavsky & Evans 2017 — continental freeboard modelling

**Korenaga, Jun; Planavsky, Noah J.; Evans, David A. D. (2017).** *Global water cycle and the coevolution of the Earth's interior and surface environment.* Philosophical Transactions of the Royal Society A: Mathematical, Physical and Engineering Sciences 375(2094):20150393. DOI: [10.1098/rsta.2015.0393](https://doi.org/10.1098/rsta.2015.0393).

**Availability:** **not held anywhere** — not in relata, no PDF. Named at `DECISIONS.decision-log.udon` lines 529–530 as *"the freeboard-modelling reference to read next. **[⊘] Not read.**"*

**Abstract** *(source: Crossref)*

> The bulk Earth composition contains probably less than 0.3% of water, but this trace amount of water can affect the long-term evolution of the Earth in a number of different ways. The foremost issue is the occurrence of plate tectonics, which governs almost all aspects of the Earth system, and the presence of water could either promote or hinder the operation of plate tectonics, depending on where water resides. The global water cycle, which circulates surface water into the deep mantle and back to the surface again, could thus have played a critical role in the Earth's history. In this contribution, we first review the present-day water cycle and discuss its uncertainty as well as its secular variation. If the continental freeboard has been roughly constant since the Early Proterozoic, model results suggest long-term net water influx from the surface to the mantle, which is estimated to be 3−4.5×10¹⁴ g yr⁻¹ on the billion years time scale. We survey geological and geochemical observations relevant to the emergence of continents above the sea level as well as the nature of Precambrian plate tectonics. The global water cycle is suggested to have been dominated by regassing, and its implications for geochemical cycles and atmospheric evolution are also discussed. This article is part of the themed issue 'The origin, history and role of water in the evolution of the inner Solar System'.

---

### van Hunen & van den Berg 2008 — strength and buoyancy limits on early subduction

**van Hunen, Jeroen; van den Berg, Arie P. (2008).** *Plate tectonics on the early Earth: Limitations imposed by strength and buoyancy of subducted lithosphere.* Lithos 103(1–2):217–235. DOI: [10.1016/j.lithos.2007.09.016](https://doi.org/10.1016/j.lithos.2007.09.016).

**Availability:** **not held anywhere** — not in relata, no PDF. Marked `⊘ unread` at `DECISIONS.decision-log.udon` line 511.

**Abstract:** ABSTRACT NOT RETRIEVED. Crossref, OpenAlex and Semantic Scholar all resolve the record but carry no abstract field; ScienceDirect (403), the Durham institutional repository (403), the Utrecht research portal (metadata only, no abstract), and NASA ADS all failed to yield the verbatim text. The bibliographic record above is confirmed against Crossref.

---

### Johnson et al. 2017 — Earth's first stable continents

**Johnson, Tim E.; Brown, Michael; Gardiner, Nicholas J.; Kirkland, Christopher L.; Smithies, R. Hugh (2017).** *Earth's first stable continents did not form by subduction.* Nature 543(7644):239–242. DOI: [10.1038/nature21383](https://doi.org/10.1038/nature21383). *(Corrigendum: Nature 545:510, DOI [10.1038/nature22385](https://doi.org/10.1038/nature22385).)*

**Availability:** **not held anywhere** — not in relata, no PDF. Marked `⊘ unread` at `DECISIONS.decision-log.udon` line 511.

> **Note on the author list.** The decision log's shorthand gives the authors as *"Johnson, Kirkland, Gardiner, Brown, Smithies, Santosh"*. The verified Crossref author list is **Johnson, Brown, Gardiner, Kirkland, Smithies** — five authors, different order, no Santosh.

**Abstract** *(source: OpenAlex)*

> The geodynamic environment in which Earth's first continents formed and were stabilized remains controversial. Most exposed continental crust that can be dated back to the Archaean eon (4 billion to 2.5 billion years ago) comprises tonalite–trondhjemite–granodiorite rocks (TTGs) that were formed through partial melting of hydrated low-magnesium basaltic rocks; notably, these TTGs have 'arc-like' signatures of trace elements and thus resemble the continental crust produced in modern subduction settings. In the East Pilbara Terrane, Western Australia, low-magnesium basalts of the Coucal Formation at the base of the Pilbara Supergroup have trace-element compositions that are consistent with these being source rocks for TTGs. These basalts may be the remnants of a thick (more than 35 kilometres thick), ancient (more than 3.5 billion years old) basaltic crust that is predicted to have existed if Archaean mantle temperatures were much hotter than today's. Here, using phase equilibria modelling of the Coucal basalts, we confirm their suitability as TTG 'parents', and suggest that TTGs were produced by around 20 per cent to 30 per cent melting of the Coucal basalts along high geothermal gradients (of more than 700 degrees Celsius per gigapascal). We also analyse the trace-element composition of the Coucal basalts, and propose that these rocks were themselves derived from an earlier generation of high-magnesium basaltic rocks, suggesting that the arc-like signature in Archaean TTGs was inherited from an ancestral source lineage. This protracted, multistage process for the production and stabilization of the first continents—coupled with the high geothermal gradients—is incompatible with modern-style plate tectonics, and favours instead the formation of TTGs near the base of thick, plateau-like basaltic crust. Thus subduction was not required to produce TTGs in the early Archaean eon.
---

# Part IV — Records that need fixing (found while compiling)

*Factual defects in the stored records, noted so they can be corrected. Not commentary on the works.*

**Duplicate relata keys (same DOI, two entries):**

- `ferguson-2016-amr-cubed` and `ferguson-2016-analyzing` are **the same paper** — both carry DOI `10.1175/mwr-d-16-0197.1` (Ferguson et al., *Analyzing the Adaptive Mesh Refinement (AMR) Characteristics of a High-Order 2D Cubed-Sphere Shallow-Water Model*, Mon. Wea. Rev. 144:4641–4666). Both are cited across the repo.

**Probable duplicate (preprint + published):**

- `coatlven-2023-large` (no DOI, 2023, *"Large **structures** simulation…"*) appears to be the preprint of `coatleven-2024-large` (DOI `10.5194/esurf-12-995-2024`, 2024, *"Large **structure** simulation…"*). The key also carries a typo (`coatlven` ← `coatleven`).

**Year mismatches between the relata key and the published record:**

| key | key says | published record says |
|---|---|---|
| `aechtner-2015-wavelet-sphere` | 2015 | Crossref/relata year field: **2014** (QJRMS 141:1712–1726, issue 690) |
| `cordonnier-2018-versatile` | 2018 | **2019** (Earth Surf. Dynam. 7:549–562) |
| `wintermeyer-2016-entropy-swe` | 2016 | published **2017** (JCP 340:200–242); arXiv v2 is 2016 |
| `fjordholm-2017-measure-valued` | 2017 | relata year field: **2015** (Found. Comput. Math.) |
| `ishimwe-2024-multi-scale` | 2024 | **2025** issue (JCP 520:113482) |
| `liu-2024-robust` | 2024 | **2025** issue (JCP 520:113485) |
| `karmakar-2025-generalised` | 2025 | **2026** issue (JCP 548:114560) |
| `gorski-2004-healpix` | 2004 | published **2005** (ApJ 622:759–771, DOI 10.1086/427976); relata holds the 2004 arXiv version (astro-ph/0409513), which is what `ref/research/pdfs/astro-ph0409513.pdf` is |
| `rein-2015-ias15` | 2015 | relata year field: **2014** |
| `tamayo-2020-reboundx` | 2020 | relata year field: **2019** |
| `staniforth-2012-grids` | 2012 | relata year field: **2011** (QJRMS 138(662):1–26, 2012 issue) |
| `ranocha-2017-swe-allthree` | 2017 | relata year field: **2016** |

**Works held in the repo but absent from relata entirely:**

- **Youngren, R. W. & Petty, M. D. (2017).** *A multi-resolution HEALPix data structure for spherically mapped point data.* Heliyon 3(6):e00332. DOI `10.1016/j.heliyon.2017.e00332`. Held as `ref/research/pdfs/1-s2.0-S2405844017304966-main.pdf` **and markdown-converted**, but has no relata record.

**Works cited repeatedly in the repo but absent from relata entirely:**

- **Snyder, J. P. (1992).** *An Equal-Area Map Projection For Polyhedral Globes.* Cartographica 29(1):10–21. DOI `10.3138/27H7-8K88-4882-1752`. Cited by name and by Table 1 in `DECISIONS.decision-log.udon` (grid work, seam-adjacency findings, grid-comparison report). Not in relata; no PDF held. Its record and abstract are in Part I §D.

**Empty directory referenced by the decision log:**

- `msc/research-lem-sota/` is referenced by four `|ref` lines in `DECISIONS.decision-log.udon` but is **empty and untracked** — it contains no files and appears in no git index. The bibkeys those lines cite (`coatleven-2024-large`, `prescott-2025-evaluation`, `bonetti-2018-theory`, `armitage-2019-short`, `coatleven-2025-postprocessing`) do all exist in relata.

**Two PDFs at the `ref/` root, not under a `pdfs/` directory:**

- `ref/gmd-19-5343-2026.pdf` = Duretz et al. 2026, GMD 19:5343 (`duretz-2026-automatic`)
- `ref/gmd-19-5601-2026.pdf` = Kern et al. 2026, GMD 19:5601 (`kern-2026-simultaneous`)

Both are in relata; both are filed outside the `ref/**/pdfs/` convention used elsewhere.
