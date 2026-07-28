//! The build reader — **one** poller over store roots, serving both live
//! watching and replay.
//!
//! Claim home: `#form-time-indexed-stage-chains` FE(5). Its exact words are the
//! design constraint here: *"Replay and live-watching are one mechanism, not two
//! features. … Building them as two features produces two, and they will
//! disagree."* So there is one census type, one ordering, and one renderer call
//! in this module; `--replay` changes only whether new landings are still
//! arriving. If a future change makes the live path and the replay path compute
//! different things, that segment's FE(5) has been falsified in practice and
//! should be revisited rather than worked around.
//!
//! The store is already the bus ( #form-store-as-save , `#detail-builder-daemon`
//! FE(1)), so this reader needs no protocol with the builder and cannot disturb
//! it: it opens no lock, writes nothing, and holds no handle the builder waits
//! on. An explorer never blocks ( #form-builder-admission ).
//!
//! ## What "replay" honestly is here, and what it is not
//!
//! Root files carry no world-time. What this module can order by is the
//! **filesystem landing time** of each root — *when the builder wrote it*, not
//! *when in world-time the state it holds occurred*. Those coincide only for a
//! build that happens to compute in world-time order. So `replay` is **build
//! history**, and [`interior`] measures the gap: for each nomos it reports how
//! many distinct time-indices exist in the store, i.e. how much interior the
//! build actually has to replay in the FE(2) sense. Where that count is 1, the
//! nomos has no interior — only endpoints — and no reader can conjure one.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::store::RootEntry;

/// How far the builder has carried one tile — the **build-state** ladder, read
/// from the store census and nothing else.
///
/// This is a claim about *what is in the store*, never about geology: a region
/// is `Eroded` because an `erosion-tile` root covers it, not because anything
/// decided it had eroded. `#form-builder-admission` FE(4) is why the
/// distinction has teeth — a view that cannot tell "built" from "computed just
/// now for you" cannot report whether `vivarium build` did anything.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum BuildState {
    /// No tile for this region in the store — anything drawn here is the fated
    /// prior, which is a pure function of the world's identity, not built state.
    Unbuilt,
    /// Only the `initial-topography` tile exists.
    InitialTopography,
    /// The fluvial `erosion-tile` exists (carved bed, no water settled).
    Eroded,
    /// The `water-tile` exists — water settled on that eroded bed.
    Watered,
}

/// What a tile's roots are, beyond how deep they go.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TileFlags {
    /// At least one root here was written under waived flux admission
    /// ( #form-builder-admission FE(3) ) — matured bytes that are not lawful
    /// evidence.
    pub provisional: bool,
    /// At least one root here was computed under a **different source tree**
    /// (`src=` mismatch). Its bytes exist and no reader at this source hash can
    /// use them, so the surface silently falls back to the prior there.
    pub stale: bool,
}

impl BuildState {
    /// Short label for a legend or HUD.
    pub fn label(self) -> &'static str {
        match self {
            BuildState::Unbuilt => "unbuilt",
            BuildState::InitialTopography => "initial-topography",
            BuildState::Eroded => "eroded",
            BuildState::Watered => "watered",
        }
    }
}

/// The store census parsed into per-tile build-state at one display level —
/// the **shared** coverage reader.
///
/// Both renderers consume this one type: the ASCII globe (`globe::render`, used
/// by `vivarium watch` / `info`) and the 3D explorer. FE(5) of
/// #form-time-indexed-stage-chains is about live-vs-replay, but the same
/// argument applies across renderers: coverage computed twice is coverage that
/// will eventually disagree with itself, and then the two instruments report
/// different worlds while both look authoritative.
pub struct Coverage {
    /// The deepest level any surface tile reached — what a viewer is looking at.
    pub level: u8,
    /// Tile edge in cells (the builder's sweep unit; 64 today).
    pub nx: usize,
    /// `(face, oi, oj)` origins with an `initial-topography` tile.
    pub initial_topo: BTreeSet<(u8, u32, u32)>,
    /// `(face, oi, oj)` → the erosion `epochs` its tile was carved at (needed to
    /// re-pull the field by its complete key, never by "the latest one").
    pub erosion: BTreeMap<(u8, u32, u32), u32>,
    /// `(face, oi, oj)` → the water tile's `(eepochs, steps)` — the eroded bed it
    /// settled onto and its own relaxation index ( #form-depend-by-key-never-latest ;
    /// reading `eepochs` as water's clock is the near-miss `time_index_field`
    /// guards against, so both are kept and named separately).
    pub watered: BTreeMap<(u8, u32, u32), (u32, u32)>,
    /// Origins with at least one root written under **waived** flux admission.
    pub provisional: BTreeSet<(u8, u32, u32)>,
    /// Origins with at least one root from a **different source tree**. These
    /// are counted but NOT laddered: see [`Coverage::parse`].
    pub stale: BTreeSet<(u8, u32, u32)>,
}

