//! vivarium-worldview — the first view over the clean-room frame.
//!
//! Depends on `vivarium-world` ONLY (the core/view wall). Where `spikes/slabs`
//! renders `vivarium-core`'s flat baked patch, this renders the cube-sphere frame:
//! pick a face region, sample it into field patches (`sample::sample_surface` —
//! data only), and build the proven point-mesh + translucent depth-shaded water
//! (idioms carried from slabs, which stays the core-backed SOTA until this
//! matures).
//!
//! It is a **survey instrument** for the fidelity ladder, not a pawn-walker yet:
//! the crude baseline's finest feature is ~40 km (fBm, 6 octaves over a face), so
//! the default sampling level is coarse (L14 ≈ 611 m cells ≈ a 156 km window) and
//! the "pawn" is a focus marker. The interesting dials:
//!   • `[` / `]` — change the sampling LEVEL live (same geographic spot, finer or
//!     coarser cells). Every rebuild reports its generation time on the HUD, so
//!     when query-graph memoization lands (DESIGN-REDUX §11–12), a revisited
//!     (level, region) will visibly drop to ~0 ms. That instrument is the point.
//!   • VIVARIUM_VERT — vertical exaggeration (default 1 = honest scale).
//!
//! Floating origin, done right (the audit's far-lands item): global face-cell
//! coordinates in metres reach ~10^7 m, where f32 resolves only ~1 m — so mesh
//! vertices AND the camera work relative to a f64 anchor at the patch centre, and
//! only anchored-relative f32 ever reaches the GPU.

use std::path::PathBuf;

use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::math::DVec2;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use vivarium_world::gen::SEA_LEVEL_M;
use vivarium_world::planet::Planet;
use vivarium_world::sample::{cell_size_m, sample_surface, SurfacePatch};
use vivarium_world::sphere::Face;

const SKY: Color = Color::srgb(0.80, 0.82, 0.84);
/// Water colour by depth (Beer–Lambert), as proven in slabs — but per METRE here
/// (the frame speaks metres, not voxels): ~30 m reads mostly opaque deep blue.
const WATER_SHALLOW: [f32; 3] = [0.46, 0.63, 0.75];
const WATER_DEEP: [f32; 3] = [0.05, 0.16, 0.38];
const WATER_ABSORB_PER_M: f32 = 0.06;

// --- Framing (ortho, true-iso default — slabs' proven constants) -------------------
const ISO_PITCH: f32 = 0.615_479_7; // atan(1/√2)
const YAW_START: f32 = std::f32::consts::FRAC_PI_4;
const YAW_STEP: f32 = std::f32::consts::FRAC_PI_4;
const ROT_LERP: f32 = 12.0;
const PITCH_MIN: f32 = 0.10;
const PITCH_MAX: f32 = 1.50;
const PITCH_RATE: f32 = 0.9;
const PAN_RATE: f32 = 0.5; // fraction of the zoom per second
const ZOOM_STEP: f32 = 1.15;
/// Focus sits this fraction of the viewport below centre (slabs' aim-ahead trick).
const FOCUS_BELOW_CENTER: f32 = 0.22;
/// Ortho standoff: must exceed any terrain span we render (affects clip, not size).
const STANDOFF: f32 = 400_000.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vivarium — worldview (cube-sphere frame)".into(),
                resolution: bevy::window::WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(SKY))
        .init_resource::<View>()
        .add_systems(Startup, setup)
        .add_systems(Update, (view_update, terrain_update, hud_update, maybe_screenshot))
        .run();
}

// --- View state --------------------------------------------------------------------

#[derive(Resource)]
struct View {
    face: Face,
    /// Cube-sphere sampling level (cell size = quarter-circumference / 2^level).
    level: u8,
    /// Patch width in cells (constant across levels; span = w · cell).
    w: usize,
    /// Focus in face-cell coordinates AT `level` (f64: cells are integers but the
    /// focus glides). Rescaled ×2 / ÷2 when the level changes so the geographic
    /// point is preserved.
    focus: DVec2,
    yaw: f32,
    yaw_target: f32,
    pitch: f32,
    /// Viewport height in METRES (absolute, so changing level doesn't jump zoom).
    zoom: f32,
    /// Vertical exaggeration (1 = honest).
    vert: f32,
}

