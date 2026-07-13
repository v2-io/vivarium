//! Dependency-free dense linear algebra for the null-space probe.
//!
//! `vivarium-world` has **zero dependencies** and this probe keeps it that way.
//! Everything here is textbook and small enough to audit by eye:
//!
//! - [`jacobi_sym`] — cyclic Jacobi eigensolver for real SYMMETRIC matrices
//!   (values + vectors). Backward-stable, and it computes the *small* eigenvalues
//!   to high relative accuracy — which is exactly what a null-space count needs.
//! - [`singular_values`] — SVD via `jacobi_sym` on `AᵀA`. **Rank deficiency is
//!   zero SINGULAR values**, and that is the general (non-symmetric) form of
//!   Cardiff's "zero eigenvalues": `σ = 0 ⟺ ∃u ≠ 0 : Au = 0`. Right-singular
//!   vectors of small σ ARE the invisible modes.
//! - [`eigenvalues_general`] — Hessenberg reduction + shifted QR (the classic
//!   `elmhes`/`hqr` pair) for a general real matrix: returns complex eigenvalues.
//!   Needed for the time-STEP operators, whose modes are travelling waves
//!   (complex pairs), where "meets no resistance" reads as |λ| ≥ 1, not λ = 0.
//! - [`jacobian_fd`] — central-difference Jacobian of any `Fn(&[f64]) -> Vec<f64>`.
//!   This is how we assemble the operator **from the kernel as written** rather
//!   than from our own re-derivation of it.
//!
//! ## The unification the probe rests on
//!
//! Cardiff's test is stated for a *stiffness* matrix `K` (a static operator):
//! a mode `u` with `Ku = 0` **meets no resistance**. Our kernels are *time
//! steppers*: `uⁿ⁺¹ = J uⁿ`. The two statements are the same one. For an explicit
//! step `J = I + Δt·A`,
//!
//! ```text
//!     λ(A) = 0   ⟺   λ(J) = 1
//! ```
//!
//! so "zero eigenvalue of the stiffness" ≡ "eigenvalue ON the unit circle of the
//! step operator" — a mode that is neither restored nor damped, and therefore
//! grows without bound under any forcing (and |λ| > 1 is strictly worse: it grows
//! on its own). We report BOTH forms, and which one is the honest reading is
//! stated per operator.

/// Cyclic Jacobi eigensolver for a real symmetric `n × n` matrix (row-major).
/// Returns `(values, vectors)` with `vectors[k]` the unit eigenvector for
/// `values[k]`, sorted by |value| ascending — smallest (i.e. most null) first.
pub fn jacobi_sym(a_in: &[f64], n: usize) -> (Vec<f64>, Vec<Vec<f64>>) {
    let mut a = a_in.to_vec();
    let mut v = vec![0.0f64; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }
    for _sweep in 0..100 {
        let mut off = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                off += a[i * n + j] * a[i * n + j];
            }
        }
        if off.sqrt() < 1e-14 * (1.0 + frob(&a, n)) {
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                let apq = a[p * n + q];
                if apq.abs() < 1e-300 {
                    continue;
                }
                let app = a[p * n + p];
                let aqq = a[q * n + q];
                let theta = (aqq - app) / (2.0 * apq);
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    -1.0 / (-theta + (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                for k in 0..n {
                    let akp = a[k * n + p];
                    let akq = a[k * n + q];
                    a[k * n + p] = c * akp - s * akq;
                    a[k * n + q] = s * akp + c * akq;
                }
                for k in 0..n {
                    let apk = a[p * n + k];
                    let aqk = a[q * n + k];
                    a[p * n + k] = c * apk - s * aqk;
                    a[q * n + k] = s * apk + c * aqk;
                }
                for k in 0..n {
                    let vkp = v[k * n + p];
                    let vkq = v[k * n + q];
                    v[k * n + p] = c * vkp - s * vkq;
                    v[k * n + q] = s * vkp + c * vkq;
                }
            }
        }
    }
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| a[i * n + i].abs().total_cmp(&a[j * n + j].abs()));
    let vals: Vec<f64> = idx.iter().map(|&i| a[i * n + i]).collect();
    let vecs: Vec<Vec<f64>> = idx.iter().map(|&i| (0..n).map(|k| v[k * n + i]).collect()).collect();
    (vals, vecs)
}

