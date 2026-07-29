# Water council probes — clip census, roll wave, identifiability (2026-07-29)

Resuming the order-of-work left standing by `DECISIONS[water-runs-outside-its-published-validity-envelope]` (council 2026-07-24). Step (1) — promote the hardcoded literals to `WaterParams` — landed 2026-07-24. Steps (2) *count the clip activation rates* and (3) *run the roll-wave probe* had not been run. A third, water-adjacent free probe (identifiability of a uniform conveyance) arrived from the router side as `#obs-routing-curl-spiral` Working Notes.

Instruments live in `crates/vivarium-world/examples/` (outside the source digest — building and running them cannot invalidate the world store). **No `src/` edits were made**; where a probe wanted one, it is written up in §2 as a proposal rather than landed.

New instruments, all re-runnable:

| path | what it does |
|---|---|
| `examples/water_clips/` | clip census — real-kernel fork-differencing plus the pinned transcription |
| `examples/roll_wave/` | growth-rate probe: slope ladder, θ ladder, grid refinement, real-kernel twin |
| `examples/identifiability/` | uniform-conveyance inertness, router and water sides |

Every table below is reproducible by running the named example; nothing here is quoted from a run whose instrument is not in the tree. All three probes are deterministic (no RNG anywhere), and `roll_wave` carries its own two-sided controls: a flat bed must not grow (measured ρ = 0.999995) and a CFL-violating `dt = 4 s` must blow up (measured ρ = 26.2).

One shared file touched: `examples/null_space/water_op.rs` gained `jarrett_slope` / `jarrett_n_cap` / `froude_cap` on `PipeParams` (mirroring the 07-24 kernel promotion) and a `positivity` / `cells` counter on `Guards`. Purely additive; every existing construction site uses `..PipeParams::kernel_default(l)` and `Guards::default()`, and `examples/null_space` still builds and runs unchanged.

---

## §0 Pre-registration (written before the first run) — and how it went

**P1 — Froude-cap incidence tracks the `Fr > 1.5` gauge.** Predicted cap incidence between 2% and 5.7% of wet cells. → **MISS, by more than an order of magnitude.** Measured **0.05%–0.21%**. The premise was also stale: see §1.0.

**P2 — clips concentrate on steep cells.** → **MISS, cleanly refuted.** Cap-firing cells are *not* steeper than the wet mean (⟨slope⟩ 0.056–0.076 against a wet mean of 0.075). They are **thin**: ⟨depth⟩ 0.28–0.93 m against a wet mean of 2.00 m.

**P3 — the positivity clamp is a negligible mass source.** → **HIT, and sharpened to zero.** It never fires at all on this workload.

**P4 — the roll-wave growth rate does not converge under refinement.** → **SPLIT, and the framing was wrong.** At the shipped roughness growth *is* grid-divergent (HIT); at low roughness it converges (MISS). But the discriminator I built the probe around turned out not to be the one that decides the question — a Froude-threshold test did, and I had not planned one. See §1.2.

**P5 — Jarrett, not θ, stabilises the shipped kernel.** → **HIT.** Reproduced independently, by a different method.

**P6 — a uniform conveyance is inert on the routed field, and inert because absent; and `manning_n` is inert on steady discharge.** → **SPLIT.** Second half **HIT to 1 part in 10⁴**. First half: the probe as derived turns out **not to be runnable** on our router, and my proxy for it was invalid — see §1.3.

---

## §1 Results

### §1.0 The number the whole water audit rests on is stale by 16×

`ASSUMPTIONS.md` (θ row and breaking-cap row) and `DECISIONS[water-runs-outside-its-published-validity-envelope]` both carry: *"MEASURED: ~5.7% of wet cells run Fr > 1.5."*

**The instrument that produced it now reports 0.35%.** This is not my reproduction disagreeing — I ran `examples/redteam_probe`'s probe H, unmodified, from the tree:

```
step    max Fr    % of wet cells supercritical (Fr > 1.5)
  50       1.90      0.02%
 100       2.00      0.14%
 200       2.00      0.29%
 400       2.00      0.35%
```

`git log -p` on `redteam_probe.rs` shows probe H's body has never been edited since it was authored. What changed is underneath it: `src/erosion.rs` and the prior moved on 2026-07-24 and after (MFD `p=1`, true great-circle neighbour distances, the fill/lake rework). The terrain the probe builds is not the terrain it built on 07-13.

**The qualitative claim survives; the magnitude does not.** We do run outside de Almeida & Bates' envelope — max Fr with the cap lifted for one step reaches **2.48** — but on 0.35% of wet cells, not 5.7%.

I did not attempt to attribute the drop to a specific landing, because doing so would need old `src/` checked out, which re-keys every cohort. Flagged as unattributed.

### §1.0b And the workload is neither "land" nor meaningfully "eroded"

Checked rather than described (`water_clips` §0), on the footprint every water Froude statistic has used:

```
derived sea level          : 5106.3 m
bed range                  : 3281.8 .. 3425.8 m   (relief 144.0 m)
cells at or below sea      : 9216/9216
cells moved by 60 epochs   : 8820/9216   max |Δh| = 1.0327 m
```

The canonical "60-epoch eroded land" footprint sits **1.7 km below the derived sea level in its entirety**, and 60 epochs move the bed by at most **1.03 m on 144 m of relief** (relief unchanged to two decimals). It is a submarine tile carrying a lightly-scratched fBm prior.

This does not invalidate the clip census — the census measures what the kernel does on the bed it is given. It does undercut the gloss attached to the 5.7% figure: *"those are the steepest, fastest cells, i.e. exactly the ones doing the erosive work."* There is essentially no erosive work happening on this footprint, and per `DECISIONS[water-world-is-the-promise-not-the-bug]` there should not be. **A supercritical-flow statistic that wants to say something about erosion needs a footprint with emerged land on it.**

### §1.1 The clip census

