# Drafted patches — for Joseph's approval. NONE APPLIED.

*2026-07-13, from the onboarding audit. Every patch is `:by claude :status proposed`.
Authority tags are **flagged, not fixed**. `LEXICON.udon` is **not touched**.*

---

## 1. [MECHANICAL, unblocked] Make the theory doc reachable from the front door

**Why:** `CLAUDE.md` L11's doc-map omits the doc `CLAUDE.md` L37 calls *required*, and
`ORIENTATION.md`'s reading list omits it entirely. **A fresh session following ORIENTATION never
reads it.** Highest value-per-character fix in the audit.

**`CLAUDE.md` line 11** — in the doc-tree sentence, replace:

> `theory/` (multiscale-methods, multiscale-seams)

with:

> `theory/` (**discretisation-and-information** — required before any field nomos; multiscale-methods, multiscale-seams)

**`ORIENTATION.md` item 8** — replace:

> 8. `doc/theory/` — the technical core: `multiscale-seams.md` (position AND time
>    as one seam discipline; the resolution-light-cone / dynamic-exponent-$z$
>    unification) + `multiscale-methods.md` (the R/L/closure operator algebra).

with:

> 8. `doc/theory/` — the technical core. **`discretisation-and-information.md` is REQUIRED
>    before authoring or reworking any field nomos** (the Prime Question, the FVM/FDM taxonomy,
>    the structure table, bias-vs-noise, the four probes — `CLAUDE.md` Level B). Then
>    `multiscale-seams.md` (position AND time as one seam discipline; the
>    resolution-light-cone / dynamic-exponent-$z$ unification) + `multiscale-methods.md` (the
>    R/L/closure operator algebra).

---

## 2. [MECHANICAL half of P0] Stop `ARCHITECTURE.md` teaching the mean-pin as safe

**Why:** `ARCHITECTURE.md` is Level B and routes every nomos author. It still states mean-pin as
*the fidelity invariant* with no flag. `DECISIONS[column-is-a-control-volume]` cites this very
line **in its own `|ref` as the evidence of the error** — and then nobody annotated it.

**Insert after `doc/ARCHITECTURE.md` §1's three laws (after line 34):**

> > [!warning]
> > **⚠ Law (1) is contested, and the contest is live (2026-07-13).** "$R\circ L=\mathrm{id}$ **on
> > the mean**" is only coherent if a stored value **is** a cell average — and **the code does not
> > agree with itself about that**: `mean-pin` reads it as a cell average, `voxel.rs`'s bilinear
> > reads it as a point sample, `gen.rs`'s `nyquist_level` produces a band-limited sample, and
> > `doc/design/DESIGN-MATERIAL.md` §4/§11 declares it a **centre point-sample** and marks that
> > *settled*. **The fork is open and it is Joseph's** (`TODO.md` §⛔ THE FORK).
> >
> > And **the mean is not enough regardless**: stream power $E = K A^m S^n$ is nonlinear in slope,
> > so $\overline{f(S)} \neq f(\bar S)$ — **carrying only the mean does not blur, it lies**
> > (Jensen ≡ aliasing). A column must carry a **sufficient statistic** (at minimum a sub-grid
> > variance) plus its exactness. ⇒ **`doc/theory/discretisation-and-information.md` (Level B,
> > required before any field nomos)** · `DECISIONS[column-is-a-control-volume-with-sufficient-statistics]`.

**Also add the theory doc to `ARCHITECTURE.md`'s header pointer list (line 3),** which currently
names only `multiscale-methods` / `multiscale-seams` / the DESIGN docs.

---

## 3. [MECHANICAL half of P0] Delete the false claim — integration is replacement

**Why:** three documents assert *"nobody has ever said what a column is."* **It is false.**
`DESIGN-MATERIAL.md` §4 said it, with reasoning, citing GDAL's `AREA_OR_POINT` registration as
prior art, and marked it **Settled** with an explicit guard *"so a later tier can't quietly treat
it as a mean/max."* A later tier then quietly treated it as a mean. **The true finding is stronger
than the false one and should replace it verbatim** (never kept-softened-with-a-pointer).

**Replacement text**, for the theory doc §5's last table row, and for
`DECISIONS[column-is-a-control-volume]`'s `|reason`:

