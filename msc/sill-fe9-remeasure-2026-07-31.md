# FE(9) lake-sized bed re-measure — 2026-07-31 evening

**Instrument:** `cargo run --release -p vivarium-world --example sill_fe9_probe`

**Footprint:** seed 17425063241017297386, L13 f1, o(640,5376), span 128, tile 64, epochs 40.

| arm | dep_cells | deepest m | stand_cells | stand_vol m³ |
|---|---|---|---|---|
| REF | 5010 | 695.6 | 5059 | 8.39e11 |
| PLAIN | 5078 | 695.6 | 5096 | 8.08e11 |
| SILL1 | 3402 | 736.1 | 3419 | 6.59e11 |

**Verdict:** On this footprint, production sill1+flux1 **lowers** closed-basin wet-limit inventory vs independent tiles / single field — inject integrates drainage rather than manufacturing the FE(9) local-sill freeze. One grain; not law. Landed into `#form-same-level-halo-exchange` ES.
