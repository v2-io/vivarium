# The missing term — derived

*Spike: principled router. 2026-07-13. `:by claude :status proposed` — nothing here is decided.*

**Epistemic tags, used literally.** **[P]** quoted from a primary read directly · **[D]** derived here, check it · **[M]** measured in this repo · **[me]** my inference, a claim not a result · **[⊘]** open.

---

## 0. The answer, stated once

Joseph's instinct — *"if there's some fuzziness, we're missing some terms or factors"* — is **correct**, and the missing factor is **not** the one my briefing hypothesised.

> ## **The routers do not have one missing term. They have a CONFLATION.**
>
> ## **`p` — the slope exponent — is applied to the DIRECTIONAL slope `(h_K − h_L)/d_KL`. That single choice makes one number do two independent jobs: it selects WHICH neighbour gets the mass (a direction), and it sets HOW CONCENTRATED the flow is (a scalar). The two are separately meaningful and jointly inseparable in that form.**
>
> ## **The consistent split is published, and it is not the one I guessed: flow concentration belongs on the magnitude of the CELL'S RECONSTRUCTED GRADIENT, `‖∇h‖^{p_w}` with `p_w = q − 1` — a SCALAR, per cell. Direction comes from the gradient vector. Neither may be smuggled into the other.**
>
> **A router that raises the directional slope to a power `q ≠ 1` is INCONSISTENT — not inaccurate, inconsistent — and no post-processing can repair it until `q` is put back to 1.** **[P]**

**The briefing's hypothesis — that the missing constraint is the SECOND MOMENT (angular variance) of the outflow weights, and that it must match the plan curvature `k_c` — is refuted on two independent grounds, below (§3). The briefing's *meta*-prediction — "`p` has been doing two jobs, badly, at once; split them and both may become principled" — is exactly right.** It named the disease and mis-named the organ.

---

## 1. What the algorithm is actually claiming (the Prime Question, answered by the literature)

