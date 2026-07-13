//! The "odd one" — the geodesic/icosahedral family. It is the entry that trades away
//! simple quadtree subdivision for near-isotropy, which is precisely the axis our
//! findings say matters.
//!
//! Three grids come out of one construction:
//!   * **icosa-tri**  — the class-I geodesic triangle mesh. It DOES quadtree (1 triangle
//!     → 4), so it is the honest "keeps LOD" member of the family. 12 valence-5 vertices.
//!   * **icosa-hex**  — its **Voronoi dual**: hexagons everywhere, 12 pentagons. This is
//!     the MPAS-style mesh. It does NOT quadtree.
//!   * **icosa-hex (SCVT)** — the same, after Lloyd iterations move each generator to its
//!     cell's centroid (a spherical centroidal Voronoi tessellation, which is what MPAS
//!     actually runs). The comparison of raw-dual vs SCVT is itself a measurement: it
//!     shows how much of the hex mesh's quality is the *dual* and how much is the
//!     *optimization*.
//!
//! The dual is a true Voronoi diagram, not an approximation: on a sphere the geodesic
//! triangulation of a point set is its Delaunay triangulation, and the circumcentre of a
//! spherical triangle `(a,b,c)` is `±unit((b−a) × (c−a))` — the point equidistant from
//! all three. So the dual cell of a generator is exactly its Voronoi cell, and the
//! primal edge is perpendicular to the dual edge **by construction**. That predicts a
//! non-orthogonality of ~0, which is a falsifiable claim the metrics will check.

#![allow(dead_code)]

use super::mesh::*;

/// The 12 icosahedron vertices and 20 faces.
fn icosahedron() -> (Vec<V3>, Vec<[usize; 3]>) {
    let p = (1.0 + 5.0f64.sqrt()) / 2.0;
    let mut vs: Vec<V3> = Vec::new();
    for &s1 in &[-1.0f64, 1.0] {
        for &s2 in &[-1.0f64, 1.0] {
            vs.push(unit([0.0, s1, s2 * p]));
            vs.push(unit([s1, s2 * p, 0.0]));
            vs.push(unit([s2 * p, 0.0, s1]));
        }
    }
    assert_eq!(vs.len(), 12);
    // edges are the 30 closest pairs; faces are the triples that are mutually adjacent
    let mut best = f64::MAX;
    for i in 0..12 {
        for j in i + 1..12 {
            let d = norm(sub(vs[i], vs[j]));
            if d < best {
                best = d;
            }
        }
    }
    let adj = |i: usize, j: usize| norm(sub(vs[i], vs[j])) < best * 1.1;
    let mut faces = Vec::new();
    for i in 0..12 {
        for j in i + 1..12 {
            for k in j + 1..12 {
                if adj(i, j) && adj(j, k) && adj(i, k) {
                    faces.push([i, j, k]);
                }
            }
        }
    }
    assert_eq!(faces.len(), 20, "icosahedron must have 20 faces, got {}", faces.len());
    (vs, faces)
}

/// Class-I geodesic subdivision: each icosa face → `f²` triangles, projected radially.
/// Returns (vertices, triangles, per-triangle icosa-face id).
pub fn geodesic(freq: usize) -> (Vec<V3>, Vec<[u32; 3]>, Vec<u32>) {
    let (iv, ifaces) = icosahedron();
    let mut dd = VertDedup::new();
    let mut tris: Vec<[u32; 3]> = Vec::new();
    let mut tface: Vec<u32> = Vec::new();
    let f = freq;
    for (fi, &[ia, ib, ic]) in ifaces.iter().enumerate() {
        let (a, b, c) = (iv[ia], iv[ib], iv[ic]);
        // v(r, s): r rows from a down to the bc edge; s across
        let v = |r: usize, s: usize, dd: &mut VertDedup| -> u32 {
            let (wa, wb, wc) = ((f - r) as f64, (r - s) as f64, s as f64);
            dd.get(add(add(scale(a, wa), scale(b, wb)), scale(c, wc)))
        };
        for r in 0..f {
            for s in 0..=r {
                let t = [v(r, s, &mut dd), v(r + 1, s, &mut dd), v(r + 1, s + 1, &mut dd)];
                tris.push(t);
                tface.push(fi as u32);
            }
            for s in 0..r {
                let t = [v(r, s, &mut dd), v(r + 1, s + 1, &mut dd), v(r, s + 1, &mut dd)];
                tris.push(t);
                tface.push(fi as u32);
            }
        }
    }
    let verts = dd.verts;
    assert_eq!(tris.len(), 20 * freq * freq);
    assert_eq!(verts.len(), 10 * freq * freq + 2, "geodesic vertex count must be 10f²+2");
    (verts, tris, tface)
}

/// Circumcentre of a spherical triangle — the point equidistant from all three, i.e.
/// the Voronoi vertex of the three generators.
fn circum(a: V3, b: V3, c: V3) -> V3 {
    let n = unit(cross(sub(b, a), sub(c, a)));
    if dot(n, a) < 0.0 { scale(n, -1.0) } else { n }
}

