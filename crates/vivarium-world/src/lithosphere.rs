//! The lithosphere column and its isostasy read — emerged land as a
//! mass-conserving *reading*, not a decreed or strictly-positive field.
//!
//! Claim home: `#form-isostasy-column` (council-accepted chain:
//! mantle-thermal → lithosphere → isostasy → sea-level → erosion; elevation is
//! a derived reading of a conserved column; "uplift rate" is dⅇ/dt, a
//! diagnostic). This module is the first convicting rung of that chain:
//!
//! - **The column** ([`Column`], [`column`]): per-cell crust + depleted-keel
//!   inventory. v1 is a *declared stand-in* for the differentiation rate law
//!   (which is open modelling): a fated, sphere-continuous cratonization field
//!   thresholds into thick felsic crust + a depleted keel — **one process, two
//!   buoyant products, stacked** — over oceanic crust whose thickness and
//!   density are functions of **mantle potential temperature** (the secular-
//!   cooling driver, here a declared dialable parameter; a mantle-thermal
//!   *nomos* is still open).
//! - **The isostasy read** ([`freeboard_m`]): Airy buoyancy height of the
//!   whole column (crust *and* keel — the keel is half the freeboard, not
//!   optional) relative to the asthenosphere, minus the global isostatic
//!   reference [`reference_m`] fixed by **area mass balance** — so rise here
//!   *is* subsidence there by construction (area-mean freeboard ≡ 0), and the
//!   field can express basins ("sit low requires a negative" — the range
//!   defect that convicted the old stand-in, `DECISIONS[uplift-is-
//!   structurally-incapable-of-keeping-its-promise]`).
//!
//! **What the driver buys, convictably:** hot mantle ⇒ thick, low-density
//! oceanic crust ⇒ high oceanic stand ⇒ small craton/ocean contrast ⇒
//! flooding; as the mantle cools the seafloor thins and densifies, basins
//! deepen, and land grows — the buoyancy half of the emergence story, now a
//! monotonicity a test can fail.
//!
//! **Declared omissions (v1, honest):** no water loading in the balance
//! (ρ_sw terms of the full freeboard equation); no crustal transport yet; the
//! craton field is fated geometry, not a differentiation rate law. Erosion's
//! mass-return to the column IS built (the rock-mass ledger, `erosion_return.rs`
//! / `#form-isostasy-column` FE(9)) — a measured operator that debits crust,
//! rebounds via this Airy read, and credits sediment, but is not yet wired into
//! the default surface. Physics tier is
//! declared Low on the nomos — the *balance* is real physics; the *inventory*
//! is a stand-in.

use crate::noise;
use crate::sphere::{CellId, CubeCoord, Face};

// --- Declared constants (every magic number anchors into ASSUMPTIONS.md) ---

/// Mantle potential temperature (°C) driving oceanic-crust thickness/density —
/// the secular-cooling control parameter. Default is Archean-hot (early-
/// Abyssal); cooling it toward ~1350 (modern) deepens basins and grows land.
/// `ASSUMPTIONS.md` "mantle potential temperature".
pub const MANTLE_TP_C: f64 = 1550.0;

/// Modern-reference mantle potential temperature (°C) for the cooling ramps —
/// the asymptote the mantle-thermal cooling trajectory decays toward
/// (`crate::mantle_thermal`).
pub const TP_MODERN_C: f64 = 1350.0;

/// Asthenosphere density (kg/m³). `ASSUMPTIONS.md` "lithosphere densities".
pub const RHO_ASTHENOSPHERE: f64 = 3300.0;
/// Felsic continental crust density (kg/m³). Same anchor.
pub const RHO_CONTINENTAL: f64 = 2750.0;
/// Depleted-keel density contrast vs asthenosphere (kg/m³) — small, but it
/// acts over 150+ km of keel. Same anchor.
pub const KEEL_DENSITY_DEFICIT: f64 = 25.0;

/// Sediment density (kg/m³) — eroded rock redeposited on submarine columns by
/// the rock-mass ledger (`crate::erosion_return`). Wet clastic/sedimentary-rock
/// order; lighter than felsic crust, so a sediment blanket adds buoyant load.
/// `ASSUMPTIONS.md` "lithosphere densities".
pub const RHO_SEDIMENT: f64 = 2500.0;

