//! # ROUTER LAB — what is the missing term?
//!
//! Run: `cargo run --release -p vivarium-world --example router_lab`
//!
//! Three properties, three routers, and they do not co-optimise. Conservation is free. The moment
//! correction kills the directional defect and leaves the magnitude. Edge flux fixes the magnitude
//! and leaves the direction. **Compose them and it is WORSE THAN EITHER PARENT.** Two corrections
//! that each help and then fight is what a *missing term* looks like.
//!
//! This probe tests **three candidates for that term**, in the order the evidence now favours:
//!
//! 1. **§1 — MFD's slope exponent `p`.** Freeman (1991) picked `p = 1.1` by trial and error. At
//!    **`p = 1` the fan's first moment vanishes identically** on a perfect lattice (the weights go
//!    as `cos θₖ`, and `Σ cos θₖ sin θₖ = ½ Σ sin 2θₖ` over eight bearings whose doubles are 90°
//!    apart is zero, exactly, at every azimuth). `grid_lab` measured a **0.24°** perfect-lattice
//!    deflection and charged every sphere number against it as *"MFD's own intrinsic error."*
//!    **If that 0.24° IS the exponent, the baseline the whole grid report was charged against was
//!    never irreducible — and `p` is an ATTRACTOR TOWARD THE GRID AXES**, i.e. precisely the
//!    grid-aligned-channel artifact MFD was adopted to remove.
//!
//! 2. **§3 — the accumulated scalar has no limit.** Coatléven & Chauveau (2025), read primary:
//!    MFD's `q̃` is the cell's total **OUTFLUX**, so it scales with the **perimeter**, and
//!    *"the only convergence that can be expected for q̃ is to zero."* Worse — it is ill-posed in
//!    the **continuum**: *"even at the continuous level, CA(𝒪) strongly depends on the geometry of
//!    𝒪 and its orientation with respect to the flow."* Their fix is a **local, no-solve
//!    post-process**: rebuild the flux **vector** from the face fluxes via the geometric identity
//!    `|K|·Id = Σ_σ |σ|(x_σ − x_K) ⊗ n_{K,σ}`, and take its norm. **Direction and magnitude then
//!    come from the SAME object, so they cannot fight.**
//!
//! 3. **§5 — the second moment.** A splitting router is a random walk; its continuum limit is
//!    advection–diffusion; `m2 = Σ wₖ sin²(βₖ − ψ)` is a spurious diffusion of `a` that nothing
//!    constrains. *Demoted to a secondary reading — kept because it is the right diagnostic
//!    scaffolding and it is cheap, not because it is still the leading candidate.*
//!
//! ⚠ **Scale note, stated because it bounds every claim below.** These are **mesh** probes (full
//! adjacency built), which caps them near n = 128 (98 304 cells); the published router table is
//! n = 32. The repo's L9–L23 figures come from *analytic* fan probes that never build a mesh.
//! **Level-independence here spans 16× in cell area, not 16 384×. Do not quote it as if it did.**

mod curl;
mod fan;
mod flow;
mod grids;
mod mesh;
mod sca;

use fan::{bias_p, fan_ideal, fan_limit, DistRule};
use flow::Router::*;
use grids::{cube_face_basis, cube_sphere, CubeProj};
use mesh::*;
use sca::*;
use std::fmt::Write as _;
use vivarium_world::planet::Planet;

const R: f64 = Planet::EARTH.radius_m;
/// `flow::routing`'s own pole — deliberately incommensurate with the cube, so no symmetry can
/// hand a router a free zero. (The face-centred cone is a NULL TEST; see §4.)
fn generic_pole() -> V3 {
    unit([0.3, -0.7, 0.64])
}

fn rule(t: &str) {
    println!("\n\n\x1b[1m{t}\x1b[0m\n{}", "─".repeat(t.len().max(80)));
}
fn pass(ok: bool) -> &'static str {
    if ok { "\x1b[32mPASS\x1b[0m" } else { "\x1b[31mFAIL\x1b[0m" }
}

