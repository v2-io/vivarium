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

1. **Cross-tile base level** — the measured cap on everything hydrological: tile-edge outlets grade every tile to its own perimeter ( `#obs-tile-outlets-grade-away-the-basins` ), so basins are bounded at one tile by construction (assembled routing of the beacon patch finds a 3.85× larger trunk), 13 % of an assembled patch is seam pits, and there is nothing anywhere for water to pond in (0.02 % closed depressions — *this*, not the water kernel, is why the planet has no lakes). Repairing it un-gates basin integration, ponding, and honest seam-free assembly at once.
2. **Era-parameterized erosion** — build the fluvial rung against colder cooling stages, where the measured suture belts (zero at the present anchor; 130–1125 cells by $T_p$ 1450–1350) and assembled continents exist. The present-anchor world's fragmented radial drainage is *physically truthful for its age*; the realistic-looking world is down the chain, and this is the rung that makes it buildable. Also the iterated erode→rebound rung `#form-isostasy-column` FE(9) names.
3. **The χ criterion's two short follow-ups** — the shape test is built and measured ( `#obs-chi-shape-is-erosions-criterion` : one $T_A$ ≈ a quarter of the χ approach; plateau ~1200–1500 epochs): re-run against the new spatially-varying driver cohort (in flight), and the G sweep that would turn the rate half from diagnostic into gate.
4. **The step-size question** (physics, Joseph's or a spike's) — whether `stable_dt`'s ceiling should scale with cell size. Blocks water's ε rung — though note the no-lakes diagnosis moved to item 1's bed property; the two causes are independent and both sufficient.
5. **The demand spool** — the first standing beacon now lives in the manifest and the builder sweeps it; the spool proper (beacons → cones → work queue, multiplicity, per-beacon policy) is untouched. Regime G is the floor: "no demand" means spine-only, not idle.
6. **Store-backed navigation** — an explorer that upgrades continuously as memos land, rather than reading a census at open.
7. Standing, unowned: the stale-`src` GC; `globe::render` unaudited against `#norm-no-depiction-without-referent`; the source digest covering `src/bin/` (editing help text invalidates a world — Joseph's call) — the sidebar adds that it also does not cover rustc/LLVM version or FMA contraction. *(Zero inland standing water at L9 is no longer standing: probed and homed at `#obs-tile-outlets-grade-away-the-basins` — the edge-sink outlet policy grades every tile to its own perimeter, so 0.02% of L9 land sits in a closed depression and there is nothing to pond in. What that opens instead: tile boundaries as the thing capping every basin, and the whole-face control that would sharpen the diagnosis.)*

## The rhythm this file protects

Claim or named gap in core → convict where possible → strengthen before soften → demote rival prose → name residuals honestly.

Two lessons from 2026-07-24 worth keeping in the rhythm. **The artifacts record what was decided; the transcripts record why.** And: **when the ask is for something visible, the visible thing is the deliverable** — a day of defensible layer-beneath work summed to a day where nothing on screen changed, and the telos ("session success is a truthful segment") governs what gets written down, not permission to leave the surface untouched.

*Last intuition pass: 2026-07-28.*
