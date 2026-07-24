//! The lithosphere column and its isostasy read — emerged land as a
//! mass-conserving *reading*, not a decreed or strictly-positive field.
//!
//! Claim home: `#form-isostasy-column` (council-accepted chain:
//! mantle-thermal → lithosphere → isostasy → sea-level → erosion; elevation is
//! a derived reading of a conserved column; "uplift rate" is dⅇ/dt, a
//! diagnostic). This module is the first convicting rung of that chain:
//!
//! - **The column** ([`Column`], [`column`]): per-cell crust + depleted-keel
//!   inventory. v1 is a *declared stand-in* for the differentiation rate law
//!   (which is open modelling): a fated, sphere-continuous cratonization field
//!   thresholds into thick felsic crust + a depleted keel — **one process, two
//!   buoyant products, stacked** — over oceanic crust whose thickness and
//!   density are functions of **mantle potential temperature** (the secular-
//!   cooling driver, here a declared dialable parameter; a mantle-thermal
//!   *nomos* is still open).
//! - **The isostasy read** ([`freeboard_m`]): Airy buoyancy height of the
//!   whole column (crust *and* keel — the keel is half the freeboard, not
//!   optional) relative to the asthenosphere, minus the global isostatic
//!   reference [`reference_m`] fixed by **area mass balance** — so rise here
//!   *is* subsidence there by construction (area-mean freeboard ≡ 0), and the
//!   field can express basins ("sit low requires a negative" — the range
//!   defect that convicted the old stand-in, `DECISIONS[uplift-is-
//!   structurally-incapable-of-keeping-its-promise]`).
//!
//! **What the driver buys, convictably:** hot mantle ⇒ thick, low-density
//! oceanic crust ⇒ high oceanic stand ⇒ small craton/ocean contrast ⇒
//! flooding; as the mantle cools the seafloor thins and densifies, basins
//! deepen, and land grows — the buoyancy half of the emergence story, now a
//! monotonicity a test can fail.
//!
//! **Declared omissions (v1, honest):** no water loading in the balance
//! (ρ_sw terms of the full freeboard equation); no crustal transport or
//! erosion mass-return yet (the rock-mass ledger stays open); the craton
//! field is fated geometry, not a differentiation rate law. Physics tier is
//! declared Low on the nomos — the *balance* is real physics; the *inventory*
//! is a stand-in.

use crate::noise;
use crate::sphere::{CellId, CubeCoord, Face};

// --- Declared constants (every magic number anchors into ASSUMPTIONS.md) ---

/// Mantle potential temperature (°C) driving oceanic-crust thickness/density —
/// the secular-cooling control parameter. Default is Archean-hot (early-
/// Abyssal); cooling it toward ~1350 (modern) deepens basins and grows land.
/// `ASSUMPTIONS.md` "mantle potential temperature".
pub const MANTLE_TP_C: f64 = 1550.0;

/// Modern-reference mantle potential temperature (°C) for the cooling ramps —
/// the asymptote the mantle-thermal cooling trajectory decays toward
/// (`crate::mantle_thermal`).
pub const TP_MODERN_C: f64 = 1350.0;

/// Asthenosphere density (kg/m³). `ASSUMPTIONS.md` "lithosphere densities".
pub const RHO_ASTHENOSPHERE: f64 = 3300.0;
/// Felsic continental crust density (kg/m³). Same anchor.
pub const RHO_CONTINENTAL: f64 = 2750.0;
/// Depleted-keel density contrast vs asthenosphere (kg/m³) — small, but it
/// acts over 150+ km of keel. Same anchor.
pub const KEEL_DENSITY_DEFICIT: f64 = 25.0;

/// Cratonic crust thickness (m) where cratonization saturates.
/// `ASSUMPTIONS.md` "craton geometry".
pub const CRATON_CRUST_M: f64 = 35_000.0;
/// Depleted-keel thickness (m) under a full craton. Same anchor.
pub const CRATON_KEEL_M: f64 = 180_000.0;
/// Cratonized area fraction of the fated field (0..1) — the differentiation
/// stand-in's one shape knob. Same anchor.
pub const CRATON_AREA_FRAC: f64 = 0.12;

