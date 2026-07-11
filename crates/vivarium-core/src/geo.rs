//! Geomorphology — the *abstraction tier* where mountains are pushed up and
//! eroded into recognizable forms. **This is a spike.** Its purpose is narrow
//! and empirical: find out whether deterministic stream-power erosion produces
//! genuinely *dendritic* drainage at scales we care about, and whether it does
//! so bit-identically from a seed. It is not yet wired into [`crate::voxel`];
//! see "What this is not, yet" below.
//!
//! ## Where this sits in the design
//!
//! doc/design/DESIGN.md (ratified 2026-06-22) treats the **planet as the abstraction tier**:
//! a slow world-creation simulation emits global fields (elevation, later
//! drainage / climate / strata) that the fast, Cartesian `seed + edits` voxel
//! world is materialized *from*. This module is the first piece of that slow
//! tier — a single global **elevation field**, evolved by uplift and fluvial
//! erosion. It deliberately runs on its own coarse grid (the "simulation grid"),
//! distinct from the voxel resolution a player walks on. The research is
//! unambiguous on why (see `ref/geology/NOTES.md` §2): emergent drainage is a
//! property of the *simulation* grid, not the render voxel, and first-order
//! channels are lost beyond ~22–24 m cells (CAESAR-Lisflood, ESurf 11:695 2023) —
//! so ~10–12 m is the target cell size when this becomes physical. Here the grid
//! is unitful; the cell size is a parameter, not yet pinned to metres.
//!
//! ## The pipeline (each verified against a primary source; `ref/geology/`)
//!
//! Per epoch, over a regular grid whose outer ring is held at base level (the
//! ocean the rivers run to):
//!
//!  1. **Uplift.** Interior elevations rise by `uplift · dt`. This is the
//!     tectonic forcing — the "mountains pushed up" half.
//!  2. **Depression filling** (Barnes, Lehman & Mulla 2014, *Priority-Flood*).
//!     Fill pits so every interior cell has a downhill path to the boundary,
//!     with a tiny ε-gradient across flats so flow direction is always defined.
//!     Determinism: the priority queue breaks ties by cell index, never by
//!     float-equality chance.
//!  3. **Flow routing (D8).** Each cell drains to its single steepest-descent
//!     neighbour (O'Callaghan & Mark 1984). After step 2 every interior cell has
//!     one, strictly lower (by ≥ ε), so the receiver relation is a forest rooted
//!     at the boundary — no cycles, fully orderable.
//!  4. **Drainage area.** Accumulate cell areas downstream. `A` is the
//!     discharge proxy that makes big rivers cut harder than rills.
//!  5. **Stream-power incision** (Whipple & Tucker 1999; solved implicitly à la
//!     Braun & Willett 2013). The geomorphic law `∂h/∂t = U − K·Aᵐ·Sⁿ`. We take
//!     **n = 1**, for which the implicit update is *linear and exact* in a single
//!     upstream pass — unconditionally stable (any `dt`) and, because the pass
//!     order is fixed, bit-deterministic. This is the "eroded into recognizable
//!     forms" half: incision organizes the uplifted block into ridges and a
//!     branching valley network.
//!  6. **Hillslope / talus** (Musgrave, Kolb & Mace 1989). Material steeper than
//!     the repose angle slumps downhill — the most voxel-native of the steps and
//!     trivially deterministic (computed from a snapshot, applied as a batch).
//!
//! ## Why an elevation-sorted order instead of Braun & Willett's O(n) stack
//!
//! Braun & Willett's real contribution is an O(n) traversal via an explicit
//! donor/stack construction (their §2). We instead derive the same
//! receiver-before-donor order by **sorting cells by filled elevation** — O(n log
//! n), a few lines, and far less to get subtly wrong in a spike. After step 2 the
//! ε-gradient guarantees receivers are strictly lower, so ascending-elevation
//! order processes every receiver before its donors (and the reverse for drainage
//! accumulation). Correct and identical in result; adopt the O(n) stack when grid
//! size makes the `log n` bite. (Flagged so a successor knows the swap is
//! available, not overlooked.)
//!
//! ## What this is not, yet (do not mistake the spike for the system)
//!
//!  - **Not wired into [`crate::voxel`].** It produces a heightfield; sampling
//!    that into voxel columns (and asserting the column world replays
//!    bit-identically) is the next increment — doc/design/DESIGN.md / NOTES.md §8 step 4.
//!  - **Not conservative.** Depression filling adds mass, and detachment-limited
//!    stream power has *no* deposition — sediment that erodes leaves the system.
//!    Real conservation (the multigrid-style "fine detail sums back to the
//!    coarse cell" invariant) is one of the two open research gaps in NOTES.md
//!    §7; not attempted here.
//!  - **Not on a sphere.** Flat patch on purpose — the sphere is the abstraction
//!    tier's *own* abstraction and can wait (NOTES.md §1).
//!  - **n = 1 only.** The general n ≠ 1 case needs Newton iteration per node
//!    (Braun & Willett §4, not yet read in full). n = 1 is the honest, exact,
//!    single-pass case to start from.
//!
//! ## First-run result (2026-06-22) — what the spike actually showed
//!
//! Run `cargo run -p vivarium-core --example erosion_preview` and read the
//! drainage map. Three of the four things we wanted hold cleanly: the field is
//! **bit-identical across runs**, flow **concentrates into a hierarchical
//! network** (rills → tributaries → trunk rivers, joining at confluences — the
//! drainage *topology* is a tree), and the **talus cap** behaves. The fourth is
//! the honest catch: the network **geometry is grid-locked** — trunks run as
//! near-straight vertical/diagonal lines rather than sinuous dendrites. This is
//! not a bug; it is precisely the **D8 single-flow-direction anisotropy** the
//! research flagged (NOTES.md §2; PNAS arXiv:1911.03519), where channels snap to
//! the 8 compass directions. The validated next increment is therefore
//! **multiple-flow-direction / D∞ drainage routing** (Tarboton) for the area
//! accumulation, keeping the single-receiver tree for the implicit erosion solve.
//! The spike succeeded at its real job: proving the deterministic pipeline works
//! *and* surfacing the exact, named limitation to fix next.

