// Quick microbench: dense-Cartesian-patch stencil vs. space-filling-curve neighbor cost.
// Answers: (A) automata stencil throughput vs patch size (cache cliffs → sweet-spot patch),
//          (B) neighbor-of-a-cell cost: Cartesian index vs Morton vs Hilbert (the ID penalty),
//          (C) encode/decode throughput (per-patch-load amortized cost).
use std::hint::black_box;
use std::time::Instant;

// ---------- (A) erosion-like 5-point stencil on a dense flat array ----------
fn bench_stencil(n: usize) -> f64 {
    let mut a = vec![0.0f32; n * n];
    let mut b = vec![0.0f32; n * n];
    for i in 0..n * n { a[i] = ((i as f32) * 0.001).sin(); }
    let iters = (200_000_000 / (n * n)).max(3); // ~2e8 cell-updates total
    let t = Instant::now();
    for _ in 0..iters {
        for y in 1..n - 1 {
            let row = y * n;
            for x in 1..n - 1 {
                let i = row + x;
                let c = a[i];
                // diffusion-like flux from 4 neighbors (stand-in for an erosion stencil)
                b[i] = c + 0.1 * ((a[i - 1] + a[i + 1] + a[i - n] + a[i + n]) * 0.25 - c);
            }
        }
        std::mem::swap(&mut a, &mut b);
    }
    let secs = t.elapsed().as_secs_f64();
    black_box(a[n + 1]);
    ((n - 2) * (n - 2) * iters) as f64 / secs / 1e6 // Mcells/s
}

// ---------- Morton (Z-order) ----------
fn part1by1(mut x: u32) -> u32 { x &= 0xffff; x = (x | (x << 8)) & 0x00FF00FF; x = (x | (x << 4)) & 0x0F0F0F0F; x = (x | (x << 2)) & 0x33333333; x = (x | (x << 1)) & 0x55555555; x }
fn compact1by1(mut x: u32) -> u32 { x &= 0x55555555; x = (x | (x >> 1)) & 0x33333333; x = (x | (x >> 2)) & 0x0F0F0F0F; x = (x | (x >> 4)) & 0x00FF00FF; x = (x | (x >> 8)) & 0x0000FFFF; x }
fn morton_enc(x: u32, y: u32) -> u32 { part1by1(x) | (part1by1(y) << 1) }
fn morton_dec(m: u32) -> (u32, u32) { (compact1by1(m), compact1by1(m >> 1)) }

// ---------- Hilbert (canonical Wikipedia xy<->d) ----------
fn hrot(n: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
    if ry == 0 {
        if rx == 1 { *x = n - 1 - *x; *y = n - 1 - *y; }
        std::mem::swap(x, y);
    }
}
fn xy2d(n: u32, mut x: u32, mut y: u32) -> u32 {
    let mut d = 0u32; let mut s = n / 2;
    while s > 0 {
        let rx = if (x & s) > 0 { 1 } else { 0 };
        let ry = if (y & s) > 0 { 1 } else { 0 };
        d += s * s * ((3 * rx) ^ ry);
        hrot(n, &mut x, &mut y, rx, ry);
        s /= 2;
    }
    d
}
fn d2xy(n: u32, mut d: u32) -> (u32, u32) {
    let (mut x, mut y) = (0u32, 0u32); let mut s = 1u32;
    while s < n {
        let rx = 1 & (d / 2);
        let ry = 1 & (d ^ rx);
        hrot(s, &mut x, &mut y, rx, ry);
        x += s * rx; y += s * ry;
        d /= 4; s *= 2;
    }
    (x, y)
}

// ---------- (B) "east neighbor key" cost, three ways ----------
fn bench_neighbor(m: u32, width: u32) -> (f64, f64, f64) {
    let ops = 50_000_000u32;
    // Cartesian: key = y*width + x ; east = key + 1
    let t = Instant::now();
    let mut acc = 0u64;
    for k in 0..ops { let key = k % (width * width); acc = acc.wrapping_add((key + 1) as u64); }
    black_box(acc);
    let cart = ops as f64 / t.elapsed().as_secs_f64() / 1e6;

    // Morton: decode, x+1, encode
    let t = Instant::now();
    let mut acc = 0u64;
    for k in 0..ops { let (x, y) = morton_dec(k & 0x3fff_ffff); acc = acc.wrapping_add(morton_enc(x + 1, y) as u64); }
    black_box(acc);
    let mort = ops as f64 / t.elapsed().as_secs_f64() / 1e6;

    // Hilbert: decode, x+1, encode
    let t = Instant::now();
    let mut acc = 0u64;
    let side = m;
    for k in 0..ops { let d = k % (side * side); let (x, y) = d2xy(side, d); let xe = (x + 1) & (side - 1); acc = acc.wrapping_add(xy2d(side, xe, y) as u64); }
    black_box(acc);
    let hilb = ops as f64 / t.elapsed().as_secs_f64() / 1e6;
    (cart, mort, hilb)
}

// ---------- (C) encode+decode throughput ----------
fn bench_codec(side: u32) -> (f64, f64) {
    let ops = 50_000_000u32;
    let t = Instant::now();
    let mut acc = 0u64;
    for k in 0..ops { let (x, y) = morton_dec(k & 0x3fff_ffff); acc = acc.wrapping_add(morton_enc(x, y) as u64); }
    black_box(acc);
    let mort = ops as f64 / t.elapsed().as_secs_f64() / 1e6;
    let t = Instant::now();
    let mut acc = 0u64;
    for k in 0..ops { let d = k % (side * side); let (x, y) = d2xy(side, d); acc = acc.wrapping_add(xy2d(side, x, y) as u64); }
    black_box(acc);
    let hilb = ops as f64 / t.elapsed().as_secs_f64() / 1e6;
    (mort, hilb)
}

fn main() {
    println!("(A) dense Cartesian 5-point stencil throughput (bytes/field = {} for the array):", 4);
    for &n in &[64usize, 128, 256, 512, 1024, 2048, 4096] {
        let mc = bench_stencil(n);
        let km = (n as f64 * 0.5) / 1000.0;
        let mb = (n * n * 4) as f64 / 1e6;
        println!("   {n:>5}x{n:<5}  ({km:>5.2} km @0.5m, {mb:>7.1} MB/field)  -> {mc:>8.1} Mcells/s", n = n, km = km, mb = mb, mc = mc);
    }
    println!("\n(B) 'east neighbor key' cost (Mops/s, higher=faster):");
    let (c, m, h) = bench_neighbor(1 << 12, 1 << 12); // 4096-side patch
    println!("   Cartesian idx+1 : {c:>8.1} Mops/s", c = c);
    println!("   Morton dec/enc  : {m:>8.1} Mops/s  ({:.1}x slower than Cartesian)", c / m, m = m);
    println!("   Hilbert dec/enc : {h:>8.1} Mops/s  ({:.1}x slower than Cartesian)", c / h, h = h);
    println!("\n(C) encode+decode roundtrip throughput (Mops/s):");
    let (m, h) = bench_codec(1 << 12);
    println!("   Morton  : {m:>8.1} Mops/s", m = m);
    println!("   Hilbert : {h:>8.1} Mops/s", h = h);
}
