//! Census of the independent vertical-state representations at one horizontal
//! address — a measurement instrument for `msc/spike-strata-primitive/`.
//!
//! For a sample of cells it evaluates every derivation of vertical information
//! the crate offers and reports whether they agree. Read-only: the store is
//! opened through `Store::open_read_only`, and nothing here computes a store
//! citizen.

use std::collections::BTreeMap;

struct Cov { nx: usize }

use vivarium_world::column::Column as StrataColumn;
use vivarium_world::lithosphere::{self, Column as LithoColumn};
use vivarium_world::sphere::{CellId, CubeCoord, Face};
use vivarium_world::store::Store;
use vivarium_world::watch::Coverage;
use vivarium_world::{erosion_return, gen, query::World, sea_level};

const TP: f64 = lithosphere::MANTLE_TP_C;

fn main() {
    let seed: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(17_425_063_241_017_297_386);
    let store_dir = std::env::args()
        .nth(2)
        .unwrap_or_else(|| format!("{}/.cache/vivarium/globe-world", std::env::var("HOME").unwrap()));

    println!("# vertical-state census   seed={seed}  T_p={TP}");
    println!("# SRC_HASH={}", vivarium_world::nomotheke::SRC_HASH);

    let sea_post = sea_level::derived_sea_level_m(seed);
    let sea_pre = sea_level::derived_sea_level_pre_ledger_at_tp(seed, TP);
    println!("\n## global scalars");
    println!("derived sea (post-ledger, live)   = {sea_post:.3} m");
    println!("derived sea (pre-ledger)          = {sea_pre:.3} m");
    println!("retired decreed SEA_LEVEL_M       = {:.3} m", gen::SEA_LEVEL_M);
    println!("live sea  -  decree               = {:.3} m", sea_post - gen::SEA_LEVEL_M);

    // ---- A. per-cell inventory at one address, at two levels -----------------
    println!("\n## A. every vertical quantity at a few addresses (level 9)");
    let probes: Vec<(&str, CellId)> = vec![
        ("mid-face ZPos", CubeCoord { face: Face::ZPos, u: 0.10, v: 0.10 }.cell(9)),
        ("mid-face XPos", CubeCoord { face: Face::XPos, u: -0.30, v: 0.40 }.cell(9)),
        ("mid-face YNeg", CubeCoord { face: Face::YNeg, u: 0.55, v: -0.20 }.cell(9)),
    ];
    for (label, cell) in &probes {
        report_cell(seed, *label, *cell, 9, sea_post, sea_pre);
    }

    // ---- B. strata column vs lithospheric column ----------------------------
    println!("\n## B. do the two `Column` types describe the same rock?");
    println!("cell                 strata_total_m  litho_crust_m  litho_keel_m  litho_sed_m  ratio_strata/litho");
    for (label, cell) in &probes {
        let s = gen::baseline_column(seed, *cell);
        let l: LithoColumn = lithosphere::column(seed, *cell);
        let litho_total = l.crust_m + l.keel_m + l.sediment_m;
        println!(
            "{label:<20} {:>14.1} {:>14.1} {:>13.1} {:>12.1} {:>19.6}",
            s.solid_thickness_m(),
            l.crust_m,
            l.keel_m,
            l.sediment_m,
            s.solid_thickness_m() / litho_total
        );
    }
    println!("(strata_total is height above the *strata* bedrock datum; litho_* are the");
    println!(" thicknesses the isostasy read integrates. Nothing relates the two data.)");

    // ---- C. is the strata column's surface the live surface? ----------------
    println!("\n## C. strata column surface vs the live tectonic surface");
    let mut max_abs = 0.0f64;
    for (label, cell) in &probes {
        let s = gen::baseline_column(seed, *cell);
        let live = sea_level::tectonic_surface_m(seed, *cell, 9);
        let d = s.solid_thickness_m() - live;
        max_abs = max_abs.max(d.abs());
        println!("{label:<20} strata={:.6}  live={:.6}  delta={:.3e}", s.solid_thickness_m(), live, d);
    }
    println!("max |delta| = {max_abs:.3e} m");

    // ---- D. level-dependence of the ledger's own land/sea classification ----
    // `erosion_return::crust_eroded_m` and `column_after_erosion` classify a cell
    // subaerial using `tectonic_surface_pre_ledger_at_tp(.., SAMPLE_LEVEL=8, ..)`
    // no matter what level the cell is. Any reader at another level classifies the
    // same cell with that level's bathymetry. Count the disagreements.
    println!("\n## D. ledger classification (fixed level 8) vs reader classification at level L");
    for level in [9u8, 13, 19] {
        let n = 96usize; // sample grid per face
        let mut flips = 0usize;
        let mut total = 0usize;
        let mut worst_surface_gap = 0.0f64;
        let mut sum_abs_gap = 0.0f64;
        for fi in 0..6u8 {
            let face = Face::from_index(fi);
            for j in 0..n {
                for i in 0..n {
                    let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let cell = CubeCoord { face, u, v }.cell(level);
                    let s8 = sea_level::tectonic_surface_pre_ledger_at_tp(seed, cell, 8, TP);
                    let sl = sea_level::tectonic_surface_pre_ledger_at_tp(seed, cell, level, TP);
                    let gap = sl - s8;
                    sum_abs_gap += gap.abs();
                    if gap.abs() > worst_surface_gap.abs() {
                        worst_surface_gap = gap;
                    }
                    if (s8 > sea_pre) != (sl > sea_pre) {
                        flips += 1;
                    }
                    total += 1;
                }
            }
        }
        println!(
            "level {level:>2}: {flips}/{total} cells ({:.3}%) disagree on subaerial-vs-submarine; \
             mean |surface gap| = {:.1} m, worst = {:+.1} m",
            100.0 * flips as f64 / total as f64,
            sum_abs_gap / total as f64,
            worst_surface_gap
        );
    }

    // ---- E. datum-difference water depth vs the settled kernel depth --------
    println!("\n## E. `gen` datum-difference water depth vs the built `water-tile` kernel depth");
    let store = match Store::open_read_only(&store_dir) {
        Ok(s) => s,
        Err(e) => {
            println!("(no store at {store_dir}: {e})");
            return;
        }
    };
    let world = World::new(&store, seed);
    let roots = store.roots().unwrap_or_default();
    println!("roots in store: {}", roots.len());
    let cov = Coverage::parse(&roots);
    println!("coverage: level={} nx={} watered_tiles={}", cov.level, cov.nx, cov.watered.len());

    // The Coverage census reports one level (the deepest surface tile), so a world
    // whose water sits at a shallower level reports zero watered tiles. Find the
    // current-source water roots directly instead.
    let src = vivarium_world::nomotheke::SRC_HASH;
    let mut direct: Vec<(u8, u8, u32, u32, usize, u32, u32)> = Vec::new();
    for r in &roots {
        if !r.key.starts_with("water-tile@") || !r.key.contains(&format!("src={src}")) {
            continue;
        }
        let g = |n: &str| -> Option<String> {
            r.key.split('|').find_map(|f| f.strip_prefix(&format!("{n}="))).map(|s| s.to_string())
        };
        if g("seed").as_deref() != Some(&seed.to_string()) {
            continue;
        }
        let (Some(f), Some(l), Some(oi), Some(oj), Some(nx), Some(ee), Some(st)) =
            (g("face"), g("level"), g("oi"), g("oj"), g("nx"), g("eepochs"), g("steps"))
        else {
            continue;
        };
        direct.push((
            f.parse().unwrap(),
            l.parse().unwrap(),
            oi.parse().unwrap(),
            oj.parse().unwrap(),
            nx.parse().unwrap(),
            ee.parse().unwrap(),
            st.parse().unwrap(),
        ));
    }
    direct.sort();
    let mut lvls: BTreeMap<u8, usize> = BTreeMap::new();
    for d in &direct {
        *lvls.entry(d.1).or_default() += 1;
    }
    println!("direct scan: {} water roots at src={src}, by level: {lvls:?}", direct.len());

    if direct.is_empty() {
        println!("(no water tiles at the current source cohort — nothing to compare)");
    } else {
        let mut compared = 0usize;
        let mut both_wet = 0usize;
        let mut datum_wet_kernel_dry = 0usize;
        let mut kernel_wet_datum_dry = 0usize;
        let mut sum_abs_diff = 0.0f64;
        let mut worst = (0.0f64, String::new());
        let mut bed_sum = 0.0f64;
        let mut surf_vs_bed = 0.0f64;
        let mut bed_n = 0usize;
        let mut tiles_reported = 0usize;
        let mut nonfinite_surf = 0usize;
        let mut nonfinite_bed = 0usize;
        let mut surf_bed_n = 0usize;
        let mut bad_tiles = 0usize;
        let mut bad_nan = 0usize;
        let mut bad_inf = 0usize;
        let mut all_cells = 0usize;
        let mut per_tile: BTreeMap<(u8, u32, u32), (usize, f64)> = BTreeMap::new();
        for &(f, level, oi, oj, nx, eepochs, steps) in direct.iter().take(24) {
            let face = Face::from_index(f);
            let Some((depth, _)) =
                world.observe().water_tile_hit(face, level, oi, oj, nx, eepochs, steps)
            else {
                continue;
            };
            let cov = Cov { nx };
            // The eroded bed this water actually settled on — so the comparison can
            // separate "the two readers disagree about the BED" from "they disagree
            // about the WATER".
            let (bed, bsrc, from_erosion) =
                world.observe().surface_prefer_eroded(face, level, oi, oj, nx, eepochs);
            {
                let nan = bed.iter().filter(|b| b.is_nan()).count();
                let inf = bed.iter().filter(|b| b.is_infinite()).count();
                if nan + inf > 0 {
                    println!("   [nonfinite] erosion tile f{f} ({oi},{oj}) eepochs={eepochs}: {nan} NaN, {inf} inf of {} cells", bed.len());
                    bad_tiles += 1;
                }
                bad_nan += nan; bad_inf += inf; all_cells += bed.len();
            }
            if tiles_reported < 3 {
                let nans = bed.iter().filter(|b| !b.is_finite()).count();
                println!(
                    "   [diag] tile f{f} ({oi},{oj}) nx={nx} eepochs={eepochs} steps={steps} \
bed.len={} nonfinite={} from_erosion={from_erosion} src={bsrc:?} bed[0]={:?} depth[0]={:?}",
                    bed.len(), nans, bed.first(), depth.first()
                );
                tiles_reported += 1;
            }
            let mut tile_n = 0usize;
            let mut tile_sum = 0.0f64;
            for dj in (0..cov.nx).step_by(4) {
                for di in (0..cov.nx).step_by(4) {
                    let cell = CellId::from_face_ij(face, oi + di as u32, oj + dj as u32, level);
                    // The datum-difference reading: exactly what
                    // `gen::column_from_surface_at_sea` puts in `Column::water_depth`.
                    let surf = sea_level::tectonic_surface_m(seed, cell, level);
                    let datum_depth = (sea_post - surf).max(0.0);
                    let kernel_depth = depth[dj * cov.nx + di] as f64;
                    if let Some(&b) = bed.get(dj * cov.nx + di) {
                        let bed_datum = (sea_post - b as f64).max(0.0);
                        bed_sum += (bed_datum - kernel_depth).abs();
                        if !surf.is_finite() { nonfinite_surf += 1; }
                        if !(b as f64).is_finite() { nonfinite_bed += 1; }
                        if surf.is_finite() && (b as f64).is_finite() {
                            surf_vs_bed += (surf - b as f64).abs();
                            surf_bed_n += 1;
                        }
                        bed_n += 1;
                    }
                    let diff = datum_depth - kernel_depth;
                    compared += 1;
                    sum_abs_diff += diff.abs();
                    tile_n += 1;
                    tile_sum += diff.abs();
                    let dw = datum_depth > 0.5;
                    let kw = kernel_depth > 0.5;
                    match (dw, kw) {
                        (true, true) => both_wet += 1,
                        (true, false) => datum_wet_kernel_dry += 1,
                        (false, true) => kernel_wet_datum_dry += 1,
                        (false, false) => {}
                    }
                    if diff.abs() > worst.0 {
                        worst = (
                            diff.abs(),
                            format!(
                                "face {f} ({},{}) datum={datum_depth:.1} kernel={kernel_depth:.1}",
                                oi + di as u32,
                                oj + dj as u32
                            ),
                        );
                    }
                }
            }
            if tile_n > 0 {
                per_tile.insert((f, oi, oj), (tile_n, tile_sum / tile_n as f64));
            }
        }
        if compared == 0 {
            println!("(water roots present but none loadable at the current source hash)");
        } else {
            println!("cells compared            : {compared}");
            println!("mean |datum - kernel|     : {:.1} m", sum_abs_diff / compared as f64);
            println!("worst                     : {}", worst.1);
            println!("both call it wet (>0.5 m) : {both_wet}");
            println!("datum wet, kernel dry     : {datum_wet_kernel_dry}");
            println!("kernel wet, datum dry     : {kernel_wet_datum_dry}");
            println!("tiles sampled             : {}", per_tile.len());
            println!("erosion tiles with nonfinite heights: {bad_tiles} of {} ({bad_nan} NaN + {bad_inf} inf of {all_cells} stored cells)", per_tile.len());
            if bed_n > 0 {
                println!("-- separating bed disagreement from water disagreement --");
                println!("nonfinite tectonic_surface_m samples      : {nonfinite_surf} of {bed_n}");
                println!("nonfinite eroded-bed samples              : {nonfinite_bed} of {bed_n}");
                println!("mean |tectonic surface - eroded bed|      : {:.1} m  (over {surf_bed_n} finite pairs)", surf_vs_bed / surf_bed_n.max(1) as f64);
                println!("mean |(sea - eroded bed) - kernel depth|  : {:.1} m", bed_sum / bed_n as f64);
            }
        }
    }

    // ---- F. the seed-dropping in `column_from_surface` ----------------------
    println!("\n## F. `gen::column_from_surface` hardcodes seed 0 for its waterline");
    let cell = CubeCoord { face: Face::ZPos, u: 0.1, v: 0.1 }.cell(9);
    let surf = sea_level::tectonic_surface_m(seed, cell, 9);
    let via_default = gen::column_from_surface(cell, surf, 2.0);
    let via_explicit = gen::column_from_surface_at_sea(cell, surf, 2.0, sea_post);
    println!("sea(seed 0)      = {:.3} m", sea_level::derived_sea_level_m(0));
    println!("sea(this world)  = {sea_post:.3} m");
    println!(
        "water_depth via column_from_surface (seed 0 datum) = {:.3} m",
        via_default.water_depth.value
    );
    println!(
        "water_depth via the world's own datum              = {:.3} m",
        via_explicit.water_depth.value
    );
    println!(
        "discrepancy                                        = {:.3} m",
        via_default.water_depth.value - via_explicit.water_depth.value
    );

    // ---- G. what the strata column actually carries ------------------------
    println!("\n## G. strata inventory of the live column (what a stratigraphic reader sees)");
    let c: StrataColumn = gen::baseline_column(seed, cell);
    println!("strata count = {}", c.strata.len());
    for (i, s) in c.strata.iter().enumerate() {
        println!(
            "  [{i}] material={:?} thickness={:.3} m saturation={:.3}",
            s.material,
            s.thickness.value,
            s.saturation
        );
    }
    println!("regolith r = {:.3} m", c.regolith_thickness_m());
    println!("water d    = {:.3} m", c.water_depth.value);

    // ---- H. post-ledger column at the same address -------------------------
    println!("\n## H. lithospheric column before / after the rock-mass ledger");
    let before = lithosphere::column(seed, cell);
    let after = erosion_return::column_after_erosion(seed, cell, TP);
    println!("before: crust={:.1} rho={:.1} keel={:.1} sed={:.1}", before.crust_m, before.crust_rho, before.keel_m, before.sediment_m);
    println!("after : crust={:.1} rho={:.1} keel={:.1} sed={:.1}", after.crust_m, after.crust_rho, after.keel_m, after.sediment_m);
    println!("crust eroded here = {:.3} m", erosion_return::crust_eroded_m(seed, cell, TP));
}

