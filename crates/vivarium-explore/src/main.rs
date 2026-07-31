//! `vivarium-explore` — the principled 3D explorer.
//!
//! **What it is for.** Not a display of results: an *instrument for detecting
//! missing physics*, whose detector is a trained human visual system. Joseph:
//! *"my brain is very highly tuned to be able to notice if there's something in
//! the visual evolution that does not seem 'natural' … it was one of the fastest
//! ways to reveal missing physics — when something is acting contrary to what my
//! mind has hundreds of thousands of hours of remembered patterns of."*
//!
//! Three consequences run through the whole design:
//!
//! 1. **Motion is resolution.** The detector runs on continuity, so playback
//!    density is an epistemic requirement, not polish — and it is bought by
//!    evaluating the cooling law at more mantle temperatures, never by
//!    interpolating between materialized stages. A tween would produce smooth
//!    motion a pattern-matcher reads as physics when it is a renderer's guess:
//!    the worst possible failure for an instrument whose value is that what you
//!    see can be trusted. See `lens::Ladder`.
//! 2. **Absences must be visible as absences.** Anything not modelled is stated
//!    on the HUD, derived from the live law where it can be. See `hud::unmodelled`.
//! 3. **A hunch must be capturable in the moment.** `C` writes a **capture pair**
//!    (`captures/*-vivarium-info.v0.1.0.udon` + `.png`) plus a classic sighting
//!    md (`#disc-explorer-debug-capture`). Default chrome is status chips
//!    (`#disc-explorer-human-chrome`); `H` cycles human → debug dump → minimal.
//!
//! **The wall.** `vivarium-world` has no idea Bevy exists (dependency direction),
//! and this binary opens the store through `Store::open_read_only`, so it cannot
//! author a world citizen even by accident — the HUD shows its own refused-write
//! count ( #form-core-view-wall , #form-builder-admission ).
//!
//! Run as `vivarium explore [dir]`, which execs this binary.

mod hud;
mod lens;
mod mesh;
mod paint;
mod pull;
mod capture;
mod sighting;
mod water;

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use vivarium_world::planet::Planet;
use vivarium_world::spec::WorldSpec;
use vivarium_world::sphere::CubeCoord;
use vivarium_world::store::Store;
use vivarium_world::watch::{self, BuildState, Coverage};

use crate::lens::{Chain, Ladder, Lens};
use crate::mesh::radius_km;
use crate::paint::Paint;
use crate::pull::{Frame, Msg, Request};
use crate::water::WaterField;

/// Levels the **whole-globe** mesh spans. After P0/P1 thrash cuts, L8/L9 whole-
/// globe is usable again (L8 ≈ 4× L7 cells, L9 ≈ 16×). Keeping the ceiling at
/// L7 forced a hard jump L7-globe → L10-window and made L8/L9 un-seeable as
/// intermediate rungs. Window mode starts only **above** this ceiling.
const LEVEL_MIN: u8 = 5;
const LEVEL_GLOBE_MAX: u8 = 9;

/// Levels the **region window** spans. Finer than the globe ceiling (close-in)
/// draws a camera-centred window **plus adjacent same-face panes** (pull expands
/// the ring) — cost set by window size × ring, not by planet area.
///
/// L13 ≈ 1.2 km cells, where the tree's fine builds live and fluvial form is a
/// *shape* rather than a single cell.
const LEVEL_MAX: u8 = 14;

/// Region window width bounds (cells). Cost is O(nx²); 384² was the old fixed
/// size that made every close-in frame feel like a full-globe remesh.
const PATCH_NX_MIN: usize = 128;
const PATCH_NX_MAX: usize = 384;

/// Relief exaggeration cycle for X. 1 = honest (a billiard ball, truthfully).
const EXAG_STEPS: [f32; 4] = [1.0, 10.0, 20.0, 50.0];

/// Full-scale steps for the change ramp (m), cycled by Z.
///
/// Fixed steps rather than a per-frame auto-fit, and that is the whole design.
/// The change field grows monotonically along the settle history — measured on
/// the default world, mean |change| runs 3.8 m at the first stage to 25.6 m at
/// the last — so a ramp that renormalized each frame would show a constant
/// picture across a scrub whose entire subject is the growth. The default sits
/// just above the last stage's mean so early stages read as faint and late ones
/// as saturated, which is what the numbers actually say.
const CHANGE_STEPS: [f32; 5] = [10.0, 40.0, 150.0, 600.0, 2400.0];

const SPACE: Color = Color::srgb(0.012, 0.014, 0.022);

/// Wrap width for the overlay panel, in characters. The panel is pinned to the
/// left half of the window so the planet is never fully behind it.
const HUD_COLS: usize = 104;

/// How densely the view samples the cooling law when the manifest asks for less.
/// 120 stages over ~3.2 Ga is ~27 Myr per frame — fine enough that emergence
/// reads as a process rather than a slideshow.
const VIEW_FRAMES_FLOOR: u32 = 120;

// ---------------------------------------------------------------- resources

#[derive(Resource)]
struct Orbit {
    yaw: f32,
    pitch: f32,
    dist: f32,
    vel_yaw: f32,
    vel_pitch: f32,
}

impl Default for Orbit {
    fn default() -> Self {
        let dist = std::env::var("VIVARIUM_DIST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3.0 * radius_km());
        Orbit { yaw: -0.9, pitch: 0.42, dist, vel_yaw: 0.0, vel_pitch: 0.0 }
    }
}

/// Ethereal time-freedom over the sky. The ephemeris is a pure function of
/// (day, hour), so scrubbing solar time is a *view* freedom touching no world
/// state — and being able to relight a scene matters to the detector: a shadow
/// artefact and a terrain artefact look alike until you move the sun.
#[derive(Resource)]
struct Sun {
    day: f32,
    hour: f32,
    play: f32,
    headlight: bool,
}

impl Default for Sun {
    fn default() -> Self {
        Sun { day: 91.0, hour: 10.0, play: 0.0, headlight: false }
    }
}

#[derive(Resource)]
struct Explorer {
    lens: Lens,
    paint: Paint,
    auto_level: bool,
    level: u8,
    exag_i: usize,
    change_i: usize,
    /// Which settle history is on the time axis.
    cohort: usize,
    /// Deep-time sweep.
    playing: bool,
    dwell: f32,
    dwell_per: f32,
    /// Replay sweep.
    replay_playing: bool,
    replay_frame: usize,
    replay_frames: Vec<usize>,
    requested: Option<Request>,
    inflight: bool,
    frame: Option<Box<Frame>>,
    /// How much overlay to draw. The instrument's own display is the one thing
    /// guaranteed to be between the observer and the world, so it tiers: an
    /// explorer whose HUD covers the planet has defeated the detector it exists
    /// to serve.
    /// 0 = human chrome (chips), 1 = full debug dump, 2 = minimal paint/lens.
    hud_level: u8,
    /// A transient line under the HUD (capture written, etc.).
    notice: String,
    notice_until: f32,
    /// Erosion fresh/stale under this binary — refreshed when the frame updates.
    erosion_census: vivarium_world::query::RegionCensus,
    /// `?` / `/` toggles keybinding legend (off by default).
    show_keys: bool,
}

#[derive(Resource)]
struct Ident {
    name: String,
    seed: u64,
    dir: PathBuf,
    /// Manifest prescription (beacon, level, …) for captures and chrome.
    demand: vivarium_world::spec::WorldSpec,
}

#[derive(Resource)]
struct LadderRes(Ladder);
/// The settle history's own axis. Kept ECS-side so the input system knows how
/// many stages exist without asking the worker — and refreshed from every frame,
/// so a chain that grows while a builder runs grows on screen too.
#[derive(Resource)]
struct ChainRes(Chain);
#[derive(Resource)]
struct CovRes(Coverage);
/// The water field, kept view-side for the cursor readout (the meshes already
/// carry its colour; this is so the pick can report a depth).
#[derive(Resource)]
struct WaterRes(WaterField);
#[derive(Resource)]
struct ReqTx(Sender<Request>);
#[derive(Resource)]
struct MsgRx(Mutex<Receiver<Msg>>);
/// Pending screenshot path for a capture, taken one frame after the dump.
#[derive(Resource, Default)]
struct PendingShot(Option<PathBuf>);

#[derive(Component)]
struct FaceEntity;
#[derive(Component)]
struct ExploreCam;
#[derive(Component)]
struct SunLight;
#[derive(Component)]
struct HudText;

// -------------------------------------------------------------------- main

/// Non-flag tokens, skipping values that belong to known flags — so
/// `explore --level 7` resolves to the default world, not to a directory called
/// `7`. (The CLI's `positionals` makes the same distinction for the same reason;
/// getting it wrong here produced exactly the "no vivium at 7" confusion the
/// announce-line discipline exists to prevent.)
fn positionals(args: &[String]) -> Vec<&str> {
    let takes_value = |a: &str| matches!(a, "--level" | "--frames" | "--paint");
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a.starts_with('-') {
            i += if takes_value(a) { 2 } else { 1 };
            continue;
        }
        out.push(a);
        i += 1;
    }
    out
}

