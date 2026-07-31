//! **Hints** — suggestions to the operator (or agent reading a dump), not a job
//! queue and not "what the binary is about to do."
//!
//! Call sites inject/revoke like emitting a log line. Active hints stack for HUD
//! / status / capture. The CLI log is **edge-triggered**: a line only when a
//! hint is newly set, its text changes, or it is revoked — each line carries
//! `file:line` of the inject/revoke site so bad hint logic is greppable.
//!
//! Spelling: **revocation** (noun), **revoke** (verb).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::panic::Location;
use std::sync::{Mutex, OnceLock};

/// One active suggestion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hint {
    pub id: String,
    pub text: String,
    /// `file:line` of the last successful [`set`].
    pub at: String,
}

/// Edge-triggered log record (set or revoke).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HintEvent {
    Set {
        id: String,
        text: String,
        at: String,
    },
    /// Hint left the active set. `was` is the text that had been shown; `at` is
    /// the **clear** site (not the original set site).
    Revoked {
        id: String,
        was: String,
        at: String,
    },
}

struct Inner {
    active: BTreeMap<String, Hint>,
    log: Vec<HintEvent>,
}

fn inner() -> &'static Mutex<Inner> {
    static H: OnceLock<Mutex<Inner>> = OnceLock::new();
    H.get_or_init(|| {
        Mutex::new(Inner {
            active: BTreeMap::new(),
            log: Vec::new(),
        })
    })
}

fn loc(caller: &Location<'_>) -> String {
    // Prefer basename so capture/HUD lines stay short; full path still unique enough
    // within the repo for `rg`.
    let file = caller.file();
    let base = file.rsplit(['/', '\\']).next().unwrap_or(file);
    format!("{base}:{}", caller.line())
}

/// Insert or refresh a hint. Logs only on first set or when `text` changes.
#[track_caller]
pub fn set(id: impl Into<String>, text: impl Into<String>) {
    set_at(id, text, Location::caller());
}

pub fn set_at(id: impl Into<String>, text: impl Into<String>, caller: &Location<'_>) {
    let id = id.into();
    let text = text.into();
    let at = loc(caller);
    let mut g = inner().lock().unwrap_or_else(|e| e.into_inner());
    let changed = match g.active.get(&id) {
        Some(h) => h.text != text,
        None => true,
    };
    if !changed {
        // Same id+text: still refresh `at`? No — keep first/last set origin stable
        // until text changes so the log points at the logic that introduced it.
        return;
    }
    g.active.insert(
        id.clone(),
        Hint {
            id: id.clone(),
            text: text.clone(),
            at: at.clone(),
        },
    );
    g.log.push(HintEvent::Set { id, text, at });
}

/// Drop a hint if present. Logs a revocation only when something was cleared.
#[track_caller]
pub fn clear(id: &str) {
    clear_at(id, Location::caller());
}

pub fn clear_at(id: &str, caller: &Location<'_>) {
    let at = loc(caller);
    let mut g = inner().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(was) = g.active.remove(id) {
        g.log.push(HintEvent::Revoked {
            id: id.to_string(),
            was: was.text,
            at,
        });
    }
}

/// Snapshot of currently unrevoked hints (stable id order).
pub fn active() -> Vec<Hint> {
    let g = inner().lock().unwrap_or_else(|e| e.into_inner());
    g.active.values().cloned().collect()
}

/// Take and clear the edge log (sets + revocations since last drain).
pub fn drain_log() -> Vec<HintEvent> {
    let mut g = inner().lock().unwrap_or_else(|e| e.into_inner());
    std::mem::take(&mut g.log)
}

/// Format one event for stderr / builder logs.
pub fn format_event(ev: &HintEvent) -> String {
    match ev {
        HintEvent::Set { id, text, at } => {
            format!("[hint] set {id} @ {at} — Hint: {text}")
        }
        HintEvent::Revoked { id, was, at } => {
            format!("[hint] revoked {id} @ {at} — was: {was}")
        }
    }
}

/// Drain the edge log to stderr (and return the same events for callers that
/// also want them).
pub fn flush_log_to_stderr() -> Vec<HintEvent> {
    let events = drain_log();
    for ev in &events {
        eprintln!("{}", format_event(ev));
    }
    events
}

