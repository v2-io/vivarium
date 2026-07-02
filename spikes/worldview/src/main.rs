//! vivarium-worldview — the first view over the clean-room frame.
//!
//! Depends on `vivarium-world` ONLY (the core/view wall). Where `spikes/slabs`
//! renders `vivarium-core`'s flat baked patch, this renders the cube-sphere frame:
//! pick a face region, sample it into field patches (`sample::sample_surface` —
//! data only), and build the proven point-mesh + translucent depth-shaded water
//! (idioms carried from slabs, which stays the core-backed SOTA until this
//! matures).
//!
//! The exploration ENGINE is at parity with slabs (auto-pitch fan probe, look-up
//! near-clip, pan/turn re-framing, HUD, autoshot) so that all remaining work is
//! WORLD fidelity, not view work. Scale is kept honest by construction:
//!   • the pawn is a real 0.5 × 2 m figure, never scaled by zoom or vert — at
//!     survey zoom it is sub-pixel *because that is true*; the focus cursor is a
//!     flat reticle that cannot read as a figure;
//!   • a map scale bar (round-number length, pixel-exact from the ortho zoom);
//!   • the HUD states the sampled window's relief range (m above sea) — the
//!     honest answer to "is there height here?";
//!   • VIVARIUM_VERT exaggeration (default 1 = honest) for survey-scale form,
//!     the standard cartographic practice — relief at a 94 km viewport is ~3% of
//!     the screen at honest scale, exactly like the real Earth from altitude.
//! The fidelity dials are the point:
//!   • `[` / `]` — change the sampling LEVEL live (same geographic spot, finer or
//!     coarser cells). Every rebuild reports its generation time on the HUD, so
//!     when query-graph memoization lands (DESIGN-REDUX §11–12), a revisited
//!     (level, region) will visibly drop to ~0 ms.
//!
//! Floating origin, done right (the audit's far-lands item): global face-cell
//! coordinates in metres reach ~10^7 m, where f32 resolves only ~1 m — so mesh
//! vertices AND the camera work relative to a f64 anchor at the patch centre, and
//! only anchored-relative f32 ever reaches the GPU.

use std::path::PathBuf;

use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::math::DVec2;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use std::sync::{Arc, Mutex};

use vivarium_world::erosion::{self, ErodedRegion, Fluvial, FluvialParams};
use vivarium_world::gen::SEA_LEVEL_M;
use vivarium_world::planet::Planet;
use vivarium_world::sample::{cell_size_m, sample_surface_with, SurfacePatch};
use vivarium_world::water::{WaterParams, WaterRegion, WaterSim};
use vivarium_world::sphere::{CellId, Face};

const SKY: Color = Color::srgb(0.80, 0.82, 0.84);
/// Water colour by depth (Beer–Lambert). Tuned for STREAMS AND POOLS, not open
/// ocean (Joseph: sea-calibrated absorption made mountain water glint-only):
/// ~0.25 m reads ~40% opaque, ~1 m ~85%, with a floor so even films register.
const WATER_SHALLOW: [f32; 3] = [0.46, 0.63, 0.75];
const WATER_DEEP: [f32; 3] = [0.05, 0.16, 0.38];
const WATER_ABSORB_PER_M: f32 = 1.2; // gentler Beer–Lambert: see the bed longer

// --- Framing (ortho, true-iso default — slabs' proven constants) -------------------
const ISO_PITCH: f32 = 0.615_479_7; // atan(1/√2)
const YAW_START: f32 = std::f32::consts::FRAC_PI_4;
const YAW_STEP: f32 = std::f32::consts::FRAC_PI_4;
const ROT_LERP: f32 = 12.0;
const PITCH_MIN: f32 = -0.5; // below level: look UP (foreground near-clipped away)
const PITCH_MAX: f32 = 1.50; // ~86°, near top-down (auto-pitch's ceiling)
const PITCH_RATE: f32 = 0.9;
const PITCH_LERP_UP: f32 = 16.0; // catch up fast when terrain rises into the way
const PITCH_LERP_DOWN: f32 = 4.0; // relax gently once clear
const PAN_RATE: f32 = 0.5; // fraction of the zoom per second
const ZOOM_STEP: f32 = 1.15;
/// Focus sits this fraction of the viewport below centre (slabs' aim-ahead trick).
const FOCUS_BELOW_CENTER: f32 = 0.22;
/// Ortho standoff: must exceed any terrain span we render (affects clip, not size).
const STANDOFF: f32 = 400_000.0;

/// The erosion TELESCOPE — nested tiers, coarse → fine, each seeded from the ones
/// below (erosion::surface_at samples finest-first; baseline outside all tiers).
/// Tier 0 (L19, ~10 km) builds synchronously; finer tiers (L21 over the on-screen
/// window, L24 around the pawn) arrive from a background thread and refine the
/// live view as they land — the multi-LOD + memoization interaction test.
#[derive(Resource)]
struct Eroded(Arc<Vec<ErodedRegion>>);

/// Channel delivering background-built tiers to the ECS.
struct TierMsg {
    region: ErodedRegion,
    epochs_total: u32,
    /// Nominal sim-years advanced by this update (EPOCH_YEARS × epochs run) —
    /// the aging-speed instrument's numerator.
    sim_years: f32,
    /// Mean |Δh| of the last epoch (m) — the convergence instrument.
    delta_m: f32,
}

/// Water tier updates (the FAST band).
struct WaterMsg {
    region: WaterRegion,
    sim_seconds: f32,
    /// Mean |Δdepth| over the burst (m) — steady-state indicator.
    delta_m: f32,
    /// True while phase 3a (FILLING: deluge to steady state) is still running.
    filling: bool,
    /// (max Froude, fraction of wet cells supercritical Fr>1.5) — the roll-wave
    /// gauge: surges are honest physics only while this shows supercritical flow.
    froude: (f32, f32),
    /// Weather right now: (raining, seconds until it changes — ∞ while filling).
    weather: (bool, f32),
    /// While filling: (the plateau target the per-burst delta must fall to
    /// = 0.15 × filling-phase peak, seconds left before the 6000 sim-s cap).
    /// Honest progress instead of a bare "SETTLING" (Joseph).
    settle: Option<(f32, f32)>,
    /// Live conservation gauge: total-water drift since the sim began (m·cells).
    drift: f64,
}

/// Nominal years per erosion epoch — a stated calibration constant (the epoch is
/// unitless in the solver; this is the display anchor for aging speed).
const EPOCH_YEARS: f32 = 100.0;

/// Physics/recipe version for the fill cache — the crude rung of §12's
/// recipe-hash: BUMP THIS whenever erosion or water physics changes, or stale
/// caches will serve worlds the current algorithms would not produce.
const FILL_ALGO_VERSION: &str = "2026-07-03b"; // b: Jarrett slope-dependent roughness

/// The FILL CACHE (first rung of DESIGN-REDUX §12–13): the filled world —
/// eroded tiers + steady-state water — is a pure function of its parameters,
/// so it is computed once per parameter-set and reloaded on every relaunch
/// (the fill costs tens of minutes; reloading costs seconds, and the LIVING
/// phase — the fun one — becomes the default experience). VIVARIUM_NOCACHE=1
/// bypasses. Format: local-machine binary, native endianness, versioned magic.
mod fill_cache {
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use vivarium_world::erosion::ErodedRegion;
    use vivarium_world::sphere::Face;
    use vivarium_world::water::WaterSim;

    const MAGIC: &[u8; 8] = b"VIVWF001";

    pub fn path(key: &str) -> PathBuf {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        key.hash(&mut h);
        let dir = dirs_cache().join("vivarium").join("worldview");
        let _ = std::fs::create_dir_all(&dir);
        dir.join(format!("fill-{:016x}.bin", h.finish()))
    }

    fn dirs_cache() -> PathBuf {
        std::env::var("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join(".cache"))
    }

    fn put_f32s(out: &mut Vec<u8>, v: &[f32]) {
        out.extend_from_slice(&(v.len() as u64).to_le_bytes());
        for x in v {
            out.extend_from_slice(&x.to_le_bytes());
        }
    }

    fn get_f32s(inp: &mut &[u8]) -> Option<Vec<f32>> {
        let n = u64::from_le_bytes(take(inp, 8)?.try_into().ok()?) as usize;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(f32::from_le_bytes(take(inp, 4)?.try_into().ok()?));
        }
        Some(v)
    }

    fn take<'a>(inp: &mut &'a [u8], n: usize) -> Option<&'a [u8]> {
        if inp.len() < n {
            return None;
        }
        let (a, b) = inp.split_at(n);
        *inp = b;
        Some(a)
    }

    pub fn save(key: &str, tiers: &[ErodedRegion], w: &WaterSim) {
        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&(tiers.len() as u32).to_le_bytes());
        for t in tiers {
            out.push(t.face as u8);
            out.push(t.level);
            out.extend_from_slice(&t.oi.to_le_bytes());
            out.extend_from_slice(&t.oj.to_le_bytes());
            put_f32s(&mut out, &t.h);
        }
        out.push(w.level);
        out.extend_from_slice(&w.origin.0.to_le_bytes());
        out.extend_from_slice(&w.origin.1.to_le_bytes());
        out.extend_from_slice(&w.cell_m.to_le_bytes());
        out.extend_from_slice(&w.atmosphere.to_le_bytes());
        out.extend_from_slice(&w.ocean.to_le_bytes());
        for f in [&w.bed, &w.depth, &w.sediment, &w.groundwater, &w.sed_bed, &w.colmation, &w.armor] {
            put_f32s(&mut out, f);
        }
        let p = path(key);
        let tmp = p.with_extension("tmp");
        if std::fs::File::create(&tmp).and_then(|mut f| f.write_all(&out)).is_ok() {
            let _ = std::fs::rename(&tmp, &p);
            eprintln!("[worldview] fill state cached → {} ({:.1} MB)", p.display(), out.len() as f64 / 1e6);
        }
    }

    pub fn load(key: &str, face: Face) -> Option<(Vec<ErodedRegion>, WaterSim)> {
        let mut bytes = Vec::new();
        std::fs::File::open(path(key)).ok()?.read_to_end(&mut bytes).ok()?;
        let mut inp = bytes.as_slice();
        if take(&mut inp, 8)? != MAGIC {
            return None;
        }
        let ntiers = u32::from_le_bytes(take(&mut inp, 4)?.try_into().ok()?);
        let mut tiers = Vec::new();
        for _ in 0..ntiers {
            let _face = take(&mut inp, 1)?[0];
            let level = take(&mut inp, 1)?[0];
            let oi = u32::from_le_bytes(take(&mut inp, 4)?.try_into().ok()?);
            let oj = u32::from_le_bytes(take(&mut inp, 4)?.try_into().ok()?);
            let h = get_f32s(&mut inp)?;
            let nx = (h.len() as f64).sqrt() as usize;
            tiers.push(ErodedRegion { face, level, oi, oj, nx, h });
        }
        let level = take(&mut inp, 1)?[0];
        let oi = u32::from_le_bytes(take(&mut inp, 4)?.try_into().ok()?);
        let oj = u32::from_le_bytes(take(&mut inp, 4)?.try_into().ok()?);
        let cell = f32::from_le_bytes(take(&mut inp, 4)?.try_into().ok()?);
        let atmosphere = f64::from_le_bytes(take(&mut inp, 8)?.try_into().ok()?);
        let ocean = f64::from_le_bytes(take(&mut inp, 8)?.try_into().ok()?);
        let bed = get_f32s(&mut inp)?;
        let nx = (bed.len() as f64).sqrt() as usize;
        let mut w = WaterSim::new(face, level, (oi, oj), nx, cell, bed, 0.0);
        w.depth = get_f32s(&mut inp)?;
        w.sediment = get_f32s(&mut inp)?;
        w.groundwater = get_f32s(&mut inp)?;
        w.sed_bed = get_f32s(&mut inp)?;
        w.colmation = get_f32s(&mut inp)?;
        w.armor = get_f32s(&mut inp)?;
        w.atmosphere = atmosphere;
        w.ocean = ocean;
        // Momentum (pipe fluxes) is not persisted: it re-spins in a few sim-s.
        w.rebaseline_budget();
        Some((tiers, w))
    }
}
#[derive(Resource)]
struct TierRx(Mutex<std::sync::mpsc::Receiver<TierMsg>>);
#[derive(Resource)]
struct WaterRx(Mutex<std::sync::mpsc::Receiver<WaterMsg>>);
#[derive(Resource, Default)]
struct WaterRes(Option<Arc<WaterRegion>>);
/// The water worker's latest vitals, for the HUD.
struct WaterStat {
    at: std::time::Instant,
    rate: f32,
    delta_m: f32,
    total_ss: f32,
    filling: bool,
    froude: (f32, f32),
    weather: (bool, f32),
    settle: Option<(f32, f32)>,
    drift: f64,
}

#[derive(Resource, Default)]
struct WaterMeta(Option<WaterStat>);

/// Wall-clock + maturity metadata per tier level (a VIEW concern — the world
/// crate stays wall-clock-free): (level, last update, total epochs, aging speed
/// sim-years/wall-second, mean |Δh| m of the last epoch).
#[derive(Resource, Default)]
struct TierMeta(Vec<(u8, std::time::Instant, u32, f32, f32)>);

/// The pawn's whereabouts, shared with the erosion worker (face cells at the
/// view's level, plus the level) — how the telescope re-anchors as you move.
#[derive(Resource, Clone)]
struct SharedFocus(Arc<Mutex<(f64, f64, u8)>>);

fn build_tier0(view: &View) -> Vec<ErodedRegion> {
    if std::env::var("VIVARIUM_ERODE").map(|v| v == "0").unwrap_or(false) {
        return Vec::new();
    }
    const SIM_LEVEL: u8 = 19; // ~19 m cells (core ran 16 m)
    // 256 ≈ 4.9 km world: settles to the full river network in minutes (the
    // whole pipeline scales ~cells²·substeps — 512 ≈ 9.8 km takes tens of
    // minutes to settle; grow it deliberately via env).
    let nx: usize = std::env::var("VIVARIUM_ERODE_NX").ok().and_then(|s| s.parse().ok()).unwrap_or(256);
    let scale = 2f64.powi(SIM_LEVEL as i32 - view.level as i32);
    let (ci, cj) = ((view.focus.x * scale) as u32, (view.focus.y * scale) as u32);
    let t = std::time::Instant::now();
    eprintln!("[worldview] eroding L{SIM_LEVEL} {nx}x{nx} around ({ci},{cj})…");
    let region = ErodedRegion::build(view.face, SIM_LEVEL, ci, cj, nx, &FluvialParams::default());
    eprintln!("[worldview] tier L{SIM_LEVEL} done in {:.1} s", t.elapsed().as_secs_f32());
    vec![region]
}

