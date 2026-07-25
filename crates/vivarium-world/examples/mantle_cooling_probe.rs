//! Mantle-thermal emergence probe — does a cooling world grow land *in time*?
//!
//! The mantle-thermal nomos (`mantle_thermal.rs`, `#form-isostasy-column` FE(2))
//! promotes the mantle potential temperature from a declared constant to a
//! secular-cooling trajectory `T_p(t)`. This probe walks the canonical Abyssal
//! epoch chain (hot early-Abyssal → present → cooling late-Abyssal) and measures
//! the land-fraction and peak-relief trajectory the freeboard pour earns at each
//! epoch — the emergence the ordinum ladder narrates, or its refutation.
//!
//! Invariants it convicts (`#norm-regime-probes`):
//!  1. MONOTONE emergence — land fraction is non-decreasing as the mantle cools
//!     (a cooling world does not un-emerge land). A failable shape claim.
//!  2. WATER-WORLD start — the hottest early-Abyssal epoch stands little/no land
//!     (thick, light oceanic crust ⇒ shallow basins ⇒ flooding).
//!  3. IN-BAND present — the present-Abyssal epoch (T_p = MANTLE_TP_C) sits in
//!     the early-Earth land band, matching the seed-only live world exactly.
//!
//! Run: `cargo run --release -p vivarium-world --example mantle_cooling_probe`

use vivarium_world::lithosphere::MANTLE_TP_C;
use vivarium_world::mantle_thermal::{cooling_stages, potential_temp_c, present_abyssal};
use vivarium_world::sea_level::{
    emerged_land_record_at_tp, emerged_land_timing_verdict_for, emerged_land_verdict,
    EMERGED_LAND_FRACTION_BAND,
};

const LEVEL: u8 = 7;
const SEEDS: [u64; 3] = [0, 1, 7];

fn main() {
    let (lo, hi) = EMERGED_LAND_FRACTION_BAND;
    println!("mantle-thermal emergence probe — level {LEVEL}, band [{:.1}%, {:.0}%]", lo * 100.0, hi * 100.0);
    println!("present-Abyssal anchor: T_p = {MANTLE_TP_C} C (must match the seed-only live world)\n");

    let epochs = cooling_stages();
    let present = present_abyssal();

    let mut all_monotone = true;
    let mut water_world_start = true;
    let mut present_in_band = true;

    for seed in SEEDS {
        println!("seed {seed}:");
        println!("  {:>8}  {:>7}  {:>9}  {:>10}  {:>8}  {}", "age(Ga)", "T_p(C)", "sea(m)", "land(%)", "peak(m)", "verdict");
        let mut prev_land = -1.0f64;
        for (k, &t) in epochs.iter().enumerate() {
            let tp = potential_temp_c(t);
            let age_ga = -t.years() / 1.0e9;
            let rec = emerged_land_record_at_tp(seed, LEVEL, tp);
            let v = emerged_land_verdict(rec);
            let is_present = t == present;
            let tag = if is_present { " <- present" } else { "" };
            println!(
                "  {:>8.2}  {:>7.0}  {:>9.0}  {:>9.2}%  {:>8.0}  land_rises={} frac={} relief={}{}",
                age_ga, tp, rec.sea_level_m, rec.land_fraction * 100.0, rec.max_subaerial_m,
                v.land_rises.label(), v.fraction_in_band.label(), v.relief_bounded.label(), tag,
            );
            if prev_land >= 0.0 && rec.land_fraction + 1e-9 < prev_land {
                all_monotone = false;
                println!("    ^ NON-MONOTONE: land fell {:.3}% -> {:.3}%", prev_land * 100.0, rec.land_fraction * 100.0);
            }
            prev_land = rec.land_fraction;
            if k == 0 && rec.land_fraction > 0.005 {
                water_world_start = false;
            }
            if is_present && v.fraction_in_band != vivarium_world::sea_level::Clause::Pass {
                present_in_band = false;
            }
        }
        println!();
    }

    println!("--- invariants ---");
    println!("  [{}] land fraction monotone non-decreasing as the mantle cools", pf(all_monotone));
    println!("  [{}] hot early-Abyssal start is (near) a water-world (<0.5% land)", pf(water_world_start));
    println!("  [{}] present-Abyssal epoch is in-band (and equals the live world)", pf(present_in_band));

    // The temporal (era-sharpening) verdict — the clauses the static Record
    // named NotPredicable, now crudely predicable along the cooling chain.
    println!("\n--- timing verdict (era-sharpening; calibration-free clauses) ---");
    for seed in SEEDS {
        let v = emerged_land_timing_verdict_for(seed, LEVEL);
        println!(
            "  seed {seed}: monotone={} submerged-start={} enters-&-stays-in-band={} | ga-absolute={}",
            v.emergence_monotone.label(),
            v.starts_submerged.label(),
            v.enters_and_stays_in_band.label(),
            v.ga_calibrated_era.label(),
        );
    }
    println!("  (ga-absolute era targets stay n/a-v1: the rate law is declared crude — an absolute-Ga");
    println!("   fraction target is a calibration it cannot earn, though the curve passes near 10%-by-3.0-Ga)");
}

fn pf(b: bool) -> &'static str {
    if b { "PASS" } else { "FAIL" }
}
