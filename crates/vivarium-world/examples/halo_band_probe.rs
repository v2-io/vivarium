//! **How far does a tile's boundary contract reach into its interior?**
//!
//! `#obs-tile-outlets-grade-away-the-basins` and `#obs-lakes-are-routed-over-not-carved-away`
//! measure *that* the contract matters at fine grain (259 m apart at the beacon)
//! and *that* it barely matters at planet grain (within 10 % at L9). Neither
//! measures the quantity a halo design actually needs: **the reach**. If boundary
//! influence dies out $w$ cells in, then a halo of depth $w$ is not a declared
//! approximation but an exact statement, and the interior beyond $w$ is final
//! under its own key regardless of its neighbours.
//!
//! This probe measures reach by **moving the boundary away and watching the
//! interior stop changing**. One fixed `CORE` footprint is carved inside windows
//! of growing halo depth $d$:
//!
//! ```text
//!   d=0    the shipped tile: CORE carved standalone, its own perimeter is the contract
//!   d=4    CORE carved inside a (CORE+8)^2 window; the outer 4 rings are discarded
//!   d=8, 16, 32, ...                        (the halo is computed and thrown away)
//! ```
//!
//! Every arm reports the *same* core cells, differing only in how far the
//! artificial boundary sits from them. If the core converges as $d$ grows, the
//! increment between successive $d$ is the honest halo-depth curve — a Cauchy
//! reading, not a claim that the widest arm is truth (it has a boundary too).
//! If the core does **not** converge, overlap alone is not a repair and the
//! design needs iterated exchange; that outcome is a finding, not a failure.
//!
//! Two spoilers this probe exists to catch, both nonlocal by construction and
//! both invisible to a kinematic-wave estimate of reach:
//!
//! - **Priority-Flood is a global operation.** A spill point anywhere can raise
//!   cells anywhere in one epoch, so influence need not travel at the incision
//!   wave's celerity.
//! - **The outlet set changes the basin partition instantly.** Under
//!   `BaseLevelSink` an interior cell may drain to a different outlet from epoch
//!   1 — a reassignment, not a propagation.
//!
//! So the band hypothesis is stated in the form that can lose: **bit-identity**
//! of the core beyond the band, reported as the fraction of core cells that are
//! bit-identical between successive arms, alongside the magnitude profile by
//! distance from the arm's own boundary.
//!
//! **The chaos control, without which none of the above means anything.** A
//! landscape evolution model can be chaotic: two carves differing by a millimetre
//! in one cell may diverge into different channel networks, and then a core will
//! fail to converge under *any* halo depth for reasons that have nothing to do
//! with boundaries. The `PERTURB` arm is the widest window with **one cell in its
//! outermost ring raised by 1 mm** — a perturbation with no physical content,
//! placed where a halo of any depth in this sweep would have absorbed it. If the
//! core diverges as far from that twin as it does across halo depths, then
//! pointwise agreement is not an achievable target and a seam repair has to be
//! judged on structure and statistics instead. That reading would not be a
//! failure of the repair; it would be a correction to what the repair can promise.
//!
//! Store-free and world-free: every input is a pure function of (seed, cell), and
//! the precipitation weight is normalized by **one common constant** across all
//! arms (the widest window's mean) so that the arms differ in domain extent and
//! in nothing else. The shipped path normalizes per tile; that is a second,
//! separable defect and folding it in here would confound the reach measurement.
//!
//! Run: `cargo run --release --example halo_band_probe`
//! Knobs (all env, all printed): `VIVARIUM_SEED`, `VIVARIUM_LEVEL`,
//! `VIVARIUM_FACE`, `VIVARIUM_OI`, `VIVARIUM_OJ`, `VIVARIUM_CORE`,
//! `VIVARIUM_EPOCHS`, `VIVARIUM_STRIDE`, `VIVARIUM_HALOS` (comma-separated).

use vivarium_world::erosion::{EdgeContract, Fluvial, FluvialParams};
use vivarium_world::sphere::{CellId, Face};

const DEFAULT_SEED: u64 = 17_425_063_241_017_297_386;

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

/// Raw fated precipitation jitter over a window — **unnormalized**. The caller
/// divides by one common constant so every arm sees the same rain field.
fn precip_raw(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
    let mut w: Vec<f32> = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
            w.push(vivarium_world::climate::precip_jitter_factor(seed, cell) as f32);
        }
    }
    w
}

/// The core sub-block of an arm's field: `core^2` cells starting `d` in from the
/// arm's own origin, row-major.
fn extract_core(h: &[f32], nx: usize, d: usize, core: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(core * core);
    for j in 0..core {
        for i in 0..core {
            out.push(h[(d + j) * nx + (d + i)]);
        }
    }
    out
}

