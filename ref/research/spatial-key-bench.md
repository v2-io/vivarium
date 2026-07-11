# Spatial-key micro-benchmark — Hilbert chunks vs. Cartesian interior

*Purpose: settle, by measurement, whether a Hilbert/S2 space-filling-curve key
hurts the automata inner loop (erosion et al. are stencil/neighbor-bound). Source:
[`spatial-key-bench.rs`](spatial-key-bench.rs) (`rustc -O -C target-cpu=native`).
Numbers below are one dev machine — treat the **ratios** as the durable result, not
the absolute Mcells/s. Feeds `doc/design/DESIGN-MATERIAL.md` §8.*

## Results

**(A) Dense Cartesian 5-point stencil (the automata inner loop), by patch size:**

| patch | size @ 0.5 m | MB/field | throughput |
|---|---|---|---|
| 64² | 0.03 km | 0.0 | 2,846 Mcells/s |
| 128² | 0.06 km | 0.1 | 5,494 |
| 256² | 0.13 km | 0.3 | 5,889 |
| 512² | 0.26 km | 1.0 | 5,995 |
| 1024² | 0.51 km | 4.2 | 6,221 |
| 2048² | 1.02 km | 16.8 | 6,181 |
| **4096²** | **2.05 km** | **67.1** | **4,486 ← cache cliff** |

~**6 billion cell-updates/s** while the working set fits L2/L3 (≲ 1 km patch,
≲ ~17 MB/field), then a cliff into RAM at 2 km. Derived: a full erosion pass on a
**512² (0.26 km) patch ≈ 44 µs**; on a **1 km patch ≈ 0.7 ms**.

**(B) "East-neighbor key" cost — three ways** (higher = faster):

| method | throughput | vs. Cartesian |
|---|---|---|
| Cartesian `idx+1` | 8,734 Mops/s | 1× |
| Morton decode/encode | 718 Mops/s | **12× slower** |
| Hilbert decode/encode | 109 Mops/s | **80× slower** |

**(C) Encode+decode round-trip:** Morton 884 Mops/s · Hilbert 116 Mops/s.

## Conclusions

1. **Never do curve-ID neighbor arithmetic in a stencil loop.** Hilbert-ID
   neighbors are **80× slower** than a Cartesian index; in a real 512² 4-neighbor
   pass that's ~9.6 ms vs. ~44 µs (~220×). Morton is better (12×) but still wrong
   for the hot loop.
2. **The curve orders *chunks*, not cells.** A chunk is the storage/streaming unit
   (Hilbert/S2 `CellId`); its **interior is a plain Cartesian row-major array**, so
   per-cell curve ops never happen — encode/decode occurs only at *chunk*
   granularity (thousands of chunks, not billions of cells), where even Hilbert's
   116 Mops/s is vast headroom. This is Minecraft's region-file design.
3. **Hilbert stays** (over Morton): since per-cell curve ops are designed out,
   Hilbert's slower codec is irrelevant and its better locality wins.
4. **Two size tiers** (forced by arithmetic + the cache cliff): a coarse macro/
   streaming **chunk** (tens of km — a 75 km chunk at 0.5 m would be 2.25×10¹⁰
   cells, undense-able, so it's an LOD/streaming container), and a fine automata
   **patch** at the cache sweet spot **~0.25–0.5 km** (256²–1024²).
5. **Halos** at chunk + cube-face boundaries carry cross-boundary neighbors so the
   inner loop never branches (standard domain-decomposition + halo-exchange).

**Net:** the hybrid — Hilbert/S2 `CellId` for chunk identity/storage/streaming/LOD,
plain Cartesian grid within each chunk, halos at seams — is confirmed by
measurement. ~6 Gcells/s in the inner loop; curve overhead amortized to nothing.
