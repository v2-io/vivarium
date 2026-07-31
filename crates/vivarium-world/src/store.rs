//! Content-addressed, memoized store — the world's portable saved state.
//! Claim home: `#form-store-as-save` (save ≡ memo store; invalidation vs
//! eviction; regenerable/irreducible frame). Teaching source:
//! `doc/design/DESIGN-REDUX.md` §13; architecture map: `core (ARCHITECTURE graduated)` §5.
//!
//! **Domain-neutral by construction.** Keys and values here are opaque; the
//! nomos above (`query.rs`) give them meaning. Nothing in this module knows
//! about terrain, columns, or `CellId`s — a spatial field, a geochemical
//! reservoir, a food-web state-vector, or an agent population all persist
//! through this *same* interface. That neutrality is the framework's invariant
//! (`core (ARCHITECTURE graduated)` domain-fixation guard), not a courtesy.
//!
//! Shape (git-repo-like): `objects/<value-hash>` hold immutable bytes;
//! `roots/<key-hash>` point at the object a complete key resolves to. Content-
//! addressing buys dedup (two keys computing identical bytes share one object)
//! and makes eviction cost only a recompute, never correctness. Copy the
//! directory → the world moves.
//!
//! MVP scope, flagged honestly:
//! - hash is 64-bit FNV-1a (fast, dependency-free) — fine for a demo's object
//!   count, **not** collision-safe at scale; swap to blake3 before this holds
//!   anything we cannot recompute.
//! - no GC/eviction, no full manifest, no run-mode canon-root guard yet.
//! - **provisional roots** (third line on the root file) mark waived flux
//!   admission (`--allow-unmet`); census and `status` surface them. This is
//!   root metadata, not a key field — same complete key, different honesty.
//! - **under-keying is the one unsafe failure** (§12 — a stale memo then
//!   *lies*), so callers must fold *every* input into the [`Key`].

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// 64-bit FNV-1a. MVP-grade content hash (see the module note on collisions).
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn hex(h: u64) -> String {
    format!("{h:016x}")
}

/// A *complete* content-addressed key: a canonical string of everything that
/// affects the value — nomos name + version + every input. Build it with
/// [`Key::field`] per input; omitting one is the under-keying trap (§12), so
/// fold in all of them (upstream hashes, params, seed, resolution, time).
#[derive(Clone, Debug)]
pub struct Key(String);

impl Key {
    /// Start a key for the nomos named `nomos` at `version`. (Version is a
    /// constant for the
    /// MVP; it graduates to a source-derived hash — coarse-enough-to-cover-deps
    /// first, IR-normalized only if the build-chain makes it clean — later.)
    pub fn new(nomos: &str, version: &str) -> Self {
        Key(format!("{nomos}@{version}"))
    }

    /// Fold every direct dependency's name+version into the key so a dep bump
    /// invalidates consumers ( #form-complete-content-addressed-key ).
    pub fn with_dep_versions(mut self, nomos: &crate::nomotheke::NomosDecl) -> Self {
        for d in nomos.deps {
            self = self.field(d.name, d.version);
        }
        self
    }

    /// Fold one input into the key. Chainable.
    pub fn field(mut self, name: &str, value: impl std::fmt::Display) -> Self {
        use std::fmt::Write;
        let _ = write!(self.0, "|{name}={value}");
        self
    }

    /// The canonical key string (also what gets hashed).
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn hash(&self) -> u64 {
        fnv1a(self.0.as_bytes())
    }
}

/// Options for a memo put. Flags are **root metadata**, not key inputs: the same
/// complete key may be lawful or provisional depending on builder admission.
#[derive(Clone, Debug, Default)]
pub struct PutOpts {
    /// Written under waived flux admission (`--allow-unmet`). Census and status
    /// must surface this; provisional roots are not lawful *in vivia* evidence
    /// ( #form-builder-admission · #form-flux-web ).
    pub provisional: bool,
    /// The **witnessed read-set**: the complete keys this memo's compute pulled
    /// as dependencies, recorded by the compute frame ( `query::World`, the
    /// under-keyed-dependency mechanism). `None` = the compute path is not yet
    /// wired for recording (pre-mechanism, or a path documented as unwired);
    /// `Some(vec![])` = recorded and genuinely read nothing. Root metadata,
    /// never a key input — two runs of a lawful compute record the same set,
    /// and an audit that finds two cohorts disagreeing on a read-set has found
    /// an under-keyed dependency ( #form-depend-by-key-never-latest FE(4)(b) ).
    pub deps: Option<Vec<String>>,
}

