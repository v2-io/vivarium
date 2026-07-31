//! Prove: abutted 64-tiles leave a lattice of uncovered cells → prior fallback.
use vivarium_world::erosion::ErodedRegion;
use vivarium_world::sphere::{CellId, Face};

fn main() {
    let nx = 64usize;
    // Two abutted fake regions on face 0 at L9
    let a = ErodedRegion {
        face: Face::from_index(0),
        level: 9,
        oi: 0,
        oj: 0,
        nx,
        h: vec![100.0f32; nx * nx],
        seed: 1,
    };
    let b = ErodedRegion {
        face: Face::from_index(0),
        level: 9,
        oi: 64,
        oj: 0,
        nx,
        h: vec![200.0f32; nx * nx],
        seed: 1,
    };
    let regions = [a, b];
    let mut uncovered = 0usize;
    let mut a_only = 0usize;
    let mut b_only = 0usize;
    // scan face cells i=0..128, j=0 at L9
    for i in 0u32..128 {
        let cell = CellId::from_face_ij(Face::from_index(0), i, 0, 9);
        let ca = regions[0].covers(cell);
        let cb = regions[1].covers(cell);
        match (ca, cb) {
            (false, false) => {
                uncovered += 1;
                print!("{i} ");
            }
            (true, false) => a_only += 1,
            (false, true) => b_only += 1,
            (true, true) => {}
        }
    }
    println!();
    println!("L9 line j=0: uncovered={uncovered} a_only={a_only} b_only={b_only}");
    // At L13, count uncovered on a strip
    let mut u13 = 0usize;
    let mut tot = 0usize;
    for i in 0u32..2048 {
        for j in 0u32..4 {
            tot += 1;
            let cell = CellId::from_face_ij(Face::from_index(0), i, j, 13);
            if !regions[0].covers(cell) && !regions[1].covers(cell) {
                u13 += 1;
            }
        }
    }
    println!("L13 strip: uncovered={u13}/{tot} ({:.1}%)", 100.0 * u13 as f64 / tot as f64);
}