/// The LIVE erosion worker: a continuous loop that keeps the fine tiers anchored
/// to the pawn and keeps running epochs on a ~0.5 s cadence, so erosion is
/// watchable as it happens. Per cycle, per tier (L21 ≈ 4.8 m cells over the
/// on-screen ~2.4 km; L24 ≈ 0.6 m cells ~150 m around the pawn):
///   • pawn moved past ~¼ of the tier's span → RE-SEED at the new centre from the
///     coarser tiers (fresh "recent epochs", Joseph's core practice);
///   • otherwise → run a few MORE epochs on the same field (incremental maturing —
///     the watchable part).
/// Each update is snapshot to the ECS; VIVARIUM_LIVE=0 falls back to one-shot.
/// Mode dispatch (Joseph, stabilizing until seams/saving land): default =
/// SETTLE — a few more macro epochs, one wide fine pass, then RAIN ONLY (no
/// re-anchoring, no re-seeding: the water's work persists). The live telescope
/// stays available via VIVARIUM_MODE=telescope.
fn spawn_fine_tiers(view: &View, base: Vec<ErodedRegion>, tx: std::sync::mpsc::Sender<TierMsg>, wtx: std::sync::mpsc::SyncSender<WaterMsg>, focus: SharedFocus) {
    if std::env::var("VIVARIUM_MODE").map(|v| v == "telescope").unwrap_or(false) {
        spawn_telescope(view, base, tx, wtx, focus);
    } else {
        spawn_settle(view, base, tx, wtx);
    }
}

/// SETTLE: sequential phases, then water-only forever.
///  1. +VIVARIUM_MACRO_EXTRA (40) L19 epochs with differential uplift, in
///     visible chunks;
///  2. one L21 fine pass over VIVARIUM_FINE_NX (1024 ≈ 4.9 km ≈ 3 mi) cells,
///     VIVARIUM_FINE_EPOCHS (6) epochs, mean-pinned to L19;
///  3. rain + sediment on that bed, indefinitely — the ONLY thing running.
fn spawn_settle(view: &View, base: Vec<ErodedRegion>, tx: std::sync::mpsc::Sender<TierMsg>, wtx: std::sync::mpsc::SyncSender<WaterMsg>) {
    if base.is_empty() {
        return;
    }
    let face = view.face;
    let rain_mult: f32 = std::env::var("VIVARIUM_RAIN").ok().and_then(|s| s.parse().ok()).unwrap_or(10.0);
    let atmos_m: f64 = std::env::var("VIVARIUM_ATMOS").ok().and_then(|s| s.parse().ok()).unwrap_or(2.0);
    // Periodic STORMS (Joseph): episodic rain is the channel-forming regime —
    // between storms, convergence dominates and threads carve. "on,off" sim-secs.
    let (storm_on, storm_off): (f32, f32) = std::env::var("VIVARIUM_STORM")
        .ok()
        .and_then(|v| {
            let mut it = v.split(',').filter_map(|t| t.trim().parse::<f32>().ok());
            Some((it.next()?, it.next()?))
        })
        .unwrap_or((400.0, 400.0));
    let macro_extra: u32 = std::env::var("VIVARIUM_MACRO_EXTRA").ok().and_then(|s| s.parse().ok()).unwrap_or(40);
    // Fine + water cover the WHOLE macro region (Joseph: one consistent border,
    // not nested ones you trip over while wandering). L21 = L19 × 4 exactly, so
    // the fine grid is the macro footprint at 4.77 m cells (2048² ≈ 9.8 km for
    // the default macro). VIVARIUM_FINE_NX overrides (focus-centred) if set.
    let fine_nx_env: Option<usize> = std::env::var("VIVARIUM_FINE_NX").ok().and_then(|s| s.parse().ok());
    let fine_epochs: u32 = std::env::var("VIVARIUM_FINE_EPOCHS").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
    // Joseph: "two or three fine passes, not just one" — total fine work is
    // passes × epochs, pinned to the macro after every chunk.
    let fine_passes: u32 = std::env::var("VIVARIUM_FINE_PASSES").ok().and_then(|s| s.parse().ok()).unwrap_or(3);
    let fine_total = fine_epochs * fine_passes;
    let f21 = view.focus * 2f64.powi(21 - view.level as i32);
    const CADENCE: std::time::Duration = std::time::Duration::from_millis(500);

    std::thread::spawn(move || {
        let mut tiers = base;

        // The fill is a pure function of these parameters (§12): key it,
        // reload it, never pay for the same fill twice. VIVARIUM_NOCACHE=1
        // forces a recompute (e.g. to watch the fill happen).
        let wl: u8 = std::env::var("VIVARIUM_WATER_LEVEL").ok().and_then(|s| s.parse().ok()).unwrap_or(21).clamp(21, 22);
        // VIVARIUM_FILL_CAP: hard ceiling on fill sim-seconds (default 6000).
        // Part of the cache key — an early-capped fill is a DIFFERENT world
        // state than a converged one (§12: under-keying is silent corruption).
        let fill_cap: f32 = std::env::var("VIVARIUM_FILL_CAP").ok().and_then(|s| s.parse().ok()).unwrap_or(6000.0);
        let key = {
            let m = tiers.iter().find(|r| r.level == 19);
            format!(
                "{FILL_ALGO_VERSION}|{face:?}|macro {},{},{}x{}e|fine {fine_nx_env:?} {fine_total}e|wl{wl}|atm{atmos_m}|deluge{}|cap{fill_cap}",
                m.map(|t| t.oi).unwrap_or(0),
                m.map(|t| t.oj).unwrap_or(0),
                m.map(|t| t.nx).unwrap_or(0),
                FluvialParams::default().epochs + macro_extra,
                rain_mult.max(60.0),
            )
        };
        let nocache = std::env::var("VIVARIUM_NOCACHE").map(|v| v == "1").unwrap_or(false);
        let cached = if nocache { None } else { fill_cache::load(&key, face) };
        let cache_hit = cached.is_some();
        if let Some((ctiers, _)) = &cached {
            eprintln!("[worldview] fill cache HIT — skipping erosion + fill, living phase immediately");
            for r in ctiers {
                let _ = tx.send(TierMsg { region: r.clone(), epochs_total: FluvialParams::default().epochs + macro_extra, sim_years: 0.0, delta_m: 0.0 });
            }
            tiers = ctiers.clone();
        }

        // Phase 1 — more macro history, watchable in chunks.
        if let Some(t0) = tiers.iter().find(|r| r.level == 19).filter(|_| !cache_hit) {
            let base_epochs = FluvialParams::default().epochs;
            let mut f19 = Fluvial::from_region(t0);
            let mut done = 0u32;
            while done < macro_extra {
                let chunk = 5.min(macro_extra - done);
                f19.erode(&FluvialParams { epochs: chunk, uplift_m: 0.05, ..Default::default() });
                done += chunk;
                let region = f19.to_region();
                tiers.retain(|r| r.level != 19);
                tiers.push(region.clone());
                tiers.sort_by_key(|r| r.level);
                if tx.send(TierMsg { region, epochs_total: base_epochs + done, sim_years: chunk as f32 * EPOCH_YEARS, delta_m: f19.last_delta_m }).is_err() {
                    return;
                }
            }
        }

        // Phase 2 (fine passes over the macro footprint, pinned) and phase-3
        // construction both live in the cache-MISS branch: a hit already
        // carries the fine tiers and the steady-state water. (The first cut
        // ran phase 2's `(None,None) => return` on hits — the worker died and
        // the world had no water at all. Caught by the two-leg cache test.)
        let mut w = if let Some((_, cw)) = cached {
            cw
        } else {
            let macro_tier = tiers.iter().find(|r| r.level == 19).cloned();
            let (oi, oj, fine_nx) = match (&macro_tier, fine_nx_env) {
                (Some(t0), None) => (t0.oi * 4, t0.oj * 4, t0.nx * 4),
                (_, Some(nx)) => {
                    let half = (nx / 2) as i64;
                    (((f21.x as i64) - half).max(0) as u32, ((f21.y as i64) - half).max(0) as u32, nx)
                }
                (None, None) => return,
            };
            let coarser = tiers.clone();
            let mut fine = Fluvial::from_surface(face, 21, oi, oj, fine_nx, |c| erosion::surface_at(c, &coarser));
            let parent = tiers.iter().find(|r| r.level == 19).cloned();
            let mut done = 0u32;
            while done < fine_total {
                let chunk = 2.min(fine_total - done);
                fine.erode(&FluvialParams { epochs: chunk, ..Default::default() });
                if let Some(par) = &parent {
                    fine.pin_block_means(19, |c| par.surface_bilinear_m(c).unwrap_or_else(|| vivarium_world::gen::surface_prior_m(c, 19)));
                }
                done += chunk;
                let region = fine.to_region();
                tiers.retain(|r| r.level != 21);
                tiers.push(region.clone());
                tiers.sort_by_key(|r| r.level);
                if tx.send(TierMsg { region, epochs_total: done, sim_years: chunk as f32 * EPOCH_YEARS, delta_m: fine.last_delta_m }).is_err() {
                    return;
                }
            }

            // Phase 3 bed: sampled through the full telescope at the water's
            // level, so finer water flows over correspondingly finer bed detail.
            let wscale = 1u32 << (wl - 21);
            let (woi, woj, wnx) = (oi * wscale, oj * wscale, fine_nx * wscale as usize);
            let cell = vivarium_world::sample::cell_size_m(wl, vivarium_world::planet::Planet::EARTH.radius_m) as f32;
            let mut wbed = vec![0.0f32; wnx * wnx];
            for y in 0..wnx {
                for x in 0..wnx {
                    let c = CellId::from_face_ij(face, woi + x as u32, woj + y as u32, wl);
                    wbed[y * wnx + x] = erosion::surface_at(c, &tiers) as f32;
                }
            }
            WaterSim::new(face, wl, (woi, woj), wnx, cell, wbed, atmos_m)
        };
        let mut sim_total: f32 = 0.0;
        let (mut prev_delta, mut plateau_bursts, mut delta_peak) = (0.0f32, 0u32, 0.0f32);
        // Phase 3a — FILLING: continuous deluge until the water distribution
        // reaches steady state (basins full, lakes at level, river in/outflow
        // balanced). SEDIMENT STAYS ON — core's kill-switch existed for
        // instabilities the momentum fixes removed (probe: channel_profile
        // sediment regimes — zero bed kinks, work bounded by sed_max_rate).
        // The fill therefore also MATURES the beds: alluvium in pools,
        // colmation sealing channels, armor on scoured reaches. Hands off to
        // the living phase (episodic storms) once the water levels out.
        let mut filling = !cache_hit;
        // VIVARIUM_WATER_SHOW: all | burst | (default: burst while filling —
        // fast-forward through the fill — then stream EVERY step once living,
        // so the water visibly flows; sim-time slows to the view's pace).
        let show = std::env::var("VIVARIUM_WATER_SHOW").unwrap_or_default();
        let mut last_writeback = std::time::Instant::now();
        loop {
            let t0 = std::time::Instant::now();
            let stream = match show.as_str() {
                "all" => true,
                "burst" => false,
                _ => !filling,
            };
            // CFL dt from the CURRENT deepest water (the ocean is real water now);
            // burst mode aims ~8 sim-s per burst, capped so deep basins can't
            // stall the loop; streaming shows every single step.
            let dt = w.stable_dt(9.8);
            let substeps: u32 = if stream { 1 } else { ((8.0 / dt) as u32).clamp(1, 400) };
            // Filling: DELUGE (the rain fudge exists to reach steady state
            // fast; the sediment work it does is compressed on the same fudged
            // clock). Living: the user's rain, in storms.
            let phase = sim_total % (storm_on + storm_off);
            let raining = filling || storm_off <= 0.0 || phase < storm_on;
            let mult = if filling { rain_mult.max(60.0) } else { rain_mult };
            let wp = WaterParams {
                precip: if raining { WaterParams::default().precip * mult } else { 0.0 },
                dt,
                ..Default::default()
            };
            sim_total += substeps as f32 * dt;
            let before = w.depth.clone();
            for _ in 0..substeps {
                w.step(&wp);
            }
            let delta: f64 = w.depth.iter().zip(before.iter()).map(|(a, b)| (a - b).abs() as f64).sum();
            let delta_mean = (delta / before.len() as f64) as f32;
            // PLATEAU detection (Joseph), corrected for the signal's shape:
            // during constant-rain FILLING the differential is itself constant
            // (≈ the input), so "not decreasing" alone fires mid-deluge. Settled
            // = the differential has FALLEN well below its filling-phase peak
            // (outflux ≈ influx) and then stopped improving. Cap at 6000 sim-s.
            if filling {
                delta_peak = delta_peak.max(delta_mean);
                if prev_delta > 0.0 && delta_mean > prev_delta * 0.97 {
                    plateau_bursts += 1;
                } else {
                    plateau_bursts = 0;
                }
                prev_delta = delta_mean;
                let fallen = delta_mean < 0.15 * delta_peak;
                if sim_total > 400.0_f32.min(fill_cap * 0.5) && ((fallen && plateau_bursts >= 3) || sim_total > fill_cap) {
                    filling = false;
                    eprintln!("[worldview] water FILLED to steady state at {sim_total:.0} sim-s (d {:.1} mm vs peak {:.1} mm) — living phase (episodic storms)", delta_mean * 1000.0, delta_peak * 1000.0);
                    if !nocache {
                        fill_cache::save(&key, &tiers, &w);
                    }
                }
            }
            let weather = if filling || storm_off <= 0.0 {
                (true, f32::INFINITY)
            } else {
                let ph = sim_total % (storm_on + storm_off);
                if ph < storm_on { (true, storm_on - ph) } else { (false, storm_on + storm_off - ph) }
            };
            let settle_info = if filling { Some((0.15 * delta_peak, (fill_cap - sim_total).max(0.0))) } else { None };
            if wtx.send(WaterMsg { region: w.to_region(), sim_seconds: substeps as f32 * dt, delta_m: delta_mean, filling, froude: w.froude(), weather, settle: settle_info, drift: w.budget_drift() }).is_err() {
                return;
            }
            // Write the carved bed back to the L21 tier (block-mean downsample
            // when the water runs finer — conservative, means preserved). Bed
            // only moves in the living phase (sediment), and the write-back
            // clones a full tier — throttle it by wall time, not by step.
            if last_writeback.elapsed().as_secs_f32() > 5.0 {
                last_writeback = std::time::Instant::now();
                if let Some(entry) = tiers.iter_mut().find(|r| r.level == 21) {
                // Derive geometry from the sim itself (valid for cache-restored
                // sims too, where the build-time locals never existed).
                let f = 1usize << (w.level - 21);
                let (enx, wnx) = (entry.nx, w.nx);
                if f == 1 && enx == wnx {
                    entry.h.copy_from_slice(&w.bed);
                } else {
                    for y in 0..enx {
                        for x in 0..enx {
                            let mut sum = 0.0f64;
                            for by in 0..f {
                                for bx in 0..f {
                                    sum += w.bed[(y * f + by) * wnx + x * f + bx] as f64;
                                }
                            }
                            entry.h[y * enx + x] = (sum / (f * f) as f64) as f32;
                        }
                    }
                }
                if tx.send(TierMsg { region: entry.clone(), epochs_total: fine_total, sim_years: 0.0, delta_m: 0.0 }).is_err() {
                    return;
                }
                }
            }
            if !stream {
                std::thread::sleep(CADENCE.saturating_sub(t0.elapsed()));
            }
        }
    });
}

