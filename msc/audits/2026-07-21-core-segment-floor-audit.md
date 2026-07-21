# Core segment floor audit — 2026-07-21

**Auditor role:** independent (not the author of these segments).  
**Repository:** `/Users/josephwecker-v2/src/archema-io/vivarium`  
**Commit under audit (HEAD target):** `197a926` — *core: open the real front door — segments as sole claim truth*  
**Binding law read:** `FORMAT.md`, `CLAUDE.md`, `core/OUTLINE.md`, all 13 `core/src/*.md`, plus spot-checks of `DECISIONS.decision-log.udon` (named entries), `LEXICON.udon` (`fated-noise`, `vivium`), `ETHICS.md` Standing Moratorium Imperative, and relevant `doc/ARCHITECTURE.md` / `doc/design/DESIGN.md` / `ASF.md` surfaces.

**Promotion posture of the floor:** all 13 segments are `stage: draft`. None is at `deps-verified` or `claims-verified`. This audit is the content gate input for that ladder — not a rubber stamp.

---

## 1. Verdict

The floor is real, not costume. All thirteen segments have complete cadence, honest `stage: draft`, and mostly restrained strength labels that track DECISIONS authority rather than inventing Joseph's assent. Source fidelity on the load-bearing Joseph decisions (segments-as-sole-truth, moratorium, probes-before-claims, water-world, grid-not-closed, algorithms-as-physics, core/view observe-only) is good; the grid question is correctly left open; the moratorium segment correctly points at full ETHICS text instead of forking the harm triple; represent-by-consequence correctly refuses to smuggle the false $R\circ L=\mathrm{id}$ claim. CLAUDE.md and README.md successfully force segment telos and reject ORIENTATION-class state maps. **The floor holds as a draft foundation, not as claims-verified law.** Two P0 defects block honest promotion: a broken depends/outline graph on `#disc-vivarium-purpose`, and `#form-core-view-wall` claiming `status: exact` while a live view still authors world-evolution parameters that the segment's own falsifier would catch — without naming that incompleteness the way `#post-determinism-as-ontology` names its agent-RNG gap. Incomplete demotion of DESIGN/ARCHITECTURE dual homes and residual ORIENTATION invitations are P1 system risks, not segment forgeries. Fix the P0s, demote the dual homes, then promote deps before the next extraction wave freezes the graph shape.

---

## 2. P0 findings (must fix)

### P0-1 — `#disc-vivarium-purpose` depends graph violates FORMAT (order + missing edges)

**Claim.** Outline order and `depends` must be independently auditable; a segment ordered before something it depends on is a finding (`FORMAT.md` §1 `depends`). Dependencies must be genuinely used for well-typedness / content review (`FORMAT.md` §2 dependency audit → `deps-verified`).

**Evidence.**

| Source | What it says |
| --- | --- |
| `core/OUTLINE.md` §I order | `#disc-vivarium-purpose` is row 2; `#def-vivium` and `#def-in-vivia` are rows 3–4 |
| `core/src/disc-vivarium-purpose.md` frontmatter | `depends: [scope-segment-canon, def-vivium, def-in-vivia]` |
| same file, Formal Expression §3 | cites `#post-determinism-as-ontology`, `#form-core-view-wall`, `#post-represent-by-consequence` as the “shared walls” the purpose claim uses |

**Defect.** (a) The purpose segment is outlined **before** two of its declared depends. (b) Formal Expression load-bearing citations of the three walls are **not** in `depends` — either add them (and reorder) or stop treating them as constitutive of the claim.

**Suggested fix.** Prefer reorder outline so definitions (and walls, if kept as depends) precede purpose; expand `depends` to include every slug Formal Expression uses as a non-optional prior. Re-run a depends/outline consistency pass on §I before any `deps-verified` stamp.

---

### P0-2 — `#form-core-view-wall` claims `exact` while FE(4) has a live falsifying surface not named in Epistemic Status

**Claim.** Segment Epistemic Status: max attainable `exact` as architecture; **falsified by** “a view-owned evolution parameter that changes what the world *is*.” Formal Expression §4 (observe-only evolution) is the load-bearing clause backed by `DECISIONS[core-view-wall-observe-only]` (`:by us`, decided) — the globe `VIVARIUM_EROSION_EPOCHS` knob was the specimen violation and was reverted.

**Evidence.**

