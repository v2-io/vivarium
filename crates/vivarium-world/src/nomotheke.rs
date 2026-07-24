//! The nomotheke — the registry of nomos, as *data* (Joseph, 2026-07-10: "I
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
//! **The declaration boxes (2026-07-24).** The 2026-07-13 audit's demand —
//! "the theory demands declarations the data model cannot hold" — is closed at
//! the data-model level: box ② geometry/regime assumptions ([`Assumes`] +
//! [`Delivered`]), box ③ statistic + exactness on every promise and consume
//! ([`Statistic`] — with `Undeclared` as the honest value for the open
//! column-semantics fork — and [`Exactness`]), box ④ the structure ledger
//! ([`StructureDecl`] over the closed [`STRUCTURES`] vocabulary), box ⑤ added
//! unphysical terms with parity + bias/noise verdict ([`UnphysicalTerm`];
//! sign-definite ⇒ Bias is enforced by test), plus discretisation [`Family`],
//! [`ExecutionClass`], and [`Timescale`] (band + dynamic exponent). The
//! *content* of each declaration is only as good as its source — the current
//! entries carry the measured 07-13 findings by citation, and the audit reads
//! them (`crate::audit::statistic_match_across_registry`).
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

/// Box ③ (`#sketch-nomos-declaration-boxes` · `#form-flux-web` FE(6)) — the
/// statistic a fluxed value guarantees (producer side) or needs (consumer
/// side). `Undeclared` is a first-class honest value: the column-semantics
/// fork (point-sample vs cell-average vs band-limited) is OPEN
/// (`DECISIONS[column-is-a-control-volume-with-sufficient-statistics]`), and
/// declaring it here by code fiat would close it silently. `Undeclared` keeps
/// the fork visible to the audit instead of buried in a `Vec<f32>`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Statistic {
    /// One global scalar (a reservoir stock) — no spatial statistic at stake.
    GlobalScalar,
    /// Cell-integrated conserved total (finite-volume reading).
    ConservedTotal,
    /// Mean (area statistic / mean rate).
    Mean,
    Max,
    Min,
    /// Point sample at the cell/column centre (finite-difference reading).
    CenterSample,
    /// Deliberately band-limited to the level's Nyquist (anti-aliased field).
    BandLimited,
    /// The semantics fork is open for this quantity — named debt, not silence.
    Undeclared,
}

/// Exactness of a statistic guarantee (box ③'s second half). `Approximate`
/// carries its error story in the surrounding declaration prose/note.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exactness {
    Exact,
    Approximate,
}

/// One quantity a nomos hands forward (its slice of a phase Promise), with its
/// conservation stance and the statistic + exactness it guarantees (box ③).
pub struct Promise {
    pub quantity: &'static str,
    pub conservation: Conservation,
    pub statistic: Statistic,
    pub exactness: Exactness,
}

/// One quantity a nomos needs, with the statistic it needs it *as* (box ③,
/// consumer side). Quantity-name match alone is necessary but not sufficient
/// (`#form-flux-web` FE(6)): a consumer reading a mean where the producer
/// guarantees a point sample is Unsound, not merely unlucky.
pub struct Consume {
    pub quantity: &'static str,
    pub needs: Statistic,
}

/// Box ② — one assumption the algorithm makes about its cells / regime, and
/// whether this world actually delivers it. The defects of 2026-07-13 were all
/// found by hand because nothing declared these (`#detail-nomos-defect-anatomy`).
pub struct Assumes {
    pub assumption: &'static str,
    pub delivered: Delivered,
    pub note: &'static str,
}

/// The verdict on an [`Assumes`] row for our grid/world.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Delivered {
    Holds,
    Violated,
    Unexamined,
}

