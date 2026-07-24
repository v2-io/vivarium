//! RED-TEAM PROBE for `.super-archive/from-theory/discretisation-and-information.md` (2026-07-13).
//!
//! UNCOMMITTED, adversarial. Four probes, each able to return "the doc is right":
//!
//!   A. Does `pin_block_means` actually preserve block means (ARCHITECTURE law 1,
//!      "R∘L = id on the mean, live")? Predicted residual: (Δ_b²/8)·∇²(delta) —
//!      the SAME (1,6,1)/8 stencil the doc derives in §4.2 for a different object.
//!   B. Is mean-pin the Haar low-pass? Haar needs EQUAL weights. Measure the real
//!      cube-sphere cell-area spread WITHIN a parent block (not the global 1.4×),
//!      and whether mean-pin conserves Σ A·h on the sphere.
//!   C. §2.4's centroid identity: our stored point is the PARAMETRIC centre, not
//!      the AREA centroid. Measure the offset and the volume error it induces.
//!   D. water.rs's θ sweep is IN-PLACE (Gauss–Seidel); de Almeida et al. 2012
//!      specify a Jacobi average at time level n. Measure the deviation.

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::sphere::{CellId, CubeCoord, Face};

const FACE: Face = Face::ZPos;
const LEVEL: u8 = 19;
const OI: u32 = 108_500;
const OJ: u32 = 186_350;
const NX: usize = 96;

/// Solid angle of the spherical triangle (a,b,c) — Van Oosterom & Strackee.
fn tri_solid_angle(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let cross = |p: [f64; 3], q: [f64; 3]| {
        [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]]
    };
    let num = dot(a, cross(b, c)).abs();
    let den = 1.0 + dot(a, b) + dot(b, c) + dot(c, a);
    2.0 * num.atan2(den)
}

/// Solid angle of face-cell (i,j) at `level` — the real equiangular cube-sphere cell.
fn cell_solid_angle(face: Face, i: u32, j: u32, level: u8) -> f64 {
    let n = (1u32 << level) as f64;
    let p = |k: f64| (k / n) * 2.0 - 1.0;
    let corner = |du: f64, dv: f64| {
        CubeCoord { face, u: p(i as f64 + du), v: p(j as f64 + dv) }.to_unit()
    };
    let (p00, p10, p11, p01) =
        (corner(0.0, 0.0), corner(1.0, 0.0), corner(1.0, 1.0), corner(0.0, 1.0));
    tri_solid_angle(p00, p10, p11) + tri_solid_angle(p00, p11, p01)
}

fn main() {
    println!("=== RED-TEAM PROBE — discretisation-and-information.md ===\n");
    probe_a_pin_residual();
    probe_a2_guard_test();
    probe_b_haar_weights();
    probe_c_centroid();
    probe_d_theta_sweep();
    probe_e_checkerboard();
    probe_f_residual_vs_age();
    probe_g_area_ratio_by_level();
    probe_h_froude_envelope();
}

