//! Per-cell geometric measures on the equiangular cube-sphere.
//!
//! **Claim trail:** `#obs-cube-locked-kernel-bias` (uniform `cell_m²` is a cube-locked
//! bias on drainage area A); closed form from the wavelet-store spike
//! (`msc/spike-wavelet-store/src/area.rs`). This module is the in-crate home for
//! solid angle → area so fluvial kernels stop using a face-uniform area.

use crate::sphere::Face;

/// Face-parameter (u, v) of the corner at integer grid coords `(i, j)` at `level`.
/// Cell `(i, j)` spans corners `(i,j) .. (i+1, j+1)`.
#[inline]
pub fn corner_uv(i: u64, j: u64, level: u8) -> (f64, f64) {
    corner_uv_i(i as i64, j as i64, level)
}

/// [`corner_uv`] over **signed** grid coordinates, so a cell off the low side of
/// the chart can be named at all.
///
/// A halo window on a region perimeter has origin `region_o + t·tile_n − d`,
/// which is negative for the first tile in each axis. The unsigned form cannot
/// express that, so callers clamped the origin to zero — which does not pad the
/// window, it *slides* it, and the tile is then carved from ground it does not
/// occupy ( #obs-halo-windows-overhang-the-chart-and-mint-nan FE(8)). The map is
/// affine in `i`, so nothing about negative indices is a special case here; only
/// the type was.
#[inline]
pub fn corner_uv_i(i: i64, j: i64, level: u8) -> (f64, f64) {
    let n = (1u64 << level) as f64;
    (2.0 * (i as f64) / n - 1.0, 2.0 * (j as f64) / n - 1.0)
}

/// Solid angle (steradians) of cube-sphere cell `(face, i, j)` at `level`, as the
/// spherical excess of the cell's two corner triangles. Face-independent on the
/// unit sphere (the six faces are congruent), so `face` is unused.
///
/// **Why not the closed form.** The gnomonic antiderivative
/// $\mathcal{F}(X,Y)=\arctan(XY/\sqrt{1+X^2+Y^2})$ gives the exact answer as
/// $\mathcal{F}(u_1,v_1)-\mathcal{F}(u_1,v_0)-\mathcal{F}(u_0,v_1)+\mathcal{F}(u_0,v_0)$
/// — four $O(1)$ terms whose difference is the (tiny) solid angle. As the cell
/// shrinks the terms converge on each other and the **relative** cancellation error
/// grows like $4^{\text{level}}$: measured 1.6e-6 median at L19, 4.1e-4 at L23,
/// 5.5e-3 (worst 6.8e-2) at L25 = `MAX_LEVEL`. That is a live-kernel quantity —
/// `cell_area_m2` is the per-cell runoff in `erosion::accumulate_drainage` and the
/// volume in `deposit` — and the error is spatially high-frequency, which is the
/// shape that reads as sub-grid texture rather than as a numerical floor.
///
/// **The form used instead.** Van Oosterom–Strackee,
/// $\tan(\Omega/2) = |a\cdot(b\times c)| / (1 + a\cdot b + b\cdot c + c\cdot a)$,
/// with the triple product evaluated as $a\cdot((b-a)\times(c-a))$ — algebraically
/// identical (the $a\times a$ and $a\cdot(b\times a)$ terms vanish) but formed from
/// *differences* of nearby corner vectors rather than from their raw components.
/// The denominator is $O(1)$ and well-conditioned. Measured against an independent
/// Gauss–Legendre quadrature of the Jacobian (a sum of positive terms times an
/// exactly-representable cell size, hence cancellation-free, and self-converged to
/// ~2e-16): relative residual 2e-11 at L19, 3e-10 at L23, 4e-10 at L25 — six to
/// seven orders better than the closed form, and no longer growing as $4^{level}$.
///
/// ⚠ **The NAIVE Van Oosterom–Strackee form is not the fix**: with the triple
/// product taken directly from the corner components it also degrades as
/// $4^{\text{level}}$ (1.8e-3 at L25), only ~7× better than the closed form. The
/// difference reformulation is the load-bearing part. Probe:
/// `examples/solid_angle_precision.rs` measures all three against the quadrature.
pub fn cell_solid_angle(_face: Face, i: u64, j: u64, level: u8) -> f64 {
    cell_solid_angle_i(_face, i as i64, j as i64, level)
}

