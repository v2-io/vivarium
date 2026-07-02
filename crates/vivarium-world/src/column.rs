//! The column — the primary storage unit (`DESIGN-MATERIAL.md` §3). A
//! *stratigraphic column* at a [`CellId`]: an ordered stack of [`Stratum`]s
//! (bedrock at index 0, surface last), plus any standing water. Real-valued
//! stratum thickness means "10.5 m of sand" is one stratum, fractional for free;
//! **voxels are a materialized view, not the storage** (§2).
//!
//! Elevation, terrain-height `b`, regolith `r`, and overburden are all *derived*
//! here, never stored — the value's meaning is declared, not assumed (§4).

use crate::material::MaterialId;
use crate::quantity::{Exactness, Quantity, Unit};
use crate::sphere::CellId;

/// A run of one material — the storage primitive for vertically-coherent matter.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stratum {
    pub material: MaterialId,
    pub thickness: Quantity, // metres, real-valued
    pub saturation: f32,     // 0..1 pore-water fraction (dynamic state)
}

impl Stratum {
    pub fn new(material: MaterialId, thickness_m: f64, saturation: f32) -> Self {
        Self { material, thickness: Quantity::exact(thickness_m, Unit::METRE), saturation }
    }
}

/// A stratigraphic column at one cube-sphere cell.
#[derive(Clone, Debug)]
pub struct Column {
    pub tile: CellId,
    /// Bedrock (index 0) → surface (last).
    pub strata: Vec<Stratum>,
    /// Standing water above the solid top, in metres (the `d` in `(b, d, r)`).
    pub water_depth: Quantity,
}

impl Column {
    /// Total solid thickness (m) above the bedrock datum — the derived terrain
    /// height `b`. Elevation is always derived, never stored (§4).
    pub fn solid_thickness_m(&self) -> f64 {
        self.strata.iter().map(|s| s.thickness.raw(Unit::METRE)).sum()
    }

    /// Regolith `r` (m): the topmost contiguous *loose* strata — the material the
    /// erosion/hydrology tiers can move (§3).
    pub fn regolith_thickness_m(&self) -> f64 {
        self.strata
            .iter()
            .rev()
            .take_while(|s| s.material.is_loose())
            .map(|s| s.thickness.raw(Unit::METRE))
            .sum()
    }

    /// Overburden (normal stress, Pa) at `depth_m` below the solid surface:
    /// `Σ ρ·g·t` over the material above. Exactness propagates from the densities
    /// (any `Approx` stratum taints the result — §5/§9).
    pub fn overburden(&self, depth_m: f64, g: f64) -> Quantity {
        let mut remaining = depth_m.max(0.0);
        let mut stress = 0.0;
        let mut exact = Exactness::Exact;
        for s in self.strata.iter().rev() {
            if remaining <= 0.0 {
                break;
            }
            let t = s.thickness.raw(Unit::METRE).min(remaining);
            let d = s.material.material().density_solid;
            stress += d.value * g * t;
            exact = exact.and(d.exact);
            remaining -= t;
        }
        Quantity { value: stress, unit: Unit::PASCAL, exact }
    }

    /// The material at `height_m` above the bedrock datum — a voxel-view query.
    /// Above the solid top → [`MaterialId::Void`] (air/water live in `water_depth`).
    pub fn material_at(&self, height_m: f64) -> MaterialId {
        let mut base = 0.0;
        for s in &self.strata {
            let top = base + s.thickness.raw(Unit::METRE);
            if height_m < top {
                return s.material;
            }
            base = top;
        }
        MaterialId::Void
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{Kind, MaterialId};
    use crate::sphere::{CubeCoord, Face};

    fn sample() -> Column {
        let tile = CubeCoord { face: Face::ZPos, u: 0.0, v: 0.0 }.cell(10);
        Column {
            tile,
            strata: vec![
                Stratum::new(MaterialId::Kind(Kind::Granite), 100.0, 0.0), // bedrock
                Stratum::new(MaterialId::Kind(Kind::Sandstone), 20.0, 0.1),
                Stratum::new(MaterialId::Kind(Kind::Sand), 5.0, 0.2), // loose top
            ],
            water_depth: Quantity::exact(0.0, Unit::METRE),
        }
    }

    #[test]
    fn derived_queries() {
        let c = sample();
        assert!((c.solid_thickness_m() - 125.0).abs() < 1e-9);
        assert!((c.regolith_thickness_m() - 5.0).abs() < 1e-9); // only the sand is loose
        assert!(matches!(c.material_at(50.0), MaterialId::Kind(Kind::Granite)));
        assert!(matches!(c.material_at(122.0), MaterialId::Kind(Kind::Sand)));
        assert!(matches!(c.material_at(130.0), MaterialId::Void));

        // overburden grows with depth; 5 m into the sand ≈ ρ_sand·g·5
        let g = 9.81;
        let ob5 = c.overburden(5.0, g).raw(Unit::PASCAL);
        let expect = 1600.0 * g * 5.0;
        assert!((ob5 - expect).abs() / expect < 1e-9, "ob5 {ob5} vs {expect}");
        assert!(c.overburden(10.0, g).value > ob5);
    }
}
