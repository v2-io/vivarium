//! The **observe surface** — every store-scanning or cohort-preferring reader,
//! on its own type so the compute/view wall is visible in the code's shape.
//!
//! `World`'s compute methods are pure in their complete keys: they read
//! dependencies **by exact key only** and memoize under keys that name
//! everything they read (`#form-depend-by-key-never-latest`). The readers here
//! do something categorically different: they scan roots, prefer among
//! cohorts (current `src=` over stale, halo bed over edge-sink), and answer
//! "what is the best built thing to *show*." That is lawful for views and
//! fatal for computes — a compute that reads "the best thing in the store"
//! makes its bytes a function of build order (violated live 2026-07-29,
//! retreated same day; the erosion_tile/water_tile incident).
//!
//! So the wall is a type: a compute path that wants one of these readers must
//! write `world.observe().…` — greppable, glaring in review, and never
//! necessary. Nothing here computes physics or writes a memo (the one
//! fallback that may compute is the fated prior via `initial_topography`,
//! which is the world's own identity function — byte-equal on every path —
//! and whose memo write is refused by read-only view stores and counted).

use crate::erosion::{self, ErodedRegion};
use crate::query::{decode_f32, key_field, RegionCensus, Source, World};
use crate::sphere::{CellId, Face};

/// Borrowed observe-only facade over a [`World`]. Construct via
/// [`World::observe`].
pub struct Observatory<'w, 's> {
    pub(crate) w: &'w World<'s>,
}

impl<'w, 's> Observatory<'w, 's> {
    /// View-facing surface pull: **prefer a store-hit eroded tile**, else fall
    /// back to initial topography. Never triggers a cold erosion compute —
    /// views must not invent work the builder has not done; they only *show*
    /// what the store already holds (core/view wall: peers that query).
    ///
    /// Returns `(heights, source, eroded)` where `eroded` is true iff the
    /// surface came from a memoized fluvial tile at `epochs`.
    ///
    /// **Note:** this hits one complete tile identity `(oi,oj,nx,epochs)`. The
    /// builder sweeps many 64×64 tiles; for a whole-face or free-roam view that
    /// must see *all* of them, use [`Self::load_eroded_regions_where`] +
    /// [`Self::assemble_surface_tile`]. Prefers a halo-exchanged memo when
    /// present, deterministically.
    pub fn surface_prefer_eroded(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
    ) -> (Vec<f32>, Source, bool) {
        if let Some((h, src)) = self.store_eroded_at(face, level, oi, oj, nx, epochs) {
            return (h, src, true);
        }
        let (tile, src) = self.w.initial_topography(face, level, oi, oj, nx);
        (tile, src, false)
    }

    /// The best eroded memo at this tile — halo preferred over plain edge-sink
    /// when both exist at the same coordinates and epochs
    /// ( #form-same-level-halo-exchange store-identity). Ties inside a rank
    /// break by lexicographically smallest key, so what a view shows never
    /// depends on store iteration order. `None` if nothing is stored.
    fn store_eroded_at(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        epochs: u32,
    ) -> Option<(Vec<f32>, Source)> {
        let Ok(roots) = self.w.store_ref().roots() else {
            return None;
        };
        let face_s = face.index().to_string();
        let level_s = level.to_string();
        let oi_s = oi.to_string();
        let oj_s = oj.to_string();
        let nx_s = nx.to_string();
        let epochs_s = epochs.to_string();
        let seed_s = self.w.seed().to_string();
        // (rank, key, provisional, h) — higher rank wins; inside a rank the
        // lexicographically smallest key wins (deterministic across stores).
        let mut best: Option<(u8, &str, bool, Vec<f32>)> = None;
        for r in &roots {
            if !r.key.starts_with("erosion-tile@") {
                continue;
            }
            if key_field(&r.key, "aspect").is_some() {
                continue;
            }
            if key_field(&r.key, "face") != Some(face_s.as_str()) {
                continue;
            }
            if key_field(&r.key, "level") != Some(level_s.as_str()) {
                continue;
            }
            if key_field(&r.key, "oi") != Some(oi_s.as_str()) {
                continue;
            }
            if key_field(&r.key, "oj") != Some(oj_s.as_str()) {
                continue;
            }
            if key_field(&r.key, "nx") != Some(nx_s.as_str()) {
                continue;
            }
            if key_field(&r.key, "epochs") != Some(epochs_s.as_str()) {
                continue;
            }
            if key_field(&r.key, "seed") != Some(seed_s.as_str()) {
                continue;
            }
            let rank = if key_field(&r.key, "edge") == Some("halo") { 1u8 } else { 0 };
            if best
                .as_ref()
                .is_some_and(|(br, bk, _, _)| *br > rank || (*br == rank && *bk <= r.key.as_str()))
            {
                continue;
            }
            let Some(bytes) = self.w.store_ref().object_bytes(&r.object) else {
                continue;
            };
            let h = decode_f32(&bytes);
            if h.len() != nx * nx {
                continue;
            }
            best = Some((rank, r.key.as_str(), r.provisional, h));
        }
        best.map(|(_, _, provisional, h)| {
            (
                h,
                if provisional {
                    Source::HitProvisional
                } else {
                    Source::Hit
                },
            )
        })
    }

