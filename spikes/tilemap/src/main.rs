//! vivarium-tilemap — a top-down *contour map* of vivarium-core's geology.
//!
//! Geometry before art. The earlier pass jumped straight to hand-painted cliff
//! tiles and they read as meaningless rock images, because the *shape* underneath
//! them was never established. This version answers the prior question first: what
//! is the simplest geometric rendering of the continuous, erosion-carved terrain
//! that a player can actually read?
//!
//! The answer (from Joseph's reference, `example-topdown-iso.png`): a **contour
//! map**. Quantize each cell's height to a discrete level; the cliffs are the
//! contour lines *between* levels, extracted with marching squares so every edge is
//! either orthogonal or a 45° chamfer (the minimal smoothing that turns blocky
//! quantized cells into something that reads as continuous terrain — the little
//! cone becomes an octagon, sharp turns become diagonal cuts). A multi-level drop
//! is several nested parallel contours, exactly as a steep slope should look. The
//! bands between contours are filled flat by material, with a gentle hillshade.
//!
//! This contour geometry is the substrate the painted CLIFFVEG art swaps onto
//! *later* — straight/corner cliff-edge tiles map directly onto these segments —
//! and it is already a legible map for agents (the pawns in the reference). The CPU
//! framebuffer keeps iteration fast; the logic is identical under a GPU tilemap.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use vivarium_core::voxel::{Volume, WORLDGEN_VERSION};
use vivarium_core::World;

// --- Worldgen params (shared cache) ----------------------------------------------
const SEED: u64 = 0x00C0_FFEE;
const DETAIL: i32 = 2;
const DEFAULT_REGION_HALF_M: i32 = 6_000;
const EPOCHS: u32 = 70;
const FINE_CELL_M: f32 = 8.0;
const FINE_EPOCHS: u32 = 0;
const N_AGENTS: usize = 24;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

/// Voxels per tile cell — the world footprint of one tile. 6 voxels = 3 m.
const CELL_VOX: i32 = 6;
/// Discrete zoom levels: pixels per tile on screen. The art tiles are ~40 px, so
/// the upper levels show them at/above native size (crisp); the lower ones are the
/// strategic map (shrunk to noise, but useful for overview).
const ZOOMS: &[i32] = &[12, 20, 32, 48, 72, 104];
const DEFAULT_ZOOM: usize = 3; // 48 px — contours read clearly, terrain still broad

fn main() {
    let world = WorldSrc::load();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vivarium — tilemap (top-down autotile engine)".into(),
                resolution: bevy::window::WindowResolution::new(WIDTH as u32, HEIGHT as u32),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(world)
        .insert_resource(Navigator::default())
        .init_resource::<RenderStats>()
        .add_systems(Startup, setup)
        .add_systems(Update, (input, rebuild, update_hud, maybe_screenshot).chain())
        .run();
}

// --- World source (cache) --------------------------------------------------------

#[derive(Resource, Clone)]
struct WorldSrc {
    volume: Arc<Volume>,
    sea: i32,
}

impl WorldSrc {
    fn load() -> Self {
        let region_half = std::env::var("VIVARIUM_REGION_HALF").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_REGION_HALF_M);
        let volume = if std::env::var_os("VIVARIUM_NO_CACHE").is_some() {
            generate(region_half)
        } else {
            let path = cache_path(region_half);
            match std::fs::read(&path).ok().and_then(|b| Volume::from_bytes(&b)) {
                Some(v) => {
                    eprintln!("tilemap: worldgen cache HIT — {}", path.display());
                    v
                }
                None => {
                    let v = generate(region_half);
                    if let Some(p) = path.parent() {
                        let _ = std::fs::create_dir_all(p);
                    }
                    let tmp = path.with_extension("bin.tmp");
                    let _ = std::fs::write(&tmp, v.to_bytes()).and_then(|_| std::fs::rename(&tmp, &path));
                    v
                }
            }
        };
        let sea = volume.sea_level();
        Self { volume: Arc::new(volume), sea }
    }
}

fn generate(region_half: i32) -> Volume {
    eprintln!("tilemap: generating eroded world (±{region_half} m)… slow tier, minutes.");
    World::eroded(SEED, N_AGENTS, DETAIL, region_half, EPOCHS, FINE_CELL_M, FINE_EPOCHS).volume
}

fn cache_path(region_half: i32) -> PathBuf {
    let dir = std::env::var_os("VIVARIUM_CACHE_DIR").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("vivarium-worldcache"));
    dir.join(format!("viv_s{SEED:x}_d{DETAIL}_r{region_half}_e{EPOCHS}_fc{FINE_CELL_M:.1}_fe{FINE_EPOCHS}_v{WORLDGEN_VERSION}.bin"))
}

