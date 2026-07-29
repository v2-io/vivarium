# Red-team of the graduate verdicts — router/grid + closure/discretisation slices

*2026-07-29. Adversarial re-adjudication of the six graduate-verdict dirs. Method: read both first-pass reports, then read the spike directories directly (every file except PDF refs and build debris) and re-checked the cited absorbing surfaces line-by-line (`core/src/*`, `DECISIONS.decision-log.udon`, `ASSUMPTIONS.md`, `.super-archive/from-theory/discretisation-and-information.md`, live harnesses). Greps run against core+DECISIONS+ASSUMPTIONS+LEXICON for every candidate uncarried claim. Nothing edited or moved. Already-flagged residue items from the first pass are conceded and not re-verified.*

**Bottom line: four of six verdicts survive; two are refuted in part — `msc/spike-nonlocal-closure/` ("graduate-clean — every claim, number, and no-go verifiably landed") has at least one load-bearing uncarried measurement, and `msc/spike-principled-router/`'s land-X list is missing two real items and one arguable. Nothing found contradicts a first-pass *residue* item; every catch below is something the first pass called covered or did not mention.**

---

## 1. `msc/spike-curl-probe/` — **verdict survives**

Checked beyond the first pass's enumeration (it listed identity restatement, D4 null theorem, level-independence, quadrature trap, honest-negative shape — but not P4/P5):

- **P4, the discriminating control** (`PREDICTIONS.md:43-47` — feed a 15°·cos(2ψ) deflection, odd around the loop; probe must return κ≈0 or it is "merely re-running the fan probe under a new name"): **carried live** in the harness — `crates/vivarium-world/examples/curl_probe/main.rs:83` ("C3 rotate 15°·cos2ψ (odd around the loop)", pass gate |κ|<5e-3) and `curl.rs:244`. The instrument-validation design survives as executable code, which is better than prose carriage.
- **P5** (gradient-projected edge flux ≪ MFD-8): carried as `obs-routing-curl-spiral` FE(4) (κ=4.5e-3, 4.5×).
- The two live citations needing re-pointing are exactly as the first pass located them (`obs-routing-curl-spiral.md:29`, `:39`, breadcrumb `:43`). Confirmed.

Nothing else in the file. **Null result; graduate-after-repointing stands.**

## 2. `msc/spike-router-pricing/` — **verdict survives**

Diffed all six files against `DECISIONS.decision-log.udon:1039-1048` (council-accepted 2026-07-24):

- `RESULTS.md`'s full content is carried at near-verbatim fidelity: the three-part decomposition with the CUBE table's ranges, the P0 fail-then-pass story (stale-kernel max|Δh|≈150 m, RESULTS.md:42-44 ≡ DECISIONS:1044), M1/M2 demoted as poor discriminators, the strawman boundary (RESULTS.md:139-148 ≡ DECISIONS:1045), FE(8) discharge scoping (DECISIONS:1047), the `6c1ad97` moving-target framing.
- `NOTE-to-coordinator.md`'s interpretive call (strawman convicts consequence, not the principled remedy) is carried in the council note at DECISIONS:1039 verbatim in substance.
- `PREDICTIONS.md`'s wrong-in-an-informative-way prior (P1) is honestly reflected — the entry states the reading flipped when EdgeFull priced.
- Only gap found: RESULTS.md limit 3's forcing spec (**80 epochs, uniform uplift 0.02 m/epoch**) is not in the DECISIONS limits list (`One seed / face / level (L19) / synthetic dome` is). Not meat — the raw logs graduate with the dir and `examples/router_pricing.rs` is live and reprints it. Below the land-before-graduate bar.

**Null result; graduate-after-repointing stands.**

## 3. `msc/spike-corrected-scheme-cost/` — **verdict survives**

