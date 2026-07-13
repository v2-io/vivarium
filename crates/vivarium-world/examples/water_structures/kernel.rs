//! INSTRUMENTED COPY of `vivarium-world::water`'s momentum + continuity stages.
//!
//! `water.rs` is NOT edited. This is a copy, and the fidelity contract is:
//!
//! * Stages reproduced **verbatim** (same expressions, same order, same in-place
//!   Gauss–Seidel sweep): 1 (rain), 1b (ocean evap), 2 (edge sea hold),
//!   3 (θ flux smoothing + `pipe_step` + flux clamp), 4 (depth update),
//!   4c (groundwater), 5 (surface evap + re-hold).
//! * Stages OMITTED: 4b / 4b² (sediment + eddy mixing). Every probe here runs with
//!   `sed_capacity = 0`, under which `water.rs` skips both blocks entirely — so the
//!   omission is a no-op for these runs, not a simplification of them.
//! * Knobs ADDED (all default to `water.rs`'s behaviour):
//!     - `theta` is a parameter, not the hardcoded `0.8`.
//!     - `smooth`: GaussSeidel (what `water.rs` does) | Jacobi (out-of-place control) | Off.
//!     - `breaking`: the Fr≈2 cap, switchable off.
//!     - per-stage ENERGY ACCOUNTING (the four sinks), which `water.rs` has no hook for.
//! * Generic over the float type via a macro ⇒ `k32` (what ships) and `k64` (the
//!   round-off control). Same source text, two precisions.
//!
//! Energy conventions (ρ = 1000 kg/m³, A = l²):
//!   PE   = Σ_i  ρ·g·A·d_i·(b_i + d_i/2)          [J]   (column centre of mass)
//!   KE   = ½·ρ·Σ_pipes f_p² / hflow_p            [J]   (v_p = f_p/(hflow_p·l), mass ρ·l²·hflow_p)
//! Both are exact functions of state; every sink below is measured as the KE delta
//! that stage produced, at fixed depth (so hflow is constant across the measurement).

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smooth {
    /// What `water.rs` ships: in-place sweep, fl/fr along +x, ft/fb along +y.
    GaussSeidel,
    /// Out-of-place: the symmetric operator the in-place sweep is an approximation of.
    Jacobi,
    Off,
}

/// Per-step energy accounting, in Joules. Sinks are NEGATIVE when they remove energy.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sinks {
    /// θ flux smoothing. Physical claim: **the thing under audit**.
    pub theta: f64,
    /// Manning bed friction. Physical claim: bed shear.
    pub friction: f64,
    /// The Fr≈2 breaking cap (Grant 1997). Physical claim: turbulent surge breaking.
    pub breaking: f64,
    /// The "cannot ship more water than the cell holds" rescale. Physical claim: NONE.
    pub clamp: f64,
    /// Gravity's work: the head-driven acceleration term (energy INTO the flux field).
    pub drive: f64,
    /// How many (cell, step) pairs the flux clamp actually fired on.
    pub clamp_fires: f64,
    /// Energy carried OUT of the domain by the transmissive outlet (PE of drained water).
    pub outflow: f64,
}

impl Sinks {
    pub fn add(&mut self, o: &Sinks) {
        self.theta += o.theta;
        self.friction += o.friction;
        self.breaking += o.breaking;
        self.clamp += o.clamp;
        self.drive += o.drive;
        self.clamp_fires += o.clamp_fires;
        self.outflow += o.outflow;
    }
}

