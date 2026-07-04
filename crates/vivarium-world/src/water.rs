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
    /// BASE Manning roughness n (s/m^⅓) for gentle channels (≈0.04). Steep
    /// channels are boulder-strewn and step-pooled — roughness RISES with
    /// slope (Jarrett 1984: n ≈ 0.39·S^0.38 for Rocky Mountain streams; our
    /// linear proxy n = base + 1.6·S, capped 0.13, tracks it near R≈1 m).
    /// With the lowland 0.04 applied everywhere, torrents ran 16 m/s — real
    /// streams top out ~3–4 (Joseph's field check); this is why.
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
    /// Fines needed to fully plug the bed's pores (m of deposited FINES).
    /// A few mm of silt seals a streambed (real colmation depths are mm–cm);
    /// scour re-opens it at the same exchange rate.
    pub plug_depth: f32,
    /// Fraction of deposited load that is pore-plugging FINES (silt/clay);
    /// the coarser rest builds alluvium without sealing. Counting ALL deposit
    /// as fines sealed the whole world within minutes of a deluge (Joseph).
    pub fines_frac: f32,
    /// Suspension threshold for FINES (Pa): below silt's ~0.06 Pa a bed can
    /// accept fines; above it the flow keeps them entrained — fines ride
    /// through steep channels and settle in slack water. Without this gate,
    /// transport-limited channels sealed into "mud sealant" (Joseph) because
    /// they never net-erode and every storm deposited fines.
    pub tau_fines: f32,
    /// Turbulent (eddy) diffusivity for suspended load and future water-borne
    /// scalars: K = base convective stirring + shear-scaled dispersion
    /// (rivers mix transversely at ~0.1–1 m²/s; lakes stir weakly but never
    /// zero). Pure advection left razor-sharp brown/blue fronts standing in
    /// still water indefinitely (Joseph: "are we missing a diffusion
    /// process?" — yes, this one).
    pub eddy_base: f32,
    pub eddy_shear: f32,
    /// Winnowing rate (per sim-s at 2×τ_c): armoring by surface grain
    /// exchange in transport-limited reaches — real beds coarsen under flood
    /// shear even with zero net erosion. Calibrated to PAVEMENT-over-several-
    /// storm-cycles (~10³ sim-s), NOT single-event flume rearrangement: the
    /// armor_regimes probe caught the 1/600 first guess saturating armor in
    /// ~300 sim-s, before channels did any measurable work — a world-wide
    /// incision freeze, silently.
    pub winnow_rate: f32,
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
            fines_frac: 0.15,
            tau_fines: 0.06,
            winnow_rate: 1.0 / 6000.0,
            eddy_base: 0.02,
            eddy_shear: 0.3,
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
    /// Kahan residual for `bed` writes. The bed sits at a ~6000 m datum where
    /// one f32 ULP is ~0.5 mm; a per-step erosion increment below the half-ULP
    /// (~0.24 mm) rounds away ENTIRELY — the source-cell exact-zero-incision
    /// anomaly (armor_regimes regime 3): τ ≫ τ_c, `sediment += e` receives
    /// mass, armor grows, and the bed never moves — solid mass created from
    /// nothing, incision frozen. Compensated summation keeps the lost low
    /// bits here until they amount to a representable change. Stays < 1 ULP
    /// (≲0.5 mm) by construction; counted in [`Self::total_solid`].
    bed_res: Vec<f32>,
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
    /// The conserved total at construction — the budget-drift baseline.
    initial_total: f64,
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
        let mut this = Self {
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
            bed_res: z.clone(),
            colmation: z.clone(),
            armor: z.clone(),
            atmosphere: atmosphere_m * (nx * nx) as f64,
            ocean: 0.0,
            last_froude: (0.0, 0.0),
            initial_total: 0.0,
            fl: z.clone(),
            fr: z.clone(),
            ft: z.clone(),
            fb: z,
        };
        this.initial_total = this.total_water();
        this
    }

    /// Re-anchor the budget baseline to the CURRENT total — for sims restored
    /// from a snapshot, whose fields were overwritten after construction.
    pub fn rebaseline_budget(&mut self) {
        self.initial_total = self.total_water();
    }

    /// Water budget drift since construction (m·cells): the LIVE conservation
    /// instrument. The physics conserves by construction and by test; what
    /// this catches is anything that breaks that later, plus honest f32
    /// rounding accumulated over hours. Should hover near zero.
    pub fn budget_drift(&self) -> f64 {
        self.total_water() - self.initial_total
    }

    /// The conserved total: surface + groundwater + atmosphere + ocean.
    pub fn total_water(&self) -> f64 {
        self.depth.iter().map(|&d| d as f64).sum::<f64>()
            + self.groundwater.iter().map(|&g| g as f64).sum::<f64>()
            + self.atmosphere
            + self.ocean
    }

    /// The conserved SOLID total: bed (+ its compensation residual) + suspended
    /// (m, cell-area units).
    pub fn total_solid(&self) -> f64 {
        self.bed.iter().map(|&b| b as f64).sum::<f64>()
            + self.bed_res.iter().map(|&r| r as f64).sum::<f64>()
            + self.sediment.iter().map(|&s| s as f64).sum::<f64>()
    }

    /// Refresh the bed from the (still-eroding) terrain tier — the quasi-static
    /// coupling. Water simply finds itself on the new surface; its volume is
    /// untouched, so conservation holds across the swap.
    pub fn set_bed(&mut self, bed: Vec<f32>) {
        assert_eq!(bed.len(), self.nx * self.nx);
        self.bed = bed;
        // The residual compensates the OLD bed's accumulation; a fresh bed
        // starts a fresh sum.
        self.bed_res.fill(0.0);
    }

    /// Compensated (Kahan) increment of `bed[i]` — see `bed_res`. Every bed
    /// write in the sediment stage goes through here; a bare `bed[i] += dz`
    /// silently drops sub-ULP increments at mountain datums.
    #[inline]
    fn bed_add(bed: &mut f32, res: &mut f32, dz: f32) {
        let y = dz + *res;
        let t = *bed + y;
        *res = y - (t - *bed);
        *bed = t;
    }

    /// One shallow-water step — the five classic stages, every transfer counted.
    pub fn step(&mut self, p: &WaterParams) {
        let nx = self.nx;
        let n = nx * nx;
        let (l, dt) = (self.cell_m, p.dt);
        let area = l * l;

        // 1. Rain: atmosphere → surface, scaled so it never rains water that
        //    does not exist. REALIZED-DELTA accounting: debit the atmosphere by
        //    what the f32 depth actually absorbed, not by the intended amount.
        //    A ~1e-6 m transfer is below the ulp of a 100 m-deep f32 cell, and
        //    at steady state the depth field is stationary — so the per-cell
        //    rounding error is FROZEN and repeats identically every step: the
        //    budget gauge's "too linear for rounding" drift was exactly this,
        //    intended-vs-realized bias summed over the deep cells (budget_probe).
        if p.precip > 0.0 && self.atmosphere > 0.0 {
            let want = p.precip as f64 * dt as f64 * n as f64;
            let scale = (self.atmosphere / want).min(1.0) as f32;
            let per_cell = p.precip * dt * scale;
            let mut landed = 0.0f64;
            for d in self.depth.iter_mut() {
                let before = *d;
                *d += per_cell;
                landed += (*d - before) as f64;
            }
            self.atmosphere -= landed;
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
        fn pipe_step(f: f32, eta_i: f32, eta_j: f32, b_i: f32, b_j: f32, dt: f32, g: f32, n_base: f32, l: f32) -> f32 {
            let hflow = eta_i.max(eta_j) - b_i.max(b_j);
            if hflow < 1e-4 {
                return 0.0; // no conveyance over the sill
            }
            let head = eta_i - eta_j;
            // Slope-dependent roughness (Jarrett 1984, linearized): steep
            // reaches are rough — this is what holds torrents to nature's
            // 2–4 m/s instead of Manning-lowland's 10+.
            let n = (n_base + 1.6 * (head.max(0.0) / l)).min(0.13);
            let accel = (f + dt * g * hflow * head).max(0.0);
            let v = accel / (hflow * l);
            let f = accel / (1.0 + dt * g * n * n * v / hflow.powf(4.0 / 3.0));
            // Breaking limit: natural steep streams self-organise to Fr ≈ 1
            // (Grant 1997) — surge fronts that outrun ~2× critical BREAK and
            // shed momentum as turbulence. Without this loss the roll waves a
            // deluge excites grow unbounded (Joseph's multi-metre travelling
            // blobs; probe: near-dry gaps between 3 m lumps).
            f.min(2.0 * (g * hflow).sqrt() * hflow * l)
        }
        let n_base = p.manning_n;
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
                    self.fl[i] = pipe_step(self.fl[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n_base, l);
                    probe(self.fl[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                if x < nx - 1 {
                    let j = i + 1;
                    self.fr[i] = pipe_step(self.fr[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n_base, l);
                    probe(self.fr[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                if y > 0 {
                    let j = i - nx;
                    self.ft[i] = pipe_step(self.ft[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n_base, l);
                    probe(self.ft[i], self.bed[j] + self.depth[j], self.bed[j]);
                }
                if y < nx - 1 {
                    let j = i + nx;
                    self.fb[i] = pipe_step(self.fb[i], eta_i, self.bed[j] + self.depth[j], bi, self.bed[j], dt, p.gravity, n_base, l);
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
                        Self::bed_add(&mut self.bed[i], &mut self.bed_res[i], dp);
                        self.sediment[i] -= dp;
                        self.sed_bed[i] += dp;
                        self.colmation[i] = (self.colmation[i] + dp * p.fines_frac / p.plug_depth).min(1.0);
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
                        Self::bed_add(&mut self.bed[i], &mut self.bed_res[i], -e);
                        self.sediment[i] += e;
                        // Scour strips the alluvium cover and re-opens the pores;
                        // cutting into parent bed winnows fines → the lag grows.
                        let into_parent = (e - self.sed_bed[i]).max(0.0);
                        self.sed_bed[i] = (self.sed_bed[i] - e).max(0.0);
                        self.colmation[i] = (self.colmation[i] - e / p.plug_depth).max(0.0);
                        self.armor[i] = (self.armor[i] + into_parent / p.armor_depth).min(1.0);
                    } else if s0 > capacity {
                        // Fines stay ENTRAINED above their suspension threshold
                        // (silt ~0.06 Pa): steep channels deposit only the
                        // coarse fraction and never colmate — the fines ride
                        // through and settle in slack water, which is where
                        // mud belongs. (Rouse-number logic, crude rung.)
                        let fines_settle = tau < p.tau_fines;
                        let frac = if fines_settle { 1.0 } else { 1.0 - p.fines_frac };
                        let dp = ((s0 - capacity) * p.sed_deposit * dt * frac).min(max_step).min(s0);
                        Self::bed_add(&mut self.bed[i], &mut self.bed_res[i], dp);
                        self.sediment[i] -= dp;
                        self.sed_bed[i] += dp;
                        if fines_settle {
                            self.colmation[i] = (self.colmation[i] + dp * p.fines_frac / p.plug_depth).min(1.0);
                        }
                        // Burial: fresh loose material covers the lag.
                        self.armor[i] = (self.armor[i] - dp / p.armor_depth).max(0.0);
                    }
                    // WINNOWING (transport-limited armoring): under sustained
                    // shear the bed surface exchanges grains — fines wash out
                    // (de-colmation) and coarse concentrates (armor) even with
                    // ZERO net erosion. This is why real steep channels are
                    // stony, not sealed, despite running at capacity.
                    if tau > p.tau_c {
                        let x = ((tau / p.tau_c - 1.0).min(3.0)) * p.winnow_rate * dt;
                        self.armor[i] = (self.armor[i] + x * (1.0 - self.sed_bed[i].min(1.0))).min(1.0);
                        self.colmation[i] = (self.colmation[i] - x * 2.0).max(0.0);
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

        // 4b². Turbulent mixing of suspended load: conservative pairwise
        // exchange driven by the CONCENTRATION difference (eddy diffusivity =
        // convective base + shear-scaled dispersion). Right-neighbour and
        // down-neighbour sweeps cover every pair once.
        if p.sed_capacity > 0.0 {
            let snap = self.sediment.clone();
            for y in 0..nx {
                for x in 0..nx {
                    let i = y * nx + x;
                    let di = self.depth[i];
                    if di < 1e-3 {
                        continue;
                    }
                    let pair = |i: usize, j: usize, s_this: &mut Self| {
                        let dj = s_this.depth[j];
                        if dj < 1e-3 {
                            return;
                        }
                        let (ci, cj) = (snap[i] / di, snap[j] / dj);
                        let vi = ((s_this.fl[i] + s_this.fr[i] + s_this.ft[i] + s_this.fb[i]) / (di * l)).min(5.0);
                        let k_eddy = p.eddy_base + p.eddy_shear * vi * l * 0.1;
                        // Mass moved: K·Δc·(interface depth)·dt / l — bounded
                        // for stability at 25% of the donor's excess.
                        let dmin = di.min(dj);
                        let m = (k_eddy * (ci - cj) * dmin * dt / l).clamp(-0.25 * snap[j], 0.25 * snap[i]);
                        s_this.sediment[i] -= m;
                        s_this.sediment[j] += m;
                    };
                    if x < nx - 1 {
                        pair(i, i + 1, self);
                    }
                    if y < nx - 1 {
                        pair(i, i + nx, self);
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
                        // Realized-delta: mirror the depth's ACTUAL f32 change
                        // into groundwater (stage 1's frozen-rounding note; the
                        // gw side's ulp is ~1000× finer, so the residual is a
                        // negligible random walk, not a frozen bias). May
                        // overshoot gw_capacity by ≤ one deep-cell ulp — the
                        // `< gw_capacity` guard stops further intake.
                        let before = self.depth[i];
                        self.depth[i] -= take;
                        self.groundwater[i] += before - self.depth[i];
                    }
                    let back = (self.groundwater[i] * p.baseflow * dt).min(self.groundwater[i]);
                    let before = self.depth[i];
                    self.depth[i] += back;
                    let realized = self.depth[i] - before;
                    if realized <= self.groundwater[i] {
                        self.groundwater[i] -= realized;
                    } else {
                        // The return is below this depth's f32 resolution and
                        // rounding overshot the store: skip — the water stays
                        // in the soil rather than being minted.
                        self.depth[i] = before;
                    }
                }
            }
        }

        // 5. Evaporation (surface → atmosphere) and re-hold the edge boundary.
        //    Realized-delta, as with rain: credit the atmosphere with what the
        //    f32 depth actually lost (see stage 1's frozen-rounding note).
        if p.evaporation > 0.0 {
            let e = p.evaporation * dt;
            for d in self.depth.iter_mut() {
                let before = *d;
                *d = (*d - e.min(*d)).max(0.0);
                self.atmosphere += (before - *d) as f64;
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
            groundwater: self.groundwater.clone(),
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
    pub groundwater: Vec<f32>,
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

    /// Groundwater store (m of water in the soil column) — near-surface
    /// saturation for wet-ground rendering and (later) slope stability.
    pub fn groundwater_m(&self, cell: CellId) -> Option<f64> {
        self.bilinear(&self.groundwater, cell)
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
        // Solid (bed + residual + suspended) is conserved by the sediment
        // stage too. ABSOLUTE tolerance: the earlier relative 1e-6 (of a
        // ~5e6 m·cells bed integral) hid ~5 m·cells of created mass — which
        // is exactly what the f32-absorption bug produced (bed subtraction
        // rounded away while `sediment += e` landed). See `bed_res`.
        let solid_after = w.total_solid();
        assert!(
            (solid_before - solid_after).abs() < 1e-3,
            "solid not conserved: {solid_before} -> {solid_after}"
        );
        assert!(w.sediment.iter().all(|s| *s >= 0.0 && s.is_finite()));
    }

    /// The bowl test above never exercises DEEP water — and deep f32 depths are
    /// where reservoir exchanges go subtly wrong: a ~1e-6 m rain/evap transfer
    /// is below the ulp of a 100–400 m depth, and at steady state the depth
    /// field is stationary, so intended-vs-realized rounding bias is FROZEN and
    /// integrates linearly (the worldview budget gauge's −0.37 m·cells/sim-s;
    /// probe: examples/budget_probe.rs). Realized-delta accounting makes every
    /// field↔reservoir exchange exact by construction; this guards it, with an
    /// ABSOLUTE bound — a relative tolerance is how the sibling bed-absorption
    /// bug stayed hidden inside a 1e-6 slack on a huge total.
    #[test]
    fn conserves_in_deep_water() {
        let nx = 48;
        let sea = crate::gen::SEA_LEVEL_M as f32;
        // Left half: 400 m-deep ocean floor; right half rises inland.
        let mut bed = vec![0.0f32; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let t = x as f32 / nx as f32;
                bed[y * nx + x] = if t < 0.5 { sea - 400.0 } else { sea - 2.0 + (t - 0.5) * 120.0 };
            }
        }
        let mut w = WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, 4.8, bed, 1.0);
        // Step at the CFL-stable dt, as every real caller does. (At a
        // CFL-VIOLATING dt on deep water this test fails for a different,
        // out-of-contract reason: the oscillating flux clamp's stage-4
        // `max(0.0)` mints small amounts. The kernel's conservation guarantee
        // is stated under the documented dt contract.)
        let deluge = WaterParams { precip: WaterParams::default().precip * 60.0, ..Default::default() };
        for _ in 0..600 {
            let dt = w.stable_dt(9.8);
            w.step(&WaterParams { dt, ..deluge });
        }
        w.rebaseline_budget();
        let storm = WaterParams { precip: WaterParams::default().precip * 10.0, ..Default::default() };
        let mut sim_s = 0.0f64;
        for _ in 0..2000 {
            let dt = w.stable_dt(9.8);
            w.step(&WaterParams { dt, ..storm });
            sim_s += dt as f64;
        }
        // Pre-fix the frozen bias integrated to ~1e2 m·cells here (−9 m·cells
        // per sim-s at this depth, linear). Post-fix the residue is unfrozen
        // f32 rounding — a random walk orders of magnitude below this bound.
        let drift = w.budget_drift().abs();
        assert!(drift < 0.5, "deep-water budget drifted: {drift} m·cells over {sim_s:.1} sim-s");
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
