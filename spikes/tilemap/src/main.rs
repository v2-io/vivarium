//! vivarium-tilemap — a square 2D top-down autotiling engine over vivarium-core.
//!
//! The question it answers: can the continuous, erosion-carved geology be mapped
//! into a discrete tile world (RPG-Maker / Zelda-LttP geometry) that reads
//! coherently — clean terraces and cliffs, legible material regions — *before* any
//! art exists? So it renders placeholder tiles: each cell's height is QUANTIZED to
//! a discrete level (the key dial), its material picked from the core, the
//! neighbour/edge config computed (what an autotile selector needs), and cliff
//! faces drawn at the level steps. Real hand-painted tiles are a later per-category
//! swap; the mapping is the thing being proven.
//!
//! Rendering is a CPU framebuffer (fast to iterate placeholders); the art version
//! would move to a sprite/atlas tilemap, but the quantization + autotile logic here
//! is identical either way.

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
const DEFAULT_ZOOM: usize = 3; // 48 px — tiles read as painted art, not noise
/// Cliff-wall height per level step, as a fraction of the tile pixel size.
const LEVEL_RISE_FRAC: f32 = 0.9;

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
    level_vox: i32, // quantization band (voxels per level) — tunable with [ ]
    dirty: bool,
}

impl Default for Navigator {
    fn default() -> Self {
        let fx = std::env::var("VIVARIUM_FOCUS_X").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let fz = std::env::var("VIVARIUM_FOCUS_Z").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        Self {
            focus: Vec2::new(fx, fz),
            zoom: DEFAULT_ZOOM,
            level_vox: std::env::var("VIVARIUM_LEVEL_VOX").ok().and_then(|s| s.parse().ok()).unwrap_or(12),
            dirty: true,
        }
    }
}

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
}

// --- Framebuffer + render --------------------------------------------------------

#[derive(Resource)]
struct Framebuffer(Handle<Image>);

const SKY: [u8; 4] = [196, 206, 214, 255];

/// A decoded RGBA tile (from the embedded 2d-circle-graphic art).
struct Tile {
    w: usize,
    h: usize,
    rgba: Vec<u8>,
}

fn load_tile(bytes: &[u8]) -> Tile {
    let img = image::load_from_memory(bytes).expect("tile decode").to_rgba8();
    Tile { w: img.width() as usize, h: img.height() as usize, rgba: img.into_raw() }
}

/// The hand-painted tiles, one per category (more = richer autotiling later).
#[derive(Resource)]
struct TileArt {
    grass: Tile,
    rock: Tile,
    cliff: Tile,
}

