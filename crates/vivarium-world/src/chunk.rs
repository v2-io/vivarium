//! The Cartesian patch — the dense working substrate automata run on
//! (#form-cellid-chunk-patch; `ref/research/spatial-key-bench.md`).
//!
//! A Hilbert [`CellId`] *addresses* a region; *inside*, cells are a plain
//! row-major array so a stencil's neighbours are `idx±1` — ~80× faster than
//! curve-id neighbours, ~6 Gcells/s while the patch fits cache (keep patches
//! ~0.25–0.5 km, i.e. 256²–1024²). A `halo` ghost border holds cross-patch
//! neighbours for the same-face fill at load; **cross-cube-face** halo fill
//! is designed, unbuilt (loader-owned — `#form-cellid-chunk-patch`). Interior
//! stencils never branch on boundaries. One `Patch<T>` per field (SoA). API
//! shape driven by first consumer, the erosion tier (`ref/erosion-port/NOTES.md`).

use crate::sphere::{CellId, Face};

/// A square Cartesian patch of one field `T`: a `w×w` interior plus a `halo`-thick
/// ghost border, over a region of one cube face at a fixed `level`. Interior coords
/// are `0..w`; halo cells are reachable at `-halo..w+halo` (so edge stencils read
/// neighbours without branching).
#[derive(Clone, Debug)]
pub struct Patch<T> {
    pub face: Face,
    pub level: u8,
    /// Interior top-left, in face-cell coords at `level`.
    pub origin_i: u32,
    pub origin_j: u32,
    pub w: usize,
    pub halo: usize,
    stride: usize,
    data: Vec<T>, // stride*stride, row-major
}

impl<T: Copy + Default> Patch<T> {
    pub fn new(face: Face, level: u8, origin_i: u32, origin_j: u32, w: usize, halo: usize) -> Self {
        let stride = w + 2 * halo;
        Self { face, level, origin_i, origin_j, w, halo, stride, data: vec![T::default(); stride * stride] }
    }

    #[inline]
    pub fn stride(&self) -> usize { self.stride }

    /// Backing index for interior coords `(x, y)` in `[-halo, w+halo)`.
    #[inline]
    fn idx(&self, x: isize, y: isize) -> usize {
        let h = self.halo as isize;
        debug_assert!(x >= -h && x < (self.w as isize + h), "x {x} out of patch");
        debug_assert!(y >= -h && y < (self.w as isize + h), "y {y} out of patch");
        ((y + h) as usize) * self.stride + (x + h) as usize
    }

    #[inline]
    pub fn get(&self, x: isize, y: isize) -> T { self.data[self.idx(x, y)] }

    #[inline]
    pub fn set(&mut self, x: isize, y: isize, v: T) {
        let i = self.idx(x, y);
        self.data[i] = v;
    }

    /// Global face-cell coords of interior cell `(x, y)`.
    #[inline]
    pub fn cell_ij(&self, x: usize, y: usize) -> (u32, u32) {
        (self.origin_i + x as u32, self.origin_j + y as u32)
    }

    /// The [`CellId`] of interior cell `(x, y)`.
    #[inline]
    pub fn cell_id(&self, x: usize, y: usize) -> CellId {
        let (i, j) = self.cell_ij(x, y);
        CellId::from_face_ij(self.face, i, j, self.level)
    }

    /// Fill interior + halo from a function of global face-cell coords. Halo cells
    /// whose global coord falls outside `[0, 2^level)` (a cube-face seam) are left
    /// at default — real cross-face halo fill is the loader's job
    /// (`ref/erosion-port/NOTES.md`).
    pub fn fill(&mut self, f: impl Fn(u32, u32) -> T) {
        let n = 1i64 << self.level;
        let h = self.halo as isize;
        for yy in -h..(self.w as isize + h) {
            for xx in -h..(self.w as isize + h) {
                let gi = self.origin_i as i64 + xx as i64;
                let gj = self.origin_j as i64 + yy as i64;
                if (0..n).contains(&gi) && (0..n).contains(&gj) {
                    let idx = self.idx(xx, yy);
                    self.data[idx] = f(gi as u32, gj as u32);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_access() {
        let mut p: Patch<f32> = Patch::new(Face::ZPos, 10, 100, 200, 8, 1);
        assert_eq!(p.stride(), 10);
        assert_eq!(p.get(0, 0), 0.0);
        p.set(3, 4, 1.5);
        assert_eq!(p.get(3, 4), 1.5);
        p.set(-1, 0, 9.0); // halo cell
        assert_eq!(p.get(-1, 0), 9.0);
    }

    #[test]
    fn cell_mapping() {
        let p: Patch<f32> = Patch::new(Face::YPos, 12, 40, 60, 4, 1);
        assert_eq!(p.cell_ij(0, 0), (40, 60));
        assert_eq!(p.cell_ij(3, 2), (43, 62));
        assert_eq!(p.cell_id(1, 1), CellId::from_face_ij(Face::YPos, 41, 61, 12));
    }

    #[test]
    fn fill_and_stencil() {
        // A linear ramp in i (incl. halo), then a 5-point average: the Laplacian of a
        // linear field is 0, so the average equals the centre — proves halo neighbours
        // feed the edge stencil correctly.
        let mut a: Patch<f32> = Patch::new(Face::XPos, 8, 10, 10, 6, 1);
        a.fill(|i, _j| i as f32);
        assert_eq!(a.get(0, 0), 10.0); // origin_i = 10
        assert_eq!(a.get(-1, 0), 9.0); // halo pulled from global i = 9

        let mut b: Patch<f32> = Patch::new(Face::XPos, 8, 10, 10, 6, 1);
        for y in 0..a.w as isize {
            for x in 0..a.w as isize {
                let avg = (a.get(x - 1, y) + a.get(x + 1, y) + a.get(x, y - 1) + a.get(x, y + 1) + a.get(x, y)) / 5.0;
                b.set(x, y, avg);
            }
        }
        assert!((b.get(2, 2) - a.get(2, 2)).abs() < 1e-4);
        assert!((b.get(5, 3) - a.get(5, 3)).abs() < 1e-4); // interior edge uses halo
    }
}
