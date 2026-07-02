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
    const FEATURES_PER_FACE: f64 = 8.0;
    let h01 = fbm(0, (c.u + 1.0) * FEATURES_PER_FACE, (c.v + 1.0) * FEATURES_PER_FACE, 6, 2.0, 0.5);
    let surface_m = SEA_LEVEL_M + (h01 - 0.5) * 3000.0; // ±1500 m of crude relief
    column_from_surface(cell, surface_m, 2.0)
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
