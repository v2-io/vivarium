# VIVARIA-DEFINITIONS (spike, 2026-07-12)

*A pseudo-concrete map of the scaffold, written to stop us confusing ourselves. Rule of the spike: **no noun gets defined in the abstract.** Every piece is shown as the actual udon it tracks (or the rust type/function it becomes), with a one-line gloss for the Latin. Where a natural in-between abstraction exists, it gets a plain **IT-register placeholder name** (`manifest-template`, not a fresh Latin coinage) — we seek the real noun only once the thing has proven it needs one. Written in the light of the regula-collapse discussion (regula → order + manifest); this doc is partly how we test whether that collapse survives contact with concreteness. Not built where marked `— NOT BUILT`; those are imagination, honestly flagged, and verified against the real `crates/vivarium-world` signatures so the rust here isn't fabricated.*

**The thesis (Joseph's steer): declare everything you can; run the kernel only.** The measure of this scaffold is how much of each nomos's **epistemology, interactions, and lifecycle** is data, not code — because if it's data, the pieces of a multiscale heterogeneous simulation **interlock declaratively** (erosion declares it needs rain; the water-cycle declares it makes rain from a conserved reservoir; the reservoir declares it needs the planetary inventory), and the honest picture of *what depends on what and how good it is* can be **audited, queried, and composed before any rust runs**. The two centerpieces below — *"the measure"* (§Layer 1) and *"the web"* (after §Layer 2) — are where that thesis is made concrete; everything else is the frame that holds them.

*Reading order is bottom-up: the store is the ground; everything above is data or code that lands in it.*

---

## Layer 0 — the store (the ground; pure rust, not authored)

The world **is** a content-addressed directory. Nothing above this layer is real until it lands here as bytes under a hash.

```udon
; NOT a udon artifact — this is the filesystem shape (DESIGN-REDUX §13).
; <world>/
;   manifest              ; the one authored file that names this world (Layer 3)
;   objects/<hash>        ; immutable results, content-addressed
;   roots                 ; (aspect, tile, level, time) -> current hash
;   mutations/            ; append-only user edits (irreducible truth)
```

```
--> Rust: crates/vivarium-world/src/store.rs  (REAL, today)
    struct Store { root: PathBuf }
    fn put(&self, key: &Key, value: &[u8]) -> io::Result<()>
    fn get(&self, key: &Key) -> Option<Vec<u8>>
    fn roots(&self) -> io::Result<Vec<(String, String)>>
    // Today the store is key -> bytes (the Key IS the address). Full
    // git-shaped content-addressing (objects/<hash> + roots + mutations/) is
    // the DESIGN-REDUX §13 direction; the shape above is what exists.
```

**Key** — the address of any memo. This is where determinism-as-ontology bites: the key names *everything* that affects the result, so equal keys ⇒ equal bytes, forever.

```
--> Rust: struct Key(String)   // minted by NomosDecl::key(); "<nomos>@<version>|…"
```

- **memo** *(gloss + concrete)*: one entry under `objects/<hash>`. "The cached converged state." Concretely: the bytes a nomos produced for a given Key.
- **fated noise / KRNG** *(gloss + concrete)*: the reason the store is sound. All randomness is `hash(seed, key) -> value` (`noise.rs`), never a stream — so re-running a Key reproduces the bytes and caching never changes the world.

---

## Layer 1 — the nomos (executable law; rust code, declared as data)

A **nomos** is the grain the store keys are made of: one keyed, versioned, executable article of world-law. The *doing* is rust (a kernel fn); the *declaration* is data (today a rust struct in `nomotheke.rs`; shown here as the udon it could equally be, since it is pure data). **Gloss:** nomos = "an instituted article of Law"; the plural is nomoi; the registry is the nomotheke.

```udon
; What the nomotheke declares, per nomos. (Today: a NomosDecl literal in Rust.
; Shown as udon to make the shape obvious and because it IS just data.)
|nomos[erosion-tile]
  :version erosion-2026-07-10a
  :fills fluvial-erosion              ; the system-role it occupies (see "slot?" below)
  :kernel Fluvial::erode              ; --> the rust fn that IS the law
  :approach procedural
  :physics med                        ; declared tier (axis B)
  :earth-fidelity med                 ; axis A
  :relation "mechanistic-causal: stream-power + Davy-Lague + talus + creep"
  :depends [spine-tile]              ; nomos->nomos (REAL: the `deps` field)
  :consumes ["surface elevation field (m)"]   ; nomos->QUANTITY — NOT BUILT (queued)
  |promise[eroded-surface].state
    :quantity "eroded surface elevation field (m)"
    :conservation exports-at-boundary  ; the honest conservation claim
  :assumptions ["stream-power `m`" "erosion `k_dt`" "erosion run length"]
```

```
--> Rust: crates/vivarium-world/src/nomotheke.rs  (REAL fields marked)
    struct NomosDecl {
        name, version, system: &str,                     // REAL
        approach: Approach, physics: Tier, earth_fidelity: Tier,  // REAL
        relation: &str, status: &str,                    // REAL
        deps: &[&NomosDecl],                             // REAL (nomos->nomos)
        promises: &[Promise],                            // REAL: Promise { quantity, conservation }
        assumptions: &[&str],                            // REAL (checked vs ASSUMPTIONS.md)
        // consumes: &[&str],                            // NOT BUILT — the quantity-level
        //                                                  flux interface; the permit-void
        //                                                  audit's prerequisite (TODO)
    }
    fn key(&self) -> Key                   // REAL: declaration mints the store key
    fn derived_physics(&self) -> Tier      // REAL: weakest-link fold over deps
```

The load-bearing structural facts (already real): the **declaration mints the key** (an undeclared nomos has no path into world-law), `assumptions` are checked against `ASSUMPTIONS.md` at test time, and `derived_*` folds tier down the dep cone (algorithm quality never launders inputs).

### The measure: how much of a nomos is *declared*, not coded

*(Joseph's steer, 2026-07-12: the more of a nomos's epistemology, interactions, and lifecycle can be declared **declaratively**, the better.)* The target shape: **the kernel fn is the only imperative part; everything else about it is data** — so the whole system is auditable, queryable, and composable from declarations alone, and rust just runs kernels. Scored honestly against what's real today:

```udon
|nomos[erosion-tile]
  ; --- IDENTITY -------------------------------------------- (declarative ✓ real)
  :version erosion-2026-07-10a
  :kernel Fluvial::erode                 ; <-- THE ONE IMPERATIVE ESCAPE HATCH
  ; --- EPISTEMOLOGY ---------------------------------------- (declarative ✓ real)
  :approach procedural  :physics med  :earth-fidelity med
  :relation "mechanistic-causal"  :status "probe-verified in testbench"
  :assumptions ["stream-power `m`" "erosion `k_dt`"]
  ; --- INTERACTIONS (the flux interface) ------------------- (partly real)
  :depends [spine-tile]                  ; ✓ real (deps)
  :consumes ["surface elevation (m)"]    ; ✗ not built — quantities in
  |promise[eroded-surface].state :conservation exports-at-boundary  ; ✓ real (quantities out)
  :execution-class relaxation            ; ✗ not built — batch-deep/relaxation/procedural-tight
  :timescale-band surface-process        ; ✗ not built — which multirate band it couples in
  ; --- LIFECYCLE ------------------------------------------- (partly real / gloss)
  :stage draft                           ; ✗ not built — draft->deps-verified->claims-verified
  |promise[eroded-surface] :maturity claimed  ; ~ derivable (has kept-by, no passing predicate yet)
  :phase abyssal                         ; ✗ not built — which order-phase it serves
```

Everything above the kernel line is (or wants to be) **data the audit reads without running anything**. What's *already* declarative: identity, the four epistemic axes, assumptions, deps, promises-with-conservation, predicates. What's *imperative-but-shouldn't-be* (the backlog this doc surfaces): the **flux interface** (`consumes`/`execution-class`/`timescale-band` — currently implicit in the kernel's rust body), the **stage lifecycle**, and the **phase binding**. Pulling each of those out of the kernel and into the declaration is the concrete meaning of "more declarative is better" — and each one that moves makes a real capability fall out for free: `consumes` → the permit-void audit; `execution-class`+`timescale-band` → automatic multirate coupling; `stage` → promotion tracking; `phase` → the order-conformance map. The kernel keeps only what is irreducibly numeric.

