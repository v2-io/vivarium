# 07 — `ASSUMPTIONS.md`

*Read in full, 2026-07-13 ~16:35. Only 68 lines — but the table cells are essays, and it is **one of
exactly two artifacts in the repo compiled into the build** (`include_str!` in `nomotheke.rs`; the
other is the ordinum). Last touched **Mon 03:55**, the audit's final commit.*

---

## 1. Surprisal

### ⛔ THE ENFORCEMENT IS ONE-DIRECTIONAL — AND IT CHECKS THE DIRECTION THAT DOESN'T MATTER.

This is the finding, and it is verified.

```rust
const LEDGER: &str = include_str!("../../../ASSUMPTIONS.md");
#[test] fn every_assumption_is_in_the_ledger() {
    for n in NOMOTHEKE { for a in n.assumptions {
        assert!(LEDGER.contains(a), "...ledger and nomotheke have drifted");
    }}
}
```

**It checks: `declared ⟹ exists in the ledger`.** A nomos may not lean on an assumption the ledger
does not carry. Fine, and it catches renames and deletions.

**It does NOT check: `consumed ⟹ declared`.** And *that* is the direction the whole mechanism was
built for — the nomotheke's own header says so:

> *"an undeclared magic value is latent, **undiscoverably unlawful**"* (Joseph, 2026-07-10) — the
> failure *"made discoverable."*

**It is not made discoverable. Measured, just now:**

| constant, added to `ASSUMPTIONS.md` on **2026-07-13** | consumed by | **declared in its `assumptions:` anchor list?** |
|---|---|---|
| **`MFD flow exponent P` = 1.1** — *"it MANUFACTURES the very bias it was supposed to cancel"* | `erosion.rs:369` | ⛔ **NO** |
| **`drainage cell area` = `cell_m²`** — *"a cube-locked fake erodibility field"* | `erosion.rs:371` | ⛔ **NO** |
| **`n` from Jarrett** (`1.6`, cap `0.13`) — *"a MEASURED positive feedback"* | `water.rs` | ⛔ **NO** |
| **θ flux-smoothing** = `0.8` — *"we run outside its published envelope"* | `water.rs` | ⛔ **NO** |
| **breaking cap** `Fr = 2.0` — *"measured saturated"* | `water.rs` | ⛔ **NO** |
| **water-surface float precision** (f32 η at a 4000 m datum) | `water.rs` | ⛔ **NO** |

`EROSION.assumptions = ["stream-power `m`", "erosion `k_dt`", "erosion run length"]` — **three, and it
leans on at least five.** `WATER.assumptions = ["bounded-fill acceleration", "atmosphere store",
"water fill steps", "SEA_LEVEL_M"]` — **four, and it leans on at least eight.**

**87/87 green. The test cannot fail on any of this**, because it only walks the arrow the wrong way.

> ### ⇒ **The audit spent a night finding six undeclared magic constants — and the mechanism whose stated purpose is to make undeclared magic constants discoverable did not, and structurally could not, find one of them.**

*(And the fix is a graph query — a **type-A** falsifier, therefore free. It needs one thing the code
does not have: a way to know which constants a kernel actually touches. That is either a hand-written
`consumes_constants` list, or — better, and it is what §5's "nomos-version auto-derived from kernel
source" was already reaching for — **derived from the source itself.** The two unbuilt items are the
same item.)*

### The ledger has grown a DEMOTE operator. It is made of typography.

I have been tracking a missing retraction primitive for four documents. **`ASSUMPTIONS.md` needed one
so badly that it invented one out of markdown:**

- `⛔ **CORRECTED 2026-07-13 — I had this BACKWARDS**` (the `P` row)
- `⛔ **ADDED 2026-07-13**` (four new rows)
- `| ~~Jarrett roughness, de Almeida–Bates stabilization~~ | — | ⛔ **MOVED + CORRECTED 2026-07-13** |`
  — **a struck-through row**, retained as a tombstone, pointing at its replacements
- `**RETIRED on the frame path (2026-07-12)**` (the rain-rate row)

**Strikethrough. Emoji. Prose.** The `status` column has an implicit vocabulary — *arbitrary · tuned ·
literature · earth-ref · missing · NOT enforced · ✅ ACQUITTED* — and **no value for `refuted`**, so
the ledger says it with `⛔` and a bold sentence.

> **The need for a demote operator is not my inference. It is empirically demonstrated by a ledger
> that built one out of formatting because the schema would not give it one.** *(And udon-core is the
> thing that turns those into fields.)*

### And the ledger has become the OVERFLOW BUFFER for the missing `NomosDecl` fields.

Look at what got added on 07-13 and ask *why it is in a magic-constant ledger at all*:

**A whole new section — `## Structures the code preserves EXACTLY (assets — declare them, and do not
break them by accident)`:**

> *"Added 2026-07-13. **These were found by probing, are not written down anywhere, and a well-meaning
> scheme rewrite could destroy them without noticing.**"*
>
> - **Well-balancedness** — ✅ *"ACQUITTED, EXACTLY. Flux = 0.000e0 bit-exactly… **[D] Structural, not
>   luck**… it **NEVER SPLITS pressure-gradient from bed-slope — and that split IS the classic failure
>   mode**."*
> - **Staggering (Arakawa-C)** — ✅ *"ALREADY TRUE, and undeclared… **`water.rs` has NO null space**…
>   a load-bearing asset that a 'staggering fix' could destroy by accident."*
> - **Acyclicity** — ✅ *"Enforced by the strictly-downhill clamp — **and never named**… **it IS the
>   discrete Lyapunov condition.** Declare it and assert the flow graph is a DAG."*
> - **rock-mass conservation** — ⛔ *"NOT enforced."*

