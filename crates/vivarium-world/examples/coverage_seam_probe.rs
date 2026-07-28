//! Coverage / staleness diagnostic for the whole-globe assemble path.
//!
//! Measures, on the world at $VIVARIUM_WORLD, at a chosen level:
//!   1. Root census: erosion-tile roots total / fresh (src == current SRC_HASH) / stale.
//!   2. Prior-fallback fraction of assembled cells (tier_at == None) and whether the
//!      covered↔fallback boundaries carry a visible elevation step (the ribbon signal).
//!
//! Run: VIVARIUM_WORLD=/path cargo run --release -p vivarium-world --example coverage_seam_probe -- [level]

use std::io::Write;
use std::time::Instant;

use vivarium_world::nomotheke::SRC_HASH;
use vivarium_world::query::World;
use vivarium_world::sphere::{CellId, Face};
use vivarium_world::store::Store;
use vivarium_world::{erosion, sea_level};

fn key_field<'a>(key: &'a str, name: &str) -> Option<&'a str> {
    let pfx = format!("{name}=");
    key.split('|').find_map(|f| f.strip_prefix(&pfx))
}

fn main() {
    let dir = std::env::var("VIVARIUM_WORLD").expect("set VIVARIUM_WORLD");
    let level: u8 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(7);
    let t0 = Instant::now();
    let store = Store::open(&dir).expect("open store");
    let spec = vivarium_world::spec::WorldSpec::load_or_create(std::path::Path::new(&dir), "probe").unwrap();
    let world = World::new(&store, spec.seed);
    let seed = spec.seed;
    eprintln!("[t {:?}] opened; computing sea level...", t0.elapsed());
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    eprintln!("[t {:?}] sea = {sea:.0} m", t0.elapsed());

    println!("world dir {dir}");
    println!("seed {seed:016x} | level {level} | current SRC_HASH {SRC_HASH} | sea {sea:.0} m");

    // 1. Root census.
    let roots = store.roots().unwrap();
    let (mut total, mut fresh, mut stale) = (0usize, 0usize, 0usize);
    for r in &roots {
        if !r.key.starts_with("erosion-tile@") {
            continue;
        }
        total += 1;
        match key_field(&r.key, "src") {
            Some(h) if h == SRC_HASH => fresh += 1,
            _ => stale += 1,
        }
    }
    println!("[census] erosion-tile roots: {total} total | {fresh} fresh (src==current) | {stale} STALE (src!=current)");

    // The ribbon-fault probe NEEDS the cross-cohort read it convicts — the
    // sharp predicate path expresses it explicitly (the convenient merging
    // loader is gone; #norm-caught-disciplines-become-mechanisms FE(2)(a)).
    let regions = world.load_eroded_regions_where(|_| true);
    println!(
        "[assemble] cross-cohort predicate returned {} regions (NO src filter — stale included, deliberately)",
        regions.len()
    );
    eprintln!("[t {:?}] regions loaded", t0.elapsed());

    let nx = 1usize << level;
    let mut cells_total = 0usize;
    let mut cells_fallback = 0usize;
    let mut fallback_land = 0usize;
    let (mut bnd_sum, mut bnd_n, mut bnd_max) = (0f64, 0usize, 0f32);
    let (mut int_sum, mut int_n) = (0f64, 0usize);

    for f in 0..6u8 {
        let face = Face::from_index(f);
        let (tile, _any) = world.assemble_surface_tile(face, level, 0, 0, nx, &regions);
        // coverage mask in one pass
        let mut covered = vec![false; nx * nx];
        for j in 0..nx {
            for i in 0..nx {
                let cell = CellId::from_face_ij(face, i as u32, j as u32, level);
                covered[j * nx + i] = erosion::tier_at(cell, &regions).is_some();
            }
        }
        for j in 0..nx {
            for i in 0..nx {
                let a = j * nx + i;
                cells_total += 1;
                if !covered[a] {
                    cells_fallback += 1;
                    if tile[a] > sea {
                        fallback_land += 1;
                    }
                }
                if i + 1 < nx {
                    let b = a + 1;
                    let dh = (tile[a] - tile[b]).abs();
                    if covered[a] != covered[b] {
                        bnd_sum += dh as f64;
                        bnd_n += 1;
                        bnd_max = bnd_max.max(dh);
                    } else if covered[a] {
                        int_sum += dh as f64;
                        int_n += 1;
                    }
                }
            }
        }
        eprintln!("[t {:?}] face {f} done", t0.elapsed());
    }
    let ffrac = cells_fallback as f64 / cells_total as f64 * 100.0;
    println!("[coverage] {cells_fallback}/{cells_total} cells = {ffrac:.2}% prior-fallback (tier_at None)");
    println!("[coverage] fallback cells above sea (edge-land candidates): {fallback_land}");
    let bnd_mean = if bnd_n > 0 { bnd_sum / bnd_n as f64 } else { 0.0 };
    let int_mean = if int_n > 0 { int_sum / int_n as f64 } else { 0.0 };
    println!(
        "[ribbon] boundary |dh| mean {bnd_mean:.0} m (max {bnd_max:.0}, n={bnd_n}) vs interior covered |dh| mean {int_mean:.0} m (n={int_n})"
    );
    let ratio = if int_mean > 0.0 { bnd_mean / int_mean } else { f64::INFINITY };
    println!("[ribbon] boundary/interior ratio {ratio:.2}x (>~1 = coverage boundaries carry a visible step)");
    std::io::stdout().flush().ok();
}
