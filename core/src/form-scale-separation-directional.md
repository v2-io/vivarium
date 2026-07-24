---
slug: form-scale-separation-directional
type: formulation
status: robust-qualitative
stage: draft
depends:
  - form-rl-closure-algebra
  - form-seam-flux-exchange
---

# Scale separation is directional (fast→slow weak)

When two processes couple across a timescale gap, stability wants the **fast→slow** feedback weak relative to slow→fast forcing. Declare coupling **direction**, not only that multirate exists. Does not restate `#form-seam-flux-exchange` multirate gloss or invent a second multirate segment.

## Formal Expression

1. **Directional separation.** Scale separation ( #form-rl-closure-algebra law 3) is not symmetric. In the Gear–Wells multirate picture, the load-bearing stability condition is a small **fast→slow Jacobian block** (primary Prop 4.1 lineage — teaching body in `doc/theory/multiscale-seams.md` §2.3). Slow may appear quasi-static to fast; fast should not strongly force the slow at every fast step without averaging / lag.
2. **Declare direction.** A nomos pair that couples across bands should declare **who is fast, who is slow, and which way the weak block lies** — not only “we use multirate.” Inventory-named `#form-multirate-coupling` as a dual home of seam law is rejected; this segment is the **residue** only.
3. **Specimen (architecture, not shipped coupler).** Erosion geological / water hydrological: never one shared timestep; water is fast, erosion slow. Live stack may still lack time-averaged discharge→erosion — **compliance debt**, not a softener of the direction law.
4. **Out of bounds.** Dynamic exponent $z$-reconciliation as established numerics; timestep-from-quadtree as law (still source/tactical); four-band DESIGN-SYSTEMS table as claim.

## Epistemic Status

**Max attainable: exact** for Gear–Wells primary; **currently `robust-qualitative`** for vivarium cash-out. Stage `draft`. Multirate agent adjudication 2026-07-23: kill dual-home multirate package; keep this grain.

## Working Notes

- Source: multiscale-seams §2.3; agent-brief multirate-coupling-adjudication.
- Do not re-promote inventory `#form-multirate-coupling`.
