//! The planet and its astronomy — the outermost, coarsest driver in the pull graph
//! (`doc/design/DESIGN-REDUX.md` §11), and the first tier built. Deliberately the **crude
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
    /// Total planetary mass (kg) — an ante-mundane charge. Load-bearing for the
    /// hydrosphere: the conserved water inventory is a declared FRACTION of it
    /// (`crate::hydrosphere`), so water can never be conjured from nothing.
    pub mass_kg: f64,
    pub axial_tilt_rad: f64,
    /// Top-of-atmosphere solar irradiance at the mean orbital distance (W/m²).
    pub solar_constant: f64,
}

impl Planet {
    /// Earth, Holocene reference.
    pub const EARTH: Planet = Planet {
        radius_m: 6_371_000.0,
        mass_kg: 5.972e24,
        axial_tilt_rad: 0.409_105_2, // 23.44°
        solar_constant: 1361.0,
    };

    /// Total surface area (m²) — `4πr²`. The datum for global-equivalent depths
    /// (a reservoir's volume spread evenly over the planet).
    pub fn surface_area_m2(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius_m * self.radius_m
    }

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
        let decl = self.solar_declination(t.year_fraction());

        // Hour angle: 0 at local solar noon, ±π at midnight. Local solar time is the
        // day fraction shifted by longitude (one full turn per day).
        let solar_day = (t.day_fraction() + geo.lon / TAU).rem_euclid(1.0);
        let hour = TAU * (solar_day - 0.5);

        // Cosine of the solar zenith angle (standard sun-position identity).
        let cos_zenith =
            geo.lat.sin() * decl.sin() + geo.lat.cos() * decl.cos() * hour.cos();