> **`DESIGN-MATERIAL.md` §4 DECLARED the semantics** — stored primitive = the **cell-integrated
> conserved volume/mass** ("this is finite-volume thinking"); surface elevation = a **derived
> centre point-sample** — and marked it *Settled*, with an explicit guard: *"so a later tier can't
> quietly treat it as a mean/max."* **The code then implemented neither**: it stores a `Vec<f32>`
> of *heights*, mean-pins them (cell-average), bilinearly interpolates them (point-sample), and
> generates them band-limited. **The sin was not silence. It was drift from a declaration nobody
> re-read** — which is a harder finding, and a more useful one, because a fresh declaration will
> drift the same way unless the code is what carries it.

**And the same doc's §7(a) must cite its own prior art:** `DESIGN-MATERIAL.md` §4's prior-art list
already contains ***Arakawa staggering***. The theory doc presents staggering as a lead the grid
report never tested. It is a **re-discovery of the project's own Level-B design doc**, and saying
so costs the doc nothing and makes it trustworthy.

**⚠ NOT drafted, because it is Joseph's:** whether `DESIGN-MATERIAL` §4/§11's *"Settled: surface
elevation = center point-sample (renderer-committed)"* now **stands** or is **superseded** by
`DECISIONS[column-is-a-control-volume]` (*"It is NOT a point"*). One of the two must yield, and
`SUPERSEDED.md` must record it. **The current state — a Level-B doc marked *settled*, flatly
contradicted by a ratified decision, with no cross-reference in either direction — is the worst of
the three options.**

---

## 4. [MECHANICAL] `ASSUMPTIONS.md` — the ledger did not track its own live finding

**Why:** theory doc §2.5 and `TODO.md` both say this row must change. It did not. Project law:
*touch a constant, touch the ledger.*

**Split the de Almeida–Bates entry out of the shared row.** Replace row 42:

```
| Jarrett roughness, Lekner–Dorf wet darkening, de Almeida–Bates stabilization | per papers | **literature** | used as intended |
```

with **two** rows:

```
| Jarrett roughness, Lekner–Dorf wet darkening | per papers | **literature** | used as intended |
| de Almeida–Bates θ flux smoothing | θ = 0.8 | `water.rs` (θ-smoothing pass, ~L362–370) | **literature** — ⚠ *and its physical claim may be NONE* | ⚠ **PROPOSED RE-STATUS (`:by claude`, 2026-07-13, NOT ratified):** the theory doc argues this is **Rhie–Chow-class STABILISATION** — an unphysical high-pass damping term whose job is to suppress a mode the collocated scheme *cannot see* (the "travelling soliton blobs"; the Nyquist/checkerboard mode in the operator's null space). If so, θ is a **filter gain**, not a modelling choice, and "used as intended" is true but not the true thing. **This is an inference, marked `[me]` in its source — it is NOT measured.** Falsifier, and it is cheap: the **null-space / eigenvalue probe** (Cardiff §3.5.3 / theory doc §6, probe 1) — assemble `water.rs`'s operator on a periodic patch and count zero eigenvalues; every one beyond the rigid-body modes is a mode the kernel is blind to. **Second, coupled prediction (also unmeasured):** θ adds dissipation with **no matching fluctuation**, and Mori–Zwanzig says they are a pair — so `water.rs` should be systematically **under-energised** at the sub-grid scale. **That would be a bias.** (theory doc §2.5, §3.4; `TODO.md` §discretisation-foundation) |
```

*Note the register, deliberately:* the ledger records the **claim as proposed with its falsifier**,
not as settled. The theory doc's own tag is `[me]`, and the ledger must not launder an inference
into a measurement.

---

## 5. [MECHANICAL] The decision log violates its own supersession schema

**Why:** the header rule is *"⚠ WHEN USED, `:supersedes` GOES ON THE `|decision[...]` LINE ITSELF —
never on its own line — so that `grep '^|decision\['` yields the whole supersession chain in one
read."* **`grid-report-supersedes-two-of-my-claims` puts it on its own line.** I grepped, and got
the chain wrong — which mattered, because the entry it supersedes carries a grid verdict in
Joseph's name.

**In `DECISIONS.decision-log.udon` (~L339–341):** move `:supersedes seam-amortization-and-the-two-grid-overlay`
off its own line and onto the header:

```
|decision[grid-report-supersedes-two-of-my-claims] :date 2026-07-12 :by claude :status proposed :topic physics :supersedes seam-amortization-and-the-two-grid-overlay
```

