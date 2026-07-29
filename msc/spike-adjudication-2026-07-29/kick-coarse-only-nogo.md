# Verdict: seal-with-scope-tightening — SUPERSEDED, see addendum below: DO-NOT-SEAL until re-measured on current main

The measured negative result stands. The *generalization* the headline claim makes beyond
what was measured does not, and it's the same disease as the `#sketch-detail-abstract-reversion`
precedent: a summary sentence claiming more than the underlying probe established.

## The exact wording problem

**DECISIONS.decision-log.udon:1063** and **core/src/obs-coarse-only-closure-nogo.md** section 3
both assert:

> "$R^2$ bounds EVERY monotone pointwise map $A_{coarse} \mapsto A_{eff}$, not just the power
> law ⇒ no pointwise coarse-only closure reaches the trunk."

I checked what was actually computed. `probes.rs:628-647` (`loglog_fit`) is a plain **Pearson R²
of an OLS fit in log-log space** — i.e., R² of the specific two-parameter family
`y = α·xᵝ`. That's the only fit anywhere in the spike (`grep -n "quantile|isotonic|rank|monoton"`
across `probes.rs area.rs mra.rs main.rs` returns nothing but the one comment asserting the
bound, `probes.rs:761`).

That claim is backwards. A power-law fit's R² is a **lower** bound on the best-achievable R²
over the class of *all* monotone functions, not an upper bound — because power-law is a strict
two-parameter subfamily of "monotone function." The actual ceiling over all monotone functions
is the isotonic-regression fit's R² (or equivalently the correlation ratio η² computed via
conditional means / binning), and isotonic regression's R² is *always* ≥ any specific parametric
fit's R², including log-log linear. Nothing in the spike computes that. So "0.06–0.36" is a
real, solid ceiling for **power-law recalibration** (and its β=1 special case, pure "scale," which
*is* legitimately subsumed since it's a restricted case of the same family) — but it is not
evidence against a quantile-mapping / isotonic / other general-monotone recalibration, which
could do meaningfully better on the same data. Unless I'm missing a justification that isn't in
the code or the pre-registration (`PREDICTIONS.md` P6/P8 only ever describe the power-law fit
too), this line oversells.

**Compounding evidence of the overreach:** `core/src/obs-coarse-only-closure-nogo.md:98`
(Working Notes / regression guard) explicitly names three families as excluded — "do not
re-propose a pointwise recalibration of $A_{coarse}$ (scale, power-law, value-quantile) as the
closure." "Value-quantile" was never implemented or measured anywhere in the spike. It's being
excluded by citation to a ceiling that (per above) doesn't bound it.

## What does hold up (checked against RUN.txt, not just the prose)

- **PROBE 0 anchor**: DECISIONS:1061 and segment both correctly re-anchor the stale +5.34m
  wavelet number to the fresh +7.17m/RMS 12.45m/bias-noise 0.71 measurement — matches
  `RUN.txt` PROBE 0 exactly, and the submarine-footprint caveat is handled the same honest way
  as the neighboring `mean-pin-retirement` entry (DECISIONS:1080) did for a similar staleness
  bug. Good hygiene, not a fresh problem.
- **The measured R² numbers themselves are accurate.** DECISIONS/segment "0.06–0.36... ceiling
  0.359" matches `RUN.txt` PROBE 6 (6 fits, max 0.346) and PROBE 8 (12 fits across 2 seeds ×
  3 *different* tiles × 2 depths, max 0.359) exactly. That's actually 18 total (tile,depth) fits
  across two independent tile sets, not just the 12 the text cites — if anything the text
  under-states the convergent evidence for the power-law-specific ceiling.
- **Tile selection is disclosed and not obviously gamed**: `probes.rs` `find_land_tiles` /
  `scan_for_land` require 100% land and relief > 150m — the same criterion is reused for both
  the 3-tile and PROBE-8 tile sets, and it's a plausible neutral land-guard (matching the same
  land-guard lesson from the PROBE 0 submarine bug), not an obvious cherry-pick toward a low R².
