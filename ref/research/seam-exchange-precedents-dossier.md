# Seam-exchange precedents: domain decomposition for the halo, and cross-subdomain flux for landscape evolution

*Literature dossier, 2026-07-29. Commissioned to close the gap named in `#form-same-level-halo-exchange` Working Notes ("Precedent not yet held… the cheapest outstanding piece of the design") and to answer the second question the flux half raises. Read-only on canon: nothing outside this file was modified.*

**Register.** Every claim below carries one of: **[verified]** — I read the primary or an authoritative publisher page in this session and quote it; **[reported]** — I have the statement second-hand from a search summary or a citing paper and did not open the primary; **[mine]** — my inference or arithmetic, not in any source. Citations marked *(unverified detail)* have a component (page range, exact venue) I could not confirm; the author/year/title are good. I have not invented a citation; where I could not get a source, I say so.

---

## 0. Verdict in four lines

1. **The Schwarz analogy is half-broken, and the half that survives is sharper than a convergence theory would have been.** Classical Schwarz is an *iteration toward a fixed point of a stationary problem*; the exchange is a *forward evolution*. The convergence theory therefore does not give $(d,\sigma)$ a footing. What does transfer, exactly and non-negotiably, is the **causal-cone / overlapped-tiling condition**, which says the governing dimensionless group is $d/(v\sigma)$, not $d$ and not $\sigma$ separately — and predicts *exactness*, not accuracy, when it exceeds 1.
2. **Your own published table already tests that prediction and passes it.** Mean-elevation error is **monotone in $d/(v\sigma)$ across all six exchange arms**, with the sharp drop exactly where theory puts it (§2.3). This is a re-reading of `#obs-exchange-repairs-the-seam-and-overlap-does-not` FE(7), not new measurement, and it is the highest-value item here.
3. **Cross-subdomain flux for landscape evolution is solved, repeatedly, and nobody solved it with a halo.** Three published families: a **two-pass global summary graph** (Barnes), a **reformulation of accumulation as a sparse linear system** (Richardson–Hill–Perron; Bangerth 2026), and a **serial master step** (eSCAPE). All three are decomposition-independent by construction; the halo appears only as a fallback that people iterate *to convergence*, never at a fixed cadence.
4. **The spill-level object of FE(9) exists, is published, and is exact.** The *depression hierarchy* + *Fill–Spill–Merge* (Barnes–Callaghan–Wickert 2020, 2021) is the per-basin scalar structure your FE(9) names as unbuilt, worked out in detail — including the merge semantics you will need when two lakes join. It is not parallelized in the source, which is exactly the gap your design would fill.

Two things I would flag as changing build order are in §4. One correction to the brief's framing is in §5.

---

## 1. What the question actually is, restated

Reading the segments before searching changed my search. The brief posed Q1 as "does DD theory give $(d,\sigma)$ a footing," which presumes the relevant DD object is a *convergence rate*. It is not, and the reason is in your own FE(2):

> "Every $\sigma$ epochs the interiors of all tiles are assembled into a snapshot; each tile then overwrites its halo cells from that **one frozen snapshot** … and carries on."

The tile *carries on*. It does not re-solve the same $\sigma$ epochs with better boundary data. So the iteration index that all Schwarz convergence theory is stated in — $u^k \to u$ for fixed problem — **has no counterpart in your scheme**. You take exactly one Schwarz sweep per time window and then advance the window. In DD vocabulary that is not "Schwarz with lagged data"; it is a **non-iterative / explicit-predictor domain decomposition** (§2.6), a much smaller and more conditional literature, or — better — it is not a solver at all but **overlapped tiling with temporal blocking** (§2.2), which is an HPC construction with a hard exactness theorem rather than an error bound.

Getting that mapping right is what makes §2.3 possible. If I had gone looking for a convergence rate I would have found Gander's estimates, matched symbols, and handed you a plausible-looking $\sigma$ law that is about the wrong object.

---

## 2. Q1 — what domain decomposition says about $(d, \sigma, \rho)$

### 2.1 The three registers, and which one you are in

| Register | Object | What varies per round | Theory available | Your scheme? |
|---|---|---|---|---|
| **Schwarz iteration** (elliptic) | preconditioner for a linear solve | iteration index $k$; problem fixed | condition number $\kappa \le C(1+H/\delta)$ | no |
| **Schwarz waveform relaxation** (evolution) | iteration over a *space-time* window | iteration $k$; window $T$ fixed | $\|u^k-u\| \le C^k\,\mathrm{erfc}\!\big(k\delta/(2\sqrt{\nu T})\big)$ | no — you never take $k=2$ |
| **Overlapped tiling / ghost-zone expansion** | redundant-compute schedule for an explicit march | time window; *no* iteration | **exactness** iff $d \ge \sigma s$ | **yes** |

Vivarium's exchange is row 3 wearing row 1's name. FE(2)'s cost formula $((n+2d)/n)^2$ is *literally* the redundant-computation overhead formula of row 3, which is a good independent sign the mapping is right rather than forced.

### 2.2 The construction that actually matches — and it carries an exactness theorem, not an error bound **[verified]**

**Ghost cell expansion** (Ding & He 2001) is the named ancestor: "expands the ghost cell layers and thus updates boundaries much less frequently … enables computations to proceed over multiple time steps without requiring boundary updates" [reported, from the publisher summary; the paper is SC'01, doi 10.1145/582034.582084]. The compiler/GPU lineage states the geometry precisely — from AN5D (Matsumura et al., CGO'20) [verified via search result text]:

> "overlapping the spatial blocks by $2 \times b_T \times \mathrm{rad}$ rows and columns (called halo regions) and redundantly loading and computing necessary cells that would fall inside surrounding blocks to process $b_T$ time-steps with only one round of global memory loads and stores per block"

Read into your symbols: $b_T = \sigma$, $\mathrm{rad} = s$ (stencil radius), halo $= d$. The condition is

$$\boxed{d \;\ge\; \sigma\, s}$$

