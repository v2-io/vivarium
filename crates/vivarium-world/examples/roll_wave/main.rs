//! ROLL-WAVE PROBE — step (3) of the order-of-work in
//! `DECISIONS[water-runs-outside-its-published-validity-envelope]`, and the
//! probe `DECISIONS[theta-is-lax-friedrichs-not-rhie-chow]` asks for by name.
//!
//! ## What is already measured, and what is not
//!
//! `DECISIONS[our-kernels-have-no-null-space-the-solitons-were-roll-waves]`
//! mapped the growth rate over the Brillouin zone **at one resolution** and
//! found the peak at `k/π = 0.33` — *a ~6-cell wave running along the flow* —
//! and read that as a **roll wave, real physics** above Fr ≈ 1.5 (Vedernikov).
//! `DECISIONS[jarrett-roughness-is-a-positive-feedback-and-is-not-used-as-intended]` then separated a
//! gentle-slope numerical artefact from the steep-slope growth. Both entries
//! are council-accepted, and both leave the same thing **[OPEN]**: *"whether
//! the roll waves are QUANTITATIVELY right."*
//!
//! ## The question this probe adds
//!
//! A ~6-cell wave is exactly what a numerical instability looks like, and also
//! what a physical instability looks like when it is under-resolved. **The
//! discriminator is grid refinement at FIXED PHYSICAL SCALE**, and it has not
//! been run:
//!
//! * A **physical** instability has a physical growth rate. Hold the slope, the
//!   depth and the domain length fixed, shrink `Δx` (and `Δt` with it), and the
//!   growth rate **per second** converges, while the peak **wavelength in
//!   metres** stays put.
//! * A **numerical** instability is a property of the stencil. Its peak sits at
//!   a fixed number of *cells* and its growth rate per second rises without
//!   bound as the grid refines.
//!
//! Honest caveat registered before running: Vedernikov linear theory for the
//! frictional shallow-water system has growth increasing monotonically with
//! wavenumber and therefore no intrinsic length scale of its own either
//! (roll-wave wavelength selection is a *nonlinear*, finite-amplitude
//! phenomenon). So grid-locking of the PEAK is weaker evidence than it looks.
//! The growth-rate convergence is the stronger half, and the two are reported
//! separately for that reason.
//!
//! ## Method
//!
//! Renormalised power iteration on the real dynamics ("twin experiment"): hold
//! a base state that is a genuine fixed point (uniform depth, relaxed flux, on
//! a periodic tilt — the classic roll-wave setting), run a broadband
//! perturbation alongside it, and rescale the difference to fixed norm every
//! step. The ratio it converges to is the spectral radius `ρ` of the linearised
//! step; `σ = ln(ρ)/Δt` is the growth rate per second, which is the only form
//! comparable across grids.
//!
//! §4 repeats the measurement on the **real `WaterSim`** so nothing rests on
//! the transcription alone.

#![allow(dead_code)]

#[path = "../null_space/water_op.rs"]
mod water_op;

use water_op::{Geom, Guards, PipeParams};

/// Deterministic broadband seed — every wavenumber excited, no RNG dependency
/// (and therefore no run-to-run variation; the curl probe's own regression note
/// is that a HashMap-order non-determinism produced its first, flattering
/// numbers).
fn broadband(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let t = i as f64;
            ((0.7 * t).sin() + 0.31 * (2.9 * t).cos() + 0.17 * (11.3 * t).sin()) / 1.5
        })
        .collect()
}

