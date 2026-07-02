//! vivarium-bevy-voxel — the Bevy half of the engine spike.
//!
//! Renders the *same* deterministic `vivarium-core` world as the Godot spike,
//! but through `bevy_voxel_world`. The core is the single source of truth; this
//! is one disposable view over it. Kept deliberately close to the Godot spike's
//! choices (detail 4, the same material palette, overcast + distance fog) so the
//! two are an apples-to-apples comparison.
//!
//! This is a first cut: terrain + LOD + fog + a slowly drifting camera, with an
//! env-gated auto-screenshot for verification. First-person controls and
//! dig/place come next, mirroring the Godot build.

use std::sync::Arc;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::message::MessageWriter;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_voxel_world::custom_meshing::{CHUNK_SIZE_F, CHUNK_SIZE_U};
// padded_chunk_shape_uniform and the VoxelWorld* / LodLevel / WorldVoxel types
// all come through the prelude.
use bevy_voxel_world::prelude::*;

use vivarium_core::voxel::Volume;

/// Voxels per metre — the real-world anchor is 0.5 m voxels, so `detail = 2`
/// (vivarium-core `METERS_PER_VOXEL`). The world is now at *true geological
/// scale*: km-tall mountains, a multi-km-wide landmass draining to a real sea.
const DETAIL: i32 = 2;
const SEED: u64 = 0x00C0_FFEE;

/// Overcast sky / fog colour, matching the Godot spike.
const SKY: Color = Color::srgb(0.80, 0.82, 0.84);

/// The Bevy voxel world config: holds a shared, read-only handle to the core
/// volume and answers `bevy_voxel_world`'s per-chunk voxel queries from it.
#[derive(Resource, Clone)]
struct VivWorld {
    volume: Arc<Volume>,
}

/// Half-extent of the eroded landmass, **in metres** (≈12 km region). Far larger
/// than the view distance, so you fly within a continent, not toward its edge.
/// (Kept modest so startup stays ~2–3 s; a bigger world wants the O(n) erosion
/// stack — see `geo`.)
const EROSION_REGION_HALF_M: i32 = 6_000;
/// Erosion epochs run once at startup. Enough maturity for the dendritic
/// ridge-and-valley network to develop (MFD routing); trades a few seconds of
/// world-creation time for it (the slow abstraction tier).
const EROSION_EPOCHS: u32 = 70;

impl Default for VivWorld {
    fn default() -> Self {
        // The slow tier: shape the terrain by uplift + stream-power erosion before
        // the first frame. Deterministic from SEED. Logged because it is the one
        // visibly non-instant step at launch.
        eprintln!(
            "vivarium: generating eroded world (seed {SEED:#x}, {EROSION_EPOCHS} epochs)…"
        );
        let volume = Volume::eroded(SEED, DETAIL, EROSION_REGION_HALF_M, EROSION_EPOCHS);
        eprintln!("vivarium: world ready.");
        Self { volume: Arc::new(volume) }
    }
}

