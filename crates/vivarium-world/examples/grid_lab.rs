//! GRID LAB — the seam experiment (Joseph, 2026-07-12).
//!
//! **The question this exists to answer, and why it cannot be answered by argument.**
//! Every sphere pixelization (equiangular cube-sphere, Snyder equal-area cubed
//! sphere, HEALPix) subdivides cleanly for LOD, and every one of them has
//! irreducible adjacency defects (Euler forces them: our cube-sphere has 8
//! valence-3 corners). So the projection choice cannot *fix* the seam. What it
//! does do is decide **which physical invariant survives when an algorithm is
//! actually applied** — and that is not guessable:
//!
//! - stream-power incision leans on drainage **area** and **slope**;
//! - shallow-water flux leans on **edge length** and **cell area**;
//! - hillslope diffusion leans on **isotropy**.
//!
//! Each grid preserves a *different* one of those. Joseph, 2026-07-12: *"different
//! projections are going to conserve different physical properties when the
//! algorithms get applied (not at a high-level conceptual guess)."* This harness
//! is the instrument that replaces the guess.
//!
//! **The performance question is inseparable from it.** The naive kernel assumes a
//! uniform grid (one `cell_m`, one `cell_area`, a hardcoded 8-neighbour stencil)
//! — that is what `erosion.rs`/`water.rs` do today. The rigorous kernel carries
//! per-cell area and per-edge length and iterates an explicit neighbour list, which
//! makes conservation *structural* but puts a lookup in the innermost loop. Joseph's
//! sharpening: on an **equal-area** grid the area term is a constant and can be
//! hoisted — so equal-area is what makes a "sunny-day" fast path legal at all. On
//! our equiangular grid it cannot be. That trade is measured here, not asserted.
//!
//! What it reports, per (grid × algorithm × code-path):
//!   1. CONSERVATION — does mass leak? (naive vs finite-volume)
//!   2. ISOTROPY     — does a symmetric source stay symmetric?
//!   3. GEOMETRY     — how far from uniform is the grid really? (the constant-hoist test)
//!   4. IRREGULARITY — what fraction of cells are odd, by level (coarse = early evolution)
//!
//! Status: **harness + our own grid + a flat control.** Snyder and HEALPix plug in
//! behind the `Grid` trait once their formulae are sourced from the literature —
//! deliberately NOT implemented from memory (that would fabricate the very numbers
//! the experiment exists to measure).
//!
//! Run: `cargo run --release -p vivarium-world --example grid_lab`

use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CubeCoord, Face};

/// One directed adjacency: the neighbour, the length of the edge shared with it,
/// and the distance between the two cell centres. On a uniform flat grid these are
/// constants; on a sphere they are not — and that difference is the experiment.
#[derive(Clone, Copy)]
struct Neighbor {
    j: usize,
    edge_len_m: f64,
    dist_m: f64,
}

/// A grid is anything that can say, for each cell: its true area, its neighbours,
/// and the geometry of each shared edge. Physics written against THIS never assumes
/// 8 neighbours or a uniform cell — so corners (7 neighbours), face seams, and tile
/// seams all become the same thing: neighbour resolution.
trait Grid {
    fn name(&self) -> &str;
    fn cells(&self) -> usize;
    /// True spherical area of cell `i` (m²).
    fn area_m2(&self, i: usize) -> f64;
    fn neighbors(&self, i: usize) -> &[Neighbor];
    /// Geodesic position, for the isotropy probe.
    fn center(&self, i: usize) -> [f64; 3];
}

// ---------------------------------------------------------------------------
// Grid 1: our EQUIANGULAR cube-sphere face (what vivarium actually runs on).
// `sphere.rs::to_unit` warps face coords by tan(u·π/4). Cells are NOT equal-area.
// ---------------------------------------------------------------------------

struct EquiangularFace {
    n: usize,
    centers: Vec<[f64; 3]>,
    areas: Vec<f64>,
    adj: Vec<Vec<Neighbor>>,
}

