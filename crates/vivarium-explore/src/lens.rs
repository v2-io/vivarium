//! **What the explorer is looking at** — and, inseparably, what it is entitled
//! to say about it.
//!
//! A lens is a *selection*, never an authorship: which materialized state to
//! observe ( #form-core-view-wall FE(4)). The four lenses are the four
//! questions an instrument for checking a world's systems gets asked —
//! *what is the world now*, *how did the deep-time driver get here*, *how did
//! the surface itself get here in world-time*, and *how did the builder get
//! here in build-time* — and the last three are deliberately the same mechanism
//! at three ends ( #form-time-indexed-stage-chains FE(5)).
//!
//! **The two time axes are different kinds and the difference is load-bearing.**
//! `Stage` walks the mantle-thermal cooling chain, which is **law-evaluable**:
//! $T_p(t)$ is closed form, so a stage nobody built can still be evaluated and
//! the view may sample it as finely as it likes. `Erosion` walks the fluvial
//! settle history, which is **materialized-only**: a stage exists because
//! something ran to produce it. Its density is exactly what the builder ran, and
//! asking for more is a *build* request, never a view parameter
//! ( #form-time-indexed-stage-chains FE(8) ). That is why `--frames` reaches the
//! first and nothing reaches the second: there is no dial to turn, and offering
//! one would be the viewer carrying knowledge that belongs to the world's phase.

use std::collections::BTreeSet;

use vivarium_world::lithosphere::MANTLE_TP_C;
use vivarium_world::mantle_thermal::{self, potential_temp_c, present_abyssal};
use vivarium_world::query::World;
use vivarium_world::store::{RootEntry, Store};
use vivarium_world::time::Time;
use vivarium_world::watch::{self, Coverage};

/// Which state of the world is on screen.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lens {
    /// The live present: the builder's fluvial tiles assembled over the tectonic
    /// surface, at the present mantle temperature.
    Present,
    /// One **deep-time** cooling stage — a materialized point on the law's own
    /// ladder (`mantle_thermal::cooling_stages`). The surface here is the pure
    /// isostatic tectonic surface at that mantle temperature: no fluvial tiles
    /// exist for any epoch but the present, and pretending otherwise would be
    /// the instrument lying about the one thing it is for.
    Stage(usize),
    /// One **erosion settle stage** — index into [`Chain::epochs`]. Every tile on
    /// screen is drawn at the *same* epoch, so this is the first **world-time**
    /// playback this tree can offer: the surface as it stood after that many
    /// epochs of fluvial work, not the order the builder happened to write it.
    Erosion(usize),
    /// **Replay**: the store as it stood after the first *n* roots landed. This
    /// is *build* history — the order the builder wrote things — not world-time.
    Replay(usize),
}

/// The deep-time ladder, with each stage's store residency.
///
/// ## Why the view samples the ladder more densely than the builder materialized
///
/// The explorer's purpose is to let a human pattern-matcher detect physics that
/// is missing or behaving unnaturally, and that detector runs **on motion**.
/// Discrete jumps defeat it: a surface that teleports between two states cannot
/// be read for whether the transition between them was plausible. So playback
/// density is instrument *resolution*, not polish.
///
/// The tempting way to buy smoothness is to interpolate between materialized
/// stages, and it is exactly wrong here — a tween produces smooth motion that a
/// pattern-matcher reads as physics when it is a renderer's guess. For an
/// instrument whose entire value is that what you see can be trusted, that is
/// the worst available failure.
///
/// The principled way costs the same and is not a compromise: `T_p(t)` is a
/// continuous law and `tectonic_surface_at_tp` accepts any `T_p`, so a denser
/// ladder is **more evaluations of the law**, never a tween. Every frame is a
/// real surface at a real mantle temperature.
///
/// Two constraints keep that from becoming view-authored evolution:
///
/// 1. The view samples along the law's own trajectory and changes nothing about
///    it. Observation density is not materialization density
///    ( #form-core-view-wall FE(4) forbids a view choosing *how the world
///    evolves*; sampling it more finely is the same freedom as choosing a render
///    level).
/// 2. The dense ladder is built by the same **nested bisection** the builder
///    uses ( #form-time-indexed-stage-chains FE(9) ), so it is a strict superset
///    of whatever the builder materialized, containing every built stage
///    bit-exactly. A stage is then either a store citizen or view-computed, per
///    stage, and `vivarium build --frames N` visibly converts the second into
///    the first.
///
/// Residency is the load-bearing column. A stage the builder materialized is a
/// **store citizen** the explorer reads; a stage it did not is something this
/// process works out for you, off the frame path, writing nothing. Both can be
/// drawn; only one of them is the world's own answer, and the difference is
/// exactly what makes `vivarium build` visible instead of optional.
pub struct Ladder {
    pub stages: Vec<Time>,
    /// Ages (Ga before the Holocene origin), one per stage.
    pub ages_ga: Vec<f32>,
    /// Mantle potential temperature (°C) per stage — the pure law read.
    pub tps: Vec<f64>,
    /// Per stage: is its `EpochReduction` a store citizen?
    pub built: Vec<bool>,
    /// The present-Abyssal stage — the one that coincides with the live world.
    pub present_idx: usize,
    /// What this vivium's manifest asked the BUILDER for.
    pub demanded_frames: u32,
    /// Stages the builder's own demand would have produced — so the HUD can say
    /// how much of what you are watching is store-backed.
    pub demanded_stages: usize,
}

