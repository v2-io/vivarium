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

use std::path::PathBuf;
use std::sync::RwLock;

use godot::prelude::*;
use vivarium_core::voxel::{Voxel, Volume, WORLDGEN_VERSION};
use vivarium_core::World;

struct VivariumExtension;

#[gdextension]
unsafe impl ExtensionLibrary for VivariumExtension {}

/// Fixed simulation rate. The world advances in identical discrete steps so the
/// tether-to-truth (determinism) property holds regardless of render framerate —
/// the same reasoning as the Bevy view's `FIXED_HZ`.
const FIXED_HZ: f32 = 30.0;

/// Build the eroded world, but reuse a frozen one from disk when we already paid
/// the cost. The erosion+hydrology worldgen is the *slow tier* — minutes for a
/// full region — and while iterating on the **view** the world never changes, so
/// recomputing it every launch is pure waste. Core gives us a faithful byte image
/// of a generated [`Volume`] ([`Volume::to_bytes`] / [`from_bytes`]); the caching
/// — *where* the bytes live, *when* to trust them — is a view concern and lives
/// here, exactly like the `RwLock`.
///
/// Correctness rests on core, not on us: the cache key includes every worldgen
/// input *and* [`WORLDGEN_VERSION`], and `from_bytes` independently re-checks that
/// version and the buffer's integrity. So a stale or corrupt file can only ever
/// trigger a regenerate, never load a wrong world. Rebuilding the [`World`] from
/// the cached volume via [`World::from_volume`] reproduces agent placement
/// identically (same seed), so a reloaded world is indistinguishable from a freshly
/// generated one.
///
/// Knobs: `VIVARIUM_NO_CACHE=1` always regenerates (and does not write);
/// `VIVARIUM_CACHE_DIR` overrides the location (default: a `vivarium-worldcache/`
/// under the OS temp dir — persists across launches, easy to wipe).
fn load_or_generate_world(
    seed: u64,
    n_agents: usize,
    detail: i32,
    region_half: i32,
    epochs: u32,
    fine_cell: f32,
    fine_epochs: u32,
) -> World {
    let generate = || {
        godot_print!(
            "[vivarium] generating eroded world (seed {seed:#x}, detail {detail}, \
             ±{region_half} m, {epochs} coarse epochs, fine {fine_cell} m × {fine_epochs})… \
             slow tier, minutes.",
        );
        World::eroded(seed, n_agents, detail, region_half, epochs, fine_cell, fine_epochs)
    };

    if std::env::var_os("VIVARIUM_NO_CACHE").is_some() {
        return generate();
    }

    let path = world_cache_path(seed, detail, region_half, epochs, fine_cell, fine_epochs);

    // Try to reload. A miss (no file), a stale version, or any corruption all fall
    // through to regeneration — `from_bytes` is the integrity gate.
    if let Ok(bytes) = std::fs::read(&path) {
        match Volume::from_bytes(&bytes) {
            Some(volume) => {
                godot_print!(
                    "[vivarium] worldgen cache HIT ({} KiB) — reloaded {}, skipping the slow tier.",
                    bytes.len() / 1024,
                    path.display(),
                );
                return World::from_volume(seed, n_agents, volume);
            }
            None => godot_warn!(
                "[vivarium] worldgen cache at {} is stale or unreadable — regenerating.",
                path.display(),
            ),
        }
    }

    let world = generate();
    match write_world_cache(&path, &world.volume) {
        Ok(()) => godot_print!(
            "[vivarium] worldgen cached → {} (next launch reloads instantly).",
            path.display(),
        ),
        Err(e) => godot_warn!("[vivarium] could not write worldgen cache {}: {e}", path.display()),
    }
    world
}

