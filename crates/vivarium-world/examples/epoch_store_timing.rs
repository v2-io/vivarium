//! Epoch-reduction store citizen — the cross-process warm-time payoff.
//!
//! The mantle-thermal cooling chain's per-epoch global scalars (derived sea +
//! rock-mass-ledger integrals) are now store citizens under complete keys
//! (`#form-store-as-save` FE(6), decided: memoized ≡ store object). The cost of
//! warming an epoch belongs at build time; a *fresh process* that Hits the store
//! never runs the ~393k-cell pour or ledger passes at all.
//!
//! Run twice against the SAME store dir to see the cross-process collapse (the
//! static staging caches reset between process invocations, so run 2 is a true
//! cold-process measurement):
//!
//! ```text
//! cargo run --release --example epoch_store_timing -- /tmp/vv-epoch   # MISS: cold pour
//! cargo run --release --example epoch_store_timing -- /tmp/vv-epoch   # HIT:  store round-trip
//! ```
//!
//! Run 1 computes and persists; run 2 (a genuinely fresh process) warms every
//! epoch from disk — the "second process start-to-warm" number that was the point.

use std::time::Instant;

use vivarium_world::lithosphere::MANTLE_TP_C;
use vivarium_world::mantle_thermal::{cooling_stages, potential_temp_c};
use vivarium_world::query::World;
use vivarium_world::store::Store;

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| "/tmp/vivarium-epoch-timing".into());
    let seed: u64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(7);

    let store = match Store::open(&dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot open store at {dir}: {e}");
            std::process::exit(1);
        }
    };
    let world = World::new(&store, seed);

    // Present epoch first, then the abyssal cooling chain (the same ladder the
    // builder materializes and the globe warmer walks).
    let mut tps: Vec<f64> = vec![MANTLE_TP_C];
    tps.extend(cooling_stages().iter().map(|&t| potential_temp_c(t)));

    let t0 = Instant::now();
    let (mut hit, mut computed) = (0usize, 0usize);
    for &tp in &tps {
        let (_r, src) = world.epoch_reduction(tp);
        if src.is_hit() {
            hit += 1;
        } else {
            computed += 1;
        }
    }
    let dt = t0.elapsed();

    println!(
        "epoch ladder: {} pulls warmed in {dt:.2?}  [{hit} hit / {computed} computed]  (seed {seed}, store {dir})",
        tps.len()
    );
    if computed > 0 {
        println!("  → cold process: paid the pour/ledger. Run again on the same dir — every epoch HITs.");
    } else {
        println!("  → warm process: every epoch served from the store, no pour. This is the payoff.");
    }
}
