//! vivarium-slabs — terrain as stacked level-slabs, as REAL 3D geometry.
//!
//! This is the "physically-defensible model" the tilemap spike was missing. That
//! view composited four independent 2D primitives (tread fill + face smear + contour
//! lines + a base) and spent its life fighting the seams where they failed to line
//! up — gaps, occlusion, cut-backs. Here we stop compositing and instead build the
//! actual surface: quantize the height field into integer elevation LEVELS, then emit
//! real geometry — one horizontal quad per cell at its level's height, plus a vertical
//! quad down every drop to a lower neighbour. Bevy's orthographic camera projects it
//! and the hardware depth buffer resolves occlusion. Consequences, all for free:
//!   • solid meshes ⇒ a coverage gap is impossible (no magenta sentinel needed);
//!   • the z-buffer ⇒ occlusion is correct by construction (no depth-key hacks);
//!   • a directional light ⇒ face shading comes from real normals, not a facing dot.
//!
//! Deliberately NOT here yet (per Joseph): chamfering, material-from-geology, the
//! reading aids. First prove the model — pancakes of terrain floating on the lower
//! elevations — is clean at its foundation.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use vivarium_core::voxel::{Volume, WORLDGEN_VERSION};
use vivarium_core::World;

// --- Worldgen params (matched to the other adapters → shared cache) ---------------
const SEED: u64 = 0x00C0_FFEE;
const DETAIL: i32 = 2;
const DEFAULT_REGION_HALF_M: i32 = 6_000;
const EPOCHS: u32 = 70;
const FINE_CELL_M: f32 = 8.0;
const FINE_EPOCHS: u32 = 0;
const N_AGENTS: usize = 24;

const SKY: Color = Color::srgb(0.80, 0.82, 0.84);
/// Water colour by depth (Beer–Lambert): shallow water is clear pale blue and lets
/// the bed show; as the column deepens the light transmitted falls off, so it turns
/// opaque and dark — occluding the ground more the deeper it is.
const WATER_SHALLOW: [f32; 3] = [0.46, 0.63, 0.75];
const WATER_DEEP: [f32; 3] = [0.05, 0.16, 0.38];
const WATER_ABSORB: f32 = 0.09; // per voxel (~0.5 m): ~15 voxels ≈ 75% opaque

// --- Iso framing (orthographic, true-iso angle) ----------------------------------
const ISO_PITCH: f32 = 0.615_479_7; // atan(1/sqrt(2))
const YAW_START: f32 = std::f32::consts::FRAC_PI_4;
const YAW_STEP: f32 = std::f32::consts::FRAC_PI_4; // 45° → 8 viewing directions
const STANDOFF: f32 = 4_000.0; // ortho: only affects occlusion/clip, not apparent size
const ZOOM_MIN: f32 = 40.0;
const ZOOM_MAX: f32 = 2_000.0;
const ZOOM_START: f32 = 260.0;
const ZOOM_STEP: f32 = 1.15;
const PAN_RATE: f32 = 0.9;
const ROT_LERP: f32 = 12.0;
/// Where the pawn sits on screen, as a fraction of the viewport BELOW centre. 0.30 ⇒
/// ~1/5 up from the bottom. Held constant across pitch: aiming ahead by A shifts the
/// pawn down-screen by A·sin(pitch)/zoom, so the aim-ahead is scaled by 1/sin(pitch)
/// to keep the pawn fixed as the angle changes (auto-pitch, R/F).
const PAWN_BELOW_CENTER: f32 = 0.30;
/// Viewing-angle (pitch) limits and rates. Higher pitch = more top-down = less
/// foreground occlusion. ISO_PITCH is the pleasant default; auto-pitch raises above
/// it only as far as needed to keep the pawn in sight.
const PITCH_MIN: f32 = -0.5; // below the pawn (~-29°): look UP at mountains. Foreground
                             // nearer than the pawn is near-clipped away so it can't wall
                             // off the view (see navigator_update).
const PITCH_MAX: f32 = 1.50; // ~86°: near top-down, so even a huge close mountain
                             // can be cleared (auto-pitch only goes this steep when
                             // one actually demands it; normal terrain stays iso).
const PITCH_RATE: f32 = 0.9; // rad/s while R/F held
const PITCH_LERP_UP: f32 = 16.0; // catch up fast when a mountain rises into the way
const PITCH_LERP_DOWN: f32 = 4.0; // relax gently back down once it's clear

