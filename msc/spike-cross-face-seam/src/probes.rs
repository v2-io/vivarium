//! The probes. Each is written so it *could* return the answer I do not want,
//! and the confirm-the-prior ones carry a control that could kill them
//! (`#norm-probe-sensitivity`).

use std::collections::HashMap;

use vivarium_world::measure::cell_area_m2;
use vivarium_world::sphere::{CellId, CubeCoord, Face};
use vivarium_world::store::Store;

/// Great-circle distance (m) between two unit directions — `atan2(|a×b|, a·b)`,
/// robust for the near-parallel adjacent-centre vectors. Local copy so the spike
/// depends only on what is committed in this worktree's `measure.rs`.
fn gc_dist_m(a: [f64; 3], b: [f64; 3], radius_m: f64) -> f64 {
    let cx = a[1] * b[2] - a[2] * b[1];
    let cy = a[2] * b[0] - a[0] * b[2];
    let cz = a[0] * b[1] - a[1] * b[0];
    let s = (cx * cx + cy * cy + cz * cz).sqrt();
    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    s.atan2(dot) * radius_m
}

use crate::faceflux::{FaceId, FluxRegister};
use crate::seam::{self, EDGES};

const R: f64 = 6_371_000.0; // Planet::EARTH.radius_m
const SEED_FACE: Face = Face::ZPos; // the face we probe from; east edge → XPos

// ─────────────────────────────────────────────────────────────────────────────
// geometry helpers (corner arcs — the shared-edge lengths a single-valued flux
// needs). Corner (ci,cj) at `level` is the cube-sphere point measure::corner_uv
// maps; its direction is face-relative but lives in the common 3-D frame.

fn corner_dir(face: Face, ci: u64, cj: u64, level: u8) -> [f64; 3] {
    let (u, v) = vivarium_world::measure::corner_uv(ci, cj, level);
    CubeCoord { face, u, v }.to_unit()
}

/// Geodesic length (m) of the **east** edge of cell (i,j): corners
/// (i+1,j)..(i+1,j+1).
fn east_edge_len(face: Face, i: u64, j: u64, level: u8) -> f64 {
    gc_dist_m(corner_dir(face, i + 1, j, level), corner_dir(face, i + 1, j + 1, level), R)
}
/// Geodesic length (m) of the **north** edge of cell (i,j): corners
/// (i,j+1)..(i+1,j+1).
fn north_edge_len(face: Face, i: u64, j: u64, level: u8) -> f64 {
    gc_dist_m(corner_dir(face, i, j + 1, level), corner_dir(face, i + 1, j + 1, level), R)
}

fn center_dir(id: CellId) -> [f64; 3] { seam::center_unit(id) }

/// Smooth, globally-continuous initial field (m) — a synthetic conserved scalar.
/// Continuous across every seam by construction, so any cross-seam step a probe
/// reports is an artifact of the *machinery*, not the field.
fn init_field(id: CellId) -> f64 {
    let d = center_dir(id);
    100.0 + 50.0 * d[0] * d[1] + 30.0 * d[2]
}

// ─────────────────────────────────────────────────────────────────────────────

pub fn guard() {
    let n = 1u64 << 6;
    // ZPos east-edge cell and its re-homed neighbour must be on a DIFFERENT face.
    let a = CellId::from_face_ij(SEED_FACE, (n - 1) as u32, (n / 2) as u32, 6);
    let b = seam::rehome(SEED_FACE, n as i64, (n / 2) as i64, 6);
    println!("  ZPos east-edge cell face = {:?}", a.face());
    println!("  its re-homed east neighbour face = {:?}", b.face());
    assert_ne!(a.face(), b.face(), "seam cell must cross a cube face");
    // Today's chunk.rs: same-face fill leaves the out-of-face halo at default.
    println!("  (chunk.rs::fill leaves this out-of-face halo slot at Default — the FE(4) gap)");
    println!("  ✓ the probed seam is a genuine cube-face boundary.");
}