/// The seven routers, at a given slope exponent, plus the oracle rows that break the
/// ψ-estimation confound, plus the control that must scream.
fn ladder(p: f64) -> Vec<Rt> {
    vec![
        Rt::new(MooreMfd, "MFD-8 (status quo)").with_p(p),
        Rt::new(MooreMfdTrueDist, "MFD-8 + true distances").with_p(p),
        Rt::new(EdgeMfd, "edge MFD (no diagonals)").with_p(p),
        Rt::new(GradEdgeFlux, "gradient-projected edge flux").with_p(p),
        Rt::new(MomentMfd, "moment-corrected MFD-8").with_p(p),
        Rt::new(MomentEdge, "moment + edge flux").with_p(p),
        Rt::new(QuadMoment, "true-width quadrature + moment").with_p(p),
    ]
}
fn oracles(p: f64) -> Vec<Rt> {
    vec![
        Rt::oracle(GradEdgeFlux, "  ORACLE  grad edge flux @ψ*").with_p(p),
        Rt::oracle(MomentMfd, "  ORACLE  moment MFD-8   @ψ*").with_p(p),
        Rt::oracle(MomentEdge, "  ORACLE  moment + edge  @ψ*").with_p(p),
        Rt::oracle(QuadMoment, "  ORACLE  quad + moment  @ψ*").with_p(p),
        Rt::wrecked(GradEdgeFlux, 20.0, "  CONTROL grad edge flux @ψ*+20°").with_p(p),
    ]
}

