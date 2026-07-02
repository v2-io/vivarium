//! **Water as a fluid** — the virtual-pipes shallow-water kernel (Mei, Decaudin
//! & Hu 2007), ported frame-native from `vivarium-core::hydro` (the proven
//! reference; see its module docs for the full reasoning). Water is a
//! volume-conserving depth `d` per cell; axial pipes carry flux that
//! *accelerates* under the water-surface head difference (`bed + d`) — that
//! acceleration is the momentum. Rain draws from a counted atmosphere store, the
//! sea is a counted reservoir, and every transfer is reservoir-to-reservoir so
//! [`WaterSim::total_water`] is invariant to round-off (guarded by a test).
//!
//! **This is the FAST band of the multirate coupling** (`ref/erosion-port/
//! NOTES.md`, DESIGN-REDUX §4): water sees the terrain as quasi-static (its
//! `bed` is refreshed from the live erosion tier between bursts), and erosion
//! keeps running — the thing core had to switch off, now expressed as a coupling
//! schedule instead of a kill-switch. v1 scope: rain → flow → pooling → sea, no
//! sediment. The sediment coupling comes NEXT and runs the other way through the
//! coupler (time-averaged discharge feeding the erosion tier's `A`), which is
//! what will make oxbows and lake-filling honest rather than arbitrary.
//!
//! **Stability:** explicit stepping under a CFL-ish limit — keep
//! `dt ≲ cell/√(g·d_max)`; the flux clamp (never ship more water than a cell
//! holds) is what forbids negative depth and conserves volume regardless.

use crate::sphere::{CellId, Face};

/// Tunables. Defaults sized for a ~4.8 m grid (L21). The rain rate carries the
/// KNOWN, DELIBERATE fudge inherited from core (~100–1000× real precipitation)
/// so basins fill in minutes of sim time rather than weeks — documented, not
/// hidden (`ref/hydrology/NOTES.md`).
#[derive(Clone, Copy, Debug)]
pub struct WaterParams {
    pub gravity: f32,
    /// Time step (s). CFL: `≲ cell/√(g·d_max)` — 0.2 s is safe to ~5 m depths
    /// on 4.8 m cells.
    pub dt: f32,
    /// Manning roughness n (s/m^⅓): real channel friction (≈0.03 smooth earth,
    /// 0.04–0.05 natural channels, 0.07 rocky). Sets the velocity equilibrium
    /// v = R^⅔·√S/n — replaces the old unphysical per-step damping knob. The
    /// pipe cross-section is likewise physical now: depth × cell width, so thin
    /// films are naturally sluggish and deep channels responsive.
    pub manning_n: f32,
    /// Precipitation (m/s per cell) drawn from the atmosphere store. ~1000× real
    /// climate-average. NOTE the budget: at this default, precip < infiltration +
    /// evaporation — drizzle soaks in and evaporates, NO surface water persists.
    /// That is real climatology (most rain never runs off); rivers come from
    /// STORM bursts well above average (the app's rain multiplier / storm cycle).
    pub precip: f32,
    /// Surface evaporation, m/s flat rate (physical form; capped by the water
    /// present). Real ≈2e-8; carries the same ~1000× cycle fudge as the rain.
    pub evaporation: f32,
    /// Ocean evaporation (m/s per cell equivalent) — the return path that CLOSES
    /// the cycle (ocean → atmosphere → rain again); without it the sky empties
    /// once and weather ends.
    pub ocean_evap: f32,
    /// Sea level (m, bed datum): cells with bed at/below it are held at the
    /// waterline, exchanging with the counted ocean store.
    pub sea_m: f32,
    // --- Sediment (Mei et al. stage; core had this OFF — the multirate frame is
    // where it can be ON). Capacity ∝ flow speed; erode toward capacity, settle
    // above it, advect with the flow. bed + suspended is exactly conserved.
    /// Carrying capacity coefficient: `C = k · |v| · slope` (Mei's tilt term).
    /// Slope in the capacity is what ignites CHANNEL INCEPTION (steep threads
    /// carve, capture flow, carve deeper) while flats and lake floors deposit
    /// (slope→0 ⇒ capacity→0 — lakes fill toward meadows for free).
    pub sed_capacity: f32,
    /// Erosion rate toward capacity (fraction of deficit per second).
    pub sed_erode: f32,
    /// Settling rate above capacity (fraction of excess per second).
    pub sed_deposit: f32,
    /// Max bed change RATE (m per sim-second) — a hard sanity bound. (Was
    /// per-step: at 100x rain that allowed ~90 m/sim-hour of planing — Joseph's
    /// staircase world.)
    pub sed_max_rate: f32,
    /// Critical shear stress τ_c (Pa): bed erosion happens where the flow's
    /// shear τ = ρ·g·d·S exceeds this — THE physical incision threshold
    /// (DESIGN-MATERIAL §15's per-material `incision_threshold`; uniform loose-
    /// sediment value here until materials couple in). Replaces two earlier
    /// ad-hoc gates (min-depth, min-discharge): films have τ≈0 and don't strip;
    /// thin-but-steep mouth reaches have high τ and carve their estuaries.
    pub tau_c: f32,
    // --- Groundwater (core's subsystem, ported): infiltration surface→soil,
    // capped by a per-cell capacity; baseflow returns it to the surface. The two
    // streambed phenomena (Joseph): COLMATION — flowing channels seal, so
    // infiltration falls where discharge is high (computed LIVE from the flow,
    // improving on core's static drainage-baked field) — and ARMORING/bedrock,
    // which per-material permeability (§15) will supply when materials couple in.
    /// Base infiltration rate (m/s) — porous soil drinking surface water.
    pub infiltration: f32,
    /// Groundwater capacity per cell (m of water the soil can hold).
    pub gw_capacity: f32,
    /// Baseflow: fraction of groundwater returning to the surface per second.
    pub baseflow: f32,
    /// Discharge (m²/s) at which channel sealing halves infiltration.
    pub seal_q: f32,
    /// Fines needed to fully plug the bed's pores (m of deposited sediment).
    /// A few mm of silt seals a streambed (real colmation depths are mm–cm);
    /// scour re-opens it at the same exchange rate.
    pub plug_depth: f32,
    /// ARMORING (Joseph's fluvial list): scour into the PARENT bed (no loose
    /// alluvium left) winnows away fines and leaves a coarse surface lag that
    /// shields the bed. `armor_depth` is the scour needed to develop a full
    /// lag (~a few grain layers, m); `armor_shield` is how much a full lag
    /// suppresses further incision (0..1). Fresh deposition buries the lag
    /// and resets it. The standard active-layer concept, crude rung.
    pub armor_depth: f32,
    pub armor_shield: f32,
}