fn spawn_telescope(
    view: &View,
    base: Vec<ErodedRegion>,
    tx: std::sync::mpsc::Sender<TierMsg>,
    wtx: std::sync::mpsc::SyncSender<WaterMsg>,
    focus: SharedFocus,
) {
    if base.is_empty() {
        return;
    }
    let live = !std::env::var("VIVARIUM_LIVE").map(|v| v == "0").unwrap_or(false);
    // VIVARIUM_RAIN: precipitation multiplier over the kernel default (which is
    // itself ~1000x real — the documented basin-filling fudge). Default 10x so a
    // first-look session sees streams gather in minutes, not tens of minutes.
    let rain_mult: f32 = std::env::var("VIVARIUM_RAIN").ok().and_then(|s| s.parse().ok()).unwrap_or(10.0);
    // VIVARIUM_ATMOS: total rainable water (m per cell). The first ship charged a
    // thimble (0.05 m — the sky ran dry in seconds and RAIN just emptied it
    // faster; found by Joseph standing in a dry valley). 2 m/cell concentrates
    // to metres-deep water in the valley network.
    let atmos_m: f64 = std::env::var("VIVARIUM_ATMOS").ok().and_then(|s| s.parse().ok()).unwrap_or(2.0);
    let face = view.face;
    const CADENCE: std::time::Duration = std::time::Duration::from_millis(500);

    struct TierState {
        level: u8,
        nx: usize,
        init_epochs: u32,
        inc_epochs: u32,
        /// Fine tiers are FINISHERS, not accumulators: Joseph observed (live,
        /// 2026-07-02) that 1–2 animated fine passes are near-ideal — smoothing
        /// + micro-channels — while long accumulation overcooks into deluge
        /// ripples. Incremental epochs stop at this cap (re-seed on move resets).
        max_epochs: u32,
        /// Uplift per epoch (m), fBm-differential — the slow band's tectonic
        /// forcing lives at the MACRO tier (§4); fine tiers inherit it on re-seed.
        uplift_m: f32,
        sim: Option<(Fluvial, u32, u32, u32)>, // (field, oi, oj, epochs_total)
    }
    let mut states = [
        TierState { level: 19, nx: 512, init_epochs: 0, inc_epochs: 1, max_epochs: u32::MAX, uplift_m: 0.05, sim: None },
        TierState { level: 21, nx: 512, init_epochs: 4, inc_epochs: 2, max_epochs: 10, uplift_m: 0.0, sim: None },
        TierState { level: 24, nx: 256, init_epochs: 2, inc_epochs: 2, max_epochs: 6, uplift_m: 0.0, sim: None },
    ];

    std::thread::spawn(move || {
        // tiers, coarse→fine, as the seeding context for (re)builds.
        let mut tiers = base;
        let mut water: Option<WaterSim> = None;
        // Resume the macro tier's simulation from the startup field.
        if let Some(t0) = tiers.iter().find(|r| r.level == 19) {
            states[0].sim = Some((Fluvial::from_region(t0), t0.oi, t0.oj, FluvialParams::default().epochs));
        }
        loop {
            let t0 = std::time::Instant::now();
            let (fx, fy, flevel) = *focus.0.lock().unwrap();
            let mut l21_reseeded = false;
            for st in states.iter_mut() {
                let scale = 2f64.powi(st.level as i32 - flevel as i32);
                let (ci, cj) = ((fx * scale) as i64, (fy * scale) as i64);
                let half = (st.nx / 2) as i64;
                let (noi, noj) = ((ci - half).max(0) as u32, (cj - half).max(0) as u32);
                let needs_seed = match &st.sim {
                    None => true,
                    Some((_, oi, oj, _)) => {
                        // Re-anchor when the pawn has drifted past ~¼ span.
                        (noi as i64 - *oi as i64).abs().max((noj as i64 - *oj as i64).abs()) > (st.nx as i64) / 4
                    }
                };
                // Finished maturing and not moving? Nothing to do for this tier.
                if !needs_seed && st.sim.as_ref().unwrap().3 >= st.max_epochs {
                    continue;
                }
                let epochs_run;
                let epochs_this;
                if needs_seed {
                    if st.level == 21 {
                        l21_reseeded = true;
                    }
                    // Seed from the tiers COARSER than this one only.
                    let coarser: Vec<ErodedRegion> = tiers.iter().filter(|r| r.level < st.level).cloned().collect();
                    let mut f = Fluvial::from_surface(face, st.level, noi, noj, st.nx, |c| erosion::surface_at(c, &coarser));
                    f.erode(&FluvialParams { epochs: st.init_epochs.max(1), uplift_m: st.uplift_m, ..Default::default() });
                    st.sim = Some((f, noi, noj, st.init_epochs));
                    epochs_run = st.init_epochs;
                    epochs_this = st.init_epochs;
                } else {
                    let (f, _, _, total) = st.sim.as_mut().unwrap();
                    f.erode(&FluvialParams { epochs: st.inc_epochs, uplift_m: st.uplift_m, ..Default::default() });
                    *total += st.inc_epochs;
                    epochs_run = *total;
                    epochs_this = st.inc_epochs;
                }
                // Joseph's conservation constraint: fine tiers REDISTRIBUTE relief
                // within the coarse surface (pin block means to the parent tier's
                // current low band) — absolute elevation belongs to the macro tier.
                // Kills interior drift, rides the macro's uplift, and shrinks the
                // tile-edge seam to the high band.
                if st.level > 19 {
                    let parent_level = if st.level == 21 { 19u8 } else { 21u8 };
                    if let Some(parent) = tiers.iter().find(|r| r.level == parent_level).cloned() {
                        let sim = &mut st.sim.as_mut().unwrap().0;
                        sim.pin_block_means(parent_level, |c| {
                            parent.surface_bilinear_m(c).unwrap_or_else(|| vivarium_world::gen::surface_prior_m(c, parent_level))
                        });
                    }
                }
                let region = st.sim.as_ref().unwrap().0.to_region();
                // Keep the local telescope current (replace-by-level, keep order).
                tiers.retain(|r| r.level != st.level);
                tiers.push(region.clone());
                tiers.sort_by_key(|r| r.level);
                let delta_m = st.sim.as_ref().unwrap().0.last_delta_m;
                if tx.send(TierMsg { region, epochs_total: epochs_run, sim_years: epochs_this as f32 * EPOCH_YEARS, delta_m }).is_err() {
                    return; // view closed
                }
            }

            // --- The FAST band: shallow water over the live L21 bed (§4). The
            // water sees terrain quasi-static (bed refreshed each cycle from the
            // still-eroding tier); erosion keeps running — the coupling schedule
            // replaces core's kill-switch. Sediment coupling is the NEXT rung.
            if let Some(l21) = tiers.iter().find(|r| r.level == 21) {
                let rebuild = water.as_ref().map(|w: &WaterSim| w.origin != (l21.oi, l21.oj)).unwrap_or(true) || l21_reseeded;
                if rebuild {
                    let cell = vivarium_world::sample::cell_size_m(21, vivarium_world::planet::Planet::EARTH.radius_m) as f32;
                    water = Some(WaterSim::new(face, 21, (l21.oi, l21.oj), l21.nx, cell, l21.h.clone(), atmos_m));
                } else if let Some(w) = water.as_mut() {
                    w.set_bed(l21.h.clone());
                }
                if let Some(w) = water.as_mut() {
                    // CFL-adaptive, same as settle mode (deep water = fast waves).
                    let wdt = w.stable_dt(9.8);
                    let substeps: u32 = ((8.0 / wdt) as u32).clamp(1, 400);
                    let wp = WaterParams { precip: WaterParams::default().precip * rain_mult, dt: wdt, ..Default::default() };
                    let before = w.depth.clone();
                    for _ in 0..substeps {
                        w.step(&wp);
                    }
                    let delta: f64 = w.depth.iter().zip(before.iter()).map(|(a, b)| (a - b).abs() as f64).sum();
                    let msg = WaterMsg {
                        region: w.to_region(),
                        sim_seconds: substeps as f32 * wdt,
                        delta_m: (delta / before.len() as f64) as f32,
                        filling: false,
                        froude: w.froude(),
                        weather: (true, f32::INFINITY),
                        settle: None,
                        drift: w.budget_drift(),
                    };
                    if wtx.send(msg).is_err() {
                        return;
                    }
                    // TWO-WAY: the water's sediment carving writes back into the
                    // L21 tier (view + finer-tier seeding see the carved bed).
                    if let Some(entry) = tiers.iter_mut().find(|r| r.level == 21) {
                        entry.h.copy_from_slice(&w.bed);
                        let mut epochs_tot = 0;
                        if let Some(st) = states.iter_mut().find(|s| s.level == 21) {
                            if let Some((f, _, _, total)) = st.sim.as_mut() {
                                f.h.copy_from_slice(&w.bed);
                                epochs_tot = *total;
                            }
                        }
                        let carved = TierMsg { region: entry.clone(), epochs_total: epochs_tot, sim_years: 0.0, delta_m: 0.0 };
                        if tx.send(carved).is_err() {
                            return;
                        }
                    }
                }
            }
            if !live {
                return; // one-shot mode: first pass only
            }
            std::thread::sleep(CADENCE.saturating_sub(t0.elapsed()));
        }
    });
}

/// Fold arriving tier updates into the telescope (replace-by-level), stamp their
/// wall-clock + maturity, and trigger one rebuild for the batch.
fn tier_update(rx: Res<TierRx>, mut eroded: ResMut<Eroded>, mut meta: ResMut<TierMeta>, mut ts: ResMut<TerrainState>) {
    let mut any = false;
    while let Ok(msg) = rx.0.lock().unwrap().try_recv() {
        let level = msg.region.level;
        // Aging speed = sim-years in this update / wall-time since the last one.
        let rate = meta
            .0
            .iter()
            .find(|(l, ..)| *l == level)
            .map(|(_, at, ..)| msg.sim_years / at.elapsed().as_secs_f32().max(1e-3))
            .unwrap_or(0.0);
        let tiers = Arc::make_mut(&mut eroded.0);
        tiers.retain(|r| r.level != level);
        tiers.push(msg.region);
        tiers.sort_by_key(|r| r.level);
        meta.0.retain(|(l, ..)| *l != level);
        meta.0.push((level, std::time::Instant::now(), msg.epochs_total, rate, msg.delta_m));
        meta.0.sort_by_key(|(l, ..)| *l);
        any = true;
    }
    // Async meshing (2026-07-03): just mark dirty — the mesher's
    // one-job-in-flight gate is the pacing, and frames never stall.
    if any {
        ts.terrain_dirty = true;
    }
}

/// Fold arriving water snapshots in; water changes ride the same rebuild
/// throttle as tiers (the mesh shows the latest snapshot at each rebuild).
fn water_update(rx: Res<WaterRx>, mut water: ResMut<WaterRes>, mut meta: ResMut<WaterMeta>, mut ts: ResMut<TerrainState>) {
    let mut newest: Option<WaterMsg> = None;
    let mut sim_s = 0.0f32;
    while let Ok(msg) = rx.0.lock().unwrap().try_recv() {
        sim_s += msg.sim_seconds;
        newest = Some(msg);
    }
    if let Some(msg) = newest {
        let rate = meta.0.as_ref().map(|w| sim_s / w.at.elapsed().as_secs_f32().max(1e-3)).unwrap_or(0.0);
        let total = meta.0.as_ref().map(|w| w.total_ss).unwrap_or(0.0) + sim_s;
        meta.0 = Some(WaterStat {
            at: std::time::Instant::now(),
            rate,
            delta_m: msg.delta_m,
            total_ss: total,
            filling: msg.filling,
            froude: msg.froude,
            weather: msg.weather,
            settle: msg.settle,
            drift: msg.drift,
        });
        water.0 = Some(Arc::new(msg.region));
        ts.water_dirty = true; // water-only refresh; no throttle — every state shows
    }
}

fn main() {
    let view = View::default();
    let (mesher_tx, mesher_rx) = spawn_mesher();
    let tier0 = build_tier0(&view);
    let focus = SharedFocus(Arc::new(Mutex::new((view.focus.x, view.focus.y, view.level))));
    let (tx, rx) = std::sync::mpsc::channel::<TierMsg>();
    // Bounded: when streaming per-step, the worker's blocking send paces the
    // sim to exactly what the view consumes — sim-time slows instead of frames
    // being skipped. (Burst mode sends rarely and never fills 2 slots.)
    let (wtx, wrx) = std::sync::mpsc::sync_channel::<WaterMsg>(2);
    spawn_fine_tiers(&view, tier0.clone(), tx, wtx, focus.clone());
    let meta = TierMeta(tier0.iter().map(|r| (r.level, std::time::Instant::now(), FluvialParams::default().epochs, 0.0, f32::INFINITY)).collect());
    let eroded = Eroded(Arc::new(tier0));
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: if std::env::var_os("VIVARIUM_AUTOSHOT").is_some() {
                    "[autoshot] vivarium — worldview (verification run)".into()
                } else {
                    "vivarium — worldview (cube-sphere frame)".into()
                },
                resolution: bevy::window::WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(SKY))
        .insert_resource(view)
        .insert_resource(eroded)
        .insert_resource(TierRx(Mutex::new(rx)))
        .insert_resource(WaterRx(Mutex::new(wrx)))
        .insert_resource(mesher_tx)
        .insert_resource(mesher_rx)
        .insert_resource(RingChunks::default())
        .insert_resource(WaterRes::default())
        .insert_resource(WaterMeta::default())
        .insert_resource(meta)
        .insert_resource(focus)
        .add_systems(Startup, setup)
        .add_systems(Update, (view_update, tier_update, water_update, mesh_dispatch, mesh_apply, flow_arrow, hud_update, scale_update, maybe_screenshot))
        .run();
}

