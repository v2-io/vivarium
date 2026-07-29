//! CLIP CENSUS — step (2) of the order-of-work in
//! `DECISIONS[water-runs-outside-its-published-validity-envelope]`:
//! *"Count the clip activation rates — the audit needs those, not θ's."*
//!
//! That entry lists the one-sided clips in `water.rs` and calls them
//! **bias by construction** (a sign-definite operation cannot average out),
//! and it says of the positivity clamp: *"a positivity clamp is a silent MASS
//! SOURCE if it fires. Unprobed."* Nobody has counted any of them. What has
//! been reported instead is the `froude()` gauge — and that gauge's `max Fr`
//! reads a bit-identical **2.00**, which is the cap, not a measurement.
//!
//! ## What this probe measures, and why in two ways
//!
//! **(A) Real kernel, by fork-differencing.** `WaterSim`'s flux state is
//! private and the type is not `Clone`, so a state cannot be branched. But
//! `step` is deterministic (there is a test), so a *trajectory* can be:
//! run a fresh sim `k` steps, then take ONE more step with a clip's parameter
//! neutralised, and diff the depth field against the same trajectory taken with
//! the clip on. Cells that differ are exactly the cells that clip touched at
//! step `k+1`, on the real kernel, on its own real trajectory. Since 2026-07-24
//! `froude_cap` and `jarrett_n_cap` are `WaterParams` fields, so this now works
//! for the breaking cap and the Jarrett roughness ceiling.
//!
//! Better still: on the neutralised branch, `froude()` reports the **uncapped**
//! Froude field — the number the gauge has never been able to show.
//!
//! **(B) Transcription, for the clips that have no parameter.** The rectifier
//! (`.max(0.0)`), the dry-sill gate (`hflow < 1e-4`), the outflow clamp and the
//! positivity clamp are hardcoded, so (A) cannot reach them. The `null_space`
//! probe's pinned transcription counts all of them (`Guards`), and is re-pinned
//! here against the real kernel **on this workload** rather than on the one it
//! was originally pinned on — a transcription pinned elsewhere is not evidence
//! here.
//!
//! Workload is the one that produced the 5.7% figure (`redteam_probe` probe H):
//! 60-epoch eroded land at L19, 96², a 2 m sheet, every non-hydrodynamic stage
//! off.

// The shared transcription carries machinery this probe does not use (the
// periodic geometry, the spectral helpers) — it is the `null_space` probe's too.
#![allow(dead_code)]

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

#[path = "../null_space/water_op.rs"]
mod water_op;

const FACE: Face = Face::ZPos;
const LEVEL: u8 = 19;
const OI: u32 = 108_500;
const OJ: u32 = 186_350;
const NX: usize = 96;
const CELL_M: f32 = 19.0;
/// Neutralising value for a cap we want switched off. Large enough that the
/// `min` can never bind, small enough not to make an f32 comparison degenerate.
const OFF: f32 = 1.0e9;

fn base_params() -> WaterParams {
    WaterParams {
        sea_m: -1.0e6,
        precip: 0.0,
        evaporation: 0.0,
        infiltration: 0.0,
        ocean_evap: 0.0,
        baseflow: 0.0,
        sed_capacity: 0.0,
        ..Default::default()
    }
}

fn eroded_bed() -> Vec<f32> {
    let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    f.erode(&FluvialParams { epochs: 60, ..Default::default() });
    f.h.clone()
}

/// A fresh sim on `bed` with a 2 m sheet — the base state probe H used.
fn seeded(bed: &[f32]) -> WaterSim {
    let mut w = WaterSim::new(FACE, LEVEL, (OI, OJ), NX, CELL_M, bed.to_vec(), 1.0e5);
    for d in w.depth.iter_mut() {
        *d = 2.0;
    }
    w
}

/// Steepest axial downhill bed slope at `i` (the same statistic the kernel's own
/// sediment stage uses), so "steep" means what the kernel means by it.
fn bed_slope(bed: &[f32], i: usize) -> f32 {
    let (x, y) = (i % NX, i / NX);
    let mut s = 0.0f32;
    let mut probe = |j: usize| s = s.max((bed[i] - bed[j]) / CELL_M);
    if x > 0 {
        probe(i - 1);
    }
    if x + 1 < NX {
        probe(i + 1);
    }
    if y > 0 {
        probe(i - NX);
    }
    if y + 1 < NX {
        probe(i + NX);
    }
    s
}

struct Arm {
    name: &'static str,
    /// The clip neutralised on the single differenced step.
    tweak: fn(WaterParams) -> WaterParams,
}

fn mean(v: &[f32]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.iter().map(|&x| x as f64).sum::<f64>() / v.len() as f64
}

