# Frontier scan — adjacent / post-cutoff SOTA (2026-07-10)

*A live external research scan for work newer or adjacent to vivarium's substrate, run because an Opus-4.8 agent's knowledge is stale past ~January 2026 and the anchor review (`galin-2019-review`) is six years old. Offloaded to **Undermind** on 2026-07-10; **results pending** (fill §Results when they return). The reusable prompt is §Prompt. Epistemic status of anything landed from the results: **unverified external** until checked against a primary and, for citable claims, registered in relata (`BIBLIOGRAPHY.md`).*

## Why this scan

The design docs are grounded in four numerical primaries (`berger-1984-adaptive`, `gear-1984-multirate`, HMM, equation-free) and one 2019 terrain review — all pre-2020. The whole architecture bets on a frontier the field itself named open (large-extent + high-precision multiscale terrain; content-addressed memoized simulation; the detail→abstract up-invalidation problem). This scan looks for what has moved since, and for adjacent fields our training likely kept siloed (CG-terrain ↔ geoscience landscape-evolution software ↔ scientific-ML surrogates ↔ incremental-computation systems research).

## What vivarium already has (so results are read against it, not duplicated)

- Erosion: implicit stream-power ($\partial h/\partial t = -kA^m s^n + u$; `cordonnier-2016-large`, `braun-2013-very`), Priority-Flood → D8/MFD, Davy–Lague deposition, talus/thermal + hillslope diffusion.
- Hydrology: conserved local-inertial shallow water, timescale-separated from erosion.
- Substrate: AMR / multirate / HMM / equation-free multiscale; a lazy content-addressed **memoized query graph** (Salsa/Adapton/Nix lineage; save == store); coordinate-hashed **determinism** (no shared mutable RNG); layered stratigraphic columns; cube-sphere power-of-two quadtree keys; ~0.5 m voxels / ~20 km shell.
- The one open problem: **detail→abstract** — upscaling an irreducible discrete edit into a memoized macro with correct **up-invalidation**.

## The seven threads (see §Prompt for the dispatched form)

1. Physically-based & learning-based terrain / landscape-evolution post-2019 (diffusion/neural synthesis, differentiable/inverse LEM, real-time continental GPU erosion).
2. Geoscience landscape-evolution software (FastScape/fastscapelib, Landlab, Badlands, TopoToolbox) and recent algorithmic advances.
3. Modern multiscale & multirate numerics (AMReX/Chombo/p4est/Dendro; MRI-GARK; **learned closures / neural operators** — FNO, DeepONet, mesh-GNN PDE surrogates).
4. **detail→abstract** (the sharpest open problem): two-way coupling with conservative upscaling of perturbations; incremental computation with **upward** propagation (beyond downstream-only Salsa/Adapton); reversible/invertible simulation caching.
5. Determinism & procedural generation at scale (post-Gabor noise; splittable-hash parallel worlds; planet-scale procedural tech).
6. Hybrid symbolic-neural / LLM-driven simulation agents (LLM **perturbs** a formal state; cognitive LOD; believable-agent & LLM social-sim; identifiability under perturbation).

## Prompt

*(The dispatched prompt is preserved verbatim in the session transcript and mirrors the seven threads above; re-paste from there, or regenerate. Key invariants to keep on any re-run: front-load "what we already have" so the scan finds superseding/adjacent not redundant; flag anything after ~Jan 2026; demand primary sources with method/author/year/venue/DOI; ask explicitly for **no-gos / still-open** findings, since a credible negative on detail→abstract is as valuable as a positive.)*

## Results — Undermind first pass (2026-07-10)

*Initial scan; a deeper multi-prompt deep-search (finer paper-finding + analysis) is queued and will extend this. Citations preserved as DOI / arXiv id (the durable identifiers); status **unverified external** until primary-checked. ⭐ = high architecture-fit / worth a real read.*

**1. Terrain synthesis & amplification.**
- ⭐ **Terrain Amplification using Multi-scale Erosion** · Schott et al. · SIGGRAPH 2024 · amplifies low-res terrain into high-res *hydrologically consistent* detail by combining fast thermal + stream-power + hillslope across scales — the closest graphics paper to our "macro shape fixed, micro detail synthesized consistently" ($L$/lift + fidelity-ladder descent).
- ⭐ **Physically-based Analytical Erosion for fast Terrain Generation** · Tzathas, Gailleton, Steer, **Cordonnier** · CGF 2024 · DOI 10.1111/cgf.15033 · replaces long iterative uplift–erosion with an *analytical/procedural* formulation + multigrid — evidence the frontier is shifting toward closed-form-ish terrain operators that fit a **memoized query graph** far better (our line — Cordonnier).
- **Large-scale Terrain Authoring through Interactive Erosion Simulation** · Schott et al. · ACM TOG 2023 · DOI 10.1145/3592787 · interactive authoring + inverse-style uplift reconstruction.
- **Interactive Authoring of Terrain using Diffusion Models** · Lochner et al. · CGF 2023 · DOI 10.1111/cgf.14941 · conditional DEM diffusion with sketch/style control — a "terrain as editable prior" upstream of our deterministic stack.

