//! DRAINAGE — the algorithm with the ordering teeth, and therefore (Joseph, 2026-07-12)
//! possibly the dominant criterion.
//!
//! Diffusion is embarrassingly parallel: every cell reads its neighbours' *previous*
//! values, so a seam is a halo exchange and nothing more. It is the LEAST informative
//! case, and the old harness only had that one.
//!
//! Priority-Flood and drainage accumulation are **globally ORDERED**: elevation-sorted,
//! strictly downstream (`erosion.rs::accumulate_drainage` sorts every cell by `h` and
//! sweeps the sorted order in reverse). At a seam, a cell's *downstream* neighbour can
//! live in another partition — a dependency edge that SERIALISES the sweep. Different
//! grids scatter those crossings differently, and that is what this measures.

#![allow(dead_code)]

use super::mesh::*;

/// A deterministic, band-limited, **grid-independent** terrain: a sum of plane waves on
/// the sphere. Every grid is handed the SAME physical field and samples it at its own
/// cell centres, so any difference in the flow statistics is the grid's doing, not the
/// terrain's.
pub fn terrain(p: V3) -> f64 {
    let mut h = 0.0;
    let mut st = 0x9E3779B97F4A7C15u64;
    let mut rnd = || {
        st ^= st << 13;
        st ^= st >> 7;
        st ^= st << 17;
        (st >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    // 6 octaves, amplitude ∝ 1/k — a red-noise landscape
    for oct in 0..6 {
        let k = 2.0f64.powi(oct) * 2.0;
        let amp = 1.0 / k;
        for _ in 0..8 {
            let d = unit([rnd(), rnd(), rnd()]);
            let ph = rnd() * std::f64::consts::PI;
            h += amp * (k * dot(p, d) + ph).sin();
        }
    }
    h
}

pub struct Sequencing {
    pub nparts: usize,
    /// Cells with at least one edge-neighbour in another partition.
    pub boundary_frac: f64,
    /// Cells whose steepest-descent RECEIVER is in another partition. These are the
    /// dependency edges that serialise a partitioned sweep.
    pub crossing_frac: f64,
    /// Does the partition-level dependency graph have CYCLES? If it does, no ordering of
    /// whole partitions exists and the sweep must synchronise at cell granularity.
    pub partition_graph_cyclic: bool,
    pub largest_scc: usize,
    /// The longest flow path, in cells — the intrinsic serialisation depth of drainage
    /// accumulation on this grid (grid-dependent only through discretisation).
    pub longest_path: usize,
    /// The maximum number of PARTITION TRANSITIONS along any flow path. This is the
    /// number of synchronisation rounds a partitioned accumulation must pay. THE number.
    pub max_partition_transitions: usize,
    pub sinks: usize,
}

/// Steepest-descent receiver over the EDGE neighbours (the real flux neighbours).
pub fn receivers(g: &Mesh, h: &[f64]) -> Vec<usize> {
    (0..g.cells())
        .map(|i| {
            let mut best = (i, 0.0f64);
            for e in &g.adj[i] {
                let s = (h[i] - h[e.j]) / e.dist_m;
                if s > best.1 {
                    best = (e.j, s);
                }
            }
            best.0
        })
        .collect()
}

pub fn sequencing(g: &Mesh) -> Sequencing {
    let h: Vec<f64> = g.centers.iter().map(|&p| terrain(p)).collect();
    let recv = receivers(g, &h);
    let n = g.cells();

    let boundary = (0..n)
        .filter(|&i| g.adj[i].iter().any(|e| g.part[e.j] != g.part[i]))
        .count();
    let crossing = (0..n).filter(|&i| recv[i] != i && g.part[recv[i]] != g.part[i]).count();
    let sinks = (0..n).filter(|&i| recv[i] == i).count();

    // partition dependency graph
    let p = g.nparts;
    let mut pg: Vec<std::collections::HashSet<usize>> = vec![Default::default(); p];
    for i in 0..n {
        if recv[i] != i && g.part[recv[i]] != g.part[i] {
            pg[g.part[i] as usize].insert(g.part[recv[i]] as usize);
        }
    }
    // Tarjan-lite: iterative SCC by Kosaraju
    let mut rev: Vec<Vec<usize>> = vec![Vec::new(); p];
    for a in 0..p {
        for &b in &pg[a] {
            rev[b].push(a);
        }
    }
    let mut order = Vec::new();
    let mut seen = vec![false; p];
    for s in 0..p {
        if seen[s] {
            continue;
        }
        let mut stack = vec![(s, 0usize)];
        seen[s] = true;
        while let Some(&mut (v, ref mut k)) = stack.last_mut() {
            let ns: Vec<usize> = pg[v].iter().cloned().collect();
            if *k < ns.len() {
                let w = ns[*k];
                *k += 1;
                if !seen[w] {
                    seen[w] = true;
                    stack.push((w, 0));
                }
            } else {
                order.push(v);
                stack.pop();
            }
        }
    }
    let mut comp = vec![usize::MAX; p];
    let mut ncomp = 0;
    for &s in order.iter().rev() {
        if comp[s] != usize::MAX {
            continue;
        }
        let mut st = vec![s];
        comp[s] = ncomp;
        while let Some(v) = st.pop() {
            for &w in &rev[v] {
                if comp[w] == usize::MAX {
                    comp[w] = ncomp;
                    st.push(w);
                }
            }
        }
        ncomp += 1;
    }
    let mut csize = vec![0usize; ncomp];
    for &c in &comp {
        csize[c] += 1;
    }
    let largest = csize.iter().cloned().max().unwrap_or(0);
    // a cycle exists iff some SCC has >1 partition, or a partition points at itself
    // (self-loops are excluded by construction: we only record cross-partition edges)
    let cyclic = largest > 1;

    // longest flow path and max partition transitions — memoised walk down the forest
    let mut depth = vec![usize::MAX; n];
    let mut trans = vec![usize::MAX; n];
    for start in 0..n {
        if depth[start] != usize::MAX {
            continue;
        }
        let mut path = Vec::new();
        let mut v = start;
        while depth[v] == usize::MAX && recv[v] != v {
            path.push(v);
            depth[v] = 0; // mark in-progress (the flow graph is a forest: no cycles)
            v = recv[v];
        }
        let (mut d, mut t) = if recv[v] == v && depth[v] == 0 && path.last() != Some(&v) {
            (0, 0)
        } else if depth[v] == usize::MAX {
            (0, 0)
        } else {
            (depth[v], trans[v])
        };
        if recv[v] == v {
            depth[v] = 0;
            trans[v] = 0;
            d = 0;
            t = 0;
        }
        let mut prev = v;
        for &w in path.iter().rev() {
            d += 1;
            if g.part[w] != g.part[prev] {
                t += 1;
            }
            depth[w] = d;
            trans[w] = t;
            prev = w;
        }
    }
    let longest = depth.iter().filter(|&&d| d != usize::MAX).cloned().max().unwrap_or(0);
    let maxt = trans.iter().filter(|&&d| d != usize::MAX).cloned().max().unwrap_or(0);

    Sequencing {
        nparts: p,
        boundary_frac: boundary as f64 / n as f64,
        crossing_frac: crossing as f64 / n as f64,
        partition_graph_cyclic: cyclic,
        largest_scc: largest,
        longest_path: longest,
        max_partition_transitions: maxt,
        sinks,
    }
}

// ===========================================================================
// THE CORNER TEST — is the valence-3 pathology real, or an MFD artifact?
// ===========================================================================
//
// The claim under test (Joseph, 2026-07-12): *"your up-neighbour's left edge is welded to
// your left-neighbour's top edge; the diagonal cell doesn't exist"* is a property of
// **MFD's 8-neighbour fan**, not of the sphere. True finite-volume flux crosses EDGES;
// diagonals share only a vertex — a zero-length edge — so they carry no flux.

#[derive(Clone, Copy, PartialEq)]
pub enum Router {
    /// `erosion.rs` today: an 8-neighbour Moore fan, weights `(drop/dist)^p`, with the
    /// diagonal distance hard-coded to `cell_m·√2`. Here it runs on the TRUE Moore graph,
    /// so at a valence-3 corner it simply finds 7 neighbours instead of 8.
    MooreMfd,
    /// MFD restricted to real EDGES (4 or 6 of them). No diagonals.
    EdgeMfd,
    /// **Gradient-projected edge flux** — D-∞ generalised to a mesh. Fit the local
    /// gradient by least squares, project it onto each real edge normal, weight by edge
    /// LENGTH, take the outgoing components. Needs no diagonals, works at any valence,
    /// conserves, and carries no grid-aligned bias.
    GradEdgeFlux,
}

impl Router {
    pub fn label(self) -> &'static str {
        match self {
            Router::MooreMfd => "8-nbr Moore MFD (status quo)",
            Router::EdgeMfd => "edge MFD (no diagonals)",
            Router::GradEdgeFlux => "gradient-projected edge flux",
        }
    }
}

/// Outgoing routing weights for cell `i` (normalised, summing to 1 unless it is a sink).
fn weights(g: &Mesh, r: Router, h: &[f64], i: usize) -> Vec<(usize, f64)> {
    const P: f64 = 1.1;
    match r {
        Router::MooreMfd => {
            let mut w = Vec::new();
            let mut tot = 0.0;
            // The status quo uses ONE cell size, with diagonals at cell_m·√2. Reproduce
            // that faithfully: the mean centre-distance stands in for `cell_m`, and a
            // Moore neighbour that is not an edge neighbour is treated as a diagonal.
            let cell_m = g.adj[i].iter().map(|e| e.dist_m).sum::<f64>() / g.adj[i].len() as f64;
            for &j in &g.moore[i] {
                let is_edge = g.adj[i].iter().any(|e| e.j == j);
                let dist = if is_edge { cell_m } else { cell_m * std::f64::consts::SQRT_2 };
                let drop = h[i] - h[j];
                if drop > 0.0 {
                    let x = (drop / dist).powf(P);
                    w.push((j, x));
                    tot += x;
                }
            }
            for e in w.iter_mut() {
                e.1 /= tot;
            }
            if tot <= 0.0 { Vec::new() } else { w }
        }
        Router::EdgeMfd => {
            let mut w = Vec::new();
            let mut tot = 0.0;
            for e in &g.adj[i] {
                let drop = h[i] - h[e.j];
                if drop > 0.0 {
                    let x = (drop / e.dist_m).powf(P) * e.edge_len_m;
                    w.push((e.j, x));
                    tot += x;
                }
            }
            for e in w.iter_mut() {
                e.1 /= tot;
            }
            if tot <= 0.0 { Vec::new() } else { w }
        }
        Router::GradEdgeFlux => {
            // least-squares gradient in the tangent plane at the cell centre
            let c = g.centers[i];
            let e0 = tangent(c, g.centers[g.adj[i][0].j]);
            let e1 = cross(c, e0);
            let (mut sxx, mut sxy, mut syy, mut sxz, mut syz) = (0.0, 0.0, 0.0, 0.0, 0.0);
            for e in &g.adj[i] {
                let d = sub(g.centers[e.j], c);
                let (x, y) = (dot(d, e0) * g.radius_m, dot(d, e1) * g.radius_m);
                let z = h[e.j] - h[i];
                sxx += x * x;
                sxy += x * y;
                syy += y * y;
                sxz += x * z;
                syz += y * z;
            }
            let det = sxx * syy - sxy * sxy;
            if det.abs() < 1e-300 {
                return Vec::new();
            }
            let gx = (syy * sxz - sxy * syz) / det;
            let gy = (sxx * syz - sxy * sxz) / det;
            // downslope direction = −∇h
            let mut w = Vec::new();
            let mut tot = 0.0;
            for e in &g.adj[i] {
                // the edge normal, expressed in the cell's tangent frame
                let nrm = tangent(c, add(c, e.normal)); // re-express at the centre
                let nrm = tangent(c, sub(g.centers[e.j], c)); // (outward through this edge)
                let _ = nrm;
                let d = tangent(c, sub(g.centers[e.j], c));
                let (nx, ny) = (dot(d, e0), dot(d, e1));
                let out = -(gx * nx + gy * ny); // outgoing component of −∇h
                if out > 0.0 {
                    let x = out * e.edge_len_m;
                    w.push((e.j, x));
                    tot += x;
                }
            }
            for e in w.iter_mut() {
                e.1 /= tot;
            }
            if tot <= 0.0 { Vec::new() } else { w }
        }
    }
}

pub struct Routing {
    /// Total accumulated ÷ total area. **1.0 exactly = the router conserved.**
    pub conservation: f64,
    /// On a perfect CONE (a surface with exact radial symmetry), the accumulation on a
    /// ring at fixed radius must be azimuthally uniform. This is the coefficient of
    /// variation of that ring — pure grid-aligned bias, with no terrain to hide behind.
    pub cone_bias_cv: f64,
    /// The same measured only over the cells adjacent to a topological defect (a
    /// valence-3 cube corner / HEALPix junction / icosa pentagon).
    pub cone_bias_cv_at_defects: f64,
    pub defect_cells: usize,
}

/// Route drainage over a field, then report conservation and grid bias.
pub fn routing(g: &Mesh, r: Router) -> Routing {
    // A perfect cone about a chosen pole: h = −(angle from the pole). Radially symmetric
    // by construction, so ANY azimuthal structure in the result is the grid talking.
    let pole = unit([0.3, -0.7, 0.64]);
    let h: Vec<f64> = g.centers.iter().map(|&p| -geodesic(p, pole)).collect();

    // elevation-sorted sweep — exactly `erosion.rs`'s global ordering
    let mut order: Vec<usize> = (0..g.cells()).collect();
    order.sort_by(|&a, &b| h[a].total_cmp(&h[b]).then_with(|| a.cmp(&b)));

    let mut acc: Vec<f64> = g.areas.clone();
    for &i in order.iter().rev() {
        let w = weights(g, r, &h, i);
        let a = acc[i];
        for (j, x) in w {
            acc[j] += a * x;
        }
    }

    // conservation: everything must end at the sinks, and the sinks must hold it all
    let total: f64 = g.total_area();
    let sink_total: f64 = (0..g.cells())
        .filter(|&i| weights(g, r, &h, i).is_empty())
        .map(|i| acc[i])
        .sum();
    let conservation = sink_total / total;

    // cone bias: take a ring at ~60° from the pole and look at the azimuthal spread of
    // accumulation, normalised by the cell's own area (so area spread does not leak in)
    let mut ring: Vec<f64> = Vec::new();
    let mut ring_def: Vec<f64> = Vec::new();
    let defect: Vec<bool> = (0..g.cells())
        .map(|i| g.moore[i].len() != 2 * g.adj[i].len())
        .collect();
    let ndef = defect.iter().filter(|&&d| d).count();
    for i in 0..g.cells() {
        let a = geodesic(g.centers[i], pole);
        if (a - 1.05).abs() < 0.06 {
            // normalised specific catchment: accumulation per unit area upstream of it
            let v = acc[i] / g.areas[i];
            ring.push(v);
            if defect[i] || g.moore[i].iter().any(|&j| defect[j]) {
                ring_def.push(v);
            }
        }
    }
    let cv = |xs: &[f64]| -> f64 {
        if xs.len() < 2 {
            return f64::NAN;
        }
        let m = xs.iter().sum::<f64>() / xs.len() as f64;
        let v = xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / xs.len() as f64;
        v.sqrt() / m
    };

    Routing {
        conservation,
        cone_bias_cv: cv(&ring),
        cone_bias_cv_at_defects: cv(&ring_def),
        defect_cells: ndef,
    }
}

/// The fan geometry AT a defect: where the 8-neighbour stencil's assumption actually
/// breaks. Returns (Moore count, the angular gaps between successive Moore neighbours in
/// degrees — an ideal fan would be eight 45° gaps).
pub fn fan_at(g: &Mesh, i: usize) -> (usize, Vec<f64>) {
    let c = g.centers[i];
    let e0 = tangent(c, g.centers[g.moore[i][0]]);
    let e1 = cross(c, e0);
    let mut ang: Vec<f64> = g.moore[i]
        .iter()
        .map(|&j| {
            let d = tangent(c, g.centers[j]);
            dot(d, e1).atan2(dot(d, e0)).to_degrees()
        })
        .collect();
    ang.sort_by(f64::total_cmp);
    let k = ang.len();
    let gaps: Vec<f64> = (0..k)
        .map(|t| {
            let d = ang[(t + 1) % k] - ang[t];
            if d < 0.0 { d + 360.0 } else { d }
        })
        .collect();
    (k, gaps)
}
