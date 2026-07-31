---
slug: disc-explorer-human-chrome
type: discussion
status: discussion-grade
stage: draft
depends:
  - disc-explorer-instrument-parity
  - disc-explorer-debug-capture
  - form-core-view-wall
  - norm-no-depiction-without-referent
  - form-fidelity-ladder
  - form-store-as-save
---

# Explorer human chrome — glanceable status, not a debug panel on the planet

The live block of text over `vivarium explore` is not a **HUD** in the product sense. It is a **dynamic debug dump** that reflows, competes with the globe, and fails human attention even when its *content* is truthful. Instrument honesty requires a channel a human can **register without reading a paragraph** ( #disc-explorer-instrument-parity ; Joseph 2026-07-31: no natural affordance to grab attention in dense stdout/HUD/`status` output). This segment owns the **human-facing chrome** ideal and present inventory; the durable full dump lives at #disc-explorer-debug-capture .

## Formal Expression

1. **Two channels, one instrument.** The explorer is the spot-check instrument for world law. It needs:
   - **Chrome** — stable, high-contrast, glanceable state (this segment).
   - **Capture** — on-demand durable dump + image for human/agent handoff ( #disc-explorer-debug-capture ).
   - **Depiction** — mesh and paints bound by #form-core-view-wall and #norm-no-depiction-without-referent (not redesigned here).

   Mixing dump text into the permanent on-globe surface **occludes** both glance and depiction.

2. **What "HUD" means here.** **Human chrome** is a small set of **status chips** (and optional short sticky banners for high-severity conditions only). It is not the full fidelity census, not the unmodelled list, not the complete paint/key legend. Those remain **available** via capture and (temporarily) via a debug mode, not via default on-screen essay.

3. **Attention is part of instrument honesty.** A truth that only exists in low-contrast rewrapping prose is not "said" for a human operator. The same bar as #norm-no-depiction-without-referent FE(4) on caveats: if the operator is matching patterns on the globe, they are **not** reading a panel. Critical state (especially **fresh=0 under this binary**) must be **structurally self-announcing** in chrome, not only present in a wall of text.

4. **Present inventory (code, 2026-07-31) — map, not endorsement.**

   | Affordance | Role today | Chrome-class? |
   |---|---|---|
   | Dense multi-line overlay (`hud.rs`, `H` cycles opacity/detail) | Full frame facts, tier line, paint, water, unmodelled | **Debug dump** (default-on) |
   | `G` / `B` / `E` | Cycle erosion cohort; frame cohort centre; scrub stages | **Yes** — findability of fine chains |
   | Paint keys `1`–`6`, `Tab` | Surface / provenance / water / seam / change / depression | **Yes** — mode chips candidates |
   | `[` `]` / `A` | Manual / auto mesh level | Mode chip candidate |
   | Startup stderr fresh/stale line | `eroded_region_census` under `SRC_HASH` | Truthful but **not glanceable** |
   | HUD stale-only tile clause | Partial freshness | Buried in dump |
   | `vivarium status` pyramid | Archive census by level/nomos | **Not** "can this binary show carve?" |
   | Sighting key → md+png | Prototype capture pair | #disc-explorer-debug-capture |

5. **Target human chrome (v1 ideal — not sealed law).** Stable strip or corner chips, same layout every frame:

   | Chip | Content | Severity |
   |---|---|---|
   | **CARVE** | `fresh N · stale M` under this `src=` (short hash) | **Red / unmissable when N=0 and M>0** |
   | **VIEW** | mesh level · WHOLE / CLOSE-IN | Always |
   | **SURFACE** | carve / prior / mixed (tier summary one token) | Always |
   | **BEACON** | demand beacon present? face+level or `none` | Always |
   | **PAINT** | current paint mode | Always |
   | **REBUILD?** | only when fresh=0 & store has stale erosion | Sticky until dismissed or rebuild |

   Optional: short notice toast on capture write (3–5 s). **No** unmodelled essay, no full tier prose, no rewrapping multi-paragraph body on default chrome.

6. **Findability stays first-class chrome, not dump.** Fine cohorts are ~10⁻³ of globe area. **B** framing from store census (not typed coords) and **G** cohort cycle remain product keys. Ideal: chip or icon when selected cohort is off-screen ("L13 window — press B").

7. **CLI companion (one glance block).** `vivarium status` should **lead** with the same human question as the CARVE chip — demand one-liner, `src=` short, fresh/stale counts, next action — **before** the fidelity pyramid. Pyramid remains useful archaeology; it must not be the only surface. Detail of CLI layout may co-live with #disc-explorer-debug-capture FE on capture schema if status dumps the same struct.

8. **Out of bounds for this segment.** Paint-law physics, FOV/mesh policy as world law, full keybinding redesign, game HUD aesthetics, daemon/spool. Depiction accessor law is #form-fidelity-ladder FE(8). Parity *bar* remains #disc-explorer-instrument-parity .

## Epistemic Status

**Max attainable: discussion-grade** until a shipped chrome build is used for real sessions and the chip set is measured against missed staleness / lost-beacon incidents.

**Currently `discussion-grade` / stage `draft`.** Sealed only as **problem framing + inventory + target chip set** from Joseph 2026-07-31 (attention failure; dump-vs-chrome split; status inscrutable). Not Joseph-ratified as product law. No implementation claim.

## Discussion

P0–P4 made thrash and open-view policy workable enough that the **next** instrument failure is attention, not pull big-O. A correct fresh/stale number that no human sees is the same class of defect as a correct ocean mask never painted: the channel is dead.

Human chrome is deliberately **smaller** than the debug dump. Completeness lives in capture pairs. The product error is treating completeness as an on-globe obligation.

## Working Notes

- **Code homes:** `crates/vivarium-explore/` (`hud.rs`, `main.rs` keys, `sighting.rs`); `watch::ErosionCohort`; `observe::eroded_region_census`.
- **Order of build (ideal):** (1) capture pair promotion — #disc-explorer-debug-capture ; (2) default chrome = chips, dump off or `H`/flag only ; (3) status lead block shares CARVE chip data.
- **First iteration shipped 2026-07-31 (awaiting Joseph check):** default `hud_level=0` chips (CARVE / VIEW / SURFACE / BEACON / pick); `H` cycles human→debug dump→minimal; `vivarium status` lead block fresh/stale/REBUILD/beacon before pyramid.
- **Do not** "fix" attention by lengthening the dump. **Do not** auto-rebuild on stale without an explicit policy decision.
- **Hotlist:** ideation rank for chrome/capture lives in #disc-known-active-hotspots when Joseph re-prioritizes after this segment lands.
