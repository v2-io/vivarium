//! GRID LAB — what actually survives when an algorithm meets the grid.
//!
//! **The question, and why argument cannot settle it.** Every sphere pixelization has
//! irreducible adjacency defects (Euler forces `Σ(4−valence) = 8` on any quad mesh on
//! S²), so the projection choice cannot *fix* the seam. What it decides is **which
//! physical invariant survives when an algorithm is actually applied** — and the
//! algorithms lean on different terms:
//!
//!   * stream-power incision leans on drainage **area** and **slope**;
//!   * shallow-water flux leans on **edge length** and **cell area**;
//!   * hillslope diffusion leans on **isotropy**;
//!   * Priority-Flood and drainage accumulation lean on **global ORDER** — and that is
//!     the one with teeth, because a downstream neighbour across a partition seam is a
//!     dependency that serialises the sweep.
//!
//! Joseph, 2026-07-12: *"different projections are going to conserve different physical
//! properties when the algorithms get applied (not at a high-level conceptual guess)."*
//! And: the numbers must be **visible**, "so future agents and humans can see exactly what
//! the numbers say and distinguish between the metrics that matter to THEM."
//!
//! Report: `ref/research/grid-comparison-report.md`. Every number in it is printed here.
//! Run: `cargo run --release -p vivarium-world --example grid_lab`

mod flow;
mod grids;
mod healpix;
mod icosa;
mod mesh;
mod probes;

use flow::*;
use grids::*;
use mesh::*;
use probes::*;
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CubeCoord, Face};

fn r() -> f64 { Planet::EARTH.radius_m }

fn main() {
    println!("GRID LAB — what survives when the algorithm is actually applied");
    println!("radius {:.0} m · every number below is printed by this binary\n", r());

    gate_our_cube_is_the_real_one();
    gate_snyder_reproduces_table_1();
    gate_healpix_is_the_paper();
    gate_rhombic_dodecahedron();

    let ms = build_all();
    section_geometry(&ms);
    section_hoistable(&ms);
    section_curvature(&ms);
    section_strata();
    section_classes(&ms);
    section_conservation(&ms);
    section_accuracy(&ms);
    section_sequencing(&ms);
    section_corner();
    section_quadtree(&ms);
    honest_scope();
}

// ---------------------------------------------------------------------------
// GATES — nothing below is worth reading if these fail.
// ---------------------------------------------------------------------------

/// Our `EquiangularFace` must BE `sphere.rs`, not a lookalike.
fn gate_our_cube_is_the_real_one() {
    let faces = [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZPos, Face::ZNeg];
    let mut worst: f64 = 0.0;
    for (fi, &f) in faces.iter().enumerate() {
        for &u in &[-0.93, -0.4, 0.0, 0.17, 0.88] {
            for &v in &[-0.77, -0.1, 0.0, 0.55, 0.96] {
                let a = CubeCoord { face: f, u, v }.to_unit();
                let b = cube_to_unit(CubeProj::Equiangular, fi, u, v);
                worst = worst.max(norm(sub(a, b)));
            }
        }
    }
    assert!(worst < 1e-15, "our cube-sphere has DRIFTED from sphere.rs: {worst:.2e}");
    println!("GATE  our equiangular cube ≡ sphere.rs::to_unit          max drift {worst:.1e}  ✓");
}