| Source | What it says |
| --- | --- |
| `core/src/form-core-view-wall.md` FE §4 | “A view does not expose knobs that choose how the world evolves (for example, how many erosion epochs to run).” |
| same file, Epistemic Status | Names founding commitment + re-assertion after globe violation; does **not** name a remaining FE(4) incomplete surface |
| same file, Working Notes | Names worldview only as “physics testbench (not fully store-backed navigation)” — navigation/store gap, not evolution-parameter gap |
| `spikes/worldview/src/main.rs` ~360–415 | View reads `VIVARIUM_MACRO_EXTRA`, `VIVARIUM_FINE_EPOCHS`, `VIVARIUM_FINE_PASSES` and **runs erosion epochs** as part of the explorer path (world-evolution parameters owned by a view) |

**Defect.** The segment states a crisp falsifier for exactness, then claims `exact` while a live peer-shaped view still authors evolution parameters of the same species the DECISIONS entry convicted. Contrast `#post-determinism-as-ontology`, which keeps `axiomatic` **and** names the incomplete agent-RNG surface in Epistemic Status. Under-naming here is status laundering: exact travels; the incompleteness does not.

**Suggested fix (strengthen, do not soften the wall).** Keep FE(4) and `status: exact` as architecture law **only if** Epistemic Status adds a **Known incomplete surface** parallel to determinism’s: worldview (and any similar spike) still runs evolution epochs / fill parameters as a view-local builder hybrid; that is a compliance debt, not a license to dissolve the wall; ethereal explorer milestone remains gated on FE(4) holding for the explorer path that claims moratorium-clear observe-only. Optionally demote Working Notes’ “navigation gap” phrasing so it does not hide the stronger FE(4) debt. Do **not** weaken the law to match the spike.

---

## 3. P1 findings (should fix before promote / before trusting sole-home)

### P1-1 — Incomplete replacement demotion for claims already segmented

**Claim.** `#scope-segment-canon` FE §4: when a segment lands, it **replaces** prior prose homes — delete, one-line pointer, or archive — not a softened twin. CLAUDE.md Working rules §1 same.

**Evidence.**

- `doc/design/DESIGN.md` still presents itself as carrying “purpose, disposition, and founding commitments,” opens recreational-first (“the fun is allowed to lead”), and still states core/view wall + determinism-as-ontology as living founding text without pointer to `#disc-vivarium-purpose`, `#form-core-view-wall`, `#post-determinism-as-ontology`.
- `doc/ARCHITECTURE.md` Layer 0 still states **represent by consequence**, core/view wall, and determinism-as-ontology as spine law without demotion to the new slugs (Working Notes of `#post-represent-by-consequence` even warn against smuggling false $R\circ L$ sentences that still live there).
- Segment Epistemic Status on `#scope-segment-canon` correctly says migration incompleteness is expected and does not license treating unmigrated prose as canon — but **landed** claims still have full rival bodies, which is the dual-home mechanism the floor exists to kill.

**Suggested fix.** For each §I claim that now has a slug: reduce DESIGN/ARCHITECTURE claim bodies to a one-line present-tense pointer (`Claim home: #slug`). Leave non-segmented material as source. Do not rewrite history into segment Discussion.

---

### P1-2 — Residual ORIENTATION-class / stale onboarding invitations outside CLAUDE/README

**Claim.** Audit question F: does the front door force segment telos, or still invite state-map / ORIENTATION drift?

**Evidence.** CLAUDE.md and README.md are clean routers (segments sole claim truth; no ORIENTATION revival). Residual invitations remain:

- Workspace `Cargo.toml` comments still point at `ORIENTATION.md` as if live (file lives only under `.archive/ORIENTATION.md`).
- `doc/PROCESS.udon` `|norm[doc-placement]` and `|norm[present-tense-bodies]` still list ORIENTATION as a root front door / current-state document.
- `doc/design/NOMOS-CONTRACT.md` onboarding block still tells agents to read `ORIENTATION.md`'s HANDOFF.
- `doc/theory/multiscale-methods.md` still refers to “ORIENTATION” as the home of an open problem.

**Suggested fix.** Sweep live-tree tactical pointers: ORIENTATION → archived with explicit non-tactical gloss, or retarget to `core/OUTLINE.md` + relevant slugs. PROCESS taxonomy should list `core/` not ORIENTATION as claim front door.

---

### P1-3 — `#norm-declaration-must-convict` strength vs ratification surface

**Claim.** Frontmatter `status: exact`. `#norm-decision-authority` FE §3: segments may state as settled only what is ratified under authority tags.