**2. Differentiable & inverse landscape evolution.** Frontier is **Bayesian/statistical inverse wrapped around forward LEMs**, not yet end-to-end differentiable.
- ⭐ **Inverting Landscapes for Uplift Histories Using the Wasserstein Distance** · Morris et al. · JGR Earth Surf 2026 · DOI 10.1029/2025JF008966 · **POST-JAN-2026** (Mar 28 2026) · optimal-transport misfit to recover uplift from noisy landscapes — *note the W₁ tie to ASF/AAT's Wasserstein machinery.*
- **Bayesian Inversion of Fluvially-Incised Landscapes** · Oryan et al. · JGR Solid Earth 2025 · DOI 10.1029/2024JB030819 · recovers uplift + stream-power params around a fluvial forward model.
- Bernard et al. 2024 (neural inverse of denudation/capture, DOI 10.1029/2024JF007636); Curry & van der Beek 2025 (inverse LEM now operational for Earth-science inference).

**3. Geoscience LEM software + algorithmic advances.**
- ⭐ **fastscapelib** (Fastscape devs; active 2026) · reusable C++/Python kernels (flow routing, eroders, multi-grid, marine transport) — the geoscience stack most naturally composable with our query-graph runtime.
- ⭐ **Implicit Algorithm for Threshold Stream Power Incision** · Braun & Deal · JGR Earth Surf 2023 · DOI 10.1029/2023JF007140 · extends the $O(n)$ implicit FastScape solver to *thresholded* SPIM with rainfall/discharge variability — a direct supersession of our plain implicit stream-power core.
- **A Modular Implicit Numerical Method for Hillslope Sediment Transport** · Ren et al. · JGR Earth Surf 2026 (Jan 3 2026) · DOI 10.1029/2025JF008675 · implicit scheme for nonlinear hillslope transport — tightens our slow (creep/$z{=}2$) operator without architecture change.
- **Landlab** (MFD/D-∞, implicit overland flow, SPACE alluvium conservation, lithology helpers), **Badlands** (basin↔marine coupling), **TTLEM/TopoToolbox 3** — the modular process-graph comparison points.

**4. AMR, multirate numerics, learned multiscale surrogates.**
- ⭐ **Multirate Time-Integration through Adaptively Refined Meshes** · Doehring et al. · arXiv:2403.05144 (2024) · paired-explicit multirate RK where the *AMR structure drives the multirate schedule* — directly on-point for our power-of-two-lock item (§3 of `multiscale-seams.md`).
- **p4est** (Burstedde et al.; active 2026) — the octree/quadtree AMR workhorse, architecturally close to our cube-sphere quadtree; **AMReX/pyAMReX** (Myers et al. 2024, DOI 10.1177/10943420241271017) production Berger–Oliger AMR + Python/GPU; **Dendro** (wavelet-guided octree + symbolic codegen).
- **Self-Adjusting Multi-Rate Runge–Kutta** · J Sci Comput 2025 · DOI 10.1007/s10915-025-03049-y · slot-into-an-adaptive-runtime MRRK.
- ⭐ **Learned closures** (our fidelity-ladder "descend to a calibrated surrogate"): data-driven stochastic closure via conditional diffusion + FNO (JCP 2025, DOI 10.1016/j.jcp.2025.113919-ish — verify); **G-PARC** graph-physics recurrent surrogate on unstructured/moving meshes, *benchmarked on fluvial hydrology* · Sci Rep 2026 · **POST-JAN-2026** (Jul 2 2026) · DOI 10.1038/s41598-026-59318-9; differentiable GNN turbulence closure on unstructured grids (Comput & Fluids 2026, **POST-JAN-2026** Jul 4 2026).