        Quantity::approx(self.solar_constant * cos_zenith.max(0.0), Unit::WATT_PER_M2)
    }

    /// Daily-MEAN top-of-atmosphere insolation (W/m²) at a latitude and point in
    /// the year — the classic sunset-hour-angle integral (exact for the crude
    /// rung's circular orbit). `h0` handles polar day (π) and night (0) naturally.
    pub fn daily_insolation(&self, lat: f64, year_fraction: f64) -> f64 {
        let decl = self.solar_declination(year_fraction);
        let h0 = (-lat.tan() * decl.tan()).clamp(-1.0, 1.0).acos();
        (self.solar_constant / std::f64::consts::PI)
            * (h0 * lat.sin() * decl.sin() + lat.cos() * decl.cos() * h0.sin())
    }

    /// Annual-mean TOA insolation (W/m²): deterministic 24-sample average of the
    /// daily mean across the year.
    pub fn annual_mean_insolation(&self, lat: f64) -> f64 {
        (0..24).map(|k| self.daily_insolation(lat, (k as f64 + 0.5) / 24.0)).sum::<f64>() / 24.0
    }

    // NOTE (2026-07-03, Joseph): the planet tier deliberately does NOT offer
    // temperature. Insolation (exact geometry, above) is the honest primitive
    // flux; temperature EMERGES later from column-level energy balance
    // (absorption, simple atmosphere normalizations) in its own tier. A fitted
    // T(lat, alt) formula was written here and removed the same hour — imposed
    // calibration at the flux tier is the knob pattern wearing a lab coat.

    /// Solar declination (radians) at year fraction in `[0, 1)` — northern
    /// vernal equinox at `0`, northern summer solstice near `0.25`.
    pub fn solar_declination(&self, year_fraction: f64) -> f64 {
        self.axial_tilt_rad * (TAU * year_fraction.rem_euclid(1.0)).sin()
    }

    /// Unit vector **from the planet toward the sun** in the world frame
    /// (+Y = north pole; lon measured from +X toward +Z, same as [`CubeCoord`]).
    ///
    /// Subsolar longitude = `2π (0.5 − day_fraction)`: at lon-0 noon
    /// (`day_fraction = 0.5`) the sun is on +X; as the day advances the
    /// subsolar point moves **west** (−15°/hour), which is the correct
    /// geometric counterpart of Earth rotating east.
    pub fn sun_direction_world(&self, day_fraction: f64, year_fraction: f64) -> [f64; 3] {
        let decl = self.solar_declination(year_fraction);
        let df = day_fraction.rem_euclid(1.0);
        let slon = TAU * (0.5 - df);
        [
            decl.cos() * slon.cos(),
            decl.sin(),
            decl.cos() * slon.sin(),
        ]
    }

    /// Sun direction as a unit vector in the LOCAL East-North-Up frame at `geo`,
    /// time `t` (standard solar-position identity; the same declination and hour
    /// angle as [`Self::insolation`]). `up < 0` means the sun is below the
    /// horizon. Morning ⇒ `east > 0`: the sun rises in the east, which is what
    /// finally anchors the compass to something physical.
    pub fn sun_direction_enu(&self, geo: Geo, t: Time) -> [f64; 3] {
        let decl = self.solar_declination(t.year_fraction());
        let solar_day = (t.day_fraction() + geo.lon / TAU).rem_euclid(1.0);
        let hour = TAU * (solar_day - 0.5);
        let up = geo.lat.sin() * decl.sin() + geo.lat.cos() * decl.cos() * hour.cos();
        let east = -decl.cos() * hour.sin();
        let north = geo.lat.cos() * decl.sin() - geo.lat.sin() * decl.cos() * hour.cos();
        let len = (up * up + east * east + north * north).sqrt().max(1e-12);
        [east / len, north / len, up / len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Poles vs equator, and day vs night, should order the way physics demands —
    // cheap sanity that the crude rung is at least self-consistent.
    #[test]
    fn climate_basics_order_like_earth() {
        let p = Planet::EARTH;
        // Annual insolation: equator > mid-latitude > pole.
        let (qe, qm, qp) = (p.annual_mean_insolation(0.0), p.annual_mean_insolation(0.7), p.annual_mean_insolation(1.4));
        assert!(qe > qm && qm > qp, "annual insolation must fall toward the pole: {qe} {qm} {qp}");
        // Polar day/night: the seasonal extremes must both appear in the flux.
        assert!(p.daily_insolation(1.4, 0.75) < 1.0, "polar night: ~zero all day");
        assert!(p.daily_insolation(1.4, 0.25) > 400.0, "polar day: strong around the clock");
        // The sun rises in the EAST (morning: local solar time ~08:00).
        let geo = Geo { lat: 0.6, lon: 0.0 };
        let morning = Time::from_deciseconds((8 * 3600) * 10);
        let enu = p.sun_direction_enu(geo, morning);
        assert!(enu[0] > 0.3 && enu[2] > 0.0, "morning sun in the eastern sky: {enu:?}");
        // Noon at northern mid-latitude: sun to the SOUTH.
        let noon = Time::from_deciseconds((12 * 3600) * 10);
        let enu = p.sun_direction_enu(geo, noon);
        assert!(enu[1] < -0.2 && enu[2] > 0.5, "noon sun south and high: {enu:?}");
    }

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

    #[test]
    fn sun_world_matches_insolation_and_moves_west() {
        let p = Planet::EARTH;
        // World-frame sun · surface normal == cos(zenith) from the insolation identity.
        for h in [0.0_f64, 6.0, 12.0, 18.0] {
            let df = h / 24.0;
            let yf = 0.0; // equinox
            let sun = p.sun_direction_world(df, yf);
            let geo = Geo { lat: 0.0, lon: 0.0 };
            let n = [
                geo.lat.cos() * geo.lon.cos(),
                geo.lat.sin(),
                geo.lat.cos() * geo.lon.sin(),
            ];
            let dot = sun[0] * n[0] + sun[1] * n[1] + sun[2] * n[2];
            let t = Time::from_deciseconds((h * 3600.0 * 10.0) as i64);
            let cos_z = {
                let decl = p.solar_declination(yf);
                let solar_day = (df + geo.lon / TAU).rem_euclid(1.0);
                let hour = TAU * (solar_day - 0.5);
                geo.lat.sin() * decl.sin() + geo.lat.cos() * decl.cos() * hour.cos()
            };
            assert!(
                (dot - cos_z).abs() < 1e-9,
                "h={h}: sun·n={dot} cos_z={cos_z}"
            );
            let _ = t;
        }
        // Subsolar longitude decreases 15° per hour (terminator marches west).
        let lon = |h: f64| {
            let s = p.sun_direction_world(h / 24.0, 0.0);
            s[2].atan2(s[0]).to_degrees()
        };
        let d = (lon(13.0) - lon(12.0) + 180.0).rem_euclid(360.0) - 180.0;
        assert!((d + 15.0).abs() < 1e-6, "subsolar should move west ~15°/h, Δ={d}");
        // World sun projects to the same ENU as sun_direction_enu (equinox, lon 0).
        let geo = Geo { lat: 0.6, lon: 0.0 };
        let morning = Time::from_deciseconds((8 * 3600) * 10);
        let enu = p.sun_direction_enu(geo, morning);
        let sun = p.sun_direction_world(morning.day_fraction(), morning.year_fraction());
        let e = [-geo.lon.sin(), 0.0, geo.lon.cos()];
        let n = [
            -geo.lat.sin() * geo.lon.cos(),
            geo.lat.cos(),
            -geo.lat.sin() * geo.lon.sin(),
        ];
        let u = [
            geo.lat.cos() * geo.lon.cos(),
            geo.lat.sin(),
            geo.lat.cos() * geo.lon.sin(),
        ];
        let we = sun[0] * e[0] + sun[1] * e[1] + sun[2] * e[2];
        let wn = sun[0] * n[0] + sun[1] * n[1] + sun[2] * n[2];
        let wu = sun[0] * u[0] + sun[1] * u[1] + sun[2] * u[2];
        let len = (we * we + wn * wn + wu * wu).sqrt();
        assert!((we / len - enu[0]).abs() < 1e-9);
        assert!((wn / len - enu[1]).abs() < 1e-9);
        assert!((wu / len - enu[2]).abs() < 1e-9);
    }

    #[test]
    fn seasons_flip_hemisphere_at_solstices() {
        let p = Planet::EARTH;
        // year_fraction 0 = N vernal equinox; 0.25 ≈ N summer; 0.75 ≈ N winter.
        let d_sum = p.solar_declination(0.25).to_degrees();
        let d_win = p.solar_declination(0.75).to_degrees();
        assert!(d_sum > 23.0 && d_sum < 24.0, "N summer decl ~+23.4, got {d_sum}");
        assert!(d_win < -23.0 && d_win > -24.0, "N winter decl ~-23.4, got {d_win}");
        // Terminator *tilt* reverses with season — that is not a day/night bug.
        assert!(d_sum * d_win < 0.0);
    }
}
