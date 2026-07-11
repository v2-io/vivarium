//! Baseline worldgen — turn a [`CellId`] into a [`Column`] (the abstract→detail
//! *easy* direction, `doc/design/DESIGN-REDUX.md` §11, and a fidelity-ladder bottom rung §12).
//!
//! [`column_from_surface`] is the **reusable assembler**: given a solid surface
//! height it builds the stratigraphy. It is *not* throwaway — the ported erosion
//! tier will feed it too; only the *source of the height* moves up the ladder.
//! [`baseline_column`] is the crude interim height from coordinate noise
//! (deterministic, pure function of the cell — §8), to be superseded by the
//! ported erosion/hydrology.

use crate::column::{Column, Stratum};
use crate::material::{Category, Kind, MaterialId};
use crate::noise::fbm3;
use crate::quantity::{Quantity, Unit};
use crate::sphere::CellId;

/// Sea-level datum, metres above the bedrock datum (`y = 0`). Provisional — a
/// coarse global reference (see the ~20 km shell, `doc/design/DESIGN-MATERIAL.md` §8); a
/// later tier may make it a proper planetary constant.
pub const SEA_LEVEL_M: f64 = 4000.0;

/// Assemble a stratigraphic column from a solid **surface height** (m above the
/// bedrock datum) and a soil mantle thickness. Bedrock (undifferentiated igneous —
/// refine later, §6) fills up to `surface − soil`; a soil stratum tops it (saturated
/// below sea level); standing water fills to [`SEA_LEVEL_M`] when the surface is
/// below it. Reusable by any tier that produces a surface height.
pub fn column_from_surface(cell: CellId, surface_m: f64, soil_m: f64) -> Column {
    let surface_m = surface_m.max(0.0);
    let soil = soil_m.clamp(0.0, surface_m);
    let bedrock = surface_m - soil;

    let mut strata = Vec::new();
    if bedrock > 0.0 {
        strata.push(Stratum::new(MaterialId::Undifferentiated(Category::Igneous), bedrock, 0.0));
    }
    if soil > 0.0 {
        let saturation = if surface_m < SEA_LEVEL_M { 1.0 } else { 0.3 };
        strata.push(Stratum::new(MaterialId::Kind(Kind::Soil), soil, saturation));
    }
    let water = (SEA_LEVEL_M - surface_m).max(0.0);
    Column { tile: cell, strata, water_depth: Quantity::exact(water, Unit::METRE) }
}

