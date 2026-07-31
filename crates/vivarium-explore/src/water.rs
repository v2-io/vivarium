//! The **water field** — the built `water-tile` nomos, loaded for display.
//!
//! Standing on its own because of a fact worth stating plainly: as of
//! 2026-07-24 the `water-tile` nomos is materialized by every `vivarium build`
//! and **has never been rendered by any view**. `vivarium watch` / `info` colour
//! a region green when a water tile *exists*; neither draws the depths. The
//! globe spike does not load it at all. So the only hydrology in the tree has
//! been invisible, while the thing everyone has been looking at — the blue on a
//! from-space globe — is not hydrology at all but the *sea datum*: every cell
//! whose tectonic surface sits below `derived_sea_m`.
//!
//! That gap is the direct cause of a real confusion: inland "lakes" that appear
//! and vanish during deep-time playback are the sea datum sweeping across
//! depressions as the mantle cools, not water going anywhere. There is no lake
//! model, no endorheic model, no drainage integration in the tree at all. The
//! honest fix is not to stop showing the datum — it is to draw the actual water
//! field next to it and let the difference be visible.
//!
//! Read-only **and** never-block: every tile arrives through
//! `World::water_tile_hit`, which is a store lookup and nothing else. The
//! non-`_hit` form runs the fill kernel on a miss, and over a whole-globe census
//! that is minutes of cold evolution on the caller's thread — measured here at
//! ~44 s to first frame the first time this module used it, on the entirely
//! ordinary occasion of a source edit having moved every key's source hash
//! ( #form-builder-admission FE(4): a view prefers a Hit and does not
//! cold-trigger evolution the builder has not done). Where the current source
//! has no settled water, this field is empty and the HUD says the tiles are
//! stale — which is the true answer, and arrives instantly.
//!
//! **Level selection (2026-07-31):** Coverage's ladder is "deepest *surface*
//! tile" (often an L13 beacon). Water was only stored at L9 full-globe — so
//! loading solely via `cov.watered` made **every** settled lake invisible
//! whenever a finer erosion beacon existed. We load the **finest water-tile
//! grain under the current `src=`**, not the coverage surface level.

use std::collections::HashMap;

use vivarium_world::nomotheke::SRC_HASH;
use vivarium_world::query::World;
use vivarium_world::sphere::Face;
use vivarium_world::store::RootEntry;
use vivarium_world::watch::{self, Coverage};

/// Standing-water depth (m) over the built tiles, at the level they were built
/// at — sampled down to whatever level the view is drawing.
pub struct WaterField {
    /// The level these tiles were built at (the water nomos's level, not
    /// necessarily the coverage surface level).
    pub level: u8,
    pub nx: usize,
    /// `(face, oi, oj)` → the tile's depth field.
    tiles: HashMap<(u8, u32, u32), Vec<f32>>,
    /// Deepest standing water found (m) — the display ramp's top.
    pub max_depth_m: f32,
    /// Water roots the census named (at the chosen level, current src).
    pub requested: usize,
    /// Of those, how many were settled under the CURRENT source hash and could
    /// therefore be shown. `loaded < requested` is a staleness finding, not a
    /// loading failure.
    pub loaded: usize,
}

/// A depth threshold below which a cell is not usefully "water". The water
/// kernel leaves a thin film almost everywhere it rains; calling that a lake
/// would make the instrument report hydrology that is not there.
pub const WET_M: f32 = 0.5;

impl WaterField {
    /// Empty — a world with no water tiles, honestly.
    pub fn none() -> WaterField {
        WaterField {
            level: 0,
            nx: 0,
            tiles: HashMap::new(),
            max_depth_m: 0.0,
            requested: 0,
            loaded: 0,
        }
    }

