//! The probes. Each is written so it *could* return the answer I do not want.
//!
//! `R` = restriction (area-weighted low-pass, the wavelet store's conservative
//! coarse-graining, depth 2: L19 128² → L17 32²). `E` = the REAL fluvial kernel.
//! The open problem: `E∘R ≠ R∘E`, and the gap is not carried by any local
//! statistic. Hypothesis: it is carried by the drainage graph `A`.

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::gen;
use vivarium_world::sphere::CellId;

use crate::area::cell_area_m2;
use crate::mra::{self, Bilinear, Grid};
use crate::{FACE, LEVEL, NX, OI, OJ, RADIUS_M};

const SEED: u64 = 0;
const DEPTH: usize = 2; // L19 → L17

/// Scan for a strongly-eroding LAND tile on the CURRENT generator (the world moved
/// since the wavelet spike's footprint was recorded). Tries quadtree-aligned 128²
/// L19 tiles across all six faces; reports the best by (land fraction, then relief).
pub fn scan_for_land() {
    use vivarium_world::sea_level::derived_sea_level_m;
    use vivarium_world::sphere::Face;
    let sea = derived_sea_level_m(SEED);
    println!("derived sea level (seed {SEED}) = {sea:.1} m");
    let faces = [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZPos, Face::ZNeg];
    let n = 1u64 << LEVEL;
    let step = (n / 8) & !(NX as u64 - 1); // 8 samples/side, quadtree-aligned
    let mut best: Option<(f64, f64, Face, u64, u64)> = None;
    for face in faces {
        let mut oj = 0;
        while oj + NX as u64 <= n {
            let mut oi = 0;
            while oi + NX as u64 <= n {
                // Sub-sample the tile (every 8th cell) for land fraction + relief.
                let (mut land, mut cnt, mut lo, mut hi) = (0u32, 0u32, f64::MAX, f64::MIN);
                for y in (0..NX as u64).step_by(8) {
                    for x in (0..NX as u64).step_by(8) {
                        let c = CellId::from_face_ij(face, (oi + x) as u32, (oj + y) as u32, LEVEL);
                        let h = gen::initial_topography_m(SEED, c, LEVEL);
                        if h > sea {
                            land += 1;
                        }
                        cnt += 1;
                        lo = lo.min(h);
                        hi = hi.max(h);
                    }
                }
                let frac = land as f64 / cnt as f64;
                let relief = hi - lo;
                let score = frac + relief / 1000.0;
                let interior = oi >= NX as u64 && oj >= NX as u64 && oi + 2 * NX as u64 <= n && oj + 2 * NX as u64 <= n;
                if frac >= 1.0 && relief > 150.0 && interior {
                    println!("  cand {face:?} ({oi},{oj}) land {:.0}% relief {relief:.0} m", frac * 100.0);
                }
                if best.map_or(true, |(bs, _, _, _, _)| score > bs) {
                    best = Some((score, frac, face, oi, oj));
                }
                oi += step;
            }
            oj += step;
        }
    }
    if let Some((_, frac, face, oi, oj)) = best {
        println!("\nBEST: {face:?} origin ({oi}, {oj})  land {:.0}%", frac * 100.0);
    }
}
const M_EXP: f64 = 0.5; // stream-power area exponent (live)

// ── footprint helpers ────────────────────────────────────────────────────────

fn clevel() -> u8 {
    LEVEL - DEPTH as u8
}
fn cnx() -> usize {
    NX >> DEPTH
}
fn coi() -> u64 {
    OI >> DEPTH
}
fn coj() -> u64 {
    OJ >> DEPTH
}
const RATIO: usize = 1 << DEPTH; // fine cells per coarse cell, per side (=4)

/// The raw prior surface (band-limited initial topography) as a `Grid` at L19.
fn prior_grid() -> Grid {
    let mut v = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            let c = CellId::from_face_ij(FACE, (OI + x) as u32, (OJ + y) as u32, LEVEL);
            v.push(gen::initial_topography_m(SEED, c, LEVEL));
        }
    }
    Grid::new(FACE, LEVEL, OI, OJ, NX, v)
}

/// Fine kernel, freshly seeded from the prior at L19 (unrun).
fn fine_kernel() -> Fluvial {
    Fluvial::from_prior(SEED, FACE, LEVEL, OI as u32, OJ as u32, NX)
}

/// Coarse kernel, seeded from R(prior) at L17 (unrun).
fn coarse_kernel_from(r_h0: &Grid) -> Fluvial {
    let cnx = cnx();
    let coi = coi();
    Fluvial::from_surface(SEED, FACE, clevel(), coi as u32, coj() as u32, cnx, |c| {
        let (_, i, j, _) = c.to_face_ij();
        let x = (i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize;
        let y = (j as i64 - coj() as i64).clamp(0, cnx as i64 - 1) as usize;
        r_h0.at(x, y)
    })
}

/// Area-weighted low-pass restriction of a fine L19 height field to L17 (root of
/// the MRA pyramid, Bilinear predictor — identical to the wavelet spike PROBE 5).
fn restrict_height(fine: &Grid) -> Grid {
    mra::decompose(fine, RADIUS_M, DEPTH, &Bilinear).root
}

/// Fine cell areas (m²), row-major, for area-weighted block reductions.
fn fine_areas() -> Vec<f64> {
    let mut a = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            a.push(cell_area_m2(FACE, OI + x, OJ + y, LEVEL, RADIUS_M));
        }
    }
    a
}

