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

/// Amplitude of the fated spatial jitter on precipitation (±fraction of the mean).
/// `ASSUMPTIONS.md` "precip jitter". A *declared placeholder for variance*, not a
/// weather model.
pub const PRECIP_JITTER: f64 = 0.5;

/// Fated, **mean-preserving** spatial jitter on precipitation (a multiplicative
/// factor about 1.0). Joseph's principle, 2026-07-12: *where you cannot yet model
/// a real stochastic process, unmodelled jitter is still closer to truth than
/// none* — because perfectly uniform rain is a physically impossible state (zero
/// spatial variance), while real precipitation varies by orders of magnitude.
///
/// Three things make this honest rather than decorative:
/// - **Deterministic** (KRNG/fBm of `(seed, cell)`) — fated noise, replayable, and
///   memoizable like everything else. No wall-clock, no stream.
/// - **Low-frequency**, not white noise. Rain is spatially correlated at synoptic /
///   continental scales (hundreds of km); per-cell white noise would be *less* true
///   than uniform. Features here are ~1000+ km.
/// - **Mean-preserving** (the fBm is zero-mean about 1.0), so the global integral
///   still equals the reservoir's throughput — the conservation we just established
///   is not broken by adding variance. (Preserved in *expectation*; exact global
///   closure is a probe worth writing.)
///
/// **What it does NOT claim:** the *pattern* is noise, not meteorology. It buys
/// "there are wet places and dry places" — it does not buy "*where*". The real
/// first-order structure on an early Earth is LATITUDINAL (ITCZ / Hadley bands
/// from rotation + insolation), which is a *modellable* pattern we have insolation
/// for and have not built yet. Any claim reading geography off this jitter is false.
pub fn precip_jitter_factor(seed: u64, cell: crate::sphere::CellId) -> f64 {
    let c = cell.to_cube();
    // Low-frequency fBm over the face (features ~1000+ km), own noise domain.
    let f = crate::noise::fbm(seed, 11, (c.u + 1.0) * 4.0, (c.v + 1.0) * 4.0, 4, 2.0, 0.5);
    (1.0 + PRECIP_JITTER * (2.0 * f - 1.0)).max(0.0)
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

    #[test]
    fn jitter_is_fated_mean_preserving_and_correlated() {
        use crate::sphere::{CellId, Face};
        let f = Face::from_index(2);
        let at = |i: u32, j: u32| precip_jitter_factor(0, CellId::from_face_ij(f, i, j, 12));

        // 1. FATED: same (seed, cell) ⇒ same factor. No stream, no wall-clock.
        assert_eq!(at(1000, 1000), at(1000, 1000));

        // 2. MEAN-PRESERVING: the factor averages ≈ 1.0 over a broad sample, so the
        //    global precip integral still equals the reservoir throughput — adding
        //    variance must not break the conservation the hydrosphere established.
        let n = 64u32;
        let mut sum = 0.0;
        for j in 0..n {
            for i in 0..n {
                sum += at(i * 61, j * 61); // stride to span the face
            }
        }
        let mean = sum / (n * n) as f64;
        assert!((0.93..1.07).contains(&mean), "jitter mean {mean} must sit at ~1.0 (conservation)");

        // 3. NON-NEGATIVE and bounded by the declared amplitude.
        assert!(at(7, 9) >= 0.0 && at(7, 9) <= 1.0 + PRECIP_JITTER + 1e-9);

        // 4. SPATIALLY CORRELATED, not white noise: adjacent cells differ far less
        //    than distant ones. Rain doesn't decorrelate cell-to-cell at ~km scales.
        let adjacent = (at(1000, 1000) - at(1001, 1000)).abs();
        let distant = (at(1000, 1000) - at(1000 + 3000, 1000)).abs();
        assert!(adjacent < distant.max(1e-6), "jitter must be low-frequency (correlated), not white noise");
    }
}
