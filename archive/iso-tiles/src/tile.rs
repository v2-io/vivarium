//! The memoization core. A cell's drawable (top face + front walls) is rasterised
//! once per distinct *key* and cached; every later cell with the same key is a free
//! lookup + blit. This file owns: the key (what makes two cells look identical),
//! the neighbour-averaged continuous corner heights (the seamless-relief trick),
//! a tiny software polygon rasteriser, and the cache.
//!
//! The key is deliberately a function of *quantized* local geometry only — height
//! banded, neighbour slopes clamped — so the vocabulary of distinct tiles is small
//! and the cache stays tiny even over a continent. That is the whole performance
//! thesis; `main.rs`'s HUD measures whether it holds.

use std::collections::HashMap;

use crate::iso::Projector;
use crate::world::SurfaceCell;
use bevy::math::Vec2;

/// Voxels per height band. Heights are quantized to bands before keying — this is
/// neworld's hard `/45` quantization, the thing that bounds the tile vocabulary.
/// Coarser bands read as distinct terraces/plateaus (calmer, more legible, more
/// neworld-like) and shrink the vocabulary; finer bands read as a busy faceted
/// mesh. At detail 2, 12 voxels = 6 m steps.
pub const BAND_VOX: i32 = 12;

/// Neighbour slope deltas (in bands) are clamped to ±this for the key. Real terrain
/// is locally smooth, so this rarely bites; it caps the worst-case key space.
const MAX_SLOPE: i32 = 2;

/// The 8 neighbour offsets in cell space, ordered N, NE, E, SE, S, SW, W, NW.
/// Index arithmetic below relies on this order (even = edge, odd = diagonal).
pub const NEIGHBORS: [(i32, i32); 8] =
    [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

/// What uniquely determines a cell's appearance. Two cells with the same key
/// rasterise identically, so the second is a cache hit. Orientation & zoom are NOT
/// in the key: the renderer clears the cache when they change (they alter every
/// tile's *shape*), keeping the key small.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileKey {
    /// Surface material id.
    material: u16,
    /// Water depth bucket (0 = dry); coarse, for the water tint.
    water: u8,
    /// **Coarse** elevation bucket for atmospheric tint — NOT the absolute band.
    /// This is the load-bearing fix: keying on absolute height made every elevation
    /// a distinct tile and exploded the vocabulary (16 % hit-rate). A handful of
    /// tint buckets keeps the vocabulary tiny; relief comes from `deltas`, which are
    /// *relative* and so independent of absolute height.
    tint: u8,
    /// Per-neighbour band deltas relative to centre, clamped to ±MAX_SLOPE. These
    /// shape the continuous top face (corner displacement).
    deltas: [i8; 8],
    /// Downward drop (in bands, 0..=15) to each of the 4 edge neighbours (N,E,S,W),
    /// UNclamped by MAX_SLOPE. Separate from `deltas` because a wall must span the
    /// *true* cliff height to a lower neighbour — clamping that (as the shape does)
    /// is what produced gaps on steep slopes; a fixed skirt over-corrected into
    /// giant curtains. A coarse 0..15-band drop covers real cliffs with little
    /// vocabulary cost.
    drops: [u8; 4],
}

/// Number of elevation tint buckets (sea → peak). Few, so the vocabulary stays small.
const TINT_BUCKETS: i32 = 16;
/// Voxels of elevation span the tint ramp covers before saturating (~peak height).
const TINT_SPAN_VOX: i32 = 5400;

/// Banded height in voxels — the height the renderer must anchor a tile at so that
/// adjacent tiles agree on shared corners (continuity). Public for frame.rs.
pub fn banded_height_vox(h: i32) -> f32 {
    band(h) as f32 * BAND_VOX as f32
}

/// A rasterised tile: RGBA pixels plus where to place it relative to the cell's
/// projected anchor (the cell centre at its banded height).
pub struct TileBitmap {
    pub w: usize,
    pub h: usize,
    pub data: Vec<u8>, // RGBA8, row-major
    pub off_x: i32,
    pub off_y: i32,
}