// --- One sampled tile cell -------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    /// Discrete elevation level (relative to sea, in bands). THE quantization.
    level: i32,
    /// Material id (vivarium-core Voxel; 4 = water).
    material: u16,
}

fn sample_cell(world: &WorldSrc, cx: i32, cz: i32, level_vox: i32) -> Cell {
    let vx = cx * CELL_VOX + CELL_VOX / 2;
    let vz = cz * CELL_VOX + CELL_VOX / 2;
    let bed = world.volume.surface_height(vx, vz).unwrap_or(world.sea);
    let depth = world.volume.water_depth_voxels(vx, vz);
    // Underwater (inland water OR open ocean beyond the patch, where the bed is
    // below sea level) renders as flat sea at the waterline. This is what turns the
    // coastline into land-meets-sea instead of land-meets-sky.
    let (surf, material) = if depth > 0 || bed < world.sea {
        ((bed + depth).max(world.sea), 4u16)
    } else {
        (bed, world.volume.voxel(vx, bed, vz).0)
    };
    let level = ((surf - world.sea) as f32 / level_vox as f32).round() as i32;
    Cell { level, material }
}

/// Classify a cell into a *tile type* from terrain features — the autotile decision
/// the core's flat "all grass" surface can't give. Water stays water; steep cells
/// (a big level step to a neighbour) become rock/cliff; grass next to water becomes
/// a sandy shore; everything else is grass. This is the rule a real autotile set
/// would key on (and where snow/desert/etc. would slot in later).
fn classify(c: Cell, n: &[Cell; 4]) -> u16 {
    if c.material == 4 {
        return 4; // water
    }
    let steep = n.iter().map(|x| (c.level - x.level).abs()).max().unwrap_or(0);
    if steep >= 2 {
        return 1; // rock / cliff
    }
    if n.iter().any(|x| x.material == 4) {
        return 2; // sandy shore (dirt)
    }
    3 // grass
}

// --- Navigator -------------------------------------------------------------------

#[derive(Resource)]
struct Navigator {
    focus: Vec2, // world voxels (x, z)
    zoom: usize,
    level_vox: i32,  // quantization band (voxels per level) — tunable with [ ]
    iso: bool,       // false = flat top-down contour map; true = 2.5D oblique lift
    rise_frac: f32,  // iso lift strength (tile fraction per level) — tunable with , .
    dirty: bool,
}

impl Default for Navigator {
    fn default() -> Self {
        let fx = std::env::var("VIVARIUM_FOCUS_X").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let fz = std::env::var("VIVARIUM_FOCUS_Z").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        Self {
            focus: Vec2::new(fx, fz),
            zoom: std::env::var("VIVARIUM_ZOOM").ok().and_then(|s| s.parse().ok()).filter(|&z| z < ZOOMS.len()).unwrap_or(DEFAULT_ZOOM),
            level_vox: std::env::var("VIVARIUM_LEVEL_VOX").ok().and_then(|s| s.parse().ok()).unwrap_or(12),
            iso: std::env::var("VIVARIUM_ISO").map(|v| v != "0").unwrap_or(true),
            rise_frac: std::env::var("VIVARIUM_RISE").ok().and_then(|s| s.parse().ok()).unwrap_or(RISE_FRAC_DEFAULT),
            dirty: true,
        }
    }
}

/// Vertical screen rise per elevation level, as a fraction of the tile size — the
/// strength of the 2.5D lift. High ground shifts up-screen by this much per level
/// and the gap below its south edge is filled by the cliff face. Tunable with , / .
const RISE_FRAC_DEFAULT: f32 = 0.85;

const PAN_CELLS_PER_SEC: f32 = 14.0;

fn input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    mut nav: ResMut<Navigator>,
) {
    let dt = time.delta_secs();
    let mut d = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        d.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        d.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        d.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        d.x -= 1.0;
    }
    if d != Vec2::ZERO {
        nav.focus += d.normalize() * (PAN_CELLS_PER_SEC * CELL_VOX as f32 * dt);
        nav.dirty = true;
    }
    for ev in wheel.read() {
        let z = nav.zoom;
        if ev.y > 0.0 && nav.zoom + 1 < ZOOMS.len() {
            nav.zoom += 1;
        } else if ev.y < 0.0 && nav.zoom > 0 {
            nav.zoom -= 1;
        }
        if nav.zoom != z {
            nav.dirty = true;
        }
    }
    // Quantization band: [ coarser terraces, ] finer.
    if keys.just_pressed(KeyCode::BracketLeft) {
        nav.level_vox = (nav.level_vox + 2).min(60);
        nav.dirty = true;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        nav.level_vox = (nav.level_vox - 2).max(2);
        nav.dirty = true;
    }
    // Tab: flat top-down ↔ 2.5D iso lift. , / . : lower / raise the lift strength.
    if keys.just_pressed(KeyCode::Tab) {
        nav.iso = !nav.iso;
        nav.dirty = true;
    }
    if keys.just_pressed(KeyCode::Comma) {
        nav.rise_frac = (nav.rise_frac - 0.1).max(0.0);
        nav.dirty = true;
    }
    if keys.just_pressed(KeyCode::Period) {
        nav.rise_frac = (nav.rise_frac + 0.1).min(2.0);
        nav.dirty = true;
    }
}

