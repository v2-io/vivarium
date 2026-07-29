# FE(6c) pricing — pre-registered predictions

*Written **before** the harness existed and before any number was produced
(`#norm-probe-sensitivity`). Nothing below was edited after the first run; the
scoring lives in `RESULTS.md`.*

## What is being priced

`#obs-routing-curl-spiral` FE(6)(c) + FE(8): the 2026-07-24 pricing convicted the
**receiver/incision tree** diagonal treatment as landscape-consequential (CUBE
+0.07 → +0.26) using a **naive D4 steepest-descent** tree, which the results file
itself names a strawman. The owed measurement is the **principled** remedy: the
Coatléven flux-vector reconstruction

```
  Q_K = (1/|K|) Σ_σ F_{K,σ} (x_σ − x_K)      q_K = ‖Q_K‖      Q̂_K = Q_K/‖Q_K‖
```

driving the tree direction (and, in the fullest arm, the magnitude), with
FE(6)(d) — the strongly-consistent (LSQ) gradient reconstruction that Coatléven
2020 Def. 4.2 makes a **hypothesis** of Thm 6.1 — as a separately-priced arm
rather than a sibling.

Baselines and controls, same fan wherever the comparison is a tree comparison:

| arm | fan | face weight | tree | magnitude consumed |
|---|---|---|---|---|
| `UniformOld` | 8 | (drop/d)¹, uniform d | D8 | raw acc |
| `LiveMfd` | 8 | (drop/d)¹ | D8 | raw acc |
| `EdgeTrue` | 4 | drop/d | D8 | raw acc |
| `EdgeFull` | 4 | drop/d | **D4 naive** (strawman) | raw acc |
| `EdgeTau` | 4 | **\|σ\|·drop/d** (transmissivity) | D8 | raw acc |
| `CoatTpfa` | 4 | \|σ\|·drop/d | **Q̂ nearest-bearing** | raw acc |
| `CoatGrad` | 4 | **\|σ\|·max(−∇h·n̂,0)** (LSQ) | Q̂ nearest-bearing | raw acc |
| `CoatMag` | 4 | \|σ\|·max(−∇h·n̂,0) | Q̂ nearest-bearing | **‖Q_K‖·√A_K** |

`EdgeTau` exists so the transmissivity factor is not confounded into the
Coatléven arms. `CoatGrad − CoatTpfa` is FE(6)(d)'s own marginal.

Metric of record is unchanged: **CUBE** = (arm−live axis-fraction at the sheared
corner) − (arm−live at the face centre), the null-test differential.

---

## Predictions

**P1 — geometric-identity gate.** The Euclidean identity `|K|·Id = Σ|σ|(x_σ−x_K)⊗n̂`
is what makes the reconstruction exact, and our cells are spherical. At L19
(≈19 m cells on a 6.37e6 m sphere) I expect the Frobenius residual `‖M−I‖` to be
**< 1e-5**, i.e. the sphere is irrelevant *here* even though DERIVATION §6 flags
it as possibly fatal at L2. If I measure > 1e-3 I will read it as **my normals or
face centres being wrong**, not as the sphere, and fix the geometry before
reporting anything downstream.

**P2 — the headline (moderate confidence).** `CoatGrad`'s CUBE will be **smaller
in magnitude than `EdgeFull`'s at every τ**, and I pre-commit the sharper
threshold `|CUBE(CoatGrad)| < 0.5·|CUBE(EdgeFull)|`. Reasoning: the strawman's
swing is attributed to a *discrete* axis-lock (a D4 tree can only ever point at
4 azimuths, and which of them wins is a function of the local Jacobian shear); a
continuous reconstructed direction has no such preferred set.

**P3 — but not benign (this is the prediction I actually expect to be the
interesting one).** `CoatGrad`'s CUBE will **not** be ~0 like the fan half was.
Range predicted: **0.02 ≤ |CUBE| ≤ 0.10** at most τ. Reasoning: the tree is a
*single-receiver* structure, so however good `Q̂` is, it is projected back onto
one of 8 lattice bearings, and the projection residual is itself a function of
the cube Jacobian. If P2 holds and P3 holds, the honest present truth is "the
principled remedy roughly halves the cube-locked orientation consequence but does
not remove it, because the defect is partly the *tree*, not the direction
estimator" — which would reframe FE(6c) rather than close it.

**P4 — FE(6)(d)'s own marginal (low-moderate confidence).** `CoatGrad` vs
`CoatTpfa` will differ **little on CUBE** (`|ΔCUBE| < 0.03`) while differing
**substantially on the field** (log-drainage Spearman between them < 0.90).
Reasoning: non-orthogonality correction is a smooth O(1) rotation of the face
weights — it moves mass around, but the cube-locked *orientation* signal is a
discrete lattice effect the rotation does not obviously address. If this is
wrong — if (d) is what buys the CUBE improvement — that is a strictly more
useful finding and it reorders the remedy stack again.

**P5 — magnitude (‖Q‖).** `‖Q_K‖·√A_K / drainage_K` will have a **median in
[0.3, 3.0]** over land (i.e. the reconstruction is an O(1) correction, not an
order-of-magnitude re-scale) but with a **long left tail** in convergent zones.
`CoatMag`'s CUBE will sit **within 0.05 of `CoatGrad`'s** — magnitude is a
per-cell scalar and should be weakly coupled to channel *orientation*. If
`CoatMag` swings hard, the magnitude half is doing orientation work and the
"direction and magnitude from one object" framing gains a measured consequence.

**P6 — the s_K = 0 carve-out (the standing limit).** Coatléven's well-posedness
excludes pits/flats/accumulation zones and Priority-Flood *manufactures* them. I
predict **> 10% of land cells are raised by the fill pass** in a mature epoch,
and that carving those cells out of the channel mask **reduces every arm's |CUBE|**
— because the fill ramp (EPS = 1e-3 m over a ~19 m cell, slope 5e-5) is where
flow direction is most arbitrary and therefore most available to be captured by
the lattice. **If carving them out instead *increases* |CUBE|, my model of where
the defect lives is wrong** and the fill machinery is masking the defect rather
than manufacturing it.

**P7 — affordability (the census open).** Wall-time for `CoatGrad` within
**2.5×** `LiveMfd`. On halo: I predict the reconstruction needs **halo 2, not
halo 1**, by the argument that `Q_K` consumes its neighbours' *outgoing splits*,
which need those neighbours' weights, which need elevation at distance 2. Note
this is predicted to be true even for `CoatTpfa` — i.e. halo 2 is the price of
the *reconstruction*, not of the LSQ gradient. I will state this as derived +
checked against the code, not as measured.

**P8 — the null I am prepared to report.** It is entirely possible `CoatGrad`'s
CUBE comes back **as large as or larger than** `EdgeFull`'s. If it does, the
finding is *"the principled remedy is also landscape-consequential and cube-locked;
the single-receiver tree is the defect, not the direction estimator"* — and that
demonstrated no-go is worth as much as a win, because it would move FE(6c) from
"build the reconstruction" to "the tree structure has to go too." I am writing
this down so that outcome cannot be reframed after the fact.

**P9 — what would make me distrust my own harness.** (a) P0 bit-match against
live `erosion.rs` failing; (b) any arm non-deterministic across 3 reruns (the
HashMap-order self-catch from the curl-probe work); (c) `EdgeFull` not
reproducing the 2026-07-24 CUBE band (+0.07 → +0.26) — this harness is a
re-implementation and `EdgeFull` is its overlap with the prior experiment, so it
is a free replication check and a failure there voids everything else.