fn world_dir(args: &[String]) -> PathBuf {
    if let Some(p) = positionals(args).first() {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("VIVARIUM_WORLD") {
        return PathBuf::from(p);
    }
    let cache = std::env::var("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".cache")
    });
    cache.join("vivarium").join("globe-world")
}

fn flag_u32(args: &[String], name: &str) -> Option<u32> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok())
}

const USAGE: &str = "\
vivarium explore -- the 3D explorer: an instrument for seeing whether a world's
                   systems are behaving the way its law says they should.

  vivarium explore [dir] [--replay] [--level L] [--frames N] [--paint MODE]

  [dir]          which vivium (else $VIVARIUM_WORLD, else ~/.cache/vivarium/globe-world)
  --replay       open on the build history rather than the present
  --level L      fix the render level (else it follows altitude)
  --frames N     how densely the VIEW samples the COOLING law (default 120).
                 This is observation density, not demand: it changes nothing
                 about the world and writes nothing. Stages the builder has
                 materialized are marked as store citizens on the timeline.
                 It reaches the cooling chain and nothing else, because that
                 chain is law-evaluable -- T_p(t) is closed form, so a denser
                 request is more evaluations of the law. It does NOT reach the
                 erosion settle history, and no flag will: that chain is
                 materialized-only, its density is exactly what the builder ran,
                 and asking for more is a build request, not a view parameter.
  --paint MODE   surface | provenance | water | seam | change | depression

THE TWO TIME AXES

  E   the EROSION SETTLE HISTORY -- world-time. Every tile on screen is drawn at
      the same epoch, so this is the surface evolving, not the builder working.
      The stages are what the builder materialized (8 on the default world) and
      nothing is interpolated between them: where the picture jumps, nothing
      exists in between, and the HUD says so.
  T   DEEP TIME -- the mantle cooling chain. Tectonics and isostasy only; no
      fluvial carve exists at any epoch but the present.
  V   REPLAY -- BUILD history, the order roots landed. Not world-time.

  Paint 5 (change) is what makes the settle history legible: across the whole
  history the mean absolute elevation change is 3.8 m rising to 25.6 m, against
  relief of kilometres, so in hypsometric colour forty epochs of erosion are
  invisible. It is signed for a reason -- 88% of cells RISE (the uplift driver)
  and 5.6% FALL (fluvial incision won there).

The store is opened READ-ONLY. This binary cannot author a world citizen; the
HUD shows the refused-write count so the wall is a number, not a promise.

