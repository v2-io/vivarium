//! **Is there a lake surface, and can a view reach it?**
//!
//! `Fluvial::fill_depressions` returns two fields and its own doc comment says the
//! difference is load-bearing: the **routing surface** (every depression raised to
//! its spill point *plus* an ε-gradient that orients flow across the resulting
//! flat) and the **standing-water depth** (true spill level minus original height,
//! ε excluded — *"the physical statement: this is where water stands"*). `erode`
//! uses them apart, and discards both when the epoch ends.
//!
//! `drainage_surface`, the *reader's* door onto the same computation, keeps only
//! the first: `fill_depth = filled_h − stored h`, which is spill fill AND ε. That
//! is the field the explorer's depression paint pulls. So the question here is not
//! rhetorical — if a view paints `fill_depth` as water, is it painting water?
//!
//! The discriminator is **levelness**, which is what makes a lake a lake. Every
//! cell of one standing body shares one spill float, so `bed + water` is
//! bit-identical across a lake and `max − min` is exactly `0.0`. The ε is applied
//! per flood step, so `bed + fill_depth` over the same cells is a *tilted* sheet,
//! and the tilt is measurable. The cleaner discriminator is a **flat with no
//! depression at all**: no water stands there (the fill's own doc promises exactly
//! zero), but the ε still raises it, so any wetness reported is manufactured.
//!
//! Reports tables, not a pass/fail — a tolerance at one threshold is exactly what
//! would hide a tilt that is small but systematic (`#norm-probe-sensitivity`).
//!
//! **Controls that can refute this probe rather than the kernel.** A is analytic:
//! the pit's spill level is known from the plane, so a wrong lake convicts the
//! probe. B contains no depression, so any lake reported there is probe error. D
//! is deliberately submarine, and must report nothing.
//!
//! **What D is for.** `Fluvial::outlets` marks every cell at or below derived sea
//! level as an outlet, so a fully submarine window has no closed basin by
//! construction and no lake is possible in it. Since ~95% of this planet is below
//! the waterline (`examples/emerged_land_probe`: 5.33% land on seed 0), a window
//! chosen without checking is *probably* submarine — which is why C scans for
//! emerged ground instead of trusting a coordinate.
//!
//! Read-only against the generator — writes nothing, opens no world store.
//!
//! Run: `cargo run --release -p vivarium-world --example lake_surface_probe`

use vivarium_world::erosion::{DrainageSurface, Fluvial, FluvialParams};
use vivarium_world::gen;
use vivarium_world::sea_level::derived_sea_level_m;
use vivarium_world::sphere::{CellId, Face};

const SEED: u64 = 0;
/// Construction level for the synthetic cases and the real tile. At L13 a cell is
/// ~1.2 km, so a 96² window spans ~117 km — a landscape with room for basins,
/// rather than the ~1.9 km patch an L19 window would cover.
const LEVEL: u8 = 13;
const NX: usize = 96;
/// Coarse scan level used to *find* emerged ground. One L8 cell is 32 L13 cells,
/// so a 96-cell window spans a 3×3 block of them.
const SCAN_LEVEL: u8 = 8;

/// One connected body of reported-wet cells, 8-connected to match the fill's own
/// neighbourhood.
struct Body {
    cells: Vec<usize>,
}

