# Landing note — FE(6c) priced: the Coatléven reconstruction is built, and the answer is not the one the stack expected

*Opus router strand, 2026-07-29. Full record: `msc/spike-router-fe6c/`
(`PREDICTIONS.md` written first, `RESULTS.md`, raw runs `RUN-1..7.txt` +
`RESULTS-run.txt`). Harness: `crates/vivarium-world/examples/router_fe6c.rs`.
Second probe: `crates/vivarium-world/examples/solid_angle_precision.rs`. Three
DECISIONS entries appended at `:by claude :status proposed`.*

**Nothing in `core/` or `core/OUTLINE.md` was edited** (per the brief — canon
lands with one pair of hands). Proposed segment text is drafted below for you to
land or reject. **No `src/` change was made**, though one is now warranted (§4).

---

## 1. The headline, in the order I'd want to read it

FE(6)(c)'s Coatléven flux-vector reconstruction now exists and is priced on the
receiver/incision tree, against the FE(8) strawman, over 8 seeds, with a null
control the metric has never had.

1. **The belief the segment records as unmeasured is TRUE.** The principled
   remedy is **~3× more cube-safe than the naive-D4 strawman** — RMS mean-CUBE
   **0.055 vs 0.177**, and |CUBE(CoatGrad)| < 0.5·|CUBE(EdgeFull)| at every
   threshold. The strawman's +0.07→+0.26 swing was the D4 tree's four fixed
   azimuths, not diagonal-removal as such.

2. **And it buys nothing over the cheapest arm in the ladder.** Kill the
   diagonals, weight faces by transmissivity `|σ|·drop/d`, and **keep the live D8
   tree** — RMS-CUBE **0.033**, the most cube-safe arm measured, sitting closest
   to the metric's own floor. No reconstruction, no gradient operator, no Q̂.
   *On the metric of record, channel orientation is no longer an argument for
   building FE(6c).* The reconstruction's real justification — that `q̃_K` has no
   mesh-independent limit — is a consistency claim CUBE does not measure, and it
   stands untouched. But the stack currently reads as though FE(6c) is owed
   *because of* the landscape stakes, and that is the part this measurement
   removes.

3. **The magnitude half is unpriced, and I want to be explicit that its arm is
   confounded rather than let the number travel.** `CoatMag` (consuming
   `‖Q_K‖·√A_K`) comes back at RMS-CUBE 0.162 — strawman-sized. But `‖Q‖` is a
   *specific* catchment area being fed to a stream-power law with `k_dt`
   unchanged. A fair test restates stream power in specific-catchment-area form
   with re-tuned erodibility. **That is a real next spike, not a result I have.**

Two census opens close on the way past: **affordability — 1.06× live wall-time**,
essentially free; **halo — 2, derived**, and it is the price of the
*reconstruction* (neighbours' outgoing splits), not of the LSQ gradient, so
`CoatTpfa` pays it too.

The **s_K = 0 carve-out** you flagged as a hard requirement is implemented and
comes back **inert**: fill-raised cells are 0.0–0.2% of the tile and excluding
them moves CUBE by < 0.003. The limit is real but it bites on **endorheic /
closed-basin** configurations, not on an uplifting dome with base-level sinks —
worth naming that condition when the limit is next invoked, because as written it
reads like a general obstacle.

## 2. The part I'd read before the headline, honestly

Two things had to be fixed before any number meant anything, and both are about
work that is already cited as present truth.

**(a) The CUBE metric had never been shown to return zero.** Added a null-pair
control: CUBE over **two D4-symmetric footprints** (face centres, ZPos and XPos),
where the cube-locked term vanishes by symmetry. Every arm comes back consistent
with zero. The metric passes — first time — and the floor is now known
(RMS-CUBE below ~0.01–0.03 is nothing).

**(b) `router_pricing.rs`'s terrain is a bare analytic paraboloid.** Its
docstring says "broad paraboloid **+ real band-limited prior detail (natural
channel seeding)**"; the detail term is
`initial_topography_m(seed, c, c.level()) − initial_topography_m(seed, c, level)`
and the harness builds every cell **at `level`**, so it is identically zero. And
**seed 0 — the seed that experiment used — has measured zero spectral content
between levels 14 and 19**, so it carries no relief at any band. The channels
being scored formed on a near-radially-symmetric dome where the router's own
lattice bias is close to the only symmetry-breaking present. That is the same
species as the face-centre cone null test in `#norm-probe-sensitivity` — the
project's own signature hazard, in the harness built to avoid it.

