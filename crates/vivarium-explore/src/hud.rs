//! The **epistemic overlay** — what is on screen, said out loud.
//!
//! The design rule for everything here: *a good explorer tells you the thing you
//! would otherwise have had to ask.* Two questions cost a real session on
//! 2026-07-24 — why land emerges as circles that stop growing, and why inland
//! lakes appear and then vanish — and both had answers sitting in the law the
//! whole time. The first is that cratons are literally a union of fated
//! spherical caps with a saturating growth scalar; the second is that there is
//! no hydrology in the rendered surface at all, only a sea datum crossing
//! depressions. Neither needed research. They needed saying.
//!
//! So the HUD carries a block nothing else in this project carries: **what is
//! not modelled**. Its entries are derived from the live law and the live census
//! wherever they can be (site counts, growth scalars, interior census), and
//! stated as prose only where the fact is structural — an absence has no number.

use vivarium_world::lithosphere;
use vivarium_world::store::RootEntry;
use vivarium_world::watch::{self, Coverage};

use crate::lens::{FrameFacts, Ladder, Lens};
use crate::paint::Paint;
use crate::pull::Frame;

/// The absences a viewer is otherwise liable to read as presences.
///
/// Ordered by how badly each one has actually misled someone.
pub fn unmodelled(frame: &Frame, ladder: &Ladder, cov: &Coverage) -> Vec<String> {
    let f = &frame.facts;
    let mut out = Vec::new();

    // 1. The hydrology gap — the one that produced the vanishing-lakes question.
    out.push(
        "No lake, endorheic-basin, or drainage-integration model exists anywhere in the tree. \
         In `surface` paint the blue is the SEA DATUM -- every cell below `derived_sea_m` -- not water. \
         An inland 'lake' that appears and vanishes across deep time is the datum sweeping through a \
         depression as the mantle cools, not water going anywhere."
            .to_string(),
    );
    if !cov.watered.is_empty() && f.water_loaded == 0 {
        out.push(format!(
            "The {} built `water-tile` roots were settled under a DIFFERENT source tree, so none is readable \
             at this binary's source hash and the water paint is empty. That is staleness, not an absence of \
             water -- `vivarium build` re-settles them. (Loading them anyway would blend two datums; \
             re-running the fill here would be a view cold-running evolution the builder has not done.)",
            cov.watered.len()
        ));
    } else if !cov.watered.is_empty() {
        out.push(format!(
            "The one real hydrology -- the built `water-tile` depth field, {} tiles -- is NOT part of the \
             surface datum and no view drew it before this one. Press 3 to see it. \
             It is a fixed {}-step fill under a declared x9000 bounded-fill acceleration, and \
             `#obs-water-fill-never-settles` measured that it does not reach a steady state, so those \
             depths are a state the kernel was stopped at, not an equilibrium.",
            cov.watered.len(),
            cov.watered.values().next().map(|(_, s)| *s).unwrap_or(0),
        ));
    }

    // 2. The craton gap — the circles that stop growing.
    out.push(format!(
        "Continents are not plates. Cratons are the union of {} fated spherical caps (fBm-warped outlines), \
         grown by a single saturating scalar `1 - e^(-4|cool)` capped at x2.5 -- currently {:.3}. \
         They stop growing by construction, they never rift, collide, or subduct, and their outlines are \
         warped circles because that is what the law says they are.",
        f.craton_sites, f.craton_growth
    ));

    // 3. The interior gap — derived from the census, not asserted.
    let reports = watch::interior(&frame.roots);
    let flat: Vec<&str> = reports
        .iter()
        .filter(|r| r.axis.is_some() && r.distinct <= 1)
        .map(|r| r.nomos.as_str())
        .collect();
    if !flat.is_empty() {
        out.push(format!(
            "No world-time interior to watch for: {} -- endpoint only (a time-index is declared in the key, \
             but the store holds ONE value of it). Deep-time playback is therefore tectonics + isostasy alone. \
             Nothing a renderer can do fixes this; materializing intermediate stages is builder work \
             ( #form-time-indexed-stage-chains FE(2) ).",
            flat.join(", ")
        ));
    }

    // 3b. The fidelity trap, when it is currently biting.
    if frame.req.level < cov.level && f.eroded_tiles > 0 && f.prior_fallback_frac > 0.99 {
        out.push(format!(
            "You are viewing at L{} but the carve was built at L{}, and an eroded region answers only at its own \
             level or finer. Every cell on screen is the uncarved prior right now -- the fluvial relief is real \
             and simply not sampled here. `]` raises the level; zooming does it automatically.",
            frame.req.level, cov.level
        ));
    }

    // 4. What the erosion that IS built does and does not represent.
    if f.eroded_tiles > 0 {
        out.push(
            "The fluvial carve is present-epoch only, run to a fixed epoch count rather than a criterion; \
             `#obs-erosion-residual-is-driver-bound` measured that sustained uplift pins its residual and that \
             most tiles do no fluvial work at all. Carved relief is a stopping point, not a graded landscape."
                .to_string(),
        );
    }

    // 5. Deep time carries no carve.
    if matches!(frame.req.lens, Lens::Stage(_)) {
        out.push(
            "This lens shows the PURE isostatic tectonic surface. No fluvial tile exists at any epoch but the \
             present, so every valley you might expect from erosion is genuinely absent here rather than smoothed."
                .to_string(),
        );
    }

    // 6. Sampling honesty about the ladder itself.
    let built = ladder.built_count();
    if built < ladder.len() {
        out.push(format!(
            "{} of {} deep-time stages on screen are VIEW-COMPUTED -- real evaluations of the cooling law at \
             real mantle temperatures, written nowhere. `vivarium demand frames={}` then `vivarium build` makes \
             them store citizens (and playback laps instant).",
            ladder.len() - built,
            ladder.len(),
            ladder.len()
        ));
    }

    // 7. Relief.
    if frame.req.exag != 1.0 {
        out.push(format!(
            "Relief is exaggerated x{:.0}. At x1 this planet is a billiard ball, truthfully -- but slopes you \
             judge by eye are {:.0}x steeper than the law says.",
            frame.req.exag, frame.req.exag
        ));
    }
    out
}