and — this is the part that has no analogue in Schwarz theory — **when it holds the interior result is bit-identical to the undecomposed computation**. These are *correctness-preserving compiler transformations*; the entire literature (time skewing, trapezoidal/cache-oblivious blocking, communication-avoiding matrix powers) exists on the premise that the answer does not change. There is no accuracy/cost tradeoff in $(d,\sigma)$ at all; there is a hard threshold and, above it, pure compute overhead.

**So the theory's first-order verdict on your design is not "here is your error budget" but "you should have zero error, and you do not."** `#obs-exchange-repairs…` FE(7) reports $d{=}16$, $\sigma{=}10$ (so $d/\sigma = 1.6 > 1$, with $s=1$) landing $4.8\,\mathrm{m}$ from the single-field carve rather than $0$. **[mine]** That residual is therefore not a discretisation error and not a "budget" in the ordinary sense — it is a *direct measurement of how far your kernel departs from finite-speed locality*, i.e. of the priority-flood and accumulation being global-within-domain. That reframing is free and it upgrades `#disc-unlawfulness-budget`'s number from a tuning constant to a diagnostic of a named physical fact.

Caveat I owe you: **the premise formally fails at epoch one.** Priority-Flood is a global operation on the domain, so the influence speed is unbounded in principle, and the exactness theorem has no right to apply. What rescues it empirically is your own FE(2): a $1\,\mathrm{mm}$ perturbation took $\approx 100$ epochs to cross $96$ cells, i.e. an *effective* front speed of $\approx 1$ cell/epoch. The honest statement is: **the operator is formally infinite-speed; the measured influence front is $\approx s = 1$ cell/epoch; the gap between them is the residual.** That is a testable decomposition, not a story (§4.1).

### 2.3 The re-reading of your own table — $d/(v\sigma)$ organises all six arms **[mine, arithmetic on your published numbers]**

If $d/(v\sigma)$ is the governing group, then arms with the same ratio should agree and error should fall monotonically in it. Taking FE(7)'s mean elevation against the single-field reference ($6128.8\,\mathrm{m}$), with $v = 1$ cell/epoch:

| arm | $d$ | $\sigma$ | $d/(v\sigma)$ | mean $h$ (m) | $\lvert$err$\rvert$ vs REF (m) |
|---|---:|---:|---:|---:|---:|
| EXCHANGE | 8 | 50 | **0.16** | 6079.9 | 48.9 |
| EXCHANGE | 4 | 10 | **0.40** | 6086.9 | 41.9 |
| EXCHANGE | 32 | 50 | **0.64** | 6087.6 | 41.2 |
| EXCHANGE | 8 | 10 | **0.80** | 6096.6 | 32.2 |
| EXCHANGE | 16 | 10 | **1.60** | 6124.0 | **4.8** |
| EXCHANGE | 32 | 10 | **3.20** | 6127.0 | **1.8** |

Six arms, two dials, **strictly monotone in the single ratio**, with the collapse (32.2 → 4.8 m, a factor of 6.7) falling exactly across $d/(v\sigma) = 1$ — the causal-cone threshold — and then flattening onto a floor of $\approx 2\,\mathrm{m}$, which is within about $2\times$ of the $0.9\,\mathrm{m}$ chaos floor FE(2) prices for mean elevation. That is the shape the theory predicts *including* the deviation it predicts: a step at 1, softened into a steep drop and a floor by the non-local mechanisms that break the premise.

Three honesty notes, because this is the claim most likely to be over-read:

- It **does not hold for mean $\lvert\Delta h\rvert$** (110.6, 108.2, 98.9, 100.3, 114.9, 90.0 against the ratio order) and it should not: that column is pointwise-flavoured and FE(2) already established pointwise agreement is not resolvable. The group organises the *structural* statistic and not the chaotic one, which is a consistency check rather than a weakness.
- The seam-step column's distance from REF (0.042, 0.117, 0.083, 0.023, 0.006, 0.051) is best near $d/(v\sigma) \approx 0.8$–$1.6$ but is not monotone; at six points with a chaos floor I would not read it either way.
- Six points, one seed, one grain, one epoch count, and $d$ and $\sigma$ are not independently randomised. **A monotone ordering of six points is suggestive, not a law.** The cheap falsifier is in §4.1.

**If it survives, three things follow.** (a) `#form-same-level-halo-exchange` FE(6)'s operating point becomes *derived* rather than *measured-and-adopted*: pick $\sigma$ from the stage stride, then $d = \lceil \kappa\, v\, \sigma \rceil$ with $\kappa \gtrsim 1.5$. (b) FE(7)'s forced coupling of $\sigma$ to `stage_stride` acquires a cost consequence — lengthening the stride now *requires* deepening the halo quadratically in compute, which is worth knowing before the stride is chosen for unrelated reasons. (c) The Working Note "exchange frequency is the cheap dial and halo depth is the expensive one" stays true in the cost currency and becomes *misleading as a design rule*: both dials move one ratio, and once $d/(v\sigma) > 1$ neither buys anything but overhead. Halving $\sigma$ and halving $d$ is free-vs-cheaper at the same fidelity.

### 2.4 What Schwarz waveform relaxation *does* say, stated so it is not over-transferred **[verified]**

Gander's survey (*Waveform Relaxation*, Encyclopedia of Applied and Computational Mathematics, Springer) gives the classical estimates verbatim. For overlapping SWR on diffusive problems:

> "converge superlinearly for diffusive problems, like the heat equation, with an error estimate of the form $\|u^k - u\| \le C^k \mathrm{erfc}\!\left(\frac{k\delta}{2\sqrt{\nu T}}\right)\|u^0-u\|$, where $\delta$ represents the overlap"

and, for the register your incision wave lives in:

> "For the wave equation, and more generally for hyperbolic systems, **where the speed of propagation is finite**, one can show that Schwarz waveform relaxation algorithms converge in a **finite number of steps**"

and on windowing:

> "convergence is very fast for $T$ small, and hence it is good to partition long time intervals into shorter so called time windows to apply the algorithm on each time window separately."

