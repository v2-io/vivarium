//! Scratch: do grown-craton overlaps exist? (deleted after use)
use vivarium_world::lithosphere as l;
use vivarium_world::sphere::{CubeCoord, Face};
fn main() {
    for seed in [0u64, 1, 7, 17425063241017297386] {
        for tp in [1550.0f64, 1450.0, 1350.0] {
            let n = 128usize;
            let (mut suture_cells, mut land_cells, mut smax) = (0usize, 0usize, 0.0f64);
            for fi in 0..6u8 {
                for j in 0..n { for i in 0..n {
                    let u = ((i as f64 + 0.5)/n as f64)*2.0 - 1.0;
                    let v = ((j as f64 + 0.5)/n as f64)*2.0 - 1.0;
                    let cell = CubeCoord{face: Face::from_index(fi), u, v}.cell(7);
                    let (w1, w2) = l::craton_weights_top2_at_tp(seed, cell, tp);
                    if w1 > 0.5 { land_cells += 1; }
                    if w2 > 0.05 { suture_cells += 1; smax = smax.max(w2); }
                }}
            }
            println!("seed {seed:>20} Tp {tp}: cratonized {land_cells:>5}  suture {suture_cells:>5}  max w2 {smax:.2}");
        }
    }
}
