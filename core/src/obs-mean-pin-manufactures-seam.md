---
slug: obs-mean-pin-manufactures-seam
type: observation
status: exact
stage: draft
depends:
  - form-rl-closure-algebra
  - form-seam-flux-exchange
  - form-face-flux-register
  - norm-probe-sensitivity
---

# Mean-pin manufactures the seam (and mass)

`erosion.rs::pin_block_means` — the operator written to enforce coarse↔fine consistency — is the single largest manufacturer of the `seam_ridge` it was meant to prevent, does **not** preserve block means, and is a **mass source**. The algebra law $R \circ L = \mathrm{id}$ stands; this observation convicts the **implementation** sold as that law. **It has no production caller** (world-gen composition in `query.rs` seeds a fine tier from the coarse surface and never pins); its live footprint was the operator docstring and one self-certifying test. The council-accepted retirement is **executed** (2026-07-24): the docstring's false-law claim is deleted, the self-certifying test is restated to convict, and the block-constant-injection replacement was measured and **convicted** — **deletion** is the executable step; the admissible non-deletion fix is leaf-only + #form-face-flux-register .

> **Numbers re-based 2026-07-24.** The figures below are current-main (seed 0, footprint face ZPos L19 (135_168, 167_936), 100% subaerial under the *derived* sea level 5106 m, deterministic ×3). The prior figures (seam $5.77$; zero-physics $1.93/1.10$; mass $+0.136\%$) were measured at (108_544, 186_368), which was land only against the **deprecated** `gen::SEA_LEVEL_M = 4000`; under derived sea that region is ~2 km of ocean and `seam_ridge` reads a fake ~1.70 no-op there ( #norm-probe-sensitivity seabed species). The qualitative verdict is unchanged and stronger; the magnitudes moved.

## Formal Expression

1. **What the operator does.** `pin_block_means` computes a per-block delta (coarse target minus fine block mean) and **bilinearly upsamples** that delta onto the fine tile. It is **not** a block-constant injection of the form $h \leftarrow h + \Delta_{\mathrm{block}}$.
2. **It does not pin the mean.** After the operator, $|\text{block mean} - \text{coarse target}|$ retains median $0.489\,\mathrm{m}$, max $2.929\,\mathrm{m}$ (PROBE 6, worst-case composition). A bilinear upsample of a piecewise-constant field does not preserve block means; the residual is the $(1,6,1)/8$ stencil (curvature of the delta). So $R \circ L = \mathrm{id}$ **on the mean is false in the code**, not merely lossy ( #form-rl-closure-algebra compliance debt — this segment is the measurement home). The honest form — block-constant injection $h \leftarrow h + \Delta_{\mathrm{block}}$ (`pin_block_means_const`) — pins the mean to machine precision (residual median $0.00012\,\mathrm{m}$), and *only* that (see FE(8)).
3. **It manufactures the seam ridge.** At macro $80$ epochs / fine $150$ epochs (PROBE 6):

   | Composition | Seam ratio $\mathrm{seam}/\sqrt{\mathrm{out}\cdot\mathrm{in}}$ | Absolute seam curv (m) |
   |-------------|---------------------------------------------------------------:|-----------------------:|
   | Today (pin ON, bilinear) | $12.81$ | $1.056$ |
   | Pin OFF | $3.84$ | $0.274$ |
   | Block-const injection (candidate) | $3.33$ | $0.524$ |
   | + up-propagate fine coarse coeffs (injection) | $8.99$ | — |
   | + outside detail $\equiv 0$ (naive wavelet store) | $33.12$ | — |
   | Pin ON + outside detail $\equiv 0$ | $48.37$ | — |

   Removing the pin and changing nothing else cuts the ridge by more than half. Injection alone (fine→coarse without pin) is **worse** than pin-off ($8.99$ vs $3.84$) — Berger–Colella need injection **and** flux reconciliation; injection alone is not the fix ( #form-face-flux-register ). The block-const candidate's *ratio* ($3.33$) reads below pin-off but its *absolute* seam curvature ($0.524$) is nearly $2\times$ pin-off's ($0.274$): the ratio is flattered by an inflated denominator (see FE(8)) — a #norm-probe-sensitivity trap.
4. **Zero-physics control.** Both tiers at **zero** epochs (no erosion, no differential aging, no carved detail): seam ratio **$1.96$ with pin ON (bilinear)**, **$1.66$ block-const**, **$1.17$ with pin OFF**. The tile machinery manufactures a $\sim 2\times$ ridge with **no physics in the world**; mean-pin is the whole of that excess.
5. **It is a mass source — and block-const does not fix it.** $\int h\,\mathrm{d}A$ of the fine tile before → after pin, **identical** for bilinear and block-const injection:

   | Fine epochs | Relative $\Delta$ mass |
   |------------:|-----------------------:|
   | 18 | $+0.0291\%$ |
   | 80 | $+0.1197\%$ |
   | 150 | $+0.2172\%$ |

   The operator re-creates rock erosion carved away because the coarse tier it pins to has eroded less. Magnitude grows with fine work — the harder the fine tier works, the more mass pin invents. Distributing the same per-block delta as a constant rather than a ramp changes nothing: coercing fine → stale coarse *is* the mass mechanism.
6. **Age drivers (corrected reading).** The seam ratio is monotone in the **fine-tier additional epochs** (the true differential gap when fine is seeded from already-eroded macro), not in a misread $|\text{fine}-\text{macro}|$. At **fixed** fine gap $150$, raising macro $18\to 80\to 150$ takes the ratio $4.29 \to 5.77 \to 8.56$ — a **second driver**: absolute roughness of both tiers.
7. **Category error inside the operator.** Pin compares fine **block means** to a coarse **point sample** at block centre — the cell-average / point-sample fork ( #form-column-control-volume family) live inside the conservation path.
8. **What this licenses — and the replacement experiment's verdict.** Retiring mean-pin is a **deletion** prerequisite for honest multiscale composition — not a cosmetic clamp. The two council-named replacement paths were measured: (a) **block-constant injection** ($h \leftarrow h + \Delta_{\mathrm{block}}$) fixes the *fidelity lie* (pins the mean to machine precision, FE(2)) and **nothing else** — it keeps the mass source (FE(5), identical) and converts the boundary ridge into a **grid-locked interior washboard**: its interior curvature jumps to $0.267\,\mathrm{m}$ ($5\times$ pin-off's $0.055$) from a discontinuous step at every $B$-cell block boundary, which is what flatters its ratio while its absolute seam curvature *worsens* (FE(3)). **Convicted.** (b) Leaf-only evolution + a single-valued **face register** ( #form-face-flux-register ) — the injection+refluxing pair — remains the admissible fix, out of scope here. **Deletion is the measured-best executable step**: lowest absolute seam ($0.274$), lowest interior curvature ($0.055$), no manufactured mass. Storing state detail coefficients alone does **not** kill the seam (naive zero-outside detail **worsens** the ratio to $33.12$).

## Epistemic Status

**Max attainable: exact** for the measurements under the spike harness (re-based land footprint; land guard PROBE 0 against derived sea). **Currently `exact`** as observation of the operator's defects. **The retire-or-replace recommendation is council-accepted and the retirement is executed** (`DECISIONS[mean-pin-manufactures-the-seam-and-the-mass]` :council; execution + replacement verdict in `DECISIONS[mean-pin-retirement-executed-block-const-convicted-deletion-is-the-fix]`, `:status proposed` — the *direction* was accepted, this *outcome* is proposed): the replacement experiment ran and **block-constant injection is convicted** (FE(8)); deletion is the executable step. The algebra aspiration in #form-rl-closure-algebra is **not** softened by this observation — strengthen-before-soften: the law stands; the implementation is convicted.

Stage `draft`. Primary instrument: `msc/spike-wavelet-store/` PROBE 6–7 + `RUN.txt` (`PinMode::{Bilinear,Off,BlockConst}`); code `erosion.rs::{pin_block_means, pin_block_means_const}` (convicted, no production caller).

## Discussion

A green in-tree guard with a $2\,\mathrm{m}$ tolerance sized to the residual is the same failure species as #norm-probe-sensitivity : the instrument cannot convict what it was sized to hide. That specimen was live in-tree (`erosion.rs::pin_preserves_parent_means`, tolerance $2\,\mathrm{m}$, "means match approximately") and is now restated to **convict** (`bilinear_pin_does_not_preserve_means_block_const_does`: block-const pins to machine precision, bilinear does not, by orders of magnitude — a self-calibrating relative gap that cannot be re-tuned into a false green). The zero-physics control is the body-signal that ends the story that the ridge is "just differential erosion"; the block-const *ratio* (below pin-off while its absolute seam is worse) is the body-signal for the ratio-denominator trap.

## Working Notes

- **Owns measurements;** #form-rl-closure-algebra keeps a short compliance-debt pointer and may drop duplicated residual numbers once this slug is stable.
- **Do not re-claim:** "mean-pin enforces $R\circ L=\mathrm{id}$ live"; "storing details makes the seam an identity"; injection-alone (bilinear *or* block-const) as seam fix; the stale RUN.txt magnitudes (5.77 / 1.93 / +0.136%) as current.
- **Dual-home demote:** ARCHITECTURE §1 mean-pin residual block; multiscale-methods graduated (`.super-archive/from-theory/`); multiscale-seams corrected lines; DECISIONS long forms remain history.
- **Open code residual RESOLVED:** the bilinear-vs-block-const-vs-delete experiment ran (2026-07-24); block-const convicted, deletion is the fix, no production caller to delete *from* (docstring + self-certifying test were the live footprint, both handled). The remaining forward residue is the admissible replacement — leaf-only + #form-face-flux-register — owned by the cross-face worktree spike, not this operator.
