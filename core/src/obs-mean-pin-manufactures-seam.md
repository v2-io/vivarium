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

`erosion.rs::pin_block_means` — the operator written to enforce coarse↔fine consistency — is the single largest manufacturer of the `seam_ridge` it was meant to prevent, does **not** preserve block means, and is a **mass source**. The algebra law $R \circ L = \mathrm{id}$ stands; this observation convicts the **implementation** sold as that law.

## Formal Expression

1. **What the operator does.** `pin_block_means` computes a per-block delta (coarse target minus fine block mean) and **bilinearly upsamples** that delta onto the fine tile. It is **not** a block-constant injection of the form $h \leftarrow h + \Delta_{\mathrm{block}}$.
2. **It does not pin the mean.** After the operator, $|\text{block mean} - \text{coarse target}|$ retains median $0.271\,\mathrm{m}$, max $1.986\,\mathrm{m}$ (PROBE 6, worst-case composition). A bilinear upsample of a piecewise-constant field does not preserve block means; the residual is the $(1,6,1)/8$ stencil (curvature of the delta). So $R \circ L = \mathrm{id}$ **on the mean is false in the code**, not merely lossy ( #form-rl-closure-algebra compliance debt — this segment is the measurement home).
3. **It manufactures the seam ridge.** At macro $80$ epochs / fine $150$ epochs (PROBE 6, reproduces `seam_ridge` class numbers $\sim 2.47/3.78/5.77$):

   | Composition | Seam ratio $\mathrm{seam}/\sqrt{\mathrm{out}\cdot\mathrm{in}}$ |
   |-------------|---------------------------------------------------------------:|
   | Today (pin ON) | $5.77$ |
   | Pin OFF | $2.03$ |
   | + up-propagate fine coarse coeffs (injection) | $3.90$ |
   | + outside detail $\equiv 0$ (naive wavelet store) | $21.99$ |
   | Pin ON + outside detail $\equiv 0$ | $33.39$ |

   Removing the pin and changing nothing else cuts the ridge by more than half. Injection alone (fine→coarse without pin) is **worse** than pin-off ($3.90$ vs $2.03$) — Berger–Colella need injection **and** flux reconciliation; injection alone is not the fix ( #form-face-flux-register ).
4. **Zero-physics control.** Both tiers at **zero** epochs (no erosion, no differential aging, no carved detail): seam ratio **$1.93$ with pin ON**, **$1.10$ with pin OFF**. The tile machinery manufactures a $\sim 2\times$ ridge with **no physics in the world**; mean-pin is the whole of that excess.
5. **It is a mass source.** $\int h\,\mathrm{d}A$ of the fine tile before → after pin:

   | Fine epochs | Relative $\Delta$ mass |
   |------------:|-----------------------:|
   | 18 | $+0.0151\%$ |
   | 80 | $+0.0748\%$ |
   | 150 | $+0.1360\%$ |

   The operator re-creates rock erosion carved away because the coarse tier it pins to has eroded less. Magnitude grows with fine work — the harder the fine tier works, the more mass pin invents.
6. **Age drivers (corrected reading).** The seam ratio is monotone in the **fine-tier additional epochs** (the true differential gap when fine is seeded from already-eroded macro), not in a misread $|\text{fine}-\text{macro}|$. At **fixed** fine gap $150$, raising macro $18\to 80\to 150$ takes the ratio $4.29 \to 5.77 \to 8.56$ — a **second driver**: absolute roughness of both tiers.
7. **Category error inside the operator.** Pin compares fine **block means** to a coarse **point sample** at block centre — the cell-average / point-sample fork ( #form-column-control-volume family) live inside the conservation path.
8. **What this licenses.** Retiring or replacing mean-pin is a **deletion/repair** prerequisite for honest multiscale composition — not a cosmetic clamp. Replacements must be judged against FE(3)–(5): leaf-only + face register ( #form-face-flux-register ), or an honest injection+refluxing pair that does not bilinear-smooth deltas. Storing state detail coefficients alone does **not** kill the seam (naive zero-outside detail **worsens** the ratio).

## Epistemic Status

**Max attainable: exact** for the measurements under the spike harness (reproduces seam_ridge class; land guard PROBE 0). **Currently `exact`** as observation of the live operator's defects; **proposal to retire** remains unratified (`DECISIONS[mean-pin-manufactures-the-seam-and-the-mass]`, `:by claude`, proposed). The algebra aspiration in #form-rl-closure-algebra is **not** softened by this observation — strengthen-before-soften: the law stands; the implementation is convicted.

Stage `draft`. Primary instrument: `msc/spike-wavelet-store/` PROBE 6–7 + `RUN.txt`; live code `erosion.rs::pin_block_means`.

## Discussion

A green in-tree guard with a $2\,\mathrm{m}$ tolerance sized to the residual is the same failure species as #norm-probe-sensitivity : the instrument cannot convict what it was sized to hide. The zero-physics control is the body-signal that ends the story that the ridge is "just differential erosion."

## Working Notes

- **Owns measurements;** #form-rl-closure-algebra keeps a short compliance-debt pointer and may drop duplicated residual numbers once this slug is stable.
- **Do not re-claim:** "mean-pin enforces $R\circ L=\mathrm{id}$ live"; "storing details makes the seam an identity"; injection-alone as seam fix.
- **Dual-home demote:** ARCHITECTURE §1 mean-pin residual block; multiscale-methods graduated (`.super-archive/from-theory/`); multiscale-seams corrected lines; DECISIONS long forms remain history.
- **Open code residual:** replace bilinear delta with block-constant injection *or* delete pin under leaf-only — experiment can fail; not decided here.