`stencil_bench.out` checked section-by-section against DECISIONS:759-777: §0 refuted conditioning hypothesis (1.7e-14 ≡ :776), §A oracle verification (2.7e-13 ≡ :765), §B f32 three-way split (3.9e-9 / 5.1e-9 / 3.06e-3 ≡ :771), §C kernel table incl. SoA/AoS 2.8× and break-even ~7 (≡ :765/:772), §D cliff N≈437 predicted-before-measured with the zero-coefficient control (≡ :767), §E frozen leak −2.85e-10 + corner-position table (≡ :771 "5e-9 at centre → 9e-5 near an edge"), §F +2.7%/0.24%/790×-is-misleading (≡ :766), and the closing residue (K6 two-pass not measured, "do not read +2.7% as the cost of fixing MFD" ≡ :773). Every number and both refuted hypotheses land. Trivial drifts only (precompute 6.8 ms in the bench vs "7.8 ms/tile" in :765; the 47× vs table-C's 43× — both consistent with §D's K0 rate). The "no segment owns it / census misses the two open items" question is the first pass's item 5, theirs to carry.

**Null result; graduate-after-repointing stands.**

## 4. `msc/spike-principled-router/` — **verdict "graduate-after-landing-X" survives in kind, but X is incomplete.** Two more real uncarried items and one arguable, all in DERIVATION.md sections the first pass never mentioned:

**R1 (real). The consistent flow-concentration substitute, and the fact that no post-process saves a `q ≠ 1` scheme.** `DERIVATION.md:219-234` ([P] Coatléven & Chauveau §5 verbatim): powers of the *directional* slope destroy consistency and *"the consistency of the flow routing algorithm will be lost again"* — i.e. **the (12)–(13) vector reconstruction (remedy-stack step (c)) cannot rescue any future `q ≠ 1` retune**; the consistent way to get flow concentration is `‖∇h‖^{p_w}` with `p_w = q−1`, a per-cell scalar on the full gradient (restated as build item 1, `DERIVATION.md:398`). Greps for `p_w` / "directional slope" / "lost again" across core, DECISIONS, ASSUMPTIONS, LEXICON: **zero carriage** (DECISIONS:498 uses `p_w` only in the GMS-subsumption line). Why it is load-bearing: `ASSUMPTIONS.md:37` pins `P=1.0` and `obs-routing-curl-spiral` FE(6)(a) records the theorem, but nothing records *how to reintroduce the concentration effect Freeman's 1.1 existed for* — the exact pressure under which a future engineer would re-raise `p` and (per the uncarried clause) silently void step (c). One sentence on FE(6) or the ASSUMPTIONS row.

**R2 (real). The router's modified equation — the mechanism behind every level-independence measurement core carries as bare fact.** `DERIVATION.md:337-371` (§4.2a): Δ(ψ) ≈ −0.332°·sin(8ψ) (wavenumber-8, 94.2% of variance), an **odd/advective, first-differential-order** spurious transverse advection — which is *why* drift integrates path-length-proportionally and never converges, while the (even, diffusive) spread converges O(√h): *"the project measured both halves of this modified equation and did not know which was which."* Grepped "advective", "sin(8", "wavenumber", "O(√h)": the only related carriage is the **water half** — `detail-nomos-defect-anatomy.md:44` (θ is even/Laplacian, "cannot be the odd/advective term a comment claimed"). The router half — the genuine odd term, and the two-kernels-had-opposite-pathologies-swapped observation (`DERIVATION.md:371`) — is **nowhere**. Core carries the measurements (obs-cube-locked FE(1) "refining worsens drift"; obs-routing-curl-spiral FE(2) "level-independent") and DECISIONS ③ carries the *second*-moment modified equation; the first-moment advective term that explains the flagship level-independence is only in the spike. Natural home: obs-routing-curl-spiral FE or `#norm-bias-vs-noise` discussion.