/// Discretisation family (`#form-declared-structure-tradeoff` FE(4)): one
/// world, several families — the declaration is *which*, and the seams between
/// families are where structure dies. A nomos may honestly be more than one
/// (erosion: collocated diffusion + a graph router).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Family {
    /// Pointwise analytic function of coordinates — no stencil, no
    /// discretisation structure at stake.
    PointwiseAnalytic,
    /// Global box/reservoir — no grid at all.
    Reservoir,
    /// Box output broadcast onto a field.
    BoxToField,
    /// Collocated grid scheme (all quantities at cell centres).
    Collocated,
    /// Staggered (Arakawa-C-like) flux-form scheme.
    StaggeredFluxForm,
    /// Graph/fan routing scheme — not a finite-volume method.
    GraphRouter,
}

/// Box ④ — the structure ledger: what this scheme preserves exactly, what
/// approximately, and what it sacrifices. Entries come from [`STRUCTURES`]
/// (closed vocabulary — a typo may not masquerade as a novel invariant), and
/// the three lists are disjoint by test. Absent from all three lists =
/// unexamined for this nomos, which the declaration does not hide as "fine."
pub struct StructureDecl {
    pub preserves_exact: &'static [&'static str],
    pub preserves_approx: &'static [&'static str],
    pub sacrifices: &'static [&'static str],
}

/// The closed structure vocabulary (box ④). Grown only alongside a nomos that
/// declares the entry — same scope discipline as [`crate::flux::VOCABULARY`].
pub const STRUCTURES: &[&str] = &[
    "local-conservation",
    "positivity",
    "well-balanced",
    "acyclicity",
    "rotational-isotropy",
    "contour-orthogonality",
    "entropy-condition",
    "monotonicity",
];

/// Box ⑤ — an unphysical term the scheme adds, with the shape the modified
/// equation gives it and its bias-vs-noise verdict (`#disc-prime-question` ·
/// `#norm-bias-vs-noise`). "A sign-definite operation cannot average out ⇒ it
/// is a bias by construction" is enforced by test, not left to discipline.
pub struct UnphysicalTerm {
    pub term: &'static str,
    pub parity: Parity,
    pub verdict: ErrVerdict,
    pub note: &'static str,
}

/// The differential/structural shape of an added term.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parity {
    /// Even-order residual — reads as numerical diffusion.
    Even,
    /// Odd-order residual — reads as dispersion / spurious advection.
    Odd,
    /// One-sided clip/clamp/limiter — sign-definite by construction.
    SignDefinite,
    /// Directional/topological deflection (grid-locked attractors, spirals).
    Directional,
}

/// Bias-vs-noise adjudication of an added term (`#norm-bias-vs-noise`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrVerdict {
    Bias,
    Noise,
    Unadjudicated,
}

/// Execution class — how this nomos runs relative to the coupling schedule
/// (declaration debt named in `#form-kernel-imperative-boundary` FE(5) and
/// `#form-fidelity-ladder` FE(5); now a field, not prose).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExecutionClass {
    /// Rare, deep batch runs; results cached toward zero marginal cost.
    BatchDeep,
    /// Relax-to-attractor, then quiescent until forcing changes.
    Relaxation,
    /// Closed-form / surrogate, evaluated constantly and cheaply.
    ProceduralTight,
}

/// Timescale declaration: which multirate band this nomos lives in, and its
/// dynamic exponent where one applies ( #sketch-dynamic-exponent-seams — z is
/// a property of the process; `None` for analytic/box articles with no
/// stability-bound scaling).
pub struct Timescale {
    pub band: &'static str,
    pub z: Option<u8>,
}