**The transfer, done carefully.** Your $\sigma$ is the *time window* $T$, your $d$ is the *overlap* $\delta$, and you take $k=1$. At $k=1$ the diffusive estimate reads $\mathrm{err} \propto \mathrm{erfc}\big(d/(2\sqrt{\nu\sigma})\big)$ — i.e. for a *diffusive* mechanism the right scaling is $d \sim \sqrt{\nu\sigma}$, and for a *hyperbolic/advective* one it is $d \sim v\sigma$ with finite-step (exact) termination. Your kernel is advective at the reach scale (an incision wave with a Courant speed), which is why §2.2's linear condition and not a square-root one is the one your data follows. **[mine]** If you ever seam a genuinely diffusive system (hillslope creep, thermal), expect the exponent to change — that is a concrete, cheap prediction the same probe would test.

Two further transfers worth holding:

- **Optimized SWR** replaces Dirichlet transmission with Robin/higher-order operators $\mathcal B_{ij}(u^k_i) = \mathcal B_{ij}(u^{k-1}_j)$ and then "converge very rapidly, independently of the mesh parameters … **there is no need for a coarse grid**" and "converge even without overlap" [verified, same source]. **[mine]** The vivarium reading: what you exchange across a seam is currently a raw Dirichlet datum (bed elevation). Exchanging a *combination* — elevation plus its normal gradient, or elevation plus the crossing discharge — is the classical route to the same fidelity at smaller $d$. That is an interesting convergence with `#form-seam-flux-exchange` FE(2): the flux is not only the second repair, it is plausibly also the transmission condition that makes the first repair cheaper. I would not claim more than "the shapes rhyme"; optimized-Schwarz parameter optimisation is a Fourier-symbol calculation your nonlinear kernel does not admit.
- **Nonlinear/chaotic reality.** SWR theory is stated for linear (or Lipschitz-nonlinear) problems with a well-posed fixed point. Your FE(2) chaos floor — $1\,\mathrm{mm} \to 395\,\mathrm{m}$, $96.6\%$ decorrelation — means there is *no* fixed point at cell resolution to converge to, and any imported estimate must be read in structural norms only. Your segments already say this; the literature does not soften it.

### 2.5 Jacobi vs Gauss–Seidel is additive vs multiplicative Schwarz, and the DD literature ratifies FE(3) exactly **[verified/reported]**

- **Additive Schwarz** applies all subdomain corrections from one frozen iterate (your Jacobi); **multiplicative Schwarz** uses the latest available (your Gauss–Seidel), converges roughly twice as fast, and is inherently sequential — the standard remedy in the literature being **multicolouring** of subdomains to recover parallelism [reported; this is textbook, Toselli & Widlund 2005, *Domain Decomposition Methods — Algorithms and Theory*, Springer Series in Computational Mathematics 34, verified as a book but I did not open its interior]. Your FE(3) admits exactly the red–black variant the literature prescribes, arrived at independently. That is ratification, not new information — but it does mean the design's one apparent oddity ("why cripple ourselves with Jacobi?") has a fifty-year-old answer and a named escape hatch.
- **Restricted Additive Schwarz** (Cai & Sarkis, *SIAM J. Sci. Comput.* 21(2):239–247, 1999) [verified citation] is additive Schwarz where each subdomain writes back only its *owned* (non-overlap) cells — which is precisely FE(2)'s "interiors are owned: a tile never writes another tile's cells." RAS is the field's default preconditioner and converges *faster* than plain AS while communicating less. Your ownership rule is RAS, and it is worth citing under that name.
- **Asynchronous Schwarz is the one thing you must not adopt.** There is a real convergence theory for exchanging whenever data happens to arrive (Chazan & Miranker 1969, chaotic relaxation *(unverified detail)*; Frommer & Szyld 2000, "On asynchronous iterations" *(unverified detail)*; Magoulès, Szyld & Venet, "Asynchronous optimized Schwarz methods with and without overlap," *Numer. Math.* 137:199–227, 2017 [verified citation]). It guarantees the **limit** is order-independent; it says nothing about the **iterates**. `#form-depend-by-key-never-latest` needs bit-identity of iterates, and you have no limit. So the honest note for a future agent tempted by async is: *the theory that permits it guarantees a property you cannot use.*
- The nearest published relative of "lagged halo data, quantified" is the **asynchrony-tolerant finite-difference** line (Donzis & Aditya, *J. Comput. Phys.* 2014; Aditya & Donzis, *J. Comput. Phys.* 350:550, 2017 *(unverified page detail)*): stability survives stale halo data but **accuracy is greatly degraded**, and they derive corrected stencils that recover formal order under delay [reported]. Structurally interesting as the only place anyone has priced staleness; not directly usable, because their correction is a Taylor-series repair of a linear stencil.

### 2.6 The one-shot-predictor family, and why it is the cautionary tale **[reported]**

If you want the literature that is *actually* "one Schwarz sweep per window, then advance," it is the explicit/implicit non-iterative DD line: Dawson, Du & Dupont (finite-difference DD for the heat equation, *Math. Comp.* 1991) and Dawson & Dupont (explicit/implicit conservative Galerkin DD, *Math. Comp.* 1992) *(unverified details)*. Interface values are predicted **explicitly** from lagged data; the interiors are then solved implicitly. The field's summary of it is uniform and blunt: **the method is only conditionally stable**, and a whole subsequent literature (EPIC, stabilised explicit-implicit DD, Lagrange-multiplier variants) exists to add a *correction* pass that updates the interface after the interiors are known.

**[mine]** Two things fall out. First, this is the theoretical home of "predict the boundary once, march, repeat," and its verdict is *conditional stability with a step-size restriction relating window length to interface geometry* — the same shape as $d \ge v\sigma$ arriving from a different direction, which is mild corroboration. Second, the field's repair — *correct the interface after the interiors exist* — is a predictor–corrector, and your Jacobi round is a predictor with no corrector. Whether a corrector is worth it for you is genuinely open; I flag it because "add a corrector" is the standard next move and it is not currently on your list.

