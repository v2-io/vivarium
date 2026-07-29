# Routing pricing experiment — RESULTS

*Ran 2026-07-24. Harness: `crates/vivarium-world/examples/router_pricing.rs`
(read-only w.r.t. live kernels; reimplements the fluvial pipeline verbatim,
swaps only the drainage router). Predictions committed first:
`PREDICTIONS.md`. Raw console: `RESULTS-run.txt`.*

## The question (from the law)

`DECISIONS[routing-violates-the-potential-identity-and-the-replacement-does-not-fix-it]`
/ `#obs-routing-curl-spiral` FE(8) quarantined the ~2% spiral's **severity** behind
one experiment: *eroded landscape with vs. without the corrected router — does the
channel network differ under identical forcing/seed?* This prices it.

## What the live kernel was doing WHEN measured (moving target — read this)

Mid-experiment, a peer landed commit **`6c1ad97` "Fan-path lengths: true
great-circle neighbour distances retire uniform cell_m"** — the live `erosion.rs`
`accumulate_drainage`/`receivers`/`incise`/`talus` now route through a true
great-circle `dist_m(a,b)`, retiring the uniform `cell_m`/`√2·cell_m` lengths.
So "the corrected router" is **not one object** — it is a stack being built live.
The experiment therefore prices two named components separately, both against the
**current live kernel** (`LiveMfd`) as baseline:

| arm | what it is | relation to live |
|---|---|---|
| `UniformOld` | 8-nbr fan, **uniform** lengths everywhere | == `erosion.rs` **before** `6c1ad97` |
| `LiveMfd` | 8-nbr fan, **true gc** lengths everywhere | == `erosion.rs` **today** (baseline) |
| `EdgeTrue` | current live, drainage **fan** is edge-only (kill diagonals) | diagonal-kill, **fan half only** |
| `EdgeFull` | edge-only **fan AND** edge-only D4 **receiver/incision tree** | diagonal-kill, **full FE(6b) flux surface** |

`UniformOld → LiveMfd` prices **the length fix that just shipped**.
`LiveMfd → EdgeTrue` prices **the fan-only diagonal-kill marginal**.
`LiveMfd → EdgeFull` prices **the full FE(6b) diagonal-kill** (fan + receiver tree). Talus
(hillslope mass movement, not channelised flux) and depression-fill connectivity stay 8-way
in every arm — the diagonal-kill is applied to the *flux* surface only.

## Validity gates (all PASS)

- **P0 — harness `LiveMfd` reproduces live `erosion::Fluvial::erode` bit-for-bit**,
  3× at both sites (off-centre corner + face centre). max|Δh| = max|Δdrainage| = 0.
  *Until this passed, every downstream number was void — and on the first run,
  against the stale (pre-`6c1ad97`) kernel copy, it correctly FAILED (max|Δh|≈150 m)
  and stopped me. The bit-match is what makes this about the live world.*
- **Determinism** — corrected-arm drainage bit-identical across 3 reruns (the
  HashMap-order self-catch guard from the curl-probe work).
- **Not a no-op** — land fraction 88.7% (off-centre) / 88.2% (centre); channels form
  (2769 / 2902 channel cells at τ=20). Not the submarine no-op trap.
- **Null-test discipline** — terrain placed **off face centre** (corner, origin
  (100,100) at L19), with the **face-centre run as the D4 acquittal control**
  (`#norm-probe-sensitivity`). A face-centred probe alone would have acquitted the
  length bias — see below.

## The load-bearing result: the null-test differential

A router's **cube-locked** channel-orientation effect = (arm − live at the corner)
**minus** (arm − live at the face centre). The face-centre term subtracts the
*generic* (non-cube) part of the routing change; what survives is the bias that
**vanishes under D4 symmetry** — the topological defect the decision priced.
Metric: channel-cell steepest-descent **axis-fraction** (share of channel steps that
are cardinal N/S/E/W rather than diagonal).

```
CUBE = (arm−live axis-fraction at corner) − (arm−live at face centre)
tau |  OLD (length fix) |  EDGE (fan diag-kill) |  FULL (fan + receiver-tree diag-kill)
 20 |         +0.1668    |        −0.0322        |         +0.0701
 50 |         +0.1003    |        −0.0402        |         +0.1479
100 |         +0.2045    |        −0.0157        |         +0.2552
```

**Reading (three distinct present truths):**

1. **The length fix (6c1ad97) had a large, clean, cube-locked landscape
   consequence.** `OLD` (the retired kernel) over-populated the grid axes by
   **+0.10 to +0.20 axis-fraction at the corner**, and that excess is **≈0 at the
   face centre** (raw: +0.0001 / −0.0009 / −0.019) — the textbook null-test
   signature. The pre-`6c1ad97` kernel manufactured 10–21 percentage points of
   excess grid-axis channel alignment at the corner; `6c1ad97` removed it.
   **Severity of the length component: REAL, and now remedied.** A face-centred
   probe would have seen the ≈0 and wrongly acquitted it — the null-test trap,
   demonstrated.