/// Cratonic crust thickness (m) where cratonization saturates.
/// `ASSUMPTIONS.md` "craton geometry".
pub const CRATON_CRUST_M: f64 = 35_000.0;
/// Depleted-keel thickness (m) under a full craton. Same anchor.
pub const CRATON_KEEL_M: f64 = 180_000.0;
/// Cratonized area fraction of the fated field (0..1) at the present-Abyssal
/// anchor — the differentiation stand-in's calibration target. Same anchor.
pub const CRATON_AREA_FRAC: f64 = 0.12;

// --- Nucleation-and-growth cratonization (the fated differentiation stand-in,
//     horizontal half; `#form-isostasy-column` FE(8)) ------------------------
//
// The craton field is NOT a threshold slice through a scale-free field — that
// is percolation speckle (many small islands, the wrong morphology class,
// however honest the heights). It is a fated **nucleation-and-growth** law: a
// few blue-noise nucleation sites per world (seed-only), each accreting a
// coherent craton of characteristic scale that GROWS as the mantle cools. This
// is the cheapest law with the right morphology class, and it makes emergence
// read as continents *assembling* (`DECISIONS[craton-nucleation-and-deep-time-
// playback-ruled-in-scope]`). Physics Low-Med, declared stand-in — still not a
// differentiation *dynamics*; the sites and their growth are fated geometry.

/// Nucleation-site count is drawn fated per seed in this inclusive range — a
/// FEW cratonic nuclei, not a scale-free continuum. `ASSUMPTIONS.md`
/// "craton geometry".
pub const CRATON_SITE_COUNT: (u64, u64) = (8, 13);
/// Mean nucleation-site accretion radius (rad) at the present-Abyssal anchor
/// (`g = 1`). A characteristic craton scale (~0.2 rad ≈ ~1350 km cap radius on
/// Earth), calibrated so the present cratonized area ≈ [`CRATON_AREA_FRAC`]
/// (convicted by `craton_fraction_is_in_band`). Same anchor.
pub const CRATON_SITE_RADIUS_MEAN_RAD: f64 = 0.235;
/// Per-site radius spread (multiplicative, `[lo, hi]` × mean) — cratons vary in
/// size, so the field carries a size *distribution*, not identical discs. Same anchor.
pub const CRATON_SITE_RADIUS_LO: f64 = 0.6;
pub const CRATON_SITE_RADIUS_HI: f64 = 1.5;
/// Minimum angular separation (rad) between accepted nucleation sites
/// (blue-noise placement — `#post-determinism-as-ontology` Working Notes:
/// distribution must match phenomenon; uniform is rarely honest). Sites nearer
/// than this are rejected, so nuclei are distinct while their grown caps may
/// still merge at the margins (isolated cratons AND merged continents). Same anchor.
pub const CRATON_SITE_MIN_SEP_RAD: f64 = 0.22;
/// Continental-margin softness (rad) — the smoothstep width over which a
/// craton's weight ramps 0→1 (shelves, not cliffs). Comfortably wider than a
/// pour-grain cell arc, so the field stays sphere-continuous. Same anchor.
pub const CRATON_MARGIN_RAD: f64 = 0.06;
/// Low-frequency boundary-warp amplitude (fraction of radius) and frequency —
/// lobes the craton outline so coastlines are irregular (made-by-something),
/// not perfect circles, while staying continuous and coherent (warp wavelength
/// ≫ margin, so it warps the outline, never speckles it). Same anchor.
pub const CRATON_WARP_AMP: f64 = 0.22;
pub const CRATON_WARP_FREQ: f64 = 2.0;
/// Growth ceiling: a hard clamp on the cooling scalar `g(T_p)` (a guard — the
/// saturation below is the effective ceiling, ~1.33). Same anchor.
pub const CRATON_GROWTH_MAX: f64 = 2.5;
/// Growth-saturation rate: accretion is fast while the mantle is hot (cratons
/// ASSEMBLE early, hot→present) and SATURATES as it cools, so post-present
/// growth is gentle. This is what keeps emergence monotone: once basins are the
/// dominant moving part, growing cratons no longer raise the global isostatic
/// reference fast enough to re-submerge marginal land. Same anchor.
pub const CRATON_GROWTH_SATURATION: f64 = 4.0;

// Fated-noise domain tags for the nucleation field (keep distinct from the
// noise.rs registry: 0/1/3/7/13/17/18/19 in use). 40 site-count · 41 site
// direction · 42 site radius · 43.. per-site boundary warp.
const DOM_SITE_COUNT: u32 = 40;
const DOM_SITE_DIR: u32 = 41;
const DOM_SITE_RADIUS: u32 = 42;
const DOM_SITE_WARP_BASE: u32 = 43;