    /// Census of `erosion-tile` roots by **source-hash freshness** — the loud
    /// signal a view needs so silent staleness stops masquerading as geography.
    ///
    /// Every nomos key folds the build-time whole-crate source digest
    /// ([`crate::nomotheke::SRC_HASH`], `#form-complete-content-addressed-key`).
    /// A root whose `src=` field differs from the current binary's hash was
    /// carved under a **different source tree** — its bytes are matured, but not
    /// this world's *current* surface. `load_eroded_regions_where` does NOT
    /// filter on this, so a stale tile is loaded and shown as if current unless
    /// the caller consults this census / uses [`Self::load_current_eroded_regions`].
    pub fn eroded_region_census(&self) -> RegionCensus {
        let Ok(roots) = self.w.store_ref().roots() else {
            return RegionCensus::default();
        };
        let mut c = RegionCensus::default();
        for r in &roots {
            if !r.key.starts_with("erosion-tile@") {
                continue;
            }
            if key_field(&r.key, "aspect").is_some() {
                continue; // stage-residual siblings are metadata, not surfaces
            }
            c.total += 1;
            if key_field(&r.key, "src") == Some(crate::nomotheke::SRC_HASH) {
                c.fresh += 1;
            } else {
                c.stale += 1;
            }
        }
        c
    }

    /// Materialize one **cohort's** `erosion-tile` roots as [`ErodedRegion`]s —
    /// the tiles carved under exactly the source tree `src`. Pure store census;
    /// order is coarse → fine by level (required by [`erosion::surface_at`]).
    ///
    /// This is the cohort-safe convenient path ( `#norm-caught-disciplines-`
    /// `become-mechanisms` FE(2)(a)): a store holds beds carved under many
    /// source trees, and a reader that merges cohorts censuses a terrain nobody
    /// built — a chimera, the fault class three independent readers hit on
    /// 2026-07-28 before the merging default was removed. Choosing the cohort
    /// is now part of the read. `watch::erosion_cohorts` enumerates what a
    /// store holds.
    pub fn load_eroded_regions_cohort(&self, src: &str) -> Vec<ErodedRegion> {
        self.load_eroded_regions_where(|key| key_field(key, "src") == Some(src))
    }

    /// [`Self::load_eroded_regions_cohort`] at the current binary's
    /// [`crate::nomotheke::SRC_HASH`] — the observe-only honest surface: a tile
    /// counts as *this* world's eroded state only if it was carved under the
    /// source now running. Stale tiles are dropped (loudly, via
    /// [`Self::eroded_region_census`]), never silently blended into the surface.
    pub fn load_current_eroded_regions(&self) -> Vec<ErodedRegion> {
        let cur = crate::nomotheke::SRC_HASH;
        self.load_eroded_regions_where(|key| key_field(key, "src") == Some(cur))
    }

    /// Load exactly the `erosion-tile` roots whose complete key `keep` accepts.
    ///
    /// Public because **replay needs it**: replaying a build in any renderer
    /// means assembling the surface from the roots that had landed by frame *n*,
    /// which is a key-set predicate and nothing more
    /// ( #form-time-indexed-stage-chains FE(5) — the live path and the replay
    /// path must be the same mechanism, and they are only the same mechanism if
    /// replay assembles through this function rather than through a private
    /// second loader). Callers that want the ordinary honest surface should use
    /// [`Self::load_current_eroded_regions`].
    ///
    /// **This is the sharp path, and cohort honesty is the caller's burden
    /// here**: a predicate that ignores `src=` merges beds carved under
    /// different source trees into a chimera — a terrain nobody built
    /// ( `#norm-caught-disciplines-become-mechanisms` FE(2)(a)). Replay
    /// predicates should pin a cohort alongside their landing cut, as the
    /// explorer's do.
    ///
    /// **One region per tile, the latest stage among accepted roots.** A staged
    /// build leaves *many* roots per tile — the settle history — and a surface
    /// is one moment, not a blend of moments. Per `(face, level, oi, oj, nx)`
    /// the highest-`epochs` accepted root wins; at equal epochs a **halo**
    /// schedule key beats a plain edge-sink key (exchange adoption), and any
    /// remaining tie breaks by lexicographically smallest key.
    pub fn load_eroded_regions_where(&self, keep: impl Fn(&str) -> bool) -> Vec<ErodedRegion> {
        let Ok(roots) = self.w.store_ref().roots() else {
            return Vec::new();
        };
        // Tile identity → (epochs, halo_rank, key, region); BTree for deterministic order.
        type TileAt = (u8, u8, u32, u32, usize); // (face, level, oi, oj, nx)
        let mut latest: std::collections::BTreeMap<TileAt, (u32, u8, String, ErodedRegion)> =
            std::collections::BTreeMap::new();
        for r in roots {
            if !r.key.starts_with("erosion-tile@") {
                continue;
            }
            if key_field(&r.key, "aspect").is_some() {
                continue;
            }
            if !keep(&r.key) {
                continue;
            }
            let Some(face_i) = key_field(&r.key, "face").and_then(|v| v.parse::<u8>().ok()) else {
                continue;
            };
            let Some(level) = key_field(&r.key, "level").and_then(|v| v.parse::<u8>().ok()) else {
                continue;
            };
            let Some(oi) = key_field(&r.key, "oi").and_then(|v| v.parse::<u32>().ok()) else {
                continue;
            };
            let Some(oj) = key_field(&r.key, "oj").and_then(|v| v.parse::<u32>().ok()) else {
                continue;
            };
            let Some(nx) = key_field(&r.key, "nx").and_then(|v| v.parse::<usize>().ok()) else {
                continue;
            };
            let epochs = key_field(&r.key, "epochs").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
            let rank = if key_field(&r.key, "edge") == Some("halo") { 1u8 } else { 0 };
            let at = (face_i, level, oi, oj, nx);
            if let Some((have_ep, have_rank, have_key, _)) = latest.get(&at) {
                if *have_ep > epochs
                    || (*have_ep == epochs && *have_rank > rank)
                    || (*have_ep == epochs && *have_rank == rank && *have_key <= r.key)
                {
                    continue;
                }
            }
            let Some(bytes) = self.w.store_ref().object_bytes(&r.object) else {
                continue;
            };
            let h = decode_f32(&bytes);
            if h.len() != nx * nx {
                continue;
            }
            let region = ErodedRegion {
                face: Face::from_index(face_i),
                level,
                oi,
                oj,
                nx,
                h,
                seed: self.w.seed(),
            };
            latest.insert(at, (epochs, rank, r.key.clone(), region));
        }
        // BTreeMap order is (face, level, …); assembly requires coarse → fine.
        let mut out: Vec<ErodedRegion> = latest.into_values().map(|(_, _, _, r)| r).collect();
        out.sort_by_key(|r| (r.level, r.face.index(), r.oi, r.oj));
        out
    }

