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
/// ([`cooling_stages`]); the clamp keeps the closed form bounded outside it.
///
/// `T_p(t) = TP_MODERN + (MANTLE_TP_C − TP_MODERN)·exp((age(t) − PRESENT)/TAU)`,
/// with `age(t)` in Ga before the origin. Pure function of `t` — fated.
pub fn potential_temp_c(t: Time) -> f64 {
    let age_ga = -t.years() / 1.0e9;
    let amp = MANTLE_TP_C - TP_MODERN_C;
    let tp = TP_MODERN_C + amp * ((age_ga - PRESENT_ABYSSAL_GA) / COOLING_TAU_GA).exp();
    tp.clamp(TP_MODERN_C, TP_HOT_MAX_C)
}

// --- The cooling stage chain -------------------------------------------------
//
// Vocabulary note, load-bearing: these are **stages**, not "epochs". `LEXICON`
// reserves `epoch` *exclusively* for the erosion solver's step unit, and this
// chain is a stage chain in the sense of `#form-time-indexed-stage-chains` FE(8)
// — keyed memos carrying a time-index. The two senses shared the word until
// 2026-07-24 and the collision cost a confused exchange ("I asked for 60 epochs
// and got 6 frames", `msc/build-parameterization-findings-2026-07-24.md` §2).
// The CLI's `--epochs` is the erosion sense and keeps the word.

/// Canonical chain endpoints and spacing, in **micro-Ga integers** so that every
/// refinement below is exact. Floating-point construction would be the obvious
/// way to write this and would silently break memo reuse: each stage's `T_p` is
/// folded into its store key ( #form-complete-content-addressed-key ), so a
/// densified chain whose "same" stage differs by one ULP invalidates every
/// existing epoch-reduction memo instead of hitting it.
const HOT_END_UGA: i64 = 3_600_000;
const COOL_END_UGA: i64 = 2_600_000;
const CANONICAL_STEP_UGA: i64 = 200_000;

/// The canonical Abyssal cooling chain: an ordered sequence of **stages** from
/// the hot early-Abyssal (near a water-world) through the present-Abyssal anchor
/// to the cooling late-Abyssal — the stages the probe and the emergence view
/// walk. Ordered by *time* (increasing t = cooling), so land fraction along it is
/// the emergence trajectory. Deliberately a short, legible chain, not a fine grid;
/// [`cooling_stages_refined`] densifies it when a viewer wants more frames.
pub fn cooling_stages() -> Vec<Time> {
    cooling_stages_refined(0)
}

/// The cooling chain refined `refinements` times by **bisection**: each level
/// halves the spacing, so level $k$ has $5\cdot2^k + 1$ stages at
/// $0.2/2^k$ Ga apart.
///
/// **Every refinement is a superset of every coarser one, exactly.** That is the
/// property the whole thing turns on ( #form-time-indexed-stage-chains FE(7) and
/// its Working Note "densification must nest"; the power-of-two subset argument
/// is Gear–Wells', #detail-seam-precedents FE(5)). Because each stage's `T_p`
/// rides in its store key, a *non*-nested densification would not refine the
/// chain — it would silently orphan every memo on it and recompute the world.
/// The integer grid above makes the nesting bit-exact rather than approximate:
/// stage $i$ of level $k$ and stage $2i$ of level $k+1$ are the same `f64`.
pub fn cooling_stages_refined(refinements: u32) -> Vec<Time> {
    let step = CANONICAL_STEP_UGA >> refinements.min(MAX_REFINEMENTS);
    let mut out = Vec::new();
    let mut uga = HOT_END_UGA;
    while uga >= COOL_END_UGA {
        out.push(time_at_ga(uga as f64 / 1.0e6));
        uga -= step;
    }
    out
}

/// Bisection levels before the micro-Ga grid stops dividing evenly (200 000 →
/// 1 at level 17). Far past any legible frame count; it exists so the shift
/// above cannot silently produce a zero step.
const MAX_REFINEMENTS: u32 = 17;

/// How many stages a refinement level yields: $5\cdot2^k + 1$.
pub fn stage_count(refinements: u32) -> usize {
    5 * (1usize << refinements.min(MAX_REFINEMENTS)) + 1
}

