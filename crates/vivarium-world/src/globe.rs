//! A primitive from-space ASCII/ANSI globe, coloured by **build-state** — what
//! each region has actually *reached* in the store — for `vivarium info`
//! (Joseph's nullable wish, DECISIONS `ascii-globe-in-info-colored-by-state`,
//! 2026-07-12). A non-animated, non-controllable snapshot.
//!
//! ## Projection — orthographic, cube-sphere-native (documented choice)
//!
//! Joseph votes HEALPix "unless a projection that fits our cube-sphere topology
//! makes more sense." It does, for *this* job. The render's whole purpose is to
//! answer **"which cells are materialised, and to what depth?"** — so every
//! screen pixel must map to exactly ONE `CellId` with no interpolation. The
//! orthographic from-space projection (the `globe_ascii` example already solves
//! it) does precisely that: a ray → unit direction → `CubeCoord::from_unit` →
//! `CellId`, native, no resampling layer. Reprojecting our native cube-sphere
//! data onto a HEALPix grid would insert a resample that *fuzzes the very
//! question* — a pixel could straddle two source cells at different build
//! depths. An equal-area cube map would show the whole planet at once (better
//! for a coverage census) but reads far less as "a planet at a glance," which is
//! the ask. Tradeoff accepted: one hemisphere at a time (the far side is
//! hidden), and mild shape distortion toward the cube corners (the known
//! cube-sphere deficiency, `sphere.rs`). For a primitive that must stay honest
//! about what is built, native-orthographic is the simplest faithful choice.
//!
//! ## What the colour means (and what it does NOT)
//!
//! Colour encodes the **deepest nomos materialised** for the tile a pixel falls
//! in — read from the store census, not assumed: unbuilt → initial-topography
//! (`spine`) → eroded (`fluvial`) → watered. This is a **build-state** claim
//! ("what is in the store"), *not* a geological-phase claim — no per-region
//! phase state exists yet (the ordinum's phases are world-global). The legend
//! says so out loud.

use std::collections::HashMap;

use crate::gen::{self, SEA_LEVEL_M};
use crate::planet::Planet;
use crate::query::World;
use crate::sphere::CubeCoord;

/// How far a tile has been carried in the build — the colour channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    /// No tile for this region in the store — rendered from the raw prior, dim.
    Unbuilt,
    /// Only the initial-topography (`spine`) tile exists.
    Spine,
    /// The fluvial-erosion tile exists (carved, no water settled yet).
    Eroded,
    /// The surface-water tile exists (water settled on the eroded bed).
    Watered,
}

/// The store census, parsed into per-tile build-state at one display level.
struct Coverage {
    level: u8,
    nx: usize,
    /// `(face, oi, oj)` origins that have a spine tile.
    spine: std::collections::HashSet<(u8, u32, u32)>,
    /// `(face, oi, oj)` → erosion `epochs` (needed to re-pull the eroded field).
    erosion: HashMap<(u8, u32, u32), u32>,
    /// `(face, oi, oj)` origins that have a settled water tile.
    watered: std::collections::HashSet<(u8, u32, u32)>,
}

/// Pull one `key=value` field out of a canonical store-key string.
fn field<'a>(key: &'a str, name: &str) -> Option<&'a str> {
    key.split('|').find_map(|f| f.strip_prefix(name).and_then(|r| r.strip_prefix('=')))
}