**R3 (arguable). The Prescott/Hyväluoma adjudication and its read-Prescott gate.** `DERIVATION.md:309-335` (§4.2b): the closed-form sweep confirms Prescott's sign *structure* but moves the zero-crossing to p=1.0 (Hyväluoma right, "Freeman overshot the null by 0.1 — and that overshoot is the entire 0.24°"), **plus** the honesty clause: Prescott was *not read primary this spike*; his 1.1 optimum may be a whole-DEM accumulated metric where dispersion enters — "read Prescott and check which metric his 1.1 optimises before acting." `ASSUMPTIONS.md:37` carries the theorem and the misread-baseline note but not the literature adjudication or the gate. Cheap: one clause on the ASSUMPTIONS row. (Without it, someone quoting Prescott's "1.1 is optimal" against the row has no recorded answer.)

**R4 (housekeeping, graduation-relevant).** `MEASUREMENTS.md:3` says *"Reasoning lives in `FINDINGS.md`"* — **no FINDINGS.md exists and never did** (both router agents died mid-write, DECISIONS:471; the reasoning survived as DERIVATION.md). Graduating freezes a dangling pointer into the archive; a one-line note in the graduation commit (or the `.super-archive` MANIFEST) that DERIVATION.md is the surviving reasoning file prevents a future re-miner from hunting for lost meat.

First pass's items 1–2 (Coatléven pits/flats limit; Thm 6.1/h^{1/2}/Def 4.2 + stale "[⊘ unread]") re-verified as genuinely uncarried — both confirmed by grep; they stand.

## 5. `msc/spike-nonlocal-closure/` — **"graduate-clean — every claim, number, and no-go verifiably landed" is REFUTED in part.** Draft A/B/C did land verbatim (verified: `NOTE-integration-drafts.md` §A ≡ `core/src/obs-coarse-only-closure-nogo.md` byte-for-byte in substance; §B ≡ DECISIONS:1050-1060; §C ≡ sketch FE(5)/(6)+WN). But "every load-bearing summary carried" is false on two counts the drafts themselves omitted:

**R5 (real — the catch of this red-team). The trunk oracle *fails at depth 1*, and core's carriage is over-broad without it.** `RUN.txt` PROBE 5 (lines 77-83): at **depth 1** the trunk (MAX) injection *over-corrects* to **−3.71 / −4.14 / −5.51 m** against baselines +3.54 / +4.21 / +2.43 — comparable-or-worse in magnitude, sign flipped — while at depth 2 it lands +0.34 / +1.32 / +0.99. The spike knew this was load-bearing: `PREDICTIONS.md:132-134` (P9) states Round 1's bracket as *"max was right at depth 2, **over-corrected at depth 1**."* Carriage check: every core surface states only the positive half — `obs-coarse-only-closure-nogo.md:27-28` and DECISIONS:1055 say "zeroes the bias at depth 2" (scope present, failure absent), and `sketch-detail-abstract-reversion.md:35` says the trunk is *"the one quantity the oracle **proved sufficient**"* — **unqualified, and contradicted by the spike's own depth-1 row**. Why it matters forward: FE(6)'s candidate closure (the minimal sub-grid channel summary carrying ≈max A_fine) inherits the depth-1 behaviour; a designer reading "proved sufficient" will build a depth-uniform summary the spike already measured over-correcting at depth 1. Fix: one clause in sketch WN ("proved sufficient *at depth 2; over-corrects at depth 1 — the summary must be depth-aware*, RUN.txt PROBE 5") or on the no-go segment's FE(1).

**R6 (arguable). The power-mean oracle is a measured-refuted candidate carried nowhere.** `RUN.txt` PROBE 3 line 51: injecting the **order-m power mean** — the natural "Jensen-corrected" restriction that FE(4)-style reasoning proposes — gives mean **−7.796 m**, *worse in magnitude than the +7.174 baseline*. Grep "power mean/power-mean" in core+DECISIONS: zero. DECISIONS:1056's "routing, not pointwise concavity, is the dominant mechanism" implies it but never states the refuted candidate, and the no-go segment's regression guard (:97-99) lists "scale, power-law, value-quantile" *recalibrations of A_coarse* — a different family (coarse-only maps), not the power-mean *restriction of fine A*. Regression-guard grade: it is precisely the next seductive wrong answer after reading the Jensen paragraph.

