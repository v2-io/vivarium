//! vivarium-iso-voxels — a deliberately-simple orthographic isometric view over the
//! REAL voxel world. No custom LOD, no memoization, no hand-rolled rasteriser: an
//! ortho camera at an iso angle, `bevy_voxel_world` draws actual cubes from core,
//! and the GPU depth buffer handles occlusion correctly (so the pawn is never
//! clipped by lower-but-nearer terrain — the failure of the previous tile spike).
//!
//! Streaming is decoupled from the render camera: an invisible `VoxelWorldCamera`
//! entity rides the ground *focus* so chunks stream around what you're looking at,
//! while the ortho camera floats back at the iso angle.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy_voxel_world::prelude::*;

use vivarium_core::voxel::{Volume, WORLDGEN_VERSION};
use vivarium_core::World;

// --- Worldgen params (matched to the godot/bevy adapters → shared cache) ---------
const SEED: u64 = 0x00C0_FFEE;
const DETAIL: i32 = 2;
const DEFAULT_REGION_HALF_M: i32 = 6_000;
const EPOCHS: u32 = 70;
const FINE_CELL_M: f32 = 8.0;
const FINE_EPOCHS: u32 = 0;
const N_AGENTS: usize = 24;

const SKY: Color = Color::srgb(0.80, 0.82, 0.84);

// --- Iso framing -----------------------------------------------------------------
const ISO_PITCH: f32 = 0.615_479_7; // atan(1/sqrt(2)) — true isometric
const YAW_START: f32 = std::f32::consts::FRAC_PI_4;
const YAW_STEP: f32 = std::f32::consts::FRAC_PI_2;
/// Eye stand-off from the focus, in voxels. bevy_voxel_world streams around the
/// camera, so this can't be huge (the focus must stay inside the spawn sphere);
/// kept moderate. Ortho, so it does not affect apparent size — only streaming
/// centre and what foreground terrain occludes.
const STANDOFF: f32 = 700.0;
/// Ortho viewport height in voxels (the zoom). Smaller = closer.
const ZOOM_MIN: f32 = 60.0;
const ZOOM_MAX: f32 = 3_000.0;
const ZOOM_START: f32 = 320.0;
const ZOOM_STEP: f32 = 1.15;
const PAN_RATE: f32 = 0.9;
const ROT_LERP: f32 = 12.0;
/// How far (as a fraction of the zoom) the camera aims *ahead* of the focus, so the
/// focus/pawn sits in the lower third of the screen rather than dead centre.
const AIM_AHEAD_FRAC: f32 = 0.4;

#[derive(Resource, Clone)]
struct VivWorld {
    volume: Arc<Volume>,
}

// `VoxelWorldConfig` requires `Default`; ours loads the world through the shared
// cache (instant when warm). Called once in `main`, then cloned to plugin + resource.
impl Default for VivWorld {
    fn default() -> Self {
        load_world()
    }
}

impl VoxelWorldConfig for VivWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    /// Stream a generous bubble — the ortho eye floats back at the iso angle, so the
    /// sphere must reach past the eye to the focus and cover the visible footprint.
    /// NB: this is a sphere with no frustum culling, so cost grows with its VOLUME —
    /// raising it is the dominant perf knob (≈8× the chunks per doubling).
    fn spawning_distance(&self) -> u32 {
        48
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let volume = Arc::clone(&self.volume);
        let sea = volume.sea_level();
        // Hollow-shell depth (voxels). The deep interior of every hill is permanently
        // occluded in a surface view, so we return AIR below `surface - shell` — the
        // buried mass is never meshed, which is the bulk of the saving. `shell` must
        // be deep enough to back cliff faces (a column must stay solid down to its
        // lower neighbours) or you'd see air behind a cliff; 96 voxels = 48 m backs
        // typical relief. VIVARIUM_SHELL tunes it.
        let shell: i32 = std::env::var("VIVARIUM_SHELL").ok().and_then(|s| s.parse().ok()).unwrap_or(96);
        Box::new(move |_chunk_pos, _lod, _prev| {
            let volume = Arc::clone(&volume);
            Box::new(move |pos: IVec3, _prev| {
                // Below the shell → air (occluded interior, never meshed).
                let h = volume.surface_height(pos.x, pos.z).unwrap_or(sea);
                if pos.y < h - shell {
                    return WorldVoxel::Air;
                }
                let v = volume.voxel(pos.x, pos.y, pos.z);
                // Water (material 4) is rendered as a separate TRANSLUCENT surface
                // mesh (see spawn_water_mesh), so the opaque terrain mesher treats it
                // as air — this exposes the solid bed beneath, which then shows
                // through the translucent water (Minecraft-style). bevy_voxel_world's
                // WorldVoxel has no translucent variant, hence the two-layer split.
                if v.is_air() || v.0 == 4 {
                    WorldVoxel::Air
                } else {
                    WorldVoxel::Solid(v.0 as u8)
                }
            })
        })
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            1 => [0, 0, 0], // stone
            2 => [1, 1, 1], // dirt
            3 => [2, 2, 2], // grass
            4 => [3, 3, 3], // water
            _ => [0, 0, 0],
        })
    }
}

