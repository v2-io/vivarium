//! **Sightings** — capturing the moment a pattern-matcher fires.
//!
//! Joseph's stated reason for wanting this instrument: *"my brain is very highly
//! tuned to be able to notice if there's something in the visual evolution that
//! does not seem 'natural' … it was one of the fastest ways to reveal missing
//! physics — when something is acting contrary to what my mind has hundreds of
//! thousands of hours of remembered patterns of."*
//!
//! That is a second epistemic channel, not a softer version of the first. A
//! probe convicts one declared thing somebody already thought to ask about
//! ( #norm-probes-before-claims ); a trained visual system scans the whole field
//! for violations of priors nobody has declared yet. The channels are
//! complementary, and this project's own record says the second one is fast.
//!
//! But it has a defect the first does not: it fires **in the moment**, at a
//! particular place and world-time, and by the time it becomes prose the
//! particulars are gone. *"The lakes looked wrong"* is not something a probe can
//! be written against. *"At stage 34 of 65 (T_p 1487 °C, 1.81 Ga), face 3 cell
//! (412, 88), 41.2°N 17.8°W, surface −188 m, sea datum −166 m, build-state
//! watered, standing water 0.0 m, and the only hydrology in the tree is a
//! never-settling 200-step fill"* is.
//!
//! So a sighting is a **dump of everything that was true when the key was
//! pressed**, beside a screenshot, in a file with a blank `## What looked wrong`
//! heading for the observer to fill in. It is an *observation*, deliberately not
//! a claim: it asserts nothing about the world, only about what was on screen
//! and what the store said at that instant. A segment or a probe is what a
//! sighting eventually turns into, and the honest ordering is that the sighting
//! comes first.
//!
//! **A sighting is never a store citizen.** It goes to `sightings/` beside the
//! world, not into `objects/` or `roots/` — an observation about a world is not
//! a part of that world, and writing one through the store would be the view
//! authoring after all ( #form-core-view-wall ).

use std::path::{Path, PathBuf};

use vivarium_world::nomotheke;
use vivarium_world::watch;

use crate::lens::Lens;
use crate::pull::Frame;

/// Everything the ECS knows that the worker does not — camera, cursor, and the
/// ladder position — gathered at keypress.
pub struct Vantage {
    pub world_name: String,
    pub seed: u64,
    pub world_dir: PathBuf,
    /// Screen-centre surface point.
    pub centre_lat_deg: f64,
    pub centre_lon_deg: f64,
    pub altitude_km: f32,
    /// The cursor pick, when the cursor is on the planet.
    pub pick: Option<Pick>,
    /// Deep-time position, when a stage lens is up.
    pub stage: Option<StagePosition>,
    /// Solar ephemeris, so a lighting artefact is distinguishable from a
    /// terrain one after the fact.
    pub sun_day: f32,
    pub sun_hour: f32,
    pub headlight: bool,
}

pub struct Pick {
    pub face: u8,
    pub level: u8,
    pub i: u32,
    pub j: u32,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub elev_m: f32,
    pub water_m: f32,
    pub state: watch::BuildState,
    pub provisional: bool,
}

pub struct StagePosition {
    pub idx: usize,
    pub total: usize,
    pub age_ga: f32,
    pub tp_c: f64,
    pub built: bool,
    pub playing: bool,
}

