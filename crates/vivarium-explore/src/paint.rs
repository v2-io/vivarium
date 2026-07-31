//! **Paint modes** — the same geometry, asked different questions.
//!
//! The whole reason this explorer exists rather than a prettier globe: Joseph
//! needs to look at a landform and be able to tell *what made it*, *at what
//! fidelity*, *whether it is provisional*, and *what is genuinely unbuilt versus
//! quietly faked*. Hypsometric colour cannot answer any of those — it is the
//! mode that hides them, because everything it draws looks equally real.
//!
//! So elevation and provenance are separate channels, and the provenance channel
//! is a first-class view rather than a debug overlay. Every mode below draws the
//! *same* heights; they differ only in what the colour means, which the HUD
//! states on screen for whichever mode is up.

use vivarium_world::watch::BuildState;

/// What colour means this frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Paint {
    /// Hypsometric: colour is elevation relative to the sea datum. The familiar
    /// picture, and the one that says least about whether it is trustworthy.
    Surface,
    /// Colour is **build-state**: how far the builder carried the tile under
    /// each cell. The direct answer to "what is real here and what is the prior
    /// standing in for it".
    Provenance,
    /// Colour is standing-water depth from the built `water-tile` field — the
    /// only hydrology in the tree, drawn for the first time. Inland water (above
    /// the sea datum) is distinguished from marine, because conflating them is
    /// exactly the mistake the from-space blue invites.
    Water,
    /// Cross-face seam disagreement, isolated and amplified. Note that seam
    /// excess is painted in **every** mode (see [`seam_overlay`]); this mode
    /// drops the threshold and removes the competing signal so the structure of
    /// a discontinuity can be read.
    Seam,
    /// Colour is **signed elevation change against the uncarved initial
    /// topography** — the surface the fluvial kernel was seeded from.
    ///
    /// This mode exists because of a measurement, not a preference. Across the
    /// default world's whole settle history the mean absolute change is 3.8 m at
    /// the first stage and 25.6 m at the last, against relief of kilometres: in
    /// hypsometric colour, forty epochs of erosion are invisible. The quantity is
    /// real and was simply never drawn.
    ///
    /// It is signed and diverging for a second measured reason. 88% of cells
    /// **rise** and 5.6% **fall** — the kernel's uplift driver acts nearly
    /// everywhere while fluvial incision is a thin minority, exactly as
    /// #obs-erosion-residual-is-driver-bound reports. A one-sided "erosion" ramp
    /// would paint uplift and call it carving, which is the decalibration
    /// #norm-no-depiction-without-referent exists to prevent. So rising and
    /// falling are opposite hues and the HUD carries both fractions.
    Change,
    /// Colour is **standing water at the spill point** — the depth at which water
    /// stands when every closed basin in the drawn surface is full to its sill.
    /// This is the *wet limit*: the hydrologic steady state under positive net
    /// supply, level to the bit across each body and exactly zero on ground that
    /// drains ( #form-ocean-is-connectivity-not-elevation ,
    /// #obs-connectivity-fills-the-basins-the-threshold-drained ).
    ///
    /// This mode exists because a repair had no picture. Until 2026-07-28 the
    /// fluvial epoch filled every closed basin and kept the raise, so the stored
    /// bed was depression-free by construction and this paint would have been
    /// black everywhere by law rather than by geography
    /// ( #obs-lakes-are-routed-over-not-carved-away ). The repair made the
    /// quantity real and left it visible only inside a probe's printout, which
    /// is the gap this closes.
    ///
    /// **It is a limit, and the caption has to keep saying so** — no evaporation,
    /// inflow, seepage or residence time is in the account, so an endorheic basin
    /// under a dry climate stands lower than this and nothing here says by how
    /// much. Volume-limited filling is a further rung and wants a $P-E$ field this
    /// project does not own.
    ///
    /// Press 3 for the *marched* water field. The difference between the two
    /// pictures is the honest subject of this one, and it is not a
    /// less-versus-more-water difference: the marched kernel is a transient that
    /// covers 40 s of world time at any level ( #obs-water-fill-never-settles ),
    /// while this is the equilibrium it is failing to reach. The equilibrium needs
    /// no settle — it is a pure function of the stored bed, so it exists at every
    /// level the bed does, including a beacon the builder carves erosion-only.
    Depression,
}

