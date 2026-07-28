//! Era-erosion feasibility probe — is there anything down the cooling chain
//! worth eroding, what does it cost, and what breaks if you thread the era
//! carelessly?
//!
//! The fluvial rung builds only at the present anchor (`query::erosion_tile`
//! seeds from `gen::initial_topography_m`, which is `sea_level::tectonic_surface_m`
//! at `MANTLE_TP_C`). The column, by contrast, is already era-parameterized
//! end to end (`freeboard_m_at_tp` → `tectonic_surface_at_tp` →
//! `derived_sea_level_at_tp`), and `#form-isostasy-column` FE(5) measured that
//! suture belts — collisional orogeny — are ZERO at the present anchor for most
//! seeds and arrive only down the chain. So the terrain whose erosion could look
//! like Earth exists as a law-evaluable surface today and has never been carved.
//!
//! This probe measures, along the canonical cooling chain
//! (`mantle_thermal::cooling_stages_refined`), the four quantities an
//! era-parameterized erosion design needs and cannot get from arithmetic:
//!
//!  1. **Is there terrain?** — land fraction, peak subaerial stand, and the
//!     *relief* (subaerial standard deviation) the fluvial kernel would work on.
//!  2. **Is there structure?** — suture-belt cell count (`craton_field_at_tp`
//!     `w2 > 0`), the orogeny that arrives in time.
//!  3. **Is there a driver, and does its magnitude move?** — the era
//!     generalization of `uplift.rs`: `freeboard(T_p − δ) − freeboard(T_p)` at
//!     *that era's* `T_p` rather than at `MANTLE_TP_C`. If the driver's
//!     magnitude changes along the chain, a single authored epoch count cannot
//!     be right at every era.
//!  4. **What does a careless thread cost?** — `erosion.rs::outlets` reaches
//!     `sea_level::derived_sea_level_m(seed)`, the *present* waterline, with no
//!     era argument. This measures the land/sea mask error that would produce
//!     if an era surface were carved against the present sea: the fraction of
//!     cells the kernel would misclassify as ocean-or-land.
//!
//! Read-only: computes from pure `lithosphere` / `sea_level` / `mantle_thermal`
//! functions, opens no store, writes nothing.
//!
//! Run: `cargo run --release -p vivarium-world --example era_erosion_feasibility_probe`

use vivarium_world::lithosphere::{self, MANTLE_TP_C};
use vivarium_world::mantle_thermal::{cooling_stages_refined, potential_temp_c};
use vivarium_world::sea_level::{derived_sea_level_at_tp, tectonic_surface_at_tp};
use vivarium_world::sphere::{CubeCoord, Face};
use vivarium_world::uplift::TP_COOLING_PER_EPOCH_C;

const LEVEL: u8 = 7;
/// Samples per face axis. 6·N² cells scanned per (seed, era).
const N: usize = 48;
const SEEDS: [u64; 3] = [0, 1, 7];

struct EraStats {
    tp: f64,
    age_ga: f64,
    sea_m: f64,
    land_frac: f64,
    peak_m: f64,
    /// Standard deviation of subaerial stand — the relief the kernel carves.
    relief_sd_m: f64,
    suture_frac: f64,
    /// Era uplift rate over land (m per declared cooling step), median and max.
    uplift_med: f64,
    uplift_max: f64,
    /// Land fraction the SAME era surface would show against the PRESENT sea.
    land_frac_present_sea: f64,
    /// Fraction of all cells whose land/sea class differs between the two seas.
    mask_error_frac: f64,
    /// Mean stand the pour-grain rock-mass ledger already removed at this era
    /// (pre-ledger surface minus post-ledger surface, over pre-ledger land) —
    /// the erosion the fluvial rung would be carving *again*.
    ledger_drop_mean_m: f64,
}

