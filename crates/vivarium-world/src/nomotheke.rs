//! The nomotheke — the registry of nomoi, as *data* (Joseph, 2026-07-10: "I
//! would love code-level enforcement of exactly what is being promised and
//! the epistemic labels and propagations").
//!
//! Claim home for the registry contract: `#form-nomotheke-registry` (undeclared
//! law is unlawful; declaration is the key stem; live test surfaces). One
//! article of law: `#def-nomos`. Flux vocabulary join: `#form-flux-web`.
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
//! Enforcement (structural, not hortatory) — see the claim segment for the
//! law; this module is the executable surface:
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

/// Fated noise — the KRNG. The source of ALL stochasticity in the world, and a pure
/// function of (seed, key): no wall-clock, no stream, no shared mutable state. It keeps
/// the ordinum's Phase-3 `seeded-asymmetry` promise (the world is "Divided"), which the
/// ordinum audit showed was `:kept-by noise` — a nomos the registry did not have.
pub static NOISE: NomosDecl = NomosDecl {
    name: "noise",
    version: "noise-2026-07-12a-krng",
    system: "fated-noise",
    approach: Approach::Statistical,
    earth_fidelity: Tier::None, // no Earth process — heterogeneity with no provenance
    physics: Tier::None,        // pure noise; conserves nothing, models nothing
    relation: "#mech stand-in: determinism-as-ontology. Genuine CHANCE to anything inside the world, a deterministic lookup to us (ASF.md §2: the aleatoric boundary is frame-relative). It is honest heterogeneity, not a mechanism",
    status: "built; the whole world's replayability rests on it (same (seed,key) → same value, forever). Keeps ordinum Phase-3 promise[seeded-asymmetry]",
    deps: &[],
    consumes: &[],
    promises: &[Promise { quantity: flux::SEEDED_ASYMMETRY, conservation: Conservation::NotTracked }],
    assumptions: &["fBm shape"],
};

/// The planet itself — tilt, spin, orbit, and the insolation rhythm that falls out of
/// them. Real, principled physics (`planet.rs`: declination + hour-angle + latitude) that
/// was NEVER DECLARED — found 2026-07-12 by the ordinum audit, which showed the ladder's
/// Phase-2 `axial-rhythms` promise is `:kept-by planet`, a nomos the registry did not have.
pub static PLANET: NomosDecl = NomosDecl {
    name: "planet",
    version: "planet-2026-07-12a",
    system: "planetary-rhythms",
    approach: Approach::Analytic,
    earth_fidelity: Tier::High, // earth-ref tilt/spin/orbit; the geometry is Earth's
    physics: Tier::High,        // exact sun-position identities — no model, just geometry
    relation: "exact geometry: solar declination from axial tilt + year-fraction, hour angle from spin, zenith from latitude. A circular-orbit approximation (no eccentricity/S(t) yet) — that is the only fudge",
    status: "keeps the ordinum's Phase-2 promise[axial-rhythms] (predicate: insolation, declination and hour-angle correct for date and latitude); drives the globe's real terminator/seasons",
    deps: &[],
    consumes: &[],
    promises: &[Promise { quantity: flux::INSOLATION, conservation: Conservation::NotTracked }],
    assumptions: &["axial tilt", "planet mass"],
};

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

/// The climate nomos — precipitation as the atmosphere reservoir's throughput
/// (`crate::climate`): the FLOW that makes the hydrosphere a cycle. A **field**
/// nomos fed by a **box** (hydrosphere) — the first cross-representation-kind
/// coupling in the flux web. v0 is globally uniform (global-mean only).
pub static CLIMATE: NomosDecl = NomosDecl {
    name: "climate",
    version: "climate-2026-07-12b-jitter",
    system: "precipitation",
    approach: Approach::Statistical, // an identity (stock/residence) + fated variance about it
    earth_fidelity: Tier::Low,       // global mean order-correct (~1 m/yr); the PATTERN is noise, not Earth's
    physics: Tier::Low,              // conserving throughput (precip = evap steady-state); no circulation/EBM
    relation: "conservation flow: precip = atmosphere stock / residence time, with fated MEAN-PRESERVING low-frequency jitter about it. The MAGNITUDE of variance is meant (uniform rain is a physically impossible state); the PATTERN is fated noise, NOT meteorology — no ITCZ, orography, or latitude bands. Reading geography off this precipitation is FALSE",
    status: "v0 — earth-ref residence time + declared jitter amplitude (ASSUMPTIONS); mean ~1 m/yr order-checked; jitter unit-tested as fated, mean-preserving (conservation intact) and spatially correlated (not white noise). NEXT RUNG: the real first-order structure is LATITUDINAL (ITCZ/Hadley from rotation + the insolation we already have)",
    deps: &[&HYDROSPHERE],
    consumes: &[flux::ATMOSPHERE_WATER],
    promises: &[Promise { quantity: flux::PRECIPITATION, conservation: Conservation::Conserved }],
    assumptions: &["atmosphere residence time", "precip jitter"],
};