/// [`cell_solid_angle`] over signed grid coordinates — see [`corner_uv_i`].
pub fn cell_solid_angle_i(_face: Face, i: i64, j: i64, level: u8) -> f64 {
    let (u0, v0) = corner_uv_i(i, j, level);
    let (u1, v1) = corner_uv_i(i + 1, j + 1, level);
    let p = |u: f64, v: f64| -> [f64; 3] {
        crate::sphere::CubeCoord { face: Face::ZPos, u, v }.to_unit()
    };
    let (a, b, c, d) = (p(u0, v0), p(u1, v0), p(u1, v1), p(u0, v1));
    tri_solid_angle(a, b, c) + tri_solid_angle(a, c, d)
}

/// Solid angle of the spherical triangle on unit vectors `a`, `b`, `c`
/// (Van Oosterom–Strackee, difference-formed triple product — see
/// [`cell_solid_angle`] for why the difference form is the load-bearing part).
#[inline]
fn tri_solid_angle(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let d1 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let d2 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let x = [
        d1[1] * d2[2] - d1[2] * d2[1],
        d1[2] * d2[0] - d1[0] * d2[2],
        d1[0] * d2[1] - d1[1] * d2[0],
    ];
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let num = dot(a, x).abs();
    let den = 1.0 + dot(a, b) + dot(b, c) + dot(c, a);
    2.0 * num.atan2(den)
}

/// Cell area in m² on a sphere of the given radius.
#[inline]
pub fn cell_area_m2(face: Face, i: u64, j: u64, level: u8, radius_m: f64) -> f64 {
    cell_solid_angle(face, i, j, level) * radius_m * radius_m
}

/// [`cell_area_m2`] over signed grid coordinates — see [`corner_uv_i`].
#[inline]
pub fn cell_area_m2_i(face: Face, i: i64, j: i64, level: u8, radius_m: f64) -> f64 {
    cell_solid_angle_i(face, i, j, level) * radius_m * radius_m
}

/// Unit direction vector at the **centre** of cell `(face, i, j)` at `level`.
/// (The equiangular `tan` map; face-relative — feeds true neighbour distances.)
#[inline]
pub fn cell_center_unit(face: Face, i: u64, j: u64, level: u8) -> [f64; 3] {
    cell_center_unit_i(face, i as i64, j as i64, level)
}

/// [`cell_center_unit`] over signed grid coordinates — see [`corner_uv_i`].
#[inline]
pub fn cell_center_unit_i(face: Face, i: i64, j: i64, level: u8) -> [f64; 3] {
    let n = (1u64 << level) as f64;
    let u = 2.0 * (i as f64 + 0.5) / n - 1.0;
    let v = 2.0 * (j as f64 + 0.5) / n - 1.0;
    crate::sphere::CubeCoord { face, u, v }.to_unit()
}

/// Great-circle distance (m) between two unit direction vectors on a sphere of
/// `radius_m`. `atan2(|a×b|, a·b)` — robust for the near-parallel vectors that
/// adjacent cell centres give (no catastrophic cancellation at fine levels).
#[inline]
pub fn gc_dist_m(a: [f64; 3], b: [f64; 3], radius_m: f64) -> f64 {
    let cx = a[1] * b[2] - a[2] * b[1];
    let cy = a[2] * b[0] - a[0] * b[2];
    let cz = a[0] * b[1] - a[1] * b[0];
    let s = (cx * cx + cy * cy + cz * cz).sqrt();
    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    s.atan2(dot) * radius_m
}

/// True great-circle distance (m) between the centres of cell `(i, j)` and its
/// neighbour at offset `(di, dj)`, same `level`. This is the honest slope/flux
/// **length** the fluvial kernels use in place of uniform `cell_m` / diagonal
/// `cell_m·√2` — a cube-locked length bias that overstates distances toward the
/// cube corners (the length sibling of uniform-area, `#obs-cube-locked-kernel-bias`).
#[inline]
pub fn neighbor_center_dist_m(face: Face, i: u64, j: u64, level: u8, di: i64, dj: i64, radius_m: f64) -> f64 {
    let a = cell_center_unit(face, i, j, level);
    let b = cell_center_unit(face, (i as i64 + di) as u64, (j as i64 + dj) as u64, level);
    gc_dist_m(a, b, radius_m)
}

