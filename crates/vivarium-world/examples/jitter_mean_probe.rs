//! The "exact global closure" probe climate.rs asked for: measures the
//! precip-jitter factor's spatial mean / min / max across faces and seeds.
//! Recorded 2026-07-24: mean drift +0.1%..+3.0% per seed; range within
//! [0.5, 1.5) (clip inert). Exactness on the PRECIPITATION promise stays
//! `Approximate` until domain normalization exists.
fn main(){
    use vivarium_world::climate::precip_jitter_factor;
    use vivarium_world::sphere::{CellId, Face};
    for seed in [0u64, 7, 42] {
        let mut min=f64::MAX; let mut max=f64::MIN; let mut sum=0.0; let mut n=0u64;
        for f in [Face::XPos, Face::YNeg, Face::ZPos] {
            for i in (0..4096u32).step_by(16) { for j in (0..4096u32).step_by(16) {
                let v = precip_jitter_factor(seed, CellId::from_face_ij(f, i, j, 12));
                min=min.min(v); max=max.max(v); sum+=v; n+=1;
            }}
        }
        println!("seed {seed}: mean {:.6}  min {:.4}  max {:.4}  n {n}", sum/n as f64, min, max);
    }
}
