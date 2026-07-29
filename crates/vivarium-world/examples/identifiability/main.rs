//! IDENTIFIABILITY PROBE — the free probe derived in `#obs-routing-curl-spiral`
//! Working Notes: *"a uniform Manning-type conveyance `k_m` (and any `η_w`) is
//! structurally inert on routed flow — perturb a uniform `k_m` and assert the
//! routed field is bit-identical; a probe that must return no change, and
//! exactly the inert-undeclared-parameter class the nomotheke exists to catch."*
//!
//! Source claim, read as recorded in `DECISIONS[the-router-is-a-scalar-pretending
//! -to-be-a-vector-and-p-is-the-bias]` from Coatléven & Chauveau: *"the choice of
//! the water mobility function η_w has NO influence on the water flux strength
//! q_w"* and *"only the CONTRASTS of the coefficient k_m will impact q_w."*
//!
//! ## The probe splits in two, and the interesting half is not the router
//!
//! **§1 The router.** `accumulate_drainage` weights receivers by
//! `(drop/dist)^P`, **normalised**. A uniform multiplicative factor on the
//! driving gradient — which is what a uniform `k_m` is — cancels in the
//! normalisation identically. So the literature's claim is true of our router,
//! and true *vacuously*: there is no conveyance term in it to be inert. The
//! probe still has to run, because "cancels identically" is an algebra claim
//! about code that also sorts, fills and thresholds, and any absolute-scale
//! constant hiding in that machinery would break it.
//!
//! **§2 `water.rs`, where a conveyance actually exists.** `manning_n` IS a
//! conveyance coefficient. The sharp form of the same statement: at hydrological
//! steady state the discharge is fixed by **mass conservation** (rain ×
//! contributing area) and cannot depend on roughness at all, while the depth
//! must absorb it — Manning normal flow gives `h ∝ n^{3/5}`, `v ∝ n^{-3/5}`,
//! `q` invariant. If that holds, `manning_n` is **unidentifiable from any
//! discharge-derived quantity** — including the drainage area the erosion tier
//! consumes — and identifiable only through depth and velocity.
//!
//! **§3 And then Jarrett.** `n = min(n_base + jarrett_slope·S, cap)` makes the
//! roughness a function of the local instantaneous surface slope. That is
//! precisely the conversion of a **uniform** conveyance into a **contrast** —
//! and contrasts, says the same literature, are exactly what *does* move `q`.
//! Measured here rather than argued.

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

const FACE: Face = Face::ZPos;
const LEVEL: u8 = 19;
const OI: u32 = 108_500;
const OJ: u32 = 186_350;
const NX: usize = 96;

fn main() {
    println!("=== IDENTIFIABILITY PROBE ===\n");
    router_uniform_conveyance();
    water_uniform_roughness();
}