/// `key → bitmap`. Cleared on orientation/zoom change.
pub struct TileCache {
    map: HashMap<TileKey, TileBitmap>,
    pub hits: u64,
    pub misses: u64,
}

impl TileCache {
    pub fn new() -> Self {
        Self { map: HashMap::new(), hits: 0, misses: 0 }
    }
    pub fn clear(&mut self) {
        self.map.clear();
        self.hits = 0;
        self.misses = 0;
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

/// Build the key for a cell from its sample and its 8 neighbours' samples.
pub fn key_for(center: &SurfaceCell, neighbors: &[SurfaceCell; 8], sea_level: i32) -> TileKey {
    let cband = band(center.height_vox);
    let mut deltas = [0i8; 8];
    for i in 0..8 {
        let d = (band(neighbors[i].height_vox) - cband).clamp(-MAX_SLOPE, MAX_SLOPE);
        deltas[i] = d as i8;
    }
    let water = if center.wet {
        ((center.water_depth_vox / 4).clamp(0, 6) + 1) as u8 // 1..=7 buckets; 0 = dry
    } else {
        0
    };
    let tint = (((center.height_vox - sea_level).max(0) * TINT_BUCKETS / TINT_SPAN_VOX)
        .clamp(0, TINT_BUCKETS - 1)) as u8;
    // True downward drop (unclamped) to each edge neighbour — N,E,S,W — for walls.
    let edge_ni = [0usize, 2, 4, 6];
    let mut drops = [0u8; 4];
    for (k, &ni) in edge_ni.iter().enumerate() {
        drops[k] = (cband - band(neighbors[ni].height_vox)).clamp(0, 15) as u8;
    }
    TileKey { material: center.material, water, tint, deltas, drops }
}

#[inline]
fn band(h: i32) -> i32 {
    // Round-to-nearest band so the surface doesn't bias downward.
    (h as f32 / BAND_VOX as f32).round() as i32
}

/// Get the cached tile for `key`, rasterising it on a miss. `proj` supplies the
/// orientation/zoom-dependent geometry (its `ex`/`ez`/`hy` basis); `sea_level`
/// tints elevation. The geometry is computed in the cell's *local* frame (relative
/// to its anchor), which is translation-invariant, so it is valid for every cell
/// sharing the key.
pub fn get_or_raster<'a>(
    cache: &'a mut TileCache,
    key: TileKey,
    proj: &Projector,
    sea_level: i32,
) -> &'a TileBitmap {
    if cache.map.contains_key(&key) {
        cache.hits += 1;
    } else {
        cache.misses += 1;
        let bmp = rasterize(&key, proj, sea_level);
        cache.map.insert(key, bmp);
    }
    cache.map.get(&key).unwrap()
}

// --- Rasterisation ---------------------------------------------------------------

/// Corner indices around the top face, CCW in cell space: the four cell corners
/// (cx,cz)=(0,0),(1,0),(1,1),(0,1). Each corner height is the average of the four
/// cells meeting there (centre + 2 edge neighbours + 1 diagonal), in bands — this
/// is what makes adjacent tiles share corner heights and meet seamlessly.
fn corner_bands(key: &TileKey) -> [f32; 4] {
    // Neighbour delta lookups by NEIGHBORS index: N0 NE1 E2 SE3 S4 SW5 W6 NW7.
    let d = |i: usize| key.deltas[i] as f32;
    // corner (0,0): centre + W(6) + N(0) + NW(7)
    let c00 = (0.0 + d(6) + d(0) + d(7)) / 4.0;
    // corner (1,0): centre + E(2) + N(0) + NE(1)
    let c10 = (0.0 + d(2) + d(0) + d(1)) / 4.0;
    // corner (1,1): centre + E(2) + S(4) + SE(3)
    let c11 = (0.0 + d(2) + d(4) + d(3)) / 4.0;
    // corner (0,1): centre + W(6) + S(4) + SW(5)
    let c01 = (0.0 + d(6) + d(4) + d(5)) / 4.0;
    [c00, c10, c11, c01]
}