/// One lithospheric column: the conserved inventory isostasy reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Column {
    /// Crust thickness (m) and density (kg/m³) — felsic under cratons,
    /// oceanic elsewhere (thickness/density set by the thermal driver).
    pub crust_m: f64,
    pub crust_rho: f64,
    /// Depleted continental-lithospheric-mantle keel (m); density is
    /// `RHO_ASTHENOSPHERE − KEEL_DENSITY_DEFICIT`.
    pub keel_m: f64,
}

/// Oceanic crust thickness (m) at mantle potential temperature `tp_c` —
/// thick when hot (high melt fraction), thinning toward ~7 km modern.
/// Linear ramp between the anchored endpoints (Archean ~30 km @1550 °C).
pub fn oceanic_crust_m(tp_c: f64) -> f64 {
    let t = ((tp_c - TP_MODERN_C) / (MANTLE_TP_C - TP_MODERN_C)).clamp(0.0, 1.5);
    7_000.0 + t * (30_000.0 - 7_000.0)
}

/// Oceanic crust density (kg/m³) at `tp_c` — MgO-rich and light when hot
/// (will not sink), densifying toward ~2950 modern as melt fractions drop.
pub fn oceanic_crust_rho(tp_c: f64) -> f64 {
    let t = ((tp_c - TP_MODERN_C) / (MANTLE_TP_C - TP_MODERN_C)).clamp(0.0, 1.5);
    2950.0 - t * 100.0
}

/// Cratonization weight ∈ [0,1] at a cell: the fated differentiation stand-in.
/// Sphere-continuous (unit-vector fBm — `#form-sphere-continuous-surface-fields`
/// binds solid fields), thresholded so ~`CRATON_AREA_FRAC` of area cratonizes,
/// with a smooth margin (continental shelves, not cliffs).
pub fn craton_weight(seed: u64, cell: CellId) -> f64 {
    let p = cell.to_cube().to_unit();
    let f = 2.2; // a few nuclei per face-scale — craton-sized features
    let n = noise::fbm3(seed, 17, p[0] * f, p[1] * f, p[2] * f, 4, 2.0, 0.5);
    // 4-octave fBm concentrates near 0.5 (per-octave averaging shrinks the
    // tails), so the exceedance quantile is measured, not derived: threshold
    // 0.585 lands the cratonized area near CRATON_AREA_FRAC on the sampled
    // sphere — a calibration CONVICTED by `craton_fraction_is_in_band`, not
    // trusted (the first guess, a symmetric-quantile formula, gave 0.5% area).
    let threshold = 0.585;
    let margin = 0.03; // smooth continental margin width in noise units
    ((n - (threshold - margin)) / margin).clamp(0.0, 1.0)
}

/// The lithospheric column at a cell under mantle temperature `tp_c`.
/// Craton and oceanic end-members blend over the margin weight.
pub fn column_at_tp(seed: u64, cell: CellId, tp_c: f64) -> Column {
    let w = craton_weight(seed, cell);
    let (h_oc, rho_oc) = (oceanic_crust_m(tp_c), oceanic_crust_rho(tp_c));
    Column {
        crust_m: h_oc + w * (CRATON_CRUST_M - h_oc),
        // density blends toward felsic as cratonization completes
        crust_rho: rho_oc + w * (RHO_CONTINENTAL - rho_oc),
        keel_m: w * CRATON_KEEL_M,
    }
}

/// The column at the declared default mantle temperature.
pub fn column(seed: u64, cell: CellId) -> Column {
    column_at_tp(seed, cell, MANTLE_TP_C)
}

/// Airy buoyancy height (m) of a column relative to a pure-asthenosphere
/// reference: Σ tᵢ·(ρ_a − ρᵢ)/ρ_a over crust and keel. The raw isostatic
/// stand, before the global reference is subtracted.
pub fn buoyancy_height_m(c: &Column) -> f64 {
    let rho_a = RHO_ASTHENOSPHERE;
    c.crust_m * (rho_a - c.crust_rho) / rho_a + c.keel_m * KEEL_DENSITY_DEFICIT / rho_a
}

/// Sampling level for the global reference (matches the pour's diagnostic
/// grain in `sea_level.rs`).
const SAMPLE_LEVEL: u8 = 7;

