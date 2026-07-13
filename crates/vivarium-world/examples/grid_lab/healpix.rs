//! HEALPix, from the primary: Górski et al. 2005, `ref/research/pdfs/markdowns/astro-ph0409513/`.
//!
//! Centres are the paper's eqs (4),(5) (north polar cap) and (8),(9) (equatorial belt),
//! mirrored for the south. Area is the paper's exact constant `Ω = π/(3·N_side²)`.
//!
//! **The corners took a derivation, and it is worth stating what it rests on.** The
//! paper gives pixel *boundaries*, not corners (§5.3): `cos θ = a + b/φ²` in the caps
//! (eqs 19–20) and `cos θ = a + b·φ` in the belt (eq 22). Inverting those:
//!
//! - In the polar cap, put `t = N_side·√(3(1−z))`. Then eq (19) reads `A ≡ t·(2φ_t/π) = k`
//!   and eq (20) reads `B ≡ t·(1 − 2φ_t/π) = k`, with `A + B = t`. So **(A, B) is a
//!   square lattice**: boundaries are the integer lines, and a pixel centre sits at
//!   `(A, B) = (jj − ½, i − jj + ½)` — half-integers, exactly as it must (checked
//!   against eqs (4),(5)). Corners are `(A ± ½, B ± ½)`.
//! - In the belt, put `U = z − 8φ/(3π)`, `V = z + 8φ/(3π)`. Eq (22)'s two families are
//!   `U = const` and `V = const` at spacing `4/(3N_side)`. So **(U, V) is a square
//!   lattice** too, centres at half-integers, corners at `±1`.
//!
//! Hence every pixel is a *diamond*: two corners at its own ring's latitude, one a ring
//! up, one a ring down. The one place this bites is the cap/belt junction, where a belt
//! pixel's poleward corner lies in the *cap* zone and must be taken from the (A,B)
//! lattice, not extrapolated from the belt's linear-in-`i` formula (which would put it
//! at a latitude no cap vertex occupies). That case is handled explicitly below.
//!
//! **The derivation is not trusted, it is checked**: `Mesh::build` asserts Euler = 2 and
//! that every edge borders exactly two pixels, and `main` asserts every pixel's spherical
//! area against the paper's `Ω` and the total against 4π. If the corner lattice were
//! wrong, none of those would hold.

#![allow(dead_code)]

use super::mesh::*;
use std::f64::consts::PI;

/// Vertex position from the (ring-level, azimuth-index) lattice.
/// `r ∈ 0..=4·nside`; `r = 0` is the north pole, `r = 4·nside` the south pole.
fn vpos(nside: usize, r: usize, m: i64) -> V3 {
    let ns = nside as f64;
    let n4 = 4 * nside;
    if r == 0 {
        return [0.0, 0.0, 1.0];
    }
    if r == n4 {
        return [0.0, 0.0, -1.0];
    }
    let (z, phi) = if r < nside {
        // north cap: eq (4) at t = r; φ from A = m mod r within quadrant qd = m / r
        let t = r as f64;
        let cnt = 4 * r;
        let mm = m.rem_euclid(cnt as i64) as f64;
        (1.0 - t * t / (3.0 * ns * ns), PI * mm / (2.0 * t))
    } else if r <= 3 * nside {
        // belt: eq (8); φ from eq (9) at half-integer j — the boundary condition
        let sigma = ((r - nside) % 2) as f64;
        let cnt = 4 * nside;
        let mm = m.rem_euclid(cnt as i64) as f64;
        (4.0 / 3.0 - 2.0 * r as f64 / (3.0 * ns), (PI / (2.0 * ns)) * (mm + sigma / 2.0))
    } else {
        // south cap: mirror
        let rp = n4 - r;
        let t = rp as f64;
        let cnt = 4 * rp;
        let mm = m.rem_euclid(cnt as i64) as f64;
        (-(1.0 - t * t / (3.0 * ns * ns)), PI * mm / (2.0 * t))
    };
    let st = (1.0 - z * z).max(0.0).sqrt();
    [st * phi.cos(), st * phi.sin(), z]
}