use crate::Rng;

/// Parameters of one erosion run. Defaults are tuned for a *visible* result on a
/// modest grid, not for physical realism — this is a spike. The stream-power
/// constants follow the Schott et al. reference implementation's defaults
/// (`ref/geology/StreamPowerErosion`): `m = 0.8 · n`-ish concavity, here taken at
/// `m = 0.5, n = 1` (the classic detachment-limited values, Whipple & Tucker
/// 1999), with `k` scaled up so erosion keeps pace with uplift in few epochs.
#[derive(Clone, Debug)]
pub struct ErosionParams {
    /// Grid edge length in cells (the simulation grid is `nx × nx`).
    pub nx: usize,
    /// Physical size of one cell. Unitful for now; ~10–12 m when this becomes
    /// metric (NOTES.md §2). Only ratios with `uplift`/`k` matter to the shape.
    pub cell_size: f32,
    /// Tectonic uplift rate of interior cells, per epoch-second.
    pub uplift: f32,
    /// Erodibility `K` in `E = K·Aᵐ·Sⁿ`. Higher ⇒ deeper, faster-cut valleys.
    pub k: f32,
    /// Drainage-area exponent `m`.
    pub m: f32,
    /// Talus repose: the maximum stable slope (rise/run). Slopes above this slump.
    pub max_slope: f32,
    /// Epochs to run, and the timestep of each. The implicit scheme is stable at
    /// any `dt`, so few large steps reach a mature landscape cheaply.
    pub epochs: u32,
    pub dt: f32,
    /// Optional waterline. When `Some(s)`, any cell at or below `s` is a drainage
    /// **outlet** — the ocean the rivers run to — and is never uplifted. This is
    /// what lets erosion run over a real, pre-existing landscape (e.g. the voxel
    /// world's Perlin terrain) and carve valleys down to its coastlines, rather
    /// than only draining off the grid edge. `None` keeps the from-scratch
    /// behaviour: the outer ring is the only base level.
    pub sea_level: Option<f32>,
    /// **Deposition efficiency `G`** (dimensionless; `0` = pure detachment, the
    /// old incision-only behaviour). The Davy & Lague (2009) deposition law: a
    /// reach lays down sediment at rate `D = G·Qs/A`, where `Qs` is the sediment
    /// flux it carries and `A` its drainage area (so `Qs/A` is a length-rate and
    /// `G` is a pure number — the settling/transport efficiency). Because
    /// deposition scales with the *flux it is already carrying* rather than an
    /// arbitrary capacity threshold, it concentrates exactly where erosion has
    /// gone quiet — slack reaches near base level — grading valley floors and
    /// building floodplains, **without** filling the steep upland channels (which
    /// keep net-incising). Higher `G` ⇒ more sediment settles before reaching the
    /// sea. (The mechanism behind CAESAR-Lisflood / FastScape-with-deposition.)
    pub deposition: f32,
}

