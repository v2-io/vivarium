//! The virtual-pipes shallow-water operator (`water.rs`), as a transcription with
//! its knobs exposed — and a **pin** against the real kernel.
//!
//! ## Why a transcription at all
//!
//! `WaterSim`'s flux state (`fl/fr/ft/fb`) is **private**, so the full state
//! vector `(d, f)` cannot be perturbed from outside the crate — and a null-space
//! probe that cannot perturb the flux field cannot see a flux mode. We also need
//! `θ` as a variable, and in the kernel it is a hard-coded `let theta = 0.8f32`.
//!
//! ## The pin (this is what makes the transcription evidence rather than a story)
//!
//! [`pin_against_kernel`] runs the real `WaterSim::step` and this transcription
//! from the same initial condition for hundreds of steps, with every non-
//! hydrodynamic stage switched off (`sed_capacity = infiltration = evaporation =
//! precip = ocean_evap = 0`, sea level below the bed so the edge-hold is inert),
//! and reports the max depth discrepancy. If they do not track, nothing below is
//! worth reading.
//!
//! ## The state
//!
//! 5 DOF per cell: `[d, fl, fr, ft, fb]`. Note that the **face between `i` and
//! `i+1` carries TWO of them** — `fr[i]` and `fl[i+1]` — each with its own
//! momentum memory, each rectified at zero (`.max(0.0)`). That redundancy is not
//! incidental; §"opposed-pipe mode" in the report is about it.

/// Per-face bed drop `Δb = b_i − b_j`, in the cell's own frame. This is the only
/// thing the pipe needs from the bed (the kernel's `max(η) − max(b)` form is
/// algebraically translation-invariant), which is what lets a *tilted* channel be
/// analysed on a *periodic* patch — the classic roll-wave setting.
#[derive(Clone)]
pub enum Geom {
    /// Periodic patch with a constant bed drop `g` (m) per cell in +x and `gy` in
    /// +y. `g = gy = 0` is a flat bed. Cardiff Fig. 17's periodic patch, with a
    /// gravitational tilt.
    ///
    /// `gy` exists as a **control**: with a pure +x tilt the four transverse
    /// (y-axis) pipes sit at exactly zero head, which is precisely the rectifier's
    /// kink (`.max(0.0)` on the accelerated flux), so a finite-difference Jacobian
    /// straddles it. A small oblique tilt moves every pipe strictly off the kink.
    /// If the two answers agree, the kink did not matter — and that is a fact we
    /// measure rather than assume.
    Periodic { nx: usize, g: f64, gy: f64 },
    /// The real kernel's closed box with a real bed array (no-flux walls).
    ClosedBox { nx: usize, bed: Vec<f64> },
}

