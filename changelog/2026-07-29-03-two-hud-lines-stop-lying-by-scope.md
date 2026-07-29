# Two HUD lines stop lying by scope

*2026-07-29. Frozen. World: `first-light`, seed `17425063241017297386` (`0xf1d242b21d8d89ea`), store `~/.cache/vivarium/globe-world`.*

Joseph's random-exploration screenshot (this date) caught two instrument-honesty defects in the explorer HUD; he asked for the fixes. View-side only — no world byte or key involved.

## Provenance (required fields)

| Field | Value |
| --- | --- |
| **Kernel / memo `src=`** | `802b4fd00ee45617` (this binary; see incident below) |
| **World-dir git** | `385a42362` post-rebuild |
| **Command** | `VIVARIUM_SHOT=… VIVARIUM_SHOT_DELAY=6 vivarium explore --paint surface` |

![[captures/key-truthification/hud-honesty-globe.png]]

*(GitHub: [`shot`](captures/key-truthification/hud-honesty-globe.png).)*

## What changed on screen

- **The standing-water line names its scope.** It read `0/0 water tiles readable at this source hash` on a world holding 384 current-src water tiles — true only at the census level (the deepest built level, the L13 beacon, which has no water phase), stated as if true of the world. It now says `…at L13, the census level`. The deeper instrument gap — the census pinning to the deepest level at all, so a whole-globe view loses the L9 water overlay whenever a deeper erosion-only beacon exists — is real, named, and *not* fixed here.
- **The face-seam health line can no longer print NaN.** `within NaN/2544 m` was a mean poisoned by non-finite samples from unmeasured edges; the health check `cross ~= within` cannot be evaluated against NaN. The instrument now counts only finite-sampled edges (`mesh::seam_stats`, and `n` is now counted from measurement rather than computed arithmetically), prints the sample count (`… over 6144 edges`), and says `no chart-seam edges measured this frame` when there are none. In this capture the line becomes informative for the first time: cross 311 ≈ within 322 — the panel seams at the prior are healthy at L8.

## Incident recorded: the verify-loop bit its own author

The water-seam conviction test added after the `2026-07-29-02` rebuild lives in `query.rs` — a lib file — so `SRC_HASH` moved and the store went stale (the CLAUDE.md verify-loop warning, demonstrated on the agent who had just re-read it). Rebuilt: cohort `802b4fd00ee45617`, **0 of 3936 payloads differ** vs `defc3c44` (the test-only change proved byte-null the same way the key change did). Total loop cost ~2 min at the new water-phase speed.

## Parked, deliberately

Rendering bugs (e.g. the prior tile drawing visibly shifted against the carve lattice in Joseph's screenshot) are **parked by decision** — truth bugs first, so the two bug populations stay separable. Sightings are recorded in `#disc-known-active-hotspots` FE(8), not chased.