fn frob(a: &[f64], n: usize) -> f64 {
    a.iter().take(n * n).map(|x| x * x).sum::<f64>().sqrt()
}

/// Singular values of a general real `n × n` matrix, ascending, with the
/// corresponding RIGHT-singular vectors (the null modes when σ ≈ 0).
///
/// Via `AᵀA` — whose eigenvalues are σ². Squaring costs half the digits of the
/// *tiny* singular values, which we account for honestly: the reported floor is
/// `√(machine-eps) · σ_max ≈ 1.5e-8 · σ_max`, and every zero-count in this probe
/// uses a threshold far above that floor and is checked against a control.
pub fn singular_values(a: &[f64], n: usize) -> (Vec<f64>, Vec<Vec<f64>>) {
    let mut ata = vec![0.0f64; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..n {
                s += a[k * n + i] * a[k * n + j];
            }
            ata[i * n + j] = s;
        }
    }
    let (vals, vecs) = jacobi_sym(&ata, n);
    (vals.iter().map(|v| v.max(0.0).sqrt()).collect(), vecs)
}

/// Eigenvalues (re, im) of a general real `n × n` matrix, via balancing +
/// Hessenberg reduction + shifted QR. Standard `balanc`/`elmhes`/`hqr`.
pub fn eigenvalues_general(a_in: &[f64], n: usize) -> Vec<(f64, f64)> {
    let mut a = a_in.to_vec();
    balance(&mut a, n);
    elmhes(&mut a, n);
    hqr(&mut a, n)
}

fn balance(a: &mut [f64], n: usize) {
    const RADIX: f64 = 2.0;
    let sqrdx = RADIX * RADIX;
    let mut last = false;
    while !last {
        last = true;
        for i in 0..n {
            let mut r = 0.0;
            let mut c = 0.0;
            for j in 0..n {
                if j != i {
                    c += a[j * n + i].abs();
                    r += a[i * n + j].abs();
                }
            }
            if c != 0.0 && r != 0.0 {
                let mut g = r / RADIX;
                let mut f = 1.0;
                let s = c + r;
                while c < g {
                    f *= RADIX;
                    c *= sqrdx;
                }
                g = r * RADIX;
                while c > g {
                    f /= RADIX;
                    c /= sqrdx;
                }
                if (c + r) / f < 0.95 * s {
                    last = false;
                    let g = 1.0 / f;
                    for j in 0..n {
                        a[i * n + j] *= g;
                    }
                    for j in 0..n {
                        a[j * n + i] *= f;
                    }
                }
            }
        }
    }
}

fn elmhes(a: &mut [f64], n: usize) {
    for m in 1..n.saturating_sub(1) {
        let mut x = 0.0f64;
        let mut i = m;
        for j in m..n {
            if a[j * n + m - 1].abs() > x.abs() {
                x = a[j * n + m - 1];
                i = j;
            }
        }
        if i != m {
            for j in (m - 1)..n {
                a.swap(i * n + j, m * n + j);
            }
            for j in 0..n {
                a.swap(j * n + i, j * n + m);
            }
        }
        if x != 0.0 {
            for i in (m + 1)..n {
                let mut y = a[i * n + m - 1];
                if y != 0.0 {
                    y /= x;
                    a[i * n + m - 1] = y;
                    for j in m..n {
                        a[i * n + j] -= y * a[m * n + j];
                    }
                    for j in 0..n {
                        a[j * n + m] += y * a[j * n + i];
                    }
                }
            }
        }
    }
    for i in 2..n {
        for j in 0..(i - 1) {
            a[i * n + j] = 0.0;
        }
    }
}