/// Cache file for a given worldgen. The name spells out every input plus
/// [`WORLDGEN_VERSION`], so distinct worlds (and worlds from a changed worldgen)
/// never collide — and the file is self-describing at a glance in the cache dir.
fn world_cache_path(
    seed: u64,
    detail: i32,
    region_half: i32,
    epochs: u32,
    fine_cell: f32,
    fine_epochs: u32,
) -> PathBuf {
    let dir = std::env::var_os("VIVARIUM_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("vivarium-worldcache"));
    let name = format!(
        "viv_s{seed:x}_d{detail}_r{region_half}_e{epochs}_fc{fine_cell:.1}_fe{fine_epochs}_v{WORLDGEN_VERSION}.bin",
    );
    dir.join(name)
}

/// Write the volume image atomically: serialize to a sibling `.tmp` then rename
/// into place, so an interrupted write never leaves a half-file that a later run
/// would have to detect and discard.
fn write_world_cache(path: &PathBuf, volume: &Volume) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("bin.tmp");
    std::fs::write(&tmp, volume.to_bytes())?;
    std::fs::rename(&tmp, path)
}

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
        //   VIVARIUM_EPOCHS       coarse (16 m) erosion epochs   (default 70)
        //   VIVARIUM_FINE_CELL    finer-erosion cell, metres     (default 8)
        //   VIVARIUM_FINE_EPOCHS  finer-erosion epochs, 0 = off  (default 0)
        // Defaults mirror spikes/bevy-voxel/src/main.rs (fine pass off by default —
        // it's the experiment: VIVARIUM_FINE_EPOCHS=8 to carve real sub-16 m
        // drainage instead of fractal noise; cost ~(span/fine_cell)² per fine epoch).
        let env_i32 = |k: &str, d: i32| {
            std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
        };
        let env_f32 = |k: &str, d: f32| {
            std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
        };
        let detail = env_i32("VIVARIUM_DETAIL", 2);
        let region_half = env_i32("VIVARIUM_REGION_HALF", 6_000);
        let epochs = env_i32("VIVARIUM_EPOCHS", 70).max(0) as u32;
        let fine_cell = env_f32("VIVARIUM_FINE_CELL", 8.0);
        let fine_epochs = env_i32("VIVARIUM_FINE_EPOCHS", 0).max(0) as u32;
        let world = load_or_generate_world(
            0x00C0_FFEE, 24, detail, region_half, epochs, fine_cell, fine_epochs,
        );
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
        // Vertical half-cell offset: the engine renders each sample as a cube
        // extending *upward* by `stride` from its min corner, so corner-sampling
        // (`y*stride`) puts the topmost solid cube's top face up to `stride-1`
        // voxels ABOVE the true surface — a systematic upward bias that grows with
        // LOD and shows as bright "bulge" steps where a coarse LOD meets a finer one
        // (the LOD seams). Sampling the cube's vertical CENTER (`+ stride/2`) makes
        // the solid/air threshold straddle the surface, cutting the worst-case error
        // from `stride` to `±stride/2` and removing the one-directional bulge.
        // Horizontal is left at the corner on purpose: coarse and fine LODs then
        // still sample the same columns at shared boundaries (centering x/z would
        // trade the vertical seam for a half-cell horizontal one). LOD0 (stride 1)
        // is unaffected — `stride/2 == 0`. Monotonic-depth solidity is preserved, so
        // no false holes.
        let y_off = stride >> 1;
        let mut out = PackedByteArray::new();
        out.resize((size.x * size.y * size.z) as usize);
        let slice = out.as_mut_slice();
        let mut i = 0;
        // Water is rendered as a *range* of palette indices, not the flat WATER id,
        // so the view can read depth and flow at a glance: deeper column → higher
        // depth level (darker blue in the palette), faster flow → higher speed level
        // (toward white). Encoding (mirrored in main.gd's palette builder):
        //   index = WATER_BASE + depth_level*SPEED_LEVELS + speed_level
        // depth_level 0..DEPTH_LEVELS-1, speed_level 0..SPEED_LEVELS-1.
        const WATER_ID: u8 = 4;
        const WATER_BASE: u8 = 64;
        const DEPTH_LEVELS: i32 = 24;
        const SPEED_LEVELS: i32 = 8;
        for z in 0..size.z {
            for y in 0..size.y {
                for x in 0..size.x {
                    let (wx, wz) = (origin.x + x * stride, origin.z + z * stride);
                    let v = vol.voxel(wx, origin.y + y * stride + y_off, wz);
                    slice[i] = if v.0 as u8 == WATER_ID {
                        // ~2 m per depth level (0.5 m voxels → /4); ~0.25 m/s per
                        // speed level.
                        let dl = (vol.water_depth_voxels(wx, wz) / 4).clamp(0, DEPTH_LEVELS - 1);
                        let sl = (vol.water_speed(wx, wz) * 4.0) as i32;
                        let sl = sl.clamp(0, SPEED_LEVELS - 1);
                        WATER_BASE + (dl * SPEED_LEVELS + sl) as u8
                    } else {
                        v.0 as u8
                    };
                    i += 1;
                }
            }
        }
        out
    }

    /// Topmost solid `y` in the generated column at `(x, z)`, or `-1` if empty.
    /// Lets the camera/controller spawn the player on the ground.
    ///
    /// NB: under the ocean this is the *seabed* (the topmost solid is below the
    /// waterline), so a camera that follows this alone flies underwater. Clamp the
    /// eye height to at least [`Self::sea_level`] — see the benchmark flight.
    #[func]
    fn surface_height(&self, x: i32, z: i32) -> i32 {
        self.world.read().unwrap().volume.surface_height(x, z).unwrap_or(-1)
    }

    /// Sea level, in voxels. The waterline the ocean fills to; a flying camera
    /// should stay above `max(surface_height, sea_level)` so it never renders the
    /// world from underneath the water surface.
    #[func]
    fn sea_level(&self) -> i32 {
        self.world.read().unwrap().volume.sea_level()
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
