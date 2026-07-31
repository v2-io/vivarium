// quick via existing example style
use vivarium_world::store::Store;
use vivarium_world::watch::{key_field, Coverage};
use vivarium_world::query::World;
use vivarium_world::nomotheke::SRC_HASH;

fn main() {
    let dir = std::env::var("HOME").unwrap() + "/.cache/vivarium/globe-world";
    let s = Store::open_read_only(&dir).unwrap();
    let roots = s.roots_shared().unwrap();
    let mut water_by_level = std::collections::BTreeMap::new();
    let mut water_fresh = 0usize;
    for r in roots.iter() {
        if !r.key.starts_with("water-tile@") { continue; }
        let Some(l) = key_field(&r.key, "level").and_then(|v| v.parse::<u8>().ok()) else { continue };
        *water_by_level.entry(l).or_insert(0usize) += 1;
        if key_field(&r.key, "src") == Some(SRC_HASH) { water_fresh += 1; }
    }
    println!("SRC_HASH={}", &SRC_HASH[..8.min(SRC_HASH.len())]);
    println!("water_by_level={water_by_level:?}");
    println!("water_fresh_src={water_fresh}");
    let cov = Coverage::parse(&roots);
    println!("cov.level={} cov.watered={} cov.erosion={}", cov.level, cov.watered.len(), cov.erosion.len());
    let w = World::new(&s, 17425063241017297386);
    // try hit one L9 water if any
    if let Some((&(f,oi,oj), &(ee,st))) = cov.watered.iter().next() {
        println!("cov first water face={f} oi={oi} oj={oj} ee={ee} st={st}");
    }
}
