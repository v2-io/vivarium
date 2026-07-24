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

**Max attainable: exact** for admission + observe-only pull as architecture under the flux web and core/view wall. Live: `bin/vivarium.rs` phase admission via `audit::requisite_chain` (refuse exit 2; waiver log line); `query::World::surface_prefer_eroded`. Stage `draft`.

**Known incomplete (open — do not soft-close):**

1. **Provisional tag (partially closed).** Waived phases set `World::set_provisional_writes`; roots gain a third-line `provisional` flag; `vivarium status` prints provisional counts (store `RootEntry` / `PutOpts`). Still open: no lib-level integration test of the end-to-end waive→census path; Hit path does not yet expose provisional in `Source`.
2. **No lib-level integration test** of the refuse path (gate lives in the binary).
3. **Full builder daemon** (beacon cones, demand spool, restart-in-place) is plan-grade (`doc/plan/builder-explorer-decoupling.md`), not this segment's built surface. Do not read FE(1)–FE(4) as claiming the full plan is shipped.
4. **Lock TOCTOU / atomic create** still engineering debt.
5. Worldview spike still hybrid-evolves on explorer paths — compliance debt on #form-core-view-wall , not a license to dissolve FE(4).

## Discussion

Status can print "unmet" while a naive builder still writes eroded tiles — that split was the de-novo P0: instruments told the truth and the materializer ignored it. Admission closes the split on the default path. The wall then says explorers must not re-open it by running epochs for a prettier frame.

## Working Notes

- **Promoted** from `msc/consolidation-wave-2026-07-21/draft-form-builder-admission.md`; store sibling fixed to #form-store-as-save (not `form-store-is-save`).
- **Dual homes demoted carefully:** `doc/plan/builder-explorer-decoupling.md` header (admission + observe-only + store bus); `query.rs` observe-only surface; abyssal-parity explorer roles. Unbuilt daemon / beacons / demand spool remain plan design — not claimed by this segment.
- Pair with maturity CLI display debt on #form-ordinum-governs-flux-web — different surface, same "show the convicting truth" telos.