impl Default for WaterParams {
    fn default() -> Self {
        Self {
            gravity: 9.8,
            dt: 0.2,
            manning_n: 0.04,
            precip: 3.0e-5,
            evaporation: 2.0e-5,
            ocean_evap: 1.0e-5,
            sea_m: crate::gen::SEA_LEVEL_M as f32,
            sed_capacity: 0.6,
            sed_erode: 0.1,
            sed_deposit: 0.5,
            sed_max_rate: 0.002,
            tau_c: 1.0,
            infiltration: 2.0e-5,
            gw_capacity: 0.3,
            baseflow: 2.0e-5,
            seal_q: 0.01,
            plug_depth: 0.005,
            armor_depth: 0.1,
            armor_shield: 0.8,
        }
    }
}

/// The pipe-model state over one face region (same footprint idiom as
/// [`crate::erosion::Fluvial`]). Bed is OWNED here and refreshed from the
/// erosion tier by the caller — the §4 quasi-static coupling point.
pub struct WaterSim {
    pub nx: usize,
    pub cell_m: f32,
    pub face: Face,
    pub level: u8,
    pub origin: (u32, u32),
    pub bed: Vec<f32>,
    pub depth: Vec<f32>,
    /// Suspended sediment (m of solid per cell). Conserved with `bed`.
    pub sediment: Vec<f32>,
    /// Groundwater store per cell (m of water, 0..gw_capacity). In the total.
    pub groundwater: Vec<f32>,
    /// Loose settled sediment (alluvium) resting on the bed, m — bookkeeping
    /// over `bed` (which already carries the mass): deposits add, scour removes
    /// it first. It is what makes a bed read sandy.
    pub sed_bed: Vec<f32>,
    /// Colmation 0..1: fraction of bed pores plugged by fines. PERSISTENT —
    /// a sealed bed stays sealed between storms (why dry riverbeds refill
    /// fast) until a flood's scour re-opens it. Joseph's "fine particles
    /// cutting off absorbancy", as state rather than an instantaneous proxy.
    pub colmation: Vec<f32>,
    /// Armor 0..1: coarse surface lag from winnowing (see `armor_depth`).
    /// Grows as scour eats PARENT bed, capped incision; buried by deposition.
    pub armor: Vec<f32>,
    /// Counted reservoirs (m of water, cell-area units): conservation partners.
    pub atmosphere: f64,
    pub ocean: f64,
    /// Froude stats captured INSIDE the last step, at the instant and against
    /// the same sill depths the breaking cap used (post-step recomputation
    /// against drained depths read as Fr 10+ on capped flow — twice).
    last_froude: (f32, f32),
    // Outgoing flux per axial pipe (m³/s), kept between steps: momentum.
    fl: Vec<f32>,
    fr: Vec<f32>,
    ft: Vec<f32>,
    fb: Vec<f32>,
}