/// Active hints as a HUD/status block (ASCII; one line per hint). Empty if none.
pub fn format_active() -> String {
    let hints = active();
    if hints.is_empty() {
        return String::new();
    }
    let mut s = String::new();
    for h in &hints {
        let _ = writeln!(s, "Hint: {}", h.text);
    }
    s
}

/// Active hints as a capture/udon fragment (includes id + origin).
pub fn format_active_udon() -> String {
    let hints = active();
    let mut s = String::new();
    if hints.is_empty() {
        let _ = writeln!(s, "hints: []");
        return s;
    }
    let _ = writeln!(s, "hints:");
    for h in &hints {
        let _ = writeln!(s, "  - id: {:?}", h.id);
        let _ = writeln!(s, "    text: {:?}", h.text);
        let _ = writeln!(s, "    at: {:?}", h.at);
    }
    s
}

/// Sync the standard erosion-bed hints from a census.
///
/// Mutual-exclusive ids: `erosion-stale-src` | `erosion-none` | `erosion-readable`.
/// Safe to call every frame — only edges hit the log.
#[track_caller]
pub fn sync_erosion_bed(fresh: usize, stale: usize, src_short: &str) {
    let caller = Location::caller();
    if fresh == 0 && stale > 0 {
        clear_at("erosion-none", caller);
        clear_at("erosion-readable", caller);
        set_at(
            "erosion-stale-src",
            format!("`vivarium build` to rebuild under this program"),
            caller,
        );
        let _ = (stale, src_short); // status lines carry counts; hint stays short
    } else if fresh == 0 {
        clear_at("erosion-stale-src", caller);
        clear_at("erosion-readable", caller);
        set_at(
            "erosion-none",
            "`vivarium build` to continue erosion-tile root builds",
            caller,
        );
    } else {
        clear_at("erosion-stale-src", caller);
        clear_at("erosion-none", caller);
        set_at(
            "erosion-readable",
            format!(
                "eroded land now visible: {fresh} readable (src {src_short}); older ignored: {stale}"
            ),
            caller,
        );
    }
}

/// Test helper: wipe process-global state (not for production paths).
#[cfg(test)]
pub fn reset_for_test() {
    let mut g = inner().lock().unwrap_or_else(|e| e.into_inner());
    g.active.clear();
    g.log.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Process-global store — tests must not interleave.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn set_is_edge_triggered_and_revoke_logs() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_for_test();
        set("a", "one");
        set("a", "one"); // no second log
        set("a", "two"); // text change → log
        clear("a");
        clear("a"); // no-op
        let log = drain_log();
        assert_eq!(log.len(), 3, "{log:?}");
        match &log[0] {
            HintEvent::Set { id, text, .. } => {
                assert_eq!(id, "a");
                assert_eq!(text, "one");
            }
            _ => panic!("expected set"),
        }
        match &log[1] {
            HintEvent::Set { text, .. } => assert_eq!(text, "two"),
            _ => panic!("expected set"),
        }
        match &log[2] {
            HintEvent::Revoked { id, was, .. } => {
                assert_eq!(id, "a");
                assert_eq!(was, "two");
            }
            _ => panic!("expected revoke"),
        }
        assert!(active().is_empty());
    }

    #[test]
    fn erosion_sync_switches_ids() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_for_test();
        sync_erosion_bed(0, 10, "deadbeef");
        assert_eq!(active().len(), 1);
        assert_eq!(active()[0].id, "erosion-stale-src");
        let _ = drain_log(); // drop first edges
        sync_erosion_bed(3, 10, "deadbeef");
        let a = active();
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].id, "erosion-readable");
        let log = drain_log();
        assert!(
            log.iter()
                .any(|e| matches!(e, HintEvent::Revoked { id, .. } if id == "erosion-stale-src")),
            "{log:?}"
        );
        assert!(
            log.iter()
                .any(|e| matches!(e, HintEvent::Set { id, .. } if id == "erosion-readable")),
            "{log:?}"
        );
    }
}