impl VoxelWorldConfig for VivWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    // Near, diggable band only — the distant terrain is drawn by the far-terrain
    // backdrop mesh (see `spawn_far_terrain`), so this can stay small and fast.
    // 16 chunks × 32 voxels × 0.5 m ≈ 256 m of full-detail voxels around the
    // camera; everything beyond is the backdrop.
    fn spawning_distance(&self) -> u32 {
        16
    }

    fn min_despawn_distance(&self) -> u32 {
        1
    }

    /// The seam to core. `bevy_voxel_world` calls this on worker threads, once
    /// per chunk; the inner closure is handed *world* voxel positions (already
    /// spaced by the chunk's LOD stride), so we just ask core for that voxel.
    /// View resolution is thus decoupled from intrinsic resolution exactly as in
    /// the Godot spike — the engine queries coarse positions far away.
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let volume = Arc::clone(&self.volume);
        Box::new(move |_chunk_pos, _lod_level, _previous| {
            let volume = Arc::clone(&volume);
            Box::new(move |pos: IVec3, _previous_voxel| {
                let v = volume.voxel(pos.x, pos.y, pos.z);
                if v.is_air() {
                    WorldVoxel::Air
                } else {
                    WorldVoxel::Solid(v.0 as u8)
                }
            })
        })
    }

    /// Map vivarium material ids to texture-array indices. (First cut uses the
    /// default texture array; flat palette colours come next, like Godot.)
    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            1 => [0, 0, 0], // stone
            2 => [1, 1, 1], // dirt
            3 => [2, 2, 2], // grass
            4 => [3, 3, 3], // water
            _ => [0, 0, 0],
        })
    }

    // --- LOD: coarser meshes with distance, the whole point of the comparison.
    fn chunk_data_shape(&self, lod_level: LodLevel) -> UVec3 {
        padded_chunk_shape_uniform(CHUNK_SIZE_U / lod_level.max(1) as u32)
    }

    fn chunk_meshing_shape(&self, lod_level: LodLevel) -> UVec3 {
        padded_chunk_shape_uniform(CHUNK_SIZE_U / lod_level.max(1) as u32)
    }

    fn chunk_lod(
        &self,
        chunk_position: IVec3,
        _previous_lod: Option<LodLevel>,
        camera_position: Vec3,
    ) -> LodLevel {
        let camera_chunk = (camera_position / CHUNK_SIZE_F).floor();
        let distance = chunk_position.as_vec3().distance(camera_chunk);
        // LOD value doubles as the sample stride. Tightened so only the nearest
        // chunks are full-res and the rest coarsen fast — far fewer fine meshes.
        if distance < 4.0 {
            1
        } else if distance < 8.0 {
            2
        } else if distance < 12.0 {
            4
        } else {
            8
        }
    }

    fn attach_chunks_to_root(&self) -> bool {
        false
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(SKY))
        .add_plugins(VoxelWorldPlugin::with_config(VivWorld::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (fly_camera, edit_voxels, log_diagnostics, maybe_screenshot))
        .run();
}

fn setup(
    mut commands: Commands,
    world: Res<VivWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Fresh diagnostics log per run.
    let _ = std::fs::write("/tmp/bevy_diag.log", "");

    // The distant terrain (v1 of the horizon-LOD plan): a single coarse mesh of
    // the whole region, behind the near voxels.
    spawn_far_terrain(&mut commands, &mut meshes, &mut materials, &world.volume);

    // Spawn near the *middle* of the region at standing height, gazing out across
    // the terrain (not down at it). No fog — we want to see as far as the renderer
    // physically reaches, hard edge and LOD seams included; that boundary is the
    // very thing we are about to fix.
    let ground = world.volume.surface_height(0, 0).unwrap_or(6_000);
    let eye = Vec3::new(0.0, ground as f32 + 12.0, 0.0); // ~6 m eye height
    // Look outward and very slightly down — across the landscape toward the horizon.
    let look = Vec3::new(800.0, ground as f32 - 60.0, 300.0);

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(eye).looking_at(look, Vec3::Y),
        VoxelWorldCamera::<VivWorld>::default(),
    ));

    // Overcast lighting: dim, shadowless key + strong flat ambient.
    commands.spawn((
        DirectionalLight { color: SKY, shadows_enabled: false, illuminance: 4000.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.3, -1.0, -0.2), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: SKY,
        brightness: 600.0,
        affects_lightmapped_meshes: true,
    });
}

