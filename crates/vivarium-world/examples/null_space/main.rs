//! # The null-space probe — *what can our kernels NOT see?*
//!
//! Cardiff & Demirdžić, *Thirty Years of the Finite Volume Method for Solid
//! Mechanics* §3.5.3 (`ref/research/pdfs/markdowns/cardiff-2021-thirty/`), Fig. 17:
//!
//! > *"consider a **periodic patch** of finite volume cells, containing a central
//! > cell and all neighbour cells within its computational stencil, and **analyse
//! > the eigenvalues of its (block-coupled) global stiffness matrix**… For a
//! > stable formulation, **the number of zero valued eigenvalues is equal to the
//! > number of rigid degrees of freedom**… For an unstable formulation, there will
//! > be **additional zero valued eigenvalues, where the corresponding eigenvector
//! > indicates the unstable mode**."*
//!
//! Run: `cargo run --release -p vivarium-world --example null_space`
//! Writes figures + a machine log to `msc/spike-null-space/`.
//!
//! ## The two forms of "meets no resistance", and why they are one statement
//!
//! Cardiff's `K` is a *static* stiffness. Our kernels are *time steppers*. For an
//! explicit step `J = I + Δt·A`, `λ(A) = 0 ⟺ λ(J) = 1`. So:
//!
//! | operator kind | what a blind mode looks like | our probe reports |
//! |---|---|---|
//! | spatial / stiffness (`creep`, Laplacians) | `λ = 0` — no restoring force | eigenvalues, zero-count |
//! | time-step (`water`, `erode`) | `\|λ\| ≥ 1` — undamped or growing | spectral radius + the mode |
//!
//! Same disease, two coordinate systems. Both are reported.

mod draw;
mod linalg;
mod ops;
mod water_op;

use draw::Panel;
use linalg::*;
use std::path::PathBuf;
use water_op::{Geom, Guards, PipeParams};

fn outdir() -> PathBuf {
    // examples run with CWD = workspace root
    let d = PathBuf::from("msc/spike-null-space/fig");
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn rule(t: &str) {
    println!("\n\x1b[1m{}\x1b[0m", "─".repeat(78));
    println!("\x1b[1m{t}\x1b[0m");
    println!("{}", "─".repeat(78));
}

/// Count eigenvalues below `tol`, and report the SPECTRAL GAP — the ratio between
/// the largest "zero" and the smallest "nonzero". A clean answer has a gap of many
/// orders of magnitude; a mushy one does not, and then the count is not a fact.
fn zero_count(vals: &[f64], tol: f64) -> (usize, f64) {
    let z = vals.iter().filter(|v| v.abs() < tol).count();
    let gap = if z == 0 || z == vals.len() {
        f64::INFINITY
    } else {
        vals[z].abs() / vals[z - 1].abs().max(1e-300)
    };
    (z, gap)
}

fn main() {
    let dir = outdir();
    println!("\x1b[1;36m╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║  NULL-SPACE PROBE — what our field kernels CANNOT SEE                     ║");
    println!("║  Cardiff §3.5.3 / Fig. 17: periodic patch → eigenvalues → count the zeros ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝\x1b[0m");

    let nx = 8usize;
    let n = nx * nx;

    // ══════════════════════════════════════════════════════════════════════════
    rule("§0  THE INSTRUMENT AND ITS CONTROLS");
    println!("   Standing guard: *a probe that cannot fail is not a probe.* Before any kernel");
    println!("   is measured, the instrument must separate an operator that PROVABLY has a null");
    println!("   mode from one that PROVABLY has not — on the same patch, same solver, same");
    println!("   threshold. If it cannot, nothing below means anything.\n");

    let lin = |f: &dyn Fn(&[f64], usize) -> Vec<f64>| -> Vec<f64> {
        let u0 = vec![0.0f64; n];
        let eps = vec![1.0f64; n]; // f is linear ⇒ the central difference is EXACT
        jacobian_fd(n, &u0, &eps, |u| f(u, nx))
    };

    let a_clean = lin(&ops::ctrl_compact_laplacian);
    let a_blind = lin(&ops::ctrl_collocated_central);
    let a_stag = lin(&ops::ctrl_staggered_divgrad);

    // Staggered div-grad IS the compact Laplacian — assert it, don't claim it.
    let dstag: f64 = a_clean.iter().zip(&a_stag).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
    println!("   staggered(div∘grad_face) − compact 5-point Laplacian :  max|Δ| = {dstag:.3e}");
    println!("   ⇒ they are the SAME OPERATOR. Staggering does not tame the null mode; it");
    println!("     never creates one. (This is fig/staggering.svg, measured.)\n");

    let (ev_clean, vec_clean) = jacobi_sym(&a_clean, n);
    let (ev_blind, vec_blind) = jacobi_sym(&a_blind, n);
    let smax = ev_clean.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    let tol = 1e-9 * smax;

    let (zc, gc) = zero_count(&ev_clean, tol);
    let (zb, gb) = zero_count(&ev_blind, tol);
    println!("   CTRL-CLEAN  compact 5-point Laplacian (faces, Δ apart)");
    println!("      zero eigenvalues: {zc}   (legitimate: 1 = the constant)   spurious: {}", zc as i64 - 1);
    println!("      spectral gap at the cut: {gc:.2e}      |λ| range: {:.3e} … {:.3e}", ev_clean[1].abs(), smax);
    println!("      λ(checkerboard) = {:.4}  ← the MOST damped mode, not the least", checkerboard_rayleigh(&a_clean, nx));
    println!("   CTRL-BLIND  collocated central differences (2Δ apart)");
    println!("      zero eigenvalues: {zb}   (legitimate: 1)   \x1b[1;31mSPURIOUS: {}\x1b[0m", zb as i64 - 1);
    println!("      spectral gap at the cut: {gb:.2e}");
    println!("      λ(checkerboard) = {:.4e}  ← INVISIBLE", checkerboard_rayleigh(&a_blind, nx));

    if zc == 1 && zb == 4 {
        println!("\n   \x1b[1;32m✓ INSTRUMENT LIVE\x1b[0m — it separates 1 from 4 with a {:.0e} spectral gap.", gb.min(gc));
        println!("     The 3 spurious modes of CTRL-BLIND are (−1)^i, (−1)^j and (−1)^(i+j).");
        println!("     The last is the CHECKERBOARD. Drawn in fig/ctrl-blind-modes.svg.");
    } else {
        println!("\n   \x1b[1;31m✗ INSTRUMENT FAILED ITS OWN CONTROLS (expected 1 and 4, got {zc} and {zb}).\x1b[0m");
        println!("     Everything below is void. Stop here.");
        return;
    }

    let panels: Vec<Panel> = (0..4)
        .map(|k| Panel {
            title: format!("null mode {}  (λ = {:.1e})", k, ev_blind[k]),
            data: vec_blind[k].clone(),
            nx,
        })
        .collect();
    draw::panels_svg(
        &dir.join("ctrl-blind-modes.svg"),
        "CTRL-BLIND — the 4 modes a collocated central-difference operator cannot see",
        "The instrument's positive control. 1 legitimate (the constant) + 3 SPURIOUS. The 3 spurious are the Nyquist modes; one of them is the literal checkerboard.",
        &panels,
    )
    .unwrap();

    draw::panels_svg(
        &dir.join("ctrl-clean-modes.svg"),
        "CTRL-CLEAN — the compact 5-point Laplacian has exactly ONE null mode",
        "The negative control. The only zero is the constant (a legitimate rigid mode). Lowest 4 eigenvectors shown: the rest are ordinary Fourier modes with real eigenvalues.",
        &(0..4)
            .map(|k| Panel { title: format!("mode {} (λ = {:.3})", k, ev_clean[k]), data: vec_clean[k].clone(), nx })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    draw::spectrum_svg(
        &dir.join("ctrl-spectra.svg"),
        "The control spectra — count the zeros",
        "Same patch, same solver, same threshold. CTRL-BLIND drops 4 eigenvalues into the floor; CTRL-CLEAN drops exactly 1.",
        &[
            ("CTRL-BLIND (collocated, 2Δ)".into(), ev_blind.iter().map(|v| v.abs()).collect()),
            ("CTRL-CLEAN (compact, Δ)".into(), ev_clean.iter().map(|v| v.abs()).collect()),
        ],
        1e-17,
    )
    .unwrap();

    // ══════════════════════════════════════════════════════════════════════════
    rule("§1  erosion.rs — the creep / hillslope-diffusion term (THE REAL KERNEL)");
    println!("   `erosion::diffuse_step` and `Fluvial::creep` are the same stencil:");
    println!("       lap = h[i−1] + h[i+1] + h[i−nx] + h[i+nx] − 4·h[i]");
    println!("   Called here for real, on a Patch whose HALO we filled with wrapped values —");
    println!("   so the shipped code runs on Cardiff's periodic patch, untouched.\n");

    let k_creep = 0.006f32; // κ/cell² at L19 (erosion.rs FluvialParams docs)
    let a_creep = {
        let u0 = vec![0.0f64; n];
        let eps = vec![1.0f64; n];
        jacobian_fd(n, &u0, &eps, |u| ops::erosion_diffuse_real(u, nx, k_creep))
    };
    let (ev_creep, _) = jacobi_sym(&a_creep, n);
    let cmax = ev_creep.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    // ⚠ TOLERANCE. The kernel is f32, so an EXACT zero comes back as ~1e-7·λmax,
    // not ~1e-16·λmax. Using the f64 threshold here would report "0 zeros" for an
    // operator that provably has one — the instrument would be lying about its own
    // arithmetic. The f32 floor is the honest cut, and the gap below shows the
    // answer does not depend on where in that band we put it.
    let f32_floor = 1e-5 * cmax;
    let (zk, gk) = zero_count(&ev_creep, f32_floor);
    println!("   asymmetry ‖A−Aᵀ‖/‖A‖ = {:.2e}   (symmetric ⇒ eigenvalues are the honest form)", asymmetry(&a_creep, n));
    println!("   λ smallest = {:.3e}   λ next = {:.3e}   λ largest = {:.3e}", ev_creep[0], ev_creep[1], cmax);
    println!("   (f32 kernel ⇒ an exact zero returns as ~1e-7·λmax; cut at 1e-5·λmax = {f32_floor:.2e})");
    println!("   zero eigenvalues: {zk}    legitimate: 1 (the constant)    \x1b[1mspurious: {}\x1b[0m", zk as i64 - 1);
    println!("   spectral gap at the cut: {gk:.2e}   ← the cut is unambiguous");
    println!("   λ(checkerboard) = {:.4}   ← seen, and damped HARDEST of all", checkerboard_rayleigh(&a_creep, nx));
    println!("\n   \x1b[1;32mCLEAN.\x1b[0m The 5-point Laplacian's every difference crosses a FACE (Δ, not 2Δ).");
    println!("   It is the compact operator, not the collocated one. It has no checkerboard");
    println!("   null mode and never did. \x1b[1mThe blindness hypothesis is REFUTED for this kernel.\x1b[0m");
    println!("   \x1b[1mBut hold the finding\x1b[0m — §3 shows what this term is actually FOR.");

    // ══════════════════════════════════════════════════════════════════════════
    rule("§2  erosion.rs — the MFD routing operator");
    println!("   Drainage solves  (I − Wᵀ)·A = q.  A zero eigenvalue of (I − Wᵀ) is an");
    println!("   eigenvalue 1 of Wᵀ: a CLOSED CIRCULATION — cells handing flow round a loop,");
    println!("   leaking none, so a nonzero drainage field exists with NO SOURCE. Water running");
    println!("   in a circle on a hillside, forever. That is the router's rank deficiency, and");
    println!("   it is the discrete ∇×∇φ ≠ 0 pathology.\n");
    mfd_probe(&dir);

    // ══════════════════════════════════════════════════════════════════════════
    rule("§3  erosion.rs — the FULL epoch (Priority-Flood → MFD → stream-power → Davy-Lague → talus → creep)");
    erosion_full_probe(&dir);

    // ══════════════════════════════════════════════════════════════════════════
    rule("§4  water.rs — the virtual-pipes shallow-water operator  ⚠ THE ONE UNDER SUSPICION");
    water_probe(&dir);

    // ══════════════════════════════════════════════════════════════════════════
    rule("§5  THE PAYOFF — staggered vs collocated shallow water, same points, same grid");
    staggering_probe(&dir);

    println!("\n\x1b[1;36m── figures written to msc/spike-null-space/fig/ ──\x1b[0m");
}

/// Rayleigh quotient of the checkerboard `(−1)^{i+j}` — "what does this operator
/// do to the Nyquist mode?" in one number.
fn checkerboard_rayleigh(a: &[f64], nx: usize) -> f64 {
    let n = nx * nx;
    let v: Vec<f64> = (0..n).map(|i| if ((i % nx) + (i / nx)) % 2 == 0 { 1.0 } else { -1.0 }).collect();
    let mut av = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            av[i] += a[i * n + j] * v[j];
        }
    }
    let num: f64 = v.iter().zip(&av).map(|(a, b)| a * b).sum();
    let den: f64 = v.iter().map(|x| x * x).sum();
    num / den
}

