# Decisions + code mining report — 2026-07-21

**Slice:** (1) DECISIONS `:status decided` without segment home; (2) code-enforced claims vs segments; (3) ASSUMPTIONS discipline.  
**Not claim canon.** Draft segments live beside this report; orchestrator promotes.

**Authority discipline (telos):** `:by joseph` / `:by us` = settled when decided. `:by claude :status decided` = tactical/practice unless Joseph ratified — do not launder into exact law beyond what code already convicts. Proposed / lead / deferred / superseded are **out of scope for promotion** except as residual notes.

---

## Ranked gaps (highest leverage first)

| Rank | Gap | Kind | Disposition |
|------|-----|------|-------------|
| **1** | Store *is* the save (memo store = portable world) | OUTLINE §III · code `store.rs` | **Draft segment** `form-store-is-save` |
| **2** | Nomotheke as registry contract (declaration → key, tests) | OUTLINE §II · `nomotheke.rs` | **Draft segment** `form-nomotheke-registry` |
| **3** | Builder admission + explorer observe-only pull | OUTLINE §III · `vivarium.rs` / `query.rs` | **Draft segment** `form-builder-admission` |
| **4** | Bias-vs-noise as first audit cut | decided `:by us` · only glossed in `#disc-prime-question` | Segment later; do not bury under Prime Question alone |
| **5** | Column = control volume + sufficient statistics | decided `:by us` (frame only) · Jensen correction in proposed | Segment later — FE must state frame-only, cite n=1 correction |
| **6** | Geometric contract (declare cell assumptions) | decided `:by us` · unbuilt surface | Segment when audit path exists; leave DECISIONS for now |
| **7** | Tentative grid: cube-sphere + stagger | decided `:by us` tentative · unpaid leaf-only price | **Leave DECISIONS only** until price weighed |
| **8** | ASSUMPTIONS one-way ledger (anchors ⊆ ledger; reverse open) | code + banner debt | Note — no soft-close |
| **9** | Residual under-keying after `with_dep_versions` | compliance vs `#form-complete-content-addressed-key` | Note — open |
| **10** | Regula collapse / manifest / run-modes | decided process + open tooling | Leave DECISIONS / LEXICON; not store law |

---

## 1. DECISIONS `:status decided` → home map

### Covered by existing segment (pointer only; DECISIONS remains history)

| Decision slug | `:by` | Segment home |
|---|---|---|
| `segments-are-sole-claim-truth` | joseph | `#scope-segment-canon` |
| `core-view-wall-observe-only` | us | `#form-core-view-wall` |
| `probes-before-claims-no-plausibility-as-verification` | joseph | `#norm-probes-before-claims` |
| `ordinum-governs-the-flux-web` | joseph | `#form-ordinum-governs-flux-web` |
| `check-the-ladder-not-modern-earth` | claude* | `#disc-check-the-ladder` |
| `algorithms-are-disguised-physical-claims` | us | `#disc-algorithms-disguise-physics` |
| `water-conserved-from-ante-mundane-mass` | joseph | `#obs-hydrosphere-box-nomos` |
| `water-world-is-the-promise-not-the-bug` | joseph | `#disc-check-the-ladder` (+ ordinum data; not sole home) |
| `nomos-plural-invariant` | joseph | `#def-nomos` (+ LEXICON) |
| `flux-web-substrate-on-common-ground` | claude* | `#form-flux-web` (practice: flux/audit) |
| `grid-question-not-closed-authority-was-inflated` | joseph | `#norm-decision-authority` (specimen + rule) |
| `asf-bridge-returns-to-the-live-tree` | joseph | `#scope-asf-reading-gates` (+ live `ASF.md` carve-out) |
| `climate-nomos-closes-the-flux-web` | claude* | **partial** `#form-flux-web` + `#obs-hydrosphere-box-nomos` — climate geography / box→field detail still unsegmented |

\*claude decided but practice or method already segment-backed; do not upgrade authority tag via segment prose.

### Drafted this wave (new segment files below)

| Decision / code locus | Proposed segment |
|---|---|
| `store.rs` + DESIGN-REDUX §13 save-file claim | `#form-store-is-save` |
| `nomotheke.rs` enforcement surface (beyond `#def-nomos` FE) | `#form-nomotheke-registry` |
| builder refuse/`--allow-unmet` + `surface_prefer_eroded` | `#form-builder-admission` |
| (touches) `uplift-is-its-own-nomos` | mentioned under registry; full uplift/isostasy later |

### Leave in DECISIONS only (or non-segment carve-out)

