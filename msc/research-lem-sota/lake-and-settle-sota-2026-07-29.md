# Lakes and the settle: what the field has actually landed on, and which of our beliefs are stale

*Literature dossier, 2026-07-29. Written against the "put a lake on screen that is honestly a lake" decision. **Not claim canon** — nothing here is a vivarium claim until a segment owns it. Every citation carries enough identity to verify; where I did not open the source I say so in the line itself.*

**Reading order if you have five minutes:** §0 (four of our beliefs are stale, one of them by hours) then §2.2 (the one theoretical result that decides the settle question) then §1.2 (the algorithm that is strictly better than what we have, and why).

**Bottom line.** The equilibrium-lake path is right, it is what the field does, and our specific construction is not a worse cousin — it independently matches a 2024 published algorithm. But the case for it is *stronger and differently shaped* than the brief's, two of the brief's blockers are no longer blockers, and the thing most likely to stop a lake appearing on screen tonight is neither the fill nor the settle: it is the outlet set.

---

## §0 Stale beliefs, led with

### 0.1 The friction blocker is stale by hours — it was fixed the same day, in the same document the brief cites

The brief's item 2 rests on `msc/agent-briefs/2026-07-29-water-council-probes.md` §1.2: the friction denominator uses the pre-friction velocity, so the steady state is a function of Δt (shipped dt ~8% slow, coarser dt degrading to −32%).

That is §1.2. **§6.1 of the same file reports the fix landed**, at kernel era `77b1f5a`, and I verified it in the source rather than taking the report's word for it. `crates/vivarium-world/src/water.rs` now carries the implicit form — the comment at line 577 reads *"Manning friction, IMPLICIT IN THE UPDATED FLUX"*, line 590 states the algebra *"`k ∝ dt`, leaving `f = l·h^{5/3}·√S/n` — Manning, dt-free"*, and line 599 is the closed-form solve of the quadratic. The measured table (§6.1) is flat at Fr = 2.0000 from dt 0.4 s down, where before it read 1.6638 at dt 0.4 s.

So *"raising the step is unsafe because the steady state is a function of Δt"* is no longer true. The residual at dt 0.8 s is attributed — by measurement, not arithmetic — to the **outflow clamp** binding (a cell ships `f·dt` against a capacity of `d·area`), which is a genuine and legitimate volume constraint, and a *different* constraint from both wave CFL and friction accuracy. If the step-size question gets re-opened, that clamp is the thing to reason about, and it has the pleasant property of being a real physical statement (you cannot ship more water than is present) rather than a calibration.

I flag this as the brief's most consequential staleness because it was load-bearing for "marching longer is not obviously a path at all," and half of that argument has been withdrawn by its own author.

### 0.2 The θ half is *not* stale — and it is better supported than a local measurement, because the primaries already say it

The brief's other half — without θ the scheme grows at every Froude number tested including 0.17, so θ holds up an otherwise-unstable discretisation rather than damping physics — stands, and §6.6 of the probe file re-ran it post-fix with all sixteen configurations still growing.

What the brief does not know is that **this is a stated property of the scheme in its own primary, not a discovery about our port.** From de Almeida & Bates (2013), §3, reading the local PDF at `ref/hydrology/pdfs/almeida-2013-applicability.pdf` (verbatim):

> "the parameter θ is a weighting factor that adjusts the amount of artificial numerical diffusion […] no artificial diffusion is added when θ = 1, while increasing amounts of numerical diffusion are added as θ is reduced below unity. In general, artificial diffusion is needed to counteract the formation of discontinuities in the solution that typically arise as a result of the nonlinearity in the system (i.e., shocks). **In practical problems, this becomes particularly necessary for low friction surfaces** or problems involving fast changes of flow, in which case **θ values between 0.7 and unity have been found to perform well.**"

Read that against the probe's crux control. The configurations that grow hardest are `n = 0.04` — the low-friction column. The shipped kernel runs θ = 0.8, inside the recommended band. The measurement is therefore a clean *reproduction* of a documented requirement of the scheme, on the exact axis (friction) the authors name, rather than evidence of a defect we introduced.

This is an upgrade, not a downgrade: `#obs`-grade language can say the scheme's need for θ at low friction is a property its authors declare and we have independently measured, which is a far stronger footing than "we found θ is load-bearing and don't know why."

### 0.3 The "Caspian gap" is not the wet-limit caveat — and it is upstream of every lake algorithm

The brief describes the Caspian gap as *"an endorheic basin that stands below its sill"* — i.e. as the volume-limited case that sill-limited fill cannot express. That is a real limitation of sill-limited fill (§1.2 below), but **it is not what the project's Caspian gap note says**, and the difference decides tonight's ordering.

`core/src/form-derived-sea-level.md` FE and `DECISIONS[the-eyes-first-verdict-basins-legitimate-circles-fading-and-a-caspian-gap]` (Joseph, 2026-07-28) record the actual mechanism: **sea is classified by elevation threshold, not by connectivity.** I read the code to check what that costs the fill, and it is worse than a census artefact:

```rust
// erosion.rs:718  fn outlets(&self)
out[i] = edge_sink || self.h[i] <= sea;
```

and then `erode` calls `self.fill_depressions(&outlets)`, which seeds its min-heap from exactly those cells and marks each `closed[i] = true` with `water[i]` never assigned.

So **any basin whose floor dips below the derived datum is seeded as an ocean outlet, and is therefore structurally incapable of holding standing water** — not because sill-limited fill is the wet limit, but because the fill has been told that basin *is* the sea. This is not fixed by Fill–Spill–Merge, by a volume-limited fill, by evaporation, or by a settle. It is fixed by classifying the ocean by connectivity.

