---
slug: form-declared-structure-tradeoff
type: formulation
status: robust-qualitative
stage: draft
depends:
  - disc-prime-question
  - norm-bias-vs-noise
  - norm-declaration-must-convict
  - form-seam-flux-exchange
---

# Structure preservation is a declared trade

Every discretisation preserves some structures of its physics exactly, some approximately, and sacrifices the rest — and the preserved structures **conflict**, so a scheme is always a choice. The sin is choosing silently: a nomos declares what it preserves, what it approximates, and what it abandons.

## Formal Expression

1. **Enumerate, then choose.** A phenomenon's physics carries **structures**: conservation laws, potentials/gradient-flow identities, vorticity/circulation, symplectic form, the second law (entropy), realizability (positivity), equilibria (well-balancedness), rotational symmetry, the timescale hierarchy. A discretisation preserves each one exactly, approximately, or not at all. Before a scheme stands as a nomos, its author enumerates the structures the phenomenon has, identifies which **conflict** under discretisation, and **declares all three lists** — preserved-exact, preserved-approximate, sacrificed.
2. **The structures conflict; the table is not a menu of independent choices.** Exact simultaneous energy + potential-enstrophy conservation is grid-privileged — available on the square C-grid (Arakawa–Lamb 1981, and even there only semi-discretely; the time integrator breaks it at truncation order) and provably unavailable from TRiSK on arbitrary meshes (Ringler 2010); a positivity limiter destroys the enstrophy budget; well-balancedness fights high-order accuracy at wet/dry fronts; entropy stability is purchased with deliberate dissipation. A nomos that declares "conserves mass ✓" while silently trading away a load-bearing structure has told a true thing and hidden the decisive one. Erosion (few structures) is the unusually clean case; a rotating fluid is the normal case.
3. **Conservation is necessary, not sufficient.** A perfectly conservative scheme can converge to non-physical solutions (expansion shocks), invent negative depths (changing the PDE's type), or generate currents in a dead-calm lake. Realizability and equilibrium are structures *distinct from* conservation; a scheme can honour one and destroy the other.
4. **One world, several discretisation families — declare the family and the between-family seams.** The live stack already runs staggered flux-form (`water.rs`), collocated diffusion (`erosion.rs` creep), a graph/fan router that is not FVM at all, a gridless reservoir (`hydrosphere`), and a box→field coupling (`climate`). Formalisation is therefore **not** "pick one FVM": each nomos declares *which family it is*, and the seams **between families** (box→field, collocated↔staggered, coarse↔fine mortar) are first-class declaration surfaces — that is where structure dies.
5. **What crosses a multiresolution seam is decidable: linear structures cross iff restriction commutes.** A seam here is the fluxes-not-states boundary of #form-seam-flux-exchange ; the crossing question is which *structures* survive it. A structure survives coarse↔fine **iff the restriction operator commutes with the operator expressing it** (WAVETRISK lineage; Balsara's div-B AMR is the 25-year-old constructive precedent) — and that law covers **only the linear/convex structures** (mass, circulation, positivity, well-balancedness). Nonlinear structures — the second law, every Jensen-bearing law — do **not** commute and cannot be made to: *project-then-square ≠ square-then-project*. A naive mortar at a non-conforming interface is provably not entropy-stable and empirically crashes. This is the cell-level Jensen cut ( #norm-bias-vs-noise FE(4)) restated at the seam.
6. **Adjudication and instruments.** Every declared sacrifice is adjudicated bias-vs-noise ( #norm-bias-vs-noise ); the added term's sign and order come from the modified equation ( #disc-prime-question ). The null-space/eigenvalue probe (count zero eigenvalues of the periodic-patch operator beyond rigid modes) is the instrument that shows what a scheme *cannot see* — run it while the current kernel can still fail.
7. **Out of bounds.** Selling a scheme by the one structure it preserves; "conservation: exact ✓" as a completed audit; retrofitting a structure claim after a rewrite makes the probe pass vacuously.

## Epistemic Status

**Max attainable: exact** as project method once ratified. **Currently `robust-qualitative`:** the principle and vocabulary are a **rediscovery of a mature field** — compatible/mimetic/structure-preserving discretisation, geometric numerical integration — per the literature recon (`msc/research-structure-preserving/`, ~45 primaries read; `DECISIONS[structure-preserving-is-a-rediscovery-adopt-the-field]`, `:by claude`, proposed). The commute law and its nonlinear boundary are read-primary (Kevlahan & Dubos; Balsara 2001; Friedrich et al. 2018 crash verified at four CFLs) — `DECISIONS[structures-cross-the-seam-only-if-linear]`, `:by claude`, proposed. The four-families census is a reading of the live crates (2026-07-13 audit day). **No Joseph DECISIONS ratification of the declare-three-lists rule as standing law** — do not cite it as `:by joseph`; the *convictability* requirement it extends is law ( #norm-declaration-must-convict ).

**Not claimed:** that `NomosDecl` can hold these declarations (no field exists for preserved/sacrificed structure — named build debt, `#sketch-nomos-declaration-boxes` box ④); that entropy-stable kernels or mortar projection operators are built; a "conflicts-with" column as settled schema. Structure-table row content lives in #detail-structure-scheme-map until per-nomos declarations exist.

Stage `draft`.

## Discussion

This is the load-bearing generalisation of the 2026-07-13 audit day: every defect found by hand that day was an *undeclared* structure choice — $\theta$'s claim-free diffusion, mean-pin's false invariant, one-sided clips (sign-definite ⇒ bias by construction), the router's manufactured circulation, and the undeclared *assets* of `water.rs` (well-balanced, staggered, no null space) that a rewrite would have destroyed. Declaration cuts both ways: it protects the world from silent lies and protects good schemes from ignorant "fixes." The information-theoretic restatement: a discretisation is lossy compression; every scheme chooses what information to destroy; the sin is destroying it silently.

## Working Notes

- **Dual-home demote:** `doc/theory/discretisation-and-information.md` §4 headers point here for the law; table/conflicts/consumer rows → #detail-structure-scheme-map . `doc/design/NOMOS-CONTRACT.md` box ④ section points here for the law.
- **Instruments owed:** null-space/eigenvalue probe on each field nomos; $R\circ\mathrm{div} = \mathrm{div}\circ R_F$ commutation probe; run curl probe against *current* kernel before any mimetic rewrite (it passes vacuously after).
- **Do not re-claim:** "the discrete topological identities are hard" — they are free on any mesh ($d\circ d=0$ is combinatorial); all difficulty lives in the Hodge star (recon carve).
- Sibling: `#obs-routing-curl-spiral` (measured specimen of a violated identity); `#sketch-nomos-declaration-boxes` (where declarations would live on `NomosDecl`).
