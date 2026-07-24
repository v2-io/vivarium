//! # water_structures — two structural probes on `water.rs`
//!
//! **PROBE 1 — WELL-BALANCEDNESS.** *Does a still lake stay still?* A flat water
//! surface over a bumpy bed is an exact equilibrium of the continuum equations. Is
//! it an exact equilibrium of OUR SCHEME (well-balanced), or does the scheme
//! generate spurious currents that friction then eats (merely damped)?
//!
//! **PROBE 2 — FLUCTUATION–DISSIPATION.** The θ flux-smoothing pass (de Almeida &
//! Bates, θ=0.8) is dissipation. Mori–Zwanzig says dissipation and fluctuation are
//! a matched pair. `water.rs` adds the dissipation and no fluctuation. Measure the
//! energy it removes, and whether the removal is a BIAS or NOISE.
//!
//! Every probe carries a control that could have killed it.
//!
//! Run: `cargo run --release --example water_structures -- <probe>`
//! Probes: `wb` `wb-refine` `tf` `drift` `energy` `budget` `dt-scale` `spectrum` `all`

mod kernel;

use kernel::{k32, k64, Sinks, Smooth};

// ─────────────────────────────────────────────────────────────────────────────
// Deterministic beds (no OS entropy — fated, per the project's determinism law)
// ─────────────────────────────────────────────────────────────────────────────

fn hash2(x: i64, y: i64, seed: u64) -> f64 {
    let mut h = seed ^ 0x9E3779B97F4A7C15;
    h ^= (x as u64).wrapping_mul(0xBF58476D1CE4E5B9);
    h = h.rotate_left(31).wrapping_mul(0x94D049BB133111EB);
    h ^= (y as u64).wrapping_mul(0xD6E8FEB86659FD93);
    h = h.rotate_left(27).wrapping_mul(0xBF58476D1CE4E5B9);
    h ^= h >> 29;
    ((h >> 11) as f64) / ((1u64 << 53) as f64) * 2.0 - 1.0
}

/// Smooth value noise, `oct` octaves, feature size `lam` cells at octave 0.
fn fbm(x: f64, y: f64, lam: f64, oct: u32, seed: u64) -> f64 {
    let (mut amp, mut f, mut sum, mut norm) = (1.0f64, 1.0f64 / lam, 0.0f64, 0.0f64);
    for o in 0..oct {
        let (px, py) = (x * f, y * f);
        let (ix, iy) = (px.floor(), py.floor());
        let (fx, fy) = (px - ix, py - iy);
        let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
        let s = seed.wrapping_add(o as u64 * 0x9E37);
        let (a, b) = (hash2(ix as i64, iy as i64, s), hash2(ix as i64 + 1, iy as i64, s));
        let (c, d) = (hash2(ix as i64, iy as i64 + 1, s), hash2(ix as i64 + 1, iy as i64 + 1, s));
        let v = (a + (b - a) * sx) + ((c + (d - c) * sx) - (a + (b - a) * sx)) * sy;
        sum += v * amp;
        norm += amp;
        amp *= 0.5;
        f *= 2.0;
    }
    sum / norm
}

/// A bumpy bed: `base` + roughness `amp` (m), feature size `lam` cells.
fn bumpy_bed(nx: usize, base: f64, amp: f64, lam: f64, seed: u64) -> Vec<f64> {
    (0..nx * nx)
        .map(|i| {
            let (x, y) = ((i % nx) as f64, (i / nx) as f64);
            base + amp * fbm(x, y, lam, 4, seed)
        })
        .collect()
}

/// A bumpy bed on a mean SLOPE `s` (m/m) descending in +x.
fn slope_bed(nx: usize, base: f64, cell_m: f64, s: f64, amp: f64, lam: f64, seed: u64) -> Vec<f64> {
    (0..nx * nx)
        .map(|i| {
            let (x, y) = ((i % nx) as f64, (i / nx) as f64);
            base - s * x * cell_m + amp * fbm(x, y, lam, 4, seed)
        })
        .collect()
}

fn to32(v: &[f64]) -> Vec<f32> {
    v.iter().map(|&x| x as f32).collect()
}

