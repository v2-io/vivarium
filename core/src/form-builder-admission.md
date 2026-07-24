---
slug: form-builder-admission
type: formulation
status: exact
stage: draft
depends:
  - form-flux-web
  - form-core-view-wall
  - form-complete-content-addressed-key
  - form-store-as-save
  - form-nomotheke-registry
  - norm-declaration-must-convict
---

# Builder admission and explorer pull

The builder may materialize only what the flux contract allows (or explicitly waives). Explorers never invent world evolution; they show what the store already holds.

## Formal Expression

1. **Two roles.** **Builder** advances a vivium by computing and putting memos under complete keys ( #form-complete-content-addressed-key · #form-store-as-save ). **Explorer / view** queries the store and never authors world-evolution parameters ( #form-core-view-wall ).
2. **Admission.** Before materializing a phase whose nomos has a requisite chain, the builder evaluates that chain against the nomotheke ( #form-flux-web · #form-nomotheke-registry ). If any quantity is **Unmet**, default admission **refuses** the phase — the world is mechanically unrunnable for that work, not "mostly fine."
3. **Waiver is named.** Exploratory materialization under unmet needs requires an explicit override (present practice: `--allow-unmet`). The waiver must be **loud in the build log**; silence is not an override. Waived artifacts are **provisional** and must not be cited as lawful *in vivia* evidence until needs are met or a producer exists.
4. **Observe-only pull.** View-facing surface queries prefer a store Hit of matured work (e.g. eroded tile) and otherwise fall back to instant prior / coarser lawful data — they do **not** cold-trigger long evolution computes that the builder has not done. Single-key form: `surface_prefer_eroded`. Builder-sweep form (many 64×64 tiles): `load_eroded_regions` + `assemble_surface_tile` / `erosion::surface_at` — used by globe and default worldview.
5. **Single-builder discipline (practice).** Concurrent builders on one vivium directory attach or yield rather than double-write scheduling state; truth remains content-addressed objects, not the lockfile. Lock mechanics are engineering; the law is that **scheduling is not a second truth channel**.

## Epistemic Status

**Max attainable: exact** for admission + observe-only pull as architecture under the flux web and core/view wall. Live: `bin/vivarium.rs` phase admission via `audit::requisite_chain` (refuse exit 2; waiver log line); waived phases set `World::set_provisional_writes` so roots carry the third-line `provisional` flag, `vivarium status` prints the counts, and the Hit path surfaces `Source::HitProvisional` (waived bytes cannot launder into a lawful Hit); `query::World::surface_prefer_eroded`. Both the refuse path and the waiver→provisional→`status` chain are convicted at the binary boundary by `tests/cli_admission.rs` (argv → process → exit-code / stdout), and the lib walks put-provisional → census → HitProvisional. Stage `draft`.

**Known incomplete (open — do not soft-close):**

1. **Query `put` errors are swallowed** (`let _ =` on memo puts) — surfacing them is an API-shape change, deferred; a failed memo put today degrades to recompute, never wrong bytes.
2. **Full builder daemon** (beacon cones, demand spool, restart-in-place) is design-grade in #detail-builder-daemon , not this segment's built surface. Do not read FE(1)–FE(4) as claiming that design is shipped.
3. **Lock TOCTOU / atomic create** still engineering debt.
4. Worldview spike still hybrid-evolves on explorer paths — compliance debt on #form-core-view-wall , not a license to dissolve FE(4).
5. **First-light blocks on a cold heavyweight compute (never-block violation, 2026-07-24).** FE(4) says a view falls back to instant prior / coarser data and does **not** block. But the globe worker computes `sea_level::derived_sea_level_m` — an L8 pour of ~393k `lithosphere::freeboard_m` (isostasy) evaluations — **synchronously before the first frame**, and each per-request assemble also runs a full-face census pass. Since today's isostasy read made `freeboard_m` expensive, first light can hang for minutes at "pulling surface from store…" (only the sun HUD, which renders independently, shows) — worse under store/CPU contention with a running builder or probe. This is a cold *compute* on the critical path, not a store pull, so it is a real never-block breach. **Recommended fix (owed, borders `sea_level`/builder so flagged not churned mid-isostasy-work):** render the prior/coarse surface immediately with a provisional coarse-level sea, refine the L8 pour asynchronously and rebuild; and/or memoize the derived sea level into the store at `vivarium build` time (store-as-save) so the globe reads it instantly instead of re-pouring per launch.

## Discussion

Status can print "unmet" while a naive builder still writes eroded tiles — that split was the de-novo P0: instruments told the truth and the materializer ignored it. Admission closes the split on the default path. The wall then says explorers must not re-open it by running epochs for a prettier frame.

## Working Notes

- Store sibling #form-store-as-save ; unbuilt daemon #detail-builder-daemon .
- **Dual homes demoted carefully:** `#detail-builder-daemon` header (admission + observe-only + store bus); `query.rs` observe-only surface; abyssal-parity explorer roles. Unbuilt daemon / beacons / demand spool remain plan design — not claimed by this segment.
- Sibling on #form-ordinum-governs-flux-web (the maturity report `vivarium status` now renders) — different surface, same "show the convicting truth" telos.
