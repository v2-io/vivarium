//! The framebuffer: one window-sized RGBA image, rebuilt only when the view
//! changed. A rebuild (1) pre-samples the surface over the visible cell rectangle
//! once, (2) builds each cell's memoized tile, (3) composites them back-to-front by
//! blitting into the buffer. Cost is bounded by screen area + the (small) tile
//! vocabulary, not by world size — the neworld property, made measurable.

use std::time::Instant;

use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::iso::{Projector, ZOOMS};
use crate::nav::Navigator;
use crate::tile::{get_or_raster, key_for, TileCache, NEIGHBORS};
use crate::world::{SurfaceCell, WorldSource};

/// Sky/clear colour the framebuffer is wiped to each rebuild (RGBA).
const SKY: [u8; 4] = [205, 209, 214, 255];

#[derive(Resource)]
pub struct Framebuffer {
    pub handle: Handle<Image>,
    pub w: usize,
    pub h: usize,
}

#[derive(Resource)]
pub struct TileCacheRes(pub TileCache);

#[derive(Resource, Default)]
pub struct RenderStats {
    pub tiles: usize,
    pub unique: usize,
    pub hit_rate: f32,
    pub rebuild_ms: f32,
    pub cells_sampled: usize,
}

/// Create the framebuffer image filled with sky.
pub fn make_framebuffer(images: &mut Assets<Image>, w: usize, h: usize) -> Framebuffer {
    let image = Image::new_fill(
        Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &SKY,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    let handle = images.add(image);
    Framebuffer { handle, w, h }
}

/// One drawable terrain cell, collected before sorting (we cannot hold a cache
/// borrow while sorting, so keys are gathered first, then rasterised in depth order).
struct Item {
    depth: f32,
    anchor: Vec2,
    key: crate::tile::TileKey,
}

pub fn rebuild_framebuffer(
    mut nav: ResMut<Navigator>,
    world: Res<WorldSource>,
    fb: Res<Framebuffer>,
    mut images: ResMut<Assets<Image>>,
    mut cache: ResMut<TileCacheRes>,
    mut stats: ResMut<RenderStats>,
) {
    if !nav.dirty {
        return;
    }
    let t0 = Instant::now();
    if nav.shape_changed {
        cache.0.clear(); // orientation/zoom changed every tile's shape
        nav.shape_changed = false;
    }
    nav.dirty = false;

    let zoom = ZOOMS[nav.zoom];
    let stride = zoom.stride_vox;
    let focus_cell = nav.focus_vox / stride as f32;
    let screen = Vec2::new(fb.w as f32, fb.h as f32);
    let proj = Projector::new(nav.orientation, zoom, focus_cell, nav.focus_h_vox, world.detail, screen);

    // --- Visible cell rectangle: invert the screen corners on the focus plane,
    // then inflate generously (tall terrain & relief project in from outside). ---
    let corners = [Vec2::ZERO, Vec2::new(screen.x, 0.0), Vec2::new(0.0, screen.y), screen];
    let mut lo = Vec2::splat(f32::MAX);
    let mut hi = Vec2::splat(f32::MIN);
    for c in corners {
        let cell = proj.screen_to_cell(c);
        lo = lo.min(cell);
        hi = hi.max(cell);
    }
    const MARGIN: i32 = 6;
    // Extra rows on the far side so tall background terrain (projected upward) is
    // still sampled. Scales with how much height a band lifts on screen.
    let height_margin = 24;
    let cx0 = lo.x.floor() as i32 - MARGIN;
    let cx1 = hi.x.ceil() as i32 + MARGIN;
    let cz0 = lo.y.floor() as i32 - MARGIN - height_margin;
    let cz1 = hi.y.ceil() as i32 + MARGIN;

    // --- Pre-sample the surface over the rectangle (+1 border for neighbours),
    // once per cell, applying the Z-slice. ---
    let gx0 = cx0 - 1;
    let gz0 = cz0 - 1;
    let gw = (cx1 - cx0 + 3).max(1) as usize;
    let gh = (cz1 - cz0 + 3).max(1) as usize;
    let mut grid: Vec<SurfaceCell> = Vec::with_capacity(gw * gh);
    for j in 0..gh as i32 {
        for i in 0..gw as i32 {
            grid.push(sample_capped(&world, gx0 + i, gz0 + j, stride, nav.slice_vox));
        }
    }
    let at = |cx: i32, cz: i32| -> SurfaceCell {
        let i = (cx - gx0) as usize;
        let j = (cz - gz0) as usize;
        grid[j * gw + i]
    };

    // --- Gather drawable items (keys + screen anchors), then depth-sort. ---
    let mut items: Vec<Item> = Vec::new();
    for cz in cz0..=cz1 {
        for cx in cx0..=cx1 {
            let center = at(cx, cz);
            let mut neighbors = [center; 8];
            for (k, (dx, dz)) in NEIGHBORS.iter().enumerate() {
                neighbors[k] = at(cx + dx, cz + dz);
            }
            let key = key_for(&center, &neighbors, world.sea_level);
            let cxf = cx as f32 + 0.5;
            let czf = cz as f32 + 0.5;
            // Anchor at the BANDED height (not raw): this is what makes adjacent
            // tiles agree on shared corners and meet seamlessly (see tile.rs).
            let anchor = proj.project(cxf, czf, crate::tile::banded_height_vox(center.height_vox));
            // Cull tiles whose anchor is far outside the screen (cheap rejection;
            // the tile bitmap is small so a generous bound is fine).
            if anchor.x < -128.0 || anchor.x > screen.x + 128.0 || anchor.y < -256.0 || anchor.y > screen.y + 128.0 {
                continue;
            }
            let depth = proj.depth(cxf, czf);
            items.push(Item { depth, anchor, key });
        }
    }
    // Painter's order: farthest (largest depth) first so nearer tiles draw over.
    items.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));

    // --- Composite into a fresh frame buffer. ---
    let mut frame = vec![0u8; fb.w * fb.h * 4];
    for px in frame.chunks_exact_mut(4) {
        px.copy_from_slice(&SKY);
    }
    for it in &items {
        let tile = get_or_raster(&mut cache.0, it.key, &proj, world.sea_level);
        blit(&mut frame, fb.w, fb.h, tile, it.anchor);
    }

    // The marker is drawn LAST, on top of the terrain. A per-cell painter's order
    // cannot correctly occlude a TALL object — a nearer-but-lower tile would clip
    // across the pawn (the jagged occlusion Joseph saw). Since the pawn is the
    // user's orientation anchor, always-visible is the right call; it is still a
    // real 3D block sitting on the focus cell's (banded) surface.
    let cell_m = stride as f32 / world.detail as f32;
    let half_foot_cells = (MARKER_FOOT_M / cell_m) * 0.5;
    let marker_h_vox = MARKER_TALL_M * world.detail as f32;
    let marker_base = crate::tile::banded_height_vox(nav.focus_h_vox as i32);
    draw_marker_block(&mut frame, fb.w, fb.h, &proj, focus_cell, half_foot_cells, marker_base, marker_h_vox);

    // --- Upload. ---
    if let Some(img) = images.get_mut(&fb.handle) {
        img.data = Some(frame);
    }

    let total = cache.0.hits + cache.0.misses;
    stats.tiles = items.len();
    stats.unique = cache.0.len();
    stats.hit_rate = if total > 0 { cache.0.hits as f32 / total as f32 } else { 0.0 };
    stats.rebuild_ms = t0.elapsed().as_secs_f32() * 1000.0;
    stats.cells_sampled = grid.len();
}

