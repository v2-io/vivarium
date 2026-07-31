use vivarium_world::nomotheke::SRC_HASH;
use vivarium_world::query::World;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;
use vivarium_world::watch::key_field;

fn main() {
    let dir = format!("{}/.cache/vivarium/globe-world", std::env::var("HOME").unwrap());
    let store = Store::open_read_only(&dir).unwrap();
    let seed = 17425063241017297386u64;
    let world = World::new(&store, seed);
    let roots = store.roots_shared().unwrap();
    let sea = vivarium_world::sea_level::derived_sea_level_m(seed) as f32;
    let mut inland_wet = 0usize;
    let mut ocean_wet = 0usize;
    let mut tiles = 0usize;
    let mut max_inland = 0.0f32;
    for r in roots.iter() {
        if !r.key.starts_with("water-tile@") { continue; }
        if key_field(&r.key, "src") != Some(SRC_HASH) { continue; }
        if key_field(&r.key, "level") != Some("9") { continue; }
        let face = key_field(&r.key, "face").and_then(|v| v.parse::<u8>().ok()).unwrap();
        let oi = key_field(&r.key, "oi").and_then(|v| v.parse::<u32>().ok()).unwrap();
        let oj = key_field(&r.key, "oj").and_then(|v| v.parse::<u32>().ok()).unwrap();
        let nx = key_field(&r.key, "nx").and_then(|v| v.parse::<usize>().ok()).unwrap();
        let ee = key_field(&r.key, "eepochs").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        let st = key_field(&r.key, "steps").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        let Some((depth, _)) = world.observe().water_tile_hit(Face::from_index(face), 9, oi, oj, nx, ee, st) else { continue };
        // need bed for ocean mask - use object from erosion? skip ocean class, count any depth > 0.5
        // load bed from water's companion - actually classify via derived: if depth>0.5 and we don't know ocean...
        // Simpler: count depth > 0.5; ocean cells also have depth. Sample: depth>0.5 on cells where we'd need bed.
        tiles += 1;
        for &d in &depth {
            if d > 0.5 {
                // approximate: if d is large and near sea freeboard, could be ocean
                if d > 50.0 { ocean_wet += 1; } else { inland_wet += 1; max_inland = max_inland.max(d); }
            }
        }
        if tiles >= 32 { break; } // sample first 32 tiles
    }
    println!("sampled_water_tiles={tiles} cells_depth>50={ocean_wet} cells_0.5..50={inland_wet} max_mid={max_inland}");
    println!("src={}", &SRC_HASH[..8]);
}