impl Paint {
    pub const ALL: [Paint; 6] = [
        Paint::Surface,
        Paint::Provenance,
        Paint::Water,
        Paint::Seam,
        Paint::Change,
        Paint::Depression,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Paint::Surface => "surface",
            Paint::Provenance => "provenance",
            Paint::Water => "water",
            Paint::Seam => "seam",
            Paint::Change => "change",
            Paint::Depression => "depression",
        }
    }

    /// Whether this mode needs the depression fill over the drawn surface. It is
    /// one Priority-Flood pass per drawn unit, so it is paid only when it is the
    /// subject — the same bargain [`Self::needs_change`] makes.
    pub fn needs_depression(self) -> bool {
        self == Paint::Depression
    }

    /// Whether this mode needs the per-cell baseline. Computing it is one law
    /// evaluation per drawn cell, so it is paid only when it is the subject.
    pub fn needs_change(self) -> bool {
        self == Paint::Change
    }

    /// What the colour on screen *means* right now — stated every frame, because
    /// a legend the viewer has to remember is a legend that gets misread.
    pub fn legend(self) -> &'static str {
        match self {
            Paint::Surface => {
                "colour = hypsometry: blue = OCEAN (connectivity, not every cell below the datum); \
                 landlocked below-datum floors are low land. Press 2 for build state, 6 for lake fill"
            }
            Paint::Provenance => {
                "colour = BUILD-STATE from the store census: grey unbuilt (fated prior only) | \
                 amber initial-topography | olive eroded (fluvial carve) | green watered | \
                 rust STALE-SOURCE (a tile exists but no reader at this source hash can use it) | \
                 magenta PROVISIONAL (waived flux admission -- not lawful evidence)"
            }
            Paint::Water => {
                "colour = standing-water DEPTH from the built water-tile field (cyan inland, deep blue marine); \
                 dry land is grey. The blue in surface mode is the sea DATUM, not this"
            }
            Paint::Seam => {
                "colour = cross-face elevation step in excess of 3x the local within-face step. \
                 Dark is healthy; magenta is a genuine chart-seam discontinuity in the world, not steep terrain"
            }
            Paint::Change => {
                "colour = SIGNED elevation change vs the uncarved initial topography (the surface erosion was \
                 seeded from). Blue = LOWERED (fluvial incision won here) | red/orange = RAISED (the uplift \
                 driver won) | near-black = unchanged. The store does not separate the two, so this is the NET; \
                 the sign is what tells them apart. Z cycles the scale"
            }
            Paint::Depression => {
                "colour = STANDING WATER AT THE SPILL POINT, the wet limit: where water stands with every \
                 closed basin full to its sill (violet shallow -> white deep), level to the bit across each \
                 body and zero on ground that drains. It assumes net supply is positive -- no evaporation, \
                 inflow, seepage or residence time is in it, so an endorheic basin under a dry climate stands \
                 lower and nothing here says by how much. Press 3 for the MARCHED field, which is a 40 s \
                 transient toward this same equilibrium, not a second opinion about it. \
                 The reader treats the drawn window's rim as a NO-FLUX WALL, so basins reaching a tile edge \
                 are counted; on an assembled multi-tile surface that mixes inherited basins with seam pits \
                 the tiling manufactures, and nothing here separates them"
            }
        }
    }

    pub fn cycle(self) -> Paint {
        let i = Paint::ALL.iter().position(|p| *p == self).unwrap_or(0);
        Paint::ALL[(i + 1) % Paint::ALL.len()]
    }
}

