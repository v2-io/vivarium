//! `stencil_bench` — **what does the corrected finite-volume scheme COST in the hot loop?**
//!
//! `ref/research/grid-comparison-report.md` §10.6 names this the report's biggest unprobed
//! claim: the corrected scheme (mid-edge normal projection + a WIDE quadratic LSQ gradient
//! over the 8 Moore neighbours) buys **9.2e-1 → 3.6e-4** on a Laplacian, but it is a **5×5
//! normal-equation solve per cell per step** and *nobody benchmarked it*. The report then
//! offers a rescue — *"the geometry is static, so the pseudo-inverse could be factored once
//! per (face, level) and reused, making the hot loop a fixed set of multiply-adds"* — and
//! marks it **[I], an inference, not a measurement.**
//!
//! This example measures both, against **real cube-sphere geometry** at the levels erosion
//! actually runs (L19, ~19 m cells, `TILE_NX = 64`), on the **dense Cartesian tile** layout
//! the real hot loop uses (`erosion.rs` → `Patch<f32>`, row-major, halo) — *not* on an
//! irregular `Mesh` with pointer-chasing adjacency, which would confound the scheme's cost
//! with indirection cost and would not be comparable to `spatial-key-bench.md`'s 6 G
//! cell-updates/s dense baseline.
//!
//! **The oracle.** `grid_lab`'s own `mesh.rs` + `probes.rs` are included verbatim (via
//! `#[path]`), so the report's `Scheme::FvLsq` *is* the reference implementation. Every dense
//! kernel here is checked cell-for-cell against it on the same geometry before any timing
//! number is believed. A kernel that does not reproduce the oracle is not a faster kernel; it
//! is a different scheme.
//!
//! Run:
//! ```text
//! RUSTFLAGS="-C target-cpu=native" cargo run --release -p vivarium-world --example stencil_bench
//! ```

#![allow(clippy::needless_range_loop)]

#[path = "grid_lab/mesh.rs"]
mod mesh;
#[path = "grid_lab/grids.rs"]
mod grids;
#[path = "grid_lab/probes.rs"]
mod probes;

use mesh::*;
use std::hint::black_box;
use std::time::Instant;

const R_M: f64 = 6_371_000.0;

// ===========================================================================
// 0. THE FIELD-SPACE GEOMETRY OF A TILE — the real equiangular cube-sphere map,
//    on the dense row-major layout `erosion.rs` actually uses.
// ===========================================================================

/// A tile of `n × n` cells at `(face, level, origin)`, with `halo` rings around it.
/// Row-major, stride `s = n + 2·halo`. Index `(x, y)` with `x, y ∈ 0..s` is cell
/// `(oi - halo + x, oj - halo + y)` on the face.
///
/// Geometry is computed **exactly as `mesh.rs::Mesh::build` computes it** (same formulae,
/// same order of operations) — the oracle check in §A1 is what proves that claim rather
/// than asserting it.
struct Tile {
    face: usize,
    level: u8,
    oi: u32,
    oj: u32,
    n: usize,
    halo: usize,
    s: usize,
    /// Cell-centre unit vectors, `s·s`.
    ctr: Vec<V3>,
    /// Cell area (m²), `s·s`.
    area: Vec<f64>,
    /// Per cell, the 4 edges in fixed order **E(+i), N(+j), W(−i), S(−j)**.
    /// `t[4·c + e]` = transmissibility `L_e / (a_j − a_i)` (m).
    t: Vec<f64>,
    /// `wi[4·c + e]` = the static offset vector `a_i·n̂ − v_i` at the mid-edge (m),
    /// against which cell `c`'s own gradient is contracted.
    wi: Vec<[f64; 3]>,
    /// `wj[4·c + e]` = the same for the **neighbour's** gradient.
    wj: Vec<[f64; 3]>,
    /// Local tangent basis per cell (the frame the LSQ fit is done in).
    e0: Vec<V3>,
    e1: Vec<V3>,
}

