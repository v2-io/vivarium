//! vivarium-app — a 2D debug *view* over `vivarium-core`.
//!
//! Deliberately thin and deliberately ugly. Two jobs only: (1) advance the
//! simulation on a fixed timestep and (2) draw the current world state. It owns
//! no simulation logic — every rule lives in the core. If you find yourself
//! wanting to put a *mechanic* here, that is the signal it belongs in
//! `vivarium-core` instead.
//!
//! Drawing is done with **gizmos** (immediate-mode debug shapes) rather than
//! sprites. For a debug view that is the right tool: no textures, no assets, no
//! per-agent entities to keep in sync — just redraw the world from core state
//! every frame. Sprites and real art come later, in their own view, when the
//! visual indulgence is the point.
//!
//! Why a *fixed* timestep: the simulation is deterministic in step-count, not in
//! wall-clock. Rendering runs at whatever framerate the machine offers; the
//! world only ever advances in identical discrete steps, or the
//! tether-to-truth property (vivarium-core's doc) quietly breaks.

use bevy::prelude::*;
use vivarium_core::World;

/// The simulation, wrapped so Bevy can hold it as a resource. The single source
/// of truth on screen.
#[derive(Resource)]
struct Sim(World);

const FIXED_HZ: f64 = 30.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vivarium — debug view".into(),
                resolution: (900u32, 900u32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.09)))
        .insert_resource(Sim(World::new(0x00C0FFEE, 24)))
        // Fixed simulation rate, decoupled from render framerate.
        .insert_resource(Time::<Fixed>::from_hz(FIXED_HZ))
        .add_systems(Startup, spawn_camera)
        .add_systems(FixedUpdate, step_simulation)
        // Redraw every frame from current world state.
        .add_systems(Update, draw_world)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Advance the world exactly one fixed step.
fn step_simulation(mut sim: ResMut<Sim>) {
    sim.0.step(1.0 / FIXED_HZ as f32);
}

/// Draw the world: the play boundary, then every agent as a circle tinted by its
/// `need`. Pure function of world state — nothing here decides anything.
fn draw_world(sim: Res<Sim>, mut gizmos: Gizmos) {
    let world = &sim.0;

    // The play boundary, so the space the agents live in is legible.
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::splat(world.bound * 2.0),
        Color::srgb(0.25, 0.27, 0.32),
    );

    for agent in &world.agents {
        gizmos.circle_2d(
            Isometry2d::from_translation(Vec2::new(agent.pos[0], agent.pos[1])),
            7.0,
            need_color(agent.need),
        );
    }
}

/// Make the otherwise-invisible scalar `need` visible: cool when low, warm when
/// high. The first tiny instance of "interface is a view over the simulation" —
/// the number is real; the color is just how we choose to look at it.
fn need_color(need: f32) -> Color {
    Color::srgb(0.2 + 0.7 * need, 0.8 - 0.5 * need, 0.9 - 0.6 * need)
}
