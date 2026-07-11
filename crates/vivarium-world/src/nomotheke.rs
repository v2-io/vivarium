//! The nomotheke — the registry of nomoi, as *data* (Joseph, 2026-07-10: "I
//! would love code-level enforcement of exactly what is being bequested and
//! the epistemic labels and propagations").
//!
//! Every nomos declares itself here: its identity, its **epistemic tags**
//! (LEXICON §5's four axes — declared on the version, once), its **deps**
//! (from which *derived* state quality is computed by weakest-link fold — a
//! hi-physics kernel fed a placeholder yields placeholder-grade state, and
//! that is now a computation, not a discipline), its **bequests** (what it
//! hands forward, each with an explicit conservation claim — so "is mass
//! conserved?" is answerable by lookup, never by archaeology), and its
//! **assumptions** (entries in `ASSUMPTIONS.md`, the magic-constant ledger).
//!
//! Enforcement (structural, not hortatory):
//! - A nomos's store [`Key`] is minted through [`NomosDecl::key`] — the
//!   declaration IS the key source, so an undeclared nomos has no way into
//!   the store's world-law namespace. (`Key::new` remains public for the
//!   domain-neutral store layer and its tests; world-law code goes through
//!   declarations.)
//! - Tests pin: registry names unique; every declared dep is itself
//!   registered; every declared assumption exists verbatim in
//!   `ASSUMPTIONS.md` (compile-time include — moving or renaming a ledger
//!   entry that a nomos leans on FAILS THE BUILD's tests, which is exactly
//!   the "undiscoverably unlawful" failure made discoverable).
//! - The CLI's fidelity pyramid reads this registry to print declared and
//!   derived physics tiers next to each nomos — the honesty column.
//!
//! This is the declared-vs-derived machinery's first rung (LEXICON §5);
//! per-artifact derivation over the store's dependency cone (with recorded
//! convergence-ε) lands with component E.

use crate::store::Key;

/// Physics-fidelity tier (LEXICON §5 axis B — the load-bearing propagated one).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tier {
    /// No physical process at all (pure noise, pure placeholder).
    None,
    Low,
    Med,
    High,
}

impl Tier {
    pub fn letter(self) -> &'static str {
        match self {
            Tier::None => "-",
            Tier::Low => "L",
            Tier::Med => "M",
            Tier::High => "H",
        }
    }
}

/// The approach codes of `PHASES.md`'s ledger (A/R/S/T/P).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Approach {
    Analytic,
    Relaxation,
    Statistical,
    Taxonomy,
    Procedural,
}

/// What a bequest claims about conservation — explicit, so the honest answer
/// to "are we conserving X?" is a lookup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Conservation {
    /// Conserved and asserted (a probe/test watches it).
    Conserved,
    /// Physically-motivated exports cross the domain boundary unaccounted
    /// (e.g. sediment leaving a tile through its outlets).
    ExportsAtBoundary,
    /// No conservation claim at all — the placeholder's honest label.
    NotTracked,
}

/// One quantity a nomos hands forward (its slice of a phase Bequest).
pub struct Bequest {
    pub quantity: &'static str,
    pub conservation: Conservation,
}

/// A nomos, declared. The declaration is stamped per *version*: change the
/// algorithm → bump the version → re-declare (and the old declaration goes to
/// git history with the old code).
pub struct NomosDecl {
    pub name: &'static str,
    pub version: &'static str,
    /// The system (LEXICON §2 noun stack) this nomos runs a rung of.
    pub system: &'static str,
    pub approach: Approach,
    /// Axis A — Earth-history fidelity of this nomos itself.
    pub earth_fidelity: Tier,
    /// Axis B — physics fidelity of this nomos itself (declared, not derived).
    pub physics: Tier,
    /// Axis C — relation type / role, stated plainly (e.g. "#mech stand-in:
    /// fBm-as-tectonics").
    pub relation: &'static str,
    /// Axis D — implementation status, stated plainly (probes, verification).
    pub status: &'static str,
    /// Upstream nomoi whose outputs this one consumes.
    pub deps: &'static [&'static NomosDecl],
    /// What it hands forward.
    pub bequests: &'static [Bequest],
    /// Verbatim anchors into `ASSUMPTIONS.md` — every magic constant this
    /// nomos leans on. Checked against the ledger by test.
    pub assumptions: &'static [&'static str],
}

impl NomosDecl {
    /// Mint the store key stem for this nomos — the ONLY sanctioned way a
    /// world-law computation gets a key (callers fold in seed + coordinates).
    pub fn key(&self) -> Key {
        Key::new(self.name, self.version)
    }

    /// Derived physics tier of this nomos's *output state*: the weakest link
    /// of its own declared physics and every dependency's derived tier.
    /// Algorithm quality never launders inputs.
    pub fn derived_physics(&self) -> Tier {
        self.deps.iter().fold(self.physics, |acc, d| acc.min(d.derived_physics()))
    }