fn report_cell(seed: u64, label: &str, cell: CellId, level: u8, sea_post: f64, sea_pre: f64) {
    let bathy = gen::bathymetry_m(seed, cell, level);
    let fb_pre = lithosphere::freeboard_m(seed, cell);
    let fb_post = erosion_return::freeboard_after_erosion_m_at_tp(seed, cell, TP);
    let surf_pre = sea_level::tectonic_surface_pre_ledger_at_tp(seed, cell, level, TP);
    let surf_post = sea_level::tectonic_surface_m(seed, cell, level);
    let topo = gen::initial_topography_m(seed, cell, level);
    let strata = gen::baseline_column(seed, cell);
    println!("\n### {label}  (level {level})");
    println!("  bathymetry                       = {bathy:>12.3} m");
    println!("  freeboard  (pre-ledger)          = {fb_pre:>12.3} m");
    println!("  freeboard  (post-ledger)         = {fb_post:>12.3} m");
    println!("  tectonic surface (pre-ledger)    = {surf_pre:>12.3} m   [{}]", if surf_pre > sea_pre { "subaerial" } else { "submarine" });
    println!("  tectonic surface (post, LIVE)    = {surf_post:>12.3} m   [{}]", if surf_post > sea_post { "subaerial" } else { "submarine" });
    println!("  gen::initial_topography_m        = {topo:>12.3} m   (alias of the live surface)");
    println!("  strata solid_thickness_m         = {:>12.3} m", strata.solid_thickness_m());
    println!("  strata water_depth               = {:>12.3} m   (datum difference, no hydrology)", strata.water_depth.value);
}
