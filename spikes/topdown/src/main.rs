//! vivarium-topdown — a character-driven pseudo-top-down explorer (Zelda-ish).
//!
//! You control a character walking the REAL voxel terrain; a PERSPECTIVE camera
//! follows from a high angle, and a coarse far-terrain backdrop carries the
//! landscape to a horizon. The bet: perspective (depth) + a character (scale) +
//! a distant vista bring back the "epic" feel the flat orthographic navigators
//! lost. Near detail is real voxels (bevy_voxel_world, hollow-shelled for speed);
//! the far backdrop is one coarse heightfield mesh, coloured by elevation.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy_voxel_world::prelude::*;

use vivarium_core::voxel::{Volume, WORLDGEN_VERSION};
use vivarium_core::World;

// --- Worldgen params (matched to the other adapters → shared cache) --------------
const SEED: u64 = 0x00C0_FFEE;
const DETAIL: i32 = 2;
const DEFAULT_REGION_HALF_M: i32 = 6_000;
const EPOCHS: u32 = 70;
const FINE_CELL_M: f32 = 8.0;
const FINE_EPOCHS: u32 = 0;
const N_AGENTS: usize = 24;

const SKY: Color = Color::srgb(0.62, 0.72, 0.86);

// --- Camera rig (high-angle perspective, Zelda-ish). Pitch/fov/dist are env-tunable
// (VIVARIUM_CAM_PITCH / _FOV / _DIST) since the right "feel" is what we're hunting.
const CAM_PITCH_DEFAULT: f32 = 1.0; // radians below horizontal (~57°)
const CAM_FOV_DEFAULT: f32 = 0.7; // ~40° vertical — a touch telephoto for an epic read
const CAM_DIST_DEFAULT: f32 = 130.0; // voxels (~65 m back) — close enough to read the character
const CAM_YAW_START: f32 = std::f32::consts::FRAC_PI_4;
const CAM_DIST_MIN: f32 = 25.0;
const CAM_DIST_MAX: f32 = 1_500.0;
const CAM_ZOOM_STEP: f32 = 1.12;

fn env_f32(key: &str, default: f32) -> f32 {
    std::env::var(key).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
}

// --- Character -------------------------------------------------------------------
const WALK_SPEED: f32 = 20.0; // voxels/sec (~10 m/s — brisk, the world is km-scale)
const RUN_MULT: f32 = 4.0;
const CHAR_HALF_H: f32 = 2.0; // half of a ~2 m capsule (4 voxels)

#[derive(Resource, Clone)]
struct VivWorld {
    volume: Arc<Volume>,
}

impl Default for VivWorld {
    fn default() -> Self {
        load_world()
    }
}

impl VoxelWorldConfig for VivWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        18
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let volume = Arc::clone(&self.volume);
        let sea = volume.sea_level();
        // Hollow shell: air below `surface - shell`, so the occluded interior is
        // never meshed (the big perf win for a surface view). Deep enough to back
        // cliff faces. VIVARIUM_SHELL tunes it.
        let shell: i32 = std::env::var("VIVARIUM_SHELL").ok().and_then(|s| s.parse().ok()).unwrap_or(96);
        Box::new(move |_chunk_pos, _lod, _prev| {
            let volume = Arc::clone(&volume);
            Box::new(move |pos: IVec3, _prev| {
                let h = volume.surface_height(pos.x, pos.z).unwrap_or(sea);
                if pos.y < h - shell {
                    return WorldVoxel::Air;
                }
                let v = volume.voxel(pos.x, pos.y, pos.z);
                if v.is_air() {
                    WorldVoxel::Air
                } else {
                    WorldVoxel::Solid(v.0 as u8)
                }
            })
        })
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            1 => [0, 0, 0],
            2 => [1, 1, 1],
            3 => [2, 2, 2],
            4 => [3, 3, 3],
            _ => [0, 0, 0],
        })
    }
}

#[derive(Resource)]
struct CamRig {
    yaw: f32,
    dist: f32,
    pitch: f32,
    fov: f32,
}

