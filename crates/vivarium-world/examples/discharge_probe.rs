//! **Where the water flows** — the drainage field the fluvial kernel computes
//! every epoch and discards, read back off stored stages and made visible.
//!
//! The kernel's `Fluvial::drainage` decides where incision happens and then dies
//! with the run: only heights reach the store. `Fluvial::drainage_surface` is the
//! reader that recovers it from any stored stage (a pure function of that stage,
//! recomputed rather than memoized — the `response_census` precedent). This probe
//! is what that reader is for, and it asks four questions the tree had not asked:
//!
//! 1. **Is there a channel network at all, and how concentrated is it?**
//! 2. **Does it integrate over world-time?** — the beacon's 30 stages are a
//!    settle history, so basin integration is watchable rather than assumed.
//! 3. **How much of the "river" is the MFD fan?** MFD with p=1.0 spreads flow
//!    over every downhill neighbour and does not converge
//!    (`#obs-cube-locked-kernel-bias` FE(1)). D8 down the same tree concentrates.
//!    Painting one and calling it the other is exactly the manufactured content
//!    `#norm-no-depiction-without-referent` exists to stop, so both are measured.
//! 4. **Do tile edges cut the basins?** Every builder tile is an independent
//!    64x64 field whose edges are outlets, so routing cannot cross a tile seam.
//!    Re-routing the *assembled* 4x4 patch as one field measures what that costs.
//!
//! Read-only: opens the live world's store read-only, writes nothing, and pulls
//! precipitation from `climate::precip_jitter_factor` directly (a pure function
//! of seed+cell) rather than through the memoizing `climate_tile`.
//!
//! Run: `cargo run --release --example discharge_probe`
//!      `VIVARIUM_WORLD=/path/to/world cargo run --release --example discharge_probe`

use vivarium_world::erosion::{DrainageSurface, ErodedRegion, Fluvial, FluvialParams, CHANNEL_THRESHOLD_CELLS};
use vivarium_world::planet::Planet;
use vivarium_world::query::World;
use vivarium_world::sample::cell_size_m;
use vivarium_world::sphere::{CellId, Face};
use vivarium_world::store::Store;

/// The beacon patch (`msc/agent-briefs/l13-patch-coords.md`; the manifest is the
/// authority — these are the fallback when the manifest cannot be read).
const BEACON_LEVEL: u8 = 13;

fn key_field<'a>(key: &'a str, field: &str) -> Option<&'a str> {
    key.split('|').find_map(|p| p.strip_prefix(field)?.strip_prefix('='))
}

