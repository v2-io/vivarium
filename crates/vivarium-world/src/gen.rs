//! Baseline worldgen — turn a [`CellId`] into a [`Column`] (the abstract→detail
//! *easy* direction, `doc/design/DESIGN-REDUX.md` §11, and a fidelity-ladder bottom rung §12).
//!
//! **Ordinum split (code-first trail from `ce55ddf` / `2f6a66d`):**
//! - [`bathymetry_m`] — seafloor / solid prior; **not** land (no decreed freeboard).
//! - freeboard / `emerged land` — Abyssal, via [`crate::uplift`] (later isostasy).
//! - sea level — derived by pouring the hydrosphere ocean ([`crate::sea_level`]).
//!
//! [`column_from_surface`] is the **reusable assembler**: given a solid surface
//! height it builds the stratigraphy. [`baseline_column`] uses tectonic surface
//! + derived sea for seed.

use crate::column::{Column, Stratum};
use crate::material::{Category, Kind, MaterialId};
use crate::noise::fbm3;
use crate::quantity::{Quantity, Unit};
use crate::sphere::CellId;

/// @deprecated **Do not use for land/water classification.** Decreed 4000 m
/// manufactured ~30% forbidden continents
/// (`DECISIONS[water-world-is-the-promise-not-the-bug]`). Prefer
/// [`crate::sea_level::derived_sea_level_m`]. Numeric residual for migrating
/// call sites only.
pub const SEA_LEVEL_M: f64 = 4000.0;

/// Assemble a column using **derived** sea level for world-seed 0 (tests that
/// need a fixed waterline should call [`column_from_surface_at_sea`]).
pub fn column_from_surface(cell: CellId, surface_m: f64, soil_m: f64) -> Column {
    column_from_surface_at_sea(
        cell,
        surface_m,
        soil_m,
        crate::sea_level::derived_sea_level_m(0),
    )
}

/// [`column_from_surface`] with an explicit sea-level datum (m above bedrock).
pub fn column_from_surface_at_sea(cell: CellId, surface_m: f64, soil_m: f64, sea_m: f64) -> Column {
    let surface_m = surface_m.max(0.0);
    let soil = soil_m.clamp(0.0, surface_m);
    let bedrock = surface_m - soil;

    let mut strata = Vec::new();
    if bedrock > 0.0 {
        strata.push(Stratum::new(MaterialId::Undifferentiated(Category::Igneous), bedrock, 0.0));
    }
    if soil > 0.0 {
        let saturation = if surface_m < sea_m { 1.0 } else { 0.3 };
        strata.push(Stratum::new(MaterialId::Kind(Kind::Soil), soil, saturation));
    }
    let water = (sea_m - surface_m).max(0.0);
    Column {
        tile: cell,
        strata,
        water_depth: Quantity::exact(water, Unit::METRE),
    }
}

/// **Bathymetry / solid prior** (m above bedrock datum): two-band fBm seafloor
/// relief **without** a decreed freeboard. Continuous across cube faces (sphere
/// sampling). Land above the waterline is Abyssal freeboard
/// ([`crate::uplift::freeboard_m`]), never this field alone.
pub fn bathymetry_m(seed: u64, cell: CellId, nyquist_level: u8) -> f64 {
    let p = cell.to_cube().to_unit();
    // Two-band prior (neworld / core scaling). Without a sea-level offset this is
    // honest seafloor relief: continental band is bathymetric wavelength, not
    // manufactured freeboard.
    //  • LONG band: λ ~1250 km, ±1500 m
    //  • SHORT band: λ ~25 km base, octaves to Nyquist, amplitude gated on long band
    let face_edge_m = std::f64::consts::FRAC_PI_2 * crate::planet::Planet::EARTH.radius_m;
    const CONT_PER_FACE: f64 = 8.0; // λ ~1250 km
    const MTN_PER_FACE: f64 = 400.0; // λ ~25 km
    let f_cont = CONT_PER_FACE / std::f64::consts::FRAC_PI_2;
    let f_mtn = MTN_PER_FACE / std::f64::consts::FRAC_PI_2;
    let cont_m =
        (fbm3(seed, 0, p[0] * f_cont, p[1] * f_cont, p[2] * f_cont, 4, 2.0, 0.5) - 0.5) * 3000.0;
    let mtn_amp = 1800.0 * ((cont_m + 200.0) / 800.0).clamp(0.0, 1.0);
    let cell_m = face_edge_m / (1u64 << nyquist_level) as f64;
    let base_lambda_m = face_edge_m / MTN_PER_FACE;
    let n_oct = ((base_lambda_m / (2.0 * cell_m)).log2().floor() as i64 + 1).clamp(1, 16) as u32;
    let mtn_m =
        (fbm3(seed, 1, p[0] * f_mtn, p[1] * f_mtn, p[2] * f_mtn, n_oct, 2.0, 0.5) - 0.5) * 2.0 * mtn_amp;
    // Positive bedrock-relative heights (shift so seafloor is mostly above y=0 for
    // column assembly); not a land freeboard.
    const SEAFLOOR_DATUM_M: f64 = 2500.0;
    SEAFLOOR_DATUM_M + cont_m + mtn_m
}