impl Default for ErosionParams {
    fn default() -> Self {
        Self {
            nx: 192,
            cell_size: 1.0,
            uplift: 0.01,
            k: 0.30,
            m: 0.5,
            max_slope: 1.0, // ~45° repose for the talus pass
            epochs: 60,
            dt: 1.0,
            sea_level: None,
            deposition: 0.0, // pure detachment unless a caller opts into deposition
        }
    }
}

/// A square elevation field and the drainage state derived from it. Plain data,
/// cheap to clone and to diff — the same discipline as [`crate::voxel::Volume`].
#[derive(Clone, Debug)]
pub struct Heightfield {
    pub nx: usize,
    pub cell_size: f32,
    /// Row-major elevations, length `nx · nx`. Index via [`Self::idx`].
    pub h: Vec<f32>,
    /// Accumulated drainage area per cell from the last erosion epoch — the
    /// thing to visualize to *see* whether the network is dendritic.
    pub drainage: Vec<f32>,
}

/// The 8 neighbour offsets, in a fixed order. Determinism note: this order is
/// the tie-break for "steepest descent" — when two neighbours tie exactly, the
/// earlier one wins, the same way on every run and every machine.
const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

impl Heightfield {
    #[inline]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.nx + x
    }

    #[inline]
    fn is_boundary(nx: usize, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == nx - 1 || y == nx - 1
    }

    /// Build the initial field: low, near-flat, with small seeded noise only.
    ///
    /// The noise is not decoration — it is *load-bearing*, and it is deliberately
    /// the *only* structure. This is the textbook landscape-evolution symmetry-
    /// breaking setup: **uniform** uplift over a near-flat interior with a fixed
    /// low boundary, where the drainage network self-organizes entirely from the
    /// competition between uplift and incision amplifying tiny initial
    /// irregularities. An imposed shape (e.g. a central dome) biases the result
    /// toward radial spokes and a false central divide — so we impose none, and
    /// let the boundary and the physics do the organizing.
    fn seeded(params: &ErosionParams, seed: u64) -> Self {
        let nx = params.nx;
        let mut rng = Rng::new(seed);
        let mut h = vec![0.0f32; nx * nx];
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                // Tiny random roughness; the outer ring stays at base level 0
                // (the coastline the rivers run to).
                h[y * nx + x] = rng.next_f32() * 0.5;
            }
        }
        Self { nx, cell_size: params.cell_size, h, drainage: vec![0.0; nx * nx] }
    }

    /// Wrap an existing heightfield (e.g. sampled from the voxel world's terrain)
    /// so erosion can be run *over* it. The drainage field starts empty and is
    /// filled by the first epoch.
    pub fn from_heights(nx: usize, cell_size: f32, h: Vec<f32>) -> Self {
        assert_eq!(h.len(), nx * nx, "heights must be nx*nx");
        Self { nx, cell_size, drainage: vec![0.0; nx * nx], h }
    }

    /// Run the full uplift→erode simulation from a seeded blank slate.
    pub fn simulate(params: &ErosionParams, seed: u64) -> Self {
        Self::seeded(params, seed).erode(params)
    }

    /// Run the uplift→erode loop in place on whatever field this already is, and
    /// return it. Used both by [`Self::simulate`] (from `seeded`) and to erode a
    /// real pre-existing landscape built with [`Self::from_heights`].
    pub fn erode(mut self, params: &ErosionParams) -> Self {
        for _ in 0..params.epochs {
            let outlets = self.outlets(params);
            self.uplift(params, &outlets);
            self.fill_depressions(&outlets);
            let receivers = self.receivers(&outlets);
            let order = self.elevation_order();
            self.accumulate_drainage(&order);
            // Snapshot before incision so the deposition pass knows how much
            // sediment each cell produced this epoch.
            let pre_incision = if params.deposition > 0.0 {
                Some(self.h.clone())
            } else {
                None
            };
            self.incise(params, &receivers, &order);
            if let Some(before) = pre_incision {
                self.deposit(params, &receivers, &order, &before);
            }
            self.talus(params);
        }
        self
    }

    /// Which cells are fixed drainage outlets this epoch: always the outer ring;
    /// additionally every cell at or below `sea_level` when one is set (the
    /// coastline rivers run to). Recomputed each epoch because erosion moves cells
    /// across the waterline. Deterministic — a pure function of the current field.
    fn outlets(&self, params: &ErosionParams) -> Vec<bool> {
        let nx = self.nx;
        let mut out = vec![false; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = self.idx(x, y);
                out[i] = Self::is_boundary(nx, x, y)
                    || params.sea_level.is_some_and(|s| self.h[i] <= s);
            }
        }
        out
    }

    /// Step 1 — raise non-outlet cells; outlets (boundary ring + any ocean) are
    /// the fixed base level the landscape drains toward.
    fn uplift(&mut self, params: &ErosionParams, outlets: &[bool]) {
        let rise = params.uplift * params.dt;
        for (i, &is_outlet) in outlets.iter().enumerate() {
            if !is_outlet {
                self.h[i] += rise;
            }
        }
    }

    /// Step 2 — Priority-Flood depression filling with an ε-gradient across flats
    /// (Barnes, Lehman & Mulla 2014). Floods inward from the boundary; every cell
    /// is raised to at least the lowest spill elevation that reaches it, plus a
    /// tiny increment so the filled surface always slopes *toward* the outlet and
    /// flow direction is never ambiguous on a plateau.
    ///
    /// Determinism: the frontier is a min-heap keyed by `(elevation, insertion
    /// index)`. Float ties never decide order — the integer index does — so two
    /// runs pop cells in exactly the same sequence.
    fn fill_depressions(&mut self, outlets: &[bool]) {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        let nx = self.nx;
        const EPS: f32 = 1e-4; // flat-gradient increment; small vs. relief

        // Min-heap entry. `BinaryHeap` is a max-heap, so we invert the compare.
        struct Cell {
            elev: f32,
            seq: u64, // insertion order: the deterministic tie-break
            i: usize,
        }
        impl PartialEq for Cell {
            fn eq(&self, o: &Self) -> bool {
                self.elev == o.elev && self.seq == o.seq
            }
        }
        impl Eq for Cell {}
        impl Ord for Cell {
            fn cmp(&self, o: &Self) -> Ordering {
                // Lowest elevation first; ties by lowest insertion seq. Inverted
                // for the max-heap. `total_cmp` keeps it total over all f32.
                o.elev
                    .total_cmp(&self.elev)
                    .then_with(|| o.seq.cmp(&self.seq))
            }
        }
        impl PartialOrd for Cell {
            fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
                Some(self.cmp(o))
            }
        }

        let mut closed = vec![false; nx * nx];
        let mut heap = BinaryHeap::new();
        let mut seq = 0u64;

        // Seed the frontier with every outlet (boundary ring + any ocean) at its
        // own elevation. Ocean cells are drains, not pits, so they must not be
        // filled — seeding them is what keeps an inland sea from being flooded shut.
        for (i, &is_outlet) in outlets.iter().enumerate() {
            if is_outlet {
                closed[i] = true;
                heap.push(Cell { elev: self.h[i], seq, i });
                seq += 1;
            }
        }

        while let Some(Cell { elev, i, .. }) = heap.pop() {
            let (x, y) = (i % nx, i / nx);
            for (dx, dy) in NEIGHBORS {
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                if closed[j] {
                    continue;
                }
                closed[j] = true;
                // Raise to the spill level + ε so the path out is always downhill.
                self.h[j] = self.h[j].max(elev + EPS);
                heap.push(Cell { elev: self.h[j], seq, i: j });
                seq += 1;
            }
        }
    }

    /// Step 3 — steepest-descent (D8) receiver of each cell. Boundary cells (and,
    /// defensively, any cell with no lower neighbour) are their own receiver: a
    /// drainage outlet. After [`Self::fill_depressions`] every interior cell has a
    /// strictly-lower neighbour, so the relation is a forest rooted at the ring.
    fn receivers(&self, outlets: &[bool]) -> Vec<usize> {
        let nx = self.nx;
        let mut recv = vec![0usize; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = self.idx(x, y);
                if outlets[i] {
                    recv[i] = i; // an outlet drains to itself (sea / grid edge)
                    continue;
                }
                let hi = self.h[i];
                let mut best = i;
                let mut best_slope = 0.0f32;
                for (dx, dy) in NEIGHBORS {
                    let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                    let dist = if dx != 0 && dy != 0 {
                        self.cell_size * std::f32::consts::SQRT_2
                    } else {
                        self.cell_size
                    };
                    let slope = (hi - self.h[j]) / dist;
                    if slope > best_slope {
                        best_slope = slope;
                        best = j;
                    }
                }
                recv[i] = best;
            }
        }
        recv
    }

    /// A processing order with every receiver *before* its donors: cell indices
    /// sorted by ascending elevation, ties broken by index. (See the module note
    /// on why this stands in for Braun & Willett's O(n) stack.)
    fn elevation_order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.h.len()).collect();
        order.sort_by(|&a, &b| {
            self.h[a].total_cmp(&self.h[b]).then_with(|| a.cmp(&b))
        });
        order
    }

    /// Step 4 — drainage area by **multiple-flow-direction** (MFD) accumulation.
    /// Walking high→low, each cell distributes its accumulated area across *all*
    /// downslope neighbours, weighted by slopeᵖ. Unlike single-flow-direction
    /// (D8), which sends all flow to one of 8 compass neighbours and so imprints a
    /// grid-aligned drainage network (the rib artifact), MFD spreads flow along
    /// the true gradient, dissolving the axis/diagonal bias into natural dendritic
    /// dissection. Still O(n) in the elevation order, still fully deterministic
    /// (fixed neighbour order, fixed processing order).
    ///
    /// `incise` still cuts toward the single steepest receiver (the implicit
    /// stream-power solve needs a tree); MFD only governs the *area* `A` that
    /// feeds `K·Aᵐ·Sⁿ` — which is what dominates where channels form, so this is
    /// where breaking the grid bias matters most.
    fn accumulate_drainage(&mut self, order: &[usize]) {
        const P: f32 = 1.1; // flow-partition exponent (Quinn et al.); >1 sharpens channels
        let nx = self.nx;
        let cell_area = self.cell_size * self.cell_size;
        for d in self.drainage.iter_mut() {
            *d = cell_area;
        }
        // High → low: a cell's full upslope contribution is gathered before it is
        // processed (everything higher came earlier), then handed downslope.
        for &i in order.iter().rev() {
            let (x, y) = (i % nx, i / nx);
            let hi = self.h[i];
            let mut weights = [0.0f32; 8];
            let mut total = 0.0f32;
            for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                let drop = hi - self.h[j];
                if drop > 0.0 {
                    let dist = if *dx != 0 && *dy != 0 {
                        self.cell_size * std::f32::consts::SQRT_2
                    } else {
                        self.cell_size
                    };
                    let w = (drop / dist).powf(P);
                    weights[k] = w;
                    total += w;
                }
            }
            if total > 0.0 {
                let amount = self.drainage[i];
                for (k, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                    if weights[k] > 0.0 {
                        let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                        self.drainage[j] += amount * (weights[k] / total);
                    }
                }
            }
        }
    }

    /// Step 5 — implicit stream-power incision, n = 1. Walking low→high (so each
    /// receiver is already updated), solve the per-cell linear equation
    ///
    /// ```text
    ///   h_i^{t+1} = h_i^* + K·dt·A_iᵐ/L · h_r^{t+1}
    ///               ─────────────────────────────────
    ///                    1 + K·dt·A_iᵐ/L
    /// ```
    ///
    /// where `h_i^*` is the post-uplift elevation and `h_r` the (already solved)
    /// receiver elevation. This is the exact, unconditionally-stable n = 1 update;
    /// boundary cells are pinned. Slopes here are always ≥ 0 (we eroded the filled
    /// surface), so no negative-incision pathology.
    fn incise(&mut self, params: &ErosionParams, recv: &[usize], order: &[usize]) {
        for &i in order {
            let r = recv[i];
            if r == i {
                continue; // outlet / base level — fixed
            }
            let (x, y) = (i % self.nx, i / self.nx);
            let (rx, ry) = (r % self.nx, r / self.nx);
            let diag = x != rx && y != ry;
            let dist = if diag {
                params.cell_size * std::f32::consts::SQRT_2
            } else {
                params.cell_size
            };
            let factor = params.k * params.dt * self.drainage[i].powf(params.m) / dist;
            self.h[i] = (self.h[i] + factor * self.h[r]) / (1.0 + factor);
        }
    }

    /// Step 5b — **deposition (Davy & Lague 2009).** Routes the sediment this
    /// epoch's incision produced downstream along the single-flow tree and lays it
    /// down at the dimensionally-honest rate `D = G·Qs/A`: `Qs` is the sediment
    /// *volume* the reach carries, `A` its drainage area, so `Qs/A` is a height and
    /// `G` (`params.deposition`) is a pure number. Because deposition scales with
    /// the load already being carried — not an arbitrary capacity — it stays small
    /// in steep upland channels (which keep net-incising) and grows where the
    /// channel slackens and `Qs` has piled up: slack reaches near base level. That
    /// is what grades valley floors and outlets to a smooth descent (no pools)
    /// while leaving the dendritic valley network intact.
    ///
    /// Operator-split from the (stable, implicit) incision: a defensible first
    /// take. The fully-coupled, unconditionally-stable form is the implicit
    /// Davy-Lague solve (Yuan et al. 2019); a clean upgrade when wanted.
    /// Deterministic: fixed downstream order and neighbour geometry.
    fn deposit(&mut self, params: &ErosionParams, recv: &[usize], order: &[usize], before: &[f32]) {
        let nx = self.nx;
        let area = self.cell_size * self.cell_size;
        let g = params.deposition;
        // Sediment volume each cell produced by this epoch's incision.
        let mut qs = vec![0.0f32; nx * nx];
        for i in 0..nx * nx {
            let eroded = before[i] - self.h[i];
            if eroded > 0.0 {
                qs[i] = eroded * area;
            }
        }
        // High → low: each cell has, by the time it is reached, gathered all the
        // flux from its (higher) donors. Deposit `G·Qs/A` as a height, conserve the
        // rest downstream; sediment reaching an outlet is lost to the sea.
        for &i in order.iter().rev() {
            let a = self.drainage[i].max(area); // drainage area, ≥ one cell
            let deposit_h = g * qs[i] / a; // Davy-Lague: a true height (Qs vol / A area)
            let deposit_vol = (deposit_h * area).min(qs[i]); // can't lay down more than carried
            self.h[i] += deposit_vol / area;
            qs[i] -= deposit_vol;
            let r = recv[i];
            if r != i {
                qs[r] += qs[i];
            }
        }
    }

    /// Step 6 — thermal/talus relaxation (Musgrave 1989). Where the drop to the
    /// lowest neighbour exceeds the repose slope, move the excess material down.
    /// Computed from a snapshot and applied as a batch so the result does not
    /// depend on iteration order — the determinism-clean way to do a relaxation.
    fn talus(&mut self, params: &ErosionParams) {
        let nx = self.nx;
        let snapshot = self.h.clone();
        let mut delta = vec![0.0f32; nx * nx];
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                let i = self.idx(x, y);
                // Steepest downhill neighbour in the snapshot.
                let hi = snapshot[i];
                let mut best = i;
                let mut best_drop = 0.0f32;
                let mut best_dist = params.cell_size;
                for (dx, dy) in NEIGHBORS {
                    let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                    let dist = if dx != 0 && dy != 0 {
                        params.cell_size * std::f32::consts::SQRT_2
                    } else {
                        params.cell_size
                    };
                    let drop = hi - snapshot[j];
                    if drop / dist > best_drop / best_dist {
                        best_drop = drop;
                        best_dist = dist;
                        best = j;
                    }
                }
                if best == i {
                    continue;
                }
                let excess = best_drop - params.max_slope * best_dist;
                if excess > 0.0 {
                    // Move half the excess this pass — a stable under-relaxation
                    // that converges over epochs rather than overshooting.
                    let moved = excess * 0.5;
                    delta[i] -= moved;
                    delta[best] += moved;
                }
            }
        }
        for (h, d) in self.h.iter_mut().zip(delta.iter()) {
            *h += *d;
        }
    }

    /// Min / max elevation over the field — for previews and sanity checks.
    pub fn elevation_range(&self) -> (f32, f32) {
        self.h.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &v| {
            (lo.min(v), hi.max(v))
        })
    }
}