Workload L19, 96² at 19 m, the bed above, 2 m sheet, every non-hydrodynamic stage off.

**(A) Real kernel, by fork-differencing.** `WaterSim` is not `Clone` and its flux state is private, but `step` is deterministic — so a trajectory can be branched even though a state cannot. Run a fresh sim `k` steps, take one more step with a clip's parameter neutralised, and diff against the same trajectory with the clip on. The cells that differ are exactly the cells that clip touched at step `k+1`, on the real kernel, on its own real trajectory.

| clip / term neutralised | incidence, % of wet cells (step 1 → 399) | ⟨slope⟩ touched : wet | ⟨depth⟩ touched : wet |
|---|---|---|---|
| **Froude breaking cap** | 0.00 → **0.05 → 0.21%** | 0.056–0.076 : 0.075 | **0.28–0.93 : 2.00 m** |
| **Jarrett `n` ceiling** | **72.5 → 63.9%** | **0.094 : 0.075** | 1.33–2.00 : 2.00 |
| Jarrett term entire | 98.2 → 98.7% | 0.075 : 0.075 | — |
| θ flux smoothing | 99.9 → 100.0% | 0.075 : 0.075 | — |

The last two rows are the **dynamic-range control**: the same instrument registers ~100% for a term that touches everything, so 0.05% for the Froude cap is a real small number and not an instrument floor.

Three findings:

1. **The Froude cap almost never fires** — two to three orders of magnitude below what the `Fr > 1.5` gauge was being read to imply.
2. **It is a thin-film clamp, not a steep-ground clamp.** Cells it touches are at a *third to a half* of the mean depth and are not steeper than average. This is dimensionally obvious in hindsight — `Fr = v/√(gh)`, and the cap is on `h^{3/2}` — but it inverts the recorded belief, and it changes what the clip's bias means: a sign-definite operation concentrated on **thin water** biases sheet flow and films, not channels.
3. **The Jarrett ceiling is the dominant clip in the kernel, by three orders of magnitude over the Froude cap.** Roughly **two-thirds of wet cells** have at least one pipe pinned at `n = 0.13` every step, and those cells *are* steeper than average (+25%). Read against `DECISIONS[jarrett-roughness-is-a-positive-feedback-and-is-not-used-as-intended]` — which measured that the feedback switches off once `n` caps — this says: **on ~2/3 of the wet domain the Jarrett term is not a feedback at all, it is a constant roughness of 0.13.** The destabilising feedback lives on the remaining third, the gentle cells. That is the same gentle/steep split the 07-13 entry found dynamically, arriving independently from a static census.

**Also measured: `max Fr = 2.00` is algebra, not measurement.** A capped pipe reads `f/(h·l)/√(g·h) = froude_cap` *exactly*, by construction. The bit-identical 2.00 was never going to be anything else. With the cap lifted for exactly one step on the real trajectory, the true demand reaches **2.48**.

**(B) The clips with no parameter**, via the `null_space` transcription (which now counts the positivity clamp too), re-pinned on *this* workload:

| step | rectifier `.max(0)` | dry sill | breaking cap | outflow clamp | positivity clamp |
|---|---|---|---|---|---|
| 1 | **50.000%** | 0.000% | 0.000% | 0.000% | **0.000%** |
| 100 | 41.031% | 0.000% | 0.008% | 0.000% | **0.000%** |
| 400 | **39.232%** | 0.016% | 0.033% | 0.011% | **0.000%** |

(rectifier / dry-sill / breaking are % of pipes; the clamps are % of cells.)

Pin on this workload: **max |Δdepth| = 1.24e-4 m over 400 steps** against a 2.000 m mean depth. Worth noting *why* that is good rather than merely small: the bed sits at a ~3300 m datum where one f32 ULP is ~2.4e-4 m, so the transcription tracks the kernel to **below one representable f32 increment of the surface**. The pin is at the floor of what the kernel can distinguish.

Two findings:

4. **The positivity clamp — `DECISIONS[…validity-envelope]`'s *"a silent MASS SOURCE if it fires. Unprobed."* — does not fire.** Zero occurrences in 400 steps × 9216 cells. Independently corroborated on the real kernel: with every reservoir stage off, `budget_drift` over 400 steps is ±1e-4 m·cells (relative ~1e-13) **and changes sign** — f32 summation noise, not a one-sided mint. The outflow clamp does make it unreachable, as its design implies. This item can be closed.

5. **The rectifier fires on 39–50% of pipes — and that is structure, not pathology.** At step 1 it is exactly 50.000%, which is the tell: the face between `i` and `i+1` carries **two** pipes (`fr[i]` and `fl[i+1]`), and water crosses it one way, so one of every opposed pair is rectified by construction. The listing of the rectifier among *"three one-sided clips … bias by construction"* needs qualifying: as a *flux* representation the pair is complete and unbiased. **What it does lose is momentum memory** — a decelerating pipe's stored flux is zeroed rather than handed to its partner, so momentum is destroyed at every flow reversal. That is a genuine one-sided sink, it is not the same claim as flux bias, and it is untested. It is also exactly the shape of the bias the primary names (*"slower flood propagation speeds"*). Offered as a hypothesis with a named probe, not a finding.

### §1.2 The roll wave — the instability is real, and it is not Vedernikov's

**First, a confound found while building the probe, which is a finding in its own right.**

`water.rs` computes the friction denominator from the **pre-friction** velocity (`let v = accel / (hflow*l)` before the implicit divide). The consequence is that the steady normal-flow solution is a **function of Δt**:

```
70% slope, d₀ = 1 m, l = 4.8 m — only dt varies.
Manning's own answer is dt-free: v = 6.436 m/s ⇒ Fr = 2.056 (capped at 2.0).
  dt 0.800  →  Fr 1.3584   (−32.1%)
  dt 0.400  →  Fr 1.6638   (−16.8%)
  dt 0.200  →  Fr 1.8484   ( −7.6%)   ← the SHIPPED pairing
  dt 0.100  →  Fr 1.9492   ( −2.5%)
  dt 0.050  →  Fr 2.0000   (  0.0%)
```