/// Pull one `key=value` field out of a canonical complete-key string.
///
/// Public because a *chain* reader outside this crate needs it: selecting one
/// settle-history stage is a predicate over `epochs=` and `src=` in the complete
/// key ( [`crate::query::World::load_eroded_regions_where`] takes exactly such a
/// predicate), and a caller that hand-rolls the parse is a second parser that
/// can disagree with this one about what a key says.
pub fn key_field<'a>(key: &'a str, name: &str) -> Option<&'a str> {
    key.split('|').find_map(|f| f.strip_prefix(name).and_then(|r| r.strip_prefix('=')))
}

impl Coverage {
    /// Parse a raw root census. Only tiles at the deepest built level count
    /// toward coverage — a mixed-level store shows its finest built rung.
    ///
    /// **The ladder counts only roots readable at the CURRENT source hash.** Any
    /// nomos key folds the whole-crate source digest, so a root written under a
    /// different source tree cannot be pulled by this binary: no reader can use
    /// its bytes, and every surface assembled here falls back to the prior where
    /// it lies. A census that laddered it anyway would report "watered" over
    /// terrain the renderer is drawing from the uncarved prior — the instrument
    /// contradicting itself in two adjacent lines, which is worse than either
    /// answer alone. Stale origins are recorded in [`Coverage::stale`] and shown
    /// as their own state, because "there is a tile here that I cannot read" is
    /// a different and more actionable fact than "there is nothing here".
    ///
    /// Provisional roots, by contrast, DO ladder: their bytes are readable and
    /// will be drawn. It is the flag, not absence, that marks them unlawful.
    pub fn parse(roots: &[RootEntry]) -> Coverage {
        let level = roots
            .iter()
            .filter(|r| {
                r.key.starts_with("initial-topography@") || r.key.starts_with("erosion-tile@")
            })
            .filter_map(|r| key_field(&r.key, "level").and_then(|v| v.parse::<u8>().ok()))
            .max()
            .unwrap_or(6);
        let mut cov = Coverage {
            level,
            nx: 64,
            initial_topo: Default::default(),
            erosion: Default::default(),
            watered: Default::default(),
            provisional: Default::default(),
            stale: Default::default(),
        };
        for r in roots {
            let k = r.key.as_str();
            let nomos = k.split('@').next().unwrap_or("");
            match key_field(k, "level").and_then(|v| v.parse::<u8>().ok()) {
                Some(l) if l == level => {}
                _ => continue,
            }
            let (face, oi, oj) = match (
                key_field(k, "face").and_then(|v| v.parse::<u8>().ok()),
                key_field(k, "oi").and_then(|v| v.parse::<u32>().ok()),
                key_field(k, "oj").and_then(|v| v.parse::<u32>().ok()),
            ) {
                (Some(f), Some(oi), Some(oj)) => (f, oi, oj),
                _ => continue,
            };
            if let Some(nx) = key_field(k, "nx").and_then(|v| v.parse::<usize>().ok()) {
                cov.nx = nx;
            }
            if r.provisional {
                cov.provisional.insert((face, oi, oj));
            }
            if key_field(k, "src") != Some(crate::nomotheke::SRC_HASH) {
                cov.stale.insert((face, oi, oj));
                continue; // exists, unreadable here — counted, never laddered
            }
            let num = |name: &str| key_field(k, name).and_then(|v| v.parse::<u32>().ok());
            match nomos {
                "initial-topography" => {
                    cov.initial_topo.insert((face, oi, oj));
                }
                "erosion-tile" => {
                    // A staged build leaves MANY roots per tile (the settle
                    // history); coverage is "how far has this tile been carved",
                    // which is the latest stage, not whichever root iterated
                    // last. Residual siblings (`aspect=`) are metadata, skipped.
                    if key_field(k, "aspect").is_some() {
                        continue;
                    }
                    let e = num("epochs").unwrap_or(0);
                    let slot = cov.erosion.entry((face, oi, oj)).or_insert(0);
                    *slot = (*slot).max(e);
                }
                "water-tile" => {
                    cov.watered.insert(
                        (face, oi, oj),
                        (num("eepochs").unwrap_or(0), num("steps").unwrap_or(0)),
                    );
                }
                _ => {}
            }
        }
        cov
    }

    /// The deepest nomos materialized for the tile containing cell `(ci, cj)` on
    /// `face`.
    pub fn state_at_cell(&self, face: u8, ci: u32, cj: u32) -> BuildState {
        let n = self.nx as u32;
        self.state(face, (ci / n) * n, (cj / n) * n)
    }

    /// Flags for the tile containing cell `(ci, cj)`. `stale` is set only when
    /// there is nothing readable at that origin — see [`Self::stale_only_tiles`].
    pub fn flags_at_cell(&self, face: u8, ci: u32, cj: u32) -> TileFlags {
        let n = self.nx as u32;
        let o = (face, (ci / n) * n, (cj / n) * n);
        TileFlags {
            provisional: self.provisional.contains(&o),
            stale: self.stale.contains(&o) && self.state(o.0, o.1, o.2) == BuildState::Unbuilt,
        }
    }

