//! **What the tile edge costs, carved both ways** — the control for
//! `#obs-tile-outlets-grade-away-the-basins`.
//!
//! That observation measured the status quo and named its own missing control:
//! *"run the depression census over a whole-face erosion tile (coast-only
//! outlets). If closed depressions appear there, the tiling is the sole cause
//! and the number is clean; if they do not, some of the absence is the kernel's
//! fill-every-epoch composition and the diagnosis must split."* This probe runs
//! that control, and runs it as a **carve**, not an assembly.
//!
//! The distinction matters and is the whole design. Assembling stored tiles and
//! re-routing them (`discharge_probe` Part 4; the χ strand's whole-face run)
//! measures what the *reader* sees when the seams are dropped — the beds were
//! still carved against edge sinks. Carving a whole face as one field
//! (`oi=oj=0`, `nx=2^level`, so `Fluvial::outlets` takes its coast-only branch)
//! measures what the *world* would have been. Only the second answers "what does
//! capture-capable base level do."
//!
//! Three surfaces, one reader:
//!
//! - **PRIOR** — the uncarved initial topography. The baseline nobody had:
//!   how many closed depressions does the world *start* with?
//! - **TILED** — the status quo. Independent `tile_nx²` tiles with edge sinks,
//!   assembled after the fact. Bit-for-bit what the builder writes.
//! - **FACE** — the control. One field over the whole cube face, coast-only
//!   outlets, same seed / prior / uplift / epochs.
//!
//! All three are then measured by the *same* whole-face reader
//! (`Fluvial::drainage_surface` on a full-face region, so the reader's own
//! outlets are coast-only in every arm and cannot be the difference).
//!
//! Store-free and world-free: initial topography, uplift and precipitation are
//! pure functions of seed and cell (`gen::initial_topography_m`,
//! `uplift::uplift_rate_m_per_epoch`, `climate::precip_jitter_factor`), so this
//! probe opens no store, writes nothing, and is reproducible from its printed
//! header alone.
//!
//! Run: `cargo run --release --example base_level_probe`
//!      `VIVARIUM_LEVEL=8 VIVARIUM_EPOCHS=120 cargo run --release --example base_level_probe`
//!
//! Knobs (all env, all printed): `VIVARIUM_SEED`, `VIVARIUM_LEVEL`,
//! `VIVARIUM_EPOCHS`, `VIVARIUM_TILE`, `VIVARIUM_FACE`, `VIVARIUM_STRIDE`.

use std::time::Instant;
use vivarium_world::erosion::{DrainageSurface, ErodedRegion, Fluvial, FluvialParams};
use vivarium_world::sphere::{CellId, Face};

/// The live `globe-world` seed — so the numbers land beside the ones
/// `#obs-tile-outlets-grade-away-the-basins` already quotes.
const DEFAULT_SEED: u64 = 17_425_063_241_017_297_386;

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d)
}