/// Write a sighting. Returns the markdown path (the screenshot lands beside it,
/// same stem, `.png`), or an error string fit to show on the HUD.
pub fn write(
    frame: &Frame,
    v: &Vantage,
    unmodelled: &[String],
    depiction: &[String],
) -> Result<PathBuf, String> {
    let dir = sightings_dir(&v.world_dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let stamp = stamp();
    let path = dir.join(format!("sighting-{stamp}.md"));
    std::fs::write(&path, body(frame, v, unmodelled, depiction, &stamp))
        .map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

/// Where sightings land: `$VIVARIUM_SIGHTINGS`, else `<world-dir>/sightings/`.
/// Beside the world rather than inside the store, and overridable so a session
/// can collect them straight into a repo working directory.
pub fn sightings_dir(world_dir: &Path) -> PathBuf {
    match std::env::var("VIVARIUM_SIGHTINGS") {
        Ok(p) if !p.is_empty() => PathBuf::from(p),
        _ => world_dir.join("sightings"),
    }
}

fn stamp() -> String {
    let s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{s}")
}

fn body(
    frame: &Frame,
    v: &Vantage,
    unmodelled: &[String],
    depiction: &[String],
    stamp: &str,
) -> String {
    use std::fmt::Write as _;
    let f = &frame.facts;
    let mut s = String::new();

    let _ = writeln!(s, "# Sighting {stamp}");
    let _ = writeln!(s);
    let _ = writeln!(
        s,
        "An **observation**, not a claim: what was on screen and what the store said, at the \
         moment something looked wrong. Nothing here asserts anything about the world — that is \
         what a probe or a segment is for, and this is the raw material one gets written against."
    );
    let _ = writeln!(s);
    let _ = writeln!(s, "## What looked wrong");
    let _ = writeln!(s);
    let _ = writeln!(s, "<!-- in your own words: what did your eye object to, and what did you expect instead? -->");
    let _ = writeln!(s);
    let _ = writeln!(s, "## Vantage");
    let _ = writeln!(s);
    let _ = writeln!(s, "- vivium `{}` seed `{:#018x}`", v.world_name, v.seed);
    let _ = writeln!(s, "- world dir `{}`", v.world_dir.display());
    let _ = writeln!(s, "- source hash `{}` (every complete key folds it)", nomotheke::SRC_HASH);
    let _ = writeln!(
        s,
        "- lens **{}**",
        match frame.req.lens {
            Lens::Present => "present (live world)".to_string(),
            Lens::Stage(i) => format!("deep-time stage {i}"),
            Lens::Replay(n) => format!("replay after {n} root landings (BUILD history, not world-time)"),
        }
    );
    if let Some(st) = &v.stage {
        let _ = writeln!(
            s,
            "- stage {}/{} · {:.3} Ga · T_p {:.1} °C · **{}** · {}",
            st.idx + 1,
            st.total,
            st.age_ga,
            st.tp_c,
            if st.built { "store citizen" } else { "VIEW-COMPUTED (not materialized)" },
            if st.playing { "playing" } else { "paused" }
        );
    }
    let _ = writeln!(
        s,
        "- camera: centre {:.3}° {:.3}° · altitude {:.0} km · render L{} · relief ×{:.0}",
        v.centre_lat_deg, v.centre_lon_deg, v.altitude_km, frame.req.level, frame.req.exag
    );
    let _ = writeln!(s, "- paint mode `{}`", frame.req.paint.name());
    let _ = writeln!(
        s,
        "- sun: day {:.0}/365 hour {:04.1}{}",
        v.sun_day,
        v.sun_hour,
        if v.headlight { " (HEADLIGHT — not the real ephemeris; a lighting artefact here is the view's, not the world's)" } else { "" }
    );
    let _ = writeln!(s);

    let _ = writeln!(s, "## The cell under the cursor");
    let _ = writeln!(s);
    match &v.pick {
        Some(p) => {
            let _ = writeln!(s, "- `face={} level={} i={} j={}`", p.face, p.level, p.i, p.j);
            let _ = writeln!(s, "- {:.4}° {:.4}°", p.lat_deg, p.lon_deg);
            let _ = writeln!(
                s,
                "- surface {:.1} m · sea datum {:.1} m · relative {:+.1} m ({})",
                p.elev_m,
                f.sea_m,
                p.elev_m - f.sea_m,
                if p.elev_m > f.sea_m { "above the datum" } else { "below the datum" }
            );
            let _ = writeln!(
                s,
                "- standing water {:.2} m (from the built `water-tile` field — the only hydrology in the tree)",
                p.water_m
            );
            let _ = writeln!(
                s,
                "- build-state **{}**{}",
                p.state.label(),
                if p.provisional { " · ⚠ PROVISIONAL (waived flux admission — not lawful evidence)" } else { "" }
            );
        }
        None => {
            let _ = writeln!(s, "(cursor was not on the planet — the vantage above is the whole record of place)");
        }
    }
    let _ = writeln!(s);

    let _ = writeln!(s, "## What the frame was drawn from");
    let _ = writeln!(s);
    let _ = writeln!(
        s,
        "- sea datum {:.1} m — {}",
        f.sea_m,
        f.sea_provenance.label()
    );
    let _ = writeln!(s, "- erosion-tile roots in this surface: {}", f.eroded_tiles);
    if f.stale_tiles > 0 {
        let _ = writeln!(
            s,
            "- ⚠ {} erosion roots carved under a DIFFERENT source tree, excluded from the surface \
             (re-run `vivarium build`)",
            f.stale_tiles
        );
    }
    let _ = writeln!(s, "- cells falling back to the uncarved prior: {:.1}%", f.prior_fallback_frac * 100.0);
    let _ = writeln!(s, "- land fraction (above the datum): {:.2}%", f.land_frac * 100.0);
    let _ = writeln!(
        s,
        "- standing water: {} cells wet, of which **{} inland** (above the datum)",
        f.water_cells, f.inland_water_cells
    );
    let _ = writeln!(
        s,
        "- cratons: {} fated sites, growth scalar {:.3} at T_p {:.1} °C (saturating `1 − e^(−4·cool)`, capped at 2.5)",
        f.craton_sites, f.craton_growth, f.tp_c
    );
    let _ = writeln!(
        s,
        "- face-seam Δh: cross {:.0} m mean / {:.0} m max · within-face {:.0} m mean / {:.0} m max",
        frame.seam.cross_mean(),
        frame.seam.cross_max,
        frame.seam.within_mean(),
        frame.seam.within_max
    );
    let _ = writeln!(s, "- view writes refused by the read-only store handle: {}", f.refused_writes);
    let _ = writeln!(s, "- pull time {:.2} s", f.pull_s);
    let _ = writeln!(s);

    let _ = writeln!(s, "## Store census at that instant");
    let _ = writeln!(s);
    let _ = writeln!(s, "```");
    let _ = write!(s, "{}", watch::honesty_block(&frame.roots));
    let _ = writeln!(s, "```");
    let _ = writeln!(s);
    let _ = writeln!(
        s,
        "The `interior` column is how much world-time a nomos has *between* its endpoints \
         ( #form-time-indexed-stage-chains FE(2) ). Where it reads `endpoint only`, there is \
         nothing between unstarted and finished for any renderer to show, and no view can \
         conjure one — materializing intermediate stages is builder work."
    );
    let _ = writeln!(s);

    let _ = writeln!(s, "## Not modelled at all (so the eye is not chasing an absence)");
    let _ = writeln!(s);
    for line in unmodelled {
        let _ = writeln!(s, "- {line}");
    }
    let _ = writeln!(s);

    let _ = writeln!(s, "## On screen without a world referent");
    let _ = writeln!(s);
    let _ = writeln!(
        s,
        "Every declared unreal affordance in the frame ( #norm-no-depiction-without-referent ). \
         Read this before trusting a shape: if what your eye objected to is on this list, the \
         objection is to the view, not to the world."
    );
    let _ = writeln!(s);
    for line in depiction {
        let _ = writeln!(s, "- {line}");
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "---");
    let _ = writeln!(s, "*Written by `vivarium explore`. Screenshot: `sighting-{stamp}.png`.*");
    s
}