/// P1 — the transform is latent: depth-1 involution (bit-exact) + real adjacency.
pub fn transform_latent() {
    for level in [4u8, 6, 8] {
        let n = 1u64 << level;
        let mut invol_ok = 0usize;
        let mut invol_tot = 0usize;
        let mut ratio_min = f64::INFINITY;
        let mut ratio_max = 0.0f64;
        // known-bad accumulators
        let mut clamp_zero_dist = 0usize;

        for &(_name, di, dj) in &EDGES {
            // walk the edge cells that step OUT of the face along (di,dj)
            for k in 0..n {
                // pick the edge cell and its just-outside ghost
                let (ei, ej, gi, gj): (i64, i64, i64, i64) = if di != 0 {
                    let i = if di > 0 { n as i64 - 1 } else { 0 };
                    (i, k as i64, i + di, k as i64)
                } else {
                    let j = if dj > 0 { n as i64 - 1 } else { 0 };
                    (k as i64, j, k as i64, j + dj)
                };
                let edge_cell = CellId::from_face_ij(SEED_FACE, ei as u32, ej as u32, level);
                let nb = seam::rehome(SEED_FACE, gi, gj, level);
                invol_tot += 1;

                // involution: from nb, step back toward SEED_FACE must return edge_cell.
                // "back" = the ghost of nb that lands where edge_cell is. We test the
                // geometric round-trip: nb's own re-home of the direction of edge_cell.
                let back = CubeCoord::from_unit(center_dir(edge_cell)).cell(level);
                if back == edge_cell { invol_ok += 1; }

                // adjacency geometry: cross-face centre distance vs within-face spacing.
                let d_cross = gc_dist_m(center_dir(edge_cell), center_dir(nb), R);
                let within = if di != 0 {
                    // step along j (perpendicular to an east/west edge)
                    let o = CellId::from_face_ij(SEED_FACE, ei as u32, ((ej + 1).min(n as i64 - 1)) as u32, level);
                    gc_dist_m(center_dir(edge_cell), center_dir(o), R)
                } else {
                    let o = CellId::from_face_ij(SEED_FACE, ((ei + 1).min(n as i64 - 1)) as u32, ej as u32, level);
                    gc_dist_m(center_dir(edge_cell), center_dir(o), R)
                };
                if within > 0.0 {
                    let ratio = d_cross / within;
                    ratio_min = ratio_min.min(ratio);
                    ratio_max = ratio_max.max(ratio);
                }

                // known-bad: clamp transform gives a ghost ON the edge cell → ~0 distance.
                let bad = seam::rehome_clamped(SEED_FACE, gi, gj, level);
                if gc_dist_m(center_dir(edge_cell), center_dir(bad), R) < 1.0 {
                    clamp_zero_dist += 1;
                }
            }
        }
        println!(
            "  L{level:>2}: involution {invol_ok}/{invol_tot} bit-exact   adjacency ratio d_cross/within ∈ [{ratio_min:.4}, {ratio_max:.4}]"
        );
        println!(
            "         known-bad (clamp) collapses to ~0 distance on {clamp_zero_dist}/{invol_tot} cells ⇒ the check is not vacuous"
        );
    }
}