/// Crude interim baseline: a coordinate-noise elevation → a column. Deterministic
/// (pure function of the cell), and — since v2, 2026-07-10 — **continuous across
/// cube-face edges and corners by construction**: the noise is sampled on the
/// 3-D unit-sphere direction (`to_unit`), not per-face `(u, v)`. (v1's per-face
/// sampling had a measured ~2–3 km cliff across every face edge — invisible for
/// weeks because no view spanned a face; the first whole-globe view exposed it
/// within hours. The `prior_is_continuous_across_faces_and_corners` probe now
/// pins continuity.)
/// The two-band prior surface height (m above bedrock datum) for `cell`, with the
/// mountain band's octaves truncated at `nyquist_level`'s cell size. Exposed so
/// the eroded-region sampler can form the *detail increment*
/// `surface_prior_m(cell, cell.level()) - surface_prior_m(cell, region_level)` —
/// exactly the octave band finer than what the erosion grid simulated (fBm
/// truncation is a prefix, §8, so the difference IS the fine octaves).
pub fn surface_prior_m(seed: u64, cell: CellId, nyquist_level: u8) -> f64 {
    let p = cell.to_cube().to_unit();
    // Two-band prior (the 2009 neworld idea — "change parameters based on earlier
    // noise" — and core's proven scaling). Slope is what makes terrain read, and
    // slope ~ amplitude/wavelength: a single continental band (±1500 m over
    // ~1250 km) is a billiard ball (~0.3% grades — measured, examples/topo.rs), so
    // the relief that *reads* must live at short wavelengths:
    //  • CONTINENTS: λ ~1250 km, ±1500 m — decides land vs ocean, shelf vs abyss.
    //  • MOUNTAINS: λ ~25 km base, octaves to Nyquist, amplitude GROWN on high
    //    continent (zero in the deep ocean, full above ~+600 m) — like core's
    //    22 km massifs. Finer structure than ~200 m is the erosion tier's job.
    // Frequency conversion (v1 counted features per face-width; a face edge
    // subtends π/2 rad, and the unit vector is ~unit length, so lattice cells
    // per unit-sphere coordinate = per_face / (π/2)):
    let face_edge_m = std::f64::consts::FRAC_PI_2 * crate::planet::Planet::EARTH.radius_m;
    const CONT_PER_FACE: f64 = 8.0; // λ ~1250 km
    const MTN_PER_FACE: f64 = 400.0; // λ ~25 km
    let f_cont = CONT_PER_FACE / std::f64::consts::FRAC_PI_2;
    let f_mtn = MTN_PER_FACE / std::f64::consts::FRAC_PI_2;
    let cont_m =
        (fbm3(seed, 0, p[0] * f_cont, p[1] * f_cont, p[2] * f_cont, 4, 2.0, 0.5) - 0.5) * 3000.0;
    let mtn_amp = 1800.0 * ((cont_m + 200.0) / 800.0).clamp(0.0, 1.0);
    // Mountain octaves run from the 25 km base down to the SAMPLE's Nyquist (2
    // cells at this cell's level): un-eroded terrain is epic at every scale it is
    // looked at — just bubblier than carved terrain (erosion adds the organized
    // channels, not the bulk verticality). Truncating fBm at the Nyquist is a
    // low-pass, so a coarse-level sample is the honest smoothed view of the fine
    // one (§5 consistency; abstraction-as-prefix, §8) rather than an alias.
    let cell_m = face_edge_m / (1u64 << nyquist_level) as f64;
    let base_lambda_m = face_edge_m / MTN_PER_FACE; // ~25 km
    let n_oct = ((base_lambda_m / (2.0 * cell_m)).log2().floor() as i64 + 1).clamp(1, 16) as u32;
    let mtn_m =
        (fbm3(seed, 1, p[0] * f_mtn, p[1] * f_mtn, p[2] * f_mtn, n_oct, 2.0, 0.5) - 0.5) * 2.0 * mtn_amp;
    SEA_LEVEL_M + cont_m + mtn_m
}

