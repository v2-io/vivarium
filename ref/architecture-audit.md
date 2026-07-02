# Architecture audit — `vivarium-core` and the core↔view contract

*Authored 2026-07-01. Scope: `crates/vivarium-core` (`voxel.rs`, `geo.rs`, `hydro.rs`,
`lib.rs`) and the contract a future real renderer / the ASF agent layer will read
through. The spikes (`spikes/**`, `archive/**`) are deliberately **out of scope** —
they are disposable instruments, not foundation. This file is a ranked inventory of
the proof-of-concept shortcuts worth deciding on before more systems land on top.
The code is the **what**; this is the **which-of-these-will-bite**.*

## Executive summary

Two things are worth deciding **this week**, both cheap now and both first tripped by
the agent layer (the real bet): (1) the **RNG story** — the world's determinism
currently rests on a *single shared PRNG stream stepped in a fixed sequential order*
(`lib.rs`), which the Bevy-ECS parallel-agent plan will silently break the first time
agents step concurrently; decide per-entity splittable seeds before that, not after.
(2) **Say out loud that the world is bounded.** The eroded landscape is a baked,
finite (±~6 km) `f32` artifact, not the "infinite pure function of the seed" the docs
lean on; the boundedness is already true in code — only the prose overclaims, and the
overclaim is load-bearing because determinism-as-ontology and the "regenerate the far
field, don't cache it" rendering decision both cite the pure-function framing.
The **big-but-deferrable** ones are the known deferral (the edit overlay's unbounded
growth + detail→abstract reconciliation) and cross-platform float determinism — real,
but they wait honestly. The owner's sharpest worry — coordinate overflow / Far Lands —
is **latent but not currently reachable**: `i32` voxel coords are sound, and the
`f32`-metre precision decay only bites ~three orders of magnitude past the bounded
region. Verdict: the foundation is unusually clean for POC work; the fixes below are
small and mostly about *naming the boundaries honestly* before building up.

## Decision table (most-severe first)

| # | Shortcut / assumption | Verified? | Load-bearing? | Retrofit cost if deferred | Recommendation |
|---|---|---|---|---|---|
| 1 | Determinism rests on one shared, sequentially-stepped `Rng` | Verified | Yes — determinism *and* the ECS agent plan | High (touches every agent draw) | **Decide now** |
| 2 | Edit overlay: unbounded, `O(log n)`, detail baked into keys, never collapses | Verified (acknowledged in-code) | Yes — the agent layer stresses it first | High but well-contained to `voxel.rs` | **Plan & defer** |
| 3 | Eroded terrain is a baked finite `f32` artifact, not a pure function | Verified | Yes — contradicts the pure-function ontology it's sold on | Low (mostly a docs/contract correction) | **Decide now (cheap)** |
| 4 | Worldgen samples in `f32` metres → Far-Lands-style precision decay | Verified mechanism; inferred severity | No, given the bounded design | Low if bounded stays true | **Leave it — but write the bound down** |
| 5 | Cross-platform bit-identical replay not guaranteed (`powf`/`sin`) | Inferred (not tested cross-machine) | Only if replays/caches cross machines | Medium | **Plan & defer** |
| 6 | `generated()` discards the 3D strata field → pure heightfield, no caves/veins | Verified | Yes for the world-materials axis; no for agents | Medium | **Plan & defer** |
| 7 | Frozen flow *direction* (`vx,vy`) baked but not exposed on `Volume` | Verified | Yes for agents/currents | Trivial (add an accessor) | **Decide now (trivial)** |
| 8 | `WORLDGEN_VERSION` is a manual bump | Verified (acknowledged in-code) | Cache correctness | Low | **Leave it (add a guard later)** |
| 9 | Hydrology CFL stability is by-eye, no runtime guard | Verified | Worldgen robustness | Low | **Leave it** |
| 10 | No change-notification / region-enumeration in the contract | Verified | The real renderer's incremental re-mesh | Low-medium | **Plan & defer** |

---

## The findings

### 1 — Determinism rests on a single shared, sequentially-stepped PRNG *(decide now)*

