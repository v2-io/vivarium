# 03 — Implementation concerns

*Half-life: per major release. Engine-level mechanisms — what Bevy's architecture forces, permits and forbids, and why. The dated inventory is [04](04-implementation-survey.md).*

## Status of this document

This is the **best-evidenced** of the four, and the reason is worth stating: its verifiers did not rely on documentation. They read the vendored crate source on this machine (`~/.cargo/registry/src/index.crates.io-*/bevy_*-0.18.1/`) and GitHub content at release tags, and several refutations turned on exactly that difference.

**The writer of this document still read none of it.** Two evidence tiers appear:

- **`[VERIFIED 3/3]`** — three independent adversarial passes.
- **`[SOURCE-READ]`** — one agent read the vendored crate file directly during synthesis. Fewer eyes than a 3/3, but the strongest possible source tier. Line numbers given are that agent's.
- **`[REFUTED 3/3]`** / **`[CONTESTED]`** / **`[UNVOTED]`** as in 01 and 02.

Detail and the ecosystem ledger: [`appendix/synthesis-bevy-0.18.md`](appendix/synthesis-bevy-0.18.md). The full raw pool (130 mined claims, 26 sources) behind it is [`appendix/harvest/run3-bevy-0.18-primitives.md`](appendix/harvest/run3-bevy-0.18-primitives.md).

### The failure mode this area specifically exhibits

**Version attribution.** Eight of the ten refuted claims here were some form of *true, but not for the release*. The recurring shapes:

- Claims about `main` presented as claims about the release line — when `main` at that date was already deep in the next dev cycle, branched from a separate release branch.
- Claims from a PR *body* that a later PR reverted **before** the tag.
- Claims true of an older minor that a newer one had already falsified.
- `docs.rs/bevy/latest/…` cited for a 0.18 fact. **`/latest` serves 0.19.0**, whose `TextFont` is a structurally different struct. `[VERIFIED 3/3]`

**Rule that follows:** for a capability claim, the release tag or the vendored source is the truth. A PR, a blog post, or a `/latest` link is a lead.

---

## 1. Text writes force layout — the mechanism, not a performance footnote

**`[SOURCE-READ]`** `bevy_text-0.18.1/src/text.rs:744` — `detect_text_needs_rerender<Root>` runs every frame and sets `ComputedTextBlock.needs_rerender = true` whenever `Changed<Root>` fires (`Root` = `Text` for UI, `Text2d` for world-space). **Any** write to the component marks the whole block dirty, regardless of whether the rendered content differs.

The field's own doc comment (`text.rs:60-67`) is candid about the cost:

> "This field currently causes UI to 'remeasure' text, even if the actual changes are non-structural and can be handled by only rerendering and not remeasuring. A full solution would probably require splitting `TextLayout` and `TextFont` into structural/non-structural components for more granular change detection. A cost/benefit analysis is needed."

So a per-frame string reassignment into one text component forces a **full Taffy remeasure and relayout every frame**, not merely a re-rasterize. A one-character digit change costs the same as a wholesale replacement.

**This was not in the original 108-agent harvest.** It was found by reading the source during synthesis.

Two corroborating items, both weaker: **`[UNVOTED]`** Bevy issue #15805 ("change detection for text is more painful after text rework"), filed by a lead maintainer, open since 2024-10-09 — but scoped to *ergonomics* of detecting a change from outside, and explicitly disclaiming any layout-cost claim. And **`[UNVOTED]`** a debug-overlay crate's own perf notes attributing its frame cost to "CPU time spent in Bevy's UI layout systems (in `PostUpdate`)" — independent, different codebase, consistent direction.

**The consequence is structural, not a settings knob.** Fewer, smaller text entities written only when their value changes; a same-value write still trips `Changed<Text>` unless gated (`set_if_neq` or an explicit comparison).

**`[UNVOTED]`** Related and separate: bevy issue #15197 diagnosed that auto-sized or right-aligned nodes whose measured width changes because digit count changes are the construct producing visible 1px jitter — and the effect **"scales with `scale_factor`… invisible at scale factor 1."** That means this class of defect is invisible in a 1× development session and appears only at Retina capture scale.

---

## 2. What is layout-free, and why that matters

**`[VERIFIED 3/3]`** `bevy_ui` 0.18 has a dedicated 2D UI-space transform — `UiTransform { translation: Val2, scale: Vec2, rotation: Rot2 }` — auto-inserted as a **required component of every `Node`**, with `UiGlobalTransform` alongside. Scaling, translating or rotating a node is therefore a *transform* mutation, not a layout mutation: it does not re-run Taffy and does not reflow siblings. Translation is expressed in responsive `Val` units rather than raw pixels.

**`[VERIFIED 3/3]`** `Outline` is explicitly documented as taking no space in the layout — it can appear and disappear without shifting anything around it.

