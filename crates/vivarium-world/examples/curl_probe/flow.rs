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
    /// The **one-line fix to the status quo**: the same 8-neighbour Moore fan, but with the
    /// TRUE centre-to-centre distances instead of `cell_m` / `cell_m·√2`. Isolates how much
    /// of MFD's error is the hardcoded distances and how much is the non-uniform *fan*,
    /// which no distance can repair.
    MooreMfdTrueDist,

    /// **MOMENT-CORRECTED MFD — the structure-preserving router** (added by the curl probe,
    /// 2026-07-13; `:by claude :status proposed`).
    ///
    /// The curl probe's decomposition says exactly what is wrong and therefore exactly what to
    /// fix. A router that takes its DIRECTION from the reconstructed gradient has **zero**
    /// spurious circulation; but every router that then SPLITS the mass among discrete
    /// neighbour cells re-manufactures a fan — because the transport that actually results is
    /// `Σ wₖ·êₖ`, a weighted sum of the neighbours' TRUE bearings, and nothing was constraining
    /// that sum to point where the water is supposed to go. Gradient-projected edge flux still
    /// carries **κ ≈ 4.5e-3** for precisely this reason.
    ///
    /// So constrain it. The physical claim — *"the transport direction is the steepest-descent
    /// direction of φ"* — is exactly the **first-moment condition**
    ///
    /// ```text
    ///     Σₖ wₖ · sin(βₖ − ψ) = 0        βₖ = the neighbour's TRUE bearing
    ///                                     ψ  = the reconstructed gradient's azimuth
    /// ```
    ///
    /// i.e. *the tangential moment of the outflow must vanish.* Impose it directly, by the
    /// **minimum-relative-entropy (exponential-tilt) reweighting** of MFD's own slope-weights:
    ///
    /// ```text
    ///     wₖ ← wₖ · exp(−λ·sin(βₖ − ψ))    with λ solving  Σ wₖ e^{−λsₖ} sₖ = 0
    /// ```
    ///
    /// `f(λ) = Σ wₖ e^{−λsₖ} sₖ` is strictly decreasing (`f' = −Σ wₖe^{−λsₖ}sₖ² < 0`), so the
    /// root exists and is unique whenever the downhill neighbours BRACKET ψ — which they do
    /// wherever water is actually flowing. Weights stay **strictly positive** and are
    /// renormalised, so:
    ///
    /// * **conservation is exact** (nonneg weights summing to 1 — on any graph, any valence);
    /// * **MFD's dispersion is preserved** (it is the closest weight set to MFD's, in KL, that
    ///   satisfies the physics — it does not collapse to a single-receiver D8, and it does not
    ///   collapse to D-∞'s two-neighbour split either);
    /// * **the transport direction is EXACTLY the gradient direction**, at every cell, at any
    ///   valence, on any grid ⇒ **κ ≡ 0. The topological identity is preserved by construction,
    ///   not approximated.**
    ///
    /// It needs no diagonals-are-45° assumption, no `cell_m·√2`, no even-spacing premise. It
    /// asks the grid where its neighbours actually are, and then does the physics.
    MomentMfd,

    /// **Gradient-projected edge flux + the moment correction.** The two defects are
    /// SEPARABLE and this composes the two fixes:
    ///
    /// | defect | what it is | fixed by |
    /// |---|---|---|
    /// | **directional bias / circulation** (κ) | the transport does not point down the gradient | the **moment constraint** |
    /// | **dispersion / catchment magnitude** | the outflow spreads over the wrong width | **edge-flux weighting** (4 real edges, weighted by edge LENGTH) |
    ///
    /// `MomentMfd` fixes the first and leaves the second (κ → 5e-6, but the cone catchment
    /// error stays ~20%: it keeps MFD's over-dispersive 8-node fan). `GradEdgeFlux` fixes the
    /// second and leaves the first (4.7% catchment error, but κ = 4.5e-3). Neither is the
    /// answer. **The moment correction is a PROJECTION that can be applied to any weight set**,
    /// so apply it to the accurate one.
    ///
    /// ⚠ **AND MEASURED: THAT NAIVE COMPOSITION FAILS, which is why it is kept here.** An edge
    /// router has only 2–4 receivers, so the moment constraint has almost no freedom: with two
    /// receivers, `Σw = 1` and `Σwₖsₖ = 0` fully determine the weights, the scheme collapses to
    /// **D-∞**, and every bit of slope-proportional magnitude information is thrown away. The
    /// catchment error gets *worse* (8.09% vs GradEdgeFlux's 4.68%), and κ does not even reach
    /// zero (1e-3 residual, from the cells where the few receivers fail to bracket ψ).
    /// **Composing two correct ideas produced something worse than either. Recorded as a
    /// negative result; `QuadMoment` is what actually works, and why.**
    MomentEdge,

    /// **THE RECOVERED KERNEL — the physical claim, discretised honestly, then constrained.**
    ///
    /// `MomentEdge` fails because the moment constraint needs FREEDOM (many receivers) while
    /// magnitude accuracy needs correct WIDTHS. MFD-8 has the receivers but assigns them widths
    /// it invented (`45°` each, always). So stop inventing them and ask the grid.
    ///
    /// Go back to the Prime Question. MFD's physical claim is *"distribute outflow over the
    /// downhill directions in proportion to slope"* — **MFD is a QUADRATURE, and the 8 cells
    /// are its nodes.** A quadrature needs two things and MFD supplies only one:
    ///
    /// ```text
    ///   wₖ  ∝  cos(βₖ − ψ)  ·  Δβₖ
    ///          └─ integrand ┘   └─ the NODE'S OWN ANGULAR WIDTH: half the gap to
    ///          (the outflow      each neighbour on either side. MFD hardcodes this
    ///           through βₖ)      to 45° and is wrong by up to 2× on our sphere.
    /// ```
    ///
    /// That is the Riemann sum MFD was always reaching for. Then apply the moment correction,
    /// which now has 8 receivers of freedom to work with. Result: exact conservation, κ at the
    /// operator floor, AND the best catchment accuracy of any router here.
    ///
    /// *(`:by claude :status proposed`. This is a kernel proposal, not a decision.)*
    QuadMoment,
}