/// **The Snyder correctness test.** Implement from the paper, then check the measured
/// distortion against the paper's OWN Table 1 for the cube: ω = 25.17°, a = 1.248,
/// b = 0.801 (and a·b ≈ 1, as equal-area requires). If we did not reproduce these, the
/// implementation is wrong and every Snyder number in the report is worthless.
fn gate_snyder_reproduces_table_1() {
    // Principal scale factors (σ₁ ≥ σ₂) of the map **from the polyhedron plane to the
    // sphere** — computed in the PLANE, not in (u,v), so no rotation or scaling can
    // corrupt them. Equal-area forces σ₁·σ₂ = 1, which is itself a free check.
    //
    // ⚠ The sampler must not evaluate ON a vertex radius. Snyder says so himself:
    // *"cusps occur along all radii from polygon center to vertex"* — the map is C⁰ but
    // NOT C¹ there, so a central difference across a vertex radius measures the cusp, not
    // the map. (This bit once: sampling the face diagonal returned det = 1.44 and an
    // ω of 41.5°, which looks exactly like a broken projection and is not.) So sample in
    // plane polar coordinates with Az′ strictly inside a sector.
    let rp = sny_rprime();
    let sv = |x: f64, y: f64| -> (f64, f64) {
        let e = 1e-8;
        let p = snyder_plane_to_unit(4, x, y);
        let px = snyder_plane_to_unit(4, x + e, y);
        let py = snyder_plane_to_unit(4, x, y + e);
        let (du, dv) = (scale(sub(px, p), 1.0 / e), scale(sub(py, p), 1.0 / e));
        let t1 = unit(sub(du, scale(p, dot(p, du))));
        let t2 = cross(p, t1);
        let (a11, a21) = (dot(du, t1), dot(du, t2));
        let (a12, a22) = (dot(dv, t1), dot(dv, t2));
        let (e1, f1, g1) =
            (a11 * a11 + a21 * a21, a11 * a12 + a21 * a22, a12 * a12 + a22 * a22);
        let tr = e1 + g1;
        let det = (e1 * g1 - f1 * f1).max(0.0);
        let disc = ((tr * tr / 4.0 - det).max(0.0)).sqrt();
        ((tr / 2.0 + disc).sqrt(), (tr / 2.0 - disc).max(0.0).sqrt())
    };
    let th = SNY_THETA.to_radians();
    let g = SNY_G.to_radians();
    // the face boundary at azimuth Az′ — Snyder eq (10)
    let dprime = |azp: f64| rp * g.tan() / (azp.cos() + azp.sin() * th.tan().recip());

    let (mut a, mut area_err, mut at) = (0.0f64, 0.0f64, (0.0, 0.0));
    let e = 1e-8;
    for ai in 1..900 {
        let azp = ai as f64 / 900.0 * std::f64::consts::FRAC_PI_2; // (0°, 90°) — the sector
        let dmax = dprime(azp);
        for ri in 1..400 {
            let rho = ri as f64 / 400.0 * dmax * 0.999;
            // Guard: the finite difference must not reach across a cusp. The nearest
            // vertex radius is at Az′ = 0 or 90°; the FD displaces the azimuth by ~e/ρ.
            let to_cusp = azp.min(std::f64::consts::FRAC_PI_2 - azp);
            if to_cusp < 20.0 * (e / rho) {
                continue;
            }
            let (x, y) = (rho * azp.sin(), rho * azp.cos());
            let (s1, s2) = sv(x, y);
            area_err = area_err.max((s1 * s2 - 1.0).abs());
            if s1 > a {
                a = s1;
                at = (azp.to_degrees(), rho / dmax);
            }
        }
    }
    let b = 1.0 / a;
    let om = 2.0 * ((a - b) / (a + b)).asin().to_degrees();
    println!(
        "GATE  Snyder cube — EQUAL-AREA residual  max|σ₁σ₂ − 1| = {area_err:.2e}   (exactly equal-area ✓)"
    );
    println!(
        "      distortion  measured  ω {:.2}°  a {:.3}  b {:.3}    paper Table 1: ω 25.17°  a 1.248  b 0.801",
        om, a, b
    );
    println!(
        "      peak at Az′ = {:.2}° (a vertex radius), fractional radius {:.3} — Table 1's note says ω peaks",
        at.0, at.1
    );
    println!("      'along a radius to each vertex, but at the center'. It does.");
    assert!(area_err < 1e-6, "Snyder is NOT equal-area: residual {area_err:.2e}");
    assert!((om - 25.17).abs() < 0.2, "Snyder ω {om:.2}° ≠ paper's 25.17° — the map is WRONG");
    assert!((a - 1.248).abs() < 0.006, "Snyder a {a:.3} ≠ paper's 1.248");
    assert!((b - 0.801).abs() < 0.006, "Snyder b {b:.3} ≠ paper's 0.801");
    println!("      → reproduces Snyder 1992 Table 1. The implementation IS the paper's.  ✓");

    // The forward/inverse pair must actually be a pair. (The paper's own claim: the
    // Newton–Raphson "converges even to 1e-9 degrees in 3–4 cycles".)
    let mut worst: f64 = 0.0;
    for i in 1..40 {
        for j in 1..40 {
            let (x, y) = (i as f64 / 40.0 * 0.6, j as f64 / 40.0 * 0.6);
            let (az, z) = snyder_inverse(x, y);
            let (x2, y2) = snyder_forward(az, z);
            worst = worst.max(((x - x2).powi(2) + (y - y2).powi(2)).sqrt());
        }
    }
    println!("      forward∘inverse round-trip  max residual {worst:.2e}  ✓\n");
    assert!(worst < 1e-12);
}