/// The header block: identity, lens, and what the colour means.
pub fn header(
    world_name: &str,
    seed: u64,
    frame: &Frame,
    ladder: &Ladder,
    cov: &Coverage,
    inflight: bool,
) -> String {
    use std::fmt::Write as _;
    let f = &frame.facts;
    let r = crate::mesh::radius_km();
    let cell_km = (r * std::f32::consts::FRAC_PI_2) / (1u32 << frame.req.level) as f32;
    let mut s = String::new();

    let lens = match frame.req.lens {
        Lens::Present => "PRESENT".to_string(),
        Lens::Stage(i) => format!(
            "DEEP TIME  stage {}/{}  {:.3} Ga  T_p {:.0} C  [{}]",
            i + 1,
            ladder.len(),
            ladder.ages_ga.get(i).copied().unwrap_or(0.0),
            ladder.tps.get(i).copied().unwrap_or(0.0),
            if ladder.built.get(i).copied().unwrap_or(false) { "built" } else { "view-computed" }
        ),
        Lens::Replay(n) => format!("REPLAY  {n} roots landed  (BUILD history, not world-time)"),
    };
    let _ = writeln!(
        s,
        "vivium \"{world_name}\" {seed:#018x}  |  {lens}{}",
        if inflight { "  |  rebuilding..." } else { "" }
    );
    let _ = writeln!(
        s,
        "view L{} (~{cell_km:.0} km/cell) of an L{} build  |  relief x{:.0}  |  pull {:.2}s  |  land {:.1}%",
        frame.req.level,
        cov.level,
        frame.req.exag,
        f.pull_s,
        f.land_frac * 100.0
    );
    let _ = writeln!(s, "{}", crate::lens::surface_provenance_line(frame.req.lens, f, cov, frame.req.level));
    let _ = writeln!(s, "paint [{}]  {}", frame.req.paint.name(), frame.req.paint.legend());
    // Every line above can run long; the caller wraps the whole block so the
    // panel never widens past the half-window it is pinned to.
    s
}

