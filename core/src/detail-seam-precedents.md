---
slug: detail-seam-precedents
type: detail
status: exact
stage: draft
depends:
  - form-seam-flux-exchange
  - form-rl-closure-algebra
  - form-scale-separation-directional
  - form-depend-by-key-never-latest
---

# Primary-read seam precedents (AMR, multirate, equation-free, HMM)

Extended support for the seam laws: what the four primary methods actually say at the constructions vivarium reuses, read from the primaries — so seam work can lean on the precedent instead of re-deriving or mis-remembering it.

## Formal Expression

Attributions below are from primary reads (verified 2026-07-03 against publisher/archive pages; sources in relata with verification events).

1. **The operator algebra is axis-agnostic in the primaries themselves.** HMM (E–Engquist): compression $\mathcal{Q}$ / reconstruction $\mathcal{R}$ with $\mathcal{Q}\mathcal{R}=I$. Equation-free (Kevrekidis): lifting $\mu$ / restriction $\mathcal{M}$ with $\mathcal{M}\mu = I$; restriction averages "over microscopic space and/or microscopic time and/or number of realizations." AMR (Berger–Oliger): refinement is in space and time together — $h_l$ and $k_l$ shrink in lockstep at constant mesh ratio $\lambda = k/h$. Multirate (Gear–Wells): the same flux-across-a-boundary pattern purely on the time axis. This is the external warrant for #form-seam-flux-exchange FE(4) (one discipline, two axes) and #form-rl-closure-algebra 's operator vocabulary.
2. **Injection is not refluxing — from the source.** Berger–Oliger's fine→coarse *injection* (averaged update at sync instants) is distinct from the conservative interface flux correction (*refluxing*, matured in Berger–Colella 1989). Their load-bearing design choice is the flux-BC tile model from the source: *subgrids are independent overlays coupled only through boundary values* interpolated from the parent.
3. **Region generation — demand clustering has a 40-year-old recipe.** AMR does not refine cell-by-cell: flagged points are clustered (nearest-neighbour, then relative-neighbour graphs for hard shapes), each cluster fitted an oriented rectangle via the second-moment matrix's eigenvectors, accepted when the **efficiency ratio** (flagged ÷ enclosed cells) clears a cutoff (½–¾), else split. Two facts travel to the tile planner: the efficiency ratio *is* the criterion for "one tile or several per island of interdependence," and **good clusters change slowly** — found once, they persist many steps and only re-orient. That is the numerical-methods precedent for betting that drainage basins are stable enough to memoize (open measurement: basin-partition stability across levels).
4. **Lifting is non-unique, and it heals.** Equation-free: a lift injects error in fast (off-manifold) modes, which are damped — the solution "quickly approaches the slow manifold for any lifting operator" after a transient. The transient *is* regional-climate nesting's spin-up artifact. Vivarium's twist is **fating** the lift ( #form-rl-closure-algebra FE(3)); the healing relaxation is still owed because the *analytic seed* (not the noise) is an estimate — the acceptance shape for any analytic-init work.
5. **Slowest-first, and why: the backup argument.** Gear–Wells order integration slowest-component-first so a failed slow step needs only "reduce the last step," retaining **one** prior value per component — fastest-first would require unstable reverse integration or prohibitive fast-value storage. Power-of-two stepsizes make every slower mesh a subset of every faster one (no wasted interpolation). Together these are the numerical-methods precedent for time-in-the-key and checkpoint chains ( #form-depend-by-key-never-latest ; #form-three-scoped-runtime FE(3)): the memo retains exactly enough history to back up one coupling step.
6. **The stability condition names its direction.** Gear–Wells Prop 4.1: multirate stability requires the **fast→slow Jacobian block** small ($h\lVert B\rVert \lt K$); off block-triangular form, "any form of stability at infinity is lost." Claim home #form-scale-separation-directional ; carried here as the precedent's exact shape.
7. **On-demand micro, and our inversions.** HMM's macro solver spawns constrained, short, small micro runs where it lacks constitutive data — the lazy pull's ancestor. Vivarium's deliberate deviations from the precedents, named so they are defended rather than drifted into: pulls are **backwards-from-now** (HMM pulls forward-in-time); refinement is **attention-driven** (AMR's is error-driven — with the honest cost that an under-resolved process outside the attention cone is exactly what the criterion misses); restriction is **per-consumer** (the literature fixes one $R$ per method).

## Epistemic Status

**Max attainable: exact** for what the primaries say (quotes and constructions verified at read time; relata carries the sources: Berger–Oliger 1984, Gear–Wells 1984, E–Engquist 2003, Kevrekidis et al. 2003). **Project mappings** (tile planner ↔ region generation, time-in-key ↔ backup argument, fated lift ↔ healing) are `robust-qualitative` readings, not measurements. Stage `draft`. This is a `detail` segment: it supports the seam formulations and decides nothing they don't.

## Discussion

The point of holding these at claim-adjacent grade is the failure mode they prevent: re-deriving a precedent badly (mean-pin was "injection" mis-built), or citing one from memory (the AL81 miscite). When seam or tile-planner work starts, the applicable precedent is here with its exact shape and its honest limits.

## Working Notes

- Source graduated: `.super-archive/from-theory/multiscale-seams.md` → `.super-archive/from-theory/` (its §3 z-material lives at #sketch-dynamic-exponent-seams ; §2.4's superseded statement at #sketch-detail-abstract-reversion ; §4's declaration checklist is owned piecewise by the segments it listed).
- Open measurement named in FE(3): basin-partition stability across levels — still unowned as a probe.