/// Solid surface for worldgen consumers: bathymetry + Abyssal freeboard
/// ([`crate::sea_level::tectonic_surface_m`]). Prefer that name for new call sites.
pub fn initial_topography_m(seed: u64, cell: CellId, nyquist_level: u8) -> f64 {
    crate::sea_level::tectonic_surface_m(seed, cell, nyquist_level)
}

pub fn baseline_column(seed: u64, cell: CellId) -> Column {
    let sea = crate::sea_level::derived_sea_level_m(seed);
    column_from_surface_at_sea(
        cell,
        initial_topography_m(seed, cell, cell.level()),
        2.0,
        sea,
    )
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
        let c = column_from_surface_at_sea(some_cell(0.0, 0.0), 5000.0, 2.0, 4000.0);
        assert!((c.solid_thickness_m() - 5000.0).abs() < 1e-9);
        assert!((c.regolith_thickness_m() - 2.0).abs() < 1e-9); // soil on top
        assert_eq!(c.water_depth.value, 0.0); // above sea level
    }

    #[test]
    fn surface_assembles_seabed() {
        let c = column_from_surface_at_sea(some_cell(0.1, 0.1), 3000.0, 2.0, 4000.0);
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
    fn bathymetry_v3_golden() {
        // Pins bathymetry (no freeboard, no decreed sea) against silent drift.
        // History: v1 per-face cliffs; v2 sphere-continuous with SEA_LEVEL_M
        // offset (retired — manufactured land). v3 = seafloor datum + fBm only.
        use crate::sphere::Face;
        let c1 = CellId::from_face_ij(Face::from_index(2), 5308416, 13238272, 24);
        let c2 = CellId::from_face_ij(Face::from_index(1), 100, 100, 19);
        // Old v2 goldens were SEA_LEVEL_M + relief; bathymetry = 2500 + same relief
        // as (v2 - 4000), i.e. v2 - 1500.
        assert!((bathymetry_m(0, c1, 24) - (GOLDEN_V2_C1 - 1500.0)).abs() < 1e-9);
        assert!((bathymetry_m(0, c2, 19) - (GOLDEN_V2_C2 - 1500.0)).abs() < 1e-9);
    }
    const GOLDEN_V2_C1: f64 = 4.16019378505400800e3;
    const GOLDEN_V2_C2: f64 = 3.50238149865287596e3;

    #[test]
    fn different_seeds_are_different_worlds() {
        let c = some_cell(0.3, 0.3);
        assert_ne!(initial_topography_m(0, c, 12), initial_topography_m(42, c, 12));
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
        //
        // Checks **tectonic** surface (bathymetry + freeboard): freeboard must
        // also be sphere-continuous, or the globe shows cube-face cliffs.
        let level = 12u8;
        let arc = std::f64::consts::FRAC_PI_2 / 4096.0; // one cell at L12
        let prior_at = |d: [f64; 3]| {
            let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            let cell = CubeCoord::from_unit([d[0] / n, d[1] / n, d[2] / n]).cell(level);
            initial_topography_m(0, cell, level)
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
