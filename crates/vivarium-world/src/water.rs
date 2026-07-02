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
    /// Virtual-pipe cross-section (m²) — flow responsiveness per unit head.
    pub pipe_area: f32,
    /// Precipitation (m/s per cell) drawn from the atmosphere store. ~1000× real.
    pub precip: f32,
    /// Surface evaporation, fraction of depth per second (returns to atmosphere).
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
    /// Carrying capacity per unit flow speed (m of sediment per m/s).
    pub sed_capacity: f32,
    /// Erosion rate toward capacity (fraction of deficit per second).
    pub sed_erode: f32,
    /// Settling rate above capacity (fraction of excess per second).
    pub sed_deposit: f32,
    /// Max bed change per step (m) — a hard sanity bound.
    pub sed_max_step: f32,
    /// Per-step flux damping (friction): undamped pipes ring — water overshoots
    /// and sloshes in surge waves (Joseph saw pulses running down valleys instead
    /// of streams). 0.99 ≈ a ~20-step (4 sim-s) momentum memory.
    pub damping: f32,
}

impl Default for WaterParams {
    fn default() -> Self {
        Self {
            gravity: 9.8,
            dt: 0.2,
            pipe_area: 4.0,
            precip: 3.0e-5,
            evaporation: 1.0e-4,
            ocean_evap: 1.0e-5,
            sea_m: crate::gen::SEA_LEVEL_M as f32,
            sed_capacity: 0.05,
            sed_erode: 0.1,
            sed_deposit: 0.5,
            sed_max_step: 0.005,
            damping: 0.99,
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
    /// Counted reservoirs (m of water, cell-area units): conservation partners.
    pub atmosphere: f64,
    pub ocean: f64,
    // Outgoing flux per axial pipe (m³/s), kept between steps: momentum.
    fl: Vec<f32>,
    fr: Vec<f32>,
    ft: Vec<f32>,
    fb: Vec<f32>,
}

impl WaterSim {
    /// A dry domain over `bed`, with `atmosphere_m` of rainable water per cell.
    pub fn new(face: Face, level: u8, origin: (u32, u32), nx: usize, cell_m: f32, bed: Vec<f32>, atmosphere_m: f64) -> Self {
        assert_eq!(bed.len(), nx * nx);
        let z = vec![0.0f32; nx * nx];
        Self {
            nx,
            cell_m,
            face,
            level,
            origin,
            bed,
            depth: z.clone(),
            sediment: z.clone(),
            atmosphere: atmosphere_m * (nx * nx) as f64,
            ocean: 0.0,
            fl: z.clone(),
            fr: z.clone(),
            ft: z.clone(),
            fb: z,
        }
    }

    /// The conserved total: surface + atmosphere + ocean (m, cell-area units).
    pub fn total_water(&self) -> f64 {
        self.depth.iter().map(|&d| d as f64).sum::<f64>() + self.atmosphere + self.ocean
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

        // 2. Hold sea cells at the waterline (exchange counted with the ocean).
        for i in 0..n {
            if self.bed[i] <= p.sea_m {
                let target = p.sea_m - self.bed[i];
                self.ocean += (self.depth[i] - target) as f64;
                self.depth[i] = target;
            }
        }

        // 3. Accelerate pipes under head differences; border treated as
        //    equal-surface (edges belong to the sea/outside — step 5 drains).
        let k = dt * p.pipe_area * p.gravity / l;
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                let s = self.bed[i] + self.depth[i];
                let head = |j: usize| s - (self.bed[j] + self.depth[j]);
                let damp = p.damping;
                if x > 0 {
                    self.fl[i] = (self.fl[i] * damp + k * head(i - 1)).max(0.0);
                }
                if x < nx - 1 {
                    self.fr[i] = (self.fr[i] * damp + k * head(i + 1)).max(0.0);
                }
                if y > 0 {
                    self.ft[i] = (self.ft[i] * damp + k * head(i - nx)).max(0.0);
                }
                if y < nx - 1 {
                    self.fb[i] = (self.fb[i] * damp + k * head(i + nx)).max(0.0);
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
                    if d < 1e-3 {
                        // No meaningful flow: everything settles.
                        let dp = self.sediment[i].min(p.sed_max_step);
                        self.bed[i] += dp;
                        self.sediment[i] -= dp;
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
                    let capacity = (p.sed_capacity * speed).min(2.0);
                    let s0 = self.sediment[i];
                    if s0 < capacity {
                        let e = ((capacity - s0) * p.sed_erode * dt).min(p.sed_max_step);
                        self.bed[i] -= e;
                        self.sediment[i] += e;
                    } else {
                        let dp = ((s0 - capacity) * p.sed_deposit * dt).min(p.sed_max_step).min(s0);
                        self.bed[i] += dp;
                        self.sediment[i] -= dp;
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

        // 5. Evaporation (surface → atmosphere) and re-hold the sea.
        if p.evaporation > 0.0 {
            let f = p.evaporation * dt;
            for d in self.depth.iter_mut() {
                let evap = *d * f;
                *d -= evap;
                self.atmosphere += evap as f64;
            }
        }
        for i in 0..n {
            if self.bed[i] <= p.sea_m {
                let target = p.sea_m - self.bed[i];
                self.ocean += (self.depth[i] - target) as f64;
                self.depth[i] = target;
            }
        }
    }

    /// Snapshot for sampling by views.
    pub fn to_region(&self) -> WaterRegion {
        WaterRegion {
            face: self.face,
            level: self.level,
            oi: self.origin.0,
            oj: self.origin.1,
            nx: self.nx,
            depth: self.depth.clone(),
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
}

impl WaterRegion {
    pub fn depth_m(&self, cell: CellId) -> Option<f64> {
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
        let at = |x: usize, y: usize| self.depth[y * self.nx + x] as f64;
        Some(at(x0, y0) * (1.0 - fx) * (1.0 - fy)
            + at(x0 + 1, y0) * fx * (1.0 - fy)
            + at(x0, y0 + 1) * (1.0 - fx) * fy
            + at(x0 + 1, y0 + 1) * fx * fy)
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
        WaterSim::new(Face::ZPos, 21, (1000, 1000), nx, 4.8, bed, 0.05)
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
        let p = WaterParams::default();
        for _ in 0..800 {
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