/// The cheapest refinement level yielding **at least** `wanted` stages.
///
/// Requested counts that are not nested counts round *up* — the caller asking
/// for 60 frames gets 81, and the CLI says so. Refusing the request or
/// interpolating to exactly 60 would both cost the nesting property, and the
/// nesting property is worth more than the round number: it is the difference
/// between densifying a chain and recomputing the world.
pub fn refinements_for(wanted: usize) -> u32 {
    (0..=MAX_REFINEMENTS).find(|&k| stage_count(k) >= wanted).unwrap_or(MAX_REFINEMENTS)
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
        let stages = cooling_stages();
        for pair in stages.windows(2) {
            assert!(pair[0] < pair[1], "cooling_stages must be time-ordered");
            let (hot, cool) = (potential_temp_c(pair[0]), potential_temp_c(pair[1]));
            assert!(cool <= hot, "T_p must not rise as time advances ({hot} → {cool})");
        }
    }

    #[test]
    fn hot_end_is_hotter_than_present_and_bounded() {
        // The early-Abyssal end is hotter than the present anchor (thick, light
        // oceanic crust ⇒ shallow basins ⇒ flooding) and never exceeds the
        // declared Hadean ceiling where the oceanic ramp saturates.
        let hot = potential_temp_c(*cooling_stages().first().unwrap());
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

    #[test]
    fn canonical_chain_is_bit_identical_to_the_authored_literals() {
        // The migration guard. Before 2026-07-24 this chain was written as the
        // f64 literal array below; every epoch-reduction memo in every existing
        // store is keyed on `T_p.to_bits()` derived from it. The integer grid
        // must reproduce those doubles EXACTLY or the rename quietly orphans
        // every one of them — a cache miss storm that would look like a
        // performance regression, not like the key change it actually is.
        let authored: Vec<Time> =
            [3.6, 3.4, PRESENT_ABYSSAL_GA, 3.0, 2.8, 2.6].iter().map(|&ga| time_at_ga(ga)).collect();
        let built = cooling_stages();
        assert_eq!(built.len(), authored.len(), "canonical chain is still six stages");
        for (b, a) in built.iter().zip(&authored) {
            assert_eq!(
                potential_temp_c(*b).to_bits(),
                potential_temp_c(*a).to_bits(),
                "T_p bits must match the authored literal — they are in the store key"
            );
        }
    }

    #[test]
    fn every_refinement_contains_every_coarser_one_exactly() {
        // The nesting law ( #form-time-indexed-stage-chains FE(7)). Not "close
        // to": the coarse stages must appear in the fine chain as the SAME f64,
        // because approximate agreement is a key miss and a key miss is a
        // recompute of the whole chain.
        for k in 0..5u32 {
            let coarse = cooling_stages_refined(k);
            let fine = cooling_stages_refined(k + 1);
            assert_eq!(fine.len(), stage_count(k + 1));
            assert_eq!(coarse.len(), stage_count(k));
            for (i, c) in coarse.iter().enumerate() {
                assert_eq!(
                    potential_temp_c(fine[i * 2]).to_bits(),
                    potential_temp_c(*c).to_bits(),
                    "level {k} stage {i} must survive bisection bit-exactly"
                );
            }
        }
    }

    #[test]
    fn refinement_rounds_a_requested_frame_count_up_never_down() {
        // "60 frames" is not a nested count; the caller gets 81 and is told.
        // Rounding DOWN would silently under-deliver the request, which is the
        // failure the honest-count line in the CLI exists to prevent.
        assert_eq!(stage_count(0), 6);
        assert_eq!(stage_count(4), 81);
        assert_eq!(refinements_for(60), 4, "60 rounds up to the 81-stage level");
        assert_eq!(refinements_for(6), 0, "an exact nested count refines no further");
        assert_eq!(refinements_for(1), 0, "below the canonical chain stays canonical");
        for wanted in [2usize, 6, 7, 11, 12, 40, 81, 82] {
            assert!(stage_count(refinements_for(wanted)) >= wanted.max(6), "never fewer than asked");
        }
    }

    #[test]
    fn the_present_anchor_survives_every_refinement() {
        // The anchor is what makes the seed-only world *be* a stage on the
        // chain; a densification that moved it would fork "present" in two.
        let present = potential_temp_c(present_abyssal()).to_bits();
        for k in 0..6u32 {
            assert!(
                cooling_stages_refined(k).iter().any(|&t| potential_temp_c(t).to_bits() == present),
                "present-Abyssal anchor missing from refinement level {k}"
            );
        }
    }
}
