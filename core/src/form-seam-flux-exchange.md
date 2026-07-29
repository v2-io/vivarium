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

1. **Seam.** A *seam* is any boundary between independently evolved pieces of a world lattice: **tile meets tile at one level**; coarse tile meets fine tile; slow system meets fast system; early epoch hands off to a finer epoch. Coupling is only at the boundary.

   The first case is the one that reads as merely administrative and is not. Two tiles of *one* nomos at *one* level are independently evolved pieces, so their shared edge is a seam under this definition and every clause below binds it — a fact that went unstated here while the kernel graded each tile to its own perimeter, and that is now measured as a starved incision driver, manufactured seam pits, and two named contracts carving beds $259\,\mathrm{m}$ apart at the beacon grain ( #obs-tile-outlets-grade-away-the-basins , #obs-lakes-are-routed-over-not-carved-away ). The same-level case is also where the **two objects** of FE(2) come apart most sharply, and it has its own claim home at #form-same-level-halo-exchange .

2. **What crosses, and it is two things.** What crosses a seam is a **flux of a conserved quantity** (sediment volume, discharge, energy, …) or a **sufficient statistic** of that flux — integrated over space and/or averaged over time as the consumer requires. **Raw state of the neighbour is not the coupling object.** Sharing mutable state across a seam is out of bounds for multiscale honesty.

   Distinct from the flux, and not governed by that prohibition, is the **boundary datum** the receiving kernel needs to be well-posed at all: the field value just outside its domain that fixes the local gradient. A datum is a boundary condition, not an exchange of a conserved quantity — nothing is transported by supplying it and nothing can be double-counted — so importing a neighbour's edge elevation is not the state-sharing this clause forbids, while importing a neighbour's full interior field to compute one's own flux from is. Fluvially the two are the bed elevation (where downhill is) and the discharge and sediment crossing (how much arrives); they fail together at an unrepaired seam and are repaired by different objects.

3. **What is guaranteed.** The coarse (or slow, or parent) side stores exactly the summary the fine (or fast, or child) side needs so that reconstruct-then-summarize returns it on the chosen statistics ( #form-rl-closure-algebra law (1)). When conservation is claimed, the flux **balances** at the interface. **Injection** (fine→coarse state update) is not **refluxing** (interface flux reconciliation). At **hanging nodes** (coarse face abuts finer sub-faces), balance is carried by a **single-valued face flux register** — claim home #form-face-flux-register — not by two-sided ghost recomputation.

4. **One discipline, two axes.** The operator algebra is axis-agnostic: $R$/$L$/closure apply whether the argument is a space cell, a time interval, or a space-time patch. There is no separate "spatial seam law" and "temporal seam law" — the flux object is the same kind either way.
   - **Space seam** — drainage-shaped islands of interdependence (upstream catchment closure + path to base level); coupling strength is the discharge/sediment crossing the shared edge.
   - **Time seam** — multirate bands: fast sees slow as quasi-static; slow sees fast as time-averaged (Gear–Wells coupling pattern + scale-separation averaging).

5. **Relation to the flux web.** #form-flux-web is the *nomos-to-nomos* contract: matched consume/promise quantity names make a world assemble. This segment is the *scale/time boundary* contract: when two tiles or two rates meet, the *physical* object that may cross is flux (or its declared statistic), not the neighbour's full field. Both are needed; neither replaces the other.

6. **Out of bounds.** (a) Hard-coded edge-outlet / zero-inflow tiles that ignore upstream discharge (tiles are not composable). (b) Mean-pin alone sold as full conservation (injection $\neq$ refluxing). (c) Treating "the finest/latest available neighbour" as the dependency — dependency must be by key ( #form-depend-by-key-never-latest ). (d) Reading a same-level tile seam as a hanging node: the repair paths are different objects and are not interchangeable — see FE(8).

7. **Floating-point is the bottom-most seam.** Conservation discipline reaches down to arithmetic: an increment that must *land* (bed deadband) or must be *counted* (rain/evap bias) is a seam crime at the scale below the smallest grid seam. Where conservation is claimed, choose the accounting explicitly per site — realized-delta accounting or compensated summation — rather than trusting bare `f32` addition. (Paid-for: the twin f32 finds, 2026-07; specimen trail in `#form-rl-closure-algebra / #detail-seam-precedents` §4.)

8. **Two repair paths, and they are not the same object.** Both are seams under FE(1) and both obey FE(2), and there the resemblance stops:

   | | same-level tile seam | grain seam (hanging node) |
   |---|---|---|
   | interface | matching, cell for cell | non-matching, one coarse face to $2^k$ fine sub-faces |
   | what is wrong | the boundary datum is invented, and the inflow flux is absent | one flux is computed twice and the two copies disagree |
   | repair | exchanged halo at a declared depth and cadence, plus seam flux injection — #form-same-level-halo-exchange | single-valued face flux register — #form-face-flux-register |

   The two slugs are not synonyms, and reaching for the register at a same-level seam does not fit: there is no hanging node, nothing is subdivided, and no flux has been double-counted. What the register *does* supply to the same-level problem is its restriction law (FE(3) there, the length-weighted low-pass), which is what lets a **flux** record cross levels where a **state** halo cannot — so the level-asymmetric case, where a tile's neighbours are resolved finer than it is, belongs to the flux object and not to the halo.

## Epistemic Status

**Max attainable: exact** for "exchange fluxes, not states" as Earth-system coupler and AMR practice (Berger–Oliger / Berger–Colella; Gear–Wells multirate; HMM on-demand micro) — primary-read precedents held at #detail-seam-precedents .

**Currently `robust-qualitative`:** the project formulation (one seam on two axes; drainage-shaped space seam; flux magnitude as degree) is stance-grade architecture written into ARCHITECTURE §4 and the multiscale theory pair. Dynamic-exponent $z$ reconciliation at a seam is **project coinage / conjecture**, not claimed here as established numerics. Type-4 reversion seam is **split** ( #sketch-detail-abstract-reversion ): state up-propagation measured solved; the nonlinear closure for a non-local flux remains open — and explicitly not on the ethereal-explorer path.

**Known compliance debt:** tile kernels still seed drainage from own area and hardcode edge-outlets; `seam_ridge` is red by design (measures the honesty gap); production face-keyed flux register and leaf-only evolution unbuilt (measured in spike — #form-face-flux-register ). Debt does not soften the law ( #norm-declaration-must-convict ).

FE(1)'s same-level case and FE(8)'s split are **`exact` as scope statements** — that two tiles of one nomos are independently evolved pieces follows from the definition, and that the two interfaces differ in matching, defect and repair is checkable against the two repair segments. What is `robust-qualitative` there is the claim that the halo is the right repair, which rides on #form-same-level-halo-exchange and its one measured grain.

Stage `draft`.

## Discussion

The seam fix and tile composability are the same work: honest boundary conditions *from* spine and neighbour fluxes. Cosmetic clamps on floating mesas do not replace that. Observer-side, the fidelity invariant ( #lexicon/term/fidelity-invariant ) is this rule restated for participants — resolve spatial and temporal resolution only as fine as the most demanding present consumer needs.

## Working Notes

- **Dual homes demoted:** multiscale-seams header + mental model; ARCHITECTURE §4; framework-to-status-quo §3; abyssal-parity checklist item 4; DESIGN-REDUX §5 join. Phase-3 flux-BC tile plan remains build trail.
- **Do not absorb:** method zoo (graduated `#form-rl-closure-algebra / #detail-seam-precedents`), $z$-from-quadtree tactical item, full four-seam catalogue — extract later if needed. (The detail→abstract rename is executed at #sketch-detail-abstract-reversion .)
- **Face register / hanging nodes:** #form-face-flux-register (measured single-valued vs two-sided; three conditions; leaf-only price). Do not restate "wavelets make the seam free" without that segment's retraction of representation≠dynamics.
- **Conflict guard:** "flux-on-the-face makes refluxing an invariant *without a bill*" is superseded; the bill is leaf-only (or explicit correction under double-evolve).
- **Cross-face spike residue map (verified 2026-07-29, correcting the from-memory guess that it belonged here):** the spike's findings live at #form-cellid-chunk-patch FE(4) (latent transform; depth cap; default-0 conviction), #form-same-level-halo-exchange ES (cube-edge scope on the exchange design), and #form-face-flux-register ES (cross-face conservation + ownership rule + matched-seam caveat). Spike graduated.
