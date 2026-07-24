//! ROCK-MASS LEDGER PROBE — erosion's mass return to the lithospheric column,
//! with isostatic rebound (`#form-isostasy-column` FE(8); `crate::erosion_return`).
//!
//! **What this measures.** The last-open item of the freeboard chain: erosion
//! debits crust from subaerial columns, rebound is automatic in the Airy read,
//! and the eroded mass is returned as a submarine sediment blanket — a closed
//! box (rock mass conserved). The calibration observation is the amber
//! over-stand (`sea_level::emerged_land_verdict`, seeds 0/7 ~2.4 km) moving
//! toward the ~2 km band — an observation for DECISIONS `proposed`, never a Kept
//! mark. Rebound is why relief is *bounded*, not planed flat.
//!
//! Run: `cargo run --release -p vivarium-world --example rock_mass_ledger_probe`

use vivarium_world::erosion_return::{
    derived_sea_level_after_erosion_at_tp, emerged_land_record_after_erosion_at_tp,
    rock_mass_balance, EROSION_MATURITY,
};
use vivarium_world::lithosphere::{surface_drop_per_crust_removed, MANTLE_TP_C, RHO_CONTINENTAL};
use vivarium_world::sea_level::{emerged_land_record_pre_ledger_at_tp, emerged_land_verdict, Clause, SUBAERIAL_RELIEF_FLAG_M};

const LEVEL: u8 = 8;

fn main() {
    let seeds: Vec<u64> = std::env::args().skip(1).filter_map(|s| s.parse().ok()).collect();
    let seeds = if seeds.is_empty() { vec![0u64, 1, 7] } else { seeds };
    let tp = MANTLE_TP_C;

    println!("ROCK-MASS LEDGER PROBE — erosion returns mass to the column; rebound bounds relief\n");
    println!(
        "  erosion maturity φ = {EROSION_MATURITY}  ·  felsic rebound = {:.0}% of eroded crust returns as uplift  ·  relief band ≤ {:.0} m",
        (1.0 - surface_drop_per_crust_removed(RHO_CONTINENTAL)) * 100.0,
        SUBAERIAL_RELIEF_FLAG_M
    );
    println!("  present-Abyssal epoch (T_p = {tp} °C), pour grain level {LEVEL}\n");

    for &seed in &seeds {
        let pre = emerged_land_record_pre_ledger_at_tp(seed, LEVEL, tp);
        let post = emerged_land_record_after_erosion_at_tp(seed, LEVEL, tp);
        let (before, after) = rock_mass_balance(seed, tp);
        let rel = (after - before).abs() / before;

        let pre_v = emerged_land_verdict(pre);
        let post_v = emerged_land_verdict(post);
        let mark = |c: Clause| match c {
            Clause::Pass => "PASS",
            Clause::Fail => "FAIL",
            Clause::Flag => "~flag",
            Clause::NotPredicable => "n/a",
        };

        println!("  seed {seed:>3}:");
        println!(
            "    pre-erosion : sea {:>5.0} m   land {:>5.2}%   peak stand {:>5.0} m   [relief {}]",
            pre.sea_level_m, pre.land_fraction * 100.0, pre.max_subaerial_m, mark(pre_v.relief_bounded)
        );
        println!(
            "    post-ledger : sea {:>5.0} m   land {:>5.2}%   peak stand {:>5.0} m   [relief {}]",
            post.sea_level_m, post.land_fraction * 100.0, post.max_subaerial_m, mark(post_v.relief_bounded)
        );
        println!(
            "    Δ peak stand: {:>+6.0} m  ({:.1}% trim)   ·   rock mass conserved: rel gap {rel:.2e}",
            post.max_subaerial_m - pre.max_subaerial_m,
            100.0 * (pre.max_subaerial_m - post.max_subaerial_m) / pre.max_subaerial_m.max(1.0),
        );
        let _ = derived_sea_level_after_erosion_at_tp(seed, tp); // exercised above; kept as the named surface
        println!();
    }

    println!("  ADOPTED (2026-07-24): 'post-ledger' IS the live default surface — the pour, land classification, globe");
    println!("  and every reader now see it; LITHO_COLUMN is Conserved (closed-box probe as instrument). 'pre' is the");
    println!("  retained pre-ledger isostatic surface. Still Claimed ≠ Kept. Open rungs: routed/proximity deposition");
    println!("  (v1 uniform blanket), iterated erode→rebound epochs (v1 one φ=1 step), water loading.");
}