**The shipped `dt = 0.2 s` at `l = 4.8 m` runs the steady state ~8% slow, one-sided**, and at the coarser steps `stable_dt` permits it degrades to −32%. This is not a CFL violation — the kernel's documented criterion `dt ≲ l/√(g·d)` gives 1.53 s here, and `stable_dt` returns 0.46 s clamped to 0.2 s, so 0.2 s is "safe" by every criterion the code states. **The binding constraint on this kernel is friction accuracy, not wave CFL, and nothing in the tree says so.** It also means any refinement ladder with `dt ∝ l` compares different base states — which invalidated my own first version of the refinement test.

**Now the main result. With `dt = 0.02 s` (converged), `n` held constant, and the breaking cap OFF — so the base state is honest unclamped Manning normal flow — the kernel is stable at every Froude number tested, and gets *more* stable as Fr rises:**

```
slope    Fr (base)      ρ          σ (1/s)    verdict
   5%      0.548     0.999970    −0.00148    stable
  20%      1.093     0.999939    −0.00306    stable
  40%      1.542     0.999873    −0.00636    stable     ← Manning Vedernikov critical
  70%      2.034     0.999719    −0.01407    stable     ← Chézy Vedernikov critical
 100%      2.426     0.999486    −0.02573    stable
```

Vedernikov predicts onset at Fr = 1.5 for a wide Manning channel. The kernel passes straight through 1.5 and 2.0 with **monotonically increasing damping**.

**I nearly stopped there and reported a clean refutation. That would have been wrong**, because the 07-13 Brillouin map was taken at **n = 0.04, θ = 1** and I had swept n = 0.13 — a neighbouring base state, not theirs. Refuting a claim on a base state its author did not use is not refuting it. Re-run on theirs (`roll_wave` §8):

```
n = 0.04 constant, θ = 1, no cap, dt = 0.02 s, l = 4.8 m
slope    Fr (base)     ρ         σ (1/s)   λ (cells)   verdict
   2%      1.129    1.000420   +0.02101      9.14      GROWS
   5%      1.784    1.001180   +0.05895      9.14      GROWS
  10%      2.522    1.002628   +0.13121      9.14      GROWS
  40%      5.038    1.008946   +0.44531      9.14      GROWS
  70%      6.660    1.012920   +0.64188      9.14      GROWS

…and under refinement at 5% slope (Fr 1.78, domain fixed at 307.2 m):
  l (m)    nx      σ (1/s)   λ (cells)    λ (m)
  19.20    16     +0.02607     8.00       153.6
   9.60    32     +0.01043     8.00        76.8
   4.80    64     +0.05895     9.14        43.9
   2.40   128     +0.03370     9.14        21.9
   1.20   256     +0.04147     8.83        10.6
```

**So there is a real growing mode, and its growth rate per second does *not* diverge under a 16× refinement** (σ scatters between 0.010 and 0.059 with no trend). By the discriminator this probe was built around, that is the signature of a *physical* instability, not a numerical one. The grid-locked wavelength (8–9 cells at every Δx, λ in metres tracking Δx down) is **not** evidence against it — the probe's own header registered that in advance: Vedernikov growth increases monotonically with wavenumber, so the discrete peak landing at the grid scale is exactly what a real roll wave would also do.

**But the onset is wrong, and that is the decisive test.** Growth is already present at **Fr = 1.129**, below the Manning Vedernikov critical of 1.5 — and it is present at every slope down to the lowest tested. The Vedernikov criterion is a statement about the Froude number *and nothing else*: `Ve = (β−1)·Fr`, unstable above 1, giving Fr = 1.5 for Manning. An instability that grows at Fr 1.13 is not it.

**§5 and §8 differ in θ as well as `n` (0.8 against 1.0), so neither can be attributed against the other.** The crux control holds θ = 1.0 in both columns, cap off, Jarrett off, so that `n` is the only difference — and a Froude criterion, by construction, cannot depend on `n`:

```
θ = 1.0 both columns, no cap, no Jarrett, dt = 0.02 s, l = 4.8 m, 64²
slope          n = 0.04                    n = 0.13
             Fr        ρ                 Fr        ρ
  0.5%     0.563   1.000156  GROWS     0.174   1.000067  GROWS
  1.0%     0.798   1.000218  GROWS     0.245   1.000056  GROWS
  2.0%     1.129   1.000420  GROWS     0.347   1.000070  GROWS
  5.0%     1.784   1.001180  GROWS     0.548   1.000208  GROWS
 10.0%     2.522   1.002628  GROWS     0.774   1.000547  GROWS
 20.0%     3.565   1.005226  GROWS     1.093   1.001186  GROWS
 40.0%     5.038   1.008946  GROWS     1.542   1.001991  GROWS
 70.0%     6.660   1.012920  GROWS     2.034   1.002887  GROWS
```

**Every configuration grows — including Fr = 0.174.** Deeply subcritical, where no roll-wave criterion of any kind permits an instability. And at matched Froude the two columns disagree by 3× (Fr 1.129 → ρ 1.000420 against Fr 1.093 → ρ 1.001186), so growth is not a function of Fr at all.

**This settles it, and it settles it in a more interesting direction than "the roll waves are real" or "the roll waves are fake."**

Compare that table against §5: same `n = 0.13`, same cap-off, same `dt` — **and stable at every slope through Fr 2.43.** The only difference is θ (0.8 against 1.0). So:

> **Without θ-smoothing the scheme is unstable at every Froude number tested, from 0.17 to 6.66. θ = 0.8 is what makes it stable.**