/// System #1 — the fBm coarse initial-topography (surface prior on the sphere).
pub static INITIAL_TOPOGRAPHY: NomosDecl = NomosDecl {
    name: "initial-topography",
    version: "initial-topography-2026-07-10b-sphere3d",
    system: "initial-topography",
    approach: Approach::Analytic,
    earth_fidelity: Tier::None, // no Earth process; hypsometry measured wrong for every era
    physics: Tier::None,        // pure coordinate noise; conserves nothing
    relation: "#mech stand-in: fBm bathymetry (seafloor relief) — NOT freeboard. Abyssal land is freeboard/isostasy (uplift nomos), never this prior alone (water-world-is-the-promise-not-the-bug)",
    status: "bathymetry prior (gen::bathymetry_m); sphere-continuous; land fraction at derived sea without freeboard is ~0 (Protogenic). Freeboard is uplift's job",
    deps: &[&NOISE],
    consumes: &[flux::SEEDED_ASYMMETRY],
    promises: &[Promise { quantity: flux::SURFACE_ELEVATION, conservation: Conservation::NotTracked }],
    assumptions: &["continental band", "mountain band", "fBm shape"],
};

/// The tectonic driver — rock-uplift rate **and** Abyssal freeboard stand-in.
/// Freeboard keeps `emerged land` (ordinum gate); full isostasy/lithosphere later.
pub static UPLIFT: NomosDecl = NomosDecl {
    name: "uplift-tile",
    version: "uplift-2026-07-23a-freeboard",
    system: "tectonic-uplift",
    approach: Approach::Analytic,
    earth_fidelity: Tier::None, // no Earth tectonic history — freeboard is a stand-in
    physics: Tier::None,        // no true isostasy yet; zero-mean freeboard + rate fBm
    relation: "#mech stand-in: (1) epoch uplift rate × low-freq fBm; (2) zero-mean freeboard (m) as isostatic buoyancy proxy — can go negative. Real path: lithosphere column thickness×density → isostasy (early-continents / Flament–Chowdhury lineage). Keeps Abyssal emerged-land as a flux promise so erosion is not silently seabed",
    status: "v0: freeboard + rate fields (uplift.rs); freeboard earns few-% land after derived sea pour; not full plate tectonics",
    deps: &[&NOISE],
    consumes: &[flux::SEEDED_ASYMMETRY],
    promises: &[
        Promise {
            quantity: flux::ROCK_UPLIFT_RATE,
            conservation: Conservation::NotTracked,
        },
        Promise {
            quantity: flux::EMERGED_LAND,
            conservation: Conservation::NotTracked,
        },
    ],
    assumptions: &["uplift rate", "freeboard amplitude"],
};