/// Everything one cell's colour can depend on, gathered so the mode functions
/// stay total (no mode can silently fall through to another mode's answer).
#[derive(Clone, Copy)]
pub struct CellFacts {
    pub h_m: f32,
    pub sea_m: f32,
    /// Whether the **ocean** reaches this cell (connectivity, not elevation —
    /// #form-ocean-is-connectivity-not-elevation ). Below-datum landlocked
    /// basins are false here.
    pub is_ocean: bool,
    pub state: BuildState,
    pub flags: vivarium_world::watch::TileFlags,
    /// Standing-water depth (m) from the built water field; 0 where none.
    pub water_m: f32,
    /// Measured cross-face excess (m) at this corner — 0 away from a face edge.
    /// See [`seam_overlay`]: this is not decoration, it is the thing that makes
    /// the mesh's manufactured continuity admissible at all.
    pub seam_excess_m: f32,
    /// Deepest water in the field (m), for the ramp.
    pub water_max_m: f32,
    /// Signed elevation change (m) vs the uncarved initial topography; 0 when the
    /// change channel is not being computed.
    pub change_m: f32,
    /// Full-scale for the change ramp (m), **fixed by the viewer, never
    /// auto-fitted per frame**. An auto-scale would renormalize every step of a
    /// time scrub, so a growing signal would look constant — the one thing the
    /// scrub exists to show, hidden by the display of it.
    pub change_scale_m: f32,
    /// Depth (m) water would stand at if this cell's basin filled to its spill
    /// point; 0 where the surface already drains. Computed only when the
    /// depression channel is up.
    pub depression_m: f32,
    /// Deepest capacity in the drawn unit (m), for the ramp.
    pub depression_max_m: f32,
}

fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// sRGB → linear. The mesh COLOR attribute is linear; feeding it sRGB washes
/// everything toward white (the spike's first-light defect, kept fixed).
fn lin(c: [f32; 3]) -> [f32; 4] {
    let f = |s: f32| if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) };
    [f(c[0]), f(c[1]), f(c[2]), 1.0]
}

/// Hypsometric ramp: **ocean** is a depth ramp (shelf → abyss); land (including
/// landlocked below-datum basins) runs green → tan → brown → snow. The shoreline
/// is where the *ocean* reaches — not every cell under the waterline
/// ( #form-ocean-is-connectivity-not-elevation ). A Caspian floor below the
/// datum is dry terrain colour, not open-ocean blue.
fn hypsometric(h_m: f32, sea_m: f32, is_ocean: bool) -> [f32; 3] {
    let rel = h_m - sea_m;
    if is_ocean {
        let t = ((-rel).max(0.0) / 3800.0).clamp(0.0, 1.0).powf(0.65);
        lerp3([0.25, 0.49, 0.62], [0.015, 0.07, 0.20], t)
    } else if rel < 350.0 {
        // Includes freeboard ≤ 0 landlocked floors — they read as low land.
        let t = ((rel + 500.0) / 850.0).clamp(0.0, 1.0);
        lerp3([0.22, 0.32, 0.18], [0.45, 0.52, 0.27], t)
    } else if rel < 1300.0 {
        lerp3([0.45, 0.52, 0.27], [0.61, 0.53, 0.36], (rel - 350.0) / 950.0)
    } else if rel < 2300.0 {
        lerp3([0.61, 0.53, 0.36], [0.47, 0.38, 0.31], (rel - 1300.0) / 1000.0)
    } else {
        lerp3([0.47, 0.38, 0.31], [0.93, 0.94, 0.96], ((rel - 2300.0) / 900.0).min(1.0))
    }
}

/// Shade to the top of the relief so form reads even in the flat-colour modes:
/// a mild elevation modulation of the base hue, so provenance blocks are not
/// featureless slabs.
fn relief_modulate(c: [f32; 3], h_m: f32, sea_m: f32) -> [f32; 3] {
    let t = ((h_m - sea_m) / 3000.0).clamp(-1.0, 1.0);
    let k = 1.0 + 0.28 * t;
    [(c[0] * k).min(1.0), (c[1] * k).min(1.0), (c[2] * k).min(1.0)]
}