// --- Framebuffer + render --------------------------------------------------------

#[derive(Resource)]
struct Framebuffer(Handle<Image>);

const SKY: [u8; 4] = [196, 206, 214, 255];

#[derive(Resource, Default)]
struct RenderStats {
    tiles: usize,
    rebuild_ms: f32,
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let img = Image::new_fill(
        Extent3d { width: WIDTH as u32, height: HEIGHT as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &SKY,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    let handle = images.add(img);
    commands.spawn(Camera2d);
    commands.spawn((Sprite::from_image(handle.clone()), Transform::default()));
    commands.insert_resource(Framebuffer(handle));
    commands.spawn((
        Text::new("…"),
        TextFont { font_size: 13.0, ..default() },
        TextColor(Color::srgb(0.05, 0.05, 0.08)),
        Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(10.0), ..default() },
        HudText,
    ));
    println!("[tilemap] WASD pan · wheel zoom · [ ] coarser/finer height bands");
}

#[derive(Component)]
struct HudText;

fn rebuild(mut nav: ResMut<Navigator>, world: Res<WorldSrc>, fb: Res<Framebuffer>, mut images: ResMut<Assets<Image>>, mut stats: ResMut<RenderStats>) {
    if !nav.dirty {
        return;
    }
    nav.dirty = false;
    let t0 = std::time::Instant::now();

    let tile = ZOOMS[nav.zoom];
    // Focus cell (fractional) → pixel origin, focus held in the lower third.
    let fcx = nav.focus.x / CELL_VOX as f32;
    let fcz = nav.focus.y / CELL_VOX as f32;
    let origin_x = WIDTH as f32 * 0.5;
    let origin_y = HEIGHT as f32 * 0.62;

    // Visible cell rectangle (+ margin so contours/faces reaching in from just
    // off-screen are still drawn). In iso mode ground north of the pawn is lifted
    // and can rise into the top of the screen, so sample extra rows north of it.
    let cols = WIDTH as i32 / tile + 4;
    let rows = HEIGHT as i32 / tile + 4;
    let north_pad = if nav.iso { rows } else { 0 };
    let cx0 = fcx.floor() as i32 - cols / 2 - 2;
    let cx1 = fcx.ceil() as i32 + cols / 2 + 2;
    let cz0 = fcz.floor() as i32 - rows / 2 - 2 - north_pad;
    let cz1 = fcz.ceil() as i32 + rows / 2 + 2;

    // Pre-sample the grid (+1 border for neighbour lookups and the marching-
    // squares cell that hangs off the far edge).
    let gw = (cx1 - cx0 + 3) as usize;
    let gh = (cz1 - cz0 + 3) as usize;
    let mut grid = Vec::with_capacity(gw * gh);
    for j in 0..gh as i32 {
        for i in 0..gw as i32 {
            grid.push(sample_cell(&world, cx0 - 1 + i, cz0 - 1 + j, nav.level_vox));
        }
    }
    let at = |cx: i32, cz: i32| -> Cell {
        let i = (cx.clamp(cx0 - 1, cx1 + 1) - (cx0 - 1)) as usize;
        let j = (cz.clamp(cz0 - 1, cz1 + 1) - (cz0 - 1)) as usize;
        grid[j * gw + i]
    };
    let cls = |cx: i32, cz: i32| -> u16 {
        classify(at(cx, cz), &[at(cx - 1, cz), at(cx + 1, cz), at(cx, cz - 1), at(cx, cz + 1)])
    };
    // Anchor the iso lift to the pawn's own cell so the pawn stays fixed on screen
    // as you pan across hills — anchoring to the lowest nearby level (as before) let
    // its height float and pushed it off-screen over tall terrain.
    let pawn_level = at(fcx.round() as i32, fcz.round() as i32).level;

    let mut frame = vec![0u8; WIDTH * HEIGHT * 4];
    for px in frame.chunks_exact_mut(4) {
        px.copy_from_slice(&SKY);
    }
    // Depth buffer for the iso lift: each filled pixel remembers its screen-nearness
    // (its row cz — further south is nearer to the viewer), so a fill only draws
    // where it is nearer than what's already there. This makes occlusion correct
    // regardless of draw order — a nearer *lower* terrace hides a farther *higher*
    // face, which draw-order-by-level alone could not do. Lines and the pawn are
    // drawn on top (they pass i32::MAX and ignore the test).
    let mut depth = vec![i32::MIN; WIDTH * HEIGHT];

    // Screen-space top-left pixel of a cell. Flat plane: no vertical offset for
    // height (that left sky gaps and broke orientation); elevation is carried
    // entirely by the contour lines + hillshade.
    let screen = |cx: i32, cz: i32| -> (i32, i32) {
        let sx = origin_x + (cx as f32 - fcx) * tile as f32;
        let sy = origin_y + (cz as f32 - fcz) * tile as f32;
        (sx as i32, sy as i32)
    };

    // Cell top shade (shared by both modes): NW-lit hillshade + gentle topo tint.
    let shade_at = |cx: i32, cz: i32, level: i32| -> f32 {
        let lvl = |dx: i32, dz: i32| at(cx + dx, cz + dz).level as f32;
        let relief = (-(lvl(1, 0) - lvl(-1, 0)) - (lvl(0, 1) - lvl(0, -1))) * 0.06;
        (1.0 + level as f32 * 0.012 + relief).clamp(0.78, 1.28)
    };

    let mut cells = 0usize;
    let th = if tile >= 28 { 2 } else { 1 };

    if nav.iso {
        // --- Stacked contour terraces (Joseph's layer lift) --------------------
        // Each elevation level is a plane, shifted up-screen by (level − pawn_level)
        // · rise — anchored to the PAWN's own terrace so the pawn stays fixed and
        // the land rises around it. Per level, bottom-up: draw the {==L} tread (each
        // cell once, at its height, chamfered by the same marching squares as the
        // contour), then the viewer-facing faces (the {≥L} boundary smeared down one
        // level, flat-shaded by facing; white underwater). Occlusion is handled by
        // the depth buffer (nearer row wins), not draw order.
        let rise = (tile as f32 * nav.rise_frac).round().max(1.0) as i32;
        let mut lmin = i32::MAX;
        let mut lmax = i32::MIN;
        for c in &grid {
            lmin = lmin.min(c.level);
            lmax = lmax.max(c.level);
        }
        // Base: a magenta SENTINEL under everything, at the lowest priority. If the
        // lifted terraces tile watertight it is fully overdrawn and never seen; any
        // magenta on screen is therefore a coverage bug (a hole the terraces left)
        // surfaced loudly rather than hidden behind a plausible-looking base colour.
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                let (sx, sy) = screen(cx, cz);
                if sx + tile < 0 || sx > WIDTH as i32 || sy + tile < 0 || sy > HEIGHT as i32 {
                    continue;
                }
                fill_rect(&mut frame, &mut depth, i32::MIN / 2 + cz, sx, sy, tile, tile, GAP_SENTINEL);
            }
        }
        for l in lmin..=lmax {
            let shift = (l - pawn_level) * rise;
            let tint = (1.0 + (l - lmin) as f32 * 0.02).min(1.4);
            // Tread — chamfered with the same marching squares as the contour.
            for cz in cz0 - 1..=cz1 {
                for cx in cx0 - 1..=cx1 {
                    let ox = (origin_x + (cx as f32 - fcx) * tile as f32) as i32 + tile / 2;
                    let oy = (origin_y + (cz as f32 - fcz) * tile as f32) as i32 + tile / 2 - shift;
                    if ox + tile < 0 || ox > WIDTH as i32 || oy + tile < 0 || oy > HEIGHT as i32 {
                        continue;
                    }
                    let m = |cx: i32, cz: i32| (at(cx, cz).level == l) as i32;
                    let (m00, m10, m01, m11) = (m(cx, cz), m(cx + 1, cz), m(cx, cz + 1), m(cx + 1, cz + 1));
                    if m00 + m10 + m01 + m11 == 0 {
                        continue;
                    }
                    let base = band_color(cls(cx, cz), at(cx, cz).material);
                    fill_march(&mut frame, &mut depth, cz, ox, oy, tile, m00, m10, m01, m11, [scale(base[0], tint), scale(base[1], tint), scale(base[2], tint)]);
                    if m00 == 1 {
                        cells += 1;
                    }
                }
            }
            if l == lmin {
                continue; // nothing steps down below the lowest layer
            }
            // Faces + contour lines — the {level ≥ L} boundary smeared down one level.
            for cz in cz0 - 1..=cz1 {
                for cx in cx0 - 1..=cx1 {
                    let ox = (origin_x + (cx as f32 - fcx) * tile as f32) as i32 + tile / 2;
                    let oy = (origin_y + (cz as f32 - fcz) * tile as f32) as i32 + tile / 2 - shift;
                    if ox + tile < 0 || ox > WIDTH as i32 || oy + tile < 0 || oy > HEIGHT as i32 {
                        continue;
                    }
                    let m = |cx: i32, cz: i32| (at(cx, cz).level >= l) as i32;
                    let water = at(cx, cz).material == 4 || at(cx + 1, cz).material == 4 || at(cx, cz + 1).material == 4 || at(cx + 1, cz + 1).material == 4;
                    let base_face = if water { WATER_FACE } else { ROCK };
                    marching_face(&mut frame, &mut depth, cz, ox, oy, tile, m(cx, cz), m(cx + 1, cz), m(cx, cz + 1), m(cx + 1, cz + 1), rise, base_face, CONTOUR, th);
                }
            }
        }
        // Pawn at the focus — anchored, so it sits at the fixed screen origin.
        let (px, py) = screen(fcx.round() as i32, fcz.round() as i32);
        draw_pawn(&mut frame, &mut depth, px, py, tile);
    } else {
        // --- Flat top-down contour map ----------------------------------------
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                let c = at(cx, cz);
                let (sx, sy) = screen(cx, cz);
                if sx + tile < 0 || sx > WIDTH as i32 || sy + tile < 0 || sy > HEIGHT as i32 {
                    continue;
                }
                let base = band_color(cls(cx, cz), c.material);
                let sh = shade_at(cx, cz, c.level);
                fill_rect(&mut frame, &mut depth, i32::MAX, sx, sy, tile, tile, [scale(base[0], sh), scale(base[1], sh), scale(base[2], sh), 255]);
                cells += 1;
            }
        }
        // Marching-squares contours: the cliffs (level field) + the shoreline
        // (is-water field). Segments are only ever orthogonal or 45°.
        for cz in cz0 - 1..=cz1 {
            for cx in cx0 - 1..=cx1 {
                let (sx, sy) = screen(cx, cz);
                let ox = sx + tile / 2;
                let oy = sy + tile / 2;
                if ox + tile < 0 || ox > WIDTH as i32 || oy + tile < 0 || oy > HEIGHT as i32 {
                    continue;
                }
                let a = at(cx, cz);
                let b = at(cx + 1, cz);
                let cc = at(cx, cz + 1);
                let d = at(cx + 1, cz + 1);
                marching_square(&mut frame, &mut depth, i32::MAX, ox, oy, tile, a.level, b.level, cc.level, d.level, CONTOUR, th);
                let w = |c: Cell| (c.material == 4) as i32;
                marching_square(&mut frame, &mut depth, i32::MAX, ox, oy, tile, w(a), w(b), w(cc), w(d), SHORE, th);
            }
        }
        let (px, py) = screen(fcx.round() as i32, fcz.round() as i32);
        draw_pawn(&mut frame, &mut depth, px, py, tile);
    }

    if let Some(img) = images.get_mut(&fb.0) {
        img.data = Some(frame);
    }
    stats.tiles = cells;
    stats.rebuild_ms = t0.elapsed().as_secs_f32() * 1000.0;
}