// --- View state --------------------------------------------------------------------

/// Display/interaction modes, cycled by T and shown top-right.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    /// Plain terrain + water.
    Normal,
    /// Erosion-tier fidelity tint (the old T toggle).
    Tiers,
    /// Pawn-local hydrology on the HUD + a flow arrow above the pawn.
    Water,
    /// Water mode + the pawn drifts with the current (stochastic but plausible).
    Float,
}

impl ViewMode {
    fn name(self) -> &'static str {
        match self {
            ViewMode::Normal => "normal",
            ViewMode::Tiers => "tiers",
            ViewMode::Water => "water",
            ViewMode::Float => "float",
        }
    }
}

#[derive(Resource)]
struct View {
    face: Face,
    /// Cube-sphere sampling level (cell size = quarter-circumference / 2^level).
    level: u8,
    /// Patch width in cells (constant across levels; span = w · cell).
    w: usize,
    /// Focus in face-cell coordinates AT `level` (f64: cells are integers but the
    /// focus glides). Rescaled ×2 / ÷2 when the level changes so the geographic
    /// point is preserved.
    focus: DVec2,
    yaw: f32,
    yaw_target: f32,
    pitch: f32,
    /// The user's chosen angle; auto-pitch raises above it only as far as needed.
    pitch_manual: f32,
    /// Auto-raise the camera when foreground would occlude the focus (slabs'
    /// proven behaviour). Pan/turn re-enable it; K/J grab manual control.
    auto_pitch: bool,
    /// Viewport height in METRES (absolute, so changing level doesn't jump zoom).
    zoom: f32,
    /// Vertical exaggeration (1 = honest).
    vert: f32,
    /// Tint terrain by its erosion-tier fidelity (derived: mode == Tiers).
    tier_debug: bool,
    mode: ViewMode,
}

impl Default for View {
    fn default() -> Self {
        // Defaults match the original vivarium-slabs start view for an honest
        // apples-to-apples: slabs' ZOOM_START was 260 voxel-units = 130 m viewport
        // at 0.5 m/voxel, and L24 cells (~0.6 m) are the nearest sampling to slabs'
        // 0.5 m voxels. Coarser survey views: VIVARIUM_LEVEL=14 + [ / ].
        let level: u8 = std::env::var("VIVARIUM_LEVEL").ok().and_then(|s| s.parse().ok()).unwrap_or(24);
        // 1024 cells ≈ 614 m at L24 — comparable ground coverage to slabs at its
        // default zoom (radius ~537 m). Gen cost scales with w²; the HUD's gen-ms
        // reports it honestly (memoization is the eventual fix, not smaller worlds).
        let w: usize = std::env::var("VIVARIUM_W").ok().and_then(|s| s.parse().ok()).unwrap_or(1024);
        let n = (1u64 << level) as f64;
        let span = w as f64 * cell_size_m(level, Planet::EARTH.radius_m);
        let _ = span;
        // Start focus in face cells at `level` (env for scripted/reproducible views —
        // the scan_land example in vivarium-world prints good coastal candidates).
        // Default focus: the known massif (a good mountainside), not mid-face ocean.
        let fi = std::env::var("VIVARIUM_FOCUS_I").ok().and_then(|s| s.parse().ok()).unwrap_or(n * (5184.0 / 16384.0));
        let fj = std::env::var("VIVARIUM_FOCUS_J").ok().and_then(|s| s.parse().ok()).unwrap_or(n * (12928.0 / 16384.0));
        // VIVARIUM_PITCH (radians) forces a fixed angle with auto off — for
        // scripted screenshots (negative = look up).
        let manual_pitch = std::env::var("VIVARIUM_PITCH").ok().and_then(|s| s.parse::<f32>().ok());
        Self {
            face: Face::ZPos,
            level,
            w,
            focus: DVec2::new(fi, fj),
            yaw: YAW_START,
            yaw_target: YAW_START,
            pitch: manual_pitch.unwrap_or(ISO_PITCH),
            pitch_manual: manual_pitch.unwrap_or(ISO_PITCH),
            auto_pitch: manual_pitch.is_none(),
            zoom: std::env::var("VIVARIUM_ZOOM").ok().and_then(|s| s.parse().ok()).unwrap_or(130.0),
            vert: std::env::var("VIVARIUM_VERT").ok().and_then(|s| s.parse().ok()).unwrap_or(1.0),
            tier_debug: std::env::var("VIVARIUM_TIERDEBUG").map(|v| v != "0").unwrap_or(false),
            mode: if std::env::var("VIVARIUM_TIERDEBUG").map(|v| v != "0").unwrap_or(false) { ViewMode::Tiers } else { ViewMode::Normal },
        }
    }
}

impl View {
    fn cell_m(&self) -> f64 {
        cell_size_m(self.level, Planet::EARTH.radius_m)
    }
    /// Clamp the focus so the sampled window stays on the face.
    fn clamp_focus(&mut self) {
        let n = (1u64 << self.level) as f64;
        let half = self.w as f64 * 0.5;
        self.focus.x = self.focus.x.clamp(half, n - half);
        self.focus.y = self.focus.y.clamp(half, n - half);
    }
}

// --- Terrain state -----------------------------------------------------------------

#[derive(Resource)]
struct TerrainState {
    ground: Option<Entity>,
    water: Option<Entity>,
    /// What the current meshes were built from.
    built_level: u8,
    origin: (u32, u32),
    /// f64 anchor (metres, face coords) at the patch centre; every f32 the GPU or
    /// camera sees is relative to this (floating origin).
    anchor_m: DVec2,
    /// The sampled fields (kept so the camera/HUD can read the height under the focus).
    fields: Option<SurfacePatch>,
    /// The water field as sampled (static sea-fill), BEFORE the live overlay —
    /// kept (Arc: shared with mesher jobs) for water-only refreshes.
    base_water: Option<Arc<Vec<f32>>>,
    /// Flat post-overlay terrain heights — the water-only job's ground truth.
    heights: Option<Arc<Vec<f32>>>,
    /// A new live water state arrived: rebuild ONLY the water mesh.
    water_dirty: bool,
    /// Tier state changed (erosion/write-back/tint toggle): full rebuild wanted.
    terrain_dirty: bool,
    /// One job of each kind in flight at most — the natural throttle.
    full_inflight: bool,
    water_inflight: bool,
    /// Relief range of the sampled window (m above sea) — the HUD's honest answer
    /// to "is there height here, and how much?"
    h_min: f32,
    h_max: f32,
    /// Generation + meshing time of the last rebuild — the memoization instrument.
    gen_ms: f32,
    ground_mat: Handle<StandardMaterial>,
    water_mat: Handle<StandardMaterial>,
}

#[derive(Component)]
struct IsoCamera;
/// The honest pawn: a real 0.5 × 2 m figure, never scaled by zoom or vert. At
/// survey zoom it is sub-pixel — that is the point (a visible "little guy" at a
/// 156 km window would be a false scale cue, which the first cursor was).
#[derive(Component)]
struct Pawn;
/// The focus cursor: a flat screen-relative reticle that reads as "selection",
/// deliberately NOT figure-shaped so it can't be mistaken for a scale reference.
#[derive(Component)]
struct FocusRing;
#[derive(Component)]
struct HudText;

#[derive(Component)]
struct ModeText;

/// A value slot in the HUD table (see the layout in `setup`).
#[derive(Component)]
struct HudSlot(u16);

#[derive(Component)]
struct LegendText;

/// Present ONLY in VIVARIUM_AUTOSHOT runs: tells a human who wanders in whose
/// window this is and what touching it does (input only moves the camera —
/// the sim is untouched; it just reframes the capture).
#[derive(Component)]
struct AutoshotBanner;

const LEGEND: &str = "\
legend                                            [H] hide
fps/gen    frame rate / terrain mesh rebuild cost (ms)
place      face + level, cell size, window ground-width, focus cell
L19/L21    erosion tiers: epochs run, aging speed (y/s), d = relief change/epoch
ss  s/s    water sim-seconds total / sim-seconds per wall-second
d mm       mean |depth change| per burst - the convergence gauge
Fr a/b%    Froude v/sqrt(g*d): <1 tranquil, >1 rushing, 2.0 = pinned at the
           breaking cap (>2 = a bug). a = grid max, b% = share supercritical
filling    deluge to steady state: current d -> target (0.15 x peak), cap left
pawn row   water at the pawn: depth, flow speed, local Fr, suspended load,
           alluvium (loose settled bed), seal (pores plugged by fines)
arrow      flow at the pawn: length ~ log speed (0.5 m/s short .. 16 m/s long),
           colour = regime (blue tranquil > white critical > red breaking)
modes [T]  normal > tiers (fidelity tint) > water (+pawn hydrology, flow arrow)
           > float (pawn drifts with the current)";
#[derive(Component)]
struct ScaleBar;
#[derive(Component)]
struct ScaleLabel;

/// Logical window height (px) — must match the WindowPlugin resolution; the ortho
/// projection maps `zoom` metres onto this, which is what makes the scale bar
/// pixel-accurate.
const WINDOW_H_PX: f32 = 720.0;

fn setup(mut commands: Commands, view: Res<View>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: view.zoom },
            far: STANDOFF + 2_000_000.0,
            near: -1.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
        IsoCamera,
        DistanceFog { color: SKY, falloff: FogFalloff::Linear { start: STANDOFF, end: STANDOFF + 2000.0 }, ..default() },
    ));

    let ground_mat = materials.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.95, cull_mode: None, ..default() });
    let water_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.2,
        reflectance: 0.3,
        cull_mode: None,
        ..default()
    });
    commands.insert_resource(TerrainState {
        ground: None,
        water: None,
        built_level: 0,
        origin: (0, 0),
        anchor_m: DVec2::ZERO,
        fields: None,
        base_water: None,
        heights: None,
        water_dirty: false,
        terrain_dirty: false,
        full_inflight: false,
        water_inflight: false,
        h_min: 0.0,
        h_max: 0.0,
        gen_ms: 0.0,
        ground_mat,
        water_mat,
    });

    // The honest pawn (0.5 × 2 m, world scale) + the flat reticle cursor.
    let pawn_mesh = meshes.add(Cuboid::new(0.5, 2.0, 0.5));
    let pawn_mat = materials.add(StandardMaterial { base_color: Color::srgb(0.85, 0.18, 0.18), perceptual_roughness: 0.9, ..default() });
    commands.spawn((Mesh3d(pawn_mesh), MeshMaterial3d(pawn_mat), Transform::default(), Pawn));
    let ring_mesh = meshes.add(Cylinder::new(0.5, 0.02));
    let ring_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.90, 0.30, 0.15, 0.30),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn((Mesh3d(ring_mesh), MeshMaterial3d(ring_mat), Transform::default(), FocusRing));

    // Slabs' proven form-lighting: warm grazing key + cool fill + low ambient.
    commands.spawn((
        DirectionalLight { color: Color::srgb(1.0, 0.98, 0.92), shadows_enabled: false, illuminance: 6500.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.6, -0.55, 0.45), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight { color: Color::srgb(0.58, 0.70, 0.92), shadows_enabled: false, illuminance: 1900.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.5, -0.4, -0.45), Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight { color: SKY, brightness: 190.0, affects_lightmapped_meshes: true });

    // The HUD is a table of fixed slots: static labels (dim) + value spans
    // (ink) that FLASH briefly when a value changes after being stable — the
    // `watch -d` idea as an instrument, self-muting for per-frame counters.
    let font = TextFont { font_size: 15.0, ..default() };
    let ink = TextColor(Color::srgb(0.08, 0.09, 0.10));
    let dim = TextColor(Color::srgb(0.36, 0.38, 0.40));
    // (label-before, slot-id) pairs; a trailing label closes each row.
    let layout: &[(&str, Option<u16>)] = &[
        ("worldview  fps ", Some(0)), ("  gen ", Some(1)), ("\n", None),
        ("place   ", Some(2)), ("  cell ", Some(3)), ("  window ", Some(4)), ("  at ", Some(5)), ("\n", None),
        ("view    facing ", Some(6)), ("  angle ", Some(7)), ("  zoom ", Some(8)), ("  vert ", Some(9)), ("\n", None),
        ("land    elev ", Some(10)), ("  relief ", Some(11)), ("  age ", Some(12)), ("\n", None),
        ("sim     ", Some(13)), ("", Some(14)), ("", Some(15)), ("\n", None),
        ("water   ", Some(16)), ("", Some(17)), ("", Some(18)), ("", Some(19)), ("", Some(20)), ("\n", None),
        ("", Some(21)), ("", Some(22)), ("", Some(23)), ("", Some(24)), ("", Some(25)), ("", Some(26)), ("", Some(27)),
    ];
    commands
        .spawn((
            Text::new(""),
            font.clone(),
            ink,
            Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(10.0), padding: UiRect::all(Val::Px(6.0)), ..default() },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
            HudText,
        ))
        .with_children(|p| {
            for (label, slot) in layout {
                if !label.is_empty() {
                    p.spawn((TextSpan::new(label.replace("\\n", "\n")), font.clone(), dim));
                }
                if let Some(id) = slot {
                    p.spawn((TextSpan::new(""), font.clone(), ink, HudSlot(*id)));
                }
            }
        });
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_some() {
        commands
            .spawn((Node {
                position_type: PositionType::Absolute,
                top: Val::Px(210.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },))
            .with_children(|p| {
                p.spawn((
                    Text::new("AUTOSHOT VERIFICATION"),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.12, 0.07, 0.0)),
                    Node { padding: UiRect::all(Val::Px(8.0)), ..default() },
                    BackgroundColor(Color::srgba(1.0, 0.72, 0.18, 0.88)),
                    AutoshotBanner,
                ));
            });
    }

    // Legend ([H] toggles) — the key that explains the instruments.
    commands.spawn((
        Text::new(LEGEND),
        TextFont { font_size: 14.0, ..default() },
        ink,
        Node { position_type: PositionType::Absolute, top: Val::Px(150.0), left: Val::Px(10.0), padding: UiRect::all(Val::Px(6.0)), display: Display::None, ..default() },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.28)),
        LegendText,
    ));
    commands.spawn((
        Text::new("mode: normal"),
        TextFont { font_size: 15.0, ..default() },
        TextColor(Color::srgb(0.08, 0.09, 0.10)),
        Node { position_type: PositionType::Absolute, top: Val::Px(8.0), right: Val::Px(10.0), padding: UiRect::all(Val::Px(6.0)), ..default() },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        ModeText,
    ));

    // Map scale bar (bottom-left): a round-number length whose pixel width is exact
    // (ortho: zoom metres map onto WINDOW_H_PX). The bar is the truth-teller the
    // first cursor wasn't — scale_update keeps it current.
    commands.spawn((
        Node { position_type: PositionType::Absolute, bottom: Val::Px(14.0), left: Val::Px(12.0), height: Val::Px(5.0), width: Val::Px(100.0), ..default() },
        BackgroundColor(Color::srgba(0.08, 0.09, 0.10, 0.85)),
        ScaleBar,
    ));
    commands.spawn((
        Text::new("…"),
        TextFont { font_size: 13.0, ..default() },
        TextColor(Color::srgb(0.08, 0.09, 0.10)),
        Node { position_type: PositionType::Absolute, bottom: Val::Px(22.0), left: Val::Px(12.0), ..default() },
        ScaleLabel,
    ));

    println!("[worldview] WASD pan · wheel zoom · Q/E rotate · K/J angle · Y auto-angle · [ ] sampling level · T mode · H legend");
}