/// One store root as the census instruments see it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootEntry {
    /// Canonical complete-key string (empty for pre-census format-v1 roots).
    pub key: String,
    /// Content-addressed object hash (hex).
    pub object: String,
    /// True when written under waived flux admission — not lawful evidence.
    pub provisional: bool,
    /// Witnessed read-set ([`PutOpts::deps`]): `None` if the writing path did
    /// not record one.
    pub deps: Option<Vec<String>>,
}

/// Hot listing of [`Store::roots`] — generation for same-process puts, entry
/// count for external writers (builder beside explore). Bodies are parsed once
/// per epoch; see `#disc-explorer-instrument-parity` P0.
struct RootsCache {
    /// [`Store::generation`] at the time of the scan.
    generation: u64,
    /// Non-tmp files under `roots/` when scanned (cheap external-writer probe).
    entry_count: usize,
    entries: Arc<Vec<RootEntry>>,
}

/// A filesystem-backed content-addressed store.
pub struct Store {
    objects: PathBuf,
    roots: PathBuf,
    /// Opened by a **view**: every put is refused and counted, never written.
    /// See [`Store::open_read_only`].
    read_only: bool,
    /// Puts refused because this handle is read-only — the wall as a *number*
    /// rather than a discipline. A view that displays this displays its own
    /// compliance.
    refused: AtomicUsize,
    /// Bumped on every successful put on **this** handle. Index cache epoch for
    /// same-process writers — not content-addressed truth (`#form-store-as-save`
    /// still forbids OS mtime as key validity).
    generation: AtomicU64,
    /// Parsed root listing; shared across pull/census so thrash does not re-read
    /// ~10⁵ root files per "updating view…".
    roots_cache: Mutex<Option<RootsCache>>,
}

impl Store {
    /// Open (creating if needed) a store rooted at `dir`. This is the
    /// **builder's** handle: it may write.
    pub fn open(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let objects = dir.join("objects");
        let roots = dir.join("roots");
        fs::create_dir_all(&objects)?;
        fs::create_dir_all(&roots)?;
        Ok(Store {
            objects,
            roots,
            read_only: false,
            refused: AtomicUsize::new(0),
            generation: AtomicU64::new(0),
            roots_cache: Mutex::new(None),
        })
    }

    /// Open a store **as a view does**: reads served normally, every [`put_with`]
    /// refused with `PermissionDenied` and counted in [`Self::refused_writes`].
    ///
    /// This is `#form-core-view-wall` FE(2) ("views obtain world state only
    /// through the sanctioned query path… may not own authoritative world state")
    /// and `#form-builder-admission` FE(1) ("explorers query the store and never
    /// author world-evolution") **enforced by construction instead of by reading
    /// the segment carefully**. The distinction it draws is the operative one: a
    /// view may *compute* a pure function of the world's identity — that is a
    /// query, and it is how a cold world still renders — but it may not
    /// **author a store citizen**, because a citizen is durable world state and
    /// authoring one makes the view a second builder.
    ///
    /// The concrete violation this closes: the globe spike's deep-time warmer (now `archive/globe-spike`)
    /// called `World::epoch_reduction`, which on a miss computes *and puts*. A
    /// view left running on an unbuilt world was silently materializing the
    /// cooling ladder — so `vivarium build` became optional magic and the store
    /// gained roots nobody's builder wrote. The same latent path exists in
    /// `vivarium watch` / `info` through `globe::render`'s `erosion_tile` pulls.
    ///
    /// Deliberately does **not** create the directories: a read-only handle on a
    /// world that does not exist yet reports an empty census rather than
    /// conjuring the shape of a vivium (`roots()` yields nothing).
    pub fn open_read_only(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        Ok(Store {
            objects: dir.join("objects"),
            roots: dir.join("roots"),
            read_only: true,
            refused: AtomicUsize::new(0),
            generation: AtomicU64::new(0),
            roots_cache: Mutex::new(None),
        })
    }

    /// Whether this handle refuses writes (a view's handle).
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// How many puts this handle has refused. `0` on a view is the wall holding;
    /// any other number names exactly how many world citizens the view tried to
    /// author, which is a finding, not a nuisance.
    pub fn refused_writes(&self) -> usize {
        self.refused.load(Ordering::Relaxed)
    }