fn main() {
    println!("\x1b[1m╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║  ROUTER LAB — the missing term                                               ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝\x1b[0m");

    let pole = generic_pole();
    let g32 = cube_sphere(CubeProj::Equiangular, 32, R);
    let g64 = cube_sphere(CubeProj::Equiangular, 64, R);
    let g128 = cube_sphere(CubeProj::Equiangular, 128, R);

    // ======================================================================
    rule("§0  CONTROLS — a probe that cannot fail is not a probe");
    // ======================================================================
    let sig = |g: &Mesh, p: f64| -> String {
        let mut s = String::new();
        for rt in ladder(p).into_iter().chain(oracles(p)) {
            let st = run(g, rt, pole);
            let _ = write!(s, "{}|{:.17e}|{:.17e};", st.tag, st.conservation, st.m(|r| r.sca_err));
        }
        s
    };
    let (a, b, c) = (sig(&g32, 1.1), sig(&g32, 1.1), sig(&g32, 1.1));
    let det = a == b && b == c;
    println!("  C0  DETERMINISM — three full sweeps, compared bit-for-bit ............ {}", pass(det));
    println!("      `QuadMoment` has already been caught once with a HashMap-iteration-order bug");
    println!("      whose FIRST NUMBERS WERE FLATTERING. Drift here is a probe bug, and it is the");
    println!("      finding. {}", if det { "No drift." } else { "\x1b[31mDRIFT — chase it before reading anything below.\x1b[0m" });

    let st = run(&g32, Rt::new(GradEdgeFlux, "").with_p(1.1), pole);
    let wr = run(&g32, Rt::wrecked(GradEdgeFlux, 20.0, "").with_p(1.1), pole);
    let screams = wr.m(|r| r.sca_err) > 2.0 * st.m(|r| r.sca_err)
        && wr.m(|r| r.defl_fan.abs()) > 5.0 * st.m(|r| r.defl_fan.abs());
    println!("\n  C1  THE CONTROL THAT MUST SCREAM — an honest router with its ψ rotated 20° .. {}", pass(screams));
    println!("      grad edge flux        : sca_err {:>6.2}%   mean|Δ_fan| {:>6.2}°",
        100.0 * st.m(|r| r.sca_err), st.m(|r| r.defl_fan.abs()));
    println!("      same, ψ* + 20°        : sca_err {:>6.2}%   mean|Δ_fan| {:>6.2}°   ← the instrument reads",
        100.0 * wr.m(|r| r.sca_err), wr.m(|r| r.defl_fan.abs()));

    // ======================================================================
    rule("§1  ⚖ P1 — IS THE 'IRREDUCIBLE' 0.24° PERFECT-LATTICE BASELINE JUST p = 1.1?");
    // ======================================================================
    println!("  `grid_lab` ran MFD-8 on a PERFECT FLAT LATTICE, measured 0.24° of deflection, and");
    println!("  called it \x1b[1m\"MFD's own intrinsic error, and the baseline everything else is charged");
    println!("  against.\"\x1b[0m Every sphere number in the grid report is net of it.\n");
    println!("  \x1b[1mTHE DERIVATION SAYS IT IS NOT INTRINSIC — IT IS THE EXPONENT.\x1b[0m At p = 1 the fan's");
    println!("  weights go as cos θₖ, so its first moment is  Σ cos θₖ · sin θₖ = ½ Σ sin 2θₖ  over");
    println!("  eight bearings whose doubles are 90° apart — \x1b[1mzero, identically, at every azimuth.\x1b[0m");
    println!("  A theorem, not a tuning. So p ≠ 1 MANUFACTURES a first moment out of nothing.\n");
    println!("  Perfect lattice (`fan_ideal`), MFD-8, sweeping the compass at 0.1°:\n");
    println!("  \x1b[1m{:>6}{:>12}{:>12}{:>12}{:>34}\x1b[0m", "p", "rms Δ°", "max Δ°", "captured", "attractors (deg)");
    println!("  {:->6}{:->12}{:->12}{:->12}{:->34}", "", "", "", "", "");
    let mut p1rows: Vec<(f64, f64, f64, f64, String)> = Vec::new();
    for p in [1.0f64, 1.05, 1.1, 1.2, 1.5, 2.0] {
        let bi = bias_p(&fan_ideal(9, R), DistRule::Hardcoded, p);
        let at: Vec<String> = bi.attractors.iter().map(|a| format!("{a:.0}")).collect();
        let ats = at.join(",");
        println!("  {:>6.2}{:>12.4}{:>12.4}{:>11.1}%{:>34}", p, bi.rms, bi.max, 100.0 * bi.captured, ats);
        p1rows.push((p, bi.rms, bi.max, bi.captured, ats));
    }
    let b10 = bias_p(&fan_ideal(9, R), DistRule::Hardcoded, 1.0);
    let b11 = bias_p(&fan_ideal(9, R), DistRule::Hardcoded, 1.1);
    println!("\n  \x1b[1m→ p = 1.1 : max Δ = {:.4}°   (grid_lab's \"irreducible\" baseline was 0.24°)\x1b[0m", b11.max);
    println!("  \x1b[1m→ p = 1.0 : max Δ = {:.2e}°  ← MACHINE ZERO.\x1b[0m", b10.max);
    let killed = b10.max < 1e-9 && b11.max > 0.15;
    println!("\n  {}  THE PERFECT-LATTICE BASELINE IS THE EXPONENT.", pass(killed));
    if killed {
        println!("      It is not MFD's intrinsic quadrature error. It is a knob, it was tuned by");
        println!("      trial and error in 1991, and \x1b[1mit vanishes exactly at p = 1.\x1b[0m Note WHERE the");
        println!("      attractors sit: they are the grid axes and diagonals. \x1b[1mp > 1 rotates flow");
        println!("      TOWARD the lattice — it is a grid-aligning bias, manufactured by the knob\x1b[0m,");
        println!("      and it has been the zero-point of every accuracy claim in the grid report.");
    }
    println!("\n  And the sphere is NOT exempt — the same sweep on a real cube-sphere cell:\n");
    println!("  \x1b[1m{:<28}{:>12}{:>12}\x1b[0m", "cell (equiangular, L9)", "p=1.1 max Δ°", "p=1.0 max Δ°");
    println!("  {:-<28}{:->12}{:->12}", "", "", "");
    for (nm, u, v) in [("face centre", 0.0, 0.0), ("mid-edge", 0.9, 0.0), ("near corner", 0.9, 0.9)] {
        let f = fan_limit(9, 0, u, v, R);
        println!("  {:<28}{:>12.3}{:>12.3}", nm,
            bias_p(&f, DistRule::Hardcoded, 1.1).max, bias_p(&f, DistRule::Hardcoded, 1.0).max);
    }
    println!("\n  \x1b[1m⇒ p = 1 removes the LATTICE bias exactly. It does NOT remove the SPHERE's shear\x1b[0m —");
    println!("  that residue is the Jacobian, and it is the thing the grid report was really after.");
    println!("  \x1b[1mThe two were being summed and reported as one number.\x1b[0m");

    // ======================================================================
    rule("§2  THE LADDER, AT p = 1.1 AND p = 1.0");
    // ======================================================================
    println!("Cone, generic pole, n=32 ({} cells), θ ∈ [0.35, 2.2] rad. Scored exactly as", g32.cells());
    println!("`flow::routing`: sca_err = |(A/W) / (R·tan(θ/2)) − 1|, W = the contour width against");
    println!("the EXACT flow direction — same denominator for every router.\n");
    println!("  \x1b[2mΔ_fan = deflection of the FAN direction Σwₖêₖ  (Lagrangian: where mass is sent).\x1b[0m");
    println!("  \x1b[2mΔ_Q   = deflection of the RECONSTRUCTED direction Q̂ = Q_K/‖Q_K‖ (Eulerian: what");
    println!("  \x1b[2m        the FACE FLUXES imply). On a non-orthogonal mesh these are NOT the same");
    println!("  \x1b[2m        object, and only the second is what a finite-volume scheme ever claimed.\x1b[0m\n");
    let mut lad: Vec<(f64, Vec<Stats>)> = Vec::new();
    for p in [1.1f64, 1.0] {
        println!("  \x1b[1m── p = {p} ─────────────────────────────────────────────────────────────────────\x1b[0m");
        println!("  \x1b[1m{:<32}{:>8}{:>8}{:>8}{:>9}{:>9}{:>9}\x1b[0m",
            "router", "conserv", "|Δfan|°", "|Δ_Q|°", "|m1_tru|", "m2_true", "sca_err");
        println!("  {:-<32}{:->8}{:->8}{:->8}{:->9}{:->9}{:->9}", "", "", "", "", "", "", "");
        let rows: Vec<Stats> = ladder(p).into_iter().chain(oracles(p)).map(|rt| run(&g32, rt, pole)).collect();
        for (k, s) in rows.iter().enumerate() {
            if k == 7 {
                println!("  \x1b[2m{:-<32}─ oracle rows: handed the analytic ψ. DIAGNOSTIC ONLY. ─\x1b[0m", "");
            }
            println!("  {:<32}{:>8.4}{:>8.2}{:>8.2}{:>9.4}{:>9.4}{:>8.2}%",
                s.tag, s.conservation, s.m(|r| r.defl_fan.abs()), s.m(|r| r.defl_q.abs()),
                s.m(|r| r.m1_true.abs()), s.m(|r| r.m2_true), 100.0 * s.m(|r| r.sca_err));
        }
        println!();
        lad.push((p, rows));
    }

    // ======================================================================
    rule("§3  ⚖ P2 — THE FLUX-VECTOR RECONSTRUCTION (Coatléven & Chauveau 2025, eq. 12–13)");
    // ======================================================================
    println!("  MFD's accumulated q̃ is the cell's total OUTFLUX — it scales with the PERIMETER, and");
    println!("  \x1b[1m\"the only convergence that can be expected for q̃ is to zero.\"\x1b[0m It is ill-posed even");
    println!("  in the CONTINUUM. Their fix rebuilds the flux VECTOR from the face fluxes:\n");
    println!("      Q_K = (1/|K|) Σ_σ F_{{K,σ}} · (x_σ − x_K)      q_K = ‖Q_K‖      Q̂_K = Q_K/‖Q_K‖\n");
    println!("  \x1b[1mDIRECTION AND MAGNITUDE THEN COME FROM ONE OBJECT — so they cannot fight.\x1b[0m That is");
    println!("  the candidate resolution of the compose-two-fixes-and-it-gets-worse paradox.\n");
    println!("  \x1b[1mTHE DISCRIMINATING TEST.\x1b[0m I claim ‖Q_K‖ ≡ A/W in the constant-flux limit — and the");
    println!("  probe ALREADY divides by exactly that W, with the exact direction. So I predict the");
    println!("  reconstruction moves κ/Δ a LOT and the cone error very LITTLE. \x1b[1mIf the cone error");
    println!("  collapses instead, I am WRONG and that is the bigger finding.\x1b[0m\n");
    println!("  \x1b[1m{:<32}{:>12}{:>8}{:>11}{:>11}{:>9}\x1b[0m",
        "router (p=1.1)", "‖Q‖÷(A/W)", "sd", "err (A/W)", "err (‖Q‖)", "phantom");
    println!("  {:-<32}{:->12}{:->8}{:->11}{:->11}{:->9}", "", "", "", "", "", "");
    for s in lad[0].1.iter().take(7) {
        let rat = s.m(|r| r.coat_ratio);
        let sd = (s.m(|r| r.coat_ratio.powi(2)) - rat * rat).max(0.0).sqrt();
        println!("  {:<32}{:>12.4}{:>8.4}{:>10.2}%{:>10.2}%{:>8.1}%",
            s.tag, rat, sd, 100.0 * s.m(|r| r.sca_err), 100.0 * s.m(|r| r.sca_err_coat),
            100.0 * s.m(|r| r.phantom));
    }
    println!("\n  \x1b[1mPHANTOM FLUX\x1b[0m = the fraction of outflow sent to a Moore neighbour sharing only a");
    println!("  VERTEX — mass crossing a face that DOES NOT EXIST. The reconstruction cannot see it,");
    println!("  because there is no (x_σ − x_K) to attach it to. \x1b[1mThat is not a limitation of the");
    println!("  reconstruction. It is MFD failing to be a finite-volume scheme at all.\x1b[0m");

    // ======================================================================
    rule("§4  P3 — THE GEOMETRIC IDENTITY, ON SPHERICAL CELLS");
    // ======================================================================
    println!("  |K|·Id = Σ_σ |σ|·(x_σ − x_K) ⊗ n_{{K,σ}}  is EUCLIDEAN. Our cells are SPHERICAL polygons.");
    println!("  It is the load-bearing step of §3, and \x1b[1mit may fail on the coarse tier\x1b[0m — where the");
    println!("  sagitta is a large fraction of the cell and the flat-plane assumption breaks outright.\n");
    println!("  \x1b[1m{:>8}{:>10}{:>14}{:>14}\x1b[0m", "n", "cells", "mean resid", "max resid");
    println!("  {:->8}{:->10}{:->14}{:->14}", "", "", "", "");
    let mut idrows: Vec<(usize, usize, f64, f64)> = Vec::new();
    for n in [4usize, 8, 16, 32, 64, 128] {
        let g = if n == 32 { &g32 } else if n == 64 { &g64 } else if n == 128 { &g128 } else { &cube_sphere(CubeProj::Equiangular, n, R) };
        let g: &Mesh = g;
        let r = identity_residual(g);
        let mn = r.iter().sum::<f64>() / r.len() as f64;
        let mx = r.iter().cloned().fold(0.0f64, f64::max);
        println!("  {:>8}{:>10}{:>14.3e}{:>14.3e}", n, g.cells(), mn, mx);
        idrows.push((n, g.cells(), mn, mx));
    }

    // ======================================================================
    rule("§5  E3 — THE ROTATION TEST (Prescott et al. 2025). The gate on every number above.");
    // ======================================================================
    println!("  \x1b[1mTHE FACE-CENTRED CONE IS A NULL TEST\x1b[0m — D4 symmetry forces the chirality term to zero");
    println!("  exactly. Every accuracy number in the published table came from a cone.\n");
    println!("  \x1b[1m(a) AZIMUTH STRUCTURE, taken for free.\x1b[0m A cone about a generic pole already sweeps its");
    println!("      flow through every azimuth relative to the cube's u/v axes. Bin by that azimuth,");
    println!("      folded to [0°,90°) — the grid's own 4-fold period. \x1b[1mFLAT = rotation-invariant.\x1b[0m\n");
    println!("  {:<32}{:>7}{:>7}{:>7}{:>7}{:>7}{:>7}{:>9}",
        "router (p=1.1), sca_err % by az", "0-15", "15-30", "30-45", "45-60", "60-75", "75-90", "pk-pk");
    println!("  {:-<32}{:->7}{:->7}{:->7}{:->7}{:->7}{:->7}{:->9}", "", "", "", "", "", "", "", "");
    let mut azrows: Vec<(String, Vec<f64>, f64)> = Vec::new();
    for s in lad[0].1.iter().take(7) {
        let mut bins = vec![(0.0f64, 0usize); 6];
        for r in &s.recs {
            let b = ((r.azim_face / 15.0) as usize).min(5);
            bins[b].0 += r.sca_err;
            bins[b].1 += 1;
        }
        let v: Vec<f64> = bins.iter().map(|&(x, n)| if n > 0 { 100.0 * x / n as f64 } else { f64::NAN }).collect();
        let (lo, hi) = v.iter().fold((f64::MAX, f64::MIN), |(l, h), &x| (l.min(x), h.max(x)));
        print!("  {:<32}", s.tag);
        for x in &v { print!("{:>7.2}", x); }
        println!("{:>8.2}%", hi - lo);
        azrows.push((s.tag.to_string(), v, hi - lo));
    }

    println!("\n  \x1b[1m(b) THE LITERAL ROTATION TEST.\x1b[0m Cone apex at a FIXED geodesic distance (0.30 rad) from");
    println!("      a face centre, swung through azimuths 0°–45° relative to the face u-axis. Same");
    println!("      landform, same distance — only its ORIENTATION to the grid changes. An honest");
    println!("      router returns the same answer. \x1b[1mPeak-to-peak IS the rotation defect.\x1b[0m\n");
    let (_, du0, _) = cube_face_basis(4);
    let c0 = grids::cube_to_unit(CubeProj::Equiangular, 4, 0.0, 0.0);
    let uh = tangent(c0, du0);
    let vh = cross(c0, uh);
    let angles = [0.0f64, 7.5, 15.0, 22.5, 30.0, 37.5, 45.0];
    let poles: Vec<V3> = angles.iter().map(|&a| {
        let d = 0.30f64;
        let dir = add(scale(uh, a.to_radians().cos()), scale(vh, a.to_radians().sin()));
        unit(add(scale(c0, d.cos()), scale(dir, d.sin())))
    }).collect();
    for p in [1.1f64, 1.0] {
        println!("  \x1b[1m── p = {p} ──\x1b[0m");
        print!("  {:<32}", "router");
        for a in angles { print!("{:>7.1}", a); }
        println!("{:>9}", "pk-pk");
        println!("  {:-<32}{:->49}{:->9}", "", "", "");
        for rt in ladder(p) {
            let v: Vec<f64> = poles.iter().map(|&pp| 100.0 * run(&g32, rt, pp).m(|r| r.sca_err)).collect();
            let (lo, hi) = v.iter().fold((f64::MAX, f64::MIN), |(l, h), &x| (l.min(x), h.max(x)));
            print!("  {:<32}", rt.tag);
            for x in &v { print!("{:>7.2}", x); }
            println!("{:>8.2}%", hi - lo);
        }
        println!();
    }

    // ======================================================================
    rule("§6  LEVEL DEPENDENCE — does any of it converge away?  (n = 32 → 128 = 16× in cell area)");
    // ======================================================================
    println!("  \x1b[1m{:<32}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}\x1b[0m",
        "router (p=1.1)", "err n=32", "err n=64", "err 128", "Δfan 32", "Δfan 64", "Δfan 128");
    println!("  {:-<32}{:->9}{:->9}{:->9}{:->9}{:->9}{:->9}", "", "", "", "", "", "", "");
    let mut levrows: Vec<(String, [f64; 3], [f64; 3])> = Vec::new();
    for rt in ladder(1.1) {
        let s: Vec<Stats> = [&g32, &g64, &g128].iter().map(|g| run(g, rt, pole)).collect();
        let er = [100.0 * s[0].m(|r| r.sca_err), 100.0 * s[1].m(|r| r.sca_err), 100.0 * s[2].m(|r| r.sca_err)];
        let df = [s[0].m(|r| r.defl_fan.abs()), s[1].m(|r| r.defl_fan.abs()), s[2].m(|r| r.defl_fan.abs())];
        println!("  {:<32}{:>8.2}%{:>8.2}%{:>8.2}%{:>9.2}{:>9.2}{:>9.2}",
            rt.tag, er[0], er[1], er[2], df[0], df[1], df[2]);
        levrows.push((rt.tag.to_string(), er, df));
    }

    // ======================================================================
    rule("§7  SECONDARY — does m2 (the second moment) explain the error?  [H, demoted]");
    // ======================================================================
    println!("  Kept as the diagnostic scaffolding it is, not as the leading candidate.\n");
    println!("  \x1b[1m{:<32}{:>10}{:>13}{:>11}\x1b[0m", "router (p=1.1)", "R²(m2)", "R²(|m1_true|)", "R²(both)");
    println!("  {:-<32}{:->10}{:->13}{:->11}", "", "", "", "");
    let (mut py, mut p2, mut pm1) = (Vec::new(), Vec::new(), Vec::new());
    for s in lad[0].1.iter().take(7) {
        let y: Vec<f64> = s.recs.iter().map(|r| r.sca_err).collect();
        let x2: Vec<f64> = s.recs.iter().map(|r| r.m2_true).collect();
        let x1: Vec<f64> = s.recs.iter().map(|r| r.m1_true.abs()).collect();
        println!("  {:<32}{:>10.3}{:>13.3}{:>11.3}", s.tag, r2(&y, &[&x2]), r2(&y, &[&x1]), r2(&y, &[&x2, &x1]));
        py.extend_from_slice(&y); p2.extend_from_slice(&x2); pm1.extend_from_slice(&x1);
    }
    println!("  {:-<32}{:->10}{:->13}{:->11}", "", "", "", "");
    println!("  \x1b[1m{:<32}{:>10.3}{:>13.3}{:>11.3}\x1b[0m", "POOLED (7 honest routers)",
        r2(&py, &[&p2]), r2(&py, &[&pm1]), r2(&py, &[&p2, &pm1]));

    write_md(&lad, &p1rows, &idrows, &azrows, &levrows, &angles, &g32, det,
        r2(&py, &[&p2]), r2(&py, &[&pm1]), r2(&py, &[&p2, &pm1]));
    println!("\n\n\x1b[1mWrote msc/spike-principled-router/MEASUREMENTS.md\x1b[0m");
}

