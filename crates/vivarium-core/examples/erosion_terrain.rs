//! Erosion run over the *actual* voxel-world terrain — take the Perlin/FBM
//! landscape `vivarium-core` already generates, and let uplift + stream-power
//! erosion carve valley networks into it, draining to its coastlines. Writes
//! hillshaded before/after images so the reshaping is visible at a glance.
//!
//! Run: `cargo run -p vivarium-core --example erosion_terrain [seed] [epochs] [out_dir]`
//!
//! This is the "see how it looks" companion to `erosion_preview`: that one starts
//! from a blank slate to study pure network emergence; this one shows what
//! erosion *does to the landscape the game has now*. Still the abstraction tier —
//! it produces a heightfield, not yet voxels (that round-trip is the next step).

use vivarium_core::geo::{ErosionParams, Heightfield};
use vivarium_core::voxel::{Volume, SEA_LEVEL};

const NX: usize = 360; // simulation-grid edge, in world units (cell_size = 1)

fn main() {
    let mut args = std::env::args().skip(1);
    let seed: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(0x00C0_FFEE);
    let epochs: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(50);
    let out_dir = args.next().unwrap_or_else(|| ".".into());

    // 1. Sample the existing voxel terrain into a heightfield. We read only the
    //    public surface_height API — this is exactly how the slow tier will draw
    //    its initial state from the world's own generation.
    let vol = Volume::new(seed);
    let mut h = vec![0.0f32; NX * NX];
    for y in 0..NX {
        for x in 0..NX {
            h[y * NX + x] = vol.surface_height(x as i32, y as i32).unwrap_or(0) as f32;
        }
    }
    let before = Heightfield::from_heights(NX, 1.0, h.clone());

    // 2. Erode it. Modest uplift keeps the mountains from simply melting to the
    //    sea over many epochs; the coastline (sea level) is the drainage outlet.
    let params = ErosionParams {
        nx: NX,
        cell_size: 1.0,
        uplift: 0.04,
        k: 0.20,
        m: 0.5,
        max_slope: 0.8,
        deposition: 0.5, // Davy-Lague transport (added 2026-06-23); see geo::deposit
        epochs,
        dt: 1.0,
        sea_level: Some(SEA_LEVEL as f32),
    };
    let after = Heightfield::from_heights(NX, 1.0, h).erode(&params);

    let (blo, bhi) = before.elevation_range();
    let (alo, ahi) = after.elevation_range();
    println!("vivarium erosion over terrain — seed {seed:#x}, {epochs} epochs, {NX}×{NX}");
    println!("  before elevation: {blo:.1}..{bhi:.1}   after: {alo:.1}..{ahi:.1}");
    println!("  sea level: {SEA_LEVEL}");

    // 3. Render. Shared elevation range so the lowering reads true between images.
    let (lo, hi) = (blo.min(alo), bhi.max(ahi));
    let sea = SEA_LEVEL as f32;
    let before_png = format!("{out_dir}/erosion_before_{seed:x}.bmp");
    let after_png = format!("{out_dir}/erosion_after_{seed:x}.bmp");
    write_bmp(&before_png, &shade(&before, lo, hi, sea, false));
    write_bmp(&after_png, &shade(&after, lo, hi, sea, true));
    println!("wrote:\n  {before_png}\n  {after_png}");
}

/// An image: width, height, and row-major RGB. Width == height == NX·SCALE.
struct Image {
    w: usize,
    h: usize,
    px: Vec<[u8; 3]>,
}

const SCALE: usize = 2; // upscale each cell to SCALE² pixels for a viewable size

