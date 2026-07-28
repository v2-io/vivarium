//! Does erosion have a convergence criterion that can *fail*? The candidate is a
//! **shape** test, measured stage by stage against real settle histories.
//!
//! Why a shape test. `#obs-erosion-residual-is-driver-bound` measured the
//! obvious criterion dead: sustained uplift leaves no zero-residual equilibrium,
//! so a tolerance on mean $|\Delta h|$ either never fires or fires immediately on
//! a tile that is merely inert — and the second failure is the dangerous one.
//! Perron & Royden (2013, *ESPL* 38:570–576) supply a criterion of a different
//! kind: at steady state, channel elevation is **linear in $\chi$**, the upstream
//! integral $\int (A_0/A)^{m/n}\,dx$. That is a statement about the surface at
//! one instant, so a pinned per-epoch residual is not an obstacle to it, and a
//! tile with no channel produces *no test* rather than a spurious pass.
//!
//! Two forms are reported side by side, and the difference between them is the
//! point ( `#obs-chi-shape-is-erosions-criterion` FE(1)–(2)): the **literature
//! form**, which assumes a spatially invariant $U$, and the **zero-parameter
//! form**, which is exact algebra on this kernel's own incision update and holds
//! under a varying $U$. A driver coherent at continent wavelength is precisely
//! where the two should part company.
//!
//! **Every longest chain is measured, and each is labelled by the uplift nomos
//! that drove it.** Picking one cohort silently is how a comparison becomes an
//! accident, and the store now holds chains under two different drivers; two
//! stages carved under different source digests are stages of two different
//! worlds ( `#form-time-indexed-stage-chains` FE(11)).
//!
//! **Read-only, and never writes.** Heights, uplift, climate and the recorded
//! residual all come from store roots carrying the *same* `src=` as the stage
//! being examined, so the uplift field is the one that actually drove that stage
//! rather than whatever the running binary would produce now.
//!
//! Run: `cargo run --release --example chi_convergence_probe`
//! Env: `VIVARIUM_WORLD` · `CHI_SRC` (pin one cohort) · `CHI_CHANNEL_CELLS`
//! (channelization threshold in median cell areas, default 10) · `CHI_EXTEND`
//! (carry one tile past the ladder, default 3000) · `CHI_G_SWEEP` (deposition
//! sweep, default on).

use std::collections::BTreeMap;

use vivarium_world::erosion::{ErodedRegion, Fluvial, FluvialParams};
use vivarium_world::planet::Planet;
use vivarium_world::sample::cell_size_m;
use vivarium_world::sphere::Face;
use vivarium_world::store::Store;
use vivarium_world::watch::key_field;

/// Minimum channel cells in a basin before its profile is fitted at all. Below
/// this a slope is noise, and reporting it would be the "insensitive green"
/// #norm-probe-sensitivity warns about.
const MIN_BASIN_CELLS: usize = 12;

/// A tile's identity within one cohort.
type TileAt = (u8, u32, u32, usize); // face, oi, oj, nx
/// tile → stage epochs → object hash.
type Cohort = BTreeMap<TileAt, BTreeMap<u32, String>>;

fn decode_f32(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// Ordinary least squares of `y` on `x` → `(slope, intercept)`.
fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let s = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    (s, my - s * mx)
}

fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// $\sqrt{1-R^2}$ of `y` on `x` — the fraction of amplitude the line leaves
/// unexplained. Dimensionless, so it is comparable across stages whose relief is
/// growing and across cohorts whose drivers differ in magnitude.
fn unexplained(x: &[f64], y: &[f64]) -> f64 {
    let (s, c) = ols(x, y);
    let my = y.iter().sum::<f64>() / y.len() as f64;
    let res: f64 = x.iter().zip(y).map(|(a, b)| (b - (c + s * a)).powi(2)).sum();
    let tot: f64 = y.iter().map(|b| (b - my).powi(2)).sum();
    (res / tot.max(f64::MIN_POSITIVE)).sqrt()
}

/// What one tile at one stage says about the criterion.
#[derive(Default, Clone, Copy)]
struct TileVerdict {
    channel_cells: usize,
    fitted_basins: usize,
    /// Literature form: how much channel-elevation amplitude $\chi$ leaves
    /// unexplained, pooled over the tile's fitted basins.
    unexplained: f64,
    /// RMS fit residual in metres, for the f32-noise-floor check.
    rms_m: f64,
    /// Median fitted $\mathrm{d}z/\mathrm{d}\chi$ over fitted basins.
    slope: f64,
    /// $U/(k_{dt}A_0^{m})$ from this tile's own uplift field.
    slope_pred: f64,
    /// **Zero-parameter form**: departure from the steady profile the kernel's
    /// own update implies, normalized against the same total as the literature
    /// form. It consumes the per-cell $U$, so it assumes no spatially invariant
    /// driver — but it also carries the composition's steepening offset (FE(5)),
    /// which the free-slope literature form absorbs. It is therefore *not* a
    /// fair head-to-head against `unexplained`.
    zero_param: f64,
    /// **Matched-parameter form** — the fair comparison, and the one that decides
    /// whether FE(2) buys anything. Regress $z$ on `z_steady` with a free
    /// intercept and scale instead of on $\chi$: two fitted parameters, exactly
    /// as many as the literature form, with the scale absorbing the deposition
    /// offset. Under a spatially invariant $U$ the predicted profile is an affine
    /// function of $\chi$, so the two columns must come out **identical** — which
    /// is also a check on the implementation. They can only diverge where $U$
    /// varies along a channel, and there the per-cell form should win.
    u_shape: f64,
    /// Fraction of fitted channel cells on non-positive uplift — where no driven
    /// steady state exists for incision to balance.
    subsiding: f64,
    /// Median over fitted basins of $\max U / \min U$ **along that basin's own
    /// channel cells**. This, and not the driver's global variability, is the
    /// quantity that decides whether the literature and matched-parameter forms
    /// can differ at all: a basin whose channel sees a constant $U$ makes the
    /// predicted profile an affine function of $\chi$, and the two fits become
    /// the same fit. At 1.00 the discrimination is unreachable, not merely
    /// unobserved.
    u_span: f64,
}

