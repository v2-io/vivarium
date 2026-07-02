//! vivarium-bevy-iso — an orthographic isometric navigator that renders the
//! visible *surface* of a `vivarium-core` world directly, with no voxel octree.
//! See SPEC.md for the why; the one-paragraph version:
//!
//! An ortho camera's visible set is a bounded box, not an expanding frustum — so
//! instead of streaming a sphere of voxel blocks (the bevy_voxel_world /
//! godot_voxel path), we mesh only the chunks under the screen, at a LOD chosen by
//! zoom. And — the load-bearing principle — **loading is decoupled from
//! interaction**: the camera and visible-set bookkeeping run every frame on the
//! main thread and never block; all core sampling + mesh building happens on the
//! async compute pool and is swapped in when ready. A chunk that isn't built yet
//! is simply absent, never a stutter. That decoupling is what made Godot's mature
//! LOD feel smooth even when the data was just as slow; here it is explicit.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::camera::ScalingMode;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;

use vivarium_core::voxel::{Volume, WORLDGEN_VERSION};
use vivarium_core::World;

// === Worldgen parameters — matched to the godot bridge so the cache is SHARED ===
// Identical (seed, detail, region, epochs, fine) means this app reloads the very
// same frozen world the godot view uses (and that Joseph is generating right now),
// straight from the on-disk cache — no regeneration, and a clean demonstration of
// the engine-agnostic core/view wall. VIVARIUM_REGION_HALF shrinks the world for
// fast iteration (a different, quick cache file), exactly like the bridge.
const SEED: u64 = 0x00C0_FFEE;
const DETAIL: i32 = 2; // voxels per metre (0.5 m anchor)
const DEFAULT_REGION_HALF_M: i32 = 6_000; // ~12 km
const EPOCHS: u32 = 70;
const FINE_CELL_M: f32 = 8.0;
const FINE_EPOCHS: u32 = 0;
const N_AGENTS: usize = 24;

// === Iso framing (mirrors godot navigator.gd so the two are comparable) =========
const ISO_PITCH: f32 = 0.615_479_7; // atan(1/sqrt(2)) — true isometric
const YAW_START: f32 = std::f32::consts::FRAC_PI_4;
const YAW_STEP: f32 = std::f32::consts::FRAC_PI_2;
const STANDOFF: f32 = 14_000.0; // eye stand-off in voxels; clears any peak, inside far plane
const ZOOM_MIN: f32 = 80.0;
const ZOOM_MAX: f32 = 8_000.0;
const ZOOM_START: f32 = 600.0;
const ZOOM_STEP: f32 = 1.12;
const PAN_RATE: f32 = 0.9; // fraction of the visible span per second
const ROT_LERP: f32 = 12.0;

// === Chunking ===================================================================
const CHUNK_VOX: i32 = 256; // chunk edge in voxels (~128 m at detail 2)
// Resident radius around the focus, as a multiple of zoom. The ortho screen's
// ground footprint half-diagonal is ~1.25·zoom (true-iso, 16:9); +1 chunk margin
// for pan headroom. This is THE bounded-visible-set knob — resident chunk count
// scales with zoom², not with the world size.
const VISIBLE_RADIUS_MULT: f32 = 1.35;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.80, 0.82, 0.84)))
        .init_resource::<ChunkManager>()
        .insert_resource(load_world())
        .insert_resource(Navigator::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                navigator_update,
                update_chunks,
                apply_finished_chunks,
                maybe_screenshot,
            ),
        )
        .run();
}

// --- World load (shared cache with the godot bridge) ----------------------------

#[derive(Resource, Clone)]
struct VivWorld {
    volume: Arc<Volume>,
}

fn region_half_m() -> i32 {
    std::env::var("VIVARIUM_REGION_HALF")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_REGION_HALF_M)
}

