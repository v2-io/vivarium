//! Sill graph — the non-local half of a same-level seam's boundary datum
//! (`#form-same-level-halo-exchange` FE(9)).
//!
//! A halo of bed elevations supplies the **local** boundary datum (where
//! downhill is on the rim). Closed basins that straddle a seam also need the
//! **spill structure** of the whole depression: that is a property of a graph
//! of basins and the sill elevations that join them, not of any single scalar
//! per basin (the per-basin scalar is constructively refuted in FE(9)).
//!
//! This module is the first code home for that object:
//! - extract a tile/window's sill graph from a heightfield + outlets
//! - flood the graph (Priority-Flood on basins) → per-basin spill levels
//! - convict the FE(9) 1-D profile: local scalar exchange freezes at the wrong
//!   spill; the whole-domain graph finds the true one
//!
//! **Not yet:** store memo / key wiring, Jacobi exchange integration, volume-
//! aware fill–spill–merge hierarchy (the edge set here is the dry skeleton of
//! that hierarchy — FE(9) + dossier §3.5). Those are the next list items.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Sentinel basin id: the outlet / ocean set (any cell marked as an outlet).
pub const OUTLET: u32 = u32::MAX;

/// One local depression (pit catchment).
#[derive(Clone, Debug, PartialEq)]
pub struct BasinNode {
    pub id: u32,
    /// Lowest bed elevation in the basin (m).
    pub floor_m: f32,
    /// True if any cell of this basin lies on the domain perimeter — a candidate
    /// to straddle a seam with a neighbour tile.
    pub touches_perimeter: bool,
}

/// An undirected sill between two basins (or a basin and [`OUTLET`]).
///
/// `sill_m` is the elevation of the lowest pass connecting the two components.
#[derive(Clone, Debug, PartialEq)]
pub struct SillEdge {
    pub a: u32,
    pub b: u32,
    pub sill_m: f32,
}

/// A tile's (or window's) sill graph.
///
/// Content is \(O(\text{basins} + \text{perimeter})\). The per-basin water level
/// is **not** stored: it is an *output* of assembling neighbouring graphs and
/// flooding (`#form-same-level-halo-exchange` FE(9)).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SillGraph {
    pub basins: Vec<BasinNode>,
    pub edges: Vec<SillEdge>,
    /// Perimeter cells that could meet a neighbour: `(local_cell_index, bed_m,
    /// basin_id)`.
    pub perimeter_portals: Vec<(usize, f32, u32)>,
}

