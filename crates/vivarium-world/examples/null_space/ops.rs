//! The operators under test — controls first, then our real kernels.
//!
//! Every operator here is a map `Vec<f64> -> Vec<f64>` on an `nx × nx` patch, so
//! [`crate::linalg::jacobian_fd`] can assemble it into a dense matrix without
//! caring what is inside. **Where a real kernel can be called, it is called** —
//! transcriptions are a last resort, and each one carries a pin (a numeric
//! agreement test against the kernel it transcribes).

use vivarium_world::chunk::Patch;
use vivarium_world::erosion::{self, Fluvial, FluvialParams};
use vivarium_world::sphere::Face;

// ─────────────────────────────────────────────────────────────────────────────
// CONTROLS — the probe must be able to return a clean answer AND a dirty one.
// "A probe that cannot fail is not a probe" (ORIENTATION, standing guard).
// ─────────────────────────────────────────────────────────────────────────────

#[inline]
fn wrap(i: isize, nx: usize) -> usize {
    (i.rem_euclid(nx as isize)) as usize
}

#[inline]
fn at(u: &[f64], nx: usize, x: isize, y: isize) -> f64 {
    u[wrap(y, nx) * nx + wrap(x, nx)]
}

/// **CTRL-CLEAN.** The compact 5-point Laplacian on a periodic patch:
/// `∇²u ≈ (u_E + u_W + u_N + u_S − 4u_P) / Δ²`.
///
/// Every difference it takes is across a FACE — the neighbour is `Δ` away, not
/// `2Δ`. Feed it the checkerboard `(−1)^{i+j}`: every neighbour is `−u_P`, so it
/// returns `−8u_P/Δ²` — **the most strongly damped mode in the spectrum, not the
/// least.** MUST show exactly ONE zero (the constant — a legitimate "rigid" mode:
/// adding a constant to an elevation field exerts no force).
pub fn ctrl_compact_laplacian(u: &[f64], nx: usize) -> Vec<f64> {
    let mut out = vec![0.0; nx * nx];
    for y in 0..nx as isize {
        for x in 0..nx as isize {
            out[y as usize * nx + x as usize] = at(u, nx, x + 1, y)
                + at(u, nx, x - 1, y)
                + at(u, nx, x, y + 1)
                + at(u, nx, x, y - 1)
                - 4.0 * at(u, nx, x, y);
        }
    }
    out
}

/// **CTRL-BLIND.** The *collocated* central-difference operator — the textbook
/// checkerboard machine, and the one Cardiff §2.3 and our own `fig/staggering.svg`
/// are about.
///
/// It is the honest composition a collocated scheme is forced into: compute a
/// gradient AT THE CELL CENTRE by central differences (`(u_{i+1} − u_{i−1})/2Δ`),
/// then take its divergence the same way. Both differences reach **two cells**,
/// so the operator only ever compares cells of the same parity:
/// `(u_{i+2} + u_{i−2} + u_{j+2} + u_{j−2} − 4u_P) / (2Δ)²`.
///
/// The `nx × nx` grid therefore decouples into **four independent sublattices**
/// (parity of `i` × parity of `j`), each carrying its own constant. So this
/// operator MUST show **4 zero eigenvalues** — one legitimate (the constant) and
/// **3 spurious**, spanned by `(−1)^i`, `(−1)^j`, and the CHECKERBOARD `(−1)^{i+j}`.
/// If the instrument cannot tell this apart from CTRL-CLEAN, it is measuring
/// nothing.
pub fn ctrl_collocated_central(u: &[f64], nx: usize) -> Vec<f64> {
    let mut out = vec![0.0; nx * nx];
    for y in 0..nx as isize {
        for x in 0..nx as isize {
            out[y as usize * nx + x as usize] = 0.25
                * (at(u, nx, x + 2, y) + at(u, nx, x - 2, y) + at(u, nx, x, y + 2) + at(u, nx, x, y - 2)
                    - 4.0 * at(u, nx, x, y));
        }
    }
    out
}