/// Under-everything sentinel: magenta so any coverage hole the lifted terraces
/// leave is impossible to miss (it should be fully overdrawn in a correct frame).
const GAP_SENTINEL: [u8; 4] = [255, 0, 255, 255];
/// Contour-line colour for elevation steps (cliffs) and the shoreline.
const CONTOUR: [u8; 3] = [54, 48, 46];
const SHORE: [u8; 3] = [40, 70, 104];
/// Cliff-face base colours (rock, and white for underwater steps). Each face is
/// flat-shaded by the *direction it faces* — a S wall, a SE wall and a SW wall get
/// distinct flat tones — like the shaded contour bands in the reference png.
const ROCK: [u8; 3] = [156, 148, 132];
const WATER_FACE: [u8; 3] = [226, 236, 248];
/// Light direction (in the ground plane, y = south) for the facing shade. Faces
/// looking toward it are brighter; roughly a low sun from the south-east.
const LIGHT: (f32, f32) = (0.55, 0.84);
/// Reading aids that make the geology legible (all requested by Joseph):
/// A — a line along the foot of every visible face (base of the cliff).
/// C — dark verticals down the left/right sides of each face.
/// B — a lime line for a contour whose drop faces *away* from the viewer (a
///     hidden cliff behind the terrace), so it isn't read as a flat edge.
const FACE_FOOT: [u8; 3] = [58, 52, 48];
const FACE_SIDE: [u8; 3] = [92, 86, 78];
const HIDDEN_CLIFF: [u8; 3] = [228, 250, 206];