const NEIGHBORS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl SillGraph {
    /// Canonical key token for a memo that carries this graph beside a stage
    /// residual (`#form-time-indexed-stage-chains` FE(3) precedent). Descriptor
    /// shape only — payload bytes are separate. Stable under reordering.
    pub fn key_token(&self) -> String {
        let mut basins: Vec<_> = self
            .basins
            .iter()
            .map(|b| (b.id, b.floor_m.to_bits(), b.touches_perimeter as u8))
            .collect();
        basins.sort_by_key(|t| t.0);
        let mut edges: Vec<_> = self
            .edges
            .iter()
            .map(|e| {
                let (lo, hi) = if e.a < e.b { (e.a, e.b) } else { (e.b, e.a) };
                (lo, hi, e.sill_m.to_bits())
            })
            .collect();
        edges.sort();
        format!(
            "sill|b{}|e{}|p{}",
            basins.len(),
            edges.len(),
            self.perimeter_portals.len()
        )
    }

    /// Extract the sill graph of a rectangular heightfield (`nx` columns ×
    /// `ny` rows, row-major) under a declared outlet mask.
    ///
    /// 1. Each non-outlet local minimum (no downhill or flat-drain neighbour) is
    ///    a basin.
    /// 2. Every other cell is assigned to the basin (or outlet) reached by
    ///    steepest-descent on the bed (flats drain deterministically by index).
    /// 3. Each pair of adjacent cells with different terminals contributes a
    ///    sill at \(\max(h_u, h_v)\); the graph keeps the **minimum** such pass
    ///    between each pair of terminals.
    pub fn extract(h: &[f32], nx: usize, outlets: &[bool]) -> Self {
        assert_eq!(h.len() % nx, 0, "h length must be nx * ny");
        let ny = h.len() / nx;
        Self::extract_rect(h, nx, ny, outlets)
    }

    /// Square convenience: `nx × nx`.
    pub fn extract_square(h: &[f32], nx: usize, outlets: &[bool]) -> Self {
        assert_eq!(h.len(), nx * nx);
        Self::extract_rect(h, nx, nx, outlets)
    }

    pub fn extract_rect(h: &[f32], nx: usize, ny: usize, outlets: &[bool]) -> Self {
        assert_eq!(h.len(), nx * ny);
        assert_eq!(outlets.len(), nx * ny);
        let n = h.len();
        let nxi = nx as i32;
        let nyi = ny as i32;

        let is_edge = |i: usize| {
            let (x, y) = (i % nx, i / nx);
            x == 0 || y == 0 || x + 1 == nx || y + 1 == ny
        };

        // Steepest-descent receiver; outlets and true pits point to self.
        // On flats (no strict downhill): drain to the equal-height neighbour
        // with the smallest index — deterministic, and enough for FE(9) + unit
        // tests; production flats still go through Priority-Flood ε on the bed.
        let mut recv = vec![0usize; n];
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                if outlets[i] {
                    recv[i] = i;
                    continue;
                }
                let hi = h[i];
                let mut best_down: Option<(f32, usize)> = None; // (drop, j)
                let mut best_flat: Option<usize> = None;
                for (dx, dy) in NEIGHBORS {
                    let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                    if nx_ < 0 || ny_ < 0 || nx_ >= nxi || ny_ >= nyi {
                        continue;
                    }
                    let j = ny_ as usize * nx + nx_ as usize;
                    let drop = hi - h[j];
                    if drop > 0.0 {
                        if best_down.is_none_or(|(d, bj)| drop > d || (drop == d && j < bj)) {
                            best_down = Some((drop, j));
                        }
                    } else if drop == 0.0 {
                        if best_flat.is_none_or(|bj| j < bj) {
                            best_flat = Some(j);
                        }
                    }
                }
                recv[i] = if let Some((_, j)) = best_down {
                    j
                } else if let Some(j) = best_flat {
                    j
                } else {
                    i // true local minimum
                };
            }
        }

        // Terminal: follow receivers; detect cycles (treat cycle min as pit).
        let mut terminal = vec![usize::MAX; n];
        for i in 0..n {
            if terminal[i] != usize::MAX {
                continue;
            }
            let mut path = Vec::new();
            let mut cur = i;
            let mut seen = HashMap::new();
            loop {
                if let Some(&t) = seen.get(&cur) {
                    // Cycle: terminal is lowest cell on the cycle.
                    let cycle = &path[t..];
                    let pit = *cycle
                        .iter()
                        .min_by(|a: &&usize, b: &&usize| h[**a].total_cmp(&h[**b]).then_with(|| a.cmp(b)))
                        .unwrap();
                    for &c in cycle {
                        terminal[c] = pit;
                    }
                    // Also map the approach path to that pit.
                    for &c in &path[..t] {
                        terminal[c] = pit;
                    }
                    break;
                }
                if terminal[cur] != usize::MAX {
                    let t = terminal[cur];
                    for &c in &path {
                        terminal[c] = t;
                    }
                    break;
                }
                seen.insert(cur, path.len());
                path.push(cur);
                if outlets[cur] || recv[cur] == cur {
                    for &c in &path {
                        terminal[c] = cur;
                    }
                    break;
                }
                cur = recv[cur];
            }
        }

        // Basin ids for pit terminals (not outlets).
        let mut pit_ids: HashMap<usize, u32> = HashMap::new();
        let mut next_id = 0u32;
        for i in 0..n {
            let t = terminal[i];
            if !outlets[t] && !pit_ids.contains_key(&t) {
                pit_ids.insert(t, next_id);
                next_id += 1;
            }
        }

        let term_label = |t: usize| -> u32 {
            if outlets[t] {
                OUTLET
            } else {
                pit_ids[&t]
            }
        };

        let mut floors = vec![f32::INFINITY; next_id as usize];
        let mut touches = vec![false; next_id as usize];
        for i in 0..n {
            let t = terminal[i];
            if outlets[t] {
                continue;
            }
            let id = pit_ids[&t] as usize;
            floors[id] = floors[id].min(h[i]);
            if is_edge(i) {
                touches[id] = true;
            }
        }

        let basins: Vec<BasinNode> = (0..next_id)
            .map(|id| BasinNode {
                id,
                floor_m: floors[id as usize],
                touches_perimeter: touches[id as usize],
            })
            .collect();

        // Boundary sills: min over adjacent pairs of max(h_u, h_v).
        let mut edges: Vec<SillEdge> = Vec::new();
        for y in 0..ny {
            for x in 0..nx {
                let i = y * nx + x;
                let li = term_label(terminal[i]);
                for (dx, dy) in NEIGHBORS {
                    let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                    if nx_ < 0 || ny_ < 0 || nx_ >= nxi || ny_ >= nyi {
                        continue;
                    }
                    let j = ny_ as usize * nx + nx_ as usize;
                    if j < i {
                        continue; // undirected, visit once
                    }
                    let lj = term_label(terminal[j]);
                    if li == lj {
                        continue;
                    }
                    let sill = h[i].max(h[j]);
                    push_edge(&mut edges, li, lj, sill);
                }
            }
        }

        let mut perimeter_portals = Vec::new();
        for i in 0..n {
            if is_edge(i) {
                perimeter_portals.push((i, h[i], term_label(terminal[i])));
            }
        }

        SillGraph {
            basins,
            edges,
            perimeter_portals,
        }
    }

    /// Flood this graph as Priority-Flood on basins: each basin's spill level is
    /// the min sill elevation on a path to [`OUTLET`] (at least its floor).
    pub fn spill_levels(&self) -> Vec<(u32, f32)> {
        let mut adj: HashMap<u32, Vec<(u32, f32)>> = HashMap::new();
        for e in &self.edges {
            adj.entry(e.a).or_default().push((e.b, e.sill_m));
            adj.entry(e.b).or_default().push((e.a, e.sill_m));
        }
        for b in &self.basins {
            adj.entry(b.id).or_default();
        }
        adj.entry(OUTLET).or_default();

        #[derive(Copy, Clone)]
        struct State {
            spill: f32,
            id: u32,
        }
        impl PartialEq for State {
            fn eq(&self, o: &Self) -> bool {
                self.spill == o.spill && self.id == o.id
            }
        }
        impl Eq for State {}
        impl Ord for State {
            fn cmp(&self, o: &Self) -> Ordering {
                o.spill.total_cmp(&self.spill).then_with(|| self.id.cmp(&o.id))
            }
        }
        impl PartialOrd for State {
            fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
                Some(self.cmp(o))
            }
        }

        let mut best: HashMap<u32, f32> = HashMap::new();
        let mut heap = BinaryHeap::new();
        best.insert(OUTLET, f32::NEG_INFINITY);
        heap.push(State {
            spill: f32::NEG_INFINITY,
            id: OUTLET,
        });

        while let Some(State { spill, id }) = heap.pop() {
            if best.get(&id).is_some_and(|&b| spill > b) {
                continue;
            }
            for &(nb, sill) in adj.get(&id).into_iter().flatten() {
                let next = spill.max(sill);
                if best.get(&nb).is_none_or(|&b| next < b) {
                    best.insert(nb, next);
                    heap.push(State { spill: next, id: nb });
                }
            }
        }

        self.basins
            .iter()
            .map(|b| {
                let s = best.get(&b.id).copied().unwrap_or(f32::INFINITY);
                let s = if s.is_finite() { s.max(b.floor_m) } else { s };
                (b.id, s)
            })
            .collect()
    }

    /// Join two graphs across a seam. `seam_edges` are `(basin_in_self,
    /// basin_in_other, sill_m)`; other basin ids are offset by `id_offset`.
    pub fn assemble_with(&self, other: &SillGraph, id_offset: u32, seam_edges: &[(u32, u32, f32)]) -> SillGraph {
        let mut basins = self.basins.clone();
        for b in &other.basins {
            basins.push(BasinNode {
                id: b.id.saturating_add(id_offset),
                floor_m: b.floor_m,
                touches_perimeter: b.touches_perimeter,
            });
        }
        let mut edges = self.edges.clone();
        for e in &other.edges {
            let a = if e.a == OUTLET { OUTLET } else { e.a.saturating_add(id_offset) };
            let b = if e.b == OUTLET { OUTLET } else { e.b.saturating_add(id_offset) };
            push_edge(&mut edges, a, b, e.sill_m);
        }
        for &(a, b_raw, sill) in seam_edges {
            let b = if b_raw == OUTLET {
                OUTLET
            } else {
                b_raw.saturating_add(id_offset)
            };
            push_edge(&mut edges, a, b, sill);
        }
        SillGraph {
            basins,
            edges,
            perimeter_portals: Vec::new(),
        }
    }
}

