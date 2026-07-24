//! The mantle-thermal nomos — secular cooling as a *process*, the head of the
//! accepted freeboard chain (`#form-isostasy-column` FE(2):
//! mantle-thermal → lithosphere → isostasy → sea-level → erosion).
//!
//! Until now the mantle potential temperature was a single declared constant
//! (`lithosphere::MANTLE_TP_C`): the world had a temperature but no *time axis*,
//! so land could not rise from the sea as the planet aged. This module promotes
//! that constant to an **article of law**: a declared secular-cooling trajectory
//! `T_p(t)` over canonical [`Time`]. Everything downstream of the column is
//! already a pure function of `T_p` (`lithosphere::freeboard_m_at_tp`, the pour
//! in `sea_level`), so a memoized chain over a handful of epochs makes emergence
//! run *in time* the way the ordinum ladder narrates: water-world → transient
//! stands → growing freeboard.
//!
//! **Honesty tier — declared crude (physics Low).** [`potential_temp_c`] is a
//! *declared* monotone curve, **not** integrated from a mantle heat budget. Its
//! two anchors are order-of-magnitude Earth pins — Archean-hot ~1550 °C, modern
//! ~1350 °C (Herzberg/Korenaga lineage; Korenaga 2017 still unread, so no
//! calibration is claimed beyond order of magnitude) — and the relaxation
//! timescale is chosen so the curve passes through the present-Abyssal anchor
//! and decays toward the modern asymptote over ~Gyr. What it buys convictably:
//! the *shape* of emergence (monotone cooling ⇒ deepening basins ⇒ growing land),
//! a falsifiable claim the probes carry. What it does **not** buy: a true rate,
//! plate history, or any promise the fraction/timing match a real era.
//!
//! **What cooling does NOT conserve (declared).** A cooling trajectory changes
//! oceanic-crust thickness and density as a *declared thermal effect* with no
//! melt-flux accounting, so the column's rock mass is **not** conserved across
//! epochs — the differentiation/rock-mass ledger that would balance it is open
//! (`#form-isostasy-column` FE(8)). The *within-epoch* mass balance still holds
//! exactly: the isostatic reference is re-integrated per `T_p`, so area-mean
//! freeboard ≡ 0 at every epoch (`lithosphere` unit test). Cooling redistributes
//! stand within an epoch and rewrites the inventory between epochs; neither mints
//! global elevation.

use crate::lithosphere::{MANTLE_TP_C, TP_MODERN_C};
use crate::time::Time;

// --- Declared trajectory constants (anchors into ASSUMPTIONS.md) -------------

/// Age (Ga before the Holocene origin) of the **present-Abyssal** reference
/// epoch — the one whose temperature is `MANTLE_TP_C`, so the live static world
/// (`lithosphere::freeboard_m`, the seed-only pour) is this epoch by
/// construction. Order-of-magnitude early-mid-Archean.
/// `ASSUMPTIONS.md` "mantle cooling trajectory".
pub const PRESENT_ABYSSAL_GA: f64 = 3.2;

/// Relaxation timescale (Ga) of the secular-cooling decay toward the modern
/// asymptote. Chosen so that ~one asymptote's-worth of contrast (`MANTLE_TP_C −
/// TP_MODERN_C`) has decayed by the Holocene origin (`exp(−PRESENT/TAU) ≈ 0.05`).
/// Declared crude — not a fitted mantle-cooling rate. Same anchor.
pub const COOLING_TAU_GA: f64 = 1.07;

/// Hottest potential temperature the trajectory is allowed to report (°C) — the
/// Hadean ceiling, at/above which the oceanic-crust ramp saturates (its clamp
/// caps melt-fraction thickening). Deeper past than the Abyssal window is not
/// asserted; the clamp keeps the closed form from running away. Same anchor.
pub const TP_HOT_MAX_C: f64 = 1650.0;

/// One canonical year-count for a Ga-before-origin age.
fn time_at_ga(age_ga: f64) -> Time {
    Time::from_years(-age_ga * 1.0e9)
}

/// The **present-Abyssal** epoch: `potential_temp_c` returns `MANTLE_TP_C` here,
/// exactly (by construction), so the seed-only live world is this epoch.
pub fn present_abyssal() -> Time {
    time_at_ga(PRESENT_ABYSSAL_GA)
}