    /// The deepest nomos materialized for the tile at origin `(oi, oj)`.
    pub fn state(&self, face: u8, oi: u32, oj: u32) -> BuildState {
        let t = (face, oi, oj);
        if self.watered.contains_key(&t) {
            BuildState::Watered
        } else if self.erosion.contains_key(&t) {
            BuildState::Eroded
        } else if self.initial_topo.contains(&t) {
            BuildState::InitialTopography
        } else {
            BuildState::Unbuilt
        }
    }

    /// Every origin with any tile, and how far each got — the coverage tally a
    /// legend footer or HUD reports. Unbuilt regions are, by construction, not
    /// in the census: what was never materialized cannot be counted here.
    pub fn tally(&self) -> [usize; 4] {
        let mut origins: BTreeSet<(u8, u32, u32)> = self.initial_topo.iter().copied().collect();
        origins.extend(self.erosion.keys().copied());
        origins.extend(self.watered.keys().copied());
        let mut t = [0usize; 4];
        for &(f, oi, oj) in &origins {
            t[self.state(f, oi, oj) as usize] += 1;
        }
        t
    }

    /// Origins whose ONLY roots are from another source tree — built, and
    /// unreadable here.
    ///
    /// Deliberately not `self.stale.len()`. A store accumulates roots across
    /// rebuilds, so after any source edit and rebuild almost every origin has
    /// both stale and current roots; counting the raw set reported "384 readable
    /// tiles" and "384 stale tiles" on the same screen, which is an instrument
    /// contradicting itself. What is actionable is the origin that has *nothing*
    /// readable.
    pub fn stale_only_tiles(&self) -> usize {
        self.stale.iter().filter(|&&(f, oi, oj)| self.state(f, oi, oj) == BuildState::Unbuilt).count()
    }

    /// How many tiles have any READABLE root at the display level.
    pub fn built_tiles(&self) -> usize {
        let mut origins: BTreeSet<(u8, u32, u32)> = self.initial_topo.iter().copied().collect();
        origins.extend(self.erosion.keys().copied());
        origins.extend(self.watered.keys().copied());
        origins.len()
    }
}

/// One store root together with when it landed on disk.
pub struct Landing {
    pub root: RootEntry,
    pub at: SystemTime,
}

/// Every root in `dir`'s store, with its landing time, ordered oldest-first.
///
/// Ties are broken by key so the order is deterministic: a replay of the same
/// store must produce the same sequence every time, and filesystem enumeration
/// order is not stable across platforms.
pub fn landings(dir: &Path) -> io::Result<Vec<Landing>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir.join("roots"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "tmp") {
            continue;
        }
        let at = path.metadata().and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH);
        let text = std::fs::read_to_string(&path)?;
        let mut lines = text.lines();
        let object = lines.next().unwrap_or("").trim().to_string();
        let key = lines.next().unwrap_or("").trim().to_string();
        let provisional = lines.any(|l| l.trim() == "provisional");
        out.push(Landing { root: RootEntry { key, object, provisional }, at });
    }
    out.sort_by(|a, b| a.at.cmp(&b.at).then_with(|| a.root.key.cmp(&b.root.key)));
    Ok(out)
}

/// Cumulative prefix lengths marking replay frame boundaries.
///
/// Landings sharing a timestamp are never split across frames — they are
/// indistinguishable in the only ordering the store offers, so cutting between
/// them would invent an order the evidence does not support. When that leaves
/// more groups than `max_frames`, whole groups are coalesced (never split).
pub fn frame_bounds(landings: &[Landing], max_frames: usize) -> Vec<usize> {
    if landings.is_empty() {
        return Vec::new();
    }
    let mut groups = Vec::new();
    for (i, l) in landings.iter().enumerate() {
        if i == 0 || l.at != landings[i - 1].at {
            groups.push(i);
        }
    }
    groups.push(landings.len());
    let n_groups = groups.len() - 1;
    let max = max_frames.max(1);
    if n_groups <= max {
        return groups[1..].to_vec();
    }
    // Coalesce: keep `max` group boundaries, spread evenly over the groups.
    (1..=max).map(|f| groups[(f * n_groups).div_ceil(max).min(n_groups)]).collect()
}

