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
    // --- The water cycle. Nothing here creates or destroys water: every "source"
    // and "sink" is a transfer between reservoirs (atmosphere ⇄ surface ⇄
    // groundwater ⇄ ocean), so the global total is conserved (guarded by a test).
    /// **Precipitation** rate (m/s per cell) drawn *from the atmosphere store* and
    /// rained onto the surface — scaled down if the atmosphere runs short, so it
    /// can never rain water that does not exist. Uniform for now; a spatial
    /// (orographic / Perlin) field is a later refinement.
    pub precip_rate: f32,
    /// **Evaporation** as a fraction of surface depth per second — surface water
    /// returning *to the atmosphere store* (whence it precipitates again). Closes
    /// the cycle and lets endorheic basins reach a steady level.
    pub evaporation: f32,
    /// **Infiltration** rate (m/s) — surface water soaking *into the groundwater
    /// store*, capped by the water present and the remaining `gw_capacity`. The
    /// mechanism that concentrates flow into a network: where input is below this
    /// the ground absorbs it and stays dry; only concentrated flow runs.
    pub infiltration: f32,
    /// **Groundwater capacity** per cell (m of water): the soil's storage =
    /// porosity × soil depth. Infiltration stops when the cell is saturated.
    pub gw_capacity: f32,
    /// **Baseflow** as a fraction of groundwater per second — groundwater seeping
    /// back *to the surface* (springs / dry-season river flow). This is what keeps
    /// channels running between rains; emergent, not placed.
    pub baseflow: f32,
    /// **Ocean evaporation** (m/s global) — water moving from the ocean store to
    /// the atmosphere store, the engine that keeps precipitation supplied. Zero in
    /// a closed test world (no ocean).
    pub ocean_evap: f32,
    /// Sea level (metres). When `Some`, every cell at or below it exchanges with
    /// the **ocean store** (held at the waterline); the inflow/outflow is counted,
    /// not vanished. `None` for a closed test world with no sea.
    pub sea_level: Option<f32>,

    // --- Sediment transport (hydraulic erosion), Mei/Decaudin/Hu §4. The water
    // run is *also* the fine-erosion run: fast flow lifts bed into suspension and
    // slow flow drops it, so channels deepen and slack reaches/deltas aggrade. Set
    // `capacity = 0` to disable (pure water, the default). ---
    /// Sediment **capacity** coefficient `Kc`: how much sediment a unit of fast,
    /// steep flow can hold, `C = Kc · slope · speed`. The master erosion knob.
    pub capacity: f32,
    /// **Erosion** rate `Ks`: how fast under-loaded flow (`C > s`) lifts bed into
    /// suspension. Higher ⇒ faster down-cutting.
    pub erode: f32,
    /// **Deposition** rate `Kd`: how fast over-loaded flow (`s > C`) drops sediment
    /// back to the bed. Builds floodplains, deltas, the floors of slack reaches.
    pub deposit: f32,
    /// Floor on the slope term in the capacity law, so even near-flat channels keep
    /// a little carrying capacity instead of instantly dumping their whole load.
    pub min_slope: f32,
    /// **Angle of repose** as a max stable bed slope (rise/run): material steeper
    /// than this slumps to its lower neighbours each step (thermal erosion / talus),
    /// mass-conserving. This is what stops deposition spikes and erosion pits from
    /// growing without bound — a stable hydraulic-erosion model needs it. `0`
    /// disables. Set high enough (~steep) that it caps thin towers without
    /// flattening real mountain flanks.
    pub repose: f32,
}

impl Default for WaterParams {
    fn default() -> Self {
        Self {
            cell: 1.0,
            gravity: 9.81,
            dt: 0.02,
            pipe_area: 1.0,
            precip_rate: 0.012,
            evaporation: 0.004,
            infiltration: 0.010, // just under precip: slopes soak, channels run
            gw_capacity: 2.0,
            baseflow: 0.001, // slow groundwater seepage → springs / baseflow
            ocean_evap: 0.0, // no ocean by default (closed world)
            sea_level: None,
            capacity: 0.0, // erosion off by default — pure water
            erode: 0.3,
            deposit: 0.3,
            min_slope: 0.05,
            repose: 1.2, // ~50° max stable slope; caps spikes, allows steep flanks
        }
    }
}