fn rasterize(key: &TileKey, proj: &Projector, sea_level: i32) -> TileBitmap {
    // DEBUG: VIVARIUM_TILES_FLAT renders flat diamonds (no relief, no walls) to
    // isolate whether confetti gaps come from relief/walls or from placement/scale.
    let flat = std::env::var_os("VIVARIUM_TILES_FLAT").is_some();
    // Corner heights in voxels, relative to the cell-centre band (local frame).
    let cb = if flat { [0.0; 4] } else { corner_bands(key) };
    let corner_h: [f32; 4] = [
        cb[0] * BAND_VOX as f32,
        cb[1] * BAND_VOX as f32,
        cb[2] * BAND_VOX as f32,
        cb[3] * BAND_VOX as f32,
    ];
    // Local screen positions of the four top corners (anchor = cell centre, h 0).
    // project() is affine; using a zero focus and centre cell gives local offsets.
    let corner_cell = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let anchor = local_anchor(proj);
    let top: [Vec2; 4] = std::array::from_fn(|i| {
        local_project(proj, corner_cell[i].0, corner_cell[i].1, corner_h[i]) - anchor
    });

    // Base colour from material + elevation tint, then slope shading for the top.
    let base = material_rgb(key.material);
    let tinted = elevation_tint(base, key.tint);
    let _ = sea_level;
    let top_rgb = if key.water > 0 {
        water_rgb(key.water)
    } else {
        let shade = top_shade(&corner_h, proj);
        scale_rgb(tinted, shade)
    };

    // Walls (column sides) are what fill the vertical risers between tiles at
    // different heights — the thing the flat-mode diagnostic showed was missing.
    // Each of the two FRONT edges (midpoint below the cell centre on screen) drops
    // straight down to its lower edge-neighbour by that neighbour's band delta,
    // plus a small overlap so even equal-height neighbours leave no hairline seam.
    // Edges (corner_a, corner_b, edge_neighbor_index→drops index): N edge 0-1 (drops
    // 0), E edge 1-2 (drops 1), S edge 2-3 (drops 2), W edge 3-0 (drops 3). Each
    // front wall drops by the TRUE cliff height to that neighbour (key.drops, in
    // bands), plus a small overlap to kill hairline seams. Because the drop is the
    // real riser, flat ground draws only the thin overlap (no curtains) while a
    // genuine cliff draws a tall face — the fix for both the gaps and the giant
    // skirts.
    const WALL_OVERLAP_VOX: f32 = 2.0;
    let edges = [(0usize, 1usize, 0usize), (1, 2, 1), (2, 3, 2), (3, 0, 3)];
    let mut walls: Vec<([Vec2; 4], [u8; 3])> = Vec::new();
    if !flat {
        for &(a, b, di) in &edges {
            let mid = (top[a] + top[b]) * 0.5;
            if mid.y <= 0.0 {
                continue; // back-facing edge — hidden behind this tile's own top
            }
            let riser = key.drops[di] as f32 * BAND_VOX as f32;
            let down = Vec2::new(0.0, (riser + WALL_OVERLAP_VOX) * -proj.hy);
            let wall = [top[a], top[b], top[b] + down, top[a] + down];
            // Differentiate the two front faces (left a touch brighter than right).
            let f = if mid.x < 0.0 { 0.62 } else { 0.48 };
            walls.push((wall, scale_rgb(tinted, f)));
        }
    }

    // Bounding box over all geometry → local buffer.
    let mut min = Vec2::splat(f32::MAX);
    let mut max = Vec2::splat(f32::MIN);
    let mut acc = |p: Vec2| {
        min = min.min(p);
        max = max.max(p);
    };
    for p in &top {
        acc(*p);
    }
    for (w, _) in &walls {
        for p in w {
            acc(*p);
        }
    }
    let off_x = min.x.floor() as i32;
    let off_y = min.y.floor() as i32;
    let w = (max.x.ceil() - min.x.floor()).max(1.0) as usize + 1;
    let h = (max.y.ceil() - min.y.floor()).max(1.0) as usize + 1;
    let mut data = vec![0u8; w * h * 4];
    let to_local = |p: Vec2| Vec2::new(p.x - off_x as f32, p.y - off_y as f32);

    // Walls first (behind), then the top face over them.
    for (quad, rgb) in &walls {
        fill_quad(&mut data, w, h, &[to_local(quad[0]), to_local(quad[1]), to_local(quad[2]), to_local(quad[3])], *rgb);
    }
    fill_quad(&mut data, w, h, &[to_local(top[0]), to_local(top[1]), to_local(top[2]), to_local(top[3])], top_rgb);

    TileBitmap { w, h, data, off_x, off_y }
}