/// Closed band vocabulary (coupling-order bands; DESIGN-SYSTEMS lineage).
pub const BANDS: &[&str] =
    &["timeless-analytic", "deep-driver", "orbital-climate", "surface-process", "fast-hydrological"];

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
    /// Upstream nomos whose outputs this one consumes.
    pub deps: &'static [&'static NomosDecl],
    /// The fluxed quantities this nomos *needs* (in), drawn from the shared
    /// [`crate::flux`] vocabulary. This is the quantity-level half of the
    /// coupling contract: `deps` names the *nomos* an occupant depends on;
    /// `consumes` names the *quantities* the role depends on, independent of who
    /// supplies them. The audit ([`crate::audit`]) matches each consumed
    /// quantity to a producer (`Promise::quantity`) — a match is an edge, a
    /// miss is the honest "rain without a sky" finding, and a permit may later
    /// license a miss. A consumed quantity that *is* met by a nomos must have
    /// that nomos in `deps` (else the complete key would omit its version) —
    /// pinned by a test. (`#detail-regula-design` §3;
    /// `VIVARIA-DEFINITIONS.md` §"the web".)
    pub consumes: &'static [Consume],
    /// What it hands forward.
    pub promises: &'static [Promise],
    /// Verbatim anchors into `ASSUMPTIONS.md` — every magic constant this
    /// nomos leans on. Checked against the ledger by test.
    pub assumptions: &'static [&'static str],
    /// Box ② context + `#form-declared-structure-tradeoff` FE(4): which
    /// discretisation family/families this article is.
    pub family: &'static [Family],
    /// Box ② — assumptions about cells/regime vs what the world delivers.
    pub assumes_geometry: &'static [Assumes],
    /// Box ④ — preserved-exact / preserved-approx / sacrificed structures.
    pub structure: StructureDecl,
    /// Box ⑤ — added unphysical terms, each with parity + bias/noise verdict.
    pub unphysical_terms: &'static [UnphysicalTerm],
    /// Execution class relative to the coupling schedule.
    pub execution: ExecutionClass,
    /// Multirate band + dynamic exponent where one applies.
    pub timescale: Timescale,
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
    promises: &[Promise {
        quantity: flux::SEEDED_ASYMMETRY,
        conservation: Conservation::NotTracked,
        statistic: Statistic::CenterSample,
        exactness: Exactness::Exact, // pure function of (seed, key): exact at every sample
    }],
    assumptions: &["fBm shape"],
    family: &[Family::PointwiseAnalytic],
    assumes_geometry: &[],
    structure: StructureDecl { preserves_exact: &[], preserves_approx: &[], sacrifices: &[] },
    unphysical_terms: &[],
    execution: ExecutionClass::ProceduralTight,
    timescale: Timescale { band: "timeless-analytic", z: None },
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
    promises: &[Promise {
        quantity: flux::INSOLATION,
        conservation: Conservation::NotTracked,
        statistic: Statistic::CenterSample,
        exactness: Exactness::Exact, // exact sun-position identities at the evaluated point
    }],
    assumptions: &["axial tilt", "planet mass"],
    family: &[Family::PointwiseAnalytic],
    assumes_geometry: &[Assumes {
        assumption: "circular orbit (no eccentricity / S(t))",
        delivered: Delivered::Holds, // the world's orbit IS what this nomos defines; the fudge is vs Earth, not vs the vivium
        note: "earth-fidelity cost, not a scheme defect",
    }],
    structure: StructureDecl { preserves_exact: &[], preserves_approx: &[], sacrifices: &[] },
    unphysical_terms: &[],
    execution: ExecutionClass::ProceduralTight,
    timescale: Timescale { band: "timeless-analytic", z: None },
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
    promises: &[Promise {
        quantity: flux::ATMOSPHERE_WATER,
        conservation: Conservation::Conserved,
        statistic: Statistic::GlobalScalar,
        exactness: Exactness::Exact, // partition exact by construction; unit-tested residual < 1e-6 km³
    }],
    assumptions: &["water mass fraction", "atmosphere fraction", "planet mass"],
    family: &[Family::Reservoir],
    assumes_geometry: &[],
    structure: StructureDecl {
        preserves_exact: &["local-conservation"], // the whole article IS a conservation law
        preserves_approx: &[],
        sacrifices: &[],
    },
    unphysical_terms: &[],
    execution: ExecutionClass::ProceduralTight,
    timescale: Timescale { band: "timeless-analytic", z: None }, // v0 static partition; becomes a cycle with flows
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
    consumes: &[Consume { quantity: flux::ATMOSPHERE_WATER, needs: Statistic::GlobalScalar }],
    promises: &[Promise {
        quantity: flux::PRECIPITATION,
        conservation: Conservation::Conserved,
        statistic: Statistic::Mean,
        // Mean preservation is unit-tested only to ±7% (0.93..1.07) and climate.rs's
        // own doc says "preserved in expectation; exact global closure is a probe
        // worth writing"; the jitter factor also carries a sign-definite .max(0.0)
        // clip. Approximate until the jitter is normalized over its domain.
        exactness: Exactness::Approximate,
    }],
    assumptions: &["atmosphere residence time", "precip jitter"],
    family: &[Family::BoxToField],
    assumes_geometry: &[],
    structure: StructureDecl {
        preserves_exact: &[],
        // the steady-state identity (stock/residence) is exact; the jittered field
        // conserves only in expectation (±7% test bound) — same debt as the
        // Approximate statistic above
        preserves_approx: &["local-conservation"],
        sacrifices: &[],
    },
    unphysical_terms: &[],
    execution: ExecutionClass::ProceduralTight,
    timescale: Timescale { band: "fast-hydrological", z: None },
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
    consumes: &[Consume { quantity: flux::SEEDED_ASYMMETRY, needs: Statistic::CenterSample }],
    promises: &[Promise {
        quantity: flux::SURFACE_ELEVATION,
        conservation: Conservation::NotTracked,
        statistic: Statistic::BandLimited, // gen band-limits fBm to the level's Nyquist — the deliberate anti-aliased reading
        exactness: Exactness::Exact,
    }],
    assumptions: &["continental band", "mountain band", "fBm shape"],
    family: &[Family::PointwiseAnalytic],
    assumes_geometry: &[],
    structure: StructureDecl { preserves_exact: &[], preserves_approx: &[], sacrifices: &[] },
    unphysical_terms: &[],
    execution: ExecutionClass::ProceduralTight,
    timescale: Timescale { band: "timeless-analytic", z: None },
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
    consumes: &[Consume { quantity: flux::SEEDED_ASYMMETRY, needs: Statistic::CenterSample }],
    promises: &[
        Promise {
            quantity: flux::ROCK_UPLIFT_RATE,
            conservation: Conservation::NotTracked,
            statistic: Statistic::CenterSample, // pointwise fBm evaluation
            exactness: Exactness::Exact,
        },
        Promise {
            quantity: flux::EMERGED_LAND,
            conservation: Conservation::NotTracked,
            statistic: Statistic::CenterSample,
            exactness: Exactness::Exact,
        },
    ],
    assumptions: &["uplift rate", "freeboard amplitude"],
    family: &[Family::PointwiseAnalytic],
    assumes_geometry: &[],
    structure: StructureDecl { preserves_exact: &[], preserves_approx: &[], sacrifices: &[] },
    unphysical_terms: &[],
    execution: ExecutionClass::ProceduralTight,
    timescale: Timescale { band: "deep-driver", z: None },
};