macro_rules! define_kernel {
    ($modname:ident, $R:ty) => {
        pub mod $modname {
            use super::{Sinks, Smooth};

            pub type R = $R;
            pub const RHO: f64 = 1000.0;

            #[derive(Clone, Copy, Debug)]
            pub struct P {
                pub gravity: R,
                pub dt: R,
                pub manning_n: R,
                pub precip: R,
                pub evaporation: R,
                pub ocean_evap: R,
                pub sea_m: R,
                pub infiltration: R,
                pub gw_capacity: R,
                pub baseflow: R,
                pub seal_q: R,
                // --- probe knobs (water.rs has these baked in) ---
                pub theta: R,
                pub smooth: Smooth,
                pub breaking: bool,
                /// Jarrett slope-roughness term (`water.rs`: on). Off ⇒ plain Manning.
                pub jarrett: bool,
                /// Hold edge cells at the waterline (`water.rs`: on).
                pub edge_hold: bool,
                /// Transmissive (zero-gradient) OUTFLOW at the +x edge. Not in `water.rs`
                /// — added so a rain-fed slope can reach a real steady state instead of
                /// filling a closed box. It is a boundary condition, not a physics change.
                pub outflow_x: bool,
            }

            impl Default for P {
                /// `water.rs` defaults, minus the sediment block (unused: sed_capacity=0).
                fn default() -> Self {
                    Self {
                        gravity: 9.8,
                        dt: 0.2,
                        manning_n: 0.04,
                        precip: 3.0e-5,
                        evaporation: 2.0e-5,
                        ocean_evap: 1.0e-5,
                        sea_m: 4000.0,
                        infiltration: 2.0e-5,
                        gw_capacity: 0.3,
                        baseflow: 2.0e-5,
                        seal_q: 0.01,
                        theta: 0.8,
                        smooth: Smooth::GaussSeidel,
                        breaking: true,
                        jarrett: true,
                        edge_hold: true,
                        outflow_x: false,
                    }
                }
            }

            impl P {
                /// A CLOSED, CONSERVATIVE box: no rain, no evaporation, no infiltration,
                /// no sea boundary. Only momentum + continuity. This is the frame every
                /// energy probe runs in.
                pub fn closed() -> Self {
                    Self {
                        precip: 0.0,
                        evaporation: 0.0,
                        ocean_evap: 0.0,
                        infiltration: 0.0,
                        baseflow: 0.0,
                        sea_m: -1.0e9, // below every bed ⇒ hold_edge_sea is a no-op
                        ..Default::default()
                    }
                }
            }

            pub struct Sim {
                pub nx: usize,
                pub cell_m: R,
                pub bed: Vec<R>,
                pub depth: Vec<R>,
                pub groundwater: Vec<R>,
                pub atmosphere: f64,
                pub ocean: f64,
                pub fl: Vec<R>,
                pub fr: Vec<R>,
                pub ft: Vec<R>,
                pub fb: Vec<R>,
            }

            impl Sim {
                pub fn new(nx: usize, cell_m: R, bed: Vec<R>, depth: Vec<R>, atmosphere_m: f64) -> Self {
                    assert_eq!(bed.len(), nx * nx);
                    assert_eq!(depth.len(), nx * nx);
                    let z = vec![0.0 as R; nx * nx];
                    Self {
                        nx,
                        cell_m,
                        bed,
                        depth,
                        groundwater: z.clone(),
                        atmosphere: atmosphere_m * (nx * nx) as f64,
                        ocean: 0.0,
                        fl: z.clone(),
                        fr: z.clone(),
                        ft: z.clone(),
                        fb: z,
                    }
                }

                pub fn total_water(&self) -> f64 {
                    self.depth.iter().map(|&d| d as f64).sum::<f64>()
                        + self.groundwater.iter().map(|&g| g as f64).sum::<f64>()
                        + self.atmosphere
                        + self.ocean
                }

                #[inline]
                pub fn eta(&self, i: usize) -> R {
                    self.bed[i] + self.depth[i]
                }

                /// Conveyance depth over the sill between i and j (`water.rs`'s h_flow).
                #[inline]
                pub fn hflow(&self, i: usize, j: usize) -> R {
                    self.eta(i).max(self.eta(j)) - self.bed[i].max(self.bed[j])
                }

                /// Potential energy [J]: Σ ρ·g·A·d·(b + d/2).
                pub fn pe(&self, g: R) -> f64 {
                    let a = (self.cell_m as f64) * (self.cell_m as f64);
                    self.depth
                        .iter()
                        .zip(self.bed.iter())
                        .map(|(&d, &b)| {
                            let (d, b) = (d as f64, b as f64);
                            RHO * (g as f64) * a * d * (b + 0.5 * d)
                        })
                        .sum()
                }

                /// Kinetic energy of the pipe flux field [J]: ½·ρ·Σ f²/hflow.
                pub fn ke(&self) -> f64 {
                    let nx = self.nx;
                    let mut e = 0.0f64;
                    let mut acc = |f: R, i: usize, j: usize, s: &Self| {
                        if f == 0.0 {
                            return;
                        }
                        let h = s.hflow(i, j);
                        if h > 1e-4 {
                            e += 0.5 * RHO * (f as f64) * (f as f64) / (h as f64);
                        }
                    };
                    for y in 0..nx {
                        for x in 0..nx {
                            let i = y * nx + x;
                            if x > 0 {
                                acc(self.fl[i], i, i - 1, self);
                            }
                            if x < nx - 1 {
                                acc(self.fr[i], i, i + 1, self);
                            }
                            if y > 0 {
                                acc(self.ft[i], i, i - nx, self);
                            }
                            if y < nx - 1 {
                                acc(self.fb[i], i, i + nx, self);
                            }
                        }
                    }
                    e
                }

                pub fn energy(&self, g: R) -> f64 {
                    self.pe(g) + self.ke()
                }

                /// `water.rs::stable_dt`, verbatim: 0.3·cell/√(g·d_max), clamped [0.005, 0.2].
                pub fn stable_dt(&self, gravity: R) -> R {
                    let dmax = self.depth.iter().cloned().fold(0.1 as R, |a: R, b: R| a.max(b));
                    let v: R = 0.3 * self.cell_m / (gravity * dmax).sqrt();
                    v.max(0.005).min(0.2)
                }

                /// Cell-centred velocity reconstruction (`water.rs::to_region`, verbatim logic).
                pub fn velocity(&self) -> (Vec<R>, Vec<R>) {
                    let (nx, l) = (self.nx, self.cell_m);
                    let n = nx * nx;
                    let (mut vx, mut vy) = (vec![0.0 as R; n], vec![0.0 as R; n]);
                    for y in 0..nx {
                        for x in 0..nx {
                            let i = y * nx + x;
                            if self.depth[i] <= 1e-3 {
                                continue;
                            }
                            let vp = |f: R, j: usize, s: &Self| -> R {
                                let h = s.hflow(i, j);
                                if h > 1e-3 {
                                    f / (h * l)
                                } else {
                                    0.0
                                }
                            };
                            let r = if x < nx - 1 { vp(self.fr[i], i + 1, self) } else { 0.0 };
                            let lf = if x > 0 { vp(self.fl[i], i - 1, self) } else { 0.0 };
                            let b = if y < nx - 1 { vp(self.fb[i], i + nx, self) } else { 0.0 };
                            let t = if y > 0 { vp(self.ft[i], i - nx, self) } else { 0.0 };
                            vx[i] = r - lf;
                            vy[i] = b - t;
                        }
                    }
                    (vx, vy)
                }

                pub fn max_speed(&self) -> f64 {
                    let (vx, vy) = self.velocity();
                    vx.iter()
                        .zip(vy.iter())
                        .map(|(&a, &b)| ((a as f64).powi(2) + (b as f64).powi(2)).sqrt())
                        .fold(0.0f64, f64::max)
                }

                pub fn max_flux(&self) -> f64 {
                    let m = |v: &Vec<R>| v.iter().map(|&f| (f as f64).abs()).fold(0.0f64, f64::max);
                    m(&self.fl).max(m(&self.fr)).max(m(&self.ft)).max(m(&self.fb))
                }

                /// Spread of the water surface as the FLOAT TYPE SEES IT (max η − min η
                /// over wet cells). For a lake at rest this is the round-off floor.
                pub fn eta_spread(&self) -> f64 {
                    let mut lo = f64::INFINITY;
                    let mut hi = f64::NEG_INFINITY;
                    for i in 0..self.nx * self.nx {
                        if self.depth[i] > 1e-6 {
                            let e = self.eta(i) as f64;
                            lo = lo.min(e);
                            hi = hi.max(e);
                        }
                    }
                    if lo.is_finite() {
                        hi - lo
                    } else {
                        0.0
                    }
                }

                // ---- the θ smoothing pass, isolated so it can be measured ----
                fn smooth_pass(&mut self, p: &P) -> f64 {
                    if p.smooth == Smooth::Off || p.theta >= 1.0 {
                        return 0.0;
                    }
                    let ke0 = self.ke();
                    let nx = self.nx;
                    let th = p.theta;
                    match p.smooth {
                        // VERBATIM from water.rs: in-place, so fl[i-1] is already updated.
                        Smooth::GaussSeidel => {
                            for y in 0..nx {
                                for x in 1..nx - 1 {
                                    let i = y * nx + x;
                                    self.fl[i] = th * self.fl[i]
                                        + (1.0 - th) * 0.5 * (self.fl[i - 1] + self.fl[i + 1]);
                                    self.fr[i] = th * self.fr[i]
                                        + (1.0 - th) * 0.5 * (self.fr[i - 1] + self.fr[i + 1]);
                                }
                            }
                            for y in 1..nx - 1 {
                                for x in 0..nx {
                                    let i = y * nx + x;
                                    self.ft[i] = th * self.ft[i]
                                        + (1.0 - th) * 0.5 * (self.ft[i - nx] + self.ft[i + nx]);
                                    self.fb[i] = th * self.fb[i]
                                        + (1.0 - th) * 0.5 * (self.fb[i - nx] + self.fb[i + nx]);
                                }
                            }
                        }
                        // The symmetric operator: reads only the OLD field.
                        Smooth::Jacobi => {
                            let (ol, or_, ot, ob) =
                                (self.fl.clone(), self.fr.clone(), self.ft.clone(), self.fb.clone());
                            for y in 0..nx {
                                for x in 1..nx - 1 {
                                    let i = y * nx + x;
                                    self.fl[i] = th * ol[i] + (1.0 - th) * 0.5 * (ol[i - 1] + ol[i + 1]);
                                    self.fr[i] = th * or_[i] + (1.0 - th) * 0.5 * (or_[i - 1] + or_[i + 1]);
                                }
                            }
                            for y in 1..nx - 1 {
                                for x in 0..nx {
                                    let i = y * nx + x;
                                    self.ft[i] = th * ot[i] + (1.0 - th) * 0.5 * (ot[i - nx] + ot[i + nx]);
                                    self.fb[i] = th * ob[i] + (1.0 - th) * 0.5 * (ob[i - nx] + ob[i + nx]);
                                }
                            }
                        }
                        Smooth::Off => unreachable!(),
                    }
                    self.ke() - ke0
                }

                /// One step. Returns the per-stage energy accounting.
                pub fn step(&mut self, p: &P) -> Sinks {
                    let nx = self.nx;
                    let n = nx * nx;
                    let (l, dt) = (self.cell_m, p.dt);
                    let area = l * l;
                    let mut s = Sinks::default();

                    // 1. Rain (realized-delta accounting).
                    if p.precip > 0.0 && self.atmosphere > 0.0 {
                        let want = p.precip as f64 * dt as f64 * n as f64;
                        let scale = (self.atmosphere / want).min(1.0) as R;
                        let per_cell = p.precip * dt * scale;
                        let mut landed = 0.0f64;
                        for d in self.depth.iter_mut() {
                            let before = *d;
                            *d += per_cell;
                            landed += (*d - before) as f64;
                        }
                        self.atmosphere -= landed;
                    }
                    // 1b. Ocean → atmosphere.
                    if p.ocean_evap > 0.0 && self.ocean > 0.0 {
                        let lift = (p.ocean_evap as f64 * dt as f64 * n as f64).min(self.ocean);
                        self.ocean -= lift;
                        self.atmosphere += lift;
                    }
                    // 2. Edge sea hold.
                    if p.edge_hold {
                        self.hold_edge_sea(p.sea_m);
                    }

                    // 3a. θ flux smoothing — MEASURED.
                    s.theta = self.smooth_pass(p);

                    // 3b. pipe_step + flux clamp, with the sinks separated out.
                    let g = p.gravity;
                    let n_base = p.manning_n;
                    let jarrett = p.jarrett;
                    let breaking = p.breaking;
                    // (f, hflow, head) -> (f_driven, f_fricted, f_broken)
                    let pipe = |f: R, eta_i: R, eta_j: R, b_i: R, b_j: R| -> Option<(R, R, R, R)> {
                        let hflow = eta_i.max(eta_j) - b_i.max(b_j);
                        if hflow < 1e-4 {
                            return None;
                        }
                        let head = eta_i - eta_j;
                        let nn = if jarrett {
                            (n_base + 1.6 * (head.max(0.0) / l)).min(0.13)
                        } else {
                            n_base
                        };
                        let driven = (f + dt * g * hflow * head).max(0.0);
                        let v = driven / (hflow * l);
                        let fricted = driven / (1.0 + dt * g * nn * nn * v / hflow.powf(4.0 / 3.0));
                        let broken = if breaking {
                            fricted.min(2.0 * (g * hflow).sqrt() * hflow * l)
                        } else {
                            fricted
                        };
                        Some((driven, fricted, broken, hflow))
                    };
                    // KE bookkeeping helper: ½ρ(f₂² − f₁²)/h
                    let de = |f1: R, f2: R, h: R| -> f64 {
                        0.5 * RHO * ((f2 as f64).powi(2) - (f1 as f64).powi(2)) / (h as f64)
                    };

                    for y in 0..nx {
                        for x in 0..nx {
                            let i = y * nx + x;
                            let (bi, eta_i) = (self.bed[i], self.bed[i] + self.depth[i]);
                            let mut run = |f: R, j: usize, s: &mut Sinks, bed: &Vec<R>, dep: &Vec<R>| -> R {
                                match pipe(f, eta_i, bed[j] + dep[j], bi, bed[j]) {
                                    None => 0.0,
                                    Some((driven, fricted, broken, h)) => {
                                        s.drive += de(f, driven, h);
                                        s.friction += de(driven, fricted, h);
                                        s.breaking += de(fricted, broken, h);
                                        broken
                                    }
                                }
                            };
                            if x > 0 {
                                self.fl[i] = run(self.fl[i], i - 1, &mut s, &self.bed, &self.depth);
                            }
                            if x < nx - 1 {
                                self.fr[i] = run(self.fr[i], i + 1, &mut s, &self.bed, &self.depth);
                            }
                            if y > 0 {
                                self.ft[i] = run(self.ft[i], i - nx, &mut s, &self.bed, &self.depth);
                            }
                            if y < nx - 1 {
                                self.fb[i] = run(self.fb[i], i + nx, &mut s, &self.bed, &self.depth);
                            }
                            // Flux clamp — MEASURED.
                            let out = (self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i]) * dt;
                            if out > self.depth[i] * area {
                                let ke0 = 0.5 * RHO * self.pipe_ke_at(i);
                                let scale = self.depth[i] * area / out;
                                self.fl[i] *= scale;
                                self.fr[i] *= scale;
                                self.ft[i] *= scale;
                                self.fb[i] *= scale;
                                let ke1 = 0.5 * RHO * self.pipe_ke_at(i);
                                s.clamp += ke1 - ke0;
                                s.clamp_fires += 1.0;
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

                    // 4-BC. FREE OVERFALL at the +x edge (probe-only; see `outflow_x`).
                    // The outlet column discharges at the CRITICAL rate q = h·√(g·h)
                    // (Fr = 1 — the classical free-outfall condition). It is a PURE SINK:
                    // it can only ever remove water, never mint it. (An earlier
                    // zero-gradient copy `depth[last] = depth[prev]` was a SOURCE whenever
                    // depth[prev] > depth[last], and it flooded the 1%-slope run by 1558%.)
                    if p.outflow_x {
                        for y in 0..nx {
                            let i = y * nx + (nx - 1);
                            let h = self.depth[i];
                            if h > 1e-4 {
                                let q = h * (g * h).sqrt(); // m²/s
                                let dh = (q * dt / l).min(h);
                                self.depth[i] -= dh;
                                self.ocean += dh as f64;
                                s.outflow -= RHO * (g as f64) * (l as f64) * (l as f64)
                                    * (dh as f64) * (self.bed[i] as f64);
                            }
                        }
                    }

                    // 4c. Groundwater.
                    if p.infiltration > 0.0 {
                        for i in 0..n {
                            let d = self.depth[i];
                            if d > 0.0 && self.groundwater[i] < p.gw_capacity {
                                let out = self.fl[i] + self.fr[i] + self.ft[i] + self.fb[i];
                                let q = out / l;
                                let seal = 1.0 / (1.0 + q / p.seal_q);
                                let take = (p.infiltration * seal * dt)
                                    .min(d)
                                    .min(p.gw_capacity - self.groundwater[i]);
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
                                self.depth[i] = before;
                            }
                        }
                    }

                    // 5. Evaporation + re-hold.
                    if p.evaporation > 0.0 {
                        let e = p.evaporation * dt;
                        for d in self.depth.iter_mut() {
                            let before = *d;
                            *d = (*d - e.min(*d)).max(0.0);
                            self.atmosphere += (before - *d) as f64;
                        }
                    }
                    if p.edge_hold {
                        self.hold_edge_sea(p.sea_m);
                    }
                    s
                }

                fn pipe_ke_at(&self, i: usize) -> f64 {
                    let nx = self.nx;
                    let (x, y) = (i % nx, i / nx);
                    let mut e = 0.0f64;
                    let mut acc = |f: R, j: usize| {
                        let h = self.hflow(i, j);
                        if h > 1e-4 && f != 0.0 {
                            e += (f as f64).powi(2) / (h as f64);
                        }
                    };
                    if x > 0 {
                        acc(self.fl[i], i - 1);
                    }
                    if x < nx - 1 {
                        acc(self.fr[i], i + 1);
                    }
                    if y > 0 {
                        acc(self.ft[i], i - nx);
                    }
                    if y < nx - 1 {
                        acc(self.fb[i], i + nx);
                    }
                    e
                }

                fn hold_edge_sea(&mut self, sea: R) {
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

                /// Manning's ANALYTIC dissipation power [W]: ∫ τ_b·|v| dA with
                /// τ_b = ρ·g·n²·|v|²/h^{1/3}. The KNOWN sink the instrument must recover.
                pub fn analytic_friction_power(&self, p: &P) -> f64 {
                    let nx = self.nx;
                    let l = self.cell_m as f64;
                    let g = p.gravity as f64;
                    let nb = p.manning_n as f64;
                    let mut w = 0.0f64;
                    let mut acc = |f: R, i: usize, j: usize, s: &Self| {
                        if f <= 0.0 {
                            return;
                        }
                        let h = s.hflow(i, j) as f64;
                        if h <= 1e-4 {
                            return;
                        }
                        let v = (f as f64) / (h * l);
                        // per unit area: ρ g n² |v|³ / h^{1/3}; the pipe's area is l·l
                        w += RHO * g * nb * nb * v.abs().powi(3) / h.powf(1.0 / 3.0) * l * l;
                    };
                    for y in 0..nx {
                        for x in 0..nx {
                            let i = y * nx + x;
                            if x > 0 {
                                acc(self.fl[i], i, i - 1, self);
                            }
                            if x < nx - 1 {
                                acc(self.fr[i], i, i + 1, self);
                            }
                            if y > 0 {
                                acc(self.ft[i], i, i - nx, self);
                            }
                            if y < nx - 1 {
                                acc(self.fb[i], i, i + nx, self);
                            }
                        }
                    }
                    w
                }
            }
        }
    };
}

define_kernel!(k32, f32);
define_kernel!(k64, f64);
