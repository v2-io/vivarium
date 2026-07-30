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

    // 4a. The chain exists but is not being watched, or does not exist at all.
    // Stated in every lens, because "there is a settle history in this store"
    // is exactly the fact a viewer cannot see from a picture of the present.
    match frame.chain.cohort.as_ref() {
        Some(c) if !matches!(frame.req.lens, Lens::Erosion(_)) => out.push(format!(
            "This store HOLDS a {}-stage erosion settle history (epochs {:?}, {} tiles at L{}) and you are not \
             looking at it. Press E. The present surface is its last stage only.",
            c.len(),
            c.epochs,
            c.tiles.first().copied().unwrap_or(0),
            c.level
        )),
        None => out.push(format!(
            "No erosion settle history exists in this store: {} carve cohort(s) present, none with more than one \
             materialized time-index. There is no world-time erosion playback to offer, and no renderer can \
             manufacture one -- materializing intermediate stages is builder work \
             ( #form-time-indexed-stage-chains FE(8) ).",
            frame.chain.all.len()
        )),
        _ => {}
    }
    // Cohorts passed over: a settle history under a source tree that is not the
    // one being scrubbed is a *different world's* history, and silently dropping
    // it is the same silence the stale-src census exists to break.
    let passed: Vec<String> = frame
        .chain
        .all
        .iter()
        .filter(|c| Some(&c.src) != frame.chain.cohort.as_ref().map(|s| &s.src))
        .map(|c| format!("{} ({} stage{})", &c.src[..8.min(c.src.len())], c.len(), if c.len() == 1 { "" } else { "s" }))
        .collect();
    if !passed.is_empty() {
        out.push(format!(
            "{} other erosion cohort(s) in this store are NOT on screen and cannot be: {}. Each was carved under a \
             different source tree, so its stages belong to a different world -- putting them on one time axis \
             would draw the difference between two kernels as though it were the passage of world-time.",
            passed.len(),
            passed.join(", ")
        ));
    }

    // 4b. What the settle history is and is not, when it is the subject.
    if let Lens::Erosion(_) = frame.req.lens {
        out.push(format!(
            "The settle history has {} materialized stages and the view cannot make more of them. Erosion is a \
             MATERIALIZED-ONLY chain: a stage exists because the kernel integrated to it, so density is what the \
             builder ran and asking for more is `vivarium build`, never a view flag \
             ( #form-time-indexed-stage-chains FE(8) ). Nothing is interpolated between the ticks; the jumps are real.",
            frame.chain.len()
        ));
        out.push(
            "Interior stages are ADDRESSABLE, not certified-accurate. The scheme's authors bound their accuracy \
             claim to the steady-state endpoint and except the transient at large timestep (drainage area lags one \
             step; knickpoint propagation has a Courant condition), and an interior stage is exactly a transient \
             state. So this history is citable and watchable, and its intermediate accuracy at the current epoch \
             size is UNDECLARED."
                .to_string(),
        );
        out.push(
            "Deep time does not move here. The whole settle history was run at the PRESENT mantle temperature, so \
             scrubbing it changes the surface and not T_p -- there is no thermal-fluvial coupling in this tree to \
             depict, and a shared slider would have implied one."
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
        Lens::Erosion(i) => {
            let c = frame.chain.cohort.as_ref();
            format!(
                "EROSION SETTLE  stage {}/{}  epoch {}{}  (WORLD-time, every tile at the same moment){}",
                i + 1,
                frame.chain.len(),
                frame.facts.stage_epoch.map(|e| e.to_string()).unwrap_or_else(|| "?".into()),
                c.map(|c| format!(" of {}", c.epochs.last().copied().unwrap_or(0))).unwrap_or_default(),
                match c {
                    Some(c) if !c.is_current => format!(
                        "  |  !! carved under source {} -- NOT this binary's ({}). \
                         A PREVIOUS world's settle history, shown faithfully",
                        &c.src[..8.min(c.src.len())],
                        &vivarium_world::nomotheke::SRC_HASH[..8]
                    ),
                    _ => String::new(),
                }
            )
        }
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
        "view L{} (~{}) {}  |  relief x{:.0}  |  pull {:.2}s  |  land {:.1}%",
        frame.req.level,
        if cell_km >= 1.0 { format!("{cell_km:.1} km/cell") } else { format!("{:.0} m/cell", cell_km * 1000.0) },
        match frame.req.patch {
            Some(p) => format!(
                "REGION WINDOW {}x{} cells on face {} at ({}, {}) -- one window into one face, not the globe",
                p.nx, p.nx, p.face, p.oi, p.oj
            ),
            None => format!("whole globe, over an L{} build", cov.level),
        },
        frame.req.exag,
        f.pull_s,
        f.land_frac * 100.0
    );
    let _ = writeln!(s, "{}", tier_line(frame));
    let _ = writeln!(
        s,
        "{}",
        crate::lens::surface_provenance_line(frame.req.lens, f, cov, frame.req.level, &frame.chain)
    );
    let _ = writeln!(s, "paint [{}]  {}", frame.req.paint.name(), frame.req.paint.legend());
    if frame.req.paint == crate::paint::Paint::Change {
        let _ = writeln!(s, "{}", change_line(frame));
    }
    if frame.req.paint == crate::paint::Paint::Depression {
        let _ = writeln!(s, "{}", depression_line(frame));
    }
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
        "       standing water: {} cells wet, {} of them INLAND  ({}/{} water tiles readable at this source hash at L{}, the census level{})  |  view writes refused: {}",
        f.water_cells,
        f.inland_water_cells,
        f.water_loaded,
        f.water_requested,
        cov.level,
        if f.water_loaded < f.water_requested { " -- STALE, rerun vivarium build" } else { "" },
        f.refused_writes
    );
    if frame.seam.n == 0 {
        let _ = writeln!(s, "       face-seam dh: no chart-seam edges measured this frame");
    } else {
        let _ = writeln!(
            s,
            "       face-seam dh: cross {:.0}/{:.0} m mean/max | within {:.0}/{:.0} m over {} edges -- healthy is cross ~= within",
            frame.seam.cross_mean(),
            frame.seam.cross_max,
            frame.seam.within_mean(),
            frame.seam.within_max,
            frame.seam.n
        );
    }
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