/// Keep the scale bar honest: pick the 1/2/5×10ᵏ length nearest ~220 px at the
/// current zoom, size the bar to its exact pixel width, and label it.
fn scale_update(view: Res<View>, mut bar: Query<&mut Node, With<ScaleBar>>, mut label: Query<&mut Text, With<ScaleLabel>>) {
    let px_per_m = WINDOW_H_PX / view.zoom;
    let target_m = 220.0 / px_per_m;
    let pow = 10f32.powf(target_m.log10().floor());
    let nice = [1.0, 2.0, 5.0, 10.0]
        .iter()
        .map(|k| k * pow)
        .min_by(|a, b| (a - target_m).abs().partial_cmp(&(b - target_m).abs()).unwrap())
        .unwrap();
    if let Ok(mut n) = bar.single_mut() {
        n.width = Val::Px(nice * px_per_m);
    }
    if let Ok(mut t) = label.single_mut() {
        t.0 = if nice >= 1000.0 { format!("{:.0} km", nice / 1000.0) } else { format!("{nice:.0} m") };
    }
}

// --- Input + camera ----------------------------------------------------------------

fn view_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    mut view: ResMut<View>,
    mut ts: ResMut<TerrainState>,
    mut cam: Query<&mut Transform, (With<IsoCamera>, Without<Pawn>, Without<FocusRing>)>,
    mut cam_proj: Query<&mut Projection, With<IsoCamera>>,
    mut fog: Query<&mut DistanceFog, With<IsoCamera>>,
    mut pawn: Query<&mut Transform, (With<Pawn>, Without<IsoCamera>, Without<FocusRing>)>,
    mut ring: Query<&mut Transform, (With<FocusRing>, Without<IsoCamera>, Without<Pawn>)>,
    shared: Res<SharedFocus>,
    water: Res<WaterRes>,
    wmeta: Res<WaterMeta>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        view.mode = match view.mode {
            ViewMode::Normal => ViewMode::Tiers,
            ViewMode::Tiers => ViewMode::Water,
            ViewMode::Water => ViewMode::Float,
            ViewMode::Float => ViewMode::Normal,
        };
        let tint = view.mode == ViewMode::Tiers;
        if view.tier_debug != tint {
            view.tier_debug = tint;
            ts.terrain_dirty = true; // tint is baked into vertex colours — rebuild
        }
    }
    *shared.0.lock().unwrap() = (view.focus.x, view.focus.y, view.level);
    let dt = time.delta_secs();
    // FLOAT mode: the pawn drifts with the current, at the pace the water
    // animation itself runs (wall-dt × displayed sim-rate), with a small
    // deterministic wobble (coordinate-hashed, §8 — no RNG) for the plausible
    // stochastic wander of a floating body.
    if view.mode == ViewMode::Float {
        if let Some(wr) = &water.0 {
            let c = CellId::from_face_ij(view.face, view.focus.x as u32, view.focus.y as u32, view.level);
            if let (Some(d), Some((vx, vy))) = (wr.depth_m(c), wr.velocity_m_s(c)) {
                if d > 0.05 && (vx != 0.0 || vy != 0.0) {
                    use vivarium_world::noise::hash01;
                    let rate = wmeta.0.as_ref().map(|w| w.rate).unwrap_or(1.0).clamp(0.0, 30.0) as f64;
                    let wob = (hash01(13, (view.focus.x * 4.0) as i64, (view.focus.y * 4.0) as i64) - 0.5) * 0.9;
                    let (sw, cw) = wob.sin_cos();
                    let (rx, ry) = (vx * cw - vy * sw, vx * sw + vy * cw);
                    let cell = view.cell_m();
                    view.focus.x += rx * dt as f64 * rate / cell;
                    view.focus.y += ry * dt as f64 * rate / cell;
                    view.clamp_focus();
                }
            }
        }
    }

    // Pan in the camera frame, in metres, converted to face cells.
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { dir.x += 1.0; }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) { dir.x -= 1.0; }
    if dir != Vec2::ZERO {
        let (s, c) = view.yaw.sin_cos();
        let world = Vec2::new(-dir.x * c + dir.y * s, dir.x * s + dir.y * c).normalize_or_zero();
        let step_m = (PAN_RATE * view.zoom * dt) as f64;
        let cell = view.cell_m();
        view.focus += DVec2::new(world.x as f64, world.y as f64) * (step_m / cell);
        view.clamp_focus();
    }
    for ev in wheel.read() {
        if ev.y > 0.0 {
            view.zoom /= ZOOM_STEP;
        } else if ev.y < 0.0 {
            view.zoom *= ZOOM_STEP;
        }
        let span = (view.w as f64 * view.cell_m()) as f32;
        view.zoom = view.zoom.clamp(span * 0.02, span * 1.5);
    }
    let turning = keys.just_pressed(KeyCode::KeyQ) || keys.just_pressed(KeyCode::KeyE);
    if keys.just_pressed(KeyCode::KeyQ) { view.yaw_target += YAW_STEP; }
    if keys.just_pressed(KeyCode::KeyE) { view.yaw_target -= YAW_STEP; }
    if (view.yaw - view.yaw_target).abs() > 1e-4 {
        view.yaw = lerp_angle(view.yaw, view.yaw_target, (ROT_LERP * dt).clamp(0.0, 1.0));
    }

    // Angle: slabs' proven interaction. Moving/turning returns to the default
    // framing with auto-pitch on; K/J grab manual control instantly (auto off);
    // Y toggles auto explicitly. Auto raises the camera only as far as needed to
    // keep the focus unoccluded by foreground terrain.
    if dir != Vec2::ZERO || turning {
        view.pitch_manual = ISO_PITCH;
        view.auto_pitch = true;
    }
    if keys.pressed(KeyCode::KeyK) {
        view.auto_pitch = false;
        view.pitch_manual = (view.pitch_manual + PITCH_RATE * dt).min(PITCH_MAX);
    }
    if keys.pressed(KeyCode::KeyJ) {
        view.auto_pitch = false;
        view.pitch_manual = (view.pitch_manual - PITCH_RATE * dt).max(PITCH_MIN);
    }
    if keys.just_pressed(KeyCode::KeyY) { view.auto_pitch = !view.auto_pitch; }

    // Sampling level: same geographic point, finer/coarser cells. Focus is in cells
    // at the current level, so it rescales by 2 exactly.
    if keys.just_pressed(KeyCode::BracketRight) && view.level < vivarium_world::sphere::MAX_LEVEL {
        view.level += 1;
        view.focus *= 2.0;
        view.clamp_focus();
    }
    if keys.just_pressed(KeyCode::BracketLeft) && view.level > 6 {
        view.level -= 1;
        view.focus *= 0.5;
        view.clamp_focus();
    }

    // Resolve the pitch: the manual angle, raised by auto only as far as the
    // foreground demands; fast up, gentle down (slabs' proven feel).
    let focus_h_raw = height_at_focus(&view, &ts);
    let target = if view.auto_pitch {
        view.pitch_manual.max(required_pitch(&view, &ts, focus_h_raw))
    } else {
        view.pitch_manual
    }
    .clamp(PITCH_MIN, PITCH_MAX);
    let rate = if target > view.pitch { PITCH_LERP_UP } else { PITCH_LERP_DOWN };
    view.pitch += (target - view.pitch) * (rate * dt).clamp(0.0, 1.0);

    // Camera + pawn + ring, all relative to the terrain anchor (floating origin).
    let cell = view.cell_m();
    let focus_m = view.focus * cell;
    let rel = focus_m - ts.anchor_m;
    let focus_h = focus_h_raw * view.vert;
    let aim_base = Vec3::new(rel.x as f32, focus_h, rel.y as f32);

    let look = (Vec3::new(view.yaw.sin(), 0.0, view.yaw.cos()) * view.pitch.cos() + Vec3::NEG_Y * view.pitch.sin()).normalize();
    let forward_h = Vec3::new(view.yaw.sin(), 0.0, view.yaw.cos());
    // Sign-preserving clamp so the focus stays put when looking up (negative pitch).
    let s = view.pitch.sin();
    let denom = if s >= 0.0 { s.max(0.15) } else { s.min(-0.15) };
    let aim = aim_base + forward_h * (view.zoom * FOCUS_BELOW_CENTER / denom);
    if let Ok(mut t) = cam.single_mut() {
        *t = Transform::from_translation(aim - look * STANDOFF).looking_at(aim, Vec3::Y);
    }
    if let Ok(mut proj) = cam_proj.single_mut() {
        if let Projection::Orthographic(o) = proj.as_mut() {
            o.scaling_mode = ScalingMode::FixedVertical { viewport_height: view.zoom };
            // Looking up: clip away foreground nearer than ~the focus depth so it
            // can't wall off the view (slabs' look-up behaviour).
            o.near = if view.pitch < 0.0 { STANDOFF - view.zoom * 0.05 } else { -1.0 };
        }
    }
    // Mild haze for a survey instrument: begin a full viewport beyond the aim so
    // the working window stays clear (slabs' tighter band at these zooms whites
    // out most of the scene).
    if let Ok(mut f) = fog.single_mut() {
        f.falloff = FogFalloff::Linear { start: STANDOFF + view.zoom * 1.2, end: STANDOFF + view.zoom * 8.0 };
    }
    // Pawn: a real 2 m figure standing on the (possibly exaggerated) ground —
    // never scaled by zoom, so it is only visible when the view is actually at
    // human scale. Ring: screen-relative cursor, flat on the ground.
    if let Ok(mut p) = pawn.single_mut() {
        // In deep water the pawn FLOATS: body submerged to the shoulders (head
        // ~0.4 m proud of the surface), bobbing on two incommensurate sines —
        // gentle in tranquil water, choppy toward the breaking regime. Pure
        // view cosmetics on the wall clock; world state is untouched.
        let mut y = 1.0; // standing: cuboid centre 1 m above its base
        if let Some(wr) = &water.0 {
            let c = CellId::from_face_ij(view.face, view.focus.x as u32, view.focus.y as u32, view.level);
            if let Some(d) = wr.depth_m(c) {
                if d >= 1.2 {
                    let v = wr.speed_m_s(c).unwrap_or(0.0) as f32;
                    let fr = (v / (9.8 * d as f32).sqrt()).clamp(0.0, 2.0);
                    let t = time.elapsed_secs();
                    let chop = 0.05 + 0.12 * (fr / 2.0);
                    let bob = chop * ((t * 3.5).sin() + 0.4 * (t * 8.2 + 1.7).sin());
                    y = ((d as f32 - 1.6) * view.vert + 1.0 + bob).max(1.0);
                }
            }
        }
        *p = Transform::from_translation(aim_base + Vec3::Y * y);
    }
    if let Ok(mut r) = ring.single_mut() {
        let s = view.zoom * 0.016;
        *r = Transform::from_translation(aim_base).with_scale(Vec3::new(s, 1.0, s));
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let d = (b - a + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    a + d * t
}

/// Height (m above sea, un-exaggerated) at face-cell coords `(fi, fj)`, if that
/// point is inside the currently sampled patch.
fn sampled_height(view: &View, ts: &TerrainState, fi: f64, fj: f64) -> Option<f32> {
    let fields = ts.fields.as_ref()?;
    if ts.built_level != view.level {
        return None;
    }
    let x = (fi - ts.origin.0 as f64).floor() as isize;
    let y = (fj - ts.origin.1 as f64).floor() as isize;
    let w = view.w as isize;
    if x >= 0 && x < w && y >= 0 && y < w {
        Some(fields.height.get(x, y) - SEA_LEVEL_M as f32)
    } else {
        None
    }
}

/// Height (m above sea, un-exaggerated) under the focus.
/// Flow arrow above the pawn (Water/Float modes): direction = local current,
/// length = normalized speed. Drawn above the water surface when submerged.
fn flow_arrow(view: Res<View>, _ts: Res<TerrainState>, water: Res<WaterRes>, pawn: Query<&Transform, With<Pawn>>, mut gizmos: Gizmos) {
    if !matches!(view.mode, ViewMode::Water | ViewMode::Float) {
        return;
    }
    let Some(wr) = &water.0 else { return };
    let Ok(pt) = pawn.single() else { return };
    let c = CellId::from_face_ij(view.face, view.focus.x as u32, view.focus.y as u32, view.level);
    let (Some(d), Some((vx, vy))) = (wr.depth_m(c), wr.velocity_m_s(c)) else { return };
    if d < 0.01 {
        return;
    }
    let speed = ((vx * vx + vy * vy) as f32).sqrt();
    if speed < 1e-3 {
        return;
    }
    let dir = Vec3::new(vx as f32, 0.0, vy as f32).normalize();
    // Log length: the measured node-speed distribution (velocity_histogram)
    // spans ~0.1..16 m/s with its living-water body at 0.5-2.5 — a linear
    // scale saturating at 2.5 made very different speeds look identical.
    let len = 1.5 * (1.0 + speed / 0.25).ln();
    // Colour = flow REGIME (more informative than raw speed): blue tranquil
    // (Fr<1) -> white critical -> red at the breaking cap (Fr~2). The same
    // speed reads differently in deep vs thin water because it IS different.
    let fr = (speed / (9.8 * d as f32).sqrt().max(1e-3)).clamp(0.0, 2.0);
    let t = fr / 2.0;
    let color = Color::srgb(
        (0.15 + 1.7 * t).min(1.0),
        (0.6 + 0.8 * (1.0 - (2.0 * t - 1.0).abs())).min(1.0) * (1.0 - 0.55 * (t - 0.5).max(0.0) * 2.0),
        (1.0 - 1.6 * t).max(0.1),
    );
    // Above the head, or above the water surface if that's higher.
    let ground_y = pt.translation.y - 1.0; // pawn cuboid is 2 m, centered
    let y = (pt.translation.y + 1.6).max(ground_y + d as f32 * view.vert + 0.6);
    let start = Vec3::new(pt.translation.x, y, pt.translation.z);
    gizmos.arrow(start, start + dir * len, color);
}

fn height_at_focus(view: &View, ts: &TerrainState) -> f32 {
    sampled_height(view, ts, view.focus.x, view.focus.y).unwrap_or(0.0)
}

/// The minimum camera pitch that clears the terrain between the camera and the
/// focus (slabs' fan probe, over the sampled height field): march rays from the
/// focus TOWARD the camera, track the steepest sight-line blocker, and return the
/// angle that clears it plus a small margin. Exaggeration-aware.
fn required_pitch(view: &View, ts: &TerrainState, focus_h_raw: f32) -> f32 {
    let cell = view.cell_m();
    let base = view.yaw + std::f32::consts::PI; // horizontal direction to the camera
    let max_dist_m = (view.zoom * 2.0).max(cell as f32 * 8.0);
    let step_m = (cell as f32).max(view.zoom / 256.0);
    let mut max_tan = 0.0f32;
    for da in [-0.28f32, -0.12, 0.0, 0.12, 0.28] {
        let a = base + da;
        let dir = DVec2::new(a.sin() as f64, a.cos() as f64);
        let mut x = step_m;
        while x <= max_dist_m {
            let p = view.focus + dir * (x as f64 / cell);
            if let Some(h) = sampled_height(view, ts, p.x, p.y) {
                max_tan = max_tan.max((h - focus_h_raw) * view.vert / x);
            }
            x += step_m;
        }
    }
    max_tan.atan() + 0.06 // clear the tallest occluder plus ~3.5°
}

// --- Terrain sampling + meshing ------------------------------------------------------
//
// ASYNC (2026-07-03): sampling + mesh construction are pure functions of Arc'd
// sim state, so they run on a mesher thread; the render thread keeps showing
// the previous mesh until a finished replacement arrives. No frame stalls, no
// wall-clock throttles — the one-job-in-flight gate is the natural pacing, and
// the water sim (paced by the VIEW's drain, not by meshing) runs free.

enum MeshJob {
    Full { face: Face, level: u8, w: usize, oi: u32, oj: u32, vert: f32, tier_debug: bool, tiers: Arc<Vec<ErodedRegion>>, water: Option<Arc<WaterRegion>> },
    /// A horizon chunk beyond the primary window — built with its OWN anchor
    /// (entity Transform bridges to the global one, so pans are free).
    Ring { face: Face, level: u8, w: usize, oi: u32, oj: u32, vert: f32, tier_debug: bool, tiers: Arc<Vec<ErodedRegion>>, water: Option<Arc<WaterRegion>> },
    WaterOnly { face: Face, level: u8, w: usize, oi: u32, oj: u32, vert: f32, anchor: DVec2, heights: Arc<Vec<f32>>, base_water: Arc<Vec<f32>>, live: Arc<WaterRegion> },
}

enum MeshDone {
    Full { level: u8, origin: (u32, u32), anchor: DVec2, fields: SurfacePatch, heights: Arc<Vec<f32>>, base_water: Arc<Vec<f32>>, ground: Mesh, water: Option<Mesh>, h_min: f32, h_max: f32, gen_ms: f32 },
    Ring { level: u8, origin: (u32, u32), anchor: DVec2, ground: Mesh, water: Option<Mesh> },
    WaterOnly { level: u8, origin: (u32, u32), water: Option<Mesh> },
}

/// Horizon chunks beyond the primary window (Joseph: expand the rendered mesh
/// progressively as cycles free up, Hilbert-ordered). Same view level (stage 1;
/// coarser far rings are stage 2). Keyed by chunk origin in view-level cells.
#[derive(Resource, Default)]
struct RingChunks {
    chunks: std::collections::HashMap<(u32, u32), (Entity, Option<Entity>, DVec2)>,
    inflight: Option<(u32, u32)>,
    /// Bumped when tiers/tint change: chunks built before this are stale.
    rev: u64,
    built_rev: std::collections::HashMap<(u32, u32), u64>,
}

#[derive(Resource)]
struct MesherTx(std::sync::mpsc::Sender<MeshJob>);
#[derive(Resource)]
struct MesherRx(Mutex<std::sync::mpsc::Receiver<MeshDone>>);

fn spawn_mesher() -> (MesherTx, MesherRx) {
    let (jtx, jrx) = std::sync::mpsc::channel::<MeshJob>();
    let (dtx, drx) = std::sync::mpsc::channel::<MeshDone>();
    std::thread::spawn(move || {
        while let Ok(job) = jrx.recv() {
            let done = match job {
                MeshJob::Full { face, level, w, oi, oj, vert, tier_debug, tiers, water } => {
                    build_full(face, level, w, oi, oj, vert, tier_debug, &tiers, water.as_deref())
                }
                MeshJob::Ring { face, level, w, oi, oj, vert, tier_debug, tiers, water } => {
                    match build_full(face, level, w, oi, oj, vert, tier_debug, &tiers, water.as_deref()) {
                        MeshDone::Full { level, origin, anchor, ground, water, .. } => MeshDone::Ring { level, origin, anchor, ground, water },
                        other => other,
                    }
                }
                MeshJob::WaterOnly { face, level, w, oi, oj, vert, anchor, heights, base_water, live } => {
                    let cell = cell_size_m(level, Planet::EARTH.radius_m);
                    let mut wtr = vec![0.0f32; w * w];
                    for j in 0..w {
                        for i in 0..w {
                            let k = j * w + i;
                            let mut v = base_water[k];
                            let c = CellId::from_face_ij(face, oi + i as u32, oj + j as u32, level);
                            if let Some(surf) = live.surface_m(c) {
                                v = v.max(surf as f32 - heights[k]);
                            }
                            wtr[k] = v;
                        }
                    }
                    let turbidity_of = |x: usize, y: usize| -> f32 {
                        let c = CellId::from_face_ij(face, oi + x as u32, oj + y as u32, level);
                        let sm = live.suspended_m(c).unwrap_or(0.0);
                        let d = live.depth_m(c).unwrap_or(0.0).max(0.02);
                        ((sm / d) as f32 * 8.0).clamp(0.0, 0.75)
                    };
                    MeshDone::WaterOnly { level, origin: (oi, oj), water: build_water_mesh(&heights, &wtr, w, cell, anchor, (oi, oj), vert, &turbidity_of) }
                }
            };
            if dtx.send(done).is_err() {
                return;
            }
        }
    });
    (MesherTx(jtx), MesherRx(Mutex::new(drx)))
}

/// The full pipeline, off-thread: sample the telescope, overlay live water by
/// SURFACE elevation on the SIMULATED bed (the honesty rules), mesh both.
#[allow(clippy::too_many_arguments)]
fn build_full(face: Face, level: u8, w: usize, oi: u32, oj: u32, vert: f32, tier_debug: bool, tiers: &[ErodedRegion], live: Option<&WaterRegion>) -> MeshDone {
    let t0 = std::time::Instant::now();
    let mut fields = sample_surface_with(face, level, oi, oj, w, |c| erosion::column_at(c, tiers));
    let mut base_water = vec![0.0f32; w * w];
    for j in 0..w {
        for i in 0..w {
            base_water[j * w + i] = fields.water.get(i as isize, j as isize);
        }
    }
    if let Some(wr) = live {
        for j in 0..w {
            for i in 0..w {
                let cell = CellId::from_face_ij(face, oi + i as u32, oj + j as u32, level);
                if let (Some(surf), Some(bed), Some(dsim)) = (wr.surface_m(cell), wr.bed_m(cell), wr.depth_m(cell)) {
                    if dsim > 0.05 {
                        fields.height.set(i as isize, j as isize, bed as f32);
                    }
                    let d = surf as f32 - fields.height.get(i as isize, j as isize);
                    if d > fields.water.get(i as isize, j as isize) {
                        fields.water.set(i as isize, j as isize, d);
                    }
                }
            }
        }
    }
    let cell = cell_size_m(level, Planet::EARTH.radius_m);
    let anchor = DVec2::new((oi as f64 + w as f64 * 0.5) * cell, (oj as f64 + w as f64 * 0.5) * cell);
    let tier_of = |x: usize, y: usize| erosion::tier_at(CellId::from_face_ij(face, oi + x as u32, oj + y as u32, level), tiers);
    let soil_of = |x: usize, y: usize| -> (f32, f32, f32, f32) {
        live.map_or((0.0, 0.0, 0.0, 0.0), |wr| {
            let c = CellId::from_face_ij(face, oi + x as u32, oj + y as u32, level);
            // Top-layer WETNESS: a surface film (even the sub-render-cutoff
            // 0–12 mm that used to be invisible) saturates the surface; the
            // groundwater store keeps ground damp between storms (0.3 m = the
            // sim's gw_capacity). Films dominate; damp soil shows at ~70%.
            let film = (wr.depth_m(c).unwrap_or(0.0) / 0.012).clamp(0.0, 1.0) as f32;
            let damp = (wr.groundwater_m(c).unwrap_or(0.0) / 0.3).clamp(0.0, 1.0) as f32 * 0.7;
            (
                wr.colmation_at(c).unwrap_or(0.0) as f32,
                wr.sed_bed_m(c).unwrap_or(0.0) as f32,
                wr.armor_at(c).unwrap_or(0.0) as f32,
                film.max(damp),
            )
        })
    };
    let turbidity_of = |x: usize, y: usize| -> f32 {
        live.map_or(0.0, |wr| {
            let c = CellId::from_face_ij(face, oi + x as u32, oj + y as u32, level);
            let sm = wr.suspended_m(c).unwrap_or(0.0);
            let d = wr.depth_m(c).unwrap_or(0.0).max(0.02);
            ((sm / d) as f32 * 8.0).clamp(0.0, 0.75)
        })
    };
    let ground = build_ground_mesh(&fields, w, cell, anchor, (oi, oj), vert, &tier_of, tier_debug, &soil_of);
    let (mut heights, mut wtr) = (vec![0.0f32; w * w], vec![0.0f32; w * w]);
    let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
    for j in 0..w {
        for i in 0..w {
            let h = fields.height.get(i as isize, j as isize);
            heights[j * w + i] = h;
            wtr[j * w + i] = fields.water.get(i as isize, j as isize);
            lo = lo.min(h - SEA_LEVEL_M as f32);
            hi = hi.max(h - SEA_LEVEL_M as f32);
        }
    }
    let water_mesh = build_water_mesh(&heights, &wtr, w, cell, anchor, (oi, oj), vert, &turbidity_of);
    MeshDone::Full {
        level,
        origin: (oi, oj),
        anchor,
        fields,
        heights: Arc::new(heights),
        base_water: Arc::new(base_water),
        ground,
        water: water_mesh,
        h_min: lo,
        h_max: hi,
        gen_ms: t0.elapsed().as_secs_f32() * 1000.0,
    }
}

/// Decide what the mesher should work on next (one job of each kind at a time).
fn mesh_dispatch(view: Res<View>, eroded: Res<Eroded>, water: Res<WaterRes>, mut ts: ResMut<TerrainState>, mut rings: ResMut<RingChunks>, tx: Res<MesherTx>) {
    let drifted = ts.fields.is_some() && {
        let cx = ts.origin.0 as f64 + view.w as f64 * 0.5;
        let cy = ts.origin.1 as f64 + view.w as f64 * 0.5;
        (view.focus.x - cx).abs().max((view.focus.y - cy).abs()) > view.w as f64 / 6.0
    };
    let needs_full = ts.fields.is_none() || ts.built_level != view.level || ts.terrain_dirty || drifted;
    if needs_full && !ts.full_inflight {
        let n = 1u64 << view.level;
        let half = view.w as u64 / 2;
        let oi = ((view.focus.x.round().max(0.0) as u64).clamp(half, n - half) - half) as u32;
        let oj = ((view.focus.y.round().max(0.0) as u64).clamp(half, n - half) - half) as u32;
        if tx.0.send(MeshJob::Full { face: view.face, level: view.level, w: view.w, oi, oj, vert: view.vert, tier_debug: view.tier_debug, tiers: eroded.0.clone(), water: water.0.clone() }).is_ok() {
            ts.full_inflight = true;
            ts.terrain_dirty = false;
        }
    }
    if ts.water_dirty && !ts.water_inflight && !ts.full_inflight && ts.built_level == view.level {
        if let (Some(h), Some(b), Some(live)) = (&ts.heights, &ts.base_water, &water.0) {
            if tx.0.send(MeshJob::WaterOnly { face: view.face, level: view.level, w: view.w, oi: ts.origin.0, oj: ts.origin.1, vert: view.vert, anchor: ts.anchor_m, heights: h.clone(), base_water: b.clone(), live: live.clone() }).is_ok() {
                ts.water_inflight = true;
                ts.water_dirty = false;
            }
        }
    }

    // RINGS: with the center current and the mesher otherwise idle, grow the
    // horizon — nearest ring first, Hilbert order within a ring (the CellId's
    // raw curve index IS the order; spatially coherent jobs, warm caches).
    if ts.full_inflight || ts.water_inflight || rings.inflight.is_some() || ts.fields.is_none() || ts.built_level != view.level {
        return;
    }
    if ts.terrain_dirty {
        rings.rev += 1; // tiers/tint changed; chunks go stale (rebuilt lazily)
    }
    let cw = (view.w / 4).max(64) as u32; // chunk span in cells (¼ window)
    let max_rings: u32 = std::env::var("VIVARIUM_RINGS").ok().and_then(|s| s.parse().ok()).unwrap_or(2);
    let n = (1u64 << view.level) as i64;
    let (o_i, o_j, wv) = (ts.origin.0 as i64, ts.origin.1 as i64, view.w as i64);
    let mut best: Option<(u32, u64, (u32, u32))> = None; // (ring, hilbert, origin)
    for ring in 1..=max_rings as i64 {
        // Chunk origins forming the ring band around the window.
        let (lo_i, hi_i) = (o_i - ring * cw as i64, o_i + wv + (ring - 1) * cw as i64);
        let (lo_j, hi_j) = (o_j - ring * cw as i64, o_j + wv + (ring - 1) * cw as i64);
        let mut cand = Vec::new();
        let mut push = |ci: i64, cj: i64| {
            if ci >= 0 && cj >= 0 && ci + (cw as i64) < n && cj + (cw as i64) < n {
                cand.push((ci as u32, cj as u32));
            }
        };
        let step = cw as i64;
        let mut ci = lo_i;
        while ci <= hi_i {
            push(ci, lo_j);
            push(ci, hi_j);
            ci += step;
        }
        let mut cj = lo_j + step;
        while cj < hi_j {
            push(lo_i, cj);
            push(hi_i, cj);
            cj += step;
        }
        for (ci, cj) in cand {
            let fresh = rings.built_rev.get(&(ci, cj)).map(|r| *r == rings.rev).unwrap_or(false);
            if rings.chunks.contains_key(&(ci, cj)) && fresh {
                continue;
            }
            let h = CellId::from_face_ij(view.face, ci, cj, view.level).0; // Hilbert curve index
            if best.map(|(br, bh, _)| (ring as u32, h) < (br, bh)).unwrap_or(true) {
                best = Some((ring as u32, h, (ci, cj)));
            }
        }
        if best.is_some() {
            break; // nearest ring first
        }
    }
    if let Some((_, _, (ci, cj))) = best {
        // +1 cell overlap: each chunk meshes w-1 quads, so without overlap a
        // one-cell strip goes missing between neighbours (white crack lines).
        if tx.0.send(MeshJob::Ring { face: view.face, level: view.level, w: cw as usize + 1, oi: ci, oj: cj, vert: view.vert, tier_debug: view.tier_debug, tiers: eroded.0.clone(), water: water.0.clone() }).is_ok() {
            rings.inflight = Some((ci, cj));
        }
    }
}

/// Swap in finished meshes (stale results from a level change are dropped).
fn mesh_apply(mut commands: Commands, view: Res<View>, mut meshes: ResMut<Assets<Mesh>>, mut ts: ResMut<TerrainState>, mut rings: ResMut<RingChunks>, rx: Res<MesherRx>) {
    while let Ok(done) = rx.0.lock().unwrap().try_recv() {
        match done {
            MeshDone::Full { level, origin, anchor, fields, heights, base_water, ground, water, h_min, h_max, gen_ms } => {
                ts.full_inflight = false;
                if level != view.level {
                    continue; // stale — dispatcher will send a fresh job
                }
                for e in [ts.ground.take(), ts.water.take()].into_iter().flatten() {
                    commands.entity(e).despawn();
                }
                ts.ground = Some(commands.spawn((Mesh3d(meshes.add(ground)), MeshMaterial3d(ts.ground_mat.clone()), Transform::default())).id());
                if let Some(wm) = water {
                    ts.water = Some(commands.spawn((Mesh3d(meshes.add(wm)), MeshMaterial3d(ts.water_mat.clone()), Transform::default())).id());
                }
                ts.built_level = level;
                ts.origin = origin;
                ts.anchor_m = anchor;
                // Re-bridge ring transforms to the (possibly moved) anchor, and
                // evict chunks that fell behind the horizon (§6: regenerable).
                let cw = (view.w / 4).max(64) as i64;
                let max_r = std::env::var("VIVARIUM_RINGS").ok().and_then(|s| s.parse().ok()).unwrap_or(2i64) + 1;
                let keep_lo_i = origin.0 as i64 - max_r * cw;
                let keep_hi_i = origin.0 as i64 + view.w as i64 + max_r * cw;
                let keep_lo_j = origin.1 as i64 - max_r * cw;
                let keep_hi_j = origin.1 as i64 + view.w as i64 + max_r * cw;
                let mut dead = Vec::new();
                for (&k, &(g, wtr, ca)) in rings.chunks.iter() {
                    if (k.0 as i64) < keep_lo_i || (k.0 as i64) > keep_hi_i || (k.1 as i64) < keep_lo_j || (k.1 as i64) > keep_hi_j {
                        commands.entity(g).despawn();
                        if let Some(e) = wtr {
                            commands.entity(e).despawn();
                        }
                        dead.push(k);
                    } else {
                        let off = Vec3::new((ca.x - anchor.x) as f32, 0.0, (ca.y - anchor.y) as f32);
                        commands.entity(g).insert(Transform::from_translation(off));
                        if let Some(e) = wtr {
                            commands.entity(e).insert(Transform::from_translation(off));
                        }
                    }
                }
                for k in dead {
                    rings.chunks.remove(&k);
                    rings.built_rev.remove(&k);
                }
                ts.fields = Some(fields);
                ts.heights = Some(heights);
                ts.base_water = Some(base_water);
                ts.h_min = h_min;
                ts.h_max = h_max;
                ts.gen_ms = gen_ms;
            }
            MeshDone::Ring { level, origin, anchor, ground, water } => {
                rings.inflight = None;
                if level != view.level {
                    continue; // stale level; eviction below clears the rest
                }
                if let Some((g, wtr, _)) = rings.chunks.remove(&origin) {
                    commands.entity(g).despawn();
                    if let Some(e) = wtr {
                        commands.entity(e).despawn();
                    }
                }
                let off = Vec3::new((anchor.x - ts.anchor_m.x) as f32, 0.0, (anchor.y - ts.anchor_m.y) as f32);
                let g = commands.spawn((Mesh3d(meshes.add(ground)), MeshMaterial3d(ts.ground_mat.clone()), Transform::from_translation(off))).id();
                let wtr = water.map(|wm| commands.spawn((Mesh3d(meshes.add(wm)), MeshMaterial3d(ts.water_mat.clone()), Transform::from_translation(off))).id());
                let rev = rings.rev;
                rings.chunks.insert(origin, (g, wtr, anchor));
                rings.built_rev.insert(origin, rev);
            }
            MeshDone::WaterOnly { level, origin, water } => {
                ts.water_inflight = false;
                if level != ts.built_level || origin != ts.origin {
                    continue; // stale
                }
                if let Some(e) = ts.water.take() {
                    commands.entity(e).despawn();
                }
                if let Some(wm) = water {
                    ts.water = Some(commands.spawn((Mesh3d(meshes.add(wm)), MeshMaterial3d(ts.water_mat.clone()), Transform::default())).id());
                }
            }
        }
    }
}

/// Elevation-ramp colour for the crude rung (materials are uniform soil-on-igneous
/// at this tier, so elevation is the honest signal to paint).
fn ground_color(h_above_sea: f32) -> [f32; 4] {
    let mix = |a: [f32; 3], b: [f32; 3], t: f32| -> [f32; 3] {
        let t = t.clamp(0.0, 1.0);
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
    };
    // Bands scaled to the prior's actual relief (continents ±1500 m + mountains to
    // ~+3300 m): green lowlands to ~1 km, bare stone through ~2 km, snow above.
    const BED: [f32; 3] = [0.45, 0.40, 0.32]; // submerged sediment
    const SHORE: [f32; 3] = [0.70, 0.64, 0.48];
    const GRASS: [f32; 3] = [0.34, 0.52, 0.26];
    const STONE: [f32; 3] = [0.50, 0.49, 0.46];
    const SNOW: [f32; 3] = [0.92, 0.93, 0.94];
    let c = if h_above_sea < 0.0 {
        mix(BED, SHORE, (h_above_sea + 150.0) / 150.0)
    } else if h_above_sea < 80.0 {
        mix(SHORE, GRASS, h_above_sea / 80.0)
    } else if h_above_sea < 2000.0 {
        mix(GRASS, STONE, (h_above_sea - 1000.0) / 1000.0)
    } else {
        mix(STONE, SNOW, (h_above_sea - 2000.0) / 600.0)
    };
    [c[0], c[1], c[2], 1.0]
}

/// Continuous point-mesh over the solid surface (slabs' proven model): one vertex
/// per cell centre, smooth normals from the height gradient (halo makes edges work).
fn build_ground_mesh(
    f: &SurfacePatch,
    w: usize,
    cell: f64,
    anchor: DVec2,
    origin: (u32, u32),
    vert: f32,
    tier_of: &dyn Fn(usize, usize) -> Option<u8>,
    tier_debug: bool,
    soil_of: &dyn Fn(usize, usize) -> (f32, f32, f32, f32),
) -> Mesh {
    use vivarium_world::noise::hash01;
    // Fidelity tints (debug): violet = raw prior (unsimulated), blue = L19 macro,
    // yellow = L21 fine, orange = L24 pawn-scale. Lerped over the terrain colour.
    fn tier_tint(t: Option<u8>) -> [f32; 3] {
        match t {
            None => [0.62, 0.45, 0.78],
            Some(19) => [0.25, 0.55, 0.95],
            Some(21) => [0.95, 0.90, 0.25],
            Some(24) => [1.00, 0.45, 0.20],
            Some(_) => [0.6, 0.6, 0.6],
        }
    }
    let h = |x: isize, y: isize| -> f32 { f.height.get(x, y) };
    let px = |i: usize| ((origin.0 as f64 + i as f64 + 0.5) * cell - anchor.x) as f32;
    let pz = |j: usize| ((origin.1 as f64 + j as f64 + 0.5) * cell - anchor.y) as f32;
    let py = |x: isize, y: isize| (h(x, y) - SEA_LEVEL_M as f32) * vert;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(w * w);
    for j in 0..w {
        for i in 0..w {
            let (x, y) = (i as isize, j as isize);
            positions.push([px(i), py(x, y), pz(j)]);
            let (gi, gj) = (origin.0 as i64 + x as i64, origin.1 as i64 + y as i64);
            // Deterministic per-cell mottle (§8): real ground is not one green.
            let m = 0.88 + 0.24 * hash01(7, gi, gj) as f32;
            let c = ground_color(h(x, y) - SEA_LEVEL_M as f32);
            let mut col = [c[0] * m, c[1] * m, c[2] * m, 1.0];
            // Bed state from the live water sim (Joseph): settled alluvium reads
            // SANDY; a colmated (pore-sealed) bed reads MUDDY; an armored
            // (coarse-lag) bed reads ROCKY grey.
            let (colm, alluvium, armor, wetness) = soil_of(i, j);
            const SAND: [f32; 3] = [0.78, 0.70, 0.52];
            const MUD: [f32; 3] = [0.42, 0.35, 0.25];
            const ROCK: [f32; 3] = [0.52, 0.50, 0.46];
            let ts = (alluvium / 0.3).clamp(0.0, 1.0) * 0.7;
            let tm = colm.clamp(0.0, 1.0) * 0.8;
            let tr = armor.clamp(0.0, 1.0) * 0.6;
            for k in 0..3 {
                col[k] += (ROCK[k] * m - col[k]) * tr;
                col[k] += (SAND[k] * m - col[k]) * ts;
                col[k] += (MUD[k] * m - col[k]) * tm;
            }
            // WET DARKENING (Lekner & Dorf 1988; soil-albedo literature): at
            // surface saturation, porous soil reflects ~0.55× dry, rock ~0.72×,
            // foliage ~0.88× (waxy — its wet look is mostly gloss, which a
            // per-mesh material can't vary; we take the honest value shift
            // only). Wet surfaces also gain ~15% chroma (the film suppresses
            // white surface scatter — wet pebbles look richer, not just dark).
            if wetness > 0.001 {
                let h_rel = h(x, y) - SEA_LEVEL_M as f32;
                let soil_like = (ts + tm + (1.0 - (h_rel / 80.0).clamp(0.0, 1.0))).clamp(0.0, 1.0);
                let rock_like = (tr + ((h_rel - 1000.0) / 1000.0).clamp(0.0, 1.0)).clamp(0.0, 1.0) * (1.0 - soil_like);
                let veg_like = (1.0 - soil_like - rock_like).max(0.0);
                let darken = soil_like * 0.45 + rock_like * 0.28 + veg_like * 0.12;
                let mean = (col[0] + col[1] + col[2]) / 3.0;
                for k in 0..3 {
                    let chroma = mean + (col[k] - mean) * (1.0 + 0.15 * wetness);
                    col[k] = chroma * (1.0 - darken * wetness);
                }
            }
            if tier_debug {
                let t = tier_tint(tier_of(i, j));
                for k in 0..3 {
                    col[k] = col[k] * 0.55 + t[k] * 0.45;
                }
            }
            colors.push(col);
            let nrm = Vec3::new(py(x - 1, y) - py(x + 1, y), 2.0 * cell as f32, py(x, y - 1) - py(x, y + 1)).normalize();
            normals.push([nrm.x, nrm.y, nrm.z]);
        }
    }
    let mut indices: Vec<u32> = Vec::with_capacity((w - 1) * (w - 1) * 6);
    for j in 0..w - 1 {
        for i in 0..w - 1 {
            let (a, b, c, d) = ((j * w + i) as u32, (j * w + i + 1) as u32, ((j + 1) * w + i) as u32, ((j + 1) * w + i + 1) as u32);
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Translucent water point-mesh at the sea surface, depth-shaded (Beer–Lambert per
/// metre). Quads only where all four corners are wet (slabs' known 1-cell shore
/// inset — acceptable, noted).
fn build_water_mesh(heights: &[f32], water: &[f32], w: usize, cell: f64, anchor: DVec2, origin: (u32, u32), vert: f32, turbidity_of: &dyn Fn(usize, usize) -> f32) -> Option<Mesh> {
    let px = |i: usize| ((origin.0 as f64 + i as f64 + 0.5) * cell - anchor.x) as f32;
    let pz = |j: usize| ((origin.1 as f64 + j as f64 + 0.5) * cell - anchor.y) as f32;

    let mut wet = vec![false; w * w];
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(w * w);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(w * w);
    // The surface field first, so normals can be taken from the ACTUAL water
    // topology — a flat [0,1,0] normal lights a descending stream like a
    // horizontal mirror (Joseph: "it seems to not reflect light differently").
    let mut surf_v = vec![0.0f32; w * w];
    for k in 0..w * w {
        surf_v[k] = (heights[k] + water[k] - SEA_LEVEL_M as f32) * vert;
    }
    let sv = |x: isize, y: isize| surf_v[(y.clamp(0, w as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize];
    for j in 0..w {
        for i in 0..w {
            let depth = water[j * w + i];
            // A rain film is invisible in reality too: only render standing water
            // (at high rain the whole window carries a cm-scale draining sheet,
            // which at any uniform alpha reads as FOG — cut it, and fade alpha in
            // from the cutoff so pools don't pop).
            wet[j * w + i] = depth > 0.025;
            // Water surface = solid top + depth (baseline: the sea plane at y = 0).
            let surf = surf_v[j * w + i];
            // Depth-graded: riffles (5-10 cm) faint ribbons, streams strong,
            // mm films still invisible — a shallow reach between pools must READ
            // as the same stream (Joseph's "seeping" was riffles under the old
            // cutoff).
            let fade = ((depth - 0.025) / 0.1).clamp(0.0, 1.0);
            // DECOUPLED (opacity kept creeping when hue and alpha shared one
            // curve): alpha is transmission (Beer–Lambert, how much bed shows
            // through), hue is depth (pale shallows -> deep blue), separately.
            let alpha = ((1.0 - (-depth * WATER_ABSORB_PER_M).exp()) * fade).clamp(0.0, 0.80);
            let m = (1.0 - (-depth * 0.5).exp()) * fade;
            positions.push([px(i), surf, pz(j)]);
            let (x, y) = (i as isize, j as isize);
            let nrm = Vec3::new(sv(x - 1, y) - sv(x + 1, y), 2.0 * cell as f32, sv(x, y - 1) - sv(x, y + 1)).normalize();
            normals.push([nrm.x, nrm.y, nrm.z]);
            // Suspended load makes water TURBID: silty brown, green suppressed
            // (Joseph). t=0 clear mountain water, t→1 flood-brown.
            let t = turbidity_of(i, j);
            const SILT: [f32; 3] = [0.52, 0.42, 0.26];
            let mut rgb = [
                WATER_SHALLOW[0] + (WATER_DEEP[0] - WATER_SHALLOW[0]) * m,
                WATER_SHALLOW[1] + (WATER_DEEP[1] - WATER_SHALLOW[1]) * m,
                WATER_SHALLOW[2] + (WATER_DEEP[2] - WATER_SHALLOW[2]) * m,
            ];
            for k in 0..3 {
                rgb[k] += (SILT[k] - rgb[k]) * t;
            }
            colors.push([rgb[0], rgb[1], rgb[2], m.clamp(0.0, 0.95)]);
        }
    }
    // Shoreline (Joseph): do NOT feather the surface down at the water's edge —
    // extend the level plane one ring INTO the hillside at the wet neighbour's
    // surface height and let the terrain's depth test cut it. The intersection
    // line IS the shoreline; the old all-corners-wet rule + bilinear smear was
    // the "meniscus climbing the banks".
    let mut ext = vec![false; w * w];
    for j in 0..w {
        for i in 0..w {
            let k = j * w + i;
            if wet[k] {
                continue;
            }
            let mut best: Option<usize> = None;
            let mut best_y = f32::NEG_INFINITY;
            for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (i as i64 + dx, j as i64 + dy);
                if nx < 0 || ny < 0 || nx >= w as i64 || ny >= w as i64 {
                    continue;
                }
                let nk = ny as usize * w + nx as usize;
                if wet[nk] && positions[nk][1] > best_y {
                    best_y = positions[nk][1];
                    best = Some(nk);
                }
            }
            // BANKS only: extend where the water surface is below this dry
            // vertex's terrain (the plane buries into the hill and the depth
            // test cuts it). At a drop-off lip the donor surface is ABOVE the
            // downhill ground — extending there floats a specular sliver of
            // water over the edge (the bright streaks along banks).
            if let Some(nk) = best {
                if best_y <= positions[k][1] + 0.05 {
                    positions[k][1] = positions[nk][1];
                    colors[k] = colors[nk];
                    normals[k] = normals[nk];
                    ext[k] = true;
                }
            }
        }
    }
    let mut indices: Vec<u32> = Vec::new();
    for j in 0..w - 1 {
        for i in 0..w - 1 {
            let (a, b, c, d) = (j * w + i, j * w + i + 1, (j + 1) * w + i, (j + 1) * w + i + 1);
            let ok = |k: usize| wet[k] || ext[k];
            if (wet[a] || wet[b] || wet[c] || wet[d]) && ok(a) && ok(b) && ok(c) && ok(d) {
                indices.extend_from_slice(&[a as u32, c as u32, b as u32, b as u32, c as u32, d as u32]);
            }
        }
    }
    if indices.is_empty() {
        return None;
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

// --- HUD -----------------------------------------------------------------------------

fn compass(yaw: f32) -> &'static str {
    const L: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let deg = yaw.to_degrees().rem_euclid(360.0);
    L[(((deg + 22.5) / 45.0) as usize) % 8]
}

fn hud_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    view: Res<View>,
    ts: Res<TerrainState>,
    meta: Res<TierMeta>,
    wmeta: Res<WaterMeta>,
    water: Res<WaterRes>,
    eroded: Res<Eroded>,
    diag: Res<bevy::diagnostic::DiagnosticsStore>,
    mut spans: Query<(&HudSlot, &mut TextSpan, &mut TextColor)>,
    mut mq: Query<&mut Text, With<ModeText>>,
    mut legend: Query<&mut Node, With<LegendText>>,
    mut hist: Local<std::collections::HashMap<u16, (String, f64, f64)>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        if let Ok(mut n) = legend.single_mut() {
            n.display = if n.display == Display::None { Display::Flex } else { Display::None };
        }
    }
    // Top-right: mode + weather + (autoshot countdown when scripted).
    if let Ok(mut mt) = mq.single_mut() {
        let wx = match wmeta.0.as_ref().map(|w| w.weather) {
            Some((true, left)) if left.is_infinite() => "   deluge".to_string(),
            Some((true, left)) => format!("   rain {left:.0}s"),
            Some((false, left)) => format!("   clear {left:.0}s"),
            None => String::new(),
        };
        **mt = format!("[T] {}{wx}   [H] legend", view.mode.name());
    }

    // ---- values, one string per slot (fixed widths keep the table aligned) ----
    let mut vals: Vec<String> = vec![String::new(); 28];
    let fps = diag.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()).unwrap_or(0.0);
    vals[0] = format!("{fps:<4.0}");
    vals[1] = format!("{:<7}", format!("{:.0} ms", ts.gen_ms));
    let cell = view.cell_m();
    let span_m = view.w as f64 * cell;
    vals[2] = format!("{:?} L{}", view.face, view.level);
    vals[3] = format!("{:<7}", if cell >= 2.0 { format!("{cell:.0} m") } else { format!("{cell:.1} m") });
    vals[4] = format!("{:<8}", if span_m >= 2000.0 { format!("{:.1} km", span_m / 1000.0) } else { format!("{span_m:.0} m") });
    vals[5] = format!("({:.0}, {:.0})", view.focus.x, view.focus.y);
    vals[6] = format!("{:<2}", compass(view.yaw_target));
    vals[7] = format!("{:<6}", format!("{:.0} deg", view.pitch.to_degrees()));
    vals[8] = format!("{:<7}", format!("{:.0} m", view.zoom));
    vals[9] = format!("{:.1}", view.vert);
    vals[10] = format!("{:<7}", format!("{:.0} m", height_at_focus(&view, &ts)));
    vals[11] = format!("{:<14}", format!("{:.0}..{:.0} m", ts.h_min, ts.h_max));
    // Sim-age of the visible window: probe corners + centre against tier ages.
    let (mut newest, mut oldest, mut prior_seen) = (f32::INFINITY, 0.0f32, false);
    let wm1 = (view.w - 1) as u32;
    for (px, py) in [(0, 0), (wm1, 0), (0, wm1), (wm1, wm1), (wm1 / 2, wm1 / 2)] {
        let c = CellId::from_face_ij(view.face, ts.origin.0 + px, ts.origin.1 + py, ts.built_level.min(25));
        match erosion::tier_at(c, &eroded.0) {
            Some(l) => {
                if let Some((_, at, ..)) = meta.0.iter().find(|(ml, ..)| *ml == l) {
                    let age = at.elapsed().as_secs_f32();
                    newest = newest.min(age);
                    oldest = oldest.max(age);
                }
            }
            None => prior_seen = true,
        }
    }
    vals[12] = if meta.0.is_empty() {
        "-".into()
    } else {
        let n = if newest.is_finite() { format!("{newest:.1}s") } else { "-".into() };
        let o = if prior_seen { "prior(unsimulated)".into() } else { format!("{oldest:.0}s") };
        format!("{n}..{o}")
    };
    for (k, (l, _, e, rate, delta)) in meta.0.iter().take(3).enumerate() {
        let d = if delta.is_finite() { format!(" d{:.0}cm", delta * 100.0) } else { String::new() };
        let chunk = if *rate > 0.0 { format!("L{l} {e}e ~{rate:.0}y/s{d}") } else { format!("L{l} {e}e{d}") };
        vals[13 + k] = format!("{chunk:<29}");
    }
    if let Some(w) = wmeta.0.as_ref() {
        vals[16] = format!("{:<10}", format!("{:.0} ss", w.total_ss));
        vals[17] = format!("{:<11}", format!("~{:.1} s/s", w.rate));
        vals[18] = format!("{:<11}", format!("d {:.1} mm", w.delta_m * 1000.0));
        vals[19] = format!("{:<12}", format!("Fr {:.1}/{:.0}%", w.froude.0, w.froude.1 * 100.0));
        vals[20] = match w.settle {
            Some((target, cap_left)) => format!(
                "FILLING d{:.0}->{:.0}mm cap {:.0}ss",
                w.delta_m * 1000.0,
                target * 1000.0,
                cap_left
            ),
            None if w.filling => "FILLING".into(),
            None => "living".into(),
        };
        // Live conservation gauge (m·cells → per-cell µm is noise; show total
        // as an honest volume number). Near-zero = the physics is keeping its
        // books; growth = something broke conservation at runtime.
        vals[20] = format!("{}   budget {:+.2} m³/m²·cells", vals[20], w.drift);
    } else {
        vals[16] = "off".into();
    }
    // Pawn hydrology (Water/Float modes, when actually in water).
    let pawn = water
        .0
        .as_ref()
        .filter(|_| matches!(view.mode, ViewMode::Water | ViewMode::Float))
        .and_then(|wr| {
            let c = CellId::from_face_ij(view.face, view.focus.x as u32, view.focus.y as u32, view.level);
            let d = wr.depth_m(c)?;
            if d < 0.005 {
                return None;
            }
            let v = wr.speed_m_s(c).unwrap_or(0.0);
            Some((
                d,
                v,
                v / (9.8 * d).sqrt().max(1e-6),
                wr.suspended_m(c).unwrap_or(0.0),
                wr.sed_bed_m(c).unwrap_or(0.0),
                wr.colmation_at(c).unwrap_or(0.0),
            ))
        });
    if let Some((d, v, fr, susp, sand, seal)) = pawn {
        vals[21] = "pawn    ".into();
        vals[22] = format!("{:<10}", format!("d {d:.2} m"));
        vals[23] = format!("{:<11}", format!("v {v:.2} m/s"));
        vals[24] = format!("{:<10}", format!("Fr {fr:.2}"));
        // Sub-mm loads printed honestly ("0 mm" hid real 0.4 mm suspensions).
        // Sediment now runs THROUGH the fill (kill-switch removed 2026-07-03,
        // probe-cleared), so these are live from the first raindrop.
        vals[25] = format!("{:<12}", format!("susp {:.2}mm", susp * 1000.0));
        vals[26] = format!("{:<14}", format!("alluv {:.0}cm", sand * 100.0));
        vals[27] = format!("seal {:.0}%", seal * 100.0);
    }

    // ---- apply to spans, flashing values that changed after being stable ----
    let now = time.elapsed_secs_f64();
    const INK: (f32, f32, f32) = (0.08, 0.09, 0.10);
    const FLASH: (f32, f32, f32) = (0.82, 0.16, 0.04);
    for (slot, mut span, mut color) in spans.iter_mut() {
        let s = &vals[slot.0 as usize];
        let st = hist.entry(slot.0).or_insert_with(|| (String::new(), now, 0.0));
        if &st.0 != s {
            // Flash only when the previous value had HELD ≥1 s — per-frame
            // counters (fps, streaming ss) self-mute; real transitions pop.
            if now - st.1 > 1.0 {
                st.2 = now + 0.8;
            }
            st.1 = now;
            st.0 = s.clone();
            span.0 = s.clone();
        }
        let t = ((st.2 - now) / 0.8).clamp(0.0, 1.0) as f32;
        *color = TextColor(Color::srgb(
            INK.0 + (FLASH.0 - INK.0) * t,
            INK.1 + (FLASH.1 - INK.1) * t,
            INK.2 + (FLASH.2 - INK.2) * t,
        ));
    }
}

fn maybe_screenshot(time: Res<Time>, mut commands: Commands, mut shot: Local<bool>, mut exit: MessageWriter<AppExit>, mut banner: Query<&mut Text, With<AutoshotBanner>>) {
    if std::env::var_os("VIVARIUM_AUTOSHOT").is_none() {
        return;
    }
    let t = time.elapsed_secs();
    let settle: f32 = std::env::var("VIVARIUM_SETTLE").ok().and_then(|s| s.parse().ok()).unwrap_or(2.5);
    if let Ok(mut b) = banner.single_mut() {
        **b = if *shot {
            "AUTOSHOT VERIFICATION   |   shot saved - safe to close or play".to_string()
        } else {
            format!(
                "AUTOSHOT VERIFICATION   |   screenshot in {:.0}s   |   input only moves the camera (reframes the capture)",
                (settle - t).max(0.0)
            )
        };
    }
    if t > settle && !*shot {
        let path = PathBuf::from(std::env::var("VIVARIUM_SHOT").unwrap_or_else(|_| "/tmp/vivarium_worldview_shot.png".into()));
        eprintln!("[worldview] SHOT_PATH={}", path.display());
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        *shot = true;
    }
    if *shot && t > settle + 1.5 {
        exit.write(AppExit::Success);
    }
}