// ─────────────────────────────────────────────────────────────────────────────
// §1 The router
// ─────────────────────────────────────────────────────────────────────────────
fn router_uniform_conveyance() {
    println!("── §1 ROUTER: is a uniform conveyance inert on the routed field? ──");

    let mut f = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    f.erode(&FluvialParams { epochs: 20, ..Default::default() });
    let h0 = f.h.clone();

    let route = |h: &[f32]| -> Vec<f32> {
        let mut g = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
        g.h = h.to_vec();
        g.drainage_surface().mfd
    };

    let d_ref = route(&h0);

    // ARM A — uniform gain. ×2 is exact in binary floating point, so if the
    // normalisation cancels it algebraically it must cancel it BIT-EXACTLY. A
    // non-power-of-two gain would confound "the structure cancels it" with
    // "rounding is small", which is the whole distinction being tested.
    let h_x2: Vec<f32> = h0.iter().map(|&v| v * 2.0).collect();
    let d_x2 = route(&h_x2);
    let identical = d_ref.iter().zip(&d_x2).all(|(a, b)| a.to_bits() == b.to_bits());
    let worst = d_ref.iter().zip(&d_x2).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    println!("   uniform ×2 on the driving field:  bit-identical = {identical}   max |Δ| = {worst:.3e} m²");

    // ARM A' — a non-power-of-two uniform gain. Structure should still cancel it
    // to within rounding; a LARGE difference here would mean the cancellation is
    // not structural at all.
    let h_x13: Vec<f32> = h0.iter().map(|&v| v * 1.3).collect();
    let d_x13 = route(&h_x13);
    let rel13 = d_ref
        .iter()
        .zip(&d_x13)
        .map(|(a, b)| if *a > 0.0 { ((a - b) / a).abs() } else { 0.0 })
        .fold(0.0f32, f32::max);
    println!("   uniform ×1.3 (inexact):           max relative |Δ| = {rel13:.3e}");

    // ARM B — the SENSITIVITY CONTROL. A probe whose answer is "no change" is
    // worthless unless the same instrument can register a change. Perturb the
    // field with a CONTRAST of the same magnitude the ×1.3 gain applied.
    let h_contrast: Vec<f32> = h0
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let (x, y) = ((i % NX) as f32, (i / NX) as f32);
            v * (1.0 + 0.3 * (0.2 * x).sin() * (0.2 * y).cos())
        })
        .collect();
    let d_con = route(&h_contrast);
    let rel_con = d_ref
        .iter()
        .zip(&d_con)
        .map(|(a, b)| if *a > 0.0 { ((a - b) / a).abs() } else { 0.0 })
        .fold(0.0f32, f32::max);
    let moved = d_ref.iter().zip(&d_con).filter(|(a, b)| a.to_bits() != b.to_bits()).count();
    println!("   CONTRAST of the same size:        max relative |Δ| = {rel_con:.3e}   ({moved}/{} cells moved)", NX * NX);

    // ── ATTRIBUTION. The uniform gain was NOT inert, which the algebra says it
    //    must be. `fill_depressions` carries `const EPS: f32 = 1e-3` — an
    //    ABSOLUTE metre used to orient flats — and an absolute length cannot
    //    survive a rescaling of relief. If that is the mechanism, the cells that
    //    moved must live in the FILLED region. Measured, not inferred.
    let mut g = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    g.h = h0.clone();
    let surf = g.drainage_surface();
    let filled: Vec<bool> = surf.fill_depth.iter().map(|&d| d > 0.0).collect();
    let n_filled = filled.iter().filter(|&&b| b).count();
    let moved2: Vec<usize> =
        (0..NX * NX).filter(|&i| d_ref[i].to_bits() != d_x2[i].to_bits()).collect();
    let moved_in_fill = moved2.iter().filter(|&&i| filled[i]).count();
    println!("\n   ATTRIBUTION of the ×2 non-invariance (`fill_depressions`'s EPS = 1e-3 m,");
    println!("   an ABSOLUTE length that cannot survive a rescaling of relief):");
    println!("      cells in the filled region:        {n_filled}/{} ({:.1}%)", NX * NX, 100.0 * n_filled as f64 / (NX * NX) as f64);
    println!("      cells whose drainage moved:        {}", moved2.len());
    println!("      …of those, inside the fill:        {moved_in_fill} ({:.1}%)", 100.0 * moved_in_fill as f64 / moved2.len().max(1) as f64);

    // The EPS hypothesis is the obvious one and the numbers above decide it.
    // Second candidate, and it is not a defect but a DESIGN FACT: the outlet
    // (base-level) set is "coast plus edge" — coast being defined against an
    // ABSOLUTE sea datum. Rescaling terrain therefore moves the BOUNDARY
    // CONDITION, which is not a conveyance perturbation at all.
    let mut g2 = Fluvial::from_prior(0, FACE, LEVEL, OI, OJ, NX);
    g2.h = h_x2.clone();
    let surf2 = g2.drainage_surface();
    let sea = vivarium_world::sea_level::derived_sea_level_m(0) as f32;
    let below1 = h0.iter().filter(|&&v| v <= sea).count();
    let below2 = h_x2.iter().filter(|&&v| v <= sea).count();
    let recv_diff = surf.recv.iter().zip(&surf2.recv).filter(|(a, b)| a != b).count();
    println!("\n   SECOND CANDIDATE — `drainage_surface` takes its outlet (base-level) set");
    println!("   from `self.outlets()` against `derived_sea_level_m` = {sea:.1} m, an ABSOLUTE");
    println!("   datum. Rescaling relief therefore moves the BOUNDARY CONDITION, not a");
    println!("   conveyance — so the ×2 arm was never a conveyance perturbation at all:");
    println!("      cells at or below sea, ×1: {below1}/{}      ×2: {below2}/{}", NX * NX, NX * NX);
    println!("      receiver-tree entries that changed: {recv_diff}/{}", NX * NX);
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// §2/§3 water.rs, where a conveyance really exists
// ─────────────────────────────────────────────────────────────────────────────

struct Steady {
    depth: f64,
    speed: f64,
    q: f64,
    fr: f32,
}