### Kinds of nomoi (only the ones that have earned a distinction)

```udon
; 1. SYSTEM nomos — produces a world aspect. REAL, three of them:
|nomos[spine-tile]   :fills surface-prior    :kernel gen::spine
|nomos[erosion-tile] :fills fluvial-erosion  :kernel Fluvial::erode
|nomos[water-tile]   :fills surface-water    :kernel WaterSim::fill

; 2. PROJECTION nomos — native representation -> canonical CellId frame. — NOT BUILT
;    (a drainage graph or slab-body computed in its own fast shape, projected lazily)
|nomos[drainage-graph->columns]  :fills drainage  :kernel Graph::project   ; imagined

; 3. ANALYSIS nomos — reads world state + an analysis, emits a RESULT-memo. — NOT BUILT
;    Its cone IS its provenance; a flaw upstream flags it by the store's invalidation.
|nomos[hypsometry-of-region]  :reads [eroded-surface]  :kernel probe::hypsometry  ; imagined
```

The three "kinds" are not a typology we impose — they fell out of the four representation kinds (field / reservoir-box / network / agent, ARCHITECTURE §0) plus the observation that *some nomoi produce world, some re-express world, some measure world*. If a fourth kind never earns its keep, it never gets a name.

---

## Layer 2 — the order (the kind-floor; udon, the `ordinum`)