### 2.7 The coarse space, and what it says about $\rho$ **[reported, mapping is mine]**

Classical result: one-level overlapping Schwarz has $\kappa = O(H/\delta)$ and **does not scale in the number of subdomains** — information travels one subdomain per iteration, so global coupling costs $O(1/H)$ iterations. Adding a **coarse space** gives $\kappa \le C(1+H/\delta)$, uniform in the subdomain count [reported; Dryja & Widlund; Toselli & Widlund 2005 ch. 3].

That "one subdomain per iteration" *is* your cone: FE(5)'s "one round of exchange spreads influence one tile outward." So your cone is not a peculiarity of erosion; it is the known and only failure mode of one-level Schwarz, and the field's answer to it is fifty years old and is **not** a bigger halo. It is a second, coarse level that carries global information in one step.

**[mine, and this is the recommendation I am least sure of but think is worth a spike.]** You have a coarse level already: the nomos is multi-level, and a coarse carve of the same region *is* a coarse-space representation of the same field. A two-level exchange — halo from same-level neighbours, plus a correction from the parent level's carve — would be the literal DD construction. What it would buy is exactly what $\rho$ costs you: with a coarse correction, truncating the cone at $\rho$ tiles no longer severs global information, because the global part arrives through the parent. That reframes $\rho$ from "a declared deficiency we will price" to "the radius beyond which the coarse level takes over," which is a much better thing for it to be. Two honest caveats: (i) the coarse-space theory is for *preconditioners*, and you have no solver to precondition, so this is an architectural analogy, not a theorem; (ii) `#obs-mean-pin-manufactures-seam` already convicted one naive cross-level state restriction, so the coarse correction would have to ride on a *flux* (which restricts honestly, `#form-face-flux-register` FE(3)) and not on raw state — which is your own FE(8) saying the same thing from the other side.

### 2.8 Where the analogy is broken, stated as a no-go

Three premises of every result in §2.4–2.7 fail for you, and naming them is worth more than the transfers:

1. **No fixed point.** Schwarz theory measures distance to *the* monolithic solution. Your FE(2) chaos floor says that solution is not pointwise reachable by anyone, including a single-field carve against itself plus a millimetre. Every imported estimate is admissible only in structural norms.
2. **The operator is not local.** Priority-Flood and accumulation are global within a domain; Lieb–Robinson/CFL/ghost-zone theory all assume a bounded domain of dependence. §2.2's residual is the price, and §2.3 suggests it behaves as if the premise nearly held — but "nearly" is a measurement, not a theorem.
3. **Order statistics defeat the whole family.** No overlap-based method — Schwarz, ghost-zone, SWR, optimized or not — can repair a quantity that is a minimum over an unbounded set. This is your FE(9) and the ASF spike's Result 5; the DD literature's own answer is §2.7's other branch, namely **mortar** (give the interface its own owned space), which the spike already landed. I searched for a Schwarz-with-order-statistics result and found nothing; that is a *not-found*, not a *does-not-exist*.

---

## 3. Q2 — cross-subdomain flux for landscape evolution: solved, three times, and not with a halo

This is where the literature is richest and where I think it most changes your build order.

### 3.1 Family A — two-pass with a global summary graph (Barnes) **[verified, primary read]**

**Barnes, R. (2016), "Parallel Priority-Flood depression filling for trillion cell digital elevation models on desktops or clusters," *Computers & Geosciences* 96:56–68, doi:10.1016/j.cageo.2016.07.001** (arXiv:1606.06204).

The mechanism, read from the primary:

- The DEM is tiled; "any two adjacent tiles must share the entire length of their adjoining edges" (so, your matching same-level seam). **No halo. No overlap. No cadence.**
- Each tile runs Priority-Flood locally, labelling each cell with a **watershed** and recording, for every pair of watersheds that meet, the lowest elevation at which they meet: "Cumulatively, all of the spillover points form a **spillover graph** connecting watersheds together."
- Each tile sends the master exactly three things: "(a) the elevations of each cell on all four edges of the tile, (b) the labels of each cell on all four edges of the tile, and (c) the tile's spillover graph. … The amount of information sent is therefore **proportional to the length of the tile's perimeter and its number of watersheds**; all of this information is sent only once per tile."
- The master joins adjacent tiles' edges into one **master graph** and runs Priority-Flood *on the graph*, producing per-watershed global spill elevations, which are broadcast back; each tile raises its cells accordingly.
- Structure: "This design is effectively **two sequential MapReduce operations**." Exactly two communication rounds, fixed, independent of geometry.
- Correctness is tested rather than proved, and tested in exactly the way you would want: "A large number of different tile dimensions are tested to ensure that the results of the new algorithm agree with the authoritative answer **independent of the tile dimension used** … did not show any deviation from the authoritative answer."

**Companion:** **Barnes, R. (2017), "Parallel non-divergent flow accumulation for trillion cell digital elevation models on desktops or clusters," *Environmental Modelling & Software* 92:202–212, doi:10.1016/j.envsoft.2017.02.022** (arXiv:1608.04431) — the same two-pass shape applied to accumulation, "guarantees a fixed number of memory access and communication events per raster cell" [abstract verified; I did not read its interior, so the per-tile summary object is **[reported]** to be perimeter-dependency links rather than a spillover graph].

**Why this matters to you.** This is your FE(9) "per-basin scalar exchanged beside the halo," built, published, and exact — and it is *both* of your open objects at once: the same perimeter object carries the spill level (the non-local datum) and, in the 2017 companion, the crossing discharge (the flux). The summary is $O(\text{perimeter} + \text{watersheds})$, which is the same $O(1)$-in-content, coordinate-determined shape your key argument needs. **[mine]** The one thing Barnes does not have and you do is *keying*: he recomputes; you would memoise the seam object under a complete key. That is your addition, and it is not small — a keyed spillover graph is the mortar object the ASF spike's Result 6b argues for, with a store behind it.

### 3.2 Family B — make accumulation a linear system and let a solver do the decomposition **[verified, primary read]**