/// project() but in a translation-free local frame (focus = origin cell at h 0),
/// so the result is the cell-local offset usable for every cell with this key.
fn local_project(proj: &Projector, cx: f32, cz: f32, height_vox: f32) -> Vec2 {
    proj.ex * cx + proj.ez * cz + Vec2::new(0.0, height_vox * proj.hy)
}
fn local_anchor(proj: &Projector) -> Vec2 {
    // Cell centre at height 0 in the local frame.
    proj.ex * 0.5 + proj.ez * 0.5
}

/// Lambert-ish shade for the top face from its corner heights, lit from up-left.
fn top_shade(corner_h: &[f32; 4], proj: &Projector) -> f32 {
    // Slope in world units: cell edge length in metres ≈ |ex|/px_per_m is awkward
    // here; use band-scale heights vs a nominal cell span for relative slope.
    let dzdx = (corner_h[1] + corner_h[2]) - (corner_h[0] + corner_h[3]);
    let dzdy = (corner_h[3] + corner_h[2]) - (corner_h[0] + corner_h[1]);
    // Treat one cell span as ~BAND_VOX*2 voxels for slope normalisation (tuned).
    let span = (BAND_VOX as f32) * 2.0;
    let n = bevy::math::Vec3::new(-dzdx / span, span / span, -dzdy / span).normalize();
    let light = bevy::math::Vec3::new(-0.5, 1.0, -0.35).normalize();
    let diff = n.dot(light).max(0.0);
    let _ = proj;
    (0.55 + 0.55 * diff).clamp(0.35, 1.15)
}

fn material_rgb(mat: u16) -> [u8; 3] {
    match mat {
        1 => [133, 130, 128], // STONE
        2 => [117, 99, 79],   // DIRT
        3 => [112, 133, 92],  // GRASS
        4 => [102, 133, 153], // WATER (rarely the surface voxel)
        _ => [120, 118, 116],
    }
}

fn water_rgb(bucket: u8) -> [u8; 3] {
    // Shallow → deep by bucket (1..=7).
    let t = ((bucket as f32 - 1.0) / 6.0).clamp(0.0, 1.0);
    let shallow = [84.0, 140.0, 200.0];
    let deep = [20.0, 55.0, 120.0];
    [
        (shallow[0] + (deep[0] - shallow[0]) * t) as u8,
        (shallow[1] + (deep[1] - shallow[1]) * t) as u8,
        (shallow[2] + (deep[2] - shallow[2]) * t) as u8,
    ]
}

/// Lighten with elevation (atmospheric-ish), so peaks read pale and valleys deep.
/// `tint` is the coarse 0..TINT_BUCKETS bucket from the key.
fn elevation_tint(rgb: [u8; 3], tint: u8) -> [u8; 3] {
    let t = tint as f32 / (TINT_BUCKETS - 1) as f32; // 0..1
    let f = 0.85 + 0.5 * t; // valleys a touch darker, peaks paler
    scale_rgb(rgb, f)
}