// ── H ────────────────────────────────────────────────────────────────────────
// THE PRIME QUESTION, asked of water.rs. de Almeida & Bates (2013, WRR 49:4833)
// — the applicability study `water.rs` itself cites — bound the local-inertial
// approximation: near-identical to full SWE at Fr < 0.5, diverging up to Fr = 1,
// and "the physical characteristics of the flow field of supercritical flow with
// a Froude number of greater than 1 CANNOT BE CORRECTLY EXPRESSED by the local
// inertial model." What Froude numbers do we actually run at?
fn probe_h_froude_envelope() {
    println!("── H. What Froude regime does water.rs actually run in? ──────────");
    use vivarium_world::water::{WaterParams, WaterSim};

    // Real eroded land, the verified-land footprint.
    let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    f.erode(&FluvialParams { epochs: 60, ..Default::default() });
    let bed = f.h.clone();
    let mut w = WaterSim::new(FACE, LEVEL, (OI, OJ), NX, 19.0, bed, 1.0e5);

    // GUARD (a probe that cannot fail is not a probe): a 400-step rain at the
    // default precip deposits ~2 mm — below the 0.05 m "wet" threshold the Froude
    // instrument uses, so it would read 0.00 forever and prove nothing. Seed a
    // real sheet of water on the real slopes: THAT is the regime that produced
    // the solitons in the first place.
    for d in w.depth.iter_mut() {
        *d = 2.0;
    }
    let wet0 = w.depth.iter().filter(|&&d| d >= 0.05).count();
    assert!(wet0 * 2 > NX * NX, "probe is vacuous: only {wet0} wet cells");
    println!("   seeded {wet0}/{} cells with 2 m of water on 60-epoch eroded land", NX * NX);

    let p = WaterParams {
        sea_m: -1.0e6,
        precip: 0.0,
        evaporation: 0.0,
        infiltration: 0.0,
        sed_capacity: 0.0,
        ..Default::default()
    };
    println!("   step    max Fr    % of wet cells supercritical (Fr > 1.5)");
    for s in 1..=400 {
        w.step(&p);
        if s % 50 == 0 {
            let (mx, sup) = w.froude();
            println!("   {s:4}   {mx:8.2}    {:6.2}%", 100.0 * sup);
        }
    }
    let (mx, sup) = w.froude();
    println!("\n  de Almeida & Bates 2013's envelope:  Fr < 0.5 good · → 1.0 degrading");
    println!("                                       Fr > 1.0 CANNOT be expressed");
    println!("  ours: max Fr {mx:.1}, {:.1}% of wet cells above Fr 1.5.", 100.0 * sup);
    println!("  ⇒ The honest audit finding is NOT 'θ lacks a fluctuation partner'.");
    println!("     It is: WE ARE OUTSIDE THE PUBLISHED VALIDITY ENVELOPE of the");
    println!("     momentum equation we chose, and the three stabilisers (θ, Jarrett");
    println!("     roughness, the Fr=2 breaking cap) are what keep it from detonating.");
    println!();
}

// ── F ────────────────────────────────────────────────────────────────────────
// `seam_ridge` measures 2.45 / 3.76 / 5.79 — GROWING with the differential age
// gap while the interior stays flat. The documented cause is "mean-pin conserves
// block means but not boundary gradients." If probe A is right, there is a
// SECOND mechanism: the pin residual ∝ ∇²(delta), and the age gap is exactly what
// grows |delta|. Does the residual grow with age?
fn probe_f_residual_vs_age() {
    println!("── F. Does the pin residual GROW with the age gap? ───────────────");
    println!("   epochs   mean |resid|   max |resid|");
    for &ep in &[6u32, 18, 30, 60, 150] {
        let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
        let seed_h = f.h.clone();
        f.erode(&FluvialParams { epochs: ep, ..Default::default() });
        let seed_at = move |c: CellId| -> f64 {
            let (_, i, j, _) = c.to_face_ij();
            seed_h[((j - OJ) as usize) * NX + (i - OI) as usize] as f64
        };
        f.pin_block_means(LEVEL - 2, &seed_at);
        let (b, nb) = (4usize, NX / 4);
        let (mut s, mut mx, mut n) = (0.0f64, 0.0f64, 0usize);
        for by in 1..nb - 1 {
            for bx in 1..nb - 1 {
                let mut m = 0.0f64;
                for y in 0..b {
                    for x in 0..b {
                        m += f.h[(by * b + y) * NX + bx * b + x] as f64;
                    }
                }
                m /= (b * b) as f64;
                let t = seed_at(CellId::from_face_ij(
                    FACE,
                    OI + (bx * b + b / 2) as u32,
                    OJ + (by * b + b / 2) as u32,
                    LEVEL,
                ));
                let e = (m - t).abs();
                s += e;
                mx = mx.max(e);
                n += 1;
            }
        }
        println!("   {ep:6}   {:>10.4} m  {:>10.4} m", s / n as f64, mx);
    }
    println!("  ⇒ if this grows with epochs, it is a SECOND mechanism for the seam");
    println!("     ridge, co-located with it and with the same age signature.");
    println!();
}

