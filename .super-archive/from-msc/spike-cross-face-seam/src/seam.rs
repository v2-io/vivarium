//! Cross-face halo mapping — the "face axis transform" of
//! `form-cellid-chunk-patch` FE(4), derived rather than tabulated.
//!
//! The claim under test: the transform is **latent in the projection**. A halo
//! cell whose grid coord runs off face A is a real cell on some face B; its
//! identity is recovered by extrapolating A's equiangular `(u,v)` past the edge
//! (`to_unit`'s `tan` keeps going), then letting `from_unit` re-home the
//! direction onto B. No hand-written 24-edge adjacency table.
//!
//! This is the same idiom `spikes/globe::cell_value` uses for its one-cell ghost
//! ring; here we (1) generalise it to arbitrary halo depth so we can measure
//! *where* it stops being exact, and (2) return the neighbour `CellId` so a real
//! halo reads **stored evolved state**, not a re-evaluated prior.

use vivarium_world::sphere::{CellId, CubeCoord, Face};

/// Cell-centre `(u, v)` of grid cell `(i, j)` at `level`, **allowing i/j off the
/// face** (negative or ≥ 2^level). Off-face coords give `|u|`>1 or `|v|`>1 — a
/// valid gnomonic-plane point the projection extends smoothly past the edge.
#[inline]
pub fn cell_center_uv(i: i64, j: i64, level: u8) -> (f64, f64) {
    let n = (1u64 << level) as f64;
    (2.0 * (i as f64 + 0.5) / n - 1.0, 2.0 * (j as f64 + 0.5) / n - 1.0)
}

/// The real cell (on whatever face) whose centre the face-A grid cell `(i, j)`
/// re-homes to. For in-face `(i, j)` this is just `from_face_ij`; for off-face
/// it crosses the seam. **The whole transform** — no adjacency table.
pub fn rehome(face: Face, i: i64, j: i64, level: u8) -> CellId {
    let n = 1i64 << level;
    if (0..n).contains(&i) && (0..n).contains(&j) {
        return CellId::from_face_ij(face, i as u32, j as u32, level);
    }
    let (u, v) = cell_center_uv(i, j, level);
    let dir = CubeCoord { face, u, v }.to_unit();
    CubeCoord::from_unit(dir).cell(level)
}

/// `(face, i, j)` of a `CellId` at its own level (thin wrapper for probe prose).
#[inline]
pub fn face_ij(id: CellId) -> (Face, i64, i64) {
    let (f, i, j, _l) = id.to_face_ij();
    (f, i as i64, j as i64)
}

/// The **known-bad** transform: clamp an off-face coord back onto the edge
/// instead of re-homing. This is what a naive "extend the last row" fill does.
/// It must fail the involution / adjacency checks, or those checks are vacuous
/// (`#norm-probe-sensitivity` §2).
pub fn rehome_clamped(face: Face, i: i64, j: i64, level: u8) -> CellId {
    let n = (1i64 << level) - 1;
    CellId::from_face_ij(face, i.clamp(0, n) as u32, j.clamp(0, n) as u32, level)
}

/// Unit centre direction of a `CellId` — for geometry checks that must work
/// *across* faces (all faces live in the same 3-D frame).
#[inline]
pub fn center_unit(id: CellId) -> [f64; 3] {
    id.to_cube().to_unit()
}

/// The four (face, edge-name, step-outward) descriptors, so probes can sweep
/// every edge of a face. `outward (di, dj)` is the direction that leaves the
/// face; the edge runs along the perpendicular axis.
pub const EDGES: [(&str, i64, i64); 4] =
    [("east(+u)", 1, 0), ("west(-u)", -1, 0), ("north(+v)", 0, 1), ("south(-v)", 0, -1)];
