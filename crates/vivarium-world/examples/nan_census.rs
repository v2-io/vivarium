//! **Does any stored payload contain a non-finite height?**
//!
//! A NaN in a stored bed is the worst class of defect this store can hold,
//! because it is *silent in the direction of looking fine*. Every comparison
//! against a NaN is false, so `h <= sea` is false and a NaN cell classifies as
//! **land** — which under #form-ocean-is-connectivity-not-elevation makes it a
//! *wall* in the ocean-connectivity mask, able to enclose a basin that is not
//! enclosed. Reductions hide it too: `f32::max` and `f32::min` return the
//! non-NaN operand, so a tile's min/max relief reads clean while a cell inside
//! it is nothing at all.
//!
//! Reports per nomos: roots scanned, roots containing any non-finite value, and
//! the worst offender's key. Zero rows is the only good answer; anything else is
//! a finding, and the key is printed so it can be pulled apart.
//!
//! Read-only: opens the store, decodes payloads, writes nothing and computes no
//! world.
//!
//! Run: `cargo run --release -p vivarium-world --example nan_census [world-dir]`

use std::collections::BTreeMap;
use vivarium_world::store::Store;

fn decode_f32(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| {
        let home = std::env::var("HOME").expect("HOME");
        format!("{home}/.cache/vivarium/globe-world")
    });
    let store = Store::open(&dir).expect("open store");
    let roots = store.roots().expect("roots");
    println!("store: {dir}");
    println!("roots: {}", roots.len());

    // Per nomos: (roots, roots with any non-finite, total cells, non-finite cells)
    let mut tally: BTreeMap<String, (usize, usize, usize, usize)> = BTreeMap::new();
    let mut worst: Option<(usize, String)> = None;

    for r in &roots {
        let nomos = r.key.split('@').next().unwrap_or("(unkeyed)").to_string();
        let Some(bytes) = store.object_bytes(&r.object) else { continue };
        let v = decode_f32(&bytes);
        let bad = v.iter().filter(|x| !x.is_finite()).count();
        let e = tally.entry(nomos).or_insert((0, 0, 0, 0));
        e.0 += 1;
        e.2 += v.len();
        if bad > 0 {
            e.1 += 1;
            e.3 += bad;
            if worst.as_ref().is_none_or(|(n, _)| bad > *n) {
                worst = Some((bad, r.key.clone()));
            }
        }
    }

    println!();
    println!("{:<46} {:>7} {:>9} {:>12} {:>10}", "nomos", "roots", "BAD roots", "cells", "BAD cells");
    for (nomos, (n, nbad, cells, cbad)) in &tally {
        let flag = if *nbad > 0 { "  <== NON-FINITE" } else { "" };
        println!("{nomos:<46} {n:>7} {nbad:>9} {cells:>12} {cbad:>10}{flag}");
    }

    // **Which cohort?** A store accumulates every source tree it was ever built
    // under, and a defect in a retired cohort is archaeology while the same defect
    // in the current one is live. `src=` is the discriminator, and the current
    // binary's own hash says which row is now.
    println!();
    println!("erosion-tile by cohort (src=), newest binary is {}:", vivarium_world::nomotheke::SRC_HASH);
    let mut by_src: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    for r in &roots {
        if !r.key.starts_with("erosion-tile@") {
            continue;
        }
        let src = r
            .key
            .split('|')
            .find_map(|f| f.strip_prefix("src="))
            .unwrap_or("(none)")
            .to_string();
        let Some(bytes) = store.object_bytes(&r.object) else { continue };
        let bad = decode_f32(&bytes).iter().filter(|x| !x.is_finite()).count();
        let e = by_src.entry(src).or_insert((0, 0, 0));
        e.0 += 1;
        if bad > 0 {
            e.1 += 1;
            e.2 += bad;
        }
    }
    let mut rows: Vec<_> = by_src.into_iter().collect();
    rows.sort_by_key(|(_, (_, nbad, _))| std::cmp::Reverse(*nbad));
    for (src, (n, nbad, cbad)) in rows {
        let now = if src == vivarium_world::nomotheke::SRC_HASH { "  <== CURRENT" } else { "" };
        println!("  src={src}  roots {n:>6}  BAD roots {nbad:>5}  BAD cells {cbad:>8}{now}");
    }

    // **Where are they?** If non-finite roots sit on the region perimeter, the
    // suspect is the halo window overhanging the cube chart — `from_surface`
    // clamps out-of-chart indices to the last row, so clamped cells share a
    // centre, and a shared centre means `dist_m` returns 0 for a real neighbour
    // pair. Division by that is a NaN mint from finite inputs, which is what the
    // census shows. A scatter of interior positions refutes this and points at
    // the arithmetic instead.
    println!();
    println!("current-cohort erosion-tile roots holding non-finite cells, by position:");
    let mut positions: Vec<(u8, u32, u32, u8, usize)> = Vec::new();
    for r in &roots {
        if !r.key.starts_with("erosion-tile@") {
            continue;
        }
        if r.key.split('|').find_map(|f| f.strip_prefix("src=")) != Some(vivarium_world::nomotheke::SRC_HASH) {
            continue;
        }
        let Some(bytes) = store.object_bytes(&r.object) else { continue };
        let bad = decode_f32(&bytes).iter().filter(|x| !x.is_finite()).count();
        if bad == 0 {
            continue;
        }
        let num = |n: &str| r.key.split('|').find_map(|f| f.strip_prefix(n)).and_then(|v| v.parse::<u32>().ok());
        positions.push((
            num("face=").unwrap_or(99) as u8,
            num("oi=").unwrap_or(0),
            num("oj=").unwrap_or(0),
            num("level=").unwrap_or(0) as u8,
            bad,
        ));
    }
    positions.sort();
    positions.dedup();
    // At L9 a whole-face region is 8×8 tiles of 64, so tile origins run 0..=448
    // and the perimeter is oi or oj ∈ {0, 448}.
    let (mut perim, mut interior) = (0usize, 0usize);
    for &(_, oi, oj, level, _) in &positions {
        let span = (1u32 << level) - 64;
        if oi == 0 || oj == 0 || oi >= span || oj >= span { perim += 1 } else { interior += 1 }
    }
    println!("  {} distinct (face,oi,oj,level); ON REGION PERIMETER: {perim}   interior: {interior}", positions.len());
    for (f, oi, oj, level, bad) in positions.iter().take(12) {
        let span = (1u32 << level) - 64;
        let where_ = if *oi == 0 || *oj == 0 || *oi >= span || *oj >= span { "perimeter" } else { "INTERIOR" };
        println!("    face {f} L{level} ({oi:>5},{oj:>5})  bad cells {bad:>5}  {where_}");
    }

    println!();
    match worst {
        None => println!("VERDICT: every payload is finite in every cell."),
        Some((n, key)) => {
            println!("VERDICT: NON-FINITE VALUES PRESENT. Worst root holds {n} of them:");
            println!("  {key}");
        }
    }
}