fn main() {
    let world = VivWorld::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(SKY))
        .add_plugins(VoxelWorldPlugin::with_config(world.clone()))
        .insert_resource(world)
        .insert_resource(Navigator::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (navigator_update, update_water, maybe_screenshot))
        .run();
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
                eprintln!("iso-voxels: worldgen cache HIT — {}", path.display());
                v
            }
            None => {
                let v = generate(region_half);
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let tmp = path.with_extension("bin.tmp");
                if std::fs::write(&tmp, v.to_bytes()).and_then(|_| std::fs::rename(&tmp, &path)).is_ok() {
                    eprintln!("iso-voxels: worldgen cached → {}", path.display());
                }
                v
            }
        }
    };
    VivWorld { volume: Arc::new(volume) }
}

fn generate(region_half: i32) -> Volume {
    eprintln!("iso-voxels: generating eroded world (±{region_half} m)… slow tier, minutes.");
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

// --- Navigator + camera ----------------------------------------------------------

#[derive(Resource)]
struct Navigator {
    /// Focus in world voxels (x, z).
    focus: Vec2,
    focus_h: f32,
    yaw: f32,
    yaw_target: f32,
    zoom: f32,
    /// Depth-fog dials (read once from env; live-tunable at launch).
    fog_on: bool,
    fog_start: f32, // clear margin past the pawn, as a fraction of zoom
    fog_span: f32,  // additional distance to full haze, as a fraction of zoom
}

impl Default for Navigator {
    fn default() -> Self {
        Self {
            focus: Vec2::ZERO,
            focus_h: 0.0,
            yaw: YAW_START,
            yaw_target: YAW_START,
            zoom: ZOOM_START,
            fog_on: std::env::var("VIVARIUM_FOG").map(|v| v != "0").unwrap_or(true),
            fog_start: env_f32("VIVARIUM_FOG_START", 0.3),
            fog_span: env_f32("VIVARIUM_FOG_SPAN", 3.0),
        }
    }
}

fn env_f32(key: &str, default: f32) -> f32 {
    std::env::var(key).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
}

/// The ortho rendering camera (floats at the iso angle).
#[derive(Component)]
struct IsoCamera;
/// The pawn marker — a real cube on the surface at the focus.
#[derive(Component)]
struct Pawn;

fn setup(
    mut commands: Commands,
    world: Res<VivWorld>,
    mut nav: ResMut<Navigator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    nav.focus_h = world.volume.surface_height(0, 0).unwrap_or(world.volume.sea_level()) as f32;

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: nav.zoom },
            far: STANDOFF + 40_000.0,
            near: -1.0,
            ..OrthographicProjection::default_3d()
        }),
        camera_transform(&nav),
        IsoCamera,
        // bevy_voxel_world streams chunks around this marker (the camera).
        VoxelWorldCamera::<VivWorld>::default(),
        // Depth fog: terrain receding up-screen (away from the pawn) hazes toward
        // the sky. The band is set each frame to bracket the pawn's distance
        // (navigator_update), so the pawn plane is clear and only the distance
        // fuzzes. Placeholder range until the first frame.
        DistanceFog {
            color: SKY,
            falloff: FogFalloff::Linear { start: STANDOFF, end: STANDOFF + 400.0 },
            ..default()
        },
    ));

    // The pawn: a ~2 m red cube standing on the surface (4 voxels at detail 2).
    let pawn_mesh = meshes.add(Cuboid::new(2.0, 4.0, 2.0));
    let pawn_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.18, 0.18),
        perceptual_roughness: 0.9,
        ..default()
    });
    commands.spawn((Mesh3d(pawn_mesh), MeshMaterial3d(pawn_mat), pawn_transform(&nav), Pawn));

    // Overcast lighting (matches the other views).
    commands.spawn((
        DirectionalLight { color: SKY, shadows_enabled: false, illuminance: 4500.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.4, -1.0, -0.25), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight { color: SKY, brightness: 650.0, affects_lightmapped_meshes: true });

    // Water is a focus-following translucent mesh, (re)built by update_water. One
    // shared material; per-vertex colour carries blue→foam + alpha.
    let water_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.25,
        reflectance: 0.3,
        double_sided: true,
        cull_mode: None,
        ..default()
    });
    commands.insert_resource(WaterState {
        entity: None,
        center: Vec2::splat(1.0e9), // force a build on frame one
        zoom: 0.0,
        material: water_mat,
        foam_speed: env_f32("VIVARIUM_WATER_FOAM", 4.0),
        min_flow: env_f32("VIVARIUM_WATER_MIN_FLOW", 0.12),
    });

    println!("[iso-voxels] WASD/arrows pan · wheel zoom · Q/E rotate 45°");
}