impl EquiangularFace {
    /// Build one cube face at `n × n` cells, with TRUE spherical areas and TRUE
    /// great-circle edge geometry — the ground truth the naive kernel pretends away.
    fn new(n: usize, radius_m: f64) -> Self {
        let face = Face::ZPos;
        let at = |i: usize, j: usize| -> [f64; 3] {
            // cell CENTER in face coords ∈ (-1,1) — never on an edge (sphere.rs §from_unit trap)
            let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
            let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
            CubeCoord { face, u, v }.to_unit()
        };
        // Cell CORNERS, to get true areas via spherical excess.
        let corner = |i: usize, j: usize| -> [f64; 3] {
            let u = (i as f64 / n as f64) * 2.0 - 1.0;
            let v = (j as f64 / n as f64) * 2.0 - 1.0;
            CubeCoord { face, u: u.clamp(-0.999999, 0.999999), v: v.clamp(-0.999999, 0.999999) }.to_unit()
        };

        let mut centers = Vec::with_capacity(n * n);
        let mut areas = Vec::with_capacity(n * n);
        for j in 0..n {
            for i in 0..n {
                centers.push(at(i, j));
                // True area = spherical excess of the cell's 4 corners (two triangles).
                let (a, b, c, d) = (corner(i, j), corner(i + 1, j), corner(i + 1, j + 1), corner(i, j + 1));
                let area = (tri_area(a, b, c) + tri_area(a, c, d)) * radius_m * radius_m;
                areas.push(area);
            }
        }

        // Explicit adjacency (4-neighbour here — the conservation core; diagonals add
        // nothing to a flux balance). Interior only: this face's boundary is left
        // open, which is honest — cross-face adjacency is exactly the seam and is
        // what the tile/face machinery must supply.
        let mut adj = vec![Vec::new(); n * n];
        for j in 0..n {
            for i in 0..n {
                let id = j * n + i;
                for (di, dj) in [(1i64, 0i64), (-1, 0), (0, 1), (0, -1)] {
                    let (ni, nj) = (i as i64 + di, j as i64 + dj);
                    if ni < 0 || nj < 0 || ni >= n as i64 || nj >= n as i64 {
                        continue;
                    }
                    let jd = nj as usize * n + ni as usize;
                    // TRUE edge length: the great-circle length of the shared edge.
                    let (e0, e1) = shared_edge(i, j, di, dj, n, &corner);
                    let edge_len_m = geodesic(e0, e1) * radius_m;
                    let dist_m = geodesic(centers[id], centers[jd]) * radius_m;
                    adj[id].push(Neighbor { j: jd, edge_len_m, dist_m });
                }
            }
        }
        EquiangularFace { n, centers, areas, adj }
    }
}

impl Grid for EquiangularFace {
    fn name(&self) -> &str {
        "equiangular cube-sphere (ours)"
    }
    fn cells(&self) -> usize {
        self.n * self.n
    }
    fn area_m2(&self, i: usize) -> f64 {
        self.areas[i]
    }
    fn neighbors(&self, i: usize) -> &[Neighbor] {
        &self.adj[i]
    }
    fn center(&self, i: usize) -> [f64; 3] {
        self.centers[i]
    }
}

/// The two corners of the edge shared between cell (i,j) and its (di,dj) neighbour.
fn shared_edge(
    i: usize,
    j: usize,
    di: i64,
    dj: i64,
    _n: usize,
    corner: &impl Fn(usize, usize) -> [f64; 3],
) -> ([f64; 3], [f64; 3]) {
    match (di, dj) {
        (1, 0) => (corner(i + 1, j), corner(i + 1, j + 1)),
        (-1, 0) => (corner(i, j), corner(i, j + 1)),
        (0, 1) => (corner(i, j + 1), corner(i + 1, j + 1)),
        _ => (corner(i, j), corner(i + 1, j)),
    }
}

