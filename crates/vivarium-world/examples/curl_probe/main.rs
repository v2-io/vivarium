//! # THE CURL PROBE
//!
//! *"Gravity-driven flow is a gradient flow ⇒ ∇×∇φ ≡ 0. Does MFD's biased fan produce a flow
//! field with nonzero discrete curl? If yes, our routing makes water circulate on hillsides —
//! not an accuracy problem, a violated identity, and refinement will never fix it."*
//! — `.super-archive/from-theory/discretisation-and-information.md` §6 probe 2; structure table §4.1 row 2.
//!
//! Run:  `cargo run --release -p vivarium-world --example curl_probe`
//!
//! The identity as it must ACTUALLY be stated, the operator, and why its control cannot fail:
//! see `curl.rs`'s module docs — that reasoning is the load-bearing part of this spike.
//! Predictions were committed BEFORE the first run: `msc/spike-curl-probe/PREDICTIONS.md`.
//! Findings: `msc/spike-curl-probe/FINDINGS.md`.

mod curl;
mod fan;
mod flow;
mod grids;
mod mesh;

use curl::*;
use mesh::*;
use vivarium_world::planet::Planet;

const R: f64 = Planet::EARTH.radius_m;
/// The level `erosion.rs` actually runs the fluvial kernel at (~19 m cells).
const L19: u32 = 19;
/// Contour samples. §0's convergence gate is what licenses this number.
const M: usize = 20_000;

fn rule(t: &str) {
    println!("\n\n\x1b[1m{t}\x1b[0m\n{}", "─".repeat(t.len().max(62)));
}
fn pass(ok: bool) -> &'static str {
    if ok { "\x1b[32mPASS\x1b[0m" } else { "\x1b[31mFAIL\x1b[0m" }
}