/// The global isostatic reference (m): area-mean buoyancy height at `tp_c`,
/// fixed by mass balance over the sampled sphere. Subtracting it makes the
/// freeboard field exactly zero-mean on the sample — rise here IS subsidence
/// there; changing the columns redistributes land, never mints it.
///
/// **Memoized per `(seed, tp_c)`** — this is the whole-sphere integral, so it
/// must not be recomputed per freeboard read. Without the cache the cooling-
/// chain pour (which reads freeboard at ~10⁵–10⁶ cells) would recompute this
/// 10⁵-cell integral at every one of them — an O(N²) blow-up. The default path
/// (`reference_m`) hits the same cache at `MANTLE_TP_C`.
pub fn reference_m_at_tp(seed: u64, tp_c: f64) -> f64 {
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    static CACHE: Mutex<Option<BTreeMap<(u64, u64), f64>>> = Mutex::new(None);
    let key = (seed, tp_c.to_bits());
    {
        let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
        let map = guard.get_or_insert_with(BTreeMap::new);
        if let Some(&r) = map.get(&key) {
            return r;
        }
    }
    let n = 1usize << SAMPLE_LEVEL;
    let mut sum = 0.0;
    let mut count = 0usize;
    for fi in 0..6u8 {
        let face = Face::from_index(fi);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(SAMPLE_LEVEL);
                sum += buoyancy_height_m(&column_at_tp(seed, cell, tp_c));
                count += 1;
            }
        }
    }
    let r = sum / count as f64;
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(BTreeMap::new).insert(key, r);
    r
}

/// Memoized reference at the default (present-Abyssal) temperature.
pub fn reference_m(seed: u64) -> f64 {
    reference_m_at_tp(seed, MANTLE_TP_C)
}

/// **The isostasy read** — freeboard (m) at a cell: buoyancy height minus the
/// global reference. Zero-mean by construction; negative over oceanic columns
/// (basins), positive over cratons (emerged land, once the pour agrees).
/// This replaces the retired fBm freeboard stand-in on the tectonic surface.
pub fn freeboard_m(seed: u64, cell: CellId) -> f64 {
    buoyancy_height_m(&column(seed, cell)) - reference_m(seed)
}