θ is therefore not "damping a mode that is at least partly REAL." It is the stabiliser of a discretisation that is otherwise unstable **in still-subcritical flow, where there is no physical instability to damp**. That is the textbook signature of artificial viscosity holding together an unstable scheme — which is precisely what a Lax–Friedrichs-class term *is*, and precisely what de Almeida et al. call it in their own words (*"a weighting factor that adjusts the amount of artificial numerical diffusion"*).

**So `DECISIONS[theta-is-lax-friedrichs-not-rhie-chow]` is more right than it knew, and the qualifier the council added to it is the part that fails.** The council's acceptance note reads: *"θ is claim-free AS A TERM … but the instability it damps is partly REAL (roll waves), so θ may be suppressing physics."* Measured: in the linear regime the instability it damps is present at Fr 0.17, so it is not roll waves and θ is not suppressing physics. The warning *"DO NOT simply delete θ as unphysical"* stands — and now stands for a **better** reason: not because it is holding back real roll waves, but because without it the scheme does not stand up at all.

**Two further growth behaviours, distinct from each other and from the above.** (i) At `n = 0.13` with θ = 0.8 the scheme is stable at l ≥ 2.4 m but grows at fine Δx and **diverges** there (§6, below). (ii) The Jarrett artefact (§7) is violently grid-divergent and fully isolated by its own control. Neither is Fr-governed either. Three growth behaviours, one shared feature: **none of them has a Froude threshold.**

A hypothesis, marked as such and **not** measured: the pipe carries flux `f` as state between steps, relaxed by an implicit friction of rate ∝ `n²`. The `(f, h)` pair is a two-variable map with a memory term and no advective coupling, and the θ-filter is the only thing dissipating the flux field itself. That would make the base instability a property of the *flux-memory discretisation* — absent from the continuum equations entirely, which is exactly why it has no Froude threshold. The clean test is a **modified-equation analysis of the two-variable step**, which `DECISIONS[structure-preserving-is-a-rediscovery-adopt-the-field]` already names as the right tool (*"what PDE is this scheme actually solving exactly"*) and which nobody has run on `water.rs`. That is the instrument I would build next.

**One consolation, and it is a real one.** At the shipped configuration the kernel *is* stable over the range that matters: `n = 0.13` is where the clip census (§1.1) shows **two-thirds of wet cells actually sit**, because the Jarrett ceiling pins them there, and at that roughness with θ = 0.8 the scheme is stable through Fr 2.43 at the operating grid. The shipped stack works. What is wrong is the *account* of why it works, in three places at once: the credit for stability belongs to θ and the Jarrett ceiling, not to θ-versus-roll-waves; the cap does no linear work at all; and the thing being damped was never physics.

**Separately, the growth at the *shipped* roughness IS grid-divergent.** Same configuration, domain fixed at 307.2 m, `dt` fixed at 0.02 s, Δx the only variable:

```
slope 70%                            slope 100%
  l (m)   nx      σ (1/s)              l (m)   nx      σ (1/s)
  19.20   16     −0.04493              19.20   16     −0.05078
   9.60   32     −0.07979               9.60   32     −0.08011
   4.80   64     −0.01407               4.80   64     −0.02573
   2.40  128     +0.00120               2.40  128     +0.01751
   1.20  256     +0.19443               1.20  256     +0.50887
```

Growth appears only at fine Δx and then rises steeply (×29 for the last halving at 100%). A physical instability's growth rate per second converges under refinement. This does the opposite.