/// Emit the marching-squares contour segment(s) for one square of the level
/// field, for *every* integer threshold the square straddles — so a multi-level
/// drop draws as nested parallel lines. Corner cells `a,b,c,d` are TL, TR, BL, BR;
/// `(ox,oy)` is the TL corner (cell-centre of `a`) in screen pixels, side `t`.
#[allow(clippy::too_many_arguments)]
fn marching_square(frame: &mut [u8], depth: &mut [i32], prio: i32, ox: i32, oy: i32, t: i32, a: i32, b: i32, c: i32, d: i32, rgb: [u8; 3], th: i32) {
    let lo = a.min(b).min(c).min(d);
    let hi = a.max(b).max(c).max(d);
    // Edge midpoints (where a contour crosses): top, bottom, left, right.
    let mt = (ox + t / 2, oy);
    let mb = (ox + t / 2, oy + t);
    let ml = (ox, oy + t / 2);
    let mr = (ox + t, oy + t / 2);
    let mut seg = |p, q| draw_line(frame, depth, prio, p, q, rgb, th);
    for thr in (lo + 1)..=hi {
        // bit0=TL bit1=TR bit2=BL bit3=BR, set when that corner is >= threshold.
        let code = (a >= thr) as u8 | (((b >= thr) as u8) << 1) | (((c >= thr) as u8) << 2) | (((d >= thr) as u8) << 3);
        match code {
            1 | 14 => seg(mt, ml),  // TL corner cut (45°)
            2 | 13 => seg(mt, mr),  // TR corner cut
            4 | 11 => seg(ml, mb),  // BL corner cut
            8 | 7 => seg(mr, mb),   // BR corner cut
            3 | 12 => seg(ml, mr),  // horizontal split
            5 | 10 => seg(mt, mb),  // vertical split
            6 => {
                seg(mt, mr);
                seg(ml, mb);
            }
            9 => {
                seg(mt, ml);
                seg(mr, mb);
            }
            _ => {} // 0 / 15: square wholly above or below this threshold
        }
    }
}