/// Which key field carries a nomos's **own** time-index, if any.
///
/// Deliberately narrow, and one entry here is a correction worth keeping: the
/// water tile's `eepochs` field is *not* its time-index — it names **which
/// eroded bed** to settle onto, a dependency selector under
/// #form-depend-by-key-never-latest . Water's own relaxation index is `steps`.
/// Reading `eepochs` as water's clock is the plausible-and-wrong story that
/// `msc/build-parameterization-findings-2026-07-24.md` §"Near-miss" records; it
/// is repeated here because this is the second place that mistake can be made.
/// The `mantle-thermal` entry is the one that must be read from the key
/// construction rather than guessed from the nomos list: the cooling-stage
/// reductions are keyed under `MANTLE_THERMAL` with `aspect=epoch-reduction`
/// (`query::World::epoch_reduction_key`), so "epoch-reduction" is not a nomos
/// name and looking for one finds nothing. Guessing it cost a wrong first
/// reading here — the one chain in the tree that *does* have an interior showed
/// up as having none.
fn time_index_field(nomos: &str) -> Option<&'static str> {
    match nomos {
        "erosion-tile" => Some("epochs"),
        "water-tile" => Some("steps"),
        "mantle-thermal" => Some("tp_bits"),
        _ => None,
    }
}

/// How addressable one nomos's interior actually is.
pub struct InteriorReport {
    pub nomos: String,
    /// The key field carrying this nomos's time-index, when it declares one.
    pub axis: Option<&'static str>,
    /// Distinct time-index values present in the store. `1` means the build has
    /// no interior for this nomos — only an endpoint.
    pub distinct: usize,
    pub roots: usize,
}

/// Measure, per nomos, how many distinct time-indices the store holds.
///
/// This is the failable half of the reader ( #norm-declaration-must-convict ):
/// FE(2) of #form-time-indexed-stage-chains claims that without a time-index a
/// build is *"opaque between started and finished"*. That is a checkable
/// statement about this tree, and this function checks it rather than restating
/// it. A nomos reporting `distinct: 1` on a completed build is the claim being
/// borne out; a nomos that grows an interior refutes it for that nomos.
pub fn interior(roots: &[RootEntry]) -> Vec<InteriorReport> {
    let mut per: BTreeMap<String, (BTreeSet<String>, usize)> = BTreeMap::new();
    for r in roots {
        let nomos = r.key.split('@').next().unwrap_or("").to_string();
        if nomos.is_empty() {
            continue;
        }
        let e = per.entry(nomos.clone()).or_default();
        e.1 += 1;
        if let Some(axis) = time_index_field(&nomos) {
            if let Some(v) = r
                .key
                .split('|')
                .find_map(|f| f.strip_prefix(axis).and_then(|r| r.strip_prefix('=')))
            {
                e.0.insert(v.to_string());
            }
        }
    }
    per.into_iter()
        .map(|(nomos, (vals, roots))| InteriorReport {
            axis: time_index_field(&nomos),
            distinct: vals.len(),
            nomos,
            roots,
        })
        .collect()
}

/// One erosion **settle history**: the stages carved under a single source tree
/// *at a single level*.
///
/// A chain is coherent only within one `(src, level)` cohort, and **both halves
/// were learned the hard way**.
///
/// `src`: two stages carved under different source trees are stages of two
/// different worlds, and ordering them on one time axis puts the difference
/// between two kernels on screen as though it were the passage of world-time —
/// the stale-`src` two-datum fault, moved onto the time axis.
///
/// `level`: a world can hold a coarse whole-globe sweep *and* a fine beacon
/// patch under the same source tree, with different epoch ladders over different
/// footprints (live instance, 2026-07-28: a global L9 sweep at epochs 5…40 beside
/// an L13 beacon at epochs 10…300). Merging those yields one ladder whose rungs
/// cover wildly unequal areas, so a scrub would show a region appearing and
/// vanishing as it stepped — a *coverage* animation reading as geology. Level is
/// therefore identity for a chain, not an attribute of one.
#[derive(Clone, Debug)]
pub struct ErosionCohort {
    /// The whole-crate source digest every stage in this cohort was carved under.
    pub src: String,
    /// Whether that digest is the one this binary was built from — i.e. whether
    /// these stages are *this* world's settle history or a previous world's.
    pub is_current: bool,
    /// Per face, the cohort's cell-grid extent at `level`:
    /// `(oi_min, oj_min, oi_end, oj_end)` half-open. A fine chain covers a small
    /// window of one face, and a viewer cannot find it by orbiting — so the
    /// extent is part of the census rather than something the view guesses.
    pub bounds: BTreeMap<u8, (u32, u32, u32, u32)>,
    /// Distinct `epochs` values present, ascending. **This is the chain's
    /// density, and it is not a view parameter**: erosion is a materialized-only
    /// chain ( #form-time-indexed-stage-chains FE(8) ), so a consumer gets
    /// exactly what was built and asking for more is a build request.
    pub epochs: Vec<u32>,
    /// Tiles present at each epoch, index-parallel to `epochs`. Unequal entries
    /// mean the chain is ragged — some tiles have a longer history than others,
    /// which a scrub must report rather than smooth over.
    pub tiles: Vec<usize>,
    /// The level every stage in this cohort was carved at — identity, see above.
    pub level: u8,
}

