//! Content-addressed, memoized store — the framework's persistence spine
//! (`DESIGN-REDUX.md` §13: "the save-file *is* the memo store").
//!
//! **Domain-neutral by construction.** Keys and values here are opaque; the
//! recipes above (`query.rs`) give them meaning. Nothing in this module knows
//! about terrain, columns, or `CellId`s — a spatial field, a geochemical
//! reservoir, a food-web state-vector, or an agent population all persist
//! through this *same* interface. That neutrality is the framework's invariant
//! (`ARCHITECTURE.md` domain-fixation guard), not a courtesy.
//!
//! Shape (git-repo-like, §13): `objects/<value-hash>` hold immutable bytes;
//! `roots/<key-hash>` point at the object a complete key resolves to. Content-
//! addressing buys dedup (two keys computing identical bytes share one object)
//! and makes eviction cost only a recompute, never correctness. Copy the
//! directory → the world moves.
//!
//! MVP scope, flagged honestly:
//! - hash is 64-bit FNV-1a (fast, dependency-free) — fine for a demo's object
//!   count, **not** collision-safe at scale; swap to blake3 before this holds
//!   anything we cannot recompute.
//! - no GC/eviction, no manifest, no run-mode enforcement yet (Phase 0 decided
//!   convention-only + a `provisional` banner; the canon-root guard is deferred
//!   to the first graduation).
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
/// affects the value — recipe name + version + every input. Build it with
/// [`Key::field`] per input; omitting one is the under-keying trap (§12), so
/// fold in all of them (upstream hashes, params, seed, resolution, time).
#[derive(Clone, Debug)]
pub struct Key(String);

impl Key {
    /// Start a key for `recipe` at `version`. (Version is a constant for the
    /// MVP; it graduates to a source-derived hash — coarse-enough-to-cover-deps
    /// first, IR-normalized only if the build-chain makes it clean — later.)
    pub fn new(recipe: &str, version: &str) -> Self {
        Key(format!("{recipe}@{version}"))
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
        let obj = fs::read_to_string(self.roots.join(hex(key.hash()))).ok()?;
        fs::read(self.objects.join(obj.trim())).ok()
    }

    /// Store `value` under `key`. The bytes land at `objects/<value-hash>`
    /// (idempotent — re-putting identical bytes is a no-op) and `roots/<key-
    /// hash>` is pointed at them. Both writes go temp-then-rename, so a reader
    /// never sees a half-written object or root.
    pub fn put(&self, key: &Key, value: &[u8]) -> io::Result<()> {
        let obj_name = hex(fnv1a(value));
        let obj_path = self.objects.join(&obj_name);
        if !obj_path.exists() {
            write_atomic(&obj_path, value)?;
        }
        write_atomic(&self.roots.join(hex(key.hash())), obj_name.as_bytes())
    }
}

/// Write via a sibling `.tmp` + rename (atomic on a single filesystem).
fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
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
        let k = Key::new("spine", "v0")
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
        let k = Key::new("spine", "v0").field("tile", 42);
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
    fn distinct_complete_keys_never_alias() {
        // The under-keying guard, positively: fold every input in and neighbours
        // stay distinct.
        let dir = tmpdir("keys");
        let s = Store::open(&dir).unwrap();
        let k1 = Key::new("spine", "v0").field("oi", 100).field("oj", 200);
        let k2 = Key::new("spine", "v0").field("oi", 100).field("oj", 201);
        s.put(&k1, b"tileA").unwrap();
        s.put(&k2, b"tileB").unwrap();
        assert_eq!(s.get(&k1).as_deref(), Some(&b"tileA"[..]));
        assert_eq!(s.get(&k2).as_deref(), Some(&b"tileB"[..]));
        let _ = fs::remove_dir_all(&dir);
    }
}
