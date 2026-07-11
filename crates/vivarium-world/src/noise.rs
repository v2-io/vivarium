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
//!
//! **The world-seed is the first argument, everywhere** (2026-07-10). A vivium's
//! identity is `(seed, law)` (LEXICON §4; the manifest, `spec.rs`) — so the seed
//! is a *required* parameter of every primitive, not ambient state; forgetting it
//! is a compile error, never an under-keyed memo. `seed = 0` reproduces the
//! pre-seed world bit-for-bit (the fold contributes nothing at zero — pinned by
//! the `seed_zero_is_the_legacy_world` golden test), so the long-lived testbench
//! world and every probe baseline remain exactly what they were.

// Known domain tags (keep unique; collisions silently correlate fields):
//   0 continents fBm · 1 mountains fBm · 3 differential-uplift fBm ·
//   7 view ground mottle · 13 float-mode wobble · 17 storm schedule (temporal:
//   keys are (storm ordinal, draw index), the first time-keyed §8 field) ·
//   18 cobble pattern (metric coords) · 19 boulder pattern (metric coords).

/// SplitMix64 finalizer — strong avalanche, so a one-bit key change scrambles the
/// whole output (no lattice artifacts).
#[inline]
pub fn splitmix64(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash a world-seed + domain tag + a 2-D integer lattice key to a well-mixed
/// `u64`. The seed multiplier is odd (xorshift*'s constant) so the fold is a
/// bijection of the seed; `seed = 0` contributes nothing (legacy world).
#[inline]
pub fn hash2(seed: u64, domain: u32, ix: i64, iy: i64) -> u64 {
    let a = (ix as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    let b = (iy as u64).wrapping_mul(0xABBF_AA6E_9B7F_1B95);
    let d = (domain as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let s = seed.wrapping_mul(0x2545_F491_4F6C_DD1D);
    splitmix64(a ^ b ^ d ^ s)
}

/// Hash a world-seed + domain tag + a **3-D** integer lattice key — the
/// sphere-continuous twin of [`hash2`], for fields sampled on unit-sphere
/// directions (the spine prior since v2) rather than per-face `(u, v)`.
#[inline]
pub fn hash3(seed: u64, domain: u32, ix: i64, iy: i64, iz: i64) -> u64 {
    let a = (ix as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    let b = (iy as u64).wrapping_mul(0xABBF_AA6E_9B7F_1B95);
    let c = (iz as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    let d = (domain as u32 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let s = seed.wrapping_mul(0x2545_F491_4F6C_DD1D);
    splitmix64(a ^ b ^ c ^ d ^ s)
}

/// Map a hash to `[0, 1)` using its top 53 bits (full `f64` mantissa).
#[inline]
pub fn unit_f64(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// Deterministic lattice value in `[0, 1)` for an integer key.
#[inline]
pub fn hash01(seed: u64, domain: u32, ix: i64, iy: i64) -> f64 {
    unit_f64(hash2(seed, domain, ix, iy))
}

/// Deterministic offset in `[lo, hi]` keyed by an integer coordinate — Joseph's
/// "jitter baked into the coordinates" (§8). E.g. sub-voxel elevation jitter.
#[inline]
pub fn jitter(seed: u64, domain: u32, ix: i64, iy: i64, lo: f64, hi: f64) -> f64 {
    lo + hash01(seed, domain, ix, iy) * (hi - lo)
}

/// Smooth (smoothstep-interpolated) value noise in `[0, 1)` — spatially coherent,
/// so adjacent samples are close (a feature continues rather than speckling).
pub fn value_noise(seed: u64, domain: u32, x: f64, y: f64) -> f64 {
    let (x0, y0) = (x.floor(), y.floor());
    let (ix, iy) = (x0 as i64, y0 as i64);
    let (fx, fy) = (x - x0, y - y0);
    let sx = fx * fx * (3.0 - 2.0 * fx); // smoothstep
    let sy = fy * fy * (3.0 - 2.0 * fy);
    let c00 = hash01(seed, domain, ix, iy);
    let c10 = hash01(seed, domain, ix + 1, iy);
    let c01 = hash01(seed, domain, ix, iy + 1);
    let c11 = hash01(seed, domain, ix + 1, iy + 1);
    let a = c00 + (c10 - c00) * sx;
    let b = c01 + (c11 - c01) * sx;
    a + (b - a) * sy
}

/// 3-D smooth value noise in `[0, 1)` — trilinear over [`hash3`] lattice values
/// with smoothstep fades. Continuous everywhere in ℝ³, which is the whole point:
/// sampled on unit-sphere points there are no chart seams to cross.
pub fn value_noise_3d(seed: u64, domain: u32, x: f64, y: f64, z: f64) -> f64 {
    let (x0, y0, z0) = (x.floor(), y.floor(), z.floor());
    let (ix, iy, iz) = (x0 as i64, y0 as i64, z0 as i64);
    let (fx, fy, fz) = (x - x0, y - y0, z - z0);
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let at = |dx: i64, dy: i64, dz: i64| unit_f64(hash3(seed, domain, ix + dx, iy + dy, iz + dz));
    let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
    let c00 = lerp(at(0, 0, 0), at(1, 0, 0), sx);
    let c10 = lerp(at(0, 1, 0), at(1, 1, 0), sx);
    let c01 = lerp(at(0, 0, 1), at(1, 0, 1), sx);
    let c11 = lerp(at(0, 1, 1), at(1, 1, 1), sx);
    lerp(lerp(c00, c10, sy), lerp(c01, c11, sy), sz)
}

/// Fractal 3-D value noise in `[0, 1)` — [`fbm`]'s sphere-continuous twin.
pub fn fbm3(seed: u64, domain: u32, x: f64, y: f64, z: f64, octaves: u32, lacunarity: f64, gain: f64) -> f64 {
    let (mut freq, mut amp, mut sum, mut norm) = (1.0, 1.0, 0.0, 0.0);
    for o in 0..octaves {
        sum += amp
            * value_noise_3d(seed, domain.wrapping_add(o.wrapping_mul(0x9E37)), x * freq, y * freq, z * freq);
        norm += amp;
        freq *= lacunarity;
        amp *= gain;
    }
    if norm > 0.0 { sum / norm } else { 0.0 }
}

/// Fractal (fBm) value noise in `[0, 1)`: `octaves` of [`value_noise`] at rising
/// frequency and falling amplitude. Each octave uses a distinct domain so they are
/// independent. `lacunarity` ~2.0, `gain` ~0.5 are the usual defaults.
pub fn fbm(seed: u64, domain: u32, x: f64, y: f64, octaves: u32, lacunarity: f64, gain: f64) -> f64 {
    let (mut freq, mut amp, mut sum, mut norm) = (1.0, 1.0, 0.0, 0.0);
    for o in 0..octaves {
        sum += amp * value_noise(seed, domain.wrapping_add(o.wrapping_mul(0x9E37)), x * freq, y * freq);
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
        assert_eq!(fbm(9, 1, 3.2, 4.7, 5, 2.0, 0.5), fbm(9, 1, 3.2, 4.7, 5, 2.0, 0.5));
        assert_eq!(hash01(9, 7, -123, 456), hash01(9, 7, -123, 456));
    }

    #[test]
    fn seed_zero_is_the_legacy_world() {
        // Golden values captured from the pre-seed code (2026-07-10) — seed 0 must
        // reproduce them bit-for-bit, so the testbench world and every probe
        // baseline survive the seed threading unchanged.
        assert_eq!(hash01(0, 0, 0, 0), 8.83310808213642606e-1);
        assert_eq!(hash01(0, 3, 17, -4), 4.21163256717576373e-1);
        assert_eq!(fbm(0, 0, 1.234, 5.678, 4, 2.0, 0.5), 3.76766550031588376e-1);
        assert_eq!(fbm(0, 1, 3.2, 4.7, 7, 2.0, 0.5), 4.34910877336530055e-1);
    }

    #[test]
    fn seeds_are_independent_worlds() {
        // Different seeds decorrelate every field; and the seed fold is not a
        // trivial offset (two nearby seeds differ as much as two distant ones).
        assert_ne!(hash01(1, 0, 5, 5), hash01(2, 0, 5, 5));
        assert_ne!(fbm(1, 0, 1.5, 2.5, 4, 2.0, 0.5), fbm(0xDEAD_BEEF, 0, 1.5, 2.5, 4, 2.0, 0.5));
        let n = 32i64;
        let mut agree = 0;
        for i in 0..n {
            for j in 0..n {
                if (hash01(1, 3, i, j) - hash01(2, 3, i, j)).abs() < 0.001 {
                    agree += 1;
                }
            }
        }
        assert!(agree < 8, "seeds 1 and 2 correlate ({agree} near-equal of {})", n * n);
    }

    #[test]
    fn noise_3d_deterministic_ranged_coherent_and_seeded() {
        assert_eq!(
            fbm3(9, 0, 1.1, 2.2, 3.3, 5, 2.0, 0.5),
            fbm3(9, 0, 1.1, 2.2, 3.3, 5, 2.0, 0.5)
        );
        for k in 0..40 {
            let t = k as f64 * 0.37;
            let v = value_noise_3d(0, 0, t, t * 1.3, t * 0.7);
            assert!((0.0..1.0).contains(&v), "3d value {v}");
        }
        let a = value_noise_3d(0, 0, 5.0, 5.0, 5.0);
        let b = value_noise_3d(0, 0, 5.001, 5.0, 5.0);
        assert!((a - b).abs() < 0.01, "not coherent: {a} vs {b}");
        assert_ne!(
            value_noise_3d(1, 0, 5.5, 5.5, 5.5),
            value_noise_3d(2, 0, 5.5, 5.5, 5.5),
            "seeds must decorrelate 3d fields"
        );
    }

    #[test]
    fn in_unit_range() {
        for j in -3..3 {
            for i in -3..3 {
                let (x, y) = (i as f64 * 1.7 + 0.3, j as f64 * 1.3 - 0.2);
                let v = value_noise(0, 0, x, y);
                let f = fbm(0, 0, x, y, 6, 2.0, 0.5);
                assert!((0.0..1.0).contains(&v), "value {v}");
                assert!((0.0..1.0).contains(&f), "fbm {f}");
            }
        }
    }

    #[test]
    fn spatially_coherent() {
        // A tiny step changes the value only a little (continuity ⇒ features connect).
        let a = value_noise(0, 0, 10.0, 10.0);
        let b = value_noise(0, 0, 10.001, 10.0);
        assert!((a - b).abs() < 0.01, "not coherent: {a} vs {b}");
    }

    #[test]
    fn domains_are_independent() {
        assert_ne!(hash01(0, 1, 5, 5), hash01(0, 2, 5, 5));
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
                let v = hash01(0, 3, i, j);
                sum += v;
                if (v - hash01(0, 3, i + 1, j)).abs() < 0.001 {
                    same_ish += 1;
                }
            }
        }
        let mean = sum / (n * n) as f64;
        assert!((mean - 0.5).abs() < 0.03, "mean {mean} off 0.5");
        assert!(same_ish < 20, "too many near-equal neighbours ({same_ish}) — poor mixing");
    }
}
