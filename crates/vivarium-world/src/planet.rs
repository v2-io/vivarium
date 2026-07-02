//! The planet and its astronomy — the outermost, coarsest driver in the pull graph
//! (`DESIGN-REDUX.md` §11), and the first tier built. Deliberately the **crude
//! bottom rung** of its fidelity ladder (§12): a perfect sphere, fixed 23.44°
//! axial tilt, a circular 1 AU orbit, and constant solar output. Higher rungs
//! (oblateness, elliptical orbit + Milankovitch cycles, solar variation) slot in
//! *behind the same flux interface* — [`Planet::insolation`] — so nothing
//! downstream has to change when a rung is swapped.
//!
//! It is pure, deterministic, and keyable — the idiom every later tier copies.

use crate::quantity::{Quantity, Unit};
use crate::sphere::Geo;
use crate::time::Time;

use std::f64::consts::TAU;

/// Earth-like reference constants for [`Epoch::Modern`](crate::time::Epoch::Modern)
/// (the Holocene). Coarse anchors for global sanity, not a detailed world.
#[derive(Clone, Copy, Debug)]
pub struct Planet {
    pub radius_m: f64,
    pub axial_tilt_rad: f64,
    /// Top-of-atmosphere solar irradiance at the mean orbital distance (W/m²).
    pub solar_constant: f64,
}

impl Planet {
    /// Earth, Holocene reference.
    pub const EARTH: Planet = Planet {
        radius_m: 6_371_000.0,
        axial_tilt_rad: 0.409_105_2, // 23.44°
        solar_constant: 1361.0,
    };

    /// Top-of-atmosphere solar irradiance on a horizontal surface at `geo`, at time
    /// `t`. The crude rung: a circular orbit with constant output means there is *no*
    /// distance- or output-variation, so only geometry matters — solar declination
    /// (from the axial tilt and the point in the year), the hour angle (from local
    /// solar time), and latitude. When the sun is below the horizon the result is 0.
    ///
    /// Marked [`Exactness::Approx`](crate::quantity::Exactness::Approx): it is the
    /// output of a deliberately low-fidelity model, and it says so.
    pub fn insolation(&self, geo: Geo, t: Time) -> Quantity {
        // Solar declination. Circular-orbit approximation with `year_fraction == 0`
        // taken at the (northern vernal) equinox ⇒ δ = ε·sin(2π · year_fraction).
        let decl = self.axial_tilt_rad * (TAU * t.year_fraction()).sin();

        // Hour angle: 0 at local solar noon, ±π at midnight. Local solar time is the
        // day fraction shifted by longitude (one full turn per day).
        let solar_day = (t.day_fraction() + geo.lon / TAU).rem_euclid(1.0);
        let hour = TAU * (solar_day - 0.5);

        // Cosine of the solar zenith angle (standard sun-position identity).
        let cos_zenith =
            geo.lat.sin() * decl.sin() + geo.lat.cos() * decl.cos() * hour.cos();

        Quantity::approx(self.solar_constant * cos_zenith.max(0.0), Unit::WATT_PER_M2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Poles vs equator, and day vs night, should order the way physics demands —
    // cheap sanity that the crude rung is at least self-consistent.
    #[test]
    fn night_is_dark_and_noon_is_bright() {
        let p = Planet::EARTH;
        let equator = Geo { lat: 0.0, lon: 0.0 };
        // t.day_fraction 0.5 at lon 0 ⇒ local noon; 0.0 ⇒ local midnight.
        let noon = p.insolation(equator, Time::from_deciseconds((super::super::time::SECONDS_PER_DAY / 2) * 10));
        let midnight = p.insolation(equator, Time::from_deciseconds(0));
        assert!(noon.value > 1000.0, "equator noon should be strong, got {}", noon.value);
        assert_eq!(midnight.value, 0.0, "equator midnight should be dark");
    }
}
