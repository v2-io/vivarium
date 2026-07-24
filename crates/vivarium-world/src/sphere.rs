//! The world lives on a sphere, addressed with a **cube-sphere**: the six faces of
//! a cube, each a square `[-1, 1]²` grid, radially projected onto the sphere.
//!
//! Chosen (`doc/design/DESIGN-REDUX.md` §11; validated in operational weather models — NOAA's
//! FV3, NASA GEOS) because each face is *already a quad grid*, so a local voxel
//! patch is just a deeply-subdivided face quad: **one coordinate system serves the
//! whole planet and the playable grid**, with no hex↔square mismatch and no
//! flat-plane approximation at the seam. Everything converts to/from a unit
//! direction vector and lat/lon (below), so we are never locked into cube-sphere.
//!
//! Convention: **+Y is the north-pole axis**; longitude is measured from +X toward
//! +Z. Known deficiency: mild area/shape distortion toward the 8 cube corners —
//! far gentler than lat/lon's pole singularities, and reducible later with an
//! equiangular/conformal projection variant (a fidelity rung on the projection).
//!
//! **Columns are radial frustums, treated as prisms.** A surface cell is really a
//! square *pyramidal frustum* — it tapers toward the planet's center and widens
//! outward. We treat it as a square prism (cuboid): at Earth radius the taper
//! across a 0.5 m cell is ~1e-7, negligible. The exact solid would only matter for
//! something modeled far enough center-ward for it to bite — which nothing in the
//! ~20 km livable shell is.

use std::f64::consts::FRAC_PI_4;

/// One of the six cube faces (by the axis its outward normal points along).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face { XPos, XNeg, YPos, YNeg, ZPos, ZNeg }

/// A *direction* on the sphere as a cube-sphere coordinate: which face, and `(u, v)`
/// in `[-1, 1]` within that face. A *position* pairs this with an altitude — see
/// [`Position`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CubeCoord {
    pub face: Face,
    pub u: f64,
    pub v: f64,
}

/// Geographic latitude/longitude in **radians** — the human-facing view. Convert
/// with [`CubeCoord::to_geo`] / [`CubeCoord::from_geo`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Geo {
    pub lat: f64,
    pub lon: f64,
}

/// A position within the ~20 km "livable" shell: a surface direction plus an
/// altitude in metres relative to the sea-level datum (deepest trench ≈ −11 km to
/// highest peak ≈ +9 km; the shell, not a solid ball — the deep interior is a
/// coarse global model that fluxes up into the shell's base).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub dir: CubeCoord,
    pub altitude_m: f64,
}

impl CubeCoord {
    /// Outward unit direction vector on the unit sphere.
    ///
    /// **Equiangular** cube-sphere: `(u, v)` are interpreted as *angles* (via `tan`),
    /// so cells subtend near-equal solid angles — max/min area ratio ≈ 1.41, versus
    /// ≈ 5.14 for the naive equidistant (gnomonic) grid, for the cost of a couple of
    /// `tan`/`atan` calls. (The variant used by operational cubed-sphere models.)
    pub fn to_unit(self) -> [f64; 3] {
        let (tu, tv) = ((self.u * FRAC_PI_4).tan(), (self.v * FRAC_PI_4).tan());
        let d = match self.face {
            Face::XPos => [1.0, tv, -tu],
            Face::XNeg => [-1.0, tv, tu],
            Face::YPos => [-tu, 1.0, tv],
            Face::YNeg => [-tu, -1.0, -tv],
            Face::ZPos => [tu, tv, 1.0],
            Face::ZNeg => [-tu, tv, -1.0],
        };
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        [d[0] / n, d[1] / n, d[2] / n]
    }

