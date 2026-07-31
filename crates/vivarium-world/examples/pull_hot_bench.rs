//! P1 stopwatch: costs that used to re-pay every "updating view…".
use std::sync::Arc;
use std::time::Instant;
use vivarium_world::query::World;
use vivarium_world::store::Store;
use vivarium_world::watch::Coverage;

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{home}/.cache/vivarium/globe-world")
    });
    let store = Store::open_read_only(&dir).expect("open");
    let seed = 1u64; // unused for store-only paths
    let world = World::new(&store, seed);

    let t0 = Instant::now();
    let roots = store.roots_shared().expect("roots");
    println!("roots_shared cold: {:?} (n={})", t0.elapsed(), roots.len());

    let t1 = Instant::now();
    for _ in 0..50 {
        let _ = store.roots_shared().unwrap();
    }
    println!("roots_shared ×50 warm: {:?}", t1.elapsed());

    let t2 = Instant::now();
    let cov = Coverage::parse(&roots);
    println!("Coverage::parse: {:?} (erosion tiles={})", t2.elapsed(), cov.erosion.len());

    let t3 = Instant::now();
    let _ = Coverage::parse(&roots);
    println!("Coverage::parse again: {:?}", t3.elapsed());

    let t4 = Instant::now();
    let census = world.observe().eroded_region_census();
    println!(
        "eroded_region_census (shared roots): {:?} fresh={} stale={}",
        t4.elapsed(),
        census.fresh,
        census.stale
    );

    let cur = vivarium_world::nomotheke::SRC_HASH;
    let view_level = 7u8;
    let t5 = Instant::now();
    let regions = world.observe().load_eroded_regions_where(|k| {
        vivarium_world::watch::key_field(k, "src") == Some(cur)
            && vivarium_world::watch::key_field(k, "level")
                .and_then(|v| v.parse::<u8>().ok())
                .is_some_and(|l| l <= view_level)
    });
    println!(
        "load_eroded L≤7 current src: {:?} (regions={})",
        t5.elapsed(),
        regions.len()
    );

    let t6 = Instant::now();
    let _ = world.observe().load_eroded_regions_where(|k| {
        vivarium_world::watch::key_field(k, "src") == Some(cur)
            && vivarium_world::watch::key_field(k, "level")
                .and_then(|v| v.parse::<u8>().ok())
                .is_some_and(|l| l <= view_level)
    });
    println!("load_eroded L≤7 again (no cache in observe): {:?}", t6.elapsed());

    // Clone cost that P1 removed from the hot path
    let t7 = Instant::now();
    let _cloned: Vec<_> = roots.as_ref().clone();
    println!("clone full roots Vec: {:?}", t7.elapsed());

    let _ = Arc::clone(&roots);
}