fn hqr(a: &mut [f64], n: usize) -> Vec<(f64, f64)> {
    let mut wr = vec![0.0f64; n];
    let mut wi = vec![0.0f64; n];
    let mut anorm = 0.0f64;
    for i in 0..n {
        for j in i.saturating_sub(1)..n {
            anorm += a[i * n + j].abs();
        }
    }
    let mut nn: isize = n as isize - 1;
    let mut t = 0.0f64;
    while nn >= 0 {
        let mut its = 0;
        loop {
            let mut l = nn;
            while l >= 1 {
                let s = a[((l - 1) as usize) * n + (l - 1) as usize].abs() + a[(l as usize) * n + l as usize].abs();
                let s = if s == 0.0 { anorm } else { s };
                if a[(l as usize) * n + (l - 1) as usize].abs() <= f64::EPSILON * s {
                    a[(l as usize) * n + (l - 1) as usize] = 0.0;
                    break;
                }
                l -= 1;
            }
            let x = a[(nn as usize) * n + nn as usize];
            if l == nn {
                wr[nn as usize] = x + t;
                wi[nn as usize] = 0.0;
                nn -= 1;
                break;
            }
            let y = a[((nn - 1) as usize) * n + (nn - 1) as usize];
            let w = a[(nn as usize) * n + (nn - 1) as usize] * a[((nn - 1) as usize) * n + nn as usize];
            if l == nn - 1 {
                let p = 0.5 * (y - x);
                let q = p * p + w;
                let z = q.abs().sqrt();
                let x2 = x + t;
                if q >= 0.0 {
                    let z = p + z.copysign(p);
                    wr[(nn - 1) as usize] = x2 + z;
                    wr[nn as usize] = if z != 0.0 { x2 - w / z } else { x2 + z };
                    wi[(nn - 1) as usize] = 0.0;
                    wi[nn as usize] = 0.0;
                } else {
                    wr[(nn - 1) as usize] = x2 + p;
                    wr[nn as usize] = x2 + p;
                    wi[(nn - 1) as usize] = -z;
                    wi[nn as usize] = z;
                }
                nn -= 2;
                break;
            }
            if its == 60 {
                // Non-convergence: report what we have rather than lie.
                wr[nn as usize] = x + t;
                wi[nn as usize] = 0.0;
                nn -= 1;
                break;
            }
            let (mut x, mut y, mut w) = (x, y, w);
            if its == 10 || its == 20 {
                t += x;
                for i in 0..=(nn as usize) {
                    a[i * n + i] -= x;
                }
                let s = a[(nn as usize) * n + (nn - 1) as usize].abs() + a[((nn - 1) as usize) * n + (nn - 2) as usize].abs();
                x = 0.75 * s;
                y = x;
                w = -0.4375 * s * s;
            }
            its += 1;
            let mut m = nn - 2;
            let (mut p, mut q, mut r) = (0.0f64, 0.0f64, 0.0f64);
            while m >= l {
                let z = a[(m as usize) * n + m as usize];
                let rr = x - z;
                let ss = y - z;
                p = (rr * ss - w) / a[((m + 1) as usize) * n + m as usize] + a[(m as usize) * n + (m + 1) as usize];
                q = a[((m + 1) as usize) * n + (m + 1) as usize] - z - rr - ss;
                r = a[((m + 2) as usize) * n + (m + 1) as usize];
                let s = p.abs() + q.abs() + r.abs();
                p /= s;
                q /= s;
                r /= s;
                if m == l {
                    break;
                }
                let u = a[(m as usize) * n + (m - 1) as usize].abs() * (q.abs() + r.abs());
                let v = p.abs()
                    * (a[((m - 1) as usize) * n + (m - 1) as usize].abs()
                        + a[(m as usize) * n + m as usize].abs()
                        + a[((m + 1) as usize) * n + (m + 1) as usize].abs());
                if u <= f64::EPSILON * v {
                    break;
                }
                m -= 1;
            }
            for i in (m + 2)..=nn {
                a[(i as usize) * n + (i - 2) as usize] = 0.0;
                if i != m + 2 {
                    a[(i as usize) * n + (i - 3) as usize] = 0.0;
                }
            }
            let mut k = m;
            while k <= nn - 1 {
                if k != m {
                    p = a[(k as usize) * n + (k - 1) as usize];
                    q = a[((k + 1) as usize) * n + (k - 1) as usize];
                    r = if k != nn - 1 { a[((k + 2) as usize) * n + (k - 1) as usize] } else { 0.0 };
                    x = p.abs() + q.abs() + r.abs();
                    if x != 0.0 {
                        p /= x;
                        q /= x;
                        r /= x;
                    }
                }
                let s = (p * p + q * q + r * r).sqrt().copysign(p);
                if s == 0.0 {
                    k += 1;
                    continue;
                }
                if k == m {
                    if l != m {
                        a[(k as usize) * n + (k - 1) as usize] = -a[(k as usize) * n + (k - 1) as usize];
                    }
                } else {
                    a[(k as usize) * n + (k - 1) as usize] = -s * x;
                }
                p += s;
                let xx = p / s;
                let yy = q / s;
                let zz = r / s;
                q /= p;
                r /= p;
                for j in (k as usize)..n {
                    let mut pp = a[(k as usize) * n + j] + q * a[((k + 1) as usize) * n + j];
                    if k != nn - 1 {
                        pp += r * a[((k + 2) as usize) * n + j];
                        a[((k + 2) as usize) * n + j] -= pp * zz;
                    }
                    a[((k + 1) as usize) * n + j] -= pp * yy;
                    a[(k as usize) * n + j] -= pp * xx;
                }
                let mmin = if nn < k + 3 { nn } else { k + 3 };
                for i in (l as usize)..=(mmin as usize) {
                    let mut pp = xx * a[i * n + k as usize] + yy * a[i * n + (k + 1) as usize];
                    if k != nn - 1 {
                        pp += zz * a[i * n + (k + 2) as usize];
                        a[i * n + (k + 2) as usize] -= pp * r;
                    }
                    a[i * n + (k + 1) as usize] -= pp * q;
                    a[i * n + k as usize] -= pp;
                }
                k += 1;
            }
        }
    }
    wr.into_iter().zip(wi).collect()
}

