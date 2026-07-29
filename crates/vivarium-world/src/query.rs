//! The lazy pull-query + the first nomos — the runtime as a demand-driven,
//! memoized query graph (`doc/design/DESIGN-REDUX.md` §11).
//!
//! Claim homes: store as durable state → `#form-store-as-save`; observe-only
//! surface pull (no cold long-evolution on the view path) →
//! `#form-builder-admission`; complete keys → `#form-complete-content-addressed-key`.
//!
//! Queries are methods on a [`World`]: the context that owns `(store, seed)`
//! **together**, so the same field feeds both the key construction and the
//! compute — the world-seed in the key and the world-seed in the KRNG draws
//! *cannot* diverge, because there is only one source (Joseph's question,
//! 2026-07-10: "is it wise to rely on coders always putting the right seed in
//! the KRNG?" — no; this struct is the structural answer). A `World` is built
//! from a manifest (`spec.rs`) in one place; nomos never see a bare seed.
//!
//! A query is *coordinate-addressed*: it builds a complete [`Key`] from its
//! (nomos, version, seed, region, resolution) inputs, checks the [`Store`], and
//! on a miss computes via the nomos and memoizes the result. Walking the world
//! is then just pulling the tiles around the observer — revisits hit the store,
//! so matured state **persists** (no re-seed-from-raw-prior; the store is the
//! save — `#form-store-as-save`). Dependencies between systems become recursion
//! in the pull.

use std::sync::atomic::{AtomicBool, Ordering};

use crate::erosion::{self, ErodedRegion, ExchangedTile, Fluvial, FluvialParams, HaloSchedule};
use crate::gen;
use crate::nomotheke::{
    CLIMATE, EROSION, HYDROSPHERE, INITIAL_TOPOGRAPHY, ISOSTASY, LITHOSPHERE, MANTLE_THERMAL, UPLIFT,
    WATER,
};
use crate::{erosion_return, sea_level};
use crate::sphere::{CellId, Face};
use crate::store::{Key, PutOpts, Store};

/// Where a pulled value came from — the memoization signal, so callers (and the
/// HUD, later) can *see* the world being built once and reused.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    /// Freshly computed and memoized (a store miss).
    Computed,
    /// Served from the store (a hit) — matured, lawful state.
    Hit,
    /// Served from the store, but the root is tagged **provisional** — written
    /// under waived flux admission (`--allow-unmet`). Matured bytes, not lawful
    /// *in vivia* evidence; consumers must not launder it into a lawful Hit.
    /// (De-novo audit P0 residual A: the Hit path now surfaces the flag.)
    HitProvisional,
}

impl Source {
    /// Any store hit, lawful or provisional (the memoization signal alone).
    pub fn is_hit(self) -> bool {
        matches!(self, Source::Hit | Source::HitProvisional)
    }
}

/// The store-citizen reduction of the mantle-thermal cooling chain at one epoch
/// mantle temperature `T_p` — the four global f64 scalars every per-cell surface
/// read of that epoch needs, so once these are in hand a `tectonic_surface_at_tp`
/// read is O(1) (no ~393k-cell pour or ledger pass). This is the durable memo the
/// store owns (`#form-store-as-save` FE(6), decided: memoized ≡ store object); the
/// in-RAM caches in `sea_level` / `erosion_return` are working-set staging primed
/// from it. The big surface FIELD is deliberately NOT stored — it regenerates
/// O(1)/cell from these scalars at whatever resolution a viewer wants (thin-save,
/// FE(7): regenerable state is not materialized, and storing it would lock the
/// sample level).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EpochReduction {
    /// Pre-ledger derived sea (m) — the pour against the raw isostatic surface.
    pub pre_ledger_sea_m: f64,
    /// Rock-mass ledger: uniform submarine sediment thickness (m) deposited.
    pub deposit_m: f64,
    /// Rock-mass ledger: area-mean post-erosion buoyancy reference (m).
    pub post_reference_m: f64,
    /// The live derived sea (m) — pour against the post-erosion surface.
    pub derived_sea_m: f64,
}

impl EpochReduction {
    fn to_bytes(self) -> Vec<u8> {
        let mut b = Vec::with_capacity(32);
        for x in [self.pre_ledger_sea_m, self.deposit_m, self.post_reference_m, self.derived_sea_m] {
            b.extend_from_slice(&x.to_le_bytes());
        }
        b
    }
    fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() != 32 {
            return None;
        }
        let f = |i: usize| f64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        Some(EpochReduction {
            pre_ledger_sea_m: f(0),
            deposit_m: f(1),
            post_reference_m: f(2),
            derived_sea_m: f(3),
        })
    }
}

/// Census of `erosion-tile` roots by source-hash freshness (see
/// [`World::eroded_region_census`]). `fresh + stale == total`.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct RegionCensus {
    /// All `erosion-tile@` roots in the store.
    pub total: usize,
    /// Roots whose `src=` equals the current binary's [`crate::nomotheke::SRC_HASH`]
    /// — this world's current eroded surface.
    pub fresh: usize,
    /// Roots carved under a different source tree (`src=` mismatch) — matured
    /// bytes, but not the current law's surface. Shown only if a caller opts in.
    pub stale: usize,
}

// Nomos identities (name, version, epistemic declaration, promises,
// assumptions) live in the NOMOTHEKE (`nomotheke.rs`) — the registry is the
// only key-mint for world-law computations, so an undeclared nomos cannot
// reach the store. Bump a version by re-declaring there (source-derived
// versions remain the §12 target).

/// One vivium, opened for querying: the store it persists in and the seed that
/// (with the law) IS its identity (LEXICON §4; `#detail-vivium-lifecycle / #disc-unlawfulness-budget`
/// Stage 0). Construct via [`World::new`] — normally from a loaded manifest
/// (`spec::WorldSpec`), the one place a bare seed is handled.
pub struct World<'s> {
    store: &'s Store,
    seed: u64,
    /// When set, memo puts tag roots `provisional` (builder waived unmet flux).
    /// Atomic so `World` stays `Sync` for parallel face pulls (globe worker).
    provisional_writes: AtomicBool,
}

impl<'s> World<'s> {
    /// Hit source for `key`, surfacing the provisional flag (waived admission)
    /// instead of laundering it into a lawful Hit.
    fn hit_source(&self, key: &Key) -> Source {
        if self.store.is_provisional(key) { Source::HitProvisional } else { Source::Hit }
    }

    pub fn new(store: &'s Store, seed: u64) -> Self {
        World {
            store,
            seed,
            provisional_writes: AtomicBool::new(false),
        }
    }

    /// The world-seed (read-only — identity is set at construction).
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Tag subsequent memo puts as provisional (or clear the tag). Builder sets
    /// this for phases admitted only under `--allow-unmet`.
    pub fn set_provisional_writes(&self, provisional: bool) {
        self.provisional_writes.store(provisional, Ordering::Relaxed);
    }

    fn put_memo(&self, key: &Key, value: &[u8]) {
        let _ = self.store.put_with(
            key,
            value,
            PutOpts {
                provisional: self.provisional_writes.load(Ordering::Relaxed),
            },
        );
    }

    /// The hydrosphere nomos — the planet's conserved water budget (`crate::hydrosphere`).
    /// A **reservoir/box**, not a field: no face/level/tile, just global stocks, so
    /// its key carries only identity and its artifact is a handful of scalars. That
    /// it pulls through the same store/memo path as the field nomos is the proof the
    /// contract is representation-agnostic. (Currently seed-invariant — pure declared
    /// ante-mundane constants — but keyed by seed for uniformity and future variation.)
    pub fn hydrosphere(&self) -> (crate::hydrosphere::Hydrosphere, Source) {
        let key = HYDROSPHERE.key().field("seed", self.seed);
        if let Some(bytes) = self.store.get(&key) {
            if let Some(h) = crate::hydrosphere::Hydrosphere::from_bytes(&bytes) {
                return (h, self.hit_source(&key));
            }
        }
        let h = crate::hydrosphere::Hydrosphere::of(&crate::planet::Planet::EARTH);
        self.put_memo(&key, &h.to_bytes());
        (h, Source::Computed)
    }

    /// The complete key for a initial-topography tile: every input folded in (§12).
    fn initial_topography_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Key {
        // deps include NOISE — bumping noise version must invalidate this tile.
        INITIAL_TOPOGRAPHY
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .with_dep_versions(&INITIAL_TOPOGRAPHY)
    }

    /// System #1 — the fBm coarse initial-topography: a `nx × nx` tile of band-limited
    /// surface-prior elevations (m), a pure function of (seed, face, level,
    /// origin, nx) via the coordinate-hashed prior. This is the conservation-
    /// honest first light: land vs water, before any principled tectonics.
    fn compute_initial_topography(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(nx * nx);
        for j in 0..nx as u32 {
            for i in 0..nx as u32 {
                let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
                out.push(gen::initial_topography_m(self.seed, cell, level) as f32);
            }
        }
        out
    }

