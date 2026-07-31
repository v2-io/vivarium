//! P0 stopwatch: cold body scan vs generation-hot roots_shared.
use std::time::Instant;
use vivarium_world::store::Store;

fn main() {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(dirs_next_or_home);
    let s = Store::open_read_only(&dir).expect("open");
    let t0 = Instant::now();
    let a = s.roots_shared().expect("roots");
    let cold = t0.elapsed();
    let t1 = Instant::now();
    for _ in 0..200 {
        let b = s.roots_shared().expect("roots");
        assert!(std::sync::Arc::ptr_eq(&a, &b));
    }
    let warm200 = t1.elapsed();
    let t2 = Instant::now();
    let _ = s.roots_invalidate_if_external().expect("external probe");
    let external = t2.elapsed();
    println!(
        "dir={dir}\nroots={}\ncold_scan={:?}\nwarm_200x={:?}\nwarm_avg={:?}\nexternal_readdir={:?}\ngen={}",
        a.len(),
        cold,
        warm200,
        warm200 / 200,
        external,
        s.generation()
    );
}

fn dirs_next_or_home() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{home}/.cache/vivarium/globe-world")
}