/// **Which fidelity tier drew each cell** — the one line a fine view cannot do
/// without.
///
/// The trap it closes is the exact mirror of #obs-coarse-view-draws-the-uncarved-prior,
/// and it is worse because it is invisible rather than merely silent. A coarse
/// view over a fine build draws the *uncarved prior* and looks unremarkable. A
/// fine view over a coarse build draws something that looks *better*: an
/// `ErodedRegion` answers any cell at its own level or finer, and its answer at
/// a finer level is a bilinear read of the coarse carve **plus the fine prior's
/// detail re-added** ( #form-fidelity-ladder ). So a view at L13 over an L9 carve
/// is full of kilometre-scale relief, correctly derived, that no fluvial kernel
/// ever computed — valleys with no drainage behind them. An eye trained on real
/// landscapes will read them as fluvial, because that is what they look like.
///
/// Nothing about the picture says so. This line does.
pub fn tier_line(frame: &Frame) -> String {
    let f = &frame.facts;
    let view = frame.req.level;
    if f.tier_cells.is_empty() {
        return format!(
            "carve tiers: NONE -- every cell on screen is the uncarved prior at L{view} \
             ({:.0}% of drawn cells)",
            f.prior_fallback_frac * 100.0
        );
    }
    let pct = |n: usize| n as f32 * 100.0 / f.cells.max(1) as f32;
    let parts: Vec<String> = f
        .tier_cells
        .iter()
        .map(|(&t, &n)| {
            let how = match t.cmp(&view) {
                std::cmp::Ordering::Equal => "at view level",
                std::cmp::Ordering::Less => "COARSER than the view",
                std::cmp::Ordering::Greater => "finer than the view",
            };
            format!("L{t} {:.0}% ({how})", pct(n))
        })
        .collect();
    let coarse: usize = f.tier_cells.iter().filter(|(&t, _)| t < view).map(|(_, &n)| n).sum();
    let warning = if pct(coarse) > 1.0 {
        format!(
            "  <<< {:.0}% of what you see is a COARSER carve sampled finely: bilinear over the carve, \
             with the fine prior's detail re-added ( #form-fidelity-ladder ). The small-scale relief there \
             is REAL law and was NOT produced by a fluvial run at this scale -- valleys with no drainage \
             behind them. `vivarium build` a beacon at this level is what changes that",
            pct(coarse)
        )
    } else {
        String::new()
    };
    format!(
        "carve tiers drawing this frame: {} | uncarved prior {:.0}%{warning}",
        parts.join(" · "),
        f.prior_fallback_frac * 100.0
    )
}