fn load_world() -> VivWorld {
    let region_half = region_half_m();
    let volume = if std::env::var_os("VIVARIUM_NO_CACHE").is_some() {
        generate_volume(region_half)
    } else {
        let path = cache_path(region_half);
        match std::fs::read(&path).ok().and_then(|b| Volume::from_bytes(&b)) {
            Some(v) => {
                eprintln!("vivarium-iso: worldgen cache HIT — reloaded {}", path.display());
                v
            }
            None => {
                let v = generate_volume(region_half);
                if let Err(e) = write_cache(&path, &v) {
                    eprintln!("vivarium-iso: could not write cache {}: {e}", path.display());
                } else {
                    eprintln!("vivarium-iso: worldgen cached → {}", path.display());
                }
                v
            }
        }
    };
    VivWorld { volume: Arc::new(volume) }
}

fn generate_volume(region_half: i32) -> Volume {
    eprintln!(
        "vivarium-iso: generating eroded world (seed {SEED:#x}, ±{region_half} m, {EPOCHS} epochs)… slow tier, minutes."
    );
    // Go through World::eroded (then take its volume) so the bytes are identical to
    // what the bridge caches — same agent-free volume, same image, shared file.
    World::eroded(SEED, N_AGENTS, DETAIL, region_half, EPOCHS, FINE_CELL_M, FINE_EPOCHS).volume
}

/// Same scheme as the godot bridge's `world_cache_path`, so the files coincide.
fn cache_path(region_half: i32) -> PathBuf {
    let dir = std::env::var_os("VIVARIUM_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("vivarium-worldcache"));
    let name = format!(
        "viv_s{SEED:x}_d{DETAIL}_r{region_half}_e{EPOCHS}_fc{FINE_CELL_M:.1}_fe{FINE_EPOCHS}_v{WORLDGEN_VERSION}.bin",
    );
    dir.join(name)
}

fn write_cache(path: &PathBuf, volume: &Volume) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("bin.tmp");
    std::fs::write(&tmp, volume.to_bytes())?;
    std::fs::rename(&tmp, path)
}

// --- Navigator state + camera ---------------------------------------------------

#[derive(Resource)]
struct Navigator {
    focus: Vec3,
    yaw: f32,
    yaw_target: f32,
    zoom: f32,
}

impl Default for Navigator {
    fn default() -> Self {
        Self { focus: Vec3::ZERO, yaw: YAW_START, yaw_target: YAW_START, zoom: ZOOM_START }
    }
}

#[derive(Component)]
struct IsoCamera;

fn setup(mut commands: Commands, world: Res<VivWorld>, mut nav: ResMut<Navigator>) {
    // Drop the focus onto the ground at the origin.
    nav.focus = ground_focus(&world.volume, Vec3::ZERO);

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: nav.zoom },
            far: STANDOFF + 32_000.0,
            near: -1.0,
            ..OrthographicProjection::default_3d()
        }),
        camera_transform(&nav),
        IsoCamera,
        // AmbientLight is a per-camera component in 0.18 (overrides GlobalAmbientLight);
        // a generous fill so block faces read under the overcast key without shadows.
        AmbientLight { brightness: 600.0, ..default() },
    ));

    // Soft overcast key light — face orientation shades the surface without hard
    // shadows, matching the godot view's mood.
    commands.spawn((
        DirectionalLight { illuminance: 9_000.0, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.7, 0.0)),
    ));
}

/// Camera transform from the navigator: look down the iso ray at the focus from
/// `STANDOFF` away. Y-up, like Godot's −Z-forward but expressed via looking_at.
fn camera_transform(nav: &Navigator) -> Transform {
    let look = (Vec3::new(nav.yaw.sin(), 0.0, nav.yaw.cos()) * ISO_PITCH.cos()
        + Vec3::NEG_Y * ISO_PITCH.sin())
    .normalize();
    let eye = nav.focus - look * STANDOFF;
    Transform::from_translation(eye).looking_at(nav.focus, Vec3::Y)
}

