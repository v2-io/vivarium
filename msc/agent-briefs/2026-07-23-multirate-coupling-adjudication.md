# Adjudication — `#form-multirate-coupling` (promote vs leave)

*Peer brief for parent session. 2026-07-23. Not claim canon.*

> **Parent re-weight (Joseph 2026-07-23):** Kill of inventory `#form-multirate-coupling` stands because the stance is **already owned by seam + R/L segments** (dual-home risk) — **not** because the water↔erosion coupler is unbuilt. Unbuilt coupler = compliance debt under already-landed law. Finer residual grains (directional separation, $z$) still need design/primary strength, not code completeness.

**Reader:** decide promote vs leave. Verdict first. Primaries and landed segments checked; code checked for live coupling.

---

## Verdict

**Do not promote `#form-multirate-coupling` as inventory-named.**

The REDUX §4 stance that inventory rows call “multirate coupling” — *each aspect on its own step; fast sees slow as quasi-static; slow sees fast as time-averaged* — is **already owned**, split cleanly across:

| Content | Landed home |
|--------|-------------|
| Time seam = multirate bands; quasi-static / time-averaged gloss; Gear–Wells pattern named | `#form-seam-flux-exchange` FE(4) |
| Scale separation as honesty law; failure ⇒ no multiscale method saves you; water/erosion specimen “never one shared timestep” | `#form-rl-closure-algebra` FE(2).3 |
| Spatial **and** temporal resolution by present demand | `#form-fidelity-invariant` |

A new formulation segment that restates “fast/slow, quasi-static/time-averaged” would be a **dual home of seam law**, not residual gold. Kill that promotion.

**What remains unowned is not one multirate package.** It is three *finer* grains with different truth-status (see Residual grains). None of them is correctly named `#form-multirate-coupling`. Promoting under that slug would either restate seam law or smuggle conjecture under an established label.

---

## What was verified (primaries)

### DESIGN-REDUX §4

Stance text only:

- Aspects have wild timescales; do not force one grid.
- Couple **multirate**: own step per aspect; fast←slow quasi-static; slow←fast time-averaged; defined coupling schedule.
- Generalizes the hydrology lesson to N aspects.
- Prior art: multiscale modeling + Earth-system flux couplers (mature forward direction).
- Open named elsewhere (§6 reversion), not in §4.

No independent formal objects beyond what seam + R/L already encode. No probe obligations unique to §4.

### `doc/theory/multiscale-seams.md`

Teaching / source (banners already point at the three claim homes). Relevant unpack:

- **§2.3 aspect-coupling seam** — the *time seam proper*: Gear–Wells interpolate-slow / extrapolate-fast; vivarium **additionally** time-averages the fast for erosion (HMM/upscaling, *not* pure Gear–Wells); slowest-first + power-of-two as memo/backup precedent; **Prop 4.1**: stability requires small **fast→slow** Jacobian block (direction of weak coupling).
- **§3 dynamic exponent $z$** — *stance / conjecture* for $z$-reconciliation at a seam; measured asymmetry: water $z{=}1$ via `stable_dt`; creep $z{=}2$ **bound but clamps**; tactical “timestep from `CellId` level” is Joseph steer, unbuilt as law.
- Explicit: `$z$-consistent resolution` is **project coinage**, not a known well-posedness criterion tied to hyperbolic–parabolic literature yet.

### Landed segments (read full)

- **`#form-seam-flux-exchange`** already owns one discipline on space×time, time-seam multirate gloss, and **explicitly refuses** $z$-reconciliation as established numerics. Working Notes: do not absorb method zoo / $z$-from-quadtree as this file.
- **`#form-rl-closure-algebra`** already owns scale-separation law + water/erosion specimen. Working Notes: method-zoo taxonomy, dynamic-exponent $z$, and detail→abstract **remain source until segmented**.
- **`#form-fidelity-invariant`** Working Notes lists “multirate coupling” as a **sibling not this file** — that is inventory aspiration, not proof of an unowned architecture claim. The sibling list should be read as *possible further grains*, not a mandate to invent a dual home.

