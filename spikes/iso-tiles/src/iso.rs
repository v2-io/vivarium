//! Isometric projection. The whole point of doing this with a *projection*
//! (rather than neworld's two hand-coded tile shapes) is that all 8 orientations —
//! the 90° iso views and the 45° dimetric views — fall out of the same math: the
//! tile's parallelogram shape is just the projected unit cell at `yaw = o·45°`.
//!
//! Coordinates: map cells `(cx, cz)` index a square grid at a chosen voxel
//! `stride`. World metres = cell · (stride / detail). Screen is y-down (Bevy image
//! space). A cell projects to screen via precomputed per-step basis vectors, so
//! projecting a cell is two multiply-adds — cheap, like neworld.

use bevy::math::Vec2;

/// Number of discrete orientations (45° apart).
pub const N_ORIENT: u8 = 8;

/// Depth-axis foreshortening for a 2:1 isometric (ground "into the screen" moves
/// half as fast vertically as "across").
const ISO_Y: f32 = 0.5;

/// Vertical exaggeration: screen pixels of *up* per metre of terrain height,
/// expressed as a fraction of the horizontal pixels-per-metre. 0.5 keeps cube
/// proportions honest for the 2:1 diamond; a touch more reads as nicer relief.
const HEIGHT_VE: f32 = 0.6;

/// A discrete zoom level: how many voxels one map cell spans (the LOD stride) and
/// how many screen pixels one world metre occupies across the screen.
#[derive(Clone, Copy)]
pub struct ZoomLevel {
    pub stride_vox: i32,
    pub px_per_m: f32,
}

/// The zoom ladder, tight (tactical) → wide (strategic). Coarser strides sample
/// the world less densely *and* shrink the on-screen cell, so the visible cell
/// count stays bounded while the metres-on-screen grows. Tuned for detail 2.
pub const ZOOMS: &[ZoomLevel] = &[
    ZoomLevel { stride_vox: 2, px_per_m: 22.0 }, // 1 m cells, close in
    ZoomLevel { stride_vox: 4, px_per_m: 13.0 }, // 2 m
    ZoomLevel { stride_vox: 8, px_per_m: 7.0 },  // 4 m
    ZoomLevel { stride_vox: 16, px_per_m: 3.6 }, // 8 m
    ZoomLevel { stride_vox: 32, px_per_m: 1.8 }, // 16 m
    ZoomLevel { stride_vox: 64, px_per_m: 0.9 }, // 32 m — whole-continent map
];

pub const DEFAULT_ZOOM: usize = 3; // 8 m cells — reads as terrain, not a wall of blocks

/// Yaw in radians for an orientation index.
#[inline]
pub fn yaw_of(orientation: u8) -> f32 {
    orientation as f32 * std::f32::consts::FRAC_PI_4
}

/// Precomputed projection for one (orientation, zoom, focus). Holds per-cell-step
/// screen basis vectors so `project` is a couple of FMAs.
#[derive(Clone, Copy)]
pub struct Projector {
    /// Screen delta for +1 in `cx` / `cz` (parallelogram edges).
    pub ex: Vec2,
    pub ez: Vec2,
    /// Screen delta (always straight up, y-negative) per voxel of height.
    pub hy: f32,
    origin: Vec2,
    fx: f32,
    fz: f32,
    focus_h_vox: f32,
    cos: f32,
    sin: f32,
}

impl Projector {
    pub fn new(
        orientation: u8,
        zoom: ZoomLevel,
        focus_cell: Vec2,
        focus_h_vox: f32,
        detail: i32,
        screen: Vec2,
    ) -> Self {
        let yaw = yaw_of(orientation);
        let (s, c) = yaw.sin_cos();
        let cell_m = zoom.stride_vox as f32 / detail as f32;
        let pm = zoom.px_per_m;
        // +cx step: world (+cell_m, 0) rotated by yaw, projected. Image space is
        // y-DOWN, and depth (into the screen) must read as *up* the screen, so the
        // depth-axis (ISO_Y) contribution is negated.
        let ex = Vec2::new(cell_m * c * pm, -cell_m * s * pm * ISO_Y);
        // +cz step: world (0, +cell_m) rotated by yaw, projected.
        let ez = Vec2::new(cell_m * -s * pm, -cell_m * c * pm * ISO_Y);
        // Height: up on screen (negative y). Metres = 1/detail per voxel.
        let hy = -(1.0 / detail as f32) * pm * HEIGHT_VE;
        Self {
            ex,
            ez,
            hy,
            origin: screen * 0.5,
            fx: focus_cell.x,
            fz: focus_cell.y,
            focus_h_vox,
            cos: c,
            sin: s,
        }
    }

    /// Project a cell corner/centre at a given height (voxels) to screen pixels.
    #[inline]
    pub fn project(&self, cx: f32, cz: f32, height_vox: f32) -> Vec2 {
        let a = cx - self.fx;
        let b = cz - self.fz;
        self.origin + self.ex * a + self.ez * b + Vec2::new(0.0, (height_vox - self.focus_h_vox) * self.hy)
    }

    /// Painter's-order depth: world distance "into the screen". Larger = farther
    /// back ⇒ drawn first. Height is *not* included (cliffs are resolved by the
    /// blit order within equal depth + the wall geometry).
    #[inline]
    pub fn depth(&self, cx: f32, cz: f32) -> f32 {
        let a = cx - self.fx;
        let b = cz - self.fz;
        a * self.sin + b * self.cos
    }

    /// Inverse of the ground projection (height 0): screen pixel → fractional cell.
    /// Used to find which cells fall under the screen rectangle. Solves the 2×2
    /// `[ex ez]` system on the (x, y_ground) components.
    pub fn screen_to_cell(&self, screen: Vec2) -> Vec2 {
        let p = screen - self.origin;
        let det = self.ex.x * self.ez.y - self.ez.x * self.ex.y;
        if det.abs() < 1e-6 {
            return Vec2::new(self.fx, self.fz);
        }
        let a = (p.x * self.ez.y - self.ez.x * p.y) / det;
        let b = (self.ex.x * p.y - p.x * self.ex.y) / det;
        Vec2::new(a + self.fx, b + self.fz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_inverts_screen_to_cell_on_ground() {
        let p = Projector::new(1, ZOOMS[2], Vec2::new(10.0, -3.0), 100.0, 2, Vec2::new(1280.0, 720.0));
        // A ground-height projection (height == focus height) must round-trip.
        for (cx, cz) in [(10.0, -3.0), (25.0, 14.0), (-8.0, 30.0)] {
            let s = p.project(cx, cz, 100.0);
            let back = p.screen_to_cell(s);
            assert!((back.x - cx).abs() < 1e-2, "cx {cx} -> {}", back.x);
            assert!((back.y - cz).abs() < 1e-2, "cz {cz} -> {}", back.y);
        }
    }

    #[test]
    fn depth_increases_away_from_camera() {
        // At orientation 0 (yaw 0), +cz is straight into the screen ⇒ deeper.
        let p = Projector::new(0, ZOOMS[0], Vec2::ZERO, 0.0, 2, Vec2::new(1280.0, 720.0));
        assert!(p.depth(0.0, 5.0) > p.depth(0.0, 0.0));
        assert!(p.depth(0.0, 0.0) > p.depth(0.0, -5.0));
    }
}
