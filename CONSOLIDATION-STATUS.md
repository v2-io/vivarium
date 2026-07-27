# Consolidation status — big-picture intuition

*Body replaced 2026-07-24 (late), on its own protocol: replace when the picture changes a lot. It did — the frontier moved from the claim channel to the instrument, and back. **Not claim canon.** When this file disagrees with `core/`, core wins.*

## The picture, in one paragraph

The claim channel is **88 segments** and in good order. What changed is that vivarium now has a **working instrument**: `vivarium explore`, a 3D view of the live world that reports its own epistemic state, replacing three view spikes with one crate. The frontier moved twice in one day — from claims-without-anything-to-see, to an instrument-without-claims, and the second gap is now closed too. The next frontier is **what the instrument can see that the world does not yet do**.

## What is true now, and measured

- **The core/view wall is a mechanism, not a discipline.** `Store::open_read_only` refuses puts with `PermissionDenied` and counts them; the explorer displays its own refused-write count. This closed a latent violation on `status`/`info`/`watch`, where `globe::render` reached a pull that computes and writes on a miss. `#form-core-view-wall` FE(6); `#form-store-as-save` FE(8) view half.
- **One view crate.** `crates/vivarium-explore`; `spikes/{globe,worldview}` and `slabs` / `vivarium-app` are in `archive/`. `vivarium explore` is a subcommand you type and a separate binary that runs, so Bevy never enters the world frame.
- **Deep time plays at the density you ask for**, by evaluating the cooling law at more temperatures — never by tweening. Nested bisection makes the view's ladder a strict superset of the builder's, bit-exactly, so coarser requests are 100 % store hits.
- **Two convergence gates are measured no-gos.** Water: no stationarity at the pinned timestep ( `#obs-water-fill-never-settles` ). Erosion: the residual is driver-bound and most tiles do no fluvial work at all ( `#obs-erosion-residual-is-driver-bound` ). The FE(4) claim stands; the *form* of each criterion is per-kernel and unearned.
- **A read coarser than the build level silently draws the uncarved prior** ( `#obs-coarse-view-draws-the-uncarved-prior` ), convicted by unit test. Found by the instrument, invisible to the eye.

## Open work, in rough order of what unblocks what

1. **Erosion has no time-interior** — `watch::interior` reports exactly one distinct time-index. This is now the *binding* constraint on the project's fastest instrument: a trained eye detects missing physics from **visual evolution**, and erosion has no evolution to show. Builder work, not view work. Not blocked by the convergence no-go, which is about criteria, not addressability.
2. **The step-size question** (physics, Joseph's or a spike's) — whether `stable_dt`'s ceiling should scale with cell size. Blocks water's ε rung.
3. **The demand spool** — beacons → cones → work queue. Untouched. Regime G is the floor: "no demand" means spine-only, not idle.
4. **Store-backed navigation** — an explorer that upgrades continuously as memos land, rather than reading a census at open.
5. Standing, unowned: the stale-`src` GC; `globe::render` unaudited against `#norm-no-depiction-without-referent`; the source digest covering `src/bin/` (editing help text invalidates a world — Joseph's call); zero inland standing water at L9, surfaced by the instrument and wanting a probe.

## The rhythm this file protects

Claim or named gap in core → convict where possible → strengthen before soften → demote rival prose → name residuals honestly.

Two lessons from 2026-07-24 worth keeping in the rhythm. **The artifacts record what was decided; the transcripts record why.** And: **when the ask is for something visible, the visible thing is the deliverable** — a day of defensible layer-beneath work summed to a day where nothing on screen changed, and the telos ("session success is a truthful segment") governs what gets written down, not permission to leave the surface untouched.

*Last intuition pass: 2026-07-24 (late).*
