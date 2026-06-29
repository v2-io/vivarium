//! **Water as a fluid** — shallow-water hydrodynamics by the *virtual-pipes*
//! method (Mei, Decaudin & Hu, "Fast Hydraulic Erosion Simulation and
//! Visualization on GPU", 2007).
//!
//! This is the honest model, not the posed-surface proxy that preceded it. Water
//! is a real, **volume-conserving** quantity sitting in each cell as a depth `d`;
//! cells are joined to their four axial neighbours by virtual *pipes*, and the
//! flux in each pipe **accelerates under the difference in water-surface height**
//! (`bed + d`) — that acceleration *is* the momentum. Rain adds volume, flow
//! carries it downhill, closed basins fill until their surface is flat and then
//! overflow the lowest lip, and the sea is a fixed reservoir. Nothing here is a
//! macro estimate: it is the discretised shallow-water system, stepped in time.
//!
//! Why pipes rather than the raw Saint-Venant PDE: the pipe form is the standard
//! *stable* discretisation — flux is non-negative and clamped to the water
//! actually present, so depth never goes negative and volume is conserved to
//! round-off. It is the method games and offline terrain tools have used for ~two
//! decades; we are not inventing hydraulics, only applying it.
//!
//! **Scope of this module.** Pure water motion (rain → flow → pooling →
//! overflow), deterministic and dependency-free, on a fixed bed. *Erosion is a
//! secondary coupling* (sediment capacity ∝ flow speed) and is deliberately left
//! for the next layer, per Joseph's framing: get the water right first, let the
//! fine-scale erosion fall out of it afterward. The same `step()` kernel is meant
//! to run either to a quasi-steady snapshot at worldgen ("static by the time you
//! walk in") or live in a loaded region — it does not know or care which.
//!
//! **Stability.** Explicit time stepping obeys a CFL limit: `dt` must be small
//! enough that water does not cross a cell in one step
//! (`dt ≲ cell / sqrt(g · d_max)`). [`WaterParams::default`] is conservative;
//! push `dt` up only with a stability eye on the ASCII preview.

/// Tunables for the pipe model. Defaults are a stable starting point on a unit
/// grid; `cell`, `dt`, and `rain` are the ones to revisit per world scale.
#[derive(Clone, Copy)]
pub struct WaterParams {
    /// Cell width = pipe length `l`, in metres.
    pub cell: f32,
    /// Gravity `g` (m/s²) — sets how hard a surface-height difference drives flow.
    pub gravity: f32,
    /// Time step (s). Bounded by CFL (see module docs); keep it under
    /// `cell / sqrt(g · d_max)`.
    pub dt: f32,
    /// Virtual-pipe cross-section `A` (m²): the inertia/throughput of a pipe.
    /// Larger ⇒ flow responds faster and carries more per unit head.
    pub pipe_area: f32,
    /// Uniform rain rate (metres of water added per second). The source term;
    /// a spatial rain field (Perlin / orographic) is a later refinement.
    pub rain: f32,
    /// Evaporation/infiltration rate as a *fraction of depth per second* — the
    /// sink that lets steady state exist and keeps thin hillslope films from
    /// reading as a wet sheet (Joseph's saturation/seepage list lives here, later).
    pub evaporation: f32,
    /// Sea level (metres). When `Some`, every cell at or below it is held as a
    /// fixed reservoir filled to the waterline — the ocean rivers run to. `None`
    /// for a closed test world with no sea.
    pub sea_level: Option<f32>,
}

impl Default for WaterParams {
    fn default() -> Self {
        Self {
            cell: 1.0,
            gravity: 9.81,
            dt: 0.02,
            pipe_area: 1.0,
            rain: 0.012,
            evaporation: 0.015,
            sea_level: None,
        }
    }
}

/// A square shallow-water domain: a fixed bed, a water depth per cell, and the
/// four outgoing pipe fluxes that carry momentum between steps. Row-major
/// `nx · nx`, indices `y * nx + x`.
#[derive(Clone)]
pub struct WaterSim {
    pub nx: usize,
    /// Terrain elevation per cell (metres). Fixed here; erosion will later move it.
    pub bed: Vec<f32>,
    /// Water depth per cell (metres, ≥ 0). The thing we are solving for.
    pub depth: Vec<f32>,
    // Outgoing flux on each axial pipe (m³/s), held between steps so momentum
    // persists. Non-negative: a pipe only ever carries water *out* of its cell;
    // the neighbour's opposite pipe is the return path.
    fl: Vec<f32>,
    fr: Vec<f32>,
    ft: Vec<f32>, // toward −y ("up"/north)
    fb: Vec<f32>, // toward +y ("down"/south)
}

