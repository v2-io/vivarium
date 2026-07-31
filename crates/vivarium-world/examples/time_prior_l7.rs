//! One-shot: how long is L7 pure prior + ocean mask for 6 faces?
fn main() {
    let seed = 17425063241017297386u64;
    let level = 7u8;
    let nx = 1usize << level;
    let t0 = std::time::Instant::now();
    let mut cells = 0usize;
    for f in 0..6u8 {
        let face = vivarium_world::sphere::Face::from_index(f);
        for j in 0..nx as u32 {
            for i in 0..nx as u32 {
                let c = vivarium_world::sphere::CellId::from_face_ij(face, i, j, level);
                let _ = vivarium_world::gen::initial_topography_m(seed, c, level);
                cells += 1;
            }
        }
    }
    let t1 = t0.elapsed();
    println!("prior L7 6 faces: {cells} cells in {:.3}s ({:.1} µs/cell)", t1.as_secs_f64(), t1.as_secs_f64()*1e6/cells as f64);
}
