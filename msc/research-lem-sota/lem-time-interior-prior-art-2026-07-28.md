# What the LEM community already knows about restart, cadence, and the balance criterion

*Literature sidebar, 2026-07-28. Written for the time-interior work on the fluvial kernel (interior stages as first-class store citizens, keyed by absolute epoch count, resting on chained-run ≡ one-shot-run bit-exactness). Not claim canon — nothing here is a vivarium claim until a segment owns it. Every citation below carries enough identity to verify; where I inferred rather than verified I say so in the line itself.*

**Reading order if you have five minutes:** §4.0 first — Braun & Willett's own accuracy claim covers the steady-state endpoint and explicitly *excludes* the transient, which is precisely what a time interior makes addressable. Then §1 (the balance criterion — it changes the shape of `#obs-erosion-residual-is-driver-bound`'s open item from "find a gate" to "compute a duration").

---

## 0. Scope, and what turned out not to be where I looked

Three questions were put to the literature: (a) known pitfalls making restart-from-checkpoint not reproduce a straight-through run; (b) any principled basis for output/snapshot cadence; (c) any standard erosion–uplift balance criterion for stream-power models under sustained uplift.

The short version of what came back:

- **(c) is the rich one, and the answer inverts the question.** The field's most-used steady-state criterion is exactly the one we measured to be unusable, the field mostly does not state its thresholds, and a 2024 short communication argues the numerical time-to-steady-state is *not a reliable quantity at all* — recommending an **analytical, a-priori** response time instead of a runtime gate. For `n = 1` that response time is a closed-form expression in static inputs. This is a stronger result than a convergence gate would have been.
- **(a) is an absence in the LEM literature and a mature engineering discipline one field over.** Landlab/terrainbento *achieve* exact restart when all state is carried — measured, bit-identical across 2/4/10/25-way chaining — but neither codebase guarantees, tests, or names the property, and the shipped default snapshot silently violates it. The real record is in Earth-system models, where exact-restart is a per-PR CI gate with named test types, and it supplies four failure shapes our list did not have. §2.
- **(b) is a verified absence in geomorphology** — zero occurrences of "restart", "checkpoint", or "spin-up" in any of the four Landlab-ecosystem papers — with one real mechanism and one real principle recoverable from the code, and the rigorous version of the question solved in a field nobody would think to look in (adjoint/AD checkpointing; and a 2019 system that is essentially this design). §3.

The most valuable thing I found was not on the list at all: §4, four measured defects in the exact kernel composition we run — one of which (§4.0) is a caveat the scheme's own authors stated in 2013 and that the field has since been quoting with the caveat stripped off.

**One thing this survey does not threaten:** the bit-exactness property itself. Nothing I found gives a reason a chained run at fixed per-epoch Δt would differ from a one-shot run in our kernel, and §4.0 explains *why* our composition has that property when most do not. Stage density can stay out of artifact identity. What the survey does threaten is the assumption that the interior stages, once addressable, are *accurate* — a separate claim, and one that has not been made yet.

---

## 1. The balance criterion — the field's answer, and why it is better news than a gate

### 1.1 The survey result: there is no standard, and the field knows it

**Gasparini, N. M., Forte, A. M., and Barnhart, K. R. (2024). "Short communication: Numerically simulated time to steady state is not a reliable measure of landscape response time." *Earth Surface Dynamics* 12:1227–1242. doi:10.5194/esurf-12-1227-2024.** (Open access; Copernicus.)

They surveyed 30 publications for the criterion used to declare steady state:

| criterion used | papers | stated a threshold? |
|---|---|---|
| rock uplift rate = erosion rate (or sediment flux = ΣU·A) | 20 | 3 |
| mean elevation unchanging | 4 | 0 |
| sediment flux unchanging | 2 | 0 |
| criterion not stated at all | 4 | — |

Verbatim on the ambiguity, which is the same ambiguity our own probe ran into: *"papers that do not use averaged criteria do not state where measurements are made: does every point on the landscape have an erosion rate that matches the rock uplift rate? Or is the average erosion rate equal to the average rock uplift rate?"*

So the honest report to `#obs-erosion-residual-is-driver-bound` is: **the dominant criterion in the literature is `E = U`, i.e. precisely the erosion–uplift balance our segment names as the criterion we actually need — and the literature almost never says at what tolerance, at what scale, or over what averaging window.** Our segment's FE(6) ("the criterion erosion actually needs is a statement about the erosion–uplift balance, which is open") is not open because we are behind the field. It is open because the field left it open.

### 1.2 The stronger claim: don't build the gate at all

Gasparini et al.'s central argument is that the numerically simulated time to steady state is dominated by artifacts, not physics: *"The predicted time to steady state from a numerical model is, in many cases, more reflective of drainage rearrangement and numerical artifacts than the time for an uplift wave to propagate through a fixed drainage network."*

Three specifics that land directly on us:

1. **Time-to-steady-state does not vary systematically with timestep.** *"for any given threshold value, the smallest estimated time to steady state is not necessarily produced using the smallest time step."* Non-monotonic. A gate calibrated at one epoch size tells you nothing about another.
2. **The culprit is flow re-routing on a raster.** They re-ran with the drainage network forced static and found *"there is much less variation among the time series with different time steps."* On Voronoi and hex grids the times were *"the most consistent."* **We reroute (Priority-Flood + D8 receivers + MFD) every epoch, on a raster.** That is the exact configuration they identify as least reliable.
3. **Floating-point precision sets the floor on any tolerance.** Verbatim: *"the time at which a particular metric appears to reach zero will depend on the floating point precision of the programming environment in which the metric is calculated."* Their metric time series in Figs. 2–3 run down to 10⁻⁶–10⁻¹⁴ m — reachable in f64, **not in our f32** (see §4.3).

Their recommendation is not a better gate. It is: *"One potentially useful approach is to consider the analytical response time to be a minimum response time"* — plus, where a range is wanted, ensembles over different random initial surfaces; plus, at minimum, *"it is critical to report both the metric being used and the threshold value for that metric."*

### 1.3 The analytical response time, and why `n = 1` is a gift

Gasparini et al. Eq. (9)–(11), following Whipple & Tucker (1999) and Whipple (2001), for `n = 1`:

$$T_A = \frac{\beta}{K}, \qquad \beta = k_a^{-\tfrac{m}{n}}\left(1 - \frac{hm}{n}\right)^{-1}\left(L^{\,1-\frac{hm}{n}} - x_c^{\,1-\frac{hm}{n}}\right)$$

with `k_a` and `h` the Hack's-law coefficients from `A = k_a (x_d)^h` (Hack 1957), `L` the longest channel length in the network, `x_c` the hillslope length (they set `x_c = 0` when diffusion is off). Valid when `hm/n ≠ 1`.

Every symbol on the right is a **static input or a network measurement available before the run**. Note what is absent: `U`. The response time is independent of uplift rate. That is not an accident of this formula — it is a structural property of `n = 1`, and the primary source is explicit about it.

**Whipple, K. X., and Tucker, G. E. (1999). "Dynamics of the stream-power river incision model: implications for height limits of mountain ranges, landscape response timescales, and research needs." *Journal of Geophysical Research: Solid Earth* 104(B8):17661–17674. doi:10.1029/1999JB900120.** (Free copy at `sseh.uchicago.edu/doc/Whipple_and_Tucker_1999.pdf`; already registered in our bibliography as `whipple-1999-dynamics`.) Read from the primary PDF, pp. 17668–17671:

- Eq. (25), the kinematic wave speed at which an erosional signal climbs the network: `C_e = −K k_a^m x^{hm} S^{n−1}`. **At `n = 1` the `S^{n−1}` term vanishes** — verbatim, p. 17671: *"for n = 1 the wave speed is independent of gradient."* The celerity depends only on position through drainage area, not on the evolving topography. The propagation problem decouples from the state.
- Eq. (29), dimensional response time to base-level fall: `T_b ∝ H K^{−1/n} U^{1/n−1}`, so at `n = 1`, `T_b ∝ H/K` with the `U` exponent zero. Verbatim, p. 17669: *"response time increases rapidly with uplift rate for n < 1, is independent of uplift rate for n = 1, and decreases rapidly with uplift rate for n > 1."*
- Eq. (34)/Fig. 7: at `n = 1` the response time is also independent of both the final uplift rate and the *magnitude* of the uplift change.
- Their own caveat, p. 17670, and it is ours too: *"Barring the effects of numerical diffusion, this is precisely the behavior observed in numerical solutions of the profile evolution equation."* See §4.1.

**Whipple, K. X. (2001). "Fluvial landscape response time: how plausible is steady-state denudation?" *American Journal of Science* 301(4–5):313–325. doi:10.2475/ajs.301.4-5.313.** The source Gasparini et al. cite for `T_A = β/K` specifically. *I did not read this one from the primary* — I have it only through Gasparini et al.'s Eq. (9) and the abstract. Worth chasing before a segment cites it directly.

**What this means for us.** The `erosion run length` **arbitrary** row in `ASSUMPTIONS.md` does not need a convergence gate to stop being arbitrary. It needs `K`, `m`, a Hack's-law fit of the tile's own drainage network, and the longest channel length — all of which we either have or can measure once per tile before running an epoch. `T_A/Δt_epoch` is a *derived, declarable epoch count*, and it is per-tile, which also handles the scheduling half of `#obs-erosion-residual-is-driver-bound` FE(3)–(5): the inert tiles get `L → 0` and therefore `T_A → 0`, converging at epoch 0 exactly as the segment's Working Note predicted from the subaerial-cell precondition, but now for a physical reason rather than a special case.

*Marking this honestly: the paragraph above is my inference from two verified formulas, not something the literature says about tiled planetary kernels. It is a hypothesis a probe could convict, not a result.*

### 1.4 The χ criterion — a balance test that is a shape test, not a rate test

The other half of the answer, and I think the more interesting one for a project that wants criteria which *can fail*.

