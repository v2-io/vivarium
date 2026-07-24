//! The rock-mass ledger — erosion's mass return to the lithospheric column, at
//! the pour grain, with isostatic rebound (`#form-isostasy-column` FE(8), the
//! last-open item of the accepted freeboard chain
//! `mantle-thermal → lithosphere → isostasy → sea-level → erosion`).
//!
//! **The gap this closes.** The isostasy read earns emerged land by column
//! buoyancy alone, and on the pre-erosion tectonic surface it *over-stands* the
//! craton — the `sea_level::emerged_land_verdict` amber flag (seeds 0/7 peak
//! ~2.4 km vs the ~2 km hot-crust ceiling). Nothing debited the column for the
//! rock erosion carries away, so nothing rebounded and nothing bounded relief.
//! This module is that debit/credit made a closed box:
//!
//!   1. **Debit (erosion).** Subaerial columns lose crust — a declared
//!      relief-driven law, [`crust_eroded_m`]: crust removed ∝ stand above sea.
//!      This is a *stand-in* for ∫(stream power) integrated over an erosion
//!      interval (physics **Low**, sibling to the fated column inventory) — the
//!      pour grain has no drainage, so height-above-base-level is the crudest
//!      honest relief proxy. Named as such, not dressed as the fluvial scheme
//!      (`#detail-erosion-composition` owns the fine-grain carving).
//!   2. **Rebound (isostasy).** Automatic in the Airy read: removing crust `δ`
//!      lowers the stand by only `δ·(ρ_a−ρ_crust)/ρ_a` (~17 % for felsic crust;
//!      the other ~83 % rebounds). See [`lithosphere::surface_drop_per_crust_removed`].
//!      This is why the ledger *bounds* relief toward the band rather than
//!      planing it flat.
//!   3. **Credit (deposition).** The eroded mass is returned to the column as a
//!      sediment blanket on submarine cells (`RHO_SEDIMENT`). v1 spreads it
//!      **uniformly** over submarine area — declared crude: no transport at the
//!      pour grain, so proximity/depth-routed deposition is the next rung.
//!      Basins fill and the seafloor rebounds down (loaded), so relief
//!      compresses from both ends.
//!
//! **The conservation claim (failable).** Rock mass is conserved across the
//! pass — `Σ (crust·ρ_crust + sediment·ρ_sed)·area` is invariant to float
//! precision (`rock_mass_balance`, the closed-box probe). This is what turns
//! `lithosphere`'s `LITHO_COLUMN` promise from `NotTracked` toward a real
//! `Conserved` edge; the decl upgrade + wiring the post-erosion surface into the
//! default pour are adoption steps (a globe-visible change bordering the view),
//! deliberately **not** taken here — this module is the measured operator and
//! the probes that convict it, keeping the default surface untouched.
//!
//! **Purity.** Like the rest of the chain, everything is a pure function of
//! `(seed, cell, T_p)`: the per-cell debit/credit reads memoized global
//! integrals (eroded mass, submarine area, post-erosion reference), never stored
//! mutable state. It is a single erosion *maturity* step — the iterated
//! erode→rebound→re-erode chain (which compounds) is a further rung, as
//! `mantle_thermal` runs discrete epochs.

use crate::gen;
use crate::lithosphere::{
    self, buoyancy_height_m, oceanic_crust_m, Column, RHO_SEDIMENT,
};
use crate::sea_level::{self, SAMPLE_LEVEL};
use crate::sphere::{CellId, CubeCoord, Face};

/// Erosion maturity `φ` — the fraction of a subaerial column's stand above sea
/// converted to crust removal in one ledger pass (dimensionless). Physics
/// **Low**: a declared stand-in for cumulative ∫(stream power) over an Abyssal
/// erosion interval, not a calibrated rate. `ASSUMPTIONS.md` "erosion maturity".
/// At `φ = 1` a 2.4 km-standing craton loses ~2.4 km of crust and rebounds to
/// ~17 % of that (~0.4 km surface drop) — an honest trim *toward* the relief
/// band, never a clamp *into* it.
pub const EROSION_MATURITY: f64 = 1.0;

