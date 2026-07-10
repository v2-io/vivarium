//! Headless "explorer" — the principled scaffold made *visible*.
//!
//! Walks a short path of tiles, pulling each eroded tile through the store,
//! rendering it as ASCII, and printing whether it was COMPUTED fresh or served
//! as a HIT (persisted). Returning to a tile you've already visited hits the
//! store — the exploration-parity property (matured state persists, no re-seed)
//! shown without a GPU.
//!
//! Run: `cargo run --release -p vivarium-world --example store_explore`

use vivarium_world::gen::SEA_LEVEL_M;
use vivarium_world::query::{erosion_tile, Source};
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
    let _ = std::fs::remove_dir_all(&dir);
    let store = Store::open(&dir)?;

    let face = Face::from_index(2);
    let (level, nx, epochs) = (19u8, 48usize, 40u32);

    // A little walk: E, then back W to where we started. The return HITS.
    let path = [(2000u32, 3000u32), (2048, 3000), (2000, 3000)];
    for (n, &(oi, oj)) in path.iter().enumerate() {
        let (tile, src) = erosion_tile(&store, face, level, oi, oj, nx, epochs);
        let tag = match src {
            Source::Computed => "COMPUTED fresh (miss → eroded → memoized)",
            Source::Hit => "HIT — served from the store (persisted, no re-seed)",
        };
        println!("\n=== step {n}: eroded tile at ({oi},{oj}), L{level} {nx}×{nx} — {tag} ===");
        render(&tile, nx);
    }

    println!("\nstore lives at {} — copy it and the world moves.", dir.display());
    Ok(())
}
