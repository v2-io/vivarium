//! SEA-LEVEL PROBE — do the ocean we declared and the basins we generated agree?
//!
//! **The question.** `SEA_LEVEL_M = 4000` is an *arbitrary datum* (`ASSUMPTIONS.md`),
//! and the ledger has always said it should be **inverted**: sea level ought to be
//! *derived* by pouring the declared water inventory into the planet's hypsometry.
//! We finally have that inventory — the `hydrosphere` nomos derives a **conserved**
//! ocean stock from the ante-mundane water-mass fraction, and right now nothing
//! consumes it.
//!
//! So this probe pours it in. It asks two things the project has never asked:
//!
//! 1. **Can the terrain even HOLD the ocean we declared?** (Compare the ocean volume
//!    against the total volume the basins can contain below the highest ground.)
//! 2. **Where would sea level actually sit** if it were derived rather than decreed —
//!    and what land fraction would follow?
//!
//! This is a coherence check between two things declared *independently* and never
//! reconciled: the water inventory (earth-referenced, conserved) and the surface
//! relief (arbitrary fBm bands, `ASSUMPTIONS.md` "continental band" ±1500 m). If they
//! disagree, that is a real mechanical finding — the same species as "rain without a
//! sky", caught the same way: by making a declared thing *derivable*.
//!
//! **Areas are TRUE spherical areas** (per-cell solid angle by spherical excess of the
//! four corners), not the uniform-cell approximation the kernels assume — because
//! `grid_lab` measured that the uniform assumption leaks up to ~10% of mass at coarse
//! levels, and a global volume integral is exactly where that would bite. The sum of
//! the solid angles is checked against 4π as a self-test.
//!
//! Run: `cargo run --release -p vivarium-world --example sea_level_probe`

use vivarium_world::gen::{initial_topography_m, SEA_LEVEL_M};
use vivarium_world::hydrosphere::Hydrosphere;
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CubeCoord, Face};

const LEVEL: u8 = 8; // 256² cells/face × 6 = 393k samples — plenty for a global integral