/// Iterate the level-`SAMPLE_LEVEL` sample grid (the pour grain), calling `f`
/// with each cell and its uniform sample area (m²) — the same area model the
/// pour uses, so the ledger's mass integrals and the re-pour stay consistent.
fn for_each_sample<F: FnMut(CellId, f64)>(mut f: F) {
    let r_m = crate::planet::Planet::EARTH.radius_m;
    let n = 1usize << SAMPLE_LEVEL;
    let area = (4.0 * std::f64::consts::PI * r_m * r_m) / (6.0 * (n * n) as f64);
    for fi in 0..6u8 {
        let face = Face::from_index(fi);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                f(CubeCoord { face, u, v }.cell(SAMPLE_LEVEL), area);
            }
        }
    }
}

/// Crust thickness (m) eroded from the column at `cell` in one pass, under
/// mantle temperature `tp_c`. Relief-driven: `φ · stand₊`, where stand is the
/// pre-erosion tectonic surface above the pre-erosion sea. Clamped to the
/// **felsic pile** (crust above the oceanic thickness) — erosion strips the
/// buoyant continental crust, never the oceanic floor. Zero on submarine cells.
pub fn crust_eroded_m(seed: u64, cell: CellId, tp_c: f64) -> f64 {
    let sea = sea_level::derived_sea_level_at_tp(seed, tp_c);
    let surf = sea_level::tectonic_surface_at_tp(seed, cell, SAMPLE_LEVEL, tp_c);
    let stand = surf - sea;
    if stand <= 0.0 {
        return 0.0;
    }
    let base = lithosphere::column_at_tp(seed, cell, tp_c);
    let felsic_head = (base.crust_m - oceanic_crust_m(tp_c)).max(0.0);
    (EROSION_MATURITY * stand).min(felsic_head)
}

/// Memoized global integrals for one `(seed, T_p)` pass — the closed-box the
/// per-cell debit/credit read. Computed once; the pour grain has ~393 k cells.
#[derive(Clone, Copy)]
struct Ledger {
    /// Uniform sediment thickness (m) deposited on every submarine cell — the
    /// credit, sized so `Σ deposit·ρ_sed·area ≡ eroded_mass` by construction.
    deposit_m: f64,
    /// Area-mean post-erosion buoyancy height (m) — the re-fixed isostatic
    /// reference, so post-erosion freeboard stays zero-mean (rise here IS
    /// subsidence there, after the ledger).
    post_reference: f64,
}

fn ledger(seed: u64, tp_c: f64) -> Ledger {
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    static CACHE: Mutex<Option<BTreeMap<(u64, u64), Ledger>>> = Mutex::new(None);
    let key = (seed, tp_c.to_bits());
    {
        let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(&l) = guard.get_or_insert_with(BTreeMap::new).get(&key) {
            return l;
        }
    }

    let sea = sea_level::derived_sea_level_at_tp(seed, tp_c);

    // Pass 1 — the debit and the submarine catchment.
    let mut eroded_mass = 0.0f64;
    let mut submarine_area = 0.0f64;
    for_each_sample(|cell, area| {
        let surf = sea_level::tectonic_surface_at_tp(seed, cell, SAMPLE_LEVEL, tp_c);
        if surf > sea {
            let base = lithosphere::column_at_tp(seed, cell, tp_c);
            let eroded = crust_eroded_m(seed, cell, tp_c);
            eroded_mass += eroded * base.crust_rho * area;
        } else {
            submarine_area += area;
        }
    });
    // Return the eroded mass to the column as a uniform submarine blanket. If
    // there is no submarine area (a dry world), the mass has nowhere to land —
    // deposit_m stays 0 and the closed box is crust-only (the probe still holds
    // because eroded_mass would then be 0: nothing stands to erode on a world
    // with no basins to have poured a sea).
    let deposit_m = if submarine_area > 0.0 { eroded_mass / (RHO_SEDIMENT * submarine_area) } else { 0.0 };

    // Pass 2 — the post-erosion isostatic reference (area-mean post buoyancy).
    let mut buoy_sum = 0.0f64;
    let mut area_sum = 0.0f64;
    for_each_sample(|cell, area| {
        let after = column_after_erosion_inner(seed, cell, tp_c, sea, deposit_m);
        buoy_sum += buoyancy_height_m(&after) * area;
        area_sum += area;
    });
    let post_reference = buoy_sum / area_sum;

    let l = Ledger { deposit_m, post_reference };
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(BTreeMap::new).insert(key, l);
    l
}