/// Default voxels per mesh cell (the footprint of one slab tile). 6 vox = 3 m at
/// detail 2. Coarser cells group the terrain into fewer, larger terraces (env
/// VIVARIUM_STRIDE) — the master "how blocky" dial alongside the band size.
const STRIDE_DEFAULT: i32 = 12;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vivarium — slabs (level meshes, ortho)".into(),
                resolution: bevy::window::WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(load_world())
        .init_resource::<Navigator>()
        .add_systems(Startup, setup)
        .add_systems(Update, (navigator_update, update_terrain, update_hud, maybe_screenshot))
        .run();
}

// --- World (shared cache) --------------------------------------------------------

#[derive(Resource, Clone)]
struct VivWorld {
    volume: Arc<Volume>,
    sea: i32,
}

fn load_world() -> VivWorld {
    let region_half = std::env::var("VIVARIUM_REGION_HALF").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_REGION_HALF_M);
    let volume = if std::env::var_os("VIVARIUM_NO_CACHE").is_some() {
        generate(region_half)
    } else {
        let path = cache_path(region_half);
        match std::fs::read(&path).ok().and_then(|b| Volume::from_bytes(&b)) {
            Some(v) => {
                eprintln!("slabs: worldgen cache HIT — {}", path.display());
                v
            }
            None => {
                let v = generate(region_half);
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let tmp = path.with_extension("bin.tmp");
                let _ = std::fs::write(&tmp, v.to_bytes()).and_then(|_| std::fs::rename(&tmp, &path));
                v
            }
        }
    };
    let sea = volume.sea_level();
    VivWorld { volume: Arc::new(volume), sea }
}

fn generate(region_half: i32) -> Volume {
    eprintln!("slabs: generating eroded world (±{region_half} m)… slow tier, minutes.");
    World::eroded(SEED, N_AGENTS, DETAIL, region_half, EPOCHS, FINE_CELL_M, FINE_EPOCHS).volume
}

fn cache_path(region_half: i32) -> PathBuf {
    let dir = std::env::var_os("VIVARIUM_CACHE_DIR").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("vivarium-worldcache"));
    dir.join(format!("viv_s{SEED:x}_d{DETAIL}_r{region_half}_e{EPOCHS}_fc{FINE_CELL_M:.1}_fe{FINE_EPOCHS}_v{WORLDGEN_VERSION}.bin"))
}

// --- Navigator + camera ----------------------------------------------------------

#[derive(Resource)]
struct Navigator {
    focus: Vec2,  // world voxels (x, z)
    focus_h: f32, // slab height at the focus (world Y) — the camera aims here
    yaw: f32,
    yaw_target: f32,
    pitch: f32,        // current viewing angle (smoothed)
    pitch_manual: f32, // the angle you set with R/F — the floor for auto-pitch
    auto_pitch: bool,  // raise pitch automatically so foreground never hides the pawn
    zoom: f32,
    /// Voxels per elevation band (the quantization) and the vertical exaggeration of
    /// the slab heights. Real height would be `level·level_vox`; `vert` scales that.
    level_vox: i32,
    vert: f32,
    stride: i32,
    points: bool, // true = continuous point-mesh surface; false = blocky slabs
}

impl Default for Navigator {
    fn default() -> Self {
        let fx = std::env::var("VIVARIUM_FOCUS_X").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let fz = std::env::var("VIVARIUM_FOCUS_Z").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        // VIVARIUM_PITCH (radians) forces a fixed starting angle with auto off — handy
        // for screenshotting a specific view (e.g. a negative value to look up).
        let manual_pitch = std::env::var("VIVARIUM_PITCH").ok().and_then(|s| s.parse::<f32>().ok());
        Self {
            focus: Vec2::new(fx, fz),
            focus_h: 0.0,
            yaw: YAW_START,
            yaw_target: YAW_START,
            pitch: manual_pitch.unwrap_or(ISO_PITCH),
            pitch_manual: manual_pitch.unwrap_or(ISO_PITCH),
            auto_pitch: manual_pitch.is_none() && std::env::var("VIVARIUM_AUTO_PITCH").map(|v| v != "0").unwrap_or(true),
            zoom: std::env::var("VIVARIUM_ZOOM").ok().and_then(|s| s.parse().ok()).unwrap_or(ZOOM_START),
            level_vox: std::env::var("VIVARIUM_LEVEL_VOX").ok().and_then(|s| s.parse().ok()).unwrap_or(18),
            vert: std::env::var("VIVARIUM_VERT").ok().and_then(|s| s.parse().ok()).unwrap_or(0.6),
            stride: std::env::var("VIVARIUM_STRIDE").ok().and_then(|s| s.parse().ok()).unwrap_or(STRIDE_DEFAULT),
            points: std::env::var("VIVARIUM_POINTS").map(|v| v != "0").unwrap_or(true),
        }
    }
}

