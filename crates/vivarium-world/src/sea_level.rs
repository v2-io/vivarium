//! Derived sea level — pour the hydrosphere's ocean stock into hypsometry.
//!
//! **Ordinum / DECISIONS path** (`water-world-is-the-promise-not-the-bug`):
//! sea level is **not** a decreed land-maker. It is inverted from (1) the conserved
//! ocean volume and (2) the planet's solid surface hypsometry. When the basins
//! cannot hold the inventory, sea level sits above the highest ground — a total
//! water-world, which **is** the Protogenic `water-covered-surface` promise.
//!
//! Emerged land is a later Abyssal product (crustal freeboard / uplift), never
//! an initial condition. See `examples/sea_level_probe.rs` (measurement that
//! convicted the old `SEA_LEVEL_M = 4000` datum) and
//! `ref/research/early-continents-survey.md` (Flament / Chowdhury targets).

use crate::gen;
use crate::hydrosphere::Hydrosphere;
use crate::planet::Planet;
use crate::sphere::{CubeCoord, Face};
use crate::lithosphere::{self, MANTLE_TP_C};
use crate::mantle_thermal;
use crate::time::Time;

/// Sampling level for the global pour (256² × 6 ≈ 393k cells). Coarse enough
/// to be cheap; fine enough that the inverted level is stable for v0.
pub(crate) const SAMPLE_LEVEL: u8 = 8;

/// Sea level (m above bedrock datum) for this world-seed at the **present-
/// Abyssal** epoch: ocean stock poured into the **live (post-rock-mass-ledger)**
/// tectonic surface at the live mantle temperature `MANTLE_TP_C`. Pure function
/// of seed; the seed-only entry point every existing caller uses.
pub fn derived_sea_level_m(seed: u64) -> f64 {
    derived_sea_level_at_tp(seed, MANTLE_TP_C)
}

/// Sea level at an explicit mantle potential temperature `tp_c` — poured against
/// the **live default surface**, which since the 2026-07-24 adoption *is* the
/// post-erosion surface: the rock-mass ledger (`crate::erosion_return`) has
/// debited the over-standing cratons, rebounded them, and returned the mass as a
/// submarine sediment blanket. As the mantle cools, basins deepen and the
/// waterline drops relative to the rising cratons. Memoized per `(seed, tp_c)`.
/// The *pre-ledger* isostatic waterline is [`derived_sea_level_pre_ledger_at_tp`].
pub fn derived_sea_level_at_tp(seed: u64, tp_c: f64) -> f64 {
    crate::erosion_return::derived_sea_level_after_erosion_at_tp(seed, tp_c)
}

/// Sea level at canonical time `t` — the mantle-thermal nomos supplies `T_p(t)`
/// (`#form-isostasy-column` FE(2), chain head), and the pour re-derives against
/// the (post-ledger) freeboard that temperature earns. The memoized epoch chain
/// that makes emergence run *in time*.
pub fn derived_sea_level_at(seed: u64, t: Time) -> f64 {
    derived_sea_level_at_tp(seed, mantle_thermal::potential_temp_c(t))
}

/// Solid surface used for the pour and for land classification at the present-
/// Abyssal epoch: bathymetry plus the **post-ledger** freeboard (see
/// [`tectonic_surface_at_tp`]).
pub fn tectonic_surface_m(seed: u64, cell: crate::sphere::CellId, nyquist_level: u8) -> f64 {
    tectonic_surface_at_tp(seed, cell, nyquist_level, MANTLE_TP_C)
}

/// The **live default tectonic surface** at mantle temperature `tp_c` — since the
/// 2026-07-24 adoption this is the *post-erosion* surface: bathymetry plus the
/// rebounded freeboard the rock-mass ledger (`crate::erosion_return`) earns
/// (cratons eroded-and-rebounded, basins sediment-loaded). Every downstream
/// reader (pour, land classification, `emerged_land_record`, globe, `gen`,
/// `query`, erosion outlets) reads *this* honest surface. The pre-ledger
/// isostatic surface is [`tectonic_surface_pre_ledger_at_tp`].
pub fn tectonic_surface_at_tp(seed: u64, cell: crate::sphere::CellId, nyquist_level: u8, tp_c: f64) -> f64 {
    crate::erosion_return::tectonic_surface_after_erosion_at_tp(seed, cell, nyquist_level, tp_c)
}

/// The **pre-ledger** tectonic surface — bathymetry (thermally invariant) plus
/// the raw isostatic freeboard the column earns at `tp_c`, *before* the rock-mass
/// ledger. This is what the ledger consumes (to classify subaerial/submarine and
/// measure stand) and what probes compare the trimmed default against. Not the
/// live surface — use [`tectonic_surface_at_tp`] for that.
pub fn tectonic_surface_pre_ledger_at_tp(seed: u64, cell: crate::sphere::CellId, nyquist_level: u8, tp_c: f64) -> f64 {
    gen::bathymetry_m(seed, cell, nyquist_level) + lithosphere::freeboard_m_at_tp(seed, cell, tp_c)
}

