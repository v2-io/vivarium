//! vivarium-bevy-voxel — the Bevy half of the engine spike.
//!
//! Renders the *same* deterministic `vivarium-core` world as the Godot spike,
//! but through `bevy_voxel_world`. The core is the single source of truth; this
//! is one disposable view over it. Kept deliberately close to the Godot spike's
//! choices (detail 4, the same material palette, overcast + distance fog) so the
//! two are an apples-to-apples comparison.
//!
//! This is a first cut: terrain + LOD + fog + a slowly drifting camera, with an
//! env-gated auto-screenshot for verification. First-person controls and
//! dig/place come next, mirroring the Godot build.

use std::sync::Arc;

use bevy::ecs::message::MessageWriter;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy_voxel_world::custom_meshing::{CHUNK_SIZE_F, CHUNK_SIZE_U};
// padded_chunk_shape_uniform and the VoxelWorld* / LodLevel / WorldVoxel types
// all come through the prelude.
use bevy_voxel_world::prelude::*;

use vivarium_core::voxel::Volume;

/// Voxels per world unit — matches the Godot spike's default so both views look
/// at the same physical world.
const DETAIL: i32 = 4;
const SEED: u64 = 0x00C0_FFEE;

/// Overcast sky / fog colour, matching the Godot spike.
const SKY: Color = Color::srgb(0.80, 0.82, 0.84);

/// The Bevy voxel world config: holds a shared, read-only handle to the core
/// volume and answers `bevy_voxel_world`'s per-chunk voxel queries from it.
#[derive(Resource, Clone)]
struct VivWorld {
    volume: Arc<Volume>,
}

impl Default for VivWorld {
    fn default() -> Self {
        Self { volume: Arc::new(Volume::with_detail(SEED, DETAIL)) }
    }
}

impl VoxelWorldConfig for VivWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        32
    }

    fn min_despawn_distance(&self) -> u32 {
        1
    }

    /// The seam to core. `bevy_voxel_world` calls this on worker threads, once
    /// per chunk; the inner closure is handed *world* voxel positions (already
    /// spaced by the chunk's LOD stride), so we just ask core for that voxel.
    /// View resolution is thus decoupled from intrinsic resolution exactly as in
    /// the Godot spike — the engine queries coarse positions far away.
    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let volume = Arc::clone(&self.volume);
        Box::new(move |_chunk_pos, _lod_level, _previous| {
            let volume = Arc::clone(&volume);
            Box::new(move |pos: IVec3, _previous_voxel| {
                let v = volume.voxel(pos.x, pos.y, pos.z);
                if v.is_air() {
                    WorldVoxel::Air
                } else {
                    WorldVoxel::Solid(v.0 as u8)
                }
            })
        })
    }

    /// Map vivarium material ids to texture-array indices. (First cut uses the
    /// default texture array; flat palette colours come next, like Godot.)
    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            1 => [0, 0, 0], // stone
            2 => [1, 1, 1], // dirt
            3 => [2, 2, 2], // grass
            4 => [3, 3, 3], // water
            _ => [0, 0, 0],
        })
    }

    // --- LOD: coarser meshes with distance, the whole point of the comparison.
    fn chunk_data_shape(&self, lod_level: LodLevel) -> UVec3 {
        padded_chunk_shape_uniform(CHUNK_SIZE_U / lod_level.max(1) as u32)
    }

    fn chunk_meshing_shape(&self, lod_level: LodLevel) -> UVec3 {
        padded_chunk_shape_uniform(CHUNK_SIZE_U / lod_level.max(1) as u32)
    }

    fn chunk_lod(
        &self,
        chunk_position: IVec3,
        _previous_lod: Option<LodLevel>,
        camera_position: Vec3,
    ) -> LodLevel {
        let camera_chunk = (camera_position / CHUNK_SIZE_F).floor();
        let distance = chunk_position.as_vec3().distance(camera_chunk);
        // LOD value doubles as the sample stride.
        if distance < 8.0 {
            1
        } else if distance < 16.0 {
            2
        } else if distance < 28.0 {
            4
        } else {
            8
        }
    }

    fn attach_chunks_to_root(&self) -> bool {
        false
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(SKY))
        .add_plugins(VoxelWorldPlugin::with_config(VivWorld::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (drift_camera, maybe_screenshot))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the camera on the surface column above the origin.
    let surface = Volume::with_detail(SEED, DETAIL).surface_height(0, 0).unwrap_or(120);
    let eye = Vec3::new(40.0, surface as f32 + 30.0, 40.0);
    let look = Vec3::new(0.0, surface as f32, 0.0);

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(eye).looking_at(look, Vec3::Y),
        VoxelWorldCamera::<VivWorld>::default(),
        // Distance fog into the overcast sky — the Bevy analogue of the Godot
        // Environment depth fog. start/end in world units (= voxels).
        DistanceFog {
            color: SKY,
            falloff: FogFalloff::Linear { start: 60.0 * DETAIL as f32, end: 110.0 * DETAIL as f32 },
            ..default()
        },
    ));

    // Overcast lighting: dim, shadowless key + strong flat ambient.
    commands.spawn((
        DirectionalLight { color: SKY, shadows_enabled: false, illuminance: 4000.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.3, -1.0, -0.2), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: SKY,
        brightness: 600.0,
        affects_lightmapped_meshes: true,
    });
}

/// Drift the camera slowly so a screenshot shows the world in motion-free but
/// non-trivial framing. Replaced by a real fly controller next.
fn drift_camera(time: Res<Time>, mut q: Query<&mut Transform, With<VoxelWorldCamera<VivWorld>>>) {
    if let Ok(mut t) = q.single_mut() {
        t.translation.x += time.delta_secs() * 8.0;
    }
}

/// Verification screenshot: when VIVARIUM_AUTOSHOT is set, wait a few seconds for
/// chunks to stream/mesh, capture to /tmp, and exit.
fn maybe_screenshot(
    mut commands: Commands,
    time: Res<Time>,
    mut taken: Local<bool>,
    mut writer: MessageWriter<AppExit>,
) {
    if std::env::var("VIVARIUM_AUTOSHOT").is_err() {
        return;
    }
    let t = time.elapsed_secs();
    // Capture once chunks have streamed/meshed...
    if !*taken && t >= 8.0 {
        *taken = true;
        let path = "/tmp/vivarium_bevy_shot.png".to_string();
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    }
    // ...then give the async GPU readback a moment to write the file before exit.
    if t >= 10.5 {
        writer.write(AppExit::Success);
    }
}