An **order** is the conformance spec a *class of kingdoms of the same kind* is measured against: purely **descriptive** of what that kind of world does, phase by phase. It carries no requirements of any particular world. (Latin `ordinum` = the file/artifact; **"order"** = the plain gloss Joseph reached for — an order for a kind of kingdom. `Ordinatio Ordinum` = "the arrangement of orders," which the word already wants.)

```udon
; tabularium/terrestris.ordinum.udon  (excerpt — the real file has 9 phases)
|ordinum[terrestris]                  ; the order for the earthlike KIND
  :version 0.1.0
  :manifold cube-sphere-3d-voxel      ; a CA testbed would be euclidean-plane-2d
  :reportatio .archive/PHASES.md      ; the prose first-guess it was compiled from

  |phase[abyssal] :num 3
    |charge[emergent-land] :tag gate                 ; a GATING prerequisite
    |promise[erosion-substrate].regime :kept-by erosion-tile :from erosion-carving
      |predicate erosion runs each cycle; its bed is what later phases build on
    |promise[emerged-land].state :from emergent-land ; gloss: no :kept-by yet (honest)
      |predicate land above sea level, non-volcanic; land-fraction toward survey band
    |defeasance :voids "1.water-covered-surface" :by emergent-land
```

```
--> Rust (parsed from udon via libudon):
    struct Ordinum { name, version, manifold, reportatio, phases: Vec<Phase> }
    struct Phase { num, name, charges: Vec<Charge>, promises: Vec<Promise>,
                   defeasances: Vec<Defeasance> }
    struct Promise { slug, kind: Kind, kept_by: Option<NomosRef>,
                     predicates: Vec<String>, from: Option<ChargeRef> }
    enum Kind { State, Regime, Capability, Limit }   // the udon `.class`
```

The pieces, each shown above rather than told:

- **charge** — a gating prerequisite (`:tag gate/earth/mech/emergent` = arrow-kind, never build-status). `Charge ⊆ promises`.
- **promise** — what a phase hands forward: `.class` typed, `:kept-by` a nomos when exact, `|predicate` its falsifiable core, no link = honest **gloss**. Maturity ladder named→specified→claimed→kept; *no fulfillment claim without a predicate.*
- **defeasance** — an explicit retirement of a prior promise (inherited promises continue implicitly). Promises-in-force at N = `⋃promises − ⋃defeasances`.
- **Law (θ)** *(gloss + concrete)*: the totality this order composes into. Concretely: `Law = { the NomosDecl each promise names in :kept-by }`. AAT calls it θ; here it's just "the set of nomoi the order's kept promises resolve to."

