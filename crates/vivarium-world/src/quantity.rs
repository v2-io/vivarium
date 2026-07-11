//! The rich value type used **at seams** — flux/coupler interfaces, storage, and
//! public API — where honesty about what a number *is* matters and the volume of
//! values is low. Hot inner loops take raw `f64` under a boundary contract; this
//! is the "rich at seams, raw in loops" decision (`doc/design/DESIGN-REDUX.md` §12), forced
//! by cost: a rich struct *per voxel per tick* is unaffordable in erosion/hydro.
//!
//! A [`Quantity`] carries what a bare `f64` cannot: its **unit** and whether it is
//! **exact** or an approximation. It is deliberately **runtime**-tagged, not
//! phantom-typed, because the honesty metadata varies per value and is meant to
//! *grow*: interval/moments (§5), jitter-state (§8), and provenance are the next
//! rungs. Start minimal; add them behind this same type without churning callers.

/// The dimension of a [`Quantity`] as SI base-unit exponents.
///
/// Stored as exponents (not a flat enum) so units **compose under arithmetic** —
/// multiply ⇒ add exponents, divide ⇒ subtract — and derived units like
/// W/m² = kg·s⁻³ or density kg·m⁻³ fall out for free instead of each needing a
/// bespoke variant. (The research pass flagged that a flat enum doesn't compose
/// and would explode as tiers introduce m/s, kg/m³, kg/(m²·s), …) Ampere and
/// candela are omitted until an electrical/luminous tier needs them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {
    pub m: i8,   // metre — length
    pub kg: i8,  // kilogram — mass
    pub s: i8,   // second — time
    pub k: i8,   // kelvin — temperature
    pub mol: i8, // mole — amount of substance (chemistry / biogeochemical cycles)
}

impl Unit {
    pub const fn new(m: i8, kg: i8, s: i8, k: i8, mol: i8) -> Self {
        Self { m, kg, s, k, mol }
    }

    pub const ONE: Unit = Unit::new(0, 0, 0, 0, 0);
    pub const METRE: Unit = Unit::new(1, 0, 0, 0, 0);
    pub const KILOGRAM: Unit = Unit::new(0, 1, 0, 0, 0);
    pub const SECOND: Unit = Unit::new(0, 0, 1, 0, 0);
    pub const KELVIN: Unit = Unit::new(0, 0, 0, 1, 0);
    pub const MOLE: Unit = Unit::new(0, 0, 0, 0, 1);
    /// W/m² = kg·s⁻³ — the astronomy tier's insolation flux.
    pub const WATT_PER_M2: Unit = Unit::new(0, 1, -3, 0, 0);
    /// kg·m⁻³ — mass density.
    pub const KG_PER_M3: Unit = Unit::new(-3, 1, 0, 0, 0);
    /// Pa = kg·m⁻¹·s⁻² — pressure / stress (cohesion, overburden, incision threshold).
    pub const PASCAL: Unit = Unit::new(-1, 1, -2, 0, 0);
    /// m² — area (permeability).
    pub const M2: Unit = Unit::new(2, 0, 0, 0, 0);

    /// Dimension of a product (add exponents).
    pub const fn mul(self, o: Unit) -> Unit {
        Unit::new(self.m + o.m, self.kg + o.kg, self.s + o.s, self.k + o.k, self.mol + o.mol)
    }
    /// Dimension of a quotient (subtract exponents).
    pub const fn div(self, o: Unit) -> Unit {
        Unit::new(self.m - o.m, self.kg - o.kg, self.s - o.s, self.k - o.k, self.mol - o.mol)
    }
}

/// Whether a value is known exactly or is an approximation. The unum "ubit" made
/// concrete (`doc/design/DESIGN-REDUX.md` §9): a value knows its own truth-status, which is
/// the mechanism the fidelity invariant's "known, bounded deficiencies" needs.
/// Starts as a bit; an `Interval`/moments rung (via `inari`) comes when a consumer
/// needs it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exactness {
    /// Exact within `f64` rounding — a measured/derived constant or a resolved value.
    Exact,
    /// An approximation: output of a lower-fidelity model, or an unresolved macro
    /// statistic standing in for detail not yet materialized.
    Approx,
}

impl Exactness {
    /// Combine two operands' exactness: exact only if *both* are exact — any
    /// approximation taints the result.
    pub const fn and(self, o: Exactness) -> Exactness {
        match (self, o) {
            (Exactness::Exact, Exactness::Exact) => Exactness::Exact,
            _ => Exactness::Approx,
        }
    }
}

/// A number that knows what it *is*. Used at seams; cross into a hot loop with
/// [`Quantity::raw`], and back with [`Quantity::exact`] / [`Quantity::approx`].
/// Arithmetic composes units and taints exactness ([`std::ops::Mul`]/[`Div`]).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
    pub exact: Exactness,
}

impl Quantity {
    pub const fn exact(value: f64, unit: Unit) -> Self {
        Self { value, unit, exact: Exactness::Exact }
    }
    pub const fn approx(value: f64, unit: Unit) -> Self {
        Self { value, unit, exact: Exactness::Approx }
    }
    /// Extract the raw magnitude for a hot loop, asserting the expected unit. The
    /// assert **is** the boundary contract: unit correctness is checked once at the
    /// seam (debug builds), then the loop runs on bare `f64` with zero overhead.
    #[inline]
    pub fn raw(self, expect: Unit) -> f64 {
        debug_assert_eq!(self.unit, expect, "unit mismatch crossing a loop boundary");
        self.value
    }
}

impl std::ops::Mul for Quantity {
    type Output = Quantity;
    fn mul(self, o: Quantity) -> Quantity {
        Quantity { value: self.value * o.value, unit: self.unit.mul(o.unit), exact: self.exact.and(o.exact) }
    }
}

impl std::ops::Div for Quantity {
    type Output = Quantity;
    fn div(self, o: Quantity) -> Quantity {
        Quantity { value: self.value / o.value, unit: self.unit.div(o.unit), exact: self.exact.and(o.exact) }
    }
}
