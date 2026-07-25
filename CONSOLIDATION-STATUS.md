# Consolidation status — big-picture intuition

*Body replaced 2026-07-24 evening (its own protocol: replace when the picture changes a lot — it did). **Not claim canon.** When this file disagrees with `core/`, core wins.*

## The picture, in one paragraph

The claim channel is **85 segments** and is in good order — zero orphans, zero dangling segment or decision references, every outline row backed by a file. The **prose mine is mostly worked**, and the earlier "closed" was an overstatement corrected below. What changed today is *where the frontier is*: the physics has been running ahead of a runtime that cannot yet support iterating on it. That is now the named work.

## The prose mine, honestly

The design/theory thick middle is adjudicated into `core/` and graduated. **Law-shaped material survived well and in several places was strengthened.** One class thinned: time, progress, build-sequence, and the observer's experience. Full audit — five independent readers, every load-bearing claim re-verified against the primary — at `msc/harness-reconciliation-2026-07-24/FINDINGS.md`.

**The instruction that used to stand here — "do not re-mine ice for claims" — is withdrawn.** It was the sentence that would have prevented the audit that found the gap. Re-mining is cheap; an archive believed exhausted is one nobody checks. Note where the fault was *not*: `#scope-segment-canon` FE(4) is a prohibition, not an assertion, and its Epistemic Status explicitly disclaims the completeness reading. **Core never overclaimed. This file did.**

## What today established (trust these; they are measured)

- **The store and memoization are real and correct.** Rebuild-with-no-changes is 24/24 hits in 8 ms. Build-order independence and resume-equals-run-through are now **convicted** on the epoch chain (`#form-depend-by-key-never-latest`) — that law had sat at `status: exact` with no instrument since it was written.
- **The daemon layer is essentially unbuilt.** No beacons, no demand spool, no cones, no scheduling beyond a breadth-first six-face sweep the CLI itself calls "the degenerate beacon." Every *prerequisite* it needs — store-as-bus, admission, observe-only pull, never-block — is built and working. It is one layer, not a rewrite.
- **Component E is the object under all four asks** (background building toward somewhere, restarting at the right point, watching the edge, watching replays). They reduce to time-indices in keys with $\varepsilon$ per stage, plus a poller over roots. Claim home: `#form-time-indexed-stage-chains`. Replay and live-watching are *one* mechanism; built as two features they become two and will disagree.
- **Finer source-digest attribution is a measured no-go** — it under-keys on present code structure and buys 0 % on the most-edited files. The live constraint is module **separation**, not attribution derivability. Do not re-open blind.
- **The water tile does not settle** — 40 s of world time at every level, residual growing, because `stable_dt`'s clamp ceiling binds at kilometre cells (`#obs-water-fill-never-settles`). This **blocks** the convergence-$\varepsilon$ rung on water: a gate there would certify a transient as converged.

## Open work, in rough order of what unblocks what

1. **The step-size question** (physics, Joseph's or a spike's) — whether `stable_dt`'s ceiling should scale with cell size. Blocks water's $\varepsilon$ rung; has structure-preservation consequences.
2. **Component E's build** — partly landed, partly refuted. **Landed:** the reader (`vivarium watch`, live and replay through one path) and the nested densification that makes a chain's density a demand parameter (`--frames`, measured to reuse memos exactly). **Refuted:** that a stage's $\varepsilon$ can be had by giving a kernel a residual tolerance. Both live relaxation kernels are now measured no-gos — water's step is pinned ( `#obs-water-fill-never-settles` ), erosion's residual is driver-bound and most tiles do no fluvial work at all ( `#obs-erosion-residual-is-driver-bound` ). Nothing in the tree records an $\varepsilon$ yet and no kernel can honestly be gated into recording one; the open work is the *form* of each criterion, not its plumbing.
3. **The demand spool** — beacons → cones → work queue. Recipe already primary-read (`#detail-seam-precedents` FE(3): cluster, oriented rectangles, efficiency ratio, clusters change slowly); cone shape is the drainage island, not a halo. **Regime G is the floor**: global aspects advance time-uniformly and need no beacon, so "no demand" means *spine-only*, not idle.
4. **The live edge** — **built**. The poller is `vivarium watch`; the physics tier (declared and derived) and the provisional flag now reach the viewer. What it can honestly order by is root *landing* time — build history, not world-time — and closing that gap is item 2's business, not the reader's. Still unsurfaced: statistic/exactness and the ordinum maturity rung, which `vivarium status` shows and the watcher does not.
5. Standing, unowned: the stale-`src` GC; `DECISIONS` has no owner and its `tooling`/`view` topics are uncited by core; `ASSUMPTIONS.md` owes a register scrub and its own retirement to generated udon.

## The rhythm this file protects

Claim or named gap in core → convict where possible → strengthen before soften → demote rival prose → name residuals honestly. Session success is a moved segment or a convicting instrument, not an update to this file.

One lesson from today worth keeping in the rhythm: **the artifacts record what was decided; the session transcripts record why**, including options weighed and declined that reach no file. An audit that reads only artifacts will mistake considered trade-offs for oversights — this one did, for about an hour, until Joseph pointed at the transcripts.

*Last intuition pass: 2026-07-24 (evening).*
