//! Matched-ratio arms for halo schedule science
//! (`#form-same-level-halo-exchange` Working Notes; dossier §4.1).
//!
//! Runs production [`carve_region_jacobi_exchange`] (sill1 + flux1 live) at
//! pairs `(d, σ)` with **matched** `d/σ` and reports mean |Δh| vs a single-field
//! REF. If the ratio is the right group, equal ratios should agree within the
//! chaos floor; if `d − σ` is the group, they will not.
//!
//! Store-free. Knobs: `VIVARIUM_SEED`, `VIVARIUM_LEVEL`, `VIVARIUM_FACE`,
//! `VIVARIUM_OI`, `VIVARIUM_OJ`, `VIVARIUM_SPAN`, `VIVARIUM_TILE`, `VIVARIUM_EPOCHS`.
//!
//! Run: `cargo run --release -p vivarium-world --example matched_ratio_probe`

use vivarium_world::erosion::{
    carve_region_jacobi_exchange, EdgeContract, Fluvial, FluvialParams, HaloSchedule,
};
use vivarium_world::sphere::{CellId, Face};

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}
fn env_usize(k: &str, d: usize) -> usize {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn main() {
    let seed = env_u64("VIVARIUM_SEED", 17_425_063_241_017_297_386);
    let level = env_usize("VIVARIUM_LEVEL", 13) as u8;
    let face = Face::from_index(env_usize("VIVARIUM_FACE", 1) as u8);
    let oi = env_usize("VIVARIUM_OI", 640) as i64;
    let oj = env_usize("VIVARIUM_OJ", 5376) as i64;
    let span = env_usize("VIVARIUM_SPAN", 128);
    let tile = env_usize("VIVARIUM_TILE", 64);
    let epochs = env_u64("VIVARIUM_EPOCHS", 50) as u32;
    assert!(span % tile == 0 && span >= tile);
    let per = span / tile;

    let prior = |i: i64, j: i64| {
        let cell = CellId::from_face_ij(face, i.max(0) as u32, j.max(0) as u32, level);
        vivarium_world::gen::initial_topography_m(seed, cell, level) as f32
    };
    let mk = |oi0: i64, oj0: i64, nx: usize| {
        let mut f = Fluvial::from_surface(seed, face, level, oi0.max(0) as u32, oj0.max(0) as u32, nx, |c| {
            vivarium_world::gen::initial_topography_m(seed, c, level)
        });
        // Windows are partial tiles → edge sinks on the window perimeter (probe
        // operating form); production Jacobi uses the same.
        f.set_edge_contract(EdgeContract::BaseLevelSink);
        f
    };

    // REF: one field.
    let mut r = Fluvial::from_surface(seed, face, level, oi.max(0) as u32, oj.max(0) as u32, span, |c| {
        vivarium_world::gen::initial_topography_m(seed, c, level)
    });
    r.set_edge_contract(EdgeContract::BaseLevelSink);
    r.erode(&FluvialParams {
        epochs,
        ..Default::default()
    });
    let reference = r.h.clone();

    let assemble = |tiles: &[vivarium_world::erosion::ExchangedTile]| -> Vec<f32> {
        let mut h = vec![0.0f32; span * span];
        for t in tiles {
            let ti = ((t.oi as i64 - oi) / tile as i64) as usize;
            let tj = ((t.oj as i64 - oj) / tile as i64) as usize;
            for j in 0..tile {
                for i in 0..tile {
                    h[(tj * tile + j) * span + (ti * tile + i)] = t.h[j * tile + i];
                }
            }
        }
        h
    };
    let mean_abs = |a: &[f32], b: &[f32]| {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs() as f64)
            .sum::<f64>()
            / a.len() as f64
    };

    // Matched-ratio pairs: (d, σ) with d/σ ∈ {0.8, 1.0, 1.6} at two scales each.
    let arms: &[(u16, u32)] = &[
        (8, 10),  // 0.8
        (16, 20), // 0.8
        (10, 10), // 1.0
        (16, 16), // 1.0
        (16, 10), // 1.6
        (32, 20), // 1.6
    ];

    println!(
        "matched_ratio_probe — seed {seed} L{level} f{} o({oi},{oj}) span {span} tile {tile} epochs {epochs}",
        face.index()
    );
    println!("production Jacobi path (sill1+flux1). mean |Δh| vs one-field REF.\n");
    println!(
        "{:>6} {:>6} {:>8} | {:>10}",
        "d", "σ", "d/σ", "mean |Δh|"
    );

    let mut by_ratio: std::collections::BTreeMap<i64, Vec<f64>> = std::collections::BTreeMap::new();
    for &(depth, cadence) in arms {
        let schedule = HaloSchedule {
            depth,
            cadence: cadence.min(epochs).max(1),
            cone_rho: 0,
        };
        let tiles = carve_region_jacobi_exchange(
            oi,
            oj,
            tile,
            per,
            per,
            epochs,
            schedule,
            |oi0, oj0, nx| mk(oi0, oj0, nx),
            prior,
            |_, _| {},
        );
        let h = assemble(&tiles);
        let m = mean_abs(&h, &reference);
        let ratio = depth as f64 / schedule.cadence as f64;
        let key = (ratio * 100.0).round() as i64;
        by_ratio.entry(key).or_default().push(m);
        println!(
            "{:>6} {:>6} {:>8.2} | {m:>10.2}",
            depth, schedule.cadence, ratio
        );
    }

    println!("\n-- within-ratio spread (lower ⇒ ratio groups better) --");
    for (k, vals) in &by_ratio {
        let r = *k as f64 / 100.0;
        let min = vals.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        println!(
            "  d/σ ≈ {r:.2}: n={} spread {:.2} m  (min {:.2}, max {:.2})",
            vals.len(),
            max - min,
            min,
            max
        );
    }
    println!("\nInterpretation: if matched ratios agree closer than cross-ratio pairs,");
    println!("the d/(vσ) group is supported; if not, keep the operating point empirical.");
}