**`slot` — a noun on probation.** A "slot" was "a system-role identified by its flux interface." But look at the udon: a promise already names the fluxed quantity (`erosion-substrate` → the eroded-bed flux) and `:kept-by` already names the occupant. So a slot ≈ *a promise's fluxed-quantity contract + its kept-by requirement* — it may not need its own noun at all. **Flagging for dissolution**, not enshrining. (`:fills fluvial-erosion` on the nomos is the only place the role-name still does independent work; keep it as a plain string until it proves it wants to be a type.)

---

## The web — nomoi interlocking by promise and flux (declaratively, before rust runs)

This is the point of the whole scaffold: the pieces of a **multiscale, heterogeneous** simulation compose *as data*, each declaring what it needs and what it hands on, so the requisite chain and its honest epistemics are computable **before any kernel executes**. The coupling contract is exactly one thing — **the fluxed-quantity string must match between a `:produces` and a `:consumes`.** A matched pair is an edge; a typo is a broken edge the audit catches; nothing shared-mutable ever passes between them.

Here is the near-future water/erosion chain, declared. Real nomoi are marked; the honest ones that don't exist yet carry `— NOT BUILT`.

```udon
; ---- a shared flux vocabulary (the ONLY thing nomoi agree on) ----
; "atmosphere water (m w.e.)"  "precipitation (m/yr)"  "surface elevation (m)"
; "eroded surface (m)"  "sediment flux (kg/m2/yr)"  "standing water depth (m)"

|nomos[spine-tile]                              ; REAL — a FIELD nomos
  :kernel gen::spine   :physics none   :relation "#mech stand-in: fBm-as-tectonics"
  :timescale-band deep-driver
  |produces "surface elevation (m)" :conservation not-tracked

|nomos[atmosphere-reservoir]                    ; — NOT BUILT — a RESERVOIR/BOX nomos
  :kernel none-yet   :physics none   :relation "declared placeholder"
  :timescale-band deep-driver
  :consumes ["planetary water inventory (km3)"]  ; itself needs the inventory declared
  |produces "atmosphere water (m w.e.)" :conservation conserved

|nomos[water-cycle]                             ; — NOT BUILT — a FIELD nomos (climate)
  :kernel none-yet   :physics low   :relation "Budyko-Sellers EBM -> precip geography"
  :timescale-band orbital-climate
  :consumes ["atmosphere water (m w.e.)" "insolation (W/m2)"]
  |produces "precipitation (m/yr)" :conservation conserved
    |predicate global precip integral equals evaporation integral (closed cycle)

|nomos[erosion-tile]                            ; REAL kernel — FIELD nomos
  :kernel Fluvial::erode   :physics med   :relation "mechanistic-causal (stream-power)"
  :timescale-band surface-process
  :depends [spine-tile]
  :consumes ["surface elevation (m)" "precipitation (m/yr)"]   ; <- HONEST need: rain to carry silt
  |produces "eroded surface (m)"     :conservation exports-at-boundary
  |produces "sediment flux (kg/m2/yr)" :conservation conserved
    |predicate mass out at tile outlets equals incision minus deposition

|nomos[surface-water]                           ; REAL kernel — FIELD nomos
  :kernel WaterSim::fill   :physics med
  :timescale-band fast-biological
  :consumes ["eroded surface (m)" "precipitation (m/yr)"]
  |produces "standing water depth (m)" :conservation conserved
```

### What the audit reads off this — with nothing running

Every one of these is a pure graph query over the declarations (no kernel touched):

- **The requisite chain (the anti-whim planner), computed by matching `consumes`→`produces`:**
  `surface-water` needs *precipitation* → kept by `water-cycle` → needs *atmosphere water* → kept by `atmosphere-reservoir` → needs *planetary water inventory* → **not declared anywhere.** So the honest answer to "can we rain principled water?" is printed as a chain that bottoms out at a missing declaration — *before* a line of physics runs. (This is exactly the `water-tile consumes precipitation with no atmosphere` specimen, now legible as a graph.)