/// v1 of the horizon-LOD plan (ref/rendering/NOTES.md): one coarse heightfield
/// mesh of the whole eroded region, sampled from `surface_height`, spawned as the
/// distant backdrop *behind* the near diggable voxels. This is an **in-world 3D
/// surface** — look toward a far massif and this draws it — not a top-down map.
///
/// It is a pure view over the core: every vertex is `surface_height(x, z)`, so it
/// is deterministic and stores no state. Sampled every `FAR_STRIDE` voxels (~the
/// erosion-cell scale) and sunk `FAR_DROP` voxels so the full-detail near voxels
/// reliably win the depth test wherever they exist; the distance, where no voxels
/// spawn, is carried entirely by this mesh. Water cells are clamped to sea level
/// and coloured as water so coastlines and lakes read at range.
fn spawn_far_terrain(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    volume: &Volume,
) {
    const FAR_STRIDE: i32 = 32; // voxels between samples (~16 m at detail 2)
    const FAR_DROP: f32 = 16.0; // voxels to sink below the near surface (z-fight guard)
    let half = EROSION_REGION_HALF_M * DETAIL; // region half-extent in voxels
    let sea = volume.sea_level();
    let n = ((2 * half) / FAR_STRIDE) as usize + 1;

    // Heights first — normals need neighbours. Below sea clamps to the waterline.
    let mut hs = vec![0i32; n * n];
    let mut wet = vec![false; n * n];
    for j in 0..n {
        for i in 0..n {
            let x = -half + i as i32 * FAR_STRIDE;
            let z = -half + j as i32 * FAR_STRIDE;
            let g = volume.surface_height(x, z).unwrap_or(sea);
            if g <= sea {
                hs[j * n + i] = sea;
                wet[j * n + i] = true;
            } else {
                hs[j * n + i] = g;
            }
        }
    }

    let mut positions = Vec::with_capacity(n * n);
    let mut colors = Vec::with_capacity(n * n);
    let mut normals = Vec::with_capacity(n * n);
    for j in 0..n {
        for i in 0..n {
            let x = (-half + i as i32 * FAR_STRIDE) as f32;
            let z = (-half + j as i32 * FAR_STRIDE) as f32;
            positions.push([x, hs[j * n + i] as f32 - FAR_DROP, z]);
            colors.push(far_color(hs[j * n + i], wet[j * n + i], sea));
            // Central-difference normal from the height grid (smooth hillshade).
            let l = hs[j * n + i.saturating_sub(1)];
            let r = hs[j * n + (i + 1).min(n - 1)];
            let d = hs[j.saturating_sub(1) * n + i];
            let u = hs[(j + 1).min(n - 1) * n + i];
            let dx = (r - l) as f32 / (2.0 * FAR_STRIDE as f32);
            let dz = (u - d) as f32 / (2.0 * FAR_STRIDE as f32);
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

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE, // multiplied by the per-vertex colours
        perceptual_roughness: 1.0,
        metallic: 0.0,
        double_sided: true,
        cull_mode: None, // winding-agnostic for v1; tighten later
        ..default()
    });

    commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(material)));
    eprintln!(
        "vivarium: far-terrain backdrop {n}×{n} grid ({} k tris)",
        (n - 1) * (n - 1) * 2 / 1000
    );
}

/// Elevation/material colour for a far-terrain vertex: water blue below sea, then
/// a green-lowland → brown-rock → white-peak ramp by height above sea (metres).
fn far_color(h: i32, wet: bool, sea: i32) -> [f32; 4] {
    if wet {
        return [0.16, 0.34, 0.55, 1.0];
    }
    let above_m = (h - sea).max(0) as f32 / DETAIL as f32;
    let t = (above_m / 2500.0).clamp(0.0, 1.0); // peaks reach ~2.7 km above sea
    let stops = [[0.28, 0.50, 0.24], [0.42, 0.40, 0.30], [0.95, 0.96, 0.98]];
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

/// Once a second, log frame health + total entity count to /tmp/bevy_diag.log
/// (and stdout). The entity count is the key signal: if it climbs without bound
/// as you fly, chunks/meshes are accumulating (despawn not keeping up) — which
/// would explain stutter that worsens over time. `worst_frame` captures the
/// nastiest hitch in each interval, which the smoothed FPS hides.
fn log_diagnostics(
    time: Res<Time>,
    diags: Res<DiagnosticsStore>,
    entities: Query<()>,
    mut last: Local<f32>,
    mut worst_dt: Local<f32>,
) {
    let dt = time.delta_secs();
    if dt > *worst_dt {
        *worst_dt = dt;
    }
    let t = time.elapsed_secs();
    if t - *last < 1.0 {
        return;
    }
    *last = t;
    let fps = diags
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let line = format!(
        "{:6.1}s  fps={:5.1}  worst_frame={:6.1}ms  entities={}\n",
        t,
        fps,
        *worst_dt * 1000.0,
        entities.iter().count(),
    );
    *worst_dt = 0.0;
    print!("[diag] {line}");
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/bevy_diag.log") {
        let _ = f.write_all(line.as_bytes());
    }
}

