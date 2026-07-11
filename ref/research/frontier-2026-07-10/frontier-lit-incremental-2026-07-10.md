# Incremental simulation and deterministic world models

##### [**Undermind**](https://undermind.ai)

---

**Research Goal:** Find scholarly literature most relevant to an architecture for deterministic, memoized, content-addressed world simulation with multiscale updates. Prioritize primary sources from 2023–2026, especially papers and preprints, and focus on frontier work rather than re-describing known foundations such as Salsa, Adapton, self-adjusting computation, Nix or Bazel style content addressing, Build Systems à la Carte, and stateless hashed randomness. The main target is the sharp open problem of detail-to-abstract upscaling: when an irreducible discrete local edit such as a dammed stream or placed structure occurs, how can its effect be conservatively lifted back into a memoized macro representation with correct upward invalidation and recomputation? Search for work on two-way or bidirectional grid coupling, conservative upscaling of local perturbations, concurrent global-local or multiscale model coupling, reversible or invertible simulation caching, memoized simulation under edits, and any computation systems that propagate changes upward rather than only downstream. Also cover adjacent threads on determinism and procedural generation at scale, including post-Gabor procedural noise, deterministic or splittable hashing and random generation for massively parallel procedural worlds, and recent planet-scale procedural techniques, plus hybrid symbolic-neural or LLM-driven simulation agents where a language model perturbs structured numeric or symbolic state rather than directly acting in the world, including cognitive level-of-detail, believable-agent or social-simulation work, and any literature on identifiability or system identification of adaptive agents under perturbation. Rank results by how directly they bear on this architecture, and treat credible negatives as valuable outcomes: if the exact detail-to-abstract problem appears genuinely unsolved, say so explicitly. Be honest about dates and uncertainty, and explicitly flag items published after January 2026 because a collaborator’s knowledge is stale beyond that point.

*Found 38 papers · July 10, 2026 · Estimated coverage of relevant papers: 80%*

## Summary of Results

No retrieved 2023–2026 source directly solves the core problem of conservative detail-to-abstract lifting after an irreducible local edit in a memoized deterministic simulator; the nearest frontier is equation-free/patch-dynamics multiscale coupling, which advances sparse microscale patches while reconstructing macroscale state, but mainly addresses downward closure and local recomputation rather than general upward invalidation of a cached macro world model \[4, 5, 3, 2, 6, 1\].

#### Closest direct threads

- **Patch dynamics / equation-free multiscale simulation**: sparse-patch execution with explicit micro↔macro coupling, including shocks and heterogeneous media \[4, 5, 3, 12, 10\].
- **Perturbation-aware upscaling**: reuse of reference local solves under local defects \[9\], runtime coarsening under local material/topology updates \[13\], and adaptive multiscale basis updates when local flow changes \[18, 27, 24\].
- **Conservative coarse/fine transfer**: field-conserving AMR coarsening directly targets child→parent conservation during coarsening \[14\]; **published after January 2026 knowledge cutoff**.

#### What is still missing

- No paper here gives a content-addressed, memoized simulation architecture with edit-local invalidation that propagates **upward** through abstractions.
- No clear reversible/invertible simulation-cache literature appears in the retrieved set.
- Hybrid symbolic-neural/LLM world-perturbation work and believable-agent/social-simulation identifiability are effectively absent from these results.

#### Adjacent systems work

- Differential or updateable DES shows exact recomputation under small scenario edits \[8, 21\].
- Simulation cloning and out-of-order/causally correct DES improve reuse/parallelism, but not multiscale abstraction maintenance \[17, 25, 19\].
- Deterministic parallel randomness remains anchored in older splittable/pedigree-style work \[35, 36\].

## Paper Catalog (38 papers)