**Perron, J. T., and Royden, L. (2013). "An integral approach to bedrock river profile analysis." *Earth Surface Processes and Landforms* 38(6):570–576. doi:10.1002/esp.3302.** (Author preprint, verified and read: `dspace.mit.edu/bitstream/handle/1721.1/75359/PerronRoyden-ESPL-preprint.pdf`.) Companion theory paper: **Royden, L., and Perron, J. T. (2013). "Solutions of the stream power equation and application to the evolution of river longitudinal profiles." *JGR: Earth Surface* 118(2):497–518. doi:10.1002/jgrf.20031** — *paywalled; I verified its citation and abstract only, not its contents.*

From the verified preprint, Eqs. (6a)/(6b):

$$z(x) = z(x_b) + \left(\frac{U}{K A_0^m}\right)^{\frac1n}\chi, \qquad \chi = \int_{x_b}^{x}\left(\frac{A_0}{A(x)}\right)^{\frac{m}{n}}dx$$

The point: **under a steady state with spatially invariant `U` and `K`, elevation is exactly linear in χ, with slope `(U/K)^{1/n}/A_0^{m/n}` and intercept the base-level elevation.** And tributaries are collinear with the main stem.

Why this is the criterion we want and a Δh tolerance is not:

- It is a statement about **shape**, so it does not care that the residual is pinned at the driver's rate. A driven steady state is precisely the case χ-linearity detects. Our measured problem — *"mean |Δh| per epoch stays near the uplift rate forever rather than falling to zero"* — is simply not an obstacle to it.
- It is **falsifiable in the way `#norm-probes-before-claims` wants**: fit `z` against χ, and *both* the residual of the fit *and* the fitted slope against the independently-known `U/K` must pass. A wrong answer is visible. Compare a Δh tolerance, which as our segment already establishes fires either never or immediately-and-wrongly.
- It **discriminates inert tiles for free**: a tile with no channel has no χ to integrate along, so it fails to produce a test rather than passing one spuriously. That is the opposite failure mode from the dangerous one FE(4) names.
- It gives the **direction of the residual**, not just the magnitude: a transient shows as a break in slope at the knickpoint's χ position, which is also a snapshot-cadence signal (§3.3).

Two honest caveats. Perron & Royden is a *field-data analysis* method; using it as a numerical-model convergence criterion is an application I did not find in the literature and am proposing, not reporting. And χ is defined per drainage network against one base level — on a planet with many basins per tile it has to be computed per-basin and aggregated, which is unaddressed anywhere I looked.

One incidental gift from the same paper, p. 7, lines 134–136: they warn about *"the 'quantization' effect introduced by a steepest descent path through gridded data, in which point-to-point distances can only have values of δ or δ√2."* That is `#obs-cube-locked-kernel-bias` FE(3), named independently by the χ community in 2013, and our retirement of uniform neighbour length in favour of true great-circle distances is the fix they recommend (use the actual `δ(x)`, not the nominal one). Corroboration, not news, but it is nice corroboration.

### 1.5 The CFL condition, which we may be silently violating or silently over-satisfying

Gasparini et al. Eq. (3)–(4) give the stability condition for the erosional wave, and it is cheap for us to evaluate:

$$C_{\max} \ge v\,\frac{\mathrm{d}t}{\mathrm{d}x}, \qquad v \approx K A^{m}\ \ (n=1)$$

with `C_max ≈ 1`. They estimate `v` from the **largest** drainage area in the network. In their setup (`K = 5×10⁻⁶ yr⁻¹`, `m = 0.5`, `A_max ≈ 49 km²`, `dx = 100 m`) this gave a stable `dt` of 2857 yr.

We have never, as far as I can find, computed this number for our kernel. It is one line from quantities we already have (`K`, `m`, max drainage area from the MFD accumulation, great-circle `dx` from `measure::gc_dist_m`), and it would tell us where our epoch sits relative to the explicit-stability scale — which is the natural companion number to `T_A`. Note that our implicit solve does not *require* it (that is the whole point of Braun–Willett), but §4.1 is about what you lose by exceeding it.

---

## 2. Restart-equals-continuous-run: a near-absence in LEM, a mature literature next door

*(§2.2–2.3 fold in findings from two parallel source-repo readings; see the note at the end of each.)*

### 2.1 The LEM literature does not treat this

I could not find a landscape-evolution paper that states, tests, or even raises the question of whether a restarted run reproduces a continuous one. That is a **verified absence** to the extent one search campaign can verify one: it is not in the Braun & Willett, Yuan et al., Campforts et al., or Gasparini et al. lines, and it is not a topic in the GMD/ESurf model-description papers I read. LEMs are not usually run as chained artifacts, so the question has not had occasion to arise.

The nearest thing in-field is oblique and comes from Gasparini et al.: their entire result is that **the trajectory is sensitive to things you would not think were state**. They show that re-deriving flow directions each step lets the network rearrange, and that this — not the physics — dominates the outcome. Their fix (force the network static) is exactly a "freeze the derived state" move. For us the implication is inverted and reassuring: because our per-epoch step re-derives *everything* from the height field, there is no hidden network state to fail to round-trip. **Our purity is what buys the bit-exactness, and it is worth saying so in the segment**, because it is a real design property and not a lucky accident.

### 2.2 The transferable prior art is in Earth-system modelling

**Liu, L., Peng, S., Zhang, C., Li, R., Wang, B., Sun, C., Liu, Q., Dong, L., Li, L., Shi, Y., He, Y., Zhao, W., and Yang, G. (2015). "Importance of bitwise identical reproducibility in earth system modeling and status report." *Geoscientific Model Development Discussions* 8:4375–4400. doi:10.5194/gmdd-8-4375-2015.** (Open access. *I read the discussion-paper version; I did not confirm whether the final GMD version differs.*)

Their framing is the one we are implicitly adopting and have not named: *"Bitwise identical reproducibility, i.e., bitwise computational results can be reproduced, guarantees the reproduction of exactly the same results."* Their empirical case is that round-off-level differences are not cosmetic — re-running CMIP5 historical experiments with only computing-environment differences produced *"significant differences or even contraries"* in decadal variation and in the sign of trends.

The taxonomy of what breaks it, as they name it: model code, input data, parameter setting, and the computing environment — *"parallel setting, compiler version, compiling option, processor version"*. Their point about longevity is the one most relevant to a content-addressed store: *"It is highly unlikely that a simulation setting can be recreated exactly after a number of years, because some parts of the original simulation setting are no longer preserved."* That is the problem `SRC_HASH` exists to solve, and it is worth knowing that the field that hit it first concluded the answer was to package the whole setting — which is what our complete-content-addressed key already does. Cross-reference `#form-complete-content-addressed-key`.

Also worth noting as prior art for the *failure*: the WRF community has a long-running, publicly documented issue that restart runs do not match continuous runs bit-for-bit (`forum.mmm.ucar.edu` thread "Restart run inconsistent with continuous run", thread 9436). *This is a forum thread, not a paper — cite it as evidence that the problem is real and common, not as a result.*

### 2.3 Landlab and terrainbento: exactness is achievable and nobody guarantees it

A parallel reading cloned both repos and **ran the experiment** rather than inferring from docs. Identity: landlab clone HEAD `0b0ef0864` (2026-07-10), terrainbento HEAD `e64a095e` (2024-09-11), released `landlab 2.11.0` / `terrainbento 2.0.1` / `numpy 1.26.4`, CPython 3.13. Everything below marked *measured* was run, not read.

**The encouraging half.** Landlab's SPACE component, driven directly, **is exactly restartable**: straight-through 200 steps versus chained runs cut into 2, 4, 10, and 25 legs came back `bit_identical=True`, `max|Δ| = 0.0`, on elevation, soil depth *and* bedrock elevation, at every leg count — provided all three fields round-trip. terrainbento's `Basic` (elevation its only integrated state) likewise bit-identical at 1, 2, 4, 8, 20 legs. **In a serial float64 kernel with all state carried, exact chaining simply holds.** Our design's load-bearing property is the kind of thing that is true when you arrange for it, and this is the closest thing to external confirmation available.

**The alarming half, and it is a state-selection failure, not a numerics failure.** Same SPACE setup, restarted from *only* `topographic__elevation` — which is exactly what a default terrainbento netCDF snapshot contains — diverged by **12 % of total relief in elevation and 41 % of relief in soil thickness after 100 steps**. Not drift; a different landscape.

Three specifics worth having:

1. **The shipped default snapshot is not a state snapshot, and the paper of record says it is.** `terrainbento/base_class/erosion_model.py:443` — `fields = fields or ["topographic__elevation"]`. Measured on a `BasicSa` run: the model integrates 16 node fields including `soil__depth` and `bedrock__elevation`; the default netCDF contains `['topographic__elevation', 'x', 'y']`. Meanwhile **Barnhart et al. (2019), *GMD* 12:1267–1297, §5.6, p. 1285** states verbatim: *"A terrainbento output file contains all of the grid fields used in that particular simulation…"* (checked against the PDF text). A reader trusting the paper would believe the output is restartable. It is not.
2. **`BasicSa` cannot be faithfully restarted through its public API at all.** `model_basicSa.py:167–168` reconstructs `bedrock__elevation` at construction as `z − soil_thickness`, via `add_zeros`, which *raises* if you supply the field yourself (measured: `FieldError`). And the reconstruction is lossy — measured, after 100 steps the independently-integrated bedrock field differs from `z − soil` on **71 of 600 nodes by 1 ULP**, because the diffuser writes `z = b + s` and `fl(fl(b+s) − s) ≠ b`.
3. **RNG is process-global mutable state**, which is worse than a stream-position problem: `generate_uniform_precip.py:722–723` calls `random.seed()` and `np.random.seed()` on the *legacy globals* from `__init__`. Merely instantiating the component silently reseeds the process. Not applicable to us — our erosion loop draws nothing (§4.4) — but worth knowing the shape exists.

**A non-finding that is itself informative:** the only parallelism in Landlab is hex-grid *geometry construction* (disjoint writes, no reductions). The physics is serial. So this ecosystem has **no experience to offer on float reassociation under parallelism** — it never came up. If we parallelize (§4.0), we are on our own.

**The verdict on the brief's four options — `guarantee` / `test` / `silently assume` / `known to violate` — is *silently assume*, and the evidence for that specific reading is worth keeping separate.** There are no restart-*mismatch* bug reports, but that is because there is no feature to file bugs against, not because the property holds. What exists is restart-*discussion*:

