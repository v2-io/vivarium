//! The grids. Every one of them is built into the same `Mesh`, so nothing downstream
//! knows which is which.
//!
//! **Provenance discipline.** Each builder says where its formulae came from. Nothing
//! here is written from memory: the Snyder map is transcribed from Snyder 1992
//! eqs (1)–(23) (PDF read this session); HEALPix from Górski et al. 2005 §5.1/§5.3
//! (`ref/research/pdfs/markdowns/astro-ph0409513/`). Where a construction is *ours*
//! and not from a source, it says so in the blurb.

#![allow(dead_code)]

use super::mesh::*;

// ===========================================================================
// The cube family: one topology, four projections.
// ===========================================================================

/// The six cube faces, as (origin, du, dv) in un-normalized cube space, matching
/// `sphere.rs::to_unit`'s axis convention exactly (verified against it in `main`).
pub fn cube_face_basis(f: usize) -> (V3, V3, V3) {
    // for face f, direction = c + u*a + v*b  (u,v ∈ [-1,1]), then normalized.
    match f {
        0 => ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]), // XPos: [1, tv, -tu]
        1 => ([-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]), // XNeg: [-1, tv, tu]
        2 => ([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]), // YPos: [-tu, 1, tv]
        3 => ([0.0, -1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, -1.0]), // YNeg: [-tu, -1, -tv]
        4 => ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),  // ZPos: [tu, tv, 1]
        _ => ([0.0, 0.0, -1.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), // ZNeg: [-tu, tv, -1]
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CubeProj {
    /// `x = tan(u·π/4)` — what `sphere.rs` actually does. Cells subtend near-equal
    /// *angles*; areas still vary.
    Equiangular,
    /// `x = u` — the naive radial projection. Snyder 1992 calls this the Gnomonic and
    /// tabulates its cube distortion as ω = 31.08°, a = 3.000, b = 1.000 (Table 1).
    Gnomonic,
    /// Snyder 1992's modified Lambert azimuthal equal-area map onto the cube face.
    /// EXACTLY equal-area by construction. Constants from the paper's Table 1:
    /// g = 54.73561032°, G = 60°, θ = 45°.
    SnyderEqualArea,
}

impl CubeProj {
    pub fn name(self) -> &'static str {
        match self {
            CubeProj::Equiangular => "cube · equiangular (OURS)",
            CubeProj::Gnomonic => "cube · gnomonic",
            CubeProj::SnyderEqualArea => "cube · Snyder equal-area",
        }
    }
}

// --- Snyder 1992, transcribed. ------------------------------------------------
//
// Table 1 (cube row):        g = 54.73561032°   G = 60°   θ = 45°
// Paper's own cube distortion figures, our correctness target: ω = 25.17°,
// a = 1.248, b = 0.801.  (a·b = 0.9997 ≈ 1, as equal-area requires.)
//
// The map is derived per *right triangle* (face centre A, vertex B, edge-midpoint C);
// the polygon's symmetry replicates it. For the square, the vertex-to-vertex sector is
// 2(90° − θ) = 90°.

pub const SNY_G: f64 = 54.735_610_32; // degrees, centre→vertex spherical distance
pub const SNY_BIG_G: f64 = 60.0; // degrees
pub const SNY_THETA: f64 = 45.0; // degrees

/// Snyder eq (1)/(2): area of the right spherical triangle A′B′C′, in steradians on a
/// unit sphere. `A_GT = (G − θ)·π/180`.
pub fn sny_agt() -> f64 { (SNY_BIG_G - SNY_THETA).to_radians() }

/// Snyder eq (3)+(5): the polyhedron half-side `R′` (in units of sphere radius R) that
/// makes the plane right-triangle area `A_MT = ½(R′ tan g)² sin θ cos θ` equal `A_GT`.
/// For the cube, `tan g = √2`, so `A_MT = ½·2R′²·½ = 0.5 R′²` ⇒ `R′ = √(2·A_GT)`.
pub fn sny_rprime() -> f64 {
    let tg = SNY_G.to_radians().tan();
    let (st, ct) = (SNY_THETA.to_radians().sin(), SNY_THETA.to_radians().cos());
    // A_MT = 0.5 * (R'·tan g)^2 * sinθ cosθ  =  k · R'^2  with k below
    let k = 0.5 * tg * tg * st * ct;
    (sny_agt() / k).sqrt()
}