/// **THE MOMENT CORRECTION — a projection applicable to ANY router's weights.**
///
/// Re-tilt a weight set until its tangential moment about the true flow direction vanishes:
/// `Σₖ wₖ·sin(βₖ − ψ) = 0`, i.e. until the transport it produces actually points down the
/// gradient. Minimum-relative-entropy (exponential) tilt, so the weights stay strictly
/// positive and the correction is the SMALLEST one (in KL) consistent with the physics —
/// it does not throw away the dispersion the input weights encoded.
///
/// `f(λ) = Σ wₖ e^{−λsₖ} sₖ` is strictly decreasing (`f′ = −Σ wₖe^{−λsₖ}sₖ² < 0`), so the root
/// exists and is unique whenever the receivers BRACKET ψ — which they do wherever water flows.
/// When they do not (a ridge crest, a one-receiver cell), the tilt is skipped and the weights
/// pass through: the correction can never make things worse, and never breaks conservation
/// (positive weights, renormalised).
fn moment_correct(g: &Mesh, i: usize, w: Vec<(usize, f64)>, psi: f64, e0: V3, e1: V3) -> Vec<(usize, f64)> {
    if w.len() < 2 {
        return w;
    }
    let c = g.centers[i];
    let s: Vec<f64> = w
        .iter()
        .map(|&(j, _)| {
            let t = tangent(c, g.centers[j]);
            (dot(t, e1).atan2(dot(t, e0)) - psi).sin()
        })
        .collect();
    let smin = s.iter().cloned().fold(f64::INFINITY, f64::min);
    let smax = s.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if !(smin < -1e-12 && smax > 1e-12) {
        return w; // the receivers do not bracket ψ — nothing to solve
    }
    let f = |lam: f64| -> f64 {
        w.iter().zip(&s).map(|(&(_, wk), &sk)| wk * (-lam * sk).exp() * sk).sum::<f64>()
    };
    let (mut lo, mut hi) = (-60.0f64, 60.0f64);
    if f(lo) < 0.0 || f(hi) > 0.0 {
        return w;
    }
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if f(mid) > 0.0 { lo = mid } else { hi = mid }
    }
    let lam = 0.5 * (lo + hi);
    let mut out: Vec<(usize, f64)> =
        w.iter().zip(&s).map(|(&(j, wk), &sk)| (j, wk * (-lam * sk).exp())).collect();
    let tot: f64 = out.iter().map(|e| e.1).sum();
    if tot <= 0.0 {
        return w;
    }
    for e in out.iter_mut() {
        e.1 /= tot;
    }
    out
}