- `landlab#1305` (2021, open) — checkpointing requested by a user hitting SLURM walltime. Barnhart: *"this feature is not formally implemented in landlab… you'll also need to save and re-build the components. If you use components that use random number generation you'll need to be careful to re-set them correctly."* A user reports components losing grid references on unpickle and offers a fix; five years later it has not landed.
- `landlab#2111` (2025, open) — `EventLayers` not restored; `__getstate__`/`__setstate__` bugs. mcflugen: *"I've known this to be an issue… since no one had reported it as a problem I assumed it wasn't widely used… my plan was to just delete them."*
- `landlab#94` — "add load/save methods" — open since **2015**.
- And the tell that makes it *silent* rather than merely absent: `landlab/tests/io/test_read_write_native.py:53` reads `if not np.array_equal(np.asarray(o1), np.asarray(o1)):` — **`o1` compared against itself.** Array-valued mismatches reached through that path can never be detected. Present at clone HEAD. The pickle round-trip test is weaker than it looks, which is the difference between "untested" and "believed tested."

CSDMS conceded the point at the standard level too: `csdms/bmi#12` (2018, still open), ipelupessy — *"quite hard in practice to achieve complete coverage… and its difficult to achieve bitwise consistent restarts this way."* Serialization state is still only a *proposed* role in the BMI v3 roadmap (2025).

### 2.3a Do not conclude that these systems self-correct

Worth recording because the measurement invites the wrong generalisation. In the SPACE/`BasicSa` experiments the 1-ULP residual **decayed**: the 2-leg discrepancy went 8×10⁻¹⁵ at T=1000 to bit-identical again by T=50000, and a deliberate 1-ULP initial perturbation never flipped a single flow receiver (0 of 1200) across three parameter regimes. Those runs were dissipative and relaxing toward steady state.

**That is a regime observation, not a property.** Two counterweights:

- The ESM record documents last-bit seeds *growing* — `MOM6#223`, `CROCODILE-CESM/MOM6#2`. Contraction is regime-dependent.
- And the mechanism that suspends any contraction argument is the one from `landlab#1670`: **the moment a tie-break can flip a receiver, the perturbation stops being a perturbation and becomes a different drainage network.** Contraction reasoning applies to the arithmetic and not to the discrete decisions layered on top of it, and in a LEM the discrete decisions are where the dynamics live (§1.2).

A second independent probe (2-D D8 + implicit SPL, flow network recomputed every step, 80×80) reproduced the contraction and sharpened it: a 1-ULP perturbation decays to **exactly zero within ~25 steps**, and even a 1 mm perturbation that *flipped a receiver* at step 10 healed to bit-identity by step 400. So the contraction survives a receiver flip, at least at toy scale.

For us this cuts one way only: bit-exactness is the property to hold, and "the error would have decayed anyway" is not an argument we should ever accept as a substitute for it.

**But the noise literature says where the sensitivity actually lives, and the answer is uncomfortably specific to us.** **Morris, C., and Roberts, G. (2025). "Impact of noise on landscapes and metrics generated with stream power models." *Earth Surf. Dynam.* 13:1003–1038.** Noise below **1 % of cumulative uplift** makes recovered tectonic rates wrong by more than **2×**; landscape *geometry* is noise-divergent while *distributional* metrics stay stable; they recommend seeded ensembles of hundreds to thousands of runs.

Reconciling that with the contraction result: the divergence budget is spent **early, during network organisation**, not at restart boundaries — near the attractor the scheme forgets perturbations, but *which attractor* was selected is set in the first phase and never revisited. Two consequences:

1. It corroborates rather than contradicts the contraction probes, and it explains them.
2. **The early network-organisation phase is precisely the part of the run a time interior makes addressable and watchable.** So the stages a viewer will find most interesting are the ones where the trajectory is least robust to perturbation — which is an argument for bit-exactness being load-bearing rather than merely tidy, and an argument that a world's seed is load-bearing far beyond cosmetics.

### 2.4 The engineering record is in Earth-system models, where it is a per-PR CI gate

This is the transferable part, and it is more mature than anything in geomorphology. CESM/CIME has *named test types* for exactly our property — `ERS`/`ERT`/`ERI`/`ERP`/`ERIO`/`IRT` for exact restart, `REP` for run-to-run reproducibility, `PET`/`PEM`/`PEA` for thread- and MPI-count bit-for-bit. MOM6's `.testing` has `test.restart`: *"Compare a single run to two runs separated by a restart."* The CESM wiki states the goal in the same words we would: *"so that you can't tell the difference between a continuous simulation and one that starts up from a restart file."*

**Four failure shapes from that corpus that our list did not have, ranked by how much they worry me for this design:**

1. **Sub-cycling phase — and the test that structurally cannot see it.** `ESCOMP/CAM#655`: restart was bit-for-bit from *odd* timesteps and not *even* ones, because radiation is sub-cycled every other step and the phase was not captured. adamrher: *"since our restart tests always restart on an odd time-step, we don't actually test for this and so this seems to have slipped through the cracks."* **If anything in our pipeline is sub-cycled with period `p`, chaining is exact only at `k ≡ 0 mod p` — and a bit-exactness test that cuts at round numbers (10, 20, 40) will never find it.** The corrective is free: cut the chained run at *coprime and adversarial* offsets (7, 13, 1, `epochs−1`), not at halves and quarters. I would put this in the test before it lands.
2. **State destroyed by re-derivation at init.** `MOM6#1119`, `#1128`: a value was correctly *in* the restart file and then overwritten by a recomputation on the first step back. It passes every "did I serialize everything?" audit, because the answer is yes. Combined with the `BasicSa` finding above, the general shape is: **any quantity we store *and* derive is a hazard, and the hazard is invisible to serialization review.** Our `drainage` field is exactly this shape (§4.4) — it is stored in the struct and re-derived each epoch. The re-derivation is what makes us safe, but it is safe by accident of ordering, not by design, and that is what the probe in §4.4 would pin.
3. **Signed zero under hashing — a hazard *created by* content-addressing.** `MOM6#1042`: `-0.0` and `+0.0` are numerically identical states that hash differently. `#form-complete-content-addressed-key` keys on complete inputs; if a stage key is ever a hash over field bytes, we need a **documented canonicalization** (normalize signed zero, canonical NaN, fixed endianness) that is *part of the hashed law*, not a serializer detail. Note we already do the analogous thing correctly on the comparison side (`f32::total_cmp`, §4.2) — this is the same problem one layer out. Related and near-free for us: MOM6 stores per-field checksums *inside* restart files and validates on load, converting "silently wrong stage" into "loud refusal."
4. **The referent is negotiable.** `GEOS-Chem#2521` achieved exactness partly by making the *continuous* run round-trip through the storage representation every step — rather than making the chained run match a privileged unchained path. If our stored field ever differs from the working field in precision, units, or normalization, that inversion is available and is not obvious. Worth an explicit consider-and-reject rather than leaving it implicit. (Second-order lesson from the same PR: single-precision output had *concealed* a separate real bug for years. Lossy storage does not only break chaining — it hides what else is broken. We are `f32` throughout; see §4.3.)

**And one reframing of our own risk ranking, which I think is right and which we would not have arrived at alone.** The one time Landlab actually met a determinism bug (`landlab#1670`) it was **not** float arithmetic — it was `argsort` tie order changing with a NumPy version, fixed with `kind="stable"`. Landscape evolution is saturated with ordering decisions: priority-flood queues, drainage stacks, equal-elevation flats, equal steepest slopes. **Tie-breaking is a higher determinism risk than float reassociation, because it survives every precision audit you run.** Our Priority-Flood already does this right (§4.2); what has *not* been checked is `receivers()` (equal steepest slope among 8 neighbours) and `elevation_order()` (equal elevations). Those are the two places I would look next in our own code.

---

## 3. Snapshot cadence

### 3.0 The hazard the question did not ask about: cadence changing the physics

**This is the most important thing in §3, and it is not about restart at all.** The framing "does chunking perturb the trajectory?" points at restart as the risk. The worst failures found in the wild are cadence altering the answer **with no restart involved** — a running simulation whose output frequency changes its own physics.

- **Badlands, measured — with the magnitude attributed carefully, because my first pass got it wrong.** Varying *only* the output display interval changes the final topography by up to **15.6 m** (display = 50 ka vs 10 ka). But that headline figure is **dominated by a separate defect**: Badlands 2.3.1 never simulates its final output interval at all (`model.py:749–794`; a run reporting 100 kyr delivers 75), so most of the 15.6 m is missing time rather than perturbed physics. **Isolated from that bug, the genuine cadence-into-physics effect is 0.31 m across 1489 of 1491 nodes** — two runs differing only in *where* the adaptive CFL timestep got clipped (`buildFlux.py:299`, `CFLtime = min(CFLtime, tEnd - tNow)`). Smaller than the headline, and still the thing that should not exist. The documentation's only remark on the parameter is that a larger value "will make your simulation run quicker."
- **terrainbento, measured.** `ErosionModel.run_for` truncates the last step of each segment to land on output times (`erosion_model.py:791–811`), so final elevation is a function of `output_interval` unless it divides `clock.step` exactly. Worse, float drift in `next_output += interval` generates spurious ~10⁻¹⁶ steps *even when it nominally divides* — 220 steps where 200 were expected. Each is a no-op for elevation but **not for the RNG stream** in stochastic models. Nothing checks or documents any of this.

  *(This corrects §3.1a below, which reported terrainbento's cadence as observationally inert. Both measurements are real and they are about different code paths: `_update_output_times` quantizes the requested output time upward onto the step grid, while `run_for` truncates a segment's final step to land on a pause. The first protects the trajectory; the second does not. The system contains both, which is itself the lesson — one path defends the invariant and another quietly breaks it.)*

- **xarray-simlab gets it right by construction, and is the only surveyed system besides ours that does.** It separates a **main clock** (every step the model takes) from **output clocks** (when snapshots are saved), and *enforces* that output clocks be a subset: `xr_accessor.py::get_output_save_steps` builds save masks via `np.in1d(main_clock, output_clock)`, and `_uniformize_clock_coords` raises `ValueError: "Clock coordinate {clock} is not synchronized with main clock…"` otherwise. **Snapshot cadence therefore cannot perturb the Δt sequence — it is pure selection.** Nobody states this as a property; it is just how Bovy built it.