/// The Snyder **inverse**: plane `(x, y)` on the face square (half-side `R′`, origin at
/// the face centre, `+y` toward a vertex) → `(Az, z)` on the sphere, in radians.
/// Eqs (17)–(23), with the Newton–Raphson of (20)–(22) on `Az`.
/// Returns `(az_rad, z_rad)` where `az` is the true spherical azimuth from the same
/// vertex ray and `z` the spherical distance from the face centre.
pub fn snyder_inverse(x: f64, y: f64) -> (f64, f64) {
    let rp = sny_rprime();
    let (bg, th, g) = (SNY_BIG_G.to_radians(), SNY_THETA.to_radians(), SNY_G.to_radians());
    let sector = 2.0 * (std::f64::consts::FRAC_PI_2 - th); // 90° for the square

    // (17), (18)
    let mut azp = x.atan2(y); // Az′, measured from +y (a vertex)
    let rho = (x * x + y * y).sqrt();
    if rho == 0.0 {
        return (0.0, 0.0);
    }
    // fold Az′ into [0, sector) — "add back the same multiple" at the end (step 2)
    let k = (azp / sector).floor();
    azp -= k * sector;
    // Within a sector the map is symmetric about the sector's bisector (the
    // edge-midpoint ray). Reflect so we always work in [0, sector/2] — the paper's
    // one-twelfth (here one-eighth) right triangle.
    let (azp_w, mirrored) = if azp > sector / 2.0 { (sector - azp, true) } else { (azp, false) };

    // (19): A_G from Az′
    let tg = g.tan();
    let ag = rp * rp * tg * tg / (2.0 * (azp_w.tan().recip() + th.tan().recip()));

    // Newton–Raphson (20)–(22) for the spherical azimuth Az, starting from Az′.
    // F(Az) = A_G − (Az + G + H(Az) − π)   [radian form of (20); (7) says
    //         A_G = (Az + G + H − 180°)·πR²/180° ≡ spherical excess on a unit sphere]
    let mut az = azp_w;
    for _ in 0..40 {
        let hh = (az.sin() * bg.sin() * g.cos() - az.cos() * bg.cos()).clamp(-1.0, 1.0).acos();
        let f = ag - (az + bg + hh - std::f64::consts::PI);
        // (21): F′(Az) = [(cos Az sin G cos g + sin Az cos G)/sin H] − 1
        let fp = (az.cos() * bg.sin() * g.cos() + az.sin() * bg.cos()) / hh.sin() - 1.0;
        let d = -f / fp; // (22) — note our F is the negation of the paper's, so is F′
        az += d;
        if d.abs() < 1e-15 {
            break;
        }
    }

    // (9), (10), (11), (23)
    let q = (tg / (az.cos() + az.sin() * th.tan().recip())).atan();
    let dp = rp * tg / (azp_w.cos() + azp_w.sin() * th.tan().recip());
    let f = dp / (2.0 * rp * (q / 2.0).sin());
    let z = 2.0 * (rho / (2.0 * rp * f)).clamp(-1.0, 1.0).asin();

    let az_out = if mirrored { sector - az } else { az };
    (az_out + k * sector, z)
}

/// The Snyder **forward** map: `(Az, z)` on the sphere → plane `(x, y)`. Eqs (5)–(16).
/// Kept because it is the paper's primary direction and it is what the round-trip test
/// checks the inverse against.
pub fn snyder_forward(az_in: f64, z: f64) -> (f64, f64) {
    let rp = sny_rprime();
    let (bg, th, g) = (SNY_BIG_G.to_radians(), SNY_THETA.to_radians(), SNY_G.to_radians());
    let sector = 2.0 * (std::f64::consts::FRAC_PI_2 - th);
    let k = (az_in / sector).floor();
    let az0 = az_in - k * sector;
    let (az, mirrored) = if az0 > sector / 2.0 { (sector - az0, true) } else { (az0, false) };

    let tg = g.tan();
    // (6), (7)
    let hh = (az.sin() * bg.sin() * g.cos() - az.cos() * bg.cos()).clamp(-1.0, 1.0).acos();
    let ag = az + bg + hh - std::f64::consts::PI;
    // (8)
    let azp = (2.0 * ag).atan2(rp * rp * tg * tg - 2.0 * ag * th.tan().recip());
    // (9), (10), (11), (12)
    let q = (tg / (az.cos() + az.sin() * th.tan().recip())).atan();
    let dp = rp * tg / (azp.cos() + azp.sin() * th.tan().recip());
    let f = dp / (2.0 * rp * (q / 2.0).sin());
    let rho = 2.0 * rp * f * (z / 2.0).sin();

    let azp_out = if mirrored { sector - azp } else { azp } + k * sector;
    (rho * azp_out.sin(), rho * azp_out.cos()) // (15), (16)
}