/// Process-global **working-set staging** for the pre-ledger pour (one f64 per
/// `(seed, T_p_bits)`) — NOT a memo tier. `#form-store-as-save` FE(6) (decided
/// 2026-07-24, `DECISIONS[memoized-means-store-object]`): the durable memo is the
/// store's epoch-reduction object under a complete key; this RAM map only stages
/// what the store owns for hot per-cell reads within a process. A reader that
/// gets a store Hit [`prime_derived_sea_pre_ledger`]s this cache so the ~393k-cell
/// pour never re-runs in a warmed process; on a cold miss the compute below fills
/// it and the store citizen is written by the caller (query::World::epoch_reduction).
static PRE_LEDGER_SEA_CACHE: std::sync::Mutex<Option<std::collections::BTreeMap<(u64, u64), f64>>> =
    std::sync::Mutex::new(None);

/// Insert a known pre-ledger derived sea into the L1 memo — used when a caller
/// has read the value from the store (or another authority) and wants the pour
/// short-circuited for this process. Idempotent; a later compute would produce
/// the identical f64 (pure function of the keyed inputs).
/// Empty the pre-ledger L1 memo. **Test affordance only**, and it exists for a
/// specific reason worth stating: the round-trip probe
/// (`query::tests::epoch_ladder_is_order_independent`) compares build legs
/// against each other, and this cache is process-global — so a cache keyed too
/// coarsely corrupts *every* leg identically and the comparison goes blind to it.
/// That insensitivity was demonstrated, not theorised (2026-07-24: an
/// under-keyed variant passed the probe). Clearing between legs is what makes
/// them independent samples rather than one sample counted three times.
#[cfg(test)]
pub(crate) fn clear_pre_ledger_cache_for_test() {
    let mut guard = PRE_LEDGER_SEA_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    *guard = None;
}

pub fn prime_derived_sea_pre_ledger(seed: u64, tp_c: f64, sea_m: f64) {
    let mut guard = PRE_LEDGER_SEA_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(std::collections::BTreeMap::new).insert((seed, tp_c.to_bits()), sea_m);
}

/// The **pre-ledger** derived waterline at `tp_c` — the pour against the raw
/// isostatic surface, before the rock-mass ledger. Memoized per `(seed, tp_c)`;
/// the ledger consumes it as an intermediate. The live default is
/// [`derived_sea_level_at_tp`].
pub fn derived_sea_level_pre_ledger_at_tp(seed: u64, tp_c: f64) -> f64 {
    let key = (seed, tp_c.to_bits());
    {
        let guard = PRE_LEDGER_SEA_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(map) = guard.as_ref() {
            if let Some(&s) = map.get(&key) {
                return s;
            }
        }
    }
    let s = pour_ocean_over(SAMPLE_LEVEL, |coord| {
        tectonic_surface_pre_ledger_at_tp(seed, coord.cell(SAMPLE_LEVEL), SAMPLE_LEVEL, tp_c)
    });
    let mut guard = PRE_LEDGER_SEA_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(std::collections::BTreeMap::new).insert(key, s);
    s
}

/// Invert the conserved ocean stock onto an arbitrary solid `surface` sampled at
/// `level` — the shared pour, used by both [`derived_sea_level_pre_ledger_at_tp`]
/// and the rock-mass ledger's post-erosion re-pour
/// (`crate::erosion_return::derived_sea_level_after_erosion_at_tp`) so both use
/// the identical hypsometric inversion (no forked pour).
/// Returns the waterline (m); a total water-world when basins cannot hold the
/// inventory (ordinum promise).
pub(crate) fn pour_ocean_over(level: u8, surface: impl Fn(CubeCoord) -> f64) -> f64 {
    let planet = Planet::EARTH;
    let ocean_km3 = Hydrosphere::of(&planet).ocean_km3;
    let n = 1usize << level;
    let r_km = planet.radius_m / 1000.0;
    let mut heights: Vec<f64> = Vec::with_capacity(6 * n * n);
    let mut areas: Vec<f64> = Vec::with_capacity(6 * n * n);
    let mut total_area = 0.0f64;
    let mut lo = f64::MAX;
    let mut hi = f64::MIN;
    let mut mean_acc = 0.0f64;

    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let h = surface(CubeCoord { face, u, v });
                // Uniform-area approx at this level — good enough for the sea-level
                // *datum*; true spherical excess remains the probe's job.
                let area = (4.0 * std::f64::consts::PI * r_km * r_km) / (6.0 * (n * n) as f64);
                heights.push(h);
                areas.push(area);
                total_area += area;
                lo = lo.min(h);
                hi = hi.max(h);
                mean_acc += h * area;
            }
        }
    }
    let mean_h = mean_acc / total_area;

    let held = |s: f64| -> f64 {
        heights
            .iter()
            .zip(areas.iter())
            .filter(|(h, _)| **h < s)
            .map(|(h, a)| (s - h) * a / 1000.0)
            .sum()
    };
    let capacity = held(hi);
    if ocean_km3 > capacity {
        // Basins cannot hold the inventory: total water-world (ordinum promise).
        return mean_h + ocean_km3 * 1000.0 / total_area;
    }
    let (mut a, mut b) = (lo, hi);
    for _ in 0..64 {
        let m = 0.5 * (a + b);
        if held(m) < ocean_km3 {
            a = m;
        } else {
            b = m;
        }
    }
    0.5 * (a + b)
}