fn main() {
    println!("\x1b[1m╔════════════════════════════════════════════════════════════════════════════╗");
    println!("║  THE CURL PROBE — is the MFD fan bias a violated TOPOLOGICAL IDENTITY?      ║");
    println!("╚════════════════════════════════════════════════════════════════════════════╝\x1b[0m\n");
    println!("  The identity is NOT '∇×q = 0'. That is FALSE for real water: q = −K∇φ with a");
    println!("  varying conveyance K has ∇×q = −∇K×∇φ ≠ 0. A probe built on it would convict a");
    println!("  perfect router and would have no honest control. The exact, mesh-independent,");
    println!("  resolution-independent content of 'water flows downhill' is:\n");
    println!("      \x1b[1mthe flow direction is ORTHOGONAL to the contours of φ\x1b[0m");
    println!("      ⇒  κ(C) = ∮_C d̂·dl / ∮_C dl = ⟨sin Δ⟩ = 0  on every closed CONTOUR C,");
    println!("         pointwise — so on every mesh, at every level, exactly.\n");
    println!("  \x1b[1mκ ≠ 0  ⟺  net transport AROUND the hill  ⟺  the router's direction field is");
    println!("  the gradient flow of NO single-valued potential.\x1b[0m The implied head does not close:");
    println!("  go round the loop, descend, and arrive back where you started. An Escher staircase.\n");
    println!("  ⚠ And the trap this probe exists to avoid: an ATTRACTOR is a CONVERGENT");
    println!("    deformation, and convergence is IRROTATIONAL. The measured 474 km plume drift");
    println!("    is therefore \x1b[1mnot\x1b[0m evidence of curl, and must not stand in for it.");

    let face_centre = grids::cube_to_unit(grids::CubeProj::Equiangular, 0, 0.0, 0.0);
    let edge_mid = unit([1.0, 0.0, 1.0]);
    let corner = unit([1.0, 1.0, 1.0]);
    let generic = unit([0.3, -0.7, 0.64]); // flow.rs's own pole — incommensurate with the cube
    let cone_g = Cone { pole: generic };

    // ==================================================================
    rule("§0  CONTROLS — a probe that cannot fail is not a probe");
    // ==================================================================
    println!("Cone, generic pole, θ = 0.9 rad, L19, {M} samples. `mean|Δ|` is the pointwise fan");
    println!("deflection, printed beside κ so that \"κ≈0 with a LARGE bias\" is visibly different");
    println!("from \"κ≈0 with no bias\". That difference is the entire point of the probe.\n");
    println!("  {:<46}{:>13}{:>10}   {}", "control", "κ", "mean|Δ|°", "");
    println!("  {:-<46}{:->13}{:->10}", "", "", "");

    let c0 = contour_circulation(Router::ExactControl, &cone_g, generic, 0.9, L19, R, M);
    println!("  {:<46}{:>13.1e}{:>10.2}   {}", "C0  exact gradient  (E ≡ 0, structurally)", c0.kappa, c0.mean_abs_defl_deg, pass(c0.kappa == 0.0));
    println!("      the zero is ALGEBRAIC, not a tolerance: every summand is identically 0.\n");

    let c2 = contour_circulation(Router::RotConstControl(15.0), &cone_g, generic, 0.9, L19, R, M);
    let exact15 = 15f64.to_radians().sin();
    println!("  {:<46}{:>13.6}{:>10.2}   {}", "C2  rotate +15° const  (exact κ = +0.258819)", c2.kappa, c2.mean_abs_defl_deg, pass((c2.kappa - exact15).abs() < 2e-3));
    println!("      \x1b[1mthe probe CAN see circulation\x1b[0m — so a zero it reports means something.");
    println!("      \x1b[2m(this control earned its keep: it caught a flipped loop orientation on the");
    println!("       first run — right magnitude, wrong sign, invisible to every other check.)\x1b[0m\n");

    let c3 = contour_circulation(Router::RotQuadControl(15.0, generic), &cone_g, generic, 0.9, L19, R, M);
    println!("  {:<46}{:>13.1e}{:>10.2}   {}", "C3  rotate 15°·cos2ψ  (odd around the loop)", c3.kappa, c3.mean_abs_defl_deg, pass(c3.kappa.abs() < 5e-3));
    println!("      \x1b[1m→ THE DISCRIMINATING CONTROL.\x1b[0m ~10° of pointwise bias, and κ vanishes.");
    println!("      A large deflection CAN integrate to nothing. So this probe is not the fan");
    println!("      probe renamed: it measures CIRCULATION, not deflection magnitude.\n");

    let c1 = contour_circulation(Router::Mfd8FlatControl, &cone_g, generic, 0.9, L19, R, M);
    println!("  {:<46}{:>13.1e}{:>10.2}   {}", "C1  MFD-8 on a PERFECT FLAT lattice", c1.kappa, c1.mean_abs_defl_deg, pass(c1.kappa.abs() < 1e-3));
    println!("      MFD's OWN intrinsic floor with the sphere removed. Every sphere number below");
    println!("      is charged against this, so the grid is not billed for MFD's own quadrature.\n");

    println!("  \x1b[1mC4  sampling-convergence gate\x1b[0m — is κ a converged integral, or is it noise?");
    println!("      A contour at L19 crosses ~10⁶ cells; sampling it at M points is only honest if");
    println!("      κ has CONVERGED in M. If it were noise it would fall as 1/√M. It does not move:\n");
    print!("        MFD-8:  ");
    for m in [1_440usize, 5_000, 20_000, 80_000] {
        let c = contour_circulation(Router::Mfd8, &cone_g, generic, 0.9, L19, R, m);
        print!("M={m:<6} κ={:>10.3e}   ", c.kappa);
    }
    println!("\n      → converged. κ is a deterministic loop integral, not a sampling artifact.");

    // ==================================================================
    rule("§1  THE SYMMETRY NULL TEST — why the OBVIOUS probe would have lied");
    // ==================================================================
    println!("A cube face is D4-symmetric. A mirror maps a face-centred contour to itself with");
    println!("REVERSED orientation and FLIPPED chirality (Δ → −Δ), so ∮sinΔ dl = −∮sinΔ dl = 0.");
    println!("\x1b[1mThe symmetric cones are therefore NULL TESTS: κ = 0 there is a theorem about the");
    println!("symmetry, not a finding about the physics.\x1b[0m Run only the obvious face-centred cone —");
    println!("the natural thing to do — and you measure zero and acquit the router.\n");
    println!("  MFD-8, θ = 0.9 rad, L19:\n");
    println!("  {:<36}{:>13}{:>12}{:>11}", "cone pole", "κ", "mean Δ°", "mean|Δ|°");
    println!("  {:-<36}{:->13}{:->12}{:->11}", "", "", "", "");
    for (n, p) in [
        ("face CENTRE   (D4 symmetric)", face_centre),
        ("cube EDGE-midpoint (symmetric)", edge_mid),
        ("cube CORNER   (symmetric)", corner),
        ("GENERIC — no symmetry with the cube", generic),
    ] {
        let c = contour_circulation(Router::Mfd8, &Cone { pole: p }, p, 0.9, L19, R, M);
        let hi = if c.kappa.abs() > 1e-5 { "\x1b[33m" } else { "" };
        println!("  {hi}{:<36}{:>13.2e}{:>12.3}{:>11.2}\x1b[0m", n, c.kappa, c.mean_defl_deg, c.mean_abs_defl_deg);
    }
    println!("\n  → the symmetric poles cancel to MACHINE ZERO while carrying a 5–7° pointwise bias.");
    println!("    Real terrain is not symmetric. The generic pole is the measurement.");

    // ==================================================================
    rule("§2  THE VERDICT — contour circulation, generic pole, every router");
    // ==================================================================
    println!("κ = the fraction of the transport running ALONG the contour instead of DOWN the");
    println!("slope. Exact answer: 0, at every θ, every router, every mesh, every level.");
    println!("`solenoidal` = |κ| / ⟨|sinΔ|⟩ — the share of the pointwise fan bias that SURVIVES the");
    println!("loop integral. The rest of the bias is convergent (the attractors) and irrotational.\n");
    println!("  {:<32}{:>7}{:>12}{:>10}{:>13}", "router", "θ", "κ", "mean|Δ|°", "solenoidal");
    println!("  {:-<32}{:->7}{:->12}{:->10}{:->13}", "", "", "", "", "");
    let mut worst: f64 = 0.0;
    for r in [Router::Mfd8, Router::Mfd8TrueDist, Router::Mfd4, Router::GradEdge] {
        for &th in &[0.3f64, 0.6, 0.9, 1.2, 1.5] {
            let c = contour_circulation(r, &cone_g, generic, th, L19, R, M);
            let sin_bias = c.mean_abs_defl_deg.to_radians().sin();
            let sol = if sin_bias > 1e-9 { c.kappa.abs() / sin_bias } else { 0.0 };
            if r == Router::Mfd8 {
                worst = worst.max(c.kappa.abs());
            }
            let hi = if c.kappa.abs() > 5e-3 { "\x1b[33m" } else { "" };
            println!(
                "  {hi}{:<32}{:>7.1}{:>12.2e}{:>10.2}{:>12.1}%\x1b[0m",
                if th == 0.3 { r.label() } else { String::new() },
                th,
                c.kappa,
                c.mean_abs_defl_deg,
                100.0 * sol
            );
        }
        println!();
    }
    println!("  \x1b[1mworst |κ| for the status-quo router: {:.2}%  of the transport circulates.\x1b[0m", 100.0 * worst);

    // ==================================================================
    rule("§3  DOES IT CONVERGE AWAY?  — the question that decides everything");
    // ==================================================================
    println!("The fan converges to the lattice sheared by the map's Jacobian, whose closed form");
    println!("contains no N. So if this circulation is REAL it must be LEVEL-INDEPENDENT; if it is");
    println!("the operator's own error it must FALL. The GradEdge column is the calibration: the");
    println!("SAME operator, SAME contour, SAME cells — and there the number DOES go to zero.\n");
    println!("  generic pole, θ = 0.3 rad (where |κ| is largest), {M} samples\n");
    println!("  {:>6}{:>12}{:>15}{:>16}{:>13}", "level", "cell (m)", "κ (MFD-8)", "κ (GradEdge)", "κ (C0)");
    println!("  {:->6}{:->12}{:->15}{:->16}{:->13}", "", "", "", "", "");
    for l in [5u32, 7, 9, 11, 13, 15, 17, 19, 21, 23] {
        let a = contour_circulation(Router::Mfd8, &cone_g, generic, 0.3, l, R, M);
        let b = contour_circulation(Router::GradEdge, &cone_g, generic, 0.3, l, R, M);
        let z = contour_circulation(Router::ExactControl, &cone_g, generic, 0.3, l, R, M);
        let star = if l == 19 { "  ← erosion runs here" } else { "" };
        println!(
            "  {:>6}{:>12.1}{:>15.4e}{:>16.2e}{:>13.1e}{star}",
            format!("L{l}"), curl::cell_m(l, R), a.kappa, b.kappa, z.kappa
        );
    }
    println!("\n  \x1b[1m→ MFD-8's κ is FLAT from L9 to L23 — 16 384× smaller cells, same circulation.");
    println!("    The operator's own error, measured on the same loop, falls ~5 orders. The");
    println!("    probe is demonstrably capable of returning zero. It does not return zero here.\x1b[0m");

    // ==================================================================
    rule("§4  THE MAP — spurious vorticity over a cube face");
    // ==================================================================
    println!("Plaquette circulation of E = d̂ − f̂ around each 2×2 loop, normalised to Ω·R/A");
    println!("(dimensionless and level-independent: spurious rotation per radian of arc). Zero for");
    println!("a perfect router by construction. Flow imposed by the generic-pole cone, so the face");
    println!("sees flow from every direction.\n");
    println!("  MFD-8 @ L19, one whole face:   \x1b[31m█▓▒ = +\x1b[0m (CCW)   \x1b[34m█▓▒ = −\x1b[0m (CW)   · ≈ 0\n");

    let n = 1i64 << L19;
    let nb = 21usize;
    let idx = |t: f64| ((((t + 1.0) * 0.5) * n as f64).floor() as i64).clamp(1, n - 3);
    let mut grid = vec![f64::NAN; nb * nb];
    for gy in 0..nb {
        for gx in 0..nb {
            let u = -1.0 + 2.0 * (gx as f64 + 0.5) / nb as f64;
            let v = -1.0 + 2.0 * (gy as f64 + 0.5) / nb as f64;
            grid[gy * nb + gx] = plaquette_curl(Router::Mfd8, &cone_g, L19, 0, idx(u), idx(v), R).unwrap_or(f64::NAN);
        }
    }
    let peak = grid.iter().filter(|x| x.is_finite()).fold(0.0f64, |m, &x| m.max(x.abs()));
    for gy in (0..nb).rev() {
        let v = -1.0 + 2.0 * (gy as f64 + 0.5) / nb as f64;
        print!("   v={v:+.2}  ");
        for gx in 0..nb {
            let t = (grid[gy * nb + gx] / peak.max(1e-30)).clamp(-1.0, 1.0);
            print!("{}", match t {
                t if t > 0.60 => "\x1b[31m█\x1b[0m",
                t if t > 0.30 => "\x1b[31m▓\x1b[0m",
                t if t > 0.08 => "\x1b[31m▒\x1b[0m",
                t if t > -0.08 => "·",
                t if t > -0.30 => "\x1b[34m▒\x1b[0m",
                t if t > -0.60 => "\x1b[34m▓\x1b[0m",
                _ => "\x1b[34m█\x1b[0m",
            });
            print!(" ");
        }
        println!();
    }
    println!("           {}", "─ ".repeat(nb));
    println!("           u=−1{}u=+1", " ".repeat(2 * nb - 9));
    println!("\n   peak |Ω·R/A| = {peak:.3}   (the face centre, where MFD is exact, is the '·' island)");

    let mut vals = Vec::new();
    let mm = 121i64;
    for gy in 0..mm {
        for gx in 0..mm {
            let u = -1.0 + 2.0 * (gx as f64 + 0.5) / mm as f64;
            let v = -1.0 + 2.0 * (gy as f64 + 0.5) / mm as f64;
            if let Some(w) = plaquette_curl(Router::Mfd8, &cone_g, L19, 0, idx(u), idx(v), R) {
                vals.push(w.abs());
            }
        }
    }
    vals.sort_by(f64::total_cmp);
    let q = |f: f64| vals[((vals.len() - 1) as f64 * f) as usize];
    println!("   census of |Ω·R/A|:  median {:.3}   90th {:.3}   worst {:.3}", q(0.5), q(0.9), q(1.0));
    print!("\n   level-independence of the LOCAL curl (mid-face cell):  ");
    for l in [5u32, 9, 13, 17, 19, 23] {
        let nn = 1i64 << l;
        print!("L{l}:{:>7.4}  ", plaquette_curl(Router::Mfd8, &cone_g, l, 0, (nn * 3) / 4, (nn * 3) / 4, R).unwrap_or(f64::NAN));
    }
    println!();

    // ==================================================================
    rule("§5  TIER B — the REAL routers on the REAL whole-sphere mesh");
    // ==================================================================
    println!("Everything above is analytic. This is the independent cross-check: the Euler-asserted");
    println!("whole-sphere mesh with combinatorial cross-face adjacency, running `grid_lab`'s ACTUAL");
    println!("routers — the ones that produced the 20.76% vs 5.17% catchment numbers — with each");
    println!("cell's outflow direction taken from its real routed weights.\n");
    let g = grids::cube_sphere(grids::CubeProj::Equiangular, 96, R);
    println!("   mesh: {} · {} cells · Euler = {} ✓\n", g.name, g.cells(), g.euler);
    let h: Vec<f64> = g.centers.iter().map(|&p| -R * geodesic(p, generic)).collect();
    println!("   {:<34}{:>10}{:>13}{:>11}", "router (grid_lab/flow.rs)", "θ", "κ", "mean|Δ|°");
    println!("   {:-<34}{:->10}{:->13}{:->11}", "", "", "", "");
    for r in [flow::Router::MooreMfd, flow::Router::MooreMfdTrueDist, flow::Router::EdgeMfd, flow::Router::GradEdgeFlux] {
        for &th in &[0.3f64, 0.9, 1.5] {
            let c = mesh_contour(&g, r, &h, generic, th, 4000);
            let hi = if c.0.abs() > 5e-3 { "\x1b[33m" } else { "" };
            println!("   {hi}{:<34}{:>10.1}{:>13.2e}{:>11.2}\x1b[0m", if th == 0.3 { r.label() } else { "" }, th, c.0, c.2);
        }
        println!();
    }
    let c = mesh_contour_exact(&g, generic, 0.3, 4000);
    println!("   {:<34}{:>10.1}{:>13.1e}{:>11.2}   \x1b[32mCONTROL\x1b[0m", "exact gradient", 0.3, c.0, c.2);
    println!("\n   → the mesh reproduces the analytic tier's sign and magnitude. Two independent");
    println!("     code paths, same answer.\n");
    println!("   \x1b[1m⚠ AND THE LINE THAT CHANGES WHAT TO BUILD:\x1b[0m gradient-projected edge flux — the");
    println!("   router the grid report proposed — is \x1b[1mNOT\x1b[0m curl-free either (κ = 4.5e-3, ~4.5×");
    println!("   better than MFD-8, not zero). Taking the DIRECTION from the gradient is not");
    println!("   enough, because the router then SPLITS the mass among discrete neighbour cells,");
    println!("   and the transport that results is Σwₖ·êₖ — \x1b[1ma fan again\x1b[0m. Nothing was constraining");
    println!("   that sum to point where the water is supposed to go. \x1b[1mSo constrain it.\x1b[0m → §8");

    // ==================================================================
    rule("§8  THE REMEDY — impose the physical claim as a CONSTRAINT on the split");
    // ==================================================================
    println!("The physical claim is \"the transport direction is the steepest-descent direction of φ\".");
    println!("Written on the actual outflow weights, that claim IS a first-moment condition:\n");
    println!("      \x1b[1mΣₖ wₖ · sin(βₖ − ψ) = 0\x1b[0m       βₖ = the neighbour's TRUE bearing");
    println!("                                     ψ  = the reconstructed gradient's azimuth\n");
    println!("*The tangential moment of the outflow must vanish.* MFD never imposed it; it just");
    println!("hoped 8 evenly-spaced nodes would deliver it for free, and on a sphere they do not.");
    println!("Impose it directly — minimum-relative-entropy tilt of MFD's OWN slope weights,");
    println!("wₖ ← wₖ·exp(−λ sin(βₖ−ψ)), λ by bisection. Weights stay strictly positive and sum to 1,");
    println!("so conservation is untouched. ~20 lines. No new stencil, no new grid, no new adjacency,");
    println!("no diagonals-are-45° premise. \x1b[2m(:by claude :status proposed — Joseph decides.)\x1b[0m\n");

    println!("\x1b[1mThe two defects are SEPARABLE, and that is the useful part.\x1b[0m The moment constraint");
    println!("fixes the DIRECTION (κ) and does nothing for the dispersion; edge-flux weighting fixes");
    println!("the DISPERSION (catchment magnitude) and does nothing for κ. The obvious move is to");
    println!("compose them. \x1b[1mI tried that. It does not work, and the table below shows why.\x1b[0m\n");

    let all = [
        flow::Router::MooreMfd,
        flow::Router::GradEdgeFlux,
        flow::Router::MomentMfd,
        flow::Router::MomentEdge,
        flow::Router::QuadMoment,
    ];
    println!("   {:<32}{:>8}{:>13}{:>11}", "router", "θ", "κ", "mean|Δ|°");
    println!("   {:-<32}{:->8}{:->13}{:->11}", "", "", "", "");
    for r in all {
        for &th in &[0.3f64, 0.9, 1.5] {
            let c = mesh_contour(&g, r, &h, generic, th, 4000);
            let hi = if r == flow::Router::QuadMoment { "\x1b[32m" } else if c.0.abs() > 5e-3 { "\x1b[33m" } else { "" };
            println!("   {hi}{:<32}{:>8.1}{:>13.2e}{:>11.2}\x1b[0m", if th == 0.3 { r.label() } else { "" }, th, c.0, c.2);
        }
        println!();
    }

    println!("   \x1b[1m⇒ THE MOMENT CONSTRAINT RESTORES THE IDENTITY EXACTLY.\x1b[0m κ falls ~4 orders, to the");
    println!("   operator's own floor, and the per-cell deflection goes to 0.02°. It does so on ANY");
    println!("   weight set with enough receivers, at any valence, on any grid.\n");
    println!("   \x1b[1mNow the honest half. Did it buy that by breaking something else?\x1b[0m\n");
    println!("   {:<32}{:>18}{:>13}{:>16}{:>15}", "router", "conservation", "cone err", "at the defects", "plume drift");
    println!("   {:-<32}{:->18}{:->13}{:->16}{:->15}", "", "", "", "", "");
    let frame = tangent(generic, [0.0, 0.0, 1.0]);
    for r in all {
        let ro = flow::routing(&g, r);
        let pl = flow::plume(&g, r, generic, frame, 0.30, 22.5, &[1.30]);
        let d = pl.drift_deg.last().copied().unwrap_or(f64::NAN);
        let km = R * 1.30f64.sin() * d.to_radians().abs() / 1000.0;
        println!(
            "   {:<32}{:>18.12}{:>12.2}%{:>15.2}%{:>12.0} km",
            r.label(), ro.conservation, 100.0 * ro.cone_err_mean, 100.0 * ro.cone_err_at_defects, km
        );
    }
    println!("\n   \x1b[1m⚠ CONSERVATION SURVIVES — exact to twelve figures, every router. ACCURACY DOES NOT");
    println!("   FOLLOW, AND BOTH OF MY ATTEMPTS TO COMPOSE THE TWO FIXES UNDER-PERFORMED:\x1b[0m\n");
    println!("   • \x1b[1mmoment + edge flux is worse than EITHER parent\x1b[0m on catchment error — 8.09%, against");
    println!("     plain edge flux's 4.68%. An edge router has only 2–4 receivers, so `Σw = 1` plus");
    println!("     `Σwₖsₖ = 0` nearly determines the weights on its own: it collapses toward D-∞ and");
    println!("     throws the slope-proportional magnitude away. Two correct ideas, composed, got");
    println!("     worse. And κ does not even reach the floor (1e-3 — the cells whose few receivers");
    println!("     fail to bracket ψ).");
    println!("   • \x1b[1mtrue-width quadrature reaches the κ floor and is still a mediocre router\x1b[0m — 19.59%");
    println!("     catchment error against edge flux's 4.68%. Fixing the direction did not fix the");
    println!("     magnitude, and nothing about the identity says it should.\n");
    println!("   \x1b[1m→ AND THE ROW-PAIR THAT MATTERS MOST IN THIS TABLE:\x1b[0m gradient-projected edge flux");
    println!("     carries \x1b[1m~900× the circulation\x1b[0m of moment-corrected MFD (4.5e-3 vs 5.0e-6) and yet");
    println!("     drifts \x1b[1mless than half as far\x1b[0m (53 km vs 121 km). \x1b[1mCirculation and drift are");
    println!("     DIFFERENT DEFECTS and they do not even rank the routers the same way.\x1b[0m Drift is");
    println!("     carried by the CONVERGENT (attractor) part of the deformation; κ is carried by the");
    println!("     SOLENOIDAL part. Kill one and the other is untouched. \x1b[1mThat is the assumption this");
    println!("     probe was built on, and it now has independent evidence: the fan probe and the curl");
    println!("     probe do not subsume each other, and neither alone is sufficient.\x1b[0m\n");
    println!("   \x1b[1mSo: the remedy for the IDENTITY is established, exact and cheap. The full kernel is");
    println!("   NOT designed, and this spike does not pretend to have designed it.\x1b[0m The moment");
    println!("   constraint is a projection to compose INTO whatever routing scheme is chosen — it is");
    println!("   not itself that scheme, and the scheme still has to win on accuracy separately.");
    println!("   \x1b[2m(:by claude :status proposed — the kernel choice is Joseph's.)\x1b[0m");

    // provenance gate
    println!("\n   \x1b[2mPROVENANCE GATE — this file is a COPY of grid_lab/flow.rs (that tree is another");
    println!("   agent's). Re-run its published §6 numbers at the report's own resolution (6·32²) to");
    println!("   prove the copy is the same code that produced them:\x1b[0m");
    let g32 = grids::cube_sphere(grids::CubeProj::Equiangular, 32, R);
    for (r, want) in [(flow::Router::MooreMfd, 20.76), (flow::Router::GradEdgeFlux, 5.17)] {
        let got = 100.0 * flow::routing(&g32, r).cone_err_mean;
        println!(
            "     {:<32} report {want:>5.2}%   here {got:>5.2}%   {}",
            r.label(),
            if (got - want).abs() < 0.02 { "\x1b[32mMATCH\x1b[0m" } else { "\x1b[31mDIVERGED\x1b[0m" }
        );
    }

    // ==================================================================
    rule("§6  REAL TERRAIN — no symmetry to hide behind, no designed contour");
    // ==================================================================
    println!("The cone is a designed test. Here the contours are TRACED — RK4 along t̂ = p̂ × f̂,");
    println!("which stays on the level set by construction — over the band-limited plane-wave");
    println!("terrain `grid_lab` scores every grid on (analytic ∇h, so C0 is still exactly zero).");
    println!("`h drift` is the gate: if the traced curve left the level set it is not a contour and");
    println!("nothing below would be an invariant.\n");

    let terr = Terrain::new(3000.0);
    println!("   {:<8}{:>9}{:>10}{:>12}{:>12}{:>12}{:>11}", "contour", "len km", "h drift", "κ MFD-8", "κ Mfd4", "κ GradEdge", "mean|Δ|°");
    println!("   {:-<8}{:->9}{:->10}{:->12}{:->12}{:->12}{:->11}", "", "", "", "", "", "", "");
    let mut acc: Vec<f64> = Vec::new();
    let mut found = 0;
    for s in 0..40 {
        let a = 0.9 * (s as f64 * 2.399963); // golden-angle scatter of seeds over the sphere
        let z = 1.0 - 2.0 * (s as f64 + 0.5) / 40.0;
        let rr = (1.0 - z * z).max(0.0).sqrt();
        let seed = unit([rr * a.cos(), rr * a.sin(), z]);
        let Some(pts) = trace_contour(&terr, seed, R, 4000.0, 6000) else { continue };
        let len_km = pts.windows(2).map(|w| geodesic(w[0], w[1]) * R).sum::<f64>() / 1000.0;
        if len_km < 300.0 {
            continue;
        }
        let h0 = terr.h(pts[0], R);
        let drift = pts.iter().map(|&p| (terr.h(p, R) - h0).abs()).fold(0.0f64, f64::max);
        let cm = polyline_circulation(Router::Mfd8, &terr, &pts, L19, R);
        let c4 = polyline_circulation(Router::Mfd4, &terr, &pts, L19, R);
        let cg = polyline_circulation(Router::GradEdge, &terr, &pts, L19, R);
        let c0t = polyline_circulation(Router::ExactControl, &terr, &pts, L19, R);
        assert!(c0t.kappa == 0.0, "C0 must be algebraically zero on a traced contour too");
        acc.push(cm.kappa.abs());
        found += 1;
        if found <= 10 {
            println!(
                "   {:<8}{:>9.0}{:>9.2}m{:>12.2e}{:>12.2e}{:>12.2e}{:>11.2}",
                format!("#{found}"), len_km, drift, cm.kappa, c4.kappa, cg.kappa, cm.mean_abs_defl_deg
            );
        }
    }
    acc.sort_by(f64::total_cmp);
    println!("\n   {} closed contours traced. \x1b[1m|κ| for MFD-8: median {:.2e}   90th {:.2e}   worst {:.2e}\x1b[0m",
        acc.len(), acc[acc.len() / 2], acc[(acc.len() * 9) / 10], acc[acc.len() - 1]);
    println!("   (the C0 control was re-asserted algebraically zero on every one of them)");

    // ==================================================================
    rule("§7  WHAT IT MEANS IN METRES — the head monodromy");
    // ==================================================================
    println!("κ is dimensionless, so anchor it. Around a closed contour of length L on ground of");
    println!("slope S, the router's IMPLIED hydraulic head fails to close by  Δφ = κ · S · L.");
    println!("Descend, go all the way round, arrive back where you started — and the water surface");
    println!("has dropped by Δφ. That is not a small error. It is not an error at all: it is a");
    println!("statement that no water surface exists.\n");
    println!("   {:<26}{:>10}{:>10}{:>16}", "loop", "S", "L (km)", "Δφ unclosed");
    println!("   {:-<26}{:->10}{:->10}{:->16}", "", "", "", "");
    for (name, s, l_km, k) in [
        ("hillslope contour", 0.10, 2.0, 0.02),
        ("small catchment", 0.05, 20.0, 0.02),
        ("large catchment", 0.02, 100.0, 0.02),
        ("— at the median |κ| ", 0.05, 20.0, 4e-3),
    ] {
        println!("   {:<26}{:>10.2}{:>10.0}{:>13.2} m", name, s, l_km, k * s * l_km * 1000.0);
    }
    println!("\n   (κ = 2e-2 is the worst measured contour; 4e-3 the median. S and L are stated,");
    println!("    not measured — this table converts, it does not discover.)");

    println!("\n");
}