fn main() {
    println!("=== WATER CLIP CENSUS — order-of-work step (2) ===\n");
    println!("workload: L{LEVEL} {NX}² @ {CELL_M} m, 60-epoch eroded bed, 2 m sheet,");
    println!("          sediment / rain / evap / infiltration / groundwater OFF\n");

    // ── §0 What the workload actually IS. ──────────────────────────────────
    // Every Froude statistic on record was taken on this footprint, described
    // as "60-epoch eroded land". Both halves of that phrase are worth checking
    // before any number taken on it is quoted.
    {
        println!("── §0 The workload, checked rather than described ────────────────");
        let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
        let before = f.h.clone();
        f.erode(&FluvialParams { epochs: 60, ..Default::default() });
        let after = f.h.clone();
        let sea = vivarium_world::sea_level::derived_sea_level_m(0) as f32;
        let lo = before.iter().cloned().fold(f32::MAX, f32::min);
        let hi = before.iter().cloned().fold(f32::MIN, f32::max);
        let maxd = before.iter().zip(&after).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
        let moved = before.iter().zip(&after).filter(|(a, b)| a != b).count();
        println!("   derived sea level          : {sea:.1} m");
        println!("   bed range                  : {lo:.1} .. {hi:.1} m   (relief {:.1} m)", hi - lo);
        println!("   cells at or below sea      : {}/{}", before.iter().filter(|&&v| v <= sea).count(), NX * NX);
        println!("   cells moved by 60 epochs   : {moved}/{}   max |Δh| = {maxd:.4} m", NX * NX);
        println!("   ⇒ read the numbers below with that in view.\n");
    }

    let bed = eroded_bed();
    let p = base_params();

    // ── Sanity: the gauge, as currently reported. ───────────────────────────
    {
        let mut w = seeded(&bed);
        println!("── The gauge as it is reported today ─────────────────────────────");
        println!("   step   max Fr    % wet cells with any pipe Fr > 1.5");
        for s in 1..=400 {
            w.step(&p);
            if s % 100 == 0 {
                let (mx, sup) = w.froude();
                println!("   {s:4}   {mx:7.4}   {:6.2}%", 100.0 * sup);
            }
        }
        println!("   (`max Fr` bit-identical at the cap is the tell: a capped pipe reads");
        println!("    f/(h·l)/√(g·h) = froude_cap EXACTLY, by algebra, not by measurement.)\n");
    }

    // ── (A) Real-kernel fork census. ────────────────────────────────────────
    let arms = [
        Arm { name: "Froude breaking cap", tweak: |q| WaterParams { froude_cap: OFF, ..q } },
        Arm { name: "Jarrett n ceiling", tweak: |q| WaterParams { jarrett_n_cap: OFF, ..q } },
        Arm { name: "Jarrett feedback (whole term)", tweak: |q| WaterParams { jarrett_slope: 0.0, ..q } },
        Arm { name: "θ flux smoothing", tweak: |q| WaterParams { theta: 1.0, ..q } },
    ];
    let checkpoints = [1usize, 10, 25, 50, 100, 200, 300, 399];

    for arm in &arms {
        println!("── (A) REAL KERNEL — incidence of: {} ─────────", arm.name);
        println!("   after   wet cells   cells touched   % of wet   ⟨slope⟩ touched : all wet   ⟨depth⟩ touched : all wet");
        for &k in &checkpoints {
            // Branch 1: k steps, then one more with the clip ON.
            let mut a = seeded(&bed);
            for _ in 0..=k {
                a.step(&p);
            }
            // Branch 2: the SAME k steps, then one with the clip neutralised.
            let mut b = seeded(&bed);
            for _ in 0..k {
                b.step(&p);
            }
            let (fr_capped, sup_capped) = if k > 0 { b.froude() } else { (0.0, 0.0) };
            b.step(&(arm.tweak)(p));
            let (fr_free, sup_free) = b.froude();

            let wet: Vec<usize> = (0..NX * NX).filter(|&i| a.depth[i] >= 0.05).collect();
            let touched: Vec<usize> =
                wet.iter().copied().filter(|&i| a.depth[i].to_bits() != b.depth[i].to_bits()).collect();

            let sl_t: Vec<f32> = touched.iter().map(|&i| bed_slope(&bed, i)).collect();
            let sl_w: Vec<f32> = wet.iter().map(|&i| bed_slope(&bed, i)).collect();
            let d_t: Vec<f32> = touched.iter().map(|&i| a.depth[i]).collect();
            let d_w: Vec<f32> = wet.iter().map(|&i| a.depth[i]).collect();

            println!(
                "   {:5}   {:9}   {:13}   {:7.2}%   {:8.4} : {:8.4}   {:8.3} : {:8.3}",
                k,
                wet.len(),
                touched.len(),
                100.0 * touched.len() as f64 / wet.len().max(1) as f64,
                mean(&sl_t),
                mean(&sl_w),
                mean(&d_t),
                mean(&d_w),
            );
            if arm.name.starts_with("Froude") {
                println!(
                    "           └ Froude at this step: WITH cap max {fr_capped:.4} / {:.2}% > 1.5   ·   \
                     CAP LIFTED for one step: max {fr_free:.4} / {:.2}% > 1.5",
                    100.0 * sup_capped,
                    100.0 * sup_free
                );
            }
        }
        println!();
    }

    // ── (A2) The positivity clamp as a mass source. ─────────────────────────
    // Every reservoir stage is off, so total_water can only change through the
    // depth field. The physics is exactly conservative; any drift is the
    // positivity clamp minting, plus honest f32 summation noise.
    {
        println!("── (A2) POSITIVITY CLAMP as a mass source (real kernel) ──────────");
        let mut w = seeded(&bed);
        w.rebaseline_budget();
        let t0 = w.total_water();
        for s in 1..=400 {
            w.step(&p);
            if s % 100 == 0 {
                let d = w.budget_drift();
                println!("   step {s:4}   budget drift {d:+.6e} m·cells   ({:+.3e} relative)", d / t0);
            }
        }
        println!("   (positive drift = mass minted. `depth = (…).max(0.0)` is the only");
        println!("    source available with every reservoir stage switched off.)\n");
    }

    // ── (A3) The live instrument. ───────────────────────────────────────────
    // `WaterSim::clips()` landed 2026-07-29 under Joseph's re-key grant. Before
    // it, the clips with no parameter could only be counted in the pinned
    // transcription (B). Now the real kernel counts its own, and (B) becomes a
    // cross-check on the transcription rather than the only source.
    {
        println!("── (A3) REAL KERNEL, self-counted (`WaterSim::clips`) ────────────");
        let mut w = seeded(&bed);
        println!("   step   rectifier   dry-sill   breaking   outflow-clamp   positivity");
        for s in 1..=400 {
            w.step(&p);
            if s == 1 || s % 100 == 0 {
                let c = w.clips();
                let pc = |n: usize, d: usize| 100.0 * n as f64 / d.max(1) as f64;
                println!(
                    "   {s:4}   {:8.3}%   {:7.3}%   {:7.3}%   {:12.3}%   {:9.3}%",
                    pc(c.rectifier, c.pipes),
                    pc(c.dry_sill, c.pipes),
                    pc(c.breaking, c.pipes),
                    pc(c.outflow_clamp, c.cells),
                    pc(c.positivity, c.cells),
                );
            }
        }
        println!();
    }

    // ── (B) Transcription census — cross-check against (A3). ────────────────
    {
        println!("── (B) TRANSCRIPTION — cross-check on (A3) ───────────────────────");
        let bed64: Vec<f64> = bed.iter().map(|&b| b as f64).collect();
        let geom = water_op::Geom::ClosedBox { nx: NX, bed: bed64 };
        let pp = water_op::PipeParams::kernel_default(CELL_M as f64);

        // Pin FIRST, on THIS workload. A transcription pinned on another bed is
        // not evidence here.
        let mut sim = seeded(&bed);
        let mut st = vec![0.0f64; 5 * NX * NX];
        for i in 0..NX * NX {
            st[5 * i] = sim.depth[i] as f64;
        }
        let mut worst = 0.0f64;
        println!("   step   rectifier   dry-sill   breaking   outflow-clamp   positivity   (of pipes / cells)");
        for s in 1..=400 {
            let mut g = water_op::Guards::default();
            sim.step(&p);
            water_op::step(&mut st, &geom, &pp, &mut g);
            for i in 0..NX * NX {
                worst = worst.max((sim.depth[i] as f64 - st[5 * i]).abs());
            }
            if s % 100 == 0 || s == 1 {
                let pc = |n: usize, d: usize| 100.0 * n as f64 / d.max(1) as f64;
                println!(
                    "   {s:4}   {:8.3}%   {:7.3}%   {:7.3}%   {:12.3}%   {:9.3}%   ({} / {})",
                    pc(g.rectifier_active, g.pipes),
                    pc(g.dry_sill, g.pipes),
                    pc(g.breaking, g.pipes),
                    pc(g.clamped, g.cells),
                    pc(g.positivity, g.cells),
                    g.pipes,
                    g.cells
                );
            }
        }
        let mean_d: f64 = st.iter().step_by(5).sum::<f64>() / (NX * NX) as f64;
        println!("\n   PIN on this workload: max |Δdepth| over 400 steps = {worst:.3e} m");
        println!("                          against a mean depth of {mean_d:.3} m");
        println!("   (the transcription is f64 and the kernel f32; the pin is the warrant");
        println!("    for reading the counts above as the kernel's own clip rates.)");
    }
}
