//! **Strengthen-before-soften on `#coarse-only-drainage-closure-is-a-nogo`.**
//!
//! The archived spike (`.super-archive/from-msc/spike-nonlocal-closure/`) measured a
//! ceiling R² <= 0.359 for the specific family `max(A_fine) ~= alpha * A_coarse^beta`
//! (log-log OLS) across 12 (tile, depth) cells, two seeds, and the segment/DECISIONS
//! text generalises that to "no POINTWISE coarse-only closure" — i.e. every monotone
//! function of A_coarse alone, not just the power-law family. A log-log OLS R² is a
//! LOWER bound on the best-achievable R2 over the class of all monotone functions,
//! not an upper bound, so the generalisation was never actually tested. This probe
//! computes the honest ceiling over that larger class directly: isotonic regression
//! (PAVA) is the closed-form least-squares argmin over ALL non-decreasing functions,
//! so its R2 on the same data is the ceiling the segment's claim actually needs.
//!
//! Isotonic is chosen over a binned correlation ratio eta^2 because it needs no bin-
//! width choice — eta^2's value depends on binning granularity in a way that would
//! reopen exactly the kind of "was the probe sensitive enough" question this probe
//! exists to close (`#norm-probe-sensitivity`).
//!
//! Reuses the exact 12 (tile, depth) footprints named in the spike's `RUN.txt` PROBE 8
//! table (seed 0: three XPos tiles; seed 1: three more XPos tiles; depths 1 and 2 each)
//! so the comparison is apples-to-apples with the ceiling already in DECISIONS/canon.
//! The archived spike's own `mra` restriction module doesn't compile against current
//! main, so the coarse initial condition here is a direct area-weighted block mean of
//! the fine initial topography — which is what `mra::decompose(...).root.v` reduces to
//! for the coarse SCALING coefficient specifically (the spike's own module doc: the
//! predictor choice only ever affects detail/compression, never the coarse value, so
//! this is not an approximation of that step, it's the same formula taken directly).
//!
//! Read-only against the generator; writes nothing, opens no world store.
//!
//! Run: `cargo run --release -p vivarium-world --example monotone_ceiling_probe`

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::gen;
use vivarium_world::measure::cell_area_m2;
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CellId, Face};

const LEVEL: u8 = 19;
const NX: usize = 128;
const RADIUS_M: f64 = Planet::EARTH.radius_m;

/// The exact 12 (tile, depth) cells from `RUN.txt` PROBE 8 — the same footprints the
/// power-law ceiling (0.359) was measured on.
const TILES: &[(u64, Face, u64, u64, &str)] = &[
    (0, Face::XPos, 327_680, 245_760, "XPos(327680,245760)"),
    (0, Face::XPos, 311_296, 262_144, "XPos(311296,262144)"),
    (0, Face::XPos, 344_064, 278_528, "XPos(344064,278528)"),
    (1, Face::XPos, 229_376, 49_152, "XPos(229376,49152)"),
    (1, Face::XPos, 245_760, 49_152, "XPos(245760,49152)"),
    (1, Face::XPos, 278_528, 131_072, "XPos(278528,131072)"),
];

/// Area-weighted block mean of a fine field down to a `ratio x ratio` coarsening —
/// exactly the coarse scaling coefficient the archived `mra::decompose` produced,
/// computed directly rather than through the (now non-compiling) lifting-scheme code.
fn area_weighted_coarsen(face: Face, oi: u64, oj: u64, level: u8, fine: &[f64], ratio: usize, cnx: usize) -> Vec<f64> {
    let mut out = vec![0.0f64; cnx * cnx];
    for cy in 0..cnx {
        for cx in 0..cnx {
            let mut num = 0.0f64;
            let mut den = 0.0f64;
            for dy in 0..ratio {
                for dx in 0..ratio {
                    let gi = oi + (cx * ratio + dx) as u64;
                    let gj = oj + (cy * ratio + dy) as u64;
                    let a = cell_area_m2(face, gi, gj, level, RADIUS_M);
                    let v = fine[(cy * ratio + dy) * (cnx * ratio) + (cx * ratio + dx)];
                    num += a * v;
                    den += a;
                }
            }
            out[cy * cnx + cx] = num / den;
        }
    }
    out
}

/// `A_coarse` (the coarse-only-computable drainage a coarse-resolution run would
/// itself produce) and the fine trunk `max(A_fine)` per coarse cell, for one
/// (seed, tile, depth) — the same statistical object `tile_fields_seeded` computed.
fn tile_pairs(seed: u64, face: Face, oi: u64, oj: u64, depth: usize) -> (Vec<f64>, Vec<f64>) {
    let ratio = 1usize << depth;
    let cnx = NX >> depth;
    let clevel = LEVEL - depth as u8;
    let (coi, coj) = (oi >> depth, oj >> depth);

    let mut h0 = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            let c = CellId::from_face_ij(face, (oi + x) as u32, (oj + y) as u32, LEVEL);
            h0.push(gen::initial_topography_m(seed, c, LEVEL));
        }
    }

    // fine run -> trunk (max) per coarse cell
    let mut ff = Fluvial::from_prior(seed, face, LEVEL, oi as u32, oj as u32, NX);
    ff.erode(&FluvialParams::default());
    let mut max_af = vec![0.0f64; cnx * cnx];
    for cy in 0..cnx {
        for cx in 0..cnx {
            let mut m = 0.0f64;
            for dy in 0..ratio {
                for dx in 0..ratio {
                    m = m.max(ff.drainage[(cy * ratio + dy) * NX + (cx * ratio + dx)] as f64);
                }
            }
            max_af[cy * cnx + cx] = m;
        }
    }

    // coarse baseline run -> its own drainage
    let r_h0 = area_weighted_coarsen(face, oi, oj, LEVEL, &h0, ratio, cnx);
    let mut cf = Fluvial::from_surface(seed, face, clevel, coi as u32, coj as u32, cnx, |c| {
        let (_, i, j, _) = c.to_face_ij();
        let x = (i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize;
        let y = (j as i64 - coj as i64).clamp(0, cnx as i64 - 1) as usize;
        r_h0[y * cnx + x]
    });
    cf.erode(&FluvialParams::default());
    let a_coarse: Vec<f64> = cf.drainage.iter().map(|&x| x as f64).collect();
    (a_coarse, max_af)
}