| Decision slug | `:by` | Why not a segment (now) |
|---|---|---|
| `regula-collapses-to-order-and-manifest` | us | Artifact retired; content is process/architecture trail; ordinum+manifest already in `#form-ordinum-governs-flux-web` |
| `regula-vocabulary-deferred` | joseph | **deferred** naming — LEXICON open, not a claim |
| `debug-poke-then-consolidate` | us | Method note; optional later ops segment |
| `new-system-must-reach-the-goal-not-reimplement-poc` | joseph | Process intent; adjacent `#disc-vivarium-purpose` |
| `terminology-register-by-layer` | us | LEXICON / naming discipline, not a core claim slug |
| `tile-as-honest-flat-artifact` | us | LEXICON term bounds |
| `cli-world-dir-default-and-symlink-promotion` | claude | Tooling convenience; not world-law |
| `initial-topography-rename-executed` | us | Done rename trail; code is truth |
| `scribal-and-a-1-indexed-ladder` | joseph | Ordinum **data** + OUTLINE §VII; phase content segments still missing |
| `superseded-ledger-lives-in-archive` | joseph | Process carve-out |
| `front-door-docs-swept-to-present-truth` | claude | Historical process |
| `the-bet-does-not-rest-on-one-question` | joseph | Process / FORMAT no-absolutes specimen |
| `seam-probes-were-measuring-seabed` | claude | Measurement specimen for probe discipline (already `#norm-probes-before-claims`) |
| `corner-pathology-confirmed-but-mfd-does-close` | claude | Measurement; physics corpus later |
| `configuration-classes-are-two-not-ten` | claude | Correction to geometric-contract sizing; folds into geometric contract when segmented |
| `mfd-fan-is-a-bias-and-does-not-converge` | claude | Measurement under bias-vs-noise; not a grid verdict |
| `the-grid-tentatively-decided-keep-the-cube-sphere-and-stagger-it` | us | **Tentative** with unpaid leaf-only-evolution price; reopen conditions named — **do not mint exact segment yet** |
| `uplift-is-its-own-nomos` | joseph | **Practice true** (code); full physics claim blocked by proposed `uplift-is-structurally-incapable…` — wait for isostasy chain |

### High-value decided, segment missing (not drafted — larger / frame-only)

| Decision slug | `:by` | Proposed home when written |
|---|---|---|
| `bias-vs-noise-is-the-decisive-audit` | us | `#form-bias-vs-noise` (or `#norm-bias-vs-noise`) — **rank 4** |
| `column-is-a-control-volume-with-sufficient-statistics` | us | `#form-column-control-volume` — status-note: frame settled, implementation undesigned; must absorb n=1 / joint-(A,S) correction from proposed `the-jensen-variable-was-wrong…` without soft-closing the frame |
| `geometric-contract-metric-set` | us | `#form-geometric-contract` — after declare/match surface exists |

### Explicitly not decided (do not promote)

All `:status proposed|lead|wish|superseded|deferred` — including grid-report verdicts still tagged proposed, structure-preserving recon, wavelet store, Jarrett/θ findings, plate-tectonics study, etc. Measurements may be solid; **verdicts stay proposed** until Joseph tags them.

---

## 2. Code-enforced claims vs segments

### Live enforcement surfaces

| Locus | What convicts | Segment coverage |
|---|---|---|
| `store.rs` | Content-addressed objects/roots; complete `Key` builder; `with_dep_versions` | **Partial** `#form-complete-content-addressed-key`; **save-file claim missing → draft `#form-store-is-save`** |
| `nomotheke.rs` | Unique names; deps registered; assumption anchors ⊆ ASSUMPTIONS; flux vocabulary closed; consumed⇒in-deps; key embeds direct dep versions; conservation on every promise; weakest-link `derived_physics` | **Partial** `#def-nomos` FE(4); **registry contract missing → draft `#form-nomotheke-registry`** |
| `audit.rs` | Unmet flux as pure graph query; requisite chains; emerged-land as standing unmet specimen | **Covered** `#form-flux-web` + `#form-ordinum-governs-flux-web` |
| `query.rs` | World owns (store, seed); pull-query memo; `with_dep_versions` on IT/climate/erosion/water; `surface_prefer_eroded` never cold-computes erosion | Key completeness → existing; **observe-only pull → draft `#form-builder-admission`** |
| `bin/vivarium.rs` | Builder admission: refuse phases with unmet requisites (exit 2) unless `--allow-unmet` logs waiver | **Draft `#form-builder-admission`**; residuals open (below) |
| `ordinum.rs` | Promise maturity / BrokenKeeper | `#form-ordinum-governs-flux-web` (display debt known) |
| ASSUMPTIONS + `include_str!` | Build fails if nomos anchor missing from ledger | `#norm-declaration-must-convict` + ASSUMPTIONS banner; **not a segment file** (carve-out) |

