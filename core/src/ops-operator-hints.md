---
slug: ops-operator-hints
type: discussion
status: discussion-grade
stage: draft
depends:
  - norm-caught-disciplines-become-mechanisms
  - form-core-view-wall
  - disc-explorer-instrument-parity
---

# Operator hints — suggestions with origins, not a next-action queue

When the instrument knows a recovery step (rebuild under this `src`, zoom for covering grain, open a beacon), it must **suggest** it without sounding like a job queue, a pipeline phase, or something the binary is about to do itself. That speech act is an **operator hint**. Hints are the same *class* of honesty surface as `bin/provenance` / `bin/check-provenance`: a value that is cheap to invent is required to carry an **origin**, and the log must make bad inject logic greppable.

This segment owns the **hint channel** (register, edge log, stack, capture field). Where hints *appear* on the globe is #disc-explorer-human-chrome ; what a capture pair must record is #disc-explorer-debug-capture ; converting inventability failures into mechanisms is #norm-caught-disciplines-become-mechanisms .

## Formal Expression

1. **Speech act.** A hint is a **suggestion to the operator** (human or agent reading status / capture / stderr). It is **not**:
   - a declaration of what the process will do next;
   - a workflow engine (`demand` → `build` → `explore`);
   - a phase ladder step;
   - an ordered “next CLI verb” for any reason other than recovery under a named condition.

   Ambiguous labels such as `next:` / `next_action` are **retired** for this role. Surface wording is `Hint: …`.

2. **Inject like a log line.** Any call site may **set** or **revoke** a hint by stable `id` when a condition rises or falls — same shape as emitting instrumentation, not a single hard-coded three-way string copied into status, explore, and capture. Multiple active, unrevoked hints **stack** (order: stable by id unless a later product rule ranks severity).

3. **Three surfaces, one active set.**

   | Surface | What it shows |
   |---|---|
   | **Active set** (HUD chrome, `vivarium status` lead, capture `hints:`) | Current unrevoked texts — glanceable stack |
   | **Edge log** (CLI / stderr) | Only on **set** (new id or text change) and on **revocation** |
   | **Origin** | Every edge line carries `file:line` of the inject or clear site |

   Revocations are first-class log events (`[hint] revoked id @ file:line — was: …`), not silent drops. Spelling: *revocation* / *revoke*.

4. **Edge-triggered, not per-frame.** Calling `set` every frame with unchanged `(id, text)` must **not** re-log. Calling `clear` when the id is absent is a no-op. Otherwise explore’s draw loop would drown the only channel that makes origins useful.

5. **Status fact vs hint.** High-severity **facts** keep their own affordance (e.g. CLI `★ REBUILD NEEDED`, HUD `*** REBUILD NEEDED ***`). The hint under them is the **recovery suggestion** (e.g. `` `vivarium build` to rebuild under this program ``), not a replacement for the scream. Do not collapse “what is wrong” into “Hint:” alone.

6. **CLI vs Bevy glyph policy (present).** Terminal status may use unicode attention marks (`★`). Bevy HUD text is **ASCII-only** until a font path carries the glyphs (tofu was a render font failure, not a reason to strip CLI affordances).

7. **Capture schema.** Session captures record the **active** set with provenance, not a single string field:

   ```yaml
   hints:
     - id: "erosion-stale-src"
       text: "`vivarium build` to rebuild under this program"
       at: "vivarium.rs:878"
   ```

   Agents iterate on this list the same way they iterate on `src` / `erosion_fresh`. The obsolete capture field `next_action` is replaced by `hints:`.

8. **First wired site (erosion bed under this binary).** Shared helper `hint::sync_erosion_bed` keeps one of three mutual-exclusive ids live:

   | Condition | id | Hint text (present) |
   |---|---|---|
   | fresh=0, stale>0 | `erosion-stale-src` | `` `vivarium build` to rebuild under this program `` |
   | fresh=0, stale=0 | `erosion-none` | `` `vivarium build` to continue erosion-tile root builds `` |
   | fresh>0 | `erosion-readable` | `eroded land now visible: …` (soft; may later be silence-by-default) |

   Further inject sites (builder.lock live, wrong view level for beacon, demand gaps) earn their own ids the same way — they do not re-open a second ad hoc “next:” string.

9. **Family: inventability and origin.** A 16-hex digest cannot be *recalled* — only read or invented (`bin/provenance`, #norm-caught-disciplines-become-mechanisms specimen 2026-07-30). A recovery command *can* be invented as prose, and was: three copies of `next: vivarium build` with no shared origin. The fix is the same shape — **make the structured channel cheaper than free-form invention**, and attach **where it was injected** so a bad condition is greppable. Hints do not replace provenance digests; they apply the same honesty class to *operator suggestions*.

10. **Out of bounds.** Auto-running the hinted command; ranking a global “optimal next session goal”; storing hints as world citizens ( #form-core-view-wall ); requiring every soft-ok state to emit a positive hint forever (silence is allowed).

## Epistemic Status

**Max attainable: discussion-grade** as product/instrument design until more inject sites and real session use stress the stack (noise, false positives, missing revocations).

**Currently `discussion-grade` / stage `draft`.** Present-tense: module `vivarium_world::hint` (`set` / `clear` / `sync_erosion_bed`, edge log, HUD/status/capture wiring) shipped 2026-07-31 after Joseph rejected ambiguous `next:` / `next_action` and required revocations on the CLI log. Not Joseph-ratified as sealed ops law; wording of individual hints remains free to improve without a segment rewrite if the speech-act rules hold.

## Discussion

The failure mode that forced this channel was not “we forgot to tell the user to build.” It was **register**: the same recovery string looked like a pipeline step, a status machine, or ambient noise depending on surface. Naming the speech act *hint*, stacking active suggestions, and logging set **and** revoke with origins turns “what should I do?” into instrumentation instead of three hardcoded branches.

Chrome owns *glance*; capture owns *durable dump*; this segment owns *what a suggestion is* so those surfaces do not reinvent vocabulary.

## Working Notes

- **Code home:** `crates/vivarium-world/src/hint.rs` (process-global active set + edge log); consumers: `vivarium status` lead, explore startup + HUD, `capture::write` / `bed_status_block`.
- **Autoshot:** `VIVARIUM_SHOT` writes the same capture pair as `C` (including `hints:`).
- **Verified 2026-07-31 afternoon (post first-light rebuild under `src=1cce2aa4…`):** pre-rebuild status set `erosion-stale-src` + ★ REBUILD; post-rebuild status set `erosion-readable` only, **no** rebuild scream (`fresh 13368 · stale 118854`). Edge log origin `vivarium.rs:878`. Soft-ok hint may still be retired to silence if chrome-noisy.
- **Do not** grow a parallel “suggestions” table in explore only; new conditions call `hint::set` / `clear` (or a small sync helper) with a stable id.
- Companions: #disc-explorer-human-chrome · #disc-explorer-debug-capture · #norm-caught-disciplines-become-mechanisms (provenance specimen).
