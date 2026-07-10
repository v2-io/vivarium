# Frontier matrix

##### [**Undermind**](https://undermind.ai)

*Vivarium frontier scan (2026-07-10) — sortable companion to `frontier-scan-2026-07-10.md` (which carries the per-thread prose + no-gos). Source: Undermind; **unverified external** until primary-checked (relata registration deferred — relata under construction). A deeper multi-prompt deep-search pass is queued.*

---


## Table of Contents

- [Frontier matrix]( #frontier-matrix)
  - [Sharp takeaways]( #sharp-takeaways)
  - [Current likely gaps]( #current-likely-gaps)

# Frontier matrix

This matrix condenses the strongest adjacent work around the target architecture. The emphasis is on what appears newer, better, or importantly adjacent, not on re-stating the older baseline.

**Legend**

- **Fit**: High = directly useful architectural analogue or likely import; Medium = partial import or strong adjacent method; Low = mainly contextual.
- **Maturity**: Production = maintained framework or broadly reusable infrastructure; Applied = serious research system used on nontrivial cases; Prototype = promising but still narrow or early.
- **POST-JAN-2026** marks work first published after January 31, 2026.

| Item | Thread | Fit | Maturity | Notes |
|:---|:---|:---|:---|:---|
| [Interactive Authoring of Terrain using Diffusion Models](https://onlinelibrary.wiley.com/doi/10.1111/cgf.14941) | Terrain synthesis | High | Applied | **2023** · Lochner et al. · Diffusion over DEMs with sketch/style control and interactive upsampling. Useful as a learned terrain prior that can sit upstream of deterministic erosion. Code/source mentioned by authors. |
| [Terrain Amplification using Multi-scale Erosion](https://research.adobe.com/publication/terrain-amplification-using-multi-scale-erosion/) | Terrain synthesis | High | Applied | **2024** · Schott et al. · Multiscale amplification of low-res terrain into hydrologically consistent fine detail using thermal, stream-power, and hillslope approximations. Very close to macro-shape plus micro-detail synthesis. |
| [Physically-based Analytical Erosion for fast Terrain Generation](https://diglib.eg.org/items/0a412929-2e29-4bcb-a597-1a4b2cc558e2) | Terrain synthesis | High | Applied | **2024** · Tzathas et al. · Replaces long uplift–erosion runs with an analytical or procedural formulation plus multigrid acceleration. Architecturally friendly for memoized evaluation. DOI 10.1111/cgf.15033. |
| [Large-scale Terrain Authoring through Interactive Erosion Simulation](https://dblp.org/rec/journals/tog/SchottPFGG23) | Terrain synthesis | High | Applied | **2023** · Schott et al. · Interactive approximate erosion authoring with inverse-style uplift reconstruction. Strong control story, but not yet the real-time continent-scale plus metre-precision solution. DOI 10.1145/3592787. |
| [Estimation of Denudation Parameters and River Capture Events From Neural Network Inverse Modeling](https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2024JF007636) | Inverse landscape evolution | Medium | Applied | **2024** · Bernard et al. · Neural inverse modeling for denudation and capture events from river and thermochronology signals. Narrower than full DEM inversion, but a real learning-based inverse geomorphology signal. |
| [Inferring Long-Term Tectonic Uplift Patterns From Bayesian Inversion of Fluvially-Incised Landscapes](https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2024JB030819) | Inverse landscape evolution | High | Applied | **2025** · Oryan et al. · Bayesian inversion around forward fluvial LEMs to recover uplift and stream-power parameters. Strong bridge from forward architecture to calibrated inference. |
| [Exploring Controls on Post-Orogenic Topographic Stasis of the Pyrenees Mountains With Inverse Landscape Evolution Modeling](https://agupubs.onlinelibrary.wiley.com/toc/21699011/2025/130/2) | Inverse landscape evolution | Medium | Applied | **2025** · Curry and van der Beek · Applied inverse LEM showing threshold erodibility dominates in that case study. Important more as proof of operational inverse workflow than as a new algorithm. |
| [Seeing Through Geomorphic Complexity to Recover Tectonics From Topography](https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2025JF008966) | Inverse landscape evolution | High | Applied | **2026** · Morris et al. · Uses Wasserstein or optimal-transport misfits to recover uplift histories from noisy landscapes. Better objective function, not just better solver. **POST-JAN-2026**. |
| [fastscapelib documentation](https://fastscapelib.readthedocs.io/en/latest/) | Geoscience LEM software | High | Production | **Active 2026** · Modular C++ and Python kernels for flow routing, eroders, multiple grids, and marine processes. The most composable geoscience stack relative to a query-graph runtime. |
| [Implicit Algorithm for Threshold Stream Power Incision Model](https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2023JF007140) | Geoscience LEM software | High | Applied | **2023** · Braun and Deal · Extends the linear-time implicit FastScape line to thresholded SPIM with rainfall or discharge variability. Direct supersession of the simpler implicit stream-power core. |
| [Landlab](https://landlab.github.io/) | Geoscience LEM software | High | Production | **Active 2026** · Modular process components including MFD and D-infinity routing, implicit overland flow, marine sediment transport, SPACE-style alluvium conservation, and lithology helpers. Strong process-graph comparison point. |
| [Badlands](https://badlands.readthedocs.io/en/latest/) | Geoscience LEM software | Medium | Production | **Active 2026** · Basin-to-marine surface-process model with hillslope diffusion, fluvial incision or transport capacity laws, and sedimentary basin coupling. Best comparison point for land–marine continuity. |
| [A Modular Implicit Numerical Method for Hillslope Sediment Transport Laws](https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2025JF008675) | Geoscience LEM software | High | Applied | **2026** · Ren et al. · Modular implicit scheme for nonlinear hillslope transport laws that were awkward to solve implicitly before. First published January 3, 2026. |
| [TTLEM](https://github.com/TopoToolbox/TTLEM) | Geoscience LEM software | Medium | Production | **Active 2026** · Object-oriented MATLAB LEM inside the TopoToolbox ecosystem. Less frontier than fastscapelib, but still a sharp bridge between DEM analysis and forward evolution modeling. |
| [AMReX and pyAMReX: Looking Beyond ECP](https://journals.sagepub.com/doi/10.1177/10943420241271017) | AMR and multirate numerics | High | Production | **2024** · Myers et al. · Current AMReX state plus Python bindings and zero-copy GPU-side data access. Best live Berger–Oliger-style production substrate with AI or ML adjacency. |
| [p4est](https://p4est.github.io/api/p4est-latest/index.html) | AMR and multirate numerics | High | Production | **Active 2026** · Parallel adaptive quadtree or octree AMR with explicit partition and adapt workflows. Very close to cube-sphere quadtree instincts. |
| [Dendro-GR](https://paralab.github.io/Dendro-GR/) | AMR and multirate numerics | Medium | Applied | **Active 2026** · Wavelet-guided octree adaptivity with symbolic code generation and GPU acceleration. Worth watching if the architecture shifts toward octree-native PDE infrastructure. |
| [Multirate Time-Integration based on Dynamic ODE Partitioning through Adaptively Refined Meshes for Compressible Fluid Dynamics](https://arxiv.org/abs/2403.05144) | AMR and multirate numerics | High | Applied | **2024** · Doehring et al. · AMR-driven dynamic partitioning plus paired-explicit multirate Runge–Kutta. One of the most on-point recent multirate papers for this architecture. |
| [Self-Adjusting Multi-Rate Runge-Kutta Methods](https://link.springer.com/article/10.1007/s10915-025-03049-y) | AMR and multirate numerics | Medium | Applied | **2025** · Practical self-adjusting multirate RK implementation. More implementable than much of the MRI-GARK literature. |
| [Data-driven stochastic closure modeling via conditional diffusion model and neural operator](https://www.sciencedirect.com/science/article/pii/S0021999125002888) | Learned multiscale surrogates | High | Applied | **2025** · Uses an FNO inside a diffusion-based stochastic closure model for unresolved multiscale dynamics. Strong example of calibrated coarse surrogate replacing expensive fine physics. |
| [G-PARC](https://www.nature.com/articles/s41598-026-59318-9) | Learned multiscale surrogates | High | Applied | **2026** · Beerman et al. · Physics-aware graph recurrent surrogate on unstructured or moving meshes, benchmarked on fluvial hydrology and other nonlinear PDEs. **POST-JAN-2026**. |
| [Generalizable data-driven turbulence closure modeling on unstructured grids with differentiable physics](https://www.sciencedirect.com/science/article/abs/pii/S0045793026002422) | Learned multiscale surrogates | High | Applied | **2026** · GNN subgrid closure embedded in a differentiable FEM Navier–Stokes solver and trained through discrete adjoints. Strongest recent closure-plus-differentiable-physics analogue found. **POST-JAN-2026**. |
| [Concurrent two-way coupling of global and local models across internal boundaries with non-matching discretizations](https://www.aimsciences.org/article/doi/10.3934/acse.2025016) | Detail to abstract | High | Applied | **2025** · Kang and Masud · Consistent two-way global-local coupling across non-matching interfaces. Closest numerical analogue to conservative lifting of local perturbations into coarse state. |
| [A fully-implicit solving approach to an adaptive multi-scale model coupling a vertical-equilibrium and full-dimensional model](https://link.springer.com/article/10.1007/s10596-025-10351-z) | Detail to abstract | Medium | Applied | **2025** · Monolithic adaptive coupling of cheap global and expensive local models. Strong adjacent result, but still numerical domain decomposition rather than semantic upward invalidation. |
| [Efficient Parallel Self-Adjusting Computation](https://arxiv.org/abs/2105.06712) | Detail to abstract | High | Applied | **2021** · Parallel change propagation with tracked dependencies. Still the clearest systems precedent for upward invalidation in a memoized DAG, but not a physical simulation answer by itself. |
| [Inversion by Partial Evaluation: A Reversible Interpreter Experiment](https://doi.org/10.48550/arXiv.2412.03122) | Detail to abstract | Low | Prototype | **2024** · Reversible-programming adjacency. Useful mainly as a hint that invertible cache replay may need stronger language machinery than current build-system analogies provide. |
| [Multiple Streams with Recurrence-Based, Counter-Based, and Splittable Random Number Generators](https://www.informs-sim.org/wsc21papers/110.pdf) | Determinism at scale | High | Production | **2021** · L’Ecuyer et al. · Strong primary source for counter-based and splittable RNG design in parallel settings. Closest formal grounding for stateless keyed randomness. |
| [JAX PRNG design](https://docs.jax.dev/en/latest/jep/263-prng.html) | Determinism at scale | High | Production | **Active 2026** · Production counter-based, key-splitting PRNG design built around Threefry hashing. Not terrain-specific, but a very clean embodiment of deterministic parallel-safe randomness. |
| [Terrain Diffusion: A Diffusion-Based Successor to Perlin Noise in Infinite, Real-Time Terrain Generation](https://arxiv.org/abs/2512.08309) | Determinism at scale | High | Prototype | **2025 preprint** · Explicitly targets seamless infinite extent, seed consistency, and constant-time random access. Very on-point if it survives scrutiny. |
| [Fast Sphere Tracing of Procedural Volumetric Noise for very Large and Detailed Scenes](https://diglib.eg.org/items/366131fe-f2df-4841-89c3-f24c30f502f8) | Determinism at scale | Medium | Applied | **2025** · Zero-storage procedural detail for very large scenes with faster tracing of nested FBM structures. More rendering than terrain evolution, but still relevant to stateless detail generation. |
| [Integrating Large Language Models into Agent Models for Multi-Agent Simulations](https://informs-sim.org/wsc24papers/con310.pdf) | Hybrid symbolic-neural agents | High | Applied | **2024** · Hattori et al. · LLM inside an agent-based model rather than a free-roaming controller. Good early sign for architectures where LLMs perturb structured agent state. |
| [AgentSociety](https://arxiv.org/abs/2502.08691) | Hybrid symbolic-neural agents | Medium | Applied | **2025** · Large-scale social simulator with LLM-driven agents, realistic environment, and explicit engine substrate. More infrastructure than toy demo. |
| [Simulating Society Requires Simulating Thought](https://openreview.net/forum?id=EvXWexakZX) | Hybrid symbolic-neural agents | Medium | Prototype | **2025** · Position paper arguing for explicit cognitive modeling rather than prompt-only behavior. Relevant to cognitive level-of-detail ideas. |
| [Hierarchical Generative Agents for Simulating Sequential Human Behavior](https://arxiv.org/abs/2606.14989) | Hybrid symbolic-neural agents | Medium | Prototype | **2026** · Persona-conditioned agents in a cognitive hierarchy for evolving disaster environments. One of the clearer recent hierarchical stateful architectures. **POST-JAN-2026**. |

## Sharp takeaways

- The best recent terrain work is moving toward **authorable learned priors**, **multiscale amplification**, and **fast analytical approximations**, not yet a clean solution to real-time physically based erosion at both continental extent and metre precision.
- Inverse geomorphology is advancing fastest through **Bayesian and statistical wrappers around forward models** and through **better misfit functions**, not yet through mature end-to-end differentiable full-terrain simulators.
- For the detail-to-abstract problem, the nearest serious ingredients are **two-way global-local PDE coupling** and **self-adjusting computation**, but the exact memoized simulation under local discrete edits still appears open.
- Deterministic world generation at scale is better grounded today by **counter-based or splittable randomness** than by any terrain-specific standard.
- The strongest learned-surrogate imports are now coming from **physics-aware graph models**, **neural operators**, and **differentiable closure training on unstructured grids**.

## Current likely gaps

- A credible primary-source solution to **large extent and high precision physically based erosion in real time**.
- A credible primary-source solution to **upward invalidation from local discrete edits into a content-addressed macro simulation state**.
- A strong literature on **identifiability or system identification of adaptive LLM-like agents under perturbation**.