/// Spatially-varying rock **erodibility resistance** — material hardness as a
/// deterministic 3D field. The erosion process samples it at the *incising bed
/// elevation*, so as a channel cuts downward it meets successive layers: that is
/// what makes **strata**, **knickpoints/waterfalls** (flow stalls on a hard
/// band), **hard-sill lakes** (a resistant bar dams a valley) and **caprock**
/// *emergent* rather than painted. `1.0` is baseline; higher resists erosion,
/// lower yields. Pure function of `(seed, x, y, z)` — cheap, reproducible.
#[derive(Clone, Copy, Debug)]
pub struct Strata {
    seed: u64,
}

impl Strata {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Hardness at a world point (metres). Depth `z` drives horizontal **strata**
    /// (alternating hard/soft bands); a low-frequency 3D component adds regional
    /// **intrusions / blobs** of harder and softer rock (Joseph's original list).
    /// Clamped to a sane resistance range so nothing is unerodable or vanishes.
    pub fn hardness(&self, x: f32, y: f32, z: f32) -> f32 {
        let p1 = hash_phase(self.seed ^ 0x9E37_79B9);
        let p2 = hash_phase(self.seed ^ 0x2545_F491);
        // Horizontal strata: alternating bands with depth, two scales.
        let bands = 0.55 * (z * 0.03 + p1).sin() + 0.25 * (z * 0.11 + p2).sin();
        // Regional blobs: low-frequency 3D value noise (intrusions, batholiths).
        let f = 0.0009;
        let blob = (value_noise3(x * f, y * f, z * f, self.seed) - 0.5) * 1.2;
        (1.0 + bands + blob).clamp(0.25, 4.0)
    }
}

