<!--
Integration drafts produced by the spike-nonlocal-closure session, for the
coordinator to integrate on `main` (sketch-detail-abstract-reversion is mid-
territory / peer-occupied, so nothing is written into core/src/ from this
worktree). Three pieces:
  A. a standalone Appendix segment for the no-go (drop into core/src/);
  B. the DECISIONS entry (:by claude :status proposed);
  C. the subject-side residue patch for #sketch-detail-abstract-reversion.
All measured numbers are in msc/spike-nonlocal-closure/RUN.txt; deterministic
(3× verified). FORMAT.md followed (LaTeX math, segment voice, Epistemic Status
with max-attainable, Working Notes as forward residue).
-->

# A. Appendix segment — the coarse-only closure no-go

*Proposed slug `obs-coarse-only-closure-nogo` (matches the `obs-` family:
`#obs-mean-pin-manufactures-seam`, `#obs-cube-locked-kernel-bias`). Appendix
row, `## Appendix — retired paths and failure galleries`. Alternatives if a
different noun is preferred: `obs-trunk-not-in-coarse-state`,
`obs-drainage-restriction-nogo`.*

```markdown
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
(two seeds, six tile$\times$depth cells each; `msc/spike-nonlocal-closure/`
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
- **Instrument:** `Fluvial::drainage_override` / `drainage_recalibrate`
  (spike-only, default-inert) are diagnostics, not kernel candidates — the
  override consumes fine info, the recalibrate is this no-go.
```

---

# B. DECISIONS entry (append to DECISIONS.decision-log.udon)

```udon
|decision[coarse-only-drainage-closure-is-a-nogo] :date 2026-07-24 :by claude :status proposed :topic physics
  :session 8145183f-e9f3-4836-b4c8-edc86bed663e
  SPIKED (`msc/spike-nonlocal-closure/`, 8 probes on the real fluvial kernel + verified-land footprints; `PREDICTIONS.md` pre-registered; `RUN.txt`; deterministic, 3× verified). The open half of `#sketch-detail-abstract-reversion` FE(5) — the nonlinear closure for a non-local flux — is **bracketed on both sides**. Positive: the closure's carrier is the drainage **trunk** (a density-vs-flux type error; area-mean is the wrong restriction for a flux). No-go: that trunk is **not recoverable from coarse state** — no pointwise coarse-only closure exists. Claim home for the no-go: `#obs-coarse-only-closure-nogo` (Appendix); positive residue stays on the sketch.
  |reason
    **RE-ANCHOR (measured, incidental but load-bearing).** The wavelet spike's `+5.34 m` commutator was measured on a world that no longer exists: its footprint (ZPos 108544,186368) is now **entirely submarine** (relief 3080–3252 m vs the current derived sea level **5106 m**); erosion there is a near-no-op (max 1.16 m) and the commutator collapses to a fabricated ~0.001 m — the land-guard failure mode. Re-anchored on a fresh 100%-land tile (ZNeg 327680,65536): $\lVert R\circ E - E\circ R\rVert$ signed mean **+7.17 m**, RMS 12.45 m, bias/noise 0.71, against 8.94 m carved. Segments quoting +5.34 need re-anchoring language when this graduates.
    **CARRIER (measured, positive — lives on the sketch).** The drainage graph is the carrier where local statistics are blind. Injecting the fine trunk $\max(A_{\text{fine}})$ per coarse cell **zeroes the bias at depth 2** (+7.17 → **+0.34 m**, bias/noise 0.71 → 0.04 — bias converted to noise, `#norm-bias-vs-noise` success metric). The area-weighted MEAN — the restriction that is exact for the conserved height integral — is robustly WRONG for A: it over-corrects to −4.3…−6.2 m across all tiles/depths, because drainage area is an **accumulated flux, not a density**. This is the cell-side twin of the wavelet spike's face-flux result (fluxes restrict length-weighted so ∫F dl telescopes): A must be coarse-grained as a flux (trunk/through-flow), not as a density (mean). A drainage-commutator proxy predicts the SIGNED height commutator at r = +0.44, where local h-detail gave −0.027.
    **NO-GO (measured — the Appendix claim).** The deployable question is whether the trunk is a coarse-only function of $A_{\text{coarse}}$. It is not. Fit $\max(A_{\text{fine}}) \approx \alpha\,A_{\text{coarse}}^{\beta}$: **$R^2 = 0.06$–0.36 across 2 seeds × 3 tiles × 2 depths** (12 cells; ceiling 0.359). $R^2$ bounds EVERY monotone pointwise map, not just the power law ⇒ no pointwise coarse-only closure reaches the trunk. Held-out, the tile-1 fit makes bias/noise WORSE (0.71 → 0.90). Mechanism: the coarse flow network REROUTES (it does not merely miscalibrate); the trunk depends on **sub-grid channel POSITION**, which no coarse per-cell quantity encodes. A sub-grid moment is not enough either (sub-grid $A^m$ variance vs gap: r ≈ −0.11).
    **POINTWISE JENSEN (measured, sub-dominant).** FE(4)'s single-epoch decomposition holds with predicted signs ($\langle A^m\rangle - \langle A\rangle^m \lt 0$ concave; $\mathrm{Cov}(A^m,S) \lt 0$), but the +7 m bias is absent at one epoch — it COMPOUNDS over epochs through the drainage feedback (wrong A → wrong incision → wrong topography → worse A), per `#norm-bias-vs-noise` FE(1). Routing, not pointwise concavity, is the dominant mechanism.
  |impact
    NOT a closed door — Joseph: a no-go is an input to the next strengthening round. FE(5) successor text (see `#sketch-detail-abstract-reversion` Working Notes) carries the not-excluded family: a coarse-network-**topology** carrier, a cheap carried **sub-grid channel summary**, or a **fine spine** at adequate resolution — the last now with a measured reason (`#detail-drainage-dependency-planning`: the graph cannot be shortcut with a coarse-state function). Kernel: `Fluvial::{drainage_override, drainage_recalibrate}` are spike-only default-inert instruments (SPIKE INSTRUMENT-tagged, drop-entirely rebase surface, disjoint from `pin_block_means`); neither graduates — the override consumes fine info, the recalibrate is this no-go. UNTESTED: exponents other than $m=0.5,n=1$; deposition/MFD-p regimes; whether a two-variable coarse carrier ($A_{\text{coarse}}$ + coarse slope) or a topology summary raises the 0.36 ceiling.
  |ref msc/spike-nonlocal-closure/{PREDICTIONS.md, RUN.txt, src/{main,probes,area,mra}.rs} · #obs-coarse-only-closure-nogo · #sketch-detail-abstract-reversion