|  | Year | Cit/yr | Title | Authors | Journal |
|---:|:--:|:--:|:---|:---|:---|
| 1 | 2025 | 1.7 | Generalised patch dynamics schemes in equation-free multiscale modelling ([link](https://doi.org/10.1016/j.jcp.2025.114560)) | T. K. Karmakar and D. C. Dalal | J. Comput. Phys. |
| 2 | 2024 |  | A Generalised Curvilinear Coordinate system-based Patch Dynamics Scheme in Equation-free Multiscale Modelling ([link](https://www.semanticscholar.org/paper/75524c9fc18189959d95f0edaa2e47e36bfde237)) | T. K. Karmakar and D. C. Dalal |  |
| 3 | 2024 | 2.5 | Efficient prediction of static and dynamical responses of functional graded beams using sparse multiscale patches ([link](https://doi.org/10.1007/s00466-025-02614-4)) | T. Tran-Duc, J. Bunder, and A. Roberts | Computational Mechanics |
| 4 | 2023 | 1.2 | Accurate and efficient multiscale simulation of a heterogeneous elastic beam via computation on small sparse patches ([link](https://doi.org/10.48550/arXiv.2301.13145)) | A. Roberts, T. Tran-Duc, J. Bunder, and Y. Kevrekidis | ArXiv |
| 5 | 2023 | 1.7 | Efficient computational homogenisation of 2D beams of heterogeneous elasticity using the patch scheme ([link](https://doi.org/10.48550/arXiv.2308.09226)) | T. Tran-Duc, J. Bunder, and A. Roberts | ArXiv |
| 6 | 2024 |  | Generalized Patch Dynamics Scheme in Equation-free Multiscale Modelling ([link](https://www.semanticscholar.org/paper/15bef289901b62472fdec22c5034b7562cab203e)) | T. K. Karmakar and D. C. Dalal |  |
| 7 | 2025 |  | Scalable learning of macroscopic stochastic dynamics ([link](https://doi.org/10.48550/arXiv.2511.12842)) | Mengyi Chen, Pengru Huang, K. Novoselov, and Qianxiao Li | ArXiv |
| 8 | 2019 | 0.1 | Exact-Differential Simulation ([link](https://doi.org/10.1145/3301499)) | Masatoshi Hanai, T. Suzumura, Elvis S. Liu, G. Theodoropoulos, and K. Perumalla | ACM Transactions on Modeling and Computer Simulation (TOMACS) |
| 9 | 2019 | 1.9 | Numerical upscaling of perturbed diffusion problems ([link](https://doi.org/10.1137/19m1278211)) | Fredrik Hellman, Tim Keil, and A. Målqvist | SIAM J. Sci. Comput. |
| 10 | 2021 | 0.9 | An Equation Free Algorithm Accurately Simulates Macroscale Shocks Arising From Heterogeneous Microscale Systems ([link](https://doi.org/10.1109/JMMCT.2021.3054012)) | J. Maclean, J. Bunder, I. Kevrekidis, and A. Roberts | IEEE Journal on Multiscale and Multiphysics Computational Techniques |
| 11 | 2020 | 1.7 | Equation-free patch scheme for efficient computational homogenisation via self-adjoint coupling ([link](https://doi.org/10.1007/s00211-021-01232-5)) | J. Bunder, I. Kevrekidis, and A. J. Roberts | Numerische Mathematik |
| 12 | 2021 | 0.8 | Adaptively detect and accurately resolve macro-scale shocks in an efficient Equation-Free multiscale simulation ([link](https://doi.org/10.1137/21m1437172)) | J. Maclean, J. Bunder, I. Kevrekidis, and A. Roberts | SIAM J. Sci. Comput. |
| 13 | 2022 |  | Fast Numerical Coarsening with Local Factorizations ([link](https://doi.org/10.1111/cgf.14619)) | Zhong-Quan He, Jesús Pérez, and M. Otaduy | Computer Graphics Forum |
| 14 | 2026 |  | Field conserving adaptive mesh refinement (AMR) scheme on massively parallel adaptive octree meshes ([link](https://doi.org/10.48550/arXiv.2602.07817)) | K. Saurabh, Makrand A. Khanwale, Masado Ishii, Hari Sundar, and B. Ganapathysubramanian | ArXiv |
| 15 | 2014 | 1.1 | On the acceleration of spatially distributed agent-based computations ([link](https://doi.org/10.1016/J.APNUM.2014.12.007)) | Ping Liu, G. Samaey, C. Gear, and I. Kevrekidis | Applied Numerical Mathematics |
| 16 | 2024 | 1.5 | Construct accurate multi-continuum micromorphic homogenisations in multi-D space-time with computer algebra ([link](https://doi.org/10.48550/arXiv.2407.03483)) | A. J. Roberts | ArXiv |
| 17 | 2024 | 0.7 | SpecSims: A Scalable Speculative Tree-based Simulation Cloning Framework for Finite Memory Machines ([link](https://doi.org/10.1145/3708885)) | Srikanth B. Yoginath, Pratishtha Shukla, James J. Nutaro, and Sudip K. Seal | ACM Transactions on Modeling and Computer Simulation |
| 18 | 2005 | 12 | Adaptive Multiscale Finite-Volume Method for Multiphase Flow and Transport in Porous Media ([link](https://doi.org/10.1137/030600795)) | P. Jenny, Seong-Hyeok Lee, and H. Tchelepi | Multiscale Model. Simul. |
| 19 | 2025 | 1.9 | Out of Order and Causally Correct: Ready-Event Discovery through Data-Dependence Analysis ([link](https://doi.org/10.1145/3726301.3728416)) | Erik J. Jensen, James F. Leathrum, Christopher J. Lynch, Katherine Smith, and Ross J. Gore | 39th ACM SIGSIM Conference on Principles of Advanced Discrete Simulation |
| 20 | 2024 | 1.1 | Robust treatment for the coarse/fine interface of adaptive mesh in the simulation of two-phase flow ([link](https://doi.org/10.1016/j.jcp.2024.113485)) | Cheng Liu, Yiding Hu, Ruoqing Gao, and Changhong Hu | J. Comput. Phys. |
| 21 | 2001 |  | Updateable Simulations ([link](https://www.semanticscholar.org/paper/0636fd1b1a23ce3cd4d9bdea23d30012e90cda8c)) |  |  |
| 22 | 2017 | 0.7 | A Feature-Enriched Multiscale Method for Simulating Complex Geomodels ([link](https://doi.org/10.2118/182701-MS)) | Knut-Andreas Lie, O. Møyner, and J. Natvig |  |
| 23 | 2024 | 0.6 | Exact Solutions and Upscaling for 1D Two‐Phase Flow in Heterogeneous Porous Media ([link](https://doi.org/10.1029/2024WR037917)) | K. O. Prempeh, Parker William George, and Pavel Bedrikovetsky | Water Resources Research |
| 24 | 2019 | 1.7 | Accelerating multiscale simulation of complex geomodels by use of dynamically adapted basis functions ([link](https://doi.org/10.1007/s10596-019-9827-z)) | Ø. Klemetsdal, O. Møyner, and Knut-Andreas Lie | Computational Geosciences |
| 25 | 2024 |  | Out-of-Order Discrete Event Simulation: Fighting Memory Boundedness while Running DES Models ([link](https://doi.org/10.1109/DS-RT62209.2024.00024)) | Romolo Marotta and Francesco Quaglia | 2024 28th International Symposium on Distributed Simulation and Real Time Applications (DS-RT) |
| 26 | 2006 | 0.2 | A SPECIALIZED UPSCALING METHOD FOR ADAPTIVE GRIDS: TIGHT INTEGRATION OF LOCAL-GLOBAL UPSCALING AND ADAPTIVITY LEADS TO ACCURATE SOLUTION OF FLOW IN HETEROGENEOUS FORMATIONS ([link](https://www.semanticscholar.org/paper/005831318e3f40a261d680496b63759246a5a2d7)) | J. Lambers |  |
| 27 | 2005 | 0.6 | An Adaptive Multiphase Multiscale Finite Volume Simulator for Heterogeneous Reservoirs ([link](https://doi.org/10.2118/93395-MS)) | H. Tchelepi, P. Jenny, Seong H. Lee, and Christian Wolfsteiner |  |
| 28 | 2016 | 11 | A multiscale restriction-smoothed basis method for high contrast porous media represented on unstructured grids ([link](https://doi.org/10.1016/j.jcp.2015.10.010)) | O. Møyner and Knut-Andreas Lie | J. Comput. Phys. |
| 29 | 2015 | 0.2 | Interactive Simulations Using Localized Reduced Basis Methods ([link](https://doi.org/10.1016/J.IFACOL.2015.05.134)) | A. Buhr and Mario Ohlberger | IFAC-PapersOnLine |
| 30 | 2022 | 1.4 | Two novel families of multiscale staggered patch schemes efficiently simulate large-scale, weakly damped, linear waves ([link](https://doi.org/10.1016/j.cma.2023.116133)) | J. Divahar, A. Roberts, T. W. Mattner, J. Bunder, and I. Kevrekidis | ArXiv |
| 31 | 2019 | 0.5 | Multiscale gradient computation for multiphase flow in porous media ([link](https://www.semanticscholar.org/paper/28f793b3f7cb3da5f3b99152c4ddc1853e321127)) | J. Moraes, H. Hajibeygi, Jan Dirk, and Doi |  |
| 32 | 2016 | 1.0 | Quasi-matrix-free Hybrid Multigrid on Dynamically Adaptive Cartesian Grids ([link](https://doi.org/10.1145/3165280)) | Marion Weinzierl and T. Weinzierl | ACM Transactions on Mathematical Software (TOMS) |
| 33 | 1996 | 4.1 | A Projection Method for Locally Refined Grids ([link](https://doi.org/10.1006/JCPH.1996.0166)) | M. Minion | Journal of Computational Physics |
| 34 | 2017 | 0.1 | Multiscale Gradient Computation for Multiphase Flow in Porous Media ([link](https://doi.org/10.2118/182625-MS)) | R. Moraes, J. Rodrigues, H. Hajibeygi, and Jan Dirk Jansen |  |
| 35 | 2012 | 3.9 | Deterministic parallel random-number generation for dynamic-multithreading platforms ([link](https://doi.org/10.1145/2145816.2145841)) | C. Leiserson, T. Schardl, and Jim Sukha | ACM SIGPLAN Symposium on Principles & Practice of Parallel Programming |
| 36 | 2014 | 4.9 | Fast splittable pseudorandom number generators ([link](https://doi.org/10.1145/2714064.2660195)) | G. Steele, D. Lea, and Christine H. Flood | ACM SIGPLAN Notices |
| 37 | 2014 | 0.9 | Construction of Multiscale Preconditioners on Stratigraphic Grids ([link](https://doi.org/10.3997/2214-4609.20141775)) | O. Møyner |  |
| 38 | 2010 |  | Conservative adaptivity and two-way self-nesting using discrete wavelets ([link](https://www.semanticscholar.org/paper/1844a4daedf3dddaee80b8d553e82b9ed6219e33)) | T. Dubos |  |

### Paper Details

1\. · 100% match · 2025 · 1.7 cit/yr\
**Generalised patch dynamics schemes in equation-free multiscale modelling** ([link](https://doi.org/10.1016/j.jcp.2025.114560))\
T. K. Karmakar and D. C. Dalal\
*J. Comput. Phys.* · Dec 1, 2025 · 1 citations

------------------------------------------------------------------------

2\. · 100% match · 2024\
**A Generalised Curvilinear Coordinate system-based Patch Dynamics Scheme in Equation-free Multiscale Modelling** ([link](https://www.semanticscholar.org/paper/75524c9fc18189959d95f0edaa2e47e36bfde237))\
T. K. Karmakar and D. C. Dalal\
May 14, 2024 · 0 citations

> The patch dynamics scheme in equation-free multiscale modelling has the potential to efficiently predict the macroscopic behaviours by simulating the microscale problem in a fraction of the space-time domain. The patch dynamics schemes developed so far are mainly on rectangular domains with uniform grids and uniform rectangular patches. In real-life problems, the geometry of the domain is not regular or simple, where rectangular and uniform grids or patches may not be useful. To address this kind of complexity, for the first time, a generalised orthogonal curvilinear coordinate system is employed in the patch dynamics scheme, applicable to both rectangular domains with non-uniform grids and non-rectangular domains; while applying this, the concept of non-uniform and non-rectangular patch configurations in the physical domain is also adopted for the first time. An explicit representation of a patch dynamics scheme on a generalised curvilinear coordinate system in a two-dimensional domain is proposed for unsteady, linear, heterogeneous convection-diffusion-reaction (CDR) problems. The proposed scheme is validated through heterogeneous convection-diffusion-reaction and non-axisymmetric diffusion problems on generalised curvilinear coordinate systems. The results demonstrate excellent accuracy and show that the method significantly outperforms full-domain simulations in terms of computational efficiency, memory usage and overall performance.

------------------------------------------------------------------------

3\. · 100% match · 2024 · 2.5 cit/yr\
**Efficient prediction of static and dynamical responses of functional graded beams using sparse multiscale patches** ([link](https://doi.org/10.1007/s00466-025-02614-4))\
T. Tran-Duc, J. Bunder, and A. Roberts\
*Computational Mechanics* · Jul 16, 2024 · 5 citations

> We develop a multiscale patch scheme for studying the system level characteristics of heterogeneous functional graded beams in 3D via accurate computational homogenisation. The algorithm is an extension of our previous work for 2D beams (Tran-Duc et al. in Int. J. Solids Struct. 292:112719, 2024) to explore out-of-plane dynamics of 3D beams of functional graded materials. The scheme computes the detailed microscale elastic equations only in sparsely spaced, small patches of the domain (akin to fe2\documentclass\[12pt\]{minimal} \usepackage{amsmath} \usepackage{wasysym} \usepackage{amsfonts} \usepackage{amssymb} \usepackage{amsbsy} \usepackage{mathrsfs} \usepackage{upgreek} \setlength{\oddsidemargin}{-69pt} \begin{document}
> ``` math
>  ^2
> ```
> \end{document}), and via symmetry-preserving interpolation between these patches. We develop new applications of the scheme to two classes of functionally graded beams, namely cross-sectionally graded and axially graded. Our approach accurately and provably predicts the macroscale system-wide behaviour. Beam deflection and natural frequencies from the patch computations agree very well with both existing experimental data and the full-domain computations, which provides a new validation of the approach and a new characterisation of the interaction between bending and twisting in graduated beams. The scheme is stable and robust, with errors consistently small and controllable by varying the number of patches. The reduction in the spatial domain of computation substantially improves the computational efficiency, with the computational time reducing by a factor of up to 17\documentclass\[12pt\]{minimal} \usepackage{amsmath} \usepackage{wasysym} \usepackage{amsfonts} \usepackage{amssymb} \usepackage{amsbsy} \usepackage{mathrsfs} \usepackage{upgreek} \setlength{\oddsidemargin}{-69pt} \begin{document}
> ``` math
> 17
> ```
> \end{document} when the patches cover 27%\documentclass\[12pt\]{minimal} \usepackage{amsmath} \usepackage{wasysym} \usepackage{amsfonts} \usepackage{amssymb} \usepackage{amsbsy} \usepackage{mathrsfs} \usepackage{upgreek} \setlength{\oddsidemargin}{-69pt} \begin{document}
> ``` math
> 27\%
> ```
> \end{document} of the beam. The scheme also accurately predicts the homogenised dynamics of periodic micro-structured materials, such as metamaterials, by simply ensuring patches are a multiple of the micro-period. Localised phenomena, such as material failures or cracks or boundary layers, may also be accurately encompassed by fully resolving them within a patch.

------------------------------------------------------------------------

4\. · 100% match · 2023 · 1.2 cit/yr\
**Accurate and efficient multiscale simulation of a heterogeneous elastic beam via computation on small sparse patches** ([link](https://doi.org/10.48550/arXiv.2301.13145))\
A. Roberts, T. Tran-Duc, J. Bunder, and Y. Kevrekidis\
*ArXiv* · Jan 20, 2023 · 4 citations

> Modern \`smart’ materials have complex microscale structure, often with unknown macroscale closure. The Equation-Free Patch Scheme empowers us to non-intrusively, efficiently, and accurately simulate over large scales through computations on only small well-separated patches of the microscale system. Here the microscale system is a solid beam of random heterogeneous elasticity. The continuing challenge is to compute the given physics on just the microscale patches, and couple the patches across un-simulated macroscale space, in order to establish efficiency, accuracy, consistency, and stability on the macroscale. Dynamical systems theory supports the scheme. This research program is to develop a systematic non-intrusive approach, both computationally and analytically proven, to model and compute accurately macroscale system levels of general complex physical and engineering systems. References
>
> R. A. Biezemans, C. Le Bris, F. Legoll, and A. Lozinski. Non-intrusive implementation of a wide variety of Multiscale Finite Element Methods. Comptes Rendus. Mécanique 351 (2023), pp. 1–46. doi: 10.5802/crmeca.178 M. P. Brenner and P. Koumoutsakos. Editorial: Machine learning and Physical Review Fluids: An editorial perspective. Phys. Rev. Fluids 6.7 (2021), p. 070001. doi: 10.1103/PhysRevFluids.6.070001 J. E. Bunder, I. G. Kevrekidis, and A. J. Roberts. Equation-free patch scheme for efficient computational homogenisation via self-adjoint coupling. Numer. Math. 149.2 (2021), pp. 229–272. doi: 10.1007/s00211-021-01232-5 J. E. Bunder, A. J. Roberts, and I. G. Kevrekidis. Good coupling for the multiscale patch scheme on systems with microscale heterogeneity. J. Comput. Phys. 337 (2017), pp. 154–174. doi: 10.1016/j.jcp.2017.02.004 References C175 M. Cao and A. J. Roberts. Multiscale modelling couples patches of nonlinear wave-like simulations. IMA J. Appl. Math. 81.2 (2016), pp. 228–254. doi: 10.1093/imamat/hxv034 J. Divahar, A. J. Roberts, T. W. Mattner, J. E. Bunder, and I. G. Kevrekidis. Two novel families of multiscale staggered patch schemes efficiently simulate large-scale, weakly damped, linear waves. Comput. Meth. Appl. Mech. Eng. 413 (2023), p. 116133. doi: 10.1016/j.cma.2023.116133. (Cit. on pp. C163, C165, C172). S. Lucarini, M. V. Upadhyay, and J. Segurado. FFT based approaches in micromechanics: fundamentals, methods and applications. Model. Sim. Mat. Sci. Eng. 30.2 (2021), p. 023002. doi: 10.1088/1361-651X/ac34e1 J. Maclean, J. E. Bunder, and A. J. Roberts. A toolbox of Equation-Free functions in Matlab/Octave for efficient system level simulation. Numer. Alg. 87 (2021), pp. 1729–1748. doi: 10.1007/s11075-020-01027-z J. Maclean, J. E. Bunder, I. G. Kevrekidis, and A. J. Roberts. An equation free algorithm accurately simulates macroscale shocks arising from heterogeneous microscale systems. IEEE J. Multiscale Multiphys. Comput. Tech. 6 (2021), pp. 8–15. doi: 10.1109/JMMCT.2021.3054012 A. J. Majda and I. Grooms. New perspectives on superparameterization for geophysical turbulence. J. Comput. Phys. Frontiers in Computational Physics 271 (2014), pp. 60–77. doi: 10.1016/j.jcp.2013.09.014 K. Matouš, M. G. D. Geers, V. G. Kouznetsova, and A. Gillman. A review of predictive nonlinear theories for multiscale modeling of heterogeneous materials. J. Comput. Phys. 330 (2017), pp. 192–220. doi: 10.1016/j.jcp.2016.10.070 K. Raju, T.-E. Tay, and V. B. C. Tan. A review of the FE2 method for composites. Multiscale Multidisc. Model. Exp. Design 4 (2021), pp. 1–24. doi: 10.1007/s41939-020-00087-x A. J. Roberts. Macroscale, slowly varying, models emerge from the microscale dynamics in long thin domains. IMA J. Appl. Math. 80.5 (2015), pp. 1492–1518. doi: 10.1093/imamat/hxv004 A. J. Roberts and I. G. Kevrekidis. General tooth boundary conditions for equation free modelling. SIAM J. Sci. Comput. 29.4 (2007), pp. 1495–1510. doi: 10.1137/060654554 A. J. Roberts, T. MacKenzie, and J. Bunder. A dynamical systems approach to simulating macroscale spatial dynamics in multiple dimensions. J. Eng. Math. 86.1 (2014), pp. 175–207. doi: 10.1007/s10665-013-9653-6 A. J. Roberts, J. Maclean, and J. E. Bunder. Equation-Free function toolbox for Matlab/Octave. Tech. rep. https://github.com/uoa1184615/EquationFreeGit, 2019–2024 G. Samaey, A. J. Roberts, and I. G. Kevrekidis. Equation-free computation: An overview of patch dynamics. Multiscale methods: bridging the scales in science and engineering. Ed. by J. Fish. Oxford University Press, 2010. Chap. 8, pp. 216–246. doi: 10.1093/acprof:oso/9780199233854.003.0008 J. Somnic and B. W. Jo. Status and challenges in homogenization methods for lattice materials. Materials 15.2 (2022), p. 605. doi: 10.3390/ma15020605 H. Whitney. Differentiable manifolds. Annal. Math. 37.3 (1936), pp. 645–680. doi: 10.2307/1968482

------------------------------------------------------------------------

5\. · 100% match · 2023 · 1.7 cit/yr\
**Efficient computational homogenisation of 2D beams of heterogeneous elasticity using the patch scheme** ([link](https://doi.org/10.48550/arXiv.2308.09226))\
T. Tran-Duc, J. Bunder, and A. Roberts\
*ArXiv* · Aug 18, 2023 · 5 citations

> Modern ‘smart’ materials have complex heterogeneous microscale structure, often with unknown macroscale closure but one we need to realise for large scale engineering and science. The multiscale Equation-Free Patch Scheme empowers us to non-intrusively, efficiently, and accurately predict the large scale, system level, solutions through computations on only small sparse patches of the given detailed microscale system. Here the microscale system is that of a 2D beam of heterogeneous elasticity, with either fixed fixed, fixed-free, or periodic boundary conditions. We demonstrate that the described multiscale Patch Scheme simply, efficiently, and stably predicts the beam’s macroscale, with a controllable accuracy, at finite scale separation. Dynamical systems theory supports the scheme. This article points the way for others to use this systematic non-intrusive approach, via a developing toolbox of functions, to model and compute accurately macroscale system-levels of general complex physical and engineering systems.

------------------------------------------------------------------------

6\. · 100% match · 2024\
**Generalized Patch Dynamics Scheme in Equation-free Multiscale Modelling** ([link](https://www.semanticscholar.org/paper/15bef289901b62472fdec22c5034b7562cab203e))\
T. K. Karmakar and D. C. Dalal\
Jan 15, 2024 · 0 citations

> There is a class of problems that exhibit smooth behavior on macroscopic scales, where only a microscopic evolution law is known. Patch dynamics scheme of \`equation-free multiscale modelling’ is one of the techniques, which aims to extract the macroscopic information using such known time-dependent microscopic model simulation in patches (which is a fraction of the space-time domain) that reduces the computational complexity. Here, extrapolation time step has an important role to reduce the error at macroscopic level. In this study, a generalized patch dynamics (GPD) scheme is proposed by distributing the gap-tooth timesteppers (GTTs) within each long (macroscopic) time step. This distribution is done in two ways, namely, GPD schemes of type-I and type-II. The proposed GPD scheme is based on three different time scales namely, micro, meso and macro to predict the system level behaviours. The GPD scheme of both types are capable of providing better accuracy with less computation time compared to the usual patch dynamics (UPD) scheme. The physical behaviours of the problems can be more appropriately addressed by the GPD scheme as one may use a non-uniform (variable) distribution of gap-tooth timesteppers (GTTs), as well as the extrapolation times based on the physics of the problem. Where the UPD scheme fails to converge for a long extrapolation time, both types of GPD schemes can be successfully applied. The whole method has been analyzed successfully for the one-dimensional reaction-diffusion problem.

------------------------------------------------------------------------

7\. · 100% match · 2025\
**Scalable learning of macroscopic stochastic dynamics** ([link](https://doi.org/10.48550/arXiv.2511.12842))\
Mengyi Chen, Pengru Huang, K. Novoselov, and Qianxiao Li\
*ArXiv* · Nov 17, 2025 · 0 citations

> Macroscopic dynamical descriptions of complex physical systems are crucial for understanding and controlling material behavior. With the growing availability of data and compute, machine learning has become a promising alternative to first-principles methods to build accurate macroscopic models from microscopic trajectory simulations. However, for spatially extended systems, direct simulations of sufficiently large microscopic systems that inform macroscopic behavior are prohibitive. In this work, we propose a framework that learns the macroscopic dynamics of large stochastic microscopic systems using only small-system simulations. Our framework employs a partial evolution scheme to generate training data pairs by evolving large-system snapshots within local patches. We subsequently derive the closure variables associated with the macroscopic observables and learn the macroscopic dynamics using a custom loss. Furthermore, we introduce a hierarchical upsampling scheme that enables the efficient generation of large-system snapshots from small-system snapshots. We empirically demonstrate the accuracy and robustness of our framework through a variety of stochastic spatially extended systems, including those described by stochastic partial differential equations, idealized lattice spin systems, and a more realistic NbMoTa alloy system.

------------------------------------------------------------------------

8\. · 100% match · 2019 · 0.1 cit/yr\
**Exact-Differential Simulation** ([link](https://doi.org/10.1145/3301499))\
Masatoshi Hanai, T. Suzumura, Elvis S. Liu, G. Theodoropoulos, and K. Perumalla\
*ACM Transactions on Modeling and Computer Simulation (TOMACS)* · Jun 18, 2019 · 1 citations

> Using computer simulation to analyze large-scale discrete event systems requires repeated executions with various scenarios or parameters. Such repeated executions can induce significant redundancy in event processing when the modification from a prior scenario to a new scenario is relatively minor, and when the altered scenario influences only a small part of the simulation. For example, in a city-scale traffic simulation, an altered scenario of blocking one junction may only affect a small part of the city for considerable length of time. However, traditional simulation approaches would still repeat the simulation for the whole city even when the changes are minor. In this article, we propose a new redundancy reduction technique for large-scale discrete event simulations, called exact-differential simulation, which simulates only the altered portions of scenarios and their influences in repeated executions while still achieving the same results as the re-execution of entire simulations. This article presents the main concepts of the exact-differential simulation, the design of its algorithm, and an approach to build an exact-differential simulation middleware that supports multiple applications of discrete event simulation. We also evaluate our approach by using two case studies, PHOLD benchmark and a traffic simulation of Tokyo.

------------------------------------------------------------------------

9\. · 100% match · 2019 · 1.9 cit/yr\
**Numerical upscaling of perturbed diffusion problems** ([link](https://doi.org/10.1137/19m1278211))\
Fredrik Hellman, Tim Keil, and A. Målqvist\
*SIAM J. Sci. Comput.* · Aug 1, 2019 · 13 citations

> In this paper we study elliptic partial differential equations with rapidly varying diffusion coefficient that can be represented as a perturbation of a reference coefficient. We develop a numerical method for efficiently solving multiple perturbed problems by reusing local computations performed with the reference coefficient. The proposed method is based on the Petrov–Galerkin Localized Orthogonal Decomposition (PG-LOD) which allows for straightforward parallelization with low communcation overhead and memory consumption. We focus on two types of perturbations: local defects which we treat by recomputation of multiscale shape functions and global mappings of a reference coefficient for which we apply the domain mapping method. We analyze the proposed method for these problem classes and present several numerical examples.

------------------------------------------------------------------------

10\. · 100% match · 2021 · 0.9 cit/yr\
**An Equation Free Algorithm Accurately Simulates Macroscale Shocks Arising From Heterogeneous Microscale Systems** ([link](https://doi.org/10.1109/JMMCT.2021.3054012))\
J. Maclean, J. Bunder, I. Kevrekidis, and A. Roberts\
*IEEE Journal on Multiscale and Multiphysics Computational Techniques* · 5 citations

> Scientists and engineers often create accurate, trustworthy, computational simulation schemes—but all too often these are too computationally expensive to execute over the time or spatial domain of interest. The equation-free approach is to marry such trusted simulations to a framework for numerical macroscale reduction—the patch dynamics scheme. This article extends the patch scheme to scenarios in which the trusted simulation resolves abrupt state changes on the microscale that appear as shocks on the macroscale. Accurate simulation for problems in these scenarios requires capturing the shock within a novel patch, and also modifying the patch coupling rules in the vicinity in order to maintain accuracy. With these two extensions to the patch scheme, straightforward arguments derive consistency conditions that match the usual order of accuracy for patch schemes. The new scheme is successfully tested to simulate a heterogeneous microscale partial differential equation. This technique will empower scientists and engineers to accurately and efficiently simulate, over large spatial domains, multiscale multiphysics systems that have rapid transition layers on the microscale.

------------------------------------------------------------------------

11\. · 99% match · 2020 · 1.7 cit/yr\
**Equation-free patch scheme for efficient computational homogenisation via self-adjoint coupling** ([link](https://doi.org/10.1007/s00211-021-01232-5))\
J. Bunder, I. Kevrekidis, and A. J. Roberts\
*Numerische Mathematik* · Jul 14, 2020 · 10 citations

> Equation-free macroscale modelling is a systematic and rigorous computational methodology for efficiently predicting the dynamics of a microscale complex system at a desired macroscale system level. In this scheme, a given microscale model is computed in small patches spread across the space-time domain, with patch coupling conditions bridging the unsimulated space. For accurate predictions, care must be taken in designing the patch coupling conditions. Here we construct novel coupling conditions which preserve self-adjoint symmetry, thus guaranteeing that the macroscale model maintains some important conservation laws of the original microscale model. Consistency of the patch scheme’s macroscale dynamics with the original microscale model is proved for systems in 1D and 2D space, and these proofs immediately extend to higher dimensions. Expanding from a system with a single configuration to an ensemble of configurations establishes that the proven consistency also holds for cases where the microscale periodicity does not integrally fill the patches. This new self-adjoint patch scheme provides an efficient, flexible, and accurate computational homogenisation, as demonstrated here with canonical examples in 1D and 2D space based on heterogenous diffusion, and is applicable to a wide range of multiscale scenarios of interest to scientists and engineers.

------------------------------------------------------------------------

12\. · 92% match · 2021 · 0.8 cit/yr\
**Adaptively detect and accurately resolve macro-scale shocks in an efficient Equation-Free multiscale simulation** ([link](https://doi.org/10.1137/21m1437172))\
J. Maclean, J. Bunder, I. Kevrekidis, and A. Roberts\
*SIAM J. Sci. Comput.* · Aug 26, 2021 · 4 citations

> The Equation-Free approach to efficient multiscale numerical computation marries trusted micro-scale simulations to a framework for numerical macro-scale reduction – the patch dynamics scheme. A recent novel patch scheme empowered the Equation-Free approach to simulate systems containing shocks on the macro-scale. However, the scheme did not predict the formation of shocks accurately, and it could not simulate moving shocks. This article resolves both issues, as a first step in one spatial dimension, by embedding the Equation-Free, shock-resolving patch scheme within a classic framework for adaptive moving meshes. Our canonical micro-scale problems exhibit heterogeneous nonlinear advection and heterogeneous diffusion. We demonstrate many remarkable benefits from the moving patch scheme, including efficient and accurate macro-scale prediction despite the unknown macro-scale closure. Equation-free methods are here extended to simulate moving, forming and merging shocks without a priori knowledge of the existence or closure of the shocks. Whereas adaptive moving mesh equations are typically stiff, typically requiring small time-steps on the macro-scale, the moving macro-scale mesh of patches is typically not stiff given the context of the micro-scale time-steps required for the sub-patch dynamics.

------------------------------------------------------------------------

13\. · 89% match · 2022\
**Fast Numerical Coarsening with Local Factorizations** ([link](https://doi.org/10.1111/cgf.14619))\
Zhong-Quan He, Jesús Pérez, and M. Otaduy\
*Computer Graphics Forum* · Dec 1, 2022 · 0 citations

> Numerical coarsening methods offer an attractive methodology for fast simulation of objects with high‐resolution heterogeneity. However, they rely heavily on preprocessing, and are not suitable when objects undergo dynamic material or topology updates. We present methods that largely accelerate the two main processes of numerical coarsening, namely training data generation and the optimization of coarsening shape functions, and as a result we manage to leverage runtime numerical coarsening under local material updates. To accelerate the generation of training data, we propose a domain‐decomposition solver based on substructuring that leverages local factorizations. To accelerate the computation of coarsening shape functions, we propose a decoupled optimization of smoothness and data fitting. We evaluate quantitatively the accuracy and performance of our proposed methods, and we show that they achieve accuracy comparable to the baseline, albeit with speed‐ups of orders of magnitude. We also demonstrate our methods on example simulations with local material and topology updates.

------------------------------------------------------------------------

14\. · 87% match · 2026\
**Field conserving adaptive mesh refinement (AMR) scheme on massively parallel adaptive octree meshes** ([link](https://doi.org/10.48550/arXiv.2602.07817))\
K. Saurabh, Makrand A. Khanwale, Masado Ishii, Hari Sundar, and B. Ganapathysubramanian\
*ArXiv* · Feb 8, 2026 · 0 citations

> Adaptive mesh refinement (AMR) is widely used to efficiently resolve localized features in time-dependent partial differential equations (PDEs) by selectively refining and coarsening the mesh. However, in long-horizon simulations, repeated intergrid interpolations can introduce systematic drift in conserved quantities, especially for variational discretizations with continuous basis functions. While interpolation from parent-to-child during refinement in continuous Galerkin (CG) discretizations is naturally conservative, the standard injection-based child-to-parent coarsening interpolation is generally not. We propose a simple, scalable field-conserving coarsening operator for parallel, octree-based AMR. The method enforces discrete global conservation during coarsening by first computing field conserving coarse-element values at quadrature points and then recovering coarse nodal degrees of freedom via an $`L^2`$ projection (mass-matrix solve), which simultaneously controls the $`L_2`$ error. We evaluate the approach on mass-conserving phase-field models, including the Cahn–Hilliard and Cahn–Hilliard–Navier–Stokes systems, and compare against injection in terms of conservation error, solution quality, and computational cost.

------------------------------------------------------------------------

15\. · 85% match · 2014 · 1.1 cit/yr\
**On the acceleration of spatially distributed agent-based computations** ([link](https://doi.org/10.1016/J.APNUM.2014.12.007))\
Ping Liu, G. Samaey, C. Gear, and I. Kevrekidis\
*Applied Numerical Mathematics* · Apr 29, 2014 · 13 citations

> In recent years, individual-based/agent-based modeling has been applied to study a wide range of applications, ranging from engineering problems to phenomena in sociology, economics and biology. Simulating such agent-based models over extended spatiotemporal domains can be prohibitively expensive due to stochasticity and the presence of multiple scales. Nevertheless, many agent-based problems exhibit smooth behavior in space and time on a macroscopic scale, suggesting that a useful coarse-grained continuum model could be obtained. For such problems, the equation-free framework 16-18 can significantly reduce the computational cost. Patch dynamics is an essential component of this framework. This scheme is designed to perform numerical simulations of an unavailable macroscopic equation on macroscopic time and length scales; it uses appropriately initialized simulations of the fine-scale agent-based model in a number of small “patches”, which cover only a fraction of the spatiotemporal domain. In this work, we construct a finite-volume-inspired conservative patch dynamics scheme and apply it to a financial market agent-based model based on the work of Omurtag and Sirovich 22. We first apply our patch dynamics scheme to a continuum approximation of the agent-based model, to study its performance and analyze its accuracy. We then apply the scheme to the agent-based model itself. Our computational experiments indicate that here, typically, the patch dynamics-based simulation needs to be performed in only 20% of the full agent simulation space, and in only 10% of the temporal domain.

------------------------------------------------------------------------

16\. · 79% match · 2024 · 1.5 cit/yr\
**Construct accurate multi-continuum micromorphic homogenisations in multi-D space-time with computer algebra** ([link](https://doi.org/10.48550/arXiv.2407.03483))\
A. J. Roberts\
*ArXiv* · Jul 3, 2024 · 3 citations

> Homogenisation empowers the efficient macroscale system level prediction of physical scenarios with intricate microscale structures. Here we develop an innovative powerful, rigorous and flexible framework for asymptotic homogenisation of dynamics at the \emph{finite} scale separation of real physics, with proven results underpinned by modern dynamical systems theory. The novel systematic approach removes most of the usual assumptions, whether implicit or explicit, of other methodologies. By no longer assuming averages the methodology constructs so-called multi-continuum or micromorphic homogenisations systematically informed by the microscale physics. The developed framework and approach enables a user to straightforwardly choose and create such homogenisations with clear physical and theoretical support, and of highly controllable accuracy and fidelity.

------------------------------------------------------------------------

17\. · 77% match · 2024 · 0.7 cit/yr\
**SpecSims: A Scalable Speculative Tree-based Simulation Cloning Framework for Finite Memory Machines** ([link](https://doi.org/10.1145/3708885))\
Srikanth B. Yoginath, Pratishtha Shukla, James J. Nutaro, and Sudip K. Seal\
*ACM Transactions on Modeling and Computer Simulation* · Dec 26, 2024 · 1 citations

> Simulation cloning is a technique in which cloned simulations whose state spaces differ partially from their parent simulation due to intervening events are spawned at runtime and concurrently advanced. It is a powerful method to carry out what-if analysis by speculatively exploring and evaluating the impact of various permutations of intervening cascade of events. Due to the exponential growth in the number of possible clones even for a small number of distinct intervening events, the practical efficacy of the approach is often severely limited by the maximum available memory of the computing host. In this paper, we introduce a novel speculative simulation cloning framework that executes a simulation cloning campaign capable of efficiently exploring an exponentially large space of clone simulations created by permutation of intervening events under a finite memory constraint. We provide a theoretical analysis of the runtime characteristics of our proposed approach and highlight its novel advantages such as memory-aware and as-long-as-needed execution. In support of our analytical findings and to demonstrate its practical feasibility, we implement a prototype of the cloning framework on a shared memory system and report its performance characteristics in the context of a heat diffusion simulation, and a power grid simulation subject to cascading disruptions from geomagnetic disturbances.

------------------------------------------------------------------------

18\. · 74% match · 2005 · 12 cit/yr\
**Adaptive Multiscale Finite-Volume Method for Multiphase Flow and Transport in Porous Media** ([link](https://doi.org/10.1137/030600795))\
P. Jenny, Seong-Hyeok Lee, and H. Tchelepi\
*Multiscale Model. Simul.* · 252 citations

> We present a multiscale finite-volume (MSFV) method for multiphase flow and transport in heterogeneous porous media. The approach extends our recently developed MSFV method for single-phase flow. We use a sequential scheme that deals with flow (i.e., pressure and total velocity) and transport (i.e., saturation) separately and differently. For the flow problem, we employ two different sets of basis functions for the reconstruction of a conservative fine-scale total velocity field. Our basis functions are designed to have local support, and that allows for adaptive computation of the flow field. We use a criterion based on the time change of the total mobility field to decide when and where to recompute our basis functions. We show that at a given time step, only a small fraction of the basis functions needs to be recomputed. Numerical experiments of difficult two-dimensional and three-dimensional test cases demonstrate the accuracy, computational efficiency, and overall scalability of the method.

------------------------------------------------------------------------

19\. · 73% match · 2025 · 1.9 cit/yr\
**Out of Order and Causally Correct: Ready-Event Discovery through Data-Dependence Analysis** ([link](https://doi.org/10.1145/3726301.3728416))\
Erik J. Jensen, James F. Leathrum, Christopher J. Lynch, Katherine Smith, and Ross J. Gore\
*39th ACM SIGSIM Conference on Principles of Advanced Discrete Simulation* · Jun 22, 2025 · 2 citations

> Data-dependence analysis can identify causally-unordered events in a pending event set. The execution of these events is independent from all other scheduled events, making them ready for execution. These events can be executed out of order or in parallel. This approach may find and utilize more parallelism than spatial-decomposition parallelization methods, which are limited by the number of subdomains and by synchronization methods. This work provides formal definitions that use data-dependence analysis to find causally-unordered events and uses these definitions to measure parallelism in several discrete-event simulation models. A variant of the event-graph formalism is proposed, which assists with identifying ready events, by more clearly visualizing data dependencies between event types. Data dependencies between two event types may be direct or indirect, where the latter case considers the scheduling of intermediate events. Data dependencies and scheduling dependencies in a discrete-event simulation model are used to define time-interval limits that support the identification of events that are ready for execution. Experimental results from serial simulation testing demonstrate the availability of numerous events that are ready for execution, depending on model characteristics. The mean size of the ready-event set varies from about 1.5 to 110 for the tested models, depending on the model type, the size of the model, and delay distribution parameters. These findings support future work to develop a parallel capability to dynamically identify and execute ready events in a multi-threaded environment.

------------------------------------------------------------------------

20\. · 66% match · 2024 · 1.1 cit/yr\
**Robust treatment for the coarse/fine interface of adaptive mesh in the simulation of two-phase flow** ([link](https://doi.org/10.1016/j.jcp.2024.113485))\
Cheng Liu, Yiding Hu, Ruoqing Gao, and Changhong Hu\
*J. Comput. Phys.* · Oct 1, 2024 · 2 citations

------------------------------------------------------------------------

21\. · 63% match · 2001\
**Updateable Simulations** ([link](https://www.semanticscholar.org/paper/0636fd1b1a23ce3cd4d9bdea23d30012e90cda8c))\
0 citations

> A technique called updateable simulations is proposed to reduce the time to complete multiple executions of a discrete event simulation program. This technique updates the results of a prior simulation run rather than re-execute the entire simulation to take into account variations in the underlying simulation model. A framework for creating updateable simulations is presented. The algorithm used in the framework lends itself to straightforward realization on parallel computers. This framework is applied to the problem of simulating a set of cascaded ATM multiplexers. Performance measurements of a parallel implementation of this simulation on a shared memory multiprocessor are presented, demonstrating that updateable simulations can yield substantial reductions in the time required to complete multiple simulation runs if there is much similarity among the runs.

------------------------------------------------------------------------

22\. · 62% match · 2017 · 0.7 cit/yr\
**A Feature-Enriched Multiscale Method for Simulating Complex Geomodels** ([link](https://doi.org/10.2118/182701-MS))\
Knut-Andreas Lie, O. Møyner, and J. Natvig\
Feb 20, 2017 · 7 citations

> Multiscale methods have been shown to offer an order-of-magnitude increase in the speed of reservoir simulators. This may enable users to model complex fluid flow and geology with greater speed and flexibility than is available with the current computational technologies. Contemporary multiscale methods typically use a restriction operator to construct a reduced system of flow equations and a prolongation operator to map pressure unknowns from the reduced flow equations back to the original simulation grid. When combined with a local smoother, this gives an iterative solver that can efficiently compute approximate pressures to within a prescribed accuracy and still provide mass-conservative fluxes. We present an adaptive and flexible framework for combining multiple sets of such multiscale approximations. Each multiscale approximation can target a certain scale; geological features like faults, fractures, facies, or other geobodies; or a particular computational challenge like propagating displacement and chemical fronts, wells being turned on or off, etc. Multiscale methods that fit the framework are characterized by three features. First, the prolongation and restriction operators are constructed using a non-overlapping partition of the fine grid. Second, the prolongation operator is composed of a set of basis functions, each of which has compact support within a support region that contains a coarse grid block. Finally, the basis functions form a partition of unity. Through a series of numerical examples that include idealized geology and flow physics as well as geological models of real assets, we demonstrate that the new framework increases the accuracy and efficiency of multiscale technology. In particular, we show how it is possible to combine multiscale approximations with different resolution as well as multiscale approximations targeting, among others, high-contrast fluvial sands; fractured carbonate reservoirs; challenging grids including faults, pinchouts and inactive cells; and complex wells. Introduction In reservoir simulation, a system of mass balance equations needs to be solved to determine the reservoir pressure and fluid composition. Each mass balance equation describes the evolution of one fluid species α in a porous medium Ω, in which multiple fluid species exist in M phases. When discretized in time and space, these equations form a system of nonlinear algebraic equations F α(p, S1, . . . , SM , xα,1, . . . , xα,M ) = qα. (1) Given a known pressure and fluid distribution at time t, Eq. 1 can be solved to determine the reservoir pressure p and distribution of fluid species (in terms of phase saturations S`and molar fractions xα,`) at time t + ∆t. In particular, by manipulating the equation system Eq. 1, it is possible to form a nonlinear system of equations for the reservoir pressure p at time t+ ∆t,

------------------------------------------------------------------------

23\. · 61% match · 2024 · 0.6 cit/yr\
**Exact Solutions and Upscaling for 1D Two‐Phase Flow in Heterogeneous Porous Media** ([link](https://doi.org/10.1029/2024WR037917))\
K. O. Prempeh, Parker William George, and Pavel Bedrikovetsky\
*Water Resources Research* · Nov 1, 2024 · 1 citations

> Upscaling of 1D two‐phase flows in heterogeneous porous media is important in interpretation of laboratory coreflood data, streamline quasi 3D modeling, and numerical reservoir simulation. In 1D heterogeneous media with properties varying along the flow direction, phase permeabilities are coordinate‐dependent. This yields the Buckley‐Leverett equation with coordinate‐dependent fractional flow f = f(s, x), which reflects the heterogeneity. So, an x‐dependency is considered to reflect microscale heterogeneity and averaging over x—upscaling. This work aims to average or upscale the heterogeneous system to obtain the homogenized media with such fractional flow function F(S) that provides the same water‐cut history at the reservoir outlet, x = 1. Thus, F(S) is an equivalent property of the medium. So far, the exact upscaling for 1D micro heterogeneous systems has not been derived. With the x‐dependency of fractional flow, the Riemann invariant is flux f, which yields exact integration of 1D flow problems. The novel exact solutions are derived for flows with continuous saturation profile, transition of shock into continuous wave, transition of continuous wave into shock, and transport in heterogeneous piecewise‐uniform rocks. The exact procedure of upscaling from f = f(s, x) to F(S) is as follows: the inverse function to the upscaled F(S) is equal to the averaged saturation over x of the inverse microscale function s = f −1(f, x). It was found that the Welge’s method as applied to heterogeneous cores provides the upscaled F(S). For characteristic finite‐difference scheme, the fluxes for microscale and upscaled‐numerical‐cell systems, coincide in all grid nodes.

------------------------------------------------------------------------

24\. · 60% match · 2019 · 1.7 cit/yr\
**Accelerating multiscale simulation of complex geomodels by use of dynamically adapted basis functions** ([link](https://doi.org/10.1007/s10596-019-9827-z))\
Ø. Klemetsdal, O. Møyner, and Knut-Andreas Lie\
*Computational Geosciences* · Jun 20, 2019 · 12 citations

> A number of different multiscale methods have been developed as a robust alternative to upscaling and as a means for accelerated reservoir simulation of high-resolution geomodels. In their basic setup, multiscale methods use a restriction operator to construct a reduced system of flow equations on a coarser grid, and a prolongation operator to map pressure unknowns from the coarse grid back to the original simulation grid. The prolongation operator consists of basis functions computed numerically by solving localized flow problems. One can use the resulting multiscale solver both as a CPR preconditioner in fully implicit simulators or as an efficient approximate iterative linear solver in a sequential setting. The latter approach has been successfully implemented in a commercial simulator. Recently, we have shown that you can obtain significantly faster convergence if you instead of using a single pair of prolongation-restriction operators apply a sequence of such operators, where some of the operators adapt to faults, fractures, facies, or other geobodies. Herein, we present how you can accelerate the convergence even further, if you also include additional basis functions that capture local changes in the pressure.

------------------------------------------------------------------------

25\. · 59% match · 2024\
**Out-of-Order Discrete Event Simulation: Fighting Memory Boundedness while Running DES Models** ([link](https://doi.org/10.1109/DS-RT62209.2024.00024))\
Romolo Marotta and Francesco Quaglia\
*2024 28th International Symposium on Distributed Simulation and Real Time Applications (DS-RT)* · Oct 7, 2024 · 0 citations

> In this article we present Out-of-order Discrete Event Simulation (ODES), a solution for sequential style execution of DES models not following timestamp order. ODES ensures anyway the same identical simulation results as timestamp ordered execution, thanks to fully correct maintenance of the simulation model data flow. At the same time, it drastically reduces the memory boundedness—namely, the impact of cache misses and of stale CPU cycles—on the simulation model execution speed. Beyond presenting foundational concepts, we also discuss our ODES-engine implementation, based on the c programming language. Additionally, we report experimental data for comparing ODES with the classical timestamp ordered execution of simulation models according to conventional sequential simulation. The relevance of ODES compared to classical timestamp ordered sequential DES not only stands in its benefits on performance, rather ODES can also assume the role of new reference for determining the speedup achievable via parallel/distributed discrete event simulation systems, compared to the single thread execution. Also, thanks to its improvements in the interaction with RAM, ODES constitutes a new framework for effective parallel replication of simulation experiments on multi-processor/multi-core machines.

------------------------------------------------------------------------

26\. · 57% match · 2006 · 0.2 cit/yr\
**A SPECIALIZED UPSCALING METHOD FOR ADAPTIVE GRIDS: TIGHT INTEGRATION OF LOCAL-GLOBAL UPSCALING AND ADAPTIVITY LEADS TO ACCURATE SOLUTION OF FLOW IN HETEROGENEOUS FORMATIONS** ([link](https://www.semanticscholar.org/paper/005831318e3f40a261d680496b63759246a5a2d7))\
J. Lambers\
4 citations

> We propose a methodology, called multi-level local-global upscaling, for generating accurate upscaled models of permeabilities or transmissibilities for flow simulation in highly heterogeneous subsurface formations. The method generates an initial adapted grid based on the given fine scale reservoir heterogeneity and potential flow paths. It then applies local-global upscaling \[1\], along with adaptivity, in an iterative manner. For highly heterogeneous (e.g., channelized) systems, this integration of grid adaptivity and upscaling is shown to consistently provide more accurate coarse scale models for global flow, relative to reference fine scale results, than do existing upscaling techniques applied to uniform grids of similar densities. The algorithm allows for rapid updating of the coarse scale model when the grid is adapted dynamically during transport to resolve important features, or as a result of changing boundary conditions. There is no need for downscaling and both refinement and coarsening can take place dynamically without destroying accurate global flow resolution on the adapted grid.

------------------------------------------------------------------------

27\. · 56% match · 2005 · 0.6 cit/yr\
**An Adaptive Multiphase Multiscale Finite Volume Simulator for Heterogeneous Reservoirs** ([link](https://doi.org/10.2118/93395-MS))\
H. Tchelepi, P. Jenny, Seong H. Lee, and Christian Wolfsteiner\
13 citations

> We developed an adaptive reservoir simulator for accurate modeling of multiphase flow and transport in large scale heterogeneous reservoirs. The simulator is based on a multiscale finite volume (MSFV) method. We describe both IMPES and sequential implicit formulations. The algorithms are sensitive to the specific characteristics of flow (i.e., pressure and total velocity) and transport (i.e., saturation). To obtain the fine scale (i.e., fine grid) flow field, two sets of basis functions - dual and primal - are constructed. The dual basis functions, which are associated with the dual coarse grid, are used to calculate the coarse scale transmissibilities. The fine scale pressure field is computed from the coarse grid pressure via superposition of the dual basis functions. Having a locally conservative fine scale velocity field is essential for accurate solution of the saturation equations (i.e., transport). The primal basis functions, which are associated with the primal coarse grid, are constructed for that purpose. The dual basis functions serve as boundary conditions to the primal basis functions. To resolve the fine scale flow field in and around wells, a special well basis function is devised. As with the other basis functions, we ensure that the support for the well basis is local. Our MSFV simulator is designed for adaptive computation of both flow and transport in the course of a simulation run. Adaptive computation of the flow field is based on the time change of the total mobility field and triggers selective updates of basis functions. The key to achieving scalable (efficient for large problems) adaptive computation of flow and transport is the use of high fidelity basis functions with local support. We demonstrate the robustness and computational efficiency of the MSFV simulator using a variety of large heterogeneous reservoir models, including the SPE 10 comparative solution problem.

------------------------------------------------------------------------

28\. · 54% match · 2016 · 11 cit/yr\
**A multiscale restriction-smoothed basis method for high contrast porous media represented on unstructured grids** ([link](https://doi.org/10.1016/j.jcp.2015.10.010))\
O. Møyner and Knut-Andreas Lie\
*J. Comput. Phys.* · 117 citations

> A wide variety of multiscale methods have been proposed in the literature to reduce runtime and provide better scaling for the solution of Poisson-type equations modeling flow in porous media. We present a new multiscale restricted-smoothed basis (MsRSB) method that is designed to be applicable to both rectilinear grids and unstructured grids. Like many other multiscale methods, MsRSB relies on a coarse partition of the underlying fine grid and a set of local prolongation operators (multiscale basis functions) that map unknowns associated with the fine grid cells to unknowns associated with blocks in the coarse partition. These mappings are constructed by restricted smoothing: Starting from a constant, a localized iterative scheme is applied directly to the fine-scale discretization to compute prolongation operators that are consistent with the local properties of the differential operators.The resulting method has three main advantages: First of all, both the coarse and the fine grid can have general polyhedral geometry and unstructured topology. This means that partitions and good prolongation operators can easily be constructed for complex models involving high media contrasts and unstructured cell connections introduced by faults, pinch-outs, erosion, local grid refinement, etc. In particular, the coarse partition can be adapted to geological or flow-field properties represented on cells or faces to improve accuracy. Secondly, the method is accurate and robust when compared to existing multiscale methods and does not need expensive recomputation of local basis functions to account for transient behavior: Dynamic mobility changes are incorporated by continuing to iterate a few extra steps on existing basis functions. This way, the cost of updating the prolongation operators becomes proportional to the amount of change in fluid mobility and one reduces the need for expensive, tolerance-based updates. Finally, since the MsRSB method is formulated on top of a cell-centered, conservative, finite-volume method, it is applicable to any flow model in which one can isolate a pressure equation. Herein, we only discuss single and two-phase incompressible models. Compressible flow, e.g., as modeled by the black-oil equations, is discussed in a separate paper.

------------------------------------------------------------------------

29\. · 53% match · 2015 · 0.2 cit/yr\
**Interactive Simulations Using Localized Reduced Basis Methods** ([link](https://doi.org/10.1016/J.IFACOL.2015.05.134))\
A. Buhr and Mario Ohlberger\
*IFAC-PapersOnLine* · 2 citations

> Abstract An interactive simulation tool should allow its user to change the geometry of the simulation and present an updated solution within a very short time span. To achieve this, the Reduced Basis Method can be used. For problems described by parametrized partial differential equations, it allows for very fast recomputation of the solution after parameter changes. In many cases, changes in the geometry can be accounted for by parametrization. However, this approach has two drawbacks: First, not all geometric variations can be described efficiently by parametrization. Second, the parametrization and thereby the type of changes possible has to be specified before the setup phase. The user is then restricted to these. To overcome these limitations, we propose to localize the basis generation in the Reduced Basis Method. Using basis functions having support only on a small subset of the domain, one can react to arbitrary local geometry modifications by recreating only the basis functions in an environment of the modification.

------------------------------------------------------------------------

30\. · 52% match · 2022 · 1.4 cit/yr\
**Two novel families of multiscale staggered patch schemes efficiently simulate large-scale, weakly damped, linear waves** ([link](https://doi.org/10.1016/j.cma.2023.116133))\
J. Divahar, A. Roberts, T. W. Mattner, J. Bunder, and I. Kevrekidis\
*ArXiv* · Oct 28, 2022 · 5 citations

> Many multiscale wave systems exhibit macroscale emergent behaviour, for example, the fluid dynamics of floods and tsunamis. Resolving a large range of spatial scales typically requires a prohibitively high computational cost. The small dissipation in wave systems poses a significant challenge to further developing multiscale modelling methods in multiple dimensions. This article develops and evaluates two families of equation-free multiscale methods on novel 2D staggered patch schemes, and demonstrates the power and utility of these multiscale schemes for weakly damped linear waves. A detailed study of sensitivity to numerical roundoff errors establishes the robustness of developed staggered patch schemes. Comprehensive eigenvalue analysis over a wide range of parameters establishes the stability, accuracy, and consistency of the multiscale schemes. Analysis of the computational complexity shows that the measured compute times of the multiscale schemes may be 10^5 times smaller than the compute time for the corresponding full-domain computation. This work provides the essential foundation for efficient large-scale simulation of challenging nonlinear multiscale waves.

------------------------------------------------------------------------

31\. · 46% match · 2019 · 0.5 cit/yr\
**Multiscale gradient computation for multiphase flow in porous media** ([link](https://www.semanticscholar.org/paper/28f793b3f7cb3da5f3b99152c4ddc1853e321127))\
J. Moraes, H. Hajibeygi, Jan Dirk, and Doi\
4 citations

> A multiscale gradient computation method for multiphase flow in heterogeneous porous media is developed. The method constructs multiscale primal and dual coarse grids, imposed on the given fine-scale computational grid. Local multiscale basis functions are computed on (dual-) coarse blocks, constructing an accurate map (prolongation operator) between coarse- and fine-scale systems. While the expensive operations involved in computing the gradients are performed at the coarse scale, sensitivities with respect to uncertain parameters (e.g., grid block permeabilities) are expressed in the fine scale via the partial derivatives of the prolongation operator. Hence, the method allows for updating of the geological model, rather than the dynamic model only, avoiding upscaling and the inevitable loss of information. The formulation and implementation are based on automatic differentiation (AD), allowing for convenient extensions to complex physics. An IMPES coupling strategy for flow and transport is followed, in the forward simulation. The flow equation is computed using a multiscale finite volume (MSFV) formulation and the transport equation is computed at the fine scale, after reconstruction of mass conservative velocity field. To assess the performance of the method, a synthetic multiphase flow test case is considered. The multiscale gradients are compared against those obtained from a fine-scale reference strategy. Apart from its computational efficiency, the benefits of the method include flexibility to accommodate variables expressed at different scales, specially in multiscale data assimilation and reservoir management studies.

------------------------------------------------------------------------

32\. · 45% match · 2016 · 1.0 cit/yr\
**Quasi-matrix-free Hybrid Multigrid on Dynamically Adaptive Cartesian Grids** ([link](https://doi.org/10.1145/3165280))\
Marion Weinzierl and T. Weinzierl\
*ACM Transactions on Mathematical Software (TOMS)* · Jul 3, 2016 · 10 citations

> We present a family of spacetree-based multigrid realizations using the tree’s multiscale nature to derive coarse grids. They align with matrix-free geometric multigrid solvers as they never assemble the system matrices, which is cumbersome for dynamically adaptive grids and full multigrid. The most sophisticated realizations use BoxMG to construct operator-dependent prolongation and restriction in combination with Galerkin/Petrov-Galerkin coarse-grid operators. This yields robust solvers for nontrivial elliptic problems. We embed the algebraic, problem-dependent, and grid-dependent multigrid operators as stencils into the grid and evaluate all matrix-vector products in situ throughout the grid traversals. Such an approach is not literally matrix-free as the grid carries the matrix. We propose to switch to a hierarchical representation of all operators. Only differences of algebraic operators to their geometric counterparts are held. These hierarchical differences can be stored and exchanged with small memory footprint. Our realizations support arbitrary dynamically adaptive grids while they vertically integrate the multilevel operations through spacetree linearization. This yields good memory access characteristics, while standard colouring of mesh entities with domain decomposition allows us to use parallel many-core clusters. All realization ingredients are detailed such that they can be used by other codes.

------------------------------------------------------------------------

33\. · 41% match · 1996 · 4.1 cit/yr\
**A Projection Method for Locally Refined Grids** ([link](https://doi.org/10.1006/JCPH.1996.0166))\
M. Minion\
*Journal of Computational Physics* · Aug 1, 1996 · 124 citations

> A numerical method for the solution of the two-dimensional Euler equatons for incompressible flow on locally refined grids is presented. The method is a second-order Godunov-projection method adapted from Bell, Colella, and Glaz. Second-order accuracy of the numerical method in time and space is established through numerical experiments. The main contributions of this work concern the formulation and implementation of a projection for refined grids. A discussion of the adjointness relation between gradient and divergence operators for a refined grid MAC projection is presented, and a refined grid approximate projection is developed. An efficient multigrid method which exactly solves the projection is developed, and a method for casting certain approximate projections as MAC projections on refined grids is presented.

------------------------------------------------------------------------

34\. · 37% match · 2017 · 0.1 cit/yr\
**Multiscale Gradient Computation for Multiphase Flow in Porous Media** ([link](https://doi.org/10.2118/182625-MS))\
R. Moraes, J. Rodrigues, H. Hajibeygi, and Jan Dirk Jansen\
Feb 20, 2017 · 1 citations

> A multiscale gradient computation method for multiphase flow in heterogeneous porous media is developed. The method constructs multiscale primal and dual coarse grids, imposed on the given fine-scale computational grid. Local multiscale basis functions are computed on (dual-) coarse blocks, constructing an accurate map (prolongation operator) between coarse- and fine-scale systems. While the expensive operations involved in computing the gradients are performed at the coarse scale, sensitivities with respect to uncertain parameters (e.g., grid block permeabilities) are expressed in the fine scale via the partial derivatives of the prolongation operator. Hence, the method allows for updating of the geological model, rather than the dynamic model only, avoiding upscaling and the inevitable loss of information. The formulation and implementation are based on automatic differentiation (AD), allowing for convenient extensions to complex physics. An IMPES coupling strategy for flow and transport is followed, in the forward simulation. The flow equation is computed using a multiscale finite volume (MSFV) formulation and the transport equation is computed at the fine scale, after reconstruction of mass conservative velocity field. To assess the performance of the method, a synthetic multiphase flow test case is considered. The multiscale gradients are compared against those obtained from a fine-scale reference strategy. Apart from its computational efficiency, the benefits of the method include flexibility to accommodate variables expressed at different scales, specially in multiscale data assimilation and reservoir management studies.

------------------------------------------------------------------------

35\. · 34% match · 2012 · 3.9 cit/yr\
**Deterministic parallel random-number generation for dynamic-multithreading platforms** ([link](https://doi.org/10.1145/2145816.2145841))\
C. Leiserson, T. Schardl, and Jim Sukha\
*ACM SIGPLAN Symposium on Principles & Practice of Parallel Programming* · Feb 25, 2012 · 56 citations

> Existing concurrency platforms for dynamic multithreading do not provide repeatable parallel random-number generators. This paper proposes that a mechanism called pedigrees be built into the runtime system to enable efficient deterministic parallel random-number generation. Experiments with the open-source MIT Cilk runtime system show that the overhead for maintaining pedigrees is negligible. Specifically, on a suite of 10 benchmarks, the relative overhead of Cilk with pedigrees to the original Cilk has a geometric mean of less than 1%. We persuaded Intel to modify its commercial C/C++ compiler, which provides the Cilk Plus concurrency platform, to include pedigrees, and we built a library implementation of a deterministic parallel random-number generator called DotMix that compresses the pedigree and then “RC6-mixes” the result. The statistical quality of DotMix is comparable to that of the popular Mersenne twister, but somewhat slower than a nondeterministic parallel version of this efficient and high-quality serial random-number generator. The cost of calling DotMix depends on the “spawn depth” of the invocation. For a naive Fibonacci calculation with n=40 that calls DotMix in every node of the computation, this “price of determinism” is a factor of 2.65 in running time, but for more realistic applications with less intense use of random numbers – such as a maximal-independent-set algorithm, a practical samplesort program, and a Monte Carlo discrete-hedging application from QuantLib – the observed “price” was less than 5%. Moreover, even if overheads were several times greater, applications using DotMix should be amply fast for debugging purposes, which is a major reason for desiring repeatability.

------------------------------------------------------------------------

36\. · 29% match · 2014 · 4.9 cit/yr\
**Fast splittable pseudorandom number generators** ([link](https://doi.org/10.1145/2714064.2660195))\
G. Steele, D. Lea, and Christine H. Flood\
*ACM SIGPLAN Notices* · Oct 15, 2014 · 58 citations

------------------------------------------------------------------------

37\. · 28% match · 2014 · 0.9 cit/yr\
**Construction of Multiscale Preconditioners on Stratigraphic Grids** ([link](https://doi.org/10.3997/2214-4609.20141775))\
O. Møyner\
Sep 8, 2014 · 11 citations

> A large number of multiscale methods have been developed based on the same basic concept: Solve localized flow problems to estimate the local effects of fine-scale petrophysical properties. Use the resulting multiscale basis functions to pose a global flow problem a coarser grid. Reconstruct conservative fine-scale approximations from the coarse-scale solution. By extending the basic concept with iteration cycles and additional local stages, one can systematically drive the fine-scale residual towards machine precision. Posed algebraically, this can be seen as a set of restriction operators for computing a reduced global problem and a set of prolongation operators for constructing conservative fine-scale approximations. Such multiscale finite-volume methods have been extensively developed for Cartesian grids in the literature. The industry, however, uses very complex with unstructured connections and degenerate cell geometries to represent realistic structural frameworks and stratigraphic architectures. A successful multiscale method should therefore be able to handle unstructured polyhedral grids, both on the fine and coarse scale, and be as flexible as possible to enable automatic coarse partitionings that adapt to wells and geological features in a way that ensures optimal accuracy for a chosen level of coarsening. Herein, we will discuss a compare a set of prolongation operators that can be combined with finite-element or finite-volume restriction operators to form different multiscale finite-volume methods. We consider the MsFV prolongation operator (developed on a dual coarse grid with unitary at coarse block vertices), the more recent MsTPFA operator (developed on primal grid with unitary flux across coarse block faces), as well as a simplified constant prolongation operator. The methods will be compared on a variety of test cases ranging from simple synthetic grids to highly complex, real-world, field models. Our discussion will focus on flexibility wrt (coarse) grids and tendency of creating oscillatory approximations. In addition, we will look at various methods for improving the methods’ convergence properties when used as preconditioners, as well as for generating novel prolongation operators. This is relevant for oil recovery because: - Multiscale methods may provide a way to significantly speed up reservoir simulation and make previously intractable problems possible to solve. - The extension of such methods to industry standard grids used for reservoir modelling enables the evaluation of the methods on real world models - The construction of basis functions for multiscale methods may have direct connections to the process of upscaling rock derived properties such as transmissibility

------------------------------------------------------------------------

38\. · 27% match · 2010\
**Conservative adaptivity and two-way self-nesting using discrete wavelets** ([link](https://www.semanticscholar.org/paper/1844a4daedf3dddaee80b8d553e82b9ed6219e33))\
T. Dubos\
0 citations

> In simulating atmosphere and oceans, multiscale modelling is desirable to track high-intensity weather patterns, to investigate the interactions between the various spatio-temporal scales of the climate system, and to perform assessments of climate change at scales small enough to derive impacts on society and ecosystems. The mainstream approach to multiscale modelling is to nest a fine, limited-area model into a coarse, global model. These models are then coupled, either one-way or two-way, in order to combine the global coverage of the global model and the fine details of the fine model.