    /// Inverse of [`to_unit`](Self::to_unit): pick the dominant axis (the face),
    /// recover the face ratios, and undo the equiangular `tan`. `d` need not be
    /// normalized.
    ///
    /// ⚠ **Edge-tie trap** (found by the globe view, 2026-07-10): exactly *on* a
    /// cube edge the dominant-axis comparison is a float tie, and the two faces
    /// reach it by different arithmetic paths (a literal `1.0` vs
    /// `tan(π/4) = 0.99999…`), so the winner differs per approach direction.
    /// Consumers must never sample *on* an edge — sample at **cell centers**
    /// (which are never on edges) and use ghost/halo cells across the boundary
    /// (the idiom documented in `spikes/globe`'s `cell_value`).
    pub fn from_unit(d: [f64; 3]) -> Self {
        let [x, y, z] = d;
        let (ax, ay, az) = (x.abs(), y.abs(), z.abs());
        let inv = |t: f64| t.atan() / FRAC_PI_4; // undo the equiangular mapping
        if ax >= ay && ax >= az {
            if x > 0.0 { CubeCoord { face: Face::XPos, u: inv(-z / ax), v: inv(y / ax) } }
            else { CubeCoord { face: Face::XNeg, u: inv(z / ax), v: inv(y / ax) } }
        } else if ay >= az {
            if y > 0.0 { CubeCoord { face: Face::YPos, u: inv(-x / ay), v: inv(z / ay) } }
            else { CubeCoord { face: Face::YNeg, u: inv(-x / ay), v: inv(-z / ay) } }
        } else if z > 0.0 {
            CubeCoord { face: Face::ZPos, u: inv(x / az), v: inv(y / az) }
        } else {
            CubeCoord { face: Face::ZNeg, u: inv(-x / az), v: inv(y / az) }
        }
    }

    /// Geographic lat/lon (radians), +Y as the pole axis.
    pub fn to_geo(self) -> Geo {
        let [x, y, z] = self.to_unit();
        Geo { lat: y.asin(), lon: z.atan2(x) }
    }

    pub fn from_geo(g: Geo) -> Self {
        let (clat, slat) = (g.lat.cos(), g.lat.sin());
        Self::from_unit([clat * g.lon.cos(), slat, clat * g.lon.sin()])
    }
}

// ---- CellId: the canonical spatial key (S2-style Hilbert cube-sphere cell) ----
//
// A `u64` packing (face, level, Hilbert-distance) — the *canonical* address for
// columns, memo keys, and the save store (`.super-archive/from-design/DESIGN-MATERIAL.md` §8): exact,
// drift-free, hashable, and Hilbert-ordered so a region is a contiguous id range
// (storage/streaming locality). The curve orders *chunks*; a chunk's interior is a
// plain Cartesian array, so per-cell Hilbert ops never happen in a hot loop — see
// `ref/research/spatial-key-bench.md` (Cartesian neighbours are ~80× faster).
//
// Layout, à la Google's S2:
//   bits 63..61 : face (0..5)
//   bits 60..0  : Hilbert distance (2 bits/level, MSB-aligned), then a `1` sentinel
//                 bit marking the level, then zeros.
// A plain *per-face* Hilbert curve (S2's cross-face continuity is a refinement we
// don't need for within-face chunk locality).

/// Deepest level: at Earth radius a level-25 cell is a ≤ 0.5 m footprint (the voxel
/// column); `2^25` cells per face edge.
pub const MAX_LEVEL: u8 = 25;

const FACE_SHIFT: u32 = 61;

/// The canonical spatial key — a cube-sphere cell at some level (see module notes).
/// `Ord`/`Hash` so it drops straight into memo keys, the save-store index, and
/// locality-ordered range scans.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CellId(pub u64);

impl Face {
    #[inline]
    pub fn index(self) -> u8 {
        match self { Face::XPos => 0, Face::XNeg => 1, Face::YPos => 2, Face::YNeg => 3, Face::ZPos => 4, Face::ZNeg => 5 }
    }
    #[inline]
    pub fn from_index(i: u8) -> Face {
        match i { 0 => Face::XPos, 1 => Face::XNeg, 2 => Face::YPos, 3 => Face::YNeg, 4 => Face::ZPos, _ => Face::ZNeg }
    }
}

