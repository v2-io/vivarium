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
use crate::uplift;

/// Sampling level for the global pour (256² × 6 ≈ 393k cells). Coarse enough
/// to be cheap; fine enough that the inverted level is stable for v0.
const SAMPLE_LEVEL: u8 = 8;

/// Sea level (m above bedrock datum) for this world-seed: ocean stock poured
/// into the **tectonic** surface (bathymetry + freeboard). Pure function of seed.
///
/// Memoized per process: the pour is deterministic and seed-keyed.
pub fn derived_sea_level_m(seed: u64) -> f64 {
    use std::sync::Mutex;
    use std::collections::HashMap;
    static CACHE: Mutex<Option<HashMap<u64, f64>>> = Mutex::new(None);
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let map = guard.get_or_insert_with(HashMap::new);
    if let Some(&s) = map.get(&seed) {
        return s;
    }
    let s = pour_ocean(seed);
    map.insert(seed, s);
    s
}

/// Solid surface used for the pour and for land classification: bathymetry
/// plus Abyssal freeboard (zero-mean isostatic stand-in).
pub fn tectonic_surface_m(seed: u64, cell: crate::sphere::CellId, nyquist_level: u8) -> f64 {
    gen::bathymetry_m(seed, cell, nyquist_level) + uplift::freeboard_m(seed, cell)
}

fn pour_ocean(seed: u64) -> f64 {
    let planet = Planet::EARTH;
    let ocean_km3 = Hydrosphere::of(&planet).ocean_km3;
    let n = 1usize << SAMPLE_LEVEL;
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
                let cell = CubeCoord { face, u, v }.cell(SAMPLE_LEVEL);
                let h = tectonic_surface_m(seed, cell, SAMPLE_LEVEL);
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