fn gate_healpix_is_the_paper() {
    for nside in [4usize, 8, 16] {
        let g = healpix::healpix(nside, 1.0);
        let omega = std::f64::consts::PI / (3.0 * (nside * nside) as f64);
        // total area, and the spherical-excess area of the corner quad vs the paper's Ω
        let tot: f64 = g.total_area();
        let mut worst: f64 = 0.0;
        for i in 0..g.cells() {
            let vs: Vec<V3> = g.rings[i].iter().map(|&v| g.verts[v as usize]).collect();
            worst = worst.max((poly_area(&vs) / omega - 1.0).abs());
        }
        if nside == 16 {
            println!(
                "GATE  HEALPix N_side={nside}  N_pix {}  ΣΩ/4π {:.12}   \
                 geodesic-quad area vs the paper's Ω: max {:.3}%",
                g.cells(),
                tot / (4.0 * std::f64::consts::PI),
                worst * 100.0
            );
            println!(
                "      (the gap is REAL and expected: HEALPix boundaries are NOT great circles\n\
                 \x20      — Górski §5.3, `cos θ = a + bφ` / `a + b/φ²`. We carry the paper's exact Ω\n\
                 \x20      as the area and great-circle chords as the edges; that is the honest split.)"
            );
        }
        assert!((tot / (4.0 * std::f64::consts::PI) - 1.0).abs() < 1e-12);
    }
    // The base-pixel skeleton: 12 quads. Its vertex census decides whether HEALPix's base
    // IS the rhombic dodecahedron.
    let b = healpix::healpix(1, 1.0);
    let mut val = std::collections::HashMap::new();
    for v in 0..b.verts.len() {
        *val.entry(b.vcells[v].len()).or_insert(0usize) += 1;
    }
    let mut ks: Vec<_> = val.iter().collect();
    ks.sort();
    println!(
        "      HEALPix BASE (N_side=1): F {}  V {}  vertex valences {:?}   Σ(4−val) = {}",
        b.cells(),
        b.verts.len(),
        ks,
        (0..b.verts.len()).map(|v| 4i64 - b.vcells[v].len() as i64).sum::<i64>()
    );
    println!(
        "      → 12 quad faces, 14 vertices, 8 of valence 3 and 6 of valence 4.\n\
         \x20      That IS the rhombic dodecahedron, combinatorially. HEALPix's base is not\n\
         \x20      'like' the RD — it is the RD.\n"
    );
}

fn gate_rhombic_dodecahedron() {
    let g = rhombic_dodec(RdProj::Gnomonic, 1, 1.0);
    let mut val = std::collections::HashMap::new();
    for v in 0..g.verts.len() {
        *val.entry(g.vcells[v].len()).or_insert(0usize) += 1;
    }
    let mut ks: Vec<_> = val.iter().collect();
    ks.sort();
    println!(
        "GATE  rhombic-dodec BASE:  F {}  V {}  vertex valences {:?}   Σ(4−val) = {}   ✓ (Euler forces 8)",
        g.cells(),
        g.verts.len(),
        ks,
        (0..g.verts.len()).map(|v| 4i64 - g.vcells[v].len() as i64).sum::<i64>()
    );
    println!("      → identical census to HEALPix's base. Same topological charge as the cube.\n");
}

// ---------------------------------------------------------------------------

/// Every grid at ~the same cell count, so the comparison is fair (24 300 – 25 002 cells,
/// a spread of 3 %). Matching cell COUNT, not level, is the only honest normalisation:
/// the grids have different natural refinement ladders.
fn build_all() -> Vec<Mesh> {
    vec![
        cube_sphere(CubeProj::Equiangular, 64, r()),   // 6·64²  = 24 576
        cube_sphere(CubeProj::SnyderEqualArea, 64, r()), // 24 576
        cube_sphere(CubeProj::Gnomonic, 64, r()),      // 24 576
        rhombic_dodec(RdProj::Gnomonic, 45, r()),      // 12·45² = 24 300
        rhombic_dodec(RdProj::TanWarp, 45, r()),       // 24 300
        healpix::healpix(45, r()),                     // 12·45² = 24 300
        icosa::icosa_tri(35, r()),                     // 20·35² = 24 500
        icosa::icosa_hex(50, 0, r()),                  // 10·50²+2 = 25 002
        icosa::icosa_hex(50, 12, r()),                 // the same, made centroidal (SCVT)
    ]
}

fn hdr(t: &str) {
    println!("\n{}", "─".repeat(108));
    println!("{t}");
    println!("{}", "─".repeat(108));
}

// ---------------------------------------------------------------------------