/// First-person fly controller: mouse-look + WASD (Space/Shift up-down, Ctrl
/// fast), mirroring the Godot spike. Left-click grabs the cursor; Esc frees it.
/// Disabled under autoshot so the verification camera stays put.
fn fly_camera(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    mut cursor_q: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut q: Query<&mut Transform, With<VoxelWorldCamera<VivWorld>>>,
) {
    if std::env::var("VIVARIUM_AUTOSHOT").is_ok() {
        return;
    }
    let Ok(mut cursor) = cursor_q.single_mut() else { return };
    if buttons.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }

    let Ok(mut transform) = q.single_mut() else { return };

    if cursor.grab_mode == CursorGrabMode::Locked && motion.delta != Vec2::ZERO {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= motion.delta.x * 0.003;
        pitch = (pitch - motion.delta.y * 0.003).clamp(-1.5, 1.5);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }

    let mut dir = Vec3::ZERO;
    let fwd = *transform.forward();
    let right = *transform.right();
    if keys.pressed(KeyCode::KeyW) {
        dir += fwd;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= fwd;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir -= right;
    }
    if keys.pressed(KeyCode::Space) {
        dir += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        dir -= Vec3::Y;
    }
    if dir != Vec3::ZERO {
        // Real-scale traversal: ~100 m/s normal, ~400 m/s fast (the landmass is
        // ~12 km across, so the toy-scale speeds would feel glacial). Voxels/s.
        let speed = if keys.pressed(KeyCode::ControlLeft) { 800.0 } else { 200.0 };
        transform.translation += dir.normalize() * speed * time.delta_secs();
    }
}

/// Dig (left) / place (right) the voxel the camera is looking at, via
/// bevy_voxel_world's raycast + set_voxel. The edit lands in the voxel world's
/// own store; mirroring it back into core (as the Godot bridge does) is a parity
/// item, but the *feel* is what this is for.
fn edit_voxels(
    buttons: Res<ButtonInput<MouseButton>>,
    cam_q: Query<&GlobalTransform, With<VoxelWorldCamera<VivWorld>>>,
    mut voxel_world: VoxelWorld<VivWorld>,
) {
    if std::env::var("VIVARIUM_AUTOSHOT").is_ok() {
        return;
    }
    let dig = buttons.just_pressed(MouseButton::Left);
    let place = buttons.just_pressed(MouseButton::Right);
    if !dig && !place {
        return;
    }
    let Ok(gt) = cam_q.single() else { return };
    let Ok(dir) = Dir3::new(gt.forward().into()) else { return };
    let ray = Ray3d::new(gt.translation(), dir);
    if let Some(hit) = voxel_world.raycast(ray, &|(_pos, _vox)| true) {
        if dig {
            voxel_world.set_voxel(hit.voxel_pos(), WorldVoxel::Air);
        } else if let Some(normal) = hit.normal {
            voxel_world.set_voxel(hit.voxel_pos() + normal.as_ivec3(), WorldVoxel::Solid(1));
        }
    }
}

/// Verification screenshot: when VIVARIUM_AUTOSHOT is set, wait a few seconds for
/// chunks to stream/mesh, capture to /tmp, and exit.
fn maybe_screenshot(
    mut commands: Commands,
    time: Res<Time>,
    mut taken: Local<bool>,
    mut writer: MessageWriter<AppExit>,
) {
    if std::env::var("VIVARIUM_AUTOSHOT").is_err() {
        return;
    }
    let t = time.elapsed_secs();
    // Capture once chunks have streamed/meshed...
    if !*taken && t >= 8.0 {
        *taken = true;
        let path = "/tmp/vivarium_bevy_shot.png".to_string();
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    // ...then give the async GPU readback a moment to write the file before exit.
    if t >= 10.5 {
        writer.write(AppExit::Success);
    }
}
