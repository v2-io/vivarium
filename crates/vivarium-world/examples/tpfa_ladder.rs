//! # TPFA convergence ladder — is the grid report's −0.50 a growing error or a plateau?
//!
//! `grid_lab` §7 prints the two-point-flux (TPFA) convergence order from **exactly one
//! pair**, N=32 → N=64. Two planar controls in `gcl_abf_probe` (A3, A4) say the two-point
//! flux converges to the *wrong operator* — an O(1) plateau — and that the largest negative
//! orders appear precisely in the coarse regime, from approaching that plateau from below.
//!
//! This binary re-uses `grid_lab`'s own mesh builders and probes **without modifying them**
//! (`#[path]` module inclusion, so the shared instrument is untouched) and extends the
//! ladder. Prediction, pre-registered in
//! `msc/agent-briefs/2026-07-29-gcl-and-abf-checks.md` §0 (A5): the order climbs toward 0
//! and the error plateaus.
//!
//! Run: `cargo run --release -p vivarium-world --example tpfa_ladder`

#[path = "grid_lab/mesh.rs"]
mod mesh;
#[path = "grid_lab/grids.rs"]
mod grids;
#[path = "grid_lab/healpix.rs"]
mod healpix;
#[path = "grid_lab/icosa.rs"]
mod icosa;
#[path = "grid_lab/flow.rs"]
mod flow;
#[path = "grid_lab/probes.rs"]
mod probes;

use grids::*;
use probes::*;
use vivarium_world::planet::Planet;

fn r() -> f64 { Planet::EARTH.radius_m }

fn main() {
    println!("# TPFA ladder — grid_lab's own scheme and meshes, more rungs\n");
    println!("Prediction (pre-registered): TPFA error plateaus at O(1); order → 0.");
    println!("If the order stays near −0.5 the error genuinely grows and that is refuted.\n");
    println!(
        "{:>7} {:>11} {:>14} {:>9} {:>14} {:>9}",
        "N/face", "cells", "TPFA rel L2", "order", "LSQ rel L2", "order"
    );
    let mut prev: Option<(f64, f64)> = None;
    for n in [16usize, 32, 64, 128, 256] {
        let g = cube_sphere(CubeProj::Equiangular, n, r());
        let (tp, _) = harmonic_error(&g, Scheme::FvCentreLine, 2);
        let (lsq, _) = harmonic_error(&g, Scheme::FvLsq, 2);
        let (o1, o2) = match prev {
            Some((p, q)) => ((p / tp).log2(), (q / lsq).log2()),
            None => (f64::NAN, f64::NAN),
        };
        println!("{n:>7} {:>11} {tp:>14.6e} {o1:>9.3} {lsq:>14.6e} {o2:>9.3}", g.cells());
        prev = Some((tp, lsq));
    }
    println!("\n(the N=32 → N=64 rung is the one grid_lab §7 reports as −0.50)");
    split_by_panel_distance();
    seam_edge_metrics();
}

