//! The climate nomos — precipitation as the atmosphere reservoir's throughput.
//!
//! This is the **flow** that makes the hydrosphere a *cycle* rather than a static
//! partition (`crate::hydrosphere`): it consumes the conserved atmosphere-water
//! stock and produces the precipitation field erosion and surface-water drink.
//!
//! **Epistemic tier: mechanically sufficient, conserving, causal — not yet true
//! to geography.** The whole v0 kernel is one identity:
//!
//! ```text
//!   mean precip (m/yr) = atmosphere stock (m w.e.) / atmosphere residence time (yr)
//! ```
//!
//! It is *conserving*: in steady state precipitation = evaporation = throughput,
//! so the total inventory is untouched — water merely cycles ocean → atmosphere →
//! ocean (the "rain from nothing" worry answered at the flow, as the mass fraction
//! answered it at the stock). It is *causal*: precipitation is driven by a real
//! declared stock and residence time, not a magic number. And it lands at the
//! right **order of magnitude by construction** — Earth's ~25 mm atmosphere over a
//! ~9-day residence is ~1 m/yr, which is Earth's mean precipitation. What it is
//! **not** yet: spatially resolved. v0 is globally UNIFORM — no ITCZ, no
//! orographic rain-shadow, no latitude bands. Those are the next rungs (a real EBM
//! / circulation model consuming insolation, `TODO.md` §climate). So a *pattern*
//! claim from this precipitation would be false; only its global mean is meant.

/// Mean residence time of water in the atmosphere (years). Earth-ref ≈ 9 days —
/// `ASSUMPTIONS.md` "atmosphere residence time". This is the single knob that
/// converts a static stock into a cycling flux; the whole crude climate is it.
pub const ATMOSPHERE_RESIDENCE_YR: f64 = 9.0 / 365.25;

/// Global-mean precipitation (m/yr): the atmosphere reservoir's throughput,
/// `stock / residence-time`. Conserving (= evaporation in steady state) and
/// order-correct (~1 m/yr for Earth). `atmosphere_m_we` comes from the hydrosphere
/// nomos (the `atmosphere water (m w.e.)` flux).
pub fn mean_precip_m_per_yr(atmosphere_m_we: f64) -> f64 {
    atmosphere_m_we / ATMOSPHERE_RESIDENCE_YR
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hydrosphere::Hydrosphere;
    use crate::planet::Planet;

    #[test]
    fn earth_mean_precip_is_order_one_metre_per_year() {
        // The crude cycle must reproduce Earth's ~1 m/yr mean precipitation from
        // the conserved atmosphere stock — the sanity that says the throughput
        // identity is wired right, not just dimensionally plausible.
        let atm = Hydrosphere::of(&Planet::EARTH).atmosphere_m_we(&Planet::EARTH);
        let p = mean_precip_m_per_yr(atm);
        assert!((0.7..1.5).contains(&p), "mean precip {p} m/yr not ~1 (Earth)");
    }

    #[test]
    fn precip_scales_with_the_stock() {
        // Causality made checkable: more atmospheric water ⇒ more rain, linearly.
        assert!(mean_precip_m_per_yr(0.05) > mean_precip_m_per_yr(0.025));
        assert_eq!(mean_precip_m_per_yr(0.0), 0.0, "no vapour, no rain");
    }
}
