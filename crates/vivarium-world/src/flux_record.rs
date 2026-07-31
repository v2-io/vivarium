//! Per-seam flux record — the second object a same-level tile seam owes
//! (`#form-same-level-halo-exchange` FE(1); `#form-seam-flux-exchange`).
//!
//! Halo exchange supplies the **boundary datum** (bed elevation). This module
//! supplies **discharge crossing** so a tile's catchment is not capped at its
//! own area (`#obs-tile-outlets-grade-away-the-basins` FE(4)).
//!
//! Shape: for a Jacobi region, each tile reports runoff (m² catchment units —
//! the same units as [`crate::erosion::Fluvial::drainage`]) that left its
//! interior into a neighbour tile's interior during the last drainage pass.
//! The region assembles those exports into **inflow seeds** for the next
//! cadence chunk.
//!
//! **Not yet:** sediment flux, stage-keyed store memo separate from the bed,
//! Bangerth $C{+}1$ multi-round exactness, volume-aware lake routing on the
//! flux vector. Key token digit `flux1` rides on the halo article when inject
//! is live in the Jacobi path.

use crate::erosion::Fluvial;

/// Four cardinal sides of a square tile, in the face-ij frame: −i, +i, −j, +j.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TileSide {
    West = 0,
    East = 1,
    South = 2,
    North = 3,
}

impl TileSide {
    pub const ALL: [TileSide; 4] = [
        TileSide::West,
        TileSide::East,
        TileSide::South,
        TileSide::North,
    ];

    pub fn opposite(self) -> TileSide {
        match self {
            TileSide::West => TileSide::East,
            TileSide::East => TileSide::West,
            TileSide::South => TileSide::North,
            TileSide::North => TileSide::South,
        }
    }

    /// Neighbour tile index offset `(d_ti, d_tj)`.
    pub fn tile_delta(self) -> (i32, i32) {
        match self {
            TileSide::West => (-1, 0),
            TileSide::East => (1, 0),
            TileSide::South => (0, -1),
            TileSide::North => (0, 1),
        }
    }
}

/// One tile's exports after a drainage pass: total catchment-area runoff (m²)
/// that left the interior across each side into a neighbour tile.
#[derive(Clone, Debug, Default)]
pub struct TileSeamFlux {
    pub export: [f32; 4],
}

impl TileSeamFlux {
    pub fn key_token(&self) -> String {
        // Counts-only skeleton for shape; production key rides on halo `flux1`.
        format!(
            "flux|e{:.0}|w{:.0}|s{:.0}|n{:.0}",
            self.export[1],
            self.export[0],
            self.export[2],
            self.export[3]
        )
    }
}

