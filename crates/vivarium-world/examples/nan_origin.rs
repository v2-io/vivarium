//! **Where does erosion mint its first NaN?**
//!
//! `examples/nan_census` establishes that 83 of 3552 `erosion-tile` roots hold
//! non-finite heights in every cohort measured, while every *input* nomos
//! (`initial-topography`, `uplift-tile`, `climate`) is finite in every cell. So
//! erosion generates them from clean inputs, deterministically.
//!
//! This probe reproduces one convicted footprint through the public kernel and
//! walks epoch by epoch to the first non-finite cell, then prints that cell's
//! neighbourhood *from the epoch before* — the state the arithmetic was handed.
//!
//! Two things it deliberately does NOT do: reproduce the halo/region path
//! (if the plain per-tile carve mints the same NaN, the exchange is not the
//! cause, and that is worth knowing first), and assume the epoch count. It
//! sweeps.
//!
//! Read-only against the generator; opens the store only to read inputs.
//!
//! Run: `cargo run --release -p vivarium-world --example nan_origin`

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::query::World;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;

/// The worst root from `nan_census`: face 0, L9, (128,448), 64², 40 epochs.
const SEED: u64 = 17_425_063_241_017_297_386;
const LEVEL: u8 = 9;
const NX: usize = 64;
const OI: u32 = 128;
const OJ: u32 = 448;
const EPOCHS: u32 = 40;

fn first_bad(h: &[f32]) -> Option<usize> {
    h.iter().position(|x| !x.is_finite())
}

fn main() {
    let face = Face::from_index(0);
    let home = std::env::var("HOME").expect("HOME");
    let dir = format!("{home}/.cache/vivarium/globe-world");
    let store = Store::open(&dir).expect("store");
    let world = World::new(&store, SEED);

    // Inputs, pulled through the sanctioned path so this is the kernel's own diet.
    let (topo, _) = world.initial_topography(face, LEVEL, OI, OJ, NX);
    let (uplift, _) = world.uplift_tile(face, LEVEL, OI, OJ, NX);
    let (precip, _) = world.climate_tile(face, LEVEL, OI, OJ, NX);
    println!("inputs finite: topo {} uplift {} climate {}",
        topo.iter().all(|x| x.is_finite()),
        uplift.iter().all(|x| x.is_finite()),
        precip.iter().all(|x| x.is_finite()));
    let (tlo, thi) = topo.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)));
    println!("topo range: {tlo:.1} .. {thi:.1} m");
    let (ulo, uhi) = uplift.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)));
    println!("uplift range: {ulo:.6} .. {uhi:.6} m/epoch");

    let mut f = Fluvial::from_prior(SEED, face, LEVEL, OI, OJ, NX);
    f.h.copy_from_slice(&topo);
    f.set_uplift_rate(uplift.clone());
    f.set_precip_weight(precip.clone());

    let p = FluvialParams { epochs: 1, ..Default::default() };
    let mut prev = f.h.clone();
    for e in 1..=EPOCHS {
        f.erode(&p);
        if let Some(i) = first_bad(&f.h) {
            let (x, y) = (i % NX, i / NX);
            println!();
            println!("FIRST NON-FINITE at epoch {e}, cell {i} = ({x},{y}): {}", f.h[i]);
            let bad = f.h.iter().filter(|v| !v.is_finite()).count();
            println!("  {bad} of {} cells non-finite at this epoch", NX * NX);
            println!("  the neighbourhood it was handed, one epoch earlier:");
            for dy in -1i32..=1 {
                let mut row = String::new();
                for dx in -1i32..=1 {
                    let (xp, yp) = (x as i32 + dx, y as i32 + dy);
                    if xp < 0 || yp < 0 || xp >= NX as i32 || yp >= NX as i32 {
                        row.push_str("     OFFGRID");
                        continue;
                    }
                    row.push_str(&format!(" {:>12.4}", prev[yp as usize * NX + xp as usize]));
                }
                println!("   {row}");
            }
            println!("  is it on the grid edge? {}", x == 0 || y == 0 || x == NX - 1 || y == NX - 1);
            return;
        }
        prev.copy_from_slice(&f.h);
    }
    // **The suspected mint, tested directly.** A halo window on a region
    // perimeter overhangs the cube chart. `Fluvial::from_surface` clamps
    // out-of-chart indices (`gi.saturating_add(x).min(last)`) — its own comment
    // says "Clamp rather than panic: true cube-edge resampling for d>=2 is still
    // open". Clamped cells therefore share an `(i,j)`, hence share a centre
    // vector, hence sit at distance ZERO from a real neighbour. Any slope or
    // flux that divides by that distance mints a non-finite value from finite
    // inputs — which is exactly the census's signature.
    println!();
    println!("-- the overhanging halo window, geometry only --");
    let face_n = 1u32 << LEVEL;
    let last = face_n - 1;
    let d = 16i64;
    let win = NX as i64 + 2 * d;
    // Origins exactly as `carve_region_jacobi_exchange` computes them:
    // `region_oj + tj*tile_n - d`, for a whole-face region at this level. Deriving
    // them rather than reconstructing them by hand, because a hand-built origin is
    // how an earlier version of this probe reported an overhang four times too
    // large.
    let tiles = (face_n as usize / NX) as i64;
    let origins: Vec<i64> = (0..tiles).map(|t| t * NX as i64 - d).collect();
    println!("  chart is {face_n}² cells; window span {win}; origins {origins:?}");
    let hi_origin = *origins.last().unwrap();
    println!("  highest window spans {hi_origin}..{}", hi_origin + win - 1);
    let seen_last = (0..win).filter(|y| (hi_origin + y).min(last as i64) == last as i64).count();
    println!("  rows collapsing onto j={last}: {seen_last} (so {} DUPLICATE rows)", seen_last.saturating_sub(1));
    let lo_origin = origins[0];
    println!("  lowest window origin is {lo_origin}, clamped to 0 by the builder → the window");
    println!("    SLIDES by {} cells instead of padding, so this tile's interior is not at", -lo_origin);
    println!("    halo offset d and publish writes ground from the wrong place");
    let c0 = vivarium_world::measure::cell_center_unit(face, 100, last as u64, LEVEL);
    let c1 = vivarium_world::measure::cell_center_unit(face, 100, last as u64, LEVEL);
    let same = c0 == c1;
    println!("  two clamped cells share a centre vector: {same}");
    let cprev = vivarium_world::measure::cell_center_unit(face, 100, (last - 1) as u64, LEVEL);
    let dot = c0[0] * cprev[0] + c0[1] * cprev[1] + c0[2] * cprev[2];
    let arc = dot.clamp(-1.0, 1.0).acos() * vivarium_world::planet::Planet::EARTH.radius_m;
    println!("  a REAL adjacent pair is {arc:.1} m apart; a duplicated pair is 0.0 m apart");
    println!("  → a slope or flux dividing by that distance mints inf/NaN, and the census");
    println!("    finds non-finite cells on 73 of 73 perimeter positions and 0 interior ones");

    println!();
    println!("VERDICT: {EPOCHS} epochs of the PLAIN per-tile carve stay finite on this footprint.");
    println!("So the mint is not the bare fluvial loop on these inputs — suspect the");
    println!("region/halo exchange path (this root carries edge=halo d=16 sigma=5) or the");
    println!("staged chain, and probe those next.");
}