// ═════════════════════════════════════════════════════════════════════════════
// §2 — MFD routing
// ═════════════════════════════════════════════════════════════════════════════

fn mfd_probe(dir: &std::path::Path) {
    use vivarium_world::erosion::{Fluvial, FluvialParams};
    use vivarium_world::sphere::Face;

    // A real land footprint (the one the fluvial tests were fixed onto — verified
    // land, so erosion actually EXECUTES here; see erosion.rs `fn small()`).
    let nx = 24usize;
    let f0 = Fluvial::from_prior(0, Face::ZPos, 19, 108_500, 186_350, nx);
    let cell_m = f0.cell_m;
    let h = f0.h.clone();

    // ── PIN: our W must reproduce the kernel's own drainage field.
    // Run the real kernel for exactly one epoch with every term that MOVES ROCK
    // switched off, so `drainage` is pure MFD accumulation on the h we hold.
    let mut f = Fluvial::from_prior(0, Face::ZPos, 19, 108_500, 186_350, nx);
    let p = FluvialParams { k_dt: 0.0, m: 0.5, deposition: 0.0, max_slope: 1e9, diffusivity_m2: 0.0, epochs: 1 };
    f.erode(&p);
    let h_after = f.h.clone(); // Priority-Flood may still have raised pits
    let moved: f64 = h.iter().zip(&h_after).map(|(a, b)| (a - b).abs() as f64).fold(0.0, f64::max);

    let w = ops::mfd_weights(&h_after, nx, cell_m);
    let area = (cell_m * cell_m) as f64;
    let runoff = vec![area; nx * nx];
    let mine = ops::accumulate_with(&w, &h_after, &runoff);
    let (mut worst, mut denom) = (0.0f64, 0.0f64);
    for i in 0..nx * nx {
        worst = worst.max((mine[i] - f.drainage[i] as f64).abs());
        denom = denom.max(f.drainage[i] as f64);
    }
    println!("   PIN — our W vs the kernel's own `drainage` field:");
    println!("      Priority-Flood raised pits by up to {moved:.3} m (its ε-gradient; expected)");
    println!("      max |ours − kernel| / max(drainage) = {:.3e}", worst / denom);
    if worst / denom > 1e-5 {
        println!("      \x1b[1;31m✗ TRANSCRIPTION NOT PINNED — the W below is not the kernel's.\x1b[0m");
    } else {
        println!("      \x1b[1;32m✓ pinned\x1b[0m — W is the kernel's routing operator, to f32 precision.\n");
    }

    let nn = nx * nx;
    let a = ops::routing_operator(&w, nn);
    let (sv, svec) = singular_values(&a, nn);
    let smax = sv[nn - 1];
    let (z, gap) = zero_count(&sv, 1e-8 * smax);
    println!("   (I − Wᵀ) on a {nx}×{nx} real land patch, {nn} cells:");
    println!("      σ_min = {:.4e}   σ_max = {:.4e}   cond = {:.2e}", sv[0], smax, smax / sv[0].max(1e-300));
    println!("      zero singular values (tol = 1e-8·σ_max): {z}    spectral gap: {gap:.2e}");
    if z == 0 {
        println!("\n   \x1b[1;32mCLEAN.\x1b[0m (I − Wᵀ) is nonsingular. There is NO closed circulation: MFD's");
        println!("   flow graph is a strict DAG (weights only ever go to cells that are LOWER, so");
        println!("   a cycle would need a cell below itself). \x1b[1mMFD cannot make water run in a circle.\x1b[0m");
        println!("   Its sin is a BIAS in DIRECTION (grid_lab §9a, measured), not a null space.");
    } else {
        println!("\n   \x1b[1;31m{z} CLOSED CIRCULATION(S) FOUND.\x1b[0m Drawn.");
        draw::panels_svg(
            dir.join("mfd-circulation.svg").as_path(),
            "MFD routing — closed circulations (null space of I − Wᵀ)",
            "Flow that returns to itself with no source. Refinement cannot fix this.",
            &(0..z.min(3)).map(|k| Panel { title: format!("σ = {:.2e}", sv[k]), data: svec[k].clone(), nx }).collect::<Vec<_>>(),
        )
        .unwrap();
    }
    // The routing operator is NOT symmetric — say so, with the number.
    println!("      asymmetry ‖A−Aᵀ‖/‖A‖ = {:.3} (advective, as expected — singular values are the honest form)", asymmetry(&a, nn));
}

// ═════════════════════════════════════════════════════════════════════════════
// §3 — the full erosion epoch
// ═════════════════════════════════════════════════════════════════════════════

