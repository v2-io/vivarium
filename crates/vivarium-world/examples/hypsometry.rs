//! Hypsometry probe — the planet's height statistics vs Earth's (Joseph's ask,
//! 2026-07-10: "are land/water proportions and height percentiles even
//! statistically plausible?").
//!
//! Samples the initial-topography prior over all six faces and prints land fraction,
//! elevation percentiles (relative to sea level), land/ocean means, and an
//! ASCII hypsometric histogram, next to Earth's reference values. Earth's
//! signature feature is **bimodality** — two humps (continental platform
//! ~+0.5 km, abyssal plain ~−4.5 km) from the two crust types (granitic
//! continental vs basaltic oceanic, floating at different isostatic heights).
//! Single-population fBm is unimodal by construction, so this probe shows at a
//! glance *which* statistical property the drowned-sponge look is missing.
//!
//! Sampling note: equiangular cells subtend near-equal solid angles (max/min
//! ≈ 1.41), so unweighted cell statistics are area-honest to ~±20% bin skew —
//! fine for the shape of the distribution, flagged for exactness.
//!
//! Env: `VIVARIUM_WORLD` (reads that vivium's manifest for the seed — point it
//! at the globe's world dir to measure the planet on screen; default) ·
//! `VIVARIUM_SEED` (explicit seed if no world dir) · `HYPSO_LEVEL` (default 7).
//!
//! Run: `cargo run --release -p vivarium-world --example hypsometry`

use vivarium_world::gen::{initial_topography_m, SEA_LEVEL_M};
use vivarium_world::spec::WorldSpec;
use vivarium_world::sphere::{CellId, Face};

fn main() {
    // The seed: the globe's world dir (measure the planet being looked at),
    // else VIVARIUM_SEED, else 0.
    let world_dir = std::env::var("VIVARIUM_WORLD").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default())
            .join(".cache/vivarium/globe-world")
    });
    let (seed, provenance) = match WorldSpec::load(&world_dir) {
        Ok(Some(spec)) => (spec.seed, format!("vivium \"{}\" at {}", spec.name, world_dir.display())),
        _ => {
            let s = std::env::var("VIVARIUM_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
            (s, format!("seed {s} (no world dir found)"))
        }
    };
    let level: u8 = std::env::var("HYPSO_LEVEL").ok().and_then(|v| v.parse().ok()).unwrap_or(7);
    let nx = 1u32 << level;

    let mut h: Vec<f64> = Vec::with_capacity((6 * nx as usize) * nx as usize);
    for f in 0..6u8 {
        let face = Face::from_index(f);
        for j in 0..nx {
            for i in 0..nx {
                let cell = CellId::from_face_ij(face, i, j, level);
                h.push(initial_topography_m(seed, cell, level) - SEA_LEVEL_M);
            }
        }
    }
    h.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = h.len();
    let pct = |p: f64| h[((p / 100.0) * (n - 1) as f64).round() as usize];
    let land: Vec<f64> = h.iter().copied().filter(|&x| x > 0.0).collect();
    let ocean: Vec<f64> = h.iter().copied().filter(|&x| x <= 0.0).collect();
    let mean = |v: &[f64]| if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 };

    println!("hypsometry of {provenance} — L{level}, {n} cells, elevations relative to sea level");
    println!();
    println!("                       this planet      Earth (reference, from memory — verify before citing)");
    println!("  land fraction        {:>8.1} %        29.2 %", 100.0 * land.len() as f64 / n as f64);
    println!("  mean land elev       {:>8.0} m        ~+840 m", mean(&land));
    println!("  mean ocean depth     {:>8.0} m        ~-3700 m", mean(&ocean));
    println!("  min (deepest)        {:>8.0} m        ~-10920 m (Challenger Deep)", h[0]);
    println!("  max (highest)        {:>8.0} m        ~+8849 m (Everest)", h[n - 1]);
    println!("  p1/p25/p50/p75/p99   {:>6.0} / {:.0} / {:.0} / {:.0} / {:.0} m", pct(1.0), pct(25.0), pct(50.0), pct(75.0), pct(99.0));
    println!();
    println!("  histogram (equal elevation bins; Earth's is BIMODAL — humps at ~+0.5 km and ~-4.5 km):");
    let (lo, hi) = (h[0], h[n - 1]);
    let bins = 36usize;
    let mut counts = vec![0usize; bins];
    for &x in &h {
        let b = (((x - lo) / (hi - lo)) * (bins - 1) as f64).round() as usize;
        counts[b.min(bins - 1)] += 1;
    }
    let cmax = *counts.iter().max().unwrap();
    for (b, &c) in counts.iter().enumerate() {
        let mid = lo + (hi - lo) * (b as f64 + 0.5) / bins as f64;
        let bar = "#".repeat((c * 48 / cmax).max(usize::from(c > 0)));
        let sea = if mid.abs() < (hi - lo) / bins as f64 { " <- sea level" } else { "" };
        println!("  {mid:>7.0} m  {bar}{sea}");
    }
}