fn ground_focus(volume: &Volume, mut p: Vec3) -> Vec3 {
    let g = volume.surface_height(p.x.round() as i32, p.z.round() as i32).unwrap_or(0);
    p.y = g.max(volume.sea_level()) as f32;
    p
}

/// Camera + input — runs every frame, never touches the world in bulk. This is the
/// interaction half of the decoupling.
fn navigator_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    world: Res<VivWorld>,
    mut nav: ResMut<Navigator>,
    mut cam: Query<(&mut Transform, &mut Projection), With<IsoCamera>>,
) {
    let dt = time.delta_secs();

    // Keyboard pan, in the screen plane (right = camera right, up = forward-on-ground).
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if dir != Vec2::ZERO {
        let step = PAN_RATE * nav.zoom * dt;
        let d = dir.normalize() * step;
        // Right/forward on the ground from the current yaw.
        let right = Vec3::new(nav.yaw.cos(), 0.0, -nav.yaw.sin());
        let fwd = Vec3::new(nav.yaw.sin(), 0.0, nav.yaw.cos());
        nav.focus += right * d.x + fwd * d.y;
    }

    // Wheel zoom.
    for ev in wheel.read() {
        if ev.y > 0.0 {
            nav.zoom = (nav.zoom / ZOOM_STEP).clamp(ZOOM_MIN, ZOOM_MAX);
        } else if ev.y < 0.0 {
            nav.zoom = (nav.zoom * ZOOM_STEP).clamp(ZOOM_MIN, ZOOM_MAX);
        }
    }

    // Q/E rotate by a quarter turn, eased.
    if keys.just_pressed(KeyCode::KeyQ) {
        nav.yaw_target -= YAW_STEP;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        nav.yaw_target += YAW_STEP;
    }
    if (nav.yaw - nav.yaw_target).abs() > 1e-4 {
        let t = (ROT_LERP * dt).clamp(0.0, 1.0);
        nav.yaw = lerp_angle(nav.yaw, nav.yaw_target, t);
    }

    // Glue the focus to the ground.
    nav.focus = ground_focus(&world.volume, nav.focus);

    // Apply to the camera.
    if let Ok((mut tf, mut proj)) = cam.single_mut() {
        *tf = camera_transform(&nav);
        if let Projection::Orthographic(o) = proj.as_mut() {
            o.scaling_mode = ScalingMode::FixedVertical { viewport_height: nav.zoom };
        }
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut d = (b - a).rem_euclid(std::f32::consts::TAU);
    if d > std::f32::consts::PI {
        d -= std::f32::consts::TAU;
    }
    a + d * t
}

// --- Chunk streaming (the decoupled loading half) -------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkKey {
    cx: i32,
    cz: i32,
}

struct ChunkSlot {
    lod: u8,
    entity: Option<Entity>,
    task: Option<Task<MeshData>>,
}

#[derive(Resource, Default)]
struct ChunkManager {
    slots: HashMap<ChunkKey, ChunkSlot>,
}

/// Raw mesh arrays — `Send`, built off-thread, assembled into a `Mesh` on the main
/// thread (asset insertion must happen there).
struct MeshData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

/// LOD (sample stride exponent) from zoom: zoomed in → fine, out → coarse. Tuned so
/// a chunk stays a few thousand verts at any zoom.
fn lod_for_zoom(zoom: f32) -> u8 {
    // stride ≈ zoom / 128, snapped to a power of two, in [1, 32] → exponent [0, 5].
    let raw = (zoom / 128.0).max(1.0);
    (raw.log2().floor() as i32).clamp(0, 5) as u8
}