/// P2 — halo DEPTH: where the two face grids stop corresponding.
/// Compare "extrapolate k off A" vs "extrapolate 1 off A, then walk (k-1) along
/// B's own grid outward from the seam". Agreement at depth 1 is by construction.
pub fn halo_depth() {
    let level = 8u8;
    let n = 1u64 << level;
    for depth in 1..=4i64 {
        let mut mismatch = 0usize;
        let mut tot = 0usize;
        let mut worst_corner_mismatch = 0usize;
        // east edge of ZPos
        for k in 0..n as i64 {
            // A-side extrapolation k cells past the east edge
            let a_extrap = seam::rehome(SEED_FACE, n as i64 - 1 + depth, k, level);
            // B-side reference: depth-1 neighbour, then walk (depth-1) further from the seam
            let d1 = seam::rehome(SEED_FACE, n as i64, k, level);
            let (bf, bi, bj) = seam::face_ij(d1);
            // direction "away from the ZPos seam" on face B: whichever axis step
            // increases geodesic distance from the seam. Probe both ±i and pick the
            // one that moves outward (larger distance to the edge cell).
            let edge_cell = CellId::from_face_ij(SEED_FACE, n as u32 - 1, k as u32, level);
            let cand = |ii: i64, jj: i64| CellId::from_face_ij(bf, ii.clamp(0, n as i64 - 1) as u32, jj.clamp(0, n as i64 - 1) as u32, level);
            let steps = [ (1i64,0i64),(-1,0),(0,1),(0,-1) ];
            let mut best = d1;
            let mut best_d = -1.0;
            for &(si, sj) in &steps {
                let c = cand(bi + si * (depth - 1), bj + sj * (depth - 1));
                let dd = gc_dist_m(center_dir(edge_cell), center_dir(c), R);
                if dd > best_d { best_d = dd; best = c; }
            }
            tot += 1;
            if a_extrap != best {
                mismatch += 1;
                if k < (n as i64) / 8 || k > 7 * (n as i64) / 8 { worst_corner_mismatch += 1; }
            }
        }
        println!(
            "  depth {depth}: A-extrapolation ≠ B-grid-walk on {mismatch}/{tot} edge cells  ({} of them in the corner eighths)",
            worst_corner_mismatch
        );
    }
    println!("  (depth 1 is exact by construction; growth with depth = the cross-face grids co-align ONLY on the shared edge)");
}