    /// Same fold for Earth-history fidelity.
    pub fn derived_earth(&self) -> Tier {
        self.deps.iter().fold(self.earth_fidelity, |acc, d| acc.min(d.derived_earth()))
    }
}

// --- The registry ----------------------------------------------------------

/// System #1 — the fBm coarse spine (surface prior on the sphere).
pub static SPINE: NomosDecl = NomosDecl {
    name: "spine-tile",
    version: "spine-2026-07-10b-sphere3d",
    system: "surface-prior",
    approach: Approach::Analytic,
    earth_fidelity: Tier::None, // no Earth process; hypsometry measured wrong for every era
    physics: Tier::None,        // pure coordinate noise; conserves nothing
    relation: "#mech stand-in: fBm-as-tectonics (ARCHITECTURE §2 — parameterization missing its error model); phase-structurally it impersonates Abyssal output rather than the Phase-2 submerged bequest (Joseph, 2026-07-10)",
    status: "built; deterministic + cross-face continuity + golden probed; hypsometry probe scores it (land 41.5%, unimodal, oceans shallow — all flagged)",
    deps: &[],
    bequests: &[Bequest { quantity: "surface elevation field (m)", conservation: Conservation::NotTracked }],
    assumptions: &["SEA_LEVEL_M", "continental band", "mountain band", "fBm shape"],
};

/// System #2 — fluvial erosion composed on the spine.
pub static EROSION: NomosDecl = NomosDecl {
    name: "erosion-tile",
    version: "erosion-2026-07-10a",
    system: "fluvial-erosion",
    approach: Approach::Procedural,
    earth_fidelity: Tier::Med, // stream-power/Davy–Lague are how real landscapes are modeled
    physics: Tier::Med,        // real process laws, uncalibrated rates, hardcoded edge policy
    relation: "mechanistic-causal (stream-power incision + deposition + talus + creep), on a stand-in substrate",
    status: "kernel probe-verified in the testbench (channel_profile, spike_probe, armor_regimes 1/3); tile form has fixed epochs (no convergence-ε — component E) and non-composable edges (plan Phase-3)",
    deps: &[&SPINE],
    bequests: &[Bequest { quantity: "eroded surface elevation field (m)", conservation: Conservation::ExportsAtBoundary }],
    assumptions: &["stream-power `m`", "erosion `k_dt`", "erosion run length"],
};

/// Every nomos there is. A store root whose name is not here is a bug.
pub static NOMOTHEKE: &[&NomosDecl] = &[&SPINE, &EROSION];

/// Look a nomos up by its key-stem name (the part before `@`).
pub fn lookup(name: &str) -> Option<&'static NomosDecl> {
    NOMOTHEKE.iter().copied().find(|n| n.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ledger, at compile time: a nomos may not lean on an assumption the
    /// ledger doesn't carry.
    const LEDGER: &str = include_str!("../../../ASSUMPTIONS.md");

    #[test]
    fn names_unique_and_deps_registered() {
        for (i, a) in NOMOTHEKE.iter().enumerate() {
            for b in &NOMOTHEKE[i + 1..] {
                assert_ne!(a.name, b.name, "duplicate nomos name");
            }
            for d in a.deps {
                assert!(
                    NOMOTHEKE.iter().any(|n| std::ptr::eq(*n, *d)),
                    "{}: dep {} not in the nomotheke",
                    a.name,
                    d.name
                );
            }
        }
    }

    #[test]
    fn every_assumption_is_in_the_ledger() {
        // The "undiscoverably unlawful" failure, made discoverable: a magic
        // constant a nomos leans on MUST have an ASSUMPTIONS.md entry, and
        // renaming/removing the entry without updating the declaration fails
        // this test.
        for n in NOMOTHEKE {
            for a in n.assumptions {
                assert!(
                    LEDGER.contains(a),
                    "{}: assumption anchor {a:?} not found in ASSUMPTIONS.md — \
                     ledger and nomotheke have drifted",
                    n.name
                );
            }
        }
    }

    #[test]
    fn derived_state_is_weakest_link() {
        // Erosion's kernel is Med physics, but it erodes a None-physics
        // placeholder — its OUTPUT is placeholder-grade. The fold, proven.
        assert_eq!(EROSION.physics, Tier::Med);
        assert_eq!(EROSION.derived_physics(), Tier::None);
        assert_eq!(EROSION.derived_earth(), Tier::None);
    }

    #[test]
    fn declarations_mint_the_keys() {
        assert!(SPINE.key().as_str().starts_with("spine-tile@spine-2026-07-10b-sphere3d"));
        assert!(EROSION.key().as_str().starts_with("erosion-tile@erosion-2026-07-10a"));
    }

    #[test]
    fn every_bequest_makes_a_conservation_claim() {
        // Vacuously structural (the type forces it) — kept as the statement of
        // intent: a bequest without a conservation stance cannot exist.
        for n in NOMOTHEKE {
            for b in n.bequests {
                let _ = b.conservation;
                assert!(!b.quantity.is_empty());
            }
        }
    }
}