### DESIGN-SYSTEMS coupling bands

Four bands (deep / orbital-climate / surface process / fast-biological) + build-order table. **Inventory and judgment**, file banner says not claim canon. Timescales themselves are standard Earth-system order-of-magnitude (high confidence as teaching map). Not a missing formulation segment by themselves.

### Code / live coupling (`crates/vivarium-world`)

| Fact | Evidence |
|------|----------|
| Water and erosion are **separate kernels** with separate timesteps (CFL `stable_dt` vs geological `epochs`) | `water.rs`, `erosion.rs` |
| Erosion’s stream-power uses **MFD drainage area from terrain** (and optional precip field), **not** time-averaged discharge from the water kernel | `erosion.rs` drainage / discharge comments; no water-kernel consume |
| Module docs still name “multirate water coupling (§4)” as a **next** increment | `erosion.rs` header |
| Port notes: kill-switch deleted in testbench fill path (2026-07-03); **OPEN: discharge→A coupling** into erosion | `ref/erosion-port/NOTES.md` |
| No production multirate schedule object, no Gear–Wells slowest-first integrator, no face flux of time-averaged discharge as coupler | flux web is quantity *names*; quantity-level schedule derivation is still design |

**Authority ≠ evidence.** The design stance is established *as prior art + project formulation*; the **specimen coupler is not built**. A segment that says “vivarium couples water and erosion multirate via time-averaged discharge” would be **false of the live stack**.

### Candidate inventory cross-check

`core-segment-candidates-2026-07-14.md` does **not** list a single `#form-multirate-coupling`. It lists finer candidates that inventory later collapsed into that slug:

- `#timescale-separation-multirate` (normative specimen: ~$10^{10}$, schedule not kill-switch, both ON)
- `#scale-separation-is-directional` (Gear–Wells; declare weak-coupling direction)
- `#dynamic-exponent-z`, `#timestep-from-quadtree-level`
- band/phase-adjacent items (`#phase-intrusion-budget`, water FAST-band placement flags)

Promotion mine row `#form-multirate-coupling` (REDUX §4; seams | Fast/slow) is a **lossy compression** of those grains into a dual-home shape.

---

## Residual grains (honest map)

If the parent session still wants *something* segmented from this neighbourhood, treat these as **separate decisions**, not as one multirate segment.

### 1. Already covered — do not re-segment

- Multirate as the **time axis of the seam** (quasi-static / time-averaged).
- Scale separation as **R/L honesty law (3)** with water/erosion specimen line.
- Observer temporal LOD / perceptual band (fidelity invariant; LEXICON).

**Action:** demote inventory row `#form-multirate-coupling` to “covered by seam + R/L”; optionally tighten `#form-fidelity-invariant` Working Notes sibling list so it does not re-invite the dual home.

### 2. Distinct formal content, promote-worthy *if* narrowed — `#result-scale-separation-directional` (suggested)

**Claim shape (not FE draft):** Scale separation is **directional**. Multirate stability (Gear–Wells 1984 Prop 4.1, primary-read in multiscale-seams) requires the **fast→slow** coupling block of the Jacobian to be small; when the system is not near block-triangular, “any form of stability at infinity is lost.” Project cash-out: erosion may read water freely (slow←fast summary); water must treat terrain as quasi-static (fast↛slow strong feedback). Where the weak direction is wrong or absent, **no schedule saves you**.

| | |
|--|--|
| **Distinct from seam law?** | Yes — seam law says *what crosses* and *quasi-static/averaged*; this says *which coupling direction must be weak* and *why failure is formal*, not just empirical hydrology lore. |
| **Max attainable** | **exact** for the Gear–Wells proposition under its ODE multirate setting (cite primary; do not transplant citation numbers without re-open). |
| **Project application** | `robust-qualitative` or `discussion-grade` until a nomos declaration / probe exists. “Nomos should declare weak-coupling direction” is **claude-proposed** (candidates inventory); nothing in `NomosDecl` holds it. |
| **Status if drafted** | `result` or `derived` for the formal half; **do not** mark the declare-direction box as settled. |
| **Depends** | `#form-rl-closure-algebra` (strengthens law 3); cites `#form-seam-flux-exchange`. |
| **What NOT to claim** | That vivarium measures Jacobian blocks; that all four DESIGN-SYSTEMS bands are formally scale-separated; that time-averaging *is* Gear–Wells (it is an extra HMM move — seams already says this). |