/// Mean and max `|a - b|`, and the fraction of cells that are **bit-identical**.
fn compare(a: &[f32], b: &[f32]) -> (f64, f64, f64) {
    let (mut sum, mut max, mut same) = (0.0f64, 0.0f64, 0usize);
    for (x, y) in a.iter().zip(b.iter()) {
        let d = (x - y).abs() as f64;
        sum += d;
        if d > max {
            max = d;
        }
        if x.to_bits() == y.to_bits() {
            same += 1;
        }
    }
    (sum / a.len().max(1) as f64, max, same as f64 / a.len().max(1) as f64)
}

/// Mean `|a - b|` over core cells binned by Chebyshev distance from the **core's
/// own perimeter** — which, for the `d = 0` arm, is the contract boundary itself.
/// Bins are coarse (powers of two) so the tail has cells in it.
///
/// Returned per bin: `(depth, cells, mean |a-b|, mean carve magnitude, ratio)`.
/// The **ratio** is the load-bearing column. Raw `|a-b|` confounds boundary
/// influence with how much a cell erodes at all — a trunk in the middle of the
/// core moves hundreds of metres under any arm, an untouched interfluve moves
/// metres — so a raw profile can read as "the interior is more affected" when it
/// only means "the interior is where the erosion is". Dividing by the local carve
/// magnitude `|h - prior|` asks the question that was meant: **what fraction of
/// this ring's carving did the boundary change?**
fn ring_profile(a: &[f32], b: &[f32], prior: &[f32], core: usize) -> Vec<(usize, usize, f64, f64, f64)> {
    let edges = [0usize, 1, 2, 4, 8, 16, 32, 64, 128];
    let mut acc = vec![(0usize, 0.0f64, 0.0f64); edges.len()];
    for y in 0..core {
        for x in 0..core {
            let r = x.min(y).min(core - 1 - x).min(core - 1 - y);
            let bin = edges.iter().rposition(|&e| r >= e).unwrap_or(0);
            let i = y * core + x;
            acc[bin].0 += 1;
            acc[bin].1 += (a[i] - b[i]).abs() as f64;
            acc[bin].2 += (b[i] - prior[i]).abs() as f64;
        }
    }
    edges
        .iter()
        .zip(acc.iter())
        .filter(|(_, (n, _, _))| *n > 0)
        .map(|(&e, &(n, s, c))| {
            let (m, cm) = (s / n as f64, c / n as f64);
            (e, n, m, cm, if cm > 0.0 { m / cm } else { 0.0 })
        })
        .collect()
}