pub fn baseline_column(seed: u64, cell: CellId) -> Column {
    column_from_surface(cell, surface_prior_m(seed, cell, cell.level()), 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::{CubeCoord, Face};

    fn some_cell(u: f64, v: f64) -> CellId {
        CubeCoord { face: Face::ZPos, u, v }.cell(12)
    }

    #[test]
    fn surface_assembles_dry_land() {
        let c = column_from_surface(some_cell(0.0, 0.0), 5000.0, 2.0);
        assert!((c.solid_thickness_m() - 5000.0).abs() < 1e-9);
        assert!((c.regolith_thickness_m() - 2.0).abs() < 1e-9); // soil on top
        assert_eq!(c.water_depth.value, 0.0); // above sea level
    }

    #[test]
    fn surface_assembles_seabed() {
        let c = column_from_surface(some_cell(0.1, 0.1), 3000.0, 2.0);
        assert!((c.water_depth.value - 1000.0).abs() < 1e-9); // 4000 - 3000
        assert!((c.solid_thickness_m() - 3000.0).abs() < 1e-9);
    }

    #[test]
    fn baseline_is_deterministic_and_varied() {
        let a = some_cell(0.2, -0.3);
        assert_eq!(baseline_column(0, a).solid_thickness_m(), baseline_column(0, a).solid_thickness_m());
        // different cells generally differ
        let h1 = baseline_column(0, some_cell(-0.5, 0.4)).solid_thickness_m();
        let h2 = baseline_column(0, some_cell(0.6, -0.1)).solid_thickness_m();
        assert!(h1 > 0.0 && h2 > 0.0);
        assert_ne!(h1, h2);
    }

    #[test]
    fn prior_v2_golden() {
        // Golden values pin the v2 (sphere-continuous) prior against silent
        // drift. History: v1 goldens (per-face sampling) were retired with v1
        // itself when its measured ~2–3 km face-edge cliffs forced the v2
        // rewrite — same day the v1 goldens were minted. GOLDEN_V2 values
        // captured 2026-07-10 immediately after the v2 swap.
        use crate::sphere::Face;
        let c1 = CellId::from_face_ij(Face::from_index(2), 5308416, 13238272, 24);
        let c2 = CellId::from_face_ij(Face::from_index(1), 100, 100, 19);
        assert_eq!(surface_prior_m(0, c1, 24), GOLDEN_V2_C1);
        assert_eq!(surface_prior_m(0, c2, 19), GOLDEN_V2_C2);
    }
    const GOLDEN_V2_C1: f64 = 4.16019378505400800e3;
    const GOLDEN_V2_C2: f64 = 3.50238149865287596e3;

    #[test]
    fn different_seeds_are_different_worlds() {
        let c = some_cell(0.3, 0.3);
        assert_ne!(surface_prior_m(0, c, 12), surface_prior_m(42, c, 12));
    }

    /// The face-seam probe (written FIRST, per the regime-probe discipline —
    /// DESIGN-REDUX §2b): the prior must be continuous across cube-face edges
    /// and corners. Method: step along many small arcs of equal length, some
    /// crossing a face edge (or skirting the (1,1,1) corner), some within one
    /// face; the cross-seam elevation deltas must be the same order as the
    /// within-face ones. The v1 per-face prior fails this catastrophically
    /// (independent noise fields per face ⇒ O(amplitude) cliffs at every edge —
    /// the deficiency the first whole-globe view made visible, 2026-07-10).
    #[test]
    fn prior_is_continuous_across_faces_and_corners() {
        use crate::sphere::CubeCoord;
        // Probe sensitivity is part of the probe (learned 2026-07-10: a first
        // draft at L10/arc 3e-3 PASSED on the discontinuous v1 prior — within-
        // face deltas at ~2-cell separation were large enough to mask a 2–3 km
        // cliff). At one-cell separation the discrimination is ~7×: v1 measured
        // cross-seam ~3,000 m vs within-face ~440 m at L12.
        let level = 12u8;
        let arc = std::f64::consts::FRAC_PI_2 / 4096.0; // one cell at L12
        let prior_at = |d: [f64; 3]| {
            let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            let cell = CubeCoord::from_unit([d[0] / n, d[1] / n, d[2] / n]).cell(level);
            surface_prior_m(0, cell, level)
        };
        // Pairs straddling the +X/+Z edge, the +X/+Y edge, and rays past the
        // (1,1,1) corner; matched within-face pairs at the same arc length.
        let mut cross_max = 0.0f64;
        let mut within_max = 0.0f64;
        for k in 0..25 {
            let t = -0.5 + k as f64 / 24.0; // slide along the edge
            // +X/+Z edge: dominant axis flips as x vs z dominates.
            let a = prior_at([1.0, t, 1.0 - arc]);
            let b = prior_at([1.0 - arc, t, 1.0]);
            cross_max = cross_max.max((a - b).abs());
            // corner (1,1,1): three samples on three faces, pairwise.
            let c1 = prior_at([1.0, 1.0 - arc + t * 1e-3, 1.0 - 2.0 * arc]);
            let c2 = prior_at([1.0 - 2.0 * arc, 1.0 - arc + t * 1e-3, 1.0]);
            cross_max = cross_max.max((c1 - c2).abs());
            // within-face control at the same separation, mid-face.
            let w1 = prior_at([1.0, 0.2 + t * 0.3, 0.1]);
            let w2 = prior_at([1.0, 0.2 + t * 0.3 + arc, 0.1]);
            within_max = within_max.max((w1 - w2).abs());
        }
        assert!(
            cross_max < within_max.max(1.0) * 4.0,
            "face-seam cliff: cross-seam max Δ {cross_max:.1} m vs within-face max Δ {within_max:.1} m"
        );
    }
}