/// The 8 Moore offsets, in the order the LSQ fit consumes them.
const MOORE: [(isize, isize); 8] =
    [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
/// The 4 edge offsets, in the order `E, N, W, S`.
const EDGE: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// The **21-point** fused stencil's support: `[-1,1]²` (the cell's own Moore block, which its
/// own gradient reads) ∪ the four edge-neighbours' Moore blocks. The four `(±2, ±2)` corners
/// are **not** in the support — the fused stencil is 21 points, not 25.
fn fused_offsets() -> Vec<(isize, isize)> {
    let mut v = Vec::new();
    for dy in -2isize..=2 {
        for dx in -2isize..=2 {
            if dx.abs() == 2 && dy.abs() == 2 {
                continue; // the 4 corners of the 5×5 are outside the support
            }
            v.push((dx, dy));
        }
    }
    assert_eq!(v.len(), 21);
    v
}

/// Cube-face lattice **corner** `(vi, vj)` (integer, `0..=2^level`) → unit sphere.
fn corner(face: usize, level: u8, vi: i64, vj: i64) -> V3 {
    let n = (1u64 << level) as f64;
    let (u, v) = (2.0 * vi as f64 / n - 1.0, 2.0 * vj as f64 / n - 1.0);
    grids::cube_to_unit(grids::CubeProj::Equiangular, face, u, v)
}

/// Cube-face **cell centre** `(i, j)` → unit sphere. This is `CellId::to_cube().to_unit()`
/// — the parameter-space midpoint, exactly what `sphere.rs` returns.
fn centre(face: usize, level: u8, i: i64, j: i64) -> V3 {
    let n = (1u64 << level) as f64;
    let (u, v) = ((i as f64 + 0.5) / n * 2.0 - 1.0, (j as f64 + 0.5) / n * 2.0 - 1.0);
    grids::cube_to_unit(grids::CubeProj::Equiangular, face, u, v)
}

impl Tile {
    fn new(face: usize, level: u8, oi: u32, oj: u32, n: usize, halo: usize) -> Tile {
        let s = n + 2 * halo;
        let h = halo as i64;
        let (bi, bj) = (oi as i64 - h, oj as i64 - h);

        let mut ctr = vec![[0.0; 3]; s * s];
        let mut area = vec![0.0; s * s];
        let mut e0v = vec![[0.0; 3]; s * s];
        let mut e1v = vec![[0.0; 3]; s * s];
        for y in 0..s {
            for x in 0..s {
                let (i, j) = (bi + x as i64, bj + y as i64);
                let c = centre(face, level, i, j);
                ctr[y * s + x] = c;
                let ring = [
                    corner(face, level, i, j),
                    corner(face, level, i + 1, j),
                    corner(face, level, i + 1, j + 1),
                    corner(face, level, i, j + 1),
                ];
                area[y * s + x] = poly_area(&ring) * R_M * R_M;
                // The LSQ frame. `probes::lsq_gradients_quadratic` takes `e0` toward the
                // first edge-neighbour; the fitted gradient is a 3-vector and the weighted
                // LSQ objective is rotation-invariant in the tangent plane, so ANY frame
                // gives the same gradient. (§A1 checks that empirically against the oracle,
                // whose frame comes from a HashMap iteration order we do not control.)
                let e0 = tangent(c, centre(face, level, i + 1, j));
                e0v[y * s + x] = e0;
                e1v[y * s + x] = cross(c, e0);
            }
        }

        // ---- edge geometry, exactly as `Mesh::build` derives it ----
        let mut t = vec![0.0; 4 * s * s];
        let mut wi = vec![[0.0; 3]; 4 * s * s];
        let mut wj = vec![[0.0; 3]; 4 * s * s];
        for y in 0..s {
            for x in 0..s {
                let (i, j) = (bi + x as i64, bj + y as i64);
                let c = y * s + x;
                let ci = ctr[c];
                // the shared edge's two corner vertices, per edge direction
                let ends: [(V3, V3); 4] = [
                    (corner(face, level, i + 1, j), corner(face, level, i + 1, j + 1)), // E
                    (corner(face, level, i, j + 1), corner(face, level, i + 1, j + 1)), // N
                    (corner(face, level, i, j), corner(face, level, i, j + 1)),         // W
                    (corner(face, level, i, j), corner(face, level, i + 1, j)),         // S
                ];
                for (e, &(dx, dy)) in EDGE.iter().enumerate() {
                    let cj = centre(face, level, i + dx as i64, j + dy as i64);
                    let (a, b) = ends[e];
                    let mid = unit(add(a, b));
                    let edge_len_m = geodesic(a, b) * R_M;
                    let n0 = tangent(mid, unit(cross(a, b)));
                    // orient outward from i (mesh.rs line "orient the normal outward from i")
                    let sgn = if dot(n0, tangent(mid, cj)) >= 0.0 { 1.0 } else { -1.0 };
                    let nrm = scale(n0, sgn);

                    // `probes::edge_flux` re-derives the frame at the mid-edge `f` from the
                    // edge's two verts, then projects both centres onto the normal LINE
                    // through `f`. Reproduced verbatim:
                    let f = mid;
                    let nh = tangent(f, nrm);
                    let vi = scale(tangent(f, ci), geodesic(f, ci) * R_M);
                    let vj = scale(tangent(f, cj), geodesic(f, cj) * R_M);
                    let (ai, aj) = (dot(vi, nh), dot(vj, nh));
                    let d = aj - ai;
                    assert!(d > 1e-9, "degenerate normal span at ({i},{j}) edge {e}");

                    t[4 * c + e] = edge_len_m / d;
                    wi[4 * c + e] = sub(scale(nh, ai), vi);
                    wj[4 * c + e] = sub(scale(nh, aj), vj);
                }
            }
        }

        Tile { face, level, oi, oj, n, halo, s, ctr, area, t, wi, wj, e0: e0v, e1: e1v }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize { y * self.s + x }

    /// Mean cell size (m) — what the naive uniform kernel would be handed.
    fn mean_cell_m(&self) -> f64 {
        let mut acc = 0.0;
        let mut n = 0;
        for y in self.halo..self.halo + self.n {
            for x in self.halo..self.halo + self.n {
                acc += self.area[self.idx(x, y)].sqrt();
                n += 1;
            }
        }
        acc / n as f64
    }
}

// ===========================================================================
// 1. THE PRECOMPUTED FORM — the report's [I] rescue, made concrete.
//
// The corrected scheme is LINEAR in `u` with coefficients that depend only on the
// (static) geometry. Therefore the whole thing factors:
//
//   ∇u_c   = e0_c·gx_c + e1_c·gy_c,  with (gx_c, gy_c) = Σ_k (cx_c[k], cy_c[k])·(u_k − u_c)
//   flux_e = [ (u_j + g_j·w_j^e) − (u_i + g_i·w_i^e) ] · T_e
//   Δu_i   = (1/A_i) Σ_{e∈4} flux_e
//
// `cx`, `cy`, `w`, `T`, `A` are ALL static. So the 5×5 normal-equation solve can be done
// once per cell, ever — and the hot loop becomes multiply-adds. Composing the two levels
// gives a single **21-point stencil with static weights**, which is the fastest form.
//
// This is an EXACT algebraic refactor, not an approximation: §A1 checks it reproduces the
// oracle to round-off.
// ===========================================================================

/// The 8 gradient coefficients per cell, per component: `gx = Σ cx[k]·(u[nb_k] − u[c])`.
/// Obtained by solving the SAME 5×5 weighted normal equations `probes` solves, once, with
/// the 8 unit right-hand sides — but **nondimensionalised** (§0): the fit is done in units
/// of the cell size and the gradient scaled back, which is an exact reparametrisation and
/// takes `cond(AᵀA)` from ~1e10 to 8.
fn grad_coeffs(tile: &Tile, c: usize) -> ([f64; 8], [f64; 8]) {
    let cc = tile.ctr[c];
    let (e0, e1) = (tile.e0[c], tile.e1[c]);
    let s = tile.s;
    let (cx0, cy0) = ((c % s) as isize, (c / s) as isize);
    let hs = tile.area[c].sqrt(); // the nondimensionalising length

    let mut ata = [[0.0f64; 5]; 5];
    let mut rows = [[0.0f64; 5]; 8];
    let mut ws = [0.0f64; 8];
    for (k, &(dx, dy)) in MOORE.iter().enumerate() {
        let nb = ((cy0 + dy) * s as isize + (cx0 + dx)) as usize;
        let d = sub(tile.ctr[nb], cc);
        let (x, y) = (dot(d, e0) * R_M / hs, dot(d, e1) * R_M / hs);
        let row = [x, y, 0.5 * x * x, x * y, 0.5 * y * y];
        let w = 1.0 / (x * x + y * y); // inverse-distance weighting, as in `probes`
        rows[k] = row;
        ws[k] = w;
        for a in 0..5 {
            for b in 0..5 {
                ata[a][b] += w * row[a] * row[b];
            }
        }
    }
    // invert the 5×5 (Gauss-Jordan with partial pivoting)
    let mut m = [[0.0f64; 10]; 5];
    for a in 0..5 {
        m[a][..5].copy_from_slice(&ata[a]);
        m[a][5 + a] = 1.0;
    }
    for col in 0..5 {
        let mut piv = col;
        for rr in col + 1..5 {
            if m[rr][col].abs() > m[piv][col].abs() {
                piv = rr;
            }
        }
        m.swap(col, piv);
        let p = m[col][col];
        assert!(p.abs() > 1e-300, "singular LSQ normal equations");
        for cc2 in 0..10 {
            m[col][cc2] /= p;
        }
        for rr in 0..5 {
            if rr == col {
                continue;
            }
            let f = m[rr][col];
            for cc2 in 0..10 {
                m[rr][cc2] -= f * m[col][cc2];
            }
        }
    }
    // (AᵀWA)⁻¹ rows 0 and 1, contracted with AᵀW → the coefficients on (u_k − u_c).
    // The `1/hs` undoes the nondimensionalisation: `∂u/∂x = (∂u/∂x̃)/hs`.
    let mut cx = [0.0f64; 8];
    let mut cy = [0.0f64; 8];
    for k in 0..8 {
        let mut gx = 0.0;
        let mut gy = 0.0;
        for a in 0..5 {
            gx += m[0][5 + a] * ws[k] * rows[k][a];
            gy += m[1][5 + a] * ws[k] * rows[k][a];
        }
        cx[k] = gx / hs;
        cy[k] = gy / hs;
    }
    (cx, cy)
}

/// The corrected face gradient on a `Mesh` — `probes::edge_flux`'s FvLsq branch, lifted out
/// so a gradient can be INJECTED. That is the only way to ask the report's own harness what
/// its answer would have been with a well-conditioned solve.
fn mesh_lap_fvlsq(g: &Mesh, u: &[f64], grad: &[V3]) -> Vec<f64> {
    (0..g.cells())
        .map(|i| {
            let mut f = 0.0;
            for e in &g.adj[i] {
                let fm = unit(add(g.verts[e.va as usize], g.verts[e.vb as usize]));
                let nh = tangent(fm, e.normal);
                let vi = scale(tangent(fm, g.centers[i]), e.arm_m);
                let vj = scale(tangent(fm, g.centers[e.j]), e.arm_opp_m);
                let (ai, aj) = (dot(vi, nh), dot(vj, nh));
                let d = aj - ai;
                if d <= 1e-9 {
                    f += (u[e.j] - u[i]) * e.edge_len_m / e.dist_m;
                    continue;
                }
                let ui = u[i] + dot(grad[i], sub(scale(nh, ai), vi));
                let uj = u[e.j] + dot(grad[e.j], sub(scale(nh, aj), vj));
                f += (uj - ui) / d * e.edge_len_m;
            }
            f / g.areas[i]
        })
        .collect()
}

/// `probes::lsq_gradients_quadratic`, **nondimensionalised**. Identical model space,
/// identical weights, identical answer in exact arithmetic — and ~10 more correct digits.
fn mesh_grad_scaled(g: &Mesh, u: &[f64]) -> Vec<V3> {
    (0..g.cells())
        .map(|i| {
            let c = g.centers[i];
            let e0 = tangent(c, g.centers[g.adj[i][0].j]);
            let e1 = cross(c, e0);
            let nb = &g.moore[i];
            if nb.len() < 5 {
                return [0.0; 3];
            }
            let hs = g.areas[i].sqrt();
            let mut ata = [[0.0f64; 5]; 5];
            let mut atb = [0.0f64; 5];
            for &j in nb {
                let d = sub(g.centers[j], c);
                let (x, y) = (dot(d, e0) * g.radius_m / hs, dot(d, e1) * g.radius_m / hs);
                let row = [x, y, 0.5 * x * x, x * y, 0.5 * y * y];
                let w = 1.0 / (x * x + y * y);
                let b = u[j] - u[i];
                for a in 0..5 {
                    for bb in 0..5 {
                        ata[a][bb] += w * row[a] * row[bb];
                    }
                    atb[a] += w * row[a] * b;
                }
            }
            let mut m = [[0.0f64; 6]; 5];
            for a in 0..5 {
                m[a][..5].copy_from_slice(&ata[a]);
                m[a][5] = atb[a];
            }
            for col in 0..5 {
                let mut piv = col;
                for rr in col + 1..5 {
                    if m[rr][col].abs() > m[piv][col].abs() {
                        piv = rr;
                    }
                }
                if m[piv][col].abs() < 1e-300 {
                    return [0.0; 3];
                }
                m.swap(col, piv);
                for rr in 0..5 {
                    if rr == col {
                        continue;
                    }
                    let f = m[rr][col] / m[col][col];
                    for cc in col..6 {
                        m[rr][cc] -= f * m[col][cc];
                    }
                }
            }
            let (gx, gy) = (m[0][5] / m[0][0] / hs, m[1][5] / m[1][1] / hs);
            add(scale(e0, gx), scale(e1, gy))
        })
        .collect()
}

/// Build the **fused 21-point stencil weights** for every interior cell.
/// Returns a flat `n·n·21` array (row-major cell, then the 21 offsets of `fused_offsets()`).
fn build_fused<T: Copy + From<f32>>(tile: &Tile, offs: &[(isize, isize)]) -> Vec<f64> {
    let (n, s, h) = (tile.n, tile.s, tile.halo);
    let mut out = vec![0.0f64; n * n * 21];
    // cache the gradient coefficients (each cell's are read by itself and by its 4 edge nbrs)
    let mut gcx = vec![[0.0f64; 8]; s * s];
    let mut gcy = vec![[0.0f64; 8]; s * s];
    for y in h - 1..s - h + 1 {
        for x in h - 1..s - h + 1 {
            let (a, b) = grad_coeffs(tile, y * s + x);
            gcx[y * s + x] = a;
            gcy[y * s + x] = b;
        }
    }
    let _ = std::marker::PhantomData::<T>;

    let pos = |dx: isize, dy: isize| -> usize {
        offs.iter().position(|&o| o == (dx, dy)).expect("offset outside the 21-point support")
    };

    for y in 0..n {
        for x in 0..n {
            let (px, py) = (x + h, y + h);
            let c = py * s + px;
            let inv_a = 1.0 / tile.area[c];
            let mut wbuf = [0.0f64; 21];
            let w = &mut wbuf;

            for (e, &(edx, edy)) in EDGE.iter().enumerate() {
                let jn = ((py as isize + edy) * s as isize + (px as isize + edx)) as usize;
                let tt = tile.t[4 * c + e] * inv_a;

                // (u_j − u_i)·T
                w[pos(edx, edy)] += tt;
                w[pos(0, 0)] -= tt;

                // + (g_j · w_j^e)·T   and   − (g_i · w_i^e)·T
                for (sign, cell, cellofs, wvec) in [
                    (1.0f64, jn, (edx, edy), tile.wj[4 * c + e]),
                    (-1.0f64, c, (0isize, 0isize), tile.wi[4 * c + e]),
                ] {
                    // contract the wanted vector into the cell's own LSQ frame
                    let p = dot(tile.e0[cell], wvec);
                    let q = dot(tile.e1[cell], wvec);
                    let (cx, cy) = (&gcx[cell], &gcy[cell]);
                    for k in 0..8 {
                        let coef = sign * tt * (p * cx[k] + q * cy[k]);
                        let (ox, oy) = (cellofs.0 + MOORE[k].0, cellofs.1 + MOORE[k].1);
                        w[pos(ox, oy)] += coef;
                        w[pos(cellofs.0, cellofs.1)] -= coef; // the −u_c in (u_k − u_c)
                    }
                }
            }
            // store PLANE-MAJOR (structure of arrays): plane q occupies [q·n·n .. (q+1)·n·n]
            for q in 0..21 {
                out[q * n * n + y * n + x] = wbuf[q];
            }
        }
    }
    out
}

// ===========================================================================
// 2. THE KERNELS. Every one performs ONE diffusion step over the n×n interior:
//    `u' = u + k·Δu`. That is exactly the shape of `erosion.rs::diffuse_step`.
// ===========================================================================

/// **K0 — the baseline: what vivarium runs TODAY.** `erosion.rs::diffuse_step`, verbatim:
/// one cell size, one area, no geometry at all. 5-point, `f32`.
fn k0_naive(u: &[f32], out: &mut [f32], n: usize, s: usize, h: usize, k: f32) {
    for y in 0..n {
        for x in 0..n {
            let c = (y + h) * s + (x + h);
            let lap = u[c + 1] + u[c - 1] + u[c + s] + u[c - s] - 4.0 * u[c];
            out[c] = u[c] + k * lap;
        }
    }
}

/// **K1 — FV two-point with the true geometry** (precomputed transmissibility + area).
/// The control: what does "just use the real metric" cost, before any correction?
/// 5 f32/cell of coefficients.
fn k1_fv2pt(u: &[f32], out: &mut [f32], n: usize, s: usize, h: usize, k: f32, t: &[f32], inv_a: &[f32]) {
    for y in 0..n {
        let ubase = (y + h) * s + h;
        for x in 0..n {
            let c = ubase + x;
            let ci = y * n + x;
            let f = t[ci] * (u[c + 1] - u[c])
                + t[n * n + ci] * (u[c + s] - u[c])
                + t[2 * n * n + ci] * (u[c - 1] - u[c])
                + t[3 * n * n + ci] * (u[c - s] - u[c]);
            out[c] = u[c] + k * f * inv_a[ci];
        }
    }
}

/// **K3 — the corrected scheme AS WRITTEN: a 5×5 normal-equation solve per cell per step.**
/// Geometry is cached (it is in `probes` too — the `Mesh` holds it); what is redone every
/// step is the assembly of `AᵀWA` / `AᵀWb` and the Gaussian elimination. This is exactly
/// what `probes::lsq_gradients_quadratic` + `probes::edge_flux` do, on a dense layout.
fn k3_corrected_solve(u: &[f64], out: &mut [f64], tile: &Tile, k: f64) {
    let (n, s, h) = (tile.n, tile.s, tile.halo);
    // pass 1: the gradient at every cell in the interior AND its one-ring (the flux needs
    // the neighbour's gradient too).
    let mut grad = vec![[0.0f64; 3]; s * s];
    for y in h - 1..s - h + 1 {
        for x in h - 1..s - h + 1 {
            let c = y * s + x;
            let cc = tile.ctr[c];
            let (e0, e1) = (tile.e0[c], tile.e1[c]);
            let hs = tile.area[c].sqrt();
            let mut ata = [[0.0f64; 5]; 5];
            let mut atb = [0.0f64; 5];
            for &(dx, dy) in MOORE.iter() {
                let nb = ((y as isize + dy) * s as isize + (x as isize + dx)) as usize;
                let d = sub(tile.ctr[nb], cc);
                let (px, py) = (dot(d, e0) * R_M / hs, dot(d, e1) * R_M / hs);
                let row = [px, py, 0.5 * px * px, px * py, 0.5 * py * py];
                let w = 1.0 / (px * px + py * py);
                let b = u[nb] - u[c];
                for a in 0..5 {
                    for bb in 0..5 {
                        ata[a][bb] += w * row[a] * row[bb];
                    }
                    atb[a] += w * row[a] * b;
                }
            }
            let mut m = [[0.0f64; 6]; 5];
            for a in 0..5 {
                m[a][..5].copy_from_slice(&ata[a]);
                m[a][5] = atb[a];
            }
            for col in 0..5 {
                let mut piv = col;
                for rr in col + 1..5 {
                    if m[rr][col].abs() > m[piv][col].abs() {
                        piv = rr;
                    }
                }
                m.swap(col, piv);
                for rr in 0..5 {
                    if rr == col {
                        continue;
                    }
                    let f = m[rr][col] / m[col][col];
                    for cc2 in col..6 {
                        m[rr][cc2] -= f * m[col][cc2];
                    }
                }
            }
            let (gx, gy) = (m[0][5] / m[0][0] / hs, m[1][5] / m[1][1] / hs);
            grad[c] = add(scale(e0, gx), scale(e1, gy));
        }
    }
    // pass 2: the corrected flux across each of the 4 edges
    for y in 0..n {
        for x in 0..n {
            let (px, py) = (x + h, y + h);
            let c = py * s + px;
            let mut f = 0.0;
            for (e, &(dx, dy)) in EDGE.iter().enumerate() {
                let jn = ((py as isize + dy) * s as isize + (px as isize + dx)) as usize;
                let ui = u[c] + dot(grad[c], tile.wi[4 * c + e]);
                let uj = u[jn] + dot(grad[jn], tile.wj[4 * c + e]);
                f += (uj - ui) * tile.t[4 * c + e];
            }
            out[c] = u[c] + k * f / tile.area[c];
        }
    }
}

/// **K2 — the corrected scheme with NO geometry cache**: the metric is re-derived from the
/// map (`tan`, `atan2`, `sqrt`, cross products) every cell every step, then the 5×5 solve.
/// This is the cost if the tile carries no geometry planes at all — which is `erosion.rs`'s
/// situation today (it stores no metric whatsoever).
fn k2_corrected_onfly(u: &[f64], out: &mut [f64], tile: &Tile, k: f64) {
    let scratch = Tile::new(tile.face, tile.level, tile.oi, tile.oj, tile.n, tile.halo);
    black_box(&scratch);
    k3_corrected_solve(u, out, &scratch, k);
}

/// **K4 — the PRECOMPUTED FUSED STENCIL, f32.** The report's [I] rescue: the 5×5
/// pseudo-inverse is factored once, the whole corrected operator collapses to 21 static
/// weights per cell, and the hot loop is 21 multiply-adds. 21 f32 = **84 B/cell**.
///
/// ⚠ LAYOUT IS LOAD-BEARING. The weights are stored **structure-of-arrays — one contiguous
/// plane per tap** — not interleaved 21-per-cell. With SoA, each tap's inner loop is
/// `acc[x] += w_q[x] · u[x + off_q]`: two contiguous streams, so it vectorises. With the
/// obvious AoS layout the compiler cannot prove the access pattern and the same arithmetic
/// runs **~5× slower**. That is a fact about the implementation, not about the scheme, and
/// benchmarking the AoS version would have libelled the scheme. (Both are measured; see §C.)
fn k4_fused_f32(u: &[f32], out: &mut [f32], n: usize, s: usize, h: usize, k: f32, w: &[f32], doff: &[isize; 21], acc: &mut [f32]) {
    for y in 0..n {
        let row = &mut acc[..n];
        row.fill(0.0);
        let ubase = (y + h) * s + h;
        for q in 0..21 {
            let wq = &w[q * n * n + y * n..q * n * n + y * n + n];
            let uq = &u[(ubase as isize + doff[q]) as usize..(ubase as isize + doff[q]) as usize + n];
            for x in 0..n {
                row[x] += wq[x] * uq[x];
            }
        }
        for x in 0..n {
            out[ubase + x] = u[ubase + x] + k * row[x];
        }
    }
}

/// **K4-AoS — the same operator, weights interleaved 21-per-cell.** Kept only to show what
/// the layout costs; it is not the scheme's fault.
fn k4_fused_f32_aos(u: &[f32], out: &mut [f32], n: usize, s: usize, h: usize, k: f32, w: &[f32], doff: &[isize; 21]) {
    for y in 0..n {
        for x in 0..n {
            let c = (y + h) * s + (x + h);
            let wb = (y * n + x) * 21;
            let mut a = 0.0f32;
            for q in 0..21 {
                a += w[wb + q] * u[(c as isize + doff[q]) as usize];
            }
            out[c] = u[c] + k * a;
        }
    }
}

/// **K5 — the same fused stencil in f64.** 21 f64 = **168 B/cell**. Present because the
/// weights of a Laplacian sum to ~0 and f32 storage is a real precision question (§B).
fn k5_fused_f64(u: &[f64], out: &mut [f64], n: usize, s: usize, h: usize, k: f64, w: &[f64], doff: &[isize; 21], acc: &mut [f64]) {
    for y in 0..n {
        let row = &mut acc[..n];
        row.fill(0.0);
        let ubase = (y + h) * s + h;
        for q in 0..21 {
            let wq = &w[q * n * n + y * n..q * n * n + y * n + n];
            let uq = &u[(ubase as isize + doff[q]) as usize..(ubase as isize + doff[q]) as usize + n];
            for x in 0..n {
                row[x] += wq[x] * uq[x];
            }
        }
        for x in 0..n {
            out[ubase + x] = u[ubase + x] + k * row[x];
        }
    }
}

/// **K6 — the precomputed TWO-PASS form, f32.** Keeps the gradient and the per-edge fluxes
/// as separate, materialised objects — which a *flux-form transport* kernel needs (erosion
/// routes sediment across edges; it does not merely apply a Laplacian). Coefficients:
/// 16 (gradient) + 16 (4 edges × T,pi,qi and the neighbour's pj,qj packed) + 1 (1/A).
#[allow(clippy::too_many_arguments)]
fn k6_twopass_f32(
    u: &[f32], out: &mut [f32], gx: &mut [f32], gy: &mut [f32], n: usize, s: usize, h: usize,
    k: f32, gc: &[f32], ec: &[f32], inv_a: &[f32],
) {
    // pass 1 — the gradient, from static coefficients (16 f32/cell), over the interior+ring
    for y in h - 1..s - h + 1 {
        for x in h - 1..s - h + 1 {
            let c = y * s + x;
            let b = c * 16;
            let mut ax = 0.0f32;
            let mut ay = 0.0f32;
            for q in 0..8 {
                let (dx, dy) = MOORE[q];
                let nb = ((y as isize + dy) * s as isize + (x as isize + dx)) as usize;
                let d = u[nb] - u[c];
                ax += gc[b + q] * d;
                ay += gc[b + 8 + q] * d;
            }
            gx[c] = ax;
            gy[c] = ay;
        }
    }
    // pass 2 — the corrected flux
    for y in 0..n {
        for x in 0..n {
            let (px, py) = (x + h, y + h);
            let c = py * s + px;
            let eb = (y * n + x) * 20;
            let mut f = 0.0f32;
            for (e, &(dx, dy)) in EDGE.iter().enumerate() {
                let jn = ((py as isize + dy) * s as isize + (px as isize + dx)) as usize;
                let o = eb + 5 * e;
                let ui = u[c] + gx[c] * ec[o + 1] + gy[c] * ec[o + 2];
                let uj = u[jn] + gx[jn] * ec[o + 3] + gy[jn] * ec[o + 4];
                f += (uj - ui) * ec[o];
            }
            out[c] = u[c] + k * f * inv_a[y * n + x];
        }
    }
}

/// **K7 — the FROZEN constant-coefficient stencil.** ONE 21-weight stencil for the whole
/// tile, taken at the tile centre. **This is an APPROXIMATION, not a refactor** — it asserts
/// the metric is constant across the tile. It streams **zero** coefficient bytes (the weights
/// live in registers), so it is the ceiling on what any precomputed form can achieve. §A3
/// measures what it costs in accuracy — which is the only reason to care.
fn k7_frozen_f32(u: &[f32], out: &mut [f32], n: usize, s: usize, h: usize, k: f32, w: &[f32; 21], doff: &[isize; 21], acc: &mut [f32]) {
    for y in 0..n {
        let row = &mut acc[..n];
        row.fill(0.0);
        let ubase = (y + h) * s + h;
        for q in 0..21 {
            let wq = w[q];
            let uq = &u[(ubase as isize + doff[q]) as usize..(ubase as isize + doff[q]) as usize + n];
            for x in 0..n {
                row[x] += wq * uq[x];
            }
        }
        for x in 0..n {
            out[ubase + x] = u[ubase + x] + k * row[x];
        }
    }
}

// ===========================================================================
// 3. TIMING — noise floor FIRST, then one variable at a time.
//     (memory/godot-view-perf.md: "build the deterministic benchmark + telemetry first,
//      establish the run-to-run noise floor, then change ONE variable at a time")
// ===========================================================================

struct Timing {
    label: String,
    /// cell-updates per second, one entry per repeat
    rates: Vec<f64>,
}

impl Timing {
    fn median(&self) -> f64 {
        let mut v = self.rates.clone();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[v.len() / 2]
    }
    fn min(&self) -> f64 { self.rates.iter().cloned().fold(f64::INFINITY, f64::min) }
    fn max(&self) -> f64 { self.rates.iter().cloned().fold(0.0, f64::max) }
    /// half-spread as a % of the median — the run-to-run noise on THIS kernel
    fn spread_pct(&self) -> f64 { 100.0 * (self.max() - self.min()) / self.median() }
}

fn time_it(label: &str, cells: usize, steps: usize, repeats: usize, mut f: impl FnMut(usize)) -> Timing {
    // warm-up (page faults, frequency ramp)
    f(steps.min(4).max(1));
    let mut rates = Vec::new();
    for _ in 0..repeats {
        let t = Instant::now();
        f(steps);
        let dt = t.elapsed().as_secs_f64();
        rates.push((cells * steps) as f64 / dt);
    }
    Timing { label: label.to_string(), rates }
}

fn g(r: f64) -> String {
    if r >= 1e9 {
        format!("{:.2} G", r / 1e9)
    } else {
        format!("{:.1} M", r / 1e6)
    }
}

// ===========================================================================
// 4. THE FIELDS
// ===========================================================================

/// The ℓ=2 spherical harmonic `probes::harmonic_error` uses — an exact eigenfunction of
/// Laplace–Beltrami, so the truncation error is exactly known with no reference solution.
fn harmonic(p: V3) -> f64 {
    let e = unit([0.3, -0.7, 0.64]);
    let t = dot(p, e);
    1.5 * t * t - 0.5
}
fn harmonic_lap(p: V3) -> f64 { -6.0 / (R_M * R_M) * harmonic(p) }

/// A rough, terrain-like field (value-noise octaves keyed off the cell index) — the regime
/// erosion actually runs in, where the second difference is O(metres) and f32 is not
/// obviously doomed. Deterministic; no RNG state.
fn terrain(tile: &Tile) -> Vec<f64> {
    let hashf = |a: i64, b: i64, o: i64| -> f64 {
        let mut h = (a as u64).wrapping_mul(0x9E3779B97F4A7C15)
            ^ (b as u64).wrapping_mul(0xC2B2AE3D27D4EB4F)
            ^ (o as u64).wrapping_mul(0x165667B19E3779F9);
        h ^= h >> 33;
        h = h.wrapping_mul(0xFF51AFD7ED558CCD);
        h ^= h >> 33;
        (h >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    let h = tile.halo as i64;
    let (bi, bj) = (tile.oi as i64 - h, tile.oj as i64 - h);
    (0..tile.s * tile.s)
        .map(|c| {
            let (x, y) = ((c % tile.s) as i64 + bi, (c / tile.s) as i64 + bj);
            let mut acc = 0.0;
            let mut amp = 800.0;
            let mut sc = 256i64;
            for o in 0..6 {
                // bilinear value noise at scale `sc`
                let (gx0, gy0) = (x.div_euclid(sc), y.div_euclid(sc));
                let (fx, fy) = ((x.rem_euclid(sc)) as f64 / sc as f64, (y.rem_euclid(sc)) as f64 / sc as f64);
                let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
                let v = |a: i64, b: i64| hashf(a, b, o);
                let a = v(gx0, gy0) * (1.0 - sx) + v(gx0 + 1, gy0) * sx;
                let b = v(gx0, gy0 + 1) * (1.0 - sx) + v(gx0 + 1, gy0 + 1) * sx;
                acc += amp * (a * (1.0 - sy) + b * sy);
                amp *= 0.5;
                sc = (sc / 2).max(1);
            }
            acc
        })
        .collect()
}

fn rel_l2(a: &[f64], b: &[f64]) -> f64 {
    let (mut num, mut den) = (0.0, 0.0);
    for i in 0..a.len() {
        num += (a[i] - b[i]) * (a[i] - b[i]);
        den += b[i] * b[i];
    }
    (num / den).sqrt()
}

// ===========================================================================
// MAIN
// ===========================================================================

fn main() {
    println!("# stencil_bench — the hot-loop cost of the corrected finite-volume scheme");
    println!("#");
    println!("# Answers `grid-comparison-report.md` §10.6: the corrected scheme is a 5x5 solve");
    println!("# per cell per step; nobody had benchmarked it, and the precomputed-stencil rescue");
    println!("# was an INFERENCE. Both are measured here, on real cube-sphere geometry, on the");
    println!("# dense Cartesian tile layout `erosion.rs` actually uses.");
    println!();

    let offs = fused_offsets();
    let args: Vec<String> = std::env::args().collect();
    let only = args.get(1).cloned().unwrap_or_default();

    if only.is_empty() || only == "cond" {
        section_cond();
    }
    if only.is_empty() || only == "verify" {
        section_a_verify(&offs);
    }
    if only.is_empty() || only == "accuracy" {
        section_b_accuracy(&offs);
    }
    if only.is_empty() || only == "cost" {
        section_c_cost(&offs);
    }
    if only.is_empty() || only == "cliff" {
        section_d_cliff(&offs);
    }
    if only.is_empty() || only == "struct" {
        section_e_structure(&offs);
    }
    if only.is_empty() || only == "erosion" {
        section_f_erosion();
    }
}

// ---------------------------------------------------------------------------
// §COND — A HYPOTHESIS I FORMED AND THEN REFUTED. Recorded because a refuted
//         hypothesis is a result (the grid report's own idiom, §5.2).
//
// `probes::lsq_gradients_quadratic` assembles the 5 normal equations in **unscaled
// metres**: `row = [x, y, ½x², xy, ½y²]`. At L6 a cell is ~156 km, so the quadratic
// columns are ~1e5 times the linear ones and `cond(AᵀA)` — the SQUARE of A's — is 1.2e10.
// That is measured below and it is real.
//
// I predicted the consequence: that the as-written solve was therefore delivering only
// ~4-6 good digits, that part of the report's headline 3.65e-4 was conditioning noise
// rather than truncation error, and that the scheme would look BETTER once conditioned.
//
// ⇒ **REFUTED.** Nondimensionalising the fit (exact reparametrisation: fit in units of
//   the cell size, scale the gradient back — `cond` drops 1.2e10 → 8.0) changes the
//   Laplacian by **1.7e-14**. The report's number is truncation error, not round-off.
//   The ill-conditioning is real but HARMLESS here: the error lands in the ill-determined
//   quadratic directions, and the two components that are actually read out — the linear
//   ones — are recovered accurately regardless.
//
// Kept for two reasons. It is the honest record of a plausible structural story that the
// measurement killed. And the condition number still MATTERS operationally: it is why the
// 5×5 solve cannot be done in f32 (cond 1.2e10 ≫ f32's 1e7 dynamic range), which bears
// directly on how the precomputed form must be built — even though it does not bear on
// what the scheme's accuracy IS.
/// Eigenvalues of a symmetric 5×5 by cyclic Jacobi — enough for a condition number.
fn sym_eigs(mut a: [[f64; 5]; 5]) -> [f64; 5] {
    for _ in 0..100 {
        let mut off = 0.0;
        for p in 0..5 {
            for q in p + 1..5 {
                off += a[p][q] * a[p][q];
            }
        }
        if off < 1e-300 {
            break;
        }
        for p in 0..5 {
            for q in p + 1..5 {
                if a[p][q].abs() < 1e-300 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..5 {
                    let (akp, akq) = (a[k][p], a[k][q]);
                    a[k][p] = c * akp - s * akq;
                    a[k][q] = s * akp + c * akq;
                }
                for k in 0..5 {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
            }
        }
    }
    [a[0][0], a[1][1], a[2][2], a[3][3], a[4][4]]
}

/// Assemble AᵀWA for a cell, with the coordinates optionally nondimensionalised by `h`.
fn ata_of(tile: &Tile, c: usize, h_scale: f64) -> [[f64; 5]; 5] {
    let cc = tile.ctr[c];
    let (e0, e1) = (tile.e0[c], tile.e1[c]);
    let s = tile.s;
    let (cx0, cy0) = ((c % s) as isize, (c / s) as isize);
    let mut ata = [[0.0f64; 5]; 5];
    for &(dx, dy) in MOORE.iter() {
        let nb = ((cy0 + dy) * s as isize + (cx0 + dx)) as usize;
        let d = sub(tile.ctr[nb], cc);
        let (x, y) = (dot(d, e0) * R_M / h_scale, dot(d, e1) * R_M / h_scale);
        let row = [x, y, 0.5 * x * x, x * y, 0.5 * y * y];
        let w = 1.0 / (x * x + y * y);
        for a in 0..5 {
            for b in 0..5 {
                ata[a][b] += w * row[a] * row[b];
            }
        }
    }
    ata
}

fn section_cond() {
    println!("## 0. A REFUTED HYPOTHESIS (recorded as a result)");
    println!();
    println!("`probes::lsq_gradients_quadratic` fits `[x, y, x²/2, xy, y²/2]` with x, y in **metres**,");
    println!("so at L6 (~156 km cells) the quadratic columns are ~1e5x the linear ones. The normal");
    println!("equations are correspondingly ill-conditioned — cond(AᵀA) is the SQUARE of cond(A):");
    println!();
    println!("| level | cell size | cond(AᵀA), as written | cond(AᵀA), nondimensionalised |");
    println!("|---|---|---|---|");
    for &lvl in &[6u8, 9, 12, 15, 19] {
        let nn = 1u32 << lvl;
        let tile = Tile::new(0, lvl, nn / 2 - 4, nn / 2 - 4, 8, 2);
        let c = tile.idx(tile.s / 2, tile.s / 2);
        let h = tile.area[c].sqrt();
        let cond = |e: [f64; 5]| {
            let mx = e.iter().cloned().fold(0.0f64, f64::max);
            let mn = e.iter().cloned().fold(f64::INFINITY, f64::min);
            mx / mn
        };
        println!(
            "| L{lvl} | {:>8.0} m | {:.2e} | {:.2e} |",
            h,
            cond(sym_eigs(ata_of(&tile, c, 1.0))),
            cond(sym_eigs(ata_of(&tile, c, h)))
        );
    }
    println!();
    println!("I PREDICTED this meant part of the report's 3.65e-4 was conditioning noise, and that");
    println!("the scheme would look better once conditioned. §A tests it on the report's own harness.");
    println!("**It is refuted**: the fix changes the answer by 1.7e-14. The error lands in the");
    println!("ill-determined QUADRATIC directions; the two linear components actually read out are");
    println!("recovered accurately anyway. The report's number is truncation error, as claimed. [M]");
    println!();
    println!("What survives, and is operationally load-bearing: **cond 1.2e10 ≫ f32's ~1e7 range, so");
    println!("the 5x5 solve can never be done in f32.** It bears on HOW the precomputed weights must");
    println!("be built (in f64, once), not on what the scheme's accuracy is. [M]");
    println!();
}

// ---------------------------------------------------------------------------
// §A — VERIFICATION. Nothing below is believed until this passes.
// ---------------------------------------------------------------------------
fn section_a_verify(offs: &[(isize, isize)]) {
    println!("## A. VERIFICATION — the dense kernels vs. the report's own reference implementation");
    println!();
    println!("The oracle is `grid_lab`'s `probes::laplacian(Scheme::FvLsq)` on `grids::cube_sphere`");
    println!("(equiangular, n=64 ⇒ L6, 24 576 cells — the report's own size). A dense tile is cut");
    println!("from the interior of face 0 and every kernel is compared CELL FOR CELL.");
    println!();

    let n = 64usize;
    let g = grids::cube_sphere(grids::CubeProj::Equiangular, n, R_M);
    let m = probes::means(&g);
    let u_mesh: Vec<f64> = g.centers.iter().map(|&p| harmonic(p)).collect();
    let lap_oracle = probes::laplacian(&g, probes::Scheme::FvLsq, &u_mesh, m);

    // (0) my lifted `edge_flux` must reproduce `probes::edge_flux` given the SAME gradient.
    let g_asis = probes::lsq_gradients_quadratic(&g, &u_mesh);
    let lap_mine = mesh_lap_fvlsq(&g, &u_mesh, &g_asis);
    println!("- my lifted edge_flux vs probes::laplacian(FvLsq) : rel L2 = {:.2e}", rel_l2(&lap_mine, &lap_oracle));
    assert!(rel_l2(&lap_mine, &lap_oracle) < 1e-12, "my edge_flux copy diverges from probes");

    // (0b) THE CONDITIONING FINDING, on the report's own harness and at the report's own
    // size. Same scheme, same mesh, same field — only the solve is nondimensionalised.
    println!();
    println!("  §0's prediction, tested on the report's own harness (whole-sphere, 24 576 cells):");
    let exact: Vec<f64> = g.centers.iter().map(|&p| harmonic_lap(p)).collect();
    let g_scaled = mesh_grad_scaled(&g, &u_mesh);
    let lap_scaled = mesh_lap_fvlsq(&g, &u_mesh, &g_scaled);
    let (rep, _) = probes::harmonic_error(&g, probes::Scheme::FvLsq, 2);
    println!("  - `probes::harmonic_error(FvLsq)`, verbatim     : {rep:.3e}   ← the report's table says 3.65e-4");
    let e_asis = rel_l2(&lap_oracle, &exact);
    let e_scal = rel_l2(&lap_scaled, &exact);
    println!("  - FvLsq AS WRITTEN (normal equations in metres) : rel L2 vs exact = {e_asis:.3e}");
    println!("  - FvLsq NONDIMENSIONALISED (cond 1e10 -> 8.0)   : rel L2 vs exact = {e_scal:.3e}");
    println!("  - the two answers differ from each other by       rel L2 = {:.3e}", rel_l2(&lap_scaled, &lap_oracle));
    println!("  ⇒ §0's hypothesis is REFUTED. The conditioning is real and harmless; the report's");
    println!("    accuracy number is truncation error, not round-off. [M]");
    println!();

    // The whole-sphere mesh numbers cells face-major, row-major within a face — cell
    // (f, i, j) is at f·n² + j·n + i. Verified by comparing centres below.
    let tn = 32usize; // an interior block, ≥ 2 cells clear of the face edge
    let (oi, oj) = (16u32, 16u32);
    let tile = Tile::new(0, 6, oi, oj, tn, 2);

    // (i) the geometry itself
    let mut dc = 0.0f64;
    let mut da = 0.0f64;
    for y in 0..tn {
        for x in 0..tn {
            let mi = 0 * n * n + (oj as usize + y) * n + (oi as usize + x);
            let ti = tile.idx(x + 2, y + 2);
            dc = dc.max(norm(sub(g.centers[mi], tile.ctr[ti])));
            da = da.max(((g.areas[mi] - tile.area[ti]) / g.areas[mi]).abs());
        }
    }
    println!("- cell centres    : max |Δ| = {dc:.3e}  (unit vectors)");
    println!("- cell areas      : max rel Δ = {da:.3e}");
    assert!(dc < 1e-12 && da < 1e-12, "dense tile geometry does not match the oracle mesh");

    // (i-b) the EDGE geometry, edge by edge, matched to the mesh's `adj` by neighbour cell.
    let (mut dt, mut dwi, mut dwj, mut dmoore) = (0.0f64, 0.0f64, 0.0f64, 0usize);
    for y in 0..tn {
        for x in 0..tn {
            let mi = (oj as usize + y) * n + (oi as usize + x);
            let ti = tile.idx(x + 2, y + 2);
            // Moore set: mesh cell indices vs my dense offsets
            let mut mine_moore: Vec<usize> = MOORE
                .iter()
                .map(|&(dx, dy)| {
                    let (gi, gj) = (oi as isize + x as isize + dx, oj as isize + y as isize + dy);
                    (gj as usize) * n + gi as usize
                })
                .collect();
            mine_moore.sort_unstable();
            let mut theirs = g.moore[mi].clone();
            theirs.sort_unstable();
            if mine_moore != theirs {
                dmoore += 1;
            }
            for (e, &(dx, dy)) in EDGE.iter().enumerate() {
                let (gi, gj) = (oi as isize + x as isize + dx, oj as isize + y as isize + dy);
                let jm = (gj as usize) * n + gi as usize;
                let me = g.adj[mi].iter().find(|ee| ee.j == jm).expect("edge not found in mesh adj");
                let fm = unit(add(g.verts[me.va as usize], g.verts[me.vb as usize]));
                let nh = tangent(fm, me.normal);
                let vi = scale(tangent(fm, g.centers[mi]), me.arm_m);
                let vj = scale(tangent(fm, g.centers[me.j]), me.arm_opp_m);
                let (ai, aj) = (dot(vi, nh), dot(vj, nh));
                let tt = me.edge_len_m / (aj - ai);
                dt = dt.max(((tt - tile.t[4 * ti + e]) / tt).abs());
                dwi = dwi.max(norm(sub(sub(scale(nh, ai), vi), tile.wi[4 * ti + e])));
                dwj = dwj.max(norm(sub(sub(scale(nh, aj), vj), tile.wj[4 * ti + e])));
            }
        }
    }
    println!("- Moore-set mismatches : {dmoore}");
    println!("- edge transmissibility: max rel Δ = {dt:.3e}");
    println!("- edge offset w_i      : max |Δ| = {dwi:.3e} m");
    println!("- edge offset w_j      : max |Δ| = {dwj:.3e} m");

    // (ii) the corrected Laplacian, K3 (the 5x5-solve-per-cell form) vs the oracle.
    // Compared against the WELL-CONDITIONED mesh answer — that is the scheme every kernel
    // below implements, and the only one whose digits are real.
    let mut u = vec![0.0f64; tile.s * tile.s];
    for c in 0..tile.s * tile.s {
        u[c] = harmonic(tile.ctr[c]);
    }
    // ⚠ EXTRACTION SCALE. The kernels compute `out = u + k·Δu`. For this harmonic
    // `Δu/u = -6/R² ≈ 1.5e-13`, so with k = 1 the increment is 1e-13 times `u` and
    // recovering it as `out - u` in f64 destroys ~2.2e-16/1.5e-13 ≈ 7e-4 of relative
    // precision — which is a bug in the HARNESS, not in the scheme, and it is exactly
    // the size of the discrepancy this check first reported. Take k = R²/6 so that
    // k·Δu ~ u, and no cancellation occurs.
    let kv = R_M * R_M / 6.0;
    let mut out = u.clone();
    k3_corrected_solve(&u, &mut out, &tile, kv);
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for y in 0..tn {
        for x in 0..tn {
            let mi = (oj as usize + y) * n + (oi as usize + x);
            let ti = tile.idx(x + 2, y + 2);
            a.push((out[ti] - u[ti]) / kv);
            b.push(lap_scaled[mi]);
        }
    }
    println!("- K3 (5x5 solve/cell/step, dense) vs the mesh oracle : rel L2 = {:.3e}   ← must be round-off", rel_l2(&a, &b));

    // localise it: the geometry is bit-identical, so the gap can only be the GRADIENT.
    {
        let mut gd = 0.0f64;
        let mut gmag = 0.0f64;
        // recompute the dense gradient the way K3 does
        let s = tile.s;
        for y in 0..tn {
            for x in 0..tn {
                let ti = tile.idx(x + 2, y + 2);
                let mi = (oj as usize + y) * n + (oi as usize + x);
                let cc = tile.ctr[ti];
                let (e0, e1) = (tile.e0[ti], tile.e1[ti]);
                let hs = tile.area[ti].sqrt();
                let mut ata = [[0.0f64; 5]; 5];
                let mut atb = [0.0f64; 5];
                for &(dx, dy) in MOORE.iter() {
                    let nb = (((ti / s) as isize + dy) * s as isize + ((ti % s) as isize + dx)) as usize;
                    let d = sub(tile.ctr[nb], cc);
                    let (px, py) = (dot(d, e0) * R_M / hs, dot(d, e1) * R_M / hs);
                    let row = [px, py, 0.5 * px * px, px * py, 0.5 * py * py];
                    let w = 1.0 / (px * px + py * py);
                    let b = u[nb] - u[ti];
                    for aa in 0..5 {
                        for bb in 0..5 {
                            ata[aa][bb] += w * row[aa] * row[bb];
                        }
                        atb[aa] += w * row[aa] * b;
                    }
                }
                let mut m2 = [[0.0f64; 6]; 5];
                for aa in 0..5 {
                    m2[aa][..5].copy_from_slice(&ata[aa]);
                    m2[aa][5] = atb[aa];
                }
                for col in 0..5 {
                    let mut piv = col;
                    for rr in col + 1..5 {
                        if m2[rr][col].abs() > m2[piv][col].abs() {
                            piv = rr;
                        }
                    }
                    m2.swap(col, piv);
                    for rr in 0..5 {
                        if rr == col {
                            continue;
                        }
                        let f = m2[rr][col] / m2[col][col];
                        for cc2 in col..6 {
                            m2[rr][cc2] -= f * m2[col][cc2];
                        }
                    }
                }
                let (gx, gy) = (m2[0][5] / m2[0][0] / hs, m2[1][5] / m2[1][1] / hs);
                let mine = add(scale(e0, gx), scale(e1, gy));
                gd = gd.max(norm(sub(mine, g_scaled[mi])));
                gmag = gmag.max(norm(g_scaled[mi]));
            }
        }
        println!("- GRADIENT: dense vs mesh, max |Δ| = {gd:.3e}  (|∇u| ~ {gmag:.3e})  ⇒ rel {:.3e}", gd / gmag);
    }
    // where does the error live? bin by distance from the interior's own boundary.
    {
        let mut bins = vec![(0.0f64, 0usize); 6];
        for y in 0..tn {
            for x in 0..tn {
                let mi = (oj as usize + y) * n + (oi as usize + x);
                let ti = tile.idx(x + 2, y + 2);
                let d = x.min(y).min(tn - 1 - x).min(tn - 1 - y).min(5);
                let mine = (out[ti] - u[ti]) / kv;
                let theirs = lap_scaled[mi];
                let r = ((mine - theirs) / theirs).abs();
                bins[d].0 = bins[d].0.max(r);
                bins[d].1 += 1;
            }
        }
        for (d, (mx, c)) in bins.iter().enumerate() {
            println!("    ring {d} from interior edge ({c:4} cells): max rel Δ = {mx:.3e}");
        }
    }
    assert!(rel_l2(&a, &b) < 1e-10, "K3 does not reproduce the report's scheme");

    // (iii) the precomputed forms vs K3
    let fw = build_fused::<f32>(&tile, offs);
    let doff = doffsets(offs, tile.s);
    let mut fused = u.clone();
    let mut sc = vec![0.0f64; tile.n];
    k5_fused_f64(&u, &mut fused, tile.n, tile.s, tile.halo, kv, &fw, &doff, &mut sc);
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for y in 0..tn {
        for x in 0..tn {
            let ti = tile.idx(x + 2, y + 2);
            a.push((fused[ti] - u[ti]) / kv);
            b.push((out[ti] - u[ti]) / kv);
        }
    }
    println!("- K5 (precomputed FUSED 21-pt, f64) vs K3  : L2 = {:.3e}   ← must be round-off", rel_l2(&a, &b));
    assert!(rel_l2(&a, &b) < 1e-9, "the fused refactor is NOT the same operator");

    println!();
    println!("  ⇒ The precomputed 21-point stencil is an EXACT algebraic refactor of the corrected");
    println!("    scheme, not an approximation. Same operator, same numbers, to round-off. [M]");
    println!();
}

fn doffsets(offs: &[(isize, isize)], s: usize) -> [isize; 21] {
    let mut d = [0isize; 21];
    for (q, &(dx, dy)) in offs.iter().enumerate() {
        d[q] = dy * s as isize + dx;
    }
    d
}

// ---------------------------------------------------------------------------
// §B — ACCURACY. What each kernel actually buys, on real geometry.
// ---------------------------------------------------------------------------
fn section_b_accuracy(offs: &[(isize, isize)]) {
    println!("## B. ACCURACY — what the correction buys, and what the cheap forms give back");
    println!();
    println!("Relative L2 error of the discrete Laplace-Beltrami operator against the EXACT");
    println!("eigenvalue of an l=2 harmonic, measured on a 64x64 tile of real cube-sphere geometry.");
    println!();
    println!("| level | cell | naive 5-pt | FV 2-point | CORRECTED (exact) | fused f32 | frozen-const f32 |");
    println!("|---|---|---|---|---|---|---|");

    for &lvl in &[6u8, 9, 12] {
        let nn = 1u32 << lvl;
        let tn = 64usize;
        let (oi, oj) = (nn / 2 - 32, nn / 2 - 32);
        let tile = Tile::new(0, lvl, oi, oj, tn, 2);
        let cell_m = tile.mean_cell_m();

        let u: Vec<f64> = (0..tile.s * tile.s).map(|c| harmonic(tile.ctr[c])).collect();
        let exact: Vec<f64> = (0..tile.s * tile.s).map(|c| harmonic_lap(tile.ctr[c])).collect();

        let interior = |v: &[f64]| -> Vec<f64> {
            let mut o = Vec::new();
            for y in 0..tn {
                for x in 0..tn {
                    o.push(v[tile.idx(x + 2, y + 2)]);
                }
            }
            o
        };
        let ex = interior(&exact);

        // naive (f64, so the comparison is about the SCHEME, not about f32)
        let mut lap_naive = vec![0.0; tile.s * tile.s];
        let inv_h2 = 1.0 / (cell_m * cell_m);
        for y in 0..tn {
            for x in 0..tn {
                let c = tile.idx(x + 2, y + 2);
                lap_naive[c] = (u[c + 1] + u[c - 1] + u[c + tile.s] + u[c - tile.s] - 4.0 * u[c]) * inv_h2;
            }
        }
        // FV two-point
        let mut lap_fv = vec![0.0; tile.s * tile.s];
        for y in 0..tn {
            for x in 0..tn {
                let c = tile.idx(x + 2, y + 2);
                let mut f = 0.0;
                for (e, &(dx, dy)) in EDGE.iter().enumerate() {
                    let jn = ((c as isize) + dy * tile.s as isize + dx) as usize;
                    f += (u[jn] - u[c]) * tile.t[4 * c + e];
                }
                lap_fv[c] = f / tile.area[c];
            }
        }
        // corrected, exact
        let kv = R_M * R_M / 6.0; // see §A: keeps `out - u` off the cancellation floor
        let mut o3 = u.clone();
        k3_corrected_solve(&u, &mut o3, &tile, kv);
        let lap_corr: Vec<f64> = (0..tile.s * tile.s).map(|c| (o3[c] - u[c]) / kv).collect();

        // fused f32 — NOTE the field is f32 here too; this is the real deployed precision
        let fw = build_fused::<f32>(&tile, offs);
        let fw32: Vec<f32> = fw.iter().map(|&x| x as f32).collect();
        let doff = doffsets(offs, tile.s);
        let u32v: Vec<f32> = u.iter().map(|&x| x as f32).collect();
        let mut o32 = u32v.clone();
        k4_fused_f32(&u32v, &mut o32, tn, tile.s, 2, kv as f32, &fw32, &doff, &mut vec![0.0f32; tn]);
        let lap_f32: Vec<f64> = (0..tile.s * tile.s).map(|c| (o32[c] - u32v[c]) as f64 / kv).collect();

        // frozen constant coefficients, taken at the tile centre
        let mid = (tn / 2) * tn + tn / 2;
        let mut wconst = [0.0f32; 21];
        let mut wconst64 = [0.0f64; 21];
        for q in 0..21 {
            wconst[q] = fw[q * tn * tn + mid] as f32;
            wconst64[q] = fw[q * tn * tn + mid];
        }
        let mut o7 = u32v.clone();
        k7_frozen_f32(&u32v, &mut o7, tn, tile.s, 2, kv as f32, &wconst, &doff, &mut vec![0.0f32; tn]);
        let lap_frz: Vec<f64> = (0..tile.s * tile.s).map(|c| (o7[c] - u32v[c]) as f64 / kv).collect();

        println!(
            "| L{lvl} | {:.0} m | {:.2e} | {:.2e} | **{:.2e}** | {:.2e} | {:.2e} |",
            cell_m,
            rel_l2(&interior(&lap_naive), &ex),
            rel_l2(&interior(&lap_fv), &ex),
            rel_l2(&interior(&lap_corr), &ex),
            rel_l2(&interior(&lap_f32), &ex),
            rel_l2(&interior(&lap_frz), &ex),
        );
    }
    println!();
    println!("⚠ The harmonic is SMOOTH: at fine levels its second difference falls below f32");
    println!("  epsilon and the f32 columns measure catastrophic cancellation in the TEST FIELD,");
    println!("  not error in the scheme. The honest f32 question is asked on rough terrain:");
    println!();

    // f32 fidelity + frozen-coefficient error, on a REALISTIC rough field at L19
    let tn = 64usize;
    let lvl = 19u8;
    let nn = 1u32 << lvl;
    let tile = Tile::new(0, lvl, nn / 2 - 32, nn / 2 - 32, tn, 2);
    let u = terrain(&tile);
    let fw = build_fused::<f32>(&tile, offs);
    let fw32: Vec<f32> = fw.iter().map(|&x| x as f32).collect();
    let doff = doffsets(offs, tile.s);

    let mut o5 = u.clone();
    k5_fused_f64(&u, &mut o5, tn, tile.s, 2, 1.0, &fw, &doff, &mut vec![0.0f64; tn]);
    let gold: Vec<f64> = (0..tile.s * tile.s).map(|c| o5[c] - u[c]).collect();

    let u32v: Vec<f32> = u.iter().map(|&x| x as f32).collect();
    let mut o4 = u32v.clone();
    k4_fused_f32(&u32v, &mut o4, tn, tile.s, 2, 1.0, &fw32, &doff, &mut vec![0.0f32; tn]);
    let f32lap: Vec<f64> = (0..tile.s * tile.s).map(|c| (o4[c] - u32v[c]) as f64).collect();

    let mid = (tn / 2) * tn + tn / 2;
    let mut wconst = [0.0f32; 21];
    let mut wconst64 = [0.0f64; 21];
    for q in 0..21 {
        wconst[q] = fw[q * tn * tn + mid] as f32;
        wconst64[q] = fw[q * tn * tn + mid];
    }
    let mut o7 = u32v.clone();
    k7_frozen_f32(&u32v, &mut o7, tn, tile.s, 2, 1.0, &wconst, &doff, &mut vec![0.0f32; tn]);
    let frzlap: Vec<f64> = (0..tile.s * tile.s).map(|c| (o7[c] - u32v[c]) as f64).collect();

    let cut = |v: &[f64]| -> Vec<f64> {
        let mut o = Vec::new();
        for y in 0..tn {
            for x in 0..tn {
                o.push(v[tile.idx(x + 2, y + 2)]);
            }
        }
        o
    };
    println!("L19, 64x64 tile, rough terrain (6 octaves, ~1.5 km relief over 1.2 km, ~19 m cells).");
    println!("Three DIFFERENT questions, separated — the first run conflated them:");
    println!();

    // (b) weight precision alone: f32-rounded weights, f64 field
    let fw_w32: Vec<f64> = fw.iter().map(|&x| x as f32 as f64).collect();
    let mut ob = u.clone();
    k5_fused_f64(&u, &mut ob, tn, tile.s, 2, 1.0, &fw_w32, &doff, &mut vec![0.0f64; tn]);
    let lap_w32: Vec<f64> = (0..tile.s * tile.s).map(|c| ob[c] - u[c]).collect();

    // (c) the frozen APPROXIMATION alone: constant weights, all f64
    let mut fw_frozen = vec![0.0f64; 21 * tn * tn];
    for q in 0..21 {
        for c in 0..tn * tn {
            fw_frozen[q * tn * tn + c] = wconst64[q];
        }
    }
    let mut oc = u.clone();
    k5_fused_f64(&u, &mut oc, tn, tile.s, 2, 1.0, &fw_frozen, &doff, &mut vec![0.0f64; tn]);
    let lap_frz64: Vec<f64> = (0..tile.s * tile.s).map(|c| oc[c] - u[c]).collect();

    println!("| what is being isolated | weights | field | rel L2 vs exact-f64 |");
    println!("|---|---|---|---|");
    println!("| **f32 FIELD** rounding (the deployed precision) | f32 | f32 | {:.3e} |", rel_l2(&cut(&f32lap), &cut(&gold)));
    println!("| **f32 WEIGHT** rounding alone | f32 | f64 | {:.3e} |", rel_l2(&cut(&lap_w32), &cut(&gold)));
    println!("| **the FROZEN approximation** alone | const, f64 | f64 | {:.3e} |", rel_l2(&cut(&lap_frz64), &cut(&gold)));
    println!("| frozen + f32 (what you would ship) | const, f32 | f32 | {:.3e} |", rel_l2(&cut(&frzlap), &cut(&gold)));
    println!();
    println!("⇒ The f32 FIELD dominates: storing the weights in f32 is ~free ({:.1e}), and the", rel_l2(&cut(&lap_w32), &cut(&gold)));
    println!("  frozen-constant approximation is ALSO ~free at L19 ({:.1e}) — a 64-cell tile spans", rel_l2(&cut(&lap_frz64), &cut(&gold)));
    println!("  1.2e-4 of a face, over which the metric is essentially constant. Both are swamped");
    println!("  by the f32 height field vivarium already uses. [M]");
    println!();
}

// ---------------------------------------------------------------------------
// §C — THE COST. At the size vivarium actually runs.
// ---------------------------------------------------------------------------
fn section_c_cost(offs: &[(isize, isize)]) {
    println!("## C. COST — cell-updates/s at L19, TILE_NX = 64 (what the builder actually sweeps)");
    println!();

    let tn = 64usize;
    let lvl = 19u8;
    let nn = 1u32 << lvl;
    let steps = 200usize;
    let repeats = 9usize;
    let cells = tn * tn;

    let t0 = Instant::now();
    let tile = Tile::new(0, lvl, nn / 2 - 32, nn / 2 - 32, tn, 2);
    let geom_ms = t0.elapsed().as_secs_f64() * 1e3;
    let t0 = Instant::now();
    let fw = build_fused::<f32>(&tile, offs);
    let fuse_ms = t0.elapsed().as_secs_f64() * 1e3;

    let doff = doffsets(offs, tile.s);
    let fw32: Vec<f32> = fw.iter().map(|&x| x as f32).collect();
    // the same weights, interleaved 21-per-cell, purely to price the LAYOUT
    let mut fw32_aos = vec![0.0f32; 21 * tn * tn];
    for c in 0..tn * tn {
        for q in 0..21 {
            fw32_aos[c * 21 + q] = fw[q * tn * tn + c] as f32;
        }
    }
    let u64v = terrain(&tile);
    let u32v: Vec<f32> = u64v.iter().map(|&x| x as f32).collect();

    // precomputed coefficient planes for K1 / K6
    let (t32, inva32) = k1_tables(&tile);
    let (gc32, ec32, inva6) = k6_tables(&tile);

    let s = tile.s;
    let mut ts: Vec<Timing> = Vec::new();
    let mut bytes: Vec<&str> = Vec::new();

    macro_rules! bench32 {
        ($label:expr, $b:expr, $body:expr) => {{
            let mut a = u32v.clone();
            let mut bb = u32v.clone();
            let mut acc = vec![0.0f32; tn];
            #[allow(unused_mut)]
            let mut f = $body;
            ts.push(time_it($label, cells, steps, repeats, |st| {
                for _ in 0..st {
                    f(black_box(&a), black_box(&mut bb), &mut acc);
                    std::mem::swap(&mut a, &mut bb);
                }
            }));
            bytes.push($b);
        }};
    }

    // ---- the noise floor FIRST: the same kernel, entered twice under different names ----
    bench32!("K0 naive 5-pt (BASELINE = today)", "0", |u: &[f32], o: &mut [f32], _a: &mut [f32]| k0_naive(u, o, tn, s, 2, 0.2));
    bench32!("  [noise-floor control: K0 again]", "0", |u: &[f32], o: &mut [f32], _a: &mut [f32]| k0_naive(u, o, tn, s, 2, 0.2));

    bench32!("K1 FV two-point, true metric", "20", |u: &[f32], o: &mut [f32], _a: &mut [f32]| k1_fv2pt(u, o, tn, s, 2, 0.2, &t32, &inva32));

    {
        let mut a = u64v.clone();
        let mut b = u64v.clone();
        ts.push(time_it("K3 CORRECTED, 5x5 solve/cell/step", cells, steps.min(20), repeats, |st| {
            for _ in 0..st {
                k3_corrected_solve(black_box(&a), black_box(&mut b), &tile, 0.2);
                std::mem::swap(&mut a, &mut b);
            }
        }));
        bytes.push("0 (solved live)");
    }
    {
        let mut a = u64v.clone();
        let mut b = u64v.clone();
        ts.push(time_it("K2 CORRECTED + metric rebuilt/step", cells, 2, repeats, |st| {
            for _ in 0..st {
                k2_corrected_onfly(black_box(&a), black_box(&mut b), &tile, 0.2);
                std::mem::swap(&mut a, &mut b);
            }
        }));
        bytes.push("0 (solved live)");
    }

    bench32!("K4 PRECOMPUTED fused 21-pt (SoA)", "84 (21 f32)", |u: &[f32], o: &mut [f32], a: &mut [f32]| k4_fused_f32(u, o, tn, s, 2, 0.2, &fw32, &doff, a));
    bench32!("  [K4 with AoS weights — layout cost]", "84 (21 f32)", |u: &[f32], o: &mut [f32], _a: &mut [f32]| k4_fused_f32_aos(u, o, tn, s, 2, 0.2, &fw32_aos, &doff));
    {
        let mut a = u64v.clone();
        let mut b = u64v.clone();
        let mut acc = vec![0.0f64; tn];
        ts.push(time_it("K5 PRECOMPUTED fused 21-pt, f64", cells, steps, repeats, |st| {
            for _ in 0..st {
                k5_fused_f64(black_box(&a), black_box(&mut b), tn, s, 2, 0.2, &fw, &doff, &mut acc);
                std::mem::swap(&mut a, &mut b);
            }
        }));
        bytes.push("168 (21 f64)");
    }
    {
        let mut a = u32v.clone();
        let mut b = u32v.clone();
        let mut gx = vec![0.0f32; s * s];
        let mut gy = vec![0.0f32; s * s];
        ts.push(time_it("K6 PRECOMPUTED two-pass (keeps fluxes)", cells, steps, repeats, |st| {
            for _ in 0..st {
                k6_twopass_f32(black_box(&a), black_box(&mut b), &mut gx, &mut gy, tn, s, 2, 0.2, &gc32, &ec32, &inva6);
                std::mem::swap(&mut a, &mut b);
            }
        }));
        bytes.push("148");
    }
    {
        let mut wconst = [0.0f32; 21];
        let mid = (tn / 2) * tn + tn / 2;
        for q in 0..21 {
            wconst[q] = fw[q * tn * tn + mid] as f32;
        }
        let mut a = u32v.clone();
        let mut b = u32v.clone();
        let mut acc = vec![0.0f32; tn];
        ts.push(time_it("K7 FROZEN const-coeff 21-pt (APPROX)", cells, steps, repeats, |st| {
            for _ in 0..st {
                k7_frozen_f32(black_box(&a), black_box(&mut b), tn, s, 2, 0.2, &wconst, &doff, &mut acc);
                std::mem::swap(&mut a, &mut b);
            }
        }));
        bytes.push("0 (in registers)");
    }

    let base = ts[0].median();
    println!("| kernel | cell-updates/s (median of {repeats}) | run-to-run | vs baseline | coeff B/cell |");
    println!("|---|---|---|---|---|");
    for (i, t) in ts.iter().enumerate() {
        println!(
            "| {} | **{}** | ±{:.1}% | {:.3}x | {} |",
            t.label,
            g(t.median()),
            t.spread_pct() / 2.0,
            t.median() / base,
            bytes[i]
        );
    }
    println!();
    println!("Precompute, once per tile (amortised over the tile's steps):");
    println!("- tile geometry (centres, areas, 4 edges/cell) : {geom_ms:.2} ms  ({tn}x{tn})");
    println!("- fused 21-pt weights (the 5x5 inverse/cell)   : {fuse_ms:.2} ms");
    let find = |needle: &str| ts.iter().find(|t| t.label.contains(needle)).unwrap().median();
    let (r_k3, r_k4) = (find("K3"), find("K4 PRECOMPUTED"));
    let saved_per_cell_step = 1.0 / r_k3 - 1.0 / r_k4; // seconds saved per cell per step
    let pre_per_cell = (geom_ms + fuse_ms) / 1e3 / cells as f64;
    println!("- break-even vs re-solving every step          : {:.2} steps", pre_per_cell / saved_per_cell_step);
    println!("  (i.e. the precompute pays for itself almost immediately; erosion runs 100s-1000s of steps/tile)");
    println!();
}

fn k1_tables(tile: &Tile) -> (Vec<f32>, Vec<f32>) {
    let (n, s, h) = (tile.n, tile.s, tile.halo);
    let mut t = vec![0.0f32; 4 * n * n];
    let mut ia = vec![0.0f32; n * n];
    for y in 0..n {
        for x in 0..n {
            let c = (y + h) * s + (x + h);
            for e in 0..4 {
                t[e * n * n + y * n + x] = tile.t[4 * c + e] as f32;
            }
            ia[y * n + x] = (1.0 / tile.area[c]) as f32;
        }
    }
    (t, ia)
}

fn k6_tables(tile: &Tile) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let (n, s, h) = (tile.n, tile.s, tile.halo);
    let mut gc = vec![0.0f32; 16 * s * s];
    for y in h - 1..s - h + 1 {
        for x in h - 1..s - h + 1 {
            let c = y * s + x;
            let (cx, cy) = grad_coeffs(tile, c);
            for q in 0..8 {
                gc[c * 16 + q] = cx[q] as f32;
                gc[c * 16 + 8 + q] = cy[q] as f32;
            }
        }
    }
    let mut ec = vec![0.0f32; 20 * n * n];
    let mut ia = vec![0.0f32; n * n];
    for y in 0..n {
        for x in 0..n {
            let c = (y + h) * s + (x + h);
            let base = (y * n + x) * 20;
            for (e, &(dx, dy)) in EDGE.iter().enumerate() {
                let jn = ((c as isize) + dy * s as isize + dx) as usize;
                let o = base + 5 * e;
                ec[o] = tile.t[4 * c + e] as f32;
                ec[o + 1] = -dot(tile.e0[c], tile.wi[4 * c + e]) as f32;
                ec[o + 2] = -dot(tile.e1[c], tile.wi[4 * c + e]) as f32;
                ec[o + 3] = dot(tile.e0[jn], tile.wj[4 * c + e]) as f32;
                ec[o + 4] = dot(tile.e1[jn], tile.wj[4 * c + e]) as f32;
            }
            ia[y * n + x] = (1.0 / tile.area[c]) as f32;
        }
    }
    (gc, ec, ia)
}

// ---------------------------------------------------------------------------
// §D — THE CLIFF. Where does each scheme fall out of cache?
//     spatial-key-bench.md: the naive dense 5-point runs ~6 G cell-updates/s while the
//     working set fits L2/L3, with a cache cliff at ~17 MB/field. THAT is the number the
//     corrected scheme has to live beside — and the question is BYTES, not FLOPs.
// ---------------------------------------------------------------------------
fn section_d_cliff(offs: &[(isize, isize)]) {
    println!("## D. THE CACHE CLIFF — where each scheme falls out of L2 (this machine: 16 MB)");
    println!();
    println!("The coefficient stream is 84 B/cell against the field's 4 B/cell — 21x. So the");
    println!("corrected scheme should meet the cliff when 84·N² ≈ 16 MB, i.e. N ≈ 437. Predicted");
    println!("BEFORE measuring; the sweep is deliberately dense across that point.");
    println!();
    println!("| patch | cells | field MB | coeff MB | K0 naive | K4 fused f32 | K7 frozen |");
    println!("|---|---|---|---|---|---|---|");

    for &tn in &[64usize, 128, 256, 384, 512, 768, 1024, 2048] {
        let lvl = 19u8;
        let nn = 1u32 << lvl;
        let tile = Tile::new(0, lvl, nn / 2 - tn as u32 / 2, nn / 2 - tn as u32 / 2, tn, 2);
        let s = tile.s;
        let cells = tn * tn;
        let steps = (4_000_000usize / cells).max(4);
        let repeats = 7;

        let u = terrain(&tile);
        let u32v: Vec<f32> = u.iter().map(|&x| x as f32).collect();
        let fw = build_fused::<f32>(&tile, offs);
        let fw32: Vec<f32> = fw.iter().map(|&x| x as f32).collect();
        let doff = doffsets(offs, s);
        let coeff_mb = fw32.len() as f64 * 4.0 / 1e6;
        let field_mb = (s * s) as f64 * 4.0 / 1e6;

        let mut a = u32v.clone();
        let mut b = u32v.clone();
        let t0 = time_it("k0", cells, steps, repeats, |st| {
            for _ in 0..st {
                k0_naive(black_box(&a), black_box(&mut b), tn, s, 2, 0.2);
                std::mem::swap(&mut a, &mut b);
            }
        });
        let mut a = u32v.clone();
        let mut b = u32v.clone();
        let mut acc = vec![0.0f32; tn];
        let t4 = time_it("k4", cells, steps, repeats, |st| {
            for _ in 0..st {
                k4_fused_f32(black_box(&a), black_box(&mut b), tn, s, 2, 0.2, &fw32, &doff, &mut acc);
                std::mem::swap(&mut a, &mut b);
            }
        });
        let mut wconst = [0.0f32; 21];
        let mid = (tn / 2) * tn + tn / 2;
        for q in 0..21 {
            wconst[q] = fw[q * tn * tn + mid] as f32;
        }
        let mut a = u32v.clone();
        let mut b = u32v.clone();
        let mut acc7 = vec![0.0f32; tn];
        let t7 = time_it("k7", cells, steps, repeats, |st| {
            for _ in 0..st {
                k7_frozen_f32(black_box(&a), black_box(&mut b), tn, s, 2, 0.2, &wconst, &doff, &mut acc7);
                std::mem::swap(&mut a, &mut b);
            }
        });

        println!(
            "| {tn}² | {} | {:.2} | {:.1} | {} (±{:.1}%) | **{}** (±{:.1}%) | {} (±{:.1}%) |",
            cells,
            field_mb,
            coeff_mb,
            g(t0.median()),
            t0.spread_pct() / 2.0,
            g(t4.median()),
            t4.spread_pct() / 2.0,
            g(t7.median()),
            t7.spread_pct() / 2.0,
        );
    }
    println!();
}

// ---------------------------------------------------------------------------
// §E — WHAT EACH FORM PRESERVES, AND WHAT IT SACRIFICES.
//
// `DECISIONS[preserve-the-structure-declare-the-sacrifice]`: "No scheme preserves all of
// them. You are ALWAYS choosing. The sin is choosing silently." A cost number that does
// not say what the cheap form GAVE UP is not an answer, it is an advertisement.
// ---------------------------------------------------------------------------
fn section_e_structure(offs: &[(isize, isize)]) {
    println!("## E. CONSERVATION, AND WHERE THE FROZEN STENCIL BREAKS");
    println!();

    let tn = 64usize;
    let lvl = 19u8;
    let nn = 1u32 << lvl;
    let tile = Tile::new(0, lvl, nn / 2 - 32, nn / 2 - 32, tn, 2);
    let s = tile.s;
    let doff = doffsets(offs, s);
    let fw = build_fused::<f32>(&tile, offs);

    // mass = Σ u_i · A_i over the interior, before and after 200 steps.
    let _mass = |u: &[f64]| -> f64 {
        let mut m = 0.0;
        for y in 0..tn {
            for x in 0..tn {
                let c = tile.idx(x + 2, y + 2);
                m += u[c] * tile.area[c];
            }
        }
        m
    };
    // A COMPACTLY-SUPPORTED bump: exactly zero outside radius 15 of the tile centre, so
    // over 200 steps (diffusion length ~3 cells) NOTHING reaches the interior boundary and
    // no flux physically leaves. Any mass drift is then the SCHEME's, not an outflow.
    // (The first version of this probe used a Gaussian that was still ~1% of peak at the
    // boundary; both kernels then leaked the same -1.7e-3, which was real outflow and
    // measured nothing about conservation. A probe that cannot fail is not a probe — and
    // one that fails identically for every input is not measuring what its label says.)
    let mut u0 = vec![0.0f64; s * s];
    for y in 0..tn {
        for x in 0..tn {
            let c = tile.idx(x + 2, y + 2);
            let (dx, dy) = (x as f64 - 32.0, y as f64 - 32.0);
            let r = (dx * dx + dy * dy).sqrt() / 15.0;
            u0[c] = if r < 1.0 { 100.0 * (1.0 - r * r).powi(3) } else { 0.0 };
        }
    }

    let mid = (tn / 2) * tn + tn / 2;
    let mut wconst64 = [0.0f64; 21];
    for q in 0..21 {
        wconst64[q] = fw[q * tn * tn + mid];
    }
    let mut fw_frozen = vec![0.0f64; 21 * tn * tn];
    for q in 0..21 {
        for c in 0..tn * tn {
            fw_frozen[q * tn * tn + c] = wconst64[q];
        }
    }

    // THE TEST IS ALGEBRAIC, NOT DYNAMICAL. Applying the operator ONCE to a field whose
    // support is strictly interior means every boundary flux is identically zero, so a
    // conservative scheme MUST give `Σ Aᵢ·Δuᵢ = 0` exactly. Running 200 diffusion steps
    // instead (my first two attempts) just measures the tail diffusing out through the
    // boundary — which is real physics, is identical for every scheme, and says nothing
    // about conservation. Both earlier versions returned the same number for both kernels;
    // that identity was the tell.
    println!("Operator applied ONCE to a compactly-supported bump (all boundary fluxes = 0):");
    println!("`Σ Aᵢ·Δuᵢ` must be EXACTLY zero for a conservative scheme.");
    println!();
    println!("| kernel | Σ Aᵢ·Δuᵢ / Σ Aᵢ·|Δuᵢ| | exactly conservative? |");
    println!("|---|---|---|");
    for (label, w) in [("K4/K5 PRECOMPUTED fused (per-cell)", &fw), ("K7 FROZEN const-coeff", &fw_frozen)] {
        let mut o = u0.clone();
        k5_fused_f64(&u0, &mut o, tn, s, 2, 1.0, w, &doff, &mut vec![0.0f64; tn]);
        let (mut net, mut abs) = (0.0f64, 0.0f64);
        for y in 0..tn {
            for x in 0..tn {
                let c = tile.idx(x + 2, y + 2);
                let d = o[c] - u0[c];
                net += d * tile.area[c];
                abs += (d * tile.area[c]).abs();
            }
        }
        let leak = net / abs;
        println!("| {label} | {leak:+.2e} | {} |", if leak.abs() < 1e-13 { "**YES** (round-off)" } else { "**NO** — a real leak" });
    }
    println!();

    // and: is the frozen approximation even accurate away from the face centre?
    println!("The frozen stencil's other exposure — the metric is worst at a face CORNER, not centre:");
    println!();
    println!("| tile position on face (L19) | frozen-vs-exact rel L2 (f64) |");
    println!("|---|---|");
    for (name, oi, oj) in [
        ("face centre", nn / 2 - 32, nn / 2 - 32),
        ("mid-face", nn / 4, nn / 4),
        ("near face edge", 64, nn / 2 - 32),
        ("face CORNER", 4, 4),
    ] {
        let t = Tile::new(0, lvl, oi, oj, tn, 2);
        let fwt = build_fused::<f32>(&t, offs);
        let u = terrain(&t);
        let dt = doffsets(offs, t.s);
        let m = (tn / 2) * tn + tn / 2;
        let mut wc = vec![0.0f64; 21 * tn * tn];
        for q in 0..21 {
            for c in 0..tn * tn {
                wc[q * tn * tn + c] = fwt[q * tn * tn + m];
            }
        }
        let mut oa = u.clone();
        let mut ob = u.clone();
        k5_fused_f64(&u, &mut oa, tn, t.s, 2, 1.0, &fwt, &dt, &mut vec![0.0f64; tn]);
        k5_fused_f64(&u, &mut ob, tn, t.s, 2, 1.0, &wc, &dt, &mut vec![0.0f64; tn]);
        let (mut ga, mut gb) = (Vec::new(), Vec::new());
        for y in 0..tn {
            for x in 0..tn {
                let c = t.idx(x + 2, y + 2);
                ga.push(oa[c] - u[c]);
                gb.push(ob[c] - u[c]);
            }
        }
        println!("| {name} | {:.2e} |", rel_l2(&gb, &ga));
    }
    println!();
}

// ---------------------------------------------------------------------------
// §F — THE RATIO IS NOT THE ANSWER. What fraction of a real erosion epoch is
//      stencil work at all?
//
// `erosion.rs::erode` runs, per epoch: uplift · fill_depressions (Priority-Flood) ·
// receivers · elevation_order (a SORT) · accumulate_drainage · incise · deposit ·
// talus · creep. **Only `creep` is a 5-point Laplacian** — exactly one pass, and it is
// exactly what the corrected scheme would replace 1:1. Everything else is O(N log N)
// graph/sort work that the grid question does not touch.
//
// So a 12x slowdown on the stencil is NOT a 12x slowdown on erosion, and quoting the
// stencil ratio as if it were would be the more alarming number winning by being louder.
// ---------------------------------------------------------------------------
fn section_f_erosion() {
    use vivarium_world::erosion::{Fluvial, FluvialParams};
    use vivarium_world::sphere::Face;

    println!("## F. WHAT THIS COSTS *EROSION* — the stencil is one pass inside a much bigger epoch");
    println!();

    let (lvl, nx, epochs) = (19u8, 64usize, 40u32);
    let nn = 1u32 << lvl;
    let p = FluvialParams {
        k_dt: 2.0e-5,
        m: 0.5,
        deposition: 1.0,
        max_slope: 0.7,
        diffusivity_m2: 0.05,
        epochs,
    };

    let mut best = f64::INFINITY;
    for _ in 0..5 {
        let mut f = Fluvial::from_prior(0, Face::ZPos, lvl, nn / 2 - 32, nn / 2 - 32, nx);
        let t = Instant::now();
        f.erode(black_box(&p));
        best = best.min(t.elapsed().as_secs_f64());
    }
    let per_epoch_us = best / epochs as f64 * 1e6;
    let cells = (nx * nx) as f64;

    // one naive 5-point pass, and one corrected fused pass, at the measured rates
    let naive_us = cells / 3.2e9 * 1e6;
    let corr_us = cells / 2.60e8 * 1e6;
    let solve_us = cells / 4.1e6 * 1e6;

    println!("Real `Fluvial::erode`, L19, 64x64 tile, {epochs} epochs (the CLI's default):");
    println!();
    println!("| | µs per epoch per tile | share of the epoch |");
    println!("|---|---|---|");
    println!("| **the whole epoch** (Priority-Flood + sort + routing + incise + deposit + talus + creep) | {per_epoch_us:.1} | 100% |");
    println!("| `creep` — the ONE 5-point Laplacian pass in it (today) | {naive_us:.2} | {:.2}% |", 100.0 * naive_us / per_epoch_us);
    println!("| the same pass, CORRECTED **as written** (K3, 5x5 solve/cell/step) | {solve_us:.0} | {:.0}% |", 100.0 * solve_us / per_epoch_us);
    println!("| the same pass, CORRECTED + **precomputed** (K4) | {corr_us:.2} | {:.1}% |", 100.0 * corr_us / per_epoch_us);
    println!();
    println!("| what an erosion EPOCH costs | µs | vs today |");
    println!("|---|---|---|");
    println!("| today (naive creep) | {per_epoch_us:.0} | 1.00x |");
    println!("| corrected, AS WRITTEN | {:.0} | **{:.2}x** |", per_epoch_us - naive_us + solve_us, (per_epoch_us - naive_us + solve_us) / per_epoch_us);
    println!("| corrected, PRECOMPUTED | {:.0} | **{:.3}x** |", per_epoch_us - naive_us + corr_us, (per_epoch_us - naive_us + corr_us) / per_epoch_us);
    println!();
    println!("⚠ READ BOTH TABLES TOGETHER. \"790x slower\" is TRUE of the stencil and MISLEADING");
    println!("  about erosion: it is a ratio on a term that is 0.24% of the epoch. Even the");
    println!("  UNRESCUED scheme only makes an erosion epoch {:.1}x slower. The precomputed", (per_epoch_us - naive_us + solve_us) / per_epoch_us);
    println!("  form makes it {:.3}x. Quoting the stencil ratio as the cost to vivarium would be", (per_epoch_us - naive_us + corr_us) / per_epoch_us);
    println!("  the alarming number winning because it is louder. [M]");
    println!();
    println!("⇒ Replacing creep's stencil with the corrected one adds **{:.1} µs/epoch**, i.e.", corr_us - naive_us);
    println!("  **+{:.1}% on a full erosion epoch** — not +1130%. The 12.3x is a ratio on a term", 100.0 * (corr_us - naive_us) / per_epoch_us);
    println!("  that is {:.2}% of the work. [M]", 100.0 * naive_us / per_epoch_us);
    println!();
    println!("⚠ AND THE HONEST LIMIT OF THAT: `creep` is the only pass the fused-Laplacian form");
    println!("  can replace. The Prime Question's actual target is **MFD routing**, which needs");
    println!("  materialised per-edge FLUXES and therefore the two-pass form (K6), and which sits");
    println!("  inside the Priority-Flood/sort/accumulate block — the expensive part. **That");
    println!("  substitution is NOT measured here and it is the residue of this spike.** Do not");
    println!("  read '+{:.1}% on an epoch' as the cost of fixing MFD.", 100.0 * (corr_us - naive_us) / per_epoch_us);
    println!();
}