// Canonical Hilbert curve on a 2^order grid (Wikipedia xy<->d); `n = 2^level`.
fn hilbert_rot(n: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
    if ry == 0 {
        if rx == 1 { *x = n - 1 - *x; *y = n - 1 - *y; }
        std::mem::swap(x, y);
    }
}
fn hilbert_xy2d(n: u32, mut x: u32, mut y: u32) -> u64 {
    let mut d = 0u64;
    let mut s = n / 2;
    while s > 0 {
        let rx = if (x & s) > 0 { 1 } else { 0 };
        let ry = if (y & s) > 0 { 1 } else { 0 };
        d += (s as u64) * (s as u64) * ((3 * rx) ^ ry) as u64;
        hilbert_rot(n, &mut x, &mut y, rx, ry);
        s /= 2;
    }
    d
}
fn hilbert_d2xy(n: u32, mut d: u64) -> (u32, u32) {
    let (mut x, mut y) = (0u32, 0u32);
    let mut s = 1u32;
    while s < n {
        let rx = (1 & (d / 2)) as u32;
        let ry = (1 & (d ^ rx as u64)) as u32;
        hilbert_rot(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        d /= 4;
        s *= 2;
    }
    (x, y)
}

impl CellId {
    /// The cell at integer face-cell coords `(i, j)` (each in `0..2^level`) — the
    /// form Cartesian patches address in (`.super-archive/from-design/DESIGN-MATERIAL.md` §8).
    pub fn from_face_ij(face: Face, i: u32, j: u32, level: u8) -> CellId {
        debug_assert!(level <= MAX_LEVEL);
        let l = level as u32;
        let dist = hilbert_xy2d(1u32 << level, i, j);
        let pos = (dist << (61 - 2 * l)) | (1u64 << (60 - 2 * l));
        CellId(((face.index() as u64) << FACE_SHIFT) | pos)
    }

    /// Integer face-cell coords: `(face, i, j, level)`.
    pub fn to_face_ij(self) -> (Face, u32, u32, u8) {
        let level = self.level();
        let l = level as u32;
        let dist = (self.0 >> (61 - 2 * l)) & ((1u64 << (2 * l)) - 1);
        let (i, j) = hilbert_d2xy(1u32 << level, dist);
        (self.face(), i, j, level)
    }

    /// The cell containing direction `c` at `level` (0..=[`MAX_LEVEL`]).
    pub fn from_cube(c: CubeCoord, level: u8) -> CellId {
        debug_assert!(level <= MAX_LEVEL);
        let n = 1u32 << level;
        let quant = |t: f64| -> u32 {
            (((t + 1.0) * 0.5 * n as f64).floor().max(0.0) as u32).min(n - 1)
        };
        CellId::from_face_ij(c.face, quant(c.u), quant(c.v), level)
    }

    #[inline]
    pub fn face(self) -> Face { Face::from_index((self.0 >> FACE_SHIFT) as u8) }

    /// Subdivision level, recovered from the sentinel bit.
    #[inline]
    pub fn level(self) -> u8 { ((60u32 - self.0.trailing_zeros()) / 2) as u8 }

    /// Cell-center direction on the sphere.
    pub fn to_cube(self) -> CubeCoord {
        let (face, i, j, level) = self.to_face_ij();
        let n = 1u32 << level;
        let center = |k: u32| ((k as f64 + 0.5) / n as f64) * 2.0 - 1.0;
        CubeCoord { face, u: center(i), v: center(j) }
    }

    /// The parent cell one level coarser — `None` at level 0.
    pub fn parent(self) -> Option<CellId> {
        let level = self.level();
        if level == 0 { return None; }
        let l = level as u32;
        let dist = ((self.0 >> (61 - 2 * l)) & ((1u64 << (2 * l)) - 1)) >> 2;
        let pl = l - 1;
        let pos = (dist << (61 - 2 * pl)) | (1u64 << (60 - 2 * pl));
        Some(CellId(((self.face().index() as u64) << FACE_SHIFT) | pos))
    }
}

impl CubeCoord {
    /// Convenience: the [`CellId`] containing this direction at `level`.
    pub fn cell(self, level: u8) -> CellId { CellId::from_cube(self, level) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn faces() -> [Face; 6] {
        [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZPos, Face::ZNeg]
    }

    #[test]
    fn cube_unit_roundtrips() {
        for face in faces() {
            for &u in &[-0.7, -0.2, 0.0, 0.3, 0.8] {
                for &v in &[-0.6, 0.0, 0.5, 0.9] {
                    let c = CubeCoord { face, u, v };
                    let back = CubeCoord::from_unit(c.to_unit());
                    assert_eq!(back.face, face, "face changed for {face:?} ({u},{v})");
                    assert!((back.u - u).abs() < 1e-9 && (back.v - v).abs() < 1e-9,
                        "uv drift for {face:?}: ({u},{v}) -> ({},{})", back.u, back.v);
                }
            }
        }
    }

    #[test]
    fn geo_roundtrips() {
        for &lat in &[-1.4, -0.5, 0.0, 0.5, 1.4] {
            for &lon in &[-3.0, -1.0, 0.0, 1.0, 2.5] {
                let g = Geo { lat, lon };
                let back = CubeCoord::from_geo(g).to_geo();
                assert!((back.lat - lat).abs() < 1e-9, "lat drift: {lat} -> {}", back.lat);
                assert!((back.lon - lon).abs() < 1e-9, "lon drift: {lon} -> {}", back.lon);
            }
        }
    }

    #[test]
    fn cellid_level_and_face_roundtrip() {
        for face in faces() {
            for level in [0u8, 1, 5, 13, 25] {
                let id = CellId::from_cube(CubeCoord { face, u: 0.1, v: -0.3 }, level);
                assert_eq!(id.level(), level, "level {face:?} L{level}");
                assert_eq!(id.face(), face, "face {face:?} L{level}");
            }
        }
    }

    #[test]
    fn cellid_cube_roundtrip_within_cell() {
        // from_cube -> to_cube lands within one cell half-width (2^-level in u,v).
        for face in faces() {
            for level in [3u8, 8, 16, 25] {
                let tol = 1.001 / (1u64 << level) as f64;
                for &u in &[-0.9, -0.2, 0.37, 0.85] {
                    for &v in &[-0.75, 0.05, 0.6] {
                        let back = CellId::from_cube(CubeCoord { face, u, v }, level).to_cube();
                        assert!((back.u - u).abs() <= tol && (back.v - v).abs() <= tol,
                            "L{level} {face:?} ({u},{v}) -> ({},{})", back.u, back.v);
                    }
                }
            }
        }
    }

    #[test]
    fn cellid_is_stable() {
        // Re-encoding a cell's own center reproduces the same id.
        for level in [0u8, 7, 25] {
            let id = CellId::from_cube(CubeCoord { face: Face::ZPos, u: 0.42, v: -0.17 }, level);
            assert_eq!(CellId::from_cube(id.to_cube(), level), id, "unstable at L{level}");
        }
    }

    #[test]
    fn cellid_parent() {
        let id = CellId::from_cube(CubeCoord { face: Face::YNeg, u: 0.3, v: 0.6 }, 20);
        let p = id.parent().unwrap();
        assert_eq!(p.level(), 19);
        assert_eq!(p.face(), Face::YNeg);
        // the child's centre falls inside the parent cell (± half a parent width)
        let (cc, pc) = (id.to_cube(), p.to_cube());
        let half = 1.001 / (1u64 << 19) as f64;
        assert!((cc.u - pc.u).abs() <= half && (cc.v - pc.v).abs() <= half, "child not inside parent");
        assert!(CellId::from_cube(CubeCoord { face: Face::XPos, u: 0.0, v: 0.0 }, 0).parent().is_none());
    }

    #[test]
    fn hilbert_is_contiguous() {
        // The defining Hilbert property (⇒ locality): consecutive distances are
        // Manhattan-adjacent cells. Validates the curve implementation.
        let n = 1u32 << 6;
        for d in 0..(n as u64 * n as u64 - 1) {
            let (x0, y0) = hilbert_d2xy(n, d);
            let (x1, y1) = hilbert_d2xy(n, d + 1);
            let man = (x0 as i64 - x1 as i64).abs() + (y0 as i64 - y1 as i64).abs();
            assert_eq!(man, 1, "d={d} not adjacent to d+1");
        }
    }
}
