---
slug: obs-the-two-point-defect-lives-on-the-panel-seam
type: observation
status: exact
stage: draft
depends:
  - form-grid-equiangular-staggered
  - norm-bias-vs-noise
  - norm-probe-sensitivity
---

# The two-point flux defect lives on the panel seam

The measured order $-0.5$ of the two-point flux on the equiangular cube-sphere is not a bulk property: ~99.5% of the whole-sphere $L^2$ error sits on the single ring of cells touching a cube-panel seam, growing at $O(1/h)$ locally while every interior band converges — so any scheme keeping a two-point flux across a panel seam has a divergent operator there, with no hanging node required.

## Formal Expression

1. **The decomposition (measured).** Two-point-flux Laplacian error against an exact degree-2 spherical harmonic, split by distance-in-cells from the nearest cube-panel edge (`examples/tpfa_ladder.rs`, N = 32–256/face): **band 0 (touching the seam) refines at order $\approx -0.51$** at every rung, while bands 1–2, 3–8, and 9–32 all reach **positive** orders (≈ +0.47/+0.41/+0.24 by N = 256). The seam ring carries ~99.5% of the whole-sphere $L^2$ defect; the face interior alone plateaus (≈ 19% relative error, order → 0).
2. **The local rate, and why the global order is exactly $-1/2$.** Band-0 *local* relative error is $O(1/h)$ — 2.5 → 5.1 → 10.3 → 20.6, doubling per rung. The clean global $-0.5$ is then arithmetic, not mechanism: an $O(1/h)$ error on a codimension-1 set contributes $h^{-1/2}$ to a 2-D $L^2$ norm.
3. **It is not bad seam geometry.** Seam-cell edge lengths, centre distances, and skew are within ~2% of interior values and converge with them; only non-orthogonality is elevated (29.7° vs 11.3° at N = 256), and that is an $O(1)$ quantity — the same magnitude the bulk carries without blowing up — so it cannot alone produce $O(1/h)$ growth. **Mechanism, held at hypothesis tier:** the metric *jump* across the seam breaks the error cancellation that keeps the bulk two-point error bounded. The measurement convicts the location; it does not yet convict the mechanism.
4. **Consequence.** A two-point flux across a cube-panel seam is a **divergent operator on that seam regardless of bulk behaviour** — and this requires no refinement mismatch: it is present on conforming, same-level seams. It is therefore a *third* independent way the seam is broken, beside Lipnikov's coarse–fine non-convergence and the bulk non-orthogonality inconsistency ( #form-face-flux-register Working Notes), and the only one of the three present without hanging nodes or non-orthogonal bulk.
5. **What this sharpens.** The grid report's §5.2 account (a global "$R/h$ doubles every refinement" mechanism, tagged `[D]` there) is superseded as the explanation of the $-0.5$: three planar controls (orthogonal, uniform-shear, varying-shear — the last two ABF-exempt parallelogram lattices) all *saturate* at $O(1)$ rather than grow, so bulk metric behaviour cannot produce the measured growth. This does **not** conflict with that report's refutation of "the 24 corner defects drag the norm" — that split was by distance from the 8 corners, not the 12 panel edges. The report's measured numbers stand; only the mechanism attribution moves.
6. **Bias, not noise.** The defect is signed, level-fixed in location, and grows under refinement — the manufactured-law class ( #norm-bias-vs-noise ), landing exactly where cube-locked structure would be read as geography.

## Epistemic Status

**Max attainable: exact** for the decomposition under its harness. **Currently `exact` for FE(1)–(2) and FE(6)** (measured, and independently re-run by a second session before landing — the numbers above reproduce); **FE(3)'s mechanism clause is `hypothesis`** (the metric-jump inference is flagged as inference by its own author); FE(4) is the direct consequence of FE(1)–(2); FE(5) is measured on the controls named.

**Scope:** one scheme (the two-point flux), one operator (Laplacian), one exact solution (degree-2 spherical harmonic), face-interior + seam-band split at N = 32–256 (interior ladder to N = 1024 in the companion probe); no multi-seed variation (the probe is deterministic geometry, not fated fields). The finding was reached by a **mispredicted pre-registration** — the author predicted the $-0.5$ was pre-asymptotic and measured it persistent, and the resolution of that contradiction is this segment (pre-registration trail: `msc/agent-briefs/2026-07-29-gcl-and-abf-checks.md` §0, A4/A5).

**Decisive test, named and unrun:** freeze the metric across a band either side of one seam and re-run — if the growth vanishes, the metric jump is the mechanism; if not, FE(3)'s hypothesis dies and the location conviction stands alone.

Stage `draft`.

## Discussion

The router remedy stack and the seam law both gain a constraint: FE(6)(c)'s vector reconstruction and any future staggered-FV scheme must treat the panel seam as its own object, because the defect is not cured by bulk fixes (true metrics, wide stencils) that leave the cross-seam flux two-point. The finding also relocates where "the sphere is brutally real": not in the bulk shear the planar controls emulate, but at the twelve lines where charts meet — the same lines the cube-locked control ( #obs-cube-locked-kernel-bias FE(5)) already watches for manufactured geography.

## Working Notes

- **Instrument:** `crates/vivarium-world/examples/tpfa_ladder.rs` (live, deterministic, re-runnable; reproduced 2026-07-29 by the landing session). Companion separations: `examples/gcl_abf_probe.rs` (ABF refutation + GCL identity — `DECISIONS[abf-quadrilateral-trap-does-not-apply-to-us]`, `DECISIONS[the-discrete-gcl-is-a-spec-not-a-defect]`).
- **Do not re-claim:** "the −0.5 is bulk non-orthogonality" (controls refute); "ABF 2005 explains our measurement" (refuted — see the DECISIONS entry); "the corner defects drag the norm" (already refuted upstream, different split).
- The decisive metric-freeze test is the cheapest next step and belongs to whoever next touches the seam family.