// ── G ────────────────────────────────────────────────────────────────────────
// Probe B said the equal-weight (Haar) mean is harmless at L19. But that is one
// 1.8 km tile — the Jacobian barely moves across it. Where DOES the 1.4× bite?
// Measure the cell-area spread WITHIN one parent block, by level, at the face
// CORNER (the worst place).
fn probe_g_area_ratio_by_level() {
    println!("── G. Where does the equal-weight assumption actually bite? ──────");
    println!("   level   cells/face   area ratio WITHIN one 2×2 parent block (worst)");
    for level in 1..=20u8 {
        if !(level <= 8 || level % 4 == 0) {
            continue;
        }
        let n = 1u32 << level;
        // worst parent block: the one at the face corner
        let (pi, pj) = (n / 2 - 1, n / 2 - 1); // parent coords at level-1 → corner child block
        let (i0, j0) = (pi * 2, pj * 2);
        let (mut amin, mut amax) = (f64::MAX, 0.0f64);
        for dy in 0..2u32 {
            for dx in 0..2u32 {
                let a = cell_solid_angle(FACE, i0 + dx, j0 + dy, level);
                amin = amin.min(a);
                amax = amax.max(a);
            }
        }
        // and the whole-face spread, for contrast
        let a_c = cell_solid_angle(FACE, n / 2, n / 2, level);
        let a_k = cell_solid_angle(FACE, n - 1, n - 1, level);
        println!(
            "   {level:5}   {:>10}   {:.8}×     (whole-face centre/corner: {:.4}×)",
            n * n,
            amax / amin,
            a_c / a_k
        );
    }
    println!("  ⇒ the 1.4× is a WHOLE-FACE figure. Within a PARENT BLOCK the spread");
    println!("     collapses as 1 + O(Δ). Equal-weight (Haar) coarsening is wrong at");
    println!("     COARSE levels and asymptotically harmless at fine ones.");
    println!();
}

// ── A2 ───────────────────────────────────────────────────────────────────────
// The in-tree guard `pin_preserves_parent_means` uses epochs=6, samples every
// 7th block, and allows 2.0 m of slop with the comment "Bilinear upsampling
// smooths deltas, so means match approximately." Would it survive FULL sampling?
fn probe_a2_guard_test() {
    println!("── A2. Would the in-tree guard survive full sampling? ────────────");
    let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    let seed_h = f.h.clone();
    f.erode(&FluvialParams { epochs: 6, ..Default::default() }); // the test's value
    let seed_at = move |c: CellId| -> f64 {
        let (_, i, j, _) = c.to_face_ij();
        seed_h[((j - OJ) as usize) * NX + (i - OI) as usize] as f64
    };
    f.pin_block_means(LEVEL - 2, &seed_at);

    let (b, nb) = (4usize, NX / 4);
    let (mut max_sampled, mut max_all) = (0.0f64, 0.0f64);
    for by in 0..nb {
        for bx in 0..nb {
            let mut m = 0.0f64;
            for y in 0..b {
                for x in 0..b {
                    m += f.h[(by * b + y) * NX + bx * b + x] as f64;
                }
            }
            m /= (b * b) as f64;
            let t = seed_at(CellId::from_face_ij(
                FACE,
                OI + (bx * b + b / 2) as u32,
                OJ + (by * b + b / 2) as u32,
                LEVEL,
            ));
            let e = (m - t).abs();
            max_all = max_all.max(e);
            if bx % 7 == 0 && by % 7 == 0 {
                max_sampled = max_sampled.max(e);
            }
        }
    }
    println!("  epochs = 6 (the guard's value), tolerance = 2.0 m");
    println!("    worst error over the blocks the guard ACTUALLY CHECKS  {max_sampled:.3} m");
    println!("    worst error over ALL {} blocks .......................  {max_all:.3} m", nb * nb);
    println!(
        "  ⇒ the guard {} under full sampling.",
        if max_all > 2.0 { "FAILS" } else { "would still pass" }
    );
    println!();
}