/// One 0/1-mask marching square, rendered as a *lifted terrace edge*: its chamfered
/// boundary segment(s), and — where the low (mask==0) side faces the viewer (south,
/// +y) — a vertical FACE made by smearing that segment straight down `rise` pixels
/// (Joseph's cheat: move the contour down a pixel at a time and the union is the
/// wall). The wall follows the chamfer exactly because it *is* the contour, and it
/// is only one level tall, so nested contours stack into taller cliffs instead of
/// any single cell streaking to the floor. `a,b,c,d` are the TL,TR,BL,BR mask bits.
#[allow(clippy::too_many_arguments)]
fn marching_face(frame: &mut [u8], depth: &mut [i32], prio: i32, ox: i32, oy: i32, t: i32, a: i32, b: i32, c: i32, d: i32, rise: i32, base: [u8; 3], line: [u8; 3], th: i32) {
    let mt = (ox + t / 2, oy);
    let mb = (ox + t / 2, oy + t);
    let ml = (ox, oy + t / 2);
    let mr = (ox + t, oy + t / 2);
    // Direction the wall faces = from the square's centre toward the low (mask==0)
    // side. Its y-component decides visibility (a north-facing wall is hidden); the
    // full direction, dotted with the light, gives ONE flat shade for the whole
    // face — so a S wall, a SE wall and a SW wall read as distinct flat tones.
    let (mut lx, mut ly, mut n) = (0.0f32, 0.0f32, 0.0f32);
    for (v, (px, py)) in [(a, (0.0, 0.0)), (b, (1.0, 0.0)), (c, (0.0, 1.0)), (d, (1.0, 1.0))] {
        if v == 0 {
            lx += px;
            ly += py;
            n += 1.0;
        }
    }
    if n == 0.0 {
        return; // wholly inside: no boundary here
    }
    let (dx, dy) = (lx / n - 0.5, ly / n - 0.5);
    let len = (dx * dx + dy * dy).sqrt().max(1e-3);
    let (nx, ny) = (dx / len, dy / len);
    let lit = (0.62 + 0.5 * (nx * LIGHT.0 + ny * LIGHT.1)).clamp(0.42, 1.15);
    let fc = [scale(base[0], lit), scale(base[1], lit), scale(base[2], lit)];
    let south = ny > 0.05; // wall faces the viewer → visible
    let code = (a as u8) | ((b as u8) << 1) | ((c as u8) << 2) | ((d as u8) << 3);
    let segs: [Option<((i32, i32), (i32, i32))>; 2] = match code {
        1 | 14 => [Some((mt, ml)), None],
        2 | 13 => [Some((mt, mr)), None],
        4 | 11 => [Some((ml, mb)), None],
        8 | 7 => [Some((mr, mb)), None],
        3 | 12 => [Some((ml, mr)), None],
        5 | 10 => [Some((mt, mb)), None],
        6 => [Some((mt, mr)), Some((ml, mb))],
        9 => [Some((mt, ml)), Some((mr, mb))],
        _ => [None, None],
    };
    for s in segs.into_iter().flatten() {
        let (p, q) = s;
        if south {
            // Vertical face: smear the segment straight down `rise` px, flat-shaded.
            // Depth-tested (prio) so a nearer terrace hides it; its crown/side/foot
            // aids share that depth so they hide with it.
            for k in 1..=rise {
                draw_line(frame, depth, prio, (p.0, p.1 + k), (q.0, q.1 + k), fc, th);
            }
            // C: dark verticals down the left/right sides of the face.
            draw_line(frame, depth, prio, p, (p.0, p.1 + rise), FACE_SIDE, th);
            draw_line(frame, depth, prio, q, (q.0, q.1 + rise), FACE_SIDE, th);
            // A: line along the foot of the face.
            draw_line(frame, depth, prio, (p.0, p.1 + rise), (q.0, q.1 + rise), FACE_FOOT, th);
            // Crown line on top.
            draw_line(frame, depth, prio, p, q, line, th);
        } else if ny < -0.05 {
            // B: the drop faces away from the viewer — a hidden cliff. Drawn on top
            // (i32::MAX) so it stays visible through nearer terrain, as a wayfinding
            // marker for cliffs you can't otherwise see.
            draw_line(frame, depth, i32::MAX, p, q, HIDDEN_CLIFF, th);
        } else {
            draw_line(frame, depth, prio, p, q, line, th);
        }
    }
}