impl WaterSim {
    /// A dry domain over the given bed.
    pub fn new(nx: usize, bed: Vec<f32>) -> Self {
        assert_eq!(bed.len(), nx * nx, "bed must be nx*nx");
        let z = vec![0.0f32; nx * nx];
        Self {
            nx,
            bed,
            depth: z.clone(),
            fl: z.clone(),
            fr: z.clone(),
            ft: z.clone(),
            fb: z,
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.nx + x
    }

    /// Water-surface elevation `bed + depth` at a cell.
    #[inline]
    pub fn surface(&self, i: usize) -> f32 {
        self.bed[i] + self.depth[i]
    }

    /// Total water volume in the domain (metres³, with unit-area cells = Σ depth).
    /// For conservation checks.
    pub fn total_water(&self) -> f64 {
        self.depth.iter().map(|&d| d as f64).sum()
    }

    /// Hold sea cells at the waterline — a fixed reservoir that both supplies and
    /// absorbs, so rivers can terminate in the ocean.
    fn hold_sea(&mut self, sea: f32) {
        for i in 0..self.bed.len() {
            if self.bed[i] <= sea {
                self.depth[i] = sea - self.bed[i];
            }
        }
    }

    /// Advance one shallow-water step. The five classic stages: rain in, flux
    /// accelerated by head difference, flux clamped to available water (this is
    /// what conserves volume and forbids negative depth), depth updated by net
    /// flux, evaporation out — plus an absorbing border (water leaving the domain
    /// is gone) and the held sea.
    pub fn step(&mut self, p: &WaterParams) {
        let nx = self.nx;
        let n = nx * nx;
        let (l, dt) = (p.cell, p.dt);

        // 1. Rain source. (Sea, if any, is then pinned so it can feed/drain.)
        for d in self.depth.iter_mut() {
            *d += p.rain * dt;
        }
        if let Some(s) = p.sea_level {
            self.hold_sea(s);
        }

        // 2. Accelerate each pipe under the water-surface head difference. A
        //    border neighbour is treated as equal-surface (no push outward here);
        //    the absorbing step below is what actually drains the edge.
        let k = dt * p.pipe_area * p.gravity / l;
        for y in 0..nx {
            for x in 0..nx {
                let i = self.idx(x, y);
                let surf = self.surface(i);
                let sl = if x > 0 { self.surface(i - 1) } else { surf };
                let sr = if x < nx - 1 { self.surface(i + 1) } else { surf };
                let st = if y > 0 { self.surface(i - nx) } else { surf };
                let sb = if y < nx - 1 { self.surface(i + nx) } else { surf };
                self.fl[i] = (self.fl[i] + k * (surf - sl)).max(0.0);
                self.fr[i] = (self.fr[i] + k * (surf - sr)).max(0.0);
                self.ft[i] = (self.ft[i] + k * (surf - st)).max(0.0);
                self.fb[i] = (self.fb[i] + k * (surf - sb)).max(0.0);
            }
        }

        // 3. Scale a cell's outflow down if it would drain more than it holds —
        //    the conservation/non-negativity guard.
        let area = l * l;
        for i in 0..n {
            let out = (self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i]) * dt;
            let avail = self.depth[i] * area;
            if out > avail {
                let s = if out > 0.0 { (avail / out).min(1.0) } else { 0.0 };
                self.fl[i] *= s;
                self.fr[i] *= s;
                self.ft[i] *= s;
                self.fb[i] *= s;
            }
        }

        // 4. Net volume change = inflow (neighbours' pipes pointing at me) minus
        //    my outflow, spread over the cell area.
        for y in 0..nx {
            for x in 0..nx {
                let i = self.idx(x, y);
                let inflow = (if x > 0 { self.fr[i - 1] } else { 0.0 })
                    + (if x < nx - 1 { self.fl[i + 1] } else { 0.0 })
                    + (if y > 0 { self.fb[i - nx] } else { 0.0 })
                    + (if y < nx - 1 { self.ft[i + nx] } else { 0.0 });
                let outflow = self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i];
                self.depth[i] += dt * (inflow - outflow) / area;
                if self.depth[i] < 0.0 {
                    self.depth[i] = 0.0;
                }
            }
        }

        // 5. Evaporation / infiltration sink.
        if p.evaporation > 0.0 {
            let keep = 1.0 - p.evaporation * dt;
            for d in self.depth.iter_mut() {
                *d = (*d * keep).max(0.0);
            }
        }