/// The declared secular-cooling trajectory: mantle potential temperature (°C)
/// at canonical time `t`. Monotone non-increasing in `t` (the planet cools as it
/// ages), passing through `MANTLE_TP_C` at [`present_abyssal`] and decaying
/// toward `TP_MODERN_C` as `t → 0` (Holocene). Clamped to `[TP_MODERN_C,
/// TP_HOT_MAX_C]` — the curve is only *asserted* over the Abyssal window
/// ([`abyssal_epochs`]); the clamp keeps the closed form bounded outside it.
///
/// `T_p(t) = TP_MODERN + (MANTLE_TP_C − TP_MODERN)·exp((age(t) − PRESENT)/TAU)`,
/// with `age(t)` in Ga before the origin. Pure function of `t` — fated.
pub fn potential_temp_c(t: Time) -> f64 {
    let age_ga = -t.years() / 1.0e9;
    let amp = MANTLE_TP_C - TP_MODERN_C;
    let tp = TP_MODERN_C + amp * ((age_ga - PRESENT_ABYSSAL_GA) / COOLING_TAU_GA).exp();
    tp.clamp(TP_MODERN_C, TP_HOT_MAX_C)
}

/// The canonical Abyssal cooling chain: an ordered sequence of epochs from the
/// hot early-Abyssal (near a water-world) through the present-Abyssal anchor to
/// the cooling late-Abyssal — the epochs the probe and the emergence view walk.
/// Ordered by *time* (increasing t = cooling), so land fraction along it is the
/// emergence trajectory. Deliberately a short, legible chain, not a fine grid.
pub fn abyssal_epochs() -> Vec<Time> {
    [3.6, 3.4, PRESENT_ABYSSAL_GA, 3.0, 2.8, 2.6].iter().map(|&ga| time_at_ga(ga)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_through_the_present_abyssal_anchor_exactly() {
        // The coherence pin between the trajectory and the live static world:
        // the seed-only column/pour use MANTLE_TP_C directly, and the cooling
        // law must agree at the present-Abyssal epoch — else "present" would
        // silently mean two temperatures.
        let tp = potential_temp_c(present_abyssal());
        assert!((tp - MANTLE_TP_C).abs() < 1e-9, "present-Abyssal T_p {tp} must equal MANTLE_TP_C {MANTLE_TP_C}");
    }

    #[test]
    fn cooling_is_monotone_in_time() {
        // The trajectory's defining structure (declared `monotonicity`): as the
        // world ages (t increases toward the Holocene), the mantle cools. A
        // failable statement of the curve's shape, over and beyond the window.
        let epochs = abyssal_epochs();
        for pair in epochs.windows(2) {
            assert!(pair[0] < pair[1], "abyssal_epochs must be time-ordered");
            let (hot, cool) = (potential_temp_c(pair[0]), potential_temp_c(pair[1]));
            assert!(cool <= hot, "T_p must not rise as time advances ({hot} → {cool})");
        }
    }

    #[test]
    fn hot_end_is_hotter_than_present_and_bounded() {
        // The early-Abyssal end is hotter than the present anchor (thick, light
        // oceanic crust ⇒ shallow basins ⇒ flooding) and never exceeds the
        // declared Hadean ceiling where the oceanic ramp saturates.
        let hot = potential_temp_c(*abyssal_epochs().first().unwrap());
        assert!(hot > MANTLE_TP_C, "early-Abyssal must be hotter than present ({hot} vs {MANTLE_TP_C})");
        assert!(hot <= TP_HOT_MAX_C, "and not exceed the declared ceiling {TP_HOT_MAX_C}");
    }

    #[test]
    fn decays_toward_the_modern_asymptote() {
        // Toward the Holocene the curve relaxes to near the modern reference —
        // the asymptote is TP_MODERN_C, not overshot below it (the clamp guards
        // the far tail).
        let modern = potential_temp_c(Time::ORIGIN);
        assert!(modern >= TP_MODERN_C, "clamp floors the curve at the modern asymptote");
        assert!(modern < MANTLE_TP_C, "by the origin the mantle has cooled well below the Abyssal anchor ({modern})");
        assert!((modern - TP_MODERN_C) < 40.0, "and sits within ~tens of °C of the modern reference ({modern})");
    }

    #[test]
    fn deterministic() {
        let t = time_at_ga(3.1);
        assert_eq!(potential_temp_c(t), potential_temp_c(t));
    }
}
