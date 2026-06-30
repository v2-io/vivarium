//! World source: load a `vivarium-core` `Volume` (through the shared worldgen
//! cache, so Joseph's already-generated 12 km world reloads instantly) and sample
//! its *surface* per map cell. The navigator is surface-only — for each (col,row)
//! map cell at a chosen voxel stride it asks the core for the topmost solid voxel,
//! its material, and the standing-water column. Pure reads of `Volume`, so the
//! sampler is cheap and thread-safe.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::prelude::Resource;
use vivarium_core::voxel::{Volume, WORLDGEN_VERSION};
use vivarium_core::World;

// Worldgen parameters — matched to the godot bridge / bevy-iso so the on-disk
// cache file is SHARED. Identical inputs ⇒ this app reloads the very same frozen
// world the other views use, no regeneration. VIVARIUM_REGION_HALF shrinks it for
// fast iteration (a separate, quick cache file), exactly like the other adapters.
const SEED: u64 = 0x00C0_FFEE;
const DETAIL: i32 = 2; // voxels per metre (0.5 m anchor)
const DEFAULT_REGION_HALF_M: i32 = 6_000;
const EPOCHS: u32 = 70;
const FINE_CELL_M: f32 = 8.0;
const FINE_EPOCHS: u32 = 0;
const N_AGENTS: usize = 24;

/// Shared, immutable world handle. `Volume` is cheap to share; all sampling is
/// `&self`.
#[derive(Resource, Clone)]
pub struct WorldSource {
    pub volume: Arc<Volume>,
    pub detail: i32,
    pub sea_level: i32,
}

/// One sampled surface cell — the raw facts the renderer quantizes into a tile key.
#[derive(Clone, Copy)]
pub struct SurfaceCell {
    /// Topmost solid voxel height (voxels). `None` columns are reported at sea level.
    pub height_vox: i32,
    /// Material id of the surface voxel (vivarium-core `Voxel`).
    pub material: u16,
    /// Standing-water depth at the column, in voxels (0 = dry land).
    pub water_depth_vox: i32,
    /// Water flow speed (m/s). Sampled now; consumed by P2 flow shading (SPEC.md).
    #[allow(dead_code)]
    pub water_speed: f32,
    /// True when this column is open sea / lake (water over it).
    pub wet: bool,
}

impl WorldSource {
    pub fn load() -> Self {
        let region_half = region_half_m();
        let volume = if std::env::var_os("VIVARIUM_NO_CACHE").is_some() {
            generate(region_half)
        } else {
            let path = cache_path(region_half);
            match std::fs::read(&path).ok().and_then(|b| Volume::from_bytes(&b)) {
                Some(v) => {
                    eprintln!("iso-tiles: worldgen cache HIT — reloaded {}", path.display());
                    v
                }
                None => {
                    let v = generate(region_half);
                    match write_cache(&path, &v) {
                        Ok(()) => eprintln!("iso-tiles: worldgen cached → {}", path.display()),
                        Err(e) => eprintln!("iso-tiles: cache write failed {}: {e}", path.display()),
                    }
                    v
                }
            }
        };
        let detail = volume.detail();
        let sea_level = volume.sea_level();
        Self { volume: Arc::new(volume), detail, sea_level }
    }

    /// Sample one map cell. `cx`/`cz` are cell indices; `stride` is the cell's
    /// edge in voxels (the LOD). The sample is taken at the cell centre.
    pub fn sample(&self, cx: i32, cz: i32, stride: i32) -> SurfaceCell {
        let x = cx * stride + stride / 2;
        let z = cz * stride + stride / 2;
        let h = self.volume.surface_height(x, z).unwrap_or(self.sea_level);
        let depth = self.volume.water_depth_voxels(x, z);
        let speed = self.volume.water_speed(x, z);
        let material = self.volume.voxel(x, h, z).0;
        SurfaceCell {
            height_vox: h,
            material,
            water_depth_vox: depth,
            water_speed: speed,
            wet: depth > 0,
        }
    }
}

fn region_half_m() -> i32 {
    std::env::var("VIVARIUM_REGION_HALF").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_REGION_HALF_M)
}

fn generate(region_half: i32) -> Volume {
    eprintln!("iso-tiles: generating eroded world (±{region_half} m, {EPOCHS} epochs)… slow tier, minutes.");
    World::eroded(SEED, N_AGENTS, DETAIL, region_half, EPOCHS, FINE_CELL_M, FINE_EPOCHS).volume
}

/// Same scheme as the other adapters' cache path, so the file coincides.
fn cache_path(region_half: i32) -> PathBuf {
    let dir = std::env::var_os("VIVARIUM_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("vivarium-worldcache"));
    let name = format!(
        "viv_s{SEED:x}_d{DETAIL}_r{region_half}_e{EPOCHS}_fc{FINE_CELL_M:.1}_fe{FINE_EPOCHS}_v{WORLDGEN_VERSION}.bin",
    );
    dir.join(name)
}

fn write_cache(path: &PathBuf, volume: &Volume) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("bin.tmp");
    std::fs::write(&tmp, volume.to_bytes())?;
    std::fs::rename(&tmp, path)
}