fn bodies(nx: usize, wet: &dyn Fn(usize) -> bool) -> Vec<Body> {
    let n = nx * nx;
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    for start in 0..n {
        if seen[start] || !wet(start) {
            continue;
        }
        let mut stack = vec![start];
        seen[start] = true;
        let mut cells = Vec::new();
        while let Some(i) = stack.pop() {
            cells.push(i);
            let (x, y) = (i % nx, i / nx);
            for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let (xp, yp) = (x as i32 + dx, y as i32 + dy);
                if xp < 0 || yp < 0 || xp >= nx as i32 || yp >= nx as i32 {
                    continue;
                }
                let j = yp as usize * nx + xp as usize;
                if !seen[j] && wet(j) {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
        out.push(Body { cells });
    }
    out.sort_by_key(|b| std::cmp::Reverse(b.cells.len()));
    out
}

/// Spread of the reported water *surface* over one body: `max − min` of
/// `bed + depth`. Exactly zero means level — a lake. Anything else is a tilted
/// sheet, and the number is how far from level it is.
fn surface_spread_m(body: &Body, bed: &[f32], depth: &[f32]) -> f32 {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;
    for &i in &body.cells {
        let s = bed[i] + depth[i];
        lo = lo.min(s);
        hi = hi.max(s);
    }
    hi - lo
}

fn volume_km3(body: &Body, depth: &[f32], area: &[f32]) -> f64 {
    body.cells.iter().map(|&i| depth[i] as f64 * area[i] as f64).sum::<f64>() / 1e9
}

fn report(label: &str, bed: &[f32], depth: &[f32], area: &[f32], nx: usize) {
    let wet = |i: usize| depth[i] > 0.0;
    let bs = bodies(nx, &wet);
    let total: usize = bs.iter().map(|b| b.cells.len()).sum();
    println!("  {label:<26} wet cells {total:>6} / {}   bodies {:>4}", nx * nx, bs.len());
    if bs.is_empty() {
        return;
    }
    println!("      {:>5}  {:>9}  {:>16}  {:>11}", "cells", "vol km³", "surface spread", "max depth");
    for b in bs.iter().take(4) {
        let spread = surface_spread_m(b, bed, depth);
        let maxd = b.cells.iter().fold(0.0f32, |m, &i| m.max(depth[i]));
        println!(
            "      {:>5}  {:>9.4}  {:>13.6} m  {:>9.3} m   {}",
            b.cells.len(),
            volume_km3(b, depth, area),
            spread,
            maxd,
            if spread == 0.0 { "LEVEL" } else { "tilted" }
        );
    }
    if bs.len() > 4 {
        println!("      … {} more bodies", bs.len() - 4);
    }
}

/// Both per-cell fields a reader can reach. `fill_depth` is the ε-augmented raise
/// the reader used to expose alone; `standing_water` is the physical spill-level
/// depth. Printing them side by side is the whole argument.
fn fields(ds: &DrainageSurface) -> Vec<(&'static str, &[f32])> {
    vec![
        ("fill_depth (raise, ε in)", &ds.fill_depth),
        ("standing_water (ε out)", &ds.standing_water),
    ]
}

/// Land fraction and relief of one candidate window, measured on the same prior
/// the erosion kernel seeds from.
fn window_stats(face: Face, oi: u32, oj: u32, sea: f64) -> (f32, f64, f64) {
    let (mut land, mut lo, mut hi) = (0usize, f64::INFINITY, f64::NEG_INFINITY);
    for y in 0..NX {
        for x in 0..NX {
            let c = CellId::from_face_ij(face, oi + x as u32, oj + y as u32, LEVEL);
            let h = gen::initial_topography_m(SEED, c, LEVEL);
            if h > sea {
                land += 1;
            }
            lo = lo.min(h);
            hi = hi.max(h);
        }
    }
    (land as f32 / (NX * NX) as f32, hi - lo, hi - sea)
}

/// Find an emerged, high-relief window by scanning the prior at `SCAN_LEVEL`
/// against **derived** sea level (not the retired `gen::SEA_LEVEL_M` datum), then
/// measuring the actual candidate windows.
fn find_emerged_window(sea: f64) -> Option<(Face, u32, u32, f32, f64)> {
    let n = 1u32 << SCAN_LEVEL;
    let ratio = 1u32 << (LEVEL - SCAN_LEVEL);
    let mut cands: Vec<(f64, Face, u32, u32)> = Vec::new();
    for face in [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZPos, Face::ZNeg] {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let h = gen::initial_topography_m(SEED, CellId::from_face_ij(face, i, j, SCAN_LEVEL), SCAN_LEVEL);
                if h > sea {
                    cands.push((h - sea, face, i, j));
                }
            }
        }
    }
    cands.sort_by(|a, b| b.0.total_cmp(&a.0));
    println!("  scan: {} emerged cells at L{SCAN_LEVEL}; testing the tallest windows", cands.len());
    for &(_, face, i, j) in cands.iter().take(24) {
        // Centre the window on the emerged cell.
        let oi = (i * ratio).saturating_sub(NX as u32 / 2);
        let oj = (j * ratio).saturating_sub(NX as u32 / 2);
        let (land, relief, stand) = window_stats(face, oi, oj, sea);
        if land > 0.98 && relief > 100.0 {
            println!(
                "  using {face:?}({oi},{oj}) at L{LEVEL}: land {:.1}%, relief {:.0} m, peak stand {:.0} m above sea",
                land * 100.0,
                relief,
                stand
            );
            return Some((face, oi, oj, land, relief));
        }
    }
    None
}