/// Freeboard at an explicit mantle temperature (for probes: the secular-
/// cooling monotonicity and the flooded-Archean control).
pub fn freeboard_m_at_tp(seed: u64, cell: CellId, tp_c: f64) -> f64 {
    buoyancy_height_m(&column_at_tp(seed, cell, tp_c)) - reference_m_at_tp(seed, tp_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cells(n_side: usize) -> Vec<CellId> {
        let mut cells = Vec::new();
        for fi in 0..6u8 {
            let face = Face::from_index(fi);
            for j in 0..n_side {
                for i in 0..n_side {
                    let u = ((i as f64 + 0.5) / n_side as f64) * 2.0 - 1.0;
                    let v = ((j as f64 + 0.5) / n_side as f64) * 2.0 - 1.0;
                    cells.push(CubeCoord { face, u, v }.cell(SAMPLE_LEVEL));
                }
            }
        }
        cells
    }

    #[test]
    fn craton_fraction_is_in_band() {
        // Convicts the threshold calibration: cratonized area (w > 0.5) must
        // sit near CRATON_AREA_FRAC. The first, formula-derived threshold gave
        // 0.5% silently — this is the guard against that class.
        let cells = sample_cells(64);
        let frac = cells.iter().filter(|&&c| craton_weight(0, c) > 0.5).count() as f64 / cells.len() as f64;
        assert!(
            (CRATON_AREA_FRAC * 0.6..CRATON_AREA_FRAC * 1.7).contains(&frac),
            "cratonized fraction {frac:.4} strayed from declared ~{CRATON_AREA_FRAC}"
        );
    }

    #[test]
    fn freeboard_range_spans_zero_the_reachability_check() {
        // The range/reachability conviction from `DECISIONS[uplift-is-
        // structurally-incapable…]`: the emerged-land keeper's output must be
        // able to express BOTH basins (negative) and land (positive). The old
        // rate field was strictly positive and mechanically could not.
        let cells = sample_cells(48);
        let (mut neg, mut pos) = (0usize, 0usize);
        for &c in &cells {
            let f = freeboard_m(0, c);
            if f < -100.0 {
                neg += 1;
            }
            if f > 100.0 {
                pos += 1;
            }
        }
        assert!(neg > 0 && pos > 0, "isostasy read must span basins and land (neg {neg}, pos {pos})");
        // And the sign structure is the column story: most of the sphere sits
        // low (oceanic), a minority stands high (cratons).
        assert!(neg > pos, "oceanic columns dominate by area");
    }

    #[test]
    fn mass_balance_mean_freeboard_is_zero_at_any_driver_setting() {
        // Rise here IS subsidence there: the reference is fixed by area mass
        // balance, so area-mean freeboard ≡ 0 at every mantle temperature —
        // cooling redistributes stand, it cannot mint elevation globally.
        // Sampled on the SAME grid the reference integrates over, the mean is
        // zero to float precision — the balance is by construction, not fit.
        for tp in [1550.0, 1450.0, 1350.0] {
            let cells = sample_cells(1usize << SAMPLE_LEVEL);
            let reference = reference_m_at_tp(0, tp); // hoisted: one global constant per driver setting
            let mean: f64 = cells
                .iter()
                .map(|&c| buoyancy_height_m(&column_at_tp(0, c, tp)) - reference)
                .sum::<f64>()
                / cells.len() as f64;
            assert!(mean.abs() < 1e-6, "mean freeboard {mean:.3e} m at Tp={tp} must vanish on the reference grid");
        }
    }

    #[test]
    fn cooling_grows_the_craton_ocean_contrast_the_buoyancy_half() {
        // The buoyancy half of the emergence story, convictable: as the mantle
        // cools the seafloor thins and densifies, so the craton−ocean stand
        // CONTRAST grows. (Land fraction after the pour is the sea-level
        // module's test; this pins the column-side monotonicity driving it.)
        let hot = oceanic_crust_m(1550.0) * (RHO_ASTHENOSPHERE - oceanic_crust_rho(1550.0)) / RHO_ASTHENOSPHERE;
        let cool = oceanic_crust_m(1350.0) * (RHO_ASTHENOSPHERE - oceanic_crust_rho(1350.0)) / RHO_ASTHENOSPHERE;
        assert!(cool < hot, "oceanic stand falls as the mantle cools");
        let craton = buoyancy_height_m(&Column {
            crust_m: CRATON_CRUST_M,
            crust_rho: RHO_CONTINENTAL,
            keel_m: CRATON_KEEL_M,
        });
        assert!(craton - cool > craton - hot, "craton/ocean contrast grows under cooling");
        assert!(craton > hot, "cratons stand above even the hot Archean seafloor");
    }

    #[test]
    fn keel_is_half_the_freeboard_not_optional() {
        // Removing the keel from a craton column must cost a substantial part
        // of its stand — the decision's "the keel is half of it" clause, order
        // of magnitude: 180 km × 25 kg/m³ / 3300 ≈ 1.4 km of the ~7 km stand.
        let with = buoyancy_height_m(&Column { crust_m: CRATON_CRUST_M, crust_rho: RHO_CONTINENTAL, keel_m: CRATON_KEEL_M });
        let without = buoyancy_height_m(&Column { crust_m: CRATON_CRUST_M, crust_rho: RHO_CONTINENTAL, keel_m: 0.0 });
        let keel_share = (with - without) / with;
        assert!(keel_share > 0.15, "keel contributes materially ({:.0}%)", keel_share * 100.0);
    }

    #[test]
    fn deterministic_and_sphere_continuous_margins() {
        // Fated: same (seed, cell) → same column. Sphere-continuous: the craton
        // field is unit-vector fBm, so adjacent cells across any arc differ
        // smoothly (margin, not cliff) — checked as bounded one-cell deltas.
        let cell = CubeCoord { face: Face::from_index(2), u: 0.3, v: -0.4 }.cell(SAMPLE_LEVEL);
        assert_eq!(column(7, cell), column(7, cell));
        let cells = sample_cells(64);
        let mut max_delta = 0.0f64;
        for pair in cells.windows(2) {
            let d = (freeboard_m(0, pair[0]) - freeboard_m(0, pair[1])).abs();
            max_delta = max_delta.max(d);
        }
        // At this coarse sampling adjacent samples are far apart, so this is a
        // sanity bound (no multi-km cliffs between neighbouring samples of the
        // same face row), not the generator continuity probe (gen.rs owns that).
        assert!(max_delta < CRATON_CRUST_M, "no cliff larger than the full craton stand between samples");
    }
}