- **The weakest-link fold, per promise, down the cone:** `erosion-tile`'s kernel is `physics: med`, but its `precipitation` input is kept by `water-cycle` (`low`) whose `atmosphere water` input is kept by `atmosphere-reservoir` (`none`, placeholder). So erosion's *derived* tier = `none` — **its "principled incision" is gloss until the reservoir lands, and the pyramid says so.** Algorithm quality never launders inputs, and the fold proves it declaratively.
- **The multirate coupling schedule falls out of `:timescale-band`:** deep-driver (spine, reservoir) seen as quasi-static by surface-process (erosion); fast-biological (surface-water) sees erosion's bed as time-averaged. The bands are declared, so the coupler knows who interpolates and who time-averages *without* the kernels negotiating it.
- **The heterogeneity is declared, not assumed:** `atmosphere-reservoir` is a box/reservoir (stocks + fluxes, no grid); the rest are fields. They couple only through the quantity strings, so a reservoir and a field compose without either knowing the other's internal shape — the domain-fixation guard, made mechanical.
- **A conservation audit is a lookup:** every `:produces` carries its claim; "is water conserved end-to-end?" reads the chain's conservation flags (`conserved` all the way through the cycle; `exports-at-boundary` flagged at erosion's outlets) — never archaeology.
- **A broken edge is a compile-time-ish error:** if `erosion` said `:consumes ["precipitation (mm/yr)"]` while `water-cycle` produced `"precipitation (m/yr)"`, the strings don't match → the edge is absent → the audit reports *erosion's rain need is unmet* even though a human would see "rain." Shared constants for the quantity vocabulary make this a build failure (the same discipline as the ASSUMPTIONS anchors).

None of this requires the simulation to run. That is the whole bet of "declare the epistemology, interactions, and lifecycle" — the honest picture of what-depends-on-what-and-how-good-is-it exists in the data, auditable, queryable, and composable, and rust's only job is to run the kernels once the graph says the composition is sound (or to run them and let the fold report exactly how gloss the result is).

---

## Layer 3 — the manifest (one world's contract; udon + `spec.rs`)

A **manifest** individuates ONE world and carries everything **prescriptive** — the choices this world makes. This is where regula's content actually lands: pin an order, aim at a phase, declare which absences you permit, set build priorities, and (regula's "second chapter") declare the participation surface. **Gloss:** a manifest names a **vivium** — one instantiated world you can name, fork, cite.

```udon
; <world>/manifest   (authored; the one udon file at a world's root)
|vivium[first-light]
  :seed 0x00                           ; identity — minted once (spec.rs today)
  :order terrestris@0.1.0              ; pins the ORDER (was: regula's job)
  :target-phase abyssal                ; how far to build this world

  |permit :defer moon        :void-on [tides moonlight]        ; a chosen absence
  |permit :defer atmosphere  :void-on [precipitation] :until reservoir-layer  ; grandfathered

  |demand                              ; build priorities (fidelity where you look)
    |beacon :region "I611-2899" :stage abyssal.3 :level finest  ; depth-first here

  |participation                       ; the exo-facing surface (regula ch.2 lands HERE)
    :modes [exploration]               ; observe-only -> moratorium-clear
    :admits ethereal
    ; stewardship duties (memo-retention, redeemer) attach here when they bind
```

```
--> Rust: crates/vivarium-world/src/spec.rs
    struct WorldSpec {                 // REAL, today — this IS the manifest
        format: u32,                   // store-format version
        name: String,                  // label — never keyed, rename freely
        seed: u64,                     // identity — every key/draw derives from it
    }
    // The prescriptive fields below are the regula-collapse PROPOSAL — NOT BUILT:
    //     order: OrderRef,            // pins ordinum@version (was regula's job)
    //     target_phase: PhaseId,
    //     permits: Vec<Permit>,       // Permit { defer, void_on, until }
    //     demand: Vec<Beacon>,        // beacons (design exists; spec field pending)
    //     participation: Participation // regula ch.2 lands here
```

- **permit** *(concrete)*: a declared, auditable absence with its **void condition** — the fluxed quantities whose consumption retires it. The *void-condition* is kind-level truth (auroras always need a field) and could equally sit on the order; the *choice to take this permit* is world-level and sits here. Shown split so we can see the seam.
- **beacon / focus** *(concrete)*: `beacon` = a standing spec-persisted priority (in `:demand`); `focus` = the live attention point (runtime, not authored). Demand changes what builds *first*, provably never *what* builds (order-independence: depend-by-key).
- **Realized / Lawful, Closed / Open** *(gloss)*: properties of the *resulting* world, not authored fields — Realized = law frozen; Lawful = law self-consistent (an asymptote); Closed = evolves from seed alone. The manifest's `:target-phase` + freeze produces a Realized world; whether it's Lawful is what the audit measures.