#[derive(Component)]
struct IsoCamera;
#[derive(Component)]
struct Pawn;

fn setup(mut commands: Commands, world: Res<VivWorld>, mut nav: ResMut<Navigator>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    nav.focus_h = focus3(&world, &nav).y;
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
        // Light distance haze: terrain far up-slope (beyond the pawn) fades toward the
        // sky. The band is retuned each frame from the zoom (navigator_update).
        DistanceFog { color: SKY, falloff: FogFalloff::Linear { start: STANDOFF, end: STANDOFF + 2000.0 }, ..default() },
    ));

    // Ground: one lit, vertex-coloured opaque surface. Double-sided so a stray
    // back-facing quad can't punch a hole while we're still proving the geometry.
    let ground_mat = materials.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.95, cull_mode: None, ..default() });
    // Water: a translucent blended surface; per-vertex colour carries the blue + alpha.
    let water_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.2,
        reflectance: 0.3,
        cull_mode: None,
        ..default()
    });
    commands.insert_resource(TerrainState { ground: None, water: None, center: Vec2::splat(1.0e9), zoom: 0.0, points: !nav.points, ground_mat, water_mat });

    // Pawn: a human-scale red marker at the focus — 0.5 m² footprint (1 voxel) and
    // 2 m tall (4 voxels), standing on its slab. (Tiny on screen at a wide zoom, as
    // a real person would be — a beacon/always-on-top marker is a later option.)
    let pawn_mesh = meshes.add(Cuboid::new(1.0, 4.0, 1.0));
    let pawn_mat = materials.add(StandardMaterial { base_color: Color::srgb(0.85, 0.18, 0.18), perceptual_roughness: 0.9, ..default() });
    commands.spawn((Mesh3d(pawn_mesh), MeshMaterial3d(pawn_mat), Transform::default(), Pawn));

    // Lighting for terrain FORM (contrast), not flat overcast:
    //  • warm key at a low, grazing angle — the rake is what makes slopes read;
    //  • a dim cool fill from the opposite side so shadowed slopes don't crush black;
    //  • a low cool ambient as the floor.
    commands.spawn((
        DirectionalLight { color: Color::srgb(1.0, 0.98, 0.92), shadows_enabled: false, illuminance: 6500.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.6, -0.55, 0.45), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight { color: Color::srgb(0.58, 0.70, 0.92), shadows_enabled: false, illuminance: 1900.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.5, -0.4, -0.45), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight { color: SKY, brightness: 190.0, affects_lightmapped_meshes: true });

    // HUD — everything needed to describe/replicate the current view for feedback.
    commands.spawn((
        Text::new("…"),
        TextFont { font_size: 15.0, ..default() },
        TextColor(Color::srgb(0.08, 0.09, 0.10)),
        Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(10.0), padding: UiRect::all(Val::Px(6.0)), ..default() },
        // Faint white panel so the dark text stays legible over dark terrain.
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        HudText,
    ));

    println!("[slabs] WASD pan · wheel zoom · Q/E rotate · K/J raise/lower angle · Y auto-angle · P slabs↔point-mesh");
}

#[derive(Component)]
struct HudText;

/// 8-point compass label for a camera yaw (the direction it faces into the scene).
fn compass(yaw: f32) -> &'static str {
    const L: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let deg = yaw.to_degrees().rem_euclid(360.0);
    L[(((deg + 22.5) / 45.0) as usize) % 8]
}