**Why this is not a footnote:** on the paraboloid, the FE(6d) arm told the
*opposite* story from real terrain. `CoatTpfa` looked strongly cube-locked
(+0.090, z = 22) and `CoatGrad` was the only arm consistent with zero — a clean
"**the corrected gradient is what buys cube-safety**" headline that dissolves the
moment the terrain has relief. I had that written down before the null control
sent me back. It would have shipped.

**What survives:** the strawman band replicates almost exactly on corrected
terrain, corrected kernel and 8 seeds (+0.073/+0.160/+0.250 vs the prior
+0.070/+0.148/+0.255). **FE(8)'s conclusion is sound. Its terrain description is
not.**

## 3. Adjacent, and this one is a live-kernel finding

**`examples/router_pricing.rs` no longer bit-matches `erosion.rs`** (max|Δh| ≈ 68 m).
The lakes/fill-undo work (`0780feb`, `1c1c5a1`) changed the pipeline materially:
`fill_depressions` returns standing water, incision skips submerged cells,
deposition traps sediment in lakes, and the fill is undone before talus/creep. Its
own NOTE predicted exactly this. Its numbers describe a retired kernel — and
`#obs-routing-curl-spiral` FE(8) cites them as present truth. The re-port is in
`router_fe6c.rs` and P0 is green; `router_pricing.rs` is left alone (yours to
retire or re-port).

**`measure::cell_solid_angle` loses relative precision as the level refines.**
The four-term arctangent difference cancels catastrophically as the cell shrinks;
relative error grows like `4^level`. Measured against Van Oosterom–Strackee
(probe: `examples/solid_angle_precision.rs`):

| L13 | L16 | **L19** | **L21** | **L23** | **L25** |
|---|---|---|---|---|---|
| 5e-10 | 3e-8 | **1.6e-6** | **3.2e-5** | **4.1e-4** | **5.5e-3** (max 6.8e-2) |

`cell_area_m2` is the per-cell runoff in `accumulate_drainage` and the volume in
`deposit`. It *replaced* uniform `cell_m²` because per-cell area accuracy was
shown to matter (`#obs-cube-locked-kernel-bias`, +17.8%). At the walk-scale tiers
the repo already contemplates, its replacement carries high-frequency per-cell
noise of 0.04%–0.5% median. Harmless at L19; a real ceiling on fine tiers, and
the *shape* (spatially random) is what would read as sub-grid texture.

Fix is a drop-in — VOS on the four corners, same inputs, no new dependency, no
behaviour change above L16. **Not landed:** a `src/` edit re-keys every world
under every cohort and you had siblings on the store. Your call when to take it.

**Cross-link:** `DECISIONS[the-discrete-gcl-is-a-spec-not-a-defect]` from this
session's structure strand records that `measure.rs` carries **no edge length,
edge normal, or arm**, and proposes the discrete GCL as their build-time
acceptance test. `router_fe6c.rs::build_geom` is a working implementation of
exactly those four quantities, and its P1 identity gate (`|K|·Id = Σ|σ|(x_σ−x_K)⊗n̂`,
residual **8.0e-16**) is an *independent* exact check on the same set. If those
land in `measure.rs`, both tests should ride along. Worth the two strands talking.

That gate also settles a named open: DERIVATION §6 listed "the identity is
EUCLIDEAN and our cells are SPHERICAL … at the coarse tier it may be fatal" as
something that could kill the whole approach. **Measured: it does not.** The
identity is exact on the tangent-plane quad at every level tested, and the
spherical/planar area agreement at L19 is limited by arithmetic, not sphericity.

## 4. Proposed segment text (yours to land — I did not edit canon)

### `#obs-routing-curl-spiral`

**FE(6)(c)** — replace the "Still open / unbuilt" framing with present truth.
Suggested replacement for the (c) clause and its standing limit:

> (c) **Priced 2026-07-29 (`DECISIONS[fe6c-reconstruction-is-cube-safe-but-the-diagonal-kill-alone-is-cheaper]`).**
> The Coatléven vector reconstruction is built and measured on the receiver tree:
> it is **~3× more cube-safe than the naive-D4 strawman** (RMS mean-CUBE 0.055 vs
> 0.177, 8 seeds, null-pair-controlled) — the FE(8) belief, confirmed — **and it
> is not more cube-safe than an edge-only transmissivity-weighted fan over the
> live D8 tree** (0.033, closest to the metric's floor). *Channel orientation is
> therefore no longer an argument for the reconstruction;* its justification
> remains the consistency of the accumulated quantity (§FE(5)), which this metric
> does not measure. Cost is 1.06× live wall-time; the reconstruction requires
> **halo 2** (it consumes neighbours' outgoing splits), independently of (d).
> **(d)'s marginal on cube-safety is within noise** (0.055 vs 0.057) while it
> changes the drainage field substantially (log-drainage Spearman 0.204) — the
> precondition relation stands on Coatléven's hypothesis structure, not on a
> measured cube-safety gain. **The magnitude half is unpriced:** consuming
> `‖Q_K‖` means restating stream power in specific-catchment-area form with a
> re-tuned erodibility, and that experiment has not been run.
> **Standing limit, narrowed:** the `s_K = 0` carve-out is implemented and
> measured **inert** on an uplifting dome with base-level sinks (fill-raised cells
> 0.0–0.2%; carve-out moves CUBE < 0.003). It bites on **endorheic** configurations,
> which remain untested.

**FE(8)** — one clause, so the citation says which kernel:

> …the pricing experiment ran (`examples/router_pricing`, against the pre-lakes
> kernel; its `LiveMfd` arm no longer bit-matches `erosion.rs` since `0780feb`/`1c1c5a1`).
> The result **replicates** on the current kernel over 8 seeds
> (`examples/router_fe6c`): CUBE +0.073/+0.160/+0.250 vs the original
> +0.070/+0.148/+0.255.

**Working Notes** — two additions I'd suggest, both because they are traps this
project has now hit twice:

> - **The CUBE null-test differential has a null control as of 2026-07-29** — two
>   D4-symmetric footprints, over which every arm returns zero. Floor: RMS-CUBE
>   below ~0.01–0.03 is not distinguishable from nothing. Numbers below that
>   floor in earlier records ("fan-diagonal kill is landscape-benign, CUBE −0.02
>   to −0.04") were **at** the floor, not below it.
> - **Do not re-use `router_pricing.rs`'s `terrain()`**: its band-limited detail
>   term is identically zero and seed 0 has no band-limited relief at all. A
>   smooth radially-symmetric dome makes the router the only symmetry-breaking
>   present — the face-centre-cone hazard wearing different clothes.

### `#obs-cube-locked-kernel-bias`

Candidate Working Note (or a `#gap` row if you'd rather it be owed): `cell_area_m2`
retired uniform `cell_m²` on accuracy grounds, and its own relative precision
degrades as `4^level` — 1.6e-6 median at L19, 4.1e-4 at L23, 5.5e-3 at L25 —
because `cell_solid_angle` cancels four O(1) arctangents. Probe:
`examples/solid_angle_precision.rs`. Drop-in fix identified, not landed (re-key).

## 5. Feedback on the brief, since you asked

The brief was the reason this went where it went, and specifically:

- **Separating "measured" from "believed" in the opening paragraph did real
  work.** I knew before starting which sentence I was allowed to lean on. When
  the paraboloid run produced a clean, flattering, *believable* story, the thing
  that made me keep going was that the brief had already modelled "here is what
  we believe / here is what has been measured" as different registers — so
  producing a third confident claim without a control felt wrong in a way I could
  name. That register is worth keeping.

- **The one thing I'd have wanted that wasn't there:** the brief passed forward
  FE(8)'s CUBE numbers as the stakes, but not the fact that **nobody had ever
  established what CUBE returns on a null**. That is not a criticism of you —
  it was not in the artifacts either; the prior RESULTS lists "one seed / one
  face / one level" under *Limits*, which reads as "a sweep would tighten it,"
  when the actual situation was "the metric has no floor and one of these numbers
  may be entirely floor." **If a metric's noise floor is unmeasured, that is a
  different fact from its scope being narrow, and it deserves to be said in the
  register of "we do not know," not filed under Limits.** Generalisable: a Limits
  list is where an unmeasured *floor* goes to look like an unmeasured *range*.

- **"If anything adjacent looks wrong, say so" was load-bearing twice** — the
  stale kernel port and the solid-angle precision both arrived through gates I
  only wrote because that sentence made adjacent findings in-scope rather than a
  distraction.

- **Pointing at the graduated spike as "the predecessor's full trail" rather than
  summarising it** was right. `DERIVATION.md` §2.1's negative result (the project
  had already implemented the direction-dependent width and didn't know it) is
  the thing that told me the reconstruction's value was in *direction*, not in
  the catchment normalisation — and no summary would have carried that.

## 6. Still on the line

Happy to: run the specific-catchment-area re-tuned stream-power arm (the honest
magnitude test); sweep faces/levels/forcing; land the VOS fix and take the re-key
when you say; re-port or retire `router_pricing.rs`; or draft the segment edits
as a patch if you'd rather review a diff than prose.