/// Decide the visible chunk set + LOD from the camera, spawn build tasks for what's
/// missing/stale, reap what's left the view. Runs every frame; spawns work but never
/// waits on it.
fn update_chunks(
    world: Res<VivWorld>,
    nav: Res<Navigator>,
    mut mgr: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let lod = lod_for_zoom(nav.zoom);
    let radius = nav.zoom * VISIBLE_RADIUS_MULT + CHUNK_VOX as f32;
    let cf = CHUNK_VOX as f32;
    let cx0 = ((nav.focus.x - radius) / cf).floor() as i32;
    let cx1 = ((nav.focus.x + radius) / cf).ceil() as i32;
    let cz0 = ((nav.focus.z - radius) / cf).floor() as i32;
    let cz1 = ((nav.focus.z + radius) / cf).ceil() as i32;

    let pool = AsyncComputeTaskPool::get();
    let mut wanted: HashMap<ChunkKey, ()> = HashMap::new();

    for cz in cz0..=cz1 {
        for cx in cx0..=cx1 {
            let key = ChunkKey { cx, cz };
            wanted.insert(key, ());
            let needs_build = match mgr.slots.get(&key) {
                // Already at the right LOD (or building it): nothing to do.
                Some(slot) => slot.lod != lod && slot.task.is_none(),
                None => true,
            };
            if needs_build {
                // Keep the existing (coarser) entity visible until the finer mesh
                // lands — apply_finished_chunks swaps it then. Read it before the
                // insert borrows the map.
                let prev_entity = mgr.slots.get(&key).and_then(|s| s.entity);
                let vol = world.volume.clone();
                let task = pool.spawn(async move { build_chunk_mesh(&vol, cx, cz, lod) });
                mgr.slots.insert(key, ChunkSlot { lod, entity: prev_entity, task: Some(task) });
            }
        }
    }

    // Reap chunks no longer in view (drop their entity and any pending task).
    mgr.slots.retain(|key, slot| {
        if wanted.contains_key(key) {
            true
        } else {
            if let Some(e) = slot.entity.take() {
                commands.entity(e).despawn();
            }
            false
        }
    });
}

/// Swap in any meshes that finished this frame. Main thread, but only touches
/// already-built data — no sampling here.
fn apply_finished_chunks(
    mut mgr: ResMut<ChunkManager>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let keys: Vec<ChunkKey> = mgr.slots.keys().copied().collect();
    for key in keys {
        let Some(slot) = mgr.slots.get_mut(&key) else { continue };
        let Some(task) = slot.task.as_mut() else { continue };
        let Some(data) = block_on(future::poll_once(task)) else { continue };
        slot.task = None;

        // Replace any previous (coarser) entity for this chunk.
        if let Some(old) = slot.entity.take() {
            commands.entity(old).despawn();
        }
        if data.indices.is_empty() {
            continue; // fully-empty chunk (e.g. open ocean) — nothing to draw
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, data.colors);
        mesh.insert_indices(Indices::U32(data.indices));
        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            metallic: 0.0,
            ..default()
        });
        let e = commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(material))).id();
        slot.entity = Some(e);
    }
}