/// One fated nucleation site: a growth centre on the sphere and its base
/// accretion radius (at `g = 1`). Growth over epochs scales the radius; the
/// centre never moves — so a site that exists at epoch t exists at t+1, grown.
#[derive(Clone, Copy, Debug)]
pub struct CratonSite {
    /// Unit-vector growth centre (fated, seed-only).
    pub dir: [f64; 3],
    /// Base accretion radius (rad) at the present-Abyssal anchor.
    pub base_radius: f64,
    /// The site's fated warp domain (per-site boundary lobing).
    warp_domain: u32,
}

/// Smoothstep clamped to `[0, 1]` — the continental-margin ramp.
#[inline]
fn smoothstep01(x: f64) -> f64 {
    let t = x.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// The fated nucleation sites for a world-seed — a FEW blue-noise-separated
/// growth centres, memoized per seed (the whole set is cheap but is read at
/// every cratonized cell of every pour). Dart-throwing: draw uniform-sphere
/// candidates and accept those at least [`CRATON_SITE_MIN_SEP_RAD`] from every
/// accepted site, until the fated target count is reached or the candidate pool
/// is exhausted. Pure function of seed (`#post-determinism-as-ontology`).
pub fn craton_sites(seed: u64) -> std::sync::Arc<Vec<CratonSite>> {
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};
    static CACHE: Mutex<Option<BTreeMap<u64, Arc<Vec<CratonSite>>>>> = Mutex::new(None);
    {
        let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(sites) = guard.get_or_insert_with(BTreeMap::new).get(&seed) {
            return sites.clone();
        }
    }

    let (n_lo, n_hi) = CRATON_SITE_COUNT;
    let span = (n_hi - n_lo + 1) as f64;
    let target =
        n_lo + (noise::unit_f64(noise::hash3(seed, DOM_SITE_COUNT, 0, 0, 0)) * span) as u64;
    let target = target.clamp(n_lo, n_hi) as usize;

    const CANDIDATE_CAP: i64 = 96;
    let cos_min_sep = CRATON_SITE_MIN_SEP_RAD.cos();
    let mut sites: Vec<CratonSite> = Vec::with_capacity(target);
    for cand in 0..CANDIDATE_CAP {
        if sites.len() >= target {
            break;
        }
        // Uniform point on the sphere from two fated draws (z uniform in [-1,1],
        // azimuth uniform) — the honest spherical-uniform prior, not lattice.
        let u1 = noise::unit_f64(noise::hash3(seed, DOM_SITE_DIR, cand, 0, 0));
        let u2 = noise::unit_f64(noise::hash3(seed, DOM_SITE_DIR, cand, 1, 0));
        let z = 2.0 * u1 - 1.0;
        let phi = 2.0 * std::f64::consts::PI * u2;
        let r = (1.0 - z * z).max(0.0).sqrt();
        let dir = [r * phi.cos(), r * phi.sin(), z];
        // Blue-noise rejection: keep nuclei distinct.
        let too_close = sites.iter().any(|s| {
            let dot = s.dir[0] * dir[0] + s.dir[1] * dir[1] + s.dir[2] * dir[2];
            dot > cos_min_sep
        });
        if too_close {
            continue;
        }
        let ru = noise::unit_f64(noise::hash3(seed, DOM_SITE_RADIUS, cand, 0, 0));
        let base_radius =
            CRATON_SITE_RADIUS_MEAN_RAD * (CRATON_SITE_RADIUS_LO + ru * (CRATON_SITE_RADIUS_HI - CRATON_SITE_RADIUS_LO));
        sites.push(CratonSite {
            dir,
            base_radius,
            warp_domain: DOM_SITE_WARP_BASE.wrapping_add(sites.len() as u32),
        });
    }

    let sites = Arc::new(sites);
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(BTreeMap::new).insert(seed, sites.clone());
    sites
}

/// **Suture crustal thickening** (m) at full overlap of two grown cratons —
/// the collisional-orogeny stand-in. Where two accreting cratons have grown
/// into contact, crust shortens and thickens; the Airy read then stands the
/// suture belt above the craton plateaus with continent-wavelength flanks —
/// which is what lets drainage integrate into basins instead of fragmenting
/// into radial nets off isolated domes (the measured fault: max catchment
/// ~34 cells at L9 before this existed). Magnitude is a declared crude
/// constant (`ASSUMPTIONS.md` "suture crustal thickening"), sized for
/// Abyssal-weak-crust relief (~+1 km stand at full suture, ≈ SUTURE_CRUST_M ·
/// (ρ_a−ρ_c)/ρ_a), NOT for modern Himalayan crust — the strength limit that
/// caps early-Earth topography is open physics and this constant respects the
/// band rather than deriving it. Crust-only in v1 (no keel thickening;
/// declared omission).
pub const SUTURE_CRUST_M: f64 = 6_000.0;

