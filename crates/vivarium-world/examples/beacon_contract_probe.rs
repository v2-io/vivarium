//! **The beacon footprint, carved under both boundary contracts** — the
//! experiment `#obs-tile-outlets-grade-away-the-basins` FE(4) and FE(6) have
//! never been run against.
//!
//! Those clauses measure the shipped beacon: sixteen $64^2$ tiles at L13, each
//! graded to its own perimeter, whose tile-local routing is 3.85× short of the
//! assembled patch's and whose network is set within ten epochs. Both are true.
//! Neither has ever been compared to *the same footprint carved without edge
//! sinks*, because until `EdgeContract` existed there was no way to ask for that
//! — `Fluvial::outlets` inferred the contract from geometry, and only a whole
//! cube face inferred its way out of the sink branch.
//!
//! `#obs-fill-writes-itself-into-the-bed` measured the L9 planet build and found
//! the grain barely matters there: a tile spans 1251 km against a basin that
//! wants 279 km, so the edges are never reached. The beacon is the opposite
//! regime — 78 km tiles, 313 km of footprint, 300 epochs — and it is the regime
//! the whole cross-tile repair is justified by. This probe asks whether that
//! justification survives being measured.
//!
//! Two carves of one footprint, one reader:
//!
//! - **TILED** — $4\times4$ tiles of $64^2$, `BaseLevelSink`. The shipped beacon.
//! - **WALL** — one $256^2$ field, `NoFluxWall`. Same prior, uplift, rain, epochs.
//!
//! The reader is a $256^2$ field under `NoFluxWall` in **both** arms, so the
//! reader's own perimeter is never a sink and cannot be the difference. (Note
//! that the earlier "assembled routing" measurement did not do this: a $256^2$
//! window at L13 is a partial tile, so its perimeter *was* a ring of sinks. That
//! does not disturb the 3.85×, which compares a 64-cell perimeter against a
//! 256-cell one either way, but it is worth knowing when reading the number.)
//!
//! Store-free and world-free, like its sibling: every input is a pure function
//! of seed and cell.
//!
//! Run: `cargo run --release --example beacon_contract_probe`
//! Knobs (all env, all printed): `VIVARIUM_SEED`, `VIVARIUM_LEVEL`,
//! `VIVARIUM_FACE`, `VIVARIUM_OI`, `VIVARIUM_OJ`, `VIVARIUM_SPAN`,
//! `VIVARIUM_TILE`, `VIVARIUM_EPOCHS`, `VIVARIUM_STRIDE`.

use vivarium_world::erosion::{DrainageSurface, EdgeContract, Fluvial, FluvialParams};
use vivarium_world::sphere::{CellId, Face};

const DEFAULT_SEED: u64 = 17_425_063_241_017_297_386;

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

/// Precipitation weight over a window, normalized by that window's own mean —
/// the shape `query::erosion_tile` uses, applied at whichever grain is carving.
fn precip_weight(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
    let mut w: Vec<f32> = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
            w.push(vivarium_world::climate::precip_jitter_factor(seed, cell) as f32);
        }
    }
    let mean = w.iter().map(|v| *v as f64).sum::<f64>() / w.len() as f64;
    if mean > 0.0 {
        for v in w.iter_mut() {
            *v /= mean as f32;
        }
    }
    w
}

fn blit(dst: &mut [f32], n: usize, di: usize, dj: usize, nx: usize, src: &[f32]) {
    for j in 0..nx {
        for i in 0..nx {
            dst[(dj + j) * n + (di + i)] = src[j * nx + i];
        }
    }
}

/// Largest MFD drainage on a subaerial cell — the whole-window max is a coastal
/// or ocean cell carrying everything that reached the sea.
fn land_trunk(d: &DrainageSurface, h: &[f32], sea: f32) -> f32 {
    let mut best = 0.0f32;
    for i in 0..h.len() {
        if h[i] > sea {
            best = best.max(d.mfd[i]);
        }
    }
    best
}

/// How far apart the two carves end, split by whether a cell is on a tile
/// perimeter ring or inside one. Returns `(ring, interior)` mean `|h_a - h_b|`.
///
/// This replaced an absolute "trench depth" (mean interior height minus mean ring
/// height, per arm), which is **confounded and was withdrawn rather than
/// published**: the uplift step skips `is_edge` cells under *every* contract, the
/// prior's own ring-versus-interior offset is already several metres by accident
/// of terrain, and a direct single-tile check contradicted the aggregate's sign.
/// A difference *between the arms* has none of those confounds — whatever the
/// terrain and the uplift pinning do, they do to both.
fn arm_gap(a: &[f32], b: &[f32], n: usize, tile: usize) -> (f64, f64) {
    let (mut ring, mut inner) = (0.0f64, 0.0f64);
    let (mut rc, mut ic) = (0usize, 0usize);
    for y in 0..n {
        for x in 0..n {
            let d = (a[y * n + x] - b[y * n + x]).abs() as f64;
            if x % tile == 0 || y % tile == 0 || x % tile == tile - 1 || y % tile == tile - 1 {
                ring += d;
                rc += 1;
            } else {
                inner += d;
                ic += 1;
            }
        }
    }
    (ring / rc.max(1) as f64, inner / ic.max(1) as f64)
}

