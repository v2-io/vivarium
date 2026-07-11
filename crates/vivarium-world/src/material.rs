//! Materials — the property bundle every physics rung reads (`doc/design/DESIGN-MATERIAL.md`
//! §5), and a material *identity* that is a point on the refinement ladder (§6).
//!
//! [`MaterialId`] is the small, storable identity kept in strata; [`Material`] is
//! the full property bundle it looks up. The identity spans the fidelity ladder:
//! fully individuated [`Kind`]s, geology-map **`Undifferentiated`** categories
//! (broad statistics, refine to Kinds later), an even cruder gameplay **`Ground`**
//! bucket (`hard/soft/loose/slippery`, no geology), and `Void`. Undifferentiated
//! and Ground materials expose the *same* interface but mark their values
//! [`Exactness::Approx`] — the ubit made concrete (§9).
//!
//! The values below are **plausible representative starters for the crude rung**,
//! not authoritative geotechnics; the middle (Mohr–Coulomb) rung refines them.
//! `density_solid` is the *bulk* density of the settled material (includes pores),
//! which is what overburden needs. Cohesion/incision are Pa; friction is stored in
//! radians; porosity/packing/erodibility are ratios.

use crate::quantity::{Exactness, Quantity, Unit};

/// Physical phase / mobility class — the discrete form of "fluidity" (survey: DF's
/// state enum). Governs falls-vs-flows-vs-holds and what counts as loose regolith.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Phase { Solid, Liquid, Gas, Powder, Paste }

/// A fully individuated material (starter set; extend freely).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Kind { Granite, Sandstone, Sand, Clay, Soil, Water }

/// A not-yet-subdivided category — the geology-map "undifferentiated" unit (§6).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Category { Igneous, Sedimentary, Metamorphic }

/// The crudest rung — ground by gameplay attribute, no geology (§6).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GroundKind { Hard, Soft, Loose, Slippery }

/// The storable material identity — a point on the refinement ladder. Small,
/// `Hash`/`Eq`, kept in [`crate::column::Stratum`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MaterialId {
    Kind(Kind),
    /// Broad category; refines deterministically to a `Kind` later (§6).
    Undifferentiated(Category),
    /// Gameplay bucket — even cruder than a category.
    Ground(GroundKind),
    /// Air / cave / excavated space.
    Void,
}

/// The property bundle a physics rung reads. Rich [`Quantity`] at this seam
/// (units + exactness); hot loops pull raw `f64` via [`Quantity::raw`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    pub phase: Phase,
    pub density_solid: Quantity,     // kg/m³ (bulk, settled)
    pub density_liquid: Quantity,    // kg/m³ (melt / saturated form)
    pub cohesion: Quantity,          // Pa   (C)
    pub friction_angle: Quantity,    // rad  (φ; == angle of repose when cohesionless)
    pub grain_size: Quantity,        // m    (d)
    pub porosity: Quantity,          // ratio
    pub permeability: Quantity,      // m²
    pub packing_fraction: Quantity,  // ratio
    pub erodibility: Quantity,       // ratio (crude K)
    pub incision_threshold: Quantity, // Pa (shear below which no incision)
}

#[allow(clippy::too_many_arguments)]
fn mat(phase: Phase, dens_s: f64, dens_l: f64, coh: f64, fric_deg: f64, grain: f64,
       poro: f64, perm: f64, pack: f64, erod: f64, incis: f64, ex: Exactness) -> Material {
    let q = |v: f64, u: Unit| Quantity { value: v, unit: u, exact: ex };
    Material {
        phase,
        density_solid: q(dens_s, Unit::KG_PER_M3),
        density_liquid: q(dens_l, Unit::KG_PER_M3),
        cohesion: q(coh, Unit::PASCAL),
        friction_angle: q(fric_deg.to_radians(), Unit::ONE),
        grain_size: q(grain, Unit::METRE),
        porosity: q(poro, Unit::ONE),
        permeability: q(perm, Unit::M2),
        packing_fraction: q(pack, Unit::ONE),
        erodibility: q(erod, Unit::ONE),
        incision_threshold: q(incis, Unit::PASCAL),
    }
}