impl Default for View {
    fn default() -> Self {
        let level: u8 = std::env::var("VIVARIUM_LEVEL").ok().and_then(|s| s.parse().ok()).unwrap_or(14);
        let w: usize = std::env::var("VIVARIUM_W").ok().and_then(|s| s.parse().ok()).unwrap_or(256);
        let n = (1u64 << level) as f64;
        let span = w as f64 * cell_size_m(level, Planet::EARTH.radius_m);
        // Start focus in face cells at `level` (env for scripted/reproducible views —
        // the scan_land example in vivarium-world prints good coastal candidates).
        let fi = std::env::var("VIVARIUM_FOCUS_I").ok().and_then(|s| s.parse().ok()).unwrap_or(n * 0.5);
        let fj = std::env::var("VIVARIUM_FOCUS_J").ok().and_then(|s| s.parse().ok()).unwrap_or(n * 0.5);
        Self {
            face: Face::ZPos,
            level,
            w,
            focus: DVec2::new(fi, fj),
            yaw: YAW_START,
            yaw_target: YAW_START,
            pitch: ISO_PITCH,
            zoom: std::env::var("VIVARIUM_ZOOM").ok().and_then(|s| s.parse().ok()).unwrap_or((span * 0.6) as f32),
            vert: std::env::var("VIVARIUM_VERT").ok().and_then(|s| s.parse().ok()).unwrap_or(1.0),
        }
    }
}

impl View {
    fn cell_m(&self) -> f64 {
        cell_size_m(self.level, Planet::EARTH.radius_m)
    }
    /// Clamp the focus so the sampled window stays on the face.
    fn clamp_focus(&mut self) {
        let n = (1u64 << self.level) as f64;
        let half = self.w as f64 * 0.5;
        self.focus.x = self.focus.x.clamp(half, n - half);
        self.focus.y = self.focus.y.clamp(half, n - half);
    }
}

// --- Terrain state -----------------------------------------------------------------

#[derive(Resource)]
struct TerrainState {
    ground: Option<Entity>,
    water: Option<Entity>,
    /// What the current meshes were built from.
    built_level: u8,
    origin: (u32, u32),
    /// f64 anchor (metres, face coords) at the patch centre; every f32 the GPU or
    /// camera sees is relative to this (floating origin).
    anchor_m: DVec2,
    /// The sampled fields (kept so the camera/HUD can read the height under the focus).
    fields: Option<SurfacePatch>,
    /// Generation + meshing time of the last rebuild — the memoization instrument.
    gen_ms: f32,
    ground_mat: Handle<StandardMaterial>,
    water_mat: Handle<StandardMaterial>,
}

#[derive(Component)]
struct IsoCamera;
#[derive(Component)]
struct FocusMarker;
#[derive(Component)]
struct HudText;