/// Mean |dh| across `tile`-multiple boundaries vs elsewhere, subaerial links.
fn seam_step(h: &[f32], n: usize, tile: usize, sea: f32) -> f64 {
    let (mut s_seam, mut s_int) = (0.0f64, 0.0f64);
    let (mut c_seam, mut c_int) = (0usize, 0usize);
    for j in 0..n {
        for i in 0..n {
            let a = j * n + i;
            for (di, dj) in [(1usize, 0usize), (0, 1)] {
                let (bi, bj) = (i + di, j + dj);
                if bi >= n || bj >= n {
                    continue;
                }
                let b = bj * n + bi;
                if h[a] <= sea && h[b] <= sea {
                    continue;
                }
                let d = (h[a] - h[b]).abs() as f64;
                if (di == 1 && bi % tile == 0) || (dj == 1 && bj % tile == 0) {
                    s_seam += d;
                    c_seam += 1;
                } else {
                    s_int += d;
                    c_int += 1;
                }
            }
        }
    }
    let m = |s: f64, c: usize| if c == 0 { 0.0 } else { s / c as f64 };
    let (a, b) = (m(s_seam, c_seam), m(s_int, c_int));
    if b > 0.0 {
        a / b
    } else {
        0.0
    }
}

fn main() {
    let seed = env_u64("VIVARIUM_SEED", DEFAULT_SEED);
    let level = env_u64("VIVARIUM_LEVEL", 13) as u8;
    let face = Face::from_index(env_u64("VIVARIUM_FACE", 1) as u8);
    let (oi, oj) = (env_u64("VIVARIUM_OI", 640) as u32, env_u64("VIVARIUM_OJ", 5376) as u32);
    let span = env_u64("VIVARIUM_SPAN", 256) as usize;
    let tile = env_u64("VIVARIUM_TILE", 64) as usize;
    let epochs = env_u64("VIVARIUM_EPOCHS", 300) as u32;
    let stride = env_u64("VIVARIUM_STRIDE", 50) as u32;
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed) as f32;
    let cell_km = vivarium_world::sample::cell_size_m(level, vivarium_world::planet::Planet::EARTH.radius_m) / 1000.0;

    println!("== the beacon footprint, carved under both boundary contracts ==");
    println!(
        "seed {seed}  f{} L{level} origin ({oi},{oj})  {span}^2 window ({:.0} km)  tiles {tile}^2 ({:.0} km)  {epochs} epochs  sea {sea:.1} m",
        face.index(),
        span as f64 * cell_km,
        tile as f64 * cell_km
    );
    println!("store: none opened, none written. All inputs are pure functions of (seed, cell).\n");

    let w_win = precip_weight(seed, face, level, oi, oj, span);
    let up_win = vivarium_world::uplift::uplift_rate_tile(seed, face, level, oi, oj, span);
    let surf = |c: CellId| vivarium_world::gen::initial_topography_m(seed, c, level);

    // The reader: one window-sized field, NoFluxWall in every arm, so its own
    // perimeter is never a sink and cannot be what separates the arms.
    let read = |h: &[f32]| -> DrainageSurface {
        let mut f = Fluvial::from_surface(seed, face, level, oi, oj, span, |_| 0.0);
        f.set_edge_contract(EdgeContract::NoFluxWall);
        f.set_precip_weight(w_win.clone());
        f.h = h.to_vec();
        f.drainage_surface()
    };

    // ---- Arm TILED: the shipped beacon — 4x4 tiles, edge sinks, assembled. ----
    let per = span / tile;
    let mut tiled_chain: Vec<(u32, Vec<f32>)> = Vec::new();
    {
        let mut tiles: Vec<Fluvial> = Vec::new();
        for tj in 0..per {
            for ti in 0..per {
                let (ti_o, tj_o) = (oi + (ti * tile) as u32, oj + (tj * tile) as u32);
                let mut f = Fluvial::from_surface(seed, face, level, ti_o, tj_o, tile, surf);
                assert_eq!(f.edge_contract(), EdgeContract::BaseLevelSink, "a beacon tile is a partial tile");
                f.set_uplift_rate(vivarium_world::uplift::uplift_rate_tile(seed, face, level, ti_o, tj_o, tile));
                f.set_precip_weight(precip_weight(seed, face, level, ti_o, tj_o, tile));
                tiles.push(f);
            }
        }
        let mut done = 0u32;
        while done < epochs {
            let k = stride.min(epochs - done);
            for f in tiles.iter_mut() {
                f.erode(&FluvialParams { epochs: k, ..Default::default() });
            }
            done += k;
            let mut bed = vec![0.0f32; span * span];
            for (t, f) in tiles.iter().enumerate() {
                blit(&mut bed, span, (t % per) * tile, (t / per) * tile, tile, &f.h);
            }
            tiled_chain.push((done, bed));
        }
    }

    // ---- Arm WALL: one field over the whole footprint, no edge sinks. ----
    let mut wall_chain: Vec<(u32, Vec<f32>)> = Vec::new();
    {
        let mut f = Fluvial::from_surface(seed, face, level, oi, oj, span, surf);
        f.set_edge_contract(EdgeContract::NoFluxWall);
        f.set_uplift_rate(up_win.clone());
        f.set_precip_weight(w_win.clone());
        let mut done = 0u32;
        while done < epochs {
            let k = stride.min(epochs - done);
            f.erode(&FluvialParams { epochs: k, ..Default::default() });
            done += k;
            wall_chain.push((done, f.h.clone()));
        }
    }

    // ---- The comparison. ----
    println!("-- the settle history under each contract, read through one NoFluxWall reader --");
    println!(
        "{:>7} | {:>9} {:>11} {:>8} | {:>9} {:>11} {:>8} | {:>10} {:>10}",
        "epochs", "T dep>1m", "T dep cap", "T seam", "W dep>1m", "W dep cap", "W seam", "gap ring", "gap inner"
    );
    for (idx, (k, tbed)) in tiled_chain.iter().enumerate() {
        let wbed = &wall_chain[idx].1;
        let (dt, dw) = (read(tbed), read(wbed));
        let gap = arm_gap(tbed, wbed, span, tile);
        println!(
            "{k:>7} | {:>9} {:>11.3e} {:>8.3} | {:>9} {:>11.3e} {:>8.3} | {:>10.1} {:>10.1}",
            dt.stats.depression_cells,
            dt.stats.depression_volume_m3,
            seam_step(tbed, span, tile, sea),
            dw.stats.depression_cells,
            dw.stats.depression_volume_m3,
            seam_step(wbed, span, tile, sea),
            gap.0,
            gap.1,
        );
    }

    let (kt, tbed) = tiled_chain.last().expect("at least one rung");
    let wbed = &wall_chain.last().expect("at least one rung").1;
    let (dt, dw) = (read(tbed), read(wbed));
    let (tt, tw) = (land_trunk(&dt, tbed, sea), land_trunk(&dw, wbed, sea));
    println!(
        "\n   at {kt} epochs: WALL / TILED land trunk = {:.2}x, largest-basin share {:.3} -> {:.3}, basins-for-half {} -> {}",
        if tt > 0.0 { tw / tt } else { 0.0 },
        dt.stats.largest_basin_share,
        dw.stats.largest_basin_share,
        dt.stats.basins_for_half,
        dw.stats.basins_for_half
    );
    println!(
        "   depression cells {} of {} ({:.1}%) TILED vs {} WALL; subaerial {} of {}.",
        dt.stats.depression_cells,
        span * span,
        100.0 * dt.stats.depression_cells as f64 / (span * span) as f64,
        dw.stats.depression_cells,
        dw.stats.subaerial,
        span * span
    );
    if dw.stats.subaerial == span * span {
        println!(
            "   NOTE: the window has NO COAST — every cell is subaerial. Under NoFluxWall its only\n   outlet is the single lowest cell, so both arms read one basin and the trunk statistic is\n   degenerate here. The depression and trench columns are what separate the arms."
        );
    }

    let gap = arm_gap(tbed, wbed, span, tile);
    println!(
        "   the two carves end {:.0} m apart on the tile rings and {:.0} m apart inside them (mean |dh|).",
        gap.0, gap.1
    );

    println!("\n-- scope --");
    println!("   * WALL is not an unbounded domain. Its perimeter is a no-flux wall, so divides are");
    println!("     forced onto the window edge instead of base level being handed to it — a different");
    println!("     undeclared contract, and the honest comparison is 'two contracts', not 'with and");
    println!("     without'. A halo exchanging flux with real neighbours is neither of these.");
    println!("   * Both arms are carved from the same prior at the present anchor, with the same");
    println!("     uplift field and the same fated rain. TILED normalizes rain per 64-cell tile (the");
    println!("     shipped path); WALL normalizes over the window. The reader uses the window field.");
}