/// Reduce a fine `NX×NX` field to a coarse `cnx×cnx` field, one value per
/// `RATIO×RATIO` block, via a caller-supplied reducer over `(value, area)` pairs.
fn block_reduce(fine: &[f32], areas: &[f64], reduce: impl Fn(&[(f64, f64)]) -> f64) -> Vec<f64> {
    let cnx = cnx();
    let mut out = vec![0.0f64; cnx * cnx];
    for cy in 0..cnx {
        for cx in 0..cnx {
            let mut cell = Vec::with_capacity(RATIO * RATIO);
            for dy in 0..RATIO {
                for dx in 0..RATIO {
                    let fx = cx * RATIO + dx;
                    let fy = cy * RATIO + dy;
                    let i = fy * NX + fx;
                    cell.push((fine[i] as f64, areas[i]));
                }
            }
            out[cy * cnx + cx] = reduce(&cell);
        }
    }
    out
}

fn awmean(cell: &[(f64, f64)]) -> f64 {
    let (mut s, mut w) = (0.0, 0.0);
    for &(v, a) in cell {
        s += v * a;
        w += a;
    }
    s / w
}
fn cmax(cell: &[(f64, f64)]) -> f64 {
    cell.iter().fold(f64::NEG_INFINITY, |m, &(v, _)| m.max(v))
}
/// Order-m power mean ⟨Aᵐ⟩^{1/m} (area-weighted) — the Jensen-exact effective
/// area: the value whose mᵗʰ power equals the mean of the mᵗʰ powers.
fn powmean_m(cell: &[(f64, f64)]) -> f64 {
    let (mut s, mut w) = (0.0, 0.0);
    for &(v, a) in cell {
        s += v.max(0.0).powf(M_EXP) * a;
        w += a;
    }
    (s / w).powf(1.0 / M_EXP)
}

// ── correlation / stats ──────────────────────────────────────────────────────

