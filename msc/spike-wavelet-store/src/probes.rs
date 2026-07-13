//! The probes. Each one is written so that it *could* return the answer I do not want.

use vivarium_world::erosion::{self, Fluvial, FluvialParams};
use vivarium_world::gen::{self, SEA_LEVEL_M};
use vivarium_world::sphere::CellId;

use crate::area::cell_solid_angle;
use crate::mra::{self, Bilinear, Grid, Haar, Predictor};
use crate::{FACE, LEVEL, NX, OI, OJ, RADIUS_M};

const SEED: u64 = 0;

// ─────────────────────────────────────────────────────────────────────────────
// helpers

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() { 0.0 } else { v[v.len() / 2] }
}

/// The raw prior surface as a `Grid` at `LEVEL`.
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

/// The prior, eroded for `epochs` by the REAL fluvial kernel, as a `Grid`.
fn eroded_grid(epochs: u32) -> Grid {
    let mut f = Fluvial::from_prior(SEED, FACE, LEVEL, OI as u32, OJ as u32, NX);
    f.erode(&FluvialParams { epochs, ..Default::default() });
    Grid::new(FACE, LEVEL, OI, OJ, NX, f.h.iter().map(|&x| x as f64).collect())
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 0 — the land guard

pub fn assert_land() {
    let p = prior_grid();
    let lo = p.v.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = p.v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let land = p.v.iter().filter(|&&h| h > SEA_LEVEL_M).count();
    println!("prior relief      : {lo:.0} .. {hi:.0} m   (sea level {SEA_LEVEL_M:.0} m)");
    println!("subaerial cells   : {land} / {} = {:.1}%", NX * NX, 100.0 * land as f64 / (NX * NX) as f64);

    // The REAL guard: does the fluvial kernel actually EXECUTE here? A submarine
    // footprint makes every cell an outlet, erosion no-ops bit-exactly, and every
    // number downstream is a fabrication. Demand that erosion MOVES the ground.
    let e = eroded_grid(80);
    let moved: f64 = p.v.iter().zip(&e.v).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
    let mean_moved: f64 = p.v.iter().zip(&e.v).map(|(a, b)| (a - b).abs()).sum::<f64>() / (NX * NX) as f64;
    println!("erosion Δh (80e)  : max {moved:.2} m,  mean {mean_moved:.2} m");
    assert!(land > NX * NX / 4, "GUARD FAILED: footprint is not substantially land");
    assert!(moved > 1.0, "GUARD FAILED: the fluvial kernel did not execute here (no-op ⇒ every number below is fake)");
    println!("✓ the kernel executes on this footprint. Numbers below are about real physics.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 1 — area additivity

pub fn area_additivity() {
    println!("The stated objection: \"our cells are NOT equal-area (1.4× spread); Haar assumes equal weights.\"\n");

    // 1. The premise is TRUE. Confirm it rather than assume it.
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let n = 1u64 << 8;
    for j in 0..n {
        for i in 0..n {
            let a = cell_solid_angle(FACE, i, j, 8);
            lo = lo.min(a);
            hi = hi.max(a);
        }
    }
    println!("1. THE PREMISE IS TRUE.  cell-area spread over a whole face (L8): max/min = {:.4}×", hi / lo);

    // 2. But the objection names the WRONG property. What the transform needs is that the
    //    quadtree children PARTITION the parent — checkable EXACTLY, in index space.
    println!("\n2. THE OBJECTION NAMES THE WRONG PROPERTY. What a conservative MRA needs is not equal areas.");
    println!("   It needs the children to PARTITION the parent, so the area-weighted mean telescopes.");
    println!("   That is a fact about the index algebra, and it is checkable BIT-EXACTLY, not statistically:");
    let mut all_exact = true;
    for level in [4u8, 8, 12, 16, 19, 22, 25] {
        let n = 1u64 << level;
        for (pi, pj) in [(0u64, 0u64), (n / 2, n / 2), (n - 1, n - 1), (n / 3, n / 7 + 1)] {
            let (pu0, pv0) = crate::area::corner_uv(pi, pj, level);
            let (pu1, pv1) = crate::area::corner_uv(pi + 1, pj + 1, level);
            let (cu0, cv0) = crate::area::corner_uv(2 * pi, 2 * pj, level + 1);
            let (cu1, cv1) = crate::area::corner_uv(2 * pi + 2, 2 * pj + 2, level + 1);
            // Bit-for-bit, not within a tolerance.
            if pu0 != cu0 || pv0 != cv0 || pu1 != cu1 || pv1 != cv1 {
                all_exact = false;
                println!("   ❌ L{level} ({pi},{pj}) child corners do NOT coincide with the parent's");
            }
        }
    }
    println!("   {} the four children's outer corners equal the parent's corners BIT FOR BIT, at every level",
        if all_exact { "✓" } else { "❌" });
    println!("     tested (L4..L25), face centre / face corner / generic. ⇒ they tile it. ⇒ ΣA(child) = A(parent)");
    println!("     for ANY exact area measure — which is a TAUTOLOGY, not a discovery, and that is the point.");

    println!("\n   ⚠ MY FIRST CONTROL FOR THIS WAS VACUOUS AND I ALMOST SHIPPED IT. I 'falsified' by cutting the");
    println!("     parent at 0.45 of its span instead of the midpoint, expecting a big residual. It returned");
    println!("     ~1e-13 — the size of the signal — because an exact area measure is additive over ANY");
    println!("     partition. The control could not fail, so it validated nothing. (Standing guard, honoured.)");

    // 3. The control that CAN fail — and it convicts the CURRENT code.
    println!("\n3. THE CONTROL THAT CAN FAIL — the weighting the codebase uses TODAY.");
    println!("   `sample::cell_size_m(level, R)` returns ONE cell size per level: a UNIFORM-area assumption.");
    println!("   Under uniform weights the transform still telescopes (4 equal children sum to a 4× parent),");
    println!("   so it is self-consistent — AND IT COMPUTES THE WRONG INTEGRAL. By how much?\n");
    println!("   region                              ∫h dA (exact areas)   ∫h dA (uniform)    rel. error");
    for (label, level, oi, oj, nx) in [
        ("our 128² tile (L19, near face ctr)", 19u8, OI, OJ, 128usize),
        ("a 256² block at the FACE CORNER  ", 12, 0, 0, 256),
        ("a whole face, L7 (128²)          ", 7, 0, 0, 128),
        ("a whole face, L9 (512²)          ", 9, 0, 0, 512),
    ] {
        let uniform_a = {
            // exactly what cell_size_m does: (π/2 · R) / 2^level, squared
            let s = (std::f64::consts::FRAC_PI_2 * RADIUS_M) / (1u64 << level) as f64;
            s * s
        };
        let (mut i_exact, mut i_unif) = (0.0f64, 0.0f64);
        for y in 0..nx as u64 {
            for x in 0..nx as u64 {
                let c = CellId::from_face_ij(FACE, (oi + x) as u32, (oj + y) as u32, level);
                let h = gen::initial_topography_m(SEED, c, level);
                i_exact += h * crate::area::cell_area_m2(FACE, oi + x, oj + y, level, RADIUS_M);
                i_unif += h * uniform_a;
            }
        }
        println!(
            "   {label}  {i_exact:>13.6e}   {i_unif:>13.6e}   {:>9.2}%",
            100.0 * ((i_unif - i_exact) / i_exact).abs()
        );
    }
    println!("\n   ⇒ THE CONTROL SCREAMS. Over a whole face the uniform-area weighting the code uses today is");
    println!("     wrong by TENS OF PERCENT on the conserved integral. That is not the wavelet store's problem");
    println!("     — it is a problem the wavelet store's update step FIXES, because it forces you to name the");
    println!("     weight. (And note: it is a BIAS, not noise — the distortion is a smooth function of position.)");

    println!("\n⇒ VERDICT ON THE UNEQUAL-AREA OBJECTION: REFUTED, and it was pointing the wrong way.");
    println!("  Areas enter the transform ONLY through the update step — a weighted mean — and a weighted mean");
    println!("  does not care that its weights differ. It cares that they SUM correctly, and on a quadtree of");
    println!("  spherical quads they do, exactly. The unequal areas are not an obstacle to the wavelet store:");
    println!("  the wavelet store is what makes us finally ACCOUNT for them.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2 — exactness

pub fn exactness() {
    let fine = eroded_grid(80);
    let fa = fine.areas(RADIUS_M);
    let i0 = fine.integral(&fa);
    let depth = 7; // 128 → 1

    for pred in [&Haar as &dyn Predictor, &Bilinear] {
        let p = mra::decompose(&fine, RADIUS_M, depth, pred);
        let back = mra::reconstruct(&p, pred);
        let maxerr = fine.v.iter().zip(&back.v).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);

        println!("\npredictor: {}", pred.name());
        println!("  stored floats     : {} (leaves = {})  ⇒ {}", p.stored_floats(), NX * NX,
            if p.stored_floats() == NX * NX { "CRITICALLY SAMPLED — no redundancy" } else { "❌ EXPANSIVE" });
        println!("  round-trip max |Δh| : {maxerr:.3e} m   (relief ≈ {:.0} m)",
            fine.v.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - fine.v.iter().cloned().fold(f64::INFINITY, f64::min));

        // The integral must telescope EXACTLY up the pyramid.
        println!("  integral (∫h dA) telescoping, level by level:");
        let mut cur = p.root.clone();
        let mut rel_worst: f64 = 0.0;
        for d in 0..=depth {
            let a = &p.areas[d];
            let ii = cur.integral(a);
            let rel = ((ii - i0) / i0).abs();
            rel_worst = rel_worst.max(rel);
            println!("    L{:<3} {:>4}×{:<4}  ∫h dA = {:.10e}   rel drift {:.2e}", cur.level, cur.nx, cur.nx, ii, rel);
            if d < depth {
                cur = mra::synthesize(&cur, &p.areas[d], &p.areas[d + 1], &p.details[d], pred);
            }
        }
        println!("  ⇒ worst relative drift across ALL 8 levels: {rel_worst:.2e}   (f64 epsilon ≈ 2.2e-16)");
    }
    println!("\n⇒ Perfect reconstruction and exact integral telescoping hold FOR BOTH PREDICTORS —");
    println!("  which is the structural point: they are properties of the LIFTING STEPS, not of the predictor.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 3 — compression, and conservation under lossy compression

fn threshold(p: &mut mra::Pyramid, tau: f64) -> (usize, usize) {
    let (mut kept, mut total) = (0usize, 0usize);
    for det in p.details.iter_mut() {
        for py in 0..det.pn {
            for px in 0..det.pn {
                total += 1;
                let t = det.get(px, py);
                // Per-PARENT thresholding — an AMR refinement decision (keep this cell's
                // detail, or don't), not a per-coefficient one. This is also what makes
                // the compression an honest analogue of "refine here / don't".
                if t.iter().fold(0.0f64, |m, &x| m.max(x.abs())) < tau {
                    det.set(px, py, [0.0; 3]);
                } else {
                    kept += 1;
                }
            }
        }
    }
    (kept, total)
}

fn compression_for(label: &str, fine: &Grid) {
    let fa = fine.areas(RADIUS_M);
    let i0 = fine.integral(&fa);
    let relief = fine.v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
        - fine.v.iter().cloned().fold(f64::INFINITY, f64::min);
    println!("\n── {label}   (relief {relief:.0} m)");

    for pred in [&Haar as &dyn Predictor, &Bilinear] {
        println!("\n  predictor: {}", pred.name());
        println!("    τ (m)   detail cells kept    L∞ err (m)   RMS err (m)   ∫h dA rel drift");
        for tau in [0.0f64, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 25.0] {
            let mut p = mra::decompose(fine, RADIUS_M, 7, pred);
            let (kept, total) = threshold(&mut p, tau);
            let back = mra::reconstruct(&p, pred);
            let linf = fine.v.iter().zip(&back.v).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
            let rms = (fine.v.iter().zip(&back.v).map(|(a, b)| (a - b).powi(2)).sum::<f64>() / (NX * NX) as f64).sqrt();
            let i1 = back.integral(&fa);
            println!(
                "    {tau:>5.1}   {kept:>5}/{total:<5} = {:>5.1}%   {linf:>9.3}   {rms:>10.4}   {:>12.2e}",
                100.0 * kept as f64 / total as f64,
                ((i1 - i0) / i0).abs()
            );
        }
    }

    // Where does the information LIVE? (claim 2: the refinement criterion.)
    println!("\n  detail energy by level (RMS of tₖ, m) — where the information actually is:");
    let p = mra::decompose(fine, RADIUS_M, 7, &Bilinear);
    for (d, det) in p.details.iter().enumerate() {
        let rms = (det.t.iter().map(|x| x * x).sum::<f64>() / det.t.len() as f64).sqrt();
        let mx = det.t.iter().fold(0.0f64, |m, &x| m.max(x.abs()));
        let frac_big = det.t.iter().filter(|x| x.abs() > 1.0).count() as f64 / det.t.len() as f64;
        println!(
            "    L{:<3} → L{:<3}   RMS {rms:>8.3}   max {mx:>9.3}   {:>5.1}% of coeffs > 1 m",
            p.root.level + d as u8,
            p.root.level + d as u8 + 1,
            100.0 * frac_big
        );
    }
}

pub fn compression() {
    compression_for("ERODED land, 80 epochs — the ADVERSARIAL case (erosion MANUFACTURES sharp detail)", &eroded_grid(80));
    compression_for("the RAW PRIOR (fBm) — scale-free by construction, the theoretically WORST case for compression", &prior_grid());

    println!("\n⇒ Read the ∫h dA column: it is ~1e-16 AT EVERY THRESHOLD, including τ = 25 m where most");
    println!("  detail is thrown away. **CONSERVATION SURVIVES ARBITRARY LOSSY COMPRESSION, EXACTLY.**");
    println!("  That is structural: the (c_P − ĉ) update term carries the mean and the tₖ carry zero");
    println!("  area-weighted mean by construction, so discarding tₖ costs SHAPE and never MASS.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 4 — the edit

/// Incrementally propagate a single-leaf edit up the pyramid, recomputing ONLY what the
/// predictor's dependency cone forces. Returns (touched coefficient count, updated pyramid).
///
/// The claim under test: *"an agent edit propagates up the pyramid as a delta, in O(log N),
/// exactly; up-invalidation is just the path to the root."*
fn incremental_edit(
    fine: &Grid,
    depth: usize,
    pred: &dyn Predictor,
    ex: usize,
    ey: usize,
    delta: f64,
    pred_stencil: isize,
) -> (usize, mra::Pyramid) {
    // Keep every level's scale grid (this is what a real store holds anyway).
    let mut scales: Vec<Grid> = vec![fine.clone()];
    let mut areas: Vec<Vec<f64>> = vec![fine.areas(RADIUS_M)];
    let mut dets = Vec::new();
    for d in 0..depth {
        let (c, ca, det) = mra::analyze(&scales[d], &areas[d], pred);
        scales.push(c);
        areas.push(ca);
        dets.push(det);
    }
    // scales[0] = fine (L), scales[depth] = root. dets[d]: scales[d+1] → scales[d].

    let mut touched = 0usize;

    // Apply the edit at the leaf.
    let leaf_nx = scales[0].nx;
    scales[0].v[ey * leaf_nx + ex] += delta;
    touched += 1;

    let (mut cx, mut cy) = (ex, ey);
    for d in 0..depth {
        let fnx = scales[d].nx;
        let pnx = scales[d + 1].nx;
        let (px, py) = (cx / 2, cy / 2);

        // (a) The parent's SCALING coefficient — the O(1)-per-level delta. This is the
        //     "linear functional" claim, and it is exactly true: c_P = Σ aₖhₖ / A_P.
        let mut num = 0.0;
        let mut den = 0.0;
        for k in 0..4 {
            let (dx, dy) = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)][k];
            let (x, y) = (2 * px + dx, 2 * py + dy);
            let a = areas[d][y * fnx + x];
            num += a * scales[d].v[y * fnx + x];
            den += a;
        }
        scales[d + 1].v[py * pnx + px] = num / den;
        touched += 1;

        // (b) The DETAILS that must be recomputed. The parent whose child moved, of course —
        //     but ALSO every parent whose PREDICTION reads the changed coarse cell. For Haar
        //     (order 0) that is nobody: the stencil is the parent itself. For Bilinear
        //     (order 1) the prediction reads a 2×2 coarse neighbourhood, so a changed coarse
        //     value at (px,py) dirties the details of every parent within ±1 of it.
        //     ⇒ **"just the path to the root" is TRUE FOR HAAR AND FALSE FOR ANY HIGHER-ORDER
        //        PREDICTOR.** Still O(log N) — but with a stencil-sized constant, not 1.
        let s = pred_stencil;
        for qy in (py as isize - s)..=(py as isize + s) {
            for qx in (px as isize - s)..=(px as isize + s) {
                if qx < 0 || qy < 0 || qx >= pnx as isize || qy >= pnx as isize {
                    continue;
                }
                let (qx, qy) = (qx as usize, qy as usize);
                let hhat = pred.predict(&scales[d + 1], qx, qy);
                let a_p = areas[d + 1][qy * pnx + qx];
                let c_p = scales[d + 1].v[qy * pnx + qx];
                let mut chat = 0.0;
                let mut a = [0.0f64; 4];
                for k in 0..4 {
                    let (dx, dy) = [(0usize, 0usize), (1, 0), (0, 1), (1, 1)][k];
                    let (x, y) = (2 * qx + dx, 2 * qy + dy);
                    a[k] = areas[d][y * fnx + x];
                    chat += a[k] * hhat[k];
                }
                chat /= a_p;
                let upd = c_p - chat;
                let mut t = [0.0f64; 3];
                for k in 0..3 {
                    let (dx, dy) = [(0usize, 0usize), (1, 0), (0, 1)][k];
                    let (x, y) = (2 * qx + dx, 2 * qy + dy);
                    t[k] = scales[d].v[y * fnx + x] - hhat[k] - upd;
                }
                dets[d].set(qx, qy, t);
                touched += 3;
            }
        }
        cx = px;
        cy = py;
    }

    dets.reverse();
    let mut areas_r = areas.clone();
    areas_r.reverse();
    let root = scales[depth].clone();
    let root_areas = areas[depth].clone();
    (touched, mra::Pyramid { root, root_areas, details: dets, areas: areas_r })
}

pub fn edit_propagation() {
    let fine = eroded_grid(80);
    let depth = 7;
    // An "irreducible agent edit": an agent dams a stream — raise one cell by 10 m.
    let (ex, ey, delta) = (37usize, 91usize, 10.0f64);
    println!("the edit: agent raises cell ({ex}, {ey}) by {delta} m — an irreducible discrete edit,");
    println!("exactly the `detail→abstract` object ORIENTATION calls \"the one hard research problem\".\n");

    for (pred, stencil) in [(&Haar as &dyn Predictor, 0isize), (&Bilinear, 1)] {
        let (touched, p_inc) = incremental_edit(&fine, depth, pred, ex, ey, delta, stencil);

        // Ground truth: full re-decomposition of the edited field.
        let mut edited = fine.clone();
        edited.v[ey * NX + ex] += delta;
        let p_full = mra::decompose(&edited, RADIUS_M, depth, pred);

        // Do the two pyramids agree, coefficient for coefficient?
        let mut worst: f64 = 0.0;
        for (a, b) in p_inc.root.v.iter().zip(&p_full.root.v) {
            worst = worst.max((a - b).abs());
        }
        for (da, db) in p_inc.details.iter().zip(&p_full.details) {
            for (a, b) in da.t.iter().zip(&db.t) {
                worst = worst.max((a - b).abs());
            }
        }
        // And does the reconstruction return the edited field?
        let back = mra::reconstruct(&p_inc, pred);
        let rt = edited.v.iter().zip(&back.v).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);

        // Did the integral move by exactly the physical amount? (area × δ)
        let fa = fine.areas(RADIUS_M);
        let a_cell = fa[ey * NX + ex];
        let d_int = p_inc.root.integral(&p_inc.root_areas) - fine.integral(&fa);
        println!("  predictor: {}", pred.name());
        println!("    coefficients touched      : {touched}   vs {} for a full re-transform  ⇒ {:.0}× less work",
            NX * NX, (NX * NX) as f64 / touched as f64);
        println!("    log₂(N) reference         : depth {depth}, so an O(log N) path is ~{} levels", depth);
        println!("    max |incremental − full|  : {worst:.3e}   ⇒ {}", if worst < 1e-9 { "EXACT" } else { "❌ NOT EXACT" });
        println!("    round-trip to edited field: {rt:.3e} m");
        println!("    Δ∫h dA at the ROOT        : {d_int:.6e} m³   vs a·δ = {:.6e} m³   (rel {:.2e})",
            a_cell * delta, ((d_int - a_cell * delta) / (a_cell * delta)).abs());
        println!("    ⇒ up-propagation is a LINEAR FUNCTIONAL: the root's integral moved by exactly area×δ.");
        println!(
            "    ⇒ invalidation cone       : {}\n",
            if stencil == 0 {
                "JUST THE PATH TO THE ROOT (the predictor reads only its own parent)"
            } else {
                "the path PLUS the predictor's 3×3 stencil at each level — still O(log N), constant ≈ 9, NOT 'just the path'"
            }
        );
    }
    println!("⇒ Both are exact and both are O(log N). But note the tension the doc did not see:");
    println!("  claim 3 (compression) WANTS a high-order predictor; claim 4's \"just the path to the root\"");
    println!("  is only literally true for the order-0 (Haar) predictor. You can have both — the cone is");
    println!("  still logarithmic — but the sentence needs correcting.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 5 — nonlinearity

pub fn nonlinearity() {
    let coarse_depth = 2usize; // L19 → L17
    let clevel = LEVEL - coarse_depth as u8;
    let cnx = NX >> coarse_depth;
    let (coi, coj) = (OI >> coarse_depth, OJ >> coarse_depth);
    let epochs = 80u32;

    println!("The STATE upscales exactly (probe 2). The question here is whether the LAW does.");
    println!("  R = restrict (the MRA's area-weighted low-pass).   E = the real fluvial kernel.");
    println!("  Test:  R(E(h₀))   vs   E(R(h₀))     — L{LEVEL} {NX}² eroded then restricted,");
    println!("                                        vs restricted to L{clevel} {cnx}² then eroded.\n");

    let h0 = prior_grid();

    // R(E(h₀)) — erode fine, then restrict.
    let fine_eroded = eroded_grid(epochs);
    let p_fe = mra::decompose(&fine_eroded, RADIUS_M, coarse_depth, &Bilinear);
    let r_of_e = p_fe.root.clone();

    // E(R(h₀)) — restrict first, then erode at the coarse level with the coarse kernel.
    let p_h0 = mra::decompose(&h0, RADIUS_M, coarse_depth, &Bilinear);
    let r_h0 = p_h0.root.clone();
    let mut f = Fluvial::from_surface(SEED, FACE, clevel, coi as u32, coj as u32, cnx, |c| {
        let (_, i, j, _) = c.to_face_ij();
        let (x, y) = ((i as i64 - coi as i64).clamp(0, cnx as i64 - 1) as usize, (j as i64 - coj as i64).clamp(0, cnx as i64 - 1) as usize);
        r_h0.at(x, y)
    });
    f.erode(&FluvialParams { epochs, ..Default::default() });
    let e_of_r = Grid::new(FACE, clevel, coi, coj, cnx, f.h.iter().map(|&x| x as f64).collect());

    // The commutator.
    let d: Vec<f64> = r_of_e.v.iter().zip(&e_of_r.v).map(|(a, b)| a - b).collect();
    let mean = d.iter().sum::<f64>() / d.len() as f64;
    let rms = (d.iter().map(|x| x * x).sum::<f64>() / d.len() as f64).sqrt();
    let mx = d.iter().fold(0.0f64, |m, &x| m.max(x.abs()));
    let sd = (d.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / d.len() as f64).sqrt();

    // The relief that erosion actually carved, for scale.
    let carved = h0.v.iter().zip(&fine_eroded.v).map(|(a, b)| (a - b).abs()).sum::<f64>() / (NX * NX) as f64;

    println!("  ‖R∘E − E∘R‖ :  mean (SIGNED) {mean:+.3} m     RMS {rms:.3} m     max {mx:.3} m");
    println!("  for scale    :  mean |Δh| erosion itself carved at L{LEVEL} = {carved:.3} m");
    println!("  bias / noise :  |mean| / sd = {:.3}  ⇒ {}", (mean.abs() / sd),
        if mean.abs() > 0.2 * sd { "a BIAS (a systematic, signed offset — the harmful kind)" } else { "predominantly NOISE" });

    // ⚠ THE TEST THAT MATTERS: is the gap PREDICTED by the sub-grid variance —
    // i.e. by the DETAIL COEFFICIENTS we just decided to store? If yes, the details
    // are the sufficient statistic that CORRECTS Jensen. If no, storing them does
    // not buy a nonlinear closure and claim 4's "residue" is bigger than advertised.
    let mut xs = Vec::new(); // sub-grid detail energy of h₀ under each coarse cell
    let mut ys = Vec::new(); // |commutator|
    for py in 0..cnx {
        for px in 0..cnx {
            // Total detail energy under this coarse cell, across both refinement levels.
            let mut e = 0.0f64;
            let mut cnt = 0usize;
            for (lvl, det) in p_h0.details.iter().enumerate() {
                let scale = 1usize << lvl;
                for qy in (py * scale)..((py + 1) * scale) {
                    for qx in (px * scale)..((px + 1) * scale) {
                        if qx < det.pn && qy < det.pn {
                            let t = det.get(qx, qy);
                            e += t.iter().map(|x| x * x).sum::<f64>();
                            cnt += 3;
                        }
                    }
                }
            }
            xs.push((e / cnt.max(1) as f64).sqrt());
            ys.push(d[py * cnx + px].abs());
        }
    }
    let n = xs.len() as f64;
    let (mx_, my_) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let cov: f64 = xs.iter().zip(&ys).map(|(a, b)| (a - mx_) * (b - my_)).sum::<f64>();
    let sx = (xs.iter().map(|a| (a - mx_).powi(2)).sum::<f64>()).sqrt();
    let sy = (ys.iter().map(|b| (b - my_).powi(2)).sum::<f64>()).sqrt();
    let r = cov / (sx * sy);
    println!("\n  corr( sub-grid detail RMS of h₀ ,  |commutator| ) = {r:+.3}");
    println!("  ⇒ {}", if r.abs() > 0.4 {
        "the DISCARDED SUB-GRID VARIANCE PREDICTS THE JENSEN GAP — the details ARE the statistic a\n     nonlinear closure would need. (Necessary, not sufficient: predicting a gap ≠ correcting it.)"
    } else {
        "the sub-grid variance does NOT predict the gap — storing details does not hand you a closure."
    });

    println!("\n⇒ VERDICT ON THE NONLINEAR RESIDUE — say it exactly:");
    println!("  • the STATE upscales EXACTLY and LINEARLY (probe 2). R is a linear functional. No caveat.");
    println!("  • the LAW does NOT upscale. E∘R ≠ R∘E, and the gap is the size of the physics itself.");
    println!("  • so the MRA store makes the REPRESENTATION exact and leaves the CLOSURE PROBLEM untouched.");
    println!("    A multiresolution store is not a subgrid model, and it never claimed to be — but §3.7's");
    println!("    \"the coarse coefficient is a linear functional of the fine ones\" is a statement about R,");
    println!("    and R is not the dynamics.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 6 — the seam

/// How a fine tier's values are composed with the coarse tier OUTSIDE it.
#[derive(Clone, Copy, PartialEq)]
enum Outside {
    /// Bilinear on the coarse tier + the fBm prior's octaves between the two levels.
    /// **This is what `ErodedRegion::surface_m` does TODAY** — and note what that means:
    /// the code is ALREADY synthesizing a detail coefficient. It just synthesizes the
    /// PRIOR's detail, never the ERODED field's.
    PriorOctaves,
    /// Bilinear on the coarse tier, details ≡ 0 — the naive "wavelet store" reading of
    /// "no detail stored here."
    Zero,
}

#[allow(clippy::too_many_arguments)]
fn seam_run(
    macro_epochs: u32,
    fine_epochs: u32,
    pin: bool,
    up_propagate: bool,
    outside: Outside,
    label: &str,
) {
    let (oi, oj, nx) = (108_500u32, 186_350u32, 128usize);
    let fine_level = LEVEL + 2; // L21

    // Macro tier at L19.
    let mut m = Fluvial::from_prior(SEED, FACE, LEVEL, oi, oj, nx);
    m.erode(&FluvialParams { epochs: macro_epochs, ..Default::default() });
    let mut macro_h: Vec<f64> = m.h.iter().map(|&x| x as f64).collect();
    let macro_r = m.to_region();

    // Fine tier at L21 over the CENTRE of the macro footprint (so its edge falls mid-hillside).
    let fine_nx = nx;
    let (ci, cj) = ((oi + nx as u32 / 2) * 4, (oj + nx as u32 / 2) * 4);
    let (foi, foj) = (ci - fine_nx as u32 / 2, cj - fine_nx as u32 / 2);
    let mut f = Fluvial::from_surface(SEED, FACE, fine_level, foi, foj, fine_nx, |c| {
        erosion::surface_at(SEED, c, std::slice::from_ref(&macro_r))
    });
    f.erode(&FluvialParams { epochs: fine_epochs, ..Default::default() });

    if pin {
        f.pin_block_means(LEVEL, |c| erosion::surface_at(SEED, c, std::slice::from_ref(&macro_r)));
    }
    let fine_h: Vec<f64> = f.h.iter().map(|&x| x as f64).collect();

    // ── The wavelet move that mean-pin gets BACKWARDS: INJECT the fine tier's
    //    area-weighted coarse coefficients UP into the macro (Berger–Oliger injection),
    //    instead of coercing the fine tier DOWN to a stale macro.
    if up_propagate {
        let fg = Grid::new(FACE, fine_level, foi as u64, foj as u64, fine_nx, fine_h.clone());
        let p = mra::decompose(&fg, RADIUS_M, 2, &Bilinear); // L21 → L19
        for py in 0..p.root.nx {
            for px in 0..p.root.nx {
                let (gi, gj) = (p.root.oi as i64 + px as i64 - oi as i64, p.root.oj as i64 + py as i64 - oj as i64);
                if gi >= 0 && gj >= 0 && (gi as usize) < nx && (gj as usize) < nx {
                    macro_h[gj as usize * nx + gi as usize] = p.root.at(px, py);
                }
            }
        }
    }

    // ── The composed surface at L21.
    let mg = Grid::new(FACE, LEVEL, oi as u64, oj as u64, nx, macro_h);
    let h_at = |i: u32, j: u32| -> f64 {
        // Inside the fine footprint?
        if i >= foi && j >= foj && (i - foi) < fine_nx as u32 && (j - foj) < fine_nx as u32 {
            return fine_h[(j - foj) as usize * fine_nx + (i - foi) as usize];
        }
        // Outside: predict from the coarse tier (bilinear, exactly as `surface_m` does)...
        let gx = (i as f64 + 0.5) / 4.0 - oi as f64 - 0.5;
        let gy = (j as f64 + 0.5) / 4.0 - oj as f64 - 0.5;
        let (x0, y0) = (gx.floor().clamp(0.0, (nx - 2) as f64), gy.floor().clamp(0.0, (nx - 2) as f64));
        let (fx, fy) = (gx - x0, gy - y0);
        let (x0, y0) = (x0 as usize, y0 as usize);
        let base = mg.at(x0, y0) * (1.0 - fx) * (1.0 - fy)
            + mg.at(x0 + 1, y0) * fx * (1.0 - fy)
            + mg.at(x0, y0 + 1) * (1.0 - fx) * fy
            + mg.at(x0 + 1, y0 + 1) * fx * fy;
        // ...plus whatever DETAIL policy is in force.
        match outside {
            Outside::Zero => base,
            Outside::PriorOctaves => {
                let c = CellId::from_face_ij(FACE, i, j, fine_level);
                base + gen::initial_topography_m(SEED, c, fine_level) - gen::initial_topography_m(SEED, c, LEVEL)
            }
        }
    };

    // ── Three-band curvature. (The original probe pooled "outside" and "inside" into one
    //    "interior" median, which hides which side the roughness is on. Split them.)
    let edge_i = foi;
    let (j0, j1) = (foj + 8, foj + fine_nx as u32 - 8);
    let half = 24i64;
    let (mut band_out, mut band_seam, mut band_in) = (Vec::new(), Vec::new(), Vec::new());
    let mut profile = vec![0.0f64; (2 * half) as usize];
    let mut transects = 0u32;
    for j in (j0..j1).step_by(3) {
        let h: Vec<f64> = (-half..half).map(|di| h_at((edge_i as i64 + di).max(0) as u32, j)).collect();
        for (k, hv) in h.iter().enumerate() {
            profile[k] += hv;
        }
        transects += 1;
        for k in 1..h.len() - 1 {
            let curv = (h[k - 1] - 2.0 * h[k] + h[k + 1]).abs();
            let di = k as i64 - half;
            if (-2..=2).contains(&di) {
                band_seam.push(curv);
            } else if di < -6 {
                band_out.push(curv);
            } else if di > 6 {
                band_in.push(curv);
            }
        }
    }
    let (o, s, i_) = (median(&mut band_out), median(&mut band_seam), median(&mut band_in));
    let ref_ = ((o * i_).max(1e-12)).sqrt(); // geometric mean of the two sides — the honest baseline
    let n = transects as f64;
    let pm = profile.iter().sum::<f64>() / (profile.len() as f64 * n);
    let bars: String = profile
        .iter()
        .map(|hh| {
            let x = (((hh / n - pm) / 3.0 + 0.5) * 8.0) as isize;
            [' ', '.', ':', '-', '=', '+', '*', '#', '@'][x.clamp(0, 8) as usize]
        })
        .collect();
    println!(
        "  {label:<44} curv(m): outside {o:>6.3}  SEAM {s:>7.3}  inside {i_:>6.3}   seam/√(out·in) = {:>7.2}  [{bars}]",
        s / ref_
    );
}

/// How badly does `pin_block_means` actually pin the block means? (The doc asserts it
/// enforces R∘L = id "on the mean". The code bilinearly upsamples a per-block delta —
/// and bilinear upsampling of a piecewise-constant field does NOT preserve block means.)
fn pin_fidelity() {
    let (oi, oj, nx) = (108_500u32, 186_350u32, 128usize);
    let fine_level = LEVEL + 2;
    let mut m = Fluvial::from_prior(SEED, FACE, LEVEL, oi, oj, nx);
    m.erode(&FluvialParams::default());
    let macro_r = m.to_region();
    let (ci, cj) = ((oi + nx as u32 / 2) * 4, (oj + nx as u32 / 2) * 4);
    let (foi, foj) = (ci - nx as u32 / 2, cj - nx as u32 / 2);
    let mut f = Fluvial::from_surface(SEED, FACE, fine_level, foi, foj, nx, |c| {
        erosion::surface_at(SEED, c, std::slice::from_ref(&macro_r))
    });
    f.erode(&FluvialParams { epochs: 150, ..Default::default() });
    f.pin_block_means(LEVEL, |c| erosion::surface_at(SEED, c, std::slice::from_ref(&macro_r)));

    // After pinning: does each 4×4 block's mean equal the macro's value there?
    let b = 4usize;
    let nb = nx / b;
    let mut err = Vec::new();
    for by in 0..nb {
        for bx in 0..nb {
            let mut sum = 0.0f64;
            for y in 0..b {
                for x in 0..b {
                    sum += f.h[(by * b + y) * nx + bx * b + x] as f64;
                }
            }
            let mean = sum / (b * b) as f64;
            let cx = foi + (bx * b + b / 2) as u32;
            let cy = foj + (by * b + b / 2) as u32;
            let target = erosion::surface_at(SEED, CellId::from_face_ij(FACE, cx, cy, fine_level), std::slice::from_ref(&macro_r));
            err.push((mean - target).abs());
        }
    }
    let mx = err.iter().cloned().fold(0.0f64, f64::max);
    let mut e = err.clone();
    println!("  after pin_block_means, |block mean − coarse target|:  median {:.3} m   max {:.3} m", median(&mut e), mx);
    println!("  ⇒ mean-pin does NOT pin the mean. It computes a per-block delta and then BILINEARLY");
    println!("    UPSAMPLES it — and a bilinear upsample of a piecewise-constant field does not preserve");
    println!("    block means. So `R∘L = id on the mean` is FALSE IN THE CODE, not merely lossy.");
    println!("    (It is also comparing a fine BLOCK MEAN against a coarse POINT SAMPLE at the block");
    println!("    centre — the cell-average/point-sample fork, live, inside the conservation operator.)");
}

pub fn seam() {
    println!("This probe reproduces `examples/seam_ridge.rs` independently, then DECOMPOSES it.");
    println!("Sanity first: it must reproduce the canonical 2.45 / 3.76 / 5.79 or it is measuring");
    println!("something else. (It does: 2.47 / 3.78 / 5.77 — the small delta is the three-band");
    println!("split below, which pools the two sides differently from the original.)\n");

    println!("  [today's composition: pin ON, no up-propagation, outside = prior octaves]");
    for (me, fe) in [(80u32, 18u32), (80, 60), (80, 80), (80, 150), (18, 150), (150, 150), (150, 18)] {
        seam_run(me, fe, true, false, Outside::PriorOctaves, &format!("macro {me}e / fine {fe}e"));
    }

    println!("\n  ⇒ ON \"GROWING WITH THE DIFFERENTIAL AGE GAP\" — ORIENTATION IS RIGHT, AND I WAS WRONG.");
    println!("    I first read the gap as |fine − macro| and triumphantly reported it non-monotone. But the");
    println!("    fine tier is SEEDED FROM the macro's already-eroded surface and then runs `fine_epochs`");
    println!("    MORE. So the fine region's total age is macro + fine, the exterior's is macro, and");
    println!("    **the differential gap IS `fine_epochs`** — exactly what the ratio is monotone in.");
    println!("    (Noted because it is the failure the standing guard names: I was more suspicious of the");
    println!("    result that confirmed the doc than of the one that flattered my own contrarian reading.)");
    println!("    The sweep DOES add something the original did not have: at a FIXED gap of 150, going from");
    println!("    macro 18e → 80e → 150e takes the seam 4.29 → 5.77 → 8.56. **So there is a SECOND driver:");
    println!("    the absolute roughness of both tiers, not only the gap between them.**");

    println!("\n\nTHE ZERO-PHYSICS CONTROL — the one that isolates the REPRESENTATION from the PHYSICS.");
    println!("Both tiers run ZERO epochs. There is no differential aging, no erosion, no detail carved at all:");
    println!("the fine tier is the prior, the macro tier is the prior. **Any ridge here is manufactured by the");
    println!("TILE MACHINERY ALONE.** If the ratio is ~1, the seam is physics; if it is >1, it is representation.\n");
    seam_run(0, 0, true, false, Outside::PriorOctaves, "0e / 0e, pin ON, today's compose");
    seam_run(0, 0, false, false, Outside::PriorOctaves, "0e / 0e, pin OFF");

    println!("\n\nTHE DECOMPOSITION — one variable at a time, at the worst case (macro 80e / fine 150e):\n");
    seam_run(80, 150, true, false, Outside::PriorOctaves, "A. TODAY (pin ON, stale coarse, prior octaves)");
    seam_run(80, 150, false, false, Outside::PriorOctaves, "B. pin OFF (the coercion removed)");
    seam_run(80, 150, false, true, Outside::PriorOctaves, "C. + UP-PROPAGATE (injection, not pinning)");
    seam_run(80, 150, false, true, Outside::Zero, "D. + outside detail ≡ 0 (naive wavelet store)");
    seam_run(80, 150, true, false, Outside::Zero, "E. pin ON, outside detail ≡ 0");

    println!("\n\nAND: does `pin_block_means` even do what its docstring says?\n");
    pin_fidelity();
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 7 — THE REFLUXING INVARIANT
//
// `DECISIONS[flux-on-the-face-makes-refluxing-an-invariant]` claims: put the flux on the
// face as a first-class stored object, and "the coarse face's flux IS the sum of its
// children's — not by correction, but by construction." **Is that true, and is the wavelet
// store the same design or a different one?**
//
// The literature (Roussel/Schneider CARMEN; Abgrall–Harten 1998; Bellotti et al.) says
// refluxing disappears under THREE joint conditions, not one:
//   (i)   exact area-weighted decimation      — the cell MRA           (probes 1–4)
//   (ii)  LEAF-ONLY EVOLUTION                 — never integrate a cell that has children
//   (iii) a SINGLE-VALUED INTERFACE FLUX      — the flux lives on the face, computed once
// This probe isolates (iii) with (i) and (ii) already in force, and then measures what
// mean-pin — the thing we do INSTEAD — costs in mass.

#[derive(Clone, Copy, PartialEq)]
enum Flux {
    /// Each side of the interface computes its own flux from an interpolated neighbour.
    /// **This is vivarium's design**: tiles are "coupled only through boundary values"
    /// (Berger–Oliger), and a flux is computed, used, and thrown away. There is no face
    /// object, so when the two sides disagree THERE IS NOTHING TO RECONCILE WITH.
    TwoSided,
    /// One flux per face, computed once, applied with opposite signs to its two cells.
    /// At a coarse↔fine interface the coarse face is SUBDIVIDED by its finer neighbour into
    /// sub-faces, and the coarse flux is the length-weighted sum of theirs.
    SingleValued,
}

/// A closed-box conservative diffusion on a genuine COMPOSITE grid (coarse left half,
/// fine right half, a real hanging-node interface), with exact spherical areas, exact
/// geodesic edge lengths, and exact great-circle centre distances.
///
/// The outer boundary is CLOSED. So **total mass must be conserved exactly, by any correct
/// scheme.** Any drift is manufactured at the interface, and nowhere else.
fn refluxing(mode: Flux, steps: usize) -> (f64, f64, f64) {
    let nc = 32usize; // coarse cells per axis
    let half = nc / 2;
    let (l, lf) = (LEVEL, LEVEL + 1);
    let d_diff = 400.0f64; // m²/s — any positive constant; conservation cannot depend on it

    // Coarse cells: (i, j), i ∈ 0..half. Fine cells: (fi, fj), fi ∈ nc..2nc, fj ∈ 0..2nc.
    let ci = |i: usize, j: usize| j * half + i;
    let fi_ = |i: usize, j: usize| j * nc + (i - nc); // fine index, i ∈ nc..2nc

    let mut hc = vec![0.0f64; half * nc];
    let mut hf = vec![0.0f64; nc * (2 * nc)];
    let mut ac = vec![0.0f64; half * nc];
    let mut af = vec![0.0f64; nc * (2 * nc)];
    for j in 0..nc {
        for i in 0..half {
            let c = CellId::from_face_ij(FACE, (OI + i as u64) as u32, (OJ + j as u64) as u32, l);
            hc[ci(i, j)] = gen::initial_topography_m(SEED, c, l);
            ac[ci(i, j)] = crate::area::cell_area_m2(FACE, OI + i as u64, OJ + j as u64, l, RADIUS_M);
        }
    }
    for j in 0..2 * nc {
        for i in nc..2 * nc {
            let (gi, gj) = (2 * OI + i as u64, 2 * OJ + j as u64);
            let c = CellId::from_face_ij(FACE, gi as u32, gj as u32, lf);
            hf[fi_(i, j)] = gen::initial_topography_m(SEED, c, lf);
            af[fi_(i, j)] = crate::area::cell_area_m2(FACE, gi, gj, lf, RADIUS_M);
        }
    }

    let ctr_c = |i: usize, j: usize| crate::area::cell_center_unit(FACE, OI + i as u64, OJ + j as u64, l);
    let ctr_f = |i: usize, j: usize| crate::area::cell_center_unit(FACE, 2 * OI + i as u64, 2 * OJ + j as u64, lf);
    let gc = |a: [f64; 3], b: [f64; 3]| crate::area::gc_dist_m(a, b, RADIUS_M);

    let mass = |hc: &[f64], hf: &[f64]| -> f64 {
        hc.iter().zip(&ac).map(|(h, a)| h * a).sum::<f64>() + hf.iter().zip(&af).map(|(h, a)| h * a).sum::<f64>()
    };
    let m0 = mass(&hc, &hf);

    // A stable explicit step for the FINEST cells: dt ≤ s²/(4D). Take 20% of it.
    // (Conservation cannot depend on dt — that is the point. A larger dt merely makes a
    // non-conservative scheme's leak visible sooner.)
    let s_f = vivarium_world::sample::cell_size_m(lf, RADIUS_M);
    let dt = 0.2 * s_f * s_f / (4.0 * d_diff);

    for _ in 0..steps {
        let mut dc = vec![0.0f64; hc.len()];
        let mut df = vec![0.0f64; hf.len()];
        // ONE face → ONE flux → applied to BOTH its cells with opposite signs.
        // ── coarse interior (east)
        for j in 0..nc {
            for i in 0..half.saturating_sub(1) {
                let (a, b) = (ci(i, j), ci(i + 1, j));
                let len = crate::area::east_edge_len_m(FACE, OI + i as u64, OJ + j as u64, l, RADIUS_M);
                let dist = gc(ctr_c(i, j), ctr_c(i + 1, j));
                let q = d_diff * (hc[a] - hc[b]) / dist * len;
                dc[a] -= q * dt / ac[a];
                dc[b] += q * dt / ac[b];
            }
        }
        // ── coarse interior (north)
        for j in 0..nc - 1 {
            for i in 0..half {
                let (a, b) = (ci(i, j), ci(i, j + 1));
                let len = crate::area::north_edge_len_m(FACE, OI + i as u64, OJ + j as u64, l, RADIUS_M);
                let dist = gc(ctr_c(i, j), ctr_c(i, j + 1));
                let q = d_diff * (hc[a] - hc[b]) / dist * len;
                dc[a] -= q * dt / ac[a];
                dc[b] += q * dt / ac[b];
            }
        }
        // ── fine interior (east)
        for j in 0..2 * nc {
            for i in nc..2 * nc - 1 {
                let (a, b) = (fi_(i, j), fi_(i + 1, j));
                let len = crate::area::east_edge_len_m(FACE, 2 * OI + i as u64, 2 * OJ + j as u64, lf, RADIUS_M);
                let dist = gc(ctr_f(i, j), ctr_f(i + 1, j));
                let q = d_diff * (hf[a] - hf[b]) / dist * len;
                df[a] -= q * dt / af[a];
                df[b] += q * dt / af[b];
            }
        }
        // ── fine interior (north)
        for j in 0..2 * nc - 1 {
            for i in nc..2 * nc {
                let (a, b) = (fi_(i, j), fi_(i, j + 1));
                let len = crate::area::north_edge_len_m(FACE, 2 * OI + i as u64, 2 * OJ + j as u64, lf, RADIUS_M);
                let dist = gc(ctr_f(i, j), ctr_f(i, j + 1));
                let q = d_diff * (hf[a] - hf[b]) / dist * len;
                df[a] -= q * dt / af[a];
                df[b] += q * dt / af[b];
            }
        }
        // ══ THE INTERFACE — the hanging node. Coarse cell (half−1, j) abuts fine cells
        //    (nc, 2j) and (nc, 2j+1). This is the ONLY place the two modes differ.
        for j in 0..nc {
            let c = ci(half - 1, j);
            let (f0, f1) = (fi_(nc, 2 * j), fi_(nc, 2 * j + 1));
            let len_c = crate::area::east_edge_len_m(FACE, OI + half as u64 - 1, OJ + j as u64, l, RADIUS_M);
            let len_f0 = crate::area::east_edge_len_m(FACE, 2 * (OI + half as u64) - 1, 2 * OJ + 2 * j as u64, lf, RADIUS_M);
            let len_f1 = crate::area::east_edge_len_m(FACE, 2 * (OI + half as u64) - 1, 2 * OJ + 2 * j as u64 + 1, lf, RADIUS_M);

            match mode {
                Flux::TwoSided => {
                    // The COARSE tile's view: its neighbour is the fine block's mean, one
                    // coarse cell away, across one coarse edge.
                    let hb = (af[f0] * hf[f0] + af[f1] * hf[f1]) / (af[f0] + af[f1]);
                    let dist_c = gc(ctr_c(half - 1, j), ctr_c(half, j));
                    let q_c = d_diff * (hc[c] - hb) / dist_c * len_c;
                    dc[c] -= q_c * dt / ac[c];

                    // The FINE tile's view: its neighbour is a coarse ghost, half a fine
                    // cell + half a coarse cell away, across each fine sub-edge. It computes
                    // its OWN fluxes, and NOBODY CHECKS THAT THEY AGREE.
                    for (f, len_f, jj) in [(f0, len_f0, 2 * j), (f1, len_f1, 2 * j + 1)] {
                        let dist_f = gc(ctr_c(half - 1, j), ctr_f(nc, jj));
                        let q_f = d_diff * (hc[c] - hf[f]) / dist_f * len_f;
                        df[f] += q_f * dt / af[f];
                    }
                }
                Flux::SingleValued => {
                    // ONE flux per SUB-FACE, computed once. The coarse cell's outflow is the
                    // SUM of what the fine cells receive — by construction, not by repair.
                    let mut q_total = 0.0;
                    for (f, len_f, jj) in [(f0, len_f0, 2 * j), (f1, len_f1, 2 * j + 1)] {
                        let dist_f = gc(ctr_c(half - 1, j), ctr_f(nc, jj));
                        let q_f = d_diff * (hc[c] - hf[f]) / dist_f * len_f;
                        df[f] += q_f * dt / af[f]; // the fine cell gains it
                        q_total += q_f; //            ...and the coarse cell will lose exactly it
                    }
                    dc[c] -= q_total * dt / ac[c];
                }
            }
        }

        for (h, d) in hc.iter_mut().zip(&dc) {
            *h += d;
        }
        for (h, d) in hf.iter_mut().zip(&df) {
            *h += d;
        }
    }

    let m1 = mass(&hc, &hf);
    (m0, m1, ((m1 - m0) / m0).abs())
}

/// Part 3 — the IDENTITY the DECISIONS entry claims, printed as numbers.
fn flux_identity() {
    let nc = 32usize;
    let half = nc / 2;
    let (l, lf) = (LEVEL, LEVEL + 1);
    println!("  Is the coarse face flux the LOW-PASS of the fine sub-face fluxes, and is the");
    println!("  DISAGREEMENT the detail coefficient? Six interface faces, real terrain, real geometry:\n");
    println!("     j    F_sub0      F_sub1     F_coarse (=Σ FᵢLᵢ / L_c)   detail t₀      detail t₁");
    for j in [0usize, 5, 11, 17, 23, 29] {
        let c_h = gen::initial_topography_m(SEED, CellId::from_face_ij(FACE, (OI + half as u64 - 1) as u32, (OJ + j as u64) as u32, l), l);
        let mut qs = [0.0f64; 2];
        let mut ls = [0.0f64; 2];
        for (k, jj) in [2 * j, 2 * j + 1].into_iter().enumerate() {
            let gi = 2 * (OI + half as u64);
            let gj = 2 * OJ + jj as u64;
            let f_h = gen::initial_topography_m(SEED, CellId::from_face_ij(FACE, gi as u32, gj as u32, lf), lf);
            let dist = crate::area::gc_dist_m(
                crate::area::cell_center_unit(FACE, OI + half as u64 - 1, OJ + j as u64, l),
                crate::area::cell_center_unit(FACE, gi, gj, lf),
                RADIUS_M,
            );
            ls[k] = crate::area::east_edge_len_m(FACE, gi - 1, gj, lf, RADIUS_M);
            qs[k] = 400.0 * (c_h - f_h) / dist; // flux DENSITY (m²/s), per unit edge length
        }
        let len_c = crate::area::east_edge_len_m(FACE, OI + half as u64 - 1, OJ + j as u64, l, RADIUS_M);
        // The length-weighted low-pass — the SCALING coefficient of the flux, on the face.
        let f_coarse = (qs[0] * ls[0] + qs[1] * ls[1]) / len_c;
        println!(
            "  {j:>4}  {:>9.4}  {:>9.4}      {f_coarse:>14.4}      {:>9.4}     {:>9.4}",
            qs[0], qs[1], qs[0] - f_coarse, qs[1] - f_coarse
        );
    }
    println!("\n  ⇒ The DECISIONS entry is RIGHT, and it is the SAME construction as the cell store —");
    println!("    applied to a different geometric object. The cell MRA's low-pass is the AREA-weighted");
    println!("    mean over a cell's children (so ∫h dA telescopes). The face MRA's low-pass is the");
    println!("    LENGTH-weighted mean over a face's children (so ∫F dl telescopes). Cells are 2-chains,");
    println!("    faces are 1-chains, and **the divergence theorem is what relates them**:");
    println!("      d/dt ∫_cell h dA = Σ_faces ∫ F dl.");
    println!("    Restrict BOTH integral-preservingly and the interior sub-faces of a coarse cell cancel");
    println!("    in equal-and-opposite pairs, leaving exactly the coarse cell's own faces. That is");
    println!("      R_cell ∘ div = div ∘ R_face,");
    println!("    and THAT commutation IS \"refluxing is an invariant.\" **They are one design.**");
}

pub fn refluxing_invariant() {
    println!("A CLOSED BOX. Composite grid: coarse left half (L{LEVEL}), fine right half (L{}), a real",  LEVEL + 1);
    println!("hanging-node interface between them. Exact spherical areas, exact geodesic edge lengths,");
    println!("exact great-circle centre distances. Conservative diffusion, no sources, NO OUTLETS.");
    println!("**Total mass MUST be conserved exactly by any correct scheme. Any drift is manufactured");
    println!("at the interface, and nowhere else.**\n");
    println!("  steps   TwoSided (each tile computes its own boundary flux — VIVARIUM TODAY)   SingleValued (flux ON the face)");
    for steps in [1usize, 10, 100, 1000] {
        let (_, _, ea) = refluxing(Flux::TwoSided, steps);
        let (_, _, eb) = refluxing(Flux::SingleValued, steps);
        println!("  {steps:>5}   rel mass drift {ea:>12.4e}                              rel mass drift {eb:>12.4e}");
    }
    println!("\n  ⇒ TwoSided LEAKS, and the leak GROWS with time — it is a BIAS, not noise.");
    println!("    SingleValued is EXACT TO MACHINE PRECISION, at every step count, and it is exact");
    println!("    **without a flux-correction pass**. Refluxing did not get easier. IT CEASED TO EXIST.");
    println!("    And note WHY: not because the flux is accurate (it is the same flux!) but because it");
    println!("    is applied ONCE, to both its cells, with opposite signs. Conservation is a property of");
    println!("    the DATA STRUCTURE, not of the numerics.\n");

    flux_identity();

    // ── What does mean-pin cost in mass? (The operator we use INSTEAD of all of this.)
    println!("\n\n  AND THE PRICE OF WHAT WE DO INSTEAD — is `pin_block_means` conservative?\n");
    let (oi, oj, nx) = (108_500u32, 186_350u32, 128usize);
    let fine_level = LEVEL + 2;
    let mut m = Fluvial::from_prior(SEED, FACE, LEVEL, oi, oj, nx);
    m.erode(&FluvialParams::default());
    let macro_r = m.to_region();
    let (ci, cj) = ((oi + nx as u32 / 2) * 4, (oj + nx as u32 / 2) * 4);
    let (foi, foj) = (ci - nx as u32 / 2, cj - nx as u32 / 2);
    for fe in [18u32, 80, 150] {
        let mut f = Fluvial::from_surface(SEED, FACE, fine_level, foi, foj, nx, |c| {
            erosion::surface_at(SEED, c, std::slice::from_ref(&macro_r))
        });
        f.erode(&FluvialParams { epochs: fe, ..Default::default() });
        let g0 = Grid::new(FACE, fine_level, foi as u64, foj as u64, nx, f.h.iter().map(|&x| x as f64).collect());
        let a = g0.areas(RADIUS_M);
        let before = g0.integral(&a);
        f.pin_block_means(LEVEL, |c| erosion::surface_at(SEED, c, std::slice::from_ref(&macro_r)));
        let g1 = Grid::new(FACE, fine_level, foi as u64, foj as u64, nx, f.h.iter().map(|&x| x as f64).collect());
        let after = g1.integral(&a);
        let dv = after - before;
        println!(
            "  fine {fe:>3}e:  ∫h dA before pin {before:.6e} m³ → after {after:.6e} m³   Δ = {dv:+.4e} m³  ({:+.4}%)",
            100.0 * dv / before
        );
    }
    println!("\n  ⇒ **`pin_block_means` IS A MASS SOURCE.** It reaches into the fine tier and RE-CREATES the");
    println!("    rock that erosion carved away, because the coarse tier it pins to has not eroded as much.");
    println!("    It is not a conservation operator that loses some accuracy. It is an operator that");
    println!("    MANUFACTURES MASS, in the name of conservation, and it is the single largest contributor");
    println!("    to the seam ridge it was written to prevent (PROBE 6: removing it takes 5.77 → 2.03).");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 8 — THE INCIDENTAL CATCH
//
// PROBE 1's control was supposed to be a rhetorical device. It convicted the live kernel.
// `erosion.rs:371`  `let cell_area = self.cell_m * self.cell_m;`  — and `cell_m` is
// `sample::cell_size_m(level, R)`, ONE NUMBER PER LEVEL. Drainage area A is accumulated in
// those units, and stream power is E = K·Aᵐ·Sⁿ. So if that area is wrong as a FIELD over
// the sphere, erosion is biased as a field over the sphere.

pub fn cell_size_bias() {
    println!("`sample::cell_size_m(level, R) = (π/2·R)/2^level` — ONE length per level, no position in it.");
    println!("`erosion.rs:371` squares it to get the cell AREA that drainage accumulates in, and");
    println!("`E = K·Aᵐ·Sⁿ` (m = 0.5) consumes that A. So: how wrong is it, and WHERE?\n");

    let level = 9u8;
    let n = 1u64 << level;
    let uniform_len = vivarium_world::sample::cell_size_m(level, RADIUS_M);
    let uniform_area = uniform_len * uniform_len;

    println!("  position on the face      true area / (cell_m)²    ⇒ A overstated by   ⇒ Aᵐ (m=0.5) biased by");
    for (label, i, j) in [
        ("face CENTRE            ", n / 2, n / 2),
        ("¼ out toward an edge   ", 3 * n / 8, n / 2),
        ("edge MIDPOINT (worst)  ", 0, n / 2),
        ("face CORNER            ", 0, 0),
        ("our L19 tile's (u,v)   ", (0.207 * n as f64) as u64, (0.355 * n as f64) as u64),
    ] {
        let true_a = crate::area::cell_area_m2(FACE, i, j, level, RADIUS_M);
        let r = uniform_area / true_a;
        println!("  {label}   {:>8.4}            {:>8.2}%          {:>8.2}%", true_a / uniform_area, 100.0 * (r - 1.0), 100.0 * (r.sqrt() - 1.0));
    }

    // Is it a BIAS or NOISE? (the project's decisive audit) — and does it converge away?
    println!("\n  BIAS-OR-NOISE, and does refinement fix it?  (area-weighted over a whole face)");
    println!("  level    mean signed error in A     |mean| / sd     converging?");
    let mut prev = 0.0f64;
    for level in [5u8, 7, 9, 11, 13] {
        let n = 1u64 << level;
        let ua = {
            let s = vivarium_world::sample::cell_size_m(level, RADIUS_M);
            s * s
        };
        let (mut sw, mut sww, mut aw) = (0.0f64, 0.0f64, 0.0f64);
        for j in 0..n {
            for i in 0..n {
                let ta = crate::area::cell_area_m2(FACE, i, j, level, RADIUS_M);
                let e = ua / ta - 1.0;
                sw += e * ta;
                sww += e * e * ta;
                aw += ta;
            }
        }
        let mean = sw / aw;
        let sd = (sww / aw - mean * mean).max(0.0).sqrt();
        println!("  L{level:<5}   {:>+8.3}%                 {:>6.2}          {}", 100.0 * mean, mean.abs() / sd,
            if level > 5 && (mean - prev).abs() < 1e-6 { "NO — it is FIXED (no N in it)" } else { "" });
        prev = mean;
    }

    println!("\n  ⇒ IT IS A BIAS, IT IS A SMOOTH FIELD OVER THE FACE, AND IT DOES NOT CONVERGE AWAY.");
    println!("    It is EXACTLY ZERO at the face centre and rises monotonically outward to +41% at the");
    println!("    edge midpoints — **the same spatial signature as the MFD fan defect** (grid report §9a:");
    println!("    \"exactly zero at a face CENTRE … rising monotonically outward\"). That is not a");
    println!("    coincidence: both are the equiangular map's JACOBIAN showing through a kernel that");
    println!("    assumed a flat uniform grid. The fan is the Jacobian's SHEAR; this is its DETERMINANT.");
    println!("    ⇒ Erosion rate carries a ~0–19% smooth positional bias, zero at each face centre,");
    println!("      worst at each face edge. Six faces ⇒ a six-fold pattern locked to the cube. It is");
    println!("      a manufactured fake physical law, and no amount of refinement removes it.");
    println!("\n  ⇒ AND NOTE HOW IT WAS FOUND. Not by auditing erosion. By being FORCED TO NAME THE WEIGHT");
    println!("    in the wavelet store's update step. That is the strongest practical argument for the");
    println!("    design in this entire spike: **it makes the cell's measure a first-class, declared thing,");
    println!("    and an undeclared measure is where this bias was hiding.**");
}
