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

use std::sync::RwLock;

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
///
/// ## Why the `RwLock`, and why every method takes `&self`
///
/// godot_voxel meshes chunks on worker threads and calls [`Self::generate_block`]
/// from them, concurrently, while the main thread issues edits ([`Self::dig`] /
/// [`Self::place`]) and steps. Two problems stack here:
///
///  - gdext forbids cross-thread access to its objects unless the
///    `experimental-threads` feature is on (it is — see Cargo.toml);
///  - even with that, a `&mut self` method (an edit) running while `&self`
///    methods (generation) run on other threads is an aliasing violation that
///    gdext's cell correctly rejects.
///
/// The fix keeps *core* pure and single-threaded — the lock is a *view* concern,
/// so it lives here, not in `vivarium-core`. The `World` sits behind a `RwLock`;
/// generation takes read locks (any number, in parallel), edits take the write
/// lock (briefly exclusive). Because the lock provides the interior mutability,
/// *every* method below takes `&self` — gdext only ever sees shared borrows, so
/// there is no exclusive-borrow conflict regardless of thread timing.
#[derive(GodotClass)]
#[class(base=Node)]
struct VivariumWorld {
    world: RwLock<World>,
    base: Base<Node>,
}

#[godot_api]
impl INode for VivariumWorld {
    fn init(base: Base<Node>) -> Self {
        // Same seed as the Bevy geology view, so the two render literally the same
        // eroded world and the comparison stays apples-to-apples.
        //
        // The world is now the *geology tier*: detail 2 (the 0.5 m voxel anchor),
        // a ~12 km landmass carved by stream-power erosion. All three knobs are
        // env-tunable so the scale can be swept without a recompile:
        //   VIVARIUM_DETAIL       voxels/unit              (default 2)
        //   VIVARIUM_REGION_HALF  half-extent in metres    (default 6000 → ~12 km)
        //   VIVARIUM_EPOCHS       erosion epochs at launch (default 70)
        // Defaults mirror spikes/bevy-voxel/src/main.rs exactly.
        let env_i32 = |k: &str, d: i32| {
            std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
        };
        let detail = env_i32("VIVARIUM_DETAIL", 2);
        let region_half = env_i32("VIVARIUM_REGION_HALF", 6_000);
        let epochs = env_i32("VIVARIUM_EPOCHS", 70).max(0) as u32;
        godot_print!(
            "[vivarium] generating eroded world (seed {:#x}, detail {detail}, \
             ±{region_half} m, {epochs} epochs)… this is the slow tier, ~seconds.",
            0x00C0_FFEE_u64,
        );
        let world = World::eroded(0x00C0_FFEE, 24, detail, region_half, epochs);
        godot_print!("[vivarium] world ready.");
        Self { world: RwLock::new(world), base }
    }

    /// A loud self-check that the FFI seam is alive: read real facts out of core
    /// and print them. If this appears in Godot's output, the bridge works.
    fn ready(&mut self) {
        let w = self.world.read().unwrap();
        let h = w.volume.surface_height(0, 0).unwrap_or(-1);
        godot_print!(
            "[vivarium] bridge live — seed {:#x}, detail {} (voxels/unit), {} agents; \
             surface at (0,0) is voxel y={}, material {}",
            w.seed,
            w.volume.detail(),
            w.agents.len(),
            h,
            w.volume.voxel(0, h, 0).0,
        );
    }
}

#[godot_api]
impl VivariumWorld {
    /// Material id of the voxel at `(x, y, z)`. `0` is air. The one call a voxel
    /// mesher needs; godot_voxel's custom generator will call this per cell.
    #[func]
    fn voxel_at(&self, x: i32, y: i32, z: i32) -> i32 {
        self.world.read().unwrap().volume.voxel(x, y, z).0 as i32
    }