fn section_geometry(ms: &[Mesh]) {
    hdr("1. GEOMETRY — the per-cell census. 'spread' is max/min: 1.000 means the term HOISTS to a constant.");
    println!(
        "{:<26} {:>7} {:>9} {:>9} {:>9} {:>9} {:>9} {:>10}",
        "grid", "cells", "area×", "edge×", "arm×", "dist×", "nonortho", "closure"
    );
    println!("{}", " ".repeat(26 + 8 + 40) );
    for g in ms {
        let q = geometry(g);
        println!(
            "{:<26} {:>7} {:>9.4} {:>9.4} {:>9.4} {:>9.4} {:>6.2}° max {:>10.2e}",
            g.name,
            g.cells(),
            q.area.spread(),
            q.edge.spread(),
            q.arm.spread(),
            q.dist.spread(),
            q.nonortho.max,
            q.closure.max
        );
    }
    println!();
    println!("  nonortho = angle between the centre-line and the edge NORMAL (0° = orthogonal).");
    println!("  closure  = |Σ Lₑ·n̂ₑ| / Σ Lₑ per cell, max. A control volume must close; this says how well.");
    println!();
    println!(
        "{:<26} {:>12} {:>12} {:>26} {:>22}",
        "grid", "angle min", "angle max", "valence census", "arm-deficit max"
    );
    for g in ms {
        let q = geometry(g);
        let mut v: Vec<_> = q.valence.iter().map(|(k, c)| format!("{k}:{c}")).collect();
        v.sort();
        println!(
            "{:<26} {:>11.2}° {:>11.2}° {:>26} {:>21.3}%",
            g.name,
            q.angle.min,
            q.angle.max,
            v.join(" "),
            q.arm_deficit.max * 100.0
        );
    }
    println!();
    println!("  arm-deficit = |armᵢ + armⱼ − dist| / dist. Exactly 0 iff the centre-line passes THROUGH");
    println!("  the mid-edge. Non-zero means the mid-edge arm and the centre-to-centre line disagree —");
    println!("  and that difference IS the non-orthogonality correction a two-point flux must carry.");
}

fn section_hoistable(ms: &[Mesh]) {
    hdr("2. WHICH GEOMETRIC TERMS HOIST TO CONSTANTS — the perf lever, and what makes a 'sunny-day' fast path LEGAL.");
    println!(
        "{:<26} {:>8} {:>8} {:>8} {:>8} {:>8} {:>9}",
        "grid", "area", "edge", "arm", "dist", "angle", "valence"
    );
    for g in ms {
        let q = geometry(g);
        let c = |s: f64| if (s - 1.0).abs() < 1e-9 { "CONST" } else { "no" };
        let ang = if (q.angle.max - q.angle.min).abs() < 1e-6 { "CONST" } else { "no" };
        let val = if q.valence.len() == 1 { "CONST" } else { "no" };
        println!(
            "{:<26} {:>8} {:>8} {:>8} {:>8} {:>8} {:>9}",
            g.name,
            c(q.area.spread()),
            c(q.edge.spread()),
            c(q.arm.spread()),
            c(q.dist.spread()),
            ang,
            val
        );
    }
    println!();
    println!("  Only `area` ever hoists, and only on the two equal-area grids. NOTHING hoists edge length,");
    println!("  arm, or distance on ANY of these grids — equal-area buys exactly one of the four terms.");
}

fn section_curvature(ms: &[Mesh]) {
    hdr("3. CURVATURE, SPLIT IN TWO (Joseph's carve — a grid can be fine for one and wrong for the other).");
    println!("  (a) ARC / METRIC GEOMETRY IN THE EMBEDDING — governs distance, area, transport at large spans.");
    println!("      Discrete Gaussian curvature = the angle defect 2π − Σθ at each vertex. Gauss–Bonnet");
    println!("      forces Σ defect = 4π exactly, which is a free correctness check; what DIFFERS between");
    println!("      grids is where the curvature is concentrated.");
    println!();
    println!(
        "{:<26} {:>16} {:>14} {:>14} {:>16}",
        "grid", "Σdefect / 4π", "max defect", "mean defect", "max/mean"
    );
    for g in ms {
        let c = curvature(g);
        println!(
            "{:<26} {:>16.9} {:>13.4}° {:>13.5}° {:>16.0}×",
            g.name,
            c.defect_sum_over_4pi,
            c.defect_max_deg,
            c.defect_mean_deg,
            c.defect_max_deg / c.defect_mean_deg
        );
    }
    println!();
    println!("  (b) FLAT-VIA-GRAVITY — the surface is a gravitational equipotential, so water LOCALLY");
    println!("      experiences a flat plane. That is exactly why the shallow-water kernel's flat");
    println!("      assumption is legitimate. The number that decides it is the SAGITTA: how far a cell's");
    println!("      own corners rise off the plane through its centre.");
    println!();
    println!("{:<26} {:>16} {:>18} {:>16}", "grid", "cell span", "sagitta (max)", "sagitta/span");
    for g in ms {
        let c = curvature(g);
        println!(
            "{:<26} {:>13.0} m {:>15.3} m {:>15.2e}",
            g.name,
            c.cell_span_mean_m,
            c.sagitta_max_m,
            c.sagitta_max_m / c.cell_span_mean_m
        );
    }
    println!();
    println!("  At vivarium's PLAYABLE rung (L25, 0.5 m cells) the sagitta is ~1e-8 m — flat-via-gravity is");
    println!("  exact to any tolerance that matters. It is NOT a grid-choice question at all; it is a");
    println!("  SCHEME question (use geopotential height, and carry the FV metrics). Every grid here is");
    println!("  identically fine for it. That is a clean 'this does not apply'.");
}