fn setup(mut commands: Commands, view: Res<View>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: view.zoom },
            far: STANDOFF + 2_000_000.0,
            near: -1.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
        IsoCamera,
        DistanceFog { color: SKY, falloff: FogFalloff::Linear { start: STANDOFF, end: STANDOFF + 2000.0 }, ..default() },
    ));

    let ground_mat = materials.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.95, cull_mode: None, ..default() });
    let water_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.2,
        reflectance: 0.3,
        cull_mode: None,
        ..default()
    });
    commands.insert_resource(TerrainState {
        ground: None,
        water: None,
        built_level: 0,
        origin: (0, 0),
        anchor_m: DVec2::ZERO,
        fields: None,
        gen_ms: 0.0,
        ground_mat,
        water_mat,
    });

    // Focus marker — a cursor, not a pawn (a 2 m pawn is sub-pixel at survey scale;
    // the pawn returns when fine tiers exist). Rescaled with zoom each frame.
    let marker_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let marker_mat = materials.add(StandardMaterial { base_color: Color::srgb(0.85, 0.18, 0.18), perceptual_roughness: 0.9, ..default() });
    commands.spawn((Mesh3d(marker_mesh), MeshMaterial3d(marker_mat), Transform::default(), FocusMarker));

    // Slabs' proven form-lighting: warm grazing key + cool fill + low ambient.
    commands.spawn((
        DirectionalLight { color: Color::srgb(1.0, 0.98, 0.92), shadows_enabled: false, illuminance: 6500.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.6, -0.55, 0.45), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight { color: Color::srgb(0.58, 0.70, 0.92), shadows_enabled: false, illuminance: 1900.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.5, -0.4, -0.45), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight { color: SKY, brightness: 190.0, affects_lightmapped_meshes: true });

    commands.spawn((
        Text::new("…"),
        TextFont { font_size: 15.0, ..default() },
        TextColor(Color::srgb(0.08, 0.09, 0.10)),
        Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(10.0), padding: UiRect::all(Val::Px(6.0)), ..default() },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        HudText,
    ));

    println!("[worldview] WASD pan · wheel zoom · Q/E rotate · K/J angle · [ ] sampling level");
}

// --- Input + camera ----------------------------------------------------------------

