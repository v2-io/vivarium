//! Does erosion have a convergence criterion that can *fail*? The candidate is a
//! **shape** test, measured stage by stage against a real settle history.
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
//! This probe reads the beacon's settle history out of the store and asks, at
//! every stage: how much of the channel network's elevation variance is left
//! unexplained by $\chi$, what slope does the fit recover, and what was the
//! recorded mean $|\Delta h|$ at that same stage. Three numbers, one of which is
//! supposed to fall while another is supposed to sit still.
//!
//! **Read-only, and never computes.** Every field — heights, uplift, climate,
//! recorded residual — is loaded from store roots carrying the *same* `src=`
//! source digest as the stage being examined, so the uplift field is the one that
//! actually drove that stage rather than whatever the running binary would
//! produce now. A cohort whose `src=` is not the running binary's is still a
//! faithful history of the world that produced it ( #form-time-indexed-stage-chains
//! FE(11)), and the cohort is named on screen.
//!
//! Run: `cargo run --release --example chi_convergence_probe`
//! Env: `VIVARIUM_WORLD` (world dir), `CHI_CHANNEL_CELLS` (channelization
//! threshold in median cell areas, default 10).

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

/// A tile's identity within one cohort.
type TileAt = (u8, u32, u32, usize); // face, oi, oj, nx

/// What one tile at one stage says about the shape criterion.
#[derive(Default, Clone, Copy)]
struct TileVerdict {
    channel_cells: usize,
    fitted_basins: usize,
    /// $\sqrt{1-R^2}$ — the fraction of channel-elevation *amplitude* that χ
    /// leaves unexplained, pooled over the tile's fitted basins. Dimensionless,
    /// so it is comparable across stages whose relief is growing.
    unexplained: f64,
    /// RMS fit residual in metres, for the f32-noise-floor sanity check.
    rms_m: f64,
    /// Median fitted $\mathrm{d}z/\mathrm{d}\chi$ over fitted basins.
    slope: f64,
    /// $U/(k_{dt}A_0^{m})$ — what the *pure* incision–uplift balance predicts.
    slope_pred: f64,
    /// Normalized departure from the zero-parameter steady profile.
    zero_param: f64,
}

/// Fit every basin of one tile and pool the result.
fn judge(f: &mut Fluvial, p: &FluvialParams, channel_min_cells: f32) -> Option<TileVerdict> {
    let mut med = f.cell_area.clone();
    med.sort_by(f32::total_cmp);
    let a0 = med[med.len() / 2];
    let thresh = channel_min_cells * a0;
    let prof = f.chi_profile(p, a0);

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
    // A tile with no channel produces NO test. That is the fail-safe: an inert
    // tile must not be reported as converged.
    let (mut ss_res, mut ss_tot, mut n_fit) = (0.0f64, 0.0f64, 0usize);
    let (mut ss_zero, mut ss_z_tot) = (0.0f64, 0.0f64);
    let (mut slopes, mut preds) = (Vec::new(), Vec::new());
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
        ss_z_tot += y.iter().map(|b| (b - my).powi(2)).sum::<f64>();
        n_fit += cells.len();
        slopes.push(s);
        preds.push(
            cells.iter().map(|&i| f.uplift_rate()[i] as f64).sum::<f64>() / cells.len() as f64
                / (p.k_dt as f64 * (a0 as f64).powf(p.m as f64)),
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
        zero_param: (ss_zero / ss_z_tot.max(f64::MIN_POSITIVE)).sqrt(),
    })
}