/// Bresenham line with a square pen of side `th`, clipped, depth-tested against
/// `prio` (pass i32::MAX to always draw on top).
fn draw_line(frame: &mut [u8], depth: &mut [i32], prio: i32, p: (i32, i32), q: (i32, i32), rgb: [u8; 3], th: i32) {
    let (mut x, mut y) = p;
    let (dx, dy) = ((q.0 - x).abs(), -(q.1 - y).abs());
    let (sx, sy) = (if x < q.0 { 1 } else { -1 }, if y < q.1 { 1 } else { -1 });
    let mut err = dx + dy;
    let r = th / 2;
    loop {
        for yy in (y - r)..=(y - r + th - 1) {
            for xx in (x - r)..=(x - r + th - 1) {
                if xx >= 0 && yy >= 0 && (xx as usize) < WIDTH && (yy as usize) < HEIGHT {
                    let idx = yy as usize * WIDTH + xx as usize;
                    if prio < depth[idx] {
                        continue;
                    }
                    depth[idx] = prio;
                    let i = idx * 4;
                    frame[i] = rgb[0];
                    frame[i + 1] = rgb[1];
                    frame[i + 2] = rgb[2];
                    frame[i + 3] = 255;
                }
            }
        }
        if x == q.0 && y == q.1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// Flat band colour by classified tile type — the fill between contour lines.
/// A light, map-like palette so the dark contour cliffs read crisply on top of it.
fn band_color(ty: u16, _material: u16) -> [u8; 3] {
    match ty {
        1 => [150, 140, 128], // rock / cliff bands
        2 => [214, 200, 150], // sandy shore
        4 => [86, 124, 168],  // water
        _ => [120, 156, 92],  // grass
    }
}

#[inline]
fn scale(v: u8, f: f32) -> u8 {
    (v as f32 * f).clamp(0.0, 255.0) as u8
}

/// Fill an axis-aligned rect into the RGBA framebuffer, clipped, opaque, depth-
/// tested: a pixel is written only where `prio` is at least the stored depth.
fn fill_rect(frame: &mut [u8], depth: &mut [i32], prio: i32, x: i32, y: i32, w: i32, h: i32, rgb: [u8; 4]) {
    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + w).min(WIDTH as i32);
    let y1 = (y + h).min(HEIGHT as i32);
    for py in y0..y1 {
        let row = py as usize * WIDTH;
        for px in x0..x1 {
            let idx = row + px as usize;
            if prio < depth[idx] {
                continue;
            }
            depth[idx] = prio;
            let i = idx * 4;
            frame[i] = rgb[0];
            frame[i + 1] = rgb[1];
            frame[i + 2] = rgb[2];
            frame[i + 3] = 255;
        }
    }
}

/// Scanline-fill a convex polygon (≤6 vertices) into the framebuffer, clipped.
/// Convexity means each scanline meets the boundary in exactly one span, so the
/// min/max x of the edge crossings is the fill extent.
fn fill_convex(frame: &mut [u8], depth: &mut [i32], prio: i32, pts: &[(i32, i32)], rgb: [u8; 3]) {
    if pts.len() < 3 {
        return;
    }
    let ymin = pts.iter().map(|p| p.1).min().unwrap().max(0);
    let ymax = pts.iter().map(|p| p.1).max().unwrap().min(HEIGHT as i32 - 1);
    for y in ymin..=ymax {
        let (mut xlo, mut xhi) = (i32::MAX, i32::MIN);
        for i in 0..pts.len() {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[(i + 1) % pts.len()];
            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let x = x0 + (x1 - x0) * (y - y0) / (y1 - y0);
                xlo = xlo.min(x);
                xhi = xhi.max(x);
            }
        }
        if xlo > xhi {
            continue;
        }
        let row = y as usize * WIDTH;
        for x in xlo.max(0)..=xhi.min(WIDTH as i32 - 1) {
            let idx = row + x as usize;
            if prio < depth[idx] {
                continue;
            }
            depth[idx] = prio;
            let i = idx * 4;
            frame[i] = rgb[0];
            frame[i + 1] = rgb[1];
            frame[i + 2] = rgb[2];
            frame[i + 3] = 255;
        }
    }
}