These two are the mechanism by which visual state change can be made free of layout cost.

---

## 3. Text rendering: what the stack actually gives you

**`[VERIFIED 3/3]`** `TextFont::font_size` is the glyph rasterization height in the font atlas, in pixels, **multiplied by the window scale factor and `UiScale`** but **not** by the entity transform or the camera projection. UI text is therefore rasterized at physical resolution automatically — no manual DPI compensation.

**`[VERIFIED 3/3]`** A distinct font atlas is generated per `(font handle, scaled font size)` combination, with an explicit performance warning in the docs. Continuously varying or animating font size is a real cost, not a free effect. **`[VERIFIED 3/3]`** `FontAtlasKey` newtypes `(AssetId<Font>, u32, FontSmoothing)` — the atlas is keyed by smoothing mode too.

**`[UNVOTED]`, surfaced only in a verifier's citation trail.** `UiScale`'s own doc comment says it *"will only affect fixed ui values like `Val::Px`"* — percentage-based sizing is not rescaled by the global lever. A single "make everything bigger" knob reaches only literal-pixel values.

**`[VERIFIED 3/3]`** OpenType feature selection is reachable from the public component API: `TextFont` carries `font_features: FontFeatures`, built via `FontFeatures::builder()`, with `FontFeatureTag` constants including `TABULAR_FIGURES`, `OLDSTYLE_FIGURES` and `STANDARD_LIGATURES`, plus numeric-valued tags. Two independently-extracted claims about this both survived 3/3. **Caveat from the docs:** this covers *".otf fonts that support them"* — the feature must exist in the font's tables.

**`[UNVOTED]`, community-sourced, treat as a lead:** Bevy's shipped default font is described (in a third-party crate's README, not a Bevy source) as a stripped FiraCode, and is not guaranteed to carry feature tables. If tabular figures matter, budget for shipping a real `.otf`.

**`[VERIFIED 3/3]`** `LineHeight` is **no longer a field on `TextFont`** — it is a standalone component required by `Text`, `Text2d` and `TextSpan`. Leading is set per-entity/per-span.

**`[VERIFIED 3/3]`** Font hinting is settable **per text block**, not per `TextSpan` — deliberately kept off `TextFont` because it is a block-level concept. Mixed-hinting runs within one block are not expressible.

**`[CONTESTED 1R/2K]`** `FontSmoothing` has exactly two variants, `AntiAliased` (default) and `None`. Subpixel/LCD antialiasing is absent from the API surface. The *enum shape* was not disputed even by the dissenting vote; the dispute was over the claim's conclusion that crispness must therefore be bought entirely outside the rendering mode — see the next entry, which is a plausible reason that conclusion is wrong.

**`[SOURCE-READ]`, and it reverses a harvested claim.** `FontSmoothing::None` is **not** merely an antialiasing toggle. Read at `bevy_text-0.18.1/src/pipeline.rs:399-401`: the value is consumed per-glyph and, when `None`, **rounds glyph positions and sizes to integers, discarding subpixel layout**. It is a pixel-snapping mechanism as well as an AA switch — directly relevant to small-text crispness.

---

## 4. Contrast affordances that exist, and the one that does not

**`[VERIFIED 3/3]`** `bevy_ui` 0.18 ships the chrome set first-party, no ecosystem crate required: `BorderRadius`, `BoxShadow` (+`ShadowStyle`, with `blur_radius`), `Outline`, the overflow/clip family (`Overflow`, `OverflowAxis`, `OverflowClipBox`, `OverflowClipMargin`, `CalculatedClip`, `OverrideClip`), `ZIndex` and `GlobalZIndex`, and a complete gradient system (`LinearGradient`, `RadialGradient`, `ConicGradient`, `BackgroundGradient`, `BorderGradient`, `ColorStop`, `AngularColorStop`, `InterpolationColorSpace`).

**`[VERIFIED 3/3]`** `BorderRadius` is **no longer a separate component** — it moved to a `border_radius` field on `Node`. Any 0.17-or-earlier example spawning it alongside `Node` will not compile.

**`[VERIFIED 3/3]`** `bevy_ui::widget::TextShadow` — `offset: Vec2` (logical px) + `color: Color`. Implemented as glyphs redrawn at an offset, **not** a blurred shadow; blur was explicitly deferred as needing shader work. Roughly 2× text draw cost.

**`[REFUTED 3/3]`, and the correction is the useful part.** The claim that `Text2d` gets no first-party shadow was **true for 0.15/0.16 and false by 0.18** — `Text2dShadow` exists in `bevy_sprite`, landed via PR #20463 (merged 2025-08-14), present at the `v0.18.0` tag. Both UI text and world-space text have shadow paths.