The probe file already measured the consequence from another direction without naming it as this: §1.3a reports that on the canonical footprint, *"at ×1, 9216/9216 cells are at or below sea"* — the entire tile classified as ocean. And `obs-lakes-are-routed-over-not-carved-away` FE(11) reads 8 tiles holding 23 depression cells across 384 L9 tiles, and attributes the smallness to edge sinks and the perimeter grade. A third cause is available and unmeasured.

The good news is that the repair is the field's own framing rather than an invention. In Barnes, Callaghan & Wickert (2021) the ocean is *"a designated sink region or the map edge"* — a label, and the depression hierarchy is a forest of trees **rooted at that label**. Priority-Flood seeded only from genuine edge/designated-ocean cells computes the connectivity for free as it floods: a landlocked below-datum basin is then simply a depression whose `spill` is its own rim, and the existing `spill` field reports it correctly with no algorithmic change at all. The one-line `|| self.h[i] <= sea` is doing the damage; the traversal that would replace it is already written.

**If I am right about this, it is the cheapest thing on tonight's list and it gates the visible outcome.** It is also the one claim in this dossier I would most want a probe to convict before acting on, because I inferred the "cannot hold water" consequence from reading `fill_depressions` rather than measuring it. The falsifier is cheap: construct a walled tile with a crater whose floor sits below the derived datum, and assert `fill_depth` is non-zero somewhere in it.

### 0.4 One question in the brief is already answered in this directory, and I did not redo it

The brief asks me to be familiar with prior work in `msc/research-lem-sota/` and to say plainly if it already answers something. It does, for the ε:

`lem-time-interior-prior-art-2026-07-28.md` §4.2 already establishes, against RichDEM's own documentation and Barnes et al. (2014), the cumulative-rise hazard (*"If a depression is too large, the imposed gradient may result in the interior of the depression being raised above the surrounding landscape"*), the non-constant-ε recommendation (`std::nextafter`), and the measured f32 binade arithmetic (increment quantises to 2 ULP at 8 000 m, dies outright at 32 768 m). It also names our total-order heap key as a virtue the field mostly lacks. I have nothing to add to any of that and did not re-derive it.

What that section does **not** contain is the alternative that removes ε rather than bounding it — see §1.4. That is the part worth reading as new.

Also worth reporting as a finding about the brief: the two loose GMD 2026 PDFs at `ref/` top level are **not relevant**. `gmd-19-5343-2026.pdf` is Duretz et al., automatic tuning of pseudo-transient solvers for heterogeneous-media deformation (geodynamics); `gmd-19-5601-2026.pdf` is Kern et al., parameter estimation in coupled marine ecosystem models. Neither touches lakes or surface hydrology. I opened both first pages to check.

Conversely `ref/hydrology/NOTES.md` **is** a direct hit and nobody pointed me at it — see §3.1.

---

## §1 The lake half

### 1.1 Our construction is current practice, and it independently matches a 2024 published algorithm

The brief worries about reinventing a worse cousin. It is not a worse cousin. `fill_depressions`'s `spill = bed[j].max(spill)` running maximum, carried beside the heap key and never ordering it, is the same construction as the water-surface step in **FastFlow** (Jain, Kerbl, Gain, Finley & Cordonnier, *Computer Graphics Forum* 43(7), 2024, doi:10.1111/cgf.15243; read from the HAL deposit, hal-04684270). Their §"River and lake modeling", verbatim:

> "We call the *water surface* the elevations obtained after filling the identified depressions. This surface should obey two conditions: it should be as close as possible to the terrain, and at the same time be monotonically non-decreasing along all stream trees, which we intuitively expect since water flows downhill. To achieve this […] we follow the water path of each stream tree upwards from destination to sources, retaining as the water surface the maximum elevation previously reached. For completeness, we optionally allow a small slope ϵ on the water surface as this is sometimes required in hydrology applications."

That is our field, our traversal (theirs runs sources-ward from the destination; ours runs outward from the outlets — the same running maximum over the same paths), and even our optional ε, described as optional in the same breath. Two independent implementations converging on the same object is the strongest kind of evidence that the object is the right one. It is also, per §1.4, a specific reason to make the ε genuinely optional in our code rather than always-on.

One caveat from their side that bears on us: they note this variant *"is only possible with the depression carving strategy, as it requires a continuous path."* Our fill is a filling strategy, and the running max still works because we track original heights separately from the mutated routing surface — which is arguably cleaner than the constraint they accepted.

The published wet-limit ceiling is therefore where we already are. Nothing to adopt here; the finding is that the segment can say so with a citation instead of hedging.

### 1.2 What is strictly better: Fill–Spill–Merge is the volume-limited generalisation, and it is the same data structure

**Barnes, R., Callaghan, K. L., & Wickert, A. D. (2021). "Computing water flow through complex landscapes – Part 3: Fill–Spill–Merge: flow routing in depression hierarchies." *Earth Surface Dynamics* 9, 105–121. doi:10.5194/esurf-9-105-2021.** Open access; read in full.

This is the answer to "is there a well-established algorithm strictly better than Priority-Flood plus the spill field." Yes, and it is not a replacement — it is a superset that keeps everything we have.

The move is to route a *finite volume* of water rather than assume net supply is positive everywhere. Water is moved downhill to pit cells (an O(N) flow accumulation), then redistributed **entirely within a depression hierarchy, with no reference to the DEM at all**, by the title's three operations: a depression fills, spills over its sill into its sibling, and when both are full they merge and begin filling their parent meta-depression. Excess reaching the root is dropped to the ocean. Only then is the water translated back to a per-cell extent and depth.

The partially-filled case — precisely the endorheic lake standing below its sill that sill-limited fill cannot express — is closed-form, via what they name the **lake-level equation** (their Eq. 6, for cells of area $a_i$ and elevation $z_i$):