impl Ladder {
    /// Read the ladder at `view_frames` sampling density and probe the store for
    /// each stage's residency. Read-only: `epoch_reduction_hit` never computes.
    ///
    /// `view_frames` is rounded up to a nested count, so the result contains
    /// every stage the builder materialized at `demanded_frames` bit-exactly.
    pub fn read(world: &World, demanded_frames: u32, view_frames: u32) -> Ladder {
        let want = view_frames.max(demanded_frames).max(2) as usize;
        let stages = mantle_thermal::cooling_stages_refined(mantle_thermal::refinements_for(want));
        let present = present_abyssal();
        let ages_ga = stages.iter().map(|t| (-t.years() / 1.0e9) as f32).collect();
        let tps: Vec<f64> = stages.iter().map(|&t| potential_temp_c(t)).collect();
        let built = tps.iter().map(|&tp| world.epoch_reduction_hit(tp).is_some()).collect();
        let present_idx = stages.iter().position(|&t| t == present).unwrap_or(0);
        let demanded_stages =
            mantle_thermal::stage_count(mantle_thermal::refinements_for(demanded_frames as usize));
        Ladder { stages, ages_ga, tps, built, present_idx, demanded_frames, demanded_stages }
    }

    /// Re-probe residency (a builder may be running in another terminal — the
    /// timeline should fill in as stages land).
    pub fn refresh_residency(&mut self, world: &World) {
        for (i, &tp) in self.tps.iter().enumerate() {
            if !self.built[i] {
                self.built[i] = world.epoch_reduction_hit(tp).is_some();
            }
        }
    }

    pub fn len(&self) -> usize {
        self.stages.len()
    }

    pub fn built_count(&self) -> usize {
        self.built.iter().filter(|b| **b).count()
    }
}

/// The erosion settle history this explorer can scrub — one cohort, chosen.
///
/// ## Why there is no density parameter here, and never will be
///
/// [`Ladder`] above buys smoothness by evaluating the cooling law at more mantle
/// temperatures, and that is legitimate because the law is closed-form. Nothing
/// of the sort is available here. An erosion stage exists because the kernel
/// integrated to it; reaching epoch 37 *means* paying the 37 epochs. So the
/// chain has exactly the stages the builder ran, this type reports that number,
/// and the only way to get more is `vivarium build`
/// ( #form-time-indexed-stage-chains FE(8) ).
///
/// The temptation the norm forbids is precise and worth naming, because it is
/// the obvious way to make this pretty: 8 stages over a whole planet is a
/// slideshow, and tweening between them would produce exactly the smooth carving
/// motion a pattern-matching eye is watching *for* — a renderer's guess wearing
/// the appearance of geomorphology ( #norm-no-depiction-without-referent FE(5),
/// which names this system by name). The honest picture is the jump, plus the
/// statement that there is nothing in between. That is what
/// [`chain_provenance_line`] says on screen every frame.
#[derive(Clone)]
pub struct Chain {
    /// The chosen cohort, if any stage chain exists at all.
    pub cohort: Option<watch::ErosionCohort>,
    /// Every cohort found, so the HUD can say what was passed over.
    pub all: Vec<watch::ErosionCohort>,
    /// Mean recorded residual per stage (m), index-parallel to the cohort's
    /// epochs; `None` where the stage recorded none (pre-chain endpoints never
    /// got one — the record is made at compute time and is never backfilled).
    pub residual_mean: Vec<Option<f32>>,
    pub residual_max: Vec<Option<f32>>,
}