- **m=0.5, n=1 and depth-1/2-only scope are honestly flagged as open**, DECISIONS:1066
  ("UNTESTED: exponents other than m=0.5,n=1; deposition/MFD-p regimes; whether a two-variable
  coarse carrier... raises the 0.36 ceiling"). Good — this is exactly the kind of honest scoping
  the R²-bound line above is missing.
- **The "what is NOT excluded" section (segment item 5)** already correctly scopes the exclusion
  to "pointwise-in-A_coarse" and leaves topology/sub-grid-summary/fine-spine open. It just
  doesn't also flag that *general nonparametric monotone* recalibration (isotonic/quantile) is
  equally untested — that's the fix.

## One more small textual issue, DECISIONS-only (segment is fine)

**DECISIONS:1063**: "Held-out, the tile-1 fit makes bias/noise WORSE (0.71 → 0.90)." Checked
against `RUN.txt` PROBE 6/7 table: the row `ZNeg(327680,65536) 2 +7.17m 0.70 → -6.99m 0.90` is
labeled `(fit)` in RUN.txt, not `HELD-OUT` — it's the calibration tile itself, not a held-out
one. **The segment gets this right** (`core/src/obs-coarse-only-closure-nogo.md`, section 3:
"the fit makes the bias worse: bias/noise rises 0.71 → 0.90 **even on the tile it was fit
to**") — the qualifier is there. DECISIONS' one-line compression drops the qualifier and reads
as if 0.71→0.90 is itself a held-out data point, which it isn't (the genuinely held-out rows are
milder-to-comparable: 0.54→0.69, 0.46→0.79, 0.64→0.86 — still all worse than baseline, so the
qualitative conclusion "recalibration makes it worse, including held out" is still true and
arguably has *better* supporting numbers available). Low stakes since the segment is canon, but
worth fixing in DECISIONS' shorthand while touching this entry anyway.

## Exact wording I'd want changed before sealing

1. **DECISIONS:1063** and **segment section 3**: delete or rewrite "$R^2$ bounds EVERY monotone
   pointwise map ... not just the power law" → something like "$R^2$ bounds every closure in the
   power-law family (of which pure scaling, β=1, is a special case); general nonparametric
   monotone recalibration (e.g. quantile mapping / isotonic regression) was not tested against
   this ceiling."
2. **segment `Working Notes` / regression guard** (line 98): drop "value-quantile" from the
   excluded list, or move it to a new explicit open item next to item 5 ("also untested: a
   general monotone (non-power-law) recalibration such as quantile mapping").
3. **DECISIONS headline / segment title** ("no pointwise coarse-only closure exists" / "The
   coarse-only trunk closure is a measured no-go"): consider whether these should read
   "power-law" instead of the unqualified "pointwise" — the unqualified version is the
   overclaim; Joseph's call on how much the title needs to say vs. the body carrying the scope.
4. (optional, minor) **DECISIONS:1063** "Held-out, the tile-1 fit..." → cite one of the actually
   held-out rows, or add "(even on the fit tile itself)" as the segment does.

## Strengthen-first option, if preferred over scope-tightening

Per house doctrine, the harder-but-better move would be to actually compute the general-monotone
ceiling (isotonic regression or a binned correlation ratio η² on the same `(A_coarse,
max(A_fine))` pairs already produced by `tile_fields`/`tile_fields_seeded`) rather than just
narrowing the wording. That's a small, cheap addition to `probes.rs` in kind (no new data
collection, same pairs) — but the spike currently doesn't compile against main
(`NOTE-integration-drafts.md` / your framing confirms this), so it would need porting first. If
that's run and the general-monotone ceiling *also* lands under ~0.4-0.5, the broader "no
pointwise coarse-only closure" claim would then be genuinely earned rather than asserted, and the
title/headline could stay as-is. I did not attempt this myself (read-only per the brief, and it
would require getting the spike compiling again) — flagging it as the strengthening path rather
than assuming scope-tightening is the only option.

## Everything else I checked and found solid, for the record

- PROBE 1-5 mechanism story (routing/rerouting, not miscalibration; sub-grid channel position
  not encoded by any per-cell coarse quantity) — numbers in DECISIONS:1062-1064 match RUN.txt
  PROBE 1-5 exactly, and the "sub-grid A^m variance vs gap r≈-0.11" figure matches PROBE 2's
  `-0.113`.
- Depends/ref block in the segment frontmatter and `|ref` line in DECISIONS both point at real,
  present files (`PREDICTIONS.md`, `RUN.txt`, `src/{main,probes,area,mra}.rs` all exist).
- No sign of the `sketch-detail-abstract-reversion` depth-1-vs-depth-2 disease specifically
  (i.e., no claim here is silently depth-2-only while reading as depth-general) — the depth
  scope (1 and 2 only) is explicitly named as untested-beyond in the impact block.

Files checked: `DECISIONS.decision-log.udon:1057-1067`,
`core/src/obs-coarse-only-closure-nogo.md` (full), `.super-archive/from-msc/spike-nonlocal-closure/{PREDICTIONS.md,RUN.txt}` (full),
`.super-archive/from-msc/spike-nonlocal-closure/src/probes.rs` (tile selection, `loglog_fit`,
`harden_ceiling`, `deployable_closure` — lines ~1-50, ~600-800).

---

## Addendum — strengthen-first attempt (mandate extended, 2026-07-29)

Per `#norm-probe-sensitivity` / `#norm-caught-disciplines-become-mechanisms`, pre-registering
before running.

### Pre-registration (written BEFORE the first run of `monotone_ceiling_probe`)

**What's being computed:** for the same 12 (tile, depth) cells named in `RUN.txt` PROBE 8
(seed 0: XPos(327680,245760)/(311296,262144)/(344064,278528); seed 1: XPos(229376,49152)/
(245760,49152)/(278528,131072); depths 1 and 2 each), recompute `(A_coarse, max(A_fine))` pairs
directly from the live kernel (`Fluvial::from_prior` for the fine run, an area-weighted-mean
restriction of the initial topography + `Fluvial::from_surface`/`erode` for the coarse run —
the same construction `tile_fields_seeded` used, minus the archived `mra` module, which I'm
replacing with a direct area-weighted block mean since that's mathematically what
`mra::decompose(...).root.v` reduces to for the coarse **scaling coefficient** specifically —
the predictor choice the module's own docs say only affects detail/compression, never the
coarse value). Then fit **isotonic regression** (PAVA, monotone non-decreasing, least-squares —
chosen over a binned correlation ratio η² because it is the closed-form argmin over the *entire*
class of monotone functions in the L2 sense, so its R² is not sensitive to a bin-width choice
the way η² would be) on the same point sets the power-law fit used (`x>0 && y>0` filter,
matching `loglog_fit`), and report its R² per cell and the max over the 12 as the general-monotone
ceiling, for direct comparison against the measured power-law ceiling of 0.359.

**Prediction, stated before running:** I expect the isotonic ceiling to land **noticeably above
0.359 but still comfortably below 0.5** — my best guess is in the **0.4–0.5** range. Reasoning:
isotonic regression is a strictly larger hypothesis class than log-log power-law (β,α fixed
functional form vs. arbitrary monotone step function), so its R² is mathematically guaranteed
to be ≥ the power-law R² on the *same* data, and PROBE 6's fitted β's cluster near 0.5–0.68 (not
close to 1), suggesting the true relationship has curvature a 2-parameter power law can already
capture reasonably well — so I don't expect a huge jump. The interesting bracket per the brief:
**under ~0.4–0.5** would earn the broad "no pointwise coarse-only closure" claim as genuinely
measured rather than asserted (the isotonic-vs-power-law gap is real but small, power law was
already capturing most of the achievable monotone structure); **near 0.359** would mean the
power law was *already* the ceiling and the "not just the power law" line was, in a sense, true
by accident despite the loglog_fit-only argument not proving it; **substantially above 0.5**
would mean the current no-go claim is genuinely false, not just overshooting its evidence — a
real class of monotone (but non-power-law) closures would recover most of the trunk variance.

### Result: the isotonic question got answered, and then overtaken by a much bigger finding

**Probe built:** `crates/vivarium-world/examples/monotone_ceiling_probe.rs` (new file, live tree,
outside `SRC_HASH`). Recomputes `(A_coarse, max(A_fine))` on the exact 12 (tile, depth)
footprints from `RUN.txt` PROBE 8, fits both the archived `loglog_fit` (reproduced verbatim, for
a cross-check) and isotonic regression (PAVA) on the same `x>0 && y>0` point sets.

**First run, on current `main` (HEAD `f7f7740`):**

```
seed tile                 dep      n  loglog_R2 iso_R2
0    XPos(327680,245760)    1   4096      0.940  0.963
0    XPos(311296,262144)    2   1024      0.814  0.976
1    XPos(245760,49152)     2   1024      0.692  0.380
...
power-law ceiling (max over 12 cells) = 0.940   (RUN.txt PROBE 8 reported 0.359)
GENERAL-MONOTONE ceiling (max isotonic R2)  = 0.982
```

**This does not match the archived 0.06–0.36 AT ALL** — not "isotonic is a bit higher than
power-law," but the power-law fit *itself*, reproduced with the same code path (`loglog_fit`,
same filter, same footprints), now measures 0.94 instead of 0.359. Before trusting that as a
real finding rather than a bug in my reimplementation, I checked it against the spike's own
commit:

**Control run, `git worktree add --detach <scratch> bd21400`** (the exact commit that landed the
archived measurement), same probe file copied in unmodified, `cargo run --release`:

```
power-law ceiling (max over 12 cells) = 0.362   (RUN.txt PROBE 8 reported 0.359)
GENERAL-MONOTONE ceiling (max isotonic R2)      = 0.401
```

**This matches the archived number almost exactly** (0.362 vs. 0.359 — the ~0.003 residual is
plausibly the `area_weighted_coarsen` vs. the archived `mra::decompose` bilinear-predictor path,
which the spike's own module doc says should agree exactly on the coarse *scaling coefficient*
regardless of predictor; close enough to call it noise, not a bug). **This validates the probe
implementation** — `tile_pairs`/`loglog_r2`/`isotonic_r2` are doing what they claim to do — and
it isolates the 0.359→0.940 jump to something that changed **in the kernel or generator between
`bd21400` (2026-07-24 12:41, the spike) and `HEAD`**, not to a reimplementation error.

**Bisection (partial, time-boxed, not exhaustive):**

- At `d9a24c4` (2026-07-24 12:57, 16 min after the spike — "Mantle-thermal nomos: the chain's
  head is law, and land emerges in time"): still **0.362 / 0.401** — matches the spike.
- At `5133a94` (2026-07-28, "Present-truth peel: the beacon HAS been carved both ways"), the
  commit immediately before the lake/fill-depression physics fix: already **0.940 / 0.982** —
  matches HEAD.

So the drift landed somewhere in the **12 commits between `d9a24c4` and `5133a94`**
(`git log --oneline d9a24c4..5133a94 -- crates/vivarium-world/src/erosion.rs
crates/vivarium-world/src/gen.rs crates/vivarium-world/src/sea_level.rs
crates/vivarium-world/src/measure.rs`), which includes candidates that plausibly touch exactly
what this measurement depends on: `1757c09` "Craton nucleation-and-growth: land is now made by
something" (generator mechanism), `fefa45e` "The epoch count gets a derivation" (erosion epoch
count), `21e56a2`/`8d4c5cc` "Rock-mass ledger" (crustal/erosion mass coupling), `6127210`
"boundary contract becomes nameable" (checked — its own commit message states behavior is
unchanged, geometry-inferred contract just got a name; unlikely culprit but not proven innocent).
**I did not narrow further than this range** — the point is decisively made without it, and a
full bisection (11 more worktree+build+run cycles) is exactly the kind of "verify the underlying
content" step that should happen before anyone relies on a specific commit attribution, not
before this verdict.

### What this means for sealing

The isotonic-vs-power-law question I was asked to strengthen has a real, clean answer **on the
kernel the spike actually ran on**: isotonic ceiling 0.401 vs. power-law 0.362, a small gap
(~0.02–0.22 per cell, median small). That's inside my pre-registered 0.4–0.5 bracket and **does
earn** the broader "no pointwise coarse-only closure" claim — on that kernel, the power law was
already capturing nearly all of the achievable monotone structure, so generalizing from
"power-law fails" to "the whole monotone family fails" was correct in substance, even though
FE 3's stated reasoning ("R² bounds every monotone map") was still backwards as an argument.

**But that kernel is not current main.** The claim in `DECISIONS.decision-log.udon:1057-1067`
and `core/src/obs-coarse-only-closure-nogo.md` carries no git-hash scope — it reads as present-
tense law about the live kernel ("no pointwise coarse-only closure can explain more than ~36% of
the trunk's variance," stated flatly, Working Notes line 98 telling future agents not to
re-propose the family). On current main, the same construction measures a power-law ceiling of
**0.94**, not 0.36 — a coarse-only power-law recalibration would, if this holds up, explain the
overwhelming majority of trunk variance today. That is not "the claim needs tighter wording," 
that is **the empirical basis for the no-go no longer describing the kernel the repo runs**. I
have not investigated *why* it changed this much or whether PROBE 6/7's held-out deployability
conclusion (bias/noise gets worse under recalibration) also flipped — only that the R² ceiling
itself, the number the whole no-go is anchored on, moved by roughly 2.6x and is no longer
excluding much of anything.

## REVISED VERDICT: do-not-seal

Not "seal-with-scope-tightening" as my first pass concluded — that verdict assumed the archived
measurement was current, and it demonstrably is not. What would have to run first:

1. **Re-run PROBE 6/8 (or this probe) on current main** to get the real present-tense ceiling —
   my numbers above (0.94 power-law / 0.98 isotonic on the 12 RUN.txt footprints) are a strong
   signal but were built to answer a narrower question and haven't been independently checked
   the way the archived PROBE 6/7/8 numbers were cross-checked against each other in `RUN.txt`.
   At minimum, someone should re-run `monotone_ceiling_probe` (it's sitting in the tree now,
   compiles, ~a few minutes to run) and look at whether PROBE 7's held-out deployability
   conclusion also inverts.
2. **If the high ceiling holds up**, the no-go as currently stated is false on current main and
   needs to be either retracted, or re-derived from whatever *does* still hold (there may still
   be a real gap — a ceiling of 0.94 is not 1.0, and PROBE 7's "makes bias worse, not better"
   mechanism argument (FE 4, the rerouting/sub-grid-channel-position mechanism) doesn't
   obviously depend on the R² magnitude and might survive independently — but that has to be
   re-checked, not assumed).
3. **If Joseph or a future adjudicator can identify a reason today's numbers are the artifact**
   (e.g. something about how `monotone_ceiling_probe` differs in a way that matters, beyond the
   `mra`-vs-area-weighted-coarsen substitution already validated as harmless at `bd21400`) that
   would obviously change this — but the bd21400 control run is a fairly strong check against
   "my new probe is just wrong."

The isotonic-vs-power-law wording fix I proposed in the first pass (item 1 in the "Exact wording
I'd want changed" list, above) is **still correct and still worth making** whenever this entry
is re-sealed — it's a real, independent defect in the argument's structure, orthogonal to the
staleness finding. But it's no longer the main event.

### Exact wording — held pending re-measurement, not for use yet

I'm not proposing final sealing wording this round, per the addendum finding above: any wording
that keeps "$R^2 \le 0.36$" or "$\sim 36\%$" as a present-tense number would be re-encoding the
same staleness problem the moment it's re-sealed. Once someone re-runs the measurement on
current main, the right shape (assuming the ceiling really has moved) is probably: state the
new ceiling with the commit it was measured at, keep bd21400's 0.36 as *historical* (the number
the mechanism/rerouting argument, FE 4, was originally argued from), and decide fresh whether
FE 4's mechanism claim ("the trunk depends on sub-grid channel position, not per-cell coarse
state") still has empirical legs at the new ceiling or needs its own re-check.

Files touched this round (read-only investigation, one new file per the extended mandate):
`crates/vivarium-world/examples/monotone_ceiling_probe.rs` (new — not yet committed by me, left
in place per the "don't edit canon, wording comes back through the coordinator" instruction —
this is code/instrument, not canon, but flagging it explicitly since it's a new file in the live
tree). Worktrees used for the bisection (`bd21400`, `d9a24c4`, `5133a94`) were created and
removed via `git worktree add --detach` / `git worktree remove --force`, no changes left behind.
