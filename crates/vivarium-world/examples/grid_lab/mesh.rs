//! The universal mesh: what every grid becomes once it is built.
//!
//! A `Mesh` is a set of spherical polygons with **combinatorial** adjacency derived
//! from shared vertices — so a cube-sphere corner, a HEALPix base-pixel junction and
//! an icosahedral pentagon are all the *same kind of thing* to everything downstream.
//! Every mesh is checked against Euler's formula (`V − E + F = 2`) at construction, so
//! a topology bug cannot silently become a physics result.

#![allow(dead_code)]

pub type V3 = [f64; 3];

pub fn dot(a: V3, b: V3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
pub fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
pub fn norm(a: V3) -> f64 { dot(a, a).sqrt() }
pub fn unit(a: V3) -> V3 { let n = norm(a); [a[0] / n, a[1] / n, a[2] / n] }
pub fn add(a: V3, b: V3) -> V3 { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
pub fn sub(a: V3, b: V3) -> V3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
pub fn scale(a: V3, s: f64) -> V3 { [a[0] * s, a[1] * s, a[2] * s] }

/// Great-circle angle (radians). `atan2` form — stable at small angles.
pub fn geodesic(a: V3, b: V3) -> f64 { norm(cross(a, b)).atan2(dot(a, b)) }

/// Spherical triangle area (steradians), l'Huilier.
pub fn tri_area(a: V3, b: V3, c: V3) -> f64 {
    let (ab, bc, ca) = (geodesic(a, b), geodesic(b, c), geodesic(c, a));
    let s = (ab + bc + ca) / 2.0;
    let t = (s / 2.0).tan() * ((s - ab) / 2.0).tan() * ((s - bc) / 2.0).tan() * ((s - ca) / 2.0).tan();
    4.0 * t.abs().sqrt().atan()
}

/// Spherical polygon area (steradians) by fanning from vertex 0.
pub fn poly_area(vs: &[V3]) -> f64 {
    (1..vs.len() - 1).map(|k| tri_area(vs[0], vs[k], vs[k + 1])).sum()
}

/// Project `v` into the tangent plane at `p` and normalize.
pub fn tangent(p: V3, v: V3) -> V3 { unit(sub(v, scale(p, dot(p, v)))) }

/// Unit tangent at `a` pointing along the great circle toward `b`.
pub fn bearing(a: V3, b: V3) -> V3 { tangent(a, b) }

// ---------------------------------------------------------------------------

/// One directed adjacency — the *joint* geometry of the pair, which is what a flux
/// across the shared edge actually needs. Joseph, 2026-07-12: *"the stencil, not the
/// cell"*: my arm, j's arm, the shared edge, the angle.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub j: usize,
    /// Great-circle length of the shared edge (m).
    pub edge_len_m: f64,
    /// Centre-to-centre great-circle distance (m). NOT the FV gradient arm.
    pub dist_m: f64,
    /// **Centre-to-mid-edge arm** (m) — mine. The correct FV gradient arm.
    pub arm_m: f64,
    /// The neighbour's arm to the same mid-edge (m).
    pub arm_opp_m: f64,
    /// My arm projected onto the edge NORMAL (m) — the orthogonal part, which is
    /// what a two-point flux may legitimately use. `arm_m` minus this is the part a
    /// non-orthogonality correction would have to carry.
    pub arm_normal_m: f64,
    pub arm_normal_opp_m: f64,
    /// **True non-orthogonality** (degrees): `90° −` the dihedral angle between the shared
    /// edge's great circle and the centre-line's great circle, measured AT THEIR CROSSING.
    /// 0 = the centre-line pierces the edge at a right angle. This is Putman & Lin's
    /// `sin α` factor. It is exactly 0 on any Voronoi mesh, by duality — a falsifiable
    /// prediction the numbers check.
    pub nonortho_deg: f64,
    /// **Skewness**: the distance from the mid-edge to the point where the centre-line
    /// actually crosses the edge, as a fraction of the edge length. A two-point flux
    /// evaluates the gradient at the crossing but applies it over the whole edge; when
    /// these differ, that is the error. ORTHOGONAL AND SKEW ARE INDEPENDENT — a hexagonal
    /// Voronoi mesh is perfectly orthogonal and still skew, which is precisely why the
    /// mid-edge arm (not the centre-to-centre line) is the right FV gradient arm.
    pub skew: f64,
    /// Outward unit normal to the shared edge, in the tangent plane at the mid-edge.
    pub normal: V3,
    /// The two endpoints of the shared edge.
    pub va: u32,
    pub vb: u32,
}

/// Everything a grid must be able to say about itself.
pub struct Mesh {
    pub name: String,
    /// One-line statement of what it *is*, for the report.
    pub blurb: String,
    pub radius_m: f64,
    pub verts: Vec<V3>,
    /// Cell corner rings, CCW (outward).
    pub rings: Vec<Vec<u32>>,
    pub centers: Vec<V3>,
    /// The area the SCHEME uses (m²). For Snyder and HEALPix this is the exact analytic
    /// equal-area value; for every other grid the cell boundaries ARE great circles, so
    /// it is the spherical excess and the two agree by construction.
    pub areas: Vec<f64>,
    /// The spherical excess of the geodesic polygon through the cell's corners — i.e. the
    /// area of the cell a finite-volume code would actually BUILD (Putman & Lin: *"the
    /// cell edges for all grids are prescribed to be great circle arcs"*). Where this
    /// differs from `areas`, the grid's equal-area guarantee is a property of the
    /// continuous MAP that the discrete cell does not inherit.
    pub areas_geodesic: Vec<f64>,
    /// Edge (flux-carrying) adjacency — the ONLY adjacency a finite volume needs.
    pub adj: Vec<Vec<Edge>>,
    /// Moore adjacency (shares ≥1 vertex) — what an 8-neighbour MFD fan needs, and
    /// where the valence-3 pathology actually lives.
    pub moore: Vec<Vec<usize>>,
    /// Vertex → incident cells.
    pub vcells: Vec<Vec<usize>>,
    /// Natural coarse partition of each cell (cube face / HEALPix base pixel /
    /// icosa face) — the unit a parallel sweep would own.
    pub part: Vec<u32>,
    pub nparts: usize,
    /// Does this grid subdivide 1→4 (the property `CellId` rests on)?
    pub quadtree: &'static str,
    /// Euler check result, recorded so the report can cite it.
    pub euler: i64,
}

impl Mesh {
    pub fn cells(&self) -> usize { self.centers.len() }

    /// Build from vertices + CCW corner rings. Adjacency is derived from **shared
    /// vertex pairs** — no geometric matching, no tolerance, for meshes whose
    /// vertices are already deduplicated by construction or by exact key.
    pub fn build(
        name: &str,
        blurb: &str,
        radius_m: f64,
        verts: Vec<V3>,
        rings: Vec<Vec<u32>>,
        centers: Vec<V3>,
        areas: Vec<f64>,
        part: Vec<u32>,
        nparts: usize,
        quadtree: &'static str,
    ) -> Mesh {
        let areas_geodesic: Vec<f64> = rings
            .iter()
            .map(|r| {
                let vs: Vec<V3> = r.iter().map(|&v| verts[v as usize]).collect();
                poly_area(&vs) * radius_m * radius_m
            })
            .collect();
        let nc = rings.len();
        let nv = verts.len();

        // vertex -> cells
        let mut vcells: Vec<Vec<usize>> = vec![Vec::new(); nv];
        for (c, r) in rings.iter().enumerate() {
            for &v in r {
                vcells[v as usize].push(c);
            }
        }

        // edge (unordered vertex pair) -> the (≤2) cells that carry it
        use std::collections::HashMap;
        let mut emap: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
        for (c, r) in rings.iter().enumerate() {
            for k in 0..r.len() {
                let (a, b) = (r[k], r[(k + 1) % r.len()]);
                let key = if a < b { (a, b) } else { (b, a) };
                emap.entry(key).or_default().push(c);
            }
        }
        let mut nedges = 0usize;
        for (k, cs) in &emap {
            assert!(cs.len() == 2, "{name}: edge {k:?} borders {} cells (must be 2)", cs.len());
            nedges += 1;
        }

        let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); nc];
        for (&(va, vb), cs) in &emap {
            let (a, b) = (verts[va as usize], verts[vb as usize]);
            let mid = unit(add(a, b));
            let edge_len_m = geodesic(a, b) * radius_m;
            // Edge normal at the mid-edge: the pole of the edge's great circle is
            // perpendicular to the edge and tangent at the mid-edge.
            let n0 = unit(cross(a, b));
            let n0 = tangent(mid, n0); // (already tangent; re-project for float hygiene)
            for &(i, j) in &[(cs[0], cs[1]), (cs[1], cs[0])] {
                let (ci, cj) = (centers[i], centers[j]);
                // orient the normal outward from i
                let sgn = if dot(n0, tangent(mid, cj)) >= 0.0 { 1.0 } else { -1.0 };
                let nrm = scale(n0, sgn);
                let arm_m = geodesic(ci, mid) * radius_m;
                let arm_opp_m = geodesic(cj, mid) * radius_m;
                let dist_m = geodesic(ci, cj) * radius_m;
                // arm projected on the normal: |arm| · cos(angle between the arm's
                // bearing AT THE MID-EDGE and the normal)
                let arm_dir_i = tangent(mid, ci); // points back at i
                let arm_dir_j = tangent(mid, cj);
                let arm_normal_m = arm_m * (-dot(arm_dir_i, nrm)).max(0.0);
                let arm_normal_opp_m = arm_opp_m * dot(arm_dir_j, nrm).max(0.0);
                // TRUE non-orthogonality: the dihedral angle between the two great
                // circles (edge, centre-line) equals the angle between their poles.
                let pole_e = unit(cross(a, b));
                let pole_c = unit(cross(ci, cj));
                let between = dot(pole_e, pole_c).abs().clamp(0.0, 1.0).acos().to_degrees();
                let ang = (90.0 - between).abs();
                // SKEW: where does the centre-line actually cross the edge?
                let xing = {
                    let x = unit(cross(pole_e, pole_c));
                    if dot(x, mid) >= 0.0 { x } else { scale(x, -1.0) }
                };
                let skew = if edge_len_m > 0.0 {
                    geodesic(mid, xing) * radius_m / edge_len_m
                } else {
                    0.0
                };
                adj[i].push(Edge {
                    j,
                    edge_len_m,
                    dist_m,
                    arm_m,
                    arm_opp_m,
                    arm_normal_m,
                    arm_normal_opp_m,
                    nonortho_deg: ang,
                    skew,
                    normal: nrm,
                    va,
                    vb,
                });
            }
        }

        // Moore adjacency: share ≥ 1 vertex.
        let mut moore: Vec<Vec<usize>> = vec![Vec::new(); nc];
        for i in 0..nc {
            let mut s: Vec<usize> = Vec::new();
            for &v in &rings[i] {
                for &c in &vcells[v as usize] {
                    if c != i {
                        s.push(c);
                    }
                }
            }
            s.sort_unstable();
            s.dedup();
            moore[i] = s;
        }

        let euler = nv as i64 - nedges as i64 + nc as i64;
        assert_eq!(euler, 2, "{name}: Euler characteristic is {euler}, not 2 — topology is wrong");

        Mesh {
            name: name.to_string(),
            blurb: blurb.to_string(),
            radius_m,
            verts,
            rings,
            centers,
            areas,
            areas_geodesic,
            adj,
            moore,
            vcells,
            part,
            nparts,
            quadtree,
            euler,
        }
    }

    /// Total mass of a per-cell density: `Σ uᵢ · areaᵢ`. What nature conserves exactly.
    pub fn mass(&self, u: &[f64]) -> f64 {
        (0..self.cells()).map(|i| u[i] * self.areas[i]).sum()
    }
    pub fn total_area(&self) -> f64 { self.areas.iter().sum() }
}