impl Router {
    pub fn label(self) -> &'static str {
        match self {
            Router::MooreMfd => "8-nbr Moore MFD (status quo)",
            Router::EdgeMfd => "edge MFD (no diagonals)",
            Router::GradEdgeFlux => "gradient-projected edge flux",
            Router::MooreMfdTrueDist => "8-nbr Moore MFD + true distances",
            Router::MomentMfd => "  moment-corrected MFD-8",
            Router::MomentEdge => "  moment + edge flux (FAILS)",
            Router::QuadMoment => "TRUE-WIDTH QUADRATURE + MOMENT",
        }
    }
}

/// The least-squares gradient in the tangent plane at cell `i`, over its EDGE neighbours.
/// Returns `(gx, gy)` in the local frame `(e0, e1)` returned alongside.
fn lsq_grad(g: &Mesh, h: &[f64], i: usize) -> Option<(f64, f64, V3, V3)> {
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
        return None;
    }
    Some((
        (syy * sxz - sxy * syz) / det,
        (sxx * syz - sxy * sxz) / det,
        e0,
        e1,
    ))
}

/// Outgoing routing weights for cell `i` (normalised, summing to 1 unless it is a sink).
///
/// ⚠ **`MooreMfd` here IS `erosion.rs`, exactly** — not an approximation of it. `erosion.rs`
/// uses one *global* `cell_m = sample::cell_size_m(level)`; this uses the cell's own mean
/// edge distance. Those differ by up to ~6% on our grid, and it makes **no difference at
/// all**: a *uniform* rescale `d → s·d` of every neighbour distance multiplies every weight
/// by `s^−p` and cancels exactly in the normalisation. So the ONLY content of MFD's distance
/// hardcode is the **ratio** `diag/axial = √2` — the absolute value of `cell_m` is inert
/// here. (It is *not* inert in `incise`/`talus`, where `dist` appears unnormalised. Different
/// question; not this probe's.)
pub fn weights(g: &Mesh, r: Router, h: &[f64], i: usize) -> Vec<(usize, f64)> {
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
        Router::MooreMfdTrueDist => {
            let mut w = Vec::new();
            let mut tot = 0.0;
            for &j in &g.moore[i] {
                let dist = geodesic(g.centers[i], g.centers[j]) * g.radius_m;
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
        Router::QuadMoment => {
            // 1. WHERE the water must go: the reconstructed gradient. (D-∞'s idea — a
            //    continuous angle, not one of 8 pre-chosen compass points.)
            let Some((gx, gy, e0, e1)) = lsq_grad(g, h, i) else { return Vec::new() };
            if (gx * gx + gy * gy).sqrt() < 1e-14 {
                return Vec::new();
            }
            let psi = (-gy).atan2(-gx);

            // 2. The quadrature NODES: every strictly-lower Moore neighbour in the downstream
            //    half-plane, at its TRUE bearing. Ask the grid; do not assume 45°.
            //    ⚠ Bearings are stored RELATIVE TO ψ, wrapped to (−π, π]. Not cosmetic: the
            //    width rule below SORTS them, and an absolute bearing has its branch cut at an
            //    arbitrary place — set by the LSQ frame, which is set by `adj[i][0]`, which is
            //    set by a HashMap iteration order. A downstream fan straddling that cut sorted
            //    arbitrarily, and the widths came out arbitrary with it. It showed up as
            //    run-to-run drift in the cone error (17.67–17.74%) while MFD-8 and edge flux
            //    were bit-stable — i.e. it was MY bug, not the harness's. Relative bearings all
            //    lie in (−90°, +90°) by the half-plane test, so they can never straddle the cut.
            let c = g.centers[i];
            let mut nb: Vec<(usize, f64)> = Vec::new(); // (j, β−ψ ∈ (−90°, 90°))
            for &j in &g.moore[i] {
                if h[j] >= h[i] {
                    continue;
                }
                let t = tangent(c, g.centers[j]);
                let beta = dot(t, e1).atan2(dot(t, e0));
                let rel = (beta - psi).sin().atan2((beta - psi).cos()); // wrap to (−π, π]
                if rel.cos() <= 0.0 {
                    continue; // not downstream: a transverse neighbour, not a receiver
                }
                nb.push((j, rel));
            }
            if nb.is_empty() {
                return Vec::new();
            }

            // 3. The quadrature WIDTHS — the piece MFD never had. Each node represents the
            //    angular sector midway to its neighbours on either side. On a perfect lattice
            //    every Δβ is 45° and this reduces to MFD exactly; on our sphere they are not,
            //    and that difference IS the bias.
            let mut ord: Vec<usize> = (0..nb.len()).collect();
            ord.sort_by(|&a, &b| nb[a].1.total_cmp(&nb[b].1));
            let k = ord.len();
            let mut w: Vec<(usize, f64)> = Vec::with_capacity(k);
            for (t, &oi) in ord.iter().enumerate() {
                let (j, rel) = nb[oi];
                let width = if k == 1 {
                    std::f64::consts::PI // a lone receiver carries the whole half-plane
                } else if t == 0 {
                    nb[ord[1]].1 - rel
                } else if t == k - 1 {
                    rel - nb[ord[k - 2]].1
                } else {
                    0.5 * (nb[ord[t + 1]].1 - nb[ord[t - 1]].1)
                };
                // the integrand: outflow through this bearing ∝ the gradient's component on it
                let x = rel.cos() * width.abs().max(1e-9);
                if x > 0.0 {
                    w.push((j, x));
                }
            }
            let tot: f64 = w.iter().map(|e| e.1).sum();
            if tot <= 0.0 {
                return Vec::new();
            }
            for e in w.iter_mut() {
                e.1 /= tot;
            }

            // 4. THE CONSTRAINT. The quadrature is now honest but still not EXACT (a Riemann
            //    sum over 8 unevenly spaced nodes has a residual first moment). Impose the
            //    physical claim: the tangential moment must vanish. Now it has 8 receivers of
            //    freedom to do it with — which is exactly what `MomentEdge` did not have.
            moment_correct(g, i, w, psi, e0, e1)
        }
        Router::MomentMfd => {
            // 1. the flow direction the PHYSICS dictates: the reconstructed gradient.
            let Some((gx, gy, e0, e1)) = lsq_grad(g, h, i) else { return Vec::new() };
            if (gx * gx + gy * gy).sqrt() < 1e-14 {
                return Vec::new();
            }
            let psi = (-gy).atan2(-gx); // azimuth of −∇h: where the water must go

            // 2. MFD's own slope-proportional weights, on the TRUE distances and the TRUE
            //    bearings. Dispersion is retained; only the premise "45° apart" is dropped.
            let c = g.centers[i];
            let mut nb: Vec<(usize, f64, f64)> = Vec::new(); // (j, w, s = sin(β−ψ))
            for &j in &g.moore[i] {
                let drop = h[i] - h[j];
                if drop <= 0.0 {
                    continue;
                }
                let dist = geodesic(c, g.centers[j]) * g.radius_m;
                let t = tangent(c, g.centers[j]);
                let beta = dot(t, e1).atan2(dot(t, e0));
                // Only the downstream half-plane can carry outflow; a "lower" cell more than
                // 90° off the gradient is a transverse neighbour, not a receiver.
                let cs = (beta - psi).cos();
                if cs <= 0.0 {
                    continue;
                }
                nb.push((j, (drop / dist).powf(P), (beta - psi).sin()));
            }
            if nb.is_empty() {
                return Vec::new();
            }

            // 3. THE CONSTRAINT: the tangential moment must vanish, Σ wₖ sin(βₖ−ψ) = 0.
            let mut w: Vec<(usize, f64)> = nb.iter().map(|&(j, wk, _)| (j, wk)).collect();
            let tot: f64 = w.iter().map(|e| e.1).sum();
            for e in w.iter_mut() {
                e.1 /= tot;
            }
            moment_correct(g, i, w, psi, e0, e1)
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
        Router::GradEdgeFlux | Router::MomentEdge => {
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
                // STRICTLY DOWNHILL ONLY. The LSQ gradient is a fit, so its outgoing
                // component through an edge can be positive even when the neighbour across
                // that edge is HIGHER. Routing there would send mass upstream, and an
                // elevation-ordered sweep (Priority-Flood, `accumulate_drainage`) has
                // already passed that cell — so the mass is stranded and never reaches an
                // outlet. Measured before this guard: conservation 0.000 instead of 1.000.
                // The guard costs nothing and restores the strictly-downstream invariant
                // that the whole ordered sweep depends on.
                if h[e.j] >= h[i] {
                    continue;
                }
                // The **edge normal** — NOT the direction to the neighbour. On a
                // non-orthogonal mesh those differ, and the difference is exactly the
                // thing a centre-line router gets wrong. `e.normal` is the outward
                // normal in the tangent plane at the mid-edge; parallel-transport it to
                // the cell centre by projecting out the radial component.
                let nv = tangent(c, e.normal);
                let (nx, ny) = (dot(nv, e0), dot(nv, e1));
                let out = -(gx * nx + gy * ny); // outgoing component of −∇h through the edge
                if out > 0.0 {
                    let x = out * e.edge_len_m; // weight by the edge LENGTH it crosses
                    w.push((e.j, x));
                    tot += x;
                }
            }
            if tot <= 0.0 {
                return Vec::new();
            }
            for e in w.iter_mut() {
                e.1 /= tot;
            }
            // ⇒ The edge-flux SPLIT is accurate in magnitude (it fixes the dispersion), but it
            //   still leaves the TRANSPORT direction Σwₖ·êₖ unconstrained — a residual fan, and
            //   κ = 4.5e-3. Impose the physical claim on the weights themselves.
            if r == Router::MomentEdge {
                let psi = (-gy).atan2(-gx); // azimuth of −∇h: where the water must go
                return moment_correct(g, i, w, psi, e0, e1);
            }
            w
        }
    }
}

pub struct Routing {
    /// Total accumulated at the sinks ÷ total area. **1.0 exactly = the router conserved.**
    pub conservation: f64,
    /// **Mean |error| against the EXACT answer.** On a cone the specific catchment area has
    /// a closed form: everything upslope drains radially, so at angular distance θ from the
    /// apex the catchment per unit contour length is
    ///
    /// ```text
    ///   a(θ) = area(cap θ) / circumference(θ)
    ///        = 2πR²(1 − cos θ) / (2πR sin θ)
    ///        = R · tan(θ/2)
    /// ```
    ///
    /// This is the standard test (Tarboton 1997 uses it for exactly this purpose) and it
    /// beats a symmetry/CV check outright: it has a TRUE VALUE, not just an expectation of
    /// uniformity, so it catches a router that is smoothly wrong as well as one that is
    /// lumpily wrong.
    pub cone_err_mean: f64,
    pub cone_err_max: f64,
    /// The same error, measured only over the 24 defect cells and their neighbours.
    pub cone_err_at_defects: f64,
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

    // ACCURACY against the closed form: a(θ) = R·tan(θ/2) per unit contour length.
    let defect: Vec<bool> = (0..g.cells())
        .map(|i| g.moore[i].len() != 2 * g.adj[i].len())
        .collect();
    let ndef = defect.iter().filter(|&&d| d).count();
    let (mut errs, mut errs_def) = (Vec::new(), Vec::new());
    for i in 0..g.cells() {
        let th = geodesic(g.centers[i], pole);
        // skip the apex (badly resolved) and the antipodal sink region
        if !(0.35..=2.2).contains(&th) {
            continue;
        }
        if weights(g, r, &h, i).is_empty() {
            continue;
        }
        // CONTOUR WIDTH — the width of the cell's outflow face projected PERPENDICULAR to
        // the flow: w = Σₑ Lₑ · max(0, n̂ₑ·f̂). (Summing the raw outflow edge lengths instead
        // double-counts whenever the flow disperses across two edges, and inflates every
        // router's error by ~2×. On a square cell with axis-aligned flow this gives h; with
        // diagonal flow it gives h√2 — both correct.)
        //
        // f̂ is the EXACT downslope direction of the cone, not the router's own estimate, so
        // every router is scored against the same denominator and the differences that show
        // up are differences in the routed mass alone.
        let fhat = scale(tangent(g.centers[i], pole), -1.0);
        let mut wid = 0.0;
        for e in &g.adj[i] {
            let mid = unit(add(g.verts[e.va as usize], g.verts[e.vb as usize]));
            let nh = tangent(g.centers[i], tangent(mid, e.normal));
            wid += e.edge_len_m * dot(nh, fhat).max(0.0);
        }
        if wid <= 0.0 {
            continue;
        }
        let exact = g.radius_m * (th / 2.0).tan();
        let err = ((acc[i] / wid) / exact - 1.0).abs();
        errs.push(err);
        if defect[i] || g.moore[i].iter().any(|&j| defect[j]) {
            errs_def.push(err);
        }
    }
    let mean = |xs: &[f64]| -> f64 {
        if xs.is_empty() {
            return f64::NAN;
        }
        xs.iter().sum::<f64>() / xs.len() as f64
    };

    Routing {
        conservation,
        cone_err_mean: mean(&errs),
        cone_err_max: errs.iter().cloned().fold(0.0f64, f64::max),
        cone_err_at_defects: mean(&errs_def),
        defect_cells: ndef,
    }
}

// ===========================================================================
// THE PLUME — bias or noise? The question that actually decides it.
// ===========================================================================
//
// A directionally-BIASED 20% routing error manufactures a fake physical law (rivers
// preferring grid axes) and ACCUMULATES down a catchment. An unbiased 20% error is noise
// and largely washes out once drainage area sums over a large upstream region. Same
// magnitude; opposite consequence. The `cone_err` columns in `Routing` cannot tell them
// apart — they are |errors|, and an |error| has thrown away the sign that carries the whole
// answer.
//
// So: the cone again, but now with the EXACT answer used as a *trajectory* rather than a
// magnitude. `h = −θ` about a pole is radially symmetric, so every flow line is a MERIDIAN:
// water released at azimuth ψ must arrive at azimuth ψ, at every θ, exactly. Release unit
// mass at ONE cell and track the mass-weighted azimuthal CENTROID as it descends.
//
//   * MFD *disperses* — that is its job — so the plume must SPREAD. Spread is not error.
//   * The CENTROID, though, has an exact answer: it must not move.
//
// centroid drift  = BIAS. It is signed, it is a function of the launch azimuth, and if it
//                   grows ∝ path length it accumulates.
// spread          = dispersion. Grows ∝ √path if it is incoherent — i.e. noise.
//
// This separates the two cleanly, which is the whole point, and it can fail: on a grid whose
// fan is honest the drift is zero and only the spread grows.

pub struct Plume {
    /// Launch azimuth about the pole (degrees).
    pub launch_deg: f64,
    /// Sampled at the end of the path: (θ, drift°, spread°, path_m, lateral_m, cells_of_path).
    pub theta: Vec<f64>,
    /// Signed azimuthal drift of the mass centroid, degrees. EXACT ANSWER: 0 at every θ.
    pub drift_deg: Vec<f64>,
    /// Mass-weighted circular spread of the plume, degrees. Growth here is dispersion, not error.
    pub spread_deg: Vec<f64>,
}

/// Release unit mass at the cell nearest `(θ0, launch)` about `pole`, route it with `r`, and
/// report the centroid drift and spread as it descends. `frame` is a tangent vector at the
/// pole defining azimuth 0.
pub fn plume(g: &Mesh, r: Router, pole: V3, frame: V3, theta0: f64, launch_deg: f64, rings: &[f64]) -> Plume {
    let h: Vec<f64> = g.centers.iter().map(|&p| -geodesic(p, pole)).collect();
    let e0 = tangent(pole, frame);
    let e1 = cross(pole, e0);
    // azimuth of a point about the pole, in the (e0,e1) frame
    let az = |p: V3| -> f64 {
        let t = tangent(pole, p);
        dot(t, e1).atan2(dot(t, e0))
    };
    // the launch cell: nearest centre to the target (θ0, launch)
    let lr = launch_deg.to_radians();
    let target = unit(add(
        scale(pole, theta0.cos()),
        scale(add(scale(e0, lr.cos()), scale(e1, lr.sin())), theta0.sin()),
    ));
    let src = (0..g.cells())
        .min_by(|&a, &b| geodesic(g.centers[a], target).total_cmp(&geodesic(g.centers[b], target)))
        .unwrap();

    let mut order: Vec<usize> = (0..g.cells()).collect();
    order.sort_by(|&a, &b| h[a].total_cmp(&h[b]).then_with(|| a.cmp(&b)));

    let mut acc = vec![0.0f64; g.cells()];
    acc[src] = 1.0;
    for &i in order.iter().rev() {
        if acc[i] <= 0.0 {
            continue;
        }
        let a = acc[i];
        for (j, x) in weights(g, r, &h, i) {
            acc[j] += a * x;
        }
    }

    // The launch azimuth is the SOURCE CELL's azimuth, not the requested one — the cell
    // centre is where the mass actually starts, and using the request would charge the
    // router for our rounding.
    let a0 = az(g.centers[src]);
    let mut theta = Vec::new();
    let mut drift_deg = Vec::new();
    let mut spread_deg = Vec::new();
    for &th in rings {
        // a thin annulus, one ring of cells wide, at angular distance th
        let band = 1.5 * (std::f64::consts::FRAC_PI_2 / (g.cells() as f64 / 6.0).sqrt());
        let (mut sx, mut sy, mut m) = (0.0, 0.0, 0.0);
        for i in 0..g.cells() {
            if acc[i] <= 0.0 {
                continue;
            }
            let t = geodesic(g.centers[i], pole);
            if (t - th).abs() > band {
                continue;
            }
            // measure azimuth RELATIVE to the launch, so the wrap is never near ±π
            let d = az(g.centers[i]) - a0;
            let d = (d.sin()).atan2(d.cos());
            sx += acc[i] * d.cos();
            sy += acc[i] * d.sin();
            m += acc[i];
        }
        if m <= 0.0 {
            continue;
        }
        let (cx, cy) = (sx / m, sy / m);
        let rlen = (cx * cx + cy * cy).sqrt().min(1.0);
        theta.push(th);
        drift_deg.push(cy.atan2(cx).to_degrees());
        // circular standard deviation: √(−2 ln R)
        spread_deg.push((-2.0 * rlen.max(1e-12).ln()).sqrt().to_degrees());
    }
    Plume { launch_deg: a0.to_degrees(), theta, drift_deg, spread_deg }
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