impl WaterSim {
    /// A domain over `bed` with `atmosphere_m` of rainable water per cell. The
    /// ocean is NOT a special case (Joseph): the basin below the waterline starts
    /// as real, simulated water — currents, sediment, deltas all live there; only
    /// the REGION EDGE is a boundary condition (the rest-of-world ocean).
    pub fn new(face: Face, level: u8, origin: (u32, u32), nx: usize, cell_m: f32, bed: Vec<f32>, atmosphere_m: f64) -> Self {
        assert_eq!(bed.len(), nx * nx);
        let z = vec![0.0f32; nx * nx];
        let sea = crate::gen::SEA_LEVEL_M as f32;
        let depth: Vec<f32> = bed.iter().map(|&b| (sea - b).max(0.0)).collect();
        Self {
            nx,
            cell_m,
            face,
            level,
            origin,
            bed,
            depth,
            sediment: z.clone(),
            groundwater: z.clone(),
            sed_bed: z.clone(),
            colmation: z.clone(),
            armor: z.clone(),
            atmosphere: atmosphere_m * (nx * nx) as f64,
            ocean: 0.0,
            last_froude: (0.0, 0.0),
            fl: z.clone(),
            fr: z.clone(),
            ft: z.clone(),
            fb: z,
        }
    }

    /// The conserved total: surface + groundwater + atmosphere + ocean.
    pub fn total_water(&self) -> f64 {
        self.depth.iter().map(|&d| d as f64).sum::<f64>()
            + self.groundwater.iter().map(|&g| g as f64).sum::<f64>()
            + self.atmosphere
            + self.ocean
    }

    /// The conserved SOLID total: bed + suspended (m, cell-area units).
    pub fn total_solid(&self) -> f64 {
        self.bed.iter().map(|&b| b as f64).sum::<f64>() + self.sediment.iter().map(|&s| s as f64).sum::<f64>()
    }

    /// Refresh the bed from the (still-eroding) terrain tier — the quasi-static
    /// coupling. Water simply finds itself on the new surface; its volume is
    /// untouched, so conservation holds across the swap.
    pub fn set_bed(&mut self, bed: Vec<f32>) {
        assert_eq!(bed.len(), self.nx * self.nx);
        self.bed = bed;
    }