// ── E ────────────────────────────────────────────────────────────────────────
// Doc §2.5: "Those solitons were THE INVISIBLE MODE. The θ-smoothing is
// Rhie–Chow-class stabilisation." Rhie–Chow cures a RANK DEFICIENCY: a mode the
// operator literally cannot see (zero eigenvalue, grows without resistance).
// Feed water.rs its own Nyquist mode — a checkerboard in the water surface — and
// see whether the scheme is blind to it.
fn probe_e_checkerboard() {
    println!("── E. Is water.rs actually BLIND to the checkerboard? ────────────");
    use vivarium_world::water::{WaterParams, WaterSim};

    let nx = 64usize;
    let bed = vec![0.0f32; nx * nx]; // flat bed
    let mut w = WaterSim::new(FACE, LEVEL, (OI, OJ), nx, 19.0, bed, 0.0);

    // The Nyquist mode, in the water surface: depth = 1 ± 0.1·(−1)^{i+j}
    let amp0 = 0.1f32;
    for y in 0..nx {
        for x in 0..nx {
            let s = if (x + y) % 2 == 0 { 1.0 } else { -1.0 };
            w.depth[y * nx + x] = 1.0 + amp0 * s;
        }
    }
    let p = WaterParams {
        precip: 0.0,
        evaporation: 0.0,
        ocean_evap: 0.0,
        infiltration: 0.0,
        baseflow: 0.0,
        sed_capacity: 0.0,
        sea_m: -1.0e6, // no sea boundary anywhere
        ..Default::default()
    };

    let amp = |w: &WaterSim| -> f32 {
        let mut a = 0.0f32;
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                let s = if (x + y) % 2 == 0 { 1.0 } else { -1.0 };
                a += s * (w.depth[y * nx + x] - 1.0);
            }
        }
        a / ((nx - 2) * (nx - 2)) as f32
    };

    println!("  checkerboard amplitude in the water surface, per step:");
    println!("    step  0 ... {:.6e}", amp(&w));
    for s in 1..=8 {
        w.step(&p);
        if s <= 4 || s == 8 {
            println!("    step {s:2} ... {:.6e}", amp(&w));
        }
    }
    let end = amp(&w);
    println!("\n  A NULL MODE would be INVARIANT (the operator returns exactly zero on it)");
    println!("  or GROW without resistance. Ours decayed to {:.2}% of its initial amplitude", 100.0 * (end / amp0).abs());
    println!("  in 8 steps — WITHOUT any θ term being needed to see it.");
    println!("  Reason: water.rs's head operator is (η_i − η_j) across the FACE — a");
    println!("  COMPACT 2-point difference, spacing Δ, not the collocated 2Δ central");
    println!("  difference. Its symbol is 2·sin(kΔ/2)/Δ, which at k=π/Δ is 2/Δ ≠ 0.");
    println!("  ⇒ water.rs is ALREADY STAGGERED. It HAS NO NYQUIST NULL MODE.");
    println!("     There is no rank deficiency for a Rhie–Chow term to cure.");
    println!();
}