/// Build one chunk's surface mesh at a sample stride of `2^lod` voxels. Heightmap
/// grid with central-difference normals and material-ramp vertex colours — the
/// proven bevy-voxel `spawn_far_terrain` pattern, scoped to a chunk. Pure read of
/// `Volume`, so it is safe to run on any number of worker threads at once.
fn build_chunk_mesh(volume: &Volume, cx: i32, cz: i32, lod: u8) -> MeshData {
    let stride = 1i32 << lod;
    let x0 = cx * CHUNK_VOX;
    let z0 = cz * CHUNK_VOX;
    let n = (CHUNK_VOX / stride) as usize + 1; // nodes per side (shares an edge with the next chunk)
    let sea = volume.sea_level();

    // Heights first (normals need neighbours). Below sea → clamp to the waterline.
    let mut hs = vec![0i32; n * n];
    let mut wet = vec![false; n * n];
    let mut any_land = false;
    for j in 0..n {
        for i in 0..n {
            let x = x0 + i as i32 * stride;
            let z = z0 + j as i32 * stride;
            let g = volume.surface_height(x, z).unwrap_or(sea);
            if g <= sea {
                hs[j * n + i] = sea;
                wet[j * n + i] = true;
            } else {
                hs[j * n + i] = g;
                any_land = true;
            }
        }
    }
    // All-ocean chunk: still draw a flat water quad so the sea reads, but skip the
    // per-vertex normal/colour work by letting the generic path handle it. (Kept
    // simple: build it like any other; the cost is trivial for a flat grid.)
    let _ = any_land;

    let mut positions = Vec::with_capacity(n * n);
    let mut normals = Vec::with_capacity(n * n);
    let mut colors = Vec::with_capacity(n * n);
    for j in 0..n {
        for i in 0..n {
            let x = (x0 + i as i32 * stride) as f32;
            let z = (z0 + j as i32 * stride) as f32;
            let h = hs[j * n + i];
            positions.push([x, h as f32, z]);
            colors.push(surface_color(h, wet[j * n + i], sea));
            let l = hs[j * n + i.saturating_sub(1)];
            let r = hs[j * n + (i + 1).min(n - 1)];
            let d = hs[j.saturating_sub(1) * n + i];
            let u = hs[(j + 1).min(n - 1) * n + i];
            let dx = (r - l) as f32 / (2.0 * stride as f32);
            let dz = (u - d) as f32 / (2.0 * stride as f32);
            let nrm = Vec3::new(-dx, 1.0, -dz).normalize();
            normals.push([nrm.x, nrm.y, nrm.z]);
        }
    }

    let mut indices = Vec::with_capacity((n - 1) * (n - 1) * 6);
    for j in 0..n - 1 {
        for i in 0..n - 1 {
            let a = (j * n + i) as u32;
            let b = a + 1;
            let c = ((j + 1) * n + i) as u32;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    MeshData { positions, normals, colors, indices }
}

/// Elevation/water colour ramp — green lowlands → brown mid → white peaks, flat
/// blue for water. Same spirit as the bevy-voxel far-terrain colours.
fn surface_color(h: i32, wet: bool, sea: i32) -> [f32; 4] {
    if wet {
        return [0.33, 0.45, 0.62, 1.0];
    }
    let above_m = (h - sea).max(0) as f32 / DETAIL as f32;
    let t = (above_m / 2500.0).clamp(0.0, 1.0);
    let stops = [[0.30, 0.46, 0.28], [0.45, 0.40, 0.31], [0.92, 0.94, 0.96]];
    let seg = (t * 2.0).min(1.999);
    let k = seg.floor() as usize;
    let f = seg - k as f32;
    [
        stops[k][0] + (stops[k + 1][0] - stops[k][0]) * f,
        stops[k][1] + (stops[k + 1][1] - stops[k][1]) * f,
        stops[k][2] + (stops[k + 1][2] - stops[k][2]) * f,
        1.0,
    ]
}

// --- Verification screenshot ----------------------------------------------------

/// When VIVARIUM_AUTOSHOT is set, wait a few seconds for chunks to build, frame a
/// wide vista, screenshot, and quit — for headless verification.
fn maybe_screenshot(
    time: Res<Time>,
    mut commands: Commands,
    mut nav: ResMut<Navigator>,
    mut shot_taken: Local<bool>,
    mut framed: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    if !*framed && t > 0.5 {
        // Wide vista to frame the whole small test region.
        nav.zoom = std::env::var("VIVARIUM_ISO_ZOOM")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5000.0);
        *framed = true;
    }
    let settle: f32 = std::env::var("VIVARIUM_SETTLE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8.0);
    if t > settle && !*shot_taken {
        let path = PathBuf::from("/tmp/vivarium_iso_shot.png");
        eprintln!("vivarium-iso: SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot_taken = true;
    }
    // Give the screenshot a beat to flush, then quit.
    if *shot_taken && t > settle + 1.5 {
        exit.write(AppExit::Success);
    }
}