/// Land fraction at the derived sea level (area where tectonic surface > sea).
/// Used by probes and tests; not a world-law input.
pub fn land_fraction_at_derived_sea(seed: u64) -> f64 {
    let sea = derived_sea_level_m(seed);
    let n = 1usize << SAMPLE_LEVEL;
    let mut land = 0usize;
    let mut total = 0usize;
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(SAMPLE_LEVEL);
                let h = tectonic_surface_m(seed, cell, SAMPLE_LEVEL);
                total += 1;
                if h > sea {
                    land += 1;
                }
            }
        }
    }
    land as f64 / total as f64
}

/// Land fraction (area where the tectonic surface stands above an *arbitrary*
/// waterline `sea_m`), sampled at `level`. The derived-sea version above is the
/// live law; this generalized form exists so a probe can pour against a
/// **decreed** datum and show what a non-derived waterline manufactures — the
/// known-bad half of the emerged-land predicate's sensitivity (#norm-probe-
/// sensitivity FE(2)).
pub fn land_fraction_at_sea(seed: u64, sea_m: f64, level: u8) -> f64 {
    let n = 1usize << level;
    let mut land = 0usize;
    let mut total = 0usize;
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(level);
                total += 1;
                if tectonic_surface_m(seed, cell, level) > sea_m {
                    land += 1;
                }
            }
        }
    }
    land as f64 / total as f64
}

// --- Record-style acceptance predicate for `promise[emerged-land]` -----------
//
// The Claimed → Kept instrument (#form-ordinum-governs-flux-web FE(4);
// #norm-probes-before-claims). Building this decides **no** verdict — it is the
// check that *could* convict the promise per cycle. The bands are literature-
// sourced (early-continents survey §6 → `ASSUMPTIONS.md`), Record-style checks on
// *earned* freeboard; they are never baked into the Protogenic prior
// (#form-derived-sea-level FE(7)).
//
// Deep time does not yet run, so "per cycle" collapses to "per seed": the
// predicate is evaluated once on the static Abyssal tectonic surface. The
// era-sharpening clauses ("land fraction >10% by ~3.0 Ga-equivalent"; "modern-
// like freeboard only ~2.5–2.2 Ga") are therefore **not-yet-predicable** and are
// named as such rather than faked — only the time-free clauses convict here.

/// Emerged-land fraction band for an early-Earth world (Hadean → mid-Archean,
/// pre-~2.5 Ga): Flament 2008 MC-median floor ~1.8%, Korenaga ceiling ~20%
/// (survey §6; well below modern 29%). A world outside this band is either a
/// drowned water-world (below floor) or a decreed/over-emerged surface (above).
pub const EMERGED_LAND_FRACTION_BAND: (f64, f64) = (0.018, 0.20);

/// Subaerial-relief flag ceiling (m). Early land stood ~1–1.5 km; >2 km
/// topography is unlikely while the crust is weak under a hot geotherm (survey
/// §6; #form-isostasy-column FE(6)). **Soft clause:** measured on the *pre-
/// erosion* tectonic surface, where the strength limit and erosion mass-return
/// that would enforce it are declared-open (#form-isostasy-column FE(8)). A
/// breach is an amber **flag** — an honest signal that isostatic freeboard alone
/// over-stands the cratons — not a hard conviction of the promise.
pub const SUBAERIAL_RELIEF_FLAG_M: f64 = 2000.0;

/// One evaluation of the emerged-land Record on a world-seed's Abyssal surface.
#[derive(Clone, Copy, Debug)]
pub struct EmergedLandRecord {
    pub sea_level_m: f64,
    pub land_fraction: f64,
    /// Highest stand above the derived sea (0 if a total water-world).
    pub max_subaerial_m: f64,
    pub level: u8,
}

/// A predicate clause's outcome. `Flag` is a soft amber (measured; a breach
/// informs but does not convict). `NotPredicable` is a clause the literature
/// predicate carries that nothing in v1 can yet convict (the temporal clauses).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Clause {
    Pass,
    Fail,
    Flag,
    NotPredicable,
}

impl Clause {
    pub fn label(self) -> &'static str {
        match self {
            Clause::Pass => "PASS",
            Clause::Fail => "FAIL",
            Clause::Flag => "flag",
            Clause::NotPredicable => "n/a-v1",
        }
    }
}

