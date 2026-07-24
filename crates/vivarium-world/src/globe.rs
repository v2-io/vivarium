//! A primitive from-space ASCII/ANSI globe, coloured by **build-state** — what
//! each region has actually *reached* in the store — for `vivarium info`
//! (Joseph's nullable wish, DECISIONS `ascii-globe-in-info-colored-by-state`,
//! 2026-07-12). A non-animated, non-controllable snapshot.
//!
//! ## Projection — Hammer equal-area oval (the whole sphere in one view)
//!
//! `vivarium info` is a global overview: you want the WHOLE planet's build-state
//! at once — the iconic CMB-style elongated oval — not a one-hemisphere disc.
//! So this is the **Hammer** projection: **equal-area** (every region's on-screen
//! size is honest — the property that lets you read a global state distribution
//! without a projection lying about how much of the world reached each depth) and
//! **whole-sphere** (both hemispheres, poles included, in a single 2:1 oval).
//!
//! Chosen over Mollweide — visually the same CMB oval, both equal-area — because
//! Hammer has a **closed-form inverse** (Mollweide needs a per-pixel Newton
//! solve). We walk the oval's output pixels, invert each to (lon, lat), convert
//! to a unit direction, and read the `CellId` there via `CubeCoord::from_geo`.
//!
//! Tradeoff, accepted and named: this is a **light resample** — an oval pixel is
//! sampled to the nearest cell, so it is NOT the per-cell-exact mapping an
//! orthographic disc gives. That is the right call HERE: the question is the
//! global state *distribution* at a glance, not per-cell precision, and
//! whole-sphere-at-once is the priority (the native-orthographic instrument still
//! lives in the `globe_ascii` example for per-cell seam work). Minor: a few
//! pixels can land on a cube-face edge — the `from_unit` edge-tie (`sphere.rs`) —
//! a cosmetic seam speckle at overview resolution, never a correctness issue.
//!
//! ## What the colour means (and what it does NOT)
//!
//! Colour encodes the **deepest nomos materialised** for the tile a pixel falls
//! in — read from the store census, not assumed: unbuilt → initial_topo
//! (`initial-topography`) → eroded (`fluvial`) → watered. This is a **build-state** claim
//! ("what is in the store"), *not* a geological-phase claim — no per-region
//! phase state exists yet (the ordinum's phases are world-global). The legend
//! says so out loud.

use std::collections::{BTreeMap, BTreeSet};

use crate::gen::{self, SEA_LEVEL_M};
use crate::planet::Planet;
use crate::query::World;
use crate::sphere::{CubeCoord, Geo};

/// How far a tile has been carried in the build — the colour channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    /// No tile for this region in the store — rendered from the raw prior, dim.
    Unbuilt,
    /// Only the initial_topo (`initial-topography`) tile exists.
    InitialTopography,
    /// The fluvial-erosion tile exists (carved, no water settled yet).
    Eroded,
    /// The surface-water tile exists (water settled on the eroded bed).
    Watered,
}

/// The store census, parsed into per-tile build-state at one display level.
struct Coverage {
    level: u8,
    nx: usize,
    /// `(face, oi, oj)` origins that have a initial_topo tile.
    initial_topo: BTreeSet<(u8, u32, u32)>,
    /// `(face, oi, oj)` → erosion `epochs` (needed to re-pull the eroded field).
    erosion: BTreeMap<(u8, u32, u32), u32>,
    /// `(face, oi, oj)` origins that have a settled water tile.
    watered: BTreeSet<(u8, u32, u32)>,
}

/// Pull one `key=value` field out of a canonical store-key string.
fn field<'a>(key: &'a str, name: &str) -> Option<&'a str> {
    key.split('|').find_map(|f| f.strip_prefix(name).and_then(|r| r.strip_prefix('=')))
}

