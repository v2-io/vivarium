//! Watching principled land-masses rise from the sea — the mantle-thermal
//! cooling chain rendered as a from-space ASCII globe sequence.
//!
//! Same orthographic projection as `globe_ascii`, but walked across the Abyssal
//! cooling epochs (`mantle_thermal::cooling_stages`). Each frame re-pours the
//! ocean against the freeboard the epoch's mantle temperature earns, so you see
//! the ordinum ladder run: hot water-world → transient stands → growing
//! cratons. Land is a height ramp; sea is `~`. This is a *view* (core/view
//! wall) — it reads the same law the segment claims, renders nothing new.
//!
//! Env: `VIVARIUM_SEED` (default 0) · `GLOBE_DIR=x,y,z` (default 1,1,1) ·
//! `GLOBE_W` char width (default 72) · `GLOBE_LEVEL` sampling level (default 7).
//!
//! Run: `cargo run --release -p vivarium-world --example emerging_land`

use vivarium_world::mantle_thermal::{cooling_stages, potential_temp_c, present_abyssal};
use vivarium_world::sea_level::{derived_sea_level_at_tp, tectonic_surface_at_tp};
use vivarium_world::sphere::CubeCoord;

fn env_or<T: std::str::FromStr>(name: &str, default: T) -> T {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn main() {
    let seed: u64 = env_or("VIVARIUM_SEED", 0);
    let w: usize = env_or("GLOBE_W", 72);
    let h = w / 2;
    let level: u8 = env_or("GLOBE_LEVEL", 7);
    let axis: [f64; 3] = std::env::var("GLOBE_DIR")
        .ok()
        .and_then(|s| {
            let p: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
            (p.len() == 3).then(|| [p[0], p[1], p[2]])
        })
        .unwrap_or([1.0, 1.0, 1.0]);

    let n = normalize(axis);
    let up = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let rx = normalize(cross(up, n));
    let ry = cross(n, rx);

    let present = present_abyssal();
    println!(
        "emerging land — seed {seed}, L{level}, looking down ({:.2},{:.2},{:.2}). \
         The mantle cools top→bottom; land rises as basins deepen.\n",
        n[0], n[1], n[2]
    );

    const RAMP: &[u8] = b".:-=+*#%@";
    for t in cooling_stages() {
        let tp = potential_temp_c(t);
        let age_ga = -t.years() / 1.0e9;
        let sea = derived_sea_level_at_tp(seed, tp);
        let mut land_cells = 0usize;
        let mut disc_cells = 0usize;
        let mut rows = Vec::with_capacity(h);
        for j in 0..h {
            let mut row = String::with_capacity(w);
            for i in 0..w {
                let x = (i as f64 + 0.5) / w as f64 * 2.0 - 1.0;
                let y = (j as f64 + 0.5) / h as f64 * 2.0 - 1.0;
                let r2 = x * x + y * y;
                if r2 >= 1.0 {
                    row.push(' ');
                    continue;
                }
                let z = (1.0 - r2).sqrt();
                let d = [
                    rx[0] * x + ry[0] * y + n[0] * z,
                    rx[1] * x + ry[1] * y + n[1] * z,
                    rx[2] * x + ry[2] * y + n[2] * z,
                ];
                let cc = CubeCoord::from_unit(d);
                let elev = tectonic_surface_at_tp(seed, cc.cell(level), level, tp);
                disc_cells += 1;
                let ch = if elev < sea {
                    '~'
                } else {
                    land_cells += 1;
                    let f = ((elev - sea) / 2200.0).clamp(0.0, 1.0);
                    RAMP[(f * (RAMP.len() - 1) as f64) as usize] as char
                };
                row.push(ch);
            }
            rows.push(row);
        }
        let frac = if disc_cells > 0 { land_cells as f64 / disc_cells as f64 * 100.0 } else { 0.0 };
        let tag = if t == present { "  <- present-Abyssal" } else { "" };
        println!("t = {age_ga:.2} Ga   T_p = {tp:.0} C   sea = {sea:.0} m   disc-land ≈ {frac:.1}%{tag}");
        for r in &rows {
            println!("{r}");
        }
        println!();
    }
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / m, v[1] / m, v[2] / m]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