    /// One shallow-water step — the five classic stages, every transfer counted.
    pub fn step(&mut self, p: &WaterParams) {
        let nx = self.nx;
        let n = nx * nx;
        let (l, dt) = (self.cell_m, p.dt);
        let area = l * l;

        // 1. Rain: atmosphere → surface, scaled so it never rains water that
        //    does not exist.
        if p.precip > 0.0 && self.atmosphere > 0.0 {
            let want = p.precip as f64 * dt as f64 * n as f64;
            let scale = (self.atmosphere / want).min(1.0) as f32;
            let per_cell = p.precip * dt * scale;
            for d in self.depth.iter_mut() {
                *d += per_cell;
            }
            self.atmosphere -= per_cell as f64 * n as f64;
        }

        // 1b. Ocean → atmosphere: the evaporation that keeps rain supplied.
        if p.ocean_evap > 0.0 && self.ocean > 0.0 {
            let lift = (p.ocean_evap as f64 * dt as f64 * n as f64).min(self.ocean);
            self.ocean -= lift;
            self.atmosphere += lift;
        }

        // 2. Boundary: only EDGE cells at/below the waterline are held (the
        //    rest-of-world ocean); interior seabed is ordinary simulated ground.
        self.hold_edge_sea(p.sea_m);

        // 3. Accelerate pipes under head differences (Saint-Venant momentum: the
        //    flow cross-section is depth × width, so df = dt·g·d̄·Δh), then apply
        //    Manning friction implicitly: f ← f / (1 + dt·g·n²·|v| / d̄^{4/3}).
        //    Border neighbours are treated as equal-surface (the edge hold drains).
        // θ flux smoothing (de Almeida & Bates 2013): each pipe blends with its
        // along-axis neighbours — the stand-in for the neglected momentum-
        // advection term. Without it, a local-inertial scheme on steep slopes
        // organises the flux field into travelling solitons decoupled from the
        // depth field (Joseph's multi-metre blobs winding down channels, shape
        // intact — probe-confirmed: Fr max 226, 75% cell-to-cell depth swings).
        // In-place (Gauss–Seidel) sweep: slightly asymmetric, faster-damping.
        let theta = 0.8f32;
        for y in 0..nx {
            for x in 1..nx - 1 {
                let i = y * nx + x;
                self.fl[i] = theta * self.fl[i] + (1.0 - theta) * 0.5 * (self.fl[i - 1] + self.fl[i + 1]);
                self.fr[i] = theta * self.fr[i] + (1.0 - theta) * 0.5 * (self.fr[i - 1] + self.fr[i + 1]);
            }
        }
        for y in 1..nx - 1 {
            for x in 0..nx {
                let i = y * nx + x;
                self.ft[i] = theta * self.ft[i] + (1.0 - theta) * 0.5 * (self.ft[i - nx] + self.ft[i + nx]);
                self.fb[i] = theta * self.fb[i] + (1.0 - theta) * 0.5 * (self.fb[i - nx] + self.fb[i + nx]);
            }
        }
        // Conveyance depth is the FLOW DEPTH OVER THE SILL between the cells —
        // max(surface) − max(bed) — not the average of the two depths. The
        // average lets a deep cell shove mass over a rim its own surface does
        // not clear ("gravity not leveling the blob out"), and computes friction
        // on a depth the constriction doesn't have. Standard in the
        // local-inertial literature (h_flow).
        #[inline]
        fn pipe_step(f: f32, eta_i: f32, eta_j: f32, b_i: f32, b_j: f32, dt: f32, g: f32, n2g: f32, l: f32) -> f32 {
            let hflow = eta_i.max(eta_j) - b_i.max(b_j);
            if hflow < 1e-4 {
                return 0.0; // no conveyance over the sill
            }
            let accel = (f + dt * g * hflow * (eta_i - eta_j)).max(0.0);
            let v = accel / (hflow * l);
            let f = accel / (1.0 + dt * n2g * v / hflow.powf(4.0 / 3.0));
            // Breaking limit: natural steep streams self-organise to Fr ≈ 1
            // (Grant 1997) — surge fronts that outrun ~2× critical BREAK and
            // shed momentum as turbulence. Without this loss the roll waves a
            // deluge excites grow unbounded (Joseph's multi-metre travelling
            // blobs; probe: near-dry gaps between 3 m lumps).
            f.min(2.0 * (g * hflow).sqrt() * hflow * l)
        }
        let n2g = p.gravity * p.manning_n * p.manning_n;
        let (mut fr_max, mut fr_wet, mut fr_sup) = (0.0f32, 0u32, 0u32);
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let (bi, eta_i) = (self.bed[i], self.bed[i] + self.depth[i]);
                let mut cell_fr = 0.0f32;
                let mut probe = |f: f32, eta_j: f32, b_j: f32| {
                    let hflow = eta_i.max(eta_j) - bi.max(b_j);
                    if hflow > 1e-3 && f > 0.0 {
                        cell_fr = cell_fr.max(f / (hflow * l) / (p.gravity * hflow).sqrt());
                    }
                };
                if x > 0 {
                    let j = i - 1;
                    self.fl[i] = pipe_step(self.fl[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n2g, l);
                    probe(self.fl[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                if x < nx - 1 {
                    let j = i + 1;
                    self.fr[i] = pipe_step(self.fr[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n2g, l);
                    probe(self.fr[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                if y > 0 {
                    let j = i - nx;
                    self.ft[i] = pipe_step(self.ft[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n2g, l);
                    probe(self.ft[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                if y < nx - 1 {
                    let j = i + nx;
                    self.fb[i] = pipe_step(self.fb[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n2g, l);
                    probe(self.fb[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                drop(probe);
                if self.depth[i] >= 0.05 {
                    fr_wet += 1;
                    fr_max = fr_max.max(cell_fr);
                    if cell_fr > 1.5 {
                        fr_sup += 1;
                    }
                }
                // Clamp: a cell cannot ship more water than it holds. THIS is
                // the conservation/stability guarantee of the pipe method.
                let out = (self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i]) * dt;
                if out > self.depth[i] * area {
                    let scale = self.depth[i] * area / out;
                    self.fl[i] *= scale;
                    self.fr[i] *= scale;
                    self.ft[i] *= scale;
                    self.fb[i] *= scale;
                }
            }
        }
        self.last_froude = (fr_max, if fr_wet > 0 { fr_sup as f32 / fr_wet as f32 } else { 0.0 });

        // 4. Depth update from net flux.
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let inflow = (if x > 0 { self.fr[i - 1] } else { 0.0 })
                    + (if x < nx - 1 { self.fl[i + 1] } else { 0.0 })
                    + (if y > 0 { self.fb[i - nx] } else { 0.0 })
                    + (if y < nx - 1 { self.ft[i + nx] } else { 0.0 });
                let outflow = self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i];
                self.depth[i] = (self.depth[i] + (inflow - outflow) * dt / area).max(0.0);
            }
        }

        // 4b. SEDIMENT: capacity ∝ |v| (velocity from pipe throughput); erode
        //     the bed toward capacity, settle above it, then advect the load
        //     with the same fractional outflow as the water (mass-conserving).
        if p.sed_capacity > 0.0 {
            for y in 0..nx {
                for x in 0..nx {
                    let i = y * nx + x;
                    let d = self.depth[i];
                    let max_step = p.sed_max_rate * dt;
                    if d < 1e-3 {
                        // No meaningful flow: everything settles.
                        let dp = self.sediment[i].min(max_step);
                        self.bed[i] += dp;
                        self.sediment[i] -= dp;
                        self.sed_bed[i] += dp;
                        self.colmation[i] = (self.colmation[i] + dp / p.plug_depth).min(1.0);
                        continue;
                    }
                    let vx = ((if x > 0 { self.fr[i - 1] } else { 0.0 }) + self.fr[i]
                        - (if x < nx - 1 { self.fl[i + 1] } else { 0.0 })
                        - self.fl[i])
                        * 0.5
                        / (l * d);
                    let vy = ((if y > 0 { self.fb[i - nx] } else { 0.0 }) + self.fb[i]
                        - (if y < nx - 1 { self.ft[i + nx] } else { 0.0 })
                        - self.ft[i])
                        * 0.5
                        / (l * d);
                    let speed = (vx * vx + vy * vy).sqrt();
                    // Local downhill bed slope (steepest of the 4 axial drops).
                    let mut slope = 0.0f32;
                    if x > 0 { slope = slope.max((self.bed[i] - self.bed[i - 1]) / l); }
                    if x < nx - 1 { slope = slope.max((self.bed[i] - self.bed[i + 1]) / l); }
                    if y > 0 { slope = slope.max((self.bed[i] - self.bed[i - nx]) / l); }
                    if y < nx - 1 { slope = slope.max((self.bed[i] - self.bed[i + nx]) / l); }
                    let capacity = (p.sed_capacity * speed * slope.clamp(0.0, 1.0)).min(2.0);
                    let s0 = self.sediment[i];
                    // Bed shear τ = ρ·g·d·S (water density 1000 kg/m³).
                    let tau = 1000.0 * 9.8 * d * slope;
                    if s0 < capacity && tau > p.tau_c {
                        // Armor shields the PARENT bed only — loose alluvium
                        // scours freely regardless of the lag beneath it.
                        let shield = if self.sed_bed[i] > 1e-4 { 1.0 } else { 1.0 - p.armor_shield * self.armor[i] };
                        let e = ((capacity - s0) * p.sed_erode * dt * shield).min(max_step);
                        self.bed[i] -= e;
                        self.sediment[i] += e;
                        // Scour strips the alluvium cover and re-opens the pores;
                        // cutting into parent bed winnows fines → the lag grows.
                        let into_parent = (e - self.sed_bed[i]).max(0.0);
                        self.sed_bed[i] = (self.sed_bed[i] - e).max(0.0);
                        self.colmation[i] = (self.colmation[i] - e / p.plug_depth).max(0.0);
                        self.armor[i] = (self.armor[i] + into_parent / p.armor_depth).min(1.0);
                    } else if s0 > capacity {
                        let dp = ((s0 - capacity) * p.sed_deposit * dt).min(max_step).min(s0);
                        self.bed[i] += dp;
                        self.sediment[i] -= dp;
                        self.sed_bed[i] += dp;
                        self.colmation[i] = (self.colmation[i] + dp / p.plug_depth).min(1.0);
                        // Burial: fresh loose material covers the lag.
                        self.armor[i] = (self.armor[i] - dp / p.armor_depth).max(0.0);
                    }
                }
            }
            // Advect: sediment leaves with the same volume fraction as the water.
            let snap = self.sediment.clone();
            for y in 0..nx {
                for x in 0..nx {
                    let i = y * nx + x;
                    let d = self.depth[i];
                    if d < 1e-4 || snap[i] <= 0.0 {
                        continue;
                    }
                    let out = (self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i]) * dt;
                    let frac = (out / (d * area)).min(1.0);
                    let moving = snap[i] * frac;
                    if moving <= 0.0 {
                        continue;
                    }
                    self.sediment[i] -= moving;
                    let total = self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i];
                    if x > 0 {
                        self.sediment[i - 1] += moving * self.fl[i] / total;
                    }
                    if x < nx - 1 {
                        self.sediment[i + 1] += moving * self.fr[i] / total;
                    }
                    if y > 0 {
                        self.sediment[i - nx] += moving * self.ft[i] / total;
                    }
                    if y < nx - 1 {
                        self.sediment[i + nx] += moving * self.fb[i] / total;
                    }
                }
            }
        }

        // 4c. Groundwater: infiltrate (sealed down where the channel flows hard —
        //     live colmation), then baseflow back. Reservoir-to-reservoir only.
        if p.infiltration > 0.0 {
            for y in 0..nx {
                for x in 0..nx {
                    let i = y * nx + x;
                    let d = self.depth[i];
                    if d > 0.0 && self.groundwater[i] < p.gw_capacity {
                        let out = self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i];
                        let q = out / l; // specific discharge proxy, m²/s
                        // Two bed phenomena (Joseph): the flow itself (armour /
                        // pressure, instantaneous) and colmation (plugged pores,
                        // persistent). Never fully watertight — 2% leaks.
                        let seal = (1.0 / (1.0 + q / p.seal_q)) * (1.0 - self.colmation[i]).max(0.02);
                        let take = (p.infiltration * seal * dt).min(d).min(p.gw_capacity - self.groundwater[i]);
                        self.depth[i] -= take;
                        self.groundwater[i] += take;
                    }
                    let back = (self.groundwater[i] * p.baseflow * dt).min(self.groundwater[i]);
                    self.groundwater[i] -= back;
                    self.depth[i] += back;
                }
            }
        }

        // 5. Evaporation (surface → atmosphere) and re-hold the edge boundary.
        if p.evaporation > 0.0 {
            let e = p.evaporation * dt;
            for d in self.depth.iter_mut() {
                let evap = e.min(*d);
                *d -= evap;
                self.atmosphere += evap as f64;
            }
        }
        self.hold_edge_sea(p.sea_m);
    }

    /// Hold edge cells with bed at/below sea at the waterline, exchange counted.
    fn hold_edge_sea(&mut self, sea: f32) {
        let nx = self.nx;
        let hold = |i: usize, s: &mut Self| {
            if s.bed[i] <= sea {
                let target = sea - s.bed[i];
                s.ocean += (s.depth[i] - target) as f64;
                s.depth[i] = target;
            }
        };
        for x in 0..nx {
            hold(x, self);
            hold((nx - 1) * nx + x, self);
        }
        for y in 1..nx - 1 {
            hold(y * nx, self);
            hold(y * nx + nx - 1, self);
        }
    }

    /// CFL-stable timestep for the CURRENT state: deep water carries fast waves
    /// (`√(g·d)`), so dt shrinks where the basin is deep. Callers should recompute
    /// per burst.
    /// Froude stats from INSIDE the last step — measured at the instant, and
    /// against the same sill depths, the breaking cap operates on. (max Fr,
    /// fraction of wet cells with any pipe Fr > 1.5.) By construction the cap
    /// bounds this at 2.0: a reading of 2.0 means flow pinned at breaking
    /// (wants to exceed); a reading ABOVE 2.0 means the cap is genuinely
    /// broken. Post-step recomputation against drained depths is how this
    /// gauge lied twice (Fr 100+, then 18+) — do not move it back out.
    pub fn froude(&self) -> (f32, f32) {
        self.last_froude
    }

    pub fn stable_dt(&self, gravity: f32) -> f32 {
        let dmax = self.depth.iter().cloned().fold(0.1f32, f32::max);
        (0.3 * self.cell_m / (gravity * dmax).sqrt()).clamp(0.005, 0.2)
    }

    /// Snapshot for sampling by views.
    pub fn to_region(&self) -> WaterRegion {
        // Net velocity per cell, for local (pawn) instrumentation. Each pipe's
        // speed is its flux over its own SILL depth (h_flow) — the conveyance
        // the momentum cap operates on. Dividing by the cell's own depth
        // inflated thin cells beside deep neighbours to 80 m/s (the same
        // instrument lie the Froude gauge told, in a third place).
        let l = self.cell_m;
        let nx = self.nx;
        let n = nx * nx;
        let (mut vx, mut vy) = (vec![0.0f32; n], vec![0.0f32; n]);
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                if self.depth[i] <= 1e-3 {
                    continue;
                }
                let eta_i = self.bed[i] + self.depth[i];
                let vp = |f: f32, j: usize| -> f32 {
                    let hflow = eta_i.max(self.bed[j] + self.depth[j]) - self.bed[i].max(self.bed[j]);
                    if hflow > 1e-3 { f / (hflow * l) } else { 0.0 }
                };
                let r = if x < nx - 1 { vp(self.fr[i], i + 1) } else { 0.0 };
                let lft = if x > 0 { vp(self.fl[i], i - 1) } else { 0.0 };
                let b = if y < nx - 1 { vp(self.fb[i], i + nx) } else { 0.0 };
                let t = if y > 0 { vp(self.ft[i], i - nx) } else { 0.0 };
                vx[i] = r - lft;
                vy[i] = b - t;
            }
        }
        WaterRegion {
            vx,
            vy,
            face: self.face,
            level: self.level,
            oi: self.origin.0,
            oj: self.origin.1,
            nx: self.nx,
            depth: self.depth.clone(),
            bed: self.bed.clone(),
            sediment: self.sediment.clone(),
            sed_bed: self.sed_bed.clone(),
            colmation: self.colmation.clone(),
            armor: self.armor.clone(),
        }
    }
}

/// A sampleable water-depth snapshot (bilinear, any finer cell) — the water
/// analogue of [`crate::erosion::ErodedRegion`].
#[derive(Clone)]
pub struct WaterRegion {
    pub face: Face,
    pub level: u8,
    pub oi: u32,
    pub oj: u32,
    pub nx: usize,
    pub depth: Vec<f32>,
    pub bed: Vec<f32>,
    pub sediment: Vec<f32>,
    pub sed_bed: Vec<f32>,
    pub colmation: Vec<f32>,
    pub armor: Vec<f32>,
    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
}

impl WaterRegion {
    /// Net flow velocity components (m/s along +i, +j) — for flow arrows/float.
    pub fn velocity_m_s(&self, cell: CellId) -> Option<(f64, f64)> {
        Some((self.bilinear(&self.vx, cell)?, self.bilinear(&self.vy, cell)?))
    }

    /// Net flow speed (m/s) — pawn-local instrumentation.
    pub fn speed_m_s(&self, cell: CellId) -> Option<f64> {
        let (x, y) = self.velocity_m_s(cell)?;
        Some((x * x + y * y).sqrt())
    }
    fn bilinear(&self, field: &[f32], cell: CellId) -> Option<f64> {
        let (face, i, j, level) = cell.to_face_ij();
        if face != self.face || level < self.level {
            return None;
        }
        let scale = (1u64 << (level - self.level)) as f64;
        let gx = (i as f64 + 0.5) / scale - self.oi as f64 - 0.5;
        let gy = (j as f64 + 0.5) / scale - self.oj as f64 - 0.5;
        if gx < 0.0 || gy < 0.0 || gx > (self.nx - 2) as f64 || gy > (self.nx - 2) as f64 {
            return None;
        }
        let (x0, y0) = (gx.floor() as usize, gy.floor() as usize);
        let (fx, fy) = (gx - x0 as f64, gy - y0 as f64);
        let at = |x: usize, y: usize| field[y * self.nx + x] as f64;
        Some(at(x0, y0) * (1.0 - fx) * (1.0 - fy)
            + at(x0 + 1, y0) * fx * (1.0 - fy)
            + at(x0, y0 + 1) * (1.0 - fx) * fy
            + at(x0 + 1, y0 + 1) * fx * fy)
    }

    pub fn depth_m(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.depth, cell)
    }

    /// Water SURFACE elevation (bed + depth, m). The surface is the physically
    /// continuous field — sample THIS when rendering over terrain of a different
    /// LOD; interpolating depth over a mismatched bed shreds a level pool into
    /// beads (Joseph's "bubbles of water flowing in waves").
    pub fn surface_m(&self, cell: CellId) -> Option<f64> {
        Some(self.bilinear(&self.bed, cell)? + self.bilinear(&self.depth, cell)?)
    }

    /// The SIMULATED bed elevation (m) — authoritative where the water has
    /// flowed: rendering painted sub-sim detail beneath simulated water shows
    /// terrain the physics never saw (dishonest; Joseph). Sample this for
    /// ground height in wet cells.
    pub fn bed_m(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.bed, cell)
    }

    /// Suspended-sediment load (m of solid), for turbidity.
    pub fn suspended_m(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.sediment, cell)
    }

    /// Settled alluvium thickness (m) — sandy beds.
    pub fn sed_bed_m(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.sed_bed, cell)
    }

    /// Pore-plug fraction 0..1 — muddy sealed beds.
    pub fn colmation_at(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.colmation, cell)
    }