/// The measured change field, in numbers, beside the colour that shows it.
///
/// The fractions are the point. 88% rising and 5.6% falling is not a caption on
/// the picture — it is the finding, and it is what stops the colour from being
/// read as "erosion everywhere" when most of what moved was the uplift driver.
pub fn change_line(frame: &Frame) -> String {
    let f = &frame.facts;
    format!(
        "change vs the uncarved prior: mean {:+.2} m | range {:.0} .. {:+.0} m | {:.1}% of cells RISEN, \
         {:.1}% FALLEN (>0.5 m) | ramp full-scale +-{:.0} m (Z cycles). Rising is the kernel's uplift driver, \
         falling is where fluvial incision outran it -- the store does not separate them, the SIGN does",
        f.change_mean,
        f.change_min,
        f.change_max,
        f.frac_rising * 100.0,
        f.frac_falling * 100.0,
        frame.req.change_scale_m,
    )
}

/// What the bed could hold, beside what it actually holds — and the gap between
/// them stated as the gap, because a viewer looking at a violet basin will
/// otherwise read it as a lake.
///
/// Both numbers come from the frame's own measurement rather than from any
/// claim: capacity from `Fluvial::drainage_surface` over the drawn surface,
/// standing water from the built `water-tile` field. When the second is zero and
/// the first is not, that is the world's present state and this line says which
/// two things stand in the way, because "no water" invites exactly the wrong
/// diagnosis ( #obs-lakes-are-routed-over-not-carved-away FE(11),
/// #obs-water-fill-never-settles ).
pub fn depression_line(frame: &Frame) -> String {
    let f = &frame.facts;
    let pct = 100.0 * f.depression_cells as f32 / f.cells.max(1) as f32;
    let cap_km3 = f.depression_capacity_m3 / 1e9;
    // Three different reasons the marched field can read zero, and only one of
    // them is about the world. Saying "nothing stands here" when no water tile was
    // even readable asserts a physical cause for a census artifact — and the
    // census pins its level to topography and erosion keys only, so a build whose
    // finest rung is erosion-only (the beacon, by decision) reports 0/0 here.
    // **Zero has several causes and only one of them is about basins.** The field
    // is read at the level that RAN ( #form-fidelity-ladder FE(8) ), so a view no
    // carve covers has nothing to report — and saying "no standing water" there
    // would state a result where the honest statement is that no kernel has run.
    // The prior's own closed basins are real relief and an *initial condition*;
    // they are not an answer.
    let held = if f.depression_cells == 0 && f.prior_fallback_frac > 0.99 {
        "  NO CARVE COVERS THIS VIEW, so there is nothing here to read: the zero is the reader          declining, not a world without basins. An eroded region answers only at its own level or          finer, and every drawn cell here fell back to the uncarved prior — whose relief does contain          closed basins, as an initial condition no kernel has processed. Zoom to a built region, or          `vivarium build` one at this level."
            .to_string()
    } else if f.prior_fallback_frac > 0.01 {
        format!(
            "  Read only where a carve answers: {:.1}% of drawn cells fell back to the uncarved prior          and are reported as dry regardless of the prior's own relief, because that relief is an          initial condition rather than a result.",
            f.prior_fallback_frac * 100.0
        )
    } else if f.water_requested == 0 {
        "  No marched water field was requested at the census level, so the two pictures cannot be          compared in this frame -- that is a census fact, not a statement about these basins"
            .to_string()
    } else if f.water_loaded == 0 {
        "  No marched water field is READABLE at this source hash, so nothing here is a comparison          against one -- rerun `vivarium build`"
            .to_string()
    } else if f.inland_water_cells == 0 {
        "  The marched water field holds NO inland water anywhere it is readable, which is about that          kernel and not about these basins: it settles 40 s of world time at any level          ( #obs-water-fill-never-settles ). The equilibrium above needs no settle; the transient          never reaches it"
            .to_string()
    } else {
        format!(
            "  Against {} cells the marched water field actually holds -- the two are different claims          about the same basins, and the gap is the transient's",
            f.inland_water_cells
        )
    };
    format!(
        "standing water at the spill point (the WET LIMIT): {} cells ({:.2}% of drawn), deepest {:.0} m,          {:.3e} km^3. Where water stands if every closed basin is full to its sill -- level to the bit          across each body, and zero on ground that drains. It assumes net supply is positive: no          evaporation, inflow, seepage or residence time is in the account, so an endorheic basin under a          dry climate stands lower and nothing here says by how much.{}          Read under a NO-FLUX WALL at each drawn unit's rim (a sink contract there would drain them and          report ~0); on a multi-tile surface inherited basins and tile-seam pits are both in this number          and nothing here separates them",
        f.depression_cells,
        pct,
        f.depression_deepest_m,
        cap_km3,
        held,
    )
}