/// The HUD: fps, the tile the pawn stands on + its elevation (m above sea), the
/// facing compass, the camera elevation angle, and the zoom — the full camera state
/// needed to reproduce a screen for terrain/core feedback, plus the mesh params.
fn update_hud(nav: Res<Navigator>, world: Res<VivWorld>, diag: Res<bevy::diagnostic::DiagnosticsStore>, mut q: Query<&mut Text, With<HudText>>) {
    let fps = diag.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()).unwrap_or(0.0);
    let (x, z) = (nav.focus.x.round() as i32, nav.focus.y.round() as i32);
    let bed = world.volume.surface_height(x, z).unwrap_or(world.sea);
    let elev_m = (bed - world.sea) as f32 / DETAIL as f32; // metres above sea (0.5 m/voxel)
    let mode = if nav.points { "point" } else { "slab" };
    if let Ok(mut text) = q.single_mut() {
        text.0 = format!(
            "slabs    {fps:>4.0} fps\npawn ({x}, {z})    elev {elev_m:.0} m\nfacing {}    angle {:.0} deg    zoom {:.0}\nstride {}    band {} vox    vert {:.1}    {mode}",
            compass(nav.yaw_target),
            nav.pitch.to_degrees(),
            nav.zoom,
            nav.stride,
            nav.level_vox,
            nav.vert,
        );
    }
}

#[derive(Resource)]
struct TerrainState {
    ground: Option<Entity>,
    water: Option<Entity>,
    center: Vec2,
    zoom: f32,
    points: bool,
    ground_mat: Handle<StandardMaterial>,
    water_mat: Handle<StandardMaterial>,
}

/// (Re)build the ground + water meshes around the focus when it moves far enough, the
/// zoom changes, or the slab/point mode is toggled — sized to the visible footprint.
fn update_terrain(mut commands: Commands, world: Res<VivWorld>, nav: Res<Navigator>, mut meshes: ResMut<Assets<Mesh>>, mut ts: ResMut<TerrainState>) {
    // Cover generously past the frustum so hilly terrain doesn't reveal the void at
    // the edges (the camera aims ahead and looks up-slope, so it sees well past the
    // focus). Bigger = fewer black edges, more triangles per rebuild.
    let radius = (nav.zoom * 2.4 + 450.0).min(3400.0);
    let moved = ts.center.distance(nav.focus);
    if ts.ground.is_some() && moved < radius * 0.3 && (ts.zoom - nav.zoom).abs() < 1.0 && ts.points == nav.points {
        return;
    }
    for e in [ts.ground.take(), ts.water.take()].into_iter().flatten() {
        commands.entity(e).despawn();
    }
    if let Some(mesh) = build_terrain_mesh(&world, nav.focus, radius, nav.level_vox, nav.vert, nav.stride, nav.points) {
        let h = meshes.add(mesh);
        ts.ground = Some(commands.spawn((Mesh3d(h), MeshMaterial3d(ts.ground_mat.clone()), Transform::default())).id());
    }
    if let Some(mesh) = build_water_mesh(&world, nav.focus, radius, nav.level_vox, nav.vert, nav.stride) {
        let h = meshes.add(mesh);
        ts.water = Some(commands.spawn((Mesh3d(h), MeshMaterial3d(ts.water_mat.clone()), Transform::default())).id());
    }
    ts.center = nav.focus;
    ts.zoom = nav.zoom;
    ts.points = nav.points;
}

/// Slab-top world height for an elevation `level` (quantized, vertically scaled).
fn slab_y(sea: i32, level: i32, level_vox: i32, vert: f32) -> f32 {
    sea as f32 + level as f32 * level_vox as f32 * vert
}