`World` holds one `rng: Rng` (`lib.rs:107`), and `World::step` draws from it *per agent,
in iteration order* (`lib.rs:181-184`, `214`). Bit-identical replay is real and tested
(`lib.rs:236-248`) — **but only because the loop is single-threaded and the draw order
is fixed.** DESIGN.md pitches Bevy specifically because "ECS is the natural substrate …
determinism + headless runs come naturally" and the two-layer mind is "hundreds of
agents cheap." The moment those agents step in parallel ECS systems and each pulls from
a shared stream, the draw *order* becomes scheduler-dependent and the tether-to-truth
property (`lib.rs:10-16`) quietly dies — the exact failure the doc says to "find and
remove before doing anything else."

Nuance worth keeping: the *worldgen* PRNG use is already parallel-safe — the Perlin
gradient hashes `(seed, xi, zi)` statelessly (`voxel.rs:809-823`), and `Strata` is a
pure hash field. Only the mutable `World.rng` sequential draw is the landmine.

- **Verified**: yes, from the code and the passing single-threaded test.
- **Recommendation**: **decide now.** Give each agent its own seeded, splittable
  sub-stream (SplitMix64 splits trivially — derive `Rng::new(hash(world_seed, agent_id,
  step))` or carry a per-agent state), so a draw depends on *identity*, never on
  scheduling. Cheap now (the agent state is a placeholder scalar); a painful retrofit
  once real fast-layer dynamics are drawing randomness everywhere.

### 2 — The edit overlay: unbounded, detail-keyed, never collapses *(plan & defer)*

The only materialized state is `edits: BTreeMap<[i32;3], Voxel>` (`voxel.rs:299`).
`set_voxel` inserts forever, *including no-op edits that restore the generated value*,
on purpose (`voxel.rs:587-591`). Every `voxel()` query is an `O(log n)` map lookup
(`voxel.rs:574-579`). The module doc is honest that this "does not yet scale" and defers
the detail→abstract reconciliation (`voxel.rs:40-47`).

Two shortcuts hide here beyond the acknowledged growth problem:
- **Detail is baked into the edit keys.** Coordinates are absolute voxels *at one
  `detail`*. There is no mechanism to translate an edit made at `detail = 2` into a view
  or world running at another resolution — so the "shared fidelity invariant" (DESIGN's
  third early decision) has *no* implementation yet; edits and the eroded sample are both
  detail-locked.
- The agent layer is precisely what converts this from theoretical to real: agents that
  dig/build/farm every tick grow the overlay without bound and pay `log n` on every
  neighbourhood query.

- **Verified**: yes (and self-acknowledged).
- **Recommendation**: **plan & defer** — this is the genuinely hard research direction,
  correctly flagged as open. But do the two *cheap* halves now if the agent layer starts
  soon: (a) drop no-op edits, or gate them behind an explicit "record that an agent
  acted here" channel separate from voxel state; (b) write down that the overlay is
  detail-locked so nobody assumes cross-resolution edits work.

### 3 — Eroded terrain is a baked, finite `f32` artifact, not a pure function *(decide now, cheap)*

`Volume` carries `eroded: Option<ErodedSurface>` (`voxel.rs:303`), a stored grid of
`f32` arrays produced by minutes of erosion + hydrology (`voxel.rs:356-551`) over a
**finite** span `±region_half_m` (6 km in the live spike, `spikes/bevy-voxel/src/main.rs:50`).
Beyond the patch, terrain grades to a flat ocean floor (`voxel.rs:709-713`). So
`surface_height` is a pure analytic function *only in the raw-FBM tier*; in the tier the
real game actually uses it is a **sampled stored artifact** that must be held in memory
or serialized.

This quietly contradicts two load-bearing DESIGN claims:
- "The whole world is a pure function of `(seed, step-count)`" (DESIGN §determinism) —
  true for the FBM path, *false* for the eroded path in the "cheap to recompute" sense
  the ontology implies. Determinism itself survives (same seed → same artifact, and the
  codec round-trips bit-identically, `voxel.rs:1240-1252`) — what fails is
  *purity-as-cheapness* and *unboundedness*.
- The rendering plan's "because the core is a pure function we *regenerate* the far
  field deterministically rather than caching it" (DESIGN §rendering / `ref/rendering`)
  — regenerating the eroded far field means re-running erosion (minutes), so in practice
  it *must* be cached. The two decisions are in tension.