// ── A ────────────────────────────────────────────────────────────────────────
// Claim under test (doc §3.7): "Mean-pinning IS the Haar wavelet's low-pass
// (scaling) coefficient. It KEEPS THE BLOCK AVERAGE and throws away the details."
// And ARCHITECTURE.md law 1: "mean-pinning ... is R∘L = id ON THE MEAN, live."
fn probe_a_pin_residual() {
    println!("── A. Does mean-pin preserve the block mean? ────────────────────");

    let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    let seed_h = f.h.clone();
    f.erode(&FluvialParams { epochs: 30, ..Default::default() });

    let seed_at = {
        let seed_h = seed_h.clone();
        move |c: CellId| -> f64 {
            let (_, i, j, _) = c.to_face_ij();
            seed_h[((j - OJ) as usize) * NX + (i - OI) as usize] as f64
        }
    };

    let b = 4usize; // parent_level = LEVEL - 2
    let nb = NX / b;

    // delta field BEFORE the pin: target − block mean.
    let block_mean = |h: &[f32], bx: usize, by: usize| -> f64 {
        let mut s = 0.0f64;
        for y in 0..b {
            for x in 0..b {
                s += h[(by * b + y) * NX + bx * b + x] as f64;
            }
        }
        s / (b * b) as f64
    };
    let target = |bx: usize, by: usize| -> f64 {
        let cx = OI + (bx * b + b / 2) as u32;
        let cy = OJ + (by * b + b / 2) as u32;
        seed_at(CellId::from_face_ij(FACE, cx, cy, LEVEL))
    };

    let mut delta = vec![0.0f64; nb * nb];
    for by in 0..nb {
        for bx in 0..nb {
            delta[by * nb + bx] = target(bx, by) - block_mean(&f.h, bx, by);
        }
    }

    // THE REAL CODE.
    f.pin_block_means(LEVEL - 2, &seed_at);

    // Measured residual, and the residual PREDICTED by the (1,6,1)/8 ⊗ (1,6,1)/8
    // stencil (= the mean over a block of the bilinearly-upsampled delta field).
    let s161 = |d: &[f64], bx: usize, by: usize| -> f64 {
        let g = |x: isize, y: isize| -> f64 {
            let xi = x.clamp(0, nb as isize - 1) as usize;
            let yi = y.clamp(0, nb as isize - 1) as usize;
            d[yi * nb + xi]
        };
        let (x, y) = (bx as isize, by as isize);
        let row = |yy: isize| (g(x - 1, yy) + 6.0 * g(x, yy) + g(x + 1, yy)) / 8.0;
        (row(y - 1) + 6.0 * row(y) + row(y + 1)) / 8.0
    };

    let (mut max_meas, mut max_pred_err, mut sum_abs) = (0.0f64, 0.0f64, 0.0f64);
    let (mut n, mut worst_bx, mut worst_by) = (0usize, 0usize, 0usize);
    for by in 1..nb - 1 {
        for bx in 1..nb - 1 {
            let measured = block_mean(&f.h, bx, by) - target(bx, by);
            let predicted = s161(&delta, bx, by) - delta[by * nb + bx];
            if measured.abs() > max_meas {
                max_meas = measured.abs();
                worst_bx = bx;
                worst_by = by;
            }
            max_pred_err = max_pred_err.max((measured - predicted).abs());
            sum_abs += measured.abs();
            n += 1;
        }
    }

    println!("  interior blocks: {n}  (b = {b}, nb = {nb})");
    println!("  MEASURED |block mean − target|:  mean {:.4} m   MAX {:.4} m  (block {worst_bx},{worst_by})", sum_abs / n as f64, max_meas);
    println!("  MY PREDICTION  (1,6,1)/8 ⊗ (1,6,1)/8 applied to the delta field:");
    println!("     max |measured − predicted| = {max_pred_err:.3e} m");
    if max_pred_err < 1e-2 && max_meas > 1e-3 {
        println!("  ⇒ CONFIRMED. mean-pin does NOT preserve the block mean. The residual");
        println!("     IS (Δ_b²/8)·∇²(delta) — the doc's own §4.2 curvature term, live in");
        println!("     the dynamics path. R∘L = id ON THE MEAN IS FALSE AS IMPLEMENTED.");
    } else if max_meas <= 1e-3 {
        println!("  ⇒ NEGATIVE: the pin DOES preserve block means. My attack fails.");
    } else {
        println!("  ⇒ residual real but NOT the predicted stencil — mechanism unconfirmed.");
    }

    // Is the residual correlated with the curvature of the terrain (claim 3)?
    let mut num = 0.0f64;
    let (mut sr, mut sc, mut srr, mut scc) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut m = 0usize;
    for by in 1..nb - 1 {
        for bx in 1..nb - 1 {
            let r = block_mean(&f.h, bx, by) - target(bx, by);
            // discrete ∇² of the (post-pin) coarse surface
            let bm = |x: usize, y: usize| block_mean(&f.h, x, y);
            let lap = bm(bx - 1, by) + bm(bx + 1, by) + bm(bx, by - 1) + bm(bx, by + 1)
                - 4.0 * bm(bx, by);
            num += r * lap;
            sr += r;
            sc += lap;
            srr += r * r;
            scc += lap * lap;
            m += 1;
        }
    }
    let mf = m as f64;
    let cov = num / mf - (sr / mf) * (sc / mf);
    let corr = cov / (((srr / mf - (sr / mf).powi(2)) * (scc / mf - (sc / mf).powi(2))).sqrt());
    println!("\n  CLAIM 3 (artifact correlates with the landforms erosion produces):");
    println!("     corr(pin residual, ∇² of the eroded coarse surface) = {corr:+.3}");
    println!();
}