impl Chain {
    pub fn read(roots: &[RootEntry]) -> Chain {
        let all = watch::erosion_cohorts(roots);
        let cohort = all.first().filter(|c| c.len() > 1).cloned();
        let n = cohort.as_ref().map(|c| c.len()).unwrap_or(0);
        Chain { cohort, all, residual_mean: vec![None; n], residual_max: vec![None; n] }
    }

    pub fn len(&self) -> usize {
        self.cohort.as_ref().map(|c| c.len()).unwrap_or(0)
    }

    pub fn epoch(&self, i: usize) -> Option<u32> {
        self.cohort.as_ref().and_then(|c| c.epochs.get(i).copied())
    }

    /// The key predicate selecting exactly one world-moment: this cohort's source
    /// tree, this epoch. Both fields matter — the epoch alone would mix cohorts,
    /// and the source alone is the whole history at once.
    pub fn stage_predicate(&self, i: usize) -> Option<(String, u32)> {
        let c = self.cohort.as_ref()?;
        Some((c.src.clone(), *c.epochs.get(i)?))
    }
}

/// What the erosion scrub is showing, said in the register of what produced it.
///
/// Three facts a viewer would otherwise have to ask for, and one they would not
/// think to ask: that the stages are addressable but their *intermediate*
/// accuracy is undeclared. The scheme's own authors bound their accuracy claim
/// to the steady-state endpoint and explicitly except the transient at large
/// timestep, and an interior stage is exactly a transient state
/// ( #form-time-indexed-stage-chains FE(8) ).
pub fn chain_provenance_line(chain: &Chain, i: usize) -> String {
    let Some(c) = chain.cohort.as_ref() else {
        return "no erosion settle history exists in this store -- `vivarium build` with a stage stride \
                is what creates one"
            .to_string();
    };
    let e = c.epochs.get(i).copied().unwrap_or(0);
    let prev = if i > 0 { c.epochs.get(i - 1).copied() } else { None };
    let between = match prev {
        Some(p) => format!(
            "NOTHING exists between epoch {p} and epoch {e} -- the jump you see IS the discontinuity, \
             and no frame will ever be inserted there by this view"
        ),
        None => "this is the earliest materialized stage; the world before it is the uncarved prior".to_string(),
    };
    format!(
        "surface = every tile at epoch {e}, one world-moment ({} tiles at L{}). {between}. \
         Materialized-only chain: {} stages is exactly what the builder ran, and there is no view dial \
         for more ( #form-time-indexed-stage-chains FE(8) ). Addressable is NOT certified-accurate -- \
         an interior stage is a transient state, and the scheme's accuracy claim covers only the endpoint",
        c.tiles.get(i).copied().unwrap_or(0),
        c.level,
        c.len(),
    )
}