/// The census block: what is built, what is provisional, what has an interior,
/// and the wall's own count.
pub fn census(frame: &Frame, cov: &Coverage) -> String {
    use std::fmt::Write as _;
    let f = &frame.facts;
    let t = cov.tally();
    let mut s = String::new();
    let _ = writeln!(
        s,
        "BUILT  {} readable tiles at L{}: watered {} | eroded {} | initial-topo {} | provisional {}{}",
        cov.built_tiles(),
        cov.level,
        t[3],
        t[2],
        t[1],
        cov.provisional.len(),
        if cov.stale_only_tiles() > 0 {
            format!(
                "  |  {} tiles have ONLY stale-source roots: built, unreadable here, drawn from the prior -- rerun `vivarium build`",
                cov.stale_only_tiles()
            )
        } else {
            String::new()
        }
    );
    let _ = writeln!(
        s,
        "       sea datum {:.0} m -- {}  |  {:.1}% of drawn cells fall back to the uncarved prior",
        f.sea_m,
        f.sea_provenance.label(),
        f.prior_fallback_frac * 100.0
    );
    let _ = writeln!(
        s,
        "       standing water: {} cells wet, {} of them INLAND  ({}/{} water tiles readable at this source hash{})  |  view writes refused: {}",
        f.water_cells,
        f.inland_water_cells,
        f.water_loaded,
        f.water_requested,
        if f.water_loaded < f.water_requested { " -- STALE, rerun vivarium build" } else { "" },
        f.refused_writes
    );
    let _ = writeln!(
        s,
        "       face-seam dh: cross {:.0}/{:.0} m mean/max | within {:.0}/{:.0} m -- healthy is cross ~= within",
        frame.seam.cross_mean(),
        frame.seam.cross_max,
        frame.seam.within_mean(),
        frame.seam.within_max
    );
    s
}

/// The deep-time timeline: one tick per stage, residency visible.
///
/// `#` = store citizen, `-` = view-computed, `[]` = selected, `*` = the
/// present-Abyssal stage that coincides with the live world. Kept to ASCII —
/// Bevy's default font has no glyphs for the block-drawing characters.
pub fn timeline(ladder: &Ladder, idx: usize, width: usize) -> String {
    let n = ladder.len();
    if n == 0 {
        return String::new();
    }
    let cols = width.min(n).max(1);
    let mut bar = String::with_capacity(cols + 2);
    for c in 0..cols {
        let i = c * n / cols;
        let sel = idx * cols / n == c;
        let ch = if i == ladder.present_idx {
            '*'
        } else if ladder.built[i] {
            '#'
        } else {
            '-'
        };
        if sel {
            bar.push('[');
            bar.push(ch);
            bar.push(']');
        } else {
            bar.push(ch);
        }
    }
    format!(
        "{:.2} Ga {bar} {:.2} Ga   {}/{} stages are store citizens ({} demanded)",
        ladder.ages_ga.first().copied().unwrap_or(0.0),
        ladder.ages_ga.last().copied().unwrap_or(0.0),
        ladder.built_count(),
        n,
        ladder.demanded_stages
    )
}

/// The honesty block from the shared reader — **the same text** `vivarium watch`
/// prints under its ASCII globe, byte for byte. One instrument at two fidelities
/// of attention, not two instruments that happen to agree today.
pub fn honesty(roots: &[RootEntry]) -> String {
    watch::honesty_block(roots)
}

/// Craton facts as a one-liner for the deep-time header.
pub fn craton_line(f: &FrameFacts) -> String {
    format!(
        "cratons: {} fated caps, growth {:.3} of the present anchor (saturating, capped x{:.1})",
        f.craton_sites,
        f.craton_growth,
        lithosphere::CRATON_GROWTH_MAX
    )
}

/// The key map, kept last so it never pushes information off the top.
pub fn keys(_paint: Paint) -> String {
    "drag spin | wheel zoom | [ ] level | A auto-level | X relief | O pole | R reset\n\
     TAB paint (surface/provenance/water/seam) | T deep time | K play/pause | J/L step stage | P present\n\
     V replay build history | , . hour | N M day | Y headlight | C CAPTURE SIGHTING | Esc quit"
        .to_string()
}