/// The settle-history timeline: one tick per **materialized** stage.
///
/// Unlike [`timeline`] there is no citizen/view-computed distinction to draw,
/// and that absence is the honest content: every tick here is a store citizen
/// because a stage that was not built does not exist
/// ( #form-time-indexed-stage-chains FE(8) ). The bar therefore reads as sparse,
/// and it should — 8 ticks is what forty epochs of settle history was built at.
pub fn chain_timeline(chain: &crate::lens::Chain, idx: usize) -> String {
    let Some(c) = chain.cohort.as_ref() else {
        return String::new();
    };
    let mut bar = String::new();
    for i in 0..c.len() {
        if i == idx {
            bar.push_str("[#]");
        } else {
            bar.push('#');
        }
        if i + 1 < c.len() {
            bar.push_str("---");
        }
    }
    let residual = match (chain.residual_mean.get(idx).copied().flatten(), chain.residual_max.get(idx).copied().flatten())
    {
        (Some(m), Some(x)) => format!(
            "stage residual: mean {m:.3} m, max {x:.3} m across {} tiles -- the mean |dh| of this stage's FINAL \
             epoch, recorded. It is what the kernel DID, not a criterion it met: sustained uplift pins erosion's \
             residual at the driver's rate, so there is no near-stationarity to gate on \
             ( #obs-erosion-residual-is-driver-bound )",
            c.tiles.get(idx).copied().unwrap_or(0)
        ),
        _ => "stage residual: NOT RECORDED for this stage (endpoint carried over from a pre-chain build; the \
              record is made at compute time and is never backfilled)"
            .to_string(),
    };
    let ragged = if c.is_square() {
        String::new()
    } else {
        format!(
            "\n  !! ragged chain: stages cover {:?} tiles respectively -- a frame is one moment only where its \
             stage covers everything; elsewhere you are seeing the uncarved prior",
            c.tiles
        )
    };
    let others = if chain.all.len() > 1 {
        format!(
            "  |  chain {}/{} (G cycles): {}",
            chain.sel + 1,
            chain.all.len(),
            chain
                .all
                .iter()
                .map(|o| format!(
                    "L{} x{}{}",
                    o.level,
                    o.len(),
                    if std::ptr::eq(o, c) { "*" } else { "" }
                ))
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        String::new()
    };
    let where_ = match (c.is_global(), c.centre()) {
        (true, _) => "whole globe".to_string(),
        (false, Some((face, i, j))) => format!(
            "a {}-cell WINDOW at face {face} ({i}, {j}) -- press B to go there, or you will not find it",
            c.span_cells()
        ),
        (false, None) => "extent unknown".to_string(),
    };
    format!(
        "epoch {} {bar} {}   {} materialized stages, stride {}, L{} over {}{}{}\n  {residual}",
        c.epochs.first().copied().unwrap_or(0),
        c.epochs.last().copied().unwrap_or(0),
        c.len(),
        c.epochs.get(1).zip(c.epochs.first()).map(|(b, a)| b - a).unwrap_or(0),
        c.level,
        where_,
        others,
        ragged,
    )
}

/// The honesty block from the shared reader — **the same text** `vivarium watch`
/// prints under its ASCII globe, byte for byte. One instrument at two fidelities
/// of attention, not two instruments that happen to agree today.
pub fn honesty(roots: &[RootEntry]) -> String {
    watch::honesty_block(roots)
}

/// The declared-affordance block: what on screen has no world referent.
///
/// #norm-no-depiction-without-referent owns this. It is generated from the
/// running view's state rather than maintained as prose, because the norm's own
/// Working Notes record that nothing enumerates these and an unmaintained list
/// would be the same gap with more words.
pub fn depiction(frame: &Frame, headlight: bool) -> Vec<String> {
    crate::paint::declared_affordances(
        frame.req.exag,
        headlight,
        frame.seam.cross_max > 0.0,
        frame.req.paint,
    )
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
    "drag spin | wheel zoom (past L9 the globe becomes a REGION WINDOW) | [ ] level | A auto-level\n\
     X relief | O pole | R reset | B GO TO THE SELECTED CHAIN'S REGION | G cycle chains\n\
     TAB paint (1 surface 2 provenance 3 water 4 seam 5 change 6 depression) | Z change scale | P present\n\
     E EROSION SETTLE HISTORY (world-time) | T deep time (mantle cooling) | V replay (build history)\n\
     K play/pause | J/L step one stage | , . hour | N M day | Y headlight | C CAPTURE SIGHTING | Esc quit"
        .to_string()
}