(Delete the standalone `:supersedes` line. The `:note` line stays — bespoke `:`-metadata on its
own line is explicitly allowed; only `:supersedes` is constrained.)

---

## 6. [🚩 FLAG ONLY — DO NOT APPLY. Joseph's authority.] The three `:by us` tags the 07-12 pass missed

*Drafted as `:note` lines so that, **if** he agrees, the correction is one paste each. The tag
changes themselves are **his** — I have deliberately not written them into the file.*

**(a) `snyder-closes-the-projection-lead`** — currently `:by us :status superseded`. Its statement
is **verbatim the retracted verdict**: *"KEEP the equiangular cube-sphere and CellId; change the
KERNELS, not the grid."* Joseph supplied the **Snyder PDF** (a source, and the entry says so);
the **verdict** was inferred from it. Proposed `:note`, matching the two entries the 07-12 pass did
correct:

```
  :note 2026-07-13 — ⚠ AUTHORITY QUESTIONED (`:by claude` audit, unratified). This entry's
    STATEMENT is verbatim the verdict that `grid-question-not-closed-authority-was-inflated`
    records Joseph as NOT having made. He supplied the Snyder primary; the verdict was drawn
    from it. The 2026-07-12 correction pass downgraded `conservation-is-not-consistency` and
    `grid-report-supersedes-two-of-my-claims` and MISSED this one. `:status superseded` does not
    cure it: a reader grepping `:by us` still finds the grid verdict in Joseph's name.
    PROPOSED: `:by claude`. Joseph's call.
```

**(b) `seam-amortization-and-the-two-grid-overlay`** — currently `:by us :status superseded`.
Contains *"**That is the argument that decides the grid does not need replacing.**"* Joseph
supplied an **intuition** (*"some way to do partial fluxes…"*), which the entry itself labels as an
intuition; the verdict was drawn from it. Same proposed `:note` shape.

**(c) `geometric-contract-metric-set`** — currently `:by us :status decided`, and its `|reason`
states *"the configurations cluster into **~10 congruence classes**."* **Measured false** by
`configuration-classes-are-two-not-ten`: the discrete count is **2** at every level; the ~10 is the
*geometric* count and grows as N²/8 (509 by L6). TODO carries the correction; **this entry does
not.** This `:note` is **purely mechanical and does not touch the tag** — safe to apply:

```
  :note 2026-07-12 — ⚠ ONE NUMBER IN THIS ENTRY IS MEASURED FALSE. "The configurations cluster
    into ~10 congruence classes" is corrected by `configuration-classes-are-two-not-ten`: the
    DISCRETE/topological class count is **2**, fixed at every level (a regular cell + the 24
    defective ones). The ~10 is the GEOMETRIC congruence count and it does NOT stay finite
    (N²/8; 509 by L6). Building a fixed dispatch table sized "~10" is correct at L3 and wrong at
    L4. The entry's STRUCTURE (discrete branch + continuous metrics computed, not tabulated) is
    unaffected and still right. [Separately: this entry's |impact block is a Claude design
    proposal sitting under a `us` tag — worth Joseph's eye.]
```

---

## 7. [🚩 FLAG ONLY] The governing principle's own tag

**`preserve-the-structure-declare-the-sacrifice`** · `:date 2026-07-13` · **`:by us :status
decided`** — and it is now the governing law in `CLAUDE.md`, the highest-privilege surface.

Its `|reason` cites Cardiff and *"the information-theoretic reading Joseph asked for"*, plus one
Joseph quote — **but the quote is about jitter, not about the principle.** There is **no recorded
ratification of the principle itself.**

**The entry one slot away does it exactly right:**

```
|decision[column-is-a-control-volume-with-sufficient-statistics]
  :status-note The FRAME is settled (Joseph: "it very much lands"). The IMPLEMENTATION is not
               designed. Do not read this as a spec.
```

**That is what a ratified `us` looks like:** the assent is quoted, and its scope is bounded.

I cannot see the conversation — he may well have ratified it in as many words. **But the record
does not show it, and the record is what a fresh agent reads.** And the shape is exactly the one
the authority legend was written *the day before* to prevent: the instance that authored the
legend on 07-12 tagged **its own governing principle** `:by us` on 07-13, with no ratification
quote, and wrote it into `CLAUDE.md` the same day.

