# run3-bevy-0.18-primitives

run: `wf_4f1d2ebb-f19`  ·  agents harvested: 108

## Scope — how the question was decomposed

_Structure the search as a version-currency spine plus four capability probes and one defect probe: first pin what Bevy 0.18 actually is from primary release/migration sources so later queries use current vocabulary, then probe the two highest-value subsystems separately (text/typography; world-anchored and animated attention affordances), audit the first-party widget layer against the ecosystem-crate layer with release-date provenance, and finish with an adversarial pass through GitHub issues and PRs. That last angle matters because "not achievable without unreasonable work" is a first-class deliverable here; every angle is scoped to force a version stamp so an authoritative-sounding pre-0.18 blog post gets caught rather than believed._

- **primary-release-truth** — `Bevy 0.18 release announcement migration guide bevy_ui changes Node Text required components`
  - Anchors everything to primary sources at the exact pinned version: what landed, what was renamed or removed, what the migration guide demands. Later angles' claims get checked against this, and it surfaces the actual 0.18 vocabulary so the remaining queries are not phrased in stale 0.13-era terms.
- **text-typography-stack** — `Bevy 0.18 text rendering cosmic-text custom font loading TextFont TextSpan UiScale scale_factor font smoothing crisp small text`
  - The brief names crisp small text over a busy 3D scene as the single highest-value thing to get right. Targets the underlying shaper, font asset loading, logical-vs-physical pixel sizing, UiScale/DPI on Retina, per-span styling, antialiasing controls, and whether OpenType features or tabular figures are reachable at all.
- **first-party-widgets** — `bevy_feathers bevy_ui_widgets Bevy 0.18 first-party widget set observers BSN scene format status`
  - Tests the delegator's own flagged guess ("feathers"?) directly rather than assuming it. Establishes whether a maintained in-repo widget layer exists, its maturity (experimental? feature-gated? known gaps?), and whether the reactive/retained-UI direction changes the recommended pattern for a status-chip panel.
- **ecosystem-crate-currency** — `bevy_egui iyes_perf_ui bevy_screen_diagnostics bevy_lunex Bevy 0.18 compatibility latest version last updated maintained`
  - Delivers the three demanded facts per crate — latest release, Bevy version supported, last update — and surfaces the first-class finding that a well-known crate stalled at 0.15. Also where to weigh whether immediate-mode debug UI (egui) is the wrong tool for a panel that must be visually calm and stable.
- **hud-techniques-attention** — `Bevy 0.18 world to screen position UI overlay billboard label off-screen indicator arrow UI animation easing blinking alert`
  - Covers the attention-affordance and spatial-anchoring half of the design target: camera world_to_viewport projection for anchoring labels to a point on the globe, off-screen direction indicators, billboarding, and whether UI animation/easing/pulsing exists as a primitive or must be hand-rolled per element with a timer.
- **limitations-and-defects** — `Bevy UI github issue blurry text high DPI fractional scale factor layout reflow screenshot API 0.18 limitation`
  - The skeptical pass: open issues, merged PRs and bug reports are the best evidence for what is known-broken versus merely undocumented. Targets fractional-DPI text blur on macOS, per-frame layout/allocation cost of mutating Text, frame-to-frame layout stability, and whether the screenshot API composites UI and text correctly — load-bearing because captures are committed as repository evidence.

## Sources fetched and mined (26)

### https://github.com/bevyengine/bevy/pull/22308
quality: **primary** · published: 2026-01-01 (merged; opened 2025-12-29)

- **[supporting]** Bevy 0.18 bumped its text shaping/rasterization backend cosmic-text from 0.15 to 0.16, and the upgrade was API-compatible for Bevy itself (no cosmic-text breaking changes to absorb). PR #22308, opened 2025-12-29, merged 2026-01-01 into the 0.18 cycle.
  > Update cosmic-text to 0.16 … **No Breaking Changes:** The upgrade introduces no incompatible API modifications to Cosmic Text itself.
