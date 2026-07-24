---
slug: sketch-detail-abstract-reversion
type: sketch
status: sketch
stage: draft
depends:
  - form-rl-closure-algebra
  - form-seam-flux-exchange
  - form-column-control-volume
  - form-store-as-save
---

# Reversion, and the real open problem: nonlinear closure for a non-local flux

*Sketch of the reversion frame (forgetting done honestly) and the precise open research residue. The historically-named "detail→abstract" problem — upscaling an irreducible discrete edit with correct up-invalidation — is **retired as measured**; what it was gesturing at, and what remains open, is the nonlinear closure for a non-local flux. Does not supersede seam or algebra law.*

## Formal Expression (sketch)

1. **Reversion as eviction.** When interest leaves a region, fine state may **revert** to cheaper abstraction: reversion is **cache eviction with regeneration on miss** ( #form-store-as-save invalidation-vs-eviction law), not silent discard of truth.
2. **Split fine state.** (i) **Derivable** from seed + macro — safe to evict, regenerate on return; (ii) **irreducible residual** — agent edits, placed structures, path-dependent history — must be kept (or checkpointed honestly). This is #form-store-as-save 's regenerable/irreducible frame, read at the seam: what crosses back upward on reversion is a declared statistic, never raw discarded state ( #form-seam-flux-exchange ).
3. **Axes of forget.** Space near the observer is *reversible* (high return probability) → prefer keep. Deep past is *monotone* (zero return) → forget more freely. Forget along the irreversible axis; conserve along the reversible.
4. **The retired half (state).** "Upscale an irreducible discrete edit into a content-addressed memoized macro with correct up-invalidation" is **solved as stated and measured**: in the multiresolution representation a single-cell edit propagates to the root **bit-exactly**, $O(\log N)$ (~29 coefficients Haar; wider but still $O(\log N)$ under higher-order predictors), with the root integral moving by exactly $\text{area}\times\delta$. (`DECISIONS[wavelet-store-spiked-the-seam-is-not-the-details]`, `:by claude`, proposed; spike `msc/spike-wavelet-store/`.) Do not re-open the state half as research.
5. **The open half (law).** The **law** does not commute with restriction: for the live erosion kernel, $\lVert R\circ E - E\circ R\rVert$ has signed mean $+5.34\,\mathrm{m}$ (RMS $9.44\,\mathrm{m}$) against the $7.22\,\mathrm{m}$ the erosion itself carved — a **bias the size of the physics**. This is not a failure of #form-rl-closure-algebra law 1 ($R\circ L=\mathrm{id}$, a state-fidelity property, which holds exactly here); it is failure of a *stronger* property the algebra never promises — commutation of the evolution operator with restriction. And it is not locally predictable: sub-grid detail RMS correlates with the coarse law's actual error at $-0.027$ — essentially zero — because drainage area is a **non-local** accumulation; coarsening changes the flow network *topologically*, and no local statistic sees that. The literature's multiresolution-for-conservation-laws results all assume a local finite-stencil flux. **The open problem is the nonlinear closure for a non-local flux.** Design lesson inherited from Harten: never project the nonlinear operator — keep the conservation-form update; use multiresolution as representation + decision layer only.
6. **Sufficient statistics still gate the receive side.** Whatever closure emerges, the macro can absorb an edit's *effect* only if it stored the statistics its consumers need ( #form-column-control-volume ). Wrong statistics ⇒ silent macro corruption. The joint, non-local character of the needed statistic (e.g. $(A,S)$ with covariance, over a network) is exactly what makes FE(5) hard.
7. **Not on ethereal-explorer path.** Read-only explorers make no irreducible edits; this problem loads when edits persist.

## Epistemic Status

**Currently `sketch` / open research** for FE(5)–(6); FE(4)'s retirement is spike-measured (`:by claude`, proposed — the *measurement* is reproducible; adopting the multiresolution store remains Joseph's call). Sources: wavelet-store spike + lit-notes; DESIGN-REDUX §6–7; multiscale-seams §2.4 (whose "unsolved anywhere" statement predates the spike and is superseded by FE(4)/(5)'s split). Stage `draft`. Do not claim the law half solved; do not rank it ("the one open problem" framing is struck — open-problem censuses are untrusted until derived from core).

## Working Notes

- **Regression guard:** do not re-state "up-invalidation of discrete edits is unsolved" — that half is measured solved; the residue is the *law* closure. Conversely do not claim the wavelet store "retires the seam" — representation ≠ dynamics (`wavelet-store-solves-the-representation-not-the-dynamics`).
- Related: leaf-only vs independent tiers ( #form-face-flux-register price); runtime home for the edit layer's place in the architecture: #form-three-scoped-runtime .