- **If he ratified it:** the fix is **one line** — a `:status-note` quoting him.
- **If he did not:** `:by claude :status proposed`, and `CLAUDE.md` keeps the principle but
  attributes it honestly.

**Either way the principle survives. Only the tag is in question.** (Same question, weaker, for
`bias-vs-noise-is-the-decisive-audit`, whose quote is prefaced *"Joseph, 2026-07-12, **reaching for
it**"* — reaching for is not deciding.)

---

## 8. [🚩 FLAG ONLY — Joseph's call] `LEXICON.udon`

**Not touched.** Assessment and the minimal patch, for approval.

**The graph is worse than "three stale statuses":** **13 `|rel :to regula` edges** — `regula` is a
**hub node** — and **`|term[vivium]`**, the project's most central noun, carries `|rel :to regula
:kind declares-conformance-to` **and** `|rel :to manifest :kind individuated-by`. **The definition
of *vivium* points at one retired term and one term that does not exist.**

**Why it misleads badly:** Level B says *"`LEXICON.udon` for every term you touch."* An agent
authoring a nomos touches `vivium`, follows the edge to `regula`, and reads a convincing 20-line
`:status settled` entry — etymology, supremacy clause, nesting rule, lineage discipline — for an
artifact that **does not exist and will not be built**. `:status settled` is defined **in the
LEXICON's own header** as *"in live use, collision-checked."* The warnings exist — in
`SUPERSEDED.md` and the decision log — **i.e. in the two files the reader was not sent to.** *The
gate sends you to the one file that does not carry the warning.* **I did exactly this and it cost
me.**

### ⚠ The blocking sub-decision, and it is his

**The LEXICON's `:status` vocabulary is `settled / carved / open`. There is no value meaning
*retired*.** The patch cannot be written until Joseph either **(a)** adds a status value
(`retired` / `superseded`), or **(b)** decides these entries leave the LEXICON for `SUPERSEDED.md`,
which already holds their story. **That, not the three tags, is what actually blocks the fix.**

### The minimal honest patch, in three parts

1. **✅ UNBLOCKED, ADDITIVE, could land today — add `|term[manifest]`.** It is real, it is in code,
   and `spec.rs`'s own doc comment already defines it (*"The vivium manifest — the file that
   individuates a world"*). **Two entries already link to a term that does not exist.** This part
   needs no schema decision. Draft:

   ```
       |term[manifest]
         :status settled
         :since 2026-07-12
         The per-world PRESCRIPTION: the file that individuates a vivium — identity (seed,
         minted once), label, :order, :target-phase, permits, participation, and the demand
         buckets the builder draws toward. Where the ORDINUM is descriptive (what the KIND of
         world requires, phase by phase), the manifest is prescriptive (what THIS world chose).
         Together they carry world-level conformance; no third artifact stands between them
         (DECISIONS[regula-collapses-to-order-and-manifest], Joseph, 2026-07-12).
         |not spec — the code file is `spec.rs` for historical reasons; the TERM is manifest.
         |rel :to vivium :kind individuates
         |rel :to ordinum :kind prescribes-against
         |rel :to beacon :kind holds
         |source crates/vivarium-world/src/spec.rs · DECISIONS[regula-collapses-to-order-and-manifest]
   ```

2. **⛔ BLOCKED on the schema call — retire `regula` / `slot` / `permit` / `regula-terrestris`.**
   Status change + one `|carve` line each pointing at
   `DECISIONS[regula-collapses-to-order-and-manifest]` and `SUPERSEDED.md`.

3. **✅ One line, and it matters most — repoint `|term[vivium]`'s edges** from `regula` to
   `ordinum` + `manifest`. The central noun should not define itself through a ghost.

---

## 9. [🚩 FLAG] Two loose ends

- **`tmptmp.md`** — 7 KB of raw dialogue in the repo root; in no doc map, not in `.gitignore`, not
  in `.archive/`. It is the substrate of the theory doc's §4.2 curvature derivation, **which
  landed** — so the file is spent. A fresh agent cannot tell whether it is canon (I read it to find
  out). **Not mine to delete.** Proposed: `msc/`, or gone.
- **Three names for one noun** — ORIENTATION says **Manifest**, the code says **`spec.rs`**, the
  LEXICON says **nothing**. One cheap decision.