$$z_w = \left(\sum_{i=1}^{N} a_i\right)^{-1}\left(V_w + \sum_{i=1}^{N} z_i a_i\right)$$

reducing for unit areas to $z_w = \frac{1}{N}(V_w + \sum z_i)$. Cells are admitted to the lake in elevation order from a priority queue, with running sums, until the accommodated volume reaches $V_w$; the LLE then sets the level exactly. Our variable cell areas on the cube-sphere are already in the general form.

Verified performance and complexity, from their §4:

| claim | value |
|---|---|
| worst-case complexity | **O(N log N)** (O(N) achievable for the flooding queue with specialised structures; they use the stdlib O(N log N) queue deliberately, "to reduce the potential for bugs") |
| the iterative alternative (Jacobi / FlowFill) | **O(N²)**, derived in their §4.1 |
| measured speed-up vs FlowFill | **86–2645×** wall time; **2064–63 480×** compute time (FlowFill on 24 cores, FSM on 1) |
| N & S America, 3.2 × 10⁸ cells | 53.2 s FSM, 231.6 s including hierarchy construction |
| Minnesota 30 m topobathy, 8.2 × 10⁸ cells | 307.8 s FSM, 792.6 s total |
| wall time dependence | independent of runoff depth and of landscape ruggedness; scales with domain size only |
| code | 1003 lines C++17, 97 % test coverage, 214 990 test assertions, Zenodo doi:10.5281/zenodo.3755142 |

Two properties matter specifically for us. First, **wall time is independent of how much water you pour in** — the failure mode our current settle has (fixed step count buys a fixed and level-independent 40 s) has no analogue. Second, they explicitly anticipate our per-epoch loop: *"If standing water is retained between invocations of Fill–Spill–Merge and new water added at each invocation, the algorithm can be used to simulate the movement of water across landscapes; we will explore this further in future work."*

The prerequisite is the depression hierarchy, its own paper: **Barnes, Callaghan & Wickert (2020), "Computing water flow through complex landscapes – Part 2: Finding hierarchies in depressions and morphological segmentations," *Earth Surf. Dynam.* 8, 431–445, doi:10.5194/esurf-8-431-2020** (identity read from the FSM reference list; **I did not open Part 2 itself** — flagged). Part 1 is FlowFill: **Callaghan & Wickert (2019), *Earth Surf. Dynam.* 7, 737–753, doi:10.5194/esurf-7-737-2019** (same provenance, also not opened).

The honest cost statement: the hierarchy is a real data structure and building it is most of the work. Their own timings show hierarchy construction dominating FSM on smaller domains (US Great Basin: 0.2 s FSM against 8.7 s total). For a 64² tile that is noise; for the whole-face L9 carve it is the thing to measure before committing.

### 1.3 Lakes coupled to erosion in an evolving landscape: CHONK is the reference, and it answers three questions our segment marks as decisions

FSM is a snapshot algorithm on fixed topography. The brief also asks about lake dynamics *coupled* to erosion, and there the reference is:

**Gailleton, B., Malatesta, L. C., Cordonnier, G., & Braun, J. (2024). "CHONK 1.0: landscape evolution framework: cellular automata meets graph theory." *Geoscientific Model Development* 17, 71–90. doi:10.5194/gmd-17-71-2024.** Open access; read the algorithm and lake sections in full.

Its abstract names our exact problem, unprompted — *"when a lake breaks the upstream–downstream continuum in the amount of sediment and water it receives and transmits."* That is `obs-lakes-are-routed-over-not-carved-away` FE(4) stated by someone else.

It offers **two** lake solvers, and the choice is the same choice we are facing:

- **Passive lakes** (§3.3.1), adapted from Cordonnier et al. (2019): connect each pit to an outlet that eventually reaches the edge, preserving the original topography, bypassing the depression geometry. Fast and stable. Their own stated limitation is exactly ours: *"It also maintains unconditional connectivity between local minima and their outlets, **ignoring endorheism**."* This is what our current composition effectively is.
- **Depression-aware** (§3.3.2): a binary tree per depression system, adapted from Barnes et al. (2020), which *"does not assume the lakes outflow and treats them as separate domains."*

Their candour on the trade is worth quoting, because it is the sort of thing a brief usually has to guess at: *"We are not presenting a version with better computing speed or accuracy compared to the work of Barnes et al. (2020) and Barnes et al. (2021). We adapt its use to our prototype […] A cleaner, performance-oriented solution could benefit from being entirely based on Barnes et al. (2020) and Barnes et al. (2021)."* Their divergence is driven by needing multiple-flow-direction compatibility, which the Barnes line is single-flow-oriented — **and we run MFD with `p = 1`, so this constraint is ours too.** That is the one genuine obstacle to lifting FSM wholesale, and CHONK is the record of what it costs to work around it.

Three of our FE(4) "physics decisions the restore forces" have published counterparts here:

| our decision | CHONK's |
|---|---|
| submerged cells do not incise (mask) | *"All the cells below water are 'de-processed' from continental processes: fluvial and hillslope processes are **reversed** with adequate correction on cells sediments and water contents."* Their outlet is only *partially* de-processed, since it feeds both the lake and downstream. |
| trapping efficiency 1 until full, surplus spills | same shape: sediment stored up to the known final volume, *"Any excess is transmitted to the outlet cell."* Our Working Note calls trapping-efficiency-1 "the least defended of the three"; the field ships it too. |
| endorheic level | **a water balance including evaporation**, their Eq. 2, iterated over cells in increasing elevation order: $V_{w\,\text{avail}} = V_{w\,\text{avail}} - N_{\text{lake}}\,dx\,dy\,(\Delta z + Q_{w\,\text{evap}})$, terminating when $V_{w\,\text{avail}} < N_{\text{lake}}\,dx\,dy\,(\Delta z + Q_{\text{evap}})$. |

