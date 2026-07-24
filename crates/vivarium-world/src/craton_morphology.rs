//! The craton **morphology probe** — a renderer-free regime instrument that
//! convicts the *shape class* of the cratonization field, not just its area
//! (`#norm-regime-probes`, `#norm-probe-sensitivity`; `#form-isostasy-column`
//! FE(8)).
//!
//! **The fault this catches.** A threshold slice through a scale-free (fBm)
//! field is *percolation speckle*: at the land threshold it fragments into many
//! tiny, ragged components — the wrong morphology class for cratons, however
//! honest the heights (Joseph, on the live globe: the land "looks distinctly
//! fBm-like"). A fated **nucleation-and-growth** field
//! (`lithosphere::craton_weight`) instead produces a FEW coherent cratons of
//! characteristic scale. This probe measures the difference and can *fail*:
//! - the nucleation-growth field passes (few components, high compactness);
//! - the retired fBm field (`lithosphere::craton_weight_fbm_speckle_known_bad`,
//!   kept expressly as the known-bad) FAILS — a probe that cannot fail on a
//!   historical broken configuration is not yet a probe for that fault class
//!   (`#norm-probe-sensitivity` FE(2)).
//!
//! **The load-bearing discriminator — fragmentation at a MATCHED land budget.**
//! Connected-component count is confounded by area: the retired fBm field's own
//! cratonized fraction swings 14–40 % by seed (percolating into one giant blob
//! at high area — itself a symptom of uncalibrated speckle). So the fair,
//! area-controlled control is to threshold the fBm field to the *same*
//! cratonized fraction as the nucleation-growth field, then compare how many
//! connected components that identical land budget breaks into. Measured (L5–6):
//! nucleation-growth carries a ~12 % budget in **5–11** coherent cratons; the
//! same budget of scale-free fBm shatters into **14–25** — a ~2–3× separation
//! that isolates *shape* from *calibration* (`#norm-probe-sensitivity` FE(2):
//! the known-bad, measured at matched area, fails the bar the live field meets).
//!
//! Components are found by union-find over cratonized samples in *direction
//! space* (angular adjacency, so a craton straddling a cube edge is ONE
//! component, never split by a chart seam — `#form-sphere-continuous-surface-fields`).
//!
//! **Boundary coherence (compactness) is also reported but does NOT separate the
//! classes** — 4-octave fBm islands are multi-cell and locally compact too
//! (both fields read ~0.8+). It is kept as an honest descriptor, not a
//! discriminator; the fragmentation-at-matched-budget count is what convicts.

use crate::sphere::{CubeCoord, Face};

/// A sub-craton neighbour arc (rad) — finer than the craton scale
/// (`lithosphere::CRATON_SITE_RADIUS_MEAN_RAD ≈ 0.235`), coarser than a
/// pour-grain cell, so the boundary-coherence statistic is scale-separated
/// (`#norm-probe-sensitivity` FE(3)).
pub const COHERENCE_ARC_RAD: f64 = 0.04;

/// Angular adjacency (rad) for the connected-component union-find — ~1.6 sample
/// cells at the probe's default level, so grid-neighbouring cratonized cells
/// connect (including across cube faces, since adjacency is in 3-space).
pub const COMPONENT_ADJ_RAD: f64 = 0.055;

/// The land threshold on the cratonization weight (the same `w > 0.5` contour
/// the calibration test and the column blend treat as "cratonized").
pub const CRATONIZED_THRESHOLD: f64 = 0.5;

/// The morphology of a cratonization field on one world.
#[derive(Clone, Copy, Debug)]
pub struct CratonMorphology {
    /// Area fraction with `w > 0.5` (the same statistic the calibration test reads).
    pub cratonized_frac: f64,
    /// Mean boundary coherence over cratonized samples — compactness ∈ [0,1].
    /// High for coherent blobs, ≈ area fraction for speckle. The discriminator.
    pub mean_coherence: f64,
    /// Number of connected cratonized components (direction-space union-find).
    pub component_count: usize,
    /// Fraction of cratonized samples in the single largest component.
    pub largest_component_frac: f64,
}

fn unit(face: Face, i: usize, j: usize, n: usize) -> [f64; 3] {
    let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
    let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
    CubeCoord { face, u, v }.to_unit()
}