- **[central]** Bevy 0.18 introduces a `FontHinting` component (mapping 1:1 to cosmic-text's `Hinting` enum) that controls glyph rasterization hinting — the first user-reachable font-hinting/crispness control in the Bevy text stack. UI `Text` requires `FontHinting::Enabled`; `Text2d` requires `FontHinting::Disabled`.
  > Version 0.16 adds font-hinting capabilities. The PR implements this through a new `FontHinting` component that maps to cosmic-text's identical `Hinting` enum. … `Text` component (pixel-aligned): "now requires `FontHinting::Enabled`" … `Text2d` component (non-pixel-aligned): "now requires `FontHinting::Disabled`"
- **[central]** Font hinting in Bevy 0.18 is settable only per text block (per entity carrying the `Text`/`Text2d` component), not per `TextSpan` — it was deliberately not added as a field on `TextFont`, so mixed-hinting runs within one text block are not expressible.
  > "Hinting needs to be set per text block, not per text span entity, so it wouldn't be appropriate to add it as a field on `TextFont`."
- **[supporting]** The hinting control exposed is binary — `FontHinting::Enabled` / `FontHinting::Disabled` — with no exposed hinting-strength, subpixel/LCD-filter, or gamma option in this API surface. (Corroborated on docs.rs, which documents `FontHinting` in bevy_text 0.19.0, released 2026-06-19 — i.e. the component survived past 0.18.)
  > "Font hinting strategy, which controls the rasterization for fonts." … `Disabled` - "Glyphs are rasterized without hinting." … `Enabled` - "Glyphs are rasterized with hinting."
- **[supporting]** Bevy 0.18's cosmic-text 0.16 bump fixes a real text-rendering defect present in earlier Bevy versions: an ASCII fastpath bug in cosmic-text 0.15 that mis-rendered fonts when multiple fonts appeared in one text block without intervening spaces (Bevy issue #22191).
  > The update resolves an ASCII fastpath bug in cosmic-text 0.15 that caused incorrect font rendering when multiple fonts appeared in a single text block without intervening spaces.

### https://github.com/bevyengine/bevy/issues/17076
quality: **primary** · published: 2025-01-01 (issue opened); last comment 2025-03-04; state re-verified OPEN on 2026-07-31

- **[central]** Bevy has no first-party text outline/border effect: the upstream feature request for it (bevyengine/bevy#17076, opened 2025-01-01) is still OPEN today, carrying maintainer labels A-Text / A-Rendering / C-Feature / D-Complex / S-Ready-For-Implementation — i.e. designed-but-unimplemented, with no assignee, branch, or PR linked. Verified via `gh issue view 17076 --repo bevyengine/bevy` on 2026-07-31: "state":"OPEN", last updated 2025-03-04. For crisp small HUD text over a busy 3D globe, outline-for-contrast must therefore be hand-rolled (duplicated text nodes) or obtained from a custom material/crate.
  > Having the ability to add effects to the text such an outline/border, a shadow or other kinds of effects allows us to place the text anywhere and keep the contrast so it can be read easily without the need of adding a rectangle with a background which in some cases it might be seen ugly or not fitting the game vision.
- **[central]** The one text effect Bevy did land is `TextShadow` (PR bevyengine/bevy#17559 "Basic UI text shadows", MERGED 2025-02-05), and it is deliberately UI-only — the component lives in `bevy_ui`, not `bevy_text`, so `Text2d` (world-space / billboard labels) gets no first-party shadow. This matters directly for anchoring labels to points on a globe: the contrast affordance available to `bevy_ui` nodes does not follow you into `Text2d`.
  > I added the `TextShadow` component to `bevy_ui` rather than `bevy_text` because it only supports the UI atm. We can add a `Text2d` version in a followup but getting the same effect in `Text2d` is trivial even without official support.
- **[central]** `TextShadow` is not a blurred shadow — it is implemented as the glyphs redrawn at an offset in a different color (fields: `offset`, `color`; system `extract_text_shadows`). Blur-radius support was explicitly deferred as requiring shader changes. So the only built-in contrast aid for text over arbitrary imagery is a hard 1-copy drop shadow, at roughly 2x the text draw cost.
  > It's not "real" shadows just the text redrawn with an offset and a different colour. Blur-radius support will need changes to the shaders and be a lot more complicated, whereas this still looks okay and took a couple of minutes to implement.
- **[supporting]** The known workaround for a text outline in Bevy, absent native border support, is stacking multiple offset shadows (commonly four) to fake a border — stated by the issue author while acknowledging #17559 does not cover border or blur. This is the practical recipe a HUD needing legible small text over unknown background would have to implement itself.
  > Also for the aim of enhancing text legibility, having support for border and blur is quite interesting. (Sometimes 4 shadows are used to create a fake border).
- **[tangential]** Bevy's text pipeline is not signed-distance-field based, which is why per-glyph effects (outline, glow, cheap scaling) are not cheaply available — asserted in-thread by a community commenter (mrec, 2025-01-01), not by a maintainer, and hedged as "AFAIK"; treat as a lead to verify against the bevy_text/cosmic-text source rather than as established fact.
  > Signed Distance Field text rendering is basically tailor-made for applying this kind of effect, but it's not Bevy's current direction AFAIK. I agree that the requirement is a very common and important one for any kind of overlay UI.

### https://bevy.org/news/bevy-0-19/
quality: **primary** · published: 2026-06-19

- **[central]** Bevy 0.19 was released on June 19, 2026, meaning the vivarium crate's pinned `bevy = "0.18"` is one release behind current as of this research (July 2026); the release links an official "0.18 to 0.19 Migration Guide", so the 0.18-targeted question has a known, documented upgrade path rather than being the current-version answer.
  > Bevy 0.19 ... Posted on June 19, 2026 by Bevy Contributors
- **[central]** Bevy's text stack no longer sits on cosmic-text: in the 0.19 cycle the engine migrated text shaping/layout from `cosmic_text` to `parley`. Any typography guidance, workaround, or internals assumption written against the cosmic-text backend (i.e. everything true for 0.18 and earlier) has an expiry date at 0.19.
  > While all of these features would have been possible in cosmic_text, we've chosen to migrate to parley during this cycle.
- **[central]** As of 0.19, fonts can be selected without loading an asset handle: `FontSource` accepts an asset handle, a family name, or a semantic category, and the semantic categories explicitly include `Monospace`. Variable-font axes are also reachable — `TextFont` gained `weight`, `width`, and `style` fields. This bears directly on monospace/numeric-column alignment for an instrument panel, and it is a capability the 0.18 pin does not have.
  > FontSource now offers three ways to identify a font: // Asset handle, // Family name, // Semantic category ... The generic variants — Serif, SansSerif, Cursive, Fantasy, Monospace, and several UI-specific ones ... TextFont has gained the weight, width, and style fields.
- **[supporting]** `TextFont::font_size` changed type in 0.19 from a bare `f32` to a `FontSize` enum carrying responsive units (`Px`, `Vw`, `Vh`, `VMin`, `VMax`, `Rem`), and a new `LetterSpacing` component was added. This is a concrete breaking change against vivarium's existing `TextFont { font_size: 12.0, ..default() }` spawn, and it is the mechanism by which HUD text sizing can be made viewport- or root-relative rather than a fixed logical-pixel constant.
  > font_size is now a FontSize enum rather than a bare f32 ... A new LetterSpacing component controls the spacing between characters
- **[supporting]** The first-party widget set is named Feathers, it is real and shipping (not planned), and in 0.19 it was rebuilt on top of BSN — Bevy Scene Notation, a Rust-like scene syntax available via a `bsn!` macro — with new widgets including text input, number input, dropdowns, disclosure toggles, panes, list views and scrollbars. Feathers is described as editor-focused, which is a caveat for product-grade instrument chrome.
  > The Feathers widgets have migrated to BSN, Bevy's next-generation scene system. ... BSN is an ergonomic Rust-like scene syntax which can be defined in Rust code via the bsn! macro

### https://bevy.org/news/bevy-0-18/
quality: **primary** · published: 2026-01-13

- **[central]** Bevy 0.18 exposes OpenType feature selection directly on `TextFont` via a `font_features` field built with `FontFeatures::builder()` and `FontFeatureTag` constants — meaning ligature control, small caps, oldstyle/lining numerals and variable-axis values (e.g. WIDTH) are reachable from the engine API without dropping to cosmic-text — but only for `.otf` fonts that carry the features, and support varies per font.
  > "fine-grained control over how text is displayed, including ligatures, small caps, and many more." ... TextFont { font: opentype_font_handle, font_features: FontFeatures::builder().enable(FontFeatureTag::STANDARD_LIGATURES).set(FontFeatureTag::WIDTH, 300).build(), ..default() } ... "OpenType font features are only available for .otf fonts that support them, and different fonts may support different subsets of OpenType features."
- **[supporting]** Bevy 0.18 added variable-weight font support: `TextFont` gained a `weight: FontWeight` field that drives smooth weight variation within a single variable font file.
  > "Bevy now supports font weights, allowing you to take advantage of variable weight fonts which embed smooth variations of a font into a single file! TextFont now has a weight: FontWeight field."
- **[central]** Bevy 0.18 introduces a `TryStableInterpolate` trait that makes `Color` and the UI layout type `Val` animatable through the engine's own animation/interpolation machinery, handling `Val` unit mismatches fallibly — i.e. pulsing/fading alert chrome and animated layout values have first-party support rather than requiring a hand-rolled per-element timer.
  > A new `TryStableInterpolate` trait enables animation of `Color` and `Val` (UI layout) types. The system handles unit mismatches gracefully through fallible interpolation.
- **[supporting]** Text decoration is component-driven in 0.18: adding a `Strikethrough` or `Underline` component to a `Text`, `Text2d`, or `TextSpan` entity renders that decoration — confirming `TextSpan` remains the per-span styling unit in this version and that decoration is per-span addressable.
  > "bevy_text now supports strikethrough and underline. To display text with strikethrough or underline, just add the Strikethrough or Underline components to any Text, Text2d, or TextSpan entity."
- **[supporting]** Bevy's first-party widget layer as of 0.18 is split between `bevy_ui` standard widgets — which gained `Popover` (floating-ui-inspired auto-positioning), `MenuPopup`, improved `RadioButton`/`RadioGroup`, an opt-in `AutoDirectionalNavigation` component for gamepad/keyboard spatial navigation, pickable individual text sections, and an `IgnoreScroll` component for sticky headers — and Bevy Feathers, which is still labeled an experimental widget library (its 0.18 addition was a `ColorPlane` 2D color picker).
  > Adding `AutoDirectionalNavigation` component enables "automatic compute navigation connections based on spatial positioning" ... "Individual text sections belonging to UI text nodes are now pickable" ... The new `IgnoreScroll` component allows UI nodes to "ignore its parent's ScrollPosition along specific axes" ... The experimental widget library added `ColorPlane`

### https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html
quality: **primary** · published: Auto-generated API docs; page fetched 2026-07-31. `/latest` build corresponds to bevy 0.19.0 (fetch reported a 2026-06-19 release date for that version — treat the exact date as unconfirmed by the page banner). The contrast page is pinned to bevy 0.18.0.

- **[central]** docs.rs `/latest` for bevy no longer resolves to 0.18 — it now serves 0.19.0, whose `TextFont` is a structurally different eight-field struct (adds `width`, `style`, `font_variations`; changes `font` from `Handle<Font>` to `FontSource` and `font_size` from `f32` to `FontSize`). Any `/latest` docs.rs link cited for a Bevy 0.18 target is therefore version-inaccurate, and the pinned URL form `docs.rs/bevy/0.18.0/...` must be used instead. Verified by fetching both: the 0.18.0 page lists exactly five fields (`font`, `font_size`, `weight`, `font_smoothing`, `font_features`).
  > pub struct TextFont { pub font: FontSource, pub font_size: FontSize, pub weight: FontWeight, pub width: FontWidth, pub style: FontStyle, pub font_smoothing: FontSmoothing, pub font_features: FontFeatures, pub font_variations: FontVariations, }
- **[central]** OpenType feature selection IS reachable in Bevy 0.18 as a first-class field on `TextFont` — `font_features: FontFeatures`, documented as covering OpenType features for .otf fonts that support them. This means tabular/lining figures (`tnum`, `lnum`) for non-jittering numeric readouts are addressable through the public component API without dropping to a custom text pipeline, contrary to the brief's expectation that OpenType features might not be reachable at all.
  > OpenType features for .otf fonts that support them.
- **[central]** In Bevy 0.18 `TextFont::font_size` is a logical-pixel quantity that the text pipeline multiplies by the window scale factor and by `UiScale` to get the rasterized glyph height, and explicitly does NOT apply the text entity's transform or the camera projection. So on a macOS Retina / fractional-DPI display, UI text is rasterized at physical resolution automatically (no manual DPI compensation needed), and `UiScale` is the single global lever for panel-wide size changes.
  > The vertical height of rasterized glyphs in the font atlas in pixels. This is multiplied by the window scale factor and `UiScale`, but not the text entity transform or camera projection.
- **[supporting]** Bevy 0.18 allocates a distinct font atlas per (font handle, scaled font size) pair, and the docs themselves warn this carries a strong performance cost. For an instrument panel this is a concrete design constraint: a small fixed type scale (a few discrete sizes) is cheap, while continuously animated/interpolated font sizes or a per-chip size ladder is expensive — and because the scaled size includes the scale-factor and `UiScale` multipliers, changing `UiScale` invalidates and regenerates every atlas.
  > a new font atlas generates for every combination of font handle and scaled font size which can have a strong performance impact
- **[supporting]** Text antialiasing in Bevy 0.18 is directly controllable per text entity via the `font_smoothing: FontSmoothing` field with a `with_font_smoothing(...)` builder method, rather than being a global renderer setting. This is the documented lever for crisp small text (e.g. selecting no-antialias for pixel-exact glyph edges in committed screenshot captures), and it is settable independently on individual HUD chips.
  > The antialiasing method to use when rendering text.

### https://github.com/bevyengine/bevy/pull/22455
quality: **primary** · published: 2026-01-15 (merged; opened 2026-01-09)

- **[central]** In Bevy 0.18, `TextFont::font_smoothing` is a no-op: the text pipeline ignores the field and always renders antialiased. The PR states the bug exists in `main` as of 2026-01-09 and is milestoned for **0.19** (merged 2026-01-15) — while Bevy 0.18.0 was tagged 2026-01-13, i.e. two days *before* the fix merged. So a 0.18 project cannot select `FontSmoothing::None`; any code setting it will silently do nothing.
  > 1. The `font_smoothing` field of `TextFont` is ignored in main currently. Instead it always uses `FontSmoothing::Antialiased`.
- **[supporting]** The `FontSmoothing` enum is absent from `bevy_text::prelude` in this era, so it cannot be reached via `bevy::prelude::*` — it must be imported explicitly as `bevy::text::FontSmoothing`. The PR adds it to the prelude, again only for 0.19; the testbed diff shows the explicit-import form required before that.
  > 2. The `FontSmoothing` enum is missing from the `bevy_text` prelude.
- **[supporting]** Font smoothing is a per-text-span property, not a per-block or global one: the fix stores the mode on each `TextEntity` inside `ComputedTextBlock` and reads it back per span index in `TextPipeline::update_buffer` (`let font_smoothing = computed.entities[span_index].font_smoothing;`). This confirms the intended 0.19+ design allows mixed smoothing across spans of one `Text`.
  > 1. Store the antialiasing mode in `ComputedTextBlock`, for use in `TextPipeline::update_buffer`.
- **[supporting]** `FontSmoothing` exposes only two variants in this era — `AntiAliased` and `None` — as shown by the added testbed example iterating `[FontSmoothing::AntiAliased, FontSmoothing::None]`. There is no subpixel/LCD, gamma, hinting, or stem-darkening option in the enum, so those knobs are not reachable through `TextFont` at all in 0.18 or 0.19.
  > 3. Add a font smoothing example to `testbed_ui`'s `Text` scene.
- **[tangential]** Bevy's screenshot-based CI did not detect a visible text-rendering regression that persisted through a release — the PR names the CI miss as its own third objective. This is direct evidence about the limits of screenshot capture as a text-fidelity check in this stack.
  > 3. The `FontSmoothing` bug wasn't caught by the screenshot CI.

### https://docs.rs/crate/bevy/0.18.1/features
quality: **primary** · published: 2026-03-04

- **[central]** In Bevy 0.18.1 both first-party widget layers are opt-in, not default: they are exposed only under `experimental_`-prefixed feature names (`experimental_bevy_ui_widgets`, `experimental_bevy_feathers`) and neither carries the `(default)` marker that `bevy_ui` and the rest of the core UI stack carry. Adding `bevy = "0.18"` with default features therefore gives you NO first-party widgets — a Cargo.toml change is required, and the `experimental_` prefix is the crate's own stability signal.
  > - experimental_bevy_ui_widgets - [bevy_internal](/crate/bevy_internal/^0.18.0)/[bevy_ui_widgets](/crate/bevy_internal/^0.18.0/features#bevy_ui_widgets) - experimental_bevy_feathers - [bevy_internal](/crate/bevy_internal/^0.18.0)/[bevy_feathers](/crate/bevy_internal/^0.18.0/features#bevy_feathers) - [experimental_bevy_ui_widgets](#experimental_bevy_ui_widgets) - bevy_ui (default) - [bevy_internal](/crate/bevy_internal/^0.18.0)/[bevy_ui](/crate/bevy_internal/^0.18.0/features#bevy_ui) (default)
- **[central]** `experimental_bevy_feathers` transitively enables `experimental_bevy_ui_widgets` — the two are stacked, not alternatives. Feathers (the styled widget/theme layer) is built on top of the headless widget layer, so opting into Feathers necessarily pulls in the widget behavior crate; you cannot take Feathers' styling without the widget layer beneath it.
  > - experimental_bevy_feathers - [bevy_internal](/crate/bevy_internal/^0.18.0)/[bevy_feathers](/crate/bevy_internal/^0.18.0/features#bevy_feathers) - [experimental_bevy_ui_widgets](#experimental_bevy_ui_widgets)
- **[central]** The entire primitive stack a from-scratch HUD needs ships in Bevy 0.18.1's default feature set with zero Cargo configuration: `bevy_ui` (layout/nodes), `bevy_ui_render` (the UI render pipeline), `bevy_text` (text rendering), `default_font` (an embedded fallback font), plus `bevy_gizmos` and `bevy_sprite`. Building status chips and instrument chrome from `Node`/`Text` primitives requires no feature-flag work, unlike either widget layer.
  > - bevy_ui_render (default) ... (default) - bevy_text (default) ... (default) - default_font (default) ... (default) - bevy_gizmos (default) ... (default) - bevy_sprite (default) ... (default)
- **[supporting]** Bevy 0.18.1's debug and dev-tooling overlays are opt-in: `bevy_ui_debug` (the UI layout debug overlay) and `bevy_dev_tools` (which gates the built-in FPS/dev overlay) both appear without the `(default)` marker. A project on default features has no built-in UI debug visualization or dev overlay available until it enables these flags.
  > - bevy_ui_debug - [bevy_internal](/crate/bevy_internal/^0.18.0)/[bevy_ui_debug](/crate/bevy_internal/^0.18.0/features#bevy_ui_debug) - bevy_dev_tools - [bevy_internal](/crate/bevy_internal/^0.18.0)/[bevy_dev_tools](/crate/bevy_internal/^0.18.0/features#bevy_dev_tools)
- **[supporting]** Bevy 0.18.1 was published 2026-03-04, and Bevy 0.19.0 was released 2026-06-19 — so as of mid-2026 the 0.18 line is one full minor version behind current, and any 0.18-targeted research is already writing against a superseded release. (Version list from the same docs.rs crate page shell as the features tab.)
  > "**0.18.1** (2026-03-04)" ... "**0.19.0** (2026-06-19)"

### https://docs.rs/bevy/0.18.0/bevy/ui_widgets/index.html
quality: **primary** · published: Not stated on page; docs.rs build pinned to bevy 0.18.0 (retrieved 2026-07-31)

- **[central]** Bevy 0.18's first-party widget layer (`bevy::ui_widgets`) is explicitly headless and ships no styling whatsoever — any visual appearance (chip backgrounds, borders, alert colors, typography) must be authored by the application, so adopting it buys interaction/state logic but zero visual chrome.
  > These widgets have no inherent styling, it's the responsibility of the user to add styling appropriate for their game or application.
- **[central]** The module is self-declared experimental at 0.18.0 with an API expected to change substantially across versions, so code written against it should be treated as carrying a migration cost at each Bevy release — a first-class version-currency finding for anyone pinning to 0.18.
  > This crate is currently experimental and under active development. The API is likely to change substantially: be prepared to migrate your code.
- **[central]** The complete widget inventory in `bevy::ui_widgets` at 0.18.0 is form controls only — Button, Checkbox, RadioButton/RadioGroup, Slider (+ SliderValue/SliderRange/SliderStep/SliderPrecision/SliderThumb), Scrollbar, MenuButton/MenuItem/MenuPopup — with no panel, status chip, label, tooltip, toast, badge, gauge, alert/banner, or text-display widget of any kind. Nothing in the first-party widget layer addresses an instrument-panel/HUD readout; that layer must be built directly on `bevy_ui` `Node`/`Text` primitives.
  > This crate provides a set of standard widgets for Bevy UI, such as buttons, checkboxes, and sliders.
- **[supporting]** The widgets use external state management by default: they emit events (`Activate`, `ValueChange<T>`) rather than mutating their own state, and self-updating behavior is opt-in via explicitly-added observer functions `checkbox_self_update` and `slider_self_update`. Interaction is observer/event-driven, not immediate-mode polling — which suits a change-detection-driven, non-reflowing HUD but means every widget needs an app-side state system written by hand.
  > Most of the widgets use external state management: this means that the widgets do not automatically update their own internal state, but instead rely on the app to update the widget state (as well as any other related game state) in response to a change event emitted by the widget.
- **[supporting]** Widget registration is granular: `UiWidgetsPlugins` is a plugin group registering observers for all widgets, but each widget has an individual plugin (`ButtonPlugin`, `CheckboxPlugin`, `SliderPlugin`, `ScrollbarPlugin`, `RadioGroupPlugin`, `MenuPlugin`) that can be added alone — so a HUD that needs, say, only a slider pays no cost for the rest of the set.
  > A plugin group that registers the observers for all of the widgets in this crate. If you don't want to use all of the widgets, you can import the individual widget plugins instead.

### https://bevy.org/news/bevy-0-17/
quality: **primary** · published: 2025-09-30

- **[central]** Bevy's first-party widget effort is deliberately split into two layers: an unstyled/headless behavior layer (bevy_ui_widgets) modeled on web headless-UI libraries, and a separate opinionated styled set (Feathers). The justification is that headless widgets supply interaction logic, state management, and accessibility without imposing appearance — meaning a project wanting its own visual language (e.g. a calm instrument panel) can take the behavior layer and skip Feathers' look entirely.
  > Headless widget libraries, such as headlessui and reakit have become popular. These provide unstyled standardized widgets that implement all of the correct interactions and behavioral logic: events, state management, accessibility, etc.
- **[central]** As of the 0.17 release, both first-party widget layers ship only behind experimental feature flags — `experimental_bevy_ui_widgets` for the headless widgets and `experimental_bevy_feathers` for Feathers — and are not enabled by default. Whether these flags still exist (or were stabilized/renamed) in 0.18 is the falsifiable follow-up.
  > Feathers is still early in development. It is currently hidden behind the `experimental_bevy_feathers` feature flag. … If you would like to try out our experimental headless widgets, enable the `experimental_bevy_ui_widgets` feature.
- **[central]** Feathers is explicitly scoped as a developer-tooling/editor widget set with a utilitarian, editor-matching aesthetic — not a general-purpose game-HUD or product-chrome toolkit. This is direct evidence against adopting Feathers wholesale for a visually-calm operator instrument panel, and in favor of taking bevy_ui_widgets' behavior layer plus custom styling.
  > Feathers is meant to be Bevy's 'developer tools' widget set, and it will be used to build the upcoming Bevy Editor. It has a utilitarian look and feel, with an opinionated feature-set tailored for editors and graphical utilities.
- **[central]** The Bevy team stated at 0.17 that the BSN scene/UI system (the `bsn!` macro and `.bsn` asset format) was targeted to land in Bevy 0.18, and that Feathers would be ported onto BSN when it did. This is a dated, checkable prediction about exactly the version this research targets — if BSN did not land in 0.18, or landed partially, that materially changes what 'the 0.18 way' to build UI is.
  > We plan to land the new bsn! macro and the .bsn asset format in Bevy 0.18. … We will port Feathers to BSN (Bevy's Next-Generation Scene/UI System) when that lands (targeted for Bevy 0.18).
- **[supporting]** Bevy 0.17 added text-contrast and chrome primitives directly usable for HUD legibility over a busy 3D scene: `TextBackgroundColor` on UI `Text`/`TextSpan` entities, `Text2dShadow` for worldspace text, per-side border colors via `BorderColor`, and UI gradients via `BackgroundGradient`/`BorderGradient` (Linear, Conic, Radial). It also replaced `Transform`/`GlobalTransform` in UI with a specialized `UiTransform` — an API break that invalidates pre-0.17 UI tutorials.
  > Insert the `TextBackgroundColor` component on a UI `Text` or `TextSpan` entity to set a background color for its text section. … In Bevy UI, `Transform` and `GlobalTransform` have been replaced by `UiTransform`

### https://bevy.org/learn/migration-guides/0-17-to-0-18/
quality: **primary** · published: Not stated on the page; the guide is versioned "0.17 to 0.18" and corresponds to the Bevy 0.18 release cycle

- **[central]** In Bevy 0.18, `LineHeight` is no longer a field on `TextFont` — it is a standalone component required by `Text`, `Text2d`, and `TextSpan`, so leading is set per-entity/per-span rather than inside the font struct. (Directly affects the vivarium HUD's existing `TextFont { font_size: 12.0, ..default() }` spawn pattern: line spacing must now be an additional component.)
  > The `line_height` field has been removed from `TextFont`. `LineHeight` is now a component required by `Text`, `Text2d`, and `TextSpan`.
- **[central]** Rounded corners are a first-party `bevy_ui` primitive in 0.18, but `BorderRadius` is no longer a separate component — it has moved to a `border_radius` field on the `Node` struct, so any 0.17-or-earlier example that spawns `BorderRadius` alongside `Node` will not compile against 0.18.
  > `BorderRadius` is no longer a component, instead a `border_radius: BorderRadius` field has been added to `Node`.
- **[central]** Font smoothing is a real, addressable dimension of Bevy 0.18's glyph rasterization: `FontAtlasKey` newtypes `(AssetId<Font>, u32, FontSmoothing)`, meaning a separate glyph atlas is cached per (font asset, size, smoothing setting). `FontAtlasSets` was removed and `FontAtlasSet` is now a resource wrapping `HashMap<FontAtlasKey, Vec<FontAtlas>>`, with the glyph/atlas helper methods reworked into free functions in the `font_atlas` module.
  > `FontAtlasSets` has been removed. · `FontAtlasKey` now newtypes a `(AssetId<Font>, u32, FontSmoothing)`. · `FontAtlasSet` is now a resource. It newtypes a `HashMap<FontAtlasKey, Vec<FontAtlas>>` and derives `Deref` and `DerefMut`. · Font atlases are looked up directly using a `FontAtlasKey`, there's no longer a separate `AssetId<Font>` to `FontAtlasKey` map. ... The `FontAtlasSet` methods `add_glyph_to_atlas`, `get_glyph_atlas_info`, and `get_outlined_glyph_texture` have been moved into the `font_atlas` module and reworked into free functions.
- **[supporting]** Bevy 0.18's `TextLayoutInfo` exposes per-run typographic geometry — including underline and strikethrough position and thickness, plus a bounding rect and span index per run — via a new `run_geometry` field that replaced `section_rects`; coordinates are unscaled and relative to the text layout's top-left. Resolving a run back to its source entity now requires `ComputedTextBlock::entities()` indexed by `span_index`.
  > The `run_geometry` field contains the non-glyph layout geometry for a run of glyphs: the run's span index, bounding rectangle, underline position and thickness, and strikethrough position and thickness. A run is a contiguous glyph sequence sharing text attributes. The coordinates are unscaled and relative to the top left corner of the text layout. Unlike `section_rects` tuples, `RunGeometry` does not include an `Entity` id. To find the corresponding text entity, call the `entities` method on the root text entity's `ComputedTextBlock` component and use the `span_index` to index into the returned slice.
- **[supporting]** In Bevy 0.18, UI picking on `Text` nodes only hits the glyph areas — the node's padding/background region no longer registers pointer hits — so a clickable status chip built as a `Text` node with padding must wrap the text in an intermediate parent node to recover a full-rectangle hit target.
  > Only the sections of `Text` node's containing text are pickable now, the non-text areas of the node do not register pointer hits. To replicate Bevy 0.17's picking behavior, use an intermediate parent node to intercept the pointer hits.

### https://github.com/bevyengine/bevy/issues/10553
quality: **primary** · published: 2023-11-14 (issue opened); state observed as still open on 2026-07-31

- **[central]** In bevy_ui, text and images are handled asymmetrically under HiDPI scale factors: text is re-rendered at the larger physical size and stays crisp, while images are merely upscaled and become blurry. This means Bevy's DPI handling is a solved problem for HUD *text* but an unsolved one for HUD *iconography / nine-patch chrome* — directly relevant to whether a status-chip panel should encode state with glyphs vs. sprite icons on a Retina macOS target.
  > The out-of-the-box behavior is that text sizes are increased (correctly, resulting in crisp text rendered at high resolution), but images are upscaled (making them blurry).
- **[supporting]** As of this fetch (2026-07-31), the issue is still OPEN with labels A-UI / C-Feature and no milestone assigned, having been filed 2023-11-14 — i.e. roughly 2.7 years and many releases (0.12 → 0.18) with no resolution and no scheduled fix. Bevy therefore still ships no built-in mechanism for resolution-variant UI images.
  > Allow using higher-res images in UI for scaling factors > 1.0, so UI does not look blurry/pixelated
- **[supporting]** Bevy's UI scaling is designed to be transparent to the application: the effective scale is composed from the OS window scale factor plus the `UiScale` resource, and the engine applies it automatically rather than exposing per-asset resolution selection.
  > Bevy aims to transparently handle scaling (from window scaling factor + UiScale).
- **[supporting]** The only workarounds available to users are hand-rolled: authoring multiple size variants of each image asset and writing a custom system that undoes Bevy's automatic scaling to pick the right handle. There is no first-party @2x/asset-variant pipeline.
  > create different variants of my assets: small-size (to use with scale = 1.0) and large-size (to use with scale > 1.0)
- **[supporting]** Crispness of Bevy UI on HiDPI displays is acknowledged in the project's own tracker as difficult to achieve for non-text UI content — a known defect rather than a misuse of the API.
  > It is currently very difficult to make a UI (with images, not just text) that looks crisp on HIDPI displays.

### https://github.com/bevyengine/bevy/issues/16689
quality: **primary** · published: 2024-12-06 (issue opened); last activity 2025-07-14; still open as of fetch on 2026-07-31

- **[central]** Bevy 0.15 introduced a regression in which the built-in screenshot API no longer captured bevy_egui-drawn UI, although it did in Bevy 0.14 — i.e. a third-party immediate-mode UI layer can be silently absent from committed screenshot evidence. (Version-stamped: reported against Bevy 0.15, 2024-12-06; labeled P-Regression by Bevy maintainers.)
  > ## Bevy version\n\n0.15\n\n## What you did\n\nI ported my application to Bevy 0.15. Now the screenshot does not contains the `bevy_egui` GUI anymore. It did in Bevy 0.14.\n\n## What went wrong\n\nI was expecting the screenshot to capture everything in the primary window like it was in 0.14.
- **[central]** The defect was never fixed in Bevy itself: it was resolved downstream in bevy_egui 0.35 (reported 2025-07-14), and the Bevy issue is still in state `open` with labels C-Bug / A-Rendering / P-Regression / S-Needs-Investigation and no activity since that date. This implies any bevy_egui release older than 0.35 has broken screenshot capture, and that the correctness of UI-in-screenshot depends on the UI crate's version, not Bevy's.
  > The issue was resolved on the bevy_egui end (fixed in 0.35). I guess we can close this now
- **[central]** The mechanism is that Bevy's screenshot path redirects the window's render target away from the swapchain to an alternative texture so the result can be copied to CPU memory; therefore anything drawn directly to the window swapchain (as bevy_egui was doing) is excluded from the capture. This is documented in Bevy's own source comment on `ExtractedWindow::swap_chain_texture_view`, quoted by the bevy_egui maintainer in the thread.
  > /// Note: this will not always be the swap chain texture view. When taking a screenshot,\n /// this will point to an alternative texture instead to allow for copying the render result\n /// to CPU memory.\n pub swap_chain_texture_view: Option<TextureView>,
- **[supporting]** Bevy maintainer alice-i-cecile attributed the failure to bevy_egui rendering outside Bevy's normal render-graph model rather than to a defect in the screenshot API itself — evidence that UI drawn through Bevy's own render graph (i.e. bevy_ui) is the path the screenshot API is designed to capture, and that bypassing it is the risk factor.
  > > Or maybe bevy_egui just shouldn't render to a window's swapchain texture directly?\n\nThis is my expectation, but I'm far from a rendering expert. It feels like you're stepping outside of Bevy's preferred rendering model by doing that.
- **[supporting]** No application-level scheduling workaround existed: the reporter tried reordering systems so the `Screenshot` spawn occurred after GUI drawing and after bevy_egui internal systems, and no ordering fixed it — so this failure class cannot be worked around by the app author and must be fixed in the UI crate or engine.
  > I tried to change the system order to make sure the command to spawn the `Screenshot` component was done after drawing the GUI and after some `bevy_egui` internal systems, and every order I thought could fix the issue but without any luck.

### https://github.com/bevyengine/bevy/issues/15805
quality: **primary** · published: 2024-10-09 (opened; last updated 2024-10-21; still open as verified 2026-07-31)

- **[central]** After Bevy's text rework, a single logical run of text is no longer one component: it can live on a `Text` component, a `Text2d` component, or be spread across several child entities carrying `TextSpan`, so any system that must react to "the text changed" has to query multiple component types and traverse the hierarchy (checking the parent for narrowing markers when working from spans). This is the reported cause of the change-detection pain.
  > With the new text stuff, one thing that seems a little rough is that you may have a Text component (with text) or a Text2d or several children with spans. So if you need to "do something when text changes" you have a bunch of different places you have to look for the actual string. With spans, you also have to check the parent if you need components there (like narrowing markers, etc).
- **[central]** There is no unified API today for detecting text changes; the only remedy is hand-rolling multiple systems per case. Bevy maintainers acknowledged this in Oct 2024 and proposed a custom `SystemParam` (alice-i-cecile/ItsDoot) or encoding the information into `ComputedTextBlock` (cart), but both remain unimplemented sketches — the issue carries `S-Ready-For-Implementation` with no milestone and one comment.
  > @ItsDoot and I feel that this could probably be solved with a nice custom SystemParam type that gathers and presents this information.
- **[supporting]** The Bevy project itself classifies the post-rework text change-detection ergonomics as a usability *regression* — not merely a missing feature — via the maintainer-applied labels on this issue (`C-Usability`, `P-Regression`, `A-Text`, `D-Modest`), and it was filed by Bevy's lead maintainer alice-i-cecile rather than an outside user.
  > "labels":["C-Usability","P-Regression","S-Ready-For-Implementation","A-Text","D-Modest"],"milestone":null,"number":15805,"state":"open","title":"Change detection for text is more painful after text rework","user":"alice-i-cecile"
- **[central]** The defect persists into the Bevy 0.18 era: as verified against the GitHub API on 2026-07-31, the issue is still open (`closed_at: null`) with last activity 2024-10-21 — roughly 21 months untouched, spanning the 0.15 through 0.18 release cycles. A 0.18 HUD that needs per-frame text updates or per-span styling must therefore assume the hand-rolled multi-system pattern, not a first-party abstraction.
  > "closed_at":null,"comments":1,"created_at":"2024-10-09T23:34:29Z", ... "state":"open", ... "updated_at":"2024-10-21T22:17:00Z"
- **[supporting]** The concrete workflow the issue names as broken is exactly the HUD-shaped one — reacting downstream to a text mutation (its example is re-localization) requires, per case, "multiple systems that cover all the ways there could be a string" rather than a single change-detection query. Note the issue is scoped to change-detection *ergonomics*; it makes no claim about layout-recompute cost on mutation, so it cannot be cited as evidence about re-layout expense.
  > This makes doing change detection on text (for things like "if the text changed, re-localize it") a bit messy. It looks like I'm going to need, for any particular case, multiple systems that cover "all the ways there could be a string squirreled away in there"

### https://docs.rs/bevy/0.18.0/bevy/feathers/index.html
quality: **primary** · published: 2026-01-13 (bevy 0.18.0 crate release date per docs.rs; docs auto-generated from crate source at that version)

- **[central]** Bevy 0.18's first-party styled widget set, bevy_feathers, is self-declared experimental and unfinished, with breaking changes, bugs, and limitations expected — i.e. it is not a stable foundation to build a production instrument panel on.
  > All that said, this crate is still experimental and unfinished! It will change in breaking ways, and there will be both bugs and limitations.
- **[central]** bevy_feathers is explicitly NOT intended for game/application UI; the maintainers' stated recommendation is to copy its code into your own project and adapt it, rather than depend on the crate. This makes it a reference implementation and source of patterns, not a widget library to adopt for a HUD.
  > While it may be tempting to use this crate for your game's UI, it's deliberately not intended for that. [...] Consider copying this code into your own project, and refining the styles and abstractions provided to meet your needs.
- **[central]** Access to bevy::feathers in the bevy facade crate at 0.18.0 requires opting into the non-default cargo feature `experimental_bevy_feathers`, so it is unavailable to a project depending on plain `bevy = "0.18"` without a features change.
  > This module requires the `experimental_bevy_feathers` feature to be enabled.
- **[supporting]** Bevy 0.18's first-party widget offering is split into two layers with different maturity: `bevy::ui_widgets` provides standard widgets and carries no feature-gating banner on the crate root module list, while the experimental, gated `bevy::feathers` supplies the styling/theming layer (theme, tokens, palette, inheritable font_styles, rounded_corners, cursor). Conflating the two misstates what is actually available unfeatured.
  > [ui_widgets](ui_widgets/index.html) - "This crate provides a set of standard widgets for Bevy UI, such as buttons, checkboxes, and sliders." [...] `font_styles` - A framework for inheritable font styles. [...] `rounded_corners` - Mechanism for specifying which corners of a widget are rounded. [...] `theme` - A framework for theming. [...] `tokens` - Design tokens used by Feathers themes.
- **[supporting]** Bevy 0.18.0 was published 2026-01-13 and is not the current release as of July 2026 — docs.rs lists 0.18.1 (2026-03-04) and 0.19.0 (2026-06-19), so the brief's premise that 0.18 is 'current as of mid-2026' is one minor version behind head.
  > The docs.rs page shows bevy 0.18.0 was released on "2026-01-13". [...] 0.19.0 (2026-06-19) [...] 0.18.1 (2026-03-04) [...] 0.18.0 (2026-01-13)

### https://github.com/IyesGames/iyes_perf_ui
quality: **primary** · published: 2025-05-20 (latest release 0.5.0 and last commit to main; repo created 2024-03-19; README undated but current as of last push)

- **[central]** iyes_perf_ui does NOT support Bevy 0.18. Its own README compatibility table tops out at Bevy 0.16 (plugin version 0.5); there is no row for 0.17 or 0.18. As of 2026-07-31 the crate is two Bevy releases behind, so it is not a usable option for the vivarium explorer's HUD without forking and porting it.
  > Bevy Compatibility: | Bevy Version | Plugin Version | |--------------|----------------| | `0.16` | `0.5` | | `0.15` | `0.4` | | `0.14` | `0.3` | | `0.13` | `0.2`,`0.1` |
- **[central]** The crate is effectively dormant: latest release is 0.5.0 published 2025-05-20, and the last commit to the default branch `main` is also 2025-05-20 12:15:28 UTC ('Update changelog', following 'bump bevy to 0.16 (#32)'). GitHub reports pushed_at = 2025-05-20 — roughly 14 months of no code activity as of 2026-07-31. The repo is NOT archived (archived: false, 11 open issues, 235 stars), so it is unmaintained-in-practice rather than formally abandoned; nobody has ported it across the 0.17 or 0.18 UI reworks.
  > | 0.5.0 | 2025-05-20 | 15,613 | … **Most Recent Commit Date:** May 20, 2025 at 12:15:28 UTC
- **[supporting]** iyes_perf_ui is built entirely on bevy_ui rather than egui or any third-party immediate-mode toolkit — direct evidence that a retained-mode, bevy_ui-native debug/status overlay with per-entry widgets is an architecturally viable pattern, independent of whether this particular crate is usable at 0.18. Its `PerfUiWidget` trait / custom-entry trait design is a reference pattern for building status chips from scratch.
  > "Made with Bevy UI (not egui or any other 3rd-party UI solution)"
- **[supporting]** The dominant per-frame cost of a bevy_ui debug overlay is UI *layout*, not text rasterization: a full overlay adds a few hundred microseconds of frame time, 'most of which is CPU time spent in Bevy's UI layout systems (in `PostUpdate`)'. This directly supports designing a HUD that avoids reflow — fixed-position chips whose content changes without changing node geometry — rather than one rebuilding a reflowing text block each frame.
  > "Displaying the Perf UI might add non-negligible overhead to your app, depending on configuration. … A 'full' UI with all the entries offered by this crate can add a few hundred microseconds of frame time on typical gaming hardware, most of which is CPU time spent in Bevy's UI layout systems (in `PostUpdate`)."
- **[supporting]** Threshold-driven visual alerting (changing a value's color or font when it crosses a bound) is a shipped, bevy_ui-implementable pattern rather than something requiring a custom renderer; and bevy_ui text requires an explicit font asset whenever Bevy's built-in default-font feature is disabled — relevant to the vivarium requirement of loading a custom font for crisp small text.
  > "Support for highlighting values using a custom font or color!" … "if you disable Bevy's built-in default font, you must specify the fonts that you would like to use"

### https://docs.rs/bevy_feathers/0.18.1
quality: **primary** · published: 2026-03-04

- **[central]** A first-party Bevy widget crate does exist and is current for 0.18: bevy_feathers 0.18.1 was published 2026-03-04 and depends on bevy_ui, bevy_ui_widgets, bevy_text, and bevy_a11y at ^0.18.0, so version currency is not the blocker for adopting it.
  > Version: 0.18.1 — Published: "2026-03-04" ... Dependencies: accesskit ^0.21, bevy_a11y ^0.18.0, bevy_app ^0.18.0, ... bevy_text ^0.18.0, bevy_ui ^0.18.0, bevy_ui_render ^0.18.0, bevy_ui_widgets ^0.18.0
- **[central]** bevy_feathers explicitly disclaims use as a game/application UI layer and instructs consumers to copy-and-refine its source rather than depend on the crate — meaning the first-party widget set is not a supported foundation for a product HUD.
  > While it may be tempting to use this crate for your game's UI, it's deliberately not intended for that.
- **[central]** The crate is self-declared experimental with promised breaking changes, bugs, and limitations, so depending on it directly carries known churn risk across Bevy releases.
  > This crate is still experimental and unfinished! It will change in breaking ways, and there will be both bugs and limitations.
- **[central]** The entire controls inventory is editor/inspector input widgets — button, checkbox, radio, slider, color slider, color swatch, color plane, toggle switch, virtual keyboard — with no status chip, panel, banner, toast, notification, scrollbar, or text-input widget; the instrument-panel design target (mode chip, freshness alarm, sticky banner, transient toast) has zero direct coverage in feathers.
  > `button()` - "Template function to spawn a button"; `checkbox()`; `color_plane()` - "Template function to spawn a 'color plane', which is a 2d picker"; `color_slider()`; `color_swatch()`; `radio()`; `slider()`; `toggle_switch()`; `virtual_keyboard()` ... The module includes a virtual keyboard function but lacks explicit documentation for scrollbars, text input fields, or panel components in this controls namespace.
- **[supporting]** What feathers does offer as reusable infrastructure — rather than widgets — is a theming stack (theme, tokens, palette, dark_theme), an inheritable font-styling framework (font_styles), rounded_corners, and cursor management, plus an accesskit ^0.21 dependency indicating AccessKit accessibility plumbing is wired through the first-party UI stack at 0.18.
  > **font_styles** – Inheritable font styling framework ... **rounded_corners** – Corner rounding specifications ... **theme** – Theming framework ... **tokens** – Design tokens ... plus `accesskit` (^0.21) and `smol_str` (^0.2)

### https://github.com/laundmo/bevy_screen_diagnostics
quality: **primary** · published: Latest release 0.8.1 published 2025-04-26; last commit to main 2025-04-26; repo metadata fetched 2026-07-31 (README on main branch is undated)

- **[central]** bevy_screen_diagnostics does NOT support Bevy 0.18 (or 0.17). Its README compatibility table on the `main` branch tops out at Bevy 0.16 → crate 0.8.1, two Bevy major releases behind the version vivarium pins. Using it on 0.18 would require forking and porting.
  > Bevy Compatibility: | Bevy | bevy_screen_diagnostics | | 0.16 | 0.8.1 | | 0.15 | 0.7 | | 0.14 | 0.6 | | 0.13 | 0.5 | | 0.12 | 0.4 | | 0.11 | 0.3 | | 0.10 | 0.2 | | 0.9 | 0.1 |
- **[central]** The crate is effectively dormant: latest and only current release is 0.8.1, published 2025-04-26T22:24:02Z (15 months before 2026-07-31), and the last commit to `main` is 2025-04-26T22:22:00Z. crates.io lists no version after 0.8.1. The repo is not archived (archived: false, 111 stars, 4 open issues), so it is stale rather than formally abandoned.
  > Max Version: 0.8.1 | Newest Version: 0.8.1 | Updated At: 2025-04-26T22:24:02.551786Z … pushed_at: 2025-04-26T22:22:04Z … archived: false … stargazers_count: 111
- **[central]** A community-authored port to Bevy 0.17 exists but has sat unmerged: PR #16 "Upgrade to Bevy 0.17" was opened 2025-10-02 and remains open with no maintainer merge; no Bevy 0.18 PR exists at all. Every prior version bump (0.13, 0.14, 0.15, 0.16) also arrived as an outside contributor PR, so upgrade cadence depends on drive-by contributions plus a maintainer merge that has now stalled.
  > | 16 | "Upgrade to Bevy 0.17" | open | 2025-10-02 | PR | Bevy 0.17 support - 3 eye reactions, not yet merged … | 15 | "Upgrade to Bevy 0.16.0" | closed | 2025-04-19 | PR | Merged 2025-04-24
- **[supporting]** Functionally the crate is a narrow engine-telemetry overlay, not an instrument-panel toolkit: it ships exactly three plugins — frame rate/frame time, entity count, and (behind the `sysinfo_plugin` feature) CPU/memory — plus a hook for user-registered diagnostics. It offers no chips, alarms, banners, toasts, focus regions, or layout primitives, so it does not address vivarium's stated design target.
  > Available Plugins: 1. `ScreenDiagnostics` - core display system 2. `ScreenFrameDiagnosticsPlugin` - framerate/frametime metrics 3. `ScreenEntityDiagnosticsPlugin` - entity count tracking 4. `ScreenSystemInformationDiagnosticsPlugin` - CPU/memory usage (requires `sysinfo_plugin` feature)
- **[supporting]** It renders through Bevy's own text stack rather than an immediate-mode layer, defaulting to Bevy's bundled stripped FiraCode via a `builtin-font` default feature, with custom fonts loadable by asset path. This means it inherits (and cannot fix) whatever bevy_ui text crispness/DPI behavior exists on the target Bevy version — it is not an independent text-rendering solution.
  > This crate uses bevy's default font (a stripped version of FiraCode) through the `builtin-font` default feature.

### https://github.com/bevyengine/bevy/issues/20590
quality: **primary** · published: 2025-08-15

- **[supporting]** Bevy exposes `Camera::world_to_viewport` as the built-in world-space-to-screen-space projection primitive (the API one would use to anchor a HUD label or off-screen indicator to a point on a 3D globe), but its returned coordinates are render-target-relative rather than viewport-relative — the issue title states this as the defect.
  > `Camera::world_to_viewport` returns coordinates relative to render target instead of viewport
- **[supporting]** The coordinate-space defect is conditioned on the camera having a non-zero viewport origin: in that case the returned position has the viewport's `target_rect.min` offset added to it. (Corollary: for a full-window camera whose viewport origin is (0,0) the two spaces coincide and the bug is inert; the manual workaround is to subtract `target_rect.min`.)
  > I was using `Camera::world_to_viewport` for a camera that has a non-zero viewport position. ... The results I were getting had `target_rect.min` (viewport position) added to them.
- **[supporting]** As of 2026-07-31 the issue is still OPEN with zero comments, no linked or referencing pull requests, and no timeline activity since the day it was filed (2025-08-15) — i.e. the behavior reported against 0.16 was never fixed through the 0.18 release cycle. (Verified directly against the GitHub API: `state: open`, `closed_at: null`, `comments: 0`, `updated_at: 2025-08-15T17:52:14Z`.)
  > "closed_at":null,"comments":0,"created_at":"2025-08-15T12:52:11Z","labels":["C-Bug","A-Rendering","S-Needs-Design"],"number":20590,"state":"open","updated_at":"2025-08-15T17:52:14Z"
- **[supporting]** Bevy maintainers triaged this the same day and classified it `S-Needs-Design` (not a straightforward bug fix), signalling that the fix requires a semantics/API decision — so a consumer should treat the current render-target-relative behavior as the de facto contract rather than expect it to be corrected, and should also expect the semantics to potentially change under them in a future release.
  > {"actor":"alice-i-cecile","created_at":"2025-08-15T17:52:14Z","event":"labeled","label":"S-Needs-Design"}
- **[tangential]** The report is filed against Bevy 0.16, so its version stamp predates 0.18 by two releases; the claim that it still applies to 0.18 rests on the absence of any fix in the issue timeline rather than on a 0.18-version test in the source itself.
  > ## Bevy version and features Bevy 0.16

### https://github.com/bevyengine/bevy/issues/15197
quality: **primary** · published: 2024-09-14 (issue opened); closed 2024-10-28 by PR #16097, merged into milestone 0.15

- **[supporting]** The "wiggling text" defect is real and reproducible in Bevy's own `text_debug` UI example (filed 2024-09-14 against main @1fd478277, confirmed by a second reporter on a different distro), but it is FIXED and not a live 0.18 defect: the issue was closed 2024-10-28 by PR #16097 ("Remove custom rounding"), milestone **0.15**. Anything citing this issue as a current bevy_ui limitation is version-stale by three releases.
  > Texts that are on right wiggle one pixel back and forth horizontally. ... Not sure if it is related to the texts at the bottom that move around a lot because of the FPS and frame time numbers. You would think the text in the top right would stay still but it still wiggles.
- **[central]** The title's diagnosis (and a maintainer's guess of "a one-frame delay" from system ordering) was WRONG. The actual root cause was UI layout rounding: Bevy had disabled Taffy's built-in rounding and substituted ad-hoc hacks that rounded only to the nearest *logical* pixel, whereas Taffy rounds to the nearest *physical* pixel. PR #16097 deleted the hacks and re-enabled Taffy rounding. (Verified still in force on `main` today: `bevy_ui::ui_node::LayoutConfig { use_rounding: bool }` exists as a per-subtree opt-out, `Default` = `true`, documented as rounding "to the nearest physical pixel ... can help prevent visual artifacts like blurry images or semi-transparent edges that can occur with sub-pixel positioning.")
  > Taffy added layout rounding a while ago but it had a couple of bugs and caused some problems with the fussy `ab_glyph` text implementation. So I disabled Taffy's builtin rounding and added some hacks ad hoc ... Since then though Taffy's rounding algorithm has improved while we've changed layout a lot and migrated to `cosmic-text` so those hacks don't help any more and in some cases cause significant problems. Also our rounding implementation only rounds to the nearest logical pixel, whereas Taffy rounds to the nearest physical pixel meaning it's much more accurate with high dpi displays.
- **[central]** The mechanism that triggered the visible jitter is precisely the pattern in vivarium's HUD: a UI text node whose measured size changes between frames because the digits inside it change. Auto-sized, right-aligned, or sibling-influencing nodes with per-frame-varying content are the destabilizing construct — the fix for the engine bug removes the 1px rounding artifact, but the underlying layout-width churn (and its effect on siblings and wrapping) is a property of the content pattern, not of the bug. Fixed-width / fixed-position chips with reserved digit slots sidestep it structurally.
  > The two text blocks here are aligned right to the same boundary but in this screen shot you can see that the lower block is one pixel off to the left. Because the size of this text node changes between frames with the reported framerate the rounding errors cause it to jump left and right.
- **[central]** bevy_ui layout-rounding artifacts scale with `scale_factor` and can be entirely invisible at scale factor 1 — meaning development or CI on a 1x display will not surface them, while a macOS Retina / fractional-DPI screenshot capture is the worst case. Any crispness or layout-stability verification for committed captures must be done at the actual capture scale factor, not at 1x.
  > These errors are much more obvious at high scale factor, you might not see any problems at a scale factor of 1.
- **[supporting]** A residual, separately-caused rendering defect was left unfixed by PR #16097: hairline gaps between a node's border and its content, attributed to rendering rather than layout, and eliminated by setting `UiAntiAlias::Off`. This identifies `UiAntiAlias` as a real, engine-level lever affecting edge crispness of bordered chrome (borders/panels), and flags bordered UI boxes as a known artifact site for pixel-exact captures.
  > This doesn't fix some of the gaps that occur between borders and content but they seem appear to be a rendering problem as they disappear with `UiAntiAlias::Off` set.

### https://github.com/vladbat00/bevy_egui
quality: **primary** · published: 2026-07-18 (latest release 0.41.1; crates.io updated_at 2026-07-18T11:21:22Z; README on main reflects the 0.41 line — the repository is continuously updated, so this is a last-modified date, not a one-time publish date)

- **[central]** bevy_egui's official compatibility table maps Bevy 0.18 to exactly one bevy_egui line — 0.39 — and that line is frozen: 0.39.1 (2026-02-06) is its last release, while the maintainer has moved development to 0.40/0.41 for Bevy 0.19. A vivarium pinned to bevy = "0.18" can therefore only use a ~5.5-month-old egui integration that will receive no further fixes short of a Bevy upgrade.
  > Bevy compatibility table: "| bevy | bevy_egui |" … "| 0.19 | 0.40-0.41 |", "| 0.18 | 0.39 |", "| 0.17 | 0.37-0.38 |". crates.io versions: "0.41.1 | 2026-07-18", "0.41.0 | 2026-06-30", "0.40.1 | 2026-06-24", "0.40.0 | 2026-06-19", "0.40.0-rc.1 | 2026-05-29", "0.39.1 | 2026-02-06", "0.39.0 | 2026-01-14".
- **[central]** Bevy 0.19 exists and shipped before ~2026-06-20 — the brief's premise that Bevy 0.18 is 'current as of mid-2026' is stale by one major release. bevy_egui's 0.40.0 release notes explicitly record the port, and its rc.1 dates to 2026-05-29, placing Bevy 0.19's release in roughly late May / June 2026.
  > Release [0.40.0] - 20-Jun-2026 — Changed: "Update to Bevy 0.19 and egui 0.34". Release [0.39.0] - 14-Jan-2026 — Changed: "Update to Bevy 0.18".
- **[supporting]** The exact dependency surface a Bevy 0.18 project inherits from bevy_egui is bevy ^0.18.0 with egui ^0.33 (plus winit ^0.30, crossbeam-channel, image ^0.25.5) — i.e. adopting egui on 0.18 also pins you two egui minor versions behind current (egui 0.35 as of bevy_egui 0.41.0), so egui-side fixes and features land only after a Bevy upgrade.
  > bevy_egui 0.39.1 dependencies: "bevy ^0.18.0" (normal, dev), "egui ^0.33" (normal, dev), "winit ^0.30", "crossbeam-channel ^0.5.8", "image ^0.25.5"; release date "2026-02-06". Compare release [0.41.0] - 30-Jun-2026 — Changed: "Updated to egui 0.35".
- **[supporting]** bevy_egui's DPI/scale-factor handling changed materially in the Bevy 0.19 line: 0.40.0 stopped driving egui scaling through set_pixels_per_point and switched to native_pixels_per_point. A Bevy 0.18 project on 0.39.x is therefore stuck on the older scaling path — directly relevant to crisp small text and deterministic screenshot layout on macOS Retina / fractional DPI.
  > Release [0.40.0] - 20-Jun-2026 — Changed: "Plugin no longer uses `set_pixels_per_point` for scaling; uses `native_pixels_per_point` instead".
- **[supporting]** bevy_egui advertises capabilities that map onto several instrument-panel needs — AccessKit accessibility plumbing, multi-window/split-screen contexts, paint callbacks for custom rendering, and render-to-texture (an official `render_egui_to_image` example) — and its current API is schedule-based (`EguiPrimaryContextPass`) with a fallible context accessor (`contexts.ctx_mut()?` returning `Result`), so any blog-era snippet using a plain infallible `ctx_mut()` in `Update` is out of date.
  > Features: "Desktop and web platforms support, Clipboard, Opening URLs, Multiple windows support and split-screen support, Paint callback support, Mobile web virtual keyboard, Accesskit support." Usage example: "use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};" … ".add_systems(EguiPrimaryContextPass, ui_example_system)" … "egui::Window::new(\"Hello\").show(contexts.ctx_mut()?, |ui| { ui.label(\"world\"); });"

### https://crates.io/crates/bevy_ui_anchor
quality: **primary** · published: 2026-07-08 (latest release 0.12.0; crate first published 2024-08-07; crates.io record updated 2026-07-08T12:26:11Z)

- **[central]** For Bevy 0.18 the correct pin is bevy_ui_anchor **0.11.0** (published 2026-02-15), NOT the latest release. The newest version, 0.12.0 (published 2026-07-08), declares a normal dependency of bevy `^0.19.0` and therefore will not compile against Bevy 0.18. Confirmed three ways: the crates.io dependencies API for 0.12.0 returns req `^0.19.0` (features bevy_ui, bevy_window, bevy_log), the same API for 0.11.0 returns req `^0.18.0`, and docs.rs for 0.12.0 lists `bevy ^0.19.0`. Collateral finding that bears on the whole research run: a crate released 2026-07-08 targeting Bevy 0.19 is direct evidence that **Bevy 0.19 shipped before mid-July 2026**, which contradicts the brief's stated premise that 0.18 is the current version — the 0.18-vs-0.19 question should be re-checked against the Bevy repo before any API-level conclusion in this research is treated as current.
  > Bevy Compatibility | Bevy | Crate | 0.19 | 0.12 | 0.18 | 0.11 | 0.17 | 0.10 | 0.16 | 0.6-0.9 | 0.15 | 0.3-0.5 | 0.14 | 0.1-0.2 — and from the crates.io dependencies API for 0.12.0: "Crate ID: bevy — Version requirement: ^0.19.0 — Kind: normal"; for 0.11.0: "Version requirement: ^0.18.0"
- **[central]** The crate supplies exactly the world-space-to-screen-space anchoring primitive the research question asks about ("world-space-to-screen-space projection for anchoring a label or an off-screen arrow to a point on the globe"), and it does so for real `bevy_ui` nodes rather than Text2d/billboards — meaning the HUD chips, labels and region markers can stay in the same flexbox/Node layout system as the rest of the panel. `AnchorUiNode` continuously drives a UI node's screen position from either a raw world position or another entity's GlobalTransform; `AnchorUiConfig` carries `anchorpoint` and `offset`; `AnchorPoint` plus `HorizontalAnchor`/`VerticalAnchor` select which corner/edge of the UI node is pinned (e.g. `bottomleft()`); `AnchoredUiNodes` is a relationship component tracking which UI nodes anchor to a given entity.
  > `AnchorUiNode` – Continuously updates UI screen location to match an in-world position or another entity's GlobalTransformation … `AnchorUiConfig` – Configures how the UI anchors to the entity … `AnchorPoint` – Defines which point on the UI element serves as the anchor … `AnchoredUiNodes` – Relationship tracking which UI nodes anchor to an entity
- **[supporting]** The crate is actively maintained and tracks Bevy's release cadence closely, which is the maintenance-status fact the brief demanded for every named crate. It has 21 published versions spanning 2024-08-07 to 2026-07-08 with no yanks, MIT-licensed throughout, and a released version corresponding to every Bevy release from 0.14 through 0.19 — i.e. it has not fallen behind the way many bevy_ui ecosystem crates have. Repository: https://github.com/TotalKrill/bevy_ui_anchor.
  > 0.12.0 | 2026-07-08 | MIT | No — 0.11.0 | 2026-02-15 | MIT | No — 0.10.0 | 2025-10-01 | MIT | No — 0.9.0 | 2025-08-11 | MIT | No … Created: 2024-08-07T20:01:07.687772Z, Updated: 2026-07-08T12:26:11.364615Z
- **[supporting]** Integration constraint worth knowing before adoption: the plugin does not infer the projection camera. A camera must be tagged with a marker component (`UiCameraMarker`) so the plugin knows which camera to project world positions through — relevant for any app with more than one camera, and a thing that will silently produce no anchoring if omitted.
  > `UiCameraMarker`: Required component to identify which camera performs position calculations … The plugin requires marking a camera with a marker component so "the plugin knows which camera to perform position calculations towards."
- **[supporting]** Adoption and documentation are both thin, so budget for reading the source rather than the docs: docs.rs reports only 38.89% of the crate documented, and crates.io reports 21,821 all-time downloads with 668 recent — small even by Bevy-ecosystem standards. Notably, neither the crates.io metadata nor the README (as retrieved) documents any off-screen clamping or edge-indicator behavior, so the brief's stated wish for "an off-screen indicator pointing toward a selected region that has scrolled out of view" is NOT confirmed as a provided feature by this source and should be treated as unverified/likely hand-rolled on top of the anchoring primitive.
  > Documentation Coverage: 38.89% of the crate is documented … Downloads: 21,821 total | 668 recent

### https://bevy.org/learn/migration-guides/0-18-to-0-19/
quality: **primary** · published: No publish date is displayed on the page (fetched 2026-07-31). The page is the official bevy.org migration guide for 0.18 → 0.19; bevy.org builds these guides incrementally during a dev cycle, so the content describes the next release relative to 0.18. No explicit "in progress / unreleased" banner was visible in the fetched content — treat the absence of a date as a limitation, not as evidence the guide is final.

- **[central]** Bevy 0.18's text stack is Cosmic Text (cosmic-text); Bevy 0.19 replaces it wholesale with Parley. This confirms the brief's suspicion about 0.18's text backend AND establishes that any typography work built on 0.18 sits on a stack the engine is about to swap out — glyph/shaping/font-selection behavior, and any code touching PositionedGlyph or TextPipeline font IDs, is version-fragile. Corroborating detail from the same guide: in 0.19 `PositionedGlyph`'s `byte_index` and `byte_length` fields are removed because Parley does not expose them, and `map_handle_to_font_id`/`get_font_id` are removed from `TextPipeline`.
  > bevy_text migration from Cosmic Text to Parley
- **[central]** In Bevy 0.18, `TextFont::font` is a `Handle<Font>` and `TextFont::font_size` is a plain `f32` (raw pixels only). 0.19 changes `font` to a `FontSource` enum (Handle or Family) and `font_size` to a `FontSize` wrapper with variants such as `FontSize::Px(35.)`. Practical consequence for 0.18 HUD work: font size is a bare float with no unit type, there is no family-name-based font selection (you must load and hold an asset Handle), and there is no rem/relative sizing — 0.19 adds a rem concept (evidenced by the new `is_rem_size_changed` parameter on `ComputedTextBlock` rerender).
  > TextFont's font field has been changed from a Handle<Font> to a FontSource, and its font_size field was changed from an f32 to a FontSize. // 0.18: TextFont { font: asset_server.load("...ttf"), font_size: 35. } // 0.19: TextFont { font: asset_server.load("...ttf").into(), font_size: FontSize::Px(35.) }
- **[central]** Bevy's first-party widget/styling set `bevy_feathers` is gated behind a cargo feature literally named `experimental_bevy_feathers` in 0.18 — i.e. it is officially flagged experimental in the version vivarium pins — and is only de-experimentalized (feature renamed to `bevy_feathers`) in 0.19. Additionally, 0.19 moves Feathers widget construction to BSN as the primary API and renames the old spawn functions (`button` → `button_bundle`), so any 0.18 Feathers usage is on an API the project itself is replacing next release.
  > The experimental_bevy_feathers feature is no longer experimental — The feature flag experimental_bevy_feathers is now bevy_feathers. ... Going forward, BSN will be the primary means to create Feathers widgets.
- **[supporting]** In Bevy 0.18, `UiWidgetsPlugins` and `InputDispatchPlugin` are NOT part of `DefaultPlugins` and must be added manually to get the headless widget set and input-focus dispatch working; 0.19 folds them into `DefaultPlugins`. A 0.18 app (like vivarium-explore) that only adds `DefaultPlugins` therefore has no widget/focus machinery running at all.
  > UiWidgetsPlugins and InputDispatchPlugin are now in DefaultPlugins — UiWidgetsPlugins and InputDispatchPlugin are now part of DefaultPlugins.
- **[supporting]** In Bevy 0.18 the headless widget components still carry the `Core` prefix (`CoreScrollbarThumb`, `CoreScrollbarDragState`, `CoreSliderDragState`), and the rich-text composition types are `TextRoot`, `TextSpanAccess` and `TextSpanComponent` with `read_span`/`write_span` accessors. 0.19 drops the `Core` prefix and consolidates the text-span types into a single `TextSection` with `get_text`/`get_text_mut`. Anyone searching docs.rs or older blog posts must match the prefix/type names to the exact version.
  > Core prefix removed from UI widget components — CoreScrollbarThumb has been renamed to ScrollbarThumb. ... TextRoot, TextSpanAccess and TextSpanComponent are replaced by TextSection

### https://docs.rs/bevy_text/0.18.0/bevy_text/
quality: **primary** · published: Not stated on the page (docs.rs auto-generated API docs; version-pinned to bevy_text 0.18.0 — the version stamp, not a publication date, is the currency guarantee here)

- **[central]** Bevy 0.18 DOES expose OpenType feature selection per text span — tabular figures are reachable natively. `TextFont` carries a `font_features: FontFeatures` field, and `FontFeatureTag` includes `TABULAR_FIGURES`, `OLDSTYLE_FIGURES`, `STANDARD_LIGATURES`, plus numeric-valued tags (e.g. `WEIGHT`) via `FontFeaturesBuilder`. This directly answers the brief's open question 'whether tabular/lining figures or OpenType feature selection is reachable at all' — yes, without an ecosystem crate. Caveat stated by the docs: FontFeatures is described as covering '.otf fonts that support them'.
  > FontFeatures ... manages OpenType features for `.otf` fonts ... [handling] "ligatures, small-caps, and fractional number display." ... `let more_font_features: FontFeatures = [FontFeatureTag::STANDARD_LIGATURES, FontFeatureTag::OLDSTYLE_FIGURES, FontFeatureTag::TABULAR_FIGURES].into();` ... From<T> where T: IntoIterator<Item = FontFeatureTag> — "for the standard case when each listed feature is a boolean type. If any features require a numeric value (like \"wght\"), use FontFeaturesBuilder instead." TextFont field: font_features: FontFeatures — "OpenType features for .otf fonts that support them."
- **[central]** Bevy 0.18 offers exactly two text antialiasing modes — grayscale AA (default) and none (pixel-art) — and subpixel/LCD/ClearType antialiasing is explicitly unsupported. `FontSmoothing` has only the variants `None` and `AntiAliased`. For crisp small text over a busy 3D scene there is no font-smoothing knob to tune beyond on/off; crispness must be bought through font choice, integer-aligned physical sizing, and backing contrast rather than through a rendering mode.
  > FontSmoothing variants — None: "No antialiasing. Useful for when you want to render text with a pixel art aesthetic." AntiAliased (Default): "The default grayscale antialiasing. Produces text that looks smooth, even at small font sizes and low resolutions with modern vector fonts." ... "Note: Subpixel antialiasing is not currently supported."
- **[central]** `TextFont::font_size` is defined as the glyph rasterization height in the font atlas in pixels, multiplied by the window scale factor and `UiScale` but NOT by entity transform or camera projection — and a distinct font atlas is generated per (font handle, scaled font size) combination, with an explicit performance warning. Two consequences for a Retina/fractional-DPI macOS instrument panel: DPI is handled automatically for UI text (logical px in, physical rasterization out), and animating or continuously varying font size is a real cost, not a free effect.
  > font_size: f32 — "The vertical height of rasterized glyphs in the font atlas in pixels." ... "This is multiplied by the window scale factor and `UiScale`, but not the text entity transform or camera projection." ... "A new font atlas is generated for every combination of font handle and scaled font size which can have a strong performance impact."
- **[supporting]** bevy_text 0.18 ships per-span text decorations and per-span background color as first-class components — `TextBackgroundColor`, `Underline`/`UnderlineColor`, `Strikethrough`/`StrikethroughColor`, with a `RunGeometry` type backing them — but `TextShadow`, `Text` (the UI component), and `Text2d` are NOT in this crate's public item index (they live in bevy_ui / bevy_sprite respectively). For contrast over arbitrary 3D imagery, per-span `TextBackgroundColor` is available inside bevy_text itself, whereas shadow/outline must be sourced from the UI crate.
  > Structs: ... RunGeometry, Strikethrough, StrikethroughColor, ... TextBackgroundColor, TextColor, ... Underline, UnderlineColor. "TextBackgroundColor: Background color of text for a section" · "RunGeometry: Geometry of a text run used to render text decorations like background colors, strikethrough, and underline" ... Notable absences: TextShadow, Text (UI component), and Text2d do not appear in this crate's public item index.
- **[supporting]** The 0.18 text stack is cosmic-text ^0.16 (with swash rasterization) wrapped as `CosmicFontSystem`, `CosmicBuffer`, and `SwashCache`, and relayout is gated by explicit change detection: `detect_text_needs_rerender` sets `ComputedTextBlock::should_rerender`. This means a per-frame `String` assignment into a `Text` does mark the block dirty and re-run layout — the documented mechanism for avoiding that cost is to not mutate the text block, i.e. split volatile values into separate span entities so only the touched block re-lays-out.
  > Dependencies: ... cosmic-text ^0.16 ... "CosmicFontSystem: Wrapper around cosmic_text's FontSystem" · "SwashCache: Wrapper around cosmic_text's SwashCache" · "CosmicBuffer: Wrapper for cosmic_text::Buffer" · "detect_text_needs_rerender: Detects changes to text blocks and sets ComputedTextBlock::should_rerender" · "FontAtlas: Rasterized glyphs are cached, stored in, and retrieved from, a FontAtlas"

### https://github.com/vleue/bevy_easings
quality: **primary** · published: Repository README undated; version evidence from crates.io — latest release v0.19.0 on 2026-06-24, Bevy-0.18-compatible v0.18.0 on 2026-01-25, crate first published 2020-09-29. Source files read at the `main` branch and the `v0.18.0` tag on 2026-07-31.

- **[central]** bevy_easings has a released version for Bevy 0.18 and is actively maintained past it — the README compatibility table maps Bevy 0.18 → bevy_easings 0.18 and Bevy 0.19 → bevy_easings 0.19, with crates.io showing v0.18.0 published 2026-01-25 and v0.19.0 published 2026-06-24 (crate created 2020-09-29, ~158k total downloads). This is one of the ecosystem crates that is version-current rather than stranded on an old Bevy.
  > | Bevy | bevy_easings | | ---- | ------------ | | main | main | | 0.19 | 0.19 | | 0.18 | 0.18 | | 0.17 | 0.17 |
- **[central]** bevy_easings implements Lerp directly for bevy_ui components — Node, BackgroundColor, TextColor, Val, and UiRect — so HUD chrome (chip background alpha, text color, position/size offsets, padding) can be animated by the crate without hand-rolling per-element interpolation. The UI impls are behind a `ui` cargo feature, and BackgroundColor additionally requires the `render` feature. Verified present in the v0.18.0 tag, not only on main.
  > #[cfg(feature = "ui")] impl Lerp for EaseValue<bevy_ui::Node> #[cfg(all(feature = "ui", feature = "render"))] impl Lerp for EaseValue<bevy_ui::BackgroundColor> #[cfg(feature = "ui")] impl Lerp for EaseValue<bevy_text::TextColor>
- **[central]** The crate provides declarative looping and oscillating animation modes — `EasingType::Loop { duration, pause }` and `EasingType::PingPong { duration, pause }`, both with an optional pause between cycles — which covers the 'blinking or pulsing alert without hand-rolling a timer per element' requirement directly: the repeat behavior is a data field on the component, not a system the caller writes.
  > pub enum EasingType { /// Only happen once Once { /// duration of the easing duration: Duration, }, /// Looping, restarting from the start once finished Loop { /// duration of the easing duration: Duration, /// duration of the pause between two loops pause: Option<Duration>, }, /// Repeat the animation back and forth PingPong { /// duration of the easing duration: Duration, /// duration of the pause before starting again in the other direction pause: Option<Duration>, }, }
- **[supporting]** Easing `Val` between mismatched unit variants (e.g. `Val::Px` → `Val::Percent`, or anything involving `Val::Auto`) silently fails: the match falls through to a catch-all that returns the starting value unchanged, with no panic, warning, or error. For a panel whose defect is layout instability, this is a silent no-op failure mode — an animation that appears to be wired up but never moves, and the diagnosis is a unit mismatch rather than a system-ordering or timing bug.
  > (Val::Percent(self_val), Val::Percent(other_val)) => { ... } (Val::Px(self_val), Val::Px(other_val)) => { ... } // ... other matching variants ... _ => EaseValue(self.0),
- **[supporting]** Interpolation shape is fully overridable beyond the ~30 stock EaseFunction variants: `EaseMethod` exposes `Linear`, `Discrete` (value jumps from start to end rather than interpolating), and `CustomFunction(fn(f32) -> f32)`. `Discrete` in particular gives a hard on/off blink rather than a sinusoidal fade, which is the correct waveform for a high-severity alarm chip that must read as unmissable rather than decorative.
  > pub enum EaseMethod { /// Follow `EaseFunction` EaseFunction(EaseFunction), /// Linear interpolation, with no function Linear, /// Discrete interpolation, eased value will jump from start to end Discrete, /// Use a custom function to interpolate the value CustomFunction(fn(f32) -> f32), }

### https://crates.io/crates/bevy_mod_billboard
quality: **primary** · published: 2024-07-10 (latest release 0.7.0 published 2024-07-10T23:54:35Z; crate first published 2023-03-06; registry metadata unchanged since)

- **[central]** bevy_mod_billboard's newest and only current release, 0.7.0, depends on Bevy 0.14; no release exists supporting any Bevy version above 0.14, making it unusable on Bevy 0.18 without a port. The crate's own compatibility table tops out at Bevy 0.14, and Cargo.toml confirms the pinned dependency.
  > crates.io API: "max_version": "0.7.0", "newest_version": "0.7.0" — Cargo.toml: "**Bevy Dependency Version:** `0.14`" — README compatibility table: "| Bevy Version | Crate Version |" / "| 0.14 | 0.7.0 |"
- **[central]** The crate has had zero development activity since 2024-07-10, over two years stale as of mid-2026. The last commit and the 0.7.0 publish happened the same day; the last substantive commit was the Bevy 0.14 upgrade itself.
  > GitHub API: "pushed_at":"2024-07-10T23:52:30Z" — crates.io API: "updated_at": "2024-07-10T23:54:35.829770Z" — most recent commits: "Update CHANGELOG, README, bump crate version" (2024-07-10T23:52:26Z), "Upgrade bevy to 0.14 (#26)" (2024-07-10T23:46:32Z)
- **[supporting]** The staleness is abandonment, not sufficiency: upgrade requests for Bevy 0.15 and 0.16 sit open and unanswered, the 0.16 one with zero replies since May 2025. The repository is NOT marked archived and carries no unmaintained notice, so it presents as alive to casual inspection (105 stars, 53,043 downloads) despite being dead — a trap for crate selection.
  > Open issues: {"comments":0,"created":"2025-05-02T20:08:05Z","n":34,"title":"Upgrade to Bevy 0.16"} and {"comments":3,"created":"2024-12-06T03:56:50Z","n":30,"title":"Bevy v0.15"} — repo metadata: "archived":false,"open_issues_count":13,"stargazers_count":105 — "downloads": 53043
- **[supporting]** The crate's public API is built on the Bundle pattern (BillboardTextBundle, BillboardTextureBundle), the pre-0.15 Bevy idiom superseded by required components. Porting to 0.18 is therefore a public-API rewrite, not a version-number bump — raising the cost of adopting or forking it.
  > README: "spawn billboards using either `BillboardTextBundle` (with text sections and styling) or `BillboardTextureBundle` (with image handles and mesh definitions)"
- **[supporting]** The capability set this crate provided — and which would have to be rebuilt from scratch or sourced elsewhere for 3D billboard labels anchored to world points — is camera-facing styled text and texture billboards with depth-culling toggles, Y-axis locking, full rotation constraints, and HDR support. Notably, text outline (needed for contrast over arbitrary imagery) was an open unanswered feature request, not a shipped feature.
  > README: "Depth culling toggles, Y-axis locking, full rotation constraints, and HDR support" — open issue: {"created":"2024-12-29T23:41:23Z","n":33,"title":"Is there any way to add outline for text?"}

### https://docs.rs/bevy_ui/0.18.0/bevy_ui/
quality: **primary** · published: 2026-01-13

- **[central]** bevy_ui 0.18.0 ships the full set of instrument-panel chrome primitives first-party, with no ecosystem crate required: BorderRadius, BoxShadow (+ShadowStyle), Outline, Overflow/OverflowAxis/OverflowClipBox/OverflowClipMargin/CalculatedClip/OverrideClip, ZIndex and GlobalZIndex, and a complete gradient system (LinearGradient, RadialGradient, ConicGradient, BackgroundGradient, BorderGradient, ColorStop, AngularColorStop, InterpolationColorSpace). Notably, Outline is explicitly layout-free, so a focus/alarm ring can be added or removed without shifting any sibling chip.
  > Outline: "adds an outline outside the edge of a UI node. Outlines do not take up space in the layout." · BoxShadow: "List of shadows to draw for a Node." · BorderRadius: "Used to add rounded corners to a UI node." · GlobalZIndex: "allows a Node entity anywhere in the UI hierarchy to escape the implicit draw ordering." · All Items structs include: BackgroundColor, BorderColor, BorderRadius, BoxShadow, CalculatedClip, ... Outline, Overflow, OverflowClipMargin, OverrideClip, ... ShadowStyle, ... AngularColorStop, BackgroundGradient, BorderGradient, ColorStop, ConicGradient, LinearGradient, RadialGradient
- **[central]** bevy_ui 0.18.0 has a dedicated 2D UI-space transform — UiTransform { translation: Val2, scale: Vec2, rotation: Rot2 } — which is auto-inserted as a required component of every Node, with UiGlobalTransform auto-inserted alongside it. This means pulsing/scaling/rotating an alert chip is a transform mutation, not a layout mutation, so an attention-grabbing animation can run without re-running Taffy layout or reflowing neighbors. Val2 translation means the offset is expressed in responsive Val units, not raw pixels.
  > UiTransform: "Relative 2D transform for UI nodes" — fields: translation: Val2 "Translate the node.", scale: Vec2 "Scale the node. A negative value reflects the node in that axis.", rotation: Rot2 "Rotate the node clockwise." · "UiGlobalTransform is automatically inserted whenever UiTransform is inserted." · Node Required Components: ComputedNode, ComputedUiTargetCamera, ComputedUiRenderTargetInfo, UiTransform, BackgroundColor, BorderColor, FocusPolicy, ScrollPosition, Visibility, ZIndex
- **[central]** Render-side UI types are NOT in bevy_ui 0.18.0 — they live in a separate crate, bevy_ui_render 0.18.0. UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterial and UiMaterialPlugin are absent from bevy_ui's All Items index and present in bevy_ui_render's. Additionally, the 0.16/0.17-era ComputedNodeTarget no longer exists; it has been split into ComputedUiTargetCamera and ComputedUiRenderTargetInfo. Any tutorial or blog post importing these from bevy_ui will not compile against 0.18.
  > bevy_ui 0.18.0 All Items structs: "...ComputedNode, ComputedUiRenderTargetInfo, ComputedUiTargetCamera, DefaultUiCamera..." (no UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterial, ComputedNodeTarget) · bevy_ui_render 0.18.0 All Items: "BoxShadowSamples ✓, MaterialNode ✓ (listed as \"ui_material::MaterialNode\"), UiMaterialPlugin ✓ ... UiAntiAlias ✓ ... UiMaterial ✓"
- **[central]** Text contrast over arbitrary imagery is a first-party component in 0.18: bevy_ui::widget::TextShadow, with offset: Vec2 in logical pixels and color: Color. The docs explicitly distinguish it from Text2dShadow, which is the Text2d (world-space 2D text) equivalent — so the UI-text and 2D-text shadow paths are separate components and must not be confused. Text itself is bevy_ui::widget::Text, alongside TextShadow, TextMeasure, TextNodeFlags, TextUiReader and TextUiWriter.
  > TextShadow (module path bevy_ui::widget): "Adds a shadow behind text" / "Use the `Text2dShadow` component for `Text2d` shadows" — offset: Vec2 "Shadow displacement in logical pixels With a value of zero the shadow will be hidden directly behind the text"; color: Color "Color of the shadow"
- **[supporting]** bevy_ui 0.18.0 (published 2026-01-13) layers on taffy ^0.9 and depends directly on accesskit ^0.21, and exposes the full CSS flexbox + grid model as 41 fields on the single Node component (including border_radius, scrollbar_width, grid_template_rows/columns, grid_auto_flow, aspect_ratio), plus Display::Grid, GridPlacement, GridTrack, RepeatedGridTrack, GridTrackRepetition and Min/MaxTrackSizingFunction. A fixed, non-reflowing chip grid is therefore expressible in stock bevy_ui.
  > Dependencies: "bevy_text ^0.18.0 (normal), taffy ^0.9 (normal) ... accesskit ^0.21" · Node: "The base component for UI entities. It describes UI layout and style properties." — Fields (41 total): display, box_sizing, position_type, overflow, scrollbar_width, overflow_clip_margin, left, right, top, bottom, width, height, ... border_radius, flex_direction, flex_wrap, flex_grow, flex_shrink, flex_basis, row_gap, column_gap, grid_auto_flow, grid_template_rows, grid_template_columns, grid_auto_rows, grid_auto_columns, grid_row, grid_column

## Verification votes recovered (75 votes over 25 claims)

### [**REFUTED** — 3R / 0K] 
Render-side UI types are NOT in bevy_ui 0.18.0 — they live in a separate crate, bevy_ui_render 0.18.0. UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterial and UiMaterialPlugin are absent from bevy_ui's All Items index and present in bevy_ui_render's. Additionally, the 0.16/0.17-era ComputedNodeTarget no longer exists; it has been split into ComputedUiTargetCamera and ComputedUiRenderTargetInfo. Any tutorial or blog post importing these from bevy_ui will not compile against 0.18.
source: https://docs.rs/bevy_ui/0.18.0/bevy_ui/
  - REFUTE(high): The claim's CORE is verified, but it contains one false statement and one falsified universal, both checked against primary sources at pinned versions.

VERIFIED TRUE (keep this part):
- docs.rs/bevy_ui/0.18.0/bevy_ui/all.html — UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterial, UiMaterialPlugin, ComputedNodeTarget are all ABSENT. ComputedNode, ComputedUiTargetCamera, ComputedUiRenderTargetI  [src: https://docs.rs/bevy_ui/0.17.2/bevy_ui/all.html (ComputedNodeTarget already absent, ComputedUiTargetCamera + ComputedUiRenderTargetInfo already present in 0.17 — change is not 0.18-era) and https://docs.rs/bevy/0.18.0/bevy/prelude/index.html (prelude re-exports UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterialPlugin, BoxShadow, ComputedUiTargetCamera — so prelude-based tutorials still compile)]
  - REFUTE(high): The claim is compound. Two of its assertions verify; two are contradicted by the same primary source it cites.

VERIFIED (I independently confirmed):
- docs.rs/bevy_ui/0.18.0/bevy_ui/all.html — UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterial, UiMaterialPlugin are all ABSENT. ComputedNode, ComputedUiTargetCamera, ComputedUiRenderTargetInfo are present.
- docs.rs/bevy_ui_render/0.18.0/bevy_u  [src: https://docs.rs/bevy_ui/0.17.2/bevy_ui/all.html (ComputedNodeTarget already absent, replacements already present in 0.17) · https://docs.rs/bevy_ui/0.16.1/bevy_ui/all.html (all five render types still in bevy_ui at 0.16) · https://docs.rs/crate/bevy_ui_render/ (earliest release 0.17.0-rc.1, 2025-09-12) · https://docs.rs/bevy/0.18.0/bevy/prelude/index.html (prelude re-exports BoxShadowSamples and MaterialNode) · https://docs.rs/bevy/0.18.0/bevy/index.html (top-level `ui_render` facade module)]
  - REFUTE(high): PARTIALLY TRUE CORE, FALSE VERSION ATTRIBUTION. The 0.18 snapshot is confirmed: bevy_ui 0.18.0 All Items lists BackgroundColor, BorderColor, BorderRadius, BoxShadow, CalculatedClip, ComputedNode, ComputedUiRenderTargetInfo, ComputedUiTargetCamera, DefaultUiCamera ... UiPlugin, UiScale, UiStack, UiTargetCamera, ZIndex — with NO UiAntiAlias, BoxShadowSamples, MaterialNode, UiMaterial, UiMaterialPlug  [src: https://docs.rs/bevy_ui/0.17.2/bevy_ui/all.html · https://docs.rs/bevy_ui/0.17.2/bevy_ui/struct.ComputedUiTargetCamera.html · https://docs.rs/bevy_ui_render/0.17.2/bevy_ui_render/all.html · https://docs.rs/bevy_ui/0.16.0/bevy_ui/all.html · https://docs.rs/bevy/0.18.0/bevy/prelude/index.html]

### [**REFUTED** — 3R / 0K] 
Bevy 0.18 introduces a `TryStableInterpolate` trait that makes `Color` and the UI layout type `Val` animatable through the engine's own animation/interpolation machinery, handling `Val` unit mismatches fallibly — i.e. pulsing/fading alert chrome and animated layout values have first-party support rather than requiring a hand-rolled per-element timer.
source: https://bevy.org/news/bevy-0-18/
  - REFUTE(high): OVERREACH ON THE LOAD-BEARING CLAUSE. The claim's factual first half is correct; its operative conclusion ("rather than requiring a hand-rolled per-element timer") is contradicted by the primary source itself.

WHAT IS CONFIRMED (bevy.org/news/bevy-0-18/, §"Interpolation for Colors and Layout", author @viridia, PR #21633; Bevy 0.18 released 2026-01-13 — current, not outdated): the trait exists, wi  [src: https://docs.rs/bevy/0.18.0/bevy/math/trait.TryStableInterpolate.html (trait signature: `fn try_interpolate_stable(&self, other: &Self, t: f32) -> Result<Self, Self::Error>` — caller supplies `t`); https://docs.rs/bevy/0.18.0/bevy/math/trait.StableInterpolate.html (documented as a math primitive); https://bevy.org/news/bevy-0-18/ (full 22-section list contains no UI animation driver; §Interpolation for Colors and Layout: "the animation player will need to modify the parameter in some other way"); https://github.com/bevyengine/bevy/pull/21633 (math-layer scope, resolves issue #20579 "Fallible interpolation")]
  - REFUTE(high): SPLIT VERDICT — factual core true, operative conclusion refuted. Refuted overall because the "i.e." clause is what the research question turns on, and it is contradicted by primary docs.

VERIFIED (keep this part): docs.rs bevy 0.18.0 confirms `bevy::math::TryStableInterpolate` is new in 0.18 with exactly three implementors — `Color` (Error = MismatchedUnitsError), `Val` (Error = MismatchedUnitsEr  [src: https://docs.rs/bevy/0.18.0/bevy/math/trait.TryStableInterpolate.html (one required method, zero provided methods); https://docs.rs/bevy/0.18.0/bevy/math/trait.StableInterpolate.html (smooth_nudge requires caller-supplied delta_time "from an updating system"); https://docs.rs/bevy/0.18.0/bevy/animation/animation_curves/index.html (animation machinery does not reference TryStableInterpolate); https://bevy.org/news/bevy-0-18/ (actual verbatim text differs from the submitted quote; "the animation player will need to" is forward-looking)]
  - REFUTE(high): SPLIT VERDICT — the trait is real; the operative conclusion ("first-party support rather than a hand-rolled per-element timer") is an overreach and is contradicted by primary docs at the same pinned version.

VERIFIED TRUE (docs.rs/bevy/0.18.0/bevy/math/trait.TryStableInterpolate.html + bevy.org/news/bevy-0-18/ §"Interpolation for Colors and Layout"):
- Trait exists: `pub trait TryStableInterpolat  [src: https://docs.rs/bevy/0.18.0/bevy/animation/animatable/trait.Animatable.html (implementor list excludes Color and Val; interpolate() is infallible) · https://docs.rs/bevy/0.18.0/bevy/animation/animation_curves/trait.AnimatableProperty.html (type Property: Animatable — bound is NOT TryStableInterpolate) · https://bevy.org/learn/migration-guides/0-17-to-0-18/ (no Animatable replacement entry) · https://bevy.org/news/bevy-0-18/ (conditional wording: animation player "will need to" snap)]

### [**REFUTED** — 3R / 0K] 
Bevy 0.18 introduces a `FontHinting` component (mapping 1:1 to cosmic-text's `Hinting` enum) that controls glyph rasterization hinting — the first user-reachable font-hinting/crispness control in the Bevy text stack. UI `Text` requires `FontHinting::Enabled`; `Text2d` requires `FontHinting::Disabled`.
source: https://github.com/bevyengine/bevy/pull/22308
  - REFUTE(high): REFUTED — the second half of the claim is inverted for shipped Bevy 0.18. Verified against the v0.18.0 and v0.18.1 git tags, not blog prose.

WHAT SURVIVES (confirmed): `FontHinting` does exist in Bevy 0.18, added by PR #22308 "Update cosmic-text to 0.16" (merged 2026-01-01, milestone 0.18; Bevy 0.18.0 tagged 2026-01-13). At tag v0.18.0, `crates/bevy_text/src/text.rs:858` defines `pub enum FontHin  [src: https://github.com/bevyengine/bevy/pull/22494 ("Disable font hinting for UI text by default", milestone 0.18, merged 2026-01-13T20:02:51Z, fixing https://github.com/bevyengine/bevy/issues/22493) — plus direct source verification at the release tags via the GitHub contents API: crates/bevy_ui/src/widget/text.rs @ v0.18.0 and @ v0.18.1 (both `#[require(..., FontHinting::Disabled)]` on `Text`), crates/bevy_sprite/src/text2d.rs @ v0.18.0, crates/bevy_text/src/text.rs @ v0.18.0 (enum def, `#[default] Disabled`, `From<FontHinting> for cosmic_text::Hinting`, and `enum FontSmoothing` at line 724).]
  - REFUTE(high): The second sentence of the claim — the actionable half — is FALSE for shipped Bevy 0.18. It quotes the PR body as if it were release state and was never checked against the tag.

WHAT'S TRUE: PR #22308 "Update cosmic-text to 0.16" is real, merged 2026-01-01T04:06:28Z, milestone 0.18 (verified via `gh pr view 22308 --repo bevyengine/bevy`). `FontHinting` exists in bevy_text 0.18.0 with `impl From<F  [src: bevyengine/bevy PR #22494 "Disable font hinting for UI text by default" (milestone 0.18, merged 2026-01-13T20:02:51Z, fixes #22493) + tagged source crates/bevy_ui/src/widget/text.rs at v0.18.0 and v0.18.1 (both `FontHinting::Disabled`), and crates/bevy_ui/src/widget/text.rs at v0.19.0 (`FontHinting::Enabled`); crates/bevy_text/src/text.rs @ v0.18.0 (`#[default] Disabled`) vs @ v0.19.0 (parley-era, no cosmic_text conversion). All retrieved via the GitHub API at the exact tags.]
  - REFUTE(high): REFUTED on the load-bearing half, for the exact version the claim pins (0.18).

VERIFIED parts (not disputed): `FontHinting` does exist as a `Component` in `bevy_text` 0.18 (docs.rs bevy_text 0.18.0, released 2026-01-13, lists enum `FontHinting`, "Font hinting strategy"), and it maps 1:1 to cosmic-text's `Hinting`. Primary source, read from the vendored crate this repo actually compiles against —   [src: Primary source-of-truth, shipped code (not PR prose): vendored crate at /Users/josephwecker-v2/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ui-0.18.1/src/widget/text.rs:99-110 (UI `Text` requires `FontHinting::Disabled`), bevy_text-0.18.1/src/text.rs:851-874 (enum + `From<FontHinting> for cosmic_text::Hinting`, `Disabled` is `#[default]`), bevy_sprite-0.18.1/src/text2d.rs:81-93 (`Text2d` also requires `Disabled`); corroborated at the release tag https://raw.githubusercontent.com/bevyengine/bevy/v0.18.0/crates/bevy_ui/src/widget/text.rs and contrasted with https://raw.githubusercontent.com/bevyengine/bevy/v0.19.0/crates/bevy_ui/src/widget/text.rs and .../main/... (both `FontHinting::Enabled`).]

### [**REFUTED** — 3R / 0K] 
Bevy has no first-party text outline/border effect: the upstream feature request for it (bevyengine/bevy#17076, opened 2025-01-01) is still OPEN today, carrying maintainer labels A-Text / A-Rendering / C-Feature / D-Complex / S-Ready-For-Implementation — i.e. designed-but-unimplemented, with no assignee, branch, or PR linked. Verified via `gh issue view 17076 --repo bevyengine/bevy` on 2026-07-31: "state":"OPEN", last updated 2025-03-04. For crisp small HUD text over a busy 3D globe, outline-for-contrast must therefore be hand-rolled (duplicated text nodes) or obtained from a custom material/crate.
source: https://github.com/bevyengine/bevy/issues/17076
  - REFUTE(high): Verified against primary sources on 2026-07-31 (gh CLI + vendored crate sources at ~/.cargo/registry/src/*/bevy_ui-0.18.1, bevy_text-0.18.1). Two of the claim's load-bearing factual assertions are FALSE, and a third is a misframing:

(1) "no assignee, branch, or PR linked" — FALSE. `gh api repos/bevyengine/bevy/issues/17076/timeline --paginate` returns three cross-referenced PRs: #19639 (2025-06-1  [src: https://github.com/bevyengine/bevy/pull/23369 (OPEN, S-Needs-Review, "feat(text): add coverage-based text outlines", updated 2026-05-05) and bevy_ui 0.18.1 src/widget/text.rs:147 (`TextShadow`)]
  - REFUTE(high): The claim mixes one verified true fact with a false factual assertion and a materially misleading inference. Verified against the same primary source it cites (`gh` API, 2026-07-31) plus the actual vendored Bevy 0.18.1 source at `~/.cargo/registry/src/index.crates.io-.../bevy_ui-0.18.1`.

WHAT SURVIVES (independently confirmed): issue #17076 is genuinely OPEN, created 2025-01-01, `updatedAt` 2025-  [src: https://github.com/bevyengine/bevy/pull/23369 (OPEN "feat(text): add coverage-based text outlines", S-Needs-Review, updated 2026-05-05) and https://github.com/bevyengine/bevy/pull/17559 (MERGED 2025-02-05 "Basic UI text shadows"); plus vendored source bevy_ui-0.18.1/src/widget/text.rs:147 (TextShadow), bevy_ui_render-0.18.1/src/lib.rs:249 (extract_text_shadows), bevy_sprite-0.18.1/src/text2d.rs:139 (Text2dShadow), bevy_text-0.18.1/src/text.rs:631 (TextBackgroundColor)]
  - REFUTE(high): The narrow metadata is true, but two load-bearing specifics are false and the practical conclusion is misleading.

VERIFIED TRUE (gh issue view 17076 --repo bevyengine/bevy --json ..., run 2026-07-31): state OPEN, createdAt 2025-01-01T09:34:21Z, updatedAt 2025-03-04T08:24:31Z, assignees [], labels exactly A-Text, A-Rendering, C-Feature, D-Complex, S-Ready-For-Implementation. Supporting quote match  [src: https://github.com/bevyengine/bevy/pull/23369 (open, "feat(text): add coverage-based text outlines", updated 2026-05-05); https://github.com/bevyengine/bevy/pull/17559 (merged 2025-02-05, "Basic UI text shadows"); bevy_ui-0.18.1/src/widget/text.rs:147 (`TextShadow`)]

### [**REFUTED** — 3R / 0K] 
The one text effect Bevy did land is `TextShadow` (PR bevyengine/bevy#17559 "Basic UI text shadows", MERGED 2025-02-05), and it is deliberately UI-only — the component lives in `bevy_ui`, not `bevy_text`, so `Text2d` (world-space / billboard labels) gets no first-party shadow. This matters directly for anchoring labels to points on a globe: the contrast affordance available to `bevy_ui` nodes does not follow you into `Text2d`.
source: https://github.com/bevyengine/bevy/issues/17076
  - REFUTE(high): REFUTED on its load-bearing conclusion. The claim's operative assertions — "`Text2d` (world-space / billboard labels) gets no first-party shadow" and "the contrast affordance available to `bevy_ui` nodes does not follow you into `Text2d`" — are FALSE for Bevy 0.18, the exact version the research question targets.

(1) DIRECT CONTRADICTION, primary source. docs.rs/bevy_sprite/0.18.0 lists a struct   [src: https://docs.rs/bevy_sprite/0.18.0/bevy_sprite/struct.Text2dShadow.html and https://bevy.org/news/bevy-0-17/ (PR https://github.com/bevyengine/bevy/pull/20463)]
  - REFUTE(high): REFUTED on version-currency — the claim is a true statement about Bevy 0.15/0.16 that is false for Bevy 0.18, which is the only version this research run is about.

1) The load-bearing consequence is factually wrong at 0.18. `Text2dShadow` EXISTS as a first-party component: https://docs.rs/bevy/0.18.0/bevy/sprite/struct.Text2dShadow.html — `pub struct Text2dShadow { pub offset: Vec2, pub color: Co  [src: https://docs.rs/bevy/0.18.0/bevy/sprite/struct.Text2dShadow.html (also https://docs.rs/bevy_text/0.18.0/bevy_text/index.html and https://docs.rs/bevy_ui/0.18.0/bevy_ui/index.html)]
  - REFUTE(high): REFUTED on two independent grounds — the operative half is factually false for Bevy 0.18, and the supporting quote is not at the cited source.

(1) OUTDATED / CONTRADICTED BY PRIMARY SOURCE AT THE PINNED VERSION. The claim's load-bearing consequence — "`Text2d` (world-space / billboard labels) gets no first-party shadow ... the contrast affordance available to `bevy_ui` nodes does not follow you i  [src: https://github.com/bevyengine/bevy/pull/20463 (merged 2025-08-14) and https://raw.githubusercontent.com/bevyengine/bevy/v0.18.0/crates/bevy_sprite/src/text2d.rs#L139 (`Text2dShadow` present at tag v0.18.0); closure of https://github.com/bevyengine/bevy/issues/19529; follow-up https://github.com/bevyengine/bevy/pull/21695 (merged 2025-10-30)]

### [**REFUTED** — 3R / 0K] 
In Bevy 0.18, `TextFont::font_smoothing` is a no-op: the text pipeline ignores the field and always renders antialiased. The PR states the bug exists in `main` as of 2026-01-09 and is milestoned for **0.19** (merged 2026-01-15) — while Bevy 0.18.0 was tagged 2026-01-13, i.e. two days *before* the fix merged. So a 0.18 project cannot select `FontSmoothing::None`; any code setting it will silently do nothing.
source: https://github.com/bevyengine/bevy/pull/22455
  - REFUTE(high): REFUTED on its central technical assertion, verified against the actual shipped crate source (not docs, not blogs).

**What checks out:** bevy 0.18.0 = 2026-01-13, 0.18.1 = 2026-03-02/04, 0.19.0 = 2026-06-18 (github.com/bevyengine/bevy/releases + crates.io versions API). PR 22455 "FontSmoothing fixes" (ickshonpe) opened 2026-01-09, merged 2026-01-15, milestone 0.19, labels A-Text/C-Bug. All dates   [src: Primary source inspection of the shipped crates: bevy_text-0.18.0.crate (downloaded from https://static.crates.io/crates/bevy_text/bevy_text-0.18.0.crate) and bevy_text-0.18.1 from the local cargo registry cache — src/pipeline.rs:399-414, src/font_atlas_set.rs:11,18, src/font_atlas.rs:48,189-215, src/lib.rs:59-65; plus release dates from https://github.com/bevyengine/bevy/releases and https://crates.io/api/v1/crates/bevy/versions; PR metadata from https://github.com/bevyengine/bevy/pull/22455]
  - REFUTE(high): REFUTED by direct inspection of the Bevy source at the v0.18.0 and v0.18.1 tags. The claim mistakes "main" (0.19-dev trunk) for the 0.18 release line.

1. The quote is real but does not say what the claim infers. `gh api repos/bevyengine/bevy/pulls/22455` confirms: created 2026-01-09, merged 2026-01-15, base=`main`, milestone=0.19. "Ignored in main currently" describes the trunk during the 0.19 cy  [src: https://raw.githubusercontent.com/bevyengine/bevy/v0.18.0/crates/bevy_text/src/pipeline.rs (line 399: `let font_smoothing = self.glyph_info[span_index].1;` — the hardcoded `FontSmoothing::AntiAliased` the PR removes is absent from the 0.18 release tag); corroborated by v0.18.1 identical source and by PR #22455's own diff pre-image]
  - REFUTE(high): REFUTED by direct inspection of the tagged 0.18 source. The claim's date arithmetic is a release-branch fallacy: PR #22455's base is `main`, and `main` on 2026-01-09 was already deep in the 0.19 dev cycle, NOT the 0.18 release content.

GROUND TRUTH (curl + grep of raw.githubusercontent.com at the tags, not a summarizer):

`crates/bevy_text/src/pipeline.rs` at v0.18.0 and v0.18.1 is byte-identical  [src: Primary source, same repo: https://raw.githubusercontent.com/bevyengine/bevy/v0.18.0/crates/bevy_text/src/pipeline.rs (lines 324, 399, 401, 423, 437) and .../v0.18.0/crates/bevy_text/src/font_atlas.rs (lines 48, 139, 212); identical at tag v0.18.1. Release timing via https://api.github.com/repos/bevyengine/bevy/releases (v0.18.0 2026-01-13, v0.18.1 2026-03-02, v0.19.0 2026-06-18). Main-branch 0.19-cycle text commits via https://api.github.com/repos/bevyengine/bevy/commits?path=crates/bevy_text/src/pipeline.rs&sha=main. PR diff via https://patch-diff.githubusercontent.com/raw/bevyengine/bevy/pull/22455.diff]

### [**REFUTED** — 3R / 0K] 
`TextShadow` is not a blurred shadow — it is implemented as the glyphs redrawn at an offset in a different color (fields: `offset`, `color`; system `extract_text_shadows`). Blur-radius support was explicitly deferred as requiring shader changes. So the only built-in contrast aid for text over arbitrary imagery is a hard 1-copy drop shadow, at roughly 2x the text draw cost.
source: https://github.com/bevyengine/bevy/issues/17076
  - REFUTE(high): MIXED — mechanism confirmed, but the load-bearing "only built-in contrast aid" clause is false against Bevy 0.18 primary source, and the citation is misattributed.

CONFIRMED (verified against v0.18.0 source, not just the 2025 PR):
- Fields exact. docs.rs/bevy_ui/0.18.0 `widget::TextShadow` = `offset: Vec2` ("Shadow displacement in logical pixels. With a value of zero the shadow will be hidden dir  [src: https://raw.githubusercontent.com/bevyengine/bevy/v0.18.0/crates/bevy_ui_render/src/lib.rs (L224-254 system-set ordering; L1008-1096 extract_text_shadows; L1174, L1220-1232 TextBackgroundColor rect) · https://docs.rs/bevy_text/0.18.0/bevy_text/struct.TextBackgroundColor.html · https://github.com/bevyengine/bevy/pull/17559 (actual source of the quote, merged 2025-02-05) · https://github.com/bevyengine/bevy/issues/17076 (cited URL — 3 comments, none containing the quote)]
  - REFUTE(high): MECHANISM HALF SURVIVES, but the citation is misattributed and the decision-driving clause ("the only built-in contrast aid") is FALSE. Verified against the actual Bevy 0.18.1 source the project compiles against, in ~/.cargo/registry/src/index.crates.io-.../ .

(1) WHAT IS TRUE. bevy_ui-0.18.1/src/widget/text.rs:147-153 defines `pub struct TextShadow { offset: Vec2, color: Color }` — exactly the t  [src: Primary source, Bevy 0.18.1 vendored source on disk: bevy_ui-0.18.1/src/widget/text.rs:147-153 (TextShadow fields); bevy_ui_render-0.18.1/src/lib.rs:1008 (extract_text_shadows); bevy_text-0.18.1/src/text.rs:631 + bevy_ui-0.18.1/src/lib.rs:68 (TextBackgroundColor, prelude-exported); bevy_ui_render-0.18.1/src/lib.rs ~1221 (extract_text_decorations renders NodeType::Rect behind each run); bevy_ui-0.18.1/src/ui_node.rs:2215/2242/2355/2780/2827 (BackgroundColor, BorderColor, Outline, BoxShadow with blur_radius). Quote provenance: https://github.com/bevyengine/bevy/pull/17559 ("Basic UI text shadows", created 2025-01-27, merged) — NOT issue 17076; verified via https://api.github.com/repos/bevyengine/bevy/issues/17076/comments (phrase absent).]
  - REFUTE(high): TWO INDEPENDENT DEFECTS. The mechanical core is true; the citation and the load-bearing "only" are not.

(1) CITATION MISATTRIBUTION — the supporting quote is NOT in the cited source. Fetched https://github.com/bevyengine/bevy/issues/17076 : it is an OPEN feature request "Add Text Effects such as border and shadow" by @deavid, 2025-01-01, and neither its body nor any of its three comments (mrec 20  [src: Primary, pinned version, read directly: /Users/josephwecker-v2/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ui-0.18.1/src/ui_node.rs (BackgroundColor:2215, BorderColor:2242, Outline:2355, BoxShadow:2780, ShadowStyle.blur_radius:2827) and .../src/widget/text.rs:147; .../bevy_ui_render-0.18.1/src/lib.rs:249,1008; .../bevy_text-0.18.1/src/text.rs:440-455. Citation check: https://github.com/bevyengine/bevy/issues/17076 + https://api.github.com/repos/bevyengine/bevy/issues/17076/comments (quote absent) vs https://api.github.com/repos/bevyengine/bevy/pulls/17559 (quote present; merged 2025-02-05, ickshonpe).]

### [**REFUTED** — 2R / 1K] 
In Bevy 0.18 `TextFont::font_size` is a logical-pixel quantity that the text pipeline multiplies by the window scale factor and by `UiScale` to get the rasterized glyph height, and explicitly does NOT apply the text entity's transform or the camera projection. So on a macOS Retina / fractional-DPI display, UI text is rasterized at physical resolution automatically (no manual DPI compensation needed), and `UiScale` is the single global lever for panel-wide size changes.
source: https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html
  - REFUTE(high): Compound claim; the mechanism half is true but the operational conclusion is contradicted by primary source, and the citation is version-unstable.

(1) SUPPORTED — the scaling mechanism. Pinned bevy 0.18.0 TextFont::font_size reads verbatim: "The vertical height of rasterized glyphs in the font atlas in pixels. This is multiplied by the window scale factor and `UiScale`, but not the text entity tr  [src: https://docs.rs/bevy/0.18.0/bevy/prelude/struct.UiScale.html and https://docs.rs/bevy/0.18.0/bevy/prelude/struct.TextFont.html (pinned 0.18.0, vs. https://docs.rs/bevy/0.19.0/... which is what the claim's /latest/ URL actually resolves to today)]
  - keep(high): CORE MECHANISM: CONFIRMED at the exact version, not just at /latest/.

I fetched https://docs.rs/bevy/0.18.0/bevy/prelude/struct.TextFont.html directly (the claim cited /latest/, which today serves 0.19.0 — see provenanc
  - REFUTE(high): PARTIAL REFUTATION — clauses 1–3 verified true at the exact version; clause 4 is contradicted by Bevy's own primary docs.

WHAT SURVIVES (verified, primary source, pinned version):
The supporting quote is verbatim-accurate at 0.18.1, not just "latest". Read directly from the vendored crate at ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_text-0.18.1/src/text.rs:261-268 — doc comment   [src: Primary source, exact pinned version, read from disk: /Users/josephwecker-v2/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ui-0.18.1/src/lib.rs:113-119 (UiScale doc: "This will only affect fixed ui values like Val::Px") · bevy_ui-0.18.1/src/update.rs:161 · bevy_ui-0.18.1/src/widget/text.rs:350-362 · bevy_text-0.18.1/src/text.rs:261-268 · bevy_text-0.18.1/src/pipeline.rs:338 · bevy_camera-0.18.1/src/camera.rs:486-491 · bevy_window-0.18.1/src/window.rs:892,904,921-922,978-983 · corroborated by https://docs.rs/bevy/0.18.0/bevy/prelude/struct.TextFont.html and https://docs.rs/bevy/0.18.0/bevy/prelude/struct.UiScale.html]

### [contested — 1R / 2K] 
Bevy 0.18 offers exactly two text antialiasing modes — grayscale AA (default) and none (pixel-art) — and subpixel/LCD/ClearType antialiasing is explicitly unsupported. `FontSmoothing` has only the variants `None` and `AntiAliased`. For crisp small text over a busy 3D scene there is no font-smoothing knob to tune beyond on/off; crispness must be bought through font choice, integer-aligned physical sizing, and backing contrast rather than through a rendering mode.
source: https://docs.rs/bevy_text/0.18.0/bevy_text/
  - REFUTE(high): PARTIALLY TRUE, BUT THE LOAD-BEARING CONCLUSION IS CONTRADICTED BY THE SAME CRATE AT THE SAME VERSION.

What survives: the narrow `FontSmoothing` fact is verbatim-accurate. Verified against the actual vendored source the project compiles against — `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_text-0.18.1/src/text.rs:724` — the enum has exactly `None` and `AntiAliased` (`#[default]`)  [src: Primary: vendored crate source at the exact compiled version — /Users/josephwecker-v2/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_text-0.18.1/src/text.rs:724 (FontSmoothing) and :851-876 (FontHinting); bevy_ui-0.18.1/src/widget/text.rs:106-108, 147, 250, 276, 289; bevy_sprite-0.18.1/src/text2d.rs:93,180. Corroborating at the cited version: https://docs.rs/bevy_text/0.18.0/bevy_text/enum.FontHinting.html and https://bevy.org/news/bevy-0-18/ (2026-01-13).]
  - keep(high): CORE CLAIM VERIFIED AT TWO INDEPENDENT PRIMARY SOURCES, AT THE EXACT PINNED VERSION.

(1) GitHub raw source at tag v0.18.0 (`crates/bevy_text/src/text.rs`) — the full enum definition, verbatim:

```rust
/// Determines wh
  - keep(high): VERIFIED AGAINST PRIMARY SOURCES AT THE PINNED VERSION. Four refutation attempts, all failed:

(1) QUOTE-SUPPORT CHECK — PASSES. Fetched https://docs.rs/bevy_text/0.18.0/bevy_text/enum.FontSmoothing.html directly. It has

### [SURVIVED — 0R / 3K] 
Bevy 0.18 DOES expose OpenType feature selection per text span — tabular figures are reachable natively. `TextFont` carries a `font_features: FontFeatures` field, and `FontFeatureTag` includes `TABULAR_FIGURES`, `OLDSTYLE_FIGURES`, `STANDARD_LIGATURES`, plus numeric-valued tags (e.g. `WEIGHT`) via `FontFeaturesBuilder`. This directly answers the brief's open question 'whether tabular/lining figures or OpenType feature selection is reachable at all' — yes, without an ecosystem crate. Caveat stated by the docs: FontFeatures is described as covering '.otf fonts that support them'.
source: https://docs.rs/bevy_text/0.18.0/bevy_text/
  - keep(high): Attempted refutation on four fronts; all failed. The claim survives against primary source at two independent levels.

1) DOCS (the cited source, verified live, not from memory): WebFetch of https://docs.rs/bevy_text/0.1
  - keep(high): Tried to refute; could not. Verified twice, at the exact cited version and at the version the project actually resolves to.

(1) PRIMARY, cited version. WebFetch of https://docs.rs/bevy_text/0.18.0/bevy_text/struct.TextF
  - keep(high): Attempted refutation on five fronts; all failed. Verified independently of the cited docs.rs page by reading the actual vendored crate source the project compiles against: /Users/josephwecker-v2/.cargo/registry/src/index

### [SURVIVED — 0R / 3K] 
`TextFont::font_size` is defined as the glyph rasterization height in the font atlas in pixels, multiplied by the window scale factor and `UiScale` but NOT by entity transform or camera projection — and a distinct font atlas is generated per (font handle, scaled font size) combination, with an explicit performance warning. Two consequences for a Retina/fractional-DPI macOS instrument panel: DPI is handled automatically for UI text (logical px in, physical rasterization out), and animating or continuously varying font size is a real cost, not a free effect.
source: https://docs.rs/bevy_text/0.18.0/bevy_text/
  - keep(high): Verified against the exact pinned version's primary source, twice, including the rendered source file (not just the rendered doc page).

1) docs.rs/bevy_text/0.18.0 source view (`src/bevy_text/text.rs`) shows the struct 
  - keep(high): TRIED TO REFUTE, FAILED — the claim survives at the highest available source tier.

1. QUOTE AUTHENTICITY — verified at the pinned version, twice, including the git tag itself (not just docs.rs rendering). Fetched https:
  - keep(high): ADVERSARIAL VERIFICATION — attempted refutation failed on every axis.

1. QUOTE AUTHENTICITY (two independent primary sources, both at the exact pinned version):
- docs.rs/bevy_text/0.18.0/bevy_text/struct.TextFont.html 

### [SURVIVED — 0R / 3K] 
bevy_ui 0.18.0 ships the full set of instrument-panel chrome primitives first-party, with no ecosystem crate required: BorderRadius, BoxShadow (+ShadowStyle), Outline, Overflow/OverflowAxis/OverflowClipBox/OverflowClipMargin/CalculatedClip/OverrideClip, ZIndex and GlobalZIndex, and a complete gradient system (LinearGradient, RadialGradient, ConicGradient, BackgroundGradient, BorderGradient, ColorStop, AngularColorStop, InterpolationColorSpace). Notably, Outline is explicitly layout-free, so a focus/alarm ring can be added or removed without shifting any sibling chip.
source: https://docs.rs/bevy_ui/0.18.0/bevy_ui/
  - keep(high): Attempted refutation on five fronts; all failed. (1) ITEM-BY-ITEM INVENTORY CHECK against the primary source at the exact pinned version (https://docs.rs/bevy_ui/0.18.0/bevy_ui/, generated from the published crate, not a
  - keep(high): DIRECT PRIMARY-SOURCE VERIFICATION at the exact pinned version. docs.rs/bevy_ui/0.18.0 (published 13 January 2026 — current for a mid-2026 question, not stale) confirms 20/20 of the enumerated names exist, checked across
  - keep(high): VERIFIED AGAINST PRIMARY SOURCE AT THE EXACT PINNED VERSION. bevy_ui 0.18.0, released 13 Jan 2026.

(1) ENUMERATION — fully confirmed, zero fabrication. The crate's complete "All Items" page (docs.rs/bevy_ui/0.18.0/bevy_

### [SURVIVED — 0R / 3K] 
bevy_ui 0.18.0 has a dedicated 2D UI-space transform — UiTransform { translation: Val2, scale: Vec2, rotation: Rot2 } — which is auto-inserted as a required component of every Node, with UiGlobalTransform auto-inserted alongside it. This means pulsing/scaling/rotating an alert chip is a transform mutation, not a layout mutation, so an attention-grabbing animation can run without re-running Taffy layout or reflowing neighbors. Val2 translation means the offset is expressed in responsive Val units, not raw pixels.
source: https://docs.rs/bevy_ui/0.18.0/bevy_ui/
  - keep(high): Verified against primary sources pinned at the exact version, including the inferential half that the quote alone did not cover.

STRUCTURAL FACTS — all confirmed at docs.rs bevy_ui 0.18.0:
1. https://docs.rs/bevy_ui/0.1
  - keep(high): Attempted refutation on five fronts; all failed. Verified against docs.rs for the exact pinned version AND the crate's rendered source (docs.rs `src/` view), not blog posts.

1) STRUCT SHAPE — CONFIRMED verbatim. `https:
  - keep(high): Verified conjunct-by-conjunct against docs.rs PINNED to 0.18.0 (the exact version under investigation) including the rendered implementation source — not a blog post, not an older version, not forum speculation.

(1) STR

### [SURVIVED — 0R / 3K] 
Text contrast over arbitrary imagery is a first-party component in 0.18: bevy_ui::widget::TextShadow, with offset: Vec2 in logical pixels and color: Color. The docs explicitly distinguish it from Text2dShadow, which is the Text2d (world-space 2D text) equivalent — so the UI-text and 2D-text shadow paths are separate components and must not be confused. Text itself is bevy_ui::widget::Text, alongside TextShadow, TextMeasure, TextNodeFlags, TextUiReader and TextUiWriter.
source: https://docs.rs/bevy_ui/0.18.0/bevy_ui/
  - keep(high): Verified directly against the pinned primary source, both halves independently.

(1) Module listing — docs.rs/bevy_ui/0.18.0/bevy_ui/widget/index.html Structs section reads verbatim: "Button, ImageMeasure, ImageNode, Ima
  - keep(high): Every checkable assertion verified directly against docs.rs at the exact pinned version (bevy_ui 0.18.0), plus three adversarial cross-checks that failed to break it.

1) EXISTENCE + PATH + FIELDS — https://docs.rs/bevy_
  - keep(high): VERIFIED against docs.rs at the exact pinned version; every load-bearing element checks out, and I tried to break it on four axes.

1. Existence + module path — CONFIRMED. Fetched https://docs.rs/bevy_ui/0.18.0/bevy_ui/w

### [SURVIVED — 0R / 3K] 
Bevy 0.18's text stack is Cosmic Text (cosmic-text); Bevy 0.19 replaces it wholesale with Parley. This confirms the brief's suspicion about 0.18's text backend AND establishes that any typography work built on 0.18 sits on a stack the engine is about to swap out — glyph/shaping/font-selection behavior, and any code touching PositionedGlyph or TextPipeline font IDs, is version-fragile. Corroborating detail from the same guide: in 0.19 `PositionedGlyph`'s `byte_index` and `byte_length` fields are removed because Parley does not expose them, and `map_handle_to_font_id`/`get_font_id` are removed from `TextPipeline`.
source: https://bevy.org/learn/migration-guides/0-18-to-0-19/
  - keep(high): Attempted refutation on four fronts; all failed. The claim survives on primary-source evidence, with one tense correction that strengthens rather than weakens it.

1. QUOTE SUPPORTS CLAIM (not an overreach). Fetched the 
  - keep(high): CONFIRMED on every substantive component, by two independent primary sources — one of which (the crates.io registry) does not depend on Bevy's own narrative at all.

**1. "Bevy 0.18's text stack is Cosmic Text" — CONFIRM
  - keep(high): SURVIVED adversarial verification on all four components, each against a primary source.

(1) 0.18 = cosmic-text — verified DIRECTLY, not inferred from the guide's title. docs.rs source of bevy_text 0.18.0 Cargo.toml.ori

### [SURVIVED — 0R / 3K] 
In Bevy 0.18, `TextFont::font` is a `Handle<Font>` and `TextFont::font_size` is a plain `f32` (raw pixels only). 0.19 changes `font` to a `FontSource` enum (Handle or Family) and `font_size` to a `FontSize` wrapper with variants such as `FontSize::Px(35.)`. Practical consequence for 0.18 HUD work: font size is a bare float with no unit type, there is no family-name-based font selection (you must load and hold an asset Handle), and there is no rem/relative sizing — 0.19 adds a rem concept (evidenced by the new `is_rem_size_changed` parameter on `ComputedTextBlock` rerender).
source: https://bevy.org/learn/migration-guides/0-18-to-0-19/
  - keep(high): Every sub-claim checks out against two independent primary sources; I tried three angles of attack and none landed.

(1) The 0.18 types — verified independently of the migration guide. docs.rs at the pinned version (http
  - keep(high): VERIFIED against primary sources at pinned versions; every sub-claim confirmed, one under-stated in the claim's favor.

(1) 0.18 types — docs.rs bevy_text 0.18.0 struct.TextFont: fields are `font: Handle<Font>` ("The spe
  - keep(high): Every sub-claim verified against three primary sources at pinned versions.

(1) 0.18 side, docs.rs bevy_text 0.18.0 (version-pinned URL, not "latest"): `pub struct TextFont { pub font: Handle<Font>, pub font_size: f32, p

### [SURVIVED — 0R / 3K] 
Bevy's first-party widget/styling set `bevy_feathers` is gated behind a cargo feature literally named `experimental_bevy_feathers` in 0.18 — i.e. it is officially flagged experimental in the version vivarium pins — and is only de-experimentalized (feature renamed to `bevy_feathers`) in 0.19. Additionally, 0.19 moves Feathers widget construction to BSN as the primary API and renames the old spawn functions (`button` → `button_bundle`), so any 0.18 Feathers usage is on an API the project itself is replacing next release.
source: https://bevy.org/learn/migration-guides/0-18-to-0-19/
  - keep(high): Both halves verified against independent PRIMARY sources; I tried to break it and could not.

(1) The 0.18-side fact — the part the cited quote only implies — is independently confirmed. https://docs.rs/crate/bevy/0.18.0
  - keep(high): Verified against four independent primary artifacts; every load-bearing sub-claim holds.

(1) "experimental_bevy_feathers in 0.18" — CONFIRMED at the source, not just the guide. Fetched https://raw.githubusercontent.com/
  - keep(high): All four sub-claims verified against primary sources, and two of them against shipped artifacts rather than the migration guide's own prose.

1. FEATURE NAME IN 0.18 — verified independently of the quote. https://docs.rs

### [SURVIVED — 0R / 3K] 
Bevy 0.18 exposes OpenType feature selection directly on `TextFont` via a `font_features` field built with `FontFeatures::builder()` and `FontFeatureTag` constants — meaning ligature control, small caps, oldstyle/lining numerals and variable-axis values (e.g. WIDTH) are reachable from the engine API without dropping to cosmic-text — but only for `.otf` fonts that carry the features, and support varies per font.
source: https://bevy.org/news/bevy-0-18/
  - keep(high): Claim survives adversarial checking; confirmed by a second, non-promotional primary source at the exact pinned version.

1. FIELD EXISTS, EXACT TYPE. https://docs.rs/bevy_text/0.18.0/bevy_text/struct.TextFont.html lists 
  - keep(high): Attempted refutation on five axes; all failed. The claim survives.

1. QUOTE SUPPORTS CLAIM (no overreach). The bevy.org/news/bevy-0-18/ section "OpenType Font Features" carries verbatim the exact code sample the claim p
  - keep(high): Tried to refute; could not. Every component verified against the pinned-version API docs, not just the release-post marketing prose.

VERIFIED AT docs.rs/bevy_text/0.18.0 (authoritative, version-pinned):
- `TextFont` in 

### [SURVIVED — 0R / 3K] 
Font hinting in Bevy 0.18 is settable only per text block (per entity carrying the `Text`/`Text2d` component), not per `TextSpan` — it was deliberately not added as a field on `TextFont`, so mixed-hinting runs within one text block are not expressible.
source: https://github.com/bevyengine/bevy/pull/22308
  - keep(high): Verified against the merged PR diff itself (https://patch-diff.githubusercontent.com/raw/bevyengine/bevy/pull/22308.diff, 236 lines, retrieved this session) plus docs.rs pinned to 0.18.0 — not just the PR comment quoted 
  - keep(high): Tried hard to refute; the claim survives on primary source at both the PR level and the shipped-crate level.

**1. Quote is real and correctly attributed.** WebFetch of https://github.com/bevyengine/bevy/pull/22308 confi
  - keep(high): Claim verified at three independent levels of primary source, all at the pinned tag v0.18.0 (not "latest", not the PR's aspirational description).

1. THE QUOTE IS GENUINE AND IN CONTEXT, NOT AN OVERREACH. PR bevyengine/

### [SURVIVED — 0R / 3K] 
In Bevy 0.18, `LineHeight` is no longer a field on `TextFont` — it is a standalone component required by `Text`, `Text2d`, and `TextSpan`, so leading is set per-entity/per-span rather than inside the font struct. (Directly affects the vivarium HUD's existing `TextFont { font_size: 12.0, ..default() }` spawn pattern: line spacing must now be an additional component.)
source: https://bevy.org/learn/migration-guides/0-17-to-0-18/
  - keep(high): CORE CLAIM VERIFIED at two independent primary levels, not just the quoted blurb.

1. Migration guide (primary, bevy.org/learn/migration-guides/0-17-to-0-18/): section heading "LineHeight is now a separate component"; bo
  - keep(high): CONFIRMED against three independent primary sources at the exact pinned version tag v0.18.0.

(1) QUOTE ACCURACY: The migration guide section "LineHeight is now a separate component" reads verbatim as quoted: "The `line_
  - keep(high): PRIMARY CLAIM: VERIFIED, three-for-three, at the exact pinned version. Not merely the migration guide — I read the source at the `v0.18.0` tag.

1. `line_height` gone from `TextFont`. docs.rs bevy_text 0.18.0 struct defi

### [SURVIVED — 0R / 3K] 
Rounded corners are a first-party `bevy_ui` primitive in 0.18, but `BorderRadius` is no longer a separate component — it has moved to a `border_radius` field on the `Node` struct, so any 0.17-or-earlier example that spawns `BorderRadius` alongside `Node` will not compile against 0.18.
source: https://bevy.org/learn/migration-guides/0-17-to-0-18/
  - keep(high): Attempted refutation on three fronts; all failed. (1) PRIMARY SOURCE CHECK — fetched https://bevy.org/learn/migration-guides/0-17-to-0-18/ directly; it does carry the BorderRadius migration entry, rendered as "`BorderRad
  - keep(high): Tried hard to break this; it survives triple primary-source confirmation.

(1) MIGRATION GUIDE (claimed source, fetched directly). https://bevy.org/learn/migration-guides/0-17-to-0-18/ contains a section headed "`BorderR
  - keep(high): Attempted refutation on all three sub-claims; each was confirmed by primary sources, and the version-currency check (the most likely failure mode for this brief) came back clean.

1. QUOTE FIDELITY / OVERREACH CHECK. Fet

### [SURVIVED — 0R / 3K] 
Font smoothing is a real, addressable dimension of Bevy 0.18's glyph rasterization: `FontAtlasKey` newtypes `(AssetId<Font>, u32, FontSmoothing)`, meaning a separate glyph atlas is cached per (font asset, size, smoothing setting). `FontAtlasSets` was removed and `FontAtlasSet` is now a resource wrapping `HashMap<FontAtlasKey, Vec<FontAtlas>>`, with the glyph/atlas helper methods reworked into free functions in the `font_atlas` module.
source: https://bevy.org/learn/migration-guides/0-17-to-0-18/
  - keep(high): Attempted refutation on three fronts (quote-overreach, independent contradiction, staleness); all three failed. The claim survives, with one scope caveat that does not rise to refutation.

1) QUOTE FIDELITY — exact. Fetc
  - keep(high): SURVIVES ADVERSARIAL REVIEW. I attempted to refute on four fronts (quote fidelity, unstated inferences, version currency, source strength) and all four held.

(1) QUOTE FIDELITY — CONFIRMED VERBATIM. I fetched the cited 
  - keep(high): ADVERSARIAL VERIFICATION FAILED TO REFUTE — every element confirmed against two independent primary sources.

(1) QUOTE FIDELITY — I re-fetched the cited primary source (https://bevy.org/learn/migration-guides/0-17-to-0-

### [SURVIVED — 0R / 3K] 
docs.rs `/latest` for bevy no longer resolves to 0.18 — it now serves 0.19.0, whose `TextFont` is a structurally different eight-field struct (adds `width`, `style`, `font_variations`; changes `font` from `Handle<Font>` to `FontSource` and `font_size` from `f32` to `FontSize`). Any `/latest` docs.rs link cited for a Bevy 0.18 target is therefore version-inaccurate, and the pinned URL form `docs.rs/bevy/0.18.0/...` must be used instead. Verified by fetching both: the 0.18.0 page lists exactly five fields (`font`, `font_size`, `weight`, `font_smoothing`, `font_features`).
source: https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html
  - keep(high): Attempted refutation on three fronts; all failed. (1) Primary-source re-fetch of https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html independently returns "bevy 0.19.0" in the header/breadcrumb and exactly the 
  - keep(high): Attempted refutation on four axes; all failed. (1) Quote-vs-claim: exact. Fetched https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html — page self-identifies as **bevy 0.19.0 (released 2026-06-19)** and lists ex
  - keep(high): Attempted refutation on three fronts; all three confirmed the claim against primary sources.

1) "/latest serves 0.19.0" — CONFIRMED twice, independently. Fetching https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont

### [SURVIVED — 0R / 3K] 
OpenType feature selection IS reachable in Bevy 0.18 as a first-class field on `TextFont` — `font_features: FontFeatures`, documented as covering OpenType features for .otf fonts that support them. This means tabular/lining figures (`tnum`, `lnum`) for non-jittering numeric readouts are addressable through the public component API without dropping to a custom text pipeline, contrary to the brief's expectation that OpenType features might not be reachable at all.
source: https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html
  - keep(high): CLAIM SURVIVES ADVERSARIAL REVIEW — verified against local 0.18 source, not just docs.

The claim has four separable assertions; the risky one is the inference beyond the quote (that `tnum`/`lnum` specifically are reacha
  - keep(high): Attempted refutation on four fronts (version, field existence, tag reachability, whether it is a declared-but-ignored stub). All four failed to refute; the claim is confirmed.

1. FIELD EXISTS AT THE PINNED VERSION. `htt
  - keep(high): CONFIRMED via primary source stronger than the one cited: the vendored crate the project actually compiles against, plus a successful compile. Four refutation attempts, all failed.

(1) FIELD EXISTS, QUOTE EXACT. `~/.car

### [SURVIVED — 0R / 3K] 
Bevy 0.18's first-party widget layer (`bevy::ui_widgets`) is explicitly headless and ships no styling whatsoever — any visual appearance (chip backgrounds, borders, alert colors, typography) must be authored by the application, so adopting it buys interaction/state logic but zero visual chrome.
source: https://docs.rs/bevy/0.18.0/bevy/ui_widgets/index.html
  - keep(high): Tried hard to refute; four independent primary sources at the exact pinned version corroborate it, and nothing contradicts it.

(1) SOURCE VERIFIED, quote is verbatim and not cherry-picked. Fetched https://docs.rs/bevy/0
  - keep(high): Every element of the claim is confirmed by primary sources at the exact pinned version; four independent refutation attempts failed.

(1) QUOTE FIDELITY — exact, not an overreach. docs.rs/bevy/0.18.0/bevy/ui_widgets/inde
  - keep(high): VERIFIED at exact version, from primary source including the crate's own source file.

1. Quote is verbatim and correctly attributed. Fetched the rendered source of `bevy_ui_widgets` 0.18.0 `lib.rs` (https://docs.rs/bevy

