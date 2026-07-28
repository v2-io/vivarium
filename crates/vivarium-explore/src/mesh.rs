//! Cube-sphere face meshing — geometry only, no store access.
//!
//! Mined from the globe spike (now `archive/globe-spike`), whose mesh work is genuinely good and whose
//! hard-won comments are kept verbatim where they record a bug that cost a live
//! sighting to find. What is *not* inherited is where the heights come from: the
//! spike's worker could author store citizens, and this one reads a tile handed
//! to it and knows nothing about a store at all.

use bevy::prelude::*;
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CubeCoord, Face};

use crate::paint::{self, CellFacts, Paint};

/// Render unit = 1 km (f32-safe at planetary scale: km magnitudes ≈ 6.4e3, where
/// f32 still resolves sub-metre; metres would put us at 6.4e6 with ~0.5 m ulps).
pub fn radius_km() -> f32 {
    (Planet::EARTH.radius_m / 1000.0) as f32
}

/// One face's mesh, built CPU-side by the worker.
pub struct FaceMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

/// Per-face seam aggregates, in metres. `cross` = elevation step ACROSS a face
/// edge; `within` = the matched-arc-length baseline inside the face — the same
/// cross-vs-within method as `gen.rs`'s seam-continuity probe, wall to wall.
#[derive(Clone, Copy, Default)]
pub struct SeamStats {
    pub cross_max: f32,
    pub within_max: f32,
    pub cross_sum: f64,
    pub within_sum: f64,
    pub n: usize,
}

impl SeamStats {
    pub fn merge(&mut self, o: &SeamStats) {
        self.cross_max = self.cross_max.max(o.cross_max);
        self.within_max = self.within_max.max(o.within_max);
        self.cross_sum += o.cross_sum;
        self.within_sum += o.within_sum;
        self.n += o.n;
    }
    pub fn cross_mean(&self) -> f32 {
        (self.cross_sum / self.n.max(1) as f64) as f32
    }
    pub fn within_mean(&self) -> f32 {
        (self.within_sum / self.n.max(1) as f64) as f32
    }
}

/// How a face's out-of-tile ghost cells are valued. The ghost ring lies on the
/// *neighbouring* faces, so it cannot come from this tile; it must come from the
/// same law the tile was built from, or the seam instrument measures the
/// discrepancy between two different surfaces instead of the world's own.
pub type GhostFn<'a> = &'a (dyn Fn(Face, i64, i64, u8) -> f32 + Sync);

/// Corner-grid elevations for one face, with a one-corner **ghost ring**:
/// `(nx+3)²` heights, grid `(i, j)` = corner `(i−1, j−1)`.
///
/// Every corner — interior, boundary, ghost — is the average of its 4 adjacent
/// cells, so both faces of a shared edge average the same four discrete cells
/// and agree to summation-order ulps: the mesh is C0 at face seams by
/// construction.
///
/// Never sample *on* an edge itself: a direction with |u| = 1 sits on
/// `from_unit`'s dominant-axis tie, and the two faces build that component
/// through different float paths (a literal 1.0 vs tan(π/4) = 0.99999…), so the
/// tie resolves differently per face and each samples a different edge cell —
/// whole-edge elevation cliffs, found live as a 60 km-deep skirt canyon.
fn cell_value(face: Face, level: u8, tile: &[f32], ci: i64, cj: i64, ghost: GhostFn) -> f32 {
    let nx = 1usize << level;
    let n = nx as i64;
    if ci >= 0 && ci < n && cj >= 0 && cj < n {
        tile[cj as usize * nx + ci as usize]
    } else {
        ghost(face, ci, cj, level)
    }
}

fn corner_heights(face: Face, level: u8, tile: &[f32], ghost: GhostFn) -> Vec<f32> {
    let nx = 1usize << level;
    let gn = nx + 3;
    let mut h = vec![0.0f32; gn * gn];
    for gj in 0..gn {
        for gi in 0..gn {
            let (ki, kj) = (gi as i64 - 1, gj as i64 - 1);
            let sum = cell_value(face, level, tile, ki - 1, kj - 1, ghost)
                + cell_value(face, level, tile, ki, kj - 1, ghost)
                + cell_value(face, level, tile, ki - 1, kj, ghost)
                + cell_value(face, level, tile, ki, kj, ghost);
            h[gj * gn + gi] = sum * 0.25;
        }
    }
    h
}

