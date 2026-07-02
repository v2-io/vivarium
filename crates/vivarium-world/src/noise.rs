//! Coordinate-hashed stochasticity — the `DESIGN-REDUX.md` §8 primitive.
//!
//! **All randomness in the world is a stateless, well-mixed hash of a stable key**
//! (never a shared mutable stream): deterministic, parallel-safe, and free of the
//! materialization-order dependence that would break memoization (§11–12). The
//! finalizer is SplitMix64 because **hash quality is a correctness property here**
//! — a naive `x·P mod Q` scheme has axis-aligned lattice correlations, i.e. the
//! very artificial structure the jitter exists to remove.
//!
//! `domain` is a field tag: one function yields infinitely many *independent*
//! fields (elevation vs. ore vs. vegetation placement) keyed by `(domain, x, y)`.
//! Continuous [`value_noise`]/[`fbm`] give spatial *coherence* (a vein continues
//! across cells); modulate their amplitude by a macro field to get §10's
//! "coherent detail under a macro constraint."

// Known domain tags (keep unique; collisions silently correlate fields):
//   0 continents fBm · 1 mountains fBm · 3 differential-uplift fBm ·
//   7 view ground mottle · 13 float-mode wobble · 17 storm schedule (temporal:
//   keys are (storm ordinal, draw index), the first time-keyed §8 field).

/// SplitMix64 finalizer — strong avalanche, so a one-bit key change scrambles the
/// whole output (no lattice artifacts).
#[inline]
pub fn splitmix64(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash a domain tag + a 2-D integer lattice key to a well-mixed `u64`.
#[inline]
pub fn hash2(domain: u32, ix: i64, iy: i64) -> u64 {
    let a = (ix as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    let b = (iy as u64).wrapping_mul(0xABBF_AA6E_9B7F_1B95);
    let d = (domain as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    splitmix64(a ^ b ^ d)
}

/// Map a hash to `[0, 1)` using its top 53 bits (full `f64` mantissa).
#[inline]
pub fn unit_f64(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// Deterministic lattice value in `[0, 1)` for an integer key.
#[inline]
pub fn hash01(domain: u32, ix: i64, iy: i64) -> f64 {
    unit_f64(hash2(domain, ix, iy))
}

/// Deterministic offset in `[lo, hi]` keyed by an integer coordinate — Joseph's
/// "jitter baked into the coordinates" (§8). E.g. sub-voxel elevation jitter.
#[inline]
pub fn jitter(domain: u32, ix: i64, iy: i64, lo: f64, hi: f64) -> f64 {
    lo + hash01(domain, ix, iy) * (hi - lo)
}

/// Smooth (smoothstep-interpolated) value noise in `[0, 1)` — spatially coherent,
/// so adjacent samples are close (a feature continues rather than speckling).
pub fn value_noise(domain: u32, x: f64, y: f64) -> f64 {
    let (x0, y0) = (x.floor(), y.floor());
    let (ix, iy) = (x0 as i64, y0 as i64);
    let (fx, fy) = (x - x0, y - y0);
    let sx = fx * fx * (3.0 - 2.0 * fx); // smoothstep
    let sy = fy * fy * (3.0 - 2.0 * fy);
    let c00 = hash01(domain, ix, iy);
    let c10 = hash01(domain, ix + 1, iy);
    let c01 = hash01(domain, ix, iy + 1);
    let c11 = hash01(domain, ix + 1, iy + 1);
    let a = c00 + (c10 - c00) * sx;
    let b = c01 + (c11 - c01) * sx;
    a + (b - a) * sy
}

/// Fractal (fBm) value noise in `[0, 1)`: `octaves` of [`value_noise`] at rising
/// frequency and falling amplitude. Each octave uses a distinct domain so they are
/// independent. `lacunarity` ~2.0, `gain` ~0.5 are the usual defaults.
pub fn fbm(domain: u32, x: f64, y: f64, octaves: u32, lacunarity: f64, gain: f64) -> f64 {
    let (mut freq, mut amp, mut sum, mut norm) = (1.0, 1.0, 0.0, 0.0);
    for o in 0..octaves {
        sum += amp * value_noise(domain.wrapping_add(o.wrapping_mul(0x9E37)), x * freq, y * freq);
        norm += amp;
        freq *= lacunarity;
        amp *= gain;
    }
    if norm > 0.0 { sum / norm } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        assert_eq!(fbm(1, 3.2, 4.7, 5, 2.0, 0.5), fbm(1, 3.2, 4.7, 5, 2.0, 0.5));
        assert_eq!(hash01(7, -123, 456), hash01(7, -123, 456));
    }

    #[test]
    fn in_unit_range() {
        for j in -3..3 {
            for i in -3..3 {
                let (x, y) = (i as f64 * 1.7 + 0.3, j as f64 * 1.3 - 0.2);
                let v = value_noise(0, x, y);
                let f = fbm(0, x, y, 6, 2.0, 0.5);
                assert!((0.0..1.0).contains(&v), "value {v}");
                assert!((0.0..1.0).contains(&f), "fbm {f}");
            }
        }
    }

    #[test]
    fn spatially_coherent() {
        // A tiny step changes the value only a little (continuity ⇒ features connect).
        let a = value_noise(0, 10.0, 10.0);
        let b = value_noise(0, 10.001, 10.0);
        assert!((a - b).abs() < 0.01, "not coherent: {a} vs {b}");
    }

    #[test]
    fn domains_are_independent() {
        assert_ne!(hash01(1, 5, 5), hash01(2, 5, 5));
    }

    #[test]
    fn well_mixed() {
        // Mean over a lattice ~0.5, and neighbours are decorrelated (avalanche):
        // no giant runs of near-equal adjacent values (the lattice-artifact failure).
        let n = 64i64;
        let mut sum = 0.0;
        let mut same_ish = 0;
        for i in 0..n {
            for j in 0..n {
                let v = hash01(3, i, j);
                sum += v;
                if (v - hash01(3, i + 1, j)).abs() < 0.001 {
                    same_ish += 1;
                }
            }
        }
        let mean = sum / (n * n) as f64;
        assert!((mean - 0.5).abs() < 0.03, "mean {mean} off 0.5");
        assert!(same_ish < 20, "too many near-equal neighbours ({same_ish}) — poor mixing");
    }
}