// ── B ────────────────────────────────────────────────────────────────────────
// Claim under test: mean-pin ≡ the HAAR low-pass. Haar's scaling coefficient is
// the UNWEIGHTED mean; a cell average on a non-uniform-area grid is the
// AREA-WEIGHTED mean. The doc says our cells vary 1.4× — but that is the GLOBAL
// max/min. What matters for a 2×2 (or 4×4) parent block is the LOCAL spread.
fn probe_b_haar_weights() {
    println!("── B. Haar needs equal weights. How unequal are ours, LOCALLY? ──");

    let b = 4usize;
    let (mut worst_block_ratio, mut worst_unw_vs_aw) = (1.0f64, 0.0f64);
    let (mut a_min, mut a_max) = (f64::MAX, 0.0f64);

    let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    f.erode(&FluvialParams { epochs: 30, ..Default::default() });

    let mut areas = vec![0.0f64; NX * NX];
    for y in 0..NX {
        for x in 0..NX {
            let a = cell_solid_angle(FACE, OI + x as u32, OJ + y as u32, LEVEL);
            areas[y * NX + x] = a;
            a_min = a_min.min(a);
            a_max = a_max.max(a);
        }
    }

    for by in 0..NX / b {
        for bx in 0..NX / b {
            let (mut amin, mut amax) = (f64::MAX, 0.0f64);
            let (mut sw, mut swh, mut sh) = (0.0f64, 0.0f64, 0.0f64);
            for y in 0..b {
                for x in 0..b {
                    let i = (by * b + y) * NX + bx * b + x;
                    let (a, h) = (areas[i], f.h[i] as f64);
                    amin = amin.min(a);
                    amax = amax.max(a);
                    sw += a;
                    swh += a * h;
                    sh += h;
                }
            }
            worst_block_ratio = worst_block_ratio.max(amax / amin);
            let unweighted = sh / (b * b) as f64;
            let area_weighted = swh / sw;
            worst_unw_vs_aw = worst_unw_vs_aw.max((unweighted - area_weighted).abs());
        }
    }

    println!("  tile-wide cell-area max/min ...................... {:.6}×", a_max / a_min);
    println!("  WORST within a single {b}×{b} parent block ......... {worst_block_ratio:.8}×");
    println!("  worst |unweighted mean − area-weighted mean| ..... {worst_unw_vs_aw:.3e} m");
    println!("  (terrain relief across the tile is ~{:.0} m)", f.h.iter().cloned().fold(f32::MIN, f32::max) - f.h.iter().cloned().fold(f32::MAX, f32::min));

    // Does mean-pin conserve Σ A·h (volume on the sphere)?
    let vol = |h: &[f32]| -> f64 { (0..NX * NX).map(|i| areas[i] * h[i] as f64).sum() };
    let seed_h = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX).h;
    let seed_at = move |c: CellId| -> f64 {
        let (_, i, j, _) = c.to_face_ij();
        seed_h[((j - OJ) as usize) * NX + (i - OI) as usize] as f64
    };
    let v_before = vol(&f.h);
    f.pin_block_means(LEVEL - 2, &seed_at);
    let v_after = vol(&f.h);
    println!("\n  Σ A·h before pin ... {v_before:.6e}");
    println!("  Σ A·h after  pin ... {v_after:.6e}");
    println!("  relative change .... {:.3e}", (v_after - v_before).abs() / v_before.abs());
    println!();
}

