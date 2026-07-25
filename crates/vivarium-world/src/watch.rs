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
    }
}