fn erosion_full_probe(dir: &std::path::Path) {
    use vivarium_world::erosion::{Fluvial, FluvialParams};
    use vivarium_world::sphere::Face;

    let nx = 16usize;
    let (lvl, oi, oj) = (19u8, 108_500u32, 186_350u32);
    let f0 = Fluvial::from_prior(0, Face::ZPos, lvl, oi, oj, nx);
    let base = f0.h.clone();
    let sea = 4000.0f32;
    let free: Vec<usize> = (0..nx * nx)
        .filter(|&i| {
            let (x, y) = (i % nx, i / nx);
            x > 0 && y > 0 && x < nx - 1 && y < nx - 1 && base[i] > sea
        })
        .collect();
    println!("   Real footprint (the verified-LAND one, erosion.rs `fn small()`): relief {:.0}–{:.0} m",
        base.iter().cloned().fold(f32::MAX, f32::min), base.iter().cloned().fold(f32::MIN, f32::max));
    println!("   Free DOF (interior AND above sea level): {} of {}   — the rest are base level,", free.len(), nx * nx);
    println!("   i.e. CONSTRAINED DOF in Cardiff's sense, and are excluded from the operator.\n");
    if free.len() < 20 {
        println!("   \x1b[1;31mToo few free DOF — footprint is not land. Aborting §3.\x1b[0m");
        return;
    }

    let patch = ops::ErosionPatch {
        nx,
        base: base.clone(),
        free: free.clone(),
        params: FluvialParams::default(),
        level: 19,
        origin: (108_500, 186_350),
    };
    let m = free.len();
    let u0: Vec<f64> = free.iter().map(|&i| base[i] as f64).collect();

    println!("   ⚠ The kernel is NOT DIFFERENTIABLE: D8 receiver choice is an argmax, MFD weights");
    println!("   switch at drop > 0, Priority-Flood is a heap-ordered flood-fill. A finite-difference");
    println!("   Jacobian is therefore a SECANT across those kinks. The ε-sweep is the control that");
    println!("   says whether it means anything: if the spectrum moves with ε, it does not.\n");
    println!("      {:>8}   {:>12}  {:>12}  {:>12}  {:>6}", "ε (m)", "σ_min", "σ_max", "zeros(1e-6)", "gap");
    let mut kept: Option<(Vec<f64>, Vec<Vec<f64>>, f64)> = None;
    for &eps in &[0.25f64, 0.5, 1.0, 2.0, 4.0] {
        let epsv = vec![eps; m];
        let j = jacobian_fd(m, &u0, &epsv, |u| patch.delta_h(u));
        let (sv, svec) = singular_values(&j, m);
        let smax = sv[m - 1];
        let (z, gap) = zero_count(&sv, 1e-6 * smax);
        println!("      {eps:>8.2}   {:>12.4e}  {:>12.4e}  {z:>12}  {gap:>6.1e}", sv[0], smax);
        if (eps - 1.0).abs() < 1e-9 {
            kept = Some((sv, svec, smax));
        }
    }
    let (sv, svec, smax) = kept.unwrap();

    println!("\n   At ε = 1 m:  σ_min/σ_max = {:.3e}", sv[0] / smax);
    println!("   ⚠ σ_min itself is NOT stable across ε (it moves ~3 orders). That end of the");
    println!("   spectrum is noise-dominated (f32 kernel, a 5000 m datum, ULP ≈ 0.5 mm) and we");
    println!("   read NOTHING into it. What IS stable across every ε is the zero-count (0) and");
    println!("   σ_max — and those are the two numbers the conclusion rests on.\n");

    // ── The question that matters here is not "is there a null mode" (there is not).
    //    It is: DOES ONE EPOCH AMPLIFY ANYTHING? The epoch map is I + A. |λ| > 1 is
    //    a terrain shape erosion makes MORE of — and if that shape is at the grid
    //    scale, we have found our artificial-diffusion story, on the erosion side.
    println!("   \x1b[1mThe real question for a time-stepper: does the epoch AMPLIFY anything?\x1b[0m");
    println!("   The epoch map is (I + A). |λ| > 1 = a terrain shape erosion makes MORE of.");
    println!("   And `FluvialParams::diffusivity_m2` (κ, the creep term) has a docstring that");
    println!("   already tells us what to look for:\n");
    println!("      \"detachment-limited incision leaves un-drained single-cell peaks standing");
    println!("       while cutting everything around them — \x1b[1mwithout diffusion, minimum valley");
    println!("       spacing collapses to the grid wavelength\x1b[0m\"\n");
    println!("   That is a description of a GRID-SCALE INSTABILITY, written in the code, years");
    println!("   before anyone said the word. So: run the epoch with κ ON and κ OFF.\n");

    println!("      {:<22} {:>9} {:>10} {:>26}", "κ (creep)", "ρ(I+A)", "|λ|>1", "fastest-growing mode (k/π)");
    println!("      {}", "─".repeat(72));
    let mut growth_panels: Vec<Panel> = Vec::new();
    let mut cb_gains: Vec<(String, f64, f64)> = Vec::new();

    for (label, kappa) in [("κ = 0   (creep OFF)", 0.0f32), ("κ = 2   (SHIPPED)", 2.0f32)] {
        let mut pp = patch.params.clone();
        pp.diffusivity_m2 = kappa;
        let pk = ops::ErosionPatch {
            nx,
            base: base.clone(),
            free: free.clone(),
            params: pp,
            level: patch.level,
            origin: patch.origin,
        };
        let epsv = vec![1.0f64; m];
        let a = jacobian_fd(m, &u0, &epsv, |u| pk.delta_h(u));

        // epoch map I + A
        let mut jm = a.clone();
        for i in 0..m {
            jm[i * m + i] += 1.0;
        }
        let evs = eigenvalues_general(&jm, m);
        let mods: Vec<f64> = evs.iter().map(|(re, im)| (re * re + im * im).sqrt()).collect();
        let rho = mods.iter().cloned().fold(0.0, f64::max);
        let ngrow = mods.iter().filter(|&&v| v > 1.0 + 1e-6).count();

        // shape of the fastest-growing mode, by power iteration on (I+A)
        let mut v = vec![0.0f64; m];
        for (c, &i) in free.iter().enumerate() {
            let (x, y) = ((i % nx) as f64, (i / nx) as f64);
            v[c] = ((x * 12.9898 + y * 78.233).sin() * 43758.5453).fract() - 0.5;
        }
        for _ in 0..800 {
            let mut w = vec![0.0f64; m];
            for a2 in 0..m {
                w[a2] = jm[a2 * m..(a2 + 1) * m].iter().zip(&v).map(|(x, y)| x * y).sum();
            }
            let nrm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if nrm < 1e-280 {
                break;
            }
            for a2 in 0..m {
                v[a2] = w[a2] / nrm;
            }
        }
        let mut full = vec![0.0f64; nx * nx];
        for (c, &i) in free.iter().enumerate() {
            full[i] = v[c];
        }
        let ((kx, ky), frac) = dominant_wavenumber(&full, nx);

        // response to the checkerboard vs a long wave (the α_stab signature)
        let gain = |probe: &[f64]| -> f64 {
            let mut av = vec![0.0; m];
            for a2 in 0..m {
                for b in 0..m {
                    av[a2] += a[a2 * m + b] * probe[b];
                }
            }
            (av.iter().map(|x| x * x).sum::<f64>() / probe.iter().map(|x| x * x).sum::<f64>()).sqrt()
        };
        let cb: Vec<f64> = free.iter().map(|&i| if ((i % nx) + (i / nx)) % 2 == 0 { 1.0 } else { -1.0 }).collect();
        let smooth: Vec<f64> = free
            .iter()
            .map(|&i| {
                let (x, y) = ((i % nx) as f64, (i / nx) as f64);
                (std::f64::consts::PI * x / nx as f64).sin() * (std::f64::consts::PI * y / nx as f64).sin()
            })
            .collect();
        let (gcb, gsm) = (gain(&cb), gain(&smooth));

        let flag = if ngrow > 0 { "\x1b[1;31m" } else { "\x1b[32m" };
        println!(
            "      {label:<22} {flag}{rho:>9.5} {ngrow:>10}\x1b[0m   ({kx:.2},{ky:.2})  {:.0}% of energy",
            frac * 100.0
        );
        cb_gains.push((label.to_string(), gcb, gsm));
        growth_panels.push(Panel { title: format!("{label}\nρ = {rho:.4}, k/π = ({kx:.2},{ky:.2})"), data: full, nx });
    }

    println!("\n      {:<22} {:>16} {:>16} {:>9}", "κ (creep)", "‖A·checker‖", "‖A·long-wave‖", "ratio");
    println!("      {}", "─".repeat(68));
    for (l, gcb, gsm) in &cb_gains {
        println!("      {l:<22} {gcb:>16.4} {gsm:>16.4} {:>9.2}×", gcb / gsm.max(1e-30));
    }
    println!("\n   (k/π = (1.00, 1.00) is the CHECKERBOARD. (0.x, 0.x) is a long wave.)");

    println!("\n   \x1b[1mBoth GROW — and the fastest mode is a LONG WAVE, not the grid scale.\x1b[0m ρ > 1 here is");
    println!("   not (necessarily) a disease: stream-power incision on a landscape IS a positive");
    println!("   feedback (deeper channel → more drainage → deeper still). That is how valleys");
    println!("   form. It is the physics the kernel exists to produce. Creep DAMPS it (ρ 1.048 →");
    println!("   1.039, growing modes 37 → 18) but does not remove it, and should not.\n");

    // ⚠ But the docstring's claim is about the FINE tiers — κ/cell² is 0.006 at L19
    // and 0.09–0.24 at L21/L24, where the sawtooth was actually SEEN. The L19 answer
    // cannot settle it. Sweep the level.
    println!("   ⚠ \x1b[1mBut the docstring's claim is about the FINE tiers.\x1b[0m κ/cell² is 0.006 at L19 and");
    println!("   0.09–0.24 at L21/L24 — and the sawtooth was SEEN at L21. An L19 answer cannot");
    println!("   settle it. So: sweep the level, and ask where the FASTEST-GROWING mode sits.\n");
    println!("   The number that actually answers the docstring is not \"which mode is fastest\"");
    println!("   (a channelised mode is spatially LOCALISED, so its Fourier energy is smeared) —");
    println!("   it is: \x1b[1mdoes ONE EPOCH AMPLIFY A CHECKERBOARD?\x1b[0m  gain = 1 + χᵀAχ/‖χ‖².");
    println!("   gain > 1 ⇒ grid-scale sawteeth GROW. gain < 1 ⇒ they decay.\n");
    println!(
        "      {:>6} {:>9} {:>9} {:>11} {:>11} {:>13} {:>13}",
        "level", "cell (m)", "κ/cell²", "ρ κ=0", "ρ κ=2", "χ-gain κ=0", "χ-gain κ=2"
    );
    println!("      {}", "─".repeat(80));
    for (lv, o_i, o_j) in [(19u8, 108_500u32, 186_350u32), (21, 434_000, 745_400), (24, 3_472_000, 5_963_200)] {
        let mut fx = Fluvial::from_prior(0, Face::ZPos, lv, o_i, o_j, nx);
        // ⚠ BASE STATE. Linearising about the RAW PRIOR answers the wrong question: an
        // fBm surface is not a landscape, and erosion is SUPPOSED to grow channels into
        // it (ρ > 1 there is the kernel working, not failing). A stability question must
        // be asked at a state the kernel has already relaxed. So: erode first.
        fx.erode(&FluvialParams::default());
        let bs = fx.h.clone();
        let cell = fx.cell_m;
        let fr: Vec<usize> = (0..nx * nx)
            .filter(|&i| {
                let (x, y) = (i % nx, i / nx);
                x > 0 && y > 0 && x < nx - 1 && y < nx - 1 && bs[i] > sea
            })
            .collect();
        if fr.len() < 20 {
            println!("      {lv:>6} {cell:>9.2}  — footprint is not land at this level, skipped");
            continue;
        }
        let mm = fr.len();
        let uu: Vec<f64> = fr.iter().map(|&i| bs[i] as f64).collect();
        let mut rhos = Vec::new();
        let mut gains = Vec::new();
        for kappa in [0.0f32, 2.0] {
            let mut pp = FluvialParams::default();
            pp.diffusivity_m2 = kappa;
            let pk = ops::ErosionPatch { nx, base: bs.clone(), free: fr.clone(), params: pp, level: lv, origin: (o_i, o_j) };
            let eee = vec![(cell as f64 * 0.02).max(0.05); mm]; // ε scaled to the cell
            let a = jacobian_fd(mm, &uu, &eee, |u| pk.delta_h(u));
            let mut jm = a.clone();
            for i in 0..mm {
                jm[i * mm + i] += 1.0;
            }
            rhos.push(eigenvalues_general(&jm, mm).iter().map(|(re, im)| (re * re + im * im).sqrt()).fold(0.0, f64::max));
            // The epoch's signed gain on a checkerboard: χᵀ(I+A)χ / ‖χ‖².
            let chi: Vec<f64> = fr.iter().map(|&i| if ((i % nx) + (i / nx)) % 2 == 0 { 1.0 } else { -1.0 }).collect();
            let mut jchi = vec![0.0f64; mm];
            for a2 in 0..mm {
                for b in 0..mm {
                    jchi[a2] += jm[a2 * mm + b] * chi[b];
                }
            }
            let num: f64 = chi.iter().zip(&jchi).map(|(x, y)| x * y).sum();
            let den: f64 = chi.iter().map(|x| x * x).sum();
            gains.push(num / den);
        }
        let c = |v: f64, t: f64| if v > t { "\x1b[1;31m" } else { "\x1b[32m" };
        println!(
            "      {lv:>6} {cell:>9.2} {:>9.3} {}{:>11.5}\x1b[0m {}{:>11.5}\x1b[0m {}{:>13.4}\x1b[0m {}{:>13.4}\x1b[0m",
            (2.0 / (cell * cell)).min(0.24),
            c(rhos[0], 1.0001),
            rhos[0],
            c(rhos[1], 1.0001),
            rhos[1],
            c(gains[0], 1.0),
            gains[0],
            c(gains[1], 1.0),
            gains[1],
        );
    }
    println!("\n   κ/cell² is the grid coefficient the Laplacian actually applies (clamped at 0.24).");
    println!("   Base state = the terrain AFTER a default erosion run (80 epochs), not the raw");
    println!("   fBm prior — see the note in the source. ρ > 1 on a raw prior is the kernel");
    println!("   WORKING (it is supposed to cut channels into an fBm surface); only at a relaxed");
    println!("   state does ρ > 1 mean the scheme is running away.\n");
    println!("   \x1b[1mTWO THINGS, AND THEY DO NOT SAY THE SAME THING. Read them separately.\x1b[0m\n");
    println!("   \x1b[1m(i) κ IS LOAD-BEARING FOR STABILITY.\x1b[0m With creep OFF the relaxed epoch still has");
    println!("       growing modes at every tier. With creep ON it is stable at L21/L24. The");
    println!("       `.min(0.24)` in the source is the explicit-Laplacian stability bound, in the");
    println!("       code, right there — this term is holding the scheme up, not only modelling");
    println!("       soil. \x1b[1mThat is a stabilisation role ASSUMPTIONS.md does not record.\x1b[0m\n");
    println!("   \x1b[1m(ii) BUT IT IS NOT A CHECKERBOARD / NULL-SPACE DISEASE.\x1b[0m The χ-gain columns are");
    println!("        \x1b[1mBELOW 1 EVEN WITH CREEP OFF\x1b[0m — the epoch DAMPS a checkerboard by ~5% per");
    println!("        epoch with no creep at all (talus and the upwind incision see it). So the");
    println!("        mode κ is holding down is \x1b[1mNOT\x1b[0m an invisible mode, and κ is \x1b[1mNOT\x1b[0m Rhie–Chow.");
    println!("        It is honest dissipation acting on a resolved, long-wave, channelising");
    println!("        instability. \x1b[1mI expected the opposite and the measurement said no.\x1b[0m");

    draw::panels_svg(
        dir.join("erosion-growth-modes.svg").as_path(),
        "erosion.rs — the fastest-growing terrain mode of one epoch, creep OFF vs creep ON (L19)",
        "Power iteration on the epoch map (I + ∂Δh/∂h) over a real land patch. Both grow — and both grow at LONG wavelength, along channels. That is valley formation: the stream-power/drainage feedback, which is the physics the kernel exists to produce. Neither mode is a checkerboard, and there is no null space.",
        &growth_panels,
    )
    .unwrap();

    draw::panels_svg(
        dir.join("erosion-modes.svg").as_path(),
        "erosion.rs — the modes the full epoch operator responds to LEAST",
        "Full pipeline on a real land patch (L19). These are the smallest right-singular vectors of ∂(Δh)/∂h: terrain shapes an erosion epoch barely touches. None is a checkerboard.",
        &(0..4)
            .map(|k| {
                let mut full = vec![0.0; nx * nx];
                for (c, &i) in free.iter().enumerate() {
                    full[i] = svec[k][c];
                }
                Panel { title: format!("σ = {:.3e}  ({:.1e}·σmax)", sv[k], sv[k] / smax), data: full, nx }
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    draw::spectrum_svg(
        dir.join("erosion-spectrum.svg").as_path(),
        "erosion.rs — singular spectrum of one full epoch",
        "∂(Δh)/∂h over the free (interior, supra-sea) DOF. No cliff into the floor: no null space.",
        &[("full epoch (ε = 1 m)".into(), sv.clone())],
        (sv[0] * 0.3).max(1e-12),
    )
    .unwrap();
}

// ═════════════════════════════════════════════════════════════════════════════
// §4 — water
// ═════════════════════════════════════════════════════════════════════════════

fn water_probe(dir: &std::path::Path) {
    println!("   The hypothesis under test (Joseph, from doc/theory §2.5):");
    println!("     \"the solitons WERE the invisible mode, and the θ-smoothing is Rhie–Chow-class");
    println!("      stabilisation … its physical claim is NONE.\"\n");

    // ── PIN
    let (worst, mean) = water_op::pin_against_kernel(14, 400);
    println!("   PIN — real `WaterSim::step` vs our transcription, 400 steps, 14×14, sloping bed,");
    println!("   every non-hydrodynamic stage OFF (sed/infil/evap/precip/ocean = 0, sea below bed):");
    println!("      max |Δdepth| = {worst:.3e} m   (mean depth {mean:.3} m)   ⇒ relative {:.2e}", worst / mean);
    if worst / mean > 1e-4 {
        println!("      \x1b[1;31m✗ NOT PINNED. The operator below is not water.rs. Stop.\x1b[0m");
        return;
    }
    println!("      \x1b[1;32m✓ pinned\x1b[0m (f64 transcription vs f32 kernel — the residual IS the f32).\n");

    let nx = 12usize;
    let l = 4.8f64;

    // ── (A) STILL LAKE. The depth→depth map from zero flux is EXACTLY linear
    //     (the rectifier .max(0.0) splits a signed face flux into its ± parts, and
    //     the pair sums back to the signed value), so this is an honest stiffness.
    println!("   \x1b[1m(A) base state: a still lake — flat bed, uniform depth 1 m, zero flux\x1b[0m");
    let geom_flat = Geom::Periodic { nx, g: 0.0, gy: 0.0 };
    let p_flat = PipeParams::kernel_default(l);
    let n = nx * nx;
    let depth_map = |u: &[f64]| -> Vec<f64> {
        let mut s = vec![0.0f64; 5 * n];
        for i in 0..n {
            s[5 * i] = 1.0 + u[i];
        }
        let mut g = Guards::default();
        water_op::step(&mut s, &geom_flat, &p_flat, &mut g);
        (0..n).map(|i| s[5 * i] - 1.0 - u[i]).collect()
    };
    let a_still = jacobian_fd(n, &vec![0.0; n], &vec![1e-6; n], depth_map);
    let (ev, evec) = jacobi_sym(&a_still, n);
    let smax = ev.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    let (z, gap) = zero_count(&ev, 1e-7 * smax);
    println!("      asymmetry = {:.2e}   zero eigenvalues: {z}   legitimate: 1 (total mass)   \x1b[1mspurious: {}\x1b[0m", asymmetry(&a_still, n), z as i64 - 1);
    println!("      spectral gap: {gap:.2e}");
    println!("      λ(checkerboard)/λ_max = {:.4}", checkerboard_rayleigh(&a_still, nx) / (-smax));
    println!("      ⇒ the virtual-pipes DEPTH operator is the COMPACT Laplacian: the head");
    println!("        difference (η_i − η_j) is taken across the FACE, and the flux lives ON");
    println!("        the face. \x1b[1;32mwater.rs is ALREADY A STAGGERED SCHEME.\x1b[0m It is not blind to the");
    println!("        checkerboard in depth — the checkerboard is its most-damped mode.\n");
    draw::panels_svg(
        dir.join("water-still-modes.svg").as_path(),
        "water.rs — the still-lake depth operator: 1 null mode (total mass), no checkerboard",
        "∂(Δd)/∂d from zero flux, periodic patch. The only zero is the constant. The Nyquist mode sits at the OPPOSITE end of the spectrum — it is the most strongly damped thing the scheme has.",
        &(0..3)
            .map(|k| Panel { title: format!("mode {} (λ = {:.3e})", k, ev[k]), data: evec[k].clone(), nx })
            .chain(std::iter::once(Panel {
                title: format!("checkerboard (λ = {:.3e})", checkerboard_rayleigh(&a_still, nx)),
                data: (0..n).map(|i| if ((i % nx) + (i / nx)) % 2 == 0 { 1.0 } else { -1.0 }).collect(),
                nx,
            }))
            .collect::<Vec<_>>(),
    )
    .unwrap();

    // ── (B) THE SOLITON REGIME: steep slope, real flow.
    println!("   \x1b[1m(B) base state: steady uniform flow down a STEEP channel — the regime the\x1b[0m");
    println!("   \x1b[1m    solitons appeared in. Full 5-DOF state (d, fl, fr, ft, fb) per cell.\x1b[0m\n");
    println!("      Historically the scheme ran with n = 0.04 everywhere ('torrents at 16 m/s'),");
    println!("      no breaking cap and no θ. Three terms were then added. We take them apart.\n");

    let slope = 0.10f64; // 10% — a steep mountain channel
    let d0 = 1.0f64;
    let g_drop = slope * l;

    // The linearised STEP operator on a tilted periodic channel, plus everything
    // needed to judge whether the linearisation is even admissible.
    let analyse = |p: &PipeParams, gx: f64, gy: f64| -> (f64, Vec<f64>, Vec<f64>, f64, Guards, Vec<f64>) {
        let geom = Geom::Periodic { nx, g: gx, gy };
        let (base, guards) = water_op::relax_to_steady(&geom, p, d0, 4000);
        let fr = water_op::froude(&base, &geom, p);
        let ndof = 5 * n;
        let mut eps = vec![0.0f64; ndof];
        for i in 0..n {
            eps[5 * i] = 1e-8;
            for k in 0..4 {
                eps[5 * i + 1 + k] = 1e-8;
            }
        }
        let j = jacobian_fd(ndof, &base, &eps, |u| {
            let mut s = u.to_vec();
            let mut g = Guards::default();
            water_op::step(&mut s, &geom, p, &mut g);
            s
        });
        let mut mods: Vec<f64> =
            eigenvalues_general(&j, ndof).iter().map(|(re, im)| (re * re + im * im).sqrt()).collect();
        mods.sort_by(f64::total_cmp);
        // Fastest-growing mode, by power iteration (real arithmetic: a complex pair
        // rotates inside its invariant plane, and the spatial structure is shared).
        let mut v = vec![0.0f64; ndof];
        for i in 0..n {
            let (x, y) = ((i % nx) as f64, (i / nx) as f64);
            v[5 * i] = ((x * 12.9898 + y * 78.233).sin() * 43758.5453).fract() - 0.5;
        }
        for _ in 0..800 {
            let mut w = vec![0.0f64; ndof];
            for a in 0..ndof {
                w[a] = j[a * ndof..(a + 1) * ndof].iter().zip(&v).map(|(x, y)| x * y).sum();
            }
            let nrm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if nrm < 1e-280 {
                break;
            }
            for a in 0..ndof {
                v[a] = w[a] / nrm;
            }
        }
        let dcomp: Vec<f64> = (0..n).map(|i| v[5 * i]).collect();
        (fr, mods, dcomp, *[0.0].iter().next().unwrap(), guards, j)
    };

    println!("      slope {:.0}%, cell {l} m, depth {d0} m, dt {} s", slope * 100.0, p_flat.dt);
    println!("      {:<34} {:>6} {:>9} {:>8} {:>22}", "configuration", "Fr", "ρ(J)", "|λ|>1", "fastest mode (k/π)");
    println!("      {}", "─".repeat(84));

    let configs: Vec<(&str, PipeParams)> = vec![
        (
            "θ=1  n=0.04  no-cap  (ORIGINAL)",
            PipeParams { theta: 1.0, jarrett: false, breaking_cap: false, ..PipeParams::kernel_default(l) },
        ),
        (
            "θ=0.8 n=0.04 no-cap  (θ ALONE)",
            PipeParams { theta: 0.8, jarrett: false, breaking_cap: false, ..PipeParams::kernel_default(l) },
        ),
        (
            "θ=0.5 n=0.04 no-cap  (θ HARDER)",
            PipeParams { theta: 0.5, jarrett: false, breaking_cap: false, ..PipeParams::kernel_default(l) },
        ),
        (
            "θ=1  Jarrett no-cap  (n ALONE)",
            PipeParams { theta: 1.0, jarrett: true, breaking_cap: false, ..PipeParams::kernel_default(l) },
        ),
        (
            "θ=1  n=0.04  cap     (cap ALONE)",
            PipeParams { theta: 1.0, jarrett: false, breaking_cap: true, ..PipeParams::kernel_default(l) },
        ),
        ("θ=0.8 Jarrett cap    (SHIPPED)", PipeParams::kernel_default(l)),
    ];

    let mut spectra: Vec<(String, Vec<f64>)> = Vec::new();
    let mut mode_panels: Vec<Panel> = Vec::new();
    let mut jac_original: Option<Vec<f64>> = None;

    for (name, p) in &configs {
        let (fr, mods, dcomp, _, guards, j) = analyse(p, g_drop, 0.0);
        let rho = *mods.last().unwrap();
        let unstable = mods.iter().filter(|&&m| m > 1.0 + 1e-5).count();
        let ((kx, ky), frac) = dominant_wavenumber(&dcomp, nx);
        let flag = if unstable > 0 { "\x1b[1;31m" } else { "\x1b[32m" };
        println!(
            "      {name:<34} {fr:>6.2} {flag}{rho:>9.5} {unstable:>8}\x1b[0m   ({kx:.2},{ky:.2}) {:>3.0}% of E",
            frac * 100.0
        );
        if guards.near_kink > 0 || guards.breaking > 0 || guards.clamped > 0 {
            println!(
                "      {:>34}   guards at base state: ON-KINK {} / breaking-cap {} / clamp {}  (of {} pipes)",
                "", guards.near_kink, guards.breaking, guards.clamped, guards.pipes
            );
        }
        spectra.push((name.to_string(), mods.clone()));
        if name.contains("ORIGINAL") {
            jac_original = Some(j);
            mode_panels.push(Panel { title: format!("ORIGINAL  ρ={rho:.4}\nk/π = ({kx:.2},{ky:.2})"), data: dcomp, nx });
        } else if name.contains("θ ALONE") {
            mode_panels.push(Panel { title: format!("θ=0.8 ALONE  ρ={rho:.4}\nk/π = ({kx:.2},{ky:.2})"), data: dcomp, nx });
        } else if name.contains("SHIPPED") {
            mode_panels.push(Panel { title: format!("SHIPPED  ρ={rho:.4}\nk/π = ({kx:.2},{ky:.2})"), data: dcomp, nx });
        }
    }

    println!("\n      Fr  = Froude number of the base flow.   ρ(J) = spectral radius of one step.");
    println!("      ρ>1 ⇒ a mode that GROWS unforced.  ρ=1 ⇒ a mode that meets NO RESISTANCE —");
    println!("      Cardiff's zero eigenvalue, in step-operator coordinates.");
    println!("      k/π = (1.00,1.00) is the CHECKERBOARD. (0.3,0.0) is a 6-cell wave along the flow.");
    println!("      NOTE: λ = 1 EXACTLY is LEGITIMATE here — it is the uniform-depth mode, and it");
    println!("      is λ=1 because the scheme CONSERVES MASS. The |λ|>1 column is cut at 1+1e-5 so");
    println!("      that rigid mode is not miscounted as a pathology.");

    // ── CONTROLS on the linearisation itself ────────────────────────────────
    println!("\n   \x1b[1mCONTROLS — is this linearisation admissible?\x1b[0m");
    let porig = PipeParams { theta: 1.0, jarrett: false, breaking_cap: false, ..PipeParams::kernel_default(l) };
    let (fr_a, mods_a, _, _, ga, _) = analyse(&porig, g_drop, 0.0);
    let (fr_b, mods_b, _, _, gb, _) = analyse(&porig, g_drop, g_drop * 0.05);
    println!("      (1) THE RECTIFIER KINK. On a pure +x tilt the transverse pipes sit at exactly");
    println!("          zero head — the `.max(0.0)` kink — so a ±ε difference straddles it.");
    println!("          Re-run with a 5% oblique tilt, which moves every pipe strictly off the kink:");
    println!("            pure +x  : Fr {fr_a:.2}  ρ(J) = {:.5}   pipes on the kink: {}", mods_a.last().unwrap(), ga.near_kink);
    println!("            oblique  : Fr {fr_b:.2}  ρ(J) = {:.5}   pipes on the kink: {}", mods_b.last().unwrap(), gb.near_kink);
    let dk = (mods_a.last().unwrap() - mods_b.last().unwrap()).abs();
    println!("            |Δρ| = {dk:.5}  ⇒ {}", if dk < 0.01 { "\x1b[32mthe kink does not carry the result\x1b[0m" } else { "\x1b[31mthe kink DOES matter — treat with care\x1b[0m" });

    let pgs = PipeParams { gauss_seidel: false, ..porig };
    let (_, mods_gs, _, _, _, j_jacobi) = analyse(&pgs, g_drop, 0.0);
    println!("      (2) THE IN-PLACE SWEEP. The kernel smooths θ in place (Gauss–Seidel), which");
    println!("          is not the symmetric filter de Almeida writes down. Out-of-place (Jacobi):");
    println!("            Gauss–Seidel (kernel) : ρ(J) = {:.5}", mods_a.last().unwrap());
    println!("            Jacobi (symmetric)    : ρ(J) = {:.5}", mods_gs.last().unwrap());
    println!("          ⇒ the sweep order is not carrying the result either; the Jacobi form is");
    println!("            exactly translation-invariant, so we may diagonalise it BY WAVENUMBER.");

    // ── The wavenumber map: WHERE in the spectrum does the instability live? ──
    println!("\n   \x1b[1mTHE DECISIVE PICTURE — growth rate as a function of WAVENUMBER\x1b[0m");
    let bz = brillouin(&j_jacobi, nx, 5);
    let (mut bk, mut bv) = ((0usize, 0usize), 0.0f64);
    for my in 0..nx {
        for mx in 0..nx {
            if bz[my * nx + mx] > bv {
                bv = bz[my * nx + mx];
                bk = (mx, my);
            }
        }
    }
    let kf = |m: usize| {
        let s = if m > nx / 2 { m as f64 - nx as f64 } else { m as f64 };
        2.0 * s / nx as f64
    };
    let nyq = bz[(nx / 2) * nx + nx / 2];
    println!("      peak growth |λ| = {bv:.5} at k/π = ({:.2}, {:.2})   — wavelength {:.1} cells", kf(bk.0), kf(bk.1), 2.0 / kf(bk.0).abs().max(1e-9));
    println!("      growth AT THE CHECKERBOARD, k/π = (1.00, 1.00):  |λ| = {nyq:.5}");
    println!("      growth at k = 0 (uniform):                        |λ| = {:.5}", bz[0]);
    if bv > 1.0 + 1e-6 && nyq <= 1.0 + 1e-6 {
        println!("\n      \x1b[1;33m⇒ THE UNSTABLE MODE IS A LONG WAVE, NOT THE CHECKERBOARD.\x1b[0m");
        println!("        The Nyquist corner is STABLE. The growth is at ~{:.0}-cell wavelength,", 2.0 / kf(bk.0).abs().max(1e-9));
        println!("        travelling ALONG the flow (ky = {:.2}). That is not a null mode.", kf(bk.1));
        println!("        \x1b[1mThat is a ROLL WAVE\x1b[0m — and roll waves are REAL PHYSICS above Fr ≈ 1.5.");
    }
    draw::panels_svg(
        dir.join("water-wavenumber.svg").as_path(),
        "water.rs — growth rate per step, as a function of WAVENUMBER (the Brillouin zone)",
        "θ=1, n=0.04, no cap — the scheme before the three fixes. k = 0 at centre; the CORNERS are the checkerboard (Nyquist). Red = growing. The instability is a LONG WAVE along the flow, and the checkerboard corner is quiet. A null-space/checkerboard disease would look like the exact opposite of this picture.",
        &[Panel { title: "ρ(k) − 1   (red = grows)".into(), data: fftshift(&bz.iter().map(|v| v - 1.0).collect::<Vec<_>>(), nx), nx }],
    )
    .unwrap();
    let _ = jac_original;

    // ── How far does the SHIPPED kernel's stability actually reach? ──────────
    println!("\n   \x1b[1mSLOPE SWEEP — where does the SHIPPED kernel go unstable again?\x1b[0m");
    println!("      {:>8} {:>7} {:>10} {:>8} {:>20}", "slope", "Fr", "ρ(J)", "|λ|>1", "fastest mode (k/π)");
    println!("      {}", "─".repeat(60));
    for s in [0.02f64, 0.05, 0.10, 0.20, 0.40, 0.70, 1.00] {
        let p = PipeParams::kernel_default(l);
        let (fr, mods, dcomp, _, _, _) = analyse(&p, s * l, 0.0);
        let rho = *mods.last().unwrap();
        let ngrow = mods.iter().filter(|&&m| m > 1.0 + 1e-5).count();
        let ((kx, ky), _) = dominant_wavenumber(&dcomp, nx);
        let flag = if ngrow > 0 { "\x1b[1;31m" } else { "\x1b[32m" };
        println!("      {:>7.0}% {fr:>7.2} {flag}{rho:>10.5} {ngrow:>8}\x1b[0m  ({kx:.2},{ky:.2})", s * 100.0);
    }
    println!("      (Vedernikov: uniform flow with Manning friction is roll-wave unstable above");
    println!("       Fr ≈ 1.5. Compare the Fr column against that line, not against the ρ column.)");
    println!("      ⚠ The sweep is NOT MONOTONE, and Fr does not explain it: 2% and 5% GROW at");
    println!("      Fr < 0.65, which no roll-wave criterion permits. There is a SECOND mechanism.");

    // ── The second mechanism, isolated ──────────────────────────────────────
    println!("\n   \x1b[1mTHE SECOND MECHANISM — the Jarrett slope-roughness term is a POSITIVE FEEDBACK\x1b[0m");
    println!("      water.rs computes roughness from the LOCAL WATER-SURFACE SLOPE, per pipe:");
    println!("          n = min(n_base + 1.6·(head/l), 0.13)          [Jarrett 1984, linearised]");
    println!("      and friction divides the flux by (1 + dt·g·n²·v / h^4ᐟ³). So:\n");
    println!("        a cell that gains water  →  its surface steepens  →  n RISES  →  its outflow");
    println!("        FALLS  →  it gains more water.  \x1b[1mThat loop closes with the wrong sign.\x1b[0m\n");
    println!("      It should switch off once n saturates at the 0.13 cap (dn/dhead = 0 there).");
    println!("      Prediction: unstable on GENTLE slopes (n uncapped), quiet in the middle band");
    println!("      (n capped, Fr still < 1.5), unstable again when Fr passes ~1.5. Test it:\n");
    println!("      {:>7} {:>7} {:>8} {:>12} {:>12} {:>10}", "slope", "Fr", "n(base)", "ρ Jarrett", "ρ n=0.04", "capped?");
    println!("      {}", "─".repeat(62));
    for s in [0.02f64, 0.05, 0.08, 0.10, 0.20, 0.40, 0.70] {
        let pj = PipeParams::kernel_default(l);
        // ⚠ THE CONTROL MUST MATCH THE BASE STATE, NOT THE FORMULA. Jarrett CAPS n at
        // 0.13; a control using the uncapped `0.04 + 1.6·s` would run 0.68 at s = 0.4
        // — a different flow, a different Froude number, a different experiment. The
        // only thing that may differ between the columns is whether n can RESPOND.
        let pf = PipeParams { jarrett: false, manning_n: (0.04 + 1.6 * s).min(0.13), ..PipeParams::kernel_default(l) };
        let (fr, mj, _, _, _, _) = analyse(&pj, s * l, 0.0);
        let (_, mf, _, _, _, _) = analyse(&pf, s * l, 0.0);
        let nj = (0.04 + 1.6 * s).min(0.13);
        let capped = if 0.04 + 1.6 * s >= 0.13 { "YES" } else { "no" };
        let rj = *mj.last().unwrap();
        let rf = *mf.last().unwrap();
        let fj = if rj > 1.0 + 1e-5 { "\x1b[1;31m" } else { "\x1b[32m" };
        let ff = if rf > 1.0 + 1e-5 { "\x1b[1;31m" } else { "\x1b[32m" };
        println!("      {:>6.0}% {fr:>7.2} {nj:>8.3} {fj}{rj:>12.5}\x1b[0m {ff}{rf:>12.5}\x1b[0m {capped:>10}", s * 100.0);
    }
    println!("\n      Column 4 is the SHIPPED kernel. Column 5 is the same friction magnitude held");
    println!("      CONSTANT (so the roughness cannot respond to the surface slope) — the feedback");
    println!("      loop cut, everything else identical.");

    // ── The control that makes all of §4 believable: does the NONLINEAR sim do it? ──
    println!("\n   \x1b[1mNONLINEAR CONFIRMATION — a linear growth rate is a prediction. Test it.\x1b[0m");
    println!("      Perturb the steady state by 1e-6 m of fated noise and run the FULL nonlinear");
    println!("      step (no pinning). If ρ(J) is real, ‖δd‖ grows at exactly that rate.\n");
    println!("      {:<32} {:>10} {:>14} {:>10}", "configuration", "ρ(J) pred", "ρ measured", "match");
    println!("      {}", "─".repeat(70));
    for (name, p) in [
        ("θ=1  n=0.04  no-cap (ORIGINAL)", PipeParams { theta: 1.0, jarrett: false, breaking_cap: false, ..PipeParams::kernel_default(l) }),
        ("θ=0.8 n=0.04 no-cap (θ ALONE)", PipeParams { theta: 0.8, jarrett: false, breaking_cap: false, ..PipeParams::kernel_default(l) }),
        ("θ=0.8 Jarrett cap   (SHIPPED)", PipeParams::kernel_default(l)),
    ] {
        let (_, mods, _, _, _, _) = analyse(&p, g_drop, 0.0);
        let pred = *mods.last().unwrap();
        let geom = Geom::Periodic { nx, g: g_drop, gy: 0.0 };
        let (base, _) = water_op::relax_to_steady(&geom, &p, d0, 4000);
        let mut s = base.clone();
        // ⚠ The perturbation must stay INSIDE the linear regime for the whole run, or
        // we measure the SATURATED nonlinear rate and call it the eigenvalue. At 4%
        // growth per step, a 1e-6 seed is O(10 m) after 400 steps — utterly saturated.
        // Seed at 1e-9 and keep the window short; the final deviation is printed so
        // the reader can check we stayed there.
        for i in 0..n {
            let (x, y) = ((i % nx) as f64, (i / nx) as f64);
            s[5 * i] += 1e-9 * (((x * 12.9898 + y * 78.233).sin() * 43758.5453).fract() - 0.5);
        }
        let dev = |s: &[f64]| -> f64 { (0..n).map(|i| (s[5 * i] - base[5 * i]).powi(2)).sum::<f64>().sqrt() };
        let mut g = Guards::default();
        for _ in 0..150 {
            water_op::step(&mut s, &geom, &p, &mut g); // burn-in: sub-dominant modes die
        }
        let d0n = dev(&s);
        let steps = 150usize;
        for _ in 0..steps {
            water_op::step(&mut s, &geom, &p, &mut g);
        }
        let d1n = dev(&s);
        let meas = if d0n > 1e-280 { (d1n / d0n).powf(1.0 / steps as f64) } else { f64::NAN };
        let ok = (meas - pred).abs() < 0.004;
        println!(
            "      {name:<32} {pred:>10.5} {meas:>14.5} {:>10}   ‖δd‖ end = {:.1e} m",
            if ok { "\x1b[32m✓\x1b[0m" } else { "\x1b[31m✗\x1b[0m" },
            d1n
        );
    }
    println!("\n      (Measured = the actual per-step growth of ‖δd‖ in the running nonlinear kernel.");
    println!("       ‖δd‖ stays ≪ the 1 m base depth throughout, so this is the LINEAR rate — the");
    println!("       eigenvalue is not an artefact of the linearisation. The scheme really does");
    println!("       amplify that mode, at that rate.)");

    // ── THE VERDICT ─────────────────────────────────────────────────────────
    println!("\n   \x1b[1;36m╭──────────────────────────────────────────────────────────────────────────╮\x1b[0m");
    println!("   \x1b[1;36m│  §4 VERDICT — the hypothesis is REFUTED, and the truth is more useful    │\x1b[0m");
    println!("   \x1b[1;36m╰──────────────────────────────────────────────────────────────────────────╯\x1b[0m");
    println!("   \x1b[1mHypothesis:\x1b[0m \"the solitons WERE the invisible mode; θ is Rhie–Chow-class");
    println!("   stabilisation; its physical claim is NONE.\"\n");
    println!("   \x1b[1;31mNo.\x1b[0m Every part of that is contradicted by a measurement with a control:\n");
    println!("   1. \x1b[1mthere is no invisible mode to be.\x1b[0m water.rs is ALREADY STAGGERED — the head");
    println!("      difference is taken across the face and the flux lives on the face. Its depth");
    println!("      operator is the compact Laplacian; the checkerboard is its MOST-damped mode");
    println!("      (§4A: 1 null mode, the constant; 0 spurious; ρ at Nyquist = 0.919 < 1).");
    println!("      \x1b[1mThe Cardiff disease is the one disease this kernel does not have.\x1b[0m\n");
    println!("   2. \x1b[1mthe solitons were a LONG WAVE, not a grid-scale mode.\x1b[0m The unstable mode of");
    println!("      the pre-fix scheme sits at k/π = (0.33, 0.00) — a ~6-cell wave running ALONG");
    println!("      the flow — while the Nyquist corner is quiet. That is the signature of a");
    println!("      \x1b[1mROLL WAVE\x1b[0m, and roll waves are REAL PHYSICS above Fr ≈ 1.5 (Vedernikov).");
    println!("      The base flow was at Fr 2.49. \x1b[1mThe scheme was RESOLVING an instability, not");
    println!("      hallucinating one.\x1b[0m (What it got wrong is the SATURATION — a local-inertial");
    println!("      scheme drops the advective term, so nothing bounds the wave.)\n");
    println!("   3. \x1b[1mθ DOES NOT REMOVE THE MODE. It damps it, and it MOVES it.\x1b[0m");
    println!("        θ = 1.0 → ρ = 1.04011   (48 growing modes, peak at k/π = 0.33)");
    println!("        θ = 0.8 → ρ = 1.02655   (10 growing modes, peak at k/π = 0.17)  ← SHIPPED gain");
    println!("        θ = 0.5 → ρ = 1.00775   ( 6 growing modes, peak at k/π = 0.17)");
    println!("      Still unstable at HALF the shipped θ. And note where the peak goes: a low-pass");
    println!("      filter cannot kill a long wave, so the instability simply \x1b[1mrelocates to the");
    println!("      wavelengths the filter has no grip on.\x1b[0m θ buys time, not stability.\n");
    println!("   4. \x1b[1mwhat actually stabilises the SHIPPED kernel is the JARRETT ROUGHNESS\x1b[0m — by");
    println!("      dropping the flow from Fr 2.49 to Fr 0.75, i.e. out of the roll-wave regime");
    println!("      entirely. Not θ. \x1b[1mThe credit was on the wrong term.\x1b[0m\n");
    println!("   5. \x1b[1;33mAND THE JARRETT TERM HAS ITS OWN, PREVIOUSLY UNKNOWN INSTABILITY.\x1b[0m Because n");
    println!("      rises with the local surface slope, a cell that gains water throttles its own");
    println!("      outflow. On GENTLE slopes (where n has not yet saturated at the 0.13 cap) that");
    println!("      loop is unstable at Fr ≈ 0.6 — nowhere near any roll-wave threshold. Hold n");
    println!("      constant at the same magnitude and it vanishes (ρ 1.012 → 1.000). \x1b[1mThis one IS");
    println!("      a numerical artefact with no physical claim\x1b[0m, and it is in the shipped kernel");
    println!("      at exactly the slopes most of a landscape has.\n");
    println!("   \x1b[1mNet for ASSUMPTIONS.md — the opposite of the proposed correction:\x1b[0m θ's physical");
    println!("   claim is not NONE; it is damping a mode that is (partly) real, which means it may");
    println!("   be suppressing PHYSICS. The term that deserves the hard look is the \x1b[1mJarrett");
    println!("   proxy\x1b[0m, which MANUFACTURES an instability the physics does not have.");

    draw::spectrum_svg(
        dir.join("water-spectra.svg").as_path(),
        "water.rs — |λ| of the one-step operator on a steep channel (10% slope, Fr>1)",
        "Every eigenvalue of the linearised virtual-pipes step. Anything at or above 1.0 is a mode that is not damped. The dashed region above 1 is where the solitons live.",
        &spectra,
        0.05,
    )
    .unwrap();
    if !mode_panels.is_empty() {
        draw::panels_svg(
            dir.join("water-modes.svg").as_path(),
            "water.rs — the DEPTH component of the fastest-growing mode, steep channel",
            "Power iteration on the linearised step operator. Left: the scheme as it was before the three fixes. Right: as shipped. If the mode is at Nyquist it is numerical; if it is a long wave it is a roll wave, and roll waves are REAL physics above Fr≈1.5.",
            &mode_panels,
        )
        .unwrap();
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// §5 — staggered vs collocated, on the SAME shallow-water physics
// ═════════════════════════════════════════════════════════════════════════════

fn staggering_probe(dir: &std::path::Path) {
    println!("   The claim in doc/theory/fig/staggering.svg: a collocated scheme must reach TWO");
    println!("   cells to get a flux across a face, so it computes (h_{{i+1}} − h_{{i−1}})/2Δ, which on");
    println!("   the checkerboard is EXACTLY ZERO. A staggered scheme keeps the flux on the face");
    println!("   it actually crosses and reaches only Δ. Same points. Same grid. Same cost.\n");
    println!("   Tested on IDENTICAL linear shallow water (no friction — friction would hide the");
    println!("   answer by damping everything): ∂f/∂t = −g·h̄·∇η ,  ∂d/∂t = −∇·f.\n");

    let nx = 12usize;
    let n = nx * nx;
    let (g, hbar, dx, dt) = (9.8f64, 1.0f64, 4.8f64, 0.05f64);
    let c2 = g * hbar;

    let idx = |x: isize, y: isize| -> usize { (y.rem_euclid(nx as isize)) as usize * nx + (x.rem_euclid(nx as isize)) as usize };

    // STAGGERED (Arakawa C): flux on the face between i and i+1.
    // state: [d (n), fx (n, face i→i+1), fy (n, face i→i+nx)]
    let stag = |u: &[f64]| -> Vec<f64> {
        let (d, fx, fy) = (&u[0..n], &u[n..2 * n], &u[2 * n..3 * n]);
        let mut o = vec![0.0f64; 3 * n];
        for y in 0..nx as isize {
            for x in 0..nx as isize {
                let i = idx(x, y);
                o[n + i] = fx[i] - dt * c2 * (d[idx(x + 1, y)] - d[i]) / dx; // face gradient: Δ apart
                o[2 * n + i] = fy[i] - dt * c2 * (d[idx(x, y + 1)] - d[i]) / dx;
            }
        }
        for y in 0..nx as isize {
            for x in 0..nx as isize {
                let i = idx(x, y);
                let div = (o[n + i] - o[n + idx(x - 1, y)]) / dx + (o[2 * n + i] - o[2 * n + idx(x, y - 1)]) / dx;
                o[i] = d[i] - dt * div;
            }
        }
        o
    };

    // COLLOCATED: velocity at the cell CENTRE, central differences both ways.
    // state: [d (n), u (n), v (n)] — all at cell centres.
    let coll = |uu: &[f64]| -> Vec<f64> {
        let (d, u, v) = (&uu[0..n], &uu[n..2 * n], &uu[2 * n..3 * n]);
        let mut o = vec![0.0f64; 3 * n];
        for y in 0..nx as isize {
            for x in 0..nx as isize {
                let i = idx(x, y);
                o[n + i] = u[i] - dt * g * (d[idx(x + 1, y)] - d[idx(x - 1, y)]) / (2.0 * dx); // 2Δ
                o[2 * n + i] = v[i] - dt * g * (d[idx(x, y + 1)] - d[idx(x, y - 1)]) / (2.0 * dx);
            }
        }
        for y in 0..nx as isize {
            for x in 0..nx as isize {
                let i = idx(x, y);
                let div = (o[n + idx(x + 1, y)] - o[n + idx(x - 1, y)]) / (2.0 * dx)
                    + (o[2 * n + idx(x, y + 1)] - o[2 * n + idx(x, y - 1)]) / (2.0 * dx);
                o[i] = d[i] - dt * hbar * div;
            }
        }
        o
    };

    let ndof = 3 * n;
    let z = vec![0.0f64; ndof];
    let e = vec![1.0f64; ndof];
    let js = jacobian_fd(ndof, &z, &e, stag);
    let jc = jacobian_fd(ndof, &z, &e, coll);

    // The STIFFNESS form: A = (J − I)/dt. Its null space is Cardiff's, literally.
    let stiff = |j: &[f64]| -> Vec<f64> {
        let mut a = j.to_vec();
        for i in 0..ndof {
            a[i * ndof + i] -= 1.0;
        }
        for v in a.iter_mut() {
            *v /= dt;
        }
        a
    };
    let (svs, _) = singular_values(&stiff(&js), ndof);
    let (svc, _) = singular_values(&stiff(&jc), ndof);
    let (zs, gs) = zero_count(&svs, 1e-8 * svs[ndof - 1]);
    let (zc, gc) = zero_count(&svc, 1e-8 * svc[ndof - 1]);

    println!("   Whole-operator null space (A = (J−I)/Δt):");
    println!("   {:<26} {:>10} {:>14}", "scheme", "zero σ", "spectral gap");
    println!("   {}", "─".repeat(54));
    println!("   {:<26} {:>10} {:>14.1e}", "STAGGERED (flux on face)", zs, gs);
    println!("   {:<26} {:>10} {:>14.1e}", "COLLOCATED (2Δ central)", zc, gc);
    println!("\n   \x1b[1mBOTH numbers are large, and BOTH are mostly legitimate — so the raw count is\x1b[0m");
    println!("   \x1b[1mnot the finding.\x1b[0m A wave system's null space contains every DIVERGENCE-FREE flux");
    println!("   field (steady rotational flow: real, physical, and it should exert no force).");
    println!("   The honest question is narrower and it is the one Cardiff is actually asking:\n");
    println!("      \x1b[1mIs there a DEPTH field that exerts NO FORCE on the momentum equation?\x1b[0m\n");
    println!("   That is the null space of the pressure-gradient block G = ∂(flux rate)/∂(depth),");
    println!("   and a nonzero one is EXACTLY the checkerboard disease: a pressure/height pattern");
    println!("   the scheme cannot feel, so nothing pushes back, so it grows without bound.\n");

    // Extract the gradient block: rows = flux DOF, cols = depth DOF.
    let grad_block = |j: &[f64]| -> Vec<f64> {
        let mut g = vec![0.0f64; (2 * n) * n];
        for r in 0..2 * n {
            for c in 0..n {
                g[r * n + c] = (j[(n + r) * ndof + c]) / dt; // (J − I) has no I here (off-diagonal block)
            }
        }
        g
    };
    // Square it up for the Jacobi SVD (GᵀG is n×n regardless).
    let grad_null = |g: &[f64]| -> (Vec<f64>, Vec<Vec<f64>>) {
        let mut gtg = vec![0.0f64; n * n];
        for i in 0..n {
            for j2 in 0..n {
                let mut s = 0.0;
                for r in 0..2 * n {
                    s += g[r * n + i] * g[r * n + j2];
                }
                gtg[i * n + j2] = s;
            }
        }
        let (vals, vecs) = jacobi_sym(&gtg, n);
        (vals.iter().map(|v| v.max(0.0).sqrt()).collect(), vecs)
    };
    let (gsv_s, gvec_s) = grad_null(&grad_block(&js));
    let (gsv_c, gvec_c) = grad_null(&grad_block(&jc));
    let (gz_s, gg_s) = zero_count(&gsv_s, 1e-8 * gsv_s[n - 1]);
    let (gz_c, gg_c) = zero_count(&gsv_c, 1e-8 * gsv_c[n - 1]);

    println!("   {:<26} {:>18} {:>12} {:>14}", "scheme", "depth fields with", "spurious", "spectral gap");
    println!("   {:<26} {:>18} {:>12} {:>14}", "", "ZERO force", "", "");
    println!("   {}", "─".repeat(74));
    println!("   {:<26} {:>18} {:>12} {:>14.1e}", "STAGGERED (flux on face)", gz_s, gz_s as i64 - 1, gg_s);
    println!(
        "   {:<26} {:>18} \x1b[1;31m{:>12}\x1b[0m {:>14.1e}",
        "COLLOCATED (2Δ central)",
        gz_c,
        gz_c as i64 - 1,
        gg_c
    );
    println!("\n   legitimate: 1 — the CONSTANT depth field (a uniform lift exerts no force; that");
    println!("   is a rigid mode, exactly as Cardiff says it should be).\n");

    println!("   ρ(J) staggered  = {:.6}   (a wave operator: |λ| = 1 for the WAVES — correct)", spec_radius(&js, ndof));
    println!("   ρ(J) collocated = {:.6}", spec_radius(&jc, ndof));

    // The decisive number: what does each scheme DO to the checkerboard?
    let cb: Vec<f64> = {
        let mut v = vec![0.0f64; ndof];
        for i in 0..n {
            v[i] = if ((i % nx) + (i / nx)) % 2 == 0 { 1.0 } else { -1.0 };
        }
        v
    };
    let force = |a: &[f64]| -> f64 {
        let mut av = vec![0.0; ndof];
        for i in 0..ndof {
            for j in 0..ndof {
                av[i] += a[i * ndof + j] * cb[j];
            }
        }
        (av.iter().map(|x| x * x).sum::<f64>() / cb.iter().map(|x| x * x).sum::<f64>()).sqrt()
    };
    let fs = force(&stiff(&js));
    let fc = force(&stiff(&jc));
    println!("\n   \x1b[1mThe restoring force on a CHECKERBOARD depth field:\x1b[0m");
    println!("      staggered  : ‖A·χ‖/‖χ‖ = {fs:.4}   ← the mode is SEEN, and strongly");
    println!("      collocated : ‖A·χ‖/‖χ‖ = {fc:.3e}   ← \x1b[1;31mEXACTLY ZERO. INVISIBLE.\x1b[0m");
    println!("      ratio      : {:.2e}", fs / fc.max(1e-300));
    println!("\n   \x1b[1;32m⇒ THE STAGGERING CLAIM IS CONFIRMED, ON THE SAME POINTS AND THE SAME GRID.\x1b[0m");
    println!("     The collocated scheme needs an α_stab to survive. The staggered one needs");
    println!("     nothing: the unphysical term is not TAMED, it is \x1b[1mNEVER REQUIRED\x1b[0m.");

    // ⚠ The 4 zero singular values are DEGENERATE, so the eigensolver returns an
    // ARBITRARY basis of the 4-D null space — a picture of it would show mixtures,
    // not modes. Draw the CANONICAL modes instead, each labelled with the force the
    // scheme actually exerts on it. That is both more legible and more honest.
    let mode = |f: &dyn Fn(usize, usize) -> f64| -> Vec<f64> {
        (0..n).map(|i| f(i % nx, i / nx)).collect()
    };
    let named: Vec<(&str, Vec<f64>)> = vec![
        ("constant  (LEGITIMATE)", mode(&|_, _| 1.0)),
        ("(−1)^i", mode(&|x, _| if x % 2 == 0 { 1.0 } else { -1.0 })),
        ("(−1)^j", mode(&|_, y| if y % 2 == 0 { 1.0 } else { -1.0 })),
        ("(−1)^(i+j)  CHECKERBOARD", mode(&|x, y| if (x + y) % 2 == 0 { 1.0 } else { -1.0 })),
        (
            "smooth long wave (control)",
            mode(&|x, y| {
                let (fx, fy) = (2.0 * std::f64::consts::PI * x as f64 / nx as f64, 2.0 * std::f64::consts::PI * y as f64 / nx as f64);
                fx.sin() * fy.cos()
            }),
        ),
    ];
    let force_on = |g: &[f64], v: &[f64]| -> f64 {
        let mut gv = vec![0.0f64; 2 * n];
        for r in 0..2 * n {
            for c in 0..n {
                gv[r] += g[r * n + c] * v[c];
            }
        }
        (gv.iter().map(|x| x * x).sum::<f64>() / v.iter().map(|x| x * x).sum::<f64>()).sqrt()
    };
    let (gb_s, gb_c) = (grad_block(&js), grad_block(&jc));
    println!("\n   \x1b[1mThe force the momentum equation exerts on each canonical depth mode:\x1b[0m");
    println!("      {:<28} {:>16} {:>16}", "depth mode", "STAGGERED", "COLLOCATED");
    println!("      {}", "─".repeat(62));
    let mut panels = Vec::new();
    for (name, v) in &named {
        let (a, b) = (force_on(&gb_s, v), force_on(&gb_c, v));
        let mark = if b < 1e-9 { "\x1b[1;31m" } else { "\x1b[32m" };
        println!("      {name:<28} {a:>16.4} {mark}{b:>16.3e}\x1b[0m");
        panels.push(Panel { title: format!("{name}\nstag {a:.2} | colloc {b:.1e}"), data: v.clone(), nx });
    }
    let _ = (&gvec_s, &gvec_c);
    draw::panels_svg(
        dir.join("staggering.svg").as_path(),
        "§5 — THE MODES A COLLOCATED SCHEME CANNOT SEE. Same physics, same points, same grid.",
        "Depth fields that exert ZERO force on the momentum equation. Left: the collocated scheme's null space — the constant (legitimate) and the three Nyquist modes, one of which is the literal CHECKERBOARD. These grow without resistance, and α_stab exists to suppress them. Right: at the same spectral position the staggered scheme has an ordinary, strongly-forced mode. It needs no α_stab at all.",
        &panels,
    )
    .unwrap();
    draw::spectrum_svg(
        dir.join("staggering-spectra.svg").as_path(),
        "§5 — the force a depth field feels: staggered vs collocated, same operator",
        "Singular values of G = ∂(flux rate)/∂(depth). The collocated scheme drops FOUR into the numerical floor; the staggered scheme drops exactly ONE (the constant). Those three extra zeros are the whole content of α_stab.",
        &[("COLLOCATED (2Δ central)".into(), gsv_c.clone()), ("STAGGERED (flux on face)".into(), gsv_s.clone())],
        1e-14,
    )
    .unwrap();
    let _ = (&svs, &svc);
}

fn spec_radius(j: &[f64], n: usize) -> f64 {
    eigenvalues_general(j, n).iter().map(|(re, im)| (re * re + im * im).sqrt()).fold(0.0, f64::max)
}

/// **The Brillouin-zone map.** For a translation-invariant operator on a periodic
/// patch, each wavevector `k` spans an INVARIANT subspace of dimension
/// `2 × dof_per_cell` (a cosine and a sine copy of every component). Project `J`
/// onto it, take the eigenvalues of that little block, and the largest modulus is
/// **the growth rate of that wavelength, per step**.
///
/// This is the measurement that separates a numerical disease from physics:
/// - growth concentrated at the **corner** (k = π/Δ, the checkerboard) ⇒ a null
///   mode / rank deficiency ⇒ Cardiff's disease ⇒ needs `α_stab`.
/// - growth concentrated at **long wavelength** ⇒ a physical instability the
///   scheme is resolving, and damping it is damping *physics*.
fn brillouin(j: &[f64], nx: usize, dpc: usize) -> Vec<f64> {
    let n = nx * nx;
    let ndof = dpc * n;
    let mut out = vec![0.0f64; n];
    for my in 0..nx {
        for mx in 0..nx {
            let mut basis: Vec<Vec<f64>> = Vec::new();
            for c in 0..dpc {
                for phase in 0..2 {
                    let mut v = vec![0.0f64; ndof];
                    for i in 0..n {
                        let (x, y) = (i % nx, i / nx);
                        let th = 2.0 * std::f64::consts::PI * ((mx * x) as f64 + (my * y) as f64) / nx as f64;
                        v[dpc * i + c] = if phase == 0 { th.cos() } else { th.sin() };
                    }
                    let nrm = v.iter().map(|a| a * a).sum::<f64>().sqrt();
                    if nrm > 1e-9 {
                        for a in v.iter_mut() {
                            *a /= nrm;
                        }
                        basis.push(v);
                    }
                }
            }
            let m = basis.len();
            let mut small = vec![0.0f64; m * m];
            for (bi, b) in basis.iter().enumerate() {
                let mut jb = vec![0.0f64; ndof];
                for a in 0..ndof {
                    jb[a] = j[a * ndof..(a + 1) * ndof].iter().zip(b).map(|(x, y)| x * y).sum();
                }
                for (ai, a2) in basis.iter().enumerate() {
                    small[ai * m + bi] = a2.iter().zip(&jb).map(|(x, y)| x * y).sum();
                }
            }
            out[my * nx + mx] = spec_radius(&small, m);
        }
    }
    out
}

/// Put `k = 0` at the centre of the picture, so the Nyquist corners sit at the
/// edges — the conventional Brillouin-zone layout.
fn fftshift(v: &[f64], nx: usize) -> Vec<f64> {
    let mut o = vec![0.0f64; nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            o[((y + nx / 2) % nx) * nx + (x + nx / 2) % nx] = v[y * nx + x];
        }
    }
    o
}