/// Rebuild a mesh with each cell's centre moved to its true **spherical centroid**.
///
/// This exists because of a measurement, not a theory. A finite-volume scheme evolves the
/// cell AVERAGE of `u`, but every scheme here reads `u` at the cell's "centre". If the
/// centre is not the centroid, `u(centre) ≠ ū` by `∇u·δ`, and that error does not vanish
/// under refinement fast enough — it caps the convergence order no matter how good the
/// flux is. Our quad grids take the cell centre to be the PARAMETER-SPACE midpoint (which
/// is what `sphere.rs::CellId::to_cube()` returns), and on a distorted cell that is NOT the
/// centroid. An SCVT hexagon, by contrast, has its generator AT the centroid by definition
/// — which turns out to be a large part of why it converges and our grid does not.
pub fn recentered(g: &Mesh) -> Mesh {
    let centers: Vec<V3> = (0..g.cells())
        .map(|i| {
            let vs: Vec<V3> = g.rings[i].iter().map(|&v| g.verts[v as usize]).collect();
            let mut acc = [0.0f64; 3];
            for k in 1..vs.len() - 1 {
                let (a, b, c) = (vs[0], vs[k], vs[k + 1]);
                let w = tri_area(a, b, c);
                acc = add(acc, scale(unit(add(add(a, b), c)), w));
            }
            unit(acc)
        })
        .collect();
    Mesh::build(
        &format!("{} +centroid", g.name),
        &g.blurb,
        g.radius_m,
        g.verts.clone(),
        g.rings.clone(),
        centers,
        g.areas.clone(),
        g.part.clone(),
        g.nparts,
        g.quadtree,
    )
}

/// Exact integer vertex key, deduplicated. Used where vertices are constructed by
/// float paths that must nonetheless coincide (HEALPix caps meeting the belt). The
/// quantum is far below the minimum vertex separation at every resolution we run, and
/// the Euler check in `Mesh::build` is what actually proves the dedup was right.
pub struct VertDedup {
    map: std::collections::HashMap<(i64, i64, i64), u32>,
    pub verts: Vec<V3>,
    q: f64,
}

impl VertDedup {
    pub fn new() -> Self {
        VertDedup { map: std::collections::HashMap::new(), verts: Vec::new(), q: 1e9 }
    }
    pub fn get(&mut self, p: V3) -> u32 {
        let p = unit(p);
        let k = (
            (p[0] * self.q).round() as i64,
            (p[1] * self.q).round() as i64,
            (p[2] * self.q).round() as i64,
        );
        if let Some(&i) = self.map.get(&k) {
            return i;
        }
        let i = self.verts.len() as u32;
        self.verts.push(p);
        self.map.insert(k, i);
        i
    }
}