(Also re-confirmed the first pass's adjacent finding: `Fluvial::drainage_override`/`drainage_recalibrate` exist only in the spike's own `src/probes.rs`; `obs-coarse-only-closure-nogo.md:100` and DECISIONS:1059 present-tense them. Their tense-fix stands, and R5/R6 strengthen the case that RUN.txt is not "raw provenance only.")

Verdict impact: **graduate is still right** (the harness no longer compiles; the dir is a frozen record), but it is graduate-after-landing-R5 (and optionally R6), not graduate-clean.

## 6. `msc/redteam-discretisation/` — **verdict survives**

Independently traced all eight findings from `probe-output.txt` (not from the first pass's map):

- **A/A2/F** → superseded-by-stronger at `obs-mean-pin-manufactures-seam` (the 2026-07-24 re-based numbers, the (1,6,1)/8 stencil identified in FE(2), the guard restated to convict in Discussion, age-growth in FE(5)/(6)). Probe A's extra correlate (corr(pin residual, ∇² eroded surface) = −0.419, line 15) is uncarried but **moot**: the operator is retired-and-deleted; a correlate of a deleted operator's artifact convicts nothing now. Not meat.
- **B/G** → carried [M]-tagged at `.super-archive/from-theory/discretisation-and-information.md:348` (whole-face 1.4× vs within-parent 1.000008× at L19, 8.5e-6 m on 143 m relief, lifted-wavelet handles coarse) and superseded at design level by DECISIONS[unbalanced-haar…]. Verified at the cited line.
- **C** → null result; the curvature family lives at `detail-structure-scheme-map` FE(4). Fine.
- **D/E/H** → carried near-verbatim: E at DECISIONS:615 (78%-in-8-steps, the 2·sin(kΔ/2)/Δ symbol, already-staggered/no-null-mode); H at DECISIONS:629-637 (Fr envelope verbatim quote, 5.7% > 1.5, **saturated-clamp tell**, three one-sided clips) + `ASSUMPTIONS.md` rows; D's GS-vs-Jacobi at DECISIONS:635 — with the un-landed ASSUMPTIONS row being the first pass's own concession, stakes since lowered by DECISIONS:786 (identical ρ) and :784's roll-wave supersession of the whole soliton line.
- `redteam_probe.rs.bak`: instrument source only; no claims beyond the eight outputs.

**Null result; graduate-clean stands** (modulo their conceded ASSUMPTIONS row).

---

## Adjacent findings (not verdicts, same species as the first pass's FE(8) catch)

- **`obs-cube-locked-kernel-bias.md` Epistemic Status is stale against the rotation-test result.** It gates: *"Cone-only '4× better router' headlines remain gated by a rotation test (Prescott) **not yet claimed here**"* — but the rotation test **ran and passed** (§5b, every router ≤1.42% pk-pk; DECISIONS:480 ⑥, council-accepted 2026-07-24). Same integration-is-replacement species as the FE(8) three-layer contradiction the first pass found in the sibling segment; a one-clause fix ("gate discharged — DECISIONS ⑥; the error is a coherent field, not orientation-dependence") in the same landing.
- **Endorsement with verification:** the first pass's framing note — DECISIONS measured council-accepted entries functioning as claim-grade absorbing surface — held up under attack everywhere I pushed on it; every "carried in DECISIONS" claim I spot-checked (L468-485, L487-501, L759-777, L1039-1048, L615, L629-637) was accurate at the fidelity claimed, including the spikes' own residue clauses. The first pass's *enumeration* was the gap, not its evidence.