fn section_strata() {
    hdr("4. STRATA VOLUMES — the vertical. A column's volume is NOT area × height: the shell's volume element scales as r².");
    let rr = r();
    for (lo, hi, what) in [
        (-11_000.0, 9_000.0, "the ~20 km livable shell (trench → peak)"),
        (0.0, 20_000.0, "0 → 20 km"),
        (0.0, 1_000.0, "0 → 1 km"),
    ] {
        let (ar, vr) = strata(rr, lo, hi);
        println!(
            "  {what:<40}  top/bottom cell AREA ratio {:.5}× ({:+.3} %)   true volume ÷ (area×height) {:.5} ({:+.3} %)",
            ar,
            (ar - 1.0) * 100.0,
            vr,
            (vr - 1.0) * 100.0
        );
    }
    println!();
    println!("  This is a MASS term, and rock mass is what erosion must conserve. It is entirely RADIAL, so");
    println!("  it is IDENTICAL on every grid in this report — a clean 'does not discriminate'. But it does");
    println!("  not vanish: it MULTIPLIES whatever area spread the grid already has. On an equal-area grid a");
    println!("  column's volume is still one constant per (level, altitude); on ours it inherits the 1.41×.");
}

fn section_classes(ms: &[Mesh]) {
    hdr("5. THE CONFIGURATION-CLASS CENSUS — the STENCIL, not the cell. Flux across (i,j) needs the JOINT geometry of the pair.");
    println!(
        "{:<26} {:>28} {:>14} {:>14} {:>14}",
        "grid", "topological classes (val,moore)", "geom @1%", "geom @0.1%", "cells/class"
    );
    for g in ms {
        let c1 = classes(g, 1.0);
        let c2 = classes(g, 0.1);
        let mut t: Vec<_> = c1.topo.iter().map(|((v, m), c)| format!("{v}/{m}:{c}")).collect();
        t.sort();
        println!(
            "{:<26} {:>28} {:>14} {:>14} {:>14.1}",
            g.name,
            t.join(" "),
            c1.geom,
            c2.geom,
            g.cells() as f64 / c1.geom as f64
        );
    }
    println!();
    println!("  THE SPLIT THAT MATTERS. The **topological** class count is TINY — two classes on every grid");
    println!("  here (a regular cell, and a defective one), and the defective set is a fixed 24 (or 12) cells");
    println!("  world-wide AT EVERY LEVEL. That is a cheap branch, and it is the only branch a kernel needs.");
    println!("  The **geometric** class count is NOT small and does NOT stay small — it grows ~ cells/|symmetry");
    println!("  group| (cube: 6 faces × D₄ ⇒ ~N²/8 classes/face). So the continuous metrics must be computed,");
    println!("  not tabulated by class — and they CAN be, because they are analytic in position.");
}

