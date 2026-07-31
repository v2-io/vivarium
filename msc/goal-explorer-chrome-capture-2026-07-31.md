# Goal: explorer human chrome + debug capture (first iteration)

**Set:** 2026-07-31 — Joseph: implement; **first-iteration check by Joseph when ready.**

**Segments:** `#disc-explorer-human-chrome` · `#disc-explorer-debug-capture`  
**Bar:** glanceable CARVE state + capture pair protocol (“last two captures”) without reading the dense dump.

## First iteration done when

1. **C** writes a dated pair under `<world>/captures/` (or `$VIVARIUM_CAPTURES`):
   - `…-vivarium-info.v0.1.0.udon` (seed, src, fresh/stale, demand, view, pick, tiers, png path)
   - `….png` screenshot
2. Default on-globe overlay is **human chrome** (chips); full dump on `H` cycle, not default.
3. `vivarium status` **leads** with demand + fresh/stale + next action before the pyramid.
4. Joseph can open explore / status and say whether attention works.

## Status

| Item | State |
|---|---|
| Goal file | this |
| Capture pair | **ready** — `C` → `captures/*-vivarium-info.v0.1.0.udon` + `.png` (+ sighting md) |
| Human chrome v0 | **ready** — default chips; `H` human→debug→min |
| status lead | **ready** — fresh/stale + REBUILD + beacon before pyramid |
| Joseph check | **waiting — first iteration ready for review** |

## How to check

1. `vivarium status` — lead block should shout REBUILD if fresh=0 (current first-light: yes).
2. `vivarium explore` — top line **CARVE** chip; small chrome not full essay; `H` cycles.
3. Press **C** — files under `~/.cache/vivarium/globe-world/captures/` (or `$VIVARIUM_CAPTURES`).
4. Optional: `vivarium build` then re-open explore — chip should show fresh > 0.

## Log

- Goal set; implementation starts.
- First iteration implemented and installed.