fn hash_phase(s: u64) -> f32 {
    (hash_u64(s) as f32 / u32::MAX as f32) * std::f32::consts::TAU
}

/// splitmix64 finalizer → 32 bits. Deterministic, well-mixed.
fn hash_u64(x: u64) -> u32 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    ((z ^ (z >> 31)) & 0xFFFF_FFFF) as u32
}

/// Trilinearly-interpolated 3D value noise in `[0, 1)`, deterministic from seed.
fn value_noise3(x: f32, y: f32, z: f32, seed: u64) -> f32 {
    let (xi, yi, zi) = (x.floor() as i64, y.floor() as i64, z.floor() as i64);
    let (sx, sy, sz) = (smooth(x - xi as f32), smooth(y - yi as f32), smooth(z - zi as f32));
    let mut acc = 0.0;
    for (dx, wx) in [(0i64, 1.0 - sx), (1, sx)] {
        for (dy, wy) in [(0i64, 1.0 - sy), (1, sy)] {
            for (dz, wz) in [(0i64, 1.0 - sz), (1, sz)] {
                let h = hash_u64(
                    (xi + dx) as u64
                        ^ ((yi + dy) as u64).wrapping_mul(0x1000_0193)
                        ^ ((zi + dz) as u64).wrapping_mul(0x0100_0000_01B3)
                        ^ seed,
                );
                acc += (h as f32 / u32::MAX as f32) * wx * wy * wz;
            }
        }
    }
    acc
}

