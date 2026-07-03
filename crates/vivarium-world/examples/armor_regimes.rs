//! Armor/scour regime probe (§2b): three invariants nature guarantees.
//!
//! STATUS 2026-07-03 (four rounds in — see git history for the arc):
//!   regime 2 (slack basin takes mud, stays soft): PASS, robust.
//!   regime 1 (transport-limited armoring): PASSED pre-eddy-diffusion
//!     (natural-supply armor 0.41→0.84 on storm-cycle timescale); FAILS
//!     after it — eddy mixing feeds hillslope wash laterally into the
//!     thalweg and burial beats winnowing. REAL interaction, not test
//!     scaffolding: eddy transport changes the channel supply balance.
//!     Open: is the eddy shear coefficient too strong, or is winnowing
//!     too weak against honest lateral supply? Decide by physics, not
//!     by making the test pass.
//!   regime 3 (supply-limited source incision): bed change EXACTLY 0.000
//!     at source cells in every version, though hand-arithmetic expects
//!     ~0.5 m export in 600 ss. Unexplained — suspect the erosion branch
//!     under thin sheet-source conditions (τ, capacity, or advection
//!     coupling at flow-path heads). Isolate with a single-column probe.
//!   1. A steep TRANSPORT-LIMITED channel (at capacity, no net erosion) must
//!      still armor within a few storm-cycles' worth of shear (winnowing), and
//!      must shed its colmation — real mountain channels are stony, not sealed.
//!   2. A SLACK basin must never armor; fines settle there — colmation and
//!      alluvium grow. Mud belongs to still water.
//!   3. A SUPPLY-LIMITED reach (nothing upstream) incises, and its own armor
//!      must SLOW that incision over time (self-limiting degradation — armor's
//!      entire geomorphic job).
//! One composite domain: a 5% ramp draining into a closed flat basin.
use vivarium_world::sphere::Face;
use vivarium_world::water::{WaterParams, WaterSim};

fn main() {
    let nx = 192usize;
    let cell = 4.8f32;
    let mut bed = vec![0.0f32; nx * nx];
    for y in 0..nx {
        for x in 0..nx {
            let across = (x as f32 - nx as f32 / 2.0) * cell;
            // Ramp for the upper 2/3 (5%), flat basin floor below; channelized.
            let down = y as f32 * cell;
            let ramp_end = nx as f32 * cell * 0.66;
            let h = if down < ramp_end { 6000.0 - down * 0.05 } else { 6000.0 - ramp_end * 0.05 };
            bed[y * nx + x] = h + (across / 40.0).powi(2).min(30.0);
        }
    }
    let mut w = WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, cell, bed.clone(), 5.0);
    let p = WaterParams { precip: WaterParams::default().precip * 10.0, ..Default::default() };
    let cx = nx / 2;
    let probe = |w: &WaterSim, y: usize| {
        let i = y * nx + cx;
        (w.armor[i], w.colmation[i], w.sed_bed[i], w.bed[i])
    };
    let (y_top, y_mid, y_basin) = (20usize, nx / 2, nx - 24);
    let bed0_top = bed[y_top * nx + cx];
    let mut bed_top_early = 0.0f32;
    let mut t = 0.0f32;
    while t < 2400.0 {
        let dt = w.stable_dt(9.8);
        let pp = WaterParams { dt, ..p };
        for _ in 0..(8.0 / dt) as u32 {
            w.step(&pp);
            t += dt;
        }
        // Decoupled upstream supply (watershed proxy) — MODEST: v2 injected
        // a whole row-width of load, the model answered with textbook
        // over-supply behaviour (aggradation wave burying the pavement,
        // transport saturation killing all incision) and correctly failed
        // every verdict. Supply must stay a fraction of local capacity.
        // v4: feed the BASIN MARGIN only — v3 injected at the channel head,
        // directly upstream of both ramp test reaches (a supply-limited reach
        // with a sediment hose above it isn't one; the mid-ramp aggraded by
        // design). The ramp regimes use NATURAL supply.
        let y_feed = (nx as f64 * 0.66) as usize + 4;
        for x in nx / 2 - 4..nx / 2 + 4 {
            w.sediment[y_feed * nx + x] += 1.0e-5 * 8.0;
        }
        if bed_top_early == 0.0 && t >= 600.0 {
            bed_top_early = w.bed[y_top * nx + cx];
        }
        if (t / 300.0).fract() < 0.02 {
            let (am, cm, sb, _) = probe(&w, y_mid);
            let (ab, cb, sbb, _) = probe(&w, y_basin);
            println!("t={t:>5.0}  mid-ramp: armor {am:.2} seal {cm:.2} alluv {sb:.2}   basin: armor {ab:.2} seal {cb:.2} alluv {sbb:.2}");
        }
    }
    let (a1, c1, _s1, _) = probe(&w, y_mid);
    let (a2, c2, s2, _) = probe(&w, y_basin);
    let (a0, _, _, bt) = probe(&w, y_top);
    let early = bed0_top - bed_top_early; // 0–600 ss (pre-pavement window)
    let late = bed_top_early - bt; // 600–2400 ss (armored window, 3× as long)
    println!("\n== verdicts ==");
    println!("1. transport-limited mid-ramp armors, sheds seal:   armor {a1:.2} (want >0.5)  seal {c1:.2} (want <0.15)  [{}]", if a1 > 0.5 && c1 < 0.15 { "PASS" } else { "FAIL" });
    println!("2. slack basin stays soft, takes the mud:           armor {a2:.2} (want <0.15) seal {c2:.2} (want >0.4) alluv {s2:.2} m [{}]", if a2 < 0.15 && c2 > 0.4 { "PASS" } else { "FAIL" });
    println!("3. supply-limited top: armor slows incision:        early 0-600ss {early:.3} m vs late 600-2400ss {late:.3} m (want measurable early, late < 1.5x early over 3x the time; armor {a0:.2}) [{}]", if early > 0.005 && late < early * 1.5 && a0 > 0.3 { "PASS" } else { "FAIL" });
    // Intuition: armor along the channel, top -> basin.
    let bars: String = (8..nx - 8).step_by(4).map(|y| {
        let a = w.armor[y * nx + cx];
        let k = ((a * 8.0) as usize).min(8);
        [' ', '.', ':', '-', '=', '+', '*', '#', '@'][k]
    }).collect();
    println!("armor profile (downhill →): [{bars}]");
}