/// Hillshade the field: Lambertian shading from a NW light over the elevation
/// gradient gives terrain its readable shape (ridges bright, valleys shadowed).
/// Below sea level is water; on the "after" image, channels carrying significant
/// drainage are tinted blue so the carved river network stands out.
fn shade(f: &Heightfield, lo: f32, hi: f32, sea: f32, rivers: bool) -> Image {
    let nx = f.nx;
    let span = (hi - lo).max(1e-6);
    // River onset: a small fraction of the whole basin's area.
    let total = (nx * nx) as f32 * f.cell_size * f.cell_size;
    let river_threshold = total * 0.0015;
    // Light direction (NW, somewhat above the horizon), normalized.
    let light = {
        let v = [-0.6f32, -0.6, 0.5];
        let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / m, v[1] / m, v[2] / m]
    };

    let mut px = vec![[0u8; 3]; nx * SCALE * nx * SCALE];
    let w = nx * SCALE;
    for y in 0..nx {
        for x in 0..nx {
            let i = f.idx(x, y);
            let h = f.h[i];
            let color = if h <= sea {
                // Water: depth-shaded blue.
                let d = ((sea - h) / span).clamp(0.0, 1.0);
                [(40.0 - 20.0 * d) as u8, (90.0 - 30.0 * d) as u8, (150.0 + 60.0 * d) as u8]
            } else {
                // Land: elevation tint × hillshade.
                let xm = x.saturating_sub(1);
                let xp = (x + 1).min(nx - 1);
                let ym = y.saturating_sub(1);
                let yp = (y + 1).min(nx - 1);
                let dzdx = (f.h[f.idx(xp, y)] - f.h[f.idx(xm, y)]) / 2.0;
                let dzdy = (f.h[f.idx(x, yp)] - f.h[f.idx(x, ym)]) / 2.0;
                // Surface normal (-dzdx, -dzdy, 1) normalized, dotted with light.
                let n = [-dzdx, -dzdy, 1.0];
                let nm = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                let lit = ((n[0] * light[0] + n[1] * light[1] + n[2] * light[2]) / nm)
                    .clamp(0.15, 1.0);
                let t = ((h - lo) / span).clamp(0.0, 1.0);
                // Green lowlands → brown/grey highlands ramp, then shaded.
                let base = [
                    90.0 + 110.0 * t,
                    130.0 - 40.0 * t,
                    80.0 + 20.0 * t,
                ];
                let mut c = [
                    (base[0] * lit) as u8,
                    (base[1] * lit) as u8,
                    (base[2] * lit) as u8,
                ];
                if rivers && f.drainage[i] >= river_threshold {
                    c = [55, 110, 190]; // river blue over land
                }
                c
            };
            // Splat the cell to a SCALE×SCALE block.
            for sy in 0..SCALE {
                for sx in 0..SCALE {
                    px[(y * SCALE + sy) * w + (x * SCALE + sx)] = color;
                }
            }
        }
    }
    Image { w, h: nx * SCALE, px }
}

/// Write a 24-bit uncompressed BMP — dependency-free, and opened by macOS
/// Preview and every image viewer. BMP rows are bottom-up and padded to 4 bytes.
fn write_bmp(path: &str, img: &Image) {
    let row_bytes = img.w * 3;
    let pad = (4 - row_bytes % 4) % 4;
    let stride = row_bytes + pad;
    let pixel_data = stride * img.h;
    let file_size = 54 + pixel_data;

    let mut buf = Vec::with_capacity(file_size);
    // -- file header (14 bytes) --
    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&(file_size as u32).to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes()); // reserved
    buf.extend_from_slice(&54u32.to_le_bytes()); // pixel data offset
    // -- info header (40 bytes, BITMAPINFOHEADER) --
    buf.extend_from_slice(&40u32.to_le_bytes());
    buf.extend_from_slice(&(img.w as i32).to_le_bytes());
    buf.extend_from_slice(&(img.h as i32).to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes()); // planes
    buf.extend_from_slice(&24u16.to_le_bytes()); // bits per pixel
    buf.extend_from_slice(&0u32.to_le_bytes()); // no compression
    buf.extend_from_slice(&(pixel_data as u32).to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes()); // 72 dpi x
    buf.extend_from_slice(&2835i32.to_le_bytes()); // 72 dpi y
    buf.extend_from_slice(&0u32.to_le_bytes()); // palette colors
    buf.extend_from_slice(&0u32.to_le_bytes()); // important colors
    // -- pixels, bottom row first, BGR --
    for y in (0..img.h).rev() {
        for x in 0..img.w {
            let [r, g, b] = img.px[y * img.w + x];
            buf.push(b);
            buf.push(g);
            buf.push(r);
        }
        buf.extend(std::iter::repeat(0u8).take(pad));
    }
    std::fs::write(path, &buf).expect("write bmp");
}