/// System #2 — fluvial erosion composed on the initial-topography.
pub static EROSION: NomosDecl = NomosDecl {
    name: "erosion-tile",
    version: "erosion-2026-07-23a-true-cell-area", // spherical A for drainage/deposit; not MFD lengths
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
    consumes: &[
        // Elevation-class reads are Undeclared on purpose: the column-semantics
        // fork (point vs mean vs band-limited) is OPEN and is resolved by
        // decision, not by a code default. Undeclared = the fork, visible.
        Consume { quantity: flux::SURFACE_ELEVATION, needs: Statistic::Undeclared },
        Consume { quantity: flux::ROCK_UPLIFT_RATE, needs: Statistic::CenterSample },
        Consume { quantity: flux::PRECIPITATION, needs: Statistic::Mean }, // discharge weight off the mean rate
        Consume { quantity: flux::EMERGED_LAND, needs: Statistic::CenterSample }, // pointwise land mask
    ],
    promises: &[Promise {
        quantity: flux::ERODED_SURFACE,
        conservation: Conservation::ExportsAtBoundary,
        statistic: Statistic::Undeclared, // the live three-way ambiguity — mean-pin read it as mean, voxel bilinear as point-sample
        exactness: Exactness::Approximate,
    }],
    assumptions: &["stream-power `m`", "erosion `k_dt`", "erosion run length"],
    family: &[Family::Collocated, Family::GraphRouter], // creep is collocated diffusion; MFD routing is a graph scheme, not FVM
    assumes_geometry: &[
        Assumes {
            assumption: "8 Moore neighbours are evenly-spaced quadrature nodes",
            delivered: Delivered::Violated,
            note: "measured: fan collapses to 2 attractors off face-centre; does not converge (L5→L23); DECISIONS[mfd-fan-is-a-bias-and-does-not-converge]",
        },
        Assumes {
            assumption: "diagonal neighbours share a face for flux",
            delivered: Delivered::Violated,
            note: "they share only a vertex — 47.8% phantom flux; makes MFD un-correctable as FV; DECISIONS[the-router-is-a-scalar…] ⑤",
        },
        Assumes {
            assumption: "uniform cell area and neighbour distances",
            delivered: Delivered::Violated,
            note: "true spherical A now used for drainage seed + deposit volume (2026-07-23 PoC); MFD split lengths still uniform cell_m — partial",
        },
    ],
    structure: StructureDecl {
        preserves_exact: &["acyclicity"], // strictly-downhill clamp — recorded as a mystery on 07-13; it is a structure
        preserves_approx: &["local-conservation"], // deposition/export accounted at boundary, not exact per-cell
        sacrifices: &["rotational-isotropy", "contour-orthogonality"], // both measured: fan attractors; κ≈2e-2 spiral
    },
    unphysical_terms: &[
        UnphysicalTerm {
            term: "MFD fan deflection (sheared quadrature)",
            parity: Parity::Directional,
            verdict: ErrVerdict::Bias,
            note: "474 km plume drift on a cone; refining worsens it",
        },
        UnphysicalTerm {
            term: "p = 1.1 outflow exponent",
            parity: Parity::Directional,
            verdict: ErrVerdict::Bias,
            note: "45°-periodic grid-locked first-moment deflection (rms 0.2419°, eight attractors); exactly zero at p = 1 — a theorem. Live at erosion.rs P",
        },
        UnphysicalTerm {
            term: "Priority-Flood ε-fill (mass minted in depressions)",
            parity: Parity::SignDefinite,
            verdict: ErrVerdict::Bias,
            note: "fill_depressions raises cells with no debit, every epoch — can only add; volume per epoch unprobed",
        },
        UnphysicalTerm {
            term: "creep diffusion-number clamp k ≤ 0.24",
            parity: Parity::SignDefinite,
            verdict: ErrVerdict::Bias,
            note: "one-sided under-diffusion at fine levels instead of z=2 sub-stepping — declared fidelity compromise ( #sketch-dynamic-exponent-seams )",
        },
    ],
    execution: ExecutionClass::BatchDeep,
    timescale: Timescale { band: "surface-process", z: Some(2) }, // creep carries the parabolic bound (clamped, not sub-stepped)
};