    /// Pull a initial-topography tile through the store: hit → load; miss → compute + memoize.
    /// Returns the tile (row-major, `nx × nx`) and whether it was computed or served.
    pub fn initial_topography(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
    ) -> (Vec<f32>, Source) {
        let key = self.initial_topography_key(face, level, oi, oj, nx);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), self.hit_source(&key));
        }
        let tile = self.compute_initial_topography(face, level, oi, oj, nx);
        self.put_memo(&key, &encode_f32(&tile));
        (tile, Source::Computed)
    }

    /// The complete key for an uplift tile (rate + freeboard identity via nomos version).
    fn uplift_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Key {
        UPLIFT
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .with_dep_versions(&UPLIFT)
    }

    /// The uplift nomos — a `nx × nx` tile of rock-uplift rates (m/epoch), pulled
    /// through the store like any other. Erosion consumes this; it is its own
    /// article of law (`crate::uplift`), so "what lifts the land" is one legible,
    /// swappable, memoized thing.
    pub fn uplift_tile(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> (Vec<f32>, Source) {
        let key = self.uplift_key(face, level, oi, oj, nx);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), self.hit_source(&key));
        }
        let tile = crate::uplift::uplift_rate_tile(self.seed, face, level, oi, oj, nx);
        self.put_memo(&key, &encode_f32(&tile));
        (tile, Source::Computed)
    }

    /// The complete key for a climate tile. It depends on the hydrosphere box
    /// (its atmosphere stock), so that version is folded in.
    fn climate_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Key {
        CLIMATE
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .with_dep_versions(&CLIMATE)
    }

    /// The climate nomos — a `nx × nx` precipitation field (m/yr). v0 is UNIFORM:
    /// it pulls the hydrosphere **box** for the atmosphere stock and fills the tile
    /// with the global-mean throughput (`stock / residence-time`). This is the
    /// first **box → field** coupling: a reservoir feeds a field through the store,
    /// each keeping its own representation. Geography (ITCZ/orography) is the next
    /// rung; for now every cell shares the mean.
    pub fn climate_tile(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> (Vec<f32>, Source) {
        let key = self.climate_key(face, level, oi, oj, nx);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), self.hit_source(&key));
        }
        let (h, _) = self.hydrosphere();
        let mean = crate::climate::mean_precip_m_per_yr(h.atmosphere_m_we(&crate::planet::Planet::EARTH));
        // Fated, mean-preserving, low-frequency jitter about the mean: uniform rain
        // is a physically impossible state (zero variance), so unmodelled variance
        // is closer to truth than none (Joseph). The PATTERN is noise, not
        // meteorology — the real first-order structure is latitudinal, unbuilt.
        let mut tile = Vec::with_capacity(nx * nx);
        for j in 0..nx as u32 {
            for i in 0..nx as u32 {
                let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
                tile.push((mean * crate::climate::precip_jitter_factor(self.seed, cell)) as f32);
            }
        }
        self.put_memo(&key, &encode_f32(&tile));
        (tile, Source::Computed)
    }

    /// The complete key for an eroded tile — including its *upstream dependencies'*
    /// identities (§12): the initial-topography surface it carves, the uplift field it carves
    /// against, and the climate precipitation that drives its discharge. If any
    /// changes, this key changes and the tile recomputes.
    ///
    /// When `halo` is `Some`, the schedule descriptor `(d, σ, ρ)` is identity
    /// (`#form-same-level-halo-exchange` FE(4)/(7); `#form-complete-content-addressed-key`
    /// FE(6)) — a tile carved under Jacobi exchange is not the same article as
    /// the shipped edge-sink tile at the same coordinates. The default path
    /// (`None`) keeps the historical key shape so every existing world stays
    /// addressable.
    fn erosion_key(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
        halo: Option<HaloSchedule>,
    ) -> Key {
        let mut k = EROSION
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .field("epochs", epochs);
        if let Some(s) = halo {
            // Descriptor, not payload: O(1), pure function of the schedule.
            k = k
                .field("edge", "halo")
                .field("d", s.depth)
                .field("sigma", s.cadence)
                .field("rho", s.cone_rho);
        }
        k.with_dep_versions(&EROSION)
    }

    /// System #2 — the fluvial-erosion tier, *composed on the initial-topography through the
    /// store*. On a miss it **pulls its input surface from the initial-topography** (which
    /// recurses into system #1 and memoizes it), seeds the fluvial kernel from
    /// that surface, runs `epochs`, and memoizes the eroded elevation field. This
    /// is the coupling property in miniature: one system depends on another
    /// *only* through a pulled, memoized, keyed surface — never shared mutable
    /// state.
    pub fn erosion_tile(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
    ) -> (Vec<f32>, Source) {
        let key = self.erosion_key(face, level, oi, oj, nx, epochs, None);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), self.hit_source(&key));
        }
        // Dependencies, all pulled (memoized — recurse into their nomos): the
        // initial-topography surface it carves, the uplift field it carves against, and the
        // climate precipitation that drives its discharge.
        let (initial_topo, _) = self.initial_topography(face, level, oi, oj, nx);
        let (uplift, _) = self.uplift_tile(face, level, oi, oj, nx);
        let (precip, _) = self.climate_tile(face, level, oi, oj, nx);
        // Relative precipitation weight = precip / tile-mean (uniform climate → all
        // 1.0 → discharge unchanged; spatial climate redistributes discharge).
        let mean = precip.iter().sum::<f32>() / precip.len().max(1) as f32;
        let precip_weight: Vec<f32> =
            if mean > 0.0 { precip.iter().map(|p| p / mean).collect() } else { vec![1.0; precip.len()] };
        // Seed erosion from the pulled initial-topography; any cell the kernel samples outside
        // the tile (edge/halo) falls back to the prior — identical values, since
        // the initial-topography IS the prior at this rung.
        let surf = |cell: CellId| -> f64 {
            let (cf, ci, cj, _) = cell.to_face_ij();
            if cf.index() == face.index() && ci >= oi && cj >= oj {
                let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                if di < nx && dj < nx {
                    return initial_topo[dj * nx + di] as f64;
                }
            }
            gen::initial_topography_m(self.seed, cell, level)
        };
        let mut f = Fluvial::from_surface(self.seed, face, level, oi, oj, nx, surf);
        f.set_uplift_rate(uplift); // erosion CONSUMES the uplift nomos's field
        f.set_precip_weight(precip_weight); // ...and the climate nomos's rain
        f.erode(&FluvialParams { epochs, ..Default::default() });
        let eroded = f.h.clone();
        self.put_memo(&key, &encode_f32(&eroded));
        (eroded, Source::Computed)
    }

    /// Carve a rectangular block of tiles under **Jacobi halo exchange** and
    /// memoize each interior under its complete key (schedule in the key —
    /// `#form-same-level-halo-exchange` FE(4)).
    ///
    /// This is the first production path for the cross-tile base-level repair:
    /// the exchange that `examples/halo_exchange_probe` measured now writes the
    /// store. A single-tile pull cannot do this alone (the dependency cone is a
    /// scheduling cost, FE(5)); a **region sweep** is the honest unit of work.
    ///
    /// Returns one [`ExchangedTile`] per tile in row-major order `(ti, tj)`, plus
    /// [`Source::Computed`] if any tile missed and [`Source::Hit`] if every tile
    /// was already present under the schedule's keys.
    ///
    /// **Not yet:** flux half of the seam, spill-level scalar for straddling
    /// basins, demand-driven single-tile cone with `ρ`, stage-chain composition
    /// under exchange (`σ` as stage stride identity).
    pub fn erosion_region_exchanged(
        &self,
        face: Face,
        level: u8,
        region_oi: u32,
        region_oj: u32,
        tile_n: usize,
        tiles_i: usize,
        tiles_j: usize,
        epochs: u32,
        schedule: HaloSchedule,
    ) -> (Vec<ExchangedTile>, Source) {
        assert!(schedule.cadence >= 1, "exchange requires cadence ≥ 1");
        // All-hit short path: every interior key present ⇒ no recompute.
        let mut all_hit = true;
        let mut hits: Vec<ExchangedTile> = Vec::with_capacity(tiles_i * tiles_j);
        for tj in 0..tiles_j {
            for ti in 0..tiles_i {
                let oi = region_oi + (ti * tile_n) as u32;
                let oj = region_oj + (tj * tile_n) as u32;
                let key = self.erosion_key(face, level, oi, oj, tile_n, epochs, Some(schedule));
                if let Some(bytes) = self.store.get(&key) {
                    hits.push(ExchangedTile {
                        oi,
                        oj,
                        nx: tile_n,
                        h: decode_f32(&bytes),
                    });
                } else {
                    all_hit = false;
                    break;
                }
            }
            if !all_hit {
                break;
            }
        }
        if all_hit {
            return (hits, Source::Hit);
        }

        // Shared rain mean across the whole block so per-tile renormalization
        // cannot confound the seam comparison (matches `halo_exchange_probe`).
        let span_i = tile_n * tiles_i;
        let span_j = tile_n * tiles_j;
        let rain_mean = {
            let mut sum = 0.0f64;
            let mut n = 0usize;
            for j in 0..span_j {
                for i in 0..span_i {
                    let cell =
                        CellId::from_face_ij(face, region_oi + i as u32, region_oj + j as u32, level);
                    sum += crate::climate::precip_jitter_factor(self.seed, cell);
                    n += 1;
                }
            }
            (sum / n.max(1) as f64) as f32
        };

        let seed = self.seed;
        let mk_window = |oi: i64, oj: i64, nx: usize| -> Fluvial {
            let oi_u = oi.max(0) as u32;
            let oj_u = oj.max(0) as u32;
            let surf = |cell: CellId| -> f64 { gen::initial_topography_m(seed, cell, level) };
            let mut f = Fluvial::from_surface(seed, face, level, oi_u, oj_u, nx, surf);
            // Uplift / precip over the window in face coords (clamp origin; for
            // negative oi the window samples via gen, so rates use the clamped
            // origin's field — same compromise the halo probe makes).
            f.set_uplift_rate(crate::uplift::uplift_rate_tile(seed, face, level, oi_u, oj_u, nx));
            let mut w = Vec::with_capacity(nx * nx);
            for j in 0..nx as i64 {
                for i in 0..nx as i64 {
                    let cell = CellId::from_face_ij(
                        face,
                        (oi + i).max(0) as u32,
                        (oj + j).max(0) as u32,
                        level,
                    );
                    let p = crate::climate::precip_jitter_factor(seed, cell) as f32;
                    w.push(if rain_mean > 0.0 { p / rain_mean } else { 1.0 });
                }
            }
            f.set_precip_weight(w);
            f
        };
        let prior = |i: i64, j: i64| -> f32 {
            let cell = CellId::from_face_ij(face, i.max(0) as u32, j.max(0) as u32, level);
            gen::initial_topography_m(seed, cell, level) as f32
        };

        let tiles = erosion::carve_region_jacobi_exchange(
            region_oi as i64,
            region_oj as i64,
            tile_n,
            tiles_i,
            tiles_j,
            epochs,
            schedule,
            mk_window,
            prior,
        );

        for t in &tiles {
            let key = self.erosion_key(face, level, t.oi, t.oj, t.nx, epochs, Some(schedule));
            self.put_memo(&key, &encode_f32(&t.h));
        }
        (tiles, Source::Computed)
    }

    /// The sibling key for one stage's measured residual — the mean $|\Delta h|$
    /// (m) of the stage's final epoch, recorded so the honesty travels with the
    /// stage instead of living in a process's transient `last_delta_m`
    /// ( #form-time-indexed-stage-chains FE(3): an ε never recorded is an
    /// unLawfulness budget asserted to be zero).
    ///
    /// `aspect` distinguishes these one-scalar roots from the height field itself
    /// (the `epoch-reduction` precedent). **This is a measured residual, not a
    /// convergence criterion** — erosion has no earnable criterion yet
    /// ( #obs-erosion-residual-is-driver-bound : sustained uplift pins the
    /// residual at the driver's rate), so this records what the kernel *did*,
    /// never certifies what it reached. Schema provisional: one f32.
    fn erosion_stage_residual_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize, epochs: u32) -> Key {
        EROSION
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .field("epochs", epochs)
            .field("aspect", "stage-residual")
            .with_dep_versions(&EROSION)
    }

    /// Read one stage's recorded residual — the mean $|\Delta h|$ (m) of its
    /// final epoch — if that stage was computed since residuals were recorded.
    /// `None` for pre-chain stages (endpoint-only worlds carry no residual for
    /// their endpoint; the record is made at compute time, never backfilled) and
    /// for stages that were never built. Store-only: never computes, never
    /// writes.
    pub fn erosion_stage_residual(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
    ) -> Option<f32> {
        let key = self.erosion_stage_residual_key(face, level, oi, oj, nx, epochs);
        let bytes = self.store.get(&key)?;
        decode_f32(&bytes).first().copied()
    }

    /// [`Self::erosion_tile`] with a **time-interior**: materialize the settle
    /// history as a chain of keyed stages, every `stage_stride` epochs, each
    /// seeded from its predecessor's stored heights
    /// ( #form-time-indexed-stage-chains FE(1)–(2): stage $n{+}1$ depends on
    /// stage $n$ by complete key, and "the start of the third stage" becomes a
    /// key rather than a description).
    ///
    /// **Stage stride is demand, not identity** on the default (no-exchange)
    /// path ( #form-manifest-prescribes-vivium FE(5)): a stage at `epochs=k`
    /// holds byte-identical heights whether it was built as a chain rung or as
    /// a one-shot run to `k` — the per-epoch step is a pure function of the
    /// height field plus keyed inputs, so chaining is exactly the one-shot
    /// computation with intermediate states persisted. The test
    /// `staged_chain_is_bit_identical_to_one_shot` convicts this.
    ///
    /// **Under Jacobi exchange the cadence `σ` is identity**
    /// (`#form-same-level-halo-exchange` FE(7)) and enters the key via
    /// [`HaloSchedule`]; staged exchange composition is not yet wired here —
    /// use [`Self::erosion_region_exchanged`] for the endpoint under a schedule.
    ///
    /// The walk is over the fixed ladder `stride, 2·stride, …, epochs` (the
    /// ladder is a function of the arguments, never of store contents —
    /// #form-depend-by-key-never-latest ); rungs already in the store seed the
    /// next rung without recompute, so a world built endpoint-only gains its
    /// interior for exactly the cost of one settle history, and a world built
    /// with this chain resumes any tail for free. Each computed rung also
    /// records its measured final-epoch residual (see
    /// [`Self::erosion_stage_residual_key`]).
    ///
    /// `stage_stride == 0` (or ≥ `epochs`) is endpoint-only, i.e. plain
    /// [`Self::erosion_tile`].
    pub fn erosion_tile_staged(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
        stage_stride: u32,
    ) -> (Vec<f32>, Source) {
        if stage_stride == 0 || stage_stride >= epochs {
            return self.erosion_tile(face, level, oi, oj, nx, epochs);
        }
        let mut ladder: Vec<u32> = (1..).map(|i| i * stage_stride).take_while(|k| *k < epochs).collect();
        ladder.push(epochs);

        // Deps and the seeded kernel are built lazily, on the first rung the
        // store is missing — an all-hit walk pulls nothing and computes nothing.
        let mut kernel: Option<Fluvial> = None;
        let mut heights: Option<Vec<f32>> = None; // state at `reached` epochs
        let mut reached = 0u32;
        let mut computed_any = false;
        for &k in &ladder {
            let key = self.erosion_key(face, level, oi, oj, nx, k, None);
            if let Some(bytes) = self.store.get(&key) {
                heights = Some(decode_f32(&bytes));
                reached = k;
                continue;
            }
            let mut f = match (kernel.take(), heights.take()) {
                // Later rung: seed from the predecessor stage's stored heights.
                (Some(f), Some(h)) => f.with_heights(h),
                (None, h) => {
                    let (initial_topo, _) = self.initial_topography(face, level, oi, oj, nx);
                    let (uplift, _) = self.uplift_tile(face, level, oi, oj, nx);
                    let (precip, _) = self.climate_tile(face, level, oi, oj, nx);
                    let mean = precip.iter().sum::<f32>() / precip.len().max(1) as f32;
                    let precip_weight: Vec<f32> = if mean > 0.0 {
                        precip.iter().map(|p| p / mean).collect()
                    } else {
                        vec![1.0; precip.len()]
                    };
                    let surf = |cell: CellId| -> f64 {
                        let (cf, ci, cj, _) = cell.to_face_ij();
                        if cf.index() == face.index() && ci >= oi && cj >= oj {
                            let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                            if di < nx && dj < nx {
                                return initial_topo[dj * nx + di] as f64;
                            }
                        }
                        gen::initial_topography_m(self.seed, cell, level)
                    };
                    let mut f = Fluvial::from_surface(self.seed, face, level, oi, oj, nx, surf);
                    f.set_uplift_rate(uplift);
                    f.set_precip_weight(precip_weight);
                    // A mid-ladder cold start (prior rungs were hits): resume
                    // from the last stored stage, not from the initial surface.
                    if let Some(h) = h {
                        f = f.with_heights(h);
                    }
                    f
                }
                (Some(_), None) => unreachable!("a built kernel always has state"),
            };
            f.erode(&FluvialParams { epochs: k - reached, ..Default::default() });
            self.put_memo(&key, &encode_f32(&f.h));
            let rkey = self.erosion_stage_residual_key(face, level, oi, oj, nx, k);
            self.put_memo(&rkey, &encode_f32(&[f.last_delta_m]));
            reached = k;
            computed_any = true;
            heights = Some(f.h.clone());
            kernel = Some(f);
        }
        let h = heights.expect("ladder is never empty");
        if computed_any {
            (h, Source::Computed)
        } else {
            let last = self.erosion_key(face, level, oi, oj, nx, epochs, None);
            (h, self.hit_source(&last))
        }
    }

    /// View-facing surface pull: **prefer a store-hit eroded tile**, else fall
    /// back to initial topography. Never triggers a cold erosion compute —
    /// views must not invent work the builder has not done; they only *show*
    /// what the store already holds (core/view wall: peers that query).
    ///
    /// Returns `(heights, source, eroded)` where `eroded` is true iff the
    /// surface came from a memoized fluvial tile at `epochs`.
    ///
    /// **Note:** this hits one complete key `(oi,oj,nx,epochs)`. The builder
    /// sweeps many 64×64 tiles; for a whole-face or free-roam view that must
    /// see *all* of them, use [`load_eroded_regions`] + [`assemble_surface_tile`].
    /// Halo-exchanged tiles (keys with `edge=halo|…`) are **not** selected here
    /// yet — views still see the default edge-sink cohort until a read path
    /// chooses the schedule.
    pub fn surface_prefer_eroded(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
    ) -> (Vec<f32>, Source, bool) {
        let key = self.erosion_key(face, level, oi, oj, nx, epochs, None);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), Source::Hit, true);
        }
        let (tile, src) = self.initial_topography(face, level, oi, oj, nx);
        (tile, src, false)
    }

    /// Census of `erosion-tile` roots by **source-hash freshness** — the loud
    /// signal a view needs so silent staleness stops masquerading as geography.
    ///
    /// Every nomos key folds the build-time whole-crate source digest
    /// ([`crate::nomotheke::SRC_HASH`], `#form-complete-content-addressed-key`).
    /// A root whose `src=` field differs from the current binary's hash was
    /// carved under a **different source tree** — its bytes are matured, but not
    /// this world's *current* surface. `load_eroded_regions` (below) does NOT
    /// filter on this, so a stale tile is loaded and shown as if current unless
    /// the caller consults this census / uses [`Self::load_current_eroded_regions`].
    pub fn eroded_region_census(&self) -> RegionCensus {
        let Ok(roots) = self.store.roots() else {
            return RegionCensus::default();
        };
        let mut c = RegionCensus::default();
        for r in &roots {
            if !r.key.starts_with("erosion-tile@") {
                continue;
            }
            if key_field(&r.key, "aspect").is_some() {
                continue; // stage-residual siblings are metadata, not surfaces
            }
            c.total += 1;
            if key_field(&r.key, "src") == Some(crate::nomotheke::SRC_HASH) {
                c.fresh += 1;
            } else {
                c.stale += 1;
            }
        }
        c
    }

    /// Materialize one **cohort's** `erosion-tile` roots as [`ErodedRegion`]s —
    /// the tiles carved under exactly the source tree `src`. Observe-only, pure
    /// store census; order is coarse → fine by level (required by
    /// [`erosion::surface_at`]).
    ///
    /// This is the cohort-safe convenient path ( `#norm-caught-disciplines-`
    /// `become-mechanisms` FE(2)(a)): a store holds beds carved under many
    /// source trees, and a reader that merges cohorts censuses a terrain nobody
    /// built — a fault class three independent readers hit on 2026-07-28 before
    /// the merging default was removed. Choosing the cohort is now part of the
    /// read. `watch::erosion_cohorts` enumerates what a store holds.
    pub fn load_eroded_regions_cohort(&self, src: &str) -> Vec<ErodedRegion> {
        self.load_eroded_regions_where(|key| key_field(key, "src") == Some(src))
    }

    /// [`Self::load_eroded_regions_cohort`] at the current binary's
    /// [`crate::nomotheke::SRC_HASH`] — the observe-only honest surface: a tile
    /// counts as *this* world's eroded state only if it was carved under the
    /// source now running. Stale tiles are dropped (loudly, via
    /// [`Self::eroded_region_census`]), never silently blended into the surface.
    pub fn load_current_eroded_regions(&self) -> Vec<ErodedRegion> {
        let cur = crate::nomotheke::SRC_HASH;
        self.load_eroded_regions_where(|key| key_field(key, "src") == Some(cur))
    }

    /// Load exactly the `erosion-tile` roots whose complete key `keep` accepts.
    ///
    /// Public because **replay needs it**: replaying a build in any renderer
    /// means assembling the surface from the roots that had landed by frame *n*,
    /// which is a key-set predicate and nothing more
    /// ( #form-time-indexed-stage-chains FE(5) — the live path and the replay
    /// path must be the same mechanism, and they are only the same mechanism if
    /// replay assembles through this function rather than through a private
    /// second loader). Callers that want the ordinary honest surface should use
    /// [`Self::load_current_eroded_regions`].
    ///
    /// **This is the sharp path, and cohort honesty is the caller's burden
    /// here**: a predicate that ignores `src=` merges beds carved under
    /// different source trees into a terrain nobody built (the fault class the
    /// removed no-filter loader made convenient — `#norm-caught-disciplines-`
    /// `become-mechanisms` FE(2)(a)). Replay predicates should pin a cohort
    /// alongside their landing cut, as the explorer's do.
    ///
    /// **One region per tile, the latest stage among accepted roots.** A staged
    /// build ([`Self::erosion_tile_staged`]) leaves *many* roots per tile — the
    /// settle history — and a surface is one moment, not a blend of moments:
    /// assembling two stages of the same tile would layer two datums whose
    /// difference is time, the same fault class as the stale-`src` ribbon. So
    /// per `(face, level, oi, oj, nx)` the highest-`epochs` accepted root wins.
    /// Replay composes correctly through this: "roots landed by frame *n*"
    /// yields the latest stage *as of that frame*, which is what the world
    /// looked like then. Stage-residual siblings (`aspect=` roots) are metadata,
    /// not surfaces, and are skipped.
    pub fn load_eroded_regions_where(&self, keep: impl Fn(&str) -> bool) -> Vec<ErodedRegion> {
        let Ok(roots) = self.store.roots() else {
            return Vec::new();
        };
        // Tile identity → (stage epochs, its region); BTree for deterministic order.
        type TileAt = (u8, u8, u32, u32, usize); // (face, level, oi, oj, nx)
        let mut latest: std::collections::BTreeMap<TileAt, (u32, ErodedRegion)> =
            std::collections::BTreeMap::new();
        for r in roots {
            if !r.key.starts_with("erosion-tile@") {
                continue;
            }
            if key_field(&r.key, "aspect").is_some() {
                continue;
            }
            if !keep(&r.key) {
                continue;
            }
            let Some(face_i) = key_field(&r.key, "face").and_then(|v| v.parse::<u8>().ok()) else {
                continue;
            };
            let Some(level) = key_field(&r.key, "level").and_then(|v| v.parse::<u8>().ok()) else {
                continue;
            };
            let Some(oi) = key_field(&r.key, "oi").and_then(|v| v.parse::<u32>().ok()) else {
                continue;
            };
            let Some(oj) = key_field(&r.key, "oj").and_then(|v| v.parse::<u32>().ok()) else {
                continue;
            };
            let Some(nx) = key_field(&r.key, "nx").and_then(|v| v.parse::<usize>().ok()) else {
                continue;
            };
            let epochs = key_field(&r.key, "epochs").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
            let at = (face_i, level, oi, oj, nx);
            if let Some((have, _)) = latest.get(&at) {
                if *have >= epochs {
                    continue; // an equal-or-later stage of this tile already loaded
                }
            }
            let Some(bytes) = self.store.object_bytes(&r.object) else {
                continue;
            };
            let h = decode_f32(&bytes);
            if h.len() != nx * nx {
                continue;
            }
            let region = ErodedRegion {
                face: Face::from_index(face_i),
                level,
                oi,
                oj,
                nx,
                h,
                seed: self.seed,
            };
            latest.insert(at, (epochs, region));
        }
        // BTreeMap order is (face, level, …); assembly requires coarse → fine.
        let mut out: Vec<ErodedRegion> = latest.into_values().map(|(_, r)| r).collect();
        out.sort_by_key(|r| (r.level, r.face.index(), r.oi, r.oj));
        out
    }

    /// Assemble an `nx×nx` height tile at `(face, level, oi, oj)` from loaded
    /// store regions + fated prior. **Observe-only:** no erosion compute, no
    /// store write. `any_eroded` is true if any cell was covered by a region.
    pub fn assemble_surface_tile(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        regions: &[ErodedRegion],
    ) -> (Vec<f32>, bool) {
        let mut tile = Vec::with_capacity(nx * nx);
        let mut any_eroded = false;
        for j in 0..nx as u32 {
            for i in 0..nx as u32 {
                let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
                if erosion::tier_at(cell, regions).is_some() {
                    any_eroded = true;
                }
                tile.push(erosion::surface_at(self.seed, cell, regions) as f32);
            }
        }
        (tile, any_eroded)
    }

    /// The complete key for a water tile — upstream identity folded in through
    /// both dependency versions plus the erosion run length its bed came from.
    fn water_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize, erosion_epochs: u32, steps: u32) -> Key {
        WATER
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .field("eepochs", erosion_epochs)
            .field("steps", steps)
            .with_dep_versions(&WATER)
    }

    /// System #3 — conserved shallow water settled on the eroded bed, *composed
    /// through the store*: pulls `erosion_tile` (memoized), runs a **fixed,
    /// deterministic** number of kernel steps (a bounded fill — never
    /// run-until-wall-clock, which would break build-order independence; the
    /// analytic hydrological init and component E's convergence-ε replace the
    /// fixed count later), and memoizes the standing-water depth field (m).
    /// Rivers and lakes exist in the store after this — fill once, hit forever
    /// (the property that retires the old testbench's re-fill-on-movement).
    /// Rain/evaporation carry the documented ~10× cycle fudge (ASSUMPTIONS.md
    /// "rain rate" / "water fill steps").
    pub fn water_tile(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        erosion_epochs: u32,
        steps: u32,
    ) -> (Vec<f32>, Source) {
        let key = self.water_key(face, level, oi, oj, nx, erosion_epochs, steps);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), self.hit_source(&key));
        }
        let (bed, _) = self.erosion_tile(face, level, oi, oj, nx, erosion_epochs);
        let (precip, _) = self.climate_tile(face, level, oi, oj, nx);
        let cell_m = crate::sample::cell_size_m(level, crate::planet::Planet::EARTH.radius_m) as f32;
        // Rain is now the climate nomos's PRINCIPLED rate — the conserved
        // reservoir's throughput (~1 m/yr for Earth), traceable to the ante-mundane
        // water-mass fraction — not a conjured constant. It is then sped up by a
        // declared **bounded-fill acceleration** so the fixed-step settle fills in a
        // bounded number of steps. The acceleration (NOT the rain) is what remains
        // unprincipled here, and the analytic hydrological init is what retires it.
        // (`ASSUMPTIONS.md` "bounded-fill acceleration".)
        const SEC_PER_YEAR: f64 = 365.25 * 86_400.0;
        const FILL_ACCEL: f64 = 9_000.0;
        // Precipitation is now spatially jittered, so take the tile MEAN — the water
        // kernel rains one uniform rate per tile. (Per-cell rain in the settle is a
        // further rung; erosion already consumes the full spatial field as a weight.)
        let precip_m_yr = if precip.is_empty() {
            0.0
        } else {
            precip.iter().map(|&p| p as f64).sum::<f64>() / precip.len() as f64
        };
        let precip_rate = (precip_m_yr / SEC_PER_YEAR * FILL_ACCEL) as f32;
        let sea = crate::sea_level::derived_sea_level_m(self.seed) as f32;
        let mut sim = crate::water::WaterSim::new(face, level, (oi, oj), nx, cell_m, bed, 2.0);
        let p = crate::water::WaterParams {
            precip: precip_rate,
            evaporation: 2.0e-4, // scaled with the accelerated cycle
            ocean_evap: 1.0e-4,
            sea_m: sea,
            ..Default::default()
        };
        for _ in 0..steps {
            sim.step(&p);
        }
        let depth = sim.depth.clone();
        self.put_memo(&key, &encode_f32(&depth));
        (depth, Source::Computed)
    }

    /// The **store-only** half of [`Self::water_tile`]: `Some` iff this exact
    /// water tile is already settled in the store. Never runs the fill kernel.
    ///
    /// A view must have this. `water_tile` on a miss runs `steps` iterations of
    /// the shallow-water kernel and, over a whole-globe census, that is minutes
    /// of cold evolution on whatever thread asked — exactly the
    /// `#form-builder-admission` FE(4) never-block clause, and it fires on the
    /// completely ordinary occasion of a source edit having moved every key's
    /// source hash. The honest view behaviour is to find no current water,
    /// display none, and say the tiles are stale; the dishonest one is to spend
    /// four minutes silently re-settling the planet so the picture looks the
    /// same as yesterday.
    #[allow(clippy::too_many_arguments)]
    pub fn water_tile_hit(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        erosion_epochs: u32,
        steps: u32,
    ) -> Option<(Vec<f32>, Source)> {
        let key = self.water_key(face, level, oi, oj, nx, erosion_epochs, steps);
        let bytes = self.store.get(&key)?;
        Some((decode_f32(&bytes), self.hit_source(&key)))
    }

    /// The complete key for an epoch reduction at mantle temperature `tp_c`.
    ///
    /// Keyed through the declared chain HEAD [`MANTLE_THERMAL`] — the nomos that
    /// indexes epochs (it produces `T_p`) and the head of the freeboard chain
    /// `mantle-thermal → lithosphere → isostasy → sea-level → erosion`. The
    /// reduction is a memoized *composition* of that already-declared chain, not a
    /// new article of law (LITHOSPHERE's own declaration names "wiring derived-sea
    /// as its own nomos edge" a deferred owned nicety — bordering the live isostasy
    /// seam — so this does NOT mint a new nomos). Completeness is guaranteed by
    /// `src=SRC_HASH` on the stem (the whole-crate source digest folds every module
    /// the pour/ledger touch — `sea_level`, `erosion_return`, `gen` bathymetry,
    /// hydrosphere — #form-complete-content-addressed-key FE(4)); the downstream
    /// chain versions are folded explicitly for legibility. `tp_bits` is the exact
    /// f64 so two epochs never alias; `aspect` distinguishes these global-scalar
    /// roots from the mantle-thermal driver itself.
    fn epoch_reduction_key(&self, tp_c: f64) -> Key {
        MANTLE_THERMAL
            .key()
            .field("seed", self.seed)
            .field("tp_bits", tp_c.to_bits())
            .field("aspect", "epoch-reduction")
            .field(LITHOSPHERE.name, LITHOSPHERE.version)
            .field(ISOSTASY.name, ISOSTASY.version)
            .field(HYDROSPHERE.name, HYDROSPHERE.version)
    }

    /// Pull the [`EpochReduction`] for mantle temperature `tp_c` through the store:
    /// Hit → decode + **stage** into the `sea_level` / `erosion_return` working-set
    /// caches (so every subsequent per-cell `tectonic_surface_at_tp` read in this
    /// process is O(1), with no pour); Miss → compute the four global reductions
    /// once and Put the store citizen. This is where "the cost belongs at build
    /// time" is realized: a warmed process that Hits never runs the ~393k-cell pour
    /// or ledger passes at all (`#form-store-as-save` FE(6), decided).
    ///
    /// A Hit is byte-identical to a fresh compute (the reductions are pure f64
    /// functions of the keyed inputs) — the staleness/purity conviction the probe
    /// carries.
    /// The **store-only** half of [`Self::epoch_reduction`]: `Some` iff the
    /// builder has already materialized this epoch, `None` otherwise. Never
    /// computes and never writes.
    ///
    /// This is what lets an explorer answer *"is this stage built, or am I
    /// looking at something this process just worked out?"* — the difference an
    /// instrument for checking the world's systems has to be able to state, and
    /// that `epoch_reduction`'s `(value, Source)` pair can only report *after*
    /// paying for the miss. A view calls this first, shows the honest unbuilt
    /// state, and only then decides whether to compute a labeled view-side
    /// estimate off the frame path.
    pub fn epoch_reduction_hit(&self, tp_c: f64) -> Option<(EpochReduction, Source)> {
        let key = self.epoch_reduction_key(tp_c);
        let bytes = self.store.get(&key)?;
        let r = EpochReduction::from_bytes(&bytes)?;
        sea_level::prime_derived_sea_pre_ledger(self.seed, tp_c, r.pre_ledger_sea_m);
        erosion_return::prime_ledger(self.seed, tp_c, r.deposit_m, r.post_reference_m);
        erosion_return::prime_derived_sea_after_erosion(self.seed, tp_c, r.derived_sea_m);
        Some((r, self.hit_source(&key)))
    }

    pub fn epoch_reduction(&self, tp_c: f64) -> (EpochReduction, Source) {
        let key = self.epoch_reduction_key(tp_c);
        if let Some(bytes) = self.store.get(&key) {
            if let Some(r) = EpochReduction::from_bytes(&bytes) {
                // Stage the store-owned reduction into the per-process caches, so
                // the pour/ledger never run in this warmed process (FE(6): RAM is
                // staging of what the store owns, never a home of record).
                sea_level::prime_derived_sea_pre_ledger(self.seed, tp_c, r.pre_ledger_sea_m);
                erosion_return::prime_ledger(self.seed, tp_c, r.deposit_m, r.post_reference_m);
                erosion_return::prime_derived_sea_after_erosion(self.seed, tp_c, r.derived_sea_m);
                return (r, self.hit_source(&key));
            }
        }
        // Miss: compute the four global reductions (each stages its own cache as a
        // side effect; order respects the chain — pre-ledger pour, then ledger,
        // then the post-erosion pour that reads both).
        let pre_ledger_sea_m = sea_level::derived_sea_level_pre_ledger_at_tp(self.seed, tp_c);
        let (deposit_m, post_reference_m) = erosion_return::ledger_scalars(self.seed, tp_c);
        let derived_sea_m = erosion_return::derived_sea_level_after_erosion_at_tp(self.seed, tp_c);
        let r = EpochReduction { pre_ledger_sea_m, deposit_m, post_reference_m, derived_sea_m };
        self.put_memo(&key, &r.to_bytes());
        (r, Source::Computed)
    }
}