#[derive(Component)]
struct Water;

#[derive(Resource)]
struct WaterState {
    entity: Option<Entity>,
    center: Vec2,
    zoom: f32,
    material: Handle<StandardMaterial>,
    foam_speed: f32,
    min_flow: f32,
}

/// (Re)build the water mesh around the focus when it moves far enough or zoom
/// changes — so water exists wherever the pawn goes, sized to the view (not a fixed
/// patch around the origin). Synchronous, but bounded to the visible area so the
/// rebuild is cheap and only fires on significant movement.
fn update_water(
    mut commands: Commands,
    world: Res<VivWorld>,
    nav: Res<Navigator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ws: ResMut<WaterState>,
) {
    let radius = (nav.zoom * 1.6 + 250.0).min(2200.0);
    let moved = ws.center.distance(nav.focus);
    let zoom_changed = (ws.zoom - nav.zoom).abs() > 1.0;
    if ws.entity.is_some() && moved < radius * 0.3 && !zoom_changed {
        return;
    }
    if let Some(e) = ws.entity.take() {
        commands.entity(e).despawn();
    }
    if let Some(mesh) = build_water_mesh(&world.volume, nav.focus, radius, ws.foam_speed, ws.min_flow) {
        let h = meshes.add(mesh);
        ws.entity = Some(commands.spawn((Mesh3d(h), MeshMaterial3d(ws.material.clone()), Water)).id());
    }
    ws.center = nav.focus;
    ws.zoom = nav.zoom;
}

/// Per-cell water stride in voxels (~3 m). Fine enough for one-cell streams.
const WATER_STRIDE: i32 = 6;

