# PREDICTIONS — nonlinear closure for a non-local flux

*Written before the first run of the new experiments (house law: predictions
precede runs). The baseline reproduction (§0) is not a new prediction — it is
the regression anchor against `msc/spike-wavelet-store` PROBE 5.*

The target, restated from `#sketch-detail-abstract-reversion` FE(5): the live
fluvial kernel `E` does not commute with restriction `R`. Measured baseline:

```
‖R∘E − E∘R‖ : mean (signed) +5.340 m   RMS 9.441 m   max 40.143 m
scale        : mean |Δh| erosion carved at L19 = 7.223 m   (bias/noise |mean|/sd = 0.686)
corr( sub-grid detail RMS of h₀ , |commutator| ) = −0.027   (local statistic is blind)
```

## The mechanism hypothesis

`E = Σ epochs of [ fill → receivers → accumulate-drainage A → incise ]`, with
implicit stream-power incision `f = k·Aᵐ/dist ; h' = (h + f·h_recv)/(1+f)`,
`m = 0.5`, `n = 1`. So per step `E` is monotone-increasing in `Aᵐ` and roughly
linear in slope `S`. Following `#detail-info-theoretic-discretisation` FE(4),
the non-commutation of `E` with `R` has **two stacked sources**:

- **(T) Routing / topological term.** The drainage-area operator `A` *itself*
  does not commute with `R`. Coarsening merges the flow network: `A_coarse`
  (drainage computed on the restricted grid) ≠ `R(A_fine)` (area-weighted
  restriction of the fine drainage). This is the genuinely **non-local** part —
  it depends on graph connectivity, which no cell-local statistic sees.
- **(J) Pointwise term.** Even given the fine `(A,S)` field, `⟨Aᵐ⟩ ≠ ⟨A⟩ᵐ`
  (Jensen, `m=0.5` concave ⇒ `⟨Aᵐ⟩ ≤ ⟨A⟩ᵐ`) and a `Cov(Aᵐ, S)` term
  (fluvial channels: high `A` co-locates with low `S` ⇒ `Cov < 0`).

## Falsifiable predictions

**P0 (anchor).** Reproduce the baseline commutator within measurement noise:
signed mean in `[+4.5, +6.0]` m, RMS in `[9, 10]` m, local-h-detail corr in
`[−0.15, +0.15]`. If not, my construction differs from PROBE 5 and everything
below is suspect.

**P1 (routing mismatch is large and non-local).** The drainage-area commutator
`δA = A_coarse − R(A_fine)` has median `|δA|/A ≥ 0.30`. An erosion-error proxy
built from it — `ε̂ = E·m·(δA/A)` (first-order sensitivity of incision to `A`)
— correlates with the actual commutator with `|r| ≥ 0.5`, i.e. **an order of
magnitude better than the −0.027 that local h-detail gave.** *This is the
central bet: the drainage graph is the closure's carrier.*

**P2 (same field, different statistic — the sharp demonstration).** Sub-grid
**A-variance** (non-local, a drainage statistic) correlates with |commutator|
with `|r| ≥ 0.4`, while sub-grid **h-variance** (local) stays `|r| < 0.15`.
Same coarse cells, two statistics; only the non-local one sees the gap. If this
holds it *explains* the −0.027 rather than just restating it.

**P3 (oracle-A closure removes the majority of the bias).** Running the coarse
kernel with its self-computed `A_coarse` replaced by the correct `R(A_fine)`
drops the signed bias from +5.34 m to `|mean| < 2.0 m` (≥ 60% reduction). If it
collapses to `|mean| < 0.7 m`, routing is essentially the *whole* story and (J)
is second-order at this `m`. *(This is a diagnostic oracle — it consumes fine
info — not a deployable closure. It convicts the mechanism.)*

**P4 (pointwise Jensen residual is real but smaller).** With total `A` per
coarse cell held correct, the analytic pointwise correction
`⟨Aᵐ S⟩ / (⟨A⟩ᵐ ⟨S⟩)` accounts for a signed residual `< 2.0 m` — present, same
sign tendency, but sub-dominant to (T) at `m=0.5`. A single-epoch variant should
match the analytic Jensen+Cov prediction to within ~20% (where the operator is
closest to one clean `Aᵐ·S` application).