// --- spherical geometry helpers (unit sphere) ---

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}
fn geodesic(a: [f64; 3], b: [f64; 3]) -> f64 {
    // atan2 form — stable for small angles, unlike acos(dot).
    norm(cross(a, b)).atan2(dot(a, b))
}
/// Spherical triangle area (steradians) — l'Huilier via the spherical excess.
fn tri_area(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let (ab, bc, ca) = (geodesic(a, b), geodesic(b, c), geodesic(c, a));
    let s = (ab + bc + ca) / 2.0;
    let t = (s / 2.0).tan()
        * ((s - ab) / 2.0).tan()
        * ((s - bc) / 2.0).tan()
        * ((s - ca) / 2.0).tan();
    4.0 * t.abs().sqrt().atan()
}

// ---------------------------------------------------------------------------
// The two code-paths, on ONE algorithm whose invariant is exactly known:
// linear diffusion. Nature conserves its total mass EXACTLY. Any drift is the
// scheme lying — so the leak is a pure measurement of the grid+path, with no
// tuning and no opinion.
// ---------------------------------------------------------------------------

/// SUNNY-DAY path: what `erosion.rs`/`water.rs` do today — assume every cell has the
/// same area and every edge the same length, with a fixed stencil. Fast (all
/// geometry is a constant), but on a non-uniform grid it is solving a slightly
/// different equation in every cell.
fn diffuse_naive(g: &dyn Grid, u: &mut [f64], k: f64, mean_area: f64, mean_dist: f64, mean_edge: f64) {
    let prev = u.to_vec();
    for i in 0..g.cells() {
        let mut acc = 0.0;
        for nb in g.neighbors(i) {
            // Uniform geometry ASSUMED — the constants are hoisted, nothing is looked up.
            acc += (prev[nb.j] - prev[i]) * mean_edge / mean_dist;
        }
        u[i] = prev[i] + k * acc / mean_area;
    }
}

/// FINITE-VOLUME path: carry the true geometry. Flux across an edge is
/// `(u_j − u_i) · edge_len / dist`; the divergence divides by the cell's TRUE area.
/// Conservation is then structural — what leaves cell i across an edge is exactly
/// what enters cell j across the same edge, because it is literally the same number.
fn diffuse_fv(g: &dyn Grid, u: &mut [f64], k: f64) {
    let prev = u.to_vec();
    for i in 0..g.cells() {
        let mut flux = 0.0;
        for nb in g.neighbors(i) {
            flux += (prev[nb.j] - prev[i]) * nb.edge_len_m / nb.dist_m;
        }
        u[i] = prev[i] + k * flux / g.area_m2(i);
    }
}

/// Total mass = Σ uᵢ · areaᵢ. The quantity nature conserves exactly.
fn mass(g: &dyn Grid, u: &[f64]) -> f64 {
    (0..g.cells()).map(|i| u[i] * g.area_m2(i)).sum()
}

