//! **Does the erosion tile settle?** The question that has to be answered before
//! a convergence-$\varepsilon$ criterion can honestly replace `epochs = 40`.
//!
//! `ASSUMPTIONS.md` carries "erosion run length" as an **arbitrary** row whose
//! named cure is a convergence gate ( #form-time-indexed-stage-chains FE(4)).
//! Installing that gate without measuring first is exactly the mistake
//! #obs-water-fill-never-settles caught on the other kernel: there, the residual
//! *grows*, so any tolerance would have certified a forty-second transient as
//! converged. `#norm-probe-sensitivity` in its plainest form — so this probe
//! reports the **trajectory**, not a verdict, and a criterion is only warranted
//! if that trajectory falls.
//!
//! What can falsify what:
//!   - a residual that does **not** fall toward zero refutes the premise that a
//!     tolerance is meaningful for erosion, and blocks the gate exactly as water
//!     is blocked;
//!   - a residual that falls, and whose knee sits well below the authored 40
//!     epochs, means the fixed count is *over*-running most tiles and a criterion
//!     is cheaper as well as honest;
//!   - a knee well *above* 40 means the authored count has been silently
//!     under-converging every tile in every world built so far.
//!
//! Several footprints are measured, not one, because "does it converge" is a
//! question about the population of tiles a build actually sweeps — an ocean tile
//! and a mountain tile have no reason to share a knee, and a criterion has to
//! serve both.
//!
//! Run: `cargo run --release -p vivarium-world --example erosion_settle_probe`

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::gen;
use vivarium_world::planet::Planet;
use vivarium_world::query::World;
use vivarium_world::sample::cell_size_m;
use vivarium_world::sphere::{CellId, Face};
use vivarium_world::store::Store;

const LEVEL: u8 = 9;
const NX: usize = 64;
const MAX_EPOCHS: u32 = 400;
const SEED: u64 = 0xF1D2_42B2_1D8D_89EA;