impl Coverage {
    /// Parse the raw census (`store.roots()` entries).
    /// The display level is the deepest level any surface tile reached; only
    /// tiles at that level count toward coverage (a mixed-level store shows its
    /// finest built rung). Provisional roots still count as built (they are in
    /// the store); status is what labels them provisional.
    fn parse(roots: &[crate::store::RootEntry]) -> Coverage {
        // Deepest level present among the surface nomos.
        let level = roots
            .iter()
            .filter(|r| r.key.starts_with("initial-topography@") || r.key.starts_with("erosion-tile@"))
            .filter_map(|r| field(&r.key, "level").and_then(|v| v.parse::<u8>().ok()))
            .max()
            .unwrap_or(6);
        let mut cov = Coverage {
            level,
            nx: 64,
            initial_topo: Default::default(),
            erosion: Default::default(),
            watered: Default::default(),
        };
        for r in roots {
            let k = r.key.as_str();
            let nomos = k.split('@').next().unwrap_or("");
            let l = match field(k, "level").and_then(|v| v.parse::<u8>().ok()) {
                Some(l) if l == level => l,
                _ => continue,
            };
            let _ = l;
            let (face, oi, oj) = match (
                field(k, "face").and_then(|v| v.parse::<u8>().ok()),
                field(k, "oi").and_then(|v| v.parse::<u32>().ok()),
                field(k, "oj").and_then(|v| v.parse::<u32>().ok()),
            ) {
                (Some(f), Some(oi), Some(oj)) => (f, oi, oj),
                _ => continue,
            };
            if let Some(nx) = field(k, "nx").and_then(|v| v.parse::<usize>().ok()) {
                cov.nx = nx;
            }
            match nomos {
                "initial-topography" => {
                    cov.initial_topo.insert((face, oi, oj));
                }
                "erosion-tile" => {
                    let epochs = field(k, "epochs").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
                    cov.erosion.insert((face, oi, oj), epochs);
                }
                "water-tile" => {
                    cov.watered.insert((face, oi, oj));
                }
                _ => {}
            }
        }
        cov
    }

    fn state(&self, face: u8, oi: u32, oj: u32) -> State {
        let t = (face, oi, oj);
        if self.watered.contains(&t) {
            State::Watered
        } else if self.erosion.contains_key(&t) {
            State::Eroded
        } else if self.initial_topo.contains(&t) {
            State::InitialTopography
        } else {
            State::Unbuilt
        }
    }
}

/// Auto colour decision: on when stdout is a TTY and `NO_COLOR` is unset.
pub fn color_auto() -> bool {
    std::env::var_os("NO_COLOR").is_none() && unsafe { isatty(1) == 1 }
}

extern "C" {
    fn isatty(fd: i32) -> i32;
}

// 256-colour palettes, low→high within each band. Chosen so the build ladder
// reads as a natural progression: grey (unbuilt) → tan/brown (raw topography) →
// khaki/olive (eroded, dry) → green (eroded + water settled). Ocean is blue by
// depth, independent of state.
const OCEAN: &[u8] = &[17, 18, 19, 20, 26, 39]; // deep → shallow
const GREY: &[u8] = &[238, 241, 244, 247, 250]; // unbuilt, low → high
const TAN: &[u8] = &[94, 130, 137, 173, 180]; // initial_topo, low → high
const OLIVE: &[u8] = &[58, 100, 142, 148, 185]; // eroded, low → high
const GREEN: &[u8] = &[22, 28, 34, 40, 156]; // watered, low → high

/// Height ramp (colour mode: glyph carries relief, colour carries state).
const RAMP: &[u8] = b" .:-=+*#%@";

fn pick(pal: &[u8], t: f64) -> u8 {
    let i = (t.clamp(0.0, 1.0) * (pal.len() - 1) as f64).round() as usize;
    pal[i.min(pal.len() - 1)]
}