/// Precip weight (relative to the tile mean) the kernel would have used — from
/// the fated jitter directly, so no store write is needed. `Fluvial::from_region`
/// leaves this at ones (uniform rain), which is NOT what the kernel ran.
fn precip_weight(seed: u64, r: &ErodedRegion) -> Vec<f32> {
    let mut w: Vec<f32> = Vec::with_capacity(r.nx * r.nx);
    for j in 0..r.nx as u32 {
        for i in 0..r.nx as u32 {
            let cell = CellId::from_face_ij(r.face, r.oi + i, r.oj + j, r.level);
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

fn surface_of(r: &ErodedRegion, seed: u64, with_rain: bool) -> DrainageSurface {
    let mut f = Fluvial::from_region(r);
    if with_rain {
        f.set_precip_weight(precip_weight(seed, r));
    }
    f.drainage_surface()
}

/// Discharge as an ASCII ramp, in units of "cells drained" (log scale). Below the
/// derived sea the cell is drawn `~` — it is ocean, not a dry channel. A blank is
/// land draining fewer than 4 cells' worth.
const RAMP: [char; 8] = ['.', ':', '-', '=', '+', '*', '#', '@'];

fn glyph(cells_drained: f32, submarine: bool) -> char {
    if submarine {
        return '~';
    }
    if cells_drained < 4.0 {
        return ' ';
    }
    // log4: 4 -> 0, 16 -> 1, 64 -> 2, ... 65536 -> 7
    let k = (cells_drained.log2() / 2.0 - 1.0).floor().clamp(0.0, 7.0) as usize;
    RAMP[k]
}

/// Max-pool a field by `blk` — the brightest cell in each block. Declared,
/// because it is the honest downsample for a thread: a mean would dissolve the
/// channel it is drawn to show, and this reports a real cell's real value.
fn max_pool(field: &[f32], nx: usize, blk: usize) -> (Vec<f32>, usize) {
    let out_n = nx / blk;
    let mut out = vec![0.0f32; out_n * out_n];
    for y in 0..out_n {
        for x in 0..out_n {
            let mut m = f32::NEG_INFINITY;
            for dy in 0..blk {
                for dx in 0..blk {
                    m = m.max(field[(y * blk + dy) * nx + x * blk + dx]);
                }
            }
            out[y * out_n + x] = m;
        }
    }
    (out, out_n)
}

/// `h` is the surface used only to decide what is ocean.
fn render(field_cells: &[f32], h: &[f32], nx: usize, sea: f32, label: &str) {
    println!("  {label}");
    for y in 0..nx {
        let row: String =
            (0..nx).map(|x| glyph(field_cells[y * nx + x], h[y * nx + x] <= sea)).collect();
        println!("  |{row}|");
    }
}

fn stats_line(tag: &str, d: &DrainageSurface) -> String {
    let s = &d.stats;
    format!(
        "{tag:<22} subaerial {:>5}/{:<5} basins {:>4} largest {:>5.1}% half-in {:>3}  \
         chan(MFD) {:>4}/{:>4}/{:>4}  chan(D8) {:>4}/{:>4}/{:>4}  spread {:>5.2}x  out-deg {:>4.2}  \
         straight p50 {:>2} max {:>3} (>=8: {:>4} cells, {:>4.0}% in fill)",
        s.subaerial,
        s.cells,
        s.basins,
        100.0 * s.largest_basin_share,
        s.basins_for_half,
        s.channel_cells_mfd[0],
        s.channel_cells_mfd[1],
        s.channel_cells_mfd[2],
        s.channel_cells_d8[0],
        s.channel_cells_d8[1],
        s.channel_cells_d8[2],
        s.spread_ratio,
        s.mean_out_degree,
        s.straight_run_p50,
        s.straight_run_max,
        s.straight_cells,
        100.0 * s.straight_in_fill_frac,
    )
}

fn main() {
    let world_dir = std::env::var("VIVARIUM_WORLD").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".cache/vivarium/globe-world")
    });
    let Ok(store) = Store::open_read_only(&world_dir) else {
        println!("(no world at {} — nothing to read)", world_dir.display());
        return;
    };
    let Ok(Some(spec)) = vivarium_world::spec::WorldSpec::load(&world_dir) else {
        println!("(no manifest at {} — skipping)", world_dir.display());
        return;
    };
    let seed = spec.seed;
    let world = World::new(&store, seed);
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed) as f32;
    let cell_km = cell_size_m(BEACON_LEVEL, Planet::EARTH.radius_m) / 1000.0;

    println!("== discharge in world \"{}\" (seed {seed}) — read-only ==", spec.name);
    println!(
        "L{BEACON_LEVEL} cells ~{cell_km:.2} km; derived sea {sea:.1} m; channel rungs = \
         {:?} x median cell area",
        CHANNEL_THRESHOLD_CELLS
    );
    println!(
        "MFD (p=1.0) is the kernel's own field and is DIFFUSED; D8 is the same tree accumulated \
         single-receiver.\nspread = maxD8/maxMFD (1.0 = the fan is as tight as a thread); \
         out-deg = discharge-weighted downhill neighbour count (1 = thread).\n"
    );

    // Which stages exist for the beacon patch, and under which source hash.
    let all = world.observe().load_eroded_regions_where(|k| key_field(k, "level") == Some("13"));
    if all.is_empty() {
        println!("(no L13 tiles in this store — the beacon has not been built)");
        return;
    }
    let cur = vivarium_world::nomotheke::SRC_HASH;
    let fresh = world
        .observe()
        .load_eroded_regions_where(|k| key_field(k, "level") == Some("13") && key_field(k, "src") == Some(cur))
        .len();
    if fresh == 0 {
        println!(
            "(!) the store is stale-by-src for this binary: L13 tiles exist but none carved under \
             {cur}.\n    Saying it out loud rather than hiding it — the network SHAPE of a stale \
             bed is still honest terrain,\n    and every number below is about the shape of a \
             surface the store really holds. Rerun `vivarium build` for the current law.\n"
        );
    }

    // ---- Part 1: the beacon patch's final stage, tile by tile ----
    let face = all[0].face;
    let (o_i, o_j) = (all.iter().map(|r| r.oi).min().unwrap(), all.iter().map(|r| r.oj).min().unwrap());
    let tile_nx = all[0].nx;
    println!("-- Part 1: beacon patch f{} origin ({o_i},{o_j}), {} tiles of {tile_nx}^2, latest stage each --", face.index(), all.len());
    let mut with: Vec<DrainageSurface> = Vec::new();
    for r in &all {
        let d = surface_of(r, seed, true);
        println!("   {}", stats_line(&format!("tile ({},{})", r.oi, r.oj), &d));
        with.push(d);
    }

    // ---- Part 2: does rain matter? uniform vs the fated jitter ----
    println!("\n-- Part 2: does the climate field move the discharge? (Fluvial::from_region defaults to UNIFORM rain) --");
    for (r, wd) in all.iter().zip(with.iter()).take(4) {
        let dry = surface_of(r, seed, false);
        println!(
            "   tile ({:>4},{:>4})  maxMFD  uniform {:>10.3e}  fated-jitter {:>10.3e}  ratio {:>5.2}   \
             chan@100 {:>4} -> {:>4}",
            r.oi,
            r.oj,
            dry.stats.max_mfd_m2,
            wd.stats.max_mfd_m2,
            wd.stats.max_mfd_m2 / dry.stats.max_mfd_m2.max(1.0),
            dry.stats.channel_cells_mfd[1],
            wd.stats.channel_cells_mfd[1],
        );
    }

    // ---- Part 3: the settle history — does the network integrate in world-time? ----
    let target = &all[all.len() / 2];
    let mut stages: Vec<u32> = Vec::new();
    {
        // Every stage key for this one tile.
        let want = (target.oi, target.oj);
        for e in (0..=1000u32).step_by(10) {
            let got = world.observe().load_eroded_regions_where(|k| {
                key_field(k, "level") == Some("13")
                    && key_field(k, "epochs").and_then(|v| v.parse::<u32>().ok()) == Some(e)
                    && key_field(k, "oi").and_then(|v| v.parse::<u32>().ok()) == Some(want.0)
                    && key_field(k, "oj").and_then(|v| v.parse::<u32>().ok()) == Some(want.1)
            });
            if !got.is_empty() {
                stages.push(e);
            }
        }
    }
    println!(
        "\n-- Part 3: tile ({},{}) across its settle history ({} stored stages) --",
        target.oi,
        target.oj,
        stages.len()
    );
    for e in &stages {
        let want = (target.oi, target.oj);
        let got = world.observe().load_eroded_regions_where(|k| {
            key_field(k, "level") == Some("13")
                && key_field(k, "epochs").and_then(|v| v.parse::<u32>().ok()) == Some(*e)
                && key_field(k, "oi").and_then(|v| v.parse::<u32>().ok()) == Some(want.0)
                && key_field(k, "oj").and_then(|v| v.parse::<u32>().ok()) == Some(want.1)
        });
        if let Some(r) = got.first() {
            let d = surface_of(r, seed, true);
            println!("   {}", stats_line(&format!("epoch {e:>4}"), &d));
        }
    }

    // ---- Part 4: tile-local routing vs the assembled patch ----
    // Every builder tile is its own field with outlet edges, so no flow path can
    // cross a tile seam. Re-routing the assembled patch measures the cut.
    let span = {
        let max_i = all.iter().map(|r| r.oi).max().unwrap();
        ((max_i - o_i) as usize / tile_nx + 1) * tile_nx
    };
    println!(
        "\n-- Part 4: tile-local routing vs the ASSEMBLED {span}^2 patch (~{:.0} km square) --",
        span as f64 * cell_km
    );
    println!("   Builder tiles route independently: every tile edge is an outlet, so a basin cannot cross one.");
    let (assembled, any) = world.observe().assemble_surface_tile(face, BEACON_LEVEL, o_i, o_j, span, &all);
    if !any {
        println!("   (assembly found no eroded coverage — skipping)");
        return;
    }
    let mut f = {
        let a = assembled.clone();
        let mut f = Fluvial::from_surface(seed, face, BEACON_LEVEL, o_i, o_j, span, |c| {
            let (cf, ci, cj, _) = c.to_face_ij();
            if cf.index() == face.index() && ci >= o_i && cj >= o_j {
                let (di, dj) = ((ci - o_i) as usize, (cj - o_j) as usize);
                if di < span && dj < span {
                    return a[dj * span + di] as f64;
                }
            }
            0.0
        });
        let mut w = Vec::with_capacity(span * span);
        for j in 0..span as u32 {
            for i in 0..span as u32 {
                w.push(vivarium_world::climate::precip_jitter_factor(
                    seed,
                    CellId::from_face_ij(face, o_i + i, o_j + j, BEACON_LEVEL),
                ) as f32);
            }
        }
        let mean = w.iter().map(|v| *v as f64).sum::<f64>() / w.len() as f64;
        for v in w.iter_mut() {
            *v /= mean as f32;
        }
        f.set_precip_weight(w);
        f
    };
    let whole = f.drainage_surface();
    println!("   {}", stats_line("assembled patch", &whole));
    let tile_max = with.iter().map(|d| d.stats.max_mfd_m2).fold(0.0f32, f32::max);
    println!(
        "   biggest trunk: tile-local {:>10.3e} m^2   assembled {:>10.3e} m^2   gain {:>5.2}x",
        tile_max,
        whole.stats.max_mfd_m2,
        whole.stats.max_mfd_m2 / tile_max.max(1.0)
    );

    // ---- Part 5: the picture ----
    println!("\n-- Part 5: the patch, drawn. Units: cells drained (log4 ramp {RAMP:?}); '~' = below derived sea; blank = <4 cells. --");
    println!("   Downsample is MAX-POOL (the brightest real cell in each block), declared: a mean would dissolve the thread.");
    let blk = if span > 128 { span / 96 } else { 1 };
    let cells_mfd = whole.in_cells(&whole.mfd);
    let cells_d8 = whole.in_cells(&whole.d8);
    let (pm, pn) = max_pool(&cells_mfd, span, blk);
    let (pd, _) = max_pool(&cells_d8, span, blk);
    let (ph, _) = max_pool(&assembled, span, blk);
    render(&pm, &ph, pn, sea, &format!("MFD p=1.0 (the kernel's own field — DIFFUSED), {blk}x max-pool"));
    render(&pd, &ph, pn, sea, &format!("D8 single-receiver (concentrates; grid-aligned artifact), {blk}x max-pool"));

    // One tile at full resolution — no pooling, nothing to argue about.
    let t = &all[all.len() / 2];
    let td = surface_of(t, seed, true);
    println!("\n   one tile at FULL resolution, no pooling — ({},{}) {tile_nx}^2:", t.oi, t.oj);
    render(&td.in_cells(&td.mfd), &t.h, tile_nx, sea, "MFD");
    render(&td.in_cells(&td.d8), &t.h, tile_nx, sea, "D8");

    // ---- Part 6: depression capacity — the standing-water question, from the erosion side ----
    println!("\n-- Part 6: closed depressions on the stored surface (Priority-Flood's own fill) --");
    println!("   This is GEOMETRIC CAPACITY to the spill point, not a lake: no evaporation, inflow,");
    println!("   seepage or residence time is in the account. It is an independent referent for");
    println!("   'could inland standing water exist here at all' that does not go through the water nomos.");
    let s = &whole.stats;
    println!(
        "   assembled patch: {} of {} cells hold >1 m of depression ({:.2}%), deepest {:.1} m, capacity {:.4e} m^3",
        s.depression_cells,
        s.cells,
        100.0 * s.depression_cells as f32 / s.cells as f32,
        s.deepest_depression_m,
        s.depression_volume_m3,
    );
    let per_tile: usize = with.iter().map(|d| d.stats.depression_cells).sum();
    let deepest_tile = with.iter().map(|d| d.stats.deepest_depression_m).fold(0.0f32, f32::max);
    println!(
        "   sum over the 16 tile-local routings: {per_tile} cells, deepest {deepest_tile:.2} m — \
         and THAT is the difference:"
    );
    println!(
        "   a builder tile is carved as if surrounded by sea (every edge an outlet), so it grades\n   \
         itself to its own perimeter and ends with no closed depression at all. Stitch the tiles\n   \
         and {:.1}% of the patch is in one — which means those depressions are a SEAM artifact of the\n   \
         tiling as much as they are geography. Both numbers are real; neither alone is the world.",
        100.0 * s.depression_cells as f32 / s.cells as f32
    );

    // ---- Part 7: the L9 bed the water nomos actually settles on ----
    // `water-tile` pulls `erosion-tile` at the build level and runs a relaxation
    // fill on it. Whether inland water can stand at all is first a question about
    // that bed's closed depressions, before it is a question about the fill.
    println!("\n-- Part 7: the L9 tiles — the bed `water-tile` settles on --");
    let l9 = world.observe().load_eroded_regions_where(|k| key_field(k, "level") == Some("9"));
    if l9.is_empty() {
        println!("   (no L9 tiles in this store)");
        return;
    }
    let sea9 = sea;
    let mut land_tiles = 0usize;
    let mut with_depressions = 0usize;
    let mut total_dep_cells = 0usize;
    let mut deepest = 0.0f32;
    let mut land_cells = 0usize;
    for r in &l9 {
        let subaerial = r.h.iter().filter(|&&h| h > sea9).count();
        if subaerial == 0 {
            continue;
        }
        land_tiles += 1;
        land_cells += subaerial;
        let d = surface_of(r, seed, true);
        if d.stats.depression_cells > 0 {
            with_depressions += 1;
        }
        total_dep_cells += d.stats.depression_cells;
        deepest = deepest.max(d.stats.deepest_depression_m);
    }
    println!(
        "   {} L9 tiles, {land_tiles} with any land ({land_cells} subaerial cells).\n   \
         tiles holding a closed depression deeper than 1 m: {with_depressions}   \
         total such cells: {total_dep_cells}   deepest anywhere: {deepest:.2} m",
        l9.len()
    );
    println!(
        "   Water stands where the bed has somewhere to hold it. If that count is zero, then zero\n   \
         inland standing water is not the water kernel failing to pond — it is the bed having no\n   \
         pond to fill, because Priority-Flood removes every depression each epoch and the tile edge\n   \
         is base level. The fill can only be as interesting as the surface underneath it."
    );
}