fn stats(d: &[f64]) -> (f64, f64, f64, f64) {
    let n = d.len() as f64;
    let mean = d.iter().sum::<f64>() / n;
    let rms = (d.iter().map(|x| x * x).sum::<f64>() / n).sqrt();
    let mx = d.iter().fold(0.0f64, |m, &x| m.max(x.abs()));
    let sd = (d.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    (mean, rms, mx, sd)
}

fn pearson(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let cov: f64 = xs.iter().zip(ys).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sx = (xs.iter().map(|a| (a - mx).powi(2)).sum::<f64>()).sqrt();
    let sy = (ys.iter().map(|b| (b - my).powi(2)).sum::<f64>()).sqrt();
    if sx == 0.0 || sy == 0.0 {
        0.0
    } else {
        cov / (sx * sy)
    }
}

// ── the commutator, computed once and shared ─────────────────────────────────

struct Commutator {
    d: Vec<f64>,          // R∘E − E∘R, per coarse cell (signed)
    a_fine_final: Vec<f32>, // fine drainage after 80e (L19)
    a_coarse: Vec<f64>,   // coarse kernel's own drainage after E∘R (L17)
    h0_fine: Vec<f64>,    // prior heights (L19)
}

fn epochs() -> u32 {
    80
}

fn compute_commutator() -> Commutator {
    let h0 = prior_grid();

    // R∘E : erode fine, then restrict.
    let mut ff = fine_kernel();
    ff.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    let fine_eroded = Grid::new(FACE, LEVEL, OI, OJ, NX, ff.h.iter().map(|&x| x as f64).collect());
    let r_of_e = restrict_height(&fine_eroded);

    // E∘R : restrict first, then erode at the coarse level.
    let r_h0 = restrict_height(&h0);
    let mut cf = coarse_kernel_from(&r_h0);
    cf.erode(&FluvialParams { epochs: epochs(), ..Default::default() });

    let e_of_r: Vec<f64> = cf.h.iter().map(|&x| x as f64).collect();
    let d: Vec<f64> = r_of_e.v.iter().zip(&e_of_r).map(|(a, b)| a - b).collect();

    Commutator { d, a_fine_final: ff.drainage.clone(), a_coarse: cf.drainage.iter().map(|&x| x as f64).collect(), h0_fine: h0.v }
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 0 — ANCHOR
pub fn anchor() {
    // THE LAND GUARD — the one the wavelet spike learned to write, honoured here.
    // A submarine footprint makes erosion a near-no-op and every number below a
    // fabrication (this is exactly how the old footprint went stale).
    let sea = vivarium_world::sea_level::derived_sea_level_m(SEED);
    let h0 = prior_grid();
    let land = h0.v.iter().filter(|&&h| h > sea).count();
    let (lo, hi) = h0.v.iter().fold((f64::MAX, f64::MIN), |(l, h), &v| (l.min(v), h.max(v)));
    let mut ff = fine_kernel();
    ff.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    let moved: f64 = h0.v.iter().zip(&ff.h).map(|(a, b)| (*a as f32 - b).abs() as f64).fold(0.0, f64::max);
    let mean_moved: f64 = h0.v.iter().zip(&ff.h).map(|(a, b)| (*a as f32 - b).abs() as f64).sum::<f64>() / (NX * NX) as f64;
    println!("  prior relief {lo:.0}..{hi:.0} m   sea {sea:.0} m   land {}/{} = {:.0}%", land, NX * NX, 100.0 * land as f64 / (NX * NX) as f64);
    println!("  erosion Δh (80e): max {moved:.2} m, mean {mean_moved:.2} m");
    assert!(land > NX * NX / 4, "LAND GUARD FAILED: footprint is not substantially land");
    assert!(moved > 1.0, "LAND GUARD FAILED: fluvial kernel did not execute (every number below would be fake)");
    println!("  ✓ land guard passed — the kernel really erodes here.\n");

    let c = compute_commutator();
    let (mean, rms, mx, sd) = stats(&c.d);
    println!("  ‖R∘E − E∘R‖ :  mean (SIGNED) {mean:+.3} m     RMS {rms:.3} m     max {mx:.3} m");
    println!("  bias / noise :  |mean| / sd = {:.3}  ⇒ {}", mean.abs() / sd,
        if mean.abs() > 0.2 * sd { "a BIAS (systematic, signed — the harmful kind)" } else { "predominantly NOISE" });
    println!("  (wavelet PROBE 5 measured +5.34 m on an OLD world; that footprint is now");
    println!("   submarine. This is a FRESH baseline on the current generator — the number");
    println!("   to beat is THIS mean, and the test is whether a closure zeroes it.)");
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 1 — the drainage-area commutator
pub fn drainage_commutator() {
    let c = compute_commutator();
    let areas = fine_areas();

    // How the FINE drainage restricts into the coarse grid, three ways.
    let r_a_mean = block_reduce(&c.a_fine_final, &areas, awmean);
    let r_a_max = block_reduce(&c.a_fine_final, &areas, cmax);

    // The drainage-area commutator: coarse-routed A vs restricted fine A.
    let da_mean: Vec<f64> = c.a_coarse.iter().zip(&r_a_mean).map(|(a, b)| a - b).collect();
    let da_max: Vec<f64> = c.a_coarse.iter().zip(&r_a_max).map(|(a, b)| a - b).collect();

    let rel = |da: &[f64], r: &[f64]| -> f64 {
        let mut v: Vec<f64> = da.iter().zip(r).map(|(d, a)| (d / a.max(1.0)).abs()).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[v.len() / 2]
    };
    println!("  A does NOT commute with R. |A_coarse − R(A_fine)| / A, median:");
    println!("    vs area-weighted MEAN of fine A : {:.1}%", 100.0 * rel(&da_mean, &r_a_mean));
    println!("    vs MAX (trunk) of fine A         : {:.1}%", 100.0 * rel(&da_max, &r_a_max));

    // The erosion-error PROXY built from the drainage commutator:
    //   ε̂ = E_carved · m · (δA / A)   — first-order incision sensitivity to A.
    // Does it predict the actual height commutator, where local h-detail (−0.027) failed?
    // E_carved per coarse cell: |R(prior) eroded| — use |commutator's| own scale via carved depth.
    let carved = carved_depth_coarse(); // mean erosion depth per coarse cell
    let proxy_from = |da: &[f64], r: &[f64]| -> Vec<f64> {
        da.iter().zip(r).zip(&carved).map(|((d, a), e)| e * M_EXP * (d / a.max(1.0))).collect()
    };
    let p_mean = proxy_from(&da_mean, &r_a_mean);
    let p_max = proxy_from(&da_max, &r_a_max);

    println!("\n  corr( drainage-commutator proxy ε̂ , actual height commutator d ) — SIGNED:");
    println!("    proxy from MEAN-restricted A : r = {:+.3}", pearson(&p_mean, &c.d));
    println!("    proxy from MAX-restricted  A : r = {:+.3}", pearson(&p_max, &c.d));
    println!("  (compare: corr(local h-detail RMS, |d|) = −0.027 — the local statistic is blind.)");
}

/// Mean erosion depth per coarse cell (E∘R carved), a scale for the proxy.
fn carved_depth_coarse() -> Vec<f64> {
    let h0 = prior_grid();
    let r_h0 = restrict_height(&h0);
    let mut cf = coarse_kernel_from(&r_h0);
    let before: Vec<f64> = cf.h.iter().map(|&x| x as f64).collect();
    cf.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    cf.h.iter().zip(&before).map(|(a, b)| (*b - *a as f64).max(0.0)).collect()
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 2 — same field, two statistics
pub fn two_statistics() {
    let c = compute_commutator();
    let areas = fine_areas();
    let absd: Vec<f64> = c.d.iter().map(|x| x.abs()).collect();

    // Sub-grid std of the LOCAL field (h0) vs the NON-LOCAL field (A^m).
    let h0f: Vec<f32> = c.h0_fine.iter().map(|&x| x as f32).collect();
    let am: Vec<f32> = c.a_fine_final.iter().map(|&a| (a.max(0.0)).powf(M_EXP as f32)).collect();

    let sub_std = |fine: &[f32]| -> Vec<f64> {
        block_reduce(fine, &areas, |cell| {
            let m = awmean(cell);
            let var: f64 = cell.iter().map(|&(v, _)| (v - m).powi(2)).sum::<f64>() / cell.len() as f64;
            var.sqrt()
        })
    };
    let sh = sub_std(&h0f);
    let sa = sub_std(&am);

    println!("  corr( sub-grid std , |commutator| ) — same coarse cells, two statistics:");
    println!("    LOCAL      sub-grid std of h₀   : r = {:+.3}", pearson(&sh, &absd));
    println!("    NON-LOCAL  sub-grid std of Aᵐ   : r = {:+.3}", pearson(&sa, &absd));
    println!("  ⇒ if only the non-local one sees the gap, that EXPLAINS the −0.027 —");
    println!("    the closure statistic exists, but it lives on the drainage graph, not in h.");
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 3 — the oracle-A closure (co-evolved)
pub fn oracle_closure() {
    let areas = fine_areas();
    let h0 = prior_grid();
    let r_h0 = restrict_height(&h0);
    let ep = epochs() as usize;

    // Run the fine kernel epoch-by-epoch, snapshotting its live drainage.
    let mut ff = fine_kernel();
    let mut a_snaps: Vec<Vec<f32>> = Vec::with_capacity(ep);
    for _ in 0..ep {
        ff.erode(&FluvialParams { epochs: 1, ..Default::default() });
        a_snaps.push(ff.drainage.clone());
    }
    let r_of_e = restrict_height(&Grid::new(FACE, LEVEL, OI, OJ, NX, ff.h.iter().map(|&x| x as f64).collect()));

    // For each restriction of A, co-evolve a coarse kernel with that A injected
    // each epoch, and measure the resulting commutator.
    let run_oracle = |reduce: &dyn Fn(&[(f64, f64)]) -> f64| -> (f64, f64) {
        let mut cf = coarse_kernel_from(&r_h0);
        for a_fine in a_snaps.iter() {
            let inj: Vec<f32> = block_reduce(a_fine, &areas, reduce).iter().map(|&x| x as f32).collect();
            cf.drainage_override = Some(inj);
            cf.erode(&FluvialParams { epochs: 1, ..Default::default() });
        }
        let d: Vec<f64> = r_of_e.v.iter().zip(cf.h.iter()).map(|(a, b)| a - *b as f64).collect();
        let (mean, rms, _, _) = stats(&d);
        (mean, rms)
    };

    // Baseline (no override), same epoch-by-epoch driver, for an honest comparison.
    let (b_mean, b_rms) = {
        let mut cf = coarse_kernel_from(&r_h0);
        cf.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
        let d: Vec<f64> = r_of_e.v.iter().zip(cf.h.iter()).map(|(a, b)| a - *b as f64).collect();
        let (mean, rms, _, _) = stats(&d);
        (mean, rms)
    };

    println!("  commutator signed mean / RMS after co-evolving the coarse kernel with an");
    println!("  injected drainage field (the oracle-A closure):");
    println!("    BASELINE (coarse routes its own A)     : mean {b_mean:+.3} m   RMS {b_rms:.3} m");
    let (m1, r1) = run_oracle(&awmean);
    println!("    oracle: R(A_fine) = area-weighted MEAN : mean {m1:+.3} m   RMS {r1:.3} m");
    let (m2, r2) = run_oracle(&cmax);
    println!("    oracle: R(A_fine) = MAX (trunk)        : mean {m2:+.3} m   RMS {r2:.3} m");
    let (m3, r3) = run_oracle(&powmean_m);
    println!("    oracle: R(A_fine) = order-m POWER MEAN : mean {m3:+.3} m   RMS {r3:.3} m");
    println!("\n  ⇒ whichever injection collapses the signed bias toward 0 IS the drainage");
    println!("    statistic the closure must carry. A residual that stays biased is the");
    println!("    part routing-magnitude alone cannot fix (pointwise / deeper).");
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 4 — the pointwise residue (single epoch, analytic vs measured)
pub fn pointwise_jensen() {
    // One epoch isolates the pointwise (Jensen + Cov) term from the 80-epoch
    // feedback, and is where FE(4)'s analytic decomposition should be tightest.
    let areas = fine_areas();
    let h0 = prior_grid();

    // R∘E, one epoch.
    let mut ff = fine_kernel();
    ff.erode(&FluvialParams { epochs: 1, ..Default::default() });
    let carved_fine: Vec<f32> = h0.v.iter().zip(&ff.h).map(|(a, b)| (*a as f32 - b)).collect();
    let r_carved = block_reduce(&carved_fine, &areas, awmean); // R of the fine incision

    // E∘R, one epoch.
    let r_h0 = restrict_height(&h0);
    let mut cf = coarse_kernel_from(&r_h0);
    let cbefore: Vec<f64> = cf.h.iter().map(|&x| x as f64).collect();
    cf.erode(&FluvialParams { epochs: 1, ..Default::default() });
    let carved_coarse: Vec<f64> = cbefore.iter().zip(cf.h.iter()).map(|(a, b)| a - *b as f64).collect();

    let d1: Vec<f64> = r_carved.iter().zip(&carved_coarse).map(|(a, b)| a - b).collect();
    let (mean, rms, _, sd) = stats(&d1);
    println!("  ONE-EPOCH commutator of the CARVED depth: mean {mean:+.3} m   RMS {rms:.3} m   bias/noise {:.2}", mean.abs() / sd);

    // Analytic FE(4) decomposition of the fine incision within each coarse cell.
    // incision_i ≈ k·A_iᵐ·S_i. Jensen term ⟨Aᵐ⟩−⟨A⟩ᵐ (×⟨S⟩); Cov(Aᵐ,S).
    // Report the sign and relative size of each — the closure's ingredient list.
    let s_fine = steepest_slope(&h0.v); // per fine cell
    let cnx = cnx();
    let (mut jensen, mut covar) = (vec![0.0f64; cnx * cnx], vec![0.0f64; cnx * cnx]);
    for cy in 0..cnx {
        for cx in 0..cnx {
            let mut am = Vec::new();
            let mut a = Vec::new();
            let mut s = Vec::new();
            let mut w = Vec::new();
            for dy in 0..RATIO {
                for dx in 0..RATIO {
                    let i = (cy * RATIO + dy) * NX + (cx * RATIO + dx);
                    let av = ff.drainage[i].max(0.0) as f64;
                    am.push(av.powf(M_EXP));
                    a.push(av);
                    s.push(s_fine[i]);
                    w.push(areas[i]);
                }
            }
            let wsum: f64 = w.iter().sum();
            let wm = |v: &[f64]| v.iter().zip(&w).map(|(x, wi)| x * wi).sum::<f64>() / wsum;
            let mean_am = wm(&am);
            let mean_a = wm(&a);
            let mean_s = wm(&s);
            let jensen_gap = mean_am - mean_a.powf(M_EXP); // ⟨Aᵐ⟩ − ⟨A⟩ᵐ  (≤ 0, concave)
            let cov: f64 = am.iter().zip(&s).zip(&w).map(|((x, si), wi)| (x - mean_am) * (si - mean_s) * wi).sum::<f64>() / wsum;
            jensen[cy * cnx + cx] = jensen_gap * mean_s;
            covar[cy * cnx + cx] = cov;
        }
    }
    let (jm, _, _, _) = stats(&jensen);
    let (cm, _, _, _) = stats(&covar);
    println!("\n  FE(4) decomposition of the sub-grid incision (per coarse cell, area-weighted):");
    println!("    Jensen term  ⟨Aᵐ⟩−⟨A⟩ᵐ (×⟨S⟩) : mean {jm:+.3e}   (concave m=0.5 ⇒ expect ≤ 0)");
    println!("    Cov(Aᵐ, S)                     : mean {cm:+.3e}   (channels: high A ↔ low S ⇒ expect < 0)");
    println!("  corr( Jensen+Cov structure , one-epoch commutator ) = {:+.3}", pearson(&jensen.iter().zip(&covar).map(|(a, b)| a + b).collect::<Vec<_>>(), &d1));
    println!("  ⇒ these are the POINTWISE ingredients; PROBE 3 tests whether they (via the");
    println!("    power-mean) plus correct routing magnitude actually zero the 80-epoch bias.");
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 5 — robustness: does the trunk-oracle collapse the bias across tiles/depths?
pub fn robustness() {
    use vivarium_world::sphere::Face;
    println!("  baseline signed-bias vs oracle-injected signed-bias, per (tile, depth).");
    println!("  MEAN = area-weighted mean (the height restriction); MAX = trunk of fine A.\n");
    println!("  {:<22} {:>3}  {:>9}  {:>9}  {:>9}", "tile", "dep", "baseline", "MEAN", "MAX(trunk)");
    let tiles = [
        (Face::ZNeg, 327_680u64, 65_536u64, "ZNeg(327680,65536)"),
        (Face::XNeg, 65_536, 262_144, "XNeg(65536,262144)"),
        (Face::ZNeg, 196_608, 458_752, "ZNeg(196608,458752)"),
    ];
    for (face, oi, oj, name) in tiles {
        for depth in [1usize, 2] {
            let (b, mn, mx) = oracle_at(face, oi, oj, depth);
            println!("  {name:<22} {depth:>3}  {b:>+8.3}m  {mn:>+8.3}m  {mx:>+8.3}m");
        }
    }
    println!("\n  ⇒ if MAX(trunk) sits an order of magnitude closer to zero than baseline and");
    println!("    MEAN across tiles/depths, the trunk statistic is the robust closure carrier,");
    println!("    not a fluke of one footprint.");
}

/// Parametric oracle: co-evolve a coarse kernel at `depth` below `(face,oi,oj)` L19
/// tile with an injected drainage field; return (baseline, mean-oracle, max-oracle)
/// signed commutator means. Self-contained (does not use the module consts).
fn oracle_at(face: vivarium_world::sphere::Face, oi: u64, oj: u64, depth: usize) -> (f64, f64, f64) {
    let ratio = 1usize << depth;
    let cnx = NX >> depth;
    let clevel = LEVEL - depth as u8;
    let (coi, coj) = (oi >> depth, oj >> depth);
    let ep = epochs() as usize;

    // fine areas for this tile
    let mut areas = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            areas.push(cell_area_m2(face, oi + x, oj + y, LEVEL, RADIUS_M));
        }
    }
    // prior heights
    let mut h0 = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            let c = CellId::from_face_ij(face, (oi + x) as u32, (oj + y) as u32, LEVEL);
            h0.push(gen::initial_topography_m(SEED, c, LEVEL));
        }
    }
    let restrict = |fine: &[f64]| -> Vec<f64> {
        mra::decompose(&Grid::new(face, LEVEL, oi, oj, NX, fine.to_vec()), RADIUS_M, depth, &Bilinear).root.v
    };
    let r_h0 = restrict(&h0);
    let mk_coarse = || {
        Fluvial::from_surface(SEED, face, clevel, coi as u32, coj as u32, cnx, |c| {
            let (_, i, j, _) = c.to_face_ij();
            let x = (i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize;
            let y = (j as i64 - coj as i64).clamp(0, cnx as i64 - 1) as usize;
            r_h0[y * cnx + x]
        })
    };
    let reduce = |fine: &[f32], f: &dyn Fn(&[(f64, f64)]) -> f64| -> Vec<f32> {
        let mut out = vec![0.0f32; cnx * cnx];
        for cy in 0..cnx {
            for cx in 0..cnx {
                let mut cell = Vec::with_capacity(ratio * ratio);
                for dy in 0..ratio {
                    for dx in 0..ratio {
                        let i = (cy * ratio + dy) * NX + (cx * ratio + dx);
                        cell.push((fine[i] as f64, areas[i]));
                    }
                }
                out[cy * cnx + cx] = f(&cell) as f32;
            }
        }
        out
    };

    // fine run with per-epoch drainage snapshots
    let mut ff = Fluvial::from_prior(SEED, face, LEVEL, oi as u32, oj as u32, NX);
    let mut snaps = Vec::with_capacity(ep);
    for _ in 0..ep {
        ff.erode(&FluvialParams { epochs: 1, ..Default::default() });
        snaps.push(ff.drainage.clone());
    }
    let r_of_e = restrict(&ff.h.iter().map(|&x| x as f64).collect::<Vec<_>>());

    let mean_of = |cf: &Fluvial| -> f64 {
        r_of_e.iter().zip(cf.h.iter()).map(|(a, b)| a - *b as f64).sum::<f64>() / (cnx * cnx) as f64
    };
    // baseline
    let mut cb = mk_coarse();
    cb.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    let base = mean_of(&cb);
    // oracle mean / max
    let run = |f: &dyn Fn(&[(f64, f64)]) -> f64| -> f64 {
        let mut cf = mk_coarse();
        for a in &snaps {
            cf.drainage_override = Some(reduce(a, f));
            cf.erode(&FluvialParams { epochs: 1, ..Default::default() });
        }
        mean_of(&cf)
    };
    (base, run(&awmean), run(&cmax))
}

// ═════════════════════════════════════════════════════════════════════════════
// PROBE 6/7 — the DEPLOYABLE coarse-only closure
use vivarium_world::sphere::Face;

const TILES: [(Face, u64, u64, &str); 3] = [
    (Face::ZNeg, 327_680, 65_536, "ZNeg(327680,65536)"),
    (Face::XNeg, 65_536, 262_144, "XNeg(65536,262144)"),
    (Face::ZNeg, 196_608, 458_752, "ZNeg(196608,458752)"),
];

/// Coarse-run drainage `A_coarse` and the fine trunk `max(A_fine)` per coarse
/// cell, plus the fine-restricted target `R(E(h0))`, for one tile/depth.
fn tile_fields(face: Face, oi: u64, oj: u64, depth: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let ratio = 1usize << depth;
    let cnx = NX >> depth;
    let clevel = LEVEL - depth as u8;
    let (coi, coj) = (oi >> depth, oj >> depth);

    let mut areas = Vec::with_capacity(NX * NX);
    let mut h0 = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            areas.push(cell_area_m2(face, oi + x, oj + y, LEVEL, RADIUS_M));
            let c = CellId::from_face_ij(face, (oi + x) as u32, (oj + y) as u32, LEVEL);
            h0.push(gen::initial_topography_m(SEED, c, LEVEL));
        }
    }
    let restrict = |fine: &[f64]| mra::decompose(&Grid::new(face, LEVEL, oi, oj, NX, fine.to_vec()), RADIUS_M, depth, &Bilinear).root.v;

    // fine run → trunk (max) per coarse cell + restricted target
    let mut ff = Fluvial::from_prior(SEED, face, LEVEL, oi as u32, oj as u32, NX);
    ff.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
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
    let r_of_e = restrict(&ff.h.iter().map(|&x| x as f64).collect::<Vec<_>>());

    // coarse baseline run → its own drainage
    let r_h0 = restrict(&h0);
    let mut cf = Fluvial::from_surface(SEED, face, clevel, coi as u32, coj as u32, cnx, |c| {
        let (_, i, j, _) = c.to_face_ij();
        let x = (i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize;
        let y = (j as i64 - coj as i64).clamp(0, cnx as i64 - 1) as usize;
        r_h0[y * cnx + x]
    });
    cf.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    let a_coarse: Vec<f64> = cf.drainage.iter().map(|&x| x as f64).collect();
    (a_coarse, max_af, r_of_e)
}

/// Log-log least-squares fit  max(A_fine) ≈ α·A_coarseᵝ  → (alpha, beta, R²).
fn loglog_fit(a_coarse: &[f64], max_af: &[f64]) -> (f64, f64, f64) {
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
    let beta = sxy / sxx;
    let alpha = (my - beta * mx).exp();
    let r2 = (sxy * sxy) / (sxx * syy);
    (alpha, beta, r2)
}

/// Run a coarse kernel with the recalibration A ← α·Aᵝ armed; return the
/// commutator's (signed mean, bias/noise) against the tile's fine-restricted target.
fn recal_bias(face: Face, oi: u64, oj: u64, depth: usize, ab: (f32, f32), r_of_e: &[f64]) -> (f64, f64) {
    let cnx = NX >> depth;
    let clevel = LEVEL - depth as u8;
    let (coi, coj) = (oi >> depth, oj >> depth);
    let mut h0 = Vec::with_capacity(NX * NX);
    for y in 0..NX as u64 {
        for x in 0..NX as u64 {
            let c = CellId::from_face_ij(face, (oi + x) as u32, (oj + y) as u32, LEVEL);
            h0.push(gen::initial_topography_m(SEED, c, LEVEL));
        }
    }
    let r_h0 = mra::decompose(&Grid::new(face, LEVEL, oi, oj, NX, h0), RADIUS_M, depth, &Bilinear).root.v;
    let mut cf = Fluvial::from_surface(SEED, face, clevel, coi as u32, coj as u32, cnx, |c| {
        let (_, i, j, _) = c.to_face_ij();
        let x = (i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize;
        let y = (j as i64 - coj as i64).clamp(0, cnx as i64 - 1) as usize;
        r_h0[y * cnx + x]
    });
    cf.drainage_recalibrate = Some(ab);
    cf.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    let d: Vec<f64> = r_of_e.iter().zip(cf.h.iter()).map(|(a, b)| a - *b as f64).collect();
    let (mean, _, _, sd) = stats(&d);
    (mean, mean.abs() / sd)
}

/// Find `want` interior, 100%-land, high-relief quadtree-aligned tiles on `seed`.
fn find_land_tiles(seed: u64, want: usize) -> Vec<(Face, u64, u64)> {
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed);
    let faces = [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZNeg, Face::ZPos];
    let n = 1u64 << LEVEL;
    let step = (n / 32) & !(NX as u64 - 1); // denser scan so every seed yields ≥3 land tiles
    let mut out = Vec::new();
    for face in faces {
        let mut oj = 0;
        while oj + NX as u64 <= n {
            let mut oi = 0;
            while oi + NX as u64 <= n {
                let interior = oi >= NX as u64 && oj >= NX as u64 && oi + 2 * NX as u64 <= n && oj + 2 * NX as u64 <= n;
                if interior {
                    let (mut land, mut cnt, mut lo, mut hi) = (0u32, 0u32, f64::MAX, f64::MIN);
                    for y in (0..NX as u64).step_by(8) {
                        for x in (0..NX as u64).step_by(8) {
                            let c = CellId::from_face_ij(face, (oi + x) as u32, (oj + y) as u32, LEVEL);
                            let h = gen::initial_topography_m(seed, c, LEVEL);
                            if h > sea {
                                land += 1;
                            }
                            cnt += 1;
                            lo = lo.min(h);
                            hi = hi.max(h);
                        }
                    }
                    if land == cnt && hi - lo > 150.0 {
                        out.push((face, oi, oj));
                        if out.len() >= want {
                            return out;
                        }
                    }
                }
                oi += step;
            }
            oj += step;
        }
    }
    out
}

/// Seed-parametric `A_coarse` and fine trunk `max(A_fine)` per coarse cell.
fn tile_fields_seeded(seed: u64, face: Face, oi: u64, oj: u64, depth: usize) -> (Vec<f64>, Vec<f64>) {
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
    let mut ff = Fluvial::from_prior(seed, face, LEVEL, oi as u32, oj as u32, NX);
    ff.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
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
    let r_h0 = mra::decompose(&Grid::new(face, LEVEL, oi, oj, NX, h0), RADIUS_M, depth, &Bilinear).root.v;
    let mut cf = Fluvial::from_surface(seed, face, clevel, coi as u32, coj as u32, cnx, |c| {
        let (_, i, j, _) = c.to_face_ij();
        let x = (i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize;
        let y = (j as i64 - coj as i64).clamp(0, cnx as i64 - 1) as usize;
        r_h0[y * cnx + x]
    });
    cf.erode(&FluvialParams { epochs: epochs(), ..Default::default() });
    (cf.drainage.iter().map(|&x| x as f64).collect(), max_af)
}

/// PROBE 8 — harden the R² ceiling on a SECOND seed: is "the trunk is not a
/// pointwise function of A_coarse" measured law, or a seed-0 anecdote?
pub fn harden_ceiling() {
    println!("  R² of the best pointwise fit max(A_fine) ≈ α·A_coarseᵝ, two seeds:");
    println!("  (R² bounds EVERY monotone pointwise closure — it is the closure ceiling.)\n");
    println!("  {:>4}  {:<20} {:>3}  {:>6}", "seed", "tile", "dep", "R²");
    let mut worst = 0.0f64; // largest R² seen = the ceiling
    for seed in [0u64, 1] {
        let tiles = find_land_tiles(seed, 3);
        for (face, oi, oj) in tiles {
            for depth in [1usize, 2] {
                let (ac, maf) = tile_fields_seeded(seed, face, oi, oj, depth);
                let (_, _, r2) = loglog_fit(&ac, &maf);
                println!("  {seed:>4}  {:<20} {depth:>3}  {r2:>6.3}", format!("{face:?}({oi},{oj})"));
                worst = worst.max(r2);
            }
        }
    }
    println!("\n  ⇒ CEILING (max R² over 2 seeds × 3 tiles × 2 depths) = {worst:.3}");
    println!("    No pointwise coarse-only closure can explain more than this fraction of the");
    println!("    trunk's variance. {}", if worst < 0.5 { "The no-go holds across seeds — measured law." } else { "⚠ a fit exists on some tile — re-examine." });
}

pub fn deployable_closure() {
    // P6 — feasibility: is the trunk a pointwise function of the coarse drainage?
    println!("  P6 — FEASIBILITY: fit  max(A_fine) ≈ α·A_coarseᵝ  (coarse-only computable):");
    println!("  {:<22} {:>3}  {:>6}  {:>7}  {:>7}", "tile", "dep", "R²", "α", "β");
    let mut fits: Vec<(usize, f64, f64, f64)> = Vec::new(); // depth, alpha, beta, r2
    let mut targets: Vec<(Face, u64, u64, usize, Vec<f64>)> = Vec::new();
    for (face, oi, oj, name) in TILES {
        for depth in [1usize, 2] {
            let (ac, maf, roe) = tile_fields(face, oi, oj, depth);
            let (alpha, beta, r2) = loglog_fit(&ac, &maf);
            println!("  {name:<22} {depth:>3}  {r2:>6.3}  {alpha:>7.3}  {beta:>7.3}");
            fits.push((depth, alpha, beta, r2));
            targets.push((face, oi, oj, depth, roe));
        }
    }

    // Calibrate on TILE 1 (fits index 0=depth1, 1=depth2); apply to HELD-OUT tiles 2 & 3.
    let cal = |depth: usize| -> (f32, f32) {
        let f = fits.iter().find(|(d, ..)| *d == depth).unwrap();
        (f.1 as f32, f.2 as f32) // tile-1 fit (first occurrence)
    };
    println!("\n  P7 — DEPLOYABLE (held-out): calibrate (α,β) on tile 1, apply coarse-only to tiles 2&3.");
    println!("  {:<22} {:>3}  {:>11}  {:>16}", "tile", "dep", "baseline b/n", "recalibrated b/n");
    // baselines from RUN PROBE 5 recomputed here for honesty
    for (ti, (face, oi, oj, name)) in TILES.iter().enumerate() {
        for depth in [1usize, 2] {
            let roe = &targets.iter().find(|(f, o, j, d, _)| f == face && o == oi && j == oj && *d == depth).unwrap().4;
            // baseline (no recal)
            let (bm, bn) = recal_bias(*face, *oi, *oj, depth, (1.0, 1.0), roe);
            let (rm, rn) = recal_bias(*face, *oi, *oj, depth, cal(depth), roe);
            let held = if ti == 0 { "(fit)" } else { "HELD-OUT" };
            println!("  {name:<22} {depth:>3}  {bm:>+6.2}m {bn:>4.2}  →  {rm:>+6.2}m {rn:>4.2}  {held}");
        }
    }
    println!("\n  ⇒ if held-out b/n drops below ~0.3 (bias→noise) with tile-1 (α,β), the coarse-only");
    println!("    recalibration is a deployable closure; if only the fit tile improves, it is a fit.");
}

/// Steepest-descent slope (rise/run, dimensionless) per fine cell, from heights.
/// Uniform cell length is used for the arm here (a slope PROXY for the covariance
/// structure, not the kernel's exact geodesic length) — declared, and it affects
/// only the reported Cov magnitude, never PROBE 3's verdict.
fn steepest_slope(h: &[f64]) -> Vec<f64> {
    let cell_m = (std::f64::consts::FRAC_PI_2 * RADIUS_M) / (1u64 << LEVEL) as f64;
    let nbr: [(i64, i64); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut s = vec![0.0f64; NX * NX];
    for y in 0..NX as i64 {
        for x in 0..NX as i64 {
            let hi = h[(y * NX as i64 + x) as usize];
            let mut best = 0.0f64;
            for (dx, dy) in nbr {
                let (nx_, ny_) = (x + dx, y + dy);
                if nx_ < 0 || ny_ < 0 || nx_ >= NX as i64 || ny_ >= NX as i64 {
                    continue;
                }
                let hj = h[(ny_ * NX as i64 + nx_) as usize];
                let dist = if dx != 0 && dy != 0 { cell_m * std::f64::consts::SQRT_2 } else { cell_m };
                let slope = (hi - hj) / dist;
                if slope > best {
                    best = slope;
                }
            }
            s[(y * NX as i64 + x) as usize] = best;
        }
    }
    s
}