**The converse invariant, which I think belongs in the segment as law.** Our design already has this property — stage boundaries partition an epoch sequence that is fixed by absolute epoch count, and no stage boundary touches Δt, the epoch count, or where uplift is applied. It is worth *stating*, because:

1. Every cadence failure found in the wild is a violation of exactly this invariant (Badlands display-clipping, terrainbento step-truncation, CHILD collapsing restart granularity onto output granularity).
2. The tempting future optimization — "coarsen the quiet stages," "adapt Δt between beacons" — **is the same violation wearing a performance win.** Stated as law, that proposal is visibly out of bounds. Unstated, it looks like a good idea.

### 3.1 The absence

**I could not find any principled basis for output cadence anywhere in the LEM literature.** What exists is convention, exposed as a user parameter with no guidance:

- LSDTopoTools exposes an *"output file interval"* parameter in its landscape-evolution documentation (`geos.ed.ac.uk/~smudd/LSDTT_docs/html/model.html`) with no stated basis for choosing it.
- Gasparini et al. (2024) compute their steady-state metrics *"over a temporal difference of 100 000 years, which was set by the longest time step of the simulations being compared"* — a comparability constraint, chosen so the metric is a Δ over a fixed *physical* interval independent of `dt`. **That is a real methodological principle even though it is about metrics rather than snapshots, and it transfers directly: our stage spacing should be defined in absolute epoch count, which is exactly what the design already does.** Good sign.

I looked in the model-description papers, the LEM software docs, and the geomorphology methods literature. The parallel source reading independently searched the full text of all four Landlab-ecosystem papers (terrainbento GMD 2019, Landlab v2.0 ESurf 2020, Hobley ESurf 2017, SPACE GMD 2017) for `restart | checkpoint | spin-up | spinup | reproduc | determinis | output interval | snapshot`, and the whole terrainbento repository besides. **Zero occurrences of "restart", "checkpoint", "spin-up", or "spinup" in any of the four papers, and zero in the entire terrainbento repository.** The only cadence content is mechanical. Reporting the absence as an absence, with the search traceable.

### 3.1a But there is one real mechanism, and one real principle

**The mechanism: quantize the request onto the integration grid; never subdivide a step.** terrainbento makes output cadence *observationally inert* by construction — `ErosionModel._update_output_times` (lines 963–977) rounds any requested output time up to the next multiple of the model step and warns. Measured: requesting `output_interval=333` with `step=10` yields snapshots at 340, 670, 1000, …, and the final topography is **bit-identical across requested intervals of 2000, 1000, 500, 250, 100, 333, 137, and 55**. Our design is structurally stronger — keying stages to absolute epoch count makes alignment a property rather than a user obligation enforced by a runtime warning — but it is worth knowing someone else arrived at the same principle from the other direction.

**The principle, from the single place in the ecosystem where cadence is chosen for a reason:** `notebooks/coupled_process_elements/model_basicCh_steady_solution.ipynb` uses the output writer *as a convergence detector* — at each snapshot it compares `max|Δz|` since the last one against a tolerance, and either declares steady state or extends the clock by another interval. **Cadence chosen as the timescale over which you want to be able to detect change.** That is a defensible basis and it is the only one I found.

With a sting in the tail that transfers directly to us: because the criterion is snapshot-to-snapshot Δz against an *absolute* tolerance, a shorter interval means a smaller Δz means **earlier false convergence**. In the one place cadence is used for anything principled, it stops being inert — it changes when the model stops. **Any quantity we derive *between* stages inherits the stage density.** That is exactly the shape of our `last_delta_m` hazard (§4.4), arrived at independently, and it is the argument for defining stage-derived residuals over a *fixed absolute epoch interval* rather than "since the previous stage" — which is also, and not coincidentally, what Gasparini et al. do with their 100 000-year window.

### 3.2 The principled literature is in automatic differentiation

The problem "store some states, recompute the rest, minimise recomputation under a storage budget" is *solved*, provably optimally, and the solution has been in production for 25 years — in adjoint/AD, not geoscience:

**Griewank, A., and Walther, A. (2000). "Algorithm 799: Revolve: an implementation of checkpointing for the reverse or adjoint mode of computational differentiation." *ACM Transactions on Mathematical Software* 26(1):19–45.** Revolve generates a checkpointing schedule that is *provably optimal* — minimising recomputation steps given the total number of timesteps and the number of checkpoints allowed in memory. The optimal placement is **binomial, not uniform**. It is used by ADOL-C, Tapenade, and dolfin-adjoint.

Successors worth knowing exist if this becomes load-bearing: Zhang et al. (2023) on optimal checkpointing for adjoint multistage time-stepping (arXiv:2106.13879), and Maddison (2023) on step-based checkpointing with high-level AD (arXiv:2305.09568). *I verified these exist and are on-topic; I did not read them.*

**The number to hold:** under binomial checkpointing both the memory and the recompute ratio grow only **logarithmically in `n`**. Dense uniform-every-N leaves a great deal on the table, and the gap widens with chain length.

**The caveat that matters for us:** classic Revolve assumes a known `n` and a *reverse-sweep* access pattern — every state once, in reverse order. Our demand is different: a viewer wants an arbitrary interior stage on request, repeatedly, in no particular order. So Revolve is not the answer as stated; the **online-checkpointing variants** (which drop the known-`n` assumption) are the branch to read if this becomes load-bearing. What transfers regardless is the central result — optimal placement is *non-uniform* — and the fact that the question has a rigorous form at all.

**One near-miss to avoid:** the Young/Daly optimal-checkpoint-interval result is the famous one in this space and it is the *wrong* one for us. It optimises against stochastic hardware failure. Nothing in a deterministic build crashes.

### 3.2a Someone has already built our time interior as a general system

**Di Girolamo, S., Schmid, P., Schulthess, T., and Hoefler, T. (2019). "SimFS: A Simulation Data Virtualizing File System Interface." arXiv:1902.03154.** *(Citation verified; contents not read by me — relayed from the parallel reading.)*

This is the time interior described as a general system in 2019: sparse checkpoints plus **on-demand re-simulation**, with a materialization policy driven by observed access patterns. That is `#form-time-indexed-stage-chains` FE(8)'s materialized-only chain plus a demand-shaped densification policy, built as a filesystem interface. It is the closest prior art to the whole design that anyone found, and it is not in geoscience — which is why nobody in the LEM world would have pointed at it.

**Its §II states our load-bearing property as a stated precondition, verbatim:** *"SimFS requires that the simulation can be re-started from checkpoints and delivers a bitwise-identical output to the original run"* — citing **Arteaga, Fuhrer & Hoefler (IPDPS 2014)** for how to obtain bit-reproducibility cheaply. So the design pattern *and* its precondition *and* a citation chain for satisfying the precondition all already exist, assembled, six years before we needed them.

### 3.2b The cadence literatures, re-ranked for our actual access pattern

Our demand is repeated scrubbing of a chain with recompute-from-ancestor — not a reverse sweep. Re-ranked on that basis, and this displaces Revolve from the top:

1. **`rr`, the reverse debugger** (`rr-debugger/rr`, `src/ReplayTimeline.cc:1523`) — an observer-relative **exponential checkpoint ladder**, O(log L) checkpoints, where **density is a policy over a stable address space rather than a property of the artifact.** That is exactly our "stage density stays out of identity" invariant, built and shipped in a production debugger. Of everything surveyed this is the closest match to what we are doing and how.
2. **Dataset versioning.** Bhattacherjee et al., *PVLDB* 8(12):1346 (2015) — our problem is their tractable **linear-chain** case; and **Toueg & Babaoğlu, *SIAM J. Comput.* 13(3):630 (1984)** is the O(n³) dynamic program for *optimal* checkpoint placement on a chain. Provable optimality for our exact topology, forty years old.
3. **In-situ visualisation "trigger" literature, for *which* stages to keep.** Kawakami et al., ISAV'20 (10.1145/3426462.3426469) give a DP for optimal K-snapshot selection minimising linear-interpolation reconstruction error. **With an empirical caution I would not skip:** entropy-triggered selection *lost* to evenly-spaced on one of their two test simulations. Change-driven cadence only pays when the change rate is strongly non-uniform — a landscape's is, but **a probe must beat the evenly-spaced null, not assume it.** SalienTime (CHI 2024) found pure saliency-driven selection clumps and needed an explicit spacing penalty — which is precisely the failure mode a naive Δh-driven ladder would hit during a drainage reorganization.
4. **Revolve** (§3.2) stays relevant for the log-`n` result and the non-uniformity insight, but its reverse-sweep assumption is not our access pattern.
5. **Bar-On et al., ICALP 2018, "Tight Bounds on Online Checkpointing"** — if we ever cap stored stages per world at `k` with relocate-to-now as the only move, the tight discrepancy bound is ln 4 ≈ 1.386.
6. **Daly 2006** is the wrong shape (no stochastic failure process here), but one structural lesson survives: optimal interval ∝ √(write-cost / recompute-rate).

**And a physics-derived ladder worth a probe, marked clearly as proposal not prior art.** Under `n = 1`, χ-space turns SPL into constant-celerity advection (§1.3), so uniform-in-`t` cadence is *already* principled for the propagation phase — this is §3.3, independently reached. The *relaxation* phase wants equal-decrement-of-disequilibrium spacing, `t_k = −τ·ln(1 − k/K)`: dense early, geometric late. Which is `rr`'s exponential ladder, derived from the physics rather than from debugger ergonomics. **Design constraint attached:** compute any such ladder as a *selection policy over epoch-keyed stages*, never as an alternative addressing scheme — otherwise the boundary set enters identity and FE(9)'s subset property is lost.

**The one fact that would sharpen all of §3, and which nobody has:** the *interior access pattern*. Sequential scrub, random-access interactive, or reverse sweep? That single fact is what separates Daly from Revolve from `rr`/SimFS. Worth pinning in the segment, because the design currently says stage density is a demand parameter without saying what shape the demand has.

### 3.2c Why this theory never reached geomorphology

