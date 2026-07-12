//! The flux-quantity vocabulary — the alphabet the nomoi couple through.
//!
//! A multiscale, heterogeneous world is assembled from nomoi that never share
//! mutable state: one hands another a **fluxed quantity** and nothing else
//! (`doc/theory/multiscale-seams.md`; the coupling contract of ARCHITECTURE §9).
//! The contract is exactly one string-match: a nomos `produces` a quantity, a
//! downstream nomos `consumes` the same quantity, and *that matched pair is the
//! only thing they agree on*. So the strings must come from **one shared
//! vocabulary**, or a typo becomes a silently-broken edge that a human reads as
//! "obviously rain feeds erosion" while the audit sees nothing connect them.
//!
//! This module is that vocabulary. Both [`crate::nomotheke::Promise::quantity`]
//! (produced) and [`crate::nomotheke::NomosDecl::consumes`] (needed) reference
//! these constants, and [`is_in_vocabulary`] backs a test that fails the build
//! if any declaration names a quantity not listed here — the same
//! typo-can't-hide discipline the `ASSUMPTIONS.md` anchors already enforce for
//! magic constants.
//!
//! **Scope discipline (deliberate).** Only quantities that a *real* nomos today
//! produces or consumes live here. The imagined near-future chain
//! (atmosphere-water, sediment-flux, insolation as a coupled input, …) is *not*
//! pre-populated — vocabulary is not built ahead of a nomos that speaks it.
//! The one apparent exception, [`PRECIPITATION`], is not an exception: erosion
//! and water both *consume* it today (erosion as an implicit uniform-rain
//! assumption in its drainage-area discharge; water as the documented ~10× rain
//! fudge, `ASSUMPTIONS.md` "rain rate"). No nomos *produces* it yet. Naming it
//! here is what turns "rain without a sky" from an unwritten assumption into a
//! mechanical audit finding (`crate::audit`; `doc/plan/regula-conformance-design.md` §3).
//!
//! **Not yet declared here (honest):** the *statistic + exactness contract* a
//! consumer needs from a producer (`… needs mean at-least L19`,
//! `VIVARIA-DECLARATIVE-FRONTIER.md`, spike 2). That is a proposal, not a
//! decision; when the seam actually needs cross-scale statistic-matching, the
//! consume entries grow from bare strings into `{quantity, statistic, level}`
//! — the vocabulary term stays the same, so this module is forward-compatible
//! with that extension by construction.

/// The surface-prior elevation field the spine produces (m). Consumed by erosion.
pub const SURFACE_ELEVATION: &str = "surface elevation field (m)";

/// The rock-uplift-rate field the uplift nomos produces (m/epoch) — the tectonic
/// driver erosion carves against. Its own separate nomos (`crate::uplift`) so
/// "what lifts the land?" is one legible, auditable, swappable article of law,
/// not a hidden term inside erosion. Consumed by erosion.
pub const ROCK_UPLIFT_RATE: &str = "rock uplift rate field (m/epoch)";

/// The eroded bed the fluvial nomos produces (m). Consumed by water.
pub const ERODED_SURFACE: &str = "eroded surface elevation field (m)";

/// The standing-water depth the water nomos produces (m). A terminal output today.
pub const STANDING_WATER_DEPTH: &str = "standing water depth field (m)";

/// Atmospheric water as a global-equivalent depth (m water-equivalent). Produced
/// by the hydrosphere reservoir (`crate::hydrosphere`) — the conserved stock a
/// climate/precipitation nomos will draw rain from. The honest root beneath
/// precipitation: this stock traces to a declared fraction of planetary mass.
pub const ATMOSPHERE_WATER: &str = "atmosphere water (m w.e.)";

/// Precipitation reaching the surface (m/yr). **Consumed** by erosion and water;
/// **produced by no nomos yet** — the live "rain without a sky" specimen the
/// requisite audit surfaces (its principled producer is the future atmosphere
/// reservoir → water-cycle chain, `TODO.md` §"The water system, decomposed").
pub const PRECIPITATION: &str = "precipitation (m/yr)";

/// Every fluxed quantity a nomos may name. A `produces`/`consumes` string not in
/// this list is a typo (or an undeclared vocabulary addition) — a test convicts
/// it, so a broken coupling edge cannot masquerade as an obvious one.
pub const VOCABULARY: &[&str] = &[
    SURFACE_ELEVATION,
    ROCK_UPLIFT_RATE,
    ATMOSPHERE_WATER,
    ERODED_SURFACE,
    STANDING_WATER_DEPTH,
    PRECIPITATION,
];

/// Is this string a known fluxed quantity? Backs the vocabulary-closure test.
pub fn is_in_vocabulary(quantity: &str) -> bool {
    VOCABULARY.contains(&quantity)
}
