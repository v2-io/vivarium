# Convictable declarations — design sketch (versions, keys, assumption anchors)

*2026-07-21. Design only — nothing here is built, and nothing here is decided. Written by a Claude (Opus 4.8) session at Grok's request following de-novo audit B (asks 4 and 5). Everything below is `:by claude` / proposal-grade; the shape is offered so the call can be made on something concrete rather than invented solo.*

## Why the two asks are one ask

Ask 4 (source-derived nomos version) and ask 5 (assumptions reverse-guard) look like separate chores. They are the same failure wearing two faces, and there is a third face neither ask named:

| # | Declaration | What is supposed to be true | What fails when it is false | Today |
|---|---|---|---|---|
| A | `NomosDecl::version` | names the algorithm's identity | nothing | hand-stamped |
| B | The store key | folds in everything that changes the bytes | nothing | hand-built per query |
| C | `NomosDecl::assumptions` | names every magic constant the kernel leans on | only the *declared→ledger* direction | substring check |

` #norm-declaration-must-convict` FE(1) is the test: a declaration is real only if something in the executable path can fail when it is false. All three currently fail that test in the direction the risk lives.

## The evidence, at honest rungs

**Measured (2026-07-21).** In a fresh world, `initial-topography` swept 24 tiles, then re-swept as 24 hits. `NOISE.version` was then changed and the binary rebuilt — `initial-topography` swept **24 hits, 0 computed**. `initial-topography` declares `deps: &[&NOISE]`, and `initial_topography_key` folds seed / face / level / oi / oj / nx and no noise version. So a declared dependency's identity change does not invalidate the dependent memo. That is under-keying — the failure ` #form-complete-content-addressed-key` FE(2) calls "a stale memo that *lies*" — live on the main store path, not in a spike. (Source was restored immediately; the experiment left no trace.)

**Read-verified, not run.** The same gap is transitive elsewhere. `water_key` folds `EROSION.version`, `INITIAL_TOPOGRAPHY.version`, `CLIMATE.version` — but not `UPLIFT.version`, which reaches it through erosion. `erosion_key` folds its three direct deps but not `NOISE` or `HYDROSPHERE`, which reach it transitively. Because keys fold dependency *version strings* rather than dependency *key hashes*, transitivity has to be re-done by hand at every consumer, and it has not been.

**Structural.** `declarations_mint_the_keys` pins the current version constants, so it fails when you *do* bump and passes when you forget. `consumed_and_met_implies_in_deps` checks the declaration graph, not the key. Nothing relates `deps` to what the key builder actually folds — which is exactly the seam the measured defect fell through.

**Context worth holding.** The asf side's 2026-07-16 note (`feedback-from-asf.md`, Level-4 handshake item 3) argues that no distributional equivalence test between nomos versions can soundly license cache reuse across a version swap — pointwise-on-keyed-draws is the *uniquely* sound rule. If that holds, key completeness is not a conservative habit that could be relaxed under measurement; it is the only sound policy, and the mechanism gap above is the gap between the policy and its enforcement.

## Sketch B first — derive the key from the declaration

Taking B before A is deliberate: B is cheaper, it closes the measured defect, and it makes A's payoff automatic instead of additive.

The shape: a nomos's key is not hand-assembled at each call site. It is derived from the declaration the nomos already carries.

```
key(nomos, coords) = nomos.name @ nomos.identity
                   | seed
                   | <coords the caller supplies: face, level, oi, oj, nx, epochs, …>
                   | dep_1.identity … dep_n.identity      ← from nomos.deps, not by hand
```

Two properties worth wanting:

1. **Dep folding is derived, not typed.** Walking `nomos.deps` at key-construction time means a new `deps` entry is folded in the moment it is declared. The measured `NOISE` defect becomes unrepresentable rather than fixed.
2. **Transitivity comes free if the folded thing is an identity, not a version string.** If each nomos's `identity` is itself a fold over its own deps' identities (a Merkle-shaped identity rather than a flat constant), then a change anywhere upstream propagates to every downstream key without any consumer knowing the graph. `water` stops needing to know `uplift` exists.

Cheap intermediate if the full derivation is too much at once: a test that asserts, for every nomos, that its key string contains an identity field for each entry in its `deps` — the same shape as the existing `consumed_and_met_implies_in_deps`, applied to keys instead of declarations. That converts the seam into a failing build without changing any key format. It would have caught the measured defect. It is the smallest thing here with real teeth.

**Cost that must be stated honestly:** any change to how identities compose changes every key, which invalidates the whole store once. That is a recompute, not a correctness event, and the store is currently small — which argues for doing it before the store holds anything unrecomputable, not after. `#form-complete-content-addressed-key`'s known-incomplete (2) makes the same argument about the FNV-1a hash width, and the two changes want the same migration window.