/// Everything the probe needs to reach the store, shared across cohorts.
struct Probe<'a> {
    store: &'a Store,
    seed: u64,
    /// `(kind, src, level, tile)` → object hash, for the non-erosion inputs.
    inputs: &'a BTreeMap<(&'static str, String, u8, TileAt), String>,
    /// `(src, level, tile, epochs)` → object hash of the recorded residual.
    residuals: &'a BTreeMap<(String, u8, TileAt, u32), String>,
    p: FluvialParams,
    channel_min_cells: f32,
    trunk_min_cells: f32,
}

impl Probe<'_> {
    /// A `Fluvial` seeded exactly as the build had it: stored heights, plus the
    /// uplift and climate fields from the **same source tree** that carved this
    /// stage — not what the running binary would compute now.
    fn seeded(&self, src: &str, level: u8, at: TileAt, object: &str) -> Option<Fluvial> {
        let (face, oi, oj, nx) = at;
        let h = decode_f32(&self.store.object_bytes(object)?);
        if h.len() != nx * nx {
            return None;
        }
        let mut f = Fluvial::from_region(&ErodedRegion {
            face: Face::from_index(face),
            level,
            oi,
            oj,
            nx,
            h,
            seed: self.seed,
        });
        let key = |kind| (kind, src.to_string(), level, at);
        let uplift = decode_f32(&self.store.object_bytes(self.inputs.get(&key("uplift-tile"))?)?);
        let precip = decode_f32(&self.store.object_bytes(self.inputs.get(&key("climate"))?)?);
        if uplift.len() != nx * nx || precip.len() != nx * nx {
            return None;
        }
        let mean = precip.iter().sum::<f32>() / precip.len() as f32;
        let pw: Vec<f32> = if mean > 0.0 {
            precip.iter().map(|v| v / mean).collect()
        } else {
            vec![1.0; precip.len()]
        };
        f.set_uplift_rate(uplift);
        f.set_precip_weight(pw);
        Some(f)
    }

    /// Fit every basin of one tile and pool the result. `None` when no basin is
    /// channelized enough to fit — a tile that produces **no test**, which is the
    /// fail-safe and never a pass.
    fn judge(&self, f: &mut Fluvial, channel_min_cells: f32) -> Option<TileVerdict> {
        let mut med = f.cell_area.clone();
        med.sort_by(f32::total_cmp);
        let a0 = med[med.len() / 2];
        let thresh = channel_min_cells * a0;
        let uplift: Vec<f32> = f.uplift_rate().to_vec();
        let prof = f.chi_profile(&self.p, a0);

        let mut by_basin: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for i in 0..prof.chi.len() {
            if prof.basin[i] != u32::MAX
                && prof.chi[i] > 0.0
                && prof.drainage[i] >= thresh
                && prof.z_steady[i].is_finite()
            {
                by_basin.entry(prof.basin[i]).or_default().push(i);
            }
        }
        let channel_cells: usize = by_basin.values().map(Vec::len).sum();
        let (mut ss_res, mut ss_tot, mut ss_zero, mut n_fit) = (0.0f64, 0.0f64, 0.0f64, 0usize);
        let mut ss_ushape = 0.0f64;
        let (mut slopes, mut preds) = (Vec::new(), Vec::new());
        let mut spans: Vec<f64> = Vec::new();
        let mut subsiding = 0usize;
        for cells in by_basin.values().filter(|v| v.len() >= MIN_BASIN_CELLS) {
            let x: Vec<f64> = cells.iter().map(|&i| prof.chi[i] as f64).collect();
            let y: Vec<f64> = cells.iter().map(|&i| prof.h[i] as f64).collect();
            let (s, c) = ols(&x, &y);
            let my = y.iter().sum::<f64>() / y.len() as f64;
            ss_res += x.iter().zip(&y).map(|(a, b)| (b - (c + s * a)).powi(2)).sum::<f64>();
            ss_tot += y.iter().map(|b| (b - my).powi(2)).sum::<f64>();
            ss_zero += cells
                .iter()
                .map(|&i| ((prof.h[i] - prof.z_steady[i]) as f64).powi(2))
                .sum::<f64>();
            // The matched-parameter form: the same two fitted parameters, against
            // the per-cell predicted profile instead of against χ.
            let w: Vec<f64> = cells.iter().map(|&i| prof.z_steady[i] as f64).collect();
            let (sw, cw) = ols(&w, &y);
            ss_ushape += w.iter().zip(&y).map(|(a, b)| (b - (cw + sw * a)).powi(2)).sum::<f64>();
            n_fit += cells.len();
            subsiding += cells.iter().filter(|&&i| uplift[i] <= 0.0).count();
            let (ulo, uhi) = cells.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &i| {
                (lo.min(uplift[i] as f64), hi.max(uplift[i] as f64))
            });
            spans.push(if ulo > 0.0 { uhi / ulo } else { f64::INFINITY });
            slopes.push(s);
            preds.push(
                cells.iter().map(|&i| uplift[i] as f64).sum::<f64>() / cells.len() as f64
                    / (self.p.k_dt as f64 * (a0 as f64).powf(self.p.m as f64)),
            );
        }
        if n_fit == 0 {
            return None;
        }
        Some(TileVerdict {
            channel_cells,
            fitted_basins: slopes.len(),
            unexplained: (ss_res / ss_tot.max(f64::MIN_POSITIVE)).sqrt(),
            rms_m: (ss_res / n_fit as f64).sqrt(),
            slope: median(&mut slopes),
            slope_pred: median(&mut preds),
            zero_param: (ss_zero / ss_tot.max(f64::MIN_POSITIVE)).sqrt(),
            u_shape: (ss_ushape / ss_tot.max(f64::MIN_POSITIVE)).sqrt(),
            subsiding: subsiding as f64 / n_fit as f64,
            u_span: median(&mut spans),
        })
    }

    /// The known-bad battery, run on a **real** network and on the **measured**
    /// profile rather than an idealization: the question is not "can a fit see a
    /// step" but "can this criterion see a step over the residual the landscape
    /// already carries" ( #norm-probe-sensitivity FE(1)).
    fn known_bads(&self, label: &str, f: &mut Fluvial) {
        let mut med = f.cell_area.clone();
        med.sort_by(f32::total_cmp);
        let a0 = med[med.len() / 2];
        let prof = f.chi_profile(&self.p, a0);
        let thresh = self.channel_min_cells * a0;
        let mut by_basin: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for i in 0..prof.chi.len() {
            if prof.basin[i] != u32::MAX && prof.chi[i] > 0.0 && prof.drainage[i] >= thresh {
                by_basin.entry(prof.basin[i]).or_default().push(i);
            }
        }
        let Some(cells) =
            by_basin.values().filter(|v| v.len() >= MIN_BASIN_CELLS).max_by_key(|v| v.len())
        else {
            println!("  {label}: no basin large enough — no test, which is the fail-safe, not a pass");
            return;
        };
        let chi: Vec<f64> = cells.iter().map(|&i| prof.chi[i] as f64).collect();
        let real: Vec<f64> = cells.iter().map(|&i| prof.h[i] as f64).collect();
        let (s_real, c_real) = ols(&chi, &real);
        let my = real.iter().sum::<f64>() / real.len() as f64;
        let amp = (real.iter().map(|b| (b - my).powi(2)).sum::<f64>() / real.len() as f64).sqrt();
        let mid = median(&mut chi.clone());
        let base = unexplained(&chi, &real);
        let linear: Vec<f64> = chi.iter().map(|c| c_real + s_real * c).collect();
        let doubled: Vec<f64> = chi.iter().map(|c| c_real + 2.0 * s_real * c).collect();
        println!("\n  {label}: largest basin {} channel cells, elevation sd {amp:.1} m", cells.len());
        println!("    exactly linear in χ (positive control)  1-R²(√) = {:.5}", unexplained(&chi, &linear));
        println!("    the measured surface                    1-R²(√) = {base:.5}");
        println!(
            "    linear at TWICE the slope               1-R²(√) = {:.5}  <- shape passes; only dz/dχ ({:.4} vs {:.4}) catches it",
            unexplained(&chi, &doubled),
            2.0 * s_real,
            s_real
        );
        println!("    knickpoint of amplitude a·sd added at median χ, on top of the measured profile:");
        for a in [0.25f64, 0.5, 1.0, 2.0, 4.0] {
            let k: Vec<f64> =
                real.iter().zip(&chi).map(|(v, c)| if *c > mid { v + a * amp } else { *v }).collect();
            let u = unexplained(&chi, &k);
            println!(
                "        a = {a:>4.2} ({:>7.1} m)   1-R²(√) = {u:.5}   ×{:.2} the measured residual{}",
                a * amp,
                u / base,
                if u > 1.2 * base { "   <- convicted" } else { "" }
            );
        }
    }

    /// Measure one cohort end to end.
    fn measure_cohort(&self, src: &str, level: u8, cohort: &Cohort, driver: &str, extend: u32) {
        let ladder: Vec<u32> = cohort
            .values()
            .flat_map(|s| s.keys().copied())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let cell_km = cell_size_m(level, Planet::EARTH.radius_m) / 1000.0;
        println!(
            "\n\n=== cohort src={src} L{level} ({cell_km:.2} km/cell) — driver {driver} — {} tiles × {} stages ===",
            cohort.len(),
            ladder.len()
        );
        if src != vivarium_world::nomotheke::SRC_HASH {
            println!("    (not the running binary's source tree — a faithful history of the world that carved it, read under its own name)");
        }

        // Stage 0 is not a synthetic: the actual initial topography these stages
        // were carved from, and the known-bad the criterion has to reject.
        let stage0: Vec<(TileAt, String)> = cohort
            .keys()
            .filter_map(|at| {
                self.inputs
                    .get(&("initial-topography", src.to_string(), level, *at))
                    .map(|o| (*at, o.clone()))
            })
            .collect();

        println!(
            "\n  {:>7} {:>8} {:>7} {:>9} {:>9} {:>9} {:>8} {:>9} {:>9} {:>8} {:>9}",
            "stage", "channel", "basins", "1-R²(√)", "U-shape", "trunk", "0-param", "dz/dχ", "U/kA^m", "ratio", "mean|Δh|"
        );
        let mut rows: Vec<(i64, TileVerdict, f64, f64)> = Vec::new();
        for (label, tiles) in std::iter::once((-1i64, stage0)).chain(ladder.iter().map(|&k| {
            (
                k as i64,
                cohort
                    .iter()
                    .filter_map(|(at, s)| s.get(&k).map(|o| (*at, o.clone())))
                    .collect::<Vec<_>>(),
            )
        })) {
            let (mut unexp, mut rms, mut slope, mut pred) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            let (mut zero, mut trunk, mut subs) = (Vec::new(), Vec::new(), Vec::new());
            let (mut ush, mut spans): (Vec<f64>, Vec<f64>) = (Vec::new(), Vec::new());
            let (mut chan, mut basins, mut no_test) = (0usize, 0usize, 0usize);
            let mut deltas: Vec<f64> = Vec::new();
            for (at, object) in &tiles {
                let Some(mut f) = self.seeded(src, level, *at, object) else { continue };
                if let Some(t) = self.judge(&mut f, self.trunk_min_cells) {
                    trunk.push(t.unexplained);
                }
                match self.judge(&mut f, self.channel_min_cells) {
                    Some(v) => {
                        chan += v.channel_cells;
                        basins += v.fitted_basins;
                        unexp.push(v.unexplained);
                        rms.push(v.rms_m);
                        slope.push(v.slope);
                        pred.push(v.slope_pred);
                        zero.push(v.zero_param);
                        ush.push(v.u_shape);
                        spans.push(v.u_span);
                        subs.push(v.subsiding);
                    }
                    None => no_test += 1,
                }
                if label >= 0 {
                    if let Some(d) = self
                        .residuals
                        .get(&(src.to_string(), level, *at, label as u32))
                        .and_then(|o| self.store.object_bytes(o))
                        .map(|b| decode_f32(&b))
                        .and_then(|d| d.first().copied())
                    {
                        deltas.push(d as f64);
                    }
                }
            }
            if unexp.is_empty() {
                println!("  {label:>7} — no channelized tile produced a test ({no_test} tiles absent)");
                continue;
            }
            let v = TileVerdict {
                channel_cells: chan,
                fitted_basins: basins,
                unexplained: median(&mut unexp),
                rms_m: median(&mut rms),
                slope: median(&mut slope),
                slope_pred: median(&mut pred),
                zero_param: median(&mut zero),
                u_shape: median(&mut ush),
                subsiding: median(&mut subs),
                u_span: median(&mut spans),
            };
            let d = if deltas.is_empty() { f64::NAN } else { median(&mut deltas) };
            let tr = if trunk.is_empty() { f64::NAN } else { median(&mut trunk) };
            println!(
                "  {:>7} {:>8} {:>7} {:>9.4} {:>9.4} {:>9.4} {:>9.4} {:>9.4} {:>9.4} {:>8.2} {:>9.4}{}",
                if label < 0 { "prior".to_string() } else { label.to_string() },
                v.channel_cells,
                v.fitted_basins,
                v.unexplained,
                v.u_shape,
                tr,
                v.zero_param,
                v.slope,
                v.slope_pred,
                v.slope / v.slope_pred,
                d,
                if no_test > 0 { format!("  ({no_test} no test)") } else { String::new() },
            );
            rows.push((label, v, d, tr));
        }

        let staged: Vec<&(i64, TileVerdict, f64, f64)> = rows.iter().filter(|r| r.0 >= 0).collect();
        let (Some(first), Some(last)) = (staged.first(), staged.last()) else { return };

        // Is the χ-linearity premise — a spatially invariant U — even satisfied?
        let last_stage = *ladder.last().unwrap();
        let last_tiles: Vec<(TileAt, String)> = cohort
            .iter()
            .filter_map(|(at, s)| s.get(&last_stage).map(|o| (*at, o.clone())))
            .collect();
        // The premise statistic must be measured on the tiles that produce a
        // test. Medianed over every tile in a mostly-ocean sweep it reports the
        // seabed — 377 of 384 inert tiles drowning the seven that were fitted.
        let (mut means, mut cv) = (Vec::new(), Vec::new());
        let (mut all_means, mut fitted) = (Vec::new(), 0usize);
        for (at, object) in &last_tiles {
            let Some(mut f) = self.seeded(src, level, *at, object) else { continue };
            let u: Vec<f64> = f.uplift_rate().iter().map(|&v| v as f64).collect();
            let mean = u.iter().sum::<f64>() / u.len() as f64;
            all_means.push(mean);
            if self.judge(&mut f, self.channel_min_cells).is_none() {
                continue;
            }
            fitted += 1;
            let sd = (u.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / u.len() as f64).sqrt();
            cv.push(sd / mean.abs().max(1e-12));
            means.push(mean);
        }
        println!("\n  == verdict ==");
        println!(
            "  driver over ALL {} tiles: means {:.4}…{:.4} m/epoch",
            all_means.len(),
            all_means.iter().cloned().fold(f64::INFINITY, f64::min),
            all_means.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        );
        println!(
            "  driver over the {fitted} tiles that produce a TEST: means {:.4}…{:.4}, within-tile σ/|mean| median {:.3}; max/min U along a fitted channel, median {:.3}",
            means.iter().cloned().fold(f64::INFINITY, f64::min),
            means.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            median(&mut cv),
            last.1.u_span,
        );
        if let Some(pr) = rows.iter().find(|r| r.0 < 0) {
            println!(
                "  uncarved prior      1-R²(√) = {:.4}   0-param = {:.4}",
                pr.1.unexplained, pr.1.zero_param
            );
        }
        println!(
            "  stage {:<5}         1-R²(√) = {:.4}   U-shape = {:.4}   0-param = {:.4}   mean|Δh| = {:.4} m/epoch",
            first.0, first.1.unexplained, first.1.u_shape, first.1.zero_param, first.2
        );
        println!(
            "  stage {:<5}         1-R²(√) = {:.4}   U-shape = {:.4}   0-param = {:.4}   mean|Δh| = {:.4} m/epoch",
            last.0, last.1.unexplained, last.1.u_shape, last.1.zero_param, last.2
        );
        // THE discrimination test. Under a spatially invariant U the two forms
        // are the same fit and must agree; they can only part where U varies
        // along a channel. A ratio at 1.000 is not a null result — it is the
        // premise reporting that it was never stressed.
        println!(
            "  literature form ÷ matched-parameter U-form: {:.3} at stage {}, {:.3} at stage {} — 1.000 means U did not vary enough to tell them apart",
            first.1.unexplained / first.1.u_shape,
            first.0,
            last.1.unexplained / last.1.u_shape,
            last.0,
        );
        println!(
            "  literature form falls on {}/{} steps (end/start {:.3}); zero-parameter form on {}/{} ({:.3}); trunk-only {:.4} → {:.4}",
            staged.windows(2).filter(|w| w[1].1.unexplained < w[0].1.unexplained).count(),
            staged.len() - 1,
            last.1.unexplained / first.1.unexplained,
            staged.windows(2).filter(|w| w[1].1.zero_param < w[0].1.zero_param).count(),
            staged.len() - 1,
            last.1.zero_param / first.1.zero_param,
            first.3,
            last.3,
        );
        println!(
            "  fitted dz/dχ vs the pure-SPL prediction: {:.2}× at stage {}, {:.2}× at stage {}; channel cells on U ≤ 0: {:.1}%",
            first.1.slope / first.1.slope_pred,
            first.0,
            last.1.slope / last.1.slope_pred,
            last.0,
            100.0 * last.1.subsiding,
        );
        println!(
            "  rms fit residual {:.2} m at the last stage — f32 at ~10³ m resolves ~1.2e-4 m, {:.0}× the noise floor",
            last.1.rms_m,
            last.1.rms_m / 1.2e-4
        );

        // The channelization sweep: a trunk-only network should be *more*
        // χ-linear, because headwaters are creep- and talus-governed and the
        // stream-power law does not claim them.
        print!("  channel threshold (median cell areas):");
        for t in [3.0f32, 10.0, 30.0, 100.0, 300.0] {
            let mut v = Vec::new();
            let mut cells = 0usize;
            for (at, object) in &last_tiles {
                let Some(mut f) = self.seeded(src, level, *at, object) else { continue };
                if let Some(j) = self.judge(&mut f, t) {
                    v.push(j.unexplained);
                    cells += j.channel_cells;
                }
            }
            if v.is_empty() {
                print!("   {t:>5.0}: no test");
            } else {
                print!("   {t:>5.0}: {:.3} ({cells})", median(&mut v));
            }
        }
        println!();

        // The tile with the most channelized cells — the one whose network has
        // the most to say — for the known-bads and the extension.
        let Some((at, object)) = last_tiles
            .iter()
            .max_by_key(|(at, o)| {
                self.seeded(src, level, *at, o)
                    .and_then(|mut f| self.judge(&mut f, self.channel_min_cells))
                    .map(|v| v.channel_cells)
                    .unwrap_or(0)
            })
            .cloned()
        else {
            return;
        };
        let Some(mut f) = self.seeded(src, level, at, &object) else { return };
        self.known_bads(&format!("stage {last_stage}, tile f{} ({},{})", at.0, at.1, at.2), &mut f);

        // The ladder ends where it ends. Whether the last stage is near an
        // asymptote or a tenth of the way is not answerable from inside it, so
        // carry one tile past. Computes; writes nothing.
        if extend > last_stage {
            println!(
                "\n  == past the end of the ladder: the same tile carried to {extend} epochs (computed here, never stored) =="
            );
            println!(
                "  {:>7} {:>9} {:>9} {:>9} {:>9} {:>8} {:>9}",
                "epoch", "1-R²(√)", "U-shape", "trunk", "0-param", "ratio", "mean|Δh|"
            );
            let step = ((extend - last_stage) / 12).max(1);
            let mut epoch = last_stage;
            loop {
                let v = self.judge(&mut f, self.channel_min_cells);
                let t = self.judge(&mut f, self.trunk_min_cells);
                if let Some(v) = v {
                    println!(
                        "  {:>7} {:>9.4} {:>9.4} {:>9.4} {:>9.4} {:>8.2} {:>9}",
                        epoch,
                        v.unexplained,
                        v.u_shape,
                        t.map(|t| t.unexplained).unwrap_or(f64::NAN),
                        v.zero_param,
                        v.slope / v.slope_pred,
                        if f.last_delta_m.is_finite() {
                            format!("{:.4}", f.last_delta_m)
                        } else {
                            "—".to_string()
                        },
                    );
                }
                if epoch >= extend {
                    break;
                }
                let take = step.min(extend - epoch);
                f.erode(&FluvialParams { epochs: take, ..self.p.clone() });
                epoch += take;
            }
            self.known_bads(&format!("the same tile at {extend} epochs"), &mut f);
        }
    }

    /// **The discrimination test the tiling cannot provide.**
    ///
    /// FE(2) earns its place only if the per-cell form beats the literature form
    /// somewhere, and it can only do that where $U$ varies **along a channel**.
    /// Measured, it does not: the driver varies ~24% across an L9 tile, but a
    /// fitted basin's channel samples ~1.5% of that, because every partial tile
    /// makes its whole edge ring a base level and no basin can exceed one tile
    /// ( #obs-tile-outlets-grade-away-the-basins ). Two facts compose into one
    /// blind spot, and neither alone would cause it.
    ///
    /// So: remove the cause. A tile covering a **whole cube face** is the one
    /// configuration `Fluvial::outlets` gives coast-only base levels, so its
    /// basins can run continental distances and cross the driver's gradient.
    /// This assembles one face from its stored tiles, runs the kernel on it as a
    /// single field, and asks the same question of a network that can finally
    /// answer it.
    ///
    /// **What this is and is not.** It is a controlled experiment about the
    /// *criterion*, run in memory and never stored. It is **not** a claim about
    /// the world: the builder never ran this network, and assembling stored tiles
    /// is separately measured to manufacture basins at the seams. Read it as
    /// "what would the two forms do given a channel that crosses a $U$ gradient",
    /// nothing more.
    fn whole_face_discrimination(&self, src: &str, level: u8, cohort: &Cohort, epochs: u32) {
        let face_n = 1usize << level;
        let per_side = face_n / 64;
        let last = cohort.values().flat_map(|s| s.keys().copied()).max().unwrap_or(0);
        // The face with the most tiles present at the last stage.
        let mut by_face: BTreeMap<u8, Vec<(TileAt, String)>> = BTreeMap::new();
        for (at, stages) in cohort {
            if at.3 != 64 {
                continue;
            }
            if let Some(o) = stages.get(&last) {
                by_face.entry(at.0).or_default().push((*at, o.clone()));
            }
        }
        // The landiest face, not merely the first complete one: fluvial form is a
        // property of land, and an arbitrary tie-break here would put the whole
        // experiment on whichever face sorted last.
        let sea = vivarium_world::sea_level::derived_sea_level_m(self.seed) as f32;
        let Some((face, tiles)) = by_face.into_iter().max_by_key(|(_, v)| {
            v.iter()
                .filter_map(|(_, o)| self.store.object_bytes(o))
                .map(|b| decode_f32(&b).iter().filter(|&&x| x > sea).count())
                .sum::<usize>()
        }) else {
            return;
        };
        if tiles.len() < per_side * per_side {
            println!(
                "\n\n=== whole-face discrimination: face {face} has {}/{} tiles at stage {last} — not assembling a partial face ===",
                tiles.len(),
                per_side * per_side
            );
            return;
        }
        println!(
            "\n\n=== whole-face discrimination: face {face} at L{level} assembled from {} stored tiles, {face_n}² cells, coast-only base level ===",
            tiles.len()
        );
        println!("    (computed in memory, never stored; a network the builder never ran — this is a test of the CRITERION, not a claim about the world)");

        let n = face_n * face_n;
        let (mut h, mut up, mut pr) = (vec![0.0f32; n], vec![0.0f32; n], vec![1.0f32; n]);
        for (at, object) in &tiles {
            let (_, oi, oj, nx) = *at;
            let Some(th) = self.store.object_bytes(object).map(|b| decode_f32(&b)) else { continue };
            let key = |k| (k, src.to_string(), level, *at);
            let Some(tu) = self.inputs.get(&key("uplift-tile")).and_then(|o| self.store.object_bytes(o)).map(|b| decode_f32(&b)) else { continue };
            let Some(tp) = self.inputs.get(&key("climate")).and_then(|o| self.store.object_bytes(o)).map(|b| decode_f32(&b)) else { continue };
            if th.len() != nx * nx || tu.len() != nx * nx || tp.len() != nx * nx {
                continue;
            }
            for y in 0..nx {
                for x in 0..nx {
                    let d = (oj as usize + y) * face_n + oi as usize + x;
                    if d < n {
                        h[d] = th[y * nx + x];
                        up[d] = tu[y * nx + x];
                        pr[d] = tp[y * nx + x];
                    }
                }
            }
        }
        let mean = pr.iter().sum::<f32>() / pr.len() as f32;
        let pw: Vec<f32> = if mean > 0.0 { pr.iter().map(|v| v / mean).collect() } else { vec![1.0; n] };
        let mut f = Fluvial::from_surface(self.seed, Face::from_index(face), level, 0, 0, face_n, |_| 0.0);
        f.h = h;
        f.set_uplift_rate(up);
        f.set_precip_weight(pw);

        println!(
            "\n  {:>7} {:>8} {:>7} {:>10} {:>9} {:>9} {:>11} {:>9}",
            "epoch", "channel", "basins", "1-R²(√)", "U-shape", "lit÷U", "maxU/minU", "ratio"
        );
        let step = (epochs / 6).max(1);
        let mut epoch = 0u32;
        loop {
            if let Some(v) = self.judge(&mut f, self.channel_min_cells) {
                println!(
                    "  {epoch:>7} {:>8} {:>7} {:>10.4} {:>9.4} {:>9.3} {:>11.3} {:>9.2}",
                    v.channel_cells,
                    v.fitted_basins,
                    v.unexplained,
                    v.u_shape,
                    v.unexplained / v.u_shape,
                    v.u_span,
                    v.slope / v.slope_pred,
                );
            } else {
                println!("  {epoch:>7}   no basin large enough — no test");
            }
            if epoch >= epochs {
                break;
            }
            let take = step.min(epochs - epoch);
            f.erode(&FluvialParams { epochs: take, ..self.p.clone() });
            epoch += take;
        }
        println!("  lit÷U above 1 means the per-cell form (FE(2)) explains the profile better than χ does;");
        println!("  it can only rise above 1 where maxU/minU along a channel does, which is the whole point of the column.");
    }

    /// Does the converged slope ratio track $1+G$? If it does, the composition
    /// has a derivable effective $K$ and the criterion's *rate* half becomes a
    /// gate rather than a diagnostic ( `#obs-chi-shape-is-erosions-criterion`
    /// FE(5) and its first Working Note). Each run settles from the same stored
    /// stage with only `deposition` changed, so the comparison isolates $G$ —
    /// that nothing else moves is the sweep's whole point.
    /// Several tiles rather than one, because a single landscape's drainage
    /// rearrangements land where they land: one tile's $G$ row can sit well off
    /// its neighbours for no reason but a basin capture, and a trend read from
    /// that is a trend read from noise. Medians across tiles; the per-tile spread
    /// is printed so the reader can see whether the median means anything.
    fn deposition_sweep(&self, src: &str, level: u8, tiles: &[(TileAt, String)], epochs: u32) {
        println!(
            "\n\n=== deposition sweep: does the rate half's offset track 1+G? ({} tiles, +{epochs} epochs each) ===",
            tiles.len()
        );
        println!(
            "  {:>6} {:>9} {:>9} {:>9} {:>8} {:>12} {:>14} {:>16}",
            "G", "1-R²(√)", "trunk", "0-param", "ratio", "ratio/(1+G)", "ratio/(1+G/2)", "ratio spread"
        );
        for g in [0.0f32, 0.25, 0.5, 1.0, 2.0] {
            let p = FluvialParams { deposition: g, epochs, ..self.p.clone() };
            let sub = Probe { p: p.clone(), ..*self };
            let (mut un, mut tr, mut zp, mut ra) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            for (at, object) in tiles {
                let Some(mut f) = self.seeded(src, level, *at, object) else { continue };
                f.erode(&p);
                let Some(v) = sub.judge(&mut f, self.channel_min_cells) else { continue };
                if let Some(t) = sub.judge(&mut f, self.trunk_min_cells) {
                    tr.push(t.unexplained);
                }
                un.push(v.unexplained);
                zp.push(v.zero_param);
                ra.push(v.slope / v.slope_pred);
            }
            if ra.is_empty() {
                println!("  {g:>6.2}   no test");
                continue;
            }
            let ratio = median(&mut ra);
            println!(
                "  {g:>6.2} {:>9.4} {:>9.4} {:>9.4} {:>8.2} {:>12.3} {:>14.3} {:>7.2}…{:<8.2}",
                median(&mut un),
                median(&mut tr),
                median(&mut zp),
                ratio,
                ratio / (1.0 + g as f64),
                ratio / (1.0 + 0.5 * g as f64),
                ra.iter().cloned().fold(f64::INFINITY, f64::min),
                ra.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            );
        }
        println!("  (a flat ratio/(1+G) would mean the offset IS the Davy–Lague factor and the rate half is derivable.");
        println!("   The second normalization is an OBSERVED REGULARITY with no derivation behind it — the candidate");
        println!("   mechanism is that only part of the eroded volume is re-laid inside the network and the rest");
        println!("   reaches an outlet and is lost to the sea, which is unverified. Do not cite it as a law.)");
    }
}