That third row is the real answer to the wet-limit caveat, and it is not "volume-limited fill" alone — it is **fill limited by a supply/evaporation balance**, which is why an endorheic lake has a stable level rather than merely an under-filled one. Their §4 result states the mechanism: *"Lake evaporation balances water input in the lake and enables a decoupling where the would-be outlet of the lake does not receive any water or sediments from the lake, inhibiting its erosion."* An endorheic basin is thus not a static curiosity; it changes what the outlet does, which changes the carve.

The upstream lineage CHONK credits for this, which is where to go for the geology rather than the algorithm: **Garcia-Castellanos (2006), "Long-term evolution of tectonic lakes: climatic controls on the development of internally drained basins," GSA, doi:10.1130/2006.2398(17)** (1D), and **Garcia-Castellanos & Jiménez-Munt (2015), *PLOS ONE* 10, e0132252, doi:10.1371/journal.pone.0132252** (2D, the TISC model). **I opened neither** — identities are from CHONK's reference list, and both are named there as the origin of endorheism in LEMs. If the evaporation-balance route gets taken, these are the two to read first, and both look open-access.

### 1.4 Off the brief's list: the ε can be deleted, not bounded — and our "long straight runs" is its documented signature

`obs-lakes-are-routed-over-not-carved-away` FE(5) leaves open, as argued-not-measured, that the ε's *directional* half still reaches the bed through the incision pattern, and points at `examples/discharge_probe`'s longest-identical-D8-direction runs. The Working Note's recorded prediction is that the artefact persists at reduced amplitude.

The literature both confirms the prediction's mechanism and offers a way to make the measurement unnecessary. RichDEM's flat-resolution documentation states the ε method's drainage *"takes a least-distance route to the flat's edges"* — i.e. **straight runs are what ε-filling produces, by construction, and they are the known aesthetic complaint against it.** Our measured long straight runs in filled ground are not a surprise to be characterised; they are the named signature.

The published replacement:

**Barnes, R., Lehman, C., & Mulla, D. (2014). "An efficient assignment of drainage direction over flat surfaces in raster digital elevation models." *Computers & Geosciences* 62, 128–135.** Preprint arXiv:1511.04433, whose abstract I read directly. Note this is the **companion** to the Priority-Flood paper we already cite (`barnes-2014-priority`, same authors, same journal volume, pages 117–127) and is a different paper — easy to conflate.

It resolves flats with **no ε at all**, by superimposing two gradients — one directed away from higher surrounding terrain, one toward lower terrain — yielding convergent flow rather than parallel least-distance flow. RichDEM's docs describe the visible difference as *"deep V-shaped notches in the flats indicating the increased convergence."* Complexity **O(N)**, against Garbrecht & Martz (1997) at O(N^{3/2}); reported speed-ups of 6.5× on 100×100 flats and 69× on 700×700.

Why this is attractive for us specifically, beyond the artefact: it retires an entire family of open items at once. No ε means no cumulative-rise hazard, no f32 binade precondition (both catalogued in the prior dossier §4.2), no `EPS = 1e-3` magic constant in `ASSUMPTIONS.md`, and no directional artefact to measure — FE(5)'s open half stops being a question. It also removes the one thing that makes the fill's mass and direction halves separable-but-entangled.

The caution, stated by RichDEM in the same breath and worth carrying into any segment: *all* reconstructed drainage in a filled depression is artificial, because *"no information about local gradients remains from the original DEM."* Swapping ε for convergent flats replaces an artefact we have measured with an artefact we would not have measured. It is a better artefact on published grounds; it is not an absence of one. Given `#norm-declaration-must-convict`, the honest landing is a `nomotheke`-style declaration either way, and the choice is which declaration is cheaper to defend.

### 1.5 If the per-epoch cost of filling ever binds, the answer is Cordonnier, not a faster Priority-Flood

Worth holding because our fill runs **every epoch** and that is exactly the regime the following paper was written for:

**Cordonnier, G., Bovy, B., & Braun, J. (2019). "A versatile, linear complexity algorithm for flow routing in topographies with depressions." *Earth Surface Dynamics* 7, 549–562. doi:10.5194/esurf-7-549-2019.** Open access; read the abstract and introduction. (Note: our bibkey is `cordonnier-2018-versatile` and the metadata says year 2018 — **the paper is 2019**; the DOI in the entry is already correct. Minor bib defect worth fixing before it is cited.)

It builds an explicit graph of drainage basins and computes flow paths within and across depressions, reaching **linear** time complexity where Priority-Flood and its variants (they cite Zhou et al. 2016 and Wei et al. 2018 as the optimised variants) do not. Their claim is specifically comparative and specifically about our situation: *"Compared to the most optimized solutions proposed so far, we show that this algorithm of flow path enforcement yields the best performance **when used in landscape evolution models**."* Their stated reason is that LEMs re-run flow enforcement at every timestep. They also let the user choose filling versus carving, which is a knob we currently do not have and which FastFlow's water-surface variant requires.

And if this ever needs to be interactive rather than merely fast, **FastFlow** (§1.1) is the GPU end of the same lineage — Cordonnier is a co-author, and it recasts depression routing as a parallel minimum-spanning-tree problem (Borůvka), reporting **O(log n)** iterations for flow routing and **O(log² n)** for depression routing, with measured 5× over prior GPU flow routing and 34–52× over parallel CPU depression routing on a 1024² terrain. Their framing of the win is the one that would matter to us: previous methods *"usually require as many iterations as the length of the longest river (∼√n)."*