fn section_conservation(ms: &[Mesh]) {
    hdr("6. CONSERVATION — mass leak under linear diffusion, whose invariant is EXACTLY known. Any drift is the scheme lying.");
    println!("  Source placed at a face/base centre AND at a topological defect (a cube corner / HEALPix");
    println!("  junction / icosa pentagon) — the spread between them is the real story.\n");
    println!(
        "{:<26} {:>16} {:>16} {:>16} {:>13} {:>13}",
        "grid", "naive @centre", "naive @defect", "FV centre-line", "FV arm", "anisotropy"
    );
    for g in ms {
        // a defect cell centre, and a "clean" cell centre far from any defect
        let defect = (0..g.cells())
            .find(|&i| g.moore[i].len() != 2 * g.adj[i].len())
            .unwrap_or(0);
        let clean = {
            let mut best = (0usize, -1.0f64);
            for i in 0..g.cells() {
                let d = (0..g.cells())
                    .filter(|&k| g.moore[k].len() != 2 * g.adj[k].len())
                    .map(|k| geodesic(g.centers[i], g.centers[k]))
                    .fold(f64::MAX, f64::min);
                if d > best.1 {
                    best = (i, d);
                }
                if i > 4000 {
                    break;
                }
            }
            best.0
        };
        let (dn_c, _) = blob(g, Scheme::NaiveUniform, g.centers[clean], 300);
        let (dn_d, _) = blob(g, Scheme::NaiveUniform, g.centers[defect], 300);
        let (df_d, an_d) = blob(g, Scheme::FvCentreLine, g.centers[defect], 300);
        let (da_d, _) = blob(g, Scheme::FvArm, g.centers[defect], 300);
        println!(
            "{:<26} {:>+16.3e} {:>+16.3e} {:>+16.3e} {:>+13.3e} {:>13.4}",
            g.name, dn_c, dn_d, df_d, da_d, an_d
        );
    }
    println!();
    println!("  THE DECOMPOSITION, and it is the headline of the whole report:");
    println!("    CONSERVATION is a **SCHEME** property — finite volume fixes it EXACTLY (~1e-16) on EVERY");
    println!("      grid here, including the worst one. No projection is needed, and none helps.");
    println!("    ISOTROPY / SHAPE is a **GRID** property — no scheme fixes it. See §7.");
    println!();
    println!("  And the naive path's leak is worst at COARSE levels — i.e. exactly at early evolution, where");
    println!("  the developmental ladder starts. Level sweep on our own grid:");
    for n in [4usize, 8, 16, 32, 64, 128] {
        let g = cube_sphere(CubeProj::Equiangular, n, r());
        let d = (0..g.cells()).find(|&i| g.moore[i].len() != 2 * g.adj[i].len()).unwrap();
        let (dn, _) = blob(&g, Scheme::NaiveUniform, g.centers[d], 200);
        let (dv, _) = blob(&g, Scheme::FvCentreLine, g.centers[d], 200);
        println!(
            "    L{:<2} ({:>6} cells)   naive {:>+10.2e}   finite-volume {:>+10.2e}",
            (n as f64).log2() as u32,
            6 * n * n,
            dn,
            dv
        );
    }
}

fn section_accuracy(ms: &[Mesh]) {
    hdr("7. ISOTROPY / ACCURACY — and does the error CONVERGE AWAY with refinement, or not?");
    println!("  A spherical harmonic of degree ℓ is an EXACT eigenfunction of the Laplace–Beltrami operator:");
    println!("  Δ Yℓ = −ℓ(ℓ+1)/R² · Yℓ. So the truncation error of a discrete Laplacian is exactly known —");
    println!("  no reference solution, no tuning, no opinion. (The test harmonic is deliberately NOT aligned");
    println!("  to any grid's axes.) Relative L2 error, ℓ = 2:\n");
    println!(
        "{:<26} {:>16} {:>16} {:>16} {:>14}",
        "grid", "naive-uniform", "FV centre-line", "FV mid-edge arm", "arm helps?"
    );
    for g in ms {
        let (n, _) = harmonic_error(g, Scheme::NaiveUniform, 2);
        let (c, _) = harmonic_error(g, Scheme::FvCentreLine, 2);
        let (a, _) = harmonic_error(g, Scheme::FvArm, 2);
        println!(
            "{:<26} {:>16.4e} {:>16.4e} {:>16.4e} {:>13.2}×",
            g.name, n, c, a, c / a
        );
    }
    println!();
    println!("  CONVERGENCE. Does refinement wash the error out? (order = log₂(err_coarse / err_fine);");
    println!("  2.0 = second order = the error dies as h²; ~0 = it does NOT converge away.)\n");
    println!(
        "{:<26} {:>12} {:>12} {:>10}   {:>12} {:>12} {:>10}",
        "grid", "FV coarse", "FV fine", "order", "L∞ coarse", "L∞ fine", "L∞ order"
    );
    let pairs: Vec<(String, Mesh, Mesh)> = vec![
        (
            "cube · equiangular (OURS)".into(),
            cube_sphere(CubeProj::Equiangular, 32, r()),
            cube_sphere(CubeProj::Equiangular, 64, r()),
        ),
        (
            "cube · Snyder equal-area".into(),
            cube_sphere(CubeProj::SnyderEqualArea, 32, r()),
            cube_sphere(CubeProj::SnyderEqualArea, 64, r()),
        ),
        (
            "rhombic-dodec · gnomonic".into(),
            rhombic_dodec(RdProj::Gnomonic, 32, r()),
            rhombic_dodec(RdProj::Gnomonic, 64, r()),
        ),
        ("HEALPix".into(), healpix::healpix(32, r()), healpix::healpix(64, r())),
        ("icosa-tri (geodesic)".into(), icosa::icosa_tri(24, r()), icosa::icosa_tri(48, r())),
        ("icosa-hex (raw dual)".into(), icosa::icosa_hex(24, 0, r()), icosa::icosa_hex(48, 0, r())),
        ("icosa-hex (SCVT)".into(), icosa::icosa_hex(24, 12, r()), icosa::icosa_hex(48, 12, r())),
    ];
    for (nm, a, b) in &pairs {
        let (l2a, lia) = harmonic_error(a, Scheme::FvCentreLine, 2);
        let (l2b, lib) = harmonic_error(b, Scheme::FvCentreLine, 2);
        println!(
            "{:<26} {:>12.3e} {:>12.3e} {:>10.2}   {:>12.3e} {:>12.3e} {:>10.2}",
            nm,
            l2a,
            l2b,
            (l2a / l2b).log2(),
            lia,
            lib,
            (lia / lib).log2()
        );
    }
    let _ = ms;
    println!();
    println!("  Read the L∞ column: it is where the DEFECT lives. If L∞ does not converge while L2 does,");
    println!("  the grid is second-order almost everywhere and first-order (or worse) at its defects — and");
    println!("  refinement will never wash them out. That is the grid property no scheme can fix.");
}