fn push_edge(edges: &mut Vec<SillEdge>, a: u32, b: u32, sill_m: f32) {
    if a == b {
        return;
    }
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    if let Some(e) = edges.iter_mut().find(|e| {
        let (x, y) = if e.a < e.b { (e.a, e.b) } else { (e.b, e.a) };
        x == lo && y == hi
    }) {
        e.sill_m = e.sill_m.min(sill_m);
    } else {
        edges.push(SillEdge {
            a: lo,
            b: hi,
            sill_m,
        });
    }
}

/// FE(9) one-dimensional bed (metres), ocean at the left edge.
///
/// | cell | 0 (ocean) | 1 | 2 | 3 | — seam — | 4 | 5 | 6 |
/// | bed  | 0         | 8 | 2 | 5 |          | 3 | 1 | 20|
pub fn fe9_profile() -> Vec<f32> {
    vec![0.0, 8.0, 2.0, 5.0, 3.0, 1.0, 20.0]
}

/// FE(9) as a true 1×N strip (one row, `nx` cells).
pub fn fe9_as_field() -> (Vec<f32>, usize, Vec<bool>) {
    let h = fe9_profile();
    let nx = h.len();
    let mut outlets = vec![false; nx];
    outlets[0] = true;
    (h, nx, outlets)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole FE(9) profile as one domain: both pits fill to the true spill
    /// of 8 m (cell 1), not the seam sill of 5 m.
    #[test]
    fn fe9_whole_domain_spills_at_eight() {
        let (h, nx, outlets) = fe9_as_field();
        let g = SillGraph::extract_rect(&h, nx, 1, &outlets);
        assert!(
            g.basins.len() >= 2,
            "expected pits A and B, got {g:?}"
        );
        let levels = g.spill_levels();
        assert!(!levels.is_empty(), "graph {g:?}");
        for &(id, s) in &levels {
            assert!(
                (s - 8.0).abs() < 1e-3,
                "basin {id} spilled at {s}, want 8; graph={g:?}"
            );
        }
        // Sanity: we should have an edge path involving sill 5 and sill 8.
        assert!(
            g.edges.iter().any(|e| (e.sill_m - 5.0).abs() < 1e-3),
            "missing seam sill 5: {g:?}"
        );
        assert!(
            g.edges.iter().any(|e| (e.sill_m - 8.0).abs() < 1e-3),
            "missing true rim sill 8: {g:?}"
        );
    }

    /// Local scalar exchange freezes at the wrong fixed point (spill 5 on each
    /// side of the seam). The whole-domain sill graph reports 8 — breaking the
    /// two-cycle FE(9) constructs by hand.
    #[test]
    fn fe9_local_scalar_freezes_at_five_assembled_graph_finds_eight() {
        let (full, nx, _) = fe9_as_field();

        // Left local read: ocean + treat seam cell 3 as a sink (the "neighbour
        // takes my water" fiction of scalar exchange).
        let mut out_l = vec![false; nx];
        out_l[0] = true;
        out_l[3] = true;
        let g_l = SillGraph::extract_rect(&full, nx, 1, &out_l);
        let a_spill = g_l
            .spill_levels()
            .into_iter()
            .filter(|(id, _)| g_l.basins.iter().any(|b| b.id == *id && (b.floor_m - 2.0).abs() < 1e-3))
            .map(|(_, s)| s)
            .fold(f32::INFINITY, f32::min);
        assert!(
            (a_spill - 5.0).abs() < 1e-3,
            "local left (pit A floor 2) should freeze at 5, got {a_spill}; {g_l:?}"
        );

        // Right local read: seam cell 3 as sink; pit B freezes at 5.
        let mut out_r = vec![false; nx];
        out_r[3] = true;
        let g_r = SillGraph::extract_rect(&full, nx, 1, &out_r);
        let b_spill = g_r
            .spill_levels()
            .into_iter()
            .filter(|(id, _)| g_r.basins.iter().any(|b| b.id == *id && (b.floor_m - 1.0).abs() < 1e-3))
            .map(|(_, s)| s)
            .fold(f32::INFINITY, f32::min);
        assert!(
            (b_spill - 5.0).abs() < 1e-3,
            "local right (pit B floor 1) should freeze at 5, got {b_spill}; {g_r:?}"
        );

        // Assembled (whole-domain) graph: true spill 8.
        let mut out_full = vec![false; nx];
        out_full[0] = true;
        let g = SillGraph::extract_rect(&full, nx, 1, &out_full);
        for &(id, s) in &g.spill_levels() {
            assert!(
                (s - 8.0).abs() < 1e-3,
                "assembled basin {id} spill {s}, want 8; {g:?}"
            );
        }

        let tok = g.key_token();
        assert!(tok.starts_with("sill|"), "key token shape: {tok}");
        assert_eq!(tok, g.key_token());
    }

    #[test]
    fn flat_field_has_no_interior_basins() {
        let nx = 8usize;
        let h = vec![10.0f32; nx * nx];
        let mut outlets = vec![false; nx * nx];
        for i in 0..nx {
            outlets[i] = true;
        }
        let g = SillGraph::extract_square(&h, nx, &outlets);
        // Flats drain by index toward the outlet row; no closed pit remains.
        assert!(
            g.basins.is_empty(),
            "flat with outlet row should have no pits: {g:?}"
        );
    }

    #[test]
    fn assemble_with_offsets_ids() {
        let (full, nx, _) = fe9_as_field();
        let mut out_l = vec![false; nx];
        out_l[0] = true;
        out_l[3] = true;
        let g_l = SillGraph::extract_rect(&full, nx, 1, &out_l);
        let mut out_r = vec![false; nx];
        out_r[3] = true;
        let g_r = SillGraph::extract_rect(&full, nx, 1, &out_r);
        // Hand seam: A's side to B's side at sill 5.
        let a = g_l.basins.iter().find(|b| (b.floor_m - 2.0).abs() < 1e-3).map(|b| b.id);
        let b = g_r.basins.iter().find(|b| (b.floor_m - 1.0).abs() < 1e-3).map(|b| b.id);
        if let (Some(a), Some(b)) = (a, b) {
            let offset = 100u32;
            let assembled = g_l.assemble_with(&g_r, offset, &[(a, b, 5.0)]);
            assert!(assembled.basins.iter().any(|x| x.id == a));
            assert!(assembled.basins.iter().any(|x| x.id == b + offset));
        }
    }
}
