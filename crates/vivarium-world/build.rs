//! Build-time source-derived nomos version.
//!
//! Hashes every `.rs` file in this crate's `src/` tree and injects the digest
//! as the `VIVARIUM_SRC_HASH` rustc-env, which `nomotheke::NomosDecl::key`
//! folds into every world-law key stem. Claim home:
//! `#form-complete-content-addressed-key` FE(4) / `#detail-epistemics-toolchain`
//! FE(5). The digest algorithm lives in `src/source_hash.rs` (self-contained,
//! std-only) and is `include!`-d here so build and lib share ONE algorithm —
//! the lib's `#[cfg(test)]` probes exercise it, and a test re-derives the digest
//! from live source and asserts it equals the value injected here (that equality
//! is what convicts a stale hash if `rerun-if-changed` below ever breaks).

include!("src/source_hash.rs");

fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let src = std::path::Path::new(&manifest).join("src");

    // Re-run whenever any source file changes. Cargo re-runs a build script when
    // ANY path emitted here changes; pointing at each source file (not just the
    // dir) is what makes an edit to a single kernel re-trigger the hash. Also
    // watch this script and the algorithm file themselves.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    if let Ok(entries) = collect_rs(&src) {
        for (rel, _) in &entries {
            println!("cargo:rerun-if-changed=src/{rel}");
        }
    }

    let digest = digest_dir(&src).unwrap_or_else(|e| {
        // Do not fail the build on an unreadable tree — emit the legible
        // sentinel so a key visibly reads `src=unhashed` (unprotected, not
        // silently colliding). println! to stderr surfaces the why.
        eprintln!("vivarium build.rs: could not hash src/ ({e}); using sentinel");
        SRC_HASH_SENTINEL.to_string()
    });
    println!("cargo:rustc-env=VIVARIUM_SRC_HASH={digest}");
}