- **Verified**: yes.
- **Recommendation**: **decide now — it's a naming fix, not a code fix.** State plainly
  that vivarium is a **bounded continent** (it already is), that the eroded tier is a
  cached artifact with a determinism guarantee (not a cheap pure function), and
  reconcile the rendering note's "regenerate don't cache" line with that reality. No
  runtime change required; the danger is a future system built on the *belief* that the
  ground is free to recompute anywhere.

### 4 — `f32`-metre worldgen sampling: the Far Lands concern, latent *(leave it — but write the bound down)*

The owner's sharpest worry, run to ground. **Voxel coordinates are `i32`** and sound:
at 0.5 m/voxel, `i32` reaches ~1e6 km before overflow — not a concern. The precision
risk is one level up: worldgen converts voxel→metre as `x as f32 / detail`
(`voxel.rs:697-698`) and samples FBM in `f32` metre space (`voxel.rs:728-745`). `f32`
carries ~7 significant digits, so the *metre* coordinate loses sub-voxel resolution once
`|x_metres|` exceeds ~1.7e7 m: around ~10,000 km out, adjacent 0.5 m voxels start
snapping to the same sample and the surface striates — the Minecraft Far-Lands analog,
in precision rather than overflow.

The reason this is **not currently load-bearing**: the eroded world is bounded to ±6 km
(finding #3), ~three orders of magnitude inside where the decay bites. The bounded
design defuses it. It only becomes real if someone takes the "infinite abstraction tier"
prose (`voxel.rs:21-24`) literally and generates terrain at continental offsets.

- **Verified**: mechanism verified; severity inferred (bounded design makes it
  unreachable today).
- **Recommendation**: **leave the code; fix the claim.** Same correction as #3 — the
  "infinite, exact" language is the actual hazard. If an unbounded world is ever wanted,
  the fix is a floating-origin / per-region local coordinate frame, and that is a *large*
  retrofit — worth a one-line "if we ever go unbounded, this is the cost" marker so it's
  a decision, not a surprise.

### 5 — Cross-platform bit-identical replay is not guaranteed *(plan & defer)*

The determinism tests compare `f32::to_bits` within a single run on one machine and
pass. But the pipeline uses `.powf` (`geo.rs:449,493`) and `.sin` (`geo.rs:622`), whose
results are libm-implementation-dependent and **not** guaranteed identical across
platforms/compilers. `sqrt` is IEEE-correct (fine); `powf`/`sin` are not. So "an agent
can trust this world the way it trusts arithmetic" and "any moment can be re-derived and
replayed" (`lib.rs:10-16`) hold for *same-binary* replay but are a latent caveat for
replays, caches, or trajectory diffs that cross machines.

- **Verified**: the calls are there; the cross-platform divergence is inferred, not
  measured. Marking it honestly: I did not run it on two architectures.
- **Recommendation**: **plan & defer.** Cheap insurance if/when replay portability
  matters: route the handful of transcendental calls through a fixed polynomial
  approximation, or pin a `libm`-crate implementation rather than the platform's. Not
  worth doing until a cache or replay actually moves between machines.

### 6 — `generated()` discards the 3D strata field: worldgen is a pure heightfield *(plan & defer)*

`Strata` computes a rich 3D hardness field (bands + intrusions, `geo.rs:604-628`) and it
*shapes* erosion (soft rock yields, hard bands resist). But the function that decides a
voxel's **material**, `generated()` (`voxel.rs:618-642`), never samples it: below the
surface it returns DIRT for a fixed depth then STONE, purely by depth. So the geology's
strata, ore-like intrusions, and caprock exist in the *shape* of the land but are
**invisible in the ground you dig** — no visible banding, no veins. Relatedly,
generation is heightfield-only (solid below `terrain_height`, air above), so despite
"3D all the way down," worldgen emits **no caves, overhangs, or arches** — the volumetric
substrate supports them, but only edits can create them.

- **Verified**: yes — `generated()` has no `Strata` reference.
- **Load-bearing**: for the DF-grade world-materials axis (axis 2), yes; for the agent
  axis (axis 3, the real bet), no.
- **Recommendation**: **plan & defer.** When the material system becomes real, sample
  `Strata` in `generated()` to map hardness→material (the field is already there and
  deterministic). Caves/overhangs are a bigger generation change and can wait.

### 7 — Frozen flow *direction* is baked but unreachable through the contract *(decide now, trivial)*

`ErodedSurface` stores the full velocity field `vx_m`, `vy_m` (`voxel.rs:186-189`) and
DESIGN/comments say it is frozen "so the view/agents can use it" for currents and
still-vs-flowing. But `Volume`'s public surface exposes only `water_speed` — the
*magnitude* (`voxel.rs:677-685`). There is no public accessor for flow **direction**, so
an agent that wants to feel a current, or a renderer that wants to draw flow, cannot get
it without reaching into private state. A small but real gap in the core↔view/agent
contract for a value that is already computed and stored.

- **Verified**: yes.
- **Recommendation**: **decide now** — add a `water_velocity(x,z) -> (f32,f32)` accessor
  mirroring `water_speed`. Trivial, and it closes the contract to match the stated intent.

### 8 — `WORLDGEN_VERSION` is a manual bump *(leave it)*

The on-disk cache keys on call parameters but **not** on the hydrology/erosion constants
baked inside `eroded_refined`; the safety net is a hand-incremented `WORLDGEN_VERSION`
(`voxel.rs:107-120`) that the code itself flags with "**If you touch worldgen output,
bump this.**" Editing `precip_rate` or the step budget without bumping serves stale
water from cache. It is correctly identified and cheap to live with.

- **Recommendation**: **leave it**; when convenient, replace the manual tag with a hash
  of the actual constants so it can't be forgotten.

### 9 — Hydrology CFL stability is by-eye, no runtime guard *(leave it)*

The shallow-water step is explicit and CFL-bound (`hydro.rs:28-31`); the worldgen `dt`
is `0.03 * cell` (`voxel.rs:482`), inside the limit for shallow inland water but
approaching it for the deepest lake cells (`dt ≲ cell/√(g·d)`). There is no runtime CFL
assertion — the guardrail is "watch the ASCII preview," plus a magnitude bound in tests
(`hydro.rs:906-910`). Fine for a baked, once-per-world run with conservation tests; it
would matter more if water ever runs *live* at a different cell size.

- **Recommendation**: **leave it**; add a cheap `debug_assert` on the CFL number if
  water ever steps live in a loaded region.

### 10 — The contract has no change-notification or region enumeration *(plan & defer)*

The core↔view contract is pull-only: `voxel(x,y,z)`, `surface_height`,
`water_depth_voxels`, `water_speed`, `detail`, `sea_level`, `world_height`. There is no
way to enumerate the edits, ask "what changed since step N," or iterate a region — so a
live renderer must re-poll and re-mesh blindly after any edit, and a headless logger has
no cheap diff primitive. This is *intentional* (chunking/dirty-tracking are view
concerns, `voxel.rs:32-39`), and correct for the current debug views — but the real
renderer and the trajectory-diff logger (both named as first-class peers in DESIGN) will
want an incremental hook.

- **Recommendation**: **plan & defer.** When the real renderer lands, add a minimal
  read-only change-feed (e.g. an edit generation counter, or "edits in AABB") *without*
  letting a chunk cache leak into core. Naming it now so it's designed, not scraped on.

---

## What is sound (stated plainly, so it isn't re-litigated)

- **`i32` voxel coordinates** — ~1e6 km of headroom at 0.5 m; overflow is a non-issue.
- **Same-binary determinism** — genuinely held and tested across `voxel`, `geo`, `hydro`,
  and `World` (bit-identical replay, `to_bits` comparisons).
- **Conservation invariants** — water (surface+groundwater+atmosphere+ocean) and solid
  (bed+suspended) are conserved and *asserted as tests* (`hydro.rs:848-869,876-911`).
  This is real build-to-last discipline.
- **The serialization codec** — bounds-checked (`ByteReader` returns `None`, never
  panics), version-gated, rejects trailing garbage, round-trips bit-identically. A stale
  or corrupt cache can only ever force a regenerate.
- **`Volume` is `Clone + Send + Sync`** (all fields are plain data) — ready to be a Bevy
  resource without wrapping.
- **The core/view wall is intact** — `vivarium-core` has no `bevy`/render dependency; the
  workspace comment makes the rule explicit and it holds.
</content>
</invoke>