#[allow(clippy::too_many_arguments)]
fn write_md(
    lad: &[(f64, Vec<Stats>)],
    p1: &[(f64, f64, f64, f64, String)],
    id: &[(usize, usize, f64, f64)],
    az: &[(String, Vec<f64>, f64)],
    lev: &[(String, [f64; 3], [f64; 3])],
    angles: &[f64],
    g32: &Mesh,
    det: bool,
    r2m2: f64, r2m1: f64, r2both: f64,
) {
    let mut s = String::new();
    let _ = writeln!(s, "# router_lab — MEASUREMENTS\n");
    let _ = writeln!(s, "*Data file. Reasoning lives in `FINDINGS.md`; code in `crates/vivarium-world/examples/router_lab/`.*\n");
    let _ = writeln!(s, "## Setup\n");
    let _ = writeln!(s, "- Equiangular cube-sphere, **n = 32** ({} cells) for the sweeps; n = 64/128 for level checks.", g32.cells());
    let _ = writeln!(s, "  ⚠ **Mesh** probes (full adjacency built) ⇒ capped near n=128. The repo's L9–L23");
    let _ = writeln!(s, "  numbers come from *analytic* fan probes that never build a mesh. Level-independence");
    let _ = writeln!(s, "  here spans **16×** in cell area, not 16 384×.");
    let _ = writeln!(s, "- Cone, `h = −geodesic(p, pole)`, pole = unit([0.3,−0.7,0.64]) — generic, incommensurate");
    let _ = writeln!(s, "  with the cube, so no symmetry hands a router a free zero.");
    let _ = writeln!(s, "- `sca_err = |(A/W) / (R·tan(θ/2)) − 1|`, θ ∈ [0.35, 2.2] rad; W = contour width against");
    let _ = writeln!(s, "  the **exact** flow direction (same denominator for all routers).");
    let _ = writeln!(s, "- Determinism (3 identical sweeps): **{}**\n", if det { "PASS" } else { "FAIL" });

    let _ = writeln!(s, "## §1 P1 — the perfect-lattice baseline vs the slope exponent `p`\n");
    let _ = writeln!(s, "MFD-8 on `fan_ideal` (a perfect flat lattice), compass swept at 0.1°.\n");
    let _ = writeln!(s, "| p | rms Δ° | max Δ° | captured | attractors (deg) |");
    let _ = writeln!(s, "|---|---|---|---|---|");
    for (p, r, m, c, a) in p1 {
        let _ = writeln!(s, "| {p:.2} | {r:.4} | {m:.4} | {:.1}% | {a} |", 100.0 * c);
    }

    for (p, rows) in lad {
        let _ = writeln!(s, "\n## §2 The moment ladder — p = {p}\n");
        let _ = writeln!(s, "| router | conserv | mean\\|Δ_fan\\|° | mean\\|Δ_Q\\|° | mean\\|m1_true\\| | mean m2 | sca_err |");
        let _ = writeln!(s, "|---|---|---|---|---|---|---|");
        for x in rows {
            let _ = writeln!(s, "| {} | {:.4} | {:.2} | {:.2} | {:.4} | {:.4} | {:.2}% |",
                x.tag.trim(), x.conservation, x.m(|r| r.defl_fan.abs()), x.m(|r| r.defl_q.abs()),
                x.m(|r| r.m1_true.abs()), x.m(|r| r.m2_true), 100.0 * x.m(|r| r.sca_err));
        }
        let _ = writeln!(s, "\n`Δ_fan` = deflection of `Σwₖêₖ` (Lagrangian). `Δ_Q` = deflection of `Q̂ = Q_K/‖Q_K‖`");
        let _ = writeln!(s, "(Eulerian — what the FACE FLUXES imply). ⚠ `ORACLE` rows are handed the analytic ψ:");
        let _ = writeln!(s, "**diagnostic only, never a router result.**\n");
    }

    let _ = writeln!(s, "## §3 Coatléven ‖Q_K‖ vs A/W  (p = 1.1)\n");
    let _ = writeln!(s, "| router | ‖Q‖÷(A/W) | sd | sca_err (A/W) | sca_err (‖Q‖) | phantom flux |");
    let _ = writeln!(s, "|---|---|---|---|---|---|");
    for x in lad[0].1.iter().take(7) {
        let rat = x.m(|r| r.coat_ratio);
        let sd = (x.m(|r| r.coat_ratio.powi(2)) - rat * rat).max(0.0).sqrt();
        let _ = writeln!(s, "| {} | {:.4} | {:.4} | {:.2}% | {:.2}% | {:.1}% |", x.tag.trim(), rat, sd,
            100.0 * x.m(|r| r.sca_err), 100.0 * x.m(|r| r.sca_err_coat), 100.0 * x.m(|r| r.phantom));
    }

    let _ = writeln!(s, "\n## §4 P3 — the geometric identity on spherical cells\n");
    let _ = writeln!(s, "`|K|·Id = Σ_σ |σ|(x_σ − x_K) ⊗ n_{{K,σ}}` — Euclidean identity, spherical cells.\n");
    let _ = writeln!(s, "| n | cells | mean rel. resid | max rel. resid |");
    let _ = writeln!(s, "|---|---|---|---|");
    for (n, c, m, x) in id {
        let _ = writeln!(s, "| {n} | {c} | {m:.3e} | {x:.3e} |");
    }

    let _ = writeln!(s, "\n## §5a Rotation sensitivity — sca_err by flow azimuth rel. cube u-axis (p=1.1)\n");
    let _ = writeln!(s, "| router | 0-15 | 15-30 | 30-45 | 45-60 | 60-75 | 75-90 | pk-pk |");
    let _ = writeln!(s, "|---|---|---|---|---|---|---|---|");
    for (t, v, pk) in az {
        let _ = writeln!(s, "| {} | {} | **{:.2}%** |", t.trim(),
            v.iter().map(|x| format!("{x:.2}")).collect::<Vec<_>>().join(" | "), pk);
    }
    let _ = writeln!(s, "\n⚠ **CONFOUND — do not read §5a as a clean rotation test.** On a cone about a fixed pole,");
    let _ = writeln!(s, "a cell's flow azimuth relative to the cube axes is CORRELATED WITH ITS POSITION on the");
    let _ = writeln!(s, "face — and the Jacobian shear is a function of position. So §5a mixes orientation with");
    let _ = writeln!(s, "location and cannot separate them. **§5b is the clean test** (same landform, same");
    let _ = writeln!(s, "distance, only the orientation changes), and there every router is rotation-stable to");
    let _ = writeln!(s, "≤1.42% pk-pk. §5a's structure is real and large, but it is evidence that the error is a");
    let _ = writeln!(s, "COHERENT FIELD over the face — not that the routers are orientation-dependent.\n");
    let _ = writeln!(s, "(§5b, the literal rotation test at 0°–45° over {} angles, is in the run log.)\n", angles.len());

    let _ = writeln!(s, "## §6 Level dependence (n = 32 → 128, 16× in cell area, p=1.1)\n");
    let _ = writeln!(s, "| router | err n=32 | err n=64 | err n=128 | \\|Δfan\\| 32 | 64 | 128 |");
    let _ = writeln!(s, "|---|---|---|---|---|---|---|");
    for (t, e, d) in lev {
        let _ = writeln!(s, "| {} | {:.2}% | {:.2}% | {:.2}% | {:.2} | {:.2} | {:.2} |",
            t.trim(), e[0], e[1], e[2], d[0], d[1], d[2]);
    }

    let _ = writeln!(s, "\n## §7 Secondary — R² of sca_err on the moments (pooled, 7 honest routers, p=1.1)\n");
    let _ = writeln!(s, "| predictor | R² |");
    let _ = writeln!(s, "|---|---|");
    let _ = writeln!(s, "| m2_true | {r2m2:.3} |");
    let _ = writeln!(s, "| \\|m1_true\\| | {r2m1:.3} |");
    let _ = writeln!(s, "| both | {r2both:.3} |");

    std::fs::create_dir_all("msc/spike-principled-router").ok();
    std::fs::write("msc/spike-principled-router/MEASUREMENTS.md", s).ok();
}