fn main() {
    let seed = env_u64("VIVARIUM_SEED", DEFAULT_SEED);
    let level = env_u64("VIVARIUM_LEVEL", 13) as u8;
    let face = Face::from_index(env_u64("VIVARIUM_FACE", 1) as u8);
    let core = env_u64("VIVARIUM_CORE", 64) as usize;
    let epochs = env_u64("VIVARIUM_EPOCHS", 300) as u32;
    let stride = env_u64("VIVARIUM_STRIDE", 50) as u32;
    let halos: Vec<usize> = std::env::var("VIVARIUM_HALOS")
        .ok()
        .map(|v| v.split(',').filter_map(|t| t.trim().parse().ok()).collect())
        .unwrap_or_else(|| vec![0, 4, 8, 16, 32, 64, 96]);
    let d_max = *halos.iter().max().expect("at least one halo depth");
    // Core origin, given as the origin of the CORE block (not of the widest arm).
    let (coi, coj) = (env_u64("VIVARIUM_OI", 736) as u32, env_u64("VIVARIUM_OJ", 5472) as u32);

    let cell_km = vivarium_world::sample::cell_size_m(level, vivarium_world::planet::Planet::EARTH.radius_m) / 1000.0;
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed) as f32;

    println!("== how far the boundary contract reaches: one core, growing halo ==");
    println!(
        "seed {seed}  f{} L{level}  core {core}^2 at ({coi},{coj}) = {:.0} km  halos {:?}  {epochs} epochs  sea {sea:.1} m",
        face.index(),
        core as f64 * cell_km,
        halos,
    );
    println!("cell {cell_km:.2} km; widest arm {}^2 = {:.0} km", core + 2 * d_max, (core + 2 * d_max) as f64 * cell_km);
    println!("store: none opened, none written. All inputs are pure functions of (seed, cell).");
    println!("rain: one common normalization constant across arms (widest window's mean).\n");

    // One common rain normalization so arms differ only in domain extent.
    let (woi, woj) = (coi - d_max as u32, coj - d_max as u32);
    let wide = core + 2 * d_max;
    let rain_mean = {
        let w = precip_raw(seed, face, level, woi, woj, wide);
        w.iter().map(|v| *v as f64).sum::<f64>() / w.len() as f64
    };

    // The uncarved prior over the core — the denominator of the ring ratio.
    let prior_core = {
        let mut v = Vec::with_capacity(core * core);
        for j in 0..core as u32 {
            for i in 0..core as u32 {
                let cell = CellId::from_face_ij(face, coi + i, coj + j, level);
                v.push(vivarium_world::gen::initial_topography_m(seed, cell, level) as f32);
            }
        }
        v
    };

    // Carve one arm: halo depth `d`, optionally with a 1 mm bump on one cell of
    // the outermost ring (the chaos control). Returns the core at every rung.
    let carve = |d: usize, perturb: bool| -> Vec<(u32, Vec<f32>)> {
        let nx = core + 2 * d;
        let (oi, oj) = (coi - d as u32, coj - d as u32);
        let mut f = Fluvial::from_surface(seed, face, level, oi, oj, nx, |c| {
            vivarium_world::gen::initial_topography_m(seed, c, level)
        });
        // Every arm is a partial tile, so every arm infers BaseLevelSink — the
        // shipped contract. The halo is the only thing that varies.
        assert_eq!(f.edge_contract(), EdgeContract::BaseLevelSink, "arm must be a partial tile");
        f.set_uplift_rate(vivarium_world::uplift::uplift_rate_tile(seed, face, level, oi, oj, nx));
        f.set_precip_weight(precip_raw(seed, face, level, oi, oj, nx).iter().map(|v| v / rain_mean as f32).collect());
        if perturb {
            // One cell, outermost ring, +1 mm. No physical content; a halo of any
            // depth in this sweep would have absorbed it.
            f.h[nx / 2] += 1e-3;
        }
        let mut chain = Vec::new();
        let mut done = 0u32;
        while done < epochs {
            let k = stride.min(epochs - done);
            f.erode(&FluvialParams { epochs: k, ..Default::default() });
            done += k;
            chain.push((done, extract_core(&f.h, nx, d, core)));
        }
        chain
    };

    // chains[arm][rung] = (epochs, core heights); the last entry is the PERTURB twin.
    let mut chains: Vec<Vec<(u32, Vec<f32>)>> = halos.iter().map(|&d| carve(d, false)).collect();
    let perturbed = carve(d_max, true);
    let widest = halos.len() - 1;

    // ---- Convergence of the core as the boundary moves away. ----
    println!("-- core vs the WIDEST arm (d = {d_max}): does moving the boundary away stop changing the interior? --");
    println!("{:>7} | {}", "epochs", halos.iter().map(|d| format!("{:>21}", format!("d={d} mean/max/same"))).collect::<String>());
    let rungs = chains[0].len();
    for r in 0..rungs {
        let (k, _) = &chains[0][r];
        let mut line = format!("{k:>7} |");
        for a in 0..halos.len() {
            let (m, mx, s) = compare(&chains[a][r].1, &chains[halos.len() - 1][r].1);
            line.push_str(&format!(" {m:>7.2} {mx:>6.0} {:>5.1}%", 100.0 * s));
        }
        println!("{line}");
    }

    println!("\n-- successive increments: |arm(d_i) - arm(d_{{i+1}})| on the core (the Cauchy reading) --");
    println!("{:>7} | {}", "epochs", halos.windows(2).map(|w| format!("{:>16}", format!("{}->{} mean/max", w[0], w[1]))).collect::<String>());
    for r in 0..rungs {
        let (k, _) = &chains[0][r];
        let mut line = format!("{k:>7} |");
        for a in 0..halos.len() - 1 {
            let (m, mx, _) = compare(&chains[a][r].1, &chains[a + 1][r].1);
            line.push_str(&format!(" {m:>8.2} {mx:>7.0}"));
        }
        println!("{line}");
    }

    // ---- The chaos control: does a 1 mm bump do what a 96-cell halo does? ----
    println!("\n-- CHAOS CONTROL: the widest arm against its twin, identical but for +1 mm on one outer-ring cell --");
    println!("   If this column tracks the halo columns above, pointwise agreement is not an achievable");
    println!("   target and the divergence above is not evidence about boundaries.");
    println!(
        "{:>7} | {:>10} {:>8} {:>8} | {:>12} {:>10}",
        "epochs", "perturb dh", "max", "same", "d=0 vs wide", "ratio"
    );
    for r in 0..rungs {
        let (k, _) = &chains[0][r];
        let (pm, pmx, ps) = compare(&perturbed[r].1, &chains[widest][r].1);
        let (hm, _, _) = compare(&chains[0][r].1, &chains[widest][r].1);
        println!(
            "{k:>7} | {pm:>10.2} {pmx:>8.0} {:>7.1}% | {hm:>12.2} {:>10.2}",
            100.0 * ps,
            if pm > 0.0 { hm / pm } else { f64::INFINITY }
        );
    }

    // ---- Is the influence a BAND, or is it everywhere? ----
    println!("\n-- the band test: |arm(d=0) - arm(d={d_max})| by distance from the d=0 arm's own boundary --");
    println!("   (a band hypothesis predicts the RATIO falls to zero at some depth; a nonlocal");
    println!("    mechanism -- the fill's global spill, or a reassigned outlet set -- predicts it does not.");
    println!("    `carve` is mean |h - prior| in that ring: the denominator that stops a trunk in the");
    println!("    middle of the core reading as boundary influence.)");
    println!("{:>7} | {:>10} {:>7} {:>10} {:>10} {:>8}", "epochs", "ring depth", "cells", "mean |dh|", "carve", "ratio");
    for r in [0usize, rungs / 2, rungs - 1] {
        let (k, _) = &chains[0][r];
        for (e, n, m, c, ratio) in ring_profile(&chains[0][r].1, &chains[widest][r].1, &prior_core, core) {
            println!("{k:>7} | {e:>10} {n:>7} {m:>10.2} {c:>10.2} {ratio:>8.3}");
        }
    }

    // ---- If pointwise agreement is unreachable, does STRUCTURE converge? ----
    println!("\n-- structural convergence: core statistics per arm at {epochs} epochs --");
    println!("   The design question the pointwise columns cannot answer. If these settle with d while");
    println!("   the pointwise columns do not, then a halo's honest promise is a core whose *structure*");
    println!("   is the neighbourhood's, not a core that matches it cell by cell.");
    let read = |h: &[f32]| {
        let mut f = Fluvial::from_surface(seed, face, level, coi, coj, core, |_| 0.0);
        f.set_edge_contract(EdgeContract::NoFluxWall);
        f.h = h.to_vec();
        f.drainage_surface()
    };
    println!(
        "{:>6} | {:>9} {:>9} {:>9} | {:>8} {:>10} {:>9}",
        "halo d", "mean h", "relief", "carve", "dep>1m", "trunk 1e10", "basin sh"
    );
    let mut rows: Vec<(String, f64, f64, f64, usize, f64, f32)> = Vec::new();
    for (a, &d) in halos.iter().enumerate() {
        let h = &chains[a][rungs - 1].1;
        let ds = read(h);
        let mean = h.iter().map(|v| *v as f64).sum::<f64>() / h.len() as f64;
        let (lo, hi) = h.iter().fold((f32::MAX, f32::MIN), |(l, g), &v| (l.min(v), g.max(v)));
        let carve = h.iter().zip(prior_core.iter()).map(|(a, b)| (a - b).abs() as f64).sum::<f64>() / h.len() as f64;
        let trunk = h.iter().zip(ds.mfd.iter()).filter(|(&e, _)| e > sea).map(|(_, &m)| m).fold(0.0f32, f32::max);
        rows.push((
            format!("{d}"),
            mean,
            (hi - lo) as f64,
            carve,
            ds.stats.depression_cells,
            trunk as f64 / 1e10,
            ds.stats.largest_basin_share,
        ));
    }
    {
        let h = &perturbed[rungs - 1].1;
        let ds = read(h);
        let mean = h.iter().map(|v| *v as f64).sum::<f64>() / h.len() as f64;
        let (lo, hi) = h.iter().fold((f32::MAX, f32::MIN), |(l, g), &v| (l.min(v), g.max(v)));
        let carve = h.iter().zip(prior_core.iter()).map(|(a, b)| (a - b).abs() as f64).sum::<f64>() / h.len() as f64;
        let trunk = h.iter().zip(ds.mfd.iter()).filter(|(&e, _)| e > sea).map(|(_, &m)| m).fold(0.0f32, f32::max);
        rows.push((
            "1mm".into(),
            mean,
            (hi - lo) as f64,
            carve,
            ds.stats.depression_cells,
            trunk as f64 / 1e10,
            ds.stats.largest_basin_share,
        ));
    }
    for (d, mean, relief, carve, dep, trunk, share) in &rows {
        println!("{d:>6} | {mean:>9.1} {relief:>9.1} {carve:>9.1} | {dep:>8} {trunk:>10.3} {share:>9.4}");
    }
    println!("   (the `1mm` row is the chaos twin of the widest arm: the spread between it and d={d_max}");
    println!("    is the floor below which no statistic here can resolve a halo's effect.)");

    println!("\n-- scope --");
    println!("   * The widest arm is not truth: it has a boundary too, {d_max} cells out. Convergence is");
    println!("     read from the successive increments, not from the distance to that arm.");
    println!("   * Every arm carves under BaseLevelSink. This measures the reach of THAT contract's");
    println!("     boundary, which is what a halo of depth d would displace -- not a comparison of");
    println!("     contracts (that is `beacon_contract_probe`).");
    println!("   * Rain is normalized by one common constant, so the per-tile renormalization of the");
    println!("     shipped path is deliberately absent here; it is a separable defect.");
}