2. **Killing diagonals in the FAN adds little further cube-locked bias.**
   `EDGE`'s CUBE column is **−0.02 to −0.04** — near zero. `EdgeTrue` *does* differ
   from live substantially (M1 Jaccard 0.60–0.86, present at the face centre too),
   but that is **generic MFD-4-vs-MFD-8 routing**, not the cube-locked defect: it
   does not vanish at the face centre. **The fan half of the diagonal remedy is
   landscape-benign.**

3. **Killing diagonals in the RECEIVER/INCISION TREE is landscape-consequential —
   and a naive D4 tree is itself NOT cube-safe.** `FULL`'s CUBE column is **+0.07 to
   +0.26** — comparable to the length fix. The receiver-tree diagonal-kill produces
   *more* axis-aligned channels at the corner (raw Δaxis up to +0.09) and *less* at
   the centre (raw Δaxis down to −0.16): a large cube-position-dependent orientation
   swing. This is the naive-D4 proxy trading the fan's spiral for a receiver-tree
   axis-lock that itself swings with the cube Jacobian. **So the diagonal remedy's
   landscape action lives in the flux TREE, not the fan — and removing diagonals
   naively (D4 steepest descent) is not a cube-safe fix.** The principled remedy
   (Coatléven edge-flux FV reconstruction replacing fan *and* tree) is the object
   FE(6c) actually names, and it is **NOT** this arm — see limits.

## Supporting metrics (and why they are NOT the discriminator)

- **M1 (channel-mask Jaccard disagreement):** large for *all* arms at *both* sites
  (0.27–0.91). M1 alone would massively over-state severity — it cannot separate
  cube-locked from generic. Reported for completeness, not conviction.
- **M2 (log-drainage Spearman):** live↔OLD 0.28–0.53, live↔EDGE 0.19–0.43, live↔FULL
  0.04–0.36 (FULL reshapes drainage most, as a D4 tree would). Same caveat:
  dominated by generic reshaping.
- **M3 axis:diag anisotropy + its null-test differential (above):** the only metric
  that isolates the cube-locked component. This is the load-bearing one.

## Verdict (proposed — measurement is MEASURED-grade; severity stays proposed)

The routing spiral's **landscape consequence is NON-NULL and REAL**, and it
decomposes into three separately-priced parts:

- **Length component (shipped in `6c1ad97`): REAL, and now remedied.** Clean
  D4-verified cube-locked signal, +0.10–0.20 axis de-biasing at the corner, ≈0 at the
  face centre. The dominant *fixed* win.
- **Fan-diagonal component: landscape-benign.** CUBE −0.02 to −0.04 — killing
  diagonals in the drainage fan adds essentially no cube-locked orientation
  consequence.
- **Receiver-tree diagonal component: landscape-CONSEQUENTIAL, and unresolved.**
  CUBE +0.07 to +0.26 — comparable to the length fix. The diagonal-kill's landscape
  action lives in the **flux tree**, not the fan; and the naive D4 proxy that removes
  diagonals is **itself not cube-safe** (it axis-locks harder at the corner).

**Therefore the router-successor question is NOT de-prioritised for channel
orientation.** The correct present truth is the opposite of a bare "low-severity":
the spiral's landscape stakes are real, the length part is fixed, the fan part is
benign, but the **flux-tree diagonal treatment is consequential and the principled
remedy is unbuilt** — FE(6c)'s Coatléven edge-flux FV reconstruction (which replaces
*both* the fan and the tree with a single convictable object) is what must be priced
next, against these measured stakes. A naive D4 diagonal-kill is not the answer.

## Limits (what this does NOT convict)

1. **The full FE(6b) flux surface IS priced (fan + receiver tree, `EdgeFull`), but
   FE(6c) is NOT.** `EdgeFull`'s D4 receiver tree is a **naive** diagonal-kill —
   literal D4 steepest descent — which is a *strawman* for the true corrected router
   (Coatléven edge-flux FV reconstruction on real face fluxes, direction and magnitude
   from one object). The naive proxy shows the receiver-tree treatment is
   landscape-consequential and that a naive kill is not cube-safe; it does **not**
   measure what the *principled* remedy would do. Pricing FE(6c) requires building the
   Coatléven reconstruction — the clear next step, now with measured stakes to justify
   it. (Depression-fill connectivity and talus stay 8-way in all arms — the diagonal
   question is scoped to the flux surface.)
2. **One seed / one face / one level (L19) / one synthetic dome** (broad paraboloid +
   real band-limited prior detail). The corner footprint is **maximum shear** — a
   pricing *bound*, not a face-average. A seed/face/level sweep would tighten it.
3. **80 epochs, uniform uplift 0.02 m/epoch.** Steady-state channel geometry at other
   forcing/timescales is unmeasured.
4. The metric of record is **channel orientation** (M3). Other landscape
   consequences (relief distribution, sediment routing, catchment sizes) are only
   probed bluntly via M1/M2.

## Reproduce

`cargo run --release -p vivarium-world --example router_pricing`
(Prints P0/determinism gates, per-site M1/M2/M3, and the null-test differential.)
