# Toolchain / ARCHITECTURE / consolidation-report slice — reconciliation against `core/`

*2026-07-24. Slice: `.super-archive/from-doc/toolchain.md` · `.super-archive/from-architecture/ARCHITECTURE.md` · `.super-archive/from-msc/consolidation-wave-2026-07-21/` (all nine files) · `.super-archive/from-msc/session-2026-07-10-mechanics.md` · `.super-archive/MANIFEST.md` + `README.md`. Not claim canon; a reconciliation instrument. Where this disagrees with `core/`, core wins on claim truth — but several rows below are claims that **core disagrees with itself** about, and those are findings, not deference cases.*

---

## Bottom line

**For this slice, FE(4) of `#scope-segment-canon` holds.** I found no substantive workflow / CLI / harness / builder / store-operations idea that was silently dropped. The peel was good, and in four places it *strengthened* what it peeled (§3). Three of your eight guessed segment homes were right; the material also landed in five places your list did not name, which is worth more to you than the confirmation (§2).

**But the answer to your question is not the answer to Joseph's question**, and I think that is the finding worth your time. What Joseph named has four properties:

| Joseph's property | Where it stands |
|---|---|
| world building working in the background toward known points of interest | **designed, unbuilt, well-owned** — ` #detail-builder-daemon ` FE(2)–(3) |
| restarting at the right points as we iterate on the algorithms | **core has taken a position against it**, with a named unblock condition — and one paragraph of core still promises the opposite (§4.1) |
| exploration able to watch it in real-time at the edge / replays | **replay referents pinned, carve is a named gap**; builder-side instruments designed; view-side unowned (§4.5) |
| with a principled understanding of what exactly they are watching | **no owner in core or in ice** — the closest thing is one word ("epistemic-tagged") in a definition-of-done (§4.6) |

The peel is not what stands between you and that. Rows 2–4 were thin in the historical corpus too. So the reconciliation you were promised is honestly *yes, it's in core* — and the design work is genuinely new work, not recovery.

---

## 1. Method, and what I did not do

Read completely: `CLAUDE.md`, `core/OUTLINE.md`, `FORMAT.md`, all nine assigned files, and 21 core segments (every one named in §2 below, plus ` #form-fidelity-invariant `, ` #form-temporal-lod-regimes `, ` #form-kernel-imperative-boundary `, ` #norm-probe-sensitivity `, ` #norm-probes-before-claims `, ` #detail-seam-precedents `, ` #detail-phenomena-systems-map `, ` #detail-abyssal-parity-build `, ` #form-in-vivia-citation `, ` #form-core-view-wall `). Verified live against `bin/check`, `clippy.toml`, `crates/vivarium-world/build.rs`, `crates/vivarium-world/src/nomotheke.rs`, `LEXICON.udon`, `CONSOLIDATION-STATUS.md`, and `git log --diff-filter=D`.

Because a string test would not settle this, each idea was tested by asking *what would core have to say for this thought to be present* and then reading the candidate segments — which is how ARCHITECTURE §3's "three twists we own" turned up in ` #detail-seam-precedents `, a segment nobody's file list named.

**One structural check worth recording:** `git log --diff-filter=D` over `doc/`, `msc/` and root markdown shows **no toolchain- or architecture-class file was ever deleted** — every graduation was a `git mv`, and the only deletions are `form-save-is-memo-store.md` (an intentional dual-home stub), `DESIGN.md` (a pointer shell removed after full-body provenance was restored), `LEXICON.md` (renamed to `.udon`), one `.svg`, and two stray tmp files. So `.super-archive/` really is the complete graduated set for this slice; there is no third pile.

