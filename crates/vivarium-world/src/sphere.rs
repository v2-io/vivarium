//! The world lives on a sphere, addressed with a **cube-sphere**: the six faces of
//! a cube, each a square `[-1, 1]²` grid, radially projected onto the sphere.
//!
//! Chosen (`DESIGN-REDUX.md` §11; validated in operational weather models — NOAA's
//! FV3, NASA GEOS) because each face is *already a quad grid*, so a local voxel
//! patch is just a deeply-subdivided face quad: **one coordinate system serves the
//! whole planet and the playable grid**, with no hex↔square mismatch and no
//! flat-plane approximation at the seam. Everything converts to/from a unit
//! direction vector and lat/lon (below), so we are never locked into cube-sphere.
//!
//! Convention: **+Y is the north-pole axis**; longitude is measured from +X toward
//! +Z. Known deficiency: mild area/shape distortion toward the 8 cube corners —
//! far gentler than lat/lon's pole singularities, and reducible later with an
//! equiangular/conformal projection variant (a fidelity rung on the projection).
//!
//! **Columns are radial frustums, treated as prisms.** A surface cell is really a
//! square *pyramidal frustum* — it tapers toward the planet's center and widens
//! outward. We treat it as a square prism (cuboid): at Earth radius the taper
//! across a 0.5 m cell is ~1e-7, negligible. The exact solid would only matter for
//! something modeled far enough center-ward for it to bite — which nothing in the
//! ~20 km livable shell is.

use std::f64::consts::FRAC_PI_4;

/// One of the six cube faces (by the axis its outward normal points along).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face { XPos, XNeg, YPos, YNeg, ZPos, ZNeg }

/// A *direction* on the sphere as a cube-sphere coordinate: which face, and `(u, v)`
/// in `[-1, 1]` within that face. A *position* pairs this with an altitude — see
/// [`Position`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CubeCoord {
    pub face: Face,
    pub u: f64,
    pub v: f64,
}

/// Geographic latitude/longitude in **radians** — the human-facing view. Convert
/// with [`CubeCoord::to_geo`] / [`CubeCoord::from_geo`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Geo {
    pub lat: f64,
    pub lon: f64,
}

/// A position within the ~20 km "livable" shell: a surface direction plus an
/// altitude in metres relative to the sea-level datum (deepest trench ≈ −11 km to
/// highest peak ≈ +9 km; the shell, not a solid ball — the deep interior is a
/// coarse global model that fluxes up into the shell's base).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub dir: CubeCoord,
    pub altitude_m: f64,
}

impl CubeCoord {
    /// Outward unit direction vector on the unit sphere.
    ///
    /// **Equiangular** cube-sphere: `(u, v)` are interpreted as *angles* (via `tan`),
    /// so cells subtend near-equal solid angles — max/min area ratio ≈ 1.41, versus
    /// ≈ 5.14 for the naive equidistant (gnomonic) grid, for the cost of a couple of
    /// `tan`/`atan` calls. (The variant used by operational cubed-sphere models.)
    pub fn to_unit(self) -> [f64; 3] {
        let (tu, tv) = ((self.u * FRAC_PI_4).tan(), (self.v * FRAC_PI_4).tan());
        let d = match self.face {
            Face::XPos => [1.0, tv, -tu],
            Face::XNeg => [-1.0, tv, tu],
            Face::YPos => [-tu, 1.0, tv],
            Face::YNeg => [-tu, -1.0, -tv],
            Face::ZPos => [tu, tv, 1.0],
            Face::ZNeg => [-tu, tv, -1.0],
        };
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        [d[0] / n, d[1] / n, d[2] / n]
    }

    /// Inverse of [`to_unit`](Self::to_unit): pick the dominant axis (the face),
    /// recover the face ratios, and undo the equiangular `tan`. `d` need not be
    /// normalized.
    pub fn from_unit(d: [f64; 3]) -> Self {
        let [x, y, z] = d;
        let (ax, ay, az) = (x.abs(), y.abs(), z.abs());
        let inv = |t: f64| t.atan() / FRAC_PI_4; // undo the equiangular mapping
        if ax >= ay && ax >= az {
            if x > 0.0 { CubeCoord { face: Face::XPos, u: inv(-z / ax), v: inv(y / ax) } }
            else { CubeCoord { face: Face::XNeg, u: inv(z / ax), v: inv(y / ax) } }
        } else if ay >= az {
            if y > 0.0 { CubeCoord { face: Face::YPos, u: inv(-x / ay), v: inv(z / ay) } }
            else { CubeCoord { face: Face::YNeg, u: inv(-x / ay), v: inv(-z / ay) } }
        } else if z > 0.0 {
            CubeCoord { face: Face::ZPos, u: inv(x / az), v: inv(y / az) }
        } else {
            CubeCoord { face: Face::ZNeg, u: inv(-x / az), v: inv(y / az) }
        }
    }

    /// Geographic lat/lon (radians), +Y as the pole axis.
    pub fn to_geo(self) -> Geo {
        let [x, y, z] = self.to_unit();
        Geo { lat: y.asin(), lon: z.atan2(x) }
    }

    pub fn from_geo(g: Geo) -> Self {
        let (clat, slat) = (g.lat.cos(), g.lat.sin());
        Self::from_unit([clat * g.lon.cos(), slat, clat * g.lon.sin()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn faces() -> [Face; 6] {
        [Face::XPos, Face::XNeg, Face::YPos, Face::YNeg, Face::ZPos, Face::ZNeg]
    }

    #[test]
    fn cube_unit_roundtrips() {
        for face in faces() {
            for &u in &[-0.7, -0.2, 0.0, 0.3, 0.8] {
                for &v in &[-0.6, 0.0, 0.5, 0.9] {
                    let c = CubeCoord { face, u, v };
                    let back = CubeCoord::from_unit(c.to_unit());
                    assert_eq!(back.face, face, "face changed for {face:?} ({u},{v})");
                    assert!((back.u - u).abs() < 1e-9 && (back.v - v).abs() < 1e-9,
                        "uv drift for {face:?}: ({u},{v}) -> ({},{})", back.u, back.v);
                }
            }
        }
    }

    #[test]
    fn geo_roundtrips() {
        for &lat in &[-1.4, -0.5, 0.0, 0.5, 1.4] {
            for &lon in &[-3.0, -1.0, 0.0, 1.0, 2.5] {
                let g = Geo { lat, lon };
                let back = CubeCoord::from_geo(g).to_geo();
                assert!((back.lat - lat).abs() < 1e-9, "lat drift: {lat} -> {}", back.lat);
                assert!((back.lon - lon).abs() < 1e-9, "lon drift: {lon} -> {}", back.lon);
            }
        }
    }
}