/// System #2 — fluvial erosion composed on the initial-topography.
pub static EROSION: NomosDecl = NomosDecl {
    name: "erosion-tile",
    version: "erosion-2026-07-12b-uplift", // now consumes the uplift nomos's field
    system: "fluvial-erosion",
    approach: Approach::Procedural,
    earth_fidelity: Tier::Med, // stream-power/Davy–Lague are how real landscapes are modeled
    physics: Tier::Med,        // real process laws, uncalibrated rates, hardcoded edge policy
    relation: "mechanistic-causal (stream-power incision + deposition + talus + creep), on a stand-in substrate",
    status: "kernel probe-verified in the testbench (channel_profile, spike_probe, armor_regimes 1/3); tile form has fixed epochs (no convergence-ε — component E) and non-composable edges (plan Phase-3)",
    deps: &[&INITIAL_TOPOGRAPHY, &UPLIFT, &CLIMATE],
    // Three needs, all now MET: the surface it carves (→ INITIAL_TOPOGRAPHY), the rock-uplift
    // rate it carves AGAINST (→ UPLIFT), and the rain that drives incision
    // (→ CLIMATE, the precipitation throughput of the conserved reservoir).
    // At v0 climate is UNIFORM, so erosion consumes it as a discharge WEIGHT
    // (precip/mean = 1 everywhere → no behaviour change); when climate gains
    // geography the weight varies and incision follows the rain.
    // FOUR needs. Three are met — and the fourth is the one that was never DECLARED:
    // fluvial erosion cannot carve a landscape that is under water. `emerged land` is
    // the ordinum's Abyssal gate-charge (`emergent-land`: "delivered by uplift /
    // proto-tectonic processes, NEVER an initial condition"), and NOTHING keeps it.
    // Declaring the need is what makes the audit tell the truth: the world is in its
    // Phase-2 water-covered state, so erosion is UNRUNNABLE here — rather than silently
    // no-op'ing on seabed, which is exactly what it has been doing.
    consumes: &[flux::SURFACE_ELEVATION, flux::ROCK_UPLIFT_RATE, flux::PRECIPITATION, flux::EMERGED_LAND],
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
    deps: &[&EROSION, &CLIMATE],
    // Settles on the eroded bed (→ EROSION) and rains the climate nomos's
    // precipitation (→ CLIMATE) — a PRINCIPLED rate now (the conserved reservoir's
    // throughput, ~1 m/yr), not a conjured fudge. The old ~1000× "rain rate" is
    // decomposed: the physical rate comes from climate; a declared bounded-fill
    // ACCELERATION (ASSUMPTIONS) still speeds the fixed-step fill pending the
    // analytic hydrological init.
    consumes: &[flux::ERODED_SURFACE, flux::PRECIPITATION],
    promises: &[Promise { quantity: flux::STANDING_WATER_DEPTH, conservation: Conservation::Conserved }],
    assumptions: &["bounded-fill acceleration", "atmosphere store", "water fill steps", "SEA_LEVEL_M"],
};

/// Every nomos there is. A store root whose name is not here is a bug.
pub static NOMOTHEKE: &[&NomosDecl] = &[&NOISE, &PLANET, &HYDROSPHERE, &CLIMATE, &INITIAL_TOPOGRAPHY, &UPLIFT, &EROSION, &WATER];

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
        assert!(INITIAL_TOPOGRAPHY.key().as_str().starts_with("initial-topography@initial-topography-2026-07-10b-sphere3d"));
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
    fn key_with_dep_versions_embeds_each_dep_identity() {
        // Same shape as consumed⇒in-deps, for keys: a complete key built via
        // `with_dep_versions` must name every direct dep. Catches the class of
        // under-keying where NOISE/HYDROSPHERE versions were omitted while
        // still listed in `deps` (2026-07-21 auditor measurement).
        use crate::store::Key;
        for n in NOMOTHEKE {
            let k = Key::new(n.name, n.version).with_dep_versions(n);
            let s = k.as_str();
            for d in n.deps {
                assert!(
                    s.contains(&format!("{}={}", d.name, d.version)),
                    "{}: key {s:?} missing dep identity field {}={}",
                    n.name,
                    d.name,
                    d.version
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
    fn precipitation_is_now_produced_by_climate() {
        // Was the live UNMET specimen ("rain without a sky"). The climate nomos —
        // the conserved reservoir's throughput — now produces it, closing the
        // water chain (hydrosphere → climate → precipitation → erosion + water).
        // Integration is replacement: the specimen resolved, so the assertion
        // flips from "no producer" to "produced by climate".
        let producers: Vec<_> =
            NOMOTHEKE.iter().filter(|n| n.promises.iter().any(|p| p.quantity == flux::PRECIPITATION)).map(|n| n.name).collect();
        assert_eq!(producers, vec!["climate"], "precipitation is produced by the climate nomos");
        let consumers: Vec<_> =
            NOMOTHEKE.iter().filter(|n| n.consumes.contains(&flux::PRECIPITATION)).map(|n| n.name).collect();
        assert_eq!(consumers, vec!["erosion-tile", "water-tile"], "erosion and water drink it");
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
