//! FE(9) lake-sized **bed** re-measure under production sill inject
//! (`#form-same-level-halo-exchange` FE(9); hotlist residual after `sill1` wire).
//!
//! Arms on one footprint:
//! - **REF** — single field (no internal seams)
//! - **PLAIN** — independent tiles, BaseLevelSink, no exchange
//! - **SILL1** — production `carve_region_jacobi_exchange` (sill1+flux1 live)
//!
//! Reports depression / standing-water stats under a **NoFluxWall** reader on the
//! assembled window (same contract as depression paint on a coastless patch).
//!
//! Store-free. Knobs: `VIVARIUM_SEED`, `LEVEL`, `FACE`, `OI`, `OJ`, `SPAN`,
//! `TILE`, `EPOCHS`.
//!
//! Run: `cargo run --release -p vivarium-world --example sill_fe9_probe`

use vivarium_world::erosion::{
    carve_region_jacobi_exchange, EdgeContract, Fluvial, FluvialParams, HaloSchedule,
};
use vivarium_world::sphere::{CellId, Face};

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}
fn env_usize(k: &str, d: usize) -> usize {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

struct LakeStats {
    dep_cells: usize,
    deepest_m: f32,
    capacity_m3: f64,
    stand_cells: usize,
    stand_volume_m3: f64,
}

fn lake_stats(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize, h: &[f32]) -> LakeStats {
    let h_owned = h.to_vec();
    let mut fl = Fluvial::from_surface(seed, face, level, oi, oj, nx, |c| {
        let (_f, i, j, _) = c.to_face_ij();
        let li = i.saturating_sub(oi) as usize;
        let lj = j.saturating_sub(oj) as usize;
        if li < nx && lj < nx {
            h_owned[lj * nx + li] as f64
        } else {
            0.0
        }
    });
    fl.set_edge_contract(EdgeContract::NoFluxWall);
    let d = fl.drainage_surface();
    let cell_a = d.stats.median_cell_area_m2 as f64;
    let mut dep_cells = 0usize;
    let mut deepest = 0.0f32;
    let mut cap = 0.0f64;
    let mut stand_cells = 0usize;
    let mut stand_vol = 0.0f64;
    for i in 0..h.len() {
        let fd = d.fill_depth[i];
        if fd > 1.0 {
            dep_cells += 1;
            deepest = deepest.max(fd);
            cap += fd as f64 * cell_a;
        }
        let sw = d.standing_water[i];
        if sw > 0.5 {
            stand_cells += 1;
            stand_vol += sw as f64 * cell_a;
        }
    }
    LakeStats {
        dep_cells,
        deepest_m: deepest,
        capacity_m3: cap,
        stand_cells,
        stand_volume_m3: stand_vol,
    }
}

fn print_arm(name: &str, s: &LakeStats) {
    println!(
        "{name:<8}  dep_cells={:>6}  deepest={:>7.1} m  cap={:.3e} m3  stand_cells={:>6}  stand_vol={:.3e} m3",
        s.dep_cells, s.deepest_m, s.capacity_m3, s.stand_cells, s.stand_volume_m3
    );
}

fn main() {
    let seed = env_u64("VIVARIUM_SEED", 17_425_063_241_017_297_386);
    let level = env_usize("VIVARIUM_LEVEL", 13) as u8;
    let face = Face::from_index(env_usize("VIVARIUM_FACE", 1) as u8);
    let oi = env_usize("VIVARIUM_OI", 640) as i64;
    let oj = env_usize("VIVARIUM_OJ", 5376) as i64;
    let span = env_usize("VIVARIUM_SPAN", 128);
    let tile = env_usize("VIVARIUM_TILE", 64);
    let epochs = env_u64("VIVARIUM_EPOCHS", 40) as u32;
    assert!(span % tile == 0 && span >= tile);
    let per = span / tile;
    let params = FluvialParams {
        epochs: 1, // erode() one epoch at a time in loops below
        ..Default::default()
    };

    let prior = |i: i64, j: i64| {
        let cell = CellId::from_face_ij(face, i.max(0) as u32, j.max(0) as u32, level);
        vivarium_world::gen::initial_topography_m(seed, cell, level) as f32
    };
    let mk = |oi0: i64, oj0: i64, nx: usize| {
        let mut f = Fluvial::from_surface(
            seed,
            face,
            level,
            oi0.max(0) as u32,
            oj0.max(0) as u32,
            nx,
            |c| vivarium_world::gen::initial_topography_m(seed, c, level),
        );
        f.set_edge_contract(EdgeContract::BaseLevelSink);
        f
    };

    println!(
        "sill_fe9_probe  seed={seed} L{level} f{} o({oi},{oj}) span={span} tile={tile} epochs={epochs}",
        face.index()
    );

    // REF: one field
    let mut r = Fluvial::from_surface(
        seed,
        face,
        level,
        oi.max(0) as u32,
        oj.max(0) as u32,
        span,
        |c| vivarium_world::gen::initial_topography_m(seed, c, level),
    );
    r.set_edge_contract(EdgeContract::BaseLevelSink);
    r.erode(&FluvialParams {
        epochs,
        ..Default::default()
    });
    let ref_h = r.h.clone();
    print_arm(
        "REF",
        &lake_stats(seed, face, level, oi.max(0) as u32, oj.max(0) as u32, span, &ref_h),
    );

    // PLAIN: independent tiles assembled
    let mut plain = vec![0.0f32; span * span];
    for ti in 0..per {
        for tj in 0..per {
            let toi = oi + (ti * tile) as i64;
            let toj = oj + (tj * tile) as i64;
            let mut fl = mk(toi, toj, tile);
            fl.erode(&FluvialParams {
                epochs,
                ..Default::default()
            });
            for y in 0..tile {
                for x in 0..tile {
                    plain[(tj * tile + y) * span + (ti * tile + x)] = fl.h[y * tile + x];
                }
            }
        }
    }
    print_arm(
        "PLAIN",
        &lake_stats(seed, face, level, oi.max(0) as u32, oj.max(0) as u32, span, &plain),
    );

    // SILL1: production Jacobi
    let schedule = HaloSchedule {
        depth: 16,
        cadence: (epochs / 8).max(1).min(10),
        cone_rho: 0,
    };
    let tiles = carve_region_jacobi_exchange(
        oi,
        oj,
        tile,
        per,
        per,
        epochs,
        schedule,
        |oi0, oj0, nx| mk(oi0, oj0, nx),
        prior,
        |_, _| {},
    );
    let mut sill = vec![0.0f32; span * span];
    for t in &tiles {
        let ti = ((t.oi as i64 - oi) / tile as i64) as usize;
        let tj = ((t.oj as i64 - oj) / tile as i64) as usize;
        for y in 0..tile {
            for x in 0..tile {
                sill[(tj * tile + y) * span + (ti * tile + x)] = t.h[y * tile + x];
            }
        }
    }
    print_arm(
        "SILL1",
        &lake_stats(seed, face, level, oi.max(0) as u32, oj.max(0) as u32, span, &sill),
    );

    println!(
        "\n# Reading: SILL1 stand_* / dep_* toward REF vs PLAIN ⇒ sill inject is doing \
         lake-sized work on this footprint. All three agree ⇒ no straddling lakes here \
         or the residual is tile-edge grading / elsewhere."
    );
    let _ = params;
}