impl Geom {
    pub fn nx(&self) -> usize {
        match self {
            Geom::Periodic { nx, .. } | Geom::ClosedBox { nx, .. } => *nx,
        }
    }
    pub fn n(&self) -> usize {
        let nx = self.nx();
        nx * nx
    }
    /// Neighbour index and bed drop across the face in direction `d`
    /// (0 = -x/left, 1 = +x/right, 2 = -y/top, 3 = +y/bottom), or `None` at a wall.
    fn nbr(&self, i: usize, d: usize) -> Option<(usize, f64)> {
        let nx = self.nx();
        let (x, y) = (i % nx, i / nx);
        match self {
            Geom::Periodic { g, gy, .. } => {
                let (j, db) = match d {
                    0 => ((x + nx - 1) % nx + y * nx, -*g),
                    1 => ((x + 1) % nx + y * nx, *g),
                    2 => (x + ((y + nx - 1) % nx) * nx, -*gy),
                    _ => (x + ((y + 1) % nx) * nx, *gy),
                };
                Some((j, db))
            }
            Geom::ClosedBox { bed, .. } => {
                let j = match d {
                    0 => {
                        if x == 0 {
                            return None;
                        }
                        i - 1
                    }
                    1 => {
                        if x + 1 == nx {
                            return None;
                        }
                        i + 1
                    }
                    2 => {
                        if y == 0 {
                            return None;
                        }
                        i - nx
                    }
                    _ => {
                        if y + 1 == nx {
                            return None;
                        }
                        i + nx
                    }
                };
                Some((j, bed[i] - bed[j]))
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct PipeParams {
    pub g: f64,
    pub dt: f64,
    pub l: f64,
    pub manning_n: f64,
    /// The de Almeida–Bates flux-smoothing gain. `1.0` = OFF (no smoothing).
    /// The kernel hard-codes `0.8`.
    pub theta: f64,
    /// Jarrett slope-roughness term. The kernel hard-codes the `1.6` coefficient
    /// and the `0.13` cap; exposed here only so it can be switched off to isolate
    /// what each term is doing.
    pub jarrett: bool,
    /// The Froude-2 breaking cap. The kernel always applies it.
    pub breaking_cap: bool,
    /// The outflow clamp (a cell cannot ship more than it holds).
    pub outflow_clamp: bool,
    /// The kernel sweeps the θ-smoother IN PLACE (Gauss–Seidel), so it reads an
    /// already-updated left neighbour and a stale right one — the filter it
    /// actually applies is **asymmetric**, not the symmetric one de Almeida writes
    /// down. `true` reproduces the kernel. `false` is the out-of-place (Jacobi)
    /// filter, which is exactly translation-invariant and therefore lets the
    /// operator be diagonalised by wavenumber. We check the two give the same
    /// spectral radius before trusting the wavenumber map.
    pub gauss_seidel: bool,
}

impl PipeParams {
    /// The kernel's own defaults at the level `water.rs` is documented for
    /// (L21, ~4.8 m cells): `WaterParams::default()` + the hard-coded `θ = 0.8`.
    pub fn kernel_default(l: f64) -> Self {
        Self {
            g: 9.8,
            dt: 0.2,
            l,
            manning_n: 0.04,
            theta: 0.8,
            jarrett: true,
            breaking_cap: true,
            outflow_clamp: true,
            gauss_seidel: true,
        }
    }
}

/// Diagnostics recorded while stepping — which of the kernel's NON-SMOOTH guards
/// actually fired. A Jacobian taken where a guard is active is a secant across a
/// kink, and we must know that rather than assume it.
#[derive(Default, Debug, Clone, Copy)]
pub struct Guards {
    pub rectifier_active: usize, // .max(0.0) fired — the pipe is strictly clipped
    /// ⚠ The one that can INVALIDATE a Jacobian: `|raw| ≤ kink_band`, i.e. the
    /// pipe sits ON the rectifier's kink, so a ±ε finite difference straddles it
    /// and returns half the one-sided slope. A base state with any of these is a
    /// base state where the linearisation is a secant across a corner.
    pub near_kink: usize,
    pub dry_sill: usize, // hflow < 1e-4 ⇒ pipe forced to zero
    pub breaking: usize, // Froude-2 cap bit
    pub clamped: usize,  // outflow clamp bit
    pub pipes: usize,
}

/// Half-width of the band around `raw = 0` counted as "on the kink". Sized to the
/// probe's own perturbation scale (see `main`), so it answers the question that
/// matters: *could my ±ε have crossed it?*
pub const KINK_BAND: f64 = 1e-5;

/// One hydrodynamic step of the virtual-pipes kernel: θ-smooth → accelerate →
/// friction → break → clamp → depth. Transcribed from `water.rs::step` stages
/// 3 and 4 (all other stages are switched off by the params in the pin).
pub fn step(state: &mut [f64], geom: &Geom, p: &PipeParams, guards: &mut Guards) {
    let nx = geom.nx();
    let n = geom.n();
    let area = p.l * p.l;
    // Layout: [d, fl, fr, ft, fb] per cell.
    let d = |s: &[f64], i: usize| s[5 * i];
    let f = |s: &[f64], i: usize, k: usize| s[5 * i + 1 + k]; // k: 0=fl,1=fr,2=ft,3=fb

    // ── θ flux smoothing (de Almeida et al. 2012). In-place Gauss–Seidel sweep,
    //    exactly as the kernel does it (which makes the smoother slightly
    //    ASYMMETRIC — it is not the symmetric filter the paper writes down).
    let th = p.theta;
    if th < 1.0 {
        let stale = state.to_vec(); // the Jacobi (out-of-place) source
        let src = |s: &[f64], i: usize, k: usize| if p.gauss_seidel { f(s, i, k) } else { f(&stale, i, k) };
        // x-axis pipes (fl, fr) smoothed along x.
        let (lo, hi) = match geom {
            Geom::Periodic { .. } => (0usize, nx),
            Geom::ClosedBox { .. } => (1usize, nx - 1),
        };
        for y in 0..nx {
            for x in lo..hi {
                let i = y * nx + x;
                for k in [0usize, 1] {
                    let im = y * nx + (x + nx - 1) % nx;
                    let ip = y * nx + (x + 1) % nx;
                    let v = th * src(state, i, k) + (1.0 - th) * 0.5 * (src(state, im, k) + src(state, ip, k));
                    state[5 * i + 1 + k] = v;
                }
            }
        }
        // y-axis pipes (ft, fb) smoothed along y.
        for y in lo..hi {
            for x in 0..nx {
                let i = y * nx + x;
                for k in [2usize, 3] {
                    let im = ((y + nx - 1) % nx) * nx + x;
                    let ip = ((y + 1) % nx) * nx + x;
                    let v = th * src(state, i, k) + (1.0 - th) * 0.5 * (src(state, im, k) + src(state, ip, k));
                    state[5 * i + 1 + k] = v;
                }
            }
        }
    }

    // ── Accelerate + friction + break, per pipe.
    let mut newf = vec![0.0f64; 4 * n];
    for i in 0..n {
        for k in 0..4 {
            let Some((j, db)) = geom.nbr(i, k) else {
                newf[4 * i + k] = f(state, i, k); // wall: kernel leaves it untouched (it is 0)
                continue;
            };
            let di = d(state, i);
            let dj = d(state, j);
            // hflow = max(η_i, η_j) − max(b_i, b_j), in the face-local frame.
            let hflow = di.max(dj - db) - 0.0f64.max(-db);
            guards.pipes += 1;
            if hflow < 1e-4 {
                guards.dry_sill += 1;
                newf[4 * i + k] = 0.0;
                continue;
            }
            let head = (di - dj) + db; // η_i − η_j
            let n_manning =
                if p.jarrett { (p.manning_n + 1.6 * (head.max(0.0) / p.l)).min(0.13) } else { p.manning_n };
            let raw = f(state, i, k) + p.dt * p.g * hflow * head;
            if raw < 0.0 {
                guards.rectifier_active += 1;
            }
            if raw.abs() <= KINK_BAND {
                guards.near_kink += 1;
            }
            let accel = raw.max(0.0);
            let v = accel / (hflow * p.l);
            let mut ff = accel / (1.0 + p.dt * p.g * n_manning * n_manning * v / hflow.powf(4.0 / 3.0));
            if p.breaking_cap {
                let cap = 2.0 * (p.g * hflow).sqrt() * hflow * p.l;
                if ff > cap {
                    guards.breaking += 1;
                    ff = cap;
                }
            }
            newf[4 * i + k] = ff;
        }
    }
    for i in 0..n {
        for k in 0..4 {
            state[5 * i + 1 + k] = newf[4 * i + k];
        }
    }

    // ── Outflow clamp.
    if p.outflow_clamp {
        for i in 0..n {
            let out = (0..4).map(|k| f(state, i, k)).sum::<f64>() * p.dt;
            let cap = d(state, i) * area;
            if out > cap && out > 0.0 {
                guards.clamped += 1;
                let s = cap / out;
                for k in 0..4 {
                    state[5 * i + 1 + k] *= s;
                }
            }
        }
    }

    // ── Depth update from net flux.
    let snap: Vec<f64> = state.to_vec();
    for i in 0..n {
        let mut inflow = 0.0;
        // Neighbour j pushing INTO i is j's pipe pointing back at i: opposite dir.
        for k in 0..4 {
            if let Some((j, _)) = geom.nbr(i, k) {
                let opp = match k {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    _ => 2,
                };
                inflow += f(&snap, j, opp);
            }
        }
        let outflow: f64 = (0..4).map(|k| f(&snap, i, k)).sum();
        state[5 * i] = (d(&snap, i) + (inflow - outflow) * p.dt / area).max(0.0);
    }
}

/// A base state: uniform depth `d0`, and the pipe fluxes relaxed to their own
/// steady value by running the operator forward. For a tilted periodic channel
/// this converges to **steady uniform flow** — the state the solitons appeared
/// on, and the only base state on which the roll-wave question is even posed.
pub fn relax_to_steady(geom: &Geom, p: &PipeParams, d0: f64, steps: usize) -> (Vec<f64>, Guards) {
    let n = geom.n();
    let mut s = vec![0.0f64; 5 * n];
    for i in 0..n {
        s[5 * i] = d0;
    }
    let mut g = Guards::default();
    for _ in 0..steps {
        // Hold the depth uniform: we want the FLUX equilibrium at fixed depth,
        // which is exactly the normal-flow (Manning) state. Depth is then a
        // conserved uniform field and the base state is a genuine steady state of
        // the periodic problem (mass in = mass out at every cell, identically).
        let mut gg = Guards::default();
        step(&mut s, geom, p, &mut gg);
        for i in 0..n {
            s[5 * i] = d0;
        }
        g = gg;
    }
    (s, g)
}

/// Froude number of a uniform-flow base state (`|v| / √(g·d)`) — the number that
/// says whether we are in the regime the hydrology work was fighting.
pub fn froude(state: &[f64], geom: &Geom, p: &PipeParams) -> f64 {
    let n = geom.n();
    let mut m = 0.0f64;
    for i in 0..n {
        let d = state[5 * i];
        if d < 1e-4 {
            continue;
        }
        let fx = state[5 * i + 2]; // fr
        let v = fx / (d * p.l);
        m = m.max(v / (p.g * d).sqrt());
    }
    m
}

// ─────────────────────────────────────────────────────────────────────────────
// THE PIN
// ─────────────────────────────────────────────────────────────────────────────

/// Run the REAL `WaterSim::step` and this transcription side by side and return
/// `(max |Δdepth|, mean depth, steps)`. Every non-hydrodynamic stage is off, so
/// the kernel executes exactly the code path transcribed above.
pub fn pin_against_kernel(nx: usize, steps: usize) -> (f64, f64) {
    use vivarium_world::sphere::Face;
    use vivarium_world::water::{WaterParams, WaterSim};

    let l = 4.8f32;
    // Low datum: keeps f32 cancellation in `max(η) − max(b)` out of the pin, so
    // the pin measures TRANSCRIPTION fidelity and nothing else. (The kernel's
    // behaviour at a 4000 m datum is a separate finding — see the report.)
    let slope = 0.05f32;
    let bed: Vec<f32> = (0..nx * nx).map(|i| 40.0 - slope * l * (i % nx) as f32).collect();

    let mut p = WaterParams::default();
    p.sed_capacity = 0.0;
    p.infiltration = 0.0;
    p.evaporation = 0.0;
    p.precip = 0.0;
    p.ocean_evap = 0.0;
    p.baseflow = 0.0;
    p.sea_m = -1.0e9; // no cell is at/below sea ⇒ hold_edge_sea is inert
    p.dt = 0.2;

    let mut sim = WaterSim::new(Face::ZPos, 21, (0, 0), nx, l, bed.clone(), 0.0);
    // A non-trivial, non-symmetric initial depth field: this must EXERCISE the
    // scheme (rectifier, breaking cap, clamp), not sit at a fixed point.
    for i in 0..nx * nx {
        let (x, y) = ((i % nx) as f64, (i / nx) as f64);
        sim.depth[i] = (0.5 + 0.4 * (0.7 * x).sin() * (1.1 * y).cos() + 0.2 * ((x * 3.0 + y * 5.0) % 1.0)) as f32;
    }
    let d_init: Vec<f64> = sim.depth.iter().map(|&d| d as f64).collect();

    let geom = Geom::ClosedBox { nx, bed: bed.iter().map(|&b| b as f64).collect() };
    let pp = PipeParams::kernel_default(l as f64);
    let mut st = vec![0.0f64; 5 * nx * nx];
    for i in 0..nx * nx {
        st[5 * i] = d_init[i];
    }

    let mut worst = 0.0f64;
    let mut g = Guards::default();
    for _ in 0..steps {
        sim.step(&p);
        step(&mut st, &geom, &pp, &mut g);
        for i in 0..nx * nx {
            worst = worst.max((sim.depth[i] as f64 - st[5 * i]).abs());
        }
    }
    let mean: f64 = st.iter().step_by(5).sum::<f64>() / (nx * nx) as f64;
    (worst, mean)
}
