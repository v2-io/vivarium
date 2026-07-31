//! **Session captures** — durable dump + screenshot pairs for handoff
//! (`#disc-explorer-debug-capture`).
//!
//! Evolves the sighting idea: same epistemic role (observation, not claim),
//! structured **udon** for agents, conventional `captures/` directory, paired
//! PNG. Sighting markdown remains available under `sightings/` for the blank
//! "what looked wrong" workflow.

use std::path::{Path, PathBuf};

use vivarium_world::nomotheke;
use vivarium_world::query::RegionCensus;
use vivarium_world::spec::WorldSpec;
use vivarium_world::watch;

use crate::lens::Lens;
use crate::pull::Frame;
use crate::sighting::{self, Vantage};

/// Schema id written into every capture file.
pub const INFO_SCHEMA: &str = "vivarium-info.v0.1.0";

/// Where captures land: `$VIVARIUM_CAPTURES`, else `<world-dir>/captures/`.
pub fn captures_dir(world_dir: &Path) -> PathBuf {
    match std::env::var("VIVARIUM_CAPTURES") {
        Ok(p) if !p.is_empty() => PathBuf::from(p),
        _ => world_dir.join("captures"),
    }
}

fn stamp() -> String {
    let s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Local wall-ish stamp for humans; unix seconds are unique enough for one machine.
    let secs = s;
    let days = secs / 86400;
    let day_s = secs % 86400;
    let h = day_s / 3600;
    let m = (day_s % 3600) / 60;
    let sec = day_s % 60;
    // Approximate civil date is not required for uniqueness; keep sortable unix + clock.
    format!("{days:05}-{h:02}{m:02}{sec:02}-u{secs}")
}

/// Write capture pair + optional classic sighting md.
/// Returns `(info_udon_path, png_path)` for the pending screenshot.
///
/// `png_override`: when `Some`, the PNG lands there (and the udon records that
/// path). When `None`, both files use a stamp under [`captures_dir`].
pub fn write(
    frame: &Frame,
    v: &Vantage,
    census: RegionCensus,
    demand: Option<&WorldSpec>,
    unmodelled: &[String],
    depiction: &[String],
) -> Result<(PathBuf, PathBuf), String> {
    write_with_png(frame, v, census, demand, unmodelled, depiction, None)
}

/// Like [`write`], but place the PNG at `png_override` when provided.
pub fn write_with_png(
    frame: &Frame,
    v: &Vantage,
    census: RegionCensus,
    demand: Option<&WorldSpec>,
    unmodelled: &[String],
    depiction: &[String],
    png_override: Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), String> {
    let dir = captures_dir(&v.world_dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let stamp = stamp();
    let stem = format!("{stamp}");
    let info_path = dir.join(format!("{stem}-{INFO_SCHEMA}.udon"));
    let png_path = match png_override {
        Some(p) => {
            if let Some(parent) = p.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("{}: {e}", parent.display()))?;
                }
            }
            p
        }
        None => dir.join(format!("{stem}.png")),
    };
    let body = udon_body(frame, v, census, demand, unmodelled, depiction, &png_path, &stamp);
    std::fs::write(&info_path, body).map_err(|e| format!("{}: {e}", info_path.display()))?;

    // Keep classic sighting for free-form "what looked wrong" notes.
    let _ = sighting::write(frame, v, unmodelled, depiction);

    Ok((info_path, png_path))
}

