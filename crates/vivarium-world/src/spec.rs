//! The vivium manifest — the file that *individuates a world*.
//!
//! A vivium's identity is content-addressed: `hash(seed, law-versions,
//! generator-versions)` (`#detail-vivium-lifecycle / #disc-unlawfulness-budget` Stage 0;
//! LEXICON §4 build-stack). The manifest is where that identity is declared and
//! the one place a bare seed is handled — everything downstream receives it
//! inside a [`crate::query::World`], so key-seed and compute-seed cannot diverge.
//!
//! **Fields divide into three buckets, and the divide is load-bearing:**
//! - **identity** — participates in content-addressed keys; changing one forks a
//!   *new* world (law = kingdom identity, LEXICON §7.2). Today: `seed`.
//!   (Law/generator versions live in each nomos's key, not here — the manifest
//!   will *pin* them at Realization, later.)
//! - **label** — human-facing, never in keys; renaming invalidates nothing.
//!   Today: `name`.
//! - **demand** *(future)* — target phase, beacons, watchpoints: changes what
//!   gets built *first*, provably never what gets built (order-independence
//!   holds because nomos depend on artifacts by key, never "latest available").
//!
//! Format: a minimal `key = value` TOML subset, hand-parsed to keep this crate
//! dependency-free. Graduates to real TOML when the CLI crate exists — the file
//! is forward-compatible with a TOML parser by construction.

use std::fmt::Write as _;
use std::io;
use std::path::Path;

/// Store-format version this code writes (byte-format compatibility — the only
/// version that lives in the *path/manifest* rather than in nomos keys, per
/// `doc/design/DESIGN-REDUX.md` §13).
pub const FORMAT: u32 = 1;

/// The manifest filename inside a world's save/store directory.
pub const MANIFEST_FILE: &str = "manifest";

/// The **demand** bucket: what *this* vivium prescribed — how far the ladder is
/// driven and at what fidelity ( #form-manifest-prescribes-vivium FE(2), whose
/// field inventory this implements: order, target phase, and demand posture).
///
/// **Demand is not law and not identity.** It changes what gets built, and in
/// what order, and never what a built artifact *contains* — nomos depend on
/// artifacts by complete key, never on "latest available"
/// ( #form-depend-by-key-never-latest ), so these fields are freely editable
/// mid-build and are deliberately **not** folded into any key. Editing `level`
/// asks for different tiles; it does not change what a tile at a given key means.
///
/// **On the two counts living here.** `erosion_epochs` and `water_steps` are
/// arbitrary numbers (`ASSUMPTIONS.md`, and #obs-erosion-residual-is-driver-bound
/// / #obs-water-fill-never-settles measured why no criterion replaces them yet).
/// Their home is *this* bucket and not `NomosDecl`, and the distinction is the
/// whole point: the manifest records **what this world asked for**, which is
/// true; a `NomosDecl` field would assert them as part of the declared law,
/// which is false. Moving an unjustified number into the law layer would make it
/// look principled without making it convictable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Demand {
    /// Which ordinum (world-kind floor) this vivium is built against.
    pub order: String,
    /// How far up the phase ladder this vivium is driven.
    pub target_phase: u32,
    /// Cube-face subdivision level the builder sweeps.
    pub level: u8,
    /// How densely the deep-time cooling chain is sampled — rounded up to a
    /// nested count when materialized ( #form-time-indexed-stage-chains FE(9) ).
    pub frames: u32,
    /// Fluvial epochs per tile. Arbitrary; see the type note above.
    pub erosion_epochs: u32,
    /// Surface-water relaxation steps per tile. Arbitrary; see the type note.
    pub water_steps: u32,
}