impl MaterialId {
    /// The property bundle for this identity — the crude starter registry.
    pub fn material(self) -> Material {
        use Exactness::{Approx, Exact};
        use Phase::*;
        match self {
            MaterialId::Kind(k) => match k {
                //                phase   dens_s dens_l  cohesion  fric° grain    poro  perm    pack  erod  incis  ex
                Kind::Granite   => mat(Solid,  2700.0, 2500.0, 2.0e7, 35.0, 2e-3,  0.01, 1e-18, 0.99, 0.05, 5.0e3, Exact),
                Kind::Sandstone => mat(Solid,  2300.0, 2400.0, 5.0e6, 30.0, 3e-4,  0.15, 1e-13, 0.85, 0.25, 1.0e3, Exact),
                Kind::Sand      => mat(Powder, 1600.0, 2000.0, 0.0,   33.0, 5e-4,  0.40, 1e-11, 0.60, 0.80, 0.5,   Exact),
                Kind::Clay      => mat(Paste,  1800.0, 1600.0, 2.0e4, 20.0, 2e-6,  0.50, 1e-17, 0.50, 0.40, 0.2,   Exact),
                Kind::Soil      => mat(Powder, 1300.0, 1500.0, 1.0e4, 30.0, 1e-4,  0.50, 1e-13, 0.55, 0.70, 0.3,   Exact),
                Kind::Water     => mat(Liquid, 1000.0, 1000.0, 0.0,    0.0, 0.0,   1.00, 1e-9,  0.00, 0.00, 0.0,   Exact),
            },
            MaterialId::Undifferentiated(c) => match c {
                Category::Igneous     => mat(Solid, 2800.0, 2600.0, 1.0e7, 33.0, 1e-3, 0.05, 1e-16, 0.95, 0.10, 3.0e3, Approx),
                Category::Sedimentary => mat(Solid, 2400.0, 2400.0, 2.0e6, 28.0, 2e-4, 0.20, 1e-13, 0.80, 0.35, 8.0e2, Approx),
                Category::Metamorphic => mat(Solid, 2900.0, 2700.0, 2.0e7, 34.0, 5e-4, 0.02, 1e-18, 0.98, 0.08, 4.0e3, Approx),
            },
            MaterialId::Ground(g) => match g {
                GroundKind::Hard     => mat(Solid,  2600.0, 2600.0, 1.0e7, 40.0, 1e-3, 0.05, 1e-16, 0.95, 0.05, 5.0e3, Approx),
                GroundKind::Soft     => mat(Paste,  1700.0, 1600.0, 5.0e3, 22.0, 1e-4, 0.45, 1e-14, 0.55, 0.50, 0.3,   Approx),
                GroundKind::Loose    => mat(Powder, 1500.0, 1900.0, 0.0,   32.0, 5e-4, 0.40, 1e-11, 0.60, 0.90, 0.2,   Approx),
                GroundKind::Slippery => mat(Paste,  1600.0, 1500.0, 2.0e3,  8.0, 1e-5, 0.50, 1e-15, 0.50, 0.60, 0.1,   Approx),
            },
            MaterialId::Void => mat(Gas, 1.2, 0.0, 0.0, 0.0, 0.0, 1.0, 1e-5, 0.0, 0.0, 0.0, Approx),
        }
    }

    /// The phase of this material.
    pub fn phase(self) -> Phase { self.material().phase }

    /// Whether this material is loose/mobile (powder or paste) — the regolith the
    /// erosion/hydrology tiers can move (`doc/design/DESIGN-MATERIAL.md` §3's `r`).
    pub fn is_loose(self) -> bool { matches!(self.phase(), Phase::Powder | Phase::Paste) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_ids() -> Vec<MaterialId> {
        let mut v = vec![MaterialId::Void];
        for k in [Kind::Granite, Kind::Sandstone, Kind::Sand, Kind::Clay, Kind::Soil, Kind::Water] {
            v.push(MaterialId::Kind(k));
        }
        for c in [Category::Igneous, Category::Sedimentary, Category::Metamorphic] {
            v.push(MaterialId::Undifferentiated(c));
        }
        for g in [GroundKind::Hard, GroundKind::Soft, GroundKind::Loose, GroundKind::Slippery] {
            v.push(MaterialId::Ground(g));
        }
        v
    }

    #[test]
    fn registry_is_total_and_sane() {
        for id in all_ids() {
            let m = id.material();
            // units correct at the seam (raw() debug-asserts them)
            let _ = m.density_solid.raw(Unit::KG_PER_M3);
            let _ = m.cohesion.raw(Unit::PASCAL);
            let _ = m.permeability.raw(Unit::M2);
            let fa = m.friction_angle.raw(Unit::ONE);
            assert!((0.0..std::f64::consts::FRAC_PI_2).contains(&fa), "friction out of range: {id:?}");
            assert!(m.cohesion.value >= 0.0 && m.density_solid.value >= 0.0, "negative: {id:?}");
            assert!((0.0..=1.0).contains(&m.porosity.value), "porosity out of [0,1]: {id:?}");
        }
    }

    #[test]
    fn exactness_tracks_differentiation() {
        assert_eq!(MaterialId::Kind(Kind::Granite).material().density_solid.exact, Exactness::Exact);
        assert_eq!(MaterialId::Undifferentiated(Category::Igneous).material().density_solid.exact, Exactness::Approx);
        assert_eq!(MaterialId::Ground(GroundKind::Hard).material().cohesion.exact, Exactness::Approx);
    }

    #[test]
    fn phases_and_looseness() {
        assert!(matches!(MaterialId::Kind(Kind::Water).phase(), Phase::Liquid));
        assert!(MaterialId::Kind(Kind::Sand).is_loose());
        assert!(!MaterialId::Kind(Kind::Granite).is_loose());
        // cohesionless sand: angle of repose == friction angle > 0
        assert!(MaterialId::Kind(Kind::Sand).material().friction_angle.value > 0.0);
        assert_eq!(MaterialId::Kind(Kind::Sand).material().cohesion.value, 0.0);
    }
}
