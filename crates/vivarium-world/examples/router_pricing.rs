//! # Routing pricing experiment — does the corrected router change the landscape?
//!
//! Prices the severity quarantined in
//! `DECISIONS[routing-violates-the-potential-identity-and-the-replacement-does-not-fix-it]`
//! (`#obs-routing-curl-spiral` FE(8)): run an eroded landscape **with and without the
//! corrected drainage router**, identical in everything else, and report whether the
//! channel network differs. A genuine "no meaningful difference" is a first-class result.
//!
//! Predictions were written first: `.super-archive/from-msc/spike-router-pricing/PREDICTIONS.md`.
//!
//! ## What is faithful and what is swapped
//!
//! The whole fluvial pipeline is reimplemented here **verbatim from `erosion.rs`** — the
//! only thing that varies between arms is the drainage accumulation step
//! (`accumulate_drainage`, "what decides WHERE channels form"). Arm `LiveMfd` must
//! reproduce the live `erosion::Fluvial::erode` final `h`/`drainage` **bit-for-bit**
//! (the P0 anchor, checked first) — otherwise nothing here is about the live world.
//!
//! Arms:
//! * `LiveMfd`   — 8-nbr Moore fan, uniform lengths `cell_m`/`√2·cell_m`, `(drop/dist)^1`.
//!                 == `erosion.rs` today.
//! * `TrueLen`   — same 8-nbr fan, but **true** great-circle neighbour lengths
//!                 (`measure::neighbor_center_dist_m`). The *length* half of the cube-lock.
//! * `EdgeTrue`  — **kill the diagonals** (they cross no face: ~47.8% phantom flux) +
//!                 true edge lengths. The named topological correction.
//!
//! Run:  `cargo run --release -p vivarium-world --example router_pricing`

use vivarium_world::erosion::{Fluvial, FluvialParams};
use vivarium_world::planet::Planet;
use vivarium_world::sphere::{CellId, Face};
use vivarium_world::{gen, measure, sample, sea_level};

const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Router {
    /// Retired kernel: 8-nbr fan, UNIFORM lengths (cell_m / √2·cell_m) everywhere.
    /// == erosion.rs BEFORE commit 6c1ad97. Historical control.
    UniformOld,
    /// Current live kernel: 8-nbr fan, TRUE great-circle distances everywhere.
    /// == erosion.rs today (post 6c1ad97). The pricing baseline; must bit-match live.
    LiveMfd,
    /// Correction under test (FAN HALF): current live, but the drainage fan is
    /// EDGE-ONLY (4 face-sharing neighbours — diagonals cross no face, ~47.8%
    /// phantom flux). Receivers/incise/talus identical to LiveMfd, true dist.
    EdgeTrue,
    /// Correction under test (FULL FE(6b) FLUX SURFACE): diagonals killed in BOTH
    /// the drainage fan AND the D8 receiver/incision tree (the tree becomes D4
    /// steepest-descent over the four face-sharing neighbours). Talus (hillslope
    /// mass movement, not channelised flux) and depression-fill connectivity stay
    /// 8-way. This is the surface a "router successor" adjudication must price.
    EdgeFull,
}

impl Router {
    #[allow(dead_code)]
    fn label(self) -> &'static str {
        match self {
            Router::UniformOld => "UniformOld (8-nbr, uniform len)  [pre-6c1ad97]",
            Router::LiveMfd => "LiveMfd    (8-nbr, true gc dist)  [== erosion.rs today]",
            Router::EdgeTrue => "EdgeTrue   (4-nbr fan, 8-nbr tree)  [kill diagonals: fan only]",
            Router::EdgeFull => "EdgeFull   (4-nbr fan + 4-nbr tree) [kill diagonals: full FE(6b)]",
        }
    }
    /// Neighbour offsets consulted in the DRAINAGE fan (4 = edge-only).
    fn fan_n_nbr(self) -> usize {
        match self {
            Router::EdgeTrue | Router::EdgeFull => 4,
            _ => 8,
        }
    }
    /// Neighbour offsets for the D8 receiver (incision) tree (4 = edge-only D4).
    fn recv_n_nbr(self) -> usize {
        match self {
            Router::EdgeFull => 4,
            _ => 8,
        }
    }
    /// Whether every distance (receivers/fan/incise/talus) is uniform (old kernel).
    fn uniform_dist(self) -> bool {
        matches!(self, Router::UniformOld)
    }
}