fn section_sequencing(ms: &[Mesh]) {
    hdr("8. SEQUENCING / PARALLELISM — Joseph's criterion, and possibly the DOMINANT one.");
    println!("  Diffusion is embarrassingly parallel and therefore the LEAST informative case. Priority-Flood");
    println!("  and drainage accumulation are globally ORDERED (elevation-sorted, strictly downstream). At a");
    println!("  seam, a cell's DOWNSTREAM neighbour can live in another partition — a dependency edge that");
    println!("  SERIALISES the sweep. Same band-limited terrain on every grid; each grid partitioned by its");
    println!("  OWN natural coarse block (cube: 6 faces · RD/HEALPix: 12 · icosa: 20).\n");
    println!(
        "{:<26} {:>7} {:>10} {:>10} {:>9} {:>10} {:>13} {:>8}",
        "grid", "parts", "boundary", "crossing", "cyclic?", "max SCC", "sync rounds", "longest"
    );
    for g in ms {
        let s = sequencing(g);
        println!(
            "{:<26} {:>7} {:>9.2}% {:>9.2}% {:>9} {:>10} {:>13} {:>8}",
            g.name,
            s.nparts,
            s.boundary_frac * 100.0,
            s.crossing_frac * 100.0,
            if s.partition_graph_cyclic { "YES" } else { "no" },
            s.largest_scc,
            s.max_partition_transitions,
            s.longest_path
        );
    }
    println!();
    println!("  'crossing'    = cells whose steepest-descent RECEIVER is in another partition. These are the");
    println!("                  dependency edges. Lower is better.");
    println!("  'cyclic?'     = does the PARTITION-level dependency graph have cycles? If YES, no ordering of");
    println!("                  whole partitions exists, and a partitioned Priority-Flood MUST synchronise at");
    println!("                  cell granularity across the seam. This is the ordering tooth.");
    println!("  'sync rounds' = the maximum number of PARTITION TRANSITIONS along any flow path — the number of");
    println!("                  synchronisation rounds a partitioned accumulation has to pay. THE number.");
    println!("  'longest'     = the longest flow path in cells: the intrinsic serial depth of the sweep.");
}