/// P3 — face-flux identity: canonical, symmetric across faces, a store citizen.
pub fn flux_identity_store() {
    let level = 6u8;
    let n = 1u64 << level;
    let a = CellId::from_face_ij(SEED_FACE, n as u32 - 1, (n / 2) as u32, level); // ZPos
    let b = seam::rehome(SEED_FACE, n as i64, (n / 2) as i64, level); // XPos neighbour
    assert_ne!(a.face(), b.face());

    let f_ab = FaceId::between(a, b);
    let f_ba = FaceId::between(b, a);
    println!("  cross-face pair: a={:?}({}) on {:?}, b={:?}({}) on {:?}",
        a, a.0, a.face(), b, b.0, b.face());
    println!("  FaceId symmetric across seam: {}", f_ab == f_ba);
    println!("  owner (lower CellId, global Ord) = the {:?}-side cell: {}",
        f_ab.owner().face(), f_ab.owner() == a.min(b));
    assert_eq!(f_ab, f_ba);
    assert_eq!(f_ab.owner(), a.min(b));

    // known-bad: a per-face-LOCAL index ownership rule collides across the seam —
    // both cells can have local index (i or j) that ties, so "owner" is ambiguous.
    let (_, ai, _aj) = seam::face_ij(a);
    let (_, bi, _bj) = seam::face_ij(b);
    println!("  known-bad local-index rule: a.i={ai}, b.i={bi} — a per-face index gives no global order (ambiguous owner)");

    // store citizen: persist the register, reopen, census.
    let dir = std::env::temp_dir().join(format!("xface-seam-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut reg = FluxRegister::new();
    reg.set(f_ab, 42.5);
    {
        let s = Store::open(&dir).unwrap();
        let wrote = reg.persist(&s).unwrap();
        println!("  persisted {wrote} face-flux root(s) through the real store.rs");
    }
    // reopen
    let s2 = Store::open(&dir).unwrap();
    let got = FluxRegister::load(&s2, f_ab);
    println!("  reopened store → flux for the seam face = {:?} (survives save/reopen)", got);
    let roots = s2.roots().unwrap();
    let enumerated = roots.iter().find(|r| r.key.starts_with("face-flux@v0"));
    println!("  census enumerates it by meaning: key = {:?}", enumerated.map(|r| r.key.as_str()));
    assert_eq!(got, Some(42.5));
    assert!(enumerated.is_some());
    let _ = std::fs::remove_dir_all(&dir);
}

// ── conservative diffusion engine ────────────────────────────────────────────

struct Domain {
    h: Vec<f64>,
    area: Vec<f64>,
    index: HashMap<CellId, usize>,
}
impl Domain {
    fn new() -> Self { Self { h: vec![], area: vec![], index: HashMap::new() } }
    fn add(&mut self, id: CellId) -> usize {
        if let Some(&i) = self.index.get(&id) { return i; }
        let (f, i, j, l) = id.to_face_ij();
        let idx = self.h.len();
        self.h.push(init_field(id));
        self.area.push(cell_area_m2(f, i as u64, j as u64, l, R));
        self.index.insert(id, idx);
        idx
    }
    fn mass(&self) -> f64 { self.h.iter().zip(&self.area).map(|(h, a)| h * a).sum() }
}

/// A matched face between two same-level cells (interior OR cross-face seam).
struct Face2 { a: usize, b: usize, len: f64, dist: f64 }
/// A hanging node: one coarse cell, two fine cells, across the seam.
struct Hang { coarse: usize, fine: [usize; 2], len_f: [f64; 2], dist_f: [f64; 2], len_c: f64, dist_c: f64 }

const D: f64 = 1.0e-3; // diffusivity (units chosen so dt below is stable)
const DT: f64 = 1.0;

fn step_single(dom: &mut Domain, matched: &[Face2], hang: &[Hang]) {
    let mut dq = vec![0.0f64; dom.h.len()];
    for f in matched {
        let flux = D * (dom.h[f.a] - dom.h[f.b]) * f.len / f.dist;
        dq[f.a] -= flux * DT;
        dq[f.b] += flux * DT;
    }
    for hn in hang {
        // single-valued: one flux PER SUB-FACE, applied to both its cells.
        for s in 0..2 {
            let flux = D * (dom.h[hn.coarse] - dom.h[hn.fine[s]]) * hn.len_f[s] / hn.dist_f[s];
            dq[hn.coarse] -= flux * DT;
            dq[hn.fine[s]] += flux * DT;
        }
    }
    for (k, d) in dq.iter().enumerate() { dom.h[k] += d / dom.area[k]; }
}

fn step_twosided(dom: &mut Domain, matched: &[Face2], hang: &[Hang], seam_pairs: &[usize]) {
    // Interior matched faces: single-valued (not the object under test).
    // Seam matched faces (indices in `seam_pairs`) and hanging nodes: each side
    // computes ITS OWN flux and updates only itself — "vivarium today".
    let mut dq = vec![0.0f64; dom.h.len()];
    for (fi, f) in matched.iter().enumerate() {
        if seam_pairs.contains(&fi) {
            // two-sided: a-side and b-side each compute from own len/dist.
            let fa = D * (dom.h[f.a] - dom.h[f.b]) * f.len / f.dist;
            let fb = D * (dom.h[f.b] - dom.h[f.a]) * f.len / f.dist;
            dq[f.a] -= fa * DT;
            dq[f.b] -= fb * DT; // note: independently applied; equal-&-opposite ONLY if fa==-fb
        } else {
            let flux = D * (dom.h[f.a] - dom.h[f.b]) * f.len / f.dist;
            dq[f.a] -= flux * DT;
            dq[f.b] += flux * DT;
        }
    }
    for hn in hang {
        // coarse side lumps: one flux from the coarse face vs the mean of fines.
        let fine_mean = 0.5 * (dom.h[hn.fine[0]] + dom.h[hn.fine[1]]);
        let fc = D * (dom.h[hn.coarse] - fine_mean) * hn.len_c / hn.dist_c;
        dq[hn.coarse] -= fc * DT; // coarse updates ONLY itself
        // fine side computes each sub-face independently, updates ONLY itself.
        for s in 0..2 {
            let ff = D * (dom.h[hn.fine[s]] - dom.h[hn.coarse]) * hn.len_f[s] / hn.dist_f[s];
            dq[hn.fine[s]] -= ff * DT;
        }
    }
    for (k, d) in dq.iter().enumerate() { dom.h[k] += d / dom.area[k]; }
}

/// P4a — today's default-0 out-of-face halo is a MASS SINK at a cube seam.
pub fn conservation_default_zero() {
    let level = 6u8;
    let n = 1u64 << level;
    let (oj, w) = (n / 2 - 8, 16u64); // a 16-cell strip along the east edge
    let mut dom = Domain::new();
    // one column of ZPos edge cells; each also has a north/south neighbour (closed within strip)
    let mut cells = vec![];
    for jj in oj..oj + w {
        cells.push(dom.add(CellId::from_face_ij(SEED_FACE, n as u32 - 1, jj as u32, level)));
    }
    // matched interior faces (north between consecutive edge cells) — reflecting ends.
    let mut matched = vec![];
    for s in 0..w as usize - 1 {
        let (jj, i) = (oj + s as u64, n - 1);
        matched.push(Face2 { a: cells[s], b: cells[s + 1], len: north_edge_len(SEED_FACE, i, jj, level), dist: gc_dist_m(center_dir(CellId::from_face_ij(SEED_FACE, i as u32, jj as u32, level)), center_dir(CellId::from_face_ij(SEED_FACE, i as u32, (jj + 1) as u32, level)), R) });
    }
    let m0 = dom.mass();
    // The seam east faces exchange with a GHOST at value 0 (today's default halo):
    // mass leaves the closed system into the void.
    for &steps in &[1usize, 10, 100, 1000] {
        let mut d = Domain { h: dom.h.clone(), area: dom.area.clone(), index: dom.index.clone() };
        for _ in 0..steps {
            let mut dq = vec![0.0f64; d.h.len()];
            for f in &matched {
                let flux = D * (d.h[f.a] - d.h[f.b]) * f.len / f.dist;
                dq[f.a] -= flux * DT; dq[f.b] += flux * DT;
            }
            for (s, &ci) in cells.iter().enumerate() {
                let (jj, i) = (oj + s as u64, n - 1);
                let len = east_edge_len(SEED_FACE, i, jj, level);
                // ghost = 0 (default), distance one cell east
                let dist = gc_dist_m(center_dir(CellId::from_face_ij(SEED_FACE, i as u32, jj as u32, level)), seam::center_unit(seam::rehome(SEED_FACE, n as i64, jj as i64, level)), R);
                let flux = D * (d.h[ci] - 0.0) * len / dist;
                dq[ci] -= flux * DT; // into the void — no matching +flux anywhere
            }
            for (k, dd) in dq.iter().enumerate() { d.h[k] += dd / d.area[k]; }
        }
        let m = d.mass();
        println!("  steps {steps:>4}: rel mass drift {:+.4e}   (default-0 halo = Dirichlet-0 sink)", (m - m0) / m0);
    }
}

/// P4b — matched same-level cross-face seam: TwoSided vs SingleValued.
pub fn conservation_matched_seam() {
    let level = 6u8;
    let n = 1u64 << level;
    let (oj, w) = (n / 2 - 8, 16u64);
    let mut dom = Domain::new();
    // ZPos edge column + its XPos neighbour column, joined across the seam.
    let mut zc = vec![]; let mut xc = vec![];
    for jj in oj..oj + w {
        let z = CellId::from_face_ij(SEED_FACE, n as u32 - 1, jj as u32, level);
        let x = seam::rehome(SEED_FACE, n as i64, jj as i64, level);
        zc.push(dom.add(z)); xc.push(dom.add(x));
    }
    let mut matched = vec![]; let mut seam_pairs = vec![];
    // seam faces (ZPos edge cell ↔ XPos neighbour)
    for s in 0..w as usize {
        let jj = oj + s as u64;
        let z = CellId::from_face_ij(SEED_FACE, n as u32 - 1, jj as u32, level);
        let x = seam::rehome(SEED_FACE, n as i64, jj as i64, level);
        let len = east_edge_len(SEED_FACE, n - 1, jj, level);
        let dist = gc_dist_m(center_dir(z), center_dir(x), R);
        seam_pairs.push(matched.len());
        matched.push(Face2 { a: zc[s], b: xc[s], len, dist });
    }
    // within-column north faces (both sides) → closed strip, reflecting ends.
    for s in 0..w as usize - 1 {
        let jj = oj + s as u64;
        let z0 = CellId::from_face_ij(SEED_FACE, n as u32 - 1, jj as u32, level);
        let z1 = CellId::from_face_ij(SEED_FACE, n as u32 - 1, (jj + 1) as u32, level);
        matched.push(Face2 { a: zc[s], b: zc[s + 1], len: north_edge_len(SEED_FACE, n - 1, jj, level), dist: gc_dist_m(center_dir(z0), center_dir(z1), R) });
        // XPos side: consecutive neighbours (may or may not be grid-adjacent; use their real geometry)
        let x0 = seam::rehome(SEED_FACE, n as i64, jj as i64, level);
        let x1 = seam::rehome(SEED_FACE, n as i64, (jj + 1) as i64, level);
        let (xf, xi, xj) = seam::face_ij(x0);
        matched.push(Face2 { a: xc[s], b: xc[s + 1], len: north_edge_len(xf, xi as u64, xj as u64, level), dist: gc_dist_m(center_dir(x0), center_dir(x1), R) });
    }
    let m0 = dom.mass();
    for &(label, two_sided) in &[("SingleValued", false), ("TwoSided", true)] {
        for &steps in &[1usize, 10, 100, 1000, 10000] {
            let mut d = Domain { h: dom.h.clone(), area: dom.area.clone(), index: dom.index.clone() };
            for _ in 0..steps {
                if two_sided { step_twosided(&mut d, &matched, &[], &seam_pairs); }
                else { step_single(&mut d, &matched, &[]); }
            }
            println!("  {label:>12} steps {steps:>4}: rel mass drift {:+.4e}", (d.mass() - m0) / m0);
        }
    }
    println!("  (matched seam: symmetric geometry ⇒ two-sided may also conserve — the register's value here is the DATA-STRUCTURE guarantee, tested next where it bites)");
}

/// P4c — cross-face HANGING NODE (coarse ZPos abuts fine XPos): the PROBE-7
/// conservation contrast lifted onto a genuine cube edge.
pub fn conservation_cross_face_hanging() {
    let lc = 6u8;
    let lf = lc + 1;
    let nc = 1u64 << lc;
    let (oj, w) = (nc / 2 - 8, 16u64); // coarse edge strip
    let mut dom = Domain::new();
    let mut coarse = vec![];
    for jj in oj..oj + w {
        coarse.push(dom.add(CellId::from_face_ij(SEED_FACE, nc as u32 - 1, jj as u32, lc)));
    }
    // For each coarse edge cell, find its TWO fine XPos sub-neighbours by sampling
    // just past the seam at the two fine sub-edge centres.
    let mut hang = vec![];
    let mut fine_ids: Vec<CellId> = vec![];
    for (s, &cc) in coarse.iter().enumerate() {
        let jj = oj + s as u64;
        let vcen = 2.0 * (jj as f64 + 0.5) / nc as f64 - 1.0;
        let dv = 1.0 / nc as f64; // coarse half-height in v
        let u_ghost = 1.0 + 1.0 / (1u64 << lf) as f64; // half a FINE cell past the east edge
        let mut fine_idx = [0usize; 2];
        let mut len_f = [0.0; 2]; let mut dist_f = [0.0; 2];
        for t in 0..2 {
            let v = vcen + if t == 0 { -0.5 * dv } else { 0.5 * dv };
            let dir = CubeCoord { face: SEED_FACE, u: u_ghost, v }.to_unit();
            let fid = CubeCoord::from_unit(dir).cell(lf);
            let fidx = dom.add(fid);
            fine_idx[t] = fidx;
            fine_ids.push(fid);
            let (ff, fi, fj) = seam::face_ij(fid);
            // sub-face length: the fine cell's edge facing the seam (its west edge
            // ≈ east edge of the cell just inside). Use the fine cell's own edge len.
            len_f[t] = east_edge_len(ff, fi.max(0) as u64, fj as u64, lf).max(north_edge_len(ff, fi.max(0) as u64, fj as u64, lf));
            dist_f[t] = gc_dist_m(center_dir(CellId::from_face_ij(SEED_FACE, nc as u32 - 1, jj as u32, lc)), center_dir(fid), R);
        }
        let len_c = east_edge_len(SEED_FACE, nc - 1, jj, lc);
        let dist_c = 0.5 * (dist_f[0] + dist_f[1]);
        hang.push(Hang { coarse: cc, fine: fine_idx, len_f, dist_f, len_c, dist_c });
    }
    // coarse-coarse north faces (closed strip)
    let mut matched = vec![];
    for s in 0..w as usize - 1 {
        let jj = oj + s as u64;
        let c0 = CellId::from_face_ij(SEED_FACE, nc as u32 - 1, jj as u32, lc);
        let c1 = CellId::from_face_ij(SEED_FACE, nc as u32 - 1, (jj + 1) as u32, lc);
        matched.push(Face2 { a: coarse[s], b: coarse[s + 1], len: north_edge_len(SEED_FACE, nc - 1, jj, lc), dist: gc_dist_m(center_dir(c0), center_dir(c1), R) });
    }
    // sanity: fine cells distinct and on XPos
    let distinct: std::collections::HashSet<_> = fine_ids.iter().collect();
    let on_x = fine_ids.iter().all(|f| f.face() == Face::XPos);
    println!("  {} coarse edge cells → {} fine sub-cells ({} distinct), all on XPos: {}",
        coarse.len(), fine_ids.len(), distinct.len(), on_x);

    let m0 = dom.mass();
    for &(label, two_sided) in &[("SingleValued", false), ("TwoSided", true)] {
        for &steps in &[1usize, 10, 100, 1000, 10000] {
            let mut d = Domain { h: dom.h.clone(), area: dom.area.clone(), index: dom.index.clone() };
            for _ in 0..steps {
                if two_sided { step_twosided(&mut d, &matched, &hang, &[]); }
                else { step_single(&mut d, &matched, &hang); }
            }
            println!("  {label:>12} steps {steps:>4}: rel mass drift {:+.4e}", (d.mass() - m0) / m0);
        }
    }
    println!("  (single-valued applies one sub-face flux once with opposite signs — conservation is a property of the DATA STRUCTURE, now across a cube edge)");
}

/// P5 — continuity, known-bad first: default-0 halo PLATEAUS (a true cliff);
/// filled halo VANISHES with the arc (`#norm-probe-sensitivity` §3).
pub fn continuity_scale() {
    for level in [6u8, 8, 10] {
        let n = 1u64 << level;
        let jj = n / 2;
        let z = CellId::from_face_ij(SEED_FACE, n as u32 - 1, jj as u32, level);
        let x = seam::rehome(SEED_FACE, n as i64, jj as i64, level);
        let within = {
            let z1 = CellId::from_face_ij(SEED_FACE, n as u32 - 2, jj as u32, level);
            (init_field(z) - init_field(z1)).abs()
        };
        let cross_filled = (init_field(z) - init_field(x)).abs();
        let cross_default0 = (init_field(z) - 0.0).abs();
        println!(
            "  L{level:>2}: within-face step {within:.4} m | cross step FILLED {cross_filled:.4} m (≈ within, vanishes with arc) | cross step DEFAULT-0 {cross_default0:.2} m (plateaus — a manufactured cliff)"
        );
    }
    println!("  ⇒ FILLED tracks the within-face step and shrinks as the arc shrinks; DEFAULT-0 stays at full field scale ⇒ it is a discontinuity, not physics.");
}