fn norm(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

struct Growth {
    rho: f64,
    /// Growth per SECOND — the only form comparable across grids.
    sigma: f64,
    /// Dominant wavelength of the converged mode, in cells and in metres.
    lam_cells: f64,
    lam_m: f64,
    froude: f64,
    guards: Guards,
}

/// Renormalised power iteration. Returns the converged per-step growth ratio.
fn growth(geom: &Geom, p: &PipeParams, d0: f64, burn: usize, meas: usize) -> Growth {
    let n = geom.n();
    let nx = geom.nx();
    let (base, g0) = water_op::relax_to_steady(geom, p, d0, 6000);
    let fr = water_op::froude(&base, geom, p);

    // Perturb the DEPTH component broadband; the flux components follow.
    let eps = 1e-9;
    let seed = broadband(n);
    let mut pert = base.clone();
    for i in 0..n {
        pert[5 * i] += eps * seed[i];
    }

    let mut b = base.clone();
    let mut ratios: Vec<f64> = Vec::new();
    let mut last_delta = vec![0.0f64; 5 * n];
    let mut g = Guards::default();
    for step in 0..(burn + meas) {
        let mut gb = Guards::default();
        let mut gp = Guards::default();
        water_op::step(&mut b, geom, p, &mut gb);
        water_op::step(&mut pert, geom, p, &mut gp);
        g = gp;
        let delta: Vec<f64> = (0..5 * n).map(|k| pert[k] - b[k]).collect();
        let nd = norm(&delta);
        if !nd.is_finite() || nd == 0.0 {
            break;
        }
        // The base state is a fixed point, so `b` should not drift; renormalise
        // the perturbation back onto it to stay strictly in the linear regime.
        let scale = eps / nd;
        if step >= burn {
            ratios.push(nd / eps);
        }
        last_delta = delta.iter().map(|d| d * scale / eps).collect();
        b = base.clone();
        pert = base.clone();
        for k in 0..5 * n {
            pert[k] += (delta[k]) * scale;
        }
    }

    let rho = if ratios.is_empty() {
        f64::NAN
    } else {
        // Geometric mean of the last half — the first half of the measurement
        // window is still shedding sub-dominant modes.
        let tail = &ratios[ratios.len() / 2..];
        (tail.iter().map(|r| r.ln()).sum::<f64>() / tail.len() as f64).exp()
    };

    // Dominant along-flow wavelength of the converged mode: DFT of the
    // y-averaged depth perturbation, in x.
    let mut prof = vec![0.0f64; nx];
    for x in 0..nx {
        for y in 0..nx {
            prof[x] += last_delta[5 * (y * nx + x)];
        }
        prof[x] /= nx as f64;
    }
    let mean_p: f64 = prof.iter().sum::<f64>() / nx as f64;
    for v in prof.iter_mut() {
        *v -= mean_p;
    }
    let (mut best_k, mut best_pw) = (0usize, -1.0f64);
    for k in 1..=nx / 2 {
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for (x, &v) in prof.iter().enumerate() {
            let ang = -2.0 * std::f64::consts::PI * (k * x) as f64 / nx as f64;
            re += v * ang.cos();
            im += v * ang.sin();
        }
        let pw = re * re + im * im;
        if pw > best_pw {
            best_pw = pw;
            best_k = k;
        }
    }
    let lam_cells = if best_k > 0 { nx as f64 / best_k as f64 } else { f64::INFINITY };
    let _ = g0;
    Growth { rho, sigma: rho.ln() / p.dt, lam_cells, lam_m: lam_cells * p.l, froude: fr, guards: g }
}

fn tilted(nx: usize, slope: f64, l: f64) -> Geom {
    Geom::Periodic { nx, g: slope * l, gy: 0.0 }
}

/// The kernel's own step at the level `water.rs` is documented for, scaled to
/// the cell size so the CFL number is the SAME on every grid in a refinement
/// ladder. (`dt = 0.2 s` at `l = 4.8 m` is the shipped pairing.)
fn dt_for(l: f64) -> f64 {
    0.2 * (l / 4.8)
}

fn main() {
    println!("=== ROLL-WAVE PROBE ===\n");

    // ── §1 Slope ladder: does the Jarrett/roll-wave separation survive? ─────
    // Reproduces the 2026-07-13 table with the parameters that are now REAL
    // `WaterParams` fields, and adds the Froude-cap arm that could not be run
    // then. The control holds `n` at the value Jarrett itself produces at this
    // base state (cap included) — the correction that entry records making.
    println!("── §1 SLOPE LADDER at l = 4.8 m, d₀ = 1 m, 64² periodic tilt ─────");
    println!("   slope     Fr      ρ SHIPPED    ρ n-CONSTANT    ρ θ=1(no smooth)  ρ no-cap    n     capped?");
    let l = 4.8;
    let d0 = 1.0;
    let nx = 64;
    for &s in &[0.02f64, 0.05, 0.08, 0.10, 0.20, 0.40, 0.70] {
        let geom = tilted(nx, s, l);
        let base = PipeParams { dt: dt_for(l), ..PipeParams::kernel_default(l) };
        let n_eff = (0.04 + 1.6 * s).min(0.13);
        let shipped = growth(&geom, &base, d0, 300, 300);
        let nconst = growth(&geom, &PipeParams { jarrett: false, manning_n: n_eff, ..base }, d0, 300, 300);
        let notheta = growth(&geom, &PipeParams { theta: 1.0, ..base }, d0, 300, 300);
        let nocap = growth(&geom, &PipeParams { froude_cap: 1.0e9, ..base }, d0, 300, 300);
        println!(
            "   {:5.0}%  {:5.2}   {:10.5}   {:12.5}   {:14.5}   {:9.5}  {:5.3}   {}",
            s * 100.0,
            shipped.froude,
            shipped.rho,
            nconst.rho,
            notheta.rho,
            nocap.rho,
            n_eff,
            if n_eff >= 0.13 { "YES" } else { "no" }
        );
    }
    println!();

    // ── §2 θ ladder — does θ remove the mode, or relocate it? ──────────────
    println!("── §2 θ LADDER at 70% slope (deep in the roll-wave regime) ───────");
    println!("   the 07-13 finding to confirm/refute: θ DAMPS and RELOCATES, never removes");
    println!("   θ        ρ         peak λ (cells)   peak λ (m)");
    {
        let s = 0.70;
        let geom = tilted(nx, s, l);
        for &th in &[1.0f64, 0.9, 0.8, 0.7, 0.5, 0.3] {
            let g = growth(&geom, &PipeParams { theta: th, dt: dt_for(l), ..PipeParams::kernel_default(l) }, d0, 300, 300);
            println!("   {th:4.2}   {:9.5}   {:13.2}   {:9.1}", g.rho, g.lam_cells, g.lam_m);
        }
    }
    println!();

    // ── §2b The base state itself depends on dt. Found while building §3. ──
    // The friction is applied with the PRE-friction velocity
    // (`let v = accel / (hflow*l)` before the implicit divide), so the steady
    // normal-flow solution is a function of Δt. That makes a refinement ladder
    // with `dt ∝ l` compare different base states — which invalidated the first
    // version of §3 below. Measured here rather than asserted.
    println!("── §2b The steady normal flow is a function of Δt ────────────────");
    println!("   70% slope, d₀ = 1 m, l = 4.8 m — ONLY dt varies. Manning's own answer");
    println!("   is dt-free: v = d^{{2/3}}·√S/n = {:.3} m/s ⇒ Fr = {:.3} (capped at 2.0).",
        d0.powf(2.0 / 3.0) * 0.70f64.sqrt() / 0.13,
        d0.powf(2.0 / 3.0) * 0.70f64.sqrt() / 0.13 / (9.8 * d0).sqrt());
    println!("   dt (s)     Fr of the relaxed base state     error vs the capped answer");
    {
        let geom = tilted(64, 0.70, 4.8);
        for &dt in &[0.8f64, 0.4, 0.2, 0.1, 0.05, 0.02, 0.01] {
            let p = PipeParams { dt, ..PipeParams::kernel_default(4.8) };
            let (base, _) = water_op::relax_to_steady(&geom, &p, d0, 20000);
            let fr = water_op::froude(&base, &geom, &p);
            println!("   {dt:5.3}      {fr:8.4}                        {:+7.2}%", 100.0 * (fr / 2.0 - 1.0));
        }
        println!("   ⇒ the shipped pairing (dt 0.2 s at l 4.8 m) sits ~8% slow, one-sided.");
    }
    println!();

    // ── §3 THE DISCRIMINATOR: refinement at fixed physical scale. ──────────
    // dt is held CONSTANT (not scaled with l) precisely because of §2b: with
    // dt ∝ l the base state changes down the ladder and the comparison is not
    // like-for-like. Constant dt = 0.02 s is CFL-safe on every grid here
    // (worst CFL = 0.052 at l = 1.2 m) and leaves Δx as the only variable.
    println!("── §3 GRID REFINEMENT at FIXED PHYSICAL SCALE ────────────────────");
    println!("   domain 307.2 m, d₀ = 1 m, dt = 0.02 s FIXED (see §2b), Δx the only variable");
    println!("   physical ⇒ σ converges and λ(m) is fixed · numerical ⇒ σ grows as Δx falls\n");
    let dt_fix = 0.02;
    for &s in &[0.05f64, 0.70] {
        let n_eff = (0.04 + 1.6 * s).min(0.13);
        println!(
            "   slope {:.0}%   (Jarrett n {} here; n-CONSTANT control holds n = {n_eff:.3})",
            s * 100.0,
            if 0.04 + 1.6 * s >= 0.13 { "CAPPED" } else { "LIVE" }
        );
        println!("      l (m)    nx      ρ SHIPPED   σ SHIPPED    ρ n-CONST   σ n-CONST    λ (cells)    λ (m)     Fr");
        for &(ll, nn) in &[(19.2f64, 16usize), (9.6, 32), (4.8, 64), (2.4, 128), (1.2, 256)] {
            let geom = tilted(nn, s, ll);
            let p = PipeParams { dt: dt_fix, ..PipeParams::kernel_default(ll) };
            let g = growth(&geom, &p, d0, 400, 400);
            let c = growth(&geom, &PipeParams { jarrett: false, manning_n: n_eff, ..p }, d0, 400, 400);
            println!(
                "      {ll:5.2}   {nn:4}    {:9.6}   {:+9.5}   {:9.6}   {:+9.5}   {:9.2}   {:8.1}   {:5.2}",
                g.rho, g.sigma, c.rho, c.sigma, g.lam_cells, g.lam_m, g.froude
            );
        }
        println!();
    }

    // ── §3b Instrument controls (`#norm-probe-sensitivity`). ───────────────
    println!("── §3b CONTROLS — the instrument must be able to say both words ──");
    {
        // KNOWN-GOOD: a flat bed with no flow is a fixed point; nothing can grow.
        let flat = tilted(64, 0.0, 4.8);
        let p = PipeParams { dt: dt_fix, ..PipeParams::kernel_default(4.8) };
        let g = growth(&flat, &p, d0, 400, 400);
        println!("   flat bed, no flow (must be ≤ 1):              ρ = {:.6}", g.rho);
        // KNOWN-BAD: a CFL-violating step must blow up.
        let steep = tilted(64, 0.05, 4.8);
        let bad = PipeParams { dt: 4.0, ..PipeParams::kernel_default(4.8) };
        let gb = growth(&steep, &bad, d0, 100, 100);
        println!("   5% slope at a CFL-VIOLATING dt = 4 s (must blow up): ρ = {:.6}", gb.rho);
    }
    println!();

    // ── §4 Same question, asked of the REAL kernel. ────────────────────────
    println!("── §4 REAL `WaterSim` twin experiment (no transcription) ─────────");
    real_kernel_twin();
    println!();

    // ── §5 The confound §3 exposed, isolated. ──────────────────────────────
    // At dt = 0.02 the 70% base state sits at Fr = 2.0000 — i.e. the BREAKING
    // CAP IS SATURATED IN THE BASE STATE. A saturated cap replaces the pipe's
    // momentum equation with an algebraic function of depth alone: the flux
    // loses its memory of itself, and a momentum perturbation cannot propagate.
    // So §3's "stable at 70%" may be measuring the cap having deleted the
    // dynamics rather than the flow being stable. Cap off, n held constant, and
    // let the base state be honest Manning normal flow.
    println!("── §5 CAP OFF: where does growth actually begin? ─────────────────");
    println!("   dt = 0.02 s, l = 4.8 m, 64², n held CONSTANT at 0.13 (no Jarrett feedback),");
    println!("   NO breaking cap ⇒ the base state is unclamped Manning normal flow.");
    println!("   Vedernikov for a wide Manning channel predicts onset at Fr = 1.5.\n");
    println!("   slope    Fr (base)     ρ        σ (1/s)     λ (cells)   λ (m)    verdict");
    for &s in &[0.05f64, 0.10, 0.20, 0.30, 0.40, 0.55, 0.70, 0.85, 1.00] {
        let geom = tilted(64, s, 4.8);
        let p = PipeParams {
            dt: dt_fix,
            jarrett: false,
            manning_n: 0.13,
            froude_cap: 1.0e9,
            ..PipeParams::kernel_default(4.8)
        };
        let g = growth(&geom, &p, d0, 400, 400);
        println!(
            "   {:5.0}%   {:8.3}   {:9.6}   {:+9.5}   {:9.2}   {:7.1}   {}",
            s * 100.0,
            g.froude,
            g.rho,
            g.sigma,
            g.lam_cells,
            g.lam_m,
            if g.rho > 1.000005 { "GROWS" } else { "stable" }
        );
    }
    println!();

    // ── §6 The discriminator again, on the UNCAPPED supercritical state. ───
    println!("── §6 REFINEMENT of the UNCAPPED supercritical flow ──────────────");
    println!("   n = 0.13 constant, no cap, dt = 0.02 s, domain 307.2 m — Δx the only variable");
    println!("   slope    l (m)    nx        ρ         σ (1/s)     λ (cells)    λ (m)      Fr");
    for &s in &[0.70f64, 1.00] {
        for &(ll, nn) in &[(19.2f64, 16usize), (9.6, 32), (4.8, 64), (2.4, 128), (1.2, 256)] {
            let geom = tilted(nn, s, ll);
            let p = PipeParams {
                dt: dt_fix,
                jarrett: false,
                manning_n: 0.13,
                froude_cap: 1.0e9,
                ..PipeParams::kernel_default(ll)
            };
            let g = growth(&geom, &p, d0, 400, 400);
            println!(
                "   {:5.0}%   {ll:5.2}   {nn:4}   {:9.6}   {:+9.5}   {:9.2}   {:8.1}   {:5.2}",
                s * 100.0,
                g.rho,
                g.sigma,
                g.lam_cells,
                g.lam_m,
                g.froude
            );
        }
        println!();
    }

    // ── §8 The 07-13 base state exactly: n = 0.04, θ = 1, no cap. ──────────
    // §5/§6 swept n = 0.13 (Jarrett's ceiling). The Brillouin map that produced
    // the roll-wave reading was taken at **n = 0.04, θ = 1, no cap, Fr 2.49** —
    // a different base state, and refuting a claim on a neighbouring base state
    // is not refuting it. Same two questions, on theirs.
    println!("── §8 THE 07-13 BASE STATE: n = 0.04 constant, θ = 1, no cap ─────");
    println!("   dt = 0.02 s, l = 4.8 m, 64²");
    println!("   slope    Fr (base)     ρ         σ (1/s)     λ (cells)   λ (m)    verdict");
    let p04 = |l: f64| PipeParams {
        dt: dt_fix,
        theta: 1.0,
        jarrett: false,
        manning_n: 0.04,
        froude_cap: 1.0e9,
        ..PipeParams::kernel_default(l)
    };
    for &s in &[0.02f64, 0.05, 0.10, 0.20, 0.40, 0.70] {
        let g = growth(&tilted(64, s, 4.8), &p04(4.8), d0, 400, 400);
        println!(
            "   {:5.0}%   {:8.3}   {:9.6}   {:+9.5}   {:9.2}   {:7.1}   {}",
            s * 100.0,
            g.froude,
            g.rho,
            g.sigma,
            g.lam_cells,
            g.lam_m,
            if g.rho > 1.000005 { "GROWS" } else { "stable" }
        );
    }
    println!("\n   and under refinement at 5% slope (their Fr ≈ 2.5 neighbourhood):");
    println!("   l (m)    nx        ρ         σ (1/s)     λ (cells)    λ (m)      Fr");
    for &(ll, nn) in &[(19.2f64, 16usize), (9.6, 32), (4.8, 64), (2.4, 128), (1.2, 256)] {
        let g = growth(&tilted(nn, 0.05, ll), &p04(ll), d0, 400, 400);
        println!(
            "   {ll:5.2}   {nn:4}   {:9.6}   {:+9.5}   {:9.2}   {:8.1}   {:5.2}",
            g.rho, g.sigma, g.lam_cells, g.lam_m, g.froude
        );
    }
    println!();

    // ── §9 The crux, isolated. ─────────────────────────────────────────────
    // §5 (n = 0.13) is stable everywhere and §8 (n = 0.04) grows everywhere —
    // but those two arms differ in θ as well as n (0.8 vs 1.0), so neither can
    // be attributed against the other. Here θ = 1.0 in BOTH, cap off, Jarrett
    // off, dt = 0.02 s: n is the only difference. If growth is the Vedernikov
    // instability it is a function of Fr ALONE and the two n columns must agree
    // wherever their Froude numbers agree.
    println!("── §9 IS THE GROWTH A FUNCTION OF Fr, OR OF n? ───────────────────");
    println!("   θ = 1.0 in both columns, no cap, no Jarrett, dt = 0.02 s, l = 4.8 m, 64²");
    println!("   Vedernikov (Manning, wide) says: unstable iff Fr > 1.5, INDEPENDENT of n.\n");
    println!("   slope        n = 0.04                      n = 0.13");
    println!("              Fr        ρ       verdict      Fr        ρ       verdict");
    for &s in &[0.005f64, 0.01, 0.02, 0.05, 0.10, 0.20, 0.40, 0.70] {
        let mk = |nn: f64| PipeParams {
            dt: dt_fix,
            theta: 1.0,
            jarrett: false,
            manning_n: nn,
            froude_cap: 1.0e9,
            ..PipeParams::kernel_default(4.8)
        };
        let a = growth(&tilted(64, s, 4.8), &mk(0.04), d0, 400, 400);
        let b = growth(&tilted(64, s, 4.8), &mk(0.13), d0, 400, 400);
        let v = |g: &Growth| if g.rho > 1.000005 { "GROWS " } else { "stable" };
        println!(
            "   {:5.1}%   {:6.3}   {:9.6}   {}   {:6.3}   {:9.6}   {}",
            s * 100.0,
            a.froude,
            a.rho,
            v(&a),
            b.froude,
            b.rho,
            v(&b)
        );
    }
    println!();

    // ── §7 The Jarrett artefact under refinement, with its own control. ────
    println!("── §7 The JARRETT artefact under refinement (5% slope, cap ON) ───");
    println!("   dt = 0.02 s, domain 307.2 m; the control holds n at Jarrett's own base value");
    println!("   l (m)     nx        ρ SHIPPED    σ SHIPPED     ρ n-CONST     λ (cells)   λ (m)");
    for &(ll, nn) in &[(4.8f64, 64usize), (2.4, 128), (1.2, 256), (0.6, 512)] {
        let geom = tilted(nn, 0.05, ll);
        let p = PipeParams { dt: dt_fix, ..PipeParams::kernel_default(ll) };
        let g = growth(&geom, &p, d0, 400, 400);
        let c = growth(&geom, &PipeParams { jarrett: false, manning_n: 0.12, ..p }, d0, 400, 400);
        println!(
            "   {ll:5.2}   {nn:5}   {:11.6}   {:+10.5}   {:11.6}   {:9.2}   {:7.1}",
            g.rho, g.sigma, c.rho, g.lam_cells, g.lam_m
        );
    }
}

/// The refinement question asked of the shipped `WaterSim` directly. No
/// periodic option exists there, so this is a long closed channel with a
/// sea-held outlet at the low end; growth is measured on the interior, away
/// from the outlet, over a window in which the base flow is quasi-steady.
fn real_kernel_twin() {
    use vivarium_world::sphere::Face;
    use vivarium_world::water::{WaterParams, WaterSim};

    println!("   long tilted channel, rain-fed, sea-held outlet; δ seeded at 1e-3 m");
    println!("   dt held CONSTANT at 0.02 s across the ladder, for the §2b reason.");
    println!("   l (m)    nx    dt (s)     ρ/step      σ (1/s)     max Fr");
    for &(l, nx) in &[(19.2f32, 32usize), (9.6, 64), (4.8, 128)] {
        let slope = 0.70f32;
        let _ = 0;
        // Low datum: at a 4000 m datum one f32 ULP of η is 2.4e-4 m, which would
        // swallow the perturbation (`#detail-nomos-defect-anatomy`, the float row).
        let bed: Vec<f32> = (0..nx * nx).map(|i| 60.0 - slope * l * (i % nx) as f32).collect();
        let dt = 0.02f32;
        let p = WaterParams {
            dt,
            sea_m: -1.0e6,
            precip: 2.0e-3,
            evaporation: 0.0,
            infiltration: 0.0,
            ocean_evap: 0.0,
            baseflow: 0.0,
            sed_capacity: 0.0,
            ..Default::default()
        };
        let mk = || {
            let mut w = WaterSim::new(Face::ZPos, 21, (0, 0), nx, l, bed.clone(), 1.0e6);
            for d in w.depth.iter_mut() {
                *d = 1.0;
            }
            w
        };
        // Spin up to a quasi-steady sheet.
        let mut a = mk();
        for _ in 0..15000 {
            a.step(&p);
        }
        let spun: Vec<f32> = a.depth.clone();
        // Twin: same trajectory, one interior cell nudged.
        let mut b = mk();
        for _ in 0..15000 {
            b.step(&p);
        }
        b.depth[(nx / 2) * nx + nx / 2] += 1.0e-3;

        let interior = |i: usize| {
            let (x, y) = (i % nx, i / nx);
            x > 3 && x + 4 < nx && y > 3 && y + 4 < nx
        };
        let mut ratios = Vec::new();
        let mut prev = 1.0e-3f64;
        let mut fr_max = 0.0f32;
        for _ in 0..2000 {
            a.step(&p);
            b.step(&p);
            fr_max = fr_max.max(a.froude().0);
            let nd: f64 = (0..nx * nx)
                .filter(|&i| interior(i))
                .map(|i| {
                    let d = (b.depth[i] as f64) - (a.depth[i] as f64);
                    d * d
                })
                .sum::<f64>()
                .sqrt();
            if nd > 0.0 && prev > 0.0 {
                ratios.push(nd / prev);
            }
            prev = nd;
        }
        let _ = spun;
        let tail = &ratios[ratios.len() / 2..];
        let rho = (tail.iter().map(|r| r.ln()).sum::<f64>() / tail.len() as f64).exp();
        println!("   {l:5.2}   {nx:4}   {dt:6.4}   {rho:10.6}   {:+10.5}   {fr_max:6.3}", rho.ln() / dt as f64);
    }
}
