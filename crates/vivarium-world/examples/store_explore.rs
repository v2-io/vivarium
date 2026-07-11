//! Headless "explorer" — the principled scaffold made *visible*.
//!
//! Opens (or creates) a **vivium**: a directory whose `manifest` declares the
//! world's identity (seed) and label (name), and whose store IS the save. Walks
//! a short path of tiles, pulling each eroded tile through the [`World`] query
//! context, rendering it as ASCII, and printing whether it was COMPUTED fresh or
//! served as a HIT (persisted). Returning to a visited tile hits the store — the
//! exploration-parity property (matured state persists, no re-seed) shown
//! without a GPU. Delete the directory (printed at the end) to be dealt a fresh
//! world with a fresh seed next run; re-running keeps the same world.
//!
//! Run: `cargo run --release -p vivarium-world --example store_explore`

use vivarium_world::gen::SEA_LEVEL_M;
use vivarium_world::query::{Source, World};
use vivarium_world::spec::WorldSpec;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;

/// Print an elevation tile as ASCII: `~` below sea level, a shaded ramp above.
fn render(tile: &[f32], nx: usize) {
    const RAMP: &[u8] = b" .:-=+*#%@";
    let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
    for &h in tile {
        lo = lo.min(h);
        hi = hi.max(h);
    }
    let span = (hi - lo).max(1e-3);
    for j in 0..nx {
        let mut line = String::with_capacity(nx);
        for i in 0..nx {
            let h = tile[j * nx + i];
            let ch = if (h as f64) < SEA_LEVEL_M {
                '~'
            } else {
                let t = (((h - lo) / span) * (RAMP.len() - 1) as f32) as usize;
                RAMP[t.min(RAMP.len() - 1)] as char
            };
            line.push(ch);
        }
        println!("{line}");
    }
    println!("  relief {lo:.0}..{hi:.0} m (sea {SEA_LEVEL_M:.0} m)");
}

fn main() -> std::io::Result<()> {
    let dir = std::env::temp_dir().join("vivarium-store-explore");
    let spec = WorldSpec::load_or_create(&dir, "demo")?;
    let store = Store::open(&dir)?;
    let world = World::new(&store, spec.seed);
    println!("vivium \"{}\" — seed {} (identity; the name is just a label)", spec.name, spec.seed);

    let face = Face::from_index(2);
    let (level, nx, epochs) = (19u8, 48usize, 40u32);

    // A little walk: E, then back W to where we started. The return HITS.
    let path = [(2000u32, 3000u32), (2048, 3000), (2000, 3000)];
    for (n, &(oi, oj)) in path.iter().enumerate() {
        let (tile, src) = world.erosion_tile(face, level, oi, oj, nx, epochs);
        let tag = match src {
            Source::Computed => "COMPUTED fresh (miss → eroded → memoized)",
            Source::Hit => "HIT — served from the store (persisted, no re-seed)",
        };
        println!("\n=== step {n}: eroded tile at ({oi},{oj}), L{level} {nx}×{nx} — {tag} ===");
        render(&tile, nx);
    }

    println!("\nvivium lives at {} — copy it and the world moves; delete it for a new seed.", dir.display());
    Ok(())
}