    /// Coarse-lag fraction 0..1 — armored (rocky) beds.
    pub fn armor_at(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.armor, cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A closed inland bowl (all bed above sea, walls high) — rain in, no way out.
    fn bowl(nx: usize) -> WaterSim {
        let mut bed = vec![5000.0f32; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let (cx, cy) = (x as f32 - nx as f32 / 2.0, y as f32 - nx as f32 / 2.0);
                bed[y * nx + x] = 5000.0 + (cx * cx + cy * cy) * 0.5;
            }
        }
        // 1 m of rainable sky: the physical Manning films are honestly slow, so
        // the test needs a real storm's worth of water and time, not a sprinkle.
        WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, 4.8, bed, 1.0)
    }

    #[test]
    fn conserves_total_water() {
        let mut w = bowl(32);
        let p = WaterParams::default();
        let before = w.total_water();
        let solid_before = w.total_solid();
        for _ in 0..400 {
            w.step(&p);
        }
        let after = w.total_water();
        assert!((before - after).abs() < 1e-3, "water not conserved: {before} -> {after}");
        assert!(w.depth.iter().all(|d| *d >= 0.0 && d.is_finite()));
        // Solid (bed + suspended) is conserved by the sediment stage too.
        let solid_after = w.total_solid();
        assert!(
            (solid_before - solid_after).abs() / solid_before.abs().max(1.0) < 1e-6,
            "solid not conserved: {solid_before} -> {solid_after}"
        );
        assert!(w.sediment.iter().all(|s| *s >= 0.0 && s.is_finite()));
    }

    #[test]
    fn rain_pools_in_the_bowl() {
        let mut w = bowl(32);
        // STORM forcing: at climate-average rain the budget is negative
        // (infiltration + evaporation exceed it — drizzle never runs off, which
        // is real climatology and was verified by probe). Runoff needs a burst.
        let p = WaterParams { precip: 6.0e-4, ..Default::default() };
        for _ in 0..1500 {
            w.step(&p);
        }
        let centre = w.depth[16 * 32 + 16];
        let corner = w.depth[2 * 32 + 2];
        assert!(centre > 0.01, "no pooling at the low centre ({centre})");
        assert!(centre > corner * 5.0, "water did not concentrate downhill (centre {centre} corner {corner})");
    }

    #[test]
    fn deterministic() {
        let p = WaterParams::default();
        let mut a = bowl(24);
        let mut b = bowl(24);
        for _ in 0..100 {
            a.step(&p);
            b.step(&p);
        }
        assert_eq!(a.depth, b.depth);
    }
}