/// Log-log least-squares fit `max(A_fine) ~= alpha * A_coarse^beta` -> R2. Reproduced
/// from the archived spike's `loglog_fit` verbatim (same filter, same formula) so the
/// number printed here is checkable against `RUN.txt` PROBE 8 on the same footprints.
fn loglog_r2(a_coarse: &[f64], max_af: &[f64]) -> f64 {
    let (mut lx, mut ly) = (Vec::new(), Vec::new());
    for (&x, &y) in a_coarse.iter().zip(max_af) {
        if x > 0.0 && y > 0.0 {
            lx.push(x.ln());
            ly.push(y.ln());
        }
    }
    let n = lx.len() as f64;
    let mx = lx.iter().sum::<f64>() / n;
    let my = ly.iter().sum::<f64>() / n;
    let sxy: f64 = lx.iter().zip(&ly).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sxx: f64 = lx.iter().map(|a| (a - mx).powi(2)).sum();
    let syy: f64 = ly.iter().map(|b| (b - my).powi(2)).sum();
    (sxy * sxy) / (sxx * syy)
}

/// Pool-adjacent-violators: the least-squares projection onto non-decreasing
/// sequences. `y` must already be ordered by ascending `x`. Standard algorithm.
fn pava(y: &[f64]) -> Vec<f64> {
    struct Block {
        val: f64,
        w: f64,
        n: usize,
    }
    let mut blocks: Vec<Block> = Vec::new();
    for &yi in y {
        let mut b = Block { val: yi, w: 1.0, n: 1 };
        while let Some(last) = blocks.last() {
            if last.val > b.val {
                let prev = blocks.pop().unwrap();
                let nw = prev.w + b.w;
                let nval = (prev.val * prev.w + b.val * b.w) / nw;
                b = Block { val: nval, w: nw, n: prev.n + b.n };
            } else {
                break;
            }
        }
        blocks.push(b);
    }
    let mut out = Vec::with_capacity(y.len());
    for b in blocks {
        for _ in 0..b.n {
            out.push(b.val);
        }
    }
    out
}

/// Isotonic-regression R2: the ceiling over ALL non-decreasing functions of x, on
/// the same (x>0, y>0)-filtered point set the log-log fit uses.
fn isotonic_r2(a_coarse: &[f64], max_af: &[f64]) -> (f64, usize) {
    let mut pts: Vec<(f64, f64)> =
        a_coarse.iter().zip(max_af).filter(|(&x, &y)| x > 0.0 && y > 0.0).map(|(&x, &y)| (x, y)).collect();
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.partial_cmp(&b.1).unwrap()));
    let n = pts.len();
    let y: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let fit = pava(&y);
    let my = y.iter().sum::<f64>() / n as f64;
    let sse: f64 = y.iter().zip(&fit).map(|(a, b)| (a - b).powi(2)).sum();
    let sst: f64 = y.iter().map(|a| (a - my).powi(2)).sum();
    (1.0 - sse / sst, n)
}

fn main() {
    println!("== monotone_ceiling_probe — strengthen-first attempt on the coarse-only closure no-go ==\n");
    println!("Pre-registered prediction (see kick-coarse-only-nogo.md addendum, written before this run):");
    println!("  isotonic ceiling expected in ~0.4-0.5; <0.4-0.5 earns the broad claim, near 0.359 means");
    println!("  the power law already WAS the ceiling, well above 0.5 falsifies the current claim.\n");

    println!(
        "{:<4} {:<20} {:>3} {:>6} {:>10} {:>6}",
        "seed", "tile", "dep", "n", "loglog_R2", "iso_R2"
    );
    let mut worst_loglog = 0.0f64;
    let mut worst_iso = 0.0f64;
    let mut rows: Vec<(u64, &str, usize, usize, f64, f64)> = Vec::new();
    for &(seed, face, oi, oj, name) in TILES {
        for depth in [1usize, 2] {
            let (ac, maf) = tile_pairs(seed, face, oi, oj, depth);
            let r2_ll = loglog_r2(&ac, &maf);
            let (r2_iso, n) = isotonic_r2(&ac, &maf);
            println!("{seed:<4} {name:<20} {depth:>3} {n:>6} {r2_ll:>10.3} {r2_iso:>6.3}");
            worst_loglog = worst_loglog.max(r2_ll);
            worst_iso = worst_iso.max(r2_iso);
            rows.push((seed, name, depth, n, r2_ll, r2_iso));
        }
    }
    println!("\n  power-law ceiling (max over 12 cells) = {worst_loglog:.3}  (RUN.txt PROBE 8 reported 0.359)");
    println!("  GENERAL-MONOTONE ceiling (max isotonic R2 over the same 12 cells) = {worst_iso:.3}");
    println!(
        "\n  gap (iso - loglog), per cell, largest first: sorted below to see where the two disagree most:"
    );
    let mut gaps: Vec<(f64, &str, usize)> = rows.iter().map(|r| (r.5 - r.4, r.1, r.2)).collect();
    gaps.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (g, name, dep) in gaps {
        println!("    {name:<20} dep {dep}  gap {g:>6.3}");
    }
}