Not done (other agents' slices, or out of budget): `.super-archive/from-plan/*`, `.super-archive/from-archive/*`, `msc/agent-briefs/*` beyond confirming which core segments cite them, `doc/PROCESS.udon`, `.archive/SUPERSEDED.md`.

---

## 2. In core, faithfully — including five homes not on your guess list

Your guesses that were right: ` #detail-epistemics-toolchain ` (toolchain.md, whole file), ` #form-store-as-save `, ` #form-complete-content-addressed-key `, ` #form-builder-admission `, ` #detail-builder-daemon `, ` #form-three-scoped-runtime `, ` #form-nomotheke-registry `. ` #ops-audit-integration ` turned out not to carry any of this slice.

The rows below are the ones where finding the home took work. Each is spot-checkable in one read.

| Source idea | Core home | Note |
|---|---|---|
| ARCH §3 **"three twists we own"** — lazy pulls are *backwards-from-now* (literature pulls forward-in-time); refinement is *attention-driven* (AMR's is error-driven); restriction is *per-consumer* | ` #detail-seam-precedents ` **FE(7)** | All three, in one clause, named as "deliberate deviations from the precedents, named so they are defended rather than drifted into" — and **strengthened**: FE(7) adds the honest cost the source never stated ("an under-resolved process outside the attention cone is exactly what the criterion misses"). Consumer-dependent $R$ additionally has law homes at ` #form-rl-closure-algebra ` FE(4) and ` #form-column-control-volume ` FE(3) (the *guaranteed vs approximate* flag survives as the exactness claim). |
| ARCH §5 **run-modes carve** — strictly-causal / replay / discardable-iteration / live-play maps to Closed vs Open-with-recorded-forcing plus the pre-participation non-intervention register | `LEXICON.udon` `\|term[run-modes] :status open` (~line 201) | Survives nearly verbatim, in the dictionary rather than in core. `OUTLINE` §III's gap row is precise about this — *"Run-modes carve (thin: LEXICON open referents + root isolation; no fat enum)"* — so the gap row and the surviving content point at each other correctly. This one reads as absent from core and is not. |
| ARCH §6 **four multirate bands** (deep drivers Myr → orbital/climate 10–100 Kyr → surface process Kyr → fast/biological yr–centuries) | ` #detail-phenomena-systems-map ` **FE(2)** | With the fast-sees-slow-quasi-static / slow-sees-fast-averaged rule; law homes at ` #form-scale-separation-directional ` and ` #form-seam-flux-exchange `. |
| ARCH §9 **add-a-system six-clause contract + CHONK prior art** | ` #form-add-system-contract ` | The most complete single peel in the slice — all six clauses, both CHONK transfers, the single-resolution caveat, and the domain-fixation guard. |
| ARCH §8(a)(b)(c) **open-problem inventory correction** (superlative struck; statement stale; "we do not currently know what this project's open problems are") | ` #disc-open-problem-census ` | Not just preserved — converted from a one-time correction into a standing habit ("open-problem lists are untrusted until derived from core"), with the census itself derived. |
| ARCH §5 `spikes/worldview` is a physics testbench, not the runtime; §8 status-quo gap list (store+nomos, coarse spine, flux-BC tiles, query front-end, RNG fix) | ` #detail-abyssal-parity-build ` FE(2), FE(4) phases 0–5 | Faithful, including "kernels proven; world-frame unbuilt" and the RNG deferral. |
| archive-report #2 backlog: *execution-class, timescale-band, stage lifecycle, phase binding as declaration data* | ` #form-kernel-imperative-boundary ` **FE(5)** | Execution-class and timescale-band landed on `NomosDecl` 2026-07-24; **stage lifecycle and phase binding are named as still-open residual debt** rather than quietly dropped. This is the honest-bookkeeping case done right. |
| archive-report #7 identifiability bet ("nowhere live after re-founding") | ` #disc-aat-vivarium-object-map ` WN(4) + ` #sketch-two-layer-mind ` FE(5) | Landed after that report was written; Level-C framing intact. |
| session §1 probe sensitivity · §4 declared dishonesty is disclosure not license | ` #norm-probe-sensitivity ` · ` #norm-declared-violation-is-not-license ` | Strong peels; §1 keeps the paid-for specimen and the scale-separation technique. |

---

## 3. Where the peel strengthened rather than preserved

Four places. Naming them because each one changes what you should design against.

**3.1 The build-time source digest retired four named correctness holes.** The decisions-code report §3 listed five open under-keying residuals against ` #form-complete-content-addressed-key `: (1) transitive dep versions, (2) `UPLIFT` using `noise::fbm` with `deps: []`, (3) `HYDROSPHERE`/`CLIMATE`/`WATER` reading `Planet::EARTH` constants outside the key, (4) hand-stamped versions load-bearing, (5) provisional builds writing lawful-looking roots. Residuals 1–4 are **subsumed** by the whole-crate digest (`build.rs` → `VIVARIUM_SRC_HASH` → `NomosDecl::key` at `nomotheke.rs:353-355` folds `field("src", SRC_HASH)` into every key stem): any `.rs` edit — including a constant, a `deps: []` omission, or an intermediate whose version nobody bumped — changes every world-law key. Residual 5 is closed differently: `PutOpts` third-line `provisional` flag, `Source::HitProvisional`, `vivarium status` counts, `tests/cli_admission.rs` convicting it at the argv boundary (` #form-builder-admission ` Epistemic Status).

The source doc's framing was *"the highest-value structural item not yet built"* (toolchain.md §standing principle). It is built. Do not design as if it isn't.

**3.2 ` #disc-open-problem-census `** converts a correction into a repeatable derivation (above).

**3.3 ` #detail-seam-precedents ` FE(7)** adds the honest cost of attention-driven refinement (above).

**3.4 ` #form-store-as-save ` FE(6)** hardened "memoize pervasively" from a cost-conditional design preference (ARCH §5) into a definitional law with two named carve-outs — *"a per-process cache holding results worth keeping past the working set is a bug against this law"* — under `DECISIONS[memoized-means-store-object]`, `:by joseph`. That is stricter than anything in ice, and it constrains harness design directly: any warm-cache you are tempted to add for the background builder has to be working-set staging, not a durability tier.

---

## 4. The middle category — in core but weakened, partial, or self-contradicting

This is the part you said would bite. Ordered by how much it bites.

### 4.1 ` #form-complete-content-addressed-key ` contradicts itself on exactly the property you need

The Discussion (line 37) says:

> "This is one of the three walls that keep parallel system work and fidelity-ladder swaps cache-transparent: **change a nomos version and exactly its dependent cone invalidates; everything else stays.**"

That sentence is ARCHITECTURE §5's promise, carried across intact. Under the live mechanism it is false: FE(4) (line 20) and Known-incomplete (3) (line 31) both state the digest is whole-crate, and Known-incomplete (3) says so plainly — *"editing any `.rs` (including a comment or a test) invalidates the whole store."* Your 24/24 measurement is the FE, not a bug.

So the segment's FE and its Epistemic Status are honest and its Discussion is a ghost of the pre-digest design. **Everything you read in the Discussion about cone-transparency, and everything downstream that cites "the three walls," reads as a shipped property and is an aspiration.** That is the single highest-value row in this report for a designer, because "restarting at the right points as we iterate" *is* cone-transparency.

Two more things core already says that you should design *with*, not around:

- Core has a **reasoned position against** the obvious fix, twice: Known-incomplete (3) — *"any hand-maintained source→nomos map would reintroduce the bump-discipline this removes"* — and ` #detail-epistemics-toolchain ` FE(5) — *"per-nomos attribution deferred (would reintroduce a hand-maintained source→nomos map)."* Consistent across two segments, so it is a stance, not an oversight.
- It also carries the **named unblock condition**: *"the ceiling waits until the module graph makes attribution derivable."* That is a specific, checkable gate. A design that makes per-nomos source attribution *derivable* (rather than declared) satisfies core's own stated condition and does not need to argue against the stance.

Also relevant and easy to miss: over-keying is the safe direction by FE(2), so any attribution scheme has to be sound-by-construction, not best-effort. A wrong-but-finer key serves a memo that lies.

**Verify:** `core/src/form-complete-content-addressed-key.md` lines 18–20, 29–31, 37; `crates/vivarium-world/build.rs`; `crates/vivarium-world/src/nomotheke.rs:353-355`; `core/src/detail-epistemics-toolchain.md` FE(5).

### 4.2 ` #form-in-vivia-citation ` is stale on the same fact

Epistemic Status honest-gap (2): *"Hand-stamped nomos versions remain present practice; source-derived versions are the target remedy."* Superseded 2026-07-24 by FE(4) of the key segment. Cheap fix; flagging because it makes the citation infrastructure look one rung less built than it is, and *in vivia* citability is downstream of exactly the thing you are about to touch.

### 4.3 The harness-operations layer of `toolchain.md` did not survive

` #detail-epistemics-toolchain ` FE(3)–(4) faithfully carry the adopt-later queue and every skip reason. What went is the operational residue — small individually, and collectively it is precisely the layer you are about to rebuild:

| Dropped from toolchain.md | Where it now lives |
|---|---|
| `cargo-mutants`: *"always `--package vivarium-world`, or every mutant rebuilds the Bevy graph"* | nowhere |
| `proptest`: *"commit `proptest-regressions/` like goldens, knowing the files persist **seeds**, not values — best-effort replay, not exact"* | nowhere |
| clippy: *"Requires `rustup component add clippy`"* | nowhere |
| *"Examples/probes still use `HashMap` — not under this gate yet"* | `clippy.toml` header comment ("Views/spikes are not gated the same way yet") — not in core |
| *"Full-workspace `-D warnings` (incl. Bevy spikes) is deferred"* | `bin/check` header comment ("Not full-workspace -D warnings yet") — not in core |

Core FE(2) does scope the gate correctly (*"on `vivarium-world` lib"*), so nothing is misstated. But the last two rows are **compliance-debt facts that live only in code comments**, which inverts the project's own code-rank rule. If you touch `bin/check` while building the harness, that is the moment to promote them into FE(2) or its Working Notes.

Also gone, and different in kind because it is a forward direction rather than a gotcha: the `uom` skip reason survives, but its successor does not — *"the honest next rung for `Quantity` is interval arithmetic (`inari`), which its own doc-comment already anticipates — when a consumer needs guaranteed-vs-approximate **bounds**, not just the flag."* That connects straight to ` #form-column-control-volume ` FE(3)'s exactness claim, and it is the only named path from the current boolean flag to something a consumer could compute with. It exists in ice and in a Rust doc-comment; not in core.

### 4.4 "The participant is the clock" lost its elastic half

ARCHITECTURE §4: *"a human clamps the world to ~2 Hz real-time; **a pure-agent world has fully elastic logical time**."* Core keeps the first half (` #detail-vivium-lifecycle ` FE(3), "~2 Hz-compatible human-perceptual grade where a human is the clock") and drops the second. Grep for `elastic`, `logical time`, `real-time`, `wall-clock` across `core/src/` returns nothing on point.

This is the regime Joseph's first clause describes — *world building working in the background*, with no human clamping the clock. The claim that logical time is unclamped when nobody is watching is the license for a background builder to run at whatever rate it can, and for an explorer to *join* a running world rather than drive it. It is one sentence, it was in the corpus, and it is the closest thing the corpus had to a statement of what "background" means.

### 4.5 Replay: referents pinned, carve open, view side unowned

The mapping survives in `LEXICON.udon` (§2 above) and the carve is an honest `OUTLINE` §III gap with a matching row in ` #disc-open-problem-census ` FE(2). ` #form-store-as-save ` FE(8) holds the frame (canon-root discipline; mechanization is compliance debt). ` #detail-vivium-lifecycle ` FE(4) BREAK-4 has the sharpest live constraint on it: *cross-platform FP non-determinism breaks replay-from-seed alone → publish phase-memos, not just seeds.* Nothing lost here. But "watching replays" as an explorer-facing capability has no owner beyond the carve.

### 4.6 "Principled understanding of what exactly they are watching" has no owner anywhere

The pieces exist and nothing joins them:

- `Source::HitProvisional` and `vivarium status` provisional counts — ` #form-builder-admission ` Epistemic Status (built)
- fidelity pyramid (histogram roots by level×stage), watchpoints (declared place/level/stage snapshots), telemetry-by-construction — ` #detail-builder-daemon ` FE(2), **builder-side instruments, unbuilt**
- four epistemic axes A/B/C/D per phenomenon — ` #disc-vivarium-purpose ` FE(3); weakest-link `derived_physics` / `derived_earth` folds on `NomosDecl`
- peer views incl. headless logger and ASCII instruments — ` #form-core-view-wall ` FE(3)
- *"roam Realized early-Abyssal, persistent, **epistemic-tagged**, no endo mind"* — ` #detail-abyssal-parity-build ` FE(4) phase 5

That last item is the whole of the explorer-legibility claim: one adjective in a definition-of-done. Nobody owns *the explorer must be able to say, for what it is currently rendering, which nomos versions, which fidelity tier, which epoch, and whether the bytes are lawful or provisional.* I checked ice for it too; it is not there either. **This is a gap in both layers, not a peel failure** — which is good news for your time budget and bad news for the "design from accumulated thinking" premise on this one clause.

---

## 5. Nowhere in core

Four items. Two are recorded-as-dropped (honest); two are unowned.

**5.1 The store-as-bus multi-process validation (session §3) — unowned, and it is the evidence under your architecture.** The design is clean at ` #detail-builder-daemon ` FE(1): *"No mediator daemon for truth… readers safe against writers without protocol. Two processes computing the same key write identical bytes — benign race by construction."* The **evidence** is only in ice: on 2026-07-10 an engine session and a view agent worked the same repo simultaneously, interleaved commits on `main`, three short messages total, the view agent fixing a store race the engine author wrote — *"the store-is-the-bus design was validated by its first real multi-process day — including surfacing its one latent race early and cheaply."*

Under ` #norm-probes-before-claims ` FE(1), "readers safe against writers without protocol" is a behavior claim. It currently has no probe that can fail and no recorded observation. It is also the exact claim a background-builder-plus-live-explorer design rests on, and the one historical datum says *there was a latent race, and it took a second process to find it.* Candidate homes: an `observation` segment, or a Working Note on ` #detail-builder-daemon ` plus a concurrency probe. Cheap either way; I did not write it (your file only).

**5.2 Compensating bugs / chirality (session §2) — deliberately dropped, recorded.** ` #norm-probe-sensitivity ` Working Notes: *"compensating bugs (§2) stay process texture."* So the drop is honest and traceable. Flagging it only because your job is the view-as-instrument, and the two morals are constraints on that channel: *screenshots cannot catch chirality bugs without a chirality reference* (a mirrored coastline reads as a coastline), and *when observation contradicts a careful derivation, the derivation has an unmodeled assumption.* The globe rendered inside-out for a while and the drag had been tuned against the mirrored world; fixing one broke the other. If you are building "watch it in real-time and know what you're watching," the first half of that pair is a design requirement, not an anecdote.

**5.3 "Instrument before tuning by feel" — slogan kept, method dropped.** ` #norm-probes-before-claims ` Discussion carries *"instruments before tuning by feel."* The method behind it — **bench + noise floor + one variable at a time** — is not in core. The archive report (#6) deferred it on purpose ("would dual-home"). Honest deferral; still a gap if a perf harness is part of what you build.

**5.4 The catalyst loop (session §5) — circular pointers, no owner.** *"After installing an honesty mechanism, expect and budget for the work it immediately surfaces — that surfacing is the mechanism succeeding, not scope creep."* ` #norm-declared-violation-is-not-license ` WN says it lives in ` #norm-probe-sensitivity ` / ` #norm-regime-probes ` / process reflections; ` #norm-probe-sensitivity ` WN says *"honesty work-queue (§5) is catalyst meta."* Each points at the other; neither owns it. Small, but it is a planning claim and you are about to install honesty mechanisms.

---

## 6. On your framing — one adjustment I'd offer

You framed this as *did the peel lose workflow / CLI / harness thinking.* Having read the slice: **the corpus never had much workflow thinking to lose.** What it had was (a) architecture — the daemon design, demand/beacons/cones, store-as-bus, depend-by-key, the three-scoped decomposition — which is peeled thoroughly and in some places improved; and (b) tooling adoption — `toolchain.md`, which is peeled minus its operations residue (§4.3).

The CLI in particular was never a subject. `toolchain.md` does not mention it. ARCHITECTURE mentions it once, dismissively (§9: *"'add a system' means a function and a CLI flag"* — as the failure mode). The nearest thing to a CLI claim in the whole slice is decisions-code's disposition of `DECISIONS[cli-world-dir-default-and-symlink-promotion]`: *"Tooling convenience; not world-law."* Your own `DECISIONS[build-cli-parameterization-is-manifest-and-nomos-debt]` and `msc/build-parameterization-findings-2026-07-24.md` are, as far as I can see, the first time the CLI surface has been treated as owing anything — and they land in the right place, because ` #form-manifest-prescribes-vivium ` FE(2) already names *"participation / demand posture for builders and explorers"* as manifest content, with FE(5) honest that `spec` carries a thin subset. That is the design-from-accumulated-thinking answer for build parameterization: it is not a CLI question, it is the manifest's unbuilt half, and core said so before you measured it.

So the condition Joseph attached is, for this slice, met — with §4.1 as the one place where meeting it means *not* trusting a sentence in core.

---

## 7. Feedback on the brief

It worked. Three things specifically:

- Withholding your hypothesis was right and I could feel it working — I spent the first hour with no prior about whether gaps existed, which is why §4.1 (a self-contradiction, not a gap) surfaced at all. A gap-hunting frame would have found nothing there.
- "A grep won't settle this — the question is whether a *thought* survived" was the single most load-bearing line. It is why I read ` #detail-seam-precedents ` at all.
- Naming your own file-list failure ("it missed a file literally named `vivium-operational-workflow.md`") did real work: I treated your eight-segment guess list as vocabulary rather than boundary, and five of the homes in §2 are outside it.

One thing the brief withheld that I'd have used: **what you already believe about the store's shape.** Your measurements section gave me tiling, timing, and compression numbers, but not whether you are contemplating changing the store layout, the key scheme, or the object granularity. §4.1 and §3.4 land differently depending on that — if you are considering per-nomos or per-module source attribution, core's stance and its named unblock condition are the first thing to read; if you are considering storage tiering only, ` #form-store-as-save ` FE(5)/(7) (invalidation vs eviction; regenerable vs irreducible) is, and the tiering question is *already framed there* as policy over that split. I hedged by covering both, which cost some sharpness.

Happy to stay on the line. The follow-ups I'd expect to be useful: whether §5.1 should become an `observation` segment or a Working Note plus probe; and a closer read of what ` #form-store-as-save ` FE(6)'s definitional "memoized means store object" permits a background builder to hold in RAM, since that is the clause most likely to constrain a daemon design and I read it once, not three times.
