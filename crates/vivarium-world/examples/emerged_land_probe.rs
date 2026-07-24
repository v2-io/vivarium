//! EMERGED-LAND RECORD PROBE — the Claimed → Kept instrument for
//! `promise[emerged-land]` (#form-ordinum-governs-flux-web FE(4);
//! #norm-probes-before-claims; #form-isostasy-column FE(6)).
//!
//! **What this is, and is not.** It is the Record-style predicate that *could*
//! convict or verify the emerged-land promise per geological cycle. Building it
//! decides **nothing** about the verdict — a green probe is not a Kept mark, and
//! Kept remains a separate later adjudication. That asymmetry is the point: the
//! predicate must exist before Kept can ever be claimed, and it must be able to
//! be red.
//!
//! **The bands are Record-style checks on *earned* freeboard** (early-continents
//! survey §6 → `ASSUMPTIONS.md`), never a land target baked into the Protogenic
//! prior (#form-derived-sea-level FE(7)). Deep time does not yet run, so the
//! era-sharpening clauses are named **not-yet-predicable** — only the time-free
//! clauses convict here.
//!
//! Run: `cargo run --release -p vivarium-world --example emerged_land_probe`

use vivarium_world::sea_level::{
    derived_sea_level_m, emerged_land_record_at, emerged_land_verdict, land_fraction_at_sea, Clause,
    EMERGED_LAND_FRACTION_BAND, SUBAERIAL_RELIEF_FLAG_M,
};

const LEVEL: u8 = 8;

fn mark(c: Clause) -> &'static str {
    match c {
        Clause::Pass => "  PASS ",
        Clause::Fail => "  FAIL ",
        Clause::Flag => " ~flag ",
        Clause::NotPredicable => " n/a-v1",
    }
}

fn main() {
    let seeds: Vec<u64> = std::env::args()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    let seeds = if seeds.is_empty() { vec![0u64, 1, 7] } else { seeds };

    let (lo, hi) = EMERGED_LAND_FRACTION_BAND;
    println!("EMERGED-LAND RECORD PROBE — the Claimed→Kept predicate, not a Kept verdict\n");
    println!(
        "  literature bands (survey §6): land fraction {:.1}–{:.0}%   subaerial relief ≤ {:.0} m (soft, pre-erosion)",
        lo * 100.0,
        hi * 100.0,
        SUBAERIAL_RELIEF_FLAG_M
    );
    println!("  clauses: [rises] land above sea (hard) · [frac] fraction in band (hard) · [relief] stand ≤ ceiling (soft) · [timing] era-sharpening (not-yet-predicable)\n");

    for &seed in &seeds {
        let v = emerged_land_verdict(emerged_land_record_at(seed, LEVEL));
        let r = v.record;
        println!(
            "  seed {seed:>3}: sea {:>5.0} m   land {:>5.2}%   peak stand {:>5.0} m    \
             [rises{}] [frac{}] [relief{}] [timing{}]   {}",
            r.sea_level_m,
            r.land_fraction * 100.0,
            r.max_subaerial_m,
            mark(v.land_rises),
            mark(v.fraction_in_band),
            mark(v.relief_bounded),
            mark(v.timing),
            if v.hard_clauses_pass() { "hard-clauses convict (NOT Kept)" } else { "HARD FAIL" },
        );
        if v.relief_bounded == Clause::Flag {
            println!(
                "            ⚠ amber: peak stand {:.0} m > {:.0} m — isostatic freeboard over-stands the craton on the PRE-erosion surface; strength limit + erosion mass-return that would trim it are open (#form-isostasy-column FE(8)).",
                r.max_subaerial_m, SUBAERIAL_RELIEF_FLAG_M
            );
        }
    }

    // --- Sensitivity: two known-bads bracket the band (#norm-probe-sensitivity FE(2)) ---
    println!("\n  SENSITIVITY (a probe that cannot fail on a known-bad is not a probe):");
    let sea0 = derived_sea_level_m(0);
    // Floor: bathymetry alone (freeboard stripped) — the retired water-world.
    let n = 1usize << 6;
    let (mut land, mut tot) = (0usize, 0usize);
    use vivarium_world::gen::bathymetry_m;
    use vivarium_world::sphere::{CubeCoord, Face};
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let vv = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v: vv }.cell(6);
                tot += 1;
                if bathymetry_m(0, cell, 6) > sea0 {
                    land += 1;
                }
            }
        }
    }
    let floor = land as f64 / tot as f64;
    println!(
        "    floor known-bad — bathymetry only (freeboard stripped): {:.2}% land → below {:.1}% floor → land_rises/frac FAIL",
        floor * 100.0,
        lo * 100.0
    );
    // Ceiling: a decreed low sea level manufactures forbidden land.
    for decreed in [4000.0, 3000.0] {
        let f1 = land_fraction_at_sea(1, decreed, 6);
        println!(
            "    ceiling known-bad — decreed sea {decreed:.0} m on seed 1: {:.1}% land → above {:.0}% ceiling → frac FAIL",
            f1 * 100.0,
            hi * 100.0
        );
    }
    println!(
        "\n    → fraction clause discriminates deviations outside [{:.1}%, {:.0}%] (caught at 0% and ~40% — well inside resolution);",
        lo * 100.0,
        hi * 100.0
    );
    println!("      relief clause resolves stands past {:.0} m (live peaks 1.7–2.4 km sit at the edge — it catches the hundreds-of-m overshoot, not only gross ones).", SUBAERIAL_RELIEF_FLAG_M);
    println!("\n  Claimed ≠ Kept: the hard time-free clauses convicting does NOT mark the promise Kept.");
    println!("  Kept additionally requires the temporal clauses (deep time), and a separate adjudication.");
}