fn scan(seed: u64, tp: f64, present_sea: f64) -> EraStats {
    let sea = derived_sea_level_at_tp(seed, tp);
    let mut land: Vec<f64> = Vec::new();
    let mut uplift: Vec<f64> = Vec::new();
    let mut total = 0usize;
    let mut suture = 0usize;
    let mut land_present = 0usize;
    let mut mask_diff = 0usize;
    let pre_sea = vivarium_world::sea_level::derived_sea_level_pre_ledger_at_tp(seed, tp);
    let mut ledger_drop: Vec<f64> = Vec::new();
    for fi in 0..6u8 {
        let face = Face::from_index(fi);
        for j in 0..N {
            for i in 0..N {
                let u = ((i as f64 + 0.5) / N as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / N as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(LEVEL);
                let h = tectonic_surface_at_tp(seed, cell, LEVEL, tp);
                total += 1;
                let is_land = h > sea;
                let is_land_present = h > present_sea;
                if is_land {
                    land.push(h - sea);
                    // The era generalization of `uplift::uplift_rate_m_per_epoch`,
                    // which today pins T_p at MANTLE_TP_C.
                    let r = lithosphere::freeboard_m_at_tp(seed, cell, tp - TP_COOLING_PER_EPOCH_C)
                        - lithosphere::freeboard_m_at_tp(seed, cell, tp);
                    uplift.push(r);
                }
                if is_land_present {
                    land_present += 1;
                }
                if is_land != is_land_present {
                    mask_diff += 1;
                }
                if lithosphere::craton_field_at_tp(seed, cell, tp).w2 > 0.0 {
                    suture += 1;
                }
                // How much stand the pour-grain ledger already took off here.
                let h_pre =
                    vivarium_world::sea_level::tectonic_surface_pre_ledger_at_tp(seed, cell, LEVEL, tp);
                if h_pre > pre_sea {
                    ledger_drop.push(h_pre - h);
                }
            }
        }
    }
    let n = total as f64;
    let peak = land.iter().cloned().fold(0.0f64, f64::max);
    let mean = if land.is_empty() { 0.0 } else { land.iter().sum::<f64>() / land.len() as f64 };
    let var = if land.len() < 2 {
        0.0
    } else {
        land.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (land.len() - 1) as f64
    };
    uplift.sort_by(f64::total_cmp);
    let med = if uplift.is_empty() { 0.0 } else { uplift[uplift.len() / 2] };
    let umax = uplift.iter().cloned().fold(f64::MIN, f64::max);
    EraStats {
        tp,
        age_ga: 0.0,
        sea_m: sea,
        land_frac: land.len() as f64 / n,
        peak_m: peak,
        relief_sd_m: var.sqrt(),
        suture_frac: suture as f64 / n,
        uplift_med: med,
        uplift_max: if uplift.is_empty() { 0.0 } else { umax },
        land_frac_present_sea: land_present as f64 / n,
        mask_error_frac: mask_diff as f64 / n,
        ledger_drop_mean_m: if ledger_drop.is_empty() {
            0.0
        } else {
            ledger_drop.iter().sum::<f64>() / ledger_drop.len() as f64
        },
    }
}

fn main() {
    // The canonical chain refined once (11 stages, 0.1 Ga apart, 3.6 → 2.6 Ga),
    // plus a **grid-aligned extension** to 1.0 Ga. The extension matters because
    // `#form-isostasy-column` FE(5) measured suture belts at $T_p$ 1450–1350,
    // which is COLDER than the canonical chain's 2.6 Ga cold end ($T_p$ 1464) —
    // so the chain as authored stops short of its own orogeny. Extending
    // `COOL_END_UGA` on the same micro-Ga grid preserves FE(9)'s bit-exact
    // nesting; sampling it here costs nothing and says whether it is worth it.
    let canonical = cooling_stages_refined(1);
    let cold_canonical_uga = 2_600_000i64;
    let mut stages: Vec<vivarium_world::time::Time> = canonical.clone();
    let mut uga = cold_canonical_uga - 100_000;
    while uga >= 1_000_000 {
        stages.push(vivarium_world::time::Time::from_years(-(uga as f64 / 1.0e6) * 1.0e9));
        uga -= 100_000;
    }
    println!("era-erosion feasibility probe — level {LEVEL}, {}x{} per face ({} cells/era)", N, N, 6 * N * N);
    println!("present anchor T_p = {MANTLE_TP_C} C; cooling step delta = {TP_COOLING_PER_EPOCH_C} C/epoch\n");

    for seed in SEEDS {
        let present_sea = derived_sea_level_at_tp(seed, MANTLE_TP_C);
        println!("seed {seed}  (present sea = {present_sea:.0} m)");
        println!(
            "  {:>7} {:>7} {:>8} {:>7} {:>8} {:>8} {:>8} {:>9} {:>9} {:>9} {:>9}",
            "age", "T_p", "sea(m)", "land%", "peak(m)", "reliefSD", "suture%", "upl_med", "upl_max", "maskErr%",
            "ledger(m)"
        );
        let mut rows: Vec<EraStats> = Vec::new();
        for (k, &t) in stages.iter().enumerate() {
            let tp = potential_temp_c(t);
            let mut s = scan(seed, tp, present_sea);
            s.age_ga = -t.years() / 1.0e9;
            let mark = if k == canonical.len() - 1 { "  <- canonical cold end" } else { "" };
            println!(
                "  {:>7.2} {:>7.1} {:>8.0} {:>6.2}% {:>8.0} {:>8.0} {:>6.2}% {:>9.3} {:>9.3} {:>8.2}% {:>9.0}{}",
                s.age_ga,
                s.tp,
                s.sea_m,
                s.land_frac * 100.0,
                s.peak_m,
                s.relief_sd_m,
                s.suture_frac * 100.0,
                s.uplift_med,
                s.uplift_max,
                s.mask_error_frac * 100.0,
                s.ledger_drop_mean_m,
                mark,
            );
            rows.push(s);
        }
        // The land fraction the SAME cold surface shows against the PRESENT sea —
        // the naive-threading error at its worst (coldest) end.
        if let Some(cold) = rows.last() {
            println!(
                "  naive-thread check at the cold end: land {:.2}% against the era sea vs {:.2}% against the PRESENT sea",
                cold.land_frac * 100.0,
                cold.land_frac_present_sea * 100.0
            );
        }
        println!();
    }

    // --- The epoch<->time arithmetic the chain composition forces ------------
    println!("--- cost and clock, if erosion runs ALONG the chain ---");
    let hot = potential_temp_c(canonical[0]);
    let cold = potential_temp_c(*canonical.last().unwrap());
    let span_c = hot - cold;
    let gap_c = span_c / (canonical.len() - 1) as f64;
    println!("  canonical chain spans {span_c:.1} C over {} stages ({gap_c:.1} C per 0.1 Ga stage gap)", canonical.len());
    println!(
        "  at delta = {TP_COOLING_PER_EPOCH_C} C/epoch that is {:.0} erosion epochs per stage gap, {:.0} for the whole chain",
        gap_c / TP_COOLING_PER_EPOCH_C,
        span_c / TP_COOLING_PER_EPOCH_C
    );
    // dT_p/d(age) at the present anchor, from the declared closed form.
    let d = 1.0e-4; // Ga
    let tp_of_age = |ga: f64| {
        potential_temp_c(vivarium_world::time::Time::from_years(-ga * 1.0e9))
    };
    let dtp_dage = (tp_of_age(3.2 + d) - tp_of_age(3.2 - d)) / (2.0 * d);
    let years_per_epoch = TP_COOLING_PER_EPOCH_C / dtp_dage.abs() * 1.0e9;
    println!("  the declared cooling curve makes delta worth {years_per_epoch:.3e} years/epoch at the present anchor");
    println!("  ASSUMPTIONS 'epoch <-> years' declares EPOCH_YEARS = 100 -> the two scales disagree by {:.3e}x", years_per_epoch / 100.0);
    println!("  a land uplift of ~0.8 m/epoch is {:.2e} m/yr at the cooling scale, {:.2e} m/yr at 100 yr/epoch", 0.8 / years_per_epoch, 0.8 / 100.0);
    println!("  (typical orogenic rock uplift is 1e-4 .. 1e-2 m/yr; both ends miss it, in opposite directions)");
}
