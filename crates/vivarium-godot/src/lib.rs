//! The Godot view over `vivarium-core`, and the bridge that makes it possible.
//!
//! ## What this is testing (the actual spike question)
//!
//! Choosing Godot over Bevy trades Bevy's "the sim and the view share one ECS"
//! elegance for Godot's far more mature native-voxel tooling — at the cost of an
//! **FFI seam** between a Rust simulation core and a C++ engine. The whole point
//! of the Godot spike is to feel how much that seam actually costs in practice.
//! This module is where the seam lives, so it is kept deliberately thin and
//! honest: Godot calls *down* into core for facts (`voxel_at`, `surface_height`)
//! and pushes edits *down* (`dig`), and core never knows Godot exists.
//!
//! ## Layering
//!
//! [`VivariumWorld`] is a Godot `Node` that *owns* a [`World`] and exposes a
//! small, typed surface to GDScript and to other (C++) extensions like
//! godot_voxel. Right now it is a bare bridge with a self-check in `ready()`;
//! the voxel terrain rendering and first-person controller land on top of this
//! once the seam itself is proven to work.

use godot::prelude::*;
use vivarium_core::voxel::Voxel;
use vivarium_core::World;

struct VivariumExtension;

#[gdextension]
unsafe impl ExtensionLibrary for VivariumExtension {}

/// Fixed simulation rate. The world advances in identical discrete steps so the
/// tether-to-truth (determinism) property holds regardless of render framerate —
/// the same reasoning as the Bevy view's `FIXED_HZ`.
const FIXED_HZ: f32 = 30.0;

/// The simulation, wrapped as a Godot node so a scene can hold it as the single
/// source of truth. Every other node reads the world through this one.
#[derive(GodotClass)]
#[class(base=Node)]
struct VivariumWorld {
    world: World,
    base: Base<Node>,
}

#[godot_api]
impl INode for VivariumWorld {
    fn init(base: Base<Node>) -> Self {
        // Same seed/agent-count as the Bevy debug view, so the two spikes look
        // at literally the same world and the comparison is apples-to-apples.
        Self { world: World::new(0x00C0_FFEE, 24), base }
    }

    /// A loud self-check that the FFI seam is alive: read real facts out of core
    /// and print them. If this appears in Godot's output, the bridge works.
    fn ready(&mut self) {
        let v = &self.world.volume;
        let h = v.surface_height(0, 0).unwrap_or(-1);
        godot_print!(
            "[vivarium] bridge live — seed {:#x}, {} agents; surface at (0,0) is y={}, \
             voxel just below is material {}",
            self.world.seed,
            self.world.agents.len(),
            h,
            v.voxel(0, h, 0).0,
        );
    }
}

#[godot_api]
impl VivariumWorld {
    /// Material id of the voxel at `(x, y, z)`. `0` is air. The one call a voxel
    /// mesher needs; godot_voxel's custom generator will call this per cell.
    #[func]
    fn voxel_at(&self, x: i32, y: i32, z: i32) -> i32 {
        self.world.volume.voxel(x, y, z).0 as i32
    }

    /// Topmost solid `y` in the generated column at `(x, z)`, or `-1` if empty.
    /// Lets the camera/controller spawn the player on the ground.
    #[func]
    fn surface_height(&self, x: i32, z: i32) -> i32 {
        self.world.volume.surface_height(x, z).unwrap_or(-1)
    }

    /// Remove the voxel at `(x, y, z)` (set it to air) and persist the edit in
    /// core. Returns the material that was there, so the view can spawn the
    /// right debris/particle. This is the write half of the seam.
    #[func]
    fn dig(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.world.volume.set_voxel(x, y, z, Voxel::AIR).0 as i32
    }

    /// Place `material` at `(x, y, z)`, persisted in core. Returns the previous
    /// material.
    #[func]
    fn place(&mut self, x: i32, y: i32, z: i32, material: i32) -> i32 {
        self.world
            .volume
            .set_voxel(x, y, z, Voxel(material as u16))
            .0 as i32
    }

    /// Advance the simulation one fixed step. Called from a fixed-rate timer in
    /// the scene, not every frame, to preserve determinism.
    #[func]
    fn step(&mut self) {
        self.world.step(1.0 / FIXED_HZ);
    }

    /// Number of agents — used by the view to size its pool of agent visuals.
    #[func]
    fn agent_count(&self) -> i32 {
        self.world.agents.len() as i32
    }

    /// World-space position of agent `i` as a Godot `Vector3`. Core stores
    /// `[x, y, z]`; this is the one place that layout is translated to Godot's.
    #[func]
    fn agent_position(&self, i: i32) -> Vector3 {
        let p = self.world.agents[i as usize].pos;
        Vector3::new(p[0], p[1], p[2])
    }
}