fn main() {
    let channel_min_cells: f32 = std::env::var("CHI_CHANNEL_CELLS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10.0);
    let world_dir = std::env::var("VIVARIUM_WORLD")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
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

    // ---- Census the settle histories present, grouped by (src, level) ----
    // Two stages carved under different source digests are stages of two
    // different worlds ( #form-time-indexed-stage-chains FE(11)); never merge.
    type Cohort = BTreeMap<TileAt, BTreeMap<u32, String>>; // tile → epochs → object
    let mut cohorts: BTreeMap<(String, u8), Cohort> = BTreeMap::new();
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
                cohorts.entry((src, level)).or_default().entry(at).or_default().insert(epochs, r.object.clone());
            }
        }
    }

    println!("== world \"{}\" (seed {}) — settle histories in the store ==", spec.name, spec.seed);
    let cur = vivarium_world::nomotheke::SRC_HASH;
    let mut best: Option<(String, u8)> = None;
    for ((src, level), c) in &cohorts {
        let ladder: std::collections::BTreeSet<u32> =
            c.values().flat_map(|s| s.keys().copied()).collect();
        let here = if src == cur { " <- running binary" } else { "" };
        println!(
            "   src={src} L{level:<2}  {:>4} tiles  {:>3} stages ({}…{}){here}",
            c.len(),
            ladder.len(),
            ladder.iter().next().copied().unwrap_or(0),
            ladder.iter().next_back().copied().unwrap_or(0),
        );
        let score = (ladder.len(), c.len());
        if best.as_ref().is_none_or(|b| {
            let bc = &cohorts[b];
            let bl: std::collections::BTreeSet<u32> =
                bc.values().flat_map(|s| s.keys().copied()).collect();
            score > (bl.len(), bc.len())
        }) {
            best = Some((src.clone(), *level));
        }
    }
    let Some((src, level)) = best else {
        println!("(no erosion stages in the store — nothing to measure)");
        return;
    };
    let cohort = &cohorts[&(src.clone(), level)];
    let ladder: Vec<u32> = cohort.values().flat_map(|s| s.keys().copied()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    if ladder.len() < 3 {
        println!("\n(the longest chain has {} stages — a settle history needs an interior to measure)", ladder.len());
        return;
    }
    let cell_km = cell_size_m(level, Planet::EARTH.radius_m) / 1000.0;
    println!(
        "\n== measuring the longest chain: src={src} L{level} ({:.2} km/cell), {} tiles × {} stages ==",
        cell_km,
        cohort.len(),
        ladder.len()
    );
    if src != cur {
        println!("   (this cohort is NOT the running binary's source tree — it is a faithful history of the world that carved it, read under its own name)");
    }
    let p = FluvialParams::default();
    println!(
        "   k_dt {} · m {} · deposition G {} · talus {} · κ {} m²/epoch · channel ≥ {channel_min_cells} median cell areas · basins fitted at ≥ {MIN_BASIN_CELLS} channel cells",
        p.k_dt, p.m, p.deposition, p.max_slope, p.diffusivity_m2
    );

    // ---- One tile, one stage → a Fluvial seeded exactly as the build had it ----
    let seeded = |at: TileAt, object: &str| -> Option<Fluvial> {
        let (face, oi, oj, nx) = at;
        let h = decode_f32(&store.object_bytes(object)?);
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
            seed: spec.seed,
        });
        // The uplift and climate fields from the SAME source tree that carved
        // this stage — not what the running binary would compute now.
        let uplift = decode_f32(&store.object_bytes(inputs.get(&("uplift-tile", src.clone(), level, at))?)?);
        let precip = decode_f32(&store.object_bytes(inputs.get(&("climate", src.clone(), level, at))?)?);
        if uplift.len() != nx * nx || precip.len() != nx * nx {
            return None;
        }
        let mean = precip.iter().sum::<f32>() / precip.len() as f32;
        let pw: Vec<f32> =
            if mean > 0.0 { precip.iter().map(|v| v / mean).collect() } else { vec![1.0; precip.len()] };
        f.set_uplift_rate(uplift);
        f.set_precip_weight(pw);
        Some(f)
    };

    // ---- Stage 0: the uncarved surface, as the criterion's known-bad ----
    // Not a synthetic: the actual initial topography these stages were carved
    // from. If the shape test cannot tell an uneroded fated-noise surface from a
    // settled one, it is not a test.
    // Two channelization thresholds, because the shape statement is about
    // *channels*: headwaters are creep- and talus-governed, not stream-power-
    // governed, and a threshold that admits them is measuring the wrong cells.
    let trunk_min_cells = 100.0f32 * channel_min_cells / 10.0;
    println!(
        "\n  {:>7} {:>8} {:>7} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>10}",
        "stage", "channel", "basins", "1-R² (√)", "trunk", "rms (m)", "dz/dχ", "predicted", "ratio", "mean|Δh|"
    );
    let mut rows: Vec<(i64, TileVerdict, f64, f64)> = Vec::new();
    let stage0: Vec<(TileAt, String)> = cohort
        .keys()
        .filter_map(|at| {
            inputs.get(&("initial-topography", src.clone(), level, *at)).map(|o| (*at, o.clone()))
        })
        .collect();
    for (label, tiles) in std::iter::once((-1i64, stage0))
        .chain(ladder.iter().map(|&k| {
            (k as i64, cohort.iter().filter_map(|(at, s)| s.get(&k).map(|o| (*at, o.clone()))).collect::<Vec<_>>())
        }))
    {
        let (mut unexp, mut rms, mut slope, mut pred, mut zero) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        let mut trunk: Vec<f64> = Vec::new();
        let (mut chan, mut basins, mut no_test) = (0usize, 0usize, 0usize);
        let mut deltas: Vec<f64> = Vec::new();
        for (at, object) in &tiles {
            let Some(mut f) = seeded(*at, object) else { continue };
            if let Some(t) = judge(&mut f, &p, trunk_min_cells) {
                trunk.push(t.unexplained);
            }
            match judge(&mut f, &p, channel_min_cells) {
                Some(v) => {
                    chan += v.channel_cells;
                    basins += v.fitted_basins;
                    unexp.push(v.unexplained);
                    rms.push(v.rms_m);
                    slope.push(v.slope);
                    pred.push(v.slope_pred);
                    zero.push(v.zero_param);
                }
                None => no_test += 1,
            }
            if label >= 0 {
                if let Some(o) = residuals.get(&(src.clone(), level, *at, label as u32)) {
                    if let Some(d) = store.object_bytes(o).map(|b| decode_f32(&b)) {
                        if let Some(&v) = d.first() {
                            deltas.push(v as f64);
                        }
                    }
                }
            }
        }
        if unexp.is_empty() {
            println!("  {:>7} — no channelized tile produced a test ({} tiles absent)", label, no_test);
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
        };
        let d = if deltas.is_empty() { f64::NAN } else { median(&mut deltas) };
        let tr = if trunk.is_empty() { f64::NAN } else { median(&mut trunk) };
        println!(
            "  {:>7} {:>8} {:>7} {:>9.4} {:>9.4} {:>9.3} {:>9.4} {:>9.4} {:>9.3} {:>10.4}{}",
            if label < 0 { "prior".to_string() } else { label.to_string() },
            v.channel_cells,
            v.fitted_basins,
            v.unexplained,
            tr,
            v.rms_m,
            v.slope,
            v.slope_pred,
            v.slope / v.slope_pred,
            d,
            if no_test > 0 { format!("  ({no_test} tiles produced no test)") } else { String::new() },
        );
        rows.push((label, v, d, tr));
    }

    // ---- What the columns say ----
    println!("\n  1-R² (√): fraction of channel-elevation amplitude χ leaves unexplained — the shape criterion.");
    println!("  dz/dχ vs predicted: the fitted slope against U/(k_dt·A₀^m), the PURE incision–uplift balance.");
    println!("  mean|Δh|: the stage's own recorded final-epoch residual — the criterion that provably cannot work.");

    println!("  trunk: the same statistic restricted to channels above {trunk_min_cells} median cell areas.");

    let staged: Vec<&(i64, TileVerdict, f64, f64)> = rows.iter().filter(|r| r.0 >= 0).collect();
    if let (Some(first), Some(last)) = (staged.first(), staged.last()) {
        let prior = rows.iter().find(|r| r.0 < 0);
        println!("\n== verdict ==");
        if let Some(pr) = prior {
            println!(
                "  uncarved prior      1-R²(√) = {:.4}   → the known-bad the criterion has to reject",
                pr.1.unexplained
            );
        }
        println!(
            "  stage {:<5}         1-R²(√) = {:.4}   mean|Δh| = {:.4} m/epoch",
            first.0, first.1.unexplained, first.2
        );
        println!(
            "  stage {:<5}         1-R²(√) = {:.4}   mean|Δh| = {:.4} m/epoch",
            last.0, last.1.unexplained, last.2
        );
        let falls = staged.windows(2).filter(|w| w[1].1.unexplained < w[0].1.unexplained).count();
        println!(
            "  the shape residual falls on {}/{} stage steps; end/start = {:.3} (trunk-only {:.4} → {:.4}, {}/{} falling)",
            falls,
            staged.len() - 1,
            last.1.unexplained / first.1.unexplained,
            first.3,
            last.3,
            staged.windows(2).filter(|w| w[1].3 < w[0].3).count(),
            staged.len() - 1,
        );
        let dfirst = first.2;
        let dlast = last.2;
        println!("  the recorded |Δh| goes {dfirst:.4} → {dlast:.4} m/epoch (ratio {:.3})", dlast / dfirst);
        println!(
            "  fitted dz/dχ vs the pure-SPL prediction: {:.2}× at stage {}, {:.2}× at stage {}",
            first.1.slope / first.1.slope_pred,
            first.0,
            last.1.slope / last.1.slope_pred,
            last.0
        );
        println!(
            "  departure from the zero-parameter steady profile: {:.3} → {:.3} (normalized)",
            first.1.zero_param, last.1.zero_param
        );
        println!(
            "  rms fit residual is {:.3} m at the last stage — f32 at ~10³ m resolves ~1.2e-4 m, so this is {:.0}× the noise floor",
            last.1.rms_m,
            last.1.rms_m / 1.2e-4
        );
    }

    // ---- Is the premise even satisfied, and is the verdict an artifact of the
    // instrument's own choices? ( #norm-probe-sensitivity ) ----
    println!("\n== sensitivity: does the verdict survive the instrument's choices? ==");
    let last_stage = *ladder.last().unwrap();
    let last_tiles: Vec<(TileAt, String)> =
        cohort.iter().filter_map(|(at, s)| s.get(&last_stage).map(|o| (*at, o.clone()))).collect();

    // (a) The χ-linearity premise is "spatially invariant U". Say how invariant.
    let (mut us, mut per_tile_cv) = (Vec::new(), Vec::new());
    for (at, object) in &last_tiles {
        let Some(f) = seeded(*at, object) else { continue };
        let u: Vec<f64> = f.uplift_rate().iter().map(|&v| v as f64).collect();
        let mean = u.iter().sum::<f64>() / u.len() as f64;
        let sd = (u.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / u.len() as f64).sqrt();
        per_tile_cv.push(sd / mean.abs().max(1e-12));
        us.push(mean);
    }
    println!(
        "  uplift over the patch: tile means {:.4}…{:.4} m/epoch, within-tile sd/mean median {:.3}",
        us.iter().cloned().fold(f64::INFINITY, f64::min),
        us.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        median(&mut per_tile_cv),
    );

    // (b) Channelization threshold. A trunk-only network should be *more*
    // χ-linear than one including headwaters, where creep, not stream power,
    // sets the form. If the verdict does not move, it is not about the threshold.
    print!("  channel threshold (median cell areas):");
    for t in [3.0f32, 10.0, 30.0, 100.0, 300.0] {
        let mut v = Vec::new();
        let mut cells = 0usize;
        for (at, object) in &last_tiles {
            let Some(mut f) = seeded(*at, object) else { continue };
            if let Some(j) = judge(&mut f, &p, t) {
                v.push(j.unexplained);
                cells += j.channel_cells;
            }
        }
        if v.is_empty() {
            print!("   {t:>5.0}: no test");
        } else {
            print!("   {t:>5.0}: {:.3} ({cells} cells)", median(&mut v));
        }
    }
    println!();

    // (c) Would a longer chain keep improving? The slope ratio is the quantity
    // still moving at the end of the ladder, so report its own trend rather than
    // asserting a limit the ladder does not reach.
    if let (true, Some(last)) = (staged.len() >= 12, staged.last()) {
        let tail = &staged[staged.len() - 10..];
        let x: Vec<f64> = tail.iter().map(|r| r.0 as f64).collect();
        let y: Vec<f64> = tail.iter().map(|r| (r.1.slope / r.1.slope_pred).ln()).collect();
        let (s, _) = ols(&x, &y);
        println!(
            "  over the last {} stages the slope ratio decays at {:.3}/100 epochs (e-folding ≈ {:.0} epochs); it is {:.2}× at stage {}",
            tail.len(),
            -s * 100.0,
            if s < 0.0 { -1.0 / s } else { f64::INFINITY },
            last.1.slope / last.1.slope_pred,
            last.0
        );
    }

    // ---- Known-bads and the sensitivity statement ( #norm-probe-sensitivity ) ----
    // Run on a *real* network, and on the measured profile rather than an
    // idealization: the question is not "can a fit see a step" but "can this
    // criterion see a step over the residual the landscape already carries".
    let known_bads = |label: &str, f: &mut Fluvial| {
        let mut med = f.cell_area.clone();
        med.sort_by(f32::total_cmp);
        let a0 = med[med.len() / 2];
        let prof = f.chi_profile(&p, a0);
        let thresh = channel_min_cells * a0;
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
        let unexplained = |x: &[f64], y: &[f64]| -> f64 {
            let (s, c) = ols(x, y);
            let my = y.iter().sum::<f64>() / y.len() as f64;
            let res: f64 = x.iter().zip(y).map(|(a, b)| (b - (c + s * a)).powi(2)).sum();
            let tot: f64 = y.iter().map(|b| (b - my).powi(2)).sum();
            (res / tot.max(f64::MIN_POSITIVE)).sqrt()
        };
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
    };

    // (d) The ladder ends where it ends. Whether stage 300 is near an asymptote
    // or a tenth of the way is not answerable from inside the ladder — so run
    // one tile *past* it. This computes; it writes nothing, and it is not a
    // world artifact ( #form-core-view-wall : the store is untouched).
    let extend: u32 = std::env::var("CHI_EXTEND").ok().and_then(|v| v.parse().ok()).unwrap_or(3000);
    // The tile with the most channelized cells at the last stage — the one whose
    // network has the most to say.
    let Some((at, object)) = last_tiles
        .iter()
        .max_by_key(|(at, o)| {
            seeded(*at, o)
                .and_then(|mut f| judge(&mut f, &p, channel_min_cells))
                .map(|v| v.channel_cells)
                .unwrap_or(0)
        })
        .cloned()
    else {
        return;
    };
    let Some(mut f) = seeded(at, &object) else { return };
    println!("\n== known-bads: what the shape test convicts, and what it cannot ==");
    known_bads(&format!("stage {last_stage}, tile f{} ({},{})", at.0, at.1, at.2), &mut f);

    if extend > last_stage {
        println!(
            "\n== past the end of the ladder: the same tile carried to {extend} epochs (computed here, never stored) ==",
        );
        println!(
            "  {:>7} {:>9} {:>9} {:>9} {:>9} {:>10}",
            "epoch", "1-R² (√)", "trunk", "dz/dχ", "ratio", "mean|Δh|"
        );
        let step = ((extend - last_stage) / 12).max(1);
        let mut at_epoch = last_stage;
        loop {
            let v = judge(&mut f, &p, channel_min_cells);
            let t = judge(&mut f, &p, trunk_min_cells);
            if let Some(v) = v {
                println!(
                    "  {:>7} {:>9.4} {:>9.4} {:>9.4} {:>9.3} {:>10}",
                    at_epoch,
                    v.unexplained,
                    t.map(|t| t.unexplained).unwrap_or(f64::NAN),
                    v.slope,
                    v.slope / v.slope_pred,
                    if f.last_delta_m.is_finite() {
                        format!("{:.4}", f.last_delta_m)
                    } else {
                        "—".to_string()
                    },
                );
            }
            if at_epoch >= extend {
                break;
            }
            let take = step.min(extend - at_epoch);
            f.erode(&FluvialParams { epochs: take, ..p.clone() });
            at_epoch += take;
        }
        known_bads(&format!("the same tile at {extend} epochs"), &mut f);
        println!(
            "\n  A tile with no channel appears in the stage table as \"produced no test\", never as a pass —\n  which is the property a |Δh| tolerance does not have ( #obs-erosion-residual-is-driver-bound FE(4))."
        );
    }
}