fn normalize(d: [f64; 3]) -> [f64; 3] {
    let m = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    [d[0] / m, d[1] / m, d[2] / m]
}

/// Measure the morphology of any cratonization field `w(seed, dir)` at sampling
/// `level`. Generic over the field so the live nucleation-growth field and the
/// retired fBm known-bad are measured by the *same* instrument — the contrast
/// is the conviction.
pub fn measure(seed: u64, level: u8, w: impl Fn(u64, [f64; 3]) -> f64) -> CratonMorphology {
    let n = 1usize << level;
    // Collect cratonized sample directions.
    let mut crat: Vec<[f64; 3]> = Vec::new();
    let mut total = 0usize;
    for fi in 0..6u8 {
        let face = Face::from_index(fi);
        for j in 0..n {
            for i in 0..n {
                total += 1;
                let p = unit(face, i, j, n);
                if w(seed, p) > CRATONIZED_THRESHOLD {
                    crat.push(p);
                }
            }
        }
    }
    let cratonized_frac = crat.len() as f64 / total as f64;
    if crat.is_empty() {
        return CratonMorphology { cratonized_frac: 0.0, mean_coherence: 0.0, component_count: 0, largest_component_frac: 0.0 };
    }

    // (1) Boundary coherence: four tangent offsets at COHERENCE_ARC_RAD.
    let d = COHERENCE_ARC_RAD;
    let (sin_d, cos_d) = (d.sin(), d.cos());
    let mut coh_sum = 0.0f64;
    for &p in &crat {
        // A tangent basis at p.
        let a = if p[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let e1 = normalize(cross(a, p));
        let e2 = cross(p, e1);
        let mut hits = 0usize;
        for off in [e1, [-e1[0], -e1[1], -e1[2]], e2, [-e2[0], -e2[1], -e2[2]]] {
            let q = [
                p[0] * cos_d + off[0] * sin_d,
                p[1] * cos_d + off[1] * sin_d,
                p[2] * cos_d + off[2] * sin_d,
            ];
            if w(seed, normalize(q)) > CRATONIZED_THRESHOLD {
                hits += 1;
            }
        }
        coh_sum += hits as f64 / 4.0;
    }
    let mean_coherence = coh_sum / crat.len() as f64;

    // (2) Connected components: union-find with angular adjacency.
    let m = crat.len();
    let mut parent: Vec<usize> = (0..m).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        let mut r = x;
        while parent[r] != r {
            r = parent[r];
        }
        let mut c = x;
        while parent[c] != c {
            let next = parent[c];
            parent[c] = r;
            c = next;
        }
        r
    }
    let cos_adj = COMPONENT_ADJ_RAD.cos();
    for i in 0..m {
        for j in (i + 1)..m {
            let dot = crat[i][0] * crat[j][0] + crat[i][1] * crat[j][1] + crat[i][2] * crat[j][2];
            if dot > cos_adj {
                let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                if ri != rj {
                    parent[ri] = rj;
                }
            }
        }
    }
    let mut sizes: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    for i in 0..m {
        let r = find(&mut parent, i);
        *sizes.entry(r).or_insert(0) += 1;
    }
    let component_count = sizes.len();
    let largest = sizes.values().copied().max().unwrap_or(0);
    let largest_component_frac = largest as f64 / m as f64;

    CratonMorphology { cratonized_frac, mean_coherence, component_count, largest_component_frac }
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// The live nucleation-growth field, adapted to the `(seed, dir)` shape the
/// probe measures (evaluates at the direction's cell at the probe level).
pub fn nucleation_growth_field(level: u8) -> impl Fn(u64, [f64; 3]) -> f64 {
    move |seed, p| {
        let cell = CubeCoord::from_unit(p).cell(level);
        crate::lithosphere::craton_weight(seed, cell)
    }
}

/// The retired fBm-threshold field at its natural threshold — the morphology
/// known-bad, area uncalibrated (14–40 % by seed).
pub fn fbm_speckle_field(level: u8) -> impl Fn(u64, [f64; 3]) -> f64 {
    move |seed, p| {
        let cell = CubeCoord::from_unit(p).cell(level);
        crate::lithosphere::craton_weight_fbm_speckle_known_bad(seed, cell)
    }
}

/// The fBm known-bad thresholded to a **matched land budget** `target_frac` —
/// the area-controlled control. Bisects the raw-fBm threshold at `level` so the
/// cratonized fraction equals `target_frac`, then returns a hard 0/1 field at
/// that threshold. Comparing its component count to the nucleation-growth
/// field's (same area) isolates morphology from calibration.
pub fn fbm_speckle_matched_field(seed: u64, level: u8, target_frac: f64) -> impl Fn(u64, [f64; 3]) -> f64 {
    let n = 1usize << level;
    // Bisect the threshold: higher threshold ⇒ less cratonized area.
    let area_at = |t: f64| -> f64 {
        let mut c = 0usize;
        let mut tot = 0usize;
        for fi in 0..6u8 {
            let face = Face::from_index(fi);
            for j in 0..n {
                for i in 0..n {
                    tot += 1;
                    let p = unit(face, i, j, n);
                    let cell = CubeCoord::from_unit(p).cell(level);
                    if crate::lithosphere::craton_fbm_raw_known_bad(seed, cell) > t {
                        c += 1;
                    }
                }
            }
        }
        c as f64 / tot as f64
    };
    let (mut lo, mut hi) = (0.3f64, 0.95f64);
    for _ in 0..28 {
        let t = 0.5 * (lo + hi);
        if area_at(t) > target_frac {
            lo = t;
        } else {
            hi = t;
        }
    }
    let thresh = 0.5 * (lo + hi);
    move |seed, p| {
        let cell = CubeCoord::from_unit(p).cell(level);
        if crate::lithosphere::craton_fbm_raw_known_bad(seed, cell) > thresh {
            1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Coarse for test speed (union-find is O(cratonized²)); the example walks
    // level 6. At L5 the discrimination is already decisive.
    const LVL: u8 = 5;

    #[test]
    fn nucleation_growth_is_a_few_coherent_cratons() {
        // The live field carries its land budget in a FEW coherent cratons of
        // characteristic scale — the class Joseph's "made-by-something" bar
        // requires. Measured at L5 (seeds 0/1/7): 5–11 components.
        for seed in [0u64, 1, 7] {
            let m = measure(seed, LVL, nucleation_growth_field(LVL));
            assert!(m.cratonized_frac > 0.0, "seed {seed}: field must cratonize some area");
            assert!(
                m.component_count <= 20,
                "seed {seed}: expected a few coherent cratons, got {} components",
                m.component_count
            );
        }
    }

    #[test]
    fn the_fbm_threshold_known_bad_fragments_at_matched_area() {
        // `#norm-probe-sensitivity` FE(2): the probe must FAIL on the retired
        // fBm field — else it is not a probe for the speckle fault. The fair,
        // area-controlled control: give the fBm field the SAME land budget as
        // the nucleation-growth field, then count how many components that
        // identical budget breaks into. Scale-free threshold shatters where
        // nucleation-growth stays coherent (measured ~2–3×). This isolates
        // shape from the retired field's uncalibrated area.
        for seed in [0u64, 1, 7] {
            let good = measure(seed, LVL, nucleation_growth_field(LVL));
            let matched = fbm_speckle_matched_field(seed, LVL, good.cratonized_frac);
            let bad = measure(seed, LVL, &matched);
            // Same land budget (bisection lands within a sample cell or two).
            assert!(
                (bad.cratonized_frac - good.cratonized_frac).abs() < 0.01,
                "seed {seed}: matched-area control must match the budget ({:.3} vs {:.3})",
                bad.cratonized_frac, good.cratonized_frac
            );
            // …yet fragments substantially more than the nucleation-growth field.
            assert!(
                bad.component_count > good.component_count * 3 / 2,
                "seed {seed}: fBm at matched area must fragment more \
                 ({} components vs {} for nucleation-growth)",
                bad.component_count, good.component_count
            );
        }
    }

    #[test]
    fn deterministic() {
        let a = measure(0, LVL, nucleation_growth_field(LVL));
        let b = measure(0, LVL, nucleation_growth_field(LVL));
        assert_eq!(a.component_count, b.component_count);
        assert_eq!(a.mean_coherence, b.mean_coherence);
    }
}