#[inline]
fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tether-to-truth property for the abstraction tier: the same seed and
    /// parameters produce a bit-identical landscape. Compared on raw bits so a
    /// single differing ULP fails the test — exactly the guarantee the voxel
    /// world already holds, extended to erosion.
    #[test]
    fn erosion_is_bit_identical() {
        let p = ErosionParams { nx: 64, epochs: 20, ..Default::default() };
        let a = Heightfield::simulate(&p, 0xC0FFEE);
        let b = Heightfield::simulate(&p, 0xC0FFEE);
        assert_eq!(a.h.len(), b.h.len());
        for (x, y) in a.h.iter().zip(b.h.iter()) {
            assert_eq!(x.to_bits(), y.to_bits(), "erosion diverged between runs");
        }
        for (x, y) in a.drainage.iter().zip(b.drainage.iter()) {
            assert_eq!(x.to_bits(), y.to_bits(), "drainage diverged between runs");
        }
    }

    /// Different seeds must give different landscapes — otherwise the seed is not
    /// actually steering anything (a way the determinism test could pass vacuously
    /// on a constant field).
    #[test]
    fn different_seeds_differ() {
        let p = ErosionParams { nx: 64, epochs: 20, ..Default::default() };
        let a = Heightfield::simulate(&p, 1);
        let b = Heightfield::simulate(&p, 2);
        let differ = a.h.iter().zip(b.h.iter()).any(|(x, y)| x.to_bits() != y.to_bits());
        assert!(differ, "two seeds produced the identical landscape");
    }

    /// Erosion must actually concentrate flow: after maturing, the drainage field
    /// should be highly skewed — most cells shed only their own area, a few carry
    /// large rivers. A dendritic network has a heavy upper tail; a non-draining
    /// or uniform field does not. This is a *necessary* condition for "dendritic",
    /// not a proof of it (the eye, via the preview example, is the real check).
    #[test]
    fn drainage_concentrates() {
        let p = ErosionParams { nx: 96, epochs: 50, ..Default::default() };
        let f = Heightfield::simulate(&p, 0xBEEF);
        let cell_area = p.cell_size * p.cell_size;
        let interior = (p.nx - 2) * (p.nx - 2);
        // The single largest river should gather a sizeable fraction of the whole
        // grid's area — i.e. the network has trunk channels, not just sheet flow.
        let max_area = f.drainage.iter().cloned().fold(0.0f32, f32::max);
        let total = interior as f32 * cell_area;
        assert!(
            max_area > 0.10 * total,
            "largest drainage {max_area} is under 10% of total {total}; flow did not concentrate"
        );
    }

    /// Eroding a *supplied* field with sea-level outlets must also be bit-identical
    /// across runs — the path the real-terrain preview takes. (Covers
    /// `from_heights` + `erode` + the `sea_level` outlet branch, which the
    /// blank-slate `simulate` tests do not exercise.)
    #[test]
    fn erode_over_field_with_sea_level_is_bit_identical() {
        let p = ErosionParams {
            nx: 48,
            epochs: 15,
            sea_level: Some(24.0),
            ..Default::default()
        };
        // A reproducible non-trivial starting field (a tilted, bumpy block).
        let make = || {
            let mut h = vec![0.0f32; p.nx * p.nx];
            let mut r = Rng::new(0xABCD);
            for (i, v) in h.iter_mut().enumerate() {
                *v = 20.0 + (i % p.nx) as f32 * 0.3 + r.next_f32() * 4.0;
            }
            Heightfield::from_heights(p.nx, p.cell_size, h).erode(&p)
        };
        let a = make();
        let b = make();
        for (x, y) in a.h.iter().zip(b.h.iter()) {
            assert_eq!(x.to_bits(), y.to_bits(), "erode-over-field diverged");
        }
    }

    /// The talus pass must hold: a deliberately over-steep spike should be cut
    /// back toward the repose slope rather than left standing or amplified.
    #[test]
    fn talus_caps_slope() {
        let p = ErosionParams { nx: 16, max_slope: 1.0, ..Default::default() };
        let mut f = Heightfield::seeded(&p, 7);
        let c = f.idx(8, 8);
        f.h[c] = 50.0; // a one-cell tower, slope ≫ repose
        let before = f.h[c];
        for _ in 0..40 {
            f.talus(&p);
        }
        assert!(f.h[c] < before - 1.0, "talus did not erode the over-steep spike");
    }
}
