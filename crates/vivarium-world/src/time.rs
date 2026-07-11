//! Time as a single canonical integer — the drift-free key dimension every
//! memoized tier is indexed by (`doc/design/DESIGN-REDUX.md` §11). One `i64` counting
//! **deciseconds** (1/10 s): fine enough for day/night, and signed it spans
//! ~±29 billion years — deeper than the age of the universe, so overflow is a
//! non-issue at this granularity.
//!
//! **Origin (`t = 0`) is the onset of the Holocene** — 11,700 years before
//! 2000 CE, the formally-ratified base of the Holocene / Greenlandian GSSP. That
//! instant begins [`Epoch::Modern`]; present day is ~`t + 11,700 yr`; deep past is
//! negative. The anchor is a coarse **reference state** (nominal solar output,
//! hydrosphere water total, cloud-cover sanity), *not* a claim about detailed
//! geography or weather at that time.
//!
//! Time here is **uniform SI seconds** — no leap seconds, no UTC (deterministic
//! replay wants a monotone count, not civil time). The year used for seasons is
//! the **mean tropical year** (`SECONDS_PER_YEAR`); calendar detail, if ever
//! needed, is a derived view, never the canonical count.

/// Deciseconds — the canonical tick — per second.
pub const DECISECONDS_PER_SECOND: i64 = 10;
/// Seconds per solar day.
pub const SECONDS_PER_DAY: i64 = 86_400;
/// Mean tropical year, in seconds (≈365.2422 d). Used only for the human-facing
/// year/day *accessors* below — never as a memo-key unit (keys hash the `i64`).
pub const SECONDS_PER_YEAR: f64 = 365.2422 * 86_400.0;

/// A signed count of deciseconds from the Holocene-onset origin. Negative = deep
/// past (pre-Holocene), positive = later. **This integer is what memo keys hash**
/// — the float accessors are derived views, never the source of truth.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Time(pub i64);

impl Time {
    /// Holocene onset — the reference instant that begins [`Epoch::Modern`].
    pub const ORIGIN: Time = Time(0);

    pub const fn from_deciseconds(ds: i64) -> Self { Time(ds) }

    /// Construct from a (possibly fractional) number of years after the origin.
    pub fn from_years(years: f64) -> Self {
        Time((years * SECONDS_PER_YEAR * DECISECONDS_PER_SECOND as f64).round() as i64)
    }

    pub fn seconds(self) -> f64 { self.0 as f64 / DECISECONDS_PER_SECOND as f64 }
    pub fn years(self) -> f64 { self.seconds() / SECONDS_PER_YEAR }

    /// Fraction through the day in `[0, 1)` — `0.0` = local midnight, `0.5` = noon.
    /// (The absolute phase at the origin is a convention; only relative day/night
    /// is meaningful at this fidelity.)
    pub fn day_fraction(self) -> f64 {
        let day = SECONDS_PER_DAY as f64;
        self.seconds().rem_euclid(day) / day
    }

    /// Fraction through the year in `[0, 1)`, `0.0` at the origin's anniversary.
    pub fn year_fraction(self) -> f64 { self.years().rem_euclid(1.0) }
}

/// A coarse, **qualitative** evolutionary stage — a named reference-state anchor,
/// *not* a uniform time block. Real geological boundaries are non-uniform, so that
/// banding is deferred until we actually simulate deep time; for now the only
/// anchor that exists is the one gameplay needs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Epoch {
    /// The Holocene: `t >= 0`. Earth-like, pre-industrial reference world.
    Modern,
}