/// The geodesic **triangle** grid — quadtrees natively (1 → 4), 12 valence-5 vertices.
pub fn icosa_tri(freq: usize, radius_m: f64) -> Mesh {
    let (verts, tris, tface) = geodesic(freq);
    let rings: Vec<Vec<u32>> = tris.iter().map(|t| t.to_vec()).collect();
    let centers: Vec<V3> = tris
        .iter()
        .map(|t| unit(add(add(verts[t[0] as usize], verts[t[1] as usize]), verts[t[2] as usize])))
        .collect();
    let mut rings = rings;
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
    Mesh::build(
        "icosa-tri (geodesic)",
        "20·f² spherical triangles; 12 valence-5 vertices; quadtrees natively",
        radius_m,
        verts,
        rings,
        centers,
        areas,
        tface,
        20,
        "YES — 1→4, native (triangle → 4 triangles)",
    )
}

/// The **hexagonal Voronoi dual** (MPAS-style). `lloyd` = number of centroidal (SCVT)
/// iterations; 0 = the raw geodesic dual.
pub fn icosa_hex(freq: usize, lloyd: usize, radius_m: f64) -> Mesh {
    let (mut gen, tris, _) = geodesic(freq);

    // primal vertex -> incident triangles
    let nv = gen.len();
    let mut inc: Vec<Vec<usize>> = vec![Vec::new(); nv];
    for (ti, t) in tris.iter().enumerate() {
        for &v in t {
            inc[v as usize].push(ti);
        }
    }

    // Lloyd/SCVT: move each generator to its Voronoi cell's centroid. Topology is held
    // fixed — legitimate because an icosahedral Delaunay triangulation is stable under
    // these small moves, and the metrics would scream (non-orthogonality) if it weren't.
    for _ in 0..lloyd {
        let cc: Vec<V3> = tris
            .iter()
            .map(|t| circum(gen[t[0] as usize], gen[t[1] as usize], gen[t[2] as usize]))
            .collect();
        let mut next = gen.clone();
        for v in 0..nv {
            let ring = order_fan(&gen, &cc, &inc[v], v);
            // area-weighted centroid of the spherical polygon
            let mut acc = [0.0f64; 3];
            for k in 1..ring.len() - 1 {
                let (a, b, c) = (cc[ring[0]], cc[ring[k]], cc[ring[k + 1]]);
                let w = tri_area(a, b, c);
                acc = add(acc, scale(unit(add(add(a, b), c)), w));
            }
            next[v] = unit(acc);
        }
        gen = next;
    }

    let cc: Vec<V3> = tris
        .iter()
        .map(|t| circum(gen[t[0] as usize], gen[t[1] as usize], gen[t[2] as usize]))
        .collect();

    let mut rings: Vec<Vec<u32>> = Vec::with_capacity(nv);
    for v in 0..nv {
        let fan = order_fan(&gen, &cc, &inc[v], v);
        rings.push(fan.iter().map(|&t| t as u32).collect());
    }
    let centers = gen.clone();
    for (c, r) in rings.iter_mut().enumerate() {
        let (a, b, d) = (cc[r[0] as usize], cc[r[1] as usize], cc[r[2] as usize]);
        if dot(cross(sub(b, a), sub(d, a)), centers[c]) < 0.0 {
            r.reverse();
        }
    }
    let areas: Vec<f64> = rings
        .iter()
        .map(|r| poly_area(&r.iter().map(|&v| cc[v as usize]).collect::<Vec<_>>()) * radius_m * radius_m)
        .collect();

    // partition by nearest icosahedron face centre — the natural coarse block
    let (iv, ifaces) = icosahedron();
    let fc: Vec<V3> = ifaces.iter().map(|&[a, b, c]| unit(add(add(iv[a], iv[b]), iv[c]))).collect();
    let part: Vec<u32> = centers
        .iter()
        .map(|&p| {
            let mut best = (0usize, -2.0f64);
            for (k, &c) in fc.iter().enumerate() {
                let d = dot(p, c);
                if d > best.1 {
                    best = (k, d);
                }
            }
            best.0 as u32
        })
        .collect();

    let (name, blurb, quad) = if lloyd == 0 {
        (
            "icosa-hex (raw dual)".to_string(),
            "Voronoi dual of the geodesic mesh: hexagons + exactly 12 pentagons".to_string(),
            "NO — a hexagon does not subdivide into 4 hexagons",
        )
    } else {
        (
            format!("icosa-hex (SCVT, {lloyd} Lloyd)"),
            "the same dual, made centroidal — this is what MPAS actually runs on".to_string(),
            "NO — a hexagon does not subdivide into 4 hexagons",
        )
    };
    Mesh::build(&name, &blurb, radius_m, cc, rings, centers, areas, part, 20, quad)
}

/// Order the triangles around a primal vertex into a CCW fan (the dual cell's corners).
fn order_fan(gen: &[V3], cc: &[V3], inc: &[usize], v: usize) -> Vec<usize> {
    let p = gen[v];
    let e1 = tangent(p, sub(cc[inc[0]], p));
    let e2 = cross(p, e1);
    let mut out: Vec<(f64, usize)> = inc
        .iter()
        .map(|&t| {
            let d = tangent(p, sub(cc[t], p));
            (dot(d, e2).atan2(dot(d, e1)), t)
        })
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out.into_iter().map(|(_, t)| t).collect()
}