I am **not** recommending either of these tonight. Priority-Flood at 64² tiles is not the bottleneck, and `obs-lakes` FE(10) already measured the fill's cost as 12.0 s against 12.4 s for 384 L9 tiles — i.e. free. They are catalogued because the ordering "adopt FSM, then discover the hierarchy is rebuilt every epoch" is a foreseeable trap, and this is the exit.

---

## §2 The settle half

### 2.1 Direct answer: no, marching a depth-averaged SWE is not how this is done for slow-filling hydrology at coarse scale

The current practice for exactly our problem — equilibrated water depth over large domains, coarse cells, coupled to an evolving bed — is a **stationary solver on the flow graph**, not a marched transient. The reference implementation:

**Gailleton, B., Steer, P., Davy, P., Schwanghart, W., & Bernard, T. (2024). "GraphFlood 1.0: an efficient algorithm to approximate 2D hydrodynamics for landscape evolution models." *Earth Surface Dynamics* 12, 1295–1313. doi:10.5194/esurf-12-1295-2024.** Open access; read in full.

GraphFlood solves the same 2D shallow-water mass balance we do, with Manning as the momentum closure, but it *iterates to the hydraulic surface* instead of marching time. Each iteration rebuilds the DAG **from the hydraulic surface $Z_h = Z + h$ rather than from the topography**, propagates $Q_{in}$ through the whole landscape at once by weighted drainage accumulation, computes $Q_{out}$ from Manning, and increments $h$ by the divergence. Convergence is declared on median $\Delta h < 1\times10^{-9}$ m.

Their diagnosis of the transient approach is a precise description of our 40 seconds:

> "its potential for longer-term and larger-scale studies remains hampered by the physics behind, which explicitly and gradually transfer water from one cell to another. Specifically, any upstream change in runoff input (e.g. precipitation) must be gradually propagated downstream one pixel per computational time step."

And the measured cost of that, on their validation channel: the stationary solvers reach the analytical depth in ~300–1000 iterations, *"roughly 400 times faster than the transient model."* Verified other numbers: ~10× faster than River.lab (Davy et al. 2017, the precipiton method); agreement with CAESAR-Lisflood and River.lab on a natural DEM centred at 3–5 × 10⁻⁴ m; convergence *for the fluvial domain* in ≲200 iterations at Green River and ≲60 at Hanalei, against 3000–4000 for the whole landscape including hillslopes; 20 s for the main rivers on ~10⁶ cells; 83 × 10⁶ cells converged on a 32 GB laptop in ~20 h at 100 s/iteration; scaling *"slightly more than linearly."*

### 2.2 The result that decides it: at steady state, the local-inertial scheme and the stationary solve target the same answer

This is the single most useful thing I found, and it is in a paper already sitting in `ref/`. From de Almeida & Bates (2013), §4, paragraph [15], verbatim from the local PDF:

> "At this point it is important to highlight that **under steady flow conditions, the local inertial system is in fact equivalent to the diffusion wave model**, as the local acceleration term in the momentum equation is zero."

And GraphFlood's momentum closure *is* the diffusion-wave/Manning form — they state they neglect inertia explicitly.

Therefore: **replacing our marched settle with a stationary solve is not an accuracy downgrade at equilibrium. It is the same equilibrium, reached without the transient.** The local-inertial formulation's whole advantage over the diffusion wave is a *stability* advantage during transients (de Almeida & Bates say so directly), and it buys nothing once the target is the stationary state.

That is what dissolves the brief's framing. "Step-size is a physics decision" is a true and important statement about a flood transient. For a lake — an object defined by being at equilibrium — **the step size is not a physics decision, because there is no reason to take steps.** The blocker is real; it is attached to the wrong deliverable.

The corollary is sharper still, and it is the reason marching lakes is a known bad idea rather than merely an inefficient one. Also from de Almeida & Bates (2013), §2, on why the local-inertial form exists at all:

> "the maximum stable time step in the explicit diffusion wave model decreases quadratically with grid refinements […] In addition, the stable time step for the diffusion wave model also depends on the water surface gradient […] so that its computational performance is **dramatically reduced in zones of near horizontal water surface. This is particularly important for large-scale simulations that unavoidably include flat water surface areas (e.g. lakes and lowland rivers).**"

They attribute this to **Hunter et al. (2006), "Improved simulation of flood flows using storage cell models," *Proc. Inst. Civil Eng. Water Manage.* 159(1), 9–18** — identity read from de Almeida's reference list; **I did not open Hunter**, and the quantitative scaling claim above is de Almeida & Bates reporting it, not me reading it. (The adjacent adaptive-timestep paper in the same reference list is Hunter et al. 2005, *Adv. Water Res.* 28, 975–991, doi:10.1016/j.advwatres.2005.03.007 — also unopened.)

A flat water surface is the *worst case* for an explicit marcher's stable step. A lake is a flat water surface. Marching to fill one is walking into the documented pathology of the scheme family, and it is precisely why the equilibrium solvers exist.

### 2.3 We are one outer loop away from GraphFlood, and it reuses what §1 already needs

This is not a rewrite. Reading GraphFlood's Algorithm 1 against our tree, every component exists:

| GraphFlood needs | we have |
|---|---|
| sink filling / local-minima resolution | `fill_depressions` (they cite Barnes et al. 2014 and Cordonnier et al. 2019 for exactly this step) |
| topological ordering | `elevation_order` |
| receivers, single or multiple flow | `receivers`, MFD `p = 1` |
| weighted flow accumulation | `accumulate_drainage` |
| Manning $Q_{out}$ | already in `water.rs`, and now dt-free (§0.1) |
| **the DAG rebuilt on $Z + h$ each iteration** | **missing — this is the whole delta** |