fn main() {
    let channel_min_cells: f32 =
        std::env::var("CHI_CHANNEL_CELLS").ok().and_then(|v| v.parse().ok()).unwrap_or(10.0);
    let extend: u32 = std::env::var("CHI_EXTEND").ok().and_then(|v| v.parse().ok()).unwrap_or(3000);
    let g_sweep = std::env::var("CHI_G_SWEEP").map(|v| v != "0").unwrap_or(true);
    let pinned = std::env::var("CHI_SRC").ok();
    let world_dir = std::env::var("VIVARIUM_WORLD").map(std::path::PathBuf::from).unwrap_or_else(|_| {
        std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".cache/vivarium/globe-world")
    });
    let Ok(store) = Store::open_read_only(&world_dir) else {
        println!("(no world at {} — nothing to measure)", world_dir.display());
        return;
    };
    let Ok(Some(spec)) = vivarium_world::spec::WorldSpec::load(&world_dir) else {
        println!("(world at {} has no manifest)", world_dir.display());
        return;
    };
    let Ok(roots) = store.roots() else {
        println!("(world at {} has no roots)", world_dir.display());
        return;
    };

    let mut cohorts: BTreeMap<(String, u8), Cohort> = BTreeMap::new();
    let mut drivers: BTreeMap<(String, u8), String> = BTreeMap::new();
    let mut residuals: BTreeMap<(String, u8, TileAt, u32), String> = BTreeMap::new();
    let mut inputs: BTreeMap<(&'static str, String, u8, TileAt), String> = BTreeMap::new();
    for r in &roots {
        let k = r.key.as_str();
        let kind = match k.split('@').next().unwrap_or("") {
            "erosion-tile" => "erosion-tile",
            "uplift-tile" => "uplift-tile",
            "climate" => "climate",
            "initial-topography" => "initial-topography",
            _ => continue,
        };
        let (Some(src), Some(face), Some(level), Some(oi), Some(oj), Some(nx)) = (
            key_field(k, "src").map(str::to_string),
            key_field(k, "face").and_then(|v| v.parse::<u8>().ok()),
            key_field(k, "level").and_then(|v| v.parse::<u8>().ok()),
            key_field(k, "oi").and_then(|v| v.parse::<u32>().ok()),
            key_field(k, "oj").and_then(|v| v.parse::<u32>().ok()),
            key_field(k, "nx").and_then(|v| v.parse::<usize>().ok()),
        ) else {
            continue;
        };
        let at: TileAt = (face, oi, oj, nx);
        if kind != "erosion-tile" {
            inputs.insert((kind, src, level, at), r.object.clone());
            continue;
        }
        let epochs = key_field(k, "epochs").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        match key_field(k, "aspect") {
            Some("stage-residual") => {
                residuals.insert((src, level, at, epochs), r.object.clone());
            }
            Some(_) => {}
            None => {
                // The uplift nomos version rides in the erosion key as a dep, so
                // a cohort can name its own driver without a second lookup.
                if let Some(u) = key_field(k, "uplift-tile") {
                    drivers.insert((src.clone(), level), u.to_string());
                }
                cohorts
                    .entry((src, level))
                    .or_default()
                    .entry(at)
                    .or_default()
                    .insert(epochs, r.object.clone());
            }
        }
    }

    println!("== world \"{}\" (seed {}) — settle histories in the store ==", spec.name, spec.seed);
    let cur = vivarium_world::nomotheke::SRC_HASH;
    let ladder_len = |c: &Cohort| {
        c.values().flat_map(|s| s.keys().copied()).collect::<std::collections::BTreeSet<_>>().len()
    };
    for ((src, level), c) in &cohorts {
        println!(
            "   src={src} L{level:<2}  {:>4} tiles  {:>3} stages   driver {}{}",
            c.len(),
            ladder_len(c),
            drivers.get(&(src.clone(), *level)).map(String::as_str).unwrap_or("?"),
            if src == cur { "  <- running binary" } else { "" },
        );
    }

    // Every longest chain, not one of them. A silent pick is how a cohort
    // comparison becomes an accident, and these differ in their driver.
    let max_len = cohorts.values().map(&ladder_len).max().unwrap_or(0);
    if max_len < 3 {
        println!("\n(the longest chain has {max_len} stages — a settle history needs an interior to measure)");
        return;
    }
    let selected: Vec<(String, u8)> = cohorts
        .iter()
        .filter(|((s, _), c)| match &pinned {
            Some(p) => s == p,
            None => ladder_len(c) == max_len,
        })
        .map(|(k, _)| k.clone())
        .collect();
    let p = FluvialParams::default();
    println!(
        "\n   measuring {} cohort(s); k_dt {} · m {} · deposition G {} · talus {} · κ {} m²/epoch · channel ≥ {channel_min_cells} median cell areas · basins fitted at ≥ {MIN_BASIN_CELLS} cells",
        selected.len(),
        p.k_dt,
        p.m,
        p.deposition,
        p.max_slope,
        p.diffusivity_m2
    );

    let probe = Probe {
        store: &store,
        seed: spec.seed,
        inputs: &inputs,
        residuals: &residuals,
        p,
        channel_min_cells,
        trunk_min_cells: channel_min_cells * 10.0,
    };
    for (src, level) in &selected {
        let driver = drivers.get(&(src.clone(), *level)).map(String::as_str).unwrap_or("?");
        probe.measure_cohort(src, *level, &cohorts[&(src.clone(), *level)], driver, extend);
    }

    // The G sweep runs once, on the last selected cohort — the rate half's open
    // question, not a per-cohort statistic.
    if g_sweep {
        let Some((src, level)) = selected.last().cloned() else { return };
        let cohort = &cohorts[&(src.clone(), level)];
        let last_stage = cohort.values().flat_map(|s| s.keys().copied()).max().unwrap_or(0);
        let mut tiles: Vec<(TileAt, String)> = cohort
            .iter()
            .filter_map(|(at, s)| s.get(&last_stage).map(|o| (*at, o.clone())))
            .collect();
        // The most channelized tiles — the ones whose networks have the most to
        // say — but more than one, so a single basin capture cannot set a trend.
        tiles.sort_by_key(|(at, o)| {
            std::cmp::Reverse(
                probe
                    .seeded(&src, level, *at, o)
                    .and_then(|mut f| probe.judge(&mut f, channel_min_cells))
                    .map(|v| v.channel_cells)
                    .unwrap_or(0),
            )
        });
        tiles.truncate(6);
        probe.deposition_sweep(&src, level, &tiles, extend.max(1500));
    }

    // The discrimination test, on the coarsest cohort available — a whole face
    // is the only footprint whose basins can cross the driver's gradient.
    if std::env::var("CHI_FACE").map(|v| v != "0").unwrap_or(true) {
        let face_epochs: u32 =
            std::env::var("CHI_FACE_EPOCHS").ok().and_then(|v| v.parse().ok()).unwrap_or(600);
        if let Some((src, level)) =
            selected.iter().min_by_key(|(_, l)| *l).cloned()
        {
            probe.whole_face_discrimination(&src, level, &cohorts[&(src.clone(), level)], face_epochs);
        }
    }
}