**That is box ④ — preserved / sacrificed structure — arriving in the magic-constant ledger, in prose,
because there is nowhere else to put it.** And the `θ` and `Jarrett` rows are not constants either:
they are **full Prime-Question analyses** (*"Jarrett 1984 is a REGRESSION for ESTIMATING `n` from a
MEASURED slope — it is not a dynamical law"*). **That is box ⑤.**

> ### ⇒ **`ASSUMPTIONS.md` is where declarations go when the data model has no field for them. It is `NomosDecl`'s overflow buffer, and it is now carrying two of the four missing boxes in markdown.**
>
> **That is a much better argument for the rebuild than any amount of theory**: the fields are not
> hypothetical, and nobody needs to be persuaded they are needed. **They already exist. They are just
> in the wrong file, in the wrong format, unenforceable.** The rebuild is a *migration*, not an
> invention.

---

## 2. What matters for today's session

**The scaffold's four unbuilt enforcement mechanisms are now all the same shape, and all four are
type-A (graph queries — cheap, static, free once udon-core lands):**

| the check | today | what it would catch |
|---|---|---|
| `consumed ⟹ declared` (constants) | ⛔ the arrow is only checked backwards | **all six** of the audit's undeclared constants |
| `|rel :to X ⟹ term[X] exists` (LEXICON) | ⛔ no check | the 2 dangling edges (`manifest`, `nomotheke`) |
| `refuted ⟹ nothing consumes it` (the **defeasance / void-condition**) | ⛔ **no `refuted` state exists** | **`p = 1.1`, the morning after it was refuted** |
| `declared_D high ∧ derived_B low ⟹ VIBE` (LEXICON §5's own firewall) | ⛔ never written | **`erosion.rs`: 87/87 green on a fake erodibility field** |

**Four checks. All free. None built. And every one of them would have fired on something the audit
spent a night finding by hand.**

---

## 3. Wandering thoughts

**The ledger is the most honest document in the project and it is *therefore* the most damning.**

Read the `SEA_LEVEL_M` row. It does not say "arbitrary, TODO." It says: *"**arbitrary — and it is
MANUFACTURING FORBIDDEN LAND**,"* then cites the probe, the measurement, the ordinum charge it
violates, the phase-structural diagnosis, and the principling path. **The row knows more about the
bug than most codebases know about their features.** And it has known since 07-12. And `gen.rs` still
decrees 4000 m.

**Knowing is not the constraint. It has never been the constraint.** This project generates true
statements at a rate I have not seen before, and it *cannot stop itself acting on false ones*, because
**truth here is stored in prose and consumed by people, while falsehood is stored in code and consumed
by the machine.** The asymmetry is total, and the two ledgers that *are* compiled in — this one and
the ordinum — are the only places the machine and the truth touch at all. **And this one is compiled
in through a substring check that walks the arrow backwards.**

⇒ **So the whole rebuild reduces to one sentence, and I think it is the sentence:**

> ### **MOVE THE TRUTH TO WHERE THE MACHINE READS.**
>
> Not "write better docs." Not "add fields." The docs are *superb*. **The machine cannot read them,
> and the one place it can, it reads one-way.** udon-core is the thing that makes the machine able to
> read; the probe↔declaration binding is the thing that makes it able to *check*; and defeasance is
> the thing that lets it *forget what stopped being true*.

---

**A smaller thing, and it is the most quietly alarming line in the file.**

> **`| water-surface float precision |`** … *"the residual current is **LINEAR IN ULP**… A **conditioning
> defect, not a scheme imbalance**… ⚠ **Open harm question (cheap, unrun):** sediment capacity is
> `C = k·|v|·slope`… a nonzero `|v|` on a sloping lake floor is a nonzero **erosion rate** — and each
> pipe is one-way (`.max(0.0)`), so **frozen round-off could RECTIFY INTO NET BED TRANSPORT**."*

**A floating-point rounding error, made one-directional by a `.max(0.0)`, becomes a physical process.**
The lake is dead calm; the scheme is *exactly* well-balanced (proven, bit-exact, at every refinement);
and f32 at a 4000 m datum forges a 14 m/day current that never decays — and the sediment law, being
sign-definite, would *integrate* it.

**This is the LEXICON's axis-D arithmetic clause** — *"an increment that must land wants compensated
summation"* — **firing exactly as written, months later, in the only place nobody was looking.** And
it is unrun. *(It is also, I note, the cleanest possible specimen of the governing principle: the
scheme preserves its structure **exactly**, and the *representation* destroys it anyway. **Structure-
preservation is not enough if the numbers cannot hold the structure.** That is a row the structure
table does not have.)*

---

**And one genuine pleasure.** The well-balancedness row ends:

> *"⚠ **Do NOT cite de Almeida & Bates for this — they discuss well-balancedness nowhere. It is ours
> to state.**"*

A ledger entry that stops you from *over-citing* — from laundering your own result into the
literature's authority. I have not seen that before either. **The same discipline that refuses to
claim `Kept` without a probe refuses to claim a paper said something it didn't.** It is the same
virtue, and it is the project's best one.

---

## Tactical residue — quarantined

- Six constants added to the ledger on 07-13; **zero added to any nomos's `assumptions:` anchor list.**
- `EROSION` declares 3 anchors, leans on ≥5. `WATER` declares 4, leans on ≥8.
- The ledger's own §"Process assumptions" already names its weakest link: *"Manual nomos-version
  constants — retired by source-derived versions."* **Source-derived versions and `consumed ⟹ declared`
  are the same unbuilt thing.**

## Queue changes
**Next:** `TODO.md` (37 K, touched 07-13) — read late, deliberately, so I can judge which items the
audit voided. Then the `doc/design/` tree (all 07-11, all pre-audit).