impl Default for Demand {
    /// The values the CLI carried as flag defaults before the manifest existed to
    /// hold them. Written into every new world so the prescription is visible and
    /// editable rather than implicit in a binary.
    fn default() -> Self {
        Demand {
            order: "terrestris".into(),
            target_phase: 4, // Abyssal — the phase the tree currently builds to
            level: 7,
            frames: 6,
            erosion_epochs: 40,
            water_steps: 200,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldSpec {
    /// Store-format version (see [`FORMAT`]).
    pub format: u32,
    /// Label: the world's human name. Never keyed; rename freely.
    pub name: String,
    /// Identity: the world-seed. Every fated-noise draw and every memo key
    /// derives from it; changing it is creating a different world.
    pub seed: u64,
    /// Demand: this vivium's prescription. Never keyed; see [`Demand`].
    pub demand: Demand,
}

impl WorldSpec {
    pub fn new(name: impl Into<String>, seed: u64) -> Self {
        WorldSpec { format: FORMAT, name: name.into(), seed, demand: Demand::default() }
    }

    /// A fresh seed for a *new* world — the one moment entropy is welcome
    /// (recorded in the manifest immediately; deterministic ever after).
    ///
    /// Not world-law: identity *minting* is outside the fated causal path.
    /// After the seed is written, every draw is pure of (seed, key).
    #[allow(clippy::disallowed_methods)] // SystemTime::now — mint only; see above
    pub fn fresh_seed() -> u64 {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        crate::noise::splitmix64(nanos ^ (std::process::id() as u64) << 32)
    }

    /// Serialize (TOML-subset; stable field order so the file diffs cleanly).
    ///
    /// The three buckets are written under their own headings because the divide
    /// is load-bearing and a reader editing this file needs to see which side of
    /// it they are on: editing `seed` forks a new world, editing `name` costs
    /// nothing, editing demand changes what gets built without invalidating a
    /// single memo.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let d = &self.demand;
        let _ = writeln!(s, "# vivarium world manifest (spec.rs)");
        let _ = writeln!(s, "# identity — in every key; changing it forks a NEW world");
        let _ = writeln!(s, "format = {}", self.format);
        let _ = writeln!(s, "seed = {}", self.seed);
        let _ = writeln!(s, "# label — never keyed; rename freely");
        let _ = writeln!(s, "name = \"{}\"", self.name.replace('"', "'"));
        let _ = writeln!(s, "# demand — what THIS vivium asked for. Never keyed; edit any time.");
        let _ = writeln!(s, "# Changes what gets built and in what order, never what a built artifact contains.");
        let _ = writeln!(s, "order = \"{}\"", d.order.replace('"', "'"));
        let _ = writeln!(s, "target_phase = {}", d.target_phase);
        let _ = writeln!(s, "level = {}", d.level);
        let _ = writeln!(s, "frames = {}", d.frames);
        let _ = writeln!(s, "erosion_epochs = {}", d.erosion_epochs);
        let _ = writeln!(s, "water_steps = {}", d.water_steps);
        s
    }

    /// Parse the TOML-subset. Unknown keys are *errors* (a typo'd identity field
    /// silently ignored would be an under-keying cousin: a world that isn't what
    /// its manifest says).
    ///
    /// **Demand keys are optional and default**, unlike identity keys. A manifest
    /// written before the demand bucket existed must still open, and it must open
    /// as the world it always was — so a missing demand key is the declared
    /// default, never an error. Identity keys stay required for the same reason
    /// they always were.
    pub fn parse(text: &str) -> Result<Self, String> {
        let (mut format, mut name, mut seed) = (None, None, None);
        let mut d = Demand::default();
        for (n, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (k, v) = line
                .split_once('=')
                .ok_or_else(|| format!("manifest line {}: expected `key = value`", n + 1))?;
            let (k, v) = (k.trim(), v.trim());
            let num = |what: &str, v: &str| -> Result<u32, String> {
                v.parse::<u32>().map_err(|e| format!("{what}: {e}"))
            };
            match k {
                "format" => format = Some(num("format", v)?),
                "name" => name = Some(v.trim_matches('"').to_string()),
                "seed" => seed = Some(v.parse::<u64>().map_err(|e| format!("seed: {e}"))?),
                "order" => d.order = v.trim_matches('"').to_string(),
                "target_phase" => d.target_phase = num("target_phase", v)?,
                "level" => d.level = num("level", v)?.min(20) as u8,
                "frames" => d.frames = num("frames", v)?,
                "erosion_epochs" => d.erosion_epochs = num("erosion_epochs", v)?,
                "water_steps" => d.water_steps = num("water_steps", v)?,
                other => return Err(format!("manifest line {}: unknown key `{other}`", n + 1)),
            }
        }
        Ok(WorldSpec {
            format: format.ok_or("manifest missing `format`")?,
            name: name.ok_or("manifest missing `name`")?,
            seed: seed.ok_or("manifest missing `seed`")?,
            demand: d,
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
    fn a_pre_demand_manifest_still_opens_as_the_world_it_always_was() {
        // Every manifest written before the demand bucket existed has exactly
        // these three keys. Refusing them, or opening them as a DIFFERENT world,
        // would strand every existing vivium — identity must survive the schema
        // growing underneath it.
        let old = "format = 1\nname = \"aldebaran\"\nseed = 42\n";
        let spec = WorldSpec::parse(old).expect("a pre-demand manifest must still open");
        assert_eq!(spec.seed, 42, "identity is unchanged");
        assert_eq!(spec.demand, Demand::default(), "absent demand keys are the declared default");
    }

    #[test]
    fn demand_roundtrips_and_is_not_identity() {
        let mut a = WorldSpec::new("demo", 7);
        a.demand.level = 9;
        a.demand.frames = 60;
        a.demand.erosion_epochs = 12;
        let b = WorldSpec::parse(&a.to_string()).unwrap();
        assert_eq!(a, b);
        // The load-bearing property: editing demand does not fork the world.
        let mut c = b.clone();
        c.demand.level = 11;
        assert_eq!(c.seed, b.seed, "demand is not identity — the seed is untouched");
    }

    #[test]
    fn demand_values_out_of_range_clamp_rather_than_strand_the_world() {
        // `level` is a u8 cap in the code; a hand-edited 99 should not make the
        // manifest unopenable, because an unopenable manifest is an unopenable
        // world. It clamps to the same ceiling the CLI applies.
        let spec = WorldSpec::parse("format = 1\nname = \"x\"\nseed = 1\nlevel = 99\n").unwrap();
        assert_eq!(spec.demand.level, 20);
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