impl ErosionCohort {
    /// Distinct materialized stages. `<= 1` means this cohort has no interior.
    pub fn len(&self) -> usize {
        self.epochs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.epochs.is_empty()
    }
    /// True when every stage covers the same number of tiles — the condition
    /// under which a scrub frame is one world-moment everywhere it draws.
    pub fn is_square(&self) -> bool {
        self.tiles.windows(2).all(|w| w[0] == w[1])
    }

    /// Whether this chain covers a whole face at its level, or a window into one.
    /// A whole-globe sweep is drawable from orbit; a beacon patch is not, and the
    /// difference decides whether a viewer needs to be taken to it.
    pub fn is_global(&self) -> bool {
        let n = 1u32 << self.level;
        self.bounds.len() > 1
            || self.bounds.values().any(|&(a, b, c, d)| a == 0 && b == 0 && c == n && d == n)
    }

    /// The cohort's centre cell — `(face, i, j)` at its own level. What a viewer
    /// has to be pointed at to see a fine chain at all.
    pub fn centre(&self) -> Option<(u8, u32, u32)> {
        // The face holding the widest extent; ties break by face index so the
        // answer is stable across runs.
        let (&face, &(i0, j0, i1, j1)) = self
            .bounds
            .iter()
            .max_by_key(|(f, &(a, b, c, d))| ((c - a) as u64 * (d - b) as u64, std::cmp::Reverse(**f)))?;
        Some((face, (i0 + i1) / 2, (j0 + j1) / 2))
    }

    /// Widest side of the cohort's extent, in cells at its level — the natural
    /// size for a window that shows the whole chain and not much else.
    pub fn span_cells(&self) -> u32 {
        self.bounds
            .values()
            .map(|&(a, b, c, d)| (c - a).max(d - b))
            .max()
            .unwrap_or(1u32 << self.level)
    }
}

/// Group every `erosion-tile` height root into settle-history cohorts by
/// `(source digest, level)`, richest first.
///
/// Ordering is *usefulness for a time scrub*: cohorts with a real interior come
/// before endpoint-only ones, the current source wins next, and among current
/// chains the one with more stages leads — so the first entry is the chain a
/// viewer should be offered, and the `is_current` flag tells them whether they
/// are watching this world or a previous one. Stage-residual siblings
/// (`aspect=`) are metadata, not surfaces, and are skipped exactly as
/// [`crate::query::World::load_eroded_regions_where`] skips them.
pub fn erosion_cohorts(roots: &[RootEntry]) -> Vec<ErosionCohort> {
    type Acc = (BTreeMap<u32, usize>, BTreeMap<u8, (u32, u32, u32, u32)>);
    let mut per: BTreeMap<(String, u8), Acc> = BTreeMap::new();
    for r in roots {
        if !r.key.starts_with("erosion-tile@") || key_field(&r.key, "aspect").is_some() {
            continue;
        }
        let num = |n| key_field(&r.key, n).and_then(|v| v.parse::<u32>().ok());
        let (Some(src), Some(epochs), Some(level), Some(oi), Some(oj), Some(nx)) = (
            key_field(&r.key, "src"),
            num("epochs"),
            key_field(&r.key, "level").and_then(|v| v.parse::<u8>().ok()),
            num("oi"),
            num("oj"),
            num("nx"),
        ) else {
            continue;
        };
        let face = key_field(&r.key, "face").and_then(|v| v.parse::<u8>().ok()).unwrap_or(0);
        let e = per.entry((src.to_string(), level)).or_default();
        *e.0.entry(epochs).or_default() += 1;
        let b = e.1.entry(face).or_insert((oi, oj, oi + nx, oj + nx));
        b.0 = b.0.min(oi);
        b.1 = b.1.min(oj);
        b.2 = b.2.max(oi + nx);
        b.3 = b.3.max(oj + nx);
    }
    let cur = crate::nomotheke::SRC_HASH;
    let mut out: Vec<ErosionCohort> = per
        .into_iter()
        .map(|((src, level), (counts, bounds))| ErosionCohort {
            is_current: src == cur,
            epochs: counts.keys().copied().collect(),
            tiles: counts.values().copied().collect(),
            bounds,
            level,
            src,
        })
        .collect();
    out.sort_by_key(|c| (c.epochs.len() == 1, !c.is_current, std::cmp::Reverse(c.epochs.len())));
    out
}