```

---

# C. Subject-side residue patch for #sketch-detail-abstract-reversion

*The sketch keeps the subject-side residue; the no-go itself moves to the
Appendix segment (cited, not restated). Proposed edits:*

**FE(5) — replace the closing lines** ("The open problem is the nonlinear
closure for a non-local flux. Design lesson inherited from Harten…") so the
present state is stated, with the re-anchored baseline:

> The **law** does not commute with restriction: on a fresh verified-land
> footprint $\lVert R\circ E - E\circ R\rVert$ has signed mean $+7.17\,\mathrm{m}$
> (bias/noise $0.71$) against the $8.94\,\mathrm{m}$ erosion carved — a bias the
> size of the physics. (The historically-quoted $+5.34\,\mathrm{m}$ was measured
> on a footprint the generator has since moved underwater; it is retired, not
> reproduced.) The carrier is the drainage **trunk**: drainage area is an
> **accumulated flux**, not a density, so its conservative restriction is the
> through-flow (trunk), **not** the area-weighted mean that heights use —
> injecting the fine trunk $\max(A_{\text{fine}})$ per coarse cell zeroes the
> bias at depth 2, while the area-mean over-corrects. This is the cell-side twin
> of the face-flux telescoping ( #form-seam-flux-exchange ). Design lesson
> inherited from Harten: never project the nonlinear operator — keep the
> conservation-form update; use multiresolution as representation + decision
> layer only.

**Add FE(6') (or fold into FE 6) — the coarse-only bound and the open family:**

> The trunk is **not recoverable from coarse state**: no pointwise function of
> the coarse-routed drainage reaches it (measured $R^2 \le 0.36$, two seeds —
> #obs-coarse-only-closure-nogo ), because the coarse network reroutes and the
> trunk depends on sub-grid channel position. A deployable closure must therefore
> carry information the coarse per-cell state does not hold: **(i)** the coarse
> drainage **topology** (multi-cell — receivers/upstream structure, not a
> per-cell value), **(ii)** a **minimal sub-grid channel summary** stored with
> the coarse cell, or **(iii)** a **fine spine** evolved at adequate resolution
> ( #detail-drainage-dependency-planning , now with a measured reason). Which is
> open (FE 5 strengthening round).

**Working Notes — add:**

> - **The no-go is an input to the next strengthening round, not a closed door**
>   (Joseph, 2026-07-24). #obs-coarse-only-closure-nogo excludes only
>   pointwise-in-$A_{\text{coarse}}$ closures; the topology / sub-grid-summary /
>   fine-spine family (FE 6') is untried.
> - **Minimal sub-grid channel summary — what it would have to carry (sketch).**
>   The trunk is set by *where the channel is* and *how much it carries*, so the
>   smallest summary that could close the gap is roughly, per coarse cell: the
>   through-flow magnitude of its dominant sub-channel (the trunk area, $\approx
>   \max A_{\text{fine}}$ — the one quantity the oracle proved sufficient) **plus**
>   the inflow/outflow **edge** it uses (which coarse face the trunk crosses), so
>   the coarse network can be re-wired to the fine trunk's actual connectivity
>   rather than the coarse routing's invented one. That is a per-cell scalar + a
>   boundary tag — a sub-grid *flux* record, not a sub-grid *variance*; it is the
>   cell-level analogue of the face-flux register ( #form-face-flux-register ),
>   and it is the natural thing a drainage-shaped dependency plan
>   ( #detail-drainage-dependency-planning ) would memoize at a seam. Untested:
>   whether that summary, injected coarse-side, reproduces the depth-2 trunk
>   result without the full fine run.
> - **Baseline re-anchor:** the fresh number is $+7.17\,\mathrm{m}$ (seed 0, ZNeg
>   327680,65536); $+5.34\,\mathrm{m}$ is dead-world and must not be re-quoted as
>   current.