/// **The staggering test, as an operator.** Build the flux ON THE FACE from the
/// two cells that actually share it (`f = (u_N − u_P)/Δ`), then take the
/// divergence of the face fluxes.
///
/// This adds no points. It uses the same points. And it lands *exactly* on the
/// compact Laplacian — which is the whole content of `doc/theory/fig/staggering.svg`:
/// **staggering does not TAME the null mode, it never CREATES it.** We assert the
/// identity numerically rather than asserting it in prose.
pub fn ctrl_staggered_divgrad(u: &[f64], nx: usize) -> Vec<f64> {
    // Face fluxes: fx[y][x] is the flux across the face between (x,y) and (x+1,y).
    let mut fx = vec![0.0; nx * nx];
    let mut fy = vec![0.0; nx * nx];
    for y in 0..nx as isize {
        for x in 0..nx as isize {
            fx[y as usize * nx + x as usize] = at(u, nx, x + 1, y) - at(u, nx, x, y);
            fy[y as usize * nx + x as usize] = at(u, nx, x, y + 1) - at(u, nx, x, y);
        }
    }
    let mut out = vec![0.0; nx * nx];
    for y in 0..nx as isize {
        for x in 0..nx as isize {
            out[y as usize * nx + x as usize] = at(&fx, nx, x, y) - at(&fx, nx, x - 1, y) + at(&fy, nx, x, y)
                - at(&fy, nx, x, y - 1);
        }
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// OUR KERNELS
// ─────────────────────────────────────────────────────────────────────────────

/// **The naive uniform 5-point Laplacian vivarium actually runs** — the REAL
/// [`erosion::diffuse_step`], not a copy of it.
///
/// `Patch` carries a halo ring, and `Patch::set` writes it. So we impose
/// PERIODICITY by filling the halo with wrapped interior values and handing the
/// patch to the untouched kernel. What is measured is the shipped code.
pub fn erosion_diffuse_real(u: &[f64], nx: usize, k: f32) -> Vec<f64> {
    let mut src: Patch<f32> = Patch::new(Face::ZPos, 12, 100, 100, nx, 1);
    let mut dst: Patch<f32> = Patch::new(Face::ZPos, 12, 100, 100, nx, 1);
    for y in 0..nx {
        for x in 0..nx {
            src.set(x as isize, y as isize, u[y * nx + x] as f32);
        }
    }
    // Periodic halo (the 5-point stencil reads only the 4 axial ghosts).
    for i in 0..nx as isize {
        src.set(-1, i, u[i as usize * nx + (nx - 1)] as f32);
        src.set(nx as isize, i, u[i as usize * nx] as f32);
        src.set(i, -1, u[(nx - 1) * nx + i as usize] as f32);
        src.set(i, nx as isize, u[i as usize] as f32);
    }
    erosion::diffuse_step(&src, &mut dst, k);
    // Return the RATE (h' − h)/k, so the operator is the Laplacian itself and its
    // null space is directly comparable with the controls.
    (0..nx * nx)
        .map(|i| {
            let x = (i % nx) as isize;
            let y = (i / nx) as isize;
            (dst.get(x, y) as f64 - u[i]) / k as f64
        })
        .collect()
}

/// The **full erosion epoch** — `Fluvial::erode` with `epochs = 1`, the real
/// pipeline (Priority-Flood → D8 receivers → MFD accumulation → implicit
/// stream-power → Davy–Lague deposition → talus → creep), returning `Δh`.
///
/// This one is a CLOSED BOX, not a periodic patch: the kernel's edge ring and
/// every sub-sea cell are *outlets* (base level = a constrained DOF, in Cardiff's
/// language). We therefore assemble the operator over the FREE DOF only — the
/// interior land cells — which is the correct analogue of Cardiff's "patch of
/// cells adjacent to a boundary" caveat, stated rather than hidden.
///
/// ⚠ The kernel is **not differentiable**: D8 receiver choice is an `argmax`,
/// MFD weights switch on and off at `drop > 0`, and Priority-Flood is a
/// heap-ordered flood-fill. A finite-difference Jacobian is therefore a SECANT,
/// not a derivative, and its validity is exactly what the ε-sweep in `main` tests.
pub struct ErosionPatch {
    pub nx: usize,
    pub base: Vec<f32>,
    pub free: Vec<usize>, // indices of free (interior, supra-sea) DOF
    pub params: FluvialParams,
    pub level: u8,
    pub origin: (u32, u32),
}

impl ErosionPatch {
    /// One epoch on the real kernel; returns Δh at the free DOF.
    pub fn delta_h(&self, u_free: &[f64]) -> Vec<f64> {
        let mut h = self.base.clone();
        for (k, &i) in self.free.iter().enumerate() {
            h[i] = u_free[k] as f32;
        }
        let mut f = Fluvial::from_surface(0, Face::ZPos, self.level, self.origin.0, self.origin.1, self.nx, |_| 0.0);
        f.h = h.clone();
        let mut p = self.params.clone();
        p.epochs = 1;
        f.erode(&p);
        self.free.iter().map(|&i| (f.h[i] - h[i]) as f64).collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MFD ROUTING — the operator, and what "rank deficiency" means for a router
// ─────────────────────────────────────────────────────────────────────────────

/// The MFD weight matrix `W`: `W[i][j]` = fraction of cell `i`'s throughput that
/// is handed to cell `j`. Transcribed from `erosion::Fluvial::accumulate_drainage`
/// (`P = 1.1`, 8 Moore neighbours, `dist = cell·√2` on the diagonals) — and
/// **pinned**: `(I − Wᵀ)⁻¹ · runoff` is checked against the drainage field the
/// real kernel produces, in `main`.
///
/// **What a null space MEANS here.** Drainage solves `(I − Wᵀ) A = q`. A zero
/// eigenvalue of `(I − Wᵀ)` — equivalently, an eigenvalue 1 of `Wᵀ` — is a
/// **closed circulation**: a set of cells that hand flow around a loop and leak
/// none of it, so a nonzero drainage field exists with NO source. That is water
/// running in a circle on a hillside forever. It is the discrete `∇ × ∇φ ≠ 0`
/// pathology, and it is a genuine rank deficiency of the routing operator.
pub fn mfd_weights(h: &[f32], nx: usize, cell_m: f32) -> Vec<Vec<(usize, f64)>> {
    const P: f32 = 1.1;
    const NEIGHBORS: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut w: Vec<Vec<(usize, f64)>> = vec![Vec::new(); nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            let i = y * nx + x;
            let hi = h[i];
            let mut ws = [0.0f32; 8];
            let mut total = 0.0f32;
            for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                let drop = hi - h[j];
                if drop > 0.0 {
                    let dist = if *dx != 0 && *dy != 0 { cell_m * std::f32::consts::SQRT_2 } else { cell_m };
                    let wk = (drop / dist).powf(P);
                    ws[k] = wk;
                    total += wk;
                }
            }
            if total > 0.0 {
                for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                    if ws[k] > 0.0 {
                        let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                        w[i].push((j, (ws[k] / total) as f64));
                    }
                }
            }
        }
    }
    w
}

/// Dense `(I − Wᵀ)` — the operator whose inverse IS drainage accumulation.
pub fn routing_operator(w: &[Vec<(usize, f64)>], n: usize) -> Vec<f64> {
    let mut a = vec![0.0f64; n * n];
    for i in 0..n {
        a[i * n + i] = 1.0;
    }
    for (i, row) in w.iter().enumerate() {
        for &(j, wij) in row {
            a[j * n + i] -= wij; // (Wᵀ)[j][i] = W[i][j]
        }
    }
    a
}

/// Accumulate drainage by the same high→low sweep the kernel uses — used to PIN
/// [`mfd_weights`] against the real `Fluvial::drainage`.
pub fn accumulate_with(w: &[Vec<(usize, f64)>], h: &[f32], runoff: &[f64]) -> Vec<f64> {
    let n = h.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| h[a].total_cmp(&h[b]).then_with(|| a.cmp(&b)));
    let mut acc = runoff.to_vec();
    for &i in order.iter().rev() {
        let amount = acc[i];
        for &(j, wij) in &w[i] {
            acc[j] += amount * wij;
        }
    }
    acc
}