/// Rain-fed tilted channel with a sea-held outlet at the low end, run to
/// hydrological steady state. Statistics are taken over the interior only — the
/// held outlet column is a boundary condition, not flow.
fn steady_channel(p_in: &WaterParams, nx: usize, l: f32, slope: f32, steps: usize) -> Steady {
    let bed: Vec<f32> = (0..nx * nx).map(|i| 60.0 - slope * l * (i % nx) as f32).collect();
    // Hold ONLY the low-x-end edge column: sea just above the last column's bed.
    let sea = 60.0 - slope * l * (nx - 2) as f32;
    let p = WaterParams { sea_m: sea, ..*p_in };
    let mut w = WaterSim::new_at_sea(FACE, 21, (0, 0), nx, l, bed, 1.0e7, sea);
    for d in w.depth.iter_mut() {
        *d = 0.0;
    }
    for _ in 0..steps {
        w.step(&p);
    }
    let r = w.to_region();
    let interior = |i: usize| {
        let (x, y) = (i % nx, i / nx);
        (8..nx - 8).contains(&x) && (8..nx - 8).contains(&y)
    };
    let idx: Vec<usize> = (0..nx * nx).filter(|&i| interior(i)).collect();
    let depth = idx.iter().map(|&i| r.depth[i] as f64).sum::<f64>() / idx.len() as f64;
    let speed = idx
        .iter()
        .map(|&i| ((r.vx[i] * r.vx[i] + r.vy[i] * r.vy[i]).sqrt()) as f64)
        .sum::<f64>()
        / idx.len() as f64;
    let q = idx
        .iter()
        .map(|&i| (r.depth[i] * (r.vx[i] * r.vx[i] + r.vy[i] * r.vy[i]).sqrt()) as f64)
        .sum::<f64>()
        / idx.len() as f64;
    Steady { depth, speed, q, fr: w.froude().0 }
}

fn water_uniform_roughness() {
    println!("── §2/§3 water.rs: is `manning_n` identifiable from discharge? ────");
    let (nx, l, slope) = (64usize, 4.8f32, 0.03f32);
    // dt = 0.02 s, not the shipped 0.2 s: the roll-wave probe's §2b measured the
    // steady normal flow to be a FUNCTION OF dt at 0.2 s (~8% slow, one-sided),
    // and a probe about steady-state scaling cannot be run on a dt-contaminated
    // steady state.
    let base = WaterParams {
        dt: 0.02,
        precip: 1.0e-3,
        evaporation: 0.0,
        infiltration: 0.0,
        ocean_evap: 0.0,
        baseflow: 0.0,
        sed_capacity: 0.0,
        ..Default::default()
    };
    let steps = 60_000;

    println!("\n   ARM 1 — UNIFORM roughness (Jarrett feedback OFF), n doubled 0.04 → 0.08");
    println!("   Manning normal flow predicts:  q ratio 1.000 (mass conservation)");
    println!("                                  h ratio {:.4} = 2^(3/5)", 2.0f64.powf(0.6));
    println!("                                  v ratio {:.4} = 2^(-3/5)", 2.0f64.powf(-0.6));
    let u1 = steady_channel(&WaterParams { jarrett_slope: 0.0, manning_n: 0.04, ..base }, nx, l, slope, steps);
    let u2 = steady_channel(&WaterParams { jarrett_slope: 0.0, manning_n: 0.08, ..base }, nx, l, slope, steps);
    println!("\n              n = 0.04      n = 0.08      ratio     predicted");
    println!("   depth   {:10.5}   {:10.5}   {:8.4}   {:8.4}", u1.depth, u2.depth, u2.depth / u1.depth, 2.0f64.powf(0.6));
    println!("   speed   {:10.5}   {:10.5}   {:8.4}   {:8.4}", u1.speed, u2.speed, u2.speed / u1.speed, 2.0f64.powf(-0.6));
    println!("   q       {:10.5}   {:10.5}   {:8.4}   {:8.4}", u1.q, u2.q, u2.q / u1.q, 1.0);
    println!("   max Fr  {:10.3}   {:10.3}", u1.fr, u2.fr);

    println!("\n   ARM 2 — the SAME doubling with the Jarrett feedback ON (shipped)");
    println!("   n = min(n_base + 1.6·S, 0.13) is a CONTRAST, not a uniform coefficient.");
    let j1 = steady_channel(&WaterParams { manning_n: 0.04, ..base }, nx, l, slope, steps);
    let j2 = steady_channel(&WaterParams { manning_n: 0.08, ..base }, nx, l, slope, steps);
    println!("\n              n_base 0.04   n_base 0.08   ratio     uniform-theory says");
    println!("   depth   {:10.5}   {:10.5}   {:8.4}   {:8.4}", j1.depth, j2.depth, j2.depth / j1.depth, 2.0f64.powf(0.6));
    println!("   speed   {:10.5}   {:10.5}   {:8.4}   {:8.4}", j1.speed, j2.speed, j2.speed / j1.speed, 2.0f64.powf(-0.6));
    println!("   q       {:10.5}   {:10.5}   {:8.4}   {:8.4}", j1.q, j2.q, j2.q / j1.q, 1.0);
    println!("   max Fr  {:10.3}   {:10.3}", j1.fr, j2.fr);

    println!("\n   ARM 3 — the CONSEQUENCE for the erosion coupling.");
    println!("   Drainage area A is a discharge-derived quantity ⇒ inert in n (ARM 1).");
    println!("   But the sediment capacity is C = k·|v|·slope — VELOCITY, not discharge.");
    println!("   capacity ratio implied by ARM 1's speeds: {:.4}", u2.speed / u1.speed);
    println!("   ⇒ roughness is invisible to the routed field and visible to the carve.");
}
