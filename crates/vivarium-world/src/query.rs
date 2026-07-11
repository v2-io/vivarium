//! The lazy pull-query + the first recipes — the runtime as a demand-driven,
//! memoized query graph (`DESIGN-REDUX.md` §11).
//!
//! Queries are methods on a [`World`]: the context that owns `(store, seed)`
//! **together**, so the same field feeds both the key construction and the
//! compute — the world-seed in the key and the world-seed in the KRNG draws
//! *cannot* diverge, because there is only one source (Joseph's question,
//! 2026-07-10: "is it wise to rely on coders always putting the right seed in
//! the KRNG?" — no; this struct is the structural answer). A `World` is built
//! from a manifest (`spec.rs`) in one place; recipes never see a bare seed.
//!
//! A query is *coordinate-addressed*: it builds a complete [`Key`] from its
//! (recipe, version, seed, region, resolution) inputs, checks the [`Store`], and
//! on a miss computes via the recipe and memoizes the result. Walking the world
//! is then just pulling the tiles around the observer — revisits hit the store,
//! so matured state **persists** (no re-seed-from-raw-prior; the store is the
//! save). Dependencies between systems become recursion in the pull.

use crate::erosion::{Fluvial, FluvialParams};
use crate::gen;
use crate::sphere::{CellId, Face};
use crate::store::{Key, Store};

/// Where a pulled value came from — the memoization signal, so callers (and the
/// HUD, later) can *see* the world being built once and reused.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    /// Freshly computed and memoized (a store miss).
    Computed,
    /// Served from the store (a hit) — matured state, persisted.
    Hit,
}

/// Recipe version for the coarse spine. Constant for the MVP; bump on any change
/// to [`World::compute_spine_tile`] (it graduates to a source-derived hash later
/// — see `DESIGN-REDUX.md` §12). Under-keying is the one unsafe failure, so this
/// is part of the key.
const SPINE_VERSION: &str = "spine-2026-07-10b-sphere3d";

/// Recipe version for the fluvial-erosion tier (system #2). Bump on any change
/// to the erosion recipe.
const EROSION_VERSION: &str = "erosion-2026-07-10a";

/// One vivium, opened for querying: the store it persists in and the seed that
/// (with the law) IS its identity (LEXICON §4; `vivium-operational-workflow.md`
/// Stage 0). Construct via [`World::new`] — normally from a loaded manifest
/// (`spec::WorldSpec`), the one place a bare seed is handled.
pub struct World<'s> {
    store: &'s Store,
    seed: u64,
}

impl<'s> World<'s> {
    pub fn new(store: &'s Store, seed: u64) -> Self {
        World { store, seed }
    }

    /// The world-seed (read-only — identity is set at construction).
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The complete key for a spine tile: every input folded in (§12).
    fn spine_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Key {
        Key::new("spine-tile", SPINE_VERSION)
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
    }

    /// System #1 — the fBm coarse spine: a `nx × nx` tile of band-limited
    /// surface-prior elevations (m), a pure function of (seed, face, level,
    /// origin, nx) via the coordinate-hashed prior. This is the conservation-
    /// honest first light: land vs water, before any principled tectonics.
    fn compute_spine_tile(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(nx * nx);
        for j in 0..nx as u32 {
            for i in 0..nx as u32 {
                let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
                out.push(gen::surface_prior_m(self.seed, cell, level) as f32);
            }
        }
        out
    }

    /// Pull a spine tile through the store: hit → load; miss → compute + memoize.
    /// Returns the tile (row-major, `nx × nx`) and whether it was computed or served.
    pub fn spine_tile(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
    ) -> (Vec<f32>, Source) {
        let key = self.spine_key(face, level, oi, oj, nx);
        if let Some(bytes) = self.store.get(&key) {
            return (decode_f32(&bytes), Source::Hit);
        }
        let tile = self.compute_spine_tile(face, level, oi, oj, nx);
        let _ = self.store.put(&key, &encode_f32(&tile));
        (tile, Source::Computed)
    }

