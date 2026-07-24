---
slug: sketch-dynamic-exponent-seams
type: sketch
status: sketch
stage: draft
depends:
  - form-seam-flux-exchange
  - form-scale-separation-directional
  - form-cellid-chunk-patch
---

# Dynamic exponent $z$ at seams (exploratory)

*Sketch peel from `doc/theory/multiscale-seams.md` §3. The per-process $z$ facts are standard numerics; the seam-reconciliation framing and the timestep-from-quadtree tactic are project stance, not established law. Not a second seam segment — #form-seam-flux-exchange and #form-scale-separation-directional own the seam laws.*

## Formal Expression (sketch)

1. **$z$ is a property of the process, not the grid.** The time resolution a space resolution demands scales as $\Delta t \sim \Delta x^{z}$: advective/wave systems (shallow water, CFL on $\sqrt{gd}$) have $z=1$; diffusive/relaxational systems (hillslope creep, groundwater; explicit parabolic bound) have $z=2$. At one spatial LOD different processes have different characteristic times — there is no single LOD↔age map. (Coarse is a low-pass of the *present*, not the past.)
2. **Live asymmetry (honest reading of the stack).** `water.rs` genuinely obeys $z=1$ (`stable_dt` recomputes CFL per burst). `erosion.rs` creep carries the $z=2$ stability *bound* but **clamps rather than sub-steps** — reduced effective diffusivity on a fixed epoch budget. The stack exhibits one live scaling and one clamped constraint; the clamp is a declared fidelity compromise, not a second scaling.
3. **Tactic (Joseph steer, 2026-07-10): derive each tile's timestep from its `CellId` level as a power of two, per $z$-sector** — halve per level at $z=1$, quarter per level at $z=2$. Neighbour tiles at different levels then mesh-synchronize by construction (Berger–Oliger constant ratio; Gear–Wells power-of-two sync), and $z$-reconciliation becomes a property of the key rather than per-tile tuning. The "if necessary" is real: clean for $z=1$ water tiles; for $z=2$ the clamp may stay, with the reconciliation gap *logged* rather than paid quadratically.
4. **Open.** "$z$-consistent resolution" at a mixed seam (a $z=1$ flux read by a $z=2$ consumer) is project coinage — not a known well-posedness criterion; hyperbolic–parabolic domain coupling is a studied problem not yet tied to it. The dyadic ladder is information-uniform only for scale-free fields; for processes with a characteristic scale the powers-of-two question is open per phenomenon.

## Epistemic Status

**Currently `sketch`.** The $z=1$/$z=2$ stability facts are standard numerical analysis; the clamp reading is a code-level observation; the quadtree-timestep tactic is a Joseph steer recorded in design prose (multiscale-seams §3), **no DECISIONS row** — do not cite as decided. The causal-cone / relativity framing in the source stays teaching; its honest reduced form is: the invariant surviving every sector is the temporal happened-before DAG, and a finite-sloped spatial cone exists only in the $z=1$ sector.

Stage `draft`.

## Working Notes

- Promote toward `formulation` only when a tile-timestep mechanism (or the logged-gap alternative) exists to convict, or Joseph ratifies the tactic.
- Source teaching (cone imagery, SR-as-special-case boundary, per-phenomenon dyadic question): multiscale-seams §3, discretisation-and-information §3.6/§3.8.
