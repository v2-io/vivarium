# Foundation validation — prior-art + crate pass on the `vivarium-world` skeleton

*Written 2026-07-01. A validation-and-prior-art pass over the five foundational
decisions in `doc/design/DESIGN-REDUX.md` and the just-written `crates/vivarium-world/`
skeleton (`quantity.rs`, `time.rs`, `sphere.rs`, `planet.rs`). Web research, real
citations. Epistemic markers inline: **confirmed** (checked against a primary or
authoritative source), **likely** (consistent secondary sources, not primary-
verified), **could-not-verify** (say so rather than assert). This is a research
report, not code — the actionable deltas are collected at the end.*

---

## 1. `fast-posit` — the crate that was "not in memory." It is real.

**Confirmed. It exists, is maintained, and is the better of the two Rust posit
options — with two caveats that matter for us (license and speed).**

- **Identity.** Crate name `fast-posit`; module `fast_posit`; repository
  `github.com/andrepd/posit-rust` (author `andrepd`). First announced on the Rust
  users forum **2025-09-27** (so it post-dates training, which is why it was not
  in memory), and **v0.2.0 was published 2026-03-23** — six months of active
  work, not abandonware.
  - <https://crates.io/crates/fast-posit>
  - <https://docs.rs/fast-posit/latest/fast_posit/>
  - <https://github.com/andrepd/posit-rust>
  - <https://users.rust-lang.org/t/fast-posit-software-implementation-of-the-posit-floating-point-format/134240>

- **API surface (confirmed from docs.rs).** Generic
  `Posit<N, ES, Int, RS>` — `N` bit-width, `ES` exponent bits, `Int` the backing
  integer (`i8`…`i128`), `RS` an optional bounded regime size — **plus** fixed
  aliases `p8/p16/p32/p64`. This is *strictly more configurable* than the
  question's `Posit<N,ES>` framing: the backing-integer parameter is exposed.

- **Quire: yes.** `Quire<N, ES, SIZE>` with aliases `q8/q16/q32/q64` (128- to
  1024-bit accumulators), supporting exact `+=`/`-=` and dot-products. This is
  the feature the design's §9 "quire for exact accumulation" needs — it is
  present and first-class. The announcement demonstrates the canonical
  `MAX + 0.1 - MAX` returning exactly `0.1`.

- **Other properties (confirmed).** `no_std`, **no non-dev dependencies**,
  `RoundFrom`/`RoundInto` conversion traits, arithmetic + comparison + floor +
  mixed-precision conversion.

