//! # `null_space_gate` — the null-space probe as a *can-fail* gate
//!
//! The full instrument lives in `examples/null_space/` (a long, readable report
//! that draws figures and narrates every kernel). Its "standing guard" section
//! already runs the two controls — but it only *prints* the verdict and exits 0
//! regardless. A guard that cannot turn the light red is narration, not a probe
//! (`#norm-probe-sensitivity` FE(2): *known-bad first; a probe that cannot fail
//! on a known-bad is not yet a probe*).
//!
//! This file is the same core wired as an assertion: it reuses `linalg.rs` and
//! `ops.rs` verbatim (no fork) and **panics — nonzero exit — if the instrument
//! ever goes blind to its own known-bad, or hallucinates a mode in its
//! known-clean control, or loses a real kernel's cleanliness.** That makes it
//! wireable into `bin/check`. It is the demonstration behind the graduation note
//! in `msc/spike-null-space-probe/NOTE.md`; the graduation cost is naming-only
//! (lift `linalg` + the control ops into the crate as `src/…` so a `#[test]` can
//! reach them — this example proves the assertion body is small and green today).
//!
//! Run: `cargo run --release -p vivarium-world --example null_space_gate`

#[path = "null_space/linalg.rs"]
mod linalg;
#[path = "null_space/ops.rs"]
mod ops;

use linalg::{jacobian_fd, jacobi_sym};

/// (#zeros below `tol`, spectral gap = |first nonzero| / |last zero|).
/// `vals` arrive sorted by |value| ascending, so the cut is contiguous.
fn zero_count(vals: &[f64], tol: f64) -> (usize, f64) {
    let z = vals.iter().filter(|v| v.abs() < tol).count();
    let gap = if z == 0 || z == vals.len() {
        f64::INFINITY
    } else {
        vals[z].abs() / vals[z - 1].abs().max(1e-300)
    };
    (z, gap)
}

/// Restoring force the operator exerts on a checkerboard field: ‖A·χ‖/‖χ‖.
/// Zero ⇒ the checkerboard is invisible (the disease); large ⇒ it is seen.
fn checkerboard_response(a: &[f64], nx: usize) -> f64 {
    let n = nx * nx;
    let chi: Vec<f64> = (0..n)
        .map(|i| if ((i % nx) + (i / nx)) % 2 == 0 { 1.0 } else { -1.0 })
        .collect();
    let mut num = 0.0;
    for r in 0..n {
        let mut s = 0.0;
        for c in 0..n {
            s += a[r * n + c] * chi[c];
        }
        num += s * s;
    }
    (num / n as f64).sqrt()
}

/// Assemble a linear periodic-patch operator into a dense matrix. `eps = 1`
/// makes the central-difference Jacobian exact for the linear kernels here.
fn assemble(nx: usize, f: &dyn Fn(&[f64], usize) -> Vec<f64>) -> Vec<f64> {
    let n = nx * nx;
    jacobian_fd(n, &vec![0.0; n], &vec![1.0; n], |u| f(u, nx))
}

