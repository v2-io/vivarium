# vivarium — ASSUMPTIONS (the magic-constant ledger)

*The canonical accounting of every value the world runs on that is not yet principled — created 2026-07-10 at Joseph's direction: "there needs to be a standard canonical place where these things must be all accounted for, so that we know exactly what assumptions are still being filled in unprincipled ways and we don't end up with guesses and magic constants (even fated ones or steered) scattered through the nomoi making it unlawful and **undiscoverably unlawful**."*

**The rule.** No numeric constant enters a nomos without an entry here (or a doc-comment pointing here). An unprincipled value is not the sin — *an unaccounted one is*: declared placeholders are honest rungs (LEXICON §5's D-on-low-B made glaring instead of camouflaged); undeclared ones are latent unLawfulness that no probe will ever localize. This file is the prose bridge until the declared-vs-derived machinery (LEXICON §5) makes each nomos carry its own declaration as *data* — at which point this ledger becomes generated, not hand-kept.

**Status vocabulary:** `earth-ref` (taken from measured Earth — principled *by reference*, jettisonable for non-Earth worlds) · `literature` (a published model/value used as intended) · `tuned` (chosen to look/behave right; documented reasoning, no process) · `arbitrary` (a placeholder someone had to pick) · `operational` (affects builds/views, never world-law — listed only when confusable).

## World-law constants (in nomoi — these shape every world)

| constant | value | lives at | status | principling path |
|---|---|---|---|---|
| `SEA_LEVEL_M` datum | 4000 m above bedrock datum | `gen.rs` | **arbitrary** | invert: sea level should be *derived* — fill the hypsometry with a declared water inventory; the constant then retires. Early-Earth targets now exist: `ref/research/early-continents-survey.md` §6 |
| continental band λ, amplitude | ~1250 km, ±1500 m | `gen.rs` (`CONT_PER_FACE`, ×3000) | **tuned** (slope argument documented) | two-mode crustal prior (hypsometry probe + early-continents survey → target stats), then real uplift. Survey §6 targets: land 2–15%, freeboard low-to-negative, max relief ~3–5 km, *compressed* bimodal |
| mountain band λ, amplitude, gate | ~25 km, ±1800 m, on cont>−200..+600 m | `gen.rs` (`MTN_PER_FACE` etc.) | **tuned** (core-proven scaling) | superseded by the erosion tier earning the relief |
| fBm shape | 4 octaves (cont), lacunarity 2.0, gain 0.5, octave-domain stride 0x9E37 | `gen.rs`/`noise.rs` | **arbitrary** (standard defaults) | irrelevant once noise is only sub-grid texture |
| soil mantle | 2.0 m everywhere | `gen.rs` `baseline_column` | **arbitrary** | pedogenesis system (DESIGN-SYSTEMS row) |
| soil saturation | 1.0 below sea / 0.3 above | `gen.rs` | **arbitrary** | groundwater/Darcy tier |
| stream-power `m` | 0.5 | `erosion.rs` defaults | **literature** (standard SPIM exponent) | per-material + threshold (Lague 2014 caveat already in survey) |
| erosion `k_dt`, deposition G, talus slope, κ | 0.02, 1.0, 0.8, 2 m²/epoch | `erosion.rs` defaults | **tuned** (probe-checked behavior) | calibrate against real denudation rates when epoch↔years is pinned |
| epoch ↔ years | `EPOCH_YEARS=100` nominal | worldview HUD | **arbitrary** | component E (time-indexed stages) must pin it |
| erosion run length | `epochs=40` (CLI/store_explore), 80 default | callers | **arbitrary** | convergence-ε gate (component E) replaces fixed counts |
| axial tilt, radius, solar constant, year length | 23.44°, 6371 km, 1361 W/m², 365.25 d | `planet.rs` `Planet::EARTH` | **earth-ref** | fine as reference; needs *declaring* in the Phase-0 parameter block (TODO) rather than hardcoding |
| vertical shell, voxel anchor | ~20 km, 0.5 m | DESIGN-MATERIAL §8 | **tuned** (decided anchors, rationale recorded) | stands unless a consumer breaks it |
| planetary water inventory | **ABSENT** — no water-mass ledger exists at frame level; "sea level" is a datum, not a volume | — | **missing** | declare inventory (earth-ref modern: ocean ≈1.39×10⁹ km³ ≈ 2.7 km global-equivalent depth; atmosphere ≈0.001% ≈ 25 mm — *from memory, verify*). **Early Earth wants MORE: mean ocean depth ~5–6 km** (survey §5/§6 — live debate, direction robust); shallow oceans are the placeholder's load-bearing error |
| water fill steps | fixed 200 steps @ dt 0.2 s, rain ×10 (`water-tile` nomos defaults) | `query.rs` | **arbitrary** (bounded fill, not converged — no near-stationarity gate) | analytic hydrological init seeds equilibrium; component E records convergence-ε |
| rock-mass conservation | **NOT enforced** — erosion exports mass through tile outlets with no global ledger | `erosion.rs` | **missing** | conservation-honest spine (plan Phase-2's own stated ambition); flux-BC tiles make exports *accountable* |

## Known honest fudges (worldview testbench — documented, patch-local)

| constant | value | status | note |
|---|---|---|---|
| rain rate | ~100–1000× real | **tuned** (documented fudge) | fills basins in minutes; analytic init retires it |
| atmosphere store `VIVARIUM_ATMOS` | 2.0 m water-equivalent | **arbitrary** (~80× real ~25 mm) | consistent with the rain fudge; both retire together |
| Jarrett roughness, Lekner–Dorf wet darkening, de Almeida–Bates stabilization | per papers | **literature** | used as intended |

## Operational (not world-law; listed to prevent confusion)

`TILE_NX=64`, CLI default `--level 7`, globe `LEVEL_MIN/MAX 5..9`, relief exaggeration ×20 (HUD-stated), globe default day 91/hour 10 — build/view knobs; they change what is computed or shown, never what is true.

## Process assumptions (meta — how the ledger itself can fail)

- Manual nomos-version constants (`SPINE_VERSION`, `FILL_ALGO_VERSION`) — the §12 known weak link; retired by source-derived versions.
- Earth-reference numbers in this file marked *from memory* are themselves unverified entries until checked (the hypsometry probe's Earth column shares this caveat, and says so on screen).