### `manifest-template` — the deferred reusable profile (IT-register placeholder)

This is the **one thing regula did that isn't yet real**: a *named, reusable, citable* prescriptive profile many worlds share. Given a plain IT name so we don't resurrect Latin for a layer we haven't needed.

```udon
; — NOT BUILT — author only when a SECOND world wants the same profile.
|manifest-template[rigorous-earth-sim]     ; the artifact "regula" was reaching for
  :order terrestris@0.1.0
  :rigor-floors { fluvial-erosion: high, surface-water: high }  ; demand ≥ tier
  :permits [moon]                                                ; profile's allowed absences
  :posture "B pinned high; A anchor, not target"                ; claim-ceiling (see below)

; a world then just: |vivium[...] :extends rigorous-earth-sim  (overrides as needed)
```

```
--> Rust: struct ManifestTemplate {...};  fn Manifest::resolve(&ManifestTemplate) -> Manifest
    (NOT BUILT — the seam is drawn so we know where it snaps in.)
```

- **posture** *(honest note)*: "what in-vivia results may claim." Nearly vestigial — the design already says a result's real epistemic state is its **cone-fold** and "the fold wins." Kept here only as a *ceiling*; if it never earns its keep beyond the fold, it dies quietly. That it wants to live on a *template* (a shared claim-policy) is itself weak evidence the whole template layer is the only place regula-ish content had any pull.

---

## Layer 4 — the vivium / kingdom (the instantiated world)

Not a separate file — the **vivium** is `manifest + store`, and once its law is frozen it is a **kingdom** (a governed world; English for regnum). Nothing new to author; these are what Layers 0–3 *become* when run.

```
--> Rust: struct World { store: Store, seed: u64 }   // query.rs, today
    // a World is a vivium; a Realized World is a kingdom.
```

- **vivium / vivia** *(concrete)*: the directory (store) + its manifest; forkable (copy the folder), citable (its manifest pins order@version + seed).
- **kingdom** *(gloss)*: a vivium whose law is frozen and governs whatever lives in it. Kingdoms nest (a vivium may author sub-vivia); the moratorium (ASF §0) sits above all of it as program law the audit enforces.

---

## The runtime — how a world is read and checked (rust functions)

Two verbs act on the layers above: **pull** (read/build) and **audit** (conform).

```
--> Pull (query.rs, REAL): the lazy memoized read — today one method per nomos.
    impl World<'s> {                       // World { store, seed }
        fn spine_tile(&self, …)   -> …     // each: check store by Key,
        fn erosion_tile(&self, …) -> …     //       run kernel on miss, memoize
        fn water_tile(&self, …)   -> …
    }
    // A generic `pull(aspect, region, level, time)` is the DIRECTION (uniform
    // demand-driven query); today the nomoi are hand-written methods. "Explore
    // a world" = a sequence of these; leave a tile, return -> HIT (store_explore).
```

```
--> Audit — NOT BUILT (this is the design; `vivarium status` prints only the
    fidelity pyramid today). "regula.rs" was going to be this; it needs no regula.
    fn audit(store: &Store, order: &Ordinum, m: &WorldSpec) -> Report {
        let live = promises_in_force(order, up_to = m.target_phase);
        //        = union(promises) - union(defeasances)      // accumulate-minus-defease
        for p in live {
            report.rung(p, maturity(p, store));   // named/specified/claimed/kept
            if p.predicates.is_empty() { report.warn("un-checkable: no predicate", p) }
        }
        for permit in &m.permits {
            if any_live_nomos_consumes(permit.void_on) { report.error("permit voided", permit) }
        }
        for d in defeasances(order) {
            if still_consumed(d.voids) { report.error("defeased-but-consumed", d) }  // incoherence
        }
        report   // printed by `vivarium status`, beside the fidelity pyramid
    }
```