/// The seam instrument. Returns the aggregates plus a per-corner **excess** grid
/// `(nx+1)²`: excess = cross step beyond 3× the *local* within step (plus a
/// 100 m floor). Terrain that is merely steep is steep on both measures and
/// stays dark; a genuine discontinuity is loud on cross alone and lights up.
fn seam_stats(face: Face, level: u8, tile: &[f32], ghost: GhostFn) -> (SeamStats, Vec<f32>) {
    let nx = 1usize << level;
    let n1 = nx + 1;
    let n = nx as i64;
    let mut excess = vec![0.0f32; n1 * n1];
    let mut st = SeamStats::default();
    let mut edge = |ic: (i64, i64), gc: (i64, i64), wc: (i64, i64), ca: (usize, usize), cb: (usize, usize)| {
        let h = cell_value(face, level, tile, ic.0, ic.1, ghost);
        let d_cross = (h - cell_value(face, level, tile, gc.0, gc.1, ghost)).abs();
        let d_within = (h - cell_value(face, level, tile, wc.0, wc.1, ghost)).abs();
        st.cross_max = st.cross_max.max(d_cross);
        st.within_max = st.within_max.max(d_within);
        st.cross_sum += d_cross as f64;
        st.within_sum += d_within as f64;
        let e = (d_cross - 3.0 * d_within - 100.0).max(0.0);
        for (i, j) in [ca, cb] {
            let k = j * n1 + i;
            excess[k] = excess[k].max(e);
        }
    };
    for k in 0..nx {
        let ki = k as i64;
        edge((ki, 0), (ki, -1), (ki, 1), (k, 0), (k + 1, 0));
        edge((ki, n - 1), (ki, n), (ki, n - 2), (k, nx), (k + 1, nx));
        edge((0, ki), (-1, ki), (1, ki), (0, k), (0, k + 1));
        edge((n - 1, ki), (n, ki), (n - 2, ki), (nx, k), (nx, k + 1));
    }
    st.n = 4 * nx;
    (st, excess)
}

/// Everything the mesher needs about one face that is not geometry.
pub struct FaceInput<'a> {
    pub face: Face,
    pub level: u8,
    pub tile: &'a [f32],
    pub exag: f32,
    pub sea_m: f32,
    pub mode: Paint,
    pub ghost: GhostFn<'a>,
    /// Build-state per cell, with the tile's flags (provisional / stale).
    pub state: &'a dyn Fn(u32, u32) -> (vivarium_world::watch::BuildState, vivarium_world::watch::TileFlags),
    /// Standing-water depth (m) per cell.
    pub water: &'a dyn Fn(u32, u32) -> f32,
    pub water_max_m: f32,
    /// Signed elevation change (m) vs the uncarved initial topography per cell.
    /// Returns 0 when the change channel is not being computed this frame.
    pub change: &'a dyn Fn(u32, u32) -> f32,
    pub change_scale_m: f32,
}