fn encode_f32(v: &[f32]) -> Vec<u8> {
    let mut b = Vec::with_capacity(v.len() * 4);
    for &x in v {
        b.extend_from_slice(&x.to_le_bytes());
    }
    b
}

fn decode_f32(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

/// Pull `name=value` from a canonical complete-key string.
fn key_field<'a>(key: &'a str, name: &str) -> Option<&'a str> {
    let pfx = format!("{name}=");
    key.split('|').find_map(|f| f.strip_prefix(&pfx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("vivarium-query-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    // ---- The round-trip probe -------------------------------------------------
    //
    // `#form-depend-by-key-never-latest` FE(1) states BUILD-ORDER INDEPENDENCE as
    // law: two builds of the same vivium advanced along *different demand orders*
    // converge to byte-identical state wherever both have materialized. It has sat
    // at `status: exact` with no instrument — which `#norm-probes-before-claims`
    // forbids for a behaviour claim. The archive named this probe as owed
    // ("checkpoint round-trip probe — resume vs run-through must agree; promote the
    // two-leg cache test from anecdote to instrument",
    // `.super-archive/from-archive/architecture-migration-2026-07-03.md`) and it was
    // never carried. This is it.
    //
    // SENSITIVITY (`#norm-probe-sensitivity`). Against pure functions of a complete
    // key these tests would pass vacuously, and a vacuous green here would be worse
    // than nothing — it would retire an honest open question. The surface that can
    // actually break the law is the three process-global caches `epoch_reduction`
    // primes on every Hit (`PRE_LEDGER_SEA_CACHE`, `LEDGER_CACHE`, `POST_SEA_CACHE`).
    // All three are keyed `(seed, tp_bits)` today; one keyed by `seed` alone would
    // make epoch N's value depend on whether epoch M ran first. The legs below
    // therefore share ONE process on purpose, so those caches stay hot across them —
    // the adversarial condition, not the convenient one — and the third test proves
    // the compared bytes really do flow through that surface.

    /// The three round-trip tests all mutate process-global L1 memos — two clear
    /// them, one deliberately poisons one. `cargo test` runs them on separate
    /// threads in one process, so without serialising, a clear can land between
    /// the poison and the read and the known-bad goes flaky. Flakiness in the
    /// instrument that certifies a law is worse than no instrument: it trains
    /// people to re-run until green.
    static CACHE_TESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn reduction_bits(r: &EpochReduction) -> [u64; 4] {
        // Bit patterns, not float comparison: the claim is *byte-identical*, and an
        // epsilon here would be a tolerance sized to hide the defect.
        [
            r.pre_ledger_sea_m.to_bits(),
            r.deposit_m.to_bits(),
            r.post_reference_m.to_bits(),
            r.derived_sea_m.to_bits(),
        ]
    }

    fn epoch_tps() -> Vec<f64> {
        crate::mantle_thermal::cooling_stages()
            .iter()
            .map(|&t| crate::mantle_thermal::potential_temp_c(t))
            .collect()
    }

    #[test]
    fn epoch_ladder_is_order_independent() {
        let _guard = CACHE_TESTS.lock().unwrap_or_else(|e| e.into_inner());
        let tps = epoch_tps();
        let seed = 20_260_724;
        let n = tps.len();

        let run = |tag: &str, order: Vec<usize>| -> Vec<[u64; 4]> {
            // Independent samples, not one sample counted three times. Without
            // this the legs share the process-global L1 memos, and a cache keyed
            // too coarsely corrupts all three identically — the comparison then
            // passes while the law is broken. That is not hypothetical: an
            // under-keyed variant was injected on 2026-07-24 and the leg-vs-leg
            // form of this test passed it.
            crate::sea_level::clear_pre_ledger_cache_for_test();
            crate::erosion_return::clear_caches_for_test();

            let dir = tmpdir(tag);
            let s = Store::open(&dir).unwrap();
            let w = World::new(&s, seed);
            let mut out = vec![[0u64; 4]; n];
            for i in order {
                out[i] = reduction_bits(&w.epoch_reduction(tps[i]).0);
            }
            out
        };

        let forward = run("roundtrip-fwd", (0..n).collect());
        let reverse = run("roundtrip-rev", (0..n).rev().collect());
        let interleaved =
            run("roundtrip-int", (0..n).step_by(2).chain((1..n).step_by(2)).collect());

        assert_eq!(forward, reverse, "reverse demand order must agree bit-for-bit");
        assert_eq!(forward, interleaved, "interleaved demand order must agree bit-for-bit");

        // Second, independent discriminator: the cooling trajectory gives every
        // epoch a distinct waterline (measured 5211 → 5012 m across the ladder,
        // strictly monotone). A cache that collapses epochs onto one value would
        // still be *consistent* across legs — invisible above — but shows here.
        // `#norm-probe-sensitivity` FE(4): a statistic identical across samples
        // that should differ is a defect signature, not a measurement.
        let seas: Vec<u64> = forward.iter().map(|b| b[0]).collect();
        let distinct: std::collections::BTreeSet<_> = seas.iter().collect();
        assert_eq!(
            distinct.len(),
            n,
            "each epoch must carry its own pre-ledger waterline; {} distinct of {n} \
             means the ladder collapsed onto a shared cache entry",
            distinct.len()
        );
    }

    #[test]
    fn resume_equals_run_through() {
        let _guard = CACHE_TESTS.lock().unwrap_or_else(|e| e.into_inner());
        let tps = epoch_tps();
        let seed = 20_260_725;
        let split = tps.len() / 2;

        let dir_a = tmpdir("roundtrip-through");
        let s_a = Store::open(&dir_a).unwrap();
        let through: Vec<_> = {
            let w = World::new(&s_a, seed);
            tps.iter().map(|&tp| reduction_bits(&w.epoch_reduction(tp).0)).collect()
        };

        // Leg two: materialize a prefix, then CLOSE the store and reopen it with a
        // fresh `World` before finishing — the "stop the builder, come back later"
        // case the archive's wording was about.
        let dir_b = tmpdir("roundtrip-resume");
        {
            let s = Store::open(&dir_b).unwrap();
            let w = World::new(&s, seed);
            for &tp in &tps[..split] {
                let _ = w.epoch_reduction(tp);
            }
        }
        let s_b = Store::open(&dir_b).unwrap();
        let w_b = World::new(&s_b, seed);
        let resumed: Vec<_> = tps
            .iter()
            .map(|&tp| {
                let (r, src) = w_b.epoch_reduction(tp);
                (reduction_bits(&r), src)
            })
            .collect();

        // The resume is real: the prefix comes back from the store, not recomputed.
        for (i, (_, src)) in resumed.iter().enumerate().take(split) {
            assert_ne!(*src, Source::Computed, "epoch {i} should resume from the store");
        }

        let resumed_bits: Vec<_> = resumed.into_iter().map(|(b, _)| b).collect();
        assert_eq!(through, resumed_bits, "resume must equal run-through, bit for bit");
    }

    #[test]
    fn a_poisoned_cache_is_visible_to_the_round_trip_comparison() {
        let _guard = CACHE_TESTS.lock().unwrap_or_else(|e| e.into_inner());
        // The known-bad the two tests above need to mean anything: if a wrong value
        // injected into the priming cache did NOT change the compared bytes, then
        // those legs are comparing something that does not flow through the cache
        // surface, and their agreement would prove nothing.
        //
        // Contained by construction: the caches are keyed `(seed, tp_bits)`, and
        // this seed and `tp` are used by no other test, so the poison cannot leak
        // into a sibling running in the same process.
        let seed = 424_242;
        let tp = 1_600.5_f64;

        let dir = tmpdir("roundtrip-clean");
        let s = Store::open(&dir).unwrap();
        let clean = World::new(&s, seed).epoch_reduction(tp).0;

        crate::sea_level::prime_derived_sea_pre_ledger(seed, tp, clean.pre_ledger_sea_m + 1_000.0);

        let dir_p = tmpdir("roundtrip-poisoned");
        let s_p = Store::open(&dir_p).unwrap();
        let poisoned = World::new(&s_p, seed).epoch_reduction(tp).0;

        assert_ne!(
            reduction_bits(&clean),
            reduction_bits(&poisoned),
            "a poisoned pre-ledger cache must move the reduction; if it cannot, the \
             order-independence legs are not exercising the surface that could break \
             the law and their green is vacuous"
        );
    }

    #[test]
    fn initial_topography_computes_then_memoizes() {
        let dir = tmpdir("initial-topography");
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 0);
        let (a1, src1) = w.initial_topography(Face::from_index(2), 19, 1000, 2000, 16);
        assert_eq!(src1, Source::Computed, "first pull computes");
        assert_eq!(a1.len(), 16 * 16, "tile is nx × nx");
        let (a2, src2) = w.initial_topography(Face::from_index(2), 19, 1000, 2000, 16);
        assert_eq!(src2, Source::Hit, "second pull hits the store");
        assert_eq!(a1, a2, "a hit returns exactly the bytes it computed");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn walking_the_world_memoizes_and_persists() {
        // The principled scaffold, demonstrated headless: pull tiles along a
        // path, then revisit. Revisits hit the store — matured state persists,
        // no recompute, no re-seed. This IS "the whole world in play, memoizing
        // as it rounds," at MVP scale.
        let dir = tmpdir("walk");
        let face = Face::from_index(1);
        let nx = 8;
        // A → B → C → back to A
        let path = [(100u32, 100u32), (108, 100), (108, 108), (100, 100)];
        let mut sources = Vec::new();
        {
            let s = Store::open(&dir).unwrap();
            let w = World::new(&s, 0);
            for &(oi, oj) in &path {
                let (_t, src) = w.initial_topography(face, 19, oi, oj, nx);
                sources.push(src);
            }
        }
        assert_eq!(
            sources,
            vec![Source::Computed, Source::Computed, Source::Computed, Source::Hit],
            "A,B,C fresh; returning to A hits the store (persisted)"
        );
        // Survives a fresh open — the store IS the save.
        let s2 = Store::open(&dir).unwrap();
        let w2 = World::new(&s2, 0);
        let (_t, src) = w2.initial_topography(face, 19, 100, 100, nx);
        assert_eq!(src, Source::Hit, "reopened store still holds the walked world");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_eroded_regions_sees_builder_tiles_not_only_full_face_keys() {
        // Builder writes 64×64 tiles; a full-face surface_prefer_eroded key does
        // not hit them. Census → ErodedRegion must recover the carved surface.
        let dir = tmpdir("census-eroded");
        let face = Face::from_index(2);
        let (level, nx, epochs) = (6u8, 16usize, 5u32);
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 7);
        let (_h, src) = w.erosion_tile(face, level, 0, 0, nx, epochs);
        assert_eq!(src, Source::Computed);
        let regions = w.load_current_eroded_regions();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].nx, nx);
        assert_eq!(regions[0].level, level);
        let (tile, any) = w.assemble_surface_tile(face, level, 0, 0, nx, &regions);
        assert!(any, "assembled tile must report eroded coverage");
        assert_eq!(tile.len(), nx * nx);
        // Pure prior path (no regions) still works and does not claim eroded.
        let (_prior, none) = w.assemble_surface_tile(face, level, 0, 0, nx, &[]);
        assert!(!none);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn stale_source_tiles_are_loaded_silently_but_the_census_and_current_loader_separate_them() {
        // The ribbon fault class, at the loader boundary (#form-sphere-continuous-
        // surface-fields FE(3) view-assembly debt; #form-builder-admission FE(4)).
        //
        // KNOWN-BAD the probe must convict: `load_eroded_regions` filters roots by
        // the `erosion-tile@` prefix alone and ignores the `src=` source-hash, so a
        // tile carved under a DIFFERENT source tree loads and assembles as if it
        // were this world's current surface — silent staleness. After any
        // vivarium-world source edit the whole store is stale-by-src, yet the globe
        // kept painting it (Joseph's edit -> globe loop, 2026-07-24): the coverage
        // boundary between stale-carved cells and current-source prior fallback is
        // the ribbon.
        //
        // The instrument: `eroded_region_census` counts fresh vs stale, and
        // `load_current_eroded_regions` drops stale so the surface is single-datum.
        use crate::store::{Key, PutOpts};
        let dir = tmpdir("stale-src");
        let face = Face::from_index(2);
        let (level, nx, epochs) = (6u8, 16usize, 5u32);
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 7);

        // A genuine, current-source eroded tile.
        assert_eq!(w.erosion_tile(face, level, 0, 0, nx, epochs).1, Source::Computed);
        let c0 = w.eroded_region_census();
        assert_eq!((c0.total, c0.fresh, c0.stale), (1, 1, 0), "one fresh tile, no stale");
        assert_eq!(w.load_current_eroded_regions().len(), 1);

        // A hand-forged tile carved under a DIFFERENT source tree: identical
        // coordinates, but src = a hash that is not the current binary's.
        let stale_key = Key::new("erosion-tile", "erosion-stale-test")
            .field("src", "deadbeefdeadbeef")
            .field("seed", 7u64)
            .field("face", face.index())
            .field("level", level)
            .field("oi", 128u32)
            .field("oj", 0u32)
            .field("nx", nx);
        s.put_with(&stale_key, &encode_f32(&vec![1234.0f32; nx * nx]), PutOpts::default()).unwrap();

        // The merging default is GONE (the mechanism): only an explicit
        // predicate can still express a cross-cohort read, and it costs the
        // predicate — which is the point.
        assert_eq!(w.load_eroded_regions_where(|_| true).len(), 2, "the sharp path can merge, explicitly");

        // The instruments separate them.
        let c1 = w.eroded_region_census();
        assert_eq!((c1.total, c1.fresh, c1.stale), (2, 1, 1), "census surfaces the stale tile");
        let current = w.load_current_eroded_regions();
        assert_eq!(current.len(), 1, "current-only loader drops the stale tile");
        assert!(current.iter().all(|r| !(r.oi == 128 && r.h.iter().all(|&h| h == 1234.0))), "stale bytes excluded");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn staged_chain_is_bit_identical_to_one_shot() {
        // THE property that makes stage stride demand rather than identity
        // ( #form-manifest-prescribes-vivium FE(5); #form-time-indexed-stage-
        // chains FE(8) ): a stage at epochs=k holds the same bytes whether built
        // as a chain rung or as a one-shot run. If this test ever breaks, the
        // stride has leaked into artifact content and MUST move into the key.
        // Two cadences, deliberately adversarial: equal legs with an unequal
        // tail (7,3 → 3+3+1) and a stride coprime to the count with a long
        // remainder (11,4 → 4+4+3). A single cadence convicts only the seams it
        // happens to cut (the odd/even-restart lesson from Earth-system-model
        // test suites — a chain test at one cadence passed for years while
        // other offsets diverged).
        for (epochs, stride, interior) in [(7u32, 3u32, vec![3u32, 6]), (11, 4, vec![4, 8])] {
            let (dir_a, dir_b) = (tmpdir("stage-oneshot"), tmpdir("stage-chain"));
            let face = Face::from_index(2);
            let (level, nx) = (6u8, 16usize);

            let sa = Store::open(&dir_a).unwrap();
            let wa = World::new(&sa, 7);
            let (one_shot, src_a) = wa.erosion_tile(face, level, 0, 0, nx, epochs);
            assert_eq!(src_a, Source::Computed);

            let sb = Store::open(&dir_b).unwrap();
            let wb = World::new(&sb, 7);
            let (staged, src_b) = wb.erosion_tile_staged(face, level, 0, 0, nx, epochs, stride);
            assert_eq!(src_b, Source::Computed);

            assert!(
                one_shot.iter().zip(staged.iter()).all(|(a, b)| a.to_bits() == b.to_bits()),
                "chained settle history must be BIT-identical to the one-shot run (epochs={epochs}, stride={stride})"
            );
            // The interior is addressable: every ladder rung is a store citizen.
            for &k in &interior {
                let (_h, _src, eroded) = wb.surface_prefer_eroded(face, level, 0, 0, nx, k);
                assert!(eroded, "interior stage epochs={k} must be a keyed citizen");
                let r = wb.erosion_stage_residual(face, level, 0, 0, nx, k);
                assert!(
                    r.is_some_and(f32::is_finite),
                    "computed stage epochs={k} must carry its measured residual"
                );
            }
            // ...and the one-shot world has none — endpoint only, no interior.
            let (_h, _src, eroded) = wa.surface_prefer_eroded(face, level, 0, 0, nx, interior[0]);
            assert!(!eroded, "a one-shot build has no interior to show");

            let _ = fs::remove_dir_all(&dir_a);
            let _ = fs::remove_dir_all(&dir_b);
        }
    }

    #[test]
    fn endpoint_only_world_gains_its_interior_and_readers_see_one_surface() {
        // The migration path for every world built before stages existed: the
        // endpoint citizen stays valid (same key shape), the staged walk fills
        // in the missing history, and surface readers still see exactly one
        // region per tile — the latest stage — never a blend of moments.
        let dir = tmpdir("stage-migrate");
        let face = Face::from_index(2);
        let (level, nx, epochs, stride) = (6u8, 16usize, 7u32, 3u32);
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 7);

        // An old world: endpoint only.
        let (endpoint, src) = w.erosion_tile(face, level, 0, 0, nx, epochs);
        assert_eq!(src, Source::Computed);

        // The staged walk computes the interior (rungs 3, 6) without disturbing
        // the endpoint, and lands on the same bytes.
        let (staged, src) = w.erosion_tile_staged(face, level, 0, 0, nx, epochs, stride);
        assert_eq!(src, Source::Computed, "interior rungs were missing and got built");
        assert!(
            endpoint.iter().zip(staged.iter()).all(|(a, b)| a.to_bits() == b.to_bits()),
            "the endpoint reached through the chain is the endpoint that was already there"
        );

        // A second staged walk is all hits — the chain is a store citizen now.
        let (_h, src) = w.erosion_tile_staged(face, level, 0, 0, nx, epochs, stride);
        assert_eq!(src, Source::Hit, "a fully materialized chain walks for free");

        // Readers: three stage roots, ONE region, and it is the latest stage.
        let regions = w.load_current_eroded_regions();
        assert_eq!(regions.len(), 1, "one region per tile — a surface is one moment");
        assert!(
            regions[0].h.iter().zip(endpoint.iter()).all(|(a, b)| a.to_bits() == b.to_bits()),
            "the region is the latest stage"
        );

        // The interior census sees the chain: 3 distinct time-indices.
        let roots = s.roots().unwrap();
        let reports = crate::watch::interior(&roots);
        let er = reports.iter().find(|r| r.nomos == "erosion-tile").expect("erosion-tile in census");
        assert_eq!(er.distinct, 3, "epochs 3, 6, 7 — the settle history is addressable");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn erosion_composes_on_the_spine_and_memoizes() {
        // The coupling proof: erosion pulls the initial-topography as a dependency (memoizing
        // it), composes on it, and its own result memoizes — all through the
        // store, no shared mutable state.
        let dir = tmpdir("erosion");
        let face = Face::from_index(2);
        let (nx, epochs) = (32usize, 20u32);
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 0);

        let (e1, src1) = w.erosion_tile(face, 19, 1000, 2000, nx, epochs);
        assert_eq!(src1, Source::Computed, "first erosion pull computes");
        assert_eq!(e1.len(), nx * nx);
        assert!(e1.iter().all(|x| x.is_finite()), "eroded field is finite");

        // Erosion's pull memoized BOTH its dependencies (the recursion): the
        // initial-topography surface it carves and the uplift field it carves against.
        let (_sp, spine_src) = w.initial_topography(face, 19, 1000, 2000, nx);
        assert_eq!(spine_src, Source::Hit, "the initial-topography dependency was memoized by erosion's pull");
        let (_up, uplift_src) = w.uplift_tile(face, 19, 1000, 2000, nx);
        assert_eq!(uplift_src, Source::Hit, "the uplift dependency was memoized by erosion's pull");

        // Re-pull erosion → hit, and deterministic:
        let (e2, src2) = w.erosion_tile(face, 19, 1000, 2000, nx, epochs);
        assert_eq!(src2, Source::Hit);
        assert_eq!(e1, e2, "a hit returns exactly the eroded bytes it computed");
        let _ = fs::remove_dir_all(&dir);
    }

    /// Production path for `#form-same-level-halo-exchange`: a region carved
    /// under Jacobi exchange memoizes each interior under a key that carries the
    /// schedule, and a second pull is a pure hit. A different schedule is a
    /// different article of law (descriptor in the key, not a silent retune).
    #[test]
    fn exchanged_region_memoizes_under_schedule_keys() {
        let dir = tmpdir("halo-exchange");
        let face = Face::from_index(1);
        let (level, tile_n, epochs) = (8u8, 16usize, 12u32);
        let (oi, oj) = (64u32, 64u32);
        let schedule = HaloSchedule { depth: 4, cadence: 4, cone_rho: 0 };
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 17_425_063_241_017_297_386);

        let (tiles, src) = w.erosion_region_exchanged(face, level, oi, oj, tile_n, 2, 2, epochs, schedule);
        assert_eq!(src, Source::Computed);
        assert_eq!(tiles.len(), 4);
        assert!(tiles.iter().all(|t| t.h.len() == tile_n * tile_n && t.h.iter().all(|v| v.is_finite())));

        let (again, src2) = w.erosion_region_exchanged(face, level, oi, oj, tile_n, 2, 2, epochs, schedule);
        assert_eq!(src2, Source::Hit, "second region pull must be all-hit");
        for (a, b) in tiles.iter().zip(again.iter()) {
            assert!(
                a.h.iter().zip(b.h.iter()).all(|(x, y)| x.to_bits() == y.to_bits()),
                "hit returns the same interior bytes"
            );
        }

        // Different σ ⇒ different keys ⇒ compute, not a silent overwrite of the first.
        let other = HaloSchedule { depth: 4, cadence: 6, cone_rho: 0 };
        let (alt, src_alt) =
            w.erosion_region_exchanged(face, level, oi, oj, tile_n, 2, 2, epochs, other);
        assert_eq!(src_alt, Source::Computed, "a different schedule must miss");
        // First schedule still hits (keys did not collide).
        assert_eq!(
            w.erosion_region_exchanged(face, level, oi, oj, tile_n, 2, 2, epochs, schedule)
                .1,
            Source::Hit
        );
        let _ = alt;
        let _ = fs::remove_dir_all(&dir);
    }

    /// Tripwire: the production exchange path is **not** a re-key of plain
    /// tiling. At unit-test grain the beacon FE(7) "closer to single-field"
    /// table is not reproducible cheaply (that table is `halo_exchange_probe`'s
    /// job at L13/300 epochs). What must hold here: exchanged interiors differ
    /// from independently carved edge-sink tiles at the same coordinates, so a
    /// schedule key that silently aliased the plain path would fail.
    #[test]
    fn exchanged_region_is_not_a_plain_tiling_alias() {
        let dir = tmpdir("halo-vs-plain");
        let face = Face::from_index(1);
        let seed = 17_425_063_241_017_297_386u64;
        let (level, tile_n, epochs) = (8u8, 16usize, 24u32);
        let (oi, oj) = (128u32, 128u32);
        let schedule = HaloSchedule { depth: 4, cadence: 4, cone_rho: 0 };
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, seed);

        let (ex, _) = w.erosion_region_exchanged(face, level, oi, oj, tile_n, 2, 2, epochs, schedule);
        let mut differed = 0usize;
        let mut cells = 0usize;
        for t in &ex {
            let (plain, _) = w.erosion_tile(face, level, t.oi, t.oj, tile_n, epochs);
            for (a, b) in t.h.iter().zip(plain.iter()) {
                cells += 1;
                if a.to_bits() != b.to_bits() {
                    differed += 1;
                }
            }
        }
        assert!(
            differed * 10 > cells,
            "exchange must move a substantial fraction of cells vs plain edge-sink \
             tiling at the same keys' coordinates ({differed}/{cells} differ) — \
             otherwise the schedule is a no-op alias"
        );
        // And the plain key must still be a different article: pulling plain
        // after exchange is a Hit on the no-halo key, not the halo key.
        let (_, plain_src) = w.erosion_tile(face, level, oi, oj, tile_n, epochs);
        assert_eq!(plain_src, Source::Hit);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn waived_provisional_root_surfaces_on_the_hit_path() {
        // De-novo audit P0 residual A, end-to-end at lib level: a root written
        // under waived admission (PutOpts.provisional) must reach the consumer
        // as HitProvisional — matured bytes, NOT laundered into a lawful Hit —
        // and must count in the census. (The bin-level argv→status walk stays
        // named on #form-builder-admission; this is the truth chain under it.)
        use crate::store::PutOpts;
        let dir = tmpdir("provisional-hit");
        let face = Face::from_index(2);
        let nx = 16usize;
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 0);

        // Lawful pull first: computes and memoizes lawfully.
        let (t1, src1) = w.initial_topography(face, 19, 1000, 2000, nx);
        assert_eq!(src1, Source::Computed);
        assert_eq!(w.initial_topography(face, 19, 1000, 2000, nx).1, Source::Hit, "lawful root reads as lawful Hit");

        // Re-mark the same key provisional (what a waived build writes).
        let key = w.initial_topography_key(face, 19, 1000, 2000, nx);
        s.put_with(&key, &encode_f32(&t1), PutOpts { provisional: true }).unwrap();
        assert!(s.is_provisional(&key));

        let (t2, src2) = w.initial_topography(face, 19, 1000, 2000, nx);
        assert_eq!(src2, Source::HitProvisional, "waived root must surface, not launder");
        assert!(src2.is_hit(), "still a memoization hit");
        assert_eq!(t1, t2, "provisional affects lawfulness metadata, never bytes");

        // Census sees it too.
        let prov = s.roots().unwrap().iter().filter(|r| r.provisional).count();
        assert_eq!(prov, 1, "census counts the waived root");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn water_composes_on_erosion_and_memoizes() {
        // System #3 through the same loop: water pulls the eroded bed
        // (memoizing erosion AND initial-topography on the way), settles deterministically,
        // memoizes. The three-system dependency chain, proven end to end.
        let dir = tmpdir("water");
        let face = Face::from_index(2);
        let (nx, eepochs, steps) = (32usize, 20u32, 60u32);
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 0);
        let (d1, src1) = w.water_tile(face, 19, 2000, 3000, nx, eepochs, steps);
        assert_eq!(src1, Source::Computed);
        assert_eq!(d1.len(), nx * nx);
        assert!(d1.iter().all(|x| x.is_finite() && *x >= 0.0), "depths finite + non-negative");
        assert!(d1.iter().any(|x| *x > 0.01), "somewhere there is standing water (sea or pond)");
        // The chain memoized its dependencies:
        assert_eq!(w.erosion_tile(face, 19, 2000, 3000, nx, eepochs).1, Source::Hit);
        assert_eq!(w.initial_topography(face, 19, 2000, 3000, nx).1, Source::Hit);
        // Re-pull hits and is byte-identical (deterministic bounded fill):
        let (d2, src2) = w.water_tile(face, 19, 2000, 3000, nx, eepochs, steps);
        assert_eq!(src2, Source::Hit);
        assert_eq!(d1, d2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn epoch_reduction_hit_equals_recompute_byte_identical() {
        // The staleness/purity conviction for the epoch-reduction store citizen
        // (#form-store-as-save FE(6), decided): the reduction is a pure f64 function
        // of the keyed inputs, so a store Hit must return EXACTLY the value a fresh
        // compute produced and persisted — no drift, no lie. If the compute ever
        // depended on unkeyed state (or the encode/decode lost a bit) this fails.
        use crate::lithosphere::MANTLE_TP_C;
        let dir = tmpdir("epoch-reduction");
        let tp = MANTLE_TP_C;
        let s = Store::open(&dir).unwrap();

        // Cold: compute + persist.
        let (r1, src1) = World::new(&s, 7).epoch_reduction(tp);
        assert_eq!(src1, Source::Computed, "first pull computes the reduction");

        // Warm (fresh World, same store): Hit, byte-identical.
        let (r2, src2) = World::new(&s, 7).epoch_reduction(tp);
        assert_eq!(src2, Source::Hit, "second pull hits the store citizen");
        assert_eq!(r1, r2, "a store Hit returns exactly the reduction that was computed");

        // The stored derived sea IS the canonical present-Abyssal waterline (the
        // value the whole codebase reads via derived_sea_level_m) — the citizen is
        // not a parallel truth, it is the same law value memoized to disk.
        assert_eq!(
            r1.derived_sea_m,
            crate::sea_level::derived_sea_level_m(7),
            "reduction.derived_sea_m must equal the canonical derived sea at MANTLE_TP_C"
        );

        // Survives a fresh store open — the store IS the save.
        let s2 = Store::open(&dir).unwrap();
        assert_eq!(World::new(&s2, 7).epoch_reduction(tp).1, Source::Hit, "reopened store still holds the reduction");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn seeds_are_distinct_worlds_in_one_store() {
        // Two Worlds sharing one object pool never alias: the seed is in every
        // key, so world A's tiles and world B's tiles coexist, and the same
        // coordinates yield different terrain.
        let dir = tmpdir("seeds");
        let s = Store::open(&dir).unwrap();
        let (wa, wb) = (World::new(&s, 1), World::new(&s, 2));
        let face = Face::from_index(2);
        let (ta, _) = wa.initial_topography(face, 19, 1000, 2000, 16);
        let (tb, src_b) = wb.initial_topography(face, 19, 1000, 2000, 16);
        assert_eq!(src_b, Source::Computed, "world B must not hit world A's memo");
        assert_ne!(ta, tb, "different seeds ⇒ different terrain at the same coordinates");
        // And each re-pull hits its own:
        assert_eq!(wa.initial_topography(face, 19, 1000, 2000, 16).1, Source::Hit);
        assert_eq!(wb.initial_topography(face, 19, 1000, 2000, 16).1, Source::Hit);
        let _ = fs::remove_dir_all(&dir);
    }
}