**`[UNVOTED]`** `TextBackgroundColor` lives in `bevy_text` itself (reachable from both UI and `Text2d` contexts) and renders a rect behind each run.

**`[REFUTED 3/3]`** The claim that `TextShadow` is "the only built-in contrast aid" — false; `TextBackgroundColor`, `Outline`, `BoxShadow` and `BackgroundColor` all exist. That claim also **misattributed its own quote**: the phrase was in PR #17559, not the issue it cited, whose three comments do not contain it.

**Genuinely absent: a real glyph-edge outline/stroke.** **`[REFUTED 3/3]`** on the framing but not the substance — issue #17076 is genuinely open (created 2025-01-01), but the claim that it is orphaned with "no assignee, branch, or PR linked" is false: the timeline shows cross-referenced work including an **open PR #23369, "feat(text): add coverage-based text outlines," `S-Needs-Review`, last updated 2026-05-05.** So: no native outline today; not an untouched topic. Current workaround per a maintainer comment is duplicated offset copies ("sometimes 4 shadows… to create a fake border").

**`[UNVOTED]`** HiDPI-crisp raster *images* in `bevy_ui` remain unsolved — an open issue since 2023-11-14, no milestone. Text is unaffected; icon/sprite assets would need hand-authored @2× variants and a manual selection system.

---

## 5. Animation: what the engine does and does not drive

**`[REFUTED 3/3]`, on its operative clause.** `TryStableInterpolate` is new in 0.18 and does implement for `Color` and `Val` with fallible unit-mismatch handling. But the conclusion that this gives first-party pulsing "rather than requiring a hand-rolled per-element timer" is **false**: `try_interpolate_stable(&self, other, t)` takes `t` as a **caller-supplied parameter**, and Bevy's animation-clip/`AnimationPlayer` machinery binds to the *different* `Animatable` trait, whose implementor list does not include `Color` or `Val` at 0.18. It is a unit-aware lerp helper you call from your own system — it removes footgun math, not the system.

There is no "attach this component and it blinks" primitive in 0.18.

---

## 6. World-space to screen-space

**`[SOURCE-READ]`** `Camera::world_to_viewport` returns render-target-relative rather than viewport-relative coordinates when the camera's viewport has a non-zero origin. Confirmed still present at `bevy_camera-0.18.1/src/camera.rs:501-531` (`world_to_viewport_core`), whose final line unconditionally adds `target_rect.min`:

```rust
let viewport_position =
    (ndc_space_coords.truncate() + Vec2::ONE) / 2.0 * target_rect.size() + target_rect.min;
```

**For a full-window camera (viewport origin (0,0)) the defect is inert.** Only a camera with a non-default viewport rect — split-screen, inset minimap — needs the manual `- target_rect.min` correction. The failure mode if a camera setup changes later is silent (wrong screen position, no panic).

**`[SOURCE-READ]`** `world_to_viewport` returns `Err(PastNearPlane)` / `Err(PastFarPlane)` for points behind the camera — a built-in signal distinct from "projected outside the viewport rect."

**`[UNVOTED]`** Clamp-to-edge and bearing arrows are not shipped by anything the corpus found; that is `atan2` work on top of the projection.

---

## 7. The screenshot path, and one structural hazard

**`[SOURCE-READ]`** `bevy_render-0.18.1/src/view/window/screenshot.rs` confirms the mechanism: spawn a `Screenshot` component, `.observe(save_to_disk(path))`, and Bevy fires `ScreenshotCaptured` when ready. **This is the documented first-party path.**

**`[UNVOTED]`, but source-grounded and consequential.** The mechanism works by redirecting the render target away from the swapchain to a copyable texture. `bevy_ui` renders through Bevy's normal render graph and is therefore captured by construction. **`bevy_egui` (pre-0.35) drew straight to the window's swapchain texture, bypassing the redirect entirely — so its UI was silently absent from every screenshot.** Filed as Bevy issue #16689, labelled `P-Regression`, fixed only on the egui side in 0.35, with no app-level system-ordering workaround per the reporter's own exhausted attempts.

*Whether the Bevy-0.18-compatible `bevy_egui` line postdates that fix — genuinely contested, 2026-07-31.* Checked directly against crates.io this repair pass: `bevy_egui` 0.39.1 (the release pinned for Bevy ^0.18) depends on `egui ^0.33`; `bevy_egui` 0.41.1 (Bevy ^0.19) depends on `egui ^0.35`. A `^0.33` requirement cannot resolve to 0.35 under normal semver — so on the face of the dependency graph, 0.39.1 predates the fix. But this is not fully closable from here: it stays open whether egui backported the fix into a 0.33.x patch (not checked by either audit or this pass), and the two audits that reviewed this dossier disagree — one (`AUDIT-2026-07-31-grok.md` §3.6) treats the dependency fact as sufficient to flip the default assumption to **hazard-present**; the other (`AUDIT-2026-07-31-grok-2.md` B-3) calls the open status **correct as-is**. Both are recorded rather than one being picked. Practical reading: **do not assume the hazard is fixed** on the 0.18 line without checking the actual resolved egui version and its changelog.

