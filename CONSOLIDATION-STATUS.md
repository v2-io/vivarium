# Consolidation status — big-picture intuition

*Body updated 2026-07-28: open-work item 1 closed. **Not claim canon.** When this file disagrees with `core/`, core wins.*

## The picture, in one paragraph

The claim channel is **88 segments** and in good order, and vivarium has a **working instrument**: `vivarium explore` / `watch`, views of the live world that report their own epistemic state. The 2026-07-24 frontier — *the instrument can see, but erosion has nothing to show* — is closed: the erosion settle history is now a materialized stage chain (interior epochs as store citizens at the manifest's stride, `#form-time-indexed-stage-chains` FE(8)), so the system Joseph most wants to watch has an addressable, replayable interior. The next frontier is **showing that interior in world-time**: replay still orders by build history (landing time), and the first honest world-time scrub — epoch-ordered erosion playback — is unblocked and unbuilt.

## What is true now, and measured

- **The core/view wall is a mechanism, not a discipline.** `Store::open_read_only` refuses puts with `PermissionDenied` and counts them; the explorer displays its own refused-write count. This closed a latent violation on `status`/`info`/`watch`, where `globe::render` reached a pull that computes and writes on a miss. `#form-core-view-wall` FE(6); `#form-store-as-save` FE(8) view half.
- **One view crate.** `crates/vivarium-explore`; `spikes/{globe,worldview}` and `slabs` / `vivarium-app` are in `archive/`. `vivarium explore` is a subcommand you type and a separate binary that runs, so Bevy never enters the world frame.
- **Deep time plays at the density you ask for**, by evaluating the cooling law at more temperatures — never by tweening. Nested bisection makes the view's ladder a strict superset of the builder's, bit-exactly, so coarser requests are 100 % store hits.
- **Two convergence gates are measured no-gos.** Water: no stationarity at the pinned timestep ( `#obs-water-fill-never-settles` ). Erosion: the residual is driver-bound and most tiles do no fluvial work at all ( `#obs-erosion-residual-is-driver-bound` ). The FE(4) claim stands; the *form* of each criterion is per-kernel and unearned.
- **A read coarser than the build level silently draws the uncarved prior** ( `#obs-coarse-view-draws-the-uncarved-prior` ), convicted by unit test. Found by the instrument, invisible to the eye.

## Open work, in rough order of what unblocks what

1. **Epoch-ordered erosion playback** — the stage chain landed (8 stages/tile on `first-light`, `watch::interior` 8 distinct vs 1), but replay orders by *landing* time and the builder is tile-major, so a replayed sweep is a carving wave, not the world evolving in step. Erosion stages carry their epoch in-key, so an epoch-ordered reader is the cheap path; stage-major build scheduling is the builder-side alternative. This is what turns the interior into the visual evolution the eye instrument actually watches for.
2. **The step-size question** (physics, Joseph's or a spike's) — whether `stable_dt`'s ceiling should scale with cell size. Blocks water's ε rung.
3. **The demand spool** — beacons → cones → work queue. Untouched. Regime G is the floor: "no demand" means spine-only, not idle.
4. **Store-backed navigation** — an explorer that upgrades continuously as memos land, rather than reading a census at open.
5. Standing, unowned: the stale-`src` GC; `globe::render` unaudited against `#norm-no-depiction-without-referent`; the source digest covering `src/bin/` (editing help text invalidates a world — Joseph's call); zero inland standing water at L9, surfaced by the instrument and wanting a probe.

## The rhythm this file protects

Claim or named gap in core → convict where possible → strengthen before soften → demote rival prose → name residuals honestly.

Two lessons from 2026-07-24 worth keeping in the rhythm. **The artifacts record what was decided; the transcripts record why.** And: **when the ask is for something visible, the visible thing is the deliverable** — a day of defensible layer-beneath work summed to a day where nothing on screen changed, and the telos ("session success is a truthful segment") governs what gets written down, not permission to leave the surface untouched.

*Last intuition pass: 2026-07-28.*