/// Craton accretion-growth scalar `g(T_p)` — how far the fated cratons have
/// grown from their nuclei at mantle temperature `T_p`. Zero at the Hadean-hot
/// ceiling (`TP_HOT_MAX_C`: nuclei not yet accreted, a near-water-world),
/// exactly `1` at the present-Abyssal anchor (`MANTLE_TP_C` — the calibrated
/// area, and the coherence pin with the seed-only world), and `> 1` into the
/// cooling future (cratons keep accreting as the mantle cools, the world
/// maturing *past* the early-Earth regime), clamped at [`CRATON_GROWTH_MAX`].
/// Monotone non-increasing in `T_p`, so along the cooling chain cratons only
/// grow — emergence is monotone and the epoch caps are NESTED by construction.
pub fn craton_growth(tp_c: f64) -> f64 {
    // `TP_HOT_MAX_C` is the mantle-thermal Hadean ceiling; kept local to avoid a
    // module cycle (mantle_thermal depends on lithosphere, not vice-versa).
    const TP_HOT_MAX_C: f64 = 1650.0;
    // Cooling progress ∈ [0,1]: 0 at the Hadean-hot ceiling, 1 at the modern
    // asymptote. A saturating accretion curve `1 − e^{−a·cool}` (fast early,
    // flattening late), normalized so the present-Abyssal anchor reads exactly 1.
    let cool = |t: f64| ((TP_HOT_MAX_C - t) / (TP_HOT_MAX_C - TP_MODERN_C)).max(0.0);
    let shape = |c: f64| 1.0 - (-CRATON_GROWTH_SATURATION * c).exp();
    (shape(cool(tp_c)) / shape(cool(MANTLE_TP_C))).min(CRATON_GROWTH_MAX)
}

/// Cratonization weight ∈ [0,1] at a cell under mantle temperature `tp_c` — the
/// fated nucleation-and-growth field. Union (max) of the grown cratons: each
/// site contributes a smoothstep cap of radius `base·g(tp)`, warped by a
/// low-frequency sphere-continuous fBm so outlines are irregular. Continuous on
/// S² (all geometry from the unit vector — `#form-sphere-continuous-surface-fields`),
/// fated, and T_p-dependent through the growth scalar (cratons accrete as the
/// mantle cools).
pub fn craton_weight_at_tp(seed: u64, cell: CellId, tp_c: f64) -> f64 {
    craton_weights_top2_at_tp(seed, cell, tp_c).0
}

/// The two largest per-site cratonization weights at a cell — `(w1, w2)`,
/// `w1 ≥ w2`. `w1` is the union field [`craton_weight_at_tp`] reads; `w2 > 0`
/// only where a **second** grown craton also reaches this cell, i.e. where two
/// accreting cratons overlap. That overlap is the **suture**: the fated-geometry
/// stand-in for collisional orogeny ( `#form-isostasy-column` — the horizontal
/// differentiation half stays fated geometry, not plate dynamics). Because each
/// cap grows with cooling, `w2`'s support appears only once neighbouring
/// cratons have grown into contact and then widens — orogeny arrives *in time*,
/// after first emergence, matching the ladder's "compressional tectonics and
/// orogeny add later relief" (FE(5) there).
pub fn craton_weights_top2_at_tp(seed: u64, cell: CellId, tp_c: f64) -> (f64, f64) {
    let g = craton_growth(tp_c);
    if g <= 0.0 {
        return (0.0, 0.0);
    }
    let p = cell.to_cube().to_unit();
    let sites = craton_sites(seed);
    let (mut w1, mut w2) = (0.0f64, 0.0f64);
    for s in sites.iter() {
        let cosang = (p[0] * s.dir[0] + p[1] * s.dir[1] + p[2] * s.dir[2]).clamp(-1.0, 1.0);
        // Skip-far: the largest this cap can reach is base·g·(1+warp_amp)+margin.
        // Cells beyond it get 0 from this site — and, crucially, we never pay the
        // fBm warp for the ~88 % oceanic sphere (cost tracks cratonized area).
        let r_max = s.base_radius * g * (1.0 + CRATON_WARP_AMP);
        if cosang < (r_max + CRATON_MARGIN_RAD).cos() {
            continue;
        }
        let theta = cosang.acos();
        let warp = (noise::fbm3(
            seed,
            s.warp_domain,
            p[0] * CRATON_WARP_FREQ,
            p[1] * CRATON_WARP_FREQ,
            p[2] * CRATON_WARP_FREQ,
            2,
            2.0,
            0.5,
        ) - 0.5)
            * 2.0
            * CRATON_WARP_AMP;
        let r_eff = s.base_radius * g * (1.0 + warp);
        let wi = smoothstep01((r_eff - theta) / CRATON_MARGIN_RAD);
        if wi > w1 {
            w2 = w1;
            w1 = wi;
        } else if wi > w2 {
            w2 = wi;
        }
    }
    (w1, w2)
}