impl Default for CamRig {
    fn default() -> Self {
        Self {
            yaw: CAM_YAW_START,
            dist: env_f32("VIVARIUM_CAM_DIST", CAM_DIST_DEFAULT),
            pitch: env_f32("VIVARIUM_CAM_PITCH", CAM_PITCH_DEFAULT),
            fov: env_f32("VIVARIUM_CAM_FOV", CAM_FOV_DEFAULT),
        }
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct ExplorerCam;

fn main() {
    let world = VivWorld::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(SKY))
        .add_plugins(VoxelWorldPlugin::with_config(world.clone()))
        .insert_resource(world)
        .init_resource::<CamRig>()
        .add_systems(Startup, setup)
        .add_systems(Update, (control, maybe_screenshot))
        .run();
}

fn setup(
    mut commands: Commands,
    world: Res<VivWorld>,
    rig: Res<CamRig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Character at the origin, on the ground.
    let gx = 0;
    let gz = 0;
    let gy = world.volume.surface_height(gx, gz).unwrap_or(world.volume.sea_level()) as f32;
    let player_pos = Vec3::new(gx as f32, gy + CHAR_HALF_H, gz as f32);
    let char_mesh = meshes.add(Capsule3d::new(1.0, 2.0));
    let char_mat = materials.add(StandardMaterial { base_color: Color::srgb(0.85, 0.2, 0.2), ..default() });
    commands.spawn((Mesh3d(char_mesh), MeshMaterial3d(char_mat), Transform::from_translation(player_pos), Player));

    // Perspective camera, following from the high angle. Carries VoxelWorldCamera so
    // chunks stream around it (it sits just above/behind the character).
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection { fov: rig.fov, near: 1.0, far: 40_000.0, ..default() }),
        camera_transform(&rig, player_pos),
        ExplorerCam,
        VoxelWorldCamera::<VivWorld>::default(),
    ));

    // The epic backdrop: one coarse colored heightfield of the whole region.
    spawn_far_terrain(&mut commands, &mut meshes, &mut materials, &world.volume);

    // Sun + sky fill.
    commands.spawn((
        DirectionalLight { color: Color::srgb(1.0, 0.98, 0.92), shadows_enabled: false, illuminance: 9_000.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.5, -1.0, -0.3), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight { color: SKY, brightness: 500.0, affects_lightmapped_meshes: true });

    println!("[topdown] WASD/arrows walk (Shift run) · wheel zoom · Q/E rotate");
}

/// Camera transform from the rig + character position: look down the high angle at
/// the character from `dist` away (up and back).
fn camera_transform(rig: &CamRig, player: Vec3) -> Transform {
    let look = (Vec3::new(rig.yaw.sin(), 0.0, rig.yaw.cos()) * rig.pitch.cos() + Vec3::NEG_Y * rig.pitch.sin()).normalize();
    let target = player + Vec3::Y * 3.0; // look a little above the feet
    let eye = target - look * rig.dist;
    Transform::from_translation(eye).looking_at(target, Vec3::Y)
}

