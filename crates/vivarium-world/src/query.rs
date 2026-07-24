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

use crate::erosion::{self, ErodedRegion, Fluvial, FluvialParams};
use crate::gen;
use crate::nomotheke::{CLIMATE, EROSION, HYDROSPHERE, INITIAL_TOPOGRAPHY, UPLIFT, WATER};
use crate::sphere::{CellId, Face};
use crate::store::{Key, PutOpts, Store};

/// Where a pulled value came from — the memoization signal, so callers (and the
/// HUD, later) can *see* the world being built once and reused.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    /// Freshly computed and memoized (a store miss).
    Computed,
    /// Served from the store (a hit) — matured state, persisted.
    Hit,
}

// Nomos identities (name, version, epistemic declaration, promises,
// assumptions) live in the NOMOTHEKE (`nomotheke.rs`) — the registry is the
// only key-mint for world-law computations, so an undeclared nomos cannot
// reach the store. Bump a version by re-declaring there (source-derived
// versions remain the §12 target).

/// One vivium, opened for querying: the store it persists in and the seed that
/// (with the law) IS its identity (LEXICON §4; `doc/plan/vivium-operational-workflow.md`
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
                return (h, Source::Hit);
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
            return (decode_f32(&bytes), Source::Hit);
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
            return (decode_f32(&bytes), Source::Hit);
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
            return (decode_f32(&bytes), Source::Hit);
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
    fn erosion_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize, epochs: u32) -> Key {
        EROSION
            .key()
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .field("epochs", epochs)
            .with_dep_versions(&EROSION)
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
        let key = self.erosion_key(face, level, oi, oj, nx, epochs);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), Source::Hit);
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
    pub fn surface_prefer_eroded(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
    ) -> (Vec<f32>, Source, bool) {
        let key = self.erosion_key(face, level, oi, oj, nx, epochs);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), Source::Hit, true);
        }
        let (tile, src) = self.initial_topography(face, level, oi, oj, nx);
        (tile, src, false)
    }

    /// Materialize every `erosion-tile` root as an [`ErodedRegion`] for observe-
    /// only sampling. Never runs the fluvial kernel — pure store census.
    /// Order is coarse → fine by level (required by [`erosion::surface_at`]).
    pub fn load_eroded_regions(&self) -> Vec<ErodedRegion> {
        let Ok(roots) = self.store.roots() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for r in roots {
            if !r.key.starts_with("erosion-tile@") {
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
            let Some(bytes) = self.store.object_bytes(&r.object) else {
                continue;
            };
            let h = decode_f32(&bytes);
            if h.len() != nx * nx {
                continue;
            }
            out.push(ErodedRegion {
                face: Face::from_index(face_i),
                level,
                oi,
                oj,
                nx,
                h,
                seed: self.seed,
            });
        }
        out.sort_by_key(|r| r.level);
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
            return (decode_f32(&bytes), Source::Hit);
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
        let regions = w.load_eroded_regions();
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
