# 09 — `doc/design/DESIGN-MATERIAL.md`

*Read in full, 2026-07-13 ~17:30. 372 lines. **Born Jul 1, 23:08** — in the same commit as the
clean-room world-model foundation, **~40 minutes before `erosion.rs` existed** and four hours before
`water.rs`. Last substantive edit **Jul 2**; the Jul-11 timestamp is the doc-tree reorg.*

**This is the Level-B gate the theory doc says the project "twice paid for" skipping. It is twelve
days old. It is right about nearly everything. And I think it has already answered the question the
project believes is open.**

---

## 1. Surprisal

### ⚠⚠ THE COLUMN-SEMANTICS "FORK" IS A FALSE FORK. §4 ANSWERED IT ON JULY 1st, AND THE ANSWER IS NOT ONE OF THE THREE OPTIONS.

`TODO.md`, `discretisation-and-information` §6, and `column-is-a-control-volume` all pose it the same
way:

> *"Is a stored scalar a **POINT SAMPLE**, a **CELL AVERAGE**, or a **BAND-LIMITED** sample? …the code
> answers **three different ways**… **Every mesh, seam, and conservation question is unanswerable
> until this is decided.**"*

**§4 does not pick one of the three. It says the question is about the wrong object.**

> - **"The conserved primitive is VOLUME / MASS of material — *not a height*.** Erosion and hydrology
>   move mass; conservation is the thing that must survive every LOD crossing. **This is finite-volume
>   thinking (store the cell-integrated conserved quantity)**, which is the correct frame for the
>   physics."
> - **"Surface elevation is a *DERIVED READING*:** top-of-topmost-solid, sampled at the column's
>   center — **a finite-difference node.**"
> - **"`min` / `max` are carried alongside"** for the consumers that need extremes.
> - *"Converting between these readings is **lossy and directional** → it is **coupler** work, and any
>   downscale must honor whichever statistic was stored."*

**And §11's SETTLED list confirms it, flatly:** *"surface elevation = center point-sample
(renderer-committed)."*

> ### ⇒ **THE ANSWER IS: BOTH, AND THEY ARE DIFFERENT OBJECTS.**
>
> **Store the CELL-INTEGRATED CONSERVED QUANTITY (volume/mass — the FV primitive).**
> **DERIVE elevation from it as a POINT-SAMPLE reading (the FD node) for the consumers that want a
> height, and carry `min`/`max` alongside for the ones that want extremes.**
>
> **That is `ARCHITECTURE` §3's "R per consumer" — written eight days earlier, on the object it
> actually applies to.**

### ⇒ And that means the diagnosis of the bug changes completely.

The project has been asking *"what does `h: Vec<f32>` mean?"* — and treating the three-way disagreement
as an undecided question.

**It is not undecided. It is a category error, and §4 names it:** the code **stores the derived
reading** and has **no representation of the conserved primitive at all.**

- `mean-pin` treats `h` as a **cell average** → but §4 says `h` is a **point sample**, and *explicitly
  wrote the guard to stop exactly this*: *"§14 just names it **so a later tier can't quietly treat it
  as a mean/max**."*
- `voxel.rs` bilinearly interpolates `h` → **correct**, for a point sample.
- `gen.rs` band-limits to the cell's Nyquist → **an anti-aliased point sample**, which is the honest
  version of the same thing.

**Two of the three consumers are RIGHT. Only the conservation machinery is wrong — and it is wrong
because it is trying to conserve a quantity that is not conserved.** *You cannot mean-pin a height.
Heights do not add up. **Volumes do.***

