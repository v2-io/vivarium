//! Calibration probe for the nucleation-growth cratonization field
//! (`#form-isostasy-column` FE(8) stand-in inventory): prints the craton-weight
//! histogram and the cratonized area fraction (w > 0.5) at the present-Abyssal
//! anchor across all six faces. The declared target is `CRATON_AREA_FRAC`;
//! `lithosphere::tests::craton_fraction_is_in_band` convicts drift. Recorded
//! 2026-07-24 (nucleation-growth): seed 0 ~0.093, seed 1 ~0.104, seed 7 ~0.131
//! (all in band). For the shape class, see `craton_morphology_probe`.
fn main(){
    use vivarium_world::lithosphere::*;
    use vivarium_world::sphere::{CubeCoord, Face};
    let mut hist=[0usize;11];
    let n=96usize;
    for fi in 0..6u8 { let face=Face::from_index(fi);
        for j in 0..n { for i in 0..n {
            let u=((i as f64+0.5)/n as f64)*2.0-1.0; let v=((j as f64+0.5)/n as f64)*2.0-1.0;
            let c=CubeCoord{face,u,v}.cell(7);
            let w=craton_weight(0,c); hist[(w*10.0) as usize]+=1;
        }}}
    println!("{:?}", hist);
    println!("craton frac (w>0.5): {:.4}", hist[6..].iter().sum::<usize>() as f64/(6*n*n) as f64);
}