Worth one line of synthesis, because it explains the absence rather than just reporting it. **Six literatures, one object:** *snapshot* (geomorphology) = *checkpoint* (automatic differentiation, reverse debuggers) = *restart dump* (HPC / Earth-system modelling) = *materialized version* (databases) = *time slice* (scientific visualisation) = *savepoint* (stream processing). No shared vocabulary, so no citation path. The crosswalk is itself the finding.

### 3.3 The physically principled cadence, if we want one

Following from §1.3–1.4, and marked clearly as **my derivation, not literature**: at `n = 1` the erosional signal is a kinematic wave of gradient-independent celerity, so it advances at a constant rate in χ. Therefore **uniform-in-epoch stage spacing is already uniform-in-χ-progress of the knickpoint** — the eye sees the wave move at constant speed. That is a genuinely defensible basis for the "every N epochs" convention *in the `n = 1` case specifically*, and it would not hold at `n ≠ 1`, where the celerity depends on the evolving gradient (Whipple & Tucker Eq. 25) and uniform-in-time cadence would show the wave accelerating or stalling.

If that is right, the honest declaration for stage density is not "arbitrary" but "uniform in epoch, which at `n = 1` is uniform in signal propagation; this ceases to be principled if `n` ever leaves 1." A cheap probe could convict it: track a knickpoint's χ position across stages and check the spacing is constant.

---

## 4. Incidental: three defects in our exact kernel composition the field has already measured

This section was not asked for and is, I think, the most useful part.

### 4.0 The scheme's own authors say the transient needs a timestep limit — and the transient is the whole point of the time interior

**This is the finding I would most want read before the design is committed.** Read from the primary, which we already hold locally at `ref/geology/pdfs/braun-2013-very.pdf`: **Braun, J., and Willett, S. D. (2013). "A very efficient O(n), implicit and parallel method to solve the stream power equation governing fluvial incision and landscape evolution." *Geomorphology* 180–181:170–179. doi:10.1016/j.geomorph.2012.10.008** (already `braun-2013-very`).

Their §7.2, verbatim, after demonstrating stability across four orders of magnitude in Δt:

> *"The solution is stable at all time steps, and its overall final geometry … is almost identical in all four simulations, showing that the algorithm is not only stable but first-order accurate for long-term (steady-state) solutions. **However, to capture the transient behavior of the solution with accuracy, limits must be placed on the time step (a) due to the explicit nature of the scheme in its dependence on drainage area, A, and (b) to satisfy the courant condition for the propagation of knickpoints – or 'topographic waves' – on the landform.**"*

**And their Appendix B, "Limit on time step," gives that limit in closed form** — read from the same PDF, p. 178–179. This is the part nobody quotes, and it is the most directly usable equation in this entire survey:

$$\frac{u\,\Delta t}{\Delta x} < C_{\max},\quad u \approx K A^{m} \;\overset{A = k_s x^{p}}{\Longrightarrow}\; \Delta t < C_{\max}\frac{\Delta x}{K k_s^{m} L^{pm}} = C_{\max}\frac{10^{4}}{n_x}$$