/// Cratonization weight at the present-Abyssal anchor (`MANTLE_TP_C`) — the
/// seed-only default the calibration test and probes read.
pub fn craton_weight(seed: u64, cell: CellId) -> f64 {
    craton_weight_at_tp(seed, cell, MANTLE_TP_C)
}

/// **RETIRED as world law; kept ONLY as the morphology probe's known-bad**
/// (`#norm-probe-sensitivity` FE(2): a probe that cannot fail on a historical
/// broken configuration is not yet a probe for that fault class). This is the
/// pre-2026-07-24 craton field: a threshold slice through a 4-octave fBm, whose
/// land-threshold morphology is *percolation speckle* — many tiny components,
/// low compactness — the wrong class Joseph identified on the live globe. The
/// `craton_morphology` probe FAILS on this and PASSES on the nucleation-growth
/// field; that contrast is the probe's conviction. Not called by any world path.
pub fn craton_weight_fbm_speckle_known_bad(seed: u64, cell: CellId) -> f64 {
    let threshold = 0.585;
    let margin = 0.03;
    ((craton_fbm_raw_known_bad(seed, cell) - (threshold - margin)) / margin).clamp(0.0, 1.0)
}

/// The raw 4-octave fBm value the retired field thresholded — exposed so the
/// morphology probe can threshold it to a *matched land budget* and compare
/// fragmentation apples-to-apples (the retired field's own area was
/// uncalibrated: 14–40 % by seed, itself a symptom). Not world law.
pub fn craton_fbm_raw_known_bad(seed: u64, cell: CellId) -> f64 {
    let p = cell.to_cube().to_unit();
    let f = 2.2;
    noise::fbm3(seed, 17, p[0] * f, p[1] * f, p[2] * f, 4, 2.0, 0.5)
}

/// One lithospheric column: the conserved inventory isostasy reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Column {
    /// Crust thickness (m) and density (kg/m³) — felsic under cratons,
    /// oceanic elsewhere (thickness/density set by the thermal driver).
    pub crust_m: f64,
    pub crust_rho: f64,
    /// Depleted continental-lithospheric-mantle keel (m); density is
    /// `RHO_ASTHENOSPHERE − KEEL_DENSITY_DEFICIT`.
    pub keel_m: f64,
    /// Redeposited sediment thickness (m), density [`RHO_SEDIMENT`) — the credit
    /// half of the rock-mass ledger (`crate::erosion_return`). Zero on the fated
    /// pre-erosion column; positive on submarine columns after an erosion pass
    /// (eroded craton crust returned as a submarine blanket). A buoyant load, so
    /// depositing here *raises* the read (basins fill) while the eroded cratons
    /// rebound down — relief compresses from both ends.
    pub sediment_m: f64,
}

/// Oceanic crust thickness (m) at mantle potential temperature `tp_c` —
/// thick when hot (high melt fraction), thinning toward ~7 km modern.
/// Linear ramp between the anchored endpoints (Archean ~30 km @1550 °C).
pub fn oceanic_crust_m(tp_c: f64) -> f64 {
    let t = ((tp_c - TP_MODERN_C) / (MANTLE_TP_C - TP_MODERN_C)).clamp(0.0, 1.5);
    7_000.0 + t * (30_000.0 - 7_000.0)
}

/// Oceanic crust density (kg/m³) at `tp_c` — MgO-rich and light when hot
/// (will not sink), densifying toward ~2950 modern as melt fractions drop.
pub fn oceanic_crust_rho(tp_c: f64) -> f64 {
    let t = ((tp_c - TP_MODERN_C) / (MANTLE_TP_C - TP_MODERN_C)).clamp(0.0, 1.5);
    2950.0 - t * 100.0
}