// ---------------------------------------------------------------------------
// Tier B: circulation over the real Mesh, using flow.rs's real routers.
// ---------------------------------------------------------------------------

/// Effective unit outflow direction at mesh cell `i` from the router's REAL routed weights:
/// `d̂ = normalize(Σ wₖ · bearing(i→k))` — where the water actually goes.
fn mesh_dir(g: &Mesh, r: flow::Router, h: &[f64], i: usize) -> Option<V3> {
    let w = flow::weights(g, r, h, i);
    if w.is_empty() {
        return None;
    }
    let c = g.centers[i];
    let mut a = [0.0f64; 3];
    for (j, x) in w {
        a = add(a, scale(tangent(c, g.centers[j]), x));
    }
    if norm(a) < 1e-12 {
        return None;
    }
    Some(unit(sub(a, scale(c, dot(c, a)))))
}

fn mesh_contour(g: &Mesh, r: flow::Router, h: &[f64], pole: V3, theta: f64, m: usize) -> (f64, f64, f64) {
    mesh_inner(g, pole, theta, m, |i| mesh_dir(g, r, h, i))
}
fn mesh_contour_exact(g: &Mesh, pole: V3, theta: f64, m: usize) -> (f64, f64, f64) {
    mesh_inner(g, pole, theta, m, |i| Some(scale(tangent(g.centers[i], pole), -1.0)))
}