**`[UNVOTED]`, a caution about screenshot-based verification generally.** Bevy's own screenshot-diff CI **failed to catch** the font-smoothing regression before it shipped to `main` — the fixing PR names this as one of its objectives. A passing screenshot diff is evidence of gross visual regression, not of subtle rendering-mode defects.

---

## 8. First-party widget layers

**`[VERIFIED 3/3]`** `bevy::ui_widgets` is explicitly **headless and ships no styling whatsoever** — adopting it buys interaction and state logic and zero visual chrome. Its inventory is form controls only (Button, Checkbox, RadioButton, RadioGroup, Slider, Scrollbar, MenuButton, MenuItem, MenuPopup), with external state management by default: widgets emit `Activate`/`ValueChange<T>` events rather than self-mutating.

**`[VERIFIED 3/3]`** `bevy_feathers` — the styled layer — is gated behind a cargo feature literally named `experimental_bevy_feathers` in 0.18. It self-disclaims product use in its own crate docs (*"it's deliberately not intended for that… consider copying this code into your own project"*), corroborated by release notes describing it as developer tooling for editors. Its inventory is also form controls only.

**`[SOURCE-READ]`, not in the original harvest.** `bevy_ui_widgets::popover::Popover` (`popover.rs:1-150`) is a shipped edge-avoidance auto-placement system: given a list of candidate `PopoverPlacement`s (side + alignment + gap) relative to a parent entity, its `PostUpdate` system picks whichever keeps the popover fully inside the window rect minus a configurable `window_margin`, falling back to least-occluded. It solves a different problem from world-point anchoring: `Popover` positions relative to a **UI parent entity** with edge-avoidance; world-space anchoring positions relative to a **world point** via camera projection with none.

---

## 9. Version fragility

**`[VERIFIED 3/3]`** Bevy 0.18's text stack is **cosmic-text**. Bevy 0.19 replaces it **wholesale with Parley**. Glyph, shaping and font-selection behaviour, and any code touching `PositionedGlyph` or `TextPipeline` font IDs, is version-fragile: in 0.19 `PositionedGlyph`'s `byte_index`/`byte_length` are removed because Parley does not expose them, and `map_handle_to_font_id`/`get_font_id` are removed from `TextPipeline`.

**`[VERIFIED 3/3]`** In 0.18, `TextFont::font` is a `Handle<Font>` and `font_size` is a plain `f32` — raw pixels, no unit type, no family-name selection, no rem/relative sizing. 0.19 changes `font` to a `FontSource` enum and `font_size` to a `FontSize` wrapper (`FontSize::Px(35.)`), and adds a rem concept.

**`[VERIFIED 3/3]`** `bevy_feathers` is de-experimentalized in 0.19 (feature renamed), and 0.19 moves Feathers widget construction to BSN as the primary API, renaming the spawn functions.

**`[REFUTED 3/3]`, correction worth keeping.** A claim that render-side UI types moved to a separate `bevy_ui_render` crate and would break tutorials was **half right**: the types (`UiAntiAlias`, `BoxShadowSamples`, `MaterialNode`, `UiMaterial`, `UiMaterialPlugin`) are indeed absent from `bevy_ui`'s index and present in `bevy_ui_render`'s — but **the prelude re-exports them**, so prelude-based code still compiles. The claim also misattributed the `ComputedNodeTarget` split to 0.18 when it had already happened by 0.17.

**`[REFUTED 3/3]`, a clean example of the release-branch fallacy.** The claim that `TextFont::font_smoothing` is a no-op in 0.18 rested on a PR stating the field is "ignored in main currently," milestoned 0.19, merged two days after 0.18.0 was tagged. But that PR's base was `main`, which was already in the 0.19 cycle; `crates/bevy_text/src/pipeline.rs` at tags `v0.18.0` and `v0.18.1` is byte-identical and does **not** contain the hardcoded override the PR removes.

**`[REFUTED 3/3]`, polarity inverted.** A claim that UI `Text` requires `FontHinting::Enabled` and `Text2d` requires `Disabled` — taken from a PR body. At the shipped `v0.18.0` and `v0.18.1` tags **both require `Disabled`**; PR #22494 ("Disable font hinting for UI text by default") merged 2026-01-13, the same day 0.18.0 was tagged, flipping the default before release. 0.19 flips it back to `Enabled`. **Hinting is off by default for both UI and world-space text in shipped 0.18.**
