//! Sample the frame into dense field [`Patch`]es for a view — **data only, no
//! rendering** (the core/view wall holds). A view picks a cube-face region and gets
//! height/water patches by generating a column per cell. The generator is swappable
//! (baseline now, erosion/higher LOD later), which is exactly what lets a view test
//! memoization at different algorithm/LOD levels.

use crate::chunk::Patch;
use crate::gen;
use crate::sphere::{CellId, Face};

/// Surface fields over a face region, in metres.
pub struct SurfacePatch {
    /// Solid surface height above the bedrock datum.
    pub height: Patch<f32>,
    /// Standing water depth above the solid surface.
    pub water: Patch<f32>,
}

/// Metres per face cell at `level` — a cube-face edge is ~a quarter great circle.
pub fn cell_size_m(level: u8, planet_radius_m: f64) -> f64 {
    (std::f64::consts::FRAC_PI_2 * planet_radius_m) / (1u64 << level) as f64
}

/// Sample a `w×w` region of `face` at `level` (origin = interior top-left face cell),
/// with a 1-cell halo (for edge normals), using the baseline generator. One column
/// generated per cell — the swap point for erosion / caching later.
pub fn sample_surface(face: Face, level: u8, origin_i: u32, origin_j: u32, w: usize) -> SurfacePatch {
    let mut height = Patch::new(face, level, origin_i, origin_j, w, 1);
    let mut water = Patch::new(face, level, origin_i, origin_j, w, 1);
    let n = 1i64 << level;
    for yy in -1..(w as isize + 1) {
        for xx in -1..(w as isize + 1) {
            let gi = origin_i as i64 + xx as i64;
            let gj = origin_j as i64 + yy as i64;
            if (0..n).contains(&gi) && (0..n).contains(&gj) {
                let col = gen::baseline_column(CellId::from_face_ij(face, gi as u32, gj as u32, level));
                height.set(xx, yy, col.solid_thickness_m() as f32);
                water.set(xx, yy, col.water_depth.value as f32);
            }
        }
    }
    SurfacePatch { height, water }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_plausible_terrain() {
        // A large enough region to span noise variation (a small patch can be
        // entirely above OR below sea level — so land-presence is not an invariant;
        // plausible heights and non-negative water are).
        let s = sample_surface(Face::ZPos, 10, 200, 200, 128);
        let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
        for y in 0..128 {
            for x in 0..128 {
                let h = s.height.get(x, y);
                assert!((0.0..8000.0).contains(&h), "height {h} implausible");
                assert!(s.water.get(x, y) >= 0.0, "negative water");
                lo = lo.min(h);
                hi = hi.max(h);
            }
        }
        // over 128 cells at level 10 the terrain should actually vary
        assert!(hi - lo > 1.0, "terrain is flat over a large region ({lo}..{hi})");
    }

    #[test]
    fn deterministic() {
        let a = sample_surface(Face::YPos, 12, 500, 700, 8);
        let b = sample_surface(Face::YPos, 12, 500, 700, 8);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(a.height.get(x, y), b.height.get(x, y));
            }
        }
    }
}