/// Render the whole-sphere Hammer oval + legend as a printable block. `lon0_deg`
/// is the central meridian (degrees, default 0 — rotate to centre a region); `w`
/// is the character width. The oval is 2:1, and terminal cells are ~2:1, so
/// `h = w/4` makes it read as a true CMB-style oval.
pub fn render(
    world: &World,
    roots: &[crate::store::RootEntry],
    w: usize,
    lon0_deg: f64,
    color: bool,
) -> String {
    let cov = Coverage::parse(roots);
    let h = (w / 4).max(1);
    let level = cov.level;
    let nx = cov.nx;
    let sea = SEA_LEVEL_M;
    // Relief scale for the land height ramp (m above sea level → full ramp).
    let relief = 3500.0;
    // Ocean depth scale (m below sea level → deepest blue).
    let depth_scale = 4000.0;

    // Hammer projection extents (R = 1): the oval spans x∈[−2√2,2√2], y∈[−√2,√2],
    // i.e. x²/8 + y²/2 ≤ 1. We invert each output pixel back to (lon, lat).
    let lon0 = lon0_deg.to_radians();
    let xmax = 2.0 * std::f64::consts::SQRT_2;
    let ymax = std::f64::consts::SQRT_2;

    // Per-tile field caches (pulled through the sanctioned query path, memoised
    // in the store — "depend by key", never re-derived from raw internals).
    let mut spine_cache: BTreeMap<(u8, u32, u32), Vec<f32>> = BTreeMap::new();
    let mut eroded_cache: BTreeMap<(u8, u32, u32), Vec<f32>> = BTreeMap::new();

    // Coverage tallies for the footer (tiles per state → % of the census).
    let mut tally = [0usize; 4];

    let mut out = String::new();
    // Reset any stray colour at the start of every row; track the active code so
    // we only emit an escape when it changes.
    for j in 0..h {
        let mut cur: i32 = -1;
        let mut row = String::with_capacity(w * 2);
        for i in 0..w {
            // Pixel → Hammer plane (y up); outside the oval is empty space.
            let px = ((i as f64 + 0.5) / w as f64 * 2.0 - 1.0) * xmax;
            let py = (1.0 - (j as f64 + 0.5) / h as f64 * 2.0) * ymax;
            if px * px / 8.0 + py * py / 2.0 > 1.0 {
                if color && cur != -1 {
                    row.push_str("\x1b[0m");
                    cur = -1;
                }
                row.push(' ');
                continue;
            }
            // Closed-form Hammer inverse → (lon, lat), then a unit direction.
            let z2 = 1.0 - (px / 4.0).powi(2) - (py / 2.0).powi(2);
            let z = z2.sqrt();
            let lat = (z * py).clamp(-1.0, 1.0).asin();
            let lon = 2.0 * (z * px).atan2(2.0 * (2.0 * z2 - 1.0)) + lon0;
            let cc = CubeCoord::from_geo(Geo { lat, lon });
            let cell = cc.cell(level);
            let (face, ci, cj, _) = cell.to_face_ij();
            let f = face.index();
            let (oi, oj) = ((ci / nx as u32) * nx as u32, (cj / nx as u32) * nx as u32);
            let st = cov.state(f, oi, oj);

            // Elevation from the deepest materialised surface field for this
            // tile (eroded → initial_topo), else the raw prior for an unbuilt region.
            let elev = match st {
                State::Watered | State::Eroded => {
                    let epochs = *cov.erosion.get(&(f, oi, oj)).unwrap_or(&0);
                    let tile = eroded_cache
                        .entry((f, oi, oj))
                        .or_insert_with(|| world.erosion_tile(face, level, oi, oj, nx, epochs).0);
                    let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                    tile.get(dj * nx + di).copied().unwrap_or(sea as f32) as f64
                }
                State::InitialTopography => {
                    let tile = spine_cache
                        .entry((f, oi, oj))
                        .or_insert_with(|| world.initial_topography(face, level, oi, oj, nx).0);
                    let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                    tile.get(dj * nx + di).copied().unwrap_or(sea as f32) as f64
                }
                State::Unbuilt => gen::initial_topography_m(world.seed(), cell, level),
            };

            let ocean = elev < sea;
            // Glyph: colour mode → relief ramp (colour carries state); plain
            // mode → state glyph (the deliverable survives with no colour).
            let ch = if color {
                if ocean {
                    '~'
                } else {
                    let t = (elev - sea) / relief;
                    RAMP[(t.clamp(0.0, 1.0) * (RAMP.len() - 1) as f64) as usize] as char
                }
            } else if ocean {
                '~'
            } else {
                match st {
                    State::Unbuilt => '`',
                    State::InitialTopography => ':',
                    State::Eroded => '+',
                    State::Watered => '#',
                }
            };

            if color {
                let code = if ocean {
                    pick(OCEAN, (sea - elev) / depth_scale)
                } else {
                    let t = ((elev - sea) / relief).clamp(0.0, 1.0);
                    match st {
                        State::Unbuilt => pick(GREY, t),
                        State::InitialTopography => pick(TAN, t),
                        State::Eroded => pick(OLIVE, t),
                        State::Watered => pick(GREEN, t),
                    }
                };
                if code as i32 != cur {
                    use std::fmt::Write as _;
                    let _ = write!(row, "\x1b[38;5;{code}m");
                    cur = code as i32;
                }
            }
            row.push(ch);
        }
        if color && cur != -1 {
            row.push_str("\x1b[0m");
        }
        out.push_str(&row);
        out.push('\n');
    }

    // Footer: what the whole census reached (tiles per state), and the legend.
    // Union every built origin — erosion/water without a matching initial-topo
    // root still count (legacy `spine-tile` keys, partial rebuilds, version churn).
    let mut origins: BTreeSet<(u8, u32, u32)> = cov.initial_topo.iter().copied().collect();
    origins.extend(cov.erosion.keys().copied());
    origins.extend(cov.watered.iter().copied());
    for &(f, oi, oj) in &origins {
        tally[cov.state(f, oi, oj) as usize] += 1;
    }
    // Unbuilt tiles are, by definition, not in the census — report the built
    // ladder only (honest: we can't count what was never materialised).
    let built = origins.len();
    let (n_spine_only, n_eroded, n_watered) = (tally[State::InitialTopography as usize], tally[State::Eroded as usize], tally[State::Watered as usize]);
    let cell_km = crate::sample::cell_size_m(level, Planet::EARTH.radius_m) / 1000.0;
    out.push_str(&format!(
        "\nprojection  Hammer equal-area oval (WHOLE sphere, area-honest; central meridian {:.0}°)\n",
        lon0_deg
    ));
    out.push_str(&format!(
        "display     L{level} · {nx}×{nx}-cell tiles · ~{cell_km:.0} km/cell · {built} tiles built (of the visible + hidden planet)\n"
    ));
    out.push_str(&format!(
        "reached     initial-topography-only {n_spine_only} · eroded {n_eroded} · watered {n_watered}   (build-state from the store census)\n"
    ));
    if color {
        out.push_str(&format!(
            "legend      \x1b[38;5;39m~\x1b[0m ocean   \x1b[38;5;247m▓\x1b[0m unbuilt   \x1b[38;5;173m▓\x1b[0m initial_topo   \x1b[38;5;148m▓\x1b[0m eroded   \x1b[38;5;40m▓\x1b[0m watered   (glyph = relief)\n"
        ));
    } else {
        out.push_str("legend      ~ ocean · ` unbuilt · : initial_topo · + eroded · # watered   (glyph = build-state; colour disabled)\n");
    }
    out.push_str("            colour = deepest nomos MATERIALISED per region (a build-state fact from the store), NOT a geological-phase claim.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_extracts_values() {
        let k = "erosion-tile@erosion-v|seed=7|face=2|level=6|oi=0|oj=64|nx=64|epochs=20";
        assert_eq!(field(k, "face"), Some("2"));
        assert_eq!(field(k, "level"), Some("6"));
        assert_eq!(field(k, "oj"), Some("64"));
        assert_eq!(field(k, "epochs"), Some("20"));
        assert_eq!(field(k, "nope"), None);
    }

    #[test]
    fn coverage_ladders_by_deepest_nomos() {
        use crate::store::RootEntry;
        let roots = vec![
            RootEntry {
                key: "initial-topography@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64".into(),
                object: "a".into(),
                provisional: false,
            },
            RootEntry {
                key: "erosion-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|epochs=20".into(),
                object: "b".into(),
                provisional: false,
            },
            RootEntry {
                key: "water-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|eepochs=20|steps=200".into(),
                object: "c".into(),
                provisional: false,
            },
            // a second face with only initial_topo reached
            RootEntry {
                key: "initial-topography@v|seed=0|face=1|level=6|oi=0|oj=0|nx=64".into(),
                object: "d".into(),
                provisional: false,
            },
        ];
        let cov = Coverage::parse(&roots);
        assert_eq!(cov.level, 6);
        assert_eq!(cov.nx, 64);
        assert_eq!(cov.state(0, 0, 0), State::Watered, "deepest nomos wins");
        assert_eq!(cov.state(1, 0, 0), State::InitialTopography, "initial-topography-only face");
        assert_eq!(cov.state(5, 0, 0), State::Unbuilt, "untouched face");
    }
}
