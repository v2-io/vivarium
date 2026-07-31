# Matched-ratio probe result — 2026-07-31 night

**Instrument:** `examples/matched_ratio_probe` on production `carve_region_jacobi_exchange` (sill1+flux1).

**Footprint:** seed 17425063241017297386, L13 f1, o(640,5376), span 128, tile 64, epochs 40.

| d | σ | d/σ | mean \|Δh\| vs REF (m) |
|---|---|-----|------------------------|
| 8 | 10 | 0.80 | 41.51 |
| 16 | 20 | 0.80 | 38.48 |
| 10 | 10 | 1.00 | 40.68 |
| 16 | 16 | 1.00 | 39.79 |
| 16 | 10 | 1.60 | 42.94 |
| 32 | 20 | 1.60 | 37.41 |

**Within-ratio spread:** 0.80 → 3.02 m; 1.00 → 0.88 m; 1.60 → 5.53 m.

**Verdict:** Not enough to promote $d/(v\sigma)$ to operating law. Production keeps measured `for_build` ($d{=}16$, stride-as-$\sigma$). Log: `/tmp/matched_ratio_probe.log`.