/// System #3 — conserved shallow water settling on the eroded bed.
pub static WATER: NomosDecl = NomosDecl {
    name: "water-tile",
    version: "water-2026-07-10a", // predates the 07-23 sea_m default change — parameter-vs-algorithm versioning rule unwritten (hand-stamp debt, #form-complete-content-addressed-key)
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
    consumes: &[
        Consume { quantity: flux::ERODED_SURFACE, needs: Statistic::Undeclared }, // the open column fork, consumer side
        Consume { quantity: flux::PRECIPITATION, needs: Statistic::Mean },
    ],
    promises: &[Promise {
        quantity: flux::STANDING_WATER_DEPTH,
        conservation: Conservation::Conserved,
        statistic: Statistic::Undeclared, // depth = surface − bed: inherits the bed's open semantics; depth does NOT interpolate
        exactness: Exactness::Approximate,
    }],
    assumptions: &["bounded-fill acceleration", "atmosphere store", "water fill steps", "SEA_LEVEL_M"],
    family: &[Family::StaggeredFluxForm], // per-face pipe fluxes kept between steps — an undeclared asset until 2026-07-13
    assumes_geometry: &[
        Assumes {
            assumption: "hydrostatic reconstruction valid (Δz < h)",
            delivered: Delivered::Violated,
            note: "proved failure for thin films on steep ground — our common case; level-dependent bias that will masquerade as a seam defect",
        },
        Assumes {
            assumption: "local-inertial momentum valid (published envelope Fr < 1)",
            delivered: Delivered::Violated,
            note: "~5.7% of wet cells supercritical (Fr > 1.5, the measured threshold) on eroded land; Froude cap measured SATURATED (max Fr ≡ 2.00 bit-identical — instrument reads its own clamp); DECISIONS[water-runs-outside-its-published-validity-envelope]",
        },
        Assumes {
            assumption: "uniform cell spacing (cell_m as one length)",
            delivered: Delivered::Unexamined,
            note: "same Jacobian class as erosion's; magnitude on water unmeasured",
        },
    ],
    structure: StructureDecl {
        preserves_exact: &["local-conservation", "well-balanced", "positivity"], // conservation unit-tested; no-null-space measured 2026-07-13; well-balanced measured 2026-07-24 (`water_structures wb`: lake-at-rest incl. partially-dry, f64 exact, f32 at the ULP conditioning floor — LAKE-AT-REST only; moving-water WB is proposed sacrificed per DECISIONS[hydrostatic-reconstruction-fails…]); positivity held by a clamp that is itself a declared term below
        preserves_approx: &[],
        sacrifices: &["entropy-condition"], // roll-wave/shock structure the scheme cannot render; θ suppresses the symptom
    },
    unphysical_terms: &[
        UnphysicalTerm {
            term: "θ = 0.8 flux smoothing (Lax–Friedrichs-class)",
            parity: Parity::Even,
            verdict: ErrVerdict::Bias,
            note: "physical claim NONE (paper's own words: artificial numerical diffusion); preserves flux mean, destroys flux variance (−24% @ 200 passes) ⇒ Jensen under-prediction in superlinear consumers. Hardcoded literal water.rs:385",
        },
        UnphysicalTerm {
            term: "flux clip accel.max(0.0)",
            parity: Parity::SignDefinite,
            verdict: ErrVerdict::Bias,
            note: "one-sided by construction; activation rate unprobed",
        },
        UnphysicalTerm {
            term: "Froude breaking cap (f ≤ 2√(gh)·h·l)",
            parity: Parity::SignDefinite,
            verdict: ErrVerdict::Bias,
            note: "measured saturated on eroded land — the cap is doing steady work, not guarding an edge case",
        },
        UnphysicalTerm {
            term: "outflow scale-down",
            parity: Parity::SignDefinite,
            verdict: ErrVerdict::Bias,
            note: "one-sided by construction; activation rate unprobed",
        },
        UnphysicalTerm {
            term: "depth positivity clamp .max(0.0)",
            parity: Parity::SignDefinite,
            verdict: ErrVerdict::Bias,
            note: "silent mass source if it fires; the price of the 'positivity' row above; activation unprobed",
        },
    ],
    execution: ExecutionClass::Relaxation,
    timescale: Timescale { band: "fast-hydrological", z: Some(1) }, // genuine CFL: stable_dt recomputed per burst
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
        assert!(EROSION.key().as_str().starts_with("erosion-tile@erosion-2026-07-23a-true-cell-area"));
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
            for c in n.consumes {
                assert!(
                    flux::is_in_vocabulary(c.quantity),
                    "{}: consumed quantity {:?} is not in flux::VOCABULARY",
                    n.name,
                    c.quantity
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
            for c in n.consumes {
                let q = c.quantity;
                if let Some(producer) = NOMOTHEKE.iter().find(|m| m.promises.iter().any(|p| p.quantity == q)) {
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
            NOMOTHEKE.iter().filter(|n| n.consumes.iter().any(|c| c.quantity == flux::PRECIPITATION)).map(|n| n.name).collect();
        assert_eq!(consumers, vec!["erosion-tile", "water-tile"], "erosion and water drink it");
    }

    #[test]
    fn structure_vocabulary_is_closed_and_lists_disjoint() {
        // Box ④: a structure name not in STRUCTURES is a typo masquerading as a
        // novel invariant; a structure in two lists is a contradiction.
        for n in NOMOTHEKE {
            let lists = [n.structure.preserves_exact, n.structure.preserves_approx, n.structure.sacrifices];
            for list in lists {
                for s in list {
                    assert!(STRUCTURES.contains(s), "{}: structure {s:?} not in STRUCTURES vocabulary", n.name);
                }
            }
            for (i, a) in lists.iter().enumerate() {
                for b in &lists[i + 1..] {
                    for s in *a {
                        assert!(!b.contains(s), "{}: structure {s:?} appears in two lists — a contradiction", n.name);
                    }
                }
            }
        }
    }

    #[test]
    fn sign_definite_terms_are_biases_by_construction() {
        // `#norm-bias-vs-noise`: a sign-definite operation cannot average out —
        // its verdict is Bias, mechanically, not by discipline.
        for n in NOMOTHEKE {
            for t in n.unphysical_terms {
                if t.parity == Parity::SignDefinite {
                    assert_eq!(
                        t.verdict,
                        ErrVerdict::Bias,
                        "{}: sign-definite term {:?} must be declared Bias — it cannot average out",
                        n.name,
                        t.term
                    );
                }
            }
        }
    }

    #[test]
    fn every_nomos_declares_family_and_a_known_band() {
        for n in NOMOTHEKE {
            assert!(!n.family.is_empty(), "{}: no discretisation family declared", n.name);
            assert!(BANDS.contains(&n.timescale.band), "{}: band {:?} not in BANDS", n.name, n.timescale.band);
        }
    }

    #[test]
    fn statistic_match_audit_finds_no_unsound_edges_and_names_the_open_fork() {
        // `#form-flux-web` FE(6)'s instrument, run over the live registry:
        // no declared-and-mismatched (Unsound) edge exists, and the Undeclared
        // set is EXACTLY the elevation-class edges — the open column-semantics
        // fork, surfaced by name rather than buried in a Vec<f32>.
        use crate::audit::{statistic_match_across_registry, StatMatch};
        let findings = statistic_match_across_registry();
        let unsound: Vec<_> =
            findings.iter().filter(|(_, _, _, v)| *v == StatMatch::Unsound).map(|(n, q, _, _)| (n.name, *q)).collect();
        assert!(unsound.is_empty(), "declared-statistic mismatches: {unsound:?}");
        let mut undeclared: Vec<_> =
            findings.iter().filter(|(_, _, _, v)| *v == StatMatch::Undeclared).map(|(n, q, _, _)| (n.name, *q)).collect();
        undeclared.sort();
        assert_eq!(
            undeclared,
            vec![
                ("erosion-tile", flux::SURFACE_ELEVATION),
                ("water-tile", flux::ERODED_SURFACE),
            ],
            "the Undeclared set is the open column-semantics fork, exactly — \
             new Undeclared edges must be deliberate, and closing one is a decision"
        );
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