/// The lithospheric column at a cell under mantle temperature `tp_c`.
/// Craton and oceanic end-members blend over the margin weight. The craton
/// weight is itself `tp_c`-dependent (nucleation-growth: cratons accrete as the
/// mantle cools), so a cooling epoch both deepens basins AND grows the cratons.
pub fn column_at_tp(seed: u64, cell: CellId, tp_c: f64) -> Column {
    let (w, suture) = craton_weights_top2_at_tp(seed, cell, tp_c);
    let (h_oc, rho_oc) = (oceanic_crust_m(tp_c), oceanic_crust_rho(tp_c));
    Column {
        // Craton blend, plus collisional thickening where a SECOND grown craton
        // overlaps (suture = w2 ≤ w1, so belts sit inside fully-cratonized
        // ground and taper with the overlap; see [`SUTURE_CRUST_M`]).
        crust_m: h_oc + w * (CRATON_CRUST_M - h_oc) + suture * SUTURE_CRUST_M,
        // density blends toward felsic as cratonization completes
        crust_rho: rho_oc + w * (RHO_CONTINENTAL - rho_oc),
        keel_m: w * CRATON_KEEL_M,
        sediment_m: 0.0, // fated pre-erosion column carries no redeposited sediment
    }
}

/// The column at the declared default mantle temperature.
pub fn column(seed: u64, cell: CellId) -> Column {
    column_at_tp(seed, cell, MANTLE_TP_C)
}

/// Airy buoyancy height (m) of a column relative to a pure-asthenosphere
/// reference: Σ tᵢ·(ρ_a − ρᵢ)/ρ_a over crust and keel. The raw isostatic
/// stand, before the global reference is subtracted.
pub fn buoyancy_height_m(c: &Column) -> f64 {
    let rho_a = RHO_ASTHENOSPHERE;
    c.crust_m * (rho_a - c.crust_rho) / rho_a
        + c.keel_m * KEEL_DENSITY_DEFICIT / rho_a
        + c.sediment_m * (rho_a - RHO_SEDIMENT) / rho_a
}

/// **Isostatic rebound of erosional unloading, as a pure algebraic identity.**
/// Removing crust thickness `δ` (density `crust_rho`) from a column lowers its
/// isostatic stand by only `δ·(ρ_a − crust_rho)/ρ_a` — *not* by `δ`. The
/// difference `δ·crust_rho/ρ_a` is the rebound: the lightened column floats
/// back up, so most of the rock removed is given back as uplift. This is why
/// erosion *bounds* relief rather than planing it flat, and the reason a
/// rock-mass ledger (`crate::erosion_return`) trims the amber over-stand toward
/// the band instead of collapsing it (`#form-isostasy-column` FE(8)).
/// For felsic crust: `(3300 − 2750)/3300 ≈ 0.167`, so ~83 % rebounds.
pub fn surface_drop_per_crust_removed(crust_rho: f64) -> f64 {
    (RHO_ASTHENOSPHERE - crust_rho) / RHO_ASTHENOSPHERE
}

/// Sampling level for the global reference (matches the pour's diagnostic
/// grain in `sea_level.rs`).
const SAMPLE_LEVEL: u8 = 7;