**P5 (no-go sharpening — first-class if it lands).** No statistic *local in the
coarse representation* (h-detail coefficients / sub-grid h-variance) can carry
the closure, because the dominant term (T) is a connectivity property of the
drainage graph. The correct carrier is the **restricted drainage field**
`R(A_fine)` (or an equivalent graph summary), consistent with
`#detail-drainage-dependency-planning` ("the drainage graph IS the dependency
structure"). The success metric per `#norm-bias-vs-noise`: a candidate converts
the **signed bias** into something **zero-mean**; P3 is that test for the
graph-carried closure.

## Sign expectation (weaker, stated to be caught if wrong)

`d = R∘E − E∘R > 0` means the coarse-evolved surface `E∘R` sits *lower* than the
true restrict-of-fine `R∘E` — i.e. **the coarse kernel over-incises**. Expected
mechanism: on the coarse grid drainage concentrates onto fewer trunk channels,
raising `Aᵐ` where `S` is still large, so incision runs faster than the fine
network's area-distributed erosion restricts to. I will let the (T)/(J)
decomposition attribute the sign rather than asserting it.

## What would make this a no-go vs a win

- **Win:** P3 holds (graph-carried closure zeroes the bias) → the closure exists
  and its carrier is the drainage graph. Feeds FE(5) successor text.
- **Sharp no-go (equally valuable):** P1 fails — the routing proxy *also* fails
  to correlate — which would mean the gap is not carried by `δA` either, and the
  non-locality is deeper than "wrong drainage area." Then the sharp statement is
  *which* summaries were measured to fail.
- **Disappointment (to avoid):** only re-confirming "it's hard / local fails,"
  which is already owned. Every run here must move a number.

---

# PREDICTIONS — Round 2: the DEPLOYABLE coarse-only closure

*Pre-registered before Round-2 runs. Round 1 established: the bias is a
drainage-restriction type error; the trunk (max fine A) zeroes it at depth 2;
area-mean is robustly wrong. The oracle consumed the fine run. Round 2 asks:
can the trunk be recovered from the COARSE spine alone?*

The subtlety: the coarse spine's own accumulation `A_coarse` (baseline) already
IS a "trunk from the coarse graph" — and it over-erodes (+7.17). So `A_coarse`
over-estimates the fine trunk `max(A_fine)`. The deployable question is whether
`max(A_fine)` is a **pointwise, coarse-only function of `A_coarse`** — if so, a
recalibration `A ← α·Aᵝ` applied to the coarse drainage each epoch is a
deployable closure; if not, the coarse network is topologically scrambled and no
pointwise post-process can reach the trunk (a sharp coarse-only no-go).

**P6 (feasibility).** `A_coarse` over-estimates the fine trunk
(`A_coarse > max(A_fine)` typically), and `log max(A_fine)` vs `log A_coarse` is
well-fit by a line: **R² ≥ 0.7, slope β ∈ (0,1)**. (Both track the true
catchment ⇒ monotone; coarse MFD concentrates more ⇒ β<1, α<1.) *If R² < 0.5 the
closure is a no-go: the coarse flow network reroutes, so the trunk is not a
function of `A_coarse`.*

**P7 (deployable closure works, held-out).** A depth-calibrated recalibration
`A ← α·Aᵝ` **fit on tile 1** and injected coarse-only (the real kernel's own
coarse drainage, transformed each epoch) on **held-out tiles 2 & 3** reduces
`|signed mean|` from ~7 m to **< 2 m** and bias/noise from ~0.7 to **< 0.3** at
both depths — i.e. it converts the bias to noise (`#norm-bias-vs-noise` success
metric) on tiles it was not fit to.

**P8 (transfer = deployability).** The fitted `(α, β)` are consistent across the
three tiles at fixed depth (**β within ±0.15**). If they diverge, the closure is
a tile-fit, not a world law — a no-go for deployability, honestly reported.

**P9 (depth-aware).** `(α, β)` differ by depth: depth 1 needs a **milder**
reduction (β closer to 1, α closer to 1) than depth 2 — consistent with Round 1's
bracket (max was right at depth 2, over-corrected at depth 1).

**Metric & discipline (unchanged):** signed mean + bias/noise ratio; held-out
tiles for the transfer test (no fitting on the tiles you score); 3× re-runs for
determinism.