/// Build one face's mesh. Geometry: corners projected onto the sphere at
/// `R + max(0, h − sea) · exag` — the ocean renders as the smooth sea-level
/// sphere (bathymetry is colour-only), which is what "show me the landmasses
/// above water" means geometrically.
pub fn build_face(input: &FaceInput) -> (FaceMesh, SeamStats) {
    let FaceInput {
        face,
        level,
        tile,
        exag,
        sea_m,
        mode,
        ghost,
        state,
        water,
        water_max_m,
        change,
        change_scale_m,
    } = *input;
    let nx = 1usize << level;
    let n1 = nx + 1;
    let gn = nx + 3;
    let r_km = radius_km();
    let h = corner_heights(face, level, tile, ghost);
    let (seam, excess) = seam_stats(face, level, tile, ghost);

    let gidx = |i: usize, j: usize| j * gn + i;
    let mut gpos = vec![Vec3::ZERO; gn * gn];
    for gj in 0..gn {
        for gi in 0..gn {
            let u = ((gi as f64 - 1.0) / nx as f64) * 2.0 - 1.0;
            let v = ((gj as f64 - 1.0) / nx as f64) * 2.0 - 1.0;
            let d = CubeCoord { face, u, v }.to_unit();
            let hm = h[gidx(gi, gj)];
            let r = (r_km + ((hm - sea_m).max(0.0) / 1000.0) * exag) as f64;
            gpos[gidx(gi, gj)] = Vec3::new((d[0] * r) as f32, (d[1] * r) as f32, (d[2] * r) as f32);
        }
    }

    // Winding is face-chirality-dependent (each face's (u,v) → 3-space basis
    // differs in handedness), so probe one quad against the outward direction
    // and flip the whole face if needed — measured, not assumed.
    //
    // `flip = true` selects the NATURAL grid winding [a,b,c] below, so it must
    // hold exactly when the natural winding's geometric normal points OUTWARD
    // (n·a > 0 ⇒ CCW seen from outside ⇒ front-facing under Bevy's default
    // cull). The spike's original probe had this comparison inverted: every face
    // emitted inward-wound triangles, the near hemisphere was culled, and the
    // globe rendered as the far shell seen from inside. A screenshot cannot
    // catch this class of bug without a chirality reference — a mirrored
    // coastline still reads as "a coastline".
    let flip = {
        let (a, b, c) = (gpos[gidx(1, 1)], gpos[gidx(2, 1)], gpos[gidx(1, 2)]);
        (b - a).cross(c - a).dot(a) > 0.0
    };

    // Smooth normals accumulated over the ghost grid, so boundary vertices feel
    // the neighbour face's slopes and both sides of an edge shade continuously.
    let mut gnorm = vec![Vec3::ZERO; gn * gn];
    for gj in 0..gn - 1 {
        for gi in 0..gn - 1 {
            let (a, b, c, d) =
                (gidx(gi, gj), gidx(gi + 1, gj), gidx(gi + 1, gj + 1), gidx(gi, gj + 1));
            let tris: [[usize; 3]; 2] =
                if flip { [[a, b, c], [a, c, d]] } else { [[a, c, b], [a, d, c]] };
            for t in tris {
                let n = (gpos[t[1]] - gpos[t[0]]).cross(gpos[t[2]] - gpos[t[0]]);
                for k in t {
                    gnorm[k] += n;
                }
            }
        }
    }

    let idx = |i: usize, j: usize| (j * n1 + i) as u32;
    let mut positions = vec![[0.0f32; 3]; n1 * n1];
    let mut normals = vec![[0.0f32; 3]; n1 * n1];
    let mut colors = vec![[0.0f32; 4]; n1 * n1];
    for j in 0..n1 {
        for i in 0..n1 {
            let g = gidx(i + 1, j + 1);
            positions[idx(i, j) as usize] = gpos[g].to_array();
            normals[idx(i, j) as usize] = gnorm[g].normalize_or_zero().to_array();
            // Corner (i,j) is shaded by the cell it is the upper-left of, clamped
            // at the far edges. Cell-accurate provenance matters more here than a
            // smooth colour gradient: a provenance boundary should land on the
            // tile boundary it actually is.
            let (ci, cj) = (i.min(nx - 1) as u32, j.min(nx - 1) as u32);
            let (st, flags) = state(ci, cj);
            colors[idx(i, j) as usize] = paint::shade(
                mode,
                CellFacts {
                    h_m: h[g],
                    sea_m,
                    state: st,
                    flags,
                    water_m: water(ci, cj),
                    seam_excess_m: excess[j * n1 + i],
                    water_max_m,
                    change_m: change(ci, cj),
                    change_scale_m,
                },
            );
        }
    }
    let mut indices = Vec::with_capacity(nx * nx * 6);
    for j in 0..nx {
        for i in 0..nx {
            let (a, b, c, d) = (idx(i, j), idx(i + 1, j), idx(i + 1, j + 1), idx(i, j + 1));
            if flip {
                indices.extend_from_slice(&[a, b, c, a, c, d]);
            } else {
                indices.extend_from_slice(&[a, c, b, a, d, c]);
            }
        }
    }

    // Perimeter SKIRTS (chunked-LOD standard practice): the two faces meeting at
    // a cube edge compute the shared corners through different trig/permutation
    // paths, so positions can disagree by a last ulp — a sub-pixel hairline that
    // shows as background-black speckle. A short inward ribbon behind each edge
    // makes any such crack show terrain colour instead.
    let drop_km = 60.0;
    let edges: [Vec<u32>; 4] = [
        (0..n1).map(|i| idx(i, 0)).collect(),
        (0..n1).map(|i| idx(i, n1 - 1)).collect(),
        (0..n1).map(|j| idx(0, j)).collect(),
        (0..n1).map(|j| idx(n1 - 1, j)).collect(),
    ];
    for edge in edges {
        let base = positions.len() as u32;
        for &v in &edge {
            let p = Vec3::from(positions[v as usize]);
            let sunk = p * (1.0 - drop_km / p.length().max(1.0));
            positions.push(sunk.to_array());
            normals.push(normals[v as usize]);
            colors.push(colors[v as usize]);
        }
        for k in 0..edge.len() - 1 {
            let (a, b) = (edge[k], edge[k + 1]);
            let (sa, sb) = (base + k as u32, base + k as u32 + 1);
            indices.extend_from_slice(&[a, b, sb, a, sb, sa, a, sb, b, a, sa, sb]);
        }
    }

    (FaceMesh { positions, normals, colors, indices }, seam)
}