/// Build the terrain surface mesh over a `2·radius` voxel square around `center`:
/// a top quad per cell at its level height, plus a vertical quad down every drop to
/// a lower neighbour (the cliff faces). One vertex-coloured mesh; the GPU does the
/// rest. Colour is placeholder-simple — grass/water/rock tops, earthy sides.
fn build_terrain_mesh(world: &VivWorld, center: Vec2, radius: f32, level_vox: i32, vert: f32, stride: i32, points: bool) -> Option<Mesh> {
    let volume = &world.volume;
    let sea = world.sea;
    let s = stride;
    let half = s as f32 * 0.5;
    let gx0 = (((center.x - radius) / s as f32).floor() as i32) * s;
    let gz0 = (((center.y - radius) / s as f32).floor() as i32) * s;
    let n = ((2.0 * radius) as i32 / s) as usize + 2;

    // Sample the quantized level + material for each cell once.
    let mut lvl = vec![0i32; n * n];
    let mut mat = vec![0u16; n * n];
    for j in 0..n {
        for i in 0..n {
            let x = gx0 + i as i32 * s;
            let z = gz0 + j as i32 * s;
            // Solid ground: always the BED (the lake/ocean floor is real terrain,
            // shown through the translucent water mesh above it), coloured by the
            // material of its top voxel.
            let bed = volume.surface_height(x, z).unwrap_or(sea);
            lvl[j * n + i] = ((bed - sea) as f32 / level_vox as f32).round() as i32;
            mat[j * n + i] = volume.voxel(x, bed, z).0;
        }
    }

    // De-noise the level field: N passes of a 3×3 median. Quantizing a bumpy
    // surface flips lone cells up/down a band, which a faithful mesh renders as
    // single-cell pillars/pits (visual confetti). The median erases those lone
    // flips while leaving real coastlines, ridges and cliffs — it is what the eye
    // does. VIVARIUM_SMOOTH sets the pass count (0 = raw, to see the noise).
    let smooth: u32 = std::env::var("VIVARIUM_SMOOTH").ok().and_then(|s| s.parse().ok()).unwrap_or(2);
    for _ in 0..smooth {
        let src = lvl.clone();
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let mut w = [0i32; 9];
                let mut k = 0;
                for dj in -1i32..=1 {
                    for di in -1i32..=1 {
                        w[k] = src[(j as i32 + dj) as usize * n + (i as i32 + di) as usize];
                        k += 1;
                    }
                }
                w.sort_unstable();
                lvl[j * n + i] = w[4]; // median of the 3×3
            }
        }
    }

    let top_color = |m: u16, level: i32| -> [f32; 4] {
        let base = match m {
            1 => [0.50, 0.49, 0.46], // stone
            2 => [0.56, 0.48, 0.36], // dirt
            _ => [0.34, 0.52, 0.26], // grass (3) / default — deeper, more saturated
        };
        let t = (1.0 + level as f32 * 0.012).clamp(0.85, 1.3); // subtle elevation tint
        [base[0] * t, base[1] * t, base[2] * t, 1.0]
    };
    let side_color: [f32; 4] = [0.44, 0.37, 0.29, 1.0]; // exposed earth/scree

    // --- Point-mesh mode (Joseph's idea) -------------------------------------------
    // One VERTEX per cell at its centre + height, and triangulate the grid into a
    // continuous surface. There are NO vertical faces, so the "wall at every band
    // crossing" that makes the slab model confetti simply doesn't exist — a slope
    // just slopes. This is the honest terrain surface (quantized heights, smoothly
    // interpolated between cell centres). Smooth per-vertex normals from the height
    // gradient give real lighting.
    if points {
        let cx = |i: usize| (gx0 + i as i32 * s) as f32 + half;
        let cz = |j: usize| (gz0 + j as i32 * s) as f32 + half;
        let vy = |i: usize, j: usize| slab_y(sea, lvl[j * n + i], level_vox, vert);
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n * n);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n * n);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(n * n);
        for j in 0..n {
            for i in 0..n {
                positions.push([cx(i), vy(i, j), cz(j)]);
                colors.push(top_color(mat[j * n + i], lvl[j * n + i]));
                let (il, ir) = (i.saturating_sub(1), (i + 1).min(n - 1));
                let (ju, jd) = (j.saturating_sub(1), (j + 1).min(n - 1));
                let nrm = Vec3::new(vy(il, j) - vy(ir, j), 2.0 * s as f32, vy(i, ju) - vy(i, jd)).normalize();
                normals.push([nrm.x, nrm.y, nrm.z]);
            }
        }
        let mut indices: Vec<u32> = Vec::with_capacity((n - 1) * (n - 1) * 6);
        for j in 0..n - 1 {
            for i in 0..n - 1 {
                let (a, b, c, d) = ((j * n + i) as u32, (j * n + i + 1) as u32, ((j + 1) * n + i) as u32, ((j + 1) * n + i + 1) as u32);
                indices.extend_from_slice(&[a, c, b, b, c, d]);
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U32(indices));
        return Some(mesh);
    }

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut quad = |p: [[f32; 3]; 4], nrm: [f32; 3], col: [f32; 4]| {
        let b = positions.len() as u32;
        positions.extend_from_slice(&p);
        normals.extend_from_slice(&[nrm; 4]);
        colors.extend_from_slice(&[col; 4]);
        indices.extend_from_slice(&[b, b + 2, b + 1, b + 1, b + 2, b + 3]);
    };

    for j in 0..n {
        for i in 0..n {
            let l = lvl[j * n + i];
            let x = (gx0 + i as i32 * s) as f32;
            let z = (gz0 + j as i32 * s) as f32;
            let y = slab_y(sea, l, level_vox, vert);

            // Top quad.
            quad(
                [[x - half, y, z - half], [x + half, y, z - half], [x - half, y, z + half], [x + half, y, z + half]],
                [0.0, 1.0, 0.0],
                top_color(mat[j * n + i], l),
            );

            // Vertical faces down to each lower neighbour.
            for (di, dj) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || nj < 0 || ni >= n as i32 || nj >= n as i32 {
                    continue;
                }
                let ln = lvl[nj as usize * n + ni as usize];
                if ln >= l {
                    continue;
                }
                let ny = slab_y(sea, ln, level_vox, vert);
                let (q, nrm) = match (di, dj) {
                    (1, 0) => ([[x + half, y, z - half], [x + half, y, z + half], [x + half, ny, z - half], [x + half, ny, z + half]], [1.0, 0.0, 0.0]),
                    (-1, 0) => ([[x - half, y, z - half], [x - half, y, z + half], [x - half, ny, z - half], [x - half, ny, z + half]], [-1.0, 0.0, 0.0]),
                    (0, 1) => ([[x - half, y, z + half], [x + half, y, z + half], [x - half, ny, z + half], [x + half, ny, z + half]], [0.0, 0.0, 1.0]),
                    _ => ([[x - half, y, z - half], [x + half, y, z - half], [x - half, ny, z - half], [x + half, ny, z - half]], [0.0, 0.0, -1.0]),
                };
                quad(q, nrm, side_color);
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

/// Build the translucent water surface over the same footprint: one horizontal quad
/// per cell that holds water (an inland column with depth, or a bed below sea level),
/// at the waterline. A separate mesh with a blended material, so the solid ground bed
/// shows through it. Colour is translucent blue per vertex.
fn build_water_mesh(world: &VivWorld, center: Vec2, radius: f32, level_vox: i32, vert: f32, stride: i32) -> Option<Mesh> {
    let volume = &world.volume;
    let sea = world.sea;
    let s = stride;
    let half = s as f32 * 0.5;
    let gx0 = (((center.x - radius) / s as f32).floor() as i32) * s;
    let gz0 = (((center.y - radius) / s as f32).floor() as i32) * s;
    let n = ((2.0 * radius) as i32 / s) as usize + 2;

    // Same point-mesh construction as the terrain: one vertex per cell at its
    // waterline, triangulated into a continuous surface (not per-cell rectangles).
    let mut wet = vec![false; n * n];
    let mut wy = vec![0.0f32; n * n];
    let mut col = vec![[0.0f32; 4]; n * n];
    for j in 0..n {
        for i in 0..n {
            let x = gx0 + i as i32 * s;
            let z = gz0 + j as i32 * s;
            let bed = volume.surface_height(x, z).unwrap_or(sea);
            let depth = volume.water_depth_voxels(x, z);
            if depth <= 0 && bed >= sea {
                continue; // dry
            }
            wet[j * n + i] = true;
            let wsurf = (bed + depth).max(sea);
            wy[j * n + i] = slab_y(sea, ((wsurf - sea) as f32 / level_vox as f32).round() as i32, level_vox, vert);
            // Depth (real voxels, independent of the vertical exaggeration) → how much
            // light gets through → opacity and how far to darken toward deep-blue.
            let m = 1.0 - (-((wsurf - bed).max(0) as f32) * WATER_ABSORB).exp();
            col[j * n + i] = [
                WATER_SHALLOW[0] + (WATER_DEEP[0] - WATER_SHALLOW[0]) * m,
                WATER_SHALLOW[1] + (WATER_DEEP[1] - WATER_SHALLOW[1]) * m,
                WATER_SHALLOW[2] + (WATER_DEEP[2] - WATER_SHALLOW[2]) * m,
                m.clamp(0.15, 0.92), // alpha = fraction of the bed it occludes
            ];
        }
    }
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n * n);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n * n);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(n * n);
    for j in 0..n {
        for i in 0..n {
            positions.push([(gx0 + i as i32 * s) as f32 + half, wy[j * n + i], (gz0 + j as i32 * s) as f32 + half]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(col[j * n + i]);
        }
    }
    let mut indices: Vec<u32> = Vec::new();
    for j in 0..n - 1 {
        for i in 0..n - 1 {
            let (a, b, c, d) = (j * n + i, j * n + i + 1, (j + 1) * n + i, (j + 1) * n + i + 1);
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

fn focus3(world: &VivWorld, nav: &Navigator) -> Vec3 {
    // Seat on the HIGHEST of the (up to) four voxels the pawn straddles, so its base
    // rests on the tallest one and never clips into a taller neighbour.
    let (x0, z0) = (nav.focus.x.floor() as i32, nav.focus.y.floor() as i32);
    let mut top = i32::MIN;
    for (dx, dz) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        top = top.max(cell_level(world, x0 + dx, z0 + dz, nav.level_vox));
    }
    Vec3::new(nav.focus.x, slab_y(world.sea, top, nav.level_vox, nav.vert), nav.focus.y)
}

/// Solid-ground level for a voxel column — the BED, so the pawn walks the solid
/// voxels (river/lake beds) rather than floating on the water surface.
fn cell_level(world: &VivWorld, x: i32, z: i32, level_vox: i32) -> i32 {
    let bed = world.volume.surface_height(x, z).unwrap_or(world.sea);
    ((bed - world.sea) as f32 / level_vox as f32).round() as i32
}

/// The minimum pitch that keeps the pawn visible: the sight-line from the camera
/// (which sits *behind* the pawn, toward the viewer) must clear the tallest bit of
/// foreground terrain between them. Walk the ground from the pawn toward the camera
/// and take the steepest rise seen; a hill at horizontal distance x and height h
/// above the pawn demands tan(pitch) ≥ (h − h_pawn)/x. Plus a small margin.
fn required_pitch(world: &VivWorld, focus: Vec2, focus_h: f32, yaw: f32, zoom: f32) -> f32 {
    // Probe a fan of rays toward the camera (base angle = yaw + π), not just one, so
    // a broad mountain whose peak is off the direct pawn→camera line is still caught.
    // Reach well past the frustum; distant terrain contributes little (÷ distance).
    let base = yaw + std::f32::consts::PI;
    let max_dist = (zoom * 2.6).max(500.0);
    let step = 8.0;
    let mut max_tan = 0.0f32;
    for da in [-0.28f32, -0.12, 0.0, 0.12, 0.28] {
        let a = base + da;
        let dir = Vec2::new(a.sin(), a.cos());
        let mut x = step;
        while x <= max_dist {
            let q = focus + dir * x;
            let h = world.volume.surface_height(q.x.round() as i32, q.y.round() as i32).unwrap_or(world.sea) as f32;
            max_tan = max_tan.max((h - focus_h) / x);
            x += step;
        }
    }
    max_tan.atan() + 0.06 // clear the tallest occluder, plus ~3.5°
}

fn camera_transform(nav: &Navigator) -> Transform {
    // The eye rides at the current pitch; aim a little ahead so the focus sits low.
    let look = (Vec3::new(nav.yaw.sin(), 0.0, nav.yaw.cos()) * nav.pitch.cos() + Vec3::NEG_Y * nav.pitch.sin()).normalize();
    let forward_h = Vec3::new(nav.yaw.sin(), 0.0, nav.yaw.cos());
    // Aim ahead by frac·zoom/sin(pitch) to hold the pawn at a fixed screen height —
    // sign-preserving clamp so it also works when looking up (negative pitch), where
    // both the aim direction and sin flip and the pawn stays low on screen.
    let s = nav.pitch.sin();
    let denom = if s >= 0.0 { s.max(0.15) } else { s.min(-0.15) };
    let aim = Vec3::new(nav.focus.x, nav.focus_h, nav.focus.y) + forward_h * (nav.zoom * PAWN_BELOW_CENTER / denom);
    let eye = aim - look * STANDOFF;
    Transform::from_translation(eye).looking_at(aim, Vec3::Y)
}

fn navigator_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    world: Res<VivWorld>,
    mut nav: ResMut<Navigator>,
    mut cam: Query<&mut Transform, (With<IsoCamera>, Without<Pawn>)>,
    mut cam_proj: Query<&mut Projection, With<IsoCamera>>,
    mut fog: Query<&mut DistanceFog, With<IsoCamera>>,
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
    // Q/E turn as if rotating the pawn left/right (the world swings the other way).
    let turning = keys.just_pressed(KeyCode::KeyQ) || keys.just_pressed(KeyCode::KeyE);
    if keys.just_pressed(KeyCode::KeyQ) { nav.yaw_target += YAW_STEP; }
    if keys.just_pressed(KeyCode::KeyE) { nav.yaw_target -= YAW_STEP; }
    if keys.just_pressed(KeyCode::KeyP) { nav.points = !nav.points; }
    if (nav.yaw - nav.yaw_target).abs() > 1e-4 {
        nav.yaw = lerp_angle(nav.yaw, nav.yaw_target, (ROT_LERP * dt).clamp(0.0, 1.0));
    }

    // Moving or turning the pawn returns to the default angle and re-enables
    // auto-pitch, so navigating always re-frames the pawn cleanly...
    if dir != Vec2::ZERO || turning {
        nav.pitch_manual = ISO_PITCH;
        nav.auto_pitch = true;
    }
    // ...while K/J are always live: pressing either immediately takes manual control
    // (auto off) — K raises the camera toward top-down, J lowers it all the way to
    // level with the pawn (occlusion and all, by design). Y toggles auto explicitly.
    if keys.pressed(KeyCode::KeyK) {
        nav.auto_pitch = false;
        nav.pitch_manual = (nav.pitch_manual + PITCH_RATE * dt).min(PITCH_MAX);
    }
    if keys.pressed(KeyCode::KeyJ) {
        nav.auto_pitch = false;
        nav.pitch_manual = (nav.pitch_manual - PITCH_RATE * dt).max(PITCH_MIN);
    }
    if keys.just_pressed(KeyCode::KeyY) { nav.auto_pitch = !nav.auto_pitch; }

    nav.focus_h = focus3(&world, &nav).y;

    // Target pitch = your manual angle, raised only as far as auto-pitch needs to
    // keep the pawn from hiding behind the foreground. Smoothed so it never snaps.
    let mut pitch_target = nav.pitch_manual;
    if nav.auto_pitch {
        pitch_target = pitch_target.max(required_pitch(&world, nav.focus, nav.focus_h, nav.yaw, nav.zoom));
    }
    pitch_target = pitch_target.clamp(PITCH_MIN, PITCH_MAX);
    // Rise fast (don't let a mountain hide the pawn during a pan), relax slowly.
    let rate = if pitch_target > nav.pitch { PITCH_LERP_UP } else { PITCH_LERP_DOWN };
    nav.pitch += (pitch_target - nav.pitch) * (rate * dt).clamp(0.0, 1.0);

    if let Ok(mut tf) = cam.single_mut() {
        *tf = camera_transform(&nav);
    }
    if let Ok(mut proj) = cam_proj.single_mut() {
        if let Projection::Orthographic(o) = proj.as_mut() {
            o.scaling_mode = ScalingMode::FixedVertical { viewport_height: nav.zoom };
            // Looking UP (pitch < 0): clip everything nearer than ~the pawn's depth, so
            // the foreground between the pawn and the bottom of the screen can't wall
            // off the pawn or the mountains beyond. Down-looking keeps the near valley.
            o.near = if nav.pitch < 0.0 { STANDOFF - 20.0 } else { -1.0 };
        }
    }
    if let Ok(mut f) = fog.single_mut() {
        // Light haze on the far distance only: clear out to ~the pawn's depth
        // (STANDOFF), then fade gently over the next few zoom-widths toward the sky.
        let start = STANDOFF + nav.zoom * 0.4;
        f.falloff = FogFalloff::Linear { start, end: start + nav.zoom * 4.0 };
    }
    if let Ok(mut tf) = pawn.single_mut() {
        // Lift by half the pawn's height (2 voxels) so its base rests on the slab.
        *tf = Transform::from_translation(focus3(&world, &nav) + Vec3::Y * 2.0);
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut d = (b - a).rem_euclid(std::f32::consts::TAU);
    if d > std::f32::consts::PI {
        d -= std::f32::consts::TAU;
    }
    a + d * t
}

fn maybe_screenshot(time: Res<Time>, mut commands: Commands, mut shot: Local<bool>, mut exit: MessageWriter<AppExit>) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(2.5);
    if t > settle && !*shot {
        let path = PathBuf::from("/tmp/vivarium_slabs_shot.png");
        eprintln!("[slabs] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.5 {
        exit.write(AppExit::Success);
    }
}
