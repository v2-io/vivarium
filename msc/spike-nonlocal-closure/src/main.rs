//! # Spike — the nonlinear closure for a NON-LOCAL flux
//!
//! Attacking `#sketch-detail-abstract-reversion` FE(5): the live fluvial kernel
//! `E` does not commute with restriction `R`. `msc/spike-wavelet-store` PROBE 5
//! measured `‖R∘E − E∘R‖` signed mean **+5.34 m** (a bias the size of the
//! physics) and — decisively — `corr(local h-detail RMS, |commutator|) = −0.027`:
//! **no local statistic predicts where the law fails.**
//!
//! The hypothesis under test: the failure is carried by the **drainage graph**.
//! Stream-power incision `f = k·Aᵐ/dist ; h' = (h + f·h_r)/(1+f)` is driven by
//! `A`, the MFD-accumulated drainage area — a **non-local** quantity whose flow
//! network changes *topologically* under coarsening. Following
//! `#detail-info-theoretic-discretisation` FE(4), the commutator has two stacked
//! sources: **(T)** the drainage-area operator itself not commuting with `R`
//! (topological, non-local), and **(J)** pointwise concavity of `Aᵐ` (Jensen,
//! `m=0.5`) plus `Cov(Aᵐ,S)`.
//!
//! Every probe runs against the REAL kernel and carries a control that could
//! kill the hypothesis. Predictions are frozen in `PREDICTIONS.md` (house law).

mod area;
mod mra;
mod probes;

use vivarium_world::planet::Planet;
use vivarium_world::sphere::Face;

// VERIFIED-LAND footprint on the CURRENT generator (found by `--scan`). The
// wavelet spike's old footprint (ZPos 108544,186368) went submarine when the
// generator/sea-level moved — its +5.34 baseline is from a world that no longer
// exists, so P0 re-anchors here rather than chasing that number. Interior,
// 100% land, ~218 m relief, quadtree-aligned for depth-2 restriction.
pub const FACE: Face = Face::ZNeg;
pub const LEVEL: u8 = 19;
pub const OI: u64 = 327_680;
pub const OJ: u64 = 65_536;
pub const NX: usize = 128;
pub const RADIUS_M: f64 = Planet::EARTH.radius_m;

fn hdr(n: &str, t: &str) {
    println!("\n\n╔══════════════════════════════════════════════════════════════════════════════");
    println!("║ {n}  —  {t}");
    println!("╚══════════════════════════════════════════════════════════════════════════════");
}

fn main() {
    if std::env::args().any(|a| a == "--scan") {
        probes::scan_for_land();
        return;
    }
    println!("SPIKE — the nonlinear closure for a non-local flux");
    println!("footprint: face ZPos L{LEVEL} ({OI}, {OJ}) {NX}×{NX}   radius {RADIUS_M:.0} m\n");

    hdr("PROBE 0", "ANCHOR — reproduce the +5.34 m baseline (else my construction is wrong)");
    probes::anchor();

    hdr("PROBE 1", "THE DRAINAGE-AREA COMMUTATOR — does A itself fail to commute, and does it predict?");
    probes::drainage_commutator();

    hdr("PROBE 2", "SAME FIELD, TWO STATISTICS — non-local A-variance vs local h-variance");
    probes::two_statistics();

    hdr("PROBE 3", "THE ORACLE-A CLOSURE — feed the coarse kernel the correct drainage, co-evolved");
    probes::oracle_closure();

    hdr("PROBE 4", "THE POINTWISE RESIDUE — single-epoch Jensen + Cov, analytic vs measured");
    probes::pointwise_jensen();

    hdr("PROBE 5", "ROBUSTNESS — does the trunk-oracle collapse the bias across tiles & depths?");
    probes::robustness();

    hdr("PROBE 6/7", "THE DEPLOYABLE COARSE-ONLY CLOSURE — recalibrate A←α·Aᵝ, held-out");
    probes::deployable_closure();

    hdr("PROBE 8", "HARDEN THE CEILING — R² of the best pointwise fit, second seed");
    probes::harden_ceiling();
}