/// Nearest-neighbour blit a tile into the framebuffer, scaled to `dw×dh`, brightness
/// `shade`, optional mirror (`flip` bit0 = horizontal, bit1 = vertical — used to
/// break grid repetition). Transparent texels are skipped.
fn blit_tile(frame: &mut [u8], t: &Tile, dx: i32, dy: i32, dw: i32, dh: i32, shade: f32, flip: u8) {
    if dw <= 0 || dh <= 0 {
        return;
    }
    let x0 = dx.max(0);
    let y0 = dy.max(0);
    let x1 = (dx + dw).min(WIDTH as i32);
    let y1 = (dy + dh).min(HEIGHT as i32);
    for py in y0..y1 {
        let mut sy = (((py - dy) as f32 / dh as f32) * t.h as f32) as usize;
        sy = sy.min(t.h - 1);
        if flip & 2 != 0 {
            sy = t.h - 1 - sy;
        }
        for px in x0..x1 {
            let mut sx = (((px - dx) as f32 / dw as f32) * t.w as f32) as usize;
            sx = sx.min(t.w - 1);
            if flip & 1 != 0 {
                sx = t.w - 1 - sx;
            }
            let si = (sy * t.w + sx) * 4;
            if t.rgba[si + 3] < 8 {
                continue;
            }
            let di = (py as usize * WIDTH + px as usize) * 4;
            frame[di] = scale(t.rgba[si], shade);
            frame[di + 1] = scale(t.rgba[si + 1], shade);
            frame[di + 2] = scale(t.rgba[si + 2], shade);
            frame[di + 3] = 255;
        }
    }
}

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
    commands.insert_resource(TileArt {
        grass: load_tile(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/grass.png"))),
        rock: load_tile(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/rock.png"))),
        cliff: load_tile(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/cliff.png"))),
    });
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

fn rebuild(mut nav: ResMut<Navigator>, world: Res<WorldSrc>, art: Res<TileArt>, fb: Res<Framebuffer>, mut images: ResMut<Assets<Image>>, mut stats: ResMut<RenderStats>) {
    if !nav.dirty {
        return;
    }
    nav.dirty = false;
    let t0 = std::time::Instant::now();

    let tile = ZOOMS[nav.zoom];
    let rise = (tile as f32 * LEVEL_RISE_FRAC) as i32;
    // Focus cell (fractional) → pixel origin in the lower third of the screen.
    let fcx = nav.focus.x / CELL_VOX as f32;
    let fcz = nav.focus.y / CELL_VOX as f32;
    let origin_x = WIDTH as f32 * 0.5;
    let origin_y = HEIGHT as f32 * 0.62;

    // Visible cell rectangle (+ generous margin for tall terrain projecting up).
    let cols = WIDTH as i32 / tile + 4;
    let rows = HEIGHT as i32 / tile + 4;
    let cx0 = fcx.floor() as i32 - cols / 2 - 2;
    let cx1 = fcx.ceil() as i32 + cols / 2 + 2;
    let cz0 = fcz.floor() as i32 - rows / 2 - 2;
    let cz1 = fcz.ceil() as i32 + rows / 2 + 2;

    // Pre-sample the grid (+1 border for neighbour/edge lookups).
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
    // Classified tile type for a cell (from its features) — what selects the tile.
    let cls = |cx: i32, cz: i32| -> u16 {
        classify(at(cx, cz), &[at(cx - 1, cz), at(cx + 1, cz), at(cx, cz - 1), at(cx, cz + 1)])
    };

    let mut frame = vec![0u8; WIDTH * HEIGHT * 4];
    for px in frame.chunks_exact_mut(4) {
        px.copy_from_slice(&SKY);
    }

    // FLAT top-down (LttP-style): every tile on one plane, so the whole screen is
    // terrain with no sky gaps and orientation is trivial. Elevation is shown the
    // top-down way — cliff-face tiles drawn as walls at height steps (below), plus
    // topographic shading — NOT a vertical offset (that left sky gaps over low
    // ground and made the view hard to read).
    let screen = |cx: i32, cz: i32| -> (i32, i32) {
        let sx = origin_x + (cx as f32 - fcx) * tile as f32;
        let sy = origin_y + (cz as f32 - fcz) * tile as f32;
        (sx as i32, sy as i32)
    };

    let mut tiles = 0usize;
    // Painter's order: back (small cz) to front, so nearer/higher tiles overdraw.
    for cz in cz0..=cz1 {
        for cx in cx0..=cx1 {
            let c = at(cx, cz);
            let ty = cls(cx, cz);
            let (sx, sy) = screen(cx, cz);
            if sx + tile < 0 || sx > WIDTH as i32 || sy + tile < 0 || sy > HEIGHT as i32 {
                continue;
            }
            // Shaded relief: a hillshade from the height gradient (light from the
            // NW) makes ridges and valleys read as elevation — the depth that going
            // flat lost — combined with a gentle topographic tint by absolute height.
            let lvl = |dx: i32, dz: i32| at(cx + dx, cz + dz).level as f32;
            let dzdx = lvl(1, 0) - lvl(-1, 0);
            let dzdz = lvl(0, 1) - lvl(0, -1);
            let relief = (-dzdx - dzdz) * 0.11; // NW-lit slopes brighter, SE darker
            let topo = c.level as f32 * 0.006;
            let shade = (0.95 + topo + relief).clamp(0.5, 1.6);
            // Mirror grass/rock pseudo-randomly per cell to break the grid repetition.
            let flip = (((cx.wrapping_mul(73_856_093)) ^ (cz.wrapping_mul(19_349_663))) & 3) as u8;
            // Top tile (flat) — painted art per category; water/sand stay colour.
            match ty {
                3 => {
                    blit_tile(&mut frame, &art.grass, sx, sy, tile, tile, shade, flip);
                    // Autotile blend: feather the rock tile in from any rock
                    // edge-neighbour, so grass↔rock boundaries are smooth, not
                    // hard rectangles. (Programmatic — no per-config tiles.)
                    for (dx, dz, e) in [(0i32, -1i32, 0u8), (1, 0, 1), (0, 1, 2), (-1, 0, 3)] {
                        if cls(cx + dx, cz + dz) == 1 {
                            blit_tile_edge(&mut frame, &art.rock, sx, sy, tile, tile, shade, e, 0.55);
                        }
                    }
                }
                1 => blit_tile(&mut frame, &art.rock, sx, sy, tile, tile, shade, flip),
                2 => fill_rect(&mut frame, sx, sy, tile, tile, [206, 190, 138, 255]), // sand
                _ => fill_rect(&mut frame, sx, sy, tile, tile, tile_color(ty, c.level, &world)), // water
            }
            // Cliff WALL into the top edge of this (lower) cell where the back
            // neighbour is higher — the wall at the base of the higher ground.
            let back = at(cx, cz - 1);
            if back.level > c.level {
                let fh = ((back.level - c.level) * rise).min(tile);
                blit_tile(&mut frame, &art.cliff, sx, sy, tile, fh, shade, 0);
            }
            tiles += 1;
        }
    }

    // Pawn marker at the focus cell (on top), sized to read at any zoom.
    let (px, py) = screen(fcx.round() as i32, fcz.round() as i32);
    let pw = (tile / 3).max(6);
    let ph = (tile / 2).max(9);
    fill_rect(&mut frame, px + tile / 2 - pw / 2, py + tile / 2 - ph, pw, ph, [40, 0, 0, 255]);
    fill_rect(&mut frame, px + tile / 2 - pw / 2 + 1, py + tile / 2 - ph + 1, pw - 2, ph - 2, [225, 45, 45, 255]);

    if let Some(img) = images.get_mut(&fb.0) {
        img.data = Some(frame);
    }
    stats.tiles = tiles;
    stats.rebuild_ms = t0.elapsed().as_secs_f32() * 1000.0;
}

/// Material → placeholder colour, shaded by elevation (atmospheric: higher = paler).
fn tile_color(material: u16, level: i32, world: &WorldSrc) -> [u8; 4] {
    let base = match material {
        1 => [140, 134, 126], // rock / cliff
        2 => [206, 190, 138], // sandy shore
        3 => [88, 132, 64],   // grass
        4 => [70, 110, 160],  // water
        _ => [120, 118, 116],
    };
    let _ = world;
    let f = (1.0 + (level as f32) * 0.04).clamp(0.7, 1.5);
    [scale(base[0], f), scale(base[1], f), scale(base[2], f), 255]
}

#[inline]
fn scale(v: u8, f: f32) -> u8 {
    (v as f32 * f).clamp(0.0, 255.0) as u8
}

/// Alpha-blend a tile into the framebuffer, scaled to `dw×dh`, masked by a
/// directional gradient that is opaque along `edge` (0=N,1=E,2=S,3=W) and fades to
/// 0 over `feather` of the cell — i.e. the higher-priority material *bleeds in* from
/// that side, giving a smooth autotile transition without per-config tiles.
fn blit_tile_edge(frame: &mut [u8], t: &Tile, dx: i32, dy: i32, dw: i32, dh: i32, shade: f32, edge: u8, feather: f32) {
    let x0 = dx.max(0);
    let y0 = dy.max(0);
    let x1 = (dx + dw).min(WIDTH as i32);
    let y1 = (dy + dh).min(HEIGHT as i32);
    for py in y0..y1 {
        let v = (py - dy) as f32 / dh as f32; // 0 top .. 1 bottom
        let sy = ((v * t.h as f32) as usize).min(t.h - 1);
        for px in x0..x1 {
            let u = (px - dx) as f32 / dw as f32; // 0 left .. 1 right
            let a = match edge {
                0 => 1.0 - v / feather,
                2 => 1.0 - (1.0 - v) / feather,
                1 => 1.0 - (1.0 - u) / feather,
                _ => 1.0 - u / feather,
            }
            .clamp(0.0, 1.0);
            if a <= 0.0 {
                continue;
            }
            let sx = ((u * t.w as f32) as usize).min(t.w - 1);
            let si = (sy * t.w + sx) * 4;
            let ta = t.rgba[si + 3] as f32 / 255.0;
            let af = a * ta;
            if af <= 0.0 {
                continue;
            }
            let di = (py as usize * WIDTH + px as usize) * 4;
            for c in 0..3 {
                let src = (t.rgba[si + c] as f32 * shade).clamp(0.0, 255.0);
                frame[di + c] = (frame[di + c] as f32 * (1.0 - af) + src * af) as u8;
            }
            frame[di + 3] = 255;
        }
    }
}

/// Fill an axis-aligned rect into the RGBA framebuffer, clipped, opaque.
fn fill_rect(frame: &mut [u8], x: i32, y: i32, w: i32, h: i32, rgb: [u8; 4]) {
    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + w).min(WIDTH as i32);
    let y1 = (y + h).min(HEIGHT as i32);
    for py in y0..y1 {
        let row = py as usize * WIDTH;
        for px in x0..x1 {
            let i = (row + px as usize) * 4;
            frame[i] = rgb[0];
            frame[i + 1] = rgb[1];
            frame[i + 2] = rgb[2];
            frame[i + 3] = 255;
        }
    }
}

fn update_hud(nav: Res<Navigator>, stats: Res<RenderStats>, diag: Res<bevy::diagnostic::DiagnosticsStore>, mut q: Query<&mut Text, With<HudText>>) {
    let fps = diag.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()).unwrap_or(0.0);
    if let Ok(mut text) = q.single_mut() {
        text.0 = format!(
            "tilemap\nfps {fps:>5.0}\nrebuild {:>5.2} ms\ntiles {:>6}\nzoom {} ({} px)\nband {} vox ({:.1} m/level)",
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