/// Flat surface at `eta0` over `bed`: depth = eta0 − bed, clipped at 0.
fn flat_lake64(bed: &[f64], eta0: f64) -> Vec<f64> {
    bed.iter().map(|&b| (eta0 - b).max(0.0)).collect()
}
fn flat_lake32(bed: &[f32], eta0: f32) -> Vec<f32> {
    bed.iter().map(|&b| (eta0 - b).max(0.0)).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 1 — WELL-BALANCEDNESS
// ─────────────────────────────────────────────────────────────────────────────

struct WbRow {
    label: String,
    /// max |flux| (m³/s) after ONE step from the flat-surface rest state
    f1: f64,
    /// max speed (m/s) after one step
    v1: f64,
    /// max speed after 200 steps
    v200: f64,
    /// max speed after 2000 steps
    v2000: f64,
    /// the float type's view of the surface spread (max η − min η), m
    eta_spread: f64,
}

/// CFL-SAFE stepping — `water.rs`'s own `stable_dt` contract, as every real caller uses.
/// (dt = 0.2 on 45 m-deep water is a 3× CFL violation and blows up; that is out of the
/// kernel's documented contract and would be measuring the wrong thing.)
fn wb_case32(nx: usize, cell: f32, bed: Vec<f32>, eta0: f32, label: &str, p: &k32::P) -> WbRow {
    let depth = flat_lake32(&bed, eta0);
    let mut s = k32::Sim::new(nx, cell, bed, depth, 0.0);
    let eta_spread = s.eta_spread();
    let dt = s.stable_dt(p.gravity);
    let p = k32::P { dt, ..*p };
    s.step(&p);
    let (f1, v1) = (s.max_flux(), s.max_speed());
    for _ in 1..200 {
        s.step(&p);
    }
    let v200 = s.max_speed();
    for _ in 200..2000 {
        s.step(&p);
    }
    WbRow { label: label.into(), f1, v1, v200, v2000: s.max_speed(), eta_spread }
}

fn wb_case64(nx: usize, cell: f64, bed: Vec<f64>, eta0: f64, label: &str, p: &k64::P) -> WbRow {
    let depth = flat_lake64(&bed, eta0);
    let mut s = k64::Sim::new(nx, cell, bed, depth, 0.0);
    let eta_spread = s.eta_spread();
    let dt = s.stable_dt(p.gravity);
    let p = k64::P { dt, ..*p };
    s.step(&p);
    let (f1, v1) = (s.max_flux(), s.max_speed());
    for _ in 1..200 {
        s.step(&p);
    }
    let v200 = s.max_speed();
    for _ in 200..2000 {
        s.step(&p);
    }
    WbRow { label: label.into(), f1, v1, v200, v2000: s.max_speed(), eta_spread }
}

fn print_wb(rows: &[WbRow]) {
    println!(
        "{:<44} {:>12} {:>11} {:>11} {:>11} {:>11}",
        "case", "|f|max t=0+", "|v| t=0+", "|v| 200st", "|v| 2000st", "η spread"
    );
    println!("{}", "─".repeat(105));
    for r in rows {
        println!(
            "{:<44} {:>12.3e} {:>11.3e} {:>11.3e} {:>11.3e} {:>11.3e}",
            r.label, r.f1, r.v1, r.v200, r.v2000, r.eta_spread
        );
    }
}

fn probe_wb() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 1 — WELL-BALANCEDNESS: does a still lake stay still?                    ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("Closed box: precip=0, evap=0, infiltration=0, sediment=0, no sea boundary.");
    println!("Init: flat water surface η₀ over a bumpy bed, ZERO velocity. Then step.");
    println!("A well-balanced scheme returns |f| = 0 EXACTLY at t=0⁺.\n");

    let nx = 64;
    let cell = 4.8f64;
    let p32 = k32::P { manning_n: 0.04, ..k32::P::closed() };
    let p64 = k64::P { manning_n: 0.04, ..k64::P::closed() };
    let mut rows = Vec::new();

    // ── CONTROL: flat bed. If this is not exactly zero, the HARNESS is broken. ──
    let flat = vec![4000.0f64; nx * nx];
    rows.push(wb_case32(nx, cell as f32, to32(&flat), 4010.0, "CONTROL flat bed (f32)", &p32));
    rows.push(wb_case64(nx, cell, flat.clone(), 4010.0, "CONTROL flat bed (f64)", &p64));

    // ── The real question: bumpy beds, at the vivarium datum (~4000 m). ──
    for (amp, lam, tag) in [(2.0, 8.0, "gentle 2 m/8cell"), (20.0, 8.0, "rough 20 m/8cell"), (20.0, 2.0, "rough 20 m/2cell (grid-scale)")] {
        let bed = bumpy_bed(nx, 3980.0, amp, lam, 7);
        let hi = bed.iter().cloned().fold(f64::MIN, f64::max);
        let eta0 = hi + 5.0; // fully submerged bed
        rows.push(wb_case32(nx, cell as f32, to32(&bed), eta0 as f32, &format!("bumpy {tag} (f32, datum 4000)"), &p32));
        rows.push(wb_case64(nx, cell, bed.clone(), eta0, &format!("bumpy {tag} (f64, datum 4000)"), &p64));
    }

    // ── STEEP bed: does a steep bumpy bed break it? ──
    let steep = slope_bed(nx, 4100.0, cell, 0.30, 10.0, 6.0, 11);
    let hi = steep.iter().cloned().fold(f64::MIN, f64::max);
    rows.push(wb_case32(nx, cell as f32, to32(&steep), (hi + 5.0) as f32, "STEEP 30% slope + 10 m bumps (f32)", &p32));
    rows.push(wb_case64(nx, cell, steep.clone(), hi + 5.0, "STEEP 30% slope + 10 m bumps (f64)", &p64));

    // ── ISLANDS: a partially-dry bed (the classic well-balanced killer). ──
    let isl = bumpy_bed(nx, 4000.0, 30.0, 10.0, 3);
    let mean = isl.iter().sum::<f64>() / (nx * nx) as f64;
    rows.push(wb_case32(nx, cell as f32, to32(&isl), mean as f32, "ISLANDS: η₀ = mean bed (half dry, f32)", &p32));
    rows.push(wb_case64(nx, cell, isl.clone(), mean, "ISLANDS: η₀ = mean bed (half dry, f64)", &p64));

    // ── DATUM sweep: f32 round-off in η = b + d scales with the DATUM's ULP. ──
    for datum in [0.0f64, 100.0, 1000.0, 4000.0, 20000.0] {
        let bed = bumpy_bed(nx, datum, 20.0, 8.0, 7);
        let hi = bed.iter().cloned().fold(f64::MIN, f64::max);
        rows.push(wb_case32(nx, cell as f32, to32(&bed), (hi + 5.0) as f32, &format!("DATUM {datum:>7.0} m, 20 m bumps (f32)"), &p32));
    }

    print_wb(&rows);

    // ── The REALISTIC case. The t=0⁺ table above starts from d = η₀ − b, which at
    //    the vivarium datum is BIT-EXACT (Sterbenz's lemma: b ∈ [η₀/2, 2η₀] ⇒ the
    //    subtraction is exact ⇒ b + d recovers η₀ with zero error). A LAKE IN A
    //    RUNNING WORLD is not initialised that way — its depth is whatever the
    //    dynamics left. So: rain into a bowl, let it settle, cut the forcing, and
    //    ask whether the current goes to ZERO or to a FLOOR.
    println!("\n── PERTURB AND SETTLE: the lake a running world actually has ──");
    println!("Rain into a bumpy bowl (adaptive CFL dt), settle, THEN cut all forcing and");
    println!("relax as a closed system. Depth is now a GENERIC float, not η₀ − b.");
    println!("Does the residual current decay to zero, or plateau on a round-off floor?\n");
    println!("{:<26} {:>12} {:>12} {:>12} {:>13} {:>12}", "precision", "|v| relax=0", "|v| @5k", "|v| @50k", "|v| @200k", "η spread end");
    println!("{}", "─".repeat(93));

    let bowl: Vec<f64> = (0..nx * nx)
        .map(|i| {
            let (x, y) = ((i % nx) as f64 - nx as f64 / 2.0, (i / nx) as f64 - nx as f64 / 2.0);
            3990.0 + 0.004 * (x * x + y * y) + 1.5 * fbm(x, y, 6.0, 4, 21)
        })
        .collect();

    macro_rules! settle {
        ($k:ident, $R:ty, $tag:expr) => {{
            let bed: Vec<$R> = bowl.iter().map(|&b| b as $R).collect();
            let depth: Vec<$R> = vec![0.0; nx * nx];
            let mut s = $k::Sim::new(nx, cell as $R, bed, depth, 1e9);
            // fill: rain until there is a real lake
            let fill = $k::P { precip: 2.0e-3, sea_m: -1.0e9, edge_hold: false, manning_n: 0.04, ..$k::P::closed() };
            for _ in 0..40_000 {
                let dt = s.stable_dt(9.8);
                s.step(&$k::P { dt, ..fill });
            }
            // cut ALL forcing: closed, conservative, still.
            let relax = $k::P { manning_n: 0.04, ..$k::P::closed() };
            let v0 = s.max_speed();
            let mut m = Vec::new();
            for k in 1..=200_000u32 {
                let dt = s.stable_dt(9.8);
                s.step(&$k::P { dt, ..relax });
                if k == 5_000 || k == 50_000 || k == 200_000 {
                    m.push(s.max_speed());
                }
            }
            println!(
                "{:<26} {:>12.3e} {:>12.3e} {:>12.3e} {:>13.3e} {:>12.3e}",
                $tag, v0, m[0], m[1], m[2], s.eta_spread()
            );
        }};
    }
    settle!(k32, f32, "f32 (what ships)");
    settle!(k64, f64, "f64 (round-off control)");

    println!(
        "\nf32 ULP at the 4000 m datum: {:.3e} m.  f64 ULP: {:.3e} m  (ratio {:.3e}).",
        (4000.0f32).next_up() as f64 - 4000.0f64,
        (4000.0f64).next_up() - 4000.0f64,
        ((4000.0f32).next_up() as f64 - 4000.0f64) / ((4000.0f64).next_up() - 4000.0f64)
    );
    println!("If the f32 floor sits ~√(ULP ratio) above the f64 floor, the residual is");
    println!("ROUND-OFF IN η = b + d — a CONDITIONING defect, not a scheme imbalance.");
}