/// Fill the inside (mask==1) region of one 0/1 marching square with `rgb` — the
/// chamfered counterpart of a cell fill, using the *same* segments as the contour
/// line so fill and outline coincide (no square-cell corners poking past the 45°
/// chamfer). `a,b,c,d` are the TL,TR,BL,BR mask bits; `(ox,oy)` the TL corner.
#[allow(clippy::too_many_arguments)]
fn fill_march(frame: &mut [u8], depth: &mut [i32], prio: i32, ox: i32, oy: i32, t: i32, a: i32, b: i32, c: i32, d: i32, rgb: [u8; 3]) {
    let (tl, tr, bl, br) = ((ox, oy), (ox + t, oy), (ox, oy + t), (ox + t, oy + t));
    let (mt, mb, ml, mr) = ((ox + t / 2, oy), (ox + t / 2, oy + t), (ox, oy + t / 2), (ox + t, oy + t / 2));
    let mut fill = |pts: &[(i32, i32)]| fill_convex(frame, depth, prio, pts, rgb);
    match (a as u8) | ((b as u8) << 1) | ((c as u8) << 2) | ((d as u8) << 3) {
        0 => {}
        15 => fill(&[tl, tr, br, bl]),
        1 => fill(&[tl, mt, ml]),
        2 => fill(&[mt, tr, mr]),
        4 => fill(&[bl, ml, mb]),
        8 => fill(&[br, mr, mb]),
        3 => fill(&[tl, tr, mr, ml]),
        5 => fill(&[tl, mt, mb, bl]),
        12 => fill(&[bl, ml, mr, br]),
        10 => fill(&[mt, tr, br, mb]),
        7 => fill(&[tl, tr, mr, mb, bl]),
        11 => fill(&[tl, tr, br, mb, ml]),
        13 => fill(&[tl, mt, mr, br, bl]),
        14 => fill(&[mt, tr, br, bl, ml]),
        6 => {
            fill(&[mt, tr, mr]);
            fill(&[bl, ml, mb]);
        }
        9 => {
            fill(&[tl, mt, ml]);
            fill(&[br, mr, mb]);
        }
        _ => {}
    }
}

/// The focus pawn — a small dark-outlined red marker, sized to read at any zoom.
/// `(x,y)` is the top-left of its (already lifted) cell.
fn draw_pawn(frame: &mut [u8], depth: &mut [i32], x: i32, y: i32, tile: i32) {
    let pw = (tile / 3).max(6);
    let ph = (tile / 2).max(9);
    fill_rect(frame, depth, i32::MAX, x + tile / 2 - pw / 2, y + tile / 2 - ph, pw, ph, [40, 0, 0, 255]);
    fill_rect(frame, depth, i32::MAX, x + tile / 2 - pw / 2 + 1, y + tile / 2 - ph + 1, pw - 2, ph - 2, [225, 45, 45, 255]);
}

fn update_hud(nav: Res<Navigator>, stats: Res<RenderStats>, diag: Res<bevy::diagnostic::DiagnosticsStore>, mut q: Query<&mut Text, With<HudText>>) {
    let fps = diag.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()).unwrap_or(0.0);
    if let Ok(mut text) = q.single_mut() {
        let mode = if nav.iso { format!("iso  rise {:.2}", nav.rise_frac) } else { "flat".to_string() };
        text.0 = format!(
            "tilemap · {mode}  [Tab]\nfps {fps:>5.0}\nrebuild {:>5.2} ms\ntiles {:>6}\nzoom {} ({} px)\nband {} vox ({:.1} m/level)",
            stats.rebuild_ms,
            stats.tiles,
            nav.zoom,
            ZOOMS[nav.zoom],
            nav.level_vox,
            nav.level_vox as f32 / DETAIL as f32,
        );
    }
}

fn maybe_screenshot(time: Res<Time>, mut commands: Commands, mut shot: Local<bool>, mut exit: MessageWriter<AppExit>) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(2.0);
    if t > settle && !*shot {
        let path = PathBuf::from("/tmp/vivarium_tilemap_shot.png");
        eprintln!("[tilemap] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.0 {
        exit.write(AppExit::Success);
    }
}