fn main() {
    let sea = derived_sea_level_m(SEED);
    println!("derived sea level (seed {SEED}): {sea:.1} m — the outlet contour");
    println!();

    println!("== A. one pit in a tilted plane, above sea — the analytic case ==");
    // A plane falling 5 m per cell in x, 800 m above the waterline at its high
    // edge, with a 200 m cone gouged from the middle. The pit's spill level is
    // the plane height at its lowest rim cell, so a correct lake is level there
    // and nothing else is wet.
    let (oi, oj) = (2000u32, 3000u32);
    let mut f = Fluvial::from_surface(SEED, Face::XPos, LEVEL, oi, oj, NX, |_| 0.0);
    for y in 0..NX {
        for x in 0..NX {
            let plane = (sea as f32 + 800.0) - 5.0 * x as f32;
            let (cx, cy) = (NX as f32 / 2.0, NX as f32 / 2.0);
            let r = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
            let gouge = if r < 20.0 { 200.0 * (1.0 - r / 20.0) } else { 0.0 };
            f.h[y * NX + x] = plane - gouge;
        }
    }
    let bed = f.h.clone();
    let area = f.cell_area.clone();
    let ds = f.drainage_surface();
    for (label, depth) in fields(&ds) {
        report(label, &bed, depth, &area, NX);
    }
    println!("      bed restored bit-identically: {}", bed == f.h);

    println!();
    println!("== B. a flat shelf and a ramp, above sea — no depression anywhere (control) ==");
    let mut f = Fluvial::from_surface(SEED, Face::XPos, LEVEL, oi, oj, NX, |_| 0.0);
    for y in 0..NX {
        for x in 0..NX {
            let half = NX / 2;
            let base = sea as f32 + 500.0;
            f.h[y * NX + x] = if x < half { base } else { base - (x - half) as f32 };
        }
    }
    let bed = f.h.clone();
    let area = f.cell_area.clone();
    let ds = f.drainage_surface();
    for (label, depth) in fields(&ds) {
        report(label, &bed, depth, &area, NX);
    }

    println!();
    println!("== C. a real eroded tile on emerged land — the workload a view paints ==");
    match find_emerged_window(sea) {
        None => println!("  no emerged window found — C cannot run, and that is a finding, not a pass"),
        Some((face, oi, oj, _, _)) => {
            let mut f = Fluvial::from_prior(SEED, face, LEVEL, oi, oj, NX);
            let p = FluvialParams { epochs: 60, ..Default::default() };
            f.erode(&p);
            let bed = f.h.clone();
            let area = f.cell_area.clone();
            let ds = f.drainage_surface();
            for (label, depth) in fields(&ds) {
                report(label, &bed, depth, &area, NX);
            }
            println!(
                "      stats: depression cells {} · capacity {:.4} km³ · deepest {:.2} m",
                ds.stats.depression_cells,
                ds.stats.depression_volume_m3 / 1e9,
                ds.stats.deepest_depression_m
            );

            // **How much of this is the carve, and how much is the prior?** The
            // question Joseph's eye asked of the view: a basin with an apparent
            // outlet to the sea should not hold water. One candidate cause is that
            // the reader is filling pits the fluvial kernel never made — the
            // band-limited prior's own small-scale relief. The same window,
            // uncarved, answers it directly: if the prior alone reports as much
            // standing water as the carved surface, the bodies are prior detail
            // rather than drainage.
            println!("  -- the SAME window, uncarved prior (how much is the carve?) --");
            let mut f0 = Fluvial::from_prior(SEED, face, LEVEL, oi, oj, NX);
            let bed0 = f0.h.clone();
            let area0 = f0.cell_area.clone();
            let ds0 = f0.drainage_surface();
            report("standing_water (PRIOR)", &bed0, &ds0.standing_water, &area0, NX);
        }
    }

    println!();
    println!("== D. a submarine window — must report nothing (outlet-policy control) ==");
    let mut f = Fluvial::from_prior(SEED, Face::XPos, 19, 2000, 3000, NX);
    let p = FluvialParams { epochs: 60, ..Default::default() };
    f.erode(&p);
    let bed = f.h.clone();
    let area = f.cell_area.clone();
    let below = bed.iter().filter(|&&h| (h as f64) <= sea).count();
    let ds = f.drainage_surface();
    println!(
        "  cells below the datum: {below}/{} — all of them reach this window's rim, so all are ocean",
        NX * NX
    );
    for (label, depth) in fields(&ds) {
        report(label, &bed, depth, &area, NX);
    }

    println!();
    println!("== F. what the READER's domain choice costs, at a view finer than the carve ==");
    // The fix this probe convicts. A view drawing L+1 over an L carve gets its
    // surface from `ErodedRegion::surface_m`, which returns bilinear-over-carve
    // PLUS the prior's detail increment. Computing standing water on THAT is
    // computing on a surface no rung produced. Sampling the region's own field
    // down to the view instead is a view of the rendered physics.
    //
    // Both columns below describe the same geography at the same view level. The
    // difference is entirely which surface the reader ran on.
    {
        let (face, oi, oj) = (Face::XPos, 6512u32, 1552u32);
        let mut carved = Fluvial::from_prior(SEED, face, LEVEL, oi, oj, NX);
        carved.erode(&FluvialParams { epochs: 60, ..Default::default() });
        let region = carved.to_region();
        let region_lake = region.standing_water();

        // The view: one level finer, same footprint.
        let vlevel = LEVEL + 1;
        let vnx = NX * 2;
        let (voi, voj) = (oi * 2, oj * 2);

        // (b) OLD reader — build the drawn surface the way a view does, then run
        // the fill on it.
        let mut drawn = Fluvial::from_surface(SEED, face, vlevel, voi, voj, vnx, |c| {
            region.surface_m(c).unwrap_or_else(|| gen::initial_topography_m(SEED, c, vlevel))
        });
        let drawn_bed = drawn.h.clone();
        let drawn_area = drawn.cell_area.clone();
        let ds_drawn = drawn.drainage_surface();

        // (c) NEW reader — sample the region's own field at its own level.
        let mut sampled = vec![0.0f32; vnx * vnx];
        let mut unanswered = 0usize;
        for j in 0..vnx {
            for i in 0..vnx {
                let c = CellId::from_face_ij(face, voi + i as u32, voj + j as u32, vlevel);
                match region.carved_index(c) {
                    Some(k) => sampled[j * vnx + i] = region_lake[k],
                    None => unanswered += 1,
                }
            }
        }

        println!("  view L{vlevel} over an L{LEVEL} carve, {vnx}² cells ({unanswered} uncovered by the region)");
        report("(b) OLD: on drawn surface", &drawn_bed, &ds_drawn.standing_water, &drawn_area, vnx);
        report("(c) NEW: sampled at carve", &drawn_bed, &sampled, &drawn_area, vnx);
        println!("      the (b) bodies are pits in re-added prior detail; (c) can only report basins a kernel carved");
    }

    println!();
    println!("== E. the planet, per whole cube face at L8 — what standing water exists ==");
    // A whole face infers `NoFluxWall`, so the only outlets are cells the ocean
    // actually reaches. This is the domain that adjudicates real basins: a window
    // narrower than an enclosed sea reads that sea as ocean, a whole face does not.
    // Coarse by declaration — an L8 cell is ~78 km, so only basins bigger than that
    // can appear at all, and the count is a floor rather than a census.
    let n = 1usize << SCAN_LEVEL;
    let (mut tot_lakes, mut tot_cells, mut tot_vol) = (0usize, 0usize, 0.0f64);
    for face in [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZPos, Face::ZNeg] {
        let mut f = Fluvial::from_prior(SEED, face, SCAN_LEVEL, 0, 0, n);
        let bed = f.h.clone();
        let area = f.cell_area.clone();
        let submerged = bed.iter().filter(|&&h| (h as f64) <= sea).count();
        let ds = f.drainage_surface();
        let wet = |i: usize| ds.standing_water[i] > 0.0;
        let bs = bodies(n, &wet);
        let cells: usize = bs.iter().map(|b| b.cells.len()).sum();
        let vol: f64 = bs.iter().map(|b| volume_km3(b, &ds.standing_water, &area)).sum();
        let deepest = ds.standing_water.iter().copied().fold(0.0f32, f32::max);
        let level_all = bs.iter().all(|b| surface_spread_m(b, &bed, &ds.standing_water) == 0.0);
        println!(
            "  {face:<6?} submerged {submerged:>6}/{}  lakes {:>4}  cells {cells:>5}  {vol:>9.1} km³  deepest {deepest:>7.1} m  all level: {level_all}",
            n * n,
            bs.len()
        );
        tot_lakes += bs.len();
        tot_cells += cells;
        tot_vol += vol;
    }
    println!("  planet total: {tot_lakes} standing bodies, {tot_cells} cells, {tot_vol:.1} km³");
}