This is the only residual I would **defend under hostile read** as claim-shaped without inventing architecture.

### 3. Specimen / paid lesson — optional thin normative, not multirate architecture

**`#timescale-separation-multirate` / water×erosion:**

- Water and erosion timescales are orders of magnitude apart (order-of-magnitude teaching; ~$10^{10}$ is lore-scale, not a measured project constant in ASSUMPTIONS).
- Separation is by **coupling schedule**, not a kill-switch that turns erosion OFF during water settle.
- Both stay ON; kill-switch deletion had a voice-discipline episode (premature 07-02 claim; 07-03 correction).

| | |
|--|--|
| **Truth enough?** | As **history + normative intent** for the water/erosion pair: yes. As “the stack implements schedule coupling with discharge→A”: **no** (OPEN). |
| **Distinct?** | Mostly **expansion of R/L specimen + port NOTES**, not new architecture. Prefer a sentence upgrade in `#form-rl-closure-algebra` Working Notes / FE specimen over a new segment. |
| **If segmented anyway** | `normative` or `observation`, `status: empirical` / `robust-qualitative`, stage draft; **compliance debt:** discharge→A unbuilt; do not claim both-ON implies honest multirate flux exchange. |

### 4. Explicitly conjecture / unready as multirate claim — $z$ package

| Grain | Status |
|-------|--------|
| Water CFL ⇒ $z{=}1$ scaling in live `stable_dt` | Measured behaviour of kernel (can support an observation later) |
| Creep $k=\kappa/\mathrm{cell}^2$ bound $z{=}2$, **clamp** not substep | Measured fidelity compromise |
| Seam well-posed only under “$z$-consistent” resolutions | **Project coinage / conjecture** — seams and form-seam both refuse this as established |
| Derive $\Delta t$ from `CellId` level power-of-two per $z$-sector | Design steer (2026-07-10), unbuilt |

**Do not** fold $z$-reconciliation into a multirate-coupling promotion. If ever segmented: honest `hypothesis` / `sketch` with max-attainable clearly below exact for the reconciliation claim; water/creep asymmetry can be `observation` without the well-posedness claim.

### 5. Teaching-only / inventory — leave in design

- Four multirate **bands** table (DESIGN-SYSTEMS / ARCHITECTURE §6).
- Phase-transition as largest multirate interface (AAT cut texture).
- Method-zoo multirate row (multiscale-methods) — correctly parked by R/L “what this is not.”
- Exact coupling schedule numbers, step ratios, termination under continuous coupling (port OPEN).

---

## If someone forces a multirate segment anyway

Hostile-read FE I would **not** defend as exact; only as a thin pointer that should not exist if R/L + seam stay primary:

> Aspects couple multirate: each on its own step; coupling objects are fluxes/statistics already required by `#form-seam-flux-exchange`; honesty requires scale separation as in `#form-rl-closure-algebra` law (3).

That is **restatement**. Status would be forced to `discussion-grade` or withdrawn. Prefer **not writing it**.

Hostile-read FE I **would** defend (narrow residual only) — not under slug `#form-multirate-coupling`:

```text
slug: result-scale-separation-directional   # name open
type: result
status: exact          # for Gear–Wells Prop 4.1 under stated setting
# + separate status line or FE for project cash-out: robust-qualitative / sketch
depends: [form-rl-closure-algebra, form-seam-flux-exchange]

FE (sketch):
1. Multirate integration of slow/fast components is absolutely stable
   only when the fast→slow coupling block is small (Gear–Wells 1984 §4).
2. Therefore scale separation is directional: the weak block is
   fast→slow; slow may integrate summaries of fast.
3. Where timescales do not separate in that direction, resolve jointly
   or accept unbounded error — no multirate schedule repairs it.
4. Vivarium specimen (intent, not live coupler): water treats terrain
   quasi-static; erosion may consume time-averaged discharge (when built).
5. Out of bounds: claiming live discharge→A multirate coupler;
   claiming z-reconciliation; claiming kill-switch lesson is the same
   as Prop 4.1 without the Jacobian direction.
```

