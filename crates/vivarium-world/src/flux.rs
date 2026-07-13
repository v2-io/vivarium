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

/// **Seeded crustal asymmetry** (dimensionless) — the fated heterogeneity the world is
/// divided by. Produced by the `noise` nomos (the KRNG: a pure function of (seed, key)),
/// consumed by the surface prior which builds relief on it. This keeps the ordinum's
/// Phase-2 `seeded-asymmetry` promise — found undeclared 2026-07-12 by the ordinum audit.
pub const SEEDED_ASYMMETRY: &str = "seeded crustal asymmetry (dimensionless)";

/// The surface-prior elevation field the initial-topography produces (m). Consumed by erosion.
pub const SURFACE_ELEVATION: &str = "surface elevation field (m)";

/// Top-of-atmosphere insolation (W/m²) — produced by the `planet` nomos from tilt,
/// spin and orbit. The ordinum's Phase-1 `axial-rhythms` promise is kept by this.
pub const INSOLATION: &str = "insolation (W/m2)";

/// **Emerged land** — surface standing above sea level (m). The ordinum's Abyssal
/// `emerged-land` promise, as a fluxed quantity: *"meaningful non-volcanic land above
/// sea level — delivered by uplift / proto-tectonic processes, never an initial
/// condition"* (`charge[emergent-land] :tag gate`).
///
/// **Fluvial erosion CONSUMES this** — you cannot carve a landscape that is under
/// water. Nothing produces it yet, which is the honest truth: the world is in its
/// Phase-1 `water-covered-surface` state, and Abyssal has not yet earned its land.
/// Declaring the need is what makes the flux web tell that truth: the audit reports
/// erosion's requirement as **UNMET**, so the world is mechanically unrunnable for
/// erosion rather than silently no-op'ing on seabed (which is exactly what it was
/// doing — see `examples/sea_level_probe.rs` and the fluvial-test footprint story).
///
/// This is the ordinum GOVERNING the flux web (Joseph, 2026-07-12): a promise the
/// ladder has not kept becomes an unmet need, and a world that depends on it cannot
/// validly run.
pub const EMERGED_LAND: &str = "emerged land — surface above sea level (m)";

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
    SEEDED_ASYMMETRY,
    SURFACE_ELEVATION,
    INSOLATION,
    EMERGED_LAND,
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