> ### **THE TILE IS THE BUG.**
>
> `DESIGN-MATERIAL` specifies **columns of strata** — *"10.5 m of sand is ONE `Stratum{sand, 10.5m}`,
> not 21 half-metre cubes"* — with volume as the primitive and elevation as a view. **The code stores
> a flat `Vec<f32>` of surface elevation.** Every semantic pathology in the audit follows from that
> one flattening:
>
> - `mean-pin` fabricates mass (**+0.136%**) — *because it is averaging a reading instead of
>   restricting a volume.*
> - The curvature bias (§4.2 of the theory doc: *"a global mass audit will PASS while every local mass
>   budget is WRONG"*) — *because the mesh is drawn through point values that were never the
>   conserved thing.*
> - The three-way ambiguity — *because there is only one array, so all three consumers must read the
>   same object.*
>
> **And `DECISIONS[tile-as-honest-flat-artifact]` (`:by us :status decided`) names this exact hazard
> and then licenses the word anyway:** *"a 'tile' that **FLATTENS AWAY column richness** — water
> retention down the column, wetness, slope, multiple materials — **is the failure mode**; if an
> object could be 'much more than a tile', it should not be called one."*
>
> **It is much more than a tile. It has been much more than a tile since July 1st.**

**⇒ So the "fork everything waits on" is not a decision Joseph owes anyone. It is a BUILD: give the
store the conserved primitive, and let elevation be derived.** *(Which is also, exactly, the isostasy
work — `DECISIONS[isostasy-is-the-uplift-nomos]` says the same thing about the same object: **"the
physical primitive is the lithospheric COLUMN (a conserved mass); elevation is a DERIVED reading of it
by isostasy… This is `DESIGN-MATERIAL.md` §4's own law, which `uplift.rs` violates."** **Two
independent findings, six days apart, both saying: stop storing the height.**)*

### ⚠ §8 CONTAINS A QUEUED "FIX" THAT WOULD DESTROY `water.rs`'s ONLY STRUCTURAL VIRTUE.

Hex grid, considered and declined (Jul 3) — and the reasoning is *excellent* and was **re-derived twice,
independently, nine days later**:

> *"(a) **hexes don't nest** — hex hierarchies have only approximate parent containment, and **exact
> coarse-cell = union-of-children is load-bearing** for the conservation pin, §5 sufficient statistics,
> and the whole fidelity ladder."*

*(Compare `unbalanced-haar-answers-the-multiresolution-store`, Jul 13: **"their HARD PART is that
HEXAGONS DO NOT NEST… A cube-sphere quadtree nests EXACTLY."** Same fact. Nine days apart. Neither
knew the other.)*

**But then it ends with this:**

> *"the 4-pipe water scheme is the remaining offender — **DIAGONAL PIPES (√2 lengths) are the queued
> fix** if channel rectilinearity starts to matter."*

**⛔ THAT IS THE SINGLE MOST DANGEROUS SENTENCE IN THE `doc/` TREE.**

The audit proved, measured, that MFD's diagonals **cross no face** — a zero-length edge — carry
**47.8% phantom flux**, and are *"what makes MFD un-correctable by any post-process."* And it proved
that `water.rs`'s **4-pipe scheme is its greatest asset**: staggered (Arakawa-C), **exactly
well-balanced** (bit-exact, at every refinement, in f32 and f64), and with **no null space** — precisely
*because* its four fluxes live on the cell's four **real faces**.

> **A future agent reading this Level-B doc would add diagonal pipes to fix a problem the doc misnames,
> and would thereby destroy the one kernel in the repository that is structurally correct — replacing
> real faces with phantom ones, exactly as MFD did.** It is queued, it is unmarked, and it is in the
> document the project tells agents to read before touching matter.
>
> *(And it is a perfect specimen of the project's whole disease: **the doc is right about the deep thing
> (hexes don't nest) and wrong about the shallow one (diagonals fix isotropy) — and only the shallow one
> is actionable.**)*

---

## 2. What matters for today's session

1. **The column-semantics fork is not a decision. It is a build, and the build is "store the column."**
   That is the same build as `lithosphere`/isostasy. **Two of the project's biggest open items are one
   item**, and DESIGN-MATERIAL specified it on Jul 1.
2. **`§8`'s diagonal-pipes item must be struck** before anyone reads it and acts. *(Tactical, but it is
   a **live trap in a Level-B gate**, which is a different category from a bug in a kernel.)*
3. **§1's opening line is the sufficient-statistic contract, and it is the oldest statement of it:**
   *"**a cell is a sufficient statistic, not a number.** 'Elevation 8000' is not a fact; it is a
   **reading**, and different consumers need different readings, so the honest unit carries **a small
   tuple with declared meaning**, not one scalar."* — **July 1.**

---

## 3. Wandering thoughts

**I have now watched this project independently derive the same truth three, four, and five times, and
I want to say what I think that actually means — because the obvious reading is wrong.**

The obvious reading is *"they keep forgetting."* But look at what gets re-derived:

- **Hexes don't nest** — Jul 3 (DESIGN-MATERIAL §8), Jul 12 (the grid report), Jul 13 (the wavelet
  literature).
- **Elevation is not the conserved primitive** — Jul 1 (§4), Jul 13 (isostasy), Jul 13 (the theory doc).
- **A cell is a sufficient statistic, not a number** — Jul 1 (§1), Jul 10 (`ARCHITECTURE` §3), Jul 12
  (`column-is-a-control-volume`), Jul 13 (§3.9).