/// The Record verdict: one clause per literature check.
#[derive(Clone, Copy, Debug)]
pub struct EmergedLandVerdict {
    pub record: EmergedLandRecord,
    /// Land rises above sea level at all (non-volcanic by construction — the
    /// craton field is differentiation, not volcanism). Hard.
    pub land_rises: Clause,
    /// Land fraction inside the early-Earth band. Hard.
    pub fraction_in_band: Clause,
    /// Peak subaerial stand at/under the relief ceiling. Soft (pre-erosion).
    pub relief_bounded: Clause,
    /// Era-sharpening timing clauses — no deep time in v1. NotPredicable.
    pub timing: Clause,
}

impl EmergedLandVerdict {
    /// The promise's *hard* time-free clauses all convict green (relief may
    /// flag amber; timing is n/a in v1). This is **not** a Kept verdict — Kept
    /// is a separate later adjudication and requires the temporal clauses too.
    pub fn hard_clauses_pass(&self) -> bool {
        self.land_rises == Clause::Pass && self.fraction_in_band == Clause::Pass
    }
}

/// Measure the emerged-land Record for `seed` at `level` (the derived sea comes
/// from the level-8 pour regardless; land fraction and relief are sampled here).
pub fn emerged_land_record_at(seed: u64, level: u8) -> EmergedLandRecord {
    let sea = derived_sea_level_m(seed);
    let n = 1usize << level;
    let mut land = 0usize;
    let mut total = 0usize;
    let mut max_sub = 0.0f64;
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(level);
                let h = tectonic_surface_m(seed, cell, level);
                total += 1;
                if h > sea {
                    land += 1;
                    max_sub = max_sub.max(h - sea);
                }
            }
        }
    }
    EmergedLandRecord {
        sea_level_m: sea,
        land_fraction: land as f64 / total as f64,
        max_subaerial_m: max_sub,
        level,
    }
}

/// The Record at the live pour grain (level 8).
pub fn emerged_land_record(seed: u64) -> EmergedLandRecord {
    emerged_land_record_at(seed, SAMPLE_LEVEL)
}

/// The emerged-land Record at an explicit mantle temperature `tp_c` — the same
/// measurement, taken along the cooling chain. Walking a sequence of `tp_c`
/// (hot → cool) gives the emergence trajectory the ordinum narrates.
pub fn emerged_land_record_at_tp(seed: u64, level: u8, tp_c: f64) -> EmergedLandRecord {
    let sea = derived_sea_level_at_tp(seed, tp_c);
    let n = 1usize << level;
    let mut land = 0usize;
    let mut total = 0usize;
    let mut max_sub = 0.0f64;
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(level);
                let h = tectonic_surface_at_tp(seed, cell, level, tp_c);
                total += 1;
                if h > sea {
                    land += 1;
                    max_sub = max_sub.max(h - sea);
                }
            }
        }
    }
    EmergedLandRecord {
        sea_level_m: sea,
        land_fraction: land as f64 / total as f64,
        max_subaerial_m: max_sub,
        level,
    }
}

/// The emerged-land Record on the **pre-ledger** isostatic surface at `tp_c` —
/// the surface *before* the rock-mass ledger trims it. Used to convict and
/// display the ledger's amber-relief trim (probes compare this against the live
/// default `emerged_land_record_at_tp`). Not the live world.
pub fn emerged_land_record_pre_ledger_at_tp(seed: u64, level: u8, tp_c: f64) -> EmergedLandRecord {
    let sea = derived_sea_level_pre_ledger_at_tp(seed, tp_c);
    let n = 1usize << level;
    let mut land = 0usize;
    let mut total = 0usize;
    let mut max_sub = 0.0f64;
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(level);
                let h = tectonic_surface_pre_ledger_at_tp(seed, cell, level, tp_c);
                total += 1;
                if h > sea {
                    land += 1;
                    max_sub = max_sub.max(h - sea);
                }
            }
        }
    }
    EmergedLandRecord {
        sea_level_m: sea,
        land_fraction: land as f64 / total as f64,
        max_subaerial_m: max_sub,
        level,
    }
}

/// Adjudicate a Record against the literature bands — the predicate itself.
pub fn emerged_land_verdict(rec: EmergedLandRecord) -> EmergedLandVerdict {
    let (lo, hi) = EMERGED_LAND_FRACTION_BAND;
    EmergedLandVerdict {
        record: rec,
        land_rises: if rec.land_fraction > 0.0 { Clause::Pass } else { Clause::Fail },
        fraction_in_band: if (lo..=hi).contains(&rec.land_fraction) { Clause::Pass } else { Clause::Fail },
        relief_bounded: if rec.max_subaerial_m <= SUBAERIAL_RELIEF_FLAG_M { Clause::Pass } else { Clause::Flag },
        // A single Record has no time axis, so it cannot convict *timing* — that
        // is a property of the trajectory, not of one snapshot. The temporal
        // clauses now live in `emerged_land_timing_verdict` (the cooling chain).
        timing: Clause::NotPredicable,
    }
}

