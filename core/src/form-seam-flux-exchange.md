---
slug: form-seam-flux-exchange
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-rl-closure-algebra
  - form-flux-web
---

# Seams exchange fluxes, not states

A seam is a boundary in space, time, or both. What crosses is a flux of a conserved quantity (or a declared sufficient statistic of one) — never raw foreign state. Position and time are one discipline on two axes.

## Formal Expression

1. **Seam.** A *seam* is any boundary between independently evolved pieces of a world lattice: coarse tile meets fine tile; slow system meets fast system; early epoch hands off to a finer epoch. Coupling is only at the boundary.

2. **What crosses.** What crosses a seam is a **flux of a conserved quantity** (sediment volume, discharge, energy, …) or a **sufficient statistic** of that flux — integrated over space and/or averaged over time as the consumer requires. **Raw state of the neighbour is not the coupling object.** Sharing mutable state across a seam is out of bounds for multiscale honesty.

3. **What is guaranteed.** The coarse (or slow, or parent) side stores exactly the summary the fine (or fast, or child) side needs so that reconstruct-then-summarize returns it on the chosen statistics ( #form-rl-closure-algebra law (1)). When conservation is claimed, the flux **balances** at the interface (injection updates the parent mean; **refluxing** corrects the interface flux mismatch — distinct operations; the latter is largely unbuilt).

4. **One discipline, two axes.** The operator algebra is axis-agnostic: $R$/$L$/closure apply whether the argument is a space cell, a time interval, or a space-time patch. There is no separate "spatial seam law" and "temporal seam law" — the flux object is the same kind either way.
   - **Space seam** — drainage-shaped islands of interdependence (upstream catchment closure + path to base level); coupling strength is the discharge/sediment crossing the shared edge.
   - **Time seam** — multirate bands: fast sees slow as quasi-static; slow sees fast as time-averaged (Gear–Wells coupling pattern + scale-separation averaging).

5. **Relation to the flux web.** #form-flux-web is the *nomos-to-nomos* contract: matched consume/promise quantity names make a world assemble. This segment is the *scale/time boundary* contract: when two tiles or two rates meet, the *physical* object that may cross is flux (or its declared statistic), not the neighbour's full field. Both are needed; neither replaces the other.

6. **Out of bounds.** (a) Hard-coded edge-outlet / zero-inflow tiles that ignore upstream discharge (tiles are not composable). (b) Mean-pin alone sold as full conservation (injection $\neq$ refluxing). (c) Treating "the finest/latest available neighbour" as the dependency — dependency must be by key (plan doctrine; sibling claim for builder/explorer decoupling).

## Epistemic Status

**Max attainable: exact** for "exchange fluxes, not states" as Earth-system coupler and AMR practice (Berger–Oliger / Berger–Colella; Gear–Wells multirate; HMM on-demand micro) — established in primary reads summarized in `doc/theory/multiscale-seams.md`.

**Currently `robust-qualitative`:** the project formulation (one seam on two axes; drainage-shaped space seam; flux magnitude as degree) is stance-grade architecture written into ARCHITECTURE §4 and the multiscale theory pair. Dynamic-exponent $z$ reconciliation at a seam is **project coinage / conjecture**, not claimed here as established numerics. Type-4 reversion seam (detail→abstract / irreducible edit upscaling) is **open** and explicitly not on the ethereal-explorer path.

**Known compliance debt:** tile kernels still seed drainage from own area and hardcode edge-outlets; `seam_ridge` is red by design (measures the honesty gap); refluxing unbuilt. Debt does not soften the law ( #norm-declaration-must-convict ).

Stage `draft`.

## Discussion

The seam fix and tile composability are the same work: honest boundary conditions *from* spine and neighbour fluxes. Cosmetic clamps on floating mesas do not replace that. Observer-side, the fidelity invariant ( #lexicon/term/fidelity-invariant ) is this rule restated for participants — resolve spatial and temporal resolution only as fine as the most demanding present consumer needs.

## Working Notes

- **Pointer targets:** `doc/theory/multiscale-seams.md` mental model + §§1–2; ARCHITECTURE §4; `doc/plan/framework-to-status-quo.md` §3 (drainage-shaped dependency); abyssal-parity-plan Phase 3 (flux-BC tiles).
- **Do not absorb:** method zoo, $z$-from-quadtree tactical item, detail→abstract rename, full four-seam catalogue — extract later if needed.
- **Conflict guard:** do not restate superseded "flux-on-the-face makes refluxing an invariant without a bill" without the wavelet-store correction (`DECISIONS[wavelet-store-solves-the-representation-not-the-dynamics]` family).
