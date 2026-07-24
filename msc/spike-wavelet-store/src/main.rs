//! # Spike — the multiresolution (wavelet) store
//!
//! Attacking `doc/theory/discretisation-and-information.md` §3.7 and
//! `DECISIONS[flux-on-the-face-makes-refluxing-an-invariant]`.
//!
//! Four claims, tested SEPARATELY because they can fail separately:
//!
//! 1. **Store the details ⇒ the seam never happens.**
//! 2. **The refinement criterion becomes information-theoretic** (refine where details are large).
//! 3. **The store compresses.**
//! 4. **It may retire `detail→abstract`** — an agent edit propagates up in exact $O(\log N)$.
//!
//! Every probe runs against the REAL kernels (`vivarium_world::{erosion, gen, sphere}`)
//! on a VERIFIED-LAND footprint, and every probe that could confirm the prior carries a
//! control that could kill it. (`ORIENTATION.md`: *a probe that cannot fail is not a
//! probe*, and be MORE suspicious of a result that confirms what you already believed.)

mod area;
mod mra;
mod probes;

use vivarium_world::planet::Planet;
use vivarium_world::sphere::Face;

/// The footprint. Quadtree-aligned (both origins are multiples of 512, hence of 128 = `NX`,
/// so the pyramid's parents are real quadtree parents all the way to the root) and — as of
/// the 2026-07-24 re-base — SUBAERIAL under the world's **derived** sea level (5106 m for
/// seed 0), interior to face ZPos (away from cube edges/corner shear), relief ~370 m.
///
/// **Why it moved:** the original (108_544, 186_368), like `seam_ridge`'s (108_500, 186_350),
/// classified as land only against the deprecated `SEA_LEVEL_M = 4000`. Once sea level became
/// *derived* (`sea_level::derived_sea_level_m`, ~5106 m) and the prior was resampled
/// (spine v2), that region is ~2 km of deep ocean and every erosion number there is a no-op
/// fabrication (`seam_ridge` now reads a fake ~1.70 on it). Re-based via `examples/land_scan`.
///
/// **The land guard is load-bearing** — `seam_ridge` reported a fabricated "22888" for
/// months because its footprint was entirely submarine and the fluvial kernel silently
/// no-op'd. `probes::assert_land` re-checks it here against the DERIVED sea level.
pub const FACE: Face = Face::ZPos;
pub const LEVEL: u8 = 19;
pub const OI: u64 = 135_168;
pub const OJ: u64 = 167_936;
pub const NX: usize = 128;
pub const RADIUS_M: f64 = Planet::EARTH.radius_m;

fn hdr(n: &str, t: &str) {
    println!("\n\n╔══════════════════════════════════════════════════════════════════════════════");
    println!("║ {n}  —  {t}");
    println!("╚══════════════════════════════════════════════════════════════════════════════");
}

fn main() {
    println!("SPIKE — the multiresolution (wavelet) store");
    println!("footprint: face {FACE:?} L{LEVEL} ({OI}, {OJ}) {NX}×{NX}   radius {RADIUS_M:.0} m");

    hdr("PROBE 0", "the land guard (a probe that cannot fail is not a probe)");
    probes::assert_land();

    hdr("PROBE 1", "AREA-ADDITIVITY — the geometric fact the whole design rests on");
    probes::area_additivity();

    hdr("PROBE 2", "EXACTNESS — perfect reconstruction + integral telescoping, on real eroded ground");
    probes::exactness();

    hdr("PROBE 3", "COMPRESSION — and does conservation survive lossy compression?");
    probes::compression();

    hdr("PROBE 4", "THE EDIT — O(log N) up-propagation, exact? And is it 'just the path to the root'?");
    probes::edit_propagation();

    hdr("PROBE 5", "NONLINEARITY — the Jensen commutator: does the LAW upscale, or only the STATE?");
    probes::nonlinearity();

    hdr("PROBE 6", "THE SEAM — decomposed. Does storing the details make it vanish?");
    probes::seam();

    hdr("PROBE 7", "THE REFLUXING INVARIANT — is flux-on-the-face the same design, and does it hold?");
    probes::refluxing_invariant();

    hdr("PROBE 8", "THE INCIDENTAL CATCH — cell_size_m is a uniform area, and erosion eats it");
    probes::cell_size_bias();

    println!("\n\n(verdicts in the spike's FINDINGS.md — this binary prints only measurements)\n");
}
