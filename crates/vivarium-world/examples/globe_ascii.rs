//! From-space ASCII globe — the seam/corner instrument (and Joseph's ask,
//! 2026-07-10: "a from-space level projection at one of the main seams /
//! cube-corners").
//!
//! Orthographic projection of the hemisphere facing the viewer, default aimed
//! straight down the (1,1,1) cube corner where three faces meet — the
//! notorious spot. Land renders as a height ramp, sea as `~`; cells where the
//! *dominant cube face changes* between adjacent pixels are overdrawn with `+`
//! (the face-boundary trace), so any terrain discontinuity that follows the
//! `+` lines is a face-seam artifact, and terrain that flows through them is
//! continuity working.
//!
//! Env: `VIVARIUM_SEED` (default 0) · `GLOBE_DIR=x,y,z` view axis (default
//! 1,1,1) · `GLOBE_W` char width (default 100) · `GLOBE_LEVEL` sampling level
//! (default 8).
//!
//! Run: `cargo run --release -p vivarium-world --example globe_ascii`

use vivarium_world::gen::{surface_prior_m, SEA_LEVEL_M};
use vivarium_world::sphere::CubeCoord;

fn env_or<T: std::str::FromStr>(name: &str, default: T) -> T {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn main() {
    let seed: u64 = env_or("VIVARIUM_SEED", 0);
    let w: usize = env_or("GLOBE_W", 100);
    let h = w / 2; // terminal cells are ~2:1
    let level: u8 = env_or("GLOBE_LEVEL", 8);
    let axis: [f64; 3] = std::env::var("GLOBE_DIR")
        .ok()
        .and_then(|s| {
            let p: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
            (p.len() == 3).then(|| [p[0], p[1], p[2]])
        })
        .unwrap_or([1.0, 1.0, 1.0]);

    // Orthonormal frame: view axis + two tangents.
    let n = normalize(axis);
    let up = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let rx = normalize(cross(up, n));
    let ry = cross(n, rx);

    println!(
        "from-space, looking down ({:.2},{:.2},{:.2}) — seed {seed}, L{level}, `+` = face boundary",
        n[0], n[1], n[2]
    );
    const RAMP: &[u8] = b" .:-=+*#%@";
    let mut faces = vec![255u8; w * h];
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
            // Disc point → world direction (orthographic: depth along +n).
            let d = [
                rx[0] * x + ry[0] * y + n[0] * z,
                rx[1] * x + ry[1] * y + n[1] * z,
                rx[2] * x + ry[2] * y + n[2] * z,
            ];
            let cc = CubeCoord::from_unit(d);
            faces[j * w + i] = cc.face.index();
            let elev = surface_prior_m(seed, cc.cell(level), level);
            let ch = if elev < SEA_LEVEL_M {
                '~'
            } else {
                let t = ((elev - SEA_LEVEL_M) / 2200.0).clamp(0.0, 1.0);
                RAMP[(t * (RAMP.len() - 1) as f64) as usize] as char
            };
            row.push(ch);
        }
        rows.push(row);
    }
    // Overdraw face boundaries: a pixel whose right/down neighbour is on a
    // different face gets `+`.
    for j in 0..h {
        let mut line: Vec<char> = rows[j].chars().collect();
        for i in 0..w {
            let f = faces[j * w + i];
            if f == 255 {
                continue;
            }
            let right = if i + 1 < w { faces[j * w + i + 1] } else { f };
            let down = if j + 1 < h { faces[(j + 1) * w + i] } else { f };
            if (right != 255 && right != f) || (down != 255 && down != f) {
                line[i] = '+';
            }
        }
        println!("{}", line.iter().collect::<String>());
    }
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / m, v[1] / m, v[2] / m]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