/// Which of the 12 base pixels a `(ring i, in-ring j)` pixel belongs to — the natural
/// coarse partition, and the object whose vertex census we want to look at.
///
/// This is an **exact inversion of the paper's own eqs (10)–(18)**, not a geometric
/// guess: for each candidate base pixel `f`, eq (17) `i = F₁(f)·N − v − 1` gives the
/// required `v = x + y` and eq (18) `j = (F₂(f)·N + h + s)/2` gives the required
/// `h = x − y`; the pixel belongs to `f` iff the resulting `x = (v+h)/2`, `y = (v−h)/2`
/// are integers in `[0, N)`. Exactly one `f` can satisfy that, and `healpix()` asserts
/// each `f` ends up with exactly `N²` pixels — so a wrong inversion cannot pass.
fn base_pixel(nside: usize, i: usize, j: usize) -> u32 {
    let ns = nside as i64;
    let s: i64 = if i < nside || i > 3 * nside { 1 } else { ((i - nside + 1) % 2) as i64 };
    let ringlen: i64 = if i < nside {
        4 * i as i64
    } else if i <= 3 * nside {
        4 * ns
    } else {
        4 * (4 * ns - i as i64)
    };
    for f in 0..12i64 {
        let f_row = f / 4;
        let f1 = f_row + 2;
        let f2 = 2 * (f % 4) - (f_row % 2) + 1;
        let v = f1 * ns - i as i64 - 1;
        if v < 0 || v > 2 * ns - 2 {
            continue;
        }
        // longitude wraps: j is only defined modulo the ring length
        for w in -1i64..=1 {
            let jj = j as i64 + w * ringlen;
            let h = 2 * jj - s - f2 * ns;
            if (v + h).rem_euclid(2) != 0 {
                continue;
            }
            let (x, y) = ((v + h) / 2, (v - h) / 2);
            if x >= 0 && x < ns && y >= 0 && y < ns {
                return f as u32;
            }
        }
    }
    panic!("HEALPix: pixel (ring {i}, j {j}) belongs to no base pixel — eq (17)/(18) inversion failed");
}