    /// Assemble an `nx×nx` height tile at `(face, level, oi, oj)` from loaded
    /// store regions + fated prior. No erosion compute, no store write.
    /// `any_eroded` is true if any cell was covered by a region.
    pub fn assemble_surface_tile(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        regions: &[ErodedRegion],
    ) -> (Vec<f32>, bool) {
        let mut tile = Vec::with_capacity(nx * nx);
        let mut any_eroded = false;
        for j in 0..nx as u32 {
            for i in 0..nx as u32 {
                let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
                if erosion::tier_at(cell, regions).is_some() {
                    any_eroded = true;
                }
                tile.push(erosion::surface_at(self.w.seed(), cell, regions) as f32);
            }
        }
        (tile, any_eroded)
    }

    /// The store-only half of `water_tile`, as a scan (a view cannot construct
    /// the exact key: the `bed=` token is the builder's knowledge, not the
    /// viewer's). Among matches the preference is deterministic: a halo-bed
    /// depth over an edge-sink one, then the lexicographically smallest key —
    /// never store iteration order. Never runs the fill kernel; the honest view
    /// behaviour on a miss is to show no water and say the tiles are stale,
    /// not to silently re-settle the planet ( `#form-builder-admission` FE(4)).
    #[allow(clippy::too_many_arguments)]
    pub fn water_tile_hit(
        &self,
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        erosion_epochs: u32,
        steps: u32,
    ) -> Option<(Vec<f32>, Source)> {
        let roots = self.w.store_ref().roots().ok()?;
        let want = [
            ("seed", self.w.seed().to_string()),
            ("face", face.index().to_string()),
            ("level", level.to_string()),
            ("oi", oi.to_string()),
            ("oj", oj.to_string()),
            ("nx", nx.to_string()),
            ("eepochs", erosion_epochs.to_string()),
            ("steps", steps.to_string()),
            // The exact-key path matched src implicitly; the scan must too, or
            // a stale cohort's water would render as current.
            ("src", crate::nomotheke::SRC_HASH.to_string()),
        ];
        let mut best: Option<(u8, &str, bool, &str)> = None; // rank, key, provisional, object
        for r in &roots {
            if !r.key.starts_with("water-tile@") {
                continue;
            }
            if want.iter().any(|(n, v)| key_field(&r.key, n) != Some(v.as_str())) {
                continue;
            }
            let rank = if key_field(&r.key, "bed").is_some_and(|b| b.starts_with("halo")) {
                1u8
            } else {
                0
            };
            if best
                .as_ref()
                .is_some_and(|(br, bk, _, _)| *br > rank || (*br == rank && *bk <= r.key.as_str()))
            {
                continue;
            }
            best = Some((rank, r.key.as_str(), r.provisional, r.object.as_str()));
        }
        let (_, _, provisional, object) = best?;
        let bytes = self.w.store_ref().object_bytes(object)?;
        let h = decode_f32(&bytes);
        if h.len() != nx * nx {
            return None;
        }
        Some((h, if provisional { Source::HitProvisional } else { Source::Hit }))
    }
}