/// Cube face `(u, v) ∈ [-1,1]²` → unit direction, under a given projection.
pub fn cube_to_unit(proj: CubeProj, f: usize, u: f64, v: f64) -> V3 {
    let (c, a, b) = cube_face_basis(f);
    match proj {
        CubeProj::Gnomonic => unit(add(c, add(scale(a, u), scale(b, v)))),
        CubeProj::Equiangular => {
            let (tu, tv) = ((u * std::f64::consts::FRAC_PI_4).tan(), (v * std::f64::consts::FRAC_PI_4).tan());
            unit(add(c, add(scale(a, tu), scale(b, tv))))
        }
        CubeProj::SnyderEqualArea => {
            // The plane face square has half-side R′ (Snyder), with +y toward a vertex.
            // Our (u,v) axes point at EDGE midpoints, so a vertex is at 45°: rotate.
            let rp = sny_rprime();
            let (px, py) = (u * rp, v * rp);
            let s = std::f64::consts::FRAC_PI_4;
            let (xr, yr) = (px * s.cos() - py * s.sin(), px * s.sin() + py * s.cos());
            let (az, z) = snyder_inverse(xr, yr);
            // Rebuild the direction: rotate the +y (vertex) ray back by −45° in the
            // tangent frame at the face centre, then walk `z` along azimuth `az`.
            let n = unit(c);
            let e_v = unit(add(scale(unit(a), s.sin()), scale(unit(b), s.cos()))); // the vertex ray (u=v=1 dir)
            let e_v = tangent(n, e_v);
            let e_p = cross(n, e_v); // completes the frame
            let dir = add(scale(e_v, az.cos()), scale(e_p, az.sin()));
            unit(add(scale(n, z.cos()), scale(dir, z.sin())))
        }
    }
}

/// Build a whole-sphere cube grid: 6 faces × n×n cells, with **combinatorial**
/// cross-face adjacency (vertex keys are exact integer lattice points on the cube, so
/// the 12 face edges and 8 corners weld with no float matching).
pub fn cube_sphere(proj: CubeProj, n: usize, radius_m: f64) -> Mesh {
    // Exact integer vertex identity: a face lattice point (f, i, j) with i,j ∈ 0..=n
    // maps to a canonical integer point on the cube surface in [-n, n]³.
    let ipt = |f: usize, i: usize, j: usize| -> (i64, i64, i64) {
        let (c, a, b) = cube_face_basis(f);
        let (ui, vj) = (2 * i as i64 - n as i64, 2 * j as i64 - n as i64); // ∈ [-n, n] doubled
        let nn = n as i64;
        let g = |k: usize| -> i64 {
            (c[k] as i64) * nn + (a[k] as i64) * ui / 1 + (b[k] as i64) * vj / 1
        };
        // c, a, b are ±1/0 basis vectors, so this is exact.
        (g(0), g(1), g(2))
    };

    let mut key2v: std::collections::HashMap<(i64, i64, i64), u32> = std::collections::HashMap::new();
    let mut verts: Vec<V3> = Vec::new();
    // SEAM PROOF. The key is a combinatorial lattice point, so two faces meeting at a
    // cube edge weld by *integer identity*, never by float matching. But that is only
    // sound if the two faces' projections actually agree there in 3-D — for a map with
    // an internal azimuth frame (Snyder) a handedness slip would weld two DIFFERENT
    // points and silently corrupt every seam length. So we assert it.
    let mut seam_max = 0.0f64;
    let mut vid = |k: (i64, i64, i64), p: V3, key2v: &mut std::collections::HashMap<(i64, i64, i64), u32>, verts: &mut Vec<V3>, seam_max: &mut f64| -> u32 {
        if let Some(&x) = key2v.get(&k) {
            let d = norm(sub(verts[x as usize], p));
            if d > *seam_max {
                *seam_max = d;
            }
            return x;
        }
        let x = verts.len() as u32;
        verts.push(p);
        key2v.insert(k, x);
        x
    };

    let mut rings: Vec<Vec<u32>> = Vec::new();
    let mut centers: Vec<V3> = Vec::new();
    let mut part: Vec<u32> = Vec::new();

    let uv = |k: usize| (k as f64 / n as f64) * 2.0 - 1.0;
    let uvc = |k: usize| ((k as f64 + 0.5) / n as f64) * 2.0 - 1.0;

    for f in 0..6 {
        for j in 0..n {
            for i in 0..n {
                let corners = [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)];
                let ring: Vec<u32> = corners
                    .iter()
                    .map(|&(ci, cj)| {
                        let p = cube_to_unit(proj, f, uv(ci), uv(cj));
                        vid(ipt(f, ci, cj), p, &mut key2v, &mut verts, &mut seam_max)
                    })
                    .collect();
                rings.push(ring);
                centers.push(cube_to_unit(proj, f, uvc(i), uvc(j)));
                part.push(f as u32);
            }
        }
    }

    assert!(
        seam_max < 1e-12,
        "{}: the 12 face edges DISAGREE by {seam_max:.3e} — two faces project the same \
         lattice vertex to different points on the sphere. The seam is not welded.",
        proj.name()
    );

    // Orient every ring CCW as seen from outside.
    for (c, r) in rings.iter_mut().enumerate() {
        let (a, b, cc) = (verts[r[0] as usize], verts[r[1] as usize], verts[r[2] as usize]);
        if dot(cross(sub(b, a), sub(cc, a)), centers[c]) < 0.0 {
            r.reverse();
        }
    }

    let areas: Vec<f64> = rings
        .iter()
        .map(|r| {
            let vs: Vec<V3> = r.iter().map(|&v| verts[v as usize]).collect();
            poly_area(&vs) * radius_m * radius_m
        })
        .collect();

    let blurb: String = match proj {
        CubeProj::Equiangular => "6 square faces, radial with a tan-warp; what vivarium runs on today".into(),
        CubeProj::Gnomonic => "6 square faces, plain radial projection (Snyder's 'Gnomonic'); the control".into(),
        CubeProj::SnyderEqualArea => "6 square faces, Snyder 1992 modified Lambert azimuthal; EXACTLY equal-area".into(),
    };
    Mesh::build(proj.name(), &blurb, radius_m, verts, rings, centers, areas, part, 6, "YES — 1→4, native")
}

