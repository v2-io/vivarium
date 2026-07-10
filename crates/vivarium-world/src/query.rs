//! The lazy pull-query + the first recipe — the runtime as a demand-driven,
//! memoized query graph (`DESIGN-REDUX.md` §11), with one system so far.
//!
//! A query is *coordinate-addressed*: it builds a complete [`Key`] from its
//! (recipe, version, region, resolution) inputs, checks the [`Store`], and on a
//! miss computes via the recipe and memoizes the result. Walking the world is
//! then just pulling the tiles around the observer — revisits hit the store, so
//! matured state **persists** (no re-seed-from-raw-prior; the store is the
//! save). Dependencies between systems become recursion in the pull; system #1
//! (the spine) has none, so this increment is the loop at its simplest.

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
/// to [`compute_spine_tile`] (it graduates to a source-derived hash later — see
/// `DESIGN-REDUX.md` §12). Under-keying is the one unsafe failure, so this is
/// part of the key.
const SPINE_VERSION: &str = "spine-2026-07-10a";

/// The complete key for a spine tile: every input folded in (§12).
fn spine_key(face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Key {
    Key::new("spine-tile", SPINE_VERSION)
        .field("face", face.index())
        .field("level", level)
        .field("oi", oi)
        .field("oj", oj)
        .field("nx", nx)
}

/// System #1 — the fBm coarse spine: a `nx × nx` tile of band-limited
/// surface-prior elevations (m), a pure function of (face, level, origin, nx)
/// via the coordinate-hashed prior. This is the conservation-honest first
/// light: land vs water, before any principled tectonics or erosion.
fn compute_spine_tile(face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
            out.push(gen::surface_prior_m(cell, level) as f32);
        }
    }
    out
}

/// Pull a spine tile through the store: hit → load; miss → compute + memoize.
/// Returns the tile (row-major, `nx × nx`) and whether it was computed or served.
pub fn spine_tile(
    store: &Store,
    face: Face,
    level: u8,
    oi: u32,
    oj: u32,
    nx: usize,
) -> (Vec<f32>, Source) {
    let key = spine_key(face, level, oi, oj, nx);
    if let Some(bytes) = store.get(&key) {
        return (decode_f32(&bytes), Source::Hit);
    }
    let tile = compute_spine_tile(face, level, oi, oj, nx);
    let _ = store.put(&key, &encode_f32(&tile));
    (tile, Source::Computed)
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
        let (a1, src1) = spine_tile(&s, Face::from_index(2), 19, 1000, 2000, 16);
        assert_eq!(src1, Source::Computed, "first pull computes");
        assert_eq!(a1.len(), 16 * 16, "tile is nx × nx");
        let (a2, src2) = spine_tile(&s, Face::from_index(2), 19, 1000, 2000, 16);
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
            for &(oi, oj) in &path {
                let (_t, src) = spine_tile(&s, face, 19, oi, oj, nx);
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
        let (_t, src) = spine_tile(&s2, face, 19, 100, 100, nx);
        assert_eq!(src, Source::Hit, "reopened store still holds the walked world");
        let _ = fs::remove_dir_all(&dir);
    }
}
