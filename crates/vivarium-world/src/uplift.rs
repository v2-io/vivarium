//! The uplift nomos — the tectonic driver, as its own article of law.
//!
//! **Why this is a separate nomos** (Joseph, 2026-07-12). "What lifts the land?"
//! should be one legible, auditable, swappable thing — not a term buried inside
//! erosion. A world eroding for geological time grows macro relief only if
//! something *uplifts* against the erosion; with no uplift the honest endpoint is
//! a peneplain (measured: 400 epochs of erosion on a flat prior stays flat; the
//! same with uplift builds ~1.4 km of dissected relief). So uplift is the macro
//! driver, and it deserves to be declared, consumed, and audited like any other:
//! erosion `consumes` [`crate::flux::ROCK_UPLIFT_RATE`], this nomos `produces` it,
//! and the flux web shows the dependency. Swap this kernel — no-op, constant,
//! or a real mantle-driven field later — and erosion recomputes by key.
//!
//! **What v0 is, honestly: ~30 lines of crude stand-in.** A single declared rate
//! constant (`ASSUMPTIONS.md` "uplift rate") times a low-frequency fBm, so the
//! land rises *differentially* — blocks lift at ~0.25×–1.75× the base rate and
//! features tilt, as real landscapes do. This is **exactly the code that used to
//! live *inside* the erosion kernel** (the old `uplift_w` weight × `uplift_m`
//! scalar), lifted out into its own nomos — consolidation, not new physics. Its
//! epistemic tag is honest: physics `None` (no mechanics — a placeholder curve,
//! not mantle convection), a `#mech stand-in`. The real driver is the
//! thermal-initial-topography / plume-upwelling work (`TODO.md`); this is the declared rung
//! beneath it, and flipping the rate to 0 makes it a clean no-op.

use crate::noise;
use crate::sphere::{CellId, Face};

/// Base rock-uplift rate (m per erosion epoch), before the differential weight.
/// Crude and uncalibrated — a declared placeholder (`ASSUMPTIONS.md` "uplift
/// rate"), not a measured tectonic rate. Set to 0.0 for a no-op world.
pub const UPLIFT_RATE_M_PER_EPOCH: f32 = 0.5;

/// The rock-uplift rate at one cell (m/epoch): the base rate modulated by a
/// low-frequency fBm so uplift is differential (`~0.25×–1.75×`). Pure function of
/// (seed, cell) — fated noise, memoizable, replayable. This is the whole kernel.
pub fn uplift_rate_m_per_epoch(seed: u64, cell: CellId) -> f64 {
    let c = cell.to_cube();
    // Low-frequency fBm (λ ≈ 5 km domain, same parameters the erosion kernel used
    // internally before this was lifted out) → a differential-uplift weight.
    let f = noise::fbm(seed, 3, (c.u + 1.0) * 2000.0, (c.v + 1.0) * 2000.0, 3, 2.0, 0.5);
    let weight = 0.25 + 1.5 * f; // ~0.25× .. 1.75×
    UPLIFT_RATE_M_PER_EPOCH as f64 * weight
}

/// A tile of rock-uplift rates (m/epoch), row-major `nx × nx` over `face` cells
/// at `level` from origin `(oi, oj)` — the field erosion consumes each epoch.
pub fn uplift_rate_tile(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(face, oi + i, oj + j, level);
            out.push(uplift_rate_m_per_epoch(seed, cell) as f32);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_is_differential_and_bounded() {
        // The weight is ~0.25×..1.75×, so rates stay within that band of the base.
        let (lo, hi) = (0.25 * UPLIFT_RATE_M_PER_EPOCH, 1.75 * UPLIFT_RATE_M_PER_EPOCH);
        let tile = uplift_rate_tile(0, Face::from_index(2), 19, 1000, 2000, 24);
        assert!(tile.iter().all(|&r| r >= lo - 1e-3 && r <= hi + 1e-3), "rates within the differential band");
        // Differential means the field is not flat (some spread across the tile).
        let (mn, mx) = tile.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), &r| (a.min(r), b.max(r)));
        assert!(mx - mn > 1e-4, "uplift is differential — the field varies across the tile");
    }

    #[test]
    fn deterministic() {
        let a = uplift_rate_tile(7, Face::from_index(1), 19, 500, 600, 16);
        let b = uplift_rate_tile(7, Face::from_index(1), 19, 500, 600, 16);
        assert_eq!(a, b, "fated noise: same (seed, coords) → same rates");
    }
}