// ===========================================================================
// The RHOMBIC DODECAHEDRON family: 12 quadrilateral faces.
// ===========================================================================
//
// The convex hull of the cube ∪ its dual octahedron. 12 rhombic faces (one per cube
// EDGE), 24 edges, 14 vertices: 8 of valence 3 (the cube-corner directions) and 6 of
// valence 4 (the cube-face-centre / octahedron-vertex directions). Euler: 14−24+12 = 2,
// and Σ(4−valence) = 8·1 + 6·0 = 8 — the SAME irreducible topological charge as the
// cube, as it must be.
//
// Why it earns a row: a rhombus subdivides into four rhombi, so it QUADTREES; and 12
// faces means each spans half the solid angle of a cube face. The caveat to measure
// rather than assume: the rhombus is not square (diagonals in ratio √2; angles
// 70.53°/109.47°), so a shear is baked into the base.
//
// **This grid's intra-face maps are OURS, not from a source.** Snyder 1992's derivation
// requires *regular* polygons (its own abstract says so), and a rhombus is not regular —
// so there is no Snyder equal-area map for this solid, and we do not invent one.

#[derive(Clone, Copy, PartialEq)]
pub enum RdProj {
    /// Bilinear in the (planar) rhombus, then radial projection. The canonical
    /// "gnomonic" analogue — no invention, exactly what radial projection means.
    Gnomonic,
    /// **Ours, by analogy** with the equiangular cube: warp each parallelogram
    /// coordinate by `tan`, scaled to the face's angular half-width along that axis.
    /// Not from any source; labelled so the reader can discount it.
    TanWarp,
}

impl RdProj {
    pub fn name(self) -> &'static str {
        match self {
            RdProj::Gnomonic => "rhombic-dodec · gnomonic",
            RdProj::TanWarp => "rhombic-dodec · tan-warp (ours)",
        }
    }
}