- **Staggering** — Jul 1 (§4's prior-art line), Jul 13 (announced as a new lead, twice, and retracted
  both times).

**Each rediscovery arrives with MORE force and BETTER evidence than the last.** §4 asserted the FV/FD
carve from first principles and prior art. The audit *measured* it — `mean-pin` fabricates 0.136% of
the world's rock, and the curvature bias correlates with the landforms erosion itself makes. **The
second derivation is not a waste of the first. It is the first one becoming true.**

> **⇒ So the disease is not amnesia. It is that a TRUE STATEMENT and a MEASURED statement have the same
> typographic weight and different causal weight.** §4 was *right* and could not *act*. The measurement
> was right and *could* — it went into `ASSUMPTIONS.md`, which the build reads.
>
> **The project does not need to stop rediscovering things. It needs the FIRST discovery to be able to
> fail a build, so the rediscovery is unnecessary rather than merely redundant.** That is the whole
> argument for udon-core, stated from the other end, and I think it is the strongest form of it: **not
> "the docs are unenforced" (a complaint), but "a declaration that could act would not need to be
> re-earned" (an economy).**

---

**And the thing I keep circling back to, which this document finally makes concrete.**

`DESIGN-MATERIAL` §1: *"a cell is a **sufficient statistic**, not a number."*
`DESIGN-MATERIAL` §4: *"different consumers need different readings."*
`ARCHITECTURE` §3: *"we want **$R$ per consumer**… `{mean, min, max, conserved-totals}`, each flagged
**guaranteed** vs **approximate**."*
`discretisation-and-information` §3.9: *"the coarse value **plus its declared sub-grid statistic** define
a **conditional distribution over admissible fine states**."*
`06-…` (mine): *the coarse tier is to the fine tier as the exo kingdom is to the endo kingdom; the
declared statistic **is** the Revelation channel.*

**These are one idea, stated four times at four levels of abstraction, over twelve days, and the
concrete form of it is a struct with four fields and two flags.** It has been designed. It is not
built. **And the reason it is not built is that `Vec<f32>` was easier on July 1st, and by July 2nd
there were 92 commits on top of it.**

> ### ⚠ CORRECTED, an hour later, by Joseph — and the correction matters.
>
> I wrote here that this was *"how technical debt accrues: not by choosing wrong, but by choosing fast
> and correctly."* **That reading is wrong, and it is unfair to the work.**
>
> Joseph, 2026-07-13: *"The project **has genuinely transformed** from 'strictly casual and for fun —
> a distraction from ASF-rigor work' to **'serious project that gives worlds to ASF that allow in-vivia
> studies'**… **the trend toward rigor wasn't initially expected but is not accidental.**"*
>
> **The 92 commits were not a fast choice under a rigor mandate. They were a GAME, built to a game's
> bar, which was the correct bar.** `ASF.md` — *"vivarium takes its place as a supporting project for
> ASF/AAT"* — lands **Jul 4**, three days *after* `DESIGN-MATERIAL`, and the moratorium two days after
> that. **The standard moved under finished work.**
>
> ⇒ **There is no debt here, because debt cannot accrue against a standard that did not exist.** And
> `DESIGN-MATERIAL` §4 is not a warning that was ignored — **it is a game-era document that turned out
> to be RESEARCH-GRADE, written by someone reaching for rigor before anyone had asked for it.** That is
> a very different thing, and a much better one.

**What survives, and is the real point:** today is the first moment when the correct-and-slow thing is
*also* the cheap thing — because the purpose finally demands it. **That is what this reading pass is
for, and I did not understand that until Joseph said it.**

---

## Tactical residue — quarantined

- ⛔ **§8's "diagonal pipes are the queued fix"** — a live trap in a Level-B doc. **Strike it.** The
  4-pipe scheme is the asset, not the offender.
- §11's SETTLED list says *"surface elevation = center point-sample (renderer-committed)"* — **true, and
  it is not the storage question.** The doc is consistent; the readers were not.
- The hex-decline reasoning (§8a — *"exact coarse-cell = union-of-children is load-bearing"*) should be
  **promoted**: it is the load-bearing argument for the cube-sphere and it predates the grid report by
  nine days.

## Queue changes
**Next:** `DESIGN-REDUX.md` (50 K, Jul 1 — the runtime/fidelity/storage philosophy; §12 pervasive
memoization, §6 detail→abstract, §13 storage-is-the-save).