(Eqs. B.1–B.5, evaluated at `m = 0.5`, `p ≈ 2`, `k_s ≈ 1/4` — Hack's law folded in, so this is a *simpler* form of the same bound Gasparini et al. reach in §1.5.) Then, verbatim:

> *"Using an explicit scheme, C_max ≤ 1 and the Courant condition is very restrictive, imposing that the time step be less than 10 y. Using the implicit scheme described here, a larger value of C_max is acceptable and using a time step of 1000 y produces an accurate solution. … **the evolution of the mean topography with time is shown for the three model runs, indicating that the transient solution depends on the time step length.** The solution with a very small time step (Δt = 1 y) is also shown for reference, **demonstrating that the transient solution converges for values of Δt ≤ 1000 y**."*

**So the honest reading is narrower than "the transient is excluded" and much narrower than the circulating "accurate at 100× the stable timestep."** The transient *is* validated — but only below a quantified bound, and the famous 100× figure is precisely this: their explicit Courant limit is 10 y, their validated transient limit is 1000 y, at *their* parameters and grid. What gets dropped when the figure is repeated is that (a) it is a transient-*convergence* threshold, not a license, (b) Eq. (B.5) makes it depend on `K`, `Δx`, `n_x` and channel length, so it does not travel between models, and (c) their own Fig. 4d shows the transient solution *moving* with Δt above it.

**Corroboration that this is not an obscure reading: it is in Landlab's own source, twice, verbatim.** The `FastscapeEroder` and `StreamPowerEroder` docstrings carry *"the accuracy of transient solutions is \*not\* timestep independent (see Braun & Willett 2013, Appendix B)"* — added by Dan Hobley in 2016 with the commit message "per revisions to the paper." Two independent readings flagged the Appendix-B pointer as unverifiable behind the paywall; we hold the PDF, and it checks out. Landlab points at Appendix B, our §7.2 quote is the prose version, and both are the same caveat stated twice in the same paper. What has not propagated is the *equation* — the abstract's "large time steps without sacrificing accuracy" is what gets quoted downstream, and Eq. (B.5) is what it is quietly conditioned on.

For us: a build that memoized only the endpoint sat in the regime where Δt barely matters. **A time interior is an assertion about exactly the quantity Fig. 4d shows moving.** Nothing here breaks bit-exactness (§4.1(1) stands), but the interior stages we are making addressable are the ones whose accuracy is bounded by an inequality we have never evaluated — and Eq. (B.5) means evaluating it is arithmetic, not research.

Two supporting specifics from the same paper, both worth knowing:

- **The scheme is semi-implicit: drainage area lags one step.** §4, verbatim: *"the drainage area, A, that appears in the right-hand side of Eqs. (22) and (24) is calculated at time t. This means that our method is not fully implicit (with respect to drainage area) but relies on the assumption that the rate of change of the geometry of the drainage network is slow in comparison to the rate of change of the elevation … **Drainage reorganization will lag one time step behind the erosion rate calculation, which imposes a time step limit for accuracy, which is difficult to quantify.**"* This is the *same mechanism* Gasparini et al. (2024) found dominates time-to-steady-state (§1.2). Two independent lines arriving at drainage rearrangement as the thing that actually controls transient behaviour is, I think, the strongest signal in this whole survey.
- **The `n = 1` solve is a sequential triangular sweep, and that is what makes it pure.** Their Eq. (22): each node's new height is a closed-form function of its own old height and its receiver's **already-updated** new height, evaluated in stack order — *"the stack order provides the way to order the equations in a way that makes the resulting system of algebraic equations upper (or lower) triangular."* So the step is order-dependent by construction, and is a pure function of the height field **exactly to the extent the stack order is deterministic**. Ours is (§4.2). Worth stating explicitly in the segment rather than leaving implicit, because it is the hinge the whole design hangs from.

  Forward-looking, since parallelism is the classic bit-exactness killer and we are single-threaded today: Braun & Willett note the solve distributes *per catchment*, and catchments are independent, so **catchment-level parallelism would not perturb the result**. The MFD drainage-area accumulation is a different matter — it is a summation, and any reassociation of it across threads would break bit-exactness. If parallelism ever arrives, that is where to look first.

Also worth a look while we are in this paper: their §6 handles local minima by **routing water across the sill** to a neighbouring catchment rather than filling with an ε-gradient — a deliberately different choice from ours, made to avoid exactly the mass-and-geometry artifact §4.2 is about. The modern successor is Cordonnier, G., Bovy, B., and Braun, J. (2019), "A versatile, linear complexity algorithm for flow routing in topographies with depressions," *Earth Surf. Dynam.* 7:549–562, doi:10.5194/esurf-7-549-2019 — *citation verified, contents not read*. If the ε-fill residual is ever promoted from "named" to "fixed," that is the line to read.

### 4.1 The Braun–Willett implicit scheme is numerically diffusive, and its answer depends on Δt

**Campforts, B., and Govers, G. (2015). "Keeping the edge: a numerical method that avoids knickpoint smearing when solving the stream power law." *JGR: Earth Surface* 120(7):1189–1205. doi:10.1002/2014JF003376.** And the model paper, which I read from the primary PDF:

**Campforts, B., Schwanghart, W., and Govers, G. (2017). "Accurate simulation of transient landscape evolution by eliminating numerical diffusion: the TTLEM 1.0 model." *Earth Surface Dynamics* 5:47–66. doi:10.5194/esurf-5-47-2017.** (Open access.)

Their §4.1, at `n = 1`, `m = 0.42` — essentially our parameters:

- **The smearing is large and visible.** At 100 m resolution over 1 Myr, *"the first-order implicit FDM suffers from considerable numerical diffusion"* against the analytical characteristics solution (their Fig. 1). The implicit knickzone is smeared and displaced; the TVD-FVM tracks the analytic answer.
- **The implicit solution is timestep-dependent, and they say so plainly.** Verbatim: *"Where the time step of the implicit scheme is unbounded by the CFL criterion, however, the solution deviates from those adhering to the CFL criterion. This illustrates that there is trade-off between numerical accuracy and numerical stability for an implicit scheme at long time steps."*
- **Discrete uplift application inserts artificial shocks.** Verbatim: *"an implicit scheme at high spatial resolution and large time steps fails to converge to an analytical solution because uplift is modeled as a discrete stepwise function rather than a continuous function … that inserts artificial shocks in the solution."* **We apply rock uplift as a discrete per-epoch addition at step 1 of `Fluvial::erode`.** At our epoch size this is the named mechanism.
- **The cost of the accuracy.** Their Fig. 5: RMSE against analytic runs ~5–45 m across resolutions. To match TVD-FVM's accuracy at 500 m, the implicit scheme (CFL < 1) must run at 150 m, taking 12× longer.

The counterweight, so this is not one-sided: **Braun, J., and Deal, E. (2023). "Implicit algorithm for threshold stream power incision model." *JGR: Earth Surface* 128. doi:10.1029/2023JF007140** — the FastScape lineage's own position is that the algorithm *"does not need to satisfy the CFL condition and provides an accurate solution for both small and very long time steps."* Gasparini et al. take this at face value (*"accurate even when the time step is more than 100 times the stable condition"*) and use only the implicit solutions when running beyond stability. So this is a live disagreement in the field about *accuracy* (both sides agree on stability), and Campforts et al. have the analytic-benchmark comparison.

**What this means for us, stated as three separate things because they have different weights:**

1. **It does not threaten the bit-exactness property.** Chaining at *fixed* per-epoch Δt reproduces the one-shot run exactly, because every epoch does the identical arithmetic in the identical order. Stage density stays legitimately out of artifact identity. Nothing here touches that.
2. **It does mean the epoch size is an accuracy parameter, not a free dial** — and it is currently in `ASSUMPTIONS.md` as `arbitrary`. Two arbitrary numbers (count and size) with different consequences: §1.3 gives the count a derivation, and §1.5 gives the size a stability scale, and Campforts et al. give it an accuracy cost.
3. **There is an analytical benchmark available** — the slope-patch method of Royden & Perron (2013), which Campforts et al. use as ground truth (their Eqs. 18–19). A probe that can *fail* against an exact solution is exactly the instrument `#norm-probes-before-claims` asks for, and we do not currently have one for the incision step. This looks to me like the single highest-value probe available to the erosion rung.

### 4.2 Priority-Flood's ε-fill has a documented floating-point failure mode, and our guard is missing

Our `Fluvial::fill_depressions` (`crates/vivarium-world/src/erosion.rs:406–461`) uses `const EPS: f32 = 1e-3`, applied as `h[j] = h[j].max(elev + EPS)` where `elev` is the *already-filled* elevation of the popped cell — so the imposed gradient **accumulates one EPS per cell along the fill path**.

Barnes' own reference implementation documents two hazards here (RichDEM docs, `richdem.readthedocs.io`, depression-filling page; the algorithm paper is **Barnes, R., Lehman, C., and Mulla, D. (2014). "Priority-flood: an optimal depression-filling and watershed-labeling algorithm for digital elevation models." *Computers & Geosciences* 62:117–127. doi:10.1016/j.cageo.2013.04.024**, already in our bibliography as `barnes-2014-priority`):

- **The cumulative-rise hazard, which is live for us.** Verbatim: *"If a depression is too large, the imposed gradient may result in the interior of the depression being raised above the surrounding landscape."* At `EPS = 1e-3 m`, a 10 000-cell flat accumulates 10 m of imposed rise. Our `detail-erosion-composition` residual table already names the ε-fill as a *"sign-definite mass mint in sinks"* — this is the same defect seen from the other side, but the *geometry* consequence (inverting the depression, hence rerouting flow through a manufactured divide) is not in the table and is worse than the mass consequence, because per §1.2 flow rerouting is what destabilises everything downstream.
- **The precision hazard, which is bounded but worth a declaration.** RichDEM does not use a fixed ε at all: *"the value ε is non-constant and must be chosen using the `std::nextafter` function"*, and even so *"it is still not possible to guarantee that the algorithm will behave correctly in situations where a DEM's precision approaches that of its storage data type."* Their mitigation: *"Using `double` instead of `float` reduces the potential for problems at a cost of twice the space used."* We are `f32` with a fixed ε.

  The arithmetic, **measured rather than estimated** (my first pass estimated ≈1.7 × 10⁴ m from the ULP inequality; a parallel reading ran it and the binade structure moves the answer): `h + 1e-3f == h` first becomes true at exactly **h = 32 768 m** — the 2¹⁵ binade boundary, where ULP = 3.9 × 10⁻³ and round-to-nearest kills the increment outright. The increment quantizes to 2 ULP at **8 000 m** and to 1 ULP from **8 192 m**. So the headroom over Everest is a factor of ~3.7, not ~2, and the *degradation* (fill gradient becoming silently elevation-dependent) begins right at the top of Earth-like relief rather than above it. Our heights are metres relative to sea level, so **this is not currently a live bug** — it is an undeclared precondition, at about a factor of two of headroom against a tall world, and it would become a silent no-op (flats with no gradient, hence no D8 orientation, hence a dead router) rather than a loud failure. If heights were ever stored as radius rather than elevation, ε is absorbed completely.

**One thing our implementation does better than the field, worth keeping.** The min-heap is keyed `(elevation, insertion seq)` with `f32::total_cmp` and an integer sequence tie-break (`erosion.rs:423–427`). That is a *total* order — no float-comparison chance, no NaN pathology, no dependence on heap-internal ordering. Most LEM implementations tie-break by whatever the container does. This is precisely the property that makes the per-epoch step a pure function and therefore makes chaining bit-exact, and it is currently an undocumented virtue rather than a stated claim. It should be a stated claim: it is load-bearing for the whole time-interior design.

### 4.3 f32 sets a hard floor under any tolerance we might declare

Combining Gasparini et al.'s *"the time at which a particular metric appears to reach zero will depend on the floating point precision"* with our storage type: at elevations of order 10³ m, one `f32` ULP is ≈ 1.2 × 10⁻⁴ m. **No per-cell Δh tolerance below ~10⁻⁴ m is meaningful in our kernel**, and their published metric curves run to 10⁻⁶–10⁻¹⁴ m — i.e. the entire range over which their metrics resolve the approach to steady state is *below our noise floor by two to ten orders of magnitude*.

This is an independent reason a Δh-tolerance gate cannot work for us, additional to and stronger than the driver-bound argument in `#obs-erosion-residual-is-driver-bound` FE(4) — that argument says the tolerance would fire wrongly; this one says that for tolerances in the range the literature actually uses, we could not evaluate it at all. It strengthens the segment rather than qualifying it. It is also *exactly* the kind of claim a probe can convict: assert that mean |Δh| on a driven tile never falls below ~10⁻⁴ m regardless of epoch count.

### 4.4 Reading `Fluvial::erode` against the literature: one clean result, one hazard

Two things fell out of checking our loop (`erosion.rs:714–753`) against the pitfalls the field names. Neither is literature; both are consequences of it.

**A refinement to how we should claim this, since the survey came back.** Purity alone is *not* rare: fastscapelib's C++ rewrite has it by architecture (no persistent context at all), and OpenLEM is close to it. **What nobody else has is purity *plus* total-order tie-breaking *plus* a test that convicts the chain** — `staged_chain_is_bit_identical_to_one_shot` appears to be the first restart-equivalence test in this field, against a GitHub-wide search that found none. So the claim to make is about the *conviction*, not the purity; claiming the purity as distinctive would be an overclaim the survey can refute.

**The clean result — the per-epoch step really is a pure function of `h`, and it is worth pinning with a probe.** Everything the epoch needs is re-derived inside the epoch: `outlets()`, `fill_depressions`, `receivers`, `elevation_order`, `accumulate_drainage` all run from the current height field before `incise` touches anything. The struct's other `Vec` fields are static geometry (`cell_area`, `centers`) or per-cell static inputs (`uplift_rate`). No RNG is drawn in the loop. The one field that *looks* like carried state is `drainage` — documented as "MFD drainage area from the last epoch" — but it is overwritten by `accumulate_drainage` before `incise` reads it, so it is an output buffer, not state. **That is exactly the property the whole time-interior design rests on, and it is currently true by construction rather than by test.** A cheap probe would convict it: zero `drainage` (and any other non-`h`, non-static field) at the top of each epoch and assert the output is bit-identical. If it is, purity is pinned; if it is not, the design has found its bug early and for free.

For accuracy, one correction to a hope I had while reading: we do **not** escape Braun & Willett's semi-implicit drainage lag (§4.0). We compute `A` from the post-uplift, post-fill height field and then hold it fixed through the incision solve — which is precisely their "A calculated at time t." Their unquantified timestep limit for accuracy applies to us as stated.

**The hazard — `last_delta_m` is the one output that is not invariant under re-chaining.** Line 716: `let track_before = if e + 1 == p.epochs { Some(self.h.clone()) } else { None };`. The residual is recorded **only on the final epoch of a call**. So:

- A one-shot run of 40 epochs produces exactly one `last_delta_m`, measured across the 39→40 boundary.
- A chained run of 4 × 10 epochs produces four, at 9→10, 19→20, 29→30, 39→40.

The *heights* are bit-identical either way — this does not touch the core claim. But `last_delta_m` is a per-**call** observable masquerading as a per-run one, and it is the closest thing the kernel currently has to `#form-time-indexed-stage-chains` FE(3)'s "each stage records the ε it reached." Two consequences, pointing opposite ways:

1. **Cut against — and now closed.** The question was what the equivalence test actually compares. Checked: `query.rs:1276–1279`, `staged_chain_is_bit_identical_to_one_shot` compares **only the height-field bytes** (`one_shot.iter().zip(staged).all(|(a, b)| a.to_bits() == b.to_bits())`), then separately asserts that interior stages exist with finite residuals. So `last_delta_m`'s per-call semantics cannot trip it, and the concern is not live. Worth keeping the *shape* in mind — a future test that widened to whole-struct comparison would start failing for a bookkeeping reason, and the failure would look like a physics bug.
2. **Cut for, and I think this is the better framing:** chaining is what *gives* us a per-stage ε. FE(3) says an ε that is never recorded is an unLawfulness budget asserted to be zero, and the one-shot build is exactly that assertion for every interior epoch. So the time interior does not merely make stages addressable — **it is the mechanism by which FE(3) becomes satisfiable at all for this kernel.** That seems to me a stronger motivation for the work than watchability, and it is not currently how the segment argues for it.

---

## 4A. Crosswalk to `#form-time-indexed-stage-chains`

Reading the segment after the literature, four of its Formal Expression rows have something waiting for them. Offered as candidate segment work, not as findings the segment already owns.

- **FE(4) — "the criterion is not always a residual tolerance, and assuming it is has now failed twice."** The literature supplies the third form the clause has been waiting for, and it is neither a count nor a tolerance: an **a-priori analytical response time** computed from static inputs (§1.3), optionally paired with a **shape test** on the converged state (§1.4). Both are convictable in the way FE(4) demands — `T_A` because a run that has not converged by `T_A` falsifies it, and χ-linearity because the fit residual *and* the fitted slope must both pass. This looks to me like the row's cure, and it arrives without having to relocate any arbitrary number into the declaration layer, which the row explicitly names as the failure mode to avoid.
- **FE(7) — the retention law, currently resting on Gear–Wells "one prior value per component."** Gear–Wells answers retention for *coupled multirate integration*. Revolve (§3.2) answers the different question of retention for **arbitrary interior recall under a storage budget**, provably optimally, and its answer contradicts the intuitive one: optimal checkpoint placement is **binomial, not uniform**. That does not displace FE(7) — the two solve different problems — but a materialized-only chain (FE(8)) whose stages a viewer will scrub is Revolve's problem shape, not Gear–Wells's, and FE(7) currently reads as though one retention law covers both.
- **FE(8) — law-evaluable vs materialized-only.** §1.3 sharpens the boundary in a way that matters. At `n = 1` the *response time* is closed-form even though the *state* is not: `T_A = β/K` is computable without integrating. So erosion is materialized-only in state while being law-evaluable in **duration**. That is exactly what is needed to answer "how many stages does this chain have?" before building any of them — which is a build-planning fact the current binary does not have a place for.
- **FE(8), the other half — a law-evaluable SPL formulation may actually exist.** §1.3 showed erosion is law-evaluable in *duration* while materialized-only in *state*. There may be more available than that: **Tzathas, P., Cordonnier, G., et al. (2024), *Computer Graphics Forum* 43** reports **analytical** solutions to the stream power law — a law-evaluable formulation of what we treat as a materialized-only chain, presumably under fixed-network assumptions. *Paywalled; neither I nor the surveyors verified the scope of those assumptions,* and the fixed-network caveat is likely fatal given §1.2. But FE(8) currently reads as though the materialized-only classification is a settled property of the kernel rather than of *this* formulation of it, and that is worth one hedge. (Same lineage, and relevant to explore-side work: Cordonnier et al. 2017 *ACM TOG* 36(4); Schott et al. 2023 *ACM TOG* 42(4), uplift-domain authoring — we already hold `schott-2023-large`; Yang et al. 2024 "Unerosion," *CGF* 43, backward-time simulation, which is the pathological inverse of a time interior.)
- **`#form-complete-content-addressed-key` — `SRC_HASH` covers source, not toolchain.** Reading `source_hash.rs` against the survey: the digest does not cover rustc/LLVM version or target features, and **FMA contraction and autovectorization can reorder f32 accumulation** — which is exactly the MFD drainage-area summation flagged in §4.0. The documented real-world instance of this defect class is OpenLEM v45's changelog, where reordering the 8 neighbours "should have an effect only if multiple neighbors have the same elevations" — an author recording a semantically-neutral refactor that silently changes results on ties (and note it is a *tie-breaking* instance, per §2.4's risk reranking). Whether toolchain identity belongs in the key is a real decision either way; the thing to avoid is its being neither in the key nor in the record.
- **FE(9) — densification by bisection, and "a stage that moves by one ULP is not the same stage."** §4.2's determinism finding is the mechanical guarantee underneath this: the Priority-Flood heap's `total_cmp` + integer-sequence tie-break is what makes the per-epoch step a pure function, hence what makes a stage's identity stable to the ULP. It is currently an implementation detail with a one-line comment. Given how much of FE(9) and the whole time-interior design rests on it, it reads to me like a claim the segment should state and a probe should convict — the more so because §2.1's survey found no other LEM that guarantees this, and one (`WRF`, in a neighbouring field) publicly does not.