/// The 12 rhombic faces, as (C1, Oa, C2, Ob) in un-normalized RD space: cube vertices
/// at (±1,±1,±1), octahedron vertices at (±2,0,0),(0,±2,0),(0,0,±2).
pub fn rd_faces() -> Vec<[V3; 4]> {
    let mut out = Vec::new();
    // one face per cube edge: fix two of the three coords, vary the third
    for axis in 0..3usize {
        for sa in [-1.0f64, 1.0] {
            for sb in [-1.0f64, 1.0] {
                // the two cube corners of this edge
                let mut c1 = [0.0; 3];
                let mut c2 = [0.0; 3];
                let (p, q) = ((axis + 1) % 3, (axis + 2) % 3);
                c1[p] = sa;
                c2[p] = sa;
                c1[q] = sb;
                c2[q] = sb;
                c1[axis] = 1.0;
                c2[axis] = -1.0;
                // the two octahedron vertices: the face-centre directions of the two
                // cube faces adjacent to this edge
                let mut oa = [0.0; 3];
                oa[p] = 2.0 * sa;
                let mut ob = [0.0; 3];
                ob[q] = 2.0 * sb;
                out.push([c1, oa, c2, ob]);
            }
        }
    }
    assert_eq!(out.len(), 12);
    out
}

/// Rhombus face coords `(s, t) ∈ [0,1]²`: `p = C1 + s(Oa − C1) + t(Ob − C1)`, so
/// `p(1,1) = Oa + Ob − C1 = C2` — a parallelogram, hence a clean quadtree.
pub fn rd_to_unit(proj: RdProj, face: &[V3; 4], s: f64, t: f64) -> V3 {
    let (c1, oa, ob) = (face[0], face[1], face[3]);
    let (ds, dt) = (sub(oa, c1), sub(ob, c1));
    let (s, t) = match proj {
        RdProj::Gnomonic => (s, t),
        RdProj::TanWarp => {
            // Half-angle subtended along each parallelogram axis, measured from the
            // face centre; warp so the *angle* is closer to linear in the coordinate —
            // the equiangular cube's trick, transplanted. OURS, not sourced.
            let ctr = unit(add(add(c1, scale(ds, 0.5)), scale(dt, 0.5))); // p(½,½)
            let half_s = geodesic(unit(add(c1, scale(dt, 0.5))), ctr); // s: 0 → ½ at t=½
            let half_t = geodesic(unit(add(c1, scale(ds, 0.5))), ctr);
            let w = |x: f64, h: f64| {
                let e = (x - 0.5) * 2.0; // ∈ [-1, 1]
                0.5 + 0.5 * (e * h).tan() / h.tan()
            };
            (w(s, half_s), w(t, half_t))
        }
    };
    unit(add(c1, add(scale(ds, s), scale(dt, t))))
}

pub fn rhombic_dodec(proj: RdProj, n: usize, radius_m: f64) -> Mesh {
    let faces = rd_faces();
    let mut dd = VertDedup::new();
    let mut rings: Vec<Vec<u32>> = Vec::new();
    let mut centers: Vec<V3> = Vec::new();
    let mut part: Vec<u32> = Vec::new();

    for (fi, f) in faces.iter().enumerate() {
        for j in 0..n {
            for i in 0..n {
                let mut c = |a: usize, b: usize| {
                    dd.get(rd_to_unit(proj, f, a as f64 / n as f64, b as f64 / n as f64))
                };
                rings.push(vec![c(i, j), c(i + 1, j), c(i + 1, j + 1), c(i, j + 1)]);
                centers.push(rd_to_unit(proj, f, (i as f64 + 0.5) / n as f64, (j as f64 + 0.5) / n as f64));
                part.push(fi as u32);
            }
        }
    }
    let verts = dd.verts.clone();
    for (c, r) in rings.iter_mut().enumerate() {
        let (a, b, cc) = (verts[r[0] as usize], verts[r[1] as usize], verts[r[2] as usize]);
        if dot(cross(sub(b, a), sub(cc, a)), centers[c]) < 0.0 {
            r.reverse();
        }
    }
    let areas: Vec<f64> = rings
        .iter()
        .map(|r| poly_area(&r.iter().map(|&v| verts[v as usize]).collect::<Vec<_>>()) * radius_m * radius_m)
        .collect();
    let blurb: String = match proj {
        RdProj::Gnomonic => "12 rhombic faces (cube ∪ octahedron hull), radial projection".into(),
        RdProj::TanWarp => "12 rhombic faces, tan-warped in the parallelogram coords — OUR construction, not sourced".into(),
    };
    Mesh::build(proj.name(), &blurb, radius_m, verts, rings, centers, areas, part, 12, "YES — 1→4, native (rhombus → 4 rhombi)")
}
