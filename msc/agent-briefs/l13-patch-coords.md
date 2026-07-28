# L13 fine patch — coordinates and provenance (2026-07-28)

*Coordination note for the explorer work; not claim canon. The manifest is the authority — read the live world's `beacon` line; this file explains how its values were chosen.*

The default world (`~/.cache/vivarium/globe-world`, "first-light", seed 17425063241017297386) now carries a standing **beacon** in its manifest demand:

```
beacon = "face=1 level=13 oi=640 oj=5376 tiles=4 epochs=300 stride=10"
```

- **Region**: 4×4 builder tiles (256×256 cells, ~310 km square, ~1.22 km/cell) at face 1, L13 origin (640, 5376) — inside the L13 image of the world's landiest L9 tile, f1 (0,320). Chosen by `examples/response_time_probe` Part 1b: 94% subaerial on the initial surface at the live seed, 993 channelized cells in the census tile.
- **epochs=300**: the measured a-priori response time — the erosional kinematic wave crosses this terrain's channel network in ≈300 epochs max (p90 ≈ 170). One full response time, not an arbitrary count.
- **stride=10**: 30 interior stages per tile as store citizens (`epochs=10,20,…,300` in the keys), each with an `aspect=stage-residual` sibling — the settle history to scrub.
- **Courant**: max ≈ 0.99 per epoch at trunk cells — right at the accuracy boundary (Braun–Willett Appendix B), so the transient is marginally but honestly resolved; p50 ≈ 0.035.

Rebuilds after source edits: any `vivarium-world` src change stales the store (SRC_HASH in every key); rerun `vivarium build` — the beacon sweep is a few seconds, the whole world ~2 min. Keys are stable across rebuilds of identical source.
