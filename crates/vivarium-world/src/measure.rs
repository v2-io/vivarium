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
    let n = (1u64 << level) as f64;
    (2.0 * (i as f64) / n - 1.0, 2.0 * (j as f64) / n - 1.0)
}

/// Exact solid angle (steradians) of cube-sphere cell `(face, i, j)` at `level`.
///
/// On a cube face, gnomonic tangents are $X=\tan(\pi u/4)$, $Y=\tan(\pi v/4)$,
/// $d\Omega = dX\,dY/(1+X^2+Y^2)^{3/2}$, with antiderivative
/// $\mathcal{F}(X,Y)=\arctan(XY/\sqrt{1+X^2+Y^2})$. Face-independent on the unit sphere.
pub fn cell_solid_angle(_face: Face, i: u64, j: u64, level: u8) -> f64 {
    let (u0, v0) = corner_uv(i, j, level);
    let (u1, v1) = corner_uv(i + 1, j + 1, level);
    let f = |u: f64, v: f64| -> f64 {
        let x = (u * std::f64::consts::FRAC_PI_4).tan();
        let y = (v * std::f64::consts::FRAC_PI_4).tan();
        (x * y / (1.0 + x * x + y * y).sqrt()).atan()
    };
    (f(u1, v1) - f(u1, v0) - f(u0, v1) + f(u0, v0)).abs()
}

/// Cell area in m² on a sphere of the given radius.
#[inline]
pub fn cell_area_m2(face: Face, i: u64, j: u64, level: u8, radius_m: f64) -> f64 {
    cell_solid_angle(face, i, j, level) * radius_m * radius_m
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