/// Where does the growing error live?
///
/// The order is a suspiciously clean −0.498. An error of size O(1/h) concentrated on a
/// **codimension-1** set (area fraction ~ h) contributes `sqrt(h · (1/h)²) = h^{-1/2}` to a
/// whole-sphere L2 norm — i.e. **exactly −0.5**. The 12 cube-panel seams are such a set.
/// `gcl_abf_probe` A5 measured the face *interior* alone and found an O(1) **plateau**, so
/// the growth is not in the bulk.
///
/// Split the same L2 by distance from the nearest panel edge, in units of cell width, and
/// watch each band separately. A cell's panel coordinates come from normalising by its
/// largest |component| (the gnomonic cube face); boundary distance is `1 − max(|a|,|b|)`,
/// and `N/2 ·` that is roughly "how many cells from the seam".
fn split_by_panel_distance() {
    println!("\n## Where the growing error lives — L2 split by distance from a panel seam");
    println!("   band = cells from the nearest cube-panel edge (0 = touching the seam)");
    println!(
        "{:>7} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "N/face", "band 0", "band 1-2", "band 3-8", "band 9-32", "band >32"
    );
    let bands = [(0usize, 1usize), (1, 3), (3, 9), (9, 33), (33, usize::MAX)];
    let mut prev: Option<Vec<f64>> = None;
    for n in [32usize, 64, 128, 256] {
        let g = cube_sphere(CubeProj::Equiangular, n, r());
        let e = mesh::unit([0.3, -0.7, 0.64]);
        let f = |p: [f64; 3]| -> f64 {
            let t = mesh::dot(p, e);
            1.5 * t * t - 0.5
        };
        let lam = -6.0 / (g.radius_m * g.radius_m);
        let u: Vec<f64> = g.centers.iter().map(|&p| f(p)).collect();
        let du = probes::laplacian(&g, Scheme::FvCentreLine, &u, probes::means(&g));
        let mut num = vec![0.0f64; bands.len()];
        let mut den = vec![0.0f64; bands.len()];
        for i in 0..g.cells() {
            let p = g.centers[i];
            let m = p[0].abs().max(p[1].abs()).max(p[2].abs());
            // the two non-dominant components, normalised — panel coordinates in [-1, 1]
            let mut o: Vec<f64> = (0..3).filter(|&k| p[k].abs() < m - 1e-12).map(|k| p[k] / m).collect();
            o.resize(2, 0.0);
            let edge_dist = 1.0 - o[0].abs().max(o[1].abs());
            // face parameter is (2/π)·atan of the panel coordinate for equiangular; the cell
            // width in that coordinate is ~2/N near the centre, so cells-from-seam ≈ N/2 · dist
            let cells_from_seam = (edge_dist * n as f64 * 0.5).floor().max(0.0) as usize;
            let b = bands.iter().position(|&(lo, hi)| cells_from_seam >= lo && cells_from_seam < hi).unwrap();
            let exact = lam * u[i];
            let err = du[i] - exact;
            num[b] += err * err * g.areas[i];
            den[b] += exact * exact * g.areas[i];
        }
        // Each band's contribution to the GLOBAL relative L2 (shared denominator), so the
        // columns are comparable and sum in quadrature to the total.
        let dtot: f64 = den.iter().sum();
        let vals: Vec<f64> = num.iter().map(|&x| (x / dtot).sqrt()).collect();
        print!("{n:>7}");
        for v in &vals {
            print!(" {v:>12.4e}");
        }
        println!();
        if let Some(p) = &prev {
            print!("{:>7}", "order");
            for (a, b) in p.iter().zip(vals.iter()) {
                print!(" {:>12.3}", (a / b).log2());
            }
            println!();
        }
        prev = Some(vals);
    }
    // Calibration note, corrected after the first run: the expected band-0 *contribution*
    // order is −0.5, not −1. A local error of size 1/h on a set of area fraction ~2/N
    // contributes sqrt(h)·(1/h) = h^{-1/2} to the global norm. The local error is recovered
    // by dividing the band's contribution by sqrt(2/N) — do that and it doubles per rung.
    println!("   band 0 ≈ −0.5 while every interior band is POSITIVE ⇒ the growth is a SEAM");
    println!("   defect on a 1-D set; the bulk of the face converges under the same scheme.");
    println!("   local band-0 error = contribution / sqrt(2/N) — doubles per rung, i.e. O(1/h).");
}

/// Is the band-0 blow-up a real geometry effect or a harness artifact?
///
/// A 1/h *relative* error at the seam is much worse than non-orthogonality can explain
/// (that is an O(1) effect), so before this is reported as a property of the grid it has
/// to be checked against the seam edges' own metrics. Print, for the worst band-0 edge and
/// for a face-centre edge, the quantities the two-point flux actually consumes.
fn seam_edge_metrics() {
    println!("\n## Seam-edge metrics — is the band-0 blow-up geometry or harness?");
    println!(
        "{:>7}  {:>10} {:>12} {:>12} {:>10} {:>10} {:>12}",
        "N/face", "where", "edge_len/h", "dist/h", "nonortho°", "skew", "L/d"
    );
    for n in [32usize, 64, 128, 256] {
        let g = cube_sphere(CubeProj::Equiangular, n, r());
        let h = (4.0 * std::f64::consts::PI * g.radius_m * g.radius_m / g.cells() as f64).sqrt();
        // classify cells by distance from a panel seam, as above
        let seam_of = |p: [f64; 3]| -> usize {
            let m = p[0].abs().max(p[1].abs()).max(p[2].abs());
            let mut o: Vec<f64> = (0..3).filter(|&k| p[k].abs() < m - 1e-12).map(|k| p[k] / m).collect();
            o.resize(2, 0.0);
            ((1.0 - o[0].abs().max(o[1].abs())) * n as f64 * 0.5).floor().max(0.0) as usize
        };
        let mut worst: Option<(f64, usize, usize)> = None; // (nonortho, i, edge idx)
        let mut ctr: Option<(usize, usize)> = None;
        for i in 0..g.cells() {
            let d = seam_of(g.centers[i]);
            for (k, e) in g.adj[i].iter().enumerate() {
                if d == 0 && seam_of(g.centers[e.j]) == 0 {
                    if worst.is_none_or(|(w, _, _)| e.nonortho_deg > w) {
                        worst = Some((e.nonortho_deg, i, k));
                    }
                }
                if d > n / 4 && ctr.is_none() {
                    ctr = Some((i, k));
                }
            }
        }
        for (label, pick) in [("seam", worst.map(|(_, i, k)| (i, k))), ("interior", ctr)] {
            if let Some((i, k)) = pick {
                let e = &g.adj[i][k];
                println!(
                    "{n:>7}  {label:>10} {:>12.5} {:>12.5} {:>10.3} {:>10.4} {:>12.5}",
                    e.edge_len_m / h,
                    e.dist_m / h,
                    e.nonortho_deg,
                    e.skew,
                    e.edge_len_m / e.dist_m
                );
            }
        }
    }
    println!("   metrics comparable to the interior ⇒ the blow-up is NOT bad edge geometry,");
    println!("   and the next suspect is the cross-face adjacency itself.");
}
