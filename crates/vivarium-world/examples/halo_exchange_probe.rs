//! **Does a halo repair the tile seam, and does exchanging with real neighbours
//! buy anything over simply overlapping?**
//!
//! `halo_band_probe` measures the reach of one tile's boundary. This probe
//! measures the repair: one footprint carved as a block of tiles, against the
//! same footprint carved as one field, under three tile disciplines.
//!
//! Every arm has the **same outer perimeter** — the block's outer boundary is
//! the reference's boundary, under the same contract — so nothing here is a
//! comparison of boundary contracts. What varies is only what happens at the
//! *internal* seams, which is the defect
//! `#obs-tile-outlets-grade-away-the-basins` measures and the thing a halo is
//! for.
//!
//! ```text
//!   REF        one span^2 field. No internal seam exists.
//!   PLAIN      (span/n)^2 tiles of n^2, carved independently. The shipped beacon.
//!   OVERLAP d  each tile carved on an (n+2d)^2 window seeded from the PRIOR;
//!              the halo is computed and discarded, only the n^2 interior is kept.
//!              No tile ever reads another tile. **The key stays a pure function
//!              of coordinates plus one `halo` field** — there is no neighbour
//!              dependency to fold in, and therefore no dependency cone at all.
//!   EXCHANGE d,s  same windows, but every `s` epochs each tile's halo is
//!              overwritten from the *assembled interior of the previous chunk*
//!              (Jacobi / additive Schwarz: all tiles read the same frozen
//!              snapshot, so the result does not depend on tile order and the
//!              build stays order-independent, `#form-depend-by-key-never-latest`).
//!              This one DOES make a tile depend on its neighbours, and the
//!              dependency is what its key would have to carry.
//! ```
//!
//! The comparison OVERLAP-vs-EXCHANGE at equal depth is the design's hinge: the
//! two cost nearly the same to compute and differ enormously in what they cost to
//! *key*. If overlap alone captures most of the repair, the cheap design is the
//! honest one. If exchange is decisively better, the dependency cone has to be
//! paid for, and then its bounded footprint is the thing to design.
//!
//! Store-free and world-free; rain normalized by one common constant across arms.
//!
//! Run: `cargo run --release --example halo_exchange_probe`
//! Knobs: `VIVARIUM_SEED`, `VIVARIUM_LEVEL`, `VIVARIUM_FACE`, `VIVARIUM_OI`,
//! `VIVARIUM_OJ`, `VIVARIUM_SPAN`, `VIVARIUM_TILE`, `VIVARIUM_EPOCHS`,
//! `VIVARIUM_HALOS`, `VIVARIUM_CADENCES`.

use vivarium_world::erosion::{DrainageSurface, EdgeContract, Fluvial, FluvialParams};
use vivarium_world::sphere::{CellId, Face};

const DEFAULT_SEED: u64 = 17_425_063_241_017_297_386;

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

fn env_list(k: &str, d: &[usize]) -> Vec<usize> {
    std::env::var(k)
        .ok()
        .map(|v| v.split(',').filter_map(|t| t.trim().parse().ok()).collect())
        .unwrap_or_else(|| d.to_vec())
}

fn precip_raw(seed: u64, face: Face, level: u8, oi: i64, oj: i64, nx: usize) -> Vec<f32> {
    let mut w = Vec::with_capacity(nx * nx);
    for j in 0..nx as i64 {
        for i in 0..nx as i64 {
            let cell = CellId::from_face_ij(face, (oi + i).max(0) as u32, (oj + j).max(0) as u32, level);
            w.push(vivarium_world::climate::precip_jitter_factor(seed, cell) as f32);
        }
    }
    w
}

fn compare(a: &[f32], b: &[f32]) -> (f64, f64) {
    let mut sum = 0.0f64;
    let mut max = 0.0f64;
    for (x, y) in a.iter().zip(b.iter()) {
        let d = (x - y).abs() as f64;
        sum += d;
        if d > max {
            max = d;
        }
    }
    (sum / a.len().max(1) as f64, max)
}