**The Jarrett artefact is the same shape and far more violent** (5% slope, cap on, `dt = 0.02 s`, control holds `n` at Jarrett's own base value):

```
  l (m)     nx     ρ SHIPPED     σ SHIPPED      ρ n-CONST     λ (cells)
   4.80     64     0.999917      −0.00417       0.999970          9.14
   2.40    128     0.999999      −0.00003       0.999999          9.14
   1.20    256     1.000557      +0.02787       0.999999          2.17
   0.60    512     1.031601      +1.55559       1.000000          2.17
```

**The control is flat at 1.000000 on every grid.** So the entire instability is the Jarrett feedback, its growth rate rises by ×56 for a single halving of Δx (roughly σ ∝ Δx⁻⁵·⁸), and its mode collapses onto the **grid scale** (2.17 cells). This is stronger than *"a numerical artefact, and ITS physical claim IS none"*: the term is **inconsistent** — refining the mesh makes it worse without bound. For a project whose architecture is multiresolution (`#form-fidelity-ladder`, the coarse↔fine seam, the wavelet store), a term whose instability grows as you refine is a structural hazard, not a nuisance.

**What survives, unchanged.** The gentle/steep separation (`§1` slope ladder at the shipped dt reproduces the 07-13 table's structure exactly: 2% and 5% vanish when `n` is held constant, 20%–70% are identical with `n` constant), and **P5 — it is the Jarrett term, not θ, that stabilises the shipped kernel**. Also confirmed and sharpened: **θ damps and relocates, never removes.** At 70% slope, ρ falls monotonically 1.0331 (θ=1) → 1.00637 (θ=0.3), still unstable at θ=0.3, and the peak wavelength visibly *moves* — 9.14 cells at θ ≥ 0.7, jumping to 16.00 cells at θ ≤ 0.5. The 07-13 entry inferred the relocation from "a low-pass filter has no grip on a long wave"; here it is directly observed.

**New: the breaking cap is invisible to linear stability.** In the shipped slope ladder the `no-cap` column is *identical to shipped in every row*. The cap does no stabilising work at infinitesimal amplitude — it is purely a **finite-amplitude** device. Which is to say the cap is standing in for the missing advective term's saturation role: an unphysical term substituted for a dropped physical one, box ⑤ of `#detail-nomos-defect-anatomy` exactly.

**Confirmed on the real kernel**, not only the transcription (`roll_wave` §4, twin experiment on a rain-fed tilted channel, `dt` fixed at 0.02 s): σ = −0.0208, −0.0251, **+0.0457** for l = 19.2, 9.6, 4.8 m. Same shape — damped coarse, growing fine.

**Limits of this result, stated plainly.** This is *linear* stability of a *uniform* base state on a *periodic* patch. Roll waves are also a finite-amplitude phenomenon, and Joseph's original observation was multi-metre travelling blobs on real terrain at finite amplitude. Nothing here measures that. What is refuted is specifically the *linear* claim — that the scheme resolves a Vedernikov instability — which is the claim the 07-13 Brillouin map made and the one the council accepted. A finite-amplitude probe on real terrain is the honest next instrument and is not built.

### §1.3 Identifiability

**§1.3a The router half is a no-go, and my proxy for it was invalid.**

The free probe as derived asks: perturb a uniform Manning-type conveyance `k_m` and assert the routed field is bit-identical. **`accumulate_drainage` has no conveyance term** — weights are `(drop/dist)^P`, normalised. So the literature's inertness claim is true of our router *vacuously*: there is nothing there to be inert. The probe cannot be run as specified.

I built a proxy — apply a uniform ×2 gain to the driving field, which the normalisation must cancel bit-exactly — and it came back **not** bit-identical, max |Δ| = 7.6e5 m². I chased two attributions and the first was wrong:

- **`fill_depressions`'s `const EPS: f32 = 1e-3` (an absolute metre).** Plausible, source-verified as present — and **refuted**: 0 of 9216 cells are in the filled region on this terrain, and 0 of the 6533 cells whose drainage moved are inside the fill.
- **The outlet set.** `drainage_surface` takes base level from `self.outlets()` against `derived_sea_level_m` = 5106.3 m — an **absolute datum**. Measured: at ×1, **9216/9216** cells are at or below sea; at ×2, **0/9216**; **8836/9216** receiver-tree entries change.

So the ×2 arm flipped the tile from entirely submarine to entirely emerged. **It was never a conveyance perturbation** — it moved the boundary condition. The honest verdict is a no-go on both halves: the probe has no referent in our router, and rescaling topography is not a substitute for rescaling conveyance, because the router's base level is an absolute datum. I recommend the Working Notes item be **rewritten**, not just marked run — see §2.

**§1.3b The water half runs, and the theory lands to 1 part in 10⁴.**

`manning_n` *is* a conveyance coefficient. Rain-fed tilted channel, sea-held outlet, run to hydrological steady state, `dt = 0.02 s` (per §1.2 — a steady-state scaling probe cannot be run on a dt-contaminated steady state), `n` doubled 0.04 → 0.08 with the Jarrett feedback OFF:

| | n = 0.04 | n = 0.08 | ratio | Manning predicts |
|---|---|---|---|---|
| depth | 0.13388 | 0.20365 | **1.5212** | 1.5157 = 2^(3/5) |
| speed | 1.10856 | 0.72862 | **0.6573** | 0.6598 = 2^(−3/5) |
| **q** | 0.15602 | 0.15600 | **0.9999** | **1.0000** |

**At hydrological steady state, discharge is inert in roughness to 1 part in 10⁴, and depth and velocity absorb all of it, at the exponents Manning predicts, to 0.4%.**

That is the identifiability statement, and it is sharper than the literature's: **`manning_n` is unidentifiable from any discharge-derived quantity** — which includes the drainage area `A` that the erosion tier consumes. Roughness is invisible to the routed field.

**But not to the carve.** The sediment capacity is `C = k·|v|·slope` — velocity, not discharge. Doubling `n` scales capacity by **0.657**. So roughness is invisible to routing and fully visible to incision, and those two consume "the same" water.

**And the Jarrett arm behaves as the contrast argument predicts.** Same doubling with the feedback on: q still invariant (1.0001 — mass conservation is not negotiable), but the depth and velocity responses are *damped* off their uniform values (1.2574 against 1.5157; 0.7953 against 0.6598), because `n = min(n_base + 1.6·S, 0.13)` is not a uniform coefficient. Also worth recording: **the Jarrett term halves the Froude number at fixed forcing** (max Fr 1.033 → 0.517). That is `DECISIONS[…jarrett…]`'s *"the credit was on the wrong term"* arriving from a third, unrelated direction.

One correction to my own framing: Coatléven's *"only the CONTRASTS of `k_m` impact `q_w`"* does not transfer intact to a mass-conserving steady state. Here `q` is pinned by rain × contributing area and is inert in roughness **uniform or contrast**. The identifiable content of conveyance lives entirely in the depth/velocity split.

---

## §2 Proposals, not landed

Drafts only. I did not touch `core/`, `ASSUMPTIONS.md`, `OUTLINE.md` or `disc-open-problem-census.md`.

**(a) `ASSUMPTIONS.md` — the θ row and the breaking-cap row.** `MEASURED: ~5.7% of wet cells run Fr > 1.5` is stale by 16× and should be **replaced** (not annotated — `#scope-segment-canon`, integration is replacement) with the current measurement. Suggested replacement text for the measured clause:

> **MEASURED 2026-07-29** on the L19/96² probe footprint: **0.35%** of wet cells run Fr > 1.5, and the breaking cap fires on **0.05–0.21%** of wet cells — cells at a third to a half of mean depth, **not** steeper than average. `max Fr = 2.00` is the cap by algebra, not a measurement; with the cap lifted for one step the true demand reaches **2.48**. *(The earlier 5.7% figure was measured 2026-07-13; the same unmodified instrument now reports 0.35% — the terrain moved under the 07-24 erosion/router landings. Do not quote 5.7%.)*

The gloss *"those are the steepest, fastest cells, i.e. exactly the ones doing the erosive work"* should go: it is refuted twice over (the cap fires on thin water, and the footprint is submarine and essentially uncarved).

**(b) A new `ASSUMPTIONS.md` row — the friction discretisation's Δt dependence.** Not currently declared anywhere:

> **friction discretisation (`water.rs` `pipe_step`)** | semi-implicit Manning applied with the **pre-friction** velocity | ⚠ **arbitrary — and MEASURED to make the steady normal flow a function of Δt.** At 70% slope, d₀ 1 m: Fr = 1.358 at dt 0.8 s, 1.848 at the **shipped** dt 0.2 s, 2.000 (converged) at dt ≤ 0.05 s. **The shipped pairing runs the steady state ~8% slow, one-sided.** This is *not* a CFL violation — the documented `dt ≲ l/√(g·d)` gives 1.53 s and `stable_dt` returns 0.2 s. **The binding accuracy constraint on this kernel is friction, not wave CFL.** `examples/roll_wave` §2b.

This interacts with `#obs-water-fill-never-settles`, which found the 0.2 s ceiling binding at kilometre cells for the opposite reason (too *small*). Both are true: 0.2 s is far below CFL and still too large for friction accuracy. Those two facts belong in the same place.

**(c) `DECISIONS` entries.** I have not appended anything. Three are owed, all `:by claude :status proposed`:
1. `the-breaking-cap-is-a-thin-film-clamp-and-fires-on-a-fifth-of-a-percent` — the clip census.
2. `the-growing-mode-has-no-froude-threshold-so-it-is-not-vedernikov` — the §9 result, superseding the roll-wave *identification* in `our-kernels-have-no-null-space-the-solitons-were-roll-waves` and the council's *"the instability it damps is partly REAL"* qualifier on `theta-is-lax-friedrichs-not-rhie-chow`. **This one supersedes two council-accepted entries and I would rather it went to the council than be written by me alone.** Note what it does *not* touch: the null-space result, the staggering asset, and the gentle/steep Jarrett separation in those entries all stand — it is the physical identification of the mode that fails.
3. `jarrett-is-inconsistent-not-merely-artefactual` — the grid-divergence result.
4. `the-shipped-timestep-is-friction-bound-not-cfl-bound` — §2b, if it does not simply ride as the ASSUMPTIONS row in (b).

**(d) A `src/` change I did not make.** The rectifier, dry-sill, outflow and positivity clips cannot be counted in the real kernel from outside, because they have no parameter to neutralise. The clean fix is one accessor in the shape the kernel already uses for `last_froude`:

```rust
/// Clip activation counts from INSIDE the last step — the same idiom as
/// `last_froude`, and for the same reason: recomputing them from outside
/// reads a different state than the clip operated on.
pub fn clips(&self) -> Clips { self.last_clips }
```

Roughly 10 lines. It would let the census run on the real kernel instead of the pinned transcription, and it would make the clip rates a live instrument rather than a probe-time reconstruction. **It re-keys every cohort**, so it is Joseph's call on timing, not mine.

**(e) `#obs-routing-curl-spiral` Working Notes — the free identifiability probe.** The item as written is not runnable (§1.3a) and should be **rewritten**, not ticked off. Suggested:

> **Identifiability, run 2026-07-29 (`examples/identifiability`) — no-go on the router half.** Our router has **no conveyance term** (`accumulate_drainage` weights are normalised `(drop/dist)^P`), so the "uniform `k_m` is inert" claim has no referent in it and cannot convict. The natural proxy — a uniform gain on the driving field — is **not** a proxy: `drainage_surface` takes base level from `derived_sea_level_m`, an *absolute* datum, so rescaling relief moves the boundary condition (measured: ×2 flips 9216/9216 submarine cells to 0/9216 and changes 8836/9216 receiver entries). **The live instance of the claim is `manning_n` in `water.rs`**, where it holds sharply: steady-state discharge is inert in roughness to 1e−4 while depth and velocity carry it at Manning's exponents.

---

## §3 Adjacent things that look wrong

1. **`water.rs`'s own module comment asserts the refuted identification.** Lines ~402–411: *"What it is suppressing is partly REAL: the blobs below are ROLL WAVES (Vedernikov; real above Fr≈1.5) … The principled retirement is therefore NOT a tuned θ — it is an entropy-stable / shock-capturing momentum stage."* §1.2 measures growth at Fr 0.174 with θ off, so what θ suppresses is not roll waves and the prescription that follows from that reading does not follow. The *conclusion* — do not simply delete θ — survives on stronger grounds. That comment is the most load-bearing prose in the kernel and it currently teaches the wrong mechanism to the next reader; correcting it is cheap and high-value.

2. **The same file carries `DECISIONS[...]` references broken across line wraps** (e.g. `DECISIONS[jarrett-roughness-is-a-positive-\n feedback…]` at the `jarrett_slope` doc comment) — the exact pattern `bin/check-decision-refs` flagged in my example and made me fix. Either the checker does not scan `src/`, or it does and those are latent. Worth a look; I did not touch `src/`.

3. **`stable_dt` reads as a safety discipline and is a constant function** — `#obs-water-fill-never-settles` already says this at tile scale. §1.2 adds that even where it *does* vary it is calibrated to the wrong constraint (wave CFL, when friction binds ~10× tighter). The helper is doing less than its name promises in both regimes.

4. **Every water Froude probe in the tree runs on a submarine footprint** (§1.0b). `redteam_probe` probe H's own guard checks that the tile is *wet*, which it trivially is — it does not check that it is *land*. A probe footprint with emerged land would need `emerged-land` kept, which is the queue head, so this may simply be blocked; but the description *"60-epoch eroded land"* should not be repeated meanwhile.

5. **`WaterSim` not being `Clone`** is what forced the fork-differencing design. That design turned out to be *better* than cloning (it measures the real trajectory), so this is not a complaint — but any future state-space probe will hit the same wall, and a `Clone` derive is free.

---

## §4 Feedback on the brief

Three things that helped, one that would have helped more.

**The beliefs-vs-measurements split was load-bearing and it worked exactly as intended.** Being told "it is *measured* that ~5.7%… it is *believed* that clip rates concentrate on the steep erosive cells" is what made me reproduce probe H unmodified instead of just building on the number — and that reproduction is how the 16×-stale figure surfaced. Had the brief said "5.7% of wet cells run supercritical" flat, I would have anchored on it and reported clip rates as a *fraction* of it. The register separation caught a real error. Worth keeping as standard.

**Naming the prior red-team finding as "may confirm, sharpen, or refute, and any of the three is valuable" mattered at a specific moment.** When §5 came back stable at every Froude number, my first instinct was that I had broken the probe. The brief having pre-legitimised refutation is part of why I went looking for the confound (§2b, the dt-dependent base state) and then re-ran on *their* base state (§8, n=0.04) rather than quietly reporting the arm that agreed.

**The `examples/` constraint was given with its reason, and the reason changed my design.** "Example code is outside the source digest" is exactly the kind of thing I would not have known and would have guessed wrong about. It also directly produced the fork-differencing method: not being able to edit `src/` is what made me look for a way to count clips from outside, and that method is more honest than a counter would have been, because it measures effect on the real trajectory rather than incidence in a reimplementation.

**What would have helped:** the brief framed the three probes as three probes. They turned out to be one — the dt-dependence of the base state (found while building probe 2) invalidated my first refinement ladder, and then propagated into probe 3, where a steady-state scaling law cannot be measured on a dt-contaminated steady state. I nearly reported probe 3 on the shipped `dt = 0.2 s` before noticing. Not a defect in the brief — nobody knew — but if a future brief bundles probes that share a base state, saying *"these share a base state; a defect in it is a defect in all three"* would put the reader on watch for it.

One small correction to a premise in the brief, offered in the spirit of the beliefs/measurements split: *"the shipped kernel resolves a real roll-wave instability it cannot saturate"* is presented as the standing understanding, and it is what the council accepted — but it was already an **inference from a linearisation at one resolution**, not a measurement of the instability's physical character. The refinement test was the missing half, and it was cheap. If there is a general lesson it is that *"[M] measured"* tags on a spectral radius mean the number was measured, not that its physical identification was.

---

*Standing by for follow-ups.*

---

## §5 Kernel truth wave (2026-07-29, later) — pre-registration

Landed under Joseph's standing re-key grant. Written before the first run.

**P7 — implicit friction removes the Δt dependence.** The defect (§2b) is that the Manning denominator is formed from the *pre-friction* velocity, so the steady state depends on Δt. Solving implicitly in the *updated* flux makes the steady state Δt-free **by algebra**: with `f = A/(1 + k·f)`, `k = dt·g·n²/(h^{7/3}·l)`, `A = f + dt·g·h·head`, steady state gives `k·f² = dt·g·h·head`, and `dt` cancels on both sides leaving `f = l·h^{5/3}·√S/n` — Manning exactly. So I predict `roll_wave` §2b reports **Fr = 2.000 at every dt from 0.8 s down**, not only at ≤0.05 s. This one is a derivation, not a guess; if it fails, the implementation is wrong, not the theory.

**P8 — the Jarrett demotion.** Three consequences pre-registered as the coordinator named them: the Jarrett-ceiling clip row **collapses** (from 64–73% of wet cells toward 0); gentle-slope **ρ → 1** (the 2% and 5% growth vanishes); and the **grid-divergence flattens** (σ at l=0.6 m falls from +1.556 toward the control's 1.000000).

**P9 — my own, and it is the one that could stop the demotion.** The 07-13 record says the Jarrett term is what stabilises the shipped kernel, by dropping Fr 2.49 → 0.75. That stabilisation came from `n` **rising on steep ground**, which is Jarrett's actual physical content and is worth keeping. So I am demoting the term to a **static, bed-slope-derived** roughness — `n = min(n_base + jarrett_slope·S_bed, cap)` computed from the **bed drop across the pipe**, never from the instantaneous free surface. That is Jarrett 1984 used *as intended* (a regression estimating `n` from a measured channel slope) rather than as a live constitutive law, and the bed is quasi-static within the fast band by construction, so no loop can close through it.

I predict the stabilisation **survives**, because the mechanism (rough steep reaches) is untouched and only the feedback path is cut. **If it does not survive, that is the finding and the demotion does not land** — I will report it rather than force it. The discriminating measurement is the steep end of the `roll_wave` §1 slope ladder plus the Froude numbers on the `water_clips` workload.

## §6 Kernel truth wave — results

### §6.1 Implicit friction (landed, `77b1f5a`)

**P7 confirmed.** `roll_wave` §2b, before → after:

| dt (s) | Fr before | Fr after |
|---|---|---|
| 0.800 | 1.3584 | 1.9166 |
| 0.400 | 1.6638 | **2.0000** |
| **0.200 (shipped)** | **1.8484** | **2.0000** |
| 0.100 | 1.9492 | **2.0000** |
| ≤0.050 | 2.0000 | **2.0000** |

Flat from dt 0.4 s down; the shipped pairing now returns the converged answer exactly instead of running ~8% slow. The residual at dt 0.8 s is **not** friction — at that step a cell ships `f·dt = 30.9×0.8 = 24.7 m³` against a capacity of `d·area = 23.04 m³`, so the **outflow clamp** binds, which is a different and legitimate mechanism. §2b now prints the clamp rate so that attribution is measured rather than arithmetic.

### §6.2 The Jarrett demotion — one prediction confirmed, one refuted

`n` is now computed from the **bed** drop across the pipe, not the instantaneous free surface. That is Jarrett 1984 used as intended (a regression estimating `n` from a *measured channel slope*), and the bed is quasi-static within the fast band, so no loop can close through it.

**P8b confirmed, completely.** The gentle-slope growth is gone, and more than that — **`ρ SHIPPED` now equals `ρ n-CONSTANT` at every slope on the ladder**, i.e. the roughness term contributes no instability anywhere:

```
slope   Fr     ρ SHIPPED   ρ n-CONSTANT      (before the demotion: 1.00003 / 1.01090 at 2% / 5%)
   2%  0.63     1.00000       1.00000
   5%  0.60     1.00000       1.00000
  20%  1.10     1.00042       1.00042
  40%  1.55     1.00948       1.00948
  70%  2.00     1.00000       1.00000
```

**P9 confirmed — the stabilisation survives, and strengthens.** This was the one that could have stopped the demotion. At 70% slope the kernel is stable (ρ 1.00000) at Fr 2.00, and on the `water_clips` workload the supercritical fraction *fell* from 0.53% to 0.00–0.07%, with max Fr mostly 1.28–1.75 instead of a pinned 2.00. Keeping `n` rising with slope is what carried the stabilisation, exactly as the 07-13 record said — the feedback path was never the part doing that work. The two measurements the coordinator asked me to hold in both hands turn out not to be in tension at all: **`n` rising on steep ground is the stabiliser; `n` responding to the water is the defect.** They were bundled in one expression, and separating them keeps the first and deletes the second.

**P8a refuted, and instructively.** I predicted the Jarrett-ceiling clip row would collapse. It did not — it is **72.6% → 72.1%** (before: 72.5% → 63.9%). The prediction was category-confused: the ceiling binding measures *how much of the domain is steep enough to saturate the linearisation*, which is a **terrain statistic**, not a dynamics statistic. Demoting to bed slope does not make the terrain gentler. What did change is the *shape* of the row: it is now nearly constant in time (72.6 → 72.1) where before it drifted (72.5 → 63.9), because a static roughness field cannot follow the water. **The drift was the feedback, visible in the census all along, and neither I nor the census noticed it until the drift stopped.**

### §6.3 Verdict on the transcription fork (coordinator's prediction)

**Half right, and the half that is right should be mechanised today.**

Right about the tax, and I paid it twice in one hour: I first changed only `water.rs` and would have "verified" the friction fix against a §2b that runs on the *transcription* — unchanged code — had I not caught it; and yesterday the clip census had to establish pin-agreement to 1-in-36,480 before its numbers counted for anything.

Wrong, I think, that its reasons to exist are gone. Its module doc gives three, and they have not aged equally:

1. **θ hardcoded** — dead since 2026-07-24. Delete from the doc.
2. **Flux state private, so it cannot be perturbed** — **still live, and load-bearing.** `clips()` gives counts and `Clone` gives *copying*, but the `roll_wave` power iteration perturbs the full 5-DOF state `(d, fl, fr, ft, fb)` and renormalises the flux components every step. That needs *write* access to the flux arrays, which neither accessor provides.
3. **f64** — **still live.** The pin sits at 1.3e-4 m, about one f32 ULP of the bed datum; a power iteration seeded at ε = 1e-9 is not expressible in f32 at all.

So the growth-rate work genuinely needs a writable, f64 copy of the kernel, and neither of my accessors touches that. The coordinator's deeper cure (a pinned f64 *mode* of the real kernel) would mean making `WaterSim` generic over the float type plus a test-only flux accessor — real work, not a cleanup.

**But the yoke is cheap right now and I recommend it: promote `water_op::pin_against_kernel` from a function the probes call into a `#[test]`.** It is already written, already parameterised, and already returns the divergence. As a test it turns "an agent discovers the fork drifted, mid-probe, if they are lucky" into "the build fails." Today's near-miss is precisely the failure mode it closes. I have not landed it because it belongs in the `null_space` example's own slice rather than mid-friction, and because a test that shells into an example needs the pin moved into the crate — a ten-minute job, not a two-minute one. Named here so it is owed rather than forgotten.

### §6.4 Bearing on the momentum-closure deferral

Joseph's new declared-lesser-law row (dropped advective term) has a due-date question, and these measurements bear on it in a direction that **relaxes** it slightly. The advective term's absence was being invoked to explain the roll-wave instability the kernel supposedly resolved-but-could-not-saturate. On the measurements, that is not what is happening: the growing mode has no Froude threshold, so it is not the instability the advective term would bound. What *is* now visible is that the breaking cap does real work at the steep end (`ρ no-cap` = 1.02298 at 70% against 1.00000 shipped), and the cap is precisely an unphysical stand-in for the missing advective saturation. So the honest statement is: **the closure upgrade is owed by the cap's existence, not by a roll-wave instability** — the debt is real and its justification changes. It becomes *due* when a claim depends on supercritical flow being quantitatively right; nothing in the current ladder does.

### §6.5 P8c — the grid divergence is gone (measured, not inferred)

The strongest indictment of the retired form was that its growth rate *diverged* under refinement. Re-run after the demotion (`VIVARIUM_ROLLWAVE_ONLY=7`), 5% slope, cap on, dt = 0.02 s, domain 307.2 m:

| l (m) | nx | σ SHIPPED **before** | σ SHIPPED **after** | ρ n-CONST after |
|---|---|---|---|---|
| 4.80 | 64 | −0.00417 | −0.00149 | 0.999970 |
| 2.40 | 128 | −0.00003 | −0.00002 | 1.000000 |
| 1.20 | 256 | **+0.02787** | **−0.00003** | 0.999999 |
| 0.60 | 512 | **+1.55559** | **−0.00000** | 1.000000 |

`ρ SHIPPED` equals `ρ n-CONST` to every digit printed, at every grid. The term that got worse without bound as the mesh refined — the thing that made it a hazard for a multiresolution architecture specifically — is now indistinguishable from holding `n` fixed. `DECISIONS[jarrett-is-inconsistent-not-merely-artefactual]` is closed by the demotion, and becomes history rather than an open indictment.

**Probe usability, fixed in passing.** The full `roll_wave` sweep is ~40 minutes and had no way to run one section — so re-verifying one claim after a kernel change cost a whole sitting, which is why the sweep that was supposed to produce this table died unfinished. `VIVARIUM_ROLLWAVE_ONLY=7` now runs §7 alone in about a minute. Sections are independent (each relaxes its own base state), so the same gate generalises to the others when someone needs them.