// --- Temporal (era-sharpening) adjudication over the cooling chain -----------
//
// The clauses the static Record named `NotPredicable` because "per cycle"
// collapsed to per-seed — there was no time axis. The mantle-thermal cooling
// trajectory (`mantle_thermal::abyssal_epochs`, `#form-isostasy-column` FE(2)/(7))
// now IS a real time axis, so a *sequence* of Records (hot → cool) can convict
// the temporal shape of emergence. What it can and cannot earn is set by the
// honesty tier of the rate law, which is declared **crude (physics Low)**:
//
//  - **Calibration-FREE** clauses are convictable — they depend only on the
//    monotone *direction* of cooling (declared) and the ordering of epochs, not
//    on any epoch→Ga mapping: monotone emergence, a submerged start, and the
//    trajectory entering and staying inside the early-Earth band.
//  - **Ga-ABSOLUTE** literature clauses ("land ≳10% by ~3.0 Ga-equivalent";
//    "modern-like freeboard only ~2.5–2.2 Ga") stay `NotPredicable`: the epoch→Ga
//    mapping is a *declared* order-of-magnitude curve (`PRESENT_ABYSSAL_GA`, τ),
//    not fitted, so testing against an absolute-Ga target would assert a
//    calibration the rate law does not earn — even though the crude curve happens
//    to pass near the 10%-by-3.0-Ga target (an encouraging coincidence, recorded
//    in the probe, not convicted).

/// Trajectory-level verdict on the temporal (era-sharpening) clauses, adjudicated
/// over a cooling sequence of Records ordered **hot → cool**.
#[derive(Clone, Copy, Debug)]
pub struct EmergedLandTimingVerdict {
    /// Land fraction is monotone non-decreasing from hot to cool — a cooling
    /// world does not *un*-emerge land. Calibration-free. `NotPredicable` if
    /// given fewer than two epochs (no time axis to test).
    pub emergence_monotone: Clause,
    /// The hottest (early-Abyssal) epoch stands below the band floor — the
    /// water-world start of the ordinum ladder. Calibration-free.
    pub starts_submerged: Clause,
    /// The trajectory reaches the early-Earth band and stays inside it through
    /// the coldest modeled epoch — band membership is monotone non-regressing
    /// and never overshoots the ceiling within the Abyssal window. Calibration-
    /// free. (This is the nucleation-growth finding: saturating craton growth
    /// bounds the cold end; the retired fBm world overshot >20% and would fail.)
    pub enters_and_stays_in_band: Clause,
    /// The literature's Ga-absolute era targets. `NotPredicable` in v1: the
    /// rate law is declared crude, so an absolute-Ga fraction target is a
    /// calibration it cannot earn (see module note above).
    pub ga_calibrated_era: Clause,
}

impl EmergedLandTimingVerdict {
    /// The calibration-free temporal clauses all convict green. **Not** a Kept
    /// verdict — Kept is a separate adjudication, and the Ga-absolute clause
    /// remains honestly NotPredicable.
    pub fn calibration_free_clauses_pass(&self) -> bool {
        self.emergence_monotone == Clause::Pass
            && self.starts_submerged == Clause::Pass
            && self.enters_and_stays_in_band == Clause::Pass
    }
}

/// Adjudicate the temporal clauses over a cooling sequence of Records
/// (**hot → cool**). Pure over the slice, so a probe can feed it synthetic
/// sequences as known-bads (`#norm-probe-sensitivity`).
pub fn emerged_land_timing_verdict(records: &[EmergedLandRecord]) -> EmergedLandTimingVerdict {
    let (lo, hi) = EMERGED_LAND_FRACTION_BAND;

    let emergence_monotone = if records.len() < 2 {
        Clause::NotPredicable
    } else if records.windows(2).all(|w| w[1].land_fraction + 1e-9 >= w[0].land_fraction) {
        Clause::Pass
    } else {
        Clause::Fail
    };

    let starts_submerged = match records.first() {
        Some(r) if r.land_fraction < lo => Clause::Pass,
        Some(_) => Clause::Fail,
        None => Clause::NotPredicable,
    };

    // Enters the band (some epoch in [lo,hi]), never overshoots the ceiling
    // (no epoch above hi), and does not regress out the bottom once emerged
    // (band membership monotone non-decreasing in ordinal band index).
    let enters_and_stays_in_band = if records.len() < 2 {
        Clause::NotPredicable
    } else {
        let reaches = records.iter().any(|r| (lo..=hi).contains(&r.land_fraction));
        let no_overshoot = records.iter().all(|r| r.land_fraction <= hi);
        // ordinal band: 0 drowned (<lo), 1 in-band, (2 would be >hi, excluded above)
        let band = |f: f64| if f < lo { 0u8 } else { 1u8 };
        let no_regress = records.windows(2).all(|w| band(w[1].land_fraction) >= band(w[0].land_fraction));
        if reaches && no_overshoot && no_regress { Clause::Pass } else { Clause::Fail }
    };

    EmergedLandTimingVerdict {
        emergence_monotone,
        starts_submerged,
        enters_and_stays_in_band,
        ga_calibrated_era: Clause::NotPredicable,
    }
}