/// The honesty block: epistemic state the registry **already declares** and that
/// has never reached a viewer.
///
/// Every value here is read from `nomotheke` or from root metadata — nothing is
/// computed or judged in this function. That is the point: the physics tier, the
/// derived (weakest-link) tier, and the provisional flag are all declared
/// upstream and were only ever visible in `vivarium status`, which nobody has
/// open while watching a build.
pub fn honesty_block(roots: &[RootEntry]) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    let reports = interior(roots);
    let provisional = roots.iter().filter(|r| r.provisional).count();

    let _ = writeln!(s, "{:<20} {:>9} {:>7}   interior (distinct time-index)", "nomos", "B dcl/drv", "roots");
    for r in &reports {
        let b = match crate::nomotheke::lookup(&r.nomos) {
            Some(d) => format!("{}/{}", d.physics.letter(), d.derived_physics().letter()),
            None => "?/?".into(),
        };
        let interior = match (r.axis, r.distinct) {
            (None, _) => "— no time-index in key".to_string(),
            (Some(a), 0) => format!("{a} declared but absent from keys"),
            (Some(a), 1) => format!("1 ({a}) — endpoint only, no interior"),
            (Some(a), n) => format!("{n} ({a})"),
        };
        let _ = writeln!(s, "{:<20} {b:>9} {:>7}   {interior}", r.nomos, r.roots);
    }
    if provisional > 0 {
        let _ = writeln!(
            s,
            "⚠ {provisional} provisional root(s) — written under waived flux admission; not lawful in vivia evidence"
        );
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(key: &str) -> RootEntry {
        RootEntry { key: key.into(), object: "o".into(), provisional: false }
    }

    #[test]
    fn key_fields_parse_out_of_a_complete_key() {
        let k = "erosion-tile@erosion-v|seed=7|face=2|level=6|oi=0|oj=64|nx=64|epochs=20";
        assert_eq!(key_field(k, "face"), Some("2"));
        assert_eq!(key_field(k, "level"), Some("6"));
        assert_eq!(key_field(k, "oj"), Some("64"));
        assert_eq!(key_field(k, "epochs"), Some("20"));
        assert_eq!(key_field(k, "nope"), None);
    }

    #[test]
    fn coverage_ladders_by_deepest_nomos() {
        let src = crate::nomotheke::SRC_HASH;
        let roots = vec![
            root(&format!("initial-topography@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|src={src}")),
            root(&format!("erosion-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|epochs=20|src={src}")),
            root(&format!("water-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|eepochs=20|steps=200|src={src}")),
            // a second face with only initial-topography reached
            root(&format!("initial-topography@v|seed=0|face=1|level=6|oi=0|oj=0|nx=64|src={src}")),
        ];
        let cov = Coverage::parse(&roots);
        assert_eq!(cov.level, 6);
        assert_eq!(cov.nx, 64);
        assert_eq!(cov.state(0, 0, 0), BuildState::Watered, "deepest nomos wins");
        assert_eq!(cov.state(1, 0, 0), BuildState::InitialTopography, "initial-topography-only face");
        assert_eq!(cov.state(5, 0, 0), BuildState::Unbuilt, "untouched face");
        assert_eq!(cov.built_tiles(), 2, "two origins have any root at all");
        assert_eq!(cov.watered[&(0, 0, 0)], (20, 200), "bed selector and water clock kept apart");
    }

    #[test]
    fn a_root_from_another_source_tree_is_counted_but_never_laddered() {
        // The instrument-contradicts-itself guard. A root whose `src=` differs
        // cannot be pulled by this binary, so every surface assembled here falls
        // back to the prior where it lies. Laddering it would paint "watered"
        // over terrain the renderer is drawing from the uncarved prior.
        let cur = crate::nomotheke::SRC_HASH;
        let roots = vec![
            root(&format!("erosion-tile@v|face=0|level=6|oi=0|oj=0|nx=64|epochs=40|src={cur}")),
            root("erosion-tile@v|face=1|level=6|oi=0|oj=0|nx=64|epochs=40|src=deadbeef"),
        ];
        let cov = Coverage::parse(&roots);
        assert_eq!(cov.state(0, 0, 0), BuildState::Eroded, "current source ladders");
        assert_eq!(cov.state(1, 0, 0), BuildState::Unbuilt, "stale source does not");
        assert!(cov.flags_at_cell(1, 0, 0).stale, "and it is flagged, not merely absent");
        assert_eq!(cov.stale_only_tiles(), 1);
        assert_eq!(cov.built_tiles(), 1, "only the readable origin counts as built");
    }

    #[test]
    fn an_origin_rebuilt_under_a_new_source_is_not_reported_stale() {
        // A store accumulates roots across rebuilds, so after any source edit and
        // rebuild almost every origin carries BOTH a stale and a current root.
        // Counting the raw stale set reported "384 readable tiles" and "384 stale
        // tiles" on one screen — the defect this pins.
        let cur = crate::nomotheke::SRC_HASH;
        let roots = vec![
            root("erosion-tile@v|face=0|level=6|oi=0|oj=0|nx=64|epochs=40|src=oldhash"),
            root(&format!("erosion-tile@v|face=0|level=6|oi=0|oj=0|nx=64|epochs=40|src={cur}")),
        ];
        let cov = Coverage::parse(&roots);
        assert_eq!(cov.state(0, 0, 0), BuildState::Eroded);
        assert_eq!(cov.stale_only_tiles(), 0, "it was rebuilt; nothing is owed here");
        assert!(!cov.flags_at_cell(0, 0, 0).stale);
    }

    #[test]
    fn coverage_maps_a_cell_to_its_tile() {
        // The explorer paints per CELL, not per tile origin; the mapping has to
        // be in the shared census or each renderer re-derives it slightly
        // differently at the tile boundary.
        let roots = vec![root(&format!(
            "erosion-tile@v|seed=0|face=3|level=9|oi=128|oj=64|nx=64|epochs=40|src={}",
            crate::nomotheke::SRC_HASH
        ))];
        let cov = Coverage::parse(&roots);
        assert_eq!(cov.state_at_cell(3, 128, 64), BuildState::Eroded, "origin cell");
        assert_eq!(cov.state_at_cell(3, 191, 127), BuildState::Eroded, "last cell in the tile");
        assert_eq!(cov.state_at_cell(3, 192, 64), BuildState::Unbuilt, "one cell past the tile");
    }

    #[test]
    fn interior_counts_distinct_time_indices_per_nomos() {
        let roots = vec![
            root("erosion-tile@v|face=0|epochs=40"),
            root("erosion-tile@v|face=1|epochs=40"),
            root("erosion-tile@v|face=0|epochs=80"),
            root("initial-topography@v|face=0"),
        ];
        let by: BTreeMap<_, _> = interior(&roots).into_iter().map(|r| (r.nomos.clone(), r)).collect();
        assert_eq!(by["erosion-tile"].distinct, 2, "two distinct epoch counts, four roots");
        assert_eq!(by["erosion-tile"].roots, 3);
        assert_eq!(by["initial-topography"].distinct, 0);
        assert_eq!(by["initial-topography"].axis, None, "no time-index declared");
    }

    #[test]
    fn water_time_index_is_steps_not_the_eroded_bed_it_settles_on() {
        // The near-miss guard. `eepochs` selects WHICH eroded bed (a dependency
        // under #form-depend-by-key-never-latest); `steps` is water's own clock.
        // Reading eepochs as the clock would report a rich interior for a nomos
        // that has none.
        let roots = vec![
            root("water-tile@v|eepochs=40|steps=200"),
            root("water-tile@v|eepochs=80|steps=200"),
        ];
        let r = interior(&roots).pop().unwrap();
        assert_eq!(r.axis, Some("steps"));
        assert_eq!(r.distinct, 1, "two eroded beds, ONE water clock value — no interior");
    }

    #[test]
    fn frames_never_split_a_timestamp_group() {
        // Landings indistinguishable in the only available ordering must stay
        // together; splitting them would invent an order the store cannot support.
        let t0 = SystemTime::UNIX_EPOCH;
        let t1 = t0 + std::time::Duration::from_secs(1);
        let mk = |k: &str, at| Landing { root: root(k), at };
        let ls = vec![mk("a", t0), mk("b", t0), mk("c", t0), mk("d", t1)];
        for max in 1..=6 {
            for &b in &frame_bounds(&ls, max) {
                assert!(b == 3 || b == 4, "frame boundary {b} split a timestamp group");
            }
        }
        assert_eq!(*frame_bounds(&ls, 9).last().unwrap(), 4, "last frame is the whole census");
    }

    #[test]
    fn landings_are_deterministic_under_equal_timestamps() {
        let t = SystemTime::UNIX_EPOCH;
        let mut ls = vec![
            Landing { root: root("b"), at: t },
            Landing { root: root("a"), at: t },
        ];
        ls.sort_by(|x, y| x.at.cmp(&y.at).then_with(|| x.root.key.cmp(&y.root.key)));
        assert_eq!(ls[0].root.key, "a", "ties break by key, not by directory order");
    }

    #[test]
    fn empty_store_replays_to_nothing_rather_than_panicking() {
        assert!(frame_bounds(&[], 10).is_empty());
        assert!(interior(&[]).is_empty());
        assert!(erosion_cohorts(&[]).is_empty());
    }

    /// The property a world-time scrub rests on: **stages never cross source
    /// trees**. Two cohorts carved under different digests are two worlds, and if
    /// this grouping ever merged them a scrub would draw the difference between
    /// two kernels as though it were elapsed world-time — indistinguishable, to
    /// the eye, from geology.
    #[test]
    fn settle_histories_are_grouped_by_source_tree_and_never_merged() {
        let cur = crate::nomotheke::SRC_HASH;
        let old = "0000000000000000";
        let tile = |src: &str, e: u32, oi: u32| {
            root(&format!(
                "erosion-tile@v|src={src}|seed=0|face=0|level=9|oi={oi}|oj=0|nx=64|epochs={e}"
            ))
        };
        let mut roots = Vec::new();
        for e in [5u32, 10, 15] {
            for oi in [0u32, 64] {
                roots.push(tile(cur, e, oi));
                // The residual sibling: metadata, and it must not be counted as a
                // stage or every epoch would appear to hold twice its tiles.
                roots.push(root(&format!(
                    "erosion-tile@v|src={cur}|seed=0|face=0|level=9|oi={oi}|oj=0|nx=64|epochs={e}|aspect=stage-residual"
                )));
            }
        }
        roots.push(tile(old, 40, 0));
        roots.push(tile(old, 60, 0));

        let cohorts = erosion_cohorts(&roots);
        assert_eq!(cohorts.len(), 2, "two source trees, two histories");
        let c = &cohorts[0];
        assert!(c.is_current, "the current source's chain is offered first");
        assert_eq!(c.epochs, vec![5, 10, 15]);
        assert_eq!(c.tiles, vec![2, 2, 2], "residual siblings are metadata, not stages");
        assert!(c.is_square());
        assert_eq!(c.level, 9);
        assert_eq!(cohorts[1].src, old);
        assert_eq!(cohorts[1].epochs, vec![40, 60]);
    }

    /// A cohort with one materialized time-index has no interior, and ordering
    /// must put it behind any chain that does — otherwise a store whose current
    /// source is endpoint-only would offer a one-frame "scrub" while a real
    /// history sat unoffered.
    #[test]
    fn endpoint_only_cohorts_sort_behind_real_chains() {
        let cur = crate::nomotheke::SRC_HASH;
        let mut roots = vec![root(&format!(
            "erosion-tile@v|src={cur}|seed=0|face=0|level=9|oi=0|oj=0|nx=64|epochs=40"
        ))];
        for e in [5u32, 10, 15] {
            roots.push(root(&format!(
                "erosion-tile@v|src=deadbeefdeadbeef|seed=0|face=0|level=9|oi=0|oj=0|nx=64|epochs={e}"
            )));
        }
        let cohorts = erosion_cohorts(&roots);
        assert_eq!(cohorts[0].len(), 3, "the chain with an interior is offered first");
        assert!(!cohorts[0].is_current, "even when it is not this binary's source");
        assert!(cohorts[1].is_current);
    }

    /// **Level is chain identity.** A world holding a coarse global sweep and a
    /// fine beacon patch under the SAME source tree holds two histories, not one.
    /// Merged, their epoch ladders interleave over wildly unequal footprints, and
    /// a scrub stepping that ladder would show a region appearing and vanishing —
    /// a coverage animation that reads, to an eye watching for erosion, as
    /// geology. (Live instance 2026-07-28: global L9 at epochs 5…40 beside an L13
    /// beacon at epochs 10…300.)
    #[test]
    fn a_coarse_sweep_and_a_fine_beacon_are_two_chains_not_one() {
        let cur = crate::nomotheke::SRC_HASH;
        let mut roots = Vec::new();
        for e in [5u32, 10, 15] {
            for oi in [0u32, 64] {
                roots.push(root(&format!(
                    "erosion-tile@v|src={cur}|seed=0|face=0|level=9|oi={oi}|oj=0|nx=64|epochs={e}"
                )));
            }
        }
        for e in [10u32, 20, 30, 40] {
            roots.push(root(&format!(
                "erosion-tile@v|src={cur}|seed=0|face=1|level=13|oi=640|oj=5376|nx=64|epochs={e}"
            )));
        }
        let cohorts = erosion_cohorts(&roots);
        assert_eq!(cohorts.len(), 2, "one source tree, two levels, two chains");
        let fine = cohorts.iter().find(|c| c.level == 13).expect("the beacon chain");
        let coarse = cohorts.iter().find(|c| c.level == 9).expect("the global chain");
        assert_eq!(fine.epochs, vec![10, 20, 30, 40], "ladders never interleave");
        assert_eq!(coarse.epochs, vec![5, 10, 15]);
        assert_eq!(cohorts[0].level, 13, "the deeper chain is offered first");

        // The extent is census, not guesswork: a viewer cannot find a 64-cell
        // window in an 8192-cell face by orbiting.
        assert!(!fine.is_global(), "a beacon patch is a window, not a face");
        assert_eq!(fine.centre(), Some((1, 640 + 32, 5376 + 32)));
        assert_eq!(fine.span_cells(), 64);
        assert!(coarse.is_global() || coarse.span_cells() == 128);
    }

    /// A ragged chain — stages covering different tile counts — is a frame that
    /// is one world-moment only where its stage reaches, and a scrub has to be
    /// able to say so rather than presenting a partial moment as a whole one.
    #[test]
    fn a_ragged_chain_reports_itself_as_ragged() {
        let cur = crate::nomotheke::SRC_HASH;
        let mut roots = Vec::new();
        for (e, tiles) in [(5u32, 2u32), (10, 1)] {
            for oi in 0..tiles {
                roots.push(root(&format!(
                    "erosion-tile@v|src={cur}|seed=0|face=0|level=9|oi={oi}|oj=0|nx=64|epochs={e}"
                )));
            }
        }
        let c = &erosion_cohorts(&roots)[0];
        assert_eq!(c.tiles, vec![2, 1]);
        assert!(!c.is_square(), "unequal coverage across stages is visible, not smoothed");
    }
}
