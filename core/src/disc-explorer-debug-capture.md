---
slug: disc-explorer-debug-capture
type: discussion
status: discussion-grade
stage: draft
depends:
  - disc-explorer-instrument-parity
  - disc-explorer-human-chrome
  - form-core-view-wall
  - norm-no-depiction-without-referent
  - ops-changelog-is-the-acceptance-check
  - form-store-as-save
  - form-fidelity-ladder
---

# Explorer debug dump and capture pairs — durable truth for handoff, not on-globe prose

Full instrument state (census, tiers, keys, pick, camera, logs) must remain **available** at arbitrary depth for humans and agents. It must **not** be the default on-globe surface ( #disc-explorer-human-chrome ). This segment owns the **debug dump** as a product object and the **capture pair** ideal that replaces "read the HUD / scroll stderr."

## Formal Expression

1. **Problem.** Today the dense explore overlay *is* the debug panel, always on. Startup stderr and `vivarium status` carry overlapping truth in forms no operator glances at. When Joseph says "check the last two captures," there is no conventional pair; when an agent needs hashes and tier fractions, the only path is chat paste or log archaeology.

2. **Capture pair (target product).** On explicit action (key) — and optionally auto on high-severity transitions later — write a **dated pair**:

   | Artifact | Role |
   |---|---|
   | `…/captures/<stamp>-vivarium-info.v0.N.N.udon` | Structured dump: seed, world path, world-git short, binary `SRC_HASH`, fresh/stale census, demand (incl. beacon), lens/paint/level, centre + pick, tier fractions, sea provenance, short notice/log tail, path to sibling image |
   | `…/captures/<stamp>.png` (or same stem) | Screenshot of the primary window after the dump is armed (existing sighting pattern: image one frame later so the toast is not required in-frame) |

   **Directory resolution (ideal):** `$VIVARIUM_CAPTURES` → manifest field if present → default `<world-dir>/captures/`. Not a store citizen ( #form-core-view-wall ): observations about a world are not part of that world's memoized law (same discipline as sightings).

3. **Social protocol.** "Look at the last two captures" means: list `captures/` by mtime, open the two newest udon+png pairs. Agent depth is optional; human may only need the PNG. This is the **affordance** stdout cannot be.

4. **Present prototype: sightings.** `crates/vivarium-explore/src/sighting.rs` already implements **key → markdown dump + next-frame PNG** under `<world>/sightings/` (or `$VIVARIUM_SIGHTINGS`). Epistemic role is correct: observation, blank "what looked wrong," not a claim segment. **Gaps vs target:** ad hoc markdown not versioned udon; path name `sightings/` not `captures/`; no schema version; no full session log; on-globe dump not retired.

5. **Relation to changelog captures.** #ops-changelog-is-the-acceptance-check FE(7) uses `changelog/captures/…` for **landing** evidence in the **source** repo (embeds, frozen entry). Live explore captures are **session / world-adjacent** debug and handoff. They may be **promoted** into a changelog entry; they must not be conflated with store roots or required for every idle orbit. One mental model, two lifecycles: *session capture* vs *landing capture*.

6. **What the dump must be able to answer (v0 schema checklist).**

   - Can this binary show carved land? (fresh/stale)
   - What did the operator demand? (level, beacon, epochs)
   - What is on screen? (lens, paint, mesh level, whole vs close-in)
   - Where is the camera / pick? (face, i, j, lat/lon, elev, water)
   - Which fidelity tiers covered the frame? (tier map + FE(8) note: mesh is `surface_at_carved`)
   - Provenance pins: seed, `SRC_HASH`, world-git short if available
   - Sibling screenshot path

7. **Retire default on-globe debug dump.** After capture pairs work:

   - Default view: human chrome only ( #disc-explorer-human-chrome ).
   - Full dump: capture key and/or `--debug-hud` / `H` cycle that **explicitly** enables the old panel.
   - Startup: at most **one** high-severity line if fresh=0; full census only in capture or status lead block.

8. **CLI.** `vivarium status` should emit the same **lead block** fields as the capture header (demand, src, fresh/stale, next action) so agents without a running explore still get glance-grade state. Pyramid remains secondary.

9. **Out of bounds.** Auto-rebuild on stale; store-writing captures; requiring udon for every changelog landing; redesign of paint shaders. Fidelity *accessor* law stays #form-fidelity-ladder .

## Epistemic Status

**Max attainable: discussion-grade** until a versioned capture schema is implemented and used for real handoffs (agent "last two captures" works without chat paste).

**Currently `discussion-grade` / stage `draft`.** Unifies Joseph 2026-07-31 direction (dump→udon+png captures; retire debug HUD; human chrome separate) with existing sightings + changelog capture practice. Not ratified as sealed product law. No implementation claim beyond "sightings prototype exists."

## Discussion

Sightings already encode the epistemic insight: the eye fires *in the moment*; durable dump preserves the referent. The failure is product: we kept the live dump as the primary UI instead of the capture path. Completing capture and thinning chrome is integration of that insight, not a new philosophy.

Versioned udon matters for agents: markdown bodies drift; a `v0.N.N` schema can grow fields without breaking "open the pair."

## Working Notes

- **Code:** `sighting.rs`, `main.rs` (PendingShot, autoshot `VIVARIUM_SHOT`), `hud.rs` (dump to relegate), `observe::eroded_region_census`.
- **Build order:** promote sighting → `captures/` + udon schema v0.1.0 → default-off dump → status lead block shared with capture header.
- **Do not** invent a second parallel "screenshot tool" beside sightings — **evolve** the sighting path.
- Companion: #disc-explorer-human-chrome (glance); #disc-explorer-instrument-parity (P0–P4 bar); #ops-changelog-is-the-acceptance-check (landing captures).