fn main() {
    let cell_km = cell_size_m(LEVEL, Planet::EARTH.radius_m) / 1000.0;
    println!("== erosion residual trajectory (L{LEVEL}, {NX}x{NX} tiles, ~{cell_km:.0} km/cell, seed {SEED:#x}) ==");
    println!("mean |dh| per epoch, m. A settle falls toward zero; the authored count is 40.\n");

    // Distinct footprints on distinct faces: whatever the terrain there happens
    // to be, not a curated set. The point is spread, not representativeness.
    let footprints: [(Face, u32, u32); 4] = [
        (Face::from_index(0), 0, 0),
        (Face::from_index(2), 64, 64),
        (Face::from_index(3), 128, 0),
        (Face::from_index(5), 64, 128),
    ];

    let marks = [1u32, 5, 10, 20, 40, 80, 160, 320, 400];
    print!("{:>22}", "footprint");
    for m in marks {
        print!("{m:>10}");
    }
    println!();

    // The probe must drive the SAME kernel configuration the builder does, or it
    // measures a kernel nobody runs. `query::World::erosion_tile` supplies two
    // fields the bare constructor leaves at zero — the uplift rate erosion carves
    // AGAINST and the precipitation weight that scales discharge — and without the
    // uplift in particular a tile has no driver and simply planes to nothing.
    let dir = std::env::temp_dir().join(format!("vivarium-erosion-probe-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let store = Store::open(&dir).expect("probe store");
    let world = World::new(&store, SEED);

    let mut knees = Vec::new();
    for (face, oi, oj) in footprints {
        let (initial_topo, _) = world.initial_topography(face, LEVEL, oi, oj, NX);
        let (uplift, _) = world.uplift_tile(face, LEVEL, oi, oj, NX);
        let (precip, _) = world.climate_tile(face, LEVEL, oi, oj, NX);
        let mean = precip.iter().sum::<f32>() / precip.len().max(1) as f32;
        let precip_weight: Vec<f32> =
            if mean > 0.0 { precip.iter().map(|p| p / mean).collect() } else { vec![1.0; precip.len()] };
        let surf = |cell: CellId| -> f64 {
            let (cf, ci, cj, _) = cell.to_face_ij();
            if cf.index() == face.index() && ci >= oi && cj >= oj {
                let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                if di < NX && dj < NX {
                    return initial_topo[dj * NX + di] as f64;
                }
            }
            gen::initial_topography_m(SEED, cell, LEVEL)
        };
        let p = FluvialParams { epochs: 1, ..Default::default() };
        let mean_uplift = uplift.iter().map(|&u| u as f64).sum::<f64>() / uplift.len() as f64;

        // TWO configurations, because one of them alone cannot tell the two
        // stories apart. Driven (the builder's own setup) shows the total
        // per-epoch change; UNDRIVEN (uplift zeroed) shows what the fluvial
        // kernel does on its own. If the driven residual equals the uplift rate
        // and the undriven residual is zero, the tile is not eroding at all — it
        // is only rising. That pair is the discriminator; either alone is a story.
        let run = |with_uplift: bool| -> Vec<f32> {
            let mut f = Fluvial::from_surface(SEED, face, LEVEL, oi, oj, NX, surf);
            if with_uplift {
                f.set_uplift_rate(uplift.clone());
            }
            f.set_precip_weight(precip_weight.clone());
            let mut trace = Vec::with_capacity(MAX_EPOCHS as usize);
            for _ in 0..MAX_EPOCHS {
                // One epoch at a time: `erode` only records the residual of its
                // LAST epoch, so stepping singly is what turns it into a trajectory.
                f.erode(&p);
                trace.push(f.last_delta_m);
            }
            trace
        };
        let trace = run(true);
        let undriven = run(false);

        // What terrain is it inert ON? Without this the finding is "three tiles do
        // nothing, cause unknown"; with it the reader can see whether inertness
        // tracks submergence or flatness, which are different defects.
        let lo = initial_topo.iter().cloned().fold(f32::INFINITY, f32::min);
        let hi = initial_topo.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let land = initial_topo.iter().filter(|&&h| h as f64 > gen::SEA_LEVEL_M).count();
        println!(
            "{:>22}   elevation {lo:.0}..{hi:.0} m, relief {:.0} m, land cells {land}/{}",
            "terrain",
            hi - lo,
            initial_topo.len()
        );

        print!("f{} ({oi:>3},{oj:>3}) drv{:>3}", face.index(), "");
        for m in marks {
            print!("{:>10.3e}", trace[(m - 1) as usize]);
        }
        println!();
        print!("{:>22}", "undriven");
        for m in marks {
            print!("{:>10.3e}", undriven[(m - 1) as usize]);
        }
        println!("   (uplift zeroed; mean uplift {mean_uplift:.3} m/epoch)");

        // The knee, defined so it can be wrong: the first epoch after which the
        // residual stays within 10% of its final value for the rest of the run.
        // Not a fitted timescale — a legible, falsifiable index.
        let final_v = trace[trace.len() - 1];
        let band = final_v * 0.1;
        let knee = (0..trace.len())
            .find(|&i| trace[i..].iter().all(|v| (v - final_v).abs() <= band))
            .map(|i| i + 1);
        knees.push((face.index(), oi, oj, knee, trace[0], final_v, mean_uplift, undriven[MAX_EPOCHS as usize - 1]));
    }

    println!("\n== verdict per footprint ==");
    for (f, oi, oj, knee, first, last, uplift, undriven_last) in &knees {
        let inert = *undriven_last == 0.0;
        let driver_bound = (last - *uplift as f32).abs() <= 0.05 * *uplift as f32;
        let verdict = if inert {
            "INERT — undriven residual is exactly 0: no fluvial work at all; the driven residual IS the uplift".to_string()
        } else if driver_bound {
            "driven steady state — residual pinned near the uplift rate, not falling to zero".to_string()
        } else {
            match knee {
                Some(k) => format!("falls; knee {k}"),
                None => "falls, no knee within the run".to_string(),
            }
        };
        println!("  f{f} ({oi},{oj}): epoch1 {first:.3e} -> epoch{MAX_EPOCHS} {last:.3e}, uplift {uplift:.3}   {verdict}");
    }
    println!(
        "\nWhat a near-stationarity gate would do here: NOTHING, or the wrong thing. Sustained uplift\n\
         means the system has no zero-residual equilibrium to detect — it approaches a DRIVEN steady\n\
         state in which erosion balances uplift and mean |dh| per epoch stays near the uplift rate\n\
         forever. A tolerance on |dh| either never fires or fires on a tile that is merely inert.\n\
         Same shape as #obs-water-fill-never-settles, different cause: there the step was pinned;\n\
         here the residual is dominated by the driver. The criterion erosion actually needs is a\n\
         statement about the EROSION-UPLIFT BALANCE, not about |dh| going to zero."
    );
}
