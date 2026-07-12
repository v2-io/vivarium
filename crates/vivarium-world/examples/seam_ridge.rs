//! The differential-aging ridge probe (DESIGN-REDUX §2b; Joseph 2026-07-03).
//!
//! Where a fine tier has run erosion epochs the terrain beyond its border has
//! not, a physically-implausible ridge or trench can form along the seam.
//! Nature's invariant: walking across a hillside, the curvature statistics
//! should not know where our tier boundary is. This probe builds the same
//! macro→fine telescope worldview uses, but with the fine tier covering only
//! the CENTRE of the macro footprint — so the fine edge falls mid-hillside —
//! then samples the composed surface along transects crossing that edge and
//! compares |second difference| (curvature) in the seam band vs the interior.
//!
//! ratio ≈ 1  → seamless. ratio >> 1 → the seam is visible as a ridge/trench.
//! This probe is expected to FAIL (ratio well above 1) as of 2026-07-03 —
//! it exists to make that failure measured, and to gate the future fix.
use vivarium_world::erosion::{self, Fluvial, FluvialParams};
use vivarium_world::sphere::{CellId, Face};

fn main() {
    // Sweep the AGE GAP: worldview's standard fine pass (18 epochs) up through
    // heavy telescope lingering (150). The pin should hold the seam at small
    // gaps; the question is where (whether) differential aging breaks it.
    for epochs in [18u32, 60, 150] {
        run(epochs);
    }
}

fn run(fine_epochs: u32) {
    let face = Face::ZPos;
    // SUBAERIAL footprint — load-bearing. This probe reported "SEAM RIDGE RATIO
    // 22888" for months. That number was `0 ÷ 1e-9`: the old footprint
    // (165_800, 413_600) sits at 3709-3715 m, entirely BELOW sea level (4000), so
    // every cell was an outlet, erosion no-op'd, the interior curvature was exactly
    // zero, and the ratio was a divide-by-zero against the epsilon floor. The tell
    // was printed all along — the ratio was bit-identical across every age gap
    // swept. THE SEAM HAD NEVER ACTUALLY BEEN MEASURED. This region is verified
    // land (relief 5072-5216 m), where the fluvial kernel actually executes.
    let (oi, oj, nx) = (108_500u32, 186_350u32, 128usize);
    let p = FluvialParams::default();

    // Macro tier (L19), fully eroded.
    let mut macro_t = Fluvial::from_prior(0, face, 19, oi, oj, nx);
    macro_t.erode(&p);
    let macro_r = macro_t.to_region();

    // Fine tier (L21) over only the CENTRE HALF of the macro footprint, then
    // mean-pinned — the fine edge falls mid-hillside.
    let fine_nx = nx; // half the macro span at 4x the resolution
    let (ci, cj) = ((oi + nx as u32 / 2) * 4, (oj + nx as u32 / 2) * 4);
    let fp = FluvialParams { epochs: fine_epochs, ..FluvialParams::default() };
    let mut fine_f = Fluvial::from_surface(0, face, 21, ci - fine_nx as u32 / 2, cj - fine_nx as u32 / 2, fine_nx, |c| erosion::surface_at(0, c, std::slice::from_ref(&macro_r)));
    fine_f.erode(&fp);
    fine_f.pin_block_means(19, |c| erosion::surface_at(0, c, std::slice::from_ref(&macro_r)));
    // ORDER MATTERS: surface_at expects coarse -> fine (it walks from the end).
    let tiers = vec![macro_r, fine_f.to_region()];


    // Transects at L21 resolution crossing the fine tier's WEST edge.
    let edge_i = tiers[1].oi; // west boundary in L21 cells
    let (j0, j1) = (tiers[1].oj + 8, tiers[1].oj + fine_nx as u32 - 8);
    let half = 24i64; // cells sampled each side of the edge
    let mut seam = Vec::new();
    let mut interior = Vec::new();
    let mut profile_sum = vec![0.0f64; (2 * half) as usize];
    let mut transects = 0u32;
    for j in (j0..j1).step_by(3) {
        let h: Vec<f64> = (-half..half)
            .map(|di| {
                let i = (edge_i as i64 + di).max(0) as u32;
                erosion::surface_at(0, CellId::from_face_ij(face, i, j, 21), &tiers)
            })
            .collect();
        for (k, hv) in h.iter().enumerate() {
            profile_sum[k] += hv;
        }
        transects += 1;
        for k in 1..h.len() - 1 {
            let curv = (h[k - 1] - 2.0 * h[k] + h[k + 1]).abs();
            let di = k as i64 - half;
            if (-2..=2).contains(&di) {
                seam.push(curv);
            } else if di.abs() > 6 {
                interior.push(curv);
            }
        }
    }
    let med = |v: &mut Vec<f64>| {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[v.len() / 2]
    };
    let (mut seam, mut interior) = (seam, interior);
    let (ms, mi) = (med(&mut seam), med(&mut interior));
    let ratio = ms / mi.max(1e-9);
    println!("fine {fine_epochs}e — transects {transects}  median |curvature|: seam {:.3} m  interior {:.3} m", ms, mi);
    println!("SEAM RIDGE RATIO: {ratio:.2}  ({})", if ratio < 1.5 { "seamless enough" } else { "the seam is visible — a ridge/trench lives at the tier boundary" });
    // Mean cross-seam profile (relative to its own mean) — the ridge's shape.
    let n = transects as f64;
    let prof: Vec<f64> = profile_sum.iter().map(|s| s / n).collect();
    let pm = prof.iter().sum::<f64>() / prof.len() as f64;
    let bars: String = prof.iter().map(|h| {
        let x = (((h - pm) / 3.0 + 0.5) * 8.0) as isize;
        [' ', '.', ':', '-', '=', '+', '*', '#', '@'][x.clamp(0, 8) as usize]
    }).collect();
    println!("mean profile across seam (edge at centre): [{bars}]");
}