/// The post-erosion column, given the (pre-erosion) sea and the deposit blanket
/// — the shared kernel of the two ledger passes and the public read.
fn column_after_erosion_inner(seed: u64, cell: CellId, tp_c: f64, sea: f64, deposit_m: f64) -> Column {
    let base = lithosphere::column_at_tp(seed, cell, tp_c);
    let surf = sea_level::tectonic_surface_at_tp(seed, cell, SAMPLE_LEVEL, tp_c);
    if surf > sea {
        // Subaerial: debit crust.
        Column { crust_m: base.crust_m - crust_eroded_m(seed, cell, tp_c), ..base }
    } else {
        // Submarine: credit the sediment blanket.
        Column { sediment_m: base.sediment_m + deposit_m, ..base }
    }
}

/// The lithospheric column at `cell` after one rock-mass-ledger pass under `tp_c`
/// — crust debited on land, sediment credited under the sea.
pub fn column_after_erosion(seed: u64, cell: CellId, tp_c: f64) -> Column {
    let l = ledger(seed, tp_c);
    let sea = sea_level::derived_sea_level_at_tp(seed, tp_c);
    column_after_erosion_inner(seed, cell, tp_c, sea, l.deposit_m)
}

/// Post-erosion isostatic freeboard (m) at `cell` under `tp_c` — the Airy read
/// of the debited/credited column, re-referenced to area mass balance, so the
/// rebound is baked in and the field stays zero-mean.
pub fn freeboard_after_erosion_m_at_tp(seed: u64, cell: CellId, tp_c: f64) -> f64 {
    let l = ledger(seed, tp_c);
    buoyancy_height_m(&column_after_erosion(seed, cell, tp_c)) - l.post_reference
}

/// Post-erosion tectonic surface (m) — bathymetry plus the rebounded freeboard.
pub fn tectonic_surface_after_erosion_at_tp(seed: u64, cell: CellId, level: u8, tp_c: f64) -> f64 {
    gen::bathymetry_m(seed, cell, level) + freeboard_after_erosion_m_at_tp(seed, cell, tp_c)
}

/// Derived sea level (m) re-poured against the post-erosion surface under `tp_c`
/// — the ledger loads basins with sediment and shaves the cratons, so the
/// waterline shifts. Memoized per `(seed, T_p)`.
pub fn derived_sea_level_after_erosion_at_tp(seed: u64, tp_c: f64) -> f64 {
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    static CACHE: Mutex<Option<BTreeMap<(u64, u64), f64>>> = Mutex::new(None);
    let key = (seed, tp_c.to_bits());
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let map = guard.get_or_insert_with(BTreeMap::new);
    if let Some(&s) = map.get(&key) {
        return s;
    }
    let s = sea_level::pour_ocean_over(SAMPLE_LEVEL, |coord| {
        tectonic_surface_after_erosion_at_tp(seed, coord.cell(SAMPLE_LEVEL), SAMPLE_LEVEL, tp_c)
    });
    map.insert(key, s);
    s
}

/// The emerged-land Record on the **post-erosion** surface at `tp_c`, sampled at
/// `level` — the same measurement `sea_level::emerged_land_record_at_tp` takes,
/// but on the rebounded surface. Feed it to `sea_level::emerged_land_verdict` to
/// see the amber relief clause move toward the band.
pub fn emerged_land_record_after_erosion_at_tp(seed: u64, level: u8, tp_c: f64) -> sea_level::EmergedLandRecord {
    let sea = derived_sea_level_after_erosion_at_tp(seed, tp_c);
    let n = 1usize << level;
    let mut land = 0usize;
    let mut total = 0usize;
    let mut max_sub = 0.0f64;
    for fi in 0..6u8 {
        let face = Face::from_index(fi);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(level);
                let h = tectonic_surface_after_erosion_at_tp(seed, cell, level, tp_c);
                total += 1;
                if h > sea {
                    land += 1;
                    max_sub = max_sub.max(h - sea);
                }
            }
        }
    }
    sea_level::EmergedLandRecord {
        sea_level_m: sea,
        land_fraction: land as f64 / total as f64,
        max_subaerial_m: max_sub,
        level,
    }
}

