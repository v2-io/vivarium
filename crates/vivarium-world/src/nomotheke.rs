//! The nomotheke — the registry of nomoi, as *data* (Joseph, 2026-07-10: "I
//! would love code-level enforcement of exactly what is being promised and
//! the epistemic labels and propagations").
//!
//! Every nomos declares itself here: its identity, its **epistemic tags**
//! (LEXICON §5's four axes — declared on the version, once), its **deps**
//! (from which *derived* state quality is computed by weakest-link fold — a
//! hi-physics kernel fed a placeholder yields placeholder-grade state, and
//! that is now a computation, not a discipline), its **consumes** (the fluxed
//! quantities it needs, from the shared [`crate::flux`] vocabulary — the
//! quantity-level coupling contract the audit resolves to producers), its
//! **promises** (what it hands forward, each with an explicit conservation
//! claim — so "is mass conserved?" is answerable by lookup, never by
//! archaeology), and its **assumptions** (entries in `ASSUMPTIONS.md`, the
//! magic-constant ledger).
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

use crate::flux;
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

/// The approach codes of DESIGN-SYSTEMS.md's algorithm ledger (A/R/S/T/P).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Approach {
    Analytic,
    Relaxation,
    Statistical,
    Taxonomy,
    Procedural,
}

/// What a promise claims about conservation — explicit, so the honest answer
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

impl Conservation {
    /// Short human label for reports (the honesty column, the flux web).
    pub fn label(self) -> &'static str {
        match self {
            Conservation::Conserved => "conserved",
            Conservation::ExportsAtBoundary => "exports-at-boundary",
            Conservation::NotTracked => "not-tracked",
        }
    }
}

/// One quantity a nomos hands forward (its slice of a phase Promise).
pub struct Promise {
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
    /// The fluxed quantities this nomos *needs* (in), drawn from the shared
    /// [`crate::flux`] vocabulary. This is the quantity-level half of the
    /// coupling contract: `deps` names the *nomoi* an occupant depends on;
    /// `consumes` names the *quantities* the role depends on, independent of who
    /// supplies them. The audit ([`crate::audit`]) matches each consumed
    /// quantity to a producer (`Promise::quantity`) — a match is an edge, a
    /// miss is the honest "rain without a sky" finding, and a permit may later
    /// license a miss. A consumed quantity that *is* met by a nomos must have
    /// that nomos in `deps` (else the complete key would omit its version) —
    /// pinned by a test. (`doc/plan/regula-conformance-design.md` §3;
    /// `VIVARIA-DEFINITIONS.md` §"the web".)
    pub consumes: &'static [&'static str],
    /// What it hands forward.
    pub promises: &'static [Promise],
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

/// The hydrosphere — the planet's conserved water budget, from an ante-mundane
/// charge (the water-mass-fraction). The framework's first NON-FIELD nomos: a
/// reservoir/box (global stocks, no spatial grid — the domain-fixation-guard
/// generality probe, ARCHITECTURE §0). Produces the atmosphere-water stock a
/// climate nomos will draw precipitation from — the honest root under "rain".
pub static HYDROSPHERE: NomosDecl = NomosDecl {
    name: "hydrosphere",
    version: "hydrosphere-2026-07-12a",
    system: "planetary-water-budget",
    approach: Approach::Analytic, // closed-form derivation from declared constants
    earth_fidelity: Tier::Low,    // earth-ref fractions; ocean lumps ice + groundwater
    physics: Tier::Low,           // conservation is EXACT; the partition is earth-ref, not dynamically modeled
    relation: "conservation law: inventory = ante-mundane water-mass-fraction × planet mass, partitioned across reservoirs (total = ocean + atmosphere, exact by construction)",
    status: "v0 box/reservoir — earth-ref fractions (ASSUMPTIONS); conservation unit-tested (residual < 1e-6 km³, Earth budget order-checked); the precip/evap FLOW between reservoirs is the next rung",
    deps: &[],
    consumes: &[], // reads ante-mundane charges (planet mass, water-mass-fraction) — declared constants, not nomos outputs
    promises: &[Promise { quantity: flux::ATMOSPHERE_WATER, conservation: Conservation::Conserved }],
    assumptions: &["water mass fraction", "atmosphere fraction", "planet mass"],
};

