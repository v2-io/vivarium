//! Baseline worldgen — turn a [`CellId`] into a [`Column`] (the abstract→detail
//! *easy* direction, `DESIGN-REDUX.md` §11, and a fidelity-ladder bottom rung §12).
//!
//! [`column_from_surface`] is the **reusable assembler**: given a solid surface
//! height it builds the stratigraphy. It is *not* throwaway — the ported erosion
//! tier will feed it too; only the *source of the height* moves up the ladder.
//! [`baseline_column`] is the crude interim height from coordinate noise
//! (deterministic, pure function of the cell — §8), to be superseded by the
//! ported erosion/hydrology.

use crate::column::{Column, Stratum};
use crate::material::{Category, Kind, MaterialId};
use crate::noise::fbm;
use crate::quantity::{Quantity, Unit};
use crate::sphere::CellId;

/// Sea-level datum, metres above the bedrock datum (`y = 0`). Provisional — a
/// coarse global reference (see the ~20 km shell, `DESIGN-MATERIAL.md` §8); a
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
/// (pure function of the cell). Per-face fBm, so cube-edge seams are a known
/// bottom-rung deficiency — superseded when erosion is ported to feed
/// [`column_from_surface`].
pub fn baseline_column(cell: CellId) -> Column {
    let c = cell.to_cube();
    // Two-band prior (the 2009 neworld idea — "change parameters based on earlier
    // noise" — and core's proven scaling). Slope is what makes terrain read, and
    // slope ~ amplitude/wavelength: a single continental band (±1500 m over
    // ~1250 km) is a billiard ball (~0.3% grades — measured, examples/topo.rs), so
    // the relief that *reads* must live at short wavelengths:
    //  • CONTINENTS: λ ~1250 km, ±1500 m — decides land vs ocean, shelf vs abyss.
    //  • MOUNTAINS: λ ~25 km base, 7 octaves (down to ~200 m), amplitude GROWN on
    //    high continent (zero in the deep ocean, full above ~+600 m) — like core's
    //    22 km massifs. Finer structure than ~200 m is the erosion tier's job.
    const CONT_PER_FACE: f64 = 8.0; // ~1250 km
    const MTN_PER_FACE: f64 = 400.0; // ~25 km
    let (su, sv) = (c.u + 1.0, c.v + 1.0);
    let cont_m = (fbm(0, su * CONT_PER_FACE, sv * CONT_PER_FACE, 4, 2.0, 0.5) - 0.5) * 3000.0;
    let mtn_amp = 1800.0 * ((cont_m + 200.0) / 800.0).clamp(0.0, 1.0);
    let mtn_m = (fbm(1, su * MTN_PER_FACE, sv * MTN_PER_FACE, 7, 2.0, 0.5) - 0.5) * 2.0 * mtn_amp;
    column_from_surface(cell, SEA_LEVEL_M + cont_m + mtn_m, 2.0)
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
        assert_eq!(baseline_column(a).solid_thickness_m(), baseline_column(a).solid_thickness_m());
        // different cells generally differ
        let h1 = baseline_column(some_cell(-0.5, 0.4)).solid_thickness_m();
        let h2 = baseline_column(some_cell(0.6, -0.1)).solid_thickness_m();
        assert!(h1 > 0.0 && h2 > 0.0);
        assert_ne!(h1, h2);
    }
}