---

### 2.5 The FastScape family, CHILD, Badlands, HyLands, OpenLEM

A parallel survey covered the rest of the ecosystem from source, running several of them. The verdict generalises: **no LEM guarantees or tests restart-equals-continuous.** A GitHub-wide code search for `test_restart_equivalence` in the LEM orbit returns nothing. The property *is* routinely CI-gated one field over — NOAA's `fv3gfs-fortran` has `test_restart_reproducibility` asserting MD5-identical restart files — so the practice exists and geomorphology simply never adopted it. Every system that attempted restart got it wrong instructively:

- **FastScape (Braun's Fortran).** No restart facility at all. The stateful context holds `h`, basement `b`, cumulative erosion `etot`, `step`, and Strati arrays — but there is **no `Set_Step` and no setter for `etot`** (verified absence by grep), and Strati state has no accessors, so a caller cannot reconstruct the context even in principle. Worse: `SetUp()` initialises `h` and the catchment-label field with **unseeded** `call random_number(...)` (`FastScape_ctx.f90:83,95`), verified to differ per process — two raw-API runs of the same experiment do not agree with *each other*. Output is VTK at `sngl()`, f32-truncated, restart-impossible.
- **fastscapelib (the modern C++ rewrite) converged on our architecture.** `spl_eroder` holds only parameters and a scratch buffer; `erode(elevation, drainage_area, dt)` is a pure per-call function; there is no persistent context. "State is the height field the caller holds" — the same purity that makes our chaining exact (§4.4). Worth borrowing: the eroder *counts* how often the implicit update tried to cut below the flooded-sill elevation and clamped (`n_corr`, `spl.hpp:210`) — a cheap always-on instrument for "how often did the solver hit its guard."
- **FastScape's sink resolvers document the fill-vs-route fork, and both avoid our artifact.** `pflood_sink_resolver` uses **`nextafter`, not a fixed ε** (§4.2), and `mst_sink_resolver` implements Cordonnier 2019 basin-graph routing. Critically, both work on an internal *hydrologically corrected copy* (`flow_graph.hpp:403–408`) — **the true elevation is never mutated.** Cordonnier et al. state their algorithm "leaves the elevation of the nodes unchanged." Our fill writes into `self.h` directly. That is a structural difference, not just a parameter choice, and it is the cleaner answer to the ε-fill mass-mint residual. *(Verified absence worth noting: Cordonnier 2019 contains no discussion of tie-breaking, determinism, or floating-point ε at all.)*
- **CHILD is the most principled restart of the cohort and still not bit-identical.** Its RNG handling is exactly right — the full 57-integer `ran3` state is serialized (`Mathutil/mathutil.cpp:59–72`), the pattern to copy if we ever gain stateful stochasticity. But all output streams open with `precision(12)` (`tOutput.cpp:181`) where doubles need 17: measured, **~100 % of elevations fail to round-trip**, worst error 5 × 10⁻⁹ m. Every CHILD restart perturbs every node. And restart granularity is *collapsed onto* output granularity — you can only resume from a written slice, the coupling our design deliberately avoids.
- **Badlands** reloads interior nodes bit-exactly but **re-derives the boundary ring rather than restoring it** — 168/328 border nodes wrong by up to 19.6 m, propagating 0.29 m inward within one output interval. Also two live bugs: `cumfail` aliased to `cumhill` on restart, and the final display interval is never simulated (a run reporting 100 kyr delivers 75).
- **HyLands/BedrockLandslider**: the RNG stream is the *only* hidden state; carrying `np.random.get_state()` across manually gives **bit-identical, 0/400 nodes**. Draw count is fixed per step, so fast-forwarding is possible; nothing does it.
- **gospl** (Salles' PETSc successor, the live parallel answer) is the only LEM with a deliberate restart-state enumeration and a stated rule — *"float64 if used in restart… if purely visualisation, do NOT add a read block."* **That is our identity/view store wall, independently arrived at.** Its regression suite explicitly asserts *statistical*, not bitwise, equivalence across processor counts, because "PETSc parallel reductions… are NOT guaranteed bitwise-identical," with measured platform drift of 0.3–1.7 %. Directly relevant to §4.0's parallelism note: the field's one parallel LEM gave up bitwise reproducibility as the price.
- **OpenLEM** has a real but unadvertised checkpoint, and a **three-tier key system** (v45 source, lines 754–817): `H`/`H8` full-double identity-grade, `H4` lossy f32 view for GIS export, `S` derived and structurally unreadable. Identity / view / derived-not-state — our store split again, arrived at independently by the field's most numerically careful author. The canonical example writes `H` to the restart file and `H4` to the view file.

**Hergarten wrote our §4.0 concern down, bluntly, about his own runs.** Hergarten 2020 (*ESurf* 8:841–854), §5: implicit schemes' *"numerical error increases linearly with δt… a systematic error… always too slow"* — and the sharper mechanism, *"In many situations, the limitation of the maximum δt arising from changes in the flow directions is more severe than the numerical error of the implicit scheme itself."* That is the drainage-lag term from Braun & Willett §4 (§4.0), named as the *dominant* one. Hergarten 2022 (*ESurf* 10:671–686, §2 p. 675) uses δt = 2⁻¹⁰ *"although the results are by far not independent of δt then"*, quantified: an 8× smaller δt changes mean avulsion rate by 1.6×, up to ~3× for large rivers. He also reports the **detachment-limited (Braun–Willett) scheme has the larger δt-dependence of his two solvers** — which is the one we run.

**And an unclaimed cadence signal sitting in his code:** OpenLEM's `computeFlowDirection()` returns the count of receiver changes per step, and Hergarten 2020 explicitly names bounding that count as *"a feasible criterion"* for δt. Nobody has used it for **output** cadence. A change-driven stage ladder indexed by drainage-reorganization count is right there and unclaimed — and it is the quantity §1.2 and §4.0 independently identify as the thing that actually controls transient behaviour.

**Two independent probes confirm the core design property.** Two agents, different code, ran the same experiment on 1-D Braun–Willett `n = 1`: straight-through versus chunked at arbitrary boundaries → **bit-identical in both f32 and f64**; the same total time with the middle half at 2× Δt → max diff ≈ 0.95 m, ≈ 0.16 % of relief. Equal-Δt chaining is exact; unequal-Δt chaining is not. That is §4.1(1) and §3.0's converse invariant, measured twice on code that is not ours.

**Citation corrections from that survey** (each was a plausible-sounding reference that does not exist): there is **no Badlands JOSS paper** — the set is Salles 2016 (*SoftwareX*, 10.1016/j.softx.2016.08.005), Salles & Hardiman 2016 (*C&G*), Salles, Ding & Brocard 2018 (*PLoS ONE* 13(4):e0195557). There is **no OpenLEM model-description paper** (software plus whichever numerics paper applies), and **no 2020 paper titled "Transient evolution of fluvial landscapes."**

---

## 5. What I did not verify, and what I would chase next

Flagged so nobody cites these on my word:

- **Braun & Willett 2013 Appendix B is now verified** (§4.0) — read from `ref/geology/pdfs/braun-2013-very.pdf`, Eqs. B.1–B.5 plus the Δt ≤ 1000 y transient-convergence statement. This was the one item flagged unverified by every parallel reading; it is settled, and it moved the finding from "the transient is excluded" to the more useful "the transient is bounded, and here is the bound."
- **My first-pass ε-absorption threshold was wrong** and is corrected in §4.2 from measurement (32 768 m, not ≈1.7 × 10⁴ m). **My first-pass attribution of Badlands' 15.6 m cadence effect was also wrong** and is corrected in §3.0 (that figure is dominated by a separate never-simulated-final-interval bug; the isolated cadence effect is 0.31 m). Both were mine, both are fixed, and both are the kind of number worth re-deriving rather than inheriting from this document.
- **Whipple (2001)** — `T_A = β/K` reached me through Gasparini et al. Eq. (9), not the primary. Chase before a segment cites it.
- **Royden & Perron (2013), JGR-ES 118:497–518** — paywalled (HTTP 402). Citation and abstract verified; contents not read. This is the one that would confirm the exact-solution structure at `n = 1` and the slope-patch method Campforts et al. use as ground truth. An author copy likely exists via MIT DSpace `1721.1/85608`; the bitstream refused automated fetch.
- **Liu et al. (2015)** — I read the GMDD discussion paper; the final GMD version may differ.
- The **Revolve successors** (arXiv:2106.13879, arXiv:2305.09568) — existence and topic verified only.
- The **WRF restart thread** is a forum post, not a result.

Two things I would chase next if this continues:

1. **The exact-solution probe.** Royden & Perron's slope-patch method as a `#norm-probes-before-claims` instrument against `Fluvial::erode`'s incision step. It is the only thing in this whole survey that could *convict* our kernel against a known-correct answer rather than against itself. §4.1(3).
2. **The Courant number for our own epoch — now from a closed form, so this is arithmetic.** Braun & Willett Eq. (B.5) (§4.0) plus §1.5, from quantities we already compute. It is the cheapest number in this whole document and it is the one §4.0 says governs whether our interior stages mean anything. I would compute it before committing the design, not after — not because it should block anything, but because if it comes back at 10³× the stable scale, the honest framing of the time interior changes from "the world's history" to "a sequence of addressable states whose intermediate accuracy is undeclared," and that is a sentence better written now than retrofitted.
3. **The χ-linearity criterion as an actual segment.** §1.4 is a proposal, and the honest next move is a probe on the one eroding footprint (f2) from `examples/erosion_settle_probe`: compute χ, fit, and see whether the fit residual falls monotonically while mean |Δh| sits pinned at the uplift rate. If it does, that single figure retires the open item in `#obs-erosion-residual-is-driver-bound` FE(6) — and it is a figure that shows a criterion working where the obvious one provably cannot, which is the kind of result the segment already knows how to hold.

---

## 6. Citations, with identity

Verified from primary text (read the PDF or the publisher page):

| key | citation |
|---|---|
| Gasparini 2024 | Gasparini, N. M., Forte, A. M., Barnhart, K. R. (2024). Short communication: Numerically simulated time to steady state is not a reliable measure of landscape response time. *Earth Surf. Dynam.* 12:1227–1242. doi:10.5194/esurf-12-1227-2024 |
| Whipple & Tucker 1999 | Whipple, K. X., Tucker, G. E. (1999). Dynamics of the stream-power river incision model… *JGR Solid Earth* 104(B8):17661–17674. doi:10.1029/1999JB900120 — *already `whipple-1999-dynamics`* |
| Perron & Royden 2013 | Perron, J. T., Royden, L. (2013). An integral approach to bedrock river profile analysis. *ESPL* 38(6):570–576. doi:10.1002/esp.3302 |
| Campforts 2017 | Campforts, B., Schwanghart, W., Govers, G. (2017). Accurate simulation of transient landscape evolution by eliminating numerical diffusion: the TTLEM 1.0 model. *Earth Surf. Dynam.* 5:47–66. doi:10.5194/esurf-5-47-2017 |
| Liu 2015 | Liu, L., et al. (2015). Importance of bitwise identical reproducibility in earth system modeling and status report. *Geosci. Model Dev. Discuss.* 8:4375–4400. doi:10.5194/gmdd-8-4375-2015 |
| RichDEM docs | Barnes, R. RichDEM documentation, depression-filling page. `richdem.readthedocs.io` |
| Braun & Willett 2013 | Braun, J., Willett, S. D. (2013). A very efficient O(n), implicit and parallel method to solve the stream power equation… *Geomorphology* 180–181:170–179. doi:10.1016/j.geomorph.2012.10.008 — *already `braun-2013-very`; local PDF `ref/geology/pdfs/`. §4.0 read from primary* |

Citation and abstract verified, contents not read:

| key | citation |
|---|---|
| Royden & Perron 2013 | Royden, L., Perron, J. T. (2013). Solutions of the stream power equation… *JGR Earth Surface* 118(2):497–518. doi:10.1002/jgrf.20031 |
| Whipple 2001 | Whipple, K. X. (2001). Fluvial landscape response time: how plausible is steady-state denudation? *Am. J. Sci.* 301(4–5):313–325. doi:10.2475/ajs.301.4-5.313 |
| Campforts & Govers 2015 | Campforts, B., Govers, G. (2015). Keeping the edge… *JGR Earth Surface* 120(7):1189–1205. doi:10.1002/2014JF003376 |
| Braun & Deal 2023 | Braun, J., Deal, E. (2023). Implicit algorithm for threshold stream power incision model. *JGR Earth Surface* 128. doi:10.1029/2023JF007140 |
| Griewank & Walther 2000 | Griewank, A., Walther, A. (2000). Algorithm 799: revolve: an implementation of checkpointing for the reverse or adjoint mode of computational differentiation. *ACM TOMS* 26(1):19–45. doi:10.1145/347837.347846 |
| Willett & Brandon 2002 | Willett, S. D., Brandon, M. T. (2002). On steady states in mountain belts. *Geology* 30(2):175–178 — the four-steady-states taxonomy (flux / topographic / thermal / exhumational); useful vocabulary, not a criterion |
| Hack 1957 | Hack, J. T. (1957). Studies of longitudinal stream profiles in Virginia and Maryland. *USGS Prof. Paper* 294-B:45–97 — the `A = k_a x^h` law Eq. (10) needs |
| Cordonnier 2019 | Cordonnier, G., Bovy, B., Braun, J. (2019). A versatile, linear complexity algorithm for flow routing in topographies with depressions. *Earth Surf. Dynam.* 7(2):549–562. doi:10.5194/esurf-7-549-2019 — the depression-routing successor to ε-filling; code at `github.com/fastscape-lem/flow-routing-depressions` |
| Barnhart 2019 | Barnhart, K. R., et al. (2019). terrainbento 1.0: a modular landscape evolution model. *Geosci. Model Dev.* 12:1267–1297 — §5.6 p. 1285 checked against the PDF; see §2.3(1) |
| SimFS 2019 | Di Girolamo, S., Schmid, P., Schulthess, T., Hoefler, T. (2019). SimFS: A Simulation Data Virtualizing File System Interface. arXiv:1902.03154 — sparse checkpoints + on-demand re-simulation; closest prior art to the whole design |
| Hergarten 2020 | Hergarten, S. (2020). Transport-limited fluvial erosion — simple formulation and efficient numerical treatment. *Earth Surf. Dynam.* 8:841–854 — §5 quantifies implicit-scheme δt error as linear and *systematic*, and names flow-direction staleness as usually the larger term |
| Hergarten 2022 | Hergarten, S. (2022). *Earth Surf. Dynam.* 10:671–686 — §2 p. 675, δt = 2⁻¹⁰ *"although the results are by far not independent of δt then"* |
| Morris & Roberts 2025 | Morris, C., Roberts, G. (2025). Impact of noise on landscapes and metrics generated with stream power models. *Earth Surf. Dynam.* 13:1003–1038 |
| Campforts 2020 | Campforts, B., Shobe, C. M., Steer, P., Vanmaercke, M., Lague, D., Braun, J. (2020). HyLands 1.0. *Geosci. Model Dev.* 13:3863–3886 — Landlab component `BedrockLandslider` |
| Salles 2016 / 2018 | Salles, T. (2016). *SoftwareX* doi:10.1016/j.softx.2016.08.005; Salles, Ding & Brocard (2018). *PLoS ONE* 13(4):e0195557 — **Badlands has no JOSS paper** |
| Liebl 2023 | Liebl, M., et al. (2023). *Geosci. Model Dev.* 16:1315–1343 — benchmarks OpenLEM v37; **there is no OpenLEM model-description paper** |
| Toueg & Babaoğlu 1984 | Toueg, S., Babaoğlu, Ö. (1984). *SIAM J. Comput.* 13(3):630 — O(n³) DP for optimal checkpoint placement on a chain; our exact topology |
| Bhattacherjee 2015 | Bhattacherjee, S., et al. (2015). *PVLDB* 8(12):1346 — dataset versioning; our chain shape is their polynomial-time case |
| Kawakami 2020 | Kawakami, et al. (2020). ISAV'20, doi:10.1145/3426462.3426469 — DP for optimal K-snapshot selection; **entropy-triggered lost to evenly-spaced in 1 of 2 benchmarks** |
| Arteaga 2014 | Arteaga, A., Fuhrer, O., Hoefler, T. (2014). IPDPS — cheap bit-reproducibility; the citation SimFS leans on for its precondition |
| Tzathas 2024 | Tzathas, P., Cordonnier, G., et al. (2024). *Computer Graphics Forum* 43 — analytical SPL solutions; **paywalled, assumptions unverified** |
| Barnes 2021 | Barnes, R., Callaghan, K. L., Wickert, A. D. (2021). Computing water flow through complex landscapes – Part 3: Fill–Spill–Merge: flow routing in depression hierarchies. *Earth Surf. Dynam.* 9:105–121 — Barnes' own successor to Priority-Flood; the modern answer to the ε-fill artifact |

Already in `ref/research/BIBLIOGRAPHY.md` and relevant here: `braun-2013-very`, `yuan-2019-new`, `barnes-2014-priority`, `lague-2014-stream`, `davy-2009-fluvial`.

*Relata registration deferred per the standing note in `BIBLIOGRAPHY.md` (relata under construction as of 2026-07-10).*