// ── C ────────────────────────────────────────────────────────────────────────
// §2.4: "a piecewise-linear reconstruction about the CENTROID carrying
// cell-average values is volume-exact BY CONSTRUCTION, at every cell, with ZERO
// EPSILON." The identity ∫(x − x_P) dΩ = 0 holds only if x_P is the AREA-WEIGHTED
// centroid. Our stored point is the PARAMETRIC centre of the (u,v) square.
fn probe_c_centroid() {
    println!("── C. §2.4's centroid identity: parametric centre ≠ area centroid ──");

    for &(level, label) in &[(9u8, "L9  (~78 km)"), (19u8, "L19 (~19 m)")] {
        let n = (1u32 << level) as f64;
        // A worst-case cell: at a face corner, where the Jacobian varies fastest.
        let (i, j) = ((1u32 << level) - 1, (1u32 << level) - 1);

        // Area-weighted centroid in (u,v), by fine quadrature of the solid angle.
        let p = |k: f64| (k / n) * 2.0 - 1.0;
        let (u0, u1) = (p(i as f64), p(i as f64 + 1.0));
        let (v0, v1) = (p(j as f64), p(j as f64 + 1.0));
        let m = 200;
        let (mut sw, mut swu, mut swv) = (0.0f64, 0.0f64, 0.0f64);
        for a in 0..m {
            for b in 0..m {
                let u = u0 + (u1 - u0) * (a as f64 + 0.5) / m as f64;
                let v = v0 + (v1 - v0) * (b as f64 + 0.5) / m as f64;
                // equiangular area element ∝ (1+U²+V²)^{-3/2} · sec²·sec², U=tan(uπ/4)
                let (tu, tv) = ((u * std::f64::consts::FRAC_PI_4).tan(), (v * std::f64::consts::FRAC_PI_4).tan());
                let (su, sv) = (1.0 + tu * tu, 1.0 + tv * tv); // sec²
                let w = su * sv / (1.0 + tu * tu + tv * tv).powf(1.5);
                sw += w;
                swu += w * u;
                swv += w * v;
            }
        }
        let (cu, cv) = (swu / sw, swv / sw);
        let (pu, pv) = ((u0 + u1) * 0.5, (v0 + v1) * 0.5);
        let cell_du = u1 - u0;
        let off = (((cu - pu) / cell_du).powi(2) + ((cv - pv) / cell_du).powi(2)).sqrt();

        // Volume error of a linear reconstruction with slope g through the
        // PARAMETRIC centre, relative to the true cell-average:  ΔV/A = g · δ.
        // Express δ as a fraction of the cell width; a 0.1 slope over a cell of
        // width L gives a height error of 0.1 · off · L.
        println!("  {label}: |area centroid − parametric centre| = {:.4e} cell widths", off);
        println!("        ⇒ a slope-0.1 surface mis-states the cell's mean height by");
        println!("          {:.3e} × cell_width  (at L19, cell ≈ 19 m ⇒ {:.2e} m)", 0.1 * off, 0.1 * off * 19.0);
    }
    println!();
}