**Richardson, A., Hill, C. N. & Perron, J. T. (2014), "IDA: An implicit, parallelizable method for calculating drainage area," *Water Resources Research* 50, doi:10.1002/2013WR014326** *(page range unverified; publisher page returned 403, so this is [reported] via Bangerth's primary citation of it and the publisher listing).* Flow routing is recast as a sparse linear system $A\mathbf w = \mathbf r$ (each node gives its water to one or more downhill neighbours, so $A$ has $\le 2$ entries per column in D4/D8), solved with standard parallel Krylov machinery. Bangerth credits independent discovery of the linear-system formulation to Eddins (2007) and Schwanghart & Kuhn (2010) as well [reported, via Bangerth].

**Bangerth, W. (2026), "Massively parallel flow routing and drainage area determination," arXiv:2606.12800 [math.NA], 11 Jun 2026** [verified, primary read] — this is the single most relevant paper I found, and it is seven weeks old.

The construction, in your terms:

- Partition the domain. Each process **renumbers its own nodes high-to-low**, making its diagonal block triangular; the off-diagonal blocks are exactly the cross-partition inflows.
- Preconditioner $B = (D^{pp\downarrow})^{-1}$: applying it "corresponds to applying the high-to-low procedure … on the nodes that are owned by a process" — i.e. **one ordinary serial drainage sweep per tile, in parallel, with no knowledge of neighbours**.
- The Richardson iteration is then $\mathbf w^{(\ell+1)} = D^{-1}(\mathbf r + R\,\mathbf w^{(\ell)})$, which Bangerth glosses: "$\mathbf r + R\mathbf w^{(\ell)}$ … describes the amount of water each node owned by the current process receives either through rain **or from neighbouring uphill off-process nodes**. The subsequent application of $D^{-1}$ then transports this water downstream."

**That is your Jacobi halo exchange.** Same object, same round structure, one exchange of a boundary vector per round — except that what crosses is the *water arriving from off-tile*, i.e. the **flux**, not the state. And then comes the result that I think is the most useful single sentence in this dossier:

> "if the domain contains streams that cross boundaries between subdomains $C$ times, then it takes $C+1$ iterations to get the amount of water correct on all processes that have part of this stream: The first iteration from the headwaters to the first subdomain boundary crossing; the second iteration from that point to the second subdomain crossing; and so on."

**This is your cone, proved, with a termination guarantee.** FE(5) says "one round of exchange spreads influence one tile outward" and calls it *derived, not measured*. Bangerth's argument is the derivation, in a setting where the object is exact rather than approximate: the cone radius needed is the number of **stream crossings**, not the number of tiles, and after $C+1$ rounds the answer is **exact, not converged**. Two consequences for you: (a) $\rho$ (cone truncation) acquires a physically meaningful setting — the maximum number of times a drainage crosses tile seams within the region — which is typically far smaller than the tile count and is *measurable from your own drainage network*; (b) the $\Theta(N^3)$ isolated-tile cost in FE(5) is an upper bound stated in rounds, and the effective $N$ is a property of the geography, not of the schedule.

Also worth holding: on one process the method is *identical* to the classical high-to-low sweep and costs the same; and Bangerth names the framing himself — "the method described above can be interpreted in the spirit of **domain decomposition** approaches … in which the subdomain solvers (here: the high-to-low solver for each partition) are exact" — and then names the open door: "could in principle be used to develop better methods if one could come up with **interface transmission conditions** that allow for building solvers that work on the **skeleton** of the domain decomposition." **[mine]** That last clause is, almost verbatim, the mortar/first-class-seam programme of the ASF spike §9.1, written by a numerical analyst who does not know he is describing your design. If any single sentence in the literature ratifies the seam-as-owned-object direction, it is that one.

### 3.3 Family C — serial master, and the hybrid in the wild **[verified]**

**Salles, T. (2019), "eSCAPE: Regional to Global Scale Landscape Evolution Model v2.0," *Geoscientific Model Development* 12:4165–4184, doi:10.5194/gmd-12-4165-2019.** A parallel global LEM on an unstructured *spherical* mesh (which makes it the closest published thing to your problem geometry). It solves drainage area as a **global implicit linear system via PETSc** (Family B, Richardson iteration with block-Jacobi preconditioning) — but **pit filling is not parallelised**: "the priority-flood algorithm … is performed on the master processor" and then broadcast, which the paper itself names as its dominant bottleneck [verified via the article page]. Successor: **gospl** (Salles et al., *JOSS* 2020, doi:10.21105/joss.02804) [reported].

**[mine]** The split in eSCAPE is exactly your two objects, and the field's revealed preference is instructive: the *flux* half (accumulation) parallelises cleanly as a linear system; the *datum* half (depression/spill) was hard enough that a production global model runs it serially on rank 0 and eats the cost. That is a strong external signal about which of your two open builds is the harder one — and it is the one you have not started.

### 3.4 Family D — iterate the halo to a fixed point (and note nobody uses a fixed cadence) **[verified, primary read]**

TauDEM's lineage (Survila, Yıldırım, Li, Liu, Tarboton & Wang, "A Scalable High-performance Topographic Flow Direction Algorithm for Hydrological Information Analysis," XSEDE16) resolves flats and flow directions by repeated ghost-zone exchange: "MPI collective communication happens during each iteration to retrieve the neighbouring cells not located on the process"; flats crossing a partition boundary go on a `borderingFlatsList` and "need interprocess communication" — iterated until nothing changes. Earlier: Wallis, Wallace, Tarboton, Watson, Schreuders & Tesfa (2009), reported as handling cross-partition calculations "in an efficient and **order-independent** manner" [reported, from a search summary — I did not obtain that primary].

**[mine]** Note the pattern across all four families: **when a halo is used at all, it is iterated to a fixed point within one time step, never advanced at a fixed cadence.** I could not find a single landscape-evolution or hydrology code that runs a fixed $\sigma$ and accepts the residual. Your $(d,\sigma)$ operating point is, as far as this search reaches, **without precedent in this field** — which is not a criticism (your constraints are different: content-addressed keys, no global barrier, memoised tiles) but is worth knowing before anyone writes "standard practice."

### 3.5 The spill-level object exists and is fully worked out **[verified]**

**Barnes, R., Callaghan, K. L. & Wickert, A. D. (2020), "Computing water flow through complex landscapes – Part 2: Finding hierarchies in depressions and morphological segmentations," *Earth Surface Dynamics* 8:431–445, doi:10.5194/esurf-8-431-2020.**
**Barnes, R., Callaghan, K. L. & Wickert, A. D. (2021), "Computing water flow through complex landscapes – Part 3: Fill–Spill–Merge: flow routing in depression hierarchies," *Earth Surface Dynamics* 9:105–121, doi:10.5194/esurf-9-105-2021.**

The **depression hierarchy** is "a forest of binary trees" over nested depressions; each node carries the depression volume ("the total volume of water that the depression, including all of its descendants, can contain before spilling over"), its current water volume (a parent holds water only when both children are full), its spill relationship, plus **geolinks** (routing between geographically adjacent leaf depressions inside a meta-depression) and **oceanlinks** (one-way routing of overflow downstream). **Fill–Spill–Merge** then routes runoff over that structure: fill, spill into the neighbour, and **merge** when both fill. The paper reports exact answers and 90–2600× speedups over "the commonly used Jacobi iteration" [verified via the article page]. Explicitly **not parallelised**: "flow accumulation algorithms can be parallelized but for simplicity we do not use these techniques here."

**[mine]** Read against your FE(9): your "per-basin scalar exchanged beside the halo" is under-specified in one respect this literature has already found the hard way. A straddling basin does not have *a* spill level; it has a spill level **per level of the hierarchy**, and the identity of the relevant node changes as water rises and sub-basins merge. So the seam object is not one number per straddling basin — it is a small tree, or a number plus the merge event that would change which number applies. That is not a reason to shrink the ambition; it is the shape the object actually has, and it is published. And your FE(9)'s "does not decay with $d$; it jumps when the true rim low enters the window" is precisely the merge event in their vocabulary.

### 3.6 The clean negative: nobody does refluxing for landscape evolution

I searched for conservative interface flux correction (Berger–Colella-style refluxing, mortars, flux registers) applied to landscape evolution, surface-process, or drainage models — across the LEM literature (FastScape/Braun–Willett lineage, Landlab, CHILD, eSCAPE/gospl, Badlands), the AMR-for-geomorphology space, and the parallel-hydrology space. **Not found.** The AMR refluxing literature is entirely fluid/astro/climate; the LEM literature parallelises by shared memory (Barnes 2018, arXiv:1803.02977, "Accelerating a Landscape Evolution Model with Parallelism" — GPU/SIMD/OpenACC parallelisation of the Braun–Willett stack, *not* domain decomposition [verified, primary read]) or by the three families above.

So `#form-face-flux-register`'s application to a fluvial/sediment interface appears to be **unoccupied**, and your two-object split (datum vs flux, FE(1)) is not a distinction the LEM field has drawn — because in a two-pass or linear-solve formulation the two are computed by different phases anyway and nobody had to name the split. **[mine]** That is a modest novelty claim and I would state it as "unfound across the searched surface," not "does not exist" — my searched surface for the negative is: LEM/geomorphology parallelisation, hydrological terrain processing, AMR conservative coupling, spherical Earth-surface models. I did not search the coastal/morphodynamics or the reservoir-simulation literatures, and the latter in particular (compositional flow with fluxes across faults) is where I would look next if the negative mattered enough to harden.

---

## 4. What I think this changes, marked as recommendation

Clearly labelled: these are my calls, not the literature's.

### 4.1 The cheapest thing here — a $d/(v\sigma)$ probe, and it reuses an existing instrument

`examples/halo_exchange_probe` already sweeps $(d,\sigma)$. Adding arms at **matched ratios** — e.g. $(d{=}4,\sigma{=}5)$ and $(d{=}16,\sigma{=}20)$ against $(d{=}8,\sigma{=}10)$, all at $d/(v\sigma) = 0.8$; and $(d{=}8,\sigma{=}5)$, $(d{=}32,\sigma{=}20)$ against $(d{=}16,\sigma{=}10)$ at $1.6$ — costs one probe run and either (a) collapses the two dials into one derived law, upgrading FE(6) from measured-operating-point to derived, or (b) refutes §2.3 as a six-point coincidence. Either outcome is worth having, and the refutation is worth as much as the confirmation because it would mean the effective front speed is not $\approx 1$ cell/epoch and your cone arithmetic needs a measured $v$. **The same probe measures $v$**: the ratio at which the error collapses *is* $1/v$.

Two arms that sharpen it further: a $(d = 0, \sigma = 1)$ arm (exchange every epoch, no overlap — the ratio-zero limit, which the theory says should be poor and which is cheap), and a very large $d/(v\sigma) \gg 3$ arm to locate the floor precisely. If the floor sits at the chaos floor, the non-local residual is small; if it sits well above, the spill-level defect is large and FE(9) moves up the queue. **That is a measurement of the FE(9) defect size that costs no new machinery**, and FE(9) currently says its size "is unknown rather than small."

### 4.2 Build the flux half as a Barnes-shaped seam object, not as a halo augmentation

Your Working Note frames the flux as "a per-seam, per-stage flux record, keyed beside the stage's residual." The literature agrees on the object and disagrees on the *round structure*: Barnes gets it exactly in two passes (perimeter summary → master graph → broadcast); Bangerth gets it exactly in $C+1$ Jacobi rounds where $C$ counts stream crossings. Both are cheaper than what you would get by deepening $d$, and both produce decomposition-independent answers, which is the property `#form-depend-by-key-never-latest` actually wants.

**[mine]** The interesting design question this raises, which I do not think your segments have asked yet: **is the seam object's dependency a fixed point or a fixed round count?** Barnes' master step is a global reduction — it needs the whole region, which is a keying problem (your cone becomes the whole block in one step). Bangerth's is local and iterative — $C+1$ rounds of neighbour exchange, each keyable exactly like your halo rounds already are. **Bangerth's shape is the one that fits your key discipline**, and the price is that $C$ (hence $\rho$) is geography-dependent rather than schedule-dependent. That is a *better* kind of dependency for you: it is a function of the terrain, which is fated, which means it is a function of the seed and coordinates, which means it is keyable.

### 4.3 The spill scalar is a tree, and it is published

§3.5. If FE(9) is built as one scalar per straddling basin it will be wrong in the merge case, and the merge case is generic at seams (a seam cuts basins at all scales). Building it against the depression-hierarchy shape from the start costs reading two papers.

### 4.4 Cite Bangerth 2026 in the halo segment's Epistemic Status

FE(5)'s cone arithmetic is currently "derived from the mechanism and not measured." Bangerth's $C+1$ argument is an independent derivation of the same thing, from a different premise set, in a setting where it is exact. That does not make your FE(5) measured, but it does make it *externally corroborated*, and the convergence of two independent derivations is the kind of evidence the charter §7 note treats as signal.

---

## 5. Feedback on the brief, offered because you asked

**The two-question split was right and the framing of Q1 was subtly wrong**, in a way that turned out to be productive. Q1 asked whether DD theory can "principle or convict" $(d,\sigma)$ — which presumes the relevant DD object is a convergence rate. The productive move was to ask *which register the scheme is in* first, and the answer (overlapped tiling, not Schwarz iteration) changed which literature mattered and produced §2.3. If the brief had specified "find the Schwarz convergence result for lagged boundary data," I would have found Gander's erfc estimate, matched $\delta \to d$ and $T \to \sigma$, and handed you a diffusive scaling law $d \sim \sqrt{\nu\sigma}$ that is **the wrong exponent for your kernel** and would have looked authoritative. The brief's openness is what prevented that; I want to name the specific near-miss because it is the same failure the AGENTIC-DELEGATION ledger records from the χ-criterion probe.

**The "artifact wins over the brief" rule paid twice.** `#obs-exchange-repairs-the-seam-and-overlap-does-not` FE(7) contains the entire empirical content of §2.3 — the brief's summary ("d=16, σ=10 at one grain put our seam statistics onto a single-field carve's values") is true and would not have let me do the arithmetic, because it reports the *conclusion* and not the *table*. And the ASF spike's §8/§9 saved me from re-deriving the order-statistic no-go and from proposing mortar as if it were new.

**One thing I would have wanted in the brief that was not there, and that only you have:** the *stencil radius* and the *characteristic incision speed* as this session understands them. I recovered $v \approx 1$ cell/epoch by inference from FE(2)'s hundred-epoch front, and the ASF spike reports Courant figures of $0.02$–$0.11$ cells/epoch at L9 and $\approx 1$ at L13 with an explicit honesty note that they are second-hand. §2.3's whole arithmetic hinges on $v$ at L13 being $\approx 1$, and if it is not, the ratio column is wrong (though its *monotonicity* survives any constant $v$, since a constant rescales all six ratios together — so the ordering result is robust to $v$ and only the location of the threshold at 1 depends on it). Worth stating that separation explicitly if this gets integrated.

**A guess I could not close.** I did not obtain the IDA 2014 primary (publisher 403) and worked from Bangerth's reading of it. If the flux build goes the linear-system route, someone should read IDA directly — particularly for how it handles depressions, which is the place Bangerth's paper is thinnest (he assumes depression filling as preprocessing).

---

## 6. Citation ledger

| # | Citation | Status |
|---|---|---|
| 1 | Barnes, R. (2016). Parallel Priority-Flood depression filling for trillion cell digital elevation models on desktops or clusters. *Computers & Geosciences* 96:56–68. doi:10.1016/j.cageo.2016.07.001. arXiv:1606.06204 | **verified** — primary read (PDF), quotes above |
| 2 | Barnes, R. (2017). Parallel non-divergent flow accumulation for trillion cell digital elevation models on desktops or clusters. *Environmental Modelling & Software* 92:202–212. doi:10.1016/j.envsoft.2017.02.022. arXiv:1608.04431 | **verified** (abstract, publisher metadata); interior **[reported]** |
| 3 | Barnes, R. (2018). Accelerating a Landscape Evolution Model with Parallelism. arXiv:1803.02977 | **verified** — primary read; journal version (Geomorphology?) **unverified** |
| 4 | Barnes, Callaghan & Wickert (2020). Computing water flow through complex landscapes – Part 2: Finding hierarchies in depressions and morphological segmentations. *Earth Surf. Dynam.* 8:431–445. doi:10.5194/esurf-8-431-2020 | **verified** citation; content **[reported]** |
| 5 | Barnes, Callaghan & Wickert (2021). …Part 3: Fill–Spill–Merge: flow routing in depression hierarchies. *Earth Surf. Dynam.* 9:105–121. doi:10.5194/esurf-9-105-2021 | **verified** — article page read, quotes above |
| 6 | Bangerth, W. (2026). Massively parallel flow routing and drainage area determination. arXiv:2606.12800 [math.NA] | **verified** — primary read, quotes above |
| 7 | Richardson, A., Hill, C. N. & Perron, J. T. (2014). IDA: An implicit, parallelizable method for calculating drainage area. *Water Resources Research* 50. doi:10.1002/2013WR014326 | **[reported]** — publisher 403; author/year/title/DOI corroborated by Bangerth's citation |
| 8 | Salles, T. (2019). eSCAPE: Regional to Global Scale Landscape Evolution Model v2.0. *Geosci. Model Dev.* 12:4165–4184. doi:10.5194/gmd-12-4165-2019 | **verified** — article page read, quotes above |
| 9 | Salles, T. et al. (2020). gospl: Global Scalable Paleo Landscape Evolution. *JOSS*. doi:10.21105/joss.02804 | **[reported]** |
| 10 | Survila, K., Yıldırım, A. A., Li, T., Liu, Y. Y., Tarboton, D. G. & Wang, S. (2016). A Scalable High-performance Topographic Flow Direction Algorithm for Hydrological Information Analysis. *XSEDE16* | **verified** — primary read |
| 11 | Wallis, C., Wallace, R., Tarboton, D. G., Watson, D. W., Schreuders, K. A. T. & Tesfa, T. K. (2009). Hydrologic Terrain Processing Using Parallel Computing | **[reported]** — not obtained |
| 12 | Gander, M. J. *Waveform Relaxation*. Encyclopedia of Applied and Computational Mathematics, Springer. doi:10.1007/978-3-540-70529-1_336 | **verified** — primary read, quotes above |
| 13 | Gander, M. J. & Stuart, A. M. (1998). Space-time continuous analysis of waveform relaxation for the heat equation. *SIAM J. Sci. Comput.* 19(6):2014–2031 | **verified** citation |
| 14 | Gander, M. J. & Zhao, H. (2002). Overlapping Schwarz waveform relaxation for the heat equation in $n$ dimensions. *BIT Numerical Mathematics* 42 | **[reported]**; page range unverified |
| 15 | Toselli, A. & Widlund, O. (2005). *Domain Decomposition Methods — Algorithms and Theory.* Springer Series in Computational Mathematics 34 | **verified** as a book; interior **[reported]** |
| 16 | Cai, X.-C. & Sarkis, M. (1999). A restricted additive Schwarz preconditioner for general sparse linear systems. *SIAM J. Sci. Comput.* 21(2):239–247 | **verified** citation |
| 17 | Dryja, M. & Widlund, O. (1987). An additive variant of the Schwarz alternating method for the case of many subregions. Courant Institute TR | **[reported]**, standard; not obtained |
| 18 | Lions, P.-L. (1988). On the Schwarz alternating method I. *Proc. 1st Int. Symp. on Domain Decomposition Methods*, SIAM | **[reported]**, standard; not obtained |
| 19 | Magoulès, F., Szyld, D. B. & Venet, C. (2017). Asynchronous optimized Schwarz methods with and without overlap. *Numerische Mathematik* 137:199–227. doi:10.1007/s00211-017-0872-z | **verified** citation |
| 20 | Chazan, D. & Miranker, W. (1969). Chaotic relaxation. *Linear Algebra and its Applications* 2:199–222 | **[reported]**, standard; not obtained |
| 21 | Frommer, A. & Szyld, D. B. (2000). On asynchronous iterations. *J. Comput. Appl. Math.* 123:201–216 | **[reported]**; not obtained |
| 22 | Ding, C. H. Q. & He, Y. (2001). A ghost cell expansion method for reducing communications in solving PDE problems. *SC'01*. doi:10.1145/582034.582084 | **verified** citation; content **[reported]** |
| 23 | Meng, J. & Skadron, K. (2009). Performance modeling and automatic ghost zone optimization for iterative stencil loops on GPUs. *ICS'09*. doi:10.1145/1542275.1542313 | **verified** citation |
| 24 | Matsumura, K., Zohouri, H. R., Wahib, M., Endo, T. & Matsuoka, S. (2020). AN5D: Automated Stencil Framework for High-Degree Temporal Blocking on GPUs. *CGO'20*. doi:10.1145/3368826.3377904. arXiv:2001.01473 | **verified** citation; halo-size quote **verified** via search-result text, not from the PDF interior |
| 25 | Donzis, D. A. & Aditya, K. (2014). Asynchronous finite-difference schemes for partial differential equations. *J. Comput. Phys.* 274 | **[reported]**; page range unverified |
| 26 | Aditya, K. & Donzis, D. A. (2017). High-order asynchrony-tolerant finite difference schemes for PDEs. *J. Comput. Phys.* 350:550–572 | **[reported]**; volume/first page from ADS |
| 27 | Dawson, C. N., Du, Q. & Dupont, T. F. (1991). A finite difference domain decomposition algorithm for numerical solution of the heat equation. *Math. Comp.* 57 | **[reported]**; details unverified |
| 28 | Dawson, C. N. & Dupont, T. F. (1992). Explicit/implicit conservative Galerkin domain decomposition procedures for parabolic problems. *Math. Comp.* 58 | **[reported]**; details unverified |
| 29 | Eddins, S. (2007); Schwanghart, W. & Kuhn, N. J. (2010) — independent formulations of flow routing as a linear system | **[reported]** via Bangerth (2026) |

**Already held elsewhere and not re-verified here:** Berger & Oliger 1984, Berger & Colella 1989 (`#detail-seam-precedents`); Bernardi–Maday–Patera 1994, Wohlmuth 2000, Lieb & Robinson 1972, Lubachevsky 1989, Dobrushin/Künsch 1982 (ASF `spike-partition-isolation-criterion-2026-07-28` §10).

---

## 7. Searched-and-not-found, with the surface named

So a future reader knows where the unexplored volume is:

- **Schwarz/DD convergence theory for a scheme that advances rather than iterates** — the closest is the explicit/implicit non-iterative DD family (§2.6), which is conditionally stable and always adds a corrector. Not found: an error theory for "one sweep per window, then march, forever."
- **Any overlap-based method for order-statistic observables.** Not found. The DD answer is mortar (already held).
- **Refluxing / conservative interface flux correction applied to landscape evolution or sediment transport.** Not found across LEM, hydrological terrain processing, AMR-for-geomorphology, spherical Earth-surface models. Not searched: coastal morphodynamics, reservoir simulation across faults.
- **Any parallel LEM or hydrology code running a fixed exchange cadence and accepting the residual.** Not found — everything either iterates to a fixed point or is exact in a bounded number of passes.
- **Content-addressed / memoised seam objects anywhere in this literature.** Not found, and not surprising: nobody else needs a tile to be reproducible under an arbitrary demand order. This is the axis on which vivarium's version is genuinely not a re-implementation.
- **Not obtained, would change something:** the IDA 2014 primary (depression handling); the interior of Barnes 2017 (the exact per-tile flux summary object — the single most directly reusable design in this dossier); Wallis et al. 2009 (the order-independence claim).