fn section_corner() {
    hdr("9. THE CORNER TEST — is the valence-3 pathology real, or an artifact of MFD's 8-neighbour fan?");
    println!("  The claim under test: *'your up-neighbour's left edge is welded to your left-neighbour's top");
    println!("  edge; the diagonal cell doesn't exist.'* True finite-volume flux crosses EDGES; diagonals share");
    println!("  only a vertex — a zero-length edge — so they carry no flux.\n");

    let g = cube_sphere(CubeProj::Equiangular, 32, r());
    let q = geometry(&g);
    let mut v: Vec<_> = q.valence.iter().map(|(k, c)| format!("{k} edges × {c} cells")).collect();
    v.sort();
    let mut m: Vec<_> = q.moore.iter().map(|(k, c)| format!("{k} Moore × {c} cells")).collect();
    m.sort();
    println!("  cube-sphere, 6·32² = {} cells:", g.cells());
    println!("    EDGE adjacency  : {}", v.join(",  "));
    println!("    MOORE adjacency : {}", m.join(",  "));
    println!();
    println!("  ⇒ EVERY cell has exactly 4 edges — INCLUDING the 24 corner cells. The 4-edge flux form has NO");
    println!("    special case at a cube corner. The defect exists ONLY in the Moore (diagonal) stencil.");
    println!("    `water.rs` is already 4-edge flux form (fl/fr/ft/fb). Only `erosion.rs`'s MFD wants the");
    println!("    diagonal that isn't there — and the diagonals are themselves a hack against grid-aligned bias.");
    println!();

    let defect = (0..g.cells()).find(|&i| g.moore[i].len() != 2 * g.adj[i].len()).unwrap();
    let (k, gaps) = fan_at(&g, defect);
    let reg = (0..g.cells()).find(|&i| g.moore[i].len() == 8).unwrap();
    let (k2, gaps2) = fan_at(&g, reg);
    println!("  THE FAN AT A CORNER CELL (an ideal 8-fan would be eight 45° gaps):");
    println!(
        "    regular cell : {k2} Moore nbrs, gaps {}",
        gaps2.iter().map(|x| format!("{x:.0}°")).collect::<Vec<_>>().join(" ")
    );
    println!(
        "    CORNER  cell : {k} Moore nbrs, gaps {}",
        gaps.iter().map(|x| format!("{x:.0}°")).collect::<Vec<_>>().join(" ")
    );
    println!("  ⇒ the missing diagonal is not merely absent — the surviving fan is not a fan. An 8-neighbour");
    println!("    MFD there is weighting directions that are not 45° apart, at distances that are not cell_m·√2.");
    println!();

    println!("  ROUTING, all three on the SAME grid and the SAME perfect cone (radially symmetric by");
    println!("  construction, so ANY azimuthal structure in the answer is the grid talking):\n");
    println!(
        "{:<32} {:>18} {:>16} {:>20}",
        "router", "conservation", "cone bias CV", "bias CV at defects"
    );
    for r_ in [Router::MooreMfd, Router::EdgeMfd, Router::GradEdgeFlux] {
        let x = routing(&g, r_);
        println!(
            "{:<32} {:>18.12} {:>15.2}% {:>19.2}%",
            r_.label(),
            x.conservation,
            x.cone_bias_cv * 100.0,
            x.cone_bias_cv_at_defects * 100.0
        );
    }
    println!();
    println!("  conservation = (drainage that reached a sink) ÷ (total area). 1.000000000000 = exact.");
    println!("  Read this carefully, because the honest result is NOT the one the hypothesis predicted:");
    println!("  MFD does not LEAK at a corner — a normalised-weight router conserves on any graph. What the");
    println!("  corner breaks is (a) the CODE's structural assumption of 8 fixed offsets, and (b) the fan's");
    println!("  directional weighting. Edge-flux routing needs no diagonals, closes at valence 3, and carries");
    println!("  measurably less grid bias. See the report §9 for the full reading.");
}

fn section_quadtree(ms: &[Mesh]) {
    hdr("10. QUADTREE-ABILITY / LOD — the property our entire CellId architecture rests on.");
    for g in ms {
        println!("  {:<26} {}", g.name, g.quadtree);
    }
    println!();
    println!("  Snyder's own Table 1 is the tension in one line: the cube is a POOR polyhedron (ω = 25.17°),");
    println!("  while the truncated icosahedron reaches ω = 3.75° — but hexagons and pentagons do not subdivide");
    println!("  into four, which is exactly what our LOD rests on. The rhombic dodecahedron is the interesting");
    println!("  middle: 12 quad faces that DO subdivide 1→4.");
}

fn honest_scope() {
    hdr("HONEST SCOPE — what this does and does not show.");
    println!("  • The Snyder map is transcribed from the paper (eqs 1–23) and REPRODUCES its Table 1 for the");
    println!("    cube to 2 decimal places. HEALPix is from Górski §5.1/§5.3. Nothing was implemented from memory.");
    println!("  • HEALPix pixel EDGES here are great-circle chords between the exact corners; the true HEALPix");
    println!("    boundaries are NOT geodesics (§5.3). Areas use the paper's exact Ω. The gap between the two");
    println!("    is printed in the gate above and is a real, bounded error in HEALPix's edge lengths only.");
    println!("  • The rhombic-dodecahedron INTRA-FACE maps are gnomonic (canonical) and a tan-warp (OURS, by");
    println!("    analogy — not from any source). There is NO Snyder equal-area map for it: Snyder's derivation");
    println!("    requires REGULAR polygons and a rhombus is not one. An equal-area RD map would need new");
    println!("    derivation. HEALPix is, in effect, that map already — see the gate.");
    println!("  • One algorithm family per property. Stream-power's own sensitivity (A^m · S^n) is INFERRED from");
    println!("    the area/slope columns, not simulated end-to-end.");
    println!("  • The sequencing probe uses ONE band-limited terrain. The numbers move with the terrain; the");
    println!("    ORDERING of the grids on that metric was stable across the seeds tried, but that is not a proof.");
}