/// Sample one cell, applying the Z-slice: above the slice the surface is cut, shown
/// as a stone cross-section at the slice height.
fn sample_capped(world: &WorldSource, cx: i32, cz: i32, stride: i32, slice: Option<i32>) -> SurfaceCell {
    let mut s = world.sample(cx, cz, stride);
    if let Some(cap) = slice {
        if s.height_vox > cap {
            s.height_vox = cap;
            s.material = 1; // STONE cross-section
            s.water_depth_vox = 0;
            s.wet = false;
        }
    }
    s
}

/// Marker block size, in metres: a ~2 m cube standing on the focus cell.
const MARKER_FOOT_M: f32 = 2.0;
const MARKER_TALL_M: f32 = 2.0;

/// Draw the marker as a real 3D box: bottom on the surface, top `marker_h_vox`
/// above. Side faces (shaded), then the top — and the whole box is depth-sorted
/// against the terrain by the caller, so hills in front occlude it.
fn draw_marker_block(
    frame: &mut [u8],
    fw: usize,
    fh: usize,
    proj: &Projector,
    focus_cell: Vec2,
    half_foot_cells: f32,
    base_h_vox: f32,
    marker_h_vox: f32,
) {
    let (fx, fz) = (focus_cell.x, focus_cell.y);
    let hf = half_foot_cells.max(0.15); // keep it visible even at coarse zoom
    // Footprint corners (CCW), bottom on the surface and top raised.
    let foot = [(fx - hf, fz - hf), (fx + hf, fz - hf), (fx + hf, fz + hf), (fx - hf, fz + hf)];
    let bot: [Vec2; 4] = std::array::from_fn(|i| proj.project(foot[i].0, foot[i].1, base_h_vox));
    let top: [Vec2; 4] = std::array::from_fn(|i| proj.project(foot[i].0, foot[i].1, base_h_vox + marker_h_vox));

    let side_dark = [150u8, 28, 28];
    let side_mid = [190u8, 40, 40];
    let top_rgb = [230u8, 80, 80];

    // Draw the four side faces far→near so the box reads solid, then the top.
    let mut sides: [(usize, f32); 4] = std::array::from_fn(|i| {
        let j = (i + 1) % 4;
        let mx = (foot[i].0 + foot[j].0) * 0.5;
        let mz = (foot[i].1 + foot[j].1) * 0.5;
        (i, proj.depth(mx, mz))
    });
    sides.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (k, (i, _)) in sides.iter().enumerate() {
        let i = *i;
        let j = (i + 1) % 4;
        let rgb = if k < 2 { side_dark } else { side_mid }; // nearer two a touch brighter
        fill_quad_frame(frame, fw, fh, &[bot[i], bot[j], top[j], top[i]], rgb);
    }
    fill_quad_frame(frame, fw, fh, &[top[0], top[1], top[2], top[3]], top_rgb);
}

