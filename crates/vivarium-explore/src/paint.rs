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
}

impl Paint {
    pub const ALL: [Paint; 4] = [Paint::Surface, Paint::Provenance, Paint::Water, Paint::Seam];

    pub fn name(self) -> &'static str {
        match self {
            Paint::Surface => "surface",
            Paint::Provenance => "provenance",
            Paint::Water => "water",
            Paint::Seam => "seam",
        }
    }

    /// What the colour on screen *means* right now — stated every frame, because
    /// a legend the viewer has to remember is a legend that gets misread.
    pub fn legend(self) -> &'static str {
        match self {
            Paint::Surface => {
                "colour = elevation vs the sea datum (blue = below it). This mode says nothing about \
                 whether what you see was built or inferred -- press 2 for that"
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

/// Hypsometric ramp: water is a depth ramp (shelf → abyss), land runs green →
/// tan → brown → snow. The shoreline is where the datum says it is — colour, not
/// artistry, decides land from water.
fn hypsometric(h_m: f32, sea_m: f32) -> [f32; 3] {
    let rel = h_m - sea_m;
    if rel <= 0.0 {
        let t = (-rel / 3800.0).clamp(0.0, 1.0).powf(0.65);
        lerp3([0.25, 0.49, 0.62], [0.015, 0.07, 0.20], t)
    } else if rel < 350.0 {
        lerp3([0.30, 0.47, 0.24], [0.45, 0.52, 0.27], rel / 350.0)
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
        Paint::Surface => hypsometric(f.h_m, f.sea_m),

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
            // Below the datum the same state reads darker, so the coastline is
            // still legible and the provenance blocks do not float free of the
            // geography they describe.
            if f.h_m <= f.sea_m {
                lerp3([0.05, 0.08, 0.14], base, 0.45)
            } else {
                relief_modulate(base, f.h_m, f.sea_m)
            }
        }

        Paint::Water => {
            let inland = f.h_m > f.sea_m;
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
pub fn declared_affordances(exag: f32, headlight: bool, has_seams: bool) -> Vec<String> {
    let mut v = Vec::new();
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
