---
slug: detail-structure-scheme-map
type: detail
status: robust-qualitative
stage: draft
depends:
  - form-declared-structure-tradeoff
  - form-column-control-volume
  - obs-routing-curl-spiral
  - obs-cube-locked-kernel-bias
  - form-scale-separation-directional
  - form-core-view-wall
---

# Structure → scheme map (per-phenomenon rows and conflicts)

Extended support for #form-declared-structure-tradeoff : the per-structure menu of *what preserves it exactly*, *what breaks if you ignore it*, and *which vivarium nomoi are affected* — plus the conflicts the flat table hides. Present-best teaching rows; not a second declare-three-lists law, and not a settled NomosDecl schema.

## Formal Expression

1. **How to use this map.** Find the structures your phenomenon carries; look up schemes that preserve each exactly; then **read FE(3)** — structures conflict; the rows are not independent choices. Declaration law lives on #form-declared-structure-tradeoff ; this segment is the field catalogue.

2. **Structure rows (present best).**

| structure the physics has | what preserves it exactly | what breaks if you ignore it | vivarium nomos affected |
|---|---|---|---|
| **local conservation** (mass, sediment, water) | finite volume / flux form | mass created or destroyed at seams | `erosion`, `water`, `hydrosphere` |
| **contour-orthogonal routing of a potential** | mimetic / structure-preserving face fluxes (see #obs-routing-curl-spiral for the exact $\kappa$ statement; *not* “curl-free $q$”) | flow spirals; Escher staircase; fake drainage | `erosion` routing — **measured violated** ($\kappa\sim 2\%$) |
| **vorticity / circulation** (Kelvin) | staggered (Arakawa C) grids; energy–enstrophy schemes | spurious modes; geostrophic balance wrong | ocean/atmosphere (queued) |
| **symplectic structure** (Hamiltonian) | symplectic integrators | secular energy drift; orbits spiral | ante-mundane orbit / moon / obliquity |
| **the second law** (entropy) | entropy-stable schemes / entropy conditions — **one-sided bound**, not a selection principle; no general convergence theory for systems | conservative schemes can converge to non-physical weak solutions (expansion shocks) | hydraulic jumps; `water.rs` shocks |
| **realizability** (positivity: $h\ge 0$) | limiters / positivity-preserving schemes | negative depth ⇒ PDE type change | `water.rs` |
| **equilibria** (still lake stays still) | well-balanced schemes | spurious currents in dead-calm water | `water.rs` — unknown if ours is well-balanced or merely damped |
| **rotational symmetry** (no preferred direction) | isotropic stencils; edge-flux not fixed fans | grid-aligned rivers | `erosion` MFD — **measured broken** ( #obs-cube-locked-kernel-bias ) |
| **timescale hierarchy** (multirate) | scale-separated / multirate coupling | fast and slow silently corrupt each other | seam; `water` × `erosion` ( #form-scale-separation-directional ) |

3. **The structures conflict (not independent menu items).**
   - Simultaneous energy + potential-enstrophy conservation is **grid-privileged**: Arakawa–Lamb 1981 achieves both exactly on the **square C-grid** (semi-discretely; time integrator breaks it at truncation order); Ringler 2010 shows TRiSK on arbitrary meshes cannot. Off the privileged grid the conflict is real; on it the sacrifice moves into the time axis.
   - A **positivity limiter destroys the enstrophy budget** (nonlinear non-conservative intervention).
   - **Well-balancedness vs high-order accuracy** fight at wet/dry fronts.
   - **Entropy stability costs exact energy** — deliberate dissipation.
   - **Erosion is unusually clean; a rotating fluid is the normal case.**

4. **Consumer reconstructions are different surfaces.** Flux/conservation wants centroid-linear discontinuous volume-exact reconstruction; render/walk wants $C^1$ continuous; drainage wants monotone (no spurious pits). They **must differ**; the discrepancy is bounded ( #form-core-view-wall deepened). Volume under a piecewise-linear interpolant through point values differs from $\sum h_i A_i$ by a **curvature term** $\sim(\Delta^2/8)\nabla^2 h$: flat agreement; ridge/valley **bias**; global closed-surface cancel (local budgets wrong while global audit passes). Monotone reconstruction is what stops Priority-Flood from fighting pits the mesh invented. Triangulating a quad asserts a fold without evidence — same class as MFD diagonals.

5. **Out of bounds.** Promoting any row as adopted successor scheme; “conflicts-with” as a closed schema column (still owed on NomosDecl); selling conservation alone as a completed audit.

## Epistemic Status

**Max attainable: exact** for classical structure↔scheme pairings once each row is tied to a primary or a project measurement. **Currently `robust-qualitative`:** table and conflict list extracted from the Jul-12/13 discretisation audit (`.super-archive/from-theory/discretisation-and-information.md` §4, corrected present-best); AL81/Ringler readings via structure-preserving recon DECISIONS; routing/MFD rows backed by measurement homes. Stage `draft`. This is a `detail` segment: supports the declare-trade formulation; decides nothing it does not.

## Discussion

Without the map, agents treat structure-preserving language as a slogan or pick one row and ignore the fights. With it, a nomos author can enumerate, see conflicts, and declare three lists — the law on #form-declared-structure-tradeoff — without re-deriving the field catalogue every session.

## Working Notes

- **Source peel:** `.super-archive/from-theory/discretisation-and-information.md` §4 / §4.1 / §4.1a / §4.2 / §4.3 (present-best; correction archaeology stays history).
- **Do not restate** the false “gravity-driven flow is curl-free” probe target — use #obs-routing-curl-spiral .
- **Owed:** NomosDecl fields for preserved/sacrificed structure; null-space eigenvalue probe as instrument (Cardiff periodic-patch recipe still teaching until built).