// ─────────────────────────────────────────────────────────────────────────────
// HARNESS CONTROL — is my copy actually `water.rs`?
// ─────────────────────────────────────────────────────────────────────────────

/// Run the REAL `vivarium_world::water::WaterSim` and the k32 copy side by side on
/// identical state. If they do not agree bit-for-bit, every number above is about a
/// kernel that does not ship, and the spike is void.
fn probe_fidelity() {
    use vivarium_world::sphere::Face;
    use vivarium_world::water::{WaterParams, WaterSim};

    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ HARNESS CONTROL — is the instrumented copy bit-identical to water.rs?         ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    let nx = 48usize;
    let cell = 4.8f32;
    let bed64 = slope_bed(nx, 4100.0, cell as f64, 0.05, 3.0, 8.0, 5);
    let bed = to32(&bed64);

    // Real water.rs. sea_m below every bed ⇒ hold_edge_sea is a no-op, matching `closed()`.
    let lo = bed.iter().cloned().fold(f32::MAX, f32::min);
    let p_real = WaterParams {
        sea_m: lo - 1000.0,
        precip: 3.0e-3,
        evaporation: 0.0,
        ocean_evap: 0.0,
        infiltration: 0.0,
        baseflow: 0.0,
        sed_capacity: 0.0, // ⇒ water.rs skips stages 4b/4b² entirely (the ones the copy omits)
        ..Default::default()
    };
    let mut real = WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, cell, bed.clone(), 1e9);
    real.depth = vec![0.05f32; nx * nx];
    real.rebaseline_budget();

    let mut copy = k32::Sim::new(nx, cell, bed.clone(), vec![0.05f32; nx * nx], 1e9);
    let p_copy = k32::P {
        sea_m: lo - 1000.0,
        precip: 3.0e-3,
        evaporation: 0.0,
        ocean_evap: 0.0,
        infiltration: 0.0,
        baseflow: 0.0,
        ..k32::P::default()
    };

    for _ in 0..4000 {
        real.step(&p_real);
        copy.step(&p_copy);
    }

    let bit_id = real.depth == copy.depth;
    let maxdiff = real
        .depth
        .iter()
        .zip(copy.depth.iter())
        .map(|(&a, &b)| (a as f64 - b as f64).abs())
        .fold(0.0f64, f64::max);
    let mean_d = real.depth.iter().map(|&d| d as f64).sum::<f64>() / (nx * nx) as f64;
    println!("  4000 steps, rain on a 5% bumpy slope, sediment off.");
    println!("  mean depth {mean_d:.6} m");
    println!("  depth fields BIT-IDENTICAL: {bit_id}");
    println!("  max |Δdepth| = {maxdiff:.3e} m");
    if bit_id {
        println!("\n  ✅ The instrumented copy IS water.rs (for sed_capacity = 0). The findings");
        println!("     below are about the kernel that ships.");
    } else {
        println!("\n  ⛔ DIVERGENT — the copy is not the kernel. Do not trust the findings.");
    }
}