Their own words on portability: *"One advantage of GraphFlood is that it can be implemented using existing computational frameworks for DEM analysis and landscape evolution model simulation […] the base of the algorithm only needs to calculate flow direction and topological order. A notable difference compared to existing frameworks is that we calculate the directed acyclic graph using the hydraulic surface rather than the topography."*

Four cautions from their results, all measured and all relevant:

1. **Δt does not vanish, it changes meaning.** In stationary mode Δt is *"a numerical stability criterion modulating the magnitude of flow depth increment"* — not a physical step. And they measured the failure mode we should expect: *"while significant overestimation provokes numerical divergence, slight overestimation converges to an underestimated final h."* A step-size knob that silently biases the converged answer low is exactly the shape of defect our clip census keeps finding, so it wants a probe from the start rather than after.
2. **CFL does not apply to half of it.** *"CFL conditions only theoretically apply to our calculation of Qout but not to the propagation of Qin in stationary mode."* Our `stable_dt` has no jurisdiction over the new loop.
3. **Single-flow under-estimates depth.** Their SFD-vs-MFD comparison finds SFD globally under-estimates $h$, with error *"concentrated around 10 %"* at Green River, because the single line over-estimates $Q_{in}$ while every other channel node over-estimates $Q_{out}$. We are MFD, so we are on the right side of this — worth knowing before anyone proposes D8 for speed.
4. **Coarsening changes the answer in a stated direction.** *"Lowering the resolution leads to lower hydraulic slopes on average and subsequently a decrease in $Q_{out}$ and an increase in the total volume of water stored on the DEM."* At kilometre cells this is not a footnote. It predicts the sign of our coarse-grain bias — more water stored than there should be — and is directly falsifiable on the fidelity ladder.

Hillslope convergence is the honest cost: 3000–4000 iterations for the whole landscape versus ≲200 for the fluvial domain, because low drainage area gives small depth increments. Their own remedy is the induced-subgraph variant (one iteration 15 ms against 250 ms; convergence 3 s against 50 s), which processes only nodes above a discharge threshold via a priority queue in **decreasing** elevation order — *"opposite to Barnes et al. (2014)"* — and gradually fills local depressions when a node has no receiver. For "a lake on screen," the fluvial-plus-lake subset is the whole target, so this optimisation is aligned with the deliverable rather than a later refinement.

### 2.4 Coarse cells in a marching LEM: the field has measured this, and the finding is a warning about interpretation

The brief asks whether coarse grid scale reframes things. The direct study is:

**Skinner, C. J., & Coulthard, T. J. (2023). "Testing the sensitivity of the CAESAR-Lisflood landscape evolution model to grid cell size." *Earth Surface Dynamics* 11, 695–711. doi:10.5194/esurf-11-695-2023.** Open access; read abstract and introduction.

CAESAR-Lisflood is the LEM that *does* march the Bates/de Almeida local-inertial scheme, so this is the closest measurement of the path we are on. Verified from the abstract: grid cell size had *"a similar level of influence as a key hydrological parameter and the choice of sediment transport law"*; discharge and sediment yields *"remained stable across different grid cell sizes until the cells became so large that the representation of the hydrological network degraded."*

The finding worth carrying, in their words:

> "Although total sediment yields remained steady when changing the grid cell sizes, closer analysis revealed that using a coarser grid resulted in it being built up from **fewer yet more geomorphically active events, risking outputs that are 'the right answer but for the wrong reasons'.**"

Read against `#form-fidelity-invariant` and the ladder, that is a caution about what a coarse-grain agreement licenses. Aggregate agreement across grain — which our seam-ratio and land-trunk tables largely report — is compatible with the event structure underneath having changed character. Their §1 also collects the flood-modelling side, including that LISFLOOD-FP performance degrades above 50 m cells because the channel is poorly represented (they attribute this to Savage et al. 2016; **unopened**), and that Manning's $n$ trades against resolution — high-resolution DEMs performing better with higher $n$, coarse with lower (attributed to Lim & Brandt 2019; **unopened**). Our Jarrett ceiling pinning two-thirds of wet cells at $n = 0.13$ on kilometre cells sits on the wrong side of that reported trade, which is an unmeasured tension rather than a defect.

### 2.5 A sharpening nobody asked for: our Froude gauge is not the envelope the citation supports

`ASSUMPTIONS.md` and `DECISIONS[water-runs-outside-its-published-validity-envelope]` gauge the kernel against **Fr > 1.5**, and the probe file replaced the stale 5.7 % with a measured 0.35 %.

But **Fr = 1.5 is the Manning Vedernikov critical, not de Almeida & Bates' validity envelope** — and the probe file's own §1.2 refuted the Vedernikov identification. So the threshold survived the death of its justification. What de Almeida & Bates (2013) actually establish, from their Conclusions [44]–[45] (read verbatim):

- the relative error in predicted depth gradients **scales with Fr²**;
- at **Fr < 0.5** the errors are *"relatively low"* — a maximum depth error of *"around 2–4 %"* for depth slopes up to ~3 %;
- at Fr in the **0.4–0.8** range with mild depth change, a maximum depth error of **4.5 %**;
- for **0.5 < Fr < 1.0** *combined with more pronounced depth variations*, *"the disparity between the two models becomes more relevant"*;
- unsteady flow: the simplified characteristic speed $\pm\sqrt{gh}$ ignores the flow velocity, giving **slower flood-wave propagation**, *"of a relatively small significance"* at low Fr and *"relevant at high Fr."*

Their entire analysis is **subcritical**. The paper makes no validity claim above Fr = 1 at all, and its own good-agreement band is Fr < 0.5.