/// System #1 — the fBm coarse spine (surface prior on the sphere).
pub static SPINE: NomosDecl = NomosDecl {
    name: "spine-tile",
    version: "spine-2026-07-10b-sphere3d",
    system: "surface-prior",
    approach: Approach::Analytic,
    earth_fidelity: Tier::None, // no Earth process; hypsometry measured wrong for every era
    physics: Tier::None,        // pure coordinate noise; conserves nothing
    relation: "#mech stand-in: fBm-as-tectonics (ARCHITECTURE §2 — parameterization missing its error model); phase-structurally it impersonates Abyssal output rather than the Phase-2 submerged promise (Joseph, 2026-07-10)",
    status: "built; deterministic + cross-face continuity + golden probed; hypsometry probe scores it (land 41.5%, unimodal, oceans shallow — all flagged)",
    deps: &[],
    consumes: &[], // a prior conjured from (seed, coordinate) alone — the world's root input
    promises: &[Promise { quantity: flux::SURFACE_ELEVATION, conservation: Conservation::NotTracked }],
    assumptions: &["SEA_LEVEL_M", "continental band", "mountain band", "fBm shape"],
};

/// The tectonic driver — rock-uplift rate, its own article of law (Joseph,
/// 2026-07-12: "uplift is a separate nomos that right now is 30 lines"). Erosion
/// consumes its output; without it the world planes to a peneplain and grows no
/// macro relief. v0 is a crude declared stub (`crate::uplift`).
pub static UPLIFT: NomosDecl = NomosDecl {
    name: "uplift-tile",
    version: "uplift-2026-07-12a-fbm-stub",
    system: "tectonic-uplift",
    approach: Approach::Analytic, // a closed-form coordinate noise field, like the spine
    earth_fidelity: Tier::None,   // no Earth tectonic history — a placeholder curve
    physics: Tier::None,          // no mechanics; low-frequency fBm stand-in, uncalibrated rate
    relation: "#mech stand-in: constant rate × low-frequency fBm (differential uplift); the real driver is the thermal-spine / plume-upwelling work (TODO)",
    status: "v0 crude stub: deterministic differential uplift-rate field (band + determinism unit-tested in uplift.rs); rate is a declared placeholder, no calibration",
    deps: &[],
    consumes: &[], // conjured from (seed, coordinate); a real driver would consume mantle-thermal state
    promises: &[Promise { quantity: flux::ROCK_UPLIFT_RATE, conservation: Conservation::NotTracked }],
    assumptions: &["uplift rate"],
};

/// System #2 — fluvial erosion composed on the spine.
pub static EROSION: NomosDecl = NomosDecl {
    name: "erosion-tile",
    version: "erosion-2026-07-12b-uplift", // now consumes the uplift nomos's field
    system: "fluvial-erosion",
    approach: Approach::Procedural,
    earth_fidelity: Tier::Med, // stream-power/Davy–Lague are how real landscapes are modeled
    physics: Tier::Med,        // real process laws, uncalibrated rates, hardcoded edge policy
    relation: "mechanistic-causal (stream-power incision + deposition + talus + creep), on a stand-in substrate",
    status: "kernel probe-verified in the testbench (channel_profile, spike_probe, armor_regimes 1/3); tile form has fixed epochs (no convergence-ε — component E) and non-composable edges (plan Phase-3)",
    deps: &[&SPINE, &UPLIFT],
    // Three needs, each met or honestly flagged: the surface it carves (met →
    // SPINE), the rock-uplift rate it carves AGAINST (met → UPLIFT — the tectonic
    // driver, its own nomos now), and the rain that drives incision (UNMET — no
    // nomos produces precipitation; erosion assumes uniform rain in its
    // drainage-area discharge, so "principled incision" stays gloss until the
    // atmosphere→water-cycle chain lands).
    consumes: &[flux::SURFACE_ELEVATION, flux::ROCK_UPLIFT_RATE, flux::PRECIPITATION],
    promises: &[Promise { quantity: flux::ERODED_SURFACE, conservation: Conservation::ExportsAtBoundary }],
    assumptions: &["stream-power `m`", "erosion `k_dt`", "erosion run length"],
};

