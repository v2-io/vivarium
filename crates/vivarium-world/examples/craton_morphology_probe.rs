//! CRATON MORPHOLOGY PROBE — the failable instrument that convicts the *shape
//! class* of the cratonization field (`#form-isostasy-column` FE(8);
//! `#norm-regime-probes`, `#norm-probe-sensitivity`).
//!
//! A threshold slice through a scale-free (fBm) field is percolation speckle —
//! the wrong morphology class for cratons, however honest the heights (Joseph,
//! on the live globe: the land "looks distinctly fBm-like"). The live field is a
//! fated nucleation-and-growth law: a FEW coherent cratons of characteristic
//! scale. This probe measures the difference, area-controlled: it thresholds the
//! retired fBm known-bad to the SAME land budget as the live field, then counts
//! how many connected components that identical budget breaks into. Nucleation-
//! growth stays coherent (few components); scale-free fBm shatters.
//!
//! Run: `cargo run --release -p vivarium-world --example craton_morphology_probe`

use vivarium_world::craton_morphology::{fbm_speckle_matched_field, measure, nucleation_growth_field};

const LEVEL: u8 = 6;

fn main() {
    let seeds: Vec<u64> = std::env::args().skip(1).filter_map(|s| s.parse().ok()).collect();
    let seeds = if seeds.is_empty() { vec![0u64, 1, 7] } else { seeds };

    println!("CRATON MORPHOLOGY PROBE — nucleation-growth vs fBm speckle at a matched land budget (L{LEVEL})\n");
    println!(
        "  {:>5}  {:>28}  {:>34}",
        "seed", "NUCLEATION-GROWTH (live)", "fBm THRESHOLD @ matched area (known-bad)"
    );
    println!(
        "  {:>5}  {:>10} {:>7} {:>8}  {:>10} {:>7} {:>8}  {:>7}",
        "", "frac", "comps", "largest", "frac", "comps", "largest", "ratio"
    );

    let mut worst_ratio = f64::MAX;
    for seed in seeds {
        let good = measure(seed, LEVEL, nucleation_growth_field(LEVEL));
        let matched = fbm_speckle_matched_field(seed, LEVEL, good.cratonized_frac);
        let bad = measure(seed, LEVEL, &matched);
        let ratio = bad.component_count as f64 / good.component_count.max(1) as f64;
        worst_ratio = worst_ratio.min(ratio);
        println!(
            "  {:>5}  {:>10.3} {:>7} {:>8.3}  {:>10.3} {:>7} {:>8.3}  {:>6.2}x",
            seed,
            good.cratonized_frac,
            good.component_count,
            good.largest_component_frac,
            bad.cratonized_frac,
            bad.component_count,
            bad.largest_component_frac,
            ratio
        );
    }
    println!(
        "\n  discriminator: fBm at matched area fragments >= {:.2}x more components than nucleation-growth.",
        worst_ratio
    );
    println!("  (boundary coherence does NOT separate the classes — both are locally compact — so it is not the bar.)");
    println!("  verdict: {}", if worst_ratio >= 1.5 { "nucleation-growth is coherent; fBm speckle convicted." } else { "FIELDS DID NOT SEPARATE — probe would be red." });
}