fn main() {
    let planet = Planet::EARTH;
    let r = planet.radius_m;
    let seed: u64 = std::env::var("VIVARIUM_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0);

    // --- 1. The declared, conserved ocean (from the ante-mundane mass fraction) ---
    let h2o = Hydrosphere::of(&planet);
    let ocean_km3 = h2o.ocean_km3;

    // --- 2. Sample the planet's surface with TRUE per-cell areas ------------------
    let n = 1usize << LEVEL;
    let mut cells: Vec<(f64, f64)> = Vec::with_capacity(6 * n * n); // (elevation_m, area_km2)
    let mut solid_angle = 0.0f64;

    for f in 0..6u8 {
        let face = Face::from_index(f);
        let corner = |i: usize, j: usize| -> [f64; 3] {
            let u = (i as f64 / n as f64) * 2.0 - 1.0;
            let v = (j as f64 / n as f64) * 2.0 - 1.0;
            CubeCoord { face, u: u.clamp(-0.999999, 0.999999), v: v.clamp(-0.999999, 0.999999) }.to_unit()
        };
        for j in 0..n {
            for i in 0..n {
                // Elevation at the cell CENTRE (never on an edge — sphere.rs's from_unit trap).
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(LEVEL);
                let h = initial_topography_m(seed, cell, LEVEL);

                // TRUE area: spherical excess of the four corners (two triangles).
                let (a, b, c, d) = (corner(i, j), corner(i + 1, j), corner(i + 1, j + 1), corner(i, j + 1));
                let omega = tri(a, b, c) + tri(a, c, d);
                solid_angle += omega;
                let area_km2 = omega * (r / 1000.0) * (r / 1000.0);
                cells.push((h, area_km2));
            }
        }
    }
    let total_area_km2: f64 = cells.iter().map(|c| c.1).sum();

    println!("SEA-LEVEL PROBE — pour the declared ocean into the generated basins\n");
    println!(
        "  self-test   Σ solid angle = {:.6} (4π = {:.6}, err {:.1e})   surface {:.4e} km²",
        solid_angle,
        4.0 * std::f64::consts::PI,
        (solid_angle - 4.0 * std::f64::consts::PI).abs(),
        total_area_km2
    );

    // --- 3. What the terrain actually is -----------------------------------------
    let (mut lo, mut hi) = (f64::MAX, f64::MIN);
    for &(h, _) in &cells {
        lo = lo.min(h);
        hi = hi.max(h);
    }
    let mean_h: f64 = cells.iter().map(|(h, a)| h * a).sum::<f64>() / total_area_km2;
    println!("\n  terrain     min {lo:.0} m   mean {mean_h:.0} m   max {hi:.0} m   (datum SEA_LEVEL_M = {SEA_LEVEL_M:.0} m)");

    // Volume of water held if sea level were at `s`.
    let held = |s: f64| -> f64 {
        cells.iter().filter(|(h, _)| *h < s).map(|(h, a)| (s - h) * a / 1000.0).sum::<f64>() // m×km² → km³
    };
    let land_frac = |s: f64| -> f64 {
        cells.iter().filter(|(h, _)| *h > s).map(|(_, a)| *a).sum::<f64>() / total_area_km2
    };

    // --- 4. The two questions -----------------------------------------------------
    let capacity_km3 = held(hi); // every basin filled to the highest ground on the planet
    println!("\n  DECLARED ocean (hydrosphere, conserved from the ante-mundane mass fraction):");
    println!("      {ocean_km3:.4e} km³   ({:.0} m spread over the whole planet)", h2o.ocean_m_we(&planet));
    println!("  BASIN CAPACITY of the generated terrain (filled to its highest point, {hi:.0} m):");
    println!("      {capacity_km3:.4e} km³");
    println!("  ratio  declared / capacity = {:.2}×", ocean_km3 / capacity_km3);

    if ocean_km3 > capacity_km3 {
        // The basins CANNOT hold it: the planet drowns and sea level sits above the
        // highest ground. Then every cell is submerged and the level is exact:
        //   ocean = Σ (s − h)·a  ⇒  s = mean_h + ocean/area
        let s = mean_h + ocean_km3 * 1000.0 / total_area_km2;
        println!("\n  ⚠ THE DECLARED OCEAN DOES NOT FIT. The generated basins are too shallow to hold it.");
        println!("      derived sea level = {s:.0} m   →  {:.0} m ABOVE the highest ground on the planet", s - hi);
        println!("      land fraction     = 0.0%   (a total water-world)");
    } else {
        // Bisect for the level whose held volume equals the declared ocean.
        let (mut a, mut b) = (lo, hi);
        for _ in 0..80 {
            let m = 0.5 * (a + b);
            if held(m) < ocean_km3 {
                a = m;
            } else {
                b = m;
            }
        }
        let s = 0.5 * (a + b);
        println!("\n  DERIVED sea level = {s:.0} m   (vs the DECLARED datum {SEA_LEVEL_M:.0} m — a {:+.0} m correction)", s - SEA_LEVEL_M);
        println!("      land fraction at the derived level  = {:.1}%", land_frac(s) * 100.0);
    }

    println!("      land fraction at the DECLARED datum = {:.1}%", land_frac(SEA_LEVEL_M) * 100.0);
    println!("\n  early-Earth survey target (ref/research/early-continents-survey.md §6): land 2–15%");
    println!("\nWHAT THIS IS: a coherence check between two INDEPENDENTLY declared things that");
    println!("have never been reconciled — the conserved water inventory (earth-ref) and the");
    println!("surface relief (arbitrary fBm bands, ASSUMPTIONS 'continental band' ±1500 m).");
    println!("A disagreement here is a real mechanical finding, not a tuning problem.");
}

// --- spherical geometry (unit sphere) ---
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn geo(a: [f64; 3], b: [f64; 3]) -> f64 {
    dot(cross(a, b), cross(a, b)).sqrt().atan2(dot(a, b))
}
/// Spherical triangle area (steradians) — l'Huilier.
fn tri(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let (ab, bc, ca) = (geo(a, b), geo(b, c), geo(c, a));
    let s = (ab + bc + ca) / 2.0;
    let t = (s / 2.0).tan() * ((s - ab) / 2.0).tan() * ((s - bc) / 2.0).tan() * ((s - ca) / 2.0).tan();
    4.0 * t.abs().sqrt().atan()
}