**Evidence.**

- Exact phrase “A declaration that cannot fail a build is a wish” originates in agent-authored `doc/theory/discretisation-and-information.md` §2.4a and is echoed in a DECISIONS narrative block on mean-pin (not a standalone `:by joseph` decision slug).
- `FORMAT.md` Open questions treats it as “vivarium's governing concern” (good adoption signal, still not a Joseph decision entry).
- Closest Joseph-ratified sibling: acceptance-test rule (promise with no predicate cannot be marked fulfilled — 2026-07-11; LEXICON `|term[promise]`; `ordinum.rs` enforcement; `DECISIONS[ordinum-governs-the-flux-web]`).

**Defect.** Exact as process law is **plausible** (project practice + mechanized instances) but **under-cited** relative to the authority norm this same floor teaches. Risk is circular: the floor asserts exact norms that the authority norm would flag as agent packaging.

**Suggested fix (strengthen first).** In Epistemic Status, ground the norm in Joseph's acceptance-test rule + ordinum/nomotheke/ASSUMPTIONS convict surfaces; mark the generalisation from promises to all nomos self-description as the standing method. If Joseph has not endorsed the generalisation, keep the body, set status to `robust-qualitative` or `conditional` until a DECISIONS entry exists — **only after** the strengthen attempt fails, not as the first move.

---

### P1-4 — `#form-core-view-wall` `depends` on moratorium is motivational, not well-typing

**Claim.** FORMAT: depends = priors that make the claim well-typed / genuinely used.

**Evidence.** Formal Expression of the wall never requires the moratorium. Moratorium appears only as a consequence gloss in Epistemic Status (“ethereal explorer is moratorium-clear partly *because*…”). The wall is independently founding in DESIGN.md and DECISIONS.

**Suggested fix.** Drop `scope-moratorium-endogenous-emergence` from depends (keep prose cross-ref), **or** move the moratorium-clear consequence into Formal Expression as an explicit derived clause with the depend justified. Prefer drop: wall is prior to, not dependent on, ethics scope.

---

### P1-5 — Diff-voice residue in Discussion / Epistemic Status

**Claim.** FORMAT §6: segment voice states what *is*, not what changed; history lives in git / DECISIONS / archive.

**Evidence (representative).**

- `#scope-segment-canon` Discussion: “Empty `core/` after the 2026-07-13 archiving left a vacuum…”
- `#disc-vivarium-purpose` Discussion: “Early design text led with recreation… Later work made…”
- `#form-core-view-wall` Epistemic Status: “re-asserted after a live violation… reverted”
- `#norm-decision-authority` Epistemic Status: “Codified after a live incident…”

**Suggested fix.** Keep one-line falsifier / regression-guard facts where they protect against reintroduction (Working Notes is the right home for “globe epochs knob was the specimen”). Rewrite Discussion/Epistemic Status into present-tense law + limits without chronicle scaffolding.

---

### P1-6 — `#disc-check-the-ladder` authority mix

**Claim.** FE §1 states ladder-priority as standing method; FE §2 water-world form is Joseph-decided.

**Evidence.**

- `DECISIONS[water-world-is-the-promise-not-the-bug]` — `:by joseph :status decided` (exact for that instance; segment says so).
- `DECISIONS[check-the-ladder-not-modern-earth]` — `:by claude :status decided` (general standing guard is agent-codified).

**Defect.** Status `robust-qualitative` is overall honest. Formal Expression §1 still reads as project method without marking that the general form’s DECISIONS entry is `:by claude`. Under `#norm-decision-authority`, that distinction should be visible at the claim site.

**Suggested fix.** In Epistemic Status (or FE §1 parenthetical): method is robust-qualitative / claude-codified generalisation of Joseph’s water-world correction; Joseph-exact applies to FE §2 instance. Do not upgrade the general method to joseph/us without his stamp.

---

## 4. P2 / nits

1. **`#def-vivium` ↔ LEXICON dual home.** Segment Formal Expression nearly restates `#lexicon/term/vivium` with mild expansion (“build-stack and law… target world state” vs LEXICON “build-stack + Target world”). Epistemic Status claims intentional split (segment = claim-home, LEXICON = dictionary). FORMAT §5 still says an inlined definition is a fork. **Nit:** keep wording bit-identical to LEXICON + one-line operational gloss, or add a one-line “dictionary form owns headword; this segment owns project claim packaging” discipline note in FORMAT open questions. Drift already visible; will grow.