/// The global isostatic reference (m): area-mean buoyancy height at `tp_c`,
/// fixed by mass balance over the sampled sphere. Subtracting it makes the
/// freeboard field exactly zero-mean on the sample — rise here IS subsidence
/// there; changing the columns redistributes land, never mints it.
///
/// **Memoized per `(seed, tp_c)`** — this is the whole-sphere integral, so it
/// must not be recomputed per freeboard read. Without the cache the cooling-
/// chain pour (which reads freeboard at ~10⁵–10⁶ cells) would recompute this
/// 10⁵-cell integral at every one of them — an O(N²) blow-up. The default path
/// (`reference_m`) hits the same cache at `MANTLE_TP_C`.
pub fn reference_m_at_tp(seed: u64, tp_c: f64) -> f64 {
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    static CACHE: Mutex<Option<BTreeMap<(u64, u64), f64>>> = Mutex::new(None);
    let key = (seed, tp_c.to_bits());
    {
        let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
        let map = guard.get_or_insert_with(BTreeMap::new);
        if let Some(&r) = map.get(&key) {
            return r;
        }
    }
    let n = 1usize << SAMPLE_LEVEL;
    let mut sum = 0.0;
    let mut count = 0usize;
    for fi in 0..6u8 {
        let face = Face::from_index(fi);
        for j in 0..n {
            for i in 0..n {
                let u = ((i as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let v = ((j as f64 + 0.5) / n as f64) * 2.0 - 1.0;
                let cell = CubeCoord { face, u, v }.cell(SAMPLE_LEVEL);
                sum += buoyancy_height_m(&column_at_tp(seed, cell, tp_c));
                count += 1;
            }
        }
    }
    let r = sum / count as f64;
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(BTreeMap::new).insert(key, r);
    r
}

/// Memoized reference at the default (present-Abyssal) temperature.
pub fn reference_m(seed: u64) -> f64 {
    reference_m_at_tp(seed, MANTLE_TP_C)
}

/// **The isostasy read** — freeboard (m) at a cell: buoyancy height minus the
/// global reference. Zero-mean by construction; negative over oceanic columns
/// (basins), positive over cratons (emerged land, once the pour agrees).
/// This replaces the retired fBm freeboard stand-in on the tectonic surface.
pub fn freeboard_m(seed: u64, cell: CellId) -> f64 {
    buoyancy_height_m(&column(seed, cell)) - reference_m(seed)
}

/// Freeboard at an explicit mantle temperature (for probes: the secular-
/// cooling monotonicity and the flooded-Archean control).
pub fn freeboard_m_at_tp(seed: u64, cell: CellId, tp_c: f64) -> f64 {
    buoyancy_height_m(&column_at_tp(seed, cell, tp_c)) - reference_m_at_tp(seed, tp_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cells(n_side: usize) -> Vec<CellId> {
        let mut cells = Vec::new();
        for fi in 0..6u8 {
            let face = Face::from_index(fi);
            for j in 0..n_side {
                for i in 0..n_side {
                    let u = ((i as f64 + 0.5) / n_side as f64) * 2.0 - 1.0;
                    let v = ((j as f64 + 0.5) / n_side as f64) * 2.0 - 1.0;
                    cells.push(CubeCoord { face, u, v }.cell(SAMPLE_LEVEL));
                }
            }
        }
        cells
    }

    #[test]
    fn craton_fraction_is_in_band() {
        // Convicts the nucleation-growth calibration: cratonized area (w > 0.5)
        // at the present-Abyssal anchor must sit near CRATON_AREA_FRAC (site
        // count × mean radius calibrated to it). Measured seed 0 ~0.093 —
        // in-band; the guard against silent drift as the field's shape knobs move.
        let cells = sample_cells(64);
        let frac = cells.iter().filter(|&&c| craton_weight(0, c) > 0.5).count() as f64 / cells.len() as f64;
        assert!(
            (CRATON_AREA_FRAC * 0.6..CRATON_AREA_FRAC * 1.7).contains(&frac),
            "cratonized fraction {frac:.4} strayed from declared ~{CRATON_AREA_FRAC}"
        );
    }

    #[test]
    fn freeboard_range_spans_zero_the_reachability_check() {
        // The range/reachability conviction from `DECISIONS[uplift-is-
        // structurally-incapable…]`: the emerged-land keeper's output must be
        // able to express BOTH basins (negative) and land (positive). The old
        // rate field was strictly positive and mechanically could not.
        let cells = sample_cells(48);
        let (mut neg, mut pos) = (0usize, 0usize);
        for &c in &cells {
            let f = freeboard_m(0, c);
            if f < -100.0 {
                neg += 1;
            }
            if f > 100.0 {
                pos += 1;
            }
        }
        assert!(neg > 0 && pos > 0, "isostasy read must span basins and land (neg {neg}, pos {pos})");
        // And the sign structure is the column story: most of the sphere sits
        // low (oceanic), a minority stands high (cratons).
        assert!(neg > pos, "oceanic columns dominate by area");
    }

    #[test]
    fn mass_balance_mean_freeboard_is_zero_at_any_driver_setting() {
        // Rise here IS subsidence there: the reference is fixed by area mass
        // balance, so area-mean freeboard ≡ 0 at every mantle temperature —
        // cooling redistributes stand, it cannot mint elevation globally.
        // Sampled on the SAME grid the reference integrates over, the mean is
        // zero to float precision — the balance is by construction, not fit.
        for tp in [1550.0, 1450.0, 1350.0] {
            let cells = sample_cells(1usize << SAMPLE_LEVEL);
            let reference = reference_m_at_tp(0, tp); // hoisted: one global constant per driver setting
            let mean: f64 = cells
                .iter()
                .map(|&c| buoyancy_height_m(&column_at_tp(0, c, tp)) - reference)
                .sum::<f64>()
                / cells.len() as f64;
            assert!(mean.abs() < 1e-6, "mean freeboard {mean:.3e} m at Tp={tp} must vanish on the reference grid");
        }
    }

    #[test]
    fn cooling_grows_the_craton_ocean_contrast_the_buoyancy_half() {
        // The buoyancy half of the emergence story, convictable: as the mantle
        // cools the seafloor thins and densifies, so the craton−ocean stand
        // CONTRAST grows. (Land fraction after the pour is the sea-level
        // module's test; this pins the column-side monotonicity driving it.)
        let hot = oceanic_crust_m(1550.0) * (RHO_ASTHENOSPHERE - oceanic_crust_rho(1550.0)) / RHO_ASTHENOSPHERE;
        let cool = oceanic_crust_m(1350.0) * (RHO_ASTHENOSPHERE - oceanic_crust_rho(1350.0)) / RHO_ASTHENOSPHERE;
        assert!(cool < hot, "oceanic stand falls as the mantle cools");
        let craton = buoyancy_height_m(&Column {
            crust_m: CRATON_CRUST_M,
            crust_rho: RHO_CONTINENTAL,
            keel_m: CRATON_KEEL_M,
            sediment_m: 0.0,
        });
        assert!(craton - cool > craton - hot, "craton/ocean contrast grows under cooling");
        assert!(craton > hot, "cratons stand above even the hot Archean seafloor");
    }

    #[test]
    fn keel_is_half_the_freeboard_not_optional() {
        // Removing the keel from a craton column must cost a substantial part
        // of its stand — the decision's "the keel is half of it" clause, order
        // of magnitude: 180 km × 25 kg/m³ / 3300 ≈ 1.4 km of the ~7 km stand.
        let with = buoyancy_height_m(&Column { crust_m: CRATON_CRUST_M, crust_rho: RHO_CONTINENTAL, keel_m: CRATON_KEEL_M, sediment_m: 0.0 });
        let without = buoyancy_height_m(&Column { crust_m: CRATON_CRUST_M, crust_rho: RHO_CONTINENTAL, keel_m: 0.0, sediment_m: 0.0 });
        let keel_share = (with - without) / with;
        assert!(keel_share > 0.15, "keel contributes materially ({:.0}%)", keel_share * 100.0);
    }

    #[test]
    fn erosional_unloading_rebounds_surface_falls_by_less_than_the_rock_removed() {
        // The rock-mass ledger's load-bearing physics (`#form-isostasy-column`
        // FE(8)): strip crust δ from a craton and the isostatic stand falls by
        // only δ·(ρ_a−ρ_crust)/ρ_a — the lightened column rebounds. A failable
        // statement that erosion BOUNDS relief (rebound-aware) rather than
        // planing it 1:1. If someone deleted the rebound (subtracted δ from the
        // stand directly), this fails.
        let full = Column { crust_m: CRATON_CRUST_M, crust_rho: RHO_CONTINENTAL, keel_m: CRATON_KEEL_M, sediment_m: 0.0 };
        let delta = 3_000.0; // remove 3 km of crust
        let eroded = Column { crust_m: CRATON_CRUST_M - delta, ..full };
        let surface_drop = buoyancy_height_m(&full) - buoyancy_height_m(&eroded);
        let expected = delta * surface_drop_per_crust_removed(RHO_CONTINENTAL);
        assert!((surface_drop - expected).abs() < 1e-6, "rebound identity broken: {surface_drop} vs {expected}");
        // The surface falls by far LESS than the rock removed (most rebounds).
        assert!(surface_drop < 0.25 * delta, "felsic rebound gives back ~83%: drop {surface_drop} of {delta} removed");
        assert!(surface_drop > 0.0, "removing crust still lowers the stand");
    }

    #[test]
    fn deterministic_and_sphere_continuous_margins() {
        // Fated: same (seed, cell) → same column. Sphere-continuous: the craton
        // field is unit-vector fBm, so adjacent cells across any arc differ
        // smoothly (margin, not cliff) — checked as bounded one-cell deltas.
        let cell = CubeCoord { face: Face::from_index(2), u: 0.3, v: -0.4 }.cell(SAMPLE_LEVEL);
        assert_eq!(column(7, cell), column(7, cell));
        let cells = sample_cells(64);
        let mut max_delta = 0.0f64;
        for pair in cells.windows(2) {
            let d = (freeboard_m(0, pair[0]) - freeboard_m(0, pair[1])).abs();
            max_delta = max_delta.max(d);
        }
        // At this coarse sampling adjacent samples are far apart, so this is a
        // sanity bound (no multi-km cliffs between neighbouring samples of the
        // same face row), not the generator continuity probe (gen.rs owns that).
        assert!(max_delta < CRATON_CRUST_M, "no cliff larger than the full craton stand between samples");
    }
}
