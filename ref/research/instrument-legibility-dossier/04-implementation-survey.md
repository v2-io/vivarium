# 04 — Implementation survey

> **PERISHABLE. Surveyed 2026-07-31 against Bevy 0.18.1.**
> This document is a dated inventory, not a finding. Every row below was true on the survey date and may not be true now. **Re-run the checks in §4 rather than reasoning from this table stale.** If the survey date is more than a few months behind you, treat the whole document as a list of things to re-check, not as information.

## Status

Figures come from the extraction agents' own primary fetches (crates.io API, docs.rs, GitHub API). **None of these crates is vendored on this machine**, so unlike [03](03-implementation-concerns.md), nothing here was verified by reading source on disk. Tier is "primary registry fetch, single pass, unvoted" unless noted.

The reason this document exists separately from 03: 03 carries mechanisms that survive a version bump; this carries facts that expire.

---

## 1. Engine version ground truth

| Fact | Value | Tier |
|---|---|---|
| `vivarium-explore` pins | `bevy = "0.18"` | read from `Cargo.toml` |
| Resolves to | **0.18.1** — the only 0.18.x in the local registry cache | checked on disk |
| 0.18.0 released | 2026-01-13 | `[VERIFIED 3/3]` |
| 0.18.1 released | **2026-03-04** | `[VERIFIED 3/3]` — closed 2026-07-31 |
| 0.19.0 released | **2026-06-19** | `[VERIFIED 3/3]` — closed 2026-07-31 |
| `docs.rs/bevy/latest/…` now serves | **0.19.0** — never cite `/latest` for a 0.18 fact | `[VERIFIED 3/3]` |
| Text backend, 0.18 | cosmic-text | `[VERIFIED 3/3]` |
| Text backend, 0.19 | **Parley** (wholesale replacement) | `[VERIFIED 3/3]` |

**Closed 2026-07-31.** This table previously recorded the two dates each as an unresolved "X or Y," reasoning that picking one would be inventing a value. Two independent external audits pointed out that the registry this document itself names as authoritative (`crates.io`) is one API call away and is the adjudicator, not a third source to weigh — refusing to check it is not the same discipline as refusing to guess between sources nobody has checked. Re-queried directly against `crates.io`'s `created_at` field (verified twice more, once by each audit, once again in this repair pass): `bevy` 0.18.0 = 2026-01-13T21:15:24Z, 0.18.1 = 2026-03-04T01:22:23Z, 0.19.0 = 2026-06-19T20:17:40Z.

---

## 2. Ecosystem crate ledger

Three facts are required for any crate in this ecosystem to be useful information: **latest release**, **which release supports the Bevy line you're on**, and **when it was last touched**. A crate name without all three is close to worthless here.