    /// Load water for display from the root listing (preferred).
    ///
    /// Picks the **finest** level that has any `water-tile` under the current
    /// `SRC_HASH`, then loads every tile at that level. Does **not** use
    /// [`Coverage`]'s surface level — that is often a beacon grain with no water.
    pub fn load_from_roots(world: &World, roots: &[RootEntry]) -> WaterField {
        // level → list of (face, oi, oj, nx, eepochs, steps)
        let mut by_level: HashMap<u8, Vec<(u8, u32, u32, usize, u32, u32)>> = HashMap::new();
        for r in roots {
            if !r.key.starts_with("water-tile@") {
                continue;
            }
            if watch::key_field(&r.key, "src") != Some(SRC_HASH) {
                continue;
            }
            let Some(level) = watch::key_field(&r.key, "level").and_then(|v| v.parse::<u8>().ok())
            else {
                continue;
            };
            let Some(face) = watch::key_field(&r.key, "face").and_then(|v| v.parse::<u8>().ok())
            else {
                continue;
            };
            let Some(oi) = watch::key_field(&r.key, "oi").and_then(|v| v.parse::<u32>().ok()) else {
                continue;
            };
            let Some(oj) = watch::key_field(&r.key, "oj").and_then(|v| v.parse::<u32>().ok()) else {
                continue;
            };
            let Some(nx) = watch::key_field(&r.key, "nx").and_then(|v| v.parse::<usize>().ok())
            else {
                continue;
            };
            let eepochs = watch::key_field(&r.key, "eepochs")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(0);
            let steps = watch::key_field(&r.key, "steps")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(0);
            by_level
                .entry(level)
                .or_default()
                .push((face, oi, oj, nx, eepochs, steps));
        }
        let Some((&level, entries)) = by_level.iter().max_by_key(|(l, _)| *l) else {
            return WaterField::none();
        };
        let nx = entries.first().map(|e| e.3).unwrap_or(64);
        let mut tiles = HashMap::with_capacity(entries.len());
        let mut max_depth_m = 0.0f32;
        let mut loaded = 0usize;
        for &(f, oi, oj, tile_nx, eepochs, steps) in entries {
            let face = Face::from_index(f);
            let Some((depth, _src)) = world
                .observe()
                .water_tile_hit(face, level, oi, oj, tile_nx, eepochs, steps)
            else {
                continue;
            };
            if depth.len() != tile_nx * tile_nx {
                continue;
            }
            for &d in &depth {
                if d.is_finite() && d > max_depth_m {
                    max_depth_m = d;
                }
            }
            tiles.insert((f, oi, oj), depth);
            loaded += 1;
        }
        WaterField {
            level,
            nx,
            tiles,
            max_depth_m,
            requested: entries.len(),
            loaded,
        }
    }

    /// Load via coverage census — **only** water at the coverage surface level.
    /// Prefer [`load_from_roots`] when a beacon made `cov.level` finer than the
    /// water build grain.
    #[allow(dead_code)] // retained for callers that only have Coverage
    pub fn load(world: &World, cov: &Coverage) -> WaterField {
        if cov.watered.is_empty() {
            return WaterField::none();
        }
        let mut tiles = HashMap::with_capacity(cov.watered.len());
        let mut max_depth_m = 0.0f32;
        for (&(f, oi, oj), &(eepochs, steps)) in &cov.watered {
            let face = Face::from_index(f);
            let Some((depth, _src)) =
                world.observe().water_tile_hit(face, cov.level, oi, oj, cov.nx, eepochs, steps)
            else {
                continue;
            };
            if depth.len() != cov.nx * cov.nx {
                continue;
            }
            for &d in &depth {
                if d.is_finite() && d > max_depth_m {
                    max_depth_m = d;
                }
            }
            tiles.insert((f, oi, oj), depth);
        }
        let loaded = tiles.len();
        WaterField {
            level: cov.level,
            nx: cov.nx,
            tiles,
            max_depth_m,
            requested: cov.watered.len(),
            loaded,
        }
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Depth (m) at a cell given in *view* coordinates at `view_level`.
    ///
    /// Sampling, not interpolation: a coarser view reads the nearest built cell.
    /// The alternative — averaging depths across a coarse cell — would turn a
    /// deep narrow lake into a shallow wide one and quietly change the thing
    /// being reported.
    pub fn depth_at(&self, face: u8, ci: u32, cj: u32, view_level: u8) -> f32 {
        if self.tiles.is_empty() {
            return 0.0;
        }
        let (ci, cj) = if view_level <= self.level {
            let s = self.level - view_level;
            (ci << s, cj << s)
        } else {
            let s = view_level - self.level;
            (ci >> s, cj >> s)
        };
        let n = self.nx as u32;
        let (oi, oj) = ((ci / n) * n, (cj / n) * n);
        match self.tiles.get(&(face, oi, oj)) {
            Some(t) => {
                let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                t.get(dj * self.nx + di).copied().unwrap_or(0.0)
            }
            None => 0.0,
        }
    }
}