2. **`#def-in-vivia` expands beyond LEXICON** (operational criteria 1–3). Correct as formalization of the register embedded in the vivium entry; ensure LEXICON later either points to the segment or stays a one-line gloss only.

3. **`#post-determinism-as-ontology` Discussion** cites `#def-in-vivia` for frame-relativity without depending on it. Acceptable as non-constitutive gloss; optional soft depend if you want the graph complete.

4. **Workspace comment drift** still names `vivarium-core` as the sim with no bevy dependency; segment correctly generalizes to `vivarium-world` + successors. Align comments when touching Cargo.toml for P1-2.

5. **Bare-absolute scan:** no FORMAT-grade superlatives (“the deepest,” “the one thing that matters”) found in the 13 bodies. Good.

6. **Math / `#` in math:** no violations found in the 13.

7. **Slug/filename consistency:** all 13 match; outline tags resolve.

8. **OUTLINE §VII phase names** (Ante-Mundane … Scribal) match `tabularium/terrestris.ordinum.udon` headwords — the Jul-14 candidate “Modern vs Historical” conflict appears **already fixed** in the live outline. Do not re-open from stale candidate text.

9. **`#norm-probes-before-claims` depends on `#norm-declaration-must-convict`.** Content relation is dual (probes convict behavior; declarations convict self-description). Depend is pedagogical, not strictly well-typing. Acceptable; optional reverse-edge discussion only.

10. **Cadence completeness:** all 13 have frontmatter, title, one-sentence summary, Formal Expression, Epistemic Status, Discussion; Working Notes present where useful; no Findings sections (optional — fine).

---

## 5. Per-segment scorecard

| slug | type/status ok? | source ok? | main issue |
| --- | --- | --- | --- |
| `scope-segment-canon` | yes — scope / exact / draft | yes — `DECISIONS[segments-are-sole-claim-truth]` `:by joseph` decided | Replacement rule stated; **not yet executed** on DESIGN/ARCHITECTURE (P1-1). Mild diff-voice in Discussion. |
| `disc-vivarium-purpose` | type/status ok (discussion / discussion-grade) | dual-purpose reconciling DESIGN + Archema is fair; phase-order clause tracks Joseph 2026-07-20 | **P0-1 depends/outline.** Missing wall depends. Diff-voice in Discussion. |
| `def-vivium` | yes — definition / exact | yes — LEXICON settled; Joseph coinage 2026-07-04 | Near-fork restatement; minor wording drift vs LEXICON (P2). |
| `def-in-vivia` | yes — definition / exact; conditional quality of studies correctly separated | yes — LEXICON register + ASF.md *in vivia* citation intent; honest about non-mechanized citation recipe | Good restraint on bit-perfect citation infrastructure. |
| `scope-moratorium-endogenous-emergence` | yes — scope / exact | **strong** — ETHICS §0 match; program law; open NPC boundary preserved; full text not forked | Best packaging pattern on the floor. |
| `post-determinism-as-ontology` | yes — postulate / axiomatic | yes — LEXICON `fated-noise` settled; DESIGN founding; incomplete agent RNG named | Model for honest incompleteness (contrast P0-2). |
| `form-core-view-wall` | type ok (formulation); **status exact overclaims relative to named falsifier** | DECISIONS observe-only `:by us` decided is solid for the law | **P0-2** live worldview evolution knobs; weak moratorium depend (P1-4). |
| `post-represent-by-consequence` | yes — postulate / axiomatic | yes — ARCHITECTURE Layer 0 consolidating principle; correctly refuses $R\circ L$ smuggle | Dual home still in ARCHITECTURE (P1-1). |
| `norm-declaration-must-convict` | type ok; **status exact under-grounded** | sibling Joseph acceptance-test is real; exact phrase is agent/FORMAT packaging | **P1-3** provenance. |
| `norm-probes-before-claims` | yes — normative / exact | yes — `DECISIONS[probes-before-claims-…]` `:by joseph` decided | Clean. |
| `norm-decision-authority` | yes — normative / exact | yes — DECISIONS legend + `grid-question-not-closed-…` `:by joseph` | Clean; mild incident-chronicle in Epistemic Status (P1-5). |
| `disc-algorithms-disguise-physics` | yes — discussion / robust-qualitative | yes — `DECISIONS[algorithms-are-disguised-physical-claims]` `:by us` decided; **grid left open** | Correct anti-inflation vs grid. |
| `disc-check-the-ladder` | status robust-qualitative ok for method | water-world joseph exact; general method claude-codified | **P1-6** authority visibility. |