    /// Fetch the value a complete key resolves to, or `None` on a miss.
    pub fn get(&self, key: &Key) -> Option<Vec<u8>> {
        let root = fs::read_to_string(self.roots.join(hex(key.hash()))).ok()?;
        let obj = root.lines().next()?.trim();
        self.object_bytes(obj)
    }

    /// Read an object by content hash (hex) — census → materialization without
    /// reconstructing the complete key.
    pub fn object_bytes(&self, object_hash: &str) -> Option<Vec<u8>> {
        fs::read(self.objects.join(object_hash)).ok()
    }

    /// Whether the root for `key` is tagged provisional (false if missing or untagged).
    pub fn is_provisional(&self, key: &Key) -> bool {
        let root = match fs::read_to_string(self.roots.join(hex(key.hash()))) {
            Ok(t) => t,
            Err(_) => return false,
        };
        root.lines().skip(2).any(|l| l.trim() == "provisional")
    }

    /// Store `value` under `key` as a **lawful** (non-provisional) root.
    pub fn put(&self, key: &Key, value: &[u8]) -> io::Result<()> {
        self.put_with(key, value, PutOpts::default())
    }

    /// Store `value` under `key` with root metadata (`PutOpts`).
    ///
    /// The bytes land at `objects/<value-hash>` (idempotent — re-putting
    /// identical bytes is a no-op) and `roots/<key-hash>` is pointed at them.
    /// Both writes go temp-then-rename, so a reader never sees a half-written
    /// object or root.
    ///
    /// Root file shape:
    /// ```text
    /// <object-hash>
    /// <canonical key string>
    /// [provisional]     # optional, when PutOpts.provisional
    /// [deps-recorded]   # optional, when PutOpts.deps is Some
    /// [dep <key>]*      # one per witnessed dependency read
    /// ```
    /// Line 2 makes the store *enumerable by meaning*. `provisional` is the
    /// honesty bit for waived admission ( #form-builder-admission residual A/B ).
    /// The `deps` lines are the witnessed read-set — telemetry for the
    /// under-keyed-dependency audit, never identity.
    pub fn put_with(&self, key: &Key, value: &[u8], opts: PutOpts) -> io::Result<()> {
        if self.read_only {
            // Refuse loudly in the return value and countably in the handle. The
            // caller's own `Source::Computed` already tells the truth downstream:
            // the value came from this process, not from the store.
            self.refused.fetch_add(1, Ordering::Relaxed);
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("read-only store (view handle): refused to author `{}`", key.as_str()),
            ));
        }
        let obj_name = hex(fnv1a(value));
        let obj_path = self.objects.join(&obj_name);
        if !obj_path.exists() {
            write_atomic(&obj_path, value)?;
        }
        let mut root = format!("{obj_name}\n{}", key.as_str());
        if opts.provisional {
            root.push_str("\nprovisional");
        }
        if let Some(deps) = &opts.deps {
            root.push_str("\ndeps-recorded");
            for d in deps {
                root.push_str("\ndep ");
                root.push_str(d);
            }
        }
        write_atomic(&self.roots.join(hex(key.hash())), root.as_bytes())?;
        // Listing epoch moves; drop cache so the next roots() rescans once.
        self.generation.fetch_add(1, Ordering::Release);
        if let Ok(mut guard) = self.roots_cache.lock() {
            *guard = None;
        }
        Ok(())
    }

    /// Same-process put generation (for probes / HUD). Starts at 0; increments
    /// after each successful [`put_with`] on this handle.
    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    /// Enumerate every root for the census instruments. Roots written before
    /// key-strings were recorded (format v1, pre-2026-07-10) appear with an
    /// empty key and should be counted as "unknown"; they are valid but not
    /// attributable. Missing third line ⇒ not provisional.
    ///
    /// Hot path: parses root files once per listing epoch (generation + entry
    /// count). Callers that only need to iterate should prefer
    /// [`roots_shared`] to avoid cloning ~10⁵ entries.
    pub fn roots(&self) -> io::Result<Vec<RootEntry>> {
        Ok(self.roots_shared()?.as_ref().clone())
    }

    /// Shared root listing for interactive instruments — same epoch rules as
    /// [`roots`], without cloning the full vector on every call.
    ///
    /// **Hot path is generation-only** (same-process puts). A full `roots/`
    /// readdir every call is itself O(archive) and re-introduces thrash at ~10⁵
    /// entries; external writers (builder beside explore) must call
    /// [`roots_invalidate_if_external`] on a throttle, then this again.
    pub fn roots_shared(&self) -> io::Result<Arc<Vec<RootEntry>>> {
        let gen = self.generation.load(Ordering::Acquire);
        {
            let guard = self.roots_cache.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(cache) = guard.as_ref() {
                if cache.generation == gen {
                    return Ok(Arc::clone(&cache.entries));
                }
            }
        }
        let entries = Arc::new(self.scan_roots()?);
        let gen_after = self.generation.load(Ordering::Acquire);
        // Only publish if no put raced during the scan.
        if gen_after == gen {
            let entry_count = entries.len();
            let mut guard = self.roots_cache.lock().unwrap_or_else(|e| e.into_inner());
            *guard = Some(RootsCache {
                generation: gen,
                entry_count,
                entries: Arc::clone(&entries),
            });
        }
        Ok(entries)
    }

    /// Drop the hot listing if the number of root files no longer matches the
    /// cached census (another process put). Returns `true` when the cache was
    /// cleared. Cost is one `read_dir` of names — not body parse. Call on a
    /// throttle from live-watch instruments, not every mesh pull.
    pub fn roots_invalidate_if_external(&self) -> io::Result<bool> {
        let count = self.roots_entry_count()?;
        let mut guard = self.roots_cache.lock().unwrap_or_else(|e| e.into_inner());
        match guard.as_ref() {
            Some(cache) if cache.entry_count != count => {
                *guard = None;
                Ok(true)
            }
            Some(_) => Ok(false),
            None => Ok(false),
        }
    }

    /// Count non-tmp root files without reading bodies.
    fn roots_entry_count(&self) -> io::Result<usize> {
        if !self.roots.is_dir() {
            return Ok(0);
        }
        let mut n = 0usize;
        for entry in fs::read_dir(&self.roots)? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "tmp") {
                continue;
            }
            n += 1;
        }
        Ok(n)
    }

    fn scan_roots(&self) -> io::Result<Vec<RootEntry>> {
        let mut out = Vec::new();
        if !self.roots.is_dir() {
            // A read-only handle on a world that was never built: no roots, not an
            // error. The empty census is the honest answer and every instrument
            // downstream already renders it as "nothing built yet".
            return Ok(out);
        }
        for entry in fs::read_dir(&self.roots)? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "tmp") {
                continue;
            }
            let text = fs::read_to_string(&path)?;
            let mut lines = text.lines();
            let object = lines.next().unwrap_or("").trim().to_string();
            let key = lines.next().unwrap_or("").trim().to_string();
            let mut provisional = false;
            let mut deps: Option<Vec<String>> = None;
            for l in lines {
                let l = l.trim();
                if l == "provisional" {
                    provisional = true;
                } else if l == "deps-recorded" {
                    deps.get_or_insert_with(Vec::new);
                } else if let Some(d) = l.strip_prefix("dep ") {
                    deps.get_or_insert_with(Vec::new).push(d.to_string());
                }
            }
            out.push(RootEntry {
                key,
                object,
                provisional,
                deps,
            });
        }
        Ok(out)
    }
}

