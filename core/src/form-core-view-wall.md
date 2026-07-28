---
slug: form-core-view-wall
type: formulation
status: exact
stage: draft
depends: []
---

# The core / view wall

The world frame never depends on a renderer. Every consumer of the world is a peer that only queries. A view observes; it does not author world-evolution parameters.

## Formal Expression

1. **No reverse dependency.** The world frame (`vivarium-world` and, by the same law, any successor core) has no dependency on rendering, windowing, or view-local types.
2. **Query-only seam.** Views obtain world state only through the sanctioned query path over the store (or an equivalent pure query API). A view may hold camera state, meshes, and HUD state; it may not own authoritative world state.
3. **Peer views.** Human renderer, logozoetic interface, headless logger, ASCII instruments — siblings, never a tower in which one view is the only gateway to the world.
4. **Observe-only evolution.** A view does not expose knobs that choose how the world evolves (for example, how many erosion epochs to run). World-evolution parameters are authored by builder / law / manifest paths, not by the explorer. (`DECISIONS[core-view-wall-observe-only]`, `:by us`, decided.)

5. **Observation density is not authorship.** A view may **evaluate declared law at points the builder never materialized** — and that is not a violation of FE(4), because law evaluation is a pure function of keyed inputs ( #post-determinism-as-ontology ) that writes nothing and changes nothing about what the world *is*. The distinction is between choosing *how the world evolves* (forbidden) and choosing *how densely one looks at it* (proper view business, and the view's own concern alone).

   The live instance, which is what makes the line worth drawing: `vivarium explore` samples the mantle-thermal cooling law at 120 stages while the manifest's `frames` demand may be 60. Both call the same law; the view's ladder is a strict superset of the builder's by nested bisection ( #form-time-indexed-stage-chains FE(9) ), so every materialized stage appears in it bit-exactly and the extra stages are real law evaluations that land nowhere. Read literally and without this clause, FE(4) would condemn that as a view-owned evolution knob and it is not one — deleting it would cost resolution and buy no compliance.

   Two conditions keep it on this side of the line, and both are load-bearing: the sampled quantity must be **pure law** (not a mutation, not a solver run that would author state), and the view must **distinguish, in what it shows, which samples are store citizens and which it evaluated for itself** ( #norm-no-depiction-without-referent : an evaluated-but-unbuilt stage has a referent — the law — but not the same referent as a built one, and conflating them would misreport what the builder has done).

6. **The wall is a handle, not a habit.** `Store::open_read_only` serves reads and refuses puts with `PermissionDenied`, counting each refusal. A view holding that handle *cannot* author a citizen, so FE(1)–(2) are enforced by the type a process is given rather than by the care of whoever wrote it, and the count is observable rather than asserted (the explorer surfaces its own refused-write count). This is the mechanization #form-store-as-save FE(8) named as compliance debt, for the view half.

## Epistemic Status

**Max attainable: exact** as architecture law — falsified by a rendering dependency into the world crate, or by a view-owned evolution parameter that changes what the world *is*. The law is founding (DESIGN.md; `DECISIONS[core-view-wall-observe-only]`, `:by us`). Stage `draft`.

**Enforcement (2026-07-24).** FE(1) holds by dependency direction (no rendering crate is reachable from the world frame, and the workspace manifest states it as a rule). FE(2) and FE(6) hold by **mechanism**: the sole view crate, `crates/vivarium-explore`, opens the store read-only, and `status` / `info` / `watch` do the same — which also closed a latent violation on those paths, where `globe::render` reached `erosion_tile` and would have *computed and written* on a miss. Two tests convict the handle.

**Known incomplete surface (compliance debt, not a soften of the law):**

1. **Unaudited views.** `crates/vivarium-explore` enumerates its own unreal affordances and holds a read-only store. `globe::render` — the ASCII projection behind `watch` / `info` — has not been audited against #norm-no-depiction-without-referent , though it now shares the read-only handle.
2. **Read-path silence, not a wall breach but adjacent.** A view asking coarser than the build level silently receives the uncarved prior ( #obs-coarse-view-draws-the-uncarved-prior ). The view authors nothing, so FE(2) is intact; what is missing is a report at the query boundary, which leaves every consumer to detect it alone.
3. **Store-backed navigation** — a true ethereal explorer that upgrades continuously as builder memos land, rather than reading a census at open — remains a gap, and is the surviving half of the original three.

*Retired from this list 2026-07-24, because the surfaces they described no longer exist:* the `spikes/worldview` opt-in hybrid and its `VIVARIUM_ALLOW_VIEW_EVOLUTION` waiver (that crate is archived; the flag is gone with it). History in `DECISIONS[the-explorer-is-an-instrument-and-the-wall-is-a-handle]`.

## Discussion

Headless calibration, human play, and logozoetic play-as-oneself are the same world under different peers. If the sim grows renderer tendrils, the other peers become second-class and *in vivia* citation becomes "whatever that UI did." Observe-only evolution is what keeps an ethereal explorer ethereal ( #scope-moratorium-endogenous-emergence is a consequence gloss, not a well-typing prior of this wall).

## Working Notes

- Worldview gate: `VIVARIUM_ALLOW_VIEW_EVOLUTION` (default off). Module docs + Cargo.toml comment name the split.
- **Store observe path (landed):** `World::load_eroded_regions` + `assemble_surface_tile`; globe and worldview default paths consume builder 64×64 tiles (not full-face keys). Seed from vivium manifest via `$VIVARIUM_WORLD`.
- Next strengthen: demand spool; water-tile depth load into observe path; periodic store reload while a builder runs; clippy bans for rendering crates in world packages.
- Specimen of FE(4) violation and revert: `DECISIONS[core-view-wall-observe-only]` (history layer).
