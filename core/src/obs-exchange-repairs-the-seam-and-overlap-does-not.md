---
slug: obs-exchange-repairs-the-seam-and-overlap-does-not
type: observation
status: exact
stage: draft
depends:
  - obs-tile-outlets-grade-away-the-basins
  - obs-lakes-are-routed-over-not-carved-away
  - form-declared-boundary-contract
  - form-seam-flux-exchange
  - form-depend-by-key-never-latest
  - norm-probes-before-claims
  - norm-probe-sensitivity
  - disc-unlawfulness-budget
---

# A tile halo repairs the seam only if it is exchanged; enlarging it and throwing it away makes the seam worse, and a millimetre bounds what either can promise

Carving each tile on an enlarged window and discarding the surplus — a halo that reads no neighbour — leaves the block **further** from a single-field carve than the shipped tiling does. Refreshing that halo from the neighbours every few epochs brings the seam signature and the mean elevation onto the single-field values. Underneath both sits a chaos floor: $+1\,\mathrm{mm}$ on one cell $96$ cells away decorrelates $96.6\,\%$ of a core in $300$ epochs.

## Formal Expression

1. **The setting.** One L13 footprint (f1, origin $(640, 5376)$, $256^2$ cells, $313\,\mathrm{km}$, seed 17425063241017297386, $300$ epochs), carved as $4\times4$ tiles of $64^2$ and compared against the same footprint carved as one $256^2$ field. Every arm has the **same outer perimeter under the same contract**, so nothing here compares boundary contracts ( #obs-tile-outlets-grade-away-the-basins does that); what varies is only what happens at the **internal** seams. Rain is normalized by one constant across arms, so the shipped path's per-tile renormalization is deliberately absent and cannot confound the comparison.

2. **Measured: a chaos floor, and it is not small.** Raising **one cell by $1\,\mathrm{mm}$** in the outermost ring of a $256^2$ window — $96$ cells from the $64^2$ core that is read out, a perturbation with no physical content that a halo of any depth would have absorbed — moves that core by:

   | epochs | 50 | 100 | 150 | 200 | 250 | 300 |
   |---|---|---|---|---|---|---|
   | mean $\lvert\Delta h\rvert$ (m) | 0.00 | 1.63 | 3.69 | 11.23 | 13.24 | 13.61 |
   | max $\lvert\Delta h\rvert$ (m) | 2 | 415 | 348 | 395 | 394 | 395 |
   | cells bit-identical | 90.2 % | 14.2 % | 5.3 % | 3.8 % | 3.4 % | 3.4 % |

   The front takes $\approx 100$ epochs to cross $96$ cells and then saturates. **Pointwise agreement between a tiled carve and a single-field carve is therefore not an achievable target at this grain**, and no halo design may promise it: the kernel amplifies a millimetre into $395\,\mathrm{m}$ and $96.6\,\%$ decorrelation. What remains promisable is agreement in *structure* — and the same control prices that too: the core's **mean elevation** differs from its unperturbed twin by $0.9\,\mathrm{m}$, so mean elevation is resolvable to about a metre where cell-by-cell agreement is not resolvable at all.

3. **Measured: the boundary's influence is an order of magnitude above that floor, so it is a real signal and not chaos.** The same core carved standalone ($d = 0$) against carved inside a $96$-cell halo differs by $188.09\,\mathrm{m}$ mean at $300$ epochs — $13.8\times$ the chaos floor, and $34.6\times$ it at $100$ epochs. The measurement below is about boundaries, not about noise, and the ratio is what establishes that.

4. **Measured: the influence is a band early and stops being one.** Boundary influence divided by the local carve magnitude ($\lvert h - \text{prior}\rvert$ in the same ring — the denominator that stops a trunk in the middle of a core reading as boundary influence), by Chebyshev ring depth into the $64^2$ core:

   | ring depth | 0 | 1 | 2 | 4 | 8 | 16 |
   |---|---|---|---|---|---|---|
   | 50 epochs | 1.030 | 0.826 | 0.498 | **0.249** | 0.393 | 0.365 |
   | 300 epochs | 1.044 | 0.810 | 0.695 | 0.696 | 1.083 | **1.450** |

   At $50$ epochs the profile decays into the interior by a factor of four — a provisional band with a final interior behind it. At $300$ epochs there is no decay and the **deepest ring is the worst**, exceeding its own carve magnitude by half. A finite influence width $w(E)$ exists while the front is still travelling and ceases to exist once it has arrived, because what arrives is not a wave amplitude that decays but a reorganization of the drainage network, which has no far field.

5. **Measured: halo depth alone converges the mean elevation, slowly, as roughly $1/d$.** Core mean elevation at $300$ epochs against the widest arm, with the chaos floor beside it:

   | halo $d$ | 0 | 4 | 8 | 16 | 32 | 64 | 96 (ref) | 1 mm twin |
   |---|---|---|---|---|---|---|---|---|
   | mean $h$ (m) | 6320.7 | 6315.0 | 6272.8 | 6223.0 | 6186.5 | 6185.3 | 6149.3 | 6148.4 |
   | error vs ref (m) | 171.4 | 165.7 | 123.5 | 73.7 | 37.2 | 36.0 | — | 0.9 |

   Halving per doubling of $d$ from $8$ to $32$, then stalling at $\approx 36\,\mathrm{m}$ where the reference's own boundary error takes over. This is a decay, not a cutoff: no depth in the sweep makes an interior final.

6. **Measured: one-shot overlap is not a repair — it is worse than the shipped tiling.** Assembled block against the single-field reference, whose own values are the target (`seam step` is mean $\lvert\Delta h\rvert$ across the $64$-cell lines over mean $\lvert\Delta h\rvert$ elsewhere, subaerial links; `cost` is compute per unit kept area, $((n+2d)/n)^2$):

   | arm | mean $\lvert\Delta h\rvert$ (m) | seam step | mean $h$ (m) | cost |
   |---|---|---|---|---|
   | **REF** — one $256^2$ field | — | **1.034** | **6128.8** | — |
   | PLAIN — $16$ tiles, shipped | 117.92 | 0.614 | 6092.6 | 1.00× |
   | OVERLAP $d=4$ | 134.98 | 1.745 | 6063.4 | 1.27× |
   | OVERLAP $d=8$ | 139.31 | 1.829 | 6052.3 | 1.56× |
   | OVERLAP $d=16$ | 145.17 | 1.937 | 6093.2 | 2.25× |
   | OVERLAP $d=32$ | 133.07 | 1.398 | 6072.3 | 4.00× |
   | EXCHANGE $d=4$, $\sigma=10$ | 108.22 | 0.917 | 6086.9 | 1.27× |
   | EXCHANGE $d=8$, $\sigma=10$ | 100.31 | **1.011** | 6096.6 | 1.56× |
   | EXCHANGE $d=16$, $\sigma=10$ | 114.94 | **1.040** | 6124.0 | 2.25× |
   | EXCHANGE $d=32$, $\sigma=10$ | 89.95 | 1.085 | **6127.0** | 4.00× |
   | EXCHANGE $d=8$, $\sigma=50$ | 110.58 | 1.076 | 6079.9 | 1.56× |
   | EXCHANGE $d=32$, $\sigma=50$ | 98.88 | 1.117 | 6087.6 | 4.00× |

   Overlap moves every column the wrong way at up to four times the compute. The mechanism is visible in PLAIN's own seam step of $0.614$, which sits **below** the reference's $1.034$: an edge-sink tile grades its perimeter flat, so the shipped seam is a small height difference between two flattened strips. Enlarging the window and discarding the surplus removes that flattening — correctly, it was an artefact — while supplying nothing in its place, so two independently carved interiors now meet at their true disagreement and the ratio overshoots to $1.4$–$1.9$.

   **The depression census the probe also prints is deliberately absent from this table, because at this footprint it cannot discriminate.** Every arm, reference included, shares one outer perimeter of edge sinks over a window with no coast, and a wall-contract reader denies that perimeter; the census is therefore dominated by that shared ring ($41\,352$ of $65\,536$ cells on the reference itself) and moves by a few percent across arms that differ by a factor on the seam step. It is the same quantity #obs-tile-outlets-grade-away-the-basins FE(5) reads at $63.5\,\%$ for a tiled beacon window, and reading it here would be reading the outer contract, not the internal seams.

7. **Measured: exchange lands on the reference, and cadence buys more than depth.** Refreshing each tile's halo from the assembled neighbours every $\sigma$ epochs puts the seam step at $1.011$ ($d=8$) and $1.040$ ($d=16$) against the reference's $1.034$, and the mean elevation within $1.8\,\mathrm{m}$ ($d=32$) and $4.8\,\mathrm{m}$ ($d=16$) of the reference's, against $36.2\,\mathrm{m}$ for the shipped tiling and $36$–$77\,\mathrm{m}$ for every overlap arm. Both are structural statistics with a chaos floor near a metre (FE(2)), so both differences are resolved. Comparing the two dials at equal cost: $\sigma{=}50 \to 10$ at $d=8$ moves mean $\lvert\Delta h\rvert$ from $110.58$ to $100.31$ **for no extra compute at all**, while $d{=}4 \to 32$ at $\sigma=10$ moves it from $108.22$ to $89.95$ for $3.2\times$ the compute. **Exchange frequency is the cheap dial and halo depth is the expensive one.**

8. **Measured: the exchange discipline is forced, and the wrong form fails catastrophically rather than marginally.** Building the same block under a forward and a reverse tile sweep ($d=8$, $\sigma=10$, $50$ epochs, $16$ tiles):

   | form | cells differing (of 65 536) | mean $\lvert\Delta h\rvert$ | max |
   |---|---|---|---|
   | **Jacobi** — every tile reads one frozen snapshot | **0** | $0$ | $0$ |
   | Gauss–Seidel — each tile published as produced | 61 724 | $130.7\,\mathrm{m}$ | $2289\,\mathrm{m}$ |

   Jacobi is bit-identical under any tile order; the Gauss–Seidel control differs on $94\,\%$ of the block by an amount the size of the signal itself. The order-independence that #form-depend-by-key-never-latest FE(1) requires is available and cheap, and the natural implementation destroys it.

## Epistemic Status

**Max attainable: exact** for the quoted measurements under `examples/halo_band_probe` and `examples/halo_exchange_probe`. Both open no store and write nothing: initial topography, uplift and precipitation jitter are pure functions of seed and cell, so every arm is reproducible from the printed header alone.

**Currently `exact` as observation.** Every clause is falsifiable and the falsifiers are cheap: a perturbed twin that stays bit-identical refutes FE(2); a boundary influence within a factor of two of the chaos floor refutes FE(3) and would void FE(4)–(7) as evidence about boundaries; a ring profile still decaying at $300$ epochs refutes FE(4); a mean elevation not ordered in $d$ refutes FE(5); an overlap arm beating PLAIN on seam step refutes FE(6); an exchange arm no closer to the reference than its overlap sibling refutes FE(7); a single differing bit between the Jacobi sweeps refutes FE(8).

**Probe sensitivity** ( #norm-probe-sensitivity ). FE(8)'s bit-identity assertion is the shape that passes vacuously — against pure functions of a fixed schedule, "two orders agree" can be true because nothing in the probe could have made them disagree. The Gauss–Seidel arm exists to convict the check: it is the same code path with the snapshot published per tile instead of per sweep, it *is* order-dependent, and the probe asserts that it is. A green Jacobi line therefore means something the assertion alone would not have meant. FE(2) plays the same role for the whole segment — without it, FE(6)'s "overlap does not converge" could not be distinguished from "nothing converges."

**Scope, and therefore not claimed.** One seed, one footprint, one level (L13), one epoch count for the block sweep, one tile grain, one kernel parameterization. The L9 planet regime is **not** measured here and is expected to differ, since #obs-lakes-are-routed-over-not-carved-away FE(7) finds the two contracts tracking within $10\,\%$ there — a repair measured at the grain where the defect is large says nothing about the grain where it is small. The seam-step statistic is **not sign-stable across regimes**: at L9 the tiling *raises* it above the uncarved null ( #obs-lakes-are-routed-over-not-carved-away FE(8)), while here the shipped tiling reads *below* the single-field reference, for the mechanism FE(6) gives. Read it as a distance from the reference's own value, never as a quantity with a fixed good direction.

**Not measured, and named rather than implied:** what exchange does to the $3.25\times$ starved incision driver ( #obs-tile-outlets-grade-away-the-basins FE(4)). The exchange arms pass **bed elevation**, and a halo of depth $d$ grows a tile's accumulating catchment by at most $d$ cells; the upstream discharge entering across a seam from beyond the halo is not supplied by any arm here. That is the second of the two repairs #form-same-level-halo-exchange separates, and this segment measures only the first.

Stage `draft`.

## Discussion

The result inverts the cheapest hope. A halo that reads no neighbour keeps a tile a pure function of its own coordinates — no dependency to fold into a key, no cone, no build-order question — and it would have been the whole repair for the price of a wider window. It does not work, and the reason it does not work is worth more than the fact: the shipped seam was *quiet* rather than *small*, flattened by the same edge-sink grading that starves the tile. Removing an artefact that was concealing a disagreement makes the disagreement visible, which reads as a regression and is in fact the first honest measurement of it.

The chaos floor changes what a repair may claim. It is tempting, having found that a boundary reaches everywhere, to conclude that tiles cannot be carved independently at all. FE(3) is what stops that: the boundary's signal stands an order of magnitude clear of the floor, so the defect is real and repairable — but it must be repaired in the currency of structure, because the currency of cell-by-cell identity is not available to anyone, including a whole-face carve compared against itself plus a millimetre.

## Working Notes

- **Instruments.** `examples/halo_band_probe` (one core, growing halo, chaos twin; $\approx 11$ s) and `examples/halo_exchange_probe` (block arms, order-independence gate; $\approx 145$ s). Both store-free. Knobs are env and printed.
- **Forward, in the order the evidence ranks them:** (a) the same block sweep at L9, where the defect is small — if exchange also lands on the reference there, the repair is grain-independent and the design has one form; (b) $\sigma$ below $10$ and the $\sigma \to 1$ limit, since cadence is the cheap dial and its floor is unmeasured; (c) flux injection at the seam, which is the untested half named in Epistemic Status and the one that would move the $3.25\times$.
- **Regression guard — do not re-propose one-shot overlap as the cheap repair.** It is the first thing a competent reading of "the boundary reaches $d$ cells in" suggests, its key story is genuinely free, and it is measured worse than doing nothing on all three statistics at up to $4\times$ the compute. FE(6) is the reason, and the reason generalizes: displacing an artificial boundary without coupling to what is on the other side of it removes a concealment, not a defect.
- **Do not read the $\sigma$ column as a convergence rate.** These are two cadences at one epoch count, not a study; that $\sigma=10$ beats $\sigma=50$ on every arm is the finding, and the shape of the dependence is not.
- **External re-read of FE(7), stress-tested (dossier 2026-07-29 §2.2–2.3 + adversarial re-analysis same day; suggestive-not-law):** mean-elevation error across all six exchange arms is strictly monotone in $d/(v\sigma)$ (with $v \approx 1$ cell/epoch inferred from FE(2)'s front) — arithmetic verified twice. What the six points *resolve* is less than that sentence carries. Refuted with margin: $d$ alone (the two $d{=}32$ arms differ 41.2 vs 1.8 m across $\sigma$), $\sigma$ alone (a 40 m spread within $\sigma{=}10$), and the diffusive exponent $d/\sqrt{\sigma}$ (ordering inverted by 9.0 m, ten times the FE(2) floor — exactly the wrong-exponent law a Schwarz-shaped reading would have imported, and the one alternative these data genuinely exclude). Not resolved: within the family $d/\sigma^p$, every $p \gt 0.86$ — and the difference form $d - \sigma$ — orders the six arms indistinguishably from the ratio; the only separating comparison is a 0.7 m transposition of the $(32,50)$ and $(4,10)$ arms, under the 0.9 m floor. So the resolved content is *one joint dial, increasing in $d$, decreasing in $\sigma$, relative exponent above 0.86* — not *the group is $d/(v\sigma)$*. The collapse lands somewhere in ratio 0.8–1.6, a factor-two gap that contains the causal-cone threshold 1 but does not locate it, and $v$ is inferred, not measured. Falsifier unchanged and still owed: matched-ratio arms on `halo_exchange_probe` (equal error across $(4,5)/(8,10)/(16,20)$ confirms ratio and refutes the difference form; the collapse ratio locates $1/v$; the floor height sizes the FE(9) defect). Full reading: `ref/research/seam-exchange-precedents-dossier.md` §2.2–2.3, §4.1; forward item (b) above is subsumed by this probe design.
- **Mean $\lvert\Delta h\rvert$ not organizing the ratio is evidence about FE(2), not about the group.** FE(2)'s floor predicts every arm's pointwise column saturates regardless of the schedule, so the column has no discriminating power for or against $d/(v\sigma)$ — had it organized, that would have *refuted the chaos floor*. Read it as a passed check of FE(2) and zero evidence toward the ratio claim; crediting it to the group (as "consistency") lends the group weight the column cannot carry.
