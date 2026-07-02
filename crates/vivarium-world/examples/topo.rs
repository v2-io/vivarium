//! ASCII topography of a face region — the GPU-free world diagnostic. Separates
//! *world* issues (is the terrain actually varied/sloped here?) from *explorer*
//! issues (is the renderer failing to show it?).
//!
//!   cargo run --release -p vivarium-world --example topo [level] [ci] [cj] [span_cells]
//!
//! Prints an elevation-banded character map of the window centred at (ci, cj) in
//! face cells at `level`, plus the numbers that matter: elevation range and the
//! mean/max adjacent-cell slope (slope is what makes terrain *read* — a world can
//! have km of relief and still be visually flat if it's spread over 1000 km).

use vivarium_world::gen::{baseline_column, SEA_LEVEL_M};
use vivarium_world::sample::cell_size_m;
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CellId, Face};

const COLS: usize = 96;
const ROWS: usize = 40;

fn main() {
    let arg = |k: usize, d: f64| std::env::args().nth(k).and_then(|s| s.parse().ok()).unwrap_or(d);
    let level = arg(1, 14.0) as u8;
    let n = 1u64 << level;
    let ci = arg(2, n as f64 * 0.5) as i64;
    let cj = arg(3, n as f64 * 0.5) as i64;
    let span = arg(4, 256.0) as i64; // window width in cells

    let cell_m = cell_size_m(level, Planet::EARTH.radius_m);
    let (si, sj) = (span.max(COLS as i64) / COLS as i64, span.max(ROWS as i64) / ROWS as i64);

    // Sample the character grid + gather stats on the full-resolution centre row
    // (adjacent cells, not the character stride, so slope is the real cell slope).
    let h_at = |i: i64, j: i64| -> f64 {
        let i = i.clamp(0, n as i64 - 1) as u32;
        let j = j.clamp(0, n as i64 - 1) as u32;
        baseline_column(CellId::from_face_ij(Face::ZPos, i, j, level)).solid_thickness_m() - SEA_LEVEL_M
    };

    let mut grid = vec![0.0f64; COLS * ROWS];
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for r in 0..ROWS {
        for c in 0..COLS {
            let i = ci - span / 2 + c as i64 * si;
            let j = cj - span / 2 + r as i64 * sj;
            let h = h_at(i, j);
            grid[r * COLS + c] = h;
            lo = lo.min(h);
            hi = hi.max(h);
        }
    }

    // Real adjacent-cell slopes along the centre row.
    let (mut slope_sum, mut slope_max, mut cnt) = (0.0f64, 0.0f64, 0u32);
    let j = cj;
    let mut prev = h_at(ci - span / 2, j);
    for k in 1..span {
        let h = h_at(ci - span / 2 + k, j);
        let s = (h - prev).abs() / cell_m;
        slope_sum += s;
        slope_max = slope_max.max(s);
        prev = h;
        cnt += 1;
    }

    // Character bands: sea by depth, land by elevation.
    let band = |h: f64| -> char {
        if h < -200.0 { ' ' }
        else if h < -50.0 { '.' }
        else if h < 0.0 { '~' }
        else if h < 50.0 { '_' }
        else if h < 200.0 { '-' }
        else if h < 450.0 { '=' }
        else if h < 700.0 { '+' }
        else if h < 950.0 { '*' }
        else if h < 1200.0 { '#' }
        else { '@' }
    };

    println!(
        "topo ZPos L{level} centre ({ci},{cj}) window {span} cells = {:.1} km   cell {:.0} m",
        span as f64 * cell_m / 1000.0,
        cell_m
    );
    for r in 0..ROWS {
        let line: String = (0..COLS).map(|c| band(grid[r * COLS + c])).collect();
        println!("{line}");
    }
    println!("bands: ' 'deep  '.'sea  '~'shallow  '_'shore  '-'low  '='mid  '+'high  '*'higher  '#'peak  '@'summit");
    println!(
        "elev {:.0}..{:.0} m   adjacent-cell slope mean {:.2}% max {:.2}%  (centre row, {} cells)",
        lo,
        hi,
        100.0 * slope_sum / cnt as f64,
        100.0 * slope_max,
        cnt
    );
}