    /// The complete key for an eroded tile — including the *upstream* dependency's
    /// identity (the spine version, §12): if the spine changes, this key changes
    /// and the eroded tile recomputes.
    fn erosion_key(&self, face: Face, level: u8, oi: u32, oj: u32, nx: usize, epochs: u32) -> Key {
        Key::new("erosion-tile", EROSION_VERSION)
            .field("seed", self.seed)
            .field("face", face.index())
            .field("level", level)
            .field("oi", oi)
            .field("oj", oj)
            .field("nx", nx)
            .field("epochs", epochs)
            .field("spine", SPINE_VERSION)
    }

    /// System #2 — the fluvial-erosion tier, *composed on the spine through the
    /// store*. On a miss it **pulls its input surface from the spine** (which
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
        // Dependency: pull the spine surface (memoized — recurses into system #1).
        let (spine, _) = self.spine_tile(face, level, oi, oj, nx);
        // Seed erosion from the pulled spine; any cell the kernel samples outside
        // the tile (edge/halo) falls back to the prior — identical values, since
        // the spine IS the prior at this rung.
        let surf = |cell: CellId| -> f64 {
            let (cf, ci, cj, _) = cell.to_face_ij();
            if cf.index() == face.index() && ci >= oi && cj >= oj {
                let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                if di < nx && dj < nx {
                    return spine[dj * nx + di] as f64;
                }
            }
            gen::surface_prior_m(self.seed, cell, level)
        };
        let mut f = Fluvial::from_surface(self.seed, face, level, oi, oj, nx, surf);
        f.erode(&FluvialParams { epochs, ..Default::default() });
        let eroded = f.h.clone();
        let _ = self.store.put(&key, &encode_f32(&eroded));
        (eroded, Source::Computed)
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
    fn spine_tile_computes_then_memoizes() {
        let dir = tmpdir("spine");
        let s = Store::open(&dir).unwrap();
        let w = World::new(&s, 0);
        let (a1, src1) = w.spine_tile(Face::from_index(2), 19, 1000, 2000, 16);
        assert_eq!(src1, Source::Computed, "first pull computes");
        assert_eq!(a1.len(), 16 * 16, "tile is nx × nx");
        let (a2, src2) = w.spine_tile(Face::from_index(2), 19, 1000, 2000, 16);
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
                let (_t, src) = w.spine_tile(face, 19, oi, oj, nx);
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
        let (_t, src) = w2.spine_tile(face, 19, 100, 100, nx);
        assert_eq!(src, Source::Hit, "reopened store still holds the walked world");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn erosion_composes_on_the_spine_and_memoizes() {
        // The coupling proof: erosion pulls the spine as a dependency (memoizing
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

        // Erosion's pull memoized its spine dependency (the recursion):
        let (_sp, spine_src) = w.spine_tile(face, 19, 1000, 2000, nx);
        assert_eq!(spine_src, Source::Hit, "the spine dependency was memoized by erosion's pull");

        // Re-pull erosion → hit, and deterministic:
        let (e2, src2) = w.erosion_tile(face, 19, 1000, 2000, nx, epochs);
        assert_eq!(src2, Source::Hit);
        assert_eq!(e1, e2, "a hit returns exactly the eroded bytes it computed");
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
        let (ta, _) = wa.spine_tile(face, 19, 1000, 2000, 16);
        let (tb, src_b) = wb.spine_tile(face, 19, 1000, 2000, 16);
        assert_eq!(src_b, Source::Computed, "world B must not hit world A's memo");
        assert_ne!(ta, tb, "different seeds ⇒ different terrain at the same coordinates");
        // And each re-pull hits its own:
        assert_eq!(wa.spine_tile(face, 19, 1000, 2000, 16).1, Source::Hit);
        assert_eq!(wb.spine_tile(face, 19, 1000, 2000, 16).1, Source::Hit);
        let _ = fs::remove_dir_all(&dir);
    }
}