**5. detail→abstract under edits — CONFIRMED GENUINELY OPEN.** No unified "irreducible local discrete edit → conservative content-addressed macro with upward invalidation" framework found. Nearest ingredients, which *do not meet*:
- **Concurrent two-way global/local coupling across non-matching discretizations** · Kang & Masud · Adv. Comput. Sci. Eng. 2025 · DOI 10.3934/acse.2025016 · closest to "local fine patch perturbs coarse global conservatively" — but PDE coupling, not memoized edit propagation.
- **Efficient Parallel Self-Adjusting Computation** · Blelloch et al. · arXiv:2105.06712 (2021) · closest systems precedent for upward invalidation in a memoized DAG — but no conservative *macro physics* update.
- fully-implicit adaptive VE↔full-dim coupling for porous media (Comput. Geosci. 2025, DOI 10.1007/s10596-025-10351-z); reversible-interpreter/partial-evaluation hint (Glück et al., arXiv:2412.03122) — suggests inverting cached execution may need reversible-language machinery.

**6. Determinism & procedural generation at scale.**
- ⭐ **Counter-based & splittable RNGs** · L'Ecuyer et al. · WSC 2021 · the best primary reference for our "determinism = stateless keyed hash" stance; **JAX PRNG** (Threefry key-splitting) is the clean production embodiment of exactly our discipline.
- ⭐ **Terrain Diffusion: a Diffusion-Based Successor to Perlin Noise for Infinite, Real-Time Terrain** · arXiv:2512.08309 (2025 preprint) · targets seamless infinite extent + seed-consistency + *constant-time random access* — near tailor-made for our tile-local deterministic worldgen (preprint; moderate confidence).
- **Fast Sphere Tracing of Procedural Volumetric Noise for very Large Scenes** · Moinet & Neyret · CGF 2025 · DOI 10.1111/cgf.70072 · zero-storage procedural noise at huge scale. (Little peer-reviewed *planet-scale deterministic* terrain per se — mostly generative or engine gray-lit.)

**7. Hybrid symbolic-neural / LLM simulation agents.** Identifiability / system-ID under perturbation LAGS behind believability + scale — *our AAT open question sits at the frontier.*
- ⭐ **Simulating Society Requires Simulating Thought** · NeurIPS 2025 position · argues realistic social sim needs *explicit cognitive modeling*, not prompted behavior — direct support for our cognitive-LOD (cheap far / rich near).
- **LLM-into-agent-models for multi-agent sim** · Hattori et al. · WSC 2024 · "LLM *perturbs* structured agent state," not "LLM drives the world" — our two-layer-mind shape, early signal.
- **AgentSociety** (arXiv:2502.08691, 2025; $10^4$ agents); **Hierarchical Generative Agents for Sequential Human Behavior** (arXiv:2606.14989, **POST-JAN-2026** Jun 12 2026) — persona-conditioned cognitive hierarchy.

**Post-Jan-2026 (new to this reader):** Morris Wasserstein-inverse (Mar 28); G-PARC fluvial surrogate (Jul 2); differentiable turbulence closure (Jul 4); Hierarchical Generative Agents (Jun 12); Bangerth *massively parallel flow routing / drainage-area* (arXiv:2606.12800, Jun 11) — DEM-scale hydrology infra worth watching.