/// Uniform-area stand-in: one cell size squared at this level (the **bias**
/// `#obs-cube-locked-kernel-bias` measures). Kept for probes that need the bad control.
#[inline]
pub fn uniform_cell_area_m2(level: u8, radius_m: f64) -> f64 {
    let cell_m = crate::sample::cell_size_m(level, radius_m);
    cell_m * cell_m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planet::Planet;

    /// PROBE 8 pin: uniform A overstates true A most at edge midpoints;
    /// face-centre is exact; face-mean signed error is level-fixed.
    #[test]
    fn true_area_vs_uniform_matches_probe8_shape() {
        let r = Planet::EARTH.radius_m;
        // L7: 128 cells/edge — enough resolution for centre/edge/mean.
        let level = 7u8;
        let n = 1u64 << level;
        let uniform = uniform_cell_area_m2(level, r);

        let centre = cell_area_m2(Face::ZPos, n / 2, n / 2, level, r);
        let edge_mid = cell_area_m2(Face::ZPos, n / 2, 0, level, r);
        // Face centre: true ≈ uniform (ratio ~1).
        let over_centre = uniform / centre - 1.0;
        assert!(
            over_centre.abs() < 0.02,
            "centre overstatement {over_centre:.4} should be ~0"
        );
        // Edge midpoint: PROBE 8 ~+41.2% overstatement of A by uniform.
        let over_edge = uniform / edge_mid - 1.0;
        assert!(
            (over_edge - 0.412).abs() < 0.03,
            "edge midpoint overstatement {over_edge:.4} expected ~0.412"
        );

        // Area-weighted mean overstatement over whole face ~+17.81%, level-fixed.
        let mut sum_true = 0.0;
        let mut sum_uni = 0.0;
        for j in 0..n {
            for i in 0..n {
                let a = cell_area_m2(Face::ZPos, i, j, level, r);
                sum_true += a;
                sum_uni += uniform;
            }
        }
        let mean_over = sum_uni / sum_true - 1.0;
        assert!(
            (mean_over - 0.17810).abs() < 0.005,
            "face mean overstatement {mean_over:.5} expected ~0.17810"
        );
    }

    /// Length sibling of the PROBE-8 area control: uniform `cell_m` (/ `√2` on
    /// diagonals) overstates true neighbour distance toward the cube corners, and
    /// matches it at the face centre. Uniform would make every ratio below exactly
    /// 1.0 — so this control **fails on the old uniform-length kernel** and passes
    /// only on the true spherical metric (`#norm-probe-sensitivity` known-bad-first).
    #[test]
    fn neighbor_length_vs_uniform_is_cube_locked() {
        let r = Planet::EARTH.radius_m;
        let level = 7u8; // 128 cells/edge
        let n = 1u64 << level;
        let cell_m = crate::sample::cell_size_m(level, r);
        let sqrt2 = std::f64::consts::SQRT_2;

        // Face centre: true east-neighbour distance ≈ uniform (ratio ~1).
        let ctr_east = neighbor_center_dist_m(Face::ZPos, n / 2, n / 2, level, 1, 0, r);
        assert!(
            (ctr_east / cell_m - 1.0).abs() < 1e-3,
            "centre east ratio {} should be ~1", ctr_east / cell_m
        );
        // Corner cell (0,0): east neighbour ~6.3% shorter than uniform.
        let cor_east = neighbor_center_dist_m(Face::ZPos, 0, 0, level, 1, 0, r);
        assert!(
            (cor_east / cell_m - 0.9372).abs() < 0.01,
            "corner east ratio {} expected ~0.937 (uniform would be 1.0)", cor_east / cell_m
        );
        // Corner diagonal (0,0)->(1,1): ~33% shorter than the uniform diagonal.
        let cor_diag = neighbor_center_dist_m(Face::ZPos, 0, 0, level, 1, 1, r);
        assert!(
            (cor_diag / (cell_m * sqrt2) - 0.672).abs() < 0.01,
            "corner diagonal ratio {} expected ~0.672 (uniform would be 1.0)", cor_diag / (cell_m * sqrt2)
        );
    }

    #[test]
    fn solid_angle_face_independent_and_positive() {
        let a = cell_solid_angle(Face::ZPos, 10, 20, 6);
        let b = cell_solid_angle(Face::XPos, 10, 20, 6);
        assert!((a - b).abs() < 1e-15);
        assert!(a > 0.0);
    }

    /// Children of a cell at level L tile it: sum of solid angles = parent.
    #[test]
    fn solid_angle_additivity_on_quadtree_children() {
        let face = Face::ZPos;
        let level = 5u8;
        let i = 3u64;
        let j = 7u64;
        let parent = cell_solid_angle(face, i, j, level);
        let mut kids = 0.0;
        for dj in 0..2u64 {
            for di in 0..2u64 {
                kids += cell_solid_angle(face, 2 * i + di, 2 * j + dj, level + 1);
            }
        }
        assert!(
            (parent - kids).abs() < 1e-14 * parent.max(1e-30),
            "parent {parent} vs kids {kids}"
        );
    }
}