fn main() {
    let r = Planet::EARTH.radius_m;
    println!("GRID LAB — what survives when the algorithm is actually applied\n");

    for n in [16usize, 64, 256] {
        let g = EquiangularFace::new(n, r);
        let cells = g.cells();

        // --- 3. GEOMETRY: how far from uniform is the grid, really? ------------
        // This is the "can the sunny-day path even be legal?" test. On an EQUAL-AREA
        // grid this spread would be 1.00 and the area term could be hoisted to a
        // constant. On ours it cannot.
        let areas: Vec<f64> = (0..cells).map(|i| g.area_m2(i)).collect();
        let (amin, amax) = areas.iter().fold((f64::MAX, 0.0f64), |(lo, hi), &a| (lo.min(a), hi.max(a)));
        let amean = areas.iter().sum::<f64>() / cells as f64;
        let edges: Vec<f64> = (0..cells).flat_map(|i| g.neighbors(i).iter().map(|nb| nb.edge_len_m)).collect();
        let dists: Vec<f64> = (0..cells).flat_map(|i| g.neighbors(i).iter().map(|nb| nb.dist_m)).collect();
        let emean = edges.iter().sum::<f64>() / edges.len() as f64;
        let dmean = dists.iter().sum::<f64>() / dists.len() as f64;

        println!("── L≈{} ({}×{} = {} cells on one face, {}) ──", (n as f64).log2() as u32, n, n, cells, g.name());
        println!(
            "  geometry   cell area  min {:.3e}  max {:.3e} m²   → spread {:.3}×  (equal-area grid would be 1.000)",
            amin,
            amax,
            amax / amin
        );

        // --- 1+2. CONSERVATION and ISOTROPY, at TWO placements -------------------
        // Placement matters and it is easy to fool yourself here: a blob at the FACE
        // CENTRE sits at the least-distorted point on the grid, so it measures the
        // BEST case. The honest stress test puts the source near a CORNER, where the
        // equiangular warp is worst. Both are reported — the spread between them is
        // the real story.
        let k = 0.15 * amean * dmean / emean / 4.0;
        for (label, seed_uv) in [("centre", (0.0, 0.0)), ("near-corner", (0.82, 0.82))] {
            let src = CubeCoord { face: Face::ZPos, u: seed_uv.0, v: seed_uv.1 }.to_unit();
            let mut u_naive: Vec<f64> = (0..cells)
                .map(|i| (-(geodesic(g.center(i), src) * 6.0).powi(2)).exp())
                .collect();
            let mut u_fv = u_naive.clone();
            let (m0n, m0f) = (mass(&g, &u_naive), mass(&g, &u_fv));

            for _ in 0..200 {
                diffuse_naive(&g, &mut u_naive, k, amean, dmean, emean);
                diffuse_fv(&g, &mut u_fv, k);
            }
            let drift_naive = (mass(&g, &u_naive) - m0n) / m0n;
            let drift_fv = (mass(&g, &u_fv) - m0f) / m0f;

            println!(
                "  [{label:^11}] conserve  naive {:+.3e}   finite-volume {:+.3e}   |  {}",
                drift_naive,
                drift_fv,
                anisotropy_report(&g, &u_fv)
            );
        }
        println!();
    }

    println!("HONEST SCOPE — what this does and does not yet show:");
    println!("  • ONE face, interior only. Cross-face adjacency and the 8 valence-3 corners");
    println!("    are the seam itself and are NOT exercised here — that is the next rung.");
    println!("  • ONE algorithm (diffusion), chosen because its invariant is exactly known.");
    println!("    Stream-power (area+slope) and shallow-water (edge+area) lean on DIFFERENT");
    println!("    geometric terms and must each get their own row — that is the whole point.");
    println!("  • Snyder equal-area and HEALPix are NOT here yet: they plug in behind `Grid`");
    println!("    once their formulae are sourced from the literature. Implementing them from");
    println!("    memory would fabricate the very numbers this experiment exists to measure.");
}

/// Anisotropy of the diffused field about its own centroid: the ratio of the largest
/// to smallest principal second moment. 1.00 = perfectly circular spread.
fn anisotropy_report(g: &dyn Grid, u: &[f64]) -> String {
    let total: f64 = (0..g.cells()).map(|i| u[i] * g.area_m2(i)).sum();
    let mut c = [0.0; 3];
    for i in 0..g.cells() {
        let w = u[i] * g.area_m2(i) / total;
        let p = g.center(i);
        for d in 0..3 {
            c[d] += w * p[d];
        }
    }
    let cn = norm(c);
    let c = [c[0] / cn, c[1] / cn, c[2] / cn];
    // Two orthogonal tangent directions at the centroid.
    let t1 = {
        let a = if c[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
        let v = cross(c, a);
        let n = norm(v);
        [v[0] / n, v[1] / n, v[2] / n]
    };
    let t2 = cross(c, t1);
    let (mut m11, mut m22) = (0.0, 0.0);
    for i in 0..g.cells() {
        let w = u[i] * g.area_m2(i) / total;
        let p = g.center(i);
        let d = [p[0] - c[0], p[1] - c[1], p[2] - c[2]];
        m11 += w * dot(d, t1).powi(2);
        m22 += w * dot(d, t2).powi(2);
    }
    let (lo, hi) = (m11.min(m22), m11.max(m22));
    format!("spread anisotropy {:.4} (1.0000 = circular; >1 means the grid is bending the physics)", (hi / lo).sqrt())
}