/// Build the cooling sequence for `seed` along `mantle_thermal::abyssal_epochs`
/// (reading the live post-ledger chain) and adjudicate its temporal clauses.
/// Records are recomputed from the (store-backed) epoch surfaces — this verdict
/// is an instrument, not world-state, so it memoizes nothing itself
/// (`#form-store-as-save` FE(6): recompute is its representation).
pub fn emerged_land_timing_verdict_for(seed: u64, level: u8) -> EmergedLandTimingVerdict {
    let records: Vec<EmergedLandRecord> = mantle_thermal::abyssal_epochs()
        .into_iter()
        .map(|t| emerged_land_record_at_tp(seed, level, mantle_thermal::potential_temp_c(t)))
        .collect();
    emerged_land_timing_verdict(&records)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthetic Record carrying only the fields the timing verdict reads.
    fn rec(frac: f64) -> EmergedLandRecord {
        EmergedLandRecord { sea_level_m: 0.0, land_fraction: frac, max_subaerial_m: 0.0, level: 6 }
    }

    #[test]
    fn timing_verdict_convicts_the_live_cooling_chain() {
        // The temporal clauses, now predicable because the cooling trajectory
        // is a real time axis. Measured (mantle_cooling_probe, level 7): every
        // seed emerges monotonically from a water-world (0.0% at 3.6 Ga) through
        // the in-band present into an in-band cold end (14–19%, under the 20%
        // ceiling — the nucleation-growth bound). Coarse (level 5) here.
        for seed in [0u64, 1, 7] {
            let v = emerged_land_timing_verdict_for(seed, 5);
            assert_eq!(v.emergence_monotone, Clause::Pass, "seed {seed}: emergence must be monotone in time");
            assert_eq!(v.starts_submerged, Clause::Pass, "seed {seed}: hot early-Abyssal is a water-world");
            assert_eq!(
                v.enters_and_stays_in_band, Clause::Pass,
                "seed {seed}: trajectory must enter the band and stay in it (no >20% overshoot)"
            );
            assert!(v.calibration_free_clauses_pass(), "seed {seed}: calibration-free clauses convict — but NOT Kept");
            // The Ga-absolute clause is honestly refused, not faked green: the
            // crude rate law cannot earn an absolute-Ga fraction target.
            assert_eq!(v.ga_calibrated_era, Clause::NotPredicable, "Ga-absolute era targets stay NotPredicable (crude rate law)");
        }
    }

    #[test]
    fn timing_verdict_catches_temporal_known_bads() {
        // #norm-probe-sensitivity: the pure-over-slice verdict must FAIL on
        // sequences that violate each convictable clause — else its green is
        // insensitive. Sequences are hot → cool.
        //
        // (1) NON-MONOTONE — land emerges then partly drowns again: a cooling
        //     world that un-emerges land. emergence_monotone FAILS.
        let dip = [rec(0.0), rec(0.05), rec(0.03), rec(0.10)];
        assert_eq!(emerged_land_timing_verdict(&dip).emergence_monotone, Clause::Fail);
        //
        // (2) OVERSHOOT — the retired fBm world's shape: monotone but the cold
        //     end blows past the 20% ceiling (measured 27% pre-nucleation).
        //     enters_and_stays_in_band FAILS; emergence_monotone still passes.
        let overshoot = [rec(0.0), rec(0.05), rec(0.15), rec(0.27)];
        let ov = emerged_land_timing_verdict(&overshoot);
        assert_eq!(ov.emergence_monotone, Clause::Pass);
        assert_eq!(ov.enters_and_stays_in_band, Clause::Fail, "a >20% cold end must fail the stay-in-band clause");
        //
        // (3) NEVER EMERGES — a perpetual water-world never reaches the band.
        //     starts_submerged passes, but enters_and_stays FAILS (never in band).
        let drowned = [rec(0.0), rec(0.001), rec(0.005), rec(0.01)];
        let dr = emerged_land_timing_verdict(&drowned);
        assert_eq!(dr.starts_submerged, Clause::Pass);
        assert_eq!(dr.enters_and_stays_in_band, Clause::Fail, "a world that never reaches the band must fail");
        //
        // (4) HOT START ALREADY EMERGED — a decreed/over-emerged world that
        //     stands land at the hottest epoch fails the water-world start.
        let hot_land = [rec(0.10), rec(0.12), rec(0.14), rec(0.16)];
        assert_eq!(emerged_land_timing_verdict(&hot_land).starts_submerged, Clause::Fail);
        //
        // A single record cannot convict a trajectory clause — no time axis.
        assert_eq!(emerged_land_timing_verdict(&[rec(0.05)]).emergence_monotone, Clause::NotPredicable);
    }

    #[test]
    fn bathymetry_alone_is_a_water_world_at_derived_sea() {
        // Without freeboard the pour matches the sea_level_probe finding: ocean
        // does not fit → sea above max peak → 0% land (Protogenic promise).
        // Freeboard is the Abyssal product; this test samples bathymetry only.
        let seed = 0u64;
        let planet = Planet::EARTH;
        let ocean_km3 = Hydrosphere::of(&planet).ocean_km3;
        let n = 1usize << 6; // coarser, still diagnostic
        let mut hi = f64::MIN;
        let mut mean = 0.0;
        let mut count = 0usize;
        for f in 0..6u8 {
            let face = Face::from_index(f);
            for j in 0..n {
                for i in 0..n {
                    let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let cell = CubeCoord { face, u, v }.cell(6);
                    let h = gen::bathymetry_m(seed, cell, 6);
                    hi = hi.max(h);
                    mean += h;
                    count += 1;
                }
            }
        }
        mean /= count as f64;
        // Capacity upper bound: fill everything to hi.
        let r_km = planet.radius_m / 1000.0;
        let area = 4.0 * std::f64::consts::PI * r_km * r_km;
        let capacity_upper = (hi - mean).max(0.0) * area / 1000.0; // rough
        // With the old decreed datum the land fraction was ~30%+; bathymetry must
        // not manufacture that by itself — freeboard is what earns land.
        let _ = (ocean_km3, capacity_upper);
        // Direct: no cell of pure bathymetry should sit above a full-flood sea
        // built from mean + ocean_m_we when relief is only a few km.
        let flood = mean + Hydrosphere::of(&planet).ocean_m_we(&planet);
        let mut above = 0usize;
        for f in 0..6u8 {
            let face = Face::from_index(f);
            for j in (0..n).step_by(4) {
                for i in (0..n).step_by(4) {
                    let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let cell = CubeCoord { face, u, v }.cell(6);
                    if gen::bathymetry_m(seed, cell, 6) > flood {
                        above += 1;
                    }
                }
            }
        }
        assert_eq!(above, 0, "bathymetry must not poke through a full-flood sea (forbidden initial land)");
    }

    #[test]
    fn freeboard_earns_some_emerged_land() {
        // Abyssal freeboard (zero-mean) lifts some crust above derived sea level.
        // Target band from early-continents survey: a few percent, not modern 29%.
        let frac = land_fraction_at_derived_sea(0);
        assert!(
            frac > 0.002 && frac < 0.30,
            "emerged land fraction {frac:.4} should be small Abyssal (not zero, not modern ~0.29)"
        );
    }

    #[test]
    fn derived_sea_is_deterministic() {
        assert_eq!(derived_sea_level_m(7), derived_sea_level_m(7));
        assert_ne!(derived_sea_level_m(0), derived_sea_level_m(1));
    }

    #[test]
    fn emerged_land_predicate_hard_clauses_convict_the_live_world() {
        // The Record-style predicate's time-free clauses on the live derived
        // pour: land rises, and land fraction sits inside the early-Earth band
        // (survey §6). Measured 2026-07-24 at level 8: seeds 0/1/7 gave
        // 4.5% / 2.1% / 4.8% — near the Flament floor, comfortably in-band.
        // Coarser here (level 6) for test speed; the finding is the same class.
        for seed in [0u64, 1, 7] {
            let v = emerged_land_verdict(emerged_land_record_at(seed, 6));
            assert_eq!(v.land_rises, Clause::Pass, "seed {seed}: land must rise (not a water-world)");
            assert_eq!(
                v.fraction_in_band, Clause::Pass,
                "seed {seed}: land fraction {:.3} outside band {:?}",
                v.record.land_fraction, EMERGED_LAND_FRACTION_BAND
            );
            assert!(v.hard_clauses_pass(), "seed {seed}: hard time-free clauses convict — but this is NOT Kept");
            // The relief clause is honestly soft: it may flag amber on the pre-
            // erosion surface (seeds 0/7 measured ~2.4 km > the 2 km ceiling).
            // A flag is a finding, never a hard fail — so the test does not gate on it.
            assert!(
                matches!(v.relief_bounded, Clause::Pass | Clause::Flag),
                "relief clause is soft: pass or amber flag, never a hard fail"
            );
            // The temporal clauses are named not-yet-predicable, never faked green.
            assert_eq!(v.timing, Clause::NotPredicable, "no deep time in v1 — timing cannot be convicted");
        }
    }

    #[test]
    fn cooling_grows_land_from_a_water_world_through_the_present() {
        // The mantle-thermal emergence conviction (`#form-isostasy-column`
        // FE(7); mantle_cooling_probe): walking the cooling chain, land fraction
        // is MONOTONE non-decreasing (a cooling world does not un-emerge land),
        // it starts a near-water-world at the hot early-Abyssal end, and the
        // present-Abyssal epoch equals the live seed-only world exactly. Coarse
        // (level 5) here for speed; the finding is the probe's (level 7).
        use crate::mantle_thermal::{abyssal_epochs, potential_temp_c, present_abyssal};
        const LVL: u8 = 5;
        for seed in [0u64, 1, 7] {
            let epochs = abyssal_epochs();
            let mut prev = -1.0f64;
            for &t in &epochs {
                let tp = potential_temp_c(t);
                let frac = emerged_land_record_at_tp(seed, LVL, tp).land_fraction;
                assert!(
                    frac + 1e-9 >= prev,
                    "seed {seed}: land fraction fell along the cooling chain ({prev:.4} -> {frac:.4}) — emergence must be monotone"
                );
                prev = frac;
            }
            // Hot early-Abyssal end (T_p ~1641) stands almost no land — the
            // ordinum's water-world start (thick, light oceanic crust).
            let hot = potential_temp_c(*epochs.first().unwrap());
            assert!(
                emerged_land_record_at_tp(seed, LVL, hot).land_fraction < 0.02,
                "seed {seed}: hot early-Abyssal must be near a water-world"
            );
            // The present-Abyssal epoch IS the live static world: the cooling
            // law passes through MANTLE_TP_C there (the coherence pin), so the
            // seed-only pour and the epoch-chain pour agree to float precision.
            let present_tp = potential_temp_c(present_abyssal());
            assert_eq!(
                derived_sea_level_at_tp(seed, present_tp),
                derived_sea_level_m(seed),
                "seed {seed}: present-Abyssal epoch must equal the live seed-only world"
            );
        }
    }

    #[test]
    fn emerged_land_predicate_catches_both_known_bads() {
        // #norm-probe-sensitivity FE(2): a probe that cannot fail on a known-bad
        // is not yet a probe for that fault class. Two bracket the live band.
        //
        // FLOOR — bathymetry alone (freeboard stripped) is the retired water-world:
        // 0% land, the predicate's land_rises clause FAILS.
        let sea = derived_sea_level_m(0);
        let bathy_only_frac = {
            let n = 1usize << 6;
            let (mut land, mut tot) = (0usize, 0usize);
            for f in 0..6u8 {
                let face = Face::from_index(f);
                for j in 0..n {
                    for i in 0..n {
                        let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                        let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                        let cell = CubeCoord { face, u, v }.cell(6);
                        tot += 1;
                        if gen::bathymetry_m(0, cell, 6) > sea {
                            land += 1;
                        }
                    }
                }
            }
            land as f64 / tot as f64
        };
        let floor_bad = EmergedLandRecord { sea_level_m: sea, land_fraction: bathy_only_frac, max_subaerial_m: 0.0, level: 6 };
        assert_eq!(emerged_land_verdict(floor_bad).land_rises, Clause::Fail, "water-world must fail land_rises");
        assert_eq!(emerged_land_verdict(floor_bad).fraction_in_band, Clause::Fail, "0% is below the band floor");

        // CEILING — a DECREED low sea level (old SEA_LEVEL_M-style datum)
        // manufactures forbidden land. Measured on the nucleation-growth field:
        // seed 1 @ 2000 m → ~63% (>>29% modern). The fraction_in_band clause
        // FAILS above the 20% ceiling. (The decreed datum is an instrument
        // parameter recalibrated to the field: the pre-nucleation fBm world used
        // 3000 m → 39.7%; the nucleation-growth surface sits higher, so a lower
        // decreed sea is the equivalent over-emergence known-bad.)
        let decreed_frac = land_fraction_at_sea(1, 2000.0, 6);
        assert!(decreed_frac > EMERGED_LAND_FRACTION_BAND.1, "decreed datum manufactures {decreed_frac:.3} land");
        let ceil_bad = EmergedLandRecord { sea_level_m: 2000.0, land_fraction: decreed_frac, max_subaerial_m: 4000.0, level: 6 };
        let cv = emerged_land_verdict(ceil_bad);
        assert_eq!(cv.fraction_in_band, Clause::Fail, "over-emerged surface must fail the band");
        assert_eq!(cv.relief_bounded, Clause::Flag, "and its 4 km stand flags the relief ceiling");

        // Sensitivity stated: the fraction clause discriminates deviations
        // outside [1.8%, 20%] (caught at 0% and ~40% here — well inside its
        // resolution); the relief clause discriminates stands past 2000 m
        // (live peaks 1.7–2.4 km sit right at that edge, so it resolves the
        // ~hundreds-of-metres overshoot, not merely gross violations).
    }
}
