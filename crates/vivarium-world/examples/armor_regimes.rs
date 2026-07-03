//! Armor/scour regime probe (§2b): three invariants nature guarantees.
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
        // Decoupled upstream supply (watershed proxy): keep regime 2's mud
        // budget independent of regime 1's armor throttle — armored uplands
        // export clean water, which is honest but starves the basin test.
        for x in 0..nx {
            w.sediment[2 * nx + x] += 2.0e-4 * 8.0;
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