fn world_git_short(world_dir: &Path) -> String {
    std::process::Command::new("git")
        .args(["-C", &world_dir.to_string_lossy(), "rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "(no world-git)".into())
}

fn udon_body(
    frame: &Frame,
    v: &Vantage,
    census: RegionCensus,
    demand: Option<&WorldSpec>,
    unmodelled: &[String],
    depiction: &[String],
    png_path: &Path,
    stamp: &str,
) -> String {
    use std::fmt::Write as _;
    let f = &frame.facts;
    let mut s = String::new();
    let _ = writeln!(s, "# {INFO_SCHEMA}");
    let _ = writeln!(s, "# Session capture — observation, not claim. Sibling screenshot: {}", png_path.display());
    let _ = writeln!(s, "# Stamp {stamp}");
    let _ = writeln!(s);
    let _ = writeln!(s, "schema: {INFO_SCHEMA}");
    let _ = writeln!(s, "vivium: {:?}", v.world_name);
    let _ = writeln!(s, "seed: {:#018x}", v.seed);
    let _ = writeln!(s, "world_dir: {:?}", v.world_dir.display().to_string());
    let _ = writeln!(s, "world_git: {:?}", world_git_short(&v.world_dir));
    let _ = writeln!(s, "src: {:?}", nomotheke::SRC_HASH);
    let _ = writeln!(s, "src_short: {:?}", &nomotheke::SRC_HASH[..8.min(nomotheke::SRC_HASH.len())]);
    let _ = writeln!(s, "erosion_fresh: {}", census.fresh);
    let _ = writeln!(s, "erosion_stale: {}", census.stale);
    let _ = writeln!(s, "erosion_total: {}", census.total);
    let next = if census.fresh == 0 && census.stale > 0 {
        "vivarium build   # eroded land not readable under this binary"
    } else if census.fresh == 0 {
        "vivarium build   # no erosion-tile roots yet"
    } else {
        "ok — zoom past L7 for covering-grain fluvial if needed"
    };
    let _ = writeln!(s, "next_action: {next:?}");
    let _ = writeln!(s, "png: {:?}", png_path.display().to_string());
    let _ = writeln!(s);

    if let Some(spec) = demand {
        let d = &spec.demand;
        let _ = writeln!(s, "demand:");
        let _ = writeln!(s, "  order: {:?}", d.order);
        let _ = writeln!(s, "  target_phase: {}", d.target_phase);
        let _ = writeln!(s, "  level: {}", d.level);
        let _ = writeln!(s, "  frames: {}", d.frames);
        let _ = writeln!(s, "  erosion_epochs: {}", d.erosion_epochs);
        let _ = writeln!(s, "  erosion_stage_stride: {}", d.erosion_stage_stride);
        let _ = writeln!(s, "  water_steps: {}", d.water_steps);
        if let Some(b) = &d.beacon {
            let _ = writeln!(
                s,
                "  beacon: \"face={} level={} oi={} oj={} tiles={} epochs={} stride={}\"",
                b.face, b.level, b.oi, b.oj, b.tiles, b.epochs, b.stride
            );
        } else {
            let _ = writeln!(s, "  beacon: none");
        }
        let _ = writeln!(s);
    }

    let _ = writeln!(s, "view:");
    let _ = writeln!(
        s,
        "  lens: {:?}",
        match frame.req.lens {
            Lens::Present => "present".into(),
            Lens::Stage(i) => format!("stage-{i}"),
            Lens::Erosion(i) => format!("erosion-{i}"),
            Lens::Replay(n) => format!("replay-{n}"),
        }
    );
    let _ = writeln!(s, "  paint: {:?}", frame.req.paint.name());
    let _ = writeln!(s, "  mesh_level: {}", frame.req.level);
    let _ = writeln!(
        s,
        "  window: {}",
        if frame.req.window.is_some() {
            "close-in"
        } else {
            "whole-globe"
        }
    );
    let _ = writeln!(s, "  relief_x: {}", frame.req.exag);
    let _ = writeln!(
        s,
        "  centre_lat_lon: [{:.5}, {:.5}]",
        v.centre_lat_deg, v.centre_lon_deg
    );
    let _ = writeln!(s, "  altitude_km: {}", v.altitude_km);
    let _ = writeln!(s);

    let _ = writeln!(s, "frame:");
    let _ = writeln!(s, "  sea_m: {}", f.sea_m);
    let _ = writeln!(s, "  sea_provenance: {:?}", f.sea_provenance.label());
    let _ = writeln!(s, "  eroded_tiles: {}", f.eroded_tiles);
    let _ = writeln!(s, "  stale_tiles: {}", f.stale_tiles);
    let _ = writeln!(s, "  prior_fallback_frac: {}", f.prior_fallback_frac);
    let _ = writeln!(s, "  land_frac: {}", f.land_frac);
    let _ = writeln!(s, "  water_cells: {}", f.water_cells);
    let _ = writeln!(s, "  inland_water_cells: {}", f.inland_water_cells);
    let _ = writeln!(s, "  pull_s: {}", f.pull_s);
    let _ = writeln!(s, "  refused_writes: {}", f.refused_writes);
    let _ = writeln!(s, "  tier_cells:");
    for (t, n) in &f.tier_cells {
        let _ = writeln!(s, "    L{t}: {n}");
    }
    let _ = writeln!(s);

    if let Some(p) = &v.pick {
        let _ = writeln!(s, "pick:");
        let _ = writeln!(s, "  face: {}", p.face);
        let _ = writeln!(s, "  level: {}", p.level);
        let _ = writeln!(s, "  i: {}", p.i);
        let _ = writeln!(s, "  j: {}", p.j);
        let _ = writeln!(s, "  lat_deg: {}", p.lat_deg);
        let _ = writeln!(s, "  lon_deg: {}", p.lon_deg);
        let _ = writeln!(s, "  elev_m: {}", p.elev_m);
        let _ = writeln!(s, "  water_m: {}", p.water_m);
        let _ = writeln!(s, "  build_state: {:?}", p.state.label());
        let _ = writeln!(s, "  provisional: {}", p.provisional);
        let _ = writeln!(s);
    } else {
        let _ = writeln!(s, "pick: null");
        let _ = writeln!(s);
    }

    let _ = writeln!(s, "store_census: |");
    for line in watch::honesty_block(&frame.roots).lines() {
        let _ = writeln!(s, "  {line}");
    }
    let _ = writeln!(s);

    let _ = writeln!(s, "unmodelled:");
    for line in unmodelled {
        let _ = writeln!(s, "  - {:?}", line);
    }
    let _ = writeln!(s, "depiction_without_referent:");
    for line in depiction {
        let _ = writeln!(s, "  - {:?}", line);
    }
    s
}

/// Human-chrome block for "can this program show eroded land?"
///
/// Plain language only — no "CARVE" (not in LEXICON, grabs no attention).
/// ASCII-only so Bevy text does not tofu middle-dots / stars.
/// When a rebuild is needed the first line is the scream; the rest is why.
pub fn bed_status_block(census: RegionCensus) -> String {
    let src = &nomotheke::SRC_HASH[..8.min(nomotheke::SRC_HASH.len())];
    if census.fresh == 0 && census.stale > 0 {
        format!(
            "*** REBUILD NEEDED ***\n\
             eroded land is in the store but not readable under this program\n\
             readable now: 0    older builds: {}    src {}\n\
             next: vivarium build",
            census.stale, src
        )
    } else if census.fresh == 0 {
        format!(
            "eroded land: none yet (src {src})\n\
             next: vivarium build"
        )
    } else {
        format!(
            "eroded land: {} readable under this program (src {})\n\
             older builds ignored: {}",
            census.fresh, src, census.stale
        )
    }
}

/// Fixed-width label + value for chrome table rows (8-char label column).
pub fn row(label: &str, value: &str) -> String {
    format!("{label:<8}{value}")
}

/// Two-column keybinding legend for the explore HUD (`?` toggle, off by default).
/// Aligned so the eye can scan the action column.
pub fn key_legend() -> String {
    let rows: &[(&str, &str)] = &[
        ("drag", "orbit"),
        ("wheel", "zoom (close-in = region window)"),
        ("[ ]", "level - / +"),
        ("A", "auto-level"),
        ("X", "relief exaggeration"),
        ("O / R", "pole / reset view"),
        ("B / G", "go to selected chain / cycle chains"),
        ("1-6 TAB", "paint (surf / prov / water / seam / chg / dep)"),
        ("Z", "change-paint scale"),
        ("P", "present surface"),
        ("E", "erosion settle history (world-time)"),
        ("T", "deep time (mantle cooling)"),
        ("V", "replay (build history)"),
        ("K J/L", "play-pause / step stage"),
        (", . N M", "hour -+ / day -+"),
        ("Y", "headlight on-off"),
        ("H", "overlay: human / debug / minimal"),
        ("C", "capture (udon + png in captures/)"),
        ("?", "this key legend"),
        ("Esc", "quit"),
    ];
    let mut s = String::from("\n");
    s.push_str(&row("KEYS", "(? again to hide)\n"));
    for (k, v) in rows {
        // key column 10 chars, then action — vertical alignment across rows
        let _ = std::fmt::Write::write_fmt(&mut s, format_args!("  {k:<10}  {v}\n"));
    }
    s
}
