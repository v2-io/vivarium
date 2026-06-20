//! vivarium-app — a 2D debug *view* over `vivarium-core`.
//!
//! This is deliberately thin and deliberately ugly: colored rectangles, no art.
//! Its only jobs are (1) advance the simulation on a fixed timestep and (2) draw
//! the current world state. It owns no simulation logic — every rule lives in
//! the core. If you find yourself wanting to put a *mechanic* here, that's the
//! signal it belongs in `vivarium-core` instead.
//!
//! Why a *fixed* timestep: the simulation is deterministic in step-count, not in
//! wall-clock. Rendering can run at whatever framerate the machine offers, but
//! the world must only ever advance in identical discrete steps, or the
//! tether-to-truth property (vivarium-core's doc) quietly breaks.

use bevy::prelude::*;
use vivarium_core::World;

/// The simulation, wrapped so Bevy can hold it as a resource. The view reads
/// this; it is the single source of truth on screen.
#[derive(Resource)]
struct Sim(World);

/// Ties a rendered entity back to its agent's index in the world.
#[derive(Component)]
struct AgentIndex(usize);

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
        .add_systems(Startup, spawn_view)
        .add_systems(FixedUpdate, step_simulation)
        // Sync the view every frame so motion looks smooth between sim steps.
        .add_systems(Update, sync_view)
        .run();
}

/// Spawn the camera and one rectangle per agent.
fn spawn_view(mut commands: Commands, sim: Res<Sim>) {
    commands.spawn(Camera2d);
    for (i, agent) in sim.0.agents.iter().enumerate() {
        commands.spawn((
            Sprite {
                color: need_color(agent.need),
                custom_size: Some(Vec2::splat(14.0)),
                ..default()
            },
            Transform::from_xyz(agent.pos[0], agent.pos[1], 0.0),
            AgentIndex(i),
        ));
    }
}

/// Advance the world exactly one fixed step.
fn step_simulation(mut sim: ResMut<Sim>) {
    sim.0.step(1.0 / FIXED_HZ as f32);
}

/// Pull each agent's position and need from the core into its sprite. The view
/// is a pure function of world state — nothing here decides anything.
fn sync_view(sim: Res<Sim>, mut query: Query<(&AgentIndex, &mut Transform, &mut Sprite)>) {
    for (idx, mut transform, mut sprite) in &mut query {
        let agent = &sim.0.agents[idx.0];
        transform.translation.x = agent.pos[0];
        transform.translation.y = agent.pos[1];
        sprite.color = need_color(agent.need);
    }
}

/// Make the otherwise-invisible scalar `need` visible: cool when low, warm when
/// high. The first tiny instance of "gameplay/interface is a view over the
/// simulation" — the number is real; the color is just how we look at it.
fn need_color(need: f32) -> Color {
    Color::srgb(0.2 + 0.7 * need, 0.8 - 0.5 * need, 0.9 - 0.6 * need)
}