- **Performance (confirmed, author's own bench).** 70–350 Mops/s on an
  11th-gen Intel core @3.8 GHz, i.e. **~4–20× slower than the hardware FPU**.
  Author claims it is "faster (or at least as fast) as any freely available
  software implementation," benchmarked against `cerlane-softposit` and
  `berkeley-softfloat`. (Self-reported; I did not independently reproduce —
  **likely** accurate given it is the design goal of the crate.)

- **License: LGPL-3.0.** *This is the sharpest practical caveat.* For a
  distributed game binary that statically links Rust crates, LGPL's relinking
  obligation is a real friction (the usual Rust remedy — dynamic linking to
  satisfy LGPL — is awkward in the Rust/Cargo model). Worth a deliberate
  decision before adoption, not a footnote.

**`softposit` comparison (confirmed-ish).** `softposit` is a Rust *port of the
reference C SoftPosit* (Cerlane Leong's implementation); latest ~0.4.0; types
`P8E0`/`P32E2` etc. and `Q8/Q16/Q32` quires; `nalgebra` integration behind a
`linalg` feature; **MSRV 1.51** (an old floor, a mild staleness signal). I could
**not verify** its current download/maintenance numbers (crates.io is JS-rendered
and blocked WebFetch; lib.rs returned 403). Net: `softposit` is the older,
reference-faithful, less-optimized option with a matrix-library tie-in;
`fast-posit` is newer, generic over backing integer, self-benchmarked faster, and
`no_std`/dependency-free — but LGPL.
  - <https://docs.rs/crate/softposit/latest>
  - <https://lib.rs/crates/softposit>

**Verdict on posits in Rust today.** Usable, for the *right* uses only:

- **Exact accumulation (design §9 "quire"): yes, real, adopt when needed.** The
  quire in either crate does exactly what "long sums of erosion/sediment made
  reproducible" asks. This is the strongest fit.
- **Cross-machine determinism: yes, but posits are not the only route.** A
  software posit is integer-arithmetic under the hood with a single defined
  rounding and no NaN-payload variance, so it *is* bit-portable. But so is
  disciplined `f64` with a fixed rounding mode / no fused-multiply-add, or fixed-
  point `i64`. Posits buy determinism *plus* tapered precision *plus* the ubit;
  if determinism is the *only* goal, they are not required.
- **NOT for hot per-voxel loops.** 4–20× slower than the FPU is fatal in the
  erosion/hydrology inner loops — which is exactly why the skeleton's "rich at
  seams, raw `f64` in loops" split is correct. Posits belong at seams,
  accumulators, and stored macro cells, never in the tight iteration.

Recommendation: **do not adopt now**; keep `f64`. Revisit `fast-posit`'s quire
specifically when the accumulated-dynamics reproducibility problem is real and
measured, and resolve the LGPL question first.

---

## 2. Rich-quantity prior art — the runtime enum is the right *shape*, but the *unit representation* will not compose

**Confirmed on the crates; one genuine design concern surfaced.**

- **Units — `uom` is the wrong tool here, and that is fine.** `uom` is
  compile-time, phantom-typed, **zero runtime cost** dimensional analysis
  (<https://docs.rs/uom/>, <https://github.com/iliekturtles/uom>). It is
  excellent — *when the unit is known at compile time*. The skeleton deliberately
  wants **runtime**-tagged honesty metadata that varies per value and grows
  (interval, jitter-state, provenance). Phantom types cannot carry runtime-varying
  metadata. So `uom` is genuinely not a fit; hand-rolling the runtime type is the
  correct call, not a reinvention. A runtime peer exists — `runtime_units`
  (<https://crates.io/crates/runtime_units>) — which stores dimensions as a
  bitfield of seven integer SI exponents plus a conversion multiplier.

- **The concern the search exposed: `enum Unit` will not compose.** The skeleton
  already needed a bespoke `WattPerM2` variant — a *derived* unit. The moment
  hydrology wants m/s, geology wants kg/m³, flux wants kg/(m²·s), an enum of
  *named* units explodes combinatorially and cannot express a product/quotient of
  two existing units. The principled representation is the one `runtime_units`
  uses and `uom` encodes in types: a **vector of SI base-dimension exponents**
  `(m, kg, s, K, …)` — then `Metre/Second` is just exponent subtraction, and any
  derived unit is free. This is the same "keep the flux interface fine-grained,
  never a monolithic blob" instinct from §12, applied to units. Recommend
  switching `Unit` from a flat enum to a small dimension-exponent struct before
  the enum accretes a dozen ad-hoc derived variants. (Low urgency, high
  eventual-churn-avoided.)

- **Uncertainty — the `Exactness` bit's growth path.** For the promised
  `Interval` rung: **`inari`** is the mature choice — a high-performance,
  **IEEE-1788-conforming** interval-arithmetic crate
  (<https://docs.rs/inari/>, <https://github.com/unageek/inari>). For
  Measurements.jl-style *linear error propagation* (value ± σ propagated through
  ops via dual numbers) there is **no mature direct Rust analog** I could find;
  the closest is `uncertain-rs` (<https://crates.io/crates/uncertain-rs>), which
  does uncertainty propagation via a lazy computation graph / sampling rather than
  Julia's closed-form dual-number approach — **could-not-verify** its maturity/
  adoption. Affine arithmetic: **no strong mature Rust crate found** (could-not-
  verify a maintained one exists). Provenance/taint tracking: no standard crate;
  this is normally hand-rolled per domain — fine to defer.

- **Single crate combining units + uncertainty?** **None found.** There is no
  Rust crate that unifies dimensional analysis and uncertainty propagation the way
  the design wants (units + exactness + interval + moments + provenance in one
  value). So hand-rolling `Quantity` is justified — it is filling a real gap, not
  duplicating an existing crate. Recommendation: **keep the hand-rolled
  `Quantity`; adopt `inari` as the `Interval` rung's engine when it lands; switch
  the unit field to a dimension-exponent representation.**

---

## 3. Cube-sphere — right choice; wrong projection variant; and space needs the same integer-canonical treatment time got

**Confirmed standard; one clear correction (equiangular) and one gap (planet-
scale tile precision).**

- **It is standard — confirmed.** The equiangular gnomonic cubed sphere is the
  grid of **GFDL's FV3** dynamical core (NOAA's operational global model) and
  **NASA GEOS** (which runs FV3). Foundational comparison of projection variants
  is Rančić, Purser & Mesinger (1996), *QJRMS*, "gnomonic versus conformal."
  - <https://www.weather.gov/media/sti/nggps/Putman_Lin_Finite_Volume_Cubed_Sphere_Grid_JCompPhys_2007.pdf>
    (Putman & Lin 2007 — the canonical finite-volume cubed-sphere reference)

- **Correction: the skeleton is using the *worst* of the three gnomonic
  variants.** `CubeCoord::to_unit` maps `(u,v) ∈ [-1,1]` linearly and then
  radially normalizes. That is the **equidistant / tangent-plane gnomonic**
  (Sadourny's original). Its **max/min cell-area ratio ≈ 5.14**. The
  **equiangular** variant (interpret `u,v` as *angles* and use `tan(u), tan(v)`
  before normalizing) drops that to **≈ 1.41** — nearly √2, a ~3.6× reduction in
  area distortion for a handful of lines of code. Conformal (Rančić) is even more
  uniform but needs a non-analytic, iterative inverse and produces tiny corner
  cells — not worth it for us. **Recommend equiangular.** (Putman & Lin 2007
  tabulate these ratios: equidistant 5.14, equi-edge 2.30, equiangular 1.41.)

- **+Y-pole convention: no problem — confirmed harmless.** The pole axis is an
  arbitrary basis choice; the roundtrip tests in `sphere.rs` already prove
  self-consistency. Nothing in the literature penalizes a +Y (vs +Z) pole; just
  stay consistent everywhere.

- **Corner singularities: real but standard.** All gnomonic cubed spheres are
  **non-orthogonal and non-conformal**, worst at the 8 cube corners, where
  coordinate lines meet at non-right angles and derivatives are discontinuous.
  This is a known, managed artifact in every operational cubed-sphere model, not a
  reason to avoid the grid. For our purposes (addressing, not solving PDEs across
  faces) it is a non-issue until a simulation aspect needs cross-face stencils —
  flag it for that day.

- **Gap the design has not addressed: planet-scale coordinate precision / tile
  addressing.** At Earth radius (6371 km), a **32-bit float resolves only ~0.38 m
  at the surface** — visible jitter for a voxel world. The universal fix is a
  **floating origin** (keep the camera at the origin, express positions relative
  to it) backed by either `f64` world coordinates or **integer tile addressing +
  local `f32/f64` offsets**.
  - <https://frozenfractal.com/blog/2024/4/11/around-the-world-14-floating-the-origin/>
  - <https://www.gamedeveloper.com/programming/a-real-time-procedural-universe-part-three-matters-of-scale>

  This is the *spatial* analog of the decision `time.rs` already made well:
  time chose a canonical `i64` so memo keys hash an exact integer, not a drifting
  float. Space should do the same — a **per-face quadtree of integer tile indices
  + a local offset within the tile** — rather than a global `f64` face-`(u,v)` that
  loses precision far from a face center *and* is awkward as a content-addressed
  memo key. `CubeCoord{ f64 u, f64 v }` is fine for the math API but should not be
  the canonical addressing/key type at planet scale. (Design-level; the skeleton's
  `CubeCoord` is fine as a *conversion* type.)

---

## 4. Time model — the facts check out; the model is correct; only annotate the timescale

**Confirmed. No pitfalls that aren't already handled; two things to document.**

- **Holocene GSSP date: confirmed correct.** The base of the Holocene is
  formally **11,700 calendar years before AD 2000 ("b2k")**, defined in the NGRIP
  Greenland ice core, ratified by the ICS in 2008; the **b2k convention is
  literally "before AD 2000" (2000-01-01)**. The Greenlandian stage (which the
  base begins) was ratified 2018. The skeleton's "11,700 yr before 2000 CE" and
  its GSSP/Greenlandian framing are accurate.
  - Walker et al. (2009), *J. Quaternary Science* — the formal GSSP definition:
    <https://ui.adsabs.harvard.edu/abs/2009JQS....24....3W>
  - ICS subdivision ratification: <https://stratigraphy.org/news/125>
  - Note: the real-world date carries a ±99 yr counting error — irrelevant to
    using the instant as a *fixed sim origin* (we define it exactly at t=0).

- **`i64` deciseconds range: confirmed adequate.** `i64::MAX` deciseconds ≈
  **±2.9 × 10¹⁰ years** — deeper than the age of the universe, as the doc claims.
  Deciseconds resolve day/night trivially. Fine.

- **Leap seconds: a non-pitfall, but annotate.** Leap seconds only exist in
  **UTC**, which tracks Earth's irregular rotation. A simulation should run on a
  **uniform SI-second timescale** (TAI/TT-like) with no leap seconds — which is
  exactly what uniform `i64` deciseconds *is*. So the model is already correct;
  just **document that the timescale is uniform SI seconds, not UTC** so nobody
  later "fixes" it by adding leap-second logic.

- **Tropical vs sidereal year: correct as used, worth a one-line note.** The
  skeleton uses the **tropical year** (365.2422 d) for the year accessor and for
  the insolation declination — which is right, because *seasons* (solar
  declination) follow the tropical year. Day length uses the mean **solar day**
  (86,400 s). Mixing mean-solar-day with tropical-year is the standard, correct
  choice; the sidereal day/year distinction only matters if you model star-field
  rotation, which we don't. No change; a comment noting "tropical year for
  seasons, mean solar day for day/night — sidereal deliberately ignored" would
  pre-empt confusion.

- **Julian Date conventions: not relevant yet.** JD (epoch noon 4713 BC) only
  matters if interoperating with an astronomy library. If that ever happens, it's
  a fixed additive offset — not a pitfall now. Could-not-verify need; likely never.

---

## 5. Prior-art fact-check of `doc/design/DESIGN-REDUX.md` — attributions are accurate; the "open frontier" is overstated

**All named authors/methods check out. One substantive correction to the novelty
claim.**

| Design claim | Verdict | Strongest reference |
|---|---|---|
| AMR (CFD/astrophysics) | **Confirmed.** Berger & Oliger 1984 (*JCP* 53:484–512); refined Berger & Colella 1989. | <https://en.wikipedia.org/wiki/Adaptive_mesh_refinement> (cites both) |
| Heterogeneous Multiscale Method "(E & Engquist)" | **Confirmed.** W. E & B. Engquist 2003; review E, Engquist, Li, Ren, Vanden-Eijnden 2007 (*Commun. Comput. Phys.*). | <https://web.math.princeton.edu/~weinan/hmm.html> |
| Equation-free "(Kevrekidis)" | **Confirmed.** Kevrekidis et al. 2003, *Comm. Math. Sci.* 1(4):715–762 (lifting/restriction, gap-tooth, patch dynamics, coarse projective integration). | <https://www.researchgate.net/publication/24247244> |
| Self-adjusting computation "(Acar)" / Salsa / Adapton | **Confirmed.** Umut Acar's self-adjusting computation (2002–); Salsa powers rust-analyzer; Adapton is a real incremental framework. All do *downstream* invalidation. | Build-systems paper §related work (below) |
| "Build Systems à la Carte" (Mokhov/Mitchell/Peyton Jones) | **Confirmed.** ICFP 2018 / *PACMPL*; extended "Theory and practice," *JFP* 2020. | <https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf> |
| Earth-system multirate coupling + flux couplers | **Confirmed.** CESM **CPL7** (MCT-based), **ESMF**/**NUOPC** — components as init/run/finalize with a coupler exchanging fluxes on their own timesteps. | <https://www.cesm.ucar.edu/models/cpl/7.0>, <https://earthsystemmodeling.org/nuopc/> |
| Regional-climate dynamical downscaling + spin-up transients | **Confirmed.** One-way nesting; temporal spin-up ~5–10 days; a *spatial* spin-up region whose width depends on the driving (LBC) resolution; boundary artifacts trimmed by an oversized domain. | Giorgi 2019, *JGR-Atmos*, "Thirty Years of Regional Climate Modeling": <https://agupubs.onlinelibrary.wiley.com/doi/full/10.1029/2018JD030094> |

**The correction that matters — §6 / §7.4 / §11-seam-3 "open frontier" is
narrower than claimed.** The design repeatedly calls the **detail→abstract /
reversion / upscaling-agent-edits-into-macro** direction "genuinely thin in the
literature" and "where vivarium is actually doing research." That is **partly
wrong**, and honesty (strengthen-before-soften cuts both ways) requires narrowing
the claim rather than leaving it inflated:

- **Upscaling fine dynamics into a coarse model is a mature, named activity.**
  That *is* what HMM and equation-free do (estimate macro closure by running
  micro on-the-fly), what **superparameterization** does (embed cheap fine models
  in coarse cells and upscale their effect), and what **two-way nesting** does in
  climate/ocean (the fine child feeds the coarse parent back).
- **The single closest hit I found:** Vandenbulcke & Barth (2019), *Ocean
  Science* 15:291, **"Upscaling of a local model into a larger-scale model"** —
  they take a high-res nested model's output as **pseudo-observations** and
  **assimilate them (ensemble Kalman filter) into the coarse parent**, explicitly
  as a stand-in for two-way nesting. That is startlingly close to vivarium's
  "push the fine locus's changes back up into the macro after the fine collapses."
  Their stated limitations even rhyme with ours: the fine model must demonstrably
  beat the coarse; coupling is weaker than true two-way; and it hinges on the
  **representativity error covariance** — which is *their* version of §5's "did
  the macro store the right sufficient statistics to receive the update?"
  - <https://os.copernicus.org/articles/15/291/2019/>

- **What genuinely *is* thin / novel for vivarium** (state this, drop the broad
  claim): not upscaling-of-*dynamics*, but upscaling of **persistent, discrete,
  irreducible agent edits** (a dammed stream, a placed structure — §6's
  "irreducible residual," which is *not* a statistical closure) **into a
  content-addressed, lazily-memoized macro cache, with correct downstream
  invalidation.** The climate/DA literature upscales *continuous fields between
  concurrently-running models*; it does **not** address (a) discrete authored
  edits as the thing upscaled, nor (b) doing it through a **memoized build-graph**
  where the up-propagation must also *invalidate cached macro derivations*
  (Salsa/Adapton explicitly do downstream-only invalidation — up-into-the-
  abstraction is beyond them, as §11 already notes). **That specific
  combination** — irreducible edits × content-addressed macro cache × up-
  invalidation — is the honest research frontier. The general "detail→abstract is
  unsolved" framing should be replaced with it.

---

## Changes this implies to the skeleton (ranked by impact)

1. **`sphere.rs`: switch to the equiangular cube projection.** *(High — cheap,
   clear win.)* The current linear-`(u,v)` + radial-normalize is the equidistant
   gnomonic, area ratio ≈ 5.14; interpreting `u,v` as angles (`tan`) makes it
   equiangular, ratio ≈ 1.41 — ~3.6× less area distortion for a few lines and a
   matching inverse (`atan`). The doc already anticipates this as "a fidelity rung
   on the projection"; there is no reason to sit on the worst rung by default.

2. **`quantity.rs`: replace the flat `Unit` enum with an SI dimension-exponent
   representation.** *(Medium-high — avoids near-term churn.)* A named-unit enum
   cannot express products/quotients (it already needed a bespoke `WattPerM2`);
   the moment m/s, kg/m³, kg/(m²·s) arrive it explodes. A small exponent-vector
   (like `runtime_units`) makes derived and fluxed units free and keeps the
   "fine-grained, never monolithic" §12 discipline. Keep hand-rolling `Quantity`
   overall — **no single crate combines units + uncertainty**, so this is filling
   a real gap, not reinventing one. Adopt **`inari`** as the engine for the
   future `Interval`/`Exactness` rung.

3. **`doc/design/DESIGN-REDUX.md`: narrow the "open frontier" claim (§6 / §7.4 / §11).**
   *(Medium — correctness/honesty.)* Upscaling of *dynamics* is mature
   (HMM / superparameterization / two-way nesting / data-assimilation-as-feedback,
   esp. **Vandenbulcke & Barth 2019**). Replace the broad "detail→abstract is
   thin/unsolved" with the precise novel claim: **irreducible discrete agent
   edits upscaled into a content-addressed, lazily-memoized macro with correct
   up-invalidation.** Add the Vandenbulcke & Barth reference as the nearest prior
   art and note its "representativity error" = our §5 sufficient-statistics
   problem.

4. **Give *space* the integer-canonical treatment `time` got.** *(Medium —
   architectural, do before addressing hardens.)* f32 resolves only ~0.38 m at
   planet radius. Adopt a floating origin and **integer per-face quadtree tile
   addressing + local offsets** as the canonical position/key type (f64
   `CubeCoord` stays fine as a *conversion/math* type). This mirrors `time.rs`'s
   "hash an exact `i64`, derive float views" and keeps positions usable as
   content-addressed memo keys.

5. **Posits: don't adopt now; keep `f64`; revisit `fast-posit`'s quire later —
   and resolve the LGPL question first.** *(Low-medium — it's a future seam
   decision, not a present one.)* `fast-posit` is confirmed real, maintained
   (v0.2.0, 2026-03), generic, with a real quire; but it is **4–20× slower than
   the FPU** (seams/accumulators only, never hot loops) and **LGPL-3.0** (real
   friction for a shipped binary). The "rich at seams, raw `f64` in loops" split
   is vindicated. `softposit` is the older reference-faithful fallback.

6. **`time.rs`: annotate, don't change.** *(Low — the facts are right.)* Add one
   comment that the timescale is **uniform SI seconds (TAI/TT-like), not UTC** (so
   no one adds leap-second logic), and one noting **tropical year for seasons /
   mean solar day for day-night, sidereal deliberately ignored.** The Holocene
   GSSP date, the b2k convention, and the `i64`-deciseconds range are all
   **confirmed correct as written.**

*Where I could not verify: `softposit`'s current download/maintenance numbers
(crates.io JS-rendered, lib.rs 403); the maturity of `uncertain-rs`; and whether
any maintained Rust affine-arithmetic crate exists (I found none, but absence of
evidence). The `fast-posit` performance numbers are the author's own bench, not
independently reproduced.*