### Prefer not duplicating

- `#form-complete-content-addressed-key` — complete-key law (already has known incomplete: hand versions, FNV, completeness enforcement).
- `#form-flux-web` — consume/promise edges + admission *principle*.
- `#form-ordinum-governs-flux-web` — ladder ↔ flux join.

Drafts cite these rather than restate them.

---

## 3. Residual under-keying / declaration gaps (open — do not soft-close)

Direct-dep under-keying (auditor de-novo A) is **fixed** for IT / climate / erosion / water via `Key::with_dep_versions` + `key_with_dep_versions_embeds_each_dep_identity`. The following remain **open compliance / design debt** against `#form-complete-content-addressed-key`:

1. **Transitive dep versions.** Keys fold *direct* `deps` only. Example: NOISE version participates in INITIAL_TOPOGRAPHY keys; EROSION keys fold INITIAL_TOPOGRAPHY **name@version**, not NOISE. If a hand-stamped intermediate version is forgotten while a leaf dep bumps, a consumer can **Hit** stale bytes. Law says upstream identities (hashes); practice is direct version strings. Strengthen: content-hash of pulled inputs and/or transitive version closure — **not claimed done**.

2. **UPLIFT uses `noise::fbm` with `deps: []`.** Kernel depends on fated-noise implementation; declaration does not list NOISE; key has no NOISE version field. Species of under-declaration that `consumed_and_met_implies_in_deps` does not catch (no flux edge).

3. **HYDROSPHERE / CLIMATE / WATER read `Planet::EARTH` constants** without PLANET (or planet version) in key. Earth-ref parameters live in assumptions ledger but not in complete keys. Changing planet mass without hydrosphere version bump → silent stale budgets.

4. **Hand-stamped nomos versions** remain load-bearing (known on complete-key segment).

5. **Builder provisional path:** `--allow-unmet` logs waiver; store census does **not** mark provisional artifacts; waived keys == lawful keys (de-novo A residuals A/B). `#form-flux-web` known-incomplete (3) discloses; **not closed**.

6. **No lib-level integration test** that `vivarium build` refuses unmet (gate lives in bin).

Do not write “under-keying fixed.” Write “direct-dep class fixed; transitive / undeclared-noise / planet / provisional still open.”

---

## 4. ASSUMPTIONS discipline

| Surface | State |
|---|---|
| Banner (file head) | Honest: load-bearing via `include_str!`; **in debt** to claim discipline; destination udon/generated; not a front door |
| One-way guard | `every_assumption_is_in_the_ledger` — nomos anchors must appear as substrings in ASSUMPTIONS.md |
| Reverse direction | **Open:** ledger rows with no nomos `assumptions:` entry (e.g. MFD `P`, drainage cell area, Jarrett, θ, Fr cap, f32 η, well-balancedness structures) are accounted in prose but **not declared on a nomos** — so the build does not fail if a kernel uses them without an anchor |
| WATER declaration gap | Frame kernel hardcodes Jarrett / θ / Fr / surface-as-depth; WATER.assumptions lists bounded-fill, atmosphere store, fill steps, SEA_LEVEL_M only — **ledger and declaration out of step** (ASSUMPTIONS body already corrected the mis-file; nomotheke anchors not updated) |
| Typography / absolutes | Banner still flags emphasis-as-evidence debt; OUTLINE “no absolutes” |
| Segment home | **Leave as root carve-out** until generated ledger; optional later `#form-assumptions-ledger` when udon lands — not this wave |

**Do not soft-close:** “every constant is declared” is false until reverse coverage and WATER anchors catch up.

---

## 5. Drafts produced this wave

| File | Slug | Why high value |
|---|---|---|
| `draft-form-store-is-save.md` | `form-store-is-save` | OUTLINE §III gap; store.rs module claim; sibling of complete-key |
| `draft-form-nomotheke-registry.md` | `form-nomotheke-registry` | OUTLINE §II gap; code-enforced registry surface beyond def-nomos |
| `draft-form-builder-admission.md` | `form-builder-admission` | OUTLINE §III gap; de-novo P0 fix; core/view wall at the builder boundary |

All `stage: draft`. No OUTLINE promotion / no commit (orchestrator).

---

## 6. What this miner deliberately did not do

- Mine proposed physics (router, wavelet, Jarrett) into segments.
- Soften complete-key or flux law to match residual practice.
- Segment the tentative grid decision.
- Rewrite ASSUMPTIONS.md or DECISIONS.
- Touch dual-home design prose (other wave agents).