---

## 6. What is actually good

Honesty requires the strengths:

1. **The door is real.** CLAUDE.md telos, session-success definition, and “no new ORIENTATION” rule match `DECISIONS[segments-are-sole-claim-truth]`. README is a pointer, not a second law. That is the structural fix the Jul-13 archive left unfinished.

2. **Cadence discipline.** All 13 files follow FORMAT §4 structure. Epistemic Status is a section, not a side note. Stage is uniformly `draft` — no fake `claims-verified`.

3. **Authority discipline on the hard cases.**  
   - Grid verdict not closed (`disc-algorithms` Working Notes + DECISIONS).  
   - Moratorium full argument stays in ETHICS; segment is scope + pointer (FORMAT anti-fork).  
   - Represent-by-consequence explicitly refuses false multiscale operator claims still living in ARCHITECTURE.  
   - *In vivia* quality claim marked conditional on fidelity program.  
   - Determinism marks agent-RNG incompleteness.

4. **Conservative strength labels on methods.** Algorithms-as-physics and check-the-ladder sit at `robust-qualitative`, not `exact`. Purpose sits at `discussion-grade`. That is the right ceiling for chosen method/purpose claims.

5. **Carved supporting authorities are coherent.** LEXICON / DECISIONS / ETHICS carve-outs in `#scope-segment-canon` match how CLAUDE’s authority table is written; executable state convicts, does not replace claim text.

6. **Working Notes are mostly forward-facing** (manifest schema later; mechanism field open; grid open; ETHICS full segmentation later) rather than victory-lap history — with the P1-5 exceptions noted.

7. **Outline names absence.** §I–VII `--GAP--` / missing rows make the incomplete migration visible. That is better than a false sense of a complete core.

8. **No invented Joseph decisions.** Spot-check of the named DECISIONS slugs used by the floor found no segment marking `:by claude` or proposed verdicts as joseph/us settled law. The authority failure mode that produced “grid question is CLOSED” is not repeated here.

---

## 7. Recommended next extraction order (≤5) if the floor holds

Do **not** extract until P0-1 and P0-2 are closed and a depends/outline pass is clean. Then, source-first (DECISIONS / LEXICON / ordinum / theory), no crate work unless a segment’s Working Notes demand a convicting probe:

1. **`#def-nomos` + nomotheke contract** (or split definition / normative) — executable self-description is already half-built; needed by declaration-convict and every physics claim. Sources: LEXICON, `nomotheke.rs`, regula-conformance plan (as source only), ordinum `:kept-by`.

2. **`#form-complete-content-addressed-key` (or postulate)** — one of ARCHITECTURE’s three walls; pairs with determinism and store-as-save. Sources: ARCHITECTURE §5, DESIGN-REDUX, DECISIONS on store/memoization.

3. **`#def-phase` / ordinum ladder semantics** — resolve LEXICON “span” vs enforced “state” before any phase physics segment; required by `#disc-check-the-ladder`’s next depth. Sources: `tabularium/terrestris.ordinum.udon`, `ordinum.rs`, DECISIONS ordinum-governs-the-flux-web.

4. **`#disc-prime-question` / modified-equation analysis** — the method that makes “algorithms disguise physics” operational and keeps plausibility from wearing verification’s clothes. Sources: `doc/theory/discretisation-and-information.md` (as source; extract present truth only).

5. **`#form-flux-web` + hydrosphere box-nomos instance** — first concrete conservation/declaration stack that can convict; already measured and Joseph-decided in pieces (`water-conserved-from-ante-mundane-mass`, ordinum wiring). Prefer one flux-web segment plus a thin hydrosphere derived/empirical sibling rather than a systems dump.

**Explicitly not next:** grid verdict packaging (Joseph has not closed it); full ETHICS segmentation (carve-out is honest); builder/explorer decoupling UI work (instrument, not claim, this phase).

---

## Auditor notes (process)

- No segments were edited. No commit made.
- Primaries were read for fidelity; this report is not a re-authoring of the floor.
- Strengthen-before-soften applied: P0-2 fix preserves the wall and demands honesty about incompleteness rather than weakening observe-only to match worldview.
- If a later pass promotes any segment past `draft` without closing P0-1, the `deps-verified` rung is false by FORMAT’s own definition.

---

*End of audit.*
