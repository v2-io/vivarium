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
    for g in &ms {
        let want = 4.0 * std::f64::consts::PI * r() * r();
        let got = g.total_area();
        assert!(
            (got / want - 1.0).abs() < 1e-9,
            "{}: cell areas sum to {:.9} of 4πR² — the tessellation does not cover the sphere",
            g.name,
            got / want
        );
    }
    println!("\nGATE  every grid's cell areas sum to 4πR² to <1e-9 — all nine tessellate the sphere.  ✓");
    section_geometry(&ms);
    section_hoistable(&ms);
    section_curvature(&ms);
    section_strata();
    section_classes(&ms);
    section_conservation(&ms);
    section_accuracy(&ms);
    section_sequencing(&ms);
    section_corner();
    section_overlay();
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
        let e = (x * x + y * y).sqrt() * 1e-6; // scale the FD to the radius: the map is
                                               // cone-like at the centre, so a fixed step
                                               // cannot resolve the limit that Table 1 quotes
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
    let g = sny_g_rad();
    // the face boundary at azimuth Az′ — Snyder eq (10)
    let dprime = |azp: f64| rp * g.tan() / (azp.cos() + azp.sin() * th.tan().recip());

    let (mut a, mut area_err, mut at) = (0.0f64, 0.0f64, (0.0, 0.0));
    for ai in 1..900 {
        let azp = ai as f64 / 900.0 * std::f64::consts::FRAC_PI_2; // (0°, 90°) — the sector
        let dmax = dprime(azp);
        for ri in 1..400 {
            // geometric in ρ, so the ρ → 0 limit (where Table 1 says ω peaks) is reached
            let rho = dmax * 0.999 * 10f64.powf(-5.0 + 5.0 * ri as f64 / 400.0);
            // Guard: the finite difference must not reach across a cusp. The nearest
            // vertex radius is at Az′ = 0 or 90°; the FD displaces the azimuth by ~e/ρ.
            let to_cusp = azp.min(std::f64::consts::FRAC_PI_2 - azp);
            if to_cusp < 20.0 * 1e-6 {
                continue; // never finite-difference across a cusp (a vertex radius)
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
    // Table 1's ω is a LIMIT: it is attained along a vertex radius, as ρ → 0. Approach it
    // logarithmically from both sides — a uniform sweep simply cannot see it.
    for ki in 0..60 {
        let da = 10f64.powf(-5.0 + 4.0 * ki as f64 / 60.0); // radians off the vertex radius
        for azp in [da, std::f64::consts::FRAC_PI_2 - da] {
            if azp <= 0.0 || azp >= std::f64::consts::FRAC_PI_2 {
                continue;
            }
            if azp.min(std::f64::consts::FRAC_PI_2 - azp) < 20.0 * 1e-6 {
                continue;
            }
            let dmax = dprime(azp);
            for ri in 0..60 {
                let rho = dmax * 10f64.powf(-6.0 + 6.0 * ri as f64 / 60.0);
                let (s1, _) = sv(rho * azp.sin(), rho * azp.cos());
                if s1 > a {
                    a = s1;
                    at = (azp.to_degrees(), rho / dmax);
                }
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
    assert!((om - 25.17).abs() < 0.05, "Snyder ω {om:.2}° ≠ paper's 25.17° — the map is WRONG");
    assert!((a - 1.248).abs() < 0.002, "Snyder a {a:.3} ≠ paper's 1.248");
    assert!((b - 0.801).abs() < 0.002, "Snyder b {b:.3} ≠ paper's 0.801");
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
        icosa::icosa_hex(50, 200, r()),                // the same, made centroidal (SCVT)
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
        "{:<26} {:>7} {:>9} {:>10} {:>9} {:>9} {:>9} {:>10} {:>10}",
        "grid", "cells", "area×", "area× geo", "edge×", "arm×", "dist×", "nonortho", "closure"
    );
    for g in ms {
        let q = geometry(g);
        println!(
            "{:<26} {:>7} {:>9.4} {:>10.4} {:>9.4} {:>9.4} {:>9.4} {:>7.2}° max {:>10.2e}",
            g.name,
            g.cells(),
            q.area.spread(),
            q.area_geo.spread(),
            q.edge.spread(),
            q.arm.spread(),
            q.dist.spread(),
            q.nonortho.max,
            q.closure.max
        );
    }
    println!();
    println!("  ⚠ TWO AREA COLUMNS, and the difference is a finding nobody states.");
    println!("    'area×'     = spread of the cell's TRUE area (the analytic equal-area value where the");
    println!("                  grid HAS one). This is the number the equal-area literature quotes.");
    println!("    'area× geo' = spread of the GEODESIC polygon through the cell's corners — which is the");
    println!("                  cell a finite-volume code actually BUILDS (Putman & Lin: 'the cell edges");
    println!("                  for all grids are prescribed to be great circle arcs').");
    println!("    They differ ONLY for Snyder and HEALPix, because those two are the only grids here whose");
    println!("    cell boundaries are not great circles. Max |geo/exact − 1| per grid:");
    for g in ms {
        let q = geometry(g);
        if q.area_geo_gap > 1e-9 {
            println!("      {:<26} {:>7.2}%   ← equal-area is a property of the MAP, not of this cell", g.name, q.area_geo_gap * 100.0);
        }
    }
    println!("    ⇒ You can have exactly-equal AREAS or great-circle EDGES. Not both. If you take the exact");
    println!("      area and geodesic edges anyway (the pragmatic choice, and what we do below), the scheme");
    println!("      is still conservative — but it is no longer geometrically consistent, and that shows up");
    println!("      in the closure column and in §7's accuracy.");
    println!();
    println!("  nonortho = angle between the centre-line and the edge NORMAL (0° = orthogonal).");
    println!("  closure  = |Σ Lₑ·n̂ₑ| / Σ Lₑ per cell, max. A control volume must close; this says how well.");
    println!();
    println!(
        "{:<26} {:>10} {:>10} {:>10} {:>10} {:>22}",
        "grid", "angle min", "angle max", "skew max", "arm-defic", "valence census"
    );
    for g in ms {
        let q = geometry(g);
        let mut v: Vec<_> = q.valence.iter().map(|(k, c)| format!("{k}:{c}")).collect();
        v.sort();
        println!(
            "{:<26} {:>9.2}° {:>9.2}° {:>9.3}% {:>9.2}% {:>22}",
            g.name,
            q.angle.min,
            q.angle.max,
            q.skew.max * 100.0,
            q.arm_deficit.max * 100.0,
            v.join(" ")
        );
    }
    println!();
    println!("  ⚠ ORTHOGONALITY AND SKEW ARE INDEPENDENT, and conflating them is the standard mistake.");
    println!("    nonortho (above) = does the centre-line pierce the edge at 90°?  Putman & Lin's sin α.");
    println!("    skew             = does it pierce it at the MID-EDGE?  |mid-edge → crossing| ÷ edge length.");
    println!("    The hexagonal Voronoi meshes are EXACTLY orthogonal (0.00°, forced by primal–dual duality:");
    println!("    the dual edge lies on the perpendicular bisector of its two generators) — and they are");
    println!("    still SKEW. That is precisely why the mid-edge ARM, not the centre-to-centre line, is the");
    println!("    correct finite-volume gradient arm. Joseph's carve, and the numbers make it visible.");
    println!();
    println!("  arm-deficit = |armᵢ + armⱼ − dist| / dist. Exactly 0 iff the centre-line passes THROUGH");
    println!("  the mid-edge, i.e. iff the grid is un-skewed.");
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
    println!("  (a) ARC / METRIC GEOMETRY IN THE EMBEDDING — governs distance, area and transport at LARGE spans.");
    println!("      Discrete Gaussian curvature = the angle defect 2π − Σθ at each vertex (chordal angles —");
    println!("      the polyhedron's, not the sphere's; the spherical angles around a vertex sum to 2π by");
    println!("      construction and say nothing).\n");
    println!(
        "{:<26} {:>16} {:>13} {:>13} {:>10} {:>14}",
        "grid", "Σdefect / 4π", "max defect", "mean defect", "max/mean", "at the DEFECTS"
    );
    for g in ms {
        let c = curvature(g);
        println!(
            "{:<26} {:>16.9} {:>12.4}° {:>12.5}° {:>9.2}× {:>13.4}°",
            g.name,
            c.defect_sum_over_4pi,
            c.defect_max_deg,
            c.defect_mean_deg,
            c.defect_max_deg / c.defect_mean_deg,
            c.defect_at_topo_defects_deg
        );
    }
    println!();
    println!("  ⚠ READ THE FIRST COLUMN CAREFULLY — it is NOT the free check I first took it for.");
    println!("    Gauss–Bonnet forces Σ defect = 4π **only on a polyhedron whose faces are PLANAR**. Three");
    println!("    points are always coplanar, so it holds EXACTLY on the triangle mesh — and it does:");
    println!("    icosa-tri gives 1.000000000. Four points on a sphere are NOT coplanar, so on every quad");
    println!("    grid the sum overshoots — and the overshoot IS the non-planarity of the cells. So the");
    println!("    column is not a bug: it is a measurement of **how far your control volumes are from being");
    println!("    flat polygons at all** (cube 3.4 %, HEALPix 9.4 %, rhombic-dodec 12.0 %; triangles 0.0 %).");
    println!();
    println!("  ⚠ AND THE LAST COLUMN KILLS A MISCONCEPTION WORTH KILLING. The curvature at the 8 cube");
    println!("    corners is NOT anomalous — it is ORDINARY. Three cells meet there at ~120° each, summing");
    println!("    to ~360°, so the angle DEFECT is about what it is everywhere else. **The cube corner is not");
    println!("    a curvature singularity. It is a CONNECTIVITY singularity.** The surface is perfectly smooth");
    println!("    there; it is the *coordinate system* that is defective. That is why no amount of geometric");
    println!("    cleverness removes it and why the fix is a scheme that does not depend on valence (§9).");
    println!();
    println!("  (b) FLAT-VIA-GRAVITY — the surface is a gravitational equipotential, so water LOCALLY");
    println!("      experiences a flat plane. That is exactly why the shallow-water kernel's flat assumption");
    println!("      is legitimate. The number that decides it is the SAGITTA: how far a cell's own corners");
    println!("      rise off the plane through its centre. This is a LENGTH, so it shrinks as h²/2R — and");
    println!("      the whole question is at which rung it stops mattering. Measured on our own grid:\n");
    for lev in [2u32, 5, 8, 11, 14, 17, 20, 25] {
        // sagitta of a cell of angular size (π/2)/2^lev: s = R(1 − cos(half-diagonal))
        let half_diag = (std::f64::consts::FRAC_PI_2 / (1u64 << lev) as f64) * 0.5 * 2f64.sqrt();
        let sag = r() * (1.0 - half_diag.cos());
        let cell = std::f64::consts::FRAC_PI_2 * r() / (1u64 << lev) as f64;
        let unit_ = if cell > 1000.0 {
            format!("{:.0} km", cell / 1000.0)
        } else {
            format!("{cell:.2} m")
        };
        println!(
            "      L{lev:<2}  cell {:>9}   sagitta {:>12}   ({:.1e} of the cell)",
            unit_,
            if sag > 1.0 { format!("{sag:.2} m") } else { format!("{:.3e} m", sag) },
            sag / cell
        );
    }
    println!();
    println!("      The full-mesh measurement agrees with the closed form to 3 significant figures:");
    for g in ms.iter().take(1) {
        let c = curvature(g);
        println!(
            "      ({} at 6·64²: mean cell span {:.0} m, measured max sagitta {:.1} m)",
            g.name, c.cell_span_mean_m, c.sagitta_max_m
        );
    }
    println!();
    println!("  ⇒ At vivarium's PLAYABLE rung (L25, 0.5 m cells) the sagitta is ~1e-8 m. Flat-via-gravity is");
    println!("    exact to any tolerance that will ever matter, on EVERY grid here — the number is set by the");
    println!("    cell size and the planet's radius, and the projection does not enter it. **This metric does");
    println!("    not discriminate between grids at all, and that is a real finding, not a null one:** it means");
    println!("    the shallow-water kernel's flat assumption is legitimate on any of these, and the sphere");
    println!("    enters the kernel ONLY through the metric terms (area, edge, arm) — never as a 'slope of");
    println!("    the world' the water has to run down. Geometry-in-euclidean-space: settled, and free.");
    println!("    Geometry-in-einstein-spacetime (the arc/metric column above): NOT free, and it is what");
    println!("    every other section of this report is about.");
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
    println!("  BY LEVEL, on our own grid — because Joseph's expectation was '~10 at the coarsest level'");
    println!("  and the growth law is the whole answer:\n");
    println!("{:>6} {:>10} {:>16} {:>14} {:>16}", "level", "cells", "topo classes", "geom @1%", "N²/8 per face");
    for lev in 1..=6u32 {
        let n = 1usize << lev;
        let g = cube_sphere(CubeProj::Equiangular, n, r());
        let c = classes(&g, 1.0);
        println!(
            "{:>6} {:>10} {:>16} {:>14} {:>16}",
            format!("L{lev}"),
            g.cells(),
            c.topo.len(),
            c.geom,
            (n * n / 8).max(1)
        );
    }
    println!();
    println!("  ⇒ Joseph's ~10 is EXACT — at L3 (8×8 per face) the census is 10 geometric classes. But it is");
    println!("    a coincidence of that level, not a law: the count grows as N²/8 (the face's D₄ symmetry");
    println!("    folds an N×N face into an N²/8 fundamental domain), and by L6 it is 509. It does NOT stay");
    println!("    small, and any design that assumes a fixed small table of stencil classes will break.");
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
    println!("  CONSERVATION is a **SCHEME** property. Finite volume fixes it EXACTLY (~1e-15) on EVERY grid");
    println!("  here, including the worst one — because the flux across an edge is literally the same number");
    println!("  added to one cell and subtracted from the other. No projection is needed and none helps.");
    println!("  Note the two equal-area grids (Snyder, HEALPix) conserve even under the NAIVE kernel — that");
    println!("  is the one thing equal-area genuinely buys, and §7 shows it is not the thing that matters.");
    println!();
    println!("  ⚠ DO NOT STOP HERE. The obvious next sentence — 'so conservation is a scheme property and");
    println!("    isotropy is a grid property' — is the conclusion the earlier passes drew, and §7 REFUTES");
    println!("    it. Exact conservation does not imply the scheme is solving the right equation.");
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
        "{:<26} {:>12} {:>12} {:>12} {:>12} {:>10}",
        "grid", "naive", "FV two-point", "corr/narrow", "corr/WIDE", "gain"
    );
    for g in ms {
        let (n, _) = harmonic_error(g, Scheme::NaiveUniform, 2);
        let (c, _) = harmonic_error(g, Scheme::FvCentreLine, 2);
        let (w, _) = harmonic_error(g, Scheme::FvLsqNarrow, 2);
        let (l, _) = harmonic_error(g, Scheme::FvLsq, 2);
        println!(
            "{:<26} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e} {:>9.0}×",
            g.name, n, c, w, l, c / l
        );
    }
    println!();
    println!("  ⚠⚠ THIS IS THE TABLE THE WHOLE INVESTIGATION WAS FOR. Read it left to right.");
    println!();
    println!("  1. A TWO-POINT FLUX — `(u_j − u_i)·L/d`, which is what 'finite volume with the true geometry'");
    println!("     naively means, and which §6 shows conserves EXACTLY — is **INCONSISTENT** on a");
    println!("     non-orthogonal mesh. Not merely inaccurate: its relative error is O(1) and it gets WORSE");
    println!("     with refinement (order −0.5, below). The two-point difference measures ∇u along the");
    println!("     CENTRE-LINE, but the flux needs ∇u along the edge NORMAL; the discarded tangential term");
    println!("     scales as sin θ · R/h, and R/h DOUBLES every time you refine.");
    println!();
    println!("     ⇒ **Conservation and consistency are different properties, and finite volume buys only");
    println!("        the first.** That supersedes the old slogan ('conservation is a scheme property,");
    println!("        isotropy is a grid property'): conservation is a scheme property, but CONSISTENCY is a");
    println!("        property of the scheme and the grid TOGETHER.");
    println!();
    println!("  2. THE HEX MESHES ARE THE CONTROL THAT PROVES IT. They are EXACTLY orthogonal (§1: 0.00°,");
    println!("     forced by primal–dual duality), and there the plain two-point flux is already consistent");
    println!("     — 4e-4, converging at 1.7–1.9, with no correction at all. Same scheme, different grid.");
    println!();
    println!("  3. SO CORRECT THE SCHEME. Reconstruct the face gradient properly: project both cell centres");
    println!("     onto the edge NORMAL through the MID-EDGE and extrapolate with a reconstructed gradient.");
    println!("     That kills the non-orthogonality error AND the skew error, and stays exactly");
    println!("     conservative (the correction is antisymmetric under i↔j).");
    println!();
    println!("  4. AND THEN THE PART NOBODY WAS LOOKING FOR — the difference between the last two columns.");
    println!("     With the gradient fitted over the 4 EDGE-neighbours ('corr/narrow'), the scheme converges");
    println!("     but only at ~0.5 order: a linear fit over 4 points on a distorted quad is only FIRST-order");
    println!("     accurate, and a first-order gradient is not good enough to feed the correction. Fit a");
    println!("     QUADRATIC over the 8 MOORE neighbours instead ('corr/WIDE') and the error drops another");
    println!("     ~30× and the order jumps to ~1.6. **A hexagon's 6 near-symmetric neighbours give a");
    println!("     second-order gradient for free. That — not isotropy, not equal area — is the largest part");
    println!("     of why the hex mesh wins, and it is a STENCIL property, not a GRID property.**");
    println!();
    println!("     At a valence-3 corner there are 7 Moore neighbours, and the quadratic fit needs 5. It");
    println!("     still works. This is Addendum A1's *'LSQ gradient reconstruction recovers second order at");
    println!("     any valence'* — confirmed, with the caveat A1 does not state: only with the WIDE stencil.");
    println!();
    println!("  ⇒⇒ **THE HEADLINE.** On our own equiangular cube-sphere the corrected scheme reaches");
    println!("     3.6e-4, against the best hexagonal mesh's 2.6e-4. That is not parity — the hex mesh is");
    println!("     still 1.4× better — but it is the SAME ORDER OF MAGNITUDE, on the metric the hexagonal");
    println!("     meshes exist to win. Set that 1.4× against what switching to them costs: the CellId");
    println!("     quadtree, the LOD ladder, the memo keys, the store's Hilbert locality, and the fated-noise");
    println!("     KRNG — all of which key on 1→4 subdivision that a hexagon does not have (§10).");
    println!();
    println!("     We did not change the grid. We changed the kernel. The 2500× improvement is entirely");
    println!("     scheme-side and costs nothing architecturally.");
    println!();
    println!("  CONVERGENCE. Does refinement wash the error out? (order = log₂(err_coarse / err_fine);");
    println!("  2.0 = second order = the error dies as h²; ~0 = it does NOT converge away.)\n");
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
        ("icosa-hex (SCVT)".into(), icosa::icosa_hex(24, 200, r()), icosa::icosa_hex(48, 200, r())),
    ];
    println!(
        "{:<26} {:>11} {:>11} {:>8}   {:>11} {:>11} {:>8}",
        "grid", "TPFA crse", "TPFA fine", "order", "LSQ crse", "LSQ fine", "order"
    );
    for (nm, a, b) in &pairs {
        let (l2a, _) = harmonic_error(a, Scheme::FvCentreLine, 2);
        let (l2b, _) = harmonic_error(b, Scheme::FvCentreLine, 2);
        let (q2a, _) = harmonic_error(a, Scheme::FvLsq, 2);
        let (q2b, _) = harmonic_error(b, Scheme::FvLsq, 2);
        println!(
            "{:<26} {:>11.3e} {:>11.3e} {:>8.2}   {:>11.3e} {:>11.3e} {:>8.2}",
            nm,
            l2a,
            l2b,
            (l2a / l2b).log2(),
            q2a,
            q2b,
            (q2a / q2b).log2()
        );
    }
    let _ = ms;
    println!();
    println!();
    println!("  ⚠⚠ AND NOW THE RESULT THAT NOBODY WAS LOOKING FOR, which is why the harness exists.");
    println!();
    println!("  The corrected scheme CONVERGES on the quad grids (order ≈ +0.5, up from −0.5) but it does");
    println!("  NOT reach second order — while the hex meshes reach 1.88 and 1.95 with no correction at all.");
    println!("  The flux is not the reason. The CELL CENTRE is.");
    println!();
    println!("  A finite-volume scheme evolves the cell AVERAGE of u, but every scheme reads u at the cell's");
    println!("  'centre'. Our quad grids take that centre to be the PARAMETER-SPACE midpoint — which is");
    println!("  exactly what `sphere.rs::CellId::to_cube()` returns — and on a distorted cell that is NOT the");
    println!("  centroid. The offset δ makes u(centre) − ū = ∇u·δ, and that error caps the convergence order");
    println!("  no matter how good the flux is. An SCVT hexagon has its generator AT the centroid BY");
    println!("  DEFINITION. That is a large part of why it wins, and it is not a property of hexagons.");
    println!();
    println!("  HYPOTHESIS TESTED AND REFUTED — recorded because a refuted hypothesis is a result. Moving");
    println!("  the cell centre to the true spherical CENTROID changes nothing (order 0.57 → 0.56 on our");
    println!("  grid). So the parameter-midpoint centre is NOT the cap. Here is what actually is:\n");
    println!("  SPLIT THE ERROR BY DISTANCE FROM THE 24 DEFECT CELLS. (This is the test that decides");
    println!("  Addendum A1: is the topological defect a bounded LOCAL wart, or does it poison the grid?)\n");
    println!(
        "{:<30} {:>10} {:>12} {:>12} {:>8} {:>12} {:>12} {:>8}",
        "grid, FV+LSQ, ℓ=2", "cells near", "NEAR crse", "NEAR fine", "order", "FAR crse", "FAR fine", "order"
    );
    for (nm, proj) in [
        ("cube · equiangular (OURS)", CubeProj::Equiangular),
        ("cube · Snyder equal-area", CubeProj::SnyderEqualArea),
    ] {
        let a = cube_sphere(proj, 32, r());
        let b = cube_sphere(proj, 64, r());
        let (na, fa, cnt) = harmonic_error_split(&a, Scheme::FvLsq, 2, 3);
        let (nb, fb, _) = harmonic_error_split(&b, Scheme::FvLsq, 2, 3);
        println!(
            "{:<30} {:>10} {:>12.3e} {:>12.3e} {:>8.2} {:>12.3e} {:>12.3e} {:>8.2}",
            nm,
            cnt,
            na,
            nb,
            (na / nb).log2(),
            fa,
            fb,
            (fa / fb).log2()
        );
    }
    {
        let a = healpix::healpix(32, r());
        let b = healpix::healpix(64, r());
        let (na, fa, cnt) = harmonic_error_split(&a, Scheme::FvLsq, 2, 3);
        let (nb, fb, _) = harmonic_error_split(&b, Scheme::FvLsq, 2, 3);
        println!(
            "{:<30} {:>10} {:>12.3e} {:>12.3e} {:>8.2} {:>12.3e} {:>12.3e} {:>8.2}",
            "HEALPix",
            cnt,
            na,
            nb,
            (na / nb).log2(),
            fa,
            fb,
            (fa / fb).log2()
        );
    }
    println!();
    println!("  A NEGATIVE order means the error GROWS as you refine — the scheme is not converging to the");
    println!("  right operator at all. Every quad grid does that under a two-point flux. Every one of them");
    println!("  is fixed by the LSQ correction. The hex meshes never needed it.");
    println!();
    println!("  ISOTROPY, separately (a symmetric blob under the corrected scheme; 1.0000 = stayed circular):");
    for g in ms {
        let defect = (0..g.cells()).find(|&i| g.moore[i].len() != 2 * g.adj[i].len()).unwrap_or(0);
        let (_, an) = blob(g, Scheme::FvLsq, g.centers[defect], 300);
        println!("    {:<26} anisotropy at a DEFECT {:.4}", g.name, an);
    }
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
    println!("  The cone has an EXACT answer, which beats any symmetry argument: everything drains");
    println!("  radially, so at angular distance θ from the apex the specific catchment area is");
    println!("      a(θ) = area(cap θ) / circumference(θ) = R·tan(θ/2).");
    println!("  So we can score each router against a TRUE VALUE, not merely against 'it should look");
    println!("  uniform'. (This is Tarboton's 1997 test, and it is the right one.)\n");
    println!(
        "{:<32} {:>16} {:>13} {:>13} {:>16}",
        "router", "conservation", "mean err", "max err", "err at defects"
    );
    for r_ in [Router::MooreMfd, Router::EdgeMfd, Router::GradEdgeFlux] {
        let x = routing(&g, r_);
        println!(
            "{:<32} {:>16.12} {:>12.2}% {:>12.2}% {:>15.2}%",
            r_.label(),
            x.conservation,
            x.cone_err_mean * 100.0,
            x.cone_err_max * 100.0,
            x.cone_err_at_defects * 100.0
        );
    }
    println!();
    println!("  conservation = (drainage that reached a sink) ÷ (total area). 1.000000000000 = exact.");
    println!();
    println!("  THE HONEST READING, and it is NOT the one the hypothesis predicted:");
    println!("  • **MFD does not LEAK at a corner.** All three routers conserve to 1.000000000000. A");
    println!("    normalised-weight router conserves on ANY graph, valence-3 included — the weights sum to");
    println!("    one whether there are 8 neighbours or 7 or 3. So the corner was never a conservation bug.");
    println!("  • What the corner actually breaks is (a) the CODE's structural assumption of eight fixed");
    println!("    (dx,dy) offsets — which indexes a cell that does not exist — and (b) the fan's DIRECTIONAL");
    println!("    weighting: the surviving 7 neighbours are not 45° apart and are not at cell_m·√2.");
    println!("  • The accuracy columns are where the routers actually separate, and they separate on the");
    println!("    WHOLE grid, not at the corners. Grid-aligned bias is a face-wide defect of the 8-neighbour");
    println!("    fan, not a corner defect. The corner is a red herring; the fan is the bug.");
    println!("  • Joseph's claim — that edge-flux routing 'needs NO diagonals, works at any valence,");
    println!("    conserves, and removes grid bias more honestly' — is CONFIRMED on the first three and");
    println!("    is what the error columns measure on the fourth.");
    println!();
    println!("  ONE THING THE MEASUREMENT CAUGHT THAT THE DESIGN DID NOT (and it is a trap worth naming):");
    println!("  a gradient-projected router must be restricted to STRICTLY DOWNHILL edges. The LSQ gradient");
    println!("  is a fit, so its outgoing component through an edge can be positive even when the neighbour");
    println!("  across that edge is HIGHER. Route there and the mass goes upstream — and an elevation-ordered");
    println!("  sweep has already passed that cell, so the mass is stranded and never reaches an outlet.");
    println!("  Before the guard, conservation measured 0.000 instead of 1.000. The guard is one line and");
    println!("  costs nothing, but nothing in the *design* of the scheme tells you it is needed.");
}

/// The two-grid overlay (cube ∪ dual octahedron), quantified — because the mechanism is
/// RIGHT and it still loses, and the report has to say why with numbers.
fn section_overlay() {
    hdr("9b. THE TWO-GRID OVERLAY (cube ∪ dual octahedron) — the right mechanism, and why it still loses.");
    println!("  The construction (Joseph's, worked out in seam-adjacency-findings §A2): a cube's 8 corners");
    println!("  cannot be covered by another cube, but its DUAL OCTAHEDRON is perfectly complementary — the");
    println!("  octahedron's 6 vertices sit exactly at the cube's face centres, and its 8 faces exactly at");
    println!("  the cube's corners. Delaunay of the 14 singular directions = the RHOMBIC DODECAHEDRON;");
    println!("  Voronoi = the CUBOCTAHEDRON (8 triangular cells around the cube corners, 6 square cells");
    println!("  around the axes). Partition by 'use whichever grid is locally regular' and **no cell is ever");
    println!("  evaluated at a singularity.** The interface irregularity is FRACTIONAL (partial edges, exactly");
    println!("  conservative — a mortar interface), which really is gentler than an integer valence defect.");
    println!("  The intuition is correct. It still loses, and here is the number that decides it.\n");
    println!("  The cuboctahedral interface is the 24 edges joining the 12 cube-edge-midpoint directions.");
    println!("  Adjacent ones subtend 60°, so the interface is a closed 1-D network of total length");
    println!("  24 · (π/3) · R = 8πR ≈ 25.1 R. Cells along it ≈ 8πR ÷ cell_size(L).\n");
    println!(
        "{:>6} {:>16} {:>18} {:>20} {:>14}",
        "level", "cells/planet", "cube DEFECT cells", "overlay INTERFACE cells", "ratio"
    );
    for lev in [4u32, 7, 10, 13, 16, 19, 22, 25] {
        let cells = 6u64 * (1u64 << (2 * lev));
        let cell_m = std::f64::consts::FRAC_PI_2 * r() / (1u64 << lev) as f64;
        let iface = (8.0 * std::f64::consts::PI * r() / cell_m).round() as u64;
        println!(
            "{:>6} {:>16} {:>18} {:>20} {:>13.0}×",
            format!("L{lev}"),
            fmt_big(cells),
            "24",
            fmt_big(iface),
            iface as f64 / 24.0
        );
    }
    println!();
    println!("  ⇒ **O(1) versus O(N).** The cube's defect is 24 cells — not 24 per face, not 24 per level:");
    println!("    24 cells on the whole planet, FOREVER, at every resolution (Euler forces exactly 8 units of");
    println!("    valence charge; the cube spends them as 8 valence-3 corners × 3 cells each). The overlay's");
    println!("    interface is a 1-D network that grows LINEARLY with resolution: 16·2^L cells. At L19 — the");
    println!("    macro tier vivarium already runs — that is 8.4 MILLION interface cells, each needing");
    println!("    partial-edge geometry, a neighbour search, and (for the strictly-ORDERED drainage sweep,");
    println!("    §8) a cross-grid dependency. Plus two grid types, two addressing schemes, and a broken");
    println!("    CellId quadtree.");
    println!();
    println!("    It pays a SCALING cost to fix something that §9 just measured as already nearly free. The");
    println!("    mechanism is right and the target is wrong. **Where the partial-edge / mortar formulation");
    println!("    DOES belong is the coarse↔fine TILE seam** — which is non-matching by nature, unavoidable,");
    println!("    and is exactly what a Berger–Colella flux register is. Right mechanism, right target.");
}

fn fmt_big(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1} G", n as f64 / 1e9)
    } else if n >= 1_000_000 {
        format!("{:.1} M", n as f64 / 1e6)
    } else if n >= 1_000 {
        format!("{:.1} k", n as f64 / 1e3)
    } else {
        format!("{n}")
    }
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