/// Build a water surface mesh over a square of `2·radius` voxels centred on
/// `center`. Standing water sits at its waterline; thin fast films sit on the bed.
/// Crucially it also emits **vertical quads** down every drop between a water cell
/// and a lower neighbour (wet OR dry bed), so falls/steps/banks render — that is
/// the "falling water on vertical faces" a flat surface can't show. Colour is per
/// vertex: calm deep = translucent blue → fast = (mostly) opaque white foam.
fn build_water_mesh(volume: &Volume, center: Vec2, radius: f32, foam_speed: f32, min_flow: f32) -> Option<Mesh> {
    let sea = volume.sea_level();
    let s = WATER_STRIDE;
    let half = s as f32 * 0.5;
    // Snap the grid origin to the stride so panning doesn't shimmer the cells.
    let gx0 = (((center.x - radius) / s as f32).floor() as i32) * s;
    let gz0 = (((center.y - radius) / s as f32).floor() as i32) * s;
    let n = ((2.0 * radius) as i32 / s) as usize + 2;

    // Sample the grid once: wet mask, surface height (water surface, or bed if dry —
    // dry is the fall target), and flow speed.
    let mut wet = vec![false; n * n];
    let mut ys = vec![0.0f32; n * n];
    let mut spd = vec![0.0f32; n * n];
    for j in 0..n {
        for i in 0..n {
            let x = gx0 + i as i32 * s;
            let z = gz0 + j as i32 * s;
            let bed = volume.surface_height(x, z).unwrap_or(sea);
            let depth = volume.water_depth_voxels(x, z);
            let speed = volume.water_speed(x, z);
            spd[j * n + i] = speed;
            if depth > 0 || speed > min_flow {
                wet[j * n + i] = true;
                ys[j * n + i] = if depth > 0 { (bed + depth) as f32 } else { bed as f32 + 1.0 };
            } else {
                ys[j * n + i] = bed as f32; // fall target for a wet higher neighbour
            }
        }
    }

    let color_for = |speed: f32| -> [f32; 4] {
        // Gentle curve so rivers stay blue; only genuinely fast water foams. Foam is
        // *mostly* opaque (0.85), not fully, so it still reads as water.
        let t = (speed / foam_speed).clamp(0.0, 1.0).powf(1.6);
        let calm = [0.20f32, 0.44, 0.70, 0.5];
        let foam = [0.93f32, 0.96, 1.0, 0.85];
        [
            calm[0] + (foam[0] - calm[0]) * t,
            calm[1] + (foam[1] - calm[1]) * t,
            calm[2] + (foam[2] - calm[2]) * t,
            calm[3] + (foam[3] - calm[3]) * t,
        ]
    };

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut quad = |p: [[f32; 3]; 4], nrm: [f32; 3], col: [f32; 4], out_p: &mut Vec<[f32; 3]>, out_n: &mut Vec<[f32; 3]>, out_c: &mut Vec<[f32; 4]>, out_i: &mut Vec<u32>| {
        let b = out_p.len() as u32;
        out_p.extend_from_slice(&p);
        out_n.extend_from_slice(&[nrm; 4]);
        out_c.extend_from_slice(&[col; 4]);
        out_i.extend_from_slice(&[b, b + 2, b + 1, b + 1, b + 2, b + 3]);
    };

    for j in 0..n {
        for i in 0..n {
            if !wet[j * n + i] {
                continue;
            }
            let x = (gx0 + i as i32 * s) as f32;
            let z = (gz0 + j as i32 * s) as f32;
            let y0 = ys[j * n + i];
            let col = color_for(spd[j * n + i]);

            // Horizontal surface quad.
            quad(
                [[x - half, y0, z - half], [x + half, y0, z - half], [x - half, y0, z + half], [x + half, y0, z + half]],
                [0.0, 1.0, 0.0],
                col,
                &mut positions, &mut normals, &mut colors, &mut indices,
            );

            // Vertical fall faces: for each lower neighbour, a quad down the shared edge.
            let neigh: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (di, dj) in neigh {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || nj < 0 || ni >= n as i32 || nj >= n as i32 {
                    continue;
                }
                let ns = ys[nj as usize * n + ni as usize];
                if y0 - ns <= 1.0 {
                    continue; // neighbour not meaningfully lower
                }
                // Falling water is fast → bias the colour toward foam.
                let fcol = color_for(spd[j * n + i].max(spd[nj as usize * n + ni as usize]).max(foam_speed * 0.5));
                let (q, nrm) = match (di, dj) {
                    (1, 0) => ([[x + half, y0, z - half], [x + half, y0, z + half], [x + half, ns, z - half], [x + half, ns, z + half]], [1.0, 0.0, 0.0]),
                    (-1, 0) => ([[x - half, y0, z - half], [x - half, y0, z + half], [x - half, ns, z - half], [x - half, ns, z + half]], [-1.0, 0.0, 0.0]),
                    (0, 1) => ([[x - half, y0, z + half], [x + half, y0, z + half], [x - half, ns, z + half], [x + half, ns, z + half]], [0.0, 0.0, 1.0]),
                    _ => ([[x - half, y0, z - half], [x + half, y0, z - half], [x - half, ns, z - half], [x + half, ns, z - half]], [0.0, 0.0, -1.0]),
                };
                quad(q, nrm, fcol, &mut positions, &mut normals, &mut colors, &mut indices);
            }
        }
    }
    if indices.is_empty() {
        return None;
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

/// World focus as a 3D point (voxels = bevy units; bevy_voxel_world is 1 voxel/unit).
fn focus3(nav: &Navigator) -> Vec3 {
    Vec3::new(nav.focus.x, nav.focus_h, nav.focus.y)
}

fn camera_transform(nav: &Navigator) -> Transform {
    let look = (Vec3::new(nav.yaw.sin(), 0.0, nav.yaw.cos()) * ISO_PITCH.cos() + Vec3::NEG_Y * ISO_PITCH.sin()).normalize();
    // Aim ahead of the focus (into the screen) so the focus sits in the lower third.
    let forward_h = Vec3::new(nav.yaw.sin(), 0.0, nav.yaw.cos());
    let aim = focus3(nav) + forward_h * (nav.zoom * AIM_AHEAD_FRAC);
    let eye = aim - look * STANDOFF;
    Transform::from_translation(eye).looking_at(aim, Vec3::Y)
}

fn pawn_transform(nav: &Navigator) -> Transform {
    // Cube origin is its centre; lift by half its 4-voxel height to sit on the ground.
    Transform::from_translation(focus3(nav) + Vec3::Y * 2.0)
}

fn navigator_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    world: Res<VivWorld>,
    mut nav: ResMut<Navigator>,
    mut cam: Query<(&mut Transform, &mut Projection, &mut DistanceFog), (With<IsoCamera>, Without<Pawn>)>,
    mut pawn: Query<&mut Transform, (With<Pawn>, Without<IsoCamera>)>,
) {
    let dt = time.delta_secs();

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { dir.x += 1.0; }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) { dir.x -= 1.0; }
    if dir != Vec2::ZERO {
        let (s, c) = nav.yaw.sin_cos();
        // A/D (dir.x) is screen-left/right; its world axis was inverted — negate it.
        let world_dx = -dir.x * c + dir.y * s;
        let world_dz = dir.x * s + dir.y * c;
        let step = PAN_RATE * nav.zoom * dt;
        nav.focus += Vec2::new(world_dx, world_dz).normalize_or_zero() * step;
    }

    for ev in wheel.read() {
        if ev.y > 0.0 {
            nav.zoom = (nav.zoom / ZOOM_STEP).clamp(ZOOM_MIN, ZOOM_MAX);
        } else if ev.y < 0.0 {
            nav.zoom = (nav.zoom * ZOOM_STEP).clamp(ZOOM_MIN, ZOOM_MAX);
        }
    }

    if keys.just_pressed(KeyCode::KeyQ) { nav.yaw_target -= YAW_STEP; }
    if keys.just_pressed(KeyCode::KeyE) { nav.yaw_target += YAW_STEP; }
    if (nav.yaw - nav.yaw_target).abs() > 1e-4 {
        nav.yaw = lerp_angle(nav.yaw, nav.yaw_target, (ROT_LERP * dt).clamp(0.0, 1.0));
    }

    nav.focus_h = world.volume.surface_height(nav.focus.x.round() as i32, nav.focus.y.round() as i32).unwrap_or(world.volume.sea_level()) as f32;

    if let Ok((mut tf, mut proj, mut fog)) = cam.single_mut() {
        *tf = camera_transform(&nav);
        if let Projection::Orthographic(o) = proj.as_mut() {
            o.scaling_mode = ScalingMode::FixedVertical { viewport_height: nav.zoom };
        }
        // Fog band tracks the pawn's distance: clear up to the pawn (+a margin),
        // hazing as terrain recedes up-screen. Distance-from-camera under ortho ≈
        // depth ≈ screen-vertical, so left/right is unaffected.
        let dist = (tf.translation - focus3(&nav)).length();
        if nav.fog_on {
            let start = dist + nav.fog_start * nav.zoom;
            fog.falloff = FogFalloff::Linear { start, end: start + nav.fog_span * nav.zoom };
        } else {
            fog.falloff = FogFalloff::Linear { start: 1.0e9, end: 1.0e9 + 1.0 };
        }
    }
    if let Ok(mut tf) = pawn.single_mut() {
        *tf = pawn_transform(&nav);
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut d = (b - a).rem_euclid(std::f32::consts::TAU);
    if d > std::f32::consts::PI {
        d -= std::f32::consts::TAU;
    }
    a + d * t
}

// --- Verification screenshot -----------------------------------------------------

fn maybe_screenshot(
    time: Res<Time>,
    mut commands: Commands,
    mut nav: ResMut<Navigator>,
    mut shot: Local<bool>,
    mut framed: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    if !*framed && t > 0.3 {
        if let Ok(z) = std::env::var("VIVARIUM_ISO_ZOOM") {
            if let Ok(zv) = z.parse::<f32>() {
                nav.zoom = zv;
            }
        }
        *framed = true;
    }
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(8.0);
    if t > settle && !*shot {
        let path = PathBuf::from("/tmp/vivarium_isovoxels_shot.png");
        eprintln!("[iso-voxels] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.5 {
        exit.write(AppExit::Success);
    }
}