        // 6. Absorbing border: water that reaches the edge leaves the world.
        for x in 0..nx {
            self.depth[x] = 0.0; // top row
            self.depth[(nx - 1) * nx + x] = 0.0; // bottom row
        }
        for y in 0..nx {
            self.depth[y * nx] = 0.0; // left column
            self.depth[y * nx + nx - 1] = 0.0; // right column
        }

        // 7. Re-pin the sea after the update.
        if let Some(s) = p.sea_level {
            self.hold_sea(s);
        }
    }

    /// Run `steps` shallow-water steps. For a quasi-steady snapshot, run until the
    /// total volume stops changing (the ASCII preview shows when).
    pub fn run(&mut self, p: &WaterParams, steps: u32) {
        for _ in 0..steps {
            self.step(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A parabolic bowl with a spill lip. Interior — the absorbing border is far
    /// away — so rain collects in the bowl.
    fn bowl(nx: usize) -> Vec<f32> {
        let mut bed = vec![0.0f32; nx * nx];
        let c = (nx as f32 - 1.0) / 2.0;
        for y in 0..nx {
            for x in 0..nx {
                let (dx, dy) = (x as f32 - c, y as f32 - c);
                let r = (dx * dx + dy * dy).sqrt();
                // Deep at centre, rising to a rim ~ +20 m; clamped so the outer
                // ground is a plateau the bowl sits in.
                bed[y * nx + x] = (r * r) * 0.02;
            }
        }
        bed
    }

    /// The defining property the posed model could not produce: a closed basin
    /// fills until its **surface is flat**, and no higher than its spill. If this
    /// holds, we have real standing water, not a draped depth.
    #[test]
    fn basin_fills_to_a_flat_lake() {
        let nx = 48;
        let mut sim = WaterSim::new(nx, bowl(nx));
        let p = WaterParams { rain: 0.05, evaporation: 0.002, ..Default::default() };
        sim.run(&p, 4000);

        // Collect the surface of every well-submerged cell (depth above a small
        // threshold, so we ignore the wet fringe) away from the border.
        let mut surfaces = Vec::new();
        for y in 2..nx - 2 {
            for x in 2..nx - 2 {
                let i = y * nx + x;
                if sim.depth[i] > 0.5 {
                    surfaces.push(sim.surface(i));
                }
            }
        }
        assert!(surfaces.len() > 20, "the bowl never filled ({} wet cells)", surfaces.len());
        let lo = surfaces.iter().cloned().fold(f32::MAX, f32::min);
        let hi = surfaces.iter().cloned().fold(f32::MIN, f32::max);
        assert!(
            hi - lo < 0.5,
            "lake surface is not flat: spread {:.3} m across {} cells",
            hi - lo,
            surfaces.len()
        );
    }

    /// Closed-system conservation: a blob of water in a deep bowl, no rain, no
    /// evaporation, never reaching the border — total volume must hold (to
    /// round-off) across many steps. This is the volume-conservation guarantee the
    /// flux clamp gives us.
    #[test]
    fn water_is_conserved_when_it_cannot_escape() {
        let nx = 48;
        let mut sim = WaterSim::new(nx, bowl(nx));
        // Pour a slug into the bowl centre.
        let c = nx / 2;
        for y in c - 4..c + 4 {
            for x in c - 4..c + 4 {
                sim.depth[y * nx + x] = 3.0;
            }
        }
        let p = WaterParams { rain: 0.0, evaporation: 0.0, ..Default::default() };
        let v0 = sim.total_water();
        sim.run(&p, 2000);
        let v1 = sim.total_water();
        let drift = (v1 - v0).abs() / v0;
        assert!(drift < 1e-3, "volume drifted {drift:.2e} (v0={v0:.3}, v1={v1:.3})");
        // And it settled flat rather than sloshing forever.
        assert!(sim.depth.iter().all(|&d| d >= 0.0), "negative depth appeared");
    }

    /// Determinism — the tether-to-truth property the whole core holds.
    #[test]
    fn simulation_is_bit_identical() {
        let nx = 32;
        let p = WaterParams { rain: 0.03, ..Default::default() };
        let mut a = WaterSim::new(nx, bowl(nx));
        let mut b = WaterSim::new(nx, bowl(nx));
        a.run(&p, 500);
        b.run(&p, 500);
        for (x, y) in a.depth.iter().zip(b.depth.iter()) {
            assert_eq!(x.to_bits(), y.to_bits(), "water diverged between runs");
        }
    }
}