/// The pipeline — a verbatim port of `erosion::Fluvial`, with `router` consulted
/// only inside `accumulate_drainage`. Fields/methods mirror the live kernel so the
/// `LiveMfd` arm can be diffed bit-for-bit against it.
struct Pipe {
    nx: usize,
    cell_m: f32,
    cell_area: Vec<f32>,
    /// Unit direction of each cell centre — the source for true gc distances
    /// (`dist_m`), exactly as the live kernel's `centers` field.
    centers: Vec<[f64; 3]>,
    h: Vec<f32>,
    drainage: Vec<f32>,
    uplift_rate: Vec<f32>,
    precip_weight: Vec<f32>,
    router: Router,
}

impl Pipe {
    fn from_surface(
        face: Face,
        level: u8,
        oi: u32,
        oj: u32,
        nx: usize,
        router: Router,
        surf: impl Fn(CellId) -> f64,
    ) -> Self {
        let radius = Planet::EARTH.radius_m;
        let cell_m = sample::cell_size_m(level, radius) as f32;
        let mut h = vec![0.0f32; nx * nx];
        let mut cell_area = vec![0.0f32; nx * nx];
        let mut centers = vec![[0.0f64; 3]; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let gi = oi + x as u32;
                let gj = oj + y as u32;
                let cell = CellId::from_face_ij(face, gi, gj, level);
                h[y * nx + x] = surf(cell) as f32;
                cell_area[y * nx + x] =
                    measure::cell_area_m2(face, gi as u64, gj as u64, level, radius) as f32;
                centers[y * nx + x] = measure::cell_center_unit(face, gi as u64, gj as u64, level);
            }
        }
        Self {
            nx,
            cell_m,
            cell_area,
            centers,
            h,
            drainage: vec![0.0; nx * nx],
            uplift_rate: vec![0.0; nx * nx],
            precip_weight: vec![1.0; nx * nx],
            router,
        }
    }

    fn set_uniform_uplift(&mut self, rate: f32) {
        self.uplift_rate = vec![rate; self.nx * self.nx];
    }

    #[inline]
    fn is_edge(nx: usize, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == nx - 1 || y == nx - 1
    }

    /// True great-circle distance between cell centres — mirrors the live kernel's
    /// `dist_m`. UniformOld arm overrides this with axis/diagonal uniform lengths.
    #[inline]
    fn dist_m(&self, a: usize, b: usize) -> f32 {
        if self.router.uniform_dist() {
            let nx = self.nx;
            let (ax, ay) = (a % nx, a / nx);
            let (bx, by) = (b % nx, b / nx);
            if ax != bx && ay != by {
                self.cell_m * std::f32::consts::SQRT_2
            } else {
                self.cell_m
            }
        } else {
            measure::gc_dist_m(self.centers[a], self.centers[b], Planet::EARTH.radius_m) as f32
        }
    }

    // Partial tile (never full-face here): edge sinks + coast. Matches erosion.rs.
    fn outlets(&self, sea: f32) -> Vec<bool> {
        let nx = self.nx;
        let mut out = vec![false; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                out[i] = Self::is_edge(nx, x, y) || self.h[i] <= sea;
            }
        }
        out
    }

    fn fill_depressions(&mut self, outlets: &[bool]) {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;
        let nx = self.nx;
        const EPS: f32 = 1e-3;
        struct Cell {
            elev: f32,
            seq: u64,
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
                o.elev.total_cmp(&self.elev).then_with(|| o.seq.cmp(&self.seq))
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
        for (i, &is_out) in outlets.iter().enumerate() {
            if is_out {
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
                self.h[j] = self.h[j].max(elev + EPS);
                heap.push(Cell { elev: self.h[j], seq, i: j });
                seq += 1;
            }
        }
    }

    fn receivers(&self, outlets: &[bool]) -> Vec<usize> {
        let nx = self.nx;
        let nxi = nx as i32;
        let mut recv = vec![0usize; nx * nx];
        for y in 0..nx {
            for x in 0..nx {
                let i = y * nx + x;
                if outlets[i] {
                    recv[i] = i;
                    continue;
                }
                let hi = self.h[i];
                let (mut best, mut best_slope) = (i, 0.0f32);
                for k in 0..self.router.recv_n_nbr() {
                    let (dx, dy) = NEIGHBORS[k];
                    let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                    if nx_ < 0 || ny_ < 0 || nx_ >= nxi || ny_ >= nxi {
                        continue;
                    }
                    let j = ny_ as usize * nx + nx_ as usize;
                    let dist = self.dist_m(i, j);
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

    fn elevation_order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.h.len()).collect();
        order.sort_by(|&a, &b| self.h[a].total_cmp(&self.h[b]).then_with(|| a.cmp(&b)));
        order
    }

    /// THE SWAPPED STEP. `P=1.0` verbatim (`powf`, not a bare divide — bit-match).
    /// LiveMfd: 8-nbr, uniform lengths. TrueLen: 8-nbr, true lengths. EdgeTrue: 4-nbr,
    /// true lengths (diagonals dropped — they cross no face).
    fn accumulate_drainage(&mut self, order: &[usize]) {
        const P: f32 = 1.0;
        let nx = self.nx;
        let n_nbr = self.router.fan_n_nbr();
        for i in 0..self.drainage.len() {
            self.drainage[i] = self.cell_area[i] * self.precip_weight[i];
        }
        for &i in order.iter().rev() {
            let (x, y) = (i % nx, i / nx);
            let hi = self.h[i];
            let mut weights = [0.0f32; 8];
            let mut total = 0.0f32;
            for k in 0..n_nbr {
                let (dx, dy) = NEIGHBORS[k];
                let (nxp, nyp) = (x as i32 + dx, y as i32 + dy);
                if nxp < 0 || nyp < 0 || nxp >= nx as i32 || nyp >= nx as i32 {
                    continue;
                }
                let j = nyp as usize * nx + nxp as usize;
                let drop = hi - self.h[j];
                if drop > 0.0 {
                    let dist = self.dist_m(i, j);
                    let w = (drop / dist).powf(P);
                    weights[k] = w;
                    total += w;
                }
            }
            if total > 0.0 {
                let amount = self.drainage[i];
                for k in 0..n_nbr {
                    if weights[k] > 0.0 {
                        let (dx, dy) = NEIGHBORS[k];
                        let j = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                        self.drainage[j] += amount * (weights[k] / total);
                    }
                }
            }
        }
    }

    fn incise(&mut self, p: &FluvialParams, recv: &[usize], order: &[usize]) {
        for &i in order {
            let r = recv[i];
            if r == i {
                continue;
            }
            let dist = self.dist_m(i, r);
            let f = p.k_dt * self.drainage[i].powf(p.m) / dist;
            self.h[i] = (self.h[i] + f * self.h[r]) / (1.0 + f);
        }
    }

    fn deposit(&mut self, p: &FluvialParams, recv: &[usize], order: &[usize], before: &[f32]) {
        let n = self.nx * self.nx;
        let mut qs = vec![0.0f32; n];
        for i in 0..n {
            let eroded = before[i] - self.h[i];
            if eroded > 0.0 {
                qs[i] = eroded * self.cell_area[i];
            }
        }
        for &i in order.iter().rev() {
            let area = self.cell_area[i];
            let a = self.drainage[i].max(area);
            let deposit_h = p.deposition * qs[i] / a;
            let deposit_vol = (deposit_h * area).min(qs[i]);
            self.h[i] += deposit_vol / area;
            qs[i] -= deposit_vol;
            let r = recv[i];
            if r != i {
                qs[r] += qs[i];
            }
        }
    }

    fn talus(&mut self, p: &FluvialParams) {
        let nx = self.nx;
        let snapshot = self.h.clone();
        let mut delta = vec![0.0f32; nx * nx];
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                let i = y * nx + x;
                let hi = snapshot[i];
                let (mut best, mut best_drop, mut best_dist) = (i, 0.0f32, self.cell_m);
                for (dx, dy) in NEIGHBORS {
                    let i2 = (y as i32 + dy) as usize * nx + (x as i32 + dx) as usize;
                    let j = i2;
                    let dist = self.dist_m(i, j);
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
                let excess = best_drop - p.max_slope * best_dist;
                if excess > 0.0 {
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

    fn creep(&mut self, p: &FluvialParams) {
        let k = (p.diffusivity_m2 / (self.cell_m * self.cell_m)).min(0.24);
        if k < 1e-5 {
            return;
        }
        let nx = self.nx;
        let snapshot = self.h.clone();
        for y in 1..nx - 1 {
            for x in 1..nx - 1 {
                let i = y * nx + x;
                let lap = snapshot[i - 1] + snapshot[i + 1] + snapshot[i - nx] + snapshot[i + nx]
                    - 4.0 * snapshot[i];
                self.h[i] += k * lap;
            }
        }
    }

    fn erode(&mut self, p: &FluvialParams, sea: f32) {
        let nx = self.nx;
        for _e in 0..p.epochs {
            let outlets = self.outlets(sea);
            for i in 0..nx * nx {
                let rate = self.uplift_rate[i];
                if rate != 0.0 {
                    let (x, y) = (i % nx, i / nx);
                    if !Self::is_edge(nx, x, y) {
                        self.h[i] += rate;
                    }
                }
            }
            self.fill_depressions(&outlets);
            let recv = self.receivers(&outlets);
            let order = self.elevation_order();
            self.accumulate_drainage(&order);
            let before = if p.deposition > 0.0 { Some(self.h.clone()) } else { None };
            self.incise(p, &recv, &order);
            if let Some(b) = before {
                self.deposit(p, &recv, &order, &b);
            }
            self.talus(p);
            self.creep(p);
        }
    }
}

// ------------------------------- terrain -------------------------------------

/// Off-centre asymmetric terrain: a broad dome (guarantees subaerial relief and
/// natural radial dendritic channels) + real band-limited prior detail (natural
/// channel seeding). The FOOTPRINT position on the face — not the dome shape — is
/// what breaks the null-test D4 symmetry: an off-corner footprint sits in the
/// sheared equiangular Jacobian where the router defect actually manifests.
fn terrain(
    seed: u64,
    face: Face,
    level: u8,
    oi: u32,
    oj: u32,
    nx: usize,
    dome_m: f64,
) -> impl Fn(CellId) -> f64 {
    let sea = sea_level::derived_sea_level_m(seed);
    move |c: CellId| -> f64 {
        let (_, i, j, _) = c.to_face_ij();
        let di = i as f64 - (oi as f64 + nx as f64 / 2.0);
        let dj = j as f64 - (oj as f64 + nx as f64 / 2.0);
        let r2 = (di * di + dj * dj) / ((nx as f64 / 2.0).powi(2));
        // Slightly anisotropic paraboloid so it is not perfectly radial either.
        let dome = dome_m * (1.0 - 0.85 * r2).max(-0.3);
        // Real prior detail (high-frequency dendritic seeding), scaled modestly.
        let detail = gen::initial_topography_m(seed, c, c.level()) - gen::initial_topography_m(seed, c, level);
        sea + dome + 0.4 * detail
    }
}

// ------------------------------- metrics -------------------------------------

/// Channel mask: cell whose drainage exceeds `tau * local cell area` and is land.
fn channel_mask(drainage: &[f32], cell_area: &[f32], h: &[f32], sea: f32, tau: f32) -> Vec<bool> {
    drainage
        .iter()
        .zip(cell_area)
        .zip(h)
        .map(|((&d, &a), &hh)| hh > sea && d > tau * a)
        .collect()
}

/// M1: Jaccard disagreement (symmetric difference / union) of two channel masks.
fn jaccard_disagreement(a: &[bool], b: &[bool]) -> f64 {
    let mut inter = 0u64;
    let mut union = 0u64;
    for (&x, &y) in a.iter().zip(b) {
        if x || y {
            union += 1;
            if x && y {
                inter += 1;
            }
        }
    }
    if union == 0 {
        return 0.0;
    }
    1.0 - inter as f64 / union as f64
}

/// M2: Spearman rank correlation of log-drainage over land cells, and relative L2.
fn drainage_divergence(da: &[f32], db: &[f32], h: &[f32], sea: f32) -> (f64, f64) {
    let idx: Vec<usize> = (0..h.len()).filter(|&i| h[i] > sea).collect();
    if idx.len() < 3 {
        return (f64::NAN, f64::NAN);
    }
    let la: Vec<f64> = idx.iter().map(|&i| (da[i].max(1.0) as f64).ln()).collect();
    let lb: Vec<f64> = idx.iter().map(|&i| (db[i].max(1.0) as f64).ln()).collect();
    // Spearman = Pearson on ranks.
    let ra = ranks(&la);
    let rb = ranks(&lb);
    let spearman = pearson(&ra, &rb);
    // relative L2 on log-drainage
    let mut num = 0.0;
    let mut den = 0.0;
    for (x, y) in la.iter().zip(&lb) {
        num += (x - y) * (x - y);
        den += x * x;
    }
    (spearman, (num / den.max(1e-12)).sqrt())
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    for (rank, &i) in idx.iter().enumerate() {
        r[i] = rank as f64;
    }
    r
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    let mut cov = 0.0;
    let mut va = 0.0;
    let mut vb = 0.0;
    for (x, y) in a.iter().zip(b) {
        cov += (x - ma) * (y - mb);
        va += (x - ma) * (x - ma);
        vb += (y - mb) * (y - mb);
    }
    cov / (va.sqrt() * vb.sqrt()).max(1e-12)
}

/// M3: channel-orientation anisotropy. For channel cells, the D8 steepest-descent
/// step direction is binned by axis-alignment. Returns (axis_fraction, diag_fraction)
/// where axis = step is cardinal (N/S/E/W), diag = step is diagonal. On a truly
/// isotropic network the expected axis:diag split reflects the 4:4 offset count
/// weighted by geometry; the DIFFERENCE between arms is what the bias predicts.
fn orientation_split(mask: &[bool], h: &[f32], nx: usize) -> (f64, f64) {
    let mut axis = 0u64;
    let mut diag = 0u64;
    for y in 0..nx {
        for x in 0..nx {
            let i = y * nx + x;
            if !mask[i] {
                continue;
            }
            // steepest-descent neighbour direction (uniform metric — pure azimuth)
            let hi = h[i];
            let mut best_drop = 0.0f32;
            let mut best_is_diag = None;
            for (dx, dy) in NEIGHBORS {
                let (nx_, ny_) = (x as i32 + dx, y as i32 + dy);
                if nx_ < 0 || ny_ < 0 || nx_ >= nx as i32 || ny_ >= nx as i32 {
                    continue;
                }
                let j = ny_ as usize * nx + nx_ as usize;
                let d = if dx != 0 && dy != 0 { std::f32::consts::SQRT_2 } else { 1.0 };
                let drop = (hi - h[j]) / d;
                if drop > best_drop {
                    best_drop = drop;
                    best_is_diag = Some(dx != 0 && dy != 0);
                }
            }
            match best_is_diag {
                Some(true) => diag += 1,
                Some(false) => axis += 1,
                None => {}
            }
        }
    }
    let tot = (axis + diag).max(1) as f64;
    (axis as f64 / tot, diag as f64 / tot)
}

// ------------------------------- driver --------------------------------------

fn run_arm(
    router: Router,
    seed: u64,
    face: Face,
    level: u8,
    oi: u32,
    oj: u32,
    nx: usize,
    dome_m: f64,
    uplift: f32,
    p: &FluvialParams,
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    let surf = terrain(seed, face, level, oi, oj, nx, dome_m);
    let mut pipe = Pipe::from_surface(face, level, oi, oj, nx, router, &surf);
    if uplift != 0.0 {
        pipe.set_uniform_uplift(uplift);
    }
    pipe.erode(p, sea);
    (pipe.h.clone(), pipe.drainage.clone(), pipe.cell_area.clone())
}

/// P0: the LiveMfd arm reproduces the live `erosion::Fluvial::erode` bit-for-bit.
fn validate_bitmatch(seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize, dome_m: f64, uplift: f32, p: &FluvialParams) -> bool {
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    let surf = terrain(seed, face, level, oi, oj, nx, dome_m);
    // Live kernel:
    let mut live = Fluvial::from_surface(seed, face, level, oi, oj, nx, &surf);
    if uplift != 0.0 {
        live.set_uniform_uplift(uplift);
    }
    live.erode(p);
    // Harness LiveMfd arm:
    let mut mine = Pipe::from_surface(face, level, oi, oj, nx, Router::LiveMfd, &surf);
    if uplift != 0.0 {
        mine.set_uniform_uplift(uplift);
    }
    mine.erode(p, sea);
    let h_ok = live.h == mine.h;
    let d_ok = live.drainage == mine.drainage;
    if !h_ok || !d_ok {
        let hd = live.h.iter().zip(&mine.h).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
        let dd = live.drainage.iter().zip(&mine.drainage).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
        eprintln!("  BITMATCH FAIL: max|Δh|={hd:.6e}  max|Δdrainage|={dd:.6e}  (h_ok={h_ok} d_ok={d_ok})");
    }
    h_ok && d_ok
}

/// Channel-orientation axis-fraction for one arm at one site/threshold.
#[allow(clippy::too_many_arguments)]
fn axis_frac(router: Router, seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize, dome_m: f64, uplift: f32, p: &FluvialParams, tau: f32) -> f64 {
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    let (h, d, area) = run_arm(router, seed, face, level, oi, oj, nx, dome_m, uplift, p);
    let m = channel_mask(&d, &area, &h, sea, tau);
    orientation_split(&m, &h, nx).0
}

fn report_site(name: &str, seed: u64, face: Face, level: u8, oi: u32, oj: u32, nx: usize, dome_m: f64, uplift: f32, p: &FluvialParams) {
    let sea = sea_level::derived_sea_level_m(seed) as f32;
    println!("\n================ SITE: {name} ================");
    println!("  face={face:?} level={level} origin=({oi},{oj}) nx={nx} dome={dome_m}m uplift={uplift}m/epoch epochs={}", p.epochs);

    // Baseline = current live kernel.
    let (h_live, d_live, area) = run_arm(Router::LiveMfd, seed, face, level, oi, oj, nx, dome_m, uplift, p);
    let land = h_live.iter().filter(|&&h| h > sea).count();
    println!("  land fraction: {}/{} = {:.1}%", land, nx * nx, 100.0 * land as f64 / (nx * nx) as f64);
    if land * 3 < nx * nx {
        println!("  !! WARNING: mostly submarine — channels may be a no-op. Results suspect.");
    }

    // Historical control (pre-length-fix kernel) and the two corrections under test.
    let (h_old, d_old, _) = run_arm(Router::UniformOld, seed, face, level, oi, oj, nx, dome_m, uplift, p);
    let (h_et, d_et, _) = run_arm(Router::EdgeTrue, seed, face, level, oi, oj, nx, dome_m, uplift, p);
    let (h_ef, d_ef, _) = run_arm(Router::EdgeFull, seed, face, level, oi, oj, nx, dome_m, uplift, p);

    println!("  vs LiveMfd baseline: [OLD]=UniformOld(pre-6c1ad97) [EDGE]=fan-only diag-kill [FULL]=fan+tree diag-kill");
    for &tau in &[20.0f32, 50.0, 100.0] {
        let m_live = channel_mask(&d_live, &area, &h_live, sea, tau);
        let m_old = channel_mask(&d_old, &area, &h_old, sea, tau);
        let m_et = channel_mask(&d_et, &area, &h_et, sea, tau);
        let m_ef = channel_mask(&d_ef, &area, &h_ef, sea, tau);
        let n_live = m_live.iter().filter(|&&b| b).count();
        println!("  -- tau={tau:>5.0}  (live channel cells: {n_live})");
        println!("     M1 Jaccard-disagree  live↔OLD={:.4}  live↔EDGE={:.4}  live↔FULL={:.4}",
            jaccard_disagreement(&m_live, &m_old), jaccard_disagreement(&m_live, &m_et), jaccard_disagreement(&m_live, &m_ef));
        let (ol_a, _) = orientation_split(&m_live, &h_live, nx);
        let (od_a, _) = orientation_split(&m_old, &h_old, nx);
        let (et_a, _) = orientation_split(&m_et, &h_et, nx);
        let (ef_a, _) = orientation_split(&m_ef, &h_ef, nx);
        println!("     M3 axis-frac  live={:.3}  Δaxis(OLD)={:+.4}  Δaxis(EDGE)={:+.4}  Δaxis(FULL)={:+.4}",
            ol_a, od_a - ol_a, et_a - ol_a, ef_a - ol_a);
    }
    let (sp_old, l2_old) = drainage_divergence(&d_live, &d_old, &h_live, sea);
    let (sp_et, l2_et) = drainage_divergence(&d_live, &d_et, &h_live, sea);
    let (sp_ef, l2_ef) = drainage_divergence(&d_live, &d_ef, &h_live, sea);
    println!("  M2 log-drainage spearman/relL2  OLD={sp_old:.3}/{l2_old:.3}  EDGE={sp_et:.3}/{l2_et:.3}  FULL={sp_ef:.3}/{l2_ef:.3}");
}

fn main() {
    let p = FluvialParams::default();
    let seed = 0u64;
    let face = Face::ZPos;
    let level = 19u8;
    let nx = 96usize;
    let n = 1u32 << level; // 524288 cells/edge
    let dome_m = 1200.0;
    let uplift = 0.02f32; // sustain relief so channels keep carving

    // Off-centre footprint: near a face CORNER (strong equiangular shear).
    let (oi_off, oj_off) = (nx as u32 + 4, nx as u32 + 4); // ~cells (100,100): deep corner region
    // Face-centre control footprint (D4 acquittal control).
    let (oi_ctr, oj_ctr) = (n / 2 - nx as u32 / 2, n / 2 - nx as u32 / 2);

    println!("############ ROUTING PRICING EXPERIMENT ############");
    println!("Prices DECISIONS[routing-violates-the-potential-identity-and-the-replacement-does-not-fix-it] severity. Predictions: .super-archive/from-msc/spike-router-pricing/PREDICTIONS.md");

    // ---- P0: bit-match anchor, run 3x (determinism) ----
    // localize divergence epoch
    {
        println!("\n---- LOCALIZE: per-epoch first divergence (off-centre) ----");
        let sea = sea_level::derived_sea_level_m(seed) as f32;
        let surf = terrain(seed, face, level, oi_off, oj_off, nx, dome_m);
        for ep in [0u32, 1, 2, 5] {
            let pp = FluvialParams { epochs: ep, ..p.clone() };
            let mut live = Fluvial::from_surface(seed, face, level, oi_off, oj_off, nx, &surf);
            live.set_uniform_uplift(uplift);
            live.erode(&pp);
            let mut mine = Pipe::from_surface(face, level, oi_off, oj_off, nx, Router::LiveMfd, &surf);
            mine.set_uniform_uplift(uplift);
            mine.erode(&pp, sea);
            let hd = live.h.iter().zip(&mine.h).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
            let ndiff = live.h.iter().zip(&mine.h).filter(|(a, b)| a != b).count();
            let ad = live.cell_area.iter().zip(&mine.cell_area).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
            let dd = live.drainage.iter().zip(&mine.drainage).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
            println!("  epochs={ep}: max|Δh|={hd:.4e}  differing cells={ndiff}  max|Δcell_area|={ad:.4e}  max|Δdrainage|={dd:.4e}");
        }
    }

    println!("\n---- P0: LiveMfd arm vs live erosion.rs (bit-for-bit), 3x ----");
    let mut all_ok = true;
    for trial in 0..3 {
        let ok_off = validate_bitmatch(seed, face, level, oi_off, oj_off, nx, dome_m, uplift, &p);
        let ok_ctr = validate_bitmatch(seed, face, level, oi_ctr, oj_ctr, nx, dome_m, uplift, &p);
        println!("  trial {trial}: off-centre bitmatch={ok_off}  centre bitmatch={ok_ctr}");
        all_ok &= ok_off && ok_ctr;
    }
    println!("  P0 {}", if all_ok { "PASS — harness LiveMfd == live erosion.rs" } else { "FAIL — results below are NOT about the live world" });

    // ---- determinism of the corrected arms (3x, expect bit-identical) ----
    println!("\n---- determinism: EdgeTrue + EdgeFull drainage bit-identical across 3 reruns ----");
    let mut det = true;
    for arm in [Router::EdgeTrue, Router::EdgeFull] {
        let base = run_arm(arm, seed, face, level, oi_off, oj_off, nx, dome_m, uplift, &p).1;
        for t in 1..3 {
            let r = run_arm(arm, seed, face, level, oi_off, oj_off, nx, dome_m, uplift, &p).1;
            if r != base {
                det = false;
                println!("  {arm:?} rerun {t}: DRIFT (nondeterministic — this is a bug and IS the finding)");
            }
        }
    }
    println!("  determinism {}", if det { "PASS (3x bit-identical)" } else { "FAIL" });

    // ---- the pricing measurement ----
    report_site("OFF-CENTRE (corner, sheared Jacobian)", seed, face, level, oi_off, oj_off, nx, dome_m, uplift, &p);
    report_site("FACE-CENTRE CONTROL (D4 acquittal)", seed, face, level, oi_ctr, oj_ctr, nx, dome_m, uplift, &p);

    // ---- THE null-test differential: cube-locked orientation bias per arm ----
    // A router's *cube-locked* channel-orientation effect = (arm−live at corner)
    // MINUS (arm−live at face centre). The face-centre term subtracts off the
    // generic (non-cube) part of the routing change; what remains is the bias that
    // vanishes under D4 symmetry — the topological defect the decision priced.
    println!("\n======= NULL-TEST DIFFERENTIAL (cube-locked channel-axis bias) =======");
    println!("  axis-fraction excess over live, corner minus centre. ~0 ⇒ no cube-locked");
    println!("  orientation consequence; large ⇒ real cube-locked landscape effect.");
    println!("  CUBE = (arm−live at corner) − (arm−live at centre). tau | OLD | EDGE(fan) | FULL(fan+tree)");
    let cube = |r: Router, tau: f32| -> f64 {
        let live_off = axis_frac(Router::LiveMfd, seed, face, level, oi_off, oj_off, nx, dome_m, uplift, &p, tau);
        let live_ctr = axis_frac(Router::LiveMfd, seed, face, level, oi_ctr, oj_ctr, nx, dome_m, uplift, &p, tau);
        let off = axis_frac(r, seed, face, level, oi_off, oj_off, nx, dome_m, uplift, &p, tau);
        let ctr = axis_frac(r, seed, face, level, oi_ctr, oj_ctr, nx, dome_m, uplift, &p, tau);
        (off - live_off) - (ctr - live_ctr)
    };
    for &tau in &[20.0f32, 50.0, 100.0] {
        println!("  {tau:>4.0} | OLD {:+.4} | EDGE {:+.4} | FULL {:+.4}",
            cube(Router::UniformOld, tau), cube(Router::EdgeTrue, tau), cube(Router::EdgeFull, tau));
    }
    println!("  Reading: OLD's large +CUBE = the length fix (6c1ad97) removed a real cube-locked");
    println!("  grid-axis channel bias. EDGE ~0 = killing diagonals in the FAN adds little further");
    println!("  cube-locked bias. FULL large +CUBE = killing diagonals in the RECEIVER TREE too is");
    println!("  landscape-consequential AND a naive D4 tree is itself NOT cube-safe (it axis-locks");
    println!("  more at the corner). The full FE(6b) surface is priced; the fan half is benign, the");
    println!("  tree half is not — the principled Coatléven FV remedy (not a naive D4 kill) is open.");
}