## Sketch A — source-derived nomos version

The remedy the segments already name (` #def-nomos` "Version honesty"; ` #form-complete-content-addressed-key` FE(4)). Options, roughly in increasing cost and increasing fidelity:

1. **Build-time hash of the kernel's source file(s)**, coarse enough to cover in-crate transitive dependencies — e.g. a `build.rs` that hashes the module files a nomos declares as its kernel, emitting a constant the declaration reads. Catches every edit to the kernel. Also fires on comment and whitespace edits, which is noise but *safe* noise: it over-keys, and over-keying costs recompute only. Given FE(2)'s stated asymmetry, that trade is already the project's answer.
2. **Hash of the crate's source tree.** Simpler, coarser, invalidates everything on any edit anywhere. Probably too coarse to live with during iteration, which is precisely the "cache-disable as workflow" failure FE(3) forbids — so this is likely a no-go rather than a rung.
3. **IR-normalized hashing** (hash after compilation, normalizing away formatting). ` #form-complete-content-addressed-key` FE(4) already marks this "non-dogmatic tightening, not settled law." Worth naming as the ceiling and not pursuing first.

**The one thing to preserve across all three:** a human-readable stem. `erosion-tile@a3f1…` is unreadable in a log, and the build log and status output are how the project currently sees itself. Something like `erosion-tile@2026-07-12b+a3f1` keeps the date-stamped intent legible while the hash carries the conviction. The hand-stamped part becomes a label; the derived part becomes the identity.

**What this does not solve.** `spikes/worldview`'s `FILL_ALGO_VERSION` is a separate cache with a separate hand-built key, outside the nomotheke entirely. Either it adopts the same derivation or it stays explicitly a testbench cache that no citation may rest on. Worth deciding rather than letting it drift.

## Sketch C — the assumptions reverse-guard

The present guard checks *declared anchor → appears in `ASSUMPTIONS.md`*. Three additions, in the order I would want them:

1. **Ledger → declared.** Every anchor row in `ASSUMPTIONS.md` is claimed by at least one nomos, or is explicitly marked retired. Cheap, purely additive, and it stops the ledger accumulating rows for constants nobody uses. Needs a stable row format to parse — which the ledger's own preamble already wants ("it should not be prose … the destination is **udon**"), so this may be the forcing function for that move rather than a thing to build against markdown.
2. **Tighten the existing check from substring to row.** `LEDGER.contains(anchor)` passes if the anchor string appears anywhere, including inside a sentence retiring it. Matching a parsed row rather than raw text costs little and removes a false-pass class.
3. **Code → declared.** The direction Joseph actually asked the ledger for ("we don't end up with guesses and magic constants scattered through the nomoi"), and the hard one. A literal-scanner over kernel modules is the obvious move and is likely to drown in false positives (array indices, `0.5`, unit conversions), so the honest options are probably: a lint that flags *named* constants (`const FOO: f32 = …`) in nomos modules that no declaration claims — narrow, low-noise, catches the real class — or a convention where magic constants must come from a declared parameter block rather than a bare `const`, making the scanner unnecessary. The second is more invasive and more terminal. Neither is obvious enough to pick from here.

## What I would not do

- **Do not pick a successor hash here.** ` #form-complete-content-addressed-key` known-incomplete (2) says explicitly that BLAKE3 is not decided and should not be treated as decided until DECISIONS records it. That still holds; this memo does not record it either.
- **Do not implement B's Merkle identity before the migration window is a decision.** It invalidates the store once, and choosing when is Joseph's call, not a side effect of a fix.
- **Do not fold the waiver into the key as a way of closing the `--allow-unmet` census gap.** It is tempting (it would make provisional artifacts key-distinct and therefore visibly separate), but it silently doubles the key space and makes a waived run's artifacts permanently un-reusable by a later lawful run even when the physics is identical. Tagging provisional artifacts in the census is the narrower answer to that problem, and `#form-flux-web` already names it as known-incomplete (3).

## Open questions I could not resolve from here

1. Should `identity` be Merkle-shaped (folding deps) or flat-with-explicit-dep-fields? Merkle is cleaner and makes transitivity unrepresentable-wrong; flat is more legible in a log and easier to debug. This is a real trade and I do not think it is mine to call.
2. Does the ledger move to udon *before* the reverse-guard, so the guard parses data rather than prose? Sketch C item 1 is much cheaper after that move than before it.
3. Is `spikes/worldview`'s cache in scope for the nomotheke discipline at all, or explicitly out?