Press C to capture: writes captures/*-vivarium-info.v0.1.0.udon + .png (and a
classic sighting md). Same form via VIVARIUM_SHOT for agent autoshot.
Default overlay is compact chrome; H cycles human / debug / minimal. ? toggles keys.
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{USAGE}");
        return;
    }
    let dir = world_dir(&args);
    let spec = match WorldSpec::load(&dir) {
        Ok(Some(s)) => s,
        Ok(None) => {
            eprintln!(
                "[explore] no vivium at {} -- nothing to explore.\n\
                 \n\
                 A world has to be individuated before it can be observed; an explorer that\n\
                 minted one would be authoring the thing it is supposed to be watching.\n\
                 \n    vivarium new {}\n    vivarium build {}\n",
                dir.display(),
                dir.display(),
                dir.display()
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("[explore] manifest error at {}: {e}", dir.display());
            std::process::exit(1);
        }
    };

    let view_frames = flag_u32(&args, "--frames").unwrap_or(VIEW_FRAMES_FLOOR);
    let fixed_level = flag_u32(&args, "--level").map(|l| l.clamp(LEVEL_MIN as u32, LEVEL_MAX as u32) as u8);
    let start_paint = args
        .iter()
        .position(|a| a == "--paint")
        .and_then(|i| args.get(i + 1))
        .and_then(|m| Paint::ALL.iter().find(|p| p.name() == m).copied())
        .unwrap_or(Paint::Surface);
    let start_replay = args.iter().any(|a| a == "--replay");

    // A read-only handle, used once here to read the census the ECS needs. The
    // worker opens its own.
    let store = Store::open_read_only(&dir).expect("read-only store open cannot fail");
    let world = vivarium_world::query::World::new(&store, spec.seed);
    let roots = store.roots().unwrap_or_default();
    if roots.is_empty() {
        eprintln!(
            "[explore] vivium \"{}\" exists but nothing is built yet.\n\
             Opening anyway: the fated prior is a pure function of the seed, so there IS a\n\
             world to look at -- it just has no built state, and the HUD will say so.\n\
             `vivarium build {}` is what fills it in.",
            spec.name,
            dir.display()
        );
    }
    let cov = Coverage::parse(&roots);
    let ladder = Ladder::read(&world, spec.demand.frames, view_frames);
    let water = WaterField::load_from_roots(&world, &roots);
    let mut chain = Chain::read(&roots, 0);
    lens::read_residuals(&store, &roots, &mut chain);
    // Fresh vs stale under *this* binary — P3 bar is carve coverage, not tile count.
    let erosion_census = world.observe().eroded_region_census();
    let src8 = &vivarium_world::nomotheke::SRC_HASH[..8.min(vivarium_world::nomotheke::SRC_HASH.len())];
    println!(
        "[explore] vivium \"{}\" (seed {:#018x}) at {} -- {} roots, {} tiles at L{}, ladder {} stages ({} built)",
        spec.name,
        spec.seed,
        dir.display(),
        roots.len(),
        cov.built_tiles(),
        cov.level,
        ladder.len(),
        ladder.built_count()
    );
    println!(
        "[explore] erosion under this binary src={src8}: {} fresh · {} stale (other src) · {} total",
        erosion_census.fresh, erosion_census.stale, erosion_census.total
    );
    if erosion_census.fresh == 0 && erosion_census.stale > 0 {
        println!(
            "[explore] *** REBUILD NEEDED *** no eroded land is *readable* here — store has \
             history under other source hashes only. Open globe is honest pure prior (fast).\n\
             [explore]   vivarium build\n\
             [explore] then re-open explore (or zoom close after tiles land under the new src)."
        );
    } else if erosion_census.fresh > 0 {
        println!(
            "[explore] eroded land readable — far view stays ≤L{LEVEL_GLOBE_MAX} whole-globe \
             (prior or coarse); zoom close past L{LEVEL_GLOBE_MAX} for covering-grain windows."
        );
    }
    match chain.cohort.as_ref() {
        Some(c) => println!(
            "[explore] erosion settle history: {} materialized stages, epochs {:?}, {} tiles at L{}, src {}{}\n\
             [explore]   press E to scrub it, 5 for the change paint. This chain is materialized-only: its\n\
             [explore]   density is what the builder ran, and no view flag can add to it.",
            c.len(),
            c.epochs,
            c.tiles.first().copied().unwrap_or(0),
            c.level,
            &c.src[..8.min(c.src.len())],
            if c.is_current {
                " (this binary's source)".to_string()
            } else {
                format!(
                    " -- NOT this binary's source ({src8}). A previous world's history; \
                     the HUD says so on every frame."
                )
            }
        ),
        None => println!(
            "[explore] no erosion settle history in this store (no nomos has more than one materialized \
             time-index), so there is nothing for E to scrub. `vivarium build` with a stage stride is what \
             creates one."
        ),
    }

    let landings = watch::landings(&dir).unwrap_or_default();
    let replay_frames = watch::frame_bounds(&landings, 240);

    let (req_tx, req_rx) = std::sync::mpsc::channel::<Request>();
    let (msg_tx, msg_rx) = std::sync::mpsc::channel::<Msg>();
    pull::spawn(dir.clone(), spec.seed, spec.demand.frames, view_frames, req_rx, msg_tx);

    let start_stage = std::env::var("VIVARIUM_STAGE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|i| *i < ladder.len());
    // `VIVARIUM_EROSION=<i>` opens on one settle stage — the same verification
    // idiom as `VIVARIUM_STAGE`, and the one that lets a session shoot the whole
    // history frame by frame and actually look at what it shipped.
    let start_erosion = std::env::var("VIVARIUM_EROSION")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|i| *i < chain.len());
    let start_lens = match (start_erosion, start_stage, start_replay && !replay_frames.is_empty()) {
        (Some(i), _, _) => Lens::Erosion(i),
        (None, Some(i), _) => Lens::Stage(i),
        (None, None, true) => Lens::Replay(replay_frames[0]),
        (None, None, false) => Lens::Present,
    };

    // Opening ON a windowed chain means opening pointed at it. Without this the
    // verification idiom (`VIVARIUM_EROSION=<i> VIVARIUM_SHOT=…`) photographs a
    // planet from orbit and calls it a look at a 300 km patch — a session
    // shipping a renderer it never actually saw.
    let mut start_orbit = Orbit::default();
    if let (Lens::Erosion(_), Some(c)) = (start_lens, chain.cohort.as_ref()) {
        if !c.is_global() {
            frame_cohort(&mut start_orbit, &mut true, c);
        }
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("vivarium explore -- {}", spec.name),
                resolution: bevy::window::WindowResolution::new(1400, 900),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(SPACE))
        .insert_resource(start_orbit)
        .insert_resource(Sun::default())
        .insert_resource(Explorer {
            lens: start_lens,
            paint: start_paint,
            auto_level: fixed_level.is_none(),
            level: fixed_level.unwrap_or(8),
            exag_i: 2,
            change_i: 2,
            cohort: 0,
            playing: false,
            dwell: 0.0,
            dwell_per: 0.09,
            replay_playing: start_replay,
            replay_frame: 0,
            replay_frames,
            requested: None,
            inflight: false,
            frame: None,
            hud_level: 0, // human chrome default (#disc-explorer-human-chrome)
            notice: String::new(),
            notice_until: 0.0,
            erosion_census,
            show_keys: false,
        })
        .insert_resource(Ident {
            name: spec.name.clone(),
            seed: spec.seed,
            dir,
            demand: spec.clone(),
        })
        .insert_resource(LadderRes(ladder))
        .insert_resource(ChainRes(chain))
        .insert_resource(CovRes(cov))
        .insert_resource(WaterRes(water))
        .insert_resource(ReqTx(req_tx))
        .insert_resource(MsgRx(Mutex::new(msg_rx)))
        .insert_resource(PendingShot::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                input_update,
                lens_update,
                request_update,
                apply_frames,
                camera_update,
                hud_update,
                capture_sighting,
                take_pending_shot,
                autoshot,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 45f32.to_radians(),
            near: 5.0,
            far: 90_000.0,
            ..default()
        }),
        Transform::default(),
        ExploreCam,
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.97, 0.90),
            illuminance: 12_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::default(),
        SunLight,
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.65, 0.72, 0.85),
        brightness: 240.0,
        affects_lightmapped_meshes: true,
    });
    commands.spawn((
        Text::new("reading the store..."),
        TextFont { font_size: 12.0, ..default() },
        TextColor(Color::srgb(0.86, 0.88, 0.91)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(10.0),
            width: Val::Percent(53.0),
            padding: UiRect::all(Val::Px(7.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
        HudText,
    ));
}

// ------------------------------------------------------------------- input

fn input_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut orbit: ResMut<Orbit>,
    mut ex: ResMut<Explorer>,
    mut sun: ResMut<Sun>,
    chain: Res<ChainRes>,
    mut exit: MessageWriter<AppExit>,
) {
    let dt = time.delta_secs().max(1e-4);
    let r = radius_km();

    let mut d = Vec2::ZERO;
    for m in motion.read() {
        d += m.delta;
    }
    let grab = 0.0022 * ((orbit.dist - r) / r).clamp(0.03, 2.5);
    if buttons.pressed(MouseButton::Left) && d != Vec2::ZERO {
        // Grab-the-surface semantics, sign set EMPIRICALLY: the front face
        // follows the cursor. The spike's original sign was tuned against an
        // inside-out globe (a winding bug), whose mirror flips apparent
        // chirality — two bugs masking each other. If `camera_update`'s
        // yaw→eye mapping ever changes, re-verify by dragging.
        orbit.yaw += d.x * grab;
        orbit.pitch += d.y * grab;
        orbit.vel_yaw = d.x * grab / dt;
        orbit.vel_pitch = d.y * grab / dt;
    } else {
        orbit.yaw += orbit.vel_yaw * dt;
        orbit.pitch += orbit.vel_pitch * dt;
        let decay = (-3.0 * dt).exp();
        orbit.vel_yaw *= decay;
        orbit.vel_pitch *= decay;
    }
    let key_rate = 1.2 * dt * ((orbit.dist - r) / r).clamp(0.08, 1.0).max(0.15);
    if keys.pressed(KeyCode::ArrowLeft) {
        orbit.yaw -= key_rate;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        orbit.yaw += key_rate;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        orbit.pitch += key_rate;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        orbit.pitch -= key_rate;
    }
    orbit.pitch = orbit.pitch.clamp(-1.55, 1.55);
    if keys.just_pressed(KeyCode::KeyO) {
        orbit.pitch = std::f32::consts::FRAC_PI_2;
        orbit.yaw = 0.0;
        orbit.vel_yaw = 0.0;
        orbit.vel_pitch = 0.0;
    }

    // Sun scrub — relighting is how a shadow artefact is told from a terrain one.
    if keys.pressed(KeyCode::Comma) {
        sun.hour -= 6.0 * dt;
    }
    if keys.pressed(KeyCode::Period) {
        sun.hour += 6.0 * dt;
    }
    if keys.pressed(KeyCode::KeyN) {
        sun.day -= 40.0 * dt;
    }
    if keys.pressed(KeyCode::KeyM) {
        sun.day += 40.0 * dt;
    }
    if keys.just_pressed(KeyCode::KeyY) {
        sun.headlight = !sun.headlight;
    }
    sun.hour += sun.play * dt;
    while sun.hour >= 24.0 {
        sun.hour -= 24.0;
        sun.day += 1.0;
    }
    while sun.hour < 0.0 {
        sun.hour += 24.0;
        sun.day -= 1.0;
    }
    sun.day = sun.day.rem_euclid(365.25);

    // Zoom: exponential in altitude — each notch takes the same fraction of the
    // remaining distance to the surface, so the approach never overshoots.
    let mut scroll = 0.0f32;
    for w in wheel.read() {
        scroll += match w.unit {
            MouseScrollUnit::Line => w.y * 1.0,
            MouseScrollUnit::Pixel => w.y * 0.02,
        };
    }
    if keys.pressed(KeyCode::Equal) {
        scroll += 2.2 * dt;
    }
    if keys.pressed(KeyCode::Minus) {
        scroll -= 2.2 * dt;
    }
    if scroll != 0.0 {
        let alt = orbit.dist - r;
        orbit.dist = r + (alt * (1.12f32).powf(-scroll)).clamp(120.0, 7.0 * r);
    }

    if keys.just_pressed(KeyCode::KeyR) {
        *orbit = Orbit::default();
    }
    // B — go to the selected chain's own extent.
    //
    // Not a convenience. A fine chain is a 256-cell window in an 8192-cell face —
    // roughly one part in a thousand of the globe by area — and an instrument that
    // holds data a viewer cannot find is an instrument that holds nothing. The
    // destination is read from the store census ( `ErosionCohort::centre` ), never
    // from a coordinate anyone typed here, so it stays right when the builder
    // moves the beacon.
    if keys.just_pressed(KeyCode::KeyB) {
        if let Some(c) = chain.0.cohort.as_ref().or(chain.0.all.first()) {
            frame_cohort(&mut orbit, &mut ex.auto_level, c);
        }
    }
    if keys.just_pressed(KeyCode::KeyX) {
        ex.exag_i = (ex.exag_i + 1) % EXAG_STEPS.len();
    }
    if keys.just_pressed(KeyCode::KeyZ) {
        ex.change_i = (ex.change_i + 1) % CHANGE_STEPS.len();
    }
    if keys.just_pressed(KeyCode::Tab) {
        ex.paint = ex.paint.cycle();
    }
    for (k, p) in [
        (KeyCode::Digit1, Paint::Surface),
        (KeyCode::Digit2, Paint::Provenance),
        (KeyCode::Digit3, Paint::Water),
        (KeyCode::Digit4, Paint::Seam),
        (KeyCode::Digit5, Paint::Change),
        (KeyCode::Digit6, Paint::Depression),
    ] {
        if keys.just_pressed(k) {
            ex.paint = p;
        }
    }
    if keys.just_pressed(KeyCode::KeyA) {
        ex.auto_level = true;
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        ex.auto_level = false;
        ex.level = ex.level.saturating_sub(1).max(LEVEL_MIN);
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        ex.auto_level = false;
        ex.level = (ex.level + 1).min(LEVEL_MAX);
    }
    if keys.just_pressed(KeyCode::KeyH) {
        ex.hud_level = (ex.hud_level + 1) % 3;
    }
    // `?` is Shift+/ on US keyboards; Bevy reports Slash for both.
    if keys.just_pressed(KeyCode::Slash) {
        ex.show_keys = !ex.show_keys;
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

/// Point the camera at a chain's own extent and frame it.
///
/// Read from the store census ( `ErosionCohort::centre` / `span_cells` ), never
/// from a coordinate typed here, so it stays right when the builder moves the
/// beacon. The altitude frames the chain's arc with headroom, and `auto_level`
/// then resolves the matching render level from the same number — so arriving at
/// a chain and seeing it at its own resolution is one action rather than three.
fn frame_cohort(orbit: &mut Orbit, auto_level: &mut bool, c: &watch::ErosionCohort) {
    let Some((face, i, j)) = c.centre() else { return };
    let n = 1u32 << c.level;
    let cu = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
    let cv = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
    let d = CubeCoord { face: vivarium_world::sphere::Face::from_index(face), u: cu, v: cv }.to_unit();
    orbit.yaw = (d[2] as f32).atan2(d[0] as f32);
    orbit.pitch = (d[1] as f32).clamp(-1.0, 1.0).asin().clamp(-1.55, 1.55);
    orbit.vel_yaw = 0.0;
    orbit.vel_pitch = 0.0;
    let span_km =
        radius_km() * std::f32::consts::FRAC_PI_2 * c.span_cells() as f32 / (1u32 << c.level) as f32;
    orbit.dist = radius_km() + (span_km * 1.4).clamp(20.0, 7.0 * radius_km());
    *auto_level = true;
}

/// Lens selection: present / deep time / replay. Every one of these is a
/// *selection among materialized or lawfully-derivable states* — none of them
/// changes how the world evolves ( #form-core-view-wall FE(4) ).
fn lens_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ex: ResMut<Explorer>,
    mut orbit: ResMut<Orbit>,
    ladder: Res<LadderRes>,
    chain: Res<ChainRes>,
) {
    let n = ladder.0.len().max(1);
    let cn = chain.0.len();

    if keys.just_pressed(KeyCode::KeyP) {
        ex.lens = Lens::Present;
        ex.playing = false;
        ex.replay_playing = false;
    }

    // G — switch which settle history is on the axis. There can be more than one
    // (a global sweep and a fine beacon are two chains, `watch::erosion_cohorts`),
    // and which one you want is not something the view can infer.
    if keys.just_pressed(KeyCode::KeyG) {
        ex.cohort = (ex.cohort + 1) % chain.0.all.len().max(1);
        if let Lens::Erosion(_) = ex.lens {
            ex.lens = Lens::Erosion(0);
        }
    }

    // E — the settle history. Opening on stage 0 rather than the present is
    // deliberate: the question this lens answers is "what changed", and it is
    // only answerable from the beginning.
    if keys.just_pressed(KeyCode::KeyE) && cn > 1 {
        ex.lens = match ex.lens {
            Lens::Erosion(_) => Lens::Present,
            _ => Lens::Erosion(0),
        };
        ex.playing = matches!(ex.lens, Lens::Erosion(_));
        ex.replay_playing = false;
        ex.dwell = 0.0;
        // A chain that covers a window rather than the globe is invisible from
        // wherever the camera happened to be, and a scrub of something off-screen
        // is a scrub of a blank planet. Going there is part of selecting it.
        if let (Lens::Erosion(_), Some(c)) = (ex.lens, chain.0.cohort.as_ref()) {
            if !c.is_global() {
                frame_cohort(&mut orbit, &mut ex.auto_level, c);
            }
        }
    }

    if let Lens::Erosion(i) = ex.lens {
        if keys.just_pressed(KeyCode::KeyK) {
            ex.playing = !ex.playing;
            ex.dwell = 0.0;
        }
        if keys.just_pressed(KeyCode::KeyJ) {
            ex.playing = false;
            ex.lens = Lens::Erosion((i + cn - 1) % cn.max(1));
        }
        if keys.just_pressed(KeyCode::KeyL) {
            ex.playing = false;
            ex.lens = Lens::Erosion((i + 1) % cn.max(1));
        }
        if ex.playing && !ex.inflight {
            ex.dwell += time.delta_secs();
            // A materialized-only chain is short — 8 stages, not 120 — so each
            // one has to be *looked at*, not swept past. The dwell is an order of
            // magnitude longer than the cooling sweep's for that reason, and the
            // jump between stages is left as a jump.
            if ex.dwell >= 0.85 {
                ex.dwell = 0.0;
                ex.lens = Lens::Erosion((i + 1) % cn.max(1));
            }
        }
    }
    if keys.just_pressed(KeyCode::KeyT) {
        ex.lens = match ex.lens {
            Lens::Stage(_) => Lens::Present,
            _ => Lens::Stage(0),
        };
        ex.playing = matches!(ex.lens, Lens::Stage(_));
        ex.replay_playing = false;
        ex.dwell = 0.0;
    }
    if keys.just_pressed(KeyCode::KeyV) && !ex.replay_frames.is_empty() {
        ex.replay_frame = 0;
        ex.lens = Lens::Replay(ex.replay_frames[0]);
        ex.replay_playing = true;
        ex.playing = false;
    }

    if let Lens::Stage(i) = ex.lens {
        if keys.just_pressed(KeyCode::KeyK) {
            ex.playing = !ex.playing;
            ex.dwell = 0.0;
        }
        if keys.just_pressed(KeyCode::KeyJ) {
            ex.playing = false;
            ex.lens = Lens::Stage((i + n - 1) % n);
        }
        if keys.just_pressed(KeyCode::KeyL) {
            ex.playing = false;
            ex.lens = Lens::Stage((i + 1) % n);
        }
        if ex.playing {
            ex.dwell += time.delta_secs();
            if ex.dwell >= ex.dwell_per {
                // Advance only when the previous surface has landed: the sweep
                // is paced by the slower of the clock and the pull, so it never
                // skips a stage it failed to draw. A skipped stage is a hole in
                // the motion, which is precisely what defeats the detector.
                if !ex.inflight {
                    ex.lens = Lens::Stage((i + 1) % n);
                    ex.dwell = 0.0;
                }
            }
        }
    }

    if let Lens::Replay(_) = ex.lens {
        if keys.just_pressed(KeyCode::KeyK) {
            ex.replay_playing = !ex.replay_playing;
        }
        if keys.just_pressed(KeyCode::KeyJ) && ex.replay_frame > 0 {
            ex.replay_playing = false;
            ex.replay_frame -= 1;
            ex.lens = Lens::Replay(ex.replay_frames[ex.replay_frame]);
        }
        if keys.just_pressed(KeyCode::KeyL) && ex.replay_frame + 1 < ex.replay_frames.len() {
            ex.replay_playing = false;
            ex.replay_frame += 1;
            ex.lens = Lens::Replay(ex.replay_frames[ex.replay_frame]);
        }
        if ex.replay_playing && !ex.inflight {
            ex.dwell += time.delta_secs();
            if ex.dwell >= 0.12 {
                ex.dwell = 0.0;
                if ex.replay_frame + 1 < ex.replay_frames.len() {
                    ex.replay_frame += 1;
                    ex.lens = Lens::Replay(ex.replay_frames[ex.replay_frame]);
                } else {
                    ex.replay_playing = false;
                }
            }
        }
    }
}

/// Altitude below which a fine region window is allowed (fraction of planet
/// radius). Above this the view is always a **whole globe** at ≤ [`LEVEL_GLOBE_MAX`].
///
/// The screenshot failure mode this encodes: at ~3900 km altitude the old code
/// still opened an L9 region window because level alone forced windowing — a
/// postage stamp on black while the HUD said "updating" for 6+ seconds. Window
/// is a *close-in* mode, not a level mode.
const CLOSE_ALT_FRAC: f32 = 0.28;

/// Resolution-on-zoom. **Latest request always wins** — the worker drains its
/// queue to the newest `Request` before each build, so a multi-second pull
/// never forces the viewer to watch every intermediate zoom level. The old
/// `!inflight` gate dropped all scroll while a globe remesh ran, which is how
/// the instrument spent seconds building a view the hand had already left.
fn request_update(orbit: Res<Orbit>, mut ex: ResMut<Explorer>, tx: Res<ReqTx>) {
    let r = radius_km();
    let alt = (orbit.dist - r).max(20.0);
    let close = alt < CLOSE_ALT_FRAC * r;

    if ex.auto_level {
        // Cell size tracks altitude so screen-space grain stays roughly constant.
        // Step *at most one level toward the target* so a fast scroll does not
        // jump L6→L13 and force a pathologically expensive first rebuild.
        // Far from the surface, never target above the whole-globe ceiling.
        let quarter = r * std::f32::consts::FRAC_PI_2;
        let target_cell = (alt * 0.008).max(0.15); // km
        let raw = (quarter / target_cell).log2().ceil() as i32;
        let hi = if close { LEVEL_MAX } else { LEVEL_GLOBE_MAX };
        let target = raw.clamp(LEVEL_MIN as i32, hi as i32) as u8;
        if target > ex.level {
            ex.level += 1;
        } else if target < ex.level {
            ex.level -= 1;
        }
    }

    // **Mesh level:** far out, clamp to the globe ceiling even if the user
    // manually raised level with `]` — otherwise we open a postage-stamp window
    // on a face you cannot even see well from this altitude.
    let mesh_level = if close {
        ex.level
    } else {
        ex.level.min(LEVEL_GLOBE_MAX)
    };

    // Window only when close *and* finer than the whole-globe ceiling (L10+
    // once LEVEL_GLOBE_MAX is 9). L8/L9 stay whole-globe so the level ladder
    // is continuous rather than L7-globe → postage-stamp.
    let want_window = close && mesh_level > LEVEL_GLOBE_MAX;
    let d = view_dir(&orbit);
    let mut window = want_window.then(|| {
        let nx = patch_nx_for(alt, mesh_level, r);
        pull::fov_cover_panes([d.x, d.y, d.z], alt, mesh_level, nx, r)
    });
    // **Sticky cover.** Without this, every pixel of orbit drag recomputes the
    // FOV mosaic and enqueues a remesh. Freeze the whole pane set while the
    // look centre stays inside a slack band of the previous centre.
    if let (Some(cov), Some(prev_req)) = (&window, &ex.requested) {
        if let Some(prev) = &prev_req.window {
            if prev.centre.face == cov.centre.face
                && prev.centre.nx == cov.centre.nx
                && prev_req.level == mesh_level
            {
                let slack = (cov.centre.nx as u32 / 8).max(8);
                let di = cov.centre.oi.abs_diff(prev.centre.oi);
                let dj = cov.centre.oj.abs_diff(prev.centre.oj);
                if di < slack && dj < slack {
                    window = Some(prev.clone());
                }
            }
        }
    }
    let want = Request {
        level: mesh_level,
        window,
        cohort: ex.cohort,
        exag: EXAG_STEPS[ex.exag_i],
        paint: ex.paint,
        lens: ex.lens,
        change_scale_m: CHANGE_STEPS[ex.change_i],
    };
    // Always enqueue the latest want. The worker keeps only the last request
    // when several pile up during a slow build.
    if ex.requested.as_ref() != Some(&want) && tx.0.send(want.clone()).is_ok() {
        ex.requested = Some(want);
        ex.inflight = true;
    }
}

/// Camera look direction (unit vector from planet centre toward the eye).
fn view_dir(orbit: &Orbit) -> Vec3 {
    Vec3::new(
        orbit.pitch.cos() * orbit.yaw.cos(),
        orbit.pitch.sin(),
        orbit.pitch.cos() * orbit.yaw.sin(),
    )
}

/// Window width so the patch roughly fills the FOV at the current altitude.
/// Flat-sky approx: visible arc ≈ 2 · alt · tan(fov/2); convert to cells at
/// this level. Clamped so cost stays bounded.
fn patch_nx_for(alt_km: f32, level: u8, r_km: f32) -> usize {
    let cell_km = (r_km * std::f32::consts::FRAC_PI_2) / (1u32 << level) as f32;
    let fov = 45f32.to_radians();
    let half = alt_km * (fov * 0.5).tan();
    let cells = (2.2 * half / cell_km.max(1e-3)).ceil() as usize;
    cells.clamp(PATCH_NX_MIN, PATCH_NX_MAX)
}

fn apply_frames(
    mut commands: Commands,
    rx: Res<MsgRx>,
    mut ex: ResMut<Explorer>,
    mut ladder: ResMut<LadderRes>,
    mut chain: ResMut<ChainRes>,
    mut cov: ResMut<CovRes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    old: Query<Entity, With<FaceEntity>>,
) {
    // Drain the channel: keep only the latest Frame. Landings are advisory and
    // must not block applying a frame that is already waiting behind them.
    let mut last_frame: Option<Box<Frame>> = None;
    {
        let rx = rx.0.lock().unwrap();
        loop {
            match rx.try_recv() {
                Ok(Msg::Frame(f)) => last_frame = Some(f),
                Ok(Msg::Landings(_)) => {}
                Ok(Msg::AlreadyCurrent(r)) => {
                    if ex.requested.as_ref() == Some(&r) {
                        ex.inflight = false;
                    }
                }
                Err(_) => break,
            }
        }
    }
    let Some(frame) = last_frame else { return };

    // Inflight clears only when the frame matches the latest request we sent —
    // otherwise a slower older build finished after a newer one was enqueued.
    if ex.requested.as_ref() == Some(&frame.req) {
        ex.inflight = false;
    }
    ladder.0.built = frame.ladder_built.clone();
    chain.0 = frame.chain.clone();
    // Worker already parsed coverage for this roots epoch — do not re-walk ~10⁵ keys.
    cov.0 = frame.coverage.clone();
    // A stage index that outlived a shrinking chain would silently address a
    // different moment. Clamp rather than reset, so a builder landing new stages
    // mid-scrub keeps you where you were looking.
    if let Lens::Erosion(i) = ex.lens {
        let n = chain.0.len();
        if n == 0 {
            ex.lens = Lens::Present;
        } else if i >= n {
            ex.lens = Lens::Erosion(n - 1);
        }
    }

    for e in &old {
        commands.entity(e).despawn();
    }
    let mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.92,
        reflectance: 0.08,
        ..default()
    });
    for fm in &frame.faces {
        let mut m = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, fm.positions.clone());
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, fm.normals.clone());
        m.insert_attribute(Mesh::ATTRIBUTE_COLOR, fm.colors.clone());
        m.insert_indices(Indices::U32(fm.indices.clone()));
        commands.spawn((
            Mesh3d(meshes.add(m)),
            MeshMaterial3d(mat.clone()),
            Transform::default(),
            FaceEntity,
        ));
    }
    ex.frame = Some(frame);
}

fn camera_update(
    orbit: Res<Orbit>,
    sun: Res<Sun>,
    mut cam: Query<(&mut Transform, &mut Projection), (With<ExploreCam>, Without<SunLight>)>,
    mut light: Query<&mut Transform, (With<SunLight>, Without<ExploreCam>)>,
) {
    let r = radius_km();
    let dir = view_dir(&orbit);
    let eye = dir * orbit.dist;
    let alt = (orbit.dist - r).max(1.0);

    // **Look target.** Far out: planet centre (classic orbit). Close in: the
    // nadir on the surface under the camera. Looking at the centre from low
    // altitude is what made the horizon "do the wrong thing" — the camera aims
    // through the planet at the far limb while the mesh underfoot foreshortens
    // into mush. Blend over ~1.2 radii of altitude.
    let nadir_blend = (1.0 - (alt / (1.2 * r)).clamp(0.0, 1.0)).powf(1.25);
    let look_at = dir * (r * nadir_blend);
    let up = if dir.y.abs() > 0.98 { Vec3::X } else { Vec3::Y };
    let t = Transform::from_translation(eye).looking_at(look_at, up);

    // **Clip planes** track altitude so we neither clip the near surface nor
    // waste depth precision on a fixed 5 km near plane.
    let near = (alt * 0.04).clamp(0.5, 400.0);
    let far = (orbit.dist + 3.0 * r).max(near * 200.0);

    if let Ok((mut c, mut proj)) = cam.single_mut() {
        *c = t;
        if let Projection::Perspective(ref mut p) = *proj {
            p.near = near;
            p.far = far;
            // Slightly wider FOV when close so a region window fills the glass.
            p.fov = (45.0 + 12.0 * nadir_blend).to_radians();
        }
    }
    if let Ok(mut s) = light.single_mut() {
        if sun.headlight {
            s.rotation = t.rotation * Quat::from_rotation_y(0.55) * Quat::from_rotation_x(-0.45);
        } else {
            // Bevy's PBR uses `transform.back()` as direction_to_light, so we
            // look toward the planet (−sun) and back points at the sun.
            let to_sun = sun_world_dir(&sun);
            *s = Transform::default().looking_to(-to_sun, Vec3::Y);
        }
    }
}

fn sun_world_dir(s: &Sun) -> Vec3 {
    let yf = (s.day as f64 / 365.25).rem_euclid(1.0);
    let df = (s.hour as f64 / 24.0).rem_euclid(1.0);
    let [x, y, z] = Planet::EARTH.sun_direction_world(df, yf);
    Vec3::new(x as f32, y as f32, z as f32)
}

/// Cursor ray → sea-level sphere hit → surface direction. On exaggerated tall
/// land the picked cell can be a cell or two off at grazing angles — the honest
/// limit of picking against a datum sphere.
fn pick_direction(camera: &Camera, gt: &GlobalTransform, cursor: Vec2, r: f32) -> Option<[f64; 3]> {
    let ray = camera.viewport_to_world(gt, cursor).ok()?;
    let (o, d) = (ray.origin, *ray.direction);
    let b = o.dot(d);
    let disc = b * b - (o.dot(o) - r * r);
    if disc < 0.0 {
        return None;
    }
    let t = -b - disc.sqrt();
    if t <= 0.0 {
        return None;
    }
    let p = (o + d * t).normalize();
    Some([p.x as f64, p.y as f64, p.z as f64])
}

/// Everything the cursor is over, in both registers a session speaks: the
/// `(face, level, i, j)` a beacon names, and lat/lon for human intuition.
fn current_pick(
    windows: &Query<&Window, With<bevy::window::PrimaryWindow>>,
    cam: &Query<(&Camera, &GlobalTransform), With<ExploreCam>>,
    frame: &Frame,
    cov: &Coverage,
    water: &WaterField,
) -> Option<sighting::Pick> {
    let cursor = windows.iter().next()?.cursor_position()?;
    let (camera, gt) = cam.iter().next()?;
    let dir = pick_direction(camera, gt, cursor, radius_km())?;
    let cc = CubeCoord::from_unit(dir);
    let level = frame.req.level;
    let (face, i, j, _) = cc.cell(level).to_face_ij();
    let f = face.index();
    // The pick reads the SAME tile the mesh was built from, so a reported
    // elevation can never drift from the one on screen. In window mode that
    // means translating into the window, and reporting nothing when the cursor
    // is outside it — an honest miss rather than a plausible number from the
    // wrong place.
    let (tile, ti, tj, nx) = match &frame.req.window {
        Some(w) => {
            // tiles[] is parallel to FOV panes (centre first).
            let mut hit = None;
            for (idx, p) in w.panes.iter().enumerate() {
                if p.face != f || i < p.oi || j < p.oj {
                    continue;
                }
                let (ti, tj) = ((i - p.oi) as usize, (j - p.oj) as usize);
                if ti >= p.nx || tj >= p.nx {
                    continue;
                }
                if let Some(tile) = frame.tiles.get(idx) {
                    hit = Some((tile, ti, tj, p.nx));
                    break;
                }
            }
            hit?
        }
        None => {
            let nx = 1usize << level;
            (frame.tiles.get(f as usize)?, i as usize, j as usize, nx)
        }
    };
    let elev_m = *tile.get(tj * nx + ti)?;
    let to_build = |c: u32| if level <= cov.level { c << (cov.level - level) } else { c >> (level - cov.level) };
    let (bi, bj) = (to_build(i), to_build(j));
    let g = cc.to_geo();
    Some(sighting::Pick {
        face: f,
        level,
        i,
        j,
        lat_deg: g.lat.to_degrees(),
        lon_deg: g.lon.to_degrees(),
        elev_m,
        water_m: water.depth_at(f, i, j, level),
        state: if matches!(frame.req.lens, Lens::Stage(_)) {
            BuildState::Unbuilt
        } else {
            cov.state_at_cell(f, bi, bj)
        },
        provisional: cov.flags_at_cell(f, bi, bj).provisional,
    })
}

#[allow(clippy::too_many_arguments)]
fn hud_update(
    time: Res<Time>,
    orbit: Res<Orbit>,
    sun: Res<Sun>,
    ex: Res<Explorer>,
    ident: Res<Ident>,
    ladder: Res<LadderRes>,
    cov: Res<CovRes>,
    water: Res<WaterRes>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform), With<ExploreCam>>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    let Ok(mut text) = hud.single_mut() else {
        return;
    };
    let Some(frame) = ex.frame.as_deref() else {
        text.0 = "reading the store...".into();
        return;
    };
    let r = radius_km();
    let alt = orbit.dist - r;
    let lens_short = match frame.req.lens {
        Lens::Present => "present".to_string(),
        Lens::Stage(i) => format!(
            "{:.2} Ga T_p{:.0}C {}/{}",
            ladder.0.ages_ga.get(i).copied().unwrap_or(0.0),
            ladder.0.tps.get(i).copied().unwrap_or(0.0),
            i + 1,
            ladder.0.len()
        ),
        Lens::Erosion(i) => format!(
            "erosion e{} {}/{}",
            frame.facts.stage_epoch.unwrap_or(0),
            i + 1,
            frame.chain.len()
        ),
        Lens::Replay(n) => format!("replay {n}"),
    };
    let window_chip = if frame.req.window.is_some() {
        "CLOSE-IN"
    } else {
        "WHOLE"
    };
    let surface_chip = {
        let view = frame.req.level;
        let coarse: usize = frame
            .facts
            .tier_cells
            .iter()
            .filter(|(&t, _)| t < view)
            .map(|(_, &n)| n)
            .sum();
        let at: usize = frame
            .facts
            .tier_cells
            .get(&view)
            .copied()
            .unwrap_or(0);
        let cells = frame.facts.cells.max(1);
        if frame.facts.tier_cells.is_empty() || frame.facts.prior_fallback_frac > 0.95 {
            "prior (no eroded surface here)".into()
        } else if coarse * 100 / cells > 5 && at * 100 / cells > 5 {
            "mixed grain".into()
        } else if at * 100 / cells >= 50 {
            format!("eroded at L{view}")
        } else {
            "coarse cover".into()
        }
    };

    let dir = Vec3::new(
        orbit.pitch.cos() * orbit.yaw.cos(),
        orbit.pitch.sin(),
        orbit.pitch.cos() * orbit.yaw.sin(),
    );
    let geo = CubeCoord::from_unit([dir.x as f64, dir.y as f64, dir.z as f64]).to_geo();
    let place = format!(
        "{:.0} km  {:.1}{} {:.1}{}",
        alt,
        geo.lat.to_degrees().abs(),
        if geo.lat >= 0.0 { "N" } else { "S" },
        geo.lon.to_degrees().abs(),
        if geo.lon >= 0.0 { "E" } else { "W" },
    );
    let pick_line = match current_pick(&windows, &cam, frame, &cov.0, &water.0) {
        Some(p) => format!(
            "F{} L{} i={} j={}  {:+.0} m  [{}]{}",
            p.face,
            p.level,
            p.i,
            p.j,
            p.elev_m - frame.facts.sea_m,
            p.state.label(),
            if p.provisional { " PROV" } else { "" },
        ),
        None => "(point at the planet)".into(),
    };
    let view_line = format!(
        "L{}  {}  {}  paint {}  x{:.0}{}",
        frame.req.level,
        window_chip,
        lens_short,
        frame.req.paint.name(),
        frame.req.exag,
        if ex.inflight { "  UPDATING" } else { "" },
    );
    let beacon_val = match &ident.demand.demand.beacon {
        Some(b) => format!(
            "f{} L{} ({},{}) {}x{} tiles",
            b.face, b.level, b.oi, b.oj, b.tiles, b.tiles
        ),
        None => "none".into(),
    };

    // --- human chrome (default) ----------------------------------------------
    if ex.hud_level == 0 {
        let mut s = String::new();
        s.push_str(&capture::bed_status_block(ex.erosion_census));
        s.push('\n');
        s.push_str(&capture::row("VIEW", &view_line));
        s.push('\n');
        s.push_str(&capture::row("SEEN", &surface_chip));
        s.push('\n');
        s.push_str(&capture::row("PLACE", &place));
        s.push('\n');
        s.push_str(&capture::row("PICK", &pick_line));
        s.push('\n');
        s.push_str(&capture::row("BEACON", &beacon_val));
        s.push('\n');
        if ex.show_keys {
            s.push_str(&capture::key_legend());
        } else {
            s.push_str(&capture::row("?", "keys"));
            s.push('\n');
        }
        if time.elapsed_secs() < ex.notice_until {
            s.push('\n');
            s.push_str(&ex.notice);
        }
        text.0 = s;
        return;
    }

    // --- minimal (paint/lens only) -------------------------------------------
    if ex.hud_level == 2 {
        let mut s = format!(
            "{}\n[{}] {}   H overlay mode",
            capture::bed_status_block(ex.erosion_census)
                .lines()
                .next()
                .unwrap_or("eroded land"),
            frame.req.paint.name(),
            lens_short
        );
        if ex.show_keys {
            s.push_str(&capture::key_legend());
        }
        if time.elapsed_secs() < ex.notice_until {
            s.push('\n');
            s.push_str(&ex.notice);
        }
        text.0 = s;
        return;
    }

    // --- full debug dump (former default) ------------------------------------
    let mut s = String::new();
    s.push_str(&capture::bed_status_block(ex.erosion_census));
    s.push_str("\n[DEBUG DUMP - H for compact chrome]\n");
    for block in [
        hud::header(&ident.name, ident.seed, frame, &ladder.0, &cov.0, ex.inflight),
        hud::census(frame, &cov.0),
    ] {
        for line in block.lines() {
            s.push_str(&wrap(line, HUD_COLS, "         "));
            s.push('\n');
        }
    }

    if let Lens::Stage(i) = frame.req.lens {
        s.push_str(&hud::timeline(&ladder.0, i, 64));
        s.push('\n');
        s.push_str(&hud::craton_line(&frame.facts));
        s.push('\n');
    }
    if let Lens::Erosion(i) = frame.req.lens {
        s.push_str(&hud::chain_timeline(&frame.chain, i));
        s.push('\n');
    }
    if let Lens::Replay(_) = frame.req.lens {
        s.push_str(&format!(
            "replay frame {}/{} -- one mechanism with `vivarium watch --replay`, ordered by root LANDING time\n",
            ex.replay_frame + 1,
            ex.replay_frames.len().max(1)
        ));
    }

    let dir = Vec3::new(
        orbit.pitch.cos() * orbit.yaw.cos(),
        orbit.pitch.sin(),
        orbit.pitch.cos() * orbit.yaw.sin(),
    );
    let geo = CubeCoord::from_unit([dir.x as f64, dir.y as f64, dir.z as f64]).to_geo();
    s.push_str(&format!(
        "alt {alt:.0} km  centre {:.1}{} {:.1}{}  |  ",
        geo.lat.to_degrees().abs(),
        if geo.lat >= 0.0 { "N" } else { "S" },
        geo.lon.to_degrees().abs(),
        if geo.lon >= 0.0 { "E" } else { "W" },
    ));
    match current_pick(&windows, &cam, frame, &cov.0, &water.0) {
        Some(p) => s.push_str(&format!(
            "pick F{} L{} i={} j={}  {:.2}{} {:.2}{}  {:+.0} m rel  water {:.2} m  [{}]{}\n",
            p.face,
            p.level,
            p.i,
            p.j,
            p.lat_deg.abs(),
            if p.lat_deg >= 0.0 { "N" } else { "S" },
            p.lon_deg.abs(),
            if p.lon_deg >= 0.0 { "E" } else { "W" },
            p.elev_m - frame.facts.sea_m,
            p.water_m,
            p.state.label(),
            if p.provisional { " PROVISIONAL" } else { "" },
        )),
        None => s.push_str("pick: point at the planet\n"),
    }

    if !sun.headlight {
        let yf = (sun.day as f64 / 365.25).rem_euclid(1.0);
        let df = (sun.hour as f64 / 24.0).rem_euclid(1.0);
        let d = Planet::EARTH.sun_direction_world(df, yf);
        s.push_str(&format!(
            "sun: day {:.0}/365 {:04.1}h  subsolar lon {:+.0} lat {:+.1} deg (the real ephemeris)\n",
            sun.day,
            sun.hour,
            d[2].atan2(d[0]).to_degrees(),
            Planet::EARTH.solar_declination(yf).to_degrees()
        ));
    } else {
        s.push_str("sun: HEADLIGHT -- a view convenience, no day/night claim. Y for the real ephemeris\n");
    }

    let unmodelled = hud::unmodelled(frame, &ladder.0, &cov.0);
    s.push('\n');
    s.push_str(hud::honesty(&frame.roots).trim_end());
    s.push_str("\n\nNOT MODELLED (so your eye is not chasing an absence):\n");
    for line in &unmodelled {
        s.push_str("  - ");
        s.push_str(&wrap(line, HUD_COLS, "    "));
        s.push('\n');
    }
    s.push_str("\nDEPICTION -- what is on screen without a world referent ( #norm-no-depiction-without-referent ):\n");
    for line in hud::depiction(frame, sun.headlight) {
        s.push_str("  - ");
        s.push_str(&wrap(&line, HUD_COLS, "    "));
        s.push('\n');
    }
    s.push('\n');
    if ex.show_keys {
        s.push_str(&capture::key_legend());
    } else {
        s.push_str("H: human/debug/minimal  ·  C capture  ·  ? keys\n");
    }

    if time.elapsed_secs() < ex.notice_until {
        s.push_str("\n\n");
        s.push_str(&ex.notice);
    }
    text.0 = s;
}

/// Cheap word wrap — Bevy's text node will not do it for us at a width we can
/// predict, and a HUD line that runs off the window is a HUD line nobody reads.
fn wrap(text: &str, width: usize, indent: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    let mut col = 0usize;
    for w in text.split_whitespace() {
        if col > 0 && col + 1 + w.len() > width {
            out.push('\n');
            out.push_str(indent);
            col = indent.len();
        } else if col > 0 {
            out.push(' ');
            col += 1;
        }
        out.push_str(w);
        col += w.len();
    }
    out
}

/// **C** — capture pair (udon + png) + classic sighting md.
#[allow(clippy::too_many_arguments)]
fn capture_sighting(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    orbit: Res<Orbit>,
    sun: Res<Sun>,
    mut ex: ResMut<Explorer>,
    ident: Res<Ident>,
    ladder: Res<LadderRes>,
    cov: Res<CovRes>,
    water: Res<WaterRes>,
    mut pending: ResMut<PendingShot>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform), With<ExploreCam>>,
) {
    if !keys.just_pressed(KeyCode::KeyC) {
        return;
    }
    let Some(frame) = ex.frame.as_deref() else {
        return;
    };
    let dir = Vec3::new(
        orbit.pitch.cos() * orbit.yaw.cos(),
        orbit.pitch.sin(),
        orbit.pitch.cos() * orbit.yaw.sin(),
    );
    let geo = CubeCoord::from_unit([dir.x as f64, dir.y as f64, dir.z as f64]).to_geo();
    let stage = match frame.req.lens {
        Lens::Stage(i) => Some(sighting::StagePosition {
            idx: i,
            total: ladder.0.len(),
            age_ga: ladder.0.ages_ga.get(i).copied().unwrap_or(0.0),
            tp_c: ladder.0.tps.get(i).copied().unwrap_or(0.0),
            built: ladder.0.built.get(i).copied().unwrap_or(false),
            playing: ex.playing,
        }),
        _ => None,
    };
    let vantage = sighting::Vantage {
        world_name: ident.name.clone(),
        seed: ident.seed,
        world_dir: ident.dir.clone(),
        centre_lat_deg: geo.lat.to_degrees(),
        centre_lon_deg: geo.lon.to_degrees(),
        altitude_km: orbit.dist - radius_km(),
        pick: current_pick(&windows, &cam, frame, &cov.0, &water.0),
        stage,
        sun_day: sun.day,
        sun_hour: sun.hour,
        headlight: sun.headlight,
    };
    let unmodelled = hud::unmodelled(frame, &ladder.0, &cov.0);
    let depiction = hud::depiction(frame, sun.headlight);
    let (notice, shot) = match capture::write(
        frame,
        &vantage,
        ex.erosion_census,
        Some(&ident.demand),
        &unmodelled,
        &depiction,
    ) {
        Ok((info, png)) => {
            println!("[explore] capture written: {}", info.display());
            println!("[explore]   screenshot → {}", png.display());
            (
                format!(
                    "CAPTURED  {}\n  + {}\n(C again anytime · agent: last files in captures/)",
                    info.display(),
                    png.display()
                ),
                Some(png),
            )
        }
        Err(e) => (format!("CAPTURE FAILED: {e}"), None),
    };
    ex.notice = notice;
    ex.notice_until = time.elapsed_secs() + 8.0;
    pending.0 = shot;
}

/// The screenshot is taken the frame *after* the dump, so the notice is not
/// required in the picture.
fn take_pending_shot(mut commands: Commands, mut pending: ResMut<PendingShot>, mut skip: Local<bool>) {
    let Some(path) = pending.0.clone() else {
        *skip = false;
        return;
    };
    if !*skip {
        *skip = true;
        return;
    }
    commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    pending.0 = None;
    *skip = false;
}

/// `VIVARIUM_SHOT` — wait for the first frame, write the **same capture pair as C**
/// (`captures/*-vivarium-info.v0.1.0.udon` + `.png`), then exit.
///
/// Value forms:
/// - `1` / `true` / `yes` / `auto` → stamp under world `captures/`
/// - a filesystem path (contains `/` or ends in `.png`) → PNG at that path;
///   udon still lands in `captures/` and records the actual PNG path
///
/// Pair with `VIVARIUM_STAGE=<i>`, `VIVARIUM_EROSION=<i>`, `--paint MODE`.
/// Optional `VIVARIUM_SHOT_DELAY=<secs>` (default **4.0**): settle after the first
/// frame (Metal black-capture specimen, 2026-07-29).
#[allow(clippy::too_many_arguments)]
fn autoshot(
    time: Res<Time>,
    orbit: Res<Orbit>,
    sun: Res<Sun>,
    mut ex: ResMut<Explorer>,
    ident: Res<Ident>,
    ladder: Res<LadderRes>,
    cov: Res<CovRes>,
    water: Res<WaterRes>,
    mut pending: ResMut<PendingShot>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform), With<ExploreCam>>,
    mut commands: Commands,
    mut armed_at: Local<Option<f32>>,
    mut shot: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(raw) = std::env::var_os("VIVARIUM_SHOT") else {
        return;
    };
    let raw = raw.to_string_lossy();
    if raw.is_empty() {
        return;
    }
    let delay = std::env::var("VIVARIUM_SHOT_DELAY")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(4.0)
        .max(0.5);
    let t = time.elapsed_secs();
    if armed_at.is_none() && ex.frame.is_some() {
        *armed_at = Some(t);
    }
    let Some(t0) = *armed_at else {
        return;
    };
    if !*shot && t > t0 + delay {
        let Some(frame) = ex.frame.as_deref() else {
            return;
        };
        let dir = Vec3::new(
            orbit.pitch.cos() * orbit.yaw.cos(),
            orbit.pitch.sin(),
            orbit.pitch.cos() * orbit.yaw.sin(),
        );
        let geo = CubeCoord::from_unit([dir.x as f64, dir.y as f64, dir.z as f64]).to_geo();
        let stage = match frame.req.lens {
            Lens::Stage(i) => Some(sighting::StagePosition {
                idx: i,
                total: ladder.0.len(),
                age_ga: ladder.0.ages_ga.get(i).copied().unwrap_or(0.0),
                tp_c: ladder.0.tps.get(i).copied().unwrap_or(0.0),
                built: ladder.0.built.get(i).copied().unwrap_or(false),
                playing: ex.playing,
            }),
            _ => None,
        };
        let vantage = sighting::Vantage {
            world_name: ident.name.clone(),
            seed: ident.seed,
            world_dir: ident.dir.clone(),
            centre_lat_deg: geo.lat.to_degrees(),
            centre_lon_deg: geo.lon.to_degrees(),
            altitude_km: orbit.dist - radius_km(),
            pick: current_pick(&windows, &cam, frame, &cov.0, &water.0),
            stage,
            sun_day: sun.day,
            sun_hour: sun.hour,
            headlight: sun.headlight,
        };
        let unmodelled = hud::unmodelled(frame, &ladder.0, &cov.0);
        let depiction = hud::depiction(frame, sun.headlight);
        // Optional explicit PNG path for scripts; else stamp under captures/.
        let png_override = {
            let looks_like_path = raw.contains('/')
                || raw.contains('\\')
                || raw.ends_with(".png")
                || raw.starts_with('.');
            let token = matches!(raw.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "auto");
            if looks_like_path && !token {
                Some(PathBuf::from(raw.as_ref()))
            } else {
                None
            }
        };
        match capture::write_with_png(
            frame,
            &vantage,
            ex.erosion_census,
            Some(&ident.demand),
            &unmodelled,
            &depiction,
            png_override,
        ) {
            Ok((info, png)) => {
                eprintln!(
                    "[explore] SHOT capture {} (settled {delay:.1}s after first frame)",
                    info.display()
                );
                eprintln!("[explore]   screenshot → {}", png.display());
                ex.notice = format!("AUTOSHOT  {}\n  + {}", info.display(), png.display());
                ex.notice_until = time.elapsed_secs() + 8.0;
                // Immediate screenshot (no notice frame required for agent runs).
                let _ = pending.0.take();
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(png));
                *shot = true;
            }
            Err(e) => {
                eprintln!("[explore] SHOT capture failed: {e}");
                *shot = true; // exit anyway so agents do not hang
            }
        }
    }
    if *shot && t > t0 + delay + 1.6 {
        exit.write(AppExit::Success);
    }
}