So the honest gauge is *"what fraction of wet cells run Fr > 0.5"*, and **nobody has measured it.** It is one line in an existing instrument, it is strictly more demanding than the current gauge, and it is the number the citation actually licenses. I would expect it to be much larger than 0.35 % — which, if so, means the validity-envelope decision is understated rather than overstated, and strengthening it is the strengthen-before-soften move available here.

(Two adjacent numbers I verified while there, in case they are useful: de Almeida & Bates report, from Neal et al. 2012, that the local inertial model ran *"up to seven times faster than the full-dynamic model, and more than 2 orders of magnitude faster than the diffusion wave model."* That last figure is the transient-marching comparison and does **not** contradict GraphFlood's 400× — GraphFlood is not a diffusion-wave *marcher*, which is the entire point of §2.2. Neal et al. 2012 **unopened**.)

---

## §3 Things I did not expect to find

### 3.1 We solved this once already, at 4 m cells, and wrote down why it worked

`ref/hydrology/NOTES.md` (authored 2026-06-29, describing the retired `vivarium-core/src/hydro.rs`) is not in the brief's reading list and is the closest prior art to tonight's task in the whole tree — including the wider literature. It documents a working three-phase composition and states the governing principle as its headline lesson:

> "**The single most important lesson: separate the timescales.** […] water finds its level in hours–days; landscapes are reshaped over millennia. The first version of this system coupled erosion *into* the water sim on one timestep and then cranked the erosion rates to carve in a few sim-minutes — which made the numbers mutually incoherent […] **There was no physical time at all** — every rate was tuned to make a picture in N iterations."

Its phase 3 is *"pure shallow-water + groundwater run to hydrological steady state on the **fixed** bed (lakes level, streams settle), then freeze the snapshot. Carves nothing, so nothing is time-conflated."* And it claims the outcome we want, by emergence: *"A flat lake **emerges** (gravity dissipates the head difference until the surface is flat) — it is never imposed by a 'fill'. The proof is `basin_fills_to_a_flat_lake`."*

Two things follow, and they point in opposite directions.

**The encouraging one:** marching to flat lakes is demonstrated in our own history, so the current 40-second failure is not evidence that marching cannot work. It is evidence about *scale*. That NOTES ran at `SIM_CELL_M = 4`, and `obs-water-fill-never-settles` FE(2) identifies the 0.2 s clamp ceiling as the CFL step for cells of roughly **2 m** — the fine testbench's scale. The clamp is not arbitrary; it is the *correct* calibration for the world this NOTES describes, carried unchanged into a path whose cells are kilometres. That is a much more sympathetic account of the clamp than "a magic constant," and it belongs in the segment.

**The discouraging one, and it is the more important:** that NOTES also records, in its own honest-fudges list, what filling those lakes actually cost.

> "**Rain rate is ~100–1000× real** (`precip_rate ≈ 0.006 m/s`). Tuned to fill the basins in ~40 min of physical water time instead of weeks. The *physics of how water moves* is time-honest; the *amount delivered per second* is not."

So the previous generation reached flat lakes by marching, and paid for it with a forcing three orders of magnitude off. **That is the price of the marching path, measured in our own tree, and it is a `#norm`-grade honesty problem rather than a performance one** — it is precisely the "no physical time at all" failure the same document warns against, displaced from the rate constants into the boundary condition. A world that fills its lakes with 1000× rain has a lake on screen that is not honestly a lake.

And the fix that NOTES names for itself is the one this dossier arrives at from the literature: *"The fix is real precip + **priming the channels with steady-state discharge so they don't start from dry**."* That is GraphFlood's stationary solution, described in 2026-06 without knowing the algorithm existed.

I would treat this file as the strongest internal argument for the equilibrium path, and as a finding about the brief: the honest-fudges section of a retired prototype's NOTES was worth more to this question than any single paper.

### 3.2 The gap the field has not closed, so we should not expect to adopt our way out of it

Worth naming so it is not discovered late. Every equilibrium method above solves for water **on a fixed bed**, and every coupled method (CHONK, CAESAR-Lisflood) alternates: solve water, then move sediment, then repeat. Nobody solves the joint bed-plus-water equilibrium. FSM routes flow *"just over the land surface"* while FlowFill evolves a land-plus-water routing surface, and the FSM authors are explicit that this is a trade rather than a strict improvement — *"These differences make FlowFill more useful for understanding temporal changes in surface water distribution, while Fill–Spill–Merge provides a more accurate snapshot of surface hydrology under equilibrium conditions."* GraphFlood, by contrast, *does* iterate on $Z + h$, which is a genuine methodological difference between the two and matters if the lake surface is meant to feed back into routing.

So `obs-lakes-are-routed-over-not-carved-away` FE(12)'s scope note — *"any claim that water now stands in these basins"* being out of bounds — is not a temporary local gap. It is where the field's seam also is. Our advantage is that we can *declare* the alternation honestly under `#form-time-indexed-stage-chains` rather than leaving it implicit, which is a thing the surveyed models mostly do not do.

---

## §4 What I would do tonight, and what I could not verify

### 4.1 Recommendation

The equilibrium-lake path is right. I would order it differently than the brief implies:

1. **Fix the outlet set first (§0.3).** Connectivity, not elevation threshold. Cheapest item, gates the visible outcome, and no lake algorithm helps until it is done. Convict it with the below-datum-crater test before believing my inference.
2. **Ship the wet limit as-is and cite it (§1.1).** `spill` is FastFlow's construction. This is the fastest honest lake on screen, and the segment can stop hedging about whether it is principled.
3. **Then the stationary solver, not a longer march (§2.1–2.3).** One outer loop on $Z + h$ over machinery we already have. Probe the Δt-biases-low failure mode from the start.
4. **Volume-limited fill when the wet limit visibly lies (§1.2), and evaporation only when an endorheic basin is the actual subject (§1.3).** FSM needs the depression hierarchy; the MFD constraint is real; CHONK is the record of what working around it costs.
5. **ε deletion (§1.4) is independent of all of the above** and retires more open items per line changed than anything else here.

The one thing I would *not* do is raise the clamp and march longer. Not because it is unsafe — §0.1 removed the main reason to think so — but because §2.2 says the destination is reachable without stepping, and §3.1 says our own last attempt at stepping to it cost a 1000× forcing.

### 4.2 Could not verify — and one I would like pulled in

Per the standing grant, named precisely:

- **`gailleton-2021-dynamic` — paywalled, and it is the one I most want.** `relata inspect` reports `unpaywall → no-oa-pdf`, `doi-direct → landing-page-only`, `STATUS: paywalled`. Its title in our library is *"Dynamic modelling framework to track sediment provenance and solve lakes…"* — i.e. it appears to be the direct predecessor of CHONK's lake solver by the same lead author, and it may state the lake algorithm more carefully than CHONK's condensed §3.4. **Joseph: this is the one worth pulling.** Everything I say about lake-solver internals currently rests on CHONK alone.
- **Barnes et al. (2020), Part 2 (depression hierarchies)** and **Callaghan & Wickert (2019), Part 1 (FlowFill)** — both stated open access, both merely not fetched by me for time. Part 2 is a genuine prerequisite if FSM is adopted, since it is the data structure. Identities taken from FSM's reference list.
- **Garcia-Castellanos (2006)** and **Garcia-Castellanos & Jiménez-Munt (2015)** — the endorheic/TISC lineage, identities from CHONK's reference list, not opened. Needed only if the evaporation-balance route is taken.
- **Hunter et al. (2006)** — the quadratic-and-gradient-dependent stability scaling in §2.2 is de Almeida & Bates reporting it. I did not open Hunter. The claim is load-bearing enough for the framing that if it goes into a segment it should be read.
- **Savage et al. (2016)**, **Lim & Brandt (2019)**, **Neal et al. (2012)** — all cited via Skinner or de Almeida, all unopened, all named as such in §2.4–2.5.
- **Not attempted at all:** whether any 2025–2026 work supersedes GraphFlood, FSM, CHONK or FastFlow. I searched and found no successors, but a null result from two web searches is weak evidence, and Undermind-grade coverage would be the honest instrument. What I can say is that these four are mutually citing, span 2021–2024, and none of them points at a method it considers better than itself.

### 4.3 Bibliographic defects found in passing

- `cordonnier-2018-versatile` is dated **2018** in relata; the paper is **2019** (*Earth Surf. Dynam.* 7, 549–562). The DOI in the entry is correct. Anything citing it currently mis-dates it. CHONK, FSM and GraphFlood all cite it as 2019.
- `barnes-2014-priority`'s page range in relata renders as 117–127; the FSM and GraphFlood reference lists both give **117–127** — consistent, but note the *companion* flat-surfaces paper (§1.4) is pages **128–135** of the same volume and is **not** in relata. Two different papers, same authors, same volume, adjacent pages; a future agent will conflate them if only one is in the library.

---

## §5 Feedback on the brief

**The beliefs-versus-measurements split worked, and then the brief did the thing it warned against.** The register separation is what made me check §1.2 against the code instead of building on it — and that is how §0.1 surfaced. But the brief presented the friction defect in measurement register (*"A same-day probe report […] measured that"*) when the same document's later sections had already withdrawn it. The staleness was not in the world; it was inside the cited artefact, one screen further down. The generalisable version of the lesson: **when citing a live working document by section, check whether a later section supersedes the one you are citing.** A same-day file is the *most* likely to contain its own refutation.

**Naming the prior-work directory and asking me to say if it already answered something was the highest-value line in the brief.** It saved me from re-deriving the ε precision arithmetic, which the prior dossier had done more carefully than I would have (§0.4). I would keep that line verbatim in future briefs.

**The amendment was worth more than the original reading list, and the gap it closed was structural.** `ref/` and `relata` held nine of the eleven papers this dossier turns on, including both decisive ones — I would have gone to the open web and read abstracts for things sitting on disk in full text. But the more interesting item was `ref/hydrology/NOTES.md` (§3.1), which is not a paper at all and which no amount of literature searching would have found. **The strongest source for a "is our account state of the art" question turned out to be our own retired prototype's honest-fudges list**, because it recorded the *price* of the path being reconsidered, and prices are exactly what published papers omit. If there is a durable rule: when asking whether an approach is current, check what the last attempt at it cost us, not only what the field says about it.

**One place the brief's framing shaped my answer and I nearly did not notice.** It asked about "the lake half" and "the settle half" as two halves, and they are genuinely coupled through one thing neither half names: the outlet set. I spent my first hour inside the two-halves frame and found §0.3 only because I went to read the project's own Caspian note to check the brief's gloss on it — the gloss was wrong, and the wrongness was the finding. Had the brief's description of the Caspian gap been accurate, I would probably have accepted it and never opened `outlets()`. That is an uncomfortable thing to report, since it means a *better* brief would have produced a worse dossier here; the transferable form is that a brief's one-line summaries of internal notes are worth checking against the notes precisely because they read as settled.

**What would have helped:** knowing that "put a lake on screen" has a rendering half. I have said nothing about depiction, and `#norm-no-depiction-without-referent` plus `form-core-view-wall` clearly bear on which of §4.1's steps actually produces something visible. If the water field already renders, step 2 may be tonight's whole job; if it does not, none of the above is sufficient and I have not looked at the instrument at all.

---

*Standing by for follow-ups.*