impl Coverage {
    /// Parse the raw census (`store.roots()` output: `(key, obj_hash)` pairs).
    /// The display level is the deepest level any surface tile reached; only
    /// tiles at that level count toward coverage (a mixed-level store shows its
    /// finest built rung).
    fn parse(roots: &[(String, String)]) -> Coverage {
        // Deepest level present among the surface nomoi.
        let level = roots
            .iter()
            .filter(|(k, _)| k.starts_with("spine-tile@") || k.starts_with("erosion-tile@"))
            .filter_map(|(k, _)| field(k, "level").and_then(|v| v.parse::<u8>().ok()))
            .max()
            .unwrap_or(6);
        let mut cov = Coverage {
            level,
            nx: 64,
            spine: Default::default(),
            erosion: Default::default(),
            watered: Default::default(),
        };
        for (k, _) in roots {
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
                "spine-tile" => {
                    cov.spine.insert((face, oi, oj));
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
        } else if self.spine.contains(&t) {
            State::Spine
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
const TAN: &[u8] = &[94, 130, 137, 173, 180]; // spine, low → high
const OLIVE: &[u8] = &[58, 100, 142, 148, 185]; // eroded, low → high
const GREEN: &[u8] = &[22, 28, 34, 40, 156]; // watered, low → high

/// Height ramp (colour mode: glyph carries relief, colour carries state).
const RAMP: &[u8] = b" .:-=+*#%@";

fn pick(pal: &[u8], t: f64) -> u8 {
    let i = (t.clamp(0.0, 1.0) * (pal.len() - 1) as f64).round() as usize;
    pal[i.min(pal.len() - 1)]
}

/// Render the globe + legend as a printable block. `axis` is the view direction
/// (the hemisphere centre); `w` is the character width (`h = w/2`, terminal
/// cells being ~2:1).
pub fn render(world: &World, roots: &[(String, String)], w: usize, axis: [f64; 3], color: bool) -> String {
    let cov = Coverage::parse(roots);
    let h = w / 2;
    let level = cov.level;
    let nx = cov.nx;
    let sea = SEA_LEVEL_M;
    // Relief scale for the land height ramp (m above sea level → full ramp).
    let relief = 3500.0;
    // Ocean depth scale (m below sea level → deepest blue).
    let depth_scale = 4000.0;

    // Orthonormal view frame (as in the globe_ascii instrument).
    let n = normalize(axis);
    let up = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let rx = normalize(cross(up, n));
    let ry = cross(n, rx);

    // Per-tile field caches (pulled through the sanctioned query path, memoised
    // in the store — "depend by key", never re-derived from raw internals).
    let mut spine_cache: HashMap<(u8, u32, u32), Vec<f32>> = HashMap::new();
    let mut eroded_cache: HashMap<(u8, u32, u32), Vec<f32>> = HashMap::new();

    // Coverage tallies for the footer (tiles per state → % of the census).
    let mut tally = [0usize; 4];

    let mut out = String::new();
    // Reset any stray colour at the start of every row; track the active code so
    // we only emit an escape when it changes.
    for j in 0..h {
        let mut cur: i32 = -1;
        let mut row = String::with_capacity(w * 2);
        for i in 0..w {
            let x = (i as f64 + 0.5) / w as f64 * 2.0 - 1.0;
            let y = (j as f64 + 0.5) / h as f64 * 2.0 - 1.0;
            let r2 = x * x + y * y;
            if r2 >= 1.0 {
                if color && cur != -1 {
                    row.push_str("\x1b[0m");
                    cur = -1;
                }
                row.push(' ');
                continue;
            }
            let z = (1.0 - r2).sqrt();
            let d = [
                rx[0] * x + ry[0] * y + n[0] * z,
                rx[1] * x + ry[1] * y + n[1] * z,
                rx[2] * x + ry[2] * y + n[2] * z,
            ];
            let cc = CubeCoord::from_unit(d);
            let cell = cc.cell(level);
            let (face, ci, cj, _) = cell.to_face_ij();
            let f = face.index();
            let (oi, oj) = ((ci / nx as u32) * nx as u32, (cj / nx as u32) * nx as u32);
            let st = cov.state(f, oi, oj);

            // Elevation from the deepest materialised surface field for this
            // tile (eroded → spine), else the raw prior for an unbuilt region.
            let elev = match st {
                State::Watered | State::Eroded => {
                    let epochs = *cov.erosion.get(&(f, oi, oj)).unwrap_or(&0);
                    let tile = eroded_cache
                        .entry((f, oi, oj))
                        .or_insert_with(|| world.erosion_tile(face, level, oi, oj, nx, epochs).0);
                    let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                    tile.get(dj * nx + di).copied().unwrap_or(sea as f32) as f64
                }
                State::Spine => {
                    let tile = spine_cache
                        .entry((f, oi, oj))
                        .or_insert_with(|| world.spine_tile(face, level, oi, oj, nx).0);
                    let (di, dj) = ((ci - oi) as usize, (cj - oj) as usize);
                    tile.get(dj * nx + di).copied().unwrap_or(sea as f32) as f64
                }
                State::Unbuilt => gen::surface_prior_m(world.seed(), cell, level),
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
                    State::Spine => ':',
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
                        State::Spine => pick(TAN, t),
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
    for &(f, oi, oj) in cov.spine.iter() {
        tally[cov.state(f, oi, oj) as usize] += 1;
    }
    // Unbuilt tiles are, by definition, not in the census — report the built
    // ladder only (honest: we can't count what was never materialised).
    let built = cov.spine.len().max(cov.erosion.len()).max(cov.watered.len());
    let (n_spine_only, n_eroded, n_watered) = (tally[State::Spine as usize], tally[State::Eroded as usize], tally[State::Watered as usize]);
    let cell_km = crate::sample::cell_size_m(level, Planet::EARTH.radius_m) / 1000.0;
    out.push_str(&format!(
        "\nprojection  orthographic, cube-sphere-native (one hemisphere; look-axis {:.2},{:.2},{:.2})\n",
        n[0], n[1], n[2]
    ));
    out.push_str(&format!(
        "display     L{level} · {nx}×{nx}-cell tiles · ~{cell_km:.0} km/cell · {built} tiles built (of the visible + hidden planet)\n"
    ));
    out.push_str(&format!(
        "reached     spine-only {n_spine_only} · eroded {n_eroded} · watered {n_watered}   (build-state from the store census)\n"
    ));
    if color {
        out.push_str(&format!(
            "legend      \x1b[38;5;39m~\x1b[0m ocean   \x1b[38;5;247m▓\x1b[0m unbuilt   \x1b[38;5;173m▓\x1b[0m initial-topography   \x1b[38;5;148m▓\x1b[0m eroded   \x1b[38;5;40m▓\x1b[0m watered   (glyph = relief)\n"
        ));
    } else {
        out.push_str("legend      ~ ocean · ` unbuilt · : initial-topography · + eroded · # watered   (glyph = build-state; colour disabled)\n");
    }
    out.push_str("            colour = deepest nomos MATERIALISED per region (a build-state fact from the store), NOT a geological-phase claim.\n");
    out
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / m, v[1] / m, v[2] / m]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
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
        let roots = vec![
            ("spine-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64".into(), "a".into()),
            ("erosion-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|epochs=20".into(), "b".into()),
            ("water-tile@v|seed=0|face=0|level=6|oi=0|oj=0|nx=64|eepochs=20|steps=200".into(), "c".into()),
            // a second face with only spine reached
            ("spine-tile@v|seed=0|face=1|level=6|oi=0|oj=0|nx=64".into(), "d".into()),
        ];
        let cov = Coverage::parse(&roots);
        assert_eq!(cov.level, 6);
        assert_eq!(cov.nx, 64);
        assert_eq!(cov.state(0, 0, 0), State::Watered, "deepest nomos wins");
        assert_eq!(cov.state(1, 0, 0), State::Spine, "spine-only face");
        assert_eq!(cov.state(5, 0, 0), State::Unbuilt, "untouched face");
    }
}