/// System #3 — conserved shallow water settling on the eroded bed.
pub static WATER: NomosDecl = NomosDecl {
    name: "water-tile",
    version: "water-2026-07-10a",
    system: "surface-water",
    approach: Approach::Relaxation,
    earth_fidelity: Tier::Low, // real hydraulics, ~10x rain-cycle fudge, bounded (not converged) fill
    physics: Tier::Med,        // local-inertial shallow water + Manning/Jarrett friction, kernel probe-verified
    relation: "mechanistic-causal (virtual-pipes shallow water, conserved atmosphere/ocean stores), on a stand-in substrate; tiles are hydrologically ISOLATED until flux-BC (plan Phase-3) — no cross-tile rivers yet",
    status: "kernel probe-verified (conserves_total_water, rain_pools_in_the_bowl, channel_profile in testbench); tile form runs a FIXED step count (no near-stationarity gate — the analytic init / component E replace it)",
    deps: &[&EROSION],
    // Settles on the eroded bed (met → EROSION, in deps). Its rain is the
    // documented ~10× fudge (ASSUMPTIONS "rain rate"): it consumes precipitation
    // that no nomos produces — the same UNMET flag as erosion, and the reason
    // the water cycle can't yet close.
    consumes: &[flux::ERODED_SURFACE, flux::PRECIPITATION],
    promises: &[Promise { quantity: flux::STANDING_WATER_DEPTH, conservation: Conservation::Conserved }],
    assumptions: &["rain rate", "atmosphere store", "water fill steps", "SEA_LEVEL_M"],
};

/// Every nomos there is. A store root whose name is not here is a bug.
pub static NOMOTHEKE: &[&NomosDecl] = &[&HYDROSPHERE, &SPINE, &UPLIFT, &EROSION, &WATER];

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
        assert!(EROSION.key().as_str().starts_with("erosion-tile@erosion-2026-07-12b-uplift"));
    }

    #[test]
    fn flux_vocabulary_is_closed() {
        // Every produced and consumed quantity must be a known flux term — the
        // typo-can't-hide discipline (a mistyped `consumes` string would else be
        // a silently-broken coupling edge the audit reads as "unmet" when the
        // human sees an obvious match). Same guarantee as the ASSUMPTIONS anchors.
        for n in NOMOTHEKE {
            for p in n.promises {
                assert!(
                    flux::is_in_vocabulary(p.quantity),
                    "{}: produced quantity {:?} is not in flux::VOCABULARY",
                    n.name,
                    p.quantity
                );
            }
            for q in n.consumes {
                assert!(
                    flux::is_in_vocabulary(q),
                    "{}: consumed quantity {q:?} is not in flux::VOCABULARY",
                    n.name
                );
            }
        }
    }

    #[test]
    fn consumed_and_met_implies_in_deps() {
        // The complete-key invariant (DESIGN-REDUX §12): if a nomos consumes a
        // quantity that some registered nomos produces, that producer MUST be in
        // its `deps` — otherwise the consumer's store key would omit the
        // producer's version, and a producer change would not invalidate the
        // consumer (a stale-memo lie). A consumed quantity NOT produced by anyone
        // is legal here — that is the honest "unmet" case the audit reports, not
        // a coherence error.
        for n in NOMOTHEKE {
            for q in n.consumes {
                if let Some(producer) = NOMOTHEKE.iter().find(|m| m.promises.iter().any(|p| p.quantity == *q)) {
                    assert!(
                        n.deps.iter().any(|d| std::ptr::eq(*d, *producer)),
                        "{}: consumes {q:?} (produced by {}), but {} is not in its deps — \
                         the complete key would omit the producer's version",
                        n.name,
                        producer.name,
                        producer.name
                    );
                }
            }
        }
    }

    #[test]
    fn precipitation_is_the_live_unmet_specimen() {
        // "Rain without a sky", mechanized: erosion and water both consume
        // precipitation, and no nomos produces it — so the audit's honest answer
        // to "can we rain principled water?" bottoms out here until the
        // atmosphere→water-cycle chain lands. If some future nomos produces
        // precipitation, this test's failure is the reminder to delete the
        // specimen (integration is replacement), not to loosen the assertion.
        let consumes_precip: Vec<_> =
            NOMOTHEKE.iter().filter(|n| n.consumes.contains(&flux::PRECIPITATION)).map(|n| n.name).collect();
        assert_eq!(consumes_precip, vec!["erosion-tile", "water-tile"], "the current precip consumers");
        let produces_precip = NOMOTHEKE.iter().any(|n| n.promises.iter().any(|p| p.quantity == flux::PRECIPITATION));
        assert!(!produces_precip, "no nomos should produce precipitation yet — the reservoir/water-cycle chain is unbuilt");
    }

    #[test]
    fn every_promise_makes_a_conservation_claim() {
        // Vacuously structural (the type forces it) — kept as the statement of
        // intent: a promise without a conservation stance cannot exist.
        for n in NOMOTHEKE {
            for b in n.promises {
                let _ = b.conservation;
                assert!(!b.quantity.is_empty());
            }
        }
    }
}