pub fn healpix(nside: usize, radius_m: f64) -> Mesh {
    let ns = nside;
    let n4 = 4 * ns;
    let mut dd_map: std::collections::HashMap<(usize, i64), u32> = std::collections::HashMap::new();
    let mut verts: Vec<V3> = Vec::new();
    let mut rings: Vec<Vec<u32>> = Vec::new();
    let mut centers: Vec<V3> = Vec::new();
    let mut part: Vec<u32> = Vec::new();

    let mut vid = |r: usize, m: i64,
                   verts: &mut Vec<V3>,
                   map: &mut std::collections::HashMap<(usize, i64), u32>| -> u32 {
        // canonicalize the azimuth index into its level's range (poles collapse to one)
        let cnt: i64 = if r == 0 || r == n4 {
            1
        } else if r < ns {
            4 * r as i64
        } else if r <= 3 * ns {
            4 * ns as i64
        } else {
            4 * (n4 - r) as i64
        };
        let mm = if cnt == 1 { 0 } else { m.rem_euclid(cnt) };
        if let Some(&x) = map.get(&(r, mm)) {
            return x;
        }
        let x = verts.len() as u32;
        verts.push(vpos(nside, r, mm));
        map.insert((r, mm), x);
        x
    };

    for i in 1..n4 {
        let (ringlen, is_cap_n, is_cap_s) = if i < ns {
            (4 * i, true, false)
        } else if i <= 3 * ns {
            (4 * ns, false, false)
        } else {
            (4 * (n4 - i), false, true)
        };
        for j in 1..=ringlen {
            // (N, E, S, W) corner lattice descriptors
            let (nc, ec, sc, wc): ((usize, i64), (usize, i64), (usize, i64), (usize, i64));
            if is_cap_n {
                let qd = ((j - 1) / i) as i64;
                let jj = j as i64 - qd * i as i64; // 1..=i
                nc = (i - 1, qd * (i as i64 - 1) + jj - 1);
                sc = (i + 1, qd * (i as i64 + 1) + jj);
                wc = (i, qd * i as i64 + jj - 1);
                ec = (i, qd * i as i64 + jj);
            } else if is_cap_s {
                let ip = (n4 - i) as i64;
                let qd = ((j as i64 - 1) / ip) as i64;
                let jj = j as i64 - qd * ip; // 1..=ip
                sc = (i + 1, qd * (ip - 1) + jj - 1);
                nc = (i - 1, qd * (ip + 1) + jj);
                wc = (i, qd * ip + jj - 1);
                ec = (i, qd * ip + jj);
            } else {
                let s = ((i - ns + 1) % 2) as i64;
                let jj = j as i64;
                wc = (i, jj - 1);
                ec = (i, jj);
                // The poleward/equatorward corners sit at the SAME longitude as the
                // centre — but only while the target level is itself in the belt. At the
                // junction rings the corner falls in the cap, whose lattice is different,
                // and must be taken from the (A,B) lattice via the W corner.
                nc = if i == ns {
                    let qd = (jj - 1).div_euclid(ns as i64);
                    let kw = (jj - 1).rem_euclid(ns as i64);
                    (i - 1, qd * (ns as i64 - 1) + kw)
                } else {
                    (i - 1, jj - s)
                };
                sc = if i == 3 * ns {
                    let qd = (jj - 1).div_euclid(ns as i64);
                    let kw = (jj - 1).rem_euclid(ns as i64);
                    (i + 1, qd * (ns as i64 - 1) + kw)
                } else {
                    (i + 1, jj - s)
                };
            }
            let ring = vec![
                vid(nc.0, nc.1, &mut verts, &mut dd_map),
                vid(ec.0, ec.1, &mut verts, &mut dd_map),
                vid(sc.0, sc.1, &mut verts, &mut dd_map),
                vid(wc.0, wc.1, &mut verts, &mut dd_map),
            ];
            rings.push(ring);
            centers.push(center(nside, i, j));
            part.push(base_pixel(nside, i, j));
        }
    }

    for (c, r) in rings.iter_mut().enumerate() {
        let (a, b, cc) = (verts[r[0] as usize], verts[r[1] as usize], verts[r[2] as usize]);
        if dot(cross(sub(b, a), sub(cc, a)), centers[c]) < 0.0 {
            r.reverse();
        }
    }

    // Area is the paper's exact constant: Ω = π/(3 N_side²). We use it, and `main`
    // separately checks the spherical-excess area of the corner quad against it — the
    // gap is a real number, because HEALPix boundaries are NOT great circles (§5.3).
    let omega = PI / (3.0 * (ns * ns) as f64);
    let areas = vec![omega * radius_m * radius_m; rings.len()];

    // The base-pixel inversion must partition the sphere exactly.
    let mut cnt = [0usize; 12];
    for &p in &part {
        cnt[p as usize] += 1;
    }
    for (f, &c) in cnt.iter().enumerate() {
        assert_eq!(c, ns * ns, "HEALPix: base pixel {f} got {c} pixels, want N_side² = {}", ns * ns);
    }
    assert_eq!(rings.len(), 12 * ns * ns, "HEALPix: N_pix must be 12·N_side²");

    Mesh::build(
        "HEALPix",
        "12 equal-area diamonds on a rhombic-dodecahedral base; iso-latitude; NESTED quadtree",
        radius_m,
        verts,
        rings,
        centers,
        areas,
        part,
        12,
        "YES — 1→4, the NESTED scheme",
    )
}

/// Pixel centre, Górski eqs (4),(5) / (8),(9), mirrored for the south.
fn center(nside: usize, i: usize, j: usize) -> V3 {
    let ns = nside as f64;
    let n4 = 4 * nside;
    let (z, phi) = if i < nside {
        (1.0 - (i * i) as f64 / (3.0 * ns * ns), (PI / (2.0 * i as f64)) * (j as f64 - 0.5))
    } else if i <= 3 * nside {
        let s = ((i - nside + 1) % 2) as f64;
        (4.0 / 3.0 - 2.0 * i as f64 / (3.0 * ns), (PI / (2.0 * ns)) * (j as f64 - s / 2.0))
    } else {
        let ip = n4 - i;
        (
            -(1.0 - (ip * ip) as f64 / (3.0 * ns * ns)),
            (PI / (2.0 * ip as f64)) * (j as f64 - 0.5),
        )
    };
    let st = (1.0 - z * z).max(0.0).sqrt();
    [st * phi.cos(), st * phi.sin(), z]
}