/// Input: walk the character (camera-relative), rotate/zoom the rig, follow.
fn control(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    world: Res<VivWorld>,
    mut rig: ResMut<CamRig>,
    mut pq: Query<&mut Transform, (With<Player>, Without<ExplorerCam>)>,
    mut cq: Query<&mut Transform, (With<ExplorerCam>, Without<Player>)>,
) {
    let dt = time.delta_secs();

    // Rig rotate / zoom.
    if keys.just_pressed(KeyCode::KeyQ) {
        rig.yaw -= std::f32::consts::FRAC_PI_4;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        rig.yaw += std::f32::consts::FRAC_PI_4;
    }
    for ev in wheel.read() {
        if ev.y > 0.0 {
            rig.dist = (rig.dist / CAM_ZOOM_STEP).clamp(CAM_DIST_MIN, CAM_DIST_MAX);
        } else if ev.y < 0.0 {
            rig.dist = (rig.dist * CAM_ZOOM_STEP).clamp(CAM_DIST_MIN, CAM_DIST_MAX);
        }
    }

    let Ok(mut pt) = pq.single_mut() else { return };

    // Walk, camera-relative on the ground plane.
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
        let (s, c) = rig.yaw.sin_cos();
        let fwd = Vec3::new(s, 0.0, c); // into the screen
        let right = Vec3::new(c, 0.0, -s);
        let mv = (fwd * dir.y + right * dir.x).normalize_or_zero();
        let speed = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            WALK_SPEED * RUN_MULT
        } else {
            WALK_SPEED
        };
        pt.translation += mv * speed * dt;
    }
    // Stick to the surface.
    let gh = world.volume.surface_height(pt.translation.x.round() as i32, pt.translation.z.round() as i32).unwrap_or(world.volume.sea_level());
    pt.translation.y = gh as f32 + CHAR_HALF_H;

    // Follow.
    if let Ok(mut ct) = cq.single_mut() {
        *ct = camera_transform(&rig, pt.translation);
    }
}

// --- World load (shared cache) ---------------------------------------------------

fn load_world() -> VivWorld {
    let region_half =
        std::env::var("VIVARIUM_REGION_HALF").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_REGION_HALF_M);
    let volume = if std::env::var_os("VIVARIUM_NO_CACHE").is_some() {
        generate(region_half)
    } else {
        let path = cache_path(region_half);
        match std::fs::read(&path).ok().and_then(|b| Volume::from_bytes(&b)) {
            Some(v) => {
                eprintln!("topdown: worldgen cache HIT — {}", path.display());
                v
            }
            None => {
                let v = generate(region_half);
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let tmp = path.with_extension("bin.tmp");
                if std::fs::write(&tmp, v.to_bytes()).and_then(|_| std::fs::rename(&tmp, &path)).is_ok() {
                    eprintln!("topdown: worldgen cached → {}", path.display());
                }
                v
            }
        }
    };
    VivWorld { volume: Arc::new(volume) }
}

fn generate(region_half: i32) -> Volume {
    eprintln!("topdown: generating eroded world (±{region_half} m)… slow tier, minutes.");
    World::eroded(SEED, N_AGENTS, DETAIL, region_half, EPOCHS, FINE_CELL_M, FINE_EPOCHS).volume
}

fn cache_path(region_half: i32) -> PathBuf {
    let dir = std::env::var_os("VIVARIUM_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("vivarium-worldcache"));
    dir.join(format!(
        "viv_s{SEED:x}_d{DETAIL}_r{region_half}_e{EPOCHS}_fc{FINE_CELL_M:.1}_fe{FINE_EPOCHS}_v{WORLDGEN_VERSION}.bin"
    ))
}

// --- Far-terrain backdrop (coarse colored heightfield of the whole region) --------

fn spawn_far_terrain(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, volume: &Volume) {
    const FAR_STRIDE: i32 = 32;
    const FAR_DROP: f32 = 16.0; // sink below the near voxels (z-fight guard)
    let half = DEFAULT_REGION_HALF_M * DETAIL;
    let sea = volume.sea_level();
    let n = ((2 * half) / FAR_STRIDE) as usize + 1;

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
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        metallic: 0.0,
        double_sided: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(material)));
    eprintln!("topdown: far backdrop {n}×{n} grid");
}

fn far_color(h: i32, wet: bool, sea: i32) -> [f32; 4] {
    if wet {
        return [0.16, 0.34, 0.55, 1.0];
    }
    let above_m = (h - sea).max(0) as f32 / DETAIL as f32;
    let t = (above_m / 2500.0).clamp(0.0, 1.0);
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

// --- Verification screenshot -----------------------------------------------------

fn maybe_screenshot(
    time: Res<Time>,
    mut commands: Commands,
    mut shot: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(10.0);
    if t > settle && !*shot {
        let path = PathBuf::from("/tmp/vivarium_topdown_shot.png");
        eprintln!("[topdown] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.5 {
        exit.write(AppExit::Success);
    }
}