/// Closed-box rock-mass balance for one ledger pass at `tp_c`: total rock mass
/// (`Σ (crust·ρ_crust + sediment·ρ_sed)·area`) **before** and **after**. The
/// conservation claim is `before ≡ after` — the ledger moves rock, it does not
/// mint or destroy it. Returned as a pair so the probe can assert the relative
/// gap (and can *fail* if the debit and credit integrals ever diverge).
pub fn rock_mass_balance(seed: u64, tp_c: f64) -> (f64, f64) {
    let l = ledger(seed, tp_c);
    let sea = sea_level::derived_sea_level_at_tp(seed, tp_c);
    let mut before = 0.0f64;
    let mut after = 0.0f64;
    for_each_sample(|cell, area| {
        let base = lithosphere::column_at_tp(seed, cell, tp_c);
        before += (base.crust_m * base.crust_rho + base.sediment_m * RHO_SEDIMENT) * area;
        let a = column_after_erosion_inner(seed, cell, tp_c, sea, l.deposit_m);
        after += (a.crust_m * a.crust_rho + a.sediment_m * RHO_SEDIMENT) * area;
    });
    (before, after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sea_level::emerged_land_verdict;

    const LVL: u8 = 6; // coarse for test speed; the probe walks level 8

    #[test]
    fn rock_mass_is_conserved_across_the_ledger_pass() {
        // THE failable conservation claim: erosion + deposition move rock, they
        // do not mint or destroy it. If the debit (eroded crust) and the credit
        // (deposited sediment) integrals ever diverge — a scaling slip, a
        // classification mismatch, an area inconsistency — this fails.
        for seed in [0u64, 1, 7] {
            let tp = lithosphere::MANTLE_TP_C;
            let (before, after) = rock_mass_balance(seed, tp);
            let rel = (after - before).abs() / before;
            assert!(rel < 1e-9, "seed {seed}: rock mass not conserved ({before:.6e} -> {after:.6e}, rel {rel:.2e})");
            // And the ledger is not a no-op: some rock actually moved (a nonzero
            // sediment blanket implies a nonzero eroded debit that balanced it).
            let l = ledger(seed, tp);
            assert!(l.deposit_m > 0.0, "seed {seed}: nothing deposited — the ledger convicts a live subaerial world");
        }
    }

    #[test]
    fn post_erosion_freeboard_is_zero_mean_rebound_redistributes() {
        // The re-referenced post-erosion freeboard preserves the mass-balance
        // invariant (rise here IS subsidence there) — area-mean ≡ 0 on the
        // reference grid, exactly as the pre-erosion read does.
        let tp = lithosphere::MANTLE_TP_C;
        let mut sum = 0.0f64;
        let mut count = 0usize;
        super::for_each_sample(|cell, _area| {
            sum += freeboard_after_erosion_m_at_tp(0, cell, tp);
            count += 1;
        });
        let mean = sum / count as f64;
        assert!(mean.abs() < 1e-6, "post-erosion mean freeboard {mean:.3e} must vanish on the reference grid");
    }

    #[test]
    fn ledger_trims_the_amber_over_stand_toward_the_band() {
        // The calibration OBSERVATION (not a verdict): the peak subaerial stand
        // falls after the ledger — rebound-bounded, toward (not clamped into)
        // the ~2 km band. Seeds 0/7 amber-flag pre-erosion (~2.4 km); the ledger
        // must move them DOWN. This is an observation for DECISIONS `proposed`,
        // never a Kept mark.
        let tp = lithosphere::MANTLE_TP_C;
        for seed in [0u64, 7] {
            let pre = sea_level::emerged_land_record_at_tp(seed, LVL, tp);
            let post = emerged_land_record_after_erosion_at_tp(seed, LVL, tp);
            assert!(
                post.max_subaerial_m < pre.max_subaerial_m,
                "seed {seed}: ledger must trim the over-stand ({:.0} m -> {:.0} m)",
                pre.max_subaerial_m, post.max_subaerial_m
            );
            // Land does not vanish — the ledger bounds relief, it does not plane
            // the continents flat (the rebound is why).
            assert!(post.land_fraction > 0.0, "seed {seed}: land must survive erosion (rebound bounds, not planes)");
            // Hard band clauses still hold on the trimmed surface.
            let v = emerged_land_verdict(post);
            assert!(v.hard_clauses_pass(), "seed {seed}: post-erosion world still convicts the hard band clauses");
        }
    }

    #[test]
    fn deterministic() {
        let tp = lithosphere::MANTLE_TP_C;
        let cell = CubeCoord { face: Face::from_index(2), u: 0.3, v: -0.4 }.cell(SAMPLE_LEVEL);
        assert_eq!(column_after_erosion(0, cell, tp), column_after_erosion(0, cell, tp));
        assert_eq!(
            derived_sea_level_after_erosion_at_tp(0, tp),
            derived_sea_level_after_erosion_at_tp(0, tp)
        );
    }
}