fn mesh_inner(g: &Mesh, pole: V3, theta: f64, m: usize, dir: impl Fn(usize) -> Option<V3>) -> (f64, f64, f64) {
    let e0 = tangent(pole, if dot(pole, [0.0, 0.0, 1.0]).abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] });
    let e1 = cross(pole, e0);
    let (mut num, mut len, mut sabs, mut ssgn, mut n) = (0.0, 0.0, 0.0, 0.0, 0usize);
    for k in 0..m {
        let psi = 2.0 * std::f64::consts::PI * (k as f64 + 0.5) / m as f64;
        let p = unit(add(scale(pole, theta.cos()), scale(add(scale(e0, psi.cos()), scale(e1, psi.sin())), theta.sin())));
        let that = unit(cross(p, scale(tangent(p, pole), -1.0)));
        let dl = R * theta.sin() * (2.0 * std::f64::consts::PI / m as f64);
        let i = (0..g.cells()).min_by(|&a, &b| geodesic(g.centers[a], p).total_cmp(&geodesic(g.centers[b], p))).unwrap();
        let c = g.centers[i];
        let fdir = scale(tangent(c, pole), -1.0); // exact cone descent
        let Some(d) = dir(i) else { continue };
        let e = sub(d, fdir);
        let e = sub(e, scale(p, dot(p, e)));
        num += dot(e, that) * dl;
        len += dl;
        let dfl = fan::wrap180(dot(d, cross(c, fdir)).atan2(dot(d, fdir)).to_degrees());
        sabs += dfl.abs();
        ssgn += dfl;
        n += 1;
    }
    (num / len, ssgn / n as f64, sabs / n as f64)
}