The whole audit reads `order` (Layer 2) + `manifest` (Layer 3) against the `store` (Layer 0) + `nomotheke` (Layer 1). **There is no regula in it.** That absence is the spike's main finding: the conformance machinery never needed the middle artifact — it needed the *order* to say what the kind requires and the *manifest* to say what this world chose.

- **CLI** *(concrete, REAL)*: `vivarium new <dir>` (write a manifest) · `build` (sweep pulls) · `status` (the audit report + fidelity pyramid) · `attach` (a second builder joins under a lock).
- **fidelity pyramid** *(concrete)*: what `status` prints — per-nomos declared vs derived tier (the honesty column); the audit adds the promise-rung column beside it.

---

## The Latin ↔ IT-register ↔ what-it-actually-is ↔ verdict

| Latin | plain / IT-register | what it actually is (udon or rust) | verdict |
|---|---|---|---|
| nomos / nomotheke | law-unit / registry | `NomosDecl` + kernel fn; `NOMOTHEKE: &[&NomosDecl]` | **keep** — real grain |
| ordinum | **order** (for a kind of kingdom) | `terrestris.ordinum.udon` → `struct Ordinum` | **keep** (gloss "order") |
| regula | — | *nothing of its own* — splits into order (Layer 2) + manifest (Layer 3) | **retire** |
| — | **manifest** | `<world>/manifest` → `struct Manifest`/`spec.rs` | **keep** (plain word) |
| — | **manifest-template** | reusable profile — NOT BUILT | **defer** (placeholder name only) |
| regnum | **kingdom** | a Realized `World` | keep as gloss; regnum itself = etymology |
| Law | Law / θ | the set of nomoi the order's kept promises name | keep (word); θ near AAT |
| slot | (role string) | a promise's flux-quantity + kept-by | **probation** — likely dissolves |
| permit | permit | `Permit { defer, void_on, until }` on the manifest | keep — real |
| posture | claim-ceiling | a manifest/template string; ≈ cone-fold | **weak** — may die into the fold |
| nomothete / nomothetic | — | never used in code | footnote-only (per the ceremony call) |

---

## Step-back — checked against the existing docs; what this must not lose

*(Reviewed ORIENTATION, ARCHITECTURE, DESIGN-REDUX/-SYSTEMS, LEXICON, the regula design, builder-explorer-decoupling, spec.rs/query.rs/store.rs/nomotheke.rs before finalizing.)*

- **The four representation kinds** (field / reservoir-box / network / agent, ARCHITECTURE §0) are the real source of "kinds of nomoi" — I mapped three (system/projection/analysis) but the *representation* axis is orthogonal and load-bearing (a reservoir nomos vs a field nomos couple differently). A fuller version tags each nomos with its representation kind. Not lost, but thin here.
- **The complete content-addressed key** (DESIGN-REDUX §12) is what makes the whole store sound; I showed the Key but under-sold that *nomos-version auto-derived from source* is the unbuilt piece that makes iteration cache-transparent. Flag, not gap.
- **Builder/explorer decoupling** (the daemon vs read-only explorers, watchpoints, order-independence) is the *operational* frame around `pull` — I showed pull but not the daemon/explorer split. It's real design; belongs in a v2 of this doc.
- **The detail→abstract open problem** (upscaling an irreducible edit into a memoized macro) touches nothing above — it's the one genuinely open research seam and sits *inside* how a projection/analysis nomos would have to invalidate upward. Named so it isn't forgotten.
- **The moratorium** is correctly *not* a scaffold noun — it's program law above the manifest's `participation`, enforced by the audit. Kept as a check, not a layer.
- **What the collapse might still cost** (honest): the order stays cleanly descriptive only if *rigor-floors* live on the manifest/template, not baked into the order. This doc puts them on `manifest-template` — good — but the moment someone writes a rigor floor into the ordinum itself, the description/policy separation is gone. That's the line to hold if we collapse.

*This is a spike: shapes over polish, `— NOT BUILT` said plainly, and at least three nouns (slot, posture, manifest-template) held at arm's length rather than enshrined. If it reads as "makes sense," it did its job; if a noun here still feels like it exists only to have a home, that noun is the next thing to cut.*