/// Face-normalized precipitation weight over a whole face — the same fated
/// jitter the kernel consumes, divided by the *face* mean rather than a tile
/// mean. (The builder normalizes per tile; see the seam note in the output.)
fn face_precip_weight(seed: u64, face: Face, level: u8, n: usize) -> Vec<f32> {
    let mut w: Vec<f32> = Vec::with_capacity(n * n);
    for j in 0..n as u32 {
        for i in 0..n as u32 {
            let cell = CellId::from_face_ij(face, i, j, level);
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

/// A window of a face-sized field, row-major.
fn window(src: &[f32], n: usize, oi: usize, oj: usize, nx: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(nx * nx);
    for j in 0..nx {
        for i in 0..nx {
            out.push(src[(oj + j) * n + (oi + i)]);
        }
    }
    out
}

fn blit(dst: &mut [f32], n: usize, oi: usize, oj: usize, nx: usize, src: &[f32]) {
    for j in 0..nx {
        for i in 0..nx {
            dst[(oj + j) * n + (oi + i)] = src[j * nx + i];
        }
    }
}

/// Read a face-sized height field as one full-face region — coast-only outlets
/// in every arm, so the *reader* is never the difference between arms.
fn read_face(seed: u64, face: Face, level: u8, n: usize, h: &[f32], w: &[f32]) -> DrainageSurface {
    let region = ErodedRegion { face, level, oi: 0, oj: 0, nx: n, h: h.to_vec(), seed };
    let mut f = Fluvial::from_region(&region);
    f.set_precip_weight(w.to_vec());
    f.drainage_surface()
}

/// Mean |Δh| across links that cross a `tile`-multiple boundary, versus links
/// that do not — restricted to pairs where at least one end is subaerial. A
/// tiling that grades each tile to its own perimeter leaves a step here; a
/// whole-face carve has nothing to leave.
fn seam_step(h: &[f32], n: usize, tile: usize, sea: f32) -> (f64, f64, usize, usize) {
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
                // The link crosses a tile boundary iff the higher index is a
                // multiple of `tile` (cell tile-1 to cell tile).
                let crosses = (di == 1 && bi % tile == 0) || (dj == 1 && bj % tile == 0);
                if crosses {
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
    (m(s_seam, c_seam), m(s_int, c_int), c_seam, c_int)
}

/// 8-connected land components of a height field, as a label per cell
/// (`usize::MAX` = ocean) plus each component's cell count and bounding-box span.
///
/// This is the census the whole comparison turned out to need. A tile-edge base
/// level can only cap a basin that *wants* to be bigger than a tile, so the
/// question "how much land even lives in a landmass wider than `tile` cells"
/// bounds how much of the planet the defect can reach.
fn land_components(h: &[f32], n: usize, sea: f32) -> (Vec<usize>, Vec<(usize, usize)>) {
    let mut label = vec![usize::MAX; n * n];
    let mut comps: Vec<(usize, usize)> = Vec::new(); // (cells, bbox span)
    let mut stack: Vec<usize> = Vec::new();
    for start in 0..n * n {
        if h[start] <= sea || label[start] != usize::MAX {
            continue;
        }
        let id = comps.len();
        let (mut lo_i, mut hi_i, mut lo_j, mut hi_j) = (n, 0usize, n, 0usize);
        let mut cells = 0usize;
        label[start] = id;
        stack.push(start);
        while let Some(c) = stack.pop() {
            let (x, y) = (c % n, c / n);
            cells += 1;
            lo_i = lo_i.min(x);
            hi_i = hi_i.max(x);
            lo_j = lo_j.min(y);
            hi_j = hi_j.max(y);
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let (a, b) = (x as i32 + dx, y as i32 + dy);
                    if a < 0 || b < 0 || a >= n as i32 || b >= n as i32 {
                        continue;
                    }
                    let k = b as usize * n + a as usize;
                    if h[k] > sea && label[k] == usize::MAX {
                        label[k] = id;
                        stack.push(k);
                    }
                }
            }
        }
        comps.push((cells, (hi_i - lo_i + 1).max(hi_j - lo_j + 1)));
    }
    (label, comps)
}

/// Largest MFD drainage over *subaerial* cells only, optionally restricted to
/// one land component. The whole-face `max_mfd_m2` is an ocean cell — every
/// submarine cell accumulates the runoff that reached the sea — so it measures
/// the face's total discharge, not a river.
fn land_trunk(d: &DrainageSurface, h: &[f32], sea: f32, label: Option<(&[usize], usize)>) -> f32 {
    let mut best = 0.0f32;
    for i in 0..h.len() {
        if h[i] <= sea {
            continue;
        }
        if let Some((lab, id)) = label {
            if lab[i] != id {
                continue;
            }
        }
        best = best.max(d.mfd[i]);
    }
    best
}

/// The rows the three arms are compared on. Every one is a falsifier for some
/// clause of `#obs-tile-outlets-grade-away-the-basins`.
fn row(tag: &str, d: &DrainageSurface, h: &[f32], sea: f32) -> String {
    let s = &d.stats;
    let land = s.subaerial.max(1) as f32;
    let dep_frac = 100.0 * s.depression_cells as f32 / land;
    let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
    let mut sum = 0.0f64;
    let mut nland = 0usize;
    for &v in h {
        if v > sea {
            lo = lo.min(v);
            hi = hi.max(v);
            sum += v as f64;
            nland += 1;
        }
    }
    let mean = if nland > 0 { sum / nland as f64 } else { 0.0 };
    format!(
        "{tag:<7} {land:>8.0} {:>8} {dep_frac:>7.2} {:>9.0} {:>11.3e} {:>10.3} {:>8} {:>8.0} {:>9.0} {:>8.0}",
        s.depression_cells,
        d.stats.deepest_depression_m,
        s.max_mfd_m2 as f64,
        s.largest_basin_share,
        s.basins_for_half,
        mean,
        if hi.is_finite() { hi - lo } else { 0.0 },
        s.mean_out_degree * 1000.0,
    )
}

fn header() -> String {
    format!(
        "{:<7} {:>8} {:>8} {:>7} {:>9} {:>11} {:>10} {:>8} {:>8} {:>9} {:>8}",
        "arm", "land", "dep>1m", "dep%", "deepest", "trunk m2", "big-share", "half-n", "meanH", "relief", "deg*1e3"
    )
}

fn main() {
    let seed = env_u64("VIVARIUM_SEED", DEFAULT_SEED);
    let level = env_u64("VIVARIUM_LEVEL", 9) as u8;
    let epochs = env_u64("VIVARIUM_EPOCHS", 40) as u32;
    let tile = env_u64("VIVARIUM_TILE", 64) as usize;
    let stride = env_u64("VIVARIUM_STRIDE", 10) as u32;
    let n = 1usize << level;
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed) as f32;
    let params = FluvialParams { epochs, ..Default::default() };

    println!("== base-level control: tile-edge sinks vs whole-face coast-only carve ==");
    println!(
        "seed {seed}  level L{level} ({n}^2 per face)  epochs {epochs}  tile {tile}^2  stride {stride}  derived sea {sea:.1} m"
    );
    println!("store: none opened, none written. All inputs are pure functions of (seed, cell).\n");

    // ---- Part 0: which face to carve. The landiest, unless told otherwise. ----
    let mut land_frac = [0.0f64; 6];
    for f in 0..6usize {
        let face = Face::from_index(f as u8);
        let mut land = 0usize;
        // Sample every 4th cell — this only picks the face.
        let step = (n / 128).max(1);
        let mut tot = 0usize;
        for j in (0..n).step_by(step) {
            for i in (0..n).step_by(step) {
                let c = CellId::from_face_ij(face, i as u32, j as u32, level);
                if vivarium_world::gen::initial_topography_m(seed, c, level) as f32 > sea {
                    land += 1;
                }
                tot += 1;
            }
        }
        land_frac[f] = land as f64 / tot as f64;
    }
    let picked = std::env::var("VIVARIUM_FACE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_else(|| {
            (0..6).max_by(|a, b| land_frac[*a].total_cmp(&land_frac[*b])).unwrap()
        });
    let face = Face::from_index(picked as u8);
    println!("-- Part 0: land fraction of the uncarved prior, per face (sampled) --");
    for (f, frac) in land_frac.iter().enumerate() {
        println!("   f{f}  {:>6.1}% {}", 100.0 * frac, if f == picked { "  <- carved" } else { "" });
    }
    println!();

    let w_face = face_precip_weight(seed, face, level, n);
    let up_face = vivarium_world::uplift::uplift_rate_tile(seed, face, level, 0, 0, n);

    // ---- Part 1: PRIOR — what the world starts with, before any carve. ----
    let t0 = Instant::now();
    let mut prior = vec![0.0f32; n * n];
    for j in 0..n as u32 {
        for i in 0..n as u32 {
            let c = CellId::from_face_ij(face, i, j, level);
            prior[j as usize * n + i as usize] = vivarium_world::gen::initial_topography_m(seed, c, level) as f32;
        }
    }
    let d_prior = read_face(seed, face, level, n, &prior, &w_face);
    println!("-- Part 1: the uncarved prior (built in {:.1?}) --", t0.elapsed());

    // ---- Part 2: the carve sweep. ----
    // One geography, one epoch count, one prior — and the tile grain swept from
    // `tile` up to the whole face. `tile == n` takes `Fluvial::outlets`' full-face
    // branch (coast-only), so the largest rung IS the control and the smallest is
    // the shipped builder. A binary tiled-vs-face comparison answers "does it
    // matter here"; the sweep answers "at what grain does it start to matter",
    // which is the form the repair actually needs.
    let carve = |g: usize, chain_out: Option<&mut Vec<(u32, Vec<f32>)>>| -> (Vec<f32>, std::time::Duration) {
        let t = Instant::now();
        let mut out = vec![0.0f32; n * n];
        let per = n / g;
        let mut chain_sink = chain_out;
        for tj in 0..per {
            for ti in 0..per {
                let (oi, oj) = (ti * g, tj * g);
                let mut f = Fluvial::from_surface(seed, face, level, oi as u32, oj as u32, g, |c| {
                    vivarium_world::gen::initial_topography_m(seed, c, level)
                });
                f.set_uplift_rate(window(&up_face, n, oi, oj, g));
                // The builder normalizes precipitation by the TILE mean.
                // Reproduced so this is the shipped path, not an approximation.
                let mut w = window(&w_face, n, oi, oj, g);
                let m = w.iter().map(|v| *v as f64).sum::<f64>() / w.len() as f64;
                if m > 0.0 {
                    for v in w.iter_mut() {
                        *v /= m as f32;
                    }
                }
                f.set_precip_weight(w);
                if let Some(sink) = chain_sink.as_deref_mut() {
                    // Stride rungs, so the settle history is measurable. The
                    // per-epoch step is identical either way
                    // (`#form-time-indexed-stage-chains` FE(2)).
                    let mut done = 0u32;
                    while done < epochs {
                        let k = stride.min(epochs - done);
                        f.erode(&FluvialParams { epochs: k, ..params.clone() });
                        done += k;
                        sink.push((done, f.h.clone()));
                    }
                } else {
                    f.erode(&params);
                }
                blit(&mut out, n, oi, oj, g, &f.h);
            }
        }
        (out, t.elapsed())
    };

    // Sweep from a grain far below the natural basin width up to the whole face.
    // The shipped grain is one rung among them, not the anchor: the interesting
    // question is where the knee is relative to the basin the geography wants,
    // and at L9 the shipped 64-cell tile turns out to be on the flat side of it.
    let mut grains: Vec<usize> = Vec::new();
    let mut g = env_u64("VIVARIUM_GRAIN_MIN", 8) as usize;
    while g <= n {
        grains.push(g);
        g *= 2;
    }
    let cell_km = vivarium_world::sample::cell_size_m(level, vivarium_world::planet::Planet::EARTH.radius_m) / 1000.0;

    let (lab, comps) = land_components(&prior, n, sea);
    let biggest = (0..comps.len()).max_by_key(|&i| comps[i].0).unwrap_or(0);

    println!("\n-- Part 2: the carve sweep. One geography, tile grain swept. --");
    println!("   cell {cell_km:.1} km at L{level}; the largest rung ({n}) is the whole-face coast-only control.");
    println!(
        "{:>7} {:>9} {:>8} {:>7} {:>9} {:>12} {:>10} {:>12} {:>9} {:>8}",
        "grain", "span km", "dep>1m", "dep%", "deepest", "land trunk", "span km", "big-land trk", "seam/int", "carve"
    );
    let mut face_bed: Option<Vec<f32>> = None;
    let mut rows: Vec<(usize, f32, f32)> = Vec::new();
    for &g in &grains {
        let (bed, dt) = carve(g, None);
        let d = read_face(seed, face, level, n, &bed, &w_face);
        let trunk = land_trunk(&d, &bed, sea, None);
        let trunk_big = land_trunk(&d, &bed, sea, Some((&lab, biggest)));
        let (ss, si, _, _) = seam_step(&bed, n, tile, sea);
        let land = d.stats.subaerial.max(1) as f32;
        println!(
            "{g:>7} {:>9.0} {:>8} {:>7.2} {:>9.0} {:>12.3e} {:>10.0} {:>12.3e} {:>9.3} {:>8.1?}",
            g as f64 * cell_km,
            d.stats.depression_cells,
            100.0 * d.stats.depression_cells as f32 / land,
            d.stats.deepest_depression_m,
            trunk as f64,
            (trunk as f64).sqrt() / 1000.0,
            trunk_big as f64,
            if si > 0.0 { ss / si } else { 0.0 },
            dt
        );
        rows.push((g, trunk, trunk_big));
        if g == n {
            face_bed = Some(bed);
        }
    }
    // The prior's own row, as the null for every column.
    {
        let (ss, si, _, _) = seam_step(&prior, n, tile, sea);
        let land = d_prior.stats.subaerial.max(1) as f32;
        println!(
            "{:>7} {:>9} {:>8} {:>7.2} {:>9.0} {:>12.3e} {:>10.0} {:>12.3e} {:>9.3} {:>8}",
            "PRIOR",
            "-",
            d_prior.stats.depression_cells,
            100.0 * d_prior.stats.depression_cells as f32 / land,
            d_prior.stats.deepest_depression_m,
            land_trunk(&d_prior, &prior, sea, None) as f64,
            (land_trunk(&d_prior, &prior, sea, None) as f64).sqrt() / 1000.0,
            land_trunk(&d_prior, &prior, sea, Some((&lab, biggest))) as f64,
            if si > 0.0 { ss / si } else { 0.0 },
            "-"
        );
    }
    println!(
        "   land trunk = largest MFD drainage on a SUBAERIAL cell (the whole-face max is an ocean\n   cell carrying the face's total discharge, which is why it is not the column here).\n   span km = sqrt(trunk), the basin's equivalent width, to compare against the grain's own span.\n   seam/int = mean |dh| across {tile}-cell boundaries over mean |dh| elsewhere; PRIOR is the null."
    );

    // ---- Part 3: what actually removes the depressions. ----
    // The sweep's `dep>1m` column falls MONOTONICALLY as the grain grows, and the
    // whole-face control holds the fewest closed depressions of any arm. That is
    // the opposite of "the tiling removed the basins," so the cause has to be
    // found rather than assumed. `Fluvial::erode` calls `fill_depressions` and
    // does NOT restore heights afterwards (`drainage_surface` and `chi_profile`
    // both save and restore; the epoch loop does not) — so every epoch writes
    // Priority-Flood's fill into the bed the store receives. If that is the
    // destroyer, the prior's depressions die in the FIRST epoch, before incision
    // has had time to do anything.
    println!("\n-- Part 3: epoch by epoch, whole face — when do the prior's depressions die? --");
    println!(
        "   the prior holds {} closed-depression cells, capacity {:.3e} m3, deepest {:.0} m.",
        d_prior.stats.depression_cells, d_prior.stats.depression_volume_m3, d_prior.stats.deepest_depression_m
    );
    println!("{:>8} {:>9} {:>7} {:>10} {:>13}", "epoch", "dep>1m", "dep%", "deepest", "capacity m3");
    {
        let mut f1 = Fluvial::from_surface(seed, face, level, 0, 0, n, |c| {
            vivarium_world::gen::initial_topography_m(seed, c, level)
        });
        f1.set_uplift_rate(up_face.clone());
        f1.set_precip_weight(w_face.clone());
        for e in 1..=8u32 {
            f1.erode(&FluvialParams { epochs: 1, ..params.clone() });
            let d = read_face(seed, face, level, n, &f1.h, &w_face);
            let land = d.stats.subaerial.max(1) as f32;
            println!(
                "{e:>8} {:>9} {:>7.2} {:>10.0} {:>13.3e}",
                d.stats.depression_cells,
                100.0 * d.stats.depression_cells as f32 / land,
                d.stats.deepest_depression_m,
                d.stats.depression_volume_m3
            );
        }
    }
    println!(
        "   Incision at these rates cannot remove a 688 m closed basin in one epoch. A collapse\n   at epoch 1 convicts the fill; a gradual decline over the eight would refute it."
    );

    let faced = face_bed.expect("the face rung is always in the sweep");
    let d_face_owned = read_face(seed, face, level, n, &faced, &w_face);
    let d_face = &d_face_owned;
    let (tiled, t_tiled) = carve(tile, None);
    let d_tiled = read_face(seed, face, level, n, &tiled, &w_face);

    // ---- Part 4: the three-surface comparison at the shipped grain. ----
    println!("\n-- Part 4: three surfaces, one whole-face reader (grain {tile} = the shipped builder) --");
    println!("   (carve cost at grain {tile}: {t_tiled:.1?})");
    println!("{}", header());
    println!("{}", row("PRIOR", &d_prior, &prior, sea));
    println!("{}", row("TILED", &d_tiled, &tiled, sea));
    println!("{}", row("FACE", d_face, &faced, sea));
    println!(
        "\n   dep>1m = cells the fill had to raise >1 m; trunk m2 = largest MFD drainage (OCEAN cell —\n   see Part 2); big-share = fraction of land runoff in the largest basin; half-n = basins\n   covering half; deg*1e3 = discharge-weighted MFD out-degree x1000."
    );

    // ---- Part 4b: where the tile edge can reach at all. ----
    let mut order: Vec<usize> = (0..comps.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(comps[i].0));
    let land_total: usize = comps.iter().map(|c| c.0).sum();
    let wide: usize = comps.iter().filter(|c| c.1 > tile).map(|c| c.0).sum();
    println!("\n-- Part 4b: the landmass census (on the prior's land mask, shared by all arms) --");
    println!(
        "   {} land components, {} land cells. In components whose bounding box spans >{tile} cells:\n   {} cells ({:.1}% of land) — this is the only land where a grain-{tile} base level could cap anything.",
        comps.len(),
        land_total,
        wide,
        100.0 * wide as f64 / land_total.max(1) as f64
    );
    println!("\n   Largest landmasses, and the trunk each carries under each carve:");
    println!(
        "{:>5} {:>9} {:>7} {:>9} {:>13} {:>13} {:>9}",
        "rank", "cells", "span", "span km", "TILED trunk", "FACE trunk", "FACE/TILED"
    );
    for (r, &c) in order.iter().take(8).enumerate() {
        let t = land_trunk(&d_tiled, &tiled, sea, Some((&lab, c)));
        let f = land_trunk(d_face, &faced, sea, Some((&lab, c)));
        println!(
            "{:>5} {:>9} {:>7} {:>9.0} {:>13.3e} {:>13.3e} {:>9.2}",
            r + 1,
            comps[c].0,
            comps[c].1,
            comps[c].1 as f64 * cell_km,
            t as f64,
            f as f64,
            if t > 0.0 { f / t } else { 0.0 }
        );
    }

    // ---- Part 5: the seam step. ----
    println!("\n-- Part 5: height step across {tile}-cell tile boundaries vs interior links (subaerial) --");
    println!("{:<7} {:>12} {:>12} {:>9} {:>12} {:>12}", "arm", "seam |dh| m", "int |dh| m", "ratio", "seam links", "int links");
    for (tag, h) in [("PRIOR", &prior), ("TILED", &tiled), ("FACE", &faced)] {
        let (s, i, cs, ci) = seam_step(h, n, tile, sea);
        println!("{tag:<7} {s:>12.3} {i:>12.3} {:>9.3} {cs:>12} {ci:>12}", if i > 0.0 { s / i } else { 0.0 });
    }
    println!("   PRIOR is the null: the prior knows nothing of tiles, so its ratio is the noise floor.");

    // ---- Part 6: does the network integrate when it is allowed to? ----
    // `#obs-tile-outlets-grade-away-the-basins` FE(6) measured the network set
    // within ten epochs and only jittering afterwards, and named the tile edge as
    // the reason (base level fixed 32 cells away in every direction, so no room
    // for capture). Run the same history under BOTH boundary contracts and the
    // clause becomes testable rather than argued.
    println!("\n-- Part 6: the settle history under both contracts (grain {tile} vs whole face) --");
    println!(
        "{:>8} {:>13} {:>11} {:>9} | {:>13} {:>11} {:>9}",
        "epochs", "tiled trunk", "big-share", "dep>1m", "face trunk", "big-share", "dep>1m"
    );
    let mut chain_t: Vec<(u32, Vec<f32>)> = Vec::new();
    let mut chain_f: Vec<(u32, Vec<f32>)> = Vec::new();
    let _ = carve(tile, Some(&mut chain_t));
    let _ = carve(n, Some(&mut chain_f));
    // The tiled arm's chain arrives tile-by-tile; regroup it by epoch count.
    let rungs: Vec<u32> = chain_f.iter().map(|(k, _)| *k).collect();
    for (idx, &k) in rungs.iter().enumerate() {
        let mut tbed = vec![0.0f32; n * n];
        let per = n / tile;
        for t in 0..per * per {
            let (ti, tj) = (t % per, t / per);
            let (_, h) = &chain_t[t * rungs.len() + idx];
            blit(&mut tbed, n, ti * tile, tj * tile, tile, h);
        }
        let dt = read_face(seed, face, level, n, &tbed, &w_face);
        let fbed = &chain_f[idx].1;
        let df = read_face(seed, face, level, n, fbed, &w_face);
        println!(
            "{k:>8} {:>13.3e} {:>11.3} {:>9} | {:>13.3e} {:>11.3} {:>9}",
            land_trunk(&dt, &tbed, sea, None) as f64,
            dt.stats.largest_basin_share,
            dt.stats.depression_cells,
            land_trunk(&df, fbed, sea, None) as f64,
            df.stats.largest_basin_share,
            df.stats.depression_cells,
        );
    }
    println!("   A trend in the FACE columns where the TILED ones are flat is the falsifier for\n   'the tile edge, not the epoch count, is what bounds basin integration'.");

    // ---- Part 7: what the reader cannot see. ----
    println!("\n-- Part 7: scope --");
    println!("   * One cube face. The FACE arm still has four edges (the cube seams) and treats");
    println!("     them as a no-flux wall (non-outlet, no receiver outside) — a DIFFERENT undeclared");
    println!("     boundary contract, not an absent one. Basins are capped at the face, not the tile.");
    println!("   * Precipitation is normalized per tile in TILED (the shipped path) and per face in");
    println!("     FACE. The reader uses the face field for all three, so the beds are compared");
    println!("     under one rain.");
    println!("   * PRIOR's depression census is measured on the prior itself, which no epoch has");
    println!("     yet filled. Comparing it to the carved arms is what splits 'the tiling removed");
    println!("     the basins' from 'the per-epoch Priority-Flood fill removed them'.");
}
