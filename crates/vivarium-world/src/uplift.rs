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
//! **v1 (2026-07-28): the rate is the column's own time-derivative.** The rate
//! field is the finite difference of the isostasy read along the mantle-thermal
//! cooling trajectory: `rate(cell) = freeboard(T_p − δ) − freeboard(T_p)` for a
//! declared per-epoch cooling step δ ([`TP_COOLING_PER_EPOCH_C`]). This retires
//! the v0 fBm stand-in (an arbitrary magnitude × fake geography) for a field
//! whose **geography is real** — craton interiors thicken and rise as accretion
//! advances, growing rims sweep outward, suture belts rise where cratons
//! collide, and cooling ocean floor **subsides** — and whose one remaining
//! arbitrary number is the epoch↔cooling scale, exactly the `epoch ↔ years`
//! arbitrariness the tree already carries, now stated once.
//!
//! Three properties the v0 field could not have, each convictable:
//! - **Signed.** Basins subside while land rises. The v0 strictly-positive
//!   field raised the entire seafloor monotonically every epoch — surfaced
//!   independently by the explorer's change paint (88–91 % of all cells
//!   rising, the whole ocean shell up-drifting), and structurally incapable of
//!   expressing subsidence (`DECISIONS[uplift-is-structurally-incapable-of-keeping-its-promise]`).
//! - **Zero-mean, exactly, on the reference grid.** Freeboard is zero-mean at
//!   every `T_p` (global mass balance), so the difference of two reads is
//!   zero-mean: rise here IS subsidence there, now in the *driver*, not only
//!   in the stock.
//! - **Coherent at continent wavelength.** The v0 fBm varied at ~5 km — noise
//!   erosion could not integrate drainage against. This field's structure is
//!   the column's: margins, zonation gradients, sutures.
//!
//! Physics tier stays **Low** — a derivative of a declared crude chain is
//! declared-crude too. It is a *diagnostic-grade driver* under
//! `#form-isostasy-column` (a height-rate is never the article that earns
//! land); what changed is that the diagnostic now reads the real chain instead
//! of inventing a texture.

use crate::lithosphere;
use crate::sphere::{CellId, Face};

/// Declared mantle cooling per erosion epoch (°C) — the epoch↔world-time scale
/// as one number (`ASSUMPTIONS.md` "cooling per erosion epoch"). Sized so the
/// rate over land is of the same order as the retired v0 base rate
/// (~0.5 m/epoch) — measured, not derived; see the ASSUMPTIONS row. Setting
/// this to 0 makes uplift an exact no-op, structurally rather than by flag.
pub const TP_COOLING_PER_EPOCH_C: f64 = 0.05;

/// The rock-uplift rate at one cell (m/epoch): the finite difference of the
/// isostasy read across one epoch's declared cooling. Pure function of
/// (seed, cell) — fated, memoizable, replayable. Signed; zero-mean on the
/// reference sample (see module doc).
pub fn uplift_rate_m_per_epoch(seed: u64, cell: CellId) -> f64 {
    let tp = lithosphere::MANTLE_TP_C;
    lithosphere::freeboard_m_at_tp(seed, cell, tp - TP_COOLING_PER_EPOCH_C)
        - lithosphere::freeboard_m_at_tp(seed, cell, tp)
}

/// A tile of rock-uplift rates (m/epoch), row-major `nx × nx` over `face` cells
/// at `level` from origin `(oi, oj)` — the field erosion consumes each epoch.
/// Indices past the face chart are clamped (halo windows at cube edges;
/// cross-face resampling is still open).
pub fn uplift_rate_tile(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize) -> Vec<f32> {
    let last = (1u32 << level).saturating_sub(1);
    let mut out = Vec::with_capacity(nx * nx);
    for j in 0..nx as u32 {
        for i in 0..nx as u32 {
            let cell = CellId::from_face_ij(
                face,
                oi.saturating_add(i).min(last),
                oj.saturating_add(j).min(last),
                level,
            );
            out.push(uplift_rate_m_per_epoch(seed, cell) as f32);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::CubeCoord;

    /// The properties the module doc claims, on a whole-sphere sample.
    #[test]
    fn rate_is_signed_and_near_zero_mean() {
        let seed = 7;
        let n = 48usize;
        let (mut sum, mut count) = (0.0f64, 0usize);
        let (mut pos, mut neg) = (0usize, 0usize);
        for fi in 0..6u8 {
            let face = Face::from_index(fi);
            for j in 0..n {
                for i in 0..n {
                    let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                    let cell = CubeCoord { face, u, v }.cell(7);
                    let r = uplift_rate_m_per_epoch(seed, cell);
                    sum += r;
                    count += 1;
                    if r > 0.0 {
                        pos += 1;
                    } else if r < 0.0 {
                        neg += 1;
                    }
                }
            }
        }
        // Signed: both uplift and subsidence exist — the v0 field's convicted
        // limitation (strictly positive everywhere) is structurally gone.
        assert!(pos > 0 && neg > 0, "rate field must be signed: {pos} rising, {neg} subsiding");
        // Near-zero mean on this (coarser-than-reference) sample: the exact
        // zero-mean holds on the reference grid; a different sampling grain
        // leaves a residual far below the field's own scale.
        let mean = sum / count as f64;
        assert!(mean.abs() < 0.05, "mean rate ≈ 0 (rise here is subsidence there): {mean}");
    }

    #[test]
    fn deterministic() {
        let a = uplift_rate_tile(7, Face::from_index(1), 9, 100, 200, 16);
        let b = uplift_rate_tile(7, Face::from_index(1), 9, 100, 200, 16);
        assert_eq!(a, b, "fated: same (seed, coords) → same rates");
    }
}