/// Mean |dh| across `tile`-multiple boundaries over mean |dh| elsewhere, subaerial
/// links — the tiling's clean signature (`#obs-lakes-are-routed-over-not-carved-away` FE(8)).
/// A repair that works drives this toward the reference's own value.
fn seam_step(h: &[f32], n: usize, tile: usize, sea: f32) -> f64 {
    let (mut ss, mut si) = (0.0f64, 0.0f64);
    let (mut cs, mut ci) = (0usize, 0usize);
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
                    ss += d;
                    cs += 1;
                } else {
                    si += d;
                    ci += 1;
                }
            }
        }
    }
    let m = |s: f64, c: usize| if c == 0 { 0.0 } else { s / c as f64 };
    let (a, b) = (m(ss, cs), m(si, ci));
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
    let (oi, oj) = (env_u64("VIVARIUM_OI", 640) as i64, env_u64("VIVARIUM_OJ", 5376) as i64);
    let span = env_u64("VIVARIUM_SPAN", 256) as usize;
    let tile = env_u64("VIVARIUM_TILE", 64) as usize;
    let epochs = env_u64("VIVARIUM_EPOCHS", 300) as u32;
    let halos = env_list("VIVARIUM_HALOS", &[4, 8, 16, 32]);
    let cadences = env_list("VIVARIUM_CADENCES", &[10, 50]);
    let per = span / tile;
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed) as f32;
    let cell_km = vivarium_world::sample::cell_size_m(level, vivarium_world::planet::Planet::EARTH.radius_m) / 1000.0;

    println!("== does a halo repair the tile seam, and does exchange beat overlap? ==");
    println!(
        "seed {seed}  f{} L{level}  span {span}^2 ({:.0} km)  tiles {per}x{per} of {tile}^2 ({:.0} km)  {epochs} epochs",
        face.index(),
        span as f64 * cell_km,
        tile as f64 * cell_km
    );
    println!("halos {halos:?}  cadences {cadences:?}  sea {sea:.1} m");
    println!("store: none opened, none written. All inputs are pure functions of (seed, cell).\n");

    let prior = |i: i64, j: i64| -> f32 {
        let cell = CellId::from_face_ij(face, i.max(0) as u32, j.max(0) as u32, level);
        vivarium_world::gen::initial_topography_m(seed, cell, level) as f32
    };
    let rain_mean = {
        let w = precip_raw(seed, face, level, oi, oj, span);
        w.iter().map(|v| *v as f64).sum::<f64>() / w.len() as f64
    };
    let mk = |o_i: i64, o_j: i64, nx: usize| -> Fluvial {
        let mut f = Fluvial::from_surface(seed, face, level, o_i.max(0) as u32, o_j.max(0) as u32, nx, |c| {
            vivarium_world::gen::initial_topography_m(seed, c, level)
        });
        f.set_uplift_rate(vivarium_world::uplift::uplift_rate_tile(
            seed,
            face,
            level,
            o_i.max(0) as u32,
            o_j.max(0) as u32,
            nx,
        ));
        f.set_precip_weight(precip_raw(seed, face, level, o_i, o_j, nx).iter().map(|v| v / rain_mean as f32).collect());
        f
    };

    // ---- REF: one field, no internal seam. ----
    let reference = {
        let mut f = mk(oi, oj, span);
        f.erode(&FluvialParams { epochs, ..Default::default() });
        f.h
    };

    // Carve a block of tiles with halo `d`, exchanging every `cadence` epochs
    // (`cadence == 0` = never: one-shot overlap, no neighbour ever read).
    // `order` permutes which tile is stepped first — the handle the
    // order-independence check turns. `gauss_seidel` writes each tile's interior
    // into the shared snapshot *as it is produced*, so later tiles in the sweep
    // read newer neighbours than earlier ones; the default (Jacobi) freezes the
    // snapshot first and lets every tile read the same one.
    // Returns the assembled `span^2` interior field.
    let block = |d: usize, cadence: u32, ep: u32, order: &[usize], gauss_seidel: bool| -> Vec<f32> {
        let nx = tile + 2 * d;
        let mut tiles: Vec<Fluvial> = Vec::new();
        for tj in 0..per {
            for ti in 0..per {
                tiles.push(mk(oi + (ti * tile) as i64 - d as i64, oj + (tj * tile) as i64 - d as i64, nx));
            }
        }
        let chunk = if cadence == 0 { ep } else { cadence };
        let mut done = 0u32;
        let mut assembled = vec![0.0f32; span * span];
        let publish = |assembled: &mut Vec<f32>, t: usize, f: &Fluvial| {
            let (ti, tj) = (t % per, t / per);
            for j in 0..tile {
                for i in 0..tile {
                    assembled[(tj * tile + j) * span + (ti * tile + i)] = f.h[(d + j) * nx + (d + i)];
                }
            }
        };
        let refill = |assembled: &Vec<f32>, t: usize, f: &mut Fluvial| {
            let (ti, tj) = (t % per, t / per);
            let (bi, bj) = ((ti * tile) as i64 - d as i64, (tj * tile) as i64 - d as i64);
            for j in 0..nx {
                for i in 0..nx {
                    if i >= d && i < d + tile && j >= d && j < d + tile {
                        continue; // the tile owns its interior
                    }
                    let (gx, gy) = (bi + i as i64, bj + j as i64);
                    f.h[j * nx + i] = if gx >= 0 && gy >= 0 && (gx as usize) < span && (gy as usize) < span {
                        assembled[gy as usize * span + gx as usize]
                    } else {
                        prior(oi + gx, oj + gy)
                    };
                }
            }
        };
        while done < ep {
            let k = chunk.min(ep - done);
            for &t in order {
                tiles[t].erode(&FluvialParams { epochs: k, ..Default::default() });
                if gauss_seidel {
                    // Publish immediately, and refill this tile's halo from a
                    // snapshot that already contains earlier tiles of this sweep.
                    publish(&mut assembled, t, &tiles[t]);
                    if cadence > 0 {
                        let snap = assembled.clone();
                        refill(&snap, t, &mut tiles[t]);
                    }
                }
            }
            done += k;
            if !gauss_seidel {
                // Assemble interiors — the frozen snapshot every tile will read.
                for (t, f) in tiles.iter().enumerate() {
                    publish(&mut assembled, t, f);
                }
                // Jacobi exchange: every tile's halo takes the snapshot's value
                // where the snapshot covers it, the prior where it does not. All
                // tiles read the SAME snapshot, so tile order cannot affect it.
                if cadence > 0 && done < ep {
                    for (t, f) in tiles.iter_mut().enumerate() {
                        refill(&assembled, t, f);
                    }
                }
            }
        }
        // Gauss-Seidel published as it went; make the final assembly uniform.
        for (t, f) in tiles.iter().enumerate() {
            publish(&mut assembled, t, f);
        }
        assembled
    };
    let fwd: Vec<usize> = (0..per * per).collect();
    let rev: Vec<usize> = (0..per * per).rev().collect();

    let read = |h: &[f32]| -> DrainageSurface {
        let mut f = Fluvial::from_surface(seed, face, level, oi as u32, oj as u32, span, |_| 0.0);
        f.set_edge_contract(EdgeContract::NoFluxWall);
        f.h = h.to_vec();
        f.drainage_surface()
    };

    let ref_seam = seam_step(&reference, span, tile, sea);
    let ref_dep = read(&reference).stats.depression_cells;
    let ref_mean = reference.iter().map(|v| *v as f64).sum::<f64>() / reference.len() as f64;
    println!("REF (one {span}^2 field, no internal seam): seam step {ref_seam:.3}, depressions {ref_dep}, mean h {ref_mean:.1} m");
    println!("   -- the seam step is a ratio at the SAME 64-cell lines the tiled arms use, so the");
    println!("      reference's own value is the null a repair drives toward, not zero.\n");

    println!(
        "{:>18} | {:>9} {:>8} | {:>9} {:>9} {:>9} | {:>6}",
        "arm", "mean |dh|", "max", "seam step", "dep>1m", "mean h", "cost"
    );
    let cost = |d: usize| ((tile + 2 * d) as f64 / tile as f64).powi(2);
    let report = |name: String, h: &[f32], c: f64| {
        let (m, mx) = compare(h, &reference);
        let mean = h.iter().map(|v| *v as f64).sum::<f64>() / h.len() as f64;
        println!(
            "{name:>18} | {m:>9.2} {mx:>8.0} | {:>9.3} {:>9} {mean:>9.1} | {c:>5.2}x",
            seam_step(h, span, tile, sea),
            read(h).stats.depression_cells,
        );
    };

    let plain = block(0, 0, epochs, &fwd, false);
    report("PLAIN (shipped)".into(), &plain, 1.0);
    for &d in &halos {
        report(format!("OVERLAP d={d}"), &block(d, 0, epochs, &fwd, false), cost(d));
    }
    for &s in &cadences {
        for &d in &halos {
            report(format!("EXCHANGE d={d} s={s}"), &block(d, s as u32, epochs, &fwd, false), cost(d));
        }
    }

    // ---- The property the design rests on, stated so it can fail. ----
    println!("\n-- order independence: the same block built in two tile orders --");
    println!("   A halo makes a tile depend on its neighbours, and a dependency that resolves");
    println!("   differently under a different build order makes the world a function of the walking");
    println!("   route (`#form-depend-by-key-never-latest` FE(1)). Jacobi exchange is the form that");
    println!("   keeps that invariant: every tile reads one frozen snapshot. Gauss-Seidel — publishing");
    println!("   each tile as it is produced — is the form that breaks it, and is run here as the");
    println!("   sensitivity control, because a bit-identity assertion that nothing can fail is not");
    println!("   evidence (`#norm-probe-sensitivity`).");
    let (od, os, oe) = (8usize, 10u32, 50u32);
    let j_fwd = block(od, os, oe, &fwd, false);
    let j_rev = block(od, os, oe, &rev, false);
    let g_fwd = block(od, os, oe, &fwd, true);
    let g_rev = block(od, os, oe, &rev, true);
    let bits = |a: &[f32], b: &[f32]| a.iter().zip(b.iter()).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
    let (jm, jx) = compare(&j_fwd, &j_rev);
    let (gm, gx) = compare(&g_fwd, &g_rev);
    println!("   (d={od}, cadence {os}, {oe} epochs, {} tiles, forward vs reverse sweep)", per * per);
    println!("   JACOBI        : {:>7} of {} cells differ   mean {jm:.3e} m   max {jx:.3e} m", bits(&j_fwd, &j_rev), span * span);
    println!("   GAUSS-SEIDEL  : {:>7} of {} cells differ   mean {gm:.3} m       max {gx:.1} m", bits(&g_fwd, &g_rev), span * span);
    assert_eq!(bits(&j_fwd, &j_rev), 0, "Jacobi exchange must be bit-identical under any tile order");
    assert!(bits(&g_fwd, &g_rev) > 0, "the Gauss-Seidel control must actually be order-dependent, or the check above is vacuous");
    println!("   => Jacobi holds the invariant; the control shows the check is not vacuous.");

    println!("\n-- how to read this --");
    println!("   * `mean |dh|` is against REF and is a POINTWISE statistic. `halo_band_probe` measures");
    println!("     a chaos floor for it: a 1 mm perturbation 96 cells away moves a core by ~13 m mean");
    println!("     over 300 epochs. Differences below that floor are not evidence about the method.");
    println!("   * `seam step` and `dep>1m` are structural and have no such floor issue at this size;");
    println!("     they are what the repair is FOR, and REF's own values are the target.");
    println!("   * `cost` is compute per unit of kept area, ((n+2d)/n)^2 — the halo is computed and");
    println!("     thrown away, so it is paid every tile, every stage.");
    println!("   * OVERLAP reads no neighbour: its key is (coords, halo). EXCHANGE reads neighbours:");
    println!("     its key must carry that dependency, and the difference between the two rows at equal");
    println!("     d is what that key complexity is worth.");
}
