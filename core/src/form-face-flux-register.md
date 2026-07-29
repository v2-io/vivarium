---
slug: form-face-flux-register
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-seam-flux-exchange
  - form-rl-closure-algebra
  - form-grid-equiangular-staggered
---

# Face-keyed flux register (Berger–Colella at hanging nodes)

When a coarse tile abuts a finer neighbour, the shared boundary is a **hanging-node interface**: one coarse face is subdivided into several fine sub-faces. Conservation requires a **single-valued flux** on that interface — one object, applied once with opposite signs — not two independently computed boundary fluxes. That structure is the Berger–Colella **flux register** (mortar) on our quadtree.

## Formal Expression

1. **Non-matching interface.** Let tile $A$ at level $\ell$ share an edge with tile $B$ at level $\ell{+}k$ ($k \ge 1$). The geometric edge of $A$ is the union of $2^k$ (in 1-D along the edge; $4^k$ faces in a full 2-D refinement picture) sub-edges of $B$. The interface is **not** one face to one face.
2. **Single-valued flux.** The coupling object is a **flux on the face** (or on each sub-face of the fine side, with the coarse face flux defined as their length-weighted sum). It is stored once, keyed as a face object ( #form-grid-equiangular-staggered ), and contributes $+F$ to one cell and $-F$ to the other. **Two-sided flux** — each tile computing its own boundary flux from an interpolated neighbour state — is out of bounds for a conservation claim: the two sides can disagree, and the disagreement has nowhere to live.
3. **Coarse face as low-pass of fine sub-faces.** $$F_{\mathrm{coarse}} = \frac{\sum_i F_i\, L_i}{L_{\mathrm{coarse}}}$$ with length-weighted details $t_i$ satisfying $\sum_i L_i t_i = 0$. This is the **same multiresolution construction** as the cell store's area-weighted low-pass, applied to **1-chains (faces)** rather than **2-chains (cells)**. The divergence theorem relates them: $R_{\mathrm{cell}} \circ \mathrm{div} = \mathrm{div} \circ R_{\mathrm{face}}$ when both restrictions preserve the integral.
4. **When refluxing ceases (three joint conditions).** Literature (CARMEN / Bellotti lineage) and the hanging-node spike agree: a separate refluxing *correction pass* is unnecessary only if **all** hold:
   1. **exact integral-preserving decimation** (area-weighted cells; length-weighted faces);
   2. **leaf-only evolution** — a cell that has children is **derived**, never independently stepped over the same ground;
   3. **single-valued interface flux** (this segment's FE(2)).
   Violating (2) while keeping (3) still needs Berger–Colella-style correction: double-evolve is exactly why refluxing was invented. Vivarium today violates (2) and largely lacks (3) in production kernels.
5. **Conservation is structural.** Exact mass balance at a hanging node is a property of **applying one flux once**, not of a more accurate flux number. The same $F$ under two-sided application leaks; under single-valued application it does not.
6. **Relation to seam law.** #form-seam-flux-exchange says what may cross (flux, not raw state) and that injection $\neq$ refluxing. This segment names **how** the flux is *identified* at non-matching resolution: face register, not per-tile ghost recomputation. #form-rl-closure-algebra 's mean-pin is **injection-shaped** and is **not** this register; retiring mean-pin is a sibling move, not a substitute for face keys.
7. **Price named.** Defining $F_{\mathrm{coarse}} \equiv \sum F_i$ by construction ties summary faces to leaves for those quantities (**leaf-only** for summary roles, or an explicit register correction if double-evolve is retained). That bill is real; calling the identity free was retracted. Governance quantities (conserved inventories free of leaf detail) are the escape hatch named in the macro-tier two-roles proposal — not decided here.

## Epistemic Status

**Max attainable: exact** for the measured hanging-node conservation contrast and for Berger–Colella / mortar practice as prior art.

**Measured (`msc/spike-wavelet-store/` PROBE 7, closed box, coarse L19 | fine L20, real hanging node, conservative diffusion, no outlets):**

| Steps | Two-sided rel. mass drift | Single-valued rel. mass drift |
|------:|--------------------------:|------------------------------:|
| 1 | $1.41\times 10^{-8}$ | $4.21\times 10^{-16}$ |
| 10 | $1.50\times 10^{-7}$ | $5.61\times 10^{-16}$ |
| 100 | $1.88\times 10^{-6}$ | $5.61\times 10^{-16}$ |
| 1000 | $2.58\times 10^{-5}$ | $5.61\times 10^{-16}$ |

Two-sided **grows** (bias). Single-valued is **flat at machine epsilon**. Coarse $=$ length-weighted low-pass of fine sub-faces confirmed on six interface faces; detail pairs sum length-weighted to zero. (The table is a captured run; `RUN.txt`'s current print is a later re-based run of the same probe with the same qualitative verdict and different digits — cite the table, or re-run, not both.)

**The contrast holds across a genuine cube edge** (cross-face spike PROBE 4c: the same hanging node lifted onto a cube edge — single-valued $\le 3.2\times 10^{-15}$ at 10k steps; two-sided grows to $4.9\times 10^{-13}$), and the face-key ownership rule ( #form-grid-equiangular-staggered FE(2), lower `CellId`, global `Ord`) is verified cross-face-safe as a real `store.rs` citizen surviving save/reopen, with the per-face-local $(i,j)$ rule convicted as ambiguous (PROBE 3). **Sensitivity caveat** (PROBE 4b): on a *matching* symmetric cross-face seam, two-sided application also conserves to epsilon — the register's value on matching seams is the data-structure guarantee (one object, nowhere to disagree), not a numeric win; the numeric win appears at non-matching interfaces.

**Currently `robust-qualitative` as project law:** the measurement is exact under the probe's scheme; **production store/kernels do not yet face-key fluxes**; leaf-only adoption in production **remains open build work, not a closed order**. DECISIONS family `refluxing-ceases-to-exist-measured`, `the-seam-is-double-evolution-not-discarded-detail`, `wavelet-store-solves-the-representation-not-the-dynamics` (`:by claude`, `:status council-accepted`, 2026-07-24) all carry the **measurement and the reconciled one-design finding** as council-accepted; `refluxing-ceases-to-exist-measured`'s own council note is explicit that *adopting* the face-keyed store + leaf-only in production still rides with the tentative grid decision and the open macro-tier resolution. `flux-on-the-face-makes-refluxing-an-invariant` is `:status superseded` — its council note says to **cite the two reinstating entries** (`the-seam-is-double-evolution…`, `refluxing-ceases-to-exist-measured`) rather than this superseded entry directly; the design it named survives, its original without-a-bill framing does not. This segment promotes the **measured structure** and the **three conditions**, not a mandate to ship leaf-only tomorrow.

Stage `draft`.

## Discussion

Joseph described the subdivided-edge case before the literature name landed: coarser face equals sum of finer edge-center fluxes. That is the flux register. The spike's contribution was to **convict** two-sided BCs and to **price** the identity (leaf-only or correction — not "wavelets make seams free").

## Working Notes

- **Instrument:** `msc/spike-wavelet-store/` PROBE 7 + `RUN.txt`; face MRA dual of cell MRA in lit-notes.
- **Do not re-claim:** "storing state detail coefficients alone kills the seam" — false (Jensen; zero-physics mean-pin ridge). Representation identity ≠ dynamics commute.
- **Sibling:** #obs-mean-pin-manufactures-seam (PROBE 6 ridge/mass; injection-alone worse than pin-off). Macro-tier governance vs summary roles still open. Production face-key addressing in `store.rs` unbuilt.
- **The seam has two independent reasons to be broken** (Lipnikov–Morel–Shashkov 2004, read primary in the structure survey): the two-point flux is **non-convergent at a coarse–fine interface even on an orthogonal mesh** (the hanging-node intensity must enter the difference expression) — independent of the non-orthogonality inconsistency the grid report measured. The mimetic literature's dissolution: treat each cell as an independent polygon and the "hanging node" stops being special (any non-conformity without modifying the scheme); the **2:1 balance rule** is the standard constraint nothing here yet adopts or declines. Survey §3.2 carries the full passage.
- **Dual-home demote targets:** multiscale-seams §2.1 refluxing paragraph; ARCHITECTURE AMR row; DECISIONS long forms once this slug is cited.
