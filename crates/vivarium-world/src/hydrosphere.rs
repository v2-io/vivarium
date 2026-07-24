//! The hydrosphere nomos — the planet's conserved water budget, derived from an
//! ante-mundane charge (Joseph, 2026-07-12).
//!
//! **The principle this exists to enforce: water is never conjured.** The
//! ante-mundane phase (ordinum Phase 1 — "the constants, the box every later
//! computation happens inside") declares what fraction of the planet's *mass* is
//! chemical water. That fraction times the planetary mass fixes a **conserved
//! inventory**, which the reservoirs (ocean, atmosphere) merely *partition*. So
//! when precipitation later draws on the atmosphere reservoir, it draws on a
//! finite, accounted stock that ultimately traces back to declared planetary
//! mass — the honest root under "can we rain principled water?" (`crate::audit`).
//! Global water-cycle conservation, in its basics: total = ocean + atmosphere,
//! always (an exact promise the nomos carries).
//!
//! **This is the framework's first NON-FIELD nomos — a reservoir / box model**
//! (ARCHITECTURE §0's four representation kinds; the domain-fixation-guard
//! generality probe). It has no spatial grid: it produces global stocks (a few
//! scalars), keyed by (seed, version) alone. That it plugs into the same
//! store / nomotheke / flux-web machinery as the field nomos is the point — the
//! contract is representation-agnostic, and this proves it.
//!
//! v0 is crude and honest: earth-referenced fractions (see `ASSUMPTIONS.md`
//! "water mass fraction" / "atmosphere fraction"), ocean lumps in ice +
//! groundwater, and the "cycle" here is only the conserved *partition* — the
//! actual precip/evap FLOW between reservoirs is the next rung (a climate nomos
//! that consumes this atmosphere stock and produces a precipitation field).

use crate::planet::Planet;

/// Density of liquid water (kg/m³) — for mass ⇄ volume. Earth-ref, effectively exact.
pub const WATER_DENSITY_KG_M3: f64 = 1000.0;

/// Proportion of the planet's total mass that is chemical water (H₂O). ANTE-MUNDANE
/// CHARGE (`ASSUMPTIONS.md` "water mass fraction"). Earth surface/hydrosphere value
/// ≈ 2.3e-4 (ocean ~1.35e21 kg / Earth ~5.97e24 kg); deep-mantle water would raise
/// it and is contested — declared, not derived, and jettisonable for other worlds.
pub const WATER_MASS_FRACTION: f64 = 2.3e-4;

/// Fraction of the total water inventory held in the atmosphere at any moment.
/// Earth-ref ≈ 9.5e-6 (~12,900 km³ of ~1.37e9 km³ total ≈ 25 mm global-equivalent).
/// `ASSUMPTIONS.md` "atmosphere fraction".
pub const ATMOSPHERE_FRACTION: f64 = 9.5e-6;

/// The planet's water budget: a conserved total, partitioned across reservoirs.
/// All volumes in km³. Ocean here lumps ice + groundwater (v0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hydrosphere {
    /// The conserved inventory: (planet mass × water-mass-fraction) / density.
    pub total_km3: f64,
    /// Ocean + (lumped) ice + groundwater.
    pub ocean_km3: f64,
    /// Atmospheric water vapour + cloud — the stock precipitation draws on.
    pub atmosphere_km3: f64,
}

impl Hydrosphere {
    /// Derive the budget from a planet's ante-mundane charges. Pure arithmetic over
    /// declared constants — the whole (crude) kernel.
    pub fn of(planet: &Planet) -> Self {
        let total_kg = planet.mass_kg * WATER_MASS_FRACTION;
        let total_km3 = total_kg / WATER_DENSITY_KG_M3 / 1.0e9; // m³ → km³
        let atmosphere_km3 = total_km3 * ATMOSPHERE_FRACTION;
        let ocean_km3 = total_km3 - atmosphere_km3; // conservation by construction
        Hydrosphere { total_km3, ocean_km3, atmosphere_km3 }
    }

    /// Atmospheric water as a global-equivalent depth (m water-equivalent) — the
    /// stock spread evenly over the planet. This is the `atmosphere water (m w.e.)`
    /// flux a climate nomos will consume. ~25 mm for Earth.
    pub fn atmosphere_m_we(&self, planet: &Planet) -> f64 {
        (self.atmosphere_km3 * 1.0e9) / planet.surface_area_m2() // km³→m³ over m²
    }

    /// Ocean as a global-equivalent depth (m) — for the derived sea-level rung later.
    pub fn ocean_m_we(&self, planet: &Planet) -> f64 {
        (self.ocean_km3 * 1.0e9) / planet.surface_area_m2()
    }

    /// The conservation residual (km³): total − Σ reservoirs. Exactly 0 by
    /// construction (up to f64 rounding) — the promise the audit checks.
    pub fn conservation_residual_km3(&self) -> f64 {
        self.total_km3 - (self.ocean_km3 + self.atmosphere_km3)
    }

    /// Serialize to bytes for the store (three f64, little-endian). A box nomos's
    /// artifact is tiny — 24 bytes vs a field tile's thousands.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(24);
        for x in [self.total_km3, self.ocean_km3, self.atmosphere_km3] {
            b.extend_from_slice(&x.to_le_bytes());
        }
        b
    }

    /// Decode from the store.
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < 24 {
            return None;
        }
        let f = |i: usize| f64::from_le_bytes(b[i..i + 8].try_into().unwrap());
        Some(Hydrosphere { total_km3: f(0), ocean_km3: f(8), atmosphere_km3: f(16) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earth_budget_matches_reference() {
        // The crude model must reproduce known Earth figures to order of magnitude,
        // or the mass-fraction chain is wrong. Total ~1.3e9 km³ (ocean ~1.34e9),
        // atmosphere ~13,000 km³ ≈ 25 mm global-equivalent.
        let h = Hydrosphere::of(&Planet::EARTH);
        assert!((1.2e9..1.5e9).contains(&h.total_km3), "total {} km³ off", h.total_km3);
        assert!((11_000.0..15_000.0).contains(&h.atmosphere_km3), "atmos {} km³ off", h.atmosphere_km3);
        let mm = h.atmosphere_m_we(&Planet::EARTH) * 1000.0;
        assert!((20.0..30.0).contains(&mm), "atmosphere global-equiv {mm} mm off ~25 mm");
    }

    #[test]
    fn water_is_conserved_exactly() {
        // The load-bearing invariant: reservoirs partition the inventory, they do
        // not add to or subtract from it. total = ocean + atmosphere.
        let h = Hydrosphere::of(&Planet::EARTH);
        assert!(h.conservation_residual_km3().abs() < 1e-6, "residual {} km³", h.conservation_residual_km3());
    }

    #[test]
    fn roundtrips_through_bytes() {
        let h = Hydrosphere::of(&Planet::EARTH);
        assert_eq!(Hydrosphere::from_bytes(&h.to_bytes()), Some(h));
    }
}