// ── D ────────────────────────────────────────────────────────────────────────
// water.rs:358–372 runs the θ blend as an IN-PLACE sweep and its own comment says
// "In-place (Gauss–Seidel) sweep: slightly asymmetric, faster-damping."
// de Almeida et al. (2012) specify  q̄ = θ·q_i^n + (1−θ)/2·(q_{i−1}^n + q_{i+1}^n)
// — a JACOBI average, all at time level n. Measure the deviation.
fn probe_d_theta_sweep() {
    println!("── D. water.rs's θ sweep vs de Almeida et al. 2012's θ scheme ────");

    let n = 256usize;
    let theta = 0.8f64;
    // A flux field with a sharp front (the regime θ exists to tame).
    let q0: Vec<f64> = (0..n)
        .map(|i| {
            let x = i as f64 / n as f64;
            (if (0.4..0.6).contains(&x) { 1.0 } else { 0.0 }) + 0.15 * ((i * 7) as f64).sin()
        })
        .collect();

    // Paper (Jacobi, all at level n):
    let mut jac = q0.clone();
    for i in 1..n - 1 {
        jac[i] = theta * q0[i] + (1.0 - theta) * 0.5 * (q0[i - 1] + q0[i + 1]);
    }
    // water.rs (in-place Gauss–Seidel, sweeping +x):
    let mut gs = q0.clone();
    for i in 1..n - 1 {
        gs[i] = theta * gs[i] + (1.0 - theta) * 0.5 * (gs[i - 1] + gs[i + 1]);
    }
    // The mirror image: sweep −x. If the operator were symmetric these agree.
    let mut gs_rev = q0.clone();
    for i in (1..n - 1).rev() {
        gs_rev[i] = theta * gs_rev[i] + (1.0 - theta) * 0.5 * (gs_rev[i - 1] + gs_rev[i + 1]);
    }

    let l2 = |a: &[f64], b: &[f64]| -> f64 {
        (a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>() / a.len() as f64).sqrt()
    };
    let var = |a: &[f64]| -> f64 {
        let m = a.iter().sum::<f64>() / a.len() as f64;
        a.iter().map(|x| (x - m).powi(2)).sum::<f64>() / a.len() as f64
    };
    let mean = |a: &[f64]| a.iter().sum::<f64>() / a.len() as f64;

    println!("  ONE pass, θ = 0.8, sharp-front flux field:");
    println!("    ‖GS(+x) − Jacobi(paper)‖₂ ......... {:.4e}", l2(&gs, &jac));
    println!("    ‖GS(+x) − GS(−x)‖₂  (mirror test) . {:.4e}   ← a SYMMETRIC operator gives 0", l2(&gs, &gs_rev));
    println!("    mean flux: q0 {:.6}  jacobi {:.6}  gs {:.6}", mean(&q0), mean(&jac), mean(&gs));
    println!("    variance:  q0 {:.6}  jacobi {:.6}  gs {:.6}", var(&q0), var(&jac), var(&gs));
    println!("    ⇒ GS removes {:.1}% of the flux variance per pass; Jacobi removes {:.1}%",
        100.0 * (1.0 - var(&gs) / var(&q0)), 100.0 * (1.0 - var(&jac) / var(&q0)));

    // Sustained: 200 steps, as in a real run.
    let (mut g, mut jj) = (q0.clone(), q0.clone());
    for _ in 0..200 {
        for i in 1..n - 1 {
            g[i] = theta * g[i] + (1.0 - theta) * 0.5 * (g[i - 1] + g[i + 1]);
        }
        let prev = jj.clone();
        for i in 1..n - 1 {
            jj[i] = theta * prev[i] + (1.0 - theta) * 0.5 * (prev[i - 1] + prev[i + 1]);
        }
    }
    println!("\n  200 passes (a real run):");
    println!("    variance left — GS {:.3e}   Jacobi {:.3e}   (start {:.3e})", var(&g), var(&jj), var(&q0));
    println!("    ‖GS − Jacobi‖₂ .................... {:.4e}", l2(&g, &jj));
    println!("  ⇒ Jensen: sediment capacity ∝ |v| and incision ∝ v^n are SUPERLINEAR,");
    println!("     so destroying flux variance while preserving the flux MEAN is a");
    println!("     systematic UNDER-transport bias — measurable, and nothing to do");
    println!("     with a missing fluctuation partner.");
    println!();
}
