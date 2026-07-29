---
slug: obs-coarse-only-closure-nogo
type: observation
status: empirical
stage: draft
depends:
  - sketch-detail-abstract-reversion
  - norm-bias-vs-noise
  - detail-drainage-dependency-planning
  - form-rl-closure-algebra
  - detail-info-theoretic-discretisation
---

# The coarse-only trunk closure is a measured no-go

The erosion closure's carrier is the drainage trunk, and the trunk is **not a
function of the coarse drainage state** — so no pointwise recalibration of the
coarse-routed drainage can supply it (measured $R^2 \le 0.36$, two seeds).

## Formal Expression

1. **The setting.** The fluvial law does not commute with restriction: for a
   fresh verified-land footprint, $\lVert R\circ E - E\circ R\rVert$ has signed
   mean $+7.17\,\mathrm{m}$ (RMS $12.45\,\mathrm{m}$, bias/noise $0.71$) against
   the $8.94\,\mathrm{m}$ erosion carved — a bias, by the decisive audit
   ( #norm-bias-vs-noise ). The carrier is the drainage graph: injecting the
   fine **trunk** area $\max(A_{\text{fine}})$ per coarse cell zeroes the bias at
   depth 2 ($+0.34\,\mathrm{m}$), where the area-weighted mean over-corrects to
   $-6.23\,\mathrm{m}$ (the positive half lives at #sketch-detail-abstract-reversion ).

2. **The seductive wrong answer.** The trunk that zeroes the bias uses the
   **fine** run. The obvious deployable move is to recover it from the coarse
   spine alone — fit $\max(A_{\text{fine}}) \approx \alpha\,A_{\text{coarse}}^{\beta}$
   and remap the coarse drainage $A \leftarrow \alpha\,A^{\beta}$ each epoch. A
   power-law recalibration of drainage area against a reference network *looks
   exactly like* the kind of closure that should work — it is monotone, it is
   coarse-only, and $A_{\text{coarse}}$ over-accumulates the trunk (median
   $\lvert A_{\text{coarse}} - \max(A_{\text{fine}})\rvert / A \approx 144\%$), so
   a downward recalibration seems obviously indicated. That is the seduction.

3. **The measured exclusion.** The fit is weak everywhere:
   $R^2 = 0.06\text{–}0.36$ across two seeds $\times$ three tiles $\times$ two
   depths (twelve cells; ceiling $0.359$). Because $R^2$ upper-bounds **every**
   monotone pointwise map $A_{\text{coarse}} \mapsto A_{\text{eff}}$ — not merely
   the power law — no pointwise coarse-only closure can explain more than
   $\sim\!36\%$ of the trunk's variance. Applied held-out, the fit makes the bias
   **worse**: bias/noise rises $0.71 \to 0.90$ even on the tile it was fit to,
   over-correcting to $\approx -6\,\mathrm{m}$.

4. **The mechanism — why it cannot work.** $A_{\text{coarse}}$ is not a
   miscalibration of the trunk; the coarse flow network **reroutes**. Two coarse
   cells with equal $A_{\text{coarse}}$ can carry very different fine trunks,
   because the trunk depends on **sub-grid channel position** — where the fine
   channel sits inside the coarse cell — which the coarse grid does not resolve
   and no per-cell coarse quantity encodes. This is the non-local flux obstruction
   made concrete: a sub-grid *moment* is not enough either (sub-grid $A^m$
   variance correlates with the gap at $r \approx -0.11$; sub-grid $h$ variance at
   $+0.06$) — the missing information is channel *geometry*, not variance.

5. **What is NOT excluded.** The no-go bounds pointwise functions of
   $A_{\text{coarse}}$ only. It does not exclude a closure that reads the coarse
   **network topology** (receivers, upstream structure — multi-cell, not
   pointwise), a carried **sub-grid channel summary**, or a **fine spine** run at
   adequate resolution. This is the measured reason behind
   #detail-drainage-dependency-planning : the drainage graph cannot be
   shortcut with a coarse-state function; it must be evolved or memoized.

## Epistemic Status

**Max attainable: robust-qualitative.** The $R^2 \le 0.36$ ceiling is empirical
(two seeds, six tile$\times$depth cells each; `.super-archive/from-msc/spike-nonlocal-closure/`
PROBE 6/8, deterministic). The structural claim — the trunk depends on sub-grid
channel position, which is independent of coarse per-cell state — is argued from
the measurement and from the routing mechanism, not yet derived; a derivation
(that $\max(A_{\text{fine}})$ is not $\sigma(A_{\text{coarse}})$-measurable) would
raise it. **Currently `empirical`.** Stage `draft`. The exclusion is scoped to
pointwise-in-$A_{\text{coarse}}$ closures at $m=0.5$, $n=1$; richer coarse-only
carriers are open (FE 5).

## Discussion

The gallery earns its keep by naming *why the wrong answer was seductive*: a
monotone recalibration of an over-accumulated drainage field is the first thing
a competent agent reaches for, it is cheap and deployable, and the over-
accumulation is real — every signal says "just scale it down." The measurement
is what convicts it, and the mechanism (rerouting, not miscalibration) is what
stops the next agent re-walking it. Held next to the positive finding it is the
other half of one result: the carrier is the trunk, and the trunk is not in the
coarse state.

## Working Notes

- **Forward (strengthening, not a closed door):** the not-excluded family (FE 5)
  is the next round — a coarse-network-topology carrier or a minimal sub-grid
  channel summary. Sketch of the latter at #sketch-detail-abstract-reversion
  Working Notes.
- **Regression guard:** do not re-propose a pointwise recalibration of
  $A_{\text{coarse}}$ (scale, power-law, value-quantile) as the closure — the
  $R^2 \le 0.36$ ceiling excludes the whole family, measured on two seeds.
- **Instrument (frozen):** `Fluvial::drainage_override` / `drainage_recalibrate`
  were spike-only, default-inert diagnostics, not kernel candidates — the
  override consumed fine info, the recalibrate was this no-go. The planned
  rebase removed them from the crate; the spike no longer compiles against
  main and is a frozen record, not a runnable instrument.
