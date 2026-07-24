//! # Spike — cross-face seam machinery
//!
//! Turning two "designed, unbuilt" rows into "spiked, measured":
//!  - `form-cellid-chunk-patch` FE(4): cross-cube-face halo fill with the face
//!    axis transform (`chunk.rs::fill` leaves out-of-face halo at default).
//!  - `form-face-flux-register`: the single-valued face flux as a real
//!    `store.rs` citizen (production store/kernels do not yet face-key fluxes).
//!
//! Every probe runs against the REAL substrate
//! (`vivarium_world::{sphere, measure, store}`); nothing is re-implemented.
//! Predictions were committed in `PREDICTIONS.md` before the first run
//! (`#norm-probe-sensitivity`: known-bad first; state the discrimination).
//!
//! This binary prints only MEASUREMENTS. Predictions were pre-registered in
//! `PREDICTIONS.md`; verdicts were delivered in the spike's landing report to
//! the coordinator (2026-07-24) — fold them into a core segment on graduation.

mod faceflux;
mod probes;
mod seam;

fn hdr(n: &str, t: &str) {
    println!("\n\n╔══════════════════════════════════════════════════════════════════════════════");
    println!("║ {n}  —  {t}");
    println!("╚══════════════════════════════════════════════════════════════════════════════");
}

fn main() {
    println!("SPIKE — cross-face seam machinery (halo transform + face-keyed flux)");
    println!("seam under test: ZPos east edge (u→+1) re-homes to XPos; radius 6371000 m");

    hdr("PROBE 0", "the seam is a genuine cube-face boundary (a guard that can fail)");
    probes::guard();

    hdr("PROBE 1", "the face axis transform is LATENT in the projection (depth-1 involution + adjacency)");
    probes::transform_latent();

    hdr("PROBE 2", "halo DEPTH — where the two cross-face grids stop corresponding");
    probes::halo_depth();

    hdr("PROBE 3", "face-flux identity: canonical, symmetric across faces, a real store citizen");
    probes::flux_identity_store();

    hdr("PROBE 4a", "CONSERVATION — today's default-0 out-of-face halo is a mass SINK at the seam");
    probes::conservation_default_zero();

    hdr("PROBE 4b", "CONSERVATION — matched same-level cross-face seam: TwoSided vs SingleValued");
    probes::conservation_matched_seam();

    hdr("PROBE 4c", "CONSERVATION — cross-face HANGING NODE (PROBE 7 lifted onto a cube edge)");
    probes::conservation_cross_face_hanging();

    hdr("PROBE 5", "CONTINUITY, known-bad first — default-0 plateaus, filled halo vanishes with the arc");
    probes::continuity_scale();

    println!("\n\n(predictions pre-registered in PREDICTIONS.md; verdicts in the landing report — this binary prints only measurements)\n");
}