#[inline]
fn scale_rgb(rgb: [u8; 3], f: f32) -> [u8; 3] {
    [
        (rgb[0] as f32 * f).clamp(0.0, 255.0) as u8,
        (rgb[1] as f32 * f).clamp(0.0, 255.0) as u8,
        (rgb[2] as f32 * f).clamp(0.0, 255.0) as u8,
    ]
}

/// Fill a convex quad (4 verts) as two triangles into an RGBA buffer (opaque).
fn fill_quad(data: &mut [u8], w: usize, h: usize, q: &[Vec2; 4], rgb: [u8; 3]) {
    fill_tri(data, w, h, q[0], q[1], q[2], rgb);
    fill_tri(data, w, h, q[0], q[2], q[3], rgb);
}

/// Scanline-fill a triangle into an RGBA buffer, opaque. Bounds-clamped.
fn fill_tri(data: &mut [u8], w: usize, h: usize, a: Vec2, b: Vec2, c: Vec2, rgb: [u8; 3]) {
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(h as f32 - 1.0) as i32;
    if max_y < min_y {
        return;
    }
    let area = edge(a, b, c);
    if area.abs() < 1e-6 {
        return;
    }
    for y in min_y..=max_y {
        let py = y as f32 + 0.5;
        // Find x-range by intersecting the scanline with the three edges.
        let mut lo = f32::MAX;
        let mut hi = f32::MIN;
        for (p, q) in [(a, b), (b, c), (c, a)] {
            if (p.y <= py && q.y > py) || (q.y <= py && p.y > py) {
                let t = (py - p.y) / (q.y - p.y);
                let x = p.x + t * (q.x - p.x);
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
        if hi < lo {
            continue;
        }
        let x0 = lo.floor().max(0.0) as i32;
        let x1 = hi.ceil().min(w as f32 - 1.0) as i32;
        for x in x0..=x1 {
            let idx = (y as usize * w + x as usize) * 4;
            data[idx] = rgb[0];
            data[idx + 1] = rgb[1];
            data[idx + 2] = rgb[2];
            data[idx + 3] = 255;
        }
    }
}

#[inline]
fn edge(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iso::{Projector, ZOOMS};

    fn cell(h: i32, mat: u16) -> SurfaceCell {
        SurfaceCell { height_vox: h, material: mat, water_depth_vox: 0, water_speed: 0.0, wet: false }
    }

    #[test]
    fn flat_neighbours_give_zero_slope_key() {
        let c = cell(100, 3);
        let ns = [c; 8];
        let k = key_for(&c, &ns, 6);
        assert_eq!(k.deltas, [0i8; 8]);
        assert_eq!(k.water, 0);
    }

    #[test]
    fn identical_cells_share_a_key_two_distinct_tiles() {
        // Two flat-grass cells at the same band must hit the cache (1 raster).
        let c = cell(100, 3);
        let ns = [c; 8];
        let key = key_for(&c, &ns, 6);
        let proj = Projector::new(1, ZOOMS[1], Vec2::ZERO, 0.0, 2, Vec2::new(1280.0, 720.0));
        let mut cache = TileCache::new();
        get_or_raster(&mut cache, key, &proj, 6);
        get_or_raster(&mut cache, key, &proj, 6);
        assert_eq!(cache.len(), 1, "same key must memoize to one tile");
        assert_eq!(cache.misses, 1);
        assert_eq!(cache.hits, 1);
    }

    #[test]
    fn slope_is_clamped_to_bound_the_vocabulary() {
        let c = cell(0, 1);
        let mut ns = [c; 8];
        ns[0] = cell(10_000, 1); // absurd neighbour
        let k = key_for(&c, &ns, 6);
        assert_eq!(k.deltas[0], MAX_SLOPE as i8);
    }
}