/// **The always-on seam announcement**, and the reason it is always on.
///
/// The mesh bridges cube-face edges C0 by construction: every corner is the
/// average of the same four discrete cells from both sides, so the two faces
/// agree to summation-order ulps. That is *manufactured continuity*. Where the
/// world's own field is discontinuous across a chart seam, the mesh smooths a
/// real defect into terrain, and terrain is exactly what a pattern-matching eye
/// will accept.
///
/// Under #norm-no-depiction-without-referent that is an unreal affordance, and
/// its three conditions are met — (a) temporary, pending the domain-side repair
/// #form-sphere-continuous-surface-fields insists on ("not edge blending or
/// cosmetic clamps"); (c) adopted because without it the globe is shot through
/// with black speckle and one-cell V-trenches that swamp every other signal.
///
/// Condition (b) is what this function is for. FE(4) says a caveat living only
/// in text is not strong for a visual instrument, because the viewer doing the
/// pattern-matching is not reading the HUD — and a smoothed seam, unlike ×20
/// relief, is not self-evident. So the disagreement is **measured and painted
/// in every paint mode**, not just the seam one. Wherever the mesh is inventing
/// continuity over a real step, the picture says so, in whatever mode you happen
/// to be in. The announcement is structural rather than textual, which is what
/// the norm asks for.
///
/// The measure discriminates rather than merely flagging edges: excess is the
/// cross-face step *beyond* 3x the local within-face step, so terrain that is
/// merely steep is steep on both measures and stays dark.
fn seam_overlay(c: [f32; 3], excess_m: f32, isolated: bool) -> [f32; 3] {
    // In `seam` mode the whole dynamic range is the subject; elsewhere the
    // overlay must be loud enough to catch an eye that is looking at something
    // else, without drowning the mode's own signal.
    let (floor, span, gain) = if isolated { (0.0, 1000.0, 1.0) } else { (150.0, 2500.0, 0.85) };
    let t = ((excess_m - floor) / span).clamp(0.0, 1.0) * gain;
    if t <= 0.0 {
        return c;
    }
    lerp3(c, [1.0, 0.0, 0.9], t)
}

pub fn shade(mode: Paint, f: CellFacts) -> [f32; 4] {
    let c = match mode {
        Paint::Surface => {
            // Inland standing water from the water-tile nomos (not ocean paint).
            // Without this, settled lakes were only visible in water mode — and
            // water mode was often empty because coverage ignored L9 water when
            // an L13 beacon set the surface level.
            if !f.is_ocean && f.water_m > crate::water::WET_M {
                let t = (f.water_m / f.water_max_m.max(1.0)).clamp(0.0, 1.0).powf(0.4);
                lerp3([0.45, 0.85, 0.95], [0.08, 0.42, 0.72], t)
            } else {
                hypsometric(f.h_m, f.sea_m, f.is_ocean)
            }
        }

        Paint::Provenance => {
            // Provisional overrides the ladder: bytes written under waived flux
            // admission must never look lawful, whatever depth they reached
            // ( #form-builder-admission FE(3) ).
            let base = if f.flags.provisional {
                [0.85, 0.15, 0.75]
            } else if f.flags.stale {
                // Built, and unreadable at this source hash. Distinct from
                // unbuilt on purpose: "rerun the builder" and "this was never
                // asked for" are different actions.
                [0.62, 0.24, 0.16]
            } else {
                match f.state {
                    BuildState::Unbuilt => [0.42, 0.42, 0.45],
                    BuildState::InitialTopography => [0.72, 0.55, 0.28],
                    BuildState::Eroded => [0.55, 0.60, 0.25],
                    BuildState::Watered => [0.25, 0.62, 0.34],
                }
            };
            // Open ocean darkens the provenance block so the coast is legible;
            // landlocked below-datum basins keep the land block (they are not sea).
            if f.is_ocean {
                lerp3([0.05, 0.08, 0.14], base, 0.45)
            } else {
                relief_modulate(base, f.h_m, f.sea_m)
            }
        }

        Paint::Water => {
            // Inland = not ocean (connectivity). A lake below the datum is inland.
            let inland = !f.is_ocean;
            if f.water_m <= crate::water::WET_M {
                // Dry: neutral grey by relief, so the water reads as the subject.
                let g = 0.22 + 0.30 * ((f.h_m - f.sea_m) / 3500.0).clamp(0.0, 1.0);
                if inland { [g, g, g * 0.98] } else { [0.06, 0.08, 0.13] }
            } else {
                let t = (f.water_m / f.water_max_m.max(1.0)).clamp(0.0, 1.0).powf(0.4);
                if inland {
                    // Inland standing water — lakes and channels, the only real
                    // hydrology anywhere in the tree. Deliberately loud.
                    lerp3([0.55, 0.95, 1.00], [0.05, 0.55, 0.85], t)
                } else {
                    lerp3([0.12, 0.24, 0.42], [0.02, 0.06, 0.16], t)
                }
            }
        }

        Paint::Seam => relief_modulate([0.16, 0.17, 0.20], f.h_m, f.sea_m),

        Paint::Change => {
            // Diverging about zero, on a signed sqrt so the small-but-real
            // majority of the signal is visible without the few-hundred-metre
            // outliers taking the whole range. Zero is near-black rather than
            // white: unchanged ground should recede, not glare.
            let t = (f.change_m / f.change_scale_m.max(1.0)).clamp(-1.0, 1.0);
            let m = t.abs().sqrt();
            let base = if t < 0.0 {
                // LOWERED — incision. Cyan through deep blue.
                lerp3([0.10, 0.11, 0.14], [0.20, 0.85, 1.00], m)
            } else {
                // RAISED — uplift outran incision here. Amber through red.
                lerp3([0.10, 0.11, 0.14], [1.00, 0.42, 0.10], m)
            };
            // A hairline at the coastline, so the change field is still read
            // against the geography it belongs to rather than floating free.
            if (f.h_m - f.sea_m).abs() < 60.0 {
                lerp3(base, [0.55, 0.55, 0.60], 0.35)
            } else {
                base
            }
        }

        Paint::Depression => {
            if f.depression_m <= 1.0 {
                // Drains. Neutral grey by relief below the datum's own dark, so
                // the capacity reads as the subject and the coastline still
                // places it. The 1 m floor is the probes' own threshold, kept
                // identical so the picture and the numbers agree.
                if f.h_m > f.sea_m {
                    let g = 0.20 + 0.28 * ((f.h_m - f.sea_m) / 3500.0).clamp(0.0, 1.0);
                    [g, g, g * 0.97]
                } else {
                    [0.05, 0.07, 0.12]
                }
            } else {
                // Violet through white: deliberately NOT the water mode's cyan,
                // because these two pictures must never be mistaken for each
                // other — one is hydrology and one is a hole.
                let t = (f.depression_m / f.depression_max_m.max(1.0)).clamp(0.0, 1.0).powf(0.45);
                lerp3([0.34, 0.16, 0.55], [0.97, 0.94, 1.00], t)
            }
        }
    };
    // Applied LAST and in every mode: see `seam_overlay`. Making this
    // conditional on the paint mode would return the C0 bridge to a text-only
    // caveat, which #norm-no-depiction-without-referent FE(4) does not accept.
    lin(seam_overlay(c, f.seam_excess_m, mode == Paint::Seam))
}