fn view_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    mut view: ResMut<View>,
    ts: Res<TerrainState>,
    mut cam: Query<&mut Transform, (With<IsoCamera>, Without<FocusMarker>)>,
    mut cam_proj: Query<&mut Projection, With<IsoCamera>>,
    mut fog: Query<&mut DistanceFog, With<IsoCamera>>,
    mut marker: Query<&mut Transform, (With<FocusMarker>, Without<IsoCamera>)>,
) {
    let dt = time.delta_secs();

    // Pan in the camera frame, in metres, converted to face cells.
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { dir.x += 1.0; }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) { dir.x -= 1.0; }
    if dir != Vec2::ZERO {
        let (s, c) = view.yaw.sin_cos();
        let world = Vec2::new(-dir.x * c + dir.y * s, dir.x * s + dir.y * c).normalize_or_zero();
        let step_m = (PAN_RATE * view.zoom * dt) as f64;
        let cell = view.cell_m();
        view.focus += DVec2::new(world.x as f64, world.y as f64) * (step_m / cell);
        view.clamp_focus();
    }
    for ev in wheel.read() {
        if ev.y > 0.0 {
            view.zoom /= ZOOM_STEP;
        } else if ev.y < 0.0 {
            view.zoom *= ZOOM_STEP;
        }
        let span = (view.w as f64 * view.cell_m()) as f32;
        view.zoom = view.zoom.clamp(span * 0.02, span * 1.5);
    }
    if keys.just_pressed(KeyCode::KeyQ) { view.yaw_target += YAW_STEP; }
    if keys.just_pressed(KeyCode::KeyE) { view.yaw_target -= YAW_STEP; }
    if (view.yaw - view.yaw_target).abs() > 1e-4 {
        view.yaw = lerp_angle(view.yaw, view.yaw_target, (ROT_LERP * dt).clamp(0.0, 1.0));
    }
    if keys.pressed(KeyCode::KeyK) { view.pitch = (view.pitch + PITCH_RATE * dt).min(PITCH_MAX); }
    if keys.pressed(KeyCode::KeyJ) { view.pitch = (view.pitch - PITCH_RATE * dt).max(PITCH_MIN); }

    // Sampling level: same geographic point, finer/coarser cells. Focus is in cells
    // at the current level, so it rescales by 2 exactly.
    if keys.just_pressed(KeyCode::BracketRight) && view.level < 22 {
        view.level += 1;
        view.focus *= 2.0;
        view.clamp_focus();
    }
    if keys.just_pressed(KeyCode::BracketLeft) && view.level > 6 {
        view.level -= 1;
        view.focus *= 0.5;
        view.clamp_focus();
    }

    // Camera + marker, all relative to the terrain anchor (floating origin).
    let cell = view.cell_m();
    let focus_m = view.focus * cell;
    let rel = focus_m - ts.anchor_m;
    let focus_h = height_at_focus(&view, &ts) * view.vert;
    let aim_base = Vec3::new(rel.x as f32, focus_h, rel.y as f32);

    let look = (Vec3::new(view.yaw.sin(), 0.0, view.yaw.cos()) * view.pitch.cos() + Vec3::NEG_Y * view.pitch.sin()).normalize();
    let forward_h = Vec3::new(view.yaw.sin(), 0.0, view.yaw.cos());
    let aim = aim_base + forward_h * (view.zoom * FOCUS_BELOW_CENTER / view.pitch.sin().max(0.15));
    if let Ok(mut t) = cam.single_mut() {
        *t = Transform::from_translation(aim - look * STANDOFF).looking_at(aim, Vec3::Y);
    }
    if let Ok(mut proj) = cam_proj.single_mut() {
        if let Projection::Orthographic(o) = proj.as_mut() {
            o.scaling_mode = ScalingMode::FixedVertical { viewport_height: view.zoom };
        }
    }
    // Mild haze for a survey instrument: begin a full viewport beyond the aim so
    // the working window stays clear (slabs' tighter band at these zooms whites
    // out most of the scene).
    if let Ok(mut f) = fog.single_mut() {
        f.falloff = FogFalloff::Linear { start: STANDOFF + view.zoom * 1.2, end: STANDOFF + view.zoom * 8.0 };
    }
    if let Ok(mut m) = marker.single_mut() {
        let s = view.zoom * 0.01;
        *m = Transform::from_translation(aim_base + Vec3::Y * s).with_scale(Vec3::new(s, s * 2.0, s));
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let d = (b - a + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    a + d * t
}

/// Height (m above sea, un-exaggerated) under the focus, from the sampled fields.
fn height_at_focus(view: &View, ts: &TerrainState) -> f32 {
    let Some(fields) = &ts.fields else { return 0.0 };
    let x = (view.focus.x - ts.origin.0 as f64).floor() as isize;
    let y = (view.focus.y - ts.origin.1 as f64).floor() as isize;
    let w = view.w as isize;
    if ts.built_level == view.level && x >= 0 && x < w && y >= 0 && y < w {
        fields.height.get(x, y) - SEA_LEVEL_M as f32
    } else {
        0.0
    }
}

// --- Terrain sampling + meshing ------------------------------------------------------

fn terrain_update(mut commands: Commands, view: Res<View>, mut meshes: ResMut<Assets<Mesh>>, mut ts: ResMut<TerrainState>) {
    // Rebuild when the level changed or the focus has drifted toward the window edge.
    let needs = match (&ts.fields, ts.built_level == view.level) {
        (None, _) => true,
        (_, false) => true,
        (Some(_), true) => {
            let cx = ts.origin.0 as f64 + view.w as f64 * 0.5;
            let cy = ts.origin.1 as f64 + view.w as f64 * 0.5;
            (view.focus.x - cx).abs().max((view.focus.y - cy).abs()) > view.w as f64 / 6.0
        }
    };
    if !needs {
        return;
    }

    let t0 = std::time::Instant::now();
    let n = 1u64 << view.level;
    let half = view.w as u64 / 2;
    let oi = (view.focus.x.round().max(0.0) as u64).clamp(half, n - half) - half;
    let oj = (view.focus.y.round().max(0.0) as u64).clamp(half, n - half) - half;
    let (oi, oj) = (oi as u32, oj as u32);

    let fields = sample_surface(view.face, view.level, oi, oj, view.w);
    let cell = view.cell_m();
    let anchor = DVec2::new((oi as f64 + view.w as f64 * 0.5) * cell, (oj as f64 + view.w as f64 * 0.5) * cell);

    for e in [ts.ground.take(), ts.water.take()].into_iter().flatten() {
        commands.entity(e).despawn();
    }
    let ground = build_ground_mesh(&fields, view.w, cell, anchor, (oi, oj), view.vert);
    ts.ground = Some(commands.spawn((Mesh3d(meshes.add(ground)), MeshMaterial3d(ts.ground_mat.clone()), Transform::default())).id());
    if let Some(water) = build_water_mesh(&fields, view.w, cell, anchor, (oi, oj), view.vert) {
        ts.water = Some(commands.spawn((Mesh3d(meshes.add(water)), MeshMaterial3d(ts.water_mat.clone()), Transform::default())).id());
    }

    ts.built_level = view.level;
    ts.origin = (oi, oj);
    ts.anchor_m = anchor;
    ts.fields = Some(fields);
    ts.gen_ms = t0.elapsed().as_secs_f32() * 1000.0;
}

/// Elevation-ramp colour for the crude rung (materials are uniform soil-on-igneous
/// at this tier, so elevation is the honest signal to paint).
fn ground_color(h_above_sea: f32) -> [f32; 4] {
    let mix = |a: [f32; 3], b: [f32; 3], t: f32| -> [f32; 3] {
        let t = t.clamp(0.0, 1.0);
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
    };
    // Bands scaled to the prior's actual relief (continents ±1500 m + mountains to
    // ~+3300 m): green lowlands to ~1 km, bare stone through ~2 km, snow above.
    const BED: [f32; 3] = [0.45, 0.40, 0.32]; // submerged sediment
    const SHORE: [f32; 3] = [0.70, 0.64, 0.48];
    const GRASS: [f32; 3] = [0.34, 0.52, 0.26];
    const STONE: [f32; 3] = [0.50, 0.49, 0.46];
    const SNOW: [f32; 3] = [0.92, 0.93, 0.94];
    let c = if h_above_sea < 0.0 {
        mix(BED, SHORE, (h_above_sea + 150.0) / 150.0)
    } else if h_above_sea < 80.0 {
        mix(SHORE, GRASS, h_above_sea / 80.0)
    } else if h_above_sea < 2000.0 {
        mix(GRASS, STONE, (h_above_sea - 1000.0) / 1000.0)
    } else {
        mix(STONE, SNOW, (h_above_sea - 2000.0) / 600.0)
    };
    [c[0], c[1], c[2], 1.0]
}

/// Continuous point-mesh over the solid surface (slabs' proven model): one vertex
/// per cell centre, smooth normals from the height gradient (halo makes edges work).
fn build_ground_mesh(f: &SurfacePatch, w: usize, cell: f64, anchor: DVec2, origin: (u32, u32), vert: f32) -> Mesh {
    let h = |x: isize, y: isize| -> f32 { f.height.get(x, y) };
    let px = |i: usize| ((origin.0 as f64 + i as f64 + 0.5) * cell - anchor.x) as f32;
    let pz = |j: usize| ((origin.1 as f64 + j as f64 + 0.5) * cell - anchor.y) as f32;
    let py = |x: isize, y: isize| (h(x, y) - SEA_LEVEL_M as f32) * vert;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(w * w);
    for j in 0..w {
        for i in 0..w {
            let (x, y) = (i as isize, j as isize);
            positions.push([px(i), py(x, y), pz(j)]);
            colors.push(ground_color(h(x, y) - SEA_LEVEL_M as f32));
            let nrm = Vec3::new(py(x - 1, y) - py(x + 1, y), 2.0 * cell as f32, py(x, y - 1) - py(x, y + 1)).normalize();
            normals.push([nrm.x, nrm.y, nrm.z]);
        }
    }
    let mut indices: Vec<u32> = Vec::with_capacity((w - 1) * (w - 1) * 6);
    for j in 0..w - 1 {
        for i in 0..w - 1 {
            let (a, b, c, d) = ((j * w + i) as u32, (j * w + i + 1) as u32, ((j + 1) * w + i) as u32, ((j + 1) * w + i + 1) as u32);
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Translucent water point-mesh at the sea surface, depth-shaded (Beer–Lambert per
/// metre). Quads only where all four corners are wet (slabs' known 1-cell shore
/// inset — acceptable, noted).
fn build_water_mesh(f: &SurfacePatch, w: usize, cell: f64, anchor: DVec2, origin: (u32, u32), vert: f32) -> Option<Mesh> {
    let px = |i: usize| ((origin.0 as f64 + i as f64 + 0.5) * cell - anchor.x) as f32;
    let pz = |j: usize| ((origin.1 as f64 + j as f64 + 0.5) * cell - anchor.y) as f32;

    let mut wet = vec![false; w * w];
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(w * w);
    for j in 0..w {
        for i in 0..w {
            let depth = f.water.get(i as isize, j as isize);
            wet[j * w + i] = depth > 0.0;
            // Water surface = solid top + depth (baseline: the sea plane at y = 0).
            let surf = (f.height.get(i as isize, j as isize) + depth - SEA_LEVEL_M as f32) * vert;
            let m = 1.0 - (-depth * WATER_ABSORB_PER_M).exp();
            positions.push([px(i), surf, pz(j)]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push([
                WATER_SHALLOW[0] + (WATER_DEEP[0] - WATER_SHALLOW[0]) * m,
                WATER_SHALLOW[1] + (WATER_DEEP[1] - WATER_SHALLOW[1]) * m,
                WATER_SHALLOW[2] + (WATER_DEEP[2] - WATER_SHALLOW[2]) * m,
                m.clamp(0.15, 0.92),
            ]);
        }
    }
    let mut indices: Vec<u32> = Vec::new();
    for j in 0..w - 1 {
        for i in 0..w - 1 {
            let (a, b, c, d) = (j * w + i, j * w + i + 1, (j + 1) * w + i, (j + 1) * w + i + 1);
            if wet[a] && wet[b] && wet[c] && wet[d] {
                indices.extend_from_slice(&[a as u32, c as u32, b as u32, b as u32, c as u32, d as u32]);
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

// --- HUD -----------------------------------------------------------------------------

fn compass(yaw: f32) -> &'static str {
    const L: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let deg = yaw.to_degrees().rem_euclid(360.0);
    L[(((deg + 22.5) / 45.0) as usize) % 8]
}

fn hud_update(view: Res<View>, ts: Res<TerrainState>, diag: Res<bevy::diagnostic::DiagnosticsStore>, mut q: Query<&mut Text, With<HudText>>) {
    let fps = diag.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()).unwrap_or(0.0);
    let cell = view.cell_m();
    let span_km = view.w as f64 * cell / 1000.0;
    let elev = height_at_focus(&view, &ts);
    if let Ok(mut text) = q.single_mut() {
        text.0 = format!(
            "worldview    {fps:>4.0} fps    gen {:.0} ms\n{:?}  L{}  cell {:.0} m  window {:.0} km\nfocus ({:.0}, {:.0})    elev {elev:.0} m\nfacing {}    angle {:.0} deg    zoom {:.0} m    vert {:.1}",
            ts.gen_ms,
            view.face,
            view.level,
            cell,
            span_km,
            view.focus.x,
            view.focus.y,
            compass(view.yaw_target),
            view.pitch.to_degrees(),
            view.zoom,
            view.vert,
        );
    }
}

fn maybe_screenshot(time: Res<Time>, mut commands: Commands, mut shot: Local<bool>, mut exit: MessageWriter<AppExit>) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(2.5);
    if t > settle && !*shot {
        let path = PathBuf::from("/tmp/vivarium_worldview_shot.png");
        eprintln!("[worldview] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.5 {
        exit.write(AppExit::Success);
    }
}