**[P] Coatléven & Chauveau 2025**, *A post-processing to restore numerical consistency for the most classical multiple flow direction algorithms* (Comput. Geosci., DOI `10.1007/s10596-025-10359-5`; read from the authors' HAL preprint of record, `refs/coatleven-chauveau-2025-CorrectedMFD-HAL-preprint.pdf`, 40 pp, full text).

Their structural theorem — their eq. (8) — is the load-bearing one. Discretise the stationary Gauckler–Manning–Strickler water-mass-conservation law with a two-point-flux finite volume + upwinding, define

- `τ_KL = |σ|·k_{m,σ} / (d_KL · s_ref^{−p_w}) · ‖G_{s,σ}‖^{p_w}` — the face transmissivity,
- `s_K = Σ_{σ downhill} τ_KL (h_{s,K} − h_{s,L})` — the cell's total downhill transmissivity,
- `q̃_K = s_K · h_{w,K} · η_w(h_{w,K})`,

and the FV scheme collapses **exactly** onto

```
  q̃_K  −  Σ_{L upstream}  τ_KL (h_{s,L} − h_{s,K}) / s_L  ·  q̃_L   =   |K| · S_{w,K}
           └──────────────── this coefficient IS the MFD weight ────────────┘
```

which **is** the MFD linear system, solved by the usual elevation-sorted downhill sweep. Set `k_m = 1, p_w = 0, B_w = 0` and it **is** classical MFD, identically.

> ### **[P] ⇒ MFD IS NOT A QUADRATURE. IT IS A TWO-POINT-FLUX FINITE-VOLUME SCHEME FOR A DIVERGENCE-FORM CONSERVATION LAW, AND `q̃_K` — THE ACCUMULATED SCALAR — IS AN *INTERMEDIATE ALGEBRAIC UNKNOWN OF THAT SCHEME*, NOT AN APPROXIMATION OF ANY PHYSICAL QUANTITY.**

This supersedes the project's standing answer (*"MFD is a quadrature and the 8 cells are its nodes"* — `CLAUDE.md`, `discretisation-and-information.md` §0). The quadrature reading is *true but not the sharpest form*, and the sharper form is what makes the correction visible. (`DECISIONS[mfd-is-the-two-point-flux-and-our-router-is-not-d-infinity]` already recorded this; the 2025 paper is now read and confirms it.)

### 1.1 [P] And what `q̃_K` converges to is ZERO

> *"since the quantity `q̃_K` approximates the outflux of a cell it is proportional to the perimeter of a cell, **the only convergence that can be expected for `q̃_K` is to zero**, while the behavior of `q̃_K/w(K)` will strongly depend on the choice of the normalization `w(K)`."* **[P]** §2.4

**[D] Check it, because it sounds wrong and is not.** `A_K` = the contributing area draining *through cell K*. Refine the mesh and the cell gets narrower, so the strip of catchment draining through it gets narrower with it: `A_K ≈ a(x)·W(K) → 0`, where `a` is the specific catchment area (the finite limit) and `W` is a length. **The contributing area of a *fixed geographic basin* is O(1); the contributing area of *a cell* is O(h). These are different objects and the literature routes one into the other.**

**⇒ Every `w(K)` normalisation in the literature — Desmet–Govers' cell diameter, Pelletier's effective flow length, Quinn's contour length — is an attempt to divide that perimeter back out with a SCALAR. [P] The paper's claim is that no scalar can do it,** because the correct divisor depends on the *flow direction relative to the cell's face geometry*, and the fix is to reconstruct the **vector**.

---

## 2. The correction — and the fact that we already had it

**[P] The reconstruction (their eq. 12–13):**

```
  Q_K  =  (1/|K|) · Σ_σ  F_{K,σ} · (x_σ − x_K)                  x_σ = FACE centre
  q_K  =  ‖Q_K‖                                                  ← "the correct output of a MFD algorithm"
```

where `F_{K,σ}` is the **signed** outward flux through face σ (their (12) writes the downhill and uphill face sets as two sums with opposite signs).

**[D] Why it is exact, and on what meshes.** It rests on their eq. (6), the divergence theorem applied to the coordinate functions:

```
  |K| · Id  =  Σ_σ  |σ| · (x_σ − x_K) ⊗ n_{K,σ}
```

For a **constant** vector field `Q`, `F_σ = |σ|·(Q·n_σ)`, so `Σ_σ F_σ (x_σ − x_K) = [Σ_σ |σ|(x_σ−x_K)⊗n_{K,σ}]·Q = |K|·Q`. **QED, on any polygon.**

> ### **[D] ⇒ THE RECONSTRUCTION IS A PURE GEOMETRIC IDENTITY. IT NEEDS NO ORTHOGONALITY.**
>
> The paper's orthogonality requirement (their eq. 4) is needed for the **two-point flux estimate** `F_σ ≈ τ(h_K − h_L)`, **not** for the reconstruction. Our cube-sphere quads are non-orthogonal — but we do not need TPFA: `grid_lab`'s LSQ gradient is precisely what Coatléven 2020 (M2AN, Def. 4.2) calls a *strongly consistent gradient reconstruction operator*, which is the hypothesis his convergence theorem actually takes. **The theory transfers; the TPFA does not, and we were never going to use it.**
>
> ⚠ **This is a load-bearing claim and it has a control: the identity is EUCLIDEAN, and our cells are SPHERICAL. Its residual on a spherical polygon is not zero and must be measured, at coarse levels especially** (`MEASUREMENTS.md`; the tentative-grid decision already records that the coarse tier is where the sphere is brutally real — sagitta 24% of the cell at L2).

### 2.1 ⚠ [D][M] But it is NOT a new term for the router table — and the probe's own code says so

`crates/vivarium-world/examples/curl_probe/flow.rs:746–761` — the cone probe **already** normalises by the **direction-dependent contour width**

```rust
let fhat = scale(tangent(g.centers[i], pole), -1.0);   // the EXACT downslope direction
let mut wid = 0.0;
for e in &g.adj[i] {
    let nh = tangent(g.centers[i], tangent(mid, e.normal));
    wid += e.edge_len_m * dot(nh, fhat).max(0.0);       // W(v̂,K) = Σ|σ|·(v̂·n̂)⁺
}
let err = ((acc[i] / wid) / exact - 1.0).abs();
```

**[D] And `‖Q_K‖` reduces to exactly `A_K / W(v̂,K)` in the constant-flux limit.** For a constant field `a·v̂`: the outflux is `q̃_K = Σ_{out} F_σ = a·Σ_{out}|σ|(v̂·n̂_σ) = a·W(v̂,K)`, so `q̃_K/W = a`; and the vector reconstruction returns `a·v̂` by the identity above. *(Worked both ways on a square cell — the influx faces, which carry the opposite sign AND the opposite lever arm, supply the factor of 2 that a naive outflux-only reading loses.)*

> ### ⚠ **⇒ THE HEADLINE TABLE'S CONE ERRORS ARE ALREADY CONTOUR-WIDTH-CORRECTED. Coatléven's post-processing, applied to the ROUTER COMPARISON, is a no-op — it is the normalisation `grid_lab` already performs, and with a *more generous* denominator (the analytic direction, not the router's estimate).**
>
> **The residual 23.16% / 4.68% / 20.26% / 8.09% are therefore REAL errors in the accumulated mass `A_K` itself. They are not normalisation artifacts, and the highest-value unread document does not dissolve them.**

**I record this as a negative result because I went looking for the opposite.** The briefing's top-priority item — *"get Coatléven 2025 and price it before you build anything; it is plausibly the missing term, already derived"* — is now **priced: for the router table, it buys nothing.** The reason is that a previous agent had *already*, independently, implemented the correct direction-dependent width in the probe (and, notably, wrote a comment explaining precisely why the naive outflow-edge sum double-counts). **The project got there first and did not know it had.**

### 2.2 ⇒ Where Coatléven DOES bite: `erosion.rs`, hard

The probe has the width. **The kernel does not.**

```rust
// crates/vivarium-world/src/erosion.rs:371, accumulate_drainage()
let cell_area = self.cell_m * self.cell_m;          // ← ONE uniform area, whole tile
...
let w = (drop / dist).powf(P);                      // ← P = 1.1, on the DIRECTIONAL slope
let dist = if diag { self.cell_m * SQRT_2 } else { self.cell_m };   // ← flat-grid metric
```

Three convicted constants in one function, and they are **not** three independent bugs — they are three faces of one omission: **the kernel was written for a flat, uniform, square grid and the metric was never carried.** (`cell_area` is already logged: `DECISIONS[drainage-area-uses-a-uniform-cell-area]`, +17.8% area-weighted mean bias, bit-identical L5→L13.)

---

## 3. ⛔ The briefing's hypothesis, refuted — and why the refutation is worth more

**The hypothesis under test:** *"Nothing pins the SPREAD. The second moment — the angular variance of the outflow distribution — is what sets how fast `a` diffuses or concentrates, and it must match the local plan curvature `k_c`."*

### 3.1 [D] Refutation 1 — `k_c` is delivered by the FIRST moment, not the second

Expand the law along a flow line. `div(a·v̂) = v̂·∇a + a·(∇·v̂) = 1`, so with `∂a/∂ℓ = v̂·∇a`:

```
  ∂a/∂ℓ  =  1  −  k_c · a          k_c  :=  ∇·v̂   (the plan/contour curvature)
```

This is Bonetti's local form, and the briefing reads it correctly: **the magnitude of `a` is governed by the divergence of the unit direction field.** But then note what `k_c` *is*: **`∇·v̂` is a derivative of the DIRECTION FIELD alone.** It is a functional of `v̂`, and of nothing else.

> ### **[D] ⇒ IF THE FIRST MOMENT IS EXACT AT EVERY CELL — i.e. the transport direction equals `−∇φ/‖∇φ‖` everywhere — THEN `v̂` IS CORRECT EVERYWHERE, HENCE `∇·v̂ = k_c` IS CORRECT AUTOMATICALLY, TO THE ACCURACY OF THE DIVERGENCE OPERATOR.**
>
> **The spread does not have to "match `k_c`". `k_c` is not a free target the second moment must be tuned against — it is an OUTPUT of the direction field the first moment already pins. The briefing set up a constraint that is already satisfied.**

### 3.2 [D] Refutation 2 — the second moment is a *spurious diffusion*, and the modified equation says so with a sign and an order

Do what `CLAUDE.md` demands: **Taylor-expand the discrete scheme and read off the PDE it is actually solving.**

A splitting router is a **Markov transition**: mass at cell `i` moves to neighbour `k` with probability `w_k`, displacement `d_k`. Drainage area is the Green's function of that transition, `A = (I − Pᵀ)^{-1}·area`. The continuum limit of a random walk with step law `p(d)` is a **Fokker–Planck / advection–diffusion equation** whose coefficients are exactly the step moments:

```
  drift        μ  =  Σ_k w_k d_k                    ← the FIRST moment
  diffusivity  D  =  ½ Σ_k w_k d_k ⊗ d_k            ← the SECOND moment
```

So the modified equation of *any* normalised splitting router is:

> ```
>   TRUE:      ∇·(a v̂)  =  1
>
>   ACTUAL:    ∇·(a v̂)  =  1  +  ∇∇ : (D a)  +  [first-moment drift error]
>                              └──────┬──────┘
>                          AN ARTIFICIAL ANISOTROPIC DIFFUSION OF DRAINAGE AREA
>                          second differential order · POSITIVE-DEFINITE by construction
> ```

**[D] And the sign is not negotiable.** `D = ½ Σ w_k d_k ⊗ d_k` is a nonnegative-weighted sum of rank-1 PSD matrices. **It is positive semi-definite always, and its trace `½ Σ w_k |d_k|² ≥ ½·min|d_k|² > 0` is bounded BELOW by the cell size.**

> ### **[D] ⇒ YOU CANNOT SET THE SECOND MOMENT TO ZERO. A SPLITTING ROUTER HAS AN IRREDUCIBLE NUMERICAL DIFFUSION FLOORED BY THE GRID. The true law `div(a v̂)=1` is PURE ADVECTION — its correct `D` is EXACTLY ZERO — so the second moment's correct value is not "match `k_c`", it is "as small as the grid permits". It is an ERROR to be MINIMISED, not a quantity to be MATCHED.**

**⇒ The "moment programme" as the briefing framed it — a ladder of constraints `Σw=1`, `Σw·s=0`, `Σw·s² = k_c` — cannot work, and not for a DOF reason. It cannot work because the third rung is pinning a numerical artifact to a physical target that the second rung already delivers.** That is the clean *"the missing term is X and here is why the moment programme cannot work"* the briefing said would be worth as much as a kernel. **[D]**

### 3.3 [D] And the orders explain every measured fact, which is the real test

| moment | what it is | error's differential order | converges under refinement? | **measured** |
|---|---|---|---|---|
| **0th** `Σw = 1` | mass | — | exact | **[M] all routers conserve to `1.000000000000`** ✓ |
| **1st** `Σ w sin(β−ψ)` | drift direction | **ZERO** (a pointwise gain/angle error) | **NO — level-independent BIAS** | **[M] κ identical L9→L23 (16 384× refinement); fan error 15.0° at every level** ✓ |
| **2nd** `Σ w sin²(β−ψ)` | dispersion | **TWO** (a diffusion) | **YES — at O(√h)**, slowly | **[M] the plume SPREAD does converge: 20.5° → 6.7°** ✓ |

**[D] The transverse spread after a path of length `L` with steps of size `h` is `√(σ²·h·L)` — it vanishes as `O(√h)`, half-order.** The drift error does not vanish at all: a fixed angular bias per step integrates to `L·(bias)` regardless of `h`.

> **⇒ THE MOMENT LADDER MAPS EXACTLY ONTO THE PROJECT'S OWN BIAS-VS-NOISE AUDIT (`discretisation-and-information.md` §1), AND IT PREDICTED — BEFORE I LOOKED — WHICH OF OUR MEASUREMENTS WOULD BE LEVEL-INDEPENDENT AND WHICH WOULD CONVERGE. It gets all three right. That is the strongest evidence the frame is correct.**

**[me] And it correctly predicts the sign of the harm.** Artificial diffusion of `a` **smears** the drainage field: it flattens the peaks (channels under-concentrated) and fills the troughs (hillslopes over-supplied). Since stream power runs `A^m` with `m = 0.5` (**concave**), a smeared `A` does *not* average out — by Jensen it **over**-predicts incision on the hillslopes and **under**-predicts it in the channels. That is the same sign, and the same mechanism, as `DECISIONS[the-jensen-variable-was-wrong-and-so-was-the-sign]`. **The router's numerical diffusion and the coarse-graining bias are the same term entering through the same variable.**

---

## 4. [D][P] So what IS the missing term? — `p` is two terms wearing one coat

Return to the actual kernel. `erosion.rs` weights outflow by

```
  w_k  ∝  ( (h_K − h_L) / d_KL ) ^ q          q = 1.1        ← the DIRECTIONAL slope, to a power
```

**[P] Coatléven & Chauveau 2025, §5, verbatim:**

> *"if one wants to incorporate powers of the slope in the flow distribution procedure, then one should **not** use powers of the directional slope `(1/d_KL)(h_{s,L} − h_{s,K})` but rather use powers of `‖G_{s,σ}‖` to remain consistent with a continuous model incorporating powers of `‖∇h_s‖`. **Otherwise, the consistency of the flow routing algorithm will be lost again.** In [42] it is even suggested to **choose different values of `q` for different grid sizes, emphasizing this non-consistency.** However, the sought flow concentration effect can be achieved in a consistent manner ... through the use of `p_w` with value `p_w = q − 1`: **the full gradient and not only the directional gradient being used this way, this does not compromise consistency** and a value independent of the mesh should be chosen according to physical considerations."*

Read the middle sentence twice. **The field re-tunes `q` per grid size. That is the confession.** An exponent that must change when you change the mesh is not a physical parameter; it is absorbing a discretisation error.

> ### **[P][D] THE MISSING TERM, NAMED:**
>
> **`(slope)^q` on the DIRECTIONAL slope is ONE number doing TWO jobs:**
>
> | job | what it should be | what `q` on the directional slope actually does |
> |---|---|---|
> | **WHICH WAY** the water goes | the reconstructed gradient **direction** `−∇h/‖∇h‖` — a *vector*, per cell | re-weights the 8 fixed compass points against each other, so the effective direction is a `q`-dependent function of the fan's geometry — **grid-locked** |
> | **HOW CONCENTRATED** the flow is | `‖∇h‖^{p_w}`, `p_w = q − 1` — a **SCALAR**, per cell, mesh-independent, chosen on physical grounds | is smuggled into the same per-direction exponent, and can only be expressed by *distorting the direction* |
>
> **Split them and both become principled. Leave them fused and NO post-processing can repair the router — the paper is explicit that consistency is lost *"again"*, i.e. the correction (12)–(13) does not save a `q ≠ 1` scheme.** **[P]**

**⇒ The briefing's meta-prediction is vindicated and its content is replaced:**

- ✅ *"`p` has been doing two jobs, badly, at once. Split them and both may become principled."* — **exactly right.**
- ❌ *"the two jobs are (masking a first-moment error) and (setting a second moment)."* — **wrong.** The two jobs are **(selecting a direction)** and **(setting a scalar flow-concentration)**. The second moment is not a job anyone wanted done; it is the *damage*.
- ❌ *"Freeman picked `p = 1.1` by trial-and-error to cancel DIRECTIONAL bias on a square lattice."* — **[⊘] unverified, and now unnecessary.** Whatever Freeman's motive, the *effect* is a consistency violation the paper characterises exactly.

### 4.1 [D] The `p ≠ 1` violation, in modified-equation form

For a cell whose downhill neighbours sit at bearings `β_k` with true slopes `S_k = S·cos(β_k − ψ)` (a locally planar surface, gradient magnitude `S`, azimuth `ψ`), the weights are `w_k ∝ (S·cos(β_k−ψ))^q`, and **the `S^q` factor cancels in the normalisation.** So:

```
  w_k  =  cos^q(β_k − ψ)  /  Σ_j cos^q(β_j − ψ)
```

> **[D] ⇒ `q` DOES NOT AFFECT FLOW CONCENTRATION AT ALL ON A PLANAR CELL — the gradient magnitude cancels exactly. Its ONLY surviving effect is to sharpen or flatten the ANGULAR distribution about `ψ`. `q` is, in its entirety, a SECOND-MOMENT knob wearing a slope-exponent's clothes.**

**That is the sharpest form of the finding, and it is checkable on paper.** The transport direction is `Σ w_k ê_k`; raising `q` concentrates the fan toward the steepest compass point (as `q → ∞`, D8; as `q → 0`, uniform over all downhill neighbours). **On an evenly-spaced fan, `Σ w_k sin(β_k − ψ) = 0` by symmetry for ANY `q`** — the first moment is free. **On our sheared cube-sphere fan the nodes are NOT evenly spaced, so the first moment is `q`-dependent and nonzero.**

> ### **[D] ⇒ AND THERE IS THE COUPLING THE BRIEFING WAS REACHING FOR, DERIVED RATHER THAN GUESSED: on a NON-UNIFORM fan, the SECOND-MOMENT KNOB `q` LEAKS INTO THE FIRST MOMENT. That is *why* `p = 1.1` looks like it is "cancelling directional bias" — on a square lattice it cannot (symmetry forbids it), but on ANY sheared lattice it does, uncontrollably, as a side-effect. `q` is a dispersion knob whose directional side-effect is exactly zero on the grid it was tuned on and NONZERO on ours.**
>
> **⇒ Freeman's `1.1` is a constant calibrated on a lattice where its only effect was benign, imported onto a lattice where it has a second, unintended, grid-locked effect. That is the whole story, and it is a *theorem about the fan*, not a fact about Freeman.** **[D — check it]**

---

## 5. What this says to build

**Ranked, with the cheap and certain first.**

1. **[P-backed, ~6 lines, certain] Set `q = 1` in `accumulate_drainage`.** It is a consistency violation with a named, published substitute. Recover the flow-concentration effect, if it is wanted, as `‖∇h‖^{p_w}` with `p_w = q−1 = 0.1` — **a scalar cell property**, computed once from the LSQ gradient, multiplying the cell's transmissivity uniformly. *This changes the direction field. Re-measure everything after it.* ⚠ **The briefing said "fix `p`, `cell_area` and the hardcoded distances first and re-measure — the baseline may not be what we think." That instruction is now backed by a theorem, not a hunch.**
2. **[P-backed] Carry the true per-cell area and the true edge geometry** (`drainage-area-uses-a-uniform-cell-area`, already logged; +17.8% bias).
3. **[P-backed] Consume `q_K = ‖Q_K‖`, not the raw accumulation.** A local, algebraic, no-solve post-processing pass, `O(N·valence)`, on output we already have. It gives the consistent **magnitude** *and* the consistent **direction** for free — which is exactly what a sediment-coupling term needs. **[⊘] It is the fix for the KERNEL, not for the router table (§2.1).**
4. **[D] MINIMISE the second moment; do not constrain it.** Subject to `Σw = 1` and the first-moment condition. The minimiser is the tightest receiver pair bracketing `ψ` — **but that is a linear program whose optimum is a vertex, i.e. D-∞, which Prescott convicts.** ⚠ **[⊘] THE OPEN QUESTION THIS SPIKE LEAVES: is D-∞ bad because its DISPERSION is too low, or because its DIRECTION ESTIMATE (8 triangular facets) is biased? These are separable and nobody has separated them.** The E2 "oracle router" in `MEASUREMENTS.md` is designed to answer exactly this, and **it is the experiment I would run next.**

---

## 6. What could still kill all of this

- **[⊘] The non-locality objection stands and I have not answered it.** `A` is a *global* accumulation. Nothing above proves that a *local* moment condition controls the *accumulated* error. The modified-equation frame (§3.2) is the honest reply — the Green's function of a local transition operator *is* controlled by that operator's moments — but it is an asymptotic argument, and the wavelet spike's `corr(local sub-grid detail, coarse-law error) = −0.027` is a measured reminder that locality does not obviously reach `A`.
- **[⊘] The geometric identity (eq. 6) is EUCLIDEAN and our cells are SPHERICAL.** Residual unmeasured at the time of writing; at the coarse tier (L2, sagitta 24% of the cell) it may be fatal. If so, the reconstruction is a fine-tier tool only.
- **[⊘] The rotation test has not run.** Until it does, *every accuracy number in the router table — including the ones this document reasons from — is a cone result at a face centre, which is a D4-symmetric NULL TEST.* This is the project's own standing gate and I have not cleared it.
- **[⊘] Pits, flats and accumulation zones are outside Coatléven's well-posedness theory entirely** (his §4: the discrete tell is `s_K = 0`). They are not a corner case in a real LEM, and Priority-Flood is *how we make them*.

---

## 7. Sources

- **[P] Coatléven, J. & Chauveau, B. (2025).** *A post-processing to restore numerical consistency for the most classical multiple flow direction algorithms.* Computational Geosciences. DOI `10.1007/s10596-025-10359-5`. **Paywalled at Springer; read in full from the authors' HAL preprint of record** (`hal-04734436v1`, deposited 2024-10-14), retrieved via the Wayback Machine. Local: `refs/coatleven-chauveau-2025-CorrectedMFD-HAL-preprint.pdf` + `.txt`.
  ⚠ **The paper does NOT prove convergence for the general model — it says so itself** (§2.3: *"we limit ourselves to numerical confirmations"*). The proof and the error estimates live in Coatléven 2020.
- **[P] Coatléven, J. (2020).** *Some multiple flow directions algorithms for irregular meshes.* ESAIM: M2AN 54(6):1917–1949. **Open access. This is the one with the theorems.** Thm 6.1 / Cor. 6.2: the flux-vector estimate converges in `L²` at rate **`h^{1/2}`** (not `O(h)`), under mesh regularity (A1) and a *strongly consistent gradient reconstruction operator* (Def. 4.2). Local: `refs/coatleven-2020-M2AN-MFD-general-meshes.pdf`.
- **[P] Coatléven, J. & Chauveau, B. (2024).** ESurf 12:995. Open access. Local: `refs/esurf-12-995-2024-coatleven-chauveau.pdf`.
- **Bonetti, S., Bragg, A. & Porporato, A. (2018).** Proc. R. Soc. A 474:20170693 — `div(a·v̂) = 1`; the local form `∂a/∂ℓ = 1 − k_c·a`. *(Not re-read this spike; taken from `DECISIONS[mfd-is-the-two-point-flux…]`, which read it.)*
- **Prescott et al. (2025).** ESurf 13:239 — D-∞ is biased along cardinal/ordinal directions; **MFD is superior to D-∞**; the **rotation test**. *(Not re-read this spike.)*
- **[thm] Standard, invoked not re-derived:** the Fokker–Planck / Kramers–Moyal expansion of a Markov jump process (the drift–diffusion read-off in §3.2); the divergence theorem (§2); Jensen (§3.3).

---

*Author's honesty note. §1, §2 and §4's quotations are from a direct read of the primary. §3 and §4.1 are **derived here and are the claims most worth attacking** — in particular **[D] §4.1's "`q` cancels the gradient magnitude exactly on a planar cell, so `q` is purely a second-moment knob"**, which is a paper-checkable algebraic claim that I believe is right and that nobody has stated in these terms. The three things I would most want killed: **(1)** that `k_c` is delivered free by the first moment (§3.1) — if false, the briefing's hypothesis revives; **(2)** that `‖Q_K‖ ≡ A_K/W` and Coatléven therefore adds nothing to the router table (§2.1) — if false, the whole table must be re-measured with `‖Q_K‖`; **(3)** that the second moment's correct value is zero rather than `k_c` (§3.2). Each has a probe attached in `MEASUREMENTS.md`.*