/// Central-difference Jacobian of `f` at `u0`: `J[a][b] = ∂f_a/∂u_b`, row-major.
///
/// **This is the load-bearing honesty of the probe.** The operator is assembled
/// from the kernel *as executed*, not from our re-derivation of what we think it
/// does. `eps` is per-DOF (kernels with mixed units — a depth and a flux — need
/// different scales).
pub fn jacobian_fd(n: usize, u0: &[f64], eps: &[f64], f: impl Fn(&[f64]) -> Vec<f64>) -> Vec<f64> {
    let mut j = vec![0.0f64; n * n];
    let mut u = u0.to_vec();
    for b in 0..n {
        let e = eps[b];
        u[b] = u0[b] + e;
        let fp = f(&u);
        u[b] = u0[b] - e;
        let fm = f(&u);
        u[b] = u0[b];
        for a in 0..n {
            j[a * n + b] = (fp[a] - fm[a]) / (2.0 * e);
        }
    }
    j
}

/// How far from symmetric a matrix is, relative to its own size: `‖A − Aᵀ‖ / ‖A‖`.
/// Reported rather than assumed — several of our kernels are advective and there
/// is no reason for them to be symmetric.
pub fn asymmetry(a: &[f64], n: usize) -> f64 {
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        for j in 0..n {
            let d = a[i * n + j] - a[j * n + i];
            num += d * d;
            den += a[i * n + j] * a[i * n + j];
        }
    }
    if den == 0.0 {
        0.0
    } else {
        (num / den).sqrt()
    }
}

/// Project a mode onto the discrete Fourier basis of an `nx × nx` periodic patch
/// and return the dominant wavenumber `(kx, ky)` in units of `π/Δ` (so `(1,1)` is
/// the CHECKERBOARD / Nyquist corner) plus the fraction of the mode's energy there.
///
/// This is what turns "there is a mode" into "**the mode is at Nyquist**" — the
/// difference between a numerical artifact and a physical wave.
pub fn dominant_wavenumber(mode: &[f64], nx: usize) -> ((f64, f64), f64) {
    let n = nx * nx;
    let total: f64 = mode.iter().take(n).map(|x| x * x).sum();
    if total <= 0.0 {
        return ((0.0, 0.0), 0.0);
    }
    let (mut best, mut best_p) = ((0usize, 0usize), -1.0f64);
    for ky in 0..nx {
        for kx in 0..nx {
            let (mut re, mut im) = (0.0f64, 0.0f64);
            for y in 0..nx {
                for x in 0..nx {
                    let ph = -2.0 * std::f64::consts::PI * ((kx * x) as f64 + (ky * y) as f64) / nx as f64;
                    re += mode[y * nx + x] * ph.cos();
                    im += mode[y * nx + x] * ph.sin();
                }
            }
            let p = (re * re + im * im) / n as f64;
            if p > best_p {
                best_p = p;
                best = (kx, ky);
            }
        }
    }
    // Map index k ∈ [0, nx) to a signed wavenumber in units of π/Δ: index nx/2 is
    // exactly Nyquist ⇒ 1.0.
    let f = |k: usize| {
        let s = if k > nx / 2 { k as f64 - nx as f64 } else { k as f64 };
        2.0 * s / nx as f64
    };
    ((f(best.0).abs(), f(best.1).abs()), best_p / total)
}
