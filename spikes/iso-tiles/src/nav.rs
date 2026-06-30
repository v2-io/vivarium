//! Navigator state + input. The camera is a focus point in world voxels plus a
//! discrete orientation, discrete zoom, and an optional Z-slice. Input runs every
//! frame and only sets a `dirty` flag; the framebuffer rebuild (frame.rs) is the
//! expensive part and happens solely when something actually changed — exactly
//! neworld's event-driven render, the reason it idled at ~0 cost.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::iso::{N_ORIENT, ZOOMS};
use crate::world::WorldSource;

#[derive(Resource)]
pub struct Navigator {
    /// Focus in world voxels (x, z). Stored in voxels so it is stable across zoom.
    pub focus_vox: Vec2,
    /// Surface height (voxels) under the focus — vertical centring of the view.
    pub focus_h_vox: f32,
    pub orientation: u8,
    pub zoom: usize,
    /// Z-slice cap in voxels: surface is drawn at `min(height, slice)`. `None` = full.
    pub slice_vox: Option<i32>,
    /// Set whenever the view changed and the framebuffer must be rebuilt. Starts
    /// true so the first frame draws. Orientation/zoom changes also flag the tile
    /// cache for a clear (`shape_changed`).
    pub dirty: bool,
    pub shape_changed: bool,
}

impl Navigator {
    pub fn new(focus_vox: Vec2, focus_h_vox: f32, zoom: usize) -> Self {
        Self {
            focus_vox,
            focus_h_vox,
            orientation: 1, // start at a 45° (dimetric) view — the angle Joseph liked
            zoom,
            slice_vox: None,
            dirty: true,
            shape_changed: true,
        }
    }
}

/// Pan speed in cells/sec (scaled by the zoom's stride so screen speed is uniform).
const PAN_CELLS_PER_SEC: f32 = 6.0;

pub fn navigator_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<MouseWheel>,
    world: Res<WorldSource>,
    mut nav: ResMut<Navigator>,
) {
    let dt = time.delta_secs();
    let stride = ZOOMS[nav.zoom].stride_vox as f32;

    // --- Pan: screen input → world, rotated by the current yaw. ----------------
    let mut input = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        input.y += 1.0; // into the screen
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        input.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        input.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        input.x -= 1.0;
    }
    if input != Vec2::ZERO {
        let yaw = crate::iso::yaw_of(nav.orientation);
        let (s, c) = yaw.sin_cos();
        // screen-right → world (c, -s); into-screen → world (s, c).
        let world_dx = input.x * c + input.y * s;
        let world_dz = -input.x * s + input.y * c;
        let step = PAN_CELLS_PER_SEC * stride * dt;
        nav.focus_vox += Vec2::new(world_dx, world_dz).normalize_or_zero() * step;
        refresh_focus_height(&world, &mut nav);
        nav.dirty = true;
    }

    // --- Zoom: wheel, discrete levels. -----------------------------------------
    for ev in wheel.read() {
        let before = nav.zoom;
        if ev.y > 0.0 && nav.zoom > 0 {
            nav.zoom -= 1;
        } else if ev.y < 0.0 && nav.zoom + 1 < ZOOMS.len() {
            nav.zoom += 1;
        }
        if nav.zoom != before {
            nav.dirty = true;
            nav.shape_changed = true;
        }
    }

    // --- Rotate: Q/E by 45°. ---------------------------------------------------
    if keys.just_pressed(KeyCode::KeyQ) {
        nav.orientation = (nav.orientation + N_ORIENT - 1) % N_ORIENT;
        nav.dirty = true;
        nav.shape_changed = true;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        nav.orientation = (nav.orientation + 1) % N_ORIENT;
        nav.dirty = true;
        nav.shape_changed = true;
    }

    // --- Z-slice: [ and ] lower/raise; \ clears. -------------------------------
    let sea = world.sea_level;
    if keys.just_pressed(KeyCode::BracketLeft) {
        let s = nav.slice_vox.unwrap_or(nav.focus_h_vox as i32 + 40);
        nav.slice_vox = Some((s - 8).max(sea - 40));
        nav.dirty = true;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        let s = nav.slice_vox.unwrap_or(nav.focus_h_vox as i32);
        nav.slice_vox = Some(s + 8);
        nav.dirty = true;
    }
    if keys.just_pressed(KeyCode::Backslash) {
        nav.slice_vox = None;
        nav.dirty = true;
    }
}

fn refresh_focus_height(world: &WorldSource, nav: &mut Navigator) {
    let h = world
        .volume
        .surface_height(nav.focus_vox.x.round() as i32, nav.focus_vox.y.round() as i32)
        .unwrap_or(world.sea_level);
    nav.focus_h_vox = h as f32;
}
