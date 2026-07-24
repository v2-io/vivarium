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
#[derive(Clone, Copy, Debug, Default)]
pub struct PutOpts {
    /// Written under waived flux admission (`--allow-unmet`). Census and status
    /// must surface this; provisional roots are not lawful *in vivia* evidence
    /// ( #form-builder-admission · #form-flux-web ).
    pub provisional: bool,
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
}

/// A filesystem-backed content-addressed store.
pub struct Store {
    objects: PathBuf,
    roots: PathBuf,
}

impl Store {
    /// Open (creating if needed) a store rooted at `dir`.
    pub fn open(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let objects = dir.join("objects");
        let roots = dir.join("roots");
        fs::create_dir_all(&objects)?;
        fs::create_dir_all(&roots)?;
        Ok(Store { objects, roots })
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
    /// [provisional]   # optional third line when PutOpts.provisional
    /// ```
    /// Line 2 makes the store *enumerable by meaning*. Line 3 is the honesty
    /// bit for waived admission ( #form-builder-admission residual A/B ).
    pub fn put_with(&self, key: &Key, value: &[u8], opts: PutOpts) -> io::Result<()> {
        let obj_name = hex(fnv1a(value));
        let obj_path = self.objects.join(&obj_name);
        if !obj_path.exists() {
            write_atomic(&obj_path, value)?;
        }
        let mut root = format!("{obj_name}\n{}", key.as_str());
        if opts.provisional {
            root.push_str("\nprovisional");
        }
        write_atomic(&self.roots.join(hex(key.hash())), root.as_bytes())
    }

    /// Enumerate every root for the census instruments. Roots written before
    /// key-strings were recorded (format v1, pre-2026-07-10) appear with an
    /// empty key and should be counted as "unknown"; they are valid but not
    /// attributable. Missing third line ⇒ not provisional.
    pub fn roots(&self) -> io::Result<Vec<RootEntry>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.roots)? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "tmp") {
                continue;
            }
            let text = fs::read_to_string(&path)?;
            let mut lines = text.lines();
            let object = lines.next().unwrap_or("").trim().to_string();
            let key = lines.next().unwrap_or("").trim().to_string();
            let provisional = lines.any(|l| l.trim() == "provisional");
            out.push(RootEntry {
                key,
                object,
                provisional,
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
        s.put_with(&k, b"waived", PutOpts { provisional: true }).unwrap();
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
}