/// The store facts one frame is drawn from — assembled by the worker, read by
/// the HUD. Everything here is *measured off the store*, never asserted.
pub struct FrameFacts {
    /// The erosion-tile roots contributing to this frame's surface.
    pub eroded_tiles: usize,
    /// Erosion roots present but carved under a different source tree.
    pub stale_tiles: usize,
    /// Fraction of drawn cells with no covering tile — painted from the fated
    /// prior. Silent fallback made countable.
    pub prior_fallback_frac: f32,
    /// Fraction of drawn cells above the sea datum.
    pub land_frac: f32,
    /// Cells carrying standing water *above* the sea datum — the only inland
    /// hydrology that exists anywhere in this tree, and no view has ever drawn
    /// it. Zero here with watered tiles present is itself a finding.
    pub inland_water_cells: usize,
    /// Total standing-water cells in the loaded water field (inland + marine).
    pub water_cells: usize,
    /// Water tiles the census named, and how many were settled under the CURRENT
    /// source hash. A gap is staleness, and it is the difference between "this
    /// world has no lakes" and "this binary cannot see the lakes it has".
    pub water_requested: usize,
    pub water_loaded: usize,
    /// Sea datum used (m), and whether it is the store's or this view's estimate.
    pub sea_m: f32,
    pub sea_provenance: SeaProvenance,
    /// Craton growth scalar at this frame's mantle temperature, and how many
    /// fated sites the union is taken over — the two numbers that explain
    /// "circles that stop growing" without anyone having to ask.
    pub craton_growth: f64,
    pub craton_sites: usize,
    pub tp_c: f64,
    /// Puts this view's store handle has refused. The wall, counted.
    pub refused_writes: usize,
    pub pull_s: f32,

    // --- the change channel: measured, not asserted -----------------------
    /// Signed elevation change (m) of every drawn cell against the **uncarved
    /// initial topography** — the surface the fluvial kernel was seeded from.
    /// Present only when the change paint is up, because computing it costs one
    /// law evaluation per cell.
    ///
    /// This is the *net* of two things the store does not separate: the kernel's
    /// uplift driver, which raises nearly everything, and fluvial incision, which
    /// lowers a small minority. The fractions below are what let a viewer read
    /// which is which rather than seeing "erosion" everywhere.
    pub change_mean: f32,
    pub change_min: f32,
    pub change_max: f32,
    /// Fraction of drawn cells that have RISEN / FALLEN by more than 0.5 m.
    pub frac_rising: f32,
    pub frac_falling: f32,
    /// This frame's world-time epoch, when the lens is the settle history.
    pub stage_epoch: Option<u32>,
    pub stage_tiles: usize,
}

/// Where the sea datum on screen came from.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SeaProvenance {
    /// A store citizen the builder materialized — the world's own answer.
    StoreCitizen,
    /// Computed by this process because the builder has not materialized this
    /// stage. Correct, off the frame path, and written nowhere.
    ViewComputed,
}

impl SeaProvenance {
    pub fn label(self) -> &'static str {
        match self {
            SeaProvenance::StoreCitizen => "store citizen (built)",
            SeaProvenance::ViewComputed => "VIEW-COMPUTED (this stage is not built)",
        }
    }
}

/// Read the recorded residual for every stage of the chosen cohort.
///
/// The residual is a keyed sibling of the stage itself (`aspect=stage-residual`,
/// one f32 — #form-time-indexed-stage-chains FE(3)), so it is read the same way
/// the stage is: off the root census, by complete key. It is read *by cohort*
/// rather than through `World::erosion_stage_residual`, which builds its key with
/// the running binary's source digest and therefore cannot see a previous
/// world's history at all.
///
/// **What the number is, and is not.** It is the mean $|\Delta h|$ of the
/// stage's final epoch — what the kernel *did*. It is not a convergence
/// criterion and nothing gates on it: sustained uplift pins erosion's residual at
/// the driver's rate, so there is no near-stationarity to detect
/// ( #obs-erosion-residual-is-driver-bound ).
pub fn read_residuals(store: &Store, roots: &[RootEntry], chain: &mut Chain) {
    let Some(c) = chain.cohort.as_ref() else {
        return;
    };
    let mut sums = vec![(0.0f64, 0usize, 0.0f32); c.len()];
    for r in roots {
        if !r.key.starts_with("erosion-tile@")
            || watch::key_field(&r.key, "aspect") != Some("stage-residual")
            || watch::key_field(&r.key, "src") != Some(c.src.as_str())
        {
            continue;
        }
        let Some(e) = watch::key_field(&r.key, "epochs").and_then(|v| v.parse::<u32>().ok()) else {
            continue;
        };
        let Some(i) = c.epochs.iter().position(|x| *x == e) else {
            continue;
        };
        let Some(bytes) = store.object_bytes(&r.object) else {
            continue;
        };
        if bytes.len() < 4 {
            continue;
        }
        let v = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        sums[i].0 += v as f64;
        sums[i].1 += 1;
        sums[i].2 = sums[i].2.max(v);
    }
    for (i, (sum, n, max)) in sums.into_iter().enumerate() {
        if n > 0 {
            chain.residual_mean[i] = Some((sum / n as f64) as f32);
            chain.residual_max[i] = Some(max);
        }
    }
}

