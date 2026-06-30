//! vivarium-iso-tiles — a memoized-tile isometric navigator over vivarium-core.
//! See SPEC.md. The thesis: per-cell appearance is a pure function of a small
//! discrete key, so it is memoized once and frames are assembled by cheap blits —
//! the reason a 16-year-old Flash engine (neworld) still hasn't been beaten for
//! feel + performance. This reconstructs that on a real eroded vivarium world.

mod frame;
mod iso;
mod nav;
mod tile;
mod world;

use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::window::WindowResolution;

use frame::{make_framebuffer, rebuild_framebuffer, RenderStats, TileCacheRes};
use iso::{DEFAULT_ZOOM, ZOOMS};
use nav::{navigator_input, Navigator};
use tile::TileCache;
use world::WorldSource;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    // Load the world up front (instant when the worldgen cache is warm) so the
    // eprintln cache log is visible and the App starts ready.
    let world = WorldSource::load();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vivarium — iso-tiles navigator".into(),
                resolution: WindowResolution::new(WIDTH as u32, HEIGHT as u32),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(world)
        .insert_resource(TileCacheRes(TileCache::new()))
        .init_resource::<RenderStats>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (navigator_input, rebuild_framebuffer, update_hud, maybe_screenshot).chain(),
        )
        .run();
}

#[derive(Component)]
struct HudText;

fn setup(mut commands: Commands, world: Res<WorldSource>, mut images: ResMut<Assets<Image>>) {
    let fb = make_framebuffer(&mut images, WIDTH, HEIGHT);

    commands.spawn(Camera2d);
    commands.spawn((Sprite::from_image(fb.handle.clone()), Transform::default()));

    // Focus on the region origin, dropped to the ground there.
    let h = world.volume.surface_height(0, 0).unwrap_or(world.sea_level) as f32;
    commands.insert_resource(Navigator::new(Vec2::ZERO, h, DEFAULT_ZOOM));
    commands.insert_resource(fb);

    commands.spawn((
        Text::new("…"),
        TextFont { font_size: 13.0, ..default() },
        TextColor(Color::srgb(0.05, 0.05, 0.08)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(10.0),
            ..default()
        },
        HudText,
    ));

    println!(
        "[iso-tiles] WASD/arrows pan · wheel zoom · Q/E rotate 45° · [ ] Z-slice down/up · \\ clear slice"
    );
}

fn update_hud(
    nav: Res<Navigator>,
    stats: Res<RenderStats>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut q: Query<&mut Text, With<HudText>>,
) {
    let fps = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let z = ZOOMS[nav.zoom];
    let slice = nav.slice_vox.map(|s| format!("{s}")).unwrap_or_else(|| "off".into());
    if let Ok(mut text) = q.single_mut() {
        text.0 = format!(
            "iso-tiles\nfps {fps:>5.0}\nrebuild {:>5.2} ms\ntiles drawn {:>6}\nunique tiles (cache) {:>5}\nhit-rate {:>4.0}%\ncells sampled {:>6}\nzoom {} ({} m cells)  orient {}/8\nslice {slice}",
            stats.rebuild_ms,
            stats.tiles,
            stats.unique,
            stats.hit_rate * 100.0,
            stats.cells_sampled,
            nav.zoom,
            z.stride_vox / 2,
            nav.orientation,
        );
    }
}

/// Headless verification: with VIVARIUM_AUTOSHOT set, optionally re-zoom, wait a
/// beat, screenshot, and quit.
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
        if let Ok(z) = std::env::var("VIVARIUM_ISO_ZOOM_LEVEL") {
            if let Ok(zi) = z.parse::<usize>() {
                nav.zoom = zi.min(ZOOMS.len() - 1);
            }
        }
        nav.dirty = true;
        nav.shape_changed = true;
        *framed = true;
    }
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(2.0);
    if t > settle && !*shot {
        let path = std::path::PathBuf::from("/tmp/vivarium_isotiles_shot.png");
        eprintln!("[iso-tiles] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.0 {
        exit.write(AppExit::Success);
    }
}