/// The view's **declared unreal affordances** — every visual element that a
/// viewer could read as a fact about the world and that does not have a world
/// referent, with the status each is claimed under.
///
/// This exists because #norm-no-depiction-without-referent's Working Notes name
/// exactly this gap: *"no instrument enumerates the live views' unreal
/// affordances."* An enumeration nobody maintains is worthless, so it is
/// generated from the running view's own state and shown on the HUD and in every
/// sighting, rather than kept as prose somewhere.
pub fn declared_affordances(
    exag: f32,
    headlight: bool,
    has_seams: bool,
    mode: Paint,
) -> Vec<String> {
    let mut v = Vec::new();
    if mode == Paint::Depression {
        // The one genuinely dangerous reading this mode invites. A violet basin
        // has the shape, position and colour of a lake, and the eye will take it
        // for one; the legend says otherwise but the legend is not what the eye
        // is doing. So the affordance is declared here, where every sighting
        // carries it, rather than only in a line the viewer may not be reading.
        v.push(
            "DEPRESSION paint draws the WET LIMIT -- where water stands with every closed basin full to              its sill. What your eye supplies and this does not is the water BALANCE: no evaporation,              inflow, seepage or residence time is in the account, so a basin under a dry climate stands              lower than drawn and nothing here says by how much. ADMITTED, and drawn in a non-water              palette on purpose, because a limit is a claim about the bed's geometry plus one assumption              about climate, not a reading of water that is there."
                .to_string(),
        );
        v.push(
            "DEPRESSION paint reads each drawn unit as a NO-FLUX WALLED domain. That is a declared              boundary contract, not an absent one: a coastless window gets no outlet but its own lowest              cell, which can make most of its area read as one basin. The alternative contract drains              every basin reaching a rim and reports ~0, so there is no neutral reader here -- only a              named one."
                .to_string(),
        );
    }
    if exag != 1.0 {
        v.push(format!(
            "relief exaggerated x{exag:.0} -- ADMITTED: self-announcing (no eye reads x20 relief as literal); \
             X cycles to x1, which is honest and looks like a billiard ball"
        ));
    }
    v.push(
        "ocean drawn as a smooth sphere: geometry is clamped at the sea datum, so bathymetric relief is \
         carried by COLOUR only -- the surface you see there is the water surface, which is real"
            .to_string(),
    );
    if has_seams {
        v.push(
            "cube-face seams bridged C0 (mesh continuity is manufactured) -- ADMITTED under the three-condition \
             exception, and announced structurally: measured cross-face excess is painted magenta in EVERY mode, \
             so where the mesh invents continuity the picture says so. Repair is domain-side, not view-side."
                .to_string(),
        );
    }
    if headlight {
        v.push(
            "HEADLIGHT: the light rides the camera. There is no such star. Y returns to the real ephemeris \
             (default), whose terminator and seasons are the planet's own"
                .to_string(),
        );
    }
    v.push(
        "NOT present, by construction: no interpolation between stages, no smoothing of unbuilt regions, \
         no cosmetic fill. Deep-time frames are law evaluations at real mantle temperatures; where a system \
         has no interior, the picture jumps."
            .to_string(),
    );
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use vivarium_world::erosion::{EdgeContract, ErodedRegion, Fluvial};
    use vivarium_world::sphere::Face;

    fn facts(h_m: f32, depression_m: f32, water_m: f32) -> CellFacts {
        CellFacts {
            h_m,
            sea_m: 0.0,
            is_ocean: h_m <= 0.0,
            state: BuildState::Eroded,
            flags: Default::default(),
            water_m,
            seam_excess_m: 0.0,
            water_max_m: 100.0,
            change_m: 0.0,
            change_scale_m: 200.0,
            depression_m,
            depression_max_m: 300.0,
        }
    }

    /// A landlocked floor below the datum must not paint as open-ocean blue
    /// ( #form-ocean-is-connectivity-not-elevation ).
    #[test]
    fn landlocked_below_datum_is_not_ocean_blue() {
        let crater = CellFacts {
            h_m: -200.0,
            sea_m: 0.0,
            is_ocean: false,
            state: BuildState::Eroded,
            flags: Default::default(),
            water_m: 0.0,
            seam_excess_m: 0.0,
            water_max_m: 1.0,
            change_m: 0.0,
            change_scale_m: 200.0,
            depression_m: 0.0,
            depression_max_m: 1.0,
        };
        let open_sea = CellFacts { is_ocean: true, ..crater };
        let c_crater = shade(Paint::Surface, crater);
        let c_sea = shade(Paint::Surface, open_sea);
        // Open ocean is deep blue (strong B, weak R); crater floor is land green/brown.
        assert!(
            c_sea[2] > c_sea[0] + 0.05,
            "open ocean should be blue-dominant, got {c_sea:?}"
        );
        assert!(
            c_crater[1] + c_crater[0] > c_crater[2],
            "landlocked floor must not paint as ocean blue, got {c_crater:?}"
        );
    }

    /// **The depression paint must not be mistakable for the water paint**, and
    /// that is a colour claim rather than a caption claim — the caption is what
    /// the eye is not reading ( #norm-no-depiction-without-referent ).
    ///
    /// Capacity is a property of the bed and standing water is hydrology; the
    /// world currently has a great deal of the first and none of the second, so
    /// two pictures that looked alike would say the opposite of the truth.
    #[test]
    fn capacity_and_standing_water_are_drawn_in_palettes_that_cannot_be_confused() {
        let deep_capacity = shade(Paint::Depression, facts(500.0, 250.0, 0.0));
        let deep_water = shade(Paint::Water, facts(500.0, 0.0, 80.0));
        // Compared on hue rather than brightness: the violet ramp rises through
        // red as well as blue, the cyan ramp does not rise through red at all.
        let red_over_blue = |c: [f32; 4]| c[0] / c[2].max(1e-6);
        assert!(
            red_over_blue(deep_capacity) > 2.0 * red_over_blue(deep_water),
            "capacity {deep_capacity:?} and standing water {deep_water:?} are too close in hue to be told apart"
        );
    }

    /// A dry cell and a filled basin must be visibly different, or the mode
    /// reports nothing ( #norm-probe-sensitivity — a paint that draws the same
    /// colour everywhere certifies nothing, exactly like a measure that never
    /// fires).
    #[test]
    fn the_depression_paint_fires_on_a_basin_and_is_quiet_on_ground_that_drains() {
        let dry = shade(Paint::Depression, facts(500.0, 0.0, 0.0));
        let shallow = shade(Paint::Depression, facts(500.0, 30.0, 0.0));
        let deep = shade(Paint::Depression, facts(500.0, 250.0, 0.0));
        let lum = |c: [f32; 4]| c[0] + c[1] + c[2];
        assert!(lum(shallow) > lum(dry) * 1.2, "a 30 m basin must separate from dry ground");
        assert!(lum(deep) > lum(shallow) * 1.5, "capacity must be readable as a depth, not a mask");
        // The 1 m floor is the probes' own threshold; below it the paint is dry.
        assert_eq!(shade(Paint::Depression, facts(500.0, 0.9, 0.0)), dry);
    }

    /// **Why `pull` sets the boundary contract explicitly instead of letting it
    /// be inferred** — the whole reason this mode shows anything at all.
    ///
    /// A drawn window short of a whole cube face infers `BaseLevelSink`, which
    /// makes the window's own rim an outlet: every basin reaching it drains, and
    /// the paint would be black for a reason about the *reader* rather than the
    /// world. This is the same understatement measured on the beacon patch,
    /// where a sink-contract reader read 19.67% of the window in closed
    /// depressions against a wall reader's 63.5%
    /// ( #obs-tile-outlets-grade-away-the-basins FE(5) ).
    ///
    /// Reverting `set_edge_contract` in `pull.rs` fails this.
    #[test]
    fn the_inferred_contract_would_drain_the_basin_this_mode_exists_to_draw() {
        // A window (not a whole face): ground ramping down in +x, with a trench
        // cut across it that spans the full y range and therefore TOUCHES the
        // window's rim — which is the geometry at issue, a basin reaching a tile
        // edge. The trench floor stays well above the ramp's far end, so the
        // window's global minimum is at the low edge and the trench is a genuine
        // closed basin rather than the domain's own sink.
        let (level, oi, oj, nx) = (9u8, 64u32, 64u32, 32usize);
        // Anchored to the derived sea, or every cell is below the datum, every
        // cell is an outlet under BOTH contracts, and the test compares two
        // no-ops (the vacuous-footprint failure `Fluvial`'s own suite records).
        let sea = vivarium_world::sea_level::derived_sea_level_m(0) as f32;
        let mut h = vec![0.0f32; nx * nx];
        for j in 0..nx {
            for i in 0..nx {
                let ramp = sea + 3000.0 - 120.0 * i as f32;
                let trench = if (6..10).contains(&i) { 400.0 } else { 0.0 };
                h[j * nx + i] = ramp - trench;
            }
        }
        let region = ErodedRegion { face: Face::ZPos, level, oi, oj, nx, h, seed: 0 };

        let inferred = Fluvial::from_region(&region);
        assert_eq!(
            inferred.edge_contract(),
            EdgeContract::BaseLevelSink,
            "a window short of a whole face must still infer the sink contract, or this test is not about the real default"
        );
        let drained = Fluvial::from_region(&region).drainage_surface().stats;

        let mut walled = Fluvial::from_region(&region);
        walled.set_edge_contract(EdgeContract::NoFluxWall);
        let held = walled.drainage_surface().stats;

        assert_eq!(
            drained.depression_cells, 0,
            "the inferred contract drains the trench through the rim it reaches — got {} cells",
            drained.depression_cells
        );
        assert!(
            held.depression_cells >= nx,
            "the declared wall must hold the trench this mode exists to draw — got {} cells, deepest {:.0} m",
            held.depression_cells,
            held.deepest_depression_m
        );
        assert!(
            held.deepest_depression_m > 100.0,
            "and must hold it at its real depth, not a sliver — got {:.0} m",
            held.deepest_depression_m
        );
    }
}
