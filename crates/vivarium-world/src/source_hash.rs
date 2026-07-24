// Source-derived nomos-version digest — the mechanism that makes hand-stamped
// version bumps stop being *load-bearing for correctness*.
//
// Claim home: `#form-complete-content-addressed-key` FE(4) ("source-derived
// versions ... coarse enough to cover transitive in-crate deps, so human
// bump-discipline is not load-bearing") and `#detail-epistemics-toolchain`
// FE(5) ("Build-time nomos-version hash (`build.rs` source-hashing into
// keys)"). This file is that build-time hash.
//
// **What it does.** At build time [`build.rs`] hashes *every* `.rs` file in
// this crate's `src/` tree and injects the digest as the `VIVARIUM_SRC_HASH`
// rustc-env; [`crate::nomotheke::NomosDecl::key`] folds it into every world-law
// key stem. Edit any kernel and the digest changes, so every memo keyed by it
// misses and recomputes — a forgotten hand-stamp bump can no longer serve a
// stale memo that lies (`#form-complete-content-addressed-key` FE(1)/(3);
// Joseph 2026-07-09: complete key, never cache-disable or manual flush).
//
// **Why whole-crate (the coarse floor, chosen deliberately).** Over-keying
// costs only recompute; under-keying is silent corruption, so when the choice
// is forced we over-key (`#form-complete-content-addressed-key` FE(2)). A
// whole-crate digest is the coarse-but-safe floor: it covers every kernel
// *and* every transitive in-crate dependency (shared helpers in `gen.rs`,
// `sphere.rs`, `material.rs`, …) with no hand-maintained "which source feeds
// which nomos" map. Per-nomos attribution would be finer, but any such map is
// itself a bump-discipline surface — forget to add a newly-shared helper to a
// nomos's file list and you under-key again, reintroducing the exact hazard
// this removes. So the floor is honest and the ceiling is deferred until (if
// ever) the module graph makes attribution derivable rather than declared.
// The price is real: editing a comment or a test in this crate invalidates the
// whole store. That is the safe direction, and eviction/recompute is the only
// cost (`#form-store-as-save` FE(5) invalidation-vs-eviction).
//
// This module is self-contained (std only, no `use` at item scope) so
// `build.rs` can `include!` it directly while the lib compiles it as a normal
// module whose `#[cfg(test)]` probes run under `cargo test`.

/// Sentinel digest if the build script could not read the source tree. It is a
/// legible, obviously-wrong value: a key carrying `src=unhashed` is visibly
/// unprotected rather than silently colliding across source revisions.
pub const SRC_HASH_SENTINEL: &str = "unhashed";

/// FNV-1a 64 — same family as the store's content hash (`store.rs`), so this
/// crate stays dependency-free. Collision-resistance is not the property we
/// need here (this is a change-detector over our own source, not an adversarial
/// digest); *change-sensitivity* is, and FNV-1a flips on a single differing byte.
fn fnv1a_step(mut h: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Digest a set of `(relative-path, bytes)` source entries into a stable hex
/// string. Sorted by path first, so filesystem directory-iteration order (which
/// is not guaranteed stable across platforms) cannot change the digest — the
/// digest is a pure function of *content + names*, nothing else. The path is
/// folded in as well as the bytes, so moving code between files (or adding an
/// empty file) is a change, matching "everything that affects the value".
pub fn digest_files(mut entries: Vec<(String, Vec<u8>)>) -> String {
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (path, bytes) in &entries {
        h = fnv1a_step(h, path.as_bytes());
        h = fnv1a_step(h, &[0x1f]); // unit separator: path ends
        h = fnv1a_step(h, bytes);
        h = fnv1a_step(h, &[0x1e]); // record separator: entry ends
    }
    format!("{h:016x}")
}

/// Collect `(relative-path, bytes)` for every `.rs` file under `root`,
/// recursively. Paths are relative to `root` and use `/` on every platform so
/// the digest is identical regardless of the host separator. `.tmp` and
/// non-`.rs` files are ignored.
pub fn collect_rs(root: &std::path::Path) -> std::io::Result<Vec<(String, Vec<u8>)>> {
    fn rec(
        dir: &std::path::Path,
        base: &std::path::Path,
        out: &mut Vec<(String, Vec<u8>)>,
    ) -> std::io::Result<()> {
        let mut entries: Vec<_> = std::fs::read_dir(dir)?.collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(std::fs::DirEntry::path);
        for e in entries {
            let p = e.path();
            if p.is_dir() {
                rec(&p, base, out)?;
            } else if p.extension().is_some_and(|x| x == "rs") {
                let rel = p
                    .strip_prefix(base)
                    .unwrap_or(&p)
                    .to_string_lossy()
                    .replace('\\', "/");
                out.push((rel, std::fs::read(&p)?));
            }
        }
        Ok(())
    }
    let mut out = Vec::new();
    rec(root, root, &mut out)?;
    Ok(out)
}

/// The whole-crate digest of a `src/` tree — [`collect_rs`] then [`digest_files`].
/// Used by `build.rs` at build time and re-derived independently by the lib's
/// probe at test time; the two must agree, which is what convicts staleness.
pub fn digest_dir(root: &std::path::Path) -> std::io::Result<String> {
    Ok(digest_files(collect_rs(root)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_order_independent() {
        // Directory iteration order must not change the digest — it is a pure
        // function of content + names.
        let a = vec![("a.rs".into(), b"one".to_vec()), ("b.rs".into(), b"two".to_vec())];
        let b = vec![("b.rs".into(), b"two".to_vec()), ("a.rs".into(), b"one".to_vec())];
        assert_eq!(digest_files(a), digest_files(b));
    }

    #[test]
    fn one_byte_of_content_flips_the_digest() {
        // The load-bearing property: a kernel edit changes the key. Here in the
        // small — a single differing byte yields a different digest, so a source
        // change cannot be served a stale memo.
        let base = vec![("k.rs".into(), b"fn kernel() { 1.0 }".to_vec())];
        let edited = vec![("k.rs".into(), b"fn kernel() { 2.0 }".to_vec())];
        assert_ne!(digest_files(base), digest_files(edited));
    }

    #[test]
    fn renaming_a_file_flips_the_digest() {
        // Path participates: moving identical code to a new file is a change.
        let a = vec![("erosion.rs".into(), b"same".to_vec())];
        let b = vec![("erosion2.rs".into(), b"same".to_vec())];
        assert_ne!(digest_files(a), digest_files(b));
    }

    #[test]
    fn splitting_content_across_a_boundary_flips_the_digest() {
        // The separators prevent "abc"+"" from colliding with "ab"+"c" once
        // paths differ; guards the trivial concatenation-collision.
        let a = vec![("x.rs".into(), b"ab".to_vec()), ("y.rs".into(), b"c".to_vec())];
        let b = vec![("x.rs".into(), b"a".to_vec()), ("y.rs".into(), b"bc".to_vec())];
        assert_ne!(digest_files(a), digest_files(b));
    }

    #[test]
    fn digest_is_stable_and_hex() {
        let d = digest_files(vec![("a.rs".into(), b"x".to_vec())]);
        assert_eq!(d.len(), 16);
        assert!(d.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