    /// Fill an entire block in one call: the materials of every voxel in the
    /// box, returned as one byte per voxel (material id; air is 0). Index order
    /// is **x fastest, then y, then z** — `i = x + size.x * (y + size.y * z)` —
    /// which the generator script mirrors.
    ///
    /// `lod` is the level-of-detail the engine is asking for: at `lod = n` each
    /// output cell stands for a `2^n`-voxel cube, so we *point-sample* the world
    /// at stride `2^n` (cell → world `origin + cell * 2^n`). This is the seam
    /// that decouples **intrinsic** voxel resolution (what the world *is*) from
    /// **view** resolution (how finely it's drawn at this distance) — the engine
    /// asks for coarse data far away, fine data near. Point-sampling is the
    /// cheapest correct choice for our heightmap world: solidity is monotonic in
    /// depth, so a coarse sample never punches false holes (it can alias a thin
    /// ridge, which a dominant-material downsample would fix later if needed).
    ///
    /// This is the hot path: godot_voxel calls it once per chunk, from worker
    /// threads. It takes `&self` and only *reads* core, so concurrent calls are
    /// sound; bulk-returning keeps the FFI crossing count at one per chunk.
    #[func]
    fn generate_block(&self, origin: Vector3i, size: Vector3i, lod: i32) -> PackedByteArray {
        let w = self.world.read().unwrap();
        let vol = &w.volume;
        let stride = 1 << lod.max(0);
        let mut out = PackedByteArray::new();
        out.resize((size.x * size.y * size.z) as usize);
        let slice = out.as_mut_slice();
        let mut i = 0;
        for z in 0..size.z {
            for y in 0..size.y {
                for x in 0..size.x {
                    let v = vol.voxel(
                        origin.x + x * stride,
                        origin.y + y * stride,
                        origin.z + z * stride,
                    );
                    slice[i] = v.0 as u8;
                    i += 1;
                }
            }
        }
        out
    }

    /// Topmost solid `y` in the generated column at `(x, z)`, or `-1` if empty.
    /// Lets the camera/controller spawn the player on the ground.
    #[func]
    fn surface_height(&self, x: i32, z: i32) -> i32 {
        self.world.read().unwrap().volume.surface_height(x, z).unwrap_or(-1)
    }

    /// Voxels per world unit (see `vivarium_core::voxel::Detail`). The view
    /// renders voxels at `1 / this` so the world looks the same physical size at
    /// any resolution, and converts physical↔voxel coordinates with it.
    #[func]
    fn voxels_per_unit(&self) -> i32 {
        self.world.read().unwrap().volume.detail()
    }

    /// Remove the voxel at `(x, y, z)` (set it to air) and persist the edit in
    /// core. Returns the material that was there, so the view can spawn the
    /// right debris/particle. This is the write half of the seam.
    #[func]
    fn dig(&self, x: i32, y: i32, z: i32) -> i32 {
        self.world.write().unwrap().volume.set_voxel(x, y, z, Voxel::AIR).0 as i32
    }

    /// Place `material` at `(x, y, z)`, persisted in core. Returns the previous
    /// material.
    #[func]
    fn place(&self, x: i32, y: i32, z: i32, material: i32) -> i32 {
        self.world
            .write()
            .unwrap()
            .volume
            .set_voxel(x, y, z, Voxel(material as u16))
            .0 as i32
    }

    /// Advance the simulation one fixed step. Called from a fixed-rate timer in
    /// the scene, not every frame, to preserve determinism.
    #[func]
    fn step(&self) {
        self.world.write().unwrap().step(1.0 / FIXED_HZ);
    }

    /// Number of agents — used by the view to size its pool of agent visuals.
    #[func]
    fn agent_count(&self) -> i32 {
        self.world.read().unwrap().agents.len() as i32
    }

    /// World-space position of agent `i` as a Godot `Vector3`. Core stores
    /// `[x, y, z]`; this is the one place that layout is translated to Godot's.
    #[func]
    fn agent_position(&self, i: i32) -> Vector3 {
        let p = self.world.read().unwrap().agents[i as usize].pos;
        Vector3::new(p[0], p[1], p[2])
    }
}