/// One convicting check. `expect_spurious` is the number of zero eigenvalues
/// *beyond* the rigid modes we allow; `rigid` is the legitimate count (the
/// constant field, for a diffusion/Laplacian operator, is 1).
fn gate(
    label: &str,
    a: &[f64],
    nx: usize,
    rigid: usize,
    expect_spurious: usize,
    checker_should_be_seen: bool,
    // Relative zero-threshold. **Tolerance is part of the probe**
    // (`#norm-probe-sensitivity`): a pure-f64 control's exact zero returns at
    // ~1e-16·λmax, but a real f32 kernel's exact zero returns at ~1e-7·λmax, so
    // the same operator counted at an f64-grade tol would falsely read "0 zeros".
    // The cut must match the arithmetic that produced the matrix.
    tol_rel: f64,
) {
    let n = nx * nx;
    let (vals, _) = jacobi_sym(a, n);
    let smax = vals.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    let tol = tol_rel * smax;
    let (zeros, gap) = zero_count(&vals, tol);
    let spurious = zeros as i64 - rigid as i64;
    let resp = checkerboard_response(a, nx);
    let seen = resp > 1e-6 * smax.max(1e-300);

    println!(
        "  {label:<40} zeros={zeros} rigid={rigid} spurious={spurious:+}  \
         gap={gap:.1e}  checker‖A·χ‖/‖χ‖={resp:.3e} ({})",
        if seen { "seen" } else { "INVISIBLE" }
    );

    assert_eq!(
        spurious, expect_spurious as i64,
        "{label}: expected {expect_spurious} spurious zero-eigenvalue(s), found {spurious}. \
         The instrument's discrimination has changed — investigate before trusting any pass."
    );
    // A mushy count (no clear gap) means the zero-count is not a fact. Require
    // ≥4 decades of daylight between the largest "zero" and the smallest
    // "nonzero". f64 controls clear this by ~1e15; an f32 kernel's clean cut is
    // ~1e5 (its exact zeros sit at the ~1e-7·λmax f32 noise floor), still far
    // above the bar — so this floor is honest across both arithmetics.
    if spurious == 0 && rigid < n {
        assert!(
            gap > 1e4,
            "{label}: spectral gap {gap:.1e} is too small to call the count clean."
        );
    }
    assert_eq!(
        seen, checker_should_be_seen,
        "{label}: checkerboard is {}, expected {}",
        if seen { "seen" } else { "INVISIBLE" },
        if checker_should_be_seen { "seen" } else { "INVISIBLE" }
    );
}

fn main() {
    let nx = 8;
    println!(
        "null_space_gate — the probe must convict its known-bad and clear its \
         known-clean.\n  patch {nx}×{nx}, periodic; zero-cut per operator \
         (1e-9·|λ|max for f64 controls, 1e-5·|λ|max for the f32 kernel).\n"
    );

    // KNOWN-CLEAN: the compact 5-point Laplacian. Every difference crosses a
    // FACE (Δ apart). One zero (the constant), no spurious mode, checkerboard is
    // its MOST-damped mode — must be seen.
    // Controls are pure f64 (exact arithmetic) ⇒ f64-grade cut.
    let clean = assemble(nx, &ops::ctrl_compact_laplacian);
    gate("CTRL-CLEAN  5-pt Laplacian", &clean, nx, 1, 0, true, 1e-9);

    // KNOWN-BAD: collocated central differences (2Δ apart). PROVABLY has three
    // spurious zeros — (−1)^i, (−1)^j, (−1)^(i+j) — and is blind to the
    // checkerboard. If this row ever comes back clean, the instrument is broken.
    let blind = assemble(nx, &ops::ctrl_collocated_central);
    gate("CTRL-BLIND  collocated central", &blind, nx, 1, 3, false, 1e-9);

    // LIVE KERNEL: the real erosion creep / hillslope-diffusion stencil
    // (`erosion::diffuse_step`), called through `ops`. Expected clean — it is the
    // compact Laplacian, not the collocated one. This is the forward-looking use:
    // a field nomos runs exactly this row before it ships.
    // κ/cell² at L19 (erosion.rs FluvialParams docs), matching the full
    // instrument's §1. f32 kernel ⇒ f32-grade cut (1e-5·λmax), not 1e-9.
    let creep = assemble(nx, &|u, nx| ops::erosion_diffuse_real(u, nx, 0.006));
    gate("LIVE  erosion creep (f32 kernel)", &creep, nx, 1, 0, true, 1e-5);

    println!(
        "\n  ✓ GATE GREEN — instrument convicts its known-bad, clears its \
         known-clean, and the live creep kernel has no null space.\n  \
         (If any row had disagreed this process would have exited nonzero.)"
    );
}
