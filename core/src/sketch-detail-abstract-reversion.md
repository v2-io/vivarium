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

# Detail→abstract reversion (open frontier — not primacy)

*Sketch of the open multiscale problem: reversion / upscaling of **irreducible discrete edits** into a memoized macro with correct invalidation. Dynamics-upscaling of continuous fields is mature prior art; this residue is not. Does not supersede seam or algebra law.*

## Formal Expression (sketch)

1. **Reversion as eviction.** When interest leaves a region, fine state may **revert** to cheaper abstraction: reversion is **cache eviction with regeneration on miss**, not silent discard of truth.
2. **Split fine state.** (i) **Derivable** from seed + macro — safe to evict, regenerate on return; (ii) **irreducible residual** — agent edits, placed structures, path-dependent history — must be kept (or checkpointed honestly).
3. **Axes of forget.** Space near the observer is *reversible* (high return probability) → prefer keep. Deep past is *monotone* (zero return) → forget more freely. Forget along the irreversible axis; conserve along the reversible.
4. **Open residue.** Upscaling continuous dynamics is mature (HMM, nesting, assimilation). **Unsolved in our frame:** upscaling an irreducible discrete edit into a content-addressed memoized macro **with correct up-invalidation** — only works if the macro stored the **right sufficient statistics** ( #form-column-control-volume , seam statistics). Wrong statistics ⇒ silent macro corruption.
5. **Not on ethereal-explorer path.** Read-only explorers make no irreducible edits; this problem loads when edits persist.

## Epistemic Status

**Currently `sketch` / open research.** DESIGN-REDUX §6–7; multiscale-seams type-4. Stage `draft`. Do not claim solved.

## Working Notes

- Supersedes leaving this only as dual-home “detail→abstract” prose without a named home.
- Related: leaf-only vs independent tiers ( #form-face-flux-register price).