/// The set of erosion-tile keys that had landed by replay frame `n`.
///
/// Replay assembles the surface through the *same* loader the live path uses
/// ( `World::load_eroded_regions_where` ), differing only in the key predicate —
/// which is what FE(5) demands: one mechanism, two ends.
pub fn replay_key_set(landings: &[watch::Landing], upto: usize) -> BTreeSet<String> {
    landings[..upto.min(landings.len())]
        .iter()
        .filter(|l| l.root.key.starts_with("erosion-tile@"))
        .map(|l| l.root.key.clone())
        .collect()
}

/// Roots that had landed by replay frame `n` — the census the honesty block and
/// coverage read for that frame, so the overlay ages with the picture.
pub fn replay_roots(landings: &[watch::Landing], upto: usize) -> Vec<RootEntry> {
    landings[..upto.min(landings.len())].iter().map(|l| l.root.clone()).collect()
}

/// The mantle temperature a lens observes.
pub fn lens_tp(lens: Lens, ladder: &Ladder) -> f64 {
    match lens {
        // The settle history is surface time, not mantle time: the fluvial chain
        // was run at the present mantle temperature, so scrubbing it must not
        // also move T_p. Two clocks that happen to share a slider would be a
        // depiction of a coupling this tree does not have.
        Lens::Present | Lens::Replay(_) | Lens::Erosion(_) => MANTLE_TP_C,
        Lens::Stage(i) => ladder.tps.get(i).copied().unwrap_or(MANTLE_TP_C),
    }
}

/// A one-line statement of what the drawn surface **is**, in the register of the
/// law that produced it. This is the line that should make an unasked question
/// unnecessary.
pub fn surface_provenance_line(
    lens: Lens,
    facts: &FrameFacts,
    cov: &Coverage,
    view_level: u8,
    chain: &Chain,
) -> String {
    match lens {
        Lens::Erosion(i) => chain_provenance_line(chain, i),
        Lens::Present => {
            if facts.eroded_tiles == 0 {
                return "surface = the pure tectonic prior. NO fluvial tile covers anything on screen \
                        (nothing carved is built) -- `vivarium build` is what changes that"
                    .to_string();
            }
            // The fidelity trap, and it is silent without this line. An
            // `ErodedRegion` answers only at its own level or finer
            // (`erosion::ErodedRegion::covers`), so a whole-globe view coarser
            // than the build level draws the UNCARVED prior everywhere while the
            // store is full of carved tiles. Nothing looks wrong; you simply are
            // not looking at the thing you think you are looking at.
            if view_level < cov.level && facts.prior_fallback_frac > 0.99 {
                format!(
                    "surface = the UNCARVED tectonic prior. {} fluvial tiles exist but NONE applies at view L{}: \
                     an eroded region answers only at its own level (L{}) or finer. Press ] to L{}, or zoom in, \
                     to see the carve at all.",
                    facts.eroded_tiles, view_level, cov.level, cov.level
                )
            } else {
                format!(
                    "surface = tectonic prior (bathymetry + isostatic freeboard), fluvially carved where the \
                     builder has tiles -- {} erosion-tile roots, {:.0}% of drawn cells fall back to the uncarved prior",
                    facts.eroded_tiles,
                    facts.prior_fallback_frac * 100.0
                )
            }
        }
        Lens::Stage(_) =>
            "surface = the PURE isostatic tectonic surface at this mantle T_p. No fluvial carve exists at any \
             epoch but the present, so deep time is tectonics + isostasy only -- never eroded terrain"
                .to_string(),
        Lens::Replay(_) => format!(
            "surface = the store as it stood after these roots landed -- BUILD history, not world-time \
             ({} erosion tiles had landed; {:.0}% of drawn cells are still the uncarved prior)",
            facts.eroded_tiles,
            facts.prior_fallback_frac * 100.0
        ),
    }
}
