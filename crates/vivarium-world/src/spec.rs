//! The vivium manifest — the file that *individuates a world*.
//!
//! A vivium's identity is content-addressed: `hash(seed, law-versions,
//! generator-versions)` (`ref/research/vivium-operational-workflow.md` Stage 0;
//! LEXICON §4 build-stack). The manifest is where that identity is declared and
//! the one place a bare seed is handled — everything downstream receives it
//! inside a [`crate::query::World`], so key-seed and compute-seed cannot diverge.
//!
//! **Fields divide into three buckets, and the divide is load-bearing:**
//! - **identity** — participates in content-addressed keys; changing one forks a
//!   *new* world (law = kingdom identity, LEXICON §7.2). Today: `seed`.
//!   (Law/generator versions live in each recipe's key, not here — the manifest
//!   will *pin* them at Realization, later.)
//! - **label** — human-facing, never in keys; renaming invalidates nothing.
//!   Today: `name`.
//! - **demand** *(future)* — target phase, beacons, watchpoints: changes what
//!   gets built *first*, provably never what gets built (order-independence
//!   holds because recipes depend on artifacts by key, never "latest available").
//!
//! Format: a minimal `key = value` TOML subset, hand-parsed to keep this crate
//! dependency-free. Graduates to real TOML when the CLI crate exists — the file
//! is forward-compatible with a TOML parser by construction.

use std::fmt::Write as _;
use std::io;
use std::path::Path;

/// Store-format version this code writes (byte-format compatibility — the only
/// version that lives in the *path/manifest* rather than in recipe keys, per
/// `DESIGN-REDUX.md` §13).
pub const FORMAT: u32 = 1;

/// The manifest filename inside a world's save/store directory.
pub const MANIFEST_FILE: &str = "manifest";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldSpec {
    /// Store-format version (see [`FORMAT`]).
    pub format: u32,
    /// Label: the world's human name. Never keyed; rename freely.
    pub name: String,
    /// Identity: the world-seed. Every fated-noise draw and every memo key
    /// derives from it; changing it is creating a different world.
    pub seed: u64,
}

impl WorldSpec {
    pub fn new(name: impl Into<String>, seed: u64) -> Self {
        WorldSpec { format: FORMAT, name: name.into(), seed }
    }

    /// A fresh seed for a *new* world — the one moment entropy is welcome
    /// (recorded in the manifest immediately; deterministic ever after).
    pub fn fresh_seed() -> u64 {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        crate::noise::splitmix64(nanos ^ (std::process::id() as u64) << 32)
    }

    /// Serialize (TOML-subset; stable field order so the file diffs cleanly).
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "# vivarium world manifest — identity + labels (spec.rs)");
        let _ = writeln!(s, "format = {}", self.format);
        let _ = writeln!(s, "name = \"{}\"", self.name.replace('"', "'"));
        let _ = writeln!(s, "seed = {}", self.seed);
        s
    }

    /// Parse the TOML-subset. Unknown keys are *errors* (a typo'd identity field
    /// silently ignored would be an under-keying cousin: a world that isn't what
    /// its manifest says).
    pub fn parse(text: &str) -> Result<Self, String> {
        let (mut format, mut name, mut seed) = (None, None, None);
        for (n, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (k, v) = line
                .split_once('=')
                .ok_or_else(|| format!("manifest line {}: expected `key = value`", n + 1))?;
            let (k, v) = (k.trim(), v.trim());
            match k {
                "format" => format = Some(v.parse::<u32>().map_err(|e| format!("format: {e}"))?),
                "name" => name = Some(v.trim_matches('"').to_string()),
                "seed" => seed = Some(v.parse::<u64>().map_err(|e| format!("seed: {e}"))?),
                other => return Err(format!("manifest line {}: unknown key `{other}`", n + 1)),
            }
        }
        Ok(WorldSpec {
            format: format.ok_or("manifest missing `format`")?,
            name: name.ok_or("manifest missing `name`")?,
            seed: seed.ok_or("manifest missing `seed`")?,
        })
    }

    /// Load the manifest from a world directory, if present.
    pub fn load(dir: impl AsRef<Path>) -> io::Result<Option<Self>> {
        let path = dir.as_ref().join(MANIFEST_FILE);
        match std::fs::read_to_string(&path) {
            Ok(text) => Self::parse(&text)
                .map(Some)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Write the manifest into a world directory (atomic, like the store).
    pub fn save(&self, dir: impl AsRef<Path>) -> io::Result<()> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;
        let path = dir.join(MANIFEST_FILE);
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, self.to_string())?;
        std::fs::rename(&tmp, path)
    }

    /// Load a world's manifest, or create-and-save one (fresh seed) if the
    /// directory has none — the `vivarium new`-shaped entry point in miniature.
    pub fn load_or_create(dir: impl AsRef<Path>, name: &str) -> io::Result<Self> {
        if let Some(spec) = Self::load(&dir)? {
            return Ok(spec);
        }
        let spec = WorldSpec::new(name, Self::fresh_seed());
        spec.save(&dir)?;
        Ok(spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let a = WorldSpec::new("aldebaran", 0x1234_5678_9ABC_DEF0);
        let b = WorldSpec::parse(&a.to_string()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn unknown_keys_are_errors() {
        assert!(WorldSpec::parse("format = 1\nname = \"x\"\nseed = 1\nsede = 2").is_err());
    }

    #[test]
    fn missing_identity_is_an_error() {
        assert!(WorldSpec::parse("format = 1\nname = \"x\"").is_err());
    }

    #[test]
    fn load_or_create_persists_identity() {
        let dir = std::env::temp_dir().join(format!("vivarium-spec-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let a = WorldSpec::load_or_create(&dir, "demo").unwrap();
        let b = WorldSpec::load_or_create(&dir, "demo-different-default").unwrap();
        assert_eq!(a, b, "second open must load the SAME identity, not mint a new seed");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