/// Write via a sibling temp file + rename (atomic on a single filesystem).
///
/// The temp name must be **unique per writer**, not just per target: concurrent
/// puts of *identical* bytes under distinct keys share one object path, and with
/// a single shared `.tmp` the rename losers hit NotFound and abort `put` before
/// the root lands — the memo silently evaporates (self-healing by recompute, per
/// the module's eviction guarantee, but a wasted recompute every run until a
/// solo put wins). Found live by the globe view (6 parallel face pulls over a
/// byte-identical-per-face world → 3 of 6 roots dropped); a 6-writer probe
/// dropped 5 of 6 nearly every round. pid + a process-wide counter make writers
/// collision-free; the final rename stays atomic, and concurrent winners are
/// interchangeable because the content is identical by construction. The name
/// still *ends* in `.tmp` so [`Store::roots`]'s census filter skips strays.
fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".{}.{}.tmp", std::process::id(), SEQ.fetch_add(1, Ordering::Relaxed)));
    let tmp = path.with_file_name(name);
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("vivarium-store-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn miss_then_hit_roundtrip() {
        let dir = tmpdir("roundtrip");
        let s = Store::open(&dir).unwrap();
        let k = Key::new("initial-topography", "v0")
            .field("face", 2)
            .field("level", 19)
            .field("oi", 100)
            .field("oj", 200);
        assert!(s.get(&k).is_none(), "a cold key must miss");
        s.put(&k, b"hello-world").unwrap();
        assert_eq!(s.get(&k).as_deref(), Some(&b"hello-world"[..]));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_view_handle_cannot_author_a_citizen() {
        // The core/view wall as a test rather than as a paragraph
        // ( #form-core-view-wall FE(2), #form-builder-admission FE(1) ). A view
        // reads everything the builder wrote and adds nothing to the world.
        let dir = tmpdir("readonly");
        let k = Key::new("initial-topography", "v0").field("tile", 7);
        {
            let builder = Store::open(&dir).unwrap();
            builder.put(&k, b"built").unwrap();
        }
        let view = Store::open_read_only(&dir).unwrap();
        assert_eq!(view.get(&k).as_deref(), Some(&b"built"[..]), "a view reads normally");

        let fresh = Key::new("erosion-tile", "v0").field("tile", 8);
        let err = view.put(&fresh, b"view-authored").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(view.refused_writes(), 1, "the refusal is counted, not merely returned");
        assert!(view.get(&fresh).is_none(), "nothing landed");
        assert_eq!(
            Store::open(&dir).unwrap().roots().unwrap().len(),
            1,
            "the world has exactly the citizens its BUILDER wrote"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_view_on_an_unbuilt_world_reports_an_empty_census_rather_than_creating_one() {
        // Opening a view must not conjure the shape of a vivium. `vivarium
        // explore` on a path with nothing in it shows "nothing built", and the
        // directory is still untouched afterwards.
        let dir = tmpdir("readonly-absent");
        let view = Store::open_read_only(&dir).unwrap();
        assert!(view.roots().unwrap().is_empty());
        assert!(!dir.exists(), "a read-only open created directories");
    }

    #[test]
    fn persists_across_reopen() {
        // The load-bearing property: the store IS the save.
        let dir = tmpdir("reopen");
        let k = Key::new("initial-topography", "v0").field("tile", 42);
        {
            let s = Store::open(&dir).unwrap();
            s.put(&k, &[1, 2, 3, 4]).unwrap();
        }
        let s2 = Store::open(&dir).unwrap();
        assert_eq!(s2.get(&k).as_deref(), Some(&[1u8, 2, 3, 4][..]));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn dedup_identical_values_share_one_object() {
        let dir = tmpdir("dedup");
        let s = Store::open(&dir).unwrap();
        s.put(&Key::new("a", "v0").field("x", 1), b"same").unwrap();
        s.put(&Key::new("b", "v0").field("y", 2), b"same").unwrap();
        let n = fs::read_dir(dir.join("objects")).unwrap().count();
        assert_eq!(n, 1, "two keys, identical bytes → one content-addressed object");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn roots_enumerate_by_meaning() {
        // The census property: what exists is answerable, with the canonical
        // key string attached — the substrate of every instrument.
        let dir = tmpdir("census");
        let s = Store::open(&dir).unwrap();
        s.put(&Key::new("initial-topography", "v0").field("level", 7), b"a").unwrap();
        s.put(&Key::new("erosion", "v0").field("level", 9), b"b").unwrap();
        let roots = s.roots().unwrap();
        assert_eq!(roots.len(), 2);
        assert!(roots.iter().any(|r| {
            r.key.starts_with("initial-topography@v0") && r.key.contains("level=7") && !r.provisional
        }));
        assert!(roots.iter().any(|r| r.key.starts_with("erosion@v0") && r.key.contains("level=9")));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn provisional_flag_survives_census_and_can_be_cleared() {
        // Waived admission must leave a durable mark (de-novo residual A/B);
        // a later lawful put must not leave the lie standing.
        let dir = tmpdir("provisional");
        let s = Store::open(&dir).unwrap();
        let k = Key::new("erosion-tile", "v0").field("level", 7);
        s.put_with(&k, b"waived", PutOpts { provisional: true, deps: None }).unwrap();
        assert!(s.is_provisional(&k));
        let roots = s.roots().unwrap();
        assert_eq!(roots.len(), 1);
        assert!(roots[0].provisional);
        assert!(roots[0].key.starts_with("erosion-tile@v0"));
        s.put(&k, b"lawful").unwrap();
        assert!(!s.is_provisional(&k));
        assert_eq!(s.get(&k).as_deref(), Some(&b"lawful"[..]));
        assert!(!s.roots().unwrap()[0].provisional);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn concurrent_identical_puts_all_land_their_roots() {
        // The dedup path under contention: six writers, one value, six keys.
        // Every root must land — pre-fix, the shared tmp name made the rename
        // losers abort before their root was written (typically 5 of 6 dropped;
        // first seen live as the globe view's parallel face pulls losing memos).
        let dir = tmpdir("race");
        let s = Store::open(&dir).unwrap();
        let bytes = vec![0xABu8; 65536];
        std::thread::scope(|scope| {
            for i in 0..6 {
                let (s, bytes) = (&s, &bytes);
                scope.spawn(move || s.put(&Key::new("probe", "v0").field("i", i), bytes).unwrap());
            }
        });
        for i in 0..6 {
            let k = Key::new("probe", "v0").field("i", i);
            assert_eq!(s.get(&k).as_deref(), Some(&bytes[..]), "root {i} was dropped by the tmp race");
        }
        assert_eq!(
            fs::read_dir(dir.join("objects")).unwrap().count(),
            1,
            "identical bytes still dedup to one object"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn distinct_complete_keys_never_alias() {
        // The under-keying guard, positively: fold every input in and neighbours
        // stay distinct.
        let dir = tmpdir("keys");
        let s = Store::open(&dir).unwrap();
        let k1 = Key::new("initial-topography", "v0").field("oi", 100).field("oj", 200);
        let k2 = Key::new("initial-topography", "v0").field("oi", 100).field("oj", 201);
        s.put(&k1, b"tileA").unwrap();
        s.put(&k2, b"tileB").unwrap();
        assert_eq!(s.get(&k1).as_deref(), Some(&b"tileA"[..]));
        assert_eq!(s.get(&k2).as_deref(), Some(&b"tileB"[..]));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn roots_listing_is_hot_within_a_generation() {
        // P0: repeated census must not re-parse every root body. Shared Arc
        // identity is the probe that the index stayed warm.
        let dir = tmpdir("hot-roots");
        let s = Store::open(&dir).unwrap();
        for i in 0..32 {
            s.put(&Key::new("probe", "v0").field("i", i), b"x").unwrap();
        }
        assert_eq!(s.generation(), 32);
        let a = s.roots_shared().unwrap();
        let b = s.roots_shared().unwrap();
        assert!(Arc::ptr_eq(&a, &b), "second roots_shared must reuse the cached Arc");
        assert_eq!(a.len(), 32);
        s.put(&Key::new("probe", "v0").field("i", 99), b"y").unwrap();
        assert_eq!(s.generation(), 33);
        let c = s.roots_shared().unwrap();
        assert!(!Arc::ptr_eq(&a, &c), "put must move the listing epoch");
        assert_eq!(c.len(), 33);
        let d = s.roots_shared().unwrap();
        assert!(Arc::ptr_eq(&c, &d));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn roots_listing_invalidates_when_entry_count_moves_externally() {
        // Builder in another process: same handle generation, more root files.
        // Hot path stays generation-only; live-watch calls
        // roots_invalidate_if_external on a throttle.
        let dir = tmpdir("ext-roots");
        let s = Store::open(&dir).unwrap();
        s.put(&Key::new("a", "v0").field("i", 0), b"a").unwrap();
        let first = s.roots_shared().unwrap();
        assert_eq!(first.len(), 1);
        {
            let other = Store::open(&dir).unwrap();
            other.put(&Key::new("b", "v0").field("i", 1), b"foreign").unwrap();
        }
        assert_eq!(s.generation(), 1, "foreign put must not bump this handle");
        // Without the external probe, the hot path would keep serving the stale Arc.
        assert!(
            Arc::ptr_eq(&first, &s.roots_shared().unwrap()),
            "generation-only path stays warm until external invalidate"
        );
        assert!(s.roots_invalidate_if_external().unwrap());
        let second = s.roots_shared().unwrap();
        assert_eq!(second.len(), 2);
        assert!(!Arc::ptr_eq(&first, &second));
        let _ = fs::remove_dir_all(&dir);
    }
}