/// Fill a convex quad into the frame as two opaque triangles.
fn fill_quad_frame(frame: &mut [u8], fw: usize, fh: usize, q: &[Vec2; 4], rgb: [u8; 3]) {
    fill_tri_frame(frame, fw, fh, q[0], q[1], q[2], rgb);
    fill_tri_frame(frame, fw, fh, q[0], q[2], q[3], rgb);
}

/// Scanline-fill a triangle directly into the framebuffer (opaque), bounds-clamped.
fn fill_tri_frame(frame: &mut [u8], fw: usize, fh: usize, a: Vec2, b: Vec2, c: Vec2, rgb: [u8; 3]) {
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(fh as f32 - 1.0) as i32;
    for y in min_y..=max_y {
        let py = y as f32 + 0.5;
        let mut lo = f32::MAX;
        let mut hi = f32::MIN;
        for (p, q) in [(a, b), (b, c), (c, a)] {
            if (p.y <= py && q.y > py) || (q.y <= py && p.y > py) {
                let t = (py - p.y) / (q.y - p.y);
                let x = p.x + t * (q.x - p.x);
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
        if hi < lo {
            continue;
        }
        let x0 = lo.floor().max(0.0) as i32;
        let x1 = hi.ceil().min(fw as f32 - 1.0) as i32;
        for x in x0..=x1 {
            let i = ((y as usize) * fw + x as usize) * 4;
            frame[i] = rgb[0];
            frame[i + 1] = rgb[1];
            frame[i + 2] = rgb[2];
            frame[i + 3] = 255;
        }
    }
}

/// Alpha-blit a tile (opaque where alpha==255) into the frame at the cell anchor.
fn blit(frame: &mut [u8], fw: usize, fh: usize, tile: &crate::tile::TileBitmap, anchor: Vec2) {
    let base_x = anchor.x.round() as i32 + tile.off_x;
    let base_y = anchor.y.round() as i32 + tile.off_y;
    for ty in 0..tile.h as i32 {
        let fy = base_y + ty;
        if fy < 0 || fy >= fh as i32 {
            continue;
        }
        for tx in 0..tile.w as i32 {
            let fx = base_x + tx;
            if fx < 0 || fx >= fw as i32 {
                continue;
            }
            let si = ((ty as usize) * tile.w + tx as usize) * 4;
            if tile.data[si + 3] == 0 {
                continue; // transparent texel
            }
            let di = ((fy as usize) * fw + fx as usize) * 4;
            frame[di] = tile.data[si];
            frame[di + 1] = tile.data[si + 1];
            frame[di + 2] = tile.data[si + 2];
            frame[di + 3] = 255;
        }
    }
}
