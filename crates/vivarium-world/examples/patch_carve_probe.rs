//! One-shot: how much did the beacon patch's settle history carve? Reads the
//! live store read-only, compares the earliest and final stages per tile.
use vivarium_world::query::World;
use vivarium_world::store::Store;
use vivarium_world::watch::key_field;
fn main() {
    let home = std::env::var("HOME").unwrap();
    let dir = std::path::PathBuf::from(home).join(".cache/vivarium/globe-world");
    let store = Store::open_read_only(&dir).unwrap();
    let spec = vivarium_world::spec::WorldSpec::load(&dir).unwrap().unwrap();
    let world = World::new(&store, spec.seed);
    let b = spec.demand.beacon.expect("beacon set");
    let (mut carve_sum, mut carve_max, mut n) = (0.0f64, 0.0f32, 0usize);
    for tj in 0..b.tiles {
        for ti in 0..b.tiles {
            let (oi, oj) = (b.oi + ti * 64, b.oj + tj * 64);
            let at = |ep: u32| {
                let rs = world.observe().load_eroded_regions_where(|k| {
                    key_field(k, "level") == Some(&b.level.to_string())
                        && key_field(k, "oi") == Some(&oi.to_string())
                        && key_field(k, "oj") == Some(&oj.to_string())
                        && key_field(k, "epochs") == Some(&ep.to_string())
                });
                rs.into_iter().next()
            };
            if let (Some(a), Some(z)) = (at(b.stride), at(b.epochs)) {
                for (x, y) in a.h.iter().zip(z.h.iter()) {
                    let d = (x - y).abs();
                    carve_sum += d as f64;
                    carve_max = carve_max.max(d);
                    n += 1;
                }
            }
        }
    }
    println!("patch |h(stage {}) − h(stage {})|: mean {:.1} m, max {:.1} m over {} cells",
        b.epochs, b.stride, carve_sum / n.max(1) as f64, carve_max, n);
}