| Crate | Latest release | Bevy 0.18-compatible line | Last activity | Standing on survey date |
|---|---|---|---|---|
| **`bevy_ui_anchor`** | 0.12.0 (2026-07-08) | **0.11.0** (2026-02-15) — 0.12.0 requires `bevy ^0.19` | 2026-07-08 | Healthy. 21 releases since 2024-08-07, no yanks, one release per Bevy cycle through 0.14→0.19. **Pin 0.11.0 exactly.** |
| **`bevy_easings`** | 0.19.0 (2026-06-24) | 0.18.0 (2026-01-25) | 2026-06-24 | Healthy. Tracks every Bevy release since 2020. |
| **`bevy_egui`** | 0.41.1 (2026-07-18) | frozen at **0.39.x** (last: 0.39.1, 2026-02-06) | actively developed, but the 0.18-compatible line is ~5.5 months stale and receiving no further fixes | Actively developed *upstream*; the 0.18 line is not. See the screenshot hazard in [03 §7](03-implementation-concerns.md). |
| **`bevy_feathers`** | **0.19.0** (2026-06-19) — *corrected 2026-07-31; this row previously read 0.18.1, conflating "latest on our line" with "latest absolute," which is exactly the confusion the three-facts rule above exists to prevent* | 0.18.1 (2026-03-04) | 0.19.0 released 2026-06-19 | 0.18-line is version-current as of its own release, one Bevy cycle behind absolute latest. Self-disclaims product use in its own docs. Feature-gated `experimental_bevy_feathers` in 0.18. |
| **`iyes_perf_ui`** | 0.5.0 (2025-05-20) | **none** — compat table tops out at Bevy 0.16 | 2025-05-20, ~14 months dormant, not archived | Two Bevy releases behind. |
| **`bevy_screen_diagnostics`** | 0.8.1 (2025-04-26) | **none** — tops out at 0.16; a community 0.17 port (PR #16, opened 2025-10-02) sits unmerged | 2025-04-26, ~15 months dormant, not archived | Two Bevy releases behind. Also narrow in scope (fps / entity count / cpu-mem). |
| **`bevy_mod_billboard`** | 0.7.0 (2024-07-10) | **none** — frozen at Bevy 0.14; 0.15/0.16 upgrade requests unanswered, one over a year old | 2024-07-10, >2 years dormant, not archived | Built on the pre-0.15 Bundle pattern — porting is an API rewrite, not a version bump. |

### 2.1 Caveats attached to specific crates

**`bevy_ui_anchor`** — requires tagging the camera with a marker component (`UiCameraMarker`) or it **silently anchors nothing**. Documentation coverage is 38.89% per docs.rs; budget time reading its ~2000-line source rather than its docs page. Its README does **not** document clamp-to-edge or arrow/bearing behaviour — the corpus flags this as *unconfirmed*, not as confirmed-absent.

**`bevy_easings`** — implements `Lerp` for `Node`, `BackgroundColor`, `TextColor`, `Val`, `UiRect` (behind a `ui` feature; `BackgroundColor` additionally needs `render`). `EasingType` ships `Loop { duration, pause }` and `PingPong { duration, pause }` as data fields on the component, and `EaseMethod::Discrete` for hard on/off transitions. Two caveats, neither independently re-checked: easing a `Val` across mismatched unit variants (`Px` ↔ `Percent`, or anything touching `Val::Auto`) **silently no-ops** — falls through to a catch-all returning the start value, no panic and no warning; and the confirmed `Lerp` impls **do not include `UiTransform`**, so a scale-pulse rather than a colour/alpha pulse is an **open question**, not a refuted one.

---

## 3. Feature gates on the survey date

| Feature | Default? | Notes |
|---|---|---|
| `experimental_bevy_ui_widgets` | **No** | Required for `bevy_ui_widgets`, including `popover::Popover` |
| `experimental_bevy_feathers` | **No** | Transitively pulls in `experimental_bevy_ui_widgets` |

`vivarium-explore`'s current `bevy = "0.18"` with no feature list gets **neither**.

---

## 4. How to re-run this survey

The whole point of separating this document is that it can be regenerated cheaply. For each crate:

```sh
# latest version + all versions with dates
curl -s https://crates.io/api/v1/crates/<CRATE>/versions | jq '.versions[] | {num, created_at, yanked}'

# which bevy version a given release depends on
curl -s https://crates.io/api/v1/crates/<CRATE>/<VERSION>/dependencies \
  | jq '.dependencies[] | select(.crate_id=="bevy") | {req, kind}'
```

For engine facts, prefer in this order: **vendored source on disk** (`~/.cargo/registry/src/index.crates.io-*/bevy_*-<VERSION>/`) → **GitHub raw at the release tag** (`raw.githubusercontent.com/bevyengine/bevy/v<VERSION>/…`) → **version-pinned docs.rs** (`docs.rs/<crate>/<VERSION>/…`, never `/latest`) → migration guide → release blog → community post.

The single highest-yield check, and the one that caught the most defects in this survey: **read the vendored crate source.** It is on disk, it is free, and it is the version that actually compiles.

---

## 5. What this survey did not establish

- Whether the Bevy-0.18-compatible `bevy_egui` line (0.39.x) postdates or predates the egui-side 0.35 fix for the screenshot-bypass defect. **Partially closed 2026-07-31, still genuinely contested between the two external audits — see 03 §7 for the full statement of both positions and why neither is picked here.** In short: `bevy_egui` 0.39.1 (the 0.18-compatible release) depends on `egui ^0.33` (checked directly against crates.io dependency data, this repair pass) — a caret requirement that cannot resolve to 0.35 unless egui backported the fix into a 0.33.x patch, which nobody in this chain has checked. One audit treats that as sufficient to flip the default assumption to hazard-present; the other still calls it correctly open. Both readings are recorded rather than one being picked.
- Whether `bevy_easings` can drive `UiTransform`. Only `Node`/`BackgroundColor`/`TextColor`/`Val`/`UiRect` were confirmed.
- Whether `bevy_ui_anchor` ships any off-screen clamping or bearing behaviour. Absent from the fetched README; not confirmed absent from the source.
- Whether Bevy's shipped default font carries OpenType feature tables. The only source is a third-party crate's README describing it as a stripped FiraCode — a lead, not a fact.
- Any crate not named by the original searches. This is a survey of what six parallel search agents surfaced, not an exhaustive census of the ecosystem.