**What NOT to claim in any of these:**

1. That `#form-multirate-coupling` is still “unowned gold.”
2. That REDUX §4 is unsegmented law (it is demotable teaching once dual homes are acknowledged).
3. That $z$-reconciliation is established numerics.
4. That four bands are project law (they are an Earth-system map).
5. That mean-pin / face register / R∘L are multirate results (they are spatial/coarse-fine; already segmented).
6. That “both stay ON” means the production path has schedule coupling — only that the kill-switch is the wrong separation tool.
7. Conflating Gear–Wells extrapolate-fast with vivarium’s **time-average** of fast for erosion (seams already separates these).

---

## Recommended parent actions

1. **Kill** promotion of `#form-multirate-coupling` (this brief’s primary deliverable).
2. **Strike or rewrite** promotion-mine residual row 2 → “covered by `#form-seam-flux-exchange` + `#form-rl-closure-algebra`; optional residual: directional scale-separation result.”
3. **Optional next promote** (only if claim mass wanted): `#result-scale-separation-directional` (or similar), FE as above, primary Gear–Wells, project cash-out under softer status; **no** $z$, **no** bands table, **no** built-coupler claim.
4. **Optional micro-edit** (not a segment): one line on `#form-rl-closure-algebra` specimen noting schedule-not-kill-switch + discharge→A still OPEN — history in DECISIONS/port NOTES, present-tense debt in Epistemic Status.
5. **Leave** $z$ / quadtree-$\Delta t$ in theory until a probe or Joseph stamps the steer as more than design.
6. **Do not** demote entire multiscale-seams.md or DESIGN-REDUX §4 to super-archive yet — still teaching + residual grains (2)–(4); dual-home *paragraphs* already pointed at claim homes.

---

## Brief feedback (welcome, as requested)

- Inventory compression (many candidates → one “multirate” slug) is the failure mode that almost created a dual home. Prefer grain-level names from the July-14 candidates table when residual is real.
- `form-fidelity-invariant` sibling list (“multirate coupling”) primed re-extraction; treat such lists as **hypotheses of unowned mass**, convictable by reading the three form segments — which this pass did.
- Truth above decisions: REDUX §4 *feels* like architecture gold because it is well written; after segment map + code, it is **mostly absorbed**. Residual value is Gear–Wells directionality and honest $z$/compliance debt, not a fourth restatement of fast/slow.

---

## Sources touched (absolute)

- `/Users/josephwecker-v2/src/arch/vivarium/doc/design/DESIGN-REDUX.md` §4
- `/Users/josephwecker-v2/src/arch/vivarium/doc/design/DESIGN-SYSTEMS.md` (bands)
- `/Users/josephwecker-v2/src/arch/vivarium/doc/theory/multiscale-seams.md` §§1–3, 2.3
- `/Users/josephwecker-v2/src/arch/vivarium/doc/theory/multiscale-methods.md` §1–2
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-seam-flux-exchange.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-rl-closure-algebra.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core/src/form-fidelity-invariant.md`
- `/Users/josephwecker-v2/src/arch/vivarium/crates/vivarium-world/src/erosion.rs` (header + drainage/discharge)
- `/Users/josephwecker-v2/src/arch/vivarium/crates/vivarium-world/src/water.rs` (`stable_dt`, `step`)
- `/Users/josephwecker-v2/src/arch/vivarium/ref/erosion-port/NOTES.md`
- `/Users/josephwecker-v2/src/arch/vivarium/core-segment-candidates-2026-07-14.md` (scale-separation / multirate rows)
- `/Users/josephwecker-v2/src/arch/vivarium/msc/promotion-mine-2026-07-23-continuity.md`

*On the line for follow-ups.*