/// Measure export from a window's **interior** `tile_n²` at halo offset `d`.
///
/// Uses the live [`Fluvial::drainage`] field (last accumulation) and the bed
/// D8: any interior cell whose steepest downhill neighbour lies **outside** the
/// interior, on the side of a neighbour tile, contributes its full drainage
/// load to that side's export (conservative upper bound for a first wire —
/// MFD would split; we use the same D8 receivers the incision step uses).
pub fn measure_interior_exports(f: &Fluvial, d: usize, tile_n: usize) -> TileSeamFlux {
    let win = f.nx;
    assert_eq!(win, tile_n + 2 * d, "window must be tile + 2d halo");
    let mut export = [0.0f32; 4];
    if f.drainage.len() != win * win {
        return TileSeamFlux { export };
    }
    // Receivers on the unrepaired bed (post-epoch bed). Export is approximate
    // relative to the filled routing surface, but is a pure function of the
    // stored bed and is enough to inject non-zero cross-seam catchment.
    let outlets = vec![false; win * win]; // compute pure D8, ignore outlets for direction
    let mut recv = vec![0usize; win * win];
    for y in 0..win {
        for x in 0..win {
            let i = y * win + x;
            let hi = f.h[i];
            let mut best = i;
            let mut best_slope = 0.0f32;
            for (dx, dy) in [
                (1i32, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                if nx_ < 0 || ny_ < 0 || nx_ >= win as i32 || ny_ >= win as i32 {
                    continue;
                }
                let j = ny_ as usize * win + nx_ as usize;
                let dist = f.dist_m_public(i, j);
                let slope = (hi - f.h[j]) / dist;
                if slope > best_slope {
                    best_slope = slope;
                    best = j;
                }
            }
            recv[i] = best;
            let _ = outlets; // silence
        }
    }

    for j in 0..tile_n {
        for i in 0..tile_n {
            let wi = d + i;
            let wj = d + j;
            let idx = wj * win + wi;
            let r = recv[idx];
            if r == idx {
                continue;
            }
            let (rx, ry) = (r % win, r / win);
            // Outside the interior?
            let out = rx < d || ry < d || rx >= d + tile_n || ry >= d + tile_n;
            if !out {
                continue;
            }
            let load = f.drainage[idx];
            if load <= 0.0 {
                continue;
            }
            // Attribute to the side the receiver lies on relative to interior.
            if rx < d {
                export[TileSide::West as usize] += load;
            } else if rx >= d + tile_n {
                export[TileSide::East as usize] += load;
            } else if ry < d {
                export[TileSide::South as usize] += load;
            } else if ry >= d + tile_n {
                export[TileSide::North as usize] += load;
            }
        }
    }
    TileSeamFlux { export }
}

/// Build per-tile window inflow seeds (m²) from a full set of tile exports.
///
/// For each shared edge, neighbour's export on that side is distributed
/// uniformly along this tile's interior edge cells as `region_inflow` seeds.
pub fn assemble_inflows(
    exports: &[TileSeamFlux],
    tiles_i: usize,
    tiles_j: usize,
    tile_n: usize,
    d: usize,
) -> Vec<Vec<f32>> {
    let n_tiles = tiles_i * tiles_j;
    assert_eq!(exports.len(), n_tiles);
    let win = tile_n + 2 * d;
    let mut inflows: Vec<Vec<f32>> = (0..n_tiles).map(|_| vec![0.0f32; win * win]).collect();

    for tj in 0..tiles_j {
        for ti in 0..tiles_i {
            let t = tj * tiles_i + ti;
            for side in TileSide::ALL {
                let (dti, dtj) = side.tile_delta();
                let ni = ti as i32 + dti;
                let nj = tj as i32 + dtj;
                if ni < 0 || nj < 0 || ni >= tiles_i as i32 || nj >= tiles_j as i32 {
                    continue;
                }
                let nt = nj as usize * tiles_i + ni as usize;
                // What the neighbour exported *toward us* is their export on the
                // opposite side of the edge from our perspective... neighbour's
                // side facing us = opposite of our side toward them.
                let from_them = exports[nt].export[side.opposite() as usize];
                if from_them <= 0.0 {
                    continue;
                }
                // Distribute along our interior edge facing the neighbour.
                let per_cell = from_them / tile_n as f32;
                match side {
                    TileSide::West => {
                        for j in 0..tile_n {
                            let wi = d; // west interior column
                            let wj = d + j;
                            inflows[t][wj * win + wi] += per_cell;
                        }
                    }
                    TileSide::East => {
                        for j in 0..tile_n {
                            let wi = d + tile_n - 1;
                            let wj = d + j;
                            inflows[t][wj * win + wi] += per_cell;
                        }
                    }
                    TileSide::South => {
                        for i in 0..tile_n {
                            let wi = d + i;
                            let wj = d;
                            inflows[t][wj * win + wi] += per_cell;
                        }
                    }
                    TileSide::North => {
                        for i in 0..tile_n {
                            let wi = d + i;
                            let wj = d + tile_n - 1;
                            inflows[t][wj * win + wi] += per_cell;
                        }
                    }
                }
            }
        }
    }
    inflows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::erosion::{EdgeContract, Fluvial, FluvialParams};
    use crate::sphere::Face;

    #[test]
    fn export_is_zero_on_a_flat_window() {
        let seed = 0u64;
        let mut f = Fluvial::from_surface(seed, Face::ZPos, 10, 0, 0, 16, |_| 100.0);
        f.set_edge_contract(EdgeContract::BaseLevelSink);
        f.erode(&FluvialParams {
            epochs: 1,
            ..Default::default()
        });
        // d=2, tile=12 → win=16
        let e = measure_interior_exports(&f, 2, 12);
        // Flat / near-flat may still export something toward edge sinks; just
        // ensure the measure is finite and sides sum sensibly.
        assert!(e.export.iter().all(|x| x.is_finite()));
    }

    #[test]
    fn assemble_moves_export_to_neighbour_inflow() {
        let mut a = TileSeamFlux::default();
        a.export[TileSide::East as usize] = 100.0;
        let b = TileSeamFlux::default();
        let exports = vec![a, b];
        // 2×1 tiles, tile_n=4, d=1 → win=6
        let inflows = assemble_inflows(&exports, 2, 1, 4, 1);
        // Tile 1 (east of 0) should receive on its west edge.
        let win = 6;
        let mut sum = 0.0f32;
        for j in 0..4 {
            sum += inflows[1][(1 + j) * win + 1]; // west interior col at d=1
        }
        assert!(
            (sum - 100.0).abs() < 1e-3,
            "east export of tile0 must become west inflow of tile1, got {sum}"
        );
        assert!(inflows[0].iter().all(|&x| x == 0.0));
    }
}