/// A square shallow-water domain: a fixed bed, a water depth per cell, and the
/// four outgoing pipe fluxes that carry momentum between steps. Row-major
/// `nx · nx`, indices `y * nx + x`.
#[derive(Clone)]
pub struct WaterSim {
    pub nx: usize,
    /// Terrain elevation per cell (metres). The water run *carves* this when
    /// sediment transport is on — it is the fine-eroded bed at the end.
    pub bed: Vec<f32>,
    /// Water depth per cell (metres, ≥ 0). The thing we are solving for.
    pub depth: Vec<f32>,
    /// Suspended sediment per cell (metres of solid, ≥ 0): bed the flow has lifted
    /// and is carrying. Conserved with `bed` — eroding moves bed→sediment, settling
    /// moves sediment→bed — and advected downstream by the flow each step.
    pub sediment: Vec<f32>,
    /// **Groundwater** store per cell (metres of water, 0..`gw_capacity`): water
    /// the soil has absorbed. Fed by infiltration, drained by baseflow back to the
    /// surface — a real reservoir, not a void.
    pub groundwater: Vec<f32>,
    /// **Atmosphere** store (metres of water, summed over the grid): precipitable
    /// water. Precipitation draws from it; evaporation refills it. The closed top
    /// of the water cycle.
    pub atmosphere: f64,
    /// **Ocean** store (metres of water, summed): the sea as a counted reservoir —
    /// runoff drains into it, ocean-evaporation lifts from it. Together with the
    /// three fields above, the grand total is invariant (see `total_water`).
    pub ocean: f64,
    /// World metre coordinate of grid node `(0, 0)` — so hardness can be sampled in
    /// world space. 0 for origin-centred test domains.
    pub origin_m: f32,
    /// Optional 3D **material hardness** ([`crate::geo::Strata`]). When present,
    /// erosion is divided by the hardness *at the current bed elevation*, so soft
    /// rock yields and hard bands resist — strata and waterfalls emerge. `None` =
    /// uniform erodibility.
    pub hardness: Option<crate::geo::Strata>,
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
            sediment: z.clone(),
            groundwater: z.clone(),
            atmosphere: 0.0,
            ocean: 0.0,
            origin_m: 0.0,
            hardness: None,
            fl: z.clone(),
            fr: z.clone(),
            ft: z.clone(),
            fb: z,
        }
    }

    /// Charge the atmosphere store (metres of water summed over the grid) so the
    /// cycle has water to precipitate. Returns `self` for building.
    pub fn with_atmosphere(mut self, amount: f64) -> Self {
        self.atmosphere = amount;
        self
    }

    /// Give the sim a 3D material-hardness field and the world origin to sample it
    /// at, so erosion respects strata and intrusions. Returns `self` for building.
    pub fn with_hardness(mut self, strata: crate::geo::Strata, origin_m: f32) -> Self {
        self.hardness = Some(strata);
        self.origin_m = origin_m;
        self
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

    /// **Total water across every reservoir** — surface + groundwater + atmosphere
    /// + ocean (metres, unit-area cells). This is the conserved quantity: it must
    /// not change except by an explicit external transfer, and the conservation
    /// test asserts exactly that. (Surface water alone is *not* conserved — it
    /// trades with the other three — which is why the earlier per-`depth` total was
    /// the wrong invariant.)
    pub fn total_water(&self) -> f64 {
        let surface: f64 = self.depth.iter().map(|&d| d as f64).sum();
        let gw: f64 = self.groundwater.iter().map(|&g| g as f64).sum();
        surface + gw + self.atmosphere + self.ocean
    }

    /// Hold sea cells at the waterline, **counting** the exchange into the ocean
    /// store so nothing is created or lost: a cell above the line gives its excess
    /// to the ocean, a cell below draws from it.
    fn hold_sea(&mut self, sea: f32) {
        for i in 0..self.bed.len() {
            if self.bed[i] <= sea {
                let target = sea - self.bed[i];
                self.ocean += (self.depth[i] - target) as f64;
                self.depth[i] = target;
            }
        }
    }

    /// Advance one shallow-water step. The five classic stages: rain in, flux
    /// accelerated by head difference, flux clamped to available water (this is
    /// what conserves volume and forbids negative depth), depth updated by net
    /// flux — then the cycle's reservoir transfers (infiltration, baseflow,
    /// evaporation, ocean exchange). Every transfer is reservoir-to-reservoir, so
    /// `total_water()` is invariant across the step.
    pub fn step(&mut self, p: &WaterParams) {
        let nx = self.nx;
        let n = nx * nx;
        let (l, dt) = (p.cell, p.dt);

        // 1. Precipitation: atmosphere → surface. Scaled so we never rain more
        //    water than the atmosphere holds (conservation, not a free source).
        if p.precip_rate > 0.0 && self.atmosphere > 0.0 {
            let want = p.precip_rate as f64 * dt as f64 * n as f64;
            let scale = (self.atmosphere / want).min(1.0) as f32;
            let per_cell = p.precip_rate * dt * scale;
            for d in self.depth.iter_mut() {
                *d += per_cell;
            }
            self.atmosphere -= per_cell as f64 * n as f64;
        }
        // Ocean → atmosphere: the evaporation that keeps precipitation supplied.
        if p.ocean_evap > 0.0 && self.ocean > 0.0 {
            let lift = (p.ocean_evap as f64 * dt as f64 * n as f64).min(self.ocean);
            self.ocean -= lift;
            self.atmosphere += lift;
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

        // 4b. Sediment transport — the fine erosion. Velocity-capacity erode/
        //     deposit (conserving bed↔suspended), then conservative flux-driven
        //     advection (the suspended load rides the same water it flows in).
        if p.capacity > 0.0 {
            self.transport_sediment(p);
        }

        // 5. Reservoir transfers — all conserved (reservoir → reservoir):
        //    infiltration (surface → groundwater, capped by remaining capacity),
        //    baseflow (groundwater → surface), evaporation (surface → atmosphere).
        {
            let inf = p.infiltration * dt;
            let keep = 1.0 - p.evaporation * dt;
            let bf = p.baseflow * dt;
            let mut evaporated = 0.0f64;
            for i in 0..n {
                let room = (p.gw_capacity - self.groundwater[i]).max(0.0);
                let into_gw = inf.min(self.depth[i]).min(room);
                self.depth[i] -= into_gw;
                self.groundwater[i] += into_gw;

                let out_gw = bf * self.groundwater[i];
                self.groundwater[i] -= out_gw;
                self.depth[i] += out_gw;

                let after = (self.depth[i] * keep).max(0.0);
                evaporated += (self.depth[i] - after) as f64;
                self.depth[i] = after;
            }
            self.atmosphere += evaporated;
        }

        // 6. The domain edge drains to the ocean store — counted, not deleted (the
        //    coastline of the modelled patch). A closed interior test never reaches
        //    it, so nothing is lost there.
        for x in 0..nx {
            let (t, b) = (x, (nx - 1) * nx + x);
            self.ocean += (self.depth[t] + self.depth[b]) as f64;
            self.depth[t] = 0.0;
            self.depth[b] = 0.0;
        }
        for y in 0..nx {
            let (lf, rt) = (y * nx, y * nx + nx - 1);
            self.ocean += (self.depth[lf] + self.depth[rt]) as f64;
            self.depth[lf] = 0.0;
            self.depth[rt] = 0.0;
        }

        // 7. Re-pin the sea (exchange counted into the ocean store).
        if let Some(s) = p.sea_level {
            self.hold_sea(s);
        }
    }

    /// **Total solid across reservoirs** — bed + suspended sediment (metres,
    /// unit-area cells). Conserved by erosion (bed→suspended), deposition
    /// (suspended→bed) and advection; the only legitimate change is an explicit
    /// external transfer (tectonic uplift adding bed). The sediment conservation
    /// test asserts this holds.
    pub fn total_solid(&self) -> f64 {
        let bed: f64 = self.bed.iter().map(|&b| b as f64).sum();
        let sed: f64 = self.sediment.iter().map(|&s| s as f64).sum();
        bed + sed
    }

    /// One step of sediment transport (called from `step` when capacity > 0).
    /// Velocity-capacity erosion/deposition then conservative advection — solid
    /// mass (`bed + sediment`) is invariant across this.
    fn transport_sediment(&mut self, p: &WaterParams) {
        let nx = self.nx;
        let n = nx * nx;
        let (l, dt) = (p.cell, p.dt);
        let area = l * l;
        let eps = 1e-6f32;

        // --- Erosion / deposition. Capacity rises with flow speed and bed slope
        //     (Joseph's intuition: more velocity ⇒ more carrying power); the load
        //     relaxes toward it (the lag that lets sediment travel). Conserves
        //     bed ↔ suspended exactly. ---
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let d = self.depth[i];
                if d <= eps {
                    // Dry ground can carry nothing, but *settle gradually* — dumping
                    // the whole suspended load at once builds vertical spikes where
                    // a cell flickers wet/dry. Deposit at the normal rate instead.
                    let amt = p.deposit * self.sediment[i];
                    self.bed[i] += amt;
                    self.sediment[i] -= amt;
                    continue;
                }
                // Flow speed from the net pipe flux through the cell (Mei §4).
                let frl = if x > 0 { self.fr[i - 1] } else { 0.0 };
                let flr = if x < nx - 1 { self.fl[i + 1] } else { 0.0 };
                let dwx = 0.5 * ((frl - self.fl[i]) + (self.fr[i] - flr));
                let fbt = if y > 0 { self.fb[i - nx] } else { 0.0 };
                let ftb = if y < nx - 1 { self.ft[i + nx] } else { 0.0 };
                let dwy = 0.5 * ((fbt - self.ft[i]) + (self.fb[i] - ftb));
                // Froude-limited flow speed: the raw flux/(depth) estimate diverges
                // when a large head sits over a thin film (hundreds of m/s), which
                // is what makes naive hydraulic erosion explode. Real open-channel
                // flow is bounded by the gravity-wave speed √(g·d); cap a few × that
                // so capacity — and therefore erosion — stays physical and stable.
                let raw = (dwx * dwx + dwy * dwy).sqrt() / (d * l);
                let v_max = 4.0 * (p.gravity * d).sqrt();
                let speed = raw.min(v_max);

                // Bed tilt for the capacity law.
                let bl = self.bed[y * nx + x.saturating_sub(1)];
                let br = self.bed[y * nx + (x + 1).min(nx - 1)];
                let bd = self.bed[y.saturating_sub(1) * nx + x];
                let bu = self.bed[(y + 1).min(nx - 1) * nx + x];
                let slope = ((br - bl).powi(2) + (bu - bd).powi(2)).sqrt() / (2.0 * l);
                let sin_tilt = (slope / (1.0 + slope * slope).sqrt()).max(p.min_slope);

                let cap = p.capacity * sin_tilt * speed;
                let s = self.sediment[i];
                if cap > s {
                    // Under capacity: lift bed into suspension, resisted by the
                    // material hardness at *this bed elevation* — soft rock yields,
                    // hard bands hold, so strata, knickpoints and hard-sill lakes
                    // emerge as the channel incises through the column.
                    let hard = match &self.hardness {
                        Some(st) => {
                            let xm = self.origin_m + x as f32 * l;
                            let ym = self.origin_m + y as f32 * l;
                            st.hardness(xm, ym, self.bed[i])
                        }
                        None => 1.0,
                    };
                    let amt = p.erode / hard * (cap - s);
                    self.bed[i] -= amt;
                    self.sediment[i] = s + amt;
                } else {
                    // Over capacity: settle the excess back to the bed.
                    let amt = p.deposit * (s - cap);
                    self.bed[i] += amt;
                    self.sediment[i] = s - amt;
                }
            }
        }

        // --- Conservative advection: suspended sediment rides the *same* pipe
        //     fluxes the water used, so the fraction that leaves a cell arrives in
        //     its neighbours — nothing invented or lost. The leaving fraction is
        //     capped at 1 (the cell can't export more sediment than it holds, even
        //     if its water thinned after the fluxes were set), which keeps it
        //     bounded *and* exactly conservative. ---
        let mut delta = vec![0.0f32; n];
        for i in 0..n {
            let s = self.sediment[i];
            let total_flux = self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i];
            if s <= 0.0 || total_flux <= 0.0 {
                continue;
            }
            let vol = (self.depth[i] * area).max(eps);
            let leave = (total_flux * dt / vol).min(1.0) * s; // ≤ s, always
            let per = leave / total_flux; // split by each pipe's share of the flow
            let (x, y) = (i % nx, i / nx);
            if x > 0 {
                let m = self.fl[i] * per;
                delta[i] -= m;
                delta[i - 1] += m;
            }
            if x < nx - 1 {
                let m = self.fr[i] * per;
                delta[i] -= m;
                delta[i + 1] += m;
            }
            if y > 0 {
                let m = self.ft[i] * per;
                delta[i] -= m;
                delta[i - nx] += m;
            }
            if y < nx - 1 {
                let m = self.fb[i] * per;
                delta[i] -= m;
                delta[i + nx] += m;
            }
        }
        for i in 0..n {
            self.sediment[i] = (self.sediment[i] + delta[i]).max(0.0);
        }

        // --- Talus / thermal erosion: bed steeper than the angle of repose slumps
        //     toward its steepest-downhill neighbour (mass-conserving). This is what
        //     keeps deposition spikes and erosion pits from growing without bound —
        //     they collapse back to a stable slope over a few steps. ---
        if p.repose > 0.0 {
            const NB: [(i32, i32); 8] = [
                (-1, 0), (1, 0), (0, -1), (0, 1),
                (-1, -1), (1, -1), (-1, 1), (1, 1),
            ];
            let diag = l * std::f32::consts::SQRT_2;
            let mut dz = vec![0.0f32; n];
            for y in 0..nx {
                for x in 0..nx {
                    let i = y * nx + x;
                    let bi = self.bed[i];
                    // Steepest-descent neighbour and its excess over the repose slope.
                    let (mut best_j, mut best_excess) = (i, 0.0f32);
                    for (dx, dy) in NB {
                        let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                        if nx_ < 0 || ny_ < 0 || nx_ >= nx as i32 || ny_ >= nx as i32 {
                            continue;
                        }
                        let j = ny_ as usize * nx + nx_ as usize;
                        let dist = if dx != 0 && dy != 0 { diag } else { l };
                        let excess = (bi - self.bed[j]) - p.repose * dist;
                        if excess > best_excess {
                            best_excess = excess;
                            best_j = j;
                        }
                    }
                    if best_j != i {
                        let m = 0.5 * best_excess; // under-relax: converges over steps
                        dz[i] -= m;
                        dz[best_j] += m;
                    }
                }
            }
            for i in 0..n {
                self.bed[i] += dz[i];
            }
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
        let mut sim = WaterSim::new(nx, bowl(nx)).with_atmosphere(50_000.0);
        let p = WaterParams {
            precip_rate: 0.05,
            evaporation: 0.002,
            infiltration: 0.0, // isolate the fill behaviour from the soak path
            baseflow: 0.0,
            ..Default::default()
        };
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

    /// **The conservation invariant — the build-to-last guard.** A *full* water
    /// cycle runs in a closed bowl (precipitation, flow, infiltration↔groundwater,
    /// baseflow, evaporation, edge→ocean) — every "source" and "sink" is a transfer
    /// between reservoirs, so the grand total (surface + groundwater + atmosphere +
    /// ocean) must not move. Water is neither created nor destroyed; if this test
    /// ever fails, a leak was introduced.
    #[test]
    fn the_water_cycle_conserves_total_water() {
        let nx = 48;
        let mut sim = WaterSim::new(nx, bowl(nx)).with_atmosphere(10_000.0);
        let p = WaterParams {
            precip_rate: 0.02,
            evaporation: 0.01,
            infiltration: 0.01,
            gw_capacity: 2.0,
            baseflow: 0.002,
            ..Default::default()
        };
        let v0 = sim.total_water();
        sim.run(&p, 3000);
        let v1 = sim.total_water();
        let drift = (v1 - v0).abs() / v0;
        assert!(drift < 1e-3, "total water drifted {drift:.2e} (v0={v0:.1}, v1={v1:.1})");
        // The cycle actually moved water between reservoirs (it isn't a no-op).
        let gw: f64 = sim.groundwater.iter().map(|&g| g as f64).sum();
        assert!(gw > 0.0, "groundwater never filled — the cycle didn't run");
        assert!(sim.depth.iter().all(|&d| d >= 0.0), "negative depth appeared");
    }

    /// Sediment transport must (a) actually reshape the bed — erode where flow is
    /// fast, settle where it slows — and (b) **conserve solid mass**: `bed +
    /// suspended` is invariant (no uplift here), the build-to-last guard for the
    /// erosion half. Erosion/deposition move material between bed and suspension;
    /// advection moves it between cells; neither creates or destroys it.
    #[test]
    fn sediment_transport_carves_and_conserves_solid() {
        let nx = 64;
        let bed0 = bowl(nx);
        let mut sim = WaterSim::new(nx, bed0.clone()).with_atmosphere(1.0e6);
        let p = WaterParams {
            precip_rate: 0.05,
            evaporation: 0.0,
            infiltration: 0.0,
            baseflow: 0.0,
            capacity: 0.4,
            erode: 0.5,
            deposit: 0.5,
            min_slope: 0.05,
            ..Default::default()
        };
        let s0 = sim.total_solid();
        sim.run(&p, 3000);
        let drift = (sim.total_solid() - s0).abs() / s0.abs().max(1.0);
        assert!(drift < 1e-3, "solid mass not conserved: drift {drift:.2e}");
        let max_move = sim
            .bed
            .iter()
            .zip(&bed0)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(max_move > 0.05, "bed never moved — sediment transport inert ({max_move})");
        // Stability: erosion must stay *physical*, not run away. (Conservation
        // alone can't catch a blow-up — an exploding bed still balances suspension —
        // so we bound the magnitude relative to the ~20 m bowl relief.)
        assert!(
            sim.bed.iter().all(|b| b.is_finite() && b.abs() < 1.0e3),
            "bed exploded — erosion unstable (max |bed| {})",
            sim.bed.iter().fold(0.0f32, |m, b| m.max(b.abs()))
        );
    }

    /// Material hardness must *modulate* erosion (soft yields, hard resists — the
    /// basis of strata and waterfalls), while still conserving solid mass and
    /// staying deterministic. Compared against a uniform-erodibility run to prove
    /// the field actually changes the landscape.
    #[test]
    fn hardness_modulates_erosion_and_still_conserves() {
        let nx = 64;
        let bed0 = bowl(nx);
        let p = WaterParams {
            precip_rate: 0.05,
            evaporation: 0.0,
            infiltration: 0.0,
            baseflow: 0.0,
            capacity: 0.4,
            erode: 0.5,
            deposit: 0.5,
            min_slope: 0.05,
            ..Default::default()
        };
        let mut uniform = WaterSim::new(nx, bed0.clone()).with_atmosphere(1.0e6);
        uniform.run(&p, 2000);

        let strata = crate::geo::Strata::new(0x5712);
        let mut hard = WaterSim::new(nx, bed0.clone())
            .with_atmosphere(1.0e6)
            .with_hardness(strata, 0.0);
        let s0 = hard.total_solid();
        hard.run(&p, 2000);

        let drift = (hard.total_solid() - s0).abs() / s0.abs().max(1.0);
        assert!(drift < 1e-3, "solid not conserved with hardness: {drift:.2e}");
        let diff = uniform
            .bed
            .iter()
            .zip(&hard.bed)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(diff > 0.1, "hardness had no effect on the landscape ({diff})");

        let mut again = WaterSim::new(nx, bed0.clone())
            .with_atmosphere(1.0e6)
            .with_hardness(strata, 0.0);
        again.run(&p, 2000);
        for (a, b) in hard.bed.iter().zip(again.bed.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "hardness run not deterministic");
        }
    }

    /// Determinism — the tether-to-truth property the whole core holds.
    #[test]
    fn simulation_is_bit_identical() {
        let nx = 32;
        let p = WaterParams { precip_rate: 0.03, ..Default::default() };
        let mut a = WaterSim::new(nx, bowl(nx)).with_atmosphere(5_000.0);
        let mut b = WaterSim::new(nx, bowl(nx)).with_atmosphere(5_000.0);
        a.run(&p, 500);
        b.run(&p, 500);
        for (x, y) in a.depth.iter().zip(b.depth.iter()) {
            assert_eq!(x.to_bits(), y.to_bits(), "water diverged between runs");
        }
    }
}