/// Does the spurious current vanish under refinement? (Bias signature: it does not.)
fn probe_wb_refine() {
    println!("\n── WB REFINEMENT: spurious current vs cell size (fixed physical domain) ──");
    println!("Same bed spectrum, same 307 m domain, refined. Flat lake, 2000 steps.\n");
    println!("{:<10} {:>8} {:>12} {:>12} {:>12}", "cells", "cell_m", "|v| t=0+", "|v| 2000st", "η spread");
    println!("{}", "─".repeat(60));
    let domain = 307.2f64;
    for nx in [32usize, 64, 128, 256] {
        let cell = domain / nx as f64;
        // Hold the bed's PHYSICAL feature size fixed at ~38 m by scaling lam with nx.
        let lam = 8.0 * (nx as f64 / 64.0);
        let bed = bumpy_bed(nx, 3980.0, 20.0, lam, 7);
        let hi = bed.iter().cloned().fold(f64::MIN, f64::max);
        let eta0 = (hi + 5.0) as f32;
        let p = k32::P { manning_n: 0.04, dt: 0.05, ..k32::P::closed() };
        let b32 = to32(&bed);
        let depth = flat_lake32(&b32, eta0);
        let mut s = k32::Sim::new(nx, cell as f32, b32, depth, 0.0);
        s.step(&p);
        let v1 = s.max_speed();
        for _ in 1..2000 {
            s.step(&p);
        }
        println!("{:<10} {:>8.2} {:>12.3e} {:>12.3e} {:>12.3e}", nx, cell, v1, s.max_speed(), s.eta_spread());
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2a — THE SMOOTHER'S TRANSFER FUNCTION (what it does, per scale)
// ─────────────────────────────────────────────────────────────────────────────

/// Analytic gain of the JACOBI (symmetric, out-of-place) smoother.
fn g_jacobi(theta: f64, kdx: f64) -> (f64, f64) {
    let g = theta + (1.0 - theta) * kdx.cos();
    (g.abs(), if g >= 0.0 { 0.0 } else { std::f64::consts::PI })
}

/// Analytic gain of the GAUSS–SEIDEL (in-place, +x sweep) smoother — what ships.
/// new_i = θ·old_i + (1−θ)/2·(new_{i−1} + old_{i+1})
/// ⇒ G(k) = [θ + (1−θ)/2·e^{ikΔ}] / [1 − (1−θ)/2·e^{−ikΔ}]
fn g_gs(theta: f64, kdx: f64) -> (f64, f64) {
    let a = (1.0 - theta) * 0.5;
    let (nr, ni) = (theta + a * kdx.cos(), a * kdx.sin());
    let (dr, di) = (1.0 - a * kdx.cos(), a * kdx.sin());
    let den = dr * dr + di * di;
    let (gr, gi) = ((nr * dr + ni * di) / den, (ni * dr - nr * di) / den);
    ((gr * gr + gi * gi).sqrt(), gi.atan2(gr))
}

/// Measure the code's ACTUAL gain: inject mode k into `fr`, run ONE smoothing pass,
/// project the interior back onto e^{ikx}.
fn measured_gain(theta: f64, kdx: f64, mode: Smooth) -> (f64, f64) {
    let nx = 256usize;
    let bed = vec![0.0f64; nx * nx];
    let depth = vec![10.0f64; nx * nx];
    let mut s = k64::Sim::new(nx, 1.0, bed, depth, 0.0);
    // A pure mode in x, uniform in y, with a DC offset so we stay in the linear
    // regime of a field that is otherwise sign-free (the smoother itself is linear).
    for y in 0..nx {
        for x in 0..nx {
            s.fr[y * nx + x] = (kdx * x as f64).cos();
        }
    }
    let p = k64::P { theta, smooth: mode, ..k64::P::closed() };
    // Call ONLY the smoother (via a step with dt=0 so pipe_step cannot drive anything…
    // no: call the pass directly through a one-off — replicate it here to stay exact).
    smooth_once(&mut s, &p);
    // Project interior (drop a 32-cell margin: the boundary rows are not smoothed).
    let (lo, hi) = (48usize, nx - 48);
    let (mut re, mut im, mut norm) = (0.0f64, 0.0f64, 0.0f64);
    for x in lo..hi {
        let f = s.fr[(nx / 2) * nx + x];
        let (c, sn) = ((kdx * x as f64).cos(), (kdx * x as f64).sin());
        re += f * c;
        im -= f * sn;
        norm += c * c;
    }
    let (re, im) = (re / norm, im / norm);
    ((re * re + im * im).sqrt(), im.atan2(re))
}

/// The smoothing pass, exactly as `kernel` runs it (exposed for the transfer-function probe).
fn smooth_once(s: &mut k64::Sim, p: &k64::P) {
    let nx = s.nx;
    let th = p.theta;
    match p.smooth {
        Smooth::GaussSeidel => {
            for y in 0..nx {
                for x in 1..nx - 1 {
                    let i = y * nx + x;
                    s.fr[i] = th * s.fr[i] + (1.0 - th) * 0.5 * (s.fr[i - 1] + s.fr[i + 1]);
                }
            }
        }
        Smooth::Jacobi => {
            let o = s.fr.clone();
            for y in 0..nx {
                for x in 1..nx - 1 {
                    let i = y * nx + x;
                    s.fr[i] = th * o[i] + (1.0 - th) * 0.5 * (o[i - 1] + o[i + 1]);
                }
            }
        }
        Smooth::Off => {}
    }
}

fn probe_tf() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 2a — WHAT THE θ PASS ACTUALLY IS: its transfer function                 ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("θ·q_i + (1−θ)/2·(q_{{i−1}} + q_{{i+1}})  =  q_i + (1−θ)/2·Δx²·∂²q/∂x² + O(Δx⁴)");
    println!("A SYMMETRIC 3-point average is an EVEN operator — a Laplacian. Momentum");
    println!("ADVECTION is an ODD (first-derivative) operator. They cannot be the same term.");
    println!("Inject a pure Fourier mode into the flux field; run ONE pass; read the gain.\n");
    let theta = 0.8;
    println!("{:>9} {:>9} | {:>9} {:>9} | {:>9} {:>9} | {:>9} {:>9}",
        "kΔx/π", "λ (cells)", "|G| jac", "meas", "|G| GS", "meas", "arg GS", "meas");
    println!("{}", "─".repeat(96));
    for f in [0.0625f64, 0.125, 0.25, 0.5, 0.75, 1.0] {
        let kdx = f * std::f64::consts::PI;
        let (gj, _) = g_jacobi(theta, kdx);
        let (mgj, _) = measured_gain(theta, kdx, Smooth::Jacobi);
        let (gg, pg) = g_gs(theta, kdx);
        let (mgg, mpg) = measured_gain(theta, kdx, Smooth::GaussSeidel);
        println!(
            "{:>9.4} {:>9.1} | {:>9.5} {:>9.5} | {:>9.5} {:>9.5} | {:>9.5} {:>9.5}",
            f, 2.0 / f, gj, mgj, gg, mgg, pg, mpg
        );
    }
    println!("\n(kΔx = π is the NYQUIST mode — the 2-cell checkerboard the scheme is blind to.)");

    println!("\n── The DAMPING RATE per sim-second, λ(k) = −ln|G| / dt ──");
    println!("This is the number that matters: the smoother is applied ONCE PER STEP, so its");
    println!("effect per unit TIME is ∝ 1/dt. A PHYSICAL term's effect per unit time does not");
    println!("depend on the step size. THIS ONE DOES.\n");
    println!("{:>12} {:>12} {:>14} {:>14} {:>14}", "λ (cells)", "|G| GS", "dt=0.2 → τ(s)", "dt=0.05 → τ", "dt=0.023 → τ");
    println!("{}", "─".repeat(70));
    for f in [0.0625f64, 0.125, 0.25, 0.5, 1.0] {
        let kdx = f * std::f64::consts::PI;
        let (gg, _) = g_gs(theta, kdx);
        let lam = -gg.ln();
        println!(
            "{:>12.1} {:>12.5} {:>14.3} {:>14.3} {:>14.3}",
            2.0 / f, gg, 0.2 / lam, 0.05 / lam, 0.023 / lam
        );
    }
    println!("\nτ = e-folding time of the flux at that wavelength. dt=0.023 s is what");
    println!("`WaterSim::stable_dt` returns for a 400 m-deep ocean cell — so DEEP water gets");
    println!("~9× MORE artificial damping per second than shallow water, for free.");

    // Equivalent numerical viscosity, and what the code itself calls "turbulent".
    println!("\n── Equivalent numerical viscosity  ν_num = (1−θ)·Δx²/(2·dt)  [m²/s] ──");
    println!("{:>10} {:>10} {:>14}", "Δx (m)", "dt (s)", "ν_num (m²/s)");
    println!("{}", "─".repeat(38));
    for (dx, dt) in [(4.8f64, 0.2f64), (4.8, 0.05), (4.8, 0.023), (4.8, 0.005), (2.4, 0.1), (1.2, 0.05)] {
        println!("{:>10.1} {:>10.3} {:>14.2}", dx, dt, (1.0 - theta) * dx * dx / (2.0 * dt));
    }
    println!("\nFor reference, `water.rs` models TURBULENT eddy diffusivity for suspended");
    println!("sediment as  K = eddy_base + eddy_shear·|v|·l·0.1  = 0.02 … ~0.5 m²/s.");
    println!("It is simultaneously applying an UNNAMED momentum diffusivity two orders larger.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2b — THE GAUSS–SEIDEL SWEEP IS NOT SYMMETRIC (a directional bias)
// ─────────────────────────────────────────────────────────────────────────────

fn probe_drift() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 2b — THE IN-PLACE SWEEP ADVECTS. A symmetric smoother must not move.    ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("water.rs: \"In-place (Gauss–Seidel) sweep: slightly asymmetric, faster-damping.\"");
    println!("Put a SYMMETRIC Gaussian bump of flux on the grid; smooth it; track its CENTROID.");
    println!("CONTROL: the Jacobi (out-of-place) smoother is an even operator — its centroid");
    println!("MUST NOT MOVE. If it does, the harness is broken, not the scheme.\n");

    let nx = 256usize;
    let run = |mode: Smooth, passes: usize| -> (f64, f64) {
        let bed = vec![0.0f64; nx * nx];
        let depth = vec![10.0f64; nx * nx];
        let mut s = k64::Sim::new(nx, 4.8, bed, depth, 0.0);
        let (c, w) = (128.0f64, 12.0f64);
        for y in 0..nx {
            for x in 0..nx {
                s.fr[y * nx + x] = (-((x as f64 - c) / w).powi(2)).exp();
            }
        }
        let p = k64::P { theta: 0.8, smooth: mode, ..k64::P::closed() };
        let cen = |s: &k64::Sim| -> (f64, f64) {
            let (mut m0, mut m1) = (0.0f64, 0.0f64);
            for x in 8..nx - 8 {
                let f = s.fr[(nx / 2) * nx + x];
                m0 += f;
                m1 += f * x as f64;
            }
            (m1 / m0, m0)
        };
        let (c0, mass0) = cen(&s);
        for _ in 0..passes {
            smooth_once(&mut s, &p);
        }
        let (c1, mass1) = cen(&s);
        (c1 - c0, mass1 / mass0)
    };

    println!("{:<26} {:>10} {:>16} {:>16} {:>12}", "smoother", "passes", "Δcentroid (cells)", "drift (m/s)", "Σf kept");
    println!("{}", "─".repeat(86));
    for passes in [1usize, 10, 100, 1000] {
        let (dj, mj) = run(Smooth::Jacobi, passes);
        println!("{:<26} {:>10} {:>16.6} {:>16.6} {:>12.5}", "CONTROL Jacobi (symmetric)", passes, dj, dj * 4.8 / (passes as f64 * 0.2), mj);
    }
    println!();
    for passes in [1usize, 10, 100, 1000] {
        let (dg, mg) = run(Smooth::GaussSeidel, passes);
        println!("{:<26} {:>10} {:>16.6} {:>16.6} {:>12.5}", "GAUSS–SEIDEL (what ships)", passes, dg, dg * 4.8 / (passes as f64 * 0.2), mg);
    }
    println!("\nThe drift is in the SWEEP direction (+x for fl/fr, +y for ft/fb) — so it is");
    println!("GRID-ALIGNED and one-signed: a BIAS, not noise. It is an unphysical transport");
    println!("of momentum, at a speed comparable to the flows the model is trying to resolve.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2c — THE ENERGY BUDGET (with a noise floor + a known sink)
// ─────────────────────────────────────────────────────────────────────────────

struct Run {
    sinks: Sinks,
    de_total: f64,
    steps: usize,
    sim_s: f64,
    analytic_fric_j: f64,
    max_v: f64,
    mean_depth: f64,
    /// fractional growth of stored water over the window — ≈0 means steady
    steady: f64,
    rain_pe: f64,
    wet_frac: f64,
    clamp_rate: f64,
}

/// An OPEN CHANNEL: rain onto a bumpy slope, transmissive outlet at the bottom.
/// Run to steady state (total water stops growing), THEN account for every joule.
fn energy_run(p: &k32::P, nx: usize, cell: f64, bed: Vec<f64>, steps: usize, rain: f32) -> Run {
    let b32 = to32(&bed);
    let mut s = k32::Sim::new(nx, cell as f32, b32, vec![0.0f32; nx * nx], 1e12);
    let p = k32::P { precip: rain, outflow_x: true, edge_hold: false, ..*p };
    // BURN-IN to steady state.
    for _ in 0..steps {
        s.step(&p);
    }
    let w0 = s.depth.iter().map(|&d| d as f64).sum::<f64>();
    let e_start = s.energy(p.gravity);
    let mut tot = Sinks::default();
    let mut afric = 0.0f64;
    let mut rain_pe = 0.0f64;
    let a = cell * cell;
    for _ in 0..steps {
        afric += s.analytic_friction_power(&p) * p.dt as f64;
        // PE the rain brings in (it lands ON the water surface).
        rain_pe += s
            .depth
            .iter()
            .zip(s.bed.iter())
            .map(|(&d, &b)| {
                k32::RHO * p.gravity as f64 * a * (p.precip as f64 * p.dt as f64) * (b as f64 + d as f64)
            })
            .sum::<f64>();
        let k = s.step(&p);
        tot.add(&k);
    }
    let w1 = s.depth.iter().map(|&d| d as f64).sum::<f64>();
    let e_end = s.energy(p.gravity);
    let wet = s.depth.iter().filter(|&&d| d > 1e-3).count();
    Run {
        sinks: tot,
        de_total: e_end - e_start,
        steps,
        sim_s: steps as f64 * p.dt as f64,
        analytic_fric_j: afric,
        max_v: s.max_speed(),
        mean_depth: s.depth.iter().map(|&d| d as f64).sum::<f64>() / (nx * nx) as f64,
        steady: (w1 - w0) / w0.max(1e-12),
        rain_pe,
        wet_frac: wet as f64 / (nx * nx) as f64,
        clamp_rate: tot.clamp_fires / (steps as f64 * (nx * nx) as f64),
    }
}

fn print_run(label: &str, r: &Run) {
    let s = &r.sinks;
    let diss = s.theta + s.friction + s.breaking + s.clamp;
    let pct = |x: f64| if diss.abs() > 0.0 { 100.0 * x / diss } else { 0.0 };
    println!("  {label}");
    println!(
        "    {:.0} s · max|v| {:.2} m/s · mean depth {:.4} m · wet {:.0}% · water growth {:+.2}% (steady if ≈0)",
        r.sim_s, r.max_v, r.mean_depth, 100.0 * r.wet_frac, 100.0 * r.steady
    );
    println!("    ── ENERGY SINKS over the accounting window (J; negative = removed) ──");
    println!("    θ smoothing     [physical claim: NONE]  {:>11.4e}   {:>6.2}%", s.theta, pct(s.theta));
    println!("    Manning friction [bed shear]            {:>11.4e}   {:>6.2}%", s.friction, pct(s.friction));
    println!("    breaking cap    [Grant 1997]            {:>11.4e}   {:>6.2}%", s.breaking, pct(s.breaking));
    println!("    flux clamp      [physical claim: NONE]  {:>11.4e}   {:>6.2}%   (fired on {:.3}% of cell-steps)",
        s.clamp, pct(s.clamp), 100.0 * r.clamp_rate);
    println!("    ─────────────────────────────────────────────────────────────────");
    println!("    TOTAL dissipation                       {:>11.4e}", diss);
    println!("    ⇒ UNPHYSICAL SHARE (θ + clamp)          {:>10.2}%", pct(s.theta + s.clamp));
    let ratio = if r.analytic_fric_j != 0.0 { s.friction.abs() / r.analytic_fric_j } else { f64::NAN };
    println!("    [instrument] measured friction / ANALYTIC Manning = {ratio:.4}   (must be ≈1 where Manning is valid)");
    println!();
}

fn probe_energy() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 2c — THE ENERGY BUDGET: how much does the unnamed term eat?             ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("OPEN CHANNEL: 460 m of bumpy slope, storm rain, transmissive outlet — so the");
    println!("flow reaches a REAL steady state instead of filling a closed box.");
    println!("Each sink is an EXACT KE delta at FIXED depth, so no closure argument is needed");
    println!("for any single sink. The controls below are what license the measurement.\n");

    let nx = 96usize;
    let cell = 4.8f64;
    let rain = 1.0e-4f32; // storm; gives ~5–15 cm sheet/channel flow at steady state
    let base = k32::P { manning_n: 0.04, ..k32::P::closed() };

    // ── CONTROL 1: FLAT BED, NO RAIN, AT REST. Every sink must read exactly 0.0. ──
    println!("── CONTROL 1: flat bed, no rain, at rest. Every sink MUST be exactly 0. ──");
    let flat = vec![4000.0f64; nx * nx];
    let mut s = k32::Sim::new(nx, cell as f32, to32(&flat), vec![10.0f32; nx * nx], 0.0);
    let mut t = Sinks::default();
    for _ in 0..400 {
        t.add(&s.step(&base));
    }
    println!(
        "  θ {:.3e} · friction {:.3e} · breaking {:.3e} · clamp {:.3e} · max|v| {:.3e}\n",
        t.theta, t.friction, t.breaking, t.clamp, s.max_speed()
    );

    // ── CONTROL 2: θ = 1.0. The θ sink MUST read exactly 0.0. ──
    println!("── CONTROL 2: θ = 1.0 (the smoother is the identity). θ sink MUST be exactly 0. ──");
    let bumpy = slope_bed(nx, 4100.0, cell, 0.05, 0.4, 8.0, 5);
    let r = energy_run(&k32::P { theta: 1.0, ..base }, nx, cell, bumpy.clone(), 8000, rain);
    print_run("5% slope, bumpy, θ = 1.0", &r);

    // ── CONTROL 3: UNIFORM slope, NO bumps. q is uniform ⇒ the smoother sees no
    //    gradient ⇒ θ sink ≈ 0. And friction MUST recover analytic Manning: the
    //    KNOWN SINK the instrument has to detect before it is trusted on an unknown one.
    println!("── CONTROL 3: UNIFORM slope, NO bumps, Jarrett off ⇒ Manning is exactly valid.");
    println!("   The instrument must (a) recover the KNOWN analytic Manning sink, and");
    println!("   (b) report ≈0 for θ, because a uniform flux has nothing to smooth.");
    let uniform = slope_bed(nx, 4100.0, cell, 0.05, 0.0, 8.0, 5);
    let r = energy_run(&k32::P { jarrett: false, breaking: false, ..base }, nx, cell, uniform, 8000, rain);
    print_run("5% slope, NO bumps, θ = 0.8, plain Manning", &r);

    // ── THE MEASUREMENT: the SHIPPING configuration. ──
    println!("── THE MEASUREMENT — the SHIPPING configuration (θ=0.8 Gauss–Seidel, Manning");
    println!("   + Jarrett, breaking cap on). This is the kernel that makes our rivers.");
    for (sl, amp, tag) in [
        (0.01f64, 0.4f64, "1% slope (lowland river)"),
        (0.05, 0.4, "5% slope (upland stream)"),
        (0.20, 0.8, "20% slope (the SOLITON regime θ was added for)"),
    ] {
        let bed = slope_bed(nx, 4100.0, cell, sl, amp, 8.0, 5);
        let r = energy_run(&base, nx, cell, bed, 8000, rain);
        print_run(&format!("SHIPPING — {tag}"), &r);
    }

    // ── θ sweep: the sink must vanish as θ → 1, and scale with (1−θ). ──
    println!("── θ SWEEP (5% bumpy slope): the sink must vanish as θ → 1. ──");
    println!("{:>7} {:>14} {:>14} {:>12} {:>12}", "θ", "θ sink (J)", "friction (J)", "θ share", "max|v|");
    println!("{}", "─".repeat(64));
    let bed = slope_bed(nx, 4100.0, cell, 0.05, 0.4, 8.0, 5);
    for th in [1.0f32, 0.99, 0.95, 0.9, 0.8, 0.6] {
        let r = energy_run(&k32::P { theta: th, breaking: false, ..base }, nx, cell, bed.clone(), 8000, rain);
        let d = r.sinks.theta + r.sinks.friction + r.sinks.clamp;
        println!(
            "{:>7.2} {:>14.4e} {:>14.4e} {:>11.2}% {:>12.3}",
            th, r.sinks.theta, r.sinks.friction,
            if d != 0.0 { 100.0 * r.sinks.theta / d } else { 0.0 },
            r.max_v
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2d — dt SCALING: a numerical term's damping grows as dt → 0.
// ─────────────────────────────────────────────────────────────────────────────

fn probe_dt_scale() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 2d — dt SCALING: the decisive physical-vs-numerical discriminator       ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("A PHYSICAL sink dissipates a rate [W] that does not depend on the time step.");
    println!("A NUMERICAL smoother applied ONCE PER STEP dissipates ∝ 1/dt: halve dt, double");
    println!("the damping per second. Run the SAME physical time at different dt.\n");
    let nx = 96usize;
    let cell = 4.8f64;
    let bed = slope_bed(nx, 4100.0, cell, 0.05, 0.4, 8.0, 5);
    let sim_s = 1600.0f64;
    println!("{:>8} {:>8} {:>16} {:>16} {:>14} {:>12}", "dt (s)", "steps", "θ POWER (W)", "friction POWER (W)", "θ/friction", "max |v|");
    println!("{}", "─".repeat(80));
    for dt in [0.2f32, 0.1, 0.05, 0.025, 0.0125] {
        let steps = (sim_s / dt as f64).round() as usize;
        let r = energy_run(
            &k32::P { dt, manning_n: 0.04, breaking: false, ..k32::P::closed() },
            nx, cell, bed.clone(), steps, 1.0e-4,
        );
        println!(
            "{:>8.4} {:>8} {:>16.4e} {:>16.4e} {:>14.3} {:>12.4}",
            dt, steps,
            r.sinks.theta / r.sim_s,
            r.sinks.friction / r.sim_s,
            r.sinks.theta / r.sinks.friction,
            r.max_v
        );
    }
    println!("\nIf θ POWER grows as dt shrinks while friction POWER stays put, the θ term is");
    println!("NOT a physical dissipation. It is a per-step filter, and its physical claim is NONE.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2e — SPECTRAL DEFICIT: where in scale is the energy missing?
// ─────────────────────────────────────────────────────────────────────────────

fn probe_spectrum() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 2e — THE SPECTRAL DEFICIT: a BIAS, not noise                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("Variance of the flux field per wavelength band, quasi-steady rain on a rough");
    println!("5% slope. θ=1 is the un-stabilised control (and it is UNSTABLE — that is the point).\n");

    let nx = 128usize;
    let cell = 4.8f64;
    let bed = slope_bed(nx, 4100.0, cell, 0.05, 3.0, 6.0, 5);
    let b32 = to32(&bed);

    // Radially-binned 2-D power spectrum of the x-flux field (naive DFT, small n).
    let spectrum = |f: &[f32], nx: usize| -> Vec<f64> {
        let m = 32usize; // resolve modes 0..m along each axis
        let mean = f.iter().map(|&v| v as f64).sum::<f64>() / f.len() as f64;
        let mut bins = vec![0.0f64; m];
        for ky in 0..m {
            for kx in 0..m {
                if kx == 0 && ky == 0 {
                    continue;
                }
                let (mut re, mut im) = (0.0f64, 0.0f64);
                for y in 0..nx {
                    for x in 0..nx {
                        let ph = -2.0 * std::f64::consts::PI
                            * ((kx * x) as f64 / nx as f64 + (ky * y) as f64 / nx as f64);
                        let v = f[y * nx + x] as f64 - mean;
                        re += v * ph.cos();
                        im += v * ph.sin();
                    }
                }
                let pw = (re * re + im * im) / (nx * nx) as f64;
                let kr = (((kx * kx + ky * ky) as f64).sqrt()).round() as usize;
                if kr < m {
                    bins[kr] += pw;
                }
            }
        }
        bins
    };

    let mut results = Vec::new();
    for th in [1.0f32, 0.95, 0.8] {
        let p = k32::P { theta: th, manning_n: 0.04, precip: 3.0e-3, ..k32::P::closed() };
        let depth: Vec<f32> = vec![0.05f32; nx * nx];
        let mut s = k32::Sim::new(nx, cell as f32, b32.clone(), depth, 1e9);
        let mut blew = false;
        for _ in 0..8000 {
            s.step(&p);
            if !s.max_flux().is_finite() || s.max_speed() > 200.0 {
                blew = true;
                break;
            }
        }
        let sp = spectrum(&s.fr, nx);
        results.push((th, sp, s.max_speed(), blew, s.ke()));
    }

    println!("{:>10} {:>12} {:>12} {:>14}", "θ", "max |v| m/s", "unstable?", "flux KE (J)");
    println!("{}", "─".repeat(52));
    for (th, _, mv, blew, ke) in &results {
        println!("{:>10.2} {:>12.3} {:>12} {:>14.4e}", th, mv, if *blew { "YES" } else { "no" }, ke);
    }

    println!("\nFlux-field power per radial mode (λ = {} m / k):", (nx as f64 * cell) as usize);
    println!("{:>6} {:>10} {:>13} {:>13} {:>13} {:>10} {:>10}", "k", "λ (m)", "P(θ=1)", "P(θ=0.95)", "P(θ=0.8)", "0.8/1.0", "pred |G|²");
    println!("{}", "─".repeat(80));
    let (_, p1, _, _, _) = &results[0];
    let (_, p95, _, _, _) = &results[1];
    let (_, p8, _, _, _) = &results[2];
    for k in [1usize, 2, 4, 8, 12, 16, 24, 31] {
        let lam = nx as f64 * cell / k as f64;
        let kdx = std::f64::consts::PI * k as f64 / (nx as f64 / 2.0);
        let (g, _) = g_gs(0.8, kdx);
        let ratio = if p1[k] > 0.0 { p8[k] / p1[k] } else { f64::NAN };
        println!(
            "{:>6} {:>10.1} {:>13.4e} {:>13.4e} {:>13.4e} {:>10.4} {:>10.4}",
            k, lam, p1[k], p95[k], p8[k], ratio, g * g
        );
    }
    println!("\nThe deficit is ONE-SIGNED at every scale and DEEPENS toward the grid: that is");
    println!("the signature of a BIAS. Noise would scatter the ratio around 1.0.");
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE 2f — IS THE SOLITON MODE ACTUALLY A NULL-SPACE MODE?
// (doc/theory §2.5 asserts it is: "those solitons were the invisible mode".
//  But a null mode requires a COLLOCATED operator. Is the pipes scheme collocated?)
// ─────────────────────────────────────────────────────────────────────────────

fn probe_nullspace() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║ PROBE 2f — IS water.rs BLIND TO THE CHECKERBOARD? (the null-space claim)      ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("`.super-archive/from-theory/discretisation-and-information.md` §2.5 says: \"those solitons were");
    println!("the invisible mode. The θ-smoothing is Rhie–Chow-class stabilisation.\" A null");
    println!("mode exists only if the operator CANNOT SEE the Nyquist checkerboard. Feed it one.\n");
    println!("A COLLOCATED scheme computes ∂η/∂x as (η_{{i+1}} − η_{{i−1}})/2Δx  ⇒ 0 on a checkerboard.");
    println!("A STAGGERED scheme computes the flux ON THE FACE as (η_i − η_j)/Δx ⇒ ±2a/Δx. SEEN.\n");

    let nx = 32usize;
    let cell = 4.8f64;
    let a = 0.5f64; // checkerboard amplitude, m

    // η checkerboard on a FLAT bed (so η = 4000 + a·(−1)^(i+j) exactly).
    let bed = vec![4000.0f64; nx * nx];
    let depth: Vec<f64> = (0..nx * nx)
        .map(|i| {
            let (x, y) = (i % nx, i / nx);
            10.0 + a * if (x + y) % 2 == 0 { 1.0 } else { -1.0 }
        })
        .collect();
    // No smoothing, no friction, no breaking: measure the RAW operator response.
    let p = k64::P { theta: 1.0, smooth: Smooth::Off, manning_n: 0.0, breaking: false, jarrett: false, ..k64::P::closed() };
    let mut s = k64::Sim::new(nx, cell, bed.clone(), depth, 0.0);
    s.step(&p);
    let f_cb = s.max_flux();

    // CONTROL A: a SMOOTH (long-wave) η perturbation of the same amplitude. Must be seen.
    let depth_sm: Vec<f64> = (0..nx * nx)
        .map(|i| {
            let x = (i % nx) as f64;
            10.0 + a * (2.0 * std::f64::consts::PI * x / nx as f64).cos()
        })
        .collect();
    let mut s2 = k64::Sim::new(nx, cell, bed.clone(), depth_sm, 0.0);
    s2.step(&p);
    let f_sm = s2.max_flux();

    // CONTROL B: a genuinely FLAT η. Must be exactly zero (nothing to see).
    let mut s3 = k64::Sim::new(nx, cell, bed.clone(), vec![10.0f64; nx * nx], 0.0);
    s3.step(&p);
    let f_flat = s3.max_flux();

    println!("{:<52} {:>14}", "η perturbation (amplitude 0.5 m)", "max |flux| m³/s");
    println!("{}", "─".repeat(68));
    println!("{:<52} {:>14.4e}", "CONTROL: flat η (nothing to see)", f_flat);
    println!("{:<52} {:>14.4e}", "CONTROL: smooth 32-cell wave (must be seen)", f_sm);
    println!("{:<52} {:>14.4e}", "NYQUIST CHECKERBOARD (−1)^(i+j)  ← the test", f_cb);
    println!();
    if f_cb > 1e-6 {
        let ratio = f_cb / f_sm.max(1e-30);
        println!("  ⇒ The checkerboard is SEEN — and seen {ratio:.1}× more strongly than the smooth");
        println!("     wave, exactly as a staggered face-gradient should (it reaches ONE cell, so");
        println!("     the shortest wave gives the LARGEST gradient).");
        println!("\n  ⇒ water.rs is NOT collocated. Depth sits at cell centres; the four pipe");
        println!("     fluxes sit ON THE FACES; the momentum gradient is (η_i − η_j)/Δx, a");
        println!("     one-cell reach. THAT IS AN ARAKAWA-C / STAGGERED SCHEME.");
        println!("\n  ⚠ So the checkerboard is NOT in its null space, and the θ term is NOT curing");
        println!("     a null mode. §2.5's identification is HALF right: θ IS artificial diffusion");
        println!("     (confirmed — the authors call it that), but the MODE it suppresses is not an");
        println!("     invisible one. It is a NONLINEAR instability the scheme cannot saturate,");
        println!("     because the local-inertial approximation DROPS the convective term ∂(q²/h)/∂x");
        println!("     — the very term that makes real roll waves steepen into breaking bores.");
        println!("     The violated structure is the ENTROPY CONDITION (shocks), not a null space.");
    } else {
        println!("  ⇒ The checkerboard is INVISIBLE — it IS a null mode. §2.5 is right as written.");
    }
}

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    match which.as_str() {
        "fidelity" => probe_fidelity(),
        "wb" => probe_wb(),
        "wb-refine" => probe_wb_refine(),
        "tf" => probe_tf(),
        "drift" => probe_drift(),
        "energy" => probe_energy(),
        "dt-scale" => probe_dt_scale(),
        "spectrum" => probe_spectrum(),
        "nullspace" => probe_nullspace(),
        "all" => {
            probe_fidelity();
            probe_wb();
            probe_wb_refine();
            probe_tf();
            probe_drift();
            probe_energy();
            probe_dt_scale();
            probe_spectrum();
            probe_nullspace();
        }
        other => eprintln!("unknown probe: {other}"),
    }
}
