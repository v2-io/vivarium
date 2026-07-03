//! Load a real fill-cache file and test the pawn-float sampling chain on it:
//! deepest cells -> depth_m at an L24 cell over them -> the bob math.
use vivarium_world::sphere::{CellId, Face};
use vivarium_world::water::WaterRegion;

fn take<'a>(inp: &mut &'a [u8], n: usize) -> &'a [u8] {
    let (a, b) = inp.split_at(n);
    *inp = b;
    a
}
fn f32s(inp: &mut &[u8]) -> Vec<f32> {
    let n = u64::from_le_bytes(take(inp, 8).try_into().unwrap()) as usize;
    (0..n).map(|_| f32::from_le_bytes(take(inp, 4).try_into().unwrap())).collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: float_probe <fill-cache.bin>");
    let bytes = std::fs::read(path).unwrap();
    let mut inp = bytes.as_slice();
    assert_eq!(take(&mut inp, 8), b"VIVWF001");
    let ntiers = u32::from_le_bytes(take(&mut inp, 4).try_into().unwrap());
    for _ in 0..ntiers {
        take(&mut inp, 2);
        take(&mut inp, 8);
        f32s(&mut inp);
    }
    let level = take(&mut inp, 1)[0];
    let oi = u32::from_le_bytes(take(&mut inp, 4).try_into().unwrap());
    let oj = u32::from_le_bytes(take(&mut inp, 4).try_into().unwrap());
    let _cell = f32::from_le_bytes(take(&mut inp, 4).try_into().unwrap());
    take(&mut inp, 16);
    let bed = f32s(&mut inp);
    let depth = f32s(&mut inp);
    let nx = (bed.len() as f64).sqrt() as usize;
    println!("water: L{level} origin ({oi},{oj}) {nx}x{nx}");
    let z = vec![0.0f32; nx * nx];
    let wr = WaterRegion { face: Face::ZPos, level, oi, oj, nx, depth: depth.clone(), bed, sediment: z.clone(), sed_bed: z.clone(), colmation: z.clone(), armor: z.clone(), groundwater: z.clone(), vx: z.clone(), vy: z };
    // Deepest five cells; test the pawn chain over each.
    let mut idx: Vec<usize> = (0..nx * nx).collect();
    idx.sort_by(|&a, &b| depth[b].partial_cmp(&depth[a]).unwrap());
    for &k in idx.iter().take(5) {
        let (i, j) = ((k % nx) as u32, (k / nx) as u32);
        // An L24 cell over the centre of this L21 cell (scale 8).
        let c24 = CellId::from_face_ij(Face::ZPos, (oi + i) * 8 + 4, (oj + j) * 8 + 4, 24);
        let d = wr.depth_m(c24);
        let y = d.map(|d| if d >= 1.05 { ((d as f32 - 1.35) + 1.0).max(1.0) } else { 1.0 });
        println!("cell ({i},{j}) raw depth {:.2} m -> depth_m(L24 centre) {:?} -> pawn y {:?}", depth[k], d, y);
    }
    // And how much of the world is float-deep at all?
    let deep = depth.iter().filter(|&&d| d >= 1.05).count();
    println!("cells >= 1.05 m: {deep} / {} ({:.2}%)", nx * nx, deep as f64 / (nx * nx) as f64 * 100.0);
}