**No-gos / still-open (confirmations that reshape nothing — they validate the plan):**
- continental-extent + metre-precision + real-time physically-based erosion — **still open** (recent work improves authoring/amplification/analytical speedups instead).
- **detail→abstract upward-invalidation — unsolved as stated.** (Our sharpest open problem is the field's too.)
- inverse geomorphology — still Bayesian-wrappers, not robust differentiable full-terrain simulators.
- LLM-agent identifiability/system-ID under perturbation — lags scale/believability. (Vivarium's AAT-sandbox bet targets a real gap.)

## Full-literature scans (deeper pass, 2026-07-10)

Two structured scans — queryable CSV + Undermind analytical summaries — now in the repo: `frontier-lit-terrain-2026-07-10.{csv,md}` (104 papers, ~82% coverage) and `frontier-lit-incremental-2026-07-10.{csv,md}` (38 papers, ~80% coverage). Query the CSVs (Cite Key / Year / Cit-per-year / DOI / **Relevance** / Abstract); the `.md` carries per-paper detail. Status: **unverified external** (relata registration in progress — the relata agent is mid-refactor; the reports ingest cleanly and `fetch --report` is ready for OA PDFs behind `--apply`).

**Terrain — the frontier *extends* the implicit-stream-power / graph-routing core, it doesn't replace it, in three directions:**
- *Efficient new implicit physics on the same core.* ⭐ **CHONK** (Gailleton et al. 2024, `gmd-17-71`) — cellular-automata × graph-theory coupling of water / sediment / lakes / provenance / lithology through *one interoperable structure*, flagged by the scan as "the most genuinely new architectural idea" — and it is **directly our heterogeneous-coupling / seam problem**. ⭐ **GraphFlood** (2024) — approximate 2D shallow-water on the flow DAG at LEM cost (~10× River.lab). Plus Braun–Deal threshold-SPIM (implicit, CFL-free) and Landlab gap-fillers (2D Exner, grain-size, landslide runout, provenance).
- *Inverse / differentiable.* Adjoint stream-power+diffusion (Petit 2025); Wasserstein objectives under geomorphic noise (Morris 2023/2026); JAX-MPM differentiable meshfree (2025).
- *Learning-based generators* — strong for authoring/amplification, weak for process fidelity (EarthGen Earth-scale cascaded; MESA / Terrain-Diffusion). Planet-scale source-to-sink is led by **global assimilative** models (Salles' hundred-Myr global *Science*; eSCAPE; gospl; Badlands) — the vein for our planet ambition.
- *Domain-agnostic corroboration:* **AdaScape** (Braun 2023 — a LEM coupled to tectonics + climate + **biodiversity**) and Salles' *landscape ↔ Phanerozoic biosphere diversification* (Nature 2023) are working **landscape↔life couplings at global scale** — the "generalize past terrain" direction already done in geoscience.

**Incremental / determinism — the two confirmations that matter most:**
- ⭐ **detail→abstract is genuinely unsolved** — the scan's explicit verdict: *"no retrieved 2023–2026 source directly solves conservative detail-to-abstract lifting after an irreducible local edit in a memoized deterministic simulator."* Nearest ingredients (which do **not** meet the need): **equation-free patch dynamics** (Karmakar–Dalal 2024/25; Roberts–Bunder–Kevrekidis — our gap-tooth-tiles substrate, actively advancing, but *downward closure only*); **perturbation-aware upscaling** (Hellman 2019; ⭐ **Fast Numerical Coarsening with Local Factorizations**, He–Otaduy CGF 2022 — runtime coarsening under local edits, a graphics-side near-hit); **conservative coarse/fine transfer** (⭐ **field-conserving AMR coarsening** on octrees, 2026 — child→parent conservation; **wavelet two-way self-nesting**, Dubos 2010). None does *content-addressed, memoized, **upward*** invalidation — that stays ours to invent.
- **LLM-perturbs-structured-state + believable-agent identifiability are effectively ABSENT** from the set — reconfirming the vivarium-as-AAT-sandbox identifiability bet is open territory, not a crowded field. Determinism grounding stays the older splittable/pedigree line (Leiserson 2012; Steele–Lea–Flood 2014 — the JAX-Threefry ancestors).

## Actions from results

- **Sortable matrix landed:** `frontier-matrix-2026-07-10.md` (Undermind; Fit / Maturity / POST-JAN-2026 columns + code-availability notes) — the tabular companion to this doc.
- **Deeper deep-search LANDED** (2026-07-10): both full-literature scans are in (see the section above + the `frontier-lit-*` files). Next: relata registration + OA fetch once the relata refactor settles; and a real read of the ⭐ additions below.
- **Read-first additions from the deeper pass:** **CHONK** (graph-coupling of heterogeneous processes = our seam architecture); **GraphFlood** (fast DAG hydro); **Fast-Numerical-Coarsening / field-conserving-AMR-coarsening / wavelet-two-way-nesting** (the conservative-upscaling ingredients nearest to detail→abstract); **AdaScape + Salles-biosphere** (domain-agnostic life-coupling at global scale).
- **Read-first shortlist** (⭐, by architecture-fit): Tzathas–Cordonnier 2024 analytical erosion; Braun–Deal 2023 threshold-SPIM (supersedes our core); fastscapelib (composable kernels); Doehring 2024 AMR-driven multirate (feeds the power-of-two-lock item); L'Ecuyer 2021 + JAX-Threefry (our determinism stance's primary refs); Kang–Masud 2025 + Blelloch 2021 (the detail→abstract frontier); Terrain-Diffusion 2025 (tile-local worldgen).
- **Relata registrations — DEFERRED (relata under heavy construction, 2026-07-10; revisit after the refactor).** Pending: **CCB\*18** (`cordonnier-2018-sculpting`, IEEE TVCG 24(5):1756–1769 — already *cited* in `DESIGN-SYSTEMS.md`, unregistered), plus any ⭐ items promoted to citable use. See the durable note in `BIBLIOGRAPHY.md`.
